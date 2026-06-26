/* Caller for the FID fixture library — references each kuna_* function so the
 * linker retains them in the (stripped) `prog` binary PR4 builds. Not used by
 * PR3 (which fingerprints lib.o directly), but lives here so the fixture is a
 * complete, regenerable set. See docs/fid-design.md §7.2.
 */

unsigned kuna_crc32(const unsigned char *p, unsigned n);
unsigned long kuna_strlen(const char *s);
void *kuna_memset(void *d, int v, unsigned long n);

int main(void) {
    char buf[32];
    kuna_memset(buf, 'A', sizeof buf);
    buf[sizeof buf - 1] = 0;
    unsigned long len = kuna_strlen(buf);
    unsigned crc = kuna_crc32((const unsigned char *)buf, (unsigned)len);
    return (int)(crc ^ len);
}
