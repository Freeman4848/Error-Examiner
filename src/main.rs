#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod ai;
mod attachment;
mod chats;
mod command;
mod hotkey;
#[cfg(target_os = "linux")]
mod linux_hotkey;
#[cfg(target_os = "linux")]
mod linux_tray;
mod parser_registry;
mod parser_schema;
#[cfg(test)]
mod parser_schema_tests;
mod parser_text;
mod preprocess;
mod preprocess_raw;
mod schema_lab;
mod schema_management;
mod storage;
mod theme;
mod ui;
mod ui_chrome;
mod ui_preview;
mod ui_schema_lab;
mod ui_settings_helpers;
mod ui_tabs;
#[cfg(target_os = "windows")]
mod windows_tray;
mod workflow;

use ai::{AiAnswer, AiRequest, ApiProtocol, ChatMessage, ProviderKind, ProviderSettings};
use command::UiCommand;
use eframe::egui::{self, Color32, Key, RichText, ScrollArea, TextEdit};
use global_hotkey::GlobalHotKeyManager;
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::mpsc,
    time::{Duration, Instant},
};
use theme::{
    apply_theme, install_fonts, load_app_icon, load_app_texture, palette, Appearance, ThemeKind,
};
use ui::{first_line, format_usage, keep_tail};

const APP_NAME: &str = "Error Explainer";
const HOTKEY_LABEL: &str = "Ctrl+Shift+Alt+C";

