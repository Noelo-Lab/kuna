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

When `ptrfromuse` is `byte` or `void` (shipped `off`),
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

When `boolbyte` is `on` (shipped `off`),
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
  This is why the option is a judgment call and ships off.

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
  (`kuna_boolbyte.rs (truncation_prints_as_cast)`, consulted by
  `printc.rs (subpiece_is_cast)`), so a truncation into a `bool` prints as the
  `(bool)` cast it is. The seed that put `bool` there already required the value
  reaching it to carry a non-zero mask of at most 1, so the cast and the low byte
  agree. The arm belongs to the option: with `boolbyte off` the printer is
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

`tests/stages/kuna-boolbyte.xml` pins the witness, the five refusals - a byte
that is also widened and added, a byte tested for its low bit, a byte stored
through a pointer, a stack byte set to 0 and then filled by a callee, and a byte
parameter copied into a slot a callee is handed the address of - and the
truncation rendering, whose first pass is the `SUB41` the printer emits without
the arm.

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
[`structsynth`](../options.md) (`off|param`, default `off`) invents the missing
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

A pair of pointer-sized members is the one run the width rule cannot settle, so
the pass looks at what the pair holds. A pair of pointers is kept: every one
DWARF can type in the census (30 of 35) is a record, `hash_table`'s `bucket`
and `bucket_limit` among them. A pair of **same-typed integers** is declined when it says nothing
the base's own pointer does not. `unsigned long *a0` read at `a0[0]` and `a0[1]`
and `struct_0 *a0` over `{ unsigned long field_0x0; unsigned long field_0x8; }`
are the same bytes, and only the second asserts a record, which two equal-width
reads cannot tell from an array. `factor`'s double-limb arithmetic is that shape
(`powm2` and `millerrabin2`, DWARF `uintmax_t *`), and there the structure
replaced a correct element pointer with a name nothing matches. The test
(`is_element_run`) needs all three of: every *raw* access is one width `w` at
exactly `0, w, 2w, ...`, judged before the widest-wins merge and the prune, so
an access the prune drops still counts as heterogeneous evidence (the four-byte
read inside a word in `structsynth-overlap-layout.xml` keeps its structure);
every field the pass would declare has the same integer metatype (`int`, `uint`
or unknown, so a pair that differs in signedness is kept); and the base already
points at an integer or `undefined` of width `w`. The last clause keeps a
`char *` read as two words, where the structure is what says "8 bytes". Since
runs of three or more already decline on width, in practice this rule decides
`{0:8, 8:8}` pairs. It is a trade, and the census states it: of the 93
same-typed integer pairs the pass minted over 16 x86-64 binaries (coreutils
`fmt`/`ls`/`sort`/`du`/`factor`/`shred`, `tar`, `find`, at `-O0` and `-O2`), the
rule declines 92, and DWARF calls 12 of those integer pointers, 29 `void *`,
5 untyped and 46 structure pointers. For those 46, mostly `stat`'s leading
`st_dev`/`st_ino`, `timespec` and `hash_entry`, the declined layout had the right
offsets and sizes, and 20 correctly placed fields leave the layout-F1 instrument.
They score the same on `type_match` either way, since a synthesized name never
matches a DWARF name, while the 12 integer pointers are exactly the rows the
option used to lose there.

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
by accident.
A field takes the type of the value the access carried when the widths agree and
the type's C spelling is its own width — a scalar or a pointer — and
`undefined<N>` otherwise.

Names are program-wide `struct_N`, probed with `TypeFactory::find_by_name` and
reused whenever the layout signature — `(offset, size, metatype, pointee name)`
per field plus the total size — matches exactly. Every mint declines on an error
rather than propagating one: `find_add` rejects a second, different definition of
a held name. Dedup is exact, not subsumptive, so two functions that touch
overlapping but unequal subsets of one real structure still get two names. The
structure is **completed before** its pointer is taken, because completing a
structure mints a fresh `Rc` and the merge tests compare high types by pointer
identity. The install itself is `funcdata.rs (vn_update_type_locked)`, which
pairs `Varnode::update_type_locked` with `HighVariable::type_dirty()` — without
the notification `high_get_type` keeps handing back the stale type, and the
printed prototype never changes. The change is signalled by bumping the action's
`count`, the only signal `Action::perform` reads.

Because the ledger lives in the program's `TypeFactory`, `struct_N` is
program-wide under `kuna decompile-all` and `kuna decompile-project`, which load
once; `kuna decompile` spawns one engine per function, so there each function
numbers from `struct_0` again.

The synthesized layout is printable rather than only inferable: the P9 option
[`structdefs`](../options.md) prints the definition of every composite a
function's C names reach, so `--option structdefs on --option structsynth param`
puts `struct struct_0 { ... };` — filler members and all — above the function
whose parameter this pass retyped, and carries the same text in the per-function
`types` array of `decompile-all --json`. Neither option is on by default, so the
pair changes nothing unless both are asked for.

**The default is `off`, and it is off on evidence.** Flipping it to `param` and
re-running the corpora moves **no** datatest assertion (675/675) and four stage
assertions, each of them the intended rendering — `ELFMAIN #1`/`#2`, where the
entry's untyped argument vector becomes `struct_0 *` and `a1[1]` becomes
`a1->field_0x8`, and `PEBNAMES-X86 #6`/`#8`, where an untyped FS-segment base
becomes `struct_0 *` and `v4[0xc]` becomes `v4->field_0x30`, the same byte
offset rescaled. Speed is not the reason either: whole-binary `decompile-all`
moves between −1.05% and +1.34% over six binaries, inside the movement of an
inert control binary on the same run. Two things keep it opt-in. On
`decbench`'s `type_match` over 444 slices the flip is worth −1 perfect function
(959 → 958) and −0.049% of the aggregate, because the metric compares pointee
spellings by name and a synthesized `struct_0 *` can never intersect a
ground-truth `WORD *`. And the type lock changes which blocks the structurer
duplicates: over a 15-binary, 5,431-function sweep, one more function
(`findutils` `find` O2 `sub_f620`) lands on the emitter defect where a `goto`
survives but its target label is never written — a defect already present in
nine functions of the same corpus with the option off. A pass that reshapes
block duplication stays opt-in until that is fixed, and for the same reason it
is not a member of the `aggressive` preset.

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
