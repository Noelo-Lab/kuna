/* A classic main that reads NO arguments. Built gcc -O2 (a PIE). elfmain must
   name it int main(void) and leave its body untouched: the call to tick() must
   take no argument (locking argc/argv/envp would hand it a phantom one). */
#include <stdio.h>

__attribute__((noinline)) static int tick(void)
{
    static int n;
    return ++n;
}

int main(void)
{
    printf("%d\n", tick());
    return 0;
}
