// Function: _DT_INIT @ 0x401000
void _DT_INIT(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: main @ 0x401020
int main(int argc,char **argv,char **envp)
{
  char v1;
  
  v1 = **argv;
  return sub_401130((long)(v1 + argc));
}

// Function: sub_401040 @ 0x401040
void sub_401040(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_401070 @ 0x401070
void sub_401070(void)
{
  return;
}

// Function: sub_401080 @ 0x401080
void sub_401080(void)
{
  return;
}

// Function: _FINI_0 @ 0x4010f0
void _FINI_0(void)
{
  if (!dat_404028) {
    sub_401080();
    dat_404028 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x401120
void _INIT_0(void)
{
  return;
}

// Function: sub_401130 @ 0x401130
long sub_401130(long a0)
{
  return a0 * 7 + 9;
}

// Function: _DT_FINI @ 0x401178
void _DT_FINI(void)
{
  return;
}

