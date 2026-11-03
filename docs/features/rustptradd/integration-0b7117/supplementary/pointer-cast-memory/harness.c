#include <stdio.h>
#include <inttypes.h>
#define OMIT_MAIN
#include "probe.c"
int main(void){uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(0x7f),UINT64_C(0x80),UINT64_C(0xff),UINT64_C(0x7fff),UINT64_C(0x8000),UINT64_C(0xffff),UINT64_C(0x7fffffff),UINT64_C(0x80000000),UINT64_C(0xffffffff),UINT64_C(0x7fffffffffffffff),UINT64_C(0x8000000000000000),UINT64_C(0xffffffffffffffff),UINT64_C(0x0123456789abcdef),UINT64_C(0xfedcba9876543210),UINT64_C(0xaaaaaaaa55555555)};for(unsigned n=0;n<18;n++){uint64_t v=values[n];for(int64_t i=-7;i<=7;i++){int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(UINT64_C(0x9e3779b97f4a7c15)*k+v);int64_t*p=a+8;printf("%02x %016" PRIx64 " ",byte_load(p,i),unsigned_load(p,i));byte_store(p,i,(uint8_t)v);unsigned char*b=(unsigned char*)a;for(unsigned k=0;k<sizeof(a);k++)printf("%02x",b[k]);putchar('\n');}}return 0;}
