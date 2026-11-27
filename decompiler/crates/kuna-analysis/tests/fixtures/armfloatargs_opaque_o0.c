/* armfloatargs opaque-callee fixture at -O0: a float and doubles passed to
 * callees nothing in the image describes, from gcc's unoptimized code.
 *
 * arm-linux-gnueabihf-gcc -O0 -fno-pie -mword-relocations -marm -mfpu=vfpv3-d16 -c armfloatargs_opaque_o0.c -o armfloatargs_opaque_o0_arm.o
 * arm-linux-gnueabihf-gcc -O0 -fno-pie -mword-relocations -mthumb -mfpu=vfpv3-d16 -c armfloatargs_opaque_o0.c -o armfloatargs_opaque_o0_thumb.o
 *
 * Each call passes a float in s0 and doubles from d1; s1 is the AAPCS-VFP
 * back-fill slot. gcc -O0 computes the arguments in scratch registers (s11,
 * s13, s15, d6, d7) and copies them into place, so those registers still hold
 * values at the call. They are no arguments, and the back-fill slot stays
 * empty: no call takes s1, and no function gains a float parameter for it. */
extern void sink_fd(float, double);
extern void sink_fdd(float, double, double);
extern void (*fp_fd)(float, double);
extern int gn;
void o0_fd(float f, double d) { sink_fd(f * 2.0f, d * 2.0); gn = 1; }
void o0_fp(float f) { fp_fd(f * 2.0f, 0.0); gn = 1; }
void o0_fdd(float f, double d, double e) { sink_fdd(f * 2.0f, d * 2.0, e * 2.0); gn = 1; }
