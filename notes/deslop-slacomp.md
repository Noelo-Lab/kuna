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

Runtime expression evaluation and compiler leaf substitution now share one
arithmetic traversal. A private generic leaf reader keeps the two public entry
points and expression variants intact, visits left before right and preserves
wrapping operations, masked shifts, division behavior and early failures. This
removes 52 production lines and the duplicated arithmetic implementation.

Actual-module comparisons agree on 357,744 outcomes across 12,336 expression
trees with overflow checks both enabled and disabled. They cover every binary
operator with integer-boundary values, unary boundaries and mixed trees with
all seven leaf kinds. Runtime reads, injected errors/panics and substitution
cursor progress also match. The applied source is identical to the compared
candidate. All 371 targeted release tests, 286 CLI comparisons and 48 binary/XML
compiler oracles pass unchanged. Full-compilation allocation request counts and
requested-byte totals remain identical across three repetitions per spec.

Alternating uninstrumented compilation runs pinned to CPU 40 measured Toy
builder at 5.7321 → 5.7030 ms (-0.51%, 110 samples per version), x86-64 at
445.9354 → 442.2835 ms (-0.82%, 12 samples), and Hexagon at 148.4401 →
149.3912 ms (+0.64%, 12 samples). Paired wall-time medians changed -0.28%,
-0.83% and +0.53%; paired child CPU medians changed -0.43%, -0.83% and +0.56%.
Every output image was identical.

The runtime benchmark passed all 16 independent lift fixtures in each of 66
runs, covering 1,171 instructions. With 30 measured samples per version on CPU
40, wall time was 427.9381 → 424.7398 ms (-0.75%; paired median -0.19%). Child
CPU changed -0.75% (paired -0.17%). Both compiler and runtime results fit the
5% budget; measurements followed warmups on the shared host.

The shared-evaluator snapshot passed all four gates: 675 upstream and 1,467
stage assertions, 7,714 workspace tests with 38 existing ignores, and the spec
check. The option catalog passed. No baseline, test count or oracle expectation
changed.

Expression table-id and operand-index remapping now share a private, left-to-right
operand visitor. Table callbacks still run separately for each reference, and
updates completed before a callback panic remain in place. Index remapping uses
a checked slice lookup with the same signed-to-unsigned conversion, preserving
negative and out-of-range indices. Public signatures are unchanged. This removes
24 production lines, including stale comments about an optional mapper result.

Actual-module comparisons agree on 196,864 outcomes across 12,304 trees in both
overflow-check modes. They include all binary operators, mixed trees, callback
order, interrupted updates, boundary indices and repeated index remaps. The
applied source matches the compared candidate. All 371 targeted release tests,
286 CLI cases and 48 binary/XML compiler oracles pass unchanged. Allocation
request counts and requested-byte totals match in three full-compilation
repetitions on Toy builder, x86-64 and Hexagon.

Alternating uninstrumented runs pinned to CPU 40 measured Toy-builder wall time
at 6.3405 → 6.3717 ms (+0.49%, 110 samples per version), x86-64 at 432.8713 →
439.9308 ms (+1.63%, 12 samples), and Hexagon at 144.1154 → 144.1817 ms
(+0.05%, 12 samples). Paired wall-time medians changed +0.47%, +1.62% and
-0.02%; paired child CPU medians changed +0.52%, +1.62% and -0.08%. These
results fit the 5% budget; every output image is identical. Measurements
followed warmups on the shared host.

The operand-remapping snapshot passed all four gates: 675 upstream and 1,467
stage assertions, 7,714 workspace tests with 38 existing ignores, and the spec
check. The option catalog passed. No baseline, test count or oracle expectation
changed.

A broader compiler comparison reruns the saved build from before macro-template
sharing against the operand-remapping snapshot. This baseline follows the
initial compiler fixes; it is not the original PR base. All three complete
output images still match. Three allocation repetitions reproduce these totals:

| Spec | Allocation requests before → now | Bytes requested before → now |
| --- | ---: | ---: |
| Toy builder | 26,410 → 16,835 (-36.26%) | 2,874,354 → 2,331,153 (-18.90%) |
| x86-64 | 3,737,622 → 2,724,101 (-27.12%) | 317,842,112 → 239,646,482 (-24.60%) |
| Hexagon | 883,974 → 689,014 (-22.05%) | 83,298,700 → 76,750,658 (-7.86%) |

These measure allocation/reallocation requests and requested bytes, not peak
or resident memory. Balanced CPU-40 timing measures Toy at 6.7870 → 6.3528 ms
(-6.40%, 110 samples per version), x86-64 at 474.1347 → 440.2971 ms (-7.14%,
12 samples), and Hexagon at 149.4232 → 144.1632 ms (-3.52%, 12 samples).
Paired wall medians are -6.42%, -7.17% and -3.24%; paired child CPU medians are
-6.88%, -7.18% and -3.25%. This comparison checks the accumulated compiler
changes, including the later refactors whose individual timings increased.


### Register and context cross-reference setup

`SleighBase::build_xrefs` and `reregister_context` iterate global symbol ids
without a temporary vector. Names remain borrowed until stored in the register
map, user-operation list or duplicate report. Register insertion uses one map
entry lookup. The original name ordering, first storage binding, duplicate-pair
order, callback sequence and partial state on errors remain intact. Public
signatures and option defaults are unchanged.

An external differential model extracts the exact two methods and context-field
helper into wrappers using the real symbol-table and storage types. All 48,000
outcomes across 2,000 tables agree, including missing scopes, name collisions,
register aliases, sparse user operations, invalid context fields, callback errors
and panics, and repeated build/register/build calls. The production build and
371 focused tests pass. All 286 CLI cases and 48 independent binary/XML compiler
fixtures preserve their outputs.

An allocation-counting runtime probe measures engine construction and SLA
initialization only, excluding input reads, metadata formatting and teardown.
Three repetitions per image produce identical register/user-operation metadata
and context sizes. Allocation/reallocation requests change from 2,357 to 2,308
for Toy, 204,401 to 203,957 for x86-64, and 78,970 to 78,230 for Hexagon.
Cumulative requested bytes decrease by 711, 18,154 and 14,900 respectively;
these figures are allocation traffic, not peak memory. Instrumented elapsed
times are not used as performance evidence.

