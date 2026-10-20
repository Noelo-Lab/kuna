# Bounded local frame views: issue #810

This records the approved design, causal investigation and implementation evidence. The implementation base still emits an incompatible
aggregate assignment for reused local storage. A tested bounded-recognition
experiment makes that C compile but loses a live conditional initialization.
The missing invariant is correspondence between physical frame writes and the
statements that initialize a generated shared backing. Creating the backing
without preserving that correspondence is unsafe.

The proposed feature extends existing `stackviews` machinery with a bounded
local policy. It does not allocate separate storage for successive object views.
Saved pointers and simultaneous aliases must continue to denote the same bytes.

## Inputs and contributor ownership

- Issue: <https://github.com/Noelo-Lab/kuna/issues/810>.
- Contributor PR: <https://github.com/Noelo-Lab/kuna/pull/1162>, unchanged head
  `3a4ff3d73ef64d6ac6ee62d669b40d998a0ac482`, original base
  `ecd456743593868ae639cebe378bdfde20e0bb80`.
- All eight original commits are by **mgostIH**
  `<13892786+mgostIH@users.noreply.github.com>`. Their bounded recognition,
  shared pipeline integration and original regression witness are the starting
  point for this design. The contributor's PR and description remain unchanged.
- Already-merged #1163, mgostIH commit
  `10669c3e4f8718721c6b045d5aa152565b9af1b3`, supersedes #1162's constant-only
  hidden-COPY re-emission with safer general byte transfers. Its pointer and
  read-timing guards must survive any implementation.
- This proposal is based on merged main
  `5fe62be4073e713d049425742bbc7c721efe04cf`, tree
  `d5fb1cb37cb66547e2c368f8cf328b99e7a9e83c`. Its content is identical to the
  reviewed memory-fix head `ac4883618d1255ab641528c247a06ba43a9e2262` used
  for the causal traces below. Neither includes new #810 engine code.
- The earlier uncommitted experiment is preserved separately, based on main
  `d1f98e52e054bc7684fd5bb13fca5088f3e09a2d` plus #1122 prerequisite content.
  It is evidence of a failed design, not an implementation of this proposal.

The proposal replaces **automatic admission based only on two complete types**
with admission that also establishes physical-write preservation and exact
view correspondence. It retains the contributor's shared-storage model and
boundedness constraints. It does not claim that the original work disappeared
or that #1162 has been superseded publicly.

## Runnable branch counterexample

Create `types.h`:

```c
struct Pair { int x, y; };
struct Handle { const int *value; };
int read_pair(const struct Pair *);
int read_handle(const struct Handle *);
extern int datum;
```

Create `helpers.c`, compiled as a separate translation unit:

```c
#include "types.h"
int read_pair(const struct Pair *p) { return p->x + p->y; }
int read_handle(const struct Handle *h) { return *h->value; }
```

Create `u.c`:

```c
#include "types.h"
int datum = 55;
__attribute__((noinline)) int t9_regs(int a, int b) {
  int r;
  { struct Pair p = {a, b}; r = read_pair(&p); }
  { struct Handle h = {&datum}; r += read_handle(&h); }
  return r;
}
__attribute__((noinline)) int t10_partial(void) {
  int r;
  { struct Pair p = {5, 7}; r = read_pair(&p); p.y = 20; r += read_pair(&p); }
  { struct Handle h = {&datum}; r += read_handle(&h); }
  return r;
}
__attribute__((noinline)) int t11_branch(int c) {
  int r;
  { struct Pair p = {5, 7}; if (c) p.x = 100; r = read_pair(&p); }
  { struct Handle h = {&datum}; r += read_handle(&h); }
  return r;
}
int main(void) {
  return !(t9_regs(3, 4) == 62 && t10_partial() == 92 && t11_branch(1) == 162);
}
```

```sh
gcc -g -O2 -fno-stack-protector -fcf-protection=none -no-pie u.c helpers.c -o u_gcc
./u_gcc
decompiler/target/release/kuna decompile ./u_gcc t11_branch --mode reliable --json --option structdefs on
decompiler/target/release/kuna decompile ./u_gcc t11_branch --mode reliable --json --option structdefs on --option stackviews on
```

Both native GCC and Clang builds exit zero. Runtime `c=0` must return **67**;
runtime nonzero `c` must return **162**. A separate driver can check both by
compiling `u.c` with `-Dmain=source_main` and calling `t11_branch(atoi(argv[1]))`.
The source above was rebuilt and replayed during this investigation.

Current default emits the two Pair fields, then the invalid expression
`v2[0] = (Pair)&datum`. Full `stackviews on` emits correct writes through one
shared union. The failed automatic-recognition experiment instead emits:

```c
__builtin_memcpy((uint1 *)&v2 + 4, &(uint1[4]){7,0,0,0}, 4);
v1 = read_pair(&v2.view_Pair);
v2.value_3 = &datum;
return read_handle(&v2.view_Handle) + v1;
```

