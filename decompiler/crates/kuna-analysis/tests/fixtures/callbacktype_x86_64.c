/* callbacktype fixture: a function whose address is handed to a declared
 * library callback slot (x86-64).
 *
 * `by_key` is passed to `qsort` and nowhere else, so the slot's declared
 * `int (*)(const void *, const void *)` is what it is: with the option off its
 * parameters are whatever its body made of them, with the option on they are
 * `void *` and the return is `int`.
 *
 * `on_int` is passed to `signal` and never reads its argument, so nothing in
 * its body recovers one; the slot declares `void (*)(int)` and the parameter
 * has to APPEAR, which is why the declaration is parked rather than voted.
 *
 * `by_key_desc` is the negative control: it is passed to `qsort` too, but its
 * address is also stored in the `ORDER` table the image loads, so it reaches
 * somewhere the `qsort` call does not explain and must keep whatever its own
 * body recovered.
 *
 * `by_name` ends `return strcmp(...)`, which gcc compiles to a `jmp`. Nothing
 * in the program reads its result, so its own recovery says `void`; the slot's
 * `int` is still computed, because `strcmp` leaves its declared `int` in the
 * register the slot's caller reads. `warn_path` is the same machine shape from
 * a `void` function cast into `glob`'s `int (*)(const char *, int)` errfunc
 * slot: its tail-called `__fprintf_chk` leaves an `int` there too, and `glob`
 * reads it, so it is declared `int` and returns that call's value, which is
 * also what IDA prints for it. `by_count` returns `strcmp`'s value past a
 * counter increment, so its call is not a tail call; nothing between the call
 * and the `ret` writes `eax`, and it is declared `int` the same way.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o callbacktype_x86_64 callbacktype_x86_64.c
 */
#include <glob.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noinline))

struct row { int key; int tag; };
struct named { const char *name; int n; };

NI int by_key(const void *a, const void *b)
{
    return ((const struct row *)a)->key - ((const struct row *)b)->key;
}

NI int by_key_desc(const void *a, const void *b)
{
    return ((const struct row *)b)->key - ((const struct row *)a)->key;
}

NI void on_int(int sig)
{
    (void)sig;
    fputs("caught\n", stderr);
}

NI int by_name(const void *a, const void *b)
{
    return strcmp(((const struct named *)a)->name, ((const struct named *)b)->name);
}

static int CMPS;

NI int by_count(const void *a, const void *b)
{
    int r = strcmp(*(const char *const *)a, *(const char *const *)b);
    CMPS++;
    return r;
}

NI void warn_path(const char *p, int e)
{
    fprintf(stderr, "%s %d\n", p, e);
}

int (*const ORDER[1])(const void *, const void *) = { by_key_desc };

int main(int argc, char **argv)
{
    struct row v[4];
    struct named w[2] = { { argv[0], 1 }, { "x", 2 } };
    glob_t g;
    for (int i = 0; i < 4; i++) {
        v[i].key = (argc + i) * 7;
        v[i].tag = i;
    }
    signal(SIGINT, on_int);
    qsort(v, 4, sizeof v[0], by_key);
    qsort(v, 4, sizeof v[0], by_key_desc);
    qsort(w, 2, sizeof w[0], by_name);
    qsort(argv, (size_t)argc, sizeof argv[0], by_count);
    glob("/tmp/*", 0, (int (*)(const char *, int))warn_path, &g);
    printf("%d %d %p %s %zu %d\n", v[0].key, v[3].tag, (void *)ORDER[0], w[0].name, g.gl_pathc, CMPS);
    return 0;
}
