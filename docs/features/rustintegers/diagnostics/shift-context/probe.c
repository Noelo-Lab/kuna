#include <stdint.h>
uint64_t insert_bit(uint64_t a,uint64_t n){return a | (UINT64_C(1) << (n & 63));}
uint64_t isolated_bit(uint64_t n){return UINT64_C(1) << (n & 63);}
int main(void){return 0;}
