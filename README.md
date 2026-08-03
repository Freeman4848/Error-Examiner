# Error Examiner

Local tray application that sends logs, stack traces, and debugging questions
to an AI provider and returns a testable explanation.

Bug reports: `freeman4848.dev@gmail.com`

## Features

- `Ctrl+Shift+Alt+C` shows or hides the window and pastes the clipboard when
  the input is empty.
- Local chat history with an explicit Clear action.
- Named chat tabs, timestamps, file preview, and image attachments.
- Optional log preparation with format-aware parsing, deduplication, redaction,
  token estimates, safe batching, and Raw fallback.
- API keys stay only in process memory and disappear on restart.
- Hard input/output limits, timeout, optional retries, token estimate, and
  user-supplied cost rates.
- Providers:
  - OpenAI Responses API with `store: false`
  - OpenAI-compatible APIs
  - Cerebras
  - LM Studio native REST API at `http://localhost:1234`
  - Google Gemini
  - Anthropic
  - Custom AI API using OpenAI-, Gemini-, or Anthropic-compatible protocols
  - offline Demo mode
- Tray integrated into the same executable on Windows and Linux.
- Executable parser schema with 57 formats, 83 verified cases, and Raw fallback.
- Raw/Parsed size comparison sends the smaller representation to the model.
- Settings and About & Help views.

## Screenshots

![Chat](screenshots/chat.png)

![Settings](screenshots/settings.png)

![About and Help](screenshots/about-help.png)

## Run locally

Linux:

```bash
cargo run --bin error-examiner
```

Build and test:

```bash
cargo test
cargo build --release --bin error-examiner
```

Windows:

```powershell
cargo run --release --bin error-examiner
```

One executable owns both the window and tray icon on Windows and Linux.

## Provider setup

Open Settings, select a provider, enter its current model ID and API key, then
use Test connection. Provider/model/base URL and limits are saved; the API key
is not.

LM Studio requires its local server on port `1234`. Select `LM Studio
(local)`; leave Model empty to auto-detect a loaded local LLM. No key is
required unless authentication was enabled in LM Studio.

OpenAI-compatible supports services such as OpenRouter, Groq, Together,
Mistral, or a custom gateway by changing Base URL and Model.

Use **Add log or image** to attach a file. Prepared logs can be inspected in
Raw, Parsed, or Compare mode before sending.

## Privacy

- Submitted text is sent only after pressing Explain.
- The selected provider receives the bounded conversation included in the
  request.
- API keys are kept in RAM and are never written to settings, history, logs,
  screenshots, or exports.
- Chat history is stored locally in the platform configuration directory.
- Pasted text and attached screenshots become part of local chat history.
- Logs can contain secrets or personal data; review them before submission.
- Model conclusions are hypotheses and must be verified.

## Limits and billing

The default budget is approximately 8,000 input tokens (24,000 characters)
and 1,500 output tokens. Automatic retries are disabled by default because an
ambiguous timeout can still have consumed tokens. Cost is displayed only when
the user enters current per-million-token prices.

## License

MIT. The bundled Noto Sans font retains its upstream license in
`third_party/Noto-Sans-COPYRIGHT`.
