# RISC-V Zcmp/Zcmt fixtures

Linked shared objects whose `Tag_RISCV_arch` lists `zcmp` (and `zcmt`), so the
loader selects the `RISCV:LE:32:Zcmp` / `RISCV:LE:64:Zcmp` language. Built with
the LLVM 23 `llc` and `ld.lld` (rustc-master toolchain; `.comment` records
LLD 23.1.5). LLVM 21 reproduces every allocated section and
`.riscv.attributes`; only `.comment`, `.symtab` and `.strtab` differ:

```sh
llc -mtriple=riscv32-unknown-elf -mattr=+m,+a,+c,+zcmp -O2 -filetype=obj keep.ll -o keep.o
ld.lld -shared -o keep.elf keep.o
llc -mtriple=riscv32-unknown-elf -mattr=+m,+a,+c,+zcmp,+zcmt -filetype=obj zc_ops.ll -o zc_ops.o
ld.lld -shared -o zc_ops.elf zc_ops.o
llc -mtriple=riscv64-unknown-elf -mattr=+m,+a,+c,+zcmp -filetype=obj zc_ops64.ll -o zc_ops64.o
ld.lld -shared -o zc_ops64.elf zc_ops64.o
```

| File | What |
|---|---|
| `keep.elf` | `keep(a, b) { x = ext(a); ext(b); return ext(x); }` at `-Os`: `cm.push {ra,s0-s1},-16` ... `cm.popret {ra,s0-s1},16` (issue #1630) |
| `zc_ops.elf` | `swapcall` (cm.mvsa01/cm.mva01s around two calls, cm.popret), `zret` (cm.popretz), `popsep` (cm.pop then ret), and `probe`: one row per Zcmp/Zcmt encoding shape, every rlist size class and spimm, cm.jt/cm.jalt, with a `c.swsp`/`c.lwsp`/`ret` tail |
| `zc_ops64.elf` | `swapcall`/`zret`/`popsep` for RV64 (8-byte saves, 32-byte frames) |
