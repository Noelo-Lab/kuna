struct hdr { unsigned magic, len; };
int parse_header(const unsigned char *buf, unsigned len, struct hdr *out)
{
    if (len < 8)
        return -1;
    out->magic = buf[0] | buf[1] << 8 | buf[2] << 16 | (unsigned)buf[3] << 24;
    out->len = buf[4] | buf[5] << 8 | buf[6] << 16 | (unsigned)buf[7] << 24;
    if (out->magic != 0x4b464944)
        return -2;
    return out->len <= len - 8 ? 0 : -3;
}
