# 05 — Types

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p5_types
  - decompiler/crates/kuna-decomp/src/substrate/dtype.rs
```

Phase 5 computes the **fact fabric** over the SSA graph: a data-type on every
live Varnode, plus the value facts the other phases consume — circular value
ranges, non-zero masks, and consume bits. None of it runs as a standalone
stage. The type-inference and constant-pointer actions sit *inside* `mainloop`
(00-overview §0.6), between heritage and the structuring tail, and iterate to
mutual quiescence with SSA simplification (chapter 03), prototype recovery
(chapter 04), and the variable model (chapter 06) — the Band-B fixpoint. The
exact slot order is `decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`: dead-code (consume bits) → non-zero masks → type inference →
the `stackstall` rule pools → block structure → constant-pointer recovery → the
`oppool2` pointer-arithmetic pool.

Option defaults, tiers, and flip guidance for every option named below live in
the generated catalog ([docs/options.md](../options.md)); the rows are defined
in `decompiler/crates/kuna-decomp/phases.toml` and the type-phase default
divergences are DIV-2 in `docs/history.md`. Untagged prose is the
Ghidra-derived port; `(kuna)` marks kuna-original passes (each named with the
upstream GH issue that inspired it, per its `phases.toml` row).

## 5.1 Type representation

The type *data model* lives in the substrate —
`decompiler/crates/kuna-decomp/src/substrate/dtype.rs` — not under `p5_types`,
because it is shared IR: every Varnode carries a `Datatype` from the moment it
is created, and every phase from lift to emission reads it. What lives in
`p5_types` is the *inference* — the passes that decide which type a Varnode
carries.

**The metatype lattice.** Every type reduces to one of 18 meta-types
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (type_metatype)`),
transcribed with explicit discriminants because **the numeric order is the
specificity order**: lower is more specific, from `TYPE_PARTIALUNION` (0)
through struct/enum/array/pointer/float down to `TYPE_UNKNOWN` (15),
`TYPE_SPACEBASE` (16) and `TYPE_VOID` (17). Propagation and cast decisions
never compare metatypes directly; they go through the 24-value refinement
`sub_metatype` (same file), which splits e.g. `TYPE_PTR` into
plain/relative/into-struct pointer ranks and `TYPE_INT` into
char/unicode/enum/plain ranks. `Datatype::type_order` — the single gate the
inference engine uses (§5.2) — resolves to `Datatype::compare` with a recursion
budget of 10 levels; past the budget, identity falls back to the interned type
id, so comparison of deep recursive structures terminates. A separate
`type_order_formal` de-prioritizes partial unions and `bool` when *choosing a
declared type* (a value that merely behaved boolean should not out-compete a
real integer type). Note what the ranking says about signedness: `sub_metatype`
puts `SUB_UINT_PLAIN` (16) ahead of `SUB_INT_PLAIN` (17), so `uint` is strictly
more specific than `int` and a single unsigned vote on a Varnode outranks every
signed one — an upstream ordering kuna transcribes verbatim and does **not**
change, because every propagation edge and every cast decision in this chapter
is calibrated against it. The consequence (an optimized counter declared
`unsigned` and then cast back to `int` at each comparison) is re-decided at the
declaration seam instead, where it changes no Varnode type and no cast: see
`option signedness` in §9.3. The third enum, `type_class`, is not a lattice at all: it
classifies types for parameter-storage assignment (general/float/pointer/
hidden-return/vector, plus four architecture-specific classes) and belongs to chapter 04's prototype models.

**One struct, no inheritance.** The C++ `Datatype` hierarchy
(`TypePointer`/`TypeArray`/`TypeStruct`/…) becomes a single `Datatype` struct
carrying the shared members (id, size, flags, name, metatype, submeta,
alignment) plus a `DatatypeKind` payload enum
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (DatatypeKind)`) with one
variant per C++ subclass; methods match on the kind to reproduce virtual
dispatch. The variants worth knowing:

- `Struct` carries ordered `TypeField`s plus a separate `TypeBitField` list for
  sub-byte fields (§5.6); `Union` carries fields that all start at offset 0 and
  is never accessed directly — every read/write goes through resolution (§5.4).
- `Enum` carries the value→name map; rendering a constant as an OR of enum
  names is an emission concern (`EnumRepresentation`, same file). Two
  constructors: `get_type_enum` builds at the factory's configured enum width,
  which `setup_sizes` derives from the architecture default (8 bytes on x86-64) —
  the C parser's notion of "an enum" — while `get_type_enum_sized` takes an
  explicit width and signedness, for an enum whose declaration states its own
  (a C `enum` is normally `int`-sized, and DWARF records it; a type whose size
  disagrees with the storage it describes will not bind to it, and a wrong size
  is worse than a missing name). Filling the map is a re-intern
  (`set_enum_values`), so the value map must be complete before installation:
  constructing a same-named enum a second time is rejected as an altered
  definition, not treated as a no-op, and a recovery pass that sees the same
  declaration once per compilation unit has to look before it builds.
- The `Partial*` variants (`PartialStruct`/`PartialUnion`/`PartialEnum`) stand
  for a byte-slice of a container — the type a Varnode gets when it holds only
  part of a struct/union/enum — each carrying the container, the byte offset,
  and a `stripped` plain type to fall back on when a formal type is required.
- `PointerRel` is a pointer *into the middle* of a container (parent + offset),
  ranked as a distinct pointer sub-metatype so a mid-struct pointer never
  unifies silently with a plain pointer to the same field type.
- `Spacebase` treats an entire address space as one struct whose "fields" are
  the symbols mapped in it. This is the pivot type of both constant-pointer
  recovery (§5.2) and stack-frame typing: a `TYPE_PTR` to `TYPE_SPACEBASE` is
  "pointer into the frame/globals", and member lookup on it is symbol-table
  lookup.
- `Code` optionally carries a full `FuncProto`, so a function-pointer call
  through it can type its arguments.

**The factory.** All types are interned:
`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (TypeFactoryImpl)` is the
per-architecture container behind the `TypeFactory` trait handle. Every
constructor (`get_base`, `get_type_pointer`, `get_type_array`, …) builds a
candidate and de-duplicates it through a `BTreeSet` ordered by
`compare_dependency`-then-id (`TreeKey` in the same file) — the C++
`DatatypeCompare` semantics, which is why structurally identical types are
pointer-equal and `Rc::ptr_eq` is a valid fast-path everywhere. A 9×8 cache
matrix holds the atomic types (sizes 0–8 × the eight meta-types from
`TYPE_FLOAT` up), with special slots for 10- and 16-byte floats and the char
types. Two policy knobs are read from the compiler spec's data-organization:
the primitive sizes/alignments, and `max_basetype_size` — a `get_base` request
larger than it does not invent a giant integer, it returns an **array of
unknown bytes** of the right size, which is the honest statement of what is
known. `get_exact_piece` answers "what type is the size-N slice at offset K of
this type" (the symbol-piece seed of §5.2); `concretize` maps residual
`TYPE_UNKNOWN` onto concrete integer types where a formal type is forced.

The interning tree's order is a dependency *comparison*, not a dependency
*ordering*: `compare_dependency` ranks by sub-metatype then **descending**
size, so a struct contained by value sorts *after* the struct that contains
it — walking the tree front to back is not definition-before-use. The
explicit fix is `decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(TypeFactoryImpl::dependent_order)`, the C++
`TypeFactory::dependentOrder`/`orderRecurse` port: a postorder DFS over each
type's typedef base (`get_typedef`) then its component sub-types
(`get_depend`), marked on `Rc` identity so a pointer cycle
(`struct A { struct B *b; }` / `struct B { struct A *a; }`) terminates with
each type listed exactly once. Its consumer is emission — chapter
[09](09-emission.md) §9.7's `doc_type_definitions` walks the list front to
back — and the two inline unit tests (`dependent_order_nested_struct`,
`dependent_order_pointer_cycle`) pin both facts: the raw tree order really is
container-first, and the DFS reorders it.

**The data organization.** The factory also carries the target's C scalar
widths, decoded from the compiler spec's `<data_organization>` by
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(decode_data_organization)` and completed by
`decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(TypeFactoryImpl::setup_sizes)` for whatever the spec left unset. kuna reads the
full set — `char`, `short`, `int`, `long`, `long long`, pointer, `wchar_t`,
`float`, `double`, `long double` — because knowing a value's *size* is not
enough to name its C type: an 8-byte integer is `long` under LP64 and
`long long` under ILP32 or LLP64, and the fleet disagrees on both (x86-64 gcc
declares `long_size` 8, x86-64 Windows declares 4 with 8-byte pointers). These
widths are a naming fact, not an inference input; the type tree interns by
sub-metatype and size alone and never consults them.

Two of them do not describe anything `sizeof` can measure. `<long_double_size>`
records the *value* width, not the storage width — the x86 specs say 10 for the
x87 extended format and annotate the storage in a comment
(`<!-- aligned-length=16 -->`) — and a spec that declares no `long double` at
all means the target aliases it to `double` (MSVC, ARM32), which is the fallback
`setup_sizes` applies. Consumers must therefore treat a long-double width as an
approximation to name, never as a layout guarantee. Thirty-seven of the 107
vendored cspecs carry no `<data_organization>` element whatsoever — every
PowerPC 32-bit one among them — so the fallbacks are the common case, not the
exception.

**The wire marshal.** Types cross the ghidra-mode wire in both directions.
Inbound, registerProgram's `<coretypes>` and per-miss getDataType answers
decode through `decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(decode_core_types, decode_type, find_by_id_or_remote)`. Outbound — the
Phase-4 `decompileAt` response — every type reference marshals through the
`Datatype::encodeRef` port (`decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(Datatype::encode_ref)`): a type with a nonzero id (and non-void metatype)
travels as the compact `<typeref name id>` — a variable-length base emits the
size-independent id (`Datatype::hash_size` is reversible, and both
`get_unsized_id` and `has_same_variable_base` now complete through it) plus
the instance size — and Java's `PcodeDataTypeManager.decodeDataType` resolves
the (name,id) pair, which for wire-delivered types originally CAME from Java,
so the echo is exact by construction. An id-less type (a derived
pointer/array, an invented unknown) travels as the full `<type>` form
(`Datatype::encode`): `encode_basic` writes name/unsized-id/size/metatype
plus the alignment (composites), core/varlength/opaquestring flags and the
display format, and each `DatatypeKind` arm reproduces its C++ subclass
override — pointer/array descend ONE level by reference, struct interleaves
`TypeField::encode`/`TypeBitField::encode` by byte offset, enum re-spells its
metatype `enum_int`/`enum_uint` with one `<val>` child per name, a typedef
collapses to `<def name id>` + the referent's reference, and `PointerRel`
encodes its pointed-to type in FULL (the one place the C++ does) plus the
parent by reference and the `<off>` child. The differential guard is the
decode side itself: `wire_encode_roundtrips_through_decode_type` (same file's
tests) asserts that whatever `encode_ref` emits, `decode_type` resolves back
to the *identical interned `Rc`* — the same intern-equality contract §5.1's
factory provides in-process.

## 5.2 Inference

Type recovery is **off** for the entire first `fullloop` iteration — early
simplification should not chase types that heritage and prototype recovery are
about to invalidate. The arming switch is
`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionStartTypes)` at the tail of `fullloop`: it flips the function's
type-recovery flag and reports a change, forcing at least one more `fullloop`
pass with inference live.

**One pass of the engine.**
`decompiler/crates/kuna-decomp/src/p5_types/coreaction_infertypes.rs
(run_infer_types)` executes the bounded bidirectional lattice in four steps:

