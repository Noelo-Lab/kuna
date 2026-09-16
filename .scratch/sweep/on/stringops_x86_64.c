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
  (*dat_3fc0)(); // jump-as-call
}

// Function: puts @ 0x1030
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_3fc8)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x1040
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3fd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x1050
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: main @ 0x1060
unsigned long main(int a0,long a1)
{
  char *v1;
  
  v1 = "x";
  if (2 <= a0)
    v1 = *(char **)(a1 + 8);
  __printf_chk(1,"%d\n",(unsigned long)(unsigned int)pick(v1));
  return 0;
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

// Function: sink @ 0x1190
void sink(char *a0)
{
  puts(a0); // tail-call
}

// Function: pick @ 0x11a0
bool pick(unsigned char *a0)
{
  unsigned int v1; // eax
  
  sink();
  v1 = (unsigned int)*a0 - 0x2d;
  if (!v1)
    v1 = (unsigned int)a0[1];
  return v1 == 0;
}

// Function: _fini @ 0x11c0
void _fini(void)
{
  return;
}

