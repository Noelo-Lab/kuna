# undefname -- a `$$undef` placeholder reaches emitted C

decbench campaign, NOVEL pool (`loose-undefname-leak`). Filed on
`O2-noinline-iproute2-ip`; the sharpest witness is `O2-tar-tar` `xstrcat`.

## The symptom

kuna emitted identifiers of the shape `$$undefNNNNNNNN`. Three things are wrong with
that at once:

1. `$$` is not a legal C identifier character, so the function does not compile.
2. The placeholder is a *second* identifier for a stack Symbol that is already
   declared under its `vN` name, so one slot is declared twice.
3. Because the two names are separate, the body **reads a variable that is never
   written** while the update lands on the other name. 221 of 283 affected functions
   in the original sample contain such an uninitialised read.

Nobody else does this: across the stored 803-binary x 11-decompiler decbench run,
kuna's panes contained `$$undef` and angr / ghidra / ida / binja / phoenix / dewolf /
r2dec never did.

### Witness 1 -- `tar` `xstrcat` (`O2`, `0x611a0`)

```c
  uint4 $$undef00000000; // stack - 0x58     <-- declared, READ TWICE, NEVER WRITTEN
  uint8 v12;             // stack - 0x58     <-- SAME SLOT, written twice, read once
  ...
      v4 = v12 & 0xffffffff;
      if (0x30 <= (uint4)$$undef00000000) {  ... }
      else v12 = (uint8)((uint4)$$undef00000000 + 8);
```

`a1` is a `va_list` and the loop is the x86-64 `va_arg` expansion
(`if (gp_offset >= 48) use overflow_arg_area else reg_save_area + gp_offset;
gp_offset += 8`). As emitted, the loop tests and increments an uninitialised
variable and the induction update it *does* perform is never read. Recompiled, it
does not terminate correctly.

### Witness 2 -- `ip` `set_qlen` (`O2-noinline`, `0x2b420`)

```c
  char $$undef00000001 [16]; // stack - 0x38
  ...
  $$undef00000001 = CONCAT124(v4[0],a1);
```

Here the placeholder is in the **Symbol table itself**, so it also leaks out of the C
into the structured surfaces: the `variables` array of `decompile-all` in JSON mode
literally carried
`{"name":"$$undef00000001","type":"unsigned int","stack_offset":-56,"size":4}`,
and `decompile-project`'s `.asm` stack-frame comments carried
`; stack: $$undef00000005 @ [stack-0x158] (int4[34])`.

## Root cause, as instrumented

`Scope::buildUndefinedName` (`database.cc:2854`) names a Symbol created with an empty
name `$$undef` plus 8 hex digits. Upstream that string is unobservable for two
reasons, and kuna inherits neither:

* upstream `ActionNameVars::apply` renames **every** still-undefined Symbol before it
  returns -- its final statement is `localmap->assignDefaultNames(base)`
  (`coreaction.cc:3079`);
* upstream `PrintC` reads `sym->getDisplayName()` **live** at emit time, so the render
  can never see a stale name.

kuna binds the name **per HighVariable** (`HighVariable::kuna_name`, a cached `String`)
and the printer reads that cache; and `Database::assign_default_names`, a complete port
of `ScopeInternal::assignDefaultNames`, had **zero call sites in the tree**
(`grep -rn assign_default_names decompiler/` returned only its own definition).

### The decisive instrument

The original filing used the console `rename` command as its discriminator. That
instrument perturbs (it also deletes the stack-canary locals), and a refuter showed the
JSON `variables` array -- which `extract_variables` reads straight off the ScopeLocal
Symbol table -- contradicts the filing's "two Symbols" reading:

```
$ kuna decompile-all .../O2/tar/stripped/tar --json --addr 0x611a0
{'name': 'v12', 'type': 'uint8', 'kind': 'stack', 'stack_offset': -88, 'size': 8}
{'name': 'v13', 'type': 'unsigned long *', 'kind': 'stack', 'stack_offset': -80, 'size': 8}
```

Exactly **one** Symbol at -88, no `$$undef` Symbol anywhere. So the printed
placeholder corresponds to no live Symbol; it is a stale `String` cached on a sibling
HighVariable.

A temporary `eprintln!` in the naming loop plus a backtrace-capturing
`HighVariable::set_kuna_name` pinned it exactly (note: `kuna decompile` shells out to
`decomp_dbg` with **stderr piped**, so the in-process `decompile-all` path is the one to
instrument):

```
[undef] high=HighVariableId(103) addr=-88 size=4 entry_size=8 sym_off=0 -> $$undef00000000
[undef-set] $$undef00000000
Backtrace [ HighVariable::set_kuna_name, name_local_highs_angr, Action::perform, ... ]
[undef] high=HighVariableId(104) addr=-88 size=8 entry_size=8 sym_off=0 -> v12
```

