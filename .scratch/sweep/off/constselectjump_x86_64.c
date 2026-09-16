// Function: csjmp @ 0x100000
long csjmp(long a0,long a1)
{
  unsigned long v1; // r9
  
  v1 = 0x100020;
  if (!a0)
    v1 = 0x100030;
  switch(v1) {
    case 0x100020:
      return a1 + 1;
    case 0x100030:
      return a1 + -1;
    
  }
}

