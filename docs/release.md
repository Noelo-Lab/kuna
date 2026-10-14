# Versioning & binary releases

How kuna versions itself and how the binary release CI
(`.github/workflows/release.yml`) turns a day's work on `main` into a tagged
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
- **Released minor numbers skip, by design.** The minor is gap-free *per commit*,
  but releases are nightly and batch a whole day, so a night that merged eight
  PRs jumps the published version by eight. `v1.89` following `v1.81` is the
  system working, not a failed run.
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

### When it runs

**One release a night, batching the day's merges** — not one per push. Three
triggers:

| Trigger | When | Why |
|---|---|---|
| `schedule` | 06:00 UTC daily (~02:00 America/New_York) | The normal path. Whatever landed that day ships together. |
| `push` to `main`, `paths: [VERSION]` | A MAJOR bump lands | A major is an intentional milestone; it should not wait for the night. |
| `workflow_dispatch` | On demand, Actions tab | Cut a release now, or re-run a failed one. |

There is deliberately **no "has anything changed?" guard**, because the version
scheme already is one: the minor *is* the commit count, so a nightly run on an
unchanged `main` recomputes the same `MAJOR.MINOR`, the `version` job finds that
tag already published, and every downstream job skips. A quiet day costs one
~30-second job and publishes nothing.

Before this, every push to `main` released: one busy day produced **18 releases
and 18 tags**.

### The jobs

1. **version** — computes `MAJOR.MINOR` from a full-history checkout and skips
   the whole run if tag `vMAJOR.MINOR` already exists (safe re-runs, and the
   no-op guard above).
2. **specs** — compiles every `.slaspec` → `.sla` once on Linux and packages
   the whole `specs/` tree as `kuna-v<ver>-specs.tar.gz`. `.sla` files are
   platform-independent, so one tree serves every OS: each **build** row
   embeds it, and it also ships as an asset (for `KUNA_SPECS`, and the
   nightly's speed baseline).
3. **build** (matrix, after **specs**) — lays the compiled tree over the
   checkout's `specs/`, then `cargo build --release` of `kuna-cli` (with
   `--features kuna-cli/embed-specs`) and `kuna-ghidra` for:
   - Linux `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` (native,
     on the `ubuntu-24.04-arm` runner)
   - Linux `x86_64-unknown-linux-musl` — a fully static build (no glibc
     dependency, runs on any Linux). kuna pulls in no C via a `cc`-crate
     dependency, so the musl target links self-contained; `rustup target add`
     is the only extra setup.
   - macOS `aarch64-apple-darwin` and `x86_64-apple-darwin` (the latter
     cross-compiled on the arm64 runner)
   - Windows `x86_64-pc-windows-msvc`
   Each archive (`kuna-v<ver>-<os-arch>.tar.gz`, `.zip` on Windows) contains
   `kuna`, `LICENSE`, `NOTICE` — `kuna` is self-contained: the decompiler
   console (`decomp_dbg`) and the SLEIGH compiler (`slacomp`) are built in
   as hidden subcommands, and `embed-specs` carries the compiled SLEIGH tree.
   Two smoke tests assert `kuna --version` prints the release version and
   that the binary, copied alone into an empty directory with no spec
   override, decompiles a fixture. (`decomp_test_dbg` is not shipped — the
   datatest harness only makes sense inside the repo.) The `kuna_ghidra`
   binaries are not in the archives: each row uploads its binary as a
   `ghidra-bin-<platform>` workflow artifact for the **ghidra-ext** job.
4. **ghidra-ext** — packages the ready-to-install KunaDecompiler Ghidra
   extension, `kuna-v<ver>-KunaDecompiler-ghidra_<ghidra-version>.zip`. It
   downloads the pinned Ghidra release (version *and* full asset URL are
   pinned in the workflow's `env`, since the asset filename embeds a build
   date; the zip is cached with `actions/cache` under a key derived from the
   asset filename, so bumping the URL always invalidates it), stages the five
   `ghidra-bin-*` binaries into the extension's `os/<platform>/` dirs
   (restoring the exec bits that artifact upload drops), and runs Ghidra's own
   `support/buildExtension.gradle` via the runner's preinstalled Gradle under
   Java 21. The job asserts with `zipinfo` that all five `os/` binaries are in
   the zip and the four unix ones carry mode 0755 — Ghidra's extension
   installer re-applies exactly those bits at install time. (Gradle 9
   normalizes archive entry modes for reproducibility, which would strip the
   exec bits; the extension's `build.gradle` pins 0755 on the `kuna_ghidra`
   entries explicitly, so the zip is correct under both Gradle 8 and 9.)
   The zip is fully self-contained (`kuna_ghidra` needs no `.sla` files —
   specs arrive over the wire), so it installs airgapped; it is version-locked
   to the pinned Ghidra release, other 12.x installs go through the
   installer's "Install Anyway" dialog. See
   `integrations/ghidra/KunaDecompiler/README.md` for the install steps.
5. **release** — `gh release create v<ver>` with all assets, tagging the built
   commit. This is what creates the git tag; no tags are pushed from anywhere
   else. (It downloads only `kuna-*` artifacts — the `ghidra-bin-*` binaries
   are inter-job plumbing, not assets.)

## Using a released binary

`kuna` is the only file a release needs. It runs its own decompiler console
and SLEIGH compiler, and carries the compiled SLEIGH tree, which it unpacks
once — on first use, under a name derived from the tree's contents — into the
per-user cache directory (`%LOCALAPPDATA%\kuna` on Windows,
`~/Library/Caches/kuna` on macOS, `$XDG_CACHE_HOME/kuna` or `~/.cache/kuna`
elsewhere, and the system temp directory when none of those is writable):

```bash
tar xzf kuna-v<ver>-linux-x86_64.tar.gz
./kuna-v<ver>-linux-x86_64/kuna decompile ./a.out main
```

`KUNA_SPECS` still overrides the built-in tree (for example with the
`kuna-v<ver>-specs.tar.gz` asset, or a tree of your own); the full list of
overrides is in `docs/cli.md`. A release whose specs did not change reuses
the tree already unpacked. Old trees are never removed automatically; deleting
the cache directory is always safe, and the next run unpacks again.

(Inside a repo checkout none of this applies — a plain `cargo build` carries no
specs, and they are found from the repo root. `make specs` then
`cargo build --release -p kuna-cli --features embed-specs` builds the
self-contained binary locally.)

The `kuna-v<ver>-KunaDecompiler-ghidra_<ghidra-version>.zip` asset is not an
archive to extract — install it from inside Ghidra (**File → Install
Extensions… → `+`**); see `integrations/ghidra/KunaDecompiler/README.md`.
