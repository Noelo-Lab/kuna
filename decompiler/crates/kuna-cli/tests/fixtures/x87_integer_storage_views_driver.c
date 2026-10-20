#include <stdint.h>
#include <string.h>
#include <dlfcn.h>
#include <stdio.h>
unsigned char storage[32];
float sample32(float *, unsigned short);
double sample64(double *, unsigned short);
void sample_both(float *, float *, double *, unsigned short);
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    void *module = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!module) return 3;
    float (*original32)(float *) = dlsym(module, "sample32");
    double (*original64)(double *) = dlsym(module, "sample64");
    void (*original_both)(float *, float *, double *) = dlsym(module, "sample_both");
    if (!original32 || !original64 || !original_both) return 4;
    unsigned short saved;
    __asm__ volatile("fnstcw %0" : "=m"(saved));
    for (unsigned rounding = 0; rounding < 4; ++rounding) {
        unsigned short control = (saved & ~0xc00u) | (rounding << 10);
        for (int i = -1024; i < 1024; ++i) {
            float input32 = i * 0.75f + 0.375f;
            double input64 = i * 1234567.75 + 0.375;
            __asm__ volatile("fldcw %0" :: "m"(control) : "memory");
            float expected32 = original32(&input32);
            double expected64 = original64(&input64);
            float expected_bits; double expected_numeric;
            original_both(&input32, &expected_bits, &expected_numeric);
            float actual32 = sample32(&input32, control);
            double actual64 = sample64(&input64, control);
            float actual_bits; double actual_numeric;
            sample_both(&input32, &actual_bits, &actual_numeric, control);
            __asm__ volatile("fldcw %0" :: "m"(saved) : "memory");
            if (memcmp(&expected32, &actual32, sizeof actual32)
                || memcmp(&expected64, &actual64, sizeof actual64)
                || memcmp(&expected_bits, &actual_bits, sizeof actual_bits)
                || memcmp(&expected_numeric, &actual_numeric, sizeof actual_numeric)) {
                fprintf(stderr, "storage mismatch at rounding=%u input=%d\n", rounding, i);
                return 1;
            }
        }
    }
    dlclose(module);
    puts("32768 native storage comparisons passed");
    return 0;
}
