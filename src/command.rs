use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

const COMMAND_ADDRESS: &str = "127.0.0.1:47661";
const COMMAND_LIMIT: u64 = 32;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiCommand {
    Open,
    Settings,
    Help,
    Exit,
    Ping,
}

pub struct CommandServer {
    receiver: mpsc::Receiver<UiCommand>,
    wake_context: Arc<Mutex<Option<eframe::egui::Context>>>,
}

impl CommandServer {
    pub fn set_context(&self, context: &eframe::egui::Context) {
        if let Ok(mut wake_context) = self.wake_context.lock() {
            *wake_context = Some(context.clone());
        }
    }

    pub fn try_recv(&self) -> Result<UiCommand, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl UiCommand {
    fn wire_name(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Settings => "settings",
            Self::Help => "help",
            Self::Exit => "exit",
            Self::Ping => "ping",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "open" => Some(Self::Open),
            "settings" => Some(Self::Settings),
            "help" => Some(Self::Help),
            "exit" => Some(Self::Exit),
            "ping" => Some(Self::Ping),
            _ => None,
        }
    }
}

pub fn start_server() -> Result<CommandServer, String> {
    let listener = TcpListener::bind(COMMAND_ADDRESS).map_err(|error| error.to_string())?;
    let (sender, receiver) = mpsc::channel();
    let wake_context: Arc<Mutex<Option<eframe::egui::Context>>> = Arc::new(Mutex::new(None));
    let thread_context = Arc::clone(&wake_context);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else {
                continue;
            };
            let _ = stream.set_read_timeout(Some(COMMAND_TIMEOUT));
            let mut command = String::new();
            if stream
                .take(COMMAND_LIMIT)
                .read_to_string(&mut command)
                .is_ok()
            {
                if let Some(command) = UiCommand::parse(&command) {
                    let _ = sender.send(command);
                    if let Ok(context) = thread_context.lock() {
                        if let Some(context) = context.as_ref() {
                            context.request_repaint();
                        }
                    }
                }
            }
        }
    });
    Ok(CommandServer {
        receiver,
        wake_context,
    })
}

pub fn send(command: UiCommand) -> Result<(), String> {
    let mut stream = TcpStream::connect(COMMAND_ADDRESS).map_err(|error| error.to_string())?;
    stream
        .write_all(command.wire_name().as_bytes())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_exact_bounded_commands() {
        assert_eq!(UiCommand::parse("open\n"), Some(UiCommand::Open));
        assert_eq!(UiCommand::parse("open extra"), None);
        assert!(COMMAND_LIMIT >= UiCommand::Settings.wire_name().len() as u64);
    }
}
