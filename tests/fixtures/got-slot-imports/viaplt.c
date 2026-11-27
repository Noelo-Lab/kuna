/* Compiled with the PLT: calls the same imports through their stubs, so a link
   with calls.o gives each import both a GOT slot and a .plt.got stub. */
typedef __SIZE_TYPE__ size_t;
void free(void *);
size_t strlen(const char *);
int memcmp(const void *, const void *, size_t);
void exit(int) __attribute__((noreturn));

void drop(void *p) { free(p); }
size_t len2(const char *s) { return strlen(s); }
int cmp2(const void *a, const void *b, size_t n) { return memcmp(a, b, n); }
void bail(void) { exit(1); }
