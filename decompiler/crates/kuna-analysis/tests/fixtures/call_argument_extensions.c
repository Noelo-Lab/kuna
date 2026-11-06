#include <stdint.h>

__attribute__((noinline)) uint64_t consume(uint64_t value) {
    __asm__ volatile("" : "+r"(value));
    return value;
}

__attribute__((noinline)) int64_t consume_signed(int64_t value) {
    __asm__ volatile("" : "+r"(value));
    return value >> 1;
}

__attribute__((noinline)) uint64_t consume_store(uint64_t *destination, uint64_t value) {
    *destination = value;
    return value;
}

__attribute__((noinline)) uint64_t pass_u8(uint64_t value) { return consume((uint8_t)value); }
__attribute__((noinline)) uint64_t pass_u16(uint64_t value) { return consume((uint16_t)value); }
__attribute__((noinline)) uint64_t pass_u32(uint64_t value) { return consume((uint32_t)value); }
__attribute__((noinline)) uint64_t pass_signed(int32_t value) { return consume((uint32_t)value); }
__attribute__((noinline)) uint64_t keep_signed(int32_t value) { return consume(value); }
__attribute__((noinline)) uint64_t same_width(uint64_t value) { return consume(value); }

__attribute__((noinline)) int64_t signed_u8(uint64_t value) { return consume_signed((uint8_t)value); }
__attribute__((noinline)) int64_t signed_u16(uint64_t value) { return consume_signed((uint16_t)value); }
__attribute__((noinline)) int64_t signed_u32(uint64_t value) { return consume_signed((uint32_t)value); }

__attribute__((noinline)) uint64_t store_u8(uint64_t *destination, uint64_t value) {
    return consume_store(destination, (uint8_t)value);
}
__attribute__((noinline)) uint64_t store_u16(uint64_t *destination, uint64_t value) {
    return consume_store(destination, (uint16_t)value);
}
__attribute__((noinline)) uint64_t store_u32(uint64_t *destination, uint64_t value) {
    return consume_store(destination, (uint32_t)value);
}
__attribute__((noinline)) uint64_t store_signed(uint64_t *destination, int32_t value) {
    return consume_store(destination, (uint32_t)value);
}

__attribute__((noinline)) uint64_t pass_signed8(int8_t value) { return consume((uint8_t)value); }
__attribute__((noinline)) uint64_t pass_signed16(int16_t value) { return consume((uint16_t)value); }
__attribute__((noinline)) uint64_t store_signed8(uint64_t *destination, int8_t value) {
    return consume_store(destination, (uint8_t)value);
}
__attribute__((noinline)) uint64_t store_signed16(uint64_t *destination, int16_t value) {
    return consume_store(destination, (uint16_t)value);
}

int main(void) { return 0; }
