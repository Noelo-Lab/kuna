# Writing and changing SLEIGH processor specs

kuna decodes every instruction through the SLEIGH processor modules under
`specs/Ghidra/Processors/`. Most were vendored from Ghidra. kuna also fixes them
(x86, SPARC, AArch64 rounding modes), extends them (the Octeon MIPS variants) and adds
modules Ghidra does not have (Cortus APS3). This page covers both kinds of work and
the places that have to change with them. Concrete examples are cited by commit.

For the SLEIGH language itself (tokens, constructors, context, p-code semantics), read
Ghidra's manual: <https://ghidra.re/ghidra_docs/languages/html/sleigh.html>. `slacomp`
compiles the same language, so it applies unchanged. This page covers only what is
specific to kuna.

## The files of a module

A module lives in `specs/Ghidra/Processors/<P>/data/languages/`:

| file | holds | read at runtime? |
|---|---|---|
| `.slaspec` (+ the `.sinc` files it `@include`s) | registers, tokens, instruction decoding and p-code semantics | no; compiled to `.sla` |
| `.sla` | the compiled form; a build artifact, gitignored | yes |
| `.ldefs` | one `<language>` per variant: its id, `.sla`, `.pspec` and the `.cspec` of each compiler | yes |
| `.pspec` | the program counter, context register defaults, volatile (MMIO) ranges | yes |
| `.cspec` | data sizes, stack pointer, return address, calling conventions, callee-saved registers | yes |
| `.dwarf` (optional) | DWARF register numbering and the CFA offset, which Ghidra reads | no; kuna's DWARF analyzer keeps the one value it uses (`call_frame_cfa`) in code |

A language id is `processor:endian:size:variant`, and a target adds the compiler id:
`Cortus:LE:32:APS3:default`. kuna finds modules by scanning every `.ldefs` under its
spec roots, so there is no registration table; the web front-end bundles the same
runtime files the same way (`docs/web-integration.md` §3).

## Compiling

```sh
kuna specs specs/Ghidra/Processors/Cortus/data/languages/aps3.slaspec   # one spec
make specs                                                               # all of them
```

`kuna specs` is `slacomp`, which has upstream `sleigh_opt`'s command line. It writes
the `.sla` next to the `.slaspec`, and the write follows a symbolic link.

In a worktree, do not run `make specs`; symlink the main tree's `.sla` files instead
(`docs/agents.md`, Conventions). That sharing is only for specs the worktree does not
change. Before compiling a spec the worktree changes, or one whose `.sinc` closure it
changes, delete that `.sla` link (`rm`, which removes the link and not its target).
Otherwise the compile overwrites the main tree's file, and every other checkout that
links to it silently decodes with the changed spec.

To look at what a constructor produces, `kuna raw-pcode <binary> <addr> --count N`
prints each instruction's bytes, disassembly and p-code before any analysis
(`docs/cli.md`). `raw-pcode` needs an object file. A headerless firmware image loads
only into `decompile`, `decompile-all`, `decompile-project` and `functions`, with
`--raw-image --target <id> --base <vma>`; `specs/Ghidra/Processors/Cortus/README.md`
shows a full example.

## Adding a module Ghidra does not have

