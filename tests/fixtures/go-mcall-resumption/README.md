# Go scheduler resumption fixture

Build the stripped AMD64 ELF with GNU as and ld.

```sh
as --64 mcall.S -o /tmp/kuna-go-mcall.o
ld -s -m elf_x86_64 --build-id=none -e _start -Ttext=0x401000 \
  /tmp/kuna-go-mcall.o -o mcall-elf
chmod a-x mcall-elf
```

The authored Go metadata identifies `runtime.mcall` by entry address.
The helper saves the caller's return PC and stack pointer before switching
stacks. Its scheduler callback restores the saved stack and jumps to the
saved PC. Resumption reaches the caller's sentinel store and machine return.
The other caller reaches terminal `runtime.goexit0` through `main.mcall`.
The stage reads the ELF as data.
