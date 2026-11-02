#include <stdint.h>
typedef struct Pair { int64_t first, second; } Pair;
typedef struct UPair { uint64_t first, second; } UPair;
uint64_t calls;
__attribute__((noinline,noclone)) Pair make_pair(int64_t a,int64_t b) { ++calls; return (Pair){a+b,a-b}; }
__attribute__((noinline,noclone)) UPair make_unsigned(int64_t a,int64_t b) { ++calls; return (UPair){(uint64_t)a+(uint64_t)b,(uint64_t)a-(uint64_t)b}; }
__attribute__((noinline,noclone)) int8_t signed_int8(int64_t a,int64_t b) { return (int8_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int64_t signed_int8_wide(int64_t a,int64_t b) { return (int8_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint8_t signed_uint8(int64_t a,int64_t b) { return (uint8_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint64_t signed_uint8_wide(int64_t a,int64_t b) { return (uint8_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int16_t signed_int16(int64_t a,int64_t b) { return (int16_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int64_t signed_int16_wide(int64_t a,int64_t b) { return (int16_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint16_t signed_uint16(int64_t a,int64_t b) { return (uint16_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint64_t signed_uint16_wide(int64_t a,int64_t b) { return (uint16_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int32_t signed_int32(int64_t a,int64_t b) { return (int32_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int64_t signed_int32_wide(int64_t a,int64_t b) { return (int32_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint32_t signed_uint32(int64_t a,int64_t b) { return (uint32_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint64_t signed_uint32_wide(int64_t a,int64_t b) { return (uint32_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int8_t unsigned_int8(int64_t a,int64_t b) { return (int8_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) uint8_t unsigned_uint8(int64_t a,int64_t b) { return (uint8_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) int16_t unsigned_int16(int64_t a,int64_t b) { return (int16_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) uint16_t unsigned_uint16(int64_t a,int64_t b) { return (uint16_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) int32_t unsigned_int32(int64_t a,int64_t b) { return (int32_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) uint32_t unsigned_uint32(int64_t a,int64_t b) { return (uint32_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) int32_t first_narrow(int64_t a,int64_t b) { return (int32_t)make_pair(a,b).first; }
__attribute__((noinline,noclone)) uint8_t shift8(int64_t a,int64_t b) { return (uint8_t)(make_unsigned(a,b).second>>8); }
__attribute__((noinline,noclone)) uint32_t shift32(int64_t a,int64_t b) { return (uint32_t)(make_unsigned(a,b).second>>32); }
__attribute__((noinline,noclone)) int64_t signed_high(int64_t a,int64_t b) { return (int32_t)(make_pair(a,b).second>>32); }
__attribute__((noinline,noclone)) uint64_t signed_as_unsigned(int64_t a,int64_t b) { return (uint64_t)make_pair(a,b).second; }
__attribute__((noinline,noclone)) int64_t unsigned_as_signed(int64_t a,int64_t b) { return (int64_t)make_unsigned(a,b).second; }
__attribute__((noinline,noclone)) int64_t exact_signed(int64_t a,int64_t b) { return make_pair(a,b).second; }
__attribute__((noinline,noclone)) uint64_t exact_unsigned(int64_t a,int64_t b) { return make_unsigned(a,b).second; }
int main(void) { return signed_int32_wide(4294967297LL,0)!=1; }
