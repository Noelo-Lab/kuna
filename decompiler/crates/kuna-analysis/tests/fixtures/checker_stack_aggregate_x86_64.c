typedef unsigned int u32;
typedef unsigned long long u64;

__attribute__((noinline)) void sink(u64 *p) {
    __asm__ volatile("" : : "r"(p) : "memory");
}

__attribute__((noinline)) u32 stack_aggregate(u32 a, u32 b) {
    u64 v12;
    ((u32 *)&v12)[0] = a;
    ((u32 *)&v12)[1] = b;
    sink(&v12);
    return ((u32 *)&v12)[0];
}

void _start(void) {
    (void)stack_aggregate(1, 2);
}
