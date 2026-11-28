/* Link-time stand-in for the C library; not shipped. */
typedef __SIZE_TYPE__ size_t;
void free(void *p) { (void)p; }
size_t strlen(const char *s) { size_t n = 0; while (s[n]) ++n; return n; }
int memcmp(const void *a, const void *b, size_t n) { (void)a; (void)b; return (int)n; }
void exit(int c) { for (;;) (void)c; }
int counter;
