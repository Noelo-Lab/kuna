# Flow-closure driver witnesses

Original N64 assembly with independently executable freestanding Linux callers.
`chain-400` writes all 400 adjacent words before returning; `_start` checks all
stores and both canaries. `retry-failure` re-enters a previously recovered
singleton with an unknown argument as the target. Its caller supplies the valid
second target, but recovery cannot prove that target without a caller contract.
The first target stays reachable and other values take an indirect tail call.
`singletons-128` passes through 128 unrelated constant `jr $25` jumps before a
two-step `$9` dispatcher whose re-entry lands one instruction before the
dispatch; no new edge reaches those jumps, so they must not spend the retry
budget. `two-chains` holds two independent 80-step dispatchers, `$10` and `$9`,
selected by the low bit of `a0`; together they need more retries than one flow
allows, so each keeps its recovered cases, its indirect exit and a budget warning. Their
callers check every store and both canaries under QEMU Octeon68XX.

Rebuild using `build.py` with the explicit tool paths and external artifact
folder described in `../indirect-dispatch/README.md`. Sources, linked images,
extracted bytes and GNU tool identities are bound in `provenance.json`.
The console regression checks the actual flow driver and its recovery count,
independently of late structuring or emitted spelling. The rejected unknown
case is an explicit coverage boundary, not a successful semantic recovery.
