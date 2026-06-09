#!/usr/bin/env bash
# Install the GitHub CLI (`gh`) into ~/.local WITHOUT root, and authenticate it from the
# token already in ~/.git-credentials, so the pipeline's workers can `gh pr create`.
# Idempotent: re-running upgrades/repairs. Verifies the token has the scope to open PRs.
set -uo pipefail

VERSION="${GH_VERSION:-2.62.0}"
ARCH="amd64"
PREFIX="$HOME/.local"
SHARE="$PREFIX/share/gh-$VERSION"
BIN="$PREFIX/bin/gh"
TARBALL="gh_${VERSION}_linux_${ARCH}.tar.gz"
URL="https://github.com/cli/cli/releases/download/v${VERSION}/${TARBALL}"

mkdir -p "$PREFIX/bin" "$PREFIX/share"

if command -v gh >/dev/null 2>&1; then
  echo "gh already on PATH: $(command -v gh) ($(gh --version | head -1))"
else
  if [ ! -x "$SHARE/bin/gh" ]; then
    echo "downloading $URL"
    tmp="$(mktemp -d)"
    curl -fsSL "$URL" -o "$tmp/$TARBALL" || { echo "download failed"; exit 1; }
    tar -xzf "$tmp/$TARBALL" -C "$tmp"
    rm -rf "$SHARE"
    mv "$tmp/gh_${VERSION}_linux_${ARCH}" "$SHARE"
    rm -rf "$tmp"
  fi
  ln -sf "$SHARE/bin/gh" "$BIN"
  echo "installed gh -> $BIN"
  case ":$PATH:" in
    *":$PREFIX/bin:"*) ;;
    *) echo "NOTE: add $PREFIX/bin to your PATH (echo 'export PATH=\"$PREFIX/bin:\$PATH\"' >> ~/.bashrc)";;
  esac
fi

export PATH="$PREFIX/bin:$PATH"

# --- authenticate from the existing git credential --------------------------
if gh auth status >/dev/null 2>&1; then
  echo "gh already authenticated."
else
  CRED="$HOME/.git-credentials"
  TOKEN=""
  if [ -f "$CRED" ]; then
    # line like https://<user>:<token>@github.com
    TOKEN="$(sed -n 's#https://[^:]*:\([^@]*\)@github.com#\1#p' "$CRED" | head -1)"
  fi
  [ -z "$TOKEN" ] && TOKEN="${GH_TOKEN:-${GITHUB_TOKEN:-}}"
  if [ -z "$TOKEN" ]; then
    echo "no token found in ~/.git-credentials or \$GH_TOKEN; run: gh auth login"
    exit 2
  fi
  echo "authenticating gh with the existing github.com token"
  echo "$TOKEN" | gh auth login --hostname github.com --git-protocol ssh --with-token || {
    echo "gh auth login failed"; exit 2; }
fi

# --- verify the token can actually open PRs on the repo ---------------------
echo "verifying repo + PR scope on Noelo-Lab/kuna ..."
if gh repo view Noelo-Lab/kuna --json viewerCanAdminister,nameWithOwner >/dev/null 2>&1; then
  echo "OK: gh can read Noelo-Lab/kuna."
  echo "    (PR creation requires the token's 'repo' scope; if `gh pr create` later"
  echo "     returns 403, regenerate the token with repo scope and re-run gh auth login.)"
else
  echo "WARNING: gh cannot view Noelo-Lab/kuna with this token — PR creation will fail."
  echo "         Provision a token with 'repo' scope and run: gh auth login --with-token"
  exit 3
fi

gh auth status
