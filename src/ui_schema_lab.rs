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
                            RichText::new("VERIFIED APPLICATION TYPES")
                                .small()
                                .color(colors.muted),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} / {}",
                                self.schema_coverage.covered, self.schema_coverage.total
                            ))
                            .size(34.0)
                            .strong()
                            .color(colors.assistant),
                        );
                        ui.label(format!(
                            "{} built-in parser formats · {} installed user formats",
                            self.schema_registry.built_in, self.schema_registry.user
                        ));
                    });

                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    if ui.button("Reload installed schemas").clicked() {
                        self.schema_registry = parser_registry::reload_user_schemas();
                        self.schema_coverage = parser_registry::coverage();
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
                    ui.add_enabled(false, egui::Button::new("Generate new schema"))
                        .on_disabled_hover_text("Generator is the next isolated block.");
                } else {
                    ui.label("Load a log in Chat; Schema Lab will use that current file.");
                    ui.add_enabled(false, egui::Button::new("Generate new schema"));
                }
            });
    }
}
