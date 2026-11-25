# PowerPC64 ELFv2 PLT call stubs

`calls.c` holds two PIC wrappers: `release` passes its pointer to `free` and
`measure` returns `strlen(s) + 1`. `provider.c` stands in for the C library at
link time. Each image links the same wrappers with a different linker or stub
form, so the stub each call reaches loads its import's `.plt` slot in a
different way:

| Image | Linker | Stub that loads the slot |
| --- | --- | --- |
| `toc.elf` | GNU ld | `std r2,24(r1); ld r12,lo(r2); mtctr r12; bctr` |
| `localentry.elf` | GNU ld `--plt-localentry` | `ld r12,lo(r2); mtctr r12; bctr` |
| `notoc.elf` | GNU ld, Power10 pc-relative callers | `mflr r12; bcl 20,31,.+4; mflr r11; mtlr r12; addis r12,r11,ha; ld r12,lo(r12); mtctr r12; bctr` |
| `pld.elf` | GNU ld `--power10-stubs=yes` | `pld r12,off; mtctr r12; bctr` |
| `lld-toc.elf` | lld | `std r2,24(r1); addis r12,r2,ha; ld r12,lo(r12); mtctr r12; bctr` |
| `lld-pld.elf` | lld, Power10 callers, no `.got` | `pld r12,off; mtctr r12; bctr` |
| `be.elf` | GNU ld, big-endian ELFv2 | as `toc.elf` |

`guards.s` checks what a stub is not. `release` is an exported function shaped
like a Power10 stub that loads `free`'s PLT slot. `hidden` has the same shape
but loads `strlen`'s GOT entry. `outer` calls `free` through the linker's stub,
then `release` and `hidden` directly.

The stub addresses are the stub symbols the linkers write for the same links
(GNU ld `--emit-stub-syms`, lld `__plt_*`); `strip` removes them. Rebuild in
this directory with clang 21.1.8, ld.lld 21.1.8, GNU ld 2.47.20260726, GNU as
2.46 for powerpc64le (`${AS}`) and llvm-strip 21.1.8:

```sh
cc() { clang -target "$1" -mcpu="$2" -O2 -fPIC -fno-ident -ffreestanding -c "$3" -o "$4"; }
link() { ld -m "$1" -shared --build-id=none -z max-page-size=0x1000 "${@:2}"; }
lld() { ld.lld -shared --build-id=none -z max-page-size=0x1000 "$@"; }
mkdir -p le be
cc powerpc64le-linux-gnu pwr8 provider.c le/provider.o
link elf64lppc -soname libc.so.6 -o le/libc.so.6 le/provider.o
cc powerpc64le-linux-gnu pwr8 calls.c toc.o
cc powerpc64le-linux-gnu pwr10 calls.c pcrel.o
${AS} -mpower10 -o guards.o guards.s
link elf64lppc -o toc.so toc.o le/libc.so.6
link elf64lppc --plt-localentry -o localentry.so toc.o le/libc.so.6
link elf64lppc -o notoc.so pcrel.o le/libc.so.6
link elf64lppc --power10-stubs=yes -o pld.so pcrel.o le/libc.so.6
link elf64lppc -o guards.so guards.o le/libc.so.6
lld -o lld-toc.so toc.o le/libc.so.6
lld -o lld-pld.so pcrel.o le/libc.so.6
clang -target powerpc64-linux-gnu -mabi=elfv2 -mcpu=pwr8 -O2 -fPIC -fno-ident -ffreestanding -c provider.c -o be/provider.o
link elf64ppc -soname libc.so.6 -o be/libc.so.6 be/provider.o
clang -target powerpc64-linux-gnu -mabi=elfv2 -mcpu=pwr8 -O2 -fPIC -fno-ident -ffreestanding -c calls.c -o be.o
link elf64ppc -o be.so be.o be/libc.so.6
for f in toc localentry notoc pld guards lld-toc lld-pld be; do llvm-strip --strip-all -o "$f.elf" "$f.so"; done
```

| File or tool | SHA-256 |
| --- | --- |
| calls.c | e7fbc74022b6579cdf63cbf6f7dac668648f31197b9fa058d4a007128d7cc97c |
| provider.c | 4c2f7619339ce6c9d8b59f5db144ef797dcbe0b93924d15015116a97bc1ffd01 |
| guards.s | e9b79e73c59a37cce54b3f4dffb7398d20dd09554a4685140ecd0e26ef4de273 |
| toc.elf | 00219b1e803e8c70a6151121e1946bbd48002a5c036705cf111f3228978e04b5 |
| localentry.elf | 335abd8c71135e2e47ca93eb4b99062dd6f8310a5ce21f696843cbe0b9cced35 |
| notoc.elf | 477aa1b2aea5750ed060ffb0b22fcf654796b194fd2b3d91dcbd38550a64190b |
| pld.elf | 6d661f4725de70ddfc66c056dcd27d374dc4436a1a0a5bebdf81878e6770e155 |
| lld-toc.elf | 97c0d0a65a61bf168ca062ed26d4e19d3c18e36de6bbe00e153acc394e57b719 |
| lld-pld.elf | 110e8ed153ca9450f2ae52bb84131b4c11ba8866ea900d6ec985bb4dea6a7ed2 |
| be.elf | d00d55da7165043860c4dca8d916e59a0cc1029fa0f2810cd504e96e2f5e62bd |
| guards.elf | cbd0ae8d4f5b93f97577a48eb39db855ceb7bed90bfa5ef7680b312dec9498f2 |
| clang | 412bbe8c60571a1eb06f48fde89635033621caeb01a9b4ee76d46711bae8e932 |
| ld.lld | 6a65863a9eba1af6b6e8969f8e96a5ad4df0e8b705f98491a28b1790ce35718c |
| ld | 2865846ece2642439cfae404ef930bfc5494c4babd2347757878b8f0f8f761cf |
| ${AS} | 5f576d51b3c5b7f0e9ea93d740b6a60a260a4826343441b7692e662a7a33fb84 |
| llvm-strip | 0de5f7adbb648ed88aa04f2d91bd05c08306b99a27741540cc0473cbac0a0b40 |
