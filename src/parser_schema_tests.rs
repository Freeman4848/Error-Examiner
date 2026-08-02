use crate::parser_schema;
use std::path::Path;

#[test]
fn parses_schema_v3_reference_fixtures() {
    let rust = parser_schema::parse(include_str!("../fixtures/broken-rust/broken-rust.log"));
    assert!(rust.supported);
    assert_eq!(rust.format_ids, ["cargo-rustc-error"]);
    assert!(rust.events.len() >= 4);

    let gcp = parser_schema::parse(include_str!("../fixtures/cloud/gcp-mixed.json"));
    assert!(gcp.supported);
    assert_eq!(gcp.format_ids.len(), 4);
    assert_eq!(gcp.events.len(), 4);
}

#[test]
fn fixture_pack_has_an_executable_adapter() {
    for root in ["fixtures/external/loghub", "fixtures/generated/runtime"] {
        visit_logs(Path::new(root), &mut |path| {
            let text = std::fs::read_to_string(path).expect("fixture must be readable");
            let parsed = parser_schema::parse(&text);
            assert!(parsed.supported, "missing adapter: {}", path.display());
            assert!(!parsed.events.is_empty(), "empty parse: {}", path.display());
            assert_eq!(
                parsed.format_ids.len(),
                1,
                "ambiguous adapter: {}",
                path.display()
            );
        });
    }
}

#[test]
fn unknown_text_still_falls_back_to_raw() {
    let parsed = parser_schema::parse("unclassified application message");
    assert!(!parsed.supported);
    assert!(parsed.events.is_empty());
}

#[test]
fn parses_downloaded_browser_stacks() {
    let fixtures = [
        ("chrome-v8.log", "browser-chromium-v8-stack"),
        ("opera-blink.log", "browser-chromium-v8-stack"),
        ("firefox-spidermonkey.log", "browser-firefox-stack"),
        ("safari-webkit.log", "browser-safari-webkit-stack"),
    ];
    for (name, expected) in fixtures {
        let path = Path::new("fixtures/external/browser-stacktracejs").join(name);
        let text = std::fs::read_to_string(path).unwrap();
        let parsed = parser_schema::parse(&text);
        assert!(parsed.supported, "missing browser adapter: {name}");
        assert_eq!(parsed.format_ids, [expected]);
    }
}

#[test]
fn parses_known_gap_corpus() {
    let text = include_str!("../fixtures/gaps/error-corpus.json");
    let cases: serde_json::Value = serde_json::from_str(text).unwrap();
    let cases = cases.as_array().unwrap();
    assert!(cases.len() >= 42);
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let expected = case["format"].as_str().unwrap();
        let input = case["input"].as_str().unwrap();
        let parsed = parser_schema::parse(input);
        assert!(parsed.supported, "missing gap adapter: {name}");
        assert_eq!(parsed.format_ids, [expected], "wrong adapter: {name}");
    }
}

#[test]
fn classifies_vpc_reject_and_skipdata_as_important() {
    let text = include_str!("../fixtures/generated/runtime/aws-vpc-flow-default.log");
    let parsed = parser_schema::parse(text);
    assert!(parsed.supported);
    assert_eq!(parsed.format_ids, ["aws-vpc-flow-default"]);
    assert_eq!(parsed.events.len(), 3);
    assert_eq!(
        parsed
            .events
            .iter()
            .filter(|event| event.severity != parser_schema::Severity::Info)
            .count(),
        2
    );
}

fn visit_logs(path: &Path, callback: &mut impl FnMut(&Path)) {
    for entry in std::fs::read_dir(path).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must be readable").path();
        if path.is_dir() {
            visit_logs(&path, callback);
        } else if path.extension().is_some_and(|extension| extension == "log") {
            callback(&path);
        }
    }
}
