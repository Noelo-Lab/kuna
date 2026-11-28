/* A double kept in xmm0 across calls that never write it (gcc's -fipa-ra).
   gcc 11.4 (Ubuntu 22.04): gcc -O2 -mavx2 -fno-optimize-sibling-calls -o floatkeep_gcc_O2_avx2 floatkeep.c   (not stripped) */
#define NI __attribute__((noinline))
volatile double dsink;
NI int bump(int *p) { *p += 1; return *p; }
NI void flip(int *p) { *p ^= 3; }
NI double keep_int(double x, int *p) { int k = bump(p); return x * x + k; }
NI double keep_void(double x, int *p) { flip(p); return x * x; }
int main(int argc, char **argv) {
  int k = argc;
  dsink = keep_int(2.0 * argc, &k);
  dsink = keep_void(1.5 * argc, &k);
  return k;
}
