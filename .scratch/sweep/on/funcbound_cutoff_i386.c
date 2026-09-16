// Function: sub_80480c0 @ 0x80480c0
void sub_80480c0(void)
{
  int v1;
  int *v2;
  int *v3;
  int v4; // stack + 0x0
  
  dat_804a4b0 = (int *)&(&Stack00000008)[v4 * 4];
  v2 = dat_804a4b0;
  do {
    v3 = &v2[1];
    v1 = *v2;
    v2 = v3;
  } while (v1);
  do {
    v2 = v3;
    if (!*v2) goto label_80480f3;
    v3 = &v2[2];
  } while (*v2 != 0x20);
  dat_804a384 = v2[1];
label_80480f3:
  sub_8048926(sub_80483b0());
  do {
  } while( true );
}

// Function: sub_8048100 @ 0x8048100
unsigned int sub_8048100(int a0,unsigned int a1)
{
  unsigned int v1;
  
  v1 = 0;
  for (; a1; a1 = a1 - 1) {
    v1 ^= ((unsigned int)*(unsigned char *)(a1 + a0) << (signed char)((a1 & 3) << 3)) + ((unsigned int)*(unsigned char *)(a1 + a0) ^ (a1 | 0x42));
  }
  return v1;
}

// Function: sub_8048330 @ 0x8048330
unsigned int sub_8048330(int a0,unsigned int a1,int a2,unsigned int a3)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  unsigned int v3;
  int v4;
  int v5; // ebx
  int v6;
  unsigned int v7; // edi
  bool v8; // zf
  
  v3 = a1;
  v1 = (unsigned int)rdtsc();
  v7 = a3 ^ v1;
  if (a2) {
    v6 = a0 + (a2 + -1) * 4;
    v4 = a2 + -1;
    do {
      v2 = sub_8048968(1,v3,v6,0);
      if (sub_8048968(4,v3,v6,v2 ^ v7 ^ v1) == -1) {
        a0 = 0x804919c;
        a1 = dat_804a3d4;
        v3 = sub_8048c18(); // tail-call
        return v3;
      }
      v6 -= 4;
      v5 = v4 + -1;
      v8 = v4 != 0;
      v4 = v5;
    } while (v8);
  }
  return v3;
}

