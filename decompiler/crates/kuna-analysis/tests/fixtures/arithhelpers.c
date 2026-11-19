/* Fixture for the `arithhelpers` call-fixups: 64-bit (and ARM 32-bit)
   arithmetic a 32-bit target delegates to a runtime helper.   i386:  gcc -m32 -O2 -g -fno-pic -nostdlib -static -Wl,-e,udiv arithhelpers.c -lgcc -o arithhelpers_i386
   Thumb: clang --target=thumbv6m-none-eabi -O2 -g -c arithhelpers.c; ld.lld -e udiv with
          bx-lr stubs for the __aeabi_* helpers (arithhelpers_stubs_thumb.s). */
typedef unsigned long long u64;
typedef long long s64;
u64 udiv(u64 a, u64 b) { return a / b; }
s64 sdiv(s64 a, s64 b) { return a / b; }
u64 umod(u64 a, u64 b) { return a % b; }
s64 smod(s64 a, s64 b) { return a % b; }
s64 shl(s64 a, int n) { return a << n; }
u64 shr(u64 a, int n) { return a >> n; }
s64 sar(s64 a, int n) { return a >> n; }
s64 mul(s64 a, s64 b) { return a * b; }
unsigned uidiv(unsigned a, unsigned b) { return a / b; }
int idiv(int a, int b) { return a / b; }
int imod(int a, int b) { return a % b; }
int both(s64 a, s64 b, s64 *r) { *r = a % b; return (int)(a / b); }
