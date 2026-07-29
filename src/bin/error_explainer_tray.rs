#[path = "../command.rs"]
#[allow(dead_code)]
mod command;

use command::UiCommand;
use ksni::{blocking::TrayMethods, Icon};
use std::{
    env,
    path::PathBuf,
    process::{Child, Command},
    time::Duration,
};

#[derive(Default)]
struct ErrorExplainerTray {
    child: Option<Child>,
}

impl ksni::Tray for ErrorExplainerTray {
    fn id(&self) -> String {
        "error_explainer".into()
    }

    fn title(&self) -> String {
        "Error Explainer".into()
    }

    fn icon_name(&self) -> String {
        String::new()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        load_icon().into_iter().collect()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.open(UiCommand::Open);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Open Error Explainer".into(),
                icon_name: "system-run".into(),
                activate: Box::new(|tray: &mut ErrorExplainerTray| {
                    let _ = tray.open(UiCommand::Open);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Settings".into(),
                icon_name: "preferences-system".into(),
                activate: Box::new(|tray: &mut ErrorExplainerTray| {
                    let _ = tray.open(UiCommand::Settings);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "About & Help".into(),
                icon_name: "help-about".into(),
                activate: Box::new(|tray: &mut ErrorExplainerTray| {
                    let _ = tray.open(UiCommand::Help);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|tray: &mut ErrorExplainerTray| {
                    let _ = command::send(UiCommand::Exit);
                    if let Some(mut child) = tray.child.take() {
                        let _ = child.wait();
                    }
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

impl ErrorExplainerTray {
    fn ensure_running(&mut self) -> Result<(), String> {
        if command::send(UiCommand::Ping).is_ok() {
            return Ok(());
        }
        if let Some(child) = &mut self.child {
            if child
                .try_wait()
                .map_err(|error| error.to_string())?
                .is_none()
            {
                return Err("Error Explainer is starting.".to_owned());
            }
            self.child = None;
        }
        let binary = app_binary()?;
        self.child = Some(
            Command::new(binary)
                .env("EE_START_HIDDEN", "1")
                .spawn()
                .map_err(|error| format!("failed to start Error Explainer: {error}"))?,
        );
        for _ in 0..20 {
            if command::send(UiCommand::Ping).is_ok() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        Err("Error Explainer did not start in time.".to_owned())
    }

    fn open(&mut self, command: UiCommand) -> Result<(), String> {
        self.ensure_running()?;
        command::send(command)
    }
}

fn app_binary() -> Result<PathBuf, String> {
    let executable = env::current_exe().map_err(|error| error.to_string())?;
    Ok(executable
        .parent()
        .ok_or_else(|| "tray executable has no parent directory".to_owned())?
        .join("error-explainer"))
}

fn load_icon() -> Result<Icon, String> {
    let rgba = image::load_from_memory(include_bytes!("../../assets/app-icon.png"))
        .map_err(|error| error.to_string())?
        .resize_exact(32, 32, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    let mut argb = Vec::with_capacity(32 * 32 * 4);
    for pixel in rgba.pixels() {
        argb.extend_from_slice(&[pixel[3], pixel[0], pixel[1], pixel[2]]);
    }
    Ok(Icon {
        width: 32,
        height: 32,
        data: argb,
    })
}

fn main() {
    let mut tray = ErrorExplainerTray::default();
    let _ = tray.ensure_running();
    let _handle = tray.spawn().expect("tray should start");
    loop {
        std::thread::park();
    }
}
