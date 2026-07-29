# Error Explainer (EE)

Local tray application that sends logs, stack traces, and debugging questions
to an AI provider and returns a testable explanation.

## Features

- `Ctrl+Shift+Alt+C` shows or hides the window and pastes the clipboard when
  the input is empty.
- Local chat history with an explicit Clear action.
- API keys stay only in process memory and disappear on restart.
- Hard input/output limits, timeout, optional retries, token estimate, and
  user-supplied cost rates.
- Providers:
  - OpenAI Responses API with `store: false`
  - OpenAI-compatible APIs
  - LM Studio at `http://localhost:1234/v1`
  - Google Gemini
  - Anthropic
  - offline Demo mode
- Native Windows tray and StatusNotifier/AppIndicator tray on Linux.
- Settings and About & Help views.

## Screenshots

![Chat](screenshots/chat.png)

![Settings](screenshots/settings.png)

![About and Help](screenshots/about-help.png)

## Run locally

Linux:

```bash
cargo run --features linux-tray --bin error-explainer-tray
```

Run only the window:

```bash
cargo run --bin error-explainer
```

Build and test:

```bash
cargo test
cargo build --release --features linux-tray --bins
```

Windows:

```powershell
cargo run --release --bin error-explainer
```

The Windows executable owns its tray icon. The Linux package contains the UI
binary and a small tray launcher.

## Provider setup

Open Settings, select a provider, enter its current model ID and API key, then
use Test connection. Provider/model/base URL and limits are saved; the API key
is not.

LM Studio requires its local server to be running on port `1234`. Select
`LM Studio (local)` and enter the loaded model identifier; no key is required.

OpenAI-compatible supports services such as OpenRouter, Groq, Together,
Mistral, or a custom gateway by changing Base URL and Model.

## Privacy

- Submitted text is sent only after pressing Explain.
- The selected provider receives the bounded conversation included in the
  request.
- API keys are kept in RAM and are never written to settings, history, logs,
  screenshots, or exports.
- Chat history is stored locally in the platform configuration directory.
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
