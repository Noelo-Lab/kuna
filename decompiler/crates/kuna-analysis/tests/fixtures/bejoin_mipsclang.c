/* option bejoin callers: a pair whose low word also feeds the high word (GH-904). */
typedef unsigned long long u64;
typedef long long i64;
#define NI __attribute__((noinline))
u64 sink64;
unsigned sink32;
NI u64 add64(u64 a, u64 b) { return a + b; }
NI unsigned add_hi(u64 a, u64 b) { return (a + b) >> 32; }
NI i64 widen1(int x) { return x + 1; }
void c_add(u64 a, u64 b) { sink64 = add64(a, b); }
void c_hi(u64 a, u64 b) { sink32 = add_hi(a, b); }
void c_widen(int x) { sink64 = widen1(x); }
