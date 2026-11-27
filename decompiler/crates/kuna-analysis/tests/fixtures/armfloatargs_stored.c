/* armfloatargs stored-double fixture: ARM hard-float double parameters that a
 * function only stores to a global.
 *
 * arm-linux-gnueabihf-gcc -O2 -marm -mfpu=vfpv3-d16 -c armfloatargs_stored.c -o armfloatargs_stored_arm.o
 * arm-linux-gnueabihf-gcc -O2 -mthumb -mfpu=vfpv3-d16 -c armfloatargs_stored.c -o armfloatargs_stored_thumb.o
 *
 * `vstr d0,[r3]` to a global becomes a copy into that global, which nothing
 * in the function reads again. put_g and put_gv store their double, put_two
 * stores two, put_ref stores its double to the stack and passes its address
 * on, sel and zero_if store a double that an if or a select merges first,
 * put_gf is the float control, and call_put passes put_g the double it
 * computes. */
double g;
volatile double gv;
float gf;
__attribute__((noinline)) void put_g(double v) { g = v; }
__attribute__((noinline)) void put_gv(double v) { gv = v; }
__attribute__((noinline)) void put_two(double a, double b) { g = a; gv = b; }
__attribute__((noinline)) void put_gf(float v) { gf = v; }
extern void use_ref(const double *);
void put_ref(double v) { use_ref(&v); }
__attribute__((noinline)) void sel(double a, double b, int c) { g = c ? a : b; }
__attribute__((noinline)) void zero_if(double v, int c) { if (c) v = 0.5; g = v; }
void call_put(double d) { put_g(d * 2.0); g += 1.0; }
