# char-pointer string-literal render (w10-stackframe-cluster) — ACCEPT, +6

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-stackframe-cluster` @ `59f9ea7`,
base 60756b5/430). Verdict: **ACCEPT**. Scoped at the stack-frame cluster but the harvested root
was the shared printc string-literal render (the named families' real roots are deeper — see
below).

## Root + fix

1. **Missing typed constant dispatch for pointer-to-char.** Ported `PrintC::pushPtrCharConstant`
   (`printc.cc:1767`) + `printCharacterConstant` (`printc.cc:1602`) → `printc.rs`
   (`push_ptr_char_constant_ir`/`print_character_constant`, wired into the pointer arm of
   `push_vn_explicit_ir` ~4226). `StringManagerUnicode::get_string_data` was already ported —
   only the printc-side reader/escape was absent.
2. **Spurious duplicate array/struct local declaration.** `emit_local_var_decls` (~1916) deduped
   HighVariables by high id instead of by Symbol; added a `(name, composite-type Rc-identity)`
   collapse faithful to C++ `emitScopeVarDecls` one-decl-per-multi-entry-Symbol (`printc.cc:2696`).
   This is a +0 substrate (flipped no assertion alone) folded into the commit.

## Effect

430 → 436 on its base; +6 on the integrated 435 tree → **441**. Gained: Intermediate pointers
#1/#2, Inlining #4, For-loop thru special #3, For-loop var used #3, Long double #10. Required
updating the in-tree `verify_w10_inline_body` count 7→8 (the new Inlining #4 string-literal pass).

## Gate (re-run at integration onto 435)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 441]`; passing-set diff vs base 435: **regressed-set EMPTY** (gained exactly
  the 6 above). printc.rs auto-merged with the BOOL_NEGATE wave (different regions).
- switch cluster: 8/16/3 — unchanged. oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining stack-frame family roots (deeper than the RSP substrate unblocks — for future waves)

- **Stack string** — `StringSequence`/`ArraySequence` + `transform()` unported in `constseq.rs`
  (stub `RuleStringCopy::apply_op` returns 0). That file is owned by the heap-string wave
  (`rport/w10-heapstring-constseq`); the stack half should land there to avoid conflict.
- **Partial splitting** — `SplitDatatype` partly works (`ptr->field` renders); the gap is the
  `arr` array local + an unrecovered memset-like call.
- **Piece Structure** — `RulePieceStructure` (register-passed struct → field split) in
  `coreaction_cleanup.rs`; `v1` still typed `int8`, fields unassigned.
- **Stack spill / Stack Return** — register+stack struct-parameter prototype join (params render
  `a0/v1` not the injected `a/d`); lives in `funcdata.rs`/`variable.rs`.

## No special-casing

General typed-constant render matching C++ `pushPtrCharConstant`; no name/address/value
hardcoding.
