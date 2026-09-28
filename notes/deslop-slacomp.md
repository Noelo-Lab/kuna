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
