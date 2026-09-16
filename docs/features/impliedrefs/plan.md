# impliedrefs / termdup — implementation plan

## 1. The change

One new module, `p6_variables/kuna_impliedrefs.rs`, holding the two option
parse bodies and the shared bound parser:

```rust
pub struct OptionImpliedRefs;   // -> Architecture::max_implied_ref
pub struct OptionTermDup;       // -> Architecture::max_term_duplication
```

Two arms in `Architecture::set_kuna_option` write the parsed value into the field
the pass already reads, and two names join `KUNA_OPTION_NAMES`. Nothing in
`coreaction_cleanup.rs` changes: `base_explicit` and `process_multiplier` were
already reading `data.get_arch().max_implied_ref` / `.max_term_duplication` off
the per-function `ArchSeam` context, which `Architecture` refreshes from these
fields at every function, so setting the field is the whole mechanism.

## 2. The value vocabulary

`values = "2|3|4|<n>"`, `default = "2"`. The parser is deliberately stricter than
upstream's `istringstream >> int` helper (`parse_int_auto`, which returns 0 for a
field with no leading digit): a field that is not wholly decimal digits is an
error, so `option impliedrefs wide` is reported instead of quietly asking for
"declare everything". The accepted range is 0..=1000000; the ceiling is the value
`base_explicit` already uses as its never-explicit sentinel for a spacebase
PTRSUB, so nothing above it can mean anything new.

A three-or-more-valued option carries no `live_field`: the codegen live reader is
a `flag ? live_true : live_false` ternary, which cannot express an integer. Both
names therefore join the no-live-reader allow-list in
`kuna_phases/tests.rs::option_values_live_value_present_for_85`, next to
`int3pad`, `warnstyle` and `namestyle`, and that count does not move.

## 3. Default identity

The defaults are the constants, so the defaults must be byte-identical. Proven
three ways:

* `make test` 675/675 and `make test-stages` PARITY OK;
* `kuna decompile-all fmt` with no options == with `--option impliedrefs 2` ==
  with `--option termdup 2`, byte for byte;
* `decompile-all` over 20 decbench ELFs (12 O2, 8 O0; coreutils, bzip2, chibios,
  sysvinit, shadow, diffutils, dpkg, e2fsprogs), this branch's `kuna` vs the
  `main` build: 152,308 lines of emitted C, 0 differing.

## 4. Test

`tests/stages/kuna-impliedrefs.xml`, one synthetic `<bytechunk>` carrying two
gcc -O2 functions and four passes:

| source | the bound it exercises | pass 1 (default) | pass 2 (option) |
|---|---|---|---|
| `int f(int **pp) { int *q = pp[1]; return q[0]+q[1]+q[2]; }` | three readers, two terms | `int4 *v1;` declared, read at all three uses | `option impliedrefs 4`: no declaration, load written out at each use |
| `int h(int a,int b,int c) { int t=a+b+c; return t*t; }` | two readers, three terms | the sum gets its own statement, read by name twice | `option termdup 3`: statement gone, sum duplicated at both uses |

Each witness isolates one bound: `f`'s expression is two terms, so `termdup`
cannot reach it, and `h`'s value has two readers, so `impliedrefs` cannot.

Plus three unit tests in the new module (shipped value parses to the upstream
constant; a higher bound parses and is confirmed; `""`, `-1`, `wide`, `1000001`
and `2.5` are all refused).

## 5. Count sites moved

`settables` 205 -> 207, tiers `(66, 78, 61)` -> `(66, 80, 61)`, catalog JSON
records 204 -> 206, `catalog_bytecompat.rs` fixture recaptured,
`tests/stages/kuna-catalog.xml` bucket #8 (`opt-in-tool`) 18 -> 20, stage corpus
count in `kuna-base/src/xml.rs` 293 -> 294, `docs/baseline-stages.json`
re-recorded (1007 -> 1013 keys), `docs/options.md` regenerated.
