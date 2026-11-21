/* One small program for every container: two data words, a table, a
 * function pointer, a zero-initialized word and four functions. HOSTED adds
 * an imported routine; START adds the entry stub a boot ROM jumps to. */
#ifdef HOSTED
extern int puts(const char *);
#endif

int counter = 7;
int table[4] = {1, 2, 3, 4};
int scratch;
int scale(int x);
int (*handler)(int) = scale;

__attribute__((noinline)) int checksum(const char *text, int count)
{
    int total = 0;
    for (int index = 0; index < count; index++)
        total = total * 31 + text[index];
    return total;
}

__attribute__((noinline)) int scale(int x)
{
    return x * 3 + counter;
}

__attribute__((noinline)) int sum(int a, int b)
{
    scratch = a;
    return handler(a) + table[b & 3];
}

int main(void)
{
#ifdef HOSTED
    puts("retro");
#endif
    return sum(counter, checksum("retro", 5)) + scratch;
}

#ifdef START
__attribute__((section(".text.start"))) void _start(void)
{
    main();
    for (;;)
        ;
}
#endif
