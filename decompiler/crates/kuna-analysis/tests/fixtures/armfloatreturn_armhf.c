/* armfloatreturn fixture: ARM hard-float (VFP) returns and arguments.
 *
 * clang --target=armv7a-linux-gnueabihf -marm -mfloat-abi=hard -mfpu=vfpv3-d16 \
 *       -O2 -fno-optimize-sibling-calls -c armfloatreturn_armhf.c -o armfloatreturn_armhf.o
 *
 * The object carries Tag_ABI_VFP_args=1, so a double comes back in d0 and a
 * float in s0.  twocalls/twocallsf pass the parameter to a first call and a
 * computed value in the same register to a second one.  bump returns an int
 * in r0 while half's double is still in d0; second leaves d0 unused below the
 * double it returns; w2 reads its two floats as the halves of d0. */
volatile double sink;
volatile int counter;
__attribute__((noinline)) double scale(double x) { sink = x; return x * 1.5; }
__attribute__((noinline)) float scalef(float x) { sink = x; return x * 1.5f; }
double fixed(void) { return 1.5; }
float narrow(double x) { return x + x; }
double twocalls(double x) { scale(x); return scale(x * 3.0) + 1.0; }
float twocallsf(float x) { scalef(x); return scalef(x * 3.0f) + 1.0f; }
int keep(double *p, double x) { *p = x * 2.0; return 1; }
__attribute__((noinline)) double half(double x) { counter++; return x * 0.5; }
int bump(double x, int k) { half(x); return k + 1; }
double second(double x, double y) { return y; }
double w2(float a, float b) { return (double)a * b; }
