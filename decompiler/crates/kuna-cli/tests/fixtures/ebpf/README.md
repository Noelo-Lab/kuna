# eBPF fixtures

Relocatable eBPF objects for `tests/stages/kuna-ebpf-elf-map-reloc.xml`,
built with clang 22 from the sources beside them:

```bash
clang --target=bpf   -O2 -c bpf_map.c     -o bpf_map.o
clang --target=bpfeb -O2 -c bpf_map.c     -o bpf_map_be.o
clang --target=bpf   -O2 -c bpf_subcall.c -o bpf_subcall.o
```

`bpf_map.o` carries one `R_BPF_64_64` against the `.maps` symbol `counters`
(the `lddw` at `prog+0x20`); `bpf_subcall.o` adds an `R_BPF_64_32` subprogram
call to `helper` in `.text`.
