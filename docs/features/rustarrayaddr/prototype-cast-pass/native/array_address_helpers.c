#include <stdint.h>
struct AddressFields;unsigned long trace=0;
__attribute__((noinline)) int64_t counted_index(int64_t i){trace=trace*10+1;return i;}
__attribute__((noinline)) struct AddressFields*counted_base(struct AddressFields*p){trace=trace*10+2;return p;}
