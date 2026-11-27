/* Stands in for the C library at link time. */
void free(void *p) { (void)p; }
unsigned long strlen(const char *s) { unsigned long n = 0; while (s[n]) n++; return n; }
