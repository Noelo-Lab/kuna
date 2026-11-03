#include <stdio.h>
#include <inttypes.h>
#define OMIT_MAIN
#include "probe.c"
int main(void){int64_t a[4]={0};uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(7),UINT64_C(0x7fffffffffffffff),UINT64_C(0x8000000000000000),UINT64_C(0xffffffffffffffff),UINT64_C(0x2000000000000000),UINT64_C(0x4000000000000000),UINT64_C(0xaaaaaaaa55555555)};for(unsigned i=0;i<10;i++){printf("%016" PRIx64 " %016" PRIx64 "\n",(uint64_t)((uintptr_t)unsigned_address(a,values[i])-(uintptr_t)a),(uint64_t)((uintptr_t)byte_address(a,values[i])-(uintptr_t)a));}return 0;}
