typedef long long i64;
typedef unsigned long long u64;
typedef unsigned u32;
u32 gsinku;
int hi_local(const i64 *p) { i64 v = *p; int hi = v >> 32; int lo = (int)v; (void)lo; return hi; }
u32 hi_lo_stored(const i64 *p) { i64 v = *p; u32 lo = (u32)v; u32 hi = (u32)(v >> 32); gsinku = lo; return hi; }
u32 hi_named(const u64 *p) { u64 v = *p; u32 h = v >> 32; return h; }
int hi_use(const i64 *p) { return hi_local(p) + 1; }
i64 ll_copy(const i64 *p) { i64 v = *p; return v; }
i64 ll_kept(const i64 *p) { i64 r = ll_copy(p); return r; }
i64 ll_passed(const i64 *p) { return ll_copy(p); }
int hi_kept(const i64 *p) { i64 r = ll_copy(p); int h = r >> 32; return h; }
u64 __atomic_load_8(const volatile void *p, int m) { (void)m; return *(const volatile u64 *)p; }
u32 hi_atomic(u64 *p) { return __atomic_load_n(p, __ATOMIC_RELAXED) >> 32; }
u64 ll_atomic(u64 *p) { return __atomic_load_n(p, __ATOMIC_RELAXED); }
