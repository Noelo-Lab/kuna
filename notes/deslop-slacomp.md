# SLEIGH compiler cleanup

Five compiler modules suppressed all dead-code warnings. A forced-warning build
identified three unused private items: an empty `macrotable` field, the
`symbol_varnode_size` helper and the `pcode_unported` panic stub. Removing these
also permits removing the blanket suppressions. The compiler builds without
warnings, and its public API and executed compilation paths are unchanged.

Module documentation now describes the completed compiler instead of unfinished
porting milestones. The end-to-end tests describe their actual fixture source:
the checkout's built `.sla` files can come from kuna, so comparing against them
alone is not an independent C++ oracle. All 44 listed source fixtures are
vendored; missing sources now fail instead of silently skipping the comparison.

The independent compiler-cleanup snapshot passed all four gates: 675 upstream assertions, 1,467 stage
assertions, 7,697 workspace tests with 38 existing ignores, and spec checks.
Catalog checks, the 41 compiler tests and public documentation generation also
passed. No regression expectations changed. The combined PR snapshot is checked
separately, with its results recorded in `notes/deslop.md`.

For an independent comparison, Ghidra's `sleigh_opt` was built from the pinned
revision `cef869af04c4740a71ad31a55704045b1b0d1644`. Both compilers compiled all
44 selected specs into temporary files. Their decompressed element streams
matched byte-for-byte for every spec. Neither the installed `.sla` files nor
the vendored sources were modified.

The independent oracle is now persistent: `tests/golden/compiler.sha256` records
those 44 C++ output digests, and `tests/compiler_parity.rs` compiles every listed
source against them. This replaces four historical test groups and their
repeated fixture list with one manifest-driven check. Each run owns a temporary
directory, avoiding the shared `/tmp/ws4b_<name>.sla` filenames.

Two concurrent test processes passed with all 149 worktree `.sla` symlinks
temporarily absent. A deliberately incorrect expected digest failed, and the
restored manifest passed. The documented C++ regeneration command reproduced
the manifest exactly. Hashing and temporary-file dependencies are test-only;
the compiler's production dependency graph is unchanged.

Compiler CLI parsing now shares filename suffix handling. Previously an omitted
extension was rejected whenever any parent directory contained a dot, including
the leading `./` in this command:

```sh
decompiler/target/release/slacomp ./decompiler/crates/kuna-slacomp/tests/golden/snips/data_le_64 /tmp/data.sla
```

The unpatched compiler reports `Unknown input file type:` followed by the input
path. Both input and output now inspect only the filename. Existing explicit
suffixes, dotfiles and unknown-extension errors are preserved. Single-file mode
also rejects extra positional arguments instead of silently ignoring them; the
usage header no longer advertises unsupported flags.

Five CLI tests cover these cases. The two omitted-extension regressions and the
extra-argument regression fail before the fix; all five pass afterward. Nine
command comparisons against the pinned C++ compiler agree on exit status and
decompressed output bytes, and previously accepted cases retain identical
compiled output. Fifty interleaved timing samples after warmup measured median
compilation times of 4.280 ms before and 4.246 ms after (-0.8%, within noise).

The independent CLI cleanup snapshot passed all four gates: 675 upstream and
1,467 stage assertions, 7,699 workspace tests with 38 existing ignores, and the
spec check. The option catalog also passed. No baseline expectations changed.

The compiler also accepted `-y` without selecting XML output. Wiring that flag
to the existing XML encoder exposed a second defect: a binary-only opcode
adapter replaced names such as `BUILD` with numbers. The encoding path now
retains `OpcodeEncoder` through the symbol table and constructor templates,
removing the adapter and its forwarding implementation. Both encoders use their
existing opcode representation. Rust callers supplying custom encoders to this
path must implement `OpcodeEncoder`; the workspace's XML and packed encoders
already do.

The XML fixture comes from the pinned C++ compiler and includes `BUILD` and
`INT_ADD`. Its CLI regression checks single-file and recursive modes; the
preserved pre-fix binary demonstrates both ignored `-y` and numeric XML opcodes.
All 44 selected specs now produce XML identical to C++ byte-for-byte, while
the same 44 default binary-output digests remain unchanged. The 357 compiler
and SLEIGH release tests passed. The encoding round-trip tests also shed an
unneeded decompression wrapper and obsolete claims that locally built specs
were independent C++ fixtures.

Fifty interleaved samples after warmup measured Toy-builder default compilation
at 8.646 ms before and 7.681 ms after (-11.2% on this fixture and host). The
complete binary output was identical. This is a local measurement, not a claim
about other specs or machines.

The independent XML cleanup snapshot passed all four gates: 675 upstream and
1,467 stage assertions, 7,700 workspace tests with 38 existing ignores, and the
spec check. The option catalog passed, and no baseline expectations changed.

The corresponding decoder also erased `OpcodeDecoder` and reconstructed the
packed protocol through a wrapper and a copied numeric conversion. It now
retains the opcode-aware interface through symbol tables, subtables and
constructor templates, using the existing XML and packed readers directly.
Custom decoder implementations on this path must implement `OpcodeDecoder`.

The pinned XML fixture initially failed even earlier: its space index was not
registered. SLA IDs had 189 repeated definitions across four modules, while XML
registration contained only a partial list. One canonical table now defines all
142 IDs and supplies registration automatically. Every name and number matches
both the previous definitions and pinned Ghidra; the old public `sla` import
paths re-export the same IDs. The XML lookup is built on demand, avoiding its
construction during ordinary binary loading.

