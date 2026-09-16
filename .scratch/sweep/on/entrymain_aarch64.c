// Function: _DT_INIT @ 0x580
void _DT_INIT(void)
{
  sub_634();
}

// Function: __libc_start_main @ 0x5c0
void __libc_start_main(void)
{
  (*dat_10fb0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x5d0
void __cxa_finalize(void)
{
  (*dat_10fb8)(); // jump-as-call
}

// Function: __gmon_start__ @ 0x5e0
void __gmon_start__(void)
{
  (*dat_10fc0)(); // jump-as-call
}

// Function: abort @ 0x5f0
void abort(void)
{
  (*dat_10fc8)(); // jump-as-call
}

// Function: sub_600 @ 0x600
void sub_600(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  __libc_start_main(0x714,v1,&Stack0000000000000008,0,0,a0);
  abort(); // no-return
}

// Function: sub_634 @ 0x634
void sub_634(void)
{
  if (dat_10fe8) {
    __gmon_start__(); // tail-call
    return;
  }
}

// Function: sub_650 @ 0x650
void sub_650(void)
{
  return;
}

// Function: sub_680 @ 0x680
void sub_680(void)
{
  return;
}

// Function: _FINI_0 @ 0x6c0
void _FINI_0(void) // return-dupe
{
  if (dat_11010)
    return;
  if (dat_10fe0)
    __cxa_finalize(dat_11008);
  sub_650();
  dat_11010 = 1;
}

// Function: _INIT_0 @ 0x710
void _INIT_0(void)
{
  sub_680(); // tail-call
}

// Function: main @ 0x714
int main(int argc,char **argv,char **envp)
{
  return argc;
}

// Function: _DT_FINI @ 0x72c
void _DT_FINI(void)
{
  return;
}

