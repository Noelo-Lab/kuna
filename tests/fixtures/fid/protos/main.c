struct hdr { unsigned magic, len; };
int parse_header(const unsigned char *buf, unsigned len, struct hdr *out);
int check(const unsigned char *p, unsigned n)
{
    struct hdr h;
    if (parse_header(p, n, &h))
        return 0;
    return h.len != 0;
}
