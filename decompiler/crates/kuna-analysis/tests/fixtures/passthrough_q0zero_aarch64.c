
struct sizing { long a, b, c, written, e; };
__attribute__((noinline)) int encode(struct sizing *s, const int *fields, const long *src)
{
    if (!fields || !src)
        return 0;
    s->written = fields[0] + src[1];
    return 1;
}
int encoded_size(unsigned long *size, const int *fields, const long *src)
{
    struct sizing s = {0};
    if (!encode(&s, fields, src))
        return 0;
    *size = s.written;
    return 1;
}
