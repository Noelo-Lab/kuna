// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff0)
    return;
  (*dat_403ff0)();
}

// Function: sub_401020 @ 0x401020
void sub_401020(void)
{
  (*dat_404010)(); // jump-as-call
}

// Function: strlen @ 0x401030
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x401040
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_404020)(); // jump-as-call
  return v1;
}

// Function: _start @ 0x401050
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403fe8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x401080
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401090
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401100
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401130
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: count_authors @ 0x401136
int count_authors(char **authors)
{
  int n; // stack - 0x14
  char **p; // stack - 0x10
  char *v1;
  
  n = 0;
  for (p = authors; *p; p = &p[1]) {
    v1 = *p;
    n += (int)strlen(v1);
  }
  return n;
}

// Function: sum_counts @ 0x40117b
unsigned long sum_counts(unsigned long *pn,int n)
{
  int i; // stack - 0x14
  unsigned long total; // stack - 0x10
  
  total = 0;
  for (i = 0; i < n; i = i + 1) {
    total += pn[i];
  }
  return total;
}

// Function: use_state @ 0x4011cc
int use_state(state_t *ps,int n)
{
  state_t *cur; // stack - 0x10
  int n_1; // stack - 0x24
  int total; // stack - 0x14
  int v1;
  
  total = 0;
  n_1 = n;
  cur = ps;
  while (v1 = n_1 + -1, 0 < n_1) {
    total += cur->flags + cur->count;
    cur = &cur[1];
    n_1 = v1;
  }
  return total;
}

// Function: walk_codes @ 0x401211
int walk_codes(code_4 *base,int n)
{
  int n_1; // stack - 0x24
  code_4 *next; // stack - 0x10
  int total; // stack - 0x14
  int v1;
  
  total = 0;
  n_1 = n;
  next = base;
  while (v1 = n_1 + -1, 0 < n_1) {
    total += (unsigned int)next->val;
    next = &next[1];
    n_1 = v1;
  }
  return total;
}

// Function: take_argvp @ 0x401252
int take_argvp(int *argcp,char ***argvp)
{
  int n; // stack - 0xc
  
  for (n = 0; (*argvp)[n]; n = n + 1) {
  }
  *argcp = n;
  return n;
}

// Function: list_len @ 0x4012a8
int list_len(node *head)
{
  node *cur; // stack - 0x10
  int n; // stack - 0x14
  
  n = 0;
  for (cur = head; cur; cur = (node *)cur->next) {
    n += cur->value;
  }
  return n;
}

// Function: ping @ 0x4012e2
int ping(ping_s *a)
{
  int v1;
  
  if (a->other) // branch-flip
    v1 = *(int *)a->other;
  else {
    v1 = 0;
  }
  return v1 + a->v;
}

// Function: pong @ 0x40131a
int pong(pong_s *b)
{
  int v1;
  
  if (b->other) // branch-flip
    v1 = b->other->v;
  else {
    v1 = 0;
  }
  return v1 + b->v;
}

// Function: main @ 0x401352
int main(int argc,char **argv)
{
  code_4 codes [2]; // stack - 0x80
  unsigned long counts [3]; // stack - 0x58
  node head; // stack - 0x78
  int n; // stack - 0xb4
  ping_s pi; // stack - 0xa8
  pong_s po; // stack - 0x98
  node tail; // stack - 0x68
  int v1; // eax
  int v2; // eax
  int v3; // eax
  unsigned int v4; // eax
  unsigned int v5; // eax
  unsigned int v6; // eax
  unsigned long v7; // rax
  char **vec; // stack - 0xb0
  
  counts[0] = 1;
  counts[1] = 2;
  counts[2] = 3;
  tail.value = 2;
  tail.next = NULL;
  head.value = 1;
  head.next = &tail;
  codes[0].op = '\x01';
  codes[0].val = 2;
  codes[1].op = '\x03';
  codes[1].val = 4;
  po.v = 0x14;
  pi.v = 10;
  pi.other = &po;
  po.other = &pi;
  global_state._0_4_ = argc;
  n = argc;
  vec = argv;
  v1 = ping(&pi);
  v2 = pong(&po);
  v3 = walk_codes(codes,2);
  v4 = list_len(&head);
  v5 = take_argvp(&n,&vec);
  v6 = use_state((state_t *)&global_state,1);
  v7 = sum_counts(counts,3);
  printf("%d %lu %d %d %d %d\n",(unsigned long)(unsigned int)count_authors((char **)default_authors),v7,(unsigned long)v6,(unsigned long)v5,(unsigned long)v4,(unsigned long)(unsigned int)(v1 + v2 + v3));
  return 0;
}

// Function: _fini @ 0x4014fc
void _fini(void)
{
  return;
}

