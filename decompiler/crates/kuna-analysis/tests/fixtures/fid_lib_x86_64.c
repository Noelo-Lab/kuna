/* FID fixture library — distinctive, self-contained functions for the FID
 * (Function-ID) fingerprinting tests. See docs/fid-design.md §7.2.
 *
 * Each function is >= 4 code units and calls NO external libc symbol, so the
 * compiled body is fully self-contained: the FID full hash is stable across
 * link and the function can be re-identified in a stripped binary purely by
 * its instruction-stream fingerprint.
 *
 * `kuna_crc32` is the star: a distinctive loop with the magic 0xEDB88320
 * immediate (the reversed CRC-32 polynomial) that exercises the specific-hash
 * scalar path and has near-zero collision risk.
 */

/* CRC-32 (the IEEE 802.3 reversed polynomial 0xEDB88320), bit-at-a-time. */
unsigned kuna_crc32(const unsigned char *p, unsigned n) {
    unsigned c = ~0u;
    while (n--) {
        c ^= *p++;
        for (int k = 0; k < 8; k++)
            c = (c >> 1) ^ (0xEDB88320u & -(c & 1));
    }
    return ~c;
}

/* strlen, hand-rolled (no libc call). */
unsigned long kuna_strlen(const char *s) {
    const char *p = s;
    while (*p)
        p++;
    return p - s;
}

/* memset, hand-rolled (no libc call). */
void *kuna_memset(void *d, int v, unsigned long n) {
    unsigned char *p = d;
    while (n--)
        *p++ = (unsigned char)v;
    return d;
}
