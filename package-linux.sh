#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
cd "$root"

cargo build --release --bin error-examiner -j 2
package="dist/Error-Examiner-Linux-x86_64"
install -d "$package"
install -m 755 target/release/error-examiner "$package/"
install -m 755 install-linux.sh "$package/"
install -m 644 error-examiner.desktop "$package/"
install -m 644 assets/app-icon.png "$package/"
install -m 644 README.md LICENSE "$package/"
(
  cd dist
  tar -czf Error-Examiner-Linux-x86_64.tar.gz \
    Error-Examiner-Linux-x86_64
)
sha256sum dist/Error-Examiner-Linux-x86_64.tar.gz \
  >dist/Error-Examiner-Linux-x86_64.tar.gz.sha256
