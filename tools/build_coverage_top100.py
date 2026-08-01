#!/usr/bin/env python3
import csv
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "parser-catalog/application-log-top100.csv"
OUTPUT = ROOT / "review/coverage-top100.md"


def main() -> None:
    rows = list(csv.DictReader(SOURCE.open()))
    if len(rows) != 100 or [int(row["rank"]) for row in rows] != list(range(1, 101)):
        raise SystemExit("catalog must contain ranks 1..100")
    counts = Counter(row["coverage"] for row in rows)
    lines = [
        "# EE coverage: 100 high-volume application log sources",
        "",
        "There is no official global size ranking for application logs. Ordering uses practical",
        "volume tiers and engineering priority; positions inside a tier are approximate.",
        "",
        f"Coverage: **{counts['covered']} covered / {counts['partial']} partial / {counts['raw']} Raw**.",
        "",
        "| # | Volume | Application / source | EE | Adapter or gap |",
        "|---:|---|---|---|---|",
    ]
    for row in rows:
        lines.append(
            f"| {row['rank']} | {row['volume']} | {row['application']} | "
            f"{row['coverage']} | {row['adapter_or_gap']} |"
        )
    lines += [
        "",
        "Definitions: `covered` = executable adapter plus fixture; `partial` = only common variants;",
        "`raw` = safe Raw fallback, therefore a candidate for a future adapter.",
        "",
        "References: [LogHub benchmark](https://github.com/logpai/loghub), "
        "[OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) "
        "and [mapping appendix](https://opentelemetry.io/docs/specs/otel/logs/data-model-appendix/).",
        "",
    ]
    OUTPUT.write_text("\n".join(lines))


if __name__ == "__main__":
    main()
