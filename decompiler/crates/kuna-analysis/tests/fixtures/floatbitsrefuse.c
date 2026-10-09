/* Bit tests floatbits must leave integer, and the ones it keeps (option floatbits).
   clang --target=thumbv7em-none-eabihf -mfpu=fpv4-sp-d16 -mfloat-abi=hard -O2 -c floatbitsrefuse.c -o floatbitsrefuse_armhf.o
   gcc -O2 -c floatbitsrefuse.c -o floatbitsrefuse_x86_64_O2.o
   clang --target=aarch64-linux-gnu -O2 -c floatbitsrefuse.c -o floatbitsrefuse_a64_O2.o */
typedef unsigned int u32;
typedef union { float f; u32 u; } fu;
#define NI __attribute__((noinline))
NI int fr_isqnan0(float x) { fu v = { x }; return (v.u & 0x7fffffffu) == 0x7fc00000u; }
NI int fr_iszero(float x) { fu v = { x }; return v.u == 0; }
NI int fr_eqbits(float a, float b) { fu x = { a }, y = { b }; return x.u == y.u; }
NI int fr_ne0(float x) { fu v = { x }; return v.u != 0; }
NI int fr_isinf(float x) { fu v = { x }; return (v.u & 0x7fffffffu) == 0x7f800000u; }
NI int fr_isone(float x) { fu v = { x }; return v.u == 0x3f800000u; }
NI float fr_sel(float x, int c) { fu v = { x }; v.u &= 0x7fffffffu; if (c) v.u = 0x7fc00001u; return v.f; }
NI float fr_unset(float x, int ok) { fu v = { x }; v.u &= 0x7fffffffu; if (!ok) v.u = 0xffffffffu; return v.f; }
NI float fr_canon(float x) { fu v = { x }; if ((v.u & 0x7fffffffu) > 0x7f800000u) v.u = 0x7fc00000u; return v.f; }
NI float fr_copysignf(float x, float y) { return __builtin_copysignf(x, y); }
NI int fr_tenth(float x) { fu v = { x }; return v.u == 0x3dcccccdu; }
NI int fr_fmax(float x) { fu v = { x }; return v.u == 0x7f7fffffu; }
NI int fr_fmin(float x) { fu v = { x }; return v.u == 0x00800000u; }
NI int fr_onep(float x) { fu v = { x }; return v.u != 0x3f800001u; }
NI int fr_third(float x) { fu v = { x }; return (v.u & 0x7fffffffu) == 0x3eaaaaabu; }
NI int fr_big(float x) { fu v = { x }; return v.u == 0x4b800001u; }
#ifdef __x86_64__
typedef union { _Float128 f; unsigned long long w[2]; } qu;
NI int fr_lowzero(_Float128 x) { qu u = { x }; return u.w[0] == 0; }
typedef int v2si __attribute__((vector_size(8)));
NI v2si fr_andv(v2si a) { return a & (v2si){0x7fffffff, 0x7fffffff}; }
typedef struct { float x, y; } V2;
NI V2 fr_negy(V2 v) { fu u = { v.y }; u.u ^= 0x80000000u; v.y = u.f; return v; }
typedef union { double d; unsigned long long u; } du;
NI double fr_lowflip(_Float128 x) { qu u = { x }; du d; d.u = u.w[0] ^ 0x8000000000000000ull; return d.d; }
#endif
