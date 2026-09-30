/* armfloatreturn fixture: ARM hard-float (VFP) returns and arguments.
 *
 * clang --target=armv7a-linux-gnueabihf -marm -mfloat-abi=hard -mfpu=vfpv3-d16 \
 *       -O2 -fno-optimize-sibling-calls -c armfloatreturn_armhf.c -o armfloatreturn_armhf.o
 *
 * The object carries Tag_ABI_VFP_args=1, so a double comes back in d0 and a
 * float in s0.  twocalls/twocallsf pass the parameter to a first call and a
 * computed value in the same register to a second one. */
volatile double sink;
__attribute__((noinline)) double scale(double x) { sink = x; return x * 1.5; }
__attribute__((noinline)) float scalef(float x) { sink = x; return x * 1.5f; }
double fixed(void) { return 1.5; }
float narrow(double x) { return x + x; }
double twocalls(double x) { scale(x); return scale(x * 3.0) + 1.0; }
float twocallsf(float x) { scalef(x); return scalef(x * 3.0f) + 1.0f; }
int keep(double *p, double x) { *p = x * 2.0; return 1; }