Balanced, CPU-40-pinned runs of all 16 independent lift fixtures (1,171
instructions per run) pass 66 times. With 30 measured samples per version,
median wall time changes from 418.9417 to 419.6009 ms (+0.1573%; paired median
+0.1763%). CPU time changes +0.2265% (paired +0.2620%), within the 5% budget.
Artifacts use `/tmp/kuna-deslop-xrefs-`: `runtime-cost/comparison.json`,
`runtime-verified/timing.json`, `verified/` and the model/build/check logs.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; the workspace has 7,714 passing tests and 38 existing
ignores across 433 groups. Spec and catalog checks pass. No baseline moved.


### Borrowed register-name lookup

The private register-name helpers return borrowed bytes from the cross-reference
map. Public `SleighBase` methods still return owned byte vectors; snapshot
lookups construct their strings directly from the borrowed name, removing an
intermediate vector. Exact storage-key matching and containing-register search
remain distinct, including their existing address-space identity behavior.

External wrappers extract the actual helper methods and both caller surfaces.
With overflow checks enabled and disabled, 3,488,000 outcomes across 2,000 maps
agree in each mode. Cases cover distinct address-space objects sharing an index,
size and offset boundaries, wrapping ranges, misses, empty names and invalid
UTF-8. All 371 focused tests pass, including the 48 pinned binary compiler
oracles. The 286 CLI cases and 48 corresponding XML fixtures are unchanged.

For each register in three real SLA images, an allocation probe queries exact,
interior, zero-size and negative-size locations through both snapshot lookup
methods. Three repetitions produce identical result bytes. Allocation requests
change from 467 to 234 for Toy, 13,987 to 6,994 for x86-64, and 5,007 to 2,504
for Hexagon; cumulative requested bytes change from 11,712 to 11,040, 373,808
to 325,144, and 133,730 to 122,065 respectively. These totals include collection
of the returned strings, excluding initialization, query setup, formatting and
teardown. They measure allocation traffic, not peak memory.

A separate uninstrumented lookup benchmark runs 100 repetitions of those queries
per sample. CPU-40-pinned, balanced before/after runs retain identical result
checksums. With 30 measured samples per image, median lookup time improves
10.85% for Toy, 6.38% for x86-64 and 5.18% for Hexagon; paired medians improve
11.02%, 6.41% and 5.23%. This isolates name lookup and is not a whole-decompiler
speed claim.

All 16 independent lift fixtures (1,171 instructions per run) pass 66 full
runtime runs. With 30 measured samples per version, median wall time changes
from 426.2699 to 422.8330 ms (-0.8063%; paired -0.6300%). CPU time changes
-0.7385% (paired -0.6970%), within the 5% budget. Artifacts use the
`/tmp/kuna-deslop-register-name-` prefix: `cost/comparison.json`,
`lookup-verified/timing.json`, `runtime-verified/timing.json`, `verified/`
and model/build/check logs.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Constructor display construction and printing

`Constructor::add_syntax` inspects the last display piece once when normalizing
whitespace or joining adjacent literals. Full-text, mnemonic and body printers
share literal/operand-piece handling, while retaining their existing index
ranges and flow-through dispatch. Redundant commentary and repeated branches
are removed: 24 fewer production lines, with public signatures unchanged.

A differential model compiles both actual symbol modules against the runtime
library. With overflow checks enabled and disabled, all 245,760 outcomes across
4,096 cases agree in each mode. The model mixes syntax insertion, operand
insertion and trailing-space removal, checks intermediate display state, and
compares printing results, partial text, operand stacks and callback traces.
It includes malformed pieces, empty and invalid UTF-8 strings, boundary indices,
flow-through lookup failures, and injected callback errors and panics.

All 371 focused tests pass, including the 48 pinned binary compiler oracles.
The 286 CLI cases and 48 corresponding XML fixtures are unchanged. A separate
runtime probe disassembles all 1,171 instruction locations from the 16 lift
fixtures through both public assembly APIs. Both APIs agree, with zero decode
errors, and all strings match the saved Rust baseline. The assembly strings
are before/after comparisons; the independent C++ fixtures supply the locations
and expected p-code, not assembly text.

CPU-40-pinned, balanced compiler measurements remain within the 5% budget:
median wall time changes +0.5246% for Toy (110 samples per version), +0.8922%
for x86-64 (12 samples) and +0.2058% for Hexagon (12 samples). Paired medians
change +0.4167%, +0.6858% and +0.1234%; compiled images are identical.

All 66 assembly benchmark runs preserve the complete output hash. With 30
measured samples per version, median wall time changes from 345.5080 to
347.0425 ms (+0.4441%; paired +0.3705%); CPU time changes +0.4112%
(paired +0.3727%). A separate 66-run p-code benchmark passes all 16 independent
fixtures each time: median wall time changes -0.5558% (paired -0.1760%) and
CPU time -0.5591% (paired -0.1877%). These full workloads include initialization.

Artifacts use `/tmp/kuna-deslop-constructor-print-`: `compiler-verified/`,
`assembly-verified/`, `runtime-verified/`, `verified/` and model/build/check
logs. The assembly driver and timing runner are `/tmp/kuna-deslop-assembly-probe.rs`
and `/tmp/kuna-deslop-assembly-timing.py`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Native runtime adapters

The native `Sleigh` register-name adapter uses the same borrowed lookup as the
base API and register snapshots, avoiding an intermediate byte vector when
forming a string. P-code emission no longer accepts an unused address-space
manager. The build result is matched directly, and unimplemented-template
reporting no longer constructs an unused walker. Public signatures, pool reuse,
emission order and diagnostic formatting remain unchanged; production code is
16 lines shorter.

The extracted register-name methods match 3,488,000 outcomes across 2,000 maps
with overflow checks enabled and disabled. The cases retain distinct-space
identity behavior, boundary sizes and offsets, wrapping ranges, misses, empty
names and invalid UTF-8. All 371 focused tests pass, including the 48 pinned
binary compiler oracles; 286 CLI cases and 48 XML fixtures are unchanged.

The independent lift corpus has no unimplemented-template errors. An additional
small compiled specification therefore exercises missing semantics in the root,
a nested constructor and delay slots, plus successful delay-slot execution,
relative branches, loads, invalid instructions and emitter panics. All 72
before/after outcomes match exactly, including 32 unimplemented errors, repeated
calls, partial emitted operations, error explanations and lengths, and assembly
after each attempt. This is a Rust before/after comparison; the error baseline
was captured before the emission/build-boundary edits. The fixture and driver
are `/tmp/kuna-deslop-runtime-errors.slaspec` and
`/tmp/kuna-deslop-runtime-errors.rs`.

