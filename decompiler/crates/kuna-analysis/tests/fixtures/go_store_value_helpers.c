struct Fields { signed char s8; unsigned char u8; short s16; unsigned short u16; int s32; unsigned int u32; long s64; unsigned long u64; };
unsigned int pointer_calls, value_calls;
__attribute__((noinline,noclone)) struct Fields *counted_destination(struct Fields *p){pointer_calls++;return p;}
__attribute__((noinline,noclone)) unsigned long counted_value(unsigned long a,unsigned long b){value_calls++;return a+b;}
