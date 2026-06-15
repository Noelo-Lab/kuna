# rport/w10-displayformat

verdict: ACCEPT-WITH-LOSSES
verifier: Opus 4.8 (independent verifier, NOT the porter)
date: 2026-06-15
round: 1

gate (re-run fresh, all in /tmp/w10-displayformat):
  - cargo build --workspace                         -> OK (clean)
  - cargo test --workspace                           -> OK, 0 failed (165 "test result: ok." lines, 0 FAILED)
  - kuna-console lib unittests                        -> 183 passed (>= 182 required)
  - cargo clippy -p kuna-decomp --lib -- -D warnings  -> clean (exit 0)
  - cargo clippy -p kuna-console -- -D warnings       -> clean (exit 0)
  - cargo clippy -p kuna-decomp --test verify_w10_displayformat -> clean (exit 0)
  - C++ oracle (python -m kuna.run_tests --all --baseline docs/baseline.json)
        -> unittests 207/207, datatests 675/675, PARITY OK  (C++ side byte-identical: diff touches only rust/)
  - B5 / printc parity (printc_parity, print_b5_boolless) -> green
  - whole-pipeline parity (corpus_bootstrap, decompile_e2e) -> green

## Summary

Faithful transcription of the `force varnode <varnode> <format>` path:
IfcForceFormat -> IfaceDecompData::readVarnode -> Funcdata::buildDynamicSymbol
(constant/equate arm) -> Scope::setDisplayFormat + setAttribute(typelock), and
the per-Symbol display-format honoring threaded into PrintC::push_integer.

The change is surgical and non-regressing: the existing `push_constant_ir`
delegates to a new `push_constant_ir_fmt(.., display_fmt_in)` with `display_fmt_in
= NONE`, so byte-parity output is unchanged for every datatest. The forced format
only ever differs when an equate Symbol is bound — which requires the interactive
`force varnode` command, never reached by the datatest corpus. The DynamicHash
core (`dynamic.rs`) is UNCHANGED on this branch (ported earlier at w7-s6, out of
scope here); this item only wires it up.

No special-casing: grepped the full diff for function-name/address/value
hardcoding (`"main"`, `0x40....`, named funcs), HashMap/HashSet, todo!/
unimplemented!, sort_unstable, and bare `as` casts in added lines — ZERO hits in
production code (the only `.unwrap()` is in a porter test; the only `as` tokens
are inside doc comments). The hex/dec/oct/bin/char encode mapping (1..5) is a
verbatim port of `Datatype::encodeIntegerFormat` (type.cc:732) — no value or
format is special-cased; `format_integer_token` renders all five shapes.

New PASSES are REAL parity, not weakening: confirmed the format-resolution fork
against the C++ `push_integer` formatter directly (see adversarial tests; binary
padding, char-escape, and signed-flip-suppression all matched the C++ oracle —
one of my own test expectations was wrong about binary zero-padding and the C++
`formatBinary` (printlanguage.cc:800) proved the Rust output correct).

## Hunt list

- Signedness: clean. `uq == !0u32` matches C++ `uq == ~((uintm)0)` (uintm=uint32_t,
  types.h:27 == rust u32). The `for i in 0..n` (n = op.num_input()) loop matches
  C++ `for(int4 i=0;i<op->numInput();++i)` — exclusive bound, no signed/unsigned
  hazard.
- Integer widths: clean. format codes are `uint4` both sides; maxduplicates 8/16
  and getBase(1,TYPE_UNKNOWN) threaded as parameters exactly as C++
  `uniqueHash`/EquateSymbol ctor read them from `glb` (dynamic.cc:433-439,
  database.cc:629).
- Wrapping: clean. The only arithmetic on uintb in the threaded path is the
  signed-flip `flip.wrapping_add(1)` in resolve_integer_format (pre-existing) —
  guarded by `sign && fmt!=force_char`, the C++ branch; not reached on the
  unsigned forced-dec/hex path (adversarial test F4 exercises u64::MAX with no
  panic).
- Comparator totality: n/a (no comparator added).
- Iteration-order provenance: clean. The readVarnode constant arm iterates
  `op.get_in(0..num_input)` by index — same order as C++ `op->getIn(i)`. No
  HashMap/HashSet anywhere in the diff.
- Off-by-one / do-while / reverse iterators: clean. No do-while; the input scan
  is a plain forward indexed loop matching the C++.
- Erase-while-iterating: n/a (no erase-during-traversal in scope).
- Exception -> Result partial-state parity: clean and notable. C++ ordering is
  buildDynamicSymbol (creates equate Symbol seeded force_hex) -> read formatString
  -> encodeIntegerFormat (THROWS on bad format, AFTER symbol exists, BEFORE
  setDisplayFormat/typelock). The Rust port reproduces this exactly: the format
  `match` returns Err before `set_display_format`/`set_attribute`, leaving the
  same partial state (equate Symbol present with default force_hex, no
  display-format override, no typelock). The buildDynamicSymbol guard order
  (isTypeLock/isNameLock then !isHighOn then getSymbol-already-exists then
  uniqueHash) is preserved verbatim.

