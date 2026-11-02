/* Same-image calls and data accesses through a MIPS PIC GOT. */
typedef unsigned long long u64;
u64 sink64;
static int counter;
int gdata = 5;
extern int puts(const char *);
__attribute__((noinline)) u64 add64(u64 a, u64 b) { return a + b; }
__attribute__((noinline, weak)) int wfn(int x) { return x + 11; }
__attribute__((noinline, visibility("hidden"))) int hfn(int x) { return x - 7; }
void c_add(u64 a, u64 b) { sink64 = add64(a, b); }
int c_calls(int x) { counter++; return wfn(x) + gdata; }
int c_puts(void) { return puts("static string"); }
int (*c_ptr(void))(int) { return hfn; }
