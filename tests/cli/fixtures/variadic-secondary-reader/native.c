#include <limits.h>
#include <stdint.h>
#include <stdio.h>
extern uint64_t __attribute__((ms_abi)) caller_integer(int);
extern uint64_t __attribute__((ms_abi)) caller_stored_integer(int *);
extern uint64_t __attribute__((ms_abi)) caller_computed_integer(int *);
extern uint64_t __attribute__((ms_abi)) caller_stored_extra(int *);
int main(void) {
    const int values[] = {INT_MIN, -123, -1, 0, 1, 99, INT_MAX - 1};
    for (unsigned i = 0; i < sizeof(values) / sizeof(values[0]); ++i) {
        int x = values[i];
        if (caller_integer(x) != (uint32_t)x) return 1;
        int stored = x;
        if (caller_stored_integer(&stored) != (uint32_t)(x + 1) || stored != x + 1) return 2;
        if (caller_computed_integer(&x) != (uint32_t)(x + 1) || x != values[i]) return 3;
        int pair[] = {x, 7};
        if (caller_stored_extra(pair) != (uint32_t)(x + 1) || pair[0] != x + 1 || pair[1] != 9) return 4;
    }
    puts("native reference passed");
    return 0;
}
