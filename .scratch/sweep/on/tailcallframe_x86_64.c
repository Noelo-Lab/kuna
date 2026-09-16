// Function: _DT_INIT @ 0x1000
void _DT_INIT(void) // return-dupe
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

// Function: strlen @ 0x1030
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3fd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x1040
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: main @ 0x1050
int main(int argc,char **argv)
{
  (*(void *)*dat_4010)(0x4040,3);
  return dat_4048 + dat_4044 + dat_404c;
}

// Function: sub_1080 @ 0x1080
void sub_1080(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_10b0 @ 0x10b0
void sub_10b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x1120
void _FINI_0(void)
{
  if (!dat_4020) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    sub_10b0();
    dat_4020 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x1160
void _INIT_0(void)
{
  return;
}

// Function: sub_11b0 @ 0x11b0
void sub_11b0(int *a0,int a1)
{
  *a0 = a1;
  if (1 <= a1) {
    a0[1] = (int)strlen((char *)&a0[4]) + 1;
    sub_1170(a0); // warn: tailcallframe: recovered tail call -> introduced call to 0x00001170
    return;
  }
  a0[1] = 0;
}

// Function: _DT_FINI @ 0x11dc
void _DT_FINI(void)
{
  return;
}

