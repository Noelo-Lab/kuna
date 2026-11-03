#include <stdio.h>
#include <inttypes.h>
#include <string.h>
#define OMIT_MAIN
#include "pointer_index.c"
extern unsigned long trace;
static void bytes(const void*p,size_t n){const unsigned char*b=p;for(size_t k=0;k<n;k++)printf("%02x",b[k]);putchar('\n');}
int main(void){uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(0x7f),UINT64_C(0x80),UINT64_C(0xff),UINT64_C(0x7fff),UINT64_C(0x8000),UINT64_C(0xffff),UINT64_C(0x7fffffff),UINT64_C(0x80000000),UINT64_C(0xffffffff),UINT64_C(0x7fffffffffffffff),UINT64_C(0x8000000000000000),UINT64_C(0xffffffffffffffff),UINT64_C(0x0123456789abcdef),UINT64_C(0xfedcba9876543210),UINT64_C(0xaaaaaaaa55555555)};for(unsigned k=0;k<18;k++){uint64_t v=values[k];for(int64_t i=-7;i<=7;i++){
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;printf("load_i8_s %016" PRIx64 "\n",(uint64_t)load_i8_s(p,(int64_t)i));}
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;store_i8_s(p,(int64_t)i,(int8_t)v);printf("store_i8_s ");bytes(a,sizeof(a));}
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;printf("address_i8_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i8_s(p,(int64_t)v)-(uintptr_t)p));}
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;printf("load_i8_u %016" PRIx64 "\n",(uint64_t)load_i8_u(p,(uint64_t)(i<0?-i:i)));}
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;store_i8_u(p,(uint64_t)(i<0?-i:i),(int8_t)v);printf("store_i8_u ");bytes(a,sizeof(a));}
{int8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int8_t*p=a+8;printf("address_i8_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i8_u(p,(uint64_t)v)-(uintptr_t)p));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;printf("load_u8_s %016" PRIx64 "\n",(uint64_t)load_u8_s(p,(int64_t)i));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;store_u8_s(p,(int64_t)i,(uint8_t)v);printf("store_u8_s ");bytes(a,sizeof(a));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;printf("address_u8_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u8_s(p,(int64_t)v)-(uintptr_t)p));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;printf("load_u8_u %016" PRIx64 "\n",(uint64_t)load_u8_u(p,(uint64_t)(i<0?-i:i)));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;store_u8_u(p,(uint64_t)(i<0?-i:i),(uint8_t)v);printf("store_u8_u ");bytes(a,sizeof(a));}
{uint8_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint8_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint8_t*p=a+8;printf("address_u8_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u8_u(p,(uint64_t)v)-(uintptr_t)p));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;printf("load_i16_s %016" PRIx64 "\n",(uint64_t)load_i16_s(p,(int64_t)i));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;store_i16_s(p,(int64_t)i,(int16_t)v);printf("store_i16_s ");bytes(a,sizeof(a));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;printf("address_i16_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i16_s(p,(int64_t)v)-(uintptr_t)p));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;printf("load_i16_u %016" PRIx64 "\n",(uint64_t)load_i16_u(p,(uint64_t)(i<0?-i:i)));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;store_i16_u(p,(uint64_t)(i<0?-i:i),(int16_t)v);printf("store_i16_u ");bytes(a,sizeof(a));}
{int16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int16_t*p=a+8;printf("address_i16_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i16_u(p,(uint64_t)v)-(uintptr_t)p));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;printf("load_u16_s %016" PRIx64 "\n",(uint64_t)load_u16_s(p,(int64_t)i));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;store_u16_s(p,(int64_t)i,(uint16_t)v);printf("store_u16_s ");bytes(a,sizeof(a));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;printf("address_u16_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u16_s(p,(int64_t)v)-(uintptr_t)p));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;printf("load_u16_u %016" PRIx64 "\n",(uint64_t)load_u16_u(p,(uint64_t)(i<0?-i:i)));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;store_u16_u(p,(uint64_t)(i<0?-i:i),(uint16_t)v);printf("store_u16_u ");bytes(a,sizeof(a));}
{uint16_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint16_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint16_t*p=a+8;printf("address_u16_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u16_u(p,(uint64_t)v)-(uintptr_t)p));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;printf("load_i32_s %016" PRIx64 "\n",(uint64_t)load_i32_s(p,(int64_t)i));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;store_i32_s(p,(int64_t)i,(int32_t)v);printf("store_i32_s ");bytes(a,sizeof(a));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;printf("address_i32_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i32_s(p,(int64_t)v)-(uintptr_t)p));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;printf("load_i32_u %016" PRIx64 "\n",(uint64_t)load_i32_u(p,(uint64_t)(i<0?-i:i)));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;store_i32_u(p,(uint64_t)(i<0?-i:i),(int32_t)v);printf("store_i32_u ");bytes(a,sizeof(a));}
{int32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int32_t*p=a+8;printf("address_i32_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i32_u(p,(uint64_t)v)-(uintptr_t)p));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;printf("load_u32_s %016" PRIx64 "\n",(uint64_t)load_u32_s(p,(int64_t)i));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;store_u32_s(p,(int64_t)i,(uint32_t)v);printf("store_u32_s ");bytes(a,sizeof(a));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;printf("address_u32_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u32_s(p,(int64_t)v)-(uintptr_t)p));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;printf("load_u32_u %016" PRIx64 "\n",(uint64_t)load_u32_u(p,(uint64_t)(i<0?-i:i)));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;store_u32_u(p,(uint64_t)(i<0?-i:i),(uint32_t)v);printf("store_u32_u ");bytes(a,sizeof(a));}
{uint32_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint32_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint32_t*p=a+8;printf("address_u32_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u32_u(p,(uint64_t)v)-(uintptr_t)p));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;printf("load_i64_s %016" PRIx64 "\n",(uint64_t)load_i64_s(p,(int64_t)i));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;store_i64_s(p,(int64_t)i,(int64_t)v);printf("store_i64_s ");bytes(a,sizeof(a));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;printf("address_i64_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i64_s(p,(int64_t)v)-(uintptr_t)p));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;printf("load_i64_u %016" PRIx64 "\n",(uint64_t)load_i64_u(p,(uint64_t)(i<0?-i:i)));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;store_i64_u(p,(uint64_t)(i<0?-i:i),(int64_t)v);printf("store_i64_u ");bytes(a,sizeof(a));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;printf("address_i64_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_i64_u(p,(uint64_t)v)-(uintptr_t)p));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;printf("load_u64_s %016" PRIx64 "\n",(uint64_t)load_u64_s(p,(int64_t)i));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;store_u64_s(p,(int64_t)i,(uint64_t)v);printf("store_u64_s ");bytes(a,sizeof(a));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;printf("address_u64_s %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u64_s(p,(int64_t)v)-(uintptr_t)p));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;printf("load_u64_u %016" PRIx64 "\n",(uint64_t)load_u64_u(p,(uint64_t)(i<0?-i:i)));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;store_u64_u(p,(uint64_t)(i<0?-i:i),(uint64_t)v);printf("store_u64_u ");bytes(a,sizeof(a));}
{uint64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(uint64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);uint64_t*p=a+8;printf("address_u64_u %016" PRIx64 "\n",(uint64_t)((uintptr_t)address_u64_u(p,(uint64_t)v)-(uintptr_t)p));}
{struct Record a[17];for(unsigned k=0;k<17;k++)a[k]=(struct Record){(int64_t)(v+k),(int32_t)(v+k),0xdeadbeef+k};struct Record*p=a+8;printf("field_load %016" PRIx64 "\n",(uint64_t)field_load(p,i));}
{struct Record a[17];for(unsigned k=0;k<17;k++)a[k]=(struct Record){(int64_t)(v+k),(int32_t)(v+k),0xdeadbeef+k};struct Record*p=a+8;field_store(p,i,(int32_t)v);printf("field_store ");bytes(a,sizeof(a));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(v+k);int64_t*p=a+8;trace=0;uint64_t result=(uint64_t)callback_load(p,i);printf("callback_load %016" PRIx64 " %lu\n",result,trace);}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(v+k);int64_t*p=a+8;trace=0;callback_store(p,i,(int64_t)v);printf("callback_store %lu ",trace);bytes(a,sizeof(a));}
{int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(v+k);int64_t*p=a+8;trace=0;uint64_t result=(uintptr_t)callback_address(p,(int64_t)v)-(uintptr_t)p;printf("callback_address %016" PRIx64 " %lu\n",result,trace);}
{int32_t x=(int32_t)v;printf("zero_load %016" PRIx64 "\n",(uint64_t)zero_load(&x));}
}}int32_t anchor=0;for(unsigned n=0;n<64;n++){printf("shift_address %016" PRIx64 "\n",(uint64_t)((uintptr_t)shift_address(&anchor,n)-(uintptr_t)&anchor));printf("logical_address %016" PRIx64 "\n",(uint64_t)((uintptr_t)logical_address(&anchor,n)-(uintptr_t)&anchor));printf("signed_address %016" PRIx64 "\n",(uint64_t)((uintptr_t)signed_address(&anchor,n)-(uintptr_t)&anchor));}return 0;}
