struct A2 { unsigned lo:3, hi:5, mid:12, x:4, f:1, y:7; };
struct A3 { unsigned mid:12, hi:5, rest:15; };
struct S3 { int smid:12; unsigned hi:5, rest:15; };

unsigned long long pair_mid(struct A3 *p, unsigned v, unsigned char *q) { unsigned w = p->mid; p->hi = v; *q = 7; return w; }
unsigned long long pair_rest(struct A3 *p, unsigned v, unsigned char *q) { unsigned w = p->rest; p->hi = v; *q = 7; return w; }
unsigned long long pair_inner(struct A2 *p, unsigned v, unsigned char *q) { unsigned w = p->mid; p->hi = v; *q = 7; return w; }
long long pair_signed(struct S3 *p, unsigned v, unsigned char *q) { int w = p->smid; p->hi = v; *q = 7; return w; }
