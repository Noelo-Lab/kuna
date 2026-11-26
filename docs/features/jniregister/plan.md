# Android JNI types and registrations

During ELF program preparation, identify Android from the image and install the
public NDK declarations in the target's type factory. The build embeds the
generated C declarations. To refresh them, update the header release and digest
in `scripts/gen_android_jni.py` and run it on the matching NDK header. The
generator checks field offsets and lowered prototypes for all four Android ABIs.

At symbol import, bind known hooks and consume registrations reached from
`JNI_OnLoad`. Follow the handles through the calling convention, resolve the
registration table using ELF relocation information, and attach facts at the
implementation address. Function enumeration and single-address decompilation
consume these same facts. The `jniregister` option controls this registration
walk.

At a typed indirect call, use the receiver, field and JNI version to refine the
output parameter of `JavaVM.GetEnv`. Ordinary type propagation can then follow
calls through the resulting environment. Explicit user prototypes retain
precedence.
