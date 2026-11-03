#include <stdint.h>
#include <stddef.h>
#define N __attribute__((noinline))
struct WideFields { uint64_t before; __uint128_t value; uint64_t after; };
struct Byte16 { unsigned char bytes[16]; };
extern __uint128_t *counted_pointer(__uint128_t *p);
N void zero_unsigned(__uint128_t *p) { *p=0; }
N void zero_signed(__int128_t *p) { *p=0; }
N void zero_field(struct WideFields *p) { p->value=0; }
N void zero_second(__uint128_t *p) { p[1]=0; }
N void zero_index(__uint128_t *p,size_t i) { p[i]=0; }
N void zero_callback(__uint128_t *p) { *counted_pointer(p)=0; }
N void zero_bytes(struct Byte16 *p) { for(size_t i=0;i<16;i++)p->bytes[i]=0; }
N void one_unsigned(__uint128_t *p) { *p=1; }
N void signed_input(__int128_t *p,int64_t x) { *p=x; }
N void unsigned_input(__uint128_t *p,uint64_t x) { *p=x; }
N void two_halves(__uint128_t *p,uint64_t lo,uint64_t hi) { *p=(__uint128_t)hi<<64|lo; }
N void copy_wide(__uint128_t *p,__uint128_t *q) { *p=*q; }
N void zero_narrow(uint64_t *p) { *p=0; }
int main(void) { return 0; }