Three repetitions of the native engine's lookup probe produce identical result
bytes. Allocation requests change from 467 to 234 for Toy, 13,987 to 6,994 for
x86-64 and 5,007 to 2,504 for Hexagon. Cumulative requested bytes change from
11,712 to 11,040, 373,808 to 325,144 and 133,730 to 122,065. The counts cover
lookup and collection of returned strings, excluding initialization, query
setup, formatting and teardown; they are allocation traffic, not peak memory.
A separate uninstrumented benchmark runs 100 query repetitions per sample.
CPU-40-pinned, balanced runs with 30 measured samples per image show median
lookup changes of -11.69%, -5.70% and -4.26% respectively (paired medians
-12.13%, -5.73% and -4.27%). These are lookup measurements, not whole-decompiler
speed claims.

Full p-code and assembly workloads each pass 66 balanced runs on CPU 40, with
30 measured samples per version. P-code output matches all 16 independent
fixtures (1,171 instructions); median wall time changes +0.9155% (paired
+0.4098%) and CPU time +0.7977% (paired +0.3293%). Assembly output at all 1,171
locations matches the saved baseline through both public APIs with zero errors;
median wall time changes -0.2440% (paired -0.1541%) and CPU time -0.2195%
(paired -0.1675%). Both workloads include initialization and stay within the
5% budget.

Artifacts use `/tmp/kuna-deslop-engine-name-`: `cost/`, `lookup-verified/`,
`runtime-verified/`, `assembly-verified/`, `verified/`, the error transcripts,
and model/build/check logs.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### P-code cache construction

P-code construction extends its varnode pool from an exact-sized iterator,
uses the cache's derived default and borrows one varnode while resolving each
relative label. Direct and dynamic inputs share location generation, and
pointer adjustment copies the original queued operation through its existing
`Clone` implementation. Indices, reuse, label growth, wrapping and error ordering
are preserved; production implementation is 13 lines shorter. The crate header
also replaces the obsolete claim that the compiler remains C++ with its current
relationship to `kuna-slacomp`.

The actual cache implementation matches 589,824 stepwise outcomes across 4,096
sequences with overflow checks both enabled and disabled. Comparisons include
partial state on errors, missing labels, repeated resolution, invalid indices,
clearing and reuse, emitted varnodes and emitter panics. All 371 focused tests
pass, including the pinned binary compiler oracles and lift vectors.

Two small specifications exercise dynamic reads and writes through memory
exports, byte slices with zero and nonzero pointer adjustments, missing root
and nested templates, delay slots, invalid instructions and emitter panics.
Their 144 runtime outcomes match before and after, including diagnostic text,
instruction lengths, partial emissions, retries and subsequent assembly.
The specifications cover both byte orders and were compiled with the pinned
C++ compiler; runtime comparisons use the saved Rust baseline. Initial fixture
attempts with an unquoted register display were rejected by both compilers;
the final specifications use literal display text.

An allocation probe decodes all 1,171 instruction locations across 16 lift
fixtures three times with independent engines. Each repetition retains the
same statuses and output digest over 7,321 p-code operations and 19,581
varnodes. Allocation requests remain 8,090 and cumulative requested bytes
remain 1,073,016. Counts cover instruction decoding and a nonallocating emitter,
excluding initialization, fixture loading, address setup and result formatting;
they do not measure peak memory. This workload shows no allocation reduction.

All 66 balanced, CPU-40-pinned timing runs pass the 16 independent lift fixtures.
With 30 measured samples per version, median wall time changes from 421.0922 ms
to 420.2576 ms (-0.1982%; paired -0.3728%) and CPU time changes -0.3103%
(paired -0.4350%), within the 5% budget.

Artifacts use `/tmp/kuna-deslop-pcode-cache-`: `model.py`, `model.rs`,
`model.log`, `cost.rs`, `cost-comparison.json`, `runtime-verified/`, the
`errors-{little,big}.slaspec` fixtures and before/after transcripts, and
build/check logs. The error driver is `/tmp/kuna-deslop-runtime-errors.rs`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.

### Runtime module documentation

The independent header cleanup replaces obsolete port milestones with the
implemented interfaces and ownership: the parser/context split, template
mutators, register lookup snapshots and IR emulation in `kuna-decomp`.
It removes 201 lines across eight headers. Root integration preserves its
already corrected `pcodecompile.rs` header and label documentation, so the
seven headers applied here remove 170 lines. Relevant representation details
remain, including byte-string names, manager-index constants and label state.

Every byte following each module header matches the committed baseline.
Documentation builds before and after pass with broken intra-doc links
treated as errors. Independent evidence is in
`/tmp/kuna-deslop-runtime-docs-body-proof.json` and matching build logs;
root verification is under `/tmp/kuna-deslop-fid-dedup.Dk64MKp8/`.
There is no executable-code change to benchmark.

The independent branch passes all four gates: 675/675 upstream and 1,467/1,467
stage assertions, 7,714 workspace tests with 38 ignores across 433 groups,
and spec/catalog checks. Root integration totals are recorded in `deslop.md`.


### XML load-image buffers

XML image encoding writes hex directly into its content string. Decoding shares
one helper for the existing permissive digit arithmetic. Redundant chunks are
removed in address order without collecting their keys or repeatedly searching
the map, and padding is appended in one batch. The second pass retains a key
snapshot because it inserts new chunks that must not be visited during that
pass. Source is 26 lines shorter, including the corrected padding contract.

The full before/after module implementations match 90,984 outcomes with
overflow checks enabled and disabled. The cases cover 1,024 maps, all 65,536
byte pairs and 1,024 longer content streams, including repeated padding, empty
and overlapping chunks, wrapping endpoints, aliased spaces, partial reads,
XML encoding and partial state on panics. All 371 focused tests pass, including
the independent compiler and lift oracles.

A public-API allocation probe opens and encodes the XML images used by the
16 lift fixtures, with three independent images per fixture. Each repetition
has identical counts. Opening changes from 388 allocation requests and 46,001
cumulative requested bytes to 258 requests and 35,089 bytes. Encoding changes
from 10,920 requests and 238,388 bytes to 520 requests and 155,188 bytes. All
26,106 encoded bytes match. Counts isolate opening and encoding; they exclude
SLA initialization, XML-tree parsing and image construction and are allocation
traffic, not peak memory.

A separate uninstrumented benchmark measures 200 repetitions per phase and
fixture, including image construction/destruction for opening and fresh output
buffers for encoding. Sixty-six balanced runs on CPU 40 provide 30 measured
samples per version. Median opening time changes -29.26% (paired -29.29%) and
encoding time -42.80% (paired -42.77%). These are isolated phase measurements.