Cortus APS3 is the reference: four files and a README (`d3617d648`), and one stage
test (#730).

1. **Write the module and a README next to `data/`.** Say where the instruction set
   came from. Cortus's was recovered from firmware because no opcode table is public,
   so each guessed constructor carries a `# GUESS` comment with its evidence, and the
   README states what is proven (lengths and operand formats for all 256 opcodes) and
   what is not (some mnemonics).
2. **Choose between `unimpl` and an opaque operation by what is known about the
   instruction.** kuna turns `unimpl` into an artificial halt: flow stops there and the
   function is marked truncated. An opaque operation (`define pcodeop
   aps3_unknown_op;`, its result assigned to the destination register) keeps the
   function whole. On Cortus this took the functions with an unimplemented-instruction
   warning from 24 to 0. But an opaque operation is a fallback, not a sound model: it
   claims the instruction has no effect beyond the outputs the constructor writes.
   - Use it when the length, the operands and the set of effects are known, and only
     the computed value is not: a register or flag the instruction writes gets an
     opaque value too.
   - Keep `unimpl` when the instruction may change control flow, access memory, or
     write state the constructor does not name. A fall-through or a missing store there
     produces a wrong CFG or wrong values with no warning, which is worse than a
     truncation that says so.
3. **Measure the `.cspec`, and record the measurement in its comment.** `aps3.cspec`
   gives, per register, the share of call sites that set it and of prologues that save
   it, which is where its argument registers and its callee-saved split come from.
4. **Mark MMIO windows volatile in the `.pspec`**, so repeated register reads do not
   fold into one.
5. **Make it loadable.** If an object format can name this processor, map it in
   `compose_language_id` (`decompiler/crates/kuna-analysis/src/loadimage_object.rs`).
   Otherwise it is reached with `--raw-image --target`.
6. **Bump the spec count.** `decompiler/crates/kuna-sleigh/tests/verify_context_commits.rs`
   asserts the number of compiled `.sla` files; every new `.slaspec` changes it.
7. **Add a stage test** with `<binaryimage arch="<id>:<compiler>">` and the
   instruction bytes in `<bytechunk>`s (`tests/stages/cortus-aps3-constructors.xml`),
   then bump the corpus count and re-record `docs/baseline-stages.json`
   (`docs/agents.md`, Conventions).

Adding the new spec to the compiler oracle (below) is optional; the Octeon variants did,
Cortus did not.

## Adapting a third-party module

Community SLEIGH modules exist for many processors Ghidra lacks (the backlog in #1789
lists them with their licenses). Starting from one is fine when its license allows it:

- **Apache-2.0, MIT, BSD or ISC**: adapt it. Keep its copyright headers, and add a
  paragraph to `NOTICE` that names the project, its URL, its license and the directory
  it went into, reproducing the license text when the license requires it (the deark
  paragraph is the model).
- **No license, or one that is unclear** (WTFPL, "all rights reserved", a missing
  `LICENSE` file): do not copy from it. Read it for its approach, and write the module
  from the processor manual.

Record in the module's README the upstream repository and the commit it was taken from,
and what kuna changed. The checks in the next section apply in full: a community
module's quality varies, and having been published is not evidence that its p-code is
right.

## Checking a new module

`slacomp` accepting a spec says nothing about its semantics. A PR that adds or largely
rewrites a module states which oracle it was checked against and what was measured.
From strongest to weakest:

| oracle | what it checks | the PR reports |
|---|---|---|
| execution: QEMU, Unicorn, or a binutils-gdb `sim/` simulator | registers, flags and memory after each instruction | instruction forms whose state matches from random inputs, and why each of the rest differs (the s390x proposal #1703 compared 1,047 forms against Unicorn) |
| a reference CPU core in MAME | the same, through MAME's debugger | the same |
| a reference disassembler (`objdump`, `llvm-objdump`) | decode and operand syntax only | mnemonic agreement over real code or assembler-generated vectors (the proposals #1700 and #1703) |
| the ISA manual alone | nothing automatic | hand-checked instruction vectors |

A disassembler oracle does not check flags, carry and borrow, delay slots or bank
switching, so those still need vectors from the manual. Vectors become a stage test:
instruction bytes in `<bytechunk>`s, one function per behavior, with the expected
p-code effect in the comment, as `tests/stages/cortus-aps3-constructors.xml` does.

## Changing a vendored spec

A constructor that produces wrong p-code is a strict correctness fix. It ships with no
option, and its stage test pins the corrected output.

**The compiler oracle.** `decompiler/crates/kuna-slacomp/tests/golden/compiler.sha256`
pins the SHA-256 of each listed spec's decompressed `.sla` element stream, as produced
by Ghidra's C++ `sleigh_opt` at the pinned `GHIDRA_REV`. The stream records source line
numbers, so any edit to a listed spec, or to a `.sinc` it includes, changes its digest,
even when the semantics of the other constructors do not change. To update an entry:

1. Show that the compiler you use reproduces the current digest on the unmodified
   sources.
2. Regenerate the digest on the changed sources with the same compiler.
3. Say in `decompiler/crates/kuna-slacomp/tests/golden/README.md` which entries changed,
   why, and with which compiler.

```sh
cargo test --manifest-path decompiler/Cargo.toml --release -p kuna-slacomp --test compiler_parity
```

An edit can leave the other variants' digests unchanged only if their preprocessed
source is unchanged, line numbers included. The `x86-16` hook in `ia.sinc` met both
conditions: it is under `@ifdef IA16` and replaced three blank lines, so the `x86` and
`x86-64` digests did not move. Keeping the line count is a technique for that, not a
guarantee. Run `compiler_parity` and look at every entry that includes the changed file.

**Shared `.sinc` files reach every language that includes them.** A change meant for
one variant goes behind a guard: `x86-16.slaspec` defines `IA16`, and the Octeon
constructors sit behind conditional guards in the shared MIPS `.sinc` files
(`e27d88545`).

**Upstream syncs.** `tools/sync_upstream.py` maps Ghidra's
`Ghidra/Processors/<P>/data/languages/` onto these directories and falls back to a
three-way merge when a file was edited locally (`docs/history.md`, Upstream provenance
& sync). Small, local edits are what keeps that merge clean.

**Describe the change** in `docs/spec/02-lift-and-flow.md`, as its Octeon and x86-16
passages do.

## Before committing

1. `kuna specs` on the changed spec compiles with no new errors.
2. The `compiler_parity` test above passes, with any digest change explained.
3. The four gates in `docs/agents.md` pass.
