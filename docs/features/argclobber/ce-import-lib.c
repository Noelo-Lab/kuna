/* Build: gcc -O2 -shared -fPIC -o libce.so ce-import-lib.c */
long ext3(int op, long a, long b)
{
    return op ? a + b : a - b;
}