The new symbol-table test fails on the original registry, fails on named opcodes
with only registration fixed, and passes with the complete change. All 358
compiler and SLEIGH release tests pass, including the 44 binary-output oracle
checks. All 44 XML outputs still match the pinned C++ reference byte-for-byte.
Fifty interleaved Toy-builder compilation samples measured 7.009 ms before and
7.209 ms after (+2.9% on this host); complete binary output remained identical.

Public Rust documentation builds without warnings after correcting 12 stale
links to private or renamed items; no visibility was widened.

The independent decoder/ID cleanup snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,701 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed; no baseline expectations changed.

Constructor encoding now borrows the template slice owned by `SleighBase`.
The old shared encode/decode adapter required mutable storage and forced a deep
copy solely to encode it. The callback now serves decoding only; symbol-table,
symbol and constructor encoders take `&[ConstructTpl]`. Section ordering, sparse
named-section indices and invalid-handle errors are preserved. Direct Rust
callers of those encoders pass their template slice instead of an adapter.
The nearby symbol-system documentation now describes the working compiler,
removing obsolete claims that its construction and serialization were unported.

Allocation profiling around `encode_to_sla_bytes` measured the following
allocation/reallocation requests and requested bytes. These are cumulative
requests during encoding, not peak or resident memory:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 337 → 27 | 548,493 → 474,989 |
| x86-64 | 42,922 → 39 | 19,647,757 → 9,765,725 |
| AARCH64 | 37,024 → 39 | 18,562,013 → 9,765,725 |
| Hexagon | 14,030 → 37 | 8,130,037 → 5,046,621 |

Six interleaved rounds of instrumented encoding measured median changes of
-3.8% for x86-64 and -2.7% for AARCH64. Separate, uninstrumented compiler runs
used 12 alternating before/after samples after warmup: Toy 10.419 → 7.644 ms,
x86-64 480.585 → 475.848 ms, AARCH64 566.650 → 441.893 ms, and Hexagon
229.681 → 228.010 ms. These are local measurements on a shared host, not a
general speed guarantee. Every measured binary output was byte-identical.
All 44 selected binary oracle checks and all 44 pinned C++ XML comparisons
remain unchanged.

The independent template-borrowing snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,701 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, Rust documentation had no warnings,
and no baseline expectations changed.

The compiler accepted context assignments in `with` blocks but discarded them.
For example, `with : [ mode=1; ] { :outer is op=0 { r0=r0+1; } }` compiled
without retaining the assignment. Blocks now own their parsed context changes;
each constructor receives copies from outer to inner blocks, then its local
changes. Inherited copies pass directly to the constructor, removing an
unnecessary allocate-then-take cycle through the parser's context arena.

The 20-line `with_context.slaspec` fixture covers nested blocks, siblings,
local overrides, `globalset`, leaving an inner block, and a constructor outside
the outer block. Its digest comes from the same pinned C++ compiler as the
existing oracle. The new case fails before the fix; all 45 binary oracle cases
pass afterward, with no existing digest changed. All 45 XML outputs also match
the pinned C++ compiler byte-for-byte. The 358 compiler and SLEIGH release tests
pass.

Fifty alternating Toy-builder compilation samples measured 7.200 → 7.215 ms
(+0.21%); 12 x86-64 samples measured 473.641 → 473.309 ms (-0.07%). Both used
warmups, and all before/after outputs for these existing specs were identical.

The independent context-inheritance snapshot passed all four gates: 675
upstream and 1,467 stage assertions, 7,701 workspace tests with 38 existing
ignores, and the spec check. The option catalog passed. No baseline or existing
oracle expectation changed.

Consistency diagnostics ignored the supplied constructor identity and used the
parser's final location. An uninitialized temporary at `fault.sinc:3`, for
example, was reported as `main.slaspec:11`. Diagnostics now resolve the existing
constructor source-file index and line number. Section finalization uses the
same lookup, and the separate constructor-location map has been removed.

Two CLI regressions put the faulty constructor in an included subtable after
an existing root constructor. Both fail before the change and match the pinned
C++ compiler's complete diagnostics afterward. They also check successful
warning output and failed compilation without an output file. Independent
comparisons preserve section-error and delay-slot behavior, exit statuses and
compiled bytes. The existing abbreviated delay-slot warning remains unchanged.
All 360 compiler and SLEIGH release tests pass, including 45 binary oracles.

An initial unpinned Toy timing batch varied enough to suggest a 5.3% slowdown.
A controlled repeat pinned both programs to CPU 40, balanced their order and
discarded warmup pairs. Across 110 samples per version, median Toy wall time
was 6.8415 → 6.8248 ms (-0.25%) and child CPU time was 6.4245 → 6.4100 ms
(-0.23%). Twelve interleaved x86-64 samples measured 473.342 → 472.030 ms
(-0.28%). The before/after compiler-output checks were identical; these are local
measurements on a shared host.

The independent constructor-location snapshot passed all four gates: 675
upstream and 1,467 stage assertions, 7,703 workspace tests with 38 existing
ignores, and the spec check. The option catalog passed, Rust documentation had
no warnings, and all 45 XML outputs still match the pinned C++ reference.
No baseline or existing oracle expectation changed.

The public constructor builder still had a partial implementation that rejected
semantic sections as “not yet ported” and ignored context changes, even though
the parser's other entry point supported both. The complete implementation now
owns its section vector directly. The parser entry point takes that vector from
its arena slot and delegates; finalization borrows the local vector instead of
looking it up repeatedly. Both public signatures are preserved.

Three API regressions cover main and sparse named sections, inherited and local
context without semantic sections, and validation errors with scope cleanup.
All three fail on the former public path and pass through both entry points
after the change. The 363 compiler and SLEIGH release tests pass, including the
45 independent binary oracle cases.

