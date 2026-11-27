# NDK Binder transactions

The mock service defines `org.example.ILevel` and passes three native function
pointers to `AIBinder_Class_define`. Decompile the stripped dispatch callback
by address in a fresh process.

```sh
kuna decompile tests/fixtures/android-binder/aarch64-linux-android.so 0x4ac4 --addr
```

The named callback compares its `transaction_code_t code` argument against
two constants. The class's registered name array identifies transaction 1 as
`readLevel` and transaction 2 as `writeLevel`. The no-map and mutable-map
controls retain the numbers without assigning names. The dense mock reaches
32 distinct switch targets and registers names for only the first two.
The NDK defines the map's first entry as transaction 1.
[Android NDK Binder reference](https://developer.android.com/ndk/reference/group/ndk-binder#aibinder_class_settransactioncodetofunctionnamemap)

The speed record times the same callback by address and checks for the
registered name in its decompilation. Run
`taskset -c 30 python3 -m docs.features.jniregister.speed --option ndkbinder`
from the repository root with an available CPU. The benchmark uses a fresh
release console per observation and alternates the settings for 100 pairs
after six warm-up pairs.

## Release timings on the AArch64 mock

| Measurement | Off median | On median | Change | Paired 95% interval |
| --- | ---: | ---: | ---: | ---: |
| Standard wall time, 31 runs per setting | 78.18 ms | 81.39 ms | +4.11% | — |
| Alternating pairs, wall time | 63.021 ms | 62.615 ms | -0.64% | -1.13% to -0.28% |
| Alternating pairs, child user + system CPU | 61.577 ms | 61.126 ms | -0.73% | -1.08% to -0.37% |

The standard wall-time gate passed its 5% budget. The paired wall-time and
child CPU medians both decreased with the option on. Both benchmarks used
CPU 30.
