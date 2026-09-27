/* callbacktype: direct callers of a parked callback (x86-64).
 *
 * A park changes only the parked function. Every function here that calls a
 * callback directly prints exactly what it prints with `--option callbacktype
 * off`, and where the call it printed contradicts the declaration -- another
 * number of arguments, or a use of a result the declaration does not return --
 * the callback is not parked at all.
 *
 * `compare_*` is handed to `qsort` and also called directly by `search_*`,
 * which forwards its own `WORD *w` and an element of its own `WORD *tab`.
 *
 *   compare_g (gcc -O0), compare_c (clang -O2 -fno-inline): parked
 *   `int (void *, void *)`; search_g/search_c keep `struct_N *`.
 *   compare_z (clang -O0): search_z's call also passes the remainder `idiv`
 *   leaves in `rdx`, three arguments against the declaration's two, so it is
 *   refused and the three-argument call stands.
 *
 * `by_name_*` reads only the first word through each parameter and hands it to
 * `strcmp`, and prints `void`. `ent_before_*` forwards its own two pointers
 * and reads the result, and `minimum_*` scans an array of 24-byte records.
 *
 *   by_name_g2 (gcc -O2): parked `int (void *, void *)`; both callers keep
 *   `unsigned long *` and their calls.
 *   by_name_c2 (clang -O2): ent_before_c2 forwards its registers untouched and
 *   prints `by_name_c2()`, no arguments against two, so it is refused.
 *
 * `worker_*` is a `pthread_create` start routine handed an integer id through
 * its `void *`; `run_share_*` runs one share itself, forwarding its own `long`
 * and using the result, and `run_quiet_*` ignores it.
 *
 *   worker_g6 (gcc -O2): parked `void *(void *)`; run_share_g6 and
 *   run_quiet_g6 keep `long` and their calls.
 *   worker_c7 (clang -O2): run_share_c7 tail-forwards its register and prints
 *   `worker_c7()`, so it is refused.
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
 *   gcc -O2 -DPART=6 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_g6.o callbacktype_forward_x86_64.c
 *   clang -O2 -DPART=7 -fno-stack-protector -fcf-protection=none \
 *       -c -o forward_c7.o callbacktype_forward_x86_64.c
 *   gcc -o callbacktype_forward_x86_64 forward_g.o forward_c.o forward_z.o \
 *       forward_g2.o forward_c2.o forward_g6.o forward_c7.o -lpthread
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char *start;
    long size;
} WORD;

#if PART == 6 || PART == 7
#include <pthread.h>
#if PART == 6
#define WORKER worker_g6
#define RUN_SHARE run_share_g6
#define RUN_QUIET run_quiet_g6
#define G g_g6
#define RUN3 run_g6
#else
#define WORKER worker_c7
#define RUN_SHARE run_share_c7
#define RUN_QUIET run_quiet_c7
#define G g_c7
#define RUN3 run_c7
#endif

static long G[16];

__attribute__((noinline)) static void *WORKER(void *arg) {
    long id = (long)arg;
    G[id & 15] += id * 2;
    return (void *)(id + 1);
}

__attribute__((noinline)) long RUN_SHARE(long id) { return (long)WORKER((void *)id) * 3; }

__attribute__((noinline)) void RUN_QUIET(long id) { WORKER((void *)id); }

long RUN3(int k) {
    pthread_t t[4];
    for (long i = 0; i < 4; i++)
        pthread_create(&t[i], NULL, WORKER, (void *)i);
    for (int i = 0; i < 4; i++)
        pthread_join(t[i], NULL);
    RUN_QUIET(k);
    return RUN_SHARE(k + 4) + G[5];
}
#elif PART == 4 || PART == 5
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
long run_g6(int k);
long run_c7(int k);

int main(int argc, char **argv) {
    printf("%d %d %d\n", run_g(argv[0]), run_c(argv[0]), run_z(argv[0]));
    printf("%d %d\n", run_g2(argc), run_c2(argc));
    printf("%ld %ld\n", run_g6(argc), run_c7(argc));
    return argc > 5;
}
#endif
#endif
