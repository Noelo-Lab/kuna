/* macho_imports.c — a tiny program whose `main` calls `printf` (an external
 * dylib import).  Linked into a Mach-O it produces a __stubs / __la_symbol_ptr
 * entry + an indirect-symbol-table entry for `_printf`.  Built freestanding so
 * no macOS SDK / crt is needed; `-undefined dynamic_lookup` lets `_printf` stay
 * an external Mach-O undefined symbol. */
int printf(const char *fmt, ...);

int compute(int n) {
    return n * 3 + 7;
}

int main(int argc, char **argv) {
    printf("%d\n", compute(argc));
    return 0;
}