Alternating before/after compiler runs pinned to CPU 40 measured Toy-builder
wall time at 6.9436 → 6.9332 ms (-0.15%, 110 samples per version), x86-64 at
483.2231 → 483.7396 ms (+0.11%, 12 samples), and Hexagon at 154.0491 →
153.1624 ms (-0.58%, 12 samples). Child CPU times changed by +0.15%, +0.12%
and -0.58%, respectively. Every measured output was byte-identical. These
local measurements followed warmups and include process startup.

The independent constructor-builder snapshot passed all four gates: 675
upstream and 1,467 stage assertions, 7,706 workspace tests with 38 existing
ignores, and the spec check. The option catalog passed, and all 45 XML outputs
still match the pinned C++ reference. No baseline or existing oracle expectation
changed.

The compiler copied each macro definition into its symbol and copied the whole
definition again for every invocation. Symbols and the expansion table now share
one immutable definition. The builder still creates independent output operations
for each invocation's parameter and label substitutions. The existing public
owned setter and borrowed getter are preserved. The expansion table no longer
uses optional slots, since completed definitions are only appended and never
removed. Contradictory arena documentation and unsupported claims of complete
macro test coverage were corrected alongside this change.

A temporary allocator counter measured full compilation, including allocation
and reallocation requests, with identical compiled output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 26,410 → 26,216 | 2,874,354 → 2,842,066 |
| x86-64 | 3,737,622 → 3,725,907 | 317,842,112 → 315,315,352 |
| Hexagon | 883,974 → 883,423 | 83,298,700 → 83,176,868 |

These are cumulative requested bytes, not peak or resident memory. Separate
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at 6.2641 →
6.2496 ms (-0.23%, 110 samples per version), x86-64 at 497.3401 → 501.0246 ms
(+0.74%, 12 samples), and Hexagon at 150.0395 → 151.0557 ms (+0.68%, 12 samples).
Child CPU time changed by -0.20%, +0.74%, and +0.67%. Runs alternated order after
warmup, and every measured output was identical. These are local shared-host
measurements.

The independent macro-sharing snapshot passed all four gates: 675 upstream and
1,467 stage assertions, 7,706 workspace tests with 38 existing ignores, and the
spec check. All 363 compiler and SLEIGH release tests pass, including 45 binary
oracle cases, and all 45 XML outputs match the pinned C++ reference. The option
catalog passed and Rust documentation had no warnings. No baseline or existing
oracle expectation changed.

The consistency checker silently skipped a temporary whose only read preceded
its only write, based on a false claim that size validation had already caught
it. For example, the semantic body `local tmp:4; r0=tmp; tmp=1;` compiled
successfully, whereas pinned Ghidra exits with status 2 and reports
`Unrecoverable error: Read of temporary before write`. The same gap affected
`local tmp:4; tmp=tmp+1;`, which reads and writes in one operation.

The checker now returns that error through optimization and the compiler pipeline
before encoding. Its obsolete forwarding wrapper was removed. The CLI prints the
error explanation rather than the Rust debug representation. A parameterized CLI
regression covers both invalid orderings and a valid write-before-read control;
it fails before the change. Both rejected inputs now match Ghidra's complete
stderr, exit status, and absence of an output file. All 364 compiler and SLEIGH
release tests pass, including the 45 existing binary oracle cases.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
6.7037 → 6.6910 ms (-0.19%, 110 samples per version), x86-64 at 473.5369 →
476.8359 ms (+0.70%, 12 samples), and Hexagon at 150.1469 → 148.6977 ms
(-0.97%, 12 samples). Child CPU times changed by -0.34%, +0.66%, and -0.98%.
Every measured output was identical. These local measurements followed warmups
and include process startup.

The independent temporary-order snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,707 workspace tests with 38 existing ignores,
and the spec check. The option catalog passed and all 45 XML outputs still
match the pinned C++ reference. No baseline or existing oracle expectation
changed.

The consistency checker now borrows varnodes and their size and offset fields
instead of cloning them for inspection. One borrowed-varnode helper replaces
five overlapping read helpers. Rule selection and unused-temporary reporting
iterate the ordered map's existing values, removing temporary key vectors and
subsequent lookups. Rule selection takes an immutable state reference and copies
only the selected record. Applying a rule still makes the owned varnode copy
needed to rewrite an operation. Check order, diagnostics and optimization rules
are unchanged.

A temporary full-compilation allocator counter produced identical output and
measured the following cumulative allocation/reallocation requests:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 26,216 → 26,200 | 2,842,066 → 2,841,554 |
| x86-64 | 3,725,907 → 3,720,678 | 315,315,352 → 314,978,144 |
| Hexagon | 883,423 → 881,560 | 83,176,868 → 83,100,348 |

These requested-byte totals are not peak or resident memory. All 364 compiler
and SLEIGH release tests pass, including the existing negative diagnostic tests
and all 45 independent binary oracle cases.

Uninstrumented compiler runs pinned to CPU 40 measured Toy-builder wall time at
6.7628 → 6.7658 ms (+0.04%, 110 samples per version), x86-64 at 478.4616 →
477.6233 ms (-0.18%, 12 samples), and Hexagon at 149.5080 → 150.1109 ms
(+0.40%, 12 samples). Child CPU times changed by +0.03%, -0.17%, and +0.38%.
Runs alternated order after warmup, and every output was identical. These are
local measurements on a shared host.

The independent checker-borrowing snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,707 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 45 XML outputs still match
the pinned C++ reference. No baseline or existing oracle expectation changed.

