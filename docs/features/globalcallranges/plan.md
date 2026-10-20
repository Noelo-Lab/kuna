# Global call access ranges

CrashLogger 1.25.0's initializer at `0x180005650` writes 22,266 distinct RAM
addresses and makes 2,325 direct calls to two tiny constructors. Each constructor
writes 16 bytes through its first argument and returns. Heritage currently builds
an INDIRECT for every global at every call: about 51.8 million guards. On main
`d1f98e52e`, a 150-second attempt reached 14,168,728 KiB RSS and stopped in heritage.

Reuse the complete bounded `kuna_calleememory` body walker and its existing cache.
Add a conservative eligibility witness for global queries: absolute memory accesses,
direct memory input/output storage and frame accesses other than the architectural return fetch make that witness
false. Unknown addresses, opaque calls/user operations, unresolved control flow,
incomplete walks and unsupported direct memory writes already fail closed.
Expose only the complete eligible summary's register-rooted read and write ranges.

One new P3 module, `kuna_globalcallranges`, resolves every effect root to the
call's exact matching register parameter and its definite constant actual value.
Only width-preserving copies and bounded constant expressions establish an actual;
unknowns, partial-register mismatches and address arithmetic wrap retain the guard.
All read and write intervals must be disjoint from the global range before
`Heritage::guard_calls` can treat its unknown effect as unaffected. Explicit effect
overrides retain their existing meaning. No types, const qualifiers or presumed
noalias/noescape properties establish disjointness.

A named `globalcallranges` option controls the behavior. Reuse the existing summary
seed and cache; avoid a second body walker or new analysis pass. If seeding cost
needs amortizing, activate the proof only for substantial call/global fanout and
keep the existing behavior for smaller functions.

Validation covers exact/disjoint/overlapping constructor destinations, static
memory observers, unknown and partial actuals, unresolved pointer accesses,
branches, loops, opaque calls, effect overrides, arithmetic overflow and analysis
reset. Compare emitted C against the option-off build, execute reduced native
observers, measure runtime/RSS on growing constructor fixtures and the public DLL,
review every changed function in a whole-corpus sweep, and run all repository gates.
Default-on requires unchanged 675 datatest assertions and the speed budget.
