// Function: _DT_INIT @ 0x1000
void _DT_INIT(void) // return-dupe
{
  if (!dat_3fe8)
    return;
  (*dat_3fe8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_3fd0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1030
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: sub_1040 @ 0x1040
void sub_1040(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_1070 @ 0x1070
void sub_1070(void)
{
  return;
}

// Function: _FINI_0 @ 0x10e0
void _FINI_0(void)
{
  if (!dat_4010) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    sub_1070();
    dat_4010 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x1120
void _INIT_0(void)
{
  return;
}

// Function: sub_1129 @ 0x1129
int sub_1129(int a0)
{
  return (a0 + 10) * 2;
}

// Function: sub_1141 @ 0x1141
int sub_1141(int a0)
{
  return (a0 + 0xb) * 2 + 1;
}

// Function: sub_115c @ 0x115c
int sub_115c(int a0)
{
  return (a0 + 0xc) * 2 + 2;
}

// Function: sub_1177 @ 0x1177
int sub_1177(int a0)
{
  return (a0 + 0xd) * 2 + 3;
}

// Function: sub_1192 @ 0x1192
int sub_1192(int a0)
{
  return (a0 + 0xe) * 2 + 4;
}

// Function: sub_11ad @ 0x11ad
int sub_11ad(int a0)
{
  return (a0 + 0xf) * 2 + 5;
}

// Function: sub_11c8 @ 0x11c8
int sub_11c8(int a0)
{
  return (a0 + 0x10) * 2 + 6;
}

// Function: sub_11e3 @ 0x11e3
int sub_11e3(int a0)
{
  return (a0 + 0x11) * 2 + 7;
}

// Function: sub_11fe @ 0x11fe
int sub_11fe(int a0)
{
  return (a0 + 0x12) * 2 + 8;
}

// Function: sub_1219 @ 0x1219
int sub_1219(int a0)
{
  return (a0 + 0x13) * 2 + 9;
}

// Function: sub_1234 @ 0x1234
int sub_1234(int a0)
{
  return (a0 + 0x14) * 2 + 10;
}

// Function: sub_124f @ 0x124f
int sub_124f(int a0)
{
  return (a0 + 0x15) * 2 + 0xb;
}

// Function: sub_126a @ 0x126a
int sub_126a(int a0)
{
  return (a0 + 0x16) * 2 + 0xc;
}

// Function: sub_1285 @ 0x1285
int sub_1285(int a0)
{
  return (a0 + 0x17) * 2 + 0xd;
}

// Function: sub_12a0 @ 0x12a0
int sub_12a0(int a0)
{
  return (a0 + 0x18) * 2 + 0xe;
}

// Function: sub_12bb @ 0x12bb
int sub_12bb(int a0)
{
  return (a0 + 0x19) * 2 + 0xf;
}

// Function: sub_12d6 @ 0x12d6
int sub_12d6(int a0)
{
  return (a0 + 0x1a) * 2 + 0x10;
}

// Function: sub_12f1 @ 0x12f1
int sub_12f1(int a0)
{
  return (a0 + 0x1b) * 2 + 0x11;
}

// Function: sub_130c @ 0x130c
int sub_130c(int a0)
{
  return (a0 + 0x1c) * 2 + 0x12;
}

// Function: sub_1327 @ 0x1327
int sub_1327(int a0)
{
  return (a0 + 0x1d) * 2 + 0x13;
}

// Function: sub_1342 @ 0x1342
int sub_1342(int a0)
{
  return (a0 + 0x1e) * 2 + 0x14;
}

// Function: sub_135d @ 0x135d
int sub_135d(int a0)
{
  return (a0 + 0x1f) * 2 + 0x15;
}

// Function: sub_1378 @ 0x1378
int sub_1378(int a0)
{
  return (a0 + 0x20) * 2 + 0x16;
}

// Function: sub_1393 @ 0x1393
int sub_1393(int a0)
{
  return (a0 + 0x21) * 2 + 0x17;
}

// Function: sub_13ae @ 0x13ae
int sub_13ae(int a0)
{
  return (a0 + 0x40) * 2 + 9;
}

// Function: main @ 0x13c9
int main(int argc,char **argv)
{
  unsigned int v1;
  int v10; // eax
  int v11; // eax
  int v12; // eax
  int v13; // eax
  int v14; // eax
  int v15; // eax
  int v16; // eax
  int v17; // eax
  int v18; // eax
  int v19; // eax
  int v2; // eax
  int v20; // eax
  int v21; // eax
  int v22; // eax
  int v23; // eax
  int v24; // eax
  int v25; // eax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  
  v1 = dat_4014;
  v2 = (**(void **)((long)(int)(dat_4014 & 1) * 8 + 0x3df0))(dat_4014);
  v3 = sub_1129(v1);
  v4 = sub_1141(v1);
  v5 = sub_115c(v1);
  v6 = sub_1177(v1);
  v7 = sub_1192(v1);
  v8 = sub_11ad(v1);
  v9 = sub_11c8(v1);
  v10 = sub_11e3(v1);
  v11 = sub_11fe(v1);
  v12 = sub_1219(v1);
  v13 = sub_1234(v1);
  v14 = sub_124f(v1);
  v15 = sub_126a(v1);
  v16 = sub_1285(v1);
  v17 = sub_12a0(v1);
  v18 = sub_12bb(v1);
  v19 = sub_12d6(v1);
  v20 = sub_12f1(v1);
  v21 = sub_130c(v1);
  v22 = sub_1327(v1);
  v23 = sub_1342(v1);
  v24 = sub_135d(v1);
  v25 = sub_1378(v1);
  return v2 + v3 + v4 + v5 + v6 + v7 + v8 + v9 + v10 + v11 + v12 + v13 + v14 + v15 + v16 + v17 + v18 + v19 + v20 + v21 + v22 + v23 + v24 + v25 + sub_1393(v1);
}

// Function: _DT_FINI @ 0x1674
void _DT_FINI(void)
{
  return;
}

