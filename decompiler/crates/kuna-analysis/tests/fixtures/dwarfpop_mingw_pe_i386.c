/* kuna GH-1920: __cdecl callees (`ret`) that the DWARF of a MinGW i386 PE
   declares without a calling convention. add_cache calls clear on one path
   of two that merge before a call to copy; clear calls the msvcrt import
   puts, so its stack pointer at `ret` is not proven, only its `ret`. version
   calls fill, which calls nothing, and then query on a local struct.

   Build (MinGW-w64 GCC 15.2.0): i686-w64-mingw32-gcc -O2 -g
          -fno-tree-loop-distribute-patterns -fno-inline -nostdlib
          -Wl,-e,__start -o dwarfpop_mingw_pe_i386.exe
          dwarfpop_mingw_pe_i386.c -lmsvcrt */
int puts(const char *);

unsigned char cache[0x100];
unsigned int used;

void *fill(void *dst, int c, unsigned int n)
{
    unsigned char *p = dst;
    while (n--)
        *p++ = (unsigned char)c;
    return dst;
}

void *clear(void *dst, int c, unsigned int n)
{
    if (n > 0x80)
        puts("big");
    return fill(dst, c, n);
}

void *copy(void *dst, const void *src, unsigned int n)
{
    unsigned char *d = dst;
    const unsigned char *s = src;
    while (n--)
        *d++ = *s++;
    return dst;
}

void add_cache(unsigned char *ptr, unsigned int len)
{
    if (used + len > sizeof cache) {
        clear(cache, 0, used);
        used = 0;
    }
    copy(cache + used, ptr, len);
    used += len;
}

struct info {
    int kind, major, minor, build;
};

void query(struct info *out)
{
    out->kind = 1;
    out->major = 6;
}

int version(void)
{
    struct info si;
    fill(&si, 0, sizeof si);
    si.build = 0x24;
    query(&si);
    return si.major * 10 + si.minor + si.build;
}

void _start(void)
{
    unsigned char b[4] = {1, 2, 3, 4};
    add_cache(b, sizeof b);
    used = version();
}
