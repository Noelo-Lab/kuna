# Root-fix boundaries for the split-register float ABI work

The prefersplit restoration, the P4 trial-storage bootstrap, and the mixed-access
pointee guard fixed the emitted C for an AArch64 scalar double passed through a
split `q` register. Three mechanisms in that work still violated the project's
ownership rules or left a bounded-completeness hole. This note records what each
one was, why the replacement is neutral rather than a patch, and what remains
outside them.

The runnable witness is `tests/stages/kuna-prefersplit.xml` and the
`prefersplit_scope` CLI tests. The original binary witness is
`sub_17bf1c` in `libshpssdk.so`, which used to pass the value as
`unsigned __int128` and now passes it as a `double`:

```c
v9 = *(double *)a0;
return sub_17e490(v9,a1);
```

With `--option protoorder off` the same call prints `unsigned long v4`, so the
typing still comes from the bootstrap and is still flippable.

## 1. A nested drive was not a scope

The bootstrap runs a callee's whole drive while the caller's drive is paused.
It saved and restored the per-drive switches it knew about, but a drive also
publishes flow and pipeline comments into `Architecture::commentdb`. A callee's
comments therefore entered the caller's program-wide output, and the
"restored exactly" claim in `recover_one_contract` was false.

`park_float_library_prototypes` was not a second leak: it `mem::take`s
`arm_float_library_prototypes`, so the outer drive had already consumed the
list and the nested call is a no-op. An audit of the drive path found no other
program-wide writer.

The drive now owns an explicit depth (`kuna_drive_scope()`, an RAII guard whose
drop balances the depth even on unwind). `kuna_drive_publishes()` is true for
the outermost drive and for a standalone flow build outside any drive; the two
comment publishers and the mapped-flow warning cleanup are gated on it. The
nested drive still hands its statements back explicitly; nothing else survives
it.

Boundary: the policy is "the outermost drive publishes". A future nested drive
that genuinely needs to publish must extend that predicate deliberately; it
must not bypass it at a call site.

## 2. The pointee width guard scanned the whole function

A float value may become a pointer's pointee only when every access through the
base uses that width. The guard rescanned every `LOAD`/`STORE` in the function
for each propagation and refused after 4096 ops. Cost was quadratic in the
function's op count, and the same evidence could be refused just because the
function grew.

`kuna_structsynth::every_access_through` now starts at the base. It walks the
pointer family forward through exactly the ops `peel_base_stride` walks backward
(`COPY`, `CAST`, constant `INT_ADD`/`PTRSUB`, `PTRADD`), then peels each reached
address to prove it names that base. The cost is one pass over the object's own
uses. Dead ops are skipped, matching the scan it replaces. MULTIEQUAL-merged
pointers are not chased, also matching `peel`; such a merge keeps the access map
of each distinct root instead of inventing a shared object.

Boundary: the 256-use budget is a per-object completeness guard. An object whose
family cannot be walked completely refuses the promotion, which is conservative
and independent of the surrounding function's size.

## 3. Partial completion was a private arrangement

`Action::perform` already records `status_mid` when an action returns `-1`, but
`ActionRestartGroup` drives its inner group through `apply` and so never got the
flag. The restart group set it privately so the bootstrap could resume, which
made the pause protocol an implicit contract between the drive and one framework
class.

`ActionGroup::apply` now records `status_mid` on both negative-return paths
(a member's partial completion and the group's own action breakpoint), following
the C++ rule that a partial member makes the group partially complete. The
restart-group special case is deleted. The new test drives a group through
`apply` directly and pins the resume point.

Boundary: any future group type that overrides `apply` must keep the same rule.
The repository currently has only `ActionGroup` and `ActionRestartGroup`.

## Verification

- `make test`: 675/675, PARITY OK
- `make test-stages`: 3203/3203, PARITY OK
- `make rust-test`: full workspace green, using an isolated pristine Go 1.18.1
- `make check-spec`: OK
- `make lint`, `make test-tools`, and `kuna catalog --check`: green
- `make test-cli`: 303/303
- `prefersplit_scope`: 9/9; `kuna_structsynth`: 32/32; `prefersplit`: 10/10;
  the new action test passes
- A Clang-compiled scalar-double wrapper prints an unsigned integer access with
  `protoorder off` and a double access with the default

The bootstrap remains capped at 32 direct callees. A CLI benchmark of
`ccw_entry`, pinned to one CPU with 41 measured runs after five warmups, gave
median times of 93.312 ms on the PR base, 92.189 ms with `protoorder off`, and
93.685 ms with the default. The default costs 1.62% against off and 0.40% against
the base, within the 5% speed budget.

Follow-up regression probes cover constants split at the 64-bit boundary,
sibling callees in a cycle, inherited watchdog deadlines, retained forwarding
claims, and unknown or conflicting stride evidence. Trial scoring now collects
ready callees before argument finalization, so a pause cannot discard deferred
repairs or let a sibling read another sibling's partial contract. Nested drives
share memory accounting and the enclosing deadline, statement recovery stays
inside the restoration boundary, and the paused action tree is restored on
unwind.
