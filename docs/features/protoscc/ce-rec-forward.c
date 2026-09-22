/* ce-forward-thunk-2param.c with the thunk made recursive.

   `fwd` names rdi and rsi and forwards rdx through `o->fn` and through its
   own recursive call, so its recovery is short of what the program consumes.
   Under `protoorder cycles` it states that short list; what must still decline
   the drop of `caller`'s rdx argument is the callee's body.

   Build: gcc -O2 -fno-optimize-sibling-calls -o ce-rec-forward ce-rec-forward.c && strip */
struct pair { long a; long b; };
struct box { long (*fn)(struct box *, long, long); };

__attribute__((noinline)) static struct pair mk(long x)
{ struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) static long real3(struct box *o, long a, long b)
{ (void)o; return a * 7 + b; }

__attribute__((noinline)) long fwd(struct box *o, long a, long b)
{ if (!a) return 0; if (a > 1000) return fwd(o, a >> 1, b) + 1; return o->fn(o, a, b); }

static struct box G;

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return fwd(&G, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{ G.fn = real3; return (int)caller(argc & 7, (long)argv[0][0], argc + 1); }
