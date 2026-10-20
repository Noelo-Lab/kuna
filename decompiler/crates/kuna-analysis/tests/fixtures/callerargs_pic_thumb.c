/* clang --target=armv7a-linux-gnueabihf -mthumb -fPIC -O2 -fno-inline -c callerargs_pic_thumb.c
   ld.lld -shared -o callerargs_pic_thumb callerargs_pic_thumb.o --strip-debug
   cb is called with bytes and also passed to apply, which calls it with a word: the
   address is built PC-relatively (ldr rX, [pc, #]; add rX, pc), so cb keeps its width. */
typedef unsigned char UC; typedef unsigned U;
#define NI __attribute__((noinline))
NI static U cb(U v, U k) { return v * 3u + (v >> 9) + k; }
NI U apply(U (*fn)(U, U), U x) { return fn(x, 7); }
NI U f0000(UC *p, U x, U y) { return cb((UC)x, y) + cb((UC)(x >> 8), y) + apply(cb, x * 1000u + y); }
