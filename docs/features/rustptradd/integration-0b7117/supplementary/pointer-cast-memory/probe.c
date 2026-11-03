#include <stdint.h>
__attribute__((noinline)) uint8_t byte_load(int64_t*p,int64_t i){return((uint8_t*)p)[i*8];}
__attribute__((noinline)) uint64_t unsigned_load(int64_t*p,int64_t i){return((uint64_t*)p)[i];}
__attribute__((noinline)) void byte_store(int64_t*p,int64_t i,uint8_t v){((uint8_t*)p)[i*8]=v;}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
