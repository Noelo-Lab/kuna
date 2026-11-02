/* option bejoin callers through MIPS PIC calls and unaligned loads (GH-904). */
typedef unsigned long long u64;
#define NI __attribute__((noinline))
struct __attribute__((packed)) unal { unsigned char tag; unsigned word; };
u64 sink64;
unsigned sink32;
NI u64 add64(u64 a, u64 b) { return a + b; }
NI unsigned add_hi(u64 a, u64 b) { return (a + b) >> 32; }
NI u64 pass(u64 a, u64 b) { u64 r = add64(a, b); sink32 = 1; return r; }
void c_add(u64 a, u64 b) { sink64 = add64(a, b); }
void c_hi(u64 a, u64 b) { sink32 = add_hi(a, b); }
void c_pass(u64 a, u64 b) { sink64 = pass(a, b) + 1; }
unsigned c_unal(u64 a, u64 b, struct unal *u) { unsigned h = add_hi(a, b); return h + u->word; }
