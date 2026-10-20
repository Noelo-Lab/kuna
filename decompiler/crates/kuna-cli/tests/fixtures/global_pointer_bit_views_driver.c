#include <stdint.h>
#include <string.h>
#include <dlfcn.h>
#include <stdio.h>
_Alignas(8) unsigned char storage[32];
void sample(void **, double *, double *);
int main(int argc, char **argv) {
    if (argc != 2 || sizeof(void *) != 8) return 2;
    void *module = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!module) return 3;
    void (*original)(void **, double *, double *) = dlsym(module, "sample");
    if (!original) return 4;
    for (unsigned i = 0; i < 4096; ++i) {
        uint64_t payload = UINT64_C(0x3ff0000000000000) + (uint64_t)(i / 2) * UINT64_C(0x100000000);
        if (i & 1) payload ^= UINT64_C(0x8000000000000000);
        void *value = (void *)(uintptr_t)payload;
        double expected_bits, expected_numeric, actual_bits, actual_numeric;
        original(&value, &expected_bits, &expected_numeric);
        sample(&value, &actual_bits, &actual_numeric);
        if (memcmp(&expected_bits, &actual_bits, 8) || memcmp(&expected_numeric, &actual_numeric, 8)) {
            fprintf(stderr, "mismatch at case %u\n", i);
            return 1;
        }
    }
    dlclose(module);
    puts("8192 native comparisons passed");
    return 0;
}
