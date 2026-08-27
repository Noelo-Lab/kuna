/* Fixture for `msvcfpconst`: MSVC `__real@` floating-point constant COMDATs.
 *
 *   clang --target=i686-pc-windows-msvc -O1 -c msvcfpconst_i386.c \
 *         -o msvcfpconst_i386.obj
 *
 * Four `__real@` symbols in one object, covering every spelling in both states:
 *
 *   defined here          __real@3fb999999999999a   16 hex, IEEE double bits (0.1)
 *                         __real@3f000000            8 hex, IEEE float bits (0.5f)
 *   undefined (extern)    __real@8@3ffec90fdaa22168c000   x87 80-bit, double (pi/4)
 *                         __real@4@3fff8000000000000000   x87 80-bit, float  (1.0f)
 *
 * The compiler emits the defined pair on its own for the two literals below.
 * The undefined pair is spelled with asm labels because it is the state a real
 * object reaches by COMDAT folding, which needs a second translation unit to
 * reproduce: the definition survives in exactly one object and every other one
 * references it undefined.  The 20-hex spelling is the VC6-era one (a 1990s
 * `bounce.obj` is full of it); the bare-bits pair is what today's compilers
 * emit.
 */
extern const double c_pi4 __asm__("__real@8@3ffec90fdaa22168c000");
extern const float c_one __asm__("__real@4@3fff8000000000000000");

double scale(double x) { return x * c_pi4 + 0.1; }
float fscale(float y) { return y * c_one * 0.5f; }
