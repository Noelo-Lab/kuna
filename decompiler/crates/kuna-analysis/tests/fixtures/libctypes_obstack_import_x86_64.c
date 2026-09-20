/* Fixture for the two obstack channels of `option libctypes off|opaque`
 * (tests/stages/kuna-libctypes.xml).
 *
 * Built: gcc -g0 -O1 -o libctypes_obstack_import_x86_64 \
 *            libctypes_obstack_import_x86_64.c
 *
 * The twin of `libctypes_obstack_x86_64`, on the other channel: here the
 * `_obstack_*` entry points are IMPORTED from glibc rather than linked in from
 * gnulib, which is what the five `dpkg` programs of the benchmark corpus do.
 * The two publishers disagree about the size parameters -- gnulib's size type
 * is `size_t`, the installed glibc header (`/usr/include/obstack.h:184`)
 * declares plain `int` -- so the spelling this image must get is `int`, and
 * asserting `size_t` here would widen the caller's own parameter.
 *
 * `-g0` for the same reason as the twin: what is under test is the MINT.
 */
#include <stddef.h>

extern void _obstack_newchunk(void *h, int len);

/* The witness: `h` reaches nothing but the imported `_obstack_newchunk`. The
 * name differs from the twin fixture's `grow` so one assertion cannot match both. */
__attribute__((noinline)) void grow_import(void *h, int want)
{
    _obstack_newchunk(h, want);
}

int main(int argc, char **argv)
{
    char buf[64];
    grow_import(buf, argc);
    return (int)(size_t)argv[0];
}
