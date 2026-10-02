/* A value a function returns in a float register, and a result a caller
   reads from a function that computes it in a callee.
   clang -O0 -o floatret_x86_64 floatret_x86_64.c -lm && strip floatret_x86_64 */
#include <math.h>
#include <stdio.h>

float gf = 1.5f;
double gd = 2.25;
int gi = 21;

__attribute__((noinline)) float qnan(void) { return nanf(""); }
__attribute__((noinline)) float pick(float x) { return x < 0 ? qnan() : x * 2; }
__attribute__((noinline)) float getf(void) { return gf; }
__attribute__((noinline)) double getd(void) { return gd; }
__attribute__((noinline)) double wrapd(void) { return getd(); }
__attribute__((noinline)) float idf(float x) { return x; }
__attribute__((noinline)) int geti(void) { return gi * 2; }
__attribute__((noinline)) int wrapi(void) { return geti(); }

int main(int argc, char **argv) {
  (void)argv;
  printf("%f %f %f %f %f %f %d\n", pick(3.0f), pick(-3.0f), getf() * 2, getd() + 1, wrapd() * 3,
         idf((float)argc + 0.25f), wrapi());
  return 0;
}
