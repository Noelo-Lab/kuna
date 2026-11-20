/* Fixture for `repstring`: copies, fills and compares gcc emits as rep movs/stos/cmps.
   gcc -m32 -Os -minline-all-stringops -fno-pic -fcf-protection=none -c repstring.c -o repstring_i386.o
   gcc      -Os -minline-all-stringops -fno-pic -fcf-protection=none -c repstring.c -o repstring_x86_64.o
   ld.lld -m elf_i386 -e fill32 -o repstring_i386 repstring_i386.o
   ld.lld -e fill32 -o repstring_x86_64 repstring_x86_64.o          (gcc 15.3, lld 22) */
void fill32(int *p, unsigned n) { __asm__ volatile("rep stosl" : "+D"(p), "+c"(n) : "a"(0x01020304) : "memory"); }
void zero32(int *p, unsigned n) { __asm__ volatile("rep stosl" : "+D"(p), "+c"(n) : "a"(0) : "memory"); }
void fill8(char *p, int c, unsigned n) { __asm__ volatile("rep stosb" : "+D"(p), "+c"(n) : "a"(c) : "memory"); }
void copy32(int *d, const int *s, unsigned n) { __asm__ volatile("rep movsl" : "+D"(d), "+S"(s), "+c"(n) : : "memory"); }
int *copy_end(int *d, const int *s, unsigned n) { __asm__ volatile("rep movsl" : "+D"(d), "+S"(s), "+c"(n) : : "memory"); return d; }
int isdash(const char *s) { return __builtin_strcmp(s, "-") == 0; }
int order(const char *a, const char *b) { return __builtin_memcmp(a, b, 5); }
int below(const char *a, const char *b) { return __builtin_memcmp(a, b, 7) < 0; }
int same_n(const char *a, const char *b, unsigned long n) { return __builtin_memcmp(a, b, n) == 0; }
int same5(const char *a, const char *b) { unsigned n = 5; unsigned char r; __asm__ volatile("repe cmpsb\n\tsete %0" : "=q"(r), "+S"(a), "+D"(b), "+c"(n) : : "memory", "cc"); return r; }
const char *mismatch(const char *a, const char *b) { unsigned n = 8; __asm__ volatile("repe cmpsb" : "+S"(a), "+D"(b), "+c"(n) : : "memory", "cc"); return a; }
