/* Default local names stay put when a directive renames one local.
   gcc -O2 -fno-inline -nostdlib -static -o stablenames_x86_64 stablenames_x86_64.c */

struct rec { long key; long val; };

__attribute__((noinline)) int is_vowel(int c)
{
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

__attribute__((noinline)) long span(const char *s)
{
    long n = 0;
    while (s[n])
        n++;
    return n;
}

__attribute__((noinline)) void format_pair(char *out, int a, int b)
{
    out[0] = (char)('0' + a % 10);
    out[1] = ':';
    out[2] = (char)('0' + b % 10);
    out[3] = 0;
}

__attribute__((noinline)) long mix(const char *s, int n, struct rec *out)
{
    long h = 5381;
    long acc = 0;
    int vowels = 0;
    int digits = 0;
    long best = -1;
    int besti = -1;
    char buf[32];
    for (int i = 0; i < n && s[i]; i++) {
        int c = (unsigned char)s[i];
        h = h * 33 + c;
        if (is_vowel(c))
            vowels++;
        if (c >= '0' && c <= '9') {
            digits++;
            acc = acc * 10 + (c - '0');
        }
        if (h > best) {
            best = h;
            besti = i;
        }
    }
    format_pair(buf, vowels, digits);
    out->key = h ^ acc;
    out->val = span(buf) + besti;
    long total = 0;
    for (long *p = &out->key; p <= &out->val; p++)
        total += *p;
    return total + best;
}

void _start(void)
{
    static const char text[] = "a1b2c3 hello 42";
    struct rec r;
    long v = mix(text, (int)span(text), &r);
    __asm__ volatile("mov %0, %%rdi\n\tmov $60, %%eax\n\tsyscall" :: "r"(v & 0x7f) : "rdi", "rax");
    for (;;) {
    }
}
