// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: main @ 0x401020
unsigned int main(void)
{
  unsigned int v1; // ebx
  
  c.0._104_8_ = 0x4040c0;
  v1 = sel & 1;
  (**(void **)&key_listener[(unsigned long)v1 * 8])(c.0,0,1,2,3,4);
  (**(void **)&two_listener[(unsigned long)v1 * 8])(c.0,0,1,2);
  return c.0._112_4_;
}

// Function: _start @ 0x401080
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010b0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x4010c0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010f0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401130
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401160
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: lookup @ 0x401170
unsigned int lookup(long a0,int a1)
{
  return a1 * 0x1f ^ (unsigned int)(a0 >> 3);
}

// Function: on_key @ 0x401180
void on_key(long a0,unsigned long a1,unsigned long a2,unsigned long a3,int a4,int a5)
{
  unsigned int v1; // eax
  long v2; // rdx
  
  if ((a5 == 1) && (*(long *)(a0 + 0x68))) {
    v1 = lookup(*(long *)(a0 + 0x68),a4 + 8,a0);
    *(unsigned int *)(v2 + 0x70) = v1;
  }
}

// Function: on_two @ 0x4011a0
void on_two(long a0,unsigned long a1,unsigned long a2,int a3) // return-dupe
{
  unsigned int v1; // eax
  long v2; // rdx
  
  if (!*(long *)(a0 + 0x68))
    return;
  v1 = lookup(*(long *)(a0 + 0x68),a3 + 3,a0);
  *(unsigned int *)(v2 + 0x70) = v1;
}

// Function: _fini @ 0x4011b8
void _fini(void)
{
  return;
}

