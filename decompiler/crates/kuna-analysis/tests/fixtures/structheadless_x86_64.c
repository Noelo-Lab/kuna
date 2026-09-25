/* structheadless fixture: a record read only past its start (x86-64).
 *
 * `mode_of` reads three members of a `struct entry` and none of them is the
 * first; it is called directly, so every caller is known and `closed` gives
 * its parameter a record. `link_of` reads two members past the start too, but
 * its address goes to `qsort`, so its callers are unknown and its parameters
 * stay `void *`. `name_len` reads the first member and one more, which
 * `structsynth` already takes.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o structheadless_x86_64 structheadless_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noinline))
struct entry { char *name; long size; unsigned int mode; unsigned int link; char ok; char linkok; };
NI unsigned int mode_of(const struct entry *e)
{
    return (e->linkok && e->size > 0) ? e->link : e->mode;
}
NI int by_link(const void *a, const void *b)
{
    const struct entry *x = a, *y = b;
    return (x->link < y->link) - (x->size < y->size);
}
NI long name_len(const struct entry *e) { return (long)strlen(e->name) + e->size; }
int main(int argc, char **argv)
{
    struct entry *v = calloc((size_t)argc, sizeof *v);
    if (!v)
        return 1;
    for (int i = 0; i < argc; i++) {
        v[i].name = argv[i];
        v[i].size = (long)strlen(argv[i]) - 2;
        v[i].mode = (unsigned)i * 7u;
        v[i].link = (unsigned)i * 3u;
        v[i].linkok = (char)(i & 1);
    }
    qsort(v, (size_t)argc, sizeof *v, by_link);
    unsigned long t = 0;
    for (int i = 0; i < argc; i++)
        t += mode_of(&v[i]) + (unsigned long)name_len(&v[i]);
    printf("%lu\n", t);
    free(v);
    return 0;
}
