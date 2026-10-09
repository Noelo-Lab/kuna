# GD lifetime feedback, 2026-10-05

Naming the group sorter's target-set object changed `head->next` into
`PTRSUB(head,0)`. The pointer printer used the whole variable's storage type
instead of the type facing that read. Reading the selected field type fixes
the next-node access and the object key/target loads. The asset-free fixture
`lifetimes_pointer_fields_x86_64` reproduces this with one object name; its
native result and unnamed/named C must all return 31.

The Dash recipe also omitted three relevant imported-function contracts.
The missing Boolean release argument at IAT RVA516d00 caused the constructor
pointer/Point mixture. The Point copy and assignment contracts at5169f8/516a08
recover the separate Point view over the allocation spill at rsp+58. Declaring
the allocator's pointer return alone did not remove the latter cast.

The shipped libcocos2d.dll establishes both Point contracts: copy constructor
RVA c6b0 copies two four-byte fields from RDX to RCX and returns RCX in RAX;
assignment c760 does the same. EXE allocation helper4d0770 returns the successful
malloc result in RAX, and constructor17ab00 returns its saved storage pointer
at17b07e. These are machine ABI declarations, not C++ source signatures.

Append [gd-dash-lifetime-contracts.kuna](gd-dash-lifetime-contracts.kuna) to the
GD project's `dash_lifecycle_readable` bundle. Keep `stackviews on`,
`stackalias off`, and `nanignore none`. The supplement targets the EXE hash and
preferred base in its header; it does not belong in the engine's default types.
The GD checkout and its durable assertions were not edited here.

Validation distinguishes strict assertion replay from execution: GD was checked
statically against its assembly; the small linked-list fixture also runs emitted
C with GCC and Clang at O0/O2. The corrected Dash bundle replays all eleven
functions with zero rejected assertions; both aggregate casts disappear, including
on the previous engine build when supplied these contracts.

The later frame-order follow-up isolates a correctness failure in
`lifetimes_frame_events_x86_64`. The first array element aliases a fully
initialized eight-byte `ChoiceItem` exposed by a helper. Native execution reads
kind1 and key23. The previous emitted C overwrites the frame with the loaded
pointer before testing kind, producing only the initial observation
(`calls=1 keys=1,0`). Keeping the register value separate until its original
store restores `calls=2 keys=1,23`.

The merge guard preserves the block and memory-observation order of native
stores into generated shared frame storage. Emission then hides reconstructed
memory versions and their unused copy/piece cycles; it keeps their IR and byte
origins. A real cached full word remains live and independent of later key and
pointer writes. A detached producer keeps a declared pointer type only when the
entire logical view originates at its single native store.

In the GD sorter, 22430a now prints only the four-byte seed, 224395 prints the
four-byte target, and 2243c6 publishes the pointer inside the3607/guard-clear
branch. Native224370 stays a load into a separate pointer variable. No artificial
initial wide read, upper-word restore or loop snapshot remains. The four-function
group bundle replays 49 assertions with zero rejects, retaining six logical uses
and five object names. This is static assembly comparison for GD, with executable
native and GCC/Clang O0/O2 checks on the three-function reducer; it does not prove
whole-game equivalence. The signed-short switch and the separately reported
Count dispatcher prototype-store panic are outside this follow-up.

For this follow-up, all 675 original and 2035 stage assertions pass. The full
workspace passes 8096 tests (38 ignored); the 31 focused stack-object CLI tests,
spec consistency, Python tooling and catalog checks pass. The existing RGB stage
now checks its three original byte/word writes, since the two synthetic
whole-RGB rewrites are gone; no earlier stage key was dropped. A paired release
comparison after the workspace run gives default output unchanged and -0.3% wall
time, stack-view reducer -1.4%, and four-function GD replay +1.0% (11.62s to 11.74s,
three measured pairs). These small timing differences do not establish a speedup.


Count dispatcher follow-up, 2026-10-06: an assertion-free query reached local
stack backing before the function had a prototype store. Parameter-view recovery
now declines that early query without creating a store or inventing arguments.
The regression exercises required merging before setup and recovers real formal
views after setup. EXE4bb210 now decompiles with only `stackviews on`; the typed
four-function Count recipe retains all 126 accepted assertions. The explicitly
declared 24-byte CountRemap backing remains part of that recipe; this fix does
not establish automatic widening of an otherwise unknown merged pointer.

This change retains 675/675 original and 2035/2035 stage assertions and passes
8097 workspace tests (38 ignored), spec, catalog, lint and Python tooling checks.
Paired release runs keep emitted C identical on the frame-event control, with
median time -1.1% by default and -0.7% with stack views; those small differences
are timing noise rather than evidence of a speedup.

Float snapshot follow-up, 2026-10-06: Cocos8ce10 cached both Point lanes as
integer words, preserved their sign-bit XOR, then added those words numerically
to floats. P6 now records detached integer frame snapshots; P9 reinterprets
their bits only at native floating consumers, using the actual C declaration.
The marker also covers register-only highs whose other instances retain a
partial-union type. Integer operations and FLOAT_INT2FLOAT remain numeric.

