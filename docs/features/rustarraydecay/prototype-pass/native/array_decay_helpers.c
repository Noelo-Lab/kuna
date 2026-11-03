#include <stdint.h>
#define N __attribute__((noinline))
struct DecayFields;uint64_t trace=0;
N struct DecayFields*counted_base(struct DecayFields*p){trace=trace*100+1;return p;}
N int8_t*pass_value_i8(int8_t*p){trace=trace*100+1;return p;}
N uint8_t*pass_value_u8(uint8_t*p){trace=trace*100+2;return p;}
N int16_t*pass_value_i16(int16_t*p){trace=trace*100+3;return p;}
N uint16_t*pass_value_u16(uint16_t*p){trace=trace*100+4;return p;}
N int32_t*pass_value_i32(int32_t*p){trace=trace*100+5;return p;}
N uint32_t*pass_value_u32(uint32_t*p){trace=trace*100+6;return p;}
N int64_t*pass_value_i64(int64_t*p){trace=trace*100+7;return p;}
N uint64_t*pass_value_u64(uint64_t*p){trace=trace*100+8;return p;}
N float*pass_value_float32(float*p){trace=trace*100+9;return p;}
N double*pass_value_float64(double*p){trace=trace*100+10;return p;}
N uint8_t**pass_value_pointers(uint8_t**p){trace=trace*100+11;return p;}
N _Bool*pass_value_flags(_Bool*p){trace=trace*100+12;return p;}
