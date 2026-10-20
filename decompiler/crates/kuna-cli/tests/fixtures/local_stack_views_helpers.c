#include "local_stack_views.h"
#include <string.h>

int datum = 55;
int *escaped_pointer;

int read_pair(const struct LifetimePair *pair) {
    return pair->x + pair->y;
}

int read_handle(const struct LifetimeHandle *handle) {
    return *handle->value;
}

int read_escaped(void) {
    return *escaped_pointer;
}

void write_saved(void) {
    *escaped_pointer = 7;
}

void set_seven(int *value) {
    *value = 7;
}

int read_saved_handle(void) {
    const int *value;
    memcpy(&value, escaped_pointer, sizeof(value));
    return *value;
}

int same_address(const struct LifetimePair *pair, const struct LifetimeHandle *handle) {
    return (const void *)pair == (const void *)handle ? *handle->value + 1 : 0;
}

unsigned long long read_range_pair(const struct LifetimePair *pair) {
    unsigned long long bits;
    memcpy(&bits, pair, sizeof(bits));
    return bits;
}

unsigned long long read_range_handle(const struct LifetimeHandle *handle) {
    unsigned long long bits;
    memcpy(&bits, handle, sizeof(bits));
    return bits;
}
