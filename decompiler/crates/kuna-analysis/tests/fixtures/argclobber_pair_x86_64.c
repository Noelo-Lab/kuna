/* The counterexample the recovered-prototype clause declines: rdx at the call
   is a join of a real remainder and the high half of a 16-byte rax:rdx struct
   return, and `use3` reads it past a jump table. Its recovered prototype takes
   three parameters, so `argclobber` keeps the argument.

   gcc -O2 -fno-inline -nostdlib -static -o argclobber_pair_x86_64 argclobber_pair_x86_64.c && strip */
struct pair { long a; long b; };
__attribute__((noinline)) struct pair mk(long x) { struct pair p = { x * 3, x ^ 0x55 }; return p; }
__attribute__((noinline)) long use3(int op, long a, long b)
{
    switch (op) {
    case 0: return a + b;
    case 1: return a - b * 7;
    case 2: return a * b;
    case 3: return (a ^ b) + 11;
    case 4: return b << 3;
    case 5: return (a | b) - 5;
    case 6: return a & ~b;
    default: return 0;
    }
}
__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c) p.a = a / d, p.b = a % d; else p = mk(a);
    return use3(c, p.a, p.b) + 1;
}
void _start(void) { caller(3, 7, 2); }
