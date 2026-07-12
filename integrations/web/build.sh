#!/usr/bin/env bash
# build.sh — assemble the self-contained in-browser decompiler demo.
#
# Produces integrations/web/dist/, a static site that runs the kuna decompiler
# entirely client-side (WebAssembly + WASI shim). Serve it with any static file
# server; nothing talks to a backend.
#
#   integrations/web/build.sh          # build + assemble dist/
#   integrations/web/build.sh --serve  # then serve dist/ on :8000
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../.." && pwd)"
ENGINE="$REPO/decompiler"
DIST="$HERE/dist"
PROFILE="release"
TARGET="wasm32-wasip1"

# The minimal x86-64 (gcc/ELF) SLEIGH set — verified byte-identical to the full
# 29 MB spec tree. Keep in sync with X86_64_SPECS in kuna-web.js.
SPEC_FILES=(
  "Ghidra/Processors/x86/data/languages/x86.ldefs"
  "Ghidra/Processors/x86/data/languages/x86-64.sla"
  "Ghidra/Processors/x86/data/languages/x86-64.pspec"
  "Ghidra/Processors/x86/data/languages/x86-64-gcc.cspec"
  "Ghidra/Processors/x86/data/languages/x86-64.dwarf"
)

echo ">> checking toolchain"
command -v cargo >/dev/null || { echo "error: cargo not found"; exit 1; }
if ! rustup target list --installed 2>/dev/null | grep -q "^$TARGET$"; then
  echo ">> installing rust target $TARGET"
  rustup target add "$TARGET"
fi

echo ">> building kuna_wasm ($TARGET, $PROFILE)"
( cd "$ENGINE" && cargo build --"$PROFILE" --target "$TARGET" -p kuna-wasm )
WASM="$ENGINE/target/$TARGET/$PROFILE/kuna_wasm.wasm"
[ -f "$WASM" ] || { echo "error: wasm not produced at $WASM"; exit 1; }

echo ">> assembling $DIST"
rm -rf "$DIST"
mkdir -p "$DIST/specs"
cp "$HERE/index.html" "$HERE/kuna-web.js" "$DIST/"
cp -r "$HERE/vendor" "$DIST/vendor"

# Optionally shrink the wasm (nice-to-have; the demo works without it).
if command -v wasm-opt >/dev/null; then
  echo ">> wasm-opt -Oz"
  wasm-opt -Oz "$WASM" -o "$DIST/kuna_wasm.wasm"
else
  echo ">> (wasm-opt not found — shipping unoptimized wasm)"
  cp "$WASM" "$DIST/kuna_wasm.wasm"
fi

echo ">> copying SLEIGH specs (x86-64)"
for f in "${SPEC_FILES[@]}"; do
  src="$REPO/specs/$f"
  if [ ! -f "$src" ]; then
    echo "error: missing spec $src"
    echo "       run 'make specs' at the repo root first (the .sla are built artifacts)."
    exit 1
  fi
  mkdir -p "$DIST/specs/$(dirname "$f")"
  cp "$src" "$DIST/specs/$f"
done

WSZ=$(du -h "$DIST/kuna_wasm.wasm" | cut -f1)
SSZ=$(du -sh "$DIST/specs" | cut -f1)
echo ">> done. dist/ ready (wasm $WSZ, specs $SSZ)."
echo "   serve it:  (cd '$DIST' && python3 -m http.server 8000)  then open http://localhost:8000"

if [ "${1:-}" = "--serve" ]; then
  echo ">> serving on http://localhost:8000 (Ctrl-C to stop)"
  ( cd "$DIST" && exec python3 -m http.server 8000 )
fi
