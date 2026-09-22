/* protoorder cycles fixture: a recursive function redone by the structsynth
 * convergence sweep (x86-64).
 *
 * `walk` calls itself on each child and reads `opts` (0x0) and `kids` (0x20)
 * through its argument, so its first decompile mints a two-field structure and
 * states it as its parameter type.  `look` is decompiled after it and reads
 * `count` (0x8) as well, so its larger layout supersedes `walk`'s, and the sweep
 * decompiles `walk` again.  That redo must not read the statement its own first
 * decompile made: at its call to itself it would type the child pointer as the
 * superseded structure while its own parameter takes the survivor.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -o protoorder_cyclestruct_x86_64 protoorder_cyclestruct_x86_64.c
 */
struct node {
    long *opts;
    long count;
    long pad0;
    long pad1;
    struct node **kids;
};

__attribute__((noinline)) long walk(struct node *p, long *acc)
{
    struct node **k = p->kids;
    long *o = p->opts;
    long s = *o;
    if (k) {
        while (*k) {
            s += walk(*k, o);
            k++;
        }
    }
    return s + (acc ? *acc : 0);
}

__attribute__((noinline)) long look(struct node *p)
{
    long *k = (long *)p->kids;
    return *p->opts + p->count + (k ? *k : 0);
}

int main(void)
{
    static long v = 3;
    static struct node leaf = {&v, 1, 0, 0, 0};
    return (int)(walk(&leaf, &v) + look(&leaf));
}
