#include <stdio.h>
int add(int a, int b) { return a + b; }
long sum_to(int n) {
    long acc = 0;
    for (int i = 1; i <= n; i++) acc += i;
    return acc;
}
int main(int argc, char **argv) {
    int x = add(argc, 3);
    long s = sum_to(x);
    printf("%ld\n", s);
    return (int)s;
}
