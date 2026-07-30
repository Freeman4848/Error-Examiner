use crate::theme::palette;
use crate::*;

impl ErrorExplainerApp {
    pub(crate) fn chat_ui(&mut self, context: &egui::Context) {
        let colors = palette(self.theme, self.opacity);
        egui::TopBottomPanel::bottom("chat_input")
            .resizable(true)
            .default_height(210.0)
            .min_height(if self.prepared_log.is_some() {
                280.0
            } else {
                150.0
            })
            .max_height(480.0)
            .frame(
                egui::Frame::none()
                    .fill(colors.panel)
                    .inner_margin(egui::Margin::same(12.0))
                    .stroke(egui::Stroke::new(0.5, colors.border)),
            )
            .show(context, |ui| {
                let input_tokens = ai::estimate_tokens(&self.input);
                let max_cost = ai::estimate_max_cost(&self.settings, self.input.chars().count());
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} · Draft ≈{} standard tokens · Response limit {}",
                            self.settings.provider.label(),
                            input_tokens,
                            self.settings.max_output_tokens
                        ))
                        .small()
                        .color(colors.muted),
                    );
                    if let Some(cost) = max_cost {
                        ui.label(
                            RichText::new(format!("estimated max ${cost:.4}"))
                                .small()
                                .color(colors.assistant),
                        );
                    }
                });
                if ui
                    .add_sized(
                        [ui.available_width(), 34.0],
                        egui::Button::new(RichText::new("+  Add log or image").strong())
                            .fill(colors.card)
                            .rounding(egui::Rounding::same(8.0)),
                    )
                    .clicked()
                {
                    self.load_file();
                }
                let editor_height = (ui.available_height() - 70.0).max(60.0);
                let editor = ui.add_sized(
                    [ui.available_width(), editor_height],
                    TextEdit::multiline(&mut self.input)
                        .hint_text("Paste logs, a stack trace, or ask a debugging question…"),
                );
                let submit_shortcut = editor.has_focus()
                    && ui.input(|input| input.key_pressed(Key::Enter) && input.modifiers.ctrl);
                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new("Paste").frame(false)).clicked() {
                        self.paste_clipboard();
                    }
                    if ui.add(egui::Button::new("Clear").frame(false)).clicked() {
                        self.input.clear();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let send = ui
                            .add_enabled(
                                !self.pending
                                    && (!self.input.trim().is_empty()
                                        || self.attachment.is_some()
                                        || self.prepared_log.is_some()),
                                egui::Button::new(if self.pending {
                                    "Analyzing…"
                                } else {
                                    "Explain · Ctrl+Enter"
                                })
                                .fill(colors.accent),
                            )
                            .clicked();
                        if send || submit_shortcut {
                            self.submit(context);
                        }
                    });
                });
                let attachment_label = self
                    .attachment
                    .as_ref()
                    .map(|image| format!("📷 {} · {}×{}", image.name, image.width, image.height));
                if let Some(label) = attachment_label {
                    ui.horizontal(|ui| {
                        ui.label(label);
                        if ui.button("Remove").clicked() {
                            self.attachment = None;
                        }
                    });
                }
                let mut cancel_log = false;
                if let Some(log) = &self.prepared_log {
                    egui::Frame::none()
                        .fill(colors.card)
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(9.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("File ready").strong().color(colors.assistant),
                                );
                                ui.label(RichText::new(&log.name).strong());
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| cancel_log = ui.button("Remove").clicked(),
                                );
                            });
                            ui.horizontal_wrapped(|ui| {
                                ui.label(if log.normalized { "Pre-parsed" } else { "Raw" });
                                ui.label(format!(
                                    "{} → {} chars",
                                    log.original_chars, log.normalized_chars
                                ));
                                ui.label(format!("≈{} standard tokens", log.estimated_tokens));
                                ui.label(format!(
                                    "{} event(s) · {} duplicate(s) · {} batch(es)",
                                    log.event_count,
                                    log.duplicate_count,
                                    log.batches.len()
                                ));
                                if log.normalized {
                                    ui.label(format!("{} important", log.important_events));
                                }
                            });
                        });
                }
                if cancel_log {
                    self.prepared_log = None;
                }
            });

        egui::CentralPanel::default().show(context, |ui| {
            self.chat_tabs_ui(ui);
            ui.horizontal(|ui| {
                ui.label(RichText::new(&self.status).small().color(colors.muted));
                if self.prepared_log.is_some()
                    && ui
                        .add(
                            egui::Button::new("Preview")
                                .fill(colors.accent)
                                .rounding(egui::Rounding::same(6.0)),
                        )
                        .clicked()
                {
                    self.preview_open = true;
                    self.preview_created = false;
                }
                if self.pending {
                    ui.spinner();
                }
                if !self.last_usage.is_empty() {
                    ui.separator();
                    ui.label(RichText::new(&self.last_usage).small());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Clear chat").clicked() {
                        self.clear_history();
                    }
                });
            });
            ui.separator();
            ScrollArea::vertical()
                .id_source("chat_scroll")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        if self.messages.is_empty() {
                            empty_state(ui, colors, &self.app_icon_texture);
                        }
                        for message in &self.messages {
                            message_card(ui, message, colors);
                        }
                    });
                });
        });
    }

    pub(crate) fn settings_ui(&mut self, context: &egui::Context) {
        let colors = palette(self.theme, self.opacity);
        egui::CentralPanel::default().show(context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Appearance");
                egui::ComboBox::from_label("Theme")
                    .selected_text(self.theme.label())
                    .show_ui(ui, |ui| {
                        for theme in ThemeKind::ALL {
                            ui.selectable_value(&mut self.theme, theme, theme.label());
                        }
                    });
                ui.add(
                    egui::Slider::new(&mut self.opacity, 0.3..=1.0)
                        .text("Window opacity")
                        .show_value(true),
                );
                ui.add_space(14.0);
                ui.heading("AI connection");
                ui.label("The API key is never written to disk. Restarting clears it.");
                ui.add_space(12.0);

                let old_provider = self.settings.provider;
                egui::ComboBox::from_label("Provider")
                    .selected_text(self.settings.provider.label())
                    .show_ui(ui, |ui| {
                        for provider in ProviderKind::ALL {
                            ui.selectable_value(
                                &mut self.settings.provider,
                                provider,
                                provider.label(),
                            );
                        }
                    });
                if self.settings.provider != old_provider {
                    self.settings.apply_provider_defaults();
                    self.api_key.clear();
                    self.available_models.clear();
                }

                if self.settings.provider != ProviderKind::Mock {
                    ui.label("Base URL");
                    ui.add_sized(
                        [ui.available_width(), 30.0],
                        TextEdit::singleline(&mut self.settings.base_url),
                    );
                    ui.label("API key (memory only)");
                    ui.add_sized(
                        [ui.available_width(), 30.0],
                        TextEdit::singleline(&mut self.api_key)
                            .password(true)
                            .hint_text("Not saved"),
                    );
                }
                ui_settings_helpers::model_picker(self, ui, context);

                ui.add_space(16.0);
                ui.heading("Limits");
                ui.horizontal(|ui| {
                    ui.label("Prepare logs before AI");
                    ui.selectable_value(&mut self.settings.normalize_logs, true, "On");
                    ui.selectable_value(&mut self.settings.normalize_logs, false, "Off");
                });
                ui.label(
                    RichText::new("Recommended: reduces noise, duplicates, and token usage.")
                        .small()
                        .color(colors.assistant),
                );
                ui.add(
                    egui::Slider::new(&mut self.settings.max_input_chars, 3_000..=120_000)
                        .text("input characters"),
                );
                ui.add(
                    egui::Slider::new(&mut self.settings.max_output_tokens, 128..=8_000)
                        .text("max output tokens"),
                );
                ui.add(
                    egui::Slider::new(&mut self.settings.timeout_seconds, 5..=180)
                        .text("timeout seconds"),
                );
                ui.add(
                    egui::Slider::new(&mut self.settings.retries, 0..=2)
                        .text("transient retries"),
                );
                if self.settings.retries > 0 {
                    ui.colored_label(
                        Color32::from_rgb(251, 146, 60),
                        "Retries may create additional billed requests after ambiguous network failures.",
                    );
                }

                ui.add_space(16.0);
                ui.heading("Optional cost estimate");
                ui.label("Enter current provider prices manually; zero means unknown.");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.settings.input_price_per_million)
                            .speed(0.1)
                            .prefix("$")
                            .suffix(" / 1M input"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.settings.output_price_per_million)
                            .speed(0.1)
                            .prefix("$")
                            .suffix(" / 1M output"),
                    );
                });

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("Save settings").clicked() {
                        self.save_settings();
                    }
                    if ui
                        .add_enabled(!self.pending, egui::Button::new("Test connection"))
                        .clicked()
                    {
                        self.test_provider(context);
                    }
                    if ui.button("Clear API key").clicked() {
                        self.api_key.clear();
                        self.status = "API key cleared from memory".to_owned();
                    }
                });
                ui.add_space(8.0);
                ui.label(&self.status);
            });
        });
    }

    pub(crate) fn about_help_ui(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Error Explainer");
                ui.label("Local AI-assisted incident triage for logs, stack traces, and debugging questions.");
                let renderer = if self.pending {
                    format!("active {:.0} FPS", self.active_fps)
                } else {
                    "idle (event-driven, no redraw loop)".to_owned()
                };
                ui.label(format!("One process: window + tray · renderer: {renderer}"));
                ui.add_space(12.0);
                ui.heading("Quick start");
                ui.label("1. Open Settings and choose a provider.");
                ui.label("2. Enter a model and, when required, an API key; then test.");
                ui.label("3. Copy logs and press Ctrl+Shift+Alt+C.");
                ui.label("4. Review evidence and verify every proposed fix.");
                ui.add_space(12.0);
                ui.heading("Privacy");
                ui.label("API keys stay in process memory and are never exported or saved.");
                ui.label("Chat history is saved locally for continuity. Clear it from Chat at any time.");
                ui.label("Your selected provider receives the submitted conversation.");
                ui.add_space(12.0);
                ui.heading("Controls");
                ui.label(format!("{HOTKEY_LABEL}: show/hide and paste clipboard when input is empty."));
                ui.label("Ctrl+Enter: send. PIN: keep the window above others. ×: hide to tray.");
                ui.add_space(12.0);
                ui.heading("Safety");
                ui.label("Model output is a hypothesis, not proof. Confirm against source code, runtime state, and reproduction steps.");
                ui.add_space(12.0);
                ui.label("Version 0.1.0 · Rust + egui/eframe");
            });
        });
    }
}

