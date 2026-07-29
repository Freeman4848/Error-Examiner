use std::sync::{mpsc, Arc, Mutex};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

use crate::command::UiCommand;

pub struct WindowsTray {
    _tray: TrayIcon,
    actions: mpsc::Receiver<UiCommand>,
    wake_context: Arc<Mutex<Option<eframe::egui::Context>>>,
}

impl WindowsTray {
    pub fn new() -> Result<Self, String> {
        let menu = Menu::new();
        let open = MenuItem::new("Open Error Explainer", true, None);
        let settings = MenuItem::new("Settings", true, None);
        let help = MenuItem::new("About & Help", true, None);
        let exit = MenuItem::new("Exit", true, None);
        menu.append_items(&[&open, &settings, &help, &exit])
            .map_err(|error| error.to_string())?;

        let open_id = open.id().clone();
        let settings_id = settings.id().clone();
        let help_id = help.id().clone();
        let exit_id = exit.id().clone();
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Error Explainer")
            .with_icon(load_icon()?)
            .build()
            .map_err(|error| error.to_string())?;

        let (sender, actions) = mpsc::channel();
        let wake_context: Arc<Mutex<Option<eframe::egui::Context>>> = Arc::new(Mutex::new(None));
        let thread_context = Arc::clone(&wake_context);
        std::thread::spawn(move || {
            while let Ok(event) = MenuEvent::receiver().recv() {
                let command = if event.id == open_id {
                    UiCommand::Open
                } else if event.id == settings_id {
                    UiCommand::Settings
                } else if event.id == help_id {
                    UiCommand::Help
                } else if event.id == exit_id {
                    UiCommand::Exit
                } else {
                    continue;
                };
                if sender.send(command).is_err() {
                    break;
                }
                if let Ok(context) = thread_context.lock() {
                    if let Some(context) = context.as_ref() {
                        context.request_repaint();
                    }
                }
            }
        });

        Ok(Self {
            _tray: tray,
            actions,
            wake_context,
        })
    }

    pub fn set_context(&self, context: &eframe::egui::Context) {
        if let Ok(mut wake_context) = self.wake_context.lock() {
            *wake_context = Some(context.clone());
        }
    }

    pub fn poll(&self) -> Option<UiCommand> {
        self.actions.try_recv().ok()
    }
}

fn load_icon() -> Result<Icon, String> {
    let image = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
        .map_err(|error| error.to_string())?
        .resize_exact(32, 32, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    Icon::from_rgba(image.into_raw(), 32, 32).map_err(|error| error.to_string())
}