1. **Seed** (`build_localtypes`): every live Varnode gets a *temporary* type
   from purely local evidence. A type-locked Varnode is its own seed (and a
   hard wall — nothing propagates over it). A Varnode covered by a type-locked
   symbol gets the exact byte-slice of the symbol's type
   (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
   (build_localtype_seed)` → the factory's `get_exact_piece`). Everything else
   asks its defining op and each reading op for a suggestion
   (`output_type_local` / `input_type_local`, same module) and keeps the most
   specific by `type_order`. The suggestions come from the per-opcode table
   `decompiler/crates/kuna-decomp/src/p5_types/typeop.rs (type_op_info)` — the
   port of the C++ `TypeOp` `inst[]` registry, which bundles each opcode's
   p-code property flags, display attributes, and local input/output
   meta-types — with two live upgrades: a CALL/CALLIND whose resolved callee
   prototype is committed suggests the callee's real return/parameter types,
   which is how a typed argument reaches the caller's stack Varnode.
2. **Propagate** (`propagate_one_type`): from each seeded Varnode, a DFS walks
   the def-use graph in both directions, at each edge asking the op's transfer
   function for the outgoing type (`propagate_type`). A pushed type is adopted
   only if it is **strictly more specific** than the target's current temporary
   (`0 > type_order`), and each Varnode is expanded at most once per walk (a
   mark bit), so a single pass is linear-ish and cannot ping-pong.
3. **Returns** (`propagate_across_returns`): unless the output prototype is
   locked, the most specific temporary among all `RETURN` value inputs is
   re-seeded onto the other returns, so one well-typed exit types them all.
4. **Write-back** (`write_back`): temporaries become permanent Varnode types;
   every change dirties the owning HighVariable (chapter 06 recomputes lazily)
   and marks the pass "changed".

The transfer functions are the heart. COPY/MULTIEQUAL/INDIRECT propagate
identity (input↔output only); the unsigned comparisons propagate input↔input;
the *signed* comparisons propagate only `TYPE_INT` ("only propagate signed
things" — a pointer compared signed must not become signed); `INT_ADD`
propagates a pointer across the add while accounting for the constant offset;
LOAD/STORE convert between pointer and pointee (`propagate_to_pointer` /
`propagate_from_pointer`); PTRADD/PTRSUB walk the pointed-to composite through
`TypePointer::downChain` (`propagate_add_in2_out`) so array/member arithmetic
yields field pointers; XOR/AND propagate only enums and float
sign-manipulation idioms, OR only enums; PIECE/SUBPIECE map between a composite
and its byte-slices (producing `Partial*` types); SEGMENT resizes pointers;
NEW takes its type from the constant pool. Everything else refuses —
propagating through an opcode with no sound transfer is how type garbage
spreads. A `TYPE_BOOL` additionally refuses to land on any Varnode whose
non-zero mask (§5.3) admits values above 1. When the incoming type is a union
(or pointer-to-union), the edge first resolves a concrete facet via
`decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
(Funcdata::resolve_in_flow)` (§5.4) — except across MULTIEQUAL/INDIRECT
markers, where the unresolved union flows on so one phi input cannot lock the
facet for all of them.

The S5→S6 feedback lives at the end of the pass: `propagate_spacebase_ref`
finds the stack-pointer input and pushes recovered pointer types (e.g. a
`mystruct *` argument) *through* spacebase arithmetic onto the addressed
stack-frame Varnodes (`propagate_ref`), which is what turns "pointer typed as
`mystruct *`" into "stack local declared `mystruct`". The pass opens by
committing any pending stack-symbol type recommendations
(`decompiler/crates/kuna-decomp/src/p6_variables/funcdata_spacebase.rs
(apply_type_recommendations)`).

**The bounded pass count.** The wrapper
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionInferTypes)` runs the engine once per `mainloop` iteration and counts
only pass runs where `write_back` reported a change. The counter is capped at
**7 passes** per (re)start: on the 7th it stops, records
`set_type_recovery_exceeded`, and the upstream "Type propagation algorithm not
settling" condition holds. The ceiling exists because the lattice itself is
monotone but the *environment* is not — rule-pool rewrites between passes can
keep presenting new ops, and a pathological int↔pointer disagreement can
alternate forever. Failure mode when the ceiling hits: types freeze at the
last settled state, and downstream pointer-arithmetic rewrites self-type their
new ops directly — `decompiler/crates/kuna-decomp/src/substrate/addtreestate.rs
(assign_propagated_type)` fires whenever `is_type_recovery_exceeded` holds —
since no further inference pass will visit them. The budget is compile-time and
deliberately latent (the `solver-budget` row in
`decompiler/crates/kuna-decomp/phases.toml`, strength HINT).

**The pointer-nesting cap (`ptrdepthcap`).** One shape reaches the ceiling by
construction rather than by pathology: a small-string-optimized C++ object.
Such an object keeps *either* the characters *or* a pointer to them in the same
first 8 bytes, chosen on a capacity field, so the compiler emits a MULTIEQUAL
whose two inputs are `PTRSUB(spacebase, -0xN)` — typed pointer-to-the-mapped-local
by the spacebase arm of `propagate_add_in2_out` — and a LOAD from that very
address, typed as the local itself. That is the equation `T = ptr(T)`, which no
finite type satisfies, so each pass adopts a type exactly one pointer level
deeper than the last and the object ends up declared `unsigned long long *****`.
Upstream refuses to build such a chain at the one seam it noticed —
`TypeFactory::getTypePointerNoDepth`, used by the LOAD/STORE transfer functions —
but the spacebase-PTRSUB arm that actually drives the escalation never routes
through it. When `ptrdepthcap` is on (shipped OFF in the catalog, ON in the
`aggressive` preset), every candidate the propagation is about to adopt is put
through `decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrdepth.rs
(cap_pointer_depth)`, which applies that same upstream rule at the single
`propagate_type_edge` funnel: a candidate whose target is itself a
pointer-to-pointer collapses to `ptr(undefined<N>)`, and `ptr(ptr(undefined<N>))`
collapses one more level when `N` is the pointer width. `ptr(undefined<N>)` is a
fixed point of the rule, and it is *less* specific than the concrete pointer
already held, so the `0 > type_order` test rejects it and the lattice settles
instead of running to the ceiling. Depth 1 and depth 2 over a concrete base are
untouched, so a genuine `char **argv` keeps its spelling.

**The `code` pointee (`codescalar`).** The generic `code` type is built with
size 1, so that `code *` arithmetic steps one byte at a time, and
`propagate_from_pointer` decides what a dereference yields by testing only
whether the pointee's size equals the access width. Those two facts together
say that a one-byte read through a function pointer yields a *value* of type
`code`. It does not: `code` has no width to hold, and the C back-end prints
that scalar `void`, so the local is declared `void v4; // al` and every later
use of it is cast back to a real width. The shape is ordinary in packers and
anti-debug stubs, which read and patch a byte of their own callee
(`v1 = *(code **)g; if (*v1 != 0xcc) (*v1)(); *v1 = f();`) — the LOAD types the
compared byte, the STORE types the stored one, and `INT_EQUAL` spreads `code`
to the compared constant as well. When `codescalar` is on (the shipped default,
DIV-138),
`decompiler/crates/kuna-decomp/src/p5_types/kuna_codescalar.rs
(blocks_value_type)` declines a `TYPE_CODE` pointee on the value side of the
LOAD/STORE transfer function, so the value keeps the size-correct default the
access already gives it. The same test is consulted once more in the cast
tail — `TypeOpStore`'s value cast would otherwise re-impose the pointee on the
stored value and print `(void)` — and nowhere else: the *pointer* keeps its own
`code *` type, so the indirect call still renders `(*v1)()`, and a `code **`
load, whose pointee is a pointer, is untouched.

**The dereferenced-only parameter (`ptrfromuse`).** A parameter the body only
ever uses as a memory base is declared as an integer, and which integer it is
depends on nothing more than whether the first field the compiler reads sits at
offset 0. The two halves of the engine both decline it. In the seed fold, every
reader of such a parameter is an `INT_ADD`, whose `get_input_local` is
`get_base(size, TYPE_INT)`, so the fold's candidate set contains no pointer at
all and the parameter comes out `int8`; an untyped callee that also takes the
parameter votes `xunknown8`, which is *less* specific and loses. In the
propagation, the pointer candidate does exist — `propagate_to_pointer` types the
`a0 + 8` result from the width of the access through it — but
`propagate_int_add` refuses to carry a pointer from an output back to an input
(the `inslot == -1` arm, transcribed from upstream), so it never reaches the
parameter. Both halves are measurable on one function: on coreutils `fmt` -O2 with
`libctypes off`, `get_line` runs `propagate_to_pointer` 117 times and hits that
refusal 12 times with its own first parameter as the target, and still prints
`void sub_3420(long a0, unsigned int a1)`, while its sibling `put_word`, which
dereferences the same kind of argument at offset 0, prints
`void sub_3000(unsigned long *a0)`. (With `libctypes` at its default the same
parameter is already named `FILE *` from the callee tables, which is why that
pair witnesses the *engine* behaviour rather than the shipped default.)

When `ptrfromuse` is `byte` or `void` (shipped `void`),
`decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrfromuse.rs
(pointer_from_use)` supplies the missing candidate directly. For each function
*input* Varnode whose width equals the default data space's address size — eight
bytes on the 64-bit images the option is named for, four on a 32-bit image, and
the reason `get_type_pointer` is never asked to mint a pointer of the wrong
width — and whose storage the prototype model says could carry a parameter (a
segment-base register such as x86-64's `FS_OFFSET` is a function input too, and
typing it defeats canary recognition), it walks the transitive descendants with a
bounded **breadth-first** worklist — a visited set and a ten-hop cap, so a
loop-carried MULTIEQUAL is entered once, and the cap is a property of the graph
rather than of the pop order — through `COPY`/`MULTIEQUAL`/`INDIRECT` identity and
through an `INT_ADD` whose other operand is a literal, and offers a pointer when
at least one terminal use is the address operand of a `LOAD` or `STORE`. It
refuses outright on any use no pointer survives: `INT_MULT`, the four
divide/remainder opcodes, `INT_2COMP`/`INT_NEGATE`, any shift, a `PIECE` that
assembles the value out of halves, any float opcode, a comparison against a
non-zero literal, and a `CALL` whose callee prototype has already committed that
argument to a non-pointer metatype. An `INT_ADD` with a *variable* addend is
neutral — it neither carries the walk nor refuses it — because either operand of
`x + y` could be the base, which is also why an induction variable used to index
a buffer never becomes a pointer through this rule. `byte` points at one unknown
byte, which the unknown-type rendering of §5.5 spells `char *`; `void` points at
nothing.

A literal addend is where the rule's one structural ambiguity lives, and it is
worth stating plainly. `p->field` and `table[i]` lower to the *same* `INT_ADD`
of a value and a constant; only the constant says which operand is the base. So
the walk asks `ActionConstantPtr::isPointer`'s own question of the addend, in the
same three steps: the default data space's pointer bounds, `resolve_constant`
into that space, then the global scope. The answer has three grades, and the walk
treats each differently.

* A constant that resolves to a **global object** kuna knows about is the base,
  so the walked value is a subscript, and the whole candidate is **refused**.
  Without this, `char table[256]; int lookup(long i){ return table[i]; }` is
  declared `int lookup(char *a0)` and prints `a0[0x4040]` — a wrong declaration,
  and the named global `table` replaced in the body by its raw address.
* A constant that is **address-like but names nothing** settles nothing, so it is
  neutral, exactly like a variable addend: it neither carries the walk nor
  refuses it. This is the grade a stripped image's `.bss` table falls into (no
  symbol at all), and also the grade a genuinely large struct falls into — bzip2's
  `bzFile` is 0x13f0 bytes and its fields are read at `+0x1394` and `+0x13e8`. A
  parameter with no other use is declined either way; one the function *also*
  dereferences at an ordinary field offset keeps its candidate from that use.
* Anything below the bound is an ordinary field offset and the walk carries on.

So the residual exposure is precise: a parameter used *only* to subscript an
unnamed byte-element table would be declined (nothing tells the walk it is a
base, but nothing gives it a base either), and a parameter used both as such a
subscript and as a real memory base is typed on the strength of the second use.
`tests/stages/kuna-ptrfromuse.xml` pins both grades, as `globalindex` and
`blindindex`.

The candidate is **folded** into `get_local_type`'s result by
`Datatype::type_order`, not installed as a replacement seed. That is the whole
reason a named type survives: a `FILE *` arriving from a callee's locked
prototype is `SUB_PTR_STRUCT`, the candidate is `SUB_PTR`, and the fold keeps the
more specific of the two, so `libctypes` (chapter 01) and DWARF both outrank this
rule wherever they have an opinion. A replacement seed at the same place would
silently win instead. Only function inputs are considered, which bounds the
change to one declaration per function; the declaration is what moves, but the
body moves with it, because once the parameter is a byte pointer the
pointer-arithmetic pool rewrites `*(int *)(a0 + 8)` into `*(int *)&a0[8]`.

The shipped value is `void`. A `void *` parameter keeps the byte arithmetic of
its field accesses, with a cast on the base (`*(long *)((long)a0 + 0x28)`),
where `byte` rewrites them into indexing; and it is the type that reaches the
most declared parameters. Through `protoorder` (chapter 04) a callee's recovered
parameter type becomes a vote on the matching argument at each call, so a
function that only forwards its arguments to a callee that dereferences them is
declared with the callee's `void *` — the shape of every `qsort` comparator and
hash-table callback, whose declared parameters are `void const *`. On the
444-slice decbench sweep `void` moves 204 functions onto a perfect `type_match`
(all of them `ls`'s sort comparators, in the three programs built from that
source at three optimisation levels) and 85 more up, `byte` moves 7, and
neither moves any function down; arity, argument and variable counts are
unchanged. The two costs are textual: every field access through the parameter
gains the `(long)` cast, and where the retyped value no longer merges with a
temporary the printer may fold that temporary into its uses — an address
computation or a load recomputed at each reader. `ActionMarkImplied`'s crossing
test still decides where that may happen: never across a call, and across a
store only when the two addresses provably differ. `off` is the upstream seed
fold exactly.

**A libc aggregate the caller reaches past (`libctypes`).** `TypeOpCall`'s
`get_input_local` turns a locked callee parameter into a vote about the argument
Varnode, and `libctypes` (chapter 01) locks many of them to a named libc
aggregate: `_obstack_newchunk(obstack *, size_t)`, `pthread_mutex_lock(pthread_mutex_t *)`.
The vote is right when the argument points at that aggregate and wrong when it
points at an object whose first member is one, because the two addresses are the
same value. grep's `struct kwset` begins with its obstack, so
`kwsincr(kwset_t, char const *, idx_t)` was declared `obstack *a0` and every other
member rendered as an element of an obstack array: offset 0x980 of the 88-byte
struct printed `&v1[0x1b].field_0x38`. An rsyslog instance record that begins
with its mutex printed `a0[9].field_0xc`.

