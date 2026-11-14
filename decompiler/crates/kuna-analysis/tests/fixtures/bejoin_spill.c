typedef long long i64;
typedef unsigned long long u64;
void ext(void);
i64 widen(int x) { ext(); return x; }
u64 uwiden(unsigned x) { ext(); return x; }
i64 local(int x) { i64 t = x; ext(); return t; }
int local_hi(int x) { i64 t = x; ext(); return t >> 32; }
int sign(int x) { ext(); return x >> 31; }
