// Function: sub_3a29 @ 0x3a29
void sub_3a29(int a0) // return-dupe
{
  dat_1a058 = a0;
  dat_1a05c = 0;
  dat_1a060 = 0;
  if (a0 == -1)
    return;
  dat_1a050 = sub_101d7;
  dat_1a05c = 0;
  dat_1a060 = 0;
}


// Function: sub_3a6c @ 0x3a6c
void sub_3a6c(unsigned int a0,int a1) // return-dupe
{
  int v1;
  unsigned int v2;
  char v3;
  
  if (0x10U - a1 < dat_1a060) {
    dat_1a05c |= (unsigned short)(a0 << ((unsigned char)dat_1a060 & 0x1f));
    if (0x3fffe <= dat_dd870) { // branch-flip
      v2 = dat_dd870 + 1;
      *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)dat_1a05c;
      dat_dd870 = v2;
      if (v2 == 0x40000)
        sub_f28e();
      v1 = dat_dd870 + 1;
      *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)(dat_1a05c >> 8);
      dat_dd870 = v1;
      if (v1 == 0x40000)
        sub_f28e();
    }
    else {
      *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)dat_1a05c;
      v1 = dat_dd870 + 2;
      *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)(dat_1a05c >> 8);
      dat_dd870 = v1;
    }
    v3 = (char)dat_1a060;
    dat_1a060 = a1 + dat_1a060 + -0x10;
    dat_1a05c = (short)((int)(a0 & 0xffff) >> (0x10U - v3 & 0x1fU));
    return;
  }
  dat_1a05c |= (unsigned short)(a0 << ((unsigned char)dat_1a060 & 0x1f));
  dat_1a060 = a1 + dat_1a060;
}


// Function: sub_3bed @ 0x3bed
unsigned int sub_3bed(unsigned int a0,int a1)
{
  unsigned int v1;
  unsigned int v2; // ebx
  int v3; // stack - 0x18
  unsigned int v4; // stack - 0x14
  
  v1 = 0;
  v3 = a1;
  v4 = a0;
  do {
    v2 = v1 | v4 & 1;
    v4 >>= 1;
    v1 = v2 * 2;
    v3 -= 1;
  } while (0 < v3);
  return v2 & 0x7fffffff;
}


// Function: sub_3c22 @ 0x3c22
void sub_3c22(void) // return-dupe x5
{
  unsigned int v1;
  int v2;
  char v3; // cl
  
  v3 = (char)dat_1a05c;
  if (dat_1a060 <= 8) {
    if (dat_1a060 <= 0) {
      dat_1a05c = 0;
      dat_1a060 = 0;
      return;
    }
    v2 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v3;
    if (v2 != 0x40000) {
      dat_1a05c = 0;
      dat_1a060 = 0;
      dat_dd870 = v2;
      return;
    }
    dat_dd870 = v2;
    sub_f28e();
    dat_1a05c = 0;
    dat_1a060 = 0;
    return;
  }
  if (dat_dd870 <= 0x3fffd) {
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v3;
    *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)((unsigned short)dat_1a05c >> 8);
    dat_dd870 += 2;
    dat_1a05c = 0;
    dat_1a060 = 0;
    return;
  }
  v1 = dat_dd870 + 1;
  *(char *)((unsigned long)dat_dd870 + 0x5c000) = v3;
  dat_dd870 = v1;
  if (v1 == 0x40000)
    sub_f28e();
  v2 = dat_dd870 + 1;
  *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)((unsigned short)dat_1a05c >> 8);
  if (v2 != 0x40000) {
    dat_1a05c = 0;
    dat_1a060 = 0;
    dat_dd870 = v2;
    return;
  }
  dat_dd870 = v2;
  sub_f28e();
  dat_1a05c = 0;
  dat_1a060 = 0;
}


