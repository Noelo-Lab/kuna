#include <stdint.h>
unsigned value_calls;
uint64_t counted_unsigned(uint64_t value) { value_calls++; return value; }
int64_t counted_signed(int64_t value) { value_calls++; return value; }
