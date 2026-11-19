/* rangefold fixture */
__attribute__((noinline)) int sink(int x) { return x * 7 + 3; }

int nested(int x) {
  if (x < 10) {
    if (x > 20)
      return sink(1);
    return sink(2);
  }
  return 0;
}

int masked(unsigned x, int *t) {
  if (x < 16)
    return t[x & 0xff];
  return -1;
}

int digit(unsigned char c) {
  if (c >= '0' && c <= '9') {
    if (c > '9')
      return -1;
    return c - '0';
  }
  return sink(c);
}
