/* armfloatreturn and armfloatargs as members of the aggressive preset
   (tests/stages/kuna-armfloat-preset.xml).
   clang --target=armv7a-linux-gnueabihf -mfloat-abi=hard -mfpu=vfpv3-d16 -O2 -c armfloat_preset.c -o armfloat_preset_hard.o
   clang --target=armv7a-linux-gnueabi -mfloat-abi=soft -O2 -c armfloat_preset.c -o armfloat_preset_soft.o */
#define NI __attribute__((noinline))
typedef union { float f; unsigned u; } fu;
NI double scale(double x) { return x * 1.5; }
NI double mix(int i, double x, float y) { return i * x + y; }
NI float backfill(float a, double b, float c) { return a + (float)b * c; }
NI float copysign_bits(float x, float y) { fu a = { x }, b = { y }; a.u = (a.u & 0x7fffffffu) | (b.u & 0x80000000u); return a.f; }
NI float payload(void) { return __builtin_nanf("0x123"); }
NI float inner(float y, float x) { if (y != y) return y; return y > x ? y - x : x * y; }
NI float wrap(float y, float x) { return inner(y, x); }
NI float use_wrap(float a, float b) { return wrap(a + 1.0f, b) * 2.0f; }
volatile int err_;
NI float oflow(int s) { err_ = 34; return s ? -1e30f * 1e30f : 1e30f * 1e30f; }
NI float scale_err(float x) { if (x > 88.0f) return oflow(0); double d = (double)x * 1.4426950408889634; return (float)(d * d + 1.0); }
NI float from_bits(unsigned u) { fu a; a.u = u; return a.f; }
NI __attribute__((optnone)) float from_bits_o0(unsigned u) { fu a; a.u = u; return a.f; }
NI void set8f(float *p, float a, float b, float c, float d, float e, float f, float g, float h) { p[0] = a; p[1] = b; p[2] = c; p[3] = d; p[4] = e; p[5] = f; p[6] = g; p[7] = h; }
#ifdef __ARM_PCS_VFP
__attribute__((naked, noinline)) void save_vfp(double *p) { __asm__("vstmia r0, {d0-d15}\n\tbx lr"); }
/* gcc 11 -O2's shape for an int getter that converts a double field on one path. */
__attribute__((naked, noinline)) int getpar(void *mp, int key, int *val, double *fval) {
  __asm__("cmp r0, #0\n\tbeq 3f\n\tcmp r1, #2\n\tbne 2f\n\tcmp r3, #0\n\tbeq 1f\n\tvldr d7, [r0, #8]\n\tvstr d7, [r3]\n"
          "1:\tcmp r2, #0\n\tbeq 4f\n\tvldr d0, [r0, #8]\n\tmov r0, #0\n\tvldr d6, 5f\n\tvmul.f64 d7, d0, d6\n\tvcvt.s32.f64 s1, d7\n\tvstr s1, [r2]\n\tbx lr\n"
          "4:\tmov r0, #0\n\tbx lr\n2:\tmov r0, #5\n\tbx lr\n3:\tmov r0, #25\n\tbx lr\n\t.p2align 3\n5:\t.double 32768.0\n");
}
#endif
