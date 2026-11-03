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
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
int main(void){uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(127),UINT64_C(128),UINT64_C(255),UINT64_C(256),UINT64_C(32767),UINT64_C(32768),UINT64_C(65535),UINT64_C(65536),UINT64_C(2147483647),UINT64_C(2147483648),UINT64_C(4294967295),UINT64_C(4294967296),UINT64_C(9223372036854775807),UINT64_C(9223372036854775808),UINT64_C(18446744073709551615)};for(unsigned i=0;i<sizeof(values)/sizeof(*values);i++){for(unsigned j=0;j<sizeof(values)/sizeof(*values);j++){uint64_t a=values[i],b=values[j];
{value_calls=0;uint64_t value=(uint64_t)as_unsigned8((int8_t)a);printf("as_unsigned8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_signed8((uint8_t)a);printf("as_signed8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)masked8((int8_t)a);printf("masked8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_add8((uint8_t)a,(int8_t)b);printf("mixed_add8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_sub8((uint8_t)a,(int8_t)b);printf("mixed_sub8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_and8((uint8_t)a,(int8_t)b);printf("mixed_and8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_or8((uint8_t)a,(int8_t)b);printf("mixed_or8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_xor8((uint8_t)a,(int8_t)b);printf("mixed_xor8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_multiply8((uint8_t)a,(int8_t)b);printf("mixed_multiply8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)inplace8((int8_t)a,(uint8_t)b);printf("inplace8 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_unsigned16((int16_t)a);printf("as_unsigned16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_signed16((uint16_t)a);printf("as_signed16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)masked16((int16_t)a);printf("masked16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_add16((uint16_t)a,(int16_t)b);printf("mixed_add16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_sub16((uint16_t)a,(int16_t)b);printf("mixed_sub16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_and16((uint16_t)a,(int16_t)b);printf("mixed_and16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_or16((uint16_t)a,(int16_t)b);printf("mixed_or16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_xor16((uint16_t)a,(int16_t)b);printf("mixed_xor16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_multiply16((uint16_t)a,(int16_t)b);printf("mixed_multiply16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)inplace16((int16_t)a,(uint16_t)b);printf("inplace16 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_unsigned32((int32_t)a);printf("as_unsigned32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_signed32((uint32_t)a);printf("as_signed32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)masked32((int32_t)a);printf("masked32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_add32((uint32_t)a,(int32_t)b);printf("mixed_add32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_sub32((uint32_t)a,(int32_t)b);printf("mixed_sub32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_and32((uint32_t)a,(int32_t)b);printf("mixed_and32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_or32((uint32_t)a,(int32_t)b);printf("mixed_or32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_xor32((uint32_t)a,(int32_t)b);printf("mixed_xor32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_multiply32((uint32_t)a,(int32_t)b);printf("mixed_multiply32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)inplace32((int32_t)a,(uint32_t)b);printf("inplace32 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_unsigned64((int64_t)a);printf("as_unsigned64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)as_signed64((uint64_t)a);printf("as_signed64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)masked64((int64_t)a);printf("masked64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_add64((uint64_t)a,(int64_t)b);printf("mixed_add64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_sub64((uint64_t)a,(int64_t)b);printf("mixed_sub64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_and64((uint64_t)a,(int64_t)b);printf("mixed_and64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_or64((uint64_t)a,(int64_t)b);printf("mixed_or64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_xor64((uint64_t)a,(int64_t)b);printf("mixed_xor64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)mixed_multiply64((uint64_t)a,(int64_t)b);printf("mixed_multiply64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)inplace64((int64_t)a,(uint64_t)b);printf("inplace64 %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)branch_value(a,b,(a&1)!=0);printf("branch_value %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)call_unsigned((uint64_t)a);printf("call_unsigned %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)call_signed((int64_t)a);printf("call_signed %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)call_add((uint64_t)a,(int64_t)b);printf("call_add %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)high_mask((uint64_t)a);printf("high_mask %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;uint64_t value=(uint64_t)negative_mask((int64_t)a);printf("negative_mask %" PRIu64 " %u\n",value,value_calls);}
{value_calls=0;double input,value;uint64_t bits;memcpy(&input,&a,8);value=float_identity(input);memcpy(&bits,&value,8);printf("float_identity %" PRIu64 " %u\n",bits,value_calls);}
{value_calls=0;uint64_t cell=a,*value=pointer_identity(&cell);printf("pointer_identity %" PRIu64 " %u %u\n",*value,(unsigned)(value==&cell),value_calls);}
{value_calls=0;bool value=bool_identity((a&1)!=0);printf("bool_identity %u %u\n",(unsigned)value,value_calls);}
}}return 0;}
