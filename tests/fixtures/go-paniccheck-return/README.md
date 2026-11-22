# Go panic check continuation fixture

Build the stripped AMD64 ELF with GNU as and ld.

```sh
as --64 paniccheck.S -o /tmp/kuna-go-paniccheck-stage.o
ld -s -m elf_x86_64 --build-id=none -e _start -Ttext=0x401000 \
  /tmp/kuna-go-paniccheck-stage.o -o paniccheck-elf
chmod a-x paniccheck-elf
```

The authored Go build marker and function table let the stage use the same
load and symbol commit path as a stripped Go program. The helper has a return
and a conditional call to `runtime.throw`. Its caller calls `runtime.gopanic`
next, followed in the machine bytes by a sentinel store. `runtime.gopanic`
and `runtime.throw` are terminal in the fixture. The no-return option control
shows the sentinel when the terminal call is treated as returning. The stage
reads the ELF as data.
