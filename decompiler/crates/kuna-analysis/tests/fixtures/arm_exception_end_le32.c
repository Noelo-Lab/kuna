/* A32 exports that end in svc or bkpt and then __builtin_unreachable(), each
   followed by a static Thumb function, built into a stripped armel shared
   object (see fixtures/README.md). */
#if PART == 1
int puts(const char *);
int u(int a, int b) __attribute__((visibility("hidden")));
int v(int a, int b) __attribute__((visibility("hidden")));
int api2(int x) { puts("api2"); return u(x, x + 3) * 5 + v(x, 2); }
#elif PART == 2
void my_exit(int c) __attribute__((noreturn));
void my_exit(int c)
{
    register int r0 asm("r0") = c;
    register int r7 asm("r7") = 248;
    asm volatile("svc #0" :: "r"(r0), "r"(r7) : "memory");
    __builtin_unreachable();
}
#elif PART == 3
__attribute__((visibility("hidden"))) int u(int a, int b) { return (a << 3) - (b >> 1) + (a & b); }
#elif PART == 4
int g_code;
void my_trap(int c) __attribute__((noreturn));
void my_trap(int c)
{
    g_code = c;
    asm volatile("bkpt #1" ::: "memory");
    __builtin_unreachable();
}
#elif PART == 5
__attribute__((visibility("hidden"))) int v(int a, int b) { if (a > b) return a - b; return b * 3 + 1; }
#endif
