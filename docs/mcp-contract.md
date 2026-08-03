# Error Examiner MCP contract v1

The MCP server exposes `prepare_error_log`; the client model decides whether to call it.

## Input and output

Provide exactly one of `path` or inline `text`. `mode` is `auto`, `raw`, `parsed`, or
`compare`; `batch_index` is zero-based and `batch_chars` is clamped to 3,000–50,000.
`auto` returns the smaller safe representation, while unknown formats always fall back to Raw.
The response reports format, selected variant, counts, token estimates, batch count, and content.

Files are limited to 10 MiB by default and must resolve inside `EE_MCP_ROOTS`. Override the
limit with `EE_MCP_MAX_INPUT_BYTES` (maximum 100 MiB). Panics are contained and become a
structured `internal_panic` response.

## Mandatory JSONL audit

Every call appends one row to `events.jsonl`. Logging cannot be disabled. Rows contain:

`timestamp`, `request_id`, `client`, `tool`, `input_name`, `detected_format`, `status`,
`raw_chars`, `parsed_chars`, `raw_tokens`, `parsed_tokens`, `sent_variant`, `events`,
`important_events`, `duplicates`, `batches`, `duration_ms`, and `error_code`.

Raw or parsed content, API keys, headers, cookies, and tokens are never written to audit rows.

## Optional artifacts

`save_artifacts` controls content storage. When enabled, Raw and Parsed are stored separately
under a timestamped request directory using the sanitized input name. When disabled, only the
mandatory JSONL audit row remains.

## Selection

The tool compares estimated Raw and Parsed token counts and returns the smaller representation.
Unknown formats return Raw. The result states `sent_variant` explicitly.

## Local Codex

Run as a local stdio server; it opens no network port. Set `EE_MCP_ROOTS` to the narrowest
directories the agent may read and `EE_MCP_DATA_DIR` to the audit directory. Codex should call
`auto` first and request later batches, `compare`, or `raw` only when evidence is insufficient.