`decompiler/crates/kuna-decomp/src/p5_types/kuna_libcfit.rs (overruns)` asks what
the caller does through the argument before the vote is taken. It walks every
load, store and derived address reached from the argument Varnode through its
descendants, with the walker `protoorder` holds its own recovered votes to
(`accesses_through` in
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs`), and declines
when any of them lies outside the aggregate: an access at a constant offset past
its end or before its start, a derived address at or past the end (rsyslog hands
`&v1[1]`, one mutex past the one it initialised, to `pthread_cond_init`), or a
step at least as large as the aggregate that is not exactly its size. coreutils
`wc` is the stepped case: it walks `&fstatus[i].st`, a `stat` 8 bytes into a
152-byte record, and reads `failed` at -8; libselinux indexes 120-byte records
that hold a 40-byte mutex at +0x38, and 120 being three mutexes does not make
them an array of mutexes. The same question guards propagation
(`kuna_libcfit::refuses`, called from `propagate_type_edge` just before a type is
adopted): a libc aggregate pointer is not carried onto a Varnode whose own
descendants reach past the end, whichever call it came from. A declined vote
falls back to `void *`, what the shipped width-stable tables say for the same
slot, so the argument settles on whatever other evidence it has (grep's
`kwsprep` prints `unsigned long *a0`, exactly as with `libctypes off`). The
declared type is untouched and is still what `ActionSetCasts` measures the call
against (`declared_input_type_local`), so a value that settles on another type
passes with a cast, `stat(v10,(stat *)v15)`, and a `void *` needs none. What
proves nothing is left alone: a step smaller than the aggregate, which is an
unknown index, a word-at-a-time struct copy (grep `main` copying its stack
`stat`) or a phi between two fields (stty's `termios *mode`); an array of the
aggregate itself (sdiff's `struct sigaction` table, stepped by exactly 152); and
a walk that runs out of budget. Walking descendants rather than the whole COPY
family keeps the answer per Varnode, but not per source variable: at `-O0`
gnulib's obstack macros copy `&kwset->obstack` into locals of their own (`__h`,
`__o`), and once copy propagation has made every call read `kwset` itself, those
locals only ever hold a copy of it. They took `obstack *` by propagating it
down from `kwset`, and with `kwset` declined nothing gives it to them, so grep
`-O0`'s `kwsprep` loses those locals' names along with the wrong one. The rule speaks
only about the names `libctypes` can mint or adopt (`AGGREGATE_NAMES` in
`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_libctypes.rs`), only while
that option is on (`ArchContext::libctypes`), and only about the vote. It never
touches a type that arrives by another road: a call's OUTPUT (`getpwnam`'s
`passwd *`), or propagation between Varnodes that are later merged into one
variable, which is how bash's `glob_vector` still prints one `DIR *` local that
also holds an unrelated `sh_malloc` result. `tests/stages/kuna-libctypes.xml`
pass 13 pins the obstack and mutex shapes and a control that keeps its name.

**What the pointer points at (`charptr`).** `ptrfromuse` decides that a value
*is* a pointer; it cannot say what is on the other end, and the shipped `void`
says so honestly. The element type is often on the table already — a callee
declares `char *` for the argument, a `%s` conversion `formatstring` resolved
names it, the object is indexed one byte at a time — and it is lost for the same
structural reason the pointer itself was: `TypeOpCall`'s `get_input_local`
states the callee's declared parameter type about the Varnode that *is* the
call's operand, and `propagate_type_edge` will not carry a pointer back over an
`INT_ADD`, a MULTIEQUAL, or the store/load pair an `-O0` spill puts between the
value a reader sees declared and the use that says "characters". GNU tar -O0
shows it on `drop_volume_label_suffix(char const *label)`, which passes `label`
to `strlen` and which kuna prints `void * sub_e680(uint1 *a0)`, the element type
coming from the byte-wide read its `isspace` test does.

When `charptr` is on (shipped `off`),
`decompiler/crates/kuna-decomp/src/p5_types/kuna_charptr.rs
(char_pointer_from_evidence)` collects that evidence with the same walk
`ptrfromuse` uses — the bounded breadth-first worklist over the transitive
descendants, a visited set, a ten-hop cap, `COPY`/`MULTIEQUAL`/`INDIRECT`/`CAST`
identity, the base slot of `PTRADD`/`PTRSUB`, a literal-addend `INT_ADD` graded
by the same three-way `ActionConstantPtr::isPointer` question — and asks of every
use whether it is about *characters* rather than whether it is a dereference at
all. Three uses answer yes.

* A **declared** callee parameter: the value reaches argument *i* of a call whose
  callee declares `char *` there, *at the base*. Declared means stated from outside the
  decompile — a `libproto`/`libcsigs` signature, a `libctypes` shell, DWARF, a
  demangled name, `--assert prototype`, or the per-call-site override
  `formatstring` installs for a resolved `%s` — and it is read through
  `declared_input_type_local`, the accessor a cast is measured against. A type
  another *recovery* voted for is deliberately not evidence: counting
  `protoorder`'s callee vote was measured on the same 444-slice sweep and scored
  lower, because a recovered `char *` is itself a guess and the walk would
  launder it into a commitment.
* A **byte-wide access at the base**: every `LOAD` or `STORE` through the
  value itself is one byte wide, or the value is indexed one byte at a time by
  an index the program computes (a `PTRADD` of element size one whose index is
  not a constant).
* A **character constant**: the value is defined by, or compared against, a
  constant that resolves to a character array in the image.

Nothing at a *fixed non-zero offset* is evidence, in any of the three. `p->flag`
and `p[0]` lower to the same one-byte `LOAD`, and only the offset tells them
apart: coreutils `ginstall`'s `announce_mkdir(char const *dir, void *options)`
reads the `bool` at `options + 0x3c`, and reading that as a character replaces a
*correct* `void *` with a wrong claim. A declared `char *` at a fixed offset says
the same thing about a field rather than about the base: cronie `crond` -O0
`sub_6715` calls `strcmp(base + 0x13, ".cron.hostname")` where `base` is a
`struct dirent *` and `0x13` is `offsetof(struct dirent, d_name)`. So the walk
carries a flag saying "this Varnode is the base plus a fixed offset", set by a
non-zero literal `INT_ADD`, a non-zero `PTRSUB`, or a `PTRADD` whose index is a
non-zero constant, and cleared by nothing; a byte access, a declared parameter
and a further one-byte step there are all neutral. The `PTRADD` half is not
optional. The same displacement arrives in two spellings: `d + 0x13` is an
`INT_ADD` while `d` is still an integer and becomes `PTRADD(d, 0x13, 1)` once an
earlier fold has typed it `void *`, so a walk that read only the `INT_ADD` would
commit on the second pass what it declined on the first. That is what turned the
`struct dirent *` of dpkg's `dpkg-trigger` -O2 `sub_58d0` and `dpkg-statoverride`
-O0 `sub_9aaf`, both of which walk `d_name` to its NUL, into `char *`. The price
is two correct idioms with exactly the shape of a field. Skipping a string's
first character before reading it, `p + 1`, looks like a field at offset one, so
coreutils `realpath`'s `path_prefix`, which advances both arguments before its
first read, `env`'s `scan_varname` and `printf`'s `print_esc` stay `void *`; and
writing a number backwards from the end of a caller's buffer, gnulib's
`umaxtostr` (`p = buf + 20; *p = 0; *--p = ...`), starts at a fixed offset, so
its `buf` stays `void *` too. A *variable* index is not an offset: `strlen(p + i)` and `p[i]` still
count, because a `PTRADD` of element size one indexed by a computed value is the
walk itself.

The refusals are what keep the rule honest: a dereference or an element step
*wider* than a byte (the program saying the element is not a character), a call
argument the callee declared as some other pointer or as a scalar, and everything
`ptrfromuse` refuses. A refusal withdraws the Varnode it was collected for and
nothing else. The walk runs once per Varnode and the vote is folded per Varnode,
while the type a reader sees belongs to the *merged variable*, so a slot walked
with byte evidence in one flow and refused in another prints the commitment:
grep -O0 `sub_6fdd`'s stack slot at `-0x20` is walked eight times with byte
evidence and four times refused, which `KUNA_CHARPTR_CENSUS=1` reports line by
line. What the refusals bound is what one Varnode's evidence may claim, not what
the merged variable may become — and they are about element *width* and
contradicting declarations, not about arithmetic shape. A constant-stride walk is
never refused: a literal displacement only marks the value as offset from its
base. The candidate is folded by `type_order` exactly as `ptrfromuse`'s is, and
it may additionally **refine** a pointer vote that points at nothing — `void *`
and `undefined1 *`, the placeholders this rule exists to resolve — while a
pointer vote at a named, aggregate or *wider* pointee (`FILE *`, `stat *`,
`long *`, a synthesized `struct_3 *`) is left as it was. Only function inputs
and Varnodes in the stack space are considered: those are the two kinds of
storage a reader sees as a declaration.

The refine guard reads the vote *in flight* — the `ct` the fold has built so far
for this Varnode — not the type the function will finally print, so a pointee
that would only have arrived by *later* propagation is not there to protect, and
"only a pointer at nothing is refined" holds of the vote, not of the output.
`unsigned char *` is the common case, and it is accepted deliberately. Where a
`uint1 *` would only have arrived by later propagation from a byte-wide use, the
`char *` vote is already sitting in the fold and wins, so `int callee(unsigned
char *a0, int a1)` in the `protoorder_x86_64` fixture becomes `int callee(char
*a0, int a1)` with the option on, and `drop_volume_label_suffix` above is the
same case. That is the same call `charbyte` makes one level down: the element
width agrees and only its signedness moves, and it is by design, because a
one-byte pointee reached through byte-only uses is what the rule exists to name.
`FILE *` next to it in the same signature is untouched. It is not only
signedness, though, and the measurement says so: over eight whole binaries three
declarations move from a *wider* pointee, all of them e2fsprogs `e2fsck`'s
`ext2fs_bitcount(unsigned int *addr, uint4 nbytes)` and its two stack copies —
a function upstream declares `const void *`. No synthesized `struct_N *`
declaration moves in that run at all.

The shipped value is `off`, and the measurement says it should stay there. On
the 444-slice decbench sweep the ground-truth class `char *` is the largest
single class — 14,645 of the 65,715 scored variables — and kuna matches 28.7% of
it against Binary Ninja's 36.8%. Split by storage, kuna is already at 64.9% on
*arguments* against Binary Ninja's 71.5%, and at 44.8% on *stack locals* against
71.4%: nine tenths of the class gap is stack locals, and within those the
dominant blocker is not a missing element type but a missing *symbol* — 932 of
the 1,648 stack misses are frame-layout slots kuna reports as `undefined8`
because the value that lived there was copy-propagated into a register
HighVariable, which chapter 06 does not export. What was left for this rule came
almost entirely from the skip-the-first-character idiom above: with a constant
`PTRADD` index counted as evidence the sweep gained 4 functions onto a perfect
`type_match` and 14 more up against 6 down (aggregate +4.93), and with the offset
rule complete it gains none onto perfect, 2 up and 4 down (aggregate -0.57 over
10,748 functions). Turning it on is a claim as well as a gain — committing a
pointer forfeits the width-only free pass an eight-byte scalar gets, and
`char *` rewrites the body's arithmetic into indexing the way `ptrfromuse byte`
does. The signedness can also move the *other* way as a knock-on — a correct
`char *` arriving as `unsigned char *`: on dash at `-O0`, 13 lines in seven
functions and four signatures go that way, two of those bodies losing a
character literal, against 23 lines that move forward and four `strcmp` casts
that disappear. `docs/features/charptr/` carries the census, the per-function
moves and that corpus classification.

**The truth-valued byte (`boolbyte`).** `TYPE_BOOL` only ever enters the
lattice as an op's *output*: every `booloutput` opcode's `get_output_local` is
`get_base(size, TYPE_BOOL)`, and `CBRANCH`'s slot-1 `get_input_local` is the
same. Nothing mints it for the value being *compared*.
`TypeOpEqual::get_input_local` votes `get_base(1, TYPE_INT)` for that, and on a
one-byte request that is the ASCII `char`, because `cache_core_types` prefers a
character type over `int1` at the same (size, metatype) cell ("Char is preferred
over other int types", transcribed from upstream). So the seed fold for a flag
byte contains `char` and nothing else, and a value the program declared `_Bool`
is declared `char` - on coreutils `basename` -O0, `void sub_2b45(unsigned long
a0, long a1, char a2)` whose only use of `a2` is `v1 = (a2) ? 0 : 10`, where
DWARF says `_Bool use_nuls`. Propagation cannot rescue it either: `BOOL_*` has
no transfer function at all, so bool travels only by COPY/MULTIEQUAL/INDIRECT
identity, and the gate above (a `TYPE_BOOL` refuses to land on a Varnode whose
non-zero mask admits values above 1) stops it at the first edge into an
unconstrained input.

When `boolbyte` is `on` (the default),
`decompiler/crates/kuna-decomp/src/p5_types/kuna_boolbyte.rs
(truth_value_type)` supplies the missing candidate. It applies to a one-byte
Varnode that is not a constant, not type-locked, not covered by a type-locked
symbol, and not in the unique space - a temporary there is an expression the
printer renders inline, and typing it would buy nothing but the `(bool)` cast
the cast tail then has to insert to reconcile the op's own output type. The
evidence has a def half and a use half, and which halves apply depends on
whether the value is written in this function.

* **The use half** walks the Varnode's transitive readers with the same bounded
  breadth-first worklist `ptrfromuse` uses - a visited set and a ten-hop cap,
  and running out of hops refuses rather than accepting, because a use the walk
  never reached has not agreed - crossing value-preserving identity (`COPY`,
  `MULTIEQUAL`) and widening (`INT_ZEXT`), so a byte that is copied to a stack
  slot and tested there is still reached. Every terminal read must be a truth test, and at
  least one of them must be a real one: a `CBRANCH` condition, a `BOOL_NEGATE`
  / `BOOL_AND` / `BOOL_OR` / `BOOL_XOR` operand, or an `INT_EQUAL`/`INT_NOTEQUAL`
  against zero. Everything else refuses. The call argument is the deliberate
  one: `putchar(c)` must not make `c` a bool because something else also tests
  it, and a store of the value is refused for the same reason - where the byte
  goes next is not evidence about what it is.
* **The def half** applies to a Varnode this function writes. Its non-zero mask
  must be at most 1, which is `ActionNonzeroMask` (§5.3) reporting "only bit 0
  is ever set" transitively over the whole def chain, computed in the pass that
  runs immediately before this one. The mask alone is not enough - `x & 1` has
  mask 1 and is a parity test, not a flag - so the defs are walked too, and only
  a literal 0 or 1, a `booloutput` result, or a copy/phi/zero-extension of those
  counts. Once a value has passed the def half it is *proven* to be 0 or 1, and
  the use half relaxes accordingly: `x == 1`, `x & 1` and `x ^ 1` become truth
  tests (on a proven flag they are the identity and the negation), and a
  `RETURN` of the value is allowed as neutral.
* **`CPUI_INDIRECT` is not identity**, and neither half crosses it. An INDIRECT
  output is the value *after* the op it annotates - a call whose callee may have
  written this storage, or a store that may alias it - so a truth test on the
  output is no evidence about the input, and a proof about the input is no
  evidence about the output. Reading it as identity is how a byte a callee fills
  with 200 gets declared `bool`: `unsigned char c = 0; fill(&c); return c ? 11 :
  12;` proves `c` is 0 before the call and tests it after, and the two facts are
  about different values. It is also how a 131 KB `fgets` line buffer whose
  first byte is the loop's terminator test came out `bool[131088]` while it was
  still being passed to `fgets`. So an INDIRECT def refuses, and an INDIRECT
  read refuses.

* A function **input** has no def to prove anything with, and always carries the
  full `0xff` mask. For a parameter the use shape is therefore the whole of the
  evidence, and the rule says so plainly: a byte the function only ever branches
  on is what a `_Bool` parameter looks like from the inside, which is an
  inference about the calling convention rather than a proof about the value.
  This is why the rule sits behind an option that can be turned off.

The candidate is **folded** into `get_local_type`'s result by
`Datatype::type_order`, not installed as a replacement seed. `SUB_BOOL` is 10,
which beats `SUB_INT_CHAR` 19 and `SUB_UINT_PLAIN` 16, so `bool` wins against
the `char`/`uint1` votes it exists to displace and loses to anything more
specific - a callee's locked parameter type, a DWARF-completed enum. Nothing
else in the lattice moves: the `TYPE_BOOL` propagation gate is left exactly as
upstream wrote it, which is what keeps the seed from travelling along a copy
chain into a value that can hold more than 0 or 1.

What a reader sees change is usually just the declaration - `if (a2)` prints the
same whether `a2` is a `char` or a `bool` - and over sixteen stripped binaries
and 7,026 functions that is 91 of the 99 functions the option changes at all.
The other eight are worth naming, because a declaration is a type and a type
reaches the printer.

* **A constant assigned into the byte re-renders.** `is_char_print` is a
  property of the type, so `v = '\x01';` becomes `v = 1;` once `v` is a `bool`
  (`grep` -O2 `sub_9130`, `sort` -O2 `sub_7af0`).
* **A truncation into the byte re-renders.**
  `CastStrategyC::is_subpiece_cast` (cast.cc:411-432) lists the destination
  metatypes a SUBPIECE may print as a cast, and `TYPE_BOOL` is not among them,
  because upstream never puts one there. Left alone the printer falls to the
  functional arm and emits the raw `SUB41(x,0)` p-code intrinsic - an undeclared
  identifier, and not compilable C, which is the class `subright` (§3.2) exists
  to keep out of the output. The option supplies the missing arm
  (`kuna_boolbyte.rs (truncation_form)`, consulted by
  `printc.rs (subpiece_cast_form)`), so a truncation into a `bool` prints as a
  cast. Which cast depends on the operand, not the destination. A C conversion
  to `bool` tests every bit of its operand while the truncation keeps only the
  low byte, and a `bool` can reach a truncated byte whose source has other bits
  set: in `unsigned char c = x & 0x201, d = c; while (d) d = 0;` the literal
  `0` makes `d` a `bool` and propagation carries it back through the copy to
  `c`, whose own mask is 1 even though `x & 0x201` is not. So the arm prints
  `(bool)x` only when every bit of `x` above the destination is proven zero, and
  `(bool)(unsigned char)x` otherwise - `(bool)(x & 0x201)` would be 1 for
  `x = 0x200`, where the byte is 0. The proof is the operand's non-zero mask,
  re-derived at print time (`kuna_boolbyte.rs (value_mask)`) over copies,
  casts, extensions, masks, constant shifts and phis, because the stored mask is
  refreshed only inside the main loop and a Varnode created later - a cast, a
  block the return duplication cloned - still carries the all-ones default. The
  arm belongs to the option: with `boolbyte off` the printer is
  byte-for-byte what it was, including on the cases that need it already -
  a comparison alone can make a destination `bool` without this rule, which it
  does on three lines in `tar` -O2 and one each in `ls` -O2 and `du` -O0.
  Counts of the intrinsic therefore go *down* with the option on, never up:
  whole-binary, `tar` -O2 4 -> 1, `ls` -O2 1 -> 0, `du` -O0 1 -> 0, `grep` -O2
  0 -> 0; over the 100 functions the option changes at all, 7 -> 0.
* **Where a byte lands in the speculative merge moves**, because §6 merges by
  type: a `bool` and a `char` in one storage stop being merge candidates and two
  `bool`s start. So a function can gain or lose a declaration, and an expression
  can come to name a different variable. `grep` -O2 `sub_109d0` splits one
  `unsigned char` in two and the array index in
  `*(unsigned long *)(a1[0x31] + a0 * 8) = v11[v32];` comes to name the other
  half; `sort` -O2 `sub_ed80` goes from 33 declarations to 32 with an otherwise
  identical body. Seven of the eight are a declaration-count move of this kind.
  Splitting a merge is not a loss of information - the two halves were one
  variable only because §6 guessed they could be - but it does move names, which
  is why this is a judgment call behind an option rather than a fix.
* **A `(bool)` cast can appear** where the cast tail has to reconcile the new
  declaration with an op that wants an integer (two over the sixteen binaries).

The option is on by default. With the default flipped none of the 675 datatest
assertions moves and the stage corpus is PARITY OK; over the decbench type
sweep the flip moves functions onto a perfect `type_match` and none off it,
with no function scored worse (the measurement is in
`docs/features/boolbyte/record.json`, `default_on_flip`). What the default costs is
the name surface described in the last bullets but one: the functions whose
merge moves renumber their remaining `vN` locals, so a `--assert type vN` or
`--assert name vN` written against an older run can address a different
variable. `--option boolbyte off` restores the upstream fold exactly - the
candidate is never offered and the printer arm is not consulted - which is the
ablation to reach for when a `bool` declaration is in question.

`tests/stages/kuna-boolbyte.xml` pins the witness, the five refusals - a byte
that is also widened and added, a byte tested for its low bit, a byte stored
through a pointer, a stack byte set to 0 and then filled by a callee, and a byte
parameter copied into a slot a callee is handed the address of - and both
truncation renderings: the one whose first pass is the `SUB41` the printer emits
without the arm, and `truncflag`, whose second pass must print
`(bool)(uint1)(a0 & 0x201)` and never the bare `(bool)(a0 & 0x201)`.

**The char-pointer byte (`charbyte`).** x86 loads a byte with `movzx`
whether the program meant it signed or not, so `*p` lifts to a `LOAD` whose
one-byte output is read by an `INT_ZEXT`. `TypeOpIntZext::get_input_local`
votes `get_base(1, TYPE_UINT)` for that byte, and in the seed fold
`SUB_UINT_PLAIN` (16) outranks `SUB_INT_CHAR` (19), so the byte is seeded
`uint1` before any propagation. When a `char *` later reaches the `LOAD`'s
address, `propagate_from_pointer` offers `char` for the byte and the edge rule
refuses it, because `char` ranks below `uint1`; and when the byte is visited
first, it pushes `uint1 *` over a pointer that already carries `char *`, and
that push wins. Either way a string walk prints in the wrong vocabulary -
coreutils `fmt` -O2 `get_line` reads ``v1 = (unsigned char *)*dat_c100; v7 =
strchr("([\'`\"",(int)(char)*v1);``, and every gnulib `mbrtowc` wrapper takes
`unsigned char *a1` only to pass `(char *)a1` on.

