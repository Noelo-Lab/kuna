/* Fixture for full-depth DWARF type resolution (kuna `typedepth`).
 *
 * Every declaration here is ordinary C whose DWARF type chain is four or more
 * DIEs long, which the pre-fix mapper (a flat `MAX_TYPE_DEPTH = 3` counted over
 * every hop, transparent `typedef`/`const`/`volatile`/`restrict` links included)
 * truncated to `void`:
 *
 *   authors    const char *const *   ptr -> const -> ptr -> const -> char
 *   pn         const size_t *        ptr -> const -> typedef -> base
 *   ps         state_t *             ptr -> typedef -> ANONYMOUS struct
 *   argvp      char ***              ptr -> ptr -> ptr -> char  (three pointer
 *                                    hops: truncated even after `cppproto`
 *                                    collapsed the qualifier hops)
 *
 * `list_len` walks a SELF-REFERENTIAL struct (`struct node { struct node *next; }`)
 * and `ping`/`pong` two MUTUALLY recursive ones: the shapes whose DWARF type
 * graphs close on themselves, i.e. what the depth budget claimed to be protecting
 * against. They must still terminate under the cycle-counting guard.
 *
 * `-O0` so each declaration keeps a `DW_OP_fbreg` stack slot (the locals path),
 * `-g` for the DWARF, and the globals exercise the CU-top-level `DW_OP_addr` arm.
 *
 * Build (from this directory):
 *   gcc -O0 -g -fno-stack-protector -fcf-protection=none -no-pie \
 *       -o typedepth_x86_64 typedepth_x86_64.c
 */
#include <stdio.h>
#include <string.h>

typedef unsigned long size_t_alias;

/* An ANONYMOUS struct reached only through its typedef -- `mbstate_t`'s shape.
   Without the qualifier collapse it interns as the shared `anon_struct`. */
typedef struct { int count; int flags; } state_t;

struct node { int value; struct node *next; };

/* Mutually recursive: each names the other. */
struct ping_s;
struct pong_s;
struct ping_s { int v; struct pong_s *other; };
struct pong_s { int v; struct ping_s *other; };

/* A global whose element type is four DIEs deep: the extent was one byte. */
const char *const default_authors[3] = { "ada", "grace", "edsger" };
static state_t global_state;

/* `const char *const *authors` -- the coreutils `version_etc_ar` shape. */
int count_authors(const char *const *authors)
{
    const char *const *p = authors;
    int n = 0;
    while (*p) {
        n += (int)strlen(*p);
        p++;
    }
    return n;
}

/* `const size_t_alias *pn` -- pointer, const, typedef, base. */
size_t_alias sum_counts(const size_t_alias *pn, int n)
{
    const size_t_alias *cur = pn;
    size_t_alias total = 0;
    int i;
    for (i = 0; i < n; i++)
        total += cur[i];
    return total;
}

/* `state_t *ps` -- pointer through a typedef to an anonymous struct. The local
   `cur` is what carries the alias: without the qualifier collapse on the C path
   it interns as the shared `anon_struct`. */
int use_state(state_t *ps, int n)
{
    state_t *cur = ps;
    int total = 0;
    while (n-- > 0) {
        total += cur->count + cur->flags;
        cur = cur + 1;
    }
    return total;
}

/* A typedef'd ANONYMOUS struct whose name is already taken by a kuna core type
   (`code`) -- zlib's `inftrees.h` does exactly this. Borrowing the alias makes
   the type factory refuse the redefinition, and the aggregate must then fall
   back to the anonymous name rather than degrade the pointer to `void *`. */
typedef struct { unsigned char op; unsigned short val; } code;

int walk_codes(code *base, int n)
{
    code *next = base;
    int total = 0;
    while (n-- > 0) {
        total += next->val;
        next = next + 1;
    }
    return total;
}

/* `char ***argvp` -- three pointer hops, the gzip `add_envopt` shape. */
int take_argvp(int *argcp, char ***argvp)
{
    char ***slot = argvp;
    char **vec = *slot;
    int n = 0;
    while (vec[n])
        n++;
    *argcp = n;
    return n;
}

/* Self-referential struct: the type graph closes on itself. */
int list_len(struct node *head)
{
    struct node *cur = head;
    int n = 0;
    while (cur) {
        n += cur->value;
        cur = cur->next;
    }
    return n;
}

/* Mutual recursion between two struct types. */
int ping(struct ping_s *a)
{
    struct ping_s *cur = a;
    return cur->v + (cur->other ? cur->other->v : 0);
}

int pong(struct pong_s *b)
{
    struct pong_s *cur = b;
    return cur->v + (cur->other ? cur->other->v : 0);
}

int main(int argc, char **argv)
{
    size_t_alias counts[3] = { 1, 2, 3 };
    struct node tail = { 2, 0 };
    struct node head = { 1, &tail };
    code codes[2] = { { 1, 2 }, { 3, 4 } };
    struct pong_s po = { 20, 0 };
    struct ping_s pi = { 10, &po };
    char **vec = argv;
    int n = argc;

    po.other = &pi;
    global_state.count = argc;
    printf("%d %lu %d %d %d %d\n",
           count_authors(default_authors),
           sum_counts(counts, 3),
           use_state(&global_state, 1),
           take_argvp(&n, &vec),
           list_len(&head),
           ping(&pi) + pong(&po) + walk_codes(codes, 2));
    return 0;
}
