/*
 * declmerge_x86_64 — regression for the undeclared-variable (invalid-C) emit bug.
 *
 *     gcc -O2 -no-pie -fno-pic declmerge_x86_64.c -o declmerge_x86_64
 *
 * `make_dir_clone` (a reduction of tar's make_directory) has a size phi
 * `sz = (…) ? len : len + 1` that kuna's merge phase fails to coalesce with the
 * strlen result. The un-coalesced phi output picks up an instance overlapping a
 * parameter register, so `PrintC::emit_local_var_decls`' `is_param`
 * storage-containment test used to SKIP it — yet it is still referenced in the body,
 * producing an **undeclared variable / invalid C** (`v5` used but never declared).
 * The fix gates the is_param skip on the high actually being named as a prototype
 * parameter, so the local is declared. This fixture is the exact reproduction.
 */
#include <string.h>
#include <stdlib.h>

struct S { void *a[16]; };

__attribute__((noinline)) void *make_dir_clone(char *s, unsigned long mode) {
    size_t len = strlen(s);
    struct S *p = malloc(0x60);
    p->a[0] = 0; p->a[6] = 0; p->a[5] = 0; p->a[8] = 0;
    size_t n, sz;
    if (len >= 2 && s[len - 1] == '/') { n = len - 1; sz = len; }
    else { n = len; sz = len + 1; }
    void *buf = malloc(sz);
    p->a[11] = buf;
    buf = memcpy(buf, s, n);
    p->a[10] = (void *)mode;
    *((char *)buf + n) = 0;
    p->a[9] = 0;
    return p;
}

int main(int argc, char **argv) { return make_dir_clone(argv[0], argc) != 0; }
