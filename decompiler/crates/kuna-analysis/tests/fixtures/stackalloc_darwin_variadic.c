#include <stdarg.h>
__attribute__((noinline)) int sink(int n, ...){ va_list ap; va_start(ap,n); char *a=va_arg(ap,char*); int s=a[0]; for(int i=0;i<9;i++) s=s*3+va_arg(ap,int); va_end(ap); return s; }
__attribute__((noinline)) int sample_vla_variadic(int n){ char a[n]; a[0]=7; __asm__ volatile("" ::: "memory"); return sink(n, a, n, 1, 2, 3, 4, 5, 6, 7, 8); }
int main(void){ return sample_vla_variadic(3); }
