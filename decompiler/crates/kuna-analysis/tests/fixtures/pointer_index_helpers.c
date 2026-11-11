#include <stdint.h>
unsigned long trace=0;
__attribute__((noinline)) int64_t counted_index(int64_t i){trace=trace*10+1;return i;}
__attribute__((noinline)) int64_t*counted_base(int64_t*p){trace=trace*10+2;return p;}
__attribute__((noinline)) int64_t counted_value(int64_t value){trace=trace*10+3;return value;}
