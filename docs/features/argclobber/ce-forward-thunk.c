/* The fifth counterexample: a pure forwarding thunk.

   `fwd` (ce-forward-thunk.s) is `mov (%rdi),%rax; jmp *%rax` -- it never names
   rsi or rdx, so kuna recovers ONE parameter for it while the function it tail
   calls consumes three.  protoorder states that short list in both its modes
   (`KUNA_PROTOORDER_TRACE=1` prints `state sub_11cd params=1`), so the
   free-bytes half of the callee clause admits a drop here and the forwarded
   rdx argument is deleted.  What declines it is the surviving-argument half:
   one recovered parameter cannot account for the two arguments that would
   remain.

   Build: gcc -O2 -o ce-forward-thunk ce-forward-thunk.c ce-forward-thunk.s
          strip ce-forward-thunk
   Check: kuna decompile-all ce-forward-thunk --option argclobber off|on
          must be byte-identical. */
#include <stdio.h>
struct pair { long a; long b; };
long fwd(void *o, long a, long b);
__attribute__((noinline)) static struct pair mk(long x)
{ struct pair p = { x*3, x ^ 0x55 }; return p; }
static long real3(void *o, long a, long b) { (void)o; return a*7 + b; }
struct box { long (*fn)(void*, long, long); } G;
__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c) p.a = a / d, p.b = a % d;
    else p = mk(a);
    return fwd(&G, p.a, p.b) + 1;
}
int main(int argc, char **argv)
{ G.fn = real3; return (int)caller(argc & 7, (long)argv[0][0], argc + 1); }
