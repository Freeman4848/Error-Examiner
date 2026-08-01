#!/usr/bin/env python3
import collections
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RAW_DIR = ROOT / "fixtures/external/loghub"
REPORT_DIR = ROOT / "review/parser-loghub-16"
HEADER = re.compile(
    r"^\[(?:line|lines) [^]]+ · (CRITICAL|ERROR|WARNING|INFO|UNKNOWN)"
    r"(?: · repeated (\d+) times)?\]\n",
    re.MULTILINE,
)


def signature(text: str) -> str:
    redacted = re.sub(
        r"(?i)(Bearer |api_key=|apikey=|password=|token=)\S+",
        r"\1[REDACTED]",
        text,
    )
    return re.sub(r"\d+", "#", redacted.lower().rstrip())


def raw_level(line: str) -> str:
    patterns = (
        (r"\[(fatal|critical|panic|error|err|severe|warning|warn|info|notice|debug|trace)\]", 1),
        (r"^\d\d-\d\d .*?\s([VDIWEF])\s", 1),
        (r"^\d{4}-\d\d-\d\d .*?,\s*(Info|Warning|Error)\s+CBS", 1),
        (r"\s(FATAL|CRITICAL|PANIC|ERROR|ERR|SEVERE|WARNING|WARN|INFO|NOTICE|DEBUG|TRACE)\s", 1),
    )
    value = ""
    for pattern, group in patterns:
        match = re.search(pattern, line, re.IGNORECASE)
        if match:
            value = match.group(group).upper()
            break
    if value in {"F", "FATAL", "CRITICAL", "PANIC"}:
        return "CRITICAL"
    if value in {"E", "ERROR", "ERR", "SEVERE"}:
        return "ERROR"
    if value in {"W", "WARN", "WARNING"}:
        return "WARNING"
    if value in {"I", "D", "V", "INFO", "NOTICE", "DEBUG", "TRACE"}:
        return "INFO"
    if re.search(r"panic|fatal", line, re.IGNORECASE):
        return "CRITICAL"
    if re.search(r"error|failed|failure|refused|not found", line, re.IGNORECASE):
        return "ERROR"
    if re.search(r"warn", line, re.IGNORECASE):
        return "WARNING"
    return "UNKNOWN"


def parsed_section(report: str) -> str:
    marker = "## Parsed\n\n```text\n"
    return report.split(marker, 1)[1].split("\n```", 1)[0]


def parsed_events(text: str):
    matches = list(HEADER.finditer(text))
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        yield match.group(1), int(match.group(2) or 1), text[match.end():end].rstrip()


def audit(path: Path) -> dict:
    raw_lines = [line.rstrip() for line in path.read_text().splitlines() if line.strip()]
    report = (REPORT_DIR / f"{path.name}.md").read_text()
    parsed = list(parsed_events(parsed_section(report)))
    raw_signatures = collections.Counter(signature(line) for line in raw_lines)
    parsed_signatures = collections.Counter()
    parsed_levels = collections.Counter()
    for level, repeats, content in parsed:
        parsed_signatures[signature(content)] += repeats
        parsed_levels[level] += repeats
    raw_levels = collections.Counter(raw_level(line) for line in raw_lines)
    checks = {
        "events": len(raw_lines) == sum(item[1] for item in parsed),
        "signatures": raw_signatures == parsed_signatures,
        "severity": raw_levels == parsed_levels,
    }
    return {
        "file": path.name,
        "status": "OK" if all(checks.values()) else "REVIEW",
        "raw_events": len(raw_lines),
        "parsed_events": sum(item[1] for item in parsed),
        "checks": checks,
        "missing_signatures": sum((raw_signatures - parsed_signatures).values()),
        "extra_signatures": sum((parsed_signatures - raw_signatures).values()),
        "raw_severity": dict(raw_levels),
        "parsed_severity": dict(parsed_levels),
    }


def main() -> None:
    results = [audit(path) for path in sorted(RAW_DIR.glob("*.log"))]
    lines = [
        "# Independent parser audit",
        "",
        "Python comparison of Raw versus Parsed; Rust parser code is not imported.",
        "",
        "| File | Status | Events | Signatures | Severity | Missing/extra |",
        "|---|---|---:|---|---|---:|",
    ]
    for item in results:
        checks = item["checks"]
        lines.append(
            f"| {item['file']} | {item['status']} | {item['raw_events']}→{item['parsed_events']} "
            f"| {'OK' if checks['signatures'] else 'FAIL'} | "
            f"{'OK' if checks['severity'] else 'FAIL'} | "
            f"{item['missing_signatures']}/{item['extra_signatures']} |"
        )
    summary = collections.Counter(item["status"] for item in results)
    lines += ["", f"Result: **{summary['OK']} OK / {summary['REVIEW']} REVIEW**.", ""]
    (REPORT_DIR / "AUDIT.md").write_text("\n".join(lines))
    (REPORT_DIR / "audit.json").write_text(json.dumps(results, indent=2) + "\n")


if __name__ == "__main__":
    main()
