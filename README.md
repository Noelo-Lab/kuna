# Kuna

An agent-first decompiler, based on [Ghidra](https://github.com/NationalSecurityAgency/ghidra), written in Rust.
Kuna aims to improve fundamental decompiler techniques, while making them more accessible to agents and thier human users.
Interested in why? Read our [motivating statement](#motivation-why-another-decompiler) and our [design principles](#design). 

<p align="center">
   <img src="./assets/kuna.png" style="width: 30%;" alt="Kuna Logo"/>
</p>

## Install & Usage
### Install
Use the release. 

### Usage
The user-facing command is the single Rust binary **`kuna`** (`decompiler/target/release/kuna`):

```bash
KUNA=decompiler/target/release/kuna

# Decompile a function by name, or by address in a stripped binary
$KUNA decompile ./a.out main
$KUNA decompile ./stripped.bin 0x401040 --addr
```

Example output for a tiny `int add(int a,int b){return a+b;} int main(){return add(2,3);}`:

```c
void main(void)

{
  add(2,3);
  return;
}
```

kuna reads ELF/PE/etc. via the `object` crate and selects the SLEIGH language automatically.
`decompile` drives `decomp_dbg` and captures the emitted C cleanly (interactive prompts never
pollute the output).

## Motivation: Why Another Decompiler?

LLM agents, like [Codex](https://chatgpt.com/codex/), have fundamentally changed how we reverse engineer and ultimately secure binaries.
Instead of humans looking directly at decompilers, LLMs are increasingly the ones using them, and humans instead read the agents' logs. As such, it feels natural that decompilers should become oriented and optimized around agents.

This paradigm shift requires more than just an [MCP](https://modelcontextprotocol.io/docs/getting-started/intro) or a [generic CLI](https://github.com/binsync/declib). It requires a composable, tunable decompiler design that can be modified on-the-fly for the situation. This hypothetical decompiler must also be fast, to reduce the agent bottleneck, and positioned to be developed rapidly.

Kuna prioritizes the following:

* Quality decompilation: text is what LLMs consume, so make it the best on the planet to get better performance from LLMs.
* Speed: remove the bottleneck common on larger binaries.
* Flexibility: give LLMs access to many knobs that change decompilation based on the goal.
* Quick development: LLMs will do the majority of coding, so target ways to improve that experience.

To achieve this, Kuna is built as an agent-first decompiler. 
The decompiler is broken into stages that can be adjusted during runtime using assertions, allowing decompilation behavior to be tuned for the task at hand. 
It is based on Ghidra, due to its speed and architectural support, but rewritten in Rust to enable safer and faster LLM-assisted development.

Kuna is also being actively improved with state-of-the-art techniques from decompilers like [angr](https://github.com/angr/angr), differential testing against Hex-Rays, and new workflows designed specifically for agent-driven reverse engineering.

There is still a lot of work to do. If you are interested in decompilers, reverse engineering, program analysis, or building tools for the next generation of LLM agents, we'd love your help. 
Join us as we push decompilers into their next stage of evolution.


## Design
kuna is organized around an explicit **stage model** (a P0 knowledge plane + S1–S9 with a
Band-B fixed point and typed feedback edges), not a linear pipeline:

- `docs/phases.md` — the stage model at a glance; `docs/history/stage-model.md` — the full normative
  model (sub-stage catalogs, feedback edges, code anchors).
- `docs/history/stage-mapping.md` — every source file mapped to a stage, anchored to the real pass
  pipeline (`universalAction`) and the runtime registry (queryable via the `stage list/map/
  catalog` console commands).
- `docs/options.md` — the tiered option catalog (transforms = the LLM control surface,
  with a generated symptom index); `docs/divergences.md` records
  kuna's intentional default changes.

### The stage-model control surface

kuna exposes Ghidra's pipeline as an explicit **stage model** with per-run, flippable sub-stage
assertions (the "LLM control API"):

```bash
$KUNA catalog --json                                   # discover the settable assertions
$KUNA decompile ./a.out main --option compareform canonical
$KUNA decompile ./sparc.elf main --option returnpair single
```

`catalog --markdown` regenerates `docs/options.md`; `catalog --check` fails on
catalog/registration drift (CI).


## Development
### Building from Source
You only need a **Rust toolchain** (`cargo`).

```bash
make binaries   # cargo-build the decompiler (decomp_dbg, decomp_test_dbg),
                # the SLEIGH compiler (slacomp), and the `kuna` CLI
make specs      # compile every SLEIGH .slaspec -> .sla with slacomp (the decoder needs these)
make            # = binaries + specs
```

Everything lands in `decompiler/target/release/`. For development, work in the cargo workspace
directly: `cd decompiler && cargo build` / `cargo test --workspace`.

### Test
```bash
make test        # the 675/675 decompiler regression parity (datatests + docs/baseline.json)
make rust-test   # the full cargo workspace suite (ported unit tests, golden differential
                 # vectors, SLEIGH-compiler .sla content-parity, ...)
```

`make test` compiles the specs with the Rust SLEIGH compiler and decodes the XML regression
corpus (`tests/datatests/`, 83 files / 675 assertions) with the Rust decompiler, end to
end — the self-sufficient correctness gate. `docs/baseline.json` is the recorded oracle.

### Layout

| Path | What it is |
|---|---|
| `decompiler/` | The engine — a cargo workspace. `kuna-base`/`kuna-num`/`kuna-sleigh`/`kuna-decomp` (decompiler), `kuna-console` (the `decomp_dbg`/`decomp_test_dbg` binaries), `kuna-slacomp` (SLEIGH compiler, `slacomp`), `kuna-cli` (the `kuna` binary) |
| `tests/datatests/` | Upstream XML decompilation regression tests (83 files → 675 assertions); the corpus `make test` runs |
| `specs/Ghidra/Processors/` | Vendored SLEIGH processor specs; `.sla` are build artifacts produced by `slacomp` |
| `Makefile` | Top-level build/test driver (Rust-only) |
| `docs/rust-port/README.md` | The port summary (what/why/how/validation) |
| `scripts/` | Python helpers backing the autonomous feature `pipeline/` (the user-facing CLI is the Rust `kuna` binary) |
| `tools/sync_upstream.py` | Pulls upstream Ghidra `specs/` + `tests/datatests/` updates |

## Provenance

Ported from Ghidra commit `cef869af04c4740a71ad31a55704045b1b0d1644`. The SLEIGH specs and the
XML regression corpus are still vendored from upstream; the C++ source that the engine was
ported from is recorded at that commit and recoverable from git history (the tree was removed
once the port was proven — see `docs/UPSTREAM.md` and `docs/rust-port/README.md`).
