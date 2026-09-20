/* The counterexample a recovered prototype alone cannot decline: `fwd` names
   rdi and rsi and nothing else, so kuna recovers TWO parameters for it while
   the function it tail-calls through `o->fn` consumes three.  Those two are
   exactly the arguments a drop would leave behind, so the accounting clause is
   satisfied and the forwarded rdx argument goes -- unless the callee's body is
   read too.  At the `jmp *(%rdi)` rdx is unwritten, so the value can still
   reach code no recovery saw and `argclobber` keeps the argument.

   gcc -O2 -fno-inline -nostdlib -static -o argclobber_thunk_x86_64 argclobber_thunk_x86_64.c && strip */
struct pair { long a; long b; };
struct box { long (*fn)(struct box *, long, long); };

__attribute__((noinline)) struct pair mk(long x) { struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) long real3(struct box *o, long a, long b) { (void)o; return a * 7 + b; }

__attribute__((noinline)) long fwd(struct box *o, long a, long b)
{
    if (!a)
        return 0;
    return o->fn(o, a, b);
}

static struct box G = { real3 };

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return fwd(&G, p.a, p.b) + 1;
}

void _start(void)
{
    long r = caller(3, 1000, 7);
    __asm__ volatile("syscall" :: "a"(60), "D"(r));
    __builtin_unreachable();
}
