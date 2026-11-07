#include <stdint.h>
uint64_t trace;
__attribute__((noinline)) __uint128_t *counted_pointer(__uint128_t *p) { trace=trace*100+1;return p; }
