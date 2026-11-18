/* Self-checking nanoMIPS workload for the emulation test in tests/nanomips_core.rs.
   Every case maps a 32-bit seed to a 32-bit result using fixed-width arithmetic
   only, so a native build of this file gives the expected values. */
#include <stdint.h>
typedef uint32_t u32;
typedef int32_t s32;
typedef uint64_t u64;
typedef int64_t s64;
#define NOINLINE __attribute__((noinline))

struct node { s32 key; struct node *next; uint16_t tag; uint8_t flag; int8_t bias; };
struct wide { u32 w[9]; };
struct bits { u32 a : 5; u32 b : 11; s32 c : 7; u32 d : 9; };

static u32 state[16];
static struct node pool[12];
static struct wide blocks[3];
static uint8_t bytes[64];
static uint16_t halves[32];
u32 shared = 0x1234;
volatile u32 sink;

static NOINLINE u32 mix(u32 a, u32 b) { a ^= b + 0x9e3779b9u + (a << 6) + (a >> 2); return a; }
static NOINLINE u32 next(u32 *s) { *s = *s * 1664525u + 1013904223u; return *s; }

static NOINLINE u32 arith(u32 x)
{
    u32 r = x, s = x;
    for (int i = 0; i < 24; i++) {
        u32 a = next(&s), b = next(&s) | 1;
        r = mix(r, a + b); r = mix(r, a - b); r = mix(r, a * b);
        r = mix(r, a / b); r = mix(r, a % b);
        r = mix(r, (u32)((s32)a / (s32)b)); r = mix(r, (u32)((s32)a % (s32)b));
        r = mix(r, (u32)(((u64)a * b) >> 32)); r = mix(r, (u32)(((s64)(s32)a * (s32)b) >> 32));
        r = mix(r, a < b); r = mix(r, (s32)a < (s32)b); r = mix(r, a == b); r = mix(r, a < 100);
        r = mix(r, (s32)a < -77); r = mix(r, a ^ 0xabc); r = mix(r, a | 0x7ff); r = mix(r, a & 0xf0f);
        r = mix(r, ~a); r = mix(r, -a); r = mix(r, ~(a | b)); r = mix(r, a + 0x12345678u); r = mix(r, a - 4000);
    }
    return r;
}

static NOINLINE u32 shifts(u32 x)
{
    u32 r = x, s = x;
    for (int i = 0; i < 40; i++) {
        u32 a = next(&s), n = next(&s) >> 27;
        r = mix(r, a << n); r = mix(r, a >> n); r = mix(r, (u32)((s32)a >> n));
        r = mix(r, (a >> n) | (a << ((32 - n) & 31)));
        r = mix(r, a << 3); r = mix(r, a >> 8); r = mix(r, (u32)((s32)a >> 17)); r = mix(r, (a >> 13) | (a << 19));
        r = mix(r, (u32)__builtin_clz(a | 1)); r = mix(r, __builtin_bswap32(a)); r = mix(r, __builtin_bswap16((uint16_t)a));
        r = mix(r, (u32)(s32)(int8_t)a); r = mix(r, (u32)(s32)(int16_t)a); r = mix(r, (uint8_t)a); r = mix(r, (uint16_t)a);
        r = mix(r, (a >> 7) & 0x1ff); r = mix(r, (a << 2) + r); r = mix(r, (a << 3) + n);
    }
    return r;
}

static NOINLINE u32 wides(u32 x)
{
    u32 s = x; u64 acc = x;
    for (int i = 0; i < 20; i++) {
        u64 a = ((u64)next(&s) << 32) | next(&s), b = ((u64)next(&s) << 32) | next(&s);
        u32 n = next(&s) >> 26;
        acc += a * b; acc ^= a >> n; acc += b << n; acc ^= (u64)((s64)a >> n);
        acc += a / (b | 1); acc ^= a % ((b >> 40) | 1); acc += (u64)((s64)a / (s64)(b | 3));
        acc = (acc << 13) | (acc >> 51);
        acc += (a < b) + (((s64)a < (s64)b) << 1);
    }
    return (u32)acc ^ (u32)(acc >> 32);
}

