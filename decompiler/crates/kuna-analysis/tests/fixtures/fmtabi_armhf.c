/* arm-linux-gnueabihf-gcc -O2 -marm -o fmtabi_armhf fmtabi_armhf.c
 * A floating vararg travels in core registers on ARM hard-float, a named
 * double in d0: a closed printf prototype must not claim it. */
#include <stdio.h>
__attribute__((noinline)) void f_conv(int x, double a) { printf("x=%f\n", (double)x); }
__attribute__((noinline)) void f_sum(double a, double b) { printf("sum=%f a=%f\n", a + b, a); }
__attribute__((noinline)) void f_is(int i, const char *s) { printf("%d %s\n", i, s); }
int main(int argc, char **argv) {
  f_conv(argc, 9.0);
  f_sum(argc, 0.5);
  f_is(argc, argv[0]);
  return 0;
}
