#!/usr/bin/env bash
#
# One-command build + deploy for the KunaDecompiler Ghidra extension: compiles the
# `kuna_ghidra` backend binary, stages it into this extension's `os/<platform>/`
# directory (a gitignored build artifact), and — if GHIDRA_INSTALL_DIR is set —
# packages the installable extension zip.
#
#   ./build.sh                              # build + stage the binary only
#   GHIDRA_INSTALL_DIR=/path ./build.sh     # also build the installable zip
#
# See README.md ("Run the kuna backend in a real Ghidra instance") for the full
# runbook, including the dev-checkout (`-Dghidra.external.modules`) path.
set -euo pipefail

# Resolve paths relative to this script (integrations/ghidra/KunaDecompiler/).
EXT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$EXT_DIR/../../.." && pwd)"

# Map the host to Ghidra's `Platform` enum directory name
# (Ghidra/Framework/Generic/.../Platform.java — note the underscore before 64 on arm).
os="$(uname -s)"; arch="$(uname -m)"
case "$os/$arch" in
	Linux/x86_64)         PLATFORM=linux_x86_64; EXE=kuna_ghidra ;;
	Linux/aarch64)        PLATFORM=linux_arm_64; EXE=kuna_ghidra ;;
	Darwin/x86_64)        PLATFORM=mac_x86_64;   EXE=kuna_ghidra ;;
	Darwin/arm64)         PLATFORM=mac_arm_64;   EXE=kuna_ghidra ;;
	*)
		echo "error: unsupported host '$os/$arch'. Build kuna_ghidra manually" >&2
		echo "       (cd decompiler && cargo build --release -p kuna-ghidra) and copy it" >&2
		echo "       into os/<ghidra-platform-dir>/ yourself." >&2
		exit 1 ;;
esac

echo ">> building the kuna_ghidra backend (release)"
( cd "$REPO_ROOT/decompiler" && cargo build --release -p kuna-ghidra )

echo ">> staging $EXE into os/$PLATFORM/"
mkdir -p "$EXT_DIR/os/$PLATFORM"
cp "$REPO_ROOT/decompiler/target/release/$EXE" "$EXT_DIR/os/$PLATFORM/$EXE"
chmod +x "$EXT_DIR/os/$PLATFORM/$EXE"

if [ -n "${GHIDRA_INSTALL_DIR:-}" ]; then
	echo ">> packaging the extension against $GHIDRA_INSTALL_DIR"
	( cd "$EXT_DIR" && gradle -PGHIDRA_INSTALL_DIR="$GHIDRA_INSTALL_DIR" buildExtension )
	echo ">> done. Install the zip in $EXT_DIR/dist/ via Ghidra: File -> Install Extensions..."
else
	echo ">> binary staged at os/$PLATFORM/$EXE."
	echo "   - release install: set GHIDRA_INSTALL_DIR and re-run to build the installable zip; or"
	echo "   - dev checkout: launch Ghidra with -Dghidra.external.modules=$EXT_DIR"
	echo "     (skip staging entirely with -Dkuna.decompiler.exe=$REPO_ROOT/decompiler/target/release/$EXE)."
fi