Full p-code timing also passes 66 balanced runs over all 16 independent
fixtures. Median wall time changes from 420.3024 ms to 416.1230 ms (-0.9944%;
paired -0.8273%) and CPU time changes -0.8186% (paired -0.6052%), within the
5% budget. This workload includes initialization.

The assembly workload passes 66 runs at all 1,171 locations through both public
APIs, with identical output and zero decode errors. Median wall time changes
-0.0530% (paired -0.3198%) and CPU time -0.0646% (paired -0.2768%), also within
the budget.

Artifacts use `/tmp/kuna-deslop-imagexml-`: `model.py`, `model.rs`, `model.log`,
`cost-comparison.json`, the `cost-{before,after}-output/` XML files,
`isolated-verified/`, `runtime-verified/`, `assembly-verified/` and build/check
logs. The phase benchmark is `speed.rs`; allocation measurements use `cost.rs`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Context buffer ownership

Context words and explicit-change masks resize their vectors directly, keeping
existing capacity instead of allocating two replacements and copying the old
prefixes. Cache hits and misses select the database slice before one shared
copy into the caller's buffer. Getter order, bounds updates and the existing
cache-hit refetch remain the same. The implementation is eight lines shorter.

The two private buffers have equal lengths: construction, reset and the custom
clone maintain that invariant, and callers receive slices. Cloning still copies
values and clears explicit-change masks. The complete module comparison matches
376,832 outcomes across 2,048 sequences in each overflow mode, including resize
and clone sequences, eight separately named context words with nonzero defaults,
registration failures, range paints, cache hits/misses, short output buffers,
write masks and partial state after panics. All 371 focused tests pass.

Three initialization measurements per specification produce identical metadata
and stable allocation counts. Toy remains at 2,308 allocation requests; x86-64
changes from 203,957 to 203,955; Hexagon remains at 78,230. Cumulative requested
bytes increase by 24, 8 and 8 respectively, from 662,877, 60,051,728 and
22,338,449. This small capacity tradeoff accompanies reuse of the vectors.
These figures cover engine construction and SLA initialization, excluding input
reading, metadata formatting and teardown; they do not measure peak memory.

Both full workloads pass 66 balanced runs on CPU 40, with 30 measured samples
per version. P-code output matches all 16 independent fixtures; median wall time
changes +0.3378% (paired +0.2433%) and CPU time +0.4215% (paired +0.2345%).
Assembly at all 1,171 locations matches through both public APIs with no errors;
median wall time changes -0.3619% (paired -0.3297%) and CPU time -0.3580%
(paired -0.3464%). Both are within the 5% budget.

Artifacts use `/tmp/kuna-deslop-context-buffer-`: the actual-module `model.py`,
`model.rs` and `model.log`, `cost/comparison.json`, `runtime-verified/`,
`assembly-verified/` and build/check logs. Initialization measurement uses
`/tmp/kuna-deslop-runtime-init-bench.rs`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Borrowed memory-state access

Memory-state register setters borrow the varnode's address-space handle while
writing the value, removing two transient `Rc` clones. Bank lookup uses one
checked vector lookup and retains the existing unmapped-space behavior. The
implementation is seven lines shorter; public interfaces and error strings stay
unchanged.

The complete-module comparison matches 8,794 outcome rows (854,281 bytes) in
each overflow mode, and the native library produces those same rows. The driver
covers 2,048 memory actions per byte order, named and varnode register access,
missing registers, null spaces, aliased/negative/out-of-range bank indices and
held-bank borrow failures. An additional 512 snippet programs per byte order
exercise arithmetic, loads, branches, retries and partial failures as collateral
coverage. All 371 focused tests pass.

An isolated benchmark measures 200,000 memory rounds and 200,000 snippet runs
per byte order, excluding environment construction. Memory rounds perform a
named write, a varnode write and a varnode read. Sixty-six balanced runs on CPU
40 provide 30 measured samples per version. Median register-access time changes
-4.4141% for little endian (paired -4.0389%) and -5.3379% for big endian (paired
-5.7443%). The unchanged snippet workload changes -1.7811% and -1.7357% (paired
-2.0858% and -1.9080%). Every result checksum matches. These are workload timings,
not an allocation or whole-decompiler speed claim.

A broader candidate removed transient snippet-operation and behavior clones as
well. Its follow-up timings exceeded the 5% budget (+5.2153% little endian and
+5.0769% big endian, paired +5.1381%/+5.0473%), so that part is excluded. The
final patch changes only `MemoryState`; snippet implementation bytes are unchanged.

Full p-code timing passes all 16 independent fixtures in each of 66 balanced
runs. Median wall time changes -0.9886% (paired +0.6725%) and CPU time -0.9747%
(paired +0.6890%), within budget. The final native benchmark executable matches
the hash of the measured executable.

Artifacts use `/tmp/kuna-deslop-emulation-borrow-`: `workload.rs`, `model.py`,
`model-final.log`, native before/after output, `isolated-verified/timing.json`,
`runtime-verified/timing.json` and build/check logs. Excluded variants and their
timing results remain in the `initial` and `refined` artifacts.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Runtime API documentation

Five runtime modules now describe their implemented interfaces directly. Symbol
payload documentation includes the existing compiler-only variants, and the
snippet-language interface identifies its actual `SleighBase` implementation.
Builder and walker descriptions no longer refer to unfinished porting waves.
The emulation and memory headers retain ownership, callback re-entry and
memory-page edge cases while removing repeated migration details.

The change removes 77 comment lines. Every non-comment, nonblank source line is
identical, including all error strings; hashes are recorded in
`/tmp/kuna-deslop-runtime-api-docs-source-proof.json`. Rustdoc passes with broken
intra-doc links denied. The candidate and rustdoc log use the same
`/tmp/kuna-deslop-runtime-api-docs-` prefix. No behavior, interface or option
changes, so runtime benchmarks do not need repeating.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Borrowed temporary records

`UniqueState::set` now lends the overlapping records to `OptimizeRecord::coalesce`
instead of cloning them into a temporary vector. Coalescing still visits the
existing definitions followed by the new record, first for span bounds and then
for read/write counts. Map replacement and checked arithmetic retain their
order. Public interfaces and compiler output are unchanged; implementation code
is two lines shorter.

A differential probe extracted the actual private record/state implementations
and compared 32,768 transitions with overflow checks both enabled and disabled.
It covers gaps, overlaps, zero/negative sizes, extreme offsets, read/write count
overflow, and state after a caught panic. Outcomes and panic messages match.
Four allocation workloads each perform 256 merges; three samples per workload
produce identical full states and stable counts. Aggregate allocation plus
reallocation requests fall from 4,867 to 2,819, and cumulative requested bytes
from 454,216 to 203,800. These are allocation requests, not peak memory.

