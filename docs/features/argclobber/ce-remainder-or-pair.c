/* The fmt shape with a callee that really takes the third argument.
   rdx at the call is a join of a division remainder (passed on purpose) and
   the high half of a 16-byte struct return; the callee reads it past a
   jump table. */
struct pair { long a; long b; };

__attribute__((noinline)) struct pair mk(long x)
{
    struct pair p = { x * 3, x ^ 0x55 };
    return p;
}

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
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return use3(c, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{
    return (int)caller(argc & 7, (long)argv, argc + 1);
}
