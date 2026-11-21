
int sum(const int *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
void sink(int *p);
int fill(int n) { int buf[8]; for (int i = 0; i < 8; i++) buf[i] = i * n; sink(buf); return buf[n & 7]; }
unsigned clamp(unsigned a, unsigned b) { if (a > b) return (a << 3) ^ b; return (a >> 2) | (b & 0xff00ff); }
