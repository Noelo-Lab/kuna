A typed callback returning a pair in two registers could lose a result when
its prototype was recovered after dataflow. Reading the first member produced
an invalid cast of the whole struct; reading the second member could return an
undefined local. The same shared call-flow defect blocked Go paired results.

The late prototype now requests a restart when its locked joined output is not
already defined at the exact address and full width. The existing prototype
override installs the full contract before dataflow is rebuilt. Complete joined
outputs retain the in-place update, and scalar or void outputs follow their
existing paths. No return register or callee signature is inferred by this fix.

The C fixture covers either member, a narrowed second member, both members,
whole-pair forwarding, scalar callbacks, void callbacks, and observable call
counts. All eight complete emitted C bodies match the authored source across
338 boundary-input comparisons at O0/O2. Seven complete Go bodies also execute
across 338 comparisons with Go 1.22.5 and 1.27.3. The narrowed Go body still
needs an explicit conversion; an independent direct-call example on unmodified
main reproduces that Go/Rust emitter gap. No Rust native-execution claim is made.

The first guard incorrectly dereferenced void return storage. Eleven existing
stage assertions caught it; the initial source, logs, and a standalone void
counterexample are retained. The corrected guard passes those 44 assertions
and the full corpus. A later test-helper borrow error was fixed before the
workspace suite passed; that failed build log is retained as well.

Final breadth covers 60 images, 34379 distinct functions and
77853 language records. All 59 existing images are unchanged. Nine records
in the callback fixture change: first, second and narrowed-second projections
in C, Go and Rust. Every changed record is listed in breadth-review.json.gz.

Timing uses nine fixed cases and 384 balanced paired rounds with one final
look. Every sample and simultaneous CPU/wall bound is retained in the timing
records. Maximum median change: 0.288%. All timing comparisons meet the predeclared 5% criterion.
