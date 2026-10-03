# Preserve calls after stack byte fills

1. Reduce the missing-call behavior to an authored ARM loop and execute its
   emitted C. Establish failure before changing heritage.
2. Guard constant-initialized stack ranges against recorded indexed byte
   stores, before partial reads are normalized. Reuse the existing INDIRECT
   representation and keep the broad alias policy opt-in.
3. Exercise byte/word reads, overlapping bytes, disjoint slots, direct stores,
   non-stack pointers, later overwrites and option combinations.
4. Compare every original/stage output, run contributor gates and measure
   off/on timing. Preserve existing assertion expectations.
