/* ce-forward-thunk-2frame.c with the forwarding frame made a two-member
   cycle: `wrapa` and `wrapb` call each other, and `wrapa` also hands its
   registers to the tail-calling thunk `fwd2`.  Under `protoorder cycles` both
   members state their recovered lists; what must still decline the drop of
   `caller`'s rdx argument is the question asked of their bodies in turn.

   Build: gcc -O2 -fno-optimize-sibling-calls -o ce-rec-mutual ce-rec-mutual.c && strip */
struct pair { long a; long b; };
struct box { long (*fn)(struct box *, long, long); };

__attribute__((noinline)) static struct pair mk(long x)
{ struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) static long real3(struct box *o, long a, long b)
{ (void)o; return a * 7 + b; }

__attribute__((noinline)) static long fwd2(struct box *o, long a, long b)
{ if (!a) return 0; return o->fn(o, a, b); }

__attribute__((noinline)) long wrapb(struct box *o, long a, long b);

__attribute__((noinline)) long wrapa(struct box *o, long a, long b)
{ if (!a) return 0; if (a & 1) return wrapb(o, a - 1, b) + 1; return fwd2(o, a, b); }

__attribute__((noinline)) long wrapb(struct box *o, long a, long b)
{ if (a > 5) return wrapa(o, a - 2, b) + 2; return fwd2(o, a, b); }

static struct box G;

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return wrapa(&G, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{ G.fn = real3; return (int)caller(argc & 7, (long)argv[0][0], argc + 1); }
