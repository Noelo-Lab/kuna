#include <stdint.h>
#define N __attribute__((noinline))
N int32_t*integer_eight(int32_t(*p)[9]){return &(*p)[8];}
N int32_t*integer_end(int32_t(*p)[9]){return &(*p)[9];}
N uint8_t**pointer_eight(uint8_t*(*p)[9]){return &(*p)[8];}
N uint8_t**pointer_end(uint8_t*(*p)[9]){return &(*p)[9];}
int main(void){return 0;}
