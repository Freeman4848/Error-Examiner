#[derive(Debug)]
struct LogEvent {
    level: u8,
    message: String,
}

fn normalize(line: &str) -> LogEvent {
    let level: u8 = "ERROR";
    LogEvent {
        level,
        message: line,
    }
}

fn main() {
    let event = normalize();
    println!("{}: {}", event.severity, event.message);
}
