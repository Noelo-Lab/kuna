#include <dlfcn.h>
#include <stdio.h>
#include <string.h>
#include <cpuid.h>
#include "sample.so.h"
static void load_control(unsigned short v) { __asm__ volatile("fldcw %0" :: "m"(v) : "memory"); }
static unsigned short control(void) { unsigned short v; __asm__ volatile("fnstcw %0" : "=m"(v)); return v; }
int main(int argc, char **argv) {
    if (argc != 2) return 90;
    void *image = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL); if (!image) { puts(dlerror()); return 91; }
    short (*ref16)(long double *) = dlsym(image, "sample_cast16");
    int (*ref32)(long double *) = dlsym(image, "sample_cast32");
    long long (*ref64)(long double *, unsigned short) = dlsym(image, "sample_cast64");
    int (*restored)(long double *, void *) = dlsym(image, "sample_restored32");
    int (*xrestored)(long double *, void *) = dlsym(image, "sample_xrestored32");
    if (!ref16 || !ref32 || !ref64 || !restored || !xrestored) return 92;
    unsigned a, b, c, d;
    int has_xsave = __get_cpuid(1, &a, &b, &c, &d) && (c & bit_OSXSAVE);
    static unsigned char area[1024] __attribute__((aligned(64))), copy[1024] __attribute__((aligned(64)));
    unsigned restorations = 0;
    static long double values[] = {0.0L, -0.0L, 0.25L, -0.25L, 0.5L, -0.5L, 0.75L, -0.75L, 1.5L, -1.5L,
        2.5L, -2.5L, 3.5L, -3.5L, 32767.75L, -32768.75L, 2147483647.5L, -2147483648.5L,
        0x1.fffffffffffffffep62L, -0x1p63L, 0x1.000000000000001p0L, -0x1.000000000000001p0L, 1e-4000L};
    unsigned short saved = control(), extras[] = {0, 0x300, 0x1000};
    unsigned comparisons = 0;
    for (unsigned mode = 0; mode < 4; mode++) {
        load_control((unsigned short)(0x37f | (mode << 10)));
        for (unsigned i = 0; i < sizeof values / sizeof values[0]; i++) {
            long double v = values[i];
            if (v > -32769.0L && v < 32768.0L) {
                if (ref16(&v) != sample_cast16(&v)) { fprintf(stderr, "16 mode=%u i=%u\n", mode, i); return 1; }
                comparisons++;
            }
            if (v > -2147483649.0L && v < 2147483648.0L) {
                if (ref32(&v) != sample_cast32(&v)) { fprintf(stderr, "32 mode=%u i=%u\n", mode, i); return 1; }
                comparisons++;
            }
            for (unsigned e = 0; e < 3; e++) {
                if (ref64(&v, extras[e]) != (long long)sample_cast64(&v, extras[e])) { fprintf(stderr, "64 mode=%u i=%u\n", mode, i); return 1; }
                comparisons++;
            }
            for (unsigned restore = 0; restore < 8; restore++) {
                unsigned short word = (unsigned short)(0x37f | ((restore & 3) << 10));
                unsigned mxcsr;
                __asm__ volatile("stmxcsr %0" : "=m"(mxcsr));
                memset(area, 0, sizeof area);
                memcpy(area, &word, 2);
                memcpy(area + 24, &mxcsr, 4);
                area[512] = restore < 4;
                if (v > -2147483649.0L && v < 2147483648.0L) {
                    memcpy(copy, area, sizeof area);
                    if (restore < 4 && restored(&v, area) != sample_restored32(&v, copy)) { fprintf(stderr, "fxrstor mode=%u restore=%u i=%u\n", mode, restore, i); return 1; }
                    if (has_xsave && xrestored(&v, area) != sample_xrestored32(&v, copy)) { fprintf(stderr, "xrstor mode=%u restore=%u i=%u\n", mode, restore, i); return 1; }
                    load_control((unsigned short)(0x37f | (mode << 10)));
                    restorations++;
                }
            }
            if (control() != (unsigned short)(0x37f | (mode << 10))) return 93;
        }
    }
    load_control(saved);
    printf("%u native comparisons passed\n%u restored control words compared\n", comparisons, restorations);
    return 0;
}