The compiler accepted `-c` but called an empty local-collision check, whose
comment incorrectly claimed exported values were unsupported. Reusing a subtable's
exported temporary through two constructor operands therefore produced no warning.
The existing check now follows exported storage through subtable alternatives and
re-exports, including dynamic-export temporaries. It visits each symbol once per
operand and stops after the first collision in each constructor. Default output
reports the count and a `-c` hint; `-c` adds the operand names and source location.
The detector lives in a private compiler module and only borrows the symbol and
template data. Existing compiled images are unchanged.

Two parameterized CLI regressions cover direct and dynamic collisions, build-only
constructors, distinct temporaries, register exports and constant exports, each
with and without `-c`. The warning regression fails before the fix. All 12 case/flag
combinations match the pinned C++ compiler's complete diagnostics, exit status and
decoded image; the Rust images are byte-identical before and after the change.
All 366 compiler and SLEIGH release tests pass, including the 45 existing binary
oracle cases.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
6.2421 → 6.2882 ms (+0.74%, 110 samples per version), x86-64 at 481.6968 →
480.3967 ms (-0.27%, 12 samples), and Hexagon at 150.7768 → 153.3527 ms
(+1.71%, 12 samples). Child CPU times changed by +0.80%, -0.27%, and +1.73%.
Every measured output was identical. These local measurements followed warmups
and include process startup.

The restored pass also received 12 alternating timing samples per version on
AArch64 and ARM8, pinned to the same CPU after warmup. AArch64 wall time was
436.5198 → 439.6371 ms (+0.71%) and ARM8 was 317.3761 → 312.1483 ms (-1.65%);
child CPU changes were +0.74% and -1.74%. All outputs were identical.

The independent collision-warning snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,709 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 45 XML outputs still match
the pinned C++ reference. No baseline or existing oracle expectation changed.

The compiler driver now stores canonical `ConstructorRef` values for its parser
handles instead of splitting and reconstructing table/index pairs. Private
helpers use the same reference through operand creation, finalization and
diagnostics. Section validation borrows the constructor's operand list, and
inherited pattern composition iterates the existing `with` stack without an
intermediate vector. Address-space helpers share the existing `PcodeCompile`
override-or-base policy. Public handle types, space setters, lookup fallbacks
and diagnostic text are unchanged. Comments now correctly locate temporary
allocation in `SleighBase`; the unused public compatibility field remains.

All 366 compiler and SLEIGH release tests pass, including the existing constructor
API, context and diagnostic regressions and 45 binary oracle cases. All 45 XML
outputs also match the pinned C++ reference; no new implementation-shaped tests
were added for this internal refactor.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
4.7249 → 4.7511 ms (+0.55%, 110 samples per version), x86-64 at 475.9100 →
478.7274 ms (+0.59%, 12 samples), and Hexagon at 150.2054 → 150.6355 ms
(+0.29%, 12 samples). Child CPU changes were +0.18%, +0.69%, and +0.29%.
Every output was identical. This repeated an initial run where both Toy versions
shifted between roughly 6.8 ms and 4.3 ms during sampling, making the ratio of
separate medians show +8.68% despite a median paired change of -0.23%. Both raw
runs are retained; these are local shared-host measurements after warmup.

The independent driver-reference snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,709 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed. No baseline or existing oracle
expectation changed.

Compiler finalization now borrows its equation arena and existing table list.
Crossbuild allocation updates referenced templates in place, removing the copied
subtable list, accumulated handle list, per-constructor named-section vectors
and template take/restore operations. A private per-table helper preserves root,
subtable, constructor and main-before-named section order, including repeated
references and missing-section skips. Templates without constructor references
remain untouched. The empty private cross-reference pass and its redundant error
check were removed; the decoder still builds runtime register cross-references.

The register-name check iterates the global scope directly and borrows original
spellings when building uppercase keys. Eight temporary case/flag comparisons
cover duplicate spellings, distinct names, similarly named user operations and
local temporaries, with and without `-s`. Complete Rust diagnostics, status and
images are unchanged. Status and decoded images match pinned C++ in all eight
cases; C++'s duplicate-name diagnostic already includes the earlier definition's
location while Rust omits that suffix. This refactor preserves that difference.

A full-compilation allocator counter measured these cumulative requests on each
of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 26,175 → 26,118 | 2,843,530 → 2,842,767 |
| x86-64 | 3,729,190 → 3,727,747 | 316,420,392 → 316,393,776 |
| Hexagon | 883,328 → 882,323 | 83,367,400 → 83,270,440 |

These are requested-byte totals, not peak or resident memory. Existing compiler
oracles cover the Hexagon crossbuild and named-section path; no expectations or
public APIs changed.

Alternating uninstrumented runs pinned to CPU 40 measured Toy-builder wall time
at 6.8399 → 6.8092 ms (-0.45%, 110 samples per version), x86-64 at 482.4734 →
477.5635 ms (-1.02%, 12 samples), and Hexagon at 151.3198 → 151.3736 ms
(+0.04%, 12 samples). Child CPU changes were -0.47%, -1.01%, and +0.02%.
Every output was identical. Measurements followed warmups on the shared host.

The independent final-pass snapshot passed all four gates: 675 upstream and
1,467 stage assertions, 7,709 workspace tests with 38 existing ignores, and the
spec check. The option catalog passed; all 45 binary and 45 XML oracle outputs
are unchanged. No baseline or existing expectation changed.

The compiler rejected a definition such as `define bitrange flag=r0[3,1];`
with a claim that `BitrangeSymbol` was unported, although the shared symbol type,
scanner classification, read/write builders and purge support were already
implemented. The driver now registers that existing symbol. Byte-aligned aliases,
range validation and public APIs retain their behavior; no runtime format or
parallel implementation was added.

