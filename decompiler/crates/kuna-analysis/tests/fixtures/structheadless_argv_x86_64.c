/* structheadless fixture: an argument vector its callers pass (x86-64).
 *
 * `obsolete_count` reads `argv[1]` and `argv[2]` and never `argv[0]`, the
 * shape of coreutils `tail`'s `parse_obsolete_option`. Its only caller is
 * `main`, which hands its `argv` to `getopt` as well, so the callers state `char **`
 * for the parameter and a record read past its start must not replace it.
 * `mode_of` reads a record past its start and is called directly too: there
 * nothing outside the function states a type, and `closed` makes it a record.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o structheadless_argv_x86_64 structheadless_argv_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#define NI __attribute__((noinline))
struct entry { char *name; long size; unsigned int mode; unsigned int link; char ok; char linkok; };
NI unsigned int mode_of(const struct entry *e)
{
    return (e->linkok && e->size > 0) ? e->link : e->mode;
}
NI long obsolete_count(int argc, char **argv)
{
    if (argc != 3 || *argv[2] == '-')
        return -1;
    return strtol(argv[1], NULL, 10) + (long)strlen(argv[2]);
}
int main(int argc, char **argv)
{
    struct entry e = { "e", (long)argc, 7u, 3u, 1, (char)(argc & 1) };
    if (getopt(argc, argv, "+") != -1)
        return 1;
    long n = obsolete_count(argc, argv);
    printf("%ld %u\n", n, mode_of(&e));
    return 0;
}
