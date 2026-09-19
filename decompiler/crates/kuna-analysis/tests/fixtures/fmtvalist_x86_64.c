/* formatstring: closing a resolved format call must not hand its phantoms to
 * the calls around it.
 *
 * credits is gnulib's version_etc shape. Built with -flto, its va_list lands in
 * the lowest four stack slots. The first fprintf passes everything in
 * registers, so without its format that call claims the va_list's slots as
 * three more arguments, and the fprintf calls in the switch, which push their
 * own arguments below the va_list, are refused the same slots. The case bodies
 * are reached only through a jump table, so their formats are not resolved.
 *
 * gcc -O2 -flto=auto -D_FORTIFY_SOURCE=2 -o fmtvalist_x86_64 fmtvalist_x86_64.c
 * (gcc 11.4, x86-64)
 */
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>

static void credits_list(FILE *out, const char *cmd, const char *pkg, const char *ver,
                  const char *const *who, size_t n) {
    if (cmd)
        fprintf(out, "%s (%s) %s\n", cmd, pkg, ver);
    else
        fprintf(out, "%s %s\n", pkg, ver);
    fprintf(out, "Copyright %s %d The Authors.\n", "(C)", 2026);
    fputs("This program comes with no warranty.\n", out);
    switch (n) {
    case 0: break;
    case 1: fprintf(out, "by %s.\n", who[0]); break;
    case 2: fprintf(out, "by %s and %s.\n", who[0], who[1]); break;
    case 3: fprintf(out, "by %s, %s and %s.\n", who[0], who[1], who[2]); break;
    case 4: fprintf(out, "by %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3]); break;
    case 5: fprintf(out, "by %s, %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3], who[4]); break;
    case 6: fprintf(out, "by %s, %s, %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3], who[4], who[5]); break;
    case 7: fprintf(out, "by %s, %s, %s, %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3], who[4], who[5], who[6]); break;
    case 8: fprintf(out, "by %s, %s, %s, %s, %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3], who[4], who[5], who[6], who[7]); break;
    case 9: fprintf(out, "by %s, %s, %s, %s, %s, %s, %s, %s and %s.\n", who[0], who[1], who[2], who[3], who[4], who[5], who[6], who[7], who[8]); break;
    default: fprintf(out, "by %s, %s, %s, %s, %s, %s, %s, %s, %s and others.\n", who[0], who[1], who[2], who[3], who[4], who[5], who[6], who[7], who[8]); break;
    }
}

static void credits_va(FILE *out, const char *cmd, const char *pkg, const char *ver, va_list who) {
    size_t n;
    const char *tab[10];
    for (n = 0; n < 10 && (tab[n] = va_arg(who, const char *)) != NULL; n++)
        ;
    credits_list(out, cmd, pkg, ver, tab, n);
}

__attribute__((noinline, noclone)) void credits(FILE *out, const char *cmd, const char *pkg, const char *ver, ...) {
    va_list who;
    va_start(who, ver);
    credits_va(out, cmd, pkg, ver, who);
    va_end(who);
}

int main(int argc, char **argv) {
    credits(stdout, "demo", "Demo Pkg", "1.0", "a", "b", argc > 2 ? argv[2] : NULL, NULL);
    return 0;
}
