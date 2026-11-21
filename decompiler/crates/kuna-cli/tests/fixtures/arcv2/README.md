# ARCv2 fixtures

Built with the Zephyr SDK 0.17.0 `arc-zephyr-elf` toolchain (GCC 12.2.0):

```bash
F="-mcpu=em4 -O2 -ffreestanding -nostdlib -Wl,-Ttext=0x1000 -Wl,-e,sum -Wl,--build-id=none"
arc-zephyr-elf-gcc $F loops.c sink.s -o loops.elf
arc-zephyr-elf-gcc $F -mbig-endian loops.c sink.s -o loops_be.elf
for f in loops.elf loops_be.elf; do arc-zephyr-elf-objcopy -R .comment -R .ARC.attributes $f; done
arc-zephyr-elf-gcc -mcpu=em4 -O2 -c add2.c -o add2.o
```

SHA-256:

```
1a0b5bb91349fd9e56a9225822de38680f27bf93ffe871730767dbcd007237bf  loops.elf
f9f71a0104fddc65e74c0d940e2067d9abd0cba21c6cd04ddffea258f22fd995  loops_be.elf
b8dba37b0632cda25a5dacbc80d672e31b020d95fc51bfacd8f71068c989c73e  add2.o
```
