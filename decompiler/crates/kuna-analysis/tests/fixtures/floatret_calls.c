/* Floats a function takes or hands back in xmm0 while the other side of the
   call keeps the bits as an integer.
   clang -O0 -o floatret_calls_clang_O0 floatret_calls.c
   gcc -O0 -o floatret_calls_gcc_O0 floatret_calls.c */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))
struct v2 { float x, y; };
float g_out;
NI unsigned f2u(float f) { unsigned u; memcpy(&u, &f, 4); return u; }
NI int signbit_(float f) { return f2u(f) >> 31; }
NI unsigned call_f2u(float f) { return f2u(f + 1.0f); }
NI float pass(float x) { return x; }
NI float fetch(unsigned *p) { p[1] = p[0] + 1; return pass(*(float *)p); }
NI struct v2 mk(float a, float b) { struct v2 r = {a, b}; return r; }
NI float first_of(struct v2 v) { return v.x; }
NI void use(float a, float b) { struct v2 v = mk(a, b); g_out = first_of(v); }
int main(int argc, char **argv) {
  (void)argv;
  unsigned a[2] = {0x3fc00000u, 0};
  float r = fetch(a);
  unsigned rb, o;
  memcpy(&rb, &r, 4);
  use(1.5f * argc, 2.5f);
  memcpy(&o, &g_out, 4);
  printf("%d %x %x %x %x\n", signbit_(-0.5f * argc), call_f2u(0.5f * argc), a[1], rb, o);
  return 0;
}