Both cached lanes in the real function now use bit transfers at ADDSS-derived
uses. The six-function geometry recipe retains all31 assertions with zero
rejects. The asset-free CLI regression compiles actual emitted backing types
and C with GCC/Clang O0/O2, checking both Point lanes, finite values, signed
zero, a quiet NaN payload and the separate integer-to-float conversion against
the native reducer. This does not prove whole-game equivalence.

All675 original and2042 stage assertions pass, retaining every prior stage
key and adding seven. The workspace passes8098 tests (38 ignored); spec,
catalog, lint and Python tooling checks pass. Paired frame-event controls keep
their C unchanged: default -0.1%, stackviews -1.3%, both within timing noise.

Cookie pointer follow-up, 2026-10-06: the loader's cookie crossed81 phi
nodes, exceeding the old32-step provenance walk. Cookie provenance now uses
an iterative worklist over the finite graph; transparent links stop on cycles,
every terminal must prove the same frame offset, and every visited node must
reach a scramble seed. Unknown inputs and seedless components still decline.
The independent256-link peel cutoff is gone too.

The native271-call reducer and its actual emitted C preserve the factory
pointer and all271 call observations with GCC/Clang O0/O2. Its ordinary helper
must stay inferred: locking that helper's prototype bypasses the memory guards
and hides the old failure. With the inferred helper, the previous build loses
the pointer in both preservation/removal modes; the patched build recovers it.
The29 focused cookie tests include long chains and refusal controls.

GD19d1e0 now returns the defined object and null on the ordinary paths.
Both keeper/removal replays accept47 assertions; the old v24 name needed
reselection from the fresh output. The factory definition at native19d36f
identifies the intended object. Printed v-numbers differ when stripping the
checker, so a selector refreshed in one mode must not be reused in the other.
The GD checkout was not edited; this is static assembly comparison there.

All675 original and2046 stage assertions pass, with every earlier stage key
retained. The workspace passes8102 tests (38 ignored), and spec, tooling, lint
and catalog checks pass. Paired controls preserve identical C and measure
default +2.0%, stackviews +2.3%, within the5% speed budget.

Saved-XMM follow-up, 2026-10-06: Win64's 16-byte unaffected effects can have
four separate four-byte SLEIGH inputs. The exact-width local restriction misses
their saves. P6 now finds wholly contained unaffected input lanes and excludes
only exact negative-frame saves whose bytes have no surviving read, other
write or escape. Each direct call needs complete existing read/write summaries;
indirect and unknown calls still block. Wrapped overlap checks take constant
time. The action retries after call placeholders resolve, then stops after its
first successful restriction so normal alias cleanup and DCE can finish.

The native CLI reducer checks all16 lower XMM6-9 lanes and the3.75f Rect result
with GCC/Clang O0/O2. Its emitted types and body stay intact; the harness supplies
the Win64 calling-convention attribute and keeps source data at the same linked
address. The optimized helper has a complete straight-line summary; its
unoptimized home-spill variant is a refusal control. Separate stages retain an
explicitly observed Win64 spill and a volatile SysV spill. Unused physical-frame
gap records can remain in JSON; the actual C declarations and assignments are
what the absence check verifies.

This does not yet clear the real-game spills: the seven-body slope replay
accepts157 assertions, and six Cocos geometry bodies accept31, with no rejects
or function errors. Slope1a13b0 still has16 spill locals and Cocos8fcc0 has12.
Imported/virtual calls prevent complete body evidence. Group5e660's R8 save is
also unresolved: entry-RSP+18 belongs to the caller, and the intervening opaque
allocator remains a possible observer. Do not infer a third formal or discard
that write from its later overwrite alone.

The next precision work is transient callee-home provenance and independently
witnessed imported/virtual-call effects. R8 additionally needs caller-argument
and escape evidence for its positive home slot. These are separate proof gaps;
the saved-lane patch leaves their barriers intact.

Specifically, slope calls vtable slot0x490 at1401a13d5; its recipe gives a
prototype but no complete target set. Cocos8fcc0 calls sharedDirector1800bfcc0,
whose lazy-init path includes an indirect call at1800bfd7f. Narrowing that path
needs pointer and stack-argument reachability through nested calls. The O0
reducer exposes another precision gap: exact pointer home stores/reloads are
tracked but also recorded as sticky escapes. A fix must retain their write
effects and prove that the caller never reads or forwards the residual home
values; reloading a pointer alone does not discharge its escape.

Validation retains all 675 original and 2046 prior stage assertions, adding four
stage expectations for a total of 2050. The workspace passes 8116 tests with
38 ignored; spec, tooling, lint and catalog checks pass. The saved-lane stage
fails on the previous build. Paired release controls keep identical emitted C:
default +1.0%, stackviews +1.1%, within the 5% speed budget. The final workspace
run used one build job, one test thread and a guard stopping the process below
25% available RAM; it completed without triggering that guard. Its 5.2 GB debug
build directory was removed afterward.

