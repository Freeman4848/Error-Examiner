#![allow(dead_code)]

#[path = "../src/parser_schema.rs"]
mod parser_schema;
#[path = "../src/parser_text.rs"]
mod parser_text;
#[path = "../src/preprocess.rs"]
mod preprocess;
#[path = "../src/preprocess_raw.rs"]
mod preprocess_raw;

use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input = root.join("fixtures/external/loghub");
    let output = root.join("review/parser-loghub-16");
    fs::create_dir_all(&output)?;
    let mut paths = fs::read_dir(input)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "log"))
        .collect::<Vec<_>>();
    paths.sort();
    let mut index = String::from("# LogHub parser review\n\n| File | Format | Chars | Events | Important | Duplicates | Batches |\n|---|---|---:|---:|---:|---:|---:|\n");
    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let raw = fs::read_to_string(&path)?;
        let parsed = preprocess::prepare(name.clone(), &raw, 12_000);
        index.push_str(&format!(
            "| {name} | {} | {}→{} | {} | {} | {} | {} |\n",
            parsed.detected_format,
            parsed.original_chars,
            parsed.normalized_chars,
            parsed.event_count,
            parsed.important_events,
            parsed.duplicate_count,
            parsed.batches.len()
        ));
        let batches = parsed
            .batches
            .iter()
            .enumerate()
            .map(|(index, batch)| format!("### Batch {}\n\n```text\n{}\n```", index + 1, batch))
            .collect::<Vec<_>>()
            .join("\n\n");
        let report = format!(
            "# {name}\n\nFormat: `{}` · chars {}→{} · events {} · important {} · duplicates {} · batches {}\n\n## Raw\n\n```text\n{}\n```\n\n## Parsed\n\n```text\n{}\n```\n\n## Sent batches\n\n{}\n",
            parsed.detected_format, parsed.original_chars, parsed.normalized_chars,
            parsed.event_count, parsed.important_events, parsed.duplicate_count,
            parsed.batches.len(), parsed.raw_preview, parsed.parsed_preview, batches
        );
        fs::write(output.join(format!("{name}.md")), report)?;
    }
    fs::write(output.join("README.md"), index)?;
    Ok(())
}
