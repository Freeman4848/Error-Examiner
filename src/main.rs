#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod ai;
mod command;
mod storage;
#[cfg(target_os = "windows")]
mod windows_tray;

use ai::{AiAnswer, AiRequest, ChatMessage, ProviderKind, ProviderSettings};
use command::UiCommand;
use eframe::egui::{self, Color32, FontId, Key, RichText, ScrollArea, TextEdit, Vec2};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::mpsc,
    time::{Duration, Instant},
};

const APP_NAME: &str = "Error Explainer";
const HOTKEY_LABEL: &str = "Ctrl+Shift+Alt+C";

fn main() -> eframe::Result<()> {
    let startup_command = command_from_args();
    let command_receiver = match command::start_server() {
        Ok(receiver) => receiver,
        Err(_) => {
            let _ = command::send(startup_command.unwrap_or(UiCommand::Open));
            return Ok(());
        }
    };

    let start_hidden =
        cfg!(target_os = "windows") || env::var("EE_START_HIDDEN").is_ok_and(|value| value == "1");
    let window_config: WindowConfig = storage::load_json("window.json");
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(APP_NAME)
        .with_inner_size(window_config.size.unwrap_or([720.0, 620.0]))
        .with_min_inner_size([420.0, 360.0])
        .with_resizable(true)
        .with_decorations(false)
        .with_visible(!start_hidden)
        .with_icon(load_app_icon());
    if let Some(position) = window_config.position {
        viewport = viewport.with_position(position);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(move |creation_context| {
            Box::new(ErrorExplainerApp::new(
                creation_context,
                command_receiver,
                start_hidden,
                startup_command,
            ))
        }),
    )
}

