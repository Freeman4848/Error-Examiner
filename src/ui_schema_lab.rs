use crate::theme::palette;
use crate::*;

#[derive(Default)]
pub(crate) struct SchemaUiState {
    selected_profile: Option<String>,
    profile_preview: String,
    log_name: String,
    log_raw: String,
    index_status: String,
    existing_profile: Option<String>,
    update_target: Option<String>,
    update_file_confirmed: bool,
    restore_confirm: bool,
    pub(crate) scroll_to_response: bool,
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
                ScrollArea::vertical()
                    .id_source("schema_page_scroll")
                    .show(ui, |ui| {
                ui.heading("Schema");
                ui.label(
                    RichText::new("Active parser profile index")
                        .color(colors.muted),
                );
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{} active", self.schema_registry.active))
                            .size(24.0)
                            .strong()
                            .color(colors.assistant),
                    );
                    ui.label(format!(
                        "{} built-in · {} user",
                        self.schema_registry.built_in, self.schema_registry.user
                    ));
                    if ui.button("Reload").clicked() {
                        let before = self.schema_registry.profiles.len();
                        self.schema_registry = parser_registry::reload_user_schemas();
                        let after = self.schema_registry.profiles.len();
                        self.schema_ui.index_status = if before == after {
                            format!("Reloaded {after} · no changes")
                        } else {
                            format!("Reloaded {before} → {after}")
                        };
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
                self.schema_profile_actions(ui, colors);
                if !self.schema_ui.index_status.is_empty() {
                    ui.label(
                        RichText::new(&self.schema_ui.index_status)
                            .small()
                            .color(colors.muted),
                    );
                }
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
                    self.schema_ui.update_target = None;
                    self.schema_ui.update_file_confirmed = true;
                    self.pick_schema_log();
                }
                if !self.schema_ui.log_name.is_empty() {
                    let total = self.schema_ui.log_raw.chars().count();
                    let sent = total.min(5_000);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&self.schema_ui.log_name).strong());
                        ui.label(format!("AI sample {sent} / {total} chars"));
                    });
                    if total > 5_000 {
                        ui.label("2500 head + 2500 tail; full log stays local for validation.");
                    }
                    if self.schema_ui.update_target.is_some()
                        && !self.schema_ui.update_file_confirmed
                    {
                        ui.colored_label(
                            colors.user,
                            "Unknown log type: confirm it belongs to the selected profile.",
                        );
                        if ui.button("Confirm update log").clicked() {
                            self.schema_ui.update_file_confirmed = true;
                        }
                    }
                }
                if ui
                    .add_enabled_ui(
                            !self.schema_pending
                                && !self.schema_ui.log_raw.is_empty()
                                && (self.schema_ui.existing_profile.is_none()
                                    || self.schema_ui.update_target.is_some())
                                && (self.schema_ui.update_target.is_none()
                                    || self.schema_ui.update_file_confirmed),
                        |ui| {
                            ui.add_sized(
                                [ui.available_width(), 46.0],
                                egui::Button::new(
                                    RichText::new(if self.schema_pending {
                                        "Generating and validating…"
                                    } else {
                                        "Generate schema draft"
                                    })
                                    .strong(),
                                ),
                            )
                        },
                    )
                    .inner
                    .clicked()
                    {
                        self.start_schema_generation(context);
                    }
                    if let Some(profile) = &self.schema_ui.existing_profile {
                        ui.colored_label(
                            colors.assistant,
                            format!("Already covered by {profile}; a new schema is not needed."),
                        );
                    } else if !self.schema_ui.log_raw.is_empty() {
                        ui.label("After generation this page will scroll to Draft and Model response below.");
                    }
                ui.label(RichText::new(&self.schema_status).color(colors.muted));
                self.schema_draft_preview(ui);
                    });
            });
    }

    fn schema_profile_actions(&mut self, ui: &mut egui::Ui, colors: crate::theme::Palette) {
        let selected = self.schema_ui.selected_profile.clone();
        let profile = selected.as_ref().and_then(|id| {
            self.schema_registry
                .profiles
                .iter()
                .find(|profile| &profile.id == id)
                .cloned()
        });
        ui.horizontal(|ui| {
            if let Some(profile) = &profile {
                let label = if profile.active { "Disable" } else { "Enable" };
                if ui.button(label).clicked() {
                    match parser_registry::set_profile_disabled(&profile.id, profile.active) {
                        Ok(registry) => {
                            self.schema_registry = registry;
                            self.schema_status = format!("{label}d: {}", profile.id);
                        }
                        Err(error) => self.schema_status = format!("Schema state error: {error}"),
                    }
                }
                if ui.button("Update").clicked() {
                    self.schema_ui.update_target = Some(profile.id.clone());
                    self.schema_ui.update_file_confirmed = false;
                    self.pick_schema_log();
                }
            }
            if ui.button("Restore defaults").clicked() {
                self.schema_ui.restore_confirm = true;
            }
        });
        ui.label(
            RichText::new(
                "Disable is reversible. Update creates an override; defaults stay intact.",
            )
            .small()
            .color(colors.muted),
        );
        if self.schema_ui.restore_confirm {
            ui.horizontal(|ui| {
                ui.colored_label(colors.user, "Restore built-ins and archive all overrides?");
                if ui.button("Confirm restore").clicked() {
                    match parser_registry::restore_defaults() {
                        Ok((registry, message)) => {
                            self.schema_registry = registry;
                            self.schema_status = message;
                        }
                        Err(error) => self.schema_status = error,
                    }
                    self.schema_ui.restore_confirm = false;
                }
                if ui.button("Cancel").clicked() {
                    self.schema_ui.restore_confirm = false;
                }
            });
        }
    }

    fn schema_index(&mut self, ui: &mut egui::Ui, colors: crate::theme::Palette) {
        let profiles = self.schema_registry.profiles.clone();
        schema_table_header(ui, colors);
        ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
            for (index, profile) in profiles.into_iter().enumerate() {
                let selected = self.schema_ui.selected_profile.as_deref() == Some(&profile.id);
                if schema_table_row(ui, &profile, selected, index, colors) {
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
                let total = text.chars().count();
                let parsed = parser_schema::parse(&text);
                if let Some(target) = &self.schema_ui.update_target {
                    match schema_lab::update_gate(target, parsed.supported, &parsed.format_ids) {
                        schema_lab::UpdateGate::Accepted => {
                            self.schema_ui.update_file_confirmed = true;
                        }
                        schema_lab::UpdateGate::ConfirmUnknown => {
                            self.schema_ui.update_file_confirmed = false;
                        }
                        schema_lab::UpdateGate::Rejected(error) => {
                            self.schema_ui.log_name.clear();
                            self.schema_ui.log_raw.clear();
                            self.schema_draft = None;
                            self.schema_status = error;
                            return;
                        }
                    }
                }
                self.schema_ui.existing_profile = (self.schema_ui.update_target.is_none()
                    && parsed.supported)
                    .then(|| parsed.format_ids.join(" + "));
                self.schema_ui.log_name = name;
                self.schema_ui.log_raw = text;
                self.schema_draft = None;
                self.schema_status = format!(
                    "Log loaded: AI receives {} / {total} chars; full validation is local.",
                    total.min(5_000)
                );
            }
            Ok(Some(attachment::InputFile::Image(_))) => {
                self.schema_status = "Schema generation accepts text logs, not images.".into();
            }
            Ok(None) => {}
            Err(error) => self.schema_status = format!("Log error: {error}"),
        }
    }

    fn start_schema_generation(&mut self, context: &egui::Context) {
        if self.schema_ui.log_raw.is_empty()
            || (self.schema_ui.existing_profile.is_some() && self.schema_ui.update_target.is_none())
        {
            return;
        }
        self.schema_pending = true;
        self.schema_draft = None;
        self.schema_status = format!("Generating with {}…", self.settings.provider.label());
        let settings = self.settings.clone();
        let api_key = self.api_key.clone();
        let name = self.schema_ui.log_name.clone();
        let raw = self.schema_ui.log_raw.clone();
        let update_target = self.schema_ui.update_target.clone();
        let update_file_confirmed = self.schema_ui.update_file_confirmed;
        let sender = self.schema_sender.clone();
        let repaint = context.clone();
        std::thread::spawn(move || {
            let result = schema_lab::generate(
                settings,
                api_key,
                name,
                raw,
                update_target,
                update_file_confirmed,
            );
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
            if self.schema_ui.scroll_to_response {
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                self.schema_ui.scroll_to_response = false;
            }
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
                self.schema_ui.update_target = None;
            }
            Err(error) => self.schema_status = format!("Activation failed: {error}"),
        }
    }
}