// Function: sub_80483b0 @ 0x80483b0
unsigned int sub_80483b0(void) // early-return x2
{
  char *v1;
  char *v10; // stack - 0x30
  unsigned int v11; // stack - 0x24
  unsigned int v12 [4]; // stack - 0x20
  char *v13; // esp
  char **v14; // esp
  char *v15; // esp
  int v16; // esi
  char *v17;
  char *v18;
  unsigned int v19; // stack - 0x2c
  char v2;
  int v20; // stack - 0x28
  char *v3;
  int v4; // eax
  int v5; // eax
  unsigned int v6; // eax
  unsigned int v7; // ecx
  int v8; // eax
  char *v9; // ebx
  
  v16 = 0;
  v12[0] = sub_8048100(sub_80483b0,0x559) & 0xffffff;
  v12[1] = sub_8048100(sub_8048968,0x86) & 0xffffff;
  v12[2] = sub_8048100(0x8048820,0xf3) & 0xffffff;
  v12[3] = sub_8048100(sub_8048330,0x8e) & 0xffffff;
  v14 = &v10;
  dat_804a4ac = sub_80488e0();
  if (dat_804a4ac == -1) {
    sub_8048c18(0x80491c0,dat_804a3d4);
    return 1;
  }
  if (!dat_804a4ac) {
    if (sub_8048968(0,0) == -1)
      sub_8048c18(0x80491e0,dat_804a3d4);
    v6 = (*(void *)swi(3))(); // int3-pad
    return v6;
  }
  v19 = (unsigned int)rdtsc();
  v20 = 0;
  while( true ) {
    *(unsigned int **)((int)v14 + -4) = &v11;
    *(unsigned int *)((int)v14 + -8) = 0x80484ed;
    sub_8048b90();
    if (((char)v11 != '\x7f') && (v11 & 0x7f)) break;
    if (!(v11 & 0x7f)) goto label_80487e3;
    switch(v20) {
      case 0:
        v7 = 0;
        v5 = 0xfc;
        do {
          *(unsigned int *)((int)v14 + -4) = v12[v7];
          *(int *)((int)v14 + -8) = v5;
          v5 += 4;
          *(int *)((int)v14 + -0xc) = dat_804a4ac;
          *(unsigned int *)((int)v14 + -0x10) = 6;
          *(unsigned int *)((int)v14 + -0x14) = 0x8048531;
          v16 += sub_8048968();
          v12[v7] = v12[v7] ^ v19;
          v7 += 1;
        } while (v7 <= 3);
        *(unsigned int *)((int)v14 + -4) = 0xfc;
        *(int *)((int)v14 + -8) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0xc) = 3;
        *(unsigned int *)((int)v14 + -0x10) = 0x8048555;
        *(unsigned int *)((int)v14 + -4) = sub_8048968();
        *(unsigned int *)((int)v14 + -8) = 0x26;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        v13 = (char *)((int)v14 + -0x10);
        *(unsigned int *)((int)v14 + -0x10) = 0x8048140;
        break;
      case 1:
        *(unsigned int *)((int)v14 + -4) = v19;
        *(unsigned int *)((int)v14 + -8) = 0x26;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x10) = 0x8048140;
        *(unsigned int *)((int)v14 + -0x14) = 0x8048580;
        sub_8048330();
        v9 = (char *)((unsigned int)((int)v14 + -0x11) & 0xfffffff0);
        *(unsigned int *)((int)v14 + -0x24) = dat_804a454;
        v15 = (char *)((int)v14 + -0x28);
        *(unsigned int *)((int)v14 + -0x28) = 0x80492a3;
        *(unsigned int *)((int)v14 + -0x2c) = 0x804859c;
        sub_8048c18();
        v5 = 4;
        v17 = v9;
        while (v5) {
          v3 = &v17[4];
          v17[0] = '\0';
          v17[1] = '\0';
          v17[2] = '\0';
          v17[3] = '\0';
          v5 -= 1;
          v17 = v3;
        }
        *(unsigned int *)((int)v14 + -0x2c) = dat_804a414;
        *(unsigned int *)((int)v14 + -0x30) = 0x10;
        *(char **)((int)v14 + -0x34) = v9;
        *(unsigned int *)((int)v14 + -0x38) = 0x80485b6;
        sub_8048bb4();
        v7 = 0xffffffff;
        v17 = v9;
        do {
          if (!v7) break;
          v7 -= 1;
          v3 = &v17[1];
          v2 = *v17;
          v17 = v3;
        } while (v2);
        if (~v7 - 1 <= 5) {
          *(unsigned int *)((int)v14 + -0x24) = dat_804a3d4;
          *(unsigned int *)((int)v14 + -0x28) = 0x8049260;
          goto label_80487ff;
        }
        *(unsigned int *)((int)v14 + -0x24) = *(unsigned int *)v9;
        *(unsigned int *)((int)v14 + -0x28) = 0x804a494;
        *(int *)((int)v14 + -0x2c) = dat_804a4ac;
        v13 = (char *)((int)v14 + -0x30);
        *(unsigned int *)((int)v14 + -0x30) = 5;
        *(unsigned int *)((int)v14 + -0x34) = 0x80485e1;
        v5 = sub_8048968();
        *(unsigned int *)((int)v14 + -0x34) = *(unsigned int *)&v9[4];
        *(unsigned int *)((int)v14 + -0x38) = 0x804a498;
        *(int *)((int)v14 + -0x3c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x40) = 5;
        *(unsigned int *)((int)v14 + -0x44) = 0x80485f8;
        v8 = sub_8048968();
        *(unsigned int *)((int)v14 + -0x24) = *(unsigned int *)&v9[8];
        *(unsigned int *)((int)v14 + -0x28) = 0x804a49c;
        *(int *)((int)v14 + -0x2c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x30) = 5;
        *(unsigned int *)((int)v14 + -0x34) = 0x8048612;
        v4 = sub_8048968();
        *(unsigned int *)((int)v14 + -0x34) = *(unsigned int *)&v9[0xc];
        *(unsigned int *)((int)v14 + -0x38) = 0x804a4a0;
        *(int *)((int)v14 + -0x3c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x40) = 5;
        *(unsigned int *)((int)v14 + -0x44) = 0x8048629;
        v16 = v16 + v5 + v8 + v4 + sub_8048968();
        *(unsigned int *)((int)v14 + -0x24) = 0x100;
        *(int *)((int)v14 + -0x28) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x2c) = 3;
        *(unsigned int *)((int)v14 + -0x30) = 0x8048640;
        *(unsigned int *)((int)v14 + -0x24) = sub_8048968();
        *(unsigned int *)((int)v14 + -0x28) = 0x11;
        *(int *)((int)v14 + -0x2c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x30) = 0x80481e0;
        break;
      case 2:
        *(unsigned int *)((int)v14 + -4) = v19 * 2;
        *(unsigned int *)((int)v14 + -8) = 0x11;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x10) = 0x80481e0;
        *(unsigned int *)((int)v14 + -0x14) = 0x804866e;
        sub_8048330();
        *(unsigned int *)((int)v14 + -0x14) = 0x104;
        *(int *)((int)v14 + -0x18) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x1c) = 3;
        *(unsigned int *)((int)v14 + -0x20) = 0x8048680;
        *(unsigned int *)((int)v14 + -0x14) = sub_8048968();
        *(unsigned int *)((int)v14 + -0x18) = 0x2f;
        *(int *)((int)v14 + -0x1c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x20) = 0x8048230;
        goto label_80487bd;
      case 3:
        *(unsigned int *)((int)v14 + -4) = v19 << 2;
        *(unsigned int *)((int)v14 + -8) = 0x2f;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x10) = 0x8048230;
        *(unsigned int *)((int)v14 + -0x14) = 0x80486b1;
        sub_8048330();
        v10 = (char *)((unsigned int)((int)v14 + -0xd) & 0xfffffff0);
        v5 = 10;
        v18 = (char *)((unsigned int)((int)v14 + -0xd) & 0xfffffff0);
        while (v5) {
          v1 = &v18[1];
          *v18 = 0;
          v5 -= 1;
          v18 = v1;
        }
        *(unsigned int *)((int)v14 + -0x20) = dat_804a454;
        *(unsigned int *)((int)v14 + -0x24) = 0x80492c1;
        *(unsigned int *)((int)v14 + -0x28) = 0x80486d8;
        sub_8048c18();
        *(unsigned int *)((int)v14 + -0x28) = dat_804a414;
        *(unsigned int *)((int)v14 + -0x2c) = 10;
        *(char **)((int)v14 + -0x30) = v10;
        *(unsigned int *)((int)v14 + -0x34) = 0x80486e8;
        sub_8048bb4();
        *(unsigned int *)((int)v14 + -0x34) = 0x10;
        *(unsigned int *)((int)v14 + -0x38) = 0;
        *(char **)((int)v14 + -0x3c) = v10;
        *(unsigned int *)((int)v14 + -0x40) = 0x80486f4;
        *(unsigned int *)((int)v14 + -0x34) = sub_80489f0();
        *(unsigned int *)((int)v14 + -0x38) = 0x804a4a4;
        *(int *)((int)v14 + -0x3c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x40) = 5;
        *(unsigned int *)((int)v14 + -0x44) = 0x804870a;
        v5 = sub_8048968();
        v8 = 10;
        v18 = v10;
        while (v8) {
          v1 = &v18[1];
          *v18 = 0;
          v8 -= 1;
          v18 = v1;
        }
        *(unsigned int *)((int)v14 + -0x20) = dat_804a454;
        *(unsigned int *)((int)v14 + -0x24) = 0x80492d2;
        *(unsigned int *)((int)v14 + -0x28) = 0x804872c;
        sub_8048c18();
        *(unsigned int *)((int)v14 + -0x28) = dat_804a414;
        *(unsigned int *)((int)v14 + -0x2c) = 10;
        *(char **)((int)v14 + -0x30) = v10;
        *(unsigned int *)((int)v14 + -0x34) = 0x804873c;
        sub_8048bb4();
        *(unsigned int *)((int)v14 + -0x34) = 0x10;
        *(unsigned int *)((int)v14 + -0x38) = 0;
        *(char **)((int)v14 + -0x3c) = v10;
        *(unsigned int *)((int)v14 + -0x40) = 0x8048748;
        *(unsigned int *)((int)v14 + -0x34) = sub_80489f0();
        *(unsigned int *)((int)v14 + -0x38) = 0x804a4a8;
        *(int *)((int)v14 + -0x3c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x40) = 5;
        *(unsigned int *)((int)v14 + -0x44) = 0x804875e;
        v16 = v16 + v5 + sub_8048968();
        *(unsigned int *)((int)v14 + -0x20) = 0x108;
        *(int *)((int)v14 + -0x24) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x28) = 3;
        *(unsigned int *)((int)v14 + -0x2c) = 0x8048775;
        *(unsigned int *)((int)v14 + -0x20) = sub_8048968();
        *(unsigned int *)((int)v14 + -0x24) = 0xc;
        *(int *)((int)v14 + -0x28) = dat_804a4ac;
        v13 = (char *)((int)v14 + -0x2c);
        *(unsigned int *)((int)v14 + -0x2c) = 0x80482f0;
        break;
      case 4:
        *(unsigned int *)((int)v14 + -4) = 0;
        *(unsigned int *)((int)v14 + -8) = 0x804a380;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x10) = 2;
        *(unsigned int *)((int)v14 + -0x14) = 0x80487a4;
        dat_804a380 = sub_8048968();
        *(unsigned int *)((int)v14 + -0x14) = v19 << 3;
        *(unsigned int *)((int)v14 + -0x18) = 0xc;
        *(int *)((int)v14 + -0x1c) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x20) = 0x80482f0;
