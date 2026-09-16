// Function: _init @ 0x4004cc
void _init(void)
{
  (*(void *)(dat_41104c + 0x6e4))();
  (*(void *)(dat_41104c + 0x770))();
}

// Function: __start @ 0x400550
void __start(void)
{
  char v1 [16];
  unsigned int v2; // stack + 0x0
  unsigned int v3; // stack - 0x10
  char *v4; // stack - 0x8
  
  v4 = v1;
  v3 = 0;
  _MIPS_STUBS_(dat_411048,v2,&Stack00000004,0);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x4005b0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4005e8
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x400634
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.1)
    return;
  deregister_tm_clones();
  completed.1 = 1;
}

// Function: frame_dummy @ 0x4006e4
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: bump @ 0x4006f0
void bump(int a0)
{
  counter = a0 + counter;
}

// Function: main @ 0x400704
int main(void)
{
  int v1; // s0
  
  v1 = counter + 3;
  counter = v1;
  __printf_chk(1,"%s %d\n",banner,v1,0x419030);
  return v1;
}

// Function: __do_global_ctors_aux @ 0x400770
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

// Function: _MIPS_STUBS_ @ 0x4007d0
void _MIPS_STUBS_(void)
{
  int v1; // gp
  
  (**(void **)(v1 + -0x7ff0))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: __printf_chk @ 0x4007e0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // gp
  
  (**(void **)(v1 + -0x7ff0))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _fini @ 0x400800
void _fini(void)
{
  (*(void *)(dat_41104c + 0x634))();
}

