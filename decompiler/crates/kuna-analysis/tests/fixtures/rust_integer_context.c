#include <stdint.h>
extern unsigned value_calls;
uint64_t counted_unsigned(uint64_t value);
uint64_t insert_bit(uint64_t a,uint64_t n){return a | (UINT64_C(1) << (n & 63));}
uint64_t isolated_bit(uint64_t n){return UINT64_C(1) << (n & 63);}
uint64_t invert_low(uint64_t n){return ~(UINT64_MAX << (n & 63));}
uint64_t shift_again(uint64_t n,uint64_t m){return (UINT64_C(1) << (n & 63)) >> (m & 63);}
uint64_t literal_right(uint64_t n){return UINT64_C(0x8000000000000000) >> (n & 63);}
int64_t signed_right(uint64_t n){return INT64_MIN >> (n & 63);}
uint64_t named_shift(uint64_t a,uint64_t n){return a << (n & 63);}
int64_t signed_shift(int64_t a,uint64_t n){return (uint64_t)a << (n & 63);}
uint64_t mixed_shift(uint64_t a,uint64_t n){return a + (UINT64_C(1) << (n & 63));}
uint64_t accumulate_bits(uint64_t n){uint64_t acc=0;for(unsigned i=0;i<(n&63);i++)acc|=UINT64_C(1)<<i;return acc;}
int64_t accumulate_calls(int64_t acc,uint64_t step,uint32_t count){while(count--){acc+=counted_unsigned(step);}return acc;}
uint64_t mixed_less(uint64_t a,int64_t b,uint64_t c){return a+b<c;}
uint64_t mixed_left(uint64_t a,int64_t b,uint64_t n){return (a+b)<<(n&63);}
uint64_t mixed_sub_less(uint64_t a,int64_t b,uint64_t c){return a-b<c;}
uint64_t mixed_sub_left(uint64_t a,int64_t b,uint64_t n){return (a-b)<<(n&63);}
#ifndef KUNA_NATIVE_MAIN
int main(void){return 0;}
#else
#include <stdio.h>
#include <inttypes.h>
int main(void){uint64_t values[]={0,1,UINT64_C(0x8000000000000000),UINT64_MAX},counts[]={0,1,31,32,63};for(unsigned i=0;i<4;i++)for(uint64_t n=0;n<64;n++)for(unsigned j=0;j<5;j++){uint64_t a=values[i],m=counts[j];value_calls=0;uint64_t accumulated=(uint64_t)accumulate_calls((int64_t)a,a^m,(uint32_t)(n&7));printf("%016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %u\n",insert_bit(a,n),isolated_bit(n),invert_low(n),shift_again(n,m),literal_right(n),(uint64_t)signed_right(n),named_shift(a,n),(uint64_t)signed_shift((int64_t)a,n),mixed_shift(a,n),accumulate_bits(n),accumulated,mixed_less(a,(int64_t)m,n),mixed_left(a,(int64_t)m,n),mixed_sub_less(a,(int64_t)m,n),mixed_sub_left(a,(int64_t)m,n),value_calls);}return 0;}
#endif
