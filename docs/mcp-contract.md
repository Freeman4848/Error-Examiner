# Error Examiner MCP contract

The MCP server exposes `prepare_error_log`; the client model decides whether to call it.

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
