#include <stdint.h>
#define NOINLINE __attribute__((noinline))
struct AddressFields {uint64_t before;int8_t i8[9];uint8_t u8[9];int16_t i16[9];uint16_t u16[9];int32_t i32[9];uint32_t u32[9];int64_t i64[9];uint64_t u64[9];float float32[9];double float64[9];uint8_t* pointers[9];_Bool flags[9];uint64_t after;};
NOINLINE int8_t*address_i8_s(struct AddressFields*p,int64_t i){return &p->i8[i];}
NOINLINE int8_t*address_i8_u(struct AddressFields*p,uint64_t i){return &p->i8[i];}
NOINLINE uint8_t*address_u8_s(struct AddressFields*p,int64_t i){return &p->u8[i];}
NOINLINE uint8_t*address_u8_u(struct AddressFields*p,uint64_t i){return &p->u8[i];}
NOINLINE int16_t*address_i16_s(struct AddressFields*p,int64_t i){return &p->i16[i];}
NOINLINE int16_t*address_i16_u(struct AddressFields*p,uint64_t i){return &p->i16[i];}
NOINLINE uint16_t*address_u16_s(struct AddressFields*p,int64_t i){return &p->u16[i];}
NOINLINE uint16_t*address_u16_u(struct AddressFields*p,uint64_t i){return &p->u16[i];}
NOINLINE int32_t*address_i32_s(struct AddressFields*p,int64_t i){return &p->i32[i];}
NOINLINE int32_t*address_i32_u(struct AddressFields*p,uint64_t i){return &p->i32[i];}
NOINLINE uint32_t*address_u32_s(struct AddressFields*p,int64_t i){return &p->u32[i];}
NOINLINE uint32_t*address_u32_u(struct AddressFields*p,uint64_t i){return &p->u32[i];}
NOINLINE int64_t*address_i64_s(struct AddressFields*p,int64_t i){return &p->i64[i];}
NOINLINE int64_t*address_i64_u(struct AddressFields*p,uint64_t i){return &p->i64[i];}
NOINLINE uint64_t*address_u64_s(struct AddressFields*p,int64_t i){return &p->u64[i];}
NOINLINE uint64_t*address_u64_u(struct AddressFields*p,uint64_t i){return &p->u64[i];}
NOINLINE float*address_float32_s(struct AddressFields*p,int64_t i){return &p->float32[i];}
NOINLINE float*address_float32_u(struct AddressFields*p,uint64_t i){return &p->float32[i];}
NOINLINE double*address_float64_s(struct AddressFields*p,int64_t i){return &p->float64[i];}
NOINLINE double*address_float64_u(struct AddressFields*p,uint64_t i){return &p->float64[i];}
NOINLINE uint8_t**address_pointers_s(struct AddressFields*p,int64_t i){return &p->pointers[i];}
NOINLINE uint8_t**address_pointers_u(struct AddressFields*p,uint64_t i){return &p->pointers[i];}
NOINLINE _Bool*address_flags_s(struct AddressFields*p,int64_t i){return &p->flags[i];}
NOINLINE _Bool*address_flags_u(struct AddressFields*p,uint64_t i){return &p->flags[i];}
NOINLINE int32_t*nested_address_s(int32_t(*p)[9],int64_t i,int64_t j){return &p[i][j];}
NOINLINE int32_t*nested_address_u(int32_t(*p)[9],int64_t i,uint64_t j){return &p[i][j];}
int64_t counted_index(int64_t);struct AddressFields*counted_base(struct AddressFields*);
NOINLINE int32_t*callback_address(struct AddressFields*p,int64_t i){i=counted_index(i);p=counted_base(p);return &p->i32[i];}
NOINLINE int32_t*wrap_signed(struct AddressFields*p,int64_t i){return (int32_t*)((uintptr_t)&p->i32[0]+(uint64_t)i*UINT64_C(4));}
NOINLINE uint32_t*wrap_unsigned(struct AddressFields*p,uint64_t i){return (uint32_t*)((uintptr_t)&p->i32[0]+(uint64_t)i*UINT64_C(4));}
NOINLINE uint8_t*wrap_byte(struct AddressFields*p,uint64_t i){return (uint8_t*)((uintptr_t)&p->i32[0]+(uint64_t)i*UINT64_C(4));}
NOINLINE int32_t*logical_address(struct AddressFields*p,uint32_t n){uint64_t i=UINT64_MAX>>n;return (int32_t*)((uintptr_t)&p->i32[0]+i*UINT64_C(4));}
NOINLINE int32_t*signed_address(struct AddressFields*p,uint32_t n){uint64_t i=(uint64_t)(INT64_MIN>>n);return (int32_t*)((uintptr_t)&p->i32[0]+i*UINT64_C(4));}
NOINLINE int32_t*left_address(struct AddressFields*p,uint32_t n){uint64_t i=UINT64_C(1)<<n;return (int32_t*)((uintptr_t)&p->i32[0]+i*UINT64_C(4));}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