`c` is unused and `Pair.x` is undefined. Fully typed helper TUs, strict aliasing,
and `-Werror=int-conversion -Werror=incompatible-pointer-types` were used for
the retained native replay. GCC O0/O2 and Clang O0 returned **1094795647** for
both inputs after stack poisoning; Clang O2 returned **574**. These are observed
undefined-read results, not portable expected numbers. All eight default
comparisons failed; all eight explicit-on comparisons returned 67/162.

The experiment's function code, reproduced by all three narrow variants, has
SHA256 `204bc4a269f86fb2b9cd34dc2a7be07790b9ff0928081e3913809fddf40d0c82`.
The frozen experimental CLI has SHA256
`64f0f3d8434f8efd14ba2b8783ae9167e3d7c1ff0e07011c1553d57fb556586b`.
No claim is made that the current default or contributor branch contains that
uncommitted experiment.

## Causal trace: a surviving COPY is not necessarily the native write

A standalone diagnostic linked the existing release libraries and read final
`Funcdata`; no engine source was changed. On the audited GCC image, instruction
`0x401221` stores the conditional x value computed at `0x40121e`.

| State | Native write Op70 | Reconstruction Op238 | Arithmetic source |
|---|---|---|---|
| Current default, no generated local union | absent | printed, `stack_store=false`, `direct_write=false` | implied INT_ADD, no backing symbol |
| Failed bounded union experiment | absent | hidden, `stack_store=false`, `direct_write=false` | implied INT_ADD, no backing symbol |
| Full `stackviews on` | present, printed, `stack_store=true`, `direct_write=true` | not needed for this store | implied INT_ADD consumed by native write |

These operation IDs describe this pinned run; tests must assert provenance and
behavior rather than hard-code arena IDs. The x expression is still in final
P-code in the failed case, but the original physical-store output is gone.
Op238 is a later storage reconstruction, not proof that the native write
survived. The earlier inference that this was solely a register/backing merge
problem was incorrect.

The relevant decision chain is:

1. `p3_dataflow/ruleaction_4.rs::RuleStoreVarnode::apply_op` converts a resolved
   STORE into a COPY and marks its output `stack_store`.
2. `p6_variables/kuna_stackalias.rs::{prepare,holds_store}` protects aliased
   physical writes when full `stackviews`/`stackalias` is on, or for #1122's
   separately eligible narrow formal ranges. General local objects have no
   default admission. `ActionDeadCode` consumes this policy, and
   `ActionVarnodeProps` consults it when clearing temporary liveness holds.
3. Late local union admission changes the representation without restoring
   native-write correspondence. `kuna_stackobjects::Origins::value` cannot call
   the unmarked reconstruction a physical write: the Pair object remains
   `defined=false`, while its later Handle object is defined.
4. `p9_emit/coreaction_casts.rs::action_set_casts` runs
   `kuna_stackversions::prepare` before view casts. Its `physical_version`
   classifies non-native operations into generated frame backing as memory SSA
   versions and marks them nonprinting. This hides Op238 after automatic union
   admission. With the default Pair layout it was still a printed field COPY.
5. `printc.rs` admits a hidden statement only if the existing
   `kuna_stackbytes::assignment` can reconstruct a safe byte transfer.
   `kuna_stackbytes::expression` does not project an implied INT_ADD. It accepts
   the constant y bytes, but correctly refuses to invent/re-read x.

An isolated widening of `kuna_stackvalueorder::moves_write` was ineffective:
that gate requires an actual `is_stack_store()` output, which Op238 lacks.
Allowing the already-existing P6 object/snapshot recovery for the local backing
also left the C identical and Pair undefined. These ablations reject both narrow
approaches. They do not justify bypassing version suppression or byte-source
guards. Full `stackalias on` retains Op70 as a physical write, providing an
additional liveness control, although it is too broad to adopt as a new default.

Instrumentation on main6d7ab7fff now pins the first loss: STORE70 lowers at
stop191; declared complete Pair/Handle contracts and the fixed frame address
already exist. DCE214 does not hold the native local write, copy propagation225
detaches its last PIECE reader, and early removal230 deletes it. Full views
instead holds the original physical write at DCE214. Reconstructed COPY238
is a different unmarked operation. The bounded policy must retain the native
write before that boundary.

## Escaped scalar and declared pointer-use controls

The existing fixture is runnable without external datasets:

```sh
fixture=decompiler/crates/kuna-analysis/tests/fixtures/lifetimes_stack_x86_64
decompiler/target/release/kuna decompile "$fixture" escaped_alias --mode reliable --json --assert "@$fixture.kuna" --assert-strict --option structdefs on
decompiler/target/release/kuna decompile "$fixture" escaped_alias --mode reliable --json --assert "@$fixture.kuna" --assert-strict --option structdefs on --option stackviews on
decompiler/target/release/kuna decompile "$fixture" escaped_object_reuse --mode reliable --json --assert "@$fixture.kuna" --assert-strict --option structdefs on --option stackviews on
```

For `escaped_alias`, a frame address escapes through the locked global
`int *escaped_pointer`; the binary writes 5 and then 7 before reading through
the saved address. Default final IR contains **no stack write**. Full views (or
explicit `stackalias on`) retains the physical COPY 7 at `0x401068` in this fixture (at `0x4010cd`
in the original archived issue image), marked `stack_store=true`; the native
result is **7**. A late backing cannot recover
that lost effect. This is the same physical-storage contract needed by bounded
local objects, even though no second aggregate type is present.

