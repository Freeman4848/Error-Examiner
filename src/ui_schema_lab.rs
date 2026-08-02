use crate::theme::palette;
use crate::*;

#[derive(Default)]
pub(crate) struct SchemaUiState {
    selected_profile: Option<String>,
    profile_preview: String,
    log_name: String,
    log_raw: String,
}

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
                ui.heading("Schema");
                ui.label(
                    RichText::new("Active parser profile index")
                        .color(colors.muted),
                );
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{} active", self.schema_registry.profiles.len()))
                            .size(24.0)
                            .strong()
                            .color(colors.assistant),
                    );
                    ui.label(format!(
                        "{} built-in · {} user",
                        self.schema_registry.built_in, self.schema_registry.user
                    ));
                    if ui.button("Reload").clicked() {
                        self.schema_registry = parser_registry::reload_user_schemas();
                    }
                });

                self.schema_index(ui, colors);
                ui.separator();
                ui.heading("New schema");
                ui.label("Only 5000 characters (head + tail) are sent to the selected AI. Full log validation stays local.");
                ui.horizontal(|ui| {
                    if ui.button("Add · choose log").clicked() {
                        self.pick_schema_log();
                    }
                    if !self.schema_ui.log_name.is_empty() {
                        ui.label(RichText::new(&self.schema_ui.log_name).strong());
                        ui.label(format!("{} chars", self.schema_ui.log_raw.chars().count()));
                    }
                });
                if ui
                    .add_enabled(
                        !self.schema_pending && !self.schema_ui.log_raw.is_empty(),
                        egui::Button::new(if self.schema_pending {
                            "Generating and validating…"
                        } else {
                            "Generate draft"
                        }),
                    )
                    .clicked()
                {
                    self.start_schema_generation(context);
                }
                ui.label(RichText::new(&self.schema_status).color(colors.muted));
                self.schema_draft_preview(ui);
            });
    }

    fn schema_index(&mut self, ui: &mut egui::Ui, colors: crate::theme::Palette) {
        let profiles = self.schema_registry.profiles.clone();
        ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
            egui::Grid::new("schema_index")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.label(RichText::new("Profile").strong());
                    ui.label(RichText::new("Application").strong());
                    ui.label(RichText::new("Origin").strong());
                    ui.end_row();
                    for profile in profiles {
                        let selected =
                            self.schema_ui.selected_profile.as_deref() == Some(&profile.id);
                        if ui.selectable_label(selected, &profile.id).clicked() {
                            self.schema_ui.selected_profile = Some(profile.id.clone());
                        }
                        ui.label(profile.application);
                        ui.colored_label(colors.muted, profile.origin);
                        ui.end_row();
                    }
                });
        });
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    self.schema_ui.selected_profile.is_some(),
                    egui::Button::new("Preview selected"),
                )
                .clicked()
            {
                if let Some(id) = &self.schema_ui.selected_profile {
                    match parser_registry::profile_json(id) {
                        Ok(json) => self.schema_ui.profile_preview = json,
                        Err(error) => self.schema_status = error,
                    }
                }
            }
            if !self.schema_registry.rejected.is_empty() {
                ui.colored_label(
                    colors.user,
                    format!("{} rejected", self.schema_registry.rejected.len()),
                );
            }
        });
        if !self.schema_ui.profile_preview.is_empty() {
            ui.collapsing("Selected schema preview", |ui| {
                if ui.button("Copy JSON").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.schema_ui.profile_preview.clone()
                    });
                }
                ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(&self.schema_ui.profile_preview).monospace(),
                        )
                        .selectable(true),
                    );
                });
            });
        }
    }

    fn pick_schema_log(&mut self) {
        match attachment::pick_input_file() {
            Ok(Some(attachment::InputFile::Log { name, text })) => {
                self.schema_ui.log_name = name;
                self.schema_ui.log_raw = text;
                self.schema_draft = None;
                self.schema_status = "Log loaded locally; ready to generate.".into();
            }
            Ok(Some(attachment::InputFile::Image(_))) => {
                self.schema_status = "Schema generation accepts text logs, not images.".into();
            }
            Ok(None) => {}
            Err(error) => self.schema_status = format!("Log error: {error}"),
        }
    }

    fn start_schema_generation(&mut self, context: &egui::Context) {
        if self.schema_ui.log_raw.is_empty() {
            return;
        }
        self.schema_pending = true;
        self.schema_draft = None;
        self.schema_status = format!("Generating with {}…", self.settings.provider.label());
        let settings = self.settings.clone();
        let api_key = self.api_key.clone();
        let name = self.schema_ui.log_name.clone();
        let raw = self.schema_ui.log_raw.clone();
        let sender = self.schema_sender.clone();
        let repaint = context.clone();
        std::thread::spawn(move || {
            let result = schema_lab::generate(settings, api_key, name, raw);
            let _ = sender.send(result);
            repaint.request_repaint();
        });
    }

    fn schema_draft_preview(&mut self, ui: &mut egui::Ui) {
        let mut install = false;
        if let Some(draft) = &self.schema_draft {
            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Saved draft preview");
                if ui.button("Copy JSON").clicked() {
                    ui.output_mut(|output| output.copied_text = draft.json.clone());
                }
                install = ui.button("Confirm and activate").clicked();
            });
            ui.label(format!("Draft: {}", draft.draft_path.display()));
            ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                ui.add(egui::Label::new(RichText::new(&draft.json).monospace()).selectable(true));
            });
        }
        if install {
            self.install_schema_draft();
        }
    }

    fn install_schema_draft(&mut self) {
        let Some(draft) = &self.schema_draft else {
            return;
        };
        match schema_lab::install(draft) {
            Ok(path) => {
                self.schema_registry = parser_registry::reload_user_schemas();
                self.schema_status = format!("Activated: {path}");
                self.schema_draft = None;
            }
            Err(error) => self.schema_status = format!("Activation failed: {error}"),
        }
    }
}
