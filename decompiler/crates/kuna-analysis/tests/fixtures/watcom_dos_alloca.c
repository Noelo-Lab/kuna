#include <malloc.h>
unsigned observe(unsigned char *p, unsigned n) { return p[0] + n; }
unsigned sample_one(unsigned n) { unsigned char *p = alloca(n); p[0] = 7; return observe(p, n); }
int main(void) { return sample_one(3) != 10; }
