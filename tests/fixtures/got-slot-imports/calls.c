/* Compiled with -fno-plt: every call and tail jump goes through the import's GOT slot. */
typedef __SIZE_TYPE__ size_t;
void free(void *);
size_t strlen(const char *);
int memcmp(const void *, const void *, size_t);
void exit(int) __attribute__((noreturn));
extern int counter;
int seen;

void release(void *p) { free(p); }
size_t length(const char *s) { return strlen(s); }
int same(const void *a, const void *b, size_t n) { return memcmp(a, b, n); }
int first(const char *s) { return s[0] + (int)strlen(s); }
int check(int x) { if (x < 0) exit(2); seen = x; return x * 2; }
int bump(void) { return ++counter; }