fn command_from_args() -> Option<UiCommand> {
    env::args()
        .skip(1)
        .find_map(|argument| match argument.as_str() {
            "--open" => Some(UiCommand::Open),
            "--settings" => Some(UiCommand::Settings),
            "--help" => Some(UiCommand::Help),
            "--exit" => Some(UiCommand::Exit),
            _ => None,
        })
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct WindowConfig {
    position: Option<[f32; 2]>,
    size: Option<[f32; 2]>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ChatHistory {
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Chat,
    Settings,
    AboutHelp,
}

#[derive(Debug, Clone, Copy)]
enum RequestPurpose {
    Chat,
    Test,
}

struct WorkerResult {
    purpose: RequestPurpose,
    result: Result<AiAnswer, String>,
}

struct ErrorExplainerApp {
    page: Page,
    input: String,
    messages: Vec<ChatMessage>,
    settings: ProviderSettings,
    api_key: String,
    status: String,
    last_usage: String,
    pending: bool,
    is_hidden: bool,
    pin_top: bool,
    command_receiver: mpsc::Receiver<UiCommand>,
    worker_sender: mpsc::Sender<WorkerResult>,
    worker_receiver: mpsc::Receiver<WorkerResult>,
    _hotkey_manager: Option<GlobalHotKeyManager>,
    hotkey: Option<HotKey>,
    saved_window: WindowConfig,
    last_window_save: Instant,
    #[cfg(target_os = "windows")]
    tray: Option<windows_tray::WindowsTray>,
}

impl ErrorExplainerApp {
    fn new(
        creation_context: &eframe::CreationContext<'_>,
        command_receiver: mpsc::Receiver<UiCommand>,
        start_hidden: bool,
        startup_command: Option<UiCommand>,
    ) -> Self {
        install_fonts(&creation_context.egui_ctx);
        apply_theme(&creation_context.egui_ctx);
        let history: ChatHistory = storage::load_json("history.json");
        let settings = storage::load_json("settings.json");
        let (worker_sender, worker_receiver) = mpsc::channel();
        let (hotkey_manager, hotkey) = register_hotkey();

        #[cfg(target_os = "windows")]
        let tray = windows_tray::WindowsTray::new().ok();
        #[cfg(target_os = "windows")]
        if let Some(tray) = &tray {
            tray.set_context(&creation_context.egui_ctx);
        }

        let mut app = Self {
            page: Page::Chat,
            input: String::new(),
            messages: history.messages,
            settings,
            api_key: String::new(),
            status: format!("Ready · {HOTKEY_LABEL}"),
            last_usage: String::new(),
            pending: false,
            is_hidden: start_hidden,
            pin_top: false,
            command_receiver,
            worker_sender,
            worker_receiver,
            _hotkey_manager: hotkey_manager,
            hotkey,
            saved_window: storage::load_json("window.json"),
            last_window_save: Instant::now(),
            #[cfg(target_os = "windows")]
            tray,
        };
        if let Some(command) = startup_command {
            app.handle_command(command, &creation_context.egui_ctx);
        }
        #[cfg(target_os = "windows")]
        if app.tray.is_none() && start_hidden {
            app.status = "Windows tray initialization failed; window kept visible.".to_owned();
            app.show(&creation_context.egui_ctx);
        }
        app
    }

    fn handle_command(&mut self, command: UiCommand, context: &egui::Context) {
        match command {
            UiCommand::Open => {
                self.page = Page::Chat;
                self.show(context);
            }
            UiCommand::Settings => {
                self.page = Page::Settings;
                self.show(context);
            }
            UiCommand::Help => {
                self.page = Page::AboutHelp;
                self.show(context);
            }
            UiCommand::Exit => context.send_viewport_cmd(egui::ViewportCommand::Close),
            UiCommand::Ping => {}
        }
    }

    fn show(&mut self, context: &egui::Context) {
        self.is_hidden = false;
        context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        context.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        context.send_viewport_cmd(egui::ViewportCommand::Focus);
        context.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if self.pin_top {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        }));
    }

    fn hide(&mut self, context: &egui::Context) {
        self.is_hidden = true;
        context.send_viewport_cmd(egui::ViewportCommand::Visible(false));
    }

    fn poll_events(&mut self, context: &egui::Context) {
        while let Ok(command) = self.command_receiver.try_recv() {
            self.handle_command(command, context);
        }
        #[cfg(target_os = "windows")]
        if let Some(tray) = &self.tray {
            if let Some(command) = tray.poll() {
                self.handle_command(command, context);
            }
        }
        let hotkey_receiver = GlobalHotKeyEvent::receiver();
        while let Ok(event) = hotkey_receiver.try_recv() {
            if self.hotkey.is_some_and(|hotkey| event.id == hotkey.id()) {
                if self.is_hidden {
                    self.page = Page::Chat;
                    self.show(context);
                    if self.input.is_empty() {
                        self.paste_clipboard();
                    }
                } else {
                    self.hide(context);
                }
            }
        }
        while let Ok(worker) = self.worker_receiver.try_recv() {
            self.pending = false;
            match worker.result {
                Ok(answer) => {
                    self.last_usage = format_usage(&answer, &self.settings);
                    match worker.purpose {
                        RequestPurpose::Chat => {
                            self.messages.push(ChatMessage {
                                role: "assistant".to_owned(),
                                content: answer.text,
                            });
                            self.trim_saved_history();
                            self.save_history();
                            self.status = "Analysis complete".to_owned();
                        }
                        RequestPurpose::Test => {
                            self.status = format!("Connection OK: {}", first_line(&answer.text));
                        }
                    }
                }
                Err(error) => self.status = error,
            }
        }
    }

    fn start_request(
        &mut self,
        messages: Vec<ChatMessage>,
        purpose: RequestPurpose,
        context: &egui::Context,
    ) {
        if self.pending {
            return;
        }
        self.pending = true;
        self.status = "Contacting provider…".to_owned();
        let request = AiRequest {
            settings: self.settings.clone(),
            api_key: self.api_key.clone(),
            messages,
        };
        let sender = self.worker_sender.clone();
        let context = context.clone();
        std::thread::spawn(move || {
            let result = ai::ask(request);
            let _ = sender.send(WorkerResult { purpose, result });
            context.request_repaint();
        });
    }

    fn submit(&mut self, context: &egui::Context) {
        let question = self.input.trim().to_owned();
        if question.is_empty() || self.pending {
            return;
        }
        let limited = keep_tail(&question, self.settings.max_input_chars);
        self.messages.push(ChatMessage {
            role: "user".to_owned(),
            content: limited,
        });
        self.save_history();
        self.input.clear();
        self.start_request(self.messages.clone(), RequestPurpose::Chat, context);
    }

    fn test_provider(&mut self, context: &egui::Context) {
        self.start_request(
            vec![ChatMessage {
                role: "user".to_owned(),
                content: "Reply with exactly: connection ok".to_owned(),
            }],
            RequestPurpose::Test,
            context,
        );
    }

    fn paste_clipboard(&mut self) {
        match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.get_text()) {
            Ok(text) => {
                self.input = keep_tail(&text, self.settings.max_input_chars);
                self.status = "Clipboard pasted locally".to_owned();
            }
            Err(error) => self.status = format!("Clipboard unavailable: {error}"),
        }
    }

    fn save_history(&mut self) {
        if let Err(error) = storage::save_json(
            "history.json",
            &ChatHistory {
                messages: self.messages.clone(),
            },
        ) {
            self.status = format!("History save failed: {error}");
        }
    }

    fn trim_saved_history(&mut self) {
        if self.messages.len() > 100 {
            let remove = self.messages.len() - 100;
            self.messages.drain(0..remove);
        }
    }

    fn clear_history(&mut self) {
        self.messages.clear();
        self.save_history();
        self.status = "Local history cleared".to_owned();
    }

    fn save_settings(&mut self) {
        match storage::save_json("settings.json", &self.settings) {
            Ok(()) => {
                self.status = "Settings saved. API key remains only in process memory.".to_owned()
            }
            Err(error) => self.status = format!("Settings save failed: {error}"),
        }
    }

    fn persist_window(&mut self, context: &egui::Context) {
        if self.last_window_save.elapsed() < Duration::from_millis(700) {
            return;
        }
        self.last_window_save = Instant::now();
        let current = context.input(|input| {
            let rect = input.viewport().outer_rect?;
            Some(WindowConfig {
                position: Some([rect.min.x, rect.min.y]),
                size: Some([rect.width(), rect.height()]),
            })
        });
        if let Some(current) = current {
            if current != self.saved_window {
                let _ = storage::save_json("window.json", &current);
                self.saved_window = current;
            }
        }
    }

    fn top_bar(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::top("top_bar")
            .exact_height(42.0)
            .show(context, |ui| {
                let drag = ui.allocate_response(
                    Vec2::new(ui.available_width(), 38.0),
                    egui::Sense::click_and_drag(),
                );
                if drag.drag_started() {
                    context.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                ui.allocate_ui_at_rect(drag.rect, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new("ERROR EXPLAINER")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(241, 245, 249)),
                        );
                        ui.label(
                            RichText::new("EE")
                                .strong()
                                .color(Color32::from_rgb(50, 213, 131)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("×").on_hover_text("Hide to tray").clicked() {
                                self.hide(context);
                            }
                            if ui
                                .selectable_label(self.page == Page::AboutHelp, "HELP")
                                .clicked()
                            {
                                self.page = Page::AboutHelp;
                            }
                            if ui
                                .selectable_label(self.page == Page::Settings, "SETTINGS")
                                .clicked()
                            {
                                self.page = Page::Settings;
                            }
                            if ui
                                .selectable_label(self.page == Page::Chat, "CHAT")
                                .clicked()
                            {
                                self.page = Page::Chat;
                            }
                            if ui
                                .selectable_label(self.pin_top, "PIN")
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
                });
            });
    }

    fn chat_ui(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::bottom("chat_input")
            .resizable(false)
            .show(context, |ui| {
                ui.add_space(8.0);
                let input_tokens = ai::estimate_tokens(&self.input);
                let max_cost = ai::estimate_max_cost(&self.settings, self.input.chars().count());
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} · ≈{} input tokens · output ≤{}",
                            self.settings.provider.label(),
                            input_tokens,
                            self.settings.max_output_tokens
                        ))
                        .small()
                        .color(Color32::from_rgb(148, 163, 184)),
                    );
                    if let Some(cost) = max_cost {
                        ui.label(
                            RichText::new(format!("estimated max ${cost:.4}"))
                                .small()
                                .color(Color32::from_rgb(50, 213, 131)),
                        );
                    }
                });
                let editor = ui.add_sized(
                    [ui.available_width(), 96.0],
                    TextEdit::multiline(&mut self.input)
                        .hint_text("Paste logs, a stack trace, or ask a debugging question…"),
                );
                let submit_shortcut = editor.has_focus()
                    && ui.input(|input| input.key_pressed(Key::Enter) && input.modifiers.ctrl);
                ui.horizontal(|ui| {
                    if ui.button("Paste clipboard").clicked() {
                        self.paste_clipboard();
                    }
                    if ui.button("Clear input").clicked() {
                        self.input.clear();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let send = ui
                            .add_enabled(
                                !self.pending && !self.input.trim().is_empty(),
                                egui::Button::new(if self.pending {
                                    "Analyzing…"
                                } else {
                                    "Explain · Ctrl+Enter"
                                })
                                .fill(Color32::from_rgb(37, 99, 235)),
                            )
                            .clicked();
                        if send || submit_shortcut {
                            self.submit(context);
                        }
                    });
                });
                ui.add_space(6.0);
            });

        egui::CentralPanel::default().show(context, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&self.status)
                        .small()
                        .color(Color32::from_rgb(148, 163, 184)),
                );
                if self.pending {
                    ui.spinner();
                }
                if !self.last_usage.is_empty() {
                    ui.separator();
                    ui.label(RichText::new(&self.last_usage).small());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Clear history").clicked() {
                        self.clear_history();
                    }
                });
            });
            ui.separator();
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if self.messages.is_empty() {
                        empty_state(ui);
                    }
                    for message in &self.messages {
                        message_card(ui, message);
                    }
                });
        });
    }

    fn settings_ui(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
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
                }

                ui.label("Model");
                ui.add_sized(
                    [ui.available_width(), 30.0],
                    TextEdit::singleline(&mut self.settings.model),
                );
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

                ui.add_space(16.0);
                ui.heading("Limits");
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

    fn about_help_ui(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Error Explainer");
                ui.label("Local AI-assisted incident triage for logs, stack traces, and debugging questions.");
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

impl eframe::App for ErrorExplainerApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_events(context);
        self.persist_window(context);
        self.top_bar(context);
        match self.page {
            Page::Chat => self.chat_ui(context),
            Page::Settings => self.settings_ui(context),
            Page::AboutHelp => self.about_help_ui(context),
        }
        context.request_repaint_after(Duration::from_millis(100));
    }
}