fn empty_state(ui: &mut egui::Ui, colors: crate::theme::Palette, icon: &egui::TextureHandle) {
    ui.vertical_centered(|ui| {
        ui.add_space(36.0);
        ui.image((icon.id(), egui::vec2(112.0, 112.0)));
        ui.add_space(12.0);
        ui.label(
            RichText::new("Paste an error. Get a testable explanation.")
                .size(20.0)
                .strong(),
        );
        ui.add_space(8.0);
        ui.label(RichText::new("Nothing is sent until you press Explain.").color(colors.muted));
    });
}

fn message_card(ui: &mut egui::Ui, message: &ChatMessage, colors: crate::theme::Palette) {
    let user = message.role == "user";
    let accent = if user { colors.user } else { colors.assistant };
    egui::Frame::none()
        .fill(colors.card)
        .stroke(egui::Stroke::new(1.0, colors.border))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 8.0).max(200.0));
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(if user { "YOU" } else { "EE" })
                        .strong()
                        .small()
                        .color(accent),
                );
                ui.label(
                    RichText::new(&message.timestamp)
                        .small()
                        .color(colors.muted),
                );
            });
            if let Some(sections) = &message.sections {
                section(ui, "Likely cause", &sections.cause, colors.user);
                section(ui, "Fix", &sections.fix, colors.assistant);
                section(ui, "Verification", &sections.verify, colors.muted);
            } else {
                ui.add(
                    egui::Label::new(&message.content)
                        .wrap(true)
                        .selectable(true),
                );
            }
            if let Some(image) = &message.image {
                ui.label(
                    RichText::new(format!(
                        "📷 {} · {}×{}",
                        image.name, image.width, image.height
                    ))
                    .small()
                    .color(colors.muted),
                );
            }
        });
    ui.add_space(8.0);
}

