/* (kuna) A zero-extended 16-bit field handed to a 32-bit parameter, read
 * through a pointer whose other uses type it `unsigned int *`.  `RuleExpandLoad`
 * used to widen `movzwl 0x68(%rdi)` into a 4-byte read and print the truncation
 * as `(short)a0[0x1a]`, which C sign-extends: 0x9abc arrived as 4294941372.
 * The round-trip test in kuna-cli/tests/decompile_all_cli.rs compiles `f` as
 * kuna prints it and checks the value `sink` receives.
 *
 * Built with:  gcc -O2 -o expandload_zext_x86_64 expandload_zext_x86_64.c
 */
#include <stdio.h>
#define NI __attribute__((noinline))
struct S { unsigned int w[26]; unsigned short h; unsigned short h2; };
unsigned acc;
NI void sink(unsigned int x) { acc = acc * 31u + x / 3u; }
NI unsigned f(struct S *s) { unsigned r = s->w[0] / 5u + (s->w[1] >> 3); sink(s->h); return r; }
int main(void) {
  static struct S st;
  st.w[0] = 1; st.w[1] = 2; st.h = 0x9abc;
  unsigned r = f(&st);
  printf("%u %u\n", r, acc);
  return 0;
}
