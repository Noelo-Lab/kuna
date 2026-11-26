#!/usr/bin/env bash
# Put a Windows/PE-capable `clang` on PATH for the fixture tests.
#
# Several CLI and decompiler tests build PE fixtures with
# `clang --target={i686,x86_64}-w64-windows-gnu` and then compile the emitted C
# natively. A host whose clang is a Nix *wrapper* cannot do that: the wrapper
# forwards a Linux `--gcc-toolchain` and a Linux `ld.bfd`, which rejects PE
# (`unrecognised emulation mode: i386pep`).
#
# This script installs two shims in a scratch dir and prints the directory, so a
# test run can work without touching the system:
#
#     eval "$(scripts/pe-test-env.sh)"     # exports PATH
#     make rust-test
#
# The shims are:
#
#   clang  — sends Windows GNU targets to an unwrapped clang and everything
#            else to the normal (wrapped) clang, so native fixtures keep their
#            libc headers.
#   ld     — dispatches to the x86_64 or i686 MinGW `ld` by the emulation named on
#            the command line (`i386pep` must be tested before `i386pe`, which is
#            a prefix of it).
#
# The MinGW binutils come from `nix-shell`, which is why this script is
# Nix-specific. On the project's dev container the toolchains are installed
# directly and no shim is needed.

set -euo pipefail

fail() {
    echo "pe-test-env: $*" >&2
    exit 1
}

command -v nix-shell >/dev/null 2>&1 || fail "nix-shell is required to obtain the MinGW binutils"
command -v clang >/dev/null 2>&1 || fail "clang is required"

# Ask nix-shell for each cross linker. `--run` puts the toolchain on PATH, so a
# plain `command -v` resolves it; nothing is left to guess from store hashes.
cross_ld() {
    local pkg=$1 prefix=$2
    nix-shell -p "$pkg" --run "command -v $prefix-ld"
}

x64_ld=$(cross_ld pkgsCross.mingwW64.buildPackages.binutils-unwrapped x86_64-w64-mingw32)
x86_ld=$(cross_ld pkgsCross.mingw32.buildPackages.binutils-unwrapped i686-w64-mingw32)
[[ -x "${x64_ld:-}" ]] || fail "no x86_64 MinGW ld (is pkgsCross.mingwW64 available?)"
[[ -x "${x86_ld:-}" ]] || fail "no i686 MinGW ld (is pkgsCross.mingw32 available?)"

# The unwrapped clang: the same store tree as the wrapper, without `-wrapper`.
wrapper=$(command -v clang)
raw=''
version=$("$wrapper" --version 2>/dev/null | sed -n 's/^clang version \([0-9][0-9.]*\).*/\1/p' | head -1)
if [[ -n "$version" ]]; then
    for candidate in /nix/store/*-clang-"$version"/bin/clang; do
        [[ -x "$candidate" ]] || continue
        case "$candidate" in *wrapper*) continue ;; esac
        raw=$candidate
        break
    done
fi
[[ -n "$raw" && -x "$raw" ]] || fail "could not locate an unwrapped clang for $wrapper (version '${version:-unknown}')"

# The system linker this shim falls back to, resolved BEFORE the shim exists so
# the shim can never exec itself.
system_ld=$(command -v ld)
[[ -n "$system_ld" ]] || fail "no system ld to fall back to"

shim=$(mktemp -d "${TMPDIR:-/tmp}/kuna-pe-test-env.XXXXXX")

{
    printf '#!/bin/sh\n'
    printf 'target= next=\n'
    printf 'for arg do\n'
    printf '  if [ "$next" ]; then target=$arg; next=; continue; fi\n'
    printf '  case "$arg" in\n'
    printf '    -target|--target) next=1 ;;\n'
    printf '    --target=*) target=${arg#*=} ;;\n'
    printf '  esac\n'
    printf 'done\n'
    printf 'case "$target" in *-windows-gnu) exec %q "$@" ;; esac\n' "$raw"
    printf 'exec %q "$@"\n' "$wrapper"
} > "$shim/clang"

# Dispatch on the EMULATION the command line names, never on a substring of any
# argument: a MinGW linker invoked for an ELF fixture (`-m elf_i386`) must not be
# intercepted by a test for `i386pe`.
{
    printf '#!/bin/sh\n'
    printf 'for a in "$@"; do\n'
    printf '  case "$a" in i386pep|-mi386pep) exec %q "$@" ;; i386pe|-mi386pe) exec %q "$@" ;; esac\n' "$x64_ld" "$x86_ld"
    printf 'done\n'
    printf 'exec %q "$@"\n' "$system_ld"
} > "$shim/ld"

chmod +x "$shim/clang" "$shim/ld"

printf 'export PATH=%q:$PATH\n' "$shim"
echo "pe-test-env: shims at $shim" >&2
