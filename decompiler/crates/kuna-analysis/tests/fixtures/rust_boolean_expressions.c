#include <stdbool.h>
extern void barrier(void);
extern bool observe(int value);

bool loop_xor(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) {
        barrier();
        value = value ^ (count == 3);
    }
    return value & (p == 0);
}

bool loop_and(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) {
        barrier();
        value = value & (count != 3);
    }
    return value | (p == 0);
}

bool loop_or(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) {
        barrier();
        value = value | (count == 3);
    }
    return value ^ (p == 0);
}

bool loop_eager(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) value = value ^ observe(count);
    return value & (p == 0);
}

bool loop_and_call(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) value = value & observe(count);
    return value & (p == 0);
}

bool loop_or_call(int a, int b, int count, void *p) {
    bool value = a == b;
    while (count-- > 0) value = value | observe(count);
    return value & (p == 0);
}

void loop_store(bool *out, int a, int b, int count) {
    bool value = a == b;
    while (count-- > 0) {
        barrier();
        value = value ^ (count == 3);
        *out = value & (a == 4);
    }
}

int main(void) { return 0; }
