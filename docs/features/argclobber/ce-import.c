/* The ce-remainder-or-pair.c caller, with a callee imported from a shared
   library that is in no signature table: the entry probe decodes the PLT
   stub, meets its indirect jump, and proves nothing. */
struct pair { long a; long b; };
long ext3(int op, long a, long b);

__attribute__((noinline)) struct pair mk(long x)
{
    struct pair p = { x * 3, x ^ 0x55 };
    return p;
}

__attribute__((noinline)) long caller(int c, long a, long d)
{
    struct pair p;
    if (c)
        p.a = a / d, p.b = a % d;
    else
        p = mk(a);
    return ext3(c, p.a, p.b) + 1;
}

int main(int argc, char **argv)
{
    return (int)caller(argc & 7, (long)argv, argc + 1);
}
