# Go channel status ABI fixture

`status.S` models the amd64 Go compiler's guarded hot path for a status-only
receive. The on-disk DWARF includes a complete receive-only `int` channel type
and a one-byte boolean result. Separate entry points omit the result or change
the element width, stack address, or direct target. The ELF is static decompiler
input. The helper bodies and cold paths are fixture stubs.

Rebuild from the repository root with GNU binutils.

```sh
as --64 -o /tmp/go-channel-status.o tests/fixtures/go-channel-status/status.S
ld -m elf_x86_64 --build-id=none -o tests/fixtures/go-channel-status/status-elf /tmp/go-channel-status.o
chmod a-x tests/fixtures/go-channel-status/status-elf
```
