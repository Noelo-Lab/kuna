# cppproto — analysis

## 1. The measured symptom

On a purpose-built `-g` C++ probe (namespace function, class with out-of-line
members including a `static` one, a template instantiated twice), kuna applied
DWARF parameter names and types to exactly the functions whose name contained no
`::` and no `<`:

| kuna signature (before) | DWARF applied? |
|---|---|
| `probe_virtual_call(void *s,int4 scale)` | names yes, type NO (`void *`, not `Shape *`) |
| `probe_template_double(float8 a,float8 b)` | yes |
| `db::inner::scaled_add(int4 a0,int4 a1)` | **no** |
| `maxof<int>(int4 a0,int4 a1)` | **no** |
| `Account::deposit(int4 *a0,int4 a1)` | **no** |
| `Shape::shape_count(int4 a0)` (static, no `this`) | **no** |

Ghidra 12.1 on the same binary emits
`int __thiscall Account::deposit(Account *this,int amount)`.

## 2. The root cause — proved, and NOT the first hypothesis

The hypothesis handed to this work was that the DIE-to-function binding matched on
the demangled name instead of `DW_AT_low_pc`. That is a real defect, but it is
not what produced the `::`-or-`<` split. Reading the probe's `.debug_info`
(`readelf --debug-dump=info`) settles it:

```
 <1><9a81>: Abbrev Number: 43 (DW_TAG_subprogram)     <- db::inner::scaled_add
    <9a82>   DW_AT_specification: <0x9a65>
    <9a86>   DW_AT_low_pc      : 0x258a
 <1><915a>: Abbrev Number: 97 (DW_TAG_subprogram)     <- Account::deposit
    <915b>   DW_AT_specification: <0x6f65>
    <9165>   DW_AT_low_pc      : 0x271c
 <1><98f1>: Abbrev Number: 43 (DW_TAG_subprogram)     <- Shape::shape_count (static)
    <98f2>   DW_AT_specification: <0x753d>
    <98f6>   DW_AT_low_pc      : 0x332b
```

Every one of them is a subprogram DEFINITION with **no `DW_AT_name` of its own**.
C++ splits a definition from its declaration; the name, and often the return
type, live on the `DW_AT_specification` target. kuna's walk guards with

```rust
if snap.declaration || snap.name.is_empty() { continue; }
```

so the whole DIE — name, prototype **and** the `DW_OP_fbreg` stack locals — was
dropped before any binding was attempted. The `static` members lose their names
for exactly this reason, which is why the artificial-`this` DIE was never the
cause.

Two corrections to theories that were in play:

* The walk **does** descend into `DW_TAG_namespace`. `snapshot_unit` snapshots
  every DIE in the unit and the subprogram arm iterates `dies.values()` with no
  depth filter, so a namespace-nested subprogram was always visited. Namespace
  handling matters only for building the qualified *name*, not for discovery.
* The definitions of namespaced functions are emitted at CU top level anyway —
  `db::inner::scaled_add`'s definition is a depth-1 DIE. Only its *declaration*
  sits inside the namespace.

### 2a. The second, independent defect: name-keyed binding

`maxof<int>` **does** carry its own `DW_AT_name`, so it survived the guard — and
still lost its prototype. `kuna functions` shows why:

```
0x39d0 name='maxof'  aliases=['maxof<int>', 'sub_39d0']
```

kuna's symbol table files the demangled template name as `maxof`, so
`Architecture::set_function_prototype_pieces("maxof<int>")` ->
`query_global_function` -> no match -> silent no-op. The same by-name park misses
every qualified name for a different reason: `find_create_scope_from_symbol_name`
files `Account::deposit` in a nested scope `Account`, and
`query_function_by_name` climbs to parents from the global scope — it never
descends. Meanwhile the READ side is already keyed by address
(`Database::function_proto_pieces(scope, addr)`), so the round trip was broken at
the write end only.

## 3. Three type-mapper defects on the same path

