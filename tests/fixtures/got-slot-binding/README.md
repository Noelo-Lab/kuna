# ELF function-slot binding controls

In the protected image, the relative and absolute calls encode the same link-time
address. Loading this shared object at another base moves only the relative call
and its GOT slot. The call to the default-visible definition of `exported`
can bind to another implementation. The writable image replaces the loader's
initial `exit` pointer with a returning callback.

GNU binutils 2.46 reproduces both images from this directory. Set `TMPDIR` to
a scratch directory before running the commands.

```sh
as --64 -o "${TMPDIR:?}/binding.o" binding.s
ld -shared -Ttext-segment=0x400000 -z relro -o binding_x86_64.so "$TMPDIR/binding.o"
ld -shared -z norelro -o writable_x86_64.so "$TMPDIR/binding.o"
readelf -Wr binding_x86_64.so
```
