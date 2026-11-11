#include <stdint.h>
uint64_t insert_bit(uint64_t a,uint64_t n){return a | (UINT64_C(1) << (n & 63));}
uint64_t isolated_bit(uint64_t n){return UINT64_C(1) << (n & 63);}
#include <stdio.h>
int main(void){uint64_t values[]={0,1,0x8000000000000000ULL,0xffffffffffffffffULL};for(unsigned i=0;i<4;i++)for(uint64_t n=0;n<64;n++)printf("%016lx %016lx\n",insert_bit(values[i],n),isolated_bit(n));return 0;}
