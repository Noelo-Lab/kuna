/* Fixture for the widened libc signature table (`option libcsigs on|off`).
 *
 * Every callee here is a libc import the shipped 206-entry table did not carry,
 * so with the table off each wrapper's pointer parameter is only ever handed to
 * a callee with no declaration and stays a width-only integer.
 *
 * gcc -O0 -g0 -fno-stack-protector -o libcwiden_x86_64 libcwiden_x86_64.c
 */
#define _GNU_SOURCE 1
#include <fcntl.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

int can_write (int dirfd, const char *name)
{
  return faccessat (dirfd, name, W_OK, AT_EACCESS) == 0;
}

int move_entry (int fromfd, const char *from, int tofd, const char *to)
{
  return renameat (fromfd, from, tofd, to);
}

int make_dir (int dirfd, const char *name)
{
  return mkdirat (dirfd, name, 0755);
}

int jail (const char *root)
{
  return chroot (root);
}

int main (int argc, char **argv)
{
  if (argc < 3)
    return 1;
  if (can_write (AT_FDCWD, argv[1]))
    return make_dir (AT_FDCWD, argv[2]);
  if (jail (argv[1]))
    return 2;
  return move_entry (AT_FDCWD, argv[1], AT_FDCWD, argv[2]);
}
