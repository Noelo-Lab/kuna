#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
rustc +1.93.1 --edition=2021 -C opt-level=3 -C lto=fat -C codegen-units=1 -C debuginfo=1 -C strip=debuginfo \
    rust_fmt_str.rs -o rust_fmt_str_real_x86_64
