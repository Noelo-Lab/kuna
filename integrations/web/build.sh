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

# The demo supports WHATEVER THE CLI SUPPORTS — every format (ELF/PE/Mach-O/COFF)
# and every architecture kuna ships a `.sla` for — with no per-arch manifest. The
# engine resolves the language for any binary; the browser preloads the small
# runtime files (the ~1.7 MB of `.ldefs`/`.pspec`/`.cspec`/`.dwarf`, which lets
# the engine resolve ANY binary) and lazily fetches just the one resolved `.sla`
# on demand. So we ship the full RUNTIME spec tree (not the `.sinc`/`.slaspec`
# SLEIGH *source*, which the decompiler never reads) plus a preload bundle of the
# small files. See docs/web-integration.md §3.
RUNTIME_EXTS=(ldefs pspec cspec dwarf sla)   # what the decompiler actually reads
SMALL_EXTS=(ldefs pspec cspec dwarf)          # everything but the big .sla (preloaded)

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
# The site: the landing page at /, the decompiler application at /decompile/,
# and the shared design system (css/fonts/images/highlighter) under /assets/.
# kuna-web.js, zip.js, the wasm and specs stay at the root — /decompile/ reaches
# them with '../', and the Node tests serve dist/ the same way a browser does.
# CNAME rides along so the deployed bundle claims kuna.noelo.org.
cp "$HERE/index.html" "$HERE/compare-samples.js" "$HERE/kuna-web.js" "$HERE/zip.js" \
   "$HERE/CNAME" "$DIST/"
cp -r "$HERE/assets" "$DIST/assets"
cp -r "$HERE/decompile" "$DIST/decompile"
cp -r "$HERE/vendor" "$DIST/vendor"

# Optionally shrink the wasm (nice-to-have; the demo works without it).
if command -v wasm-opt >/dev/null; then
  echo ">> wasm-opt -Oz"
  wasm-opt -Oz "$WASM" -o "$DIST/kuna_wasm.wasm"
else
  echo ">> (wasm-opt not found — shipping unoptimized wasm)"
  cp "$WASM" "$DIST/kuna_wasm.wasm"
fi

# Sanity: the .sla are built artifacts — fail early with a helpful message.
if [ -z "$(find "$REPO/specs" -name '*.sla' -print -quit)" ]; then
  echo "error: no .sla found under $REPO/specs"
  echo "       run 'make specs' at the repo root first (the .sla are built artifacts)."
  exit 1
fi

echo ">> copying the full runtime SLEIGH tree (all formats/arches; lazy-fetched)"
find_expr=()
for e in "${RUNTIME_EXTS[@]}"; do find_expr+=(-name "*.$e" -o); done
unset 'find_expr[${#find_expr[@]}-1]'   # drop trailing -o
( cd "$REPO/specs" && find . \( "${find_expr[@]}" \) -type f -print0 ) \
  | ( cd "$REPO/specs" && rsync -q --files-from=- -0 . "$DIST/specs/" 2>/dev/null ) \
  || {  # rsync may be absent — fall back to cp
    ( cd "$REPO/specs" && find . \( "${find_expr[@]}" \) -type f -print0 \
      | while IFS= read -r -d '' f; do mkdir -p "$DIST/specs/$(dirname "$f")"; cp "$f" "$DIST/specs/$f"; done )
  }

echo ">> building the small-files preload bundle (specs-small.json)"
small_find=()
for e in "${SMALL_EXTS[@]}"; do small_find+=(-name "*.$e" -o); done
unset 'small_find[${#small_find[@]}-1]'
SPECS_DIR="$REPO/specs" DEST="$DIST/specs-small.json" python3 - "${SMALL_EXTS[@]}" <<'PY'
import os, sys, json, base64
root = os.environ["SPECS_DIR"]; dest = os.environ["DEST"]; exts = set("." + e for e in sys.argv[1:])
out = {}
for dp, _, fns in os.walk(root):
    for fn in fns:
        if os.path.splitext(fn)[1] in exts:
            p = os.path.join(dp, fn)
            rel = os.path.relpath(p, root).replace(os.sep, "/")
            with open(p, "rb") as f:
                out[rel] = base64.b64encode(f.read()).decode("ascii")
with open(dest, "w") as f:
    json.dump(out, f, separators=(",", ":"))
print(f"   {len(out)} small files bundled -> {os.path.relpath(dest, os.path.dirname(dest))}")
PY

WSZ=$(du -h "$DIST/kuna_wasm.wasm" | cut -f1)
SSZ=$(du -sh "$DIST/specs" | cut -f1)
BSZ=$(du -h "$DIST/specs-small.json" | cut -f1)
echo ">> done. dist/ ready (wasm $WSZ, specs $SSZ on-server, preload bundle $BSZ)."
echo "   serve it:  (cd '$DIST' && python3 -m http.server 8000)  then open http://localhost:8000"

if [ "${1:-}" = "--serve" ]; then
  echo ">> serving on http://localhost:8000 (Ctrl-C to stop)"
  ( cd "$DIST" && exec python3 -m http.server 8000 )
fi