## findings

  - F1 (minor): readVarnode's "Missing p-code sequence number" is downgraded from
    IfaceParseError to IfaceExecutionError. C++ readVarnode throws
    `IfaceParseError` here (ifacedecomp.cc:1480), which the console renders as
    "Command parsing error: ..." (ifacedecomp.cc:3625). The Rust `read_varnode`
    returns a plain `String` for this case (ifacedecomp.rs:419) and the
    IfcForceFormat call site maps the whole result through `IfaceError::execution`
    (ifacedecomp.rs:2154), so it renders as "Execution error: ...". The error
    KIND is observable (the dispatcher prefixes Parse vs Execution differently),
    so this is a faithful-transcription gap. Severity is minor: it is on an
    error-only path (a malformed `force varnode` on a constant lacking a seqnum),
    affects only the message prefix, not the success path, and is not exercised by
    any datatest or the B5 oracle. The other readVarnode/IfcForceFormat throws all
    keep their correct class (all the genuine IfaceExecutionError throws).
       cpp:  decompiler/cpp/ifacedecomp.cc:1480 (IfaceParseError)
             decompiler/cpp/ifacedecomp.cc:3624-3629 (Parse vs Execution prefixes)
       rust: rust/crates/kuna-console/src/ifacedecomp.rs:419 (plain String)
             rust/crates/kuna-console/src/ifacedecomp.rs:2154 (map_err execution)

  Not findings (verified faithful / out of scope):
  - readVarnode loc-scan arm (exactly one of pc/uq) errs as a documented seam —
    UNREACHABLE for `force varnode` (a constant always takes the IPTR_CONSTANT
    arm, which requires the seqnum and IS implemented). Traces to LOSS-029
    (beginLoc(Address)/endLoc(Address) loc-set range probes unported). No
    functional loss for this item.
  - buildDynamicSymbol non-constant arm (addDynamicSymbol over high->getType())
    errs as a documented seam — UNREACHABLE for `force varnode` (the command
    rejects non-constant varnodes BEFORE calling buildDynamicSymbol). Traces to
    LOSS-036 (funcdata_varnode.cc:1304 buildDynamicSymbol listed among W4-deferred
    methods; the Varnode->SymbolEntry retype link). No functional loss for this
    item.
  - The `vn->setSymbolEntry(...)` -> `high->getSymbol()` effect is reproduced by a
    merged-tree stand-in (`HighVariable::kuna_equate_symbol`) read back by
    `vn_high_display_format` in PrintC. Faithful for the constant-format path:
    setAttribute uses typelock only (not namelock), so C++ push_integer skips
    pushEquate (needs isNameLocked) and runs `displayFormat = sym->getDisplayFormat()`
    unconditionally — exactly what the stand-in returns.

## adversarial tests

Added rust/crates/kuna-decomp/tests/verify_w10_displayformat.rs (5 tests, all
passing against both the branch worktree and the rust-port main tree; targets the
load-bearing forced-format-wins fork of C++ push_integer, engine-free and
oracle-grounded):

  - forced_format_overrides_natural_base_default  — a non-NONE display_fmt_in wins
    over the val<=10 / mostNaturalBase default (255 forced dec -> "255";
    7 forced hex -> "0x7").
  - forced_format_beats_force_hex_and_force_dec_mods — the Symbol's forced format
    is checked before the force_hex/force_dec mods (C++ `if (displayFormat!=0)`
    precedence), with a NONE control proving the mod path.
  - each_encode_integer_format_renders_its_own_shape — all five encodeIntegerFormat
    codes render distinct C++ shapes (0x41 / 65 / 0101 / 0b01000001 / 'A'); the
    binary padding matches C++ formatBinary's byte-boundary rule.
  - force_char_suppresses_signed_flip — C++ `sign && displayFormat!=force_char`:
    0xFF size-1 forced char is NOT two's-complemented ('\xff'); contrast forced dec
    flips to -1.
  - forced_dec_on_all_ones_uintb_prints_full_magnitude — u64::MAX forced dec prints
    18446744073709551615 (no signed flip, no debug wrap panic); forced hex prints
    the full 16 nibbles.

## losses

ACCEPT-WITH-LOSSES is granted on the basis that the two error-as-documented seams
in the diff (readVarnode loc-scan arm; buildDynamicSymbol non-constant arm) are
both UNREACHABLE by the `force varnode` command and trace to pre-existing,
already-recorded ledger entries:
  - LOSS-029 (readVarnode loc-scan -> beginLoc(Address)/endLoc(Address))
  - LOSS-036 (buildDynamicSymbol non-constant arm -> W4 Varnode->SymbolEntry link)

This item introduces NO new functional loss: the entire reachable surface of
`force varnode` (constant resolution + equate Symbol + force-format + type-lock +
print honoring) is ported and verified. F1 (Parse->Execution error class on the
seqnum-missing path) is a minor cosmetic divergence on an error-only path; it is
recorded as a NEW loss (see losses.md) rather than blocking, consistent with the
attempt-budget guidance for a faithful PARTIAL with zero parity regression.
