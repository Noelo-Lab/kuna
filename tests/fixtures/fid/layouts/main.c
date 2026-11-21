struct hdr;
struct bits { unsigned a : 3, b : 5; int c; };
long hdr_score(struct hdr *h, long x);
struct bits make_bits(int x);
long take_bits(struct bits b, long k);
static char buf[64];
long entry(long c)
{
    struct bits b = make_bits((int)c);
    return take_bits(b, c) + hdr_score((struct hdr *)buf, c);
}
