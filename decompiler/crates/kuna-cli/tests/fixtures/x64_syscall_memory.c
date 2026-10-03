int sink;
static __attribute__((always_inline)) inline void raw(int number, int fd, int *p) {
  register long a asm("rax") = number;
  register long b asm("rdi") = fd;
  register long c asm("rsi") = (long)p;
  register long d asm("rdx") = 4;
  register long e asm("r10") = 0, f asm("r8") = 0, g asm("r9") = 0;
  asm volatile("syscall" : "+r"(a), "+r"(b), "+r"(c), "+r"(d), "+r"(e), "+r"(f), "+r"(g) : : "rcx", "r11", "memory");
}
int pointer_before(int *p) { int v = *p; raw(0, 17, p); return v * 5 + *p; }
int pointer_expr(int *p, int x) { int v = *p * x; raw(0, 17, p); return v + *p; }
int pointer_twice(int *p) { raw(0, 17, p); int v = *p; raw(0, 17, p); return v * 5 + *p; }
int global_before(int x) { int v = sink * x; raw(0, 17, &sink); return v + sink; }
int global_twice(void) { raw(0, 17, &sink); int v = sink; raw(0, 17, &sink); return v * 5 + sink; }
int global_store(int x) { sink = x + 7; raw(1, 18, &sink); sink = 0; return x + 7; }
int pointer_store(int *p, int x) { *p = x + 7; raw(1, 18, p); *p = 0; return x + 7; }
int global_reload(int x) { int v = x + 7; sink = v; raw(0, 17, &sink); return v * 5 + sink; }
int barrier(int *p) { int v = *p; asm volatile("lfence" ::: "memory"); return v * 5 + *p; }
int plain(int *p, int x) { return *p * x + *p; }
static __attribute__((always_inline)) inline long raw_length(int fd, int *p, int size) {
  register long a asm("rax") = 0, b asm("rdi") = fd;
  register long c asm("rsi") = (long)p, d asm("rdx") = size;
  register long e asm("r10") = 0, f asm("r8") = 0, g asm("r9") = 0;
  asm volatile("syscall" : "+r"(a), "+r"(b), "+r"(c), "+r"(d), "+r"(e), "+r"(f), "+r"(g) : : "rcx", "r11", "memory");
  return a;
}
int return_snapshot(int *p) {
  long a = raw_length(17, p, 4), b = raw_length(18, p, 2);
  return a * 17 + b + *p;
}
