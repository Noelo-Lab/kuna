/* A float bit helper whose float return a reader withdraws (option floatbits).
   clang --target=thumbv7em-none-eabihf -mfpu=fpv4-sp-d16 -mfloat-abi=hard -O2 -c floatbitswithdraw.c -o floatbitswithdraw_armhf.o */
typedef union { float f; unsigned u; } fu;
__attribute__((noinline)) float fbabs(float x) { fu v = { x }; v.u &= 0x7fffffffu; return v.f; }
__attribute__((noinline)) float from_bits(unsigned u) { fu v; v.u = u; return fbabs(v.f); }
__attribute__((noinline)) unsigned abs_bits(unsigned u) { fu v; v.u = u; fu r = { fbabs(v.f) }; return r.u; }
