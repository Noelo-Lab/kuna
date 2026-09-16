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
int main(void)
{
  __printf_chk(1,"%d\n",(unsigned long)(unsigned int)apply(mode_truncate,level_error,7));
  return 0;
}

// Function: _start @ 0x1090
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x10c0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x10f0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1130
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

// Function: frame_dummy @ 0x1170
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: apply @ 0x1180
int apply(mode m,level l,int n)
{
  if (m != mode_append) {
    if (l == level_error)
      n = -n;
    return n;
  }
  return n + 1;
}

// Function: _fini @ 0x1194
void _fini(void)
{
  return;
}