When `charbyte` is `on` (shipped `on`),
`decompiler/crates/kuna-decomp/src/p5_types/kuna_charbyte.rs (note_edge)` watches
the `LOAD` edges while `run_infer_types` propagates and records the byte of
each one where the pointer carries `char *` and the byte holds `uint1` only
because of the zero-extension: an `INT_ZEXT` reads it and no other reader's
`get_input_local` votes `TYPE_UINT`. A mask, an unsigned comparison, a logical
shift, an unsigned division or an `unsigned char` call argument is the program
using the byte as a number, and such a byte is never recorded. If anything was
recorded, the pass starts over: the local types are rebuilt,
`decompiler/crates/kuna-decomp/src/p5_types/kuna_charbyte.rs (seed_char)` seeds
each recorded byte `char` - the fold it would have had without the
zero-extension's vote, unless the byte is type-locked or its storage is covered
by a type-locked symbol, whose type stands - and propagation runs again from the
new seeds by the unchanged rules. No edge is overridden, and that is what keeps the output sound.
An edge override (take `char` on the pointer edge although the lattice ranks it
lower) leaves every Varnode the byte had already typed `uint1` behind: a
constant compared with the byte keeps `uint1`, and `char v1; ... v1 == 0xe9` is
never true in C. Re-seeding lets `char` travel with the byte from the start, so
the constant prints `'\xe9'`, a `uint1` reaching the byte from anywhere else
still wins it, and the pointer keeps `char *` because the byte now pushes
`char *`. A fresh propagation can also tip a contest the rule has no stake in -
on bash -O2 `param_expand` a string pointer joined with an `int *` parameter
came out `int *` once the byte stopped voting `unsigned char *` - so
`decompiler/crates/kuna-decomp/src/p5_types/kuna_charbyte.rs (keep_or_restore)`
compares the two propagations Varnode by Varnode and keeps the second only if
every type is unchanged or is the same type with `uint1` read as `char`, through
any depth of pointers and arrays; otherwise the first propagation's types are
put back and the function prints exactly as upstream. The second propagation
runs only in a function where something was recorded; with the option off
nothing is recorded and the pass runs once, exactly as upstream.

Two kinds of byte are never retyped, whether recorded or only reached through
a pointer the recorded bytes retyped. The first is a byte that reaches a
`CALL`, `CALLIND` or `CALLOTHER` argument, a `RETURN`, or the `BRANCHIND` of a
switch, directly or through copies, joins and one-byte arithmetic
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_charbyte.rs (read_without_cast)`).
kuna shrinks a call argument to the byte when the `movzx` that widened it folds
away, and C passes a `char` argument through the default promotions with no cast
to show it, so `logit("%d",c)` would pass -128 for the byte 0x80 where the
binary passes 128; likewise the switch header prints the byte itself, and a
`char` switch variable never equals `case 0x80:`. The printed line is the same in
both arms; only the declaration moves, which is why the check is on the reader
and not on the text.
The second is a counter
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_charbyte.rs (is_counter)`): a
loaded byte whose own one-byte sum or difference is stored back through the
address it was loaded from (`*p = c + 1`), directly or through copies and joins.
The only `char *` such a byte meets is the default signed vote of that sum, and
taking it would turn a genuine `unsigned char *` into `char *` with `c + '\x01'`.
A digit stored into a different buffer (`*buf = d + '0'`) is character
arithmetic and is not a counter. Neither kind is recorded, and one found in the
second propagation - a sibling `a0[1]` read through the retyped pointer, a copy
or a join of a recorded byte - puts the first propagation's types back. So does
a byte that took `char` while the pointer it is loaded through did not end at
`char *`: such a byte prints as a cast of its own load (ssh -O2 printed
`(uint4)(uint1)(char)v2[1]`), which is the opposite of what the rule is for.
For the same reason the first propagation is kept when a pointer that took
`char *` copies into or out of one that stayed `unsigned char *` (through a
`COPY`, `MULTIEQUAL`, `INDIRECT`, `PTRADD`, `PTRSUB` or pointer arithmetic):
the two would print as separate variables joined by a cast, as a string
walker's loop pointer did (`v5 = (unsigned char *)&a0[1]`).
With `structsynth` on, a synthesized struct whose pointer field moves from
`unsigned char *` to `char *` can become identical to a struct already
synthesized and take its name; the `struct_N` numbers of every function
decompiled after it then shift by one, a renaming with no other effect.

The widened value is never claimed: the `INT_ZEXT` output keeps its own type,
and the cast tail prints the zero-extension of a `char` as the `(unsigned
char)` cast it is - `*v2 = (unsigned int)(unsigned char)*a1`, and a ctype
table index `__ctype_b_loc()[(unsigned char)c]`, the idiom the source wrote. A
byte read through an `unsigned char *` never meets a `char *` and stays
`uint1`. Over fmt, ls, sort and grep at -O0 and -O2 the option changes 21 of
3,248 functions: 39 byte and pointer declarations and 15 signatures move from
`unsigned char` to `char`, character constants compared with or stored into
those bytes print as literals (`v3 != '\t'`, `*v4 == '-'`), and no parameter is
added or removed. A constant added to such a byte takes `char` with it and prints
the way kuna already prints arithmetic on any `char` - `c + '\xd0'` for `c -
0x30` - which the 8 binaries never show and bash -O2 shows on 6 lines (its off
arm already has 14). Because the speculative merge of §6 joins only variables of
one type, three functions change a declaration count: two split a `char` byte
or pointer from an unrelated `unsigned char` value that had shared its
declaration, and one joins two `char *` locals. Every changed comparison was
checked by compiling both forms and evaluating them over all 256 byte values.
`tests/stages/kuna-charbyte.xml` pins the witness, the `unsigned char *` and
unsigned-compare controls, the `0xe9` comparison, and a pointer that is
`char *` only because `strlen` takes one.

