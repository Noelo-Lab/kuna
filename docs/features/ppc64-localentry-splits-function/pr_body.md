## What was broken

RE-need `ppc64-localentry-splits-function` (round 2, severity **major**, credibility 0.9,
2 recorded instances):

> Decompile or list any function in a PPC64 ELFv2 image and you get two functions where
> there is one: the named symbol truncated to its 8-byte global-entry prologue, and the
> whole real body filed under an anonymous `sub_<hex>` 8 bytes later.

On this repo's own `plt_ppc64le` fixture, at merged main:

```
$ kuna decompile .../fixtures/plt_ppc64le __do_global_ctors_aux
void __do_global_ctors_aux(void)
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

$ kuna functions .../fixtures/plt_ppc64le --json
{"name": "__do_global_ctors_aux", "address_hex": "0x940", "size": 8}
{"name": "sub_948",               "address_hex": "0x948", "size": 112}
```

and the same 8-byte-husk / anonymous-body pairing for `deregister_tm_clones`,
`__do_global_dtors_aux` and `frame_dummy` — every routine in the image with a distinct
local entry. Live on default `kuna decompile` / `kuna functions` / `kuna decompile-all`,
no option or mode needed. The filing builder measured 8 such truncations over 33
non-x86-64 fixtures.

## Root cause — the filed hypothesis, confirmed, with its open half closed

The need's hypothesis had two halves and left the first unverified ("not verified by the
captain: whether the +8 entries come from call targets, from a prologue matcher, or from
the ELF reader"). Measured here: **call targets**.

```
$ kuna xrefs plt_ppc64le --to 0x948
0x5f4   call    _init+0x34      bl 0x948
```

Ablation pins it exactly: `--option funcstart_patterns off`, `--option aif off` and
`--option listing off` each leave the four entries in place; `listing off` **plus**
`fast_funcdisc off` removes them. Both of those consumers read `listing.functions()`, so
the entries are minted by the recursive-descent walk's function worklist and nothing else.
(The need warned that a `--option ... off` ablation on this path may not take effect —
that turned out to be because `kuna functions` accepts no `--option` at all, so the
ablations must be driven through `decompile-all`.)

The reason the walk mints them is the ELFv2 ABI. A PPC64 function has a *global* entry —
the symbol's `st_value`, whose first two instructions materialise the TOC pointer `r2`
from `r12` — and a *local* entry a few bytes later, where a caller that already holds the
right `r2` (anything in the same module) branches instead. The distance is recorded per
symbol in `st_other`; `readelf -sW` prints it as `[<localentry>: 8]`. Nothing in kuna read
that field, so an intra-module `bl` landing eight bytes past a function symbol looked like
any other CALL target. Then P2's `funcboundflow` did exactly what it says on the tin: the
"next function entry" its fall-through reached was eight bytes into the function being
decompiled.

Note `register_tm_clones` is *not* split even though it carries the same annotation — the
only reference to its local entry is `frame_dummy`'s tail `b`, not a `bl`, and the walk
mints a function only at CALL targets. That asymmetry is what confirms the call-target
diagnosis over the prologue-matcher one.

## The mechanism

New option **`ppclocalentry`** (`on|off`, default **on**, analysis tier, P1
code/data-partition, DIV-107), implemented in
`decompiler/crates/kuna-analysis/src/listing/kuna_ppclocalentry.rs` and consulted at the
walk's CALL-target seam — the same seam, one line above, that already carries
`unmappedentry`. An address that a defined `STT_FUNC` symbol declares to be its own local
entry is never claimed as a function, because by the ABI's construction the two entries
are the same routine.

Four guards, all readable off the symbol table:

1. `st_other`'s local-entry field must decode to a real offset — the ABI packs it in bits
   5–7 as `(1 << n) >> 2 << 2`, so only `n ∈ 2..=6` (4/8/16/32/64 bytes) folds; 0 and 1
   mean the entries coincide and 7 is reserved.
2. A sized symbol must actually contain its own local entry (`offset < st_size`).
3. The local entry must not be the address of any other defined text symbol.
4. The global entry must itself be a walk seed, with no other seed between the two.

