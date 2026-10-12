#include <stdint.h>
#include <stdio.h>
#include <string.h>

extern float widened_update(float, float, float);
extern void packed_update(float *, float *, float *, float *);
extern float float_update(float, float, float);
extern float widen_roundtrip(float);
extern float widened_product(float, float, float);
extern float widened_quotient(float, float, float);
#ifdef ROUNDTRIP
extern float printed_widened_update(float, float, float);
extern void printed_packed_update(float *, float *, float *, float *);
extern float printed_float_update(float, float, float);
extern float printed_widen_roundtrip(float);
extern float printed_widened_product(float, float, float);
extern float printed_widened_quotient(float, float, float);
#endif

static uint32_t bits(float value) {
    uint32_t result;
    memcpy(&result, &value, sizeof result);
    return result;
}

static float value(uint32_t pattern) {
    float result;
    memcpy(&result, &pattern, sizeof result);
    return result;
}

#ifdef ROUNDTRIP
static int mismatch(const char *name, float native, float printed) {
    if (bits(native) == bits(printed)) return 0;
    fprintf(stderr, "%s: native %08x, printed %08x\n", name, bits(native), bits(printed));
    return 1;
}
#endif

static int check(float a, float b, float c) {
    float accum[2] = {a, c}, theta[2] = {b, b}, axis[2] = {c, a}, out[2];
    packed_update(out, accum, theta, axis);
    if (bits(out[0]) != bits(widened_update(a, b, c)) ||
        bits(out[1]) != bits(widened_update(c, b, a))) return 1;
#ifdef ROUNDTRIP
    float printed[2];
    printed_packed_update(printed, accum, theta, axis);
    if (mismatch("scalar", out[0], printed_widened_update(a, b, c)) ||
        mismatch("packed[0]", out[0], printed[0]) ||
        mismatch("packed[1]", out[1], printed[1]) ||
        mismatch("float", float_update(a, b, c), printed_float_update(a, b, c)) ||
        mismatch("conversion", widen_roundtrip(a), printed_widen_roundtrip(a)) ||
        mismatch("product", widened_product(a, b, c), printed_widened_product(a, b, c)) ||
        mismatch("quotient", widened_quotient(a, b, c), printed_widened_quotient(a, b, c))) {
        fprintf(stderr, "inputs %08x %08x %08x\n", bits(a), bits(b), bits(c));
        return 1;
    }
#endif
    return 0;
}

int main(void) {
    float a = 0.00001f, b = 0.000001f;
    if (bits(widened_update(a, b, a)) != 0x358637bd ||
        bits(float_update(a, b, a)) != 0x358637c0 || check(a, b, a)) return 1;
    const uint32_t finite[] = {
        0, 0x80000000, 1, 0x80000001, 0x007fffff, 0x00800000,
        0x3f800000, 0xbf800000, 0x4b800000, 0x7f7fffff, 0xff7fffff
    };
    for (unsigned i = 0; i < sizeof finite / sizeof *finite; ++i)
        for (unsigned j = 0; j < sizeof finite / sizeof *finite; ++j)
            for (unsigned k = 0; k < sizeof finite / sizeof *finite; ++k)
                if (check(value(finite[i]), value(finite[j]), value(finite[k]))) return 1;
    const uint32_t special[] = {0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321};
    for (unsigned i = 0; i < sizeof special / sizeof *special; ++i) {
        float s = value(special[i]);
        if (check(s, b, a) || check(a, s, a) || check(a, b, s)) return 1;
    }
    uint32_t seed = 0x12345678;
    for (unsigned i = 0; i < 10000; ++i) {
        float inputs[3];
        for (unsigned j = 0; j < 3; ++j) {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            inputs[j] = value(seed & 0xfeffffff);
        }
        if (check(inputs[0], inputs[1], inputs[2])) return 1;
    }
    return 0;
}
