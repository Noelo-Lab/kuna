#include <stdint.h>
#define N __attribute__((noinline))
N void wide_copy(__uint128_t *p,const __uint128_t *q){*p=*q;}
N void wide_zero(__uint128_t *p){*p=0;}
N void wide_unsigned(__uint128_t *p,uint64_t x){*p=x;}
N void wide_signed(__int128_t *p,int64_t x){*p=x;}
N void wide_halves(__uint128_t *p,uint64_t lo,uint64_t hi){*p=(__uint128_t)hi<<64|lo;}
N void wide_add(__uint128_t *p,const __uint128_t *q){*p=*q+1;}
int main(void){return 0;}
