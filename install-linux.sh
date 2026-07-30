#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
bin_dir="${HOME}/.local/bin"
app_dir="${HOME}/.local/share/applications"
icon_dir="${HOME}/.local/share/icons/hicolor/256x256/apps"

install -d "$bin_dir" "$app_dir" "$icon_dir"
install -m 755 "$root/error-explainer" "$bin_dir/error-explainer"
install -m 644 "$root/app-icon.png" "$icon_dir/error-explainer.png"
install -m 644 "$root/error-explainer.desktop" \
  "$app_dir/error-explainer.desktop"

echo "Installed Error Explainer. Start it from the application menu."
