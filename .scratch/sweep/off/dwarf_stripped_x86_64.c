// Function: _DT_INIT @ 0x401000
void _DT_INIT(void) // return-dupe
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

// Function: printf @ 0x401040
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: sub_401050 @ 0x401050
void sub_401050(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_401080 @ 0x401080
void sub_401080(void)
{
  return;
}

// Function: sub_401090 @ 0x401090
void sub_401090(void)
{
  return;
}

// Function: _FINI_0 @ 0x401100
void _FINI_0(void)
{
  if (!dat_404030) {
    sub_401090();
    dat_404030 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x401130
void _INIT_0(void)
{
  return;
}

// Function: add_values @ 0x401136
long add_values(long a,long b)
{
  return b + a;
}

// Function: compute @ 0x401153
int compute(long n,char *label)
{
  printf("%s=%ld\n",label,add_values(n,5));
  return 0;
}

// Function: main @ 0x401198
int main(int argc,char **argv)
{
  char *label;
  
  label = *argv;
  return compute((long)argc,label);
}

// Function: _DT_FINI @ 0x4011c4
void _DT_FINI(void)
{
  return;
}