**The Windows segment base (`pebnames`).** A Windows user-mode thread keeps its
Thread Environment Block at the base of `GS` on x86-64 and of `FS` on x86, and
x86 SLEIGH lowers a segment-prefixed operand to `GS_OFFSET + disp` /
`FS_OFFSET + disp` over a register input nothing in the function writes. No seed
types that register, so inference leaves it an integer and every read through it
is a magic offset — the anti-debug probe of the PEB's `BeingDebugged` byte prints
`*(char *)(*(long long *)(v1 + 0x60) + 2)`, and on x86 the base degrades into an
integer array whose elements are the TEB's fields. The fix is a type lock, placed
where every other lock comes from: `decompiler/crates/kuna-decomp/src/p5_types/kuna_pebnames.rs
(ActionPebNames)` runs once per function at the head of the universal schedule,
before heritage, and when the raw p-code reads the segment-base register and never
stores through it, it maps a type-locked, name-locked local Symbol `TEB *teb` over
the register's input storage (use point one before the entry, so only the input
version is covered).
Heritage then creates the input Varnode carrying both locks, and ordinary
pointer-arithmetic recovery and field rendering do the rest:
`teb->ProcessEnvironmentBlock->BeingDebugged`, `teb->Self->ProcessEnvironmentBlock->NtGlobalFlag`,
`teb->ExceptionList`. The structures come from
`decompiler/crates/kuna-decomp/src/p5_types/kuna_pebnames.rs (teb_pointer_type)`:
a `PEB` and a `TEB` (with the `NT_TIB` header and the `CLIENT_ID` pair flattened
in). A field is named only when its offset, name and size agree in every
PDB-derived layout the Vergilius Project publishes for the architecture — x64
`_TEB`/`_PEB` from XP SP2 through Windows 11 25H2, and x86 `_TEB`/`_PEB` from XP
SP3 through Windows 10 22H2 plus every x64 kernel's WOW64 `_TEB32`/`_PEB32` — so
a name that changed between releases (`CrossProcessFlags` was
`EnvironmentUpdateCount` on XP, `ApiSetMap` was `FreeList`/`SparePebPtr0` before
Windows 7, the x86 `BitField` was `SpareBool` on XP) is left a hole, and a read in
a hole prints the offset-named `field_0x<off>`. The structures have grown with
nearly every release, so each is built as a variable-length type at the largest
published size (TEB 0x1878 and PEB 0x7d0 on x64, 0x1038 and 0x488 on x86).
Variable length is what keeps a phantom neighbour's name out:
`decompiler/crates/kuna-decomp/src/substrate/addtreestate.rs (AddTreeState::calc_subtype)`
takes a variable-length base as size 0, so a constant offset past the end of the
modelled fields (`gs:[0x18d8]`, which a fixed-size TEB would wrap into the next
TEB's `ProcessEnvironmentBlock`) — or a negative one — stays an integer addition
on the cast base rather than an array index. A pointer
whose target is not modelled (`StackBase`, `ProcessHeap`, `Ldr`, …) is typed
`undefined<ptrsize>` rather than `void *`, so the field names a value without
pushing a pointer type into the variables it merges with; only `Self` and
`ProcessEnvironmentBlock` are typed pointers, because they are what the chain
follows. Completing a structure mints a new `Rc`, so a field cannot point at the
structure that holds it: `TEB.Self` points at an identical `_TEB` (the Windows
structure tag) whose own `Self` is opaque, which resolves one hop through `Self`
and leaves a second hop an untyped value.

**A write through the base declines.** A store through a typed base gives the
stored value the field's type, and on x86 nearly every such store is an exception
frame linking its registration record into `ExceptionList`. That record's stack
layout is recovered by the local-variable restructure from how the untyped base is
used: the slot receiving `fs:[0]` takes the base's inferred element type, and the
open range behind the stored record address pulls the handler and state slots in
with it, which is how MSVC's `{Next, handler, state}` record becomes one `int4 [3]`.
With the field opaque that record split into three unrelated scalars; modelling
`_EXCEPTION_REGISTRATION_RECORD` instead put the record over the wrong slots and
spread a record-pointer type onto whatever kuna's stack analysis believed was
stored (a /GS cookie, a handler constant, a call result). So a function that
stores through the base is left as upstream renders it. Raw p-code is not yet SSA,
so `segment_use` in the same file finds such a store by storage within each
instruction that reads the register: a segment override always lowers to
`tmp = SEG_OFFSET + addr` feeding that instruction's `LOAD` or `STORE`, and any
non-`LOAD` op consuming a derived address derives its output too.

The gate is the resolved `peb_names` flag on the ArchSeam. Every mode requires a
Windows compiler spec (`windows`/`clangwindows`), and the register is chosen by the
code space's address size (`GS_OFFSET` on x86-64, `FS_OFFSET` on x86). The register
choice is why the x86-64 ELF canary at `fs:0x28` and the x86 one at `gs:0x14` never
reach the pass, and why an `fs:` read in 64-bit code or a `gs:` read in 32-bit code
stays untyped; the compiler-spec requirement is what keeps a non-Windows image that
does read the GS base (x86-64) or FS base (x86) untyped even under `on`. The segment base is a
TEB only in user mode — a driver keeps its KPCR there and firmware nothing — so
the shipped default `auto`
additionally requires the loader's image fact `image_windows_user`, written at
`load file` from the PE optional header (`Subsystem` 2 or 3) by
`decompiler/crates/kuna-analysis/src/loader/format/pe.rs (is_windows_user_mode_image)`;
the XML bootstrap never writes it, so `auto` is inert on the datatest corpus. `on`
trusts the compiler spec alone, for shellcode or a stage bytechunk. A Symbol
already mapped over the register (a user `type varnode`) wins, a `TEB`/`_TEB`/`PEB`
type that is not this exact variable-length layout already in the program makes
the pass decline, and when the gate is off the pass takes back the non-isolated
Symbol it mapped on an earlier decompile of the same function. Because that Symbol
is type-locked it also survives a second decompile under the gate, and the pass
recognises it there instead of mapping another over the same storage, which would
leave an anonymous `TEB *v2` carrying every field read.

Three limits are known. A variable that holds `teb->Self` or
`teb->ProcessEnvironmentBlock` on one path and an unrelated value on another takes
the pointer type, so the unrelated value gains a cast (`(_TEB *)` on a call result in
one obfuscated binary), and a parameter compared or merged with such a value can
take it too. The store check sees only the instructions that read the segment
register, so a store through a pointer loaded from the TEB
(`mov eax,fs:[0x18]; mov [eax],esp`) is typed as `teb->Self->ExceptionList = ...`.
And `auto` needs the loader fact, which only the object loader writes, so it never
fires in the Ghidra front-end; `on` does. Exercised by
`tests/stages/kuna-pebnames.xml`, `tests/stages/kuna-pebnames-x86.xml` (both
with near misses that must not gain a name, and x86 SEH and MSVC C++ EH frames that
must render exactly as with the option off) and
`decompiler/crates/kuna-console/tests/verify_pebnames.rs`.

**The casting boundary.** Inference annotates; it never converts. Where the
final Varnode type disagrees with what an op requires, nothing in phase 5
reconciles it — the disagreement survives to
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionSetCasts)` in the one-shot tail, which renders an explicit cast
(chapter 09). So the symptom of a lost propagation is a spurious `(int *)`
cast in the output, never wrong data-flow.

**Constant pointers.** The other typerecovery action in `mainloop`,
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionConstantPtr)`, turns bare constants into symbol references. Decision
rule for the simple case: a constant with a single reader, already typed as a
pointer (or used where only a pointer makes sense), whose value resolves to a
mapped global symbol, is rewritten to reference that symbol via the space's
spacebase (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(spacebase_constant)`) — and the symbol's type then seeds the next inference
pass. The machinery around it is all about *not* firing: the action is gated
on type recovery having started and capped at 4 passes per function; the space
is chosen by `select_infer_space` (an explicit pointer type's space attribute
wins; otherwise the architecture's inferable spaces, tie-broken by scanning
forward for a LOAD/STORE that names the space); usage arms (call arguments,
comparisons, PIECE, COPY, plain INT_ADD) are additionally gated by the P0
`inferconstptr` option; the value must lie inside the space's pointer bounds;
and a constant whose bit pattern has fewer than 3 bit-transitions is rejected
as a probable flag/mask (`p9_emit/coreaction_render.rs (is_pointer)` — the ActionConstantPtr file), because turning `0x100000`
into a pointer corrupts every function that uses it as a bit. On acceptance
the symbol lookup requires an exact hit unless the target is a character
array (string constants may point mid-string).

An explicit global data declaration is authoritative at an exact address.
Both `--assert 'data …'` surfaces — the in-process assertion plane and its
`map address` console lowering — route through
`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
(upsert_data_mapped)`. If analysis already planted non-function data there,
the declaration retypes, resizes, and renames that mapping in place before
setting its type/name locks; it does not add a second overlapping symbol.
This ordering matters to phase 5 because ordinary container lookup selects the
smallest covering symbol. For example, operand-reference analysis can read the
low byte and high-byte NUL of a short UTF-16 literal as `char[2]`; a later
`wchar_t[3]` assertion must replace that object so constant-pointer recovery
and COPY propagation see the declared two-byte character type. Neighboring
data mappings and function symbols are never replaced, and no global string
length or encoding heuristic is changed.

Two (kuna) escapes hook exactly here, both shipped default-on (DIV-2,
`decompiler/crates/kuna-decomp/phases.toml`):

- **(kuna GH-6930)** [`inferfuncentry`](../options.md): the
  bit-transitions rejection is skipped when the constant resolves *exactly* to
  a known function entry
  (`decompiler/crates/kuna-decomp/src/p5_types/kuna_inferfuncentry.rs
  (kuna_is_function_entry)`, wired in `coreaction_render.rs
  (kuna_const_is_function_entry)`) — a function placed at a power-of-two image
  base is a single-bit value, but it is still a function. Ordinary data
  constants never match, so flag semantics are preserved.

  The rejection this skips is the only guard that keeps integer flags, masks
  and round sizes out of the symbol table, so the skip is allowed only where
  the constant is being used as an address. Round buffer sizes collide with
  function entries often enough in a position-independent executable, where
  `.init` sits at a low round offset, that this is not hypothetical: coreutils
  `tail` at `-O0` computes `MIN (n_remaining, BUFSIZ)`, `BUFSIZ` is `0x2000`
  and `0x2000` is that image's `_DT_INIT`, so the bound printed as
  `if (_DT_INIT < v9)`, the other arm of the select as `v1 = _DT_INIT`, and the
  `uintmax_t` byte counter they bounded became `void *` — in `dump_remainder`,
  and through [`protoorder`](../options.md) into its caller's stack variable.

  `decompiler/crates/kuna-decomp/src/p5_types/kuna_inferfuncentry.rs
  (entry_escape_applies)` decides this, on two readings of the same value. The
  reader itself must not be arithmetic: an ordering comparison, a multiply, a
  divide, a remainder or a shift reads its constant as a number
  (`kuna_inferfuncentry.rs (reads_as_integer)`), and C has none of them between
  a function pointer and anything else. A comparison also pins the value one
  below its literal (`kuna_inferfuncentry.rs (orders_its_constant)`), because the
  strict/non-strict rewrite moves the bound by one on the way here: the same
  `MIN (n, BUFSIZ)` arrives as `INT_LESS(0x2000, n)` in `tail` and as
  `INT_LESS(0x2001, n)` in `head`. The reader alone does not settle it,
  because the assignment arm of a `MIN` is a plain COPY and so is every entry of
  a function-pointer table — coreutils `od` at `-O2` selects between `sub_3f20`,
  `sub_3ff0`, `sub_40a0` and `sub_4150` that way, and `0x3ff0` is the one of the
  four with fewer than three bit transitions. What separates them is the rest of
  the function: the escape is also declined for a value the function reads as a
  number *anywhere*, which `ActionConstantPtr::apply` collects once per pass from
  the constant snapshot it already takes
  (`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs`).
- **(kuna GH-8471)** [`thumbfuncptr`](../options.md): a Thumb function pointer
  is `fn|1`; constant-pointer recovery produces `PTRSUB(fn) + 1`, and the
  simplification rule that normally deletes out-of-bounds PTRSUBs would
  collapse it back to a hex literal. The guard
  `decompiler/crates/kuna-decomp/src/p5_types/kuna_thumbfuncptr.rs
  (kuna_preserve_thumb_funcptr)` — consulted by
  `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
  (RulePtrsubUndo)` — keeps the PTRSUB when the base resolves through the
  spacebase to a `TYPE_CODE` symbol and the leftover offset fits inside the
  architecture's `funcptr_align` mode bits. Inert on architectures with no
  alignment encoding (`funcptr_align == 0`).

**(kuna) Struct synthesis from access offsets (`structsynth`).** A stripped
binary keeps no record of the aggregate a pointer points at, so the lattice above
gives a dereferenced parameter a pointee it can prove and stops there:
`unsigned long *`, and every field read rendered as `*(unsigned int *)&a0[1]`.
[`structsynth`](../options.md) (`off|param|locals|nest|all`, default `locals`) invents the missing
layout from the accesses themselves.

`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth.rs
(ActionStructSynth)` runs inside `mainloop` immediately after `ActionInferTypes`
in the `typerecovery` group, so the pointer-arithmetic pools below rewrite the
accesses on the same iteration. It fires **once** per function, and only once
propagation has reached its fixpoint: `ActionInferTypes::apply` returns 0
unconditionally — a type change is deliberately not a data-flow change — so
nothing else in the schedule can observe that the lattice has stopped moving,
and an action placed after it would otherwise read the first of up to seven
passes. `run_infer_types` returning "no change" therefore records a plateau flag
on the `Funcdata` (`funcdata.rs (kuna_infertypes_settled)`), and a function whose
lattice never settled (the 7-pass ceiling) declines.

The evidence is one read-only walk of the live `LOAD`/`STORE` ops. Each address
operand is peeled through `COPY`, `CAST` and constant `INT_ADD`/`PTRADD`/`PTRSUB`
to a `(base, byte offset)` pair; the access width and the loaded or stored
value's type are recorded per offset. Phis are **not** peeled. Chasing a
loop-carried `MULTIEQUAL` to its root reports offsets `{0, 1}` for
`while (*p) p = &p[1]`, which would synthesize a two-field structure over a
string, so a base that reaches a phi at all declines — an induction variable is
an array walk, a different hypothesis.

Pointer-ness is the one fact the pass never invents: the base must already carry
`TYPE_PTR`, which is why a parameter the lattice still believes is an integer —
`fmt`'s `get_line`, whose first parameter is the real `FILE *` — is left for a
named-type pass. A pointee that is a *named* composite (DWARF, a parsed
declaration, a libc shell, an earlier synthesized structure) wins outright, as
does a type-locked base. The remaining declines are: a non-constant offset term
(that is an array, TRex §3.3.3), the base used as an integer, fewer than two
distinct offsets, no access at offset 0, an offset that is negative or at least
`0x8000`, a uniform-stride run of one width (a `memset`/`memcpy` walk rather
than a layout, Howard §4.5), and a stack- or spacebase-rooted base — the frame
is `varmap.rs`'s to lay out and is never wrapped. The array signal does not need
a dereference: `p + n` for a non-constant `n`, even where only a callee
dereferences the result, is an index, which is what `ls`'s `mpsort` computes when
it passes `&base[n]` to its recursive callee while reading `base[0]` and
`base[1]` itself. The uniform-run rule tolerates a gap below pointer width — a
function that writes ten of a buffer's twelve bytes is ordinary, and demanding
exact contiguity made any such buffer a structure — but not at pointer width,
where a record with an untouched member between two observed ones is the common
case (`gnulib`'s `struct hash_table` is ten pointer-sized members whose readers
touch different subsets of them). The rule takes `int fd[2]` only in its plain form; an optimized
one that also saves and restores the pair as a single 8-byte word has mixed
widths, and it is the layout prune below that declines it, by leaving a single
field where two elements sat inside the wide access. A buffer whose *last*
element is written wider than the rest escapes both rules and is the pass's known
false-positive shape: `ls`'s `strmode` fills a `char[12]` with ten one-byte
stores and one two-byte store, so the widths are not uniform and the wide store
is past the narrow ones rather than over them.

**A lone field (`calleevote fields`).** "Fewer than two distinct offsets" and
"no access at offset 0" have one exception, and it is decided outside the
function. A pointer parameter read at exactly one constant offset other than
zero, with an access of four bytes or more, is a one-field record when the
function's callers are all known direct calls: the whole-binary driver marks
such a function on its `Funcdata` (`kuna_calleevote_closed`; chapter
[04](04-calls-and-prototypes.md), `calleevote`), and
`kuna_structsynth.rs (is_lone_field)` admits it. The condition is what keeps
the callbacks out: a `qsort` comparator or a hash-table hasher reads one field
of what it was handed exactly the way gnulib's `hash_get_n_buckets` does, but
its address is stored or taken, its contract declares `void *`, and it keeps
`void *`. Chapter 04 lists what counts as stored and where no function is
closed at all: every architecture but x86-64 (a function address there is
built from two instructions, which the one-instruction reference walk does not
join), a relocatable object, and an image without section headers. So the lone
field is read only in an x86-64 image, and there only for a function whose
address is not stored as a pointer-width word; a callback reached through an
offset table is not seen. A byte or halfword at a fixed offset is as often a character of a
buffer as a field, so those still need a second field. So does a base from
which an address at a constant offset reaches a phi (`address_walks`): that is
a pointer stepped through a loop over an array, `argv[1]` read and then
`argv + 2` walked, where a lone field would read one element as a record and
step through the array at the record's size. An address at a constant offset
handed to a call (`getfilecon (name, &f->scontext)`) is not a walk; a record's
member is passed that way all the time, and refusing it was measured to cost
21 functions whose record decbench credits. An array read at one constant
index and nothing else (`return a[2];`) is indistinguishable from a getter and
does become a one-field record; its C is still exact. The same closed function also gets
one more propagation pass when it has such a candidate and its main loop would
end on the pass that first typed it (`wants_settle_pass`): the plateau flag is
set only by a pass that changes nothing, and a short getter's loop ends before a
second pass runs, so without it the synthesizer is never offered the function at
all. A one-field record answers only for its own shape in the ledger below, and
it gives way to the record every caller passes (`points_at_lone_record`).

At a conflicting offset the **widest** access wins. A field wider than an access
renders as a cast of the field (`(uint4)w->b`); a field narrower than an access
loses the field name altogether (`*(uint1 **)w`).

The layout is then **pruned to one a C compiler reproduces byte for byte**. The
declaration is exported as source by `kuna decompile-project`, so it is only
worth anything if the compiler reading it back puts `field_0xK` at offset K, and
three shapes of access break that without changing a character of the body,
which goes on naming its fields as though the header agreed: an access whose
width the exported prelude cannot spell at that width (`undefined3` is a 4-byte
`unsigned int` there, `undefined5`/`6`/`7` are 8-byte); an access at an offset
its own alignment does not divide, which the compiler pads forward, moving every
later field; and an access inside the bytes a wider access already claimed,
where a second field moves every later field by its own width. Each is dropped
to a hole. What survives is naturally aligned and 1, 2, 4 or 8 bytes wide, and
two such ranges either nest or are disjoint, so the surviving layout cannot
overlap at all — one sweep in offset order decides it.

What survives is then made **dense**: every byte from 0 to the end of the last
surviving field belongs to a member, a gap becoming an
`undefined1 field_0x<hex>[N]` member of alignment 1 at the offset it is named
for. That is not tidiness. `printc.cc:1015-1033` resolves an address inside a
structure to the field that contains it and, when no field does, invents the
member name `field_0x<hex>` anyway, while `compose_type_body` renders the same
gap in the exported header as `undefined1 _pad<hex>[N]` — so a body would name a
member the header does not declare. The gaps are reachable without a
dereference, because address arithmetic gets there: `p + 8` handed to a callee
prints `&p->field_0x8`. Filler is always an array, one byte included, so that an
array-typed member is exactly the padding and never a field the pass claims.

The structure's size is the end of its last surviving field rounded up to the
width of the widest surviving field, and the bytes that rounding adds are
covered by an `undefined1` filler member like any other gap — so a layout whose
last field is a `uint4` at 0xc8 next to an `int8` somewhere below it is 0xd0
bytes with `undefined1 field_0xcc[4]` on the end, not 0xcc bytes. That is what a
C compiler does with the same members, and the body depends on it: `a0[1]` on a
structure the decompiler sized 0xcc and the compiler sizes 0xd0 is a different
address, which no amount of correct `offsetof` catches. The bound on the size is
still SecondWrite §5.1's `UpdateStructure` rule — the evidence that became
fields, not the evidence that was pruned; taking the size from the accesses
instead let one misaligned far access declare a 64 KB type for a 16-byte object.
There is no `variable_length` flag: that flag makes `propagate_from_pointer`
refuse the pointee outright and gives `AddTreeState` a zero size, which
invalidates every `TypePointerRel`. Past the end there is no member and no
invented name: the printer spells the address as an element of the structure
array plus a byte offset, so a pruned read at 0x10001 of a 16-byte layout
renders as
`*(uint4 *)((int8)&a0[0x1000].field_0x0 + 1)` — the right address, readable only
by accident. An unaligned read that straddles the end can come out worse: the
crazyflie `cf2` O2 reads at `0x8022c48` and `0x8022d08` are split into four byte
reads of `a0[1]` and printed in piece syntax (`v3._0_1_ = a0[1].field_0x0`), which
is the right value but not C.
A field takes the type of the value the access carried when the widths agree and
the type's C spelling is its own width — a scalar or a pointer — and
`undefined<N>` otherwise. It commits to a signedness only when every access of
its width does: one read carrying a signed integer and another an unsigned or
undefined one leave the field `undefined<N>` (`Evidence::record`). The reason is
an extension the text cannot show. `find`'s `consider_visiting` reads `fts_info`
into signed comparisons and, with `movzwl`, into a call argument whose extension
the call absorbed; typed `short`, the field made `sub_7510(a1->field_0x68)`
sign-extend what the binary zero-extends, the value `RuleExpandLoad` keeps the
unsigned spelling to preserve (chapter 03). An unsigned field is safe in both
directions, since a sign-dependent operation is its own p-code op and prints its
own cast.
A float beside anything else is a different disagreement: the bytes are a union
member read two ways, and C has no scalar that reads them as both. Typed `long`
(or `undefined8`), a field read by a `movsd` prints `(double)a0->field_0x8`, a
value conversion of bits the binary reinterprets; typed `double`, the same field
read by `cvtsi2sdq` prints `(double)(long)a0->field_0x8`, the conversion the other
way. So a field whose accesses of its width carry a float and a non-float is raw
bytes, `undefined1 field_0x<hex>[N]` like the filler of a gap, and every read of
it casts the address: `*(double *)a0->field_0x8`, `(double)*(long *)a0->field_0x8`.
The value classes of every access of the field's width are accumulated rather
than compared pair by pair, so a sign contest that already cleared the type cannot
hide a later float (`Slot::reinterpreted`). A narrower access is not the field's
evidence and needs no rule: it already prints through an address cast
(`*(float *)&a0->field_0x8`).

Names are program-wide `struct_N`, minted and handed out by the **layout
ledger** (`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/ledger.rs`),
which is the set of `struct_N` the program's `TypeFactory` already holds: the
ledger probes `find_by_name` from `struct_0` up to the first free name, reads
each one's layout back, and declines any whose members are not all named
`field_0x<hex>` — that name is held by a type from somewhere else (DWARF, a
parsed header) and is never handed out as the answer to an access pattern.
Every mint declines on an error rather than propagating one: `find_add` rejects
a second, different definition of a held name.

Two functions rarely read all of one record. `ls`'s two `fileinfo`
comparators (`rev_strcmp_size` and `rev_strcmp_df_size`) are one record read
twice: the first claims `{0: char *, 0x48: long}` and the second those two plus
`{0xa8: int, 0xac: unsigned int}`. Comparing layouts for *equality*, which is
all the first version did, gives the one record two names. The ledger therefore
matches by **subsumption**: a measured layout `L` is answered with a minted
structure `S` when `S` is at least as large and every field `L` claims exists in
`S` at the same offset, at the same width and with the same type. Filler is not
a claim and never takes part. Over `fmt`, `ls`, `sort`, `du`, `find` and `tar` at
O0 and O2 the same twelve binaries then name 471 records where equality named
515, and no function loses a structure it named before. Agreement on a shared field is
**exact**, `undefined<N>` included: two different types at one offset — a
`char *` against a `uint4` — are the obvious conflict, and a field one reader
could not type is its own answer rather than a wildcard the other may fill in.
Letting `undefined8` be absorbed by anything eight bytes wide was measured and
rejected, because it unifies records that merely share a shape: `du` O2's
`sub_c090` writes two double bit patterns and a zero byte at `{0, 8, 0x10}`,
which a wildcard rule reports as contained by an unrelated record whose offset 0
is a pointer, and the body then prints
`field_0x0 = (void *)0x3f80000000000000`. Among the structures that do contain
`L`, the **smallest** is the one reused, since every extra field a reused
structure carries is a claim this function's evidence does not support.

Containment alone is not the whole rule, because it stops being evidence in two
ways. The container can dwarf the measurement: a reader that measured the two
words of `ls`'s `hash_entry` is contained by the 200-byte layout of its `stat`
and would be declared to hold 174 bytes of members nothing read. And the
measurement can be too generic to name a record. Two integer words are how
distinct records begin: `struct stat` starts `{dev, ino}`, and so do gnulib's
`cycle_check_state` and `cp`'s `Src_to_dest`, while a list node, a `timespec`
and a hash entry are two integer words to a stripped reader as well. Unguarded,
`cycle_check(struct cycle_check_state *, const struct stat *)` came out with
both parameters declared as the state record. A structure that says *strictly
more* than `L` measured therefore answers only when it claims at most **twice**
as many fields as `L` and is at most **four times** its size, and when `L`
claims at least two fields of which one is a typed pointer, or at least three
without one. A typed pointer claim is stronger evidence because its pointee is
part of the field's identity, and it is what keeps the two `fileinfo`
comparators, 80 bytes measured against 176, on one name. A pointer whose pointee
says nothing (`code *`, `void *`, `undefinedN *`) is not a typed pointer. A
layout whose every claim past offset 0 is such a pointer is a table of slots,
such as an interface, an ops vector or a vtable, and it is answered only by a
structure of exactly its own shape. Distinct tables differ only in how many
slots they have. In `rsyslogd` O2, seven `*_if_s` interface records of 80 to 136
bytes are each a version word followed by function pointers. Without the table
rule they all took the 144-byte layout of `statsobj_if_s`, and of the 31 fields
that added, one was real. A layout with no typed pointer is answered only by a
structure of its own size that claims nothing past the reader's last field. Such
a structure may fill in the holes between the fields the reader measured, but
not the bytes after them, because records that begin alike part ways exactly
there. Every netlink reader in `ip` measures the same `nlmsghdr` words ahead of
a payload of its own. dpkg's 24-byte `pkg_queue` begins the way a 40-byte
command record does, and it ends in the tail padding where the 24-byte
`pkg_spec` keeps two flag bytes. Last, no structure answers with a member inside
the alignment padding the reader's claims leave between two of them (from the
end of one claim up to the next claim or the next multiple of that claim's
width, whichever comes first). Records also part ways in their first words.
Every `bash` command record begins with an `int flags`; `if_com` then has a
pointer at 8, while `for_com`, `select_com` and `case_com` put an `int line` at
4, in exactly the bytes `if_com` pads. Without this rule `execute_if_command`'s
`IF_COM *` took `for_com`'s layout, and six `it_init_*` readers of `ITEMLIST`,
which measure `{0: int, 0x10: pointer}`, took `case_com`'s. A hole wider than
its padding is still room for a member the reader skipped, and a reader with a
typed pointer may still be given members in its tail padding: `fmt`'s
`put_word` measures `{0: char *, 8: int}` of a `WORD` whose `int space` sits
at 0xc.

Each bound catches a shape the others miss. Bounding the size alone still answers `ls` O0 `sub_afe2`'s two fields with a
sixteen-field structure at 1.5× the size. A 2× size bound refuses the
`fileinfo` pair. The two-integer-word shape passes both growth bounds (2 claims
against 4, 16 bytes against 32). Two layouts of exactly the same shape always
share a name whatever the bounds say.

Containment is also checked against the bytes a reader touched without claiming
them. The prune drops an unaligned word, a 16-byte copy or a width the header
cannot spell, and such an access prints through whatever member holds its first
byte. The engine decides whether a load may be printed after a store by
comparing the two addresses (`ActionMarkImplied`'s `isPossibleAlias`), and it
treats one base plus two different constants as two different objects without
asking how wide either access is. That test is only as safe as the spelling it
is given. Take a reader that loads four bytes at offset 7 and then stores one
byte at 8, beside a writer that stores a `char` at each of 7 through 10. The
reader's own layout spells the load `*(uint4 *)&a0->field_0x4[3]`, which the test
cannot tell apart from `a0->field_0x8`, so the load keeps its own statement ahead
of the store. Answered with the writer's structure, the same load becomes
`*(uint4 *)&a0->field_0x7`, is judged distinct from the store, and prints after
the byte it reads has been overwritten. A structure therefore answers for a
reader only if it lays exactly the reader's own members (offsets, widths, types
and filler) over every byte range the reader accessed without claiming
(`ledger.rs (keeps_unclaimed)`), so each such access is spelled as it is under
the reader's own layout. The reader it turns away mints its own shape even
though a live structure contains it, so that name is superseded from the start,
and the convergence sweep below decides the reader again to the same answer.
`tests/stages/structsynth-unclaimed-bytes.xml` pins the shape. Over the 101
builds of the first five sets measured below, the veto changes one parameter: `rsyslogd` O2
`timeConvertToUTC`'s `struct syslogTime *`, whose 4-byte store at offset 7 a
same-size layout with bytes at 7 to 10 answered, keeps its own shape and loses 4
true fields. The address test
itself is unchanged, and with the option off it misorders the same reader on raw
offsets: `*(uint4 *)((int8)a0 + 7)` also prints after `*(char *)&a0[2] = ...`.

The rules were drawn from builds scored against DWARF
(`docs/features/structsynth/dedup_heldout.py`, which scores claimed fields with
the measure of `layoutscore.py` and lists every set). The growth bounds come from
`fmt`, `ls`, `sort` and `du`. The pointer rule comes from 17 independently picked
builds and was checked on 22 more. The table and integer rules come from 28
builds a reviewer picked, mostly outside coreutils, and were checked on 26 chosen
before any result under them was seen. The padding rule comes from 42 builds a
second reviewer picked (with it, `bash` O0's command records and dpkg's
`pkg_queue` keep their own names) and was checked on 34 more chosen before any
result under it was seen. Against equality dedup on main `75f832e3`,
claimed-field precision pooled over the 169 held-out builds is 0.8908 against
0.8902, with 347 more true fields, and recall rises on every set. The rule is a
trade, not a free win. An absorbed parameter is declared with the container's
whole field list: 53 builds gain precision and 6 lose it (`kmod` at O0, O2 and
O2-noinline and `grep` O2 lose the most, under a point each), and on the last 34
builds precision is 0.8852 against 0.8855. The record can also be wrong. Of the
258 parameters answered by a strictly larger structure across all 177 builds, 4
are given a structure measured from a different DWARF record and 5 cannot be
checked. All four are `e2fsck` readers with no typed pointer filled in between
their own fields by a same-size layout, and two of them take `ext2_inode_large`
for an `ext2_inode`, which is the same record for its first 128 bytes. Without
the padding rule the same builds give 11 more (10 of 28 absorptions on the
second reviewer's 18 builds).

When the containment runs the other way — the new layout strictly contains one
already minted — the minted one cannot be widened. `find_add` refuses to redefine
a held name, and a completed structure is already shared by every function that
took a pointer to it, so altering it in place would retype functions that never
measured the extra fields. The larger layout is minted under a fresh name and the
smaller one is **superseded**: still defined, and handed out again only to a
layout that no live structure answers for. Supersession is not recorded
anywhere. It is a property of the minted set — `S` is superseded exactly when
some other minted structure strictly subsumes it — so it is read from the
factory on each lookup, for the entries that answer, and cannot drift out of
step with the types that are actually defined. A lookup reads the layout of
only those held structures whose size lets them answer, or supersede an answer:
from the measured layout's own size up to sixteen times it. It reads
supersession only for the structures that answer.

That leaves an ordering artifact, because `decompile-all` decompiles in address
order: `rev_strcmp_size` at the lower address mints the smaller layout,
`rev_strcmp_df_size` supersedes it, and the first function's own text still
names the superseded structure it was given before the larger one existed. So a
whole-program batch closes the gap itself.
`kuna-console/src/project.rs (converge_synthesized_structs)` runs once after the
batch, asks the ledger which structures were superseded
(`ledger::superseded_names`), and decompiles again exactly the results whose C,
prototype line, exported variable rows or `types` array spell one of those names
as a whole identifier. The lookup now answers each with the survivor where the
survivor is in reach.

It is one sweep, not a fixpoint, and it does not always leave one name per
record, because the growth bounds are not transitive. Take `fb` claiming four
fields, `fa` two of them and `fc` eight that include `fb`'s four, in that address
order. `fb` mints, `fa` reuses it (four claims against two), and `fc` supersedes
it (eight against four). Decided again, `fb` moves to `fc`'s structure, but eight
claims are out of `fa`'s reach, so the lookup answers `fa` with the superseded
structure it was given the first time, the smallest one that still answers for
it. The record ends with two names. Apart from the unclaimed-bytes veto above,
the lookup never mints a name that an existing structure answers for: that name
would be superseded the moment it existed. So a mint only happens when nothing
held answers, and the sweep mints nothing as long as each function it decides
again measures the same layout it measured the first time. The fixture
`decompiler/crates/kuna-analysis/tests/fixtures/structsynthchain_x86_64.c` and
the CLI probe `tests/cli/structsynth-sweep-mints-no-third-name.json` pin this
shape. A batch that synthesized nothing pays one ledger probe for the whole run.

The convergence is a property of the eager batch (`decompile_targets`,
`decompile_export_targets`). The streaming project export has already written a
body by the time its name could be superseded, so it reports MORE records than
the eager export of the same program: `cp` O2 exports 31 definitions under
`--stream` against 30 eagerly (main: 33 and 33). Each is self-consistent, since
the header prune keeps every definition the document still names. A sharded
`--jobs N` run replays the ledger and ends with the eager batch's names (below).

The structure is **completed before** its pointer is taken, because completing a
structure mints a fresh `Rc` and the merge tests compare high types by pointer
identity. The install itself is `funcdata.rs (vn_update_type_locked)`, which
pairs `Varnode::update_type_locked` with `HighVariable::type_dirty()` — without
the notification `high_get_type` keeps handing back the stale type, and the
printed prototype never changes. The change is signalled by bumping the action's
`count`, the only signal `Action::perform` reads.

Because the ledger lives in the program's `TypeFactory`, `struct_N` is
program-wide under `kuna decompile-all` and `kuna decompile-project`, which load
once; `kuna decompile` spawns one engine per function, so there each function
numbers from `struct_0` again, and so would each worker process of a `--jobs N`
run if it were left to itself. It is not. The decision a lookup takes reads only
data -- the measured layout, its members, the bytes it accessed without claiming,
and the layouts, members and sizes of the structures held -- so
`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/shard.rs (Replay)`
can take it without a factory: it scans the `struct_<n>` slots as the ledger
does, skipping a name another type holds, calls the same `best_of` and
`keeps_unclaimed`, and mints at the first free slot. A `--jobs` worker records
each lookup a function makes, with a recipe for every field type (a named type
by name and id, a pointer or byte array around its rebuilt element), and the
answer its own ledger gave, with the lookup that minted that structure in the
worker. The parent replays the records in target order. A structure is its
members, so when a function's own answers and the serial ones name structures
built from the same members, lookup for lookup and one name for one name
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/shard.rs
(renaming)`), the function's text is the serial text with other numbers and
is renamed. A structure minted from its own function's lookup is the common
case: the serial run minted the same members under another number. Every other
function is decompiled again by a worker that has destroyed the structures it
minted itself and every type built on one, installed the replayed structures in
mint order
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/shard.rs
(install_table)`) and answers each lookup with its replayed name. A named field
type gets a recipe only if the worker's load created it
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/shard.rs (AtLoad)`): a pass
that interns a type the first time a function needs it, as `pebnames` does
`PEB` and `TEB`, leaves it in some workers and not in others, so a structure
with such a field is named by one ordered worker instead. The parent also
answers the sweep: a structure still answers every layout it answered once, so
the sweep's lookups never mint, and the ones whose answer changes are renamed
or decompiled in the same pool. The synthesized structures of a sharded run are
therefore the eager batch's, name for name -- the batch a pool can run, which on
`decompile-all` means `--option protoorder off`, since the callee-first order
(chapter [04](04-calls-and-prototypes.md)) decides what a function measures and
no worker can see another worker's callees. Chapter [00](00-overview.md) has the
pool's side, including the checks that send a run back to one ordered worker.