Uninstrumented versions of those workloads ran in 66 balanced CPU-pinned runs,
with 30 measured samples per version and 2,000 repetitions per workload. Median
aggregate time falls 20.4571% (paired median 20.4078%); each workload improves.
Full compiler timing remains within the 5% budget: Toy +0.1480%, x86 -1.4271%,
Hexagon +1.0475% wall time (paired +0.2280%, -1.7217%, +1.0006%). Every generated
binary image matches. The 371 focused tests pass, including compiler and lift
oracles; 286 CLI cases and 48 XML outputs remain identical. Probe sources, raw
samples, output states and logs use `/tmp/kuna-deslop-unique-records-`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Compiler API documentation

Six pattern/compiler modules now describe current interfaces without porting-wave
labels or unfinished-driver claims. Scanner tokens are resolved to symbol ids;
parsed field qualities are converted by the driver; failed includes restore the
previous location before the scanner reports an error. The pattern header keeps
the sign-dependent `do_or` mutation behavior, and compiler comments retain arena
ownership and template-rewrite responsibilities.

The patch removes 73 comment lines. Every non-comment, nonblank source line is
identical, with hashes in `/tmp/kuna-deslop-compiler-api-docs-source-proof.json`.
Rustdoc passes for both SLEIGH crates with broken intra-doc links denied. The
candidate and validation logs use `/tmp/kuna-deslop-compiler-api-docs-`.
No behavior, public interface or option changes, so timing does not need repeating.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Pattern normalization

`PatternBlock::normalize` uses fixed-width leading/trailing bit counts for byte
padding and a reverse-position search for the final nonzero mask word. This
removes 24 lines of manual loops while preserving the leading-word drain,
mask/value shifts, offset updates and sentinel behavior. Public interfaces and
storage layout are unchanged.

A differential probe compiled both complete, actual modules and compared
32,768 outcomes per overflow mode. It covers zero and unaligned masks, sentinel
sizes, extreme offsets, unequal mask/value lengths, panic messages and partial
state, including repeated normalization. All 128 complete block-state fixtures
also match. The 371 focused tests, 286 compiler CLI cases and 48 XML outputs pass;
nine complete runtime initialization metadata files match across three samples
each for Toy, x86 and Hexagon.

The isolated benchmark reuses input buffers and normalizes two million blocks
per workload. Across 66 balanced CPU-pinned runs, aggregate median time improves
20.1247% (paired 20.2969%). The unchanged sentinel workload measures 7.3198%
slower (paired 6.7173%), so this is not an improvement for every microbenchmark.
Recompiling the probe against the final library reproduces the measured binary
hashes. The compiler, fresh-initialization and full p-code workloads stay within
the 5% budget: compiler wall deltas are -0.1117%, +0.3686%, +0.0029% for
Toy/x86/Hexagon, and initialization deltas are -0.5271%, +0.0151%, -0.9184%.
Initialization uses an uninstrumented probe with source bytes read before timing;
every measured run initializes 300/10/30 fresh engines respectively. Full p-code
wall time changes +0.3311% (paired +0.2845%); all 16 independent lift fixtures
pass in each of 66 runs. Every compiled image is identical. Sources, exact
baselines, raw samples and logs use `/tmp/kuna-deslop-pattern-normalize-`.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Context and image documentation

The context and image-loader modules describe their implemented contracts
without migration inventories or repeated class/trait explanations. The headers
retain context mask reset and flow-stop rules, fresh database reads on cache
hits, image byte-count/error conventions, XML symbol cursors and the unchanged
read-only address set during clearing or relocation. `ImageBytes` is included
in the loader overview.

The three files lose 94 comment lines. Every non-comment, nonblank source line
is identical, with hashes in `/tmp/kuna-deslop-context-image-docs-source-proof.json`.
Rustdoc passes with broken intra-doc links denied. Candidate and validation
logs use `/tmp/kuna-deslop-context-image-docs-`. No behavior or public interface
changes; no timing repetition is needed for this documentation patch.

All four repository gates pass: 675/675 upstream and 1,467/1,467 stage
assertions retain parity; 7,714 workspace tests pass with 38 existing ignores
across 433 groups. Spec and catalog checks pass. No baseline moved.


### Shared XML image relocation

Byte chunks and symbols use one relocation helper. It retains the unsigned
word-size scaling, signed 32-bit byte-offset truncation, address wrapping and
ordered overwrite behavior. Chunks move before symbols; read-only markers and
the symbol cursor retain their addresses. Both maps keep the existing partial
state if relocation panics. The change removes 14 source lines and two temporary
address-space reference-count increments/decrements; public interfaces and
payload ownership are unchanged.

The actual-module differential probe matches 92,160 outcomes with overflow
checks both enabled and disabled. It covers 1,024 maps and ten signed adjustments,
inverse retries, wrap collisions, distinct spaces with equal indices, stale
cursors, read-only markers, reads, XML encoding, and missing-space panic payloads
and partial state. All 371 focused tests pass.

The public-API probe opens real XML images containing 0/1/16/256 source chunks
and symbols across byte-addressed and word-addressed spaces. All 16 complete
XML outputs match after four adjustments. Across 66 balanced CPU-pinned runs
with 30 measured samples per version, aggregate median relocation time improves
4.3530% (paired 4.4523%). The individual changes are +2.3111%, -5.5181%,
-1.3188%, and -7.2525%, so the empty-image control also stays within the 5%
budget. Image setup and XML checksums are outside the timed section; each sample
performs 1,000,000/200,000/20,000/2,000 alternating relocations respectively.
No allocation reduction is claimed. Probe sources, full outputs, hashes, raw
samples and logs use `/tmp/kuna-deslop-image-relocation-`.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Compiler symbol ownership documentation

`slghsymbol.rs` and `sleighbase.rs` now describe their remapping and arena
ownership contracts without migration-stage labels. Operand expressions own
independent references that need remapping, constructor sections store handles
into the base's template arena, and missing symbol replacements retain their
original ids. Source-file decoding extends the index range used for encoding.
The two files lose 33 comment lines; every noncomment, nonblank source line is
identical before and after. Public signatures and runtime behavior are unchanged.

Rustdoc passes with broken intra-doc links denied for `kuna-sleigh` and
`kuna-slacomp`. Source identity evidence and build/check logs use the local
`/tmp/kuna-deslop-symbol-ownership-docs-` prefix.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Borrowed constructor operand expressions

