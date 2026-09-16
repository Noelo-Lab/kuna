// Function: __libc_start_main @ 0x10150
void __libc_start_main(void)
{
  (*dat_10200)(); // jump-as-call
}

// Function: sub_10160 @ 0x10160
void sub_10160(void)
{
  __libc_start_main(main);
}

// Function: main @ 0x10180
int main(int argc,char **argv,char **envp)
{
  sub_10190();
  return 0;
}

// Function: sub_10190 @ 0x10190
unsigned int sub_10190(void)
{
  return 0x2a;
}

