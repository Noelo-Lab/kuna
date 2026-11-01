# Plan

1. Reproduce a by-value child field disappearing from a stripped Go struct.
2. Order reachable named value dependencies with a bounded iterative walk,
   retaining complete existing layouts and refusing cycles or invalid extents.
3. Check a child without a typelink, malformed graphs and real amd64/arm64 builds.
4. Compare complete type surfaces and whole-image C, measure cost against the
   exact parent build, and run the repository checks before publication.
