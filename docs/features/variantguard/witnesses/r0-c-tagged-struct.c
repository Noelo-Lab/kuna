/* (W7/W8) A C tagged-union-shaped struct: two paths, two constants at offset 0. */
struct kv { unsigned long code; unsigned long val; };

struct kv c_pick(unsigned x) {
    struct kv r;
    if (x > 10) { r.code = 0; r.val = (unsigned long)x * 3; }
    else        { r.code = 1; r.val = (unsigned long)x + 1; }
    return r;
}

unsigned long c_kindval(unsigned x) {
    struct kv r = c_pick(x);
    return r.code ? r.val + 100 : r.val;
}
