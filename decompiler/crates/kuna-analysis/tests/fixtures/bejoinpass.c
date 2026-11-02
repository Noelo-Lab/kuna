typedef unsigned long long u64;
typedef unsigned int u32;
#define NI __attribute__((noinline))
u64 sink;
u64 (*hook)(u64, u64);
void ext(void);
NI u64 add64(u64 a, u64 b) { return a + b; }
void store(u64 a, u64 b) { sink = add64(a, b); }
NI u32 hi(u64 a, u64 b) { return add64(a, b) >> 32; }
NI u64 wrap(u64 a, u64 b) { volatile int x = 0; u64 r = add64(a, b); (void)x; return r; }
NI u64 viahook(u64 a, u64 b) { u64 r = hook(a, b); ext(); return r; }
void usehook(u64 a, u64 b) { sink = viahook(a, b) ^ 5; }
