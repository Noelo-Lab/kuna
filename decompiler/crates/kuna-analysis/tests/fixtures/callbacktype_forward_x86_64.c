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
 * `by_name_*` reads only the first word through each parameter and hands it to
 * `strcmp`, so its first decompile states `undefined8 *` for both (printed
 * `unsigned long *`) and prints `void`. `ent_before_*` forwards its own two
 * pointers and reads the result, and `minimum_*` scans an array of 24-byte
 * records with it. The declared `int` return differs from the printed
 * `void`, so both callers are decompiled again after the park; they keep the
 * `unsigned long *` the statement gave them and index it as they did, not as
 * `(char *)` byte offsets from a `void *`.
 *
 *   by_name_g2 (gcc -O2), by_name_c2 (clang -O2).
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O0 -DPART=1 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_g.o callbacktype_forward_x86_64.c
 *   clang -O2 -fno-inline -DPART=2 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_c.o callbacktype_forward_x86_64.c
 *   clang -O0 -DPART=3 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_z.o callbacktype_forward_x86_64.c
 *   gcc -O2 -DPART=4 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_g2.o callbacktype_forward_x86_64.c
 *   clang -O2 -DPART=5 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_c2.o callbacktype_forward_x86_64.c
 *   gcc -o callbacktype_forward_x86_64 forward_g.o forward_c.o forward_z.o \
 *       forward_g2.o forward_c2.o
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char *start;
    long size;
} WORD;

#if PART == 4 || PART == 5
#if PART == 4
#define BY_NAME by_name_g2
#define ENT_BEFORE ent_before_g2
#define MINIMUM minimum_g2
#define TAB tab_g2
#define RUN2 run_g2
#else
#define BY_NAME by_name_c2
#define ENT_BEFORE ent_before_c2
#define MINIMUM minimum_c2
#define TAB tab_c2
#define RUN2 run_c2
#endif

struct ent {
    const char *name;
    long id;
    struct ent *next;
};

static struct ent TAB[4] = {{"d", 4, 0}, {"b", 2, 0}, {"c", 3, 0}, {"a", 1, 0}};

__attribute__((noinline)) static int BY_NAME(const void *a, const void *b) {
    const struct ent *x = a, *y = b;
    return strcmp(x->name, y->name);
}

__attribute__((noinline)) int ENT_BEFORE(struct ent *p, struct ent *q) { return BY_NAME(p, q) < 0; }

__attribute__((noinline)) struct ent *MINIMUM(struct ent *v, int n) {
    struct ent *m = v;
    for (int i = 1; i < n; i++)
        if (BY_NAME(&v[i], m) < 0)
            m = &v[i];
    return m;
}

int RUN2(int k) {
    qsort(TAB, 4, sizeof TAB[0], BY_NAME);
    return ENT_BEFORE(&TAB[k & 1], &TAB[2]) + (int)MINIMUM(TAB, 4)->id;
}
#else

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
int run_g2(int k);
int run_c2(int k);

int main(int argc, char **argv) {
    printf("%d %d %d\n", run_g(argv[0]), run_c(argv[0]), run_z(argv[0]));
    printf("%d %d\n", run_g2(argc), run_c2(argc));
    return argc > 5;
}
#endif
#endif