1. `DW_TAG_class_type` fell through to `_ => None`, so every `Foo *this`
   degraded to `void *` (the pointer arm's `.or_else(get_type_void)`).
2. `build_pieces` used `build_datatype(...)?`, so ONE unmappable parameter type
   discarded the entire prototype rather than that one parameter.
3. Not in the brief, found while fixing (1): `MAX_TYPE_DEPTH` is 3 and counts raw
   DIE hops, including the transparent `typedef`/`const`/`volatile`/`restrict`
   ones. A `const` member function's `this` is `const Account *const` — four DIEs
   — so `Account::available`'s `this` still came out `void *` after (1) was
   fixed. The same cap is why plain C `char **authors` (`const char *const *`),
   `idx_t *pn` and `mbstate_t *ps` rendered `void *` in coreutils `fmt`.

## 4. What the whole-binary sweep found (and cost)

Sweeping `decompile-all` with the gate off vs on, and classifying every hunk,
turned up three defects that were invisible on the witness — the normal failure
mode this discipline exists to catch.

**4.1 GCC IPA clones (`.isra`/`.constprop`/`.part`).** A clone is also a nameless
`DW_TAG_subprogram`, but linked with `DW_AT_abstract_origin`, and its signature
is deliberately NOT the original's. In coreutils `fmt`:

```
 <1><27d2>: DW_TAG_subprogram          <- put_word.isra.1 @ 0x23d0
    <27d3>   DW_AT_abstract_origin: <0xabf>       (put_word)
 <2><27ed>: DW_TAG_formal_parameter
    <27ee>   DW_AT_abstract_origin: <0xacd>       (Word *w)
 <2><27f2>: DW_TAG_formal_parameter
    <27f3>   DW_AT_abstract_origin: <0xacd>       (Word *w)   <- the SAME one
```

IPA-SRA split the aggregate into two scalars and pointed both at the one source
parameter, so the chase produced `put_word(Word *w,Word *w)` and pushed a
struct-pointer type onto what is really a length — at the definition and at every
`put_line` call site. **Decision: follow `DW_AT_specification` only.** The cost is
real and accepted (a concrete out-of-line instance — a destructor body, a `.cold`
part — recovers nothing), and the entire C++ payoff is untouched because member
and namespace definitions use `DW_AT_specification`.

**4.2 Variadic-template arity.** `std::vector<int>::emplace_back<int>` keeps its
expanded pack argument inside a `DW_TAG_GNU_formal_parameter_pack` grouping DIE,
not as a direct child. Scanning only direct children locked a one-parameter
signature onto a two-parameter function, and the argument vanished at every call
site. Fixed by flattening the pack.

**4.3 Zero-width by-value aggregates.** A class maps to a named OPAQUE type of
size 0 (fields are the sibling increment). Handing storage assignment a
zero-width by-value parameter did not merely lose precision — it failed
`std::vector<int>::_M_realloc_insert` outright with "dereferencing a
null/sentinel space pointer", i.e. the function stopped decompiling at all.
Fixed by degrading any non-positive-size parameter type to an `undefined<n>` at
the DIE's own `DW_AT_byte_size`, which keeps the width — and therefore the
storage — right.

**4.4 Anonymous-aggregate name collision.** Naming every unnamed struct
`anon_struct` interns unrelated types under one name; the qualifier-strip of
section 3.3 made that reachable in many more places. Fixed by naming an anonymous
aggregate after the typedef it was reached through, which is also what Ghidra's
importer does: `__mbstate_t *ps`, not `anon_struct *ps`.

## 5. Final sweep results

| binary | functions | changed | new errors | arity drops |
|---|---|---|---|---|
| `regglobal_fmt_x86_64` (real C, coreutils `fmt`, `-O2 -g`) | 169 | 9 | 0 | 0 |
| C++ probe `-O0` (STL) | 190 | 83 | 0 | 0 |
| C++ probe `-O2` (STL) | 72 | 7 | 0 | 0 |
| `dwarf_globals_x86_64` (small C `-O0 -g`) | 11 | 0 | 0 | 0 |

Every one of the 9 C hunks is an improvement: four `void *` parameters resolved
to their real pointee once qualifier hops stopped consuming the depth budget
(`char **authors`, `int8 *pn`, `uint8 *val`, `__mbstate_t *ps`) and one DWARF
parameter name recovered (`bool same_paragraph`). No plain-C function lost
information, errored, or changed arity.

## 6. Deliberately out of scope

* **Struct/class fields.** A class stays a named opaque, so `this->balance` still
  prints as `*(int4 *)((int8)this + 4)`. Populating `DW_TAG_member` /
  `DW_AT_data_member_location` is the sibling increment; its seam is
  `build_datatype`'s struct/class arms, which this change leaves structurally
  untouched.
* **`__thiscall`.** `DW_AT_object_pointer` is the attribute Ghidra's
  `ThisCallingConventionDWARFFunctionFixup` uses to select the `__thiscall`
  model. kuna cannot register one yet (`ProtoModel::decode` is a stub), so the
  artificial `this` is carried as an ordinary first parameter — which is what the
  x86-64 SysV ABI does with it regardless. Noting the attribute here is the seam
  for that later work.
