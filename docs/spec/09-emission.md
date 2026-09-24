# 09 — Emission

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p9_emit
```

This phase renders the finished decompilation: it inserts the explicit
cast/field-access operations a C compiler would need to *see* the recovered
types (§9.1), then walks the structured block tree of chapter 08 and the SSA
expression graph to build a token stream (§9.2), naming every value on the way
out (§9.3), folding in string literals and analysis comments (§9.4), and
choosing between pointer-arithmetic and array notation (§9.5). Everything here
is presentation: after `ActionSetCasts` the IR is only mutated by inserting
print-support ops (CAST, `PTRSUB #0`), never by changing computation. In the
registry (`decompiler/crates/kuna-decomp/phases.toml`) P9 carries the
sub-decisions `cast-policy`, `naming-policy`, `literal-format`,
`pointer-notation`, `condition-form`, `brace-form`, `warning-style`,
`type-definition-preamble`, `constant-address-global`, and `external-refinement` — plus, via the `presentcompare` group row, the P9 half of the P3-declared `comparison-canonicalization` decision
(the console/`kassert` assertion writer — an output *consumer* that writes P0
assertions for the next run, not an algorithm of this folder). One P9-registered
pass lives outside the folder: the (kuna, GH-558) comparison canonicalizer
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_compareform.rs
(ActionPresentCompareForm)`, group `presentcompare`, is described with its
folder in chapter [03](03-ssa-and-simplification.md). Option defaults, tiers,
and flip guidance for the kuna settable options named below live in the
generated catalog ([docs/options.md](../options.md)); the upstream console
knobs (`nocastprinting`, `integerformat`, `nullprinting`, `inplaceops`,
`maxlinewidth`, `indentincrement`) are surfaceTable rows in `phases.toml`, set
via the console `option` command, and are not part of the settable catalog.
The intentional default divergences are DIV-1/2/5/6/7 and the C-surface
normalization defaults (DIV-34 brace placement, DIV-35 NULL printing,
DIV-36 compound assignments, DIV-37 truthy conditions, DIV-38 single-statement
brace elision, DIV-39 inline warning slugs) in `docs/history.md`.

**Condition form (P9/`condition-form`, `option truthycond`).** In boolean
contexts — an if/while/for/ternary condition, or an operand of `&&`/`||`/`!`
— a comparison against zero carries no information beyond the value's own
truthiness, so the kuna default (DIV-37) renders `if (x != 0)` as `if (x)`
and `if (p == NULL)` as `if (!p)`. The printer threads a
`CONDITION_CONTEXT` mod bit from the condition push sites
(`printc.rs (PrintC::op_push_ir)` scopes it off across every
non-boolean-preserving operator, so a value use like `v = (x != 0)` never
rewrites), and `printc.rs (PrintC::op_binary_ir)` consumes it — after the
negate-token flip has settled which comparison prints — by eliding the one
eligible zero operand (`printc.rs (PrintC::truthy_other_operand)`: a plain
constant zero, directly or through one implied CAST, that is not
float-typed, enum-typed, or equate-named). The surviving operand keeps the
context bit, so stacked boolean comparisons collapse fully. `option
truthycond off` restores upstream Ghidra's explicit comparisons, exercised
by `tests/stages/kuna-cnorm-truthycond.xml`.

**Brace form (P9/`brace-form`, `option braceelide`).** A single-statement if
body renders braceless with the statement indented on the next line (kuna
default, DIV-38): `printc.rs (PrintC::emit_block_if)` consults
`printc.rs (PrintC::if_body_elides)` — the body must be a plain
single-statement `BlockCopy` leaf (exactly one op that the statement walk
would print, no label line, no comment positioned in the block), which also
rules out a nested `if` body so eliding can never capture a dangling else.
Multi-statement bodies, else arms, and loop/switch bodies always keep their
braces; the pre-existing `if (cond) goto L;` one-liner and the `else if`
collapse are unaffected. `option braceelide off` restores upstream Ghidra's
braced form, exercised by `tests/stages/kuna-cnorm-braceelide.xml`.

**(kuna) Decode-failure halt statements (`option decodehalt`, default `on`).**
A `CPUI_RETURN` the flow follower planted because the bytes would not decode
(chapter 02) is not a return, and printing it as one asserts something about the
program that the analysis never established. `printc.rs` renders the three
decode-failure halt causes as upstream `PrintC::opReturn`'s pseudo-calls —
`halt_baddata()`, `halt_unimplemented()`, `halt_missing()`
(`kuna_decodehalt.rs (halt_call_name)`) — and the P2 warnings land beside them
through the ordinary comment channel. `noreturn` halts keep the plain `return;`.
The elision below is suppressed for exactly those three causes while the option
is on: that statement is the truncation marker, not the source falling off the
end of a void body, and eliding it deletes the only thing saying the body is
incomplete — leaving its warning attached to whatever decoded before it. A
`noreturn` halt keeps whatever the elision already did with it, and `option
decodehalt off` restores both halves together.

**Void tail-return elision (P9/`brace-form`, `option voidtailreturn`, default
OFF).** kuna prints the function's final `CPUI_RETURN` unconditionally, so a void
function ends `... }` / `return;` / `}` — a statement the C source it was compiled
from does not have, because the source simply falls off the end of the body.
`printc.rs (PrintC::emit_function_body)` computes at most one elidable op per
function via `printc.rs (elidable_void_tail_return)` and
`printc.rs (PrintC::emit_basic_block_ops)` skips exactly that op. Four conditions
must all hold, and each has a named counterexample:

- the prototype returns void — a `return <value>;` is never redundant;
- the op is the tail of the LAST structured leaf, reached by descending only the
  containers that print no construct of their own (`Graph`, `Ls`), so a return
  nested inside an `if`/loop/switch arm is never touched;
- that leaf is not an unstructured goto target — bash `rl_echo_signal_char`
  prints `label_115e79:` directly above its trailing `return;`, and eliding the
  statement would leave a label with nothing after it, which is invalid C;
- exactly one structured leaf carries that RETURN op. `returndup` and `taildup`
  clone a shared epilogue by ALIASING one op across several leaves, so
  suppressing by identity would also delete genuine mid-body early returns; a
  unique owner makes the elision positional rather than identity-based.

A `BlockCopy` mirror is resolved through to the bblocks block it stands for
(`printc.rs (structured_leaf_tail)`) — the normal shape of a structured
function's trailing block, and the reason the direct `sblocks_basic_tail` lookup
is not sufficient. The motivation is measured and structural as well as
stylistic: pyjoern merges `FUNCTION_END` into its predecessor only when that
predecessor's sole successor is `FUNCTION_END`, so a source function whose tail
is an `if` or a loop has NO node there at all, while kuna's printed `return;`
re-materialises one — one extra node, one or two extra edges, and an
`is_exitpoint` role flip that on its own defeats the GED isomorphism test.

**Warning style (P9/`warning-style`, `option warnstyle`).** Analysis warnings
render as terse `// slug` end-of-line comments on the line they describe
(kuna default `inline`, DIV-39): `printc.rs (PrintC::emit_comment_group)`
maps each WARNING-type comment through the slug table
(`printc.rs (warning_slug)` — `no-return`, `branch-flip`, `return-dupe`,
`jump-as-call`, count-suffixed slugs like the header's `early-return x3` and
the `int3-pad x4` chapter 02 buffers for a decoded `int3`, where the count is
the pad's length in bytes; an
unrecognized text keeps its full body behind a `warn:` marker) and collects
it; `printc.rs (PrintC::flush_eol_warnings)` appends the collected slugs as
one `// slug, slug` token at the owning line's last token — the statement
semicolon, the `if (cond)` header (braced, braceless, goto, and ternary
forms), the loop-header brace, and the function prototype for header
warnings. Non-warning comments (user comments, `dwarf_lines`) always keep
their banner-line form, and a body whose only pending comments are
inline-rendered warnings still qualifies for `braceelide`. `option
warnstyle banner` restores upstream Ghidra's full
`/* WARNING: ... */` lines, exercised by
`tests/stages/kuna-cnorm-warnstyle.xml`.

## 9.1 Casts

**The pass.** `ActionSetCasts` (registry row P9/`cast-policy`, group `casts`)
is scheduled near the very end of the pass tree — after `ActionNameVars` and
before `ActionFinalStructure`
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`); its driver is
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_casts.rs
(Funcdata::action_set_casts)`. It walks basic blocks in order and the ops of
each block in sequence (a snapshot per block, so ops it inserts are never
revisited), skipping unprinted ops and existing CASTs. Per op, in a fixed
order: repair PTRADD/PTRSUB ops whose pointer type no longer fits (below),
give every unresolved union edge a last chance to resolve, cast the *inputs*
first (the output token may depend on them), then the output.

**The decision oracle.** The per-edge question — "does this conversion need a
visible token?" — is delegated to the language's cast strategy,
`decompiler/crates/kuna-decomp/src/p9_emit/cast.rs (CastStrategy)`, with
`cast.rs (CastStrategyC)` the C rules (`CastStrategyJava` exists for the
deferred Java back-end, §9.6). The strategy answers four kinds of question:
does an assignment between two data-types need a cast (`cast_standard`); is a
ZEXT/SEXT/SUBPIECE representable as a cast at all
(`is_zext_cast`/`is_sext_cast`/`is_subpiece_cast`); does C integer promotion
already imply a conversion (`int_promotion_type` and friends); and what type
does integer arithmetic naturally produce (`arithmetic_output_standard`). The
simple case of `cast_standard`: identical types (or typedefs of the same base)
never cast; a value coming from `void` always casts; a size change always
casts; and within same-size integers the int/uint/bool/unknown family is
mutually cast-free unless the operator *cares* about signedness (the
`care_uint_int` flag comparisons and pointer targets set). Pointer pairs are
peeled in parallel first — different word sizes or different address spaces
force a cast, `void *` never does, and once inside a pointer the signedness
care is always on.