Constructor pattern building borrows each operand's defining expression instead
of copying the expression tree before choosing between its defining symbol and
expression. The symbol branch still takes precedence, including malformed
operands that contain both definitions. Public signatures and ownership remain
unchanged; `slghsymbol.rs` loses four source lines.

The complete before/after modules agree on 12,288 outcomes per overflow-checking
mode across 4,096 symbol tables, including 4,623 outcomes without diagnostics
and 980 panic outcomes. Comparisons include diagnostics, panic payloads, partial
table and equation state, and retries. All 371 focused SLEIGH tests pass; 286
compiler CLI cases preserve status, both output streams and compiled images,
and all 48 XML output fixtures match.

A public-API allocation probe runs `SleighCompile::run_compilation` three times
per specification in fresh processes. All samples agree, and all nine complete
compiled images and diagnostic streams match. Allocation/reallocation requests
for Toy, x86 and Hexagon fall from 16,816 / 2,671,156 / 681,381 to
16,798 / 2,671,102 / 680,770. Requested bytes fall from
2,328,801 / 233,293,010 / 75,834,626 to 2,328,225 / 233,291,282 / 75,815,074.
Across one compilation of each, that is 683 fewer requests and 21,856 fewer
requested bytes, not a measure of peak memory. Both versions use the same
input and output paths, with compiler construction outside the counter.

Balanced CPU-pinned compiler timings use 110 measured runs per version for Toy
and 12 each for x86 and Hexagon, after warmups. Median wall-time deltas are
+0.570% / +0.358% / -0.219%; paired medians are +0.611% / +0.239% / -0.571%.
All compiled images match, and every workload stays inside the 5% budget.
Models, source snapshots, allocation probes, native compiler snapshots and raw
timing samples use the local `/tmp/kuna-deslop-operand-expression-` prefix.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Emulator callback contracts

Emulator API documentation now describes its Rust traits and callbacks directly.
Default breakpoint hooks return `true` without changing emulator state; missing
registered callbacks return `false`. Registration replaces the callback for a
key and retains the supplied shared callback. Operation names use the first
translator match, with a low-level error for an unknown name. The execution
and memory trait docs distinguish engine-provided handlers from the shared
handlers that reject unsupported operations.

`emulate.rs` loses 109 comment lines. Every noncomment, nonblank source line is
identical, so public signatures and runtime behavior are unchanged. Rustdoc
passes with broken intra-doc links denied for `kuna-sleigh` and `kuna-slacomp`.
Source identity evidence and check logs use the local
`/tmp/kuna-deslop-emulator-contract-docs-` prefix.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Borrowed runtime context commands

Runtime context application iterates over each constructor's stored commands
without cloning their expressions. The symbol table stays immutable while the
commands update the parser context. Stored order, errors, preceding local
updates and queued commits remain unchanged. Public signatures are unchanged;
`sleigh.rs` loses three source lines.

A differential model uses the actual before/after `apply_context` bodies with a
probe walker, matching 36,864 outcomes per overflow-checking mode across 4,096
symbol tables. It compares errors, panic payloads, local context, queued commits,
event order and retries. Nested context application is a mock stress case;
actual decoding rejects `inst_next2` while its parser state is uninitialized.
The actual-engine probe confirms that refusal and matches 192 complete p-code
and context records across both byte orders, including sequential writes,
commits, malformed expressions, delay slots and retries. An additional 144
runtime error/retry outcomes match. All 371 focused SLEIGH tests pass.

Across 16 runtime fixtures, three samples per fixture agree on statuses,
p-code counts and hashes. For 1,171 instruction calls, allocation/reallocation
requests fall from 8,090 to 2,110 and requested bytes from 1,073,016 to 824,424.
These totals describe one repetition and exclude engine initialization; they
do not measure peak memory. Each repetition emits 7,321 operations and 19,581
varnodes without errors. Independent golden fixtures compare complete p-code.

An uninstrumented probe times three fresh-engine repetitions per fixture, with
initialization outside the timer. In 66 balanced CPU-pinned runs, 30 measured
per version, aggregate median instruction time improves 6.805% (paired 6.664%).
Individual median deltas range from -41.538% to -0.354%; every fixture stays
within the 5% budget. Whole lift-oracle runs improve 0.837% in median wall time
and 0.851% in CPU time (paired 0.746% and 0.742%). Each of those 66 runs passes
all 16 fixtures. Models, source snapshots, native probes, hashes, raw samples
and logs use the local `/tmp/kuna-deslop-context-commands-` prefix.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Borrowed runtime handle definitions

Runtime handle resolution evaluates operand expressions and fixes result
templates by reference. Their immutable SLEIGH tables retain ownership while
the parser context receives the computed handles. The existing error paths and
preceding handle updates remain intact. Public signatures are unchanged;
`sleigh.rs` loses one source line.

Constructor documentation now correctly describes its retained compiler fields,
owned context commands and template-arena handles. It previously claimed that
`pattern`, `pateq` and `inerror` had been dropped. All noncomment source in
`slghsymbol.rs` is identical; three comment lines are removed overall. Rustdoc
passes with broken intra-doc links denied.

All 371 focused SLEIGH tests pass. A public-API probe matches 360 complete
p-code/context records across both byte orders, covering constants, expression
trees, token/context/address values, nested `inst_next2` decoding during handle
resolution, malformed operands, missing expressions, division panics, delay
slots and retries. An additional 144 runtime error/retry outcomes match.
All 16 standard runtime fixture statuses, p-code counts and hashes agree.

For 1,171 instruction calls, allocation/reallocation requests fall from 2,110
to 1,412 and requested bytes from 824,424 to 802,088. Three samples per fixture
agree; these totals describe one repetition with engine initialization outside
the counter, not peak memory. Each repetition emits 7,321 operations and 19,581
varnodes without errors. Baseline executables and allocation results were
copied from the validated context-command change's final snapshots, after
verifying exact source identity; hashes record that provenance.

Across 66 balanced CPU-pinned runs, 30 measured per version, the uninstrumented
instruction probe improves 3.936% in aggregate median time (paired 3.776%).
Individual deltas range from -11.299% to +1.022%, all within the 5% budget.
Each run uses three fresh-engine repetitions per fixture, with initialization
outside the timer. Whole lift-oracle runs improve 0.337% in median wall time
and 0.380% in CPU time (paired 0.481% and 0.503%); each of those 66 runs passes
all 16 fixtures. Native probes, complete outputs, source snapshots, hashes,
raw timing samples and logs use `/tmp/kuna-deslop-runtime-handle-expression-`.

