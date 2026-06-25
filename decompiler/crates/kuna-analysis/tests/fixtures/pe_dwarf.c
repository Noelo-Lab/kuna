// Fixture source for the MinGW-PE DWARF gate (s1_dwarf on PE) — the PE analog of
// the ELF `dwarf_stripped_x86_64`: the function NAMES live ONLY in `.debug_*`.
//
// Build recipe (in the `kuna-dev` container; MinGW `-g` emits standard `.debug_*`
// DWARF sections in the PE):
//
//   x86_64-w64-mingw32-gcc -g -O0 pe_dwarf.c -o pe_g.exe
//   x86_64-w64-mingw32-objcopy --strip-symbol first_byte --strip-symbol add \
//       --strip-symbol main  pe_g.exe  pe_dwarf.exe
//
// The `--strip-symbol` pass removes the COFF-symtab FUNC entries while keeping the
// `.debug_*` sections, so `first_byte`/`add` are recoverable BY NAME **only** via
// DWARF (the s1_dwarf pass's `SymFact{Function}`). With `.debug_*` also stripped
// (the no-DWARF baseline) those names do not resolve at all — the clean
// before/after the e2e asserts.
//
// `first_byte` additionally takes a **`char *` parameter**, so the recovered
// signature carries a DWARF-typed `char *` (this mirrors the ELF `cet_pie`
// `char *binary` typed-signature gate).

int first_byte(char *label) {
    return label[0];
}

int add(int a, int b) {
    return a + b;
}

int main(void) {
    return first_byte("kuna") + add(2, 3);
}
