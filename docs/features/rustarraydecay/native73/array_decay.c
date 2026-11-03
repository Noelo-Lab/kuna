#include <stdint.h>
#define N __attribute__((noinline))
struct DecayFields {uint64_t before;int8_t i8[9];uint8_t u8[9];int16_t i16[9];uint16_t u16[9];int32_t i32[9];uint32_t u32[9];int64_t i64[9];uint64_t u64[9];float float32[9];double float64[9];uint8_t* pointers[9];_Bool flags[9];uint64_t after;};
int8_t*pass_value_i8(int8_t*);
N int8_t*field_i8(struct DecayFields*p){return p->i8;}
N int8_t*whole_i8(int8_t(*p)[9]){return *p;}
N int8_t*end_i8(int8_t(*p)[9]){return &(*p)[9];}
N void*erased_i8(struct DecayFields*p){return (void*)p->i8;}
N void stored_i8(int8_t**out,struct DecayFields*p){*out=p->i8;}
N int8_t*passed_i8(struct DecayFields*p){return pass_value_i8(p->i8);}
uint8_t*pass_value_u8(uint8_t*);
N uint8_t*field_u8(struct DecayFields*p){return p->u8;}
N uint8_t*whole_u8(uint8_t(*p)[9]){return *p;}
N uint8_t*end_u8(uint8_t(*p)[9]){return &(*p)[9];}
N void*erased_u8(struct DecayFields*p){return (void*)p->u8;}
N void stored_u8(uint8_t**out,struct DecayFields*p){*out=p->u8;}
N uint8_t*passed_u8(struct DecayFields*p){return pass_value_u8(p->u8);}
int16_t*pass_value_i16(int16_t*);
N int16_t*field_i16(struct DecayFields*p){return p->i16;}
N int16_t*whole_i16(int16_t(*p)[9]){return *p;}
N int16_t*end_i16(int16_t(*p)[9]){return &(*p)[9];}
N void*erased_i16(struct DecayFields*p){return (void*)p->i16;}
N void stored_i16(int16_t**out,struct DecayFields*p){*out=p->i16;}
N int16_t*passed_i16(struct DecayFields*p){return pass_value_i16(p->i16);}
uint16_t*pass_value_u16(uint16_t*);
N uint16_t*field_u16(struct DecayFields*p){return p->u16;}
N uint16_t*whole_u16(uint16_t(*p)[9]){return *p;}
N uint16_t*end_u16(uint16_t(*p)[9]){return &(*p)[9];}
N void*erased_u16(struct DecayFields*p){return (void*)p->u16;}
N void stored_u16(uint16_t**out,struct DecayFields*p){*out=p->u16;}
N uint16_t*passed_u16(struct DecayFields*p){return pass_value_u16(p->u16);}
int32_t*pass_value_i32(int32_t*);
N int32_t*field_i32(struct DecayFields*p){return p->i32;}
N int32_t*whole_i32(int32_t(*p)[9]){return *p;}
N int32_t*end_i32(int32_t(*p)[9]){return &(*p)[9];}
N void*erased_i32(struct DecayFields*p){return (void*)p->i32;}
N void stored_i32(int32_t**out,struct DecayFields*p){*out=p->i32;}
N int32_t*passed_i32(struct DecayFields*p){return pass_value_i32(p->i32);}
uint32_t*pass_value_u32(uint32_t*);
N uint32_t*field_u32(struct DecayFields*p){return p->u32;}
N uint32_t*whole_u32(uint32_t(*p)[9]){return *p;}
N uint32_t*end_u32(uint32_t(*p)[9]){return &(*p)[9];}
N void*erased_u32(struct DecayFields*p){return (void*)p->u32;}
N void stored_u32(uint32_t**out,struct DecayFields*p){*out=p->u32;}
N uint32_t*passed_u32(struct DecayFields*p){return pass_value_u32(p->u32);}
int64_t*pass_value_i64(int64_t*);
N int64_t*field_i64(struct DecayFields*p){return p->i64;}
N int64_t*whole_i64(int64_t(*p)[9]){return *p;}
N int64_t*end_i64(int64_t(*p)[9]){return &(*p)[9];}
N void*erased_i64(struct DecayFields*p){return (void*)p->i64;}
N void stored_i64(int64_t**out,struct DecayFields*p){*out=p->i64;}
N int64_t*passed_i64(struct DecayFields*p){return pass_value_i64(p->i64);}
uint64_t*pass_value_u64(uint64_t*);
N uint64_t*field_u64(struct DecayFields*p){return p->u64;}
N uint64_t*whole_u64(uint64_t(*p)[9]){return *p;}
N uint64_t*end_u64(uint64_t(*p)[9]){return &(*p)[9];}
N void*erased_u64(struct DecayFields*p){return (void*)p->u64;}
N void stored_u64(uint64_t**out,struct DecayFields*p){*out=p->u64;}
N uint64_t*passed_u64(struct DecayFields*p){return pass_value_u64(p->u64);}
float*pass_value_float32(float*);
N float*field_float32(struct DecayFields*p){return p->float32;}
N float*whole_float32(float(*p)[9]){return *p;}
N float*end_float32(float(*p)[9]){return &(*p)[9];}
N void*erased_float32(struct DecayFields*p){return (void*)p->float32;}
N void stored_float32(float**out,struct DecayFields*p){*out=p->float32;}
N float*passed_float32(struct DecayFields*p){return pass_value_float32(p->float32);}
double*pass_value_float64(double*);
N double*field_float64(struct DecayFields*p){return p->float64;}
N double*whole_float64(double(*p)[9]){return *p;}
N double*end_float64(double(*p)[9]){return &(*p)[9];}
N void*erased_float64(struct DecayFields*p){return (void*)p->float64;}
N void stored_float64(double**out,struct DecayFields*p){*out=p->float64;}
N double*passed_float64(struct DecayFields*p){return pass_value_float64(p->float64);}
uint8_t**pass_value_pointers(uint8_t**);
N uint8_t**field_pointers(struct DecayFields*p){return p->pointers;}
N uint8_t**whole_pointers(uint8_t*(*p)[9]){return *p;}
N uint8_t**end_pointers(uint8_t*(*p)[9]){return &(*p)[9];}
N void*erased_pointers(struct DecayFields*p){return (void*)p->pointers;}
N void stored_pointers(uint8_t***out,struct DecayFields*p){*out=p->pointers;}
N uint8_t**passed_pointers(struct DecayFields*p){return pass_value_pointers(p->pointers);}
_Bool*pass_value_flags(_Bool*);
N _Bool*field_flags(struct DecayFields*p){return p->flags;}
N _Bool*whole_flags(_Bool(*p)[9]){return *p;}
N _Bool*end_flags(_Bool(*p)[9]){return &(*p)[9];}
N void*erased_flags(struct DecayFields*p){return (void*)p->flags;}
N void stored_flags(_Bool**out,struct DecayFields*p){*out=p->flags;}
N _Bool*passed_flags(struct DecayFields*p){return pass_value_flags(p->flags);}
struct DecayFields*counted_base(struct DecayFields*);
N int32_t*passed_counted(struct DecayFields*p){return pass_value_i32(counted_base(p)->i32);}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
