/* Fixture for the structural no-return propagation consumer
 * (`noreturn_propagate`).  `my_die` is a CUSTOM no-return wrapper the static
 * name list does not know: it unconditionally ends in `abort()`.  It is called
 * from a SINGLE site (`compute`), so it is BELOW `noreturn_disc`'s >=3 evidence
 * threshold and at -O2 its `call abort` is followed by NOP alignment padding —
 * so ONLY the propagation pass (seeded from the Known `abort`, last-real-insn
 * rule, no threshold) concludes `my_die` no-return.  With it concluded, the
 * single `call my_die` in `compute` is no-return and the post-call dead code is
 * eliminated.  Built: gcc -O2 -no-pie -fno-pic. */
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline)) static void my_die(const char *m) {
    fprintf(stderr, "fatal: %s\n", m);
    abort();
}

__attribute__((noinline)) long compute(long x) {
    if (x < 0)
        my_die("negative");
    return x * 2 + 7;
}

int main(int argc, char **argv) {
    (void)argv;
    return (int)compute((long)argc - 2);
}
