/* Fixture for data-global naming from DWARF (kuna Feature #1, matching IDA/Ghidra).
 *
 * Multi-byte file-scope globals with DWARF `DW_TAG_variable` + `DW_OP_addr`
 * locations. Before the fix, kuna installed each data global with a size-1 code
 * type, so a 4-/8-byte memory access queried `queryContainer(addr, 4|8)` and
 * found no covering SymbolEntry -- the global rendered `dat_<addr>`. The fix maps
 * each DWARF global with an `undefined<size>` type (size from `DW_AT_type`), so
 * the container query matches and the ActionNameVars global-scope query binds the
 * name (`g_counter`/`g_total`/`g_name`). `uniform`-style 1-byte globals already
 * worked; these exercise the 4-/8-byte path.
 *
 * `bump` is `noinline` so the read-modify-write of the globals survives to a real
 * memory access in a callee (not folded into main).
 *
 * Build (DWARF required -> -g):
 *   gcc -O2 -g -fno-stack-protector -fcf-protection=none \
 *       -o dwarf_globals_x86_64 dwarf_globals_x86_64.c
 */

int g_counter;   /* 4-byte int  global (.bss)  */
long g_total;    /* 8-byte long global (.bss)  */
char *g_name;    /* 8-byte ptr  global (.bss)  */

__attribute__((noinline)) int bump(int n)
{
	g_counter += n;
	g_total += n;
	return g_counter;
}

int main(int argc, char **argv)
{
	g_name = argv[argc - 1];
	return bump(argc) + (int)g_total + (g_name != 0);
}