`escaped_object_reuse` saves the address, reads a Pair, replaces those bytes
with a pointer, then observes the replacement through both the old address and
the Handle view; expected result **122**. On the reviewed memory-fix engine,
full views still emits `escaped_pointer = (stack_views_... *)&v3` despite the
locked `int4 *` receiving contract. GCC and Clang reject this under
`-Werror=incompatible-pointer-types`. The existing test harness does not treat
that warning as an error for every lifetime control; native success alone is
not enough.

`kuna_stackuses::input_contract` recognizes a persistent COPY destination or
STORE receiver, but the final publication in this trace is a register CAST
whose high variable also contains the global. The visible source operation no
longer exposes the original persistent receiver. A pointer-use contract must
remain attached to its actual publication/use boundary through this mapping.
The first loss is copy propagation183 into INDIRECT151, followed by early
removal187 of native receiver COPY13. Required merge425 then gives its frame
pointer source a persistent global high. Preserving the original publication
before propagation lets existing typed-member binding emit
`escaped_pointer = &v3.view_int4` without a printer bypass. Loop PHIs require
the same preservation: bypassing a publication into MULTIEQUAL can synthesize
extra assignments before and after the loop.

`simultaneous_views` must remain **56**, with pointer equality and replacement
bytes observed at one physical object. The proposal includes these strict C
controls; it does not claim the failed experiment passes them.

## Compiler portability and packed numeric sources

The Ubuntu 26.04 CI run lowered the constant Pair initializer to a read-only
eight-byte pool load. Function-only native replay had supplied `datum` but no
definition for that compiler-generated global. The replay now reads the actual
bounded ELF read-only bytes into a `uint8` definition without changing the emitted
body. Plain GCC/Clang inputs remain, with an additional GCC SLP input arm.

That arm exposed a separate real failure in the previously validated build:
`local_register(3,4)` returned 55 instead of 62. Native STORE25 at `0x4011d7`
survived as a held COPY, whose source PIECE125 at `0x4011ca` packs `b,a`.
LOCAL heritage created a forced first-call INDIRECT147 that FULL did not need.
COPY propagation bypassed the original memory COPY, and its backing union type
then reached the external register PIECE through the marker. Required merging
and cleanup hid both the initializer and its native COPY as self-copies.

Keeping that exact COPY-to-marker edge preserves the INDIRECT, retains the
numeric packed value and emits `view_uint8 = CONCAT44(b,a)`, matching FULL.
The controlled edge ablation passes all four strict GCC/Clang O0/O2 cells for
62 and 60; the corrected engine also passes the added SLP arm. The guard requires
the original generational STORE identity, source point, exact recorded output
address/width, live COPY output and stack-store flag, and whole-write containment
inside one admitted local footprint. It changes only marker propagation. It adds
no numeric type inference, PIECE rule, printer bypass or callee-summary policy.
The earlier passing gates and corpus remain historical evidence and do not
supersede this newly discovered wrong output.

## Scope decision

The final read-boundary control exposed a separate physical-extent omission:
two initialized adjacent eight-byte regions are accessed through a locked
four-byte pointee at offset four, but the binary performs an eight-byte LOAD.
The rejected write-only guard admitted an eight-byte union and separated its
neighbor. All four strict GCC/Clang O0/O2 emitted programs returned the wrong
upper word; the native oracle returned `0x5566778801020304` in every cell.
The existing early access check now includes actual LOAD output widths as
negative evidence. The crossing case declines before preservation and exactly
matches its prior unsupported C; the contained LOAD4 control executes correctly.
This adds no saved read state, positive construction rule, wider object or pass.

The owning decision is **P6 alias-facets: when a fixed local frame footprint may
receive automatic shared views**, with preservation established before P3
destructive dataflow work. A late shape match alone is inadequate. A sound
implementation needs an early local footprint/write contract, its lifecycle
through dataflow, and typed use binding after mapping. #1122's early hook can be
reused structurally, but its formal-parameter eligibility is not local evidence.

This is larger than one Action/Rule-shaped pass. It adds provenance/policy state
and spans existing preservation, mapping and pointer-contract boundaries.
`docs/improvement-pipeline.md` §2.4, §4 and standing requirement 5 therefore
require a draft proposal and explicit approval before implementation. The
ordinary issue sweep does not invoke the RE-friction lane's captain approval.
The user approved [proposal.md](proposal.md). Implementation now follows its
bounded native-write and typed-publication contract. Mandatory strict native
controls, 0/675 default ablation and CPU-pinned whole-image speed checks pass.
The complete workspace passes 9,178 tests with 38 ignored and zero failures;
stage, tooling and spec gates also pass. Final whole-corpus evidence records
zero C changes across 1,071 ordinary images and 60 reviewed hunks across 50
typed images, with no new failure identities. The corrected GCC-SLP initializer
and native marker guard are included in these results; earlier passing evidence
does not supersede the rejected initializer failure.
