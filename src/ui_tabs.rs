use crate::*;

impl ErrorExaminerApp {
    pub(crate) fn chat_tabs_ui(&mut self, ui: &mut egui::Ui) {
        let tabs: Vec<(usize, String)> = self
            .chat_tabs
            .iter()
            .enumerate()
            .map(|(index, tab)| (index, tab.title.clone()))
            .collect();
        let mut switch_to = None;
        let mut close = None;
        let mut save = false;
        ui.horizontal_wrapped(|ui| {
            for (index, mut title) in tabs {
                if self.active_chat == index {
                    let edit = ui.add_sized(
                        [150.0, 24.0],
                        TextEdit::singleline(&mut title).hint_text("Chat name"),
                    );
                    if edit.changed() {
                        self.chat_tabs[index].title = title;
                    }
                    save |= edit.lost_focus();
                } else if ui
                    .add_enabled(!self.pending, egui::SelectableLabel::new(false, title))
                    .clicked()
                {
                    switch_to = Some(index);
                }
                if self.chat_tabs.len() > 1
                    && ui
                        .add_enabled(!self.pending, egui::Button::new("×").frame(false))
                        .clicked()
                {
                    close = Some(index);
                }
                ui.separator();
            }
            if ui
                .add_enabled(!self.pending, egui::Button::new("+"))
                .on_hover_text("New chat")
                .clicked()
            {
                self.new_chat();
            }
        });
        if let Some(index) = switch_to {
            self.switch_chat(index);
        }
        if let Some(index) = close {
            self.close_chat(index);
        }
        if save {
            self.save_history();
        }
    }
}
