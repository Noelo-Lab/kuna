#include <stdint.h>
int64_t logical_min(uint64_t n) { return UINT64_C(0x8000000000000000) >> (n & 63); }
int64_t logical_max(uint64_t n) { return UINT64_MAX >> (n & 63); }
uint64_t arithmetic_min(uint64_t n) { return INT64_MIN >> (n & 63); }
uint64_t arithmetic_negative(uint64_t n) { return INT64_C(-3) >> (n & 63); }
#ifdef KUNA_NATIVE_MAIN
#include <stdio.h>
#include <inttypes.h>
int main(void) {
    for (uint64_t n = 0; n < 64; ++n) {
        printf("%016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 "\n",
            (uint64_t)logical_min(n), (uint64_t)logical_max(n), arithmetic_min(n), arithmetic_negative(n));
    }
    return 0;
}
#else
int main(void) { return 0; }
#endif