Two eleven-line fixtures exercise reads and writes of one-bit, nine-bit and
byte-aligned aliases in little and big endian specifications. Their independent
C++ hashes extend the existing compiler oracle from 45 to 47 cases without a new
test runner. The oracle test fails before the fix at the stale rejection and
passes afterward; all 366 compiler/SLEIGH release tests pass. All eight CLI
comparisons (the two fixtures and six smaller probes) match the pinned C++
compiler's complete diagnostics, status and decoded images. Previously accepted
byte-aligned probes retain byte-identical Rust images.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
5.2539 → 5.2205 ms (-0.64%, 110 samples per version), x86-64 at 566.6605 →
557.9362 ms (-1.54%, 12 samples), and Hexagon at 157.8122 → 154.3188 ms
(-2.21%, 12 samples). Child CPU changes were -0.82%, +0.69%, and -2.20%.
Every measured output was identical. Measurements followed warmups on the shared
host and include scheduling variation.

The independent bitrange snapshot passed all four gates: 675 upstream and 1,467
stage assertions, 7,709 workspace tests with 38 existing ignores, and the spec
check. The option catalog passed. All 47 binary and 47 XML oracle cases match
pinned C++, including the two new fixtures; the original 45 expectations and
both parity baselines are unchanged.

The three attachment handlers now share duplicate reporting and their symbol
replacement loop. Each public entry point still selects its own table type and
diagnostic labels; variable-width validation remains between duplicate warnings
and per-symbol replacement. The duplicate-selection algorithm is unchanged:
`[a, b, b, a]` still reports `b`. The refactor removes 26 production lines and
keeps table ownership and public APIs unchanged.

All 366 compiler/SLEIGH release tests pass, including 47 binary oracle cases.
Seven temporary before/after CLI comparisons cover duplicate entries for each
attachment kind, table-size errors, and mixed register widths. Complete Rust
diagnostics, exit status and output bytes are unchanged. The first six cases
also match pinned C++ exactly. For mixed widths, C++ already repeats the error
for each attached symbol while Rust reports it once per directive; this refactor
preserves that existing difference.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
6.7384 → 6.8142 ms (+1.13%, 110 samples per version), x86-64 at 484.1825 →
478.9084 ms (-1.09%, 12 samples), and Hexagon at 155.1930 → 154.8527 ms
(-0.22%, 12 samples). Child CPU changes were +0.94%, -1.08%, and -0.20%.
Every output was identical. Measurements followed warmups on the shared host.

The independent attachment snapshot passed all four gates: 675 upstream and
1,467 stage assertions, 7,709 workspace tests with 38 existing ignores, and the
spec check. The option catalog passed, and all 47 XML outputs still match
pinned C++. No baseline or existing oracle expectation changed.

Symbol compaction updated numeric ids but left each scope's name lookup map
pointing at old ids. With a transient section before user operations `first`
and `second`, cleanup put them in slots 0 and 1, yet looking up `first` returned
`second` and looking up `second` returned nothing. Compaction now remaps scope
bindings along with parent scopes and symbol references. Two public-API tests
cover name/id lookup agreement, scope iteration, repeated cleanup, and removal
of macro/subtable operands and empty scopes. The lookup regression fails before
the fix; the ownership-removal control already passes.

Cleanup now takes ownership of removed symbols and uses their existing names
and operand lists. This removes name copies for retained symbols and the
separate lists of child ids, scopes and copied names used for deletion. The
retention rules and public APIs are unchanged, and the misleading description
of which global symbols survive was corrected. The production path shrank by
38 lines. All 368 compiler/SLEIGH release tests pass, including the 47 binary
oracle cases; the standalone public-API reproduction now resolves both names
and scope ids correctly.

A full-compilation allocator counter measured these cumulative requests on each
of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 26,118 → 25,854 | 2,842,767 → 2,841,316 |
| x86-64 | 3,727,747 → 3,705,594 | 316,393,776 → 316,197,430 |
| Hexagon | 882,323 → 871,393 | 83,270,440 → 83,189,763 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
4.7168 → 4.7837 ms (+1.42%, 110 samples per version), x86-64 at 476.0627 →
478.9264 ms (+0.60%, 12 samples), and Hexagon at 150.9603 → 151.6380 ms
(+0.45%, 12 samples). Child CPU changes were +1.04%, +0.61%, and +0.46%.
Every output was identical. Measurements followed warmups on the shared host.

The independent symbol-compaction snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,711 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 47 XML outputs still match
pinned C++. No baseline or existing oracle expectation changed.

Symbol insertion now moves its owned name into the scope map instead of copying
it again. Duplicate diagnostics read the name from the stored symbol, and
global insertion keeps its validated scope borrow across the independent symbol
append. Replacement updates the name binding directly instead of removing and
reinserting a map node. Public signatures and insertion/error ordering stay the
same. The production path shrank by eight lines.

The existing duplicate-name test now checks that rejected insertions keep their
slots without changing the original binding, and that explicitly replacing a
rejected slot rebinds its name. Those assertions pass before and after. A
standalone public-API probe also preserves complete results, errors and table
state for missing and invalid scopes, nested insertion, replacement, and an
undefined symbol id. All 368 compiler/SLEIGH release tests pass, including the
47 binary oracle cases.

A full-compilation allocator counter measured these cumulative requests on each
of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 25,854 → 25,593 | 2,841,316 → 2,840,249 |
| x86-64 | 3,705,594 → 3,683,498 | 316,197,430 → 316,008,316 |
| Hexagon | 871,393 → 860,478 | 83,189,763 → 83,111,390 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
6.1958 → 6.1448 ms (-0.82%, 110 samples per version), x86-64 at 487.6291 →
488.3288 ms (+0.14%, 12 samples), and Hexagon at 182.8576 → 187.1021 ms
(+2.32%, 12 samples). Child CPU changes were -0.63%, +0.14%, and +2.30%.
Every output was identical. Measurements followed warmups on the shared host.

