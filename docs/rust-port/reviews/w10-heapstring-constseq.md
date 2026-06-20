# Review: w10-heapstring-constseq (Round 1)

**Verdict: ACCEPT_WITH_LOSSES**

Branch `rport/w10-heapstring-constseq` (tip `53ac719`), parent/merge-base `79e51bd`.
Diff: `git diff 79e51bd..rport/w10-heapstring-constseq -- rust/` — **rust-only**,
two files, `+147/-22`.

## Scope of the change

The diff restores **one** previously-stubbed function and adds four adversarial
tests. It does **not** wire the heapstring transform.

* `rust/crates/kuna-decomp/src/constseq.rs` — `ArraySequence::select_string_copy_function`
  changed from an unconditional `BUILTIN_MEMCPY` stub to the faithful identity
  selection; doc comments updated.
* `rust/crates/kuna-decomp/src/constseq/tests.rs` — 4 new tests (additive only).

No other file is touched. **No reserved file** (coreaction_infertypes / funcdata_spacebase /
varmap / coreaction_casts / funcdata.rs / funcdata_block / blockaction / heritage.rs /
ruleaction_2/3/5 / printc.rs / dynamic.rs / database.rs) appears in the diff. Verified via
`git diff --name-only`.

## (1) Faithfulness — PASS

C++ `ArraySequence::selectStringCopyFunction` (`constseq.cc:161-175`):

```cpp
TypeFactory *types = data.getArch()->types;
if (charType == types->getTypeChar(types->getSizeOfChar()))  { index = numElements; return BUILTIN_STRNCPY; }
else if (charType == types->getTypeChar(types->getSizeOfWChar())) { index = numElements; return BUILTIN_WCSNCPY; }
index = numElements * charType->getAlignSize();
return BUILTIN_MEMCPY;
```

The Rust is line-faithful:
* narrow match → `(BUILTIN_STRNCPY, num_elements)`
* wide match → `(BUILTIN_WCSNCPY, num_elements)`
* fall-through → `(BUILTIN_MEMCPY, num_elements * char_type.get_align_size())`

The C++ pointer-identity `==` is rendered as `Rc::ptr_eq(&self.char_type, &narrow)`.
This is a faithful analogue: `TypeFactory::get_type_char` returns the single interned
`Rc` from `charcache[s]` (`dtype.rs:4939`), so `ptr_eq` against another `get_type_char(s)`
result is true iff it is the same interned core char type — exactly the C++ semantics.
The "no factory / unbuilt char type" path falls through to `BUILTIN_MEMCPY`, which is
correct: an unbuilt char type can never compare equal, and the C++ never sees a null
`charType`, so the fall-through matches.

The collect side (`form_byte_array`, `MINIMUM_SEQUENCE_LENGTH`, the COPY/STORE op-list
guards) and the `RuleStringCopy`/`RuleStringStore` `apply_op` bodies are **unchanged** by
this diff and remain in their gated (decline → `return 0`) form, byte-identical to the
rule being disabled. The CALLOTHER/builtin-insertion `transform()`/`buildStringCopy()`
half is *not* ported and is *not* claimed to be.

## (2) No special-casing — PASS

`grep -iE` over the added lines for string literals (`"Message`, `FOUR`, `THREE`,
`ELEVEN`, `warning!`, `NEGATIVE`), datatest addresses (`0x100000`, `fillin`, `negptr`),
name/value comparisons (`== "`, `strcmp`, `getName() ==`, `funcname`) — **zero hits** in
both src and tests. The selection is a general factory-driven identity match; the tests
build their own type fixtures and a genuinely-foreign 1-byte INT for the fall-through.

## (3) heapstring REAL parity — HONEST +0 (loss, not special-cased)

`heapstring.xml` requires the rendered `builtin_strncpy(ptr->val,"...",N)` CALLOTHER
(7 stringmatches), which needs the transform + the W8 printer's CALLOTHER user-op-name
resolution + `BUILTIN_STRINGDATA`→literal lookup (`printc.cc:1609`). None of that is
ported. Re-ran via the branch's own harness: **Heap string #1–#7 all FAIL** (per-test
`FAIL --` lines). This matches the commit's "heapstring +0" and is honestly recorded as
a loss — the restored selection is dormant (the rule never reaches it).

## (4) No regression — PASS (independently verified)

* `cargo test --workspace` (heapstring worktree): **EXIT 0, 3654 passed, 0 failed**.
  The 4 new tests (`select_copy_fn_narrow_char_picks_strncpy`,
  `select_copy_fn_wide_char_picks_wcsncpy`, `select_copy_fn_non_char_falls_through_to_memcpy`,
  `select_copy_fn_no_factory_falls_through_to_memcpy`) all pass.
* `cargo clippy --lib -p kuna-decomp`: **EXIT 0**, no warnings from the changed file.
* Datatests, heapstring branch: **400/675 passing, 275 failing**.
* Datatests, baseline (`79e51bd`, independently built): **400/675 passing, 275 failing**.
* `comm`/`diff` of the full FAIL sets: **byte-identical**. Regressed set **EMPTY**;
  gained set **EMPTY** (consistent with +0). switchind / switchhide / ifswitch / boolless /
  condconst / condconst2 / stackstring all unchanged.
* C++ oracle: diff is **rust-only** (no `decompiler/`, no `docs/baseline.json`), so the
  C++ 675/675 PARITY OK and B0 are untouched by construction.

## Adversarial tests — 4 added (≥3 required), all green

Narrow→strncpy, wide→wcsncpy (asserts wchar size 2 + char-count index), foreign-type
fall-through→memcpy (byte-count index), and no-factory fall-through→memcpy. They pin both
the function selection *and* the index semantics (char count vs byte count) on every path.

## Losses

* **LOSS (carried, not new):** heapstring datatest remains 0/7 — the `RuleStringStore`/
  `RuleStringCopy` `transform()`/`buildStringCopy()` half is unported (gated on the
  `StringManager` registration seam, the typed-pointer PTRSUB/PTRADD builder, and the W8
  printer CALLOTHER/`BUILTIN_STRINGDATA` rendering). The restored
  `select_string_copy_function` is correct but **dormant** until that half lands.
