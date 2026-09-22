/* calleevote fixture: callbacks in a relocatable object (x86-64).
 *
 * `get_c` and `get_b` read one field of their argument and `direct` calls both
 * with its record; `fwd` is called with a string. All three also sit in the
 * const table `OPS`, whose words are zero in the object file and filled in by
 * an R_X86_64_64 relocation each, so they have callers nobody can list and
 * `calleevote` states nothing about any of them: the getters keep `void *`
 * and `fwd` keeps its integer parameter.
 *
 * Build:
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -c -o calleevote_ops_x86_64.o calleevote_ops_x86_64.c
 */
#define NI __attribute__((noinline))
struct rec { long a, b, c; };
struct ops { long (*get)(void *); long (*get2)(void *); long (*fn)(void *); };
extern long sink(void *p, long n);
static NI long get_c(void *p) { return *(long *)((char *)p + 0x10); }
static NI long get_b(void *p) { return *(long *)((char *)p + 0x8); }
static NI long fwd(void *p) { return sink(p, 16) + 1; }
const struct ops OPS = { get_c, get_b, fwd };
long direct(struct rec *r) { return get_c(r) + get_b(r) + r->a; }
long use_str(void) { return fwd("hello world"); }
