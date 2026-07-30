#!/usr/bin/env python3
"""Download small, sanitized Loghub excerpts for parser fixtures."""

import json
import re
import urllib.request
from pathlib import Path

COMMIT = "dd61d0952749ee7963bde24220d1be5ede023033"
DATASETS = [
    "Android",
    "Apache",
    "BGL",
    "HDFS",
    "HPC",
    "Hadoop",
    "HealthApp",
    "Linux",
    "Mac",
    "OpenSSH",
    "OpenStack",
    "Proxifier",
    "Spark",
    "Thunderbird",
    "Windows",
    "Zookeeper",
]
PRIORITY = re.compile(
    r"error|warn|fail|fatal|critical|exception|panic|denied|invalid",
    re.IGNORECASE,
)
REDACTIONS = [
    (re.compile(r"\b(?:\d{1,3}\.){3}\d{1,3}\b"), "<IP>"),
    (
        re.compile(
            r"\b[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-"
            r"[89ab][0-9a-f]{3}-[0-9a-f]{12}\b",
            re.IGNORECASE,
        ),
        "<UUID>",
    ),
    (re.compile(r"\b[\w.+-]+@[\w.-]+\.[A-Za-z]{2,}\b"), "<EMAIL>"),
]


def sanitize(line):
    for pattern, replacement in REDACTIONS:
        line = pattern.sub(replacement, line)
    return line.rstrip()


def choose(lines, limit=40):
    priority = [line for line in lines if PRIORITY.search(line)]
    normal = [line for line in lines if not PRIORITY.search(line)]
    selected = priority[: limit // 2] + normal[: limit - min(len(priority), limit // 2)]
    return [sanitize(line) for line in selected if line.strip()]


def main():
    output = Path("fixtures/external/loghub")
    output.mkdir(parents=True, exist_ok=True)
    records = []
    for dataset in DATASETS:
        relative = f"{dataset}/{dataset}_2k.log"
        url = f"https://raw.githubusercontent.com/logpai/loghub/{COMMIT}/{relative}"
        with urllib.request.urlopen(url, timeout=30) as response:
            lines = response.read().decode("utf-8", errors="replace").splitlines()
        selected = choose(lines)
        target = output / f"{dataset.lower()}.log"
        target.write_text("\n".join(selected) + "\n", encoding="utf-8")
        records.append(
            {
                "dataset": dataset,
                "source": url,
                "upstream_lines": len(lines),
                "fixture_lines": len(selected),
                "sanitized": True,
            }
        )
    provenance = {
        "source_repository": "https://github.com/logpai/loghub",
        "commit": COMMIT,
        "usage_note": "Public research dataset; retain attribution and cite Loghub.",
        "fixtures": records,
    }
    (output / "provenance.json").write_text(
        json.dumps(provenance, indent=2) + "\n", encoding="utf-8"
    )
    print(f"ok: {len(records)} datasets, {sum(r['fixture_lines'] for r in records)} lines")


if __name__ == "__main__":
    main()