// Function: sub_3d59 @ 0x3d59
void sub_3d59(char *a0,int a1,int a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  unsigned char v4; // cl
  unsigned char v5; // cl
  int v6; // stack - 0x14
  char *v7; // stack - 0x10
  
  sub_3c22();
  v6 = a1;
  v7 = a0;
  if (a2) {
    v4 = (unsigned char)a1; // branch-flip
    v5 = (unsigned char)((unsigned int)a1 >> 8);
    if (0x3fffe <= dat_dd870) {
      v1 = dat_dd870 + 1;
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = v4;
      dat_dd870 = v1;
      if (v1 == 0x40000)
        sub_f28e();
      v1 = dat_dd870 + 1;
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
      dat_dd870 = v1;
      if (v1 == 0x40000)
        sub_f28e();
    }
    else {
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = v4;
      v1 = dat_dd870 + 2;
      *(unsigned char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = v5;
      dat_dd870 = v1;
    }
    if (0x3fffe <= dat_dd870) { // branch-flip
      v1 = dat_dd870 + 1;
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = ~v4;
      dat_dd870 = v1;
      if (v1 == 0x40000)
        sub_f28e();
      v1 = dat_dd870 + 1;
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = ~v5;
      dat_dd870 = v1;
      v6 = a1;
      v7 = a0;
      if (v1 == 0x40000)
        sub_f28e();
    }
    else {
      *(unsigned char *)((unsigned long)dat_dd870 + 0x5c000) = ~v4;
      v1 = dat_dd870 + 2;
      *(unsigned char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = ~v5;
      dat_dd870 = v1;
      v6 = a1;
      v7 = a0;
    }
  }
  while (v2 = v6 + -1, v6) {
    v3 = &v7[1];
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = *v7;
    dat_dd870 = v1;
    v6 = v2;
    v7 = v3;
    if (v1 == 0x40000)
      sub_f28e();
  }
}


// Function: sub_3f52 @ 0x3f52
void sub_3f52(int a0) // return-dupe
{
  unsigned long v1; // rbx
  
  if ((a0 <= 0) || (10 <= a0))
    sub_f5ef("bad pack level"); // no-return
  memset((void *)0xcd000,0,0x10000);
  dat_1a0a0 = 0xffffffff;
  dat_1a098 = 0;
  dat_1a090 = (unsigned int)*(unsigned short *)((long)a0 * 8 + 0x19042);
  dat_1a080 = (unsigned int)*(unsigned short *)((long)a0 * 8 + 0x19040);
  dat_1a0a8 = (unsigned int)*(unsigned short *)((long)a0 * 8 + 0x19044);
  dat_1a07c = (unsigned int)*(unsigned short *)((long)a0 * 8 + 0x19046);
  dat_1a074 = 0;
  dat_1a068 = 0;
  dat_1a08c = (*dat_1a050)(0xad000,0x10000);
  if ((dat_1a08c) && (dat_1a08c != 0xffffffff)) {
    dat_1a088 = 0;
    while ((dat_1a08c <= 0x105 && (!dat_1a088))) {
      sub_4350();
    }
    dat_1a084 = 0;
    for (v1 = 0; (unsigned int)v1 < 2; v1 = (unsigned long)((unsigned int)v1 + 1)) {
      dat_1a084 = ((unsigned int)*(unsigned char *)(v1 + 0xad000) ^ dat_1a084 << 5) & 0x7fff;
    }
    return;
  }
  dat_1a08c = 0;
  dat_1a088 = 1;
}


// Function: sub_40fe @ 0x40fe
unsigned int sub_40fe(unsigned int a0)
{
  char *v1;
  unsigned int v10; // stack - 0x44
  unsigned int v11; // stack - 0x3c
  unsigned int v12; // stack - 0x38
  unsigned int v2;
  int v3; // eax
  unsigned long v4; // rax
  char *v5; // rbx
  char *v6;
  char *v7;
  char v8;
  char v9;
  
  v11 = dat_1a07c;
  v6 = (char *)((unsigned long)dat_1a074 + 0xad000);
  v12 = dat_1a070;
  v2 = dat_1a074;
  if (dat_1a074 <= 0x7ef9)
    v2 = 0x7efa;
  v1 = (char *)((unsigned long)dat_1a074 + 0xad102);
  v8 = v6[(long)(int)dat_1a070 + -1];
  v9 = v6[(int)dat_1a070];
  v10 = a0;
  if (dat_1a080 <= dat_1a070)
    v11 = dat_1a07c >> 2;
  do {
    v4 = (unsigned long)v10;
    v7 = (char *)(v4 + 0xad000);
    if ((((v9 == v7[(int)v12]) && (v8 == v7[(long)(int)v12 + -1])) && (*v7 == *v6)) && (*(char *)(v4 + 0xad001) == v6[1])) {
      v7 = (char *)(v4 + 0xad002);
      v6 = &v6[2];
      while (((((v5 = &v6[1], *v5 == v7[1] && (v5 = &v6[2], *v5 == v7[2])) && ((v5 = &v6[3], *v5 == v7[3] && ((v5 = &v6[4], *v5 == v7[4] && (v5 = &v6[5], *v5 == v7[5])))))) && (v5 = &v6[6], *v5 == v7[6])) && (v5 = &v6[7], *v5 == v7[7]))) {
        v5 = &v6[8];
        v7 = &v7[8];
        if ((*v5 != *v7) || (v6 = v5, v1 <= v5)) break;
      }
      v3 = 0x102 - ((int)v1 - (int)v5);
      v6 = (char *)((unsigned long)dat_1a074 + 0xad000);
      if ((int)v12 < v3) {
        dat_1a078 = v10;
        if (dat_1a0a8 <= v3)
          return v3;
        v8 = v6[(long)v3 + -1];
        v9 = v6[v3];
        v12 = v3;
      }
    }
    v10 = (unsigned int)*(unsigned short *)((unsigned long)(v10 & 0x7fff) * 2 + 0xbd000);
    if (v10 <= v2 - 0x7efa)
      return v12;
    v11 -= 1;
    if (!v11)
      return v12;
  } while( true );
}


// Function: sub_4350 @ 0x4350
void sub_4350(void) // return-dupe x2
{
  unsigned short v1;
  short v2; // ax
  int v3; // eax
  unsigned long v4; // rbx
  int v5; // stack - 0x1c
  
  v5 = ((int)dat_19020 - dat_1a08c) - dat_1a074;
  if (v5 != -1) { // branch-flip
    if (0xfefa <= dat_1a074) {
      memcpy((void *)0xad000,(void *)0xb5000,0x8000);
      dat_1a078 -= 0x8000;
      dat_1a074 -= 0x8000;
      if (dat_1a0a0 != 0xffffffff)
        dat_1a0a0 -= 0x8000;
      dat_1a068 -= 0x8000;
      for (v4 = 0; (unsigned int)v4 <= 0x7fff; v4 = (unsigned long)((unsigned int)v4 + 1)) {
        v1 = *(unsigned short *)((v4 + 0x8000) * 2 + 0xbd000);
        if (0x8000 <= v1) // branch-flip
          v2 = v1 + 0x8000;
        else {
          v2 = 0;
        }
        *(short *)((v4 + 0x8000) * 2 + 0xbd000) = v2;
      }
      for (v4 = 0; (unsigned int)v4 <= 0x7fff; v4 = (unsigned long)((unsigned int)v4 + 1)) {
        v1 = *(unsigned short *)(v4 * 2 + 0xbd000);
        if (0x8000 <= v1) // branch-flip
          v2 = v1 + 0x8000;
        else {
          v2 = 0;
        }
        *(short *)(v4 * 2 + 0xbd000) = v2;
      }
      v5 += 0x8000;
    }
  }
  else {
    v5 = -2;
  }
  if (dat_1a088)
    return;
  v3 = (*dat_1a050)((unsigned long)dat_1a074 + (unsigned long)dat_1a08c + 0xad000,v5);
  if ((v3) && (v3 != -1)) {
    dat_1a08c += v3;
    return;
  }
  dat_1a088 = 1;
  memset((void *)((unsigned long)dat_1a074 + (unsigned long)dat_1a08c + 0xad000),0,2);
}


// Function: sub_4567 @ 0x4567
void sub_4567(unsigned int a0,int a1) // return-dupe
{
  int v1; // stack - 0x20
  unsigned int v2; // stack - 0x1c
  unsigned int v3; // stack - 0xc
  
  v1 = a1;
  v2 = a0;
  if (a0 <= 0xfff) {
    for (v3 = a0; v3 <= 0xfff; v3 = v3 + 1) {
      if (v3 == a1 + a0)
        return;
      dat_1a098 += *(unsigned char *)((unsigned long)v3 + 0xad000);
    }
    v1 = a1 + a0 + -0x1000;
    v2 = 0x1000;
  }
  for (v3 = v2; v3 < v1 + v2; v3 = v3 + 1) {
    dat_1a098 = (dat_1a098 + *(unsigned char *)((unsigned long)v3 + 0xad000)) - (unsigned long)*(unsigned char *)((unsigned long)(v3 - 0x1000) + 0xad000);
    if ((dat_1a0a0 == 0xffffffff) && (!(dat_1a098 & 0xfff)))
      dat_1a0a0 = (unsigned long)v3;
  }
}


// Function: sub_4677 @ 0x4677
void sub_4677(void) // ternary x2
{
  char v1;
  unsigned short v2;
  unsigned int v3;
  char *v4; // rax
  int v5; // stack - 0x14
  unsigned int v6; // stack - 0x10
  
  v5 = 0;
  v6 = 0;
  dat_1a070 = 2;
  while (dat_1a08c) {
    dat_1a084 = ((unsigned int)*(unsigned char *)((unsigned long)(dat_1a074 + 2) + 0xad000) ^ dat_1a084 << 5) & 0x7fff;
    v2 = *(unsigned short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000);
    v3 = (unsigned int)v2;
    *(unsigned short *)((unsigned long)(dat_1a074 & 0x7fff) * 2 + 0xbd000) = v2;
    *(short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000) = (short)dat_1a074;
    if (((v3) && (dat_1a074 - v3 <= 0x7efa)) && ((unsigned long)dat_1a074 <= (unsigned long)(dat_19020 - 0x106U))) {
      v6 = sub_40fe(v3);
      if (dat_1a08c < v6)
        v6 = dat_1a08c;
    }
    if (3 <= v6) { // branch-flip
      v5 = sub_c45d(dat_1a074 - dat_1a078,v6 - 3);
      dat_1a08c -= v6;
      if (dat_dd874)
        sub_4567(dat_1a074,v6);
      if (v6 <= dat_1a090) { // branch-flip
        v6 -= 1;
        v3 = dat_1a074;
        do {
          dat_1a074 = v3;
          v3 = dat_1a074 + 1;
          dat_1a084 = ((unsigned int)*(unsigned char *)((unsigned long)(dat_1a074 + 3) + 0xad000) ^ dat_1a084 << 5) & 0x7fff;
          *(unsigned short *)((unsigned long)(v3 & 0x7fff) * 2 + 0xbd000) = *(unsigned short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000);
          *(short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000) = (short)v3;
          v6 -= 1;
        } while (v6);
        dat_1a074 += 2;
        v6 = 0;
      }
      else {
        dat_1a074 = v6 + dat_1a074;
        v6 = 0;
        dat_1a084 = (unsigned int)*(unsigned char *)((unsigned long)(dat_1a074 + 1) + 0xad000) ^ (unsigned int)*(unsigned char *)((unsigned long)dat_1a074 + 0xad000) << 5;
      }
    }
    else {
      v1 = *(char *)((unsigned long)dat_1a074 + 0xad000);
      v5 = sub_c45d(0,(unsigned char)v1);
      if (dat_dd874)
        sub_4567(dat_1a074,1);
      dat_1a08c -= 1;
      dat_1a074 += 1;
    }
    if ((dat_dd874) && (dat_1a0a0 < dat_1a074)) {
      dat_1a0a0 = 0xffffffff;
      v5 = 2;
    }
    if (v5) {
      v4 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
      sub_c1ea(v4,dat_1a074 - dat_1a068,v5 + -1,0);
      dat_1a068 = (unsigned long)dat_1a074;
    }
    while ((dat_1a08c <= 0x105 && (!dat_1a088))) {
      sub_4350();
    }
  }
  v4 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
  sub_c1ea(v4,dat_1a074 - dat_1a068,v5 + -1,1);
}


// Function: sub_4a9e @ 0x4a9e
void sub_4a9e(int a0) // return-dupe, ternary x4
{
  int v1;
  unsigned short v2;
  bool v3;
  unsigned int v4; // eax
  unsigned int v5;
  unsigned int v6;
  char *v7; // rax
  int v8; // stack - 0x28
  
  v8 = 0;
  v3 = 0;
  v6 = 2;
  sub_3f52(a0);
  if (a0 <= 3) {
    sub_4677();
    return;
  }
  while (v1 = dat_1a078, dat_1a08c) {
    dat_1a084 = ((unsigned int)*(unsigned char *)((unsigned long)(dat_1a074 + 2) + 0xad000) ^ dat_1a084 << 5) & 0x7fff;
    v2 = *(unsigned short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000);
    v4 = (unsigned int)v2;
    *(unsigned short *)((unsigned long)(dat_1a074 & 0x7fff) * 2 + 0xbd000) = v2;
    *(short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000) = (short)dat_1a074;
    v5 = 2;
    dat_1a070 = v6;
    if ((((v4) && (v6 < dat_1a090)) && (dat_1a074 - v4 <= 0x7efa)) && ((unsigned long)dat_1a074 <= (unsigned long)(dat_19020 - 0x106U))) {
      v5 = sub_40fe(v4);
      if (dat_1a08c < v5)
        v5 = dat_1a08c;
      if ((v5 == 3) && (0x1001 <= dat_1a074 - dat_1a078))
        v5 = 2;
    }
    v6 = v5;
    if ((3 <= dat_1a070) && (v6 <= dat_1a070)) {
      v8 = sub_c45d((dat_1a074 - v1) + -1,dat_1a070 - 3);
      dat_1a08c = (dat_1a08c - dat_1a070) + 1;
      v6 = dat_1a070 - 2;
      v5 = dat_1a074;
      if (dat_dd874) {
        v1 = dat_1a070 - 1;
        dat_1a070 = v6;
        sub_4567(dat_1a074,v1);
        v6 = dat_1a070;
        v5 = dat_1a074;
      }
      do {
        dat_1a074 = v5;
        dat_1a070 = v6;
        v5 = dat_1a074 + 1;
        dat_1a084 = ((unsigned int)*(unsigned char *)((unsigned long)(dat_1a074 + 3) + 0xad000) ^ dat_1a084 << 5) & 0x7fff;
        *(unsigned short *)((unsigned long)(v5 & 0x7fff) * 2 + 0xbd000) = *(unsigned short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000);
        *(short *)(((unsigned long)dat_1a084 + 0x8000) * 2 + 0xbd000) = (short)v5;
        dat_1a070 -= 1;
        v6 = dat_1a070;
      } while (dat_1a070);
      v3 = 0;
      v6 = 2;
      dat_1a074 += 2;
      if ((dat_dd874) && (dat_1a0a0 < dat_1a074)) {
        dat_1a0a0 = 0xffffffff;
        v8 = 2;
      }
      if (v8) {
        v7 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
        sub_c1ea(v7,dat_1a074 - dat_1a068,v8 + -1,0);
        dat_1a068 = (unsigned long)dat_1a074;
      }
    }
    else if (v3) {
      v8 = sub_c45d(0,(unsigned char)*(char *)((unsigned long)(dat_1a074 - 1) + 0xad000));
      if ((dat_dd874) && (dat_1a0a0 < dat_1a074)) {
        dat_1a0a0 = 0xffffffff;
        v8 = 2;
      }
      if (v8) {
        v7 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
        sub_c1ea(v7,dat_1a074 - dat_1a068,v8 + -1,0);
        dat_1a068 = (unsigned long)dat_1a074;
      }
      if (dat_dd874)
        sub_4567(dat_1a074,1);
      dat_1a074 += 1;
      dat_1a08c -= 1;
    }
    else {
      if ((dat_dd874) && (dat_1a0a0 < dat_1a074)) {
        dat_1a0a0 = 0xffffffff;
        v8 = 2;
        v7 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
        sub_c1ea(v7,dat_1a074 - dat_1a068,1,0);
        dat_1a068 = (unsigned long)dat_1a074;
      }
      v3 = 1;
      if (dat_dd874)
        sub_4567(dat_1a074,1);
      dat_1a074 += 1;
      dat_1a08c -= 1;
    }
    while ((dat_1a08c <= 0x105 && (!dat_1a088))) {
      sub_4350();
    }
  }
  if (v3)
    sub_c45d(0,(unsigned char)*(char *)((unsigned long)(dat_1a074 - 1) + 0xad000));
  v7 = (0 <= (long)dat_1a068) ? (char *)((dat_1a068 & 0xffffffff) + 0xad000) : NULL; // branch-flip
  sub_c1ea(v7,dat_1a074 - dat_1a068,v8 + -1,1);
}


// Function: sub_5097 @ 0x5097
void sub_5097(void)
{
  fprintf(stderr,"Try `%s --help\' for more information.\n",dat_dd010);
  sub_8e40(1); // no-return
}


// Function: sub_50ce @ 0x50ce
void sub_50ce(void)
{
  long *v1;
  long *v2; // stack - 0x10
  
  v2 = (long *)0x18a40;
  printf("Usage: %s [OPTION]... [FILE]...\n",dat_dd010);
  while (*v2) {
    v1 = &v2[1];
    printf("%s\n",(char *)*v2);
    v2 = v1;
  }
}


// Function: sub_513b @ 0x513b
void sub_513b(void)
{
  long *v1;
  long *v2; // stack - 0x10
  
  v2 = (long *)0x186a0;
  printf("%s %s\n",dat_dd010,dat_19558);
  while (*v2) {
    v1 = &v2[1];
    printf("%s\n",(char *)*v2);
    v2 = v1;
  }
}


// Function: sub_51af @ 0x51af
void sub_51af(void)
{
  sub_513b();
  printf("\n");
  printf("Written by Jean-loup Gailly.\n");
}


// Function: sub_51e7 @ 0x51e7
void sub_51e7(char *a0)
{
  int v1;
  
  v1 = *__errno_location();
  fprintf(stderr,"%s: ",dat_dd010);
  *__errno_location() = v1;
  perror(a0);
  dat_dd018 = 1;
}


// Function: main @ 0x5249
int main(int argc,char **argv,char **envp) // ternary
{
  int v1;
  char **v10; // stack - 0x20
  unsigned long v11; // stack - 0x18
  unsigned long v12; // stack - 0x10
  char *v2;
  long v3; // rax
  char **v4; // stack - 0x28
  int v5; // stack - 0x38
  int v6; // stack - 0x34
  long v7; // fs_offset
  int v8; // stack - 0x30
  int v9; // stack - 0x2c
  
  v12 = *(unsigned long *)(v7 + 0x28);
  dat_dd010 = (char *)sub_f40a(*argv);
  v11 = strlen(dat_dd010);
  if ((5 <= v11) && (v2 = &dat_dd010[v11 - 4], !strcmp(v2,".exe")))
    dat_dd010[v11 - 4] = '\0';
  v4 = argv;
  dat_dd8a8 = sub_f454(&v5,&v4,"GZIP");
  if (dat_dd8a8) // branch-flip
    v10 = v4;
  else {
    v10 = NULL;
  }
  dat_dd8b0 = ".gz";
  dat_dd8b8 = strlen(".gz");
  while( true ) {
    v6 = -1;
    if (v10) {
      if ((v10[optind]) && (v2 = v10[optind], !strcmp(v2,"--")))
        v8 = 0xb0;
      else {
        v8 = getopt_long(v5,v10,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x186e0,&v6);
        if (0 <= v8) // branch-flip
          v8 += 0x83;
        else {
          if (optind != v5) {
            fprintf(stderr,"%s: %s: non-option in GZIP environment variable\n",dat_dd010,v10[optind]);
            sub_5097(); // no-return
          }
          if ((v5 != 1) && (!dat_dd004))
            fprintf(stderr,"%s: warning: GZIP environment variable is deprecated; use an alias or script\n",dat_dd010);
          free(v10);
          v10 = NULL;
          optind = 1;
          v6 = -1;
        }
      }
    }
    if (!v10)
      v8 = getopt_long(argc,argv,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",(option *)0x186e0,&v6);
    if (v8 <= -1) break;
    switch(v8) {
      default:
        if ((0x83 <= v8) && (v8 != 0xc2)) {
          fprintf(stderr,"%s: ",dat_dd010);
          if (0 <= v6) // branch-flip
            fprintf(stderr,"--%s: ",*(char **)((long)v6 * 0x20 + 0x186e0));
          else {
            fprintf(stderr,"-%c: ",(char)v8 + '}');
          }
          fprintf(stderr,"option not valid in GZIP environment variable\n");
        }
        sub_5097(); // no-return
      case 0x48:
      case 0x68:
        sub_50ce();
        sub_8e91(); // no-return
      case 0x4c:
        sub_513b();
        sub_8e91(); // no-return
      case 0x4d:
        dat_19094 = 0;
        break;
      case 0x4e:
      case 0xd1:
        dat_19094 = 0;
        dat_19090 = 0;
        break;
      case 0x53:
        dat_dd8b8 = strlen(optarg);
        dat_dd8b0 = optarg;
        break;
      case 0x56:
        sub_51af();
        sub_8e91(); // no-return
      case 0x5a:
        fprintf(stderr,"%s: -Z not supported in this version\n",dat_dd010);
        sub_5097(); // no-return
      case 0x61:
        dat_dd87c = 1;
        break;
      case 0x62:
        dat_19098 = atoi(optarg);
        for (; *optarg; optarg = &optarg[1]) {
          if ((*optarg <= '/') || (':' <= *optarg)) {
            fprintf(stderr,"%s: -b operand is not an integer\n",dat_dd010);
            sub_5097(); // no-return
          }
        }
        break;
      case 99:
        dat_dd000 = 1;
        break;
      case 100:
        dat_dd880 = 1;
        break;
      case 0x66:
        dat_dd884 += 1;
        break;
      case 0x6b:
        dat_dd888 = 1;
        break;
      case 0x6c:
        dat_dd000 = 1;
        dat_dd008 = 1;
        dat_dd880 = 1;
        dat_dd890 = 1;
        break;
      case 0x6d:
        dat_19094 = 1;
        break;
      case 0x6e:
      case 0xf1:
        dat_19094 = 1;
        dat_19090 = 1;
        break;
      case 0x71:
      case 0xf4:
        dat_dd004 = 1;
        dat_dd894 = 0;
        break;
      case 0x72:
        dat_dd88c = 1;
        break;
      case 0x74:
        dat_dd000 = 1;
        dat_dd880 = 1;
        dat_dd008 = 1;
        break;
      case 0x76:
      case 0xf9:
        dat_dd894 += 1;
        dat_dd004 = 0;
        break;
      case 0x80:
        dat_dd878 = 1;
        break;
      case 0x81:
      case 0x104:
        dat_dd874 = 1;
        break;
      case 0x82:
        dat_dd879 = '\x01';
        break;
      case 0xb4:
      case 0xb5:
      case 0xb6:
      case 0xb7:
      case 0xb8:
      case 0xb9:
      case 0xba:
      case 0xbb:
      case 0xbc:
        v8 -= 0x83;
        goto label_5783;
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
label_5783:
        dat_190a0 = v8 + -0x30;
      
    }
  }
  if (dat_19094 < 0)
    dat_19094 = dat_dd880;
  if (dat_19090 <= -1)
    dat_19090 = dat_dd880;
  v9 = argc - optind;
  if ((dat_dd87c) && (!dat_dd004))
    fprintf(stderr,"%s: option --ascii ignored on this system\n",dat_dd010);
  if ((dat_dd8b8) && (dat_dd8b8 <= 0x1e)) {
    dat_dd940 = (dat_dd004) ? 0xd : 0; // branch-flip
    sub_8c07();
    if (v9) { // branch-flip
      while (optind < argc) {
        v1 = optind + 1;
        v3 = (long)optind;
        optind = v1;
        sub_5f11(argv[v3]);
      }
    }
    else {
      sub_5b3e();
    }
    if ((dat_ddd60) && (close(0))) {
      strcpy((char *)0xdd060,"stdin");
      sub_f6c7(); // no-return
    }
    if (dat_dd890) {
      if ((!dat_dd004) && (2 <= v9))
        sub_7fdc(0xffffffff);
      v1 = sub_109d7(stdout);
      if (v1)
        sub_f74d(); // no-return
    }
    if (((dat_dd000) && ((((dat_dd879 && (fdatasync(1))) && (*__errno_location() != 0x16)) || (close(1))))) && (*__errno_location() != 9))
      sub_f74d(); // no-return
    sub_8e40(dat_dd018); // no-return
  }
  fprintf(stderr,"%s: invalid suffix \'%s\'\n",dat_dd010,dat_dd8b0);
  sub_8e40(1); // no-return
}


// Function: sub_5a65 @ 0x5a65
unsigned long sub_5a65(void) // return-dupe
{
  if ((!dat_dd880) || (dat_dd89c))
    return 1;
  if (dat_dd86c == dat_dd868) {
    if ((dat_dd868 != 0x40000) || (sub_f0bc(1) == -1))
      return 1;
    dat_dd86c = 0;
  }
  return 0;
}


// Function: sub_5acc @ 0x5acc
void sub_5acc(void)
{
  char v1 [16];
  
  dat_dd020 = 0xffffffffffffffff;
  dat_dd038 = 0xffffffffffffffff;
  if (((dat_de198 & 0xf000) == 0x8000) && ((dat_dd020 = dat_de1b0, !dat_19094 || (dat_dd890)))) {
    v1 = sub_1108f(0xde180);
    dat_dd038 = SUB168(v1,8);
    dat_dd030 = SUB168(v1,0);
  }
}


// Function: sub_5b3e @ 0x5b3e
void sub_5b3e(void) // return-dupe x6
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  bool v4; // zf
  
  if (((!dat_dd884) && (!dat_dd890)) && ((dat_dd878 || (v4 = dat_dd880 == 0, isatty((unsigned int)v4))))) {
    if (!dat_dd004) {
      if (dat_dd880) { // branch-flip
        v3 = "de";
        v2 = "read from";
      }
      else {
        v3 = "";
        v2 = "written to";
      }
      fprintf(stderr,"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n",dat_dd010,v2,v3,dat_dd010);
    }
    sub_8e40(1); // no-return
  }
  strcpy((char *)0xdd060,"stdin");
  strcpy((char *)0xdd460,"stdout");
  if (fstat(0,(stat *)0xde180)) {
    sub_51e7("standard input");
    sub_8e40(1); // no-return
  }
  sub_5acc();
  sub_f078();
  dat_dd000 = 1;
  dat_dd8a0 = 0;
  dat_dd860 = 0;
  dat_ddd60 = 1;
  if ((dat_dd880) && (dat_1909c = sub_72f1(0), dat_1909c <= -1))
    sub_8e40(dat_dd018); // no-return
  while( true ) {
    v1 = (*dat_190d0)(0,1);
    if (v1)
      return;
    if (sub_5a65()) break;
    dat_1909c = sub_72f1(dat_dd860);
    if (dat_1909c < 0)
      return;
    dat_dd048 = 0;
  }
  if (dat_dd890) {
    sub_7fdc(dat_1909c);
    return;
  }
  if (!dat_dd894)
    return;
  if (dat_dd008) {
    fprintf(stderr," OK\n");
    return;
  }
  if (dat_dd880)
    return;
  sub_f7a6(dat_dd040 - (dat_dd048 - dat_e1698),dat_dd040,stderr);
  fprintf(stderr,"\n");
}


// Function: sub_5dee @ 0x5dee
unsigned int sub_5dee(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // stack - 0x18
  char *v2; // stack - 0x10
  
  v1 = a1;
  v2 = a0;
  if (!a1) {
    v2 = ".-";
    v1 = 1;
  }
  if ((!memcmp((void *)0xddd80,v2,v1)) && (!*(char *)(v1 + 0xddd80)))
    return 1;
  return 0;
}


// Function: sub_5e60 @ 0x5e60
int sub_5e60(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x18
  char *v2; // stack - 0x10
  
  if (sub_5dee(a0,a1) != '\x01') {
    if (0 <= dat_190a8)
      close(dat_190a8);
    v1 = a1;
    v2 = a0;
    if (!a1) {
      v2 = ".-";
      v1 = 1;
    }
    memcpy((void *)0xddd80,v2,v1);
    *(char *)(v1 + 0xddd80) = 0;
    dat_190a8 = sub_10863((char *)0xddd80,0x10000);
  }
  return dat_190a8;
}


// Function: sub_5f11 @ 0x5f11
void sub_5f11(char *a0) // return-dupe x10, ternary x2
{
  int v1; // eax
  int v2;
  char *v3; // rax
  sigset_t v4;
  
  v1 = strcmp(a0,"-");
  v2 = dat_dd000;
  if (!v1) {
    sub_5b3e();
    dat_dd000 = v2;
    return;
  }
  dat_dd860 = sub_6d5a(a0,(stat *)0xde180);
  if (dat_dd860 <= -1)
    return;
  if ((dat_de198 & 0xf000) == 0x4000) {
    if (dat_dd88c) {
      sub_897b(dat_dd860,a0);
      return;
    }
    close(dat_dd860);
    if (!dat_dd004)
      fprintf(stderr,"%s: %s is a directory -- ignored\n",dat_dd010,(char *)0xdd060);
    if (dat_dd018)
      return;
    dat_dd018 = 2;
    return;
  }
  if (!dat_dd000) {
    if ((dat_de198 & 0xf000) != 0x8000) {
      if (!dat_dd004)
        fprintf(stderr,"%s: %s is not a directory or a regular file - ignored\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018)
        dat_dd018 = 2;
      close(dat_dd860);
      return;
    }
    if (dat_de198 & 0x800) {
      if (!dat_dd004)
        fprintf(stderr,"%s: %s is set-user-ID on execution - ignored\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018)
        dat_dd018 = 2;
      close(dat_dd860);
      return;
    }
    if (dat_de198 & 0x400) {
      if (!dat_dd004)
        fprintf(stderr,"%s: %s is set-group-ID on execution - ignored\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018)
        dat_dd018 = 2;
      close(dat_dd860);
      return;
    }
    if (!dat_dd884) {
      if (dat_de198 & 0x200) {
        if (!dat_dd004)
          fprintf(stderr,"%s: %s has the sticky bit set - file ignored\n",dat_dd010,(char *)0xdd060);
        if (!dat_dd018)
          dat_dd018 = 2;
        close(dat_dd860);
        return;
      }
      if (2 <= dat_de190) {
        if (!dat_dd004) {
          v3 = (dat_de190 != 2) ? "s" : ""; // branch-flip
          fprintf(stderr,"%s: %s has %lu other link%s -- file ignored\n",dat_dd010,(char *)0xdd060,dat_de190 - 1,v3);
        }
        if (!dat_dd018)
          dat_dd018 = 2;
        close(dat_dd860);
        return;
      }
    }
  }
  sub_5acc();
  if ((dat_dd000) && (!dat_dd008))
    strcpy((char *)0xdd460,"stdout");
  else if (sub_6ff8()) {
    close(dat_dd860);
    return;
  }
  sub_f078();
  dat_dd8a0 = 0;
  if ((dat_dd880) && (dat_1909c = sub_72f1(dat_dd860), dat_1909c <= -1)) {
    close(dat_dd860);
    return;
  }
  if (dat_dd000) // branch-flip
    dat_dd864 = 1;
  else {
    if (sub_6814())
      return;
    if ((((!dat_dd880) && (dat_dd01c)) && (!dat_dd894)) && (!dat_dd004))
      fprintf(stderr,"%s: %s compressed to %s\n",dat_dd010,(char *)0xdd060,(char *)0xdd460);
  }
  if (!dat_dd01c)
    dat_dd01c = (unsigned int)(dat_19090 == 0);
  if ((dat_dd894) && (!dat_dd890))
    fprintf(stderr,"%s:\t",(char *)0xdd060);
  while (v2 = (*dat_190d0)(dat_dd860,dat_dd864), !v2) {
    if ((sub_5a65()) || (dat_1909c = sub_72f1(dat_dd860), dat_1909c < 0)) goto label_645e;
    dat_dd048 = 0;
  }
  dat_1909c = -1;
label_645e:
  if (close(dat_dd860))
    sub_f6c7(); // no-return
  if (dat_dd890) {
    sub_7fdc(dat_1909c);
    return;
  }
  if (!dat_dd000) {
    sub_870d(0xde180);
    if (((dat_dd879) && ((((0 <= dat_190a8 && (fdatasync(dat_190a8))) && (*__errno_location() != 0x16)) || ((fsync(dat_dd864) && (*__errno_location() != 0x16)))))) || (close(dat_dd864)))
      sub_f74d(); // no-return
    if (!dat_dd888) {
      v3 = (char *)sub_10244((char *)0xdd060);
      if (sub_5dee((char *)0xdd060,&v3[-0xdd060])) // branch-flip
        v2 = dat_190a8;
      else {
        v2 = -1;
      }
      sigprocmask(0,(sigset_t *)0xdd8c0,&v4);
      dat_190a4 = 0xffffffff;
      if (0 <= v2) // branch-flip
        v2 = unlinkat(v2,v3,0);
      else {
        v2 = sub_f430((char *)0xdd060);
      }
      if (v2) // branch-flip
        v2 = *__errno_location();
      else {
        v2 = 0;
      }
      sigprocmask(2,&v4,NULL);
      if (v2) {
        if (!dat_dd004)
          fprintf(stderr,"%s: ",dat_dd010);
        if (!dat_dd018)
          dat_dd018 = 2;
        if (!dat_dd004) {
          *__errno_location() = v2;
          perror((char *)0xdd060);
        }
      }
    }
  }
  if (dat_1909c != -1) {
    if (!dat_dd894)
      return;
    if (dat_dd008) // branch-flip
      fprintf(stderr," OK");
    else if (dat_dd880) // branch-flip
      sub_f7a6(dat_dd048 - (dat_dd040 - dat_e1698),dat_dd048,stderr);
    else {
      sub_f7a6(dat_dd040 - (dat_dd048 - dat_e1698),dat_dd040,stderr);
    }
    if (!dat_dd008) {
      v3 = (dat_dd888) ? "created" : "replaced with"; // branch-flip
      fprintf(stderr," -- %s %s",v3,(char *)0xdd460);
    }
    fprintf(stderr,"\n");
    return;
  }
  if (dat_dd000)
    return;
  sub_8ebb(0);
}


// Function: sub_67dc @ 0x67dc
void sub_67dc(char *a0,char *a1)
{
  char *v1;
  char *v2;
  char v3;
  char *v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  v4 = a1;
  v5 = a0;
  do {
    v1 = &v4[1];
    v2 = &v5[1];
    v3 = *v4;
    *v5 = v3;
    v4 = v1;
    v5 = v2;
  } while (v3);
}


// Function: sub_6814 @ 0x6814
unsigned long sub_6814(void) // return-dupe x2
{
  bool v1;
  int v2;
  char *v3; // rax
  sigset_t v4;
  int v5; // stack - 0xb8
  char *v6; // stack - 0xa8
  
  v1 = 0;
  v6 = (char *)0xdd460;
  v5 = -100;
  if (!dat_dd888) {
    v3 = (char *)sub_10244((char *)0xdd460);
    v2 = sub_5e60((char *)0xdd460,&v3[-0xdd460]);
    if (0 <= v2) {
      v6 = v3;
      v5 = v2;
    }
  }
  while( true ) {
    while( true ) {
      sub_67dc((char *)0xdd960,(char *)0xdd460);
      sigprocmask(0,(sigset_t *)0xdd8c0,&v4);
      dat_190a4 = sub_10ae4(v5,v6,0xc1,0x180);
      dat_dd864 = dat_190a4;
      v2 = *__errno_location();
      sigprocmask(2,&v4,NULL);
      if (0 <= dat_dd864) {
        if ((v1) && (dat_dd880)) {
          if (!dat_dd004)
            fprintf(stderr,"%s: %s: warning, name truncated\n",dat_dd010,(char *)0xdd460);
          if (!dat_dd018)
            dat_dd018 = 2;
        }
        return 0;
      }
      if (v2 != 0x11) break;
      if (sub_85c8()) {
        close(dat_dd860);
        return 1;
      }
    }
    if (v2 != 0x24) break;
    sub_83ea((char *)0xdd460);
    v1 = 1;
  }
  sub_51e7((char *)0xdd460);
  close(dat_dd860);
  return 1;
}


// Function: sub_6a54 @ 0x6a54
char * sub_6a54(char *a0) // ternary
{
  char *v1;
  unsigned char v10; // stack - 0x65
  long *v11; // stack - 0x58
  char *v12; // stack - 0x50
  int v2; // eax
  int v3; // eax
  int v4; // eax
  unsigned long v5; // rax
  long v6; // rax
  unsigned char *v7; // rax
  long v8;
  char v9 [40];
  
  v10 = 0;
  for (v11 = (long *)0x190e8; *v11; v11 = &v11[1]) {
    v5 = strlen((char *)*v11);
    if ((dat_dd8b8 < v5) && (v8 = *v11, v6 = v5 - dat_dd8b8, !strcmp(dat_dd8b0,(char *)(v8 + v6)))) {
      v10 = 1;
      break;
    }
  }
  v7 = (unsigned char *)sub_12377(dat_dd8b0);
  sub_f393(v7);
  v8 = (v10) ? 8 : 0; // branch-flip
  *(unsigned char **)(v8 * 8 + 0x190e0) = v7;
  v11 = (long *)((unsigned long)v10 * 8 + 0x190e0);
  v2 = (int)strlen(a0);
  if (0x21 <= v2) // branch-flip
    strcpy(v9,&a0[(long)v2 + -0x20]);
  else {
    strcpy(v9,a0);
  }
  sub_f393(v9);
  v3 = (int)strlen(v9);
  v12 = NULL;
  while( true ) {
    v1 = (char *)*v11;
    v4 = (int)strlen(v1);
    if (((v4 < v3) && (v9[(v3 - v4) + -1] != '/')) && (v1 = (char *)*v11, !strcmp(&v9[(long)v3 - (long)v4],v1))) break;
    v11 = &v11[1];
    if (!*v11) {
      free(v7); // return-dupe
      return v12;
    }
  }
  v12 = &a0[(long)v2 - (long)v4];
  free(v7);
  return v12;
}


// Function: sub_6c6d @ 0x6c6d
int sub_6c6d(long a0,unsigned int a1,stat *a2)
{
  int v1;
  int v2; // eax
  long v3; // rax
  unsigned int v4; // stack - 0x34
  int v5; // stack - 0x28
  long v6; // stack - 0x18
  
  v5 = -100;
  v4 = a1;
  if ((!dat_dd000) && (!dat_dd884))
    v4 = a1 | 0x20000;
  v6 = a0;
  if (!dat_dd888) {
    v3 = sub_10244(a0);
    v2 = sub_5e60(a0,v3 - a0);
    if (0 <= v2) {
      v6 = v3;
      v5 = v2;
    }
  }
  v2 = sub_10ae4(v5,v6,v4);
  if ((0 <= v2) && (fstat(v2,a2))) {
    v1 = *__errno_location();
    close(v2);
    *__errno_location() = v1;
    return -1;
  }
  return v2;
}


// Function: sub_6d5a @ 0x6d5a
int sub_6d5a(char *a0,stat *a1) // early-return x5
{
  char *v1;
  int v2; // eax
  unsigned long v3; // rax
  int v4; // stack - 0x40
  long *v5; // stack - 0x30
  
  v4 = 0;
  v5 = (long *)0x19140;
  dat_19140 = dat_dd8b0;
  if (strlen(a0) <= 0x3fe) {
    strcpy((char *)0xdd060,a0);
    v2 = sub_6c6d(0xdd060,0x900,a1);
    if (0 <= v2)
      return v2;
    if ((!dat_dd880) || (*__errno_location() != 2)) {
      sub_51e7((char *)0xdd060);
      return -1;
    }
    if (sub_6a54((char *)0xdd060)) {
      sub_51e7((char *)0xdd060);
      return -1;
    }
    v3 = strlen((char *)0xdd060);
    if (!strcmp(dat_dd8b0,".gz"))
      v5 = (long *)0x19148;
    while( true ) {
      v1 = (char *)*v5;
      strcpy((char *)0xdd060,a0);
      if (0x400 <= strlen(v1) + (long)(int)v3) break;
      strcat((char *)0xdd060,v1);
      v2 = sub_6c6d(0xdd060,0x900,a1);
      if (0 <= v2)
        return v2;
      if (*__errno_location() != 2) {
        sub_51e7((char *)0xdd060);
        return -1;
      }
      if (!strcmp(v1,dat_dd8b0))
        v4 = *__errno_location();
      v5 = &v5[1];
      if (!*v5) {
        strcpy((char *)0xdd060,a0);
        strcat((char *)0xdd060,dat_dd8b0);
        *__errno_location() = v4;
        sub_51e7((char *)0xdd060);
        return -1;
      }
    }
  }
  fprintf(stderr,"%s: %s: file name too long\n",dat_dd010,a0);
  dat_dd018 = 1;
  return -1;
}


// Function: sub_6ff8 @ 0x6ff8
unsigned long sub_6ff8(void)
{
  unsigned char *v1; // rax
  
  strcpy((char *)0xdd460,(char *)0xdd060);
  v1 = (unsigned char *)sub_6a54((char *)0xdd460);
  if (dat_dd880) { // branch-flip
    if (!v1) {
      if ((!dat_dd88c) && (dat_dd008))
        return 0;
      if ((dat_dd894) || ((!dat_dd88c && (!dat_dd004)))) {
        if (!dat_dd004)
          fprintf(stderr,"%s: %s: unknown suffix -- ignored\n",dat_dd010,(char *)0xdd060);
        if (!dat_dd018)
          dat_dd018 = 2;
      }
      return 2;
    }
    sub_f393(v1);
    if ((strcmp((char *)v1,".tgz")) && (strcmp((char *)v1,".taz")))
      *v1 = 0;
    else {
      strcpy((char *)v1,".tar");
    }
  }
  else {
    if ((v1) && (!dat_dd884)) {
      if ((dat_dd894) || ((!dat_dd88c && (!dat_dd004))))
        fprintf(stderr,"%s: %s already has %s suffix -- unchanged\n",dat_dd010,(char *)0xdd060,(char *)v1);
      return 2;
    }
    dat_dd01c = 0;
    if (0x400 <= strlen((char *)0xdd460) + dat_dd8b8) {
      if (!dat_dd004)
        fprintf(stderr,"%s: %s: file name too long\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018)
        dat_dd018 = 2;
      return 2;
    }
    strcat((char *)0xdd460,dat_dd8b0);
  }
  return 0;
}


// Function: sub_7240 @ 0x7240
void sub_7240(long a0,unsigned int a1)
{
  unsigned int v1;
  char v2; // stack - 0x11
  long v3; // stack - 0x20
  
  v3 = a0;
  do {
    while( true ) {
      if (!v3)
        return;
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v2 = sub_f0bc(0);
      else {
        v1 = dat_dd86c + 1;
        v2 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      if (a1 & 2)
        sub_efda(&v2,1);
      if (v3 == -1) break;
      v3 -= 1;
    }
  } while (v2);
}


// Function: sub_72f1 @ 0x72f1
unsigned int sub_72f1(unsigned int a0) // early-return x2, return-dupe x3
{
  unsigned int v1;
  unsigned long v10; // rax
  unsigned long v11;
  unsigned long v12;
  unsigned long v13;
  unsigned char *v14; // rax
  char *v15; // rax
  char v16; // stack - 0x1a
  char v17; // stack - 0x1c
  unsigned int v18; // stack - 0x50
  unsigned int v19; // stack - 0x4c
  unsigned char *v2;
  unsigned int v20; // stack - 0x48
  unsigned char *v21; // stack - 0x38
  char v22; // stack - 0x1b
  char v23; // stack - 0x19
  char v24; // stack - 0x18
  unsigned char v25; // stack - 0x17
  char v26; // stack - 0x16
  char v27; // stack - 0x15
  char v28; // stack - 0x14
  char v29; // stack - 0x13
  unsigned short v3;
  char v30; // stack - 0x12
  char v31; // stack - 0x11
  unsigned char v4;
  char v5;
  unsigned char v6;
  unsigned int v7;
  unsigned int v8;
  unsigned int v9;
  
  if ((dat_dd884) && (dat_dd000)) {
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v18 = sub_f0bc(1);
    else {
      v7 = dat_dd86c + 1;
      v18 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v16 = (char)v18;
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v19 = sub_f0bc(1);
    else {
      v7 = dat_dd86c + 1;
      v19 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v23 = (char)v19;
  }
  else {
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v16 = sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v16 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v18 = 0;
    if (v16) { // branch-flip
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v23 = sub_f0bc(0);
      else {
        v7 = dat_dd86c + 1;
        v23 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v7;
      }
      v19 = 0;
    }
    else {
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v19 = sub_f0bc(1);
      else {
        v7 = dat_dd86c + 1;
        v19 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v7;
      }
      v23 = (char)v19;
    }
  }
  dat_1909c = 0xffffffff;
  dat_dd8a0 += 1;
  dat_e1698 = 0;
  dat_dd89c = 0;
  if ((memcmp(&v16,(void *)0x13a23,2)) && (memcmp(&v16,(void *)0x13a26,2))) {
    if (((memcmp(&v16,(void *)0x13b7c,2)) || (dat_dd86c != 2)) || (memcmp((void *)0x1b000,(void *)0x13b7c,4))) {
      if (memcmp(&v16,(void *)0x13b81,2)) { // branch-flip
        if (memcmp(&v16,(void *)0x13b84,2)) { // branch-flip
          if (memcmp(&v16,(void *)0x13b87,2)) { // branch-flip
            if (((dat_dd884) && (dat_dd000)) && (!dat_dd890)) {
              dat_1909c = 0;
              dat_190d0 = sub_ef1d;
              if (v19 != 0xffffffff)
                dat_dd86c -= 1;
              dat_dd89c = 1;
              if (v18 != 0xffffffff)
                sub_f321(1,&v16,1);
            }
          }
          else {
            dat_190d0 = sub_d7d6;
            dat_1909c = 3;
            dat_dd89c = 1;
          }
        }
        else {
          dat_190d0 = sub_d846;
          dat_1909c = 1;
          dat_dd89c = 1;
        }
      }
      else {
        dat_190d0 = sub_e3fd;
        dat_1909c = 2;
      }
    }
    else {
      dat_dd86c = 0;
      dat_190d0 = sub_e86a;
      if (sub_e655(a0))
        return 0xffffffff;
      dat_dd89c = 1;
    }
  }
  else {
    if (dat_dd868 <= dat_dd86c) // branch-flip
      dat_1909c = sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      dat_1909c = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    if (dat_1909c != 8) {
      fprintf(stderr,"%s: %s: unknown method %d -- not supported\n",dat_dd010,(char *)0xdd060,dat_1909c);
      dat_dd018 = 1;
      return 0xffffffff;
    }
    dat_190d0 = sub_e86a;
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v4 = sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v4 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    if (v4 & 0x20) {
      fprintf(stderr,"%s: %s is encrypted -- not supported\n",dat_dd010,(char *)0xdd060);
      dat_dd018 = 1;
      return 0xffffffff;
    }
    if (v4 & 0xc0) {
      fprintf(stderr,"%s: %s has flags 0x%x -- not supported\n",dat_dd010,(char *)0xdd060,(unsigned int)v4);
      dat_dd018 = 1;
      if (dat_dd884 <= 1) {
        dat_dd018 = 1;
        return 0xffffffff;
      }
    }
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v10 = (unsigned long)(int)sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v10 = (unsigned long)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v11 = (unsigned long)(int)sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v11 = (unsigned long)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v11 = v10 | v11 << 8;
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v12 = (unsigned long)(int)sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v12 = (unsigned long)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v12 = v11 | v12 << 0x10;
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v13 = (unsigned long)(int)sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v13 = (unsigned long)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    v13 = v12 | v13 << 0x18;
    if ((v13) && (!dat_19094)) {
      if (0 <= (long)v13) { // branch-flip
        dat_dd038 = 0;
        dat_dd030 = v13;
      }
      else {
        if (!dat_dd004)
          fprintf(stderr,"%s: %s: MTIME %lu out of range for this platform\n",dat_dd010,(char *)0xdd060,v13);
        if (!dat_dd018)
          dat_dd018 = 2;
        dat_dd030 = 0x7fffffffffffffff;
        dat_dd038 = 999999999;
      }
    }
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v30 = sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v30 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    if (dat_dd868 <= dat_dd86c) // branch-flip
      v31 = sub_f0bc(0);
    else {
      v7 = dat_dd86c + 1;
      v31 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v7;
    }
    if (v4 & 2) {
      v24 = 8;
      v26 = (char)v10;
      v27 = (char)(v11 >> 8);
      v28 = (char)(v12 >> 0x10);
      v29 = (char)(v13 >> 0x18);
      v25 = v4;
      sub_efda(NULL,0);
      sub_efda(&v16,10);
    }
    if (v4 & 4) {
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v5 = sub_f0bc(0);
      else {
        v7 = dat_dd86c + 1;
        v5 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v7;
      }
      v17 = v5;
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v22 = sub_f0bc(0);
      else {
        v7 = dat_dd86c + 1;
        v22 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v7;
      }
      v3 = CONCAT11(v22,v5);
      if (dat_dd894)
        fprintf(stderr,"%s: %s: extra field of %u bytes ignored\n",dat_dd010,(char *)0xdd060,(unsigned int)v3);
      if (v4 & 2)
        sub_efda(&v17,2);
      sub_7240((unsigned int)v3,v4);
    }
    if (v4 & 8) {
      if ((dat_19090) || (((dat_dd000 && (!dat_dd890)) || (2 <= dat_dd8a0))))
        sub_7240(-1,v4);
      else {
        v14 = (unsigned char *)sub_f40a((char *)0xdd460);
        v21 = v14;
        while( true ) {
          if (dat_dd868 <= dat_dd86c) // branch-flip
            v6 = sub_f0bc(0);
          else {
            v7 = dat_dd86c + 1;
            v6 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
            dat_dd86c = v7;
          }
          *v21 = v6;
          v2 = &v21[1];
          if (!*v21) break;
          v21 = v2;
          if ((unsigned char *)0xdd860 <= v2)
            sub_f5ef("corrupted input -- file name too large"); // no-return
        }
        if (v4 & 2)
          sub_efda(v14,(int)v2 - (int)v14);
        v15 = (char *)sub_f40a(v14);
        memmove(v14,v15,strlen(v15) + 1);
        if ((!dat_dd890) && (v14))
          dat_dd890 = 0;
      }
    }
    if (v4 & 0x10)
      sub_7240(-1,v4);
    if (v4 & 2) {
      v7 = sub_efda(&v16,0) & 0xffff;
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v8 = sub_f0bc(0);
      else {
        v9 = dat_dd86c + 1;
        v8 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v9;
      }
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v9 = sub_f0bc(0);
      else {
        v1 = dat_dd86c + 1;
        v9 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      v8 |= v9 << 8;
      if (v8 != v7) {
        fprintf(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",dat_dd010,(char *)0xdd060,v8,v7);
        dat_dd018 = 1;
        if (dat_dd884 <= 1) {
          dat_dd018 = 1;
          return 0xffffffff;
        }
      }
    }
    if (dat_dd8a0 == 1)
      dat_e1698 = (unsigned long)(dat_dd86c + 8);
  }
  if (0 <= (int)dat_1909c)
    return dat_1909c;
  if (dat_dd8a0 == 1) {
    fprintf(stderr,"\n%s: %s: not in gzip format\n",dat_dd010,(char *)0xdd060);
    dat_dd018 = 1;
    return 0xffffffff;
  }
  if (!v16) {
    v20 = v19;
    while (!v20) {
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v20 = sub_f0bc(1);
      else {
        v7 = dat_dd86c + 1;
        v20 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v7;
      }
    }
    if (v20 == 0xffffffff) {
      if (!dat_dd894)
        return 0xfffffffd;
      if (!dat_dd004)
        fprintf(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018) {
        dat_dd018 = 2;
        return 0xfffffffd;
      }
      return 0xfffffffd;
    }
  }
  if (!dat_dd004)
    fprintf(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",dat_dd010,(char *)0xdd060);
  if (!dat_dd018) {
    dat_dd018 = 2;
    return 0xfffffffe;
  }
  return 0xfffffffe;
}


// Function: sub_7fdc @ 0x7fdc
void sub_7fdc(int a0)
{
  tm *v1; // rax
  unsigned long v2; // stack - 0x18
  
  if ((dat_190c8) && (0 <= a0)) {
    dat_190c8 = 0;
    if (dat_dd894)
      printf("method  crc     date  time  ");
    if (!dat_dd004)
      printf("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,"uncompressed");
  }
  else if (a0 <= -1) {
    if (dat_ddd68 <= 0)
      return;
    if (dat_ddd70 <= 0)
      return;
    if (dat_dd894)
      printf("                            ");
    if ((dat_dd894) || (!dat_dd004)) {
      sub_f81a(stdout,dat_ddd68,0x13);
      printf(" ");
      sub_f81a(stdout,dat_ddd70,0x13);
      printf(" ");
    }
    sub_f7a6(dat_ddd70 - (dat_ddd68 - dat_e1698),dat_ddd70,stdout);
    printf(" (totals)\n");
    return;
  }
  v2 = 0xffffffffffffffff;
  if ((a0 == 8) && (!dat_dd89c))
    v2 = dat_e1680;
  if (dat_dd894) {
    v1 = localtime((void *)0xdd030);
    printf("%5s %08lx ",*(unsigned long *)((long)a0 * 8 + 0x18b20),v2);
    if (v1) // branch-flip
      printf("%s%3d %02d:%02d ",&"Jan"[(long)*(int *)&v1->field_0x10 * 4],(unsigned long)*(unsigned int *)&v1->field_0xc,(unsigned long)*(unsigned int *)&v1->field_0x8,(unsigned long)*(unsigned int *)&v1->field_0x4);
    else {
      printf("??? ?? ??:?? ");
    }
  }
  sub_f81a(stdout,dat_dd040,0x13);
  printf(" ");
  sub_f81a(stdout,dat_dd048,0x13);
  printf(" ");
  if (dat_dd040 != -1) { // branch-flip
    if (0 <= dat_ddd68)
      dat_ddd68 = dat_dd040 + dat_ddd68;
  }
  else {
    dat_ddd68 = -1;
    dat_e1698 = 0;
    dat_dd048 = 0;
    dat_dd040 = 0;
  }
  if (dat_dd048 != -1) { // branch-flip
    if (0 <= dat_ddd70)
      dat_ddd70 = dat_dd048 + dat_ddd70;
  }
  else {
    dat_ddd70 = -1;
    dat_e1698 = 0;
    dat_dd048 = 0;
    dat_dd040 = 0;
  }
  sub_f7a6(dat_dd048 - (dat_dd040 - dat_e1698),dat_dd048,stdout);
  printf(" %s\n",0xdd460);
}


// Function: sub_83ea @ 0x83ea
void sub_83ea(char *a0) // return-dupe
{
  char *v1;
  int v2; // eax
  char *v3; // rax
  int v4; // stack - 0x24
  char *v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  v5 = NULL;
  v4 = 3;
  v2 = (int)strlen(a0);
  if (dat_dd880) {
    if (2 > v2)
      sub_f5ef("name too short"); // no-return
    a0[(long)v2 + -1] = '\0';
    return;
  }
  v3 = (char *)sub_6a54(a0);
  if (v3) {
    *v3 = 0;
    dat_dd01c = 1;
    if ((5 <= v2) && (!strcmp(&v3[-4],".tar"))) {
      strcpy(&v3[-4],".tgz");
      return;
    }
    do {
      v6 = (char *)sub_10244(a0);
      while (*v6) {
        v2 = (int)strcspn(v6,".");
        v6 = &v6[v2];
        if (v4 < v2)
          v5 = &v6[-1];
        if (*v6)
          v6 = &v6[1];
      }
    } while ((!v5) && (v4 = v4 + -1, v4));
    v1 = v5;
    if (v5) { // branch-flip
      do {
        v5 = v1;
        *v5 = v5[1];
        v1 = &v5[1];
      } while (*v5);
    }
    else {
      v5 = strrchr(a0,0x2e);
      if (!v5)
        sub_f5ef("internal error in shorten_name"); // no-return
      if (!v5[1])
        v5 = &v5[-1];
    }
    strcpy(v5,dat_dd8b0);
    return;
  }
  sub_f5ef("can\'t recover suffix\n"); // no-return
}


// Function: sub_85c8 @ 0x85c8
unsigned long sub_85c8(void) // return-dupe
{
  char v1;
  
  if (!dat_dd884) {
    v1 = '\0';
    fprintf(stderr,"%s: %s already exists;",dat_dd010,(char *)0xdd460);
    if ((dat_dd898) && ((dat_dd878 || (isatty(0))))) {
      fprintf(stderr," do you wish to overwrite (y or n)? ");
      sub_109d7(stderr);
      v1 = sub_123a8();
    }
    if (!v1) {
      fprintf(stderr,"\tnot overwritten\n");
      if (dat_dd018)
        return 1;
      dat_dd018 = 2;
      return 1;
    }
  }
  if (!sub_f430((char *)0xdd460))
    return 0;
  sub_51e7((char *)0xdd460);
  return 1;
}


// Function: sub_86dc @ 0x86dc
void sub_86dc(int a0,unsigned long a1,unsigned int a2,unsigned int a3)
{
  fchown(a0,a2,a3);
}


// Function: sub_870d @ 0x870d
void sub_870d(long a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_870d
{
  unsigned int v1;
  bool v2; // al
  int v3;
  unsigned long v4; // stack - 0x38
  char v5 [16];
  unsigned long v6; // stack - 0x30
  long v7; // stack - 0x28
  long v8; // stack - 0x20
  
  v1 = *(unsigned int *)(a0 + 0x18);
  v5 = sub_1105b(a0);
  v6 = SUB168(v5,8);
  v4 = SUB168(v5,0);
  v5 = sub_1108f(a0);
  v8 = SUB168(v5,8);
  v7 = SUB168(v5,0);
  v2 = (((dat_dd880) && (0 <= dat_dd038)) && ((v7 != dat_dd030 || (v8 != dat_dd038))));
  if (v2) {
    v7 = dat_dd030;
    v8 = dat_dd038;
  }
  v3 = sub_113ca(dat_dd864,(char *)0xdd460,&v4);
  if (v3) { // branch-flip
    v3 = *__errno_location();
    if (!dat_dd004)
      fprintf(stderr,"%s: ",dat_dd010);
    if (!dat_dd018)
      dat_dd018 = 2;
    if (!dat_dd004) {
      *__errno_location() = v3;
      perror((char *)0xdd460);
    }
  }
  else {
    if ((!v2) || (dat_dd894 <= 1)) goto label_8897;
    fprintf(stderr,"%s: timestamp restored\n",(char *)0xdd460);
  }
label_8897:
  sub_86dc(dat_dd864,0xdd460,0xffffffff,*(unsigned int *)(a0 + 0x20));
  if (fchmod(dat_dd864,v1 & 0x1ff)) {
    v3 = *__errno_location();
    if (!dat_dd004)
      fprintf(stderr,"%s: ",dat_dd010);
    if (!dat_dd018)
      dat_dd018 = 2;
    if (!dat_dd004) {
      *__errno_location() = v3;
      perror((char *)0xdd460);
    }
  }
  sub_86dc(dat_dd864,0xdd460,*(unsigned int *)(a0 + 0x1c),0xffffffff);
}


// Function: sub_897b @ 0x897b
void sub_897b(int a0,char *a1) // return-dupe x2
{
  DIR *v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char v5 [1032];
  char *v6; // stack - 0x440
  unsigned long v7; // stack - 0x438
  unsigned long v8; // stack - 0x420
  
  v1 = fdopendir(a0);
  if (!v1) {
    sub_51e7(a1);
    close(a0);
    return;
  }
  v2 = (char *)sub_10c61(v1,0);
  if (!v2)
    sub_51e7(a1);
  if (closedir(v1))
    sub_51e7(a1);
  if (!v2)
    return;
  for (v6 = v2; *v6; v6 = &v6[v8 + 1]) {
    v3 = strlen(a1);
    v7 = v3;
    v4 = strlen(v6);
    v8 = v4;
    if ((strcmp(v6,".")) && (strcmp(v6,".."))) {
      if (0x3fe <= v4 + v3) { // branch-flip
        fprintf(stderr,"%s: %s/%s: pathname too long\n",dat_dd010,a1,v6);
        dat_dd018 = 1;
      }
      else {
        strcpy(v5,a1);
        if ((*(char *)sub_10244(v5)) && (v5[v3 - 1] != '/')) {
          v7 = v3 + 1;
          v5[v3] = '/';
        }
        strcpy(&v5[v7],v6);
        sub_5f11(v5);
      }
    }
  }
  free(v2);
}


// Function: sub_8c07 @ 0x8c07
void sub_8c07(void)
{
  int v1;
  void *v2 [17]; // stack - 0xa8
  int v3; // stack - 0xb0
  undefined4 v4; // stack - 0x20
  
  sigemptyset((sigset_t *)0xdd8c0);
  for (v3 = 0; v3 < 6; v3 = v3 + 1) {
    sigaction(*(int *)((long)v3 * 4 + 0x190b0),NULL,(sigaction *)v2);
    if (v2[0] != (void *)0x1)
      sigaddset((sigset_t *)0xdd8c0,*(int *)((long)v3 * 4 + 0x190b0));
  }
  v2[0] = sub_8fbb;
  v2[1] = (void *)dat_dd8c0;
  v2[2] = (void *)dat_dd8c8;
  v2[3] = (void *)dat_dd8d0;
  v2[4] = (void *)dat_dd8d8;
  v2[5] = (void *)dat_dd8e0;
  v2[6] = (void *)dat_dd8e8;
  v2[7] = (void *)dat_dd8f0;
  v2[8] = (void *)dat_dd8f8;
  v2[9] = (void *)dat_dd900;
  v2[10] = (void *)dat_dd908;
  v2[0xb] = (void *)dat_dd910;
  v2[0xc] = (void *)dat_dd918;
  v2[0xd] = (void *)dat_dd920;
  v2[0xe] = (void *)dat_dd928;
  v2[0xf] = (void *)dat_dd930;
  v2[0x10] = (void *)dat_dd938;
  v4 = 0;
  for (v3 = 0; v3 < 6; v3 = v3 + 1) {
    v1 = *(int *)((long)v3 * 4 + 0x190b0);
    if (sigismember((void *)0xdd8c0,v1)) {
      if (!v3)
        dat_dd898 = 1;
      sigaction(*(int *)((long)v3 * 4 + 0x190b0),(sigaction *)v2,NULL);
    }
  }
}


// Function: sub_8e40 @ 0x8e40
void sub_8e40(int a0)
{
  if (dat_de210)
    exit(a0); // no-return
  dat_de210 = 1;
  free(dat_dd8a8);
  dat_dd8a8 = NULL;
  exit(a0); // no-return
}


// Function: sub_8e91 @ 0x8e91
void sub_8e91(void)
{
  int v1; // eax
  
  v1 = sub_10304(stdout);
  if (v1)
    sub_f74d(); // no-return
  sub_8e40(0); // no-return
}


// Function: sub_8ebb @ 0x8ebb
void sub_8ebb(char a0) // return-dupe
{
  int v1;
  sigset_t v2;
  char v3 [1032];
  
  if (a0 != '\x01')
    sigprocmask(0,(sigset_t *)0xdd8c0,&v2);
  v1 = dat_190a4;
  if (0 <= dat_190a4) {
    dat_190a4 = -1;
    close(v1);
    sub_67dc(v3,(char *)0xdd960);
    sub_f430(v3);
  }
  if (a0 == '\x01')
    return;
  sigprocmask(2,&v2,NULL);
}


// Function: sub_8f9f @ 0x8f9f
void sub_8f9f(void)
{
  sub_8ebb(0);
  sub_8e40(1); // no-return
}


// Function: sub_8fbb @ 0x8fbb
void sub_8fbb(int a0)
{
  sub_8ebb(1);
  if (a0 == dat_dd940)
    _exit(2); // no-return
  signal(a0,0);
  raise(a0);
}


// Function: sub_9005 @ 0x9005
unsigned long sub_9005(unsigned int *a0,unsigned int a1,unsigned int a2,long a3,long a4,long *a5,unsigned int *a6) // return-dupe x4, ternary
{
  unsigned long *v1;
  unsigned long v10; // rbx
  unsigned long v11;
  unsigned int v12 [20]; // stack - 0x5e8
  unsigned int v13 [20]; // stack - 0x598
  unsigned int v14 [290];
  long v15 [16]; // stack - 0x548
  unsigned int v16;
  unsigned int v17;
  unsigned int v18;
  unsigned long v19; // r12
  int v2;
  unsigned int *v20;
  unsigned int *v21;
  long v22; // r14
  int v23; // r15d
  int v24; // stack - 0x65c
  long *v25; // stack - 0x650
  unsigned int v26; // stack - 0x61c
  int v27; // stack - 0x618
  int v28; // stack - 0x614
  unsigned int v29; // stack - 0x610
  unsigned int v3;
  int v30; // stack - 0x60c
  unsigned int v31; // stack - 0x608
  unsigned int *v32; // stack - 0x600
  char v33; // stack - 0x5f8
  undefined6 v34; // stack - 0x5f6
  long v35; // stack - 0x5f0
  char v4;
  unsigned char v5;
  unsigned int v6; // eax
  void *v7; // rax
  unsigned int v8;
  unsigned int v9; // eax
  
  memset(v12,0,0x44);
  v21 = a0;
  v16 = a1;
  do {
    v12[*v21] = v12[*v21] + 1;
    v21 = &v21[1];
    v16 -= 1;
  } while (v16);
  if (a1 == v12[0]) {
    v7 = malloc(0x30);
    if (v7) {
      *(unsigned long *)((long)v7 + 8) = 0;
      *(char *)((long)v7 + 0x10) = 99;
      *(char *)((long)v7 + 0x11) = 1;
      *(char *)((long)v7 + 0x20) = 99;
      *(char *)((long)v7 + 0x21) = 1;
      *a5 = (long)v7 + 0x10;
      *a6 = 1;
      dat_de224 += 3;
      return 0;
    }
    return 3;
  }
  v10 = 1;
  while ((v16 = (unsigned int)v10, v16 <= 0x10 && (!v12[v10]))) {
    v10 = (unsigned long)(v16 + 1);
  }
  v29 = *a6;
  if (*a6 < v16)
    v29 = v16;
  v19 = 0x10;
  while ((v16 = (unsigned int)v19, v16 && (!v12[v19]))) {
    v19 = (unsigned long)(v16 - 1);
  }
  if (v16 < v29)
    v29 = v16;
  *a6 = v29;
  v30 = 1 << ((unsigned char)v10 & 0x1f);
  for (v11 = v10; (unsigned int)v11 < v16; v11 = (unsigned long)((unsigned int)v11 + 1)) {
    if ((int)(v30 - v12[v11]) <= -1)
      return 2;
    v30 = (v30 - v12[v11]) * 2;
  }
  v30 -= v12[v19];
  if (v30 <= -1)
    return 2;
  v12[v19] = v30 + v12[v19];
  v18 = 0;
  v13[1] = 0;
  v21 = v12;
  v32 = &v13[2];
  while( true ) {
    v21 = &v21[1];
    v17 = (int)v19 - 1;
    v19 = (unsigned long)v17;
    if (!v17) break;
    v18 += *v21;
    v20 = &v32[1];
    *v32 = v18;
    v32 = v20;
  }
  v18 = 0;
  do {
    v21 = &a0[1];
    v17 = *a0;
    if (v17) {
      v3 = v13[v17];
      v13[v17] = v3 + 1;
      v14[v3] = v18;
    }
    v18 += 1;
    a0 = v21;
  } while (v18 < a1);
  v18 = v13[(int)v16];
  v17 = 0;
  v13[0] = 0;
  v21 = v14;
  v28 = -1;
  v24 = -v29;
  v15[0] = 0;
  v22 = 0;
  v31 = 0;
  v25 = a5;
  do {
    v23 = (int)v10;
    if ((int)v16 < v23) {
      if ((v30) && (v16 != 1))
        return 1;
      return 0;
    }
    v26 = v12[v23];
    while( true ) {
      v3 = v26 - 1;
      v2 = v28;
      if (!v26) break;
      while (v28 = v2, (int)(v29 + v24) < v23) {
        v2 = v28 + 1;
        v24 = v29 + v24;
        v8 = v16 - v24;
        if (v29 < v8)
          v8 = v29;
        v9 = v23 - v24;
        v6 = 1 << ((unsigned char)v9 & 0x1f);
        if (v26 < v6) {
          v27 = (v6 - v3) + -1;
          v32 = &v12[v23];
          if (v9 < v8) {
            while (v9 = v9 + 1, v9 < v8) {
              v32 = &v32[1];
              if ((unsigned int)(v27 * 2) <= *v32) break;
              v27 = v27 * 2 - *v32;
            }
          }
        }
        v31 = 1 << ((unsigned char)v9 & 0x1f);
        v7 = malloc((unsigned long)(v31 + 1) << 4);
        if (!v7) {
          if (v2)
            sub_975b(v15[0]);
          return 3;
        }
        dat_de224 = v31 + dat_de224 + 1;
        *v25 = (long)v7 + 0x10;
        v25 = (long *)((long)v7 + 8);
        *v25 = 0;
        v22 = (long)v7 + 0x10;
        v15[v2] = v22;
        if (v2) {
          v13[v2] = v17;
          v1 = (unsigned long *)(v15[v28] + (unsigned long)(v17 >> ((char)v24 - (char)v29 & 0x1fU)) * 0x10);
          *v1 = CONCAT62(v34,CONCAT11((char)v29,(unsigned char)v9 + 0x10));
          v1[1] = v22;
          v35 = v22;
        }
      }
      v5 = (unsigned char)v24;
      v4 = (char)v10;
      if (&v14[v18] <= v21) { // branch-flip
        v33 = 99;
        v20 = v21;
      }
      else if (a2 <= *v21) { // branch-flip
        v33 = (char)*(unsigned short *)(a4 + (unsigned long)(*v21 - a2) * 2);
        v20 = &v21[1];
        v35 = CONCAT62(v35._2_6_,*(unsigned short *)(a3 + (unsigned long)(*v21 - a2) * 2));
      }
      else {
        v33 = (0x100 <= *v21) ? 0xf : 0x10; // branch-flip
        v35 = CONCAT62(v35._2_6_,(short)*v21);
        v20 = &v21[1];
      }
      for (v8 = v17 >> (v5 & 0x1f); v8 < v31; v8 = (1 << (v4 - v5 & 0x1f)) + v8) {
        v1 = (unsigned long *)(v22 + (unsigned long)v8 * 0x10);
        *v1 = CONCAT62(v34,CONCAT11(v4 - v5,v33));
        v1[1] = v35;
      }
      v8 = 1 << (v4 - 1U & 0x1fU);
      while (v17 & v8) {
        v9 = v8 >> 1;
        v17 ^= v8;
        v8 = v9;
      }
      v17 ^= v8;
      while( true ) {
        v21 = v20;
        v26 = v3;
        if (((1 << ((unsigned char)v24 & 0x1f)) - 1U & v17) == v13[v28]) break;
        v24 -= v29;
        v28 -= 1;
      }
    }
    v10 = (unsigned long)(v23 + 1);
  } while( true );
}


// Function: sub_975b @ 0x975b
unsigned long sub_975b(long a0)
{
  void *v1; // rbx
  
  while (a0) {
    v1 = (void *)(a0 + -0x10);
    a0 = *(long *)(a0 + -8);
    free(v1);
  }
  return 0;
}


// Function: sub_979a @ 0x979a
unsigned long sub_979a(long a0,long a1,unsigned int a2,unsigned int a3) // return-dupe x2, ternary
{
  unsigned short v1;
  unsigned int v10; // stack - 0x44
  unsigned int v11; // stack - 0x40
  unsigned int v12; // stack - 0x3c
  unsigned char *v13; // stack - 0x30
  unsigned short v2;
  unsigned char v3;
  unsigned int v4;
  unsigned int v5;
  unsigned int v6;
  unsigned int v7; // r12d
  unsigned long v8; // r12
  unsigned long v9;
  
  v12 = dat_dd870;
  v1 = *(unsigned short *)((long)(int)a2 * 2 + 0x192e0);
  v2 = *(unsigned short *)((long)(int)a3 * 2 + 0x192e0);
  v9 = dat_de218;
  v5 = dat_de220;
  while( true ) {
    while( true ) {
      for (; v5 < a2; v5 = v5 + 8) {
        if (dat_dd868 <= dat_dd86c) { // branch-flip
          dat_dd870 = v12;
          v3 = sub_f0bc(0);
        }
        else {
          v6 = dat_dd86c + 1;
          v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
          dat_dd86c = v6;
        }
        v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
      }
      v13 = (unsigned char *)(a0 + (unsigned long)((unsigned int)v9 & (unsigned int)v1) * 0x10);
      v6 = (unsigned int)*v13;
      if (0x11 <= *v13) {
        do {
          if (v6 == 99)
            return 1;
          v9 >>= v13[1] & 0x3f;
          for (v5 = v5 - v13[1]; v5 < v6 - 0x10; v5 = v5 + 8) {
            if (dat_dd868 <= dat_dd86c) { // branch-flip
              dat_dd870 = v12;
              v3 = sub_f0bc(0);
            }
            else {
              v4 = dat_dd86c + 1;
              v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
              dat_dd86c = v4;
            }
            v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
          }
          v13 = (unsigned char *)((unsigned long)((unsigned int)*(unsigned short *)((unsigned long)(v6 - 0x10) * 2 + 0x192e0) & (unsigned int)v9) * 0x10 + *(long *)&v13[8]);
          v6 = (unsigned int)*v13;
        } while (0x10 < v6);
      }
      v9 >>= v13[1] & 0x3f;
      v5 -= v13[1];
      if (v6 != 0x10) break;
      v6 = v12 + 1;
      *(char *)((unsigned long)v12 + 0xad000) = (char)*(unsigned short *)&v13[8];
      v12 = v6;
      if (v6 == 0x8000) {
        dat_dd870 = v6;
        sub_f2cc();
        v12 = 0;
      }
    }
    if (v6 == 0xf) break;
    for (; v5 < v6; v5 = v5 + 8) {
      if (dat_dd868 <= dat_dd86c) { // branch-flip
        dat_dd870 = v12;
        v3 = sub_f0bc(0);
      }
      else {
        v4 = dat_dd86c + 1;
        v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v4;
      }
      v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
    }
    v10 = ((unsigned int)*(unsigned short *)((unsigned long)v6 * 2 + 0x192e0) & (unsigned int)v9) + (unsigned int)*(unsigned short *)&v13[8];
    v9 >>= (unsigned char)v6 & 0x3f;
    for (v5 = v5 - v6; v5 < a3; v5 = v5 + 8) {
      if (dat_dd868 <= dat_dd86c) { // branch-flip
        dat_dd870 = v12;
        v3 = sub_f0bc(0);
      }
      else {
        v6 = dat_dd86c + 1;
        v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v6;
      }
      v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
    }
    v13 = (unsigned char *)(a1 + (unsigned long)((unsigned int)v9 & (unsigned int)v2) * 0x10);
    v6 = (unsigned int)*v13;
    if (0x11 <= *v13) {
      do {
        if (v6 == 99)
          return 1;
        v9 >>= v13[1] & 0x3f;
        for (v5 = v5 - v13[1]; v5 < v6 - 0x10; v5 = v5 + 8) {
          if (dat_dd868 <= dat_dd86c) { // branch-flip
            dat_dd870 = v12;
            v3 = sub_f0bc(0);
          }
          else {
            v4 = dat_dd86c + 1;
            v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
            dat_dd86c = v4;
          }
          v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
        }
        v13 = (unsigned char *)((unsigned long)((unsigned int)*(unsigned short *)((unsigned long)(v6 - 0x10) * 2 + 0x192e0) & (unsigned int)v9) * 0x10 + *(long *)&v13[8]);
        v6 = (unsigned int)*v13;
      } while (0x10 < v6);
    }
    v9 >>= v13[1] & 0x3f;
    for (v5 = v5 - v13[1]; v5 < v6; v5 = v5 + 8) {
      if (dat_dd868 <= dat_dd86c) { // branch-flip
        dat_dd870 = v12;
        v3 = sub_f0bc(0);
      }
      else {
        v4 = dat_dd86c + 1;
        v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v4;
      }
      v9 |= (unsigned long)v3 << ((unsigned char)v5 & 0x3f);
    }
    v11 = (v12 - *(unsigned short *)&v13[8]) - ((unsigned int)v9 & (unsigned int)*(unsigned short *)((unsigned long)v6 * 2 + 0x192e0));
    v9 >>= (unsigned char)v6 & 0x3f;
    v5 -= v6;
    do {
      v11 &= 0x7fff;
      if (v11 <= v12) // branch-flip
        v6 = v12;
      else {
        v6 = v11;
      }
      v6 = 0x8000 - v6;
      if (v10 < v6)
        v6 = v10;
      v8 = (unsigned long)v6;
      v10 -= v6;
      v4 = (v12 <= v11) ? v11 - v12 : v12 - v11; // branch-flip
      if (v6 <= v4) { // branch-flip
        memcpy((void *)((unsigned long)v12 + 0xad000),(void *)((unsigned long)v11 + 0xad000),v8);
        v12 += v6;
        v11 += v6;
      }
      else {
        do {
          v6 = v11 + 1;
          v4 = v12 + 1;
          *(char *)((unsigned long)v12 + 0xad000) = *(char *)((unsigned long)v11 + 0xad000);
          v7 = (int)v8 - 1;
          v8 = (unsigned long)v7;
          v11 = v6;
          v12 = v4;
        } while (v7);
      }
      if (v12 == 0x8000) {
        dat_dd870 = v12;
        sub_f2cc();
        v12 = 0;
      }
    } while (v10);
  }
  dat_dd870 = v12;
  dat_de218 = v9;
  dat_de220 = v5;
  return 0;
}


// Function: sub_9d74 @ 0x9d74
unsigned long sub_9d74(void) // return-dupe
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  unsigned int v4; // ebx
  unsigned long v5; // r12
  unsigned int v6; // stack - 0x20
  unsigned int v7; // stack - 0x1c
  
  v2 = dat_dd870;
  v7 = dat_dd870;
  v5 = dat_de218 >> (signed char)(dat_de220 & 7);
  for (v4 = dat_de220 - (dat_de220 & 7); v4 <= 0xf; v4 = v4 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v2;
      v3 = sub_f0bc(0);
    }
    else {
      v1 = dat_dd86c + 1;
      v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    v5 |= (unsigned long)v3 << ((unsigned char)v4 & 0x3f);
  }
  v6 = (unsigned int)v5 & 0xffff;
  v5 >>= 0x10;
  for (v4 = v4 - 0x10; v4 <= 0xf; v4 = v4 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v2;
      v3 = sub_f0bc(0);
    }
    else {
      v1 = dat_dd86c + 1;
      v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    v5 |= (unsigned long)v3 << ((unsigned char)v4 & 0x3f);
  }
  if (v6 != (~(unsigned int)v5 & 0xffff))
    return 1;
  v5 >>= 0x10;
  v4 -= 0x10;
  while (v2 = v6 - 1, v6) {
    for (; v4 <= 7; v4 = v4 + 8) {
      if (dat_dd868 <= dat_dd86c) { // branch-flip
        dat_dd870 = v7;
        v3 = sub_f0bc(0);
      }
      else {
        v1 = dat_dd86c + 1;
        v3 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      v5 |= (unsigned long)v3 << ((unsigned char)v4 & 0x3f);
    }
    dat_dd870 = v7 + 1;
    *(char *)((unsigned long)v7 + 0xad000) = (char)v5;
    v7 = dat_dd870;
    if (dat_dd870 == 0x8000) {
      sub_f2cc();
      v7 = 0;
    }
    v5 >>= 8;
    v4 -= 8;
    v6 = v2;
  }
  dat_dd870 = v7;
  dat_de218 = v5;
  dat_de220 = v4;
  return 0;
}


// Function: sub_9f58 @ 0x9f58
int sub_9f58(void) // early-return x2
{
  int v1; // eax
  long v2; // rax
  unsigned int v3 [144];
  unsigned long v4; // stack - 0x4a8
  unsigned int v5; // stack - 0x4b4
  unsigned long v6; // stack - 0x4a0
  unsigned int v7; // stack - 0x4b0
  int v8; // stack - 0x4ac
  
  for (v8 = 0; v8 <= 0x8f; v8 = v8 + 1) {
    v2 = (long)v8;
    v3[v2] = 8;
  }
  for (; v8 <= 0xff; v8 = v8 + 1) {
    v2 = (long)v8;
    v3[v2] = 9;
  }
  for (; v8 <= 0x117; v8 = v8 + 1) {
    v2 = (long)v8;
    v3[v2] = 7;
  }
  for (; v8 <= 0x11f; v8 = v8 + 1) {
    v2 = (long)v8;
    v3[v2] = 8;
  }
  v5 = 7;
  v1 = sub_9005(v3,0x120,0x101,0x191e0,0x19220,&v4,&v5);
  if (!v1) {
    for (v8 = 0; v8 <= 0x1d; v8 = v8 + 1) {
      v2 = (long)v8;
      v3[v2] = 5;
    }
    v7 = 5;
    v1 = sub_9005(v3,0x1e,0,0x19260,0x192a0,&v6,&v7);
    v8 = v1;
    if (v1 <= 1) {
      v1 = sub_979a(v4,v6,v5,v7);
      if (!v1) {
        sub_975b(v4);
        sub_975b(v6);
        v1 = 0;
        return v1;
      }
      v1 = 1;
      return v1;
    }
    sub_975b(v4);
    v1 = v8;
  }
  return v1;
}


// Function: sub_a191 @ 0xa191
unsigned int sub_a191(void) // early-return x2
{
  unsigned int v1;
  unsigned long v10;
  unsigned int v11; // stack - 0x550
  unsigned int v12; // stack - 0x54c
  unsigned int v13; // stack - 0x548
  unsigned int v14; // stack - 0x544
  unsigned int v15; // stack - 0x540
  unsigned int v16; // stack - 0x53c
  unsigned int v17; // stack - 0x538
  unsigned int v18; // stack - 0x534
  unsigned int v19; // stack - 0x530
  unsigned char v2;
  unsigned int v20; // stack - 0x52c
  int v3;
  unsigned int v4;
  unsigned int v5 [318];
  long v6; // stack - 0x528
  unsigned int v7; // stack - 0x558
  char *v8; // stack - 0x520
  unsigned int v9; // stack - 0x554
  
  v14 = dat_dd870;
  v10 = dat_de218;
  for (v4 = dat_de220; v4 <= 4; v4 = v4 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v14;
      v2 = sub_f0bc(0);
    }
    else {
      v1 = dat_dd86c + 1;
      v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
  }
  v15 = ((unsigned int)v10 & 0x1f) + 0x101;
  v10 >>= 5;
  for (v4 = v4 - 5; v4 <= 4; v4 = v4 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v14;
      v2 = sub_f0bc(0);
    }
    else {
      v1 = dat_dd86c + 1;
      v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
  }
  v16 = ((unsigned int)v10 & 0x1f) + 1;
  v10 >>= 5;
  for (v4 = v4 - 5; v4 <= 3; v4 = v4 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v14;
      v2 = sub_f0bc(0);
    }
    else {
      v1 = dat_dd86c + 1;
      v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
  }
  v17 = ((unsigned int)v10 & 0xf) + 4;
  v10 >>= 4;
  v4 -= 4;
  if ((0x11f <= v15) || (0x1f <= v16))
    return 1;
  for (v12 = 0; v12 < v17; v12 = v12 + 1) {
    for (; v4 <= 2; v4 = v4 + 8) {
      if (dat_dd868 <= dat_dd86c) { // branch-flip
        dat_dd870 = v14;
        v2 = sub_f0bc(0);
      }
      else {
        v1 = dat_dd86c + 1;
        v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
    }
    v5[*(unsigned int *)((unsigned long)v12 * 4 + 0x19180)] = (unsigned int)v10 & 7;
    v10 >>= 3;
    v4 -= 3;
  }
  for (; v12 <= 0x12; v12 = v12 + 1) {
    v5[*(unsigned int *)((unsigned long)v12 * 4 + 0x19180)] = 0;
  }
  v7 = 7;
  v11 = sub_9005(v5,0x13,0x13,0,0,&v6,&v7);
  if (v11) { // branch-flip
    if (v11 == 1)
      sub_975b(v6);
  }
  else {
    if (!v6)
      return 2;
    v18 = v16 + v15;
    v19 = (unsigned int)*(unsigned short *)((long)(int)v7 * 2 + 0x192e0);
    v13 = 0;
    v11 = 0;
    while (v11 < v18) {
      for (; v4 < v7; v4 = v4 + 8) {
        if (dat_dd868 <= dat_dd86c) { // branch-flip
          dat_dd870 = v14;
          v2 = sub_f0bc(0);
        }
        else {
          v1 = dat_dd86c + 1;
          v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
          dat_dd86c = v1;
        }
        v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
      }
      v8 = (char *)(v6 + (unsigned long)((unsigned int)v10 & v19) * 0x10);
      v12 = (unsigned int)(unsigned char)v8[1];
      v10 >>= v8[1] & 0x3fU;
      v4 -= v12;
      if (*v8 == 'c') {
        sub_975b(v6);
        return 2;
      }
      v12 = (unsigned int)*(unsigned short *)&v8[8];
      if (0x10 <= v12) { // branch-flip
        if (v12 != 0x10) { // branch-flip
          if (v12 != 0x11) { // branch-flip
            for (; v4 <= 6; v4 = v4 + 8) {
              if (dat_dd868 <= dat_dd86c) { // branch-flip
                dat_dd870 = v14;
                v2 = sub_f0bc(0);
              }
              else {
                v1 = dat_dd86c + 1;
                v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
                dat_dd86c = v1;
              }
              v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
            }
            v12 = ((unsigned int)v10 & 0x7f) + 0xb;
            v10 >>= 7;
            v4 -= 7;
            if (v18 < v12 + v11)
              return 1;
            while (v3 = v12 + -1, v12) {
              v1 = v11 + 1;
              v5[(int)v11] = 0;
              v11 = v1;
              v12 = v3;
            }
            v13 = 0;
            v12 = 0xffffffff;
          }
          else {
            v12 = 0x11;
            for (; v4 <= 2; v4 = v4 + 8) {
              if (dat_dd868 <= dat_dd86c) { // branch-flip
                dat_dd870 = v14;
                v2 = sub_f0bc(0);
              }
              else {
                v1 = dat_dd86c + 1;
                v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
                dat_dd86c = v1;
              }
              v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
            }
            v12 = ((unsigned int)v10 & 7) + 3;
            v10 >>= 3;
            v4 -= 3;
            if (v18 < v12 + v11)
              return 1;
            while (v3 = v12 + -1, v12) {
              v1 = v11 + 1;
              v5[(int)v11] = 0;
              v11 = v1;
              v12 = v3;
            }
            v13 = 0;
            v12 = 0xffffffff;
          }
        }
        else {
          v12 = 0x10;
          for (; v4 < 2; v4 = v4 + 8) {
            if (dat_dd868 <= dat_dd86c) { // branch-flip
              dat_dd870 = v14;
              v2 = sub_f0bc(0);
            }
            else {
              v1 = dat_dd86c + 1;
              v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
              dat_dd86c = v1;
            }
            v10 |= (unsigned long)v2 << ((unsigned char)v4 & 0x3f);
          }
          v12 = ((unsigned int)v10 & 3) + 3;
          v10 >>= 2;
          v4 -= 2;
          if (v18 < v12 + v11)
            return 1;
          while (v3 = v12 + -1, v12) {
            v1 = v11 + 1;
            v5[(int)v11] = v13;
            v11 = v1;
            v12 = v3;
          }
          v12 = 0xffffffff;
        }
      }
      else {
        v1 = v11 + 1;
        v5[(int)v11] = v12;
        v11 = v1;
        v13 = v12;
      }
    }
    sub_975b(v6);
    v7 = dat_19304;
    dat_de218 = v10;
    dat_de220 = v4;
    v11 = sub_9005(v5,v15,0x101,0x191e0,0x19220,&v6,&v7);
    if (v11) { // branch-flip
      if (v11 == 1)
        sub_975b(v6);
    }
    else {
      v9 = dat_19308;
      v11 = sub_9005(&v5[v15],v16,0,0x19260,0x192a0,&v8,&v9);
      if (v11) { // branch-flip
        if (v11 == 1)
          sub_975b(v8);
        sub_975b(v6);
      }
      else {
        v3 = sub_979a(v6,v8,v7,v9);
        v20 = (unsigned int)(v3 != 0);
        sub_975b(v6);
        sub_975b(v8);
        v11 = v20;
      }
    }
  }
  return v11;
}


// Function: sub_aa58 @ 0xaa58
unsigned long sub_aa58(unsigned int *a0) // early-return
{
  unsigned int v1;
  unsigned char v2;
  unsigned int v3;
  unsigned long v4; // rax
  int v5;
  unsigned int v6;
  unsigned long v7;
  
  v1 = dat_dd870;
  v7 = dat_de218;
  v5 = dat_de220;
  while (!v5) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v1;
      v2 = sub_f0bc(0);
    }
    else {
      v6 = dat_dd86c + 1;
      v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v6;
    }
    v7 |= v2;
    v5 = 8;
  }
  *a0 = (unsigned int)v7 & 1;
  v7 >>= 1;
  for (v6 = v5 - 1; v6 < 2; v6 = v6 + 8) {
    if (dat_dd868 <= dat_dd86c) { // branch-flip
      dat_dd870 = v1;
      v2 = sub_f0bc(0);
    }
    else {
      v3 = dat_dd86c + 1;
      v2 = *(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v3;
    }
    v7 |= (unsigned long)v2 << ((unsigned char)v6 & 0x3f);
  }
  v3 = (unsigned int)v7 & 3;
  dat_de218 = v7 >> 2;
  dat_de220 = v6 - 2;
  if (v3 != 2) { // branch-flip
    if (v7 & 3) { // branch-flip
      if (v3 != 1)
        return 2;
      v4 = sub_9f58();
    }
    else {
      v4 = sub_9d74();
    }
  }
  else {
    v4 = sub_a191();
  }
  return v4;
}


// Function: sub_ab98 @ 0xab98
int sub_ab98(void)
{
  int v1;
  int v2; // stack - 0x1c
  unsigned int v3; // stack - 0x18
  int v4; // stack - 0x14
  
  dat_dd870 = 0;
  dat_de220 = 0;
  dat_de218 = 0;
  v3 = 0;
  v1 = v4;
  do {
    v4 = v1;
    dat_de224 = 0;
    v4 = sub_aa58(&v2);
    if (v4)
      return v4;
    if (v3 < dat_de224)
      v3 = dat_de224;
    v1 = 0;
  } while (!v2);
  for (; 7 < dat_de220; dat_de220 = dat_de220 - 8) {
    dat_dd86c -= 1;
  }
  sub_f2cc();
  return 0;
}


// Function: sub_ac6f @ 0xac6f
void sub_ac6f(unsigned long a0,unsigned long a1) // return-dupe
{
  int v1;
  int v2; // stack - 0x1c
  int v3; // stack - 0x18
  int v4; // stack - 0x14
  int v5; // stack - 0x10
  int v6; // stack - 0xc
  
  dat_e1168 = 0;
  dat_e1160 = 0;
  if (dat_df0c2) {
    dat_e1178 = a1;
    dat_e1170 = a0;
    dat_e1168 = 0;
    dat_e1160 = 0;
    return;
  }
  v4 = 0;
  for (v5 = 0; v5 <= 0x1b; v5 = v5 + 1) {
    *(int *)((long)v5 * 4 + 0xe0040) = v4;
    v2 = 0;
    while (v2 < 1 << ((unsigned char)*(unsigned int *)((long)v5 * 4 + 0x19320) & 0x1f)) {
      v1 = v4 + 1;
      *(char *)((long)v4 + 0xdfd40) = (char)v5;
      v2 += 1;
      v4 = v1;
    }
  }
  *(char *)((long)(v4 + -1) + 0xdfd40) = (char)v5;
  v6 = 0;
  for (v5 = 0; v5 <= 0xf; v5 = v5 + 1) {
    *(int *)((long)v5 * 4 + 0xe00c0) = v6;
    v2 = 0;
    while (v2 < 1 << ((unsigned char)*(unsigned int *)((long)v5 * 4 + 0x193a0) & 0x1f)) {
      v1 = v6 + 1;
      *(char *)((long)v6 + 0xdfe40) = (char)v5;
      v2 += 1;
      v6 = v1;
    }
  }
  v6 >>= 7;
  for (; v5 <= 0x1d; v5 = v5 + 1) {
    *(int *)((long)v5 * 4 + 0xe00c0) = v6 << 7;
    v2 = 0;
    while (v2 < 1 << ((char)*(unsigned int *)((long)v5 * 4 + 0x193a0) - 7U & 0x1fU)) {
      v1 = v6 + 1;
      *(char *)((long)(v6 + 0x100) + 0xdfe40) = (char)v5;
      v2 += 1;
      v6 = v1;
    }
  }
  for (v3 = 0; v3 <= 0xf; v3 = v3 + 1) {
    *(unsigned short *)((long)v3 * 2 + 0xdf1e0) = 0;
  }
  v2 = 0;
  while (v2 <= 0x8f) {
    v1 = v2 + 1;
    *(unsigned short *)((long)v2 * 4 + 0xdec42) = 8;
    dat_df1f0 += 1;
    v2 = v1;
  }
  while (v2 <= 0xff) {
    v1 = v2 + 1;
    *(unsigned short *)((long)v2 * 4 + 0xdec42) = 9;
    dat_df1f2 += 1;
    v2 = v1;
  }
  while (v2 <= 0x117) {
    v1 = v2 + 1;
    *(unsigned short *)((long)v2 * 4 + 0xdec42) = 7;
    dat_df1ee += 1;
    v2 = v1;
  }
  while (v2 <= 0x11f) {
    v1 = v2 + 1;
    *(unsigned short *)((long)v2 * 4 + 0xdec42) = 8;
    dat_df1f0 += 1;
    v2 = v1;
  }
  dat_e1170 = a0;
  dat_e1178 = a1;
  sub_b76a(0xdec40,0x11f);
  for (v2 = 0; v2 <= 0x1d; v2 = v2 + 1) {
    *(unsigned short *)((long)v2 * 4 + 0xdf0c2) = 5;
    *(unsigned short *)((long)v2 * 4 + 0xdf0c0) = sub_3bed(v2,5);
  }
  sub_aff6();
}


// Function: sub_aff6 @ 0xaff6
void sub_aff6(void)
{
  int v1; // stack - 0xc
  
  for (v1 = 0; v1 <= 0x11d; v1 = v1 + 1) {
    *(unsigned short *)((long)v1 * 4 + 0xde240) = 0;
  }
  for (v1 = 0; v1 <= 0x1d; v1 = v1 + 1) {
    *(unsigned short *)((long)v1 * 4 + 0xdeb40) = 0;
  }
  for (v1 = 0; v1 <= 0x12; v1 = v1 + 1) {
    *(unsigned short *)((long)v1 * 4 + 0xdf140) = 0;
  }
  dat_de640 = 1;
  dat_e1158 = 0;
  dat_e1150 = 0;
  dat_e1148 = 0;
  dat_e1144 = 0;
  dat_e1140 = 0;
  dat_e114c = 0;
  dat_e114d = 1;
}


// Function: sub_b0dd @ 0xb0dd
void sub_b0dd(long a0,int a1)
{
  int v1;
  int v2; // stack - 0x24
  int v3; // stack - 0x10
  
  v1 = *(int *)((long)a1 * 4 + 0xdf200);
  v2 = a1;
  for (v3 = a1 * 2; v3 <= dat_dfaf4; v3 = v3 << 1) {
    if ((v3 < dat_dfaf4) && ((*(unsigned short *)(a0 + (long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) * 4) < *(unsigned short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) || ((*(short *)(a0 + (long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) * 4) == *(short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) && (*(unsigned char *)((long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) + 0xdfb00) <= *(unsigned char *)((long)*(int *)((long)v3 * 4 + 0xdf200) + 0xdfb00)))))))
      v3 += 1;
    if ((*(unsigned short *)(a0 + (long)v1 * 4) < *(unsigned short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4)) || ((*(short *)(a0 + (long)v1 * 4) == *(short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) && (*(unsigned char *)((long)v1 + 0xdfb00) <= *(unsigned char *)((long)*(int *)((long)v3 * 4 + 0xdf200) + 0xdfb00))))) break;
    *(unsigned int *)((long)v2 * 4 + 0xdf200) = *(unsigned int *)((long)v3 * 4 + 0xdf200);
    v2 = v3;
  }
  *(int *)((long)v2 * 4 + 0xdf200) = v1;
}


// Function: sub_b363 @ 0xb363
void sub_b363(struct_0 *a0) // return-dupe
{
  unsigned short v1;
  int v10; // stack - 0x44
  unsigned int v11; // stack - 0x40
  unsigned int v12; // stack - 0x3c
  int v13; // stack - 0x38
  int v14; // stack - 0x34
  int v2;
  int v3;
  int v4;
  int v5;
  long v6;
  long v7;
  long v8;
  int v9;
  
  v6 = a0->field_0x0;
  v7 = a0->field_0x10;
  v2 = a0->field_0x18;
  v3 = a0->field_0x24;
  v4 = a0->field_0x20;
  v8 = a0->field_0x8;
  v14 = 0;
  for (v12 = 0; (int)v12 <= 0xf; v12 = v12 + 1) {
    *(unsigned short *)((long)(int)v12 * 2 + 0xdf1e0) = 0;
  }
  *(unsigned short *)(v6 + (long)*(int *)((long)dat_dfaf8 * 4 + 0xdf200) * 4 + 2) = 0;
  v10 = dat_dfaf8;
  while (v10 = v10 + 1, v10 <= 0x23c) {
    v5 = *(int *)((long)v10 * 4 + 0xdf200);
    v12 = *(unsigned short *)(v6 + (unsigned long)*(unsigned short *)(v6 + (long)v5 * 4 + 2) * 4 + 2) + 1;
    if (v4 < (int)v12) {
      v14 += 1;
      v12 = v4;
    }
    *(short *)(v6 + (long)v5 * 4 + 2) = (short)v12;
    if (v5 <= v3) {
      *(short *)((long)(int)v12 * 2 + 0xdf1e0) = *(short *)((long)(int)v12 * 2 + 0xdf1e0) + 1;
      v13 = 0;
      if (v2 <= v5)
        v13 = *(int *)(v7 + (long)(v5 - v2) * 4);
      v1 = *(unsigned short *)(v6 + (long)v5 * 4);
      dat_e1150 += (unsigned long)v1 * (long)(int)(v13 + v12);
      if (v8)
        dat_e1158 += (unsigned long)v1 * (long)(int)(v13 + (unsigned int)*(unsigned short *)(v8 + (long)v5 * 4 + 2));
    }
  }
  v12 = v4;
  if (!v14)
    return;
  do {
    do {
      v9 = v12;
      v12 = v9 - 1;
    } while (!*(short *)((long)(int)v12 * 2 + 0xdf1e0));
    *(short *)((long)(int)v12 * 2 + 0xdf1e0) = *(short *)((long)(int)v12 * 2 + 0xdf1e0) + -1;
    *(short *)((long)v9 * 2 + 0xdf1e0) = *(short *)((long)v9 * 2 + 0xdf1e0) + 2;
    *(short *)((long)v4 * 2 + 0xdf1e0) = *(short *)((long)v4 * 2 + 0xdf1e0) + -1;
    v14 -= 2;
    v12 = v4;
  } while (0 < v14);
  for (; v12; v12 = v12 - 1) {
    v11 = (unsigned int)*(unsigned short *)((long)(int)v12 * 2 + 0xdf1e0);
    while (v11) {
      v10 -= 1;
      v2 = *(int *)((long)v10 * 4 + 0xdf200);
      if (v2 <= v3) {
        if (*(unsigned short *)(v6 + (long)v2 * 4 + 2) != v12) {
          dat_e1150 += (unsigned long)*(unsigned short *)(v6 + (long)v2 * 4) * ((long)(int)v12 - (unsigned long)*(unsigned short *)(v6 + (long)v2 * 4 + 2));
          *(short *)(v6 + (long)v2 * 4 + 2) = (short)v12;
        }
        v11 -= 1;
      }
    }
  }
}


// Function: sub_b76a @ 0xb76a
void sub_b76a(long a0,int a1)
{
  short v1;
  int v2; // eax
  short v3 [20];
  short v4; // stack - 0x46
  int v5; // stack - 0x44
  int v6; // stack - 0x40
  
  v4 = 0;
  for (v5 = 1; v5 <= 0xf; v5 = v5 + 1) {
    v4 = (v4 + *(short *)((long)(v5 + -1) * 2 + 0xdf1e0)) * 2;
    v3[v5] = v4;
  }
  for (v6 = 0; v6 <= a1; v6 = v6 + 1) {
    v2 = (unsigned int)*(unsigned short *)(a0 + (long)v6 * 4 + 2);
    if (v2) {
      v1 = v3[v2];
      v3[v2] = v1 + 1;
      *(unsigned short *)(a0 + (long)v6 * 4) = sub_3bed((unsigned short)v1,v2);
    }
  }
}


// Function: sub_b868 @ 0xb868
void sub_b868(struct_1 *a0) // ternary
{
  char v1;
  long v2;
  int v3;
  long v4;
  int v5;
  int v6; // stack - 0x30
  int v7; // stack - 0x2c
  int v8; // stack - 0x28
  
  v2 = a0->field_0x0;
  v4 = a0->field_0x8;
  v8 = a0->field_0x1c;
  v7 = -1;
  dat_dfaf4 = 0;
  dat_dfaf8 = 0x23d;
  for (v6 = 0; v6 < v8; v6 = v6 + 1) {
    if (*(short *)(v2 + (long)v6 * 4)) { // branch-flip
      v7 = v6;
      dat_dfaf4 += 1;
      *(int *)((long)dat_dfaf4 * 4 + 0xdf200) = v6;
      *(char *)((long)v6 + 0xdfb00) = 0;
    }
    else {
      *(unsigned short *)(v2 + (long)v6 * 4 + 2) = 0;
    }
  }
  while (dat_dfaf4 <= 1) {
    if (2 <= v7) // branch-flip
      v5 = 0;
    else {
      v5 = v7 + 1;
      v7 = v5;
    }
    dat_dfaf4 += 1;
    *(int *)((long)dat_dfaf4 * 4 + 0xdf200) = v5;
    v5 = *(int *)((long)dat_dfaf4 * 4 + 0xdf200);
    *(unsigned short *)(v2 + (long)v5 * 4) = 1;
    *(char *)((long)v5 + 0xdfb00) = 0;
    dat_e1150 -= 1;
    if (v4)
      dat_e1158 -= (unsigned long)*(unsigned short *)(v4 + (long)v5 * 4 + 2);
  }
  a0->field_0x24 = v7;
  for (v6 = dat_dfaf4 / 2; 0 < v6; v6 = v6 + -1) {
    sub_b0dd(v2,v6);
  }
  do {
    v3 = dat_df204;
    v5 = dat_dfaf4 + -1;
    dat_df204 = *(int *)((long)dat_dfaf4 * 4 + 0xdf200);
    dat_dfaf4 = v5;
    sub_b0dd(v2,1);
    *(int *)((long)(dat_dfaf8 + -1) * 4 + 0xdf200) = v3;
    dat_dfaf8 -= 2;
    *(int *)((long)dat_dfaf8 * 4 + 0xdf200) = dat_df204;
    *(short *)(v2 + (long)v8 * 4) = *(short *)(v2 + (long)dat_df204 * 4) + *(short *)(v2 + (long)v3 * 4);
    v1 = (*(unsigned char *)((long)dat_df204 + 0xdfb00) <= *(unsigned char *)((long)v3 + 0xdfb00)) ? *(char *)((long)v3 + 0xdfb00) : *(char *)((long)dat_df204 + 0xdfb00); // branch-flip
    *(char *)((long)v8 + 0xdfb00) = v1 + '\x01';
    v4 = v2 + (long)dat_df204 * 4;
    *(short *)(v4 + 2) = (short)v8;
    *(unsigned short *)(v2 + (long)v3 * 4 + 2) = *(unsigned short *)(v4 + 2);
    v5 = v8 + 1;
    dat_df204 = v8;
    sub_b0dd(v2,1);
    v8 = v5;
  } while (1 < dat_dfaf4);
  dat_dfaf8 -= 1;
  *(int *)((long)dat_dfaf8 * 4 + 0xdf200) = dat_df204;
  sub_b363(a0);
  sub_b76a(v2,v7);
}


// Function: sub_bc5c @ 0xbc5c
void sub_bc5c(long a0,int a1) // ternary
{
  unsigned short v1;
  unsigned int v2; // eax
  int v3; // stack - 0x24
  unsigned int v4; // stack - 0x20
  unsigned int v5; // stack - 0x1c
  int v6; // stack - 0x18
  int v7; // stack - 0x14
  int v8; // stack - 0x10
  
  v4 = 0xffffffff;
  v1 = *(unsigned short *)(a0 + 2);
  v6 = 0;
  v7 = 7;
  v8 = 4;
  if (!v1) {
    v7 = 0x8a;
    v8 = 3;
  }
  *(unsigned short *)(a0 + ((long)a1 + 1) * 4 + 2) = 0xffff;
  v3 = 0;
  v5 = (unsigned int)v1;
  while (v3 <= a1) {
    v2 = (unsigned int)*(unsigned short *)(a0 + ((long)v3 + 1) * 4 + 2);
    v6 += 1;
    if ((v7 <= v6) || (v5 != v2)) {
      if (v8 <= v6) { // branch-flip
        if (v5) { // branch-flip
          if (v5 != v4)
            *(short *)((long)(int)v5 * 4 + 0xdf140) = *(short *)((long)(int)v5 * 4 + 0xdf140) + 1;
          dat_df180 += 1;
        }
        else {
          dat_df188 = (0xb <= v6) ? dat_df188 + 1 : dat_df184 + 1; // branch-flip
        }
      }
      else {
        *(short *)((long)(int)v5 * 4 + 0xdf140) = *(short *)((long)(int)v5 * 4 + 0xdf140) + (short)v6;
      }
      v6 = 0;
      v4 = v5;
      if (v2) { // branch-flip
        if (v5 != v2) { // branch-flip
          v7 = 7;
          v8 = 4;
        }
        else {
          v7 = 6;
          v8 = 3;
        }
      }
      else {
        v7 = 0x8a;
        v8 = 3;
      }
    }
    v3 += 1;
    v5 = v2;
  }
}


// Function: sub_be31 @ 0xbe31
void sub_be31(long a0,int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x24
  unsigned int v4; // stack - 0x20
  unsigned int v5; // stack - 0x1c
  int v6; // stack - 0x18
  int v7; // stack - 0x14
  int v8; // stack - 0x10
  
  v4 = 0xffffffff;
  v6 = 0;
  v7 = 7;
  v8 = 4;
  if (!(*(unsigned short *)(a0 + 2))) {
    v7 = 0x8a;
    v8 = 3;
  }
  v3 = 0;
  v5 = (unsigned int)*(unsigned short *)(a0 + 2);
  while (v3 <= a1) {
    v2 = (unsigned int)*(unsigned short *)(a0 + ((long)v3 + 1) * 4 + 2);
    v1 = v6 + 1;
    if ((v7 <= v1) || (v5 != v2)) {
      if (v8 <= v1) { // branch-flip
        if (v5) { // branch-flip
          if (v5 != v4) {
            sub_3a6c(*(unsigned short *)((long)(int)v5 * 4 + 0xdf140),*(unsigned short *)((long)(int)v5 * 4 + 0xdf142));
            v1 = v6;
          }
          v6 = v1;
          sub_3a6c(dat_df180,dat_df182);
          sub_3a6c(v6 + -3,2);
        }
        else if (0xb <= v1) { // branch-flip
          sub_3a6c(dat_df188,dat_df18a);
          sub_3a6c(v6 + -10,7);
        }
        else {
          sub_3a6c(dat_df184,dat_df186);
          sub_3a6c(v6 + -2,3);
        }
      }
      else {
        do {
          v6 = v1;
          sub_3a6c(*(unsigned short *)((long)(int)v5 * 4 + 0xdf140),*(unsigned short *)((long)(int)v5 * 4 + 0xdf142));
          v1 = v6 + -1;
        } while (v6 + -1);
      }
      v6 = 0;
      v4 = v5;
      v1 = v6;
      if (v2) { // branch-flip
        if (v5 != v2) { // branch-flip
          v7 = 7;
          v8 = 4;
        }
        else {
          v7 = 6;
          v8 = 3;
        }
      }
      else {
        v7 = 0x8a;
        v8 = 3;
      }
    }
    v6 = v1;
    v3 += 1;
    v5 = v2;
  }
}


// Function: sub_c06c @ 0xc06c
int sub_c06c(void)
{
  int v1; // stack - 0xc
  
  sub_bc5c(0xde240,dat_194c4);
  sub_bc5c(0xdeb40,dat_19504);
  sub_b868((struct_1 *)0x19520);
  v1 = 0x12;
  while ((2 < v1 && (!*(short *)((long)(int)(unsigned int)*(unsigned char *)((long)v1 + 0x19470) * 4 + 0xdf142)))) {
    v1 -= 1;
  }
  dat_e1150 += (v1 + 1) * 3 + 0xe;
  return v1;
}


// Function: sub_c120 @ 0xc120
void sub_c120(int a0,int a1,int a2)
{
  int v1; // stack - 0xc
  
  sub_3a6c(a0 + -0x101,5);
  sub_3a6c(a1 + -1,5);
  sub_3a6c(a2 + -4,4);
  for (v1 = 0; v1 < a2; v1 = v1 + 1) {
    sub_3a6c(*(unsigned short *)((long)(int)(unsigned int)*(unsigned char *)((long)v1 + 0x19470) * 4 + 0xdf142),3);
  }
  sub_be31(0xde240,a0 + -1);
  sub_be31(0xdeb40,a1 + -1);
}


// Function: sub_c1ea @ 0xc1ea
long sub_c1ea(char *a0,unsigned long a1,int a2,int a3)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  *(char *)((unsigned long)dat_e1148 + 0xe0140) = dat_e114c;
  if (*dat_e1170 == -1)
    sub_c97c();
  sub_b868((struct_1 *)0x194a0);
  sub_b868((struct_1 *)0x194e0);
  v1 = sub_c06c();
  v3 = dat_e1150 + 10U >> 3;
  v2 = dat_e1158 + 10U >> 3;
  dat_e1168 = a1 + dat_e1168;
  if (v2 <= v3)
    v3 = v2;
  if ((a1 + 4 <= v3) && (a0)) {
    sub_3a6c(a3,3);
    dat_e1160 = (dat_e1160 + 10 & 0xfffffffffffffff8) + (a1 + 4) * 8;
    sub_3d59(a0,a1 & 0xffffffff,1);
  }
  else if (v2 != v3) { // branch-flip
    sub_3a6c(a3 + 4,3);
    sub_c120(dat_194c4 + 1,dat_19504 + 1,v1 + 1);
    sub_c6f9(0xde240,0xdeb40);
    dat_e1160 = dat_e1160 + dat_e1150 + 3;
  }
  else {
    sub_3a6c(a3 + 2,3);
    sub_c6f9(0xdec40,0xdf0c0);
    dat_e1160 = dat_e1160 + dat_e1158 + 3;
  }
  sub_aff6();
  if (a3) { // branch-flip
    sub_3c22();
    dat_e1160 += 7;
  }
  else if ((a2) && (dat_e1160 & 7)) {
    sub_3a6c(0,3);
    dat_e1160 = dat_e1160 + 10 & 0xfffffffffffffff8;
    sub_3d59(a0,0,1);
  }
  return (long)dat_e1160 >> 3;
}


// Function: sub_c45d @ 0xc45d
unsigned long sub_c45d(int a0,int a1) // return-dupe, ternary
{
  unsigned int v1;
  unsigned char v2;
  int v3; // eax
  int v4; // stack - 0x1c
  unsigned long v5; // stack - 0x18
  
  v1 = dat_e1140;
  dat_e1140 += 1;
  *(char *)((unsigned long)v1 + 0x1b000) = (char)a1;
  if (a0) { // branch-flip
    a0 -= 1;
    v3 = *(unsigned char *)((long)a1 + 0xdfd40) + 0x101;
    *(short *)((long)v3 * 4 + 0xde240) = *(short *)((long)v3 * 4 + 0xde240) + 1;
    v2 = (0x100 <= a0) ? *(unsigned char *)((long)((a0 >> 7) + 0x100) + 0xdfe40) : *(unsigned char *)((long)a0 + 0xdfe40); // branch-flip
    *(short *)((long)(int)(unsigned int)v2 * 4 + 0xdeb40) = *(short *)((long)(int)(unsigned int)v2 * 4 + 0xdeb40) + 1;
    v1 = dat_e1144 + 1;
    *(short *)((unsigned long)dat_e1144 * 2 + 0x9c800) = (short)a0;
    dat_e114c = dat_e114d | dat_e114c;
    dat_e1144 = v1;
  }
  else {
    *(short *)((long)a1 * 4 + 0xde240) = *(short *)((long)a1 * 4 + 0xde240) + 1;
  }
  dat_e114d *= '\x02';
  if (!(dat_e1140 & 7)) {
    v1 = dat_e1148 + 1;
    *(unsigned char *)((unsigned long)dat_e1148 + 0xe0140) = dat_e114c;
    dat_e114c = 0;
    dat_e114d = '\x01';
    dat_e1148 = v1;
  }
  if ((3 <= dat_190a0) && (!(dat_e1140 & 0xfff))) {
    v5 = (unsigned long)dat_e1140 << 3;
    for (v4 = 0; v4 <= 0x1d; v4 = v4 + 1) {
      v5 += (unsigned long)*(unsigned short *)((long)v4 * 4 + 0xdeb40) * ((long)*(int *)((long)v4 * 4 + 0x193a0) + 5);
    }
    if ((dat_e1144 < dat_e1140 >> 1) && (v5 >> 3 < (unsigned long)dat_1a074 - dat_1a068 >> 1))
      return 1;
  }
  if ((dat_e1140 != 0x7fff) && (dat_e1144 != 0x8000))
    return 0;
  return 1;
}


// Function: sub_c6f9 @ 0xc6f9
void sub_c6f9(long a0,long a1) // ternary
{
  unsigned int v1;
  unsigned int v10; // stack - 0x20
  unsigned int v11; // stack - 0x1c
  unsigned char v2;
  unsigned short v3;
  int v4;
  int v5;
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned char v8; // stack - 0x25
  unsigned int v9; // stack - 0x24
  
  v9 = 0;
  v10 = 0;
  v11 = 0;
  v8 = 0;
  if (dat_e1140) {
    do {
      if (!(v9 & 7)) {
        v1 = v11 + 1;
        v8 = *(unsigned char *)((unsigned long)v11 + 0xe0140);
        v11 = v1;
      }
      v1 = v9 + 1;
      v5 = (unsigned int)*(unsigned char *)((unsigned long)v9 + 0x1b000);
      if (v8 & 1) { // branch-flip
        v6 = (unsigned int)*(unsigned char *)((long)v5 + 0xdfd40);
        sub_3a6c(*(unsigned short *)(a0 + (unsigned long)(v6 + 0x101) * 4),*(unsigned short *)(a0 + (unsigned long)(v6 + 0x101) * 4 + 2));
        v4 = *(int *)((unsigned long)v6 * 4 + 0x19320);
        if (v4)
          sub_3a6c(v5 - *(int *)((unsigned long)v6 * 4 + 0xe0040),v4);
        v5 = v10 + 1;
        v3 = *(unsigned short *)((unsigned long)v10 * 2 + 0x9c800);
        v6 = (unsigned int)v3;
        v2 = (0x100 <= v6) ? *(unsigned char *)((unsigned long)((v3 >> 7) + 0x100) + 0xdfe40) : *(unsigned char *)((unsigned long)v6 + 0xdfe40); // branch-flip
        v7 = (unsigned int)v2;
        sub_3a6c(*(unsigned short *)(a1 + (unsigned long)v7 * 4),*(unsigned short *)(a1 + (unsigned long)v7 * 4 + 2));
        v4 = *(int *)((unsigned long)v7 * 4 + 0x193a0);
        v10 = v5;
        if (v4)
          sub_3a6c(v6 - *(int *)((unsigned long)v7 * 4 + 0xe00c0),v4);
      }
      else {
        sub_3a6c(*(unsigned short *)(a0 + (long)v5 * 4),*(unsigned short *)(a0 + (long)v5 * 4 + 2));
      }
      v8 >>= 1;
      v9 = v1;
    } while (v1 < dat_e1140);
  }
  sub_3a6c(*(unsigned short *)(a0 + 0x400),*(unsigned short *)(a0 + 0x402));
}


// Function: sub_c97c @ 0xc97c
void sub_c97c(void)
{
  int v1; // stack - 0x14
  unsigned int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  v2 = 0;
  v3 = 0;
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    v3 += *(unsigned short *)((long)v1 * 4 + 0xde240);
  }
  for (; v1 <= 0x7f; v1 = v1 + 1) {
    v2 += *(unsigned short *)((long)v1 * 4 + 0xde240);
  }
  for (; v1 <= 0xff; v1 = v1 + 1) {
    v3 += *(unsigned short *)((long)v1 * 4 + 0xde240);
  }
  *dat_e1170 = (unsigned short)(v3 <= v2 >> 2);
}


// Function: sub_ca3c @ 0xca3c
void sub_ca3c(int a0)
{
  unsigned int v1;
  int v2; // stack - 0xc
  
  dat_e13c0 <<= (unsigned char)a0 & 0x1f;
  v2 = a0;
  while (dat_e13c8 < v2) {
    v2 -= dat_e13c8;
    dat_e13c0 |= (unsigned short)(dat_e13c4 << ((unsigned char)v2 & 0x1f));
    if (dat_dd868 <= dat_dd86c) // branch-flip
      dat_e13c4 = sub_f0bc(1);
    else {
      v1 = dat_dd86c + 1;
      dat_e13c4 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
      dat_dd86c = v1;
    }
    if (dat_e13c4 == 0xffffffff)
      dat_e13c4 = 0;
    dat_e13c8 = 8;
  }
  dat_e13c8 -= v2;
  dat_e13c0 |= (unsigned short)(dat_e13c4 >> ((unsigned char)dat_e13c8 & 0x1f));
}


// Function: sub_cb39 @ 0xcb39
int sub_cb39(unsigned int a0)
{
  int v1; // eax
  
  v1 = (unsigned int)dat_e13c0;
  sub_ca3c(a0);
  return v1 >> (0x10U - (char)a0 & 0x1fU);
}


// Function: sub_cb74 @ 0xcb74
void sub_cb74(void)
{
  dat_e13c0 = 0;
  dat_e13c4 = 0;
  dat_e13c8 = 0;
  sub_ca3c(0x10);
}


// Function: sub_cba6 @ 0xcba6
void sub_cba6(unsigned int a0,long a1,unsigned int a2,long a3) // ternary
{
  unsigned int v1;
  long v10; // fs_offset
  unsigned int v11; // stack - 0xc0
  unsigned int v12; // stack - 0xbc
  unsigned int v13; // stack - 0xb8
  unsigned int v14; // stack - 0xb4
  unsigned short *v15; // stack - 0xa0
  unsigned long v16; // stack - 0x10
  unsigned char v2;
  unsigned int v3;
  unsigned int v4; // eax
  long v5; // rax
  unsigned char v6; // cl
  short v7 [24];
  unsigned short v8 [20]; // stack - 0x38
  unsigned short v9 [24];
  
  v16 = *(unsigned long *)(v10 + 0x28);
  for (v11 = 1; v11 <= 0x10; v11 = v11 + 1) {
    v7[v11] = 0;
  }
  for (v11 = 0; v11 < a0; v11 = v11 + 1) {
    v7[(int)(unsigned int)*(unsigned char *)(a1 + (unsigned long)v11)] = v7[(int)(unsigned int)*(unsigned char *)(a1 + (unsigned long)v11)] + 1;
  }
  v8[1] = 0;
  for (v11 = 1; v11 <= 0x10; v11 = v11 + 1) {
    v8[v11 + 1] = v8[v11] + (v7[v11] << (0x10U - (char)v11 & 0x1fU));
  }
  if (!v8[0x11]) {
    v2 = (unsigned char)a2;
    v6 = 0x10 - v2;
    for (v11 = 1; v11 <= a2; v11 = v11 + 1) {
      v8[v11] = (unsigned short)((int)(unsigned int)v8[v11] >> (v6 & 0x1f));
      v9[v11] = (unsigned short)(1 << (v2 - (char)v11 & 0x1f));
    }
    for (; v11 <= 0x10; v11 = v11 + 1) {
      v9[v11] = (unsigned short)(1 << (0x10U - (char)v11 & 0x1fU));
    }
    v11 = (int)(unsigned int)v8[(int)(a2 + 1)] >> (v6 & 0x1f);
    if (v11) {
      while (v11 != 1 << (v2 & 0x1f)) {
        v3 = v11 + 1;
        *(unsigned short *)(a3 + (unsigned long)v11 * 2) = 0;
        v11 = v3;
      }
    }
    v13 = 0;
    v14 = a0;
    do {
      if (a0 <= v13)
        return;
      v3 = (unsigned int)*(unsigned char *)(a1 + (unsigned long)v13);
      if (v3) {
        v4 = (unsigned int)v9[v3] + (unsigned int)v8[v3];
        if (v3 <= a2) { // branch-flip
          if ((unsigned int)(1 << (v2 & 0x1f)) < v4)
            sub_f5ef("Bad table\n"); // no-return
          for (v11 = (unsigned int)v8[v3]; v11 < v4; v11 = v11 + 1) {
            *(unsigned short *)(a3 + (unsigned long)v11 * 2) = (unsigned short)v13;
          }
        }
        else {
          v12 = (unsigned int)v8[v3];
          v15 = (unsigned short *)(a3 + (unsigned long)(v8[v3] >> (v6 & 0x1f)) * 2);
          for (v11 = v3 - a2; v11; v11 = v11 + -1) {
            if (!*v15) {
              *(unsigned short *)((unsigned long)v14 * 2 + 0xbd000) = 0;
              *(unsigned short *)(((unsigned long)v14 + 0x8000) * 2 + 0xbd000) = *(unsigned short *)((unsigned long)v14 * 2 + 0xbd000);
              v1 = v14 + 1;
              *v15 = (unsigned short)v14;
              v14 = v1;
            }
            v5 = (v12 & 1 << (0xf - v2 & 0x1f)) ? (unsigned long)*v15 + 0x8000 : (long)(int)(unsigned int)*v15; // branch-flip
            v15 = (unsigned short *)(v5 * 2 + 0xbd000);
            v12 <<= 1;
          }
          *v15 = (unsigned short)v13;
        }
        v8[v3] = (unsigned short)v4;
      }
      v13 += 1;
    } while( true );
  }
  sub_f5ef("Bad table\n"); // no-return
}


// Function: sub_d0a0 @ 0xd0a0
void sub_d0a0(int a0,unsigned int a1,int a2) // return-dupe
{
  int v1;
  unsigned short v2; // ax
  int v3;
  unsigned int v4;
  int v5; // stack - 0x18
  unsigned int v6; // stack - 0x14
  unsigned int v7; // stack - 0x10
  
  v3 = sub_cb39(a1);
  if (!v3) {
    v2 = sub_cb39(a1);
    for (v5 = 0; v5 < a0; v5 = v5 + 1) {
      *(char *)((long)v5 + 0xe1180) = 0;
    }
    for (v5 = 0; v5 <= 0xff; v5 = v5 + 1) {
      *(unsigned short *)((long)v5 * 2 + 0xe11c0) = v2;
    }
    return;
  }
  v5 = 0;
  do {
    if (v3 <= v5) {
      while (v5 < a0) {
        v3 = v5 + 1;
        *(char *)((long)v5 + 0xe1180) = 0;
        v5 = v3;
      }
      sub_cba6(a0,0xe1180,8,0xe11c0);
      return;
    }
    v6 = (unsigned int)(dat_e13c0 >> 0xd);
    if (v6 == 7) {
      for (v7 = 0x1000; dat_e13c0 & v7; v7 = v7 >> 1) {
        v6 += 1;
      }
      if (0x11 <= (int)v6)
        sub_f5ef("Bad table\n"); // no-return
    }
    v4 = v6;
    if ((int)v6 <= 5)
      v4 = 6;
    sub_ca3c(v4 - 3);
    v1 = v5 + 1;
    *(char *)((long)v5 + 0xe1180) = (char)v6;
    v5 = v1;
    if (v1 == a2) {
      v6 = sub_cb39(2);
      while (v6 = v6 + -1, 0 <= (int)v6) {
        v1 = v5 + 1;
        *(char *)((long)v5 + 0xe1180) = 0;
        v5 = v1;
      }
    }
  } while( true );
}


// Function: sub_d23e @ 0xd23e
void sub_d23e(void) // return-dupe, ternary x2
{
  int v1;
  unsigned short v2;
  unsigned short v3; // ax
  int v4;
  int v5; // stack - 0x18
  unsigned int v6; // stack - 0x14
  unsigned int v7; // stack - 0x10
  
  v4 = sub_cb39(9);
  if (!v4) {
    v3 = sub_cb39(9);
    for (v5 = 0; v5 <= 0x1fd; v5 = v5 + 1) {
      *(char *)((long)v5 + 0x5c000) = 0;
    }
    for (v5 = 0; v5 <= 0xfff; v5 = v5 + 1) {
      *(unsigned short *)((long)v5 * 2 + 0x9c800) = v3;
    }
    return;
  }
  v5 = 0;
  while (v5 < v4) {
    v6 = (unsigned int)*(unsigned short *)((long)(int)(unsigned int)(dat_e13c0 >> 8) * 2 + 0xe11c0);
    if (0x13 <= v6) {
      v7 = 0x80;
      do {
        v2 = (dat_e13c0 & v7) ? *(unsigned short *)(((long)(int)v6 + 0x8000) * 2 + 0xbd000) : *(unsigned short *)((long)(int)v6 * 2 + 0xbd000); // branch-flip
        v6 = (unsigned int)v2;
        v7 >>= 1;
      } while (0x12 < v6);
    }
    sub_ca3c((unsigned char)*(char *)((long)(int)v6 + 0xe1180));
    if (3 <= v6) { // branch-flip
      v1 = v5 + 1;
      *(char *)((long)v5 + 0x5c000) = (char)v6 + '\xfe';
      v5 = v1;
    }
    else {
      if (v6) { // branch-flip
        v6 = (v6 != 1) ? sub_cb39(9) + 0x14 : sub_cb39(4) + 3; // branch-flip
      }
      else {
        v6 = 1;
      }
      while (v6 = v6 + -1, 0 <= (int)v6) {
        v1 = v5 + 1;
        *(char *)((long)v5 + 0x5c000) = 0;
        v5 = v1;
      }
    }
  }
  while (v5 <= 0x1fd) {
    v4 = v5 + 1;
    *(char *)((long)v5 + 0x5c000) = 0;
    v5 = v4;
  }
  sub_cba6(0x1fe,0x5c000,0xc,0x9c800);
}


// Function: sub_d445 @ 0xd445
unsigned int sub_d445(void) // ternary
{
  unsigned short v1;
  unsigned int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  if (!dat_e11a0) {
    dat_e11a0 = sub_cb39(0x10);
    if (!dat_e11a0) {
      dat_e11a0 = 0;
      return 0x1fe;
    }
    sub_d0a0(0x13,5,3);
    sub_d23e();
    sub_d0a0(0xe,4,0xffffffff);
  }
  dat_e11a0 -= 1;
  v2 = (unsigned int)*(unsigned short *)((long)(int)(unsigned int)(dat_e13c0 >> 4) * 2 + 0x9c800);
  if (0x1fe <= v2) {
    v3 = 8;
    do {
      v1 = (dat_e13c0 & v3) ? *(unsigned short *)(((unsigned long)v2 + 0x8000) * 2 + 0xbd000) : *(unsigned short *)((unsigned long)v2 * 2 + 0xbd000); // branch-flip
      v2 = (unsigned int)v1;
      v3 >>= 1;
    } while (0x1fe <= v2);
  }
  sub_ca3c((unsigned char)*(char *)((unsigned long)v2 + 0x5c000));
  return v2;
}


// Function: sub_d564 @ 0xd564
unsigned int sub_d564(void) // ternary
{
  unsigned short v1;
  unsigned int v2; // stack - 0x20
  unsigned int v3; // stack - 0x1c
  
  v2 = (unsigned int)*(unsigned short *)((long)(int)(unsigned int)(dat_e13c0 >> 8) * 2 + 0xe11c0);
  if (0xe <= v2) {
    v3 = 0x80;
    do {
      v1 = (dat_e13c0 & v3) ? *(unsigned short *)(((unsigned long)v2 + 0x8000) * 2 + 0xbd000) : *(unsigned short *)((unsigned long)v2 * 2 + 0xbd000); // branch-flip
      v2 = (unsigned int)v1;
      v3 >>= 1;
    } while (0xe <= v2);
  }
  sub_ca3c((unsigned char)*(char *)((unsigned long)v2 + 0xe1180));
  if (v2)
    v2 = sub_cb39(v2 - 1) + (1 << ((char)v2 - 1U & 0x1fU));
  return v2;
}


// Function: sub_d641 @ 0xd641
void sub_d641(void)
{
  sub_cb74();
  dat_e11a0 = 0;
}


// Function: sub_d65b @ 0xd65b
void sub_d65b(void)
{
  sub_d641();
  dat_e13cc = 0;
  dat_e13d0 = 0;
}


// Function: sub_d67f @ 0xd67f
unsigned int sub_d67f(unsigned int a0,long a1)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0x10
  
  v2 = 0;
  do {
    dat_e13cc -= 1;
    if (dat_e13cc <= -1) {
      do {
        while( true ) {
          v1 = sub_d445();
          if (v1 == 0x1fe) {
            dat_e13d0 = 1;
            return v2;
          }
          if (v1 <= 0xff) break;
          dat_e13cc = v1 - 0xfd;
          dat_e13d4 = (v2 - sub_d564()) - 1 & 0x1fff;
          while (dat_e13cc = dat_e13cc + -1, 0 <= dat_e13cc) {
            *(char *)(a1 + (unsigned long)v2) = *(char *)(a1 + (unsigned long)dat_e13d4);
            dat_e13d4 = dat_e13d4 + 1 & 0x1fff;
            v2 += 1;
            if (v2 == a0)
              return v2;
          }
        }
        *(char *)(a1 + (unsigned long)v2) = (char)v1;
        v2 += 1;
      } while (v2 != a0);
      return v2;
    }
    *(char *)(a1 + (unsigned long)v2) = *(char *)(a1 + (unsigned long)dat_e13d4);
    dat_e13d4 = dat_e13d4 + 1 & 0x1fff;
    v2 += 1;
  } while (v2 != a0);
  return v2;
}


// Function: sub_d7d6 @ 0xd7d6
unsigned long sub_d7d6(unsigned int a0,unsigned int a1)
{
  int v1; // eax
  
  dat_dd860 = a0;
  dat_dd864 = a1;
  sub_d65b();
  while (!dat_e13d0) {
    v1 = sub_d67f(0x2000,0xad000);
    if (v1)
      sub_f321(a1,(void *)0xad000,v1);
  }
  return 0;
}


// Function: sub_d846 @ 0xd846
unsigned long sub_d846(unsigned int a0,unsigned int a1) // return-dupe, ternary x3
{
  int v1;
  unsigned int v10; // stack - 0x74
  unsigned int v11; // stack - 0x70
  unsigned int v12; // stack - 0x6c
  int v13; // stack - 0x68
  int v14; // stack - 0x64
  unsigned char *v15; // stack - 0x58
  unsigned long v16; // stack - 0x50
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x40
  long v19; // stack - 0x38
  unsigned char v2;
  long v20; // stack - 0x30
  unsigned int v3;
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  long v7; // rdx
  unsigned int v8; // stack - 0x7c
  int v9; // stack - 0x78
  
  if (dat_dd868 <= dat_dd86c) // branch-flip
    dat_19098 = sub_f0bc(0);
  else {
    v3 = dat_dd86c + 1;
    dat_19098 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
    dat_dd86c = v3;
  }
  dat_19548 = dat_19098 & 0x80;
  if (dat_19098 & 0x60) {
    if (!dat_dd004)
      fprintf(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",dat_dd010,(char *)0xdd060,dat_19098 & 0x60);
    if (!dat_dd018)
      dat_dd018 = 2;
  }
  dat_19098 &= 0x1f;
  v7 = 1L << (signed char)dat_19098;
  if (0x11 <= dat_19098) {
    fprintf(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",dat_dd010,(char *)0xdd060,dat_19098,0x10);
    dat_dd018 = 1;
    return 1;
  }
  v12 = dat_dd868;
  v11 = 9;
  v20 = 0x1ff;
  v10 = 0x1ff;
  v17 = 0xffffffffffffffff;
  v8 = 0;
  v9 = 0;
  v18 = (unsigned long)(dat_dd86c << 3);
  v19 = (dat_19548) ? 0x101 : 0x100; // branch-flip
  memset((void *)0xbd000,0,0x100);
  for (v16 = 0xff; 0 <= (long)v16; v16 = v16 + -1) {
    *(char *)(v16 + 0xad000) = (char)v16;
  }
label_da53:
  do {
    v3 = (unsigned int)((long)v18 >> 3);
    if (v3 <= dat_dd868) // branch-flip
      dat_dd868 -= v3;
    else {
      dat_dd868 = 0;
    }
    for (v13 = 0; v13 < (int)dat_dd868; v13 = v13 + 1) {
      *(char *)((long)v13 + 0x1b000) = *(char *)((long)(int)(v3 + v13) + 0x1b000);
    }
    v18 = 0;
    if (dat_dd868 <= 0x3f) {
      v12 = sub_f194(a0,(void *)((unsigned long)dat_dd868 + 0x1b000),0x40000);
      if (v12 == 0xffffffff)
        sub_f6c7(); // no-return
      dat_dd868 = v12 + dat_dd868;
      dat_dd040 += (int)v12;
    }
    v4 = (v12) ? ((unsigned long)dat_dd868 - (unsigned long)(dat_dd868 % v11)) * 8 : (unsigned long)dat_dd868 * 8 - (long)(int)(v11 - 1); // branch-flip
    while ((long)v18 < v4) {
      if (v20 < v19) {
        v18 = ((long)(int)(v11 << 3) - (long)((long)(int)(v11 << 3) + (v18 - 1)) % (long)(int)(v11 << 3)) + (v18 - 1);
        v11 += 1;
        if (v11 != dat_19098) // branch-flip
          v20 = (1L << ((unsigned char)v11 & 0x3f)) + -1;
        else {
          v20 = v7;
        }
        v10 = (1 << ((unsigned char)v11 & 0x1f)) - 1;
        goto label_da53;
      }
      v5 = (unsigned long)v10 & (long)(unsigned long)*(undefined3 *)(((long)v18 >> 3) + 0x1b000) >> ((unsigned char)v18 & 7);
      v18 += (long)(int)v11;
      if (v17 != 0xffffffffffffffff) { // branch-flip
        if ((v5 == 0x100) && (dat_19548)) {
          memset((void *)0xbd000,0,0x100);
          v19 = 0x100;
          v18 = ((long)(int)(v11 << 3) - (long)((long)(int)(v11 << 3) + (v18 - 1)) % (long)(int)(v11 << 3)) + (v18 - 1);
          v11 = 9;
          v20 = 0x1ff;
          v10 = 0x1ff;
          goto label_da53;
        }
        v15 = (char *)0xac7fe;
        v16 = v5;
        if (v19 <= (long)v5) {
          if (v19 < (long)v5) {
            if (1 <= v9)
              sub_f321(a1,(void *)0x5c000,v9);
            v6 = (dat_dd000) ? "corrupt input." : "corrupt input. Use zcat to recover some data."; // branch-flip
            sub_f5ef(v6); // no-return
          }
          v15 = (char *)0xac7fd;
          dat_ac7fd = (char)v8;
          v16 = v17;
        }
        for (; 0x100 <= v16; v16 = (unsigned long)*(unsigned short *)(v16 * 2 + 0xbd000)) {
          v15 = &v15[-1];
          *v15 = *(char *)(v16 + 0xad000);
        }
        v2 = *(unsigned char *)(v16 + 0xad000);
        v8 = (unsigned int)v2;
        v15 = &v15[-1];
        *v15 = v2;
        v14 = 0xac7fe - (int)v15;
        if (0x40000 <= v9 + v14) { // branch-flip
          do {
            if (0x40000U - v9 < v14)
              v14 = 0x40000 - v9;
            if (1 <= v14) {
              memcpy((void *)((long)v9 + 0x5c000),v15,(long)v14);
              v9 += v14;
            }
            if (0x40000 <= v9) {
              sub_f321(a1,(void *)0x5c000,v9);
              v9 = 0;
            }
            v15 = &v15[v14];
            v14 = 0xac7fe - (int)v15;
          } while (0 < v14);
        }
        else {
          memcpy((void *)((long)v9 + 0x5c000),v15,(long)v14);
          v9 += v14;
        }
        if (v19 < v7) {
          *(short *)(v19 * 2 + 0xbd000) = (short)v17;
          *(unsigned char *)(v19 + 0xad000) = v2;
          v19 += 1;
        }
        v17 = v5;
      }
      else {
        if (0x100 <= v5)
          sub_f5ef("corrupt input."); // no-return
        v8 = (unsigned int)v5;
        v1 = v9 + 1;
        *(char *)((long)v9 + 0x5c000) = (char)v5;
        v9 = v1;
        v17 = v5;
      }
    }
    if (!v12) {
      if (v9 <= 0)
        return 0;
      sub_f321(a1,(void *)0x5c000,v9);
      return 0;
    }
  } while( true );
}


// Function: sub_dff1 @ 0xdff1
unsigned int sub_dff1(void)
{
  unsigned int v1;
  int v2; // eax
  
  if (dat_dd868 <= dat_dd86c) // branch-flip
    v2 = sub_f0bc(0);
  else {
    v1 = dat_dd86c + 1;
    v2 = (unsigned int)*(unsigned char *)((unsigned long)dat_dd86c + 0x1b000);
    dat_dd86c = v1;
  }
  if (v2 <= -1)
    sub_f5ef("invalid compressed data -- unexpected end of file"); // no-return
  return v2;
}


// Function: sub_e055 @ 0xe055
void sub_e055(void)
{
  int v1;
  unsigned long v2; // rax
  unsigned int v3; // stack - 0x28
  int v4; // stack - 0x24
  int v5; // stack - 0x20
  int v6; // stack - 0x1c
  
  v6 = 1;
  dat_e13e0 = 0;
  for (v5 = 1; v5 <= 4; v5 = v5 + 1) {
    v2 = dat_e13e0 << 8;
    dat_e13e0 = (unsigned char)sub_dff1() | v2;
  }
  dat_e13e8 = (unsigned int)(unsigned char)sub_dff1();
  if ((!dat_e13e8) || (0x1a <= dat_e13e8))
    sub_f5ef("invalid compressed data -- Huffman code bit length out of range"); // no-return
  v5 = 0;
  for (v3 = 1; (int)v3 <= (int)dat_e13e8; v3 = v3 + 1) {
    *(unsigned int *)((long)(int)v3 * 4 + 0xe1580) = (unsigned int)(unsigned char)sub_dff1();
    if ((int)(v6 - (unsigned int)(v3 == dat_e13e8)) < *(int *)((long)(int)v3 * 4 + 0xe1580))
      sub_f5ef("too many leaves in Huffman tree"); // no-return
    v6 = ((v6 - *(int *)((long)(int)v3 * 4 + 0xe1580)) + 1) * 2 + -1;
    v5 += *(int *)((long)(int)v3 * 4 + 0xe1580);
  }
  if (0x100 <= v5)
    sub_f5ef("too many leaves in Huffman tree"); // no-return
  *(int *)((long)(int)dat_e13e8 * 4 + 0xe1580) = *(int *)((long)(int)dat_e13e8 * 4 + 0xe1580) + 1;
  v4 = 0;
  for (v3 = 1; (int)v3 <= (int)dat_e13e8; v3 = v3 + 1) {
    *(int *)((long)(int)v3 * 4 + 0xe1500) = v4;
    v5 = *(int *)((long)(int)v3 * 4 + 0xe1580);
    while (0 < v5) {
      v1 = v4 + 1;
      *(char *)((long)v4 + 0xe1400) = sub_dff1();
      v5 -= 1;
      v4 = v1;
    }
  }
  *(int *)((long)(int)dat_e13e8 * 4 + 0xe1580) = *(int *)((long)(int)dat_e13e8 * 4 + 0xe1580) + 1;
}


// Function: sub_e3fd @ 0xe3fd
unsigned long sub_e3fd(unsigned int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  unsigned int v3; // eax
  unsigned int v4;
  unsigned long v5; // rax
  unsigned int v6; // stack - 0x2c
  
  dat_dd860 = a0;
  dat_dd864 = a1;
  sub_e055();
  sub_e29d();
  dat_e1678 = 0;
  dat_e1670 = 0;
  v2 = (1 << ((unsigned char)dat_e1668 & 0x1f)) - 1;
  v1 = *(int *)((long)(int)dat_e13e8 * 4 + 0xe1580);
  while( true ) {
    for (; dat_e1678 < (int)dat_e1668; dat_e1678 = dat_e1678 + 8) {
      v5 = dat_e1670 << 8;
      dat_e1670 = (unsigned char)sub_dff1() | v5;
    }
    v3 = (unsigned int)(dat_e1670 >> ((char)dat_e1678 - (char)dat_e1668 & 0x3fU)) & v2;
    v6 = (unsigned int)*(unsigned char *)((unsigned long)v3 + 0x5c000);
    if (v6) // branch-flip
      v3 >>= (char)dat_e1668 - *(unsigned char *)((unsigned long)v3 + 0x5c000) & 0x1f;
    else {
      v6 = dat_e1668;
      v4 = v2;
      while (v3 < *(unsigned int *)((long)(int)v6 * 4 + 0xe1600)) {
        v6 += 1;
        v4 = v4 * 2 + 1;
        for (; dat_e1678 < (int)v6; dat_e1678 = dat_e1678 + 8) {
          v5 = dat_e1670 << 8;
          dat_e1670 = (unsigned char)sub_dff1() | v5;
        }
        v3 = (unsigned int)(dat_e1670 >> ((char)dat_e1678 - (char)v6 & 0x3fU)) & v4;
      }
    }
    if ((v3 == v1 - 1U) && (v6 == dat_e13e8)) break;
    v4 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0xad000) = *(char *)((unsigned long)(v3 + *(int *)((long)(int)v6 * 4 + 0xe1500)) + 0xe1400);
    dat_dd870 = v4;
    if (v4 == 0x8000)
      sub_f2cc();
    dat_e1678 -= v6;
  }
  sub_f2cc();
  if ((dat_dd048 & 0xffffffff) != dat_e13e0)
    sub_f5ef("invalid compressed data--length error"); // no-return
  return 0;
}


// Function: sub_e655 @ 0xe655
unsigned long sub_e655(unsigned int a0) // return-dupe x3
{
  unsigned long v1; // rdx
  
  v1 = (unsigned long)dat_dd86c;
  dat_dd86c += (unsigned int)*(unsigned short *)(v1 + 0x1b01c) + *(unsigned short *)(v1 + 0x1b01a) + 0x1e;
  dat_dd860 = a0;
  if ((dat_dd868 < dat_dd86c) || (((long)(int)(unsigned int)*(unsigned short *)(v1 + 0x1b002) << 0x10 | (long)(int)(unsigned int)*(unsigned short *)(v1 + 0x1b000)) != 0x4034b50)) {
    fprintf(stderr,"\n%s: %s: not a valid zip file\n",dat_dd010,(char *)0xdd060);
    dat_dd018 = 1;
    return 1;
  }
  dat_1909c = (unsigned int)*(unsigned char *)(v1 + 0x1b008);
  if ((dat_1909c) && (dat_1909c != 8)) {
    fprintf(stderr,"\n%s: %s: first entry not deflated or stored -- use unzip\n",dat_dd010,(char *)0xdd060);
    dat_dd018 = 1;
    return 1;
  }
  dat_e1688 = *(unsigned char *)(v1 + 0x1b006) & 1;
  if (*(unsigned char *)(v1 + 0x1b006) & 1) {
    fprintf(stderr,"\n%s: %s: encrypted file -- use unzip\n",dat_dd010,(char *)0xdd060);
    dat_dd018 = 1;
    return 1;
  }
  dat_e168c = 1;
  dat_e1690 = (unsigned int)((*(unsigned char *)(v1 + 0x1b006) & 8) != 0);
  return 0;
}


// Function: sub_e86a @ 0xe86a
char sub_e86a(unsigned int a0,unsigned int a1) // early-return, ternary
{
  unsigned int v1;
  bool v10;
  int v11; // stack - 0x54
  unsigned long v12; // stack - 0x48
  unsigned long v13; // stack - 0x40
  unsigned short v14; // stack - 0x36
  unsigned short v15; // stack - 0x34
  unsigned short v16; // stack - 0x32
  unsigned short v17; // stack - 0x2c
  unsigned short v18; // stack - 0x2a
  char v2;
  char v3; // al
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rcx
  unsigned long v7;
  unsigned long v8;
  unsigned short v9; // stack - 0x38
  
  v12 = 0;
  v13 = 0;
  dat_dd860 = a0;
  dat_dd864 = a1;
  sub_efda(NULL,0);
  if ((dat_e168c) && (!dat_e1690)) {
    v4 = (unsigned int)CONCAT11(dat_1b00f,dat_1b00e);
    v7 = (unsigned long)v4;
    v4 = (unsigned int)CONCAT11(dat_1b011,dat_1b010);
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v5 = (unsigned long)((unsigned long)v5 | v7);
    v12 = (unsigned long)v5;
    v4 = (unsigned int)CONCAT11(dat_1b017,dat_1b016);
    v7 = (unsigned long)v4;
    v4 = (unsigned int)CONCAT11(dat_1b019,dat_1b018);
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v5 = (unsigned long)((unsigned long)v5 | v7);
    v13 = (unsigned long)v5;
  }
  if (dat_1909c != 8) { // branch-flip
    if ((!dat_e168c) || (dat_1909c))
      sub_f5ef("internal error, invalid method"); // no-return
    v4 = (unsigned int)CONCAT11(dat_1b017,dat_1b016);
    v7 = (unsigned long)v4;
    v4 = (unsigned int)CONCAT11(dat_1b019,dat_1b018);
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v7 |= (unsigned long)v5;
    v4 = (unsigned int)CONCAT11(dat_1b013,dat_1b012);
    v8 = (unsigned long)v4;
    v4 = (unsigned int)CONCAT11(dat_1b015,dat_1b014);
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v6 = (dat_e1688) ? 0xc : 0; // branch-flip
    v5 = (unsigned long)((v8 | (unsigned long)v5) - v6);
    if ((char *)v7 != v5) {
      v4 = (unsigned int)CONCAT11(dat_1b013,dat_1b012);
      v8 = (unsigned long)v4;
      v4 = (unsigned int)CONCAT11(dat_1b015,dat_1b014);
      v5 = (long)(long)v4;
      v5 = (unsigned long)((long)v5 << 0x10);
      fprintf(stderr,"len %lu, siz %lu\n",v7,v8 | (unsigned long)v5);
      sub_f5ef("invalid compressed data--length mismatch"); // no-return
    }
    while (v8 = v7 - 1, v7) {
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v3 = sub_f0bc(0);
      else {
        v1 = dat_dd86c + 1;
        v3 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      v1 = dat_dd870 + 1;
      *(char *)((unsigned long)dat_dd870 + 0xad000) = v3;
      v7 = v8;
      dat_dd870 = v1;
      if (v1 == 0x8000)
        sub_f2cc();
    }
    sub_f2cc();
  }
  else {
    v4 = sub_ab98();
    if (v4 == 3)
      sub_f634(); // no-return
    if (v4)
      sub_f5ef("invalid compressed data--format violated"); // no-return
  }
  if (dat_e168c) { // branch-flip
    if (dat_e1690) {
      for (v11 = 0; v11 <= 0xf; v11 = v11 + 1) {
        if (dat_dd868 <= dat_dd86c) // branch-flip
          v3 = sub_f0bc(0);
        else {
          v1 = dat_dd86c + 1;
          v3 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
          dat_dd86c = v1;
        }
        *(char *)((long)&v9 + (long)v11) = v3;
      }
      v4 = (unsigned int)v15;
      v7 = (unsigned long)v4;
      v4 = (unsigned int)v16;
      v5 = (long)(long)v4;
      v5 = (unsigned long)((long)v5 << 0x10);
      v5 = (unsigned long)((unsigned long)v5 | v7);
      v12 = (unsigned long)v5;
      v4 = (unsigned int)v17;
      v7 = (unsigned long)v4;
      v4 = (unsigned int)v18;
      v5 = (long)(long)v4;
      v5 = (unsigned long)((long)v5 << 0x10);
      v5 = (unsigned long)((unsigned long)v5 | v7);
      v13 = (unsigned long)v5;
    }
  }
  else {
    for (v11 = 0; v11 <= 7; v11 = v11 + 1) {
      if (dat_dd868 <= dat_dd86c) // branch-flip
        v3 = sub_f0bc(0);
      else {
        v1 = dat_dd86c + 1;
        v3 = *(char *)((unsigned long)dat_dd86c + 0x1b000);
        dat_dd86c = v1;
      }
      *(char *)((long)&v9 + (long)v11) = v3;
    }
    v4 = (unsigned int)v9;
    v7 = (unsigned long)v4;
    v4 = (unsigned int)v14;
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v5 = (unsigned long)((unsigned long)v5 | v7);
    v12 = (unsigned long)v5;
    v4 = (unsigned int)v15;
    v7 = (unsigned long)v4;
    v4 = (unsigned int)v16;
    v5 = (long)(long)v4;
    v5 = (unsigned long)((long)v5 << 0x10);
    v5 = (unsigned long)((unsigned long)v5 | v7);
    v13 = (unsigned long)v5;
  }
  v5 = (unsigned long)sub_efda((unsigned char *)0x5c000,0);
  v10 = (char *)v12 != v5;
  if (v10)
    fprintf(stderr,"\n%s: %s: invalid compressed data--crc error\n",dat_dd010,(char *)0xdd060);
  v5 = (unsigned long)(dat_dd048 & 0xffffffff);
  if ((char *)v13 != v5)
    fprintf(stderr,"\n%s: %s: invalid compressed data--length error\n",dat_dd010,(char *)0xdd060);
  v10 = (char *)v13 != v5 || v10;
  if (((dat_e168c) && (dat_dd86c + 4 < dat_dd868)) && (v5 = (char *)((unsigned long)dat_dd86c + 0x1b000), v2 = *v5, v5 = (unsigned long)(unsigned long)dat_dd86c, v5 = (char *)((long)v5 + 0x1b001), v4 = (unsigned int)CONCAT11(*v5,v2), v7 = (unsigned long)v4, v5 = (unsigned long)(unsigned long)dat_dd86c, v5 = (char *)((long)v5 + 0x1b002), v2 = *v5, v5 = (unsigned long)(unsigned long)dat_dd86c, v5 = (char *)((long)v5 + 0x1b003), v4 = (unsigned int)CONCAT11(*v5,v2), v5 = (long)(long)v4, v5 = (unsigned long)((long)v5 << 0x10), v5 = (unsigned long)((unsigned long)v5 | v7), v5 == (char *)0x4034b50)) {
    if (dat_dd000) { // branch-flip
      if (!dat_dd004)
        fprintf(stderr,"%s: %s has more than one entry--rest ignored\n",dat_dd010,(char *)0xdd060);
      if (!dat_dd018)
        dat_dd018 = 2;
    }
    else {
      fprintf(stderr,"%s: %s has more than one entry -- unchanged\n",dat_dd010,(char *)0xdd060);
      v10 = 1;
    }
  }
  dat_e168c = 0;
  dat_e1690 = 0;
  dat_e1680 = v12;
  if (!v10) {
    v3 = 0;
    dat_e168c = 0;
    dat_e1690 = 0;
    return v3;
  }
  dat_dd018 = 1;
  if (dat_dd008) {
    dat_dd018 = 1;
    v3 = v10;
    return v3;
  }
  sub_8f9f(); // no-return
}


// Function: sub_ef1d @ 0xef1d
unsigned long sub_ef1d(unsigned int a0,unsigned int a1)
{
  int v1; // eax
  
  *__errno_location() = 0;
  while( true ) {
    if (dat_dd868 <= dat_dd86c)
      return 0;
    sub_f321(a1,(void *)((unsigned long)dat_dd86c + 0x1b000),dat_dd868 - dat_dd86c);
    v1 = sub_f194(a0,(void *)0x1b000,0x40000);
    if (v1 == 0xffffffff) break;
    dat_dd040 += v1;
    dat_dd86c = 0;
    dat_dd868 = v1;
  }
  sub_f6c7(); // no-return
}


// Function: sub_efda @ 0xefda
unsigned long sub_efda(unsigned char *a0,int a1)
{
  unsigned char *v1;
  int v2; // stack - 0x1c
  unsigned char *v3; // stack - 0x18
  
  if (a0) { // branch-flip
    v2 = a1;
    v3 = a0;
    if (!a1) {
      return dat_19550 ^ 0xffffffff;
    }
    do {
      v1 = &v3[1];
      dat_19550 = *(unsigned long *)((long)(int)(((unsigned int)*v3 ^ (unsigned int)dat_19550) & 0xff) * 8 + 0x14680) ^ dat_19550 >> 8;
      v2 -= 1;
      v3 = v1;
    } while (v2);
  }
  else {
    dat_19550 = 0xffffffff;
  }
  return dat_19550 ^ 0xffffffff;
}


// Function: sub_f05f @ 0xf05f
unsigned long sub_f05f(void)
{
  return dat_19550 ^ 0xffffffff;
}


// Function: sub_f078 @ 0xf078
void sub_f078(void)
{
  dat_dd870 = 0;
  dat_dd86c = 0;
  dat_dd868 = 0;
  dat_dd048 = 0;
  dat_dd040 = 0;
}


// Function: sub_f0bc @ 0xf0bc
unsigned long sub_f0bc(int a0) // early-return
{
  int v1; // eax
  
  dat_dd868 = 0;
  do {
    v1 = sub_f194(dat_dd860,(void *)((unsigned long)dat_dd868 + 0x1b000),0x40000 - dat_dd868);
    if (!v1) break;
    if (v1 == -1)
      sub_f6c7(); // no-return
    dat_dd868 = v1 + dat_dd868;
  } while (dat_dd868 <= 0x3ffff);
  if (dat_dd868) {
    dat_dd86c = 1;
    dat_dd040 += (unsigned long)dat_dd868;
    return (unsigned long)dat_1b000;
  }
  if (a0)
    return 0xffffffff;
  sub_f2cc();
  *__errno_location() = 0;
  sub_f6c7(); // no-return
}


// Function: sub_f194 @ 0xf194
int sub_f194(int a0,void *a1,unsigned int a2)
{
  int v1; // eax
  unsigned int v2; // stack - 0x20
  int v3; // stack - 0x10
  
  v2 = a2;
  if ((int)a2 <= -1)
    v2 = 0x7fffffff;
  v3 = (int)read(a0,a1,(unsigned long)v2);
  if (v3 <= -1) {
    if (*__errno_location() == 0xb) {
      v1 = sub_103bd(a0,3);
      if (0 <= v1) {
        if (v1 & 0x800) { // branch-flip
          if (sub_103bd(a0,4,v1 & 0xfffff7ff) != -1)
            v3 = (int)read(a0,a1,(unsigned long)v2);
        }
        else {
          *__errno_location() = 0xb;
        }
      }
    }
  }
  return v3;
}


// Function: sub_f254 @ 0xf254
void sub_f254(int a0,void *a1,unsigned int a2)
{
  unsigned int v1; // stack - 0x10
  
  v1 = a2;
  if ((int)a2 <= -1)
    v1 = 0x7fffffff;
  write(a0,a1,(unsigned long)v1);
}


// Function: sub_f28e @ 0xf28e
void sub_f28e(void) // return-dupe
{
  if (!dat_dd870)
    return;
  sub_f321(dat_dd864,(void *)0x5c000,dat_dd870);
  dat_dd870 = 0;
}


// Function: sub_f2cc @ 0xf2cc
void sub_f2cc(void) // return-dupe
{
  if (!dat_dd870)
    return;
  sub_efda((unsigned char *)0xad000,dat_dd870);
  sub_f321(dat_dd864,(void *)0xad000,dat_dd870);
  dat_dd870 = 0;
}


// Function: sub_f321 @ 0xf321
void sub_f321(unsigned int a0,void *a1,unsigned int a2) // return-dupe
{
  unsigned int v1; // eax
  void *v2; // stack - 0x28
  unsigned int v3; // stack - 0x20
  
  dat_dd048 += (unsigned long)a2;
  v2 = a1;
  v3 = a2;
  if (dat_dd008)
    return;
  while( true ) {
    v1 = sub_f254(a0,v2,v3);
    if (v1 == v3)
      return;
    if (v1 == 0xffffffff) break;
    v3 -= v1;
    v2 = (void *)((long)v2 + (unsigned long)v1);
  }
  sub_f74d(); // no-return
}


// Function: sub_f393 @ 0xf393
unsigned char * sub_f393(unsigned char *a0)
{
  unsigned char v1;
  unsigned char *v2; // stack - 0x10
  
  for (v2 = a0; *v2; v2 = &v2[1]) {
    if (*(unsigned short *)((unsigned long)*v2 * 2 + *(long *)__ctype_b_loc()) & 0x100) { // branch-flip
      v1 = *v2;
      v1 = (unsigned char)tolower((unsigned int)v1);
    }
    else {
      v1 = *v2;
    }
    *v2 = v1;
  }
  return a0;
}


// Function: sub_f40a @ 0xf40a
unsigned long sub_f40a(char *a0)
{
  return sub_10244(a0);
}


// Function: sub_f430 @ 0xf430
int sub_f430(char *a0)
{
  return unlink(a0);
}


// Function: sub_f454 @ 0xf454
char * sub_f454(int *a0,unsigned long *a1,char *a2) // early-return x2
{
  char *v1;
  char v2;
  long *v3;
  char *v4; // rax
  int v5; // stack - 0x2c
  char *v6; // stack - 0x28
  long *v7; // stack - 0x20
  
  v5 = 0;
  v4 = getenv(a2);
  if (!v4)
    return NULL;
  v4 = (char *)sub_12377(v4);
  v6 = v4;
  while (*v6) {
    v6 = &v6[strspn(v6," \t")];
    if (!*v6) break;
    v1 = &v6[strcspn(v6," \t")];
    v6 = v1;
    if (*v1) {
      v6 = &v1[1];
      *v1 = '\0';
    }
    v5 += 1;
  }
  if (!v5) {
    free(v4);
    return NULL;
  }
  *a0 = v5 + 1;
  v7 = (long *)sub_1224e((long)(*a0 + 1),8);
  v3 = (long *)*a1;
  *a1 = v7;
  *v7 = *v3;
  v6 = v4;
  for (; v7 = &v7[1], 0 < v5; v5 = v5 + -1) {
    v6 = &v6[strspn(v6," \t")];
    *v7 = (long)v6;
    do {
      v1 = &v6[1];
      v2 = *v6;
      v6 = v1;
    } while (v2);
  }
  *v7 = 0;
  return v4;
}


// Function: sub_f5ef @ 0xf5ef
void sub_f5ef(char *a0)
{
  fprintf(stderr,"\n%s: %s: %s\n",dat_dd010,(char *)0xdd060,a0);
  sub_8f9f(); // no-return
}


// Function: sub_f634 @ 0xf634
void sub_f634(void)
{
  fprintf(stderr,"\n%s: memory_exhausted\n",dat_dd010);
  sub_8f9f(); // no-return
}


// Function: sub_f666 @ 0xf666
void sub_f666(char *a0) // return-dupe
{
  if (!dat_dd004)
    fprintf(stderr,"%s: %s: warning: %s\n",dat_dd010,(char *)0xdd060,a0);
  if (dat_dd018)
    return;
  dat_dd018 = 2;
}


// Function: sub_f6c7 @ 0xf6c7
void sub_f6c7(void)
{
  int v1;
  
  v1 = *__errno_location();
  fprintf(stderr,"\n%s: ",dat_dd010);
  if (v1) { // branch-flip
    *__errno_location() = v1;
    perror((char *)0xdd060);
  }
  else {
    fprintf(stderr,"%s: unexpected end of file\n",(char *)0xdd060);
  }
  sub_8f9f(); // no-return
}


// Function: sub_f74d @ 0xf74d
void sub_f74d(void)
{
  int v1;
  
  v1 = *__errno_location();
  fprintf(stderr,"\n%s: ",dat_dd010);
  *__errno_location() = v1;
  perror((char *)0xdd460);
  sub_8f9f(); // no-return
}


// Function: sub_f7a6 @ 0xf7a6
void sub_f7a6(long a0,long a1,FILE *a2)
{
  double v1;
  
  if (a1) // branch-flip
    v1 = (dat_14ee8 * (double)a0) / (double)a1;
  else {
    v1 = dat_14ef0;
  }
  fprintf(a2,"%5.1f%%",v1);
}


// Function: sub_f81a @ 0xf81a
void sub_f81a(FILE *a0,long a1,int a2)
{
  int v1;
  char *v2;
  char v3 [10];
  char v4 [62];
  int v5; // stack - 0x7c
  long v6; // stack - 0x78
  char *v7; // stack - 0x60
  
  v7 = &v3[2];
  v6 = a1;
  v2 = v7;
  if (0 <= a1) { // branch-flip
    do {
      v7 = &v7[-1];
      *v7 = (char)v6 + (char)(v6 / 10) * '\xf6' + '0';
      v6 /= 10;
    } while (v6);
  }
  else {
    do {
      v7 = v2;
      v7[-1] = 0x30 - ((char)v6 + (char)(v6 / 10) * '\xf6');
      v6 /= 10;
      v2 = &v7[-1];
    } while (v6);
    v7 = &v7[-2];
    *v7 = '-';
  }
  v5 = a2 - (((int)v4 + 0x40) - (int)v7);
  while (v1 = v5 + -1, 0 < v5) {
    putc(0x20,a0);
    v5 = v1;
  }
  for (; v7 < &v3[2]; v7 = &v7[1]) {
    putc((int)*v7,a0);
  }
}


// Function: sub_f9dd @ 0xf9dd
unsigned long sub_f9dd(unsigned int a0,unsigned int a1)
{
  unsigned int v1;
  unsigned short v10; // stack - 0x22
  long v11; // stack - 0x20
  char *v12; // stack - 0x18
  int v2;
  char *v3;
  char v4;
  char v5; // al
  char v6; // ah
  char v7; // ah
  unsigned short v8; // stack - 0x24
  char v9; // stack - 0x25
  
  v9 = 0;
  if (dat_dd01c)
    v9 = 8;
  dat_5c003 = v9;
  dat_5c002 = 8;
  dat_5c001 = 0x8b;
  dat_5c000 = 0x1f;
  v10 = 0;
  v8 = 0;
  dat_dd870 = 4;
  dat_1909c = 8;
  dat_dd860 = a0;
  dat_dd864 = a1;
  if (0 <= dat_dd038) { // branch-flip
    if ((1 <= dat_dd030) && (dat_dd030 <= 0xffffffff))
      v11 = dat_dd030;
    else {
      sub_f666("file timestamp out of range for gzip format");
      v11 = 0;
    }
  }
  else {
    v11 = 0;
  }
  if (0x3fffe <= dat_dd870) { // branch-flip
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)v11;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)((unsigned long)v11 >> 8);
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
  }
  else {
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)v11;
    v1 = dat_dd870 + 2;
    *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)((unsigned long)v11 >> 8);
    dat_dd870 = v1;
  }
  v5 = (char)((unsigned long)v11 >> 0x10); // branch-flip
  if (0x3fffe <= dat_dd870) {
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)((unsigned long)v11 >> 0x18);
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
  }
  else {
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    v1 = dat_dd870 + 2;
    *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)((unsigned long)v11 >> 0x18);
    dat_dd870 = v1;
  }
  sub_efda(NULL,0);
  sub_3a29(a1);
  sub_ac6f(&v8,0x1909c);
  if (dat_190a0 != 1) { // branch-flip
    if (dat_190a0 == 9)
      v10 |= 2;
  }
  else {
    v10 |= 4;
  }
  v1 = dat_dd870 + 1;
  *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)v10;
  dat_dd870 = v1;
  if (v1 == 0x40000)
    sub_f28e();
  v1 = dat_dd870 + 1;
  *(char *)((unsigned long)dat_dd870 + 0x5c000) = 3;
  dat_dd870 = v1;
  if (v1 == 0x40000)
    sub_f28e();
  if (dat_dd01c) {
    v12 = (char *)sub_f40a((char *)0xdd060);
    do {
      v1 = dat_dd870 + 1;
      *(char *)((unsigned long)dat_dd870 + 0x5c000) = *v12;
      dat_dd870 = v1;
      if (v1 == 0x40000)
        sub_f28e();
      v3 = &v12[1];
      v4 = *v12;
      v12 = v3;
    } while (v4);
  }
  dat_e1698 = (unsigned long)dat_dd870;
  sub_4a9e(dat_190a0);
  if ((dat_dd020 != -1) && (dat_dd040 != dat_dd020))
    fprintf(stderr,"%s: %s: file size changed while zipping\n",dat_dd010,(char *)0xdd060);
  if (0x3fffe <= dat_dd870) { // branch-flip
    v5 = sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v7;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
  }
  else {
    v5 = sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    dat_dd870 = v1;
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v6;
    dat_dd870 = v1;
  }
  if (0x3fffe <= dat_dd870) { // branch-flip
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = dat_2;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = dat_3;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
  }
  else {
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = dat_2;
    dat_dd870 = v1;
    sub_f05f();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = dat_3;
    dat_dd870 = v1;
  }
  if (0x3fffe <= dat_dd870) { // branch-flip
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)dat_dd040;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)((unsigned long)dat_dd040 >> 8);
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
  }
  else {
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)dat_dd040;
    v1 = dat_dd870 + 2;
    *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)((unsigned long)dat_dd040 >> 8);
    dat_dd870 = v1;
  }
  v5 = (char)((unsigned long)dat_dd040 >> 0x10); // branch-flip
  if (0x3fffe <= dat_dd870) {
    v1 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    dat_dd870 = v1;
    if (v1 == 0x40000)
      sub_f28e();
    v2 = dat_dd870 + 1;
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = (char)((unsigned long)dat_dd040 >> 0x18);
    dat_dd870 = v2;
    if (v2 == 0x40000)
      sub_f28e();
  }
  else {
    *(char *)((unsigned long)dat_dd870 + 0x5c000) = v5;
    v2 = dat_dd870 + 2;
    *(char *)((unsigned long)(dat_dd870 + 1) + 0x5c000) = (char)((unsigned long)dat_dd040 >> 0x18);
    dat_dd870 = v2;
  }
  dat_e1698 += 8;
  sub_f28e();
  return 0;
}


// Function: sub_101d7 @ 0x101d7
unsigned int sub_101d7(unsigned char *a0,unsigned int a1)
{
  unsigned int v1; // eax
  
  v1 = sub_f194(dat_dd860,a0,a1);
  if (v1) {
    if (v1 == 0xffffffff)
      sub_f6c7(); // no-return
    sub_efda(a0,v1);
    dat_dd040 += (unsigned long)v1;
  }
  return v1;
}


// Function: sub_10244 @ 0x10244
char * sub_10244(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_1029b;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_1029b:
  }
  return v2;
}


// Function: sub_102b1 @ 0x102b1
unsigned long sub_102b1(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}


// Function: sub_103bd @ 0x103bd
int sub_103bd(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_1072f(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_10760(a0,(unsigned int)v2);
  if (0xc <= a1) { // branch-flip
    if ((0x40b <= a1) || (a1 <= 0x3ff)) {
      return fcntl(a0,a1,v2); // return-dupe
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x2c5) {
      return fcntl(a0,a1,v2 & 0xffffffff); // return-dupe
    }
    v1 &= 0x502;
  }
  else {
    if (a1 <= -1) {
      return fcntl(a0,a1,v2);
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x515)
      return fcntl(a0,a1,v2 & 0xffffffff);
    v1 &= 0xa0a;
  }
  if (v1)
    return fcntl(a0,a1);
  return fcntl(a0,a1,v2);
}


// Function: sub_1072f @ 0x1072f
int sub_1072f(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}


// Function: sub_10760 @ 0x10760
int sub_10760(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_e16a0) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_e16a0 = 1;
    else {
      v3 = sub_1072f(a0,a1);
      if (0 <= v3)
        dat_e16a0 = -1;
    }
  }
  else {
    v3 = sub_1072f(a0,a1);
  }
  if ((0 <= v3) && (dat_e16a0 == -1)) {
    v2 = fcntl(v3,1);
    if ((0 <= (int)v2) && (fcntl(v3,2,(unsigned long)(v2 | 1)) != -1))
      return v3;
    v1 = *__errno_location();
    close(v3);
    *__errno_location() = v1;
    v3 = -1;
  }
  return v3;
}


