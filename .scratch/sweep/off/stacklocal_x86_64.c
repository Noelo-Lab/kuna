// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: sub_401020 @ 0x401020
void sub_401020(void)
{
  (*dat_404010)(); // jump-as-call
}

// Function: __isoc99_scanf @ 0x401040
int __isoc99_scanf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: _start @ 0x401050
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
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

// Function: compute_sum @ 0x401136
int compute_sum(int n)
{
  int accumulator; // stack - 0x10
  int counter; // stack - 0xc
  
  accumulator = 0;
  for (counter = 0; counter < n; counter = counter + 1) {
    __isoc99_scanf("%d",&accumulator);
    accumulator = counter + accumulator;
  }
  return accumulator;
}

// Function: main @ 0x40118c
int main(int argc,char **argv)
{
  return compute_sum(argc);
}

// Function: _fini @ 0x4011ac
void _fini(void)
{
  return;
}