The `.h` of a project export lists the minted structures after every other
type, in ascending `N`
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/ledger.rs
(in_name_order)`). The factory's tree orders two structures of one size field by
field and then by the ADDRESS of a field's type, and minted structures always
agree that far on their first field, `field_0x0`; rendered in tree order, the
same serial export of `sort` O2 declared its structures in three different
orders over five runs. A minted structure holds nothing by value but scalars and
byte arrays, and nothing holds one by value. Under `nest` (below) it can also
hold a pointer to another minted structure or to itself, which the forward
declarations at the top of the header already name. So after everything else
is a valid place to define it, and the order is now the same every run and in
every worker.

The synthesized layout is printable rather than only inferable: the P9 option
[`structdefs`](../options.md) prints the definition of every composite a
function's C names reach, so `--option structdefs on --option structsynth param`
puts `struct struct_0 { ... };` — filler members and all — above the function
whose parameter this pass retyped, and carries the same text in the per-function
`types` array of `decompile-all --json`. `structdefs` is off by default, so the
layouts of a default run live in the `decompile-project` header and nowhere in a
function's own text.

**Nested records (`nest`).** Under `param` a field that holds a pointer keeps
the scalar its loads carried, so every read through it is raw offset arithmetic
again: `fmt`-style code that loads `o->in` and reads it three times prints
`v1 = a0->field_0x0; ... (long)*(int *)((long)v1 + 0xc)`. The value `nest` of
`structsynth` (`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/nest.rs
(nest_fields)`) also measures the records the pointer fields of an accepted
record point at. Every `LOAD` of one pointer-sized field is a value of that
field's type, so the dereferences made through all of them are one base's
evidence, merged exactly as `Evidence::record` merges the accesses of one base
(`Evidence::absorb`): the widest access wins, a sign contest stays contested,
and one loop-carried load, index or integer use of any of them declines the
whole field. That evidence is held to the conditions a parameter is. Each loaded
value must already carry `TYPE_PTR` and must not be type-locked. The lattice must
have settled. There must be two distinct offsets, one of them 0, and they must
not be an array's uniform run. What passes is pruned and made dense as a
parameter's layout is, looked up in the ledger like any other layout, and the
field is typed as a pointer to the completed structure the ledger answered. The
nested record is looked up before the record that holds it, so the pointer names
a completed type. This goes at most two records below the parameter. A field
whose loads fail any condition keeps the type it had.

A loaded record is the **same** record when everything it was seen to hold, the
enclosing record also holds: each of its claims at the same offset, width and
type, with the bytes it accessed without claiming laid out alike
(`ledger::keeps_unclaimed`). Then the field points at the enclosing record
itself. `grep`'s kwset `treenext` is the shape: unrolled by the compiler, it
reads `{0, 8, 0x10, 0x18}` of a tree node and the same four fields, with the
same types, of the nodes its `llink` and `rlink` point at, so both links become
`struct_N *` of the node's own `struct_N`. The test runs after the standalone
conditions, so a single word is never enough: `obstack`'s `_obstack_newchunk`
reads the chunk's `prev` at 8 where the obstack keeps an integer word, and its
chunk is not an obstack. Only the record directly holding the field is compared,
since a record two levels up would have to be minted together with the one in
between. A record whose types only nearly agree (an unrolling one level deep
reads the inner node's links as `unsigned long` because they are only passed to
calls) gets a record of its own with the same offsets: correct, but a second name.

A structure that points at itself cannot be completed first, because its own
field names it, and completing a structure mints a fresh `Rc`. The ledger
therefore mints it around its incomplete shell
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth/ledger.rs (mint)`):
`get_type_struct`, a pointer to the shell, then the members. That is the model
a DWARF `struct node { struct node *next; }` already lives with (chapter 01), and
it costs the same thing there: a value loaded through the self pointer has a type
with no members, and every read through it printed as
`*(long **)((long)v1 + 8)`. For a synthesized structure the shell is resolved.
Where a `LOAD` or `STORE` value type is a pointer to such a shell, and the
completed structure of that name is a synthesized one that itself points at that
exact shell,
`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth.rs (resolve_self_pointer)`
gives the pointer to the completed structure instead. It is called from the
propagation edge (`coreaction_infertypes.rs (propagate_load_store)`) and from the
load and store cast tokens of chapter 09. A DWARF forward declaration, or any
other incomplete type, is left alone. A node three links down then reads
`v3->field_0x18`, not `*(uint1 *)&v3[3]`. In the ledger a self pointer is keyed
`SELF` rather than by its pointee's name (`ledger.rs (SELF_KEY)`), so two
self-pointing layouts of one shape are one record whatever each was going to be
called. Every other nested pointer is keyed by its pointee, which makes the
nested record part of the field's identity: two readers that nested different
records at one offset are different records.

Three limits follow. The convergence sweep can mint under `nest`. A reader
decided again after its inner record was superseded measures an outer layout
whose pointer names the survivor, and no structure held answers for that. A
`--jobs` worker records a request whose field points at a synthesized structure
or at itself with no recipe, so such a structure is named by the one ordered
worker (chapter 00). And a loaded value the lattice types as an integer declines
however it is dereferenced. Over ten builds of the campaign corpus that is most
of the field loads that are dereferenced at constant offsets: taking pointer-ness
from the dereferences, and dropping the offset-0 condition for a nested record,
adds 31 nested fields, each a pointer to a structure in DWARF. It is not done
here, because pointer-ness is the one fact this pass does not invent.

