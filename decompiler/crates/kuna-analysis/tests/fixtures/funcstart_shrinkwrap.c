/* Early-exit functions whose prologue the compiler sinks past the first return. */
struct buf { unsigned long used, size; char *data; };
extern void grow(struct buf *, unsigned long);
extern void fill(void *, int, unsigned long);
void buf_dup_char(struct buf *b, int c, unsigned long n)
{
  if (!n)
    return;
  grow(b, n);
  fill(b->data + b->used, c, n);
  b->used += n;
}
int buf_last(struct buf *b) { return b->used ? b->data[b->used - 1] : -1; }
