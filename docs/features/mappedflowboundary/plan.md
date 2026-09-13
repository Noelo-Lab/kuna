# Design

1. Carry over strict mapped-failure versus undefined-external diagnostics.
2. Gate matching linked ELF i386/x86-64 flows with `mappedflowboundary`.
3. Snapshot the loader's current shared bytes for each flow follow; release
   the snapshot before later overlays. Validate actual instruction bytes in
   SLEIGH before context commits or p-code emission, without another decode.
   On a rejected span, preserve overlapbranch's separately mapped target using
   length-only probes; never lift the padded fall-through or incomplete target.
4. After flow overrides, known-function bounds and declared-range policy,
   terminate an unmapped fall-through with a registered missing halt and
   warnings before draining queued paths, including p-code-free chains.
   Explicit branches may cross gaps; later stub filling reuses the endpoint.
5. Cover partial instructions, warmed padding, touching/overlapping segments,
   mapped data tails, overlays, repeated flows, and wrong targets. Refresh
   only mapping warnings when eligible flow is rebuilt, including with recovery off.
6. Run unchanged 675 parity, stage parity, the workspace suite, spec/catalog
   checks, an interleaved off/on speed check, and the public ELF x86 sweep.
   Default on requires unchanged original expectations and <=5% slowdown.
