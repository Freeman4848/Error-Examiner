# Error Examiner v0.1.0 — Linux x86_64

Semantic log preprocessing and AI-assisted error triage.

## Run

Extract the archive, then run:

```bash
./error-examiner
```

To add Error Examiner to the application menu:

```bash
bash install-linux.sh
```

The tray requires StatusNotifier/AppIndicator support.

## First run

Open **Settings**, select a provider, enter its exact model ID and API key when
required, then use **Test connection**. API keys are kept only in memory.

Use **Add log or image**, review the Raw/Parsed preview, then press **Explain**.
Unknown formats safely fall back to Raw.

The Cerebras connection has been smoke-tested. OpenAI, LM Studio, Gemini,
Anthropic, and custom compatible adapters are included but are not yet verified
in this release; use **Test connection** before submitting a log.

## Integrity

Verify the binary from the extracted directory:

```bash
sha256sum -c SHA256SUMS.txt
```

Bug reports: `freeman4848.dev@gmail.com`

## License

MIT. See `LICENSE`.
