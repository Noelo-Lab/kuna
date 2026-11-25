typedef unsigned long u64;
struct Padded {
    u64 : 64;
    u64 a;
    u64 b;
};
_Static_assert(__builtin_offsetof(struct Padded,a) == 8, "a offset");
_Static_assert(__builtin_offsetof(struct Padded,b) == 16, "b offset");
#define PROBE(name,offset) \
__attribute__((noinline,used)) u64 name(const struct Padded *p) { \
    u64 result; \
    __builtin_memcpy(&result, ((const unsigned char *)p) + (offset), sizeof result); \
    return result; \
}
PROBE(before_a0,0)
PROBE(before_a4,4)
PROBE(at_a8,8)
PROBE(at_b16,16)
