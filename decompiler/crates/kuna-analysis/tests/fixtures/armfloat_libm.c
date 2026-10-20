/* armfloatreturn fixture: a hard-float ARM executable whose own named
 * functions call imported <math.h> functions.
 *
 * arm-linux-gnueabihf-gcc -O2 -fno-builtin -fno-stack-protector \
 *     -Wl,--build-id=none armfloat_libm.c -o armfloat_libm_hard -lm
 *
 * `kuna decompile armfloat_libm_hard mant` decompiles one function and no
 * other: frexp's declared prototype must reach that first decompile. */
#include <math.h>
double mant(double x, int *e) { return frexp(x, e) * 2.0; }
double wave(double x) { return sin(x) + cos(x); }
int main(int argc, char **argv) { int e; return (int)(mant(argc, &e) + wave(argc)) + e; }
