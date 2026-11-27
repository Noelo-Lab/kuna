/* armfloatargs opaque-callee fixture: ARM hard-float doubles passed to callees
 * that nothing in the image describes (undefined symbols and a function
 * pointer).
 *
 * arm-linux-gnueabihf-gcc -O2 -marm -mfpu=vfpv3-d16 -c armfloatargs_opaque.c -o armfloatargs_opaque_arm.o
 * arm-linux-gnueabihf-gcc -O2 -mthumb -mfpu=vfpv3-d16 -c armfloatargs_opaque.c -o armfloatargs_opaque_thumb.o
 *
 * scale_d, load_d, conv_d, two_d, mixed_id, indirect_d and moved_d write the
 * d registers they pass for the call. chain_d hands an earlier call's result
 * on, forward_d its own entry d0, and after_store stores the double it
 * computes before a call that takes none: as for s0 and r0, none of those is
 * an argument. ign has a body that never reads d0, so call_ign's double is
 * not passed: the callee answers for its own reads. mixed_fd passes a float in
 * s0 and a double in d1; s1, the AAPCS-VFP back-fill slot, still holds half of
 * the caller's entry d0 and is no argument.
 *
 * The linked armfloatargs_opaque.so reaches the same callees through PLT stubs,
 * with armfloatargs_opaque_split.s linked in:
 * arm-linux-gnueabihf-gcc -O2 -marm -mfpu=vfpv3-d16 -fPIC -shared -nostdlib \
 *   -Wl,--build-id=none -Wl,-z,max-page-size=0x1000 armfloatargs_opaque.c \
 *   armfloatargs_opaque_split.s -o armfloatargs_opaque.so */
extern double fd(void);
extern void g(void);
extern void gd(double);
extern void sink_d(double);
extern void sink_dd(double, double);
extern void sink_id(int, double);
extern void sink_fd(float, double);
double gx;
int gn;
void scale_d(double d) { sink_d(d * 3.0); gn = 1; }
void load_d(void) { sink_d(gx); gn = 1; }
void conv_d(int n) { sink_d((double)n); gn = 1; }
void two_d(double a, double b) { sink_dd(b * 2.0, a * 3.0); gn = 1; }
void mixed_id(int n, double d) { sink_id(n + 1, d * 2.0); gn = 1; }
void indirect_d(void (*fp)(double), double d) { fp(d * 2.0); gn = 1; }
void moved_d(void) { double x = fd(); g(); gd(x); }
void chain_d(void) { gd(fd()); }
void forward_d(double d) { sink_d(d); gn = 1; }
void after_store(double d) { gx = d * 2.0; g(); gn = 1; }
__attribute__((noinline)) void ign(double x) { gn = 2; }
void call_ign(double d) { ign(d * 2.0); gn = 1; }
void mixed_fd(double d, float f) { sink_fd(f, d * 2.0); gn = 1; }
