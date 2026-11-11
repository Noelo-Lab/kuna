#include <stdint.h>
#include <stddef.h>
#define NOINLINE __attribute__((noinline))
NOINLINE int8_t load_i8_s(int8_t*p,int64_t i){return p[i];}
NOINLINE void store_i8_s(int8_t*p,int64_t i,int8_t value){p[i]=value;}
NOINLINE int8_t* address_i8_s(int8_t*p,int64_t i){return (int8_t*)((uintptr_t)p+(uint64_t)i*1);}
NOINLINE int8_t load_i8_u(int8_t*p,uint64_t i){return p[i];}
NOINLINE void store_i8_u(int8_t*p,uint64_t i,int8_t value){p[i]=value;}
NOINLINE int8_t* address_i8_u(int8_t*p,uint64_t i){return (int8_t*)((uintptr_t)p+(uint64_t)i*1);}
NOINLINE uint8_t load_u8_s(uint8_t*p,int64_t i){return p[i];}
NOINLINE void store_u8_s(uint8_t*p,int64_t i,uint8_t value){p[i]=value;}
NOINLINE uint8_t* address_u8_s(uint8_t*p,int64_t i){return (uint8_t*)((uintptr_t)p+(uint64_t)i*1);}
NOINLINE uint8_t load_u8_u(uint8_t*p,uint64_t i){return p[i];}
NOINLINE void store_u8_u(uint8_t*p,uint64_t i,uint8_t value){p[i]=value;}
NOINLINE uint8_t* address_u8_u(uint8_t*p,uint64_t i){return (uint8_t*)((uintptr_t)p+(uint64_t)i*1);}
NOINLINE int16_t load_i16_s(int16_t*p,int64_t i){return p[i];}
NOINLINE void store_i16_s(int16_t*p,int64_t i,int16_t value){p[i]=value;}
NOINLINE int16_t* address_i16_s(int16_t*p,int64_t i){return (int16_t*)((uintptr_t)p+(uint64_t)i*2);}
NOINLINE int16_t load_i16_u(int16_t*p,uint64_t i){return p[i];}
NOINLINE void store_i16_u(int16_t*p,uint64_t i,int16_t value){p[i]=value;}
NOINLINE int16_t* address_i16_u(int16_t*p,uint64_t i){return (int16_t*)((uintptr_t)p+(uint64_t)i*2);}
NOINLINE uint16_t load_u16_s(uint16_t*p,int64_t i){return p[i];}
NOINLINE void store_u16_s(uint16_t*p,int64_t i,uint16_t value){p[i]=value;}
NOINLINE uint16_t* address_u16_s(uint16_t*p,int64_t i){return (uint16_t*)((uintptr_t)p+(uint64_t)i*2);}
NOINLINE uint16_t load_u16_u(uint16_t*p,uint64_t i){return p[i];}
NOINLINE void store_u16_u(uint16_t*p,uint64_t i,uint16_t value){p[i]=value;}
NOINLINE uint16_t* address_u16_u(uint16_t*p,uint64_t i){return (uint16_t*)((uintptr_t)p+(uint64_t)i*2);}
NOINLINE int32_t load_i32_s(int32_t*p,int64_t i){return p[i];}
NOINLINE void store_i32_s(int32_t*p,int64_t i,int32_t value){p[i]=value;}
NOINLINE int32_t* address_i32_s(int32_t*p,int64_t i){return (int32_t*)((uintptr_t)p+(uint64_t)i*4);}
NOINLINE int32_t load_i32_u(int32_t*p,uint64_t i){return p[i];}
NOINLINE void store_i32_u(int32_t*p,uint64_t i,int32_t value){p[i]=value;}
NOINLINE int32_t* address_i32_u(int32_t*p,uint64_t i){return (int32_t*)((uintptr_t)p+(uint64_t)i*4);}
NOINLINE uint32_t load_u32_s(uint32_t*p,int64_t i){return p[i];}
NOINLINE void store_u32_s(uint32_t*p,int64_t i,uint32_t value){p[i]=value;}
NOINLINE uint32_t* address_u32_s(uint32_t*p,int64_t i){return (uint32_t*)((uintptr_t)p+(uint64_t)i*4);}
NOINLINE uint32_t load_u32_u(uint32_t*p,uint64_t i){return p[i];}
NOINLINE void store_u32_u(uint32_t*p,uint64_t i,uint32_t value){p[i]=value;}
NOINLINE uint32_t* address_u32_u(uint32_t*p,uint64_t i){return (uint32_t*)((uintptr_t)p+(uint64_t)i*4);}
NOINLINE int64_t load_i64_s(int64_t*p,int64_t i){return p[i];}
NOINLINE void store_i64_s(int64_t*p,int64_t i,int64_t value){p[i]=value;}
NOINLINE int64_t* address_i64_s(int64_t*p,int64_t i){return (int64_t*)((uintptr_t)p+(uint64_t)i*8);}
NOINLINE int64_t load_i64_u(int64_t*p,uint64_t i){return p[i];}
NOINLINE void store_i64_u(int64_t*p,uint64_t i,int64_t value){p[i]=value;}
NOINLINE int64_t* address_i64_u(int64_t*p,uint64_t i){return (int64_t*)((uintptr_t)p+(uint64_t)i*8);}
NOINLINE uint64_t load_u64_s(uint64_t*p,int64_t i){return p[i];}
NOINLINE void store_u64_s(uint64_t*p,int64_t i,uint64_t value){p[i]=value;}
NOINLINE uint64_t* address_u64_s(uint64_t*p,int64_t i){return (uint64_t*)((uintptr_t)p+(uint64_t)i*8);}
NOINLINE uint64_t load_u64_u(uint64_t*p,uint64_t i){return p[i];}
NOINLINE void store_u64_u(uint64_t*p,uint64_t i,uint64_t value){p[i]=value;}
NOINLINE uint64_t* address_u64_u(uint64_t*p,uint64_t i){return (uint64_t*)((uintptr_t)p+(uint64_t)i*8);}
struct Record{int64_t before;int32_t value;uint32_t tag;};
int64_t counted_index(int64_t);int64_t*counted_base(int64_t*);int64_t counted_value(int64_t);
NOINLINE int32_t field_load(struct Record*p,int64_t i){return p[i].value;}
NOINLINE void field_store(struct Record*p,int64_t i,int32_t value){p[i].value=value;}
NOINLINE int64_t callback_load(int64_t*p,int64_t i){i=counted_index(i);p=counted_base(p);return p[i];}
NOINLINE void callback_store(int64_t*p,int64_t i,int64_t value){value=counted_value(value);i=counted_index(i);p=counted_base(p);p[i]=value;}
NOINLINE int64_t* callback_address(int64_t*p,int64_t i){i=counted_index(i);p=counted_base(p);return (int64_t*)((uintptr_t)p+(uint64_t)i*8);}
NOINLINE int32_t zero_load(int32_t*p){return *p;}
__attribute__((noinline)) int*shift_address(int*p,unsigned n){return(int*)((uintptr_t)p+(UINT64_C(1)<<(n&63))*4);}
__attribute__((noinline)) int*logical_address(int*p,unsigned n){return(int*)((uintptr_t)p+(UINT64_MAX>>(n&63))*4);}
__attribute__((noinline)) int*signed_address(int*p,unsigned n){return(int*)((uintptr_t)p+(uint64_t)(INT64_MIN>>(n&63))*4);}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
