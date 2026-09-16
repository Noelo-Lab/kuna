// Function: transform @ 0x401330
unsigned int transform(int a0,int a1,int a2,int a3) // return-dupe
{
  int v1;
  int v10; // edx
  unsigned int v11 [4]; // stack - 0x10
  int v12; // ebp
  unsigned int *v13; // edi
  int v14; // stack - 0x1c
  int v15; // stack - 0x18
  unsigned char v2;
  unsigned char v3;
  unsigned int v4;
  int v5;
  unsigned int *v6;
  unsigned int *v7; // eax
  int v8;
  unsigned int *v9;
  
  v11[0] = dat_4060bc;
  v11[1] = dat_4060c0;
  v11[3] = dat_4060c8;
  v5 = 0;
  v11[2] = dat_4060c4;
  v10 = 4;
  v14 = 0;
  if (1 <= a1) {
    v8 = 4;
    v15 = 4;
    do {
      v12 = 0;
      if (4 <= v8) {
        v1 = v10 * 4;
        v10 += v8;
        v2 = *(unsigned char *)(v5 + a0);
        v9 = (unsigned int *)(v1 + 0x4060bc);
        do {
          v3 = *(unsigned char *)(a0 + v12);
          v13 = &v9[4];
          v5 = 4;
          v6 = v11;
          do {
            v7 = &v6[1];
            v4 = *v9;
            v9 = &v9[1];
            *v6 = *v6 ^ -(unsigned int)((v3 & v2) != 0) & v4;
            v5 -= 1;
            v6 = v7;
          } while (v5);
          v12 += 1;
          v9 = v13;
        } while (v12 <= v14);
        v5 = v14;
        v8 = v15;
      }
      v5 += 1;
      v8 += 4;
      v14 = v5;
      v15 = v8;
    } while (v5 < a1);
  }
  v5 = 0;
  if (a3 <= 0)
    return 1;
  do {
    if (v11[v5 >> 5] & 1 << ((unsigned char)v5 & 0x1f)) // branch-flip
      *(char *)(v5 + a2) = 1;
    else {
      *(char *)(v5 + a2) = 0;
    }
    v5 += 1;
  } while (v5 < a3);
  return 1;
}

