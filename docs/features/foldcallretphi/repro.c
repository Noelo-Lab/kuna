#include <stdio.h>
__attribute__((noinline)) int work(FILE *f) { return fgetc(f) >= 0; }
int main(int argc, char **argv)
{
    int ok = 1;
    for (int i = 1; i < argc; i++)
        ok &= work(stdin);
    return ok;
}
