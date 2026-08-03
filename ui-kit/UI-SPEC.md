# Error Examiner UI specification

Reusable shell for Demo 2. The domain logic is intentionally excluded.

## Stack and viewport

- Rust 2021, `eframe/egui 0.27`, `image 0.25`, `serde 1`.
- Initial window: `720 × 620`; minimum: `420 × 360`.
- Resizable, frameless, transparent viewport; Windows starts hidden in tray.
- Custom resize hit area: `12 px` on every edge and corner.
- Header: `78 px`; inner margin `14 × 5`; border `0.5 px`.
- Center header icon: `30 × 30`; icon source: 256 px RGBA PNG.

## Typography and geometry

- Embedded `Noto Sans Regular`; first proportional font, independent of OS.
- Body `14 px`; button `13 px`; heading `20 px`; product title `15 px`.
- Item spacing `8 × 7`; button padding `10 × 5`.
- Window radius `12 px`; message cards `10 px`; controls/cards `8 px`.
- Main input panel: default `210 px`, min `150 px`, max `480 px`.
- Input panel min with prepared file: `280 px`; inner margin `12 px`.
- Chat tab editor: `150 × 24`.

## Palettes

Order: background, panel, card, input, border, text, muted, accent, user,
assistant.

- Black: `#070708 #0A0A0B #0F0F11 #0C0C0E #26262A #F4F4F5
  #A1A1AA #52525B #F87171 #4ADE80`
- Forest: `#06100C #091811 #0D2218 #081B12 #1E402F #E7F8EE
  #87AA95 #1C704B #F87171 #4ADE80`
- Midnight (default): `#080F1E #0A1222 #0F172A #091120 #1E293B
  #E2E8F0 #94A3B8 #2563EB #F87171 #32D583`
- Light: `#F7F8FA #FFFFFF #FFFFFF #FFFFFF #E2E8F0 #111827 #64748B
  #2563EB #DC2626 #059669`

Opacity range is `0.3..=1.0`. Apply alpha only to background, panel, card,
and input; keep text, borders, accents, and status colors opaque.

## Interaction rules

- One process owns window and tray.
- Header exposes Pin, WinLock, minimize, maximize/restore, and close-to-tray.
- Window dragging starts from either product-title word.
- WinLock disables moving and resizing; Pin toggles always-on-top.
- Rendering is event-driven when idle; no permanent repaint/FPS loop.
- Every long content area owns its own vertical scroll region.
- Keyboard shortcuts use physical key codes where global-hotkey permits it.

## Included source snapshots

- `theme.rs`: fonts, palettes, opacity, widget styling, icon loading.
- `ui_chrome.rs`: header, controls, navigation, resize hit testing.
- `ui_tabs.rs`: compact editable tab strip.
- `NotoSans-Regular.ttf`: embedded cross-platform UI font.
- `app-icon.png`: replace for Demo 2 while keeping the same loading path.
