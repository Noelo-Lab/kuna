/* callerptr fixture: a pointer-width parameter whose own body says nothing
 * about it, and every caller of which passes a pointer (x86-64).
 *
 * `push` only stores its item in a list node. Its callers pass a record they
 * fill (`add_rec`), the string `strdup` returns (`add_name`) and a null
 * (`add_none`): three pointers, no type in common, so `callerptr` states
 * `void *`. `remember` stores its argument in a table; its callers pass a
 * string literal, the string `getenv` returns and a null, so the callers
 * agree on `char *`. `slot_of` shifts and divides the pointer it is handed,
 * which a pointer type would print as casts, so it keeps its integer.
 * `by_item` is a qsort comparator: its callers are unknown and it keeps
 * what its own body says. `is_table` compares its argument with the address
 * of a table that holds a zero byte, and its callers pass a literal, a
 * string and a null: a `char *` would print that address as a string literal
 * cut at the zero, so it keeps its integer. `via` only forwards its argument
 * to `is_tab2`, which compares it with the table: `via` may take the pointer,
 * but `is_tab2`, whose one caller then passes a `char *`, keeps its integer.
 * `hits` compares an address it derives from its argument with the table, and
 * keeps its integer too, and so does `far_cmp`, which reaches the comparison
 * through a dozen adds and merges, and `off_in`, which subtracts the table's
 * address. `align8` rounds the address it is handed up to a multiple of
 * eight, a number, so it keeps its integer.
 *
 * Build (symbols kept but the table's, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o callerptr_x86_64 callerptr_x86_64.c
 *   strip -N table callerptr_x86_64
 *   gcc -O0 -fno-stack-protector -fcf-protection=none \
 *       -o callerptr_O0_x86_64 callerptr_x86_64.c
 *   strip -N table callerptr_O0_x86_64
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#define NI __attribute__((noinline))
struct node { struct node *next; const void *item; };
struct rec { int a; long b; };
static struct node *head;
static const char *names[8];
static int nnames;
static const unsigned char table[16] = {1, 2, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
NI void push(const void *item)
{
    struct node *n = malloc(sizeof *n);
    if (!n)
        exit(1);
    n->next = head;
    n->item = item;
    head = n;
}
NI void add_rec(int a)
{
    struct rec *r = malloc(sizeof *r);
    if (!r)
        exit(1);
    r->a = a;
    r->b = a * 3L;
    push(r);
}
NI void add_name(const char *s) { push(strdup(s)); }
NI void add_none(void) { push(0); }
NI void remember(const char *s) { names[nnames++ & 7] = s; }
NI unsigned long slot_of(const void *p) { return ((unsigned long)p >> 4) % 31; }
NI int is_table(long p) { return p == (long)table; }
NI uintptr_t align8(uintptr_t p) { return (p + 7) & ~(uintptr_t)7; }
static volatile long sink;
NI int is_tab2(long p) { return p == (long)table; }
NI int via(long p) { sink = 1; return is_tab2(p); }
long g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11, g12;
NI int far_cmp(long p)
{
    long a = p + g1;
    if (g2)
        a += g3;
    a += g4;
    if (g5)
        a += g6;
    a += g7;
    if (g8)
        a += g9;
    a += g10;
    if (g11)
        a += g12;
    a += g1 * 3;
    if (g2 > 4)
        a += g4 * 5;
    a += g5 * 7;
    if (g6 > 2)
        a += g7 * 9;
    return a == (long)table;
}
NI long off_in(long p) { return p - (long)table; }
NI int hits(long p, int n)
{
    int c = 0;
    for (int i = 0; i < n; i++)
        c += (p + i == (long)table);
    return c;
}
NI long check(const char *a, long *w)
{
    w[0] = a[0];
    return is_table((long)"xyz") + is_table((long)a) + is_table(0) + align8((uintptr_t)a) - (uintptr_t)a
        + align8((uintptr_t)w) - (uintptr_t)w + via((long)"xyz") + via((long)a) + via(0)
        + hits((long)"xyz", 2) + hits((long)a, (int)w[0] & 1) + hits(0, 0)
        + far_cmp((long)"xyz") + far_cmp((long)a) + far_cmp(0)
        + (off_in((long)"xyz") != 0) + (off_in((long)a) != 0) + (off_in(0) != 0);
}
NI int by_item(const void *a, const void *b)
{
    return (*(const struct node *const *)a)->item < (*(const struct node *const *)b)->item;
}
int main(int argc, char **argv)
{
    add_rec(argc);
    add_name(argv[0]);
    add_none();
    remember("first");
    remember(getenv("HOME"));
    remember(0);
    struct node *v[3];
    int k = 0;
    unsigned long s = 0;
    for (struct node *n = head; n && k < 3; n = n->next) {
        v[k++] = n;
        s += slot_of(n) + slot_of(argv);
    }
    qsort(v, k, sizeof v[0], by_item);
    long w[2];
    s += check(argv[0], w) + w[0] + table[argc & 15];
    printf("%d %lu %d %s\n", k, s, nnames, names[0]);
    return 0;
}
