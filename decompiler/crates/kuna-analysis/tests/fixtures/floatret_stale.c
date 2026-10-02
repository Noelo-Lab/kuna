/* Wrappers a caller reads a pointer through before the wrapper is known to
   return one: `find` and `slot` hand back what `lookup` and `slot_of` return.
   gcc -O0 -o floatret_stale_gcc_O0 floatret_stale.c   (not stripped) */
#include <stdio.h>
#define NI __attribute__((noinline))
struct rec { long a, b, c, d; unsigned int e, f; };
struct rec table[4];
long *slots[4];
NI struct rec *lookup(int k) { struct rec *r = &table[k & 3]; r->d = r->a + r->b; return r; }
NI struct rec *find(int k) { return lookup(k); }
NI void set_e(int k, unsigned v) { find(k)->e = v; }
NI long get_c(int k) { return find(k)->c; }
NI long *slot_of(int k) { long *p = slots[k & 3]; p[1] = p[0] + 1; return p; }
NI long *slot(int k) { return slot_of(k); }
NI void put(int k, long v) { slot(k)[2] = v; }
NI long take(int k) { return slot(k)[3]; }
int main(int argc, char **argv) {
  (void)argv;
  static long store[4][4];
  for (int i = 0; i < 4; i++) slots[i] = store[i];
  set_e(argc, 7); put(argc, 11); store[1][3] = 5; table[1].c = 13;
  printf("%u %ld %ld %ld\n", table[1].e, store[1][2], take(argc), get_c(argc));
  return 0;
}
