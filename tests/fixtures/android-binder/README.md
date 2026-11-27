# NDK Binder service

The four ABI fixtures compile the same service for Android API 35. On API 36
the class factory also registers its transaction names using the weak NDK
import. The name array has ordinary writable static storage. The stripped
AArch64 controls compile the same service with the map disabled, or with an
exported function that overwrites one name at runtime. The dense variant calls
through a hidden forwarding function and uses a 32-way jump table. Only the
first two transaction names are registered.

Set `NDK` to an Android NDK r28c installation. Run these commands from this directory.

```sh
toolchain="$NDK/toolchains/llvm/prebuilt/linux-x86_64"
build() {
    "$toolchain/bin/${1}35-clang" -shared -fPIC -O2 -fno-stack-protector -fvisibility=hidden \
        -D__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__ -DNAMED="$2" -DMUTABLE_NAME="$3" \
        -Wl,-s,--build-id=none,--icf=none,--no-undefined service.c -lbinder_ndk -o "$4"
}
for target in aarch64-linux-android armv7a-linux-androideabi i686-linux-android x86_64-linux-android; do
    build "$target" 1 0 "$target.so"
done
build aarch64-linux-android 0 0 no-map.so
build aarch64-linux-android 1 1 mutable-map.so
"$toolchain/bin/aarch64-linux-android35-clang" -shared -fPIC -O2 -fno-stack-protector \
    -fvisibility=hidden -D__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__ -DNAMED=1 -DDENSE=1 \
    -Wl,-s,--build-id=none,--icf=none,--no-undefined service.c forward.c -lbinder_ndk -o dense-helper.so
chmod 0644 *.so
```
