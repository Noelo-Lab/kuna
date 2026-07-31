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
`pointer-notation`, `condition-form`, and `external-refinement` — plus, via the `presentcompare` group row, the P9 half of the P3-declared `comparison-canonicalization` decision
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
normalization defaults (DIV-33 brace placement, DIV-34 NULL printing,
DIV-35 compound assignments, DIV-36 truthy conditions) in `docs/history.md`.

**Condition form (P9/`condition-form`, `option truthycond`).** In boolean
contexts — an if/while/for/ternary condition, or an operand of `&&`/`||`/`!`
— a comparison against zero carries no information beyond the value's own
truthiness, so the kuna default (DIV-36) renders `if (x != 0)` as `if (x)`
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
answer means no cast — only the constant-suffix marking runs. Otherwise the
machinery avoids stacking tokens: a value already produced by a CAST is
retyped or bypassed rather than double-cast; a constant is simply retyped;
and a pointer-to-struct being read as pointer-to-its-first-field inserts a
`PTRSUB(ptr, #0)` — rendered as `&ptr->field` / `ptr->field` — instead of a
cast (`coreaction_casts.rs (test_struct_offset0)`). Only when all of that
fails is a real `CPUI_CAST` op inserted before the reader, with an implied
unique output carrying the required type.

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

**Repairs and failure mode.** Late type propagation can invalidate the pointer
model a PTRADD/PTRSUB was built on; the driver demotes them back to raw
arithmetic (`cast_fixup_ptradd` undoes the scaling; `cast_fixup_ptrsub`
becomes COPY or INT_ADD) rather than print a field access into the wrong type.
The upstream LOAD/STORE pointer diagnostics (`checkPointerIssues`) are
warnings-only in C++, and in kuna the hook `coreaction_casts.rs
(Funcdata::cast_check_pointer_issues)` is a faithful no-op — a missing
diagnostic comment, never a changed expression. When the cast strategy loses,
the failure is always cosmetic: a spurious `(int4)` token or a missing one —
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
render `dest = cond ? a : b` (`printc.rs (PrintC::emit_block_if_ite)`).

**The declined-structure shell.** If the structured tree is *absent* (S8
produced no `sblocks` — a structuring failure), the printer does not emit a
flat op listing: it keeps the brace-matched prototype shell and plants the
single comment `/* WARNING: structured blocks unavailable (structuring
declined at a stub) */` in the body (`printc.rs
(PrintC::emit_function_document)`). The failure mode is deliberately loud and
syntactically valid, so batch consumers (`kuna decompile-all --json`) get a
parseable function with an explicit tombstone rather than pseudo-C garbage.

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

**Precedence without an AST.** Operators and leaves are not buffered into a
tree; they stream through a reverse-polish stack. `printc.rs (PrintC::push_op)`
pushes an operator's static token — the singleton table `printc.rs (tokens)`
carries each C operator's precedence, associativity, arity stage, spacing, and
its negated complement — and *at push time* decides parenthesization by the
pure predicate `printlanguage.rs (parentheses)`: compare the enclosing token's
precedence/associativity/type against the incoming one, with special stages
for pre/post-surround tokens (calls, subscripts, casts) and the (kuna, DIV-1,
GH-2786) rule that adjacent identical `-`/`+` prefix tokens always
parenthesize so they cannot merge into `--`/`++`. `printc.rs
(PrintC::push_atom)` emits a leaf and then unwinds every operator whose
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
prototype (kuna DIV-33 — upstream's `skip_line` default leaves a blank line
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
(the `NULL` token for pointer zeros — kuna DIV-34 flips it default-ON, so a
null pointer renders `NULL` where upstream renders `(type *)0x0`; `option
nullprinting off` restores the casted form, exercised by
`tests/stages/kuna-cnorm-nullprint.xml`), `option inplaceops` (kuna DIV-35
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
int`/`unsigned long`, pointer-to-unknown as `void *`) at the declarator/cast
chokepoints, without touching the actual data-type lattice — `option
realtypes off` restores `undefined<N>`. Per-symbol format assertions
(`map convert`, `force datatype`) override the global format at the same
point ([docs/options.md](../options.md)).

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

**Where names bind vs. where they render.** The *assignment* is a P6 pass —
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionNameVars)` binds one name per HighVariable (symbol-derived where a
mapped/global symbol exists, e.g. the DIV-24 DWARF data-global names; the
sequential default otherwise) — and P9 only *consumes* the binding: the leaf
push of §9.2 renders every member of a HighVariable through the one bound
name, which is also what keeps a register/global copy-shadow merge reading as
a single variable. The angr scheme's second visible artifact is P9-owned: each
local declaration gains a trailing storage comment — `// rax` (register,
lowercased), `// stack - 0x10` (frame-relative signed offset), `// rdx:rax` (a
join value's register pieces), `// tmp` (an SSA temporary with no machine
home) — rendered by the rules of `kuna_naming.rs (kuna_storage_comment)` from
the declaration representative's storage; `option namestyle ghidra` emits no
storage comments. DIV-5 re-pinned 185 of the 675 upstream datatest assertions
to the angr names; `option namestyle ghidra` reproduces the pre-DIV-5 bytes.

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
once): a declaration is suppressed **only** when its fully rendered signature —
final declarator type, name, array adornment, and (under angr naming) the
storage comment — is byte-identical to one already emitted
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_dedupvardecls.rs
(DeclDedup)`). Keying on the rendered bytes makes the collapse provably
lossless: two same-named locals at different slots or types differ in
signature and both survive, so a genuine collision stays visible. `option
dedupvardecls off` restores the one-line-per-HighVariable rendering.

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
results — keyed by address (`stringmanage.rs
(StringManagerUnicode::get_string_data)`). Rendering escapes per codepoint:
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

## 9.7 Whole-program document renders (`kuna decompile-project`)

**(kuna) Three additive render surfaces** back the `kuna decompile-project`
project export (the CLI driver is
`decompiler/crates/kuna-cli/src/decompile_project.rs`; usage in
`docs/agents.md`). All three are pure *readers* of finished state — they run
after analysis, insert no ops, flip no options, and change no byte of any
existing render path (the datatest / stages / `decompile-all --json` outputs
are untouched), which is why none carries a `phases.toml` row or a DIV entry.

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
no emitter markup exists for type definitions). Two documented `(kuna)`
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

**The prototype — one token stream, two documents.** The prototype segment of
§9.2's document walk was extracted verbatim into `printc.rs
(PrintC::emit_prototype_declaration)` — pure code motion, byte-identical
inside `emit_function_document` — so `printc.rs (PrintC::doc_prototype)` can
drive the IDENTICAL token sequence standalone: the same
`set_output_stream()` → emit → `output_str()` capture harness as
`doc_function_full`, plus a trailing `;`, minus the header warning comments.
The contract this buys the export: the `.h` prototype minus its `;` matches
the `.c` definition line **token-for-token** — there is no second prototype
printer to drift. The public driver is `decompile_drive.rs
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