// Function: sub_10863 @ 0x10863
void sub_10863(char *a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa8
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = v2;
  sub_110e3(open(a0,a1,(unsigned long)v1));
}


// Function: sub_1099f @ 0x1099f
void sub_1099f(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_10a26(a0,0,1);
}


// Function: sub_10a26 @ 0x10a26
unsigned long sub_10a26(FILE *a0,long a1,int a2) // early-return x2
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 == -1)
      return 0xffffffff;
    *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
    *(long *)&a0->field_0x90 = v1;
    return 0;
  }
  return CONCAT44(dat_4,fseeko(a0,a1,a2));
}


// Function: sub_10ae4 @ 0x10ae4
void sub_10ae4(int a0,char *a1,unsigned int a2)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa0
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = v2;
  sub_110e3(openat(a0,a1,a2,(unsigned long)v1));
}


// Function: sub_10c22 @ 0x10c22
void sub_10c22(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1);
}


// Function: sub_10c61 @ 0x10c61
void * sub_10c61(DIR *a0,unsigned int a1) // early-return x2, ternary
{
  unsigned long *v1;
  char *v10; // stack - 0x58
  long v11; // stack - 0x50
  long v12; // stack - 0x48
  dirent *v13; // stack - 0x40
  char *v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  char *v16; // stack - 0x28
  char *v2; // rax
  char *v3;
  long v4; // rax
  long v5; // stack - 0x78
  char *v6; // stack - 0x80
  void *v7; // stack - 0x70
  void *v8; // stack - 0x68
  long v9; // stack - 0x60
  
  v7 = NULL;
  v6 = NULL;
  v8 = NULL;
  v5 = 0;
  v9 = 0;
  v10 = NULL;
  v12 = *(long *)((unsigned long)a1 * 8 + 0x18b70);
  if (!a0)
    return NULL;
  while( true ) {
    *__errno_location() = 0;
    v13 = readdir(a0);
    if (!v13) break;
    v14 = &v13->field_0x13;
    if (*v14 != '.') // branch-flip
      v4 = 0;
    else {
      v4 = (v13->field_0x14 != '.') ? 1 : 2; // branch-flip
    }
    if (v14[v4]) {
      v2 = &v13->field_0x13;
      v15 = strlen(v2) + 1;
      if (v12) { // branch-flip
        if (v9 == v5)
          v8 = (void *)sub_1204c(v8,&v5,1,-1,8);
        v1 = (unsigned long *)(v9 * 8 + (long)v8);
        *v1 = sub_12377(v14);
        v9 += 1;
      }
      else {
        if ((long)v6 - (long)v10 <= (long)v15) {
          v4 = v15 - ((long)v6 - (long)v10);
          v7 = (void *)sub_1204c(v7,&v6,v4,0x7ffffffffffffffe,1);
        }
        memcpy(&v10[(long)v7],v14,v15);
      }
      v10 = &v10[v15];
    }
  }
  if (*__errno_location()) {
    free(v8);
    free(v7);
    return NULL;
  }
  if (v12) { // branch-flip
    if (v9)
      qsort(v8,v9,8,v12);
    v3 = &v10[1];
    v7 = (void *)sub_11d96(v3);
    v10 = NULL;
    for (v11 = 0; v11 < v9; v11 = v11 + 1) {
      v16 = (char *)((long)v7 + (long)v10);
      v3 = stpcpy(v16,*(char **)((long)v8 + v11 * 8));
      v10 = &v3[(long)&v10[1 - (long)v16]];
      free(*(void **)((long)v8 + v11 * 8));
    }
    free(v8);
  }
  else if (v10 == v6) {
    v3 = &v10[1];
    v7 = (void *)sub_11e25(v7,v3);
  }
  *(char *)((long)v7 + (long)v10) = '\0';
  return v7;
}


