# Dev container — reproducible build + cross-arch fixture environment

`.devcontainer/` provides a fully reproducible environment for (a) building and testing
kuna and (b) building cross-architecture ELF test fixtures (x86-64, ARM32/Thumb, AArch64,
RISC-V64, MIPS/MIPSel/MIPS64el, PPC64le, SPARC64) **including their linkers**, plus Go and
a pinned Rust toolchain.

The single most important thing this container fixes: the original build host had **no ARM
linker**, which blocked the documented "ARM decode end-to-end test" (see
`docs/analysis-port-log.md`, Increment 17). The container ships every cross toolchain *with*
its `binutils` (assembler + `ld`), so a **linked** ARM Thumb ELF — not just a bare `.o` — can
now be produced in-env.

## Files

| File | Role |
|---|---|
| `.devcontainer/Dockerfile` | The image. **Env-only** — it does NOT copy the repo; the workspace is mounted at runtime so one image serves every branch/worktree. |
| `.devcontainer/devcontainer.json` | VS Code / `devcontainer` CLI config. References the Dockerfile, runs as user `dev`, mounts the repo at `/workspaces/kuna`. |

## Design notes

- **Base `ubuntu:22.04`** — matches the host toolchain era so the vendored cross-arch ELF
  fixtures stay byte-reproducible.
- **Env-only image** — the Dockerfile never `COPY`s the repo. Mount the worktree at
  `/workspaces/kuna` at runtime. The build context is just `.devcontainer/` (tiny).
- **Non-root `dev` user (uid 1000)** with **passwordless sudo** (`/etc/sudoers.d/dev`).
- **Privileged by default** — `devcontainer.json` sets `privileged: true` and
  `runArgs: ["--privileged"]`. This (and passwordless sudo) is **intentional per project
  policy**: an agent must always be able to run privileged and `apt-get install` anything
  without rebuilding the image.
- **Rust pinned to 1.90.0** (the host version; matches `decompiler/Cargo.toml`
  `rust-version = "1.90"`), installed via rustup **as the `dev` user** with `CARGO_HOME` /
  `RUSTUP_HOME` under `/home/dev`. The image build fails if `rustc` is not 1.90.0 or if
  `arm-linux-gnueabihf-gcc` is missing (a baked-in sanity gate).

## Build the image

From the repo root:

```bash
docker build -t kuna-dev -f .devcontainer/Dockerfile .devcontainer
```

(Build context is `.devcontainer` on purpose — the image is env-only, so it needs none of the
repo.) Or open the folder in VS Code with the Dev Containers extension / use the `devcontainer`
CLI, which reads `.devcontainer/devcontainer.json` directly.

## Run the gates inside the container

Mount the repo worktree and run the make targets. From the repo root:

```bash
# build all binaries (decomp_dbg, decomp_test_dbg, slacomp, kuna)
docker run --rm -v "$(pwd)":/workspaces/kuna -w /workspaces/kuna kuna-dev \
    bash -lc 'make binaries'

# the three required gates
docker run --rm -v "$(pwd)":/workspaces/kuna -w /workspaces/kuna kuna-dev \
    bash -lc 'make test'         # 675/675 datatest parity  -> PARITY OK
docker run --rm -v "$(pwd)":/workspaces/kuna -w /workspaces/kuna kuna-dev \
    bash -lc 'make test-stages'  # stage-model corpus       -> PARITY OK
docker run --rm -v "$(pwd)":/workspaces/kuna -w /workspaces/kuna kuna-dev \
    bash -lc 'make rust-test'    # full cargo workspace suite
```

