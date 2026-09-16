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
  (*dat_3fc8)(); // jump-as-call
}

// Function: __printf_chk @ 0x1030
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3fd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x1040
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: main @ 0x1050
unsigned long main(int a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x18
  char *v2; // rsi
  char v3 [16];
  unsigned long v4; // stack - 0x10
  
  v3 = make((long)a0,(long)*(char *)*a1);
  v4 = SUB168(v3,8);
  v1 = SUB168(v3,0);
  v2 = "%ld %ld\n";
  v3 = passthru(&v1);
  __printf_chk(1,v2,SUB168(v3,0),SUB168(v3,8));
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

// Function: make @ 0x1190
undefined16 make(long a0,long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1 + 7;
  v1._0_8_ = a0 * 3;
  return v1._0_16_;
}

// Function: passthru @ 0x11a0
undefined16 passthru(char (*a0)[16])
{
  return *a0;
}

// Function: _fini @ 0x11a8
void _fini(void)
{
  return;
}

