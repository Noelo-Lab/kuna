/* clang -O2 -g -o charsign_x86_64_clang_O2 charsign.c
   gcc -O2 -g -o charsign_x86_64_gcc_O2 charsign.c
   clang --target=aarch64-linux-gnu -O2 -g -c -o charsign_aarch64_O2.o charsign.c
   (clang 14.0.0, gcc 11.4.0) */
#define KEEP __attribute__((noinline))
typedef unsigned char u8;
int sprintf(char *, const char *, ...);

KEEP int fmt(char *b, const char *f, unsigned char c) { return sprintf(b, f, c); }
KEEP int fmts(char *b, const char *f, signed char c) { return sprintf(b, f, c); }
KEEP int fmt8(char *b, const char *f, u8 c) { return sprintf(b, f, c); }
KEEP int fmtc(char *b, const char *f, char c) { return sprintf(b, f, c); }
KEEP int is_hi(unsigned char c) { return c >= 0x80; }
KEEP long wid(unsigned char c) { return c; }
KEEP long wids(signed char c) { return c; }
KEEP long wus(unsigned short c) { return c; }
KEEP long wss(short c) { return c * 3L; }
KEEP int wb(_Bool b) { return b + 5; }
KEEP unsigned long ulen(const char *s) { unsigned long n = 0; while (s[n]) n++; return n; }
KEEP int rd1(unsigned char *b, int v) { b[0] = v; return 1; }
KEEP int getc1(int v) { unsigned char b[1]; rd1(b, v); return b[0]; }

int main(int argc, char **argv)
{
    char b[16];
    return fmt(b, argv[0], argc) + fmts(b, argv[0], argc) + fmt8(b, argv[0], argc) + fmtc(b, argv[0], argc) + is_hi(argc) + (int)wid(argc)
        + (int)wids(argc) + (int)wus(argc) + (int)wss(argc) + wb(argc) + (int)ulen(argv[0]) + getc1(argc);
}