label_80487bd:
        *(unsigned int *)((int)v14 + -0x24) = 0x80487c2;
        sub_8048330();
        goto label_80487c5;
      case 5:
        goto label_80487c5;
      default:
        goto label_80487e3;
      
    }
    *(unsigned int *)&v13[-4] = 0x804878b;
    sub_8048330();
    v14 = (char *)&v13[0x10];
label_80487c5:
    *(unsigned int *)((int)v14 + -4) = 0;
    *(unsigned int *)((int)v14 + -8) = 0;
    *(int *)((int)v14 + -0xc) = dat_804a4ac;
    *(unsigned int *)((int)v14 + -0x10) = 7;
    *(unsigned int *)((int)v14 + -0x14) = 0x80487d6;
    v5 = sub_8048968();
    v20 += 1;
    v16 += v5;
  }
  *(unsigned int *)((int)v14 + -4) = dat_804a3d4;
  *(unsigned int *)((int)v14 + -8) = 0x8049286;
  *(unsigned int *)((int)v14 + -0xc) = 0x80484d1;
  sub_8048c18();
label_80487e3:
  if (!v16) {
    *(unsigned int *)((int)v14 + -4) = 0x8048820;
    *(unsigned int *)((int)v14 + -8) = 0x8048815;
    v6 = sub_8048900();
    return v6;
  }
  *(unsigned int *)((int)v14 + -4) = dat_804a3d4;
  v15 = (char *)((int)v14 + -8);
  *(unsigned int *)((int)v14 + -8) = 0x8049220;
