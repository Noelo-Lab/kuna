#include <stdint.h>
#include <stdbool.h>
extern unsigned value_calls;
uint64_t counted_unsigned(uint64_t value);
int64_t counted_signed(int64_t value);
uint8_t as_unsigned8(int8_t value) { return value; }
int8_t as_signed8(uint8_t value) { return value; }
uint8_t masked8(int8_t value) { return value & 0x55; }
uint8_t mixed_add8(uint8_t left, int8_t right) { return left + right; }
uint8_t mixed_sub8(uint8_t left, int8_t right) { return left - right; }
uint8_t mixed_and8(uint8_t left, int8_t right) { return left & right; }
uint8_t mixed_or8(uint8_t left, int8_t right) { return left | right; }
uint8_t mixed_xor8(uint8_t left, int8_t right) { return left ^ right; }
uint8_t mixed_multiply8(uint8_t left, int8_t right) { return left * right; }
int8_t inplace8(int8_t left, uint8_t right) { left += right; return left; }
uint16_t as_unsigned16(int16_t value) { return value; }
int16_t as_signed16(uint16_t value) { return value; }
uint16_t masked16(int16_t value) { return value & 0x55; }
uint16_t mixed_add16(uint16_t left, int16_t right) { return left + right; }
uint16_t mixed_sub16(uint16_t left, int16_t right) { return left - right; }
uint16_t mixed_and16(uint16_t left, int16_t right) { return left & right; }
uint16_t mixed_or16(uint16_t left, int16_t right) { return left | right; }
uint16_t mixed_xor16(uint16_t left, int16_t right) { return left ^ right; }
uint16_t mixed_multiply16(uint16_t left, int16_t right) { return left * right; }
int16_t inplace16(int16_t left, uint16_t right) { left += right; return left; }
uint32_t as_unsigned32(int32_t value) { return value; }
int32_t as_signed32(uint32_t value) { return value; }
uint32_t masked32(int32_t value) { return value & 0x55; }
uint32_t mixed_add32(uint32_t left, int32_t right) { return left + right; }
uint32_t mixed_sub32(uint32_t left, int32_t right) { return left - right; }
uint32_t mixed_and32(uint32_t left, int32_t right) { return left & right; }
uint32_t mixed_or32(uint32_t left, int32_t right) { return left | right; }
uint32_t mixed_xor32(uint32_t left, int32_t right) { return left ^ right; }
uint32_t mixed_multiply32(uint32_t left, int32_t right) { return left * right; }
int32_t inplace32(int32_t left, uint32_t right) { left += right; return left; }
uint64_t as_unsigned64(int64_t value) { return value; }
int64_t as_signed64(uint64_t value) { return value; }
uint64_t masked64(int64_t value) { return value & 0x55; }
uint64_t mixed_add64(uint64_t left, int64_t right) { return left + right; }
uint64_t mixed_sub64(uint64_t left, int64_t right) { return left - right; }
uint64_t mixed_and64(uint64_t left, int64_t right) { return left & right; }
uint64_t mixed_or64(uint64_t left, int64_t right) { return left | right; }
uint64_t mixed_xor64(uint64_t left, int64_t right) { return left ^ right; }
uint64_t mixed_multiply64(uint64_t left, int64_t right) { return left * right; }
int64_t inplace64(int64_t left, uint64_t right) { left += right; return left; }
int64_t branch_value(uint64_t left, uint64_t right, bool choose_left) { int64_t value; if (choose_left) value = left; else value = right; return value; }
int64_t call_unsigned(uint64_t value) { return counted_unsigned(value); }
uint64_t call_signed(int64_t value) { return counted_signed(value); }
uint64_t call_add(uint64_t left, int64_t right) { return counted_unsigned(left) + right; }
int64_t high_mask(uint64_t value) { return value & UINT64_C(0x7fffffffffffffff); }
uint64_t negative_mask(int64_t value) { return value & INT64_C(-129); }
double float_identity(double value) { return value; }
uint64_t * pointer_identity(uint64_t *value) { return value; }
bool bool_identity(bool value) { return value; }
int main(void) { return 0; }
