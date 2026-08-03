# subright — plan

## Scope: small

One rule body, one file, no new module, no registration change, no schedule
change. `RuleSubRight` is already registered `rrow!("subright", "cleanup", ...)`
at `infra/universalaction.rs:552`, in upstream's own position in the cleanup
pool. The whole change is the ~45-line tail of one `apply_op`.

## The change

Complete the port of
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
(RuleSubRight::apply_op)` from upstream `ruleaction.cc:7291-7340`
(`GHIDRA_REV=cef869af`), faithfully and completely — including every guard
upstream applies, because each one exists to protect a rendering kuna already
gets right:

1. `c == 0` — the SUBPIECE is already least-significant; it renders as a cast.
2. `outvn` and `a` both address-tied with `overlap == c` — the SUBPIECE is a
   storage marker `ActionCopyMarker` will convert. Dropping this guard would
   regress stack-piece writes that currently render `sym._n_m_` into shift
   chains.
3. The `isPieceStructured` special-print branch (already ported) stays first, so
   a struct/union/array field extraction is still rendered as a field access.

Then the rewrite: lump a lone `INT_RIGHT`/`INT_SRIGHT` descendant with a constant
shift when `outvn->getSize() + c == a->getSize()` (declining an `INT_RIGHT` whose
combined amount would evaluate to zero and clamping `INT_SRIGHT` to the sign
bit), synthesize the shift op before the SUBPIECE typed `TYPE_UINT`/`TYPE_INT` at
the input width, and rewrite the SUBPIECE's offset input to 0.

One kuna-side deviation, marked in the code: the type-factory availability check
is hoisted above the first mutation. Upstream reads `glb->types` only after it
has already rewired the graph; a hand-built `Funcdata` with no type factory (the
rule-level unit-test fixtures) would otherwise leave a half-applied transform
behind. On a real architecture the factory is always present, so behavior is
identical.

## No option — the gating decision

`CLAUDE.md` gates *features* ("behavior that is a judgment call, not universally
better") and exempts "a strict bug fix that only corrects wrong output". This is
the exempt case, for four independent reasons:

1. **The old output is not C.** `SUB41`/`SUB81`/`SUB84` are undeclared
   identifiers; kuna emits no header defining them. There is no user who wants
   the non-compiling spelling, so there is no honest `use_when` prose to write
   for a `subright` option.
2. **It restores upstream, it does not diverge from it.** This is finishing a
   port, not making a design choice: the output shape is byte-identical to
   Ghidra's on the witness.
3. **The ablation is silent.** 0 of 675 vendored-from-upstream datatest
   assertions move — exactly what "restoring upstream behavior" predicts.
4. **The switch already exists.** `subright` is a registered rule-group name in
   the action pool, so `option togglerule subright off` already gives a bisector
   the pre-fix rendering. A new `settableTable` row would add catalog-count churn
   and a documented knob nobody should flip.

An independent decider subagent was asked to make this call from the repo's own
rules and precedent and returned `ungated`; its verdict is recorded verbatim in
`record.json` under `decisions`.

Precedent is unanimous for this shape: #226 (DIV-46), #232 (DIV-48), #235
(DIV-49), #238 (DIV-50), #242 (DIV-52) are all `fix(pN):` corrections that change
emitted C with no settable option, shipping instead a `docs/history.md` row, spec
prose, and a `ghdec-*` stage test.

## Where it is recorded

`docs/history.md` **Convergences** table, not a DIV row: the registry's own rule
is that "a port defect whose fix moves kuna *back onto* upstream's default earns
no DIV row, but a corpus-wide output shift still needs a record for anyone
bisecting."

## Test

`tests/stages/ghdec-subright.xml`, two-pass over a real `binary -> decompile`
path, on a 44-byte i386 bytechunk built from

    void put_u32_be(u8 *p, u32 v) { p[0] = (v >> 24) & 0xff; ... }
    u32  mulhi_shift(u32 a, u32 b) { return (u32)(((u64)a * b) >> 32) >> 3; }

(`gcc -m32 -O1 -fno-stack-protector -fno-pic`). `put_u32_be` covers the plain
rewrite and the `c == 0` bail; `mulhi_shift` covers the lone-descendant lumping
(the `mull` high half is `SUBPIECE(prod,4)` read only by a constant `INT_RIGHT`,
folded into one shift by `32+3 == 0x23`) and uses a *variable* multiplier so
`RuleDivOpt` declines and the truncation actually reaches the printer.

Pass 1 is the default (the fix); pass 2 sets `option togglerule subright off` and
pins the pre-fix rendering. 1 of 8 assertions passes pre-fix, 8/8 post-fix.

## Risks, and how each was checked

- **Datatest churn** — measured: 675/675 PARITY OK, no re-pin.
- **`ActionCopyMarker` interaction** — the address-tied bail is ported verbatim;
  the corpus residuals confirm it fires (42 of the 45 remaining raw operators are
  exactly that family).
- **`op_unlink` ordering in the lumping path** — transcribed in upstream's order
  (unlink the SUBPIECE, rebind `op` to the descendant, retype it to SUBPIECE,
  then build the shift at the *descendant's* address).
- **Speed** — the rule fires on ~3% of functions and adds one op each; measured
  interleaved A/B, reported in `record.json`.
