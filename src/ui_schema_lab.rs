use crate::theme::palette;
use crate::*;

impl ErrorExplainerApp {
    pub(crate) fn schema_lab_ui(&mut self, context: &egui::Context) {
        let colors = palette(self.theme, self.opacity);
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(colors.background)
                    .inner_margin(egui::Margin::same(18.0)),
            )
            .show(context, |ui| {
                ui.heading("Schema Lab");
                ui.label(
                    RichText::new("Generate and validate a parser schema separately from chat.")
                        .color(colors.muted),
                );
                ui.add_space(20.0);

                egui::Frame::none()
                    .fill(colors.card)
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(18.0))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("VERIFIED PARSER PROFILES")
                                .small()
                                .color(colors.muted),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{}",
                                self.schema_registry.built_in + self.schema_registry.user
                            ))
                            .size(34.0)
                            .strong()
                            .color(colors.assistant),
                        );
                        ui.label(format!(
                            "{} built-in · {} installed user profiles",
                            self.schema_registry.built_in, self.schema_registry.user
                        ));
                    });

                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    if ui.button("Reload installed schemas").clicked() {
                        self.schema_registry = parser_registry::reload_user_schemas();
                    }
                    if !self.schema_registry.rejected.is_empty() {
                        ui.colored_label(
                            colors.user,
                            format!(
                                "{} schema file(s) rejected",
                                self.schema_registry.rejected.len()
                            ),
                        );
                    }
                });

                ui.separator();
                if let Some(log) = &self.prepared_log {
                    ui.label(RichText::new(format!("Current file: {}", log.name)).strong());
                    ui.label(format!(
                        "Detected: {}",
                        if log.detected_format.is_empty() {
                            "Raw / unknown"
                        } else {
                            &log.detected_format
                        }
                    ));
                    ui.label(format!(
                        "{} characters available for schema validation",
                        log.original_chars
                    ));
                    if ui
                        .add_enabled(
                            !self.schema_pending,
                            egui::Button::new(if self.schema_pending {
                                "Generating and validating…"
                            } else {
                                "Generate new schema"
                            }),
                        )
                        .clicked()
                    {
                        self.start_schema_generation(context);
                    }
                } else {
                    ui.label("Load a log in Chat; Schema Lab will use that current file.");
                    ui.add_enabled(false, egui::Button::new("Generate new schema"));
                }
                ui.label(RichText::new(&self.schema_status).color(colors.muted));

                let mut install = false;
                if let Some(draft) = &self.schema_draft {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.heading("Validated schema preview");
                        if ui.button("Copy JSON").clicked() {
                            ui.output_mut(|output| output.copied_text = draft.json.clone());
                        }
                        install = ui.button("Install schema").clicked();
                    });
                    ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(&mut draft.json.clone())
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .interactive(false),
                        );
                    });
                }
                if install {
                    self.install_schema_draft();
                }
            });
    }

    fn start_schema_generation(&mut self, context: &egui::Context) {
        let Some(log) = &self.prepared_log else {
            return;
        };
        self.schema_pending = true;
        self.schema_draft = None;
        self.schema_status = format!("Generating with {}…", self.settings.provider.label());
        let settings = self.settings.clone();
        let api_key = self.api_key.clone();
        let name = log.name.clone();
        let raw = log.raw_preview.clone();
        let sender = self.schema_sender.clone();
        let repaint = context.clone();
        std::thread::spawn(move || {
            let result = schema_lab::generate(settings, api_key, name, raw);
            let _ = sender.send(result);
            repaint.request_repaint();
        });
    }

    fn install_schema_draft(&mut self) {
        let Some(draft) = &self.schema_draft else {
            return;
        };
        match schema_lab::install(draft) {
            Ok(path) => {
                self.schema_registry = parser_registry::reload_user_schemas();
                self.schema_status = format!("Installed locally: {path}");
            }
            Err(error) => self.schema_status = format!("Install failed: {error}"),
        }
    }
}
