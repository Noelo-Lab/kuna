/* Mach-O test fixture (freestanding — cross-compiles without an SDK/linker).
 * Rebuild:  clang -c --target=x86_64-apple-macos11 -ffreestanding -O0 \
 *             -o sample_macho.o sample_macho.c
 * (produces a Mach-O x86_64 object; kuna resolves x86-64.sla for it. Symbols are
 *  underscore-prefixed per the Mach-O ABI: _compute, _sum_to, _add.)
 */
static int add(int a, int b) { return a + b; }
static long sum_to(int n) { long acc = 0; for (int i = 1; i <= n; i++) acc += i; return acc; }
int compute(int n) { return (int)sum_to(add(n, 3)); }
