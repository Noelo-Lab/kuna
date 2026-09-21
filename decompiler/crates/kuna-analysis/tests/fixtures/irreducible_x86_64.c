/*
 * Irreducible-loop fixture (x86-64, GH-668): a loop with TWO entries, which the
 * structurer cannot fold into any structured form, so it keeps one residual
 * `goto`.  C spells that as a real `goto label_<addr>;`; the Rust back-end has
 * no form for a jump that is neither a `break` nor a `continue`, so it renders
 * the site as a comment plus a diverging `panic!("kuna: unstructured goto to
 * ...")`.  The fixture exists to hold that one rendering difference still.
 *
 * Build: gcc -O0 -no-pie -fno-pic -fno-asynchronous-unwind-tables \
 *            -fno-unwind-tables -fno-stack-protector -nostdlib -nostartfiles \
 *            -e irreducible -o irreducible_x86_64 irreducible_x86_64.c
 *
 * `-e irreducible` makes the one interesting function the ELF entry point, so a
 * libc-less 5 KB image is still fully discovered.  It is never executed: the
 * function returns into no runtime.
 */

int irreducible(int n, int m) {
    int i = 0;
    if (n > 0) goto L2;   /* the second entry into the L1/L2 cycle */
L1:
    i += 1;
    if (i >= m) goto L3;
L2:
    i += 2;
    if (i < 100) goto L1;
L3:
    return i;
}
