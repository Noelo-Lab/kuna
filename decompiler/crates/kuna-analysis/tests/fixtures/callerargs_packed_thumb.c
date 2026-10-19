/* clang --target=thumbv7em-none-eabi -mcpu=cortex-m4 -O2 -c callerargs_packed_thumb.c
   ld.lld -o callerargs_packed_thumb callerargs_packed_thumb.o --nmagic -Ttext=0x20000 -e f0000 --strip-debug
   cbb is called with truth values and also through a packed table entry with x + 5: the
   pointer sits at an unaligned address, so cbb keeps its int parameter. */
typedef unsigned char UC; typedef unsigned U;
#define NI __attribute__((noinline))
NI static U cbb(int flag, U k) { return (U)flag * 100u + k; }
struct __attribute__((packed)) ent { UC tag; U (*fn)(int, U); };
struct ent tbl[2] = { { 1, cbb }, { 2, 0 } };
NI U dispatch(int i, U x) { return tbl[i].fn ? tbl[i].fn((int)x, 7) : 0; }
NI U f0000(UC *p, U x, U y) { return cbb(x > y, 1) + cbb(0, 2) + dispatch(0, x + 5); }
