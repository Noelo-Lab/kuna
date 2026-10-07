/* A function that stores its double into a global and also reads or writes
   the global's bits as an integer through an index from the global's address,
   which the whole-program scan cannot see (the address at the load is not a
   constant). From a declared scalar (ixa, lv0, ixs) any index refuses the float;
   from a struct or an array (fld, neg) every load and store through the index
   must move a whole float. put only indexes a double array as doubles and
   keeps it.
   pas hands gdp to a double parameter after reading its bits through an
   index: the argument vote refuses it too.
   x86_64:  gcc -O2 / gcc -O0, each with -nostdlib -static -fno-pie -no-pie
            -fno-asynchronous-unwind-tables
   aarch64: aarch64-linux-gnu-gcc -O2 -fno-pie -fno-asynchronous-unwind-tables -c */
#define NI __attribute__((noinline))
double gda = 2.5, gdl = 2.5, gds = 2.5, gdp = 2.5;
struct S { long a; double d; } gs = {1, 2.5};
double hist[4] = {2.5, 1.0, 1.0, 1.0};
double harr[4] = {2.5, 1.0, 1.0, 1.0};
long g7;
double gout;
NI void ixa(int i, double b) { g7 = ((volatile long *)&gda)[i] + 1; gda = b; }
NI void lv0(int i, double b) { long *p = (long *)&gdl; g7 = p[i] + 1; gdl = b; }
NI void ixs(int i, long x, double b) { gds = b; ((volatile long *)&gds)[i] = x + 1; }
NI void fld(int i, double b) { g7 = ((volatile long *)&gs)[i] + 1; gs.d = b; }
NI void neg(int i, double b) { g7 = ((long *)hist)[i] + 1; hist[2] = b; }
NI void put(int i, double b) { gout = harr[i] * 2.0; harr[0] = b; }
NI void sink(double x) { gout = x * 3.0; }
NI void pas(int i) { g7 = ((volatile long *)&gdp)[i] + 1; sink(gdp); }
NI void setp(double v) { gdp = v; }
NI double rd(void) { return gda * 1.0 + gdl + gds + gs.d + hist[2] + harr[0]; }
volatile int sinkv;
void _start(void) {
  int i = sinkv;
  ixa(i, 4.5); lv0(i, 4.5); ixs(i, 7, 4.5); fld(i, 4.5); neg(i, 4.5); put(i, 4.5);
  setp(1.5); pas(i);
  gout = rd();
  for (;;) {}
}