All four required gates passed: `make test` (675/675), `make test-stages`
(1,467/1,467), `make rust-test` (7,714 passed, 38 ignored across 433 groups),
and `make check-spec`. Both parity gates report `PARITY OK`; the option
catalog and whitespace checks also pass. Neither baseline changed.

### Synthetic operand walker state

Operand-value evaluation now keeps only the synthetic instruction offset it
uses. An optional offset replaces `OobState` and its unused constructor/length
fields; one walker construction handles both a resolved operand offset and the
current-node fallback. Explicit defining expressions are borrowed, while
symbol-produced expressions retain ownership through `Cow`. Public APIs and
existing error paths are unchanged. The code refactor removes 39 source lines.

The actual offset-helper bodies agree on 368,640 outcomes per overflow-checking
mode over 8,192 immutable contexts: 4,753 offsets, 84,960 fallbacks, 104,366
errors and 174,561 panics. The model substitutes field-compatible private
context/cursor probes, uses the real symbol table, and normalizes the old
valid/offset result to an option; it is not a full-module or full expression
evaluator model. A public-API probe separately matches 1,080 complete p-code
and context records across direct, fallback and defining-symbol paths in both
byte orders. It covers nested-reference rejection, context-time `inst_next2`
refusal, missing definitions, arithmetic panics, delay slots and retries.
An additional 144 runtime error/retry outcomes and all 16 standard runtime
fixture statuses, p-code counts and hashes match. All 371 focused tests pass.

Allocation totals on the standard fixtures are unchanged: 1,412 requests and
802,088 requested bytes per 1,171 instruction calls, with three stable samples
per fixture. No allocation reduction or peak-memory improvement is claimed.
In 66 balanced CPU-pinned runs, 30 measured per version, median instruction
time changes +0.567% (paired +0.687%); individual deltas range from -4.299%
to +3.619%. Whole lift-oracle median wall/CPU times change -0.356%/-0.295%
(paired -0.303%/-0.342%). All workloads stay within the 5% budget; each of the
66 whole-fixture runs passes all 16 oracles. Sources, hashes, model adaptations,
native probes, complete outputs and raw timing samples use the local
`/tmp/kuna-deslop-runtime-operand-eval-` prefix.

A subsequent documentation-only pass removes another 85 comment lines while
preserving every noncomment source line of the measured implementation.
It states that reader cursors are copied, explains arena cursor rebasing and
decode-time pattern capture, and describes operand masks as snapshots taken
before visiting the containing constructor's children. Stale migration notes
and repeated implementation narration are removed. The source proof links the
measured algorithm snapshot to the final documented source.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.

### Reachable snippet grammar and context-bit contracts

The snippet parser no longer carries a private `New` token, match arm and
expression builder that neither its lexer nor its symbol classifier can
produce. `new` remains an ordinary identifier; the lexed but rejected `borrow`
token is unchanged. The refactor and related documentation remove 38 source
lines without changing public APIs. The `clear()` contract now correctly says
that it removes `inst_dest` and `inst_ref`, and the constant-space accessor
has its own description instead of an unrelated label-resolution comment.

Complete before/after parser modules agree on 10,656 outcomes per overflow
checking mode: 444 byte programs, both byte orders, four language bindings
for `new`, and fresh, reused and cleared parsers. All parsing uses the public
`SnippetLanguage`/`parse_stream` boundary with real supporting library modules.
The rebuilt native library matches all 10,656 records, including full XML
operation templates, diagnostics, result presence and temporary-base state;
3,122 outcomes succeed and none panic. All 371 focused tests pass.

A separate native parser benchmark uses 66 balanced CPU-pinned runs, 30 measured
per version. Each run parses 444 programs 32 times in eight configurations,
113,664 parses in total, with language and fixture setup outside the timer.
Median aggregate time changes +1.464% (paired -0.047%); individual medians range
from -1.307% to +4.439%, within the 5% budget. Every run preserves all eight
configuration digests. This is a structural cleanup, with no speed or allocation
improvement claimed. Complete source snapshots, model adaptations, native
outputs, hashes and raw samples use `/tmp/kuna-deslop-snippet-new-`.

Context-bit documentation also removes 23 comment lines. It correctly numbers
bits from most significant 0 to least significant 31 and states that callers
must provide an ordered, nonnegative range within one word. The constructor
does not validate that precondition. A separate source proof under
`/tmp/kuna-deslop-context-bitrange-docs-` confirms all noncomment source lines
are unchanged, including the fields exposed by derived `Debug` output.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.

### Shared token-pattern construction

The true-pattern constructor delegates to the existing boolean constructor.
Concatenation reuses the minimum-length calculation and performs one final
intersection after clamping the alignment shift to zero. Token copying,
ellipsis checks and their rejection order are unchanged. These three small
consolidations remove 15 source lines without adding a helper or changing APIs.

Complete source modules agree on 20,736 public-API outcomes per overflow mode,
including complete result and input states: 7,560 successes, 13,176 errors and
no panics. A boundary adapter appends private-state constructors to the same
unchanged modules and compares 32,768 additional cases per mode. It varies token
sizes across integer boundaries with true/false patterns to bound allocations.
Checked mode preserves 3,091 overflow panics, 10,069 successes and 19,608 errors;
unchecked mode preserves 12,360 successes and 20,408 errors. The real release
library separately matches all 20,736 complete public outcomes after rebuilding.
All 371 focused tests pass, including the compiler binary and lift oracles.

In 66 balanced CPU-pinned native runs, 30 measured per version, aggregate median
time changes +0.823% (paired +0.911%). The six ellipsis configurations range
from -0.371% to +1.745%, within the 5% budget; every run preserves their result
digests. Each run performs 124,416 concatenations over 1,296 fixture pairs,
with fixture preparation outside the timer. No speed or allocation improvement
is claimed. Sources, complete records, model adaptations, hashes and raw samples
use `/tmp/kuna-deslop-token-concat-`.

The accompanying equation documentation removes another 38 comment lines.
It describes token identity, mutable expression children, operand-layout state
and arena-owned nodes without obsolete port milestones or ownership lectures.
The source proof confirms every noncomment line matches the modeled code;
the cumulative branch's earlier header and method-documentation fixes remain
intact. Rustdoc passes with broken intra-doc links denied.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.

### Primitive bit queries and native wide division

