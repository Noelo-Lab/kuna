#include <dlfcn.h>
#include <stdio.h>
#include "sample.so.h"
static void load_control(unsigned short v) { __asm__ volatile("fldcw %0" :: "m"(v) : "memory"); }
static unsigned short control(void) { unsigned short v; __asm__ volatile("fnstcw %0" : "=m"(v)); return v; }
int main(int argc, char **argv) {
    if (argc != 2) return 90;
    void *image = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL); if (!image) { puts(dlerror()); return 91; }
    short (*ref16)(long double *) = dlsym(image, "sample_cast16");
    int (*ref32)(long double *) = dlsym(image, "sample_cast32");
    long long (*ref64)(long double *, unsigned short) = dlsym(image, "sample_cast64");
    if (!ref16 || !ref32 || !ref64) return 92;
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
            if (control() != (unsigned short)(0x37f | (mode << 10))) return 93;
        }
    }
    load_control(saved);
    printf("%u native comparisons passed\n", comparisons);
    return 0;
}
