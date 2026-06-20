# Verifier review — w10-enum-render (Round 1)

**Verdict: ACCEPT**

Branch `rport/w10-enum-render` (`2f68b66`), merge-base `85acc75`. Independent verifier
(not the porter; no porter notes consulted). Diff scope: **only**
`rust/crates/kuna-decomp/src/printc.rs` (+79) and `printc/tests.rs` (+119). No other path
changed (verified `git diff --name-only 85acc75..rport/w10-enum-render`).

## What was ported

`PrintC::push_enum_constant_ir` — the enum arm of C++ `PrintC::pushConstant`
(printc.cc:1817-1833) which, for an `is_enum_type()` data-type, delegates to
`PrintC::pushEnumConstant` (printc.cc:1735-1756). The `get_matches` decomposition itself
(`TypeEnum::getMatches`/`TypePartialEnum::getMatches`, type.cc:1537-1586/2707-2713) was
ported in a prior wave; this diff is the render/RPN-push arm plus the `pushConstant`
dispatch (`if ct.is_enum_type()`).

## (1) FAITHFULNESS — byte-faithful to printc.cc:1735-1756

Line-by-line against C++ `pushEnumConstant`:

| C++ (printc.cc) | Rust (printc.rs) | Verdict |
|---|---|---|
| `getMatches(val,rep)` (1740) | `ct.get_matches(val)` → `EnumRepresentation` | ✓ (Err→raw-int fallback, unreachable for decoded enums) |
| `if rep.shiftAmount!=0 pushOp(&shift_right,op)` (1742) | `if rep.shift_amount!=0 push_op(SHIFT_RIGHT)` | ✓ |
| `if rep.complement pushOp(&bitwise_not,op)` (1744) | `if rep.complement push_op(BITWISE_NOT)` | ✓ |
| `for(i=size-1;i>0;--i) pushOp(&enum_cat,op)` (1746) | `for _ in 1..len() { push_op(ENUM_CAT) }` (=len-1) | ✓ |
| `for(i=0;i<size;++i) pushAtom(Atom(name[i],tag,const_color,op,vn,val))` (1748) | `for name in &match_name { push_atom(Atom::with_value(name,VarToken,const_color,op,vn,val)) }` | ✓ forward order |
| `if rep.shiftAmount!=0 push_integer(shiftAmount,4,false,tag,vn,op,0)` (1750) | `push_constant_ir_fmt(shift_amount,4,op,NONE)` | ✓ (sz=4, unsigned, fmt 0) |
| else `push_integer(val,ct->getSize(),false,tag,vn,op,ct->getDisplayFormat())` (1753) | else `push_constant_ir_fmt(val,ct.get_size(),op,ct.get_display_format())` | ✓ |

Token defs match upstream exactly (printc.cc:30/43/78):
`bitwise_not "~",1,62,unary_prefix`, `shift_right ">>",2,46,binary,1`,
`enum_cat "|",2,26,true,binary` — all identical in printc.rs:160/186/252.
`Atom::with_value` is a faithful transcription of the C++ 6-arg ctor (casetoken→intValue,
else→vn; printlanguage.hh:249-258). `display_format::NONE == 0` = C++ `displayFormat=0`.
`push_constant_ir_fmt`'s hardcoded `sign=false` matches the always-unsigned enum call
sites. The `len-1` cat-loop and forward-order atom emission reproduce the RPN push order
exactly.

`TagType::VarToken` hardcode is **correct**: the sole production caller is the
`pushVnExplicit` dispatch (printc.rs:3889 = C++ `tag=vartoken`). The other C++ caller
(`emitSwitchCase`, printc.cc:3305, `casetoken`) does **not** route through
`push_enum_constant_ir` in Rust — the Rust switch-case path (printc.rs:2267) renders
labels via plain `push_constant_ir`, a pre-existing port gap untouched here. So no
casetoken value ever reaches this arm; the hardcode is faithful to the actual call graph,
not a shortcut.

Enum-vs-float-vs-char dispatch ordering: an enum is TYPE_INT/TYPE_UINT (never
TYPE_FLOAT) and char-print/enum flags are mutually exclusive, so the FLOAT-then-enum order
is safe; char-print is an unrelated pre-existing gap.

## (2) NO SPECIAL-CASING — generic over the namemap

