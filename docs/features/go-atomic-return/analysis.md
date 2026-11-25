# AMD64 Go atomic additions returned directly

An AMD64 `LOCK XADD` implements a single read-modify-write. The ordinary IR
contains the instruction's lock envelope, arithmetic, memory store, and
old register value. In Go output that envelope gives `LOCK()` and `UNLOCK()`
around separate loads and a store. A source expression needs the same update
and its new value, including signed overflow at the physical operand width.

P2 records a bounded original instruction envelope. P9 verifies that all
live effects belong to it or to the same returned sum. It uses the retained
old-register load, the actual stored sum, and the same typed delta and
address. It emits the whole Go return as one signed `sync/atomic` add. The
address and delta are evaluated once. C and Rust continue to use their
ordinary IR renderers.

The stage covers both operand widths, a live old value or carry bit,
unlocked and differently locked instructions, HLE and narrow operands,
shadowed package names, incompatible types, unrelated stores, a retry, and
a call after the update. The source and output language gates are exercised
separately. An independent Go observer for the proposed expressions verifies
signed wrapping, canaries around both targets, and unique values under
concurrent increments. The compiler lowers each observer wrapper to one
`LOCK XADD` with register-only arithmetic for the returned value.

The scope is a typed direct return such as `main.bump` or `main.bump64`.
A result reused by a branch and return needs a once-only local binding. Other
operations and architecture families require distinct evidence, including
AArch64 LSE and exclusive-monitor paths and compare-and-swap retry loops.
The staged source awaits its candidate build and runtime validation.
