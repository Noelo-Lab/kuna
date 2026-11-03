#define _GNU_SOURCE
#include <stdio.h>
#include <inttypes.h>
#include <string.h>
#include <sys/mman.h>
#define OMIT_MAIN
#include "array_address.c"
extern unsigned long trace;
int main(void){uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(7),UINT64_C(8),UINT64_C(9),UINT64_C(0x7f),UINT64_C(0x80),UINT64_C(0x7fffffff),UINT64_C(0x80000000),UINT64_C(0xffffffff),UINT64_C(0x7fffffffffffffff),UINT64_C(0x8000000000000000),UINT64_C(0xffffffffffffffff),UINT64_C(0x0123456789abcdef),UINT64_C(0xfedcba9876543210),UINT64_C(0xaaaaaaaa55555555),UINT64_C(0x4000000000000000)};for(int mode=0;mode<2;mode++){void*memory=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);if(memory==MAP_FAILED)return 2;memset(memory,0xa5,4096);struct AddressFields*p=memory;int32_t(*np)[9]=(int32_t(*)[9])((char*)memory+1024)+2;if(mode&&mprotect(memory,4096,PROT_NONE))return 3;printf("mode %d\n",mode);
for(int64_t i=0;i<=9;i++)printf("address_i8_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i8_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i8_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i8_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u8_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u8_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u8_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u8_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i16_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i16_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i16_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i16_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u16_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u16_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u16_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u16_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i32_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i32_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i32_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i32_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u32_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u32_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u32_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u32_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i64_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i64_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_i64_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_i64_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u64_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u64_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_u64_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_u64_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_float32_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_float32_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_float32_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_float32_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_float64_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_float64_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_float64_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_float64_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_pointers_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_pointers_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_pointers_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_pointers_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_flags_s %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_flags_s(p,(int64_t)i)-(uintptr_t)p));
for(int64_t i=0;i<=9;i++)printf("address_flags_u %ld %016" PRIx64 "\n",(long)i,(uint64_t)((uintptr_t)address_flags_u(p,(uint64_t)i)-(uintptr_t)p));
for(int64_t i=-1;i<=1;i++)for(int64_t j=0;j<=9;j++)printf("nested_address_s %ld %ld %016" PRIx64 "\n",(long)i,(long)j,(uint64_t)((uintptr_t)nested_address_s(np,i,(int64_t)j)-(uintptr_t)np));
for(int64_t i=-1;i<=1;i++)for(int64_t j=0;j<=9;j++)printf("nested_address_u %ld %ld %016" PRIx64 "\n",(long)i,(long)j,(uint64_t)((uintptr_t)nested_address_u(np,i,(uint64_t)j)-(uintptr_t)np));
for(int64_t i=0;i<=9;i++){trace=0;uintptr_t result=(uintptr_t)callback_address(p,i);printf("callback_address %ld %016" PRIx64 " %lu\n",(long)i,(uint64_t)(result-(uintptr_t)p),trace);}
for(unsigned k=0;k<18;k++)printf("wrap_signed %016" PRIx64 " %016" PRIx64 "\n",values[k],(uint64_t)((uintptr_t)wrap_signed(p,(int64_t)values[k])-(uintptr_t)p));
for(unsigned k=0;k<18;k++)printf("wrap_unsigned %016" PRIx64 " %016" PRIx64 "\n",values[k],(uint64_t)((uintptr_t)wrap_unsigned(p,(uint64_t)values[k])-(uintptr_t)p));
for(unsigned k=0;k<18;k++)printf("wrap_byte %016" PRIx64 " %016" PRIx64 "\n",values[k],(uint64_t)((uintptr_t)wrap_byte(p,(uint64_t)values[k])-(uintptr_t)p));
for(unsigned n=0;n<64;n++)printf("logical_address %u %016" PRIx64 "\n",n,(uint64_t)((uintptr_t)logical_address(p,n)-(uintptr_t)p));
for(unsigned n=0;n<64;n++)printf("signed_address %u %016" PRIx64 "\n",n,(uint64_t)((uintptr_t)signed_address(p,n)-(uintptr_t)p));
for(unsigned n=0;n<64;n++)printf("left_address %u %016" PRIx64 "\n",n,(uint64_t)((uintptr_t)left_address(p,n)-(uintptr_t)p));
if(munmap(memory,4096))return 4;}return 0;}