fn main() -> eframe::Result<()> {
    let startup_command = command_from_args();
    let command_server = match command::start_server() {
        Ok(server) => server,
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
        .with_transparent(true)
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
                command_server,
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
    SchemaLab,
    Settings,
    AboutHelp,
}

#[derive(Debug, Clone, Copy)]
enum RequestPurpose {
    Chat,
    Test,
    Batch { index: usize, total: usize },
    Synthesis,
}

struct WorkerResult {
    purpose: RequestPurpose,
    result: Result<AiAnswer, String>,
}

struct ErrorExplainerApp {
    page: Page,
    input: String,
    attachment: Option<ai::ImageAttachment>,
    prepared_log: Option<preprocess::PreparedLog>,
    preview_open: bool,
    preview_created: bool,
    preview_mode: ui_preview::PreviewMode,
    preview_pane: ui_preview::PreviewPane,
    app_icon_texture: egui::TextureHandle,
    batch_queue: std::collections::VecDeque<String>,
    batch_findings: Vec<String>,
    batch_total: usize,
    messages: Vec<ChatMessage>,
    chat_tabs: Vec<chats::ChatTab>,
    active_chat: usize,
    schema_registry: parser_registry::RegistryStatus,
    schema_sender: mpsc::Sender<Result<schema_lab::SchemaDraft, String>>,
    schema_receiver: mpsc::Receiver<Result<schema_lab::SchemaDraft, String>>,
    schema_draft: Option<schema_lab::SchemaDraft>,
    schema_pending: bool,
    schema_status: String,
    schema_ui: ui_schema_lab::SchemaUiState,
    settings: ProviderSettings,
    api_key: String,
    status: String,
    last_usage: String,
    pending: bool,
    is_hidden: bool,
    pin_top: bool,
    window_locked: bool,
    command_server: command::CommandServer,
    worker_sender: mpsc::Sender<WorkerResult>,
    worker_receiver: mpsc::Receiver<WorkerResult>,
    model_sender: mpsc::Sender<Result<Vec<String>, String>>,
    model_receiver: mpsc::Receiver<Result<Vec<String>, String>>,
    available_models: Vec<String>,
    loading_models: bool,
    _hotkey_manager: Option<GlobalHotKeyManager>,
    hotkey_receiver: mpsc::Receiver<()>,
    theme: ThemeKind,
    opacity: f32,
    applied_theme: Option<ThemeKind>,
    applied_opacity: f32,
    active_fps: f32,
    saved_window: WindowConfig,
    last_window_save: Instant,
    #[cfg(target_os = "windows")]
    tray: Option<windows_tray::WindowsTray>,
    #[cfg(target_os = "linux")]
    _tray: Option<ksni::blocking::Handle<linux_tray::LinuxTray>>,
    #[cfg(target_os = "linux")]
    shortcuts: linux_hotkey::Shortcuts,
}

impl ErrorExplainerApp {
    fn new(
        creation_context: &eframe::CreationContext<'_>,
        command_server: command::CommandServer,
        start_hidden: bool,
        startup_command: Option<UiCommand>,
    ) -> Self {
        install_fonts(&creation_context.egui_ctx);
        let history: ChatHistory = storage::load_json("history.json");
        let mut workspace: chats::Workspace = storage::load_json("chats.json");
        if workspace.tabs.is_empty() {
            workspace.tabs.push(chats::ChatTab {
                title: "Chat 1".to_owned(),
                messages: history.messages,
                ..Default::default()
            });
        }
        workspace.active = workspace.active.min(workspace.tabs.len() - 1);
        let active_tab = workspace.tabs[workspace.active].clone();
        let settings = storage::load_json("settings.json");
        let appearance: Appearance = storage::load_json("appearance.json");
        let (worker_sender, worker_receiver) = mpsc::channel();
        let (model_sender, model_receiver) = mpsc::channel();
        let (schema_sender, schema_receiver) = mpsc::channel();
        let (hotkey_manager, hotkey_receiver) = hotkey::register(&creation_context.egui_ctx);
        let app_icon_texture = load_app_texture(&creation_context.egui_ctx);
        let backup_status = schema_management::ensure_backup();
        let schema_registry = parser_registry::reload_user_schemas();
        let schema_coverage = parser_registry::coverage();
        debug_assert_eq!(
            schema_coverage.covered + schema_coverage.partial + schema_coverage.raw,
            schema_coverage.total
        );
        command_server.set_context(&creation_context.egui_ctx);

        #[cfg(target_os = "windows")]
        let tray = windows_tray::WindowsTray::new().ok();
        #[cfg(target_os = "windows")]
        if let Some(tray) = &tray {
            tray.set_context(&creation_context.egui_ctx);
        }
        #[cfg(target_os = "linux")]
        let tray = linux_tray::spawn().ok();

        let mut app = Self {
            page: Page::Chat,
            input: active_tab.input,
            attachment: None,
            prepared_log: None,
            preview_open: false,
            preview_created: false,
            preview_mode: ui_preview::PreviewMode::Compare,
            preview_pane: ui_preview::PreviewPane::Raw,
            app_icon_texture,
            batch_queue: std::collections::VecDeque::new(),
            batch_findings: Vec::new(),
            batch_total: 0,
            messages: active_tab.messages,
            chat_tabs: workspace.tabs,
            active_chat: workspace.active,
            schema_registry,
            schema_sender,
            schema_receiver,
            schema_draft: None,
            schema_pending: false,
            schema_status: backup_status
                .err()
                .unwrap_or_else(|| "Select Add and choose a log to begin.".to_owned()),
            schema_ui: ui_schema_lab::SchemaUiState::default(),
            settings,
            api_key: String::new(),
            status: format!("Ready · {HOTKEY_LABEL}"),
            last_usage: String::new(),
            pending: false,
            is_hidden: start_hidden,
            pin_top: false,
            window_locked: false,
            command_server,
            worker_sender,
            worker_receiver,
            model_sender,
            model_receiver,
            available_models: Vec::new(),
            loading_models: false,
            _hotkey_manager: hotkey_manager,
            hotkey_receiver,
            theme: appearance.theme,
            opacity: appearance.opacity.clamp(0.3, 1.0),
            applied_theme: None,
            applied_opacity: 0.0,
            active_fps: 0.0,
            saved_window: storage::load_json("window.json"),
            last_window_save: Instant::now(),
            #[cfg(target_os = "windows")]
            tray,
            #[cfg(target_os = "linux")]
            _tray: tray,
            #[cfg(target_os = "linux")]
            shortcuts: linux_hotkey::Shortcuts::new(),
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

    fn paste_clipboard(&mut self) {
        match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.get_text()) {
            Ok(text) => {
                self.input = keep_tail(&text, self.settings.max_input_chars);
                self.status = "Clipboard pasted locally".to_owned();
            }
            Err(error) => self.status = format!("Clipboard unavailable: {error}"),
        }
    }

    fn save_settings(&mut self) {
        let settings = storage::save_json("settings.json", &self.settings);
        let appearance = storage::save_json(
            "appearance.json",
            &Appearance {
                theme: self.theme,
                opacity: self.opacity,
            },
        );
        match settings.and(appearance) {
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
}

impl eframe::App for ErrorExplainerApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        palette(self.theme, self.opacity)
            .background
            .to_normalized_gamma_f32()
    }

    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "linux")]
        self.shortcuts.apply(context);
        if self.pending {
            let fps = context.input(|input| 1.0 / input.stable_dt.max(0.001));
            self.active_fps = if self.active_fps == 0.0 {
                fps
            } else {
                self.active_fps * 0.9 + fps * 0.1
            };
        }
        if self.applied_theme != Some(self.theme)
            || (self.applied_opacity - self.opacity).abs() > f32::EPSILON
        {
            apply_theme(context, self.theme, self.opacity);
            self.applied_theme = Some(self.theme);
            self.applied_opacity = self.opacity;
        }
        self.poll_events(context);
        self.persist_window(context);
        self.top_bar(context);
        match self.page {
            Page::Chat => self.chat_ui(context),
            Page::SchemaLab => self.schema_lab_ui(context),
            Page::Settings => self.settings_ui(context),
            Page::AboutHelp => self.about_help_ui(context),
        }
        self.log_preview_window(context);
        ui_chrome::window_resize(context, self.window_locked);
        if self.pending {
            context.request_repaint_after(Duration::from_millis(16));
        }
    }
}
