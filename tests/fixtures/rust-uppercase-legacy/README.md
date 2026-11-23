# Legacy hash-shaped function names

`names.S` defines ten independent symbols, with distinct constant-return
bodies. The stage reads the ELF's symbols and decompiles the bodies as data.

```sh
as --64 -o /tmp/kuna-uppercase-legacy.o names.S
ld -m elf_x86_64 --build-id=none -e 0x401000 -Ttext 0x401000 \
  -o names.elf /tmp/kuna-uppercase-legacy.o
chmod 444 names.elf
```