The independent symbol-insertion snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,711 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 47 XML outputs still match
pinned C++. No baseline or existing oracle expectation changed.

Pattern construction now borrows completed constructor patterns for common
subpattern folding and decision-tree setup, and borrows context changes while
validating them. Decision nodes retain ownership of their simplified patterns
by moving the simplifier's result directly into the node. This removes a
private clone-only extractor and 14 production lines. Pattern ownership,
missing-pattern handling, public APIs and diagnostics are unchanged. The
pattern-building tests also lose an unused type-name placeholder and a stale
header that described the parser as a stub. No fixture expectation changed.

All 368 compiler/SLEIGH release tests passed, including the 47 binary oracles.
Twenty CLI probes preserve complete Rust diagnostics, statuses and output
images: all six declaration orders of two overlapping patterns and their
intersection, an unresolved overlap, identical patterns, a same-constructor
disjunction and nested specialization, each with default settings and `-l`.
Accepted images match pinned C++; three failing cases retain the preexisting
generic Rust errors instead of C++'s paired source-location diagnostics.

A full-compilation allocator counter measured these cumulative requests on each
of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 25,593 → 22,760 | 2,840,249 → 2,827,889 |
| x86-64 | 3,683,498 → 3,443,118 | 316,008,316 → 314,678,216 |
| Hexagon | 860,478 → 778,303 | 83,111,390 → 82,743,710 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
6.2010 → 6.1455 ms (-0.90%, 110 samples per version), x86-64 at 481.6017 →
479.2046 ms (-0.50%, 12 samples), and Hexagon at 150.9999 → 149.8060 ms
(-0.79%, 12 samples). Child CPU changes were -1.06%, -0.50%, and -0.80%.
Every output was identical. Measurements followed warmups on the shared host.

The independent pattern-ownership snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,711 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 47 XML outputs still match
pinned C++. No baseline or existing oracle expectation changed.

Pattern-building errors were collected and then discarded, leaving invalid
specifications with only `No output produced`. Decision-tree errors discarded
the constructor identities and printed one generic message. They also shared
local constructor ids across tables, so matching id pairs in separate tables
collapsed into one report. Unreferenced-table warnings omitted the table name.

The compiler now reports accumulated pattern-building errors with the affected
subtable's source location and prints the name in unused-table warnings. Each
decision tree collects errors independently, and the driver qualifies the pairs
with `ConstructorRef` before reporting both constructors' source locations.
Identical patterns remain errors in either mode; ordinary conflicts are still
accepted by default and reported with `-l`. The runtime property API and its
pair-deduplication rules are unchanged. The built-in instruction table has no
user declaration, so its build errors use the collected constructor details
without assigning the source header as its location.

Three CLI tests cover seven cases: undefined operands in the root and an
included subtable, a named unused-table warning, and conflicts in two included
tables whose local constructor ids repeat (overlap/identical patterns, each
with default settings and `-l`). All three tests fail before the fix and pass
when the same test driver invokes pinned C++. After the fix, all 371 compiler
and SLEIGH release tests pass, including the 47 binary oracle cases.

A separate 27-case comparison includes those cases plus the previous pattern
ordering probes. All final diagnostics, statuses and decoded images match
pinned C++. Before/after Rust exit statuses and all accepted raw images remain
identical. The three generic-message differences recorded above are now fixed.

Alternating compiler runs pinned to CPU 40 measured Toy-builder wall time at
6.6211 → 6.6270 ms (+0.09%, 110 samples per version), x86-64 at 475.7773 →
467.1450 ms (-1.81%, 12 samples), and Hexagon at 148.5649 → 148.5627 ms
(-0.0015%, 12 samples). Child CPU changes were +0.17%, -1.79%, and +0.03%.
Every output was identical. Measurements followed warmups on the shared host.

A further 256 deterministic generated cases (128 small specs, each in default
and strict mode) preserve before/after Rust status, stdout and raw images. They
also expose existing differences from C++'s suppression of later errors for an
already-marked constructor: three default-mode specifications are accepted by
C++ but rejected by Rust, and 96 diagnostic transcripts differ. Re-running the
pre-fix compiler confirms these acceptance differences predate this change.
The diagnostic fix leaves the runtime's existing pair-deduplication rules intact.

The independent pattern-diagnostics snapshot passed all four gates: 675 upstream
and 1,467 stage assertions, 7,714 workspace tests with 38 existing ignores, and
the spec check. The option catalog passed, and all 47 XML outputs still match
pinned C++. No baseline or existing oracle expectation changed.

Decision-tree construction now sorts terminal pattern indices, checks conflicts
against the original patterns, then moves the owned patterns into their final
order. This removes two copies of every terminal pattern list, the search to
recover an original index by value, and an empty pointer-comparison branch.
Insertion order and conflict resolution by pattern value and constructor id
are preserved. Branch expansion also consumes compatible values from an
iterator instead of building a temporary list. The production path shrank by
41 lines; public APIs and diagnostic-pair deduplication are unchanged.

The 11-line `terminal_patterns.slaspec` fixture covers an overlap resolved by
its intersection, a disjunction, and nested specializations. Its digest was
generated with pinned C++ at the repository path before changing the ordering
code; the independent manifest now covers 48 specs, with all previous entries
unchanged. Default, strict and XML output agree with pinned C++. The existing
compiler/SLEIGH release suite passes all 371 tests.

