# Nullable relocations

Original C and x86-64 assembly, with sources and compiler/tool identities in
`provenance.json`. No vendor implementation is included. ELF files are relocatable
objects. `weak.c` covers null guards, imported calls, argument preservation, and
defined-symbol controls. `address.c` returns weak addresses with zero/positive/
negative addends, and an unrelated fixed literal equal to a synthetic extern
address. `rip.S` covers x86 RIP-relative address fields and, in
`register_offset`, an absolute displacement on a register. `readonly.S` and
`readonly-mips.S` check full-width read-only weak pointer fields and their
addends. Mixed N32/N64 objects combine PIC and ordinary address spellings of the
same symbol. `common.c` puts two COMMON objects and direct calls beside the weak
function, and compares an argument with its address through one immediate.
`weak-x86-64-pic.elf` is `weak.c` built `-fPIC`: its `GOTPCREL` operands are not
modeled and must diagnose.

Build with `build.py --cc ... --mips-cc ... --linker ... --artifacts ...`.
The CLI regression replays emitted C at O0/O2 with UBSan. Guard inputs are in
[-5000,4999], with zero and bound imports and exact call/argument counts. Address
bit patterns include signed-low carry boundaries and full 64-bit high bits.
The x86 small-model address object has negative R_X86_64_32 addends, so its
address-replay bindings keep every relocated field within its representable
range. The guard object separately exercises the unbound zero case. The large-model
guard declares STT_FUNC explicitly because its absolute relocations alone do
not distinguish functions from data; the ordinary forms retain STT_NOTYPE and
use actual code-relocation evidence.

Original-machine link/execution qualification is separate from emitted-C replay;
unsupported relocation/operand forms must diagnose and are not semantic passes.
