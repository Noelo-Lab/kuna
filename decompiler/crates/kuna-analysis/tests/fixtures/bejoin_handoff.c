typedef long long i64;
typedef unsigned long long u64;
u64 sink;
__attribute__((noinline)) u64 add64(u64 a, u64 b) { return a + b; }
double to_dbl(u64 a, u64 b) { return (double)add64(a, b); }
__attribute__((noinline)) u64 sum(u64 a, u64 b) { return a + b; }
void store(u64 a, u64 b) { sink = sum(a, b); }
__attribute__((noinline)) i64 wrap(u64 a, u64 b) { return sum(a, b); }
int sign_of(u64 a, u64 b) { if (wrap(a, b) < 0) return 1; return 2; }