**Returned records (`locals`).** Under `param` only a parameter is measured,
so a record a function obtains from a call keeps whatever pointee the lattice
gave it: an allocation it fills in prints as `v1 = (unsigned long *)xmalloc(0x18);
*(char *)((long)v1 + 0x14) = 0x6e;`, and a pointer a lookup returns as
`(long)(int)v1[2] + v1[1] + *v1`. The value `locals` of `structsynth` makes the
value a `CALL` or `CALLIND` returns a base too
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth.rs (is_call_return)`).
Single assignment makes that value the local's only definition, and it is held
to every condition a parameter is: already a pointer, not type-locked, no phi,
index or integer use, two offsets including 0, not an array run, settled
lattice. The peel already stops at a `CALL`'s output, so the evidence is the
same walk. Five conditions are new, and each is about the value rather than
its accesses. A callee declared to return a pointer to something (`char *`,
`struct stat *`) has said what the pointer is, so only a `void *` or unlocked
return is measured.

The value must not end up sharing a variable with anything else
(`kuna_structsynth.rs (copies_alone)`), because a type-locked member decides the
type of the whole variable it is merged into. A copy is merged with what it
copies, a phi input with the phi's other inputs, and a value in address-tied
storage with everything else stored there, so the value declines when it, or a
`COPY`/`CAST` of it, reaches a `MULTIEQUAL` or `INDIRECT` or lands in tied
storage. The return register needs one more look
(`kuna_structsynth.rs (storage_is_tied)`): it is tied as a whole-function local
once merging starts
(`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs (mark_output_storage_addr_tied)`),
long after this pass has run, whenever a phi writes it. So a value whose storage
overlaps what any live `RETURN` reads, where some varnode overlapping that
return storage is a phi's output, declines too. The match is by overlapping
bytes, not by exact size: an `int` function returns `eax`, the call's result is
the 8-byte `rax`, and `kmod`'s module loader kept one `rax` variable for a name
from `strdup`, its length from `strlen` and a `calloc` record, which the record
type would have declared `struct_N *`. `tar`'s `wordsplit_add_segm` is the same
shape with the status in `rax`.

A function also has one return type, and every value it returns is typed by it
(`kuna_structsynth.rs (returned_beside_others)`). A record returned on one path
while another `RETURN` returns a different value declines; a constant zero is
the only other value allowed, since it is the null any pointer can be. Without
this, `dash`'s `pipeline`, which returns either what `command()` returned or a
new record, cast `command()`'s result to the record
(`v2 = (struct_6 *)sub_12010();`), and a two-path function returning
`getname()` or a filled-in record declared the name a record pointer.

A base whose every access stores text is a string buffer, however unevenly the
compiler split the copy (`kuna_structsynth.rs (stored_text)`). Text is either
a constant whose bytes are characters (`bytes_text`) or bytes loaded from a
read-only string literal, read back from the image: `tar` builds
`"SCHILY.xattr."` in a fresh allocation as an 8-, a 4- and a 1-byte constant
store, and `cp` fills the tail `mempcpy` returns with an 8-byte and a 1-byte
load of `"CuXXXXXX"`. A literal the heap model re-defines at a call (an
`INDIRECT` on read-only memory) is still the image's bytes. A store of zero is
neither text nor evidence against it, since a record's initializer stores zeros
as often as a string's terminator.

Last, a record ends where this function's accesses end, so a base declines
when a copy of it forms an address at or past that end
(`kuna_structsynth.rs (points_past)`). `sortlines` reads a merge node up to the
byte at 0x54 and passes `&node->lock` at 0x58 to `pthread_mutex_lock`; `mountlist`
keeps `&me->me_next` as its list tail; a message allocation passes its flexible
body. Typed on the accesses, each of those addresses prints as `&v1[1]`, and the
mutex, the tail or the body takes the record's type. `all` is `locals` and
`nest` together.

Globals are not bases, and that was measured rather than assumed. Over the eight
layout builds (`fmt`, `ls`, `sort`, `du` at O0 and O2) thirteen global-RAM
varnodes are dereferenced at two or more constant offsets, and every one reaches
a phi or an `INDIRECT` -- a global pointer is re-read after every call -- so none
passes a parameter's conditions; and a global *record* is not one base in the IR
at all, because each access to it is an absolute address of its own. Recognizing
one is a question about a symbol's extent across functions, which this
per-function action does not see. A pointer loaded from an array element or
from a record's field is left to `nest` and to the array hypothesis.

The class is small, and most of it is not on `type_match`'s surface. The same
eight builds hold 750 ground-truth struct-pointer locals against 816
struct-pointer parameters, but at -O2 a local lives in a register, which
`variables[]` never exports. At -O0 the stack slot a returned pointer is stored
to is copy-propagated away, so `framelayout` exports it from the first
restructure pass, before type recovery starts; `slotptr` (chapter 06) types
such a slot from the value stored into it, which is how a returned record
reaches `variables[]` at all. Of the call-returned values
dereferenced at two or more constant offsets 37 (O0) and 17 (O2) meet a
parameter's conditions. Over 14 builds (`fmt`, `ls`, `sort`, `du`, `grep` and
`tar` at O0 and O2, `find` and `diff` at O2) the locals declared a record pointer
and defined by a call go from 40 to 116. Joined to DWARF at the call site (the
`mov %rax,-K(%rbp)` after the call at -O0, a location list starting at the return
address in `rax` at -O2, the callee's return type failing both), 103 are struct
pointers, none is anything else, and 13 do not join; the claimed fields of the
new records are 173 of 203 exact by offset and width (0.852, against 0.862 for
parameters; the misses are merged byte stores such as `mode_change`'s
`op`/`flag` pair and a union member measured at its first word). Parameter
layouts do not move (claimed-field precision 0.8622 and recall 0.0666 in both
arms, nesting unchanged) and the headers carry 539 structures instead of 500.

**The default is `locals`.** Flipping it moves no datatest assertion
(675/675) and no stage assertion of another feature (1306/1306; the only new
keys are `tests/stages/structsynth-locals.xml`'s, whose first pass pins
`param`), and `make test-cli` is 221/221. The 444-slice typesweep is 1,472
perfect in both arms (aggregate 3,773.19) with no function better or worse and
no function's scored variable count changed. Of its 65,377 scored decisions
708 differ only in `struct_N` numbering and 95 change type: each is a -O0 stack
slot a returned record is stored to, `undefined8`, `long *` or `int8 *` under
`param` and `struct_N *` under `locals`, and in every one DWARF names a struct
pointer (44 `hash_entry *`, 9 `predicate *`, 8 `group *`, 7 `dev_ino *`,
6 `passwd *`, ...). None replaces a correct pointer; the pinned metric scores
them the same in both arms because it compares pointee names. Over 29 binaries and 12,939 functions (x86-64
coreutils `fmt`, `ls`, `sort` and `du` at O0 and O2, `grep`, `tar` and `gzip` at
O0 and O2, `find` at O0 and O2, `diff`, `sdiff`, `cp`, `bzip2`, `useradd`,
`dash`, `crond`, `kmod` and `xmlwf` at O2, `diff3` at O0, `sort` at
O2-noinline, and the ARM32 `chibios` and `freertos` at O0) 872 functions change
against `param`: 769 only in `struct_N` numbering, because a returned record is
minted between parameter records; 83 are the same statements with each access
through the returned pointer respelled from its byte offset to a field, every
declaration that changes being a pointer that became a record defined by one
callee's result (checked by a classifier that compares declarations and casts,
not only bodies); and 20 were read. In 15 of those the record gets a register
variable of its own where `param` had merged it with an unrelated value of the
same primitive pointer type (gnulib's `transfer_entries` kept the bucket and
the new entry in one variable; the source has two), so the function declares
one more local (`ls`'s `print_dir` two); 4 are `find` predicate records whose
name field holds a string literal; one is a local typed `void *` because it is
stored to a `void *` field. No function's control flow, call sequence or
argument count changes, and of the 27 return types that move 24 are struct
pointers in DWARF, two return a record behind a `void *` handle and one builds
a `Cell` whose DWARF type sits on an inlined abstract origin.

Serial whole-binary `decompile-all` time (interleaved min-of-15 against `param`
over `fmt`, `ls` and `sort` at O2 and `bash` O2, on a machine shared with other
work) moves by -8.2% on `fmt` (its median by 0.0%), -1.3% on `ls`, +3.4% on
`sort` and +1.2% on `bash`, inside the +5% budget. `sort`'s output is identical
in both arms, and an A/A control, a second `param` arm interleaved in the same
run, reads 6.7% slower than the first with `locals` between the two, so its
delta is the machine's noise rather than work this value does.
A sharded run keeps pace because a function asks for its widest layout first
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_structsynth.rs (synthesize)`).
A returned record and a parameter of one function are often two layouts of one
object, and when a later lookup's layout contains an earlier one, the function's
own ledger supersedes its earlier answer, which a `--jobs` replay cannot
reproduce, so the pool sent every function with a synthesized structure back to
its one ordered worker: tar O2 `decompile-all --jobs 8` took 26.0 s against
15.2 s under `param`. Asked widest first, a later layout never contains an
earlier one, the pool renames in place (on tar O2, 109 functions renamed and 10
decompiled again, against 100 and 9 under `param`), the output is identical to
the serial run with `--option protoorder off`, and `--jobs 8` takes 15.4 s
against 15.0 s under `param` (interleaved min-of-11). Parameters alone keep the
order `param` always had.

**`param` was the first default.** Flipping it on moves **no** datatest assertion
(675/675). In `tests/stages` it reaches four assertions of other features, each
the intended rendering of a pointer input: `PEBNAMES-X86 #6`/`#8`, where the
FS-segment base of an SEH-linking function that `pebnames` leaves untyped becomes
a two-member `struct_0 *` (`v->field_0x0` for the registration link, `v->field_0x30`
for the PEB pointer, the same byte offsets `v[0xc]` spelled before), and
`ELFMAIN #1`/`#2`, where `elfmain off` leaves the entry's argument vector untyped
and the pass would read `argv[0]`/`argv[1]` as a two-field structure; that pass
pins the upstream form with `option structsynth off`, since an array of `char *`
is not a record. Over a 14-binary, 5,610-function `decompile-all` sweep (x86-64
coreutils, findutils, grep, gzip, bzip2, diffutils and tar at O0 and O2, and two
ARM32 firmwares) 483 functions change. 350 of them are the same statements with
each parameter access rewritten from its byte offset to a field; the other 133
were read by hand, and every one is a consequence of the pointee type rather than
a change of meaning: an index rescaled to the structure's size or spelled past its
end (`&a0[1].field_0x8`), an `undefined1` filler array decaying to its address,
constant byte stores merged into one store of the field's width, a load folded
into its use or a sum held in a temporary, a local taking the pointer type of the
field it was loaded from, a literal respelled for the field's signedness, and one
return block the structurer now duplicates (`findutils` `find` O2 `sub_f620`, whose
remaining `goto label_f752` keeps its label). No function in either arm has a
`goto` whose label is missing, and the 311 structures of eight serial project
exports compile with every `offsetof(struct_N, field_0xK)` equal to K.
Whole-binary `decompile-all` time moves between −3.4% and +2.9% (interleaved
min-of-15 over `fmt`, `ls` and `sort` at O2 and the 1.3 MB `bash` O2), against a
+5% budget.

**`nest` stays opt-in.** Against `param` on the same build it moves no datatest
assertion (675/675) and no stage assertion of any other feature (the only new
keys are `tests/stages/structsynth-nest.xml`'s, whose first pass pins `param`).
Over 15 binaries and 7,819 functions (x86-64 coreutils `fmt`, `ls`, `sort` and
`du`, findutils `find`, grep, gzip, bzip2, diffutils `diff` and tar, and the ARM32
firmwares `chibios` at O0 and O2-noinline and `freertos` at O0) 93 functions
change. 86 of them differ only in `struct_N` numbering, because a nested record is
minted before the record that holds it. The other 7 are reads respelled through
the nested record, together with two effects of the new pointee type: one local
that no longer shares a declaration with a word of another type, and one byte
address held in a temporary that its compare and its store both use. No control
flow moves, and nothing on ARM32 changes. The 444-slice typesweep is 1,349
perfect in both arms with no function better or worse, the TRex score is
unchanged, and 487 structures over ten project exports compile with every
`offsetof` exact. Serial whole-binary `decompile-all` time moves by −1.7% to −5.0%
against `param` (interleaved min-of-15 over `fmt`, `ls` and `sort` at O2 and `bash`
O2). What keeps it off by default is the sharded run. A structure with a nested
or self field has no recipe another process can rebuild, so `--jobs` names every
function with a synthesized structure in its one ordered worker, where `param`
renames most of them in place. `decompile-all --jobs 8` on tar O2 takes 26.0 s
under `nest` against 15.9 s under `param`, with output identical to the serial
run either way. Giving the shard protocol a recipe for a field that points at
another request's structure, or at its own, is what would make `nest` a default
candidate.

The cost is on decbench's `type_match`, and it is accepted: the metric compares
pointee spellings by name, so a synthesized `struct_0 *` can never intersect a
ground-truth `WORD *`, and over 444 slices the flip is worth −1 perfect function
(959 → 958) and −0.05% of the aggregate. Every decision it moves off a match is
a real false positive of one shape — a buffer of primitive elements the array
rule does not recognise: `factor`'s GMP limb arrays (`__uintmax_t *`, read at
offsets 0 and 8, two pointer-sized elements the uniform-run rule keeps as a
record) and `shred`'s seven-byte `char *` name buffer (stores of 4, 2 and 1
bytes). The other 1,295 decisions it touches were already misses: a
`struct_N *` where the ground truth names the aggregate (`Hash_table *`,
`stat *`, `fileinfo *`, …), which a metric that credits an anonymous structure
against a named one would count. The pass is not a member of any `--mode` preset
list: `reliable` is the shipped defaults, so it synthesizes `locals`;
`aggressive` inherits the default rather than pinning a value below `all`;
`fast` changes discovery only.

Type facts are *consumed* back into the graph by the typerecovery rules: the
`oppool2` pool (`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_5.rs
(RulePushPtr, RuleStructOffset0, RulePtrArith)`) materializes PTRADD/PTRSUB
member access from pointer types, and `ruleaction_6.rs (RulePtraddUndo,
RulePtrsubUndo)` in the main pool reverts them when the types they were built
from degrade — the visible ping that the 7-pass ceiling exists to bound.

## 5.3 Ranges & consume bits

The rest of the S5 fact fabric (the framing derives from the study in
the archived stage-model study (git history) §7.3; every claim below is re-verified against
the Rust).

**Circular ranges.**
`decompiler/crates/kuna-decomp/src/p5_types/rangeutil.rs (CircleRange)` is the
value domain: a half-open interval `[left, right)` on the circle of integers
mod 2^n, with an optional power-of-two step. The circle (rather than a plain
interval) is what lets one object exactly represent "signed negative", a
wrapped `x-c < k` guard, or a strided jump-table index. The per-opcode
`pull_back_*` operators invert an op's effect on a range (given the output
range, what was the input range), and `push_forward_*` run it forward. The
live consumers are (a) jump-table recovery — the guard analysis in
`decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs (GuardRecord,
circlerange_pull_back)` pulls the branch condition back to the switch variable
to bound the case count (chapter 02), and (b) the boolean-expression melding
rule `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs
(RuleRangeMeld)`, which pulls two comparison ranges back to a common Varnode
and intersects/unions them into one comparison.

**The value-set solver.** On top of `CircleRange` sits an abstract
interpretation layer (`ValueSet`, `ValueSetSolver`, same file): a
Bourdoncle-style weak topological ordering over the SSA constraint graph, with
widening to force loop convergence. The shipped strategies are read from the
code: `WidenerFull` widens at iteration **2** (snapping the unstable bound to a
"landmark" — typically the loop-guard constant — or its complement) and gives
up to full range at iteration **5**; `WidenerNone` freezes whatever has been
reached by iteration **3**. The reason for the two-stage schedule: one cheap
guess (the landmark) is usually exactly the loop bound, and if it is not, more
iteration is wasted work.

