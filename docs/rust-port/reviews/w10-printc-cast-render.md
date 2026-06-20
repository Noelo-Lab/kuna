# w10-printc-cast-render
verdict: ACCEPT
verifier: opus-4.8-1m (independent verifier, round 1)
date: 2026-06-15
branch: rport/w10-printc-cast-render @ 76a0764 (+ 3d655be verifier tests)
gate: cargo test --workspace -> 3428 passed / 0 failed / 37 ignored
      (base rust-port = 3423; branch = base + 5 new verifier tests, zero regressions);
      cargo clippy -p kuna-decomp --lib -- -D warnings -> clean;
      C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` ->
      207/207 unit + 675/675 datatest, PARITY OK (untouched — diff is rust-only).

## Scope

The diff (88 lines, three files, only `rust/`) closes the last printer cast seam:
`PrintC::opTypeCast` (printc.cc:468), the C cast-notation `(type)operand` form
shared by `opCast` / `opFloatFloat2Float` / `opFloatTrunc` (printc.hh:332/333/341,
all unconditional `{ opTypeCast(op); }`). Previously `OpEmitKind::TypeCast` shared
the `Func|Custom` catch-all arm and rendered the functional `OPC(args)` form
(LOSS at losses.md:1852). Now:

  - `op_push_ir` routes `OpEmitKind::TypeCast` to a new `op_type_cast_ir`
    (printc.rs:2580), split out of the `Func|Custom` arm.
  - `op_type_cast_ir` (printc.rs:2717) mirrors printc.cc:479-483: under
    `!option_nocasts` it pushes the `typecast` op-token and the OUTPUT varnode's
    def-facing type, then recurses on in0.

Two existing tests were adapted to the now-faithful notation (not weakened — see
"No special-casing / no test weakening").

C++ source in scope read at the rust-port tree: printc.cc:396-484
(`checkAddressOfCast`, `opTypeCast`), printc.hh:201/332/333/341,
the `typecast` token (printc.cc:36).

## Faithfulness (transcription, not a shortcut)

opTypeCast has four parts; the port reproduces three and seams the fourth:

1. `Datatype *dt = op->getOut()->getHighTypeDefFacing();` — Rust reads
   `fd.vbank().get(out).get_type_def_facing()` off `op->getOut()`
   (printc.rs:2721-2724). The target type is the OUTPUT varnode's type, NOT a
   hardcoded or opcode-keyed string. `get_type_def_facing` (varnode.rs:586)
   returns the varnode's own data-type (the W8 union `findResolve` arm is a
   documented seam, faithful when no resolution is needed — none of the scalar
   casts here need it).
2. `if (!option_nocasts) { pushOp(&typecast,op); pushType(dt); }` — Rust gates on
   `!self.options.nocasts`, pushes `tokens::TYPECAST`, then `push_cast_type`. The
   `TYPECAST` op-token (printc.rs:172: `"(", ")", 2, 62, false, Presurround, 0, 0`)
   is byte-identical to the C++ `PrintC::typecast` (printc.cc:36). `push_cast_type`
   renders the base type's declarator front+back as a TypeToken (the int->float
   cast's only form; the pointer/array declarator stack is a pre-existing,
   separately-tested seam).
3. `pushVn(op->getIn(0),op,mods);` — Rust recurses via `push_vn_ir(fd,arch,in0,op)`,
   the real RPN operand descent (implied-op recursion + explicit resolution).
4. `if (dt->isPointerToArray()) { if (checkAddressOfCast) { addressof } }`
   (printc.cc:472-478) — NOT ported; marked `// SEAM(printc opTypeCast
   pointer-to-array address-of arm)` with a precise note. This arm needs in0's
   read-facing high type and the TypePointer/TypeArray element walk
   (checkAddressOfCast, printc.cc:396), and provably never fires for the scalar
   `CPUI_CAST` / float-conversion casts this routes (their output is a scalar
   floatN/intN, not a pointer-to-array). Accepted as a faithful partial, but it
   causes NO observable divergence on the current corpus.

Adversarial confirmation (5 in-crate end-to-end tests, committed 3d655be) drives
the private `op_type_cast_ir` against real ops:
  - `(int8)operand` renders; the SAME op with output type float4 renders
    `(float4)...` and with `MyWeirdType` renders `(MyWeirdType)...` — proving the
    spelling is the output varnode's display-name verbatim, not opcode-keyed.
  - all three opcodes (CAST / FLOAT_FLOAT2FLOAT / FLOAT_TRUNC) render `(T)operand`,
    never the functional `OPC(args)`.
  - a lower-precedence operand (`INT_ADD`) is parenthesized: `(float8)(1 + 2)`
    (the typecast is a presurround prec-2 token; operand precedence/recursion is
    real, not a flat string concat).
  - `option_nocasts` suppresses the `(T)` and prints only the operand.

## No special-casing / no test weakening

- TypeCast table entries are EXACTLY the three unconditional-opTypeCast opcodes
  (printc.rs:874/875/877 = printc.hh:332/333/341). Opcodes that only conditionally
  call opTypeCast from hand-written overrides — opZext/opSext (printc.cc:809/822),
  opSubpiece (printc.cc:895) — correctly stay `Custom`. No over-routing.
- grep of `op_type_cast_ir`: no opcode literal, no type-name literal, no function
  name, no magic offset, no `0x` constant — the body is `nocasts` gate +
  out-varnode type lookup + in0 recurse.
- The two adapted tests are STRENGTHENED, not weakened:
  - `verify_w10_struct_corpus.rs:1033`: the deref regex was widened to ALSO match
    the new correct C-notation `(type *)` group; it still requires a real `*(...)`
    deref over `a0` (or `a0[..]`) AND still forbids a functional `LOAD(...,a0)`.
  - `verify_w10_callsite_args_adversarial.rs:501`: a naive "first `)`" arg parser
    (which would have truncated the now-correct `(int8)RSP + const` arg at the
    cast's own paren and masked the value) was replaced with a balanced-paren
    matcher; the three downstream assertions (exactly 1 arg, not raw RDI, must be
    the resolved stack value) are unchanged.

## Hunt list

- Signedness: n/a — no signed/unsigned comparisons in the added code. clean.
- Integer widths: n/a — no integer arithmetic added. clean.
- Wrapping: n/a — no arithmetic on uintb-lineage values. clean.
- Comparator totality: n/a — no comparators added. clean.
- Iteration-order provenance: n/a — no container iteration; no HashMap/HashSet. clean.
- Off-by-one / do-while / reverse iterators: n/a — no loops in the new fn. clean.
- Erase-while-iterating: n/a — no mutation-during-traversal. clean.
- Exception -> Result partial-state parity: n/a — opTypeCast throws nothing; the
  Rust `Option`/`if let` guards on a missing out/in0 silently no-op (no push),
  which is the safe degenerate (a well-formed cast op always has both). clean.

## Mechanical pass

- bare `as` casts in added lines: NONE.
- todo!/unimplemented!/HashMap/HashSet/sort_unstable in added lines: NONE.
- cargo clippy -p kuna-decomp --lib: clean (the 4 pre-existing warnings are in
  unrelated files typeop/tests.rs + double.rs, predate this change).
- Passing-set diff base(rust-port) -> branch: regressions = EMPTY; new = exactly
  the 5 verifier tests. boolless / readstruct / condconst_conn and every other
  previously-passing assertion still pass (they are in the preserved set).
- C++ oracle untouched (diff touches only `rust/`): 207/207 + 675/675, PARITY OK.

## Faithful-partial losses

The pointer-to-array `checkAddressOfCast` address-of arm (printc.cc:472-478) is
not ported. It is a documented SEAM that causes no observable divergence on the
current corpus (the routed scalar/float casts never produce a pointer-to-array
output). This is a faithful partial of a larger upstream cast/array layer, not a
weakening of the ported behavior; it is NOT a per-decompilation render loss today.
Recorded as LOSS-186 in losses.md.

findings: none (no correctness divergence found).

adversarial tests (committed 3d655be, src/printc/tests.rs::w10_printc_cast_render):
  - cast_renders_output_type_int8
  - cast_text_follows_output_type_not_opcode
  - all_three_typecast_opcodes_render_cast_notation
  - cast_parenthesizes_lower_precedence_operand
  - nocasts_suppresses_the_cast_keeps_operand

losses: LOSS-186 (faithful partial — opTypeCast pointer-to-array
        address-of arm; no observable divergence on the corpus).
