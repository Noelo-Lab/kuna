/* macho_cstringconst_guard fixture: an arm64 Mach-O dylib at base 0 whose
   integer arguments land inside __cstring without being literals: a size
   setvbuf declares size_t, at the first byte of "hello world", and a value
   passed through an untyped function pointer two bytes into it. */
typedef unsigned long size_t;
extern int setvbuf(void *, char *, int, size_t);
extern void (*sink)(void *, void *, void *);
void sized(void *fp) { setvbuf(fp, 0, 0, 0x58b); }
void mid(void *ctx) { sink(ctx, (void *)0x58d, 0); }
void lit(void *ctx) { sink(ctx, "id", "hello world"); }
