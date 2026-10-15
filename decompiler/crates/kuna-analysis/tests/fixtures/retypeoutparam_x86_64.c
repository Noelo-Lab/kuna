/* Locals the function only passes by address, as `read`'s buffer.
   clang -O2 -o retypeoutparam_x86_64 retypeoutparam_x86_64.c */
#include <unistd.h>

__attribute__((noinline)) int rd(void)
{
    char c = 0;
    if (read(0, &c, 1) != 1)
        return -1;
    return c;
}

__attribute__((noinline)) int rdn(void)
{
    int n = 0;
    if (read(0, &n, sizeof n) != sizeof n)
        return -1;
    return n;
}

int main(void)
{
    return rd() + rdn();
}
