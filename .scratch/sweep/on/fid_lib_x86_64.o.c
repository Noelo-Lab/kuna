// Function: kuna_crc32 @ 0x400000
unsigned int kuna_crc32(unsigned char *a0,unsigned int a1)
{
  unsigned char v1;
  unsigned int v2; // eax
  int v3; // edx
  unsigned char *v4; // rsi
  
  if (a1) {
    v2 = 0xffffffff;
    v4 = &a0[a1];
    do {
      v1 = *a0;
      a0 = &a0[1];
      v2 ^= v1;
      v3 = 8;
      do {
        v2 = -(v2 & 1) & 0xedb88320 ^ v2 >> 1;
        v3 -= 1;
      } while (v3);
    } while (a0 != v4);
    return ~v2;
  }
  return 0;
}

// Function: kuna_strlen @ 0x400050
long kuna_strlen(char *a0)
{
  char *v1;
  
  if (*a0) {
    v1 = a0;
    do {
      v1 = &v1[1];
    } while (*v1);
    return (long)v1 - (long)a0;
  }
  return 0;
}

// Function: kuna_memset @ 0x400080
char * kuna_memset(char *a0,char a1,long a2)
{
  char *v1;
  char *v2; // rax
  char *v3; // rcx
  
  v2 = a0;
  v1 = &a0[a2];
  if (a2) {
    do {
      v3 = &a0[1];
      *a0 = a1;
      a0 = v3;
    } while (v3 != v1);
  }
  return v2;
}

