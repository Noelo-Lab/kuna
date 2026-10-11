# Bounded preservation before constant-pointer recovery

The owning phase is P3 dataflow. Keep the exact declared global array base
available to the existing pointer-recovery pass instead of combining it with
a negative element displacement too early. Do not rewrite arithmetic or infer
an array from a nearby symbol.

Query the complete single-use additive address tree, permitting same-width
copies and casts. Require a LOAD/STORE in one compatible byte-addressed space,
an exact declared array base, a negative whole-element displacement bounded by
the array size, and variable coefficients divisible by the element size.
Decline pointer-valued variable terms, genuine sharing, incompatible widths,
ambiguous spaces and previously rejected pointer constants.

Apply the guard at the three constant-combination rules that can erase the base.
Limit it to active type recovery and the first three constant-pointer passes;
otherwise leave normal folding available. Expose the decision as
`globalarrayanchor`, catalog default OFF, enabled by `aggressive` after corpus
and timing evaluation. Explicit OFF remains available after preset selection.

Validate the option boundary and rejected patterns in SPARC32 stages and direct
IR tests. AArch64 stages also exercise LOAD/STORE, multiple indices, unsigned
widening from signed arrays and integer bit loads from float arrays. Compare
whole public Debian outputs across twelve architectures, reconfirm differences
with larger budgets and repeated unchanged configurations, and distinguish
positive typed synthetic controls from unaffected corpus paths.
