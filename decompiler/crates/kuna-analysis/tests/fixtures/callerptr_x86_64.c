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
 * what its own body says.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o callerptr_x86_64 callerptr_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define NI __attribute__((noinline))
struct node { struct node *next; const void *item; };
struct rec { int a; long b; };
static struct node *head;
static const char *names[8];
static int nnames;
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
    printf("%d %lu %d %s\n", k, s, nnames, names[0]);
    return 0;
}
