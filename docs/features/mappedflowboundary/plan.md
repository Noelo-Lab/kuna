# Design

1. Carry over strict mapped-failure versus undefined-external diagnostics.
2. Gate matching linked ELF i386/x86-64 flows with `mappedflowboundary`.
3. Snapshot the loader's current shared bytes for each flow follow; release
   the snapshot before later overlays. Validate actual instruction bytes in
   SLEIGH before context commits or p-code emission, without another decode.
4. After flow overrides and known-function bounds, terminate an unmapped
   fall-through through existing warning and missing-halt machinery, then
   drain other mapped worklist paths. Explicit branches may cross gaps.
5. Cover partial instructions, warmed padding, touching/overlapping segments,
   mapped data tails, overlays, repeated flows, and wrong targets. Refresh
   only mapping warnings when eligible flow is rebuilt.
6. Run unchanged 675 parity, stage parity, the workspace suite, spec/catalog
   checks, an interleaved off/on speed check, and the public ELF x86 sweep.
   Default on requires unchanged original expectations and <=5% slowdown.
