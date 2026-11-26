/* kuna `rangetest` fixture: range tests that gcc -O2 folds into one unsigned
   comparison of a subtraction, a wrapping one, a loop test, a switch and a
   test that heads a chain nested in another.
   Built with Ubuntu 22.04's gcc 11.4:
     gcc -O2 -fno-inline -o rangetest_gcc_O2_x86_64 rangetest.c  */
#include <stdio.h>

volatile int knob;

void printable(unsigned char c)
{
    if (c >= 0x20 && c <= 0x7e)
        knob = c;
}

int bits(int n)
{
    if (n < 1 || n > 30)
        return -1;
    knob = n;
    return 0;
}

void reject(int n)
{
    if (n < 1 || n > 30)
        knob = n;
}

void digit(unsigned char c)
{
    if (c >= '0' && c <= '9')
        knob = c;
}

int lower_after(int a, char c)
{
    if (a > 2 && c >= 'a' && c <= 'z')
        return 1;
    return 0;
}

int isdig(int c)
{
    return c >= '0' && c <= '9';
}

int is_port(unsigned short p)
{
    return p >= 1024 && p <= 49151;
}

long scaled(unsigned long n)
{
    if (n == 0 || n > 0x1fffffffffffffffUL)
        return -1;
    return (long)(n * 8);
}

int small(int x)
{
    return x >= -16 && x < 16;
}

int number(const char *s)
{
    int n = 0;
    while (*s >= '0' && *s <= '9') {
        n = n * 10 + (*s - '0');
        s++;
    }
    return n;
}

int pick(int x)
{
    switch (x) {
    case 3: knob = 1; break;
    case 4: knob = 7; break;
    case 5: knob = 2; break;
    case 6: knob = 9; break;
    case 7: knob = 5; break;
    case 8: knob = 4; break;
    default: return -1;
    }
    return 0;
}

int ctl(int b, int c)
{
    int n = 0;
    while (b-- > 0) {
        if (b == 7 || ((c < 9 || c > 10) && c != 5))
            n += c;
        knob = b;
    }
    return n;
}

#ifndef RANGETEST_NO_MAIN
int main(int argc, char **argv)
{
    (void)argv;
    printable((unsigned char)argc);
    reject(argc);
    return bits(argc) + isdig(argc) + is_port((unsigned short)argc) + (int)scaled((unsigned long)argc)
        + small(argc) + lower_after(argc, 'q') + number("12") + pick(argc);
}
#endif
