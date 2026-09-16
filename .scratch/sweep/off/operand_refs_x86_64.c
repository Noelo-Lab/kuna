// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: _start @ 0x401020
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x401050
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401060
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x401090
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x4010d0
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401100
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: mystery @ 0x401106
int mystery(char *a0)
{
  return (int)a0[1] + (int)*a0;
}

// Function: main @ 0x40112e
void main(void)
{
  mystery("hi");
}

// Function: _fini @ 0x401160
void _fini(void)
{
  return;
}

