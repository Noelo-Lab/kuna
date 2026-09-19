/* (kuna) `protoorder` fixture -- coreutils tail's `tail_bytes` shape.  `dump`
 * compares its byte count against 0x1000, which is also the address of `.init`,
 * so its own recovery types the parameter `void *`.  Voted onto `n_bytes` in
 * `tail_bytes`, that pointer reaches `end_pos` and then the `st_size` read, the
 * `struct stat` splits into `char [56]` plus a separate `st_blksize` local, and
 * the output reads that local although nothing writes it (fstat does, through
 * the struct).  The value is integer-used (`-n_bytes`, lseek's `off_t`), so the
 * vote is refused and `stat` stays whole.
 *
 * Built with:  gcc -O0 -o protoorder_stackstruct_x86_64 protoorder_stackstruct_x86_64.c && strip protoorder_stackstruct_x86_64
 */
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#define NI __attribute__((noinline))
#define BUFSZ 0x1000
NI unsigned long dump(int fd, unsigned long n_bytes) {
  unsigned long n_written = 0, n_remaining = n_bytes;
  while (1) {
    char buffer[BUFSZ];
    unsigned long n = n_remaining < BUFSZ ? n_remaining : BUFSZ;
    long r = read(fd, buffer, n);
    if (r <= 0) break;
    write(1, buffer, r);
    n_written += r;
    if (n_bytes != (unsigned long)-1) { n_remaining -= r; if (n_remaining == 0 || n_bytes == (unsigned long)-2) break; }
  }
  return n_written;
}
NI int usable(struct stat const *sb) { return S_ISREG(sb->st_mode); }
NI int tail_bytes(int fd, unsigned long n_bytes, unsigned long *read_pos, int from_start) {
  struct stat stats;
  if (fstat(fd, &stats)) return 1;
  if (from_start) {
    if (n_bytes <= 0x7fffffffffffffff && lseek(fd, n_bytes, SEEK_CUR) != -1) *read_pos += n_bytes;
    else return 0;
    n_bytes = (unsigned long)-1;
  } else {
    long end_pos = -1, current_pos = -1;
    int copy = 0;
    if (n_bytes <= 0x7fffffffffffffff) {
      if (usable(&stats)) {
        end_pos = stats.st_size;
        long smallish = stats.st_blksize <= 0 || 0x2000000000000000 < stats.st_blksize ? 0x200 : stats.st_blksize;
        copy = smallish < end_pos;
      } else {
        current_pos = lseek(fd, -n_bytes, SEEK_END);
        copy = current_pos != -1;
        if (copy) end_pos = current_pos + n_bytes;
      }
    }
    if (!copy) return 2;
    if (current_pos == -1) current_pos = lseek(fd, 0, SEEK_CUR);
    if (current_pos < end_pos) {
      long remaining = end_pos - current_pos;
      if (n_bytes < remaining) { current_pos = end_pos - n_bytes; lseek(fd, current_pos, SEEK_SET); }
    }
    *read_pos = current_pos;
  }
  *read_pos += dump(fd, n_bytes);
  return 0;
}
int main(int argc, char **argv) { unsigned long p = 0; printf("%d\n", tail_bytes(argc, 3, &p, argc > 2)); return 0; }
