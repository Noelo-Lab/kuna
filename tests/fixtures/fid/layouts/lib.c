enum __attribute__((packed)) kind { K_A, K_B, K_C };
struct hdr { enum kind k; unsigned char flag; int len; long id; };
struct bits { unsigned a : 3, b : 5; int c; };

long hdr_score(struct hdr *h, long x) { return (h->len * x + h->id) ^ h->flag; }
struct bits make_bits(int x) { struct bits r = { x & 7, (x >> 3) & 31, x * 3 }; return r; }
long take_bits(struct bits b, long k) { return b.a * k + b.b + b.c; }
