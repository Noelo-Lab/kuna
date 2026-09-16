// Function: stub @ 0x100000
void stub(void) // warn: Control flow encountered bad instruction data
{
  int v1; // ecx
  unsigned char *v2; // rsi
  
  v2 = (unsigned char *)0x101000;
  v1 = 0x20;
  do {
    *v2 = *v2 ^ 0x5a;
    v2 = &v2[1];
    v1 -= 1;
  } while (v1);
  switch(0x101000) {
    case 0x101000:
      halt_baddata(); // warn: Bad instruction - Truncating control flow here
    
  }
}

