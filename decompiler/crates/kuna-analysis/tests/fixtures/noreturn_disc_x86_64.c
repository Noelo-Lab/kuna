#include <stdio.h>
#include <stdlib.h>

/* A custom no-return wrapper the static name-lists do NOT know.
   It is no-return *in fact* because it tail-calls exit(), but it is
   NOT marked __attribute__((noreturn)). */
static void die(const char *m) {
    fprintf(stderr, "fatal: %s\n", m);
    exit(1);
}

int compute_a(int x) {
    if (x < 0)
        die("a: negative");
    return x * 2;
}

int compute_b(int x) {
    if (x == 0)
        die("b: zero");
    return 100 / x;
}

int compute_c(int x, int y) {
    if (y == 0)
        die("c: divide by zero");
    return x / y;
}

int compute_d(const char *s) {
    if (!s)
        die("d: null pointer");
    return (int) s[0];
}

int main(int argc, char **argv) {
    int a = compute_a(argc);
    int b = compute_b(argc + 1);
    int c = compute_c(a, b);
    int d = compute_d(argv[0]);
    printf("%d %d %d %d\n", a, b, c, d);
    return 0;
}
