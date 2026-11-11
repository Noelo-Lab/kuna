/* Big-endian two-register returns the callee alone cannot prove: a 64-bit
 * sum and a 64-bit product compile to the bytes of the int that returns
 * their high word, so only their callers tell them apart. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
u64 sinku;
u32 sink32;
NI void ext(void) { __asm__ volatile(""); }
NI u64 add64(u64 a, u64 b) { return a + b; }
NI u32 add_hi(u64 a, u64 b) { return (a + b) >> 32; }
NI u64 umul64(u32 a, u32 b) { return (u64)a * b; }
NI u32 umul_hi(u32 a, u32 b) { return ((u64)a * b) >> 32; }
NI int keep(int a, int b) { return a * 3 + 1; }
NI u64 pass(u64 a, u64 b) { volatile int x = 0; u64 r = add64(a, b); (void)x; return r; }
void c_add(u64 a, u64 b) { sinku = add64(a, b); }
void c_hi(u64 a, u64 b) { sink32 = add_hi(a, b); }
void c_mul(u32 a, u32 b) { sinku = umul64(a, b); }
void c_mulhi(u32 a, u32 b) { sink32 = umul_hi(a, b); }
int c_keep(int a, int b) { return keep(a, b) + b; }
void c_pass(u64 a, u64 b) { sinku = pass(a, b); }
