/* calleevote fixture: what the complete caller set of a function says about
 * its parameters (x86-64).
 *
 * `report` reads three fields of its `struct table` and passes the pointer on
 * to `peek_used`, which reads one of them; every call to `peek_used` passes
 * that record, so `types` states it there. `same` only compares its arguments,
 * and its one caller passes strings, so `types` states `char *` for both. `get_cap` is a
 * one-field getter called directly, so `fields` gives it a record. `by_used`
 * reads one field of each argument too, but its address goes to `qsort`, so
 * its callers are unknown and its parameters stay `void *`. `mark` stores a
 * four-byte constant through the string its one caller passes; a `char *`
 * there would print that store as four character stores, so `types` states
 * nothing for it.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o calleevote_x86_64 calleevote_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noinline))
struct table { int n; long used; long buckets; long cap; };
static const char *last = "none";
NI long peek_used(const struct table *t) { return t->used * 3; }
NI long report(struct table *t)
{
    long s = 0;
    for (int i = 0; i < t->n; i++)
        s += t->used ^ i;
    return s + t->buckets + peek_used(t);
}
NI int same(const char *a, const char *b) { return a == b; }
NI long get_cap(const struct table *t) { return t->cap; }
NI int by_used(const void *a, const void *b)
{
    return ((const struct table *)a)->used < ((const struct table *)b)->used;
}
NI void mark(void *p) { *(unsigned int *)((char *)p + 4) = 0xffffffffu; }
int main(int argc, char **argv)
{
    struct table v[4];
    for (int i = 0; i < 4; i++) {
        v[i].n = argc + i;
        v[i].used = i * argc;
        v[i].buckets = i;
        v[i].cap = 16;
    }
    qsort(v, 4, sizeof v[0], by_used);
    char *s = strdup(argv[0]);
    mark(s);
    printf("%ld %ld %d %s\n", report(&v[0]), get_cap(&v[1]), same(argv[0], last), s);
    return 0;
}
