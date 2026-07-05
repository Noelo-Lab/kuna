/* Fixture for the `error(nonzero,...)`-conditional no-return recognizer
 * (`noreturn_error`, decbench F2). Built:
 *
 *     gcc -O2 -no-pie -fno-pic noreturn_error_x86_64.c -o noreturn_error_x86_64
 *
 * `err_fatal` is declared `_Noreturn` (exactly like GNU diffutils
 * `pfatal_with_name`), so gcc emits its tail as `call error(2,errno,"%s",m)`
 * with NO `ret` + trailing NOP alignment padding — but the *stripped* binary
 * carries no noreturn annotation, so kuna must INFER it from the call:
 * glibc error(status,...) calls exit(status) and never returns WHEN status != 0.
 *
 * `err_warn` is the CONTROL: its tail is `error(0,errno,"%s",m)` — status 0, so
 * error() RETURNS — and the function is a normal returning function. The
 * recognizer must NOT conclude it no-return (a false positive would drop live
 * caller code). `compute` / `compute_warn` are the single call sites.
 */
#include <error.h>
#include <errno.h>

__attribute__((noinline, noreturn)) static void err_fatal(const char *m) {
    error(2, errno, "%s", m);   /* status 2 (nonzero) -> exit(2), never returns */
    __builtin_unreachable();
}

/* Returns: error(0,...) prints and returns. NOT a no-return wrapper. */
__attribute__((noinline)) static int err_warn(const char *m) {
    error(0, errno, "%s", m);   /* status 0 -> error() returns */
    return 1;
}

__attribute__((noinline)) long compute(long x) {
    if (x < 0) err_fatal("negative");   /* the ONE call site */
    return x * 2 + 7;
}

__attribute__((noinline)) long compute_warn(long x) {
    if (x < 0) return err_warn("neg");  /* the ONE call site */
    return x + 1;
}

int main(int argc, char **argv) {
    return (int)(compute(argc) + compute_warn(argc)) + (long)argv;
}
