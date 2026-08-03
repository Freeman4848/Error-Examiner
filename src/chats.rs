use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct Workspace {
    pub(crate) tabs: Vec<ChatTab>,
    pub(crate) active: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct ChatTab {
    pub(crate) title: String,
    pub(crate) input: String,
    pub(crate) messages: Vec<ChatMessage>,
}

impl ErrorExaminerApp {
    fn sync_active_chat(&mut self) {
        if let Some(tab) = self.chat_tabs.get_mut(self.active_chat) {
            tab.input = self.input.clone();
            tab.messages = self.messages.clone();
        }
    }

    pub(crate) fn save_history(&mut self) {
        self.sync_active_chat();
        let workspace = Workspace {
            tabs: self.chat_tabs.clone(),
            active: self.active_chat,
        };
        if let Err(error) = storage::save_json("chats.json", &workspace) {
            self.status = format!("Chat save failed: {error}");
        }
    }

    pub(crate) fn trim_saved_history(&mut self) {
        if self.messages.len() > 100 {
            self.messages.drain(0..self.messages.len() - 100);
        }
    }

    pub(crate) fn clear_history(&mut self) {
        self.messages.clear();
        self.input.clear();
        self.attachment = None;
        self.prepared_log = None;
        self.save_history();
        self.status = "Current chat cleared".to_owned();
    }

    pub(crate) fn new_chat(&mut self) {
        self.sync_active_chat();
        let number = self.chat_tabs.len() + 1;
        self.chat_tabs.push(ChatTab {
            title: format!("Chat {number}"),
            ..Default::default()
        });
        self.active_chat = self.chat_tabs.len() - 1;
        self.load_active_chat();
        self.save_history();
    }

    pub(crate) fn switch_chat(&mut self, index: usize) {
        if index == self.active_chat || index >= self.chat_tabs.len() {
            return;
        }
        self.sync_active_chat();
        self.active_chat = index;
        self.load_active_chat();
        self.save_history();
    }

    pub(crate) fn close_chat(&mut self, index: usize) {
        if self.chat_tabs.len() <= 1 || index >= self.chat_tabs.len() {
            return;
        }
        self.sync_active_chat();
        self.chat_tabs.remove(index);
        if index < self.active_chat {
            self.active_chat -= 1;
        } else if self.active_chat >= self.chat_tabs.len() {
            self.active_chat = self.chat_tabs.len() - 1;
        }
        self.load_active_chat();
        self.save_history();
    }

    fn load_active_chat(&mut self) {
        if let Some(tab) = self.chat_tabs.get(self.active_chat).cloned() {
            self.input = tab.input;
            self.messages = tab.messages;
            self.attachment = None;
            self.prepared_log = None;
            self.last_usage.clear();
        }
    }

    pub(crate) fn name_active_chat(&mut self, title: &str) {
        let Some(tab) = self.chat_tabs.get_mut(self.active_chat) else {
            return;
        };
        if tab.title.starts_with("Chat ") {
            let title: String = title.chars().take(28).collect();
            if !title.trim().is_empty() {
                tab.title = title;
            }
        }
    }
}
