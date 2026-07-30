use eframe::egui;
use global_hotkey::GlobalHotKeyManager;
use std::sync::mpsc;

pub(crate) fn register(
    context: &egui::Context,
) -> (Option<GlobalHotKeyManager>, mpsc::Receiver<()>) {
    #[cfg(target_os = "linux")]
    return (None, crate::linux_hotkey::register(context));

    #[cfg(not(target_os = "linux"))]
    {
        use global_hotkey::{
            hotkey::{Code, HotKey, Modifiers},
            GlobalHotKeyEvent,
        };
        let (sender, receiver) = mpsc::channel();
        let Ok(manager) = GlobalHotKeyManager::new() else {
            return (None, receiver);
        };
        let hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT),
            Code::KeyC,
        );
        if manager.register(hotkey).is_err() {
            return (Some(manager), receiver);
        }
        let events = GlobalHotKeyEvent::receiver().clone();
        let context = context.clone();
        std::thread::spawn(move || {
            while let Ok(event) = events.recv() {
                if event.id == hotkey.id() {
                    if sender.send(()).is_err() {
                        break;
                    }
                    context.request_repaint();
                }
            }
        });
        (Some(manager), receiver)
    }
}
