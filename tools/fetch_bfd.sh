#!/usr/bin/env bash
# Fetch libbfd (from binutils-dev + libbinutils) into a LOCAL prefix without root.
#
# decomp_dbg / decomp_opt / decomp_test_dbg link against libbfd. The normal way to
# provide it is `sudo apt-get install -y binutils-dev`. When you have no root, run
# this script and then build with:
#
#     ./tools/fetch_bfd.sh
#     make BFD_PREFIX="$(pwd)/.bfdlocal"
#
# It downloads the .deb packages (apt-get download needs no root) and extracts them
# into ./.bfdlocal (gitignored). Debian/Ubuntu only.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${1:-$ROOT/.bfdlocal}"

if ! command -v apt-get >/dev/null 2>&1; then
  echo "error: apt-get not found; this helper is Debian/Ubuntu only." >&2
  echo "       Install libbfd another way and pass BFD_PREFIX to make." >&2
  exit 1
fi

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "Downloading binutils-dev + libbinutils into $WORK ..."
( cd "$WORK" && apt-get download binutils-dev libbinutils )

mkdir -p "$PREFIX"
for deb in "$WORK"/*.deb; do
  echo "Extracting $(basename "$deb") -> $PREFIX"
  dpkg-deb -x "$deb" "$PREFIX"
done

# Verify both halves of what the Makefile's BFD_PREFIX support needs: the header
# AND the shared lib. The lib lands under the dpkg multiarch dir (which need not
# match `gcc -dumpmachine`), so locate it by searching rather than guessing.
BFD_SO="$(find "$PREFIX/usr/lib" -name 'libbfd.so' -print -quit 2>/dev/null || true)"
if [ ! -f "$PREFIX/usr/include/bfd.h" ]; then
  echo "error: bfd.h not found under $PREFIX/usr/include after extraction." >&2
  exit 1
fi
if [ -z "$BFD_SO" ]; then
  echo "error: libbfd.so not found under $PREFIX/usr/lib after extraction." >&2
  exit 1
fi
echo
echo "libbfd ready at: $PREFIX"
echo "  header: $PREFIX/usr/include/bfd.h"
echo "  lib:    $BFD_SO"
echo
echo "Now build with:  make BFD_PREFIX=\"$PREFIX\" binaries"