A separate comparison extracts the final ordering function and the original
implementation into a standalone driver. Across 50,000 deterministic lists of
instruction, context and combined patterns, including repeated constructor
ids, offsets and constant patterns, the exact pattern order and both diagnostic
pair lists agree. Another 286 CLI cases preserve complete before/after status,
stdout, stderr, and raw/decoded images. The broader C++ differences documented
above remain unchanged.

A full-compilation allocator counter measured these cumulative requests on each
of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 22,760 → 21,461 | 2,827,889 → 2,804,625 |
| x86-64 | 3,443,134 → 3,332,858 | 314,678,714 → 312,500,330 |
| Hexagon | 778,303 → 762,392 | 82,743,710 → 82,475,694 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
6.6912 → 6.6356 ms (-0.83%, 110 samples per version), x86-64 at 476.2450 →
475.8762 ms (-0.08%, 12 samples), and Hexagon at 151.6542 → 152.5819 ms
(+0.61%, 12 samples). Child CPU changes were -1.02%, -0.09%, and +0.63%.
Every output was identical. Measurements followed warmups on the shared host.

The decision-tree snapshot passed all four gates: 675 upstream and 1,467 stage
assertions, 7,714 workspace tests with 38 existing ignores, and the spec check.
The option catalog passed, and all 48 XML outputs match pinned C++. No baseline
or existing oracle expectation changed.

Decision-field scoring now reuses one bounded counter array per search instead
of allocating a vector for every candidate. A named eight-bit maximum bounds
both the array and candidate widths; each score clears only its active bins.
Fixed-pattern counts, entropy arithmetic, candidate order and tie-breaking
remain unchanged. This changes no public API or compiler decision rule.

A standalone comparison extracts the old and new production scoring and field
selection functions. All 800,000 scores agree bit for bit across instruction,
context and combined patterns, varying field widths and offsets; all 5,000
selected fields also agree. The existing 371 compiler/SLEIGH release tests,
48 binary and XML oracles, and 286 CLI comparisons pass without changes to
expected output, status or diagnostics.

The full-compilation allocation counter measured these cumulative requests on
each of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 21,461 → 17,009 | 2,804,625 → 2,332,137 |
| x86-64 | 3,332,858 → 2,727,789 | 312,500,330 → 239,689,410 |
| Hexagon | 762,392 → 699,760 | 82,475,694 → 76,800,014 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
7.5387 → 7.3719 ms (-2.21%, 110 samples per version), x86-64 at 473.8591 →
448.6788 ms (-5.31%, 12 samples), and Hexagon at 148.5946 → 147.4061 ms
(-0.80%, 12 samples). Child CPU changes were -2.41%, -5.16%, and -0.82%.
Every output was identical. Measurements followed warmups on the shared host.

The counter-reuse snapshot passed all four gates: 675 upstream and 1,467 stage
assertions, 7,714 workspace tests with 38 existing ignores, and the spec check.
The option catalog passed. No baseline, test count or oracle expectation changed.

Constructor-id resolution and matched-pattern capture now share one decision
walk. Subtable lookup and non-subtable index validation also use a single path
for both public APIs. Id-only calls borrow the matched leaf without cloning its
pattern. Public signatures, constructor selection, failure messages and pattern
capture semantics are unchanged. The production source shrank by 61 lines,
including obsolete claims that parser walkers did not exist and matched-leaf
resolution was unused during decoding.

The existing compiler/SLEIGH release suite passes all 371 tests, including
48 independent compiler oracles, instruction-mask checks and 16 C++ lift
fixtures covering 1,171 instructions. No new test scaffolding was needed.
Alternating runs of the complete lift-fixture suite pinned to CPU 40 measured
424.2964 → 425.0527 ms (+0.18%, 30 samples per version after three warmup pairs).
Child CPU changed by +0.14%. All 66 runs reproduced the expected lift output.
This measures fixture loading and lifting together on the shared host.

The resolution snapshot passed all four gates: 675 upstream and 1,467 stage
assertions, 7,714 workspace tests with 38 existing ignores, and the spec check.
The option catalog passed. No baseline, test count or oracle expectation changed.

Aligned instruction-pattern algebra now borrows its normalized blocks directly
instead of copying a block and shifting it by zero. Nonzero alignment keeps its
existing owned shift. Decision construction also uses the default node directly
and iterates context then instruction fields explicitly, removing two empty
constructor distinctions and toggle-and-break loops. Production source shrank
by 16 lines. Public APIs and pattern, field-selection and diagnostic behavior
are unchanged.

The production modules match the separately compared candidates: 280,000
algebra results agree exactly across instruction, context, combined and OR
patterns, constants, multiword blocks and positive/negative/zero shifts;
50,000 chosen fields also agree. All 371 existing compiler/SLEIGH release tests,
48 binary/XML oracles and 286 CLI cases pass without expectation changes.
The current pre-change CLI was checked again before the comparison.

The full-compilation allocation counter measured these cumulative requests on
each of three repetitions, with identical output before and after:

| Spec | Requests before → after | Bytes requested before → after |
| --- | ---: | ---: |
| Toy builder | 17,009 → 16,835 | 2,332,137 → 2,331,441 |
| x86-64 | 2,727,789 → 2,724,101 | 239,689,410 → 239,674,658 |
| Hexagon | 699,760 → 689,014 | 76,800,014 → 76,757,030 |

These are requested-byte totals, not peak or resident memory. Alternating
uninstrumented runs pinned to CPU 40 measured Toy-builder wall time at
6.3577 → 6.3447 ms (-0.20%, 110 samples per version), x86-64 at 434.9820 →
436.4796 ms (+0.34%, 12 samples), and Hexagon at 144.7279 → 142.9611 ms
(-1.22%, 12 samples). Child CPU changes were -0.18%, +0.33%, and -1.22%.
Every output was identical. Measurements followed warmups on the shared host.

