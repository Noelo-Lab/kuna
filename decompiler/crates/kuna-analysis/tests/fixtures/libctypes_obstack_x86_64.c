/* Fixture for the second round of `option libctypes off|opaque`
 * (tests/stages/kuna-libctypes.xml).
 *
 * Built: gcc -g0 -O1 -o libctypes_obstack_x86_64 libctypes_obstack_x86_64.c
 *
 * `-g0`: with debug info the named table would ADOPT this file's own types,
 * and what is under test is the MINT.
 *
 * Two channels, one image:
 *
 *   - `_obstack_newchunk` is DEFINED here, not imported, exactly as gnulib
 *     links its obstack copy into every coreutils/grep/tar image. `grow` has
 *     no other evidence for what its pointer addresses, so the pointee name
 *     can only come from that one declaration.
 *   - `putspent` is IMPORTED, and `emit` passes both of its aggregates
 *     straight through: the `struct spwd *` and the stream.
 */
#include <stddef.h>

extern int putspent(void *sp, void *stream);

/* The gnulib entry point, defined by the image. */
__attribute__((noinline)) void _obstack_newchunk(void *h, size_t len)
{
    char **slots = (char **)h;
    slots[3] = slots[3] + len;
}

/* The witness: `h` reaches nothing but `_obstack_newchunk`. */
__attribute__((noinline)) void grow(void *h, size_t want)
{
    _obstack_newchunk(h, want);
}

/* The witness: both slots are named by the import's declaration alone. */
__attribute__((noinline)) int emit(void *sp, void *stream)
{
    return putspent(sp, stream);
}

int main(int argc, char **argv)
{
    char buf[64];
    grow(buf, (size_t)argc);
    return emit(buf, argv[0]);
}
