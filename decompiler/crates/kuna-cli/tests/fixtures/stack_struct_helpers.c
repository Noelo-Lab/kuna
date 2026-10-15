struct header { unsigned int magic; unsigned short version; unsigned char width; unsigned char height; unsigned int count; unsigned char *pixels; };
void fetch(void *dst, unsigned long n) { unsigned char *d = dst; for (unsigned long i = 0; i < n; i++) d[i] = (unsigned char)(n - i); }
void setup(struct header *h) { h->count = h->width * h->height; h->pixels = (unsigned char *)h; }
int report(unsigned int w, unsigned int h, unsigned long sum) { return (int)(w * 1000 + h * 100 + sum); }
void *memset(void *p, int c, unsigned long n) { unsigned char *d = p; while (n--) *d++ = (unsigned char)c; return p; }
