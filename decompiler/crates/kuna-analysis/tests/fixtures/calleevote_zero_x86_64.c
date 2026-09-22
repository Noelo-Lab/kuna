/* calleevote fixture: a const ops table inside code loaded at address 0
 * (x86-64, the firmware layout).
 *
 * `get_c` and `get_b` each read one field of their argument. `go1` and `go2`
 * call both directly with a `struct node *`, and the const table `ops` names
 * both, so `run` also calls them on a character buffer. The table sits in
 * `.text`, which the linker script loads at address 0: the words of every
 * section the image loads are read whatever its address, so both getters
 * have callers nobody can list and keep `void *`. `pad` only keeps the
 * getters off address 0.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-toplevel-reorder -fno-pic -fno-stack-protector -fcf-protection=none \
 *       -ffreestanding -nostdlib -static -Wl,--build-id=none,-n \
 *       -T calleevote_zero_x86_64.ld -o calleevote_zero_x86_64 calleevote_zero_x86_64.c
 *
 * calleevote_zero_x86_64.ld:
 *   ENTRY(_start)
 *   SECTIONS {
 *     . = 0x0;
 *     .text : { *(.text*) *(.rodata*) }
 *     . = 0x1000;
 *     .data : { *(.data*) }
 *     .bss : { *(.bss*) *(COMMON) }
 *     /DISCARD/ : { *(.note*) *(.eh_frame*) *(.comment) }
 *   }
 */
#define NI __attribute__((noinline))
struct node { long a; long b; long c; };
volatile long sink;
char buf[64] = "hello world, this is a buffer";
struct node gn;
NI long pad(long x) { return x * 7 + sink; }
NI long get_c(void *p) { return *(long *)((char *)p + 16); }
NI long get_b(void *p) { return *(long *)((char *)p + 8); }
typedef long (*op_t)(void *);
const op_t ops[2] = { get_c, get_b };
NI void run(int i, void *x) { sink = ops[i](x) + 1; }
NI long go1(struct node *n) { long v = get_c(n) + get_b(n); sink = v; return v + 1; }
NI long go2(struct node *n) { long v = get_c(n) + get_b(n); sink = v; return v + 2; }
void _start(void)
{
    gn.c = 3;
    run((int)sink, buf);
    sink = pad(go1(&gn) + go2(&gn));
    for (;;)
        ;
}
