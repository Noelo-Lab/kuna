/* Fixture for the libc-aggregate fit rule of `option libctypes`
 * (tests/stages/kuna-libctypes.xml, `kuna_libcfit`).
 *
 * Built: gcc -g0 -O1 -o libctypes_container_x86_64 libctypes_container_x86_64.c
 *
 * A libc aggregate is often the FIRST member of a larger object, and then the
 * object's address and the member's are the same value. grep's kwset is the
 * corpus case: `struct kwset { struct obstack obstack; ... }`, whose address
 * `_obstack_newchunk` reads while the rest of the function reads members far
 * past the obstack's 88 bytes. `kws_grow` is that shape and `inst_refs` is the
 * same with a `pthread_mutex_t` at the head (an rsyslog instance record); both
 * must keep their own pointer instead of becoming an array of the aggregate.
 * `grow_in` is the control: its pointer stays inside the obstack and keeps the
 * name.
 *
 * `-g0`: what is under test is the table's vote, not a DWARF prototype.
 */
#include <obstack.h>
#include <pthread.h>

struct kwset {
    struct obstack obstack;
    long words;
    long pad[290];
    long trans;
};

struct inst {
    pthread_mutex_t mut;
    int refs;
    char name[64];
    long tail;
};

__attribute__((noinline)) void kws_grow(struct kwset *k, int n)
{
    _obstack_newchunk(&k->obstack, n);
    k->trans = n;
    k->words++;
}

__attribute__((noinline)) int inst_refs(struct inst *p)
{
    pthread_mutex_lock(&p->mut);
    p->refs++;
    int r = (int)p->tail;
    pthread_mutex_unlock(&p->mut);
    return r;
}

__attribute__((noinline)) void grow_in(struct obstack *h, int n)
{
    _obstack_newchunk(h, n);
    h->next_free += n;
}

static struct kwset ks;
static struct inst in;

int main(int argc, char **argv)
{
    (void)argv;
    kws_grow(&ks, argc);
    grow_in(&ks.obstack, argc);
    return inst_refs(&in);
}
