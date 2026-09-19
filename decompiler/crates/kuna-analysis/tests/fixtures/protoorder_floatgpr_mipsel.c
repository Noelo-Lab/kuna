/* protoorder fixture: a float passed in a general register (MIPS o32).
 *
 * o32 passes `x` in a1 after an int first argument, so `h`'s recovered
 * prototype is `float h(int, float)` with the float in a GPR.  Each g* reads a
 * word, hands its bits to h as a float, and also adds, compares or stores the
 * same word as an integer.  A float vote on that value would print the integer
 * arithmetic as a value conversion -- `(int)v1 + 3`, `(int)v1 < 0x3fc00000`,
 * `v1 == 1.5000001` -- so the vote is refused and the word stays `int`.  g20
 * and g22 take a halfword or bytes out of the word (`(short)((unsigned int)v1
 * >> 0x10)` converts a float), and g24 stores it through an `int *`
 * (`q[1] = (int)v1`); those votes are refused too.
 *
 * Build (symbols kept, no DWARF):
 *   mipsel-linux-gnu-gcc -O2 -fno-inline -fno-ipa-ra \
 *       -o protoorder_floatgpr_mipsel protoorder_floatgpr_mipsel.c
 */
#include <string.h>
#define NI __attribute__((noinline))
NI float h(int k, float x) { return x * 2.5f + (float)k; }
NI int g3(int k, int *p) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); return (int)r + bits + 3; }
NI int g5(int k, int *p) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); return (int)r + (bits < 0x3fc00000); }
NI int g6(int k, int *p) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); return (int)r + (bits == 0x3fc00001) * 100; }
NI int g8(int k, int *p) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); p[0] = bits; return (int)r - bits; }
NI unsigned g9(int k, unsigned *p) { unsigned bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); return (unsigned)r + (bits > 0x3fc00000u); }
NI int g20(int k, int *p, short *q) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); *q = (short)(bits >> 16); return (int)r; }
NI int g22(int k, int *p, char *q) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); q[0] = (char)(bits >> 8); q[1] = (char)(bits >> 16); return (int)r; }
NI int g24(int k, int *p, int *q) { int bits = p[3]; float f; memcpy(&f, &bits, 4); float r = h(k, f); q[1] = bits; q[0] = (int)r; return 0; }
int main(int argc, char **argv) {
  int arr[4] = {1,2,3,0x3fc00001}; short s; char c[2]; int q[2];
  return g3(argc, arr) + g5(argc, arr) + g6(argc, arr) + g8(argc, arr) + g9(argc, (unsigned*)arr)
       + g20(argc, arr, &s) + g22(argc, arr, c) + g24(argc, arr, q) + s + c[1] + q[1];
}
