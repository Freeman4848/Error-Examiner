use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreviewMode {
    Raw,
    Parsed,
    Compare,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreviewPane {
    Raw,
    Parsed,
}

impl ErrorExplainerApp {
    pub(crate) fn log_preview_window(&mut self, context: &egui::Context) {
        if !self.preview_open {
            return;
        }
        let Some(log) = self.prepared_log.clone() else {
            self.preview_open = false;
            return;
        };
        let mut builder = egui::ViewportBuilder::default()
            .with_title(format!("Log preview · {}", log.name))
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([520.0, 360.0])
            .with_resizable(true)
            .with_decorations(true);
        if !self.preview_created {
            builder = builder.with_maximized(true);
        }
        let mode = &mut self.preview_mode;
        let active_pane = &mut self.preview_pane;
        let accent = palette(self.theme, self.opacity).accent;
        let close_requested = context.show_viewport_immediate(
            egui::ViewportId::from_hash_of("log_preview"),
            builder,
            |preview_context, _class| {
                egui::TopBottomPanel::top("preview_modes").show(preview_context, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(mode, PreviewMode::Raw, "Raw");
                        ui.selectable_value(mode, PreviewMode::Parsed, "Parsed");
                        ui.selectable_value(mode, PreviewMode::Compare, "Compare");
                        ui.separator();
                        ui.label(format!(
                            "{} -> {} chars · ~{} standard tokens",
                            log.original_chars, log.normalized_chars, log.estimated_tokens
                        ));
                    });
                });
                egui::CentralPanel::default().show(preview_context, |ui| match mode {
                    PreviewMode::Raw => {
                        preview_text(ui, "raw", "Raw log", &log.raw_preview, true, accent);
                    }
                    PreviewMode::Parsed => {
                        preview_text(
                            ui,
                            "parsed",
                            "Prepared log",
                            &log.parsed_preview,
                            true,
                            accent,
                        );
                    }
                    PreviewMode::Compare => {
                        ui.columns(2, |columns| {
                            if preview_text(
                                &mut columns[0],
                                "compare_raw",
                                "Raw log",
                                &log.raw_preview,
                                *active_pane == PreviewPane::Raw,
                                accent,
                            ) {
                                *active_pane = PreviewPane::Raw;
                            }
                            if preview_text(
                                &mut columns[1],
                                "compare_parsed",
                                "Prepared log",
                                &log.parsed_preview,
                                *active_pane == PreviewPane::Parsed,
                                accent,
                            ) {
                                *active_pane = PreviewPane::Parsed;
                            }
                        });
                    }
                });
                preview_context.input(|input| input.viewport().close_requested())
            },
        );
        self.preview_created = true;
        if close_requested {
            self.preview_open = false;
            self.preview_created = false;
        }
    }
}

fn preview_text(
    ui: &mut egui::Ui,
    id: &str,
    heading: &str,
    text: &str,
    active: bool,
    accent: Color32,
) -> bool {
    let pane_rect = ui.available_rect_before_wrap();
    let clicked = ui
        .interact(
            pane_rect,
            ui.id().with(id).with("focus"),
            egui::Sense::click(),
        )
        .clicked();
    egui::Frame::none()
        .stroke(egui::Stroke::new(
            if active { 2.0 } else { 1.0 },
            if active {
                accent
            } else {
                ui.visuals().widgets.noninteractive.bg_stroke.color
            },
        ))
        .inner_margin(egui::Margin::same(6.0))
        .show(ui, |ui| {
            ui.label(RichText::new(heading).heading().color(if active {
                accent
            } else {
                ui.visuals().text_color()
            }));
            ui.separator();
            ScrollArea::both()
                .id_source(id)
                .enable_scrolling(active)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(egui::Label::new(text).wrap(false).selectable(true));
                });
        });
    clicked
}