fn section(ui: &mut egui::Ui, title: &str, text: &str, accent: Color32) {
    if text.is_empty() {
        return;
    }
    ui.add_space(5.0);
    ui.label(RichText::new(title).strong().small().color(accent));
    ui.add(egui::Label::new(text).wrap(true).selectable(true));
}

pub(crate) fn keep_tail(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    const PREFIX: &str = "[document head truncated]\n";
    let tail_count = max_chars.saturating_sub(PREFIX.chars().count());
    let mut chars: Vec<char> = text.chars().rev().take(tail_count).collect();
    chars.reverse();
    format!("{PREFIX}{}", chars.into_iter().collect::<String>())
}

pub(crate) fn format_usage(answer: &AiAnswer, settings: &ProviderSettings) -> String {
    let input = answer
        .input_tokens
        .map(|tokens| tokens.to_string())
        .unwrap_or_else(|| "?".to_owned());
    let output = answer
        .output_tokens
        .map(|tokens| tokens.to_string())
        .unwrap_or_else(|| "?".to_owned());
    let cost = match (answer.input_tokens, answer.output_tokens) {
        (Some(input), Some(output))
            if settings.input_price_per_million > 0.0
                || settings.output_price_per_million > 0.0 =>
        {
            let value = input as f64 * settings.input_price_per_million / 1_000_000.0
                + output as f64 * settings.output_price_per_million / 1_000_000.0;
            format!(" · ≈${value:.4}")
        }
        _ => String::new(),
    };
    let speed = answer
        .tokens_per_second
        .map(|value| format!(" · {value:.1} tok/s"))
        .unwrap_or_default();
    let ttft = answer
        .time_to_first_token_seconds
        .map(|value| format!(" · TTFT {value:.2}s"))
        .unwrap_or_default();
    format!("Request {input} tokens · Response {output} tokens{cost}{speed}{ttft}")
}

pub(crate) fn first_line(text: &str) -> String {
    text.lines()
        .next()
        .unwrap_or("ok")
        .chars()
        .take(120)
        .collect()
}
