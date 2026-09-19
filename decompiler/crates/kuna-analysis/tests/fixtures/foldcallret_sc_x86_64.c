/* (kuna GH-684) A call whose result is combined with a comparison by a
 * non-short-circuit `&`: the binary always makes the call, and C's `&&`/`||`
 * skip their right-hand operand.  `foldcallret` used to fold `tick` into the
 * right-hand operand, so the printed C only called it when `a > 5`.  The
 * round-trip test in kuna-cli/tests/decompile_all_cli.rs compiles `w1f` and
 * `w4f` as kuna prints them and counts the calls.
 *
 * Built with:
 *   gcc   -O0 -o foldcallret_sc_gcc_O0_x86_64   foldcallret_sc_x86_64.c
 *   clang -O0 -o foldcallret_sc_clang_O0_x86_64 foldcallret_sc_x86_64.c
 *   clang -O2 -o foldcallret_sc_clang_O2_x86_64 foldcallret_sc_x86_64.c
 * Each prints "2 0".
 */
#include <stdio.h>
#define NI __attribute__((noinline))
int calls;
int gflag;
NI int tick(int x) { calls++; return x - 3; }
NI void w1f(int a) { int r = tick(a); if ((r == 0) & (a > 5)) puts("A"); }
NI void w4f(int a) { int r = tick(a); gflag = (r == 0) & (a > 5); }
int main(int argc, char **argv) { w1f(argc); w4f(argc); printf("%d %d\n", calls, gflag); return 0; }
