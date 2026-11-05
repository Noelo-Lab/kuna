#include <stdbool.h>
#include <stdint.h>

__attribute__((noinline)) uint64_t wide_value(uint64_t value) {
    __asm__ volatile("" : "+r"(value));
    return value;
}

#define NARROW(name, type) \
    __attribute__((noinline)) type name(bool skip, uint64_t value) { \
        type result = 0; \
        if (!skip) result = wide_value(value); \
        return result; \
    }

NARROW(narrow_u8, uint8_t)
NARROW(narrow_i8, int8_t)
NARROW(narrow_u16, uint16_t)
NARROW(narrow_i16, int16_t)
NARROW(narrow_u32, uint32_t)
NARROW(narrow_i32, int32_t)
NARROW(narrow_bool, uint8_t)
NARROW(full_width, uint64_t)

__attribute__((noinline)) uint64_t wide_shift(uint64_t value) {
    __asm__ volatile("" : "+r"(value));
    return value >> 1;
}

__attribute__((noinline)) uint8_t shifted_bool(bool skip, uint64_t value) {
    uint8_t result = 0;
    if (!skip) result = wide_shift(value);
    return result;
}

int main(void) { return 0; }
