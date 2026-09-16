// Function: _init @ 0x1000
void _init(void) // return-dupe
{
  if (!dat_3fe8)
    return;
  (*dat_3fe8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_3fb8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1060
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: strlen @ 0x1070
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3fc0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x1080
void __stack_chk_fail(void)
{
  (*dat_3fc8)(); // jump-as-call
}

// Function: __printf_chk @ 0x1090
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3fd0)(); // jump-as-call
  return v1;
}

// Function: _start @ 0x10a0
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x10d0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x1100
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1140
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x1180
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: take_struct @ 0x1189
int take_struct(P8 p,int k)
{
  return p.b + p.a + k;
}

// Function: ret_struct @ 0x119a
P8 ret_struct(unsigned int x)
{
  P8 v1; // rax
  
  v1.a = x + 1;
  v1.b = x * 7;
  return v1;
}

// Function: ret_big @ 0x11b2
Big24 * ret_big(Big24 *rethidden,long q)
{
  rethidden->x = q;
  rethidden->y = q * 2;
  rethidden->z = q * 3;
  return rethidden;
}

// Function: take_union @ 0x11cc
int take_union(U4 u)
{
  return (int)u._1_1_ + u.i;
}

// Function: union_second @ 0x11d9
float union_second(U4 *u)
{
  return u->f;
}

// Function: union_third @ 0x11e2
char union_third(U4 *u)
{
  return u->c[2];
}

// Function: take_nest @ 0x11eb
long take_nest(Nest *n)
{
  return (long)((int)n->tag + (n->inner).a + (n->inner).b) + n->tail;
}

// Function: take_bits @ 0x11ff
unsigned int take_bits(Bits *b)
{
  return b->lo + b->mid + b->hi;
}

// Function: walk_list @ 0x121b
int walk_list(Node *n)
{
  int v1; // eax
  
  if (n) {
    v1 = 0;
    do {
      v1 += n->val;
      n = (Node *)n->next;
    } while (n);
    return v1;
  }
  return 0;
}

// Function: read_same_small @ 0x123b
int read_same_small(Same *s)
{
  return s->only;
}

// Function: main @ 0x1242
int main(int argc,char **argv)
{
  Big24 b; // stack - 0x98
  Bits bb; // stack - 0xc0
  char big [16]; // stack - 0x58
  Node n1; // stack - 0xa8
  Node n2; // stack - 0xb8
  Nest nn; // stack - 0x78
  P8 p; // rdi
  Same sm; // stack - 0xbc
  U4 u; // stack - 0x5c
  char v1 [16];
  P8 v10; // rax
  float v11; // xmm0_da
  char v2; // al
  unsigned int v3; // eax
  unsigned int v4; // eax
  unsigned int v5; // eax
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned long v8; // rax
  long v9; // rax
  
  ret_big(&b,(long)argc);
  n2.val = 5;
  n2.next = NULL;
  n1.val = 3;
  n1.next = &n2;
  nn.tail = b.x;
  bb.hi = argc;
  bb.mid = argc;
  bb.lo = argc;
  v1._8_8_ = 0;
  v1._0_8_ = (unsigned long)(unsigned int)argc & 0xff;
  big._0_8_ = SUB168(v1._0_16_ * ZEXT816(0x101010101010101),0);
  big._8_8_ = SUB168(v1._0_16_ * ZEXT816(0x101010101010101),8) + ((unsigned long)(unsigned int)argc & 0xff) * 0x101010101010101;
  sm.only = argc;
  nn.tag = (char)argc;
  nn.inner.a = argc;
  nn.inner.b = argc * 2;
  u.i = argc;
  v8 = strlen(*argv);
  v11 = union_second(&u);
  v2 = union_third(&u);
  v3 = read_same_big((Same_16 *)big);
  v4 = read_same_small(&sm);
  v5 = walk_list(&n1);
  v6 = take_bits(&bb);
  v9 = take_nest(&nn);
  v7 = take_union(u);
  v10 = ret_struct(argc);
  p.b = argc * 2;
  p.a = argc;
  __printf_chk(1,"%d %ld %d %ld %u %d %d %d %d\n",(unsigned long)(unsigned int)take_struct(p,1),(long)v10 >> 0x20,(unsigned long)v7,v9,(unsigned long)v6,(unsigned long)v5,(unsigned long)v4,(unsigned long)v3,(unsigned long)(unsigned int)((int)v2 + (int)v8 + (int)v11));
  return 0;
}

// Function: read_same_big @ 0x1429
int read_same_big(Same_16 *s)
{
  return s->second + (int)s->first + s->third;
}

// Function: _fini @ 0x1438
void _fini(void)
{
  return;
}

