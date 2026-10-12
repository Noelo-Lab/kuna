/* Float helpers that only work on the bits of a float-register input (option floatbits).
   clang --target=thumbv7em-none-eabihf -mfpu=fpv4-sp-d16 -mfloat-abi=hard -O2 -c floatbitshelper.c -o floatbitshelper_armhf.o
   clang --target=aarch64-linux-gnu -O0 -c floatbitshelper.c -o floatbitshelper_a64_O0.o
   gcc -O2 -c floatbitshelper.c -o floatbitshelper_x86_64_O2.o
   gcc -O2 -DFB_MAIN -o floatbitshelper_x86_64_gcc_O2 floatbitshelper.c && strip floatbitshelper_x86_64_gcc_O2 */
typedef unsigned int u32;
typedef unsigned long long u64;
typedef union { float f; u32 u; } fu;
typedef union { double d; u64 u; } du;
#define NI __attribute__((noinline))
NI float fb_fabsf(float x) { fu v = { x }; v.u &= 0x7fffffffu; return v.f; }
NI float fb_negf(float x) { fu v = { x }; v.u ^= 0x80000000u; return v.f; }
NI float fb_copysignf(float x, float y) { fu a = { x }, b = { y }; a.u = (a.u & 0x7fffffffu) | (b.u & 0x80000000u); return a.f; }
NI int fb_isnanf(float x) { fu v = { x }; return (v.u & 0x7fffffffu) > 0x7f800000u; }
NI double fb_fabs(double x) { du v = { x }; v.u &= 0x7fffffffffffffffull; return v.d; }
NI u32 fb_bump(float x) { fu v = { x }; return v.u + 1; }
NI void fb_store(float x, u32 *p) { fu v = { x }; *p = v.u & 0x7fffffffu; }
NI float fb_mix(float x, u32 k) { fu v = { x }; v.u = (v.u & 0x7fffffffu) | k; return v.f; }
NI float fb_use(float a, float b) { return fb_fabsf(a - b) * 2.0f + fb_negf(b) + fb_copysignf(3.0f, a) + (float)fb_isnanf(b); }
NI double fb_used(double a) { return fb_fabs(a) * 3.0; }
NI void fb_keep(float x, u32 *p) { fu v = { fb_fabsf(x) }; *p = v.u + 1; }

#ifdef FB_MAIN
#include <stdio.h>
#include <string.h>
static u32 fbits(float f) { u32 u; memcpy(&u, &f, 4); return u; }
static u64 dbits(double d) { u64 u; memcpy(&u, &d, 8); return u; }
int main(int argc, char **argv) {
  volatile float a = argc > 5 ? 0.0f : -1.5f, b = argc > 5 ? 1.0f : 2.25f;
  volatile double d = argc > 5 ? 0.0 : -4.5;
  u32 s = 0, k = 0;
  fb_store(a, &s);
  fb_keep(a, &k);
  printf("%x %x %x %d %llx %x %x %x %x %llx %x\n", fbits(fb_fabsf(a)), fbits(fb_negf(a)), fbits(fb_copysignf(b, a)),
         fb_isnanf(a), dbits(fb_fabs(d)), fb_bump(a), s, fbits(fb_mix(a, 0x80000000u)), fbits(fb_use(a, b)), dbits(fb_used(d)), k);
  return 0;
}
#endif
