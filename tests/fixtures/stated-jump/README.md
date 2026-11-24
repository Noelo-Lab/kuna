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
and linker as above. The two explicit flags restate that binutils release's
defaults, so binutils 2.44 and 2.46 produce the same bytes.

```sh
as --64 -mx86-used-note=yes -o pie.o pie.S
ld -pie --no-dynamic-linker --build-id=none --hash-style=both -Ttext=0x1000 -e _start pie.o -o pie.elf
./pie.elf
```

SHA-256 for `pie.S` is `abeb43746f6275817db61577eb8b11a9a9138e430136f6b928eb002059093e17`.
SHA-256 for `pie.elf` is `2e280cdace67bc88d9f0bcd961e3543fd08cd4c9b4f0e95f9e170d3139a853cf`.

`fixed.elf` links the same source at a fixed address. Discovery lists its two
address-taken case blocks as functions although no symbol declares them, so
it checks that stated destinations there remain cases of `dispatch`.

```sh
as --64 -mx86-used-note=yes -o pie.o pie.S
ld --build-id=none -Ttext=0x401000 -e _start pie.o -o fixed.elf
./fixed.elf
```

SHA-256 for `fixed.elf` is `a1bbd7b5bfda23a1f98637560ee4e2df4ee9e1617aff8afe7bd82abd8db5858f`.

`dos.exe` is a 42-byte MZ executable with a two-paragraph header and no
relocations. Its code jumps through `ax` and returns 11 at offset 2 or 22 at
offset 6. DOS chooses the segment it runs at, so a stated destination cannot
label its cases. Write it from the hexadecimal bytes:

```sh
printf '%s\n' 4d5a2a0001000000020000000000000000010000000000001c00000000000000ffe0b80b00c3b81600c3 | xxd -r -p >dos.exe
```

SHA-256 for `dos.exe` is `1afb7518b37d754870e1f2154dceec2fc5ce42b7548a8031e93f956bfa15671a`.

`owners.S` holds three dispatchers that jump through `rdi`, linked at a fixed
address. `owners::split` reaches its GCC-style cold part
`_ZN6owners5splitEl.cold`, a local function symbol whose demangled name equals
its parent's. The unsymbolized function at 0x401040 tail-jumps to `exported`,
whose symbol has no size. The function at 0x401060 has no symbol of its own
and loops back to its entry from beside the size-less symbol `inner`. The
entry point checks every path, and the image exits with status zero when all
of them match.

```sh
as --64 -mx86-used-note=yes -o owners.o owners.S
ld --build-id=none -Ttext=0x401000 -e _start owners.o -o owners.elf
./owners.elf
```

SHA-256 for `owners.S` is `6cffbd08cd5b7f261b00ca08faa1e0c56bbc0db051ce488d9fc07e7e65fb4ad2`.
SHA-256 for `owners.elf` is `2ec77414b8ec78fdb59afbad6d3f18b2e512478dd463c808965bcfe040330048`.

`micromips.S` is a big-endian MIPS32 dispatcher at 0x400108 and a microMIPS
function `m` at 0x400128 whose symbol carries the mode bit (0x400129) and no
`st_other` marker. Assemble it with LLVM 21.1.8 and drop the linker's
`.comment` section:

```sh
llvm-mc -triple=mips-linux-gnu -mcpu=mips32r2 -filetype=obj -o micromips.o micromips.S
ld.lld -Ttext=0x400100 -e disp -o micromips.linked micromips.o
llvm-objcopy --remove-section=.comment micromips.linked micromips.elf
```

| File or tool | SHA-256 |
| --- | --- |
| micromips.S | daa3acbd357223fc0e69ae22bcd5f55bd8fe44950c8ac4b080072c6b0ef456e4 |
| micromips.elf | 087f6dd5278fb048e60729beb10c83683316882d42ac55c315e8bef86ea3628d |
| llvm-mc | 500b83140384a0b1c997ea0c9caac21557fc5cd6a8d2374d823c46043eb3f6ad |
| ld.lld | 6a65863a9eba1af6b6e8969f8e96a5ad4df0e8b705f98491a28b1790ce35718c |
| llvm-objcopy | 0de5f7adbb648ed88aa04f2d91bd05c08306b99a27741540cc0473cbac0a0b40 |