The solver is bound to the live IR. `decompiler/crates/kuna-decomp/src/p5_types/rangeutil.rs
(ValueSetSolver::establish_value_sets)` seeds the system from a set of sink
Varnodes (walking each sink's def chain and stopping at ops whose integer
range is unknowable — calls, LOADs, floating point — which enter as
full-range roots, with the stack-pointer input tracked as a *relative* set),
lifts every dominating CBRANCH condition into per-read equations
(`generate_constraints` / `apply_constraints`, using
`circlerange_pull_back` to pull the branch range back to a system Varnode and
`FlowBlock::restricted_by_conditional` to decide on which out-edge it holds),
and computes the weak topological order; `(ValueSetSolver::solve)` then
iterates per-opcode `push_forward_*` transfers over that order — looping each
partition component until it stabilizes — under a chosen `Widener`. Where the
C++ threads a `Varnode -> ValueSet` back-pointer and mark bits through the IR,
the port keeps a `VarnodeId -> node` map plus side sets on the solver, which
is observationally identical.

The solver's one in-tree client is the LoadGuard range refinement at
`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs (LoadGuard)`,
gated by `option loadguardrange` (default on): at the end of each heritage
pass the pointer of every newly discovered indexed-stack LOAD/STORE guard is
solved for its `[min,max,step]` window (chapter
[03](03-ssa-and-simplification.md)), and the refined, range-locked guards are
what let the P6 frame layout size an indexed stack array by its real index
bound (chapter [06](06-variables-and-merge.md)). The refinement is
load-bearing for correctness, not just precision: with it off, every guard
keeps the conservative whole-space range, and the P6 fallback bound of 3
splits any element past index 3 of an indexed array into a separate
never-assigned scalar — an *under*-sized array with out-of-bounds subscripts
in the printed C (GH-182), not merely extra heritage conservatism.

**Non-zero masks.** `decompiler/crates/kuna-decomp/src/substrate/
funcdata_varnode.rs (Funcdata::calc_nz_mask)`, driven once per `mainloop` pass
by `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionNonzeroMask)`, computes for every Varnode the mask of bits that can
possibly be non-zero: a forward DFS in post-order over the def-use graph, then
a fixpoint re-pass across the MULTIEQUAL loop edges the DFS clipped. The mask
feeds the boolean gate in §5.2, sub-variable flow (chapter 03), and dozens of
simplification rules.

**Consume bits.** The dual analysis, backwards:
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionDeadCode)` — group `deadcode`, classified P5 in
`decompiler/crates/kuna-decomp/phases.toml` because its artifact is a bit-level
fact even though its effect is deletion. Starting from ops whose effects are
observable (stores, calls, returns, branches), it pushes a per-Varnode
*consumed-bits* mask backwards through each defining op's transfer
(`dc_push_consumed` worklist), so e.g. a SUBPIECE consumes only the bytes it
extracts. Anything whose consume mask ends up empty is dead and is deleted —
this is the pass that destroys the lift-time ops phase 2 created. Its
pathological-case machinery lives in chapter 03's heritage: deletion is
suppressed for an address space still inside its *dead-code delay* window, and
a free Varnode reappearing at an already-heritaged address bumps the delay and
restarts (00-overview §0.7). Failure mode: consuming too little deletes a
computation the binary needed (the classic symptom is a wrong parameter list
feeding chapter 04's trial pruning); consuming too much merely leaves clutter
for later pools.

## 5.4 Union resolution

A union-typed value has no single correct field — each *access* has a correct
field. The artifact is therefore per-edge: `ResolvedUnion` records the winning
facet, keyed by `ResolveEdge` (type id + op/address encoding + op sequence
number, with the C++ map's exact total order —
`decompiler/crates/kuna-decomp/src/p5_types/unionresolve.rs (ResolveEdge)`),
cached per function in the `union_map`
(`decompiler/crates/kuna-decomp/src/p5_types/funcdata_union.rs
(Funcdata::get_union_field, Funcdata::set_union_field)`).

Resolution triggers wherever a `needs_resolution` type crosses an edge: during
propagation (§5.2), during cast planning, and in the printer's facing-type
lookups. The dispatch is
`decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
(Funcdata::resolve_in_flow)`; its decision rule for the simple case, in order:
(1) the per-edge cache; (2) an operator-asserted facet — a `map unionfacet`
DynamicHash symbol, consulted through
`funcdata_union.rs (Funcdata::get_address_based_union_field)`, which is the P0
override surface for this whole section (the `aggregate-union` row in
`decompiler/crates/kuna-decomp/phases.toml`); (3) the scoring engine.

**Facet trials and scoring.**
`decompiler/crates/kuna-decomp/src/p5_types/unionresolve_run.rs
(ScoreUnionFields)` fits every candidate field (plus "the union as a whole",
score index 0 / field −1) against the surrounding data-flow. From the access
edge it launches one *trial* per candidate and walks outward level by level —
down through readers, up through definitions — scoring each op it meets with
per-opcode metatype fit tables
(`decompiler/crates/kuna-decomp/src/p5_types/unionresolve.rs
(score_trial_down_pure, score_trial_up_pure)`): a pointer flowing into a LOAD
scores well, an enum flowing into FLOAT_ADD scores badly. Trials stop and
score terminally at type locks (`score_locked_type`), locked
parameters/returns (`score_parameter`, `score_return_type`), truncations
(`score_truncation`), and constants (`score_constant_fit` — including a
"looks like a pointer" bounds test). The budgets are read from the code
(`unionresolve.rs`): at most **6** levels (`MAX_PASSES`), no new level once
**256** trials have run (`THRESHOLD`), hard stop at **1024** trials
(`MAX_TRIALS`) — a union inside a big expression web must not turn one
resolution into a whole-function analysis. The winner is the highest total
(`compute_best_index`; strict `>`, ties keep the earliest field, whole-union
first). Double-counting is prevented by a visited set keyed on the (Varnode, candidate-field score index) pair (`unionresolve_run.rs (VisitMark)`), so one Varnode is scored at most once per candidate facet.

Failure mode: scoring is heuristic, and a wrong facet renders the wrong member
access on every edge that shares the cached resolution; because the cache is
per-edge, one early bad pick does not poison other accesses, and the
`map unionfacet` assertion is the surgical override. The scoring weights
themselves are deliberately latent (no option) — matching upstream, where they
are compile-time.

## 5.5 Double precision

Compilers split a 2N-byte value into two N-byte registers; the IR then shows
every operation twice (lo half, hi half with carry), glued by SUBPIECE/PIECE.
`decompiler/crates/kuna-decomp/src/p5_types/double.rs` recovers the logical
whole. All four driver rules ride the main `oppool1` / `doubleload` groups in
the schedule (`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`).

**Marking.** `double.rs (RuleDoubleIn)` fires on SUBPIECE. Decision rule
(`attempt_marking`): the SUBPIECE must truncate *exactly the top half* of a
whole that is credibly one logical value — a type-locked primitive input, or
the output of an arithmetic/floating-point op — and a companion SUBPIECE of
the bottom half must exist. Both pieces get precision marks (`precis_lo` /
`precis_hi`). The producing-op restriction exists because for logical ops
there is no way to tell whether the whole was ever "one value"; marking a
coincidental pair would fuse unrelated variables.

**Pairing and rewrite.** Once marked, `double.rs (SplitVarnode)` describes the
hi/lo pair (plus the whole, when one exists — `whole_list` re-finds it from
the SUBPIECE fan-out). Each application pushes the logical operation **one
level** through the graph: `double.rs (apply_rule_in)` walks the pair's
readers and dispatches to the per-opcode `*Form` matcher families — `AddForm`/
`SubForm` for the add/carry and sub/borrow cascades, `LogicalForm` for
paired AND/OR/XOR, `Equal1Form`/`Equal2Form`/`Equal3Form` for the three
equality shapes, `LessThreeWay`/`LessConstForm` for the compare cascades,
`ShiftForm`, `MultForm` (the three-multiply 2N×2N pattern), `PhiForm` for
paired MULTIEQUALs, `IndirectForm`, and `CopyForceForm`. Every form runs a
full `verify` of the exact two-halves pattern — same operand order, same
tie-breakers as upstream — **before** mutating anything, then replaces the
half-ops with the single whole-width op; the halves die by consume-bit
analysis (§5.3). One level per rule firing means the repeat-applied pool
unzips an arbitrarily long cascade pass by pass.

`double.rs (RuleDoubleOut)` runs the same forms anchored at a PIECE (the value
is *built* from halves rather than split into them), merging two persistent
input halves into one logical input via `combine_input_varnodes`.
`RuleDoubleLoad` / `RuleDoubleStore` (same file) fuse two adjacent half-width
LOADs/STOREs into one whole-width access — requiring address contiguity in
the right endian order and proving no interfering write between the two ops.

**When it wins/loses.** It wins when the compiler's lowering kept the standard
shapes: the output shows one 2N-bit variable with ordinary arithmetic. It
declines — silently and safely — when the marking guards fail, when a
consumer shape matches no form, when the function still has unreachable
blocks (`RuleDoubleIn` waits, since dead code fakes patterns), or when the
whole would exceed 8 bytes as a constant (the `uintb` precision bound, `double.rs
(SIZEOF_UINTB)`). The failure rendering is not wrong code but *unfused* code:
`CONCAT`/`SUB` pseudo-ops and doubled arithmetic in the output. Because every
form verifies before rewriting, a wrong fuse is designed out rather than
detected after.

## 5.6 kuna extensions & the late rewrite families

The remaining `p5_types` passes are the late aggregate rewrites: they run in
the 22-rule `cleanup` pool *after* `fullloop` exits (00-overview §0.6), when
types and symbols are final enough to justify rewriting memory idioms.
Defaults below are stated from `decompiler/crates/kuna-decomp/phases.toml`.

**Constant sequences** (`decompiler/crates/kuna-decomp/src/p5_types/constseq.rs`).
Pattern: code writes a string one character at a time — a run of constant
COPYs into a stack/global char array, or constant STOREs through a heap
pointer. The shared base `constseq.rs (ArraySequence)` owns the discipline:
gather the sibling writes in the same block, keep the maximal window with no
interfering LOAD/STORE/CALL between members (`check_interference`), and
assemble the constants into one byte array by offset with endian-correct
unpacking, a single NUL allowed, contiguity required, and at least **4**
elements (`ArraySequence::MINIMUM_SEQUENCE_LENGTH`; upper bound 0x20000). The
two drivers are `constseq.rs (RuleStringCopy)` — COPY-into-array, requiring
the destination be an address-tied char array backed by a symbol container —
and `constseq.rs (RuleStringStore)` — STORE-through-pointer
(`HeapSequence`), which reconstructs the base pointer and per-store offsets
through the PTRADD/ADD forest. Rewrite: the run collapses to one
`memcpy`/`strncpy`/`wcsncpy` builtin CALLOTHER whose source is an internal
string the printer renders as a quoted literal. Failure mode: a guard miss
(interference, gap, non-printable bytes) declines and the per-element
assignments simply remain; the interference check exists because moving all
the writes to one call site is only sound if nothing observed the array
half-written.

**(kuna GH-9230/1537) Constant fill —**
[`memsetrecover`](../options.md)**, default on** (DIV-2).
`decompiler/crates/kuna-decomp/src/p5_types/kuna_memsetsequence.rs
(RuleMemsetCopy)` extends the same machinery to runs that spell no string: an
unrolled or SIMD `memset`/`bzero` otherwise renders as dozens of
`buf[i] = '\0';` stores. It reuses the string driver's collection
(`constseq.rs (StringSequence)` `build_for_fill`) and applies the fill test
`kuna_memsetsequence.rs (detect_fill_run)`: sorted by offset, the COPYs must
tile a contiguous region with one repeated fill byte, with **at least 2 COPYs
and a 16-byte minimum footprint** — the guard that keeps a lone string NUL
terminator from being claimed as a memset (the Stack-string ablation in
DIV-2). Rewrite: one `builtin_memset(dest, value, count)` CALLOTHER; teardown
shares the string path's COPY removal. Off restores the per-element stores.

**(kuna) Read-only string block copy —**
[`rodatastring`](../options.md)**, default on** (DIV-113).
`decompiler/crates/kuna-decomp/src/p5_types/kuna_rodatastring.rs
(RuleRodataStringCopy)` covers the third shape of the same idiom: the whole
literal already exists in read-only memory, so the compiler emits a BLOCK copy
— one or more wide loads out of `.rodata`/`__cstring` re-stored into the frame
— instead of per-character constants. Those loads survive heritage as free
read-only memory varnodes rather than p-code constants, so `RuleStringCopy`
declines at its constant-input guard and the run reaches the printer as
partial-symbol slice assignments: `v1[0] = (char[8])s_100003f1d._0_8_;` and
`v8._0_9_ = s_100003f1d._16_9_;` — neither of which is legal C (there is no
array cast, and `._0_9_` is member syntax applied to an array object), and
which hide a string the engine has already recovered at that address.

The rule claims a run only when every step is a fact rather than an inference:
each COPY's source is `Varnode::isReadOnly` free memory (so the image bytes
*are* the run-time bytes); all the sources lie inside one covering data symbol
whose type is a char-printable array — the symbol the string-literal analysis
planted; source and destination advance in lockstep, so the run is a straight
block copy and not a shuffle; the COPYs tile the destination **exactly**, no
gap and no overlap, across the symbol's whole length, so nothing is invented
and nothing is dropped; the image bytes really are one NUL-terminated string of
exactly that length; and the members share a basic block with no interfering
LOAD/STORE/CALL between them (the same `ArraySequence::interfereBetween` window
the string driver demands). A run of a single COPY is deliberately left alone —
the defect being repaired is the *split* copy, and a whole-string single COPY
already renders as one assignment. Rewrite: one
`builtin_strncpy(dest, "…", n)` CALLOTHER built by `constseq.rs
(StringSequence)` `from_rodata_run`/`transform_rodata`, reusing the string
path's `constructTypedPointer` and COPY teardown unchanged. Off restores the
slice assignments. Failure mode: any guard miss declines silently and the
output is byte-identical to `off` — the destination stack slices the run wrote
survive as unread declarations, because the local variable map still sees the
frame carved by the original wide stores.

**Bitfields** (`decompiler/crates/kuna-decomp/src/p5_types/bitfield.rs`).
Pattern: a struct with sub-byte fields is accessed through shift/mask soup on
a byte container. The six `cleanup` rules fire only when the container's type
*has* declared bitfields (`Datatype::has_bitfields` — the triples collected
from `TypeBitField` in `dtype.rs`): `RuleBitFieldStore`/`RuleBitFieldOut`
trace backward from a store (or mapped write) through the OR/AND/SHIFT web and
re-express it as explicit `INSERT` ops per field
(`decompiler/crates/kuna-decomp/src/p5_types/bitfield/insert.rs
(BitFieldInsertTransform)`); `RuleBitFieldLoad`/`RuleBitFieldIn` trace forward
from a load and re-express the extractions as sign/zero `PULL` ops
(`bitfield/pull.rs (BitFieldPullTransform)`); `RulePullAbsorb`/
`RuleInsertAbsorb` (`bitfield/absorb.rs`) then consolidate a shared byte
container so each field renders as its own `ptr->field = …` statement. All
geometry runs through the endian-aware `bitfield.rs (BitRange)` value type —
bit numbering is where big/little endian diverge, and getting it wrong scrambles
adjacent fields. Failure mode: any trace step the transform cannot prove
(e.g. a masked value escaping to an op outside the recognized web) declines
before mutation, leaving the raw shift/mask expressions in the output; the
type is never consulted speculatively, so untyped code is untouched.

**Preferred splits**
(`decompiler/crates/kuna-decomp/src/p5_types/prefersplit.rs
(PreferSplitManager)`). The inverse of §5.5: some processors keep two logical
values in one physical register (SIMD halves), and the spec can declare a
`<prefersplit>` table of storage+offset records. Wherever the whole register
appears as the single producer/consumer of a COPY/PIECE/SUBPIECE/LOAD/STORE/
INT_ZEXT, the manager rewrites that op into two piece-ops (each opcode has a
paired `test*`/`split*` guard, and the op-insertion order is transcribed
exactly because it is output-determining); a second sweep (`split_additional`)
cleans up temporaries the first sweep exposed. Honest port status: the
transforms are ported and unit-tested, but the pass-0 heritage hook that
drives them (`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs
(Heritage::heritage)`) is a documented stub — inert for every architecture
without split records, which is the entire current test surface, so no live
output depends on it yet.
