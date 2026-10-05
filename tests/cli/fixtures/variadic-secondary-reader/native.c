#include <limits.h>
#include <stdint.h>
#include <stdio.h>
extern uint64_t __attribute__((ms_abi)) caller_integer(int);
extern uint64_t __attribute__((ms_abi)) caller_stored_integer(int *);
extern uint64_t __attribute__((ms_abi)) caller_imported_integer(int *);
extern uint64_t __attribute__((ms_abi)) caller_computed_integer(int *);
extern uint64_t __attribute__((ms_abi)) caller_stored_extra(int *);
extern uint64_t __attribute__((ms_abi)) caller_stored_two(int *);
extern uint64_t __attribute__((ms_abi)) caller_clamped_integer(int *,int);
extern uint64_t __attribute__((ms_abi)) caller_clamped_call(int *,int *);
extern uint64_t __attribute__((ms_abi)) caller_stored_comparison(int *,int);
extern uint64_t __attribute__((ms_abi)) caller_stored_boolean(int *,int);
int main(void) {
    const int values[] = {INT_MIN, -123, -1, 0, 1, 99, INT_MAX - 1};
    for (unsigned i = 0; i < sizeof(values) / sizeof(values[0]); ++i) {
        int x = values[i];
        if (caller_integer(x) != (uint32_t)x) return 1;
        int stored = x;
        if (caller_stored_integer(&stored) != (uint32_t)(x + 1) || stored != x + 1) return 2;
        stored = x;
        if (caller_imported_integer(&stored) != (uint32_t)(x + 1) || stored != x + 1) return 10;
        if (caller_computed_integer(&x) != (uint32_t)(x + 1) || x != values[i]) return 3;
        int pair[] = {x, 7};
        if (caller_stored_extra(pair) != (uint32_t)(x + 1) || pair[0] != x + 1 || pair[1] != 9) return 4;
        int two[] = {x, -123};
        uint64_t packed = ((uint64_t)(uint32_t)-121 << 32) | (uint32_t)(x + 1);
        if (caller_stored_two(two) != packed || two[0] != x + 1 || two[1] != -121) return 5;
    }
    const int selected[] = {INT_MIN+1,-123,-1,0,1,5,6,7,11,12,13,99,INT_MAX-1};
    const int tags[] = {INT_MIN,-1,0,1,2,INT_MAX};
    for (unsigned i = 0; i < sizeof(selected)/sizeof(selected[0]); ++i) {
        for (unsigned j = 0; j < sizeof(tags)/sizeof(tags[0]); ++j) {
            int value = selected[i], tag = tags[j];
            int expected = value + (tag == 1 ? 1 : -1);
            if (expected < 6) expected = 6;
            if (expected > 12) expected = 12;
            if (caller_clamped_integer(&value,tag) != (uint32_t)expected || value != expected) return 6;
            value = selected[i];
            if (caller_clamped_call(&value,&tag) != (uint32_t)expected || value != expected || tag != tags[j]) return 7;
            value = selected[i]; expected = value + (tag == 1);
            if (caller_stored_comparison(&value,tag) != (uint32_t)expected || value != expected) return 8;
            value = selected[i]; expected = value + ((tag < 0) != (tag == 1));
            if (caller_stored_boolean(&value,tag) != (uint32_t)expected || value != expected) return 9;
        }
    }
    puts("native reference passed");
    return 0;
}