Four public bit queries now use Rust's primitive leading-zero, trailing-zero
and population-count operations, retaining their zero sentinels. Multiprecision
division replaces the private digit splitting, shifting and Knuth machinery
with native `u128` division and remainder. The existing 64-bit path and the
smaller-numerator shortcut remain. Zero division still panics for a 64-bit
numerator and returns the same error for a wider one, leaving outputs untouched.
A private wide helper stays separate from the narrow wrapper with
`#[inline(never)]` to keep wide-path overhead off narrow calls. Public APIs are
unchanged; the six source files together lose 313 lines, including documentation.

The source comparisons cover 4,211,522 values for all four bit helpers and
211,460 division cases in each overflow mode. Division preserves 211,078
successes, 192 errors and 190 panics, including failure payloads and output-array
state. Both complete division modules compile unchanged in the retained model;
bit helper bodies are extracted verbatim. The actual rebuilt library separately
passes those same cases against the fixed earlier Knuth-division reference.
The extra `u128` reference in the original harness is now the implementation,
so it is not counted as independent evidence. Focused tests pass: 293 passed
and three existing ignored tests across 23 groups.

In 66 balanced CPU-pinned native runs, 30 measured per version, aggregate median
time improves 62.599% (paired 61.901%). Wide division improves 76.322%, division
by a 32-bit value 55.240%, smaller-numerator handling 25.407%, and the narrow
64-bit path 7.573%. Bit-helper medians improve 28.836% to 79.896%. All eight
scenario digests match in every run. Each run performs 1,048,576 calls; fixture
construction and independent expected digests stay outside the timers. These
are numeric microbenchmarks, not an end-to-end decompiler speed claim.

Four earlier implementations were rejected because the narrow-division control
regressed despite aggregate improvements: a shared helper, its inline variant,
direct digit normalization, and flat native division. Their raw timings and
executables remain beside the retained split-helper evidence under
`/tmp/kuna-deslop-bit-counts-`. The retained source/library hashes and reference
scope are recorded separately, so discarded results cannot be confused with
this implementation.

Numeric documentation now describes implemented register lookup and format
providers, shared ownership, actual module contents and descending varnode
size ordering. The address-space lookup comment also fixes a pre-existing
broken rustdoc method link. These documentation changes preserve all
noncomment source lines; their proof files record the snapshots.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.


### Base error, encoding and address-space contracts

The base headers now describe implemented interfaces instead of promising
later port waves. Error documentation distinguishes the `Decoder` catch
hierarchy and explains that conversion of the database's duplicate-function
payload retains only its explanation. The corresponding database comment
no longer promises another error-enum variant.

Encoding documentation describes explicit ID registration, byte strings,
opcode extensions and borrowed address-space managers. It also records the
existing XML close behavior without implying extra validation. Space docs
cover shared ownership and join records, installed register lookup, constructor
flags and endianness, and the implemented truncation wrapper. The constraint
on decoding spaces through a borrowed manager remains explicit.

The four files lose 144 comment lines. Their snapshot comparison preserves
every noncomment, nonblank source line, including all error strings and public
APIs. Evidence is under `/tmp/kuna-deslop-base-contract-docs-`; no runtime or
performance change is claimed.

Base rustdoc passes with broken intra-doc links denied. Expanding that check
to the engine finds 198 existing broken-link diagnostics across 57 files.
Repeating it with all four original source snapshots produces the identical
message/file multiset; no new broken link is introduced. The before/after
logs and comparison are retained with the other evidence.

All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.


### Native limb arithmetic and comparison

The remaining generic comparison, addition and subtraction loops are replaced
by unsigned `u128` comparisons and wrapping arithmetic. Two private conversion
helpers share the little-endian limb representation with wide division. Public
signatures and existing tests are unchanged; the module loses 33 lines.

Complete before/after modules agree on 2,259,081 operand pairs for both
comparisons, addition and subtraction in each overflow mode. Another 324,617
division cases retain 324,108 successes, 319 errors and 190 panics, including
failure payloads and result arrays. Arithmetic compares against the original
limb loops. The division reference already uses native `u128`, so those cases
check conversion behavior rather than an independent division algorithm.
The actual rebuilt library passes the same cases against the fixed prior
module. Focused tests pass: 293 passed, three existing ignored, across 23 groups.

In 66 balanced CPU-pinned native runs, 30 measured per version, aggregate
median time is 1.018% lower (paired 0.979% lower). All twelve individual medians
stay within the 5% budget, ranging from 14.058% lower to 0.682% higher. Every
scenario digest matches on every run. Each run performs 1,572,864 calls;
fixture construction and reference evaluation stay outside the timers.
This is a numeric microbenchmark, not an end-to-end decompiler speed claim.
Source, executable and library hashes, complete model comparisons and raw
samples are retained under `/tmp/kuna-deslop-limb-arithmetic-`.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.


### Direct mutable partition lookup

`PartMap::get_value_mut` borrows the predecessor entry directly instead of
cloning its key and searching for it again. The default interval has an early
return, covering both an empty map and queries before its first split point.
The inclusive lookup boundary, returned value, split structure and public
signatures are unchanged. Existing tests are unchanged; the module loses
seven lines, including more precise lookup documentation.

Both complete source modules agree after 131,072 mutation steps in each
overflow mode. The sequences use integer and String keys with Vec values,
covering default, exact, interior and extreme lookups, splitting, clearing,
bounds and full map contents. The rebuilt library separately agrees with the
fixed original module. Focused tests pass: 293 passed, three existing ignored,
across 23 groups.

In 66 balanced CPU-pinned native runs, 30 measured per version, aggregate
median lookup time is 46.728% lower (paired 46.363%). All twelve scenarios
improve, by 28.664% to 83.244%; each run performs 786,432 mutable lookups across
integer and String keys. Every result digest and final map/default state agrees
with the original implementation. Fixture construction and reference execution
stay outside the timers. These are lookup measurements, not whole-decompiler
speedups. Evidence and source/library hashes are under
`/tmp/kuna-deslop-partmap-mut-`.

A direct-range version without the default return was rejected because its
empty integer-map case increased 16.495%. An empty-only return was also
rejected because the nonempty default interval increased 5.907%. Their sources,
executables and raw samples remain under the `direct-*` and `empty-*` prefixes.

Final rustdoc passes with broken intra-doc links denied. All four required gates
passed: `make test` (675/675), `make test-stages` (1,467/1,467), `make rust-test`
(7,714 passed, 38 ignored across 433 groups), and `make check-spec`. Both parity
gates report `PARITY OK`; catalog and whitespace checks pass. Neither baseline changed.