static NOINLINE u32 fields(u32 x)
{
    u32 r = x, s = x; struct bits f = {0};
    for (int i = 0; i < 32; i++) {
        u32 a = next(&s);
        f.a = a; f.b = a >> 5; f.c = a >> 9; f.d += a >> 3;
        r = mix(r, f.a); r = mix(r, f.b); r = mix(r, (u32)f.c); r = mix(r, f.d);
        if (a & 0x400) r ^= 0x55aa; if (!(a & 0x80000)) r += 3; if (a & 1) r = ~r;
        if ((s32)a >= 0) r += 5; if ((s32)a <= 0) r ^= 9; if ((s32)a > 0) r -= 11; if ((s32)a < 0) r *= 3;
        if (a >= 0x80001234u) r += 1; if ((a & 0x7f) == 100) r += 7; if ((a & 0x7f) != 12) r ^= 2;
        if ((s32)(a & 0x7f) >= 64) r += 8; if ((a & 0x7f) < 31) r ^= 0x80;
    }
    return r;
}

static NOINLINE u32 select(u32 x)
{
    switch (x) {
    case 0: return 11; case 1: return 0x2222; case 2: return 37; case 3: return mix(x, 41);
    case 4: return 5; case 5: return x * 77; case 6: return ~x; case 7: return 0xdeadbeef;
    case 8: return 123456789; case 9: return x << 9;
    default: return mix(x, x);
    }
}
static NOINLINE u32 selects(u32 x) { u32 r = x; for (u32 i = 0; i < 14; i++) r = mix(r, select((x + i) % 12)); return r; }

static NOINLINE u32 lists(u32 x)
{
    u32 s = x, r = 0;
    for (int i = 0; i < 12; i++) {
        pool[i].key = (s32)(next(&s) % 9); pool[i].next = i + 1 < 12 ? &pool[i + 1] : 0;
        pool[i].tag = (uint16_t)next(&s); pool[i].flag = (uint8_t)i; pool[i].bias = (int8_t)next(&s);
    }
    for (s32 key = 0; key < 9; key++)
        for (struct node *n = pool; n; n = n->next)
            if (n->key == key) { r = mix(r, n->tag + n->flag + (u32)n->bias); break; }
    return r;
}

static NOINLINE void copy(struct wide *d, const struct wide *s) { *d = *s; }
static NOINLINE u32 memory(u32 x)
{
    u32 s = x, r = 0; struct wide local;
    for (int i = 0; i < 9; i++) blocks[0].w[i] = next(&s);
    copy(&blocks[1], &blocks[0]); local = blocks[1]; local.w[4] ^= x; copy(&blocks[2], &local);
    for (int i = 0; i < 64; i++) bytes[i] = (uint8_t)(next(&s) >> 24);
    for (int i = 0; i < 32; i++) halves[i] = (uint16_t)(bytes[i] * 259u + bytes[63 - i]);
    for (int i = 0; i < 9; i++) r = mix(r, blocks[2].w[i] + blocks[1].w[8 - i]);
    for (int i = 0; i < 32; i++) r = mix(r, halves[i] + (u32)(s32)(int8_t)bytes[i] + (u32)(s32)(int16_t)halves[31 - i]);
    for (int i = 0; i < 16; i++) { state[i] = r + i; r = mix(r, state[(i * 7) & 15]); }
    shared += r; sink = r;
    return r ^ shared ^ sink;
}

static NOINLINE u32 many(u32 a, u32 b, u32 c, u32 d, u32 e, u32 f, u32 g, u32 h, u32 i, u32 j, u64 k)
{
    return mix(a + b * 2 + c * 3, d + e * 5) + mix(f + g * 7, h) + i * j + (u32)(k >> 20);
}
static NOINLINE u32 depth(u32 n, u32 acc) { u32 buf[6]; if (n == 0) return acc; for (int i = 0; i < 6; i++) buf[i] = mix(acc, n + i); return depth(n - 1, buf[n % 6]) + buf[(n + 1) % 6]; }
static u32 twice(u32 (*f)(u32), u32 x) { return f(f(x)); }
static NOINLINE u32 calls(u32 x)
{
    u32 r = many(x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7, x + 8, x + 9, ((u64)x << 32) | ~x);
    u32 (*volatile fp)(u32) = select;
    r = mix(r, depth(9, x)); r = mix(r, fp(x & 7)); r = mix(r, twice(fp, x % 5));
    return r;
}

u32 (*const table[])(u32) __attribute__((section(".table"), used)) = { arith, shifts, wides, fields, selects, lists, memory, calls };
