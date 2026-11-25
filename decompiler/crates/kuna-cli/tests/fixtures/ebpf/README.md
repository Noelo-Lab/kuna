# eBPF fixtures

Relocatable eBPF objects for `tests/stages/kuna-ebpf-elf-map-reloc.xml`,
built from the sources beside them; clang 21.1.8 reproduces every object
byte-for-byte (the first three were first built with clang 22):

```bash
clang --target=bpf   -O2 -c bpf_map.c         -o bpf_map.o
clang --target=bpfeb -O2 -c bpf_map.c         -o bpf_map_be.o
clang --target=bpf   -O2 -c bpf_subcall.c     -o bpf_subcall.o
clang --target=bpf   -O2 -c bpf_static_call.c -o bpf_static_call.o
clang --target=bpfeb -O2 -c bpf_static_call.c -o bpf_static_call_be.o
```

`bpf_map.o` carries one `R_BPF_64_64` against the `.maps` symbol `counters`
(the `lddw` at `prog+0x20`); `bpf_subcall.o` adds an `R_BPF_64_32` subprogram
call to `helper` in `.text`. `bpf_static_call.o` calls two static functions in
`.text`; both `R_BPF_64_32` relocations name the `.text` section symbol and the
call's `imm` holds the callee's instruction index less one (`-1` for `twice`,
`2` for `plus7`).
