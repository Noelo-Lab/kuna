/* calleevote fixture: a callback that is also called directly (MIPS32 LE).
 *
 * `get_c` reads one field of its argument. `go1` and `go2` call it directly
 * with a `struct node *`, and `_start` also hands its address to `run`, which
 * calls it on a character buffer. MIPS builds that address from two
 * instructions (`lui`+`addiu`), which no one-instruction reference walk sees,
 * so no function of this image may count as having every caller known:
 * `get_c` keeps `void *`, and `run` keeps its integer second parameter.
 *
 * Build (symbols kept, no DWARF):
 *   clang --target=mipsel-linux-gnu -O2 -fno-pic -mno-abicalls -ffreestanding \
 *       -fno-asynchronous-unwind-tables -c calleevote_callback_mipsel.c -o cb.o
 *   mipsel-linux-gnu-ld -e _start --build-id=none -o calleevote_callback_mipsel cb.o
 */
#define NI __attribute__((noinline))
struct node { long a; long b; long c; };
volatile long sink;
char buf[64] = "hello world, this is a buffer";
struct node gn;
NI long get_c(void *p) { return *(long *)((char *)p + 8); }
NI void run(long (*f)(void *), void *x) { sink = f(x) + 1; }
NI long go1(struct node *n) { long v = get_c(n); sink = v; return v + 1; }
NI long go2(struct node *n) { long v = get_c(n); sink = v; return v + 2; }
void _start(void)
{
    gn.c = 3;
    run(get_c, buf);
    sink = go1(&gn) + go2(&gn);
    for (;;)
        ;
}
