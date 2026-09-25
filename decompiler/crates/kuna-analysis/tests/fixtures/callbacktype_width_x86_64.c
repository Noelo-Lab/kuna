/* callbacktype negative controls: a callback wider than the slot it is cast
 * into (x86-64).
 *
 * Every function here has the right NUMBER of parameters for its slot, and
 * every one is also called directly, so the only thing wrong with the slot's
 * declaration is a width.
 *
 * `cleanup` takes a `struct ctx *` and is cast into `signal`'s
 * `void (*)(int)`. Its body reads all eight bytes of the first argument
 * register, where the slot declares a four-byte `int`. Parking the slot would
 * rebuild the pointer in the body out of the declared half and an unset upper
 * half, and print the direct call as `cleanup((int)G)`.
 *
 * `add_total` takes a `long` and is cast the same way. `main` passes it a
 * product that does not fit in 32 bits, so the truncated call computes a
 * different number from the program's.
 *
 * `lcmp` returns a `long` and is cast into `qsort`'s
 * `int (*)(const void *, const void *)`. Parking the slot would subtract in
 * the low half, and the direct call that prints the whole result would get
 * its high half back as a global that is not in the image.
 *
 * `lcmp_tail` is the same return, but the value is what a tail-called `strtol`
 * leaves: the body's own recovery computes nothing, and the call's declared
 * `long` is wider than the slot's `int`.
 *
 * `zcmp` returns a `long` holding a zero-extended comparison. Its body writes
 * only `eax`, so nothing in it is wider than an `int`; the direct call that
 * hands the whole `rax` to `printf("%ld")` is what says it is.
 *
 * `cmp_outer` returns `cmp_inner`'s result when it is non-zero, and both are
 * `qsort` comparators. At -O2 the outer one passes the whole `rax` the inner
 * call left straight through its `ret`, so declaring the inner one `int` while
 * the outer one keeps its own 8-byte return leaves the outer one returning a
 * register half of which nothing set.
 *
 * `noipa` for the reason callbacktype_refused_x86_64.c gives.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o callbacktype_width_x86_64 callbacktype_width_x86_64.c
 */
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noipa))

struct ctx {
    FILE *f;
    int n;
};

static struct ctx *G;
static long TOTAL;

NI void cleanup(struct ctx *c)
{
    if (c) {
        fclose(c->f);
        c->n = 0;
    }
}

NI void add_total(long x)
{
    TOTAL += x;
    printf("%ld\n", TOTAL);
}

NI long lcmp(const void *a, const void *b)
{
    return *(const long *)a - *(const long *)b;
}

NI long lcmp_tail(const void *a, const void *b)
{
    return strtol((const char *)a, 0, (int)(long)b & 15);
}

NI long zcmp(const void *a, const void *b)
{
    return (unsigned)(*(const int *)a < *(const int *)b);
}

NI int cmp_inner(const void *a, const void *b)
{
    return strcmp(*(char *const *)a, *(char *const *)b);
}

NI int cmp_outer(const void *a, const void *b)
{
    int r = cmp_inner(a, b);

    if (r)
        return r;
    return *(const long *)a < *(const long *)b ? -1 : *(const long *)a != *(const long *)b;
}

int main(int argc, char **argv)
{
    long v[3] = { 3L << 40, argc, 1 };
    char *s[3] = { argv[0], "12", "7" };

    G = malloc(sizeof *G);
    G->f = fopen(argv[0], "r");
    G->n = argc;
    signal(SIGTERM, (void (*)(int))cleanup);
    signal(SIGINT, (void (*)(int))add_total);
    qsort(v, 3, sizeof v[0], (int (*)(const void *, const void *))lcmp);
    qsort(s, 3, sizeof s[0], (int (*)(const void *, const void *))lcmp_tail);
    qsort(v, 3, sizeof v[0], (int (*)(const void *, const void *))zcmp);
    qsort(s, 3, sizeof s[0], cmp_inner);
    qsort(s, 3, sizeof s[0], cmp_outer);
    if (argc > 1)
        add_total(strtol(argv[1], 0, 0) * 1000000007L);
    printf("%ld %ld %s\n", lcmp(&v[2], &v[0]), lcmp_tail(argv[argc - 1], (void *)10), s[0]);
    printf("%ld\n", zcmp(&v[0], &v[1]));
    cleanup(G);
    return (int)TOTAL;
}
