/* The eighth counterexample: the sixth one, one call frame deeper.

   `wrap` forwards its three registers to `fwd2` with a plain direct call, and
   `fwd2` is the tail-calling thunk of ce-forward-thunk-2param.c.  Both recover
   two parameters -- `fwd2` because it names only rdi and rsi, `wrap` because
   what it calls states only those two -- so the accounting is satisfied at
   `wrap` and the forwarded rdx argument is deleted although real3 multiplies
   by it.

   Naming `fwd2` is not accounting for it.  What declines the drop is the same
   question asked of `fwd2`'s body in turn: at its `jmp *(%rdi)` rdx is still
   the caller's, so the value can reach code no recovery saw.

   Build: gcc -O2 -fno-optimize-sibling-calls -o ce-forward-thunk-2frame \
            ce-forward-thunk-2frame.c && strip */
struct pair { long a; long b; };
struct box { long (*fn)(struct box *, long, long); };

__attribute__((noinline)) static struct pair mk(long x)
{ struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) static long real3(struct box *o, long a, long b)
{ (void)o; return a * 7 + b; }

__attribute__((noinline)) static long fwd2(struct box *o, long a, long b)
{ if (!a) return 0; return o->fn(o, a, b); }

__attribute__((noinline)) long wrap(struct box *o, long a, long b)
{ if (!a) return 0; return fwd2(o, a, b); }

static struct box G;

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return wrap(&G, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{ G.fn = real3; return (int)caller(argc & 7, (long)argv[0][0], argc + 1); }
