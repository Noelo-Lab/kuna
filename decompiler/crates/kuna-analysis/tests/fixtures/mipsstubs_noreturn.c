/* Calls through .MIPS.stubs lazy-binding stubs: returning imports and no-return ones. */
typedef unsigned long size_t;
typedef struct FILE FILE;
FILE *fopen(const char *, const char *);
int fputs(const char *, FILE *);
int fclose(FILE *);
void *memcpy(void *, const void *, size_t);
int puts(const char *);
int strcmp(const char *, const char *);
void exit(int) __attribute__((noreturn));
void abort(void) __attribute__((noreturn));
int sink;
int save(const char *path, const char *text) {
  FILE *f = fopen(path, "w");
  if (!f) return -1;
  fputs(text, f);
  fclose(f);
  return 0;
}
int copy(char *dst, const char *src, size_t n) {
  memcpy(dst, src, n);
  return dst[0] + 1;
}
int same(const char *a, const char *b) {
  int r = strcmp(a, b);
  sink = r;
  return r == 0;
}
void say(const char *m) {
  puts(m);
  sink = 2;
}
void die(int code) {
  puts("fatal");
  exit(code);
}
int check(int x) {
  if (x < 0) abort();
  return x * 2;
}
