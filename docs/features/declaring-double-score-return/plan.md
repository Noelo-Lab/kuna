# Implementation plan

1. Extend `calleeretpreserves` output characterization only for a locked `void`
   prototype, using the prototype model's ABI output entries. Preserve only an
   exact logical output slice inside a wider machine-register range and model
   every overlapping flank as killed. Preserve the existing decoded-body,
   body-write, and explicit-effect vetoes.
2. Let the exact MSVC cookie recognizer run non-destructively when
   `calleeretpreserves` is on. Require the checker to be locked void in this arm,
   collect and seed every exact call marker before requesting one restart, and
   leave all ops intact. Permit incomplete failure-tail summaries, retain
   positive direct writes and STORE spaces recovered before the unresolved
   edge, and let every recovered ABI-return write, every STORE into an ABI
   output processor space, and every explicit effect veto the marker.
3. Make long and loop-carried cookie values provable with a bounded 256-link
   peel and a cycle-aware phi walk that requires agreeing non-backedge inputs
   and at least one concrete scramble seed.
4. Add unit controls for ABI output versus scratch storage, generic and
   exact-marker callee-body clobbers, generic and exact-marker explicit effects,
   an incomplete output-space STORE with no direct written range, nine checker
   sites in one restart, long carry chains, loop phis, non-deletion, and both
   gates off.
5. Vendor a deterministic PE64 twin and pin both option arms, a checker that
   writes XMM0 before a nested call, and a real 16-byte XMM overlap whose low
   ABI-output lane survives while its upper scratch lane remains killed. Re-run
   the exact dataset witness, parity suites, workspace tests, specification
   checks, and catalog freshness.
