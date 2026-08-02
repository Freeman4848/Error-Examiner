use crate::*;

impl ErrorExplainerApp {
    pub(crate) fn load_file(&mut self) {
        match attachment::pick_input_file() {
            Ok(Some(attachment::InputFile::Image(image))) => {
                self.status = format!("Attached image {}", image.name);
                self.attachment = Some(image);
                self.prepared_log = None;
            }
            Ok(Some(attachment::InputFile::Log { name, text })) => {
                let budget = (self.settings.max_input_chars * 4 / 5).max(3_000);
                let prepared = if self.settings.normalize_logs {
                    Ok(preprocess::prepare(name, &text, budget))
                } else {
                    preprocess::prepare_raw(name, text, self.settings.max_input_chars)
                };
                let prepared = match prepared {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        self.status = error;
                        return;
                    }
                };
                self.status = format!(
                    "Inserted {}: {} events, {} batch(es)",
                    prepared.name,
                    prepared.event_count,
                    prepared.batches.len()
                );
                if self.input.trim().is_empty() {
                    self.input = "Find the root cause in the inserted log.".to_owned();
                }
                self.prepared_log = Some(prepared);
                self.attachment = None;
            }
            Ok(None) => {}
            Err(error) => self.status = format!("Log error: {error}"),
        }
    }

    pub(crate) fn poll_events(&mut self, context: &egui::Context) {
        while let Ok(command) = self.command_server.try_recv() {
            self.handle_command(command, context);
        }
        #[cfg(target_os = "windows")]
        if let Some(tray) = &self.tray {
            if let Some(command) = tray.poll() {
                self.handle_command(command, context);
            }
        }
        while self.hotkey_receiver.try_recv().is_ok() {
            if self.is_hidden {
                self.page = Page::Chat;
                self.show(context);
                if self.input.is_empty() {
                    self.paste_clipboard();
                }
            } else {
                self.hide(context);
            }
        }
        while let Ok(worker) = self.worker_receiver.try_recv() {
            self.pending = false;
            match worker.result {
                Ok(answer) => self.handle_answer(worker.purpose, answer, context),
                Err(error) => {
                    self.batch_queue.clear();
                    self.status = error;
                }
            }
        }
        while let Ok(result) = self.model_receiver.try_recv() {
            self.loading_models = false;
            match result {
                Ok(models) => {
                    if !models.contains(&self.settings.model) {
                        self.settings.model = models[0].clone();
                    }
                    self.status = format!("Loaded {} models", models.len());
                    self.available_models = models;
                }
                Err(error) => self.status = error,
            }
        }
        while let Ok(result) = self.schema_receiver.try_recv() {
            self.schema_pending = false;
            match result {
                Ok(draft) => {
                    self.schema_status = format!(
                        "Draft validated: {} event(s), {}",
                        draft.event_count,
                        draft.format_ids.join(" + ")
                    );
                    self.schema_draft = Some(draft);
                    self.schema_ui.scroll_to_response = true;
                }
                Err(error) => {
                    self.schema_status = format!("Draft rejected: {error}");
                    self.schema_draft = None;
                }
            }
        }
    }

    fn handle_answer(
        &mut self,
        purpose: RequestPurpose,
        answer: AiAnswer,
        context: &egui::Context,
    ) {
        if let Some(model) = &answer.resolved_model {
            if self.settings.provider == ProviderKind::LmStudio && self.settings.model != *model {
                self.settings.model = model.clone();
                let _ = storage::save_json("settings.json", &self.settings);
            }
        }
        self.last_usage = format_usage(&answer, &self.settings);
        match purpose {
            RequestPurpose::Chat | RequestPurpose::Synthesis => {
                self.push_answer(answer.text);
                self.status = "Analysis complete".to_owned();
            }
            RequestPurpose::Test => {
                self.status = format!("Connection OK: {}", first_line(&answer.text));
            }
            RequestPurpose::Batch { index, total } => {
                self.batch_findings.push(answer.text);
                self.status = format!("Analyzed batch {index}/{total}");
                if self.batch_queue.is_empty() {
                    self.start_synthesis(context);
                } else {
                    self.start_next_batch(context);
                }
            }
        }
    }

    fn push_answer(&mut self, text: String) {
        self.messages.push(ChatMessage {
            role: "assistant".to_owned(),
            sections: Some(ai::parse_sections(&text)),
            content: text,
            timestamp: ai::now_timestamp(),
            image: None,
        });
        self.trim_saved_history();
        self.save_history();
    }

    fn start_request(
        &mut self,
        messages: Vec<ChatMessage>,
        purpose: RequestPurpose,
        context: &egui::Context,
    ) {
        if self.pending {
            return;
        }
        self.pending = true;
        self.status = "Contacting provider…".to_owned();
        let request = AiRequest {
            settings: self.settings.clone(),
            api_key: self.api_key.clone(),
            messages,
            system_prompt: None,
        };
        let sender = self.worker_sender.clone();
        let context = context.clone();
        std::thread::spawn(move || {
            let result = ai::ask(request);
            let _ = sender.send(WorkerResult { purpose, result });
            context.request_repaint();
        });
    }

    pub(crate) fn submit(&mut self, context: &egui::Context) {
        if self.pending {
            return;
        }
        if let Some(prepared) = self.prepared_log.take() {
            self.submit_prepared(prepared, context);
            return;
        }
        let question = self.input.trim().to_owned();
        if question.is_empty() && self.attachment.is_none() {
            return;
        }
        let question = if question.is_empty() {
            "Analyze this screenshot.".to_owned()
        } else {
            question
        };
        let limited = keep_tail(&question, self.settings.max_input_chars);
        self.name_active_chat(question.lines().next().unwrap_or("Chat"));
        self.messages.push(ChatMessage {
            role: "user".to_owned(),
            content: limited,
            timestamp: ai::now_timestamp(),
            image: self.attachment.take(),
            ..Default::default()
        });
        self.save_history();
        self.input.clear();
        self.start_request(self.messages.clone(), RequestPurpose::Chat, context);
    }

    fn submit_prepared(&mut self, prepared: preprocess::PreparedLog, context: &egui::Context) {
        self.name_active_chat(&prepared.name);
        let instruction = if self.input.trim().is_empty() {
            "Find the root cause without inventing errors.".to_owned()
        } else {
            self.input.trim().to_owned()
        };
        self.messages.push(ChatMessage {
            role: "user".to_owned(),
            content: format!(
                "Analyze {}: {} events, {} duplicates removed, {} batches.",
                prepared.name,
                prepared.event_count,
                prepared.duplicate_count,
                prepared.batches.len()
            ),
            timestamp: ai::now_timestamp(),
            ..Default::default()
        });
        self.save_history();
        self.input.clear();
        self.batch_total = prepared.batches.len();
        self.batch_findings.clear();
        self.batch_queue = prepared
            .batches
            .into_iter()
            .map(|batch| format!("{instruction}\n\n{batch}"))
            .collect();
        if self.batch_total <= 1 {
            let content = self.batch_queue.pop_front().unwrap_or_default();
            self.start_request(
                vec![ChatMessage {
                    role: "user".to_owned(),
                    content,
                    ..Default::default()
                }],
                RequestPurpose::Chat,
                context,
            );
        } else {
            self.start_next_batch(context);
        }
    }

    fn start_next_batch(&mut self, context: &egui::Context) {
        let Some(content) = self.batch_queue.pop_front() else {
            return;
        };
        let index = self.batch_total - self.batch_queue.len();
        self.start_request(
            vec![ChatMessage {
                role: "user".to_owned(),
                content: format!("Log batch {index}/{}:\n\n{content}", self.batch_total),
                ..Default::default()
            }],
            RequestPurpose::Batch {
                index,
                total: self.batch_total,
            },
            context,
        );
    }

    fn start_synthesis(&mut self, context: &egui::Context) {
        let findings = self.batch_findings.join("\n\n");
        self.start_request(
            vec![ChatMessage {
                role: "user".to_owned(),
                content: format!(
                    "Combine these batch findings. If no failure exists, say so. Follow CAUSE/FIX/VERIFY format.\n\n{findings}"
                ),
                ..Default::default()
            }],
            RequestPurpose::Synthesis,
            context,
        );
    }

    pub(crate) fn test_provider(&mut self, context: &egui::Context) {
        self.start_request(
            vec![ChatMessage {
                role: "user".to_owned(),
                content: "Reply with exactly: connection ok".to_owned(),
                ..Default::default()
            }],
            RequestPurpose::Test,
            context,
        );
    }

    pub(crate) fn load_models(&mut self, context: &egui::Context) {
        if self.loading_models {
            return;
        }
        self.loading_models = true;
        self.status = "Loading models…".to_owned();
        let settings = self.settings.clone();
        let api_key = self.api_key.clone();
        let sender = self.model_sender.clone();
        let context = context.clone();
        std::thread::spawn(move || {
            let _ = sender.send(ai::list_models(settings, api_key));
            context.request_repaint();
        });
    }
}