`grep` of the function body for hardcoded flag names / values / function names: the only
hits (`FLAG`, `0x20`) are in doc comments quoting the C++ example. The body is fully
generic over `rep.match_name` from `get_matches`. The committed test
`generic_over_namemap_no_hardcoding` proves it: the SAME value `0x5` through two unrelated
enums renders each enum's own names (`RED_HI|RED_LO` vs `BLUE_BIT2|BLUE_BIT0`).

## (3) REAL parity vs oracle B5 (enum.xml #3/#4/#5)

C++ oracle `decomp_test_dbg`: **207/207 unit + 675/675 datatest assertions, PARITY OK**
(includes enum.xml's 5 string-match assertions). Oracle render targets:
- #3 `(FLAG_20|FLAG_8|FLAG_4)` (0x2c) — Rust test `flag_or_decomposition_msb_first`
  asserts the live `emit_expr` render `render_enum(0x2c) == "FLAG_20|FLAG_8|FLAG_4"`
  (MSB-first, exact). Parens come from the parent `&`/`==` op, asserted bare here.
- #5 `(FLAG_100000|FLAG_800)` — same MSB-first OR decomposition pattern.
- #4 8-way `(HIGH_1000|...|HIGH_1)` — same many-flag forward-order OR.
- #2 `>> 0x20` — the partial-enum shift; `get_matches` partial path + the
  `push_constant_ir_fmt(shift_amount,4,…)` arm reproduce it (`0x20 = 4*8`).

The tests drive the REAL RPN/`EmitNoMarkup` back-end (`emit_expr`) and assert the rendered
C string, re-derived from `get_matches` + the C++ push order — not stub equality.

## (4) NO REGRESSION

- **`cargo test --workspace`** (independently run): **3613 passed / 0 failed / 37 ignored**.
- **Regressed-set EMPTY**: independently captured the passing set at merge-base `85acc75`
  (3608 passing, copied built `.sla` in) vs the branch (3613). `comm -23 base branch` =
  empty. `comm -13` = exactly the 5 new enum tests. Net delta = +5, all additive.
- **boolless / readstruct / condconst_conn byte-identical**: all present and passing in
  BOTH sets (`boolless_print_c_emits_structured_body`,
  `w10_mc_a3a_readstruct_pointer_return_byte_identical`,
  `w10_eob_condconst_conn_forms_andand_byte_identical`, …) → byte-identical by the
  empty-regression proof.
- **switch datatests byte-identical**: the diff does not touch the switch-case render path
  (printc.rs:2267, `push_constant_ir`); all switch tests pass identically on both branches.
- **No reserved file touched**: `git diff --name-only 85acc75..rport/w10-enum-render`
  matches none of coreaction_infertypes.rs, subflow.rs, funcdata_varnode.rs, variable.rs,
  coreaction_cleanup.rs, coreaction_casts.rs, dynamic.rs. Only printc.rs + tests.
- **C++ oracle 675/675 PARITY OK untouched** — no C++/specs/baseline.json changed; B0
  unchanged.
- **clippy clean**: `cargo clippy --lib -p kuna-decomp` and `cargo clippy --workspace`
  both exit 0, no warnings.

## (5) Adversarial tests — 5 committed (≥3 required)

`printc::tests::w10_enum_render::` — all 5 pass:
`flag_or_decomposition_msb_first`, `single_flag_no_cat`,
`numeric_remainder_falls_back_to_raw_integer` (asserts `0x10` raw fallback, empty
match_name, no flag names — confirms the else-branch), `complement_emits_tilde` (`~EIGHT`,
asserts `rep.complement`), `generic_over_namemap_no_hardcoding`. Each cross-checks the
data layer (`get_matches`) AND the live render, so a hardcode would fail the generic test.

## Notes (not blockers)

- The `get_matches` Err→raw-integer fallback in the new arm is an unreachable-in-practice
  safety net: a decoded enum co-sets `flags::enumtype` and `DatatypeKind::Enum`
  (dtype.rs:5244), so `is_enum_type()` true ⇒ `get_matches` returns `Ok`. The fallback
  degrades to the same raw-integer rendering as the C++ else-branch — no divergence.
- Enum-typed **switch labels** render as raw integers (Rust switch path bypasses
  `pushConstant`'s type dispatch). Pre-existing gap, unchanged by this diff, no oracle
  assertion exercises it. Tracked as a port-wide loss elsewhere, not introduced here.

No blockers. No in-scope losses.
