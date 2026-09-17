# structdefs — analysis

## The gap

kuna recovers composite layouts and then does not show them on the surface a
caller reads. `kuna decompile ./fmt get_prefix` prints

```
int get_prefix(_IO_FILE *f)
{
  ...
  v4 = (unsigned char *)f->_IO_read_ptr;
```

and nothing in the output says what an `_IO_FILE` is. The layout the field
accesses are printed *against* is real — it is in the type factory, and
`kuna decompile-project` writes it into the export's `.h` — but reaching it
means exporting a whole project folder for one function. On a stripped binary
the same thing happens one level down: `libctypes` gives the argument the named
opaque shell `FILE *`, which is strictly better than `long`, and the caller
still has nothing that says `FILE` is a struct at all.

angr prints the typedefs a function references above it, which is a large part
of why its output reads struct-aware. kuna had the renderer
(`PrintC::doc_type_definitions`, the `docTypeDefinitions` port) and only a
whole-project caller for it.

## What the option does

`option structdefs on` (P9, default off) prints the definitions of the
composite, enum and typedef types the function's own C names, above the
function, in definition-before-use order.

```
$ kuna decompile ./fmt get_prefix --option structdefs on
typedef struct _IO_marker _IO_marker; /* opaque */
typedef struct _IO_FILE _IO_FILE;
typedef struct _IO_codecvt _IO_codecvt; /* opaque */
typedef struct _IO_wide_data _IO_wide_data; /* opaque */

struct _IO_FILE {
    int _flags;
    undefined1 _pad4[4];
    char *_IO_read_ptr;
    ...
};

int get_prefix(_IO_FILE *f)
```

## The in-repo witness

`dwarfstructs_x86_64` carries a self-referential `struct Node`, which is the case
kuna's forward-declaration-first rendering exists for:

```
$ kuna decompile .../fixtures/dwarfstructs_x86_64 walk_list --option structdefs on
typedef struct Node Node;

struct Node {
    int val;
    undefined1 _pad4[4];
    Node *next;
};

int walk_list(Node *n)
```

## Which types, and why those

The roots are the function's semantic type surface: the prototype's return and
parameter types, every Varnode's data-type, and every mapped Symbol type behind
a HighVariable. From each root the walk descends through the typedef base and
the component sub-types `Datatype::get_depend` reports (a pointer's pointee, an
array's element, a struct's fields) and pushes each definable type in postorder.

That set is a **superset** of the type names the printer can spell: a local
declaration is read off a high or its mapped symbol, the signature off the
prototype, a cast off the Varnode type it casts to. So the preamble cannot omit
a definition the body refers to. Collecting from the emitted token stream
instead would be exact for names that print, and would miss the struct behind
`p->field_0x8`, whose tag name the C never prints at all. The cost of the
choice is the opposite error: a type carried by a Varnode that contributes no
token can be defined above a function that never names it.

Core types are never defined (they are the language's, and the project export
declares them once in its recompile prelude). An incomplete struct prints as its
forward declaration alone, `typedef struct FILE FILE; /* opaque */`.

## One renderer, two surfaces

The text is `printc::render_type_definitions` — the same renderer that builds
`decompile-project`'s `.h` type block — over the referenced subset, so a
preamble line and a header line for one type are the same line. A project
export's `.c` bodies therefore suppress the preamble: they include the header
that already carries every definition, and printing them again above each body
would redefine them. Measured: a `decompile-project` run with `--option
structdefs on` is byte-identical to one without, serially, under `--jobs 2`, and
under `--stream`.

The preamble is documentation, not a translation unit — an `undefined1` padding
member needs the export's recompile prelude to compile — which is the other half
of why the definitions stay in the header on the surface that is meant to
rebuild.

## Line numbers stay honest

The preamble goes through the emitter, before `begin_function`, rather than
being prepended to the finished string, so every line-indexed surface counts it
the same way the text does. With the option on, `walk_list`'s
`line_mappings[0].line_number` is 13 and line 13 of `code` is `if (n) {` — the
eight preamble lines shifted both together.

## Measurements

**Default is byte-identical to main.** Six whole-binary `decompile-all` runs
(66,761 lines of C) against a main build: identical, every one.

| binary | lines | vs main (default) |
|---|---|---|
| coreutils fmt O2 stripped | 4,070 | identical |
| grep O2 stripped | 18,251 | identical |
| gzip O0 stripped | 6,787 | identical |
| bzip2 O2 stripped | 9,765 | identical |
| coreutils ls O0 unstripped | 11,958 | identical |
| diffutils diff O2 stripped | 15,930 | identical |

**With the option on, every hunk is an insertion.** The same six binaries,
option on vs option off, every unified-diff hunk classified
(`classify-hunks.py`, output in `corpus-sweep-on-hunks.json`):

```
TOTAL hunks=523 added=8657 deleted=0 non_definition_added=0
```

Zero deleted lines and zero added lines that are not type-definition text: the
option adds definitions above functions and changes nothing else. The
distribution is the expected one — 8,071 of those 8,657 lines are on `ls`, the
one unstripped binary in the set, where DWARF gives most functions a real
multi-field struct; the five stripped binaries average 117 added lines each
(one opaque `FILE`/`option` typedef per function that touches one).

**The `types` array is readable by the layout instrument as it stands.**
`scripts/decbench/structscore.py`'s `header_layouts` — unchanged — parses the
concatenated `functions[].types[].definition` of one `decompile-all --json` run
over `dwarfstructs_x86_64` into seven struct layouts (`Big24`, `Bits`, `Nest`,
`Node`, `P8`, `Same`, `Same_16`; saved in `structscore-reads-types.json`), so the
layout side no longer needs a project export to score a binary.

**Metric: neutral, by construction.** decbench's `type_match` reads the
`--json` `variables[]` array, which this option does not touch; the C text it
does touch is read only for functions whose `variables[]` is empty. The `types`
array is new and no metric reads it yet — it exists so
`scripts/decbench/structscore.py`'s layout side can read a recovered layout from
the per-function record instead of parsing the project header.

**Speed.** Default off costs one bool test. The on arm is measured in
`record.json`; the box was under three-lane load throughout, so the honest
statistic is the interleaved minimum, not the median (on one run the OFF median
came out *above* the ON median).
