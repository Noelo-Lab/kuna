// Function: _init @ 0x4004e0
void _init(void)
{
  (*(void *)(dat_41102c + 0x6f4))();
  (*(void *)(dat_41102c + 0x780))();
}

// Function: __start @ 0x400560
void __start(void)
{
  char v1 [16];
  unsigned int v2; // stack + 0x0
  unsigned int v3; // stack - 0x10
  char *v4; // stack - 0x8
  
  v4 = v1;
  v3 = 0;
  _MIPS_STUBS_(dat_411028,v2,&Stack00000004,0);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x4005c0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4005f8
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x400644
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.1)
    return;
  deregister_tm_clones();
  completed.1 = 1;
}

// Function: frame_dummy @ 0x4006f4
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x400700
unsigned int main(unsigned int a0,unsigned int a1)
{
  unsigned int v1; // stack + 0x0
  unsigned int v2; // stack + 0x4
  
  v1 = a0;
  v2 = a1;
  puts("hello");
  printf(0x400888,v1);
  return 0;
}

// Function: __do_global_ctors_aux @ 0x400780
void __do_global_ctors_aux(void)
{
  unsigned int *v1; // s0
  void *v2;
  
  if (dat_410ff0 != (void *)0xffffffff) {
    v1 = (unsigned int *)0x410ff0;
    v2 = dat_410ff0;
    do {
      v1 = &v1[-1];
      (*v2)();
      v2 = (void *)*v1;
    } while (v2 != (void *)0xffffffff);
    return;
  }
}

// Function: _MIPS_STUBS_ @ 0x4007e0
void _MIPS_STUBS_(void)
{
  int v1; // gp
  
  (**(void **)(v1 + -0x7ff0))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: printf @ 0x4007f0
int printf(char *a0,...)
{
  int v1; // gp
  
  (**(void **)(v1 + -0x7ff0))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: puts @ 0x400800
int puts(char *a0)
{
  int v1; // gp
  
  (**(void **)(v1 + -0x7ff0))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _fini @ 0x400820
void _fini(void)
{
  (*(void *)(dat_41102c + 0x644))();
}

