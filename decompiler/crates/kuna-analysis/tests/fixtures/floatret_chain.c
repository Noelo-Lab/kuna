/* A float return handed on through wrappers to a caller that keeps the bits as
   an integer: a double through one wrapper and through two, a float through
   one, and a struct of two floats through one.
   gcc -O1 -o floatret_chain_gcc_O1 floatret_chain.c && strip floatret_chain_gcc_O1
   mipsel-linux-gnu-gcc -O0 -o floatret_chain_mips_O0 floatret_chain.c && mipsel-linux-gnu-strip floatret_chain_mips_O0 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))
struct v2 { float x, y; };
double darr[4] = {1.5, -0.0, 2.25, 3.0};
float farr[4] = {1.5f, -0.0f, 2.25f, 3.0f};
struct v2 parr[4] = {{1.0f, 2.0f}, {3.0f, 4.0f}, {5.0f, 6.0f}, {7.0f, 8.0f}};
unsigned long long sink[8];
NI double getd(const double *p, int i) { return p[i]; }
NI double wrapd(const double *p, int i) { return getd(p, i); }
NI double wrap2(const double *p, int i) { return wrapd(p, i); }
NI float getf(const float *p, int i) { return p[i]; }
NI float wrapf(const float *p, int i) { return getf(p, i); }
NI struct v2 getp(const struct v2 *p, int i) { return p[i]; }
NI struct v2 wrapp(const struct v2 *p, int i) { return getp(p, i); }
NI void rd(const double *p, int i) { double d = wrapd(p, i); unsigned long long u; memcpy(&u, &d, 8); sink[0] = u; sink[1] = u >> 32; }
NI void rd2(const double *p, int i) { double d = wrap2(p, i); unsigned long long u; memcpy(&u, &d, 8); sink[2] = u; sink[3] = u >> 32; }
NI void rdf(const float *p, int i) { float f = wrapf(p, i); unsigned u; memcpy(&u, &f, 4); sink[4] = u; sink[5] = u >> 16; }
NI void rdp(const struct v2 *p, int i) { struct v2 v = wrapp(p, i); unsigned long long u; memcpy(&u, &v, 8); sink[6] = u; sink[7] = u >> 32; }
int main(int argc, char **argv) {
  (void)argv;
  rd(darr, argc + 1);
  rd2(darr, argc + 2);
  rdf(farr, argc + 1);
  rdp(parr, argc);
  printf("%llx %llx %llx %llx %llx %llx %llx %llx\n", sink[0], sink[1], sink[2], sink[3], sink[4], sink[5], sink[6], sink[7]);
  return 0;
}
