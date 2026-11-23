/* gotonext stage fixture: gcc / clang -O2 -fno-pie -no-pie -nostartfiles */
__attribute__((noinline)) int sink(int x) { return x * 7 + 3; }
__attribute__((noinline)) int power(int a, int b) { int r = 1; while (b-- > 0) r *= a; return r; }

int fall(int x, int *p) {
  switch (x) {
  case 0: p[0] = sink(1);
  case 1: p[1] = sink(2); break;
  case 2: p[2] = sink(3);
  case 3: p[3] = sink(4);
  case 4: p[4] = sink(x); break;
  case 5: p[7] = sink(9); break;
  case 6: p[8] = sink(5);
  case 7: p[9] = sink(4); break;
  default: return -1;
  }
  return p[5];
}

int arith(int op, int a, int b) {
  switch (op) {
  case 0: return a + b;
  case 1: return a - b;
  case 2: return a * b;
  case 3: return sink(a) - b;
  case 4: return (b == 2) ? a * a : power(a, b);
  case 5: return a / b;
  case 6: return sink(a / b);
  case 7: return -a;
  default: return 0;
  }
}

int strcase(const char *s, const char *t) {
  for (int i = 0; i < 6; i++) {
    for (int j = 0;; j++) {
      unsigned char c = s[j], d = t[j];
      if (c - 'a' < 26u) c -= 32;
      if (d - 'a' < 26u) d -= 32;
      if (c != d) break;
      if (!c) return i;
    }
    t += 16;
  }
  return -1;
}
