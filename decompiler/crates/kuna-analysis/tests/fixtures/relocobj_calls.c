/* Calls and addresses in PowerPC and MIPS relocatable objects (GH-904). */
typedef unsigned long long u64;
#define NI __attribute__((noinline))
extern u64 sink64;
extern int sink32;
u64 ext64(u64);
int extint(int);
NI u64 add64(u64 a, u64 b) { return a + b; }
void store(u64 a, u64 b) { sink64 = add64(a, b); }
int callint(int x) { return extint(x) + sink32; }
u64 wrap(u64 a) { return ext64(a) + 1; }
