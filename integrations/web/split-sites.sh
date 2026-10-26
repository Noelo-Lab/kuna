#!/usr/bin/env bash
# split-sites.sh — split build.sh's dist/ into the two deployed sites.
#
# build.sh assembles one tree with the decompiler at /decompile/, which is what the
# tests serve. This moves the decompiler to the root of dist-run/ (decompile.run) and
# leaves dist/ as kuna.noelo.org: the landing page, /dev-viz/, and a /decompile/ that
# forwards to decompile.run with its query and fragment. Run it after build.sh.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST="$HERE/dist"
RUN="$HERE/dist-run"
SITE_URL="https://decompile.run/"
HOME_URL="https://kuna.noelo.org/"

[ -f "$DIST/decompile/index.html" ] && [ -f "$DIST/kuna_wasm.wasm" ] \
  || { echo "error: $DIST is not a fresh build.sh bundle; run build.sh first"; exit 1; }

echo ">> assembling $RUN"
rm -rf "$RUN"
mkdir -p "$RUN"
for f in kuna-web.js kuna-worker.js kuna-worker-client.js zip.js sha256.js \
         kuna_wasm.wasm specs-small.json specs vendor decompile; do
  mv "$DIST/$f" "$RUN/"
done
cp -r "$DIST/assets" "$RUN/assets"
sed -e 's#href="\.\./assets/#href="./assets/#g' \
    -e 's#href="\./decompile\.css"#href="./decompile/decompile.css"#' \
    -e 's#src="\./app\.js"#src="./decompile/app.js"#' \
    -e "s#href=\"\.\./\"#href=\"$HOME_URL\"#g" \
    "$RUN/decompile/index.html" > "$RUN/index.html"
rm "$RUN/decompile/index.html"
if grep -n -E '(href|src)="\.\./|(href|src)="\./(app\.js|decompile\.css)"' "$RUN/index.html"; then
  echo "error: decompile/index.html gained a relative path split-sites.sh does not rewrite"
  exit 1
fi
cp "$HERE/decompile-run/README.md" "$HERE/decompile-run/CNAME" "$RUN/"
touch "$RUN/.nojekyll"

echo ">> pointing $DIST/decompile/ at $SITE_URL"
mkdir -p "$DIST/decompile"
cp "$HERE/decompile-run/redirect.html" "$DIST/decompile/index.html"
sed -i -e "s#href=\"\./decompile/\"#href=\"$SITE_URL\"#g" "$DIST/index.html"
sed -i -e "s#href=\"\.\./decompile/\"#href=\"$SITE_URL\"#g" "$DIST/dev-viz/index.html"
if grep -n -E 'href="(\.\.?/)decompile/"' "$DIST/index.html" "$DIST/dev-viz/index.html"; then
  echo "error: a site page still links to the local /decompile/"
  exit 1
fi
echo ">> done. $RUN -> decompile.run ($(du -sh "$RUN" | cut -f1)), $DIST -> kuna.noelo.org ($(du -sh "$DIST" | cut -f1))"
