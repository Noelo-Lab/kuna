/* Fixture for `--option dwarfstructs` (kuna-console/tests/verify_dwarfstructs.rs).
 *
 * Every aggregate shape the DWARF layout importer has to get right, in one -g
 * -O1 binary: a struct passed BY VALUE, a small (register-pair) struct return, a
 * large (sret) struct return, a union, a nested struct, a bitfield, and a
 * self-referential linked-list node. `noinline` keeps gcc from emitting an
 * abstract instance + a nameless out-of-line copy, which is a separate
 * DW_AT_abstract_origin gap the fixture is not about.
 *
 * `collide.c` defines a SECOND `struct Same` of a different size, so the linked
 * binary carries two same-named, different-sized aggregates -- the collision the
 * type factory refuses once fields are installed.
 */
#include <stdio.h>
#include <string.h>

#define NI __attribute__((noinline))

typedef struct { int a; int b; } P8;
typedef struct { long x; long y; long z; } Big24;
typedef union { int i; float f; char c[4]; } U4;
typedef struct { char tag; P8 inner; long tail; } Nest;
struct Bits { unsigned int lo : 3; unsigned int mid : 5; unsigned int hi : 24; };
struct Node { int val; struct Node *next; };

/* The collision: this `Same` is 4 bytes; collide.c's is 16. */
struct Same { int only; };

NI int   take_struct(P8 p, int k)      { return p.a + p.b + k; }
NI P8    ret_struct(unsigned int x)    { P8 s; s.a = x + 1; s.b = x * 7; return s; }
NI Big24 ret_big(long q)               { Big24 b; b.x = q; b.y = q * 2; b.z = q * 3; return b; }
NI int   take_union(U4 u)              { return u.i + u.c[1]; }
/* Three readers of the SAME union through a pointer: a union's members all sit
 * at offset 0, so any offset-keyed deduplication collapses it to its first
 * member and `u->f` / `u->c[2]` silently become reads of `u->i`. */
NI float union_second(U4 *u)           { return u->f; }
NI char  union_third(U4 *u)            { return u->c[2]; }
NI long  take_nest(Nest *n)            { return n->tag + n->inner.a + n->inner.b + n->tail; }
NI unsigned int take_bits(struct Bits *b) { return b->lo + b->mid + b->hi; }
NI int   walk_list(struct Node *n)     { int t = 0; while (n) { t += n->val; n = n->next; } return t; }
NI int   read_same_small(struct Same *s) { return s->only; }

int read_same_big(void *s);

int main(int argc, char **argv)
{
  P8 p = { argc, argc * 2 };
  Big24 b = ret_big(argc);
  U4 u; u.i = argc;
  struct Node n2 = { 5, 0 }; struct Node n1 = { 3, &n2 };
  Nest nn = { (char)argc, p, b.x };
  struct Bits bb; memset(&bb, 0, sizeof bb);
  bb.lo = argc; bb.mid = argc; bb.hi = argc;
  struct Same sm = { argc };
  char big[16]; memset(big, argc, sizeof big);
  printf("%d %ld %d %ld %u %d %d %d %d\n",
         take_struct(p, 1), (long)ret_struct(argc).b, take_union(u),
         take_nest(&nn), take_bits(&bb), walk_list(&n1),
         read_same_small(&sm), read_same_big(big),
         (int)strlen(argv[0]) + (int)union_second(&u) + union_third(&u));
  return 0;
}
