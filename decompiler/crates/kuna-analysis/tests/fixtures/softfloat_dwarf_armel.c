/* softfloat_dwarf_armel fixture: DWARF-described double prototypes on a
 * soft-float ARM image.
 *
 * clang --target=armv7a-linux-gnueabi -marm -mfloat-abi=soft -O2 -g \
 *       -fno-optimize-sibling-calls -c softfloat_dwarf_armel.c -o softfloat_dwarf_armel.o
 *
 * The object's .ARM.attributes carry no Tag_ABI_VFP_args, so every double
 * travels in a core-register pair: dmix receives a in r0:r1 and k in r2, and
 * returns in r0:r1. */
volatile double sink;
__attribute__((noinline)) double dmix(double a, int k) { sink = a; return a * k; }
double s4(double a, int k) { return dmix(a, k * 2) * a; }