label_80487ff:
  *(unsigned int *)&v15[-4] = 0x8048804;
  sub_8048c18();
  return 1;
}

// Function: sub_80488a5 @ 0x80488a5
unsigned int sub_80488a5(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_80488da @ 0x80488da
void sub_80488da(void)
{
  return;
}

// Function: sub_80488e0 @ 0x80488e0
unsigned int sub_80488e0(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_8048900 @ 0x8048900
unsigned int sub_8048900(unsigned int a0) // return-dupe
{
  if (0x20 <= dat_804a540)
    return 0xffffffff;
  *(unsigned int *)(dat_804a540 * 4 + 0x804a4c0) = a0;
  dat_804a540 += 1;
  return 0;
}

// Function: sub_8048926 @ 0x8048926
void sub_8048926(unsigned int a0)
{
  unsigned int v1;
  int v2;
  
  v1 = a0;
  v2 = dat_804a540;
  sub_80488da(a0);
  while (v2) {
label_804894f:
    v2 -= 1;
    (**(void **)(v2 * 4 + 0x804a4c0))();
  }
  sub_80488a5(v1);
  goto label_804894f;
}

// Function: sub_8048960 @ 0x8048960
unsigned int sub_8048960(void)
{
  return 0x804b560;
}

// Function: sub_8048968 @ 0x8048968
unsigned int sub_8048968(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  unsigned int v1; // stack - 0x8
  unsigned int v2;
  unsigned int v3;
  
  dat_804b560 = 0;
  switch(a0) {
    case 0:
    case 8:
    case 0x10:
    case 0x11:
      v3 = 0;
      v2 = 0;
      break;
    case 1:
    case 2:
    case 3:
      v2 = 0xffffffff;
      if (sub_8048d90(a0,a1,a2,&v1) != -1)
        v2 = v1;
      return v2;
    default:
      v2 = a2;
      v3 = a3;
    
  }
  return sub_8048d90(a0,a1,v2,v3);
}

// Function: sub_8048b90 @ 0x8048b90
void sub_8048b90(unsigned int a0)
{
  sub_8048d98(0xffffffff,a0,0);
}

// Function: sub_8048bb4 @ 0x8048bb4
char * sub_8048bb4(char *a0,int a1,unsigned int a2) // early-return
{
  char *v1;
  int v2;
  unsigned int v3;
  int v4; // eax
  int v5;
  char *v6;
  
  v3 = a2;
  v2 = a1;
  v1 = a0;
  v5 = a1;
  v6 = a0;
  if (1 < a1) {
    while (v4 = sub_8048f7c(v3), v4 != -1) {
      *v6 = (char)v4;
      v5 -= 1;
      v6 = &v6[1];
      if ((v4 == 10) || (v5 <= 1)) break;
    }
  }
  if ((v5 != v2) && (!sub_8048dec(v3))) {
    *v6 = 0;
    return v1;
  }
  return NULL;
}

// Function: sub_8048c18 @ 0x8048c18
void sub_8048c18(char *a0,unsigned int a1)
{
  char *v1;
  char v2;
  unsigned int v3; // ecx
  char *v4;
  
  v3 = 0xffffffff;
  v4 = a0;
  do {
    if (!v3) break;
    v3 -= 1;
    v1 = &v4[1];
    v2 = *v4;
    v4 = v1;
  } while (v2);
  sub_8048c58(a0,~v3 - 1,1,a1);
}

// Function: sub_8048c58 @ 0x8048c58
unsigned int sub_8048c58(char *a0,unsigned int a1,unsigned int a2,unsigned int *a3)
{
  char *v1;
  char v2;
  unsigned int v3;
  unsigned int *v4;
  int v5; // eax
  unsigned int v6;
  unsigned int v7; // eax
  unsigned int v8;
  
  v4 = a3;
  v3 = a1;
  v6 = a3[1];
  v7 = a1 * a2;
  if (v6 & 0x100) { // branch-flip
    if ((a2) && (v7 / a2 == a1)) {
      if (((unsigned int)a3[4] <= v7 && v7 - a3[4]) || (v6 & 0x10)) {
        v5 = sub_8048e0b(a3);
        if (v5)
          return 0;
        do {
          v6 = sub_8048da0(*v4,a0,v7);
          if (v6 != 0xffffffff) break;
        } while (*(int *)sub_8048960() == 4);
      }
      else {
        v1 = a0;
        for (v8 = v7; v6 = v7, 1 <= (int)v8; v8 = v8 - 1) {
          v2 = *v1;
          if (sub_8049030(v2,v4)) {
            v6 = v7 - v8;
            goto label_8048cfd;
          }
          v1 = &v1[1];
        }
      }
      if (0 <= (int)v6) {
label_8048cfd:
        v7 = 0;
        if (v3)
          v7 = v6 / v3;
        return v7;
      }
      v4[1] = v4[1] | 1;
    }
  }
  else {
    a3[1] = v6 | 1;
  }
  return 0;
}

// Function: sub_8048d54 @ 0x8048d54
void sub_8048d54(void)
{
  sub_8048e0b(dat_804a3d4);
}

// Function: sub_8048d68 @ 0x8048d68
void sub_8048d68(void)
{
  sub_8048e0b(dat_804a414);
}

// Function: sub_8048d7c @ 0x8048d7c
void sub_8048d7c(void)
{
  sub_8048e0b(dat_804a454);
}

// Function: sub_8048d90 @ 0x8048d90
unsigned int sub_8048d90(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_8048d98 @ 0x8048d98
unsigned int sub_8048d98(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_8048da0 @ 0x8048da0
unsigned int sub_8048da0(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_8048dec @ 0x8048dec
unsigned int sub_8048dec(int a0)
{
  return *(unsigned int *)(a0 + 4) & 1;
}

// Function: sub_8048df8 @ 0x8048df8
void sub_8048df8(void)
{
  sub_8048e0b(0);
}

// Function: sub_8048e0b @ 0x8048e0b
unsigned int sub_8048e0b(unsigned int *a0) // early-return x2, return-dupe
{
  unsigned int *v1;
  int v2;
  int v3; // eax
  unsigned int v4;
  
  v1 = a0;
  if (!a0) {
    sub_8048d68();
    v4 = 0;
    sub_8048d7c();
    sub_8048d54();
    v2 = dat_804b568;
    while (v2) {
      v3 = sub_8048e0b(v2);
      v2 = *(int *)(v2 + 0x18);
      if (v3)
        v4 = 0xffffffff;
    }
    return v4;
  }
  if (*(unsigned char *)&a0[1] & 4) {
    if (a0[3] - a0[2])
      sub_80490ec(*a0,a0[3] - a0[2],1);
    v1[3] = 0;
    v1[2] = 0;
    return 0;
  }
  if ((a0[3]) && (v2 = sub_8048da0(*a0,a0[5],a0[3]), v2 != v1[3])) {
    v1[1] = v1[1] | 1;
    return 0xffffffff;
  }
  v1[3] = 0;
  return 0;
}

// Function: sub_8048ed0 @ 0x8048ed0
unsigned int sub_8048ed0(int *a0,unsigned int a1) // early-return, return-dupe
{
  int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  v2 = a1;
  v1 = a0;
  if (!dat_804b564) {
    dat_804b564 = 1;
    sub_8048900(sub_8048df8);
  }
  if ((v1[1] & 4U) == v2) {
    if (*v1)
      return 0;
    sub_8048d7c();
    return 0;
  }
  v3 = sub_8048e0b(v1);
  v1[1] = v1[1] & 0xfffffffbU | v2;
  return v3;
}

// Function: sub_8048f7c @ 0x8048f7c
unsigned int sub_8048f7c(unsigned int *a0)
{
  unsigned char v1;
  unsigned int *v2;
  int v3; // eax
  unsigned int v4;
  
  v2 = a0;
  if ('\0' <= *(char *)&a0[1]) { // branch-flip
label_804901b:
    v2[1] = v2[1] | 1;
    v4 = 0xffffffff; // return-dupe
  }
  else {
    if (*(char *)((int)a0 + 0x21)) {
      *(char *)((int)a0 + 0x21) = 0;
      return (unsigned int)*(unsigned char *)&a0[8];
    }
    v3 = sub_80490fc(a0);
    if (v3)
      return 0xffffffff;
    if (sub_8048ed0(v2,4))
      return 0xffffffff;
    v4 = v2[3];
    if ((unsigned int)v2[2] <= v4) {
      v3 = sub_80490f4(*v2,v2[5],v2[4]);
      if (!v3) {
        v2[1] = v2[1] | 2;
        v4 = 0xffffffff;
        return v4;
      }
      if (v3 < 0) goto label_804901b;
      v2[3] = 0;
      v2[2] = v3;
      v4 = 0;
    }
    v1 = *(unsigned char *)(v4 + v2[5]);
    v2[3] = v4 + 1;
    v4 = (unsigned int)v1;
  }
  return v4;
}

// Function: sub_8049030 @ 0x8049030
unsigned int sub_8049030(int a0,unsigned int *a1) // return-dupe x2
{
  unsigned int v1;
  unsigned int *v2;
  int v3; // eax
  unsigned int v4;
  
  v2 = a1;
  v4 = a1[1];
  if (v4 & 0x100) {
    v3 = sub_8048ed0(a1,0);
    if ((!v3) && (((unsigned int)v2[3] < (unsigned int)(v2[4] - 1) || (!sub_8048e0b(v2))))) {
      if (*(unsigned char *)&v2[1] & 0x10) { // branch-flip
        v1 = *v2;
        if (sub_8048da0(v1,&a0,1) == 1)
          return 0;
      }
      else {
        *(char *)(v2[3] + v2[5]) = (char)a0;
        v2[3] = v2[3] + 1;
        if ((((!(v2[1] & 8)) || (a0 != 10)) && (!(v2[1] & 0x10))) || (!sub_8048e0b(v2)))
          return 0;
      }
    }
    v4 = v2[1];
  }
  v2[1] = v4 | 1;
  return 0xffffffff;
}

// Function: sub_80490ec @ 0x80490ec
unsigned int sub_80490ec(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_80490f4 @ 0x80490f4
unsigned int sub_80490f4(void) // early-return
{
  unsigned int v1; // eax
  
  v1 = (*dat_804a384)();
  if (v1 <= 0xffffff83)
    return v1;
  *(unsigned int *)sub_8048960() = -v1;
  return 0xffffffff;
}

// Function: sub_80490fc @ 0x80490fc
unsigned int sub_80490fc(int a0)
{
  if (*(char *)(a0 + 0x21))
    return 0;
  return *(unsigned int *)(a0 + 4) & 2;
}