fn register_hotkey() -> (Option<GlobalHotKeyManager>, Option<HotKey>) {
    let Ok(manager) = GlobalHotKeyManager::new() else {
        return (None, None);
    };
    let hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT),
        Code::KeyC,
    );
    if manager.register(hotkey).is_err() {
        return (Some(manager), None);
    }
    (Some(manager), Some(hotkey))
}

fn install_fonts(context: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto".to_owned());
    context.set_fonts(fonts);
}

fn apply_theme(context: &egui::Context) {
    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = Vec2::new(9.0, 8.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(Color32::from_rgb(226, 232, 240));
    style.visuals.window_fill = Color32::from_rgb(8, 15, 30);
    style.visuals.panel_fill = Color32::from_rgb(8, 15, 30);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(17, 28, 50);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(30, 41, 65);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(37, 99, 235);
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, FontId::proportional(22.0));
    context.set_style(style);
}

fn empty_state(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(64.0);
        ui.label(
            RichText::new("Paste an error. Get a testable explanation.")
                .size(22.0)
                .strong(),
        );
        ui.add_space(8.0);
        ui.label("Logs and stack traces stay local until you press Explain.");
        ui.label("Start safely with the offline Demo provider.");
    });
}

fn message_card(ui: &mut egui::Ui, message: &ChatMessage) {
    let user = message.role == "user";
    let accent = if user {
        Color32::from_rgb(248, 113, 113)
    } else {
        Color32::from_rgb(50, 213, 131)
    };
    egui::Frame::none()
        .fill(Color32::from_rgb(15, 23, 42))
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(30, 41, 59)))
        .rounding(egui::Rounding::same(8.0))
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label(
                RichText::new(if user { "YOU" } else { "EE" })
                    .strong()
                    .small()
                    .color(accent),
            );
            ui.label(&message.content);
        });
    ui.add_space(8.0);
}

fn keep_tail(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    const PREFIX: &str = "[document head truncated]\n";
    let tail_count = max_chars.saturating_sub(PREFIX.chars().count());
    let mut chars: Vec<char> = text.chars().rev().take(tail_count).collect();
    chars.reverse();
    format!("{PREFIX}{}", chars.into_iter().collect::<String>())
}

fn format_usage(answer: &AiAnswer, settings: &ProviderSettings) -> String {
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
    format!("{input} in / {output} out{cost}")
}

fn first_line(text: &str) -> String {
    text.lines()
        .next()
        .unwrap_or("ok")
        .chars()
        .take(120)
        .collect()
}

fn load_app_icon() -> std::sync::Arc<egui::IconData> {
    let image = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
        .expect("embedded app icon should decode")
        .resize_exact(256, 256, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    std::sync::Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width: 256,
        height: 256,
    })
}
