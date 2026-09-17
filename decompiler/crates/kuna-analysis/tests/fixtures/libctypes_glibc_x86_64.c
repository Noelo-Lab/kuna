/* Fixture for `option libctypes glibc` (tests/stages/kuna-libctypes.xml).
 *
 * Built: gcc -O1 -o libctypes_glibc_x86_64 libctypes_glibc_x86_64.c
 *        (no `-g`, on purpose)
 *
 * The sibling `libctypes_stat_x86_64` carries debug info, so its `struct stat`
 * comes from DWARF and the named table only has to adopt it. This one carries
 * none, so every named aggregate in it is MINTED by the table, which is the
 * only arm in which the `glibc` field layouts are installed at all.
 *
 * Two field reads, one per aggregate:
 *
 *   - `next_byte` is the `getc_unlocked` refill body: it compares
 *     `f->_IO_read_ptr` against `f->_IO_read_end` and calls the imported
 *     `__uflow(FILE *)`, which is the only thing in the binary that says what
 *     the parameter is;
 *   - `size_of` reads `st->st_size` out of a buffer the imported
 *     `stat(const char *, struct stat *)` filled.
 *
 * `noinline` keeps both out of `main`, so each one is a function the stage
 * script can load by name out of `.symtab`.
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <sys/stat.h>

__attribute__((noinline)) int next_byte(FILE *f)
{
    return getc_unlocked(f);
}

__attribute__((noinline)) long size_of(const char *path, struct stat *st)
{
    if (stat(path, st))
        return -1;
    return (long)st->st_size;
}

int main(int argc, char **argv)
{
    struct stat st;
    (void)argc;
    if (size_of(argv[0], &st) < 0)
        return 1;
    return next_byte(stdin);
}
