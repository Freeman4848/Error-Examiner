#![allow(dead_code)]

#[path = "../src/parser_registry.rs"]
mod parser_registry;
#[path = "../src/parser_schema.rs"]
mod parser_schema;
#[path = "../src/parser_text.rs"]
mod parser_text;
#[path = "../src/preprocess.rs"]
mod preprocess;
#[path = "../src/preprocess_raw.rs"]
mod preprocess_raw;
#[path = "../src/storage.rs"]
mod storage;

use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut raw = String::new();
    io::stdin().read_to_string(&mut raw)?;
    let parsed = preprocess::prepare("stdin.log".into(), &raw, 12_000);
    println!(
        "format={} events={} important={} selected={} chars={}→{}",
        if parsed.detected_format.is_empty() {
            "Raw"
        } else {
            &parsed.detected_format
        },
        parsed.event_count,
        parsed.important_events,
        parsed.selected_variant,
        parsed.original_chars,
        parsed.normalized_chars
    );
    Ok(())
}