**Integer promotion.** C silently promotes small integers, so many extensions
must *not* print. The strategy classifies a sub-`int` value's promotion as
unsigned, signed, either, or unknown (`cast.rs
(CastStrategyC::int_promotion_type)`, an IR walk that recurses through the
value's defining ops; the promotion width is the target's `sizeof(int)`,
`cast.rs (CastStrategyC::new)`). A ZEXT/SEXT whose input promotes compatibly
is *implied* and emits nothing (`is_extension_cast_implied`, consumed by the
printer's extension arms in §9.2); comparisons and the signedness-sensitive
div/rem/shift ops only accept a cast-free operand when both sides promote the
same way (`check_int_promotion_for_compare`/`check_int_promotion_for_extension`).
When a constant's printed form would change its arithmetic class, the strategy
instead flags the constant for suffixing — `mark_explicit_unsigned` (the `U`
suffix) and `mark_explicit_long_size` (the `L`/`LL` suffix) — which the
literal formatter reads back at §9.2's constant push.

**Casting an input.** `coreaction_casts.rs (Funcdata::cast_input)` asks the
per-opcode `getInputCast` surface (`coreaction_casts.rs (get_input_cast)`)
what type the operator requires at that slot: LOAD/STORE coerce the pointer to
match the moved value, EQUAL-class compares coerce both sides to the more
ordered of the two operand types, LESS-class compares and div/rem/shift gate
on promotion, PIECE/SUBPIECE/INSERT never cast, and everything else falls to
the default `cast_standard(input-type-local, read-facing-high-type)`. A `None`
answer means no cast — only the constant-suffix marking runs; the STORE's value
slot returns `None` for a `code` pointee under `codescalar` (chapter 05), which
is the printing half of that option. Otherwise the
machinery avoids stacking tokens: a value already produced by a CAST is
retyped or bypassed rather than double-cast; a constant is simply retyped;
and a pointer-to-struct being read as pointer-to-its-first-field inserts a
`PTRSUB(ptr, #0)` — rendered as `&ptr->field` / `ptr->field` — instead of a
cast (`coreaction_casts.rs (test_struct_offset0)`). Only when all of that
fails is a real `CPUI_CAST` op inserted before the reader, with an implied
unique output carrying the required type.

**Narrowed call arguments (kuna).** Dead-code analysis counts only the
possibly-nonzero bits of a call input as consumed, so when a caller
zero-extends a sub-`int` value into its argument register the subvariable-flow
rules (chapter [03](03-ssa-and-simplification.md)) trim the input to the narrow
value: `RDI = ZEXT(x:2)` becomes the 2-byte `x`. The p-code is right, but C
promotes a sub-`int` argument expression to `int` before the call, so the
default arm alone printed `sink(a0 * 3)` for a callee that reads the whole
register, passing a value the binary never passed.

That trim is the evidence that the binary zero-extended. When
`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs
(SubvariableFlow::try_call_pull)` narrows a call input whose non-zero mask lies
inside the logical mask (`decompiler/crates/kuna-decomp/src/p9_emit/kuna_truncarg.rs
(drops_only_zero_bits)`), `do_replacement` records the slot, its new size and
the call's input count on the call's `FuncCallSpecs`
(`kuna_truncarg.rs (note_trimmed_arg)`). A later trim of the same slot that
drops possibly-nonzero bits clears the record. A narrow input with no record
keeps its promoted form, because nothing says which extension the binary
used. Three kinds of input have no record: a narrow parameter forwarded
untouched, a sign-extension trimmed because the callee reads only its low
bytes, and an input that was narrow from the start. clang forwards a narrow
parameter as the register it received (`mov %dil,acc8; jmp sinki`), relying on
the caller's extension, so an unsigned cast there would pass 255 where the
binary passed -1.

The default arm consults `kuna_truncarg.rs (narrowed_arg_cast)` whenever
`cast_standard` asks for nothing. The input must be recorded, still at the
recorded size in a call with the recorded input count, narrower than
`sizeof(int)`, integer- or unknown-typed and not an enum, and in a slot no
declared prototype type-locks. For such an input it asks whether the
argument's promoted C value is already the zero-extension of the p-code value:
`int_promotion_type` answers for an expression, the natural extension of the
type the value prints as for a variable, a load or a cast, and a ZEXT always
is. The type it prints as is not always the one the cast strategy sees. A
one-byte value no pass ever typed is TYPE_UNKNOWN, which upstream's promotion
tables treat as unsigned, but `realtypes` spells it `char` in C output, and C
sign-extends a `char` wherever it is signed. A trimmed byte load (`movzbl`)
reads exactly that way, `sink(*(char *)(a0 + 3))`. The check therefore runs
through a view of the cast context (`coreaction_casts.rs
(FuncdataCastContext::with_unknown_byte_as)`) that reports a non-constant
one-byte TYPE_UNKNOWN as the signed `int1`, for the argument and for the
operands its promotion looks at. The view is on only when the per-function
`ArchContext::unknown_byte_is_char` says the printer spells such a byte
`char` (`realtypes` on, C output); with `realtypes` off the byte prints as
`xunknown1` and keeps the unsigned reading. When the promoted value is not the
zero-extension, the argument is cast to the unsigned integer of its own width,
as in `sink((uint2)(a0 * 3))` and `sink((uint1)*(char *)(a0 + 3))`. The cast
restores the zero-extension the trim removed. Two cases get no cast. The first
is an expression whose C value provably lies in `[0, 2^(8*size))`, such as
`(a0 != 2) + 6`, where promotion cannot change the value
(`kuna_truncarg.rs (c_range)`). The bound is taken over constants with a clear
sign bit, truth values, and `+ * & | ^` of those. The second is a slot whose
parameter type a declared prototype locks, because C converts the argument to
that narrow type itself. An argument that already prints as a truncating cast
(a SUBPIECE rendered `(char)v2`, every read of which is a call argument) is
retyped to the unsigned type rather than given a second cast
(`kuna_truncarg.rs (retype_truncation)`). Upstream Ghidra prints the promoted
form. Where the callee's own definition also declares the narrow parameter the
cast is redundant, though never wrong. Only an output language whose
`LangCaps::integer_promotion` is set gets the cast (C, read through
`ArchContext::int_promotion`). Rust arithmetic keeps its operands' width, so
`sink(a0 * 3)` with a `u16` `a0` already carries the truncation there. The rule
has no option because it only restores a conversion the binary performed.
Pinned by `tests/stages/kuna-truncarg.xml`.

One value-changing case is still open. A 32-bit argument trimmed out of a
64-bit register keeps its `int` type, so `sink64(a0 * 3)` with `int a0`
reaches a callee defined as `void sink64(unsigned long a0)` sign-extended,
where the binary's 32-bit write zero-extended it. It is left alone because
nearly every `int` argument on x86-64 is such a trim, and the cast would land
on all of them to fix the few whose callee reads the full register.

**Casts C already performs (kuna `castimplied`).** `is_extension_cast_implied`
hides an extension only when integer arithmetic, or a comparison against an
explicit operand of the same metatype, reads it; for every other reader, and for
any extension at the top of a statement, it answers no. So upstream prints
`memchr(v10,(int)v4,n)` where `memchr` declares an `int`, `v2 = (long)v1;` into
a `long v2`, `return (unsigned long)v1;` from an `unsigned long` function, and
`(long)(int)(unsigned int)(unsigned char)c` for one byte widened to 64 bits. C
performs each of those conversions itself: an argument is converted to its
parameter's type when a prototype is in scope, the right side of `=` to the type
of the left, a returned value to the return type, and a conversion between
integer types depends only on the value converted. With the option
`castimplied` on (the default; `off` restores upstream's rendering), the printer
leaves such a cast out
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_castimplied.rs (ImpliedCasts::drops)`,
asked from `printc.rs (PrintC::implied_cast_drops)` by the `CPUI_CAST` arm and
by both extension arms). The IR keeps every CAST op; only the token is omitted,
and the operand is pushed bare, as a `signedness`-dropped cast's is (§9.3).

Only integer conversions are considered: a `CPUI_CAST` between integer types,
or an INT_SEXT/INT_ZEXT the strategy renders as a cast. Consecutive conversions
form a chain whose top is the first one read by something that is not an
integer conversion. The top fixes the type of the whole chain, so a conversion
below it goes when it keeps its operand's value: every value of the operand's
printed C type lies in the conversion's target range (`kuna_castimplied.rs
(preserves)`). The operand's C type is taken as known only where the text states
it: a declared variable (the spelling the declaration line wrote), a conversion
that still prints (its target), a conversion this rule leaves out (its own
operand's type, which it preserved), a truncation printed as a cast, a load
`*(T *)p` through a pointer printed with that cast or declared `T *`, and a
subscript `((T *)p)[k]` whose base prints with that cast (the form `castarith`
below gives a load). An
arithmetic operand is not known, because C promotes `a - b` over two
`unsigned char`s to a negative `int` where the p-code wraps; neither is a
constant or a call. Under that rule `(long)(int)(unsigned int)(unsigned char)c`
becomes `(long)(unsigned char)c`: the `unsigned char` value passes through
`unsigned int` and `int` unchanged. A sign change or a narrowing below the top
always stays, as does anything under a pointer or float conversion.

The top itself goes when it is an identity (its operand already has exactly
that C type), or when it is a value-preserving widening whose value flows
straight into one of three places C converts at. The first is an argument of a
direct call whose parameter is type-locked (a declared prototype or a measured
libc signature), spelled exactly as the cast; a trial prototype, a varargs
position, a call carrying a per-call-site prototype override (a resolved format
string closes its varargs into ordinary parameters that way), and an argument
`truncarg` narrowed on purpose are all refused (`kuna_castimplied.rs
(trusted_param)`). The second is the right side of a `lhs = e;` statement whose
left side is a local or parameter the printer declared with exactly that
spelling. The arms of `c ? a : b` do not count: their type is the two arms'
common type, not the destination's. The third is `return` from a function whose
printed return type is that spelling. The declarations are recorded as the
printer writes them (the header's return type and parameters, each local's
declaration line), so the comparison is on the text C will see. One more
destination counts when `castsign` is on (§9.3): the left side of `lhs = e;`
whose declaration `signedness` re-signed, when the declared type is as wide as
the cast's target (`kuna_castimplied.rs (ImpliedCasts::assigns_to)`). A
conversion to an N-bit integer depends only on the value modulo 2^N (C11
6.3.1.3 for the unsigned case; gcc and clang define the signed case the same
way), so `lhs = e;` stores the bits `lhs = (T)e;` stored whatever `e`'s C type
is. Without it, re-declaring `unsigned long v18;` as `long v18;` would bring
back the `(unsigned long)` this rule dropped from `v18 = dat_dd86c << 3;`
because the spellings used to match. An implied
COPY prints as its operand and is looked through (`kuna_castimplied.rs
(through_copies)`). `truncarg`'s argument casts and `boolbyte`'s
`(bool)(unsigned char)` are never candidates, and nothing is dropped for an
output language without implicit integer conversions (Rust). The ported
`CastStrategyC` and its pinned decision matrix are unchanged. Pinned by
`tests/stages/kuna-castimplied.xml`, and by a compiled round trip
(`kuna-cli/tests/decompile_all_cli.rs`, `an_implied_cast_round_trips_through_the_printed_c`)
whose functions print the same values with the option off and on, under gcc and
clang.

**Casting an output.** `coreaction_casts.rs (Funcdata::cast_output)` compares
the *token* type the operator naturally produces — `coreaction_casts.rs
(get_output_token)`: COPY/PTRADD echo the input, arithmetic takes the
promoted meet of its inputs, shifts take the shiftee (bool→int), LOAD reads
through the pointer, PTRSUB/SUBPIECE/PIECE walk composite geometry, default is
the opcode's local output type — against the declared type of the output
HighVariable. An implied output is usually just retyped in place; an explicit
one gets a CAST (or a `PTRSUB #0`, by the same struct-offset-0 test) inserted
*after* the op, splitting the output into a fresh implied unique. A type-locked
implied value that is not feeding a RETURN forces the cast even when the
lattice would allow silence — the user's declared type must stay visible.
A LOAD's token, and the type a STORE asks of its value, is the pointee of the
address; when that pointee is a synthesized structure's pointer to its own
incomplete shell (`structsynth nest`, chapter 05), both take the pointer to the
completed structure instead (`coreaction_casts.rs (resolve_self_pointer)`), the
same type the lattice gave the value, so no cast is printed between a node and
the link it was loaded from.

**Union edges.** A value whose data-type still `needs_resolution()` (a union,
or a pointer to one) is resolved per read/write edge: `coreaction_casts.rs
(Funcdata::cast_resolve_union)` consults the per-function resolution cache and,
on a miss, runs the inference-time scorer once more (`resolve_in_flow`) — the
same last-chance the C++ takes. A resolved pointer edge materializes as a
`PTRSUB #0` carrying the chosen field; a resolved implied value is marked
`implied_field` so §9.2 renders `<def-expr>.field`. Two adjustment passes
(`cast_try_resolution_adjustment`, `cast_try_resolution_copy`) record a
compatible field choice instead of casting when one exists, so unions prefer
field syntax over cast syntax.

**Pointer arithmetic in pointer terms (kuna, `castarith`).** A pointer plus a
constant byte offset reaches this pass as a plain `INT_ADD` whenever chapter 05
could build no `PTRSUB` or `PTRADD` for it: a `void *` base has no field at the
offset and no element size, and a typed base whose element is not the access
width has neither either. The rules above price an `INT_ADD` as integer
arithmetic, so the default arm casts the pointer to `long` on the way in and
the output arm casts the sum back to a pointer on the way out, and a 4-byte
read at `+0xac` prints `*(unsigned int *)((long)a0 + 0xac)`, two casts for one
read. With `option castarith` on (the default, C output only) the driver hands
every such `INT_ADD` to `decompiler/crates/kuna-decomp/src/p9_emit/kuna_castarith.rs
(rewrite)` right after the PTRADD/PTRSUB repairs, so a sum that a repair has just
demoted is seen too, and before any input or output of the op is cast.

`kuna_castarith.rs (plan)` accepts it when one input is a non-constant whose
read-facing type is a pointer into a byte-addressed space, the other is a
constant, and the sum is used as a `T *`. When a lone LOAD reads through the
sum, or a lone STORE writes through it, `T` is the value it moves, which is the
type the LOAD/STORE input cast would otherwise have recast the pointer to
(`kuna_castarith.rs (access_type)`); the sum's own pointee takes precedence when
it is the same type or an integer of the same width, so the pointer keeps the
spelling the old output had. Any other use takes the sum's own pointee, and a
`void` pointee counts in bytes (`char`) only for a value that stays inside the
function (`kuna_castarith.rs (stays_in_function)`): a call argument or return
value carries its type to the whole-program prototype votes of chapter 04, which
must not learn a `char *` the analysis never derived. `T` must be a scalar or a
pointer whose size equals its aligned size. When the offset, read as a signed
constant, is a whole number of `T`s, the op becomes `PTRADD(base, #k,
#sizeof T)` with `k` typed as a signed pointer-sized integer, so a negative
offset is a negative index.

The base is cast to `T *` by a new implied `CPUI_CAST` inserted before the op,
with two exceptions. A variable whose own type is its read-facing type and whose
printed declaration (a parameter's prototype type or a local's declared type,
`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs (declared_variable_type)`,
the lookup the switch printer uses) points at `T` or at an integer of `T`'s width
is used bare (`kuna_castarith.rs (declared_as)`), so the declaration a reader
sees scales the index the same way; a float element never counts, because
converting it changes the bits. An implied CAST read only by this op is
retargeted to `T *`, the way the input machinery above retypes a cast rather
than stack a second one. An implied CAST that other ops read too is left alone,
and the new cast reads that cast's input instead, when the input is an integer
or a pointer (`kuna_castarith.rs (implied_cast_source)`): both casts only
reinterpret the same pointer-sized bits, so `*(uint1 *)(a0[1] + 10)`, whose
integer `a0[1]` the integer form converted to a pointer and straight back,
becomes `((uint1 *)a0[1])[10]` and not `((uint1 *)(unsigned long *)a0[1])[10]`.
The printer then renders the `PTRADD` like any other (§9.2, §9.5):
`((unsigned int *)a0)[0x2b]` under a dereference, and
`&((T *)p)[k]` as a value, or `(T *)p + k` with `arraynotation off`, so the one
spelling decision stays with that option.

The computed value is unchanged: `k * sizeof(T)` is the original offset, the
base is converted pointer to pointer with no integer in between (or, under a
shared cast, from the integer the integer form converted too), the element
is the access width, C's `sizeof` of the printed `T` is that same width, and
the printed index reads back as `k`. The `sizeof` condition is why an enum
element keeps the integer form (`kuna_castarith.rs (plan)`). kuna prints every
enum as a plain `enum`, whose size C leaves to the implementation (an `int`
under gcc and clang), while a packed enum, one compiled with `-fshort-enums`,
or a C++ `enum class : uint8_t` is 1 or 2 bytes in the binary, so
`((color *)p)[3]` would read 12 bytes past `p` instead of 3. For the same
reason the element must be a type the target's data model names at exactly its
width (`kuna_castarith.rs (c_sizeof_is_size)`): an integer of a width the model
has, a one-byte `bool`, a `float` or `double`, or a pointer of the model's
pointer size. A `long double`, whose `sizeof` is 16, 12 or 8 by target, and a
16-byte integer keep the integer form. The index condition is why a negative
index of 2^31 elements or more keeps the integer form too. C gives the literals `0x80000000` through
`0xffffffff` the type `unsigned int`, so `-0x80000000` is +2^31 and
`((long *)a0)[-0x80000000]` would index forward. The integer form's byte offset
is then at least 2^32 in magnitude for any element wider than a byte, a `long`
literal that negates correctly. A positive index keeps its value whatever type C
gives the literal. A one-byte element's offset in the unsigned range is misread
by the integer form as well; that is how the printer spells a negative constant,
and this rule leaves it as it was. The integer form stays wherever the printed C could
otherwise convert a value or cost a cast. An offset that is not a whole number of
elements (`*(unsigned int *)((long)a0 + 0x6a)`), a variable index, an aggregate,
enum or padded element, and a word-addressed space keep it. So does a sum read as an
integer (integer arithmetic on it, a store of it into an integer slot, or an
assignment to a variable declared as an integer, where the pointer form costs
more casts than the integer form), and an address several LOADs or
STOREs share, since only a lone access fixes the element. A STORE of a value
whose defining op the pass has not reached keeps it too, because that op's own
output cast may still retype the value (a float operation's result becomes
`float`) and an integer element would then convert it; it is accepted only when
the opcode fixes the kind, float arithmetic for a float element and integer
arithmetic for a non-`bool` integer one (`kuna_castarith.rs
(store_value_settled)`). Finally, a constant that names a global keeps it, and
so does an address-like constant beside an integer that was cast to a pointer:
`table[i]` compiles to the same `INT_ADD`, with the table as the constant and the
subscript as the "pointer" (`decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrfromuse.rs
(constant_may_be_global_base)`). A record base keeps `p->field` wherever the
offset lands on a field, because chapter 05 already made that access a `PTRSUB`.
An offset where the record has no field is still an `INT_ADD` here and is
converted like any other: a read past the record's extent
(`((unsigned int *)teb)[0x410]`, `0x1040` bytes into a TEB32 that ends before
it) or inside a field (`((int *)&a0->field_0x10)[1]`, the upper half of an
8-byte field). Upstream Ghidra prints the integer round trip. Pinned by
`tests/stages/kuna-castarith.xml` and by the compiled round trip
`a_pointer_plus_whole_elements_round_trips_through_the_printed_c` in
`decompiler/crates/kuna-cli/tests/decompile_all_cli.rs`.

The rewrite only removes ops: the integer form's two casts become one or none.
That can move one structuring decision. The tail
duplication passes of chapter 08 (`gotoreduce`, `taildup`, `crossjumprevert`)
run after this pass and bound the tail they copy by its op count, `CAST`s
included, so a tail that was just over the bound can fit after the rewrite, and
a `goto` to it becomes a duplicated `return` marked `// return-dupe`. The
duplicate computes what the goto reached. It is rare: when this shipped, 4 of
75,296 functions over 201 binaries changed their control flow, all of them this
way and all through `taildup`, and with `option taildup off` both arms printed
the same `goto`s.

The JSON surface loses a little provenance: a subscript is a surround token and
carries no op, as for every native subscript, so an instruction whose only
printed ops were the add and the access through it maps to no line in
`line_mappings` and drops out of its variable's `addresses`.

**Repairs and failure mode.** Late type propagation can invalidate the pointer
model a PTRADD/PTRSUB was built on; the driver demotes them back to raw
arithmetic (`cast_fixup_ptradd` undoes the scaling; `cast_fixup_ptrsub`
becomes COPY or INT_ADD) rather than print a field access into the wrong type.
The upstream LOAD/STORE pointer diagnostics (`checkPointerIssues`) are
warnings-only in C++, and in kuna the hook `coreaction_casts.rs
(Funcdata::cast_check_pointer_issues)` is a faithful no-op — a missing
diagnostic comment, never a changed expression. Apart from the narrowed
call arguments above, including the open 32-to-64-bit case, when the cast
strategy loses the failure is cosmetic: a spurious `(int4)` token or a missing
one —
the computation is unchanged, and the upstream console knob
`option nocastprinting` suppresses every cast token at print time without
touching the inserted ops.

## 9.2 PrintC

**Three layers.** The C back-end is a stack of three components in this
folder: the token emitter `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs
(PrintC)` (the `c-language` capability, the registered default), the RPN
expression driver it embeds (the `PrintLanguage` machinery — its pure data
model and decision function live in
`decompiler/crates/kuna-decomp/src/p9_emit/printlanguage.rs`, the driving
methods in `printc.rs`), and the low-level emitters in
`decompiler/crates/kuna-decomp/src/p9_emit/prettyprint.rs`: a line-breaking
`EmitPrettyPrint` wrapping either the plain-text `EmitNoMarkup` (the
byte-exact datatest path) or the XML `EmitMarkup` (the Ghidra-client path).
The whole-document entry is `printc.rs (PrintC::doc_function_full)`, driven by
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs` after analysis
completes.

**The document walk.** `printc.rs (PrintC::emit_function_document)` emits, in
order: the function's header warning comments (§9.4), the prototype (return
type from the recovered proto, else `void`; parameters with their declared or
default names; `, ...` for varargs), the open brace, one declaration line per
named local (§9.3), and then the body — a recursive walk of the structured
block tree S8 produced, `printc.rs (PrintC::emit_block_graph)` dispatching each
node by block type: basic/copy blocks emit their statement list; condition
nodes glue two clauses with `&&`/`||` inside parens; if-nodes print
`if (cond)` with a *pending-brace* mechanism so an else-clause consisting of a
single if collapses to `else if` — unless a goto label or comment forces the
brace; while-do nodes render `while (cond)`, or a `for (init; cond; iter)`
header when the loop carries the recovered initialize/iterate statements
(`printc.rs (PrintC::emit_for_loop)`); do-while and infinite loops, switches
with their case labels (`printc.rs (PrintC::emit_block_switch)`), goto blocks,
and — when the S8 `iteregion` pass marked an assignment diamond — the ternary
render `dest = cond ? a : b` (`printc.rs (PrintC::emit_block_if_ite)`), or —
when the S8 `iteboolean` pass marked a short-circuit `0`/`1` select — the
boolean-assignment render `dest = ( cond );` / `dest = !( cond );`
(`printc.rs (PrintC::emit_block_if_bool)`; checked first, so the more specific
form wins when both marks are present). Both re-derive their S8 match from the
addl-flag on the condition's `CBRANCH` and emit the condition through the same
`ONLY_BRANCH` renderer the `if (...)` header uses, so the condition's
parenthesization, short-circuiting and any comma-expression side effects are
identical to the `if` form they replace.

Every non-default switch arm emits its recovered numeric label even when the
arm has no p-code op available as a token-markup anchor. Such labels use the
same switch-width, signedness, and integer-format rules as op-backed labels,
but are emitted as plain syntax with no fabricated `opref`; `default:` remains
an unvalued label.

**Pending-brace ownership.** The `else if` collapse is a *lazy* brace. An
if-node that is itself the else-clause of its parent registers a brace with the
emitter (`printc.rs (PrintC::emit_block_if)`); the brace opens only if
something forces a line break before that clause prints its own `if (` header —
a statement in the clause's condition block, a goto label, or a comment. If
nothing does, the frame cancels its own registration and the header prints on
the `else` line, giving `else if`. The cancel decision belongs to the
*registering frame*, not to whoever happens to be printing when the emitter's
shared slot is non-empty: a clause's condition block can itself lead with a
whole nested if-statement (S8 folds a run of sibling guards into one `BlockIf`
whose condition component is a `BlockList` of the earlier guards), and that
nested frame must let the ancestor's brace fire instead of consuming it. This
mirrors upstream's pointer-identity test (`emit->hasPendingPrint(&pendingBrace)`
against the frame's own object) and is not cosmetic: a nested frame that
cancels the ancestor's brace renders *itself* as the `else if` and leaves the
real clause's `if` header on a fresh line at the parent's indent, so that
clause's body executes on the then-path too. It is reachable whenever a
statement-carrying clause lands in the else slot — in practice after a
§8.1 `branchflip` arm swap. A debug-build assertion in
`printc.rs (PrintC::emit_block_if)` requires every registered brace to be
resolved by its own frame, either fired or self-cancelled; the shape is pinned
end-to-end by `tests/stages/ghdec-branchflip-armswap.xml`.

**The declined-structure shell.** If the structured tree is *absent* (S8
produced no `sblocks`), the printer does not emit a flat op listing: it keeps
the brace-matched prototype shell and plants a single comment in the body
(`printc.rs (PrintC::emit_function_document)`). The failure mode is
deliberately loud and syntactically valid, so batch consumers (`kuna
decompile-all --json`) get a parseable function with an explicit tombstone
rather than pseudo-C garbage. The comment distinguishes the two ways a
function can arrive here, because they call for different investigations: when
the drive recorded *why* the pipeline aborted for this function
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::set_kuna_pipeline_failure)` — chapter [00](00-overview.md) §0.2), the tombstone is
`/* WARNING: decompilation failed: <reason> */`, naming the recoverable error
verbatim; otherwise the pipeline genuinely ran and structuring produced
nothing, and the tombstone is `/* WARNING: structured blocks unavailable
(structuring declined) */`. The reason text is flattened to one line and any
`*/` neutralized, so it can never break out of the comment.

**Expressions: the push/pop opcode walk.** A statement is one op tree:
`printc.rs (PrintC::emit_statement)` opens a statement group, and `printc.rs
(PrintC::emit_expression_ir)` pushes an assignment token plus the output's
symbol if the root op has one, then recurses via the per-opcode push dispatch
`printc.rs (PrintC::op_push_ir)` (the `PrintC::op*` overrides; the
opcode→token mapping is the data table `printc.rs (op_emit_kind)`). Each
operand is fetched by `printc.rs (PrintC::push_vn_ir)` with the simple rule of
the whole emitter: an **implied** Varnode expands in place — its defining op
is pushed recursively (threading the reading op down so the ZEXT/SEXT arms can
ask §9.1's is-the-extension-implied question); an **explicit** Varnode becomes
a leaf token (`printc.rs (PrintC::push_vn_explicit_ir)`) — a constant
(dispatched by read-facing metatype to the float, enum-flag decomposition,
character, string-pointer (§9.4), or integer formatter, the last honoring
per-symbol display formats, the `U`/`L`/`LL` suffix flags from §9.1, and
signedness from the type), or a named variable, including the partial-cover
walk that renders `name.field`, `name[index]`, a `(int4)name` truncation cast,
or the artificial `name._8_4_` member when a Varnode covers only part of its
mapped symbol (`printc.rs (PrintC::push_partial_symbol_ir)`).

**Which symbols enter the partial walk.** Upstream routes *every* partial cover
of a mapped Symbol through that walk — the walk itself decides, per type, what
token describes the access — and kuna does the same: STRUCT, UNION and ARRAY
symbols all enter it. The array case is the one worth stating, because an array
is the only composite whose member token can be chosen without looking at the
access size, and doing so is wrong. The walk's ARRAY arm applies upstream's
`TypeArray::getSubEntry` test — the access maps to element `off /
elementAlignSize` **only if** what remains of it after that division still fits
inside one element — so a one-byte read at offset 3 of a `char[16]` renders
`g[3]`, while an eight-byte read at offset 0 of the same symbol does not
describe any element and falls to the artificial member `g._0_8_`. That
distinction is not cosmetic: rendering the eight-byte access as `g[0]` names a
single `char`, so the emitted statement claims a width the program does not
use, and a reader (or a consumer diffing against a source build) cannot tell a
byte store from a word store. The rule is therefore that an array subscript is
emitted only where the subscript is the *whole* truth about the access, and the
sized `._<off>_<size>_` member carries every access that spans elements. The
same walk keeps descending afterwards, so an array of unions resolves past the
subscript into the cached field (`arr[3].ffield`).

**The whole-array cover** reaches the walk but not its ARRAY arm, because
upstream breaks out at the top of the walk whenever the request is the symbol
in full (offset 0, size equal to the symbol's) and leaves the caller to render
a bare name. For an array that break is the same size-blindness one level up:
kuna's caller then took its own whole-array `name[index]` branch, and a
sixteen-byte `movaps` transfer through a `char v30[16]` VM register bank
printed `v30[0] = v32[0];` — one byte named on each side of a sixteen-byte
copy — while the four-byte accesses to the same bank a few lines away printed
`v30._0_4_`. **(kuna) arraycoverwidth** (default **on**,
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_arraycoverwidth.rs
(spans_multiple_elements)`) suppresses that break for a TYPE_ARRAY whose
element stride is smaller than the access, so a full-width cover falls to the
same artificial `._<off>_<size>_` member a partial one gets and the two
spellings agree: `v30._0_16_ = v32._0_16_;`. The predicate is deliberately
narrow — a scalar, a struct, a union, and any access that fits inside one
element are all left with the upstream break, so `g[3]` and the bare
whole-symbol name for a non-array are unchanged. Turning the option off
restores the upstream break and with it the element-zero subscript.

This is what makes the 16-byte register-pair return legible. P5's type factory
has no integer primitive wider than `max_basetype_size`, so a
`CONCAT88`-shaped return value is typed `undefined1[16]` (upstream behavior,
§5), and P6 merges the two halves the callee writes into that one local. The
halves are then eight-byte writes into a byte array — exactly the access the
subscript cannot describe — and they render `v1._0_8_` / `v1._8_8_`. The
whole-container operand reads `return v1._0_16_ << 0x40;` under
`arraycoverwidth`, which states the width but is still not compilable C: it
shifts a member of an array. What this does **not** fix is that container type
itself. Recovering it needs either a scalar wide-integer type or a mid-end fold
of the `CONCAT88(0,x) << 64` idiom; neither is done here.

**Leaves with no Symbol.** Not every leaf has one. When no mapped symbol covers
the storage the leaf falls through to the upstream `pushUnnamedLocation`
naming, `printc.rs (kuna_unnamed_location_name)`: the register name covering
`(address, size)` if the translator has one, else the angr-style `dat_<addr>`
for a data space (§9.3), else the capitalized `Space<hex>` form —
`Stack00000008`, `Unique00001a80`. These name the *storage*, not a variable,
and are deliberately never declared: they are extern-like markers that a value
lives somewhere the analysis never resolved to a variable, exactly as upstream's
`stack0x00000008` is (kuna capitalizes the space and drops the `0x` so the
token is at least a legal C identifier).

One unnamed high-variable case is deliberately canonicalized before that
choice. If any member is the unaffected input stack pointer, every member uses
that member's storage-form leaf (`Register0000000000000000` on x86-64), with
the translator's register-name shortcut disabled. Copy propagation can merge
the input stack pointer with register and unique-space members; printing each
member's own address would otherwise make one value appear under unrelated,
undeclared names such as `RAX` and `Unique10000064`. This spelling change does
not synthesize the definition that may be absent after the merge.

The same leaf serves the **spacebase** arm of `printc.rs
(PrintC::op_ptrsub_ir)`. A `PTRSUB(sp, off)` is a reference into the stack (or
global) frame; P6 binds a Symbol to the offset constant whenever the recovered
frame layout has one, and the arm then renders `&local_10` / `&myval.b`
through the partial-symbol walk above. When P6 bound nothing — the frame's
spacebase could not be tracked to a constant, so every reference stays relative
to the *entry* stack pointer and the offsets land outside the mapped frame,
which is what an `alloca`/`_chkstk` stack probe does — the reference still
names real storage, and it renders `&Stack00000008` through the same
unnamed-location leaf (`printc.rs (PrintC::push_spacebase_unnamed_ir)`, whose
address comes from `printc.rs (spacebase_unnamed_address)`, the C++
`TypeSpacebase::getAddress`). What that arm must not do is fall back to the
*functional* render `PTRSUB(ESP, 8)`, kuna's behavior before this leaf existed:
`PTRSUB` is an internal p-code operator and `ESP` a raw machine register, and
emitting either makes the whole function something no C parser accepts
(`tests/stages/ghdec-spacebase-unnamed.xml`, DIV-46).

The structure arm has the same failure through a different door. C++
`opPtrsub` reads the pointer type of the *variable* the base belongs to
(`getHighTypeReadFacing`), and the kuna printer reads the base Varnode's own
type, which is the same thing until one member of a merged variable carries a
narrower pointer than the rest. A global assigned a `struct_N *` and also read
back into a loop over `unsigned int *` is such a variable: `ActionSetCasts`
inserted `PTRSUB(dat_263f0, #0)` against the variable's `struct_N *`, and the
member feeding it reads `unsigned int *`, whose pointee is no structure, so the
op fell to the functional render and printed `v4 = PTRSUB(dat_263f0,0);`.
`printc.rs (ptrsub_resolves)` asks whether the member's own type can name a
member at all (a pointer to a structure, union, array or spacebase, or a
relative pointer), and only when it cannot does the arm take the variable's
type instead (`printc.rs (high_pointer_type)`: a mapped symbol's type, else
the merged variable's type representative), which prints
`v4 = &dat_263f0->field_0x0;`. Every output this changes was the functional
render before; on `main` one `tar` O0 function printed it
(`a0->field_0x8 = PTRSUB(dat_9f6a8,0);`).

**Precedence without an AST.** Operators and leaves are not buffered into a
tree; they stream through a reverse-polish stack. `printc.rs (PrintC::push_op)`
pushes an operator's static token — the singleton table `printc.rs (tokens)`
carries each C operator's precedence, associativity, arity stage, spacing, and
its negated complement — and *at push time* decides parenthesization by the
pure predicate `printlanguage.rs (parentheses)`: compare the enclosing token's
precedence/associativity/type against the incoming one, with special stages
for pre/post-surround tokens (calls, subscripts, casts) and the (kuna, DIV-1,
GH-2786) rule that adjacent identical `-`/`+` prefix tokens always
parenthesize so they cannot merge into `--`/`++`.

Two of those tokens are not the ones integer arithmetic uses. `*` and `+` are
associative over the integers, so an operand built by the same token needs no
parentheses on either side and a chain of them flattens to `a * b * c`. IEEE-754
arithmetic is not associative: `a * (b * c)` rounds at different points than
`(a * b) * c` and can differ in the last bit, so flattening a float chain emits C
that computes a different value than the p-code it came from. `CPUI_FLOAT_MULT`
and `CPUI_FLOAT_ADD` therefore push their own tokens, `printc.rs
(tokens::FLOAT_MULTIPLY)` and `printc.rs (tokens::FLOAT_PLUS)`, spelled and
ranked exactly like the integer pair but carrying `left_to_right_only` instead of
`associative`. That flag is the third answer `printlanguage.rs (parentheses)` can
give a same-token operand: `associative` leaves both operands bare, a plain
non-associative token (`-`, `/`) parenthesizes both, and `left_to_right_only`
leaves the first bare and parenthesizes the second. It is the exact rule, because
C already groups `*` and `+` left-to-right: a left operand re-parses into the
tree it came from, and only a right operand needs the parentheses to survive the
round trip. So the two orderings a compiler can emit stay distinguishable in the
output -- `a * b * c` for the left-grouped one and `a * (b * c)` for the
right-grouped one -- where before they printed identically
(`tests/stages/kuna-floatgrouping.xml`). Float subtraction and division already
reached the right answer through the non-associative integer tokens they share.

`printc.rs (PrintC::push_atom)` emits a leaf and then unwinds every operator whose
operand count is now satisfied (`emit_op` prints each operator's text at the
right visit stage — between operands for binary tokens, at open/close for
surrounds). Contextual rendering flows through a modifier word saved and
restored around each descent (`printlanguage.rs (PrintContext)`): e.g. a LOAD
feeding a STORE address prints `*ptr` or hides the dereference
(`print_load_value`/`print_store_value`), a negated condition flips a
comparison token to its complement instead of printing `!`.

**The pretty-printer.** All of the above emits *logical* tokens; line breaks
are chosen by `prettyprint.rs (EmitPrettyPrint)`, an Oppen-style streaming
formatter transcribed verbatim because its breaks are part of the byte-exact
output: tokens queue in a circular buffer (initial capacity 300, grown by 200
with reference fixup) while a scan pass computes each open group's size — held
negative until the group's close commits it — and `advanceleft` flushes tokens
whose size is final. A forced newline carries the `999999` "won't fit"
sentinel as its space cost so it always breaks; an ordinary break token whose
content no longer fits either indents to the group's saved column or, when
breaking would recover fewer than 10 characters, stays on the line;
overflow permanently raises inner indents to guarantee at least half a line of
working space (`prettyprint.rs (EmitPrettyPrint::overflow)`), and inside a
comment every forced break re-emits the comment fill prefix. Defaults: 100
columns (`option maxlinewidth`), indent step 2 (`option indentincrement`),
comment indent 20; brace placement per construct via the four `braceformat`
fields of `printc.rs (PrintCOptions)`: if/loop/switch braces sit on the same
line as their construct, and a function's brace sits directly under its
prototype (kuna DIV-34 — upstream's `skip_line` default leaves a blank line
between the prototype and `{`; `option braceformat function skip` restores
it, exercised by `tests/stages/kuna-cnorm-protogap.xml`).

**Position maps.** Every token can carry a resolved back-reference,
`prettyprint.rs (MarkupRef)`: an op reference (the op's `getTime`, the same id
the function's `<ast>` encoding writes as `<seqnum uniq>`) and a Varnode
reference (`getCreateIndex`, the `<addr ref>` id). On the plain-text path no
reference is even computed — output is byte-identical to a markup-less build —
but under `EmitMarkup` (selected by `printc.rs (PrintC::set_markup)`, the
ghidra-mode front-end) `<variable>` elements carry `varref`/`opref` (declarations add `symref`) and `<op>` elements carry `opref`; plain `<syntax>` elements carry only color/content that resolve against the AST by
construction, which is how the Ghidra client maps a clicked token back to
p-code, and how statement groups map to addresses.

**Literal format.** The remaining P9/`literal-format` knobs all act at the
constant/type-name chokepoints of this walk: `option integerformat`
(hex/dec/best — "best" scores which base makes the constant's digit pattern
most natural, `printlanguage.rs (most_natural_base)`), `option nullprinting`
(the `NULL` token for pointer zeros — kuna DIV-35 flips it default-ON, so a
null pointer renders `NULL` where upstream renders `(type *)0x0`; `option
nullprinting off` restores the casted form, exercised by
`tests/stages/kuna-cnorm-nullprint.xml`), `option inplaceops` (kuna DIV-36
default-ON with the `emitInplaceOp` consumer ported: a standalone statement
`out = out OP y` whose first input is the same HighVariable as the output
renders as the compound assignment `out OP= y` for the ten integer
operators, and a negative signed INT_ADD addend folds to `out -= c`;
comma contexts — for-loop headers and condition-block side effects — keep
the spelled-out upstream form, so `for (...; i = i + 1)` is unchanged;
`option inplaceops off` restores everything, exercised by
`tests/stages/kuna-cnorm-compoundassign.xml`), and
the (kuna, DIV-6) `realtypes` relabel: residual `TYPE_UNKNOWN` values render
as size-correct real C types (`char`/`unsigned short`/`unsigned
int`/`unsigned long`) at the declarator/cast chokepoints, without touching
the actual data-type lattice — `option realtypes off` restores
`undefined<N>`. The **same** size table applies to a `TYPE_UNKNOWN`
*pointee*, so `undefined8 *` reads `unsigned long *` and `undefined4 *`
reads `unsigned int *`: the relabel is presentation only, and the index and
cast expressions the walk builds elsewhere are still scaled by the original
pointee size, so a declaration that shrank its pointee would contradict its
own body (`void *a3` alongside `a3[1]` meaning byte offset 8 — not
compilable C, and a store cast down to `*(void *)` loses its width
entirely). `void` is therefore only the **fallback** under a pointer, for
the residual sizes with no natural single C type (0, 3, 5, 6, 7, …); as a
scalar those sizes keep `undefined<N>` (DIV-48,
`printc.rs (realtype_unknown_base)`, exercised by
`tests/stages/ghdec-realtypes-pointee.xml`). A genuine `TYPE_VOID` pointee
is not a residual unknown and never enters the relabel, so the opaque
`void *` of `free`/`malloc`/`memcpy` is unaffected. Per-symbol format assertions
(`map convert`, `force datatype`) override the global format at the same
point ([docs/options.md](../options.md)).

**Valid C type names** (kuna, DIV-75, `option ctypes`). `realtypes` covers only
residual `TYPE_UNKNOWN`, and the *named* core types beside it are not C at all:
kuna interns them as `uint1`/`int4`/`float8`/`float10`/`code`, a verbatim port of
upstream's no-`<coretypes>` fallback branch, which the real Ghidra application
never takes because its Java side supplies its own names over the wire. That
split is directly observable — one function declares `unsigned int v3;` (relabelled)
next to `int4 v1;` (not) — and it is why the emitted C does not compile.
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_ctypes.rs (core_type_spelling)`
extends the same one chokepoint to every core type: the type's *size* is matched
against the target's own declared widths, in declaration order, first hit wins
(the port of Ghidra's `DataOrganizationImpl.getIntegerCTypeApproximation`).
Declaration order is what makes it per-architecture rather than a guess: under
LP64 both `long` and `long long` are 8 bytes and an 8-byte integer must read
`long`, while under ILP32 and LLP64 `long` is 4 and the same size lands on
`long long`. The widths come from the compiler spec (chapter
[05](05-types.md) §5.1); the same size therefore renders `unsigned long` on
x86-64 System V and `unsigned long long` on i386, from one table.

Three cases resist an exact answer, and each is decided rather than left to fall
out of the table. A 1-byte integer is `signed char`/`unsigned char`, never bare
`char` — its signedness is implementation-defined, and kuna reserves the `char`
core type for text. `code` is Ghidra's pseudo-type for a function body and only
ever reaches the output as `code *`, which becomes `void *`. And floating point
is the one place an approximation is unavoidable: an exact width match wins, but
a width above `double` with no exact match spells `long double`, which is how the
x87 `float10` is reached. No target has a 10-byte `sizeof` — the x86 cspecs
record 10 as the *value* width and annotate the storage in a comment — so that
spelling is an approximation of storage, deliberately the same one the recompile
prelude already makes, since the emitted `.c` and `.h` must not disagree.

Integer widths with no C type at all (3, 5, 6, 7, and 16-byte integers) keep
their `undefined<N>` form. They are **not** widened: `(undefined3)x` is a 24-bit
truncation and `(unsigned int)x` is not, so rounding up would change what the
emitted code means.

The rename is presentation only — the interned core types keep their names,
because a core type's id is `hash_name(name)`, Ghidra-style identifiers are
derived from the first character of the type's name (`float8` is what makes
`fVar1`), and the console's C-type parser resolves base types solely through
`TypeFactory::find_by_name`, which the corpus feeds `int4`/`float8` from 269
script lines. The shipped catalog default is `off`, which is what the XML
parity corpora run at (42 datatest assertions pin the Ghidra spellings); the
`aggressive` preset turns it on, and `auto` selects `aggressive` under 500 KiB,
so valid C is the rendering every real-binary surface gets. Exercised by
`tests/stages/kuna-ctypes.xml` and the per-architecture CLI gate
`ctypes_per_arch`.

## 9.3 Naming

**(angr) namestyle — the policy.** The master toggle `option namestyle
angr|ghidra` (default `angr` since DIV-5; live flag `name_style_angr` set in
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`) re-skins every *default* (generated) name; user
and recovered names are never touched. The policy helpers live in
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_naming.rs` with the pure
address renderers in `decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
(kuna_global_data_name, kuna_function_name, kuna_label_name, kuna_arg_name)`.
Under the angr scheme: locals and decompiler temporaries are `v1`, `v2`, … —
**sequential per function**, not SSA-subscripted (one name per merged
HighVariable, exactly one counter); parameters with no recovered name are
`a0`, `a1`, … by signature slot; global data reads `dat_<addr>` (lowercase
bare hex), unnamed callees `sub_<addr>`, goto targets `label_<addr>`
(`printc.rs (PrintC::block_label_name)`, from the target block's entry
address so goto and label always agree). Under `ghidra` the upstream scheme
returns: `param_N`, type-prefixed `iVar1`/`uVar2`-style locals
(`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(kuna_default_local_name)`), `func_`/`code_` addresses. Both schemes are
recognized as "generated" by `kuna_naming.rs (kuna_is_generated_name)` so
cross-function name recommendation never propagates a default name.

(kuna) **The ghidra-mode third style (Phase 3, DIV-77).** The ghidra-mode
process sets a separate `name_style_ghidra` flag alongside (not instead of)
the angr default; the resolver `Architecture::kuna_name_style` /
`ArchContext::kuna_name_style` gives it precedence at exactly the
ADDRESS-DERIVED fallback sites — an unresolved callee prints `FUN_%08x`
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (fspec_printed_name)`),
an unnamed global `DAT_%08x` (`printc.rs (kuna_global_data_name)`), a goto
target `LAB_%08x` (`printc.rs (PrintC::block_label_name)`), the renderers in
`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
(ghidra_function_name, ghidra_global_data_name, ghidra_label_name)` — because
the Java side's dynamic-name heuristics key on those spellings
(`isDynamicSymbolName`). Local/parameter naming keeps the angr scheme (the
only ported local-naming pass). Never set on the standalone path, so both
existing styles are byte-identical.

**Where names bind vs. where they render.** The *assignment* is a P6 pass —
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionNameVars)` binds one name per HighVariable (symbol-derived where a
mapped/global symbol exists, e.g. the DIV-24 DWARF data-global names; the
sequential default otherwise) — and P9 only *consumes* the binding: the leaf
push of §9.2 renders every member of a HighVariable through the one bound
name, which is also what keeps a register/global copy-shadow merge reading as
a single variable. Recovered prototype inputs cross that boundary through
`Funcdata::link_proto_params`: an unnamed input is materialized as `aN` exactly
when `name_style_angr` is set, and otherwise as `param_N`, using the same
`kuna_materialized_param_name` helper as the signature and declaration
suppression paths. The test is deliberately the local-style flag rather than
`kuna_name_style`: GUI Ghidra mode sets its address-style override while leaving
`name_style_angr` on, so its parameters remain `aN`. An explicit nonempty
prototype name always wins. The angr scheme's second visible artifact is P9-owned: each
local declaration gains a trailing storage comment — `// rax` (register,
lowercased), `// stack - 0x10` (frame-relative signed offset), `// rdx:rax` (a
join value's register pieces), `// tmp` (an SSA temporary with no machine
home) — rendered by the rules of `kuna_naming.rs (kuna_storage_comment)` from
the declaration representative's storage; `option namestyle ghidra` emits no
storage comments. DIV-5 re-pinned 185 of the 675 upstream datatest assertions
to the angr names; `option namestyle ghidra` reproduces the pre-DIV-5 bytes.

**(kuna) declhightype — the declared type is the merged variable's own.**
Walking HighVariables rather than the symbol table also decides *which type* a
declaration states. Upstream writes `sym->getType()`, and a symbol's type is the
merged HighVariable's — the type of the member Varnode
`HighVariable::getTypeRepresentative` picks as the most specialized, which is
also the type `ActionSetCasts` queried through `getHighTypeReadFacing` when it
decided whether each use in the body needed a cast. Declaration and body
therefore agree by construction. kuna instead reads the type off its
*declaration representative* — the first address-tied member, else member zero —
which is a different Varnode, and for a high whose members disagree the
declaration then states a type no use was ever checked against. The witness is
an obfuscated x86-64 dispatch loop that merges one RAX across the whole
function: a `char *`-returning call feeds `MOVSX EAX,byte ptr [RAX]`, the cast
machinery sees a one-byte pointee and leaves the dereference uncast, and the
declaration says `unsigned long long *` — so the same `*v9` reads eight bytes
past the object and drops the sign extension. The option `declhightype`
(default on) declares the type representative's type, which is the
`sym->getType()` upstream would have written
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_declhightype.rs
(type_representative)`). It is presentation only: no cast, no statement and no
p-code moves. It applies only to a high the symbol table does not describe — a
mapped local keeps the declaration representative's type, because kuna does not
always type-lock the storage a symbol claims and the most specialized member of
a mapped local can be a transient the symbol never described. The declaration
representative still supplies the storage comment, the array adornment and the
composite mapped-symbol override, all of which outrank this. `option
declhightype off` restores the declaration representative's type.

**(kuna) signedness — an integer local is declared at the signedness its
operations ask for.** The declaration also states a *signedness*, and the type
lattice decides that in a way the body often contradicts. `Datatype::type_order`
ranks `SUB_UINT_PLAIN` (16) ahead of `SUB_INT_PLAIN` (17), so in
`get_local_type`'s "keep the most specific" fold one `uint` vote outranks every
`int` vote on the same Varnode, and no later phase re-decides (§5.1). On
optimized x86-64 those `uint` votes are structural rather than semantic: a
32-bit instruction zero-extends into its 64-bit register, and a strength-reduced
loop bound is masked and shifted — `INT_ZEXT`, `INT_AND`, `INT_RIGHT` and
`INT_XOR` each seed `TYPE_UINT` on both operands. A counter the source declared
`int` is therefore declared `unsigned int`, and `ActionSetCasts` writes the
signedness back at every comparison: `unsigned int v1;` … `if (0 <= (int)v1)` …
`while (v4 < (int)v1)`.

The option `signedness` (`upstream|auto|prefer-signed|prefer-unsigned`, default
`auto`) decides it at the declaration seam instead, from the operations the
body actually applies to the value
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_typeround.rs (plan)`). This is
TRex's *type rounding* (Bosamiya, Woo and Parno, USENIX Security 2025, §3.3.5):
signedness is not propagated, it is chosen once from the accumulated
size-tagged operation set, with a flag for the case nothing observed settles.
For each declared high of plain integer type the pass walks the operations
reachable from its *printed* (explicit) members and folds them into one verdict:
`INT_SLESS`, `INT_SLESSEQUAL`, `INT_SDIV`, `INT_SREM`, `INT_SRIGHT` (operand 0),
`INT_SEXT`, `INT_SCARRY` and `INT_SBORROW` demand signed; `INT_LESS`,
`INT_LESSEQUAL`, `INT_DIV`, `INT_REM`, `INT_RIGHT` (operand 0), `INT_ZEXT` and
`INT_CARRY` demand unsigned; a `CPUI_CAST` to a plain integer of the same width
is a demand for the type it casts to. `INT_LEFT` (operand 0) also pulls unsigned,
and it is the one entry in that list which is a stated **preference rather than a
soundness requirement**: `a << k` shifts the same bits into the same places under
either declaration, so it is not part of what makes a flip meaning-preserving. The rule is kept because a value the body ORs and shifts left
is a bit buffer, which C source spells unsigned, and because a signed `<<` is the
one otherwise-neutral operator `-fsanitize=undefined` reports; it is not kept
because `negative << k` is undefined, an argument that does not separate `<<`
from `+ - *`, whose signed overflow is undefined on the same footing and which
are neutral here. `auto` declares the value signed when every demand is signed and
unsigned when every demand is unsigned, and leaves it alone otherwise;
`prefer-signed` and `prefer-unsigned` additionally settle the no-demand case,
which is TRex's observation that C programmers write `int` when the signedness
does not matter. A *defining* operation votes but cannot veto, because a
definition converts at a fixed width and is bit-identical either way: the
divide-and-shift family (`INT_SDIV`, `INT_SREM`, `INT_SRIGHT` signed;
`INT_DIV`, `INT_REM`, `INT_RIGHT` unsigned) says what kind of number came out
and votes accordingly, while `INT_SEXT`, `INT_ZEXT` and `INT_2COMP` say nothing
at all — an extension describes the operand it widened, not the widened value,
so `int v = *p;` off an `unsigned char *` and `uintmax_t max = (long)(int)n;`
off a `movslq` are both ordinary C. Nothing else changes: no Varnode type, no
inference pass, no cast decision, and no prototype or symbol type — so the
`variables` JSON surface and every recovered signature are byte-identical, and
so is the whole output under `upstream`.

Which arm ships. `auto` is the default: it moves a declaration only when every
signedness-sensitive reader of the value agrees and nothing vetoes it, so the
only text it can change is that declaration and the casts the new declaration
makes into no-ops. That is what the evidence for the flip measures — 0 of 675
datatest assertions, PARITY OK on the stage corpus, and over twelve binaries
from nine projects at `-O0` and `-O2` — eight x86-64 and four 32-bit ARM
firmware images, 15,124 functions — 377 declaration flips and 443 cast tokens
dropped, none added, with no other hunk of any kind. `prefer-signed` is
the more faithful arm and stays opt-in: settling the unobserved values the way C
source does takes agreement with DWARF on 238 unstripped twins from 93.4% to
98.4% overall and from 71.9% to 94.8% at `-O2`, but it moves 7,081 declarations
image-wide, which is more than the datatest corpus can absorb as a default and
more than an unanimity argument covers. `prefer-unsigned` is the opposite
tie-break, kept as that arm's control and not recommended — 4 of its 40
DWARF-judged flips are right. `upstream` restores the declaration type inference
produced, byte for byte, which is what to select when diffing against upstream
Ghidra.

Soundness rests on the demand set covering every C construct whose meaning
depends on an operand's signedness — which is to say, on the *neutral* list being
signedness-independent. Everything the pass lets through is
signedness-independent at a fixed width — `+ - * & | ^ == !=`, unary `~` and
`-`, an assignment, a call argument, a `return`, a stored value, a truncation or
a concatenation — because two's-complement arithmetic and a same-width
conversion produce identical bits either way.

kuna's own cast strategy draws the same line, which corroborates the split
without proving it. The ordered comparisons, `/ %`, `>>` and the two extensions
pass `care_uint_int = true` to `CastStrategyC::cast_standard`
(`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_casts.rs
(get_input_cast)`), and every op on the neutral list above passes `false` or
takes no cast at all. The demand set is a **superset** of that
`care_uint_int = true` set with one op held out: `INT_SCARRY`, `INT_SBORROW`,
`INT_CARRY`, `CPUI_CAST` and `INT_LEFT` all fall through to the dispatch's
default arm, which passes `care_uint_int = false`, and are demanded on anyway;
`FLOAT_INT2FLOAT` is the converse — it passes `care_uint_int = true` whenever the
operand's nonzero mask has its top bit set, and the pass vetoes it outright
rather than demanding on it, which is strictly stronger. Demanding on the carry
intrinsics and the cast anyway is an over-constraint, not a gap — a carry
intrinsic names its own signedness and a same-width cast prints a token that
establishes the type — and only ever declines a flip that might otherwise have
been made; `INT_LEFT` is the stated preference described above.

"At a fixed width" is a precondition of that list, not a turn of phrase, and it
is the third rule. C's integer promotions convert every operand narrower than
`int` to `int` *before* any of those operators runs, and which extension is
performed is read off the declaration, so below the promotion width nothing is
neutral: `(short)-1 == -1` is true while `(unsigned short)0xffff == -1` is a
constant false, and `+ - * & | ^ << == !=` all diverge the same way. The pass
therefore declines any high whose declared type is narrower than the promotion
width, whatever the operations say — and the promotion width that decides this
is **the `int` of the compiler that reads kuna's output, not the target's**.
Emitted C is read and compiled where `int` is 4 bytes however small the target
cspec's `<data_organization><integer_size>` is; that value is 2 on `avr8gcc`,
`avr8egcc`, `TI_MSP430`, `TI_MSP430X`, `CR16`, `PIC24` and `x86-16`, and reading
the guard off it would let a 2-byte local be re-signed on those targets and turn
an emitted `if (v1 != -1)` into a constant-true test. The floor is therefore
`max(4, TypeFactory::get_size_of_int())`
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_typeround.rs
(MIN_PROMOTION_SIZE)`), pinned on x86-64 by `tests/stages/kuna-signedness.xml`
and on a 2-byte-`int` target by `tests/stages/kuna-signedness-int16.xml`. A width change
*between* two integers is an explicit `INT_SEXT`/`INT_ZEXT` in p-code, so a
mixed-width expression is constrained by the extension op rather than slipping
through as neutral.

Two further rules close the gap between "this operation" and "the printed
expression". First, the walk follows *implied* results: `v + 1 < 0` prints as
one expression, so the C type of `v + 1`, and therefore whether the comparison
is signed, follows `v`'s declaration; an operation that keeps carrying the
operand's type extends the walk to its readers, and one whose result is a
declared variable of its own ends it. Some of the *demanding* operations carry
the operand's type as well — `>>` and `<<` print at the shiftee's promoted type
and `/ %` at the usual-arithmetic-conversion type of their operands — so those
record their demand and continue the walk, which is what keeps a second operator
further out (`(v << 3) >> 2`) from disagreeing unseen, while a comparison, a
cast-printing extension and a carry intrinsic stop it because their printed
result establishes a type of its own. Second, anything unclassified vetoes the
variable outright — a `LOAD` or `STORE` address, a `PTRADD`/`PTRSUB` index
(where `base[v]` really does differ between a signed and an unsigned `v`, and no
cast is inserted), any `FLOAT_*` operation, an indirect branch, a type-locked
member, and a `CPUI_CAST` whose target is anything but a plain integer of the
same width (a pointer, a float, a `char`, an enum, a typedef, a different
width). A flip is written only for a high that **owns its declaration line**. The
printer's candidate list is filtered to sole-named entries before any collapse
runs (`retain_sole_named`), because every collapse that follows — the
composite-Symbol retain, `collapse_symbol_decls`, `DeclDedup` and the
`local_name_aliases` group suppression — pairs two candidates rendering the same
name. Without that filter a flip on one high of such a group could move the
collapse's own key, splitting a declaration that used to collapse, or re-sign
the single line a *non*-flipped sibling's uses read through; with it, which
declarations exist is exactly what it was under `upstream`.

When a high does flip, each `CPUI_CAST` on it whose target is the very
type now declared is a no-op token and is dropped, which is the visible half of
the change: `if (0 <= (int)v1)` becomes `if (0 <= v1)`. The drop is authorized
only by the declaration the emitter actually wrote, so a mapped-symbol, array or
collapse override that takes the declared type back also takes the cast back.

**(kuna) castsign — the same rule for frame locals and pointer indexes.**
`signedness` leaves three places alone where the program compares a value only
signed, and those account for a quarter of the unsigned-to-signed casts kuna
printed in the 2026-09-23 cast census: `unsigned long v15; // stack - 0x60` …
`if (0 <= (long)v15)`. First, every high mapped onto a Symbol is skipped, and a
stack local *is* mapped onto its frame Symbol, at offset 0. Second, a
`PTRADD` index and a conversion to a pointer veto the variable. Third, a body
local one of whose members is an input (a slot or register read before any
write) is skipped as if it were a prototype parameter. With the option
`castsign` on, the pass admits all three
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_castsign.rs`,
consulted from `kuna_typeround.rs (plan)`), and the declaration and the casts
it makes no-ops change exactly as they do for a register local, under two
further conditions described below.

A frame local is admitted only when its high covers its Symbol whole: offset 0,
a Symbol type (if any) that is a plain integer of the declaration's width, and
no other high in the function printing the same name
(`kuna_castsign.rs (whole_slot_local)`). The last condition is what keeps an
address-taken local out. `&v15` is a high of its own bound to the same name,
and re-declaring `v15` would change the C type of `&v15` under a pointer that
was typed for the old declaration. A split slot is kept out by the same test.
No high bound to a type-locked Symbol is re-declared, whichever relaxation
admitted it (`kuna_castsign.rs (symbol_type_locked)`). A `--assert type`, a
DWARF local and a type committed from Ghidra each lock the Symbol, and the
member varnodes the evidence walk checks for a lock do not carry it, so the
test reads the Symbol itself: the naming pass's bind, a dynamic Symbol, and
the Symbol containing each address-tied member. A declared `unsigned long`
keeps its declaration and its `(long)` casts even when every reader is signed.
The declaration line is the one the printer writes from that high. The frame
Symbol's type is not changed, so the `variables` JSON surface still reports the
type inference chose for the slot. A `PTRADD` index at pointer width and a
conversion to a pointer of the value's own width are neutral readers
(`kuna_castsign.rs (neutral_reader)`). `p[v]` computes `p + v * size` modulo
the address width under either declaration, and the signed reading is the one
that keeps a negative index inside C's defined behaviour. gcc and clang convert
a same-width integer to a pointer by keeping its bits. A narrower index is
widened by an explicit `INT_SEXT`/`INT_ZEXT` before the `PTRADD`, and that
extension still constrains the variable. The input-member case needs no test
of its own: the printer declares only body locals, and `retain_sole_named`
drops every planned high it does not declare, signature parameters included.

**Arithmetic keeps an admitted variable unsigned**
(`kuna_castsign.rs (can_overflow)`). Re-declaring `v` signed does not only
remove the casts at its comparisons: every `v + k`, `v - k`, `v * k`, `-v` and
`v << k` the body prints then computes in the signed type, where it used to
compute in the unsigned one. Unsigned arithmetic wraps, which is what the
binary does; signed arithmetic is undefined on overflow, and gcc folds on that
assumption even at `-O0`, clang at `-O2`. `0 <= (long)(v1 - 1)` over a
`long v1` is treated as `0 < v1`, which is false at `v1 == LONG_MIN` where the
binary's `v1 - 1` is `LONG_MAX`; `v1 = v1 - 1, 0 <= v1` over a `long v1` exits
at once for the same `v1` under `-O2`, where the binary loops. So a high this option
admits (a frame local, a body local with an input member, or a verdict a
neutral reader decided) is left alone when any of those operators reads it,
directly or through the expression its value is printed into (the walk follows
implied values, as above). The operand position matters only for `<<`, whose
count is not the value shifted. `& | ^ ~`, a comparison, a truncation, an
assignment, a call argument, a store and a `return` cannot overflow and keep
the variable admissible. A same-width conversion from the old declaration's
type into the new one (`v = strtoul(...)` into a `long v`) is defined by gcc
and clang as the same bits.

**A wide literal keeps an admitted variable unsigned**
(`kuna_castsign.rs (wide_literal)`). The printer may spell a constant whose top
bit is set as an unsuffixed decimal: `3000000000` for a 4-byte constant,
`10000000000000000000` for an 8-byte one. In C that literal's type is wider than
the declaration (`long`, and a 128-bit type in gcc), so `v == 3000000000`
converts `v` to it. An `unsigned int v` is zero-extended, which is the 32-bit
comparison the binary makes; an `int v` is sign-extended, and the comparison is
false for every `v`. A `|` or `^` with such a literal, or whose result meets one
(`(v | 0x80000) == 10000000000000000000`), goes wrong the same way, and `&` is
treated alike. So a high this option admits is
left alone when a `==`, `!=`, `&`, `|` or `^` meets it, or the expression its
value is printed into, with a constant whose top bit is set, either as the other
operand or inside the expression the other operand is printed as (through
`+ - * / % & | ^ ~ << >>`). A hex literal of that value has the unsigned type of
the declaration's width and would convert correctly, but whether the printer
chooses hex depends on the constant's display format and on `integerformat`, so
every such constant vetoes. On the castbench corpus that leaves two
declarations unsigned that would otherwise be re-signed, both beside hex
literals: `ls` O2 `(v17 & v25) != 0xffffffffffffffff` (five casts) and `tar` O0
`v12 != 0xffffffffffffffff`.

**A flip must remove a cast** (`kuna_castsign.rs (drops_printed_cast)`). A high
this option admits is re-declared only when one of its declared members is read
by a `CPUI_CAST` to exactly the new declaration's type that prints today. A cast
whose value is only a call argument, the right side of an assignment or a
`return` may already be left out by `castimplied` (§9.1), so it does not count.
Anywhere else in an expression it prints, including under another conversion
(`SEXT816((long)v34)`, `(long)(int)v8`, `p[(int)v16]`): a same-width sign change
never preserves the value, so `castimplied` keeps it there. A flip that removes
nothing would change the declaration for no printed benefit.

A decision any of these relaxations made may only declare the value **signed**.
The census counts 20 signed-to-unsigned casts on locals against 526 the other
way, and IDA's opposite bias costs it 587 casts, so a mirror rule would buy
little and could overshoot. A value some reader needs unsigned keeps its
declaration: an unsigned comparison, a logical shift right, unsigned `/` or `%`,
or a zero-extension. That includes a zero-extension into a 64-bit return or a
varargs slot, where the high bits are read. Two uses were measured as unsigned
evidence and rejected. One is an argument to a type-locked `size_t` parameter:
gnulib's `idx_t` is signed and is passed to `realloc` and `memcpy` everywhere,
and counting such arguments cost 46 casts over the census corpus while getting
`xpalloc`'s `nbytes` wrong. The other is a store through an unsigned pointer,
whose pointee type inference usually took from the stored value itself. A
same-width conversion computes the same bits in both cases, so neither changes
what the C computes. Re-declaring the left side of an assignment would lose a
widening `castimplied` dropped because the spellings matched, so that rule also
accepts a re-signed declaration of the cast's width (§9.1). The option acts on C
output only. Rust has no implicit integer conversions, so `let mut v1: i64;` would
not accept the `u64` that `strlen` returns (`printc.rs (emit_function_document)`
passes it only when the language's `integer_promotion` capability is set, the gate
`castimplied` uses). With the option off the output is byte-identical to
`signedness` alone. Pinned by
`tests/stages/kuna-castsign.xml` (pass 1 off, pass 2 on: a stack value
compared signed and used as `a0[v1]` is declared `int8`; an index the body
decrements, a logically shifted value and an address-taken stack value stay
unsigned in both passes; pass 3 maps the first value's slot `uint8`, and the
locked declaration keeps its casts), by a compiled round trip
(`kuna-cli/tests/decompile_all_cli.rs`,
`a_signed_only_variable_round_trips_through_the_printed_c`: the printed C is
built with gcc and clang at `-O0` and `-O2` and fed `2^63 - 1`, `2^63`,
`2^63 + 1` and the 32-bit edges, and every build must print what the fixture
binary prints; `castsign_eq_x86_64.c` compares values with `3000000000u` and
`10000000000000000000UL`, directly and after a `|`, and keeps them unsigned),
and by `castsign_leaves_a_locked_declaration_alone` in the
same file (a `--assert type` on a stack and a register local, and a DWARF local
declared `unsigned long`, stay unsigned with the option on).

What the rule reads is the compiler's instruction selection, not the source.
Where a compiler proved a `size_t` non-negative and emitted a signed compare on
it, `auto` declares that local signed and the source said unsigned. The walk
`auto` runs for a register local also treats `+ - *` as neutral, so a register
local it re-declares may be the operand of arithmetic the binary wraps: the
emitted C then computes what the binary computes when it is compiled with
`-fwrapv`, and may not without it at the edge of the range (gcc, and clang at
`-O2`, fold `(long)(v1 + 1) <= v1` over a `long v1` to false, where the binary
finds it true for `v1 == LONG_MAX`). Nor does that walk check wide literals: clang
`-O0` code that compares `ntohl(*p)` with `3000000000u` prints
`int v1; // eax` beside `if (v1 != 3000000000)`, false for every `v1`.
`castsign` does not extend either trade to the highs it admits (above). `docs/features/signedness/analysis.md`
carries the measured agreement rate against DWARF on unstripped binaries, which
is the number this option is judged on.

**(kuna) paramrefdecl — an `&parameter` reference is the parameter.** Because
the emitter walks HighVariables rather than the symbol table, it also has to
decide for itself which highs upstream would *not* have declared:
`PrintC::emitScopeVarDecls` is asked only for `no_category` symbols, so a
`function_parameter` symbol appears in the prototype and nowhere else. kuna
answers that by asking whether a `function_parameter` symbol contains the
storage of one of the high's member Varnodes — which is right for every high
that has storage of its own, and blind to a parameter whose address is the only
thing the body ever takes. A `&symbol` reference is carried as the offset
constant of a `PTRSUB(spacebase, off)` (§9.3), bound by
`ActionNameVars::linkSpacebaseSymbol` to the symbol owning the referenced frame
slot; when that slot is an incoming stack parameter the body never reads as a
value, every Varnode of the reference high lives in the constant space, the
storage test finds no parameter, and the high is declared in the body under the
name it took from the parameter symbol it is bound to. The result is a
signature parameter and a body local of the same name and different types in one
scope, which is not compilable C.

The option `paramrefdecl` (default on since DIV-143) supplies the missing half
of the same predicate, on symbol identity rather than storage: a high whose
bound symbol — the identity `linkSpacebaseSymbol` already recorded, kuna's
`HighVariable::kuna_ref_symbol` for upstream `Varnode::setSymbolReference` — has
category `function_parameter` is that parameter, so no declaration is emitted and
the `&a0` in the body resolves to the prototype's own `a0`
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_paramrefdecl.rs
(references_parameter_symbol)`). It is not a rename: the parameter is not
duplicated under a fresh identifier, the second object simply stops existing. The
skip inherits the storage arm's guard that the high's name is one of the
prototype's parameter names, so a symbol renamed out from under the prototype
keeps its declaration rather than becoming an undeclared variable, and a
reference to a `no_category` local (`&v7`) carries a different category and is
untouched. The same symbol identity answers the `variables` JSON surface, which
collects a parameter's uses by matching Varnodes against the parameter's storage
and so reported an address-taken parameter as unused while the emitted C showed
the reference; the reference is attributed to the parameter only when no
storage-backed or name-backed evidence was found
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_paramrefdecl.rs
(parameter_reference_varrefs)`). `option paramrefdecl off` restores the distinct
body object, but the function-scope uniqueness invariant gives it a suffix
instead of re-declaring the parameter's identifier.

**(angr) dedupvardecls — collapsing duplicate declarations.** kuna's
declaration emitter walks HighVariables, not the upstream symbol table (which
declares each Symbol exactly once), so many scalar HighVariables sharing one
stack slot — all bound to the same name by the shared-storage naming — would
each emit a textually identical declaration line (x86_64/cvs `main` declared
one slot 166×). Composite symbols (arrays/structs/unions) are always collapsed
to one declaration per mapped symbol by an unconditional identity check in
`printc.rs (PrintC::emit_local_var_decls)`; the scalar analogue is the option
`dedupvardecls` (default on since DIV-7; row `source_decompiler = "angr"` —
angr's variable recovery yields one variable per storage location, declared
once), which collapses in two steps.

*By symbol.* Several HighVariables whose declaration representatives resolve to
one containing `ScopeLocal` symbol, and that render the same identifier, are one
variable and emit one declaration — the invariant upstream gets for free by
walking the symbol table. The symbol behind a storage location is the
smallest entry containing its base byte — upstream `Funcdata::linkSymbol`'s own
query — ignoring the use-point exactly as the parameter-category query of §9.3
does (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
(ScopeLocal::containing_symbol_for_storage)`), and the survivor is the first in
emission order. When the collapsing highs *agree* about the slot's type, that
recovered type stands, being the sharper information. When they *disagree*, the
survivor declares the symbol's own type — upstream `emitVarDecl` declares
`sym->getType()` — unless the symbol's type is narrower than the widest storage
the group covers, in which case the widest member wins. kuna's `ScopeLocal`
ranges can be narrower than the accesses that reach them, and a declaration
smaller than the object the body writes through would be a new defect rather
than a faithful one.

*By rendered line.* A declaration is then suppressed when its fully rendered
signature — final declarator type and its post-name suffix, name, array
adornment, and (under angr naming) the storage comment — is byte-identical to
one already emitted
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_dedupvardecls.rs
(DeclDedup)`). Keying on the rendered bytes makes this step provably lossless:
two same-named locals at different slots or types differ in signature and both
survive.

*By overlap group.* Register aliases can form a `VariableGroup` without any
mapped `ScopeLocal` Symbol: for example, separate AL and AH highs plus the AX
high that covers both. When exactly one declared piece contains every other
piece and all carry the same recovered name, only that whole-value high is
declared. References to the suppressed subpieces resolve through the whole
owner, so the byte writes render as `v4._0_1_` / `v4._1_1_` against one
`undefined2 v4` declaration. An absent or ambiguous whole cover does not trigger
this collapse, and differing user/debug names are never merged. If such grouped
pieces still collide after every semantic collapse, each is instead suffixed and
its access is rendered relative to its own piece-sized declaration (never, for
example, `byte_2._1_1_` against a one-byte object).

After the semantic and rendered-line collapses, declaration identifiers are
made unconditionally unique within the C function scope. The allocator reserves
parameter names, every original local spelling, every snapshotted global-symbol
name, and every rendered direct-callee name; it keeps the first available
spelling and gives any genuinely distinct collision a deterministic `_N`
suffix. Reserving the non-locals prevents a generated `value_1` declaration from
capturing an existing `value_1` global access or direct call. The chosen spelling
is keyed by HighVariable and used by every body reference path as well as its
declaration; existing parameter, user/debug, Ghidra-style, global, and callee
names remain authoritative.

Partial covers of a mapped scalar are suppressed only when another
HighVariable with the same name actually represents the whole storage: its
first member is non-constant, starts at symbol offset zero, and has the symbol's
full width (`printc.rs (high_name_has_scalar_whole_sibling)`). A constant
`PTRSUB` offset bound to `&local` also carries the symbol's name and full width,
but its declaration is filtered as an address reference; treating that constant
as the whole sibling would suppress every real partial and leave `local` used
without any declaration. When no whole storage high exists, the partials reach
the symbol-keyed collapse above, which retains one declaration for the shared
name.

The symbol step is what makes the collapse total for a *mapped* slot. The line
step alone left one stack slot declared twice under one name with two types
whenever two of its live ranges did not merge and recovered different types
(DIV-52), which is not compilable C and which no rendered-line key can catch.
No collapse can remove the last declaration of a referenced name: the symbol
step requires the identifier to match, the line step requires the whole line to
match, and the overlap step requires a declared unique whole owner. `option
dedupvardecls off` restores the one-line-per-HighVariable behavior for the two
option-controlled collapses; overlap-owner collapsing and collision suffixing
remain active because unique, correctly bound C identifiers are an output
validity invariant rather than a presentation choice.

## 9.4 Strings & comments

**String literals.** A constant pointer whose target type is a character type
triggers the string probe at the leaf push (§9.2): resolve the constant to an
address in the default data space, require the location to be **read-only**
in the global scope (writable data may have changed since load — refuse to
print a literal), then ask the string manager for decoded bytes and emit the
escaped, quoted literal — with an `L` prefix for wide characters and, when the
literal was clipped, the terminator `..." /* TRUNCATED STRING LITERAL */`
(`printc.rs (PrintC::push_ptr_char_constant_ir, PrintC::print_character_constant)`).
On any refusal the constant falls back to the ordinary integer render, so a
wrong guess costs readability, never correctness. The manager itself —
`decompiler/crates/kuna-decomp/src/p9_emit/stringmanage.rs
(StringManagerUnicode)`, one shared instance per Architecture with a
2048-character budget — pulls loadimage bytes 32 at a time until it finds a
character-width-aligned NUL (no terminator within budget, or unreadable
memory ⇒ not a string), validates the whole buffer as UTF-8/UTF-16/UTF-32 by
element width (any invalid codepoint or unpaired surrogate rejects the entire
literal), re-encodes to UTF-8, and caches the result — including negative
results — keyed by address and the complete decode mode (character width plus
opaque-string status), so one interpretation cannot supply the cached result
for another regardless of function-decompilation order. The decode mode is
marshalled with the result; legacy entries without it are not cache hits
(`stringmanage.rs (StringManagerUnicode::get_string_data)`). Rendering escapes per codepoint:
`printlanguage.rs (unicode_needs_escape)` classifies control characters,
separators, bidi markers, surrogates and private-use ranges as escape-worthy,
and `printc.rs (print_unicode)` emits the named C escapes then falls to `print_char_hex_escape`, which emits only `\x` (zero-padded to 2/4/8 hex digits by codepoint magnitude). Internal strings
(not in the loadimage) can be registered under a constant-space hash address
and resolve through the same cache.

The probe's entry condition is a **type**, not a detected string boundary: the
manager reads from whatever address it is handed, so the whole question is
whether the constant arrived carrying a character-pointer type. Type inference
supplies that for a typed callee parameter, and for a constant that hits the
start of a detected literal `ActionConstantPtr` supplies it via the global
spacebase reference. (ida) The remaining case is a pointer into the **interior**
of a read-only character array — how a compiler that merges string constants
shares one literal's tail (`"coreutils"` is stored only as bytes 4.. of
`"GNU coreutils"`; `"%s"` as the tail of `"%s: %s"`). `ActionConstantPtr`
recognizes it — upstream deliberately relaxes its exact-hit requirement for
character arrays — but the reference it builds for an interior hit is a spacebase
`PTRSUB` plus an `INT_ADD` of the residual, and constant folding collapses the
pair straight back to the bare constant, discarding the type; the exact-start
case survives only because its residual is zero. Upstream repairs this later
(`RulePtrsubCharConstant` rewrites the reference into a typed constant); kuna
instead types the constant where the evidence already exists — the covering
symbol is a character-printable array, which is stronger proof than inspecting
the bytes — and leaves the exact-hit path untouched
(`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionConstantPtr)`). Everything after that is the ordinary probe, including its
fallback: if the address turns out not to be read-only, or the bytes do not
decode, the constant prints as an integer exactly as before.

**The zero-character literal.** The probe's accept test is the string manager's
`is_string`, and that answers yes for *any* read-only location whose first
character-width unit is a NUL: `stringmanage.rs
(StringManager::check_characters)` walks only as far as the first terminator, so
for a zero-length string it validates zero characters and can reject nothing.
A pointer into a binary blob that happens to open with a row of zeroes therefore
passes the probe and renders `p = ""` — a token that names no byte of the image,
in place of the address, which was the only thing in the statement a reader
could follow.

Emptiness alone is not the tell, though, and this is where a narrower rule goes
wrong: `setlocale(6,"")` is idiomatic C, and a linker that merges string
constants stores a program's only `""` as the tail NUL of some other literal, so
the empty string is *real* there and the address would be the worse render.
What separates the two is the evidence the emptiness test never looked at — the
bytes *past* the terminator. A genuine `""` is followed by the next literal in
the table (`00 00 00 65 72 72 6f 72 20 69 6e 20 72 65 67 75`, the `""` `nl`
hands `setlocale`); a blob pointer is followed by bytes no C string holds
(`00 00 00 00 00 00 00 00 00 77 df 77 ff fd ff 7f`). **(kuna) emptystrconst**
(default **on**, `decompiler/crates/kuna-decomp/src/p9_emit/kuna_emptystrconst.rs
(declines_literal, reads_as_string_data)`) reads sixteen bytes at the constant
and declines the literal only when the escape walk emitted no characters *and*
those bytes positively contradict string data: skip the terminator run, walk the
next run up to its NUL, and reject if any byte in it is outside printable ASCII
and `\t`/`\n`/`\r`. The constant then falls through to the ordinary casted-hex
print. It is a falsification test on purpose, so everything it cannot judge keeps
the upstream literal — a window of nothing but NULs (padding at the end of a
section), a run that reaches the window's end still spelling text, an address
whose neighbourhood is unreadable, and of course any literal with even one
character of its own (`"\n"`, `"%"`). Only the first run after the terminator is
judged: `du`'s `fts_alloc(sp,"",0)` opens the table `"" "." ".."` and keeps its
quotes even though relocation bytes follow four bytes later. Turning the option
off restores the empty literal in every case. What
this does **not** repair is the reason the blob pointer carried a
character-pointer type in the first place: it shared a merged live range with a
genuine `char *` parameter (§6), and the probe is doing what it is supposed to do
for a `char *` constant once that type is established.

**A character pointer the probe declines is still an address.** When the bytes
at a `char *` constant do not decode as a string — the GB18030 quote glyphs
gnulib's `gettext_quote` returns (`a1 07 65 00`) are the common case — the
constant falls through to the pointer arm's casted-hex print, and with
`globalref` on (§9.9) to `&dat_<addr>` declared `char`.

**Comments.** Comments reach the output through the P0 knowledge plane, never
inline in the IR: analysis passes call `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::warning, Funcdata::warning_header)` — buffered per function, then
flushed by the decompile drive into the Architecture-wide comment database
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (CommentDatabase)`
with byte-exact de-duplication (same function, address, and text ⇒ dropped),
so re-decompilation never doubles a warning. At print time
`printc.rs (PrintC::setup_comments)` loads the function's comments into the
sorter `decompiler/crates/kuna-decomp/src/p9_emit/comment.rs (CommentSorter)`,
which bins each comment by the basic block containing its address and orders
it against the block's ops; the body emitters then interleave them — each
statement flushes the comments sorted before it (`printc.rs
(PrintC::emit_comment_group)`), and a construct that folds several blocks onto
one line (an `if` header) pre-flushes its whole subtree so no comment can land
mid-expression (`printc.rs (PrintC::emit_comment_block_tree)`), forcing the
pending `else if` brace when it does. Which categories display is a
`PrintContext` default (`printlanguage.rs
(PrintContext::reset_defaults_internal)`): header + warning-header types
render as `/* ... */` lines above the prototype, user and warning types inside
the body at the 20-column comment indent; a comment is marked emitted after
printing so overlapping windows never repeat it.

## 9.5 Pointer & array notation

**(kuna) arraynotation** — the second GH-558 decision (its registry row
records `ghidra-upstream` because the upstream *issue*, not upstream code,
motivated it; the implementation is kuna-original). A scaled pointer-add
(`PTRADD`) has three renders, decided at `printc.rs (PrintC::op_ptradd_ir)`:
inside a load/store context (the `print_load_value`/`print_store_value`
modifier is set, i.e. the pointer is being dereferenced) it is always the
subscript `base[index]` — that is upstream behavior and not optional; a
*standalone* PTRADD — the address value itself, passed to a call or stored —
is upstream `base + index`, and kuna's `option arraynotation` (default **on**,
DIV-2 lineage; option struct in
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_arraynotation.rs
(OptionArrayNotation)`, the flag on `printc.rs (PrintCOptions)`) renders it
`&base[index]` instead, keeping the element-typed reading — implemented as
two ordinary RPN tokens (address-of wrapping a subscript), so the surrounding
expression parenthesizes through the normal §9.2 precedence predicate. The
display-side relatives follow the same philosophy — a
symbol-mapped array access renders `name[index]` with the index in its
natural base, and a spacebase reference to a mapped local renders `&a` /
`&myval.b` through the PTRSUB symbol markup (§9.2's partial-symbol walk).
Flip `off` for consumers that diff against raw pointer arithmetic
([docs/options.md](../options.md)); the toggle is per-render and pure
presentation.

## 9.6 Alternate languages

**(kuna) The output-language plane.** The C++ tree kuna was ported from selected a
back-end through a `PrintLanguageCapability` registry over a three-level hierarchy
(`PrintLanguage` → `PrintC` → `PrintJava : public PrintC`); the port flattened
that into one concrete `PrintC`. kuna re-erects the seam by **parameterizing** the
single emitter rather than growing a second one: `PrintC` carries one `out_lang`
field, and every language-varying site reads a `&'static` policy object through
`printc.rs (PrintC::lang)` instead of naming a `keywords::`/`tokens::` constant.
The RPN driver, the op emitters, `parentheses`, the cast plumbing, the comment
sorter and the markup back-end are shared verbatim — there is no duplicated
emitter, which is what keeps one implementation of "emit an `if`" as languages are
added.

Three artifacts make up the plane, all in
`decompiler/crates/kuna-decomp/src/p9_emit/`:

- **`kuna_lang.rs`** — `OutLang` (the selector), `LangProfile` (the surface
  vocabulary: the keyword and punctuation spellings the emitters use, plus the
  `OpToken`s whose *spelling* varies; the ~40 arithmetic/comparison/shift tokens
  are identical in every language kuna targets and stay in `printc.rs (tokens)`),
  and `LangCaps` — **what the emitter is allowed to produce**. `LangCaps` is what
  lets a language that cannot express a construct never be handed one, instead of
  the operator being asked to flip the kuna rendering defaults that would produce
  it (`truthycond`'s implicit-bool condition, `braceelide`'s braceless body,
  `condfold`'s comma operand, `nullprinting`'s `NULL`). Its load-bearing member is
  `switch_captures_break`: a C `switch` captures a bare `break`, which is why
  `p8_structure/kuna_loopbreak_recovery.rs` legitimately retags a
  goto-to-switch-exit as `f_break_goto` (§8.3); a language whose switch does *not*
  capture `break` must re-resolve that scope or emit a jump to the wrong place.
- **`kuna_langtypes.rs`** — `TypeSpeller` and `SpellCtx`. Type *recovery* (P5) is
  language-independent; only the spelling differs, and it lives in the printer for
  the reason `kuna_ctypes.rs` records: `Datatype::hash_name` makes the registered
  name determine the type id, so renaming the interned core types would break the
  Ghidra wire protocol. `SpellCtx` is the former `RealTypeCtx` — `Copy`, already
  threaded through every declarator chokepoint — now also carrying the language, so
  the free-function declarator family reaches its speller with no new parameter.
  `TypeSpeller::declarator` is documented as `<front><name><back>` rather than
  promising a meaningful `back`, because the front/back split is a C-ism: C
  declarators wrap the identifier (`int4 (*a)[1]`) where other languages' types are
  pure prefixes.
- **`p4_calls/kuna_langabi.rs`** — `LangAbi`, the ABI seam. It owns one decision,
  consulted by the Rust prototype emitter: which `extern` a signature declares.
  Rust declares `extern "C"` exactly when the prototype is variadic (rustc admits
  a C-variadic on nothing else) and otherwise nothing, which means the
  unspellable default `extern "Rust"`. Chapter [04](04-calls-and-prototypes.md)
  explains why the axis is thin and what a third language would add.
- **`kuna_langc.rs`** — `CSpeller`, the c-language policy object. It carries the
  declarator algorithm transcribed from `pushTypeStart`/`pushTypeEnd`/
  `buildTypeStack` and the `realtypes`/`ctypes` relabelling (DIV-5/DIV-6), moved
  verbatim out of `printc.rs`, which keeps thin dispatchers.

  C declarator modifiers are consumed outermost-to-base around the identifier.
  A pointer prefixes the declarator; a later array postfix groups the complete
  accumulated declarator before appending its suffix.  Thus a pointer to an
  array is `T (*p)[N]`, an array of pointers is `T *p[N]`, and deeper
  pointer/array alternations preserve the same precedence instead of moving only
  the nearest parenthesis.  Casts, parameters, locals, members, typedefs, and
  exported type strings all share this speller. The internal postfix builder is
  generic enough to obey the same rule for a function suffix, although current
  `Datatype` traversal supplies pointer and array modifiers only.

  Every declaration places both halves around its name, because the front of a
  pointer-to-array declarator (`T (*`) is not a type on its own.
  `TypeSpeller::type_name`, the declaration-position spelling, therefore returns
  the pair too, with an empty back for everything but a C pointer whose
  declarator needs one. A cast or an exported type string is `<front><back>`
  (`T (*)[N]`); a parameter or a struct member is `<front><name><back>`; a local
  is `<front><name>[ [count]]<back>`, so an array of pointers to arrays keeps its
  count inside the group (`T (*v [2])[N]`); and a return type puts its front
  before the function name and its back after the parameter list
  (`T (* f(int x))[N]`). The rendered-line collapse of §9.3 keys on the back as
  well, so two locals differing only in it both survive.

The invariant that makes the seam free: every `LANG_C` field **is** the constant
it replaces, asserted field-by-field — and by pointer identity for the tokens,
since `printlanguage.rs (parentheses)` decides parenthesization with `ptr::eq`.
Reading the C profile therefore produces the identical token, so introducing the
plane is a byte-identical rewrite and `docs/baseline.json` is never re-pinned.

**(kuna) The rust-language back-end.** `option setlanguage rust-language` selects
it (`p0_knowledge/options.rs (OptionSetLanguage)`, the upstream selector, which
now rejects a name no back-end claims rather than silently keeping C). The
recovered function is not re-analysed: the same `Funcdata`, the same types, the
same structured block tree render through a different profile. Three files carry
it: `p9_emit/kuna_langrust.rs` (the profile, the capability record, the three
`OpToken`s whose spelling or precedence differs, and the emitters for the shapes
that are not C's), `p9_emit/kuna_rusttypes.rs` (the speller), and the
`LangForms` matches in `printc.rs` that choose between them.

What differs, and why each is a language fact rather than a preference:

- **Signature** `unsafe fn n(mut a0: T) -> R`. `unsafe` because a decompiled body
  dereferences raw pointers, and an `unsafe fn`'s body carries them with no inner
  block; `mut` on every parameter because a decompiled body assigns to its
  parameter slots and the recovery does not distinguish the ones that do. A unit
  return is omitted rather than spelled `-> ()`.
- **Declarations** `let mut n: T;`, with an array count folded *into* the type
  (`[T; N]`) rather than trailing the identifier.
- **Types** `i8`..`i128` / `u8`..`u128` / `f32` / `f64` / `bool` / `*mut T` /
  `[T; N]` / `()`. A recovered text byte spells `u8`, never `char` — a Rust `char`
  is a 4-byte Unicode scalar with a validity invariant that a decompiled byte does
  not carry. A width Rust cannot name (3/5/6/7, x87's 10) spells `[u8; N]`, which
  is *more* faithful than C's `undefined3`: it names the storage exactly and does
  not claim to be a scalar.
- **Recovered composite names** are spelled in *type* position rather than
  flattened to an identifier. A composite carries whatever spelling the debug
  info recorded — `Result<u8, u32>`, `Vec<u8, alloc::alloc::Global>`,
  `(u8, u32)` — and every character in those is legal Rust type syntax, so the
  identifier sanitiser would turn `Result<u8, u32>` into `Result_u8__u32_` and
  discard the one thing the reader wanted. What falls outside the type grammar
  (a DWARF `{closure#0}`, a codegen-unit suffix) still collapses to `_`, and a
  name whose brackets do not balance falls back to the identifier sanitiser
  whole, because a stray `<` would swallow the rest of the declaration.
- **Names** are rendered as Rust *paths*, not flattened into identifiers. `::` is
  legal wherever a name is used — a call, a static — because that position takes a
  path; only a `fn` *definition* needs a bare identifier, so it takes the last
  component and the full path goes in a comment directly above. Flattening `::` to
  `__` everywhere (what a naive identifier sanitiser does) discards the module
  structure for nothing and produces `alloc__vec__Vec__resize` where
  `alloc::vec::Vec::resize` is both valid and what a reader expects.
- **Operator precedence** comes from a declared ladder, not from per-token
  numbers: `LangProfile`'s `PrecLadder` names each tier once, in order, with its
  rank. A language remaps only the tokens whose tier it MOVES — everything else
  keeps the ported C table's number — so the ladder must speak the same numbers
  the table does, and a test asserts that any tier a language leaves in place
  keeps C's rank exactly. Rust moves four: `& ^ |` sit ABOVE the comparisons where
  C puts them below, every comparison shares one non-associative tier where C
  ranks relational above equality, and `as` gets a tier of its own between the
  multiplicative and unary operators. The first of those is why the ladder is
  declared rather than hand-numbered — it changes what `a | b == c` *means*, so
  getting it wrong is a silent wrong answer rather than a syntax error, and
  hand-numbering eleven tokens is precisely how that happened once already.
- **Casts** `x as T`, which inverts the operand order relative to C's `(T)x`;
  every cast site brackets its operand with `push_cast_open`/`push_cast_close`
  instead of emitting the type inline. The `as` token additionally sets
  `OpToken::paren_before_angle`, because `x as i32 < 5` parses `i32 <` as the
  start of generic arguments and precedence alone cannot express that.
- **Loops** `loop { }` for the infinite form, `while c { }` with no parenthesised
  condition, and `loop { body; if !(c) { break; } }` for the bottom-tested one.
  A condition carrying statements — what C renders as a comma expression — becomes
  Rust's block expression `while { stmt; c } { }`. The C `for` header is not
  reached at all: `analyze_for_loops` is gated on `LangCaps::c_for` at the
  `ArchContext` copy, because the reroll physically MOVES the initializer and
  increment and rendering that as a `while` would drop them, while moving them
  back at print time would let a `continue` skip the increment.
- **Multi-way branches** `match v { A | B => { … } _ => {} }`. Arms do not fall
  out, so C's explicit `break;` is dropped; a `match` on an integer must be
  exhaustive, so a `_` arm is synthesised when the recovered switch had no
  `default`; and a wildcard must come last, so the default arm is hoisted (safe,
  because the remaining patterns are disjoint integer literals). Multi-label arms
  are free — `emit_switch_case` already enumerates one `case N:` per jump-table
  index for a shared block, and the same list joins with ` | `. Note the ordinary
  `case A: case B: body` shape is ONE recovered case with two indices, not a
  fall-through chain.
- **Selection expressions** the `iteregion` recovery renders `dest = if c { A }
  else { B };`. Rust's `if`/`else` is an expression, so this is a *better* form
  than C's `?:`, not a lost one — the passes stay on.
- **Literals** no `U`/`L`/`LL` suffixes (Rust infers the literal type, and naming
  a width here would assert one this site does not know); `b'a'` for a 1-byte
  character constant with a printable spelling and the integer otherwise, since
  Rust has no `'\xff'`; and a string body escaped with Rust's set, which has no
  `\a`/`\b`/`\v`/`\f` and in which a single quote must be BARE — `"PCRE\'s"`
  does not tokenize.
- **Three kuna rendering defaults are suppressed by capability, not by asking an
  operator to flip them**: `truthycond` (DIV-37) would emit `if x` on a `u32`,
  `braceelide` (DIV-38) would emit a braceless body, and `nullprinting` (DIV-35)
  would emit `NULL` (Rust spells it `core::ptr::null_mut()`). `condfold`'s comma
  operand has no Rust form either and is refused the same way.
- **What Rust cannot express** is marked, never silently emitted. The structurer
  manufactures `goto`s as its escape hatch and Rust has no form for one that is
  neither a `break` nor a `continue`; those render as a comment plus a **diverging**
  `panic!("kuna: unstructured goto to <label>")`. Diverging so the document still
  type-checks in any position, loud so a reader cannot mistake it for a
  translation, and greppable because that count over a whole-binary render IS the
  quality number for this back-end — exactly what `gotoreduce`, `taildup`,
  `ifelseflatten` and `crossjumprevert` reduce. A genuine switch fall-through gets
  the same treatment: resolving it needs the next arm's body duplicated, which is
  a block-graph edit and not a printing decision.

The contract is **`syn::parse_file` validity, not `rustc` compilation**, and
`kuna-console/tests/verify_outlang_rust_syntax.rs` enforces exactly that by
parsing the emitted document. Decompiled output calls functions that have no
definition, `CARRY4(a, b)` has no Rust spelling, and `[u8; 3]` does not do
arithmetic; making the output compile is a separate and much larger project.

**The marker is reported, not only emitted** (GH-668). A body whose jump became a
`panic!` is a translation up to that jump and not past it, so a consumer that
reads only the exit code, stderr and the `error` field used to be told a lossy
render was a clean one. Two channels carry the fact now. Every surface that
renders a function names it on stderr in one line — the count, the function, and
the two ways out (`--language c`, which spells the jump as a real `goto`, or a
P8 structuring option that removes it) — and every per-function JSON record
carries `unstructured_gotos`, which is `0` on a C render and on a Rust one with
nothing lost. The **verdict does not move**: exit stays `0` and `error` stays
`null`, because the rest of the body is usable output and a caller that already
treats success as "there is code here" is not wrong.

Both channels count occurrences of `UNSTRUCTURED_GOTO_MARKER` in the *rendered
text*, exported from `p9_emit/kuna_langrust.rs` beside the printer that builds
the marker from it. Counting the text rather than the emitter is what makes one
mechanism serve every surface: `kuna decompile` drives `decomp_dbg` as a
subprocess and would never see an in-engine counter, a `--jobs` worker ships
framed records across a pipe, and the WASM front-end holds only the document —
but all of them hold `code`. The matched text includes the marker's unescaped
`"`, which a Rust string literal in the decompiled program can never contain
(the printer escapes an interior quote), so program data cannot inflate the
count. The switch fall-through marker is a comment with no diverging call and is
deliberately outside this count: it is a different lossy construct, and naming
one number `unstructured_gotos` that meant two things would be worse than
reporting the one.

Two surfaces refuse the language rather than half-honouring it. `kuna-ghidra`
pins its Clang token-markup document to C (`process.rs`), because that document
is consumed by Ghidra's C token model and Rust text in C token slots is a GUI
regression. `kuna decompile-project` errors, because its `.c`/`.h`/`.asm` export
is C-shaped end to end.

The Java back-end is deliberately not ported:
`decompiler/crates/kuna-decomp/src/p9_emit/printjava.rs (PrintJava)` is a
recorded LOSS whose constructor returns an error — upstream `PrintJava` is a
thin `PrintC` subclass (shared token table and RPN driver, eight overrides for
object references and `instanceof`), and no oracle datatest selects the
`java-language` back-end, so kuna registers only `c-language` (the default
capability, `printc.rs (CAPABILITY_NAME)`). What *is* live is the Java half of
the cast strategy, `cast.rs (CastStrategyJava)` — Java's pointer-encoded
object references change which extensions and pointer conversions are
representable as casts — kept current alongside `CastStrategyC` so a future
`PrintJava` port is emitter wiring only.

## 9.7 Whole-program document renders (`kuna decompile-project`, `kuna decompile-graph`)

**(kuna) Three additive render surfaces** back the `kuna decompile-project`
project export (the CLI driver is
`decompiler/crates/kuna-cli/src/decompile_project.rs`; usage in
`docs/agents.md`). All three are pure *readers* of finished state — they run
after analysis, insert no ops, flip no options, and change no byte of any
existing render path (the datatest / stages / `decompile-all --json` outputs
are untouched), which is why none carries a `phases.toml` row or a DIV entry.
That still holds under `--stream`, one level down: the streamed export
drives these same renders and changes no byte any of them produces, but the
project folder it assembles from them has a layout of its own — the single
documented exception, described at the end of this section.

**Type definitions — the `docTypeDefinitions` port.** `printc.rs
(PrintC::doc_type_definitions)` is the previously-unported C++
`PrintC::docTypeDefinitions` surface — the console `print C types` command
(`decompiler/crates/kuna-console/src/ifacedecomp.rs (IfcPrintCTypes)`) was a
stub and now wires through it, via the driver
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (print_c_types)`.
It emits a C definition for every user-defined data-type in the factory,
consuming `decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(TypeFactoryImpl::dependent_order)` (chapter [05](05-types.md) §5.1) so every
definition precedes its uses. Core types, unnamed types, and the internal
`Partial*` slices are skipped; what renders is typedefs, structs, unions, and
enums (`printc.rs (render_type_definitions)`; the per-type body renderers —
`compose_type_body`, `compose_enum_body`, `compose_typedef_line` — are pure
functions for unit-testability, and emission is direct string building, since
no emitter markup exists for type definitions). The structures `structsynth`
minted are moved after every other type, in ascending `N`
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/ledger.rs
(in_name_order)`), because the tree orders them by the address of a field's type
and the header would otherwise change from run to run (chapter
[05](05-types.md), struct synthesis). Two documented `(kuna)`
divergences from the upstream emission, both in service of "the `.h` always
compiles":

- **Forward-declaration block first.** Upstream prints one anonymous
  `typedef struct {…} name;` per type — a form that cannot express a
  self-referential or mutually recursive pointer field. kuna instead emits a
  `typedef struct <n> <n>;` tag+typedef forward declaration for every
  struct/union up front, then the bodies as plain `struct <n> { … };` in
  dependency order; an incomplete (field-less) struct emits *only* the
  forward declaration, annotated `/* opaque */`.
- **Explicit padding fields.** Struct field-offset gaps and trailing padding
  (the field extents vs `get_size()`) render as `undefined1 _pad<hexoff>[N];`
  members, so `sizeof(struct <n>)` under a recompile matches the decompiler's
  layout. Bitfields render best-effort (`<type> <name> : <bits>;`, padding
  suppressed since their byte coverage overlaps the gap computation); unions
  carry no padding.

A non-C identifier is rewritten by `printc.rs (sanitize_type_name)` (annotated
`/* renamed from "…" */`), and a later duplicate name emits a
`/* duplicate type name skipped */` comment instead of a redefinition — the
first definition wins.

**(kuna) A type name and a function name cannot be the same name.** C keeps
typedefs and functions in one namespace at file scope, and POSIX supplies
several spellings that are both: `stat`, `sigaction`, `group` are each a struct
tag and a function. An image that carries `struct stat` in its debug info and
calls `stat()` therefore produced a header where
`int stat(const char *, stat *);` is `error: 'stat' redeclared as a different
kind of symbol` — and, because the parse does not recover, every declaration
after it fails too. `decompiler/crates/kuna-console/src/project.rs
(build_header)` resolves the clash in favour of the type: a function whose name
is one of the header's typedef names (`project.rs (typedef_names)`) has its
prototype emitted as a comment carrying the full signature, instead of as a
declaration. The type is what every other signature in the file depends on; the
suppressed prototype is one line, and it is still printed. The rule is the
header's alone: the body still spells a call to such a function and a cast to
such a type with the one name they share, so the exported `.c` keeps the clash
the header sheds.

**The prototype — one token stream, two documents.** The prototype segment of
§9.2's document walk was extracted verbatim into `printc.rs
(PrintC::emit_prototype_declaration)` — pure code motion, byte-identical
inside `emit_function_document` — so `printc.rs (PrintC::doc_prototype)` can
drive the IDENTICAL token sequence standalone: the same
`set_output_stream()` → emit → `output_str()` capture harness as
`doc_function_full`, plus a trailing `;`, minus the header warning comments.
The contract this buys the export: the `.h` prototype minus its `;` matches
the `.c` definition line **token-for-token** — there is no second prototype
printer to drift. The exporter does not canonicalize the reserved C entry-point
name: a recovered `void main(void)` remains exactly that in both documents rather
than being rewritten to an unsupported `int` signature. Consumers that only need
to syntax-check the header can macro-remap `main` before inclusion. The public driver is `decompile_drive.rs
(print_c_prototype)` (a function with no recovered proto store renders
`void <name>(void);`).

**The recompile prelude.** `decompile_drive.rs (print_c_recompile_prelude)`
generates the typedef block that makes the other two renders compile: one
standard-C typedef per interned *core* scalar type (`typedef unsigned int
uint4;`, …; 8-byte integers always spell `long long` so the text is
data-model independent; `bool` is covered by `#include <stdbool.h>`;
`char`/`void` are real C and emit nothing), then the fixed Ghidra/kuna
`undefined` family — `undefined`, `undefined1..8` (3/5/6/7 mapped to the next
larger unsigned integer, each carrying a sizeof-divergence note) and
`undefined16`/`undefined32` as byte-array structs. The non-printer half of
the export (section enumeration, one-instruction disassembly, raw image
bytes, named data symbols for the `.asm`/`README.md` artifacts) lives on the
console engine, `decompiler/crates/kuna-console/src/engine.rs
(ConsoleProgram::sections, disassemble_at, read_bytes, global_data_symbols)`,
not in this folder.

**The streamed layout.** `kuna decompile-project --stream` (chapter
[00](00-overview.md)) drives these same renders one result at a time and
adds three pieces of its own, none of which a non-stream export can see.
The linear disassembly walk becomes **resumable**
(`decompiler/crates/kuna-console/src/project_stream.rs (AsmSweep)`): the
same section-by-section walk
`decompiler/crates/kuna-console/src/project.rs (build_asm)` performs,
cut into steps against a code-byte budget, carrying its scratch state —
above all an in-progress `db` run — across the cut, so a chopped budget
shifts which step a byte is written in and not which byte is written.
`header_lines()` plus every step plus the data tail is the non-stream
`.asm` exactly. The **variables section** (`project_stream.rs
(render_variables_section)`) then carries what the labels lost: a
streamed sweep runs to completion long before any function has
variables, and an append-only file cannot go back under a label, so the
per-function `; arg:` / `; stack:` blocks are collected into one address-then-name
ordered `; --- variables ---` section appended after the sweep, ahead of the unchanged
`; --- data ---` tail (`project_stream.rs (render_data_tail)`). The set of comment
lines is exactly the set removed from under the labels. Finally the
README gains a second **layout** and an in-progress render
(`decompiler/crates/kuna-console/src/project.rs (render_readme,
ReadmeLayout, render_readme_streaming)`), the first describing the
streamed `.c`/`.asm` and the two extra files, the second adding a banner
and a status table while the export runs; the `Standard` layout is
byte-identical to what the single renderer emitted before it was split.

**The graph document.** `kuna decompile-graph`
(`decompiler/crates/kuna-cli/src/decompile_graph.rs`) is another additive reader
of completed program state: one JSON document holding every discovered function
with its recovered signature, parameters, C body and assembly, plus the call
edges between them. It adds no analysis facts, mutates no IR and changes no
existing C rendering, so it has neither an option row nor a DIV entry. The
field-by-field schema is `docs/cli.md`; what follows is why the document says
what it says.

Every question the document answers is answered by the surface that already owns
it, because two surfaces disagreeing about one program is the failure mode here.
Which entries exist and which of them have bodies is the whole-binary target
policy of §0.2 (`function_entries_canonical` for the rows,
`decompiler/crates/kuna-cli/src/decompile_all.rs (resolve_targets)` — i.e.
`function_entries_executable` — for the bodies), so an address that is callable
but not executable content is a labelled row — `import` for a pointer slot the
program calls through, `data` for a named address that is simply not code —
rather than a body lifted out of a pointer table. Naming such an address with
`--addr` does not buy an exception: the row would then contradict its own `kind`,
and what the run produced would be a plausible-looking function lifted out of a
pointer table. What a function *is* comes from
the shared per-function classifier
(`decompiler/crates/kuna-console/src/classify.rs`), the same one the browser
inventory groups by. Bodies and parameters come from the shared decompile loop
(`decompiler/crates/kuna-console/src/project.rs (decompile_targets)`), which
isolates a failure to one record — and that record's `error` is carried into the
document, so a consumer counting functions is not silently counting a subset.
Assembly comes from the listing walk
(`decompiler/crates/kuna-cli/src/disassemble.rs (function_listing)`), so an
undecodable byte inside a body is a `.byte` row and not the loss of the whole
listing. Edges come from the reference index `kuna xrefs` answers with, through
the same call-graph model `--reachable-from` walks
(`decompiler/crates/kuna-cli/src/decompile_all.rs (CallGraph::callees_of)`).

**Both ends of every edge are rows of the same document.** A reference into the
middle of a body resolves to the body, and one that lands in no discovered
function at all — a `CALL 0x0` off a nulled relocation, a branch into a gap
between entries — is not a call-graph edge and is not emitted, because a
consumer that has to model containment itself to use the edge list has been given
the wrong list. An edge's kind is the `kuna xrefs` kind, so the document and that
command cannot describe one program differently: `call`, `jump` for a tail call or
a branch into a neighbouring entry, and `data` for a function whose address is
materialized. The third is not optional decoration — it is how `main` has a
caller at all in a glibc program, where `_start` hands it to `__libc_start_main`
as a pointer, and dropping it would under-report exactly the indirection an
obfuscated program leans on. A materialized address that does not land on a known
function entry is not an edge (that is a string or a global, not a callee), and
where a caller both calls a function and mentions its address, the one edge
carries the stronger of the two kinds. A computed call whose destination is not a
decode-time constant has no static target and therefore no edge at all; it is
reported as `hasIndirectCalls` on the row that contains the call site — `CALLIND`
only, folded onto its function by the same ordered containment that decides which
function an instruction's references are listed under
(`decompiler/crates/kuna-analysis/src/listing/xrefs.rs (XrefIndex::has_indirect_calls)`)
— while a forwarding veneer's `jmp [slot]` is an indirect *branch* whose
destination is in fact known, and is reported as `forwardsTo` instead. An
imported API call, `call qword ptr [slot]`, is the case where a `CALLIND` *does*
carry an edge: the slot is a decode-time constant the reference walk reads out of
the instruction (§1.6), so the row is both an edge to the import and
`hasIndirectCalls`. That slot
is only recoverable where the jump names it as a decode-time constant, so an
AArch64 stub that computes it across `adrp`/`ldr`/`br` is a `thunk` row with a
null `forwardsTo`.

**Ordering is total, so two runs of one command are byte-identical.** Function
rows are entry-VMA ordered; each caller's edges follow in first-reference order
with a contiguous zero-based `calleeOrder`, deduplicated on the callee. The key
is `address` and only `address` — a name is not unique in the document, since a
thunk, the pointer slot it forwards through and the callable they stand for are
three rows under one name.

**The document is C.** `codeC` names its language, so this surface refuses any
other rather than half-honouring the request, exactly as the project export does
(and it is excluded from the same auto-language policy, so a rustc-built binary
does not silently produce Rust in a field called `codeC`).

**Absent provenance is `null`, never a placeholder.** `analysisImageBase` is the
PE optional-header ImageBase when present and otherwise the lowest non-empty
loadable segment VMA, keeping it in the same static VMA space as the function and
edge addresses; a relocatable object has no comparable static base and reports
`null` rather than its synthetic loader layout. A field that could never be
filled is not carried: the loader retains no library-module mapping, so the
document has no module-qualified callee rather than a key that is always `null`.

## 9.8 The per-function type-definition preamble (`structdefs`)

The composite definitions §9.7 renders exist only on the whole-program
surface. A single function printed to a terminal gets none of them: a body that
reads `f->_flags` names `FILE` in its signature and nothing says what a `FILE`
is, so the layout the decompiler recovered — the thing the field accesses are
printed *against* — is invisible on the surface most callers actually read.
`option structdefs on` (P9 sub-decision `type-definition-preamble`, default off)
prints it, above the function, the way angr prints its typedefs.

**What the preamble contains** is the definable types that function's own C can
name: composites, enums and typedefs, never core types.
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_structdefs.rs
(referenced_types)` collects the function's semantic type surface — the
prototype's return and parameter types, every Varnode data-type, and every
mapped Symbol type behind a HighVariable — and walks each one's dependency cone
(`kuna_structdefs.rs (visit)`): the typedef base first, then the component
sub-types `substrate/dtype.rs (Datatype::get_depend)` reports — a pointer's
pointee, an array's element, a struct's fields — pushing each definable type
after everything it depends on. The result is definition-before-use over
exactly the referenced subset, which is the order §9.7's
`TypeFactoryImpl::dependent_order` produces over the whole factory, computed
here from the roots instead so a whole-binary run costs the types a function
touches rather than the types the program interned.

**Those roots are chosen to be a superset of what the printer spells.** Every
type name in the emitted C is read off one of them — a local declaration off a
high or its mapped symbol, the signature off the prototype, a cast off the
Varnode type it casts to — so the preamble cannot omit a definition the body
refers to. Collecting from the emitted token stream instead would be exact for
names that print, and would miss the struct behind `p->field_0x8`, whose tag
name the C never prints at all. The cost of the choice is the opposite error: a
type carried by a Varnode that contributes no token can be defined above a
function that never names it.

**One renderer, two surfaces.** `printc.rs
(PrintC::emit_type_definition_preamble)` hands the subset to the same
`printc.rs (render_type_definitions)` §9.7 builds the export's `.h` type block
with, so a preamble line and a header line for one type are the same line: the
forward-declaration block first, then bodies, an incomplete struct printing as
`typedef struct FILE FILE; /* opaque */`, a padding gap as `undefined1
_pad<hexoff>[N];`. It is emitted as whole lines rather than tokens, before
`begin_function`, because a type definition has no `PcodeOp` or `Varnode` for
the markup back-end to bind to. A `kuna decompile-project` export therefore
*suppresses* the preamble in its bodies
(`decompiler/crates/kuna-console/src/project.rs (decompile_pulled)`): those
bodies include a header that already carries every definition, and printing them
again above each body would redefine them. The suppression is keyed on a batch
option that says exactly that — *this caller renders a header that defines these
types* (`DecompileOptions::header_carries_types`, set by the export surfaces and
by a `--jobs` worker serving one, which is the worker the export asks for the
`.h` type block with `--jobs-types`). It is deliberately **not** keyed on
`want_proto`: `kuna decompile-graph` asks for prototypes too, and its document
is per-function C with no header artifact, so it keeps the preamble inside
`codeC` exactly as `decompile-all` keeps it in `code`. The preamble is
documentation rather than a translation unit — a `undefined1` padding member
needs the export's recompile prelude to compile — which is the other half of why
the definitions stay in the header on the surface that is meant to rebuild.

**Two disclosed roughnesses.** A definition whose name is also a function name
prints directly above that function's own definition: `typedef struct stat
stat;` over `int stat(char *a0,stat *a1)` is not a translation unit a compiler
accepts, and the export's `.h` says as much where it drops such a prototype
(§9.7). The preamble does not apply that guard — it is documentation, and
hiding the layout of `stat` from the one function that is about to use it costs
more than the collision does. And the over-inclusion the Varnode-rooted walk
buys (above) is real: a function whose only contact with `FILE` is handing
`stdout` to a callee still gets `FILE` defined above it. On a stripped corpus,
where a libc shell has no members and so pulls nothing else in, that is about one
definition in seven (55 of 367 over stripped `cmp`, `od`, `find`, `tar`); on
their unstripped twins it falls to 50 of 10,912, because a DWARF type the body
does not name is almost always a dependency of one it does.

**The preamble is C, and says so by declining.** The renderer builds the
project export's `.h`, so its output is C whatever the active output language
is: under `--language rust` a `struct X { … };` block carrying Rust-spelled
field types would be neither valid Rust nor readable C. `kuna
decompile-project` refuses a non-C output language outright for exactly that
reason (§9.6, §9.7); the preamble takes the same decision one step smaller —
it declines, and the body is emitted as it would have been. Teaching the
definition renderer the language plane is that plane's work, not this option's.

**The machine-readable half.** `decompile_drive.rs
(extract_type_definitions)` reports the same set as the `types` array of a
`kuna decompile-all --json` function record: one object per type carrying its
name, its definition text (`kuna_structdefs.rs (definition_text)` — the body for
a complete composite, the enum block, the typedef line, or the opaque forward
declaration) and its size, so a consumer reads a recovered layout without
parsing C out of `code`. The key is always present and is empty unless the
option is on: the array and the printed preamble are one decision. The factory
can hold two data-types under one name — a DWARF image carries both the
forward-declared `struct _IO_FILE` and the defined one — which the rendered text
already resolves by printing one forward declaration and one body;
`kuna_structdefs.rs (dedup_by_name)` applies the same rule to the records, since
two entries for `_IO_FILE` reporting size 0 and size 216 is a contradiction a
JSON consumer has no way to resolve.

## 9.9 Constant addresses named as globals (`globalref`)

A pointer-typed constant that no global Symbol covers has exactly one spelling
in the pointer arm of `printc.rs (PrintC::push_vn_explicit_ir)`: the forced-hex
integer behind a typecast, `sub_e4ca((struct_2 *)0x2b080)`. Upstream reaches a
name only through `coreaction_render.rs (ActionConstantPtr)`, which links a
constant to a Symbol that already exists, and a stripped image has none for most
of its data — while kuna prints `dat_2b080` two statements away for the same
address, because *reading* an unnamed global goes through the unnamed-location
leaf (§9.3), which names it. The type campaign made the cast commoner rather
than rarer: once a callee's parameter is recovered as `struct_2 *`, the
constant a caller passes it is typed `struct_2 *` too, so `sub_e4ca(0x2b080)` at
the campaign baseline became `sub_e4ca((struct_2 *)0x2b080)` (572 such casts on
the functions kuna and IDA both emit before the campaign, 1,118 after it). IDA
prints `sub_E4CA(qword_2B080)` and no cast.

`option globalref` (P9 sub-decision `constant-address-global`, default **on**)
prints such a constant as `&dat_<addr>` — the name the unnamed-location leaf
already uses, or `DAT_%08x` under `namestyle ghidra` — and records the global as
an object of the pointed-to type. The expression then has exactly the constant's
pointer type, so the cast is gone because the object is declared at that type,
not because it was suppressed, and its value is the address it replaces
wherever `dat_<addr>` is placed at `<addr>`. The decision is
`decompiler/crates/kuna-decomp/src/p9_emit/kuna_globalref.rs (Plan::decide)`,
consulted at the top of the pointer arm after the string probe (§9.4) and
before the NULL token and the cast; `printc.rs (PrintC::push_global_ref_ir)`
emits the `&` and the name, bound to the constant Varnode for markup.

**Where a global can be.** The object loader classifies its sections once
(`decompiler/crates/kuna-analysis/src/loader/kuna_globalref.rs
(holds_program_objects)`): an allocated section of initialized, read-only or
zero-filled data is a place a program object can live; code is not, a TLS
template is not (its addresses are offsets, not run-time locations), and
neither are the tables the toolchain and the run-time loader own even when
their kind says data — `.got`, `.plt`, `.eh_frame`, `.gcc_except_table`,
`.interp`, `.dynamic`, the PE import/export/exception/relocation directories,
the Mach-O symbol-pointer and unwind sections. The merged ranges reach the
engine as `infra/architecture.rs (Architecture::globalref_ranges)`, installed by
the object bootstrap beside the `litpoolconst` ranges; every path without a
section table leaves them empty, so the rule is inert for the XML datatest
corpus.

**What one function knows.** `kuna_globalref.rs (plan)` walks the function
once when its document starts, and records three facts:

- the constant values the function reads *as numbers* — a non-pointer constant
  read by an ordering compare, a multiply, a divide, a remainder or a shift
  (`kuna_inferfuncentry.rs (reads_as_integer)`, the question `inferfuncentry`
  asks of a constant that lands on a function entry: coreutils `tail`'s
  `BUFSIZ` is `0x2000`, which is that image's `_DT_INIT`);
- every direct access to the default data space — the `dat_<addr>` the body
  reads or writes — by start, width and type;
- for each in-range constant address, the pointed-to types the function reads it
  at: through `void *` only, at one other type (a `void *` use besides it is
  fine), or at two (`Seen`).

**What it refuses.** The constant keeps its cast when the address lies outside
every range (a pointer-typed `0x1`, `0xffffffff` or `-1` is not an address, and
is 21% of the corpus's constant casts); when a global Symbol already covers it
(that is `ActionConstantPtr`'s case, and minting a second name would collide);
when the same function reads the value as a number; when it uses the address at
two different pointed-to types; when it also reads or writes the storage
directly at another start, width or type — `sigemptyset((sigset_t
*)0x2b460)` beside `v2[1] = (void *)dat_2b460;` in `ls` would declare a
`sigset_t` the next statement reads as a pointer, and a direct `int` read beside
an `unsigned int *` use would change what a compare on the direct read means once
the header declares the global; and for a pointer to code. An `undefinedN` direct
read is accepted against an unsigned integer of the same size, since that is the
C type the export's prelude defines it as. A `void *` constant needs one more
thing: the declared object's pointer must convert to `void *` exactly as the
cast did, which C does for an argument, a returned value, a copy or store into
`void *` storage and an equality test against another `void *`, and does not for
a compare against a pointer of another type. An address the function only ever
uses through `void *` is declared at the one type it reads the storage at
directly (`Plan::direct_type_at`), or as the unknown byte.

**The declaration.** `decompile_drive.rs (extract_global_objects)` reads back
off the printer, after `print_c`, every address the function named and every
piece of unnamed program data it read or wrote directly, each with the C
declaration the structure-member declarator builds for it (`printc.rs
(declaration_text)`), its size, and whether it stood for `void`, was a direct
access, or is a record or union. They travel as `FuncResult::globals`, which
the `--jobs` wire carries like the type records, and
`decompiler/crates/kuna-console/src/project.rs (global_declarations)` turns
them into one `extern` line per named address in the `decompile-project`
header, in a block of its own between the type definitions and the
prototypes. The header can hold one declaration per address, and every
function declares the object at the type *it* uses it at, so the choice is
constrained by what the declaration does to the other functions: it is what
every direct `dat_<addr>` read and write compiles against, and a scalar of
another type than theirs would silently change what they compute — a signed
compare turned unsigned, a store truncated — where a pointer of the wrong
type in a body that takes the address is a diagnosed mismatch. So a record or
union some function takes the address of wins, the larger first (a scalar
access of it does not compile at all); otherwise the one type the direct
accesses agree on; otherwise, when the program reads the address directly at
two types, nothing is declared and a comment says why; and with no direct
access, a type is preferred to the unknown byte a `void *` use stands for,
then the larger object, then the declaration more functions make. Every other
type is listed in a comment on the line. Over the 45 exports of the cast
corpus that is 891 declarations, 173 of them with such a comment and 60
addresses left undeclared; `gcc -fsyntax-only` reports 36,471 errors against
36,648 without the option, because direct reads of a now-declared name
compile, and 15,348 warnings against 14,962, the difference being the
incompatible-pointer warnings of those 173 addresses and pre-existing
diagnostics a declared name lets the compiler reach. An array is not a record
for this purpose: it decays to a pointer, so a scalar compare against it
compiles to something else. `project.rs (names_any_type)` reads the
declarations too, so a `struct_N` that only a global names is still redone by
the `structsynth` convergence sweep and kept by the header's type pruning.
The directly read `dat_<addr>` names are still not declared on their own
account; only an address some function takes is.

**The value is the binary's.** `decompiler/crates/kuna-cli/tests/decompile_all_cli.rs
(a_constant_address_named_as_a_global_round_trips_through_the_printed_c)`
exports a non-PIE fixture whose data has no symbols both ways, compiles each
witness caller exactly as printed against the export's own header with gcc and
clang, links it with every `dat_<addr>` placed at `<addr>` and the fixture's data
mapped where the binary keeps it, and requires the printed callers to compute
what the binary computes, with the option on and off: a record, two scalars, a
table and its one-past-the-end, a `void *` libc argument, a pointer compare and
a `char *` that is not a string, and four controls that keep the cast.

**Measured.** Over the 45-binary cast corpus (coreutils `fmt`, `ls`, `sort`, `du`,
`cp`, `tail`, `wc`, `grep`, `gzip`, the four diffutils, `tar` and `find`, each at
O0, O2 and O2-noinline) every one of the 1,883 lines the option changes is the
`(T *)0x<addr>` → `&dat_<addr>` substitution and nothing else, removing 1,979
casts; with the option off the output is byte-identical to the build without it.
On the 4,815 functions kuna and IDA both emit, casts fall from 45,126 to 44,001
and no function gains one. Variables and types are untouched, so `type_match`
cannot move (1,609 perfect functions in both arms of the 444-slice sweep).

