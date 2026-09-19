/* protoorder fixture: a float stored beside an int (x86-64).
 *
 * `fill` loads `fp[3]`, passes it to `h(int, float)` and stores it into
 * `s->f`, next to the int `s->i`.  kuna types `s` as `int *`, so a float vote
 * on the loaded value printed the store as `v1[1] = (int)v3` -- a value
 * conversion where the machine copies bits.  The vote is refused on a value
 * that is stored, and the store stays `v1[1] = v3`.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -o protoorder_floatstore_x86_64 protoorder_floatstore_x86_64.c
 */
#define NI __attribute__((noinline))
struct S { int i; float f; };
NI float h(int k, float x) { return x * 2.5f + (float)k; }
NI int fill(int k, float *fp, struct S *s) { float x = fp[3]; float r = h(k, x); s->i = (int)r; s->f = x; return s->i + 1; }
int main(int argc, char **argv) { float fa[4] = {1, 2, 3, 1.5f}; struct S s[1]; return fill(argc, fa, s) + (int)s[0].f; }
