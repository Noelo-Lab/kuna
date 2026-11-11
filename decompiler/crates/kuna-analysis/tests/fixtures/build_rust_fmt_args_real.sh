#!/bin/sh
set -eu
cd "$(dirname "$0")"
rustc +1.93.1 --edition=2021 -C opt-level=2 -C debuginfo=1 -C strip=debuginfo rust_fmt_args.rs -o rust_fmt_args_real_x86_64
