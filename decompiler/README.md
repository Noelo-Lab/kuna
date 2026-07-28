# The kuna engine (cargo workspace)

The Rust decompilation engine — the whole product lives in this workspace. It began as a
line-faithful port of Ghidra's C++ decompiler and SLEIGH compiler (the C++ tree has been
removed; see `../docs/rust-port/README.md`) and has since grown its own analysis tier,
CLI, and front-ends.

Design decisions live in `../docs/rust-port/` — start with the ADRs in
`../docs/rust-port/adr/` (IR arenas, ordered containers, integer semantics, error model,
dispatch, stage registry, re-runability). Working conventions: `../docs/agents.md`.

## Layout

| Crate | What |
|---|---|
| `crates/kuna-base` | Foundation: types, errors, XML + marshaling, spaces/addresses, raw p-code, context db, compression, translate/loadimage traits. |
| `crates/kuna-num` | Multiprecision, IEEE float emulation, CircleRange. |
| `crates/kuna-sleigh` | `.sla` reader + instruction decode runtime + the compile-side pattern machinery. |
| `crates/kuna-slacomp` | The SLEIGH compiler (binary `slacomp`, same CLI as upstream `sleigh_opt`). |
| `crates/kuna-decomp` | The decompiler core (phase-foldered `src/`: `p0_knowledge/`…`p9_emit/`, `substrate/`, `infra/`) + the phase model/option surface. |
| `crates/kuna-analysis` | The program-prep loader/analyzer tier (ELF markup, strings, DWARF, function discovery). |
| `crates/kuna-console` | Console front-end; bins `decomp_dbg` and `decomp_test_dbg`; the in-process engine + project-export modules. |
| `crates/kuna-cli` | The user-facing `kuna` binary (`../docs/cli.md`). |
| `crates/kuna-ghidra` | The `kuna_ghidra` binary — kuna as stock Ghidra's decompiler core (`../docs/ghidra-integration.md`). |
| `crates/kuna-wasm` | The `kuna_wasm` binary (wasm32-wasip1) — the in-browser decompiler (`../docs/web-integration.md`). |
| `crates/kuna-harness`, `crates/kuna-lift-diff` | Dev-only test-harness + differential helpers. |

## Build & test

From the repo root, `make binaries` / `make test` / `make rust-test` (see
`../docs/agents.md`). Or directly:

```bash
cd decompiler
cargo build --release -p kuna-console -p kuna-harness -p kuna-slacomp -p kuna-cli
cargo test --workspace
cargo clippy --workspace -- -D warnings   # HashMap/HashSet are deny (clippy.toml)
```

Binaries land in `decompiler/target/release/`.
