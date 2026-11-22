extern unsigned mix_fold(unsigned x, unsigned k);
extern unsigned sum_bytes(const unsigned char *p, unsigned long n);
extern unsigned get_seven(void);

unsigned wrap_mix(unsigned x) { return mix_fold(x, 11) + 1; }
unsigned wrap_sum(const unsigned char *p) { return sum_bytes(p, 11) + 1; }
unsigned wrap_seven(unsigned x) { return get_seven() + x; }
