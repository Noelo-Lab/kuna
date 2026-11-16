/* clang --target=powerpc64le-linux-gnu -O1 -c publish_toc_ppc64le.c -o p.o
 * && ld.lld -e publish p.o -o publish_toc_ppc64le (clang and LLD 23.1.5, per
 * the image's .comment). LLD keeps the global entry's
 * `addis r2,r12,2; addi r2,r2,-32736`; GNU ld rewrites a non-PIC one to
 * `lis r2,..`, which needs no r12. The local entry is publish+8. */
unsigned char *escaped;
void publish(unsigned char *p) { escaped = p; }
