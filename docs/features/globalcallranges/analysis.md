# Global call guard allocation growth

The public CrashLoggerSSE v1.25.0 release contains `SKSE/Plugins/CrashLogger.dll`
with SHA-256 `c5ef54156f4eff3d94769dd839ca465ad5c302293d397fd452fb805f1b328122`.
Obtain the exact input with:

```sh
gh release download v1.25.0 --repo alandtse/CrashLoggerSSE --pattern CrashLogger_1.25.0.7z
7z x CrashLogger_1.25.0.7z SKSE/Plugins/CrashLogger.dll
```

Its initializer at `0x180005650` is one straight-line basic block with 22,266
explicit RAM writes and 2,325 calls. The calls target only `0x180092b20` (851)
and `0x180113870` (1,474). These helpers each store two words through their first
register argument and return that same pointer. The second helper zeroes both
words; the first copies its other two arguments. Their only LOAD fetches RET's
return address. Thus their instantiated destination intervals are exact, small,
and disjoint from almost all globals the initializer writes.

Heaptrack identified `Heritage::guard_calls -> Funcdata::new_indirect_op` as the
allocation source: almost one million INDIRECTs already exist at a 2 GiB stop.
The varnode arena dominated the heap, followed by operation storage, input
vectors and indexes. The initializer potentially requests about 51.8 million
range/call guards. Raising limits or shrinking one IR field would leave that
cross product intact.

With the same current processor specs and release engine, run:

```sh
/usr/bin/time -v kuna decompile SKSE/Plugins/CrashLogger.dll 0x180005650 --addr \
  --max-fn-memory 4096 --max-fn-seconds 120 --option globalcallranges off --json
/usr/bin/time -v kuna decompile SKSE/Plugins/CrashLogger.dll 0x180005650 --addr \
  --max-fn-memory 4096 --max-fn-seconds 120 --option globalcallranges on --json
```

On prerequisite tree `3c5ade6c30d1088bf972d5105e1b12ed2c8fa20b`, the off run
stopped at 4,214,208 KiB RSS after 17.60 seconds, without a body. The on run
completed after 62.07 seconds at 289,084 KiB RSS and emitted 685 KiB of C in JSON.
A separate higher-limit baseline reached 14,168,728 KiB and exhausted a 150-second
budget inside heritage. Merging the unusually large collection of typed globals
dominates the successful initializer's remaining runtime. Failure time is not a
completed-decompilation speed baseline.

The native constructor fixture grows the same call/global cross product. At 512
calls/global stores, the original engine used 763,764 KiB and 11.01 seconds; the
proof used 76,280 KiB and 0.33 seconds. Emitted C was byte-identical for 64, 128,
256 and 512 stores/calls. The checked-in 64-store fixture also has a native
checksum oracle. Its stage regression pins 4,096 INDIRECTs with the option off,
none with it on, and unchanged stores and calls in both C bodies.

The actual DLL's emitted initializer has exactly the raw p-code's 24,591 effect
events in the same order and at the same targets. Every stored scalar, string
literal and constructor argument agrees: zero unknown values and zero mismatches.
`verify_reporter.py DATA_DIRECTORY` repeats this check against
`crashlogger-raw.txt` from `load addr; print raw`, `final-on.c` extracted from the
successful JSON function record above, and the extracted DLL at
`SKSE/Plugins/CrashLogger.dll`.

Independent native controls cover overlapping constructors and readers, different
read/write pointer arguments, absolute global accesses, unknown aliasing and
nonaliasing actuals, overwrite observers, unresolved pointer loads/stores,
diamonds, loops and partial stores. Threshold-sized controls exercised both
System V and Windows register layouts, locked and inferred arguments, and O0/O2
compilation of emitted C. All 112 executions and four fresh byte-overlay cases
passed. Another 24 executions covered global pointer publication/reload and
returned-pointer aliasing, with a disjoint publication positive control. All
passed. A whole-DLL on/off comparison excluding the huge initializer found zero
changed complete records among the other 2,935 functions.

An earlier six-run alternating whole-DLL measurement had median wall times of
51.79 seconds off and 52.55 seconds on (+1.47%). On the final composition, five
repetitions of the checked-in fixture measured 228.55 ms off and 144.91 ms on
(-36.60%), within the 5% budget.

The final composition again produced zero changed full records across all 2,935
healthy functions. Five alternating repeats of a fixed 16-function sample,
stratified across function sizes and including the eight largest healthy bodies,
measured medians of 38.22 seconds off and 38.13 seconds on (-0.24%). All sample
records matched in every run. The default text command also completed at
264,856 KiB RSS in 65.43 seconds, with C byte-identical to the JSON record.
