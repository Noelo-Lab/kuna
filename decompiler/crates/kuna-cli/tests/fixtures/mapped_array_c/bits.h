#include <stdint.h>
static inline uint64_t CONCAT24(uint16_t high, uint32_t low) {
    return ((uint64_t)high << 32) | low;
}
static inline uint64_t CONCAT26(uint16_t high, uint64_t low) {
    return ((uint64_t)high << 48) | (low & UINT64_C(0xffffffffffff));
}
static inline uint32_t SUB84(uint64_t value, unsigned byte_offset) {
    return value >> (8 * byte_offset);
}
static inline uint16_t SUB42(uint32_t value, unsigned byte_offset) {
    return value >> (8 * byte_offset);
}
