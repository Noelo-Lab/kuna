# kuna Rust port

A parallel Rust reimplementation of the kuna C++ decompiler. The C++ tree
(`../decompiler/cpp/`) is the **test oracle** and stays green throughout the
port; the Rust binaries speak the same console-command surface as
`decomp_dbg`/`decomp_test_dbg`, so the XML datatests and the Python harness
(`kuna/run_tests.py`, `kuna/decompile.py`) verify the port unchanged.

Design decisions live in `../docs/rust-port/` — start with the ADRs in
`../docs/rust-port/adr/` (IR arenas, ordered containers, integer semantics,
error model, dispatch, stage registry, re-runability).

## Layout

| Crate | What (C++ -> crate mapping in each crate's `lib.rs`) |
|---|---|
| `crates/kuna-base` | Foundation: types, errors, XML + marshaling, spaces/addresses, raw p-code, context db, compression, translate/loadimage traits. |
| `crates/kuna-num` | Multiprecision, IEEE float emulation, CircleRange. |
| `crates/kuna-sleigh` | `.sla` reader + instruction decode runtime. **No compiler** — `sleigh_opt` stays C++ and keeps producing the `.sla` files. |
| `crates/kuna-decomp` | The decompiler core: DECCORE + the kuna stage model. |
| `crates/kuna-console` | Console front-end; bins `decomp_dbg` and `decomp_test_dbg`. |
| `crates/kuna-harness` | Dev-only differential helpers (Rust engine vs. C++ oracle). |

Dependency edges: `num`/`sleigh` -> `base`; `decomp` -> `base`+`num`+`sleigh`;
`console` -> `decomp`; `harness` -> everything.

## Build

From the repo root (preferred — matches the C++ build driver):

```bash
make rust        # cargo build --release -p kuna-console (the two binaries)
make rust-test   # cargo test --workspace
```

Or directly:

```bash
cd rust
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings   # HashMap/HashSet are deny (clippy.toml)
```

Binaries land in `rust/target/release/decomp_dbg` and
`rust/target/release/decomp_test_dbg` (placeholders for now: they print
"kuna rust engine: not yet implemented" and exit 1).

## Engine switch

The Python tooling selects the engine via `KUNA_ENGINE`:

```bash
KUNA_ENGINE=rust python -m kuna.decompile ./a.out main
KUNA_ENGINE=rust python -m kuna.run_tests --all --baseline docs/baseline.json
```

Default (unset or `KUNA_ENGINE=cpp`) is the C++ oracle under
`decompiler/cpp/`. With `KUNA_ENGINE=rust` the same harness drives
`rust/target/release/decomp_dbg` / `decomp_test_dbg` instead — same commands,
same datatests, same baseline.
