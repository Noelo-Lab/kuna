#include <stdint.h>
__attribute__((noinline)) int*shift_address(int*p,unsigned n){return(int*)((uintptr_t)p+(UINT64_C(1)<<(n&63))*4);}
__attribute__((noinline)) int*logical_address(int*p,unsigned n){return(int*)((uintptr_t)p+(UINT64_MAX>>(n&63))*4);}
__attribute__((noinline)) int*signed_address(int*p,unsigned n){return(int*)((uintptr_t)p+(uint64_t)(INT64_MIN>>(n&63))*4);}
int main(void){return 0;}
