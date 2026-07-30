use eframe::egui;
use std::sync::mpsc;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ConnectionExt, GrabMode, ModMask},
    rust_connection::RustConnection,
};

const PHYSICAL_C: u8 = 54;
const SHORTCUT_KEYS: [u8; 5] = [38, 54, 53, 55, 52]; // A, C, X, V, Z

pub(crate) struct Shortcuts {
    connection: Option<RustConnection>,
    down: [bool; 5],
}

impl Shortcuts {
    pub(crate) fn new() -> Self {
        Self {
            connection: x11rb::connect(None).ok().map(|(connection, _)| connection),
            down: [false; 5],
        }
    }

    pub(crate) fn apply(&mut self, context: &egui::Context) {
        if !context.input(|input| input.viewport().focused.unwrap_or(false)) {
            return;
        }
        let Some(connection) = &self.connection else {
            return;
        };
        let Some(keys) = connection
            .query_keymap()
            .ok()
            .and_then(|cookie| cookie.reply().ok())
            .map(|reply| reply.keys)
        else {
            return;
        };
        let modifiers = context.input(|input| input.modifiers);
        for (index, keycode) in SHORTCUT_KEYS.into_iter().enumerate() {
            let pressed = keys[usize::from(keycode / 8)] & (1 << (keycode % 8)) != 0;
            let trigger = modifiers.ctrl && pressed && !self.down[index];
            self.down[index] = pressed;
            if trigger {
                inject_shortcut(context, index, modifiers);
            }
        }
    }
}

fn inject_shortcut(context: &egui::Context, index: usize, modifiers: egui::Modifiers) {
    context.input_mut(|input| match index {
        0 => input.events.push(egui::Event::Key {
            key: egui::Key::A,
            physical_key: Some(egui::Key::A),
            pressed: true,
            repeat: false,
            modifiers,
        }),
        1 if !input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Copy)) =>
        {
            input.events.push(egui::Event::Copy)
        }
        2 if !input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Cut)) =>
        {
            input.events.push(egui::Event::Cut)
        }
        3 if !input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Paste(_))) =>
        {
            if let Ok(text) =
                arboard::Clipboard::new().and_then(|mut clipboard| clipboard.get_text())
            {
                input.events.push(egui::Event::Paste(text));
            }
        }
        4 => input.events.push(egui::Event::Key {
            key: egui::Key::Z,
            physical_key: Some(egui::Key::Z),
            pressed: true,
            repeat: false,
            modifiers,
        }),
        _ => {}
    });
}

pub(crate) fn register(context: &egui::Context) -> mpsc::Receiver<()> {
    let (sender, receiver) = mpsc::channel();
    let context = context.clone();
    std::thread::spawn(move || {
        let Ok((connection, screen)) = x11rb::connect(None) else {
            return;
        };
        let root = connection.setup().roots[screen].root;
        let modifiers = ModMask::CONTROL | ModMask::SHIFT | ModMask::M1;
        for ignored in [
            ModMask::default(),
            ModMask::LOCK,
            ModMask::M2,
            ModMask::LOCK | ModMask::M2,
        ] {
            let _ = connection.grab_key(
                false,
                root,
                modifiers | ignored,
                PHYSICAL_C,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
            );
        }
        let _ = connection.flush();
        while let Ok(event) = connection.wait_for_event() {
            if matches!(
                event,
                x11rb::protocol::Event::KeyPress(key) if key.detail == PHYSICAL_C
            ) {
                if sender.send(()).is_err() {
                    break;
                }
                context.request_repaint();
            }
        }
    });
    receiver
}
