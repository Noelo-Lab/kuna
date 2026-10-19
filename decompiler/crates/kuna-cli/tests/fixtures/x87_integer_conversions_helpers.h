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
#define X87_CLOBBERS "memory", "st", "st(1)", "st(2)", "st(3)", "st(4)", "st(5)", "st(6)", "st(7)"
#define _fxrstor(area) __asm__ volatile("fxrstor %0" :: "m"(*(const char (*)[512])(area)) : X87_CLOBBERS, \
    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10", "xmm11", "xmm12", "xmm13", "xmm14", "xmm15")
#define xrstor(area, size) __asm__ volatile("xrstor %0" :: "m"(*(const char (*)[576])(area)), "a"(1), "d"(0) : X87_CLOBBERS)
