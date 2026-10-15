#include <stdint.h>

volatile uint64_t sink;

__attribute__((noinline)) uint64_t va(void *p) { return ((uint64_t *)p)[4] + ((uint64_t *)p)[0x1a]; }
__attribute__((noinline)) uint64_t vb(uint64_t *p) { return p[4] + p[0x1a]; }
__attribute__((noinline)) uint64_t vc(void *p, int i) { return ((uint64_t *)p)[i] + ((uint64_t *)p)[4]; }

void *g_table[8];
__attribute__((noinline)) uint64_t vd(int i) { return ((uint64_t *)g_table[i])[4]; }

int main(void) { sink = va(0) + vb(0) + vc(0, 1) + vd(2); return (int)sink; }
