/* clang -O0 on big-endian MIPS materializes a zero in both $2 and $3 and then
 * overwrites $2, so realeof and both, which return an int in $2, leave a
 * literal zero in $3: the low word of a returned long long. hi_only returns
 * that same shape on purpose. */
int realeof(unsigned a) { return a != 0 && a != 1; }
int both(int a, int b) { return a && b; }
unsigned long long hi_only(unsigned x) { return (unsigned long long)x << 32; }
