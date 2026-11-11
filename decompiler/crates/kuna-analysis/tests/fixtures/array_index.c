#include <stdint.h>
#define NOINLINE __attribute__((noinline))
struct ArrayFields {uint64_t before;int8_t i8[9];uint8_t u8[9];int16_t i16[9];uint16_t u16[9];int32_t i32[9];uint32_t u32[9];int64_t i64[9];uint64_t u64[9];uint64_t after;};
NOINLINE int8_t load_i8_s(struct ArrayFields*p,int64_t i){return p->i8[i];}
NOINLINE void store_i8_s(struct ArrayFields*p,int64_t i,int8_t value){p->i8[i]=value;}
NOINLINE int8_t load_i8_u(struct ArrayFields*p,uint64_t i){return p->i8[i];}
NOINLINE void store_i8_u(struct ArrayFields*p,uint64_t i,int8_t value){p->i8[i]=value;}
NOINLINE uint8_t load_u8_s(struct ArrayFields*p,int64_t i){return p->u8[i];}
NOINLINE void store_u8_s(struct ArrayFields*p,int64_t i,uint8_t value){p->u8[i]=value;}
NOINLINE uint8_t load_u8_u(struct ArrayFields*p,uint64_t i){return p->u8[i];}
NOINLINE void store_u8_u(struct ArrayFields*p,uint64_t i,uint8_t value){p->u8[i]=value;}
NOINLINE int16_t load_i16_s(struct ArrayFields*p,int64_t i){return p->i16[i];}
NOINLINE void store_i16_s(struct ArrayFields*p,int64_t i,int16_t value){p->i16[i]=value;}
NOINLINE int16_t load_i16_u(struct ArrayFields*p,uint64_t i){return p->i16[i];}
NOINLINE void store_i16_u(struct ArrayFields*p,uint64_t i,int16_t value){p->i16[i]=value;}
NOINLINE uint16_t load_u16_s(struct ArrayFields*p,int64_t i){return p->u16[i];}
NOINLINE void store_u16_s(struct ArrayFields*p,int64_t i,uint16_t value){p->u16[i]=value;}
NOINLINE uint16_t load_u16_u(struct ArrayFields*p,uint64_t i){return p->u16[i];}
NOINLINE void store_u16_u(struct ArrayFields*p,uint64_t i,uint16_t value){p->u16[i]=value;}
NOINLINE int32_t load_i32_s(struct ArrayFields*p,int64_t i){return p->i32[i];}
NOINLINE void store_i32_s(struct ArrayFields*p,int64_t i,int32_t value){p->i32[i]=value;}
NOINLINE int32_t load_i32_u(struct ArrayFields*p,uint64_t i){return p->i32[i];}
NOINLINE void store_i32_u(struct ArrayFields*p,uint64_t i,int32_t value){p->i32[i]=value;}
NOINLINE uint32_t load_u32_s(struct ArrayFields*p,int64_t i){return p->u32[i];}
NOINLINE void store_u32_s(struct ArrayFields*p,int64_t i,uint32_t value){p->u32[i]=value;}
NOINLINE uint32_t load_u32_u(struct ArrayFields*p,uint64_t i){return p->u32[i];}
NOINLINE void store_u32_u(struct ArrayFields*p,uint64_t i,uint32_t value){p->u32[i]=value;}
NOINLINE int64_t load_i64_s(struct ArrayFields*p,int64_t i){return p->i64[i];}
NOINLINE void store_i64_s(struct ArrayFields*p,int64_t i,int64_t value){p->i64[i]=value;}
NOINLINE int64_t load_i64_u(struct ArrayFields*p,uint64_t i){return p->i64[i];}
NOINLINE void store_i64_u(struct ArrayFields*p,uint64_t i,int64_t value){p->i64[i]=value;}
NOINLINE uint64_t load_u64_s(struct ArrayFields*p,int64_t i){return p->u64[i];}
NOINLINE void store_u64_s(struct ArrayFields*p,int64_t i,uint64_t value){p->u64[i]=value;}
NOINLINE uint64_t load_u64_u(struct ArrayFields*p,uint64_t i){return p->u64[i];}
NOINLINE void store_u64_u(struct ArrayFields*p,uint64_t i,uint64_t value){p->u64[i]=value;}
NOINLINE int32_t constant_load(struct ArrayFields*p){return p->i32[3];}
NOINLINE uint64_t zero_load(struct ArrayFields*p){return p->u64[0];}
NOINLINE int32_t*array_address_s(struct ArrayFields*p,int64_t i){return &p->i32[i];}
NOINLINE int32_t*array_address_u(struct ArrayFields*p,uint64_t i){return &p->i32[i];}
int64_t counted_index(int64_t);struct ArrayFields*counted_base(struct ArrayFields*);int32_t counted_value(int32_t);
NOINLINE int32_t callback_load(struct ArrayFields*p,int64_t i){i=counted_index(i);p=counted_base(p);return p->i32[i];}
NOINLINE void callback_store(struct ArrayFields*p,int64_t i,int32_t value){value=counted_value(value);i=counted_index(i);p=counted_base(p);p->i32[i]=value;}
NOINLINE int32_t nested_load_s(int32_t(*p)[9],int64_t i,int64_t j){return p[i][j];}
NOINLINE void nested_store_s(int32_t(*p)[9],int64_t i,int64_t j,int32_t value){p[i][j]=value;}
NOINLINE int32_t nested_load_u(int32_t(*p)[9],int64_t i,uint64_t j){return p[i][j];}
NOINLINE void nested_store_u(int32_t(*p)[9],int64_t i,uint64_t j,int32_t value){p[i][j]=value;}
NOINLINE int32_t right_context(struct ArrayFields*p,uint64_t shift){return p->i32[UINT64_C(7)>>shift];}
NOINLINE int32_t signed_context(struct ArrayFields*p,uint64_t shift){return p->i32[(INT64_C(-8)>>shift)+8];}
NOINLINE int32_t left_context(struct ArrayFields*p,uint64_t shift){return p->i32[UINT64_C(1)<<(shift&3)];}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
