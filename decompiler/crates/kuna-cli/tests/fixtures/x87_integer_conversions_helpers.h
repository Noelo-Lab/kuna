#include <stdint.h>
#define FLOAT_ROUND(value) __builtin_rintl(value)
#define CONVERSION(name,type,instruction) \
static inline type name(long double value, unsigned short control) { \
    unsigned short saved; type result; \
    __asm__ volatile("fnstcw %0" : "=m"(saved) :: "memory"); \
    __asm__ volatile("fldcw %0" :: "m"(control) : "memory"); \
    __asm__ volatile("fldt %1; " instruction " %0" : "=m"(result) : "m"(value) : "st", "memory"); \
    __asm__ volatile("fldcw %0" :: "m"(saved) : "memory"); \
    return result; \
}
CONVERSION(x87_float_to_int16,int16_t,"fistps")
CONVERSION(x87_float_to_int32,int32_t,"fistpl")
CONVERSION(x87_float_to_int64,int64_t,"fistpll")
#undef CONVERSION
