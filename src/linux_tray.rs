use crate::{command, command::UiCommand};
use ksni::{blocking::TrayMethods, Icon};

pub struct LinuxTray;

impl ksni::Tray for LinuxTray {
    fn id(&self) -> String {
        "error_examiner".into()
    }

    fn title(&self) -> String {
        "Error Examiner".into()
    }

    fn icon_name(&self) -> String {
        String::new()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        load_icon().into_iter().collect()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = command::send(UiCommand::Open);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Open Error Examiner".into(),
                icon_name: "system-run".into(),
                activate: Box::new(|_| {
                    let _ = command::send(UiCommand::Open);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Settings".into(),
                icon_name: "preferences-system".into(),
                activate: Box::new(|_| {
                    let _ = command::send(UiCommand::Settings);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "About & Help".into(),
                icon_name: "help-about".into(),
                activate: Box::new(|_| {
                    let _ = command::send(UiCommand::Help);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| {
                    let _ = command::send(UiCommand::Exit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn spawn() -> Result<ksni::blocking::Handle<LinuxTray>, String> {
    LinuxTray.spawn().map_err(|error| error.to_string())
}

fn load_icon() -> Result<Icon, String> {
    let rgba = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
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
