# Native calls into a mock player

All four stripped libraries are built from `player.c`. The bridge exercises
two successive method IDs, a helper that populates a static struct in
`JNI_OnLoad`, a caller that reads it through another function, a choice between
two methods, and a cache overwritten by an unknown function. A stripped
method registered in the same initializer makes its own static Java call.
Other entries exercise constructors, virtual, static and nonvirtual dispatch,
weak and strong references, and incompatible class or return types.

From this directory, with Android NDK r28c installed:

```sh
for abi in aarch64-linux-android armv7a-linux-androideabi i686-linux-android x86_64-linux-android; do
    "$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/${abi}24-clang" -shared -fPIC -O2 \
        -fno-stack-protector -Wl,-s,--build-id=none,--icf=none player.c -o "$abi.so"
    chmod 0644 "$abi.so"
done
```
