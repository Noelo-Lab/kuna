int scale(int x) { return x * 3 + 7; }
int kernel_entry(int x) { int s = 0; for (int i = 0; i < x; i++) s += scale(i); return s; }
