/* formatstring: outgoing stack argument slots around a resolved format call.
 *
 * read_int, read_chars: clang keeps a scanf destination in the slot its
 * `push rax` makes, which at the call is the first outgoing stack argument
 * slot. sscanf writes it through the pointer it is passed, so what the
 * function returns is the value read.
 *
 * show9: the ninth double goes on the stack, behind the four integer
 * registers the call leaves unused; it is still an argument.
 *
 * clang -O2 -o fmtslots_x86_64 fmtslots_x86_64.c  (clang 14, x86-64)
 */
#include <stdio.h>

__attribute__((noinline)) int read_int(const char *s) {
    int a = 5;
    sscanf(s, "%d", &a);
    return a + 1;
}

__attribute__((noinline)) int read_chars(const char *s) {
    char c = 0, d = 0;
    sscanf(s, "%c %c", &c, &d);
    return c * 3 + d;
}

__attribute__((noinline)) void show9(double a, double b, double c, double d, double e, double f,
                                     double g, double h, double i, int n) {
    printf("%f %f %f %f %f %f %f %f %f %d\n", a, b, c, d, e, f, g, h, i, n);
    puts("done");
}

int main(int argc, char **argv) {
    show9(1, 2, 3, 4, 5, 6, 7, 8, argc, argc);
    return read_int(argv[0]) + read_chars(argv[argc - 1]);
}