Read that as the whole bug:

* high **103** is the 4-byte partial cover of the slot. `sym_off == 0` but
  `size (4) != entry_size (8)`, so `ScopeLocal::resolve_default_name_override`
  (`varmap.rs:1538`) fails its whole-symbol gate and returns the Symbol's *current*
  display name -- the placeholder -- which `set_kuna_name` freezes onto the high.
* high **104** is the 8-byte whole cover, visited later in location order. Its gate
  passes, so the shared Symbol is renamed to `v12`.
* Nothing back-fills 103's cache.

The gate itself is **not** the defect: `!(sym_off == 0 && size == entry_size)` is a
faithful port of C++'s `sym->isNameUndefined() && high->getSymbolOffset() < 0` namerec
condition. Upstream is safe with the identical gate purely because its printer re-reads
the Symbol. The divergence is kuna's cache, and the port already knew about this class
of hazard -- `coreaction_cleanup.rs:2778-2790` documents having moved the *spacebase*
pass after the main naming loop for exactly this reason. The same hazard **inside** the
loop, between two highs of one Symbol, was not addressed.

Mechanism 2 (witness 2) is simply the unwired catch-all: no high reaches the
whole-cover gate at all, so the Symbol is never renamed and keeps the placeholder in
the database.

## The fix

`p6_variables/kuna_undefname.rs`, called from the tail of `name_local_highs_angr` --
the same place the port already resolves this class of hazard.

1. **`assign_scope_default_names`** wires `Database::assign_default_names` through a new
   one-line `ScopeLocal::assign_default_names` accessor, reproducing
   `coreaction.cc:3079`. Every remaining `$$undef` Symbol gets its `buildDefaultName`.
   This also cleans `extract_variables` and the project `.asm` annotations for free,
   because those read the Symbol table.
2. **`refresh_placeholder_high_names`** re-walks the highs; any whose `kuna_name()` has
   the placeholder shape (`len == 15 && starts_with("$$undef")`, i.e.
   `Symbol::is_name_undefined`) is re-resolved through the **same**
   `ScopeLocal::query_container_for_link(addr, usepoint)` containment query that bound
   the stale name, and re-bound to the now-final name. This is kuna's stand-in for
   upstream's live `getDisplayName()` read, applied once at the end of the pass.

Only the name is rewritten. The in-symbol offset and symbol type the naming pass bound
came from this same entry and were already correct.

**This is not a no-op.** The stale cache is reachable only inside
`if let Some(info) = container`, i.e. `query_container_for_link` already succeeded for
that high's name representative and usepoint, so re-running the identical query at end
of pass returns the same entry with the now-final name. Empirically every emitted
placeholder is gone -- see `plan.md` for the corpus A/B.

**Ordering.** Both steps run after every `vN` has been handed out and after the
spacebase attach pass, so neither can renumber an existing local: a Symbol named in
step 1 takes an index above every other local (`ip set_qlen`'s becomes `v7`, one above
`v6`), and step 2 consumes no index at all. Placing the tail after the spacebase pass
also keeps `link_symbol_reference`'s `is_name_undefined` bail -- and therefore every
`&symbol` render -- byte-identical.

The tempting one-liner (drop the whole-cover gate so the first touch always renames)
would also work, but it moves *when* a Symbol consumes `base`, renumbering every later
local in any function where a partial high precedes its whole-cover sibling. The repair
form is the zero-churn one, and it keeps the C++-faithful gate.

## Why no option

`AGENTS.md`: "a strict bug fix that only corrects wrong output needs no flag." An
identifier containing `$$` is not valid C under any naming policy, and the split made
the body read a name nothing writes. The ablation confirms it: `kuna test --all
--baseline docs/baseline.json` stays **PARITY OK 675/675** with the fix unconditionally
on and no re-pin.

## Interaction with DIV-52 -- corrected

The triage predicted that DIV-52's `collapse_symbol_decls` would fold the now-duplicate
declaration once the names agreed. Measured, that is **not** the mechanism that fires.
The declaration is folded by the older, ungated `is_scalar_partial_piece` guard
(`printc.rs:2516-2536`): a high that is a strict partial of a scalar mapped Symbol is
skipped when a whole-cover sibling **of the same name** exists. Before the fix the names
differed (`$$undef00000000` vs `v12`) so the guard could not match; after the fix it
does. Because that guard is not part of `dedupvardecls`, the fold happens on the
`option dedupvardecls off` path too -- which pass 2 of the stage test pins.

## Owning phase

**P6** (variable and storage model): `ActionNameVars` lives in `p6_variables/`. The
symptom appears at P9, but P9 only reads a cache P6 wrote -- `docs/phases.md`'s "a
symptom's phase is not always its decision's phase". Spec prose:
`docs/spec/06-variables-and-merge.md` section 6.1, "Closing out the undefined names".
