// Function: sub_10000 @ 0x10000
void sub_10000(void)
{
  char *v1;
  unsigned char *v2;
  unsigned char v3; // al
  char v4; // cl
  int v5; // ecx
  char *v6;
  unsigned char *v7;
  
  do {
    v1 = &v6[1];
    v4 = (char)v5;
    v3 = (*v6 + '\a') - v4;
    v3 = (v3 >> 1 | v3 * '\x80') + 0x75;
    v3 = ((v3 >> 5 | v3 * '\b') + 0x59) - v4 ^ 0x92;
    v2 = &v7[1];
    *v7 = (v3 << 3 | v3 >> 5) + v4 ^ 0x92;
    v5 -= 1;
    v6 = v1;
    v7 = v2;
  } while (v5);
}

