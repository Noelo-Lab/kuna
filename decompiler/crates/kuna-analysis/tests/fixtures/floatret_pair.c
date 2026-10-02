/* A pair of floats a function returns in xmm0 -- a struct of two floats, the
   first two of three, a float _Complex -- while its caller keeps the eight
   bytes as an integer.
   gcc -O2 -o floatret_pair_gcc_O2 floatret_pair.c && strip floatret_pair_gcc_O2 */
#include <stdio.h>
#include <string.h>
#include <complex.h>
#define NI __attribute__((noinline))
struct v2 { float x, y; };
struct v3 { float x, y, z; };
unsigned long long sink[8];
NI struct v2 k2(void) { struct v2 r = {1.0f, 2.0f}; return r; }
NI struct v3 k3(void) { struct v3 r = {1.0f, 2.0f, 3.0f}; return r; }
NI float _Complex kc(void) { return 1.0f + 2.0f * I; }
NI void rd(int i) { struct v2 v = k2(); unsigned long long u; memcpy(&u, &v, 8); sink[0] = u; sink[1] = u >> 32; sink[2] = i; }
NI void rd3(void) { struct v3 v = k3(); unsigned long long u; memcpy(&u, &v, 8); sink[3] = u; sink[4] = u >> 32; }
NI void rdc(void) { float _Complex z = kc(); unsigned long long u; memcpy(&u, &z, 8); sink[5] = u; sink[6] = u >> 32; }
int main(int argc, char **argv) {
  (void)argv;
  rd(argc);
  rd3();
  rdc();
  printf("%llx %llx %llx\n", sink[0], sink[1], sink[2]);
  printf("%llx %llx %llx %llx\n", sink[3], sink[4], sink[5], sink[6]);
  return 0;
}
