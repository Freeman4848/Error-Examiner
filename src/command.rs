use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

const COMMAND_ADDRESS: &str = "127.0.0.1:47661";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiCommand {
    Open,
    Settings,
    Help,
    Exit,
    Ping,
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

pub fn start_server() -> Result<mpsc::Receiver<UiCommand>, String> {
    let listener = TcpListener::bind(COMMAND_ADDRESS).map_err(|error| error.to_string())?;
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else {
                continue;
            };
            let mut command = String::new();
            if stream.read_to_string(&mut command).is_ok() {
                if let Some(command) = UiCommand::parse(&command) {
                    let _ = sender.send(command);
                }
            }
        }
    });
    Ok(receiver)
}

pub fn send(command: UiCommand) -> Result<(), String> {
    let mut stream = TcpStream::connect(COMMAND_ADDRESS).map_err(|error| error.to_string())?;
    stream
        .write_all(command.wire_name().as_bytes())
        .map_err(|error| error.to_string())
}