`decompiler/target/` is gitignored, so building inside the mounted worktree leaves the host
tree clean of artifacts (the `target/` produced is the host's own and is never committed).

For an interactive session:

```bash
docker run --rm -it -v "$(pwd)":/workspaces/kuna -w /workspaces/kuna kuna-dev bash -l
```

## Build a cross-arch fixture (the ARM-linker unblock)

The headline capability — produce a **linked** ARM Thumb executable (the previously-blocked
fixture), then confirm it has a `LOAD`/exec segment:

```bash
printf 'int f(int x){return x+1;} void _start(){f(41);}' > /tmp/thumb.c
arm-linux-gnueabihf-gcc -mthumb -static -nostdlib -e _start /tmp/thumb.c -o /tmp/arm_thumb_linked
readelf -h /tmp/arm_thumb_linked   # Type EXEC, Machine ARM
readelf -l /tmp/arm_thumb_linked   # a LOAD segment (R E)
```

Verified output: `Type: EXEC`, `Machine: ARM`, entry `0x100d1` (odd = Thumb bit set), and a
`LOAD` segment with `R E` flags. Before this container, `which arm-linux-gnueabihf-gcc` on the
host was empty.

The same pattern builds fixtures for the other arches by swapping the triple, e.g.:

```bash
aarch64-linux-gnu-gcc       -static -nostdlib -e _start /tmp/x.c -o /tmp/aarch64_linked
riscv64-linux-gnu-gcc       -static -nostdlib -e _start /tmp/x.c -o /tmp/riscv64_linked
mipsel-linux-gnu-gcc        -static -nostdlib -e _start /tmp/x.c -o /tmp/mipsel_linked
powerpc64le-linux-gnu-gcc   -static -nostdlib -e _start /tmp/x.c -o /tmp/ppc64le_linked
sparc64-linux-gnu-gcc       -static -nostdlib -e _start /tmp/x.c -o /tmp/sparc64_linked
```

## Toolchain inventory

| Category | Packages / commands |
|---|---|
| Core build | `build-essential`, `make`, `cmake`, `pkg-config`, `gcc`, `g++`, `clang` 14, `lld` (`ld.lld`) |
| Rust | rustup, toolchain **1.90.0** (default), `rustfmt`, `clippy` — `cargo`/`rustc` on the `dev` PATH |
| ARM32/Thumb | `gcc-arm-linux-gnueabihf`, `g++-arm-linux-gnueabihf` → `arm-linux-gnueabihf-{gcc,g++,ld,as}` |
| AArch64 | `gcc-aarch64-linux-gnu`, `g++-aarch64-linux-gnu` → `aarch64-linux-gnu-{gcc,g++,ld}` |
| RISC-V64 | `gcc-riscv64-linux-gnu` → `riscv64-linux-gnu-{gcc,ld}` |
| MIPS | `gcc-mips-linux-gnu`, `gcc-mipsel-linux-gnu`, `gcc-mips64el-linux-gnuabi64` → `mips[-/el/64el]-…-{gcc,ld}` |
| PPC64le | `gcc-powerpc64le-linux-gnu` → `powerpc64le-linux-gnu-{gcc,ld}` |
| SPARC64 | `gcc-sparc64-linux-gnu` → `sparc64-linux-gnu-{gcc,ld}` |
| Binary inspection | `binutils-multiarch` (multiarch `readelf`/`objdump`), `file` |
| Languages / tools | `golang-go` (Go 1.18), `python3` + `python3-pip`, `git`, `curl`, `wget`, `ca-certificates`, `gnupg`, `unzip`, `xz-utils`, `gdb`, `sudo`, `less`, `vim` |

Every `gcc-<triple>` package depends on `binutils-<triple>`, so the cross **linkers** come in
for free — that is what unblocks producing *linked* cross-arch ELFs, not just objects.

### Verified cross-compiler versions (ubuntu:22.04)

- ARM / AArch64 / RISC-V64 / PPC64le / SPARC64 GCC: **11.4.0**
- MIPS / MIPSel / MIPS64el GCC: **10.3.0** (the 22.04 archive ships these MIPS cross GCCs at
  the 10.x series; this is the stock package, no substitution).

## Out of scope (add later if scope expands)

Non-ELF toolchains are **not** installed, because non-ELF formats are out of the current
analysis-port scope:

- **mingw-w64** (PE / Windows) — `sudo apt-get install -y gcc-mingw-w64 g++-mingw-w64`
- **osxcross** (Mach-O / macOS) — not packaged in apt; build from source.

Because the container has passwordless sudo and is privileged, an agent can install these (or
anything else) at runtime when PE/Mach-O loader work begins, without changing the image.

## Package-name notes / substitutions

No substitutions were required — every requested `gcc-<triple>` package exists in the
`ubuntu:22.04` archive under the exact name. The only thing worth recording is the MIPS cross
GCCs being 10.3.0 vs 11.4.0 for the others (stock packaging, above). If a future arch's
package name differs, find it with `apt-cache search gcc-<arch>` and note the substitution
here.
