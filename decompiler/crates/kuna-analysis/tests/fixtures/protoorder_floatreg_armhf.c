/* protoorder fixture: a float that stays in a float register (ARM hard-float).
 *
 * `pick` puts 1500.0, 1000.0 or gf's result in s0 and passes it to
 * `h1(float)`.  A float vote is refused on a value another call produces or
 * reads when nothing declares its type -- except in a register the calling
 * convention assigns to floats, which is itself the declaration.  Without that
 * exception the constants print as their bits, `h1(0x44bb8000)` and
 * `v1 = 0x447a0000`; with it they print `1500.0` and `1000.0`.
 *
 * Built (off any ARM linker; this host has none) with:
 *   clang --target=armv7a-linux-gnueabihf -mfloat-abi=hard -mfpu=vfpv3 -O2 \
 *       -fno-inline -c protoorder_floatreg_armhf.c -o protoorder_floatreg_armhf.o
 */
#define NI __attribute__((noinline))
NI float h1(float x) { return x * 2.5f; }
NI float gf(float *p) { return p[3] * 3.0f; }
NI float pick(int m, float *p) { float v; if (m == 1) v = 1500.0f; else if (m == 2) v = gf(p); else v = 1000.0f; return h1(v); }
int main(int argc, char **argv) { float fa[4] = {1, 2, 3, 1.5f}; return (int)pick(argc, fa); }
