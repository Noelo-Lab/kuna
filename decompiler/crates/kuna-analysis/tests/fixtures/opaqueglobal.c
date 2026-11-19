/* Predicates an obfuscator plants in front of real code, and the look-alikes
   that depend on their input (option opaquepredicate).
   clang -O0 -nostdlib -static -fno-pie -fno-asynchronous-unwind-tables
         -fcf-protection=none -Wl,--build-id=none opaqueglobal.c -o opaqueglobal_x86_64_clang_O0 */
#define NI __attribute__((noinline))
int x, y;            /* never written: the two globals of bogus control flow */
int seven = 7;       /* initialized, never written */
int live = 3;        /* written by set_live */
int esc;             /* its address is passed to take */
int arr[4];          /* written through an index */
int next;            /* never written, near arr */

NI int sink(int a) { return a ^ 0x1234; }
NI void set_live(int v) { live = v; }
NI void take(int *p) { *p = 5; }
NI void fill(int i, int v) { arr[i & 3] = v; }

/* Never-written globals (value `globals`). */
NI int bogus(int z) { if (y >= 10) return sink(z * 3); return z + 1; }
NI int init(int z) { if (seven != 7) return sink(z * 5); return z + 2; }
NI int afterarr(int z) { if (next != 0) return sink(z * 6); return z + 6; }
NI int written(int z) { if (live > 2) return z + 3; return z * 7; }
NI int taken(int z) { take(&esc); if (esc > 2) return z + 4; return z * 9; }

/* Narrow inputs: true for every byte, false for some 32-bit value. */
NI int cube(unsigned char a, int z) { unsigned v = a; if ((v * v * v - v) % 3 != 0) return sink(z * 11); return z + 5; }
NI int square(signed char a, int z) { int v = a; if (v * v < 0) return sink(z * 13); return z + 7; }
NI int cubewide(unsigned v, int z) { if ((v * v * v - v) % 3 != 0) return z * 11; return z + 5; }

/* A comparison the branch it sits in already decided. */
NI int again(int v, int z) { if (v > 5) { z += sink(z); if (v <= 5) z *= 15; } return z; }
NI int range(int v, int z) { if (v < 5) { z += sink(z); if (v > 10) z *= 17; } return z; }
NI int rangekeep(int v, int z) { if (v < 5) { z += sink(z); if (v > 2) z *= 19; } return z; }

/* Both arms are the same code. */
NI int same(int v, int *p) { if (v & 4) { *p = *p * 21 + v; } else { *p = *p * 21 + v; } return v; }
NI int samecall(int v, int z) { if (v * v > 10) { z = sink(z * 23 + 1); } else { z = sink(z * 23 + 1); } return z; }
NI int differ(int v, int *p) { if (v & 4) { *p = *p * 21 + v; } else { *p = *p * 25 + v; } return v; }

void _start(void) {
  int t = 1;
  set_live(t); fill(t, t);
  t += bogus(t) + init(t) + afterarr(t) + written(t) + taken(t);
  t += cube(t, t) + square(t, t) + cubewide(t, t);
  t += again(t, t) + range(t, t) + rangekeep(t, t);
  t += same(t, &t) + samecall(t, t) + differ(t, &t);
  for (;;) ;
}
