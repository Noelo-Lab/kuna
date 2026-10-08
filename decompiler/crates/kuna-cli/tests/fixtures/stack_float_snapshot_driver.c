#include <stdint.h>
#include <string.h>

typedef float float4;
typedef int int4;
typedef int8_t int1;
typedef int16_t int2;
typedef int64_t int8;
typedef uint8_t uint1;
typedef uint16_t uint2;
typedef uint32_t uint4;
typedef uint64_t uint8;
#ifdef KUNA_NATIVE
typedef struct FloatPair { float x; float y; } FloatPair;
#else
typedef uint8_t undefined1;
/* KUNA_EMITTED_TYPES */
#endif

#ifdef KUNA_NATIVE
float4 flip_first_and_add(FloatPair *source, float4 bias, int4 flip);
float4 flip_second_and_add(FloatPair *source, float4 bias, int4 flip);
float4 integer_to_float(int4 value);
#endif
void copy_point(FloatPair *destination, const FloatPair *source);
uint4 observe_word(const uint4 *source);

#ifndef KUNA_NATIVE
/* KUNA_EMITTED_CODE */
#endif

#ifndef KUNA_NATIVE
void copy_point(FloatPair *destination, const FloatPair *source) {
    *destination = *source;
}

uint4 observe_word(const uint4 *source) {
    uint4 value;
    memcpy(&value, source, sizeof value);
    return value;
}
#endif

static uint32_t bits(float value) {
    uint32_t result;
    memcpy(&result, &value, sizeof result);
    return result;
}

static float from_bits(uint32_t value) {
    float result;
    memcpy(&result, &value, sizeof result);
    return result;
}

int main(void) {
    FloatPair point = {1.5f, 1.5f};
    if (bits(flip_first_and_add(&point, 2.25f, 1)) != UINT32_C(0x3f400000)) return 1;
    if (bits(flip_second_and_add(&point, 2.25f, 1)) != UINT32_C(0x3f400000)) return 2;

    point.x = from_bits(UINT32_C(0x80000000));
    if (bits(flip_first_and_add(&point, from_bits(UINT32_C(0x80000000)), 0)) != UINT32_C(0x80000000)) return 3;
    point.y = from_bits(UINT32_C(0x80000000));
    if (bits(flip_second_and_add(&point, from_bits(UINT32_C(0x80000000)), 0)) != UINT32_C(0x80000000)) return 4;

    point.x = from_bits(UINT32_C(0x7fc12345));
    if (bits(flip_first_and_add(&point, 1.0f, 0)) != UINT32_C(0x7fc12345)) return 5;
    point.y = from_bits(UINT32_C(0x7fc12345));
    if (bits(flip_second_and_add(&point, 1.0f, 0)) != UINT32_C(0x7fc12345)) return 6;

    if (integer_to_float((int4)UINT32_C(0x3f800000)) != 1065353216.0f) return 7;
    return 0;
}
