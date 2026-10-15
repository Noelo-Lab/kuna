#include <assert.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>

extern double rounded32(int value);
extern float narrowed64(long value);
extern double widened16(short value);
extern float exact32(int value);
extern int trunc_after_narrow(int value, float *out);
#ifdef ROUNDTRIP
extern double printed_rounded32(int value);
extern float printed_narrowed64(long value);
extern double printed_widened16(short value);
extern float printed_exact32(int value);
extern int printed_trunc_after_narrow(int value, float *out);
#endif

static uint32_t bits(float value) {
    uint32_t result;
    memcpy(&result, &value, sizeof(result));
    return result;
}

int main(void) {
    const int values[] = {0, 1, -1, 16777217, -16777217, INT_MAX, INT_MIN};
    for (unsigned i = 0; i < sizeof(values) / sizeof(values[0]); ++i) {
        volatile int value = values[i];
        assert(rounded32(value) == (double)(float)value);
        assert(bits(exact32(value)) == bits((float)value));
        float output;
        assert(trunc_after_narrow(value, &output) == value);
        assert(bits(output) == bits((float)value));
#ifdef ROUNDTRIP
        assert(printed_rounded32(value) == rounded32(value));
        assert(bits(printed_exact32(value)) == bits(exact32(value)));
        float printed_output;
        assert(printed_trunc_after_narrow(value, &printed_output) == value);
        assert(bits(printed_output) == bits(output));
#endif
    }
    const long large[] = {0, 1, -1, 4611686293305294849LL, -4611686293305294849LL, LONG_MAX, LONG_MIN};
    for (unsigned i = 0; i < sizeof(large) / sizeof(large[0]); ++i) {
        volatile long value = large[i];
        assert(bits(narrowed64(value)) == bits((float)(double)value));
#ifdef ROUNDTRIP
        assert(bits(printed_narrowed64(value)) == bits(narrowed64(value)));
#endif
    }
    const short small[] = {0, 1, -1, SHRT_MAX, SHRT_MIN};
    for (unsigned i = 0; i < sizeof(small) / sizeof(small[0]); ++i) {
        assert(widened16(small[i]) == (double)small[i]);
#ifdef ROUNDTRIP
        assert(printed_widened16(small[i]) == widened16(small[i]));
#endif
    }
    assert(rounded32(16777217) == 16777216.0);
    assert(bits(narrowed64(4611686293305294849LL)) == UINT32_C(0x5e800000));
}
