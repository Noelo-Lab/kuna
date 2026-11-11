#include <stdint.h>
__attribute__((noinline)) uint64_t* unsigned_address(int64_t*p,uint64_t i){return(uint64_t*)((uintptr_t)p+i*8);}
__attribute__((noinline)) uint8_t* byte_address(int64_t*p,uint64_t i){return(uint8_t*)((uintptr_t)p+i*8);}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
