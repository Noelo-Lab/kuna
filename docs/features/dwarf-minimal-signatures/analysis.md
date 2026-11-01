# Minimal DWARF is not a void prototype

The pinned angr fixture `tests/x86_64/rust/fmt_arguments/fmtdemo-1.93.1`
contains Rust standard-library DWARF with names, ranges and inline call sites,
but no type or parameter records. `_print` at native address `0x48650` is one
such function. Both Kuna DWARF prototype producers interpreted those omissions
as `void _print(void)`, erasing the template and argument pointers at its callers.
The fixture is PIE: angr maps it at `0x400000`, whereas Kuna uses its native VMAs.

The importer now tests each compilation unit once for signature evidence.
Without any type reference or formal/variadic parameter record, neither
producer installs a prototype. Symbol and source-location imports remain
available. Minimal-unit names retain their namespace qualification: a bare
`write` alias for `core::fmt::write` otherwise borrows the C `write(int, void *,
size_t)` signature and loses the Rust formatting argument. A third fixture
combines a genuine libc `write` import with a namespaced four-argument Rust
`write` and checks all four computed arguments. This also conservatively declines a genuinely all-void unit with no
type references: its debug information does not distinguish it from the
minimal representation. A unit containing signature evidence retains the
existing prototype behavior.

The two small committed fixtures have identical source and instruction bodies,
compiled with `-C debuginfo=1` and `-C debuginfo=2`. The minimal caller previously
rendered `call_input(void) { add_input(); }`; it now keeps the incoming value
and `add_input(a0 * 3)`. The full-debug control retains its named `u64` parameter
and `u64` result. The function body still determines returns where debug types
are absent; this change does not invent a Rust signature or a format macro.

Whole-binary differences and final measurements are recorded beside this file.

The final comparison covers 8,235 functions: both minimal control functions,
three of thirteen functions in the imported-name control, and 378 of 455
functions in the Rust 1.93 fixture change. The full-debug control and all 7,763
functions of the older Rust `fmt` binary retain identical bodies. Both sides
complete with zero errors at a 180-second per-function budget. The initial
60-second runs timed out on the same large backtrace-cache function; its
separate longer runs and the final whole-image runs complete.

Every changed hunk is indexed in the per-fixture `*-hunks.json` files. The
Rust 1.93 results were also compared to unmodified main with `dwarf off`:
454 functions match after normalizing only namespace prefixes, the leading
function-name comment and the definition's name. The one function that exceeded
the control run's 60-second budget also matches in a separate 180-second serial
comparison. Thus these bodies recover the existing dataflow result when the
uninformative debug signatures are absent; the retained qualified debug names
do not introduce different argument or control-flow behavior. This is not a
claim that all existing Rust output is correct: unstructured branches still
carry Kuna's existing warning and diverging placeholder in Rust output.

The final 31-pair timing is 327.71 → 324.18 ms (−1.08%) for the Rust 1.93
`print_one` caller, including loading. CPU time is −1.23%. Raw samples and
superseded diagnostic measurements are retained beside this file.
