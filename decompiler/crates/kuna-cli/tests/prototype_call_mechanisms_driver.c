#include <dlfcn.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

typedef void (*sample_fn)(double *, double *);
static uint64_t random_state = UINT64_C(73919);
static uint64_t next(void) {
    random_state ^= random_state << 13;
    random_state ^= random_state >> 7;
    random_state ^= random_state << 17;
    return random_state;
}
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    void *original = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    void *regenerated = dlopen(argv[2], RTLD_NOW | RTLD_LOCAL);
    if (!original || !regenerated) return 2;
    sample_fn reference = (sample_fn)dlsym(original, "sample");
    sample_fn actual = (sample_fn)dlsym(regenerated, "sample");
    if (!reference || !actual) return 2;
    const double seeds[4][4] = {
        {0., -0., 0., -0.}, {1., 2., 4., 8.},
        {1e30, 1., -1e30, 3.}, {1e-200, -1e200, 1e200, -1e-200}
    };
    const double special[5] = {INFINITY, -INFINITY, NAN, 0., -0.};
    unsigned mismatches = 0;
    for (unsigned test = 0; test < 280; ++test) {
        double values[4];
        if (test < 4) memcpy(values, seeds[test], sizeof values);
        else if (test < 260) {
            for (unsigned j = 0; j < 4; ++j) {
                double fraction = (double)(int32_t)(next() >> 32) / 2147483648.0;
                int exponent = (int)(next() % 1800) - 900;
                values[j] = ldexp(fraction, exponent);
            }
        } else {
            const double ordinary[4] = {2., 3., 5., 7.};
            memcpy(values, ordinary, sizeof values);
            values[(test - 260) % 4] = special[(test - 260) / 4];
        }
        double expected, observed;
        reference(values, &expected);
        actual(values, &observed);
        if (isnan(expected) && isnan(observed)) continue;
        if (memcmp(&expected, &observed, sizeof expected)) {
            if (mismatches++ == 0)
                fprintf(stderr, "case %u: expected %a, observed %a\n", test, expected, observed);
        }
    }
    dlclose(original);
    dlclose(regenerated);
    return mismatches ? 1 : 0;
}
