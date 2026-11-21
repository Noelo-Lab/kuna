typedef unsigned int u32;
typedef unsigned long long u64;

extern int ext_log(int);
extern void ext_fill(int *, int);

int g_counter = 7;
static const int table[8] = {3, 1, 4, 1, 5, 9, 2, 6};

int sum(const int *p, int n) {
  int s = 0;
  for (int i = 0; i < n; i++) s += p[i];
  return s;
}

int pick(int x, int y) {
  switch (x) {
    case 0: y += 3; break;
    case 1: y *= 5; break;
    case 2: y -= ext_log(y); break;
    case 3: y ^= 0x55; break;
    default: y = 0;
  }
  return y + 1;
}

int bump(void) {
  g_counter += table[g_counter & 7];
  return ext_log(g_counter);
}

static int add(int a, int b) { return a + b; }
static int sub(int a, int b) { return a - b; }
static int mul(int a, int b) { return a * b; }
int (*ops[3])(int, int) = {add, sub, mul};
int apply(int k, int a, int b) { return ops[k](a, b); }

int frame(int a) {
  int buf[4];
  ext_fill(buf, a);
  return buf[0] + buf[3];
}

u64 mix(u64 a, u32 b) { return (a << 7 | a >> 57) ^ (u64)b * 0x9e3779b97f4a7c15ull; }

double favg(double a, double b) { return (a + b) * 0.5; }
