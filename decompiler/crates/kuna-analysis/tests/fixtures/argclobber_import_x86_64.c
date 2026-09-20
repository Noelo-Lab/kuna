/* The seventh argclobber counterexample: a wrapper that forwards a register to
   an import with no signature -- no function pointer anywhere, plain C.

   `wrap` tests `a` and calls `ext3`, whose signature nothing states, so nothing
   at that call site reads rdx and kuna recovers TWO parameters for `wrap`.
   Those two are exactly the arguments a drop would leave behind, so the
   recovered-prototype accounting is satisfied and the forwarded rdx argument is
   deleted -- together with mk()'s 16-byte rax:rdx return half that feeds it --
   although `ext3` multiplies by it.

   What declines it is the direct call itself: `ext3` carries neither a source
   declaration nor a prototype this run recovered, so a value still live in rdx
   when control reaches it can reach a read no recovery saw.

   `ext3` has to be a real dynamic import: link it against a stub library so the
   PLT entry jumps through its GOT slot.  The library is not needed to decompile
   the binary, and is not committed.

   Build: echo 'long ext3(void*o,long a,long b){(void)o;return a*7+b;}' > ce.c
          gcc -O2 -shared -fPIC -o libce.so ce.c
          gcc -O2 -fno-inline -fno-optimize-sibling-calls \
              -o argclobber_import_x86_64 argclobber_import_x86_64.c \
              -L. -lce -Wl,-rpath,. && strip argclobber_import_x86_64 */
extern long ext3(void *o, long a, long b);
struct pair { long a; long b; };

__attribute__((noinline)) struct pair mk(long x)
{ struct pair p = { x * 3, x ^ 0x55 }; return p; }

__attribute__((noinline)) long wrap(void *o, long a, long b)
{ if (!a) return 0; return ext3(o, a, b); }

static long G;

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return wrap(&G, p.a, p.b) + 1;
}

int main(int c, char **v) { return (int)caller(c & 7, (long)v[0][0], c + 3); }
