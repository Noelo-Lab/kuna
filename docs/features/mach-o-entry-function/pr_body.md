## What was broken

RE-friction round 2, need `mach-o-entry-function` (major, 1 instance, challenge
`653d88600f4238b24302b0ec` — `crackme0x04`, Mach-O 64-bit x86_64 PIE, stripped):

> functions listed the program routine only as `sub_100003a80`; no function named `main`
> was present, and decompilation rendered it as `unsigned long sub_100003a80(void)`.

`kuna functions` answered with 24 functions, every one of them a `sub_<addr>`. The one an
agent needs first — where the program starts — was indistinguishable from the other
twenty-three, so finding `main` meant reading bodies until one looked like a prompt loop.

The image had been saying which one it was the whole time. `LC_MAIN` is a Mach-O load
command whose `entryoff` field is documented as the file offset of `main()`, `ld64` emits it
for every normally-linked executable, `dyld` calls `__TEXT.vmaddr + entryoff` as
`main(argc, argv, envp, apple)` — and, unlike the symbol table, it survives `strip` intact.
kuna's Mach-O entry oracle already *read* that command: `sub_100003a80` **is**
`0x100000000 + 0x3a80`. Only the name and the prototype were missing.

The filed hypothesis — "the Mach-O `LC_MAIN` loader path does not label/type the entry
routine" — stands.

## Mechanism

New option **`machomain`** (`on|off`, default **on**, DIV-111), a P1 analysis pass in
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_machomain.rs`:

- the **name** `main` rides the existing `entry_names` overlay — the same additive map the
  commit boundary already consults for the ELF `_INIT_<i>` / `_DT_INIT` names — so the
  commit's idempotent cross-scope probe still lets a real symbol win;
- the **prototype** `int main(int argc, char **argv)` is parked by that name.

It is the Mach-O counterpart of the PE `entrymainproto` pass, and typed differently on
purpose. That pass reports the widths a *recovered call site* establishes and refuses to
assert the C library's declaration, because the same shape carries `wmain`'s `wchar_t **`.
Mach-O has no in-image call site to read (the runtime that calls `main` lives in
`libdyld.dylib`) and needs none: `LC_MAIN` **is** the POSIX `main` by definition, so
`int` / `char **` is the honest spelling — and it lets a string literal render through
`argv[i]`. `envp` is deliberately left undeclared.

Five refusals keep the claim honest: anything that is not an `MH_EXECUTE` Mach-O; an
`LC_UNIXTHREAD`-only image (that entry is the crt's `start`, not `main`); an entry outside
every executable section; an entry that already carries a function symbol; and an image that
already defines a symbol spelled `main`.

```
$ kuna functions crackme0x04 --json | grep -A2 '"name": "main"'
      "name": "main",
      "address_hex": "0x100003a80",

$ kuna decompile crackme0x04 main
int main(int argc,char **argv)
{ ...

$ kuna decompile crackme0x04 --addr 0x100003a80 --option machomain off
unsigned long sub_100003a80(void)
{ ...
```

## The acceptance probe that now passes

```
python -m scripts.repipe.verify --need mach-o-entry-function --json
→ counts {"total": 1, "pass": 1, "fail": 0, "closed": 1, "regressed": 0}
   acceptance a-9e76eaa91fcb  PASS
     exit_code eq 0                 → 0
     stdout_matches[0] "name.*main" → name": "main"
     stdout_is_json                 → true
```

Promoted verbatim-in-shape into **`tests/cli/mach-o-entry-function.json`**, re-pointed off
the dataset binary (CI has no dataset; `verify --promote` refuses a `binary_source: dataset`
target) onto a new in-repo fixture — and *tightened*, because the filed clause `name.*main`
is satisfied by `_main` too and would go green on an unstripped image the pass never
touches. The promoted probe asserts the exact `"name": "main"` line, the **absence** of the
pre-fix `sub_1000005b0`, `functions[*].name eq main`, and `count ge 5`.

The fixture is **`macho_stripped_main`** — `macho_imports` with the defined-symbol names
taken away, which is what `strip` leaves behind on a Mach-O executable, so `LC_MAIN` is the
only thing left in it saying `0x1000005b0` is `main`. `macho_stripped_main.py` beside it
regenerates it byte-identically. The pair is now the positive/negative twin for the same
test: `macho_imports` names that address `_main` itself and the pass leaves it alone.

## The evidence, because the four gates cannot supply it

The captain's B_PLAN note is right that "0/675 assertions changed" proves nothing here —
**both parity corpora are symbol-less ELF bytechunks**, so a Mach-O load-command change
cannot move them in either direction. So:

**Collateral sweep — `decompile-all`, on vs off, over all 23 Mach-O images in the RE
dataset** (x86_64, arm64, i386, ppc; thin and fat). Every changed line read.

| | |
|---|---|
| fired | 8 |
| refused | 15 — **fully accounted for**: 12 already carry a `_main` symbol at the `LC_MAIN` address, 3 are `LC_UNIXTHREAD`-only pre-10.8 images |
| unexplained misses / false fires | 0 / 0 |
| exit-code mismatches | 0 |
| total diff lines across all 23 | 46 |
| statements moved, deleted or re-anchored | **0** |

Of the 8 that fire: 6 change only the declaration line; 1 also narrows a local from
`unsigned int` to `int` (the return type propagating); 2 also gain **one spurious argument
at an unprototyped callee** — `___chkstk_darwin(CONCAT44(v7,argc))` and
`sub_100003790(CONCAT44(v11,argc))`. Declaring `argc` makes the first ABI argument register
live at `main`'s own entry, so a callee kuna has no prototype for now finds a value in it.
That is the standing behaviour the PE pass above already documents and measures at 4 of its
30 firing images; the answer is prototype coverage at the callee, not withholding the entry
declaration. It is recorded in `not_closed`.

**Tests.** `decompiler/crates/kuna-console/tests/verify_machomain.rs` — the two-pass gate
(off: `sub_1000005b0(...)` with no argument named; default: `main` with `argc`/`argv`) plus
the guard that the un-stripped twin keeps its own `_main` in **both** arms and gains no
function. Four unit tests over the pure seams (`entry_main_vma`, `macho_main_entry_vma`):
the stripped claim, the named-twin refusal on x86-64 *and* arm64, and inertness on ELF and
PE.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK** — 675/675 assertions |
| `make test-stages` | **PARITY OK** — 610/610 assertions |
| `make rust-test` | green (workspace suite) |
| `make check-spec` | OK, lenient **and** `--strict` |
| `make test-cli` | 27/27 (26 before, +the promoted probe) |
| `kuna catalog --check` | catalog OK |
| `scripts.repipe.counters` | no drift |
| speed | **+0.11%** (min) / +0.46% (median), interleaved min-of-15 pairs — budget 5% |

Speed is measured **interleaved** (15 alternating pairs, order flipped every pair, address
selector on both arms) because `scripts.pipeline.timeit` run sequentially on this box
reported −23.57% on a pass that does one extra walk of a 16-entry load-command list: it
times every `off` repeat and then every `on` repeat, so a warming trend lands entirely on the
second arm. Interleaved: off 178.41 ms / on 178.61 ms (min) — noise, as the mechanism
predicts. Recorded in `record.json`.

Docs: `docs/spec/01-program-prep.md` (the owning chapter — prose beside the
`entrymainproto` section it is the counterpart of), the DIV-111 row in `docs/history.md`,
and a regenerated `docs/options.md`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
