#!/usr/bin/env python3
"""Convert Gemini's taxonomy into an executable two-stage parser catalog."""

import json
import sys
from pathlib import Path

JSON_CONTAINERS = {"json-object", "json-array", "ndjson"}


def predicates(values):
    result = []
    for value in values:
        predicate = "json_path_exists" if value.startswith("$.") else "regex"
        result.append({"type": predicate, "value": value})
    return result


def detection(node):
    source = node.get("detect", {})
    return {
        "required": predicates(source.get("required", [])),
        "any": predicates(source.get("any", [])),
        "forbidden": predicates(source.get("forbidden", [])),
    }


def leaves(node, path=()):
    children = node.get("children", [])
    if not children:
        yield path, node
        return
    for child in children:
        yield from leaves(child, path + (node,))


def decoder(node):
    container_id = node["id"]
    return {
        "id": container_id,
        "detect": detection(node),
        "emits": "json-record" if container_id in JSON_CONTAINERS else "text-record",
        "iteration": {
            "json-object": "single",
            "json-array": "array-items",
            "ndjson": "lines",
            "structured-text": "lines",
            "multiline-text": "blocks",
            "unknown": "single",
        }[container_id],
        "fallback": "raw",
    }


def format_leaf(container, ancestors, leaf):
    rules = [
        {"scope": node["id"], "detect": detection(node)}
        for node in ancestors[1:]
        if any(node.get("detect", {}).values())
    ]
    if any(leaf.get("detect", {}).values()):
        rules.append({"scope": leaf["id"], "detect": detection(leaf)})
    return {
        "id": leaf["id"],
        "label": leaf["label"],
        "record_kind": (
            "json-record" if container["id"] in JSON_CONTAINERS else "text-record"
        ),
        "family_path": [node["id"] for node in ancestors[1:]],
        "match_groups": rules,
        "confidence": leaf["confidence"],
        "parser_id": leaf["parser_id"],
        "canonical_fields": leaf["canonical_fields"],
        "event_boundary": leaf["event_boundary"],
        "severity_mapping": leaf["severity_mapping"],
        "fixture_plan": leaf["fixture_plan"],
        "fallback": "raw",
    }


def validate(pipeline):
    formats = pipeline["classifier"]["formats"]
    ids = [item["id"] for item in formats]
    errors = []
    if len(ids) != len(set(ids)):
        errors.append("duplicate format ids")
    for item in formats:
        if not item["parser_id"]:
            errors.append(f'{item["id"]}: missing parser_id')
        confidence = item["confidence"]
        if not 0 <= confidence["medium"] <= confidence["high"] <= 1:
            errors.append(f'{item["id"]}: invalid confidence thresholds')
        if item["fallback"] != "raw":
            errors.append(f'{item["id"]}: fallback must be raw')
    if errors:
        raise ValueError("; ".join(errors))


def main():
    source = Path(
        sys.argv[1] if len(sys.argv) > 1 else "parser-catalog/gemini-taxonomy-v1.json"
    )
    target = Path(
        sys.argv[2] if len(sys.argv) > 2 else "parser-catalog/parser-pipeline-v2.json"
    )
    catalog = json.loads(source.read_text(encoding="utf-8"))
    containers = catalog["root"]["children"]
    formats = []
    for container in containers:
        for ancestors, leaf in leaves(container):
            if leaf["kind"] != "fallback":
                formats.append(format_leaf(container, ancestors, leaf))
    pipeline = {
        "schema_version": 2,
        "source_schema_version": catalog["schema_version"],
        "decoders": [decoder(node) for node in containers],
        "classifier": {"formats": formats, "fallback": "raw"},
    }
    validate(pipeline)
    target.write_text(
        json.dumps(pipeline, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    print(f"ok: {len(pipeline['decoders'])} decoders, {len(formats)} formats")


if __name__ == "__main__":
    main()
