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
  (*dat_3fd0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1030
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: main @ 0x1040
int main(int argc,char **argv)
{
  long v1; // rdx
  
  g_name = argv[(long)argc + -1];
  return (bump(argc) + (int)g_total + 1) - (unsigned int)(v1 == 0);
}

// Function: _start @ 0x1070
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x10a0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x10d0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1110
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

// Function: frame_dummy @ 0x1150
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: bump @ 0x1160
int bump(int n)
{
  g_counter += n;
  g_total += n;
  return g_counter;
}

// Function: _fini @ 0x117c
void _fini(void)
{
  return;
}

