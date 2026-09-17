/* Fixture for `option libctypes glibc`'s TARGET gate
   (decompiler/crates/kuna-console/tests/verify_libctypes_glibc_target.rs).

   Built: mipsel-linux-gnu-gcc -g -O1 -o libctypes_mips32_glibc_le32 \
                                  libctypes_mips32_glibc_le32.c

   A 32-bit MIPS glibc image, WITH debug info on purpose. Its `.dynstr` names
   `libc.so.6` and a `GLIBC_2.x` version, so only the architecture half of
   `glibc::target_is_glibc_x86_64` refuses it -- and its own DWARF defines
   `struct stat` and `struct timespec` with `st_dev` and `tv_sec` at offset 0,
   because those names are the kernel ABI's and are shared by every libc and
   every architecture.

   That is the combination that once let a `--define-function 0x…=fopen`
   directive mint the x86-64 layouts here: `sizeof(FILE)` is not 216 on this
   target and no pointer in it is eight bytes wide. `hold` exists only to put
   `stat` and `timespec` in the debug info; `bump` is a body to declare a name
   at. */
#include <stdio.h>
#include <sys/stat.h>

__attribute__((noinline)) long hold(const char *p, struct stat *st) {
    if (stat(p, st)) return -1;
    return st->st_size + st->st_mtim.tv_sec;
}

__attribute__((noinline)) int bump(int n) { return n + 7; }

int main(int c, char **v) {
    struct stat st;
    printf("%ld %d\n", hold(v[0], &st), bump(c));
    return c;
}
