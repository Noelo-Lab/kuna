/* The sixth counterexample: a forwarding thunk written in plain C.

   `fwd` names rdi and rsi and nothing else -- it tests `a` and tail-calls
   through `o->fn` -- so kuna recovers TWO parameters for it while the function
   it jumps to consumes three.  Two recovered parameters are exactly the two
   arguments the drop would leave behind, so BOTH halves of the recovered-
   prototype clause are satisfied and the forwarded rdx argument is deleted,
   together with mk()'s 16-byte rax:rdx return half that feeds it.  Unlike
   ce-forward-thunk.s this needs no hand assembly: gcc -O2 emits
   `test %rsi,%rsi; je; jmp *(%rdi)`.

   What declines it is the callee's body, not its prototype: at the `jmp
   *(%rdi)` the callee has not written rdx, so the caller's value can still
   reach code no recovery saw (`resolve_forward_transfer`).

   Build: gcc -O2 -o ce-forward-thunk-2param ce-forward-thunk-2param.c && strip */
struct pair { long a; long b; };
struct box { long (*fn)(struct box *, long, long); };

__attribute__((noinline)) static struct pair mk(long x)
{ struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) static long real3(struct box *o, long a, long b)
{ (void)o; return a * 7 + b; }

__attribute__((noinline)) long fwd(struct box *o, long a, long b)
{ if (!a) return 0; return o->fn(o, a, b); }

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