fn schema_table_header(ui: &mut egui::Ui, colors: crate::theme::Palette) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 32.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 4.0, colors.card);
    paint_table_lines(painter, rect, colors.border);
    paint_table_text(
        painter,
        rect,
        "Profile",
        "Application",
        "Origin",
        colors.text,
        true,
    );
}

fn schema_table_row(
    ui: &mut egui::Ui,
    profile: &parser_registry::ProfileSummary,
    selected: bool,
    index: usize,
    colors: crate::theme::Palette,
) -> bool {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::click());
    let fill = if selected {
        colors.accent
    } else if response.hovered() {
        colors.border
    } else if index % 2 == 1 {
        colors.card
    } else {
        Color32::TRANSPARENT
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, fill);
    paint_table_lines(painter, rect, colors.border);
    paint_table_text(
        painter,
        rect,
        &profile.id,
        &profile.application,
        &if profile.active {
            profile.origin.clone()
        } else {
            format!("{} · disabled", profile.origin)
        },
        colors.text,
        false,
    );
    response.clicked()
}

fn paint_table_lines(painter: &egui::Painter, rect: egui::Rect, color: Color32) {
    let (first, second) = table_dividers(rect);
    let stroke = egui::Stroke::new(1.0, color);
    painter.line_segment(
        [
            egui::pos2(first, rect.top()),
            egui::pos2(first, rect.bottom()),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(second, rect.top()),
            egui::pos2(second, rect.bottom()),
        ],
        stroke,
    );
    painter.line_segment([rect.left_bottom(), rect.right_bottom()], stroke);
}

fn paint_table_text(
    painter: &egui::Painter,
    rect: egui::Rect,
    profile: &str,
    application: &str,
    origin: &str,
    color: Color32,
    strong: bool,
) {
    let (first, second) = table_dividers(rect);
    let font = egui::FontId::proportional(if strong { 14.0 } else { 13.5 });
    let cells = [
        (rect.left(), first, profile),
        (first, second, application),
        (second, rect.right(), origin),
    ];
    for (left, right, text) in cells {
        let clip = egui::Rect::from_min_max(
            egui::pos2(left + 1.0, rect.top()),
            egui::pos2(right - 1.0, rect.bottom()),
        );
        painter.with_clip_rect(clip).text(
            egui::pos2(left + 9.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            text,
            font.clone(),
            color,
        );
    }
}

fn table_dividers(rect: egui::Rect) -> (f32, f32) {
    (
        rect.left() + rect.width() * 0.38,
        rect.left() + rect.width() * 0.84,
    )
}
