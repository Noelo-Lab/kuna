/* GH-657 verbatim.
 *
 *   gcc -O2 -o /tmp/p1 repro.c && /tmp/p1; echo $?     # 50
 *   kuna decompile /tmp/p1 main
 *
 * Before: `k = 0x2a; return f(7) + 0x2a;`  (50 in the binary, 91 in that C)
 * After:  `v1 = f(7); k = 0x2a; return v1 + 0x2a;`
 */
int k = 1;
__attribute__((noinline)) int f(int x) { return x + k; }
int main(void) { int t = f(7); k = 42; return t + k; }
