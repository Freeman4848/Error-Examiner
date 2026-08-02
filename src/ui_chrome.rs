use crate::theme::palette;
use crate::*;

impl ErrorExplainerApp {
    pub(crate) fn top_bar(&mut self, context: &egui::Context) {
        let colors = palette(self.theme, self.opacity);
        egui::TopBottomPanel::top("window_chrome")
            .exact_height(78.0)
            .frame(
                egui::Frame::none()
                    .fill(colors.panel)
                    .inner_margin(egui::Margin::symmetric(14.0, 5.0))
                    .stroke(egui::Stroke::new(0.5, colors.border)),
            )
            .show(context, |ui| {
                ui.horizontal(|ui| {
                    let error = ui.add(
                        egui::Label::new(
                            RichText::new("Error")
                                .strong()
                                .size(15.0)
                                .color(colors.user),
                        )
                        .sense(egui::Sense::click_and_drag()),
                    );
                    let explainer = ui.add(
                        egui::Label::new(
                            RichText::new("Explainer")
                                .strong()
                                .size(15.0)
                                .color(colors.assistant),
                        )
                        .sense(egui::Sense::click_and_drag()),
                    );
                    if (error.drag_started() || explainer.drag_started()) && !self.window_locked {
                        context.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    let icon_rect = egui::Rect::from_center_size(
                        egui::pos2(ui.max_rect().center().x, error.rect.center().y),
                        egui::vec2(30.0, 30.0),
                    );
                    ui.painter().image(
                        self.app_icon_texture.id(),
                        icon_rect,
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("×").on_hover_text("Hide to tray").clicked() {
                            self.hide(context);
                        }
                        let maximized =
                            context.input(|input| input.viewport().maximized.unwrap_or(false));
                        if ui
                            .button(if maximized { "❐" } else { "□" })
                            .on_hover_text("Maximize / restore")
                            .clicked()
                        {
                            context.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
                        }
                        if ui.button("—").on_hover_text("Minimize").clicked() {
                            context.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                        if ui
                            .selectable_label(self.window_locked, "WinLock")
                            .on_hover_text("Lock moving and resizing")
                            .clicked()
                        {
                            self.window_locked = !self.window_locked;
                        }
                        if ui
                            .selectable_label(self.pin_top, "Pin")
                            .on_hover_text("Always on top")
                            .clicked()
                        {
                            self.pin_top = !self.pin_top;
                            context.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                                if self.pin_top {
                                    egui::WindowLevel::AlwaysOnTop
                                } else {
                                    egui::WindowLevel::Normal
                                },
                            ));
                        }
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    for (page, label) in [
                        (Page::Chat, "Explain"),
                        (Page::SchemaLab, "Schema"),
                        (Page::Settings, "Settings"),
                        (Page::AboutHelp, "Help"),
                    ] {
                        if ui.selectable_label(self.page == page, label).clicked() {
                            self.page = page;
                        }
                    }
                });
            });
    }
}

pub(crate) fn window_resize(context: &egui::Context, locked: bool) {
    use egui::{CursorIcon, ResizeDirection, ViewportCommand};
    if locked {
        return;
    }
    let rect = context.screen_rect();
    let Some(pointer) = context.pointer_hover_pos() else {
        return;
    };
    let edge = 12.0;
    let left = pointer.x <= rect.left() + edge;
    let right = pointer.x >= rect.right() - edge;
    let top = pointer.y <= rect.top() + edge;
    let bottom = pointer.y >= rect.bottom() - edge;
    let direction = match (left, right, top, bottom) {
        (true, _, true, _) => Some((ResizeDirection::NorthWest, CursorIcon::ResizeNwSe)),
        (_, true, true, _) => Some((ResizeDirection::NorthEast, CursorIcon::ResizeNeSw)),
        (true, _, _, true) => Some((ResizeDirection::SouthWest, CursorIcon::ResizeNeSw)),
        (_, true, _, true) => Some((ResizeDirection::SouthEast, CursorIcon::ResizeNwSe)),
        (true, _, _, _) => Some((ResizeDirection::West, CursorIcon::ResizeHorizontal)),
        (_, true, _, _) => Some((ResizeDirection::East, CursorIcon::ResizeHorizontal)),
        (_, _, true, _) => Some((ResizeDirection::North, CursorIcon::ResizeVertical)),
        (_, _, _, true) => Some((ResizeDirection::South, CursorIcon::ResizeVertical)),
        _ => None,
    };
    if let Some((direction, cursor)) = direction {
        context.set_cursor_icon(cursor);
        if context.input(|input| input.pointer.primary_pressed()) {
            context.send_viewport_cmd(ViewportCommand::BeginResize(direction));
        }
    }
}
