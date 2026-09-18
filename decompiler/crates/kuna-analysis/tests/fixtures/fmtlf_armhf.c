/* arm-linux-gnueabihf-gcc -O2 -marm -o fmtlf_armhf fmtlf_armhf.c
 * l has no effect on a floating printf conversion: %lf prints a double. For
 * scanf it selects a double destination. */
#include <stdio.h>
__attribute__((noinline)) void show(double d, int n) { printf("value=%lf n=%d\n", d, n); }
__attribute__((noinline)) void show2(int n, double d) { printf("n=%d v=%lf\n", n, d); }
__attribute__((noinline)) double rd(const char *s) { double d = 0; sscanf(s, "%lf", &d); return d; }
int main(int argc, char **argv) {
  show(argc * 0.5, argc);
  show2(argc, argc * 2.0);
  return (int)rd(argv[0]);
}
