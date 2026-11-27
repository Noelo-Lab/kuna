# Android JNI bridge

The four Android libraries compile the same C source for API 24 with NDK r28c.
The JNI header in this release is byte-identical to the r30 header used by Kuna.
The x86-64 Linux library is an off-target control with the same exported names.

`JNI_OnLoad` obtains an environment, looks up `org/example/Bridge`, and registers
two methods. The array argument of `countBytes` establishes the ordinary JNI
ABI. The descriptor of `increment` also fits an Android critical native method
whose only machine argument is the Java integer. The exported methods have JNI
long names. `ordinary_lookup` calls a function at the same table index as
`FindClass` through an unrelated pointer.

To rebuild from this directory, set `NDK` to the r28c installation.

```sh
toolchain="$NDK/toolchains/llvm/prebuilt/linux-x86_64"
for target in aarch64-linux-android armv7a-linux-androideabi i686-linux-android x86_64-linux-android; do
    "$toolchain/bin/${target}24-clang" -shared -fPIC -O2 -fno-stack-protector \
        -Wl,-s,--build-id=none,--icf=none bridge.c -o "$target.so"
done
include_dir=$(mktemp -d)
cp "$toolchain/sysroot/usr/include/jni.h" "$include_dir/"
cc -shared -fPIC -O2 -s -I "$include_dir" -Wl,--build-id=none bridge.c -o x86_64-linux-gnu.so
chmod 0644 *.so
```

`patched-helper.c` declares the two interface slots it uses, so it needs no NDK.
Its `JNI_OnLoad` registers one table as written, then registers a second table
after a straight-line helper stores a new implementation into it through an
index. The second table's static contents are not what the VM receives. These
commands use Clang 21.1.8 and LLD:

```sh
for abi in aarch64 x86_64; do
    clang --target=$abi-linux-android24 -fuse-ld=lld -shared -fPIC -O2 -nostdlib \
        -fno-stack-protector -Wl,-s,--build-id=none patched-helper.c -o patched-helper-$abi.so
done
chmod 0644 patched-helper-*.so
```
