/* Fixture for `option libctypes off|opaque` (tests/stages/kuna-libctypes.xml).
 *
 * Built: gcc -g -O1 -o libctypes_stat_x86_64 libctypes_stat_x86_64.c
 *
 * Two halves, on purpose:
 *
 *   - `mode_of` hands `stat` a `struct stat *`, so the DWARF this file carries
 *     defines `struct stat` with its real members. The named table must ADOPT
 *     that definition, not mint a shell over it: `st->st_mode` has to survive
 *     the option being on.
 *   - `close_it` / `open_it` declare the stream `void *` themselves, so no
 *     `FILE`/`_IO_FILE` DIE exists anywhere in this file and the named table
 *     has to MINT `FILE` for the imported `fclose`/`fopen`.
 */
#include <sys/stat.h>

extern int fclose(void *stream);
extern void *fopen(const char *path, const char *mode);

int mode_of(const char *path, struct stat *st)
{
    if (stat(path, st))
        return -1;
    return (int)st->st_mode;
}

void *open_it(const char *path) { return fopen(path, "r"); }

int close_it(void *stream) { return fclose(stream); }

int main(void)
{
    struct stat st;
    void *f = open_it("/etc/hostname");
    if (f)
        close_it(f);
    return mode_of("/etc/hostname", &st);
}