Callee-home follow-up, 2026-10-07: pointer stores into positive callee-frame
cells now retain their escape and write footprints separately. An unread Win64
home cell can discharge its pointer escape only after checking the locked ABI,
exact call-site mapping and the caller's complete observable uses. Partial
reloads, lost pointer provenance, modeled pointer returns, ambiguous accesses,
unknown calls and published frame addresses remain barriers. Caller scans have
fixed budgets and do not recursively discharge another callee's homes.

Repeated pointer reloads across field writes retain provenance conditionally:
each write must be disjoint from the saved pointer cell at the actual call
site. Physical alias checks apply independently of calling convention; only
the escape-discharge proof requires Win64's home layout. This preserves
existing precision on other ABIs without assuming their cells are private.

An unrelated argument can also expose a home through truncated pointer bits.
Discharge now checks every unresolved incoming source, narrow/bulk frame reads
and loss-metadata overflow. Full register coverage distinguishes initialized
scalars from entry fragments; partial pointer overwrites retain their original
root. Other calls use the same strict fragment barriers even without homes of
their own, so publication through a second helper cannot bypass the proof.

The GCC O0 rectangle-copy reducer now loses its XMM6-9 save locals while
preserving the Rect computation. Actual emitted types and C reproduce the
3.75f result and all sixteen seeded register lanes under GCC/Clang O0/O2.
Caller reads, home-address publication and truncated frame-address publication
retain all four saved registers. The last control previously discharged unrelated
argument homes; losing frame provenance now blocks observation proofs globally.
Clang O0 places pointers in negative private locals; argument writes still
invalidate those spills, so that different precision case remains unresolved.

The reviewed Group caller does not expose entry-RSP+0x18, but this neither
covers the other 86 call sites nor closes allocator callbacks. The inspected
reference CRT can invoke a registered allocation handler and then return
successfully. Imported and nested effects, singleton provenance and those
callbacks still require proofs; the current patch does not clear every game
XMM/R8 spill. The native caller-published-home counterexample remains valid.

Upstream integration, 2026-10-08: the standalone lifetime branch includes this
callee-home refinement. Parameter naming through the console's P9 naming-policy
assertion uses the same prototype replay as ordinary naming. Later narrow
register definitions retain their own identities even when an incoming
parameter occupies the wider register; the executable regression includes
inputs with nonzero upper words.

Against upstream `919984b8`, the original baseline remains 675/675 and the
stage corpus passes 2541/2541. The workspace passes 8429 tests with 38 ignored;
the release Ghidra suite passes all 61 tests, including its normally ignored
sort/grep breadth test. The CLI corpus passes 277/277 and all 42 Python tooling
tests pass. Spec consistency, lint, catalog freshness, shared counters and
ElementId uniqueness pass.

The stage baseline adds 141 assertions net. Its one replaced key strengthens
CALLRETPAIR #6 to preserve the integer return in both modes. The byte-comparison
CLI probe now checks the actual byte read and incompatible pointer assignment,
rather than forbidding a legitimate word-pointer local belonging to a different
lifetime. It accepts healthy upstream and lifetime-aware output and still rejects
the original broken output with `declhightype off`. Executable coverage checks
all failed and successful lookup paths under GCC and Clang at O0/O2.

A fresh replay accepts all 237 assertions across four Group bodies, seven slope
bodies and six Cocos geometry bodies, with no rejects or function errors. These
are static assembly/assertion checks, not whole-game execution equivalence.
The self-contained Pair/Handle reducer reuses eight frame bytes, names the two
objects independently, and returns 67 both natively and through its actual
emitted declarations and body under GCC/Clang O0/O2.

Forty measured interleaved release pairs per case, after two warmup pairs,
compare this branch with upstream `919984b8` in reliable mode. The final row
compares `stackviews off` and `on` within this branch. CPU time includes child
processes; wall time measures the complete CLI invocation. Default-mode emitted
C is identical on all three measured fixtures. These small-fixture measurements
stay within the 5% default speed budget.

| Case | Wall before → after | Wall delta | CPU before → after | CPU delta |
|---|---:|---:|---:|---:|
| Default: stack-object reuse | 121.03 → 123.50 ms | +2.0% | 109.63 → 109.93 ms | +0.3% |
| Default: register reuse | 124.25 → 126.13 ms | +1.5% | 113.07 → 112.60 ms | -0.4% |
| Default: frame events (three functions) | 139.28 → 135.95 ms | -2.4% | 126.41 → 125.36 ms | -0.8% |
| Same build: stackviews off → on | 118.33 → 117.40 ms | -0.8% | 112.93 → 112.83 ms | -0.1% |

Reproduce with built specs and matching console binaries beside each CLI:

```sh
python3 docs/features/lifetimes/speed.py BEFORE_KUNA AFTER_KUNA /tmp/lifetime-speed.json
```
