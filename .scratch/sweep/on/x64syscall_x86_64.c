// Function: kernel_call @ 0x100000
unsigned long kernel_call(unsigned long a0,unsigned long a1,unsigned long a2)
{
  return syscall(a0,a1,a2,a1);
}

// Function: plain_add @ 0x10002b
long plain_add(long a0,long a1)
{
  return a0 + a1;
}

