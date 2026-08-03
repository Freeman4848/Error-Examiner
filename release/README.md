# Error Examiner v0.1.0

Local semantic log preprocessing and AI-assisted error triage.

## Windows 10/11 x86_64

1. Extract the portable ZIP.
2. Run `Error-Examiner.exe`.
3. Optionally run `Create Desktop Shortcut.cmd`.

No installer is required. The release build is produced with Rust MSVC.

## Linux x86_64

Run directly from the extracted directory:

```bash
./error-examiner
```

For an application-menu entry, run:

```bash
bash install-linux.sh
```

The tray requires StatusNotifier/AppIndicator support.

## First Run

Open **Settings**, choose a provider, enter its current model ID and API key
when required, and use **Test connection**. API keys stay only in memory.

Use **Add log or image**, review Raw/Parsed preview, then press **Explain**.
Unknown formats safely fall back to Raw.

## Integrity

Compare the downloaded file with the bundled or adjacent SHA-256 checksum.
Source code and build workflow are published in the repository.

Bug reports: `freeman4848.dev@gmail.com`

## License

MIT. See `LICENSE`.
