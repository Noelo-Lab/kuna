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
