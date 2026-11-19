// No-return names from public headers that the vendored lists lacked:
// glibc _Exit / thrd_exit and libstdc++ std::rethrow_exception. Each
// function's failure path ends in one of them, so the next function follows.
#include <cstdlib>
#include <exception>
#include <threads.h>

extern "C" __attribute__((noinline)) int quit_now(int x)
{
    if (x > 3)
        _Exit(x);
    return x + 1;
}

extern "C" __attribute__((noinline)) int quit_thread(int x)
{
    if (x > 4)
        thrd_exit(x);
    return x + 2;
}

extern "C" __attribute__((noinline)) int replay(std::exception_ptr *p, int x)
{
    if (x > 5)
        std::rethrow_exception(*p);
    return x * 5;
}

int main(int argc, char **)
{
    std::exception_ptr p;
    return quit_now(argc) + quit_thread(argc) + replay(&p, argc);
}
