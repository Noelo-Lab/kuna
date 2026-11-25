typedef __SIZE_TYPE__ size_t;
void free(void *);
size_t strlen(const char *);
void release(void *p) { free(p); }
size_t measure(const char *s) { return strlen(s) + 1; }
