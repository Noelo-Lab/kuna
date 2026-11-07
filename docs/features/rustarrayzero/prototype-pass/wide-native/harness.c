#define main fixture_main
#include "wide.c"
#undef main
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void){uint64_t values[]={UINT64_C(0),UINT64_C(1),UINT64_C(2),UINT64_C(9223372036854775807),UINT64_C(9223372036854775808),UINT64_C(18446744073709551615),UINT64_C(81985529216486895),UINT64_C(18364758544493064720)};size_t positions[]={16,32,96};for(size_t a=0;a<8;a++){for(size_t b=0;b<8;b++){uint64_t lo=values[a],hi=values[b];for(size_t location=0;location<3;location++){
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_copy((__uint128_t*)p,(__uint128_t*)(memory+96));printf("wide_copy %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_zero((__uint128_t*)p);printf("wide_zero %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_unsigned((__uint128_t*)p,lo);printf("wide_unsigned %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_signed((__int128_t*)p,(int64_t)lo);printf("wide_signed %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_halves((__uint128_t*)p,lo,hi);printf("wide_halves %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
{unsigned char*memory=malloc(128);if(memory==NULL)return 1;memset(memory,0xa5,128);__uint128_t source=((__uint128_t)hi<<64)|lo;memcpy(memory+96,&source,16);unsigned char*p=memory+positions[location];wide_add((__uint128_t*)p,(__uint128_t*)(memory+96));printf("wide_add %zu %zu %zu ",a,b,location);for(size_t i=0;i<128;i++)printf("%02x",memory[i]);puts("");free(memory);}
}}}return 0;}
