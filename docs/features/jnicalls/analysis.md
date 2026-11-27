# Native calls to Java

The stripped player mock looks up `demo/Player.seek(I)V` in two places. The
first native method invokes its own lookup. A second native method reads an
ID saved by a helper during `JNI_OnLoad`. Query the full managed signature
to find both native callers.

```sh
kuna xrefs tests/fixtures/android-java-calls/aarch64-linux-android.so \
  --to 'Ldemo/Player;->seek(I)V'
```

Both rows have a native call address and a separate lookup instruction. A
lookup class identifies the class used to resolve the ID. The VM chooses
the implementation for virtual dispatch at runtime. The cached row is
incomplete: code the walk does not reach, such as a function called only
through a pointer, could store another ID in the cache. A call inside a
helper is incomplete for the same reason. The same mock has a conditional
choice of two IDs, a global overwritten through another function, and a
primitive native entry that calls through an unrelated table. Only the
first two produce Java call records.

The option's speed record times single-function decompilation of the cached
caller. Each measured run loads the ELF in a fresh release console and checks
whether the call annotation appeared. From the repository root, run
`taskset -c 30 python3 -m docs.features.jniregister.speed --option jnicalls`
with an available CPU. The benchmark alternates the order for 100 pairs after
six warm-up pairs and records wall time and child CPU time.

## Release timings on the AArch64 mock

| Measurement | Off median | On median | Change | Paired 95% interval |
| --- | ---: | ---: | ---: | ---: |
| Standard wall time, 31 runs per setting | 85.60 ms | 89.47 ms | +4.52% | — |
| Alternating pairs, wall time | 82.470 ms | 85.933 ms | +4.20% | -2.33% to +10.10% |
| Alternating pairs, child user + system CPU | 78.939 ms | 81.263 ms | +2.94% | -2.03% to +9.35% |

The standard wall-time gate passed its 5% budget. The paired wall-time and
child CPU estimates were also below 5%, though both 95% intervals extend
above it. Both benchmarks used CPU 30.
