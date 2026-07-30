#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
cd "$root"

cargo build --release --bin error-explainer
package="dist/Error-Explainer-Linux-x86_64"
install -d "$package"
install -m 755 target/release/error-explainer "$package/"
install -m 755 install-linux.sh "$package/"
install -m 644 error-explainer.desktop "$package/"
install -m 644 assets/app-icon.png "$package/"
install -m 644 README.md LICENSE "$package/"
(
  cd dist
  tar -czf Error-Explainer-Linux-x86_64.tar.gz \
    Error-Explainer-Linux-x86_64
)
sha256sum dist/Error-Explainer-Linux-x86_64.tar.gz \
  >dist/Error-Explainer-Linux-x86_64.tar.gz.sha256