// Function: sub_10f97 @ 0x10f97
void * sub_10f97(char *a0,unsigned int a1) // early-return x2
{
  DIR *v1; // rax
  void *v2; // rax
  
  v1 = (DIR *)sub_123f7(a0);
  if (!v1)
    return NULL;
  v2 = (void *)sub_10c61(v1,a1);
  if (closedir(v1)) {
    free(v2);
    return NULL;
  }
  return v2;
}


// Function: sub_110e3 @ 0x110e3
int sub_110e3(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_12614(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}


// Function: sub_11134 @ 0x11134
int sub_11134(unsigned long *a0) // early-return
{
  int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  v1 = 0;
  v2 = 0;
  if ((((a0[1] == 0x3fffffff) || (a0[1] == 0x3ffffffe)) || ((0 <= (long)a0[1] && ((long)a0[1] <= 999999999)))) && (((a0[3] == 0x3fffffff || (a0[3] == 0x3ffffffe)) || ((0 <= (long)a0[3] && ((long)a0[3] <= 999999999)))))) {
    if ((a0[1] == 0x3fffffff) || (a0[1] == 0x3ffffffe)) {
      *a0 = 0;
      v1 = 1;
      if (a0[1] == 0x3ffffffe)
        v2 = 1;
    }
    if ((a0[3] == 0x3fffffff) || (a0[3] == 0x3ffffffe)) {
      a0[2] = 0;
      v1 = 1;
      if (a0[3] == 0x3ffffffe)
        v2 += 1;
    }
    return v1 + (unsigned int)(v2 == 1);
  }
  *__errno_location() = 0x16;
  return -1;
}


// Function: sub_1129e @ 0x1129e
unsigned long sub_1129e(long a0,long *a1) // return-dupe x2
{
  unsigned long *v1;
  char v2 [16];
  
  v1 = (unsigned long *)*a1;
  if ((v1[1] == 0x3ffffffe) && (v1[3] == 0x3ffffffe))
    return 1;
  if ((v1[1] == 0x3fffffff) && (v1[3] == 0x3fffffff)) {
    *a1 = 0;
    return 0;
  }
  if (v1[1] != 0x3ffffffe) { // branch-flip
    if (v1[1] == 0x3fffffff)
      sub_124c4(v1);
  }
  else {
    v2 = sub_1105b(a0);
    *v1 = SUB168(v2,0);
    v1[1] = SUB168(v2,8);
  }
  if (v1[3] == 0x3ffffffe) {
    v2 = sub_1108f(a0);
    v1[2] = SUB168(v2,0);
    v1[3] = SUB168(v2,8);
    return 0;
  }
  if (v1[3] != 0x3fffffff)
    return 0;
  sub_124c4((timespec *)&v1[2]);
  return 0;
}


// Function: sub_113ca @ 0x113ca
unsigned long sub_113ca(int a0,char *a1,long *a2) // early-return x4
{
  long *v1;
  long *v10; // stack - 0x140
  long v11; // stack - 0xf8
  long v12; // stack - 0xd8
  char v13 [16];
  int v14; // stack - 0x148
  long *v15; // stack - 0x138
  long *v16; // stack - 0x130
  long v17; // stack - 0x128
  long v18; // stack - 0x120
  long v19; // stack - 0x110
  long v2;
  long v20; // stack - 0x108
  long v21; // stack - 0x100
  long v22; // stack - 0xf0
  long v23; // stack - 0xe8
  long v24; // stack - 0xe0
  long v25; // stack - 0xd0
  long v26; // stack - 0xc8
  long v27; // stack - 0xc0
  long v3;
  long *v4;
  int v5;
  unsigned int v6; // eax
  unsigned long v7; // rax
  long v8; // stack - 0x118
  stat v9; // stack - 0xb8
  
  if (a2) // branch-flip
    v10 = &v8;
  else {
    v10 = NULL;
  }
  v14 = 0;
  if (v10) {
    v19 = a2[1];
    v8 = *a2;
    v21 = a2[3];
    v20 = a2[2];
    v14 = sub_11134(v10);
  }
  if (v14 <= -1)
    return 0xffffffff;
  if ((a0 <= -1) && (!a1)) {
    *__errno_location() = 9;
    return 0xffffffff;
  }
  if (0 <= dat_e16a4) {
    if (v14 == 2) {
      if (0 <= a0) // branch-flip
        v5 = fstat(a0,&v9);
      else {
        v5 = stat(a1,&v9);
      }
      v4 = v10;
      if (v5)
        return 0xffffffff;
      if (v10[1] != 0x3ffffffe) { // branch-flip
        if (v10[3] == 0x3ffffffe) {
          v1 = &v10[2];
          v13 = sub_1108f(&v9);
          *v1 = SUB168(v13,0);
          v4[3] = SUB168(v13,8);
        }
      }
      else {
        v13 = sub_1105b(&v9);
        *v4 = SUB168(v13,0);
        v4[1] = SUB168(v13,8);
      }
      v14 = 3;
    }
    if (a0 <= -1) {
      v6 = utimensat(-100,a1,v10,0);
      if (1 <= (int)v6)
        *__errno_location() = 0x26;
      if ((!v6) || (*__errno_location() != 0x26)) {
        dat_e16a4 = 1;
        return (unsigned long)v6;
      }
    }
    if (0 <= a0) {
      v6 = futimens(a0,v10);
      if (1 <= (int)v6)
        *__errno_location() = 0x26;
      if ((!v6) || (*__errno_location() != 0x26)) {
        dat_e16a4 = 1;
        return (unsigned long)v6;
      }
    }
  }
  dat_e16a4 = 0xffffffff;
  dat_e16a8 = 0xffffffff;
  if (v14) {
    if (v14 != 3) {
      if (0 <= a0) // branch-flip
        v5 = fstat(a0,&v9);
      else {
        v5 = stat(a1,&v9);
      }
      if (v5)
        return 0xffffffff;
    }
    if ((v10) && (sub_1129e(&v9,&v10)))
      return 0;
  }
  if (v10) { // branch-flip
    v11 = *v10;
    v22 = v10[1] / 1000;
    v23 = v10[2];
    v24 = v10[3] / 1000;
    v15 = &v11;
  }
  else {
    v15 = NULL;
  }
  if (0 <= a0) { // branch-flip
    if (!futimesat(a0,NULL,v15)) {
      if (v15) {
        v2 = v15[1];
        v3 = v15[3];
        if ((499999 < v2 || 499999 < v3) && (!fstat(a0,&v9))) {
          v17 = v9._72_8_ - *v15;
          v18 = v9._88_8_ - v15[2];
          v16 = NULL;
          v25 = v15[1];
          v12 = *v15;
          v27 = v15[3];
          v26 = v15[2];
          if ((499999 < v2) && ((v17 == 1 && (!sub_11006(&v9)))))
            v16 = &v12;
          if (((499999 < v3) && (v18 == 1)) && (!sub_11032(&v9)))
            v16 = &v12;
          if (v16)
            futimesat(a0,NULL,v16);
        }
      }
      return 0;
    }
    if (!a1)
      return 0xffffffff;
    v7 = CONCAT44(dat_4,utimes(a1,v15));
  }
  else {
    v7 = CONCAT44(dat_4,futimesat(-100,a1,v15));
  }
  return v7;
}


// Function: sub_11a12 @ 0x11a12
void sub_11a12(char *a0,long *a1)
{
  sub_113ca(0xffffffff,a0,a1);
}


// Function: sub_11a3d @ 0x11a3d
unsigned long sub_11a3d(char *a0,unsigned long *a1) // early-return x2
{
  unsigned long *v1;
  int v10; // stack - 0xe8
  unsigned long v11; // stack - 0xd0
  unsigned long v12; // stack - 0xc8
  unsigned long v13; // stack - 0xc0
  unsigned long *v2;
  int v3; // eax
  unsigned int v4; // eax
  unsigned long v5; // rax
  unsigned long v6; // stack - 0xd8
  stat v7; // stack - 0xb8
  unsigned long *v8; // stack - 0xe0
  char v9 [16];
  
  if (a1) // branch-flip
    v8 = &v6;
  else {
    v8 = NULL;
  }
  v10 = 0;
  if (v8) {
    v11 = a1[1];
    v6 = *a1;
    v13 = a1[3];
    v12 = a1[2];
    v10 = sub_11134(v8);
  }
  if (v10 <= -1)
    return 0xffffffff;
  if (0 <= dat_e16a8) {
    if (v10 == 2) {
      v3 = lstat(a0,&v7);
      v2 = v8;
      if (v3)
        return 0xffffffff;
      if (v8[1] != 0x3ffffffe) { // branch-flip
        if (v8[3] == 0x3ffffffe) {
          v1 = &v8[2];
          v9 = sub_1108f(&v7);
          *v1 = SUB168(v9,0);
          v2[3] = SUB168(v9,8);
        }
      }
      else {
        v9 = sub_1105b(&v7);
        *v2 = SUB168(v9,0);
        v2[1] = SUB168(v9,8);
      }
      v10 = 3;
    }
    v4 = utimensat(-100,a0,v8,0x100);
    if (1 <= (int)v4)
      *__errno_location() = 0x26;
    if ((!v4) || (*__errno_location() != 0x26)) {
      dat_e16a4 = 1;
      dat_e16a8 = 1;
      return (unsigned long)v4;
    }
  }
  dat_e16a8 = 0xffffffff;
  if (v10) {
    if ((v10 != 3) && (lstat(a0,&v7)))
      return 0xffffffff;
    if ((v8) && (sub_1129e(&v7,&v8)))
      return 0;
  }
  if ((!v10) && (lstat(a0,&v7)))
    return 0xffffffff;
  if ((v7._24_4_ & 0xf000) != 0xa000) {
    v5 = sub_113ca(0xffffffff,a0,v8);
    return v5;
  }
  *__errno_location() = 0x26;
  return 0xffffffff;
}


// Function: sub_11d4e @ 0x11d4e
long sub_11d4e(long a0)
{
  if (!a0)
    sub_f634(); // no-return
  return a0;
}


// Function: sub_11d70 @ 0x11d70
void sub_11d70(unsigned long a0)
{
  sub_11d4e(malloc(a0));
}


// Function: sub_11d96 @ 0x11d96
void sub_11d96(unsigned long a0)
{
  sub_11d4e(sub_12547(a0));
}


// Function: sub_11dbc @ 0x11dbc
void sub_11dbc(unsigned long a0)
{
  sub_11d70(a0);
}


// Function: sub_11dda @ 0x11dda
void * sub_11dda(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_f634(); // no-return
  return v1;
}


// Function: sub_11e25 @ 0x11e25
void sub_11e25(void *a0,unsigned long a1)
{
  sub_11d4e(sub_12565(a0,a1));
}


// Function: sub_11e56 @ 0x11e56
void * sub_11e56(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_f634(); // no-return
  return v1;
}


// Function: sub_11eb0 @ 0x11eb0
void sub_11eb0(void *a0,unsigned long a1,unsigned long a2)
{
  sub_11d4e(sub_125c5(a0,a1,a2));
}


// Function: sub_11ee9 @ 0x11ee9
void sub_11ee9(unsigned long a0,unsigned long a1)
{
  sub_11e56(NULL,a0,a1);
}


// Function: sub_11f14 @ 0x11f14
void sub_11f14(unsigned long a0,unsigned long a1)
{
  sub_11eb0(NULL,a0,a1);
}


// Function: sub_11f3f @ 0x11f3f
void sub_11f3f(void *a0,unsigned long *a1)
{
  sub_11f6d(a0,a1,1);
}


// Function: sub_11f6d @ 0x11f6d
unsigned long sub_11f6d(void *a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  bool v5; // cf
  unsigned long v6; // stack - 0x18
  
  v6 = *a1;
  if (a0) { // branch-flip
    v3 = (v6 >> 1) + 1;
    v5 = CARRY8(v3,v6);
    v6 = v3 + v6;
    if (v5)
      sub_f634(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_11e56(a0,v6,a2);
  *a1 = v6;
  return v4;
}


// Function: sub_1204c @ 0x1204c
unsigned long sub_1204c(void *a0,long *a1,long a2,long a3,long a4) // ternary
{
  long v1;
  long v2; // rax
  unsigned long v3; // rax
  long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  
  v1 = *a1;
  v4 = v1 + (v1 >> 1);
  if (SCARRY8(v1,v1 >> 1))
    v4 = 0x7fffffffffffffff;
  if ((0 <= a3) && (a3 < v4))
    v4 = a3;
  v5 = v4 * a4;
  if (SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)) // branch-flip
    v2 = 0x7fffffffffffffff;
  else {
    v2 = (0x80 <= (long)v5) ? 0 : 0x80; // branch-flip
  }
  if (v2) {
    v4 = v2 / a4;
    v5 = v2 - v2 % a4;
  }
  if (!a0)
    *a1 = 0;
  if ((v4 - v1 < a2) && ((v4 = a2 + v1, SCARRY8(a2,v1) || (((0 <= a3 && (a3 < v4)) || (v5 = v4 * a4, SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)))))))
    sub_f634(); // no-return
  v3 = sub_11dda(a0,v5);
  *a1 = v4;
  return v3;
}


// Function: sub_12208 @ 0x12208
void sub_12208(unsigned long a0)
{
  sub_1224e(a0,1);
}


// Function: sub_1222b @ 0x1222b
void sub_1222b(unsigned long a0)
{
  sub_1227f(a0,1);
}


// Function: sub_1224e @ 0x1224e
void sub_1224e(unsigned long a0,unsigned long a1)
{
  sub_11d4e(calloc(a0,a1));
}


// Function: sub_1227f @ 0x1227f
void sub_1227f(unsigned long a0,unsigned long a1)
{
  sub_11d4e(sub_1259c(a0,a1));
}


// Function: sub_122b0 @ 0x122b0
void sub_122b0(char *a0,unsigned long a1)
{
  memcpy((void *)sub_11d70(a1),a0,a1);
}


// Function: sub_122e8 @ 0x122e8
void sub_122e8(void *a0,unsigned long a1)
{
  memcpy((void *)sub_11d96(a1),a0,a1);
}


// Function: sub_12328 @ 0x12328
void sub_12328(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_11d96(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}


// Function: sub_12377 @ 0x12377
void sub_12377(char *a0)
{
  sub_122b0(a0,strlen(a0) + 1);
}


// Function: sub_123a8 @ 0x123a8
char sub_123a8(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_123a8
{
  char v1; // al
  int v2; // stack - 0xc
  
  v2 = getchar();
  v1 = !((v2 != 0x79) && (v2 != 0x59));
  while ((v2 != 10 && (v2 != -1))) {
    v2 = getchar();
  }
  return v1;
}


// Function: sub_123f7 @ 0x123f7
DIR * sub_123f7(char *a0)
{
  int v1; // eax
  int v2; // stack - 0x24
  DIR *v3; // stack - 0x18
  DIR *v4; // stack - 0x10
  
  v3 = opendir(a0);
  if (v3) {
    v1 = dirfd(v3);
    if ((0 <= v1) && (v1 <= 2)) {
      v1 = sub_103bd(v1,0x406,3);
      if (0 <= v1) { // branch-flip
        v4 = fdopendir(v1);
        v2 = *__errno_location();
        if (!v4)
          close(v1);
      }
      else {
        v2 = *__errno_location();
        v4 = NULL;
      }
      closedir(v3);
      *__errno_location() = v2;
      v3 = v4;
    }
  }
  return v3;
}


// Function: sub_124c4 @ 0x124c4
void sub_124c4(timespec *a0)
{
  clock_gettime(0,a0);
}


// Function: sub_124e8 @ 0x124e8
undefined16 sub_124e8(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_124c4(v1);
  return v1._0_16_;
}


// Function: sub_12614 @ 0x12614
void sub_12614(unsigned int a0)
{
  sub_103bd(a0,0,3);
}