The pattern-construction snapshot passed all four gates: 675 upstream and 1,467
stage assertions, 7,714 workspace tests with 38 existing ignores, and the spec
check. The option catalog passed. No baseline, test count or oracle expectation
changed.

Pattern masks and values now share one word extractor. It retains the signed
extension to unsigned word indices, zero fill for missing words and masked
shift counts. Disjoint-pattern specialization, identity and intersection
resolution use ordered instruction/context iteration, preserving early exits
and the rules for absent or unconstrained blocks. This removes 120 production
lines while retaining every public signature.

A comparison of the actual old and new modules checks 1,844,850 mask/value reads
and boundary outcomes, 280,000 algebra results and 60,000 pattern predicates.
All agree with overflow checks both enabled and disabled, including negative
positions, cross-word fields, unusual widths and integer-boundary panics. The
production source matches the compared candidate. All 371 existing release
tests, 48 binary/XML compiler oracles and 286 CLI cases pass unchanged.

Full-compilation allocation requests and requested bytes are identical in
three repetitions on Toy builder, x86-64 and Hexagon, as are the output images.
Alternating uninstrumented runs pinned to CPU 40 measured Toy-builder wall time
at 6.2820 → 6.2337 ms (-0.77%, 110 samples per version), x86-64 at 435.3269 →
432.6248 ms (-0.62%, 12 samples), and Hexagon at 143.9592 → 144.8409 ms
(+0.61%, 12 samples). Child CPU changes were -0.65%, -0.62%, and +0.62%.
Every output was identical. Measurements followed warmups on the shared host.

The extraction/comparison snapshot passed all four gates: 675 upstream and 1,467
stage assertions, 7,714 workspace tests with 38 existing ignores, and the spec
check. The option catalog passed. No baseline, test count or oracle expectation
changed.

OR-pattern queries now use short-circuit iterator predicates, and simplification
reuses the same conservative always-true query. The instruction-unconstrained
query still requires every alternative to qualify. Pattern-block comparisons
use the maximum extent and cap positive remaining spans at one word. This
removes 38 production lines without changing public signatures or truth rules.

The actual old and new modules agree on 40,000 OR-query and simplification
cases, including empty and singleton alternatives, in addition to 1,844,850
mask/value reads and boundary outcomes, 280,000 algebra results and 60,000
pattern predicates. Both overflow-check settings pass. All 371 targeted release
tests, 48 binary/XML compiler oracles and 286 CLI cases remain unchanged.
Allocation requests, requested bytes and output images match on Toy builder,
x86-64 and Hexagon in three repetitions.

Alternating uninstrumented runs pinned to CPU 40 measured x86-64 at
434.4786 → 440.4147 ms (+1.37%, 12 samples per version) and Hexagon at
143.9275 → 144.2885 ms (+0.25%, 12 samples). Toy's first 110-sample run had
separate medians of 5.6761 → 6.3005 ms (+11.00%); a 220-sample repeat reversed
them to 6.2230 → 5.8031 ms (-6.75%). The median of each adjacent after/before
pair was +0.95% and +0.93%, respectively. Pooling all 330 samples gives
6.2230 → 6.2540 ms (+0.50%), with a paired median of +0.94%. Pooled child CPU
medians change +0.42% (paired +0.92%). Both runs are retained: the short Toy
measurement is sensitive to host variation, while paired results agree within
the 5% budget. All measured images are identical.

The query/bounds snapshot passed all four gates: 675 upstream and 1,467 stage
assertions, 7,714 workspace tests with 38 existing ignores, and the spec check.
The option catalog passed. No baseline, test count or oracle expectation changed.

Token alignment now compares paired prefix or suffix slices in the required
direction, shares unmatched-prefix accumulation and selects the resulting
token list once. Common subpatterns copy their shared prefix or suffix once,
replacing repeated front insertion for suffixes. Four redundant clears of new,
empty vectors are removed. This removes 16 production lines; public signatures,
ellipsis flags, error priority, first mismatch and input ownership are preserved.
An existing unused test import is also removed.

Actual-module comparisons agree on 72,000 alignment/algebra outcomes and their
input/result states, plus 100,000 integer-boundary alignment outcomes and partial
result states. Both overflow-check settings pass. Cases include every pair of
five pattern forms, all ellipsis combinations, empty token lists, differing
sizes/endianness with the same index, and overflow during reverse accumulation.
The applied source matches the compared candidate. All 371 targeted release
tests pass without warnings; 286 CLI cases and 48 binary/XML compiler oracles
remain identical.

Full-compilation allocation request counts remain 16,835 for Toy builder,
2,724,101 for x86-64 and 689,014 for Hexagon in three repetitions. Requested
bytes fall by 288, 28,176 and 6,372 respectively, to 2,331,153, 239,646,482 and
76,750,658. These totals do not measure peak or resident memory.

Alternating uninstrumented runs pinned to CPU 40 measured Toy-builder wall time
at 5.7332 → 5.2800 ms (-7.90%, 110 samples per version), x86-64 at 439.3014 →
438.2380 ms (-0.24%, 12 samples), and Hexagon at 145.3572 → 145.1342 ms
(-0.15%, 12 samples). Paired wall-time medians were +0.11%, -0.40% and -0.23%;
paired child CPU medians were -0.19%, -0.40% and -0.24%. The short Toy run's
separate medians vary with the shared host, so it does not establish a speedup.
All results fit the 5% budget and every measured image is identical.

The token-alignment snapshot passed all four gates: 675 upstream and 1,467 stage
assertions, 7,714 workspace tests with 38 existing ignores, and the spec check.
The option catalog passed. No baseline, test count or oracle expectation changed.
