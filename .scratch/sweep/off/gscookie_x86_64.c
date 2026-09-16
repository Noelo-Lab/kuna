// Function: fmt @ 0x401000
long fmt(unsigned long a0,long a1,long a2,long a3,long a4,long a5,long a6,long a7)
{
  return a6 + a7 + a1 + a2 + a3 + a4 + a5;
}

// Function: report @ 0x40101a
void report(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = dat_403005 ^ (unsigned long)&Stackffffffffffffffc8;
  fmt("%s%s",1,2,3,4,5,a0,a1);
  if ((v1 ^ (unsigned long)&Stackffffffffffffffc8) == dat_403005)
    return;
  (*(void *)invalidInstructionException())(); // jump-as-call
}

