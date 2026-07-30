# Parser fixtures

- `broken-rust/`: reproducible Cargo/rustc compiler diagnostics.
- `generated/runtime/`: locally generated compiler, runtime, CLI, Docker,
  PostgreSQL, npm, and Git errors with generation metadata.
- `external/loghub/`: sanitized excerpts from 16 public Loghub datasets with
  pinned-source provenance.

Regenerate:

```bash
python3 tools/generate_runtime_fixtures.py
python3 tools/collect_loghub_fixtures.py
```

Fixtures are development inputs and are not included in packaged applications.