Guard 4 is what makes this provably non-destructive rather than merely careful: it keeps
the walk's instruction closure invariant, because the bytes at the local entry are reached
as the global entry's fall-through either way. The fold can only ever remove the duplicate
second entry over a body that is walked regardless — it can never lose a body. As with
`unmappedentry`, only the *function* claim is withheld; the Call cross-reference is filed
in both directions either way.

PPC64-only and inert on an image whose symbols carry no local-entry annotation, so the
architecture check short-circuits before any per-seed work is done on every other target.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need ppc64-localentry-splits-function
acceptance suite
  PASS   closed         ppc64-localentry-splits-function
total=1 pass=1 fail=0 closed=1 regressed=0 indeterminate=0
```

```
$ kuna decompile .../fixtures/plt_ppc64le __do_global_ctors_aux
void __do_global_ctors_aux(void) // return-dupe
{
  ...
  do {
    (*v2)();
    v3 = &v3[-1];
    v2 = (void *)*v3;
  } while (v2 != (void *)0xffffffffffffffff);
}
```

Promoted verbatim (clauses and in-repo target unchanged) into
`tests/cli/ppc64-localentry-splits-function.json`, and backed by a cross-crate two-pass
cargo test — `kuna-console/tests/verify_ppclocalentry.rs` — which `make rust-test` does
run. No `tests/stages/` case: that corpus is symbol-less `<binaryimage>` bytechunks, and a
defect gated on the ELF symbol table's `st_other` field is structurally unreproducible
there (the same reason `noreturn_propagate` shipped with a cross-crate e2e instead).

## Sweep

`plt_ppc64le` is the only PowerPC binary in the tree, so the whole in-repo blast radius is
one image, swept whole-binary with `decompile-all`:

* **Option OFF vs pre-change main: byte-identical.** The two-pass gate holds exactly.
* **Default vs before:** 19 entries → 15. The four husks and their four anonymous twins
  collapse into four correctly-sized functions (`deregister_tm_clones` 8→96,
  `__do_global_dtors_aux` 8→240, `frame_dummy` 8→12, `__do_global_ctors_aux` 8→120 —
  each matching `objdump`). Every `funcboundflow` truncation warning in the image is gone.
  Nothing is added and no surviving function shrinks (asserted by the third cargo test).
* The one non-obvious hunk: `__do_global_dtors_aux`'s TOC-relative displacements change
  base, `v1 + -0x7ef0` → `v2 + 0x1f850`. That is the same address — the function now
  starts at its global entry, so the decompiler follows `r12` through the `addis/addi`
  TOC setup instead of starting from an opaque `r2` — and every offset checks out
  (`r2 = r12 + 0x27740`, so `r2 - 0x7ef0 == r12 + 0x1f850`).

**Known residual, recorded rather than hidden:** a call site to a folded local entry still
renders as `sub_6f8()` — a generated name for an address that is no longer in the
inventory. Naming it after its enclosing function is a decompiler-core call-resolution
change, a second mechanism in a different crate, and is left as a follow-up. It is
strictly smaller than the defect being fixed: before this PR the body itself was
unreachable under its real name.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK**, 675/675 assertions |
| `make test-stages` | **PARITY OK**, 603/603 assertions |
| `make rust-test` | green — **5,341 passed / 0 failed** |
| `make check-spec` | OK (lenient **and** `--strict`) |
| `kuna catalog --check` | catalog OK (138 settables) |
| `scripts.repipe.clitests` | 19/19 |
| `scripts.repipe.counters` | no drift |
| acceptance `a-fb2b9c0f7f5b` | **PASS** |

Speed: **-0.97%** (interleaved off/on, 25 rounds, median) on a provably inert x86-64 target, against an **on-vs-on control of +0.02%** on the same target — the option's architecture check returns an empty map before any per-seed work, so both arms run identical code there. Budget 5%. The witness measures +8.80% median (control +0.34%), which is the cost of decompiling a recovered 120-byte body where the off arm decompiled an empty husk. Minima are unusable on this box: the identical-work control swings +23.43% on the minimum, so medians are reported.

`docs/baseline.json` is untouched.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
