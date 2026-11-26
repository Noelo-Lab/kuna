# Android JNI prototype recovery

The mock in `tests/fixtures/android-jni/bridge.c` registers a byte-array
method and calls `GetArrayLength` on its input. Stripping removes the
implementation's local C symbol. The native registration still carries its
Java descriptor `([B)I` and implementation address.

Running `kuna decompile tests/fixtures/android-jni/aarch64-linux-android.so
0x48c0 --addr` on the parent revision produces this declaration.

```c
void sub_48c0(long *a0,unsigned long a1,unsigned long a2)
```

With registration recovery enabled, the same address produces the declaration
below and the indirect call resolves to the NDK's `GetArrayLength` field.

```c
jint Java_org_example_Bridge_countBytes(JNIEnv *env,jobject receiver,jbyteArray arg0)
```

The mock's second registered implementation takes one integer and uses the
critical-native ABI. Its descriptor `(I)I` is also valid for an ordinary JNI
method. Both can be named, and only the byte-array method supplies enough
evidence for the two hidden parameters.

## Speed on the AArch64 mock

Run `taskset -c 30 python3 -m docs.features.jniregister.speed` from the repo
root with the release console and built specs. Substitute an available CPU if
needed. Each observation starts a fresh console and decompiles `0x48c0`. After six
warm-up pairs, the script measures 100 pairs, alternating which setting runs
first. It checks that enabling recovery produces the registered name each time.

| Median per decompilation | Off | On | Change (paired bootstrap 95% interval) |
| --- | ---: | ---: | ---: |
| Wall time | 100.675 ms | 102.334 ms | +1.65% (−1.60% to +3.09%) |
| Child user + system CPU | 98.029 ms | 98.251 ms | +0.23% (−1.02% to +2.57%) |

The standard pipeline timed 31 runs off followed by 31 on and measured 66.83 ms
and 83.12 ms (+24.38%). In the paired run the off median moved from 110.04 ms
in the first quarter to 76.29 ms in the last. Alternating the order accounts
for timing variation across the run. Separately, timing the registration function
on the loaded mock for 100 pairs of ten calls gave medians of 1.79 µs off and
87.07 µs on. The additional 85.28 µs recovered two registered methods.
