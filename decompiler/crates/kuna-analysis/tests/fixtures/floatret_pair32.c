/* A 64-bit integer returned in two registers on a 32-bit target, handed on by
   functions whose callers read both registers: `hi` sets the high register
   after its call, `wrap` is `call; ret`.
   clang --target=i386-linux-gnu -O2 -c -o floatret_pair32_i386_O2.o floatret_pair32.c
   clang --target=arm-linux-gnueabi -O0 -c -o floatret_pair32_arm_O0.o floatret_pair32.c */
typedef unsigned long long u64;
#define NI __attribute__((noinline))
NI u64 zsum(unsigned a, unsigned b) { return a + b; }
NI u64 hi(unsigned a, unsigned b) { return zsum(a, b) | 0xff00000000ULL; }
NI u64 ins16(unsigned a, unsigned short b) { return ((a + 1) << 16) | b; }
NI u64 wrap(unsigned a, unsigned short b) { return ins16(a, b); }
u64 sink[2];
NI void use(unsigned a, unsigned b) { sink[0] = hi(a, b); sink[1] = wrap(a, (unsigned short)b); }
