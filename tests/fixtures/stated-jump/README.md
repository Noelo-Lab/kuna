# Stated jump destinations

The handwritten x86-64 Linux function in `dispatch.S` jumps through a table
supplied by its caller. At 0x401009, the three possible destination addresses
are 0x401020, 0x401030 and 0x401050. They return 11, 22 and 33. The guard
miss at 0x40100b returns 77 without reading the table. The native entry point
exercises the destinations in the order third, first, second, then the miss.
It exits with status zero when all four values match. The ELF also contains an
unrelated indirect jump and a compiler-generated switch from `ordinary.c`.

Rebuild in this directory with GNU as and ld 2.47.20260726 and GCC 15.2.0.

```sh
as --64 -o dispatch.o dispatch.S
gcc -O2 -fno-pie -fno-ident -fno-reorder-blocks-and-partition \
    -fno-asynchronous-unwind-tables -fcf-protection=none -c ordinary.c -o ordinary.o
ld -m elf_x86_64 --build-id=none -Ttext=0x401000 -o dispatch.elf dispatch.o ordinary.o
./dispatch.elf
```

| File or tool | SHA-256 |
| --- | --- |
| dispatch.S | 809f7fbd95dc2743b8df0e1329cc8b1e01f3bcc68aa635e19c0d04a6feec6cf2 |
| ordinary.c | accf83a7f494ef983c6e2e7d0067864ce7fca43d10c75397525c71c3ed76116d |
| dispatch.elf | 39c54c192b6312ac081085be15a017fb9d856e5f0edfca49a2fdb8d06267c268 |
| as | 52192a9bd71873873ceaf23a9bcc09ca2c6d507c2460922b45c0e7c5bb210177 |
| ld | 2865846ece2642439cfae404ef930bfc5494c4babd2347757878b8f0f8f761cf |
| gcc | b5f1b773a7c733738352000c92a077dc5852a1a2fc6d836b1e411be1e9ec5f88 |

The static PIE in `pie.S` computes its jump destinations at runtime using
RIP-relative LEA. Its entry checks both taken paths and the guard miss. Under
an unknown load base its two link-time VMAs establish successors but cannot
label a switch on the unadjusted pointer. Rebuild using the same assembler
and linker as above.

```sh
as --64 -o pie.o pie.S
ld -pie --no-dynamic-linker --build-id=none -Ttext=0x1000 -e _start pie.o -o pie.elf
./pie.elf
```

SHA-256 for `pie.S` is `abeb43746f6275817db61577eb8b11a9a9138e430136f6b928eb002059093e17`.
SHA-256 for `pie.elf` is `2e280cdace67bc88d9f0bcd961e3543fd08cd4c9b4f0e95f9e170d3139a853cf`.
