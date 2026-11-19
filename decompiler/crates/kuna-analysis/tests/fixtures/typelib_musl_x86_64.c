/* Fixture for `option typelib` on an image linked against musl
 * (kuna-cli/tests/typelib.rs).
 *
 * Built against the musl 1.2.6 headers, with no musl toolchain: the C library
 * is a stub shared object carrying musl's soname, so the image imports the
 * functions and records `libc.musl-x86_64.so.1` as needed.
 *
 *   M=musl-1.2.6; mkdir -p gen/bits
 *   sed -f $M/tools/mkalltypes.sed $M/arch/x86_64/bits/alltypes.h.in \
 *       $M/include/alltypes.h.in > gen/bits/alltypes.h
 *   cp $M/arch/x86_64/bits/syscall.h.in gen/bits/syscall.h
 *   echo 'int strerror_r(){return 0;} int basename(){return 0;} int getrusage(){return 0;}' \
 *     | clang --target=x86_64-linux-musl -nostdlib -shared -fPIC -x c - \
 *         -Wl,-soname,libc.musl-x86_64.so.1 -o libc.musl-x86_64.so.1
 *   clang --target=x86_64-linux-musl -O1 -fPIC -nostdinc -nostdlib -shared \
 *       -I$M/arch/x86_64 -I$M/arch/generic -Igen -I$M/include \
 *       typelib_musl_x86_64.c -L. -l:libc.musl-x86_64.so.1 -o typelib_musl_x86_64
 *
 * `strerror_r` and `basename` are declared differently by glibc, and
 * `struct rusage` is laid out differently, so the library offers all three to
 * this image only.
 */
#include <libgen.h>
#include <string.h>
#include <sys/resource.h>

long max_rss(void) {
    struct rusage ru;
    if (getrusage(RUSAGE_SELF, &ru) != 0)
        return -1;
    return ru.ru_maxrss + ru.ru_utime.tv_sec;
}

int describe(int err, char *buf, unsigned long len) {
    return strerror_r(err, buf, len) == 0 ? buf[0] : 0;
}

char *leaf(char *path) {
    return basename(path);
}
