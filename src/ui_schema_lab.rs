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
                    if ui
                        .add_enabled(
                            self.schema_ui.selected_profile.is_some(),
                            egui::Button::new("Preview selected"),
                        )
                        .clicked()
                    {
                        self.preview_selected_schema();
                    }
                });
                self.selected_schema_preview(ui);
                self.schema_index(ui, colors);
                ui.separator();
                ui.heading("New schema");
                ui.label("Only 5000 characters (head + tail) are sent to the selected AI. Full log validation stays local.");
                if ui
                    .add_sized(
                        [ui.available_width(), 44.0],
                        egui::Button::new(RichText::new("+  Add log file").strong()),
                    )
                    .clicked()
                {
                    self.pick_schema_log();
                }
                if !self.schema_ui.log_name.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&self.schema_ui.log_name).strong());
                        ui.label(format!("{} chars", self.schema_ui.log_raw.chars().count()));
                    });
                }
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
            for profile in profiles {
                let selected = self.schema_ui.selected_profile.as_deref() == Some(&profile.id);
                let row = format!(
                    "{}   ·   {}   ·   {}",
                    profile.id, profile.application, profile.origin
                );
                if ui
                    .add_sized(
                        [ui.available_width(), 28.0],
                        egui::SelectableLabel::new(selected, row),
                    )
                    .clicked()
                {
                    self.schema_ui.selected_profile = Some(profile.id);
                }
            }
        });
        if !self.schema_registry.rejected.is_empty() {
            ui.colored_label(
                colors.user,
                format!("{} rejected", self.schema_registry.rejected.len()),
            );
        }
    }

    fn preview_selected_schema(&mut self) {
        let Some(id) = &self.schema_ui.selected_profile else {
            return;
        };
        match parser_registry::profile_json(id) {
            Ok(json) => self.schema_ui.profile_preview = json,
            Err(error) => self.schema_status = error,
        }
    }

    fn selected_schema_preview(&mut self, ui: &mut egui::Ui) {
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
            ui.separator();
            ui.label(RichText::new("MODEL RESPONSE").strong());
            egui::Frame::none()
                .fill(ui.visuals().faint_bg_color)
                .rounding(egui::Rounding::same(10.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.label(&draft.model_answer);
                    ui.label(
                        RichText::new(format!("Saved: {}", draft.response_path.display())).small(),
                    );
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
