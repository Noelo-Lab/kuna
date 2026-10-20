#include "local_stack_views.h"

__attribute__((noinline)) int local_constant(void) {
    int result;
    {
        struct LifetimePair pair = {5, 7};
        result = read_pair(&pair);
    }
    {
        struct LifetimeHandle handle = {&datum};
        result += read_handle(&handle);
    }
    return result;
}

__attribute__((noinline)) int local_register(int a, int b) {
    int result;
    {
        struct LifetimePair pair = {a, b};
        result = read_pair(&pair);
    }
    {
        struct LifetimeHandle handle = {&datum};
        result += read_handle(&handle);
    }
    return result;
}

__attribute__((noinline)) int local_partial(void) {
    int result;
    {
        struct LifetimePair pair = {5, 7};
        result = read_pair(&pair);
        pair.y = 20;
        result += read_pair(&pair);
    }
    {
        struct LifetimeHandle handle = {&datum};
        result += read_handle(&handle);
    }
    return result;
}

__attribute__((noinline)) int local_branch(int condition) {
    int result;
    {
        struct LifetimePair pair = {5, 7};
        if (condition)
            pair.x = 100;
        result = read_pair(&pair);
    }
    {
        struct LifetimeHandle handle = {&datum};
        result += read_handle(&handle);
    }
    return result;
}

int main(void) {
    return local_constant() != 67 || local_register(3, 4) != 62 ||
           local_partial() != 92 || local_branch(0) != 67 ||
           local_branch(1) != 162 || local_branch(-1) != 162;
}
