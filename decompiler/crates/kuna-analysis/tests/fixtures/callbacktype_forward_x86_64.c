/* callbacktype: a bsearch helper that forwards its own pointer to a parked
 * comparator (x86-64).
 *
 * `compare_*` is handed to `qsort`, so callbacktype declares it
 * `int (const void *, const void *)`. `search_*` calls the same comparator
 * directly with its own `WORD *w` and an element of its own `WORD *tab`.
 * The comparator's first decompile reads two fields through each parameter,
 * and that is what types the helper's `w` as a `struct_N *`. The helper uses
 * `w` for nothing else, so when it is decompiled again after the park, the
 * declaration's `void *` would take the type away.
 *
 *   search_g (gcc -O0): its caller passes a frame `WORD[]` for `tab`, so
 *   `calleevote` decompiles it again after the park.
 *   search_c (clang -O2 -fno-inline): `n` is propagated; the call passes the
 *   same two arguments and reads the `int` the comparator already returned.
 *   search_z (clang -O0): the call also passes the remainder `idiv` leaves in
 *   `rdx`, a third argument the declaration's closed list drops.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O0 -DPART=1 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_g.o callbacktype_forward_x86_64.c
 *   clang -O2 -fno-inline -DPART=2 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_c.o callbacktype_forward_x86_64.c
 *   clang -O0 -DPART=3 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_z.o callbacktype_forward_x86_64.c
 *   gcc -o callbacktype_forward_x86_64 forward_g.o forward_c.o forward_z.o
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char *start;
    long size;
} WORD;

#if PART == 1
#define COMPARE compare_g
#define SEARCH search_g
#define RUN run_g
#elif PART == 2
#define COMPARE compare_c
#define SEARCH search_c
#define RUN run_c
#else
#define COMPARE compare_z
#define SEARCH search_z
#define RUN run_z
#endif

static int COMPARE(const void *a, const void *b) {
    const WORD *x = a, *y = b;
    long n = x->size < y->size ? x->size : y->size;
    int r = memcmp(x->start, y->start, n);
    if (r)
        return r;
    return (x->size > y->size) - (x->size < y->size);
}

__attribute__((noinline)) static int SEARCH(WORD *w, WORD *tab, long n) {
    long lo = 0, hi = n - 1;
    while (lo <= hi) {
        long mid = (lo + hi) / 2;
        int v = COMPARE(w, &tab[mid]);
        if (v < 0)
            hi = mid - 1;
        else if (v > 0)
            lo = mid + 1;
        else
            return 1;
    }
    return 0;
}

int RUN(char *s) {
    WORD tab[3] = {{"b", 1}, {"a", 1}, {"c", 1}};
    qsort(tab, 3, sizeof tab[0], COMPARE);
    WORD w = {s, (long)strlen(s)};
    return SEARCH(&w, tab, 3);
}

#if PART == 1
int run_c(char *s);
int run_z(char *s);

int main(int argc, char **argv) {
    printf("%d %d %d\n", run_g(argv[0]), run_c(argv[0]), run_z(argv[0]));
    return argc > 5;
}
#endif
