// Fixture source for the Mach-O DWARF gate (s1_dwarf on Mach-O) — the Mach-O
// analog of the ELF `dwarf_stripped_x86_64`: the function NAMES live ONLY in the
// `__DWARF,__debug_*` sections. (No system headers — cross-compiling to macOS
// without an SDK; the `char *` parameter is all the gate needs.)
//
// Build recipe (clang on Linux can cross-emit a Mach-O object; the DWARF lands in
// the `__DWARF,__debug_*` sections of the relocatable object):
//
//   clang -target x86_64-apple-macos11 -g -O0 -c macho_dwarf.c -o macho_dwarf.o
//   llvm-objcopy --redefine-sym _first_byte=_l0 --redefine-sym _add=_l1 macho_dwarf.o
//
// A Mach-O object with `SUBSECTIONS_VIA_SYMBOLS` will not let strip remove its
// function symbols (they delimit subsections), so instead `--redefine-sym`
// RENAMES the symtab FUNC entries (`_first_byte`→`_l0`, `_add`→`_l1`). DWARF still
// names them `first_byte`/`add`, so those names are recoverable BY NAME **only**
// via the s1_dwarf pass reading `__DWARF,__debug_info`. With the `__DWARF` sections
// removed (the no-DWARF baseline) those names do not resolve at all.
//
// `first_byte` additionally takes a **`char *` parameter**, so the recovered
// signature carries a DWARF-typed `char *` (mirrors the ELF `cet_pie` gate).

int first_byte(char *label) {
    return label[0];
}

int add(int a, int b) {
    return a + b;
}

int main(void) {
    return first_byte("kuna") + add(2, 3);
}
