struct job { int stopstatus; unsigned nprocs : 16, state : 8, used : 1, changed : 1; };
struct pair { int a : 8; int b : 8; int c : 16; };
struct node { void *p; unsigned type : 8, constraint : 10, dup : 1; };
extern unsigned ctx(int) __attribute__((pure));
extern int peek(struct node *t);
extern int create(struct node *t);

int job_state(struct job *jp) { if (jp->state == 1) return 5; if (jp->state == 2) return 7; return jp->nprocs; }
int pair_a(struct pair s) { return s.a * 3; }
int node_accept(struct node *n, int i) {
  if (n->constraint) {
    unsigned c = ctx(i);
    if (((n->constraint & 4) && !(c & 1)) || ((n->constraint & 8) && (c & 1)) || ((n->constraint & 0x20) && !(c & 2)))
      return 0;
  }
  return 1;
}
int node_store(void *m) {
  struct node t;
  peek(&t);
  if (t.type == 2) return 1;
  t.type = 6;
  t.p = m;
  return create(&t);
}
int node_save(int k) {
  struct node t;
  peek(&t);
  unsigned a = t.type;
  if (k) create(&t);
  return a + t.type;
}
