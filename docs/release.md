# Versioning & binary releases

How kuna versions itself and how the binary release CI
(`.github/workflows/release.yml`) turns every commit on `main` into a tagged
GitHub Release.

## The version scheme: MAJOR.MINOR

kuna versions are **two components only** — `MAJOR.MINOR` (e.g. `0.14`,
`1.3`). There is no patch component.

| Component | Where it lives | Who/what changes it |
|---|---|---|
| **MAJOR** | The top-level `VERSION` file (a single integer). | A maintainer, by editing the file in a PR. `.github/CODEOWNERS` routes any `VERSION` change to maintainer review (enforced once branch protection on `main` requires code-owner review). |
| **MINOR** | Not stored — **derived**: the number of commits on `main` since `VERSION` last changed. | Every commit, automatically. No bot commits, no push-backs: the count *is* the bump. |

Consequences worth knowing:

- A commit that edits `VERSION` is itself version `MAJOR.0` — bumping the major
  resets the minor.
- The minor number can skip values as seen in releases only if a release run is
  ever skipped/failed; the numbering itself is gap-free per commit.
- Merge strategy matters: a squash-merge bumps the minor by 1 per PR; a merge
  or rebase of N commits bumps it by N.

Compute it locally:

```bash
make version          # or: bash scripts/version.sh
```

The script needs full git history (the minor is a commit count) and errors out
on shallow clones.

## What `kuna --version` reports

Release CI exports `KUNA_VERSION=<MAJOR.MINOR>` while building; `kuna-cli`
bakes it in via `option_env!` and `kuna --version` prints exactly that (Cargo's
own package version must stay three-part semver, so it is not used for
releases). Dev builds without the env var fall back to the workspace Cargo
version.

## The release workflow

`.github/workflows/release.yml`, on every push to `main` (plus manual
`workflow_dispatch`):

1. **version** — computes `MAJOR.MINOR` from a full-history checkout and skips
   the whole run if tag `vMAJOR.MINOR` already exists (safe re-runs).
2. **build** (matrix) — `cargo build --release` of `kuna-cli`, `kuna-console`,
   and `kuna-slacomp` for:
   - Linux `x86_64-unknown-linux-gnu`
   - macOS `aarch64-apple-darwin` and `x86_64-apple-darwin` (the latter
     cross-compiled on the arm64 runner)
   - Windows `x86_64-pc-windows-msvc`
   Each archive (`kuna-v<ver>-<os-arch>.tar.gz`, `.zip` on Windows) contains
   `kuna`, `decomp_dbg`, `slacomp`, `LICENSE`, `NOTICE`. A smoke test asserts
   `kuna --version` prints the release version. (`decomp_test_dbg` is not
   shipped — the datatest harness only makes sense inside the repo.)
3. **specs** — compiles every `.slaspec` → `.sla` once on Linux and packages
   the whole `specs/` tree as `kuna-v<ver>-specs.tar.gz`. `.sla` files are
   platform-independent, so one asset serves every OS.
4. **release** — `gh release create v<ver>` with all assets, tagging the built
   commit. This is what creates the git tag; no tags are pushed from anywhere
   else.

## Using a released binary

The binaries resolve each other as siblings of the `kuna` executable, so
extracting one archive keeps `kuna decompile` (which spawns `decomp_dbg`) and
`kuna specs` (which spawns `slacomp`) working. The engine additionally needs
the compiled SLEIGH tree at runtime:

```bash
tar xzf kuna-v<ver>-linux-x86_64.tar.gz
tar xzf kuna-v<ver>-specs.tar.gz
export KUNA_SPECS=$PWD/specs
./kuna-v<ver>-linux-x86_64/kuna decompile ./a.out main
```

(Inside a repo checkout none of this applies — binaries live in
`decompiler/target/release/` and specs are found from the repo root.)
