__attribute__((noinline,used)) unsigned audit_mix(unsigned x) { return (x ^ 0x7351u) * 17u + (x >> 3); }
__attribute__((noinline,used)) unsigned audit_fold(unsigned x) { return audit_mix(x) ^ 0x31415926u; }
