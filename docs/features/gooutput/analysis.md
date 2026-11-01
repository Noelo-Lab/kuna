# Missing Go output selection

A native Go 1.22.5 noinline add function decompiles as:

```c
int main.add(int a0,int a1)
{
  return a0 + a1;
}
```

Requesting `--language go` fails with `unknown output language "go" (expected
auto, or one of: c-language, rust-language)`. The source, binary and captured
outputs are in the local research bundle. The function itself is recovered;
the output-language registry has no Go entry.

Pinned angr's TestGoTypeSpelling and TestGoCodegenHandlers run locally: five
tests pass, covering scalar/pointer/array/function spelling, handler coverage,
register fallback, precedence and field-gap fallback. The reference fixture
fauxware is 8,776 bytes, SHA256
c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b.

Kuna already centralizes language profiles and several statement-form dispatch
points. The new backend can reuse those without inventing a second IR or C
printer. Important gaps to audit include Rust-specific fallback assumptions
behind false language capabilities, such as rendering a non-ternary selection
as a Rust if expression, and bottom-tested loop continue behavior. These must
be handled explicitly for Go.

The initial feature changes explicit Go output only. Automatic language policy,
Go multi-result ABI recovery, metadata facets and runtime folds remain separate
features and are not claimed as complete by syntax support.

The reference Go 1.22.5 basics class also passes locally (eight functions,
including its prototype, multi-result, runtime-cleanup and ownership checks).
Its binary is 1,471,746 bytes, SHA256
b0c5144b05517c764da886a30b5eeca5709b6b655e93b55ae4967042943db50f.
Kuna's current C output for the same eight functions is captured separately;
this syntax feature alone will not satisfy all of those semantic expectations.

## Implemented output surface

Go now shares the existing IR, RPN printer and structured blocks. Exact-width
scalars, arrays and pointers use Go spelling, and known function-pointer
prototypes become function values. Conversions preserve pointer bridges and
float storage bits. Bottom-tested loops carry a fresh first-iteration flag so
continue reaches the original condition. Shared switch case bodies use explicit
fallthrough only when the current block graph proves a unique adjacent target;
stale chain metadata is insufficient after return duplication.

Names are escaped injectively, including keywords, predeclared names and the
helper package names math/unsafe. Source constants retain their spelling. A
readonly C string pointer retains its zero byte through unsafe.StringData.
Unknown helpers, incomplete source types and missing ABI facts remain visible.

The compiled scalar fixture covers arithmetic, control flow, pointer reads and
writes, numeric and bit conversions, readonly text, callback calls, name
collisions and observable call side effects. The callback prototype is supplied
explicitly: the current DWARF importer loses that nested prototype. The native
Go fixture separately exercises the real Go loader. Syntax parsing across whole
images is not claimed as compilation or complete Go semantic recovery.

C/Rust byte controls and timing use main e4fd472ae463e60a4643183988f6d0a2556408ee.
The proposal's older baseline remains recorded as proposal evidence only.

## Validation commands

The end-to-end stage fixture is `tests/stages/ghangr-gooutput.xml`. It selects C,
then Go, then C again over the same image. Go integration tests are registered
in kuna-console's integration suite:

```sh
cargo test --manifest-path decompiler/Cargo.toml -p kuna-console --test integration verify_outlang_go::
cargo test --manifest-path decompiler/Cargo.toml -p kuna-decomp --lib kuna_langgo::tests::
```

`tests/fixtures/go-output/check.go` supplies imports, a test driver and expected
values; emitted function bodies are inserted without editing. Both the default
structure settings and disabled taildup/crossjumprevert/gotoreduce settings run
the same 41-function checks. A separate asserted variadic signature uses a real
`_args` parameter and verifies that the synthetic tail cannot redeclare it. The native Go test builds fourteen named functions,
parses all fourteen and executes four recovered scalar functions. The unit suite
covers the reference's twenty precedence cases plus boolean XOR and complement;
redundant parentheses on left-nested operands are accepted.

Whole-image diffs and per-function classifications are adjacent to this document.
They describe the newly selectable Go presentation of the same recovered IR.
Switch fallthrough bodies were inspected separately: grep's recursive option
case and binary-files option retain their following case bodies; its error chain
and quoting switch retain their represented flow. The source fixture executes
two independent shared-case paths. Condition closures in faillog retain its
assignment and time call inside the conditional operand. The Rust image's ten
fallthrough edges retain the demangler, gimli, path and allocator case bodies;
AST-derived counts distinguish a fallthrough statement from that word in a
function name or string. The parser checks do
not establish the semantics of missing type or runtime metadata.

Timing uses 31 alternating samples after three warmups per configuration, pinned
to CPU 79. C/Rust controls compare baseline main with this implementation in the
same language; explicit Go is compared with the current original language.
Go output remains opt-in. Raw samples and exact deltas are in `timing-*.json`.

## Recorded results

The final run covers seven inputs and 11,965 distinct image/function pairs.
All 23,930 C/Rust function records match baseline in full. The six Go-output
inputs contain 4,202 functions; all parse with Go's parser, with no decompilation
errors. The compiled checks pass in both structure configurations and for the
variadic-name collision. All 9,986 workspace tests pass (39 ignored); original
parity is 675/675 and stage parity is 3,445/3,445. The original baseline is
unchanged. Spec, catalog, counters, lint and Python tooling checks pass.

The maximum measured baseline-to-current slowdown is below 2% across the four
31-sample timing cases, within the 5% budget. Exact wall/CPU deltas and samples
are retained without rounding in the adjacent JSON records.
