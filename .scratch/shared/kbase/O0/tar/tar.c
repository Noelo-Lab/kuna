// Function: sub_ab09 @ 0xab09
void sub_ab09(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long *v1; // rax
  
  if (!dat_9f8fc)
    return;
  v1 = (unsigned long *)sub_60b5b(0x30);
  if (dat_9f068) // branch-flip
    *dat_9f068 = v1;
  else {
    dat_9f060 = v1;
  }
  *v1 = 0;
  v1[1] = dat_9efb0 - dat_9efa0 >> 9;
  dat_9f068 = v1;
  v1[2] = sub_60cc6(a0);
  v1[3] = a1;
  v1[4] = a2;
  v1[5] = 0;
  return;
}


// Function: sub_abd9 @ 0xabd9
int8 * sub_abd9(uint8 a0)
{
  int8 *v1; // stack - 0x10
  
  v1 = dat_9f060;
  while (((v1 && (*v1)) && ((uint8)(*(int8 *)(*v1 + 8) << 9) <= a0))) {
    v1 = (int8 *)*v1;
  }
  return v1;
}


// Function: sub_ac2b @ 0xac2b
void sub_ac2b(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = dat_9f060;
  while ((v2 && (v2 != a0))) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[2]);
    free(v2);
    v2 = v1;
  }
  dat_9f060 = v2;
  if (v2)
    return;
  dat_9f068 = v2;
  return;
}


// Function: sub_acb0 @ 0xacb0
void sub_acb0(unsigned long *a0,int8 a1) // return-dupe
{
  unsigned long *v1; // stack - 0x10
  
  sub_ac2b(a0);
  v1 = a0;
  if (!a0)
    return;
  for (; v1; v1 = (unsigned long *)*v1) {
    v1[1] = v1[1] + a1;
    v1[5] = 0;
  }
  return;
}


// Function: sub_ad11 @ 0xad11
void sub_ad11(void)
{
  sub_1d3e6(0x9f1e8);
  return;
}


// Function: sub_ad2b @ 0xad2b
void sub_ad2b(unsigned long *a0)
{
  sub_ab09(*a0,a0[0x11],a0[0x11]);
  return;
}


// Function: sub_ad66 @ 0xad66
void sub_ad66(void) // return-dupe
{
  if (!dat_9f8fc)
    return;
  sub_ac2b(0);
  return;
}


// Function: sub_ad86 @ 0xad86
void sub_ad86(unsigned long a0) // return-dupe
{
  if (!dat_9f060)
    return;
  *(unsigned long *)(dat_9f060 + 0x20) = a0;
  return;
}


// Function: sub_adb0 @ 0xadb0
void sub_adb0(void)
{
  dat_9f044 = 0;
  return;
}


// Function: sub_adc5 @ 0xadc5
void sub_adc5(void)
{
  sub_521fa(0x9f9e0);
  dat_9f9f0 = dat_9f9e0;
  dat_9f9f8 = dat_9f9e8;
  dat_9fa00 = dat_9f9e0;
  dat_9fa08 = dat_9f9e8;
  return;
}


// Function: sub_ae17 @ 0xae17
void sub_ae17(void)
{
  sub_521fa(0x9f9f0);
  dat_9fa00 = dat_9f9f0;
  dat_9fa08 = dat_9f9f8;
  return;
}


// Function: sub_ae4d @ 0xae4d
float8 sub_ae4d(void)
{
  int8 v1; // stack - 0x28
  int8 v2; // stack - 0x20
  
  sub_521fa(&v1);
  dat_9f238 = dat_9f238 + ((float8)(v1 - dat_9fa00) + (float8)(v2 - dat_9fa08) / dat_80790);
  sub_521fa(0x9fa00);
  return dat_9f238;
}


// Function: sub_aefd @ 0xaefd
int8 sub_aefd(int4 a0,int4 *a1)
{
  int4 v1; // stack - 0xc
  
  v1 = *a1;
  while( true ) {
    if (!*(int4 *)((int8)v1 * 0x18 + 0x9c020)) {
      *a1 = v1;
      return 0;
    }
    if (a0 == *(int4 *)((int8)v1 * 0x18 + 0x9c020)) break;
    v1 += 1;
  }
  *a1 = v1 + 1;
  return (int8)v1 * 0x18 + 0x9c020;
}


// Function: sub_afa2 @ 0xafa2
int8 sub_afa2(unsigned int *a0) // early-return x2
{
  int8 v1; // rax
  
  if (dat_9f898) // branch-flip
    v1 = dat_9f898;
  else {
    if (!dat_9f240) {
      v1 = 0;
      return v1;
    }
    *a0 = 0;
    v1 = sub_aefd(dat_9f240,a0);
    if (!v1) {
      v1 = 0;
      return v1;
    }
    v1 = *(int8 *)(v1 + 8);
  }
  return v1;
}


// Function: sub_b012 @ 0xb012
unsigned long sub_b012(unsigned long a0) // early-return x2
{
  int8 v1; // rax
  
  if (dat_9f898)
    return 0;
  v1 = sub_aefd(dat_9f240,a0);
  if (v1)
    return *(unsigned long *)(v1 + 8);
  return 0;
}


// Function: sub_b065 @ 0xb065
unsigned long sub_b065(unsigned int a0) // early-return
{
  int8 v1; // rax
  unsigned int v2 [3]; // stack - 0x1c
  
  v2[0] = 0;
  v1 = sub_aefd(a0,v2);
  if (v1)
    return *(unsigned long *)(v1 + 0x10);
  return 0;
}


// Function: sub_b0cb @ 0xb0cb
unsigned int sub_b0cb(char *a0) // return-dupe
{
  int4 v1; // eax
  char *v2; // rax
  int8 v3;
  char *v4; // stack - 0x30
  char v5; // stack - 0x19
  unsigned int *v6; // stack - 0x18
  
  v4 = a0;
  if (!a0)
    v4 = &v3;
  dat_9efa8 = dat_9efa0;
  v5 = dat_9f049;
  dat_9f049 = 1;
  *v4 = sub_bb4b() == 0;
  dat_9f049 = v5;
  if ((dat_9efa0 != dat_9efa8) && (((v2 = (char *)((int8)dat_9efa0 + 0x101), !strcmp(v2,"ustar") || (v2 = (char *)((int8)dat_9efa0 + 0x101), !strcmp(v2,"ustar  "))) && (v1 = sub_25d84(dat_9efa0,1), v1 == 1))))
    return 1;
  v6 = (unsigned int *)0x9a170;
  while( true ) {
    if ((unsigned int *)0x9a230 <= v6)
      return 0;
    v1 = memcmp(dat_9efa0,*(void **)&v6[4],*(uint8 *)&v6[2]);
    if (!v1) break;
    v6 = &v6[6];
  }
  return *v6;
}


// Function: sub_b214 @ 0xb214
void sub_b214(void) // return-dupe x2
{
  int4 v1; // eax
  char v2 [24];
  uint4 v3; // stack - 0x90
  
  if (dat_9f860 == 4)
    dat_9fc18 = 0;
  if (dat_9fc14 != -1) {
    dat_9fc18 = dat_9fc14 != 0;
    return;
  }
  if (((dat_9f8fc != '\x01') && (!dat_9f898)) && (v1 = fstat(dat_9f9cc,v2), !v1)) {
    dat_9fc18 = (v3 & 0xf000) == 0x8000;
    return;
  }
  dat_9fc18 = 0;
  return;
}


// Function: sub_b2cf @ 0xb2cf
int4 sub_b2cf(void)
{
  int4 v1; // eax
  char *v2; // rax
  char v3; // stack - 0x15
  int4 v4; // stack - 0x14
  
  if ((((dat_9fdf8 != '\x01') && (v2 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0 = v2, v2)) && ((char *)*dat_9fbd8 < v2)) && (v2 = (uint8)((int8)v2 - *dat_9fbd8), v2 = memchr((void *)*dat_9fbd8,0x2f,(uint8)v2), !v2))
    v1 = sub_3f860(*dat_9fbd8,0,0x40000000,dat_9f950);
  else {
    v1 = open((char *)*dat_9fbd8,0,0x1b6);
  }
  dat_9f9cc = v1;
  if (v1 == -1) {
    v1 = -1;
    return v1;
  }
  if (dat_9f8fc != '\x01') {
    if (!dat_9f898) {
      v1 = sub_b0cb(&v3);
      v4 = v1;
      if (v1) { // branch-flip
        if (v1 == 1) {
          if (v3) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v2 = gettext("This does not look like a tar archive");
            error(0,0,v2);
            dat_9fdd0 = 2;
          }
          v1 = dat_9f9cc;
          return v1;
        }
        dat_9f240 = v1;
      }
      else {
        if (v3) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v2 = gettext("This does not look like a tar archive");
          error(0,0,v2);
          dat_9fdd0 = 2;
        }
        sub_32970(*dat_9fbd8,0);
        if (!dat_9f898) {
          v1 = dat_9f9cc;
          return v1;
        }
      }
    }
    if (0x40000000 <= dat_9f9cc) { // branch-flip
      v1 = dat_9f9cc + -0x40000000;
      sub_3fe0c(v1);
    }
    else {
      close(dat_9f9cc);
    }
    dat_9f048 = 0;
    v1 = sub_3399b();
    dat_9f049 = 1;
    dat_9f040 = v1;
  }
  dat_9efc0 = 0;
  dat_9efa8 = dat_9efa0;
  v1 = dat_9f9cc;
  return v1;
}


// Function: sub_b570 @ 0xb570
int4 sub_b570(float8 a0,void *a1,char *a2)
{
  uint8 v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char v4 [664];
  char v5 [656];
  float8 v6; // xmm0_qa
  int4 v7; // stack - 0x550
  
  v7 = 0;
  if ((a2) && (*a2))
    v7 = fprintf(a1,"%s: ",gettext(a2));
  if ((dat_9f238 <= 0.0) || (dat_80798 <= a0 / dat_9f238))
    v2 = 0x800c3;
  else {
    v6 = a0 / dat_9f238;
    if (dat_807a0 <= v6)
      v1 = (int8)(v6 - dat_807a0) ^ 0x8000000000000000;
    else {
      v1 = (uint8)v6;
    }
    v2 = sub_53f1c(v1,v4,0x1b0,1,1);
  }
  if (dat_807a0 <= a0)
    v1 = (int8)(a0 - dat_807a0) ^ 0x8000000000000000;
  else {
    v1 = (uint8)a0;
  }
  v3 = sub_53f1c(v1,v5,0x1b0,1,1);
  return fprintf(a1,"%.0f (%s, %s/s)",v3,v2) + v7;
}


// Function: sub_b768 @ 0xb768
int4 sub_b768(void *a0,unsigned long *a1,unsigned int a2,int4 a3)
{
  int4 v1; // eax
  int4 v2; // eax
  char *v3; // rax
  uint8 v4; // rax
  unsigned long v5;
  char v6 [24];
  float8 v7; // xmm0_qa
  int4 v8; // stack - 0x3c
  
  if (dat_9f860 != 8) {
    if ((8 < dat_9f860) || (7 < dat_9f860)) {
      abort(); // no-return, return-dupe
    }
    if (5 <= dat_9f860) {
      v3 = gettext((char *)*a1);
      v8 = sub_b570(a0,v3);
      goto label_ba6a;
    }
    if (3 < dat_9f860) {
      if (dat_9f860 != 4) {
        abort();
      }
      v1 = sub_b570(a0,*a1);
      fputc_unlocked(a2,a0);
      v2 = sub_b570(a0,a1[1],a1[1],dat_9f000 + dat_9f008);
      fputc_unlocked(a2,a0);
      v8 = v1 + v2 + 2;
      if ((a1[2]) && (*(char *)a1[2])) {
        v3 = gettext((char *)a1[2]);
        v8 += fprintf(a0,"%s: ",v3);
      }
      v7 = (float8)(uint8)(dat_9f870 * (dat_9efc0 - dat_9f370)) - (dat_9f008 + dat_9f000);
      if (dat_807a0 <= v7)
        v4 = (int8)(v7 - dat_807a0) ^ 0x8000000000000000;
      else {
        v4 = (uint8)v7;
      }
      v5 = sub_54e5a(v4,v6);
      v8 += fprintf(a0,"%s",v5);
      goto label_ba6a;
    }
    if (!dat_9f860)
      abort();
  }
  v7 = dat_9f000 + dat_9f008;
  v5 = a1[1];
  v8 = sub_b570(a0,v5,v5,v7);
label_ba6a:
  if (a3) {
    fputc_unlocked(a3,a0);
    v8 += 1;
  }
  return v8;
}


// Function: sub_baa2 @ 0xbaa2
void sub_baa2(void)
{
  sub_b768(stderr,0x9a230,10,10);
  return;
}


// Function: sub_bacd @ 0xbacd
int8 sub_bacd(void)
{
  return dat_9f038 + (dat_9efb0 - dat_9efa0 >> 9);
}


// Function: sub_baf9 @ 0xbaf9
void sub_baf9(void) // return-dupe
{
  if (!dat_9f048)
    return;
  dat_9efa8 = (int8)dat_9f868 * 0x200 + dat_9efa0;
  dat_9efb0 = dat_9efa0;
  dat_9efb8 = 1;
  dat_9f048 = 0;
  return;
}


// Function: sub_bb4b @ 0xbb4b
int8 sub_bb4b(void)
{
  if (dat_9efb0 == dat_9efa8) {
    if (dat_9f048)
      return 0;
    sub_cac8();
    if (dat_9efb0 == dat_9efa8) {
      dat_9f048 = 1;
      return 0;
    }
  }
  return dat_9efb0;
}


// Function: sub_bba7 @ 0xbba7
void sub_bba7(uint8 a0)
{
  for (; dat_9efb0 <= a0; dat_9efb0 = dat_9efb0 + 0x200) {
  }
  if (dat_9efa8 < dat_9efb0)
    abort(); // no-return
  return;
}


// Function: sub_bbf5 @ 0xbbf5
int8 sub_bbf5(int8 a0)
{
  return dat_9efa8 - a0;
}


// Function: sub_bc11 @ 0xbc11
void sub_bc11(int4 a0) // return-dupe
{
  if (!close(a0))
    return;
  sub_3e1ff(gettext("(pipe)"));
  return;
}


// Function: sub_bc48 @ 0xbc48
void sub_bc48(void)
{
  int4 v1;
  unsigned long v2; // rax
  
  v1 = dat_9f030;
  if (!*(int8 *)((int8)dat_9f030 * 8 + 0x9f020)) {
    v2 = sub_2b601((int8)dat_9f030 * 8 + 0x9f010,dat_9f870);
    *(unsigned long *)((int8)v1 * 8 + 0x9f020) = v2;
  }
  dat_9efa0 = *(int8 *)((int8)dat_9f030 * 8 + 0x9f020);
  dat_9efb0 = dat_9efa0;
  dat_9efa8 = (int8)dat_9f868 * 0x200 + dat_9efa0;
  return;
}


// Function: sub_bd12 @ 0xbd12
void sub_bd12(int4 a0)
{
  char *v1;
  
  v1 = (char *)*dat_9fbd8;
  if (!strcmp(v1,"-")) {
    if (isatty((uint4)(a0 != 0))) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      if (a0) // branch-flip
        v1 = gettext("Refusing to write archive contents to terminal (missing -f option?)");
      else {
        v1 = gettext("Refusing to read archive contents from terminal (missing -f option?)");
      }
      error(0,0,v1);
      sub_177f4(); // no-return
    }
  }
  return;
}


// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(uint4 a0) // return-dupe x2
{
  char *v1;
  void *v2;
  unsigned long v3; // rax
  uint8 v4; // rax
  char v5; // stack - 0x2d
  int4 v6; // stack - 0x2c
  int4 v7; // stack - 0x28
  int4 v8; // stack - 0x24
  
  v6 = 0;
  if (!dat_9f870) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Invalid value for record_size"));
    sub_177f4(); // no-return
  }
  if (!dat_9fbe0) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("No archive name given"));
    sub_177f4(); // no-return
  }
  sub_39d1c(0x9fa20);
  dat_9f030 = 0;
  sub_bc48();
  if (a0 != 2) // branch-flip
    dat_9efb8 = a0;
  else {
    dat_9efb8 = 0;
  }
  sub_bd12(dat_9efb8);
  dat_9f049 = dat_9f94d;
  dat_9efc0 = 0;
  if (dat_9f898) { // branch-flip
    switch(a0) {
      case 0:
        dat_9f040 = sub_3399b();
        dat_9f049 = 1;
        dat_9efa8 = dat_9efa0;
        break;
      case 1:
        dat_9f040 = sub_3321b();
        break;
      case 2:
        abort(); // no-return
      
    }
    if (((!dat_9fbf0) && (a0 == 1)) && (v1 = (char *)*dat_9fbd8, !strcmp(v1,"-")))
      dat_9efd0 = stderr;
  }
  else {
    v1 = (char *)*dat_9fbd8;
    if (strcmp(v1,"-")) { // branch-flip
      if (a0 != 2) { // branch-flip
        if (a0 <= 2) {
          if (a0) { // branch-flip
            if (a0 == 1) {
              if (dat_9f884) {
                sub_2a571(*dat_9fbd8,1);
                v6 = 1;
              }
              if (dat_9f9b0) { // branch-flip
                if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbd8 < dat_9fdf0 && (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v2 = (void *)*dat_9fbd8, !memchr(v2,0x2f,v4)))))
                  dat_9f9cc = sub_3f860(*dat_9fbd8,0x42,0x40000000,dat_9f950);
                else {
                  dat_9f9cc = open((char *)*dat_9fbd8,0x42,0x1b6);
                }
              }
              else if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbd8 < dat_9fdf0 && (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v2 = (void *)*dat_9fbd8, !memchr(v2,0x2f,v4)))))
                dat_9f9cc = sub_3f860(*dat_9fbd8,0x41,0x40000000,dat_9f950);
              else {
                dat_9f9cc = creat(*dat_9fbd8,0x1b6);
              }
            }
          }
          else {
            dat_9f9cc = sub_b2cf();
            if (0 <= dat_9f9cc)
              sub_b214();
          }
        }
      }
      else {
        if ((((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0)) && ((char *)*dat_9fbd8 < dat_9fdf0)) && (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v2 = (void *)*dat_9fbd8, !memchr(v2,0x2f,v4)))
          dat_9f9cc = sub_3f860(*dat_9fbd8,0x42,0x40000000,dat_9f950);
        else {
          dat_9f9cc = open((char *)*dat_9fbd8,0x42,0x1b6);
        }
        if (2 <= (uint4)sub_b0cb(0)) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Cannot update compressed archives"));
          sub_177f4(); // no-return
        }
      }
    }
    else {
      dat_9f049 = 1;
      if (dat_9f9b0) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Cannot verify stdin/stdout archive"));
        sub_177f4(); // no-return
      }
      if (a0 != 2) { // branch-flip
        if (a0 <= 2) {
          if (a0) { // branch-flip
            if ((a0 == 1) && (dat_9f9cc = 1, !dat_9fbf0))
              dat_9efd0 = stderr;
          }
          else {
            dat_9f9cc = 0;
            v7 = sub_b0cb(&v5);
            if ((v7 != 1) && (v7)) {
              if (dat_9fdc8)
                (*dat_9fdc8)();
              v3 = sub_b065(v7);
              error(0,0,gettext("Archive is compressed. Use %s option"),v3);
              sub_177f4(); // no-return
            }
            if (v5) {
              if (dat_9fdc8)
                (*dat_9fdc8)();
              error(0,0,gettext("This does not look like a tar archive"));
              dat_9fdd0 = 2;
            }
          }
        }
      }
      else {
        dat_9f9cc = 0;
        dat_9efd8 = 1;
        dat_9efa8 = dat_9efa0;
        if (!dat_9fbf0)
          dat_9efd0 = stderr;
      }
    }
  }
  if ((dat_9f9cc <= -1) || ((dat_9f9cc <= 0x3fffffff && (sub_32b11() != '\x01')))) {
    v8 = *__errno_location();
    if (v6)
      sub_2a874();
    *__errno_location() = v8;
    sub_3e3b4(*dat_9fbd8); // no-return
  }
  sub_32bdc();
  sub_32b88();
  if (!a0) {
    sub_bb4b();
    return;
  }
  if (2 > a0 - 1) {
    dat_9efc8 = 0;
    return;
  }
  return;
}


// Function: sub_c54f @ 0xc54f
int8 sub_c54f(void) // ternary
{
  int8 v1; // stack - 0x28
  unsigned long *v2; // stack - 0x20
  uint8 v3; // stack - 0x18
  int8 v4; // stack - 0x10
  
  sub_106d8(1);
  if ((dat_9f990 == 0.0) || (dat_9f008 < dat_9f990)) {
    v1 = (dat_9f9d0) ? dat_9f870 : sub_3301a(); // branch-flip
  }
  else {
    *__errno_location() = 0x1c;
    v1 = 0;
  }
  if (((v1) && (dat_9f8fc)) && (!dat_9f070)) {
    v2 = (unsigned long *)sub_abd9(v1);
    if (v2) {
      v3 = v1 + v2[1] * -0x200;
      v2[5] = v2[5] + (v3 >> 9);
      if ((uint8)v2[4] < v3)
        v3 = v2[4];
      v2[4] = v2[4] - v3;
      if (v2[4]) // branch-flip
        v4 = v2[1];
      else {
        v4 = v2[5] + v2[1];
        v2 = (unsigned long *)*v2;
      }
      sub_acb0(v2,-v4);
    }
  }
  return v1;
}


// Function: sub_c6d5 @ 0xc6d5
void sub_c6d5(unsigned long a0)
{
  int4 v1;
  
  if (dat_9f999) {
    v1 = *__errno_location();
    sub_baa2();
    *__errno_location() = v1;
  }
  sub_d132(*dat_9fbe8,a0,dat_9f870); // no-return
}


// Function: sub_c729 @ 0xc729
void sub_c729(void)
{
  int4 v1;
  
  sub_3e403(*dat_9fbe8);
  v1 = dat_9f044;
  if (!dat_9f038) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("At beginning of tape, quitting now"));
    sub_177f4(); // no-return
  }
  dat_9f044 += 1;
  if (0xb <= v1) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Too many errors, quitting"));
    sub_177f4(); // no-return
  }
  return;
}


// Function: sub_c7e6 @ 0xc7e6
unsigned int sub_c7e6(void) // return-dupe
{
  int4 v1; // eax
  char v2 [24];
  uint4 v3; // stack - 0x90
  
  v1 = fstat(dat_9f9cc,v2);
  if (v1) {
    sub_2b444(*dat_9fbe8);
    return 0;
  }
  if (((v3 & 0xf000) != 0x6000) && ((v3 & 0xf000) != 0x2000))
    return 0;
  return 1;
}


// Function: sub_c881 @ 0xc881
void sub_c881(uint8 a0)
{
  int8 v1; // rax
  uint8 v2; // stack - 0x30
  uint8 v3; // stack - 0x28
  int8 v4; // stack - 0x20
  
  v4 = a0 + dat_9efa0;
  v3 = dat_9f870 - a0;
  v2 = a0;
  if (((((v3) && (!(v3 & 0x1ff))) && (dat_9ec84 & 0x400000)) && ((!dat_9f038 && (a0)))) && (sub_c7e6())) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,(char *)ngettext("Record size = %lu block","Record size = %lu blocks",a0 >> 9),a0 >> 9);
  }
  while ((v3 & 0x1ff || (((v3 && (v2)) && (dat_9f049))))) {
    if (v2) {
      while( true ) {
        if (0x40000000 <= dat_9f9cc) // branch-flip
          v2 = sub_3fe62(dat_9f9cc + -0x40000000,v4,v3);
        else {
          v2 = sub_5ec70(dat_9f9cc,v4,v3);
        }
        if (v2 != 0xffffffffffffffff) break;
        sub_c729();
      }
    }
    if (!v2) break;
    if (dat_9f049 != '\x01') {
      v1 = dat_9f870 - v3;
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,(char *)ngettext("Unaligned block (%lu byte) in archive","Unaligned block (%lu bytes) in archive",v1),v1);
      sub_177f4(); // no-return
    }
    v3 -= v2;
    v4 += v2;
  }
  dat_9efa8 = (dat_9f870 - v3 & 0xfffffffffffffe00) + dat_9efa0;
  dat_9efc0 += 1;
  return;
}


// Function: sub_cac8 @ 0xcac8
void sub_cac8(void) // return-dupe x4
{
  int8 v1; // rax
  
  if ((!dat_9efb8) && (dat_9fd70)) {
    dat_9efb8 = 1;
    dat_9fd70 = '\0';
    sub_cc3a();
    if ((int8)dat_9efa8 - dat_9efa0 >> 9 < (int8)dat_9f868) {
      memset(dat_9efa8,0,((int8)dat_9f868 - ((int8)dat_9efa8 - dat_9efa0 >> 9)) * 0x200);
      dat_9efa8 = (void *)((int8)dat_9f868 * 0x200 + dat_9efa0);
      return;
    }
  }
  v1 = dat_9efb0 - dat_9efa0;
  dat_9f038 += (int8)dat_9efa8 - dat_9efa0 >> 9;
  dat_9efb0 = dat_9efa0;
  dat_9efa8 = (int8)((int8)dat_9f868 * 0x200 + dat_9efa0);
  if (dat_9efb8 == 2)
    abort(); // no-return
  if (3 <= dat_9efb8) {
    dat_9efb0 = dat_9efa0;
    return;
  }
  if (dat_9efb8) {
    if (dat_9efb8 != 1) {
      dat_9efb0 = dat_9efa0;
      return;
    }
    (*dat_9f050)(v1);
    return;
  }
  sub_f662();
  return;
}


// Function: sub_cc3a @ 0xcc3a
void sub_cc3a(void) // return-dupe x3
{
  char v1;
  int4 v2; // eax
  int8 v3; // rax
  unsigned short v4 [2]; // stack - 0x18
  int8 v5; // stack - 0x20
  unsigned int v6; // stack - 0x14
  
  v4[0] = 4;
  v6 = 1;
  if (0x40000000 <= dat_9f9cc) { // branch-flip
    sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v4);
    v1 = dat_3;
  }
  else {
    v2 = ioctl(dat_9f9cc,0x40086d01,v4);
    v1 = (char)((uint4)v2 >> 0x18);
  }
  if ('\0' <= v1)
    return;
  if (*__errno_location() == 5) {
    if (0x40000000 <= dat_9f9cc) { // branch-flip
      sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v4);
      v1 = dat_3;
    }
    else {
      v2 = ioctl(dat_9f9cc,0x40086d01,v4);
      v1 = (char)((uint4)v2 >> 0x18);
    }
    if ('\xff' < v1)
      return;
  }
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v5 = sub_400be(dat_9f9cc + -0x40000000,0,1);
  else {
    v5 = lseek(dat_9f9cc,0,1);
  }
  v5 -= dat_9efa8 - (int8)dat_9efa0;
  if (v5 <= -1)
    v5 = 0;
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v3 = sub_400be(dat_9f9cc + -0x40000000,v5,0);
  else {
    v3 = lseek(dat_9f9cc,v5,0);
  }
  if (v3 == v5)
    return;
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Cannot backspace archive file; it may be unreadable without -i"));
  if (dat_9efa0 == dat_9fd78)
    return;
  memset(dat_9efa0,0,(int8)dat_9fd78 - (int8)dat_9efa0);
  return;
}


// Function: sub_ce79 @ 0xce79
uint8 sub_ce79(int8 a0) // early-return x2
{
  int8 v1; // rax
  int8 v2; // rax
  uint8 v3; // rax
  uint8 v4;
  
  v1 = sub_bacd();
  v2 = (int8)dat_9f868 - (dat_9efb0 - dat_9efa0 >> 9);
  if (a0 <= v2 * 0x200)
    return 0;
  v3 = (uint8)(a0 + v2 * -0x200) / dat_9f870;
  if (v3) {
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v3 = sub_400be(dat_9f9cc + -0x40000000,dat_9f870 * v3,1);
    else {
      v3 = lseek(dat_9f9cc,dat_9f870 * v3,1);
    }
    if ((int8)v3 > -1) {
      if (v3 % dat_9f870) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("rmtlseek not stopped at a record boundary"));
        sub_177f4(); // no-return
      }
      v4 = v3;
      if ((int8)v3 <= -1)
        v4 = v3 + 0x1ff;
      v3 = ((int8)v4 >> 9) - v1;
      dat_9efc0 += (int8)v3 / (int8)dat_9f868;
      dat_9f038 = ((int8)v4 >> 9) - (int8)dat_9f868;
      dat_9efb0 = dat_9efa8;
    }
    return v3;
  }
  return 0;
}


// Function: sub_d046 @ 0xd046
void sub_d046(void)
{
  int4 v1;
  
  if ((dat_9fd70) || (dat_9efb8 == 1)) {
    do {
      sub_cac8();
    } while (dat_9efa0 < dat_9efb0);
  }
  sub_ae4d();
  if (dat_9f9b0)
    sub_11a1c();
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v1 = sub_3fe0c(dat_9f9cc + -0x40000000);
  else {
    v1 = close(dat_9f9cc);
  }
  if (v1)
    sub_3e1ff(*dat_9fbe8);
  sub_32ca8(dat_9f040,dat_9f048);
  sub_39d1c(0x9fa20);
  free(dat_9f010);
  free(dat_9f018);
  sub_ac2b(0);
  return;
}


// Function: sub_d132 @ 0xd132
void sub_d132(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int4 v1;
  
  sub_3eb4b(a0,a1,a2);
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v1 = sub_3fe0c(dat_9f9cc + -0x40000000);
  else {
    v1 = close(dat_9f9cc);
  }
  if (v1)
    sub_3e1ff(*dat_9fbe8);
  sub_32ca8(dat_9f040,0);
  sub_177f4(); // no-return
}


// Function: sub_d1c6 @ 0xd1c6
void sub_d1c6(void) // return-dupe x3
{
  void *v1; // rax
  unsigned long v2; // rax
  
  v1 = fopen(dat_9f9b8,"r");
  if (!v1) {
    if (*__errno_location() == 2)
      return;
    sub_3e38b(dat_9f9b8);
    return;
  }
  if ((__isoc99_fscanf(v1,"%d",0x9c014) == 1) && (0 <= dat_9c014)) {
    if (ferror_unlocked(v1))
      sub_3e403(dat_9f9b8);
    if (!fclose(v1))
      return;
    sub_3e1ff(dat_9f9b8);
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = sub_5e548(dat_9f9b8);
  error(0,0,gettext("%s: contains invalid volume number"),v2);
  sub_177f4(); // no-return
}


// Function: sub_d2e3 @ 0xd2e3
void sub_d2e3(void) // return-dupe x2
{
  void *v1; // rax
  
  v1 = fopen(dat_9f9b8,"w");
  if (!v1) {
    sub_3e38b(dat_9f9b8);
    return;
  }
  fprintf(v1,"%d\n",(uint8)dat_9c014);
  if (ferror_unlocked(v1))
    sub_3eb22(dat_9f9b8);
  if (!fclose(v1))
    return;
  sub_3e1ff(dat_9f9b8);
  return;
}


// Function: sub_d386 @ 0xd386
void sub_d386(void)
{
  dat_9c014 += 1;
  if (dat_9c014 <= -1) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Volume number overflow"));
    sub_177f4(); // no-return
  }
  dat_9c010 += 1;
  return;
}


// Function: sub_d3f9 @ 0xd3f9
void sub_d3f9(void *a0)
{
  uint4 v1;
  char *v10; // stack - 0x30
  char *v11; // stack - 0x28
  char v2;
  char *v3;
  bool v4;
  unsigned long *v5;
  unsigned long v6; // rax
  int8 v7; // rax
  unsigned long v8; // stack - 0x38
  char *v9; // stack - 0x40
  
  v9 = NULL;
  v8 = 0;
  v4 = 0;
label_d7df:
  if (v4) {
    free(v9); // return-dupe
    return;
  }
  fputc_unlocked(7,stderr);
  v6 = sub_5e84e(*dat_9fbe8);
  v1 = dat_9c014 + 1;
  fprintf(stderr,gettext("Prepare volume #%d for %s and hit return: "),(uint8)v1,v6);
  fflush_unlocked(stderr);
  v7 = getline(&v9,&v8,a0);
  v5 = dat_9fbe8;
  if (v7 <= 0) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("EOF where user reply was expected"));
    if (((dat_9f860 != 6) && (dat_9f860 != 7)) && (dat_9f860 != 5)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("WARNING: Archive is incomplete"));
    }
    sub_177f4(); // no-return
  }
  if (((*v9 == '\n') || (*v9 == 'y')) || (*v9 == 'Y')) {
    free(v9);
    return;
  }
  v2 = *v9;
  if (v2 == 'q') {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("No new volume; exiting.\n"));
    if (((dat_9f860 != 6) && (dat_9f860 != 7)) && (dat_9f860 != 5)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("WARNING: Archive is incomplete"));
    }
    sub_177f4(); // no-return
  }
  if (v2 <= 'q') {
    v3 = v9;
    if (v2 == 'n') {
      do {
        do {
          v10 = &v3[1];
          v3 = v10;
        } while (*v10 == ' ');
      } while (*v10 == '\t');
      for (v11 = v10; (*v11 && (*v11 != '\n')); v11 = &v11[1]) {
      }
      *v11 = '\0';
      if (*v10) { // branch-flip
        v6 = sub_60cc6(v10);
        *v5 = v6;
        v4 = 1;
      }
      else {
        fprintf(stderr,"%s",gettext("File name not specified. Try again.\n"));
      }
      goto label_d7df;
    }
    if ('o' <= v2) goto label_d7b6;
    if (v2 != '!') { // branch-flip
      if (v2 == '?') {
        fprintf(stderr,gettext(" n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n"));
        if (dat_9f9a9 != '\x01')
          fprintf(stderr,gettext(" !             Spawn a subshell\n"));
        fprintf(stderr,gettext(" ?             Print this list\n"));
        goto label_d7df;
      }
    }
    else if (dat_9f9a9 != '\x01') {
      sub_32df0();
      goto label_d7df;
    }
  }
label_d7b6:
  fprintf(stderr,gettext("Invalid input. Type ? for help.\n"));
  goto label_d7df;
}


// Function: sub_d818 @ 0xd818
unsigned long sub_d818(uint4 a0) // return-dupe
{
  char *v1;
  void *v2;
  int4 v3;
  unsigned long v4; // rax
  uint8 v5; // rax
  int4 v6; // stack - 0x1c
  
  if ((!dat_9f248) && (!dat_9f8c0)) {
    if (dat_9f9cc) // branch-flip
      dat_9f248 = stdin;
    else {
      dat_9f248 = fopen("/dev/tty","r");
    }
  }
  if (dat_9f320)
    return 0;
  if (dat_9f9b0)
    sub_11a1c();
  sub_29547(0x9efe0,0);
  sub_29547(0x9efe8,0);
  dat_9eff8 = 0;
  dat_9eff0 = 0;
  dat_9efb0 = dat_9efa0;
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v3 = sub_3fe0c(dat_9f9cc + -0x40000000);
  else {
    v3 = close(dat_9f9cc);
  }
  if (v3)
    sub_3e1ff(*dat_9fbe8);
  dat_9fbe8 = &dat_9fbe8[1];
  if (&dat_9fbd8[dat_9fbe0] == dat_9fbe8) {
    dat_9fbe8 = dat_9fbd8;
    dat_9f250 = 1;
  }
  v6 = dat_9f250;
  while( true ) {
    if (v6) {
      if (dat_9f8c0) { // branch-flip
        if (dat_9f9b8)
          sub_d2e3();
        v3 = sub_345e7(dat_9fbe8,dat_9c014 + 1);
        if (v3) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v4 = sub_5e84e(dat_9f8c0);
          error(0,0,gettext("%s command failed"),v4);
          sub_177f4(); // no-return
        }
      }
      else {
        sub_d3f9(dat_9f248);
      }
    }
    v1 = (char *)*dat_9fbe8;
    if (strcmp(v1,"-")) { // branch-flip
      if (dat_9f9b0) { // branch-flip
        if ((((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbe8,0x3a), dat_9fdf0)) && ((char *)*dat_9fbe8 < dat_9fdf0)) && (v5 = (int8)dat_9fdf0 - *dat_9fbe8, v2 = (void *)*dat_9fbe8, !memchr(v2,0x2f,v5)))
          dat_9f9cc = sub_3f860(*dat_9fbe8,0x42,0x40000000,dat_9f950);
        else {
          dat_9f9cc = open((char *)*dat_9fbe8,0x42,0x1b6);
        }
      }
      else if (a0 != 2) { // branch-flip
        if (a0 <= 2) {
          if (a0) { // branch-flip
            if (a0 == 1) {
              if (dat_9f884)
                sub_2a571(*dat_9fbe8,1);
              if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbe8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbe8 < dat_9fdf0 && (v5 = (int8)dat_9fdf0 - *dat_9fbe8, v2 = (void *)*dat_9fbe8, !memchr(v2,0x2f,v5)))))
                dat_9f9cc = sub_3f860(*dat_9fbe8,0x41,0x40000000,dat_9f950);
              else {
                dat_9f9cc = creat(*dat_9fbe8,0x1b6);
              }
            }
          }
          else {
            if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbe8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbe8 < dat_9fdf0 && (v5 = (int8)dat_9fdf0 - *dat_9fbe8, v2 = (void *)*dat_9fbe8, !memchr(v2,0x2f,v5)))))
              dat_9f9cc = sub_3f860(*dat_9fbe8,0,0x40000000,dat_9f950);
            else {
              dat_9f9cc = open((char *)*dat_9fbe8,0,0x1b6);
            }
            sub_b214();
          }
        }
      }
      else if ((((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbe8,0x3a), dat_9fdf0)) && ((char *)*dat_9fbe8 < dat_9fdf0)) && (v5 = (int8)dat_9fdf0 - *dat_9fbe8, v2 = (void *)*dat_9fbe8, !memchr(v2,0x2f,v5)))
        dat_9f9cc = sub_3f860(*dat_9fbe8,0x42,0x40000000,dat_9f950);
      else {
        dat_9f9cc = open((char *)*dat_9fbe8,0x42,0x1b6);
      }
    }
    else {
      dat_9f049 = 1;
      dat_9f9cc = 0;
    }
    if (0 <= dat_9f9cc) break;
    sub_3e3da(*dat_9fbe8);
    if (((dat_9f9b0 != '\x01') && (a0 == 1)) && (dat_9f884))
      sub_2a874();
    v6 = 1;
  }
  return 1;
}


// Function: sub_de84 @ 0xde84
unsigned long sub_de84(unsigned long a0) // return-dupe
{
  sub_39c55(a0);
  if (sub_25ecf(0x9f460,a0,0) == 1) {
    sub_bba7(dat_9f460);
    return 1;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("This does not look like a tar archive"));
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_df26 @ 0xdf26
unsigned long sub_df26(void)
{
  char v1;
  char v10 [32];
  char v11 [32];
  unsigned int v12; // stack - 0xa4
  int8 v13; // stack - 0x98
  uint8 v14; // stack - 0x90
  char *v2;
  int4 v3; // eax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  int8 v8; // stack - 0xa0
  char v9 [24];
  
  if (3 <= dat_9f860) { // branch-flip
    if (dat_9f860 == 8) goto label_df5e;
label_df6a:
    v12 = 0;
  }
  else {
    if (!dat_9f860) goto label_df6a;
label_df5e:
    v12 = 2;
  }
  if (sub_d818(v12) != '\x01')
    return 1;
  while( true ) {
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v13 = sub_3fe62(dat_9f9cc + -0x40000000,dat_9efa0,dat_9f870);
    else {
      v13 = sub_5ec70(dat_9f9cc,dat_9efa0,dat_9f870);
    }
    if (v13 != -1) break;
    sub_c729();
  }
  if (v13 != dat_9f870)
    sub_c881(v13);
  v8 = sub_bb4b();
  if (!v8) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("This does not look like a tar archive"));
    return 0;
  }
  v1 = *(char *)(v8 + 0x9c);
  if (v1 != 'g') { // branch-flip
    if (v1 <= 'g') {
      if (v1 != 'M') {
        if (v1 != 'V') goto label_e2de;
        if (sub_de84(0x9f080) != '\x01')
          return 0;
        sub_39d1c(0x9f080);
        sub_29591(0x9efe0,dat_9f460,100);
        sub_bba7(v8);
        v8 = sub_bb4b();
        if (*(char *)(v8 + 0x9c) != 'M') goto label_e2de;
      }
      if (sub_de84(0x9f080) != '\x01')
        return 0;
      sub_39d1c(0x9f080);
      sub_29591(0x9efe8,dat_9f460,100);
      dat_9eff0 = sub_27a73(dat_9f460 + 0x7c,0xc);
      dat_9eff8 = sub_27a73(dat_9f460 + 0x171,0xc);
    }
  }
  else {
    sub_39c55(0x9f080);
    if (sub_25ecf(&v8,0x9f080,2) != 2) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("This does not look like a tar archive"));
      return 0;
    }
    sub_1df5f(0x9f080);
    sub_39d1c(0x9f080);
    v3 = sub_25ecf(&v8,0x9f080,0);
    if (v3 != 1) { // branch-flip
      if (v3 != 5) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("This does not look like a tar archive"));
        return 0;
      }
    }
    else {
      sub_bba7(v8);
    }
  }
label_e2de:
  if (dat_9f060) {
    if (!dat_9efe8) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_5e84e(*(unsigned long *)(dat_9f060 + 0x10));
      error(0,0,gettext("%s is not continued on this volume"),v4);
      return 0;
    }
    v3 = strcmp(dat_9efe8,*(char **)(dat_9f060 + 0x10));
    if (v3) {
      if ((((dat_9f864 != 6) && (dat_9f864 != 2)) || (v2 = *(char **)(dat_9f060 + 0x10), strlen(v2) <= 99)) || (v3 = strncmp(dat_9efe8,*(char **)(dat_9f060 + 0x10),100), v3)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v4 = sub_5e84e(*(unsigned long *)(dat_9f060 + 0x10));
        error(0,0,gettext("%s is not continued on this volume"),v4);
        return 0;
      }
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_5e84e(*(unsigned long *)(dat_9f060 + 0x10));
      error(0,0,gettext("%s is possibly continued on this volume: header contains truncated name"),v4);
    }
    v14 = dat_9eff8 + dat_9eff0;
    if ((*(uint8 *)(dat_9f060 + 0x18) != v14) || (v14 < dat_9eff8)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_54e5a(dat_9eff8,v9);
      v5 = sub_54e5a(dat_9eff0,v10);
      v6 = sub_54e5a(*(unsigned long *)(dat_9f060 + 0x18),v11);
      v7 = sub_5e84e(dat_9efe8);
      error(0,0,gettext("%s is the wrong size (%s != %s + %s)"),v7,v6,v5,v4);
      return 0;
    }
    if (*(int8 *)(dat_9f060 + 0x18) - *(int8 *)(dat_9f060 + 0x20) != dat_9eff8) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_54e5a(dat_9eff8,v9);
      v5 = sub_54e5a(*(unsigned long *)(dat_9f060 + 0x20),v10);
      v6 = sub_54e5a(*(unsigned long *)(dat_9f060 + 0x18),v11);
      error(0,0,gettext("This volume is out of sequence (%s - %s != %s)"),v6,v5,v4);
      return 0;
    }
  }
  sub_d386();
  return 1;
}


// Function: sub_e680 @ 0xe680
void * sub_e680(uint1 *a0) // return-dupe
{
  uint8 v1; // rax
  void *v2; // rax
  uint1 *v3; // stack - 0x20
  
  v1 = strlen((char *)a0);
  if (!v1)
    return NULL;
  v3 = &a0[v1 - 1];
  while ((a0 < v3 && (*(uint2 *)((uint8)*v3 * 2 + *(int8 *)__ctype_b_loc()) & 0x800))) {
    v3 = &v3[-1];
  }
  if ((a0 < v3) && (a0 < &v3[-7])) {
    if (!memcmp(&v3[-7]," Volume ",8)) {
      v1 = (int8)&v3[-7] - (int8)a0;
      v2 = (void *)sub_60b5b(v1 + 1);
      memcpy(v2,a0,v1);
      *(char *)(v1 + (int8)v2) = 0;
      return v2;
    }
  }
  return NULL;
}


// Function: sub_e78a @ 0xe78a
bool sub_e78a(char *a0) // early-return x2
{
  int4 v1; // eax
  char *v2; // rax
  bool v3; // stack - 0x11
  
  v3 = 0;
  v1 = fnmatch(dat_9f9c0,a0,0);
  if (!v1)
    return 1;
  if (dat_9f8fc != '\x01')
    return 0;
  v2 = (char *)sub_e680(a0);
  if (v2) {
    v1 = fnmatch(v2,dat_9f9c0,0);
    v3 = v1 == 0;
    free(v2);
  }
  return v3;
}


// Function: sub_e825 @ 0xe825
void sub_e825(void)
{
  char v1; // al
  int8 v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char v5 [360];
  char v6 [80];
  
  if (!dat_9efe0) {
    v2 = sub_bb4b();
    if (!v2) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v3 = sub_5e84e(dat_9f9c0);
      error(0,0,gettext("Archive not labeled to match %s"),v3);
      sub_177f4(); // no-return
    }
    if (*(char *)(v2 + 0x9c) != 'V') { // branch-flip
      if (*(char *)(v2 + 0x9c) == 'g') {
        sub_39c55(v5);
        sub_1e28e(v6,v2,sub_2798b(v2 + 0x7c,0xc));
        sub_1df5f(v5);
        sub_39d1c(v5);
      }
    }
    else {
      sub_29591(0x9efe0,v2,100);
    }
  }
  if (dat_9efe0) {
    v1 = sub_e78a(dat_9efe0);
    if (v1 == '\x01')
      return;
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v3 = sub_5e821(1,dat_9f9c0);
    v4 = sub_5e821(0,dat_9efe0);
    error(0,0,gettext("Volume %s does not match %s"),v4,v3);
    sub_177f4(); // no-return
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v3 = sub_5e84e(dat_9f9c0);
  error(0,0,gettext("Archive not labeled to match %s"),v3);
  sub_177f4(); // no-return
}


// Function: sub_ea79 @ 0xea79
void sub_ea79(char *a0) // return-dupe
{
  char *v1; // rax
  
  if (dat_9f864 == 4) {
    sub_1e1d3("GNU.volume.label",0x9f080,a0);
    return;
  }
  v1 = (char *)sub_bb4b();
  memset(v1,0,0x200);
  strcpy(v1,a0);
  sub_29547(0x9fa28,v1);
  dat_9fa30 = sub_4e2e5(dat_9fa28);
  v1[0x9c] = 'V';
  sub_12af2(dat_9f9e0,&v1[0x88],0xc);
  sub_1422e(0x9fa20,v1,0xffffffffffffffff);
  sub_bba7(v1);
  return;
}


// Function: sub_eb6f @ 0xeb6f
void sub_eb6f(void)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3; // rax
  char v4 [24];
  
  v1 = (char *)sub_54e5a((int8)dat_9c010,v4);
  v2 = strlen(dat_9f9c0);
  v3 = (char *)sub_60b5b(strlen(v1) + v2 + 9);
  sprintf(v3,"%s %s %s",dat_9f9c0,"Volume",v1);
  sub_ea79(v3);
  free(v3);
  return;
}


// Function: sub_ec37 @ 0xec37
void sub_ec37(int8 a0) // return-dupe
{
  int8 v1; // rax
  void *v2; // stack - 0x1c8
  void *v3; // stack - 0x1c0
  unsigned int v4; // stack - 0x158
  uint4 v5; // stack - 0x154
  uint4 v6; // stack - 0x150
  unsigned long v7; // stack - 0x140
  unsigned long v8; // stack - 0xb0
  
  if (dat_9f864 != 4)
    return;
  memset(&v2,0,0x1b8);
  v2 = *(void **)(a0 + 0x10);
  v4 = 0x81a4;
  v3 = v2;
  v5 = getuid();
  v6 = getgid();
  v2 = (void *)sub_1cd02(&v2,"%d/GNUFileParts/%f.%n",(int8)dat_9c010);
  v3 = v2;
  v7 = *(unsigned long *)(a0 + 0x20);
  v8 = v7;
  v1 = sub_1381d(&v2);
  if (v1) {
    sub_1418c(sub_135e3(0,&v2,v1));
    free(v2);
    return;
  }
  abort(); // no-return
}


// Function: sub_ed89 @ 0xed89
void sub_ed89(void) // return-dupe
{
  if (dat_9f8fc) {
    sub_eb6f();
    return;
  }
  sub_ea79(dat_9f9c0);
  return;
}


// Function: sub_edb5 @ 0xedb5
void sub_edb5(int8 a0)
{
  unsigned int v1;
  void *v2; // rax
  unsigned long v3; // rax
  uint8 v4; // stack - 0x28
  
  v2 = (void *)sub_bb4b();
  v4 = strlen(*(char **)(a0 + 0x10));
  if (0x65 <= v4) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v3 = sub_5e548(*(unsigned long *)(a0 + 0x10));
    error(0,0,gettext("%s: file name too long to be stored in a GNU multivolume header, truncated"),v3);
    v4 = 100;
  }
  memset(v2,0,0x200);
  memcpy(v2,*(void **)(a0 + 0x10),v4);
  *(char *)((int8)v2 + 0x9c) = 0x4d;
  sub_12a9a(*(unsigned long *)(a0 + 0x20),(int8)v2 + 0x7c,0xc);
  sub_12a9a(*(int8 *)(a0 + 0x18) - *(int8 *)(a0 + 0x20),(int8)v2 + 0x171,0xc);
  v1 = dat_9f9ac;
  dat_9f9ac = 0;
  sub_1422e(0x9fa20,v2,0xffffffffffffffff);
  dat_9f9ac = v1;
  sub_bba7(v2);
  return;
}


// Function: sub_ef1c @ 0xef1c
void sub_ef1c(int8 a0) // return-dupe
{
  int8 v1; // stack - 0x18
  
  if (dat_9f864 == 4) {
    v1 = *(int8 *)(a0 + 0x18) - *(int8 *)(a0 + 0x20);
    sub_1e1d3("GNU.volume.filename",0x9f080,*(unsigned long *)(a0 + 0x10));
    sub_1e1d3("GNU.volume.size",0x9f080,a0 + 0x20);
    sub_1e1d3("GNU.volume.offset",0x9f080,&v1);
    return;
  }
  sub_edb5(a0);
  return;
}


// Function: sub_eff1 @ 0xeff1
void sub_eff1(void)
{
  int8 v1; // rax
  
  sub_106d8(0);
  dat_9f044 = 0;
  if ((dat_9efd8) && (dat_9f038)) {
    dat_9f9cc = 1;
    v1 = sub_3301a();
    dat_9f9cc = 0;
    if (v1 != dat_9f870)
      sub_c6d5(v1); // no-return
  }
  while( true ) {
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v1 = sub_3fe62(dat_9f9cc + -0x40000000,dat_9efa0,dat_9f870);
    else {
      v1 = sub_5ec70(dat_9f9cc,dat_9efa0,dat_9f870);
    }
    if (v1 == dat_9f870) break;
    if (v1 != -1) {
      sub_c881(v1);
      return;
    }
    sub_c729();
  }
  dat_9efc0 += 1;
  return;
}


// Function: sub_f0fb @ 0xf0fb
void sub_f0fb(void)
{
  int8 v1; // rax
  
  v1 = sub_c54f();
  if (v1 != dat_9f870)
    sub_c6d5(v1); // no-return
  dat_9efc8 += 1;
  dat_9f008 = dat_9f008 + (float8)v1;
  return;
}


// Function: sub_f160 @ 0xf160
void sub_f160(void) // return-dupe
{
  int8 v1; // rax
  
  sub_106d8(0);
  dat_9f044 = 0;
  if ((dat_9efd8) && (dat_9f038)) {
    dat_9f9cc = 1;
    v1 = sub_3301a();
    dat_9f9cc = 0;
    if (v1 != dat_9f870)
      sub_c6d5(v1); // no-return
  }
  while( true ) {
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v1 = sub_3fe62(dat_9f9cc + -0x40000000,dat_9efa0,dat_9f870);
    else {
      v1 = sub_5ec70(dat_9f9cc,dat_9efa0,dat_9f870);
    }
    if (v1 == dat_9f870) {
      dat_9efc0 += 1;
      return;
    }
    if (((!v1) || ((v1 == -1 && (*__errno_location() == 0x1c)))) && (dat_9f8fc)) break;
    if (v1 != -1) {
      sub_c881(v1);
      return;
    }
    sub_c729();
  }
  do {
  } while (sub_df26() != '\x01');
  if (dat_9efb0 != dat_9efa8)
    return;
  sub_cac8();
  return;
}


// Function: sub_f2b9 @ 0xf2b9
void sub_f2b9(void)
{
  dat_9f058 = sub_eff1;
  sub_f160();
  dat_9f058 = sub_f2b9;
  return;
}


// Function: sub_f2e5 @ 0xf2e5
void sub_f2e5(int8 a0) // return-dupe x2
{
  int8 v1; // rax
  void *v2; // stack - 0x38
  void *v3; // stack - 0x30
  uint8 v4; // stack - 0x28
  uint8 v5; // stack - 0x20
  
  v4 = sub_c54f();
  if ((v4 != dat_9f870) && (dat_9f8fc != '\x01'))
    sub_c6d5(v4); // no-return
  if (v4)
    dat_9efc8 += 1;
  dat_9f008 = dat_9f008 + (float8)(int8)v4;
  if (v4 != dat_9f870) {
    v1 = sub_abd9(v4);
    if (v4 & 0x1ff) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("write did not end on a block boundary"));
      dat_9fdd0 = 2;
      sub_c6d5(v4); // no-return
    }
    if (((((int8)v4 <= -1) && (*__errno_location() != 0x1c)) && (*__errno_location() != 5)) && (*__errno_location() != 6))
      sub_c6d5(v4); // no-return
    if (sub_d818(1) != '\x01')
      return;
    sub_39d1c(0x9f080);
    sub_d386();
    dat_9f000 = dat_9f008 + dat_9f000;
    dat_9f008 = 0.0;
    v3 = (void *)(v4 + dat_9efa0);
    v4 = a0 - v4;
    dat_9f030 = (uint4)(dat_9f030 == 0);
    sub_bc48();
    dat_9f070 = 1;
    if (dat_9f9c0)
      sub_eb6f();
    if (v1)
      sub_ef1c(v1);
    sub_135e3(1,0x9f080,sub_bb4b());
    sub_39d1c(0x9f080);
    if (v1)
      sub_ec37(v1);
    v2 = (void *)sub_bb4b();
    sub_acb0(v1,(int8)v2 - dat_9efa0 >> 9);
    dat_9f070 = 0;
    v5 = sub_bbf5(v2);
    while (v5 < v4) {
      memcpy(v2,v3,v5);
      v3 = (void *)((int8)v3 + v5);
      v4 -= v5;
      sub_bba7((int8)v2 + (v5 - 1 & 0xfffffffffffffe00));
      v2 = (void *)sub_bb4b();
      v5 = sub_bbf5(v2);
    }
    memcpy(v2,v3,v4);
    memset((void *)((int8)v2 + v4),0,v5 - v4);
    sub_bba7((int8)v2 + (v4 - 1 & 0xfffffffffffffe00));
    sub_bb4b();
    return;
  }
  return;
}


// Function: sub_f627 @ 0xf627
void sub_f627(unsigned long a0)
{
  dat_9f050 = sub_f0fb;
  sub_f2e5(a0);
  dat_9f050 = sub_f627;
  return;
}


// Function: sub_f662 @ 0xf662
void sub_f662(void)
{
  (*dat_9f058)();
  return;
}


// Function: sub_f676 @ 0xf676
void sub_f676(void)
{
  (*dat_9f050)(dat_9f870);
  return;
}


// Function: sub_f694 @ 0xf694
void sub_f694(uint4 a0)
{
  dat_9f058 = sub_f2b9;
  dat_9f050 = sub_f627;
  sub_bdb0(a0);
  if (a0 != 2) {
    if (3 <= a0) {
      sub_ae17(); // return-dupe
      return;
    }
    if (a0) {
      if ((a0 == 1) && (dat_9efc8 = 0, dat_9f9c0))
        sub_ed89();
      sub_ae17();
      return;
    }
  }
  if (dat_9f9c0)
    sub_e825();
  sub_ae17();
  return;
}


// Function: sub_f720 @ 0xf720
int8 sub_f720(unsigned int a0)
{
  int8 v1; // rax
  
  v1 = sub_60c06(0x18);
  if (dat_9f270) // branch-flip
    *dat_9f270 = v1;
  else {
    dat_9f268 = v1;
  }
  dat_9f270 = (int8 *)v1;
  *(unsigned int *)(v1 + 8) = a0;
  return v1;
}


// Function: sub_f77f @ 0xf77f
char * sub_f77f(unsigned long a0)
{
  char *v1; // rax
  uint8 v2; // rax
  
  v1 = (char *)sub_60cc6(a0);
  v2 = strlen(v1);
  if (((*v1 == '\"') || (*v1 == '\'')) && (v1[v2 - 1] == *v1)) {
    memmove(v1,&v1[1],v2 - 2);
    v1[v2 - 2] = '\0';
  }
  sub_2961d(v1);
  return v1;
}


// Function: sub_f825 @ 0xf825
void sub_f825(char *a0) // return-dupe x8
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  char *v3; // stack - 0x28
  int8 v4; // stack - 0x20
  uint8 v5; // stack - 0x18
  
  if (!dat_9f278) {
    sigemptyset((void *)0x9f280);
    dat_9f278 = 1;
  }
  if ((!strcmp(a0,".")) || (!strcmp(a0,"dot"))) {
    sub_f720(0);
    return;
  }
  if (!strcmp(a0,"bell")) {
    sub_f720(1);
    return;
  }
  if (!strcmp(a0,"echo")) {
    sub_f720(2);
    return;
  }
  if (!strncmp(a0,"echo=",5)) {
    v4 = sub_f720(2);
    v2 = sub_f77f(&a0[5]);
    *(unsigned long *)(v4 + 0x10) = v2;
    return;
  }
  if (!strncmp(a0,"exec=",5)) {
    v4 = sub_f720(5);
    v2 = sub_f77f(&a0[5]);
    *(unsigned long *)(v4 + 0x10) = v2;
    return;
  }
  if (!strncmp(a0,"ttyout=",7)) {
    v4 = sub_f720(3);
    v2 = sub_f77f(&a0[7]);
    *(unsigned long *)(v4 + 0x10) = v2;
    return;
  }
  if (!strncmp(a0,"sleep=",6)) {
    v5 = strtoul(&a0[6],&v3,10);
    if (!*v3) {
      *(uint8 *)(sub_f720(4) + 0x10) = v5;
      return;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("%s: not a valid timeout"),a0);
    sub_177f4(); // no-return
  }
  if (!strcmp(a0,"totals")) {
    sub_f720(6);
    return;
  }
  if (!strncmp(a0,"wait=",5)) {
    v4 = sub_f720(7);
    v1 = sub_35562(&a0[5]);
    *(unsigned int *)(v4 + 0x10) = v1;
    sigaddset((void *)0x9f280,*(int4 *)(v4 + 0x10));
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("%s: unknown checkpoint action"),a0);
  sub_177f4(); // no-return
}


// Function: sub_fb72 @ 0xfb72
void sub_fb72(void) // return-dupe
{
  if (((!dat_9f278) && (dat_9f890)) && (!dat_9f268))
    sub_f825(0x807b3);
  if (dat_9f278 == 1) {
    sigprocmask(0,(void *)0x9f280,NULL);
    if (dat_9f890) {
      dat_9f278 = 2;
      return;
    }
    dat_9f278 = 2;
    dat_9f890 = 10;
    return;
  }
  return;
}


// Function: sub_fbee @ 0xfbee
uint8 sub_fbee(void *a0) // early-return
{
  int4 v1; // eax
  char *v2; // rax
  uint8 v3; // rax
  char v4 [2];
  uint2 v5; // stack - 0x16
  
  v1 = fileno(a0);
  if ((ioctl(v1,0x5413,v4)) || (!v5)) {
    v2 = getenv("COLUMNS");
    if ((!v2) || (v3 = strtol(v2,NULL,10), (int8)v3 <= 0))
      return 0x50;
  }
  else {
    v3 = (uint8)v5;
  }
  return v3;
}


// Function: sub_fca5 @ 0xfca5
int8 sub_fca5(char *a0,int8 *a1,int8 *a2,uint8 *a3)
{
  uint8 v1;
  int8 v2;
  char *v3; // rax
  uint8 v4; // rax
  
  if ((*a0 == '{') && (v3 = strchr(&a0[1],0x7d), v3)) {
    v4 = (int8)v3 - (int8)a0;
    if (*a3 < v4) {
      *a3 = v4;
      v1 = *a3;
      v2 = *a2;
      *a2 = sub_60b8d(v2,v1);
    }
    memcpy((void *)*a2,&a0[1],v4 - 1);
    *(char *)((v4 - 1) + *a2) = 0;
    *a1 = (int8)&v3[1];
    return *a2;
  }
  *a1 = (int8)a0;
  return 0;
}


// Function: sub_fd96 @ 0xfd96
uint8 sub_fd96(void *a0,uint8 a1,char *a2,char a3,unsigned int a4) // ternary
{
  int4 v1; // eax
  char *v10; // stack - 0x1c0
  uint8 v11; // stack - 0x1b8
  int4 v12; // stack - 0x19c
  char *v13; // stack - 0x190
  unsigned long *v14; // stack - 0x188
  char *v15; // stack - 0x180
  char *v16; // stack - 0x178
  char *v17; // stack - 0x170
  void *v18; // stack - 0x168
  uint8 v19; // stack - 0x160
  uint8 v2; // rax
  int8 v20; // stack - 0x120
  unsigned long v21; // stack - 0xf0
  unsigned long v3; // rax
  int8 v4; // rax
  char v5 [24];
  char *v6; // stack - 0x198
  uint8 v7; // stack - 0x128
  unsigned long v8 [4];
  char v9 [16]; // stack - 0x158
  
  if (a3) // branch-flip
    v15 = gettext("write");
  else {
    v15 = gettext("read");
  }
  v16 = (char *)sub_54e5a(a4,v5);
  v13 = NULL;
  v10 = a2;
  if (!a2) {
    v10 = (a3) ? gettext("Write checkpoint %u") : gettext("Read checkpoint %u"); // branch-flip
  }
  v6 = v10;
  v11 = a1;
  do {
    if (!*v6) {
      fflush_unlocked(a0);
      return v11;
    }
    if (*v6 != '%') { // branch-flip
      fputc_unlocked((int4)*v6,a0);
      if (*v6 != '\r') // branch-flip
        v11 += 1;
      else {
        v11 = 0;
        dat_9f300 = 1;
      }
    }
    else {
      v6 = &v6[1];
      if (*v6 == '{') {
        v13 = (char *)sub_fca5(v6,&v6,0x9f310,0x9f318);
        if (!v13) {
          fputc_unlocked(0x25,a0);
          fputc_unlocked((int4)*v6,a0);
          v11 += 2;
          goto label_10380;
        }
      }
      v1 = (int4)*v6;
      if (v1 == 0x2a) {
        if (v13) // branch-flip
          v19 = strtol(v13,NULL,10);
        else {
          v19 = sub_fbee(a0);
        }
        for (; v11 < v19; v11 = v11 + 1) {
          fputc_unlocked(0x20,a0);
        }
        goto label_10328;
      }
      if (((0x2a <= v1) && (v1 <= 0x75)) && (0x54 <= v1)) {
        switch(v1) {
          case 0x54:
            v14 = (unsigned long *)0x9c140;
            sub_ae4d();
            if (v13) {
              v21 = 0x8088d;
              v1 = sub_461a6(v13,&v7,0x4644);
              if (v1) { // branch-flip
                if (dat_9fdc8)
                  (*dat_9fdc8)();
                v3 = sub_4647f(&v7);
                error(0,0,gettext("cannot split string \'%s\': %s"),v13,v3);
                dat_9fdd0 = 2;
              }
              else {
                for (v12 = 0; (uint8)(int8)v12 < v7; v12 = v12 + 1) {
                  v8[v12] = *(unsigned long *)((int8)v12 * 8 + v20);
                }
                for (; v12 <= 2; v12 = v12 + 1) {
                  v8[v12] = 0;
                }
                v14 = v8;
              }
            }
            v1 = sub_b768(a0,v14,0x2c,0);
            v11 += (int8)v1;
            if (v13)
              sub_46399(&v7);
            break;
          default:
            goto label_102eb;
          case 99:
            v1 = sub_fd96(a0,v11,dat_9c158,a3,a4);
            v11 += (int8)v1;
            break;
          case 100:
            sub_ae4d();
            v11 += (int8)fprintf(a0,"%.0f");
            break;
          case 0x73:
            fputs_unlocked(v15,a0);
            v2 = strlen(v15);
            v11 += v2;
            break;
          case 0x74:
            if (v13) // branch-flip
              v17 = v13;
            else {
              v17 = "%c";
            }
            gettimeofday(v9,NULL);
            v18 = localtime(v9);
            v4 = sub_4f679(a0,v17,v18,0,(int4)v9._8_8_ * 1000);
            v11 += v4;
            break;
          case 0x75:
            fputs_unlocked(v16,a0);
            v2 = strlen(v16);
            v11 += v2;
          
        }
      }
      else {
label_102eb:
        fputc_unlocked(0x25,a0);
        fputc_unlocked((int4)*v6,a0);
        v11 += 2;
      }
label_10328:
      v13 = NULL;
    }
label_10380:
    v6 = &v6[1];
  } while( true );
}


// Function: sub_103d4 @ 0x103d4
void sub_103d4(char a0)
{
  char v1 [4];
  int4 v2; // stack - 0x1c
  unsigned long *v3; // stack - 0x18
  
  for (v3 = dat_9f268; v3; v3 = (unsigned long *)*v3) {
    switch(*(unsigned int *)&v3[1]) {
      case 0:
        fputc_unlocked(0x2e,dat_9efd0);
        fflush_unlocked(dat_9efd0);
        break;
      case 1:
        if (!dat_9f308)
          dat_9f308 = fopen("/dev/tty","w");
        if (dat_9f308) {
          fputc_unlocked(7,dat_9f308);
          fflush_unlocked(dat_9f308);
        }
        break;
      case 2:
        v2 = fprintf(stderr,"%s: ",dat_9fe48);
        sub_fd96(stderr,(int8)v2,v3[2],a0,dat_9f260);
        fputc_unlocked(10,stderr);
        break;
      case 3:
        if (!dat_9f308)
          dat_9f308 = fopen("/dev/tty","w");
        if (dat_9f308)
          sub_fd96(dat_9f308,0,v3[2],a0,dat_9f260);
        break;
      case 4:
        sleep((uint4)v3[2]);
        break;
      case 5:
        sub_348d7(v3[2],*dat_9fbe8,dat_9f260);
        break;
      case 6:
        sub_ae4d();
        sub_baa2();
        break;
      case 7:
        sigwait(0x9f280,v1);
      
    }
  }
  return;
}


// Function: sub_10622 @ 0x10622
void sub_10622(void)
{
  int8 v1;
  unsigned long *v2; // stack - 0x18
  int8 v3; // stack - 0x10
  
  for (v2 = dat_9f268; v2; v2 = (unsigned long *)*v2) {
    if (((*(int4 *)&v2[1] == 3) && (dat_9f308)) && (dat_9f300)) {
      v3 = sub_fbee(dat_9f308);
      while( true ) {
        v1 = v3 + -1;
        if (!v3) break;
        fputc_unlocked(0x20,dat_9f308);
        v3 = v1;
      }
      fputc_unlocked(0xd,dat_9f308);
      fflush_unlocked(dat_9f308);
    }
  }
  return;
}


// Function: sub_106d8 @ 0x106d8
void sub_106d8(char a0)
{
  if (dat_9f890) {
    dat_9f260 += 1;
    if (!(dat_9f260 % dat_9f890))
      sub_103d4(a0);
  }
  return;
}


// Function: sub_10729 @ 0x10729
void sub_10729(void)
{
  if (dat_9f890) {
    sub_10622();
    if (dat_9f308)
      fclose(dat_9f308);
  }
  return;
}


// Function: sub_1075e @ 0x1075e
void sub_1075e(void) // return-dupe
{
  char v1 [8];
  
  dat_9f328 = sub_2b601(v1,dat_9f870);
  if (!dat_9f8e0)
    return;
  sub_24090();
  return;
}


// Function: sub_107be @ 0x107be
void sub_107be(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,int8 a8,int8 a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  unsigned long v1;
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  char v2; // al
  char v3 [16];
  unsigned int v4; // stack - 0xd8
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v2) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  if (a9) {
    v1 = *(unsigned long *)(a8 + 8);
    fprintf(dat_9efd0,"%s: ",sub_29516(1,v1));
    v4 = 0x10;
    v5 = 0x30;
    v6 = &Stack0000000000000008;
    v7 = v3;
    vfprintf(dat_9efd0,a9,&v4);
    fprintf(dat_9efd0,"\n");
  }
  sub_39e98(1);
  return;
}


// Function: sub_10903 @ 0x10903
unsigned long sub_10903(void)
{
  return 1;
}


// Function: sub_1091a @ 0x1091a
unsigned long sub_1091a(uint8 a0,void *a1) // return-dupe x2, return-dupe
{
  int4 v1; // eax
  uint8 v2; // rax
  
  v2 = sub_2a9e9(dat_9f324,dat_9f328,a0);
  if (v2 != a0) {
    if (v2 == 0xffffffffffffffff) {
      sub_3e403(dat_9fa28);
      sub_107be(0x9fa20,0);
      return 0;
    }
    sub_107be(0x9fa20,ngettext("Could only read %lu of %lu byte","Could only read %lu of %lu bytes",a0),v2,a0);
    return 0;
  }
  v1 = memcmp(a1,dat_9f328,a0);
  if (!v1)
    return 1;
  sub_107be(0x9fa20,gettext("Contents differ"));
  return 0;
}


// Function: sub_10a26 @ 0x10a26
void sub_10a26(int8 a0,code *a1)
{
  int8 v1; // rax
  code *v2; // stack - 0x38
  uint8 v3; // stack - 0x20
  uint8 v4; // stack - 0x18
  
  v4 = *(uint8 *)(a0 + 0x88);
  sub_ad2b(a0);
  v2 = a1;
  while( true ) {
    if (!v4) {
      sub_ad66();
      return;
    }
    v1 = sub_bb4b();
    if (!v1) break;
    v3 = sub_bbf5(v1);
    if (v4 < v3)
      v3 = v4;
    if (!(*v2)(v3,v1))
      v2 = sub_10903;
    sub_bba7(v1 + (v3 - 1));
    v4 -= v3;
    sub_ad86(v4);
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Unexpected EOF in archive"));
  dat_9fdd0 = 2;
  return;
}


// Function: sub_10b3a @ 0x10b3a
unsigned long sub_10b3a(unsigned long a0,unsigned long a1) // return-dupe
{
  if (sub_2a9b8(a0,a1)) {
    if (*__errno_location() != 2) // branch-flip
      sub_3ea03(a0);
    else {
      sub_3ea2c(a0);
    }
    sub_107be(0x9fa20,0);
    return 0;
  }
  return 1;
}


// Function: sub_10bb7 @ 0x10bb7
void sub_10bb7(void) // return-dupe x3
{
  int4 v1; // eax
  char v2 [24];
  uint4 v3; // stack - 0x90
  
  v1 = sub_10b3a(dat_9fa28,v2);
  if (!v1)
    return;
  if ((v3 & 0xf000) != 0x4000) {
    sub_107be(0x9fa20,gettext("File type differs"));
    return;
  }
  if (!((v3 ^ dat_9fa90) & 0xfff))
    return;
  sub_107be(0x9fa20,gettext("Mode differs"));
  return;
}


// Function: sub_10c82 @ 0x10c82
void sub_10c82(void) // return-dupe x5
{
  char *v1;
  int4 v2; // eax
  char v3 [24];
  char v4 [16];
  uint4 v5; // stack - 0xa0
  int8 v6; // stack - 0x88
  
  v1 = dat_9fa28;
  v2 = sub_10b3a(dat_9fa28,v3);
  if (!v2) {
    sub_28891();
    return;
  }
  if ((v5 & 0xf000) != 0x8000) {
    sub_107be(0x9fa20,gettext("File type differs"));
    sub_28891();
    return;
  }
  if ((v5 ^ dat_9fa90) & 0xfff)
    sub_107be(0x9fa20,gettext("Mode differs"));
  if (sub_32ebf(v3,0x9fa78) != '\x01')
    sub_107be(0x9fa20,gettext("Uid differs"));
  if (sub_32ee4(v3,0x9fa78) != '\x01')
    sub_107be(0x9fa20,gettext("Gid differs"));
  v4 = sub_5fd9e(v3);
  v2 = sub_39e35(SUB168(v4,0),SUB168(v4,8),dat_9fb18,dat_9fb20);
  if (v2)
    sub_107be(0x9fa20,gettext("Mod time differs"));
  if ((*(char *)(dat_9f460 + 0x9c) != 'S') && (v6 != dat_9faa8)) {
    sub_107be(0x9fa20,gettext("Size differs"));
    sub_28891();
    return;
  }
  dat_9f324 = openat(dat_9c268,v1,dat_9fc08);
  if (dat_9f324 <= -1) {
    sub_3e38b(v1);
    sub_28891();
    sub_107be(0x9fa20,0);
    return;
  }
  if (dat_9fb40) // branch-flip
    sub_30972(dat_9f324,0x9fa20);
  else {
    sub_10a26(0x9fa20,sub_1091a);
  }
  if ((dat_9f880 == 1) && (v6)) {
    v4 = sub_5fd6a(v3);
    v2 = sub_2ab5e(dat_9f324,dat_9c268,v1,SUB168(v4,0),SUB168(v4,8));
    if (v2)
      sub_3ead0(v1);
  }
  v2 = close(dat_9f324);
  if (!v2)
    return;
  sub_3e1ff(v1);
  return;
}


// Function: sub_10fea @ 0x10fea
void sub_10fea(void)
{
  int4 v1; // eax
  unsigned long v2; // rax
  char v3 [144];
  char v4 [152];
  
  v1 = sub_10b3a(dat_9fa28,v3);
  if (((v1) && (v1 = sub_10b3a(dat_9fa38,v4), v1)) && (sub_32f09(v3,v4) != '\x01')) {
    v2 = sub_29516(0,dat_9fa38);
    sub_107be(0x9fa20,gettext("Not linked to %s"),v2);
  }
  return;
}


// Function: sub_110be @ 0x110be
void sub_110be(void) // return-dupe
{
  int4 v1; // eax
  uint8 v2; // rax
  char *v3;
  uint8 v4; // rax
  char v5 [1032];
  
  v2 = strlen(dat_9fa38);
  if (0x400 <= v2) // branch-flip
    v3 = (char *)sub_60b5b(v2 + 1);
  else {
    v3 = v5;
  }
  v4 = readlinkat(dat_9c268,dat_9fa28,v3,v2 + 1);
  if (0 <= (int8)v4) { // branch-flip
    if ((v4 != v2) || (v1 = memcmp(dat_9fa38,v3,v2), v1))
      sub_107be(0x9fa20,gettext("Symlink differs"));
  }
  else {
    if (*__errno_location() != 2) // branch-flip
      sub_3e6b3(dat_9fa28);
    else {
      sub_3e6dc(dat_9fa28);
    }
    sub_107be(0x9fa20,0);
  }
  if (v3 == v5)
    return;
  free(v3);
  return;
}


// Function: sub_11230 @ 0x11230
void sub_11230(void) // return-dupe x4
{
  int4 v1; // eax
  char v2 [24];
  bool v3;
  uint4 v4; // stack - 0x90
  int8 v5; // stack - 0x80
  
  v1 = sub_10b3a(dat_9fa28,v2);
  if (!v1)
    return;
  if (*(char *)(dat_9f460 + 0x9c) != '3') { // branch-flip
    if (*(char *)(dat_9f460 + 0x9c) != '4') // branch-flip
      v3 = (v4 & 0xf000) != 0x1000;
    else {
      v3 = (v4 & 0xf000) != 0x6000;
    }
  }
  else {
    v3 = (v4 & 0xf000) != 0x2000;
  }
  if (v3) {
    sub_107be(0x9fa20,gettext("File type differs"));
    return;
  }
  if (((*(char *)(dat_9f460 + 0x9c) == '3') || (*(char *)(dat_9f460 + 0x9c) == '4')) && (dat_9faa0 != v5)) {
    sub_107be(0x9fa20,gettext("Device number differs"));
    return;
  }
  if (!((v4 ^ dat_9fa90) & 0xfff))
    return;
  sub_107be(0x9fa20,gettext("Mode differs"));
  return;
}


// Function: sub_113b1 @ 0x113b1
int4 sub_113b1(char *a0,char *a1) // early-return x3
{
  char v1;
  int8 v2; // rax
  uint8 v3; // rdx
  char *v4; // stack - 0x28
  char *v5; // stack - 0x20
  
  v4 = a1;
  v5 = a0;
  do {
    do {
      if (!*v5) {
        v1 = *v4;
        return (int4)v1;
      }
    } while (0x15 < (uint4)((int4)*v5 - 0x44U));
    v3 = 1L << ((uint1)((int4)*v5 - 0x44U) & 0x3f);
    if (v3 & 0x114000) {
      v1 = *v4;
      return (int4)v1;
    }
    if (v3 & 0x200400) { // branch-flip
      v1 = *v4;
      if (!strchr("YN",(int4)v1))
        return 1;
      if (strcmp(&v5[1],&v4[1]))
        return 1; // return-dupe
      v2 = strlen(v5) + 1;
      v5 = &v5[v2];
      v4 = &v4[v2];
    }
    else if (v3 & 1) {
      if (strcmp(v5,v4))
        return 1;
      v2 = strlen(v5) + 1;
      v5 = &v5[v2];
      v4 = &v4[v2];
    }
  } while( true );
}


// Function: sub_11512 @ 0x11512
void sub_11512(unsigned long *a0) // return-dupe x2
{
  unsigned long v1;
  int4 v2; // eax
  int8 v3; // rax
  code *v4; // stack - 0x18
  
  if (!*(int4 *)&a0[0x35]) {
    v4 = NULL;
    v2 = sub_157f9(a0[0x33],*a0,dat_9fc08);
    if (0 <= v2) { // branch-flip
      if (fstat(v2,&a0[0xb])) { // branch-flip
        v4 = sub_2b444;
        close(v2);
      }
      else {
        *(int4 *)&a0[0x35] = v2;
      }
    }
    else {
      v4 = sub_2b2b1;
    }
    if (v4) {
      sub_2b48b(*a0,0,v4);
      return;
    }
  }
  v3 = sub_2241b(sub_21eb9(a0));
  if (!v3) {
    sub_10a26(a0,sub_10903);
    return;
  }
  v1 = a0[0x32];
  if (!sub_113b1(v1,v3))
    return;
  sub_107be(a0,gettext("Contents differ"));
  return;
}


// Function: sub_11650 @ 0x11650
void sub_11650(void) // return-dupe x6
{
  int4 v1; // eax
  int8 v2; // rax
  char v3 [24];
  uint4 v4; // stack - 0x90
  int8 v5; // stack - 0x78
  
  if (dat_9fa30) {
    sub_10bb7();
    return;
  }
  v1 = sub_10b3a(dat_9fa28,v3);
  if (!v1)
    return;
  if ((v4 & 0xf000) != 0x8000) {
    sub_107be(0x9fa20,gettext("File type differs"));
    sub_28891();
    return;
  }
  v2 = sub_2798b(dat_9f460 + 0x171,0xc);
  if (((0 <= v2) && (!SCARRY8(v2,dat_9faa8))) && (v5 == v2 + dat_9faa8)) {
    v1 = openat(dat_9c268,dat_9fa28,dat_9fc08);
    if (v1 <= -1) {
      sub_3e38b(dat_9fa28);
      sub_107be(0x9fa20,0);
      sub_28891();
      return;
    }
    if (0 <= (int8)lseek(v1,v2,0)) // branch-flip
      sub_10a26(0x9fa20,sub_1091a);
    else {
      sub_3e7a9(dat_9fa28,v2);
      sub_107be(0x9fa20,0);
    }
    if (!close(v1))
      return;
    sub_3e1ff(dat_9fa28);
    return;
  }
  sub_107be(0x9fa20,gettext("Size differs"));
  sub_28891();
  return;
}


// Function: sub_11897 @ 0x11897
void sub_11897(void) // return-dupe x7
{
  char v1;
  unsigned long v2; // rax
  
  sub_bba7(dat_9f460);
  if (dat_9f9ac) {
    if (dat_9f320)
      fprintf(dat_9efd0,gettext("Verify "));
    sub_2864c(0x9fa20,dat_9f460,0xffffffffffffffff);
  }
  switch(*(char *)(dat_9f460 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x37:
    case 0x53:
      goto label_119b7;
    case 0x31:
      sub_10fea();
      return;
    case 0x32:
      sub_110be();
      return;
    case 0x33:
    case 0x34:
    case 0x36:
      sub_11230();
      return;
    case 0x35:
    case 0x44:
      if (sub_2490b(0x9fa20))
        sub_11512(0x9fa20);
      sub_10bb7();
      return;
    case 0x4d:
      sub_11650();
      return;
    case 0x56:
      return;
    
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v1 = *(char *)(dat_9f460 + 0x9c);
  v2 = sub_5e548(dat_9fa28);
  error(0,0,gettext("%s: Unknown file type \'%c\', diffed as normal file"),v2,(uint8)(uint4)(int4)v1);
  dat_9fdd0 = 2;
label_119b7:
  if (!dat_9fa30) {
    sub_10c82();
    return;
  }
  sub_10bb7();
  return;
}


// Function: sub_11a1c @ 0x11a1c
void sub_11a1c(void)
{
  bool v1;
  int4 v2; // eax
  int8 v3; // rax
  unsigned long v4; // rax
  unsigned short v5 [2]; // stack - 0x40
  char v6 [24];
  int4 v7; // stack - 0x4c
  uint4 v8; // stack - 0x48
  unsigned int v9; // stack - 0x3c
  
  v1 = 0;
  if (sub_3ed80()) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Archive contains file names with leading prefixes removed."));
    v1 = 1;
  }
  if (sub_3ba20()) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Archive contains transformed file names."));
    v1 = 1;
  }
  if (v1) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Verification may fail to locate original files."));
  }
  sub_20f88();
  if (!dat_9f328)
    sub_1075e();
  fsync(dat_9f9cc);
  ioctl(dat_9f9cc,0x24b);
  v5[0] = 2;
  v9 = 1;
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v2 = sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v5);
  else {
    v2 = ioctl(dat_9f9cc,0x40086d01,v5);
  }
  if (v2 < 0) {
    if (*__errno_location() == 5) {
      if (0x40000000 <= dat_9f9cc) // branch-flip
        v2 = sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v5);
      else {
        v2 = ioctl(dat_9f9cc,0x40086d01,v5);
      }
      if (0 <= v2) goto label_11c88;
    }
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v3 = sub_400be(dat_9f9cc + -0x40000000,0,0);
    else {
      v3 = lseek(dat_9f9cc,0,0);
    }
    if (v3) {
      sub_3e867(*dat_9fbd8);
      return;
    }
  }
label_11c88:
  dat_9efb8 = 0;
  dat_9f320 = 1;
  sub_f662();
  while( true ) {
    v7 = sub_25ecf(0x9f460,0x9fa20,0);
    if (v7 == 5) {
      v8 = 0;
      do {
        v8 += 1;
        sub_bba7(dat_9f460);
        v7 = sub_25ecf(0x9f460,0x9fa20,0);
      } while (v7 == 5);
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,(char *)ngettext("VERIFY FAILURE: %d invalid header detected","VERIFY FAILURE: %d invalid headers detected",(int8)(int4)v8),(uint8)v8);
      dat_9fdd0 = 2;
    }
    if (v7 == 4) break;
    if (v7 != 3) { // branch-flip
      sub_26747(dat_9f460,0x9fa20,0x9f468,1);
      sub_11897();
      sub_39d1c(0x9fa20);
    }
    else {
      sub_bba7(dat_9f460);
      if (dat_9f8bd != '\x01') {
        if (sub_25ecf(0x9f460,0x9fa20,0) == 3) {
          dat_9efb8 = 1;
          dat_9f320 = 0;
          return;
        }
        if (dat_9ec84 & 1) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v4 = sub_54e5a(sub_bacd(),v6);
          error(0,0,gettext("A lone zero block at %s"),v4);
        }
      }
    }
  }
  dat_9f320 = 0;
  dat_9efb8 = 1;
  return;
}


// Function: sub_11ea1 @ 0x11ea1
void sub_11ea1(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_60b5b(0x28);
  v1[4] = dat_9f330;
  *v1 = a0;
  *(unsigned int *)&v1[2] = a1;
  v1[3] = a2;
  v1[1] = strlen(a0);
  dat_9f330 = v1;
  return;
}


// Function: sub_11f18 @ 0x11f18
void sub_11f18(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  if ((dat_9f9ac) && (dat_9ec84 & 4)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v1 = sub_5e285(1,a1);
    v2 = sub_5e548(a0);
    error(0,0,gettext("%s: contains a cache directory tag %s; %s"),v2,v1,a2);
  }
  return;
}


// Function: sub_11fbf @ 0x11fbf
unsigned int sub_11fbf(unsigned long a0,unsigned long *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_11fbf
{
  code *v1;
  bool v2; // al
  int4 v3; // eax
  unsigned long *v4; // stack - 0x10
  
  v4 = dat_9f330;
  do {
    if (!v4)
      return 0;
    v3 = sub_157f9(a0,*v4,dat_9fc08);
    if (0 <= v3) {
      v2 = !((v4[3]) && (v1 = (code *)v4[3], !(*v1)(v3)));
      close(v3);
      if (v2) {
        if (a1)
          *a1 = *v4;
        return *(unsigned int *)&v4[2];
      }
    }
    v4 = (unsigned long *)v4[4];
  } while( true );
}


// Function: sub_12087 @ 0x12087
unsigned int sub_12087(int4 a0) // return-dupe
{
  char v1 [56];
  
  if ((read(a0,v1,0x2b) == 0x2b) && (!memcmp(v1,"Signature: 8a477f597d28d172789f06886806bc55",0x2b)))
    return 1;
  return 0;
}


// Function: sub_12105 @ 0x12105
void sub_12105(uint8 a0,int8 a1,int8 a2)
{
  uint8 v1; // stack - 0x18
  int8 v2; // stack - 0x10
  
  v1 = a0;
  v2 = a2;
  do {
    v2 -= 1;
    *(uint1 *)(v2 + a1) = ((uint1)v1 & 7) + 0x30;
    v1 >>= 3;
  } while (v2);
  return;
}


// Function: sub_12157 @ 0x12157
void sub_12157(int8 a0,int8 a1,uint8 a2)
{
  uint8 v1; // stack - 0x10
  
  for (v1 = 0; (v1 < a2 && (*(char *)(v1 + a0) = *(char *)(a1 + v1), *(char *)(v1 + a0))); v1 = v1 + 1) {
  }
  return;
}


// Function: sub_121ac @ 0x121ac
void sub_121ac(int8 a0,unsigned long a1,int8 a2) // return-dupe
{
  sub_12157(a0,a1,a2);
  if (dat_9f864 != 2)
    return;
  *(char *)(a0 + a2 + -1) = 0;
  return;
}


// Function: sub_121fb @ 0x121fb
void sub_121fb(int4 a0,uint8 a1,int8 a2,int8 a3)
{
  uint8 v1; // stack - 0x20
  int8 v2; // stack - 0x18
  
  v1 = a1;
  v2 = a3;
  do {
    v2 -= 1;
    *(char *)(v2 + a2) = (char)v1;
    v1 = v1 >> 8 | (int8)-a0 << 0x38;
  } while (v2);
  return;
}


// Function: sub_12262 @ 0x12262
unsigned long sub_12262(int4 a0,int4 a1,int8 a2,unsigned long a3,code *a4,unsigned long a5,int8 a6,unsigned long a7) // early-return
{
  uint8 v1;
  char *v10; // stack - 0xd8
  char *v11; // stack - 0xd0
  char *v12; // stack - 0xc8
  uint8 v13; // stack - 0xc0
  unsigned long v14; // stack - 0xb8
  int8 v15; // stack - 0xb0
  char *v16; // stack - 0xa8
  char *v17; // stack - 0xa0
  uint8 v18; // stack - 0x98
  uint8 v19; // stack - 0x90
  int8 v2; // rax
  unsigned long v3;
  char v4 [32];
  char v5 [32];
  char v6 [32];
  int8 v7;
  uint4 v8; // stack - 0xdc
  char v9 [23];
  
  v3 = a7;
  if (a1) { // branch-flip
    if (0x40 <= (uint8)((a6 + -1) * 8)) // branch-flip
      v13 = 0xffffffffffffffff;
    else {
      v13 = (1L << (((char)a6 + '\xff') * '\b' & 0x3fU)) - 1;
    }
  }
  else if (0x40 <= (uint8)(a6 * 3 - 3U)) // branch-flip
    v13 = 0xffffffffffffffff;
  else {
    v13 = (1L << ((char)a6 * '\x03' - 3U & 0x3fU)) - 1;
  }
  v14 = sub_54e5a(v13,v4);
  if (a1) { // branch-flip
    if (v13 != 0xffffffffffffffff) // branch-flip
      v1 = v13;
    else {
      v1 = 0x7fffffffffffffff;
    }
    v15 = v1 + 1;
    v2 = sub_54e5a(v15,v5);
    v10 = (char *)(v2 + -1);
    *v10 = 0x2d;
    v16 = v10;
  }
  else {
    v10 = (char *)0x80d84;
  }
  if (a0) { // branch-flip
    v11 = (char *)(sub_54e5a(-a2,v6) + -1);
    *v11 = 0x2d;
    v17 = v11;
  }
  else {
    v11 = (char *)sub_54e5a(a2,&v7);
  }
  if (a4) {
    v18 = (*a4)(&v8) & v13;
    v8 &= dat_9f864 == 6;
    if (v8) // branch-flip
      v19 = -v18;
    else {
      v19 = v18;
    }
    v12 = (char *)sub_54e5a(v19,v9);
    if (v8) {
      v12 = &v12[-1];
      *v12 = 0x2d;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("value %s out of %s range %s..%s; substituting %s"),v11,v3,v10,v14,v12);
    v3 = sub_1262b(v8,v19,a3,0,a5,a6,v3);
    return v3;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("value %s out of %s range %s..%s"),v11,v3,v10,v14);
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_1262b @ 0x1262b
unsigned long sub_1262b(int4 a0,uint8 a1,int8 a2,unsigned long a3,char *a4,int8 a5,unsigned long a6) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1262b
{
  int4 v1; // eax
  uint8 v2;
  unsigned long v3; // rax
  char v4; // dl
  uint8 v5; // rdx
  unsigned long v6; // stack - 0x38
  
  v1 = !((dat_9f864 != 6) && (dat_9f864 != 2));
  if (!a0) {
    if (0x40 <= (uint8)(a5 * 3 - 3U)) // branch-flip
      v2 = 0xffffffffffffffff;
    else {
      v2 = (1L << ((char)a5 * '\x03' - 3U & 0x3fU)) - 1;
    }
    if (a1 <= v2) {
      a4[a5 + -1] = 0;
      sub_12105(a1,a4,a5 + -1);
      return 1;
    }
  }
  if (v1) { // branch-flip
    if (a0) // branch-flip
      v2 = ~a1;
    else {
      v2 = a1;
    }
    if (0x40 <= (uint8)((a5 + -1) * 8)) // branch-flip
      v5 = 0xffffffffffffffff;
    else {
      v5 = (1L << (((char)a5 + '\xff') * '\b' & 0x3fU)) - 1;
    }
    if (v2 <= v5) {
      v4 = (a0) ? 0xff : 0x80; // branch-flip
      *a4 = v4;
      sub_121fb(a0,a1,&a4[1],a5 + -1);
      return 1;
    }
    v6 = a3;
    if ((a0) && (v2 = a5 * 3 - 3, (uint8)(a2 * 8) < v2 || !(a2 * 8 - v2))) {
      if (!dat_9f348) {
        dat_9f348 = 1;
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Generating negative octal headers"));
      }
      a4[a5 + -1] = 0;
      if (0x40 <= (uint8)(a2 << 3)) // branch-flip
        v2 = 0xffffffffffffffff;
      else {
        v2 = (1L << ((uint1)((int4)a2 << 3) & 0x3f)) - 1;
      }
      sub_12105(v2 & a1,a4,a5 + -1);
      return 1;
    }
  }
  else {
    v6 = 0;
  }
  v3 = sub_12262(a0,v1,a1,a2,v6,a4,a5,a6);
  return v3;
}


// Function: sub_128a8 @ 0x128a8
int4 sub_128a8(unsigned int *a0)
{
  if (!dat_9f34c) {
    if (!sub_2c261("nobody",0x9f34c))
      dat_9f34c = -2;
  }
  *a0 = 0;
  return dat_9f34c;
}


// Function: sub_12901 @ 0x12901
void sub_12901(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(0,a0,4,sub_128a8,a1,a2,"gid_t");
  return;
}


// Function: sub_12957 @ 0x12957
void sub_12957(uint4 a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(a0 >> 0x1f,(int8)(int4)a0,4,0,a1,a2,"major_t");
  return;
}


// Function: sub_129ae @ 0x129ae
void sub_129ae(uint4 a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(a0 >> 0x1f,(int8)(int4)a0,4,0,a1,a2,"minor_t");
  return;
}


// Function: sub_12a05 @ 0x12a05
void sub_12a05(uint4 a0,unsigned long a1,unsigned long a2)
{
  if (((dat_9f864 == 4) || (dat_9f864 == 3)) || (dat_9f864 == 6))
    a0 &= 0xfff;
  sub_1262b(0,a0,4,0,a1,a2,"mode_t");
  return;
}


// Function: sub_12a9a @ 0x12a9a
void sub_12a9a(uint8 a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(a0 >> 0x3f,a0,8,0,a1,a2,"off_t");
  return;
}


// Function: sub_12af2 @ 0x12af2
void sub_12af2(uint8 a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(a0 >> 0x3f,a0,8,0,a1,a2,"time_t");
  return;
}


// Function: sub_12b4a @ 0x12b4a
int4 sub_12b4a(unsigned int *a0)
{
  if (!dat_9f350) {
    if (!sub_2c173("nobody",0x9f350))
      dat_9f350 = -2;
  }
  *a0 = 0;
  return dat_9f350;
}


// Function: sub_12ba3 @ 0x12ba3
void sub_12ba3(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(0,a0,4,sub_12b4a,a1,a2,"uid_t");
  return;
}


// Function: sub_12bf9 @ 0x12bf9
void sub_12bf9(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1262b(0,a0,8,0,a1,a2,"uintmax_t");
  return;
}


// Function: sub_12c4c @ 0x12c4c
void sub_12c4c(unsigned long a0,int8 a1,int8 a2)
{
  sub_12157(a1,a0,a2);
  *(char *)(a1 + a2 + -1) = 0;
  return;
}


// Function: sub_12c90 @ 0x12c90
uint4 sub_12c90(int8 a0) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_12c90
{
  int8 v1;
  uint4 v2; // eax
  uint8 v3;
  
  v2 = *(uint4 *)(a0 + 0x18) & 0xf000;
  if (v2 == 0x4000) {
    v2 = 1;
    return v2;
  }
  v2 = *(uint4 *)(a0 + 0x18) & 0xf000;
  if (v2 == 0x8000) {
    if (!dat_9f9d0) {
      if ((!*(int8 *)(a0 + 0x30)) && (v2 = *(uint4 *)(a0 + 0x18) & 0x124, v2 == 0x124)) {
        v2 = 0;
        return v2;
      }
      v2 = 1;
      return v2;
    }
    if ((dat_9f999) && (dat_9f979)) {
      v3 = *(int8 *)(a0 + 0x30);
      if ((int8)v3 <= -1)
        v3 += 0x1ff;
      v3 = (int8)v3 >> 9;
      v1 = v3;
      v2 = ((*(uint8 *)(a0 + 0x30) & 0x1ff) && (v3 = *(int8 *)(a0 + 0x30) + 0x1ff, 0x3ff <= v3));
      v3 = (int8)(int4)v2;
      v3 += v1;
      if (*(int8 *)(a0 + 0x40) < (int8)v3) {
        v2 = 1;
        return v2;
      }
    }
    v2 = 0;
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_12da0 @ 0x12da0
void sub_12da0(void)
{
  void *v1; // rax
  
  v1 = (void *)sub_bb4b();
  memset(v1,0,0x200);
  sub_bba7(v1);
  v1 = (void *)sub_bb4b();
  memset(v1,0,sub_bbf5(v1));
  sub_bba7(v1);
  return;
}


// Function: sub_12e0f @ 0x12e0f
void * sub_12e0f(unsigned long a0,unsigned long a1,uint8 a2)
{
  void *v1; // rax
  
  v1 = (void *)sub_bb4b();
  memset(v1,0,0x200);
  sub_121ac(v1,a0,100);
  sub_12a9a(a1,(int8)v1 + 0x7c,0xc);
  if (0 <= (int8)a2) { // branch-flip
    if (0x200000000 <= a2)
      a2 = 0x1ffffffff;
  }
  else {
    a2 = 0;
  }
  sub_12af2(a2,(int8)v1 + 0x88,0xc);
  sub_12a05(0x81a4,(int8)v1 + 100,8);
  sub_12ba3(0,(int8)v1 + 0x6c,8);
  sub_12901(0,(int8)v1 + 0x74,8);
  memcpy((void *)((int8)v1 + 0x101),"ustar",6);
  memcpy((void *)((int8)v1 + 0x107),(void *)0x80e49,2);
  return v1;
}


// Function: sub_12f4e @ 0x12f4e
void * sub_12f4e(int8 a0)
{
  void *v1; // rax
  
  v1 = (void *)sub_bb4b();
  memset(v1,0,0x200);
  sub_121ac(v1,*(unsigned long *)(a0 + 8),100);
  return v1;
}


// Function: sub_12f9f @ 0x12f9f
void sub_12f9f(unsigned long a0,char *a1,char a2)
{
  int8 v1; // rax
  char *v2; // stack - 0x38
  uint8 v3; // stack - 0x20
  uint8 v4; // stack - 0x18
  void *v5; // stack - 0x10
  
  v3 = strlen(a1) + 1;
  v1 = sub_12e0f("././@LongLink",v3,0);
  if (dat_9f934 != '\x01') {
    if (!dat_9f358) {
      sub_2bfd1(0,0x9f358);
      sub_2c0a2(0,0x9f360);
    }
    sub_12c4c(dat_9f358,v1 + 0x109,0x20);
    sub_12c4c(dat_9f360,v1 + 0x129,0x20);
  }
  strcpy((char *)(v1 + 0x101),"ustar  ");
  *(char *)(v1 + 0x9c) = a2;
  sub_1422e(a0,v1,0xffffffffffffffff);
  v5 = (void *)sub_bb4b();
  v4 = sub_bbf5(v5);
  v2 = a1;
  while (v4 < v3) {
    memcpy(v5,v2,v4);
    v2 = &v2[v4];
    v3 -= v4;
    sub_bba7((int8)v5 + (v4 - 1 & 0xfffffffffffffe00));
    v5 = (void *)sub_bb4b();
    v4 = sub_bbf5(v5);
  }
  memcpy(v5,v2,v3);
  memset((void *)((int8)v5 + v3),0,v4 - v3);
  sub_bba7((int8)v5 + (v3 - 1 & 0xfffffffffffffe00));
  return;
}


// Function: sub_13197 @ 0x13197
uint8 sub_13197(int8 a0,uint8 a1)
{
  uint8 v1; // stack - 0x28
  uint8 v2; // stack - 0x10
  
  if (0x9d <= a1) // branch-flip
    v1 = 0x9c;
  else {
    v1 = a1;
    if (*(char *)(a0 + (a1 - 1)) == '/')
      v1 = a1 - 1;
  }
  do {
    v2 = v1 - 1;
    if (!v2)
      return 0;
    v1 = v2;
  } while (*(char *)(v2 + a0) != '/');
  return v2;
}


// Function: sub_1320b @ 0x1320b
void * sub_1320b(char *a0) // early-return x2
{
  uint8 v1; // rax
  unsigned long v2; // rax
  uint8 v3; // rax
  void *v4; // rax
  
  v1 = strlen(a0);
  if (0x101 <= v1) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_5e548(a0);
    error(0,0,gettext("%s: file name is too long (max %d); not dumped"),v2,0x100);
    dat_9fdd0 = 2;
    return NULL;
  }
  v3 = sub_13197(a0,v1);
  if (((v3) && ((v1 - v3) - 1 <= 100)) && (v1 - v3 != 1)) {
    v4 = (void *)sub_bb4b();
    memset(v4,0,0x200);
    memcpy((void *)((int8)v4 + 0x159),a0,v3);
    memcpy(v4,&a0[v3 + 1],(v1 - v3) - 1);
    return v4;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = sub_5e548(a0);
  error(0,0,gettext("%s: file name is too long (cannot be split); not dumped"),v2);
  dat_9fdd0 = 2;
  return NULL;
}


// Function: sub_133a8 @ 0x133a8
void sub_133a8(int8 a0) // return-dupe x2
{
  unsigned long v1; // rax
  uint8 v2; // rdx
  
  if (dat_9f864 <= 6) {
    v2 = 1L << ((uint1)dat_9f864 & 0x3f);
    if (v2 & 0x2a) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = sub_5e548(*(unsigned long *)(a0 + 0x18));
      error(0,0,gettext("%s: link name is too long; not dumped"),v1);
      dat_9fdd0 = 2;
      return;
    }
    if (v2 & 0x44) {
      sub_12f9f(a0,*(unsigned long *)(a0 + 0x18),0x4b);
      return;
    }
    if (v2 & 0x10) {
      sub_1e1d3("linkpath",a0,0);
      return;
    }
  }
  abort(); // no-return
}


// Function: sub_134b6 @ 0x134b6
unsigned long sub_134b6(int8 a0) // early-return
{
  char *v1;
  unsigned long v2;
  
  switch(dat_9f864) {
    default:
      abort(); // no-return
    case 1:
      v1 = *(char **)(a0 + 8);
      if (100 <= strlen(v1)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v2 = sub_5e548(*(unsigned long *)(a0 + 8));
        error(0,0,gettext("%s: file name is too long (max %d); not dumped"),v2,99);
        dat_9fdd0 = 2;
        return 0;
      }
      break;
    case 2:
    case 6:
      sub_12f9f(a0,*(unsigned long *)(a0 + 8),0x4c);
      break;
    case 3:
    case 5:
      v2 = *(unsigned long *)(a0 + 8);
      return sub_1320b(v2);
    case 4:
      sub_1e1d3(0x80f06,a0,0);
    
  }
  v2 = sub_12f4e(a0);
  return v2;
}


// Function: sub_135e3 @ 0x135e3
void * sub_135e3(char a0,int8 a1,void *a2)
{
  void *v1; // rax
  char v2 [520];
  char v3; // stack - 0x234
  void *v4; // stack - 0x230
  unsigned long v5; // stack - 0x228
  
  if ((*(int8 *)(a1 + 0x178)) || (!*(int8 *)(a1 + 0x168)))
    v1 = a2;
  else {
    v1 = (int8)(a1 + 0x168);
    sub_1e672(v1);
    memcpy(v2,a2,0x200);
    if (a0) { // branch-flip
      v3 = 0x67;
      v1 = (void *)sub_1d18d();
      v4 = v1;
      v5 = dat_9f9e0;
    }
    else {
      v3 = 0x78;
      v1 = (void *)sub_1d123(a1);
      v4 = v1;
      if (dat_9f910) // branch-flip
        v1 = (void *)dat_9f920;
      else {
        v1 = *(void **)(a1 + 0xb0);
      }
      v5 = v1;
    }
    sub_1d280(v3,v4,v5,a1 + 0x168);
    free(v4);
    v1 = (void *)sub_bb4b();
    memcpy(v1,v2,0x200);
  }
  return v1;
}


// Function: sub_13775 @ 0x13775
void sub_13775(int8 a0) // return-dupe, ternary
{
  unsigned long v1;
  char *v2;
  uint8 v3; // rbx
  
  if ((dat_9f864 == 4) && (v1 = *(unsigned long *)(a0 + 8), sub_3c9b7(v1) != '\x01')) {
    sub_1e1d3(0x80f06,a0,0);
    sub_12f4e(a0);
    return;
  }
  v3 = (dat_9f864 != 2) ? 100 : 99; // branch-flip
  v2 = *(char **)(a0 + 8);
  if (v3 < strlen(v2)) {
    sub_134b6(a0);
    return;
  }
  sub_12f4e(a0);
  return;
}


// Function: sub_1381d @ 0x1381d
int8 sub_1381d(int8 a0) // ternary
{
  uint4 v1;
  int8 v10; // stack - 0x58
  uint8 v11; // stack - 0x38
  uint4 v12; // stack - 0x70
  uint4 v13; // stack - 0x6c
  uint4 v14; // stack - 0x68
  uint4 v15; // stack - 0x64
  uint8 v16; // stack - 0x50
  int8 v17; // stack - 0x48
  int8 v18; // stack - 0x40
  int8 v19; // stack - 0x30
  char *v2;
  char v3; // al
  uint4 v4; // eax
  int4 v5; // eax
  int8 v6;
  unsigned long v7;
  char v8; // dl
  int8 v9; // stack - 0x60
  
  v9 = 0;
  v10 = 0;
  v17 = sub_13775(a0);
  if (!v17)
    return 0;
  sub_29274(*(unsigned int *)(a0 + 0x74),a0 + 0x74,&v9);
  sub_293ce(*(unsigned int *)(a0 + 0x78),a0 + 0x78,&v10);
  if (dat_9f8f0) {
    v1 = *(uint4 *)(a0 + 0x70);
    v4 = sub_55b51(*(unsigned int *)(a0 + 0x70),(*(uint4 *)(a0 + 0x70) & 0xf000) == 0x4000,dat_9f8f8,dat_9f8f0,0);
    *(uint4 *)(a0 + 0x70) = v1 & 0xfffff000 | v4;
  }
  if ((dat_9f864 != 1) && (dat_9f864 != 3))
    sub_12a05(*(unsigned int *)(a0 + 0x70),v17 + 100,8);
  else {
    sub_12a05(*(uint4 *)(a0 + 0x70) & 0xfff,v17 + 100,8);
  }
  v12 = *(uint4 *)(a0 + 0x74);
  if ((dat_9f864 == 4) && (0x200000 <= v12)) {
    sub_1e1d3(0x80f28,a0,0);
    v12 = 0;
  }
  v6 = v17 + 0x6c;
  if (sub_12ba3(v12,v6,8) != '\x01')
    return 0;
  v13 = *(uint4 *)(a0 + 0x78);
  if ((dat_9f864 == 4) && (0x200000 <= v13)) {
    sub_1e1d3(0x80f2c,a0,0);
    v13 = 0;
  }
  v6 = v17 + 0x74;
  if (sub_12901(v13,v6,8) != '\x01')
    return 0;
  v16 = *(uint8 *)(a0 + 0x88);
  if ((dat_9f864 == 4) && (0x200000000 <= v16)) {
    sub_1e1d3(0x80f30,a0,0);
    v16 = 0;
  }
  v3 = sub_12a9a(v16,v17 + 0x7c,0xc);
  if (v3 != '\x01')
    return 0;
  if (dat_9f910 != 2) { // branch-flip
    if (dat_9f910 <= 2) {
      if (dat_9f910) { // branch-flip
        if (dat_9f910 == 1) {
          v11 = dat_9f920;
          v19 = dat_9f928;
        }
      }
      else {
        v19 = *(int8 *)(a0 + 0x100);
        v11 = *(uint8 *)(a0 + 0xf8);
      }
    }
  }
  else {
    v5 = sub_5ff90(*(unsigned long *)(a0 + 0xf8),*(unsigned long *)(a0 + 0x100),dat_9f920,dat_9f928);
    if (1 <= v5) { // branch-flip
      v11 = dat_9f920;
      v19 = dat_9f928;
    }
    else {
      v19 = *(int8 *)(a0 + 0x100);
      v11 = *(uint8 *)(a0 + 0xf8);
    }
  }
  if (dat_9f864 == 4) {
    if ((0x1ffffffff < v11) || (v19))
      sub_1e1d3("mtime",a0,&v11);
    if (0x200000000 <= v11)
      v11 = 0;
  }
  v3 = sub_12af2(v11,v17 + 0x88,0xc);
  if (v3 != '\x01')
    return 0;
  if (((*(uint4 *)(a0 + 0x70) & 0xf000) != 0x2000) && ((*(uint4 *)(a0 + 0x70) & 0xf000) != 0x6000)) {
    if (((dat_9f864 != 6) && (dat_9f864 != 2)) && ((v6 = v17 + 0x149, sub_12957(0,v6,8) != '\x01' || (v6 = v17 + 0x151, sub_129ae(0,v6,8) != '\x01'))))
      return 0;
  }
  else {
    v14 = gnu_dev_major(*(unsigned long *)(a0 + 0x80));
    v15 = gnu_dev_minor(*(unsigned long *)(a0 + 0x80));
    if ((dat_9f864 == 4) && (0x200000 <= v14)) {
      sub_1e1d3("devmajor",a0,0);
      v14 = 0;
    }
    v6 = v17 + 0x149;
    if (sub_12957(v14,v6,8) != '\x01')
      return 0;
    if ((dat_9f864 == 4) && (0x200000 <= v15)) {
      sub_1e1d3("devminor",a0,0);
      v15 = 0;
    }
    v6 = v17 + 0x151;
    if (sub_129ae(v15,v6,8) != '\x01')
      return 0;
  }
  if (dat_9f864 != 4) { // branch-flip
    if ((dat_9f8be) && ((dat_9f864 == 2 || (dat_9f864 == 6)))) {
      sub_12af2(*(unsigned long *)(a0 + 0xe8),v17 + 0x159,0xc);
      sub_12af2(*(unsigned long *)(a0 + 0x108),v17 + 0x165,0xc);
    }
  }
  else {
    sub_1e1d3("atime",a0,0);
    sub_1e1d3("ctime",a0,0);
  }
  v8 = (dat_9f864 != 1) ? 0x30 : 0; // branch-flip
  *(char *)(v17 + 0x9c) = v8;
  if (dat_9f864 != 6) {
    if ((6 < dat_9f864) || (4 < dat_9f864))
      abort(); // no-return, return-dupe
    if (3 <= dat_9f864) {
      memcpy((void *)(v17 + 0x101),"ustar",6);
      memcpy((void *)(v17 + 0x107),(void *)0x80e49,2);
      goto label_13edf;
    }
    if (dat_9f864 == 1) goto label_13edf;
    if (dat_9f864 != 2) {
      abort();
    }
  }
  strcpy((char *)(v17 + 0x101),"ustar  ");
label_13edf:
  if ((dat_9f864 != 1) && (!dat_9f934)) {
    if (v9) { // branch-flip
      v7 = sub_60cc6(v9);
      *(unsigned long *)(a0 + 0x20) = v7;
    }
    else {
      sub_2bfd1(*(unsigned int *)(a0 + 0x74),a0 + 0x20);
    }
    if (v10) { // branch-flip
      v7 = sub_60cc6(v10);
      *(unsigned long *)(a0 + 0x28) = v7;
    }
    else {
      sub_2c0a2(*(unsigned int *)(a0 + 0x78),a0 + 0x28);
    }
    if ((dat_9f864 == 4) && ((v2 = *(char **)(a0 + 0x20), 0x21 <= strlen(v2) || (v7 = *(unsigned long *)(a0 + 0x20), sub_3c9b7(v7) != '\x01'))))
      sub_1e1d3("uname",a0,0);
    sub_12c4c(*(unsigned long *)(a0 + 0x20),v17 + 0x109,0x20);
    if ((dat_9f864 == 4) && ((v2 = *(char **)(a0 + 0x28), 0x21 <= strlen(v2) || (v7 = *(unsigned long *)(a0 + 0x28), sub_3c9b7(v7) != '\x01'))))
      sub_1e1d3("gname",a0,0);
    sub_12c4c(*(unsigned long *)(a0 + 0x28),v17 + 0x129,0x20);
  }
  if (dat_9f864 == 4) {
    if (1 <= dat_9f968) {
      if (*(int8 *)(a0 + 0x38))
        sub_1e1d3("SCHILY.acl.access",a0,0);
      if (*(int8 *)(a0 + 0x48))
        sub_1e1d3("SCHILY.acl.default",a0,0);
    }
    if ((1 <= dat_9f964) && (*(int8 *)(a0 + 0x30)))
      sub_1e1d3("RHT.security.selinux",a0,0);
    if (1 <= dat_9f96c) {
      v11 = 0;
      v18 = *(int8 *)(a0 + 0x160);
      for (; v11 < *(uint8 *)(a0 + 0x158); v11 = v11 + 1) {
        sub_1e1d3(*(unsigned long *)(v18 + v11 * 0x18),a0,&v11);
      }
    }
  }
  return v17;
}


// Function: sub_1418c @ 0x1418c
void sub_1418c(uint1 *a0)
{
  uint1 *v1;
  int8 v2;
  int4 v3; // stack - 0x1c
  int8 v4; // stack - 0x18
  uint1 *v5; // stack - 0x10
  
  memcpy(&a0[0x94],"        ",8);
  v3 = 0;
  v4 = 0x200;
  v5 = a0;
  while (v2 = v4 + -1, v4) {
    v1 = &v5[1];
    v3 += (uint4)*v5;
    v4 = v2;
    v5 = v1;
  }
  sub_12bf9((int8)v3,&a0[0x94],7);
  sub_bba7(a0);
  return;
}


// Function: sub_1422e @ 0x1422e
void sub_1422e(unsigned long a0,int8 a1,unsigned long a2)
{
  if ((((dat_9f9ac) && (*(char *)(a1 + 0x9c) != 'K')) && (*(char *)(a1 + 0x9c) != 'L')) && ((*(char *)(a1 + 0x9c) != 'x' && (*(char *)(a1 + 0x9c) != 'g')))) {
    dat_9f468 = dat_9f864;
    sub_2864c(a0,a1,a2);
  }
  sub_1418c(sub_135e3(0,a0,a1));
  return;
}


// Function: sub_142d7 @ 0x142d7
void sub_142d7(int8 a0)
{
  void *v1; // rax
  int8 v2; // stack - 0x20
  
  for (v2 = a0; 0 < v2; v2 = v2 + -0x200) {
    v1 = (void *)sub_bb4b();
    memset(v1,0,0x200);
    sub_bba7(v1);
  }
  return;
}


// Function: sub_14327 @ 0x14327
unsigned long sub_14327(int4 a0,unsigned long *a1) // return-dupe
{
  unsigned long v1; // rax
  int8 v2; // rax
  uint8 v3;
  unsigned long v4; // rax
  char v5 [24];
  uint8 v6; // stack - 0x60
  uint8 v7; // stack - 0x58
  
  v6 = a1[0x11];
  v1 = sub_bacd();
  v2 = sub_1381d(a1);
  if (!v2)
    return 2;
  sub_1422e(a1,v2,v1);
  sub_ab09(a1[1],a1[0x11],a1[0x11]);
  while( true ) {
    if ((int8)v6 <= 0)
      return 0;
    v2 = sub_bb4b();
    v7 = sub_bbf5(v2);
    if (v6 < v7) {
      v7 = v6;
      v3 = (uint8)((uint4)v6 & 0x1ff);
      if (v3)
        memset((void *)(v2 + v6),0,0x200 - v3);
    }
    if (1 <= a0) // branch-flip
      v3 = sub_2a9e9(a0,v2,v7);
    else {
      v3 = v7;
    }
    if (v3 == 0xffffffffffffffff) break;
    v6 -= v3;
    sub_bba7(v2 + (v7 - 1 & 0xfffffffffffffe00));
    if (v3 != v7) {
      memset((void *)(v2 + v3),0,v7 - v3);
      if (dat_9ec84 & 0x80) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v1 = sub_54e5a(v6,v5);
        v4 = sub_5e548(*a1);
        error(0,0,(char *)ngettext("%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v6),v4,v1);
      }
      if (dat_9f8bc != '\x01')
        sub_39e98(1);
      sub_142d7(v6 + (v3 - v7));
      return 1;
    }
  }
  sub_2b2f8(*a1,a1[0x11] - v6,v7);
  sub_142d7(v6);
  return 1;
}


// Function: sub_145e3 @ 0x145e3
void sub_145e3(unsigned long *a0,char *a1)
{
  int8 v1;
  uint8 v10; // stack - 0x80
  void *v11; // stack - 0x78
  char *v12; // stack - 0x70
  uint8 v13; // stack - 0x68
  char *v14; // stack - 0x60
  void *v15; // stack - 0x58
  unsigned long v16; // stack - 0x50
  void *v17; // stack - 0x48
  uint8 v18; // stack - 0x40
  uint8 v19; // stack - 0x38
  char v2; // al
  uint8 v20; // stack - 0x30
  uint8 v21; // stack - 0x28
  int4 v3; // eax
  unsigned long v4;
  char *v5; // rax
  uint8 v6;
  uint8 v7; // rax
  char *v8; // stack - 0x90
  uint8 v9; // stack - 0x88
  
  v1 = a0[0x33];
  v15 = NULL;
  v16 = sub_bacd();
  a0[0x11] = 0;
  v15 = (void *)sub_1381d(a0);
  if (v15) {
    sub_17909(a0);
    if ((dat_9f8be) && (dat_9f864 != 4))
      *(char *)((int8)v15 + 0x9c) = 0x44;
    else {
      *(char *)((int8)v15 + 0x9c) = 0x35;
    }
    if (dat_9f8be != '\x01') // branch-flip
      sub_1422e(a0,v15,v16);
    else if (*(int8 *)(dat_9f6b0 + 0x38)) {
      if (dat_9f864 == 4) {
        v4 = *(unsigned long *)(dat_9f6b0 + 0x38);
        sub_1e1d3("GNU.dumpdir",a0,sub_22456(v4));
        sub_1422e(a0,v15,v16);
        return;
      }
      v16 = sub_bacd();
      v17 = (void *)sub_22456(*(unsigned long *)(dat_9f6b0 + 0x38));
      v18 = sub_20bfe(v17);
      sub_12a9a(v18,(int8)v15 + 0x7c,0xc);
      sub_1422e(a0,v15,v16);
      v11 = v17;
      v9 = v18;
      sub_ab09(a0[1],v18,v18);
      while (0 < (int8)v9) {
        v15 = (void *)sub_bb4b();
        v10 = sub_bbf5(v15);
        if (v9 < v10) {
          v10 = v9;
          v19 = (uint8)((uint4)v9 & 0x1ff);
          if (v19)
            memset((void *)(v9 + (int8)v15),0,0x200 - v19);
        }
        memcpy(v15,v11,v10);
        v9 -= v10;
        v11 = (void *)((int8)v11 + v10);
        sub_bba7((int8)v15 + (v10 - 1 & 0xfffffffffffffe00));
      }
      return;
    }
    if (dat_9f930) {
      if (((dat_9f935) && (v1)) && (*(int8 *)(a0[0x33] + 0x58) != a0[0xb])) {
        if ((dat_9f9ac) && (dat_9ec84 & 0x40000)) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v4 = sub_5e548(*a0);
          error(0,0,gettext("%s: file is on a different filesystem; not dumped"),v4);
        }
      }
      else {
        v3 = sub_11fbf(a0,&v8);
        if (v3 != 3) {
          switch(v3) {
            case 0:
              v12 = (char *)sub_60cc6(*a0);
              v20 = strlen(v12);
              v13 = v20;
              v14 = a1;
              while( true ) {
                v6 = strlen(v14);
                v21 = v6;
                if (!v6) break;
                if (v13 < v6 + v20) {
                  v13 = v6 + v20;
                  v1 = v13 + 1;
                  v12 = (char *)sub_60b8d(v12,v1);
                  v6 = v21;
                }
                v21 = v6;
                strcpy(&v12[v20],v14);
                v2 = sub_17c43(v12,a0);
                if (v2 != '\x01')
                  sub_16a25(a0,v14,v12);
                v14 = &v14[v21 + 1];
              }
              free(v12);
              break;
            case 1:
              v5 = gettext("contents not dumped");
              sub_11f18(*a0,v8,v5);
              v6 = strlen((char *)*a0);
              v7 = strlen(v8);
              v13 = v7 + v6 + 1;
              v12 = (char *)sub_60b5b(v13);
              strcpy(v12,(char *)*a0);
              strcat(v12,v8);
              sub_16a25(a0,v8,v12);
              free(v12);
              break;
            case 2:
              v5 = gettext("contents not dumped");
              sub_11f18(*a0,v8,v5);
            
          }
        }
      }
    }
  }
  return;
}


// Function: sub_14bcf @ 0x14bcf
void sub_14bcf(int8 *a0)
{
  char *v1;
  int8 v2;
  uint8 v3; // stack - 0x10
  
  v1 = (char *)*a0;
  for (v3 = strlen(v1); (v3 && (*(char *)((v3 - 1) + *a0) == '/')); v3 = v3 - 1) {
  }
  if (*(char *)(v3 + *a0) != '/') {
    v2 = *a0;
    *a0 = sub_60b8d(v2,v3 + 2);
  }
  *(char *)(v3 + *a0) = 0x2f;
  *(char *)(v3 + 1 + *a0) = 0;
  return;
}


// Function: sub_14c7c @ 0x14c7c
unsigned long sub_14c7c(int8 a0)
{
  int8 v1; // stack - 0x10
  
  if (((*__errno_location() == 0x18) && (a0)) && (*(int8 *)(a0 + 0x198))) {
    for (v1 = *(int8 *)(*(int8 *)(a0 + 0x198) + 0x198); v1; v1 = *(int8 *)(v1 + 0x198)) {
      if ((1 <= *(int4 *)(v1 + 0x1a8)) && ((!*(int8 *)(v1 + 0x198) || (*(int4 *)(*(int8 *)(v1 + 0x198) + 0x1a8) <= 0)))) {
        sub_39c7e(v1);
        return 1;
      }
    }
    *__errno_location() = 0x18;
  }
  return 0;
}


// Function: sub_14d3d @ 0x14d3d
unsigned long sub_14d3d(int8 a0)
{
  int4 v1;
  unsigned long v2; // rax
  
  do {
    v1 = *(int4 *)(a0 + 0x1a8);
    *(void **)(a0 + 0x1a0) = fdopendir(v1);
    if (*(int8 *)(a0 + 0x1a0)) {
      v2 = sub_5eee2(*(unsigned long *)(a0 + 0x1a0),dat_9fc2c);
      return v2;
    }
  } while (sub_14c7c(a0) == '\x01');
  return 0;
}


// Function: sub_14db4 @ 0x14db4
bool sub_14db4(unsigned long *a0)
{
  void *v1; // rax
  
  v1 = (void *)sub_14d3d(a0);
  if (v1) {
    sub_145e3(a0,v1);
    sub_1588e(a0);
    free(v1);
  }
  else {
    sub_2b3a4(*a0);
  }
  return v1 != NULL;
}


// Function: sub_14e23 @ 0x14e23
void sub_14e23(void) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_14e23
{
  uint8 v1;
  char *v10; // stack - 0x1f0
  uint8 v11; // stack - 0x1e8
  int8 v12; // stack - 0x1e0
  int8 v13; // stack - 0x1d8
  uint8 v14; // stack - 0x1d0
  int4 v15; // stack - 0x20
  unsigned long v2;
  char *v3;
  char v4; // al
  int4 v5; // eax
  uint8 v6; // stack - 0x200
  int8 v7 [11]; // stack - 0x1c8
  char v8 [336];
  void *v9; // stack - 0x1f8
  
  v5 = ((dat_9f6a0 != 2) && (dat_9f8a0 != '\x01'));
  dat_9f338 = (int8)v5;
  sub_f694(1);
  sub_ad11();
  if (dat_9f8be) { // branch-flip
    v6 = 0;
    v9 = NULL;
    sub_2e719();
    while( true ) {
      v13 = sub_2eda0();
      if (!v13) break;
      v2 = *(unsigned long *)(v13 + 0x10);
      if (sub_17c43(v2,0) != '\x01')
        sub_16a25(0,*(unsigned long *)(v13 + 0x10),*(unsigned long *)(v13 + 0x10));
    }
    sub_2ee43();
    while( true ) {
      v13 = sub_2eda0();
      if (!v13) break;
      v2 = *(unsigned long *)(v13 + 0x10);
      if (sub_17c43(v2,0) != '\x01') {
        v3 = *(char **)(v13 + 0x10);
        v11 = strlen(v3);
        while (v6 <= v11) {
          v9 = (void *)sub_60bd8(v9,&v6);
        }
        memcpy(v9,*(void **)(v13 + 0x10),v11);
        if (*(char *)((int8)v9 + (v11 - 1)) != '/') {
          v1 = v11 + 1;
          *(char *)(v11 + (int8)v9) = 0x2f;
          v11 = v1;
        }
        sub_39c55(v7);
        v10 = (char *)sub_2241b(*(unsigned long *)(v13 + 0x38));
        if (v10) {
          for (; *v10; v10 = &v10[v14 + 1]) {
            v14 = strlen(v10);
            if (*v10 == 'Y') {
              if (!v7[0]) {
                v5 = openat(dat_9c268,*(char **)(v13 + 0x10),dat_9fc0c);
                if (v5 <= -1) {
                  sub_2b48b(*(unsigned long *)(v13 + 0x10),*(int8 *)(v13 + 0x40) == 0,sub_2b2b1);
                  break;
                }
                v15 = v5;
                if (fstat(v5,v8)) {
                  sub_2b48b(*(unsigned long *)(v13 + 0x10),*(int8 *)(v13 + 0x40) == 0,sub_2b444);
                  break;
                }
                v2 = *(unsigned long *)(v13 + 0x10);
                v7[0] = sub_60cc6(v2);
              }
              while (v6 < v11 + v14) {
                v9 = (void *)sub_60bd8(v9,&v6);
              }
              strcpy((char *)(v11 + (int8)v9),&v10[1]);
              sub_16a25(v7,&v10[1],v9);
            }
          }
        }
        sub_39d1c(v7);
      }
    }
    free(v9);
  }
  else {
    while( true ) {
      v12 = sub_2d37c(1);
      if (!v12) break;
      v4 = sub_17c43(v12,0);
      if (v4 != '\x01')
        sub_16a25(0,v12,v12);
    }
  }
  sub_12da0();
  sub_d046();
  sub_3be6b();
  if (!dat_9f8e0)
    return;
  sub_24600();
  return;
}


// Function: sub_1529f @ 0x1529f
uint8 sub_1529f(uint8 *a0,uint8 a1)
{
  return (a0[1] ^ *a0) % a1;
}


// Function: sub_152df @ 0x152df
unsigned long sub_152df(uint8 *a0,uint8 *a1)
{
  uint8 v1; // rax
  
  v1 = a1[1] ^ a0[1] | *a0 ^ *a1;
  return CONCAT71((undefined7)(v1 >> 8),v1 == 0);
}


// Function: sub_15331 @ 0x15331
void sub_15331(unsigned long a0) // return-dupe
{
  unsigned long v1; // rax
  
  if (dat_9ec84 & 0x20) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v1 = sub_5e548(a0);
    error(0,0,gettext("%s: Unknown file type; file ignored"),v1);
  }
  if (dat_9f8bc == '\x01')
    return;
  sub_39e98(2);
  return;
}


// Function: sub_153bb @ 0x153bb
unsigned long sub_153bb(unsigned long *a0) // ternary
{
  int8 v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  uint8 v4; // rbx
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  
  if ((dat_9f340) && ((dat_9f338 < (uint8)a0[0xd] || (dat_9f94e)))) {
    v6 = a0[0xc];
    v5 = a0[0xb];
    v1 = sub_52567(dat_9f340,&v5);
    if (v1) {
      v2 = (char *)sub_3edd9(v1 + 0x18,1,dat_9f878);
      if (*(int8 *)(v1 + 0x10))
        *(int8 *)(v1 + 0x10) = *(int8 *)(v1 + 0x10) + -1;
      v3 = sub_bacd();
      sub_29547(&a0[3],v2);
      v4 = (dat_9f864 != 2) ? 100 : 99; // branch-flip
      if (v4 < strlen(v2))
        sub_133a8(a0);
      a0[0x11] = 0;
      v1 = sub_1381d(a0);
      if (!v1)
        return 0;
      sub_12157(v1 + 0x9d,v2,100);
      *(char *)(v1 + 0x9c) = 0x31;
      sub_1422e(a0,v1,v3);
      if (dat_9f94e)
        sub_3bebb(*a0,0);
      return 1;
    }
  }
  return 0;
}


// Function: sub_15598 @ 0x15598
void sub_15598(unsigned long *a0)
{
  unsigned long v1; // rax
  uint8 v2; // rax
  char *v3; // stack - 0x28
  unsigned long *v4; // stack - 0x20
  unsigned long *v5; // stack - 0x18
  
  if ((!dat_9f8a1) && (dat_9f338 < (uint8)a0[0xd])) {
    v3 = NULL;
    v1 = sub_3edd9(*a0,1,dat_9f878);
    sub_29547(&v3,v1);
    sub_3b9f2(&v3,2);
    v2 = strlen(v3);
    v4 = (unsigned long *)sub_60b5b(v2 + 0x19);
    v4[1] = a0[0xc];
    *v4 = a0[0xb];
    v4[2] = a0[0xd];
    strcpy((char *)&v4[3],v3);
    free(v3);
    if (((!dat_9f340) && (dat_9f340 = sub_52c1c(0,0,sub_1529f,sub_152df,0), !dat_9f340)) || (v5 = (unsigned long *)sub_5395d(dat_9f340,v4), !v5))
      sub_1783c(); // no-return
    if (v5 != v4)
      abort(); // no-return
    v4[2] = v4[2] + -1;
  }
  return;
}


// Function: sub_1573f @ 0x1573f
void sub_1573f(void) // return-dupe
{
  unsigned long v1; // rax
  int8 v2; // stack - 0x20
  
  if (!dat_9f340)
    return;
  for (v2 = sub_525fe(dat_9f340); v2; v2 = sub_5265e(dat_9f340,v2)) {
    if (*(int8 *)(v2 + 0x10)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = sub_5e84e(v2 + 0x18);
      error(0,0,gettext("Missing links to %s."),v1);
    }
  }
  return;
}


// Function: sub_157f9 @ 0x157f9
int4 sub_157f9(int8 a0,char *a1,int4 a2)
{
  int4 v1;
  
  if (dat_9f368 != '\x01') {
    dat_9f368 = '\x01';
    strerror(2);
    gettext("");
  }
  while( true ) {
    if (a0) // branch-flip
      v1 = *(int4 *)(a0 + 0x1a8);
    else {
      v1 = dat_9c268;
    }
    v1 = openat(v1,a1,a2);
    if (0 <= v1) break;
    if (!sub_14c7c(a0))
      return v1;
  }
  return v1;
}


// Function: sub_1588e @ 0x1588e
void sub_1588e(int8 a0)
{
  unsigned long *v1;
  int4 v2; // eax
  int8 v3; // stack - 0xa8
  int4 v4; // stack - 0xb8
  int8 v5; // stack - 0xa0
  
  v1 = *(unsigned long **)(a0 + 0x198);
  if ((v1) && (!*(int4 *)&v1[0x35])) {
    v4 = openat(*(int4 *)(a0 + 0x1a8),"..",dat_9fc0c);
    if (0 <= v4) { // branch-flip
      if (((fstat(v4,&v3)) || (v1[0xc] != v5)) || (v1[0xb] != v3)) {
        close(v4);
        v4 = 2;
      }
    }
    else {
      v4 = -*__errno_location();
    }
    if ((v4 <= -1) && (v2 = openat(dat_9c268,(char *)*v1,dat_9fc0c), 0 <= v2)) {
      if ((fstat(v4,&v3)) || ((v1[0xc] != v5 || (v1[0xb] != v3))))
        close(v2);
      else {
        v4 = v2;
      }
    }
    *(int4 *)&v1[0x35] = v4;
  }
  return;
}


// Function: sub_15a63 @ 0x15a63
void sub_15a63(unsigned long *a0,char *a1,unsigned long a2) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_15a63
{
  unsigned long v1;
  bool v10; // zf
  char v11 [16];
  bool v12; // stack - 0x108
  char v13; // stack - 0x107
  int4 v14; // stack - 0x104
  int4 v15; // stack - 0x100
  uint4 v16; // stack - 0xfc
  code *v17; // stack - 0xf0
  unsigned long v18; // stack - 0xe8
  int8 v19; // stack - 0xe0
  int4 v2; // eax
  int8 v20; // stack - 0xd8
  int8 v21; // stack - 0xd0
  unsigned long v22; // stack - 0xc8
  unsigned long v23; // stack - 0xc0
  int8 v24; // stack - 0x88
  unsigned long v3;
  char *v4;
  int8 v5;
  uint8 v6; // rbx
  unsigned long v7; // stack - 0xf8
  char v8 [48];
  bool v9; // zf
  
  v18 = 0xffffffffffffffff;
  v15 = 0;
  v19 = a0[0x33];
  v9 = v19 == 0;
  if (v9)
    v14 = dat_9c268;
  else {
    v14 = *(int4 *)(v19 + 0x1a8);
  }
  v17 = NULL;
  if ((dat_9f8c8) && (!sub_34af4(0x8109d,a2)))
    return;
  sub_29547(a0,a2);
  v3 = sub_3edd9(a2,0,dat_9f878);
  sub_29547(&a0[1],v3);
  sub_3b9f2(&a0[1],1);
  if ((0 <= v14) || (v9)) {
    v2 = fstatat(v14,a1,&a0[0xb],dat_9fc10);
    if (v2) // branch-flip
      v17 = sub_2b444;
    else if (sub_12c90(&a0[0xb])) {
      v15 = sub_157f9(v19,a1,dat_9fc08);
      if (0 <= v15) { // branch-flip
        *(int4 *)&a0[0x35] = v15;
        if (fstat(v15,&a0[0xb]))
          v17 = sub_2b444;
      }
      else {
        v17 = sub_2b2b1;
      }
    }
  }
  else {
    *__errno_location() = -v14;
    v17 = sub_2b2b1;
  }
  if (v17) {
    sub_2b48b(a2,v9,v17);
    return;
  }
  v20 = a0[0x11];
  a0[0x23] = v20;
  v11 = sub_5fd6a(&a0[0xb]);
  a0[0x1d] = SUB168(v11,0);
  a0[0x1e] = SUB168(v11,8);
  v11 = sub_5fd9e(&a0[0xb]);
  a0[0x1f] = SUB168(v11,0);
  a0[0x20] = SUB168(v11,8);
  v11 = sub_5fd84(&a0[0xb]);
  v23 = SUB168(v11,8);
  v22 = SUB168(v11,0);
  a0[0x21] = v22;
  a0[0x22] = v23;
  if (((((dat_9f8be != '\x01') || (v9)) && ((*(uint4 *)&a0[0xe] & 0xf000) != 0x4000)) && (v2 = sub_5ff90(a0[0x1f],a0[0x20],dat_9f900,dat_9f908), v2 <= -1)) && ((!dat_9f87c || (v2 = sub_5ff90(a0[0x21],a0[0x22],dat_9f900,dat_9f908), v2 <= -1)))) {
    if (dat_9f8be == '\x01')
      return;
    if (dat_9f9ac) {
      if (dat_9ec84 & 0x100) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v3 = sub_5e548(a2);
        error(0,0,gettext("%s: file is unchanged; not dumped"),v3);
        return;
      }
      return;
    }
    return;
  }
  if (sub_32b37(a0)) {
    if (dat_9ec84 & 0x400) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v3 = sub_5e548(a2);
      error(0,0,gettext("%s: file is the archive; not dumped"),v3);
      return;
    }
    return;
  }
  v10 = (*(uint4 *)&a0[0xe] & 0xf000) == 0x4000;
  if ((!v10) && (sub_153bb(a0)))
    return;
  if ((!v10) && ((*(uint4 *)&a0[0xe] & 0xf000) != 0x8000)) {
    if ((*(uint4 *)&a0[0xe] & 0xf000) == 0xa000) {
      v3 = a0[0x11];
      a0[3] = sub_47649(v14,a1,v3);
      if (!a0[3]) {
        if (*__errno_location() == 0xc)
          sub_1783c(); // no-return
        sub_2b48b(a2,v9,sub_2b35d);
        return;
      }
      sub_3b9f2(&a0[3],4);
      v6 = (dat_9f864 != 2) ? 100 : 99; // branch-flip
      v4 = (char *)a0[3];
      if (v6 < strlen(v4))
        sub_133a8(a0);
      sub_3d566(v14,a1,a0,0);
      sub_3d1a5(v14,a1,a0,0);
      v18 = sub_bacd();
      a0[0x11] = 0;
      v21 = sub_1381d(a0);
      if (v21) {
        sub_12157(v21 + 0x9d,a0[3],100);
        *(char *)(v21 + 0x9c) = 0x32;
        sub_1422e(a0,v21,v18);
        if (dat_9f94e)
          sub_3bebb(a2,0);
        sub_15598(a0);
        return;
      }
      return;
    }
    if ((*(uint4 *)&a0[0xe] & 0xf000) != 0x2000) { // branch-flip
      if ((*(uint4 *)&a0[0xe] & 0xf000) != 0x6000) { // branch-flip
        if ((*(uint4 *)&a0[0xe] & 0xf000) != 0x1000) {
          if ((*(uint4 *)&a0[0xe] & 0xf000) != 0xc000) {
            sub_15331(a2);
            return;
          }
          if (dat_9ec84 & 0x20) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v3 = sub_5e548(a2);
            error(0,0,gettext("%s: socket ignored"),v3);
            return;
          }
          return;
        }
        v13 = '6';
        sub_3cf89(v14,a1,a0,0,1);
        sub_3d566(v14,a1,a0,0);
        sub_3d1a5(v14,a1,a0,0);
      }
      else {
        v13 = '4';
        sub_3cf89(v14,a1,a0,0,1);
        sub_3d566(v14,a1,a0,0);
        sub_3d1a5(v14,a1,a0,0);
      }
    }
    else {
      v13 = '3';
      sub_3cf89(v14,a1,a0,0,1);
      sub_3d566(v14,a1,a0,0);
      sub_3d1a5(v14,a1,a0,0);
    }
    if (dat_9f864 == 1) {
      sub_15331(a2);
      return;
    }
    v18 = sub_bacd();
    a0[0x11] = 0;
    v21 = sub_1381d(a0);
    if (v21) {
      *(char *)(v21 + 0x9c) = v13;
      if (v13 != '6') {
        v5 = v21 + 0x149;
        v3 = a0[0x10];
        sub_12957(gnu_dev_major(v3),v5,8);
        v5 = v21 + 0x151;
        v3 = a0[0x10];
        sub_129ae(gnu_dev_minor(v3),v5,8);
      }
      sub_1422e(a0,v21,v18);
      if (dat_9f94e) {
        sub_3bebb(a2,0);
        return;
      }
      return;
    }
    return;
  }
  sub_3cf89(v14,a1,a0,0,!v10);
  sub_3d566(v14,a1,a0,v15);
  sub_3d1a5(v14,a1,a0,v15);
  if (v10) {
    sub_14bcf(a0);
    sub_14bcf(&a0[1]);
    if (sub_11fbf(a0,&v7) == 3) {
      v4 = gettext("directory not dumped");
      sub_11f18(*a0,v7,v4);
      return;
    }
    v12 = (bool)sub_14db4(a0);
    v15 = *(int4 *)&a0[0x35];
    if (v9)
      v14 = dat_9c268;
    else {
      v14 = *(int4 *)(v19 + 0x1a8);
    }
    goto label_16229;
  }
  if ((v15) && (dat_9f979)) {
    v5 = a0[0x11];
    if (v5 <= -1)
      v5 += 0x1ff;
    v2 = ((a0[0x11] & 0x1ff) && (0x3ff <= a0[0x11] + 0x1ff));
    if ((int8)v2 + (v5 >> 9) <= (int8)a0[0x13]) goto label_161c2;
    v16 = sub_30090(v15,a0);
    if (v16 == 3)
      v16 = sub_14327(v15,a0);
  }
  else {
label_161c2:
    v16 = sub_14327(v15,a0);
  }
  if (v16 == 3)
    abort(); // no-return
  if ((v16 <= 3) && (v16 < 2))
    sub_15598(a0);
  v12 = v16 == 0;
label_16229:
  if (v12) {
    if (0 <= v15) { // branch-flip
      if (v15) // branch-flip
        v12 = fstat(v15,v8) == 0;
      else if ((0 <= v14) || (v9)) {
        v2 = fstatat(v14,a1,v8,dat_9fc10);
        v12 = v2 == 0;
      }
      else {
        *__errno_location() = -v14;
        v12 = 0;
      }
    }
    else {
      *__errno_location() = -v15;
      v12 = 0;
    }
    if (v12 != 1)
      sub_2b48b(a2,v9,sub_2b444);
  }
  if (v12) {
    v11 = sub_5fd84(v8);
    v2 = sub_5ff90(SUB168(v11,0),SUB168(v11,8),v22,v23);
    if (((v2) && ((dat_9f94e != '\x01' || (!v10)))) || (v24 > v20)) {
      if (dat_9ec84 & 0x10) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v3 = sub_5e548(a2);
        error(0,0,gettext("%s: file changed as we read it"),v3);
      }
      sub_39e98(1);
    }
    else if ((((dat_9f880 == 1) && (v15)) && ((v10 || (v20)))) && (v3 = a0[0x1d], v1 = a0[0x1e], sub_2ab5e(v15,v14,a1,v3,v1)))
      sub_3ead0(a2);
  }
  if ((sub_39c7e(a0) & v12) && (dat_9f94e))
    sub_3bebb(a2,v10);
  return;
}


// Function: sub_16a25 @ 0x16a25
void sub_16a25(int8 a0,unsigned long a1,unsigned long a2)
{
  char v1 [408];
  int8 v2; // stack - 0x30
  
  sub_39c55(v1);
  v2 = a0;
  sub_15a63(v1,a1,a2);
  if ((a0) && (dat_9f8e0))
    sub_2130d(a0);
  sub_39d1c(v1);
  return;
}


// Function: sub_16add @ 0x16add
void sub_16add(int8 a0) // return-dupe
{
  char v1;
  int4 v2; // eax
  int8 v3; // rax
  int8 v4; // rax
  unsigned short v5 [2]; // stack - 0x18
  bool v6;
  int8 v7; // stack - 0x30
  int4 v8; // stack - 0x14
  
  if (!a0)
    return;
  v8 = (int4)a0;
  if (0 <= a0) { // branch-flip
    v5[0] = 3;
    v6 = a0 == v8;
  }
  else {
    v5[0] = 4;
    v8 = -v8;
    v6 = (int8)v8 == -a0;
  }
  if (v6) {
    if (0x40000000 <= dat_9f9cc) { // branch-flip
      sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v5);
      v1 = dat_3;
    }
    else {
      v2 = ioctl(dat_9f9cc,0x40086d01,v5);
      v1 = (char)((uint4)v2 >> 0x18);
    }
    if ('\xff' < v1)
      return;
    if (*__errno_location() == 5) {
      if (0x40000000 <= dat_9f9cc) { // branch-flip
        sub_40254(dat_9f9cc + -0x40000000,0x40086d01,v5);
        v1 = dat_3;
      }
      else {
        v2 = ioctl(dat_9f9cc,0x40086d01,v5);
        v1 = (char)((uint4)v2 >> 0x18);
      }
      if ('\xff' < v1)
        return;
    }
  }
  if (0x40000000 <= dat_9f9cc) // branch-flip
    v3 = sub_400be(dat_9f9cc + -0x40000000,0,1);
  else {
    v3 = lseek(dat_9f9cc,0,1);
  }
  v4 = a0 * dat_9f870;
  v7 = v4 + v3;
  if ((v4 / a0 == dat_9f870) && (SUB81((uint8)v4 >> 0x3f,0) == v7 < v3)) {
    if (v7 <= -1)
      v7 = 0;
    if (0x40000000 <= dat_9f9cc) // branch-flip
      v3 = sub_400be(dat_9f9cc + -0x40000000,v7,0);
    else {
      v3 = lseek(dat_9f9cc,v7,0);
    }
    if (v3 == v7)
      return;
  }
  sub_3e7a9(*dat_9fbd8,v7);
  return;
}


// Function: sub_16d4a @ 0x16d4a
void sub_16d4a(int4 a0)
{
  unsigned long v1;
  
  v1 = dat_9efa0;
  dat_9efa0 = dat_9f378;
  if (dat_9f384) { // branch-flip
    dat_9f9cc = 1;
    sub_f676();
    dat_9f9cc = 0;
  }
  else {
    sub_16add((dat_9f370 + dat_9efc8) - dat_9efc0);
    sub_f676();
  }
  dat_9efa0 = v1;
  if ((a0) && (dat_9f384 != '\x01'))
    sub_16add(dat_9efc0 - (dat_9f370 + dat_9efc8));
  dat_9f380 = 0;
  return;
}


// Function: sub_16e0f @ 0x16e0f
void sub_16e0f(int8 a0,uint8 a1)
{
  int4 v1;
  unsigned long *v2;
  unsigned long *v3;
  unsigned long *v4;
  uint8 v5; // rcx
  int8 v6; // rdx
  unsigned long *v7;
  uint8 v8; // stack - 0x10
  
  for (v8 = 0; v8 < a1; v8 = v8 + 1) {
    v7 = (unsigned long *)(v8 * 0x200 + a0);
    v1 = dat_9f380 + 1;
    v4 = (unsigned long *)((int8)dat_9f380 * 0x200 + dat_9f378);
    *v4 = *v7;
    v4[0x3f] = v7[0x3f];
    v6 = (int8)v4 - (int8)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
    v5 = (uint8)((int4)v6 + 0x200U >> 3);
    v7 = (unsigned long *)((int8)v7 - v6);
    v4 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
    while (v5) {
      v3 = &v4[1];
      v2 = &v7[1];
      *v4 = *v7;
      v5 -= 1;
      v7 = v2;
      v4 = v3;
    }
    dat_9f380 = v1;
    if (v1 == dat_9f868)
      sub_16d4a(1);
  }
  return;
}


// Function: sub_16ee1 @ 0x16ee1
void sub_16ee1(int8 a0,uint8 a1) // return-dupe
{
  uint8 v1; // rax
  
  v1 = a1 >> 9;
  a1 += v1 * -0x200;
  sub_16e0f(a0,v1);
  memcpy((void *)((int8)dat_9f380 * 0x200 + dat_9f378),(void *)(v1 * 0x200 + a0),a1);
  if (a1 <= 0x1ff)
    memset((void *)((int8)dat_9f380 * 0x200 + dat_9f378 + a1),0,0x200 - a1);
  dat_9f380 += 1;
  if (dat_9f380 != dat_9f868)
    return;
  sub_16d4a(1);
  return;
}


// Function: sub_16fd5 @ 0x16fd5
void sub_16fd5(void)
{
  int8 v1; // stack - 0x10
  
  sub_bba7(dat_9f460);
  v1 = dat_9faa8 + 0x1ff;
  if (dat_9faa8 + 0x1ff <= -1)
    v1 = dat_9faa8 + 0x3fe;
  v1 >>= 9;
  while (dat_9efa8 - dat_9efb0 >> 9 <= v1) {
    v1 -= dat_9efa8 - dat_9efb0 >> 9;
    sub_cac8();
  }
  dat_9efb0 = v1 * 0x200 + dat_9efb0;
  return;
}


// Function: sub_17069 @ 0x17069
void sub_17069(void)
{
  unsigned long *v1;
  bool v10;
  uint4 v11; // stack - 0x38
  uint4 v12; // stack - 0x34
  int4 v13; // stack - 0x30
  int4 v14; // stack - 0x2c
  int4 v15; // stack - 0x28
  int8 v16; // stack - 0x18
  unsigned long *v2;
  char *v3;
  uint4 v4; // eax
  int4 v5; // eax
  int8 v6; // rax
  uint8 v7; // rcx
  unsigned long *v8;
  unsigned long *v9;
  
  v11 = 0;
  v12 = 0;
  sub_2d3b1();
  sub_f694(2);
  v3 = (char *)*dat_9fbd8;
  dat_9f384 = strcmp(v3,"-") == 0;
  do {
    v4 = sub_25ecf(0x9f460,0x9fa20,1);
    switch(v4) {
      case 0:
        abort(); // no-return
      case 1:
        v6 = sub_2ed0e(dat_9fa28);
        if (v6) { // branch-flip
          *(int8 *)(v6 + 0x30) = *(int8 *)(v6 + 0x30) + 1;
          if (dat_9f8d0) // branch-flip
            v10 = *(int8 *)(v6 + 0x30) != dat_9f8d0;
          else {
            v10 = *(int8 *)(v6 + 0x30) == 0;
          }
          if (!v10) goto label_1718c;
          sub_28891();
        }
        else {
          sub_28891();
        }
        break;
      case 2:
label_1718c:
        v11 = v4;
        break;
      case 3:
        if (!dat_9f8bd) goto label_171b6;
        sub_bba7(dat_9f460);
        break;
      case 4:
label_171b6:
        v11 = 4;
        break;
      case 5:
        sub_bba7(dat_9f460);
        if ((v12 != 5) && (v12 <= 5)) {
          if (v12 == 4)
            abort(); // no-return
          if (v12 <= 4) {
            if (v12) { // branch-flip
              if (2 < v12 - 1) break;
            }
            else {
              if (dat_9fdc8)
                (*dat_9fdc8)();
              error(0,0,gettext("This does not look like a tar archive"));
            }
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Skipping to next header"));
            dat_9fdd0 = 2;
          }
        }
      
    }
    v12 = v4;
  } while (!v11);
  dat_9f370 = dat_9efc0 + -1;
  dat_9f378 = (void *)sub_60b5b(dat_9f870);
  if ((v11 == 1) || (v11 == 2)) {
    dat_9efd8 = 0;
    dat_9f380 = (int4)((int8)dat_9efb0 - (int8)dat_9efa0 >> 9);
    if (dat_9f380)
      memcpy(dat_9f378,dat_9efa0,(int8)(dat_9f380 << 9));
    if (v11 == 1) {
      v11 = 0;
      sub_16fd5();
    }
    while (v11 != 4) {
      if (dat_9efb0 == dat_9efa8)
        sub_cac8();
      switch(sub_25ecf(0x9f460,0x9fa20,0)) {
        case 0:
        case 2:
          abort(); // no-return
        case 1:
          sub_1df5f(0x9fa20);
          v6 = sub_2ed0e(dat_9fa28);
          if (v6) {
            *(int8 *)(v6 + 0x30) = *(int8 *)(v6 + 0x30) + 1;
            if (dat_9f8d0) // branch-flip
              v10 = *(int8 *)(v6 + 0x30) == dat_9f8d0;
            else {
              v10 = *(int8 *)(v6 + 0x30) != 0;
            }
            if (v10) {
              sub_16fd5();
              break;
            }
          }
          if (dat_9fb90) // branch-flip
            sub_16ee1(dat_9fb98,dat_9fb90);
          else {
            sub_16e0f(dat_9f470,dat_9f480);
            sub_16e0f(dat_9f478,dat_9f488);
          }
          v8 = (unsigned long *)((int8)dat_9f380 * 0x200 + (int8)dat_9f378);
          *v8 = *dat_9f460;
          v8[0x3f] = dat_9f460[0x3f];
          v6 = (int8)v8 - (int8)(unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
          v7 = (uint8)((int4)v6 + 0x200U >> 3);
          v9 = (unsigned long *)((int8)dat_9f460 - v6);
          v8 = (unsigned long *)((uint8)&v8[1] & 0xfffffffffffffff8);
          while (v7) {
            v2 = &v8[1];
            v1 = &v9[1];
            *v8 = *v9;
            v7 -= 1;
            v9 = v1;
            v8 = v2;
          }
          dat_9f380 += 1;
          v16 = dat_9faa8 + 0x1ff;
          if (dat_9faa8 + 0x1ff <= -1)
            v16 = dat_9faa8 + 0x3fe;
          v16 >>= 9;
          sub_bba7(dat_9f460);
          if (dat_9f380 == dat_9f868)
            sub_16d4a(1);
          v13 = (int4)((int8)dat_9efa8 - (int8)dat_9efb0 >> 9);
          if (v16 < v13)
            v13 = (int4)v16;
          while (v16) {
            if (dat_9efb0 == dat_9efa8) {
              sub_f662();
              dat_9efb0 = dat_9efa0;
              v13 = dat_9f868;
              if (v16 < dat_9f868)
                v13 = (int4)v16;
            }
            v14 = v13;
            if (dat_9f868 - dat_9f380 < v13)
              v14 = dat_9f868 - dat_9f380;
            if (!v14)
              abort(); // no-return
            memcpy((void *)((int8)dat_9f380 * 0x200 + (int8)dat_9f378),dat_9efb0,(int8)(v14 << 9));
            dat_9f380 = v14 + dat_9f380;
            dat_9efb0 = (void *)((int8)v14 * 0x200 + (int8)dat_9efb0);
            v16 -= v14;
            v13 -= v14;
            if (dat_9f380 == dat_9f868)
              sub_16d4a(1);
          }
          break;
        case 3:
          if (dat_9f8bd) // branch-flip
            sub_bba7(dat_9f460);
          else {
            v11 = 4;
          }
          break;
        case 4:
          v11 = 4;
          break;
        case 5:
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Deleting non-header from archive"));
          dat_9fdd0 = 2;
          sub_bba7(dat_9f460);
          break;
        default:
          abort(); // no-return
        
      }
      sub_39d1c(0x9fa20);
    }
    v15 = 0;
    do {
      v5 = dat_9f868 - dat_9f380;
      memset((void *)((int8)dat_9f378 + (int8)dat_9f380 * 0x200),0,(int8)(v5 * 0x200));
      v15 += v5;
      sub_16d4a(v15 <= 1);
    } while (v15 <= 1);
    if (((dat_9f384 != '\x01') && (dat_9f9cc <= 0x3fffffff)) && (v5 = sub_32f52(dat_9f9cc), v5))
      sub_3ea7e(*dat_9fbd8);
  }
  free(dat_9f378);
  sub_d046();
  sub_2dbba();
  return;
}


// Function: sub_177f4 @ 0x177f4
void sub_177f4(void)
{
  if (dat_9f388)
    (*dat_9f388)();
  error(2,0,gettext("Error is not recoverable: exiting now"));
  return;
}


// Function: sub_1783c @ 0x1783c
void sub_1783c(void)
{
  error(0,0,"%s",gettext("memory exhausted"));
  sub_177f4(); // no-return
}


// Function: sub_17879 @ 0x17879
void sub_17879(char *a0,unsigned int a1) // return-dupe
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_60b5b(strlen(a0) + 0x10);
  *v1 = 0;
  *(unsigned int *)&v1[1] = a1;
  strcpy((char *)((int8)v1 + 0xc),a0);
  if (dat_9f398) {
    *dat_9f398 = v1;
    dat_9f398 = v1;
    return;
  }
  dat_9f390 = v1;
  dat_9f398 = v1;
  return;
}


// Function: sub_17909 @ 0x17909
void sub_17909(int8 a0) // return-dupe
{
  code *v1;
  unsigned long *v10; // rax
  unsigned long *v11; // stack - 0x50
  unsigned long *v12; // stack - 0x48
  unsigned long *v13; // stack - 0x40
  unsigned long v2;
  unsigned long v3;
  unsigned int v4;
  int4 v5;
  void *v6; // rax
  char *v7; // rax
  unsigned long v8; // rax
  int8 v9; // rax
  
  v12 = NULL;
  v13 = NULL;
  if (*(int8 *)(a0 + 0x1b0))
    return;
  v11 = dat_9f390;
  do {
    if (!v11) {
      *(unsigned long **)(a0 + 0x1b0) = v12;
      return;
    }
    if (a0) // branch-flip
      v4 = *(unsigned int *)(a0 + 0x1a8);
    else {
      v4 = dat_9c268;
    }
    if (!faccessat(v4,(int8)v11 + 0xc,0,0)) {
      v5 = sub_157f9(a0,(int8)v11 + 0xc,0);
      if (v5 != -1) { // branch-flip
        v6 = fdopen(v5,"r");
        if (v6) { // branch-flip
          v8 = sub_4e53e();
          v9 = sub_18242((int8)v11 + 0xc);
          if (*(int8 *)(v9 + 0x18)) {
            v1 = *(code **)(v9 + 0x18);
            v2 = *(unsigned long *)(v9 + 0x20);
            *(unsigned long *)(v9 + 0x20) = (*v1)(v2);
          }
          v2 = *(unsigned long *)(v9 + 0x20);
          v3 = *(unsigned long *)(v9 + 0x10);
          if (sub_4f189(v3,v8,v6,0x50000001,10,v2)) {
            v5 = *__errno_location();
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,v5,"%s",sub_5e548((int8)v11 + 0xc));
            sub_177f4(); // no-return
          }
          fclose(v6);
          v10 = (unsigned long *)sub_60b5b(0x20);
          v10[3] = v8;
          if (*(int4 *)&v11[1]) // branch-flip
            v4 = *(unsigned int *)(v9 + 8);
          else {
            v4 = *(unsigned int *)&v11[1];
          }
          *(unsigned int *)&v10[2] = v4;
          v10[1] = v13;
          *v10 = 0;
          if (v13) // branch-flip
            *v13 = v10;
          else {
            v12 = v10;
          }
          v13 = v10;
        }
        else {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v7 = gettext("%s: fdopen failed");
          error(0,*__errno_location(),v7,(int8)v11 + 0xc);
          dat_9fdd0 = 2;
          close(v5);
        }
      }
      else {
        sub_3e38b((int8)v11 + 0xc);
      }
    }
    v11 = (unsigned long *)*v11;
  } while( true );
}


// Function: sub_17bda @ 0x17bda
void sub_17bda(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = *(unsigned long **)(a0 + 0x1b0);
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_4e904(v2[3]);
    free(v2);
    v2 = v1;
  }
  *(unsigned long *)(a0 + 0x1b0) = 0;
  return;
}


// Function: sub_17c43 @ 0x17c43
char sub_17c43(char *a0,int8 a1) // early-return x2
{
  char v1; // al
  int8 v2; // stack - 0x38
  char v3; // stack - 0x25
  uint4 v4; // stack - 0x24
  unsigned long *v5; // stack - 0x20
  char *v6; // stack - 0x18
  void *v7; // stack - 0x10
  
  v6 = NULL;
  v7 = NULL;
  v4 = 0;
  v1 = sub_4ed6a(dat_9f8a8,a0);
  if (v1)
    return '\x01';
  if (!a1)
    return '\0';
  v3 = '\0';
  v2 = a1;
  do {
    if ((!v2) || (v3 == '\x01')) {
      free(v7);
      return v3;
    }
    for (v5 = *(unsigned long **)(v2 + 0x1b0); v5; v5 = (unsigned long *)*v5) {
      if (!(*(uint4 *)&v5[2] & v4)) {
        v3 = sub_4ed6a(v5[3],a0);
        if (v3) break;
        if (!v6) {
          for (v6 = a0; (*v6 == '.' && (v6[1] == '/')); v6 = &v6[2]) {
          }
        }
        v3 = sub_4ed6a(v5[3],v6);
        if (v3) break;
        if (!v7)
          v7 = (void *)sub_4e13e(a0);
        v3 = sub_4ed6a(v5[3],v7);
        if (v3) break;
      }
    }
    v2 = *(int8 *)(v2 + 0x198);
    v4 = 2;
  } while( true );
}


// Function: sub_17ddd @ 0x17ddd
void sub_17ddd(unsigned long a0,unsigned long a1,unsigned int a2) // return-dupe
{
  uint8 v1; // stack - 0xf8
  uint8 v2; // stack - 0x100
  int8 v3; // stack - 0xf0
  
  if (sub_461a6(a1,&v1,0x844))
    return;
  for (v2 = 0; v2 < v1; v2 = v2 + 1) {
    sub_4ee5d(a0,*(unsigned long *)(v2 * 8 + v3),a2);
  }
  sub_46399(&v1);
  return;
}


// Function: sub_17eb5 @ 0x17eb5
void sub_17eb5(unsigned long a0,char *a1,unsigned int a2)
{
  char *v1; // stack - 0x18
  
  v1 = a1;
  while( true ) {
    if (!(*(uint2 *)((int8)*v1 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000)) break;
    v1 = &v1[1];
  }
  if ((*v1) && (*v1 != '#')) {
    if ((*v1 == '\\') && (v1[1] == '#'))
      v1 = &v1[1];
    sub_4ee5d(a0,v1,a2);
  }
  return;
}


// Function: sub_17f4f @ 0x17f4f
void sub_17f4f(unsigned long a0,char *a1,uint4 a2)
{
  uint4 v1; // stack - 0x1c
  char *v2; // stack - 0x18
  
  v2 = a1;
  while( true ) {
    if (!(*(uint2 *)((int8)*v2 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000)) break;
    v2 = &v2[1];
  }
  if ((*v2) && (*v2 != '#')) {
    v1 = a2;
    if (*v2 == '!') {
      if (v2[1] != '!') { // branch-flip
        v1 = a2 | 0x20000000;
        v2 = &v2[1];
      }
      else {
        v2 = &v2[2];
      }
    }
    if (!strncmp(v2,"RE:",3)) {
      v2 = &v2[3];
      v1 = v1 & 0xefffffff | 0x8000000;
    }
    sub_4ee5d(a0,v2,v1);
  }
  return;
}


// Function: sub_18025 @ 0x18025
unsigned int * sub_18025(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x9f3a0;
  *a0 = 0x8000000;
  return a0;
}


// Function: sub_18059 @ 0x18059
void sub_18059(unsigned long a0,char *a1,uint4 a2,uint4 *a3)
{
  uint8 v1; // rax
  char *v2; // rax
  uint4 v3; // stack - 0x3c
  char *v4; // stack - 0x38
  
  v4 = a1;
  while (*(uint2 *)((int8)*v4 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000) {
    v4 = &v4[1];
  }
  if ((*v4) && (*v4 != '#')) {
    if (strncmp(v4,"syntax:",7)) { // branch-flip
      v1 = strlen(v4);
      v3 = a2;
      if (v4[v1 - 1] == '/') {
        v2 = (char *)sub_60b5b(v1);
        memcpy(v2,v4,v1 - 1);
        v2[v1 - 1] = '\0';
        v4 = v2;
        sub_4e34b(a0,v2);
        v3 = a2 | 0x4000008;
      }
      if (*a3 != 0x8000000) // branch-flip
        v3 &= 0xf7ffffff;
      else {
        v3 &= 0xefffffff;
      }
      sub_4ee5d(a0,v4,v3 | *a3);
    }
    else {
      v4 = &v4[7];
      while (*(uint2 *)((int8)*v4 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000) {
        v4 = &v4[1];
      }
      if (strcmp(v4,"regexp")) { // branch-flip
        if (!strcmp(v4,"glob"))
          *a3 = 0x10000000;
      }
      else {
        *a3 = 0x8000000;
      }
    }
  }
  return;
}


// Function: sub_18242 @ 0x18242
int8 * sub_18242(char *a0)
{
  char *v1;
  int8 *v2; // stack - 0x10
  
  v2 = (int8 *)0x9c160;
  while ((*v2 && (v1 = (char *)*v2, strcmp(v1,a0)))) {
    v2 = &v2[5];
  }
  return v2;
}


// Function: sub_18293 @ 0x18293
void sub_18293(void)
{
  int8 *v1; // stack - 0x10
  
  for (v1 = (int8 *)0x9c160; *v1; v1 = &v1[5]) {
    sub_17879(*v1,0);
  }
  return;
}


// Function: sub_182d5 @ 0x182d5
unsigned int sub_182d5(int4 a0) // return-dupe
{
  if ((a0 != 0x26) && (a0 != 0x5f))
    return 1;
  return 0;
}


// Function: sub_182fd @ 0x182fd
void sub_182fd(void) // return-dupe
{
  dat_9f3a8 = geteuid() == 0;
  dat_9f960 += (uint4)(uint1)dat_9f3a8;
  dat_9f95c += (uint4)(uint1)dat_9f3a8;
  dat_9f3ac = umask(0);
  if (1 <= dat_9f960) {
    dat_9f3b0 = 0;
    return;
  }
  umask(dat_9f3ac);
  dat_9f3b0 = dat_9f3ac;
  return;
}


// Function: sub_18387 @ 0x18387
int4 * sub_18387(int4 a0,unsigned long a1,uint4 a2,unsigned int a3)
{
  int4 v1;
  uint4 v2;
  uint4 v3; // eax
  int4 *v4; // rax
  
  if (0 <= a0) { // branch-flip
    v3 = fchmod(a0,a2);
    v2 = v3;
    if (v3) {
      v4 = __errno_location();
      v1 = *v4;
      if (!sub_182d5(v1)) {
        v4 = (uint8)fchmodat(dat_9c268,a1,a2,a3); // return-dupe
        return v4;
      }
    }
    v4 = (uint8)(uint8)v2;
  }
  else {
    v4 = (uint8)fchmodat(dat_9c268,a1,a2,a3);
  }
  return v4;
}


// Function: sub_183ee @ 0x183ee
unsigned long sub_183ee(unsigned int a0,unsigned long a1,uint4 a2,int4 a3,int4 a4) // early-return
{
  int4 v1; // stack - 0xc
  
  if (sub_18387(a0,a1,a2,a3)) // branch-flip
    v1 = *__errno_location();
  else {
    v1 = 0;
  }
  if (((v1 == 1) && (a2 & 0x800)) && (!sub_5c835())) {
    if (sub_18387(a0,a1,a2,a3)) // branch-flip
      v1 = *__errno_location();
    else {
      v1 = 0;
    }
    sub_5c826();
  }
  if (((a3) && (a4 != 0x32)) && (sub_182d5(v1) != '\x01')) {
    if (sub_18387(a0,a1,a2,0)) // branch-flip
      v1 = *__errno_location();
    else {
      v1 = 0;
    }
  }
  if ((v1) && ((a4 != 0x32 || (sub_182d5(v1))))) {
    *__errno_location() = v1;
    return 0xffffffff;
  }
  return 0;
}


// Function: sub_184f7 @ 0x184f7
int4 * sub_184f7(int4 a0,unsigned long a1,uint4 a2,uint4 a3,unsigned int a4)
{
  int4 v1;
  uint4 v2;
  uint4 v3; // eax
  int4 *v4; // rax
  
  if (0 <= a0) { // branch-flip
    v3 = fchown(a0,a2,a3);
    v2 = v3;
    if (v3) {
      v4 = __errno_location();
      v1 = *v4;
      if (!sub_182d5(v1)) {
        v4 = (uint8)fchownat(dat_9c268,a1,a2,a3,a4); // return-dupe
        return v4;
      }
    }
    v4 = (uint8)(uint8)v2;
  }
  else {
    v4 = (uint8)fchownat(dat_9c268,a1,a2,a3,a4);
  }
  return v4;
}


// Function: sub_1856b @ 0x1856b
void sub_1856b(int4 a0,char *a1,void *a2,int4 a3) // return-dupe
{
  if (0 <= a0) {
    fstat(a0,a2);
    return;
  }
  fstatat(dat_9c268,a1,a2,a3);
  return;
}


// Function: sub_185b8 @ 0x185b8
void sub_185b8(unsigned long a0,uint4 a1,uint4 a2,unsigned int a3,uint4 a4,uint4 a5,unsigned int a6,unsigned int a7)
{
  char v1; // al
  int4 v2; // eax
  uint4 v3; // eax
  char v4 [24];
  uint4 v5; // stack - 0xc0
  uint4 v6; // stack - 0x90
  
  v1 = (char)a6;
  if ((~a5 | a4 ^ a1) & a2) {
    v5 = a4;
    if (~(a2 | a5) & 0xfff) {
      v2 = sub_1856b(a3,a0,v4,a7);
      if (v2) {
        sub_3ea03(a0);
        return;
      }
      v5 = v6;
    }
    v3 = a1 & a2 | ~a2 & v5 & 0xfff;
    if (((v5 & 0xfff) != v3) && (v2 = sub_183ee(a3,a0,v3,a7,(int4)v1), v2))
      sub_3e0b2(a0,v3);
  }
  return;
}


// Function: sub_18719 @ 0x18719
void sub_18719(unsigned long a0,int8 a1,int8 a2) // return-dupe x5
{
  int4 v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  int8 v4; // stack - 0x68
  char v5 [40];
  int8 v6; // stack - 0x60
  int8 v7; // stack - 0x58
  int8 v8; // stack - 0x50
  
  if (a1 <= -1) {
    if (!(dat_9ec84 & 0x8000))
      return;
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_27abc(a1,a2,1);
    error(0,0,gettext("%s: implausibly old time stamp %s"),a0,v2);
    return;
  }
  v1 = sub_5ff90(dat_9f9f0,dat_9f9f8,a1,a2);
  if (0 <= v1)
    return;
  sub_521fa(&v4);
  v1 = sub_5ff90(v4,v6,a1,a2);
  if (0 <= v1)
    return;
  v7 = a1 - v4;
  v8 = a2 - v6;
  if (v8 <= -1) {
    v8 += 1000000000;
    v7 -= 1;
  }
  if (!(dat_9ec84 & 0x8000))
    return;
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = sub_29f2e(v7,v8,v5);
  v3 = sub_27abc(a1,a2,1);
  error(0,0,gettext("%s: time stamp %s is %s s in the future"),a0,v3,v2);
  return;
}


// Function: sub_18913 @ 0x18913
void sub_18913(unsigned long a0,int8 a1,unsigned int a2,uint4 a3,uint4 a4,char a5,unsigned int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned long v10; // stack - 0x20
  unsigned int v2;
  char v3; // al
  int4 v4;
  unsigned long v5; // stack - 0x38
  unsigned long v6; // r10
  uint4 v7; // stack - 0x64
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = (char)a6;
  if ((dat_9f99a != '\x01') && (v3 != '\x01')) {
    if (dat_9f8be) { // branch-flip
      v8 = *(unsigned long *)(a1 + 0xf0);
      v5 = *(unsigned long *)(a1 + 0xe8);
    }
    else {
      v8 = 0x3ffffffe;
    }
    v10 = *(unsigned long *)(a1 + 0x100);
    v9 = *(unsigned long *)(a1 + 0xf8);
    v4 = sub_4f4ec(a2,dat_9c268,a0,&v5,a7);
    if (v4) { // branch-flip
      if (a5 == '2') {
        v4 = *__errno_location();
        if (!sub_182d5(v4)) goto label_18a37;
      }
      sub_3ead0(a0);
    }
    else {
      if (dat_9f8be)
        sub_18719(a0,v5,v8);
      sub_18719(a0,v9,v10);
    }
  }
label_18a37:
  v7 = a4;
  if ((1 <= dat_9f95c) && (v3 != '\x01')) {
    v1 = *(unsigned int *)(a1 + 0x74);
    v2 = *(unsigned int *)(a1 + 0x78);
    v4 = sub_184f7(a2,a0,v1,v2,a7);
    if (v4) { // branch-flip
      if (a5 == '2') {
        v4 = *__errno_location();
        if (!sub_182d5(v4)) goto label_18acc;
      }
      sub_3e169(a0,v1,v2);
    }
    else if ((~a4 | a3) & 0x49)
      v7 = a4 & ~(a3 & 0xc00);
  }
label_18acc:
  if ((1 <= dat_9f960) && (v3 != '\x01'))
    v6 = 0xfff;
  else {
    v6 = 0x1ff;
  }
  sub_185b8(a0,*(uint4 *)(a1 + 0x70) & ~dat_9f3b0,v6,a2,a3,v7,(int4)a5,a7);
  sub_3d8a1(a1,a0,(int4)a5,1);
  sub_3d003(a1,a0,(int4)a5);
  sub_3d609(a1,a0,(int4)a5);
  return;
}


// Function: sub_18b89 @ 0x18b89
unsigned long * sub_18b89(char *a0)
{
  uint8 v1;
  char *v2;
  unsigned long *v3; // stack - 0x10
  
  v3 = dat_9f3b8;
  while ((v3 && (((*(char *)&v3[0xb] == '\x01' || (v1 = v3[0x13], v2 = (char *)v3[0x14], strncmp(a0,v2,v1))) || ((a0[v3[0x13]] != '/' || (v2 = (char *)sub_4df20(a0), v2 != &a0[v3[0x13] + 1]))))))) {
    v3 = (unsigned long *)*v3;
  }
  return v3;
}


// Function: sub_18c40 @ 0x18c40
void sub_18c40(int8 *a0)
{
  int8 v1;
  int8 v2; // stack - 0xa8
  int8 *v3; // stack - 0xb0
  int8 v4; // stack - 0xa0
  
  v3 = a0;
  do {
    *(char *)&v3[0xb] = 1;
    v1 = v3[0x14];
    if (sub_2a9b8(v1,&v2)) // branch-flip
      sub_3ea03(v3[0x14]);
    else {
      v3[1] = v2;
      v3[2] = v4;
    }
    v3 = (int8 *)*v3;
  } while ((v3) && (*(char *)&v3[0xb] != '\x01'));
  return;
}


// Function: sub_18d26 @ 0x18d26
void sub_18d26(char *a0,int8 a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int a5) // return-dupe
{
  char *v1;
  unsigned long v2;
  int8 v3;
  int4 v4; // eax
  uint8 v5; // rax
  unsigned long v6; // stack - 0xa8
  unsigned long *v7; // stack - 0xb8
  unsigned long v8; // stack - 0xa0
  
  v5 = strlen(a0);
  v7 = dat_9f3b8;
  while ((v7 && (v1 = (char *)v7[0x14], strcmp(v1,a0)))) {
    v7 = (unsigned long *)*v7;
  }
  if (v7) { // branch-flip
    if (*(char *)&v7[10]) {
      v4 = fstatat(dat_9c268,(char *)v7[0x14],&v6,*(int4 *)((int8)v7 + 0x54));
      if (v4) // branch-flip
        sub_3ea03(v7[0x14]);
      else {
        v7[1] = v6;
        v7[2] = v8;
      }
    }
  }
  else {
    v7 = (unsigned long *)sub_60b5b(0xa8);
    *v7 = dat_9f3b8;
    v7[0x13] = v5;
    dat_9f3b8 = v7;
    v7[0x14] = sub_60cc6(a0);
    *(char *)&v7[0xb] = 0;
    if (a1) {
      v7[1] = *(unsigned long *)(a1 + 0x58);
      v7[2] = *(unsigned long *)(a1 + 0x60);
    }
  }
  *(unsigned int *)&v7[3] = a4;
  if (a1) {
    *(unsigned int *)((int8)v7 + 0x1c) = *(unsigned int *)(a1 + 0x74);
    *(unsigned int *)&v7[4] = *(unsigned int *)(a1 + 0x78);
    v2 = *(unsigned long *)(a1 + 0xf0);
    v7[5] = *(unsigned long *)(a1 + 0xe8);
    v7[6] = v2;
    v2 = *(unsigned long *)(a1 + 0x100);
    v7[7] = *(unsigned long *)(a1 + 0xf8);
    v7[8] = v2;
  }
  *(unsigned int *)&v7[9] = a2;
  *(unsigned int *)((int8)v7 + 0x4c) = a3;
  *(bool *)&v7[10] = a1 == 0;
  *(unsigned int *)((int8)v7 + 0x54) = a5;
  *(unsigned int *)((int8)v7 + 0x5c) = dat_9f600;
  v7[0xc] = 0;
  if ((a1) && (sub_29547(&v7[0xc],*(unsigned long *)(a1 + 0x30)), *(int8 *)(a1 + 0x38))) {
    v3 = *(int8 *)(a1 + 0x40);
    v2 = *(unsigned long *)(a1 + 0x38);
    v7[0xd] = sub_60c8e(v2,v3 + 1);
    v7[0xe] = *(unsigned long *)(a1 + 0x40);
  }
  else {
    v7[0xd] = 0;
    v7[0xe] = 0;
  }
  if ((a1) && (*(int8 *)(a1 + 0x48))) {
    v3 = *(int8 *)(a1 + 0x50);
    v2 = *(unsigned long *)(a1 + 0x48);
    v7[0xf] = sub_60c8e(v2,v3 + 1);
    v7[0x10] = *(unsigned long *)(a1 + 0x50);
  }
  else {
    v7[0xf] = 0;
    v7[0x10] = 0;
  }
  if (a1) // branch-flip
    sub_1d854(a1,&v7[0x12],&v7[0x11]);
  else {
    v7[0x12] = 0;
    v7[0x11] = 0;
  }
  if (!sub_2a1fe(a0))
    return;
  sub_18c40(v7);
  return;
}


// Function: sub_191c5 @ 0x191c5
void sub_191c5(unsigned long a0,int8 *a1)
{
  int4 v1; // eax
  unsigned long v2; // rax
  int8 v3; // stack - 0xb8
  unsigned long *v4; // stack - 0xc0
  int8 v5; // stack - 0xb0
  unsigned int v6; // stack - 0xa0
  
  v4 = dat_9f3b8;
  while( true ) {
    if (!v4) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v2 = sub_5e548(a0);
      error(0,0,gettext("%s: Unexpected inconsistency when making directory"),v2);
      dat_9fdd0 = 2;
      return;
    }
    v1 = fstatat(dat_9c268,(char *)v4[0x14],&v3,*(int4 *)((int8)v4 + 0x54));
    if (v1) break;
    if ((v3 == *a1) && (v5 == a1[1])) {
      v4[1] = dat_9fa78;
      v4[2] = dat_9fa80;
      *(unsigned int *)&v4[3] = dat_9fa90;
      *(unsigned int *)((int8)v4 + 0x1c) = dat_9fa94;
      *(unsigned int *)&v4[4] = dat_9fa98;
      v4[5] = dat_9fb08;
      v4[6] = dat_9fb10;
      v4[7] = dat_9fb18;
      v4[8] = dat_9fb20;
      *(unsigned int *)&v4[9] = v6;
      *(unsigned int *)((int8)v4 + 0x4c) = 0xffffffff;
      *(char *)&v4[10] = 0;
      return;
    }
    v4 = (unsigned long *)*v4;
  }
  sub_3ea03(v4[0x14]);
  return;
}


// Function: sub_193d1 @ 0x193d1
void sub_193d1(void *a0)
{
  free(*(void **)((int8)a0 + 0xa0));
  sub_1d55d(*(unsigned long *)((int8)a0 + 0x90),*(unsigned long *)((int8)a0 + 0x88));
  free(*(void **)((int8)a0 + 0x60));
  free(*(void **)((int8)a0 + 0x68));
  free(*(void **)((int8)a0 + 0x78));
  free(a0);
  return;
}


// Function: sub_19454 @ 0x19454
void sub_19454(char *a0)
{
  unsigned long *v1;
  char *v2;
  unsigned long *v3; // stack - 0x20
  unsigned long *v4; // stack - 0x18
  
  v4 = NULL;
  v3 = dat_9f3b8;
  while( true ) {
    if (!v3)
      return;
    v1 = (unsigned long *)*v3;
    if ((*(int4 *)((int8)v3 + 0x5c) == dat_9f600) && (v2 = (char *)v3[0x14], !strcmp(v2,a0))) break;
    v4 = v3;
    v3 = v1;
  }
  sub_193d1(v3);
  if (v4) {
    *v4 = v1;
    return;
  }
  dat_9f3b8 = v1;
  return;
}


// Function: sub_194f9 @ 0x194f9
void sub_194f9(char *a0,char *a1)
{
  char *v1;
  unsigned long *v2; // stack - 0x10
  
  v2 = dat_9f3b8;
  while( true ) {
    if (!v2)
      return;
    if ((*(int4 *)((int8)v2 + 0x5c) == dat_9f600) && (v1 = (char *)v2[0x14], !strcmp(v1,a0))) break;
    v2 = (unsigned long *)*v2;
  }
  free((void *)v2[0x14]);
  v2[0x14] = sub_60cc6(a1);
  v2[0x13] = strlen(a1);
  return;
}


// Function: sub_195aa @ 0x195aa
int4 sub_195aa(char *a0,char *a1) // ternary
{
  int4 v1;
  uint4 v2; // eax
  uint4 v3; // eax
  char v4 [152];
  int4 v5; // stack - 0xc8
  char *v6; // stack - 0xb8
  
  v6 = a0;
  do {
    if (!*v6)
      return 0;
    if ((((*v6 == '/') && (v6 != a0)) && (v6[-1] != '/')) && ((v6[-1] != '.' || (((v6 != &a0[1] && (v6[-2] != '/')) && ((v6[-2] != '.' || ((v6 != &a0[2] && (v6[-3] != '/')))))))))) {
      *v6 = '\0';
      v2 = ~dat_9f3ac & 0x1ff;
      v3 = (dat_9f3a8) ? 0 : 0xc0; // branch-flip
      v5 = mkdirat(dat_9c268,a0,v3 | v2);
      if (v5) { // branch-flip
        if (*__errno_location() != 0x11) { // branch-flip
          v1 = *__errno_location();
          v5 = fstatat(dat_9c268,a0,v4,0);
          if (v5) {
            *__errno_location() = v1;
            sub_3e310(a0);
          }
        }
        else {
          v5 = 0;
        }
      }
      else {
        sub_18d26(a0,0,~dat_9f3b0 & (v3 | v2),0x1ff,v2,0x100);
        sub_286af(a0,(int4)v6 - (int4)a0,v2);
        *a1 = 1;
      }
      *v6 = '/';
      if (v5)
        return v5;
    }
    v6 = &v6[1];
  } while( true );
}


// Function: sub_19851 @ 0x19851
unsigned int sub_19851(unsigned long a0,char *a1,int8 a2) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  char v3 [152];
  char v4 [16];
  char *v5; // stack - 0xb8
  
  v5 = a1;
  if (!a1) {
    if (sub_2a9b8(a0,v3)) {
      if (*__errno_location() != 2) {
        sub_3ea2c(a0);
        return 1;
      }
      return 0;
    }
    v5 = v3;
  }
  if ((*(uint4 *)&v5[0x18] & 0xf000) == 0x4000)
    return 0;
  v4 = sub_5fd9e(v5);
  v1 = *(unsigned long *)(a2 + 0xf8);
  v2 = *(unsigned long *)(a2 + 0x100);
  if (1 <= (int4)sub_39e35(v1,v2,SUB168(v4,0),SUB168(v4,8)))
    return 0;
  return 1;
}


// Function: sub_19950 @ 0x19950
unsigned long sub_19950(char *a0,char a1,char *a2) // return-dupe
{
  int4 v1;
  char v2 [152];
  char *v3; // stack - 0xb0
  
  v1 = *__errno_location();
  v3 = NULL;
  if (*a2)
    return 0;
  if (v1 != 0x5f) { // branch-flip
    if (0x60 <= v1) goto label_19ba6;
    if (v1 == 0x28) goto label_19a11;
    if (0x28 < v1) goto label_19ba6;
    if (v1 == 0x1f) goto label_19a11;
    if (0x1f < v1) goto label_19ba6;
    if (v1 != 2) {
      if (v1 != 0x11) goto label_19ba6;
      goto label_19a89;
    }
  }
  else {
label_19a11:
    if (((a1 != '\x01') || (dat_9f8d8 != 2)) || (dat_9f8a0)) goto label_19ba6;
    if (strchr(a0,0x2f)) {
      if (sub_2a9b8(a0,v2)) goto label_19ba6;
      v3 = v2;
    }
label_19a89:
    switch(dat_9f8d8) {
      case 0:
      case 1:
      case 2:
label_19b47:
        if (1 <= (int4)sub_2a36f(a0,0))
          return 1;
        break;
      case 3:
        break;
      case 4:
        return 0;
      case 5:
        if (!(dat_9ec84 & 0x100000))
          return 2;
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("%s: skipping existing file"),a0);
        return 2;
      case 6:
        if (!sub_19851(a0,v3,0x9fa20)) goto label_19b47;
      
    }
  }
  if ((!sub_195aa(a0,a2)) && (*a2))
    return 1;
label_19ba6:
  *__errno_location() = v1;
  return 0;
}


// Function: sub_19bce @ 0x19bce
unsigned long sub_19bce(unsigned long a0,int8 a1,uint4 a2,char a3,unsigned int *a4) // return-dupe
{
  int4 v1; // eax
  char v2; // stack - 0x15
  uint4 v3; // stack - 0x14
  
  v2 = 0;
  if ((dat_9f96c <= 0) || (!*(int8 *)(a1 + 0x158)))
    return 0;
  v3 = ~dat_9f3b0 & dat_9fa90 & 0x1ff;
  while( true ) {
    v1 = mknodat(dat_9c268,a0,v3 ^ a2,0);
    if (!v1) {
      sub_3d8a1(a1,a0,(int4)a3,0);
      *a4 = 1;
      return 0;
    }
    v1 = sub_19950(a0,0,&v2);
    if (v1 == 2) break;
    if ((v1 <= 2) && (!v1)) {
      sub_28891();
      sub_3e38b(a0);
      return 1;
    }
  }
  return 0;
}


// Function: sub_19ce9 @ 0x19ce9
void sub_19ce9(char *a0,uint1 a1)
{
  char v1;
  int8 v10; // stack - 0x268
  char v11 [48];
  bool v12; // zf
  unsigned int v13; // stack - 0x280
  unsigned int v14; // stack - 0x27c
  int8 v15; // stack - 0x260
  unsigned int v16; // stack - 0x250
  unsigned long v17; // stack - 0x1a8
  unsigned long v18; // stack - 0x1a0
  unsigned long v19; // stack - 0x198
  uint8 v2;
  unsigned long v20; // stack - 0x190
  unsigned long v21; // stack - 0x188
  unsigned int v22; // stack - 0x168
  unsigned int v23; // stack - 0x164
  unsigned int v24; // stack - 0x160
  unsigned long v25; // stack - 0xf0
  unsigned long v26; // stack - 0xe8
  unsigned long v27; // stack - 0xe0
  unsigned long v28; // stack - 0xd8
  unsigned long v29; // stack - 0x80
  void *v3;
  unsigned long v30; // stack - 0x78
  bool v4;
  bool v5;
  unsigned long *v6;
  int4 v7; // eax
  uint8 v8; // rax
  unsigned long v9; // rax
  
  v8 = strlen(a0);
  v4 = 0;
  while( true ) {
    v6 = dat_9f3b8;
    if (!dat_9f3b8)
      return;
    v5 = 0;
    v13 = *(unsigned int *)&dat_9f3b8[9];
    v14 = *(unsigned int *)((int8)dat_9f3b8 + 0x4c);
    v1 = *(char *)&dat_9f3b8[0xb];
    v12 = v1 || v4;
    if (a1 < *(uint1 *)&dat_9f3b8[0xb]) break;
    if (((((uint8)dat_9f3b8[0x13] < v8) && (a0[dat_9f3b8[0x13]])) && ((a0[dat_9f3b8[0x13]] == '/' || (a0[dat_9f3b8[0x13] + -1] == '/')))) && (v2 = dat_9f3b8[0x13], v3 = (void *)dat_9f3b8[0x14], !memcmp(a0,v3,v2)))
      return;
    sub_2adf1(*(unsigned int *)((int8)v6 + 0x5c));
    if (v1 || v4) {
      v7 = fstatat(dat_9c268,(char *)v6[0x14],&v10,*(int4 *)((int8)v6 + 0x54));
      if (v7) { // branch-flip
        sub_3ea03(v6[0x14]);
        v5 = 1;
      }
      else {
        v13 = v16;
        v14 = 0xffffffff;
        if ((v10 != v6[1]) || (v15 != v6[2])) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v9 = sub_5e548(v6[0x14]);
          error(0,0,gettext("%s: Directory renamed before its status could be extracted"),v9);
          dat_9fdd0 = 2;
          v5 = 1;
        }
      }
    }
    if (!v5) {
      v22 = *(unsigned int *)&v6[3];
      v23 = *(unsigned int *)((int8)v6 + 0x1c);
      v24 = *(unsigned int *)&v6[4];
      v26 = v6[6];
      v25 = v6[5];
      v28 = v6[8];
      v27 = v6[7];
      v17 = v6[0xc];
      v18 = v6[0xd];
      v19 = v6[0xe];
      v20 = v6[0xf];
      v21 = v6[0x10];
      v30 = v6[0x12];
      v29 = v6[0x11];
      sub_18913(v6[0x14],v11,0xffffffff,v13,v14,0x35,*(char *)&v6[10],*(unsigned int *)((int8)v6 + 0x54));
    }
    dat_9f3b8 = (unsigned long *)*v6;
    sub_193d1(v6);
    v4 = v12;
  }
  return;
}


// Function: sub_1a10f @ 0x1a10f
unsigned long sub_1a10f(char *a0)
{
  int4 v1;
  char v2;
  int4 v3; // eax
  int4 *v4; // rax
  char v5 [24];
  uint4 v6; // stack - 0x90
  
  v1 = *__errno_location();
  v3 = fstatat(dat_9c268,a0,v5,0x100);
  if (((v3) || ((v6 & 0xf000) != 0xa000)) || ((v3 = fstatat(dat_9c268,a0,v5,0), v3 || ((v6 & 0xf000) != 0x4000))))
    v2 = 0;
  else {
    v2 = 1;
  }
  v4 = __errno_location();
  *v4 = v1;
  return CONCAT71((undefined7)((uint8)v4 >> 8),v2);
}


// Function: sub_1a1ec @ 0x1a1ec
uint4 sub_1a1ec(int8 a0) // ternary
{
  uint4 v1; // eax
  uint4 v2; // eax
  
  if ((1 <= dat_9f95c) || (1 <= dat_9f960))
    v1 = 0x1c0;
  else {
    v1 = 0x1ff;
  }
  v2 = (dat_9f3a8) ? 0 : 0xc0; // branch-flip
  return v2 | *(uint4 *)(a0 + 0x18) & v1;
}


// Function: sub_1a238 @ 0x1a238
int4 sub_1a238(unsigned long a0,int4 a1)
{
  int4 v1; // eax
  unsigned long v10; // stack - 0x98
  uint4 v11; // stack - 0x90
  unsigned int v12; // stack - 0x8c
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x80
  unsigned long v15; // stack - 0x78
  unsigned long v16; // stack - 0x70
  unsigned long v17; // stack - 0x68
  unsigned long v18; // stack - 0x60
  unsigned long v19; // stack - 0x58
  int8 v2; // stack - 0xa8
  unsigned long v20; // stack - 0x50
  unsigned long v21; // stack - 0x48
  unsigned long v22; // stack - 0x40
  unsigned long v23; // stack - 0x38
  unsigned long v24; // stack - 0x30
  unsigned long v25; // stack - 0x28
  unsigned long v26; // stack - 0x20
  char v3; // stack - 0xbd
  int4 v4; // stack - 0xbc
  uint4 v5; // stack - 0xb8
  unsigned int v6; // stack - 0xb4
  unsigned int v7; // stack - 0xb0
  uint4 v8; // stack - 0xac
  unsigned long v9; // stack - 0xa0
  
  v5 = 0;
  v6 = 0;
  v7 = 0;
  v3 = '\0';
  if ((dat_9f935) && (!dat_9fc20)) {
    v1 = fstatat(dat_9c268,".",&v2,0);
    if (v1) // branch-flip
      sub_2b444(0x8137b);
    else {
      dat_9fc20 = v2;
    }
  }
  if (dat_9f8be) // branch-flip
    sub_25377(a0);
  else if (a1 == 0x44)
    sub_28891();
  v8 = sub_1a1ec(0x9fa78);
  do {
    v4 = mkdirat(dat_9c268,a0,v8);
    if (!v4) {
      v5 = ~dat_9f3b0 & v8;
      v6 = 0x1ff;
      v7 = 0x100;
      break;
    }
    if ((*__errno_location() == 0x11) && ((((v3 || (dat_9f8dc)) || (dat_9f8d8 == 1)) || ((!dat_9f8d8 || (dat_9f8d8 == 2)))))) {
      if ((dat_9f8dc) && (sub_1a10f(a0)))
        return 0;
      if (!sub_2a9b8(a0,&v2)) {
        v5 = v11;
        v6 = 0xffffffff;
        if ((v11 & 0xf000) == 0x4000) {
          if (v3) {
            sub_191c5(a0,&v2);
            return 0;
          }
          if (dat_9f8d8 == 1) {
            v8 = sub_1a1ec(&v2);
            v4 = sub_183ee(0xffffffff,a0,v8,0x100,0x35);
            if (v4) // branch-flip
              sub_3e0b2(a0,v8);
            else {
              dat_9fa78 = v2;
              dat_9fa80 = v9;
              dat_9fa90 = CONCAT44(v12,v11);
              dat_9fa88 = v10;
              dat_9fa98 = v13;
              dat_9faa0 = v14;
              dat_9faa8 = v15;
              dat_9fab0 = v16;
              dat_9fab8 = v17;
              dat_9fac0 = v18;
              dat_9fac8 = v19;
              dat_9fad0 = v20;
              dat_9fad8 = v21;
              dat_9fae0 = v22;
              dat_9fae8 = v23;
              dat_9faf0 = v24;
              dat_9faf8 = v25;
              dat_9fb00 = v26;
              v5 = ~dat_9f3b0 & v8;
              v6 = 0x1ff;
              v7 = 0x100;
            }
          }
          break;
        }
      }
      *__errno_location() = 0x11;
    }
    v1 = sub_19950(a0,0,&v3);
    if ((v1 == 2) || (2 < v1)) break;
    if (!v1) {
      if (*__errno_location() != 0x11) {
        sub_3e310(a0);
        return 1;
      }
      break;
    }
  } while (v1 == 1);
  if (((!v4) || (!dat_9f8d8)) || (dat_9f8d8 == 2))
    sub_18d26(a0,0x9fa20,v5,v6,dat_9fa90 & 0xffffffff,v7);
  return v4;
}


// Function: sub_1a6b5 @ 0x1a6b5
int4 sub_1a6b5(char *a0,int4 a1,uint4 a2,int4 a3,uint4 *a4,unsigned int *a5) // ternary
{
  int4 v1;
  int4 v2; // eax
  char v3 [24];
  bool v4; // zf
  uint4 v5; // stack - 0xb4
  uint4 v6; // stack - 0x90
  
  v4 = dat_9f8d8 != 2;
  if (v4)
    v5 = 0x809c1;
  else {
    v5 = (dat_9f8a0) ? 0x80b41 : 0xa0b41; // branch-flip
  }
  if (a3)
    v5 &= 0xffffff7f;
  if (((a1 == 0x37) && (!dat_9f3c8)) && (dat_9f3c8 = 1, dat_9ec84 & 8)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Extracting contiguous files as regular files"));
  }
  v2 = openat(dat_9c268,a0,v5,(uint8)a2);
  if (0 <= v2) {
    if (v4) {
      *a4 = ~dat_9f3b0 & a2;
      *a5 = 0x1ff;
    }
    else {
      if (fstat(v2,v3)) {
        v1 = *__errno_location();
        close(v2);
        *__errno_location() = v1;
        return -1;
      }
      if ((v6 & 0xf000) != 0x8000) {
        close(v2);
        *__errno_location() = 0x11;
        return -1;
      }
      *a4 = v6;
      *a5 = 0xffffffff;
    }
  }
  return v2;
}


// Function: sub_1a8d0 @ 0x1a8d0
int4 sub_1a8d0(unsigned long a0,unsigned int a1) // early-return, ternary x2
{
  uint4 v1; // eax
  uint4 v10; // stack - 0x3c
  int4 v11; // stack - 0x38
  int4 v12; // stack - 0x34
  uint8 v13; // stack - 0x28
  int8 v14; // stack - 0x20
  uint8 v15; // stack - 0x18
  int4 v2; // eax
  uint8 v3; // stack - 0x30
  char v4; // stack - 0x4d
  unsigned int v5; // stack - 0x48
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // r8
  int4 v8; // stack - 0x44
  uint4 v9; // stack - 0x40
  
  v4 = 0;
  v1 = (1 <= dat_9f95c) ? 0xffffffc0 : 0xffffffff; // branch-flip
  v9 = v1 & dat_9fa90 & 0x1ff;
  if (1 <= dat_9f95c) // branch-flip
    v10 = v1 & dat_9fa90 & 0x3f;
  else {
    v10 = 0;
  }
  v6 = 0;
  v5 = 0;
  if (dat_9f998) // branch-flip
    v8 = 1;
  else if (dat_9f9a0) { // branch-flip
    v8 = sub_34323(a0,0x66,0x9fa20);
    if (v8 <= -1) {
      sub_28891();
      return 0;
    }
  }
  else {
    v3 &= 0xffffffff00000000;
    v2 = sub_19bce(a0,0x9fa20,v10,(int4)(char)a1,&v3);
    if (v2)
      return 1;
    while( true ) {
      v8 = sub_1a6b5(a0,a1,v9,v3 & 0xffffffff,&v6,&v5);
      if (0 <= v8) break;
      v11 = sub_19950(a0,1,&v4);
      if (v11 != 1) {
        sub_28891();
        if (v11 == 2)
          return 0;
        sub_3e38b(a0);
        return 1;
      }
    }
  }
  sub_ad2b(0x9fa20);
  if (dat_9fb40) // branch-flip
    sub_302a4(v8,0x9fa20,&v3);
  else {
    v3 = dat_9faa8;
    do {
      if ((int8)v3 <= 0) goto label_1ab9d;
      sub_ad86(v3);
      v14 = sub_bb4b();
      if (!v14) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Unexpected EOF in archive"));
        dat_9fdd0 = 2;
        goto label_1ab9d;
      }
      v13 = sub_bbf5(v14);
      if (v3 < v13)
        v13 = v3;
      *__errno_location() = 0;
      v15 = sub_2aa95(v8,v14,v13);
      v3 -= v13;
      sub_bba7(v14 + (v13 - 1));
    } while (v15 == v13);
    if (!dat_9f9a0)
      sub_3eb4b(a0,v15,v13);
  }
label_1ab9d:
  sub_287c1(v3);
  sub_ad66();
  if (dat_9f998)
    return 0;
  if (!dat_9f9a0) {
    v7 = (dat_9f8d8 != 2) ? 0x100 : 0; // branch-flip
    sub_18913(a0,0x9fa20,v8,v6,v5,(int4)(char)a1,0,v7);
  }
  v12 = close(v8);
  if (v12 <= -1)
    sub_3e1ff(a0);
  if (dat_9f9a0)
    sub_343ea();
  return v12;
}


// Function: sub_1ac61 @ 0x1ac61
unsigned long * sub_1ac61(char *a0) // early-return x2, return-dupe
{
  int4 v1; // eax
  int8 v2; // stack - 0xa8
  unsigned long *v3; // stack - 0xb0
  int8 v4; // stack - 0xa0
  
  if (!dat_9f3c0)
    return NULL;
  v1 = fstatat(dat_9c268,a0,&v2,0x100);
  if (!v1) {
    for (v3 = dat_9f3c0; (v3 && ((v3[1] != v2 || (v3[2] != v4)))); v3 = (unsigned long *)*v3) {
    }
    return v3;
  }
  if (*__errno_location() == 2)
    return NULL;
  sub_3ea03(a0);
  return NULL;
}


// Function: sub_1ad5b @ 0x1ad5b
unsigned long sub_1ad5b(char *a0,char a1,unsigned long a2,unsigned long *a3) // return-dupe x2
{
  int4 v1; // eax
  uint8 v2; // rax
  unsigned long *v3; // rax
  int8 v4; // rax
  unsigned long v5; // stack - 0xb8
  char v6 [16];
  unsigned long v7; // stack - 0xb0
  
  do {
    v1 = openat(dat_9c268,a0,0xc1,0);
    if (0 <= v1) {
      if (fstat(v1,&v5)) {
        sub_3ea03(a0);
        close(v1);
        return 0xffffffff;
      }
      if (close(v1)) {
        sub_3e1ff(a0);
        return 0xffffffff;
      }
      v2 = strlen(dat_9fa38);
      v3 = (unsigned long *)sub_60b5b(v2 + 0xa1);
      if (a3) { // branch-flip
        *v3 = *a3;
        *a3 = v3;
      }
      else {
        *v3 = dat_9f3c0;
        dat_9f3c0 = v3;
      }
      v3[1] = v5;
      v3[2] = v7;
      v6 = sub_5fdb8(&v5);
      v3[3] = SUB168(v6,0);
      v3[4] = SUB168(v6,8);
      *(char *)&v3[5] = a1;
      if (a1) {
        *(unsigned int *)((int8)v3 + 0x2c) = dat_9fa90;
        *(unsigned int *)&v3[6] = dat_9fa94;
        *(unsigned int *)((int8)v3 + 0x34) = dat_9fa98;
        v3[7] = dat_9fb08;
        v3[8] = dat_9fb10;
        v3[9] = dat_9fb18;
        v3[10] = dat_9fb20;
      }
      *(unsigned int *)&v3[0xb] = dat_9f600;
      v2 = strlen(a0);
      v3[0xc] = sub_60b5b(v2 + 9);
      *(unsigned long *)v3[0xc] = 0;
      strcpy((char *)(v3[0xc] + 8),a0);
      v3[0xd] = 0;
      sub_29547(&v3[0xd],dat_9fa50);
      v3[0xe] = 0;
      v3[0xf] = 0;
      v3[0x10] = 0;
      v3[0x11] = 0;
      sub_1d854(0x9fa20,&v3[0x13],&v3[0x12]);
      strcpy((char *)&v3[0x14],dat_9fa38);
      v4 = sub_18b89(a0);
      if (!v4)
        return 0;
      sub_18c40(v4);
      return 0;
    }
    if ((*__errno_location() == 0x11) && (sub_1ac61(a0)))
      return 0;
    v1 = sub_19950(a0,0,a2);
    if (v1 == 2)
      return 0;
  } while ((3 <= v1) || (v1));
  sub_3e38b(a0);
  return 0xffffffff;
}


// Function: sub_1b161 @ 0x1b161
unsigned long sub_1b161(char *a0) // early-return x3
{
  unsigned long v1;
  int4 v10; // stack - 0x160
  int4 v11; // stack - 0x15c
  unsigned long *v12; // stack - 0x158
  char *v13; // stack - 0x150
  int8 v14; // stack - 0x148
  unsigned long *v15; // stack - 0x140
  int8 v16; // stack - 0x130
  int8 v17; // stack - 0xa0
  char v2; // al
  int4 v3; // eax
  unsigned long v4;
  char v5; // stack - 0x165
  int8 v6; // stack - 0x138
  int8 v7; // stack - 0xa8
  char v8 [16];
  int4 v9; // stack - 0x164
  
  v5 = 0;
  v13 = dat_9fa38;
  if ((dat_9f878 != '\x01') && (v2 = sub_2efdc(dat_9fa38), v2))
    v4 = sub_1ad5b(a0,0,&v5,0);
  else {
    v14 = sub_1ac61(v13);
    if (!v14) {
      do {
        v9 = linkat(dat_9c268,v13,dat_9c268,a0,0);
        v10 = *__errno_location();
        if (!v9) {
          v12 = dat_9f3c0;
          if ((dat_9f3c0) && (v3 = fstatat(dat_9c268,v13,&v6,0x100), !v3)) {
            for (; v12; v12 = (unsigned long *)*v12) {
              if (((*(int4 *)&v12[0xb] == dat_9f600) && (v12[1] == v6)) && (v12[2] == v16)) {
                v8 = sub_5fdb8(&v6);
                v4 = v12[3];
                v1 = v12[4];
                if (!sub_5ff90(v4,v1,SUB168(v8,0),SUB168(v8,8))) {
                  v15 = (unsigned long *)sub_60b5b(strlen(a0) + 9);
                  strcpy((char *)&v15[1],a0);
                  *v15 = v12[0xc];
                  v12[0xc] = v15;
                  return 0;
                }
              }
            }
          }
          return 0;
        }
        if (((v10 == 0x11) && (v3 = strcmp(v13,a0), !v3)) || (((v3 = fstatat(dat_9c268,v13,&v6,0x100), !v3 && ((v3 = fstatat(dat_9c268,a0,&v7,0x100), !v3 && (v6 == v7)))) && (v16 == v17))))
          return 0;
        *__errno_location() = v10;
        v11 = sub_19950(a0,0,&v5);
      } while (v11 == 1);
      if (v11 == 2)
        return 0;
      if ((dat_9f8be == '\x01') && (*__errno_location() == 0x11))
        return 0;
      sub_3e277(v13,a0);
      return 1;
    }
    v4 = sub_1ad5b(a0,0,&v5,v14);
  }
  return v4;
}


// Function: sub_1b508 @ 0x1b508
unsigned long sub_1b508(unsigned long a0) // early-return
{
  char v1; // al
  int4 v2; // eax
  unsigned long v3; // rax
  char v4; // stack - 0x11
  
  v4 = 0;
  if ((dat_9f878 != '\x01') && ((*dat_9fa38 == '/' || (v1 = sub_2efdc(dat_9fa38), v1)))) {
    v3 = sub_1ad5b(a0,1,&v4,0);
    return v3;
  }
  while( true ) {
    v2 = symlinkat(dat_9fa38,dat_9c268,a0);
    if (!v2) {
      sub_18913(a0,0x9fa20,0xffffffff,0,0,0x32,0,0x100);
      return 0;
    }
    v2 = sub_19950(a0,0,&v4);
    if (v2 == 2) break;
    if ((v2 <= 2) && (!v2)) {
      sub_3e944(dat_9fa38,a0);
      return 0xffffffff;
    }
  }
  return 0;
}


// Function: sub_1b63a @ 0x1b63a
unsigned long sub_1b63a(unsigned long a0,char a1) // ternary
{
  uint4 v1; // eax
  int4 v2; // eax
  char v3; // stack - 0x15
  uint4 v4; // stack - 0x14
  
  v3 = 0;
  v1 = (1 <= dat_9f95c) ? 0xffffffc0 : 0xffffffff; // branch-flip
  v4 = v1 & dat_9fa90 & 0x61ff;
  while( true ) {
    v2 = mknodat(dat_9c268,a0,v4,dat_9faa0);
    if (!v2) {
      sub_18913(a0,0x9fa20,0xffffffff,~dat_9f3b0 & v4,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v2 = sub_19950(a0,0,&v3);
    if (v2 == 2) break;
    if ((v2 <= 2) && (!v2)) {
      sub_3e362(a0);
      return 0xffffffff;
    }
  }
  return 0;
}


// Function: sub_1b745 @ 0x1b745
unsigned long sub_1b745(unsigned long a0,char a1) // ternary
{
  uint4 v1; // eax
  int4 v2; // eax
  char v3; // stack - 0x15
  uint4 v4; // stack - 0x14
  
  v3 = 0;
  v1 = (1 <= dat_9f95c) ? 0xffffffc0 : 0xffffffff; // branch-flip
  v4 = v1 & dat_9fa90 & 0x1ff;
  while( true ) {
    v2 = mkfifoat(dat_9c268,a0,v4);
    if (!v2) {
      sub_18913(a0,0x9fa20,0xffffffff,~dat_9f3b0 & v4,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v2 = sub_19950(a0,0,&v3);
    if (v2 == 2) break;
    if ((v2 <= 2) && (!v2)) {
      sub_3e339(a0);
      return 0xffffffff;
    }
  }
  return 0;
}


// Function: sub_1b84c @ 0x1b84c
unsigned long sub_1b84c(unsigned long a0,uint4 a1,unsigned long *a2) // return-dupe x6, return-dupe
{
  int4 v1; // eax
  unsigned long v2; // rax
  code *v3; // stack - 0x20
  
  switch(a1) {
    case 0:
    case 0x30:
    case 0x37:
      if (dat_9fa30) // branch-flip
        v3 = sub_1a238;
      else {
        v3 = sub_1a8d0;
      }
      break;
    default:
      if (dat_9ec84 & 0x10000) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v2 = sub_5e548(a0);
        error(0,0,gettext("%s: Unknown file type \'%c\', extracted as normal file"),v2,(uint8)a1);
      }
      v3 = sub_1a8d0;
      break;
    case 0x31:
      v3 = sub_1b161;
      break;
    case 0x32:
      v3 = sub_1b508;
      break;
    case 0x33:
      dat_9fa90 |= 0x2000;
      v3 = sub_1b63a;
      break;
    case 0x34:
      dat_9fa90 |= 0x6000;
      v3 = sub_1b63a;
      break;
    case 0x35:
    case 0x44:
      v3 = sub_1a238;
      if (dat_9fba8)
        dat_9fc31 = 1;
      break;
    case 0x36:
      v3 = sub_1b745;
      break;
    case 0x4b:
    case 0x4c:
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Unexpected long name header"));
      dat_9fdd0 = 2;
      return 0;
    case 0x4d:
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v2 = sub_5e548(dat_9fa28);
      error(0,0,gettext("%s: Cannot extract -- file is continued from another volume"),v2);
      dat_9fdd0 = 2;
      return 0;
    case 0x53:
      v3 = sub_1a8d0;
      break;
    case 0x56:
      return 0;
    
  }
  if ((dat_9f998) || (dat_9f9a0)) {
    if (v3 != sub_1a8d0)
      return 0;
  }
  else if (dat_9f8d8 != 3) { // branch-flip
    if ((dat_9f8d8 == 6) && (sub_19851(a0,0,0x9fa20))) {
      if (!(dat_9ec84 & 0x800))
        return 0;
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v2 = sub_5e84e(a0);
      error(0,0,gettext("Current %s is newer or same age"),v2);
      return 0;
    }
  }
  else {
    v1 = sub_2a36f(a0,dat_9f94c);
    if (((!v1) && (*__errno_location())) && (*__errno_location() != 2)) {
      sub_3eaa7(a0);
      return 0;
    }
  }
  *a2 = v3;
  return 1;
}


// Function: sub_1bbcb @ 0x1bbcb
void sub_1bbcb(void) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1bbcb
{
  unsigned int v1;
  char v2;
  bool v3; // al
  char v4; // al
  int4 v5;
  unsigned long v6; // rax
  code *v7; // stack - 0x28
  
  dat_9f388 = sub_1c2ee;
  sub_bba7(dat_9f460);
  v3 = ((dat_9f878 != '\x01') && (v2 = sub_2efdc(dat_9fa20), v2));
  if (v3) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v6 = sub_5e548(dat_9fa20);
    error(0,0,gettext("%s: Member name contains \'..\'"),v6);
    dat_9fdd0 = 2;
  }
  if (((!*dat_9fa28) || (v3)) || ((dat_9f8c8 && (v5 = sub_34af4("extract",dat_9fa28), !v5)))) {
    sub_28891();
    return;
  }
  if (dat_9f9ac)
    sub_2864c(0x9fa20,dat_9f460,0xffffffffffffffff);
  v1 = dat_9f600;
  if (dat_9fc31 != '\x01') {
    sub_19ce9(dat_9fa28,0);
    sub_2adf1(v1);
  }
  if ((dat_9f884) && (v2 = sub_2a571(dat_9fa28,0), v2 != '\x01')) {
    v5 = *__errno_location();
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v6 = sub_5e548(dat_9fa28);
    error(0,v5,gettext("%s: Was unable to backup this file"),v6);
    dat_9fdd0 = 2;
    sub_28891();
    return;
  }
  if (sub_301de(0x9fa20)) // branch-flip
    v2 = 'S';
  else {
    v2 = *(char *)(dat_9f460 + 0x9c);
  }
  v4 = sub_1b84c(dat_9fa28,(int4)v2,&v7);
  if (v4) { // branch-flip
    v5 = (*v7)(dat_9fa28,(int4)v2);
    if (!v5)
      return;
  }
  else {
    sub_28891();
  }
  if (!dat_9f884)
    return;
  sub_2a874();
  return;
}


// Function: sub_1be65 @ 0x1be65
void sub_1be65(void)
{
  unsigned long v1;
  unsigned long *v10; // stack - 0x290
  char *v11; // stack - 0x288
  int8 v12; // stack - 0x260
  unsigned long v13; // stack - 0x1a8
  unsigned long v14; // stack - 0x1a0
  unsigned long v15; // stack - 0x198
  unsigned long v16; // stack - 0x190
  unsigned long v17; // stack - 0x188
  unsigned int v18; // stack - 0x168
  unsigned int v19; // stack - 0x164
  unsigned long v2;
  unsigned int v20; // stack - 0x160
  unsigned long v21; // stack - 0xf0
  unsigned long v22; // stack - 0xe8
  unsigned long v23; // stack - 0xe0
  unsigned long v24; // stack - 0xd8
  unsigned long v25; // stack - 0x80
  unsigned long v26; // stack - 0x78
  unsigned long *v3;
  int4 v4; // eax
  char *v5; // rax
  int8 v6; // stack - 0x268
  char v7 [48];
  char v8 [16];
  unsigned long *v9; // stack - 0x298
  
  v9 = dat_9f3c0;
  while (v9) {
    v11 = NULL;
    sub_2adf1(*(unsigned int *)&v9[0xb]);
    for (v10 = (unsigned long *)v9[0xc]; v10; v10 = (unsigned long *)*v10) {
      v5 = (char *)&v10[1];
      v4 = fstatat(dat_9c268,v5,&v6,0x100);
      if (((!v4) && (v6 == v9[1])) && (v12 == v9[2])) {
        v8 = sub_5fdb8(&v6);
        v1 = v9[4];
        v2 = v9[3];
        if (!sub_5ff90(SUB168(v8,0),SUB168(v8,8),v2,v1)) {
          v4 = unlinkat(dat_9c268,v5,0);
          if (v4) // branch-flip
            sub_3eaa7(v5);
          else if ((!v11) || (v4 = linkat(dat_9c268,v11,dat_9c268,v5,0), v4)) {
            if (*(char *)&v9[5] != '\x01') { // branch-flip
              v4 = linkat(dat_9c268,&v9[0x14],dat_9c268,v5,0);
              if (v4)
                sub_3e277(&v9[0x14],v5);
            }
            else {
              v4 = symlinkat(&v9[0x14],dat_9c268,v5);
              if (v4) // branch-flip
                sub_3e944(&v9[0x14],v5);
              else {
                v18 = *(unsigned int *)((int8)v9 + 0x2c);
                v19 = *(unsigned int *)&v9[6];
                v20 = *(unsigned int *)((int8)v9 + 0x34);
                v22 = v9[8];
                v21 = v9[7];
                v24 = v9[10];
                v23 = v9[9];
                v13 = v9[0xd];
                v14 = v9[0xe];
                v15 = v9[0xf];
                v16 = v9[0x10];
                v17 = v9[0x11];
                v26 = v9[0x13];
                v25 = v9[0x12];
                sub_18913(v5,v7,0xffffffff,0,0,0x32,0,0x100);
                v11 = v5;
              }
            }
          }
        }
      }
    }
    v10 = (unsigned long *)v9[0xc];
    while (v10) {
      v3 = (unsigned long *)*v10;
      free(v10);
      v10 = v3;
    }
    sub_1d55d(v9[0x13],v9[0x12]);
    free((void *)v9[0xd]);
    v3 = (unsigned long *)*v9;
    free(v9);
    v9 = v3;
  }
  dat_9f3c0 = NULL;
  return;
}


// Function: sub_1c2ee @ 0x1c2ee
void sub_1c2ee(void)
{
  sub_19ce9(0x81623,0);
  sub_1be65();
  sub_19ce9(0x81623,1);
  return;
}


// Function: sub_1c326 @ 0x1c326
unsigned long sub_1c326(unsigned long a0,unsigned long a1) // return-dupe
{
  int4 v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  int8 v4;
  int4 v5; // stack - 0x24
  
  v1 = renameat(dat_9c268,a0,dat_9c268,a1);
  if (!v1) {
    sub_194f9(a0,a1);
    return 1;
  }
  v5 = *__errno_location();
  if ((v5 == 2) && (!sub_195aa(a1,&v4))) {
    v1 = renameat(dat_9c268,a0,dat_9c268,a1);
    if (!v1)
      return 1;
    v5 = *__errno_location();
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = sub_5e821(1,a1);
  v3 = sub_5e821(0,a0);
  error(0,v5,gettext("Cannot rename %s to %s"),v3,v2);
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_1c47d @ 0x1c47d
void sub_1c47d(int8 *a0,void *a1,uint8 a2)
{
  int8 v1;
  
  v1 = *a0;
  if ((uint8)(*(int8 *)(v1 + 0x20) - *(int8 *)(v1 + 0x18)) < a2)
    _obstack_newchunk(v1,a2);
  memcpy(*(void **)(v1 + 0x18),a1,a2);
  *(uint8 *)(v1 + 0x18) = *(int8 *)(v1 + 0x18) + a2;
  a0[1] = a0[1] + a2;
  return;
}


// Function: sub_1c52b @ 0x1c52b
void sub_1c52b(int8 *a0,char a1)
{
  int8 v1;
  char *v2;
  
  v1 = *a0;
  if (*(int8 *)(v1 + 0x20) == *(int8 *)(v1 + 0x18))
    _obstack_newchunk(v1,1);
  v2 = *(char **)(v1 + 0x18);
  *(char **)(v1 + 0x18) = &v2[1];
  *v2 = a1;
  a0[1] = a0[1] + 1;
  return;
}


// Function: sub_1c5b0 @ 0x1c5b0
void sub_1c5b0(int8 *a0,uint8 a1)
{
  int8 v1;
  
  v1 = *a0;
  if ((uint8)(*(int8 *)(v1 + 0x20) - *(int8 *)(v1 + 0x18)) < a1)
    _obstack_newchunk(v1,a1);
  *(uint8 *)(v1 + 0x18) = *(int8 *)(v1 + 0x18) + a1;
  a0[1] = a0[1] + a1;
  return;
}


// Function: sub_1c63f @ 0x1c63f
unsigned long sub_1c63f(char *a0)
{
  char *v1;
  unsigned long *v2; // stack - 0x10
  
  v2 = dat_9f3d8;
  while( true ) {
    if (!v2)
      return 0;
    v1 = (char *)v2[1];
    if (!fnmatch(v1,a0,0)) break;
    v2 = (unsigned long *)*v2;
  }
  return 1;
}


// Function: sub_1c69c @ 0x1c69c
unsigned long sub_1c69c(char *a0)
{
  char *v1;
  unsigned long *v2; // stack - 0x10
  
  v2 = dat_9f3e8;
  while( true ) {
    if (!v2)
      return 0;
    v1 = (char *)v2[1];
    if (!strcmp(v1,a0)) break;
    v2 = (unsigned long *)*v2;
  }
  return 1;
}


// Function: sub_1c6f4 @ 0x1c6f4
void sub_1c6f4(unsigned long *a0,unsigned long a1,int8 a2)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  v1 = (unsigned long *)sub_60b5b(0x18);
  v1[1] = sub_60cc6(a1);
  if (a2) // branch-flip
    v2 = sub_60cc6(a2);
  else {
    v2 = 0;
  }
  v1[2] = v2;
  *v1 = *a0;
  *a0 = v1;
  return;
}


// Function: sub_1c772 @ 0x1c772
void sub_1c772(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  if (!a0)
    return;
  v2 = (unsigned long *)*a0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[1]);
    free((void *)v2[2]);
    free(v2);
    v2 = v1;
  }
  *a0 = 0;
  return;
}


// Function: sub_1c7ea @ 0x1c7ea
void sub_1c7ea(unsigned long a0)
{
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Keyword %s is unknown or not yet implemented"),a0);
  sub_3814c(2); // no-return
}


// Function: sub_1c846 @ 0x1c846
void sub_1c846(unsigned long a0,unsigned long *a1,unsigned long a2) // return-dupe
{
  char v1; // al
  char *v2; // stack - 0x30
  char v3 [16];
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  
  v3 = sub_29ff7(a2,&v2,0);
  v5 = SUB168(v3,8);
  v4 = SUB168(v3,0);
  v1 = sub_294f0(v4,v5);
  if ((v1 == '\x01') && (!*v2)) {
    *a1 = v4;
    sub_29547(a0,a2);
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Time stamp is out of allowed range"));
  dat_9fdd0 = 2;
  return;
}


// Function: sub_1c92e @ 0x1c92e
void sub_1c92e(uint1 *a0,uint1 *a1) // return-dupe x6
{
  bool v1;
  unsigned long v2; // rax
  uint1 *v3; // stack - 0x20
  
  v1 = 1;
  if (a1 == a0) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_5e84e(a0);
    error(0,0,gettext("Malformed pax option: %s"),v2);
    sub_3814c(2); // no-return
  }
  v3 = a1;
  if (a1[-1] == 0x3a) {
    v3 = &a1[-1];
    v1 = 0;
  }
  while ((a0 < v3 && (*(uint2 *)((uint8)*v3 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000))) {
    v3 = &v3[-1];
  }
  *v3 = 0;
  do {
    v3 = &a1[1];
    if (!*v3) break;
    a1 = v3;
  } while (*(uint2 *)((uint8)*v3 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000);
  if (!strcmp((char *)a0,"delete")) {
    if (!sub_1d9d3(v3)) {
      sub_1c6f4(0x9f3d8,v3,0);
      return;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_5e84e(v3);
    error(0,0,gettext("Pattern %s cannot be used"),v2);
    sub_3814c(2); // no-return
  }
  if (!strcmp((char *)a0,"exthdr.name")) {
    sub_29547(0x9f3f8,v3);
    return;
  }
  if (!strcmp((char *)a0,"globexthdr.name")) {
    sub_29547(0x9f410,v3);
    return;
  }
  if (!strcmp((char *)a0,"exthdr.mtime")) {
    sub_1c846(0x9f400,0x9f408,v3);
    return;
  }
  if (!strcmp((char *)a0,"globexthdr.mtime")) {
    sub_1c846(0x9f418,0x9f420,v3);
    return;
  }
  if (sub_1da4b(a0)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Keyword %s cannot be overridden"),a0);
    sub_3814c(2); // no-return
  }
  if (v1) {
    sub_1c6f4(0x9f3e0,a0,v3);
    return;
  }
  sub_1c6f4(0x9f3e8,a0,v3);
  return;
}


// Function: sub_1cc78 @ 0x1cc78
void sub_1cc78(char *a0)
{
  char *v1; // rax
  char *v2; // stack - 0x18
  
  v2 = strtok(a0,",");
  while( true ) {
    if (!v2)
      return;
    v1 = strchr(v2,0x3d);
    if (!v1) break;
    sub_1c92e(v2,v1);
    v2 = strtok(NULL,",");
  }
  sub_1c7ea(v2); // no-return
}


// Function: sub_1cd02 @ 0x1cd02
char * sub_1cd02(unsigned long *a0,char *a1,unsigned long a2)
{
  char *v1;
  char *v10; // stack - 0x90
  char *v11; // stack - 0x88
  char *v12; // stack - 0x80
  void *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  char *v17; // stack - 0x58
  char v2;
  unsigned long v3;
  char *v4;
  char *v5; // rax
  char v6 [20];
  char v7 [12];
  char v8 [20];
  char v9 [4];
  
  v13 = NULL;
  v14 = NULL;
  v15 = NULL;
  v16 = NULL;
  v17 = NULL;
  v10 = NULL;
  for (v12 = a1; *v12; v12 = &v12[1]) {
    if ((*v12 != '%') || (!v12[1]))
      v10 = &v10[1];
    else {
      v12 = &v12[1];
      v2 = *v12;
      if (v2 != 'p') { // branch-flip
        if ('q' <= v2) { // branch-flip
label_1cef4:
          v10 = &v10[2];
        }
        else if (v2 != 'n') { // branch-flip
          if ('o' <= v2) goto label_1cef4;
          if (v2 != 'f') { // branch-flip
            if ('g' <= v2) goto label_1cef4;
            if (v2 != '%') { // branch-flip
              if (v2 != 'd') goto label_1cef4;
              if (a0) {
                if (!v13) {
                  v3 = *a0;
                  v13 = (void *)sub_4e10c(v3);
                }
                v14 = (char *)sub_3edd9(v13,0,dat_9f878);
                v10 = &v10[strlen(v14)];
              }
            }
            else {
              v10 = &v10[1];
            }
          }
          else if (a0) {
            v15 = (char *)sub_4df20(*a0);
            v10 = &v10[strlen(v15)];
          }
        }
        else {
          v17 = (char *)sub_54e5a(a2,v8);
          v10 = &v9[(int8)&v10[-(int8)v17]];
        }
      }
      else {
        v16 = (char *)sub_54e5a((int8)getpid(),v6);
        v10 = &v7[(int8)&v10[-(int8)v16]];
      }
    }
  }
  v5 = (char *)sub_60b5b(&v10[1]);
  v11 = v5;
  v4 = v11;
  v1 = a1;
label_1d0c7:
  while( true ) {
    while( true ) {
      v12 = v1;
      v11 = v4;
      if (!*v12) {
        free(v13);
        for (; (v5 < v11 && (v11[-1] == '/')); v11 = &v11[-1]) {
        }
        *v11 = '\0';
        return v5;
      }
      if (*v12 == '%') break;
      *v11 = *v12;
      v4 = &v11[1];
      v1 = &v12[1];
    }
    v2 = v12[1];
    if (v2 != 'p') break;
    v11 = stpcpy(v11,v16);
    v4 = v11;
    v1 = &v12[2];
  }
  if (v2 <= 'p') {
    if (v2 == 'n') {
      v11 = stpcpy(v11,v17);
      v4 = v11;
      v1 = &v12[2];
      goto label_1d0c7;
    }
    if ('o' <= v2) goto label_1d063;
    if (v2 == 'f') {
      if (v15)
        v11 = stpcpy(v11,v15);
      v4 = v11;
      v1 = &v12[2];
      goto label_1d0c7;
    }
    if (v2 <= 'f') {
      if (v2 == '%') {
        *v11 = *v12;
        v4 = &v11[1];
        v1 = &v12[2];
        goto label_1d0c7;
      }
      if (v2 == 'd') {
        if (v14)
          v11 = stpcpy(v11,v14);
        v4 = v11;
        v1 = &v12[2];
        goto label_1d0c7;
      }
    }
  }
label_1d063:
  v1 = &v12[1];
  *v11 = *v12;
  v4 = &v11[1];
  if (*v1) {
    v11[1] = *v1;
    v4 = &v11[2];
    v1 = &v12[2];
  }
  goto label_1d0c7;
}


// Function: sub_1d123 @ 0x1d123
void sub_1d123(unsigned long a0)
{
  if (!dat_9f3f8)
    sub_29547(0x9f3f8,*(unsigned long *)((int8)(int4)(uint4)dat_9f9c8 * 8 + 0x9c240));
  sub_1cd02(a0,dat_9f3f8,0);
  return;
}


// Function: sub_1d18d @ 0x1d18d
void sub_1d18d(void)
{
  char *v1;
  uint8 v2; // rax
  char *v3; // stack - 0x30
  
  if (!dat_9f410) {
    v1 = *(char **)(((int8)(int4)(uint4)dat_9f9c8 + 2) * 8 + 0x9c240);
    v3 = getenv("TMPDIR");
    if (!v3)
      v3 = "/tmp";
    v2 = strlen(v3);
    dat_9f410 = (char *)sub_60b5b(strlen(v1) + v2 + 1);
    strcpy(dat_9f410,v3);
    strcat(dat_9f410,v1);
  }
  sub_1cd02(0,dat_9f410,dat_9f3d0 + 1);
  return;
}


// Function: sub_1d280 @ 0x1d280
void sub_1d280(char a0,unsigned long a1,unsigned long a2,int8 a3) // return-dupe
{
  int8 v1; // rax
  void *v2; // rax
  unsigned long v3; // stack - 0x40
  uint8 v4; // stack - 0x28
  void *v5; // stack - 0x20
  uint8 v6; // stack - 0x18
  
  v4 = *(uint8 *)(a3 + 8);
  v3 = a2;
  if (a0 != 'g') { // branch-flip
    if ((a0 == 'x') && (dat_9f400))
      v3 = dat_9f408;
  }
  else if (dat_9f418)
    v3 = dat_9f420;
  v1 = sub_12e0f(a1,v4,v3);
  *(char *)(v1 + 0x9c) = a0;
  sub_1418c(v1);
  v5 = *(void **)(a3 + 0x10);
  do {
    v2 = (void *)sub_bb4b();
    v6 = 0x200;
    if (v4 < 0x200)
      v6 = v4;
    memcpy(v2,v5,v6);
    if (v6 <= 0x1ff)
      memset((void *)((int8)v2 + v6),0,0x200 - v6);
    v5 = (void *)((int8)v5 + v6);
    v4 -= v6;
    sub_bba7(v2);
  } while (v4);
  sub_1e785(a3);
  if (a0 != 'g')
    return;
  dat_9f3d0 += 1;
  return;
}


// Function: sub_1d3e6 @ 0x1d3e6
void sub_1d3e6(int8 *a0) // return-dupe
{
  void *v1; // rax
  unsigned long *v2; // stack - 0x18
  
  if (dat_9f3e0) {
    sub_1e173(a0);
    for (v2 = dat_9f3e0; v2; v2 = (unsigned long *)*v2) {
      sub_1ecd7(v2[2],v2[1],a0);
    }
  }
  if (!*a0)
    return;
  sub_1e672(a0);
  v1 = (void *)sub_1d18d();
  sub_1d280(0x67,v1,dat_9f9e0,a0);
  free(v1);
  return;
}


// Function: sub_1d498 @ 0x1d498
void sub_1d498(void)
{
  if (dat_9f3e0) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("can\'t update global extended header record"));
    sub_3814c(2); // no-return
  }
  return;
}


// Function: sub_1d4f4 @ 0x1d4f4
void sub_1d4f4(int8 a0)
{
  *(unsigned long *)(a0 + 0x160) = 0;
  *(unsigned long *)(a0 + 0x158) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  *(unsigned long *)(a0 + 0x40) = 0;
  *(unsigned long *)(a0 + 0x48) = 0;
  *(unsigned long *)(a0 + 0x50) = 0;
  *(unsigned long *)(a0 + 0x30) = 0;
  return;
}


// Function: sub_1d55d @ 0x1d55d
void sub_1d55d(void *a0,uint8 a1)
{
  uint8 v1; // stack - 0x10
  
  for (v1 = 0; v1 < a1; v1 = v1 + 1) {
    free(*(void **)((int8)a0 + v1 * 0x18));
    free(*(void **)((int8)a0 + v1 * 0x18 + 8));
  }
  free(a0);
  return;
}


// Function: sub_1d5e6 @ 0x1d5e6
void sub_1d5e6(int8 *a0,int8 *a1,unsigned long a2,unsigned long a3,int8 a4)
{
  int8 v1;
  int8 v2;
  int8 v3;
  
  v1 = *a1;
  *a1 = v1 + 1;
  v2 = *a1;
  v3 = *a0;
  *a0 = sub_60b8d(v3,v2 * 0x18);
  v2 = *a0;
  *(unsigned long *)(v2 + v1 * 0x18) = sub_60cc6(a2);
  v2 = *a0;
  *(unsigned long *)(v2 + v1 * 0x18 + 8) = sub_60c8e(a3,a4 + 1);
  *(int8 *)(*a0 + v1 * 0x18 + 0x10) = a4;
  return;
}


// Function: sub_1d6da @ 0x1d6da
void sub_1d6da(char *a0)
{
  char *v1; // stack - 0x18
  char *v2; // stack - 0x10
  
  v1 = a0;
  v2 = a0;
  do {
    if (*v1 != '%') { // branch-flip
label_1d763:
      *v2 = *v1;
      if (!*v1)
        return;
      v1 = &v1[1];
    }
    else if ((v1[1] != '3') || (v1[2] != 'D')) {
      if ((v1[1] != '2') || (v1[2] != '5')) goto label_1d763;
      *v2 = '%';
      v1 = &v1[3];
    }
    else {
      *v2 = '=';
      v1 = &v1[3];
    }
    v2 = &v2[1];
  } while( true );
}


// Function: sub_1d78e @ 0x1d78e
void sub_1d78e(int8 a0,char *a1,unsigned long a2,unsigned long a3)
{
  uint8 v1; // rax
  char *v2; // rax
  
  v1 = strlen(a1);
  v2 = (char *)sub_60b5b(strlen("SCHILY.xattr.") + v1 + 1);
  stpcpy(stpcpy(v2,"SCHILY.xattr."),a1);
  sub_1d5e6(a0 + 0x160,a0 + 0x158,v2,a2,a3);
  free(v2);
  return;
}


// Function: sub_1d854 @ 0x1d854
void sub_1d854(int8 a0,unsigned long *a1,unsigned long *a2)
{
  uint8 v1; // stack - 0x28
  
  v1 = 0;
  *a1 = 0;
  *a2 = 0;
  for (; v1 < *(uint8 *)(a0 + 0x158); v1 = v1 + 1) {
    sub_1d5e6(a1,a2,*(unsigned long *)(v1 * 0x18 + *(int8 *)(a0 + 0x160)),*(unsigned long *)(v1 * 0x18 + *(int8 *)(a0 + 0x160) + 8),*(unsigned long *)(v1 * 0x18 + *(int8 *)(a0 + 0x160) + 0x10));
  }
  return;
}


// Function: sub_1d940 @ 0x1d940
int8 * sub_1d940(char *a0)
{
  char *v1;
  uint8 v2; // rax
  int8 *v3; // stack - 0x10
  
  v3 = (int8 *)0x9a260;
  do {
    if (!*v3)
      return NULL;
    if (*(char *)((int8)v3 + 0x1c)) { // branch-flip
      v2 = strlen((char *)*v3);
      v1 = (char *)*v3;
      if (!strncmp(v1,a0,v2))
        return v3;
    }
    else {
      v1 = (char *)*v3;
      if (!strcmp(v1,a0))
        return v3;
    }
    v3 = &v3[4];
  } while( true );
}


// Function: sub_1d9d3 @ 0x1d9d3
unsigned long sub_1d9d3(char *a0)
{
  char *v1;
  int8 *v2; // stack - 0x10
  
  v2 = (int8 *)0x9a260;
  while( true ) {
    if (!*v2)
      return 0;
    if (((*(char *)((int8)v2 + 0x1c) != '\x01') && (*(uint4 *)&v2[3] & 1)) && (v1 = (char *)*v2, !fnmatch(a0,v1,0))) break;
    v2 = &v2[4];
  }
  return 1;
}


// Function: sub_1da4b @ 0x1da4b
unsigned long sub_1da4b(char *a0)
{
  char *v1;
  int8 *v2; // stack - 0x10
  
  v2 = (int8 *)0x9a260;
  while( true ) {
    if (!*v2)
      return 0;
    if (((*(char *)((int8)v2 + 0x1c) != '\x01') && (*(uint4 *)&v2[3] & 1)) && (v1 = (char *)*v2, !strcmp(v1,a0))) break;
    v2 = &v2[4];
  }
  return 1;
}


// Function: sub_1dabe @ 0x1dabe
unsigned long sub_1dabe(int8 a0,unsigned long *a1,code *a2,unsigned long a3) // return-dupe x5, return-dupe
{
  int4 v1;
  char *v2; // stack - 0x48
  char *v3; // stack - 0x40
  char *v4; // stack - 0x38
  uint8 v5; // stack - 0x30
  uint8 v6; // stack - 0x28
  char *v7; // stack - 0x20
  char *v8; // stack - 0x18
  
  v4 = (char *)*a1;
  v5 = (*(int8 *)(a0 + 8) + *(int8 *)(a0 + 0x10)) - (int8)v4;
  for (v3 = v4; (*v3 == ' ' || (*v3 == '\t')); v3 = &v3[1]) {
  }
  if (10 <= (uint4)((int4)*v3 - 0x30U)) {
    if (!*v3)
      return 0;
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Malformed extended header: missing length"));
    dat_9fdd0 = 2;
    return 0;
  }
  v6 = strtoumax(v3,&v2,10);
  if (v5 < v6) {
    v1 = (int4)v3;
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Extended header length %*s is out of range"),(uint8)(uint4)((int4)v2 - v1),v3);
    dat_9fdd0 = 2;
    return 0;
  }
  v7 = &v4[v6];
  for (v3 = v2; (*v3 == ' ' || (*v3 == '\t')); v3 = &v3[1]) {
  }
  if (v3 != v2) {
    v8 = v3;
    v3 = strchr(v3,0x3d);
    if ((v3) && (v3 < v7)) {
      if (v7[-1] == '\n') {
        v7[-1] = '\0';
        *v3 = v7[-1];
        (*a2)(a3,v8,&v3[1],&v7[0xfffffffffffffffe - (int8)v3]);
        *v3 = '=';
        v7[-1] = '\n';
        *a1 = v7;
        return 1;
      }
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Malformed extended header: missing newline"));
      dat_9fdd0 = 2;
      return 0;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Malformed extended header: missing equal sign"));
    dat_9fdd0 = 2;
    return 0;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Malformed extended header: missing blank after length"));
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_1de07 @ 0x1de07
void sub_1de07(unsigned long *a0,unsigned long a1)
{
  code *v1;
  unsigned long *v2; // rax
  uint8 v3; // rax
  unsigned long *v4; // stack - 0x30
  
  for (v4 = a0; v4; v4 = (unsigned long *)*v4) {
    v2 = (unsigned long *)sub_1d940(v4[1]);
    if (v2) {
      v1 = (code *)v2[2];
      v3 = strlen((char *)v4[2]);
      (*v1)(a1,*v2,v4[2],v3);
    }
  }
  return;
}


// Function: sub_1de86 @ 0x1de86
void sub_1de86(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  int8 v1; // rax
  
  if (!sub_1c63f(a1)) {
    if (!sub_1c69c(a1)) {
      v1 = sub_1d940(a1);
      if (v1) // branch-flip
        (**(code **)(v1 + 0x10))(a0,a1,a2,a3);
      else if (dat_9ec84 & 0x20000) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Ignoring unknown extended header keyword \'%s\'"),a1);
      }
    }
  }
  return;
}


// Function: sub_1df5f @ 0x1df5f
void sub_1df5f(int8 a0) // return-dupe
{
  int8 v1; // stack - 0x18
  
  sub_1de07(dat_9f3e0,a0);
  sub_1de07(dat_9f3f0,a0);
  if (*(int8 *)(a0 + 0x170)) {
    v1 = *(int8 *)(a0 + 0x178) + 0x200;
    do {
    } while (sub_1dabe(a0 + 0x168,&v1,sub_1de86,a0));
  }
  sub_1de07(dat_9f3e8,a0);
  *(unsigned long *)(a0 + 0x118) = *(unsigned long *)(a0 + 0x88);
  if (!*(char *)(a0 + 0x150))
    return;
  *(unsigned long *)(a0 + 0x88) = *(unsigned long *)(a0 + 0x148);
  return;
}


// Function: sub_1e063 @ 0x1e063
void sub_1e063(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3) // return-dupe
{
  int8 v1; // rax
  
  v1 = sub_1d940(a1);
  if ((v1) && (*(uint4 *)(v1 + 0x18) & 2)) {
    (**(code **)(v1 + 0x10))(a0,a1,a2,a3);
    return;
  }
  sub_1c6f4(a0,a1,a2);
  return;
}


// Function: sub_1e0e7 @ 0x1e0e7
void sub_1e0e7(int8 a0) // return-dupe
{
  int8 v1; // stack - 0x18
  
  if (!*(int8 *)(a0 + 8))
    return;
  v1 = *(int8 *)(a0 + 0x10) + 0x200;
  sub_1c772(0x9f3f0);
  do {
  } while (sub_1dabe(a0,&v1,sub_1e063,0x9f3f0));
  return;
}


// Function: sub_1e173 @ 0x1e173
void sub_1e173(int8 *a0) // return-dupe
{
  if (*a0)
    return;
  *a0 = sub_60b5b(0x58);
  _obstack_begin(*a0,0,0,sub_60b5b,dat_9bfc0);
  return;
}


// Function: sub_1e1d3 @ 0x1e1d3
void sub_1e1d3(unsigned long a0,int8 a1,unsigned long a2)
{
  int8 v1; // rax
  
  if ((((!*(int8 *)(a1 + 0x178)) && (v1 = sub_1d940(a0), v1)) && (*(int8 *)(v1 + 8))) && (!sub_1c63f(a0))) {
    sub_1e173(a1 + 0x168);
    if (sub_1c69c(a0) != '\x01')
      (**(code **)(v1 + 8))(a1,a0,a1 + 0x168,a2);
  }
  return;
}


// Function: sub_1e28e @ 0x1e28e
void sub_1e28e(int8 a0,void *a1,uint8 a2)
{
  uint8 v1;
  uint8 v2; // stack - 0x30
  void *v3; // stack - 0x28
  int8 v4; // stack - 0x18
  uint8 v5; // stack - 0x10
  
  v4 = 0;
  v2 = a2;
  if ((int8)a2 <= -1)
    v2 = 0;
  if (0xfffffffffffffdff <= v2)
    sub_1783c(); // no-return
  v1 = v2 + 0x200;
  *(uint8 *)(a0 + 8) = v1;
  *(unsigned long *)(a0 + 0x10) = sub_60b5b(v2 + 0x201);
  *(char *)(v1 + *(int8 *)(a0 + 0x10)) = 0;
  v2 = v1;
  v3 = a1;
  while( true ) {
    v5 = v2;
    if (0x201 <= v2)
      v5 = 0x200;
    if (!v3) break;
    memcpy((void *)(*(int8 *)(a0 + 0x10) + v4),v3,v5);
    sub_bba7(v3);
    v3 = (void *)sub_bb4b();
    v4 += v5;
    v2 -= v5;
    if ((int8)v2 <= 0)
      return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Unexpected EOF in archive"));
  sub_177f4(); // no-return
}


// Function: sub_1e3ca @ 0x1e3ca
char * sub_1e3ca(char *a0)
{
  char v1;
  char *v2; // stack - 0x20
  int8 v3; // stack - 0x10
  
  if (dat_9f428) // branch-flip
    *dat_9f428 = 0;
  else {
    dat_9f430 = 0x100;
    dat_9f428 = (char *)sub_60b5b(0x100);
  }
  v3 = 0;
  for (v2 = a0; *v2; v2 = &v2[1]) {
    v1 = *v2;
    if (dat_9f430 <= v3 + 2U)
      dat_9f428 = (char *)sub_60bd8(dat_9f428,0x9f430);
    if (v1 != '%') { // branch-flip
      if (v1 != '=') // branch-flip
        dat_9f428[v3] = v1;
      else {
        strcpy(&dat_9f428[v3],"%3D");
        v3 += 2;
      }
    }
    else {
      strcpy(&dat_9f428[v3],"%25");
      v3 += 2;
    }
    v3 += 1;
  }
  dat_9f428[v3] = 0;
  return dat_9f428;
}


// Function: sub_1e4ff @ 0x1e4ff
void sub_1e4ff(unsigned long a0,unsigned long a1,unsigned long a2,int8 a3)
{
  char *v1; // rax
  uint8 v2; // rax
  int8 v3; // rax
  char *v4; // rax
  char v5 [20];
  char v6 [4];
  bool v7; // zf
  char *v8; // stack - 0x50
  
  v8 = NULL;
  v1 = (char *)sub_1e3ca(a1);
  v2 = strlen(v1);
  do {
    v3 = sub_54e5a(&v8[a3 + v2 + 3],v5);
    v4 = &v6[-v3];
    v7 = v4 != v8;
    v8 = v4;
  } while (v7);
  sub_1c47d(a0,v3,v4);
  sub_1c52b(a0,0x20);
  sub_1c47d(a0,v1,v2);
  sub_1c52b(a0,0x3d);
  sub_1c47d(a0,a2,a3);
  sub_1c52b(a0,10);
  return;
}


// Function: sub_1e634 @ 0x1e634
void sub_1e634(unsigned long a0,unsigned long a1,char *a2)
{
  sub_1e4ff(a0,a1,a2,strlen(a2));
  return;
}


// Function: sub_1e672 @ 0x1e672
void sub_1e672(int8 *a0)
{
  int8 v1;
  int8 v2;
  unsigned long *v3; // stack - 0x20
  
  for (v3 = dat_9f3e8; v3; v3 = (unsigned long *)*v3) {
    sub_1ecd7(v3[2],v3[1],a0);
  }
  v1 = *a0;
  v2 = *(int8 *)(v1 + 0x10);
  if (v2 == *(int8 *)(v1 + 0x18))
    *(uint1 *)(v1 + 0x50) = *(uint1 *)(v1 + 0x50) | 2;
  *(uint8 *)(v1 + 0x18) = ~*(uint8 *)(v1 + 0x30) & *(int8 *)(v1 + 0x18) + *(int8 *)(v1 + 0x30);
  if ((uint8)(*(int8 *)(v1 + 0x20) - *(int8 *)(v1 + 8)) < (uint8)(*(int8 *)(v1 + 0x18) - *(int8 *)(v1 + 8)))
    *(unsigned long *)(v1 + 0x18) = *(unsigned long *)(v1 + 0x20);
  *(unsigned long *)(v1 + 0x10) = *(unsigned long *)(v1 + 0x18);
  a0[2] = v2;
  return;
}


// Function: sub_1e785 @ 0x1e785
void sub_1e785(int8 *a0)
{
  if (*a0) { // branch-flip
    _obstack_free(*a0,0);
    free((void *)*a0);
    *a0 = 0;
  }
  else {
    free((void *)a0[2]);
  }
  a0[2] = 0;
  a0[1] = 0;
  return;
}


// Function: sub_1e848 @ 0x1e848
void sub_1e848(int8 a0)
{
  *(unsigned long *)(a0 + 0x18) = 0;
  return;
}


// Function: sub_1e863 @ 0x1e863
void sub_1e863(int8 a0,char *a1) // return-dupe
{
  int8 v1;
  
  if (*(int8 *)(a0 + 0x10))
    return;
  sub_1e173(a0);
  v1 = *(int8 *)(a0 + 0x18);
  *(uint8 *)(a0 + 0x18) = v1 + strlen(a1);
  sub_1c47d(a0,a1,strlen(a1));
  return;
}


// Function: sub_1e8dc @ 0x1e8dc
unsigned long sub_1e8dc(int8 *a0,char *a1) // return-dupe
{
  int8 v1;
  uint8 v2; // rax
  char *v3; // rax
  char *v4; // rax
  char *v5; // rax
  char v6 [20];
  char v7 [4];
  bool v8; // zf
  char *v9; // stack - 0x78
  
  v9 = NULL;
  if (a0[2])
    return 0;
  sub_1e173(a0);
  v2 = strlen(a1);
  v1 = a0[3];
  do {
    v3 = (char *)sub_54e5a(&v9[v1 + v2 + 3],v6);
    v4 = &v7[-(int8)v3];
    v8 = v4 != v9;
    v9 = v4;
  } while (v8);
  v4 = &v4[strlen(a1) + 2];
  sub_1c5b0(a0,v4);
  sub_1c52b(a0,10);
  v5 = (char *)((-(int8)v4 - a0[3]) + -1 + *(int8 *)(*a0 + 0x18));
  memmove(&v5[(int8)v4],v5,a0[3]);
  v3 = stpcpy(v5,v3);
  *v3 = ' ';
  *stpcpy(&v3[1],a1) = '=';
  return 1;
}


// Function: sub_1ec02 @ 0x1ec02
void sub_1ec02(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [32];
  char v4 [24];
  
  v1 = sub_54bde(a2,v3);
  v2 = sub_54e5a(a3,v4);
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Extended header %s=%s is out of range %s..%s"),a0,a1,v1,v2);
  dat_9fdd0 = 2;
  return;
}


// Function: sub_1ecd7 @ 0x1ecd7
void sub_1ecd7(unsigned long a0,unsigned long a1,unsigned long a2)
{
  void *v1; // stack - 0x18
  
  if (sub_3c8a0(1,a0,&v1) != '\x01')
    v1 = (void *)sub_60cc6(a0);
  sub_1e634(a2,a1,v1);
  free(v1);
  return;
}


// Function: sub_1ed64 @ 0x1ed64
void sub_1ed64(int8 *a0,unsigned long a1) // return-dupe
{
  if (*a0) {
    free((void *)*a0);
    *a0 = 0;
  }
  if (sub_3c8a0(0,a1,a0) == '\x01')
    return;
  sub_29547(a0,a1);
  return;
}


// Function: sub_1edd0 @ 0x1edd0
void sub_1edd0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [40];
  
  sub_1e634(a3,a2,sub_29f2e(a0,a1,v1));
  return;
}


// Function: sub_1ee4e @ 0x1ee4e
unsigned long sub_1ee4e(unsigned long *a0,char *a1,unsigned long a2) // return-dupe
{
  char v1; // al
  char *v2; // stack - 0x30
  char v3 [16];
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  
  v3 = sub_29ff7(a1,&v2,1);
  v5 = SUB168(v3,8);
  v4 = SUB168(v3,0);
  v1 = sub_294f0(v4,v5);
  if (v1 == '\x01') {
    *a0 = v4;
    a0[1] = v5;
    return 1;
  }
  if ((a1 < v2) && (!*v2)) {
    sub_1ec02(a2,a1,0x8000000000000000,0x7fffffffffffffff);
    return 0;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Malformed extended header: invalid %s=%s"),a2,a1);
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_1ef79 @ 0x1ef79
void sub_1ef79(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  char v1 [24];
  
  sub_1e634(a4,a1,sub_29c96(a0,a2,a3,v1));
  return;
}


// Function: sub_1efed @ 0x1efed
void sub_1efed(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1ef79(a0,a1,0,0xffffffffffffffff,a2);
  return;
}


// Function: sub_1f02b @ 0x1f02b
unsigned long sub_1f02b(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4) // return-dupe x2
{
  char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  v2 = sub_29cfc(a1,&v1,a2,a3);
  if ((*__errno_location() != 0x16) && (!*v1)) {
    if (*__errno_location() == 0x22) {
      sub_1ec02(a4,a1,a2,a3);
      return 0;
    }
    *a0 = v2;
    return 1;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Malformed extended header: invalid %s=%s"),a4,a1);
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_1f1bd @ 0x1f1bd
void sub_1f1bd(void)
{
  return;
}


// Function: sub_1f1d8 @ 0x1f1d8
void sub_1f1d8(void)
{
  return;
}


// Function: sub_1f1f3 @ 0x1f1f3
void sub_1f1f3(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1edd0(*(unsigned long *)(a0 + 0xe8),*(unsigned long *)(a0 + 0xf0),a1,a2);
  return;
}


// Function: sub_1f237 @ 0x1f237
void sub_1f237(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ee4e(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xe8) = v1;
  *(unsigned long *)(a0 + 0xf0) = v2;
  return;
}


// Function: sub_1f2ae @ 0x1f2ae
void sub_1f2ae(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ef79(*(unsigned int *)(a0 + 0x78),a1,0,0xffffffff,a2);
  return;
}


// Function: sub_1f2f6 @ 0x1f2f6
void sub_1f2f6(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1f02b(v1,a2,0,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x78) = v1[0];
  return;
}


// Function: sub_1f36d @ 0x1f36d
void sub_1f36d(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ecd7(*(unsigned long *)(a0 + 0x28),a1,a2);
  return;
}


// Function: sub_1f3a7 @ 0x1f3a7
void sub_1f3a7(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(a0 + 0x28,a2);
  return;
}


// Function: sub_1f3dd @ 0x1f3dd
void sub_1f3dd(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ecd7(*(unsigned long *)(a0 + 0x18),a1,a2);
  return;
}


// Function: sub_1f417 @ 0x1f417
void sub_1f417(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(a0 + 0x18,a2);
  return;
}


// Function: sub_1f44d @ 0x1f44d
void sub_1f44d(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1edd0(*(unsigned long *)(a0 + 0x108),*(unsigned long *)(a0 + 0x110),a1,a2);
  return;
}


// Function: sub_1f491 @ 0x1f491
void sub_1f491(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ee4e(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0x108) = v1;
  *(unsigned long *)(a0 + 0x110) = v2;
  return;
}


// Function: sub_1f508 @ 0x1f508
void sub_1f508(int8 a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1;
  unsigned long v2;
  
  if (a3) { // branch-flip
    v2 = a3[1];
    v1 = *a3;
  }
  else {
    v2 = *(unsigned long *)(a0 + 0x100);
    v1 = *(unsigned long *)(a0 + 0xf8);
  }
  sub_1edd0(v1,v2,a1,a2,v1,v2);
  return;
}


// Function: sub_1f574 @ 0x1f574
void sub_1f574(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ee4e(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xf8) = v1;
  *(unsigned long *)(a0 + 0x100) = v2;
  return;
}


// Function: sub_1f5eb @ 0x1f5eb
void sub_1f5eb(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ecd7(*(unsigned long *)(a0 + 8),a1,a2);
  return;
}


// Function: sub_1f625 @ 0x1f625
void sub_1f625(int8 a0,unsigned long a1)
{
  unsigned long v1;
  
  sub_1ed64(a0,a1);
  sub_1ed64(a0 + 8,a1);
  v1 = *(unsigned long *)(a0 + 8);
  *(char *)(a0 + 0x10) = sub_4e2e5(v1);
  return;
}


// Function: sub_1f67d @ 0x1f67d
void sub_1f67d(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if (*(char *)(a0 + 0x151) == '\x01')
    return;
  sub_1f625(a0,a2);
  return;
}


// Function: sub_1f6c1 @ 0x1f6c1
void sub_1f6c1(int8 a0,unsigned long a1,unsigned long a2)
{
  *(char *)(a0 + 0x151) = 1;
  sub_1f625(a0,a2);
  return;
}


// Function: sub_1f6fe @ 0x1f6fe
void sub_1f6fe(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1efed(*(unsigned long *)(a0 + 0x88),a1,a2);
  return;
}


// Function: sub_1f73e @ 0x1f73e
void sub_1f73e(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x88) = v1;
  return;
}


// Function: sub_1f7b7 @ 0x1f7b7
void sub_1f7b7(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ef79(*(unsigned int *)(a0 + 0x74),a1,0,0xffffffff,a2);
  return;
}


// Function: sub_1f7ff @ 0x1f7ff
void sub_1f7ff(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1f02b(v1,a2,0,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x74) = v1[0];
  return;
}


// Function: sub_1f876 @ 0x1f876
void sub_1f876(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ecd7(*(unsigned long *)(a0 + 0x20),a1,a2);
  return;
}


// Function: sub_1f8b0 @ 0x1f8b0
void sub_1f8b0(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(a0 + 0x20,a2);
  return;
}


// Function: sub_1f8e6 @ 0x1f8e6
void sub_1f8e6(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_1f6fe(a0,a1,a2,a3);
  return;
}


// Function: sub_1f91d @ 0x1f91d
void sub_1f91d(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(char *)(a0 + 0x150) = 1;
  *(unsigned long *)(a0 + 0x148) = v1;
  return;
}


// Function: sub_1f9a1 @ 0x1f9a1
void sub_1f9a1(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1efed(*(unsigned long *)(a0 + 0x130),a1,a2);
  return;
}


// Function: sub_1f9de @ 0x1f9de
void sub_1f9de(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0xffffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x138) = v1;
  *(unsigned long *)(a0 + 0x140) = sub_60c29(v1,0x10);
  *(unsigned long *)(a0 + 0x130) = 0;
  return;
}


// Function: sub_1fa7f @ 0x1fa7f
void sub_1fa7f(int8 a0,unsigned long a1,unsigned long a2,int8 *a3)
{
  sub_1efed(*(unsigned long *)(*a3 * 0x10 + *(int8 *)(a0 + 0x140)),a1,a2);
  return;
}


// Function: sub_1fad8 @ 0x1fad8
void sub_1fad8(int8 a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0x7fffffffffffffff,a1))
    return;
  if (*(uint8 *)(a0 + 0x130) < *(uint8 *)(a0 + 0x138)) {
    *(unsigned long *)(*(int8 *)(a0 + 0x130) * 0x10 + *(int8 *)(a0 + 0x140)) = v1;
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Malformed extended header: excess %s=%s"),"GNU.sparse.offset",a2);
  dat_9fdd0 = 2;
  return;
}


// Function: sub_1fbda @ 0x1fbda
void sub_1fbda(int8 a0,unsigned long a1,unsigned long a2,int8 *a3)
{
  sub_1efed(*(unsigned long *)(*a3 * 0x10 + *(int8 *)(a0 + 0x140) + 8),a1,a2);
  return;
}


// Function: sub_1fc34 @ 0x1fc34
void sub_1fc34(int8 a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  int8 v1;
  unsigned long v2; // stack - 0x18
  
  if (!sub_1f138(&v2,a2,0x7fffffffffffffff,a1))
    return;
  if (*(uint8 *)(a0 + 0x130) < *(uint8 *)(a0 + 0x138)) {
    v1 = *(int8 *)(a0 + 0x130);
    *(int8 *)(a0 + 0x130) = v1 + 1;
    *(unsigned long *)(v1 * 0x10 + *(int8 *)(a0 + 0x140) + 8) = v2;
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Malformed extended header: excess %s=%s"),a1,a2);
  dat_9fdd0 = 2;
  return;
}


// Function: sub_1fd46 @ 0x1fd46
void sub_1fd46(int8 a0,unsigned long a1,char *a2)
{
  unsigned long *v1;
  char v2;
  int8 v3;
  bool v4;
  char *v5; // stack - 0x48
  char *v6; // stack - 0x70
  unsigned long v7; // stack - 0x40
  unsigned long v8; // stack - 0x38
  unsigned long v9; // stack - 0x30
  
  v4 = 1;
  *(unsigned long *)(a0 + 0x130) = 0;
  v6 = a2;
  while( true ) {
    if (10 <= (uint4)((int4)*v6 - 0x30U)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Malformed extended header: invalid %s=%s"),a1,v6);
      dat_9fdd0 = 2;
      return;
    }
    *__errno_location() = 0;
    v7 = strtoimax(v6,&v5,10);
    if (v4) {
      v8 = v7;
      if (*__errno_location() == 0x22) {
        sub_1ec02(a1,v6,0,0x7fffffffffffffff);
        return;
      }
    }
    else {
      v9 = v7;
      if (*__errno_location() == 0x22) {
        sub_1ec02(a1,v6,0,0x7fffffffffffffff);
        return;
      }
      if (*(uint8 *)(a0 + 0x138) <= *(uint8 *)(a0 + 0x130)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Malformed extended header: excess %s=%s"),a1,v6);
        dat_9fdd0 = 2;
        return;
      }
      v3 = *(int8 *)(a0 + 0x130);
      *(int8 *)(a0 + 0x130) = v3 + 1;
      v1 = (unsigned long *)(*(int8 *)(a0 + 0x140) + v3 * 0x10);
      *v1 = v8;
      v1[1] = v9;
    }
    v4 = !v4;
    if (!*v5) {
      if (v4)
        return;
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Malformed extended header: invalid %s: odd number of values"),a1);
      dat_9fdd0 = 2;
      return;
    }
    if (*v5 != ',') break;
    v6 = &v5[1];
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = *v5;
  error(0,0,gettext("Malformed extended header: invalid %s: unexpected delimiter %c"),a1,(uint8)(uint4)(int4)v2);
  dat_9fdd0 = 2;
  return;
}


// Function: sub_2004a @ 0x2004a
void sub_2004a(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_1e4ff(a2,a1,a3,sub_20bfe(a3));
  return;
}


// Function: sub_2008c @ 0x2008c
void sub_2008c(int8 a0,unsigned long a1,void *a2,uint8 a3)
{
  *(unsigned long *)(a0 + 400) = sub_60b5b(a3);
  memcpy(*(void **)(a0 + 400),a2,a3);
  return;
}


// Function: sub_200e3 @ 0x200e3
void sub_200e3(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_1ecd7(a3,a1,a2);
  return;
}


// Function: sub_20119 @ 0x20119
void sub_20119(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(0x9efe0,a2);
  return;
}


// Function: sub_2014e @ 0x2014e
void sub_2014e(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  sub_1efed(*a3,a1,a2);
  return;
}


// Function: sub_20192 @ 0x20192
void sub_20192(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_9eff0 = v1;
  return;
}


// Function: sub_20201 @ 0x20201
void sub_20201(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  sub_1efed(*a3,a1,a2);
  return;
}


// Function: sub_20245 @ 0x20245
void sub_20245(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1f138(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_9eff8 = v1;
  return;
}


// Function: sub_202b4 @ 0x202b4
void sub_202b4(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(0x9efe8,a2);
  return;
}


// Function: sub_202e9 @ 0x202e9
void sub_202e9(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ecd7(*(unsigned long *)(a0 + 0x30),a1,a2);
  return;
}


// Function: sub_20323 @ 0x20323
void sub_20323(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1ed64(a0 + 0x30,a2);
  return;
}


// Function: sub_20359 @ 0x20359
void sub_20359(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1e4ff(a2,a1,*(unsigned long *)(a0 + 0x38),*(unsigned long *)(a0 + 0x40));
  return;
}


// Function: sub_20398 @ 0x20398
void sub_20398(int8 a0,unsigned long a1,unsigned long a2,int8 a3)
{
  *(unsigned long *)(a0 + 0x38) = sub_60c8e(a2,a3 + 1);
  *(int8 *)(a0 + 0x40) = a3;
  return;
}


// Function: sub_203e2 @ 0x203e2
void sub_203e2(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1e4ff(a2,a1,*(unsigned long *)(a0 + 0x48),*(unsigned long *)(a0 + 0x50));
  return;
}


// Function: sub_20421 @ 0x20421
void sub_20421(int8 a0,unsigned long a1,unsigned long a2,int8 a3)
{
  *(unsigned long *)(a0 + 0x48) = sub_60c8e(a2,a3 + 1);
  *(int8 *)(a0 + 0x50) = a3;
  return;
}


// Function: sub_2046b @ 0x2046b
void sub_2046b(int8 a0,unsigned long a1,unsigned long a2,int8 *a3)
{
  sub_1e4ff(a2,a1,*(unsigned long *)(*(int8 *)(a0 + 0x160) + *a3 * 0x18 + 8),*(unsigned long *)(*(int8 *)(a0 + 0x160) + *a3 * 0x18 + 0x10));
  return;
}


// Function: sub_204f5 @ 0x204f5
void sub_204f5(unsigned long a0,char *a1,void *a2,int8 a3)
{
  char v1;
  unsigned long v10; // stack - 0x30
  uint8 v11; // stack - 0x28
  void *v12; // stack - 0x20
  void *v13; // stack - 0x18
  int8 v2;
  uint8 v3; // rax
  uint8 v4; // rax
  int8 v5; // stack - 0x48
  int8 *v6; // rsp
  char *v7; // rsp
  void *v8; // stack - 0x40
  char *v9; // stack - 0x38
  
  v5 = a3;
  v8 = a2;
  v9 = a1;
  v10 = a0;
  v3 = strlen(a1);
  v11 = v3;
  v4 = ((v3 + 0x18) / 0x10) * 0x10;
  for (v6 = &v5; v6 != (int8 *)((int8)&v5 - (v4 & 0xfffffffffffff000)); v6 = (char *)((int8)v6 + -0x1000)) {
    *(unsigned long *)((int8)v6 + -8) = *(unsigned long *)((int8)v6 + -8);
  }
  v2 = -(uint8)((uint4)v4 & 0xfff);
  v7 = (char *)((int8)v6 + v2);
  if (v4 & 0xfff)
    *(unsigned long *)((int8)v6 + ((uint8)((uint4)v4 & 0xfff) - 8) + v2) = *(unsigned long *)((int8)v6 + ((uint8)((uint4)v4 & 0xfff) - 8) + v2);
  v12 = (void *)((uint8)((int8)v6 + v2 + 0xf) & 0xfffffffffffffff0);
  *(unsigned long *)((int8)v6 + v2 + -8) = 0x205d8;
  memcpy((void *)((uint8)((int8)v6 + v2 + 0xf) & 0xfffffffffffffff0),v9,v3 + 1,*(char *)((int8)v6 + v2 + -8));
  v3 = ((v5 + 0x18U) / 0x10) * 0x10;
  for (; v7 != (char *)((int8)v6 + (v2 - (v3 & 0xfffffffffffff000))); v7 = &v7[-0x1000]) {
    *(unsigned long *)&v7[-8] = *(unsigned long *)&v7[-8];
  }
  v2 = -(uint8)((uint4)v3 & 0xfff);
  if (v3 & 0xfff)
    *(unsigned long *)&v7[((uint8)((uint4)v3 & 0xfff) - 8) + v2] = *(unsigned long *)&v7[((uint8)((uint4)v3 & 0xfff) - 8) + v2];
  v13 = (void *)((uint8)&v7[v2 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v7[v2 + -8] = 0x20680;
  memcpy((void *)((uint8)&v7[v2 + 0xf] & 0xfffffffffffffff0),v8,v5 + 1,v7[v2 + -8]);
  *(unsigned long *)&v7[v2 + -8] = 0x2068c;
  sub_1d6da(v12);
  *(unsigned long *)&v7[v2 + -8] = 0x2069b;
  v1 = v7[v2 + -8];
  *(unsigned long *)&v7[v2 + -8] = 0x206b7;
  sub_1d78e(v10,strlen("SCHILY.xattr.",v1) + (int8)v12,v13,v5);
  return;
}


// Function: sub_206ce @ 0x206ce
void sub_206ce(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1efed(*(unsigned int *)(a0 + 0x124),a1,a2);
  return;
}


// Function: sub_2070c @ 0x2070c
void sub_2070c(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1f138(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x124) = v1[0];
  return;
}


// Function: sub_2077e @ 0x2077e
void sub_2077e(int8 a0,unsigned long a1,unsigned long a2)
{
  sub_1efed(*(unsigned int *)(a0 + 0x128),a1,a2);
  return;
}


// Function: sub_207bc @ 0x207bc
void sub_207bc(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1f138(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x128) = v1[0];
  return;
}


// Function: sub_2082e @ 0x2082e
int8 * sub_2082e(char *a0,char *a1)
{
  char v1;
  int8 v2;
  int8 *v3; // rax
  int8 v4; // stack - 0x40
  int8 v5; // stack - 0x38
  uint8 v6; // stack - 0x30
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v4 = 0;
  v5 = 0;
  v6 = 1;
  for (v8 = a0; *v8; v8 = &v8[v2]) {
    v2 = strlen(v8) + 1;
    v6 += v2;
    if (a1) { // branch-flip
      v1 = *v8;
      if (strchr(a1,(int4)v1)) goto label_208a4;
    }
    else {
label_208a4:
      v4 += 1;
    }
    v5 += 1;
  }
  v3 = (int8 *)sub_60b5b(v6 + 0x20);
  *v3 = (int8)&v3[4];
  memcpy((void *)*v3,a0,v6);
  v3[1] = v5;
  v3[2] = v4;
  v3[3] = sub_60c29(v4 + 1,8);
  v4 = 0;
  v7 = (char *)*v3;
  do {
    if (!*v7) {
      *(unsigned long *)(v4 * 8 + v3[3]) = 0;
      return v3;
    }
    if (a1) { // branch-flip
      v1 = *v7;
      if (strchr(a1,(int4)v1)) goto label_2096f;
    }
    else {
label_2096f:
      v2 = v4 + 1;
      *(char **)(v4 * 8 + v3[3]) = &v7[1];
      v4 = v2;
    }
    v7 = &v7[strlen(v7) + 1];
  } while( true );
}


// Function: sub_209d4 @ 0x209d4
void sub_209d4(unsigned long a0)
{
  sub_2082e(a0,0x81be0);
  return;
}


// Function: sub_209fc @ 0x209fc
void sub_209fc(void *a0)
{
  free(*(void **)((int8)a0 + 0x18));
  free(a0);
  return;
}


// Function: sub_20a2b @ 0x20a2b
void sub_20a2b(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1);
  return;
}


// Function: sub_20a6a @ 0x20a6a
int8 sub_20a6a(int8 a0,unsigned long a1) // early-return x2
{
  int8 *v1; // rax
  unsigned long v2; // stack - 0x28
  int8 v3; // stack - 0x20
  
  if (!a0)
    return 0;
  v2 = a1;
  v3 = a0;
  v1 = (int8 *)bsearch(&v2,*(unsigned long *)(a0 + 0x18),*(unsigned long *)(a0 + 0x10),8,sub_20a2b);
  if (v1)
    return *v1 + -1;
  return 0;
}


// Function: sub_20ad3 @ 0x20ad3
char * sub_20ad3(int8 *a0)
{
  uint8 v1;
  int8 v2;
  char *v3; // stack - 0x28
  
  v1 = a0[2];
  v3 = NULL;
  if ((int4)a0[1]) { // branch-flip
    v3 = (char *)(v1 + *(int8 *)*a0);
    if (!*v3)
      return NULL;
    v2 = a0[2];
    a0[2] = strlen(v3) + v2 + 1;
  }
  else if (v1 < *(uint8 *)(*a0 + 0x10)) {
    v3 = (char *)(*(int8 *)(v1 * 8 + *(int8 *)(*a0 + 0x18)) + -1);
    a0[2] = a0[2] + 1;
  }
  return v3;
}


// Function: sub_20b9f @ 0x20b9f
void sub_20b9f(unsigned long a0,unsigned int a1,int8 *a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_60b5b(0x18);
  *v1 = a0;
  *(unsigned int *)&v1[1] = a1;
  v1[2] = 0;
  *a2 = (int8)v1;
  sub_20ad3(v1);
  return;
}


// Function: sub_20bfe @ 0x20bfe
int8 sub_20bfe(char *a0)
{
  int8 v1; // rax
  char *v2; // stack - 0x20
  int8 v3; // stack - 0x18
  
  v3 = 0;
  for (v2 = a0; *v2; v2 = &v2[v1]) {
    v1 = strlen(v2) + 1;
    v3 += v1;
  }
  return v3 + 1;
}


// Function: sub_20c51 @ 0x20c51
void sub_20c51(int8 a0,unsigned long a1)
{
  sub_52834(*(unsigned long *)(a0 + 0x50),a1);
  return;
}


// Function: sub_20c86 @ 0x20c86
unsigned long sub_20c86(int8 a0,int8 a1)
{
  int4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x50),*(char **)(a1 + 0x50));
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_20ccc @ 0x20ccc
uint8 sub_20ccc(int8 a0,uint8 a1)
{
  return (uint8)(*(int8 *)(a0 + 0x20) + *(int8 *)(a0 + 0x18)) % a1;
}


// Function: sub_20d05 @ 0x20d05
unsigned int sub_20d05(int8 a0,int8 a1) // return-dupe
{
  if ((*(int8 *)(a0 + 0x18) == *(int8 *)(a1 + 0x18)) && (*(int8 *)(a0 + 0x20) == *(int8 *)(a1 + 0x20)))
    return 1;
  return 0;
}


// Function: sub_20d60 @ 0x20d60
unsigned long * sub_20d60(char *a0,unsigned long a1)
{
  unsigned long *v1; // rax
  uint8 v2; // stack - 0x18
  
  v2 = strlen(a0);
  v1 = (unsigned long *)sub_60b5b(0x60);
  *v1 = 0;
  v1[6] = 0;
  v1[5] = v1[6];
  v1[8] = 0;
  *(unsigned int *)((int8)v1 + 0x3c) = 0;
  if ((2 <= v2) && (a0[v2 - 1] == '/'))
    v2 -= 1;
  v1[0xb] = sub_60b5b(v2 + 1);
  memcpy((void *)v1[0xb],a0,v2);
  *(char *)(v2 + v1[0xb]) = 0;
  v1[10] = a1;
  v1[9] = 0;
  return v1;
}


// Function: sub_20e58 @ 0x20e58
void sub_20e58(void *a0)
{
  free(*(void **)((int8)a0 + 0x50));
  free(*(void **)((int8)a0 + 0x58));
  free(a0);
  return;
}


// Function: sub_20e97 @ 0x20e97
unsigned long sub_20e97(unsigned long a0)
{
  unsigned long v1; // rax
  
  v1 = sub_29aac(dat_9f600,a0);
  v1 = sub_20d60(a0,v1);
  if (dat_9f440) // branch-flip
    *dat_9f440 = v1;
  else {
    dat_9f438 = v1;
  }
  dat_9f440 = (unsigned long *)v1;
  return v1;
}


// Function: sub_20f0e @ 0x20f0e
void sub_20f0e(char *a0,char *a1)
{
  uint8 v1; // rax
  uint8 v2; // rax
  unsigned long *v3; // stack - 0x20
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  for (v3 = dat_9f438; v3; v3 = (unsigned long *)*v3) {
    sub_29b93(&v3[0xb],a0,v1,a1,v2);
  }
  return;
}


// Function: sub_20f88 @ 0x20f88
void sub_20f88(void)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  if (dat_9f448)
    sub_52d91(dat_9f448);
  if (dat_9f450)
    sub_52d91(dat_9f450);
  v2 = dat_9f438;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_20e58(v2);
    v2 = v1;
  }
  dat_9f440 = 0;
  dat_9f438 = NULL;
  return;
}


// Function: sub_21019 @ 0x21019
int8 sub_21019(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char a5,unsigned int a6,int8 a7)
{
  char v1; // al
  int8 v2; // rax
  unsigned long v3; // rax
  int8 v4; // rax
  
  v1 = (char)a6;
  v2 = sub_20e97(a0);
  *(unsigned long *)(v2 + 8) = a1;
  *(unsigned long *)(v2 + 0x10) = a2;
  *(unsigned long *)(v2 + 0x18) = a3;
  *(unsigned long *)(v2 + 0x20) = a4;
  *(unsigned int *)(v2 + 0x38) = 1;
  if (a5)
    *(uint4 *)(v2 + 0x3c) = *(uint4 *)(v2 + 0x3c) | 2;
  if (v1)
    *(uint4 *)(v2 + 0x3c) = *(uint4 *)(v2 + 0x3c) | 4;
  if (a7) { // branch-flip
    v3 = sub_209d4(a7);
    *(unsigned long *)(v2 + 0x28) = v3;
  }
  else {
    *(unsigned long *)(v2 + 0x28) = 0;
  }
  if (!dat_9f448) {
    dat_9f448 = sub_52c1c(0,0,sub_20c51,sub_20c86,0);
    if (!dat_9f448) {
      sub_1783c(); // no-return, return-dupe
    }
  }
  v4 = sub_5395d(dat_9f448,v2);
  if (!v4)
    sub_1783c();
  if (!dat_9f450) {
    dat_9f450 = sub_52c1c(0,0,sub_20ccc,sub_20d05,0);
    if (!dat_9f450) {
      sub_1783c(); // no-return, return-dupe
    }
  }
  v4 = sub_5395d(dat_9f450,v2);
  if (v4)
    return v2;
  sub_1783c();
}


// Function: sub_211c6 @ 0x211c6
unsigned long sub_211c6(unsigned long a0) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  if (!dat_9f448)
    return 0;
  v1 = sub_29aac(dat_9f600,a0);
  v1 = sub_20d60(a0,v1);
  v2 = sub_52567(dat_9f448,v1);
  sub_20e58(v1);
  return v2;
}


// Function: sub_21244 @ 0x21244
void sub_21244(int8 a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  sub_29b93(a0 + 0x58,a1,a2,a3,a4);
  return;
}


// Function: sub_2128a @ 0x2128a
unsigned long sub_2128a(unsigned long a0,unsigned long a1) // early-return
{
  int8 v1; // rax
  unsigned long v2; // rax
  
  if (!dat_9f450)
    return 0;
  v1 = sub_20d60(0x81be4,0);
  *(unsigned long *)(v1 + 0x18) = a0;
  *(unsigned long *)(v1 + 0x20) = a1;
  v2 = sub_52567(dat_9f450,v1);
  sub_20e58(v1);
  return v2;
}


// Function: sub_2130d @ 0x2130d
void sub_2130d(unsigned long *a0) // return-dupe x2
{
  int4 v1;
  int8 v2; // rax
  char v3 [152];
  char v4 [16];
  
  v2 = sub_211c6(*a0);
  if (!v2)
    return;
  v1 = *(int4 *)&a0[0x35];
  if (fstat(v1,v3)) {
    sub_2b444(*(unsigned long *)(v2 + 0x58));
    return;
  }
  v4 = sub_5fd9e(v3);
  *(int8 *)(v2 + 8) = SUB168(v4,0);
  *(int8 *)(v2 + 0x10) = SUB168(v4,8);
  return;
}


// Function: sub_213c6 @ 0x213c6
int8 sub_213c6(char *a0,int8 a1,uint4 a2,char *a3)
{
  char *v1;
  unsigned int v10; // edx
  unsigned long v11; // stack - 0x58
  char v12 [16];
  int8 v13; // stack - 0x50
  int8 *v14; // stack - 0x48
  int8 v15; // stack - 0x40
  int8 v16; // stack - 0x38
  int8 v2;
  int8 v3;
  bool v4;
  char v5; // al
  int4 v6; // eax
  uint4 v7; // eax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v14 = (int8 *)(a1 + 0x58);
  v5 = -(char)(*v14 >> 0x3f);
  v4 = 0;
  v13 = sub_211c6(a0);
  if (v13) {
    if (*(uint4 *)(v13 + 0x3c) & 1) {
      if (!(a2 & 0x20)) {
        *a3 = 0x4e;
        return v13;
      }
      sub_29547(v13 + 0x58,a0);
    }
    v1 = *(char **)(v13 + 0x58);
    if (strcmp(v1,a0)) {
      *a3 = 0x4e;
      return v13;
    }
    if (((dat_9f8ec) && (((!(*(uint4 *)(v13 + 0x3c) & 2) || (v5 != '\x01')) && (*(int8 *)(v13 + 0x18) != *v14)))) || (*(int8 *)(v13 + 0x20) != v14[1])) {
      v16 = sub_2128a(*v14,v14[1]);
      if (v16) { // branch-flip
        v1 = *(char **)(v16 + 0x58);
        if (strcmp(v1,a0)) {
          if (dat_9ec84 & 0x2000) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v8 = sub_5e821(1,*(unsigned long *)(v16 + 0x58));
            v9 = sub_5e548(a0);
            error(0,0,gettext("%s: Directory has been renamed from %s"),v9,v8);
          }
          *(int8 *)(v13 + 0x40) = v16;
          *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 0x10;
          *(uint4 *)(v16 + 0x3c) = *(uint4 *)(v16 + 0x3c) & 0xffffffef;
          sub_20f0e(*(unsigned long *)(v16 + 0x58),a0);
        }
        *(unsigned int *)(v13 + 0x38) = 1;
      }
      else {
        v4 = 1;
        *(unsigned int *)(v13 + 0x38) = 2;
        *(int8 *)(v13 + 0x18) = *v14;
        *(int8 *)(v13 + 0x20) = v14[1];
      }
      if (v5)
        *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 2;
    }
    else {
      *(unsigned int *)(v13 + 0x38) = 1;
    }
    *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 4;
    goto label_218d6;
  }
  v2 = v14[1];
  v3 = *v14;
  v15 = sub_2128a(v3,v2);
  v2 = v14[1];
  v3 = *v14;
  v12 = sub_5fd9e(v14);
  v13 = sub_21019(a0,SUB168(v12,0),SUB168(v12,8),v3,v2,v5,1,0);
  if (v15) {
    v1 = *(char **)(v15 + 0x58);
    if (strcmp(v1,a0)) {
      if (dat_9ec84 & 0x2000) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v8 = sub_5e821(1,*(unsigned long *)(v15 + 0x58));
        v9 = sub_5e548(a0);
        error(0,0,gettext("%s: Directory has been renamed from %s"),v9,v8);
      }
      *(int8 *)(v13 + 0x40) = v15;
      *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 0x10;
      *(uint4 *)(v15 + 0x3c) = *(uint4 *)(v15 + 0x3c) & 0xffffffef;
      sub_20f0e(*(unsigned long *)(v15 + 0x58),a0);
    }
    *(unsigned int *)(v13 + 0x38) = 1;
    goto label_218d6;
  }
  *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 8;
  if (dat_9ec84 & 0x1000) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v8 = sub_5e548(a0);
    error(0,0,gettext("%s: Directory is new"),v8);
  }
  if (dat_9f8e0) { // branch-flip
label_218c3:
    v10 = 2;
  }
  else {
    v12 = sub_5fd9e(v14);
    v6 = sub_5ff90(SUB168(v12,0),SUB168(v12,8),dat_9f900,dat_9f908);
    if (v6 < 0) goto label_218c3;
    if (dat_9f87c) {
      v12 = sub_5fd84(v14);
      v6 = sub_5ff90(SUB168(v12,0),SUB168(v12,8),dat_9f900,dat_9f908);
      if (v6 <= -1) goto label_218c3;
    }
    v10 = 1;
  }
  *(unsigned int *)(v13 + 0x38) = v10;
label_218d6:
  if (((dat_9f935) && (*(int8 *)(a1 + 0x198))) && (*v14 != *(int8 *)(*(int8 *)(a1 + 0x198) + 0x58))) {
    if (dat_9ec84 & 0x40000) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v8 = sub_5e548(*(unsigned long *)(v13 + 0x58));
      error(0,0,gettext("%s: directory is on a different filesystem; not dumped"),v8);
    }
    *(unsigned int *)(v13 + 0x38) = 0;
    if (*(int8 *)(v13 + 0x28)) {
      sub_209fc(*(unsigned long *)(v13 + 0x28));
      *(unsigned long *)(v13 + 0x28) = 0;
    }
    v4 = 0;
  }
  else if ((a2 & 0x10) && (*(uint4 *)(v13 + 0x38) = a2 & 3, !*(int4 *)(v13 + 0x38)))
    *a3 = 0x4e;
  if ((v4) && (dat_9ec84 & 0x2000)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v8 = sub_5e548(a0);
    error(0,0,gettext("%s: Directory has been renamed"),v8);
  }
  *(uint4 *)(v13 + 0x3c) = *(uint4 *)(v13 + 0x3c) | 1;
  if (*(int4 *)(v13 + 0x38)) {
    v7 = sub_11fbf(a1,&v11);
    if (v7 != 3) { // branch-flip
      if (v7 <= 3) {
        if (v7 != 2) { // branch-flip
          if (((v7 <= 2) && (v7)) && (v7 == 1)) {
            sub_11f18(a0,v11,gettext("contents not dumped"));
            *(unsigned int *)(v13 + 0x38) = 0;
            *(unsigned long *)(v13 + 0x48) = v11;
          }
        }
        else {
          sub_11f18(a0,v11,gettext("contents not dumped"));
          *(unsigned long *)(v13 + 0x48) = v11;
        }
      }
    }
    else {
      sub_11f18(a0,v11,gettext("directory not dumped"));
      *a3 = 0x4e;
      *(unsigned int *)(v13 + 0x38) = 0;
    }
  }
  return v13;
}


// Function: sub_21b74 @ 0x21b74
void sub_21b74(int8 a0,char *a1)
{
  char *v1;
  char *v10; // stack - 0x38
  char *v11; // stack - 0x30
  unsigned long v12; // stack - 0x28
  char *v2;
  char v3;
  unsigned long v4;
  void *v5; // rax
  char *v6; // rax
  uint8 v7; // stack - 0x50
  uint8 v8; // stack - 0x48
  int8 v9; // stack - 0x40
  
  if (*(int4 *)(a0 + 0x38) != 2) { // branch-flip
    if (*(int8 *)(a0 + 0x40)) { // branch-flip
      if (*(int8 *)(*(int8 *)(a0 + 0x40) + 0x30)) // branch-flip
        v12 = *(unsigned long *)(*(int8 *)(a0 + 0x40) + 0x30);
      else {
        v12 = *(unsigned long *)(*(int8 *)(a0 + 0x40) + 0x28);
      }
    }
    else {
      v12 = *(unsigned long *)(a0 + 0x28);
    }
  }
  else {
    v12 = 0;
  }
  v8 = 0;
  v9 = 0;
  for (v10 = a1; *v10; v10 = &v10[strlen(v10) + 1]) {
    v9 = strlen(v10) + v9 + 2;
    v8 += 1;
  }
  v5 = (void *)sub_60c29(v8,8);
  v7 = 0;
  for (v10 = a1; *v10; v10 = &v10[strlen(v10) + 1]) {
    *(char **)(v7 * 8 + (int8)v5) = v10;
    v7 += 1;
  }
  qsort(v5,v8,8,sub_20a2b);
  v6 = (char *)sub_60b5b(v9 + 1);
  v11 = v6;
  for (v7 = 0; v7 < v8; v7 = v7 + 1) {
    v4 = *(unsigned long *)((int8)v5 + v7 * 8);
    if (sub_20a6a(v12,v4)) { // branch-flip
      if (*(int8 *)(a0 + 0x48)) // branch-flip
        *v11 = 'I';
      else {
        *v11 = ' ';
      }
    }
    else if (*(int8 *)(a0 + 0x48)) // branch-flip
      *v11 = 'I';
    else {
      *v11 = 'Y';
    }
    v11 = &v11[1];
    v10 = *(char **)((int8)v5 + v7 * 8);
    do {
      v1 = &v10[1];
      v2 = &v11[1];
      *v11 = *v10;
      v3 = *v11;
      v10 = v1;
      v11 = v2;
    } while (v3);
  }
  *v11 = '\0';
  *(unsigned long *)(a0 + 0x30) = *(unsigned long *)(a0 + 0x28);
  *(unsigned long *)(a0 + 0x28) = sub_2082e(v6,0);
  free(v6);
  free(v5);
  return;
}


// Function: sub_21e0d @ 0x21e0d
void sub_21e0d(int8 a0)
{
  uint8 v1; // rax
  char *v2; // rax
  
  v1 = strlen(*(char **)(a0 + 0x48));
  v2 = (char *)sub_60b5b(v1 + 3);
  *v2 = 0x59;
  memcpy(&v2[1],*(void **)(a0 + 0x48),v1 + 1);
  v2[v1 + 2] = 0;
  *(unsigned long *)(a0 + 0x30) = *(unsigned long *)(a0 + 0x28);
  *(unsigned long *)(a0 + 0x28) = sub_2082e(v2,0);
  free(v2);
  return;
}


// Function: sub_21eb9 @ 0x21eb9
int8 sub_21eb9(unsigned long *a0) // ternary
{
  char v1; // al
  char v10; // stack - 0x235
  unsigned int v11; // stack - 0x234
  int4 v12; // stack - 0x230
  int4 v13; // stack - 0x22c
  char *v14; // stack - 0x220
  code *v15; // stack - 0x218
  unsigned long v16; // stack - 0x210
  void *v17; // stack - 0x208
  int8 v18; // stack - 0x200
  void *v19; // stack - 0x1f8
  int4 v2; // eax
  int8 v20; // stack - 0x1f0
  unsigned long v21; // stack - 0x1e8
  unsigned long v22; // stack - 0x1e0
  uint4 v23; // stack - 0x168
  unsigned long *v24; // stack - 0x40
  int4 v25; // stack - 0x30
  int4 *v3; // rax
  int8 v4;
  void *v5; // stack - 0x228
  char v6 [88];
  int8 v7 [3]; // stack - 0x180
  unsigned long v8;
  char v9 [16];
  
  v16 = *a0;
  v17 = (void *)sub_14d3d(a0);
  v18 = a0[0xb];
  v10 = a0[0x33] == 0;
  if (!v17)
    sub_3e72e(v16);
  sub_17909(a0);
  v19 = (void *)sub_60cc6(v16);
  sub_298d7(v19);
  v8 = (v10) ? 0x20 : 0; // branch-flip
  v20 = sub_213c6(v19,a0,v8,&v4);
  free(v19);
  v21 = sub_2b66c(v16);
  if (v17) {
    if (*(int4 *)(v20 + 0x38)) { // branch-flip
      sub_21b74(v20,v17);
      v8 = *(unsigned long *)(v20 + 0x28);
      v14 = (char *)sub_20b9f(v8,1,&v5);
      while (v14) {
        v22 = sub_2b75b(v21,&v14[1]);
        if (*v14 != 'I') { // branch-flip
          v1 = sub_17c43(v22,a0);
          if (v1) // branch-flip
            *v14 = 'N';
          else {
            v12 = *(int4 *)&a0[0x35];
            v15 = NULL;
            sub_39c55(v6);
            if (0 <= v12) { // branch-flip
              v2 = fstatat(v12,&v14[1],v7,dat_9fc10);
              if (v2) // branch-flip
                v15 = sub_2b444;
              else if ((v23 & 0xf000) == 0x4000) {
                v13 = sub_157f9(a0,&v14[1],dat_9fc08);
                if (0 <= v13) { // branch-flip
                  v25 = v13;
                  v2 = fstat(v13,v7);
                  if (v2)
                    v15 = sub_2b444;
                }
                else {
                  v15 = sub_2b2b1;
                }
              }
            }
            else {
              v3 = __errno_location();
              *v3 = -v12;
              v15 = sub_2b2b1;
            }
            if (v15) { // branch-flip
              sub_2b48b(v22,0,v15);
              *v14 = 'N';
            }
            else if ((v23 & 0xf000) != 0x4000) { // branch-flip
              if ((dat_9f935) && (v18 != v7[0]))
                *v14 = 'N';
              else if (*v14 != 'Y') {
                v9 = sub_5fd9e(v7);
                v2 = sub_5ff90(SUB168(v9,0),SUB168(v9,8),dat_9f900,dat_9f908);
                if (0 <= v2) { // branch-flip
label_2236f:
                  *v14 = 'Y';
                }
                else {
                  if (dat_9f87c) {
                    v9 = sub_5fd84(v7);
                    v2 = sub_5ff90(SUB168(v9,0),SUB168(v9,8),dat_9f900,dat_9f908);
                    if (0 <= v2) goto label_2236f;
                  }
                  *v14 = 'N';
                }
              }
            }
            else {
              v11 = 0;
              if (dat_9f930) { // branch-flip
                if (*(int4 *)(v20 + 0x38) == 2)
                  v11 = 0x12;
              }
              else {
                v11 = 0x10;
              }
              *v14 = 'D';
              v24 = a0;
              sub_213c6(v22,v6,v11,v14);
              sub_1588e(v6);
            }
            sub_39d1c(v6);
          }
        }
        else {
          *v14 = 'N';
        }
        v14 = (char *)sub_20ad3(v5);
      }
      free(v5);
    }
    else if (*(int8 *)(v20 + 0x48))
      sub_21e0d(v20);
  }
  sub_2b72d(v21);
  free(v17);
  return v20;
}


// Function: sub_2241b @ 0x2241b
unsigned long sub_2241b(int8 a0) // early-return x2
{
  if (!a0)
    return 0;
  if (*(int8 *)(a0 + 0x28))
    return **(unsigned long **)(a0 + 0x28);
  return 0;
}


// Function: sub_22456 @ 0x22456
int8 sub_22456(unsigned long a0) // early-return
{
  int8 v1; // rax
  
  v1 = sub_2241b(a0);
  if (v1)
    return v1;
  return 0x81ca8;
}


// Function: sub_2248c @ 0x2248c
void sub_2248c(int8 a0,char *a1,char *a2)
{
  char *v1;
  uint8 v2; // rax
  
  if (*a1)
    a1 = (char *)sub_3edd9(a1,0,dat_9f878);
  if (*(int8 *)(a0 + 0x20) == *(int8 *)(a0 + 0x18))
    _obstack_newchunk(a0,1);
  v1 = *(char **)(a0 + 0x18);
  *(char **)(a0 + 0x18) = &v1[1];
  *v1 = 0x52;
  v2 = strlen(a1) + 1;
  if ((uint8)(*(int8 *)(a0 + 0x20) - *(int8 *)(a0 + 0x18)) < v2)
    _obstack_newchunk(a0,v2);
  memcpy(*(void **)(a0 + 0x18),a1,v2);
  *(uint8 *)(a0 + 0x18) = *(int8 *)(a0 + 0x18) + v2;
  if (*a2)
    a2 = (char *)sub_3edd9(a2,0,dat_9f878);
  if (*(int8 *)(a0 + 0x20) == *(int8 *)(a0 + 0x18))
    _obstack_newchunk(a0,1);
  v1 = *(char **)(a0 + 0x18);
  *(char **)(a0 + 0x18) = &v1[1];
  *v1 = 0x54;
  v2 = strlen(a2) + 1;
  if ((uint8)(*(int8 *)(a0 + 0x20) - *(int8 *)(a0 + 0x18)) < v2)
    _obstack_newchunk(a0,v2);
  memcpy(*(void **)(a0 + 0x18),a2,v2);
  *(uint8 *)(a0 + 0x18) = *(int8 *)(a0 + 0x18) + v2;
  return;
}


// Function: sub_226b7 @ 0x226b7
void sub_226b7(int8 a0,int8 a1)
{
  char *v1;
  char *v2; // rax
  uint8 v3; // rax
  int8 v4; // stack - 0x48
  int8 v5; // stack - 0x40
  
  for (v4 = a0; (v4 && (a0 != *(int8 *)(v4 + 0x40))); v4 = *(int8 *)(v4 + 0x40)) {
  }
  v5 = a0;
  if (v4) { // branch-flip
    v2 = (char *)sub_4e10c(*(unsigned long *)(a0 + 0x58));
    if (*(int8 *)(a1 + 0x20) == *(int8 *)(a1 + 0x18))
      _obstack_newchunk(a1,1);
    v1 = *(char **)(a1 + 0x18);
    *(char **)(a1 + 0x18) = &v1[1];
    *v1 = 0x58;
    v3 = strlen(v2) + 1;
    if ((uint8)(*(int8 *)(a1 + 0x20) - *(int8 *)(a1 + 0x18)) < v3)
      _obstack_newchunk(a1,v3);
    memcpy(*(void **)(a1 + 0x18),v2,v3);
    *(uint8 *)(a1 + 0x18) = *(int8 *)(a1 + 0x18) + v3;
    sub_2248c(a1,*(unsigned long *)(a0 + 0x58),0x81be4);
    for (; v5 != v4; v5 = *(int8 *)(v5 + 0x40)) {
      sub_2248c(a1,*(unsigned long *)(*(int8 *)(v5 + 0x40) + 0x58),*(unsigned long *)(v5 + 0x58));
    }
    sub_2248c(a1,0x81be4,*(unsigned long *)(v4 + 0x58));
    free(v2);
  }
  else {
    for (; (v5 && (*(int8 *)(v5 + 0x40))); v5 = *(int8 *)(v5 + 0x40)) {
      sub_2248c(a1,*(unsigned long *)(*(int8 *)(v5 + 0x40) + 0x58),*(unsigned long *)(v5 + 0x58));
    }
  }
  return;
}


// Function: sub_228cd @ 0x228cd
void sub_228cd(int8 a0) // return-dupe
{
  char *v1;
  uint8 v10; // stack - 0x38
  uint1 v11; // stack - 0x18
  void *v2; // rax
  char v3 [8];
  uint8 v4; // stack - 0xd0
  unsigned long *v5; // stack - 0xc8
  int8 v6; // stack - 0x60
  char *v7; // stack - 0x58
  char *v8; // stack - 0x50
  char *v9; // stack - 0x48
  
  if (!dat_9f438)
    return;
  _obstack_begin(v3,0,0,sub_60b5b,dat_9bfc0);
  v2 = (void *)sub_2241b(a0);
  if (v2) { // branch-flip
    v4 = sub_20bfe(v2) - 1;
    if ((uint8)((int8)v9 - (int8)v8) < v4)
      _obstack_newchunk(v3,v4);
    memcpy(v8,v2,v4);
    v8 = &v8[v4];
  }
  else {
    v4 = 0;
  }
  for (v5 = dat_9f438; v5; v5 = (unsigned long *)*v5) {
    if (*(uint4 *)((int8)v5 + 0x3c) & 0x10)
      sub_226b7(v5,v3);
  }
  if ((a0) && ((int8)v8 - (int8)v7 != v4)) {
    if (v9 == v8)
      _obstack_newchunk(v3,1);
    v1 = &v8[1];
    *v8 = 0;
    v8 = v1;
    sub_209fc(*(unsigned long *)(a0 + 0x28));
    v1 = v7;
    if (v7 == v8)
      v11 |= 2;
    v8 = (char *)(~v10 & (uint8)&v8[v10]);
    if ((uint8)((int8)v9 - v6) < (uint8)((int8)v8 - v6))
      v8 = v9;
    v7 = v8;
    *(unsigned long *)(a0 + 0x28) = sub_209d4(v1);
  }
  _obstack_free(v3,0);
  return;
}


// Function: sub_22c72 @ 0x22c72
void sub_22c72(int4 a0,char *a1)
{
  char v1;
  char v10 [16];
  int8 v11; // stack - 0x68
  char *v12; // stack - 0x60
  char *v13; // stack - 0x58
  uint8 v14; // stack - 0x50
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x40
  unsigned long v17; // stack - 0x38
  uint8 v18; // stack - 0x30
  char v2; // al
  int4 v3; // eax
  int8 v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  int8 v7; // stack - 0x78
  char *v8; // stack - 0x80
  char *v9; // stack - 0x70
  
  v8 = NULL;
  v7 = 0;
  v11 = 1;
  if (a0 != 1) { // branch-flip
    v5 = strdup(a1);
    v8 = v5;
    v7 = strlen(v5) + 1;
  }
  else {
    v4 = getline(&v8,&v7,dat_9f458);
    if (v4 <= 0) {
      sub_3e403(dat_9f8e0);
      free(v8);
      return;
    }
    v11 += 1;
  }
  v10 = sub_29ff7(v8,&v9,0);
  dat_9f908 = SUB168(v10,8);
  dat_9f900 = SUB168(v10,0);
  v1 = sub_294f0(dat_9f900,dat_9f908);
  if (v1 != '\x01') {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v5 = gettext("Invalid time stamp");
    v6 = sub_5e548(dat_9f8e0);
    error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
    sub_177f4(); // no-return
  }
  if ((a0 == 1) && (*v9)) {
    v13 = &v9[1];
    *__errno_location() = 0;
    v14 = strtoumax(v13,&v9,10);
    if ((!*__errno_location()) && (1000000000 <= v14))
      *__errno_location() = 0x22;
    if ((*__errno_location()) || (v13 == v9)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v5 = gettext("Invalid time stamp");
      v6 = sub_5e548(dat_9f8e0);
      error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
      dat_9fdd0 = 2;
      dat_9f900 = 0x8000000000000000;
      dat_9f908 = 0xffffffffffffffff;
    }
    else {
      dat_9f908 = v14;
    }
  }
  while( true ) {
    v4 = getline(&v8,&v7,dat_9f458);
    v3 = (int4)v4;
    if (v3 <= 0) {
      free(v8);
      return;
    }
    v1 = *v8;
    v12 = &v8[v1 == '+'];
    v11 += 1;
    if (v8[(int8)v3 + -1] == '\n')
      v8[(int8)v3 + -1] = '\0';
    if (a0 != 1) { // branch-flip
      v18 = 0;
      v17 = 0;
    }
    else {
      v10 = sub_29ff7(v12,&v9,0);
      v18 = SUB168(v10,8);
      v17 = SUB168(v10,0);
      v12 = v9;
      v2 = sub_294f0(v17,v18);
      if ((v2 != '\x01') || (*v12 != ' ')) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid modification time");
        v6 = sub_5e548(dat_9f8e0);
        error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
        sub_177f4(); // no-return
      }
      *__errno_location() = 0;
      v14 = strtoumax(v12,&v9,10);
      if ((!*__errno_location()) && (1000000000 <= v14))
        *__errno_location() = 0x22;
      if (((*__errno_location()) || (v12 == v9)) || (*v9 != ' ')) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid modification time (nanoseconds)");
        v6 = sub_5e548(dat_9f8e0);
        error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
        sub_177f4(); // no-return
      }
      v18 = v14;
      v12 = v9;
    }
    v15 = sub_29cfc(v12,&v9,0,0xffffffffffffffff);
    v12 = v9;
    if ((*__errno_location()) || (*v12 != ' ')) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v5 = gettext("Invalid device number");
      v6 = sub_5e548(dat_9f8e0);
      error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
      sub_177f4(); // no-return
    }
    v16 = sub_29cfc(v12,&v9,0,0xffffffffffffffff);
    v12 = v9;
    if ((*__errno_location()) || (*v12 != ' ')) break;
    v12 = &v12[1];
    sub_2961d(v12);
    sub_21019(v12,v17,v18,v15,v16,v1 == '+',0,0);
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v5 = gettext("Invalid inode number");
  v6 = sub_5e548(dat_9f8e0);
  error(0,*__errno_location(),"%s:%ld: %s",v6,v11,v5);
  sub_177f4(); // no-return
}


// Function: sub_232e4 @ 0x232e4
int4 sub_232e4(void *a0,int8 a1,int8 *a2)
{
  char *v1;
  int4 v2; // stack - 0x34
  int8 v3; // stack - 0x30
  
  v3 = 0;
  v2 = getc_unlocked(a0);
  while ((v2 != -1 && (v2))) {
    if (*(int8 *)(a1 + 0x20) == *(int8 *)(a1 + 0x18))
      _obstack_newchunk(a1,1);
    v1 = *(char **)(a1 + 0x18);
    *(char **)(a1 + 0x18) = &v1[1];
    *v1 = (char)v2;
    v3 += 1;
    v2 = getc_unlocked(a0);
  }
  if (*(int8 *)(a1 + 0x20) == *(int8 *)(a1 + 0x18))
    _obstack_newchunk(a1,1);
  v1 = *(char **)(a1 + 0x18);
  *(char **)(a1 + 0x18) = &v1[1];
  *v1 = 0;
  *a2 = v3;
  return v2;
}


// Function: sub_233ed @ 0x233ed
unsigned long sub_233ed(void *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long *a4) // return-dupe
{
  int4 v1;
  char v10 [24];
  char v11 [32];
  int4 v12; // stack - 0xb8
  uint4 v13; // stack - 0xb4
  uint4 v2; // eax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  char v8 [32];
  char v9 [32];
  
  v2 = getc_unlocked(a0);
  v12 = 0;
  v13 = v2;
  while (((!v12 && (v2 == 0x2d)) || (v13 - 0x30 <= 9))) {
    v8[v12] = (char)v13;
    if (v12 == 0x14) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_54d1c(ftello(a0),v9);
      v5 = sub_5e548(dat_9f8e0);
      error(0,0,gettext("%s: byte %s: %s %.*s... too long"),v5,v4,a1,0x15,v8);
      sub_177f4(); // no-return
    }
    v12 += 1;
    v13 = getc_unlocked(a0);
  }
  v8[v12] = 0;
  if (0 <= (int4)v13) {
    if (v13) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_54d1c(ftello(a0),v9);
      v5 = sub_5e548(dat_9f8e0);
      error(0,0,gettext("%s: byte %s: %s %s followed by invalid byte 0x%02x"),v5,v4,a1,v8,(uint8)v13);
      sub_177f4(); // no-return
    }
    *a4 = sub_29cfc(v8,0,a2,a3);
    v1 = *__errno_location();
    if (!v1)
      return 1;
    if (v1 == 0x22) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = sub_54e5a(a3,v10);
      v5 = sub_54bde(a2,v11);
      v6 = sub_54d1c(ftello(a0),v9);
      v7 = sub_5e548(dat_9f8e0);
      error(0,0x22,gettext("%s: byte %s: (valid range %s..%s)\n\t%s %s"),v7,v6,v5,v4,a1,v8);
      sub_177f4(); // no-return
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v4 = sub_54d1c(ftello(a0),v9);
    v5 = sub_5e548(dat_9f8e0);
    error(0,v1,gettext("%s: byte %s: %s %s"),v5,v4,a1,v8);
    sub_177f4(); // no-return
  }
  if (ferror_unlocked(a0))
    sub_3e5d6(dat_9f8e0); // no-return
  if (!v12)
    return 0;
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v3 = gettext("Unexpected EOF in snapshot file");
  v4 = sub_5e548(dat_9f8e0);
  error(0,0,"%s: %s",v4,v3);
  sub_177f4(); // no-return
}


// Function: sub_238b6 @ 0x238b6
void sub_238b6(unsigned long a0,unsigned long *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  
  if ((sub_233ed(a0,0x81e04,0x8000000000000000,0x7fffffffffffffff,&v3)) && (sub_233ed(a0,0x81e08,0,999999999,&v4))) {
    *a1 = v3;
    a1[1] = v4;
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v1 = gettext("Unexpected EOF in snapshot file");
  v2 = sub_5e548(dat_9f8e0);
  error(0,0,"%s: %s",v2,v1);
  sub_177f4(); // no-return
}


// Function: sub_239ca @ 0x239ca
void sub_239ca(void)
{
  uint8 v1;
  uint8 v10; // stack - 0x110
  char v11 [24];
  bool v12; // zf
  int8 v13; // stack - 0x108
  int8 v14; // stack - 0x100
  char *v15; // stack - 0xf8
  uint8 v16; // stack - 0xf0
  uint8 v17; // stack - 0xe8
  char *v18; // stack - 0xe0
  uint8 v19; // stack - 0xd8
  char v2; // al
  uint8 v20; // stack - 0xd0
  char *v21; // stack - 0xc8
  uint8 v22; // stack - 0xc0
  unsigned long v23; // stack - 0xb0
  uint8 v24; // stack - 0xa0
  uint8 v25; // stack - 0x98
  uint8 v26; // stack - 0x90
  uint8 v27; // stack - 0x88
  uint8 v28; // stack - 0x78
  uint1 v29; // stack - 0x58
  int4 v3; // eax
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  char v7 [8];
  int8 v8; // stack - 0x118
  unsigned long v9; // stack - 0xb8
  
  _obstack_begin(v7,0,0,sub_60b5b,dat_9bfc0);
  sub_238b6(dat_9f458,0x9f900);
  while( true ) {
    v2 = sub_233ed(dat_9f458,0x81e0d,0,1,&v8);
    if (v2 != '\x01')
      return;
    v12 = v8 != 0;
    sub_238b6(dat_9f458,&v9);
    v2 = sub_233ed(dat_9f458,0x81e11,0,0xffffffffffffffff,&v8);
    if (v2 != '\x01') break;
    v13 = v8;
    v2 = sub_233ed(dat_9f458,0x81e15,0,0xffffffffffffffff,&v8);
    if (v2 != '\x01') break;
    v14 = v8;
    v3 = sub_232e4(dat_9f458,v7,&v10);
    if (v3) break;
    v15 = v7;
    v16 = v25;
    if (v25 == v26)
      v29 |= 2;
    v26 = ~v28 & v26 + v28;
    if (v27 - v24 < v26 - v24)
      v26 = v27;
    v25 = v26;
    v17 = v16;
    do {
      v3 = sub_232e4(dat_9f458,v7,&v10);
      if (v3) break;
    } while (2 <= v10);
    v3 = getc_unlocked(dat_9f458);
    v1 = v25;
    if (v3) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v4 = gettext("Missing record terminator");
      v5 = ftello(dat_9f458);
      v5 = sub_54d1c(v5,v11);
      v6 = sub_5e548(dat_9f8e0);
      error(0,0,gettext("%s: byte %s: %s"),v6,v5,v4);
      sub_177f4(); // no-return
    }
    v18 = v7;
    v19 = v25;
    if (v25 == v26)
      v29 |= 2;
    v26 = ~v28 & v26 + v28;
    if (v27 - v24 < v26 - v24)
      v26 = v27;
    v25 = v26;
    v20 = v19;
    sub_21019(v17,v9,v23,v13,v14,v12,0,v1);
    v21 = v7;
    v22 = v20;
    if ((v20 <= v24) || (v27 <= v20))
      _obstack_free(v21,v20);
    else {
      v25 = v20;
      v26 = v20;
    }
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v4 = gettext("Unexpected EOF in snapshot file");
  v5 = sub_5e548(dat_9f8e0);
  error(0,0,"%s: %s",v5,v4);
  sub_177f4(); // no-return
}


// Function: sub_23f6b @ 0x23f6b
void sub_23f6b(void)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [24];
  char v4 [32];
  int8 *v5; // stack - 0x60
  
  printf("This tar\'s snapshot file field ranges are\n");
  printf("   (%-15s => [ %s, %s ]):\n\n","field name",0x81e8f,0x81e8b);
  for (v5 = (int8 *)0x9a640; *v5; v5 = &v5[3]) {
    v1 = sub_29c96(v5[2],v5[1],v5[2],v3);
    v2 = sub_29c96(v5[1],v5[1],v5[2],v4);
    printf("    %-15s => [ %s, %s ],\n",*v5,v2,v1);
  }
  printf("\n");
  return;
}


// Function: sub_24090 @ 0x24090
void sub_24090(void) // return-dupe x2
{
  int4 v1; // eax
  int8 v2; // rax
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  char *v5; // stack - 0x30
  int4 v6; // stack - 0x38
  char *v7; // stack - 0x20
  uint8 v8; // stack - 0x18
  
  v5 = NULL;
  v4 = 0;
  v6 = 0x42;
  if (!dat_9f8e8)
    v6 = 0x242;
  v1 = open(dat_9f8e0,v6,0x1b6);
  if (v1 <= -1) {
    sub_3e38b(dat_9f8e0);
    return;
  }
  dat_9f458 = fdopen(v1,"r+");
  if (!dat_9f458) {
    sub_3e38b(dat_9f8e0);
    close(v1);
    return;
  }
  sub_2eda0();
  sub_2ee43();
  v2 = getline(&v5,&v4,dat_9f458);
  if (1 <= v2) {
    v1 = strncmp(v5,"GNU tar",7);
    if (v1) // branch-flip
      v8 = 0;
    else {
      v7 = &v5[8];
      if (v5[7] != '-') {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(1,0,gettext("Bad incremental file format"));
        return;
      }
      for (; *v7 != '-'; v7 = &v7[1]) {
        if (!*v7) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(1,0,gettext("Bad incremental file format"));
          return;
        }
      }
      v3 = &v7[1];
      v8 = strtoumax(v3,0,10);
    }
    if (2 <= v8) { // branch-flip
      if (v8 != 2) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(1,0,gettext("Unsupported incremental format version: %lu"),v8);
        return;
      }
      sub_239ca();
    }
    else {
      sub_22c72(v8 & 0xffffffff,v5);
    }
  }
  v1 = ferror_unlocked(dat_9f458);
  if (v1)
    sub_3e403(dat_9f8e0);
  free(v5);
  return;
}


// Function: sub_24350 @ 0x24350
unsigned long sub_24350(int8 a0,void *a1) // ternary
{
  unsigned long v1;
  char *v10; // stack - 0x30
  int4 v2; // eax
  char *v3;
  uint8 v4; // rax
  char v5 [24];
  void *v6; // stack - 0x50
  char *v7; // stack - 0x48
  int8 v8; // stack - 0x40
  void *v9; // stack - 0x38
  
  v8 = a0;
  v9 = a1;
  if (*(uint4 *)(a0 + 0x3c) & 4) {
    v10 = (*(uint4 *)(a0 + 0x3c) & 2) ? (void *)0x81f2c : (void *)0x81f2e; // branch-flip
    fwrite_unlocked(v10,2,1,a1);
    v3 = (char *)sub_29c96(*(unsigned long *)(v8 + 8),0x8000000000000000,0x7fffffffffffffff,v5);
    v10 = v3;
    fwrite_unlocked(v10,strlen(v3) + 1,1,v9);
    v3 = (char *)sub_54bde(*(unsigned long *)(v8 + 0x10),v5);
    v10 = v3;
    fwrite_unlocked(v10,strlen(v3) + 1,1,v9);
    v3 = (char *)sub_29c96(*(unsigned long *)(v8 + 0x18),0,0xffffffffffffffff,v5);
    v10 = v3;
    fwrite_unlocked(v10,strlen(v3) + 1,1,v9);
    v3 = (char *)sub_29c96(*(unsigned long *)(v8 + 0x20),0,0xffffffffffffffff,v5);
    v10 = v3;
    fwrite_unlocked(v10,strlen(v3) + 1,1,v9);
    v3 = *(char **)(v8 + 0x58);
    fwrite_unlocked(*(void **)(v8 + 0x58),strlen(v3) + 1,1,v9);
    if (*(int8 *)(v8 + 0x28)) {
      v1 = *(unsigned long *)(v8 + 0x28);
      v7 = (char *)sub_20b9f(v1,0,&v6);
      while (v7) {
        v4 = strlen(v7);
        fwrite_unlocked(v7,v4 + 1,1,v9);
        v7 = (char *)sub_20ad3(v6);
      }
      free(v6);
    }
    fwrite_unlocked((void *)0x81f30,2,1,v9);
  }
  v2 = ferror_unlocked(v9);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
}


// Function: sub_24600 @ 0x24600
void sub_24600(void) // return-dupe x2
{
  void *v1;
  int4 v2; // eax
  char *v3; // rax
  char v4 [24];
  
  v1 = dat_9f458;
  if (!dat_9f458)
    return;
  v2 = fseeko(dat_9f458,0,0);
  if (v2)
    sub_3e780(dat_9f8e0);
  v2 = fileno(v1);
  if (sub_32f52(v2))
    sub_3ea55(dat_9f8e0);
  fprintf(v1,"%s-%s-%d\n","GNU tar",0x81f33,2);
  v3 = (char *)sub_54bde(dat_9f9e0,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  v3 = (char *)sub_54e5a(dat_9f9e8,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  if ((!ferror_unlocked(v1)) && (dat_9f448))
    sub_5279c(dat_9f448,sub_24350,v1);
  if (ferror_unlocked(v1))
    sub_3eb22(dat_9f8e0);
  if (!fclose(v1))
    return;
  sub_3e1ff(dat_9f8e0);
  return;
}


// Function: sub_247c6 @ 0x247c6
void sub_247c6(int8 a0)
{
  void *v1; // rax
  void *v2; // rax
  uint8 v3; // stack - 0x30
  uint8 v4; // stack - 0x28
  void *v5; // stack - 0x20
  
  v3 = *(uint8 *)(a0 + 0x88);
  v1 = (void *)sub_60b5b(v3);
  v5 = v1;
  sub_bba7(dat_9f460);
  sub_ad2b(a0);
  while( true ) {
    if (!v3) {
      sub_ad66();
      *(void **)(a0 + 400) = v1;
      *(char *)(a0 + 0x189) = 1;
      return;
    }
    sub_ad86(v3);
    v2 = (void *)sub_bb4b();
    if (!v2) break;
    v4 = sub_bbf5(v2);
    if (v3 < v4)
      v4 = v3;
    memcpy(v5,v2,v4);
    v5 = (void *)((int8)v5 + v4);
    sub_bba7((int8)v2 + (v4 - 1));
    v3 -= v4;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(1,0,gettext("Unexpected EOF in archive"));
  return;
}


// Function: sub_2490b @ 0x2490b
char sub_2490b(int8 a0)
{
  if ((*(char *)(a0 + 0x188)) && (!*(int8 *)(a0 + 400)))
    sub_247c6(a0);
  return *(char *)(a0 + 0x188);
}


// Function: sub_24953 @ 0x24953
unsigned long sub_24953(uint1 *a0) // return-dupe x6
{
  uint1 v1;
  bool v2;
  uint4 v3; // stack - 0x24
  uint1 *v4; // stack - 0x20
  
  v2 = 0;
  v3 = 0;
  v4 = a0;
  do {
    if (!*v4) {
      if (!v3) {
        if ((v2) && (dat_9ec84 & 2)) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Malformed dumpdir: \'X\' never used"));
        }
        return 1;
      }
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Malformed dumpdir: expected \'%c\' but found end of data"),(uint8)v3);
      dat_9fdd0 = 2;
      return 0;
    }
    if ((v3) && (v3 != (int4)(char)*v4)) {
      v1 = *v4;
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Malformed dumpdir: expected \'%c\' but found %#3o"),(uint8)v3,(uint8)v1);
      dat_9fdd0 = 2;
      return 0;
    }
    switch(*v4) {
      case 0x52:
        if (!v4[1]) {
          if (!v2) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Malformed dumpdir: empty name in \'R\'"));
            dat_9fdd0 = 2;
            return 0;
          }
          v2 = 0;
        }
        v3 = 0x54;
        break;
      case 0x54:
        if (v3 != 0x54) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Malformed dumpdir: \'T\' not preceded by \'R\'"));
          dat_9fdd0 = 2;
          return 0;
        }
        if ((!v4[1]) && (!v2)) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Malformed dumpdir: empty name in \'T\'"));
          dat_9fdd0 = 2;
          return 0;
        }
        v3 = 0;
        break;
      case 0x58:
        if (v2) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Malformed dumpdir: \'X\' duplicated"));
          dat_9fdd0 = 2;
          return 0;
        }
        v2 = 1;
      
    }
    v4 = &v4[strlen((char *)v4) + 1];
  } while( true );
}


// Function: sub_24ca0 @ 0x24ca0
unsigned long sub_24ca0(unsigned long a0) // return-dupe x3
{
  int4 v1;
  char *v10; // stack - 0x108
  char *v11; // stack - 0x100
  void *v12; // stack - 0xf8
  char *v13; // stack - 0xf0
  char *v14; // stack - 0xe8
  char *v15; // stack - 0xe0
  uint4 v16; // stack - 0xa0
  char v2; // al
  char *v3; // rax
  uint8 v4; // rax
  unsigned long v5; // rax
  unsigned long v6;
  char *v7; // rax
  unsigned long v8; // rax
  int8 v9 [3]; // stack - 0xb8
  
  v13 = NULL;
  if (sub_2490b(0x9fa20) != '\x01')
    return 0;
  v3 = (char *)sub_2b8e1(a0,0);
  if (!v3)
    return 0;
  v2 = sub_24953(dat_9fbb0);
  if (v2 != '\x01')
    return 0;
  v11 = dat_9fbb0;
  do {
    if (!*v11) {
      free(v13);
      v5 = sub_209d4(dat_9fbb0);
      v12 = NULL;
      for (v10 = v3; *v10; v10 = &v10[strlen(v10) + 1]) {
        free(v12);
        v12 = (void *)sub_2ee85(a0,v10);
        if (sub_2a9b8(v12,v9)) { // branch-flip
          if (*__errno_location() != 2) {
            sub_2b444(v12);
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v6 = sub_5e548(v12);
            error(0,0,gettext("%s: Not purging directory: unable to stat"),v6);
          }
        }
        else {
          v7 = (char *)sub_20a6a(v5,v10);
          if (((!v7) || ((*v7 == 'D' && ((v16 & 0xf000) != 0x4000)))) || ((*v7 == 'Y' && ((v16 & 0xf000) == 0x4000)))) {
            if ((dat_9f935) && (v9[0] != dat_9fc20)) {
              if (dat_9fdc8)
                (*dat_9fdc8)();
              v6 = sub_5e548(v12);
              error(0,0,gettext("%s: directory is on a different device: not purging"),v6);
            }
            else if ((dat_9f8c8 != '\x01') || (sub_34af4("delete",v12))) {
              if (dat_9f9ac) {
                v8 = sub_5e84e(v12);
                v6 = dat_9fe48;
                fprintf(dat_9efd0,gettext("%s: Deleting %s\n"),v6,v8);
              }
              if (!sub_2a36f(v12,1)) {
                v1 = *__errno_location();
                if (dat_9fdc8)
                  (*dat_9fdc8)();
                v6 = sub_5e548(v12);
                error(0,v1,gettext("%s: Cannot remove"),v6);
                dat_9fdd0 = 2;
              }
            }
          }
        }
      }
      free(v12);
      sub_209fc(v5);
      free(v3);
      return 1;
    }
    if (*v11 != 'X') { // branch-flip
      if (*v11 == 'R') {
        v14 = &v11[1];
        v11 = &v11[strlen(v11) + 1];
        v15 = &v11[1];
        if (*v14)
          v14 = (char *)sub_3edd9(v14,0,dat_9f878);
        if (*v15)
          v15 = (char *)sub_3edd9(v15,0,dat_9f878);
        if (*v14) { // branch-flip
          if (!*v15)
            v15 = v13;
        }
        else {
          v14 = v13;
        }
        if (sub_1c326(v14,v15) != '\x01') {
          free(v13);
          free(v3);
          return 0;
        }
      }
    }
    else {
      v4 = strlen(&v11[1]);
      v13 = (char *)sub_60b8d(v13,v4 + 0xc);
      memcpy(v13,&v11[1],v4);
      v13[v4] = '/';
      memcpy(&v13[v4 + 1],"tar.XXXXXX",0xb);
      if (!mkdtemp(v13)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = sub_5e84e(v13);
        v7 = gettext("Cannot create temporary directory using template %s");
        error(0,*__errno_location(),v7,v5);
        dat_9fdd0 = 2;
        free(v13);
        free(v3);
        return 0;
      }
    }
    v11 = &v11[strlen(v11) + 1];
  } while( true );
}


// Function: sub_25377 @ 0x25377
void sub_25377(unsigned long a0) // return-dupe
{
  if (sub_24ca0(a0) == '\x01')
    return;
  sub_28891();
  return;
}


// Function: sub_253a2 @ 0x253a2
void sub_253a2(char *a0,int8 a1)
{
  char v1;
  bool v2;
  int8 v3; // stack - 0x28
  char *v4; // stack - 0x20
  
  v2 = 0;
  v3 = a1;
  v4 = a0;
  while (v3) {
    v1 = *v4;
    if (v1) { // branch-flip
      if (((('\0' <= v1) && (v1 <= 'Y')) && ('D' <= v1)) && (0x314401UL >> (v1 + 0xbcU & 0x3f) & 1)) {
        fprintf(dat_9efd0,"%c",(uint8)(uint4)(int4)*v4);
        if (!v2) {
          fprintf(dat_9efd0," ");
          v2 = 1;
        }
        v4 = &v4[1];
        v3 -= 1;
      }
      else {
        fputc_unlocked((int4)*v4,dat_9efd0);
        v4 = &v4[1];
        v3 -= 1;
      }
    }
    else {
      fputc_unlocked(10,dat_9efd0);
      v4 = &v4[1];
      v3 -= 1;
      v2 = 0;
    }
  }
  return;
}


// Function: sub_254c9 @ 0x254c9
void sub_254c9(void)
{
  int4 v1; // stack - 0xc
  
  memset((void *)0x9f4a0,0x40,0x100);
  for (v1 = 0; v1 <= 0x3f; v1 = v1 + 1) {
    *(char *)((int8)(int4)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/%s: Omitting"[v1] + 0x9f4a0) = (char)v1;
  }
  return;
}


// Function: sub_25529 @ 0x25529
char * sub_25529(char *a0,int4 *a1)
{
  int4 v1;
  char *v2; // rax
  char *v3; // stack - 0x20
  uint8 v4; // stack - 0x10
  
  v1 = *a1;
  if (v1 != 4) { // branch-flip
    v3 = a0;
    if (v1 <= 4) {
      if (v1 != 1) { // branch-flip
        if (v1 == 2) {
          v2 = (char *)sub_3edd9(a0,1,dat_9f878);
          v3 = v2;
        }
      }
      else {
        v2 = (char *)sub_3edd9(a0,0,dat_9f878);
        v3 = v2;
      }
    }
    if (dat_9f970) {
      v2 = (uint8)sub_2ef5c(v3,dat_9f970);
      v4 = (uint8)v2;
      if (v2 == (char *)0xffffffffffffffff) {
        v2 = (uint8)strlen(v3);
        v4 = (uint8)v2;
      }
      v3 = &v3[v4];
    }
    v2 = v3;
  }
  else {
    v2 = a0;
  }
  return v2;
}


// Function: sub_255f5 @ 0x255f5
void sub_255f5(unsigned long a0,unsigned int a1)
{
  unsigned int v1; // stack - 0x14
  unsigned long v2; // stack - 0x10
  
  v1 = a1;
  v2 = a0;
  sub_3b8af(a0,a1,sub_25529,&v1);
  return;
}


// Function: sub_25627 @ 0x25627
void sub_25627(unsigned long *a0)
{
  char *v1;
  int4 v2; // eax
  int4 v3; // eax
  uint8 v4; // rax
  unsigned long v5; // rax
  char *v6; // stack - 0x18
  
  v1 = (char *)*a0;
  for (v6 = v1; (*v6 && ((*v6 == '/' || (*v6 == '.')))); v6 = &v6[1]) {
  }
  if (*v6) { // branch-flip
    v4 = strlen(dat_9f938);
    v2 = (int4)v4;
    v3 = strncmp(v6,dat_9f938,(int8)v2);
    if (!v3) {
      if (v6[v2] == '/')
        return;
      if (!v6[v2])
        return;
    }
    v5 = sub_2ee85(dat_9f938,v1);
    *a0 = v5;
    sub_29946(*a0);
  }
  else {
    v5 = sub_60cc6(dat_9f938);
    *a0 = v5;
  }
  free(v1);
  return;
}


// Function: sub_25733 @ 0x25733
void sub_25733(int4 a0,int8 a1) // return-dupe x2
{
  if (a0 == 0x56)
    return;
  sub_255f5(a1 + 8,1);
  if (a0 != 0x31) { // branch-flip
    if (a0 == 0x32)
      sub_255f5(a1 + 0x18,4);
  }
  else {
    sub_255f5(a1 + 0x18,2);
  }
  if (!dat_9f936)
    return;
  sub_25627(0x9fa28);
  return;
}


// Function: sub_257b8 @ 0x257b8
void sub_257b8(code *a0)
{
  unsigned int v1;
  char v2;
  unsigned long v3; // rax
  char v4 [24];
  unsigned int v5; // stack - 0x58
  
  v5 = 0;
  sub_254c9();
  sub_2d3b1();
  sub_f694(0);
  do {
    v1 = v5;
    sub_39d1c(0x9fa20);
    v5 = sub_25ecf(0x9f460,0x9fa20,0);
    switch(v5) {
      case 0:
      case 2:
        abort(); // no-return
      case 1:
        sub_26747(dat_9f460,0x9fa20,0x9f468,1);
        v2 = sub_2d88a(dat_9fa28);
        if (v2 != '\x01') { // branch-flip
label_25932:
          v2 = *(char *)(dat_9f460 + 0x9c);
          if (v2 != 'V') {
            if (v2 <= 'V') {
              if (v2 != '5') { // branch-flip
                if (v2 == 'M') goto label_259bf;
              }
              else if (dat_9f978) {
                if (dat_9fdc8)
                  (*dat_9fdc8)();
                v3 = sub_5e548(dat_9fa28);
                error(0,0,gettext("%s: Omitting"),v3);
              }
            }
            sub_28891();
            break;
          }
        }
        else {
          if (0 <= dat_9f908) {
            dat_9fb18 = sub_279d7(dat_9f460 + 0x88,0xc);
            dat_9fb20 = 0;
            sub_5ff90(dat_9fb18,0,dat_9f900,dat_9f908);
            if (dat_3 < '\0') goto label_25932;
          }
          v2 = sub_17c43(dat_9fa28,dat_9fbb8);
          if (v2) goto label_25932;
        }
label_259bf:
        sub_25733((int4)*(char *)(dat_9f460 + 0x9c),0x9fa20);
        (*a0)();
        break;
      case 3:
        if (dat_9f88c) {
          v3 = sub_54e5a(sub_bacd(),v4);
          fprintf(dat_9efd0,gettext("block %s: ** Block of NULs **\n"),v3);
        }
        sub_bba7(dat_9f460);
        if (dat_9f8bd != '\x01') {
          if ((sub_25ecf(0x9f460,0x9fa20,0) != 3) && (dat_9ec84 & 1)) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v3 = sub_54e5a(sub_bacd(),v4);
            error(0,0,gettext("A lone zero block at %s"),v3);
          }
          sub_d046(); // return-dupe
          sub_2dbba();
          return;
        }
        v5 = v1;
        break;
      case 4:
        if (dat_9f88c) {
          v3 = sub_54e5a(sub_bacd(),v4);
          fprintf(dat_9efd0,gettext("block %s: ** End of File **\n"),v3);
        }
        sub_d046();
        sub_2dbba();
        return;
      case 5:
        sub_bba7(dat_9f460);
        switch(v1) {
          case 0:
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("This does not look like a tar archive"));
            dat_9fdd0 = 2;
            goto label_25bde;
          case 1:
          case 3:
            if (dat_9f88c) {
              v3 = sub_54e5a((sub_bacd() - dat_9f480) - dat_9f488,v4);
              fprintf(dat_9efd0,gettext("block %s: "),v3);
            }
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Skipping to next header"));
            dat_9fdd0 = 2;
            goto label_25ca5;
          case 4:
          case 5:
            break;
          case 2:
            abort(); // no-return
          
        }
        break;
      default:
        goto label_25cce;
      
    }
  } while (sub_2da15(0x9fa20) != '\x01');
  sub_d046();
  sub_2dbba();
  return;
}


// Function: sub_25cf3 @ 0x25cf3
void sub_25cf3(void)
{
  unsigned long v1; // rax
  
  v1 = sub_bacd();
  if (dat_9f9ac)
    sub_2864c(0x9fa20,dat_9f460,v1);
  if ((dat_9f8be) && (3 <= dat_9f9ac)) {
    if (sub_2490b(0x9fa20)) {
      v1 = sub_20bfe(dat_9fbb0);
      sub_253a2(dat_9fbb0,v1);
    }
  }
  sub_28891();
  return;
}


// Function: sub_25d84 @ 0x25d84
unsigned long sub_25d84(uint1 *a0,char a1) // return-dupe x3
{
  uint1 *v1;
  int8 v2;
  int4 v3; // eax
  int4 v4; // stack - 0x28
  int4 v5; // stack - 0x24
  int8 v6; // stack - 0x18
  uint1 *v7; // stack - 0x10
  
  v4 = 0;
  v5 = 0;
  v6 = 0x200;
  v7 = a0;
  while (v2 = v6 + -1, v6) {
    v4 += (uint4)*v7;
    v1 = &v7[1];
    v5 += (char)*v7;
    v6 = v2;
    v7 = v1;
  }
  if (!v4)
    return 3;
  v6 = 8;
  while (v2 = v6 + -1, v6) {
    v4 -= (uint4)a0[v6 + 0x93];
    v5 -= (char)a0[v6 + 0x93];
    v6 = v2;
  }
  v3 = sub_26c90(&a0[0x94],8,0,0,0x7fffffff,1,a1);
  if (v3 <= -1)
    return 5;
  if ((v4 + 0x100 != v3) && (v5 + 0x100 != v3))
    return 5;
  return 1;
}


// Function: sub_25ecf @ 0x25ecf
int4 sub_25ecf(unsigned long *a0,int8 a1,int4 a2)
{
  char *v1;
  char v10 [32];
  char v11 [100];
  unsigned long *v12;
  int4 v13; // stack - 0x1b4
  unsigned long *v14; // stack - 0x1b0
  uint8 v15; // stack - 0x1a8
  uint8 v16; // stack - 0x1a0
  unsigned long *v17; // stack - 0x198
  unsigned long *v18; // stack - 0x190
  uint8 v19; // stack - 0x188
  unsigned long *v2;
  uint8 v20; // stack - 0x180
  char *v21; // stack - 0x178
  char *v22; // stack - 0x170
  char v23; // stack - 0xb4
  char v24; // stack - 0x7d
  unsigned long *v3;
  unsigned long v4;
  unsigned long *v5;
  uint8 v6; // rax
  void *v7; // rax
  uint8 v8;
  int8 v9; // rdx
  
  v17 = NULL;
  v18 = NULL;
  v19 = 0;
  v20 = 0;
  while( true ) {
    v5 = (unsigned long *)sub_bb4b();
    *a0 = v5;
    if (!v5) {
      v13 = 4;
      free(v17); // return-dupe
      free(v18);
      return v13;
    }
    v13 = sub_25d84(v5,0);
    if (v13 != 1) {
      free(v17);
      free(v18);
      return v13;
    }
    if (*(char *)((int8)v5 + 0x9c) != '1') { // branch-flip
      *(unsigned long *)(a1 + 0x88) = sub_2798b((int8)v5 + 0x7c,0xc);
      if (*(int8 *)(a1 + 0x88) <= -1) {
        v13 = 5;
        free(v17);
        free(v18);
        return v13;
      }
    }
    else {
      *(unsigned long *)(a1 + 0x88) = 0;
    }
    if ((((*(char *)((int8)v5 + 0x9c) != 'L') && (*(char *)((int8)v5 + 0x9c) != 'K')) && (*(char *)((int8)v5 + 0x9c) != 'x')) && ((*(char *)((int8)v5 + 0x9c) != 'g' && (*(char *)((int8)v5 + 0x9c) != 'X')))) break;
    if (a2 == 1) {
      v13 = 2;
      free(v17);
      free(v18);
      return v13;
    }
    if ((*(char *)((int8)v5 + 0x9c) != 'L') && (*(char *)((int8)v5 + 0x9c) != 'K')) {
      if ((*(char *)((int8)v5 + 0x9c) != 'x') && (*(char *)((int8)v5 + 0x9c) != 'X')) {
        if (*(char *)((int8)v5 + 0x9c) == 'g') {
          if (!dat_9f490)
            dat_9f490 = (void *)sub_60b5b(0x200);
          memcpy(dat_9f490,v5,0x200);
          memset(v10,0,0x20);
          sub_1e28e(v10,v5,sub_2798b((int8)v5 + 0x7c,0xc));
          sub_1e0e7(v10);
          sub_1e785(v10);
          if (a2 == 2) {
            v13 = 2;
            free(v17);
            free(v18);
            return v13;
          }
        }
      }
      else {
        sub_1e28e(a1 + 0x168,v5,sub_2798b((int8)v5 + 0x7c,0xc));
      }
    }
    else {
      v8 = *(uint8 *)(a1 + 0x88);
      v6 = (uint8)((uint4)v8 & 0x1ff);
      v15 = v8 + 0x200;
      if (v6)
        v15 = (v15 - v6) + 0x200;
      if ((*(uint8 *)(a1 + 0x88) != v8) || (v15 < v8))
        sub_1783c(); // no-return
      v14 = (unsigned long *)sub_60b5b(v15 + 1);
      if (*(char *)((int8)v5 + 0x9c) != 'L') { // branch-flip
        free(v18);
        v18 = v14;
        v20 = v15 >> 9;
      }
      else {
        free(v17);
        v17 = v14;
        v19 = v15 >> 9;
      }
      sub_bba7(v5);
      *v14 = *v5;
      v14[0x3f] = v5[0x3f];
      v9 = (int8)v14 - (int8)(unsigned long *)((uint8)&v14[1] & 0xfffffffffffffff8);
      v8 = (uint8)((int4)v9 + 0x200U >> 3);
      v5 = (unsigned long *)((int8)v5 - v9);
      v12 = (unsigned long *)((uint8)&v14[1] & 0xfffffffffffffff8);
      while (v8) {
        v3 = &v12[1];
        v2 = &v5[1];
        *v12 = *v5;
        v8 -= 1;
        v5 = v2;
        v12 = v3;
      }
      v14 = &v14[0x40];
      for (v15 = v15 - 0x200; v15; v15 = v15 - v16) {
        v7 = (void *)sub_bb4b();
        if (!v7) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Unexpected EOF in archive"));
          dat_9fdd0 = 2;
          break;
        }
        v16 = sub_bbf5(v7);
        if (v15 < v16)
          v16 = v15;
        memcpy(v14,v7,v16);
        v14 = (unsigned long *)((int8)v14 + v16);
        sub_bba7((int8)v7 + (v16 - 1));
      }
      *(char *)v14 = 0;
    }
  }
  free(dat_9f470);
  if (v17) { // branch-flip
    v21 = (char *)&v17[0x40];
    dat_9f470 = v17;
    dat_9f480 = v19;
    v17 = NULL;
  }
  else {
    v22 = v11;
    if ((*(char *)((int8)v5 + 0x159)) && (!strcmp((char *)((int8)v5 + 0x101),"ustar"))) {
      memcpy(v22,(void *)((int8)v5 + 0x159),0x9b);
      v24 = 0;
      v8 = strlen(v22);
      v1 = &v22[v8];
      v22 = &v11[v8 + 1];
      *v1 = '/';
    }
    memcpy(v22,v5,100);
    v22[100] = '\0';
    v21 = v11;
    dat_9f470 = NULL;
    dat_9f480 = 0;
  }
  sub_29547(a1,v21);
  sub_29547(a1 + 8,v21);
  v4 = *(unsigned long *)(a1 + 8);
  *(char *)(a1 + 0x10) = sub_4e2e5(v4);
  free(dat_9f478);
  if (v18) { // branch-flip
    v21 = (char *)&v18[0x40];
    dat_9f478 = v18;
    dat_9f488 = v20;
    v18 = NULL;
  }
  else {
    memcpy(v11,(void *)((int8)v5 + 0x9d),100);
    v23 = 0;
    v21 = v11;
    dat_9f478 = NULL;
    dat_9f488 = 0;
  }
  sub_29547(a1 + 0x18,v21);
  free(v17);
  free(v18);
  return v13;
}


// Function: sub_26747 @ 0x26747
void sub_26747(int8 a0,int8 a1,int4 *a2,int4 a3) // return-dupe x2, ternary x2
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  int8 v3; // rax
  char v4; // stack - 0x29
  int4 v5; // stack - 0x28
  unsigned int v6; // stack - 0x24
  
  v6 = sub_2790d(a0 + 100,8,&v4);
  if (strcmp((char *)(a0 + 0x101),"ustar")) { // branch-flip
    if (strcmp((char *)(a0 + 0x101),"ustar  ")) // branch-flip
      v5 = 1;
    else {
      v5 = (v4) ? 2 : 6; // branch-flip
    }
  }
  else if (((((*(char *)(a0 + 0x1db)) || (*(char *)(a0 + 0x1dc) <= '/')) || ('8' <= *(char *)(a0 + 0x1dc))) || ((*(char *)(a0 + 0x1e7) != ' ' || (*(char *)(a0 + 0x1e8) <= '/')))) || (('8' <= *(char *)(a0 + 0x1e8) || (*(char *)(a0 + 499) != ' ')))) {
    v5 = (*(int8 *)(a1 + 0x170)) ? 4 : 3; // branch-flip
  }
  else {
    v5 = 5;
  }
  *a2 = v5;
  *(unsigned int *)(a1 + 0x70) = v6;
  *(unsigned long *)(a1 + 0xf8) = sub_279d7(a0 + 0x88,0xc);
  *(unsigned long *)(a1 + 0x100) = 0;
  if (*(char *)(a0 + 0x109)) // branch-flip
    v3 = a0 + 0x109;
  else {
    v3 = 0;
  }
  sub_29591(a1 + 0x20,v3,0x20);
  if (*(char *)(a0 + 0x129)) // branch-flip
    v3 = a0 + 0x129;
  else {
    v3 = 0;
  }
  sub_29591(a1 + 0x28,v3,0x20);
  sub_1d4f4(a1);
  if ((v5 != 2) || (!dat_9f8be)) {
    if (v5 != 5) { // branch-flip
      *(unsigned long *)(a1 + 0x108) = dat_9f9e0;
      *(unsigned long *)(a1 + 0x110) = dat_9f9e8;
      *(unsigned long *)(a1 + 0xe8) = *(unsigned long *)(a1 + 0x108);
      *(unsigned long *)(a1 + 0xf0) = *(unsigned long *)(a1 + 0x110);
    }
    else {
      *(unsigned long *)(a1 + 0xe8) = sub_279d7(a0 + 0x1dc,0xc);
      *(unsigned long *)(a1 + 0x108) = sub_279d7(a0 + 0x1e8,0xc);
      *(unsigned long *)(a1 + 0x110) = 0;
      *(unsigned long *)(a1 + 0xf0) = *(unsigned long *)(a1 + 0x110);
    }
  }
  else {
    *(unsigned long *)(a1 + 0xe8) = sub_279d7(a0 + 0x159,0xc);
    *(unsigned long *)(a1 + 0x108) = sub_279d7(a0 + 0x165,0xc);
    *(unsigned long *)(a1 + 0x110) = 0;
    *(unsigned long *)(a1 + 0xf0) = *(unsigned long *)(a1 + 0x110);
  }
  if (v5 != 1) { // branch-flip
    if (a3) {
      if (((dat_9f934) || (!*(char *)(a0 + 0x109))) || (!sub_2c173(a0 + 0x109,a1 + 0x74)))
        *(unsigned int *)(a1 + 0x74) = sub_27a2b(a0 + 0x6c,8);
      if (((dat_9f934) || (!*(char *)(a0 + 0x129))) || (!sub_2c261(a0 + 0x129,a1 + 0x78)))
        *(unsigned int *)(a1 + 0x78) = sub_27831(a0 + 0x74,8);
    }
    if (2 <= (uint4)((int4)*(char *)(a0 + 0x9c) - 0x33U)) // branch-flip
      *(unsigned long *)(a1 + 0x80) = 0;
    else {
      v1 = sub_278c3(a0 + 0x151,8);
      v2 = sub_27879(a0 + 0x149,8);
      *(unsigned long *)(a1 + 0x80) = gnu_dev_makedev(v2,v1);
    }
  }
  else {
    *(unsigned int *)(a1 + 0x74) = sub_27a2b(a0 + 0x6c,8);
    *(unsigned int *)(a1 + 0x78) = sub_27831(a0 + 0x74,8);
    *(unsigned long *)(a1 + 0x80) = 0;
  }
  sub_1df5f(a1);
  if (!sub_301de(a1)) {
    *(char *)(a1 + 0x120) = 0;
    if ((((dat_9f468 != 6) && (dat_9f468 != 2)) || (*(char *)(dat_9f460 + 0x9c) != 'D')) && (!*(int8 *)(a1 + 400)))
      return;
    *(char *)(a1 + 0x188) = 1;
    return;
  }
  sub_30241(a1);
  *(char *)(a1 + 0x120) = 1;
  return;
}


// Function: sub_26c90 @ 0x26c90
unsigned long sub_26c90(uint1 *a0,uint8 a1,int8 a2,int8 a3,uint8 a4,char a5,unsigned int a6)
{
  uint1 *v1;
  char v10 [4]; // stack - 0x4d8
  int8 v11;
  char v12 [31];
  char v13 [999];
  char v14 [33];
  char *v15; // rsp
  char *v16; // rsp
  char *v17; // rsp
  char *v18; // rsp
  char *v19; // rsp
  int4 v2;
  char *v20; // rsp
  bool v21;
  char v22; // stack - 0x4d4
  uint8 v23; // stack - 0x4d0
  int8 v24; // stack - 0x4c8
  int8 v25; // stack - 0x4c0
  uint8 v26; // stack - 0x4b8
  uint1 *v27; // stack - 0x4b0
  bool v28; // stack - 0x4a6
  bool v29; // stack - 0x4a5
  int4 v3;
  uint4 v30; // stack - 0x4a4
  uint4 v31; // stack - 0x4a0
  int4 v32; // stack - 0x49c
  uint8 v33; // stack - 0x498
  uint1 *v34; // stack - 0x490
  uint1 *v35; // stack - 0x488
  char *v36; // stack - 0x480
  char *v37; // stack - 0x478
  int8 v38; // stack - 0x470
  uint8 v39; // stack - 0x468
  int8 v4;
  uint8 v40; // stack - 0x460
  void *v41; // stack - 0x458
  uint1 *v42; // stack - 0x450
  uint1 *v5;
  int8 *v6; // rax
  uint8 v7;
  char *v8; // rax
  unsigned long v9; // rax
  
  v15 = v10;
  v27 = a0;
  v26 = a1;
  v25 = a2;
  v24 = a3;
  v23 = a4;
  v22 = a5;
  v10[0] = (char)a6;
  v38 = a3;
  v39 = -a3;
  v35 = &a0[a1];
  v28 = 0;
  v34 = &a0[*a0 == 0];
  while( true ) {
    if (v34 == v35) {
      if ((v25) && (v10[0] != '\x01')) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Blanks in header where numeric %s value expected"),v25);
        dat_9fdd0 = 2;
      }
      return 0xffffffffffffffff;
    }
    v6 = __ctype_b_loc();
    v1 = v34;
    if (!(*(uint2 *)((uint8)*v34 * 2 + *v6) & 0x2000)) break;
    v34 = &v34[1];
  }
  v33 = 0;
  if (8 <= (uint4)((int4)(char)*v34 - 0x30U)) { // branch-flip
    if (!v22) {
      if ((*v34 != 0x2d) && (*v34 != 0x2b)) {
        if ((*v34 == 0x80) || (*v34 == 0xff)) {
          v31 = (int4)(char)*v34 & 0x40;
          v40 = (int8)(int4)-v31 << 0x32;
          v1 = &v34[1];
          v33 = (uint8)(int4)(((int4)(char)*v34 & 0x3fU) - v31);
          v34 = v1;
          while( true ) {
            v1 = v34;
            v34 = &v34[1];
            v33 = (uint8)*v1 + v33 * 0x100;
            if (v34 == v35) break;
            if (v33 != (v33 & 0xffffffffffffff | (int8)(int4)-v31 << 0x32)) {
              if ((v25) && (v10[0] != '\x01')) {
                if (dat_9fdc8)
                  (*dat_9fdc8)();
                error(0,0,gettext("Archive base-256 value is out of %s range"),v25);
                dat_9fdd0 = 2;
              }
              return 0xffffffffffffffff;
            }
          }
          v28 = v31 != 0;
          if (v31)
            v33 = -v33;
        }
      }
      else {
        if ((v10[0] != '\x01') && (dat_9f5a1 != '\x01')) {
          dat_9f5a1 = '\x01';
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Archive contains obsolescent base-64 headers"));
        }
        v1 = v34;
        v34 = &v34[1];
        v28 = *v1 == 0x2d;
        for (; v34 != v35; v34 = &v34[1]) {
          v32 = (int4)*(char *)((int8)(int4)(uint4)*v34 + 0x9f4a0);
          if (0x40 <= v32) break;
          if (v33 != (v33 & 0x3ffffffffffffff)) {
            v7 = ((v26 + 0x18) / 0x10) * 0x10;
            for (; v15 != &v10[-(v7 & 0xfffffffffffff000)]; v15 = &v15[-0x1000]) {
              *(unsigned long *)&v15[-8] = *(unsigned long *)&v15[-8];
            }
            v4 = -(uint8)((uint4)v7 & 0xfff);
            if (v7 & 0xfff)
              *(unsigned long *)&v15[((uint8)((uint4)v7 & 0xfff) - 8) + v4] = *(unsigned long *)&v15[((uint8)((uint4)v7 & 0xfff) - 8) + v4];
            v41 = (void *)((uint8)&v15[v4 + 0xf] & 0xfffffffffffffff0);
            v16 = &v15[v4 + -8];
            v16[0] = '\xb0';
            v16[1] = 'r';
            v16[2] = '\x02';
            v16[3] = '\0';
            v16[4] = '\0';
            v16[5] = '\0';
            v16[6] = '\0';
            v16[7] = '\0';
            memcpy((void *)((uint8)&v15[v4 + 0xf] & 0xfffffffffffffff0),v27,v26,v15[v4 + -8]);
            *(char *)(v26 + (int8)v41) = 0;
            if ((v25) && (v10[0] != '\x01')) {
              if (dat_9fdc8) {
                v17 = &v15[v4 + -8];
                v17[0] = '\xf1';
                v17[1] = 'r';
                v17[2] = '\x02';
                v17[3] = '\0';
                v17[4] = '\0';
                v17[5] = '\0';
                v17[6] = '\0';
                v17[7] = '\0';
                (*dat_9fdc8)();
              }
              v18 = &v15[v4 + -8];
              v18[0] = '\0';
              v18[1] = 's';
              v18[2] = '\x02';
              v18[3] = '\0';
              v18[4] = '\0';
              v18[5] = '\0';
              v18[6] = '\0';
              v18[7] = '\0';
              v9 = sub_5e84e(v41);
              v19 = &v15[v4 + -8];
              builtin_strncpy(&v15[v4 + -8],"\x12s\x02",4);
              v19[4] = '\0';
              v19[5] = '\0';
              v19[6] = '\0';
              v19[7] = '\0';
              v8 = gettext("Archive signed base-64 string %s is out of %s range",v15[v4 + -8]);
              v20 = &v15[v4 + -8];
              builtin_strncpy(&v15[v4 + -8],"6s\x02",4);
              v20[4] = '\0';
              v20[5] = '\0';
              v20[6] = '\0';
              v20[7] = '\0';
              error(0,0,v8,v9,v25);
              dat_9fdd0 = 2;
            }
            return 0xffffffffffffffff;
          }
          v33 = (int8)v32 | v33 << 6;
        }
      }
    }
  }
  else {
    v42 = v34;
    v29 = 0;
    while( true ) {
      v5 = v34;
      v34 = &v34[1];
      v33 += (int8)((char)*v5 + -0x30);
      if ((v34 == v35) || (7 < (uint4)((int4)(char)*v34 - 0x30U))) break;
      v29 = v33 != (v33 & 0x1fffffffffffffff) || v29;
      v33 *= 8;
    }
    if (((v29) || (v23 < v33)) && (('2' <= (char)*v1 && (v25)))) {
      v30 = (int4)(char)*v1 - 0x30U | 4;
      v21 = 0;
      v33 = 0;
      v34 = v1;
      while( true ) {
        v33 += (int8)(int4)(7 - v30);
        v34 = &v34[1];
        if ((v34 == v35) || (7 < (uint4)((int4)(char)*v34 - 0x30U))) break;
        v30 = (int4)(char)*v34 - 0x30;
        v21 = v33 != (v33 & 0x1fffffffffffffff) || v21;
        v33 *= 8;
      }
      v33 += 1;
      v29 = !v33 || v21;
      if ((v33 && !v21) && (v33 <= v39)) {
        if (v10[0] != '\x01') {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v3 = (int4)v42;
          v2 = (int4)v34;
          error(0,0,gettext("Archive octal value %.*s is out of %s range; assuming two\'s complement"),(uint8)(uint4)(v2 - v3),v42,v25);
        }
        v28 = 1;
      }
    }
    if (v29) {
      if ((v25) && (v10[0] != '\x01')) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v3 = (int4)v42;
        v2 = (int4)v34;
        error(0,0,gettext("Archive octal value %.*s is out of %s range"),(uint8)(uint4)(v2 - v3),v42,v25);
        dat_9fdd0 = 2;
      }
      return 0xffffffffffffffff;
    }
  }
  if (((v34 != v35) && (*v34)) && (!(*(uint2 *)((uint8)*v34 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000))) {
    if (v25) {
      if (!dat_9f5a8) {
        dat_9f5a8 = sub_5c92f(0);
        sub_5c9a2(dat_9f5a8,8);
      }
      for (; (v27 != v35 && (!v35[-1])); v35 = &v35[-1]) {
      }
      sub_5dd44(&v11,1000,v27,(int8)v35 - (int8)v27,dat_9f5a8);
      if (v10[0] != '\x01') {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Archive contains %.*s where numeric %s value expected"),1000,&v11,v25);
        dat_9fdd0 = 2;
      }
    }
    return 0xffffffffffffffff;
  }
  if (v28) // branch-flip
    v7 = v39;
  else {
    v7 = v23;
  }
  if (v33 <= v7) {
    if (v28) // branch-flip
      v7 = -v33;
    else {
      v7 = v33;
    }
    return sub_294ad(v7);
  }
  if ((v25) && (v10[0] != '\x01')) {
    v36 = (char *)sub_54e5a(v39,v12);
    v37 = (char *)sub_54e5a(v33,v13);
    if (v28) {
      v37 = &v37[-1];
      *v37 = 0x2d;
    }
    if (v39) {
      v36 = &v36[-1];
      *v36 = 0x2d;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v9 = sub_54e5a(v23,v14);
    error(0,0,gettext("Archive value %s is out of %s range %s..%s"),v37,v25,v36,v9);
    dat_9fdd0 = 2;
  }
  return 0xffffffffffffffff;
}


// Function: sub_27831 @ 0x27831
void sub_27831(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"gid_t",0,0xffffffff,0,0);
  return;
}


// Function: sub_27879 @ 0x27879
void sub_27879(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"major_t",0xffffffff80000000,0x7fffffff,0,0);
  return;
}


// Function: sub_278c3 @ 0x278c3
void sub_278c3(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"minor_t",0xffffffff80000000,0x7fffffff,0,0);
  return;
}


// Function: sub_2790d @ 0x2790d
uint4 sub_2790d(unsigned long a0,unsigned long a1,unsigned long a2)
{
  uint8 v1; // rax
  
  v1 = sub_26c90(a0,a1,"mode_t",0x8000000000000000,0xffffffffffffffff,0,0);
  *(bool *)a2 = (v1 & 0xfffffffffffff000) != 0;
  return (uint4)v1 & 0xfff;
}


// Function: sub_2798b @ 0x2798b
void sub_2798b(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"off_t",0,0x7fffffffffffffff,0,0);
  return;
}


// Function: sub_279d7 @ 0x279d7
void sub_279d7(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"time_t",0x8000000000000000,0x7fffffffffffffff,0,0);
  return;
}


// Function: sub_27a2b @ 0x27a2b
void sub_27a2b(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"uid_t",0,0xffffffff,0,0);
  return;
}


// Function: sub_27a73 @ 0x27a73
void sub_27a73(unsigned long a0,unsigned long a1)
{
  sub_26c90(a0,a1,"uintmax_t",0,0xffffffffffffffff,0,0);
  return;
}


// Function: sub_27abc @ 0x27abc
char * sub_27abc(int8 a0,int4 a1,char a2) // early-return, return-dupe, ternary
{
  int8 v1;
  char *v2; // rax
  int8 v3; // stack - 0x38
  int4 v4; // stack - 0x3c
  char *v5; // stack - 0x30
  void *v6; // stack - 0x28
  
  v4 = a1;
  v3 = a0;
  if ((a0 < 0) && (a1)) {
    v3 = a0 + 1;
    v4 = 1000000000 - a1;
  }
  if (dat_9f879) // branch-flip
    v6 = (void *)gmtime(&v3);
  else {
    v6 = localtime(&v3);
  }
  if (!v6) {
    if (0 <= a0) // branch-flip
      v1 = v3;
    else {
      v1 = -v3;
    }
    v5 = (char *)sub_54e5a(v1,0x9f5c6);
    if (a0 < 0) {
      v5 = &v5[-1];
      *v5 = 0x2d;
    }
    while( true ) {
      v2 = (a2) ? (char *)0x9f5e1 : (char *)0x9f5d4; // branch-flip
      if (v5 <= v2) break;
      v5 = &v5[-1];
      *v5 = 0x20;
    }
    if (a2)
      sub_29e23(v4,0x9f5da);
    return v5;
  }
  if (a2) {
    strftime((char *)0x9f5c0,0x25,"%Y-%m-%d %H:%M:%S",v6);
    sub_29e23(v4,strlen((char *)0x9f5c0) + 0x9f5c0);
    return (char *)0x9f5c0;
  }
  strftime((char *)0x9f5c0,0x25,"%Y-%m-%d %H:%M",v6);
  return (char *)0x9f5c0;
}


// Function: sub_27c80 @ 0x27c80
void sub_27c80(int8 *a0,int8 a1,int8 a2) // ternary
{
  int8 v1;
  char v10 [56];
  char v11 [9];
  char v12 [2];
  char v13 [32];
  char v14 [32];
  char v15 [32];
  char v16; // stack - 0xc4
  char v17; // stack - 0xc6
  int8 v18; // stack - 0x110
  int8 v19; // stack - 0xe8
  void *v2;
  char *v20; // stack - 0xe0
  char *v21; // stack - 0xd8
  char v22; // stack - 0xc5
  uint4 v3; // eax
  unsigned int v4;
  int4 v5; // eax
  int4 v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  uint8 v9; // rax
  
  if (dat_9fc30) { // branch-flip
    if (a0[1]) // branch-flip
      v19 = a0[1];
    else {
      v19 = *a0;
    }
  }
  else if (*a0) // branch-flip
    v19 = *a0;
  else {
    v19 = a0[1];
  }
  if (dat_9f88c) {
    v18 = a2;
    if (a2 <= -1)
      v18 = sub_bacd();
    v7 = sub_54e5a((v18 - dat_9f480) - dat_9f488,v10);
    fprintf(dat_9efd0,gettext("block %s: "),v7);
  }
  v2 = dat_9efd0;
  if (2 <= dat_9f9ac) { // branch-flip
    v16 = 0x3f;
    switch(*(char *)(a1 + 0x9c)) {
      case 0:
      case 0x30:
      case 0x53:
        v16 = ((char)a0[2]) ? 100 : 0x2d; // branch-flip
        break;
      case 0x31:
        v16 = 0x68;
        break;
      case 0x32:
        v16 = 0x6c;
        break;
      case 0x33:
        v16 = 99;
        break;
      case 0x34:
        v16 = 0x62;
        break;
      case 0x35:
        v16 = 100;
        break;
      case 0x36:
        v16 = 0x70;
        break;
      case 0x37:
        v16 = 0x43;
        break;
      case 0x44:
        v16 = 100;
        break;
      case 0x4b:
      case 0x4c:
        v16 = 0x4c;
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Unexpected long name header"));
        dat_9fdd0 = 2;
        break;
      case 0x4d:
        v16 = 0x4d;
        break;
      case 0x56:
        dat_9f5a0 = 1;
        v16 = 0x56;
      
    }
    sub_3dd6f((int4)a0[0xe],v11);
    sub_3d9ec(a0,v12);
    v8 = (char *)sub_27abc(a0[0x1f],a0[0x20],dat_9f87a);
    v3 = (uint4)strlen(v8);
    if ((int4)dat_9c264 < (int4)v3)
      dat_9c264 = v3;
    if ((((a0[4]) && (*(char *)a0[4])) && (dat_9f468 != 1)) && (dat_9f934 != '\x01'))
      v20 = (char *)a0[4];
    else {
      v4 = *(unsigned int *)((int8)a0 + 0x74);
      v20 = (char *)sub_54e5a(v4,v13);
    }
    if (((a0[5]) && (*(char *)a0[5])) && ((dat_9f468 != 1 && (dat_9f934 != '\x01'))))
      v21 = (char *)a0[5];
    else {
      v1 = a0[0xf];
      v21 = (char *)sub_54e5a((int4)v1,v14);
    }
    if (2 <= (uint4)((int4)*(char *)(a1 + 0x9c) - 0x33U)) { // branch-flip
      v1 = a0[0x11];
      strcpy(v10,(char *)sub_54e5a(v1,v15));
    }
    else {
      v4 = gnu_dev_major(a0[0x10]);
      strcpy(v10,(char *)sub_54e5a(v4,v15));
      strcat(v10,",");
      v4 = gnu_dev_minor(a0[0x10]);
      strcat(v10,(char *)sub_54e5a(v4,v15));
    }
    v5 = (int4)strlen(v10);
    v9 = strlen(v20);
    v6 = v5 + (int4)v9 + (int4)strlen(v21) + 2;
    if (dat_9c260 < v6)
      dat_9c260 = v6;
    fprintf(dat_9efd0,"%s %s/%s %*s %-*s",&v16,v20,v21,(uint8)(uint4)((dat_9c260 - v6) + v5),v10,(uint8)dat_9c264,v8);
    fprintf(dat_9efd0," %s",sub_5e2eb(v19));
    if ((dat_9fc30) && ((char)a0[2]))
      fputc_unlocked(0x2f,dat_9efd0);
    switch(*(char *)(a1 + 0x9c)) {
      case 0:
      case 0x30:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x44:
      case 0x53:
        putc_unlocked(10,dat_9efd0);
        break;
      default:
        v17 = *(char *)(a1 + 0x9c);
        v22 = 0;
        v7 = sub_5e84e(&v17);
        fprintf(dat_9efd0,gettext(" unknown file type %s\n"),v7);
        break;
      case 0x31:
        v7 = sub_5e2eb(a0[3]);
        fprintf(dat_9efd0,gettext(" link to %s\n"),v7);
        break;
      case 0x32:
        v1 = a0[3];
        fprintf(dat_9efd0," -> %s\n",sub_5e2eb(v1));
        break;
      case 0x4b:
        fprintf(dat_9efd0,gettext("--Long Link--\n"));
        break;
      case 0x4c:
        fprintf(dat_9efd0,gettext("--Long Name--\n"));
        break;
      case 0x4d:
        v7 = sub_27a73(a1 + 0x171,0xc);
        strcpy(v10,(char *)sub_54e5a(v7,v15));
        fprintf(dat_9efd0,gettext("--Continued at byte %s--\n"),v10);
        break;
      case 0x56:
        fprintf(dat_9efd0,gettext("--Volume Header--\n"));
      
    }
  }
  else {
    fputs_unlocked((char *)sub_5e2eb(v19),v2);
    if ((dat_9fc30) && ((char)a0[2]))
      fputc_unlocked(0x2f,dat_9efd0);
    fputc_unlocked(10,dat_9efd0);
  }
  fflush_unlocked(dat_9efd0);
  sub_3db33(a0);
  return;
}


// Function: sub_28528 @ 0x28528
void sub_28528(void)
{
  char v1 [136];
  char v2 [20];
  char v3 [8];
  char v4 [440];
  char v5 [4];
  char v6; // stack - 0x17c
  
  memset(v1,0,0x200);
  v6 = 0x56;
  if (dat_9f490)
    memcpy(v2,(void *)(dat_9f490 + 0x88),0xc);
  sub_39c55(v3);
  sub_29547(v4,0x8289c);
  sub_26747(v1,v3,v5,0);
  sub_29547(v4,dat_9efe0);
  sub_27c80(v3,v1,0);
  sub_39d1c(v3);
  return;
}


// Function: sub_2864c @ 0x2864c
void sub_2864c(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if (((dat_9f468 == 4) && (dat_9f5a0 != '\x01')) && (dat_9efe0)) {
    sub_28528();
    dat_9f5a0 = '\x01';
  }
  sub_27c80(a0,a1,a2);
  return;
}


// Function: sub_286af @ 0x286af
void sub_286af(unsigned long a0,unsigned long a1,unsigned int a2) // return-dupe
{
  unsigned long v1; // rax
  char *v2; // rax
  char v3 [10];
  char v4 [24];
  char v5; // stack - 0x43
  
  if (dat_9f9ac <= 1)
    return;
  v5 = 100;
  sub_3dd6f(a2,v3);
  if (dat_9f88c) {
    v1 = sub_54e5a(sub_bacd(),v4);
    fprintf(dat_9efd0,gettext("block %s: "),v1);
  }
  v1 = sub_5e2eb(a0);
  v2 = gettext("Creating directory:");
  fprintf(dat_9efd0,"%s %*s %s\n",&v5,(uint8)(uint4)(dat_9c260 + 1 + dat_9c264),v2,v1);
  return;
}


// Function: sub_287c1 @ 0x287c1
void sub_287c1(int8 a0)
{
  int8 v1; // rax
  int8 v2; // stack - 0x20
  
  v2 = a0;
  if (dat_9fc18) {
    v1 = sub_ce79(a0);
    if (0 <= v1) // branch-flip
      v2 = a0 + v1 * -0x200;
    else {
      dat_9fc18 = '\0';
    }
  }
  sub_ad86(v2);
  while( true ) {
    if (v2 <= 0)
      return;
    v1 = sub_bb4b();
    if (!v1) break;
    sub_bba7(v1);
    v2 -= 0x200;
    sub_ad86(v2);
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Unexpected EOF in archive"));
  sub_177f4(); // no-return
}


// Function: sub_28891 @ 0x28891
void sub_28891(void) // return-dupe
{
  char v1;
  
  if (dat_9fba9 == '\x01')
    return;
  v1 = *(char *)(dat_9f460 + 0x9c);
  sub_bba7(dat_9f460);
  sub_ad2b(0x9fa20);
  if (dat_9fb40) // branch-flip
    sub_303bd(0x9fa20);
  else if (v1 != '5')
    sub_287c1(dat_9faa8);
  sub_ad66();
  return;
}


// Function: sub_28913 @ 0x28913
void sub_28913(void)
{
  char v1; // al
  void *v2; // rax
  
  sub_254c9();
  sub_2d3b1();
  sub_f694(0);
  if (sub_25ecf(0x9f460,0x9fa20,0) == 1) {
    sub_26747(dat_9f460,0x9fa20,0x9f468,0);
    if (*(char *)(dat_9f460 + 0x9c) == 'V')
      sub_29591(0x9efe0,dat_9f460,100);
    if (dat_9efe0) {
      if (dat_9f9ac)
        sub_28528();
      v1 = sub_2d88a(dat_9efe0);
      if ((v1 != '\x01') && (dat_9f8fc)) {
        v2 = (void *)sub_e680(dat_9efe0);
        sub_2d88a(v2);
        free(v2);
      }
    }
  }
  sub_d046();
  sub_2dd64();
  return;
}


// Function: sub_28a20 @ 0x28a20
uint8 sub_28a20(uint8 *a0,uint8 a1)
{
  return *a0 % a1;
}


// Function: sub_28a4d @ 0x28a4d
unsigned long sub_28a4d(int8 *a0,int8 *a1)
{
  return CONCAT71((undefined7)((uint8)*a1 >> 8),*a0 == *a1);
}


// Function: sub_28a83 @ 0x28a83
unsigned long sub_28a83(uint8 *a0,unsigned long a1,unsigned long a2,uint8 a3,unsigned long a4,uint4 a5) // return-dupe x2
{
  char *v1; // stack - 0x20
  uint8 v2; // stack - 0x18
  
  *__errno_location() = 0;
  v2 = strtoumax(a1,&v1,10);
  if ((!*v1) && (!*__errno_location())) {
    if (a3 < v2) {
      error(0,0,gettext("%s:%u: %s out of range: %s"),a4,(uint8)a5,a2,a1);
      return 0xffffffff;
    }
    *a0 = v2;
    return 0;
  }
  error(0,0,gettext("%s:%u: invalid %s: %s"),a4,(uint8)a5,a2,a1);
  return 0xffffffff;
}


// Function: sub_28bbb @ 0x28bbb
void sub_28bbb(int8 *a0,char *a1,code *a2,unsigned long a3,unsigned long a4)
{
  char *v1;
  char *v10; // stack - 0x150
  uint4 v11; // stack - 0x15c
  uint4 v12; // stack - 0x158
  int8 v13; // stack - 0x130
  void *v14; // stack - 0x128
  int8 v15; // stack - 0x120
  char *v16; // stack - 0x118
  int8 *v17; // stack - 0x110
  int8 *v18; // stack - 0x100
  unsigned long v19; // stack - 0xc8
  bool v2;
  int4 v3; // eax
  unsigned long v4; // rax
  int8 v5;
  int8 v6; // stack - 0x108
  int8 v7; // stack - 0x140
  int8 v8; // stack - 0x138
  unsigned long v9; // stack - 0x148
  
  v10 = NULL;
  v9 = 0;
  v2 = 0;
  v14 = fopen(a1,"r");
  if (!v14)
    sub_3e3b4(a1); // no-return
  v19 = 0x828f0;
  v11 = 0x8e44;
  v12 = 0;
label_29135:
  do {
    while( true ) {
      do {
        v15 = getline(&v10,&v9,v14);
        if (v15 <= 0) {
          if (v11 & 8)
            sub_46399(&v6);
          fclose(v14);
          if (!v2)
            return;
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("errors reading map file"));
          sub_177f4(); // no-return
        }
        v13 = 0;
        v12 += 1;
        v3 = sub_461a6(v10,&v6,v11);
        if (v3) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v4 = sub_4647f(&v6);
          error(0,0,gettext("%s:%u: cannot split line: %s"),a1,(uint8)v12,v4);
          sub_177f4(); // no-return
        }
        v11 |= 8;
      } while (!v6);
      if (v6 == 2) break;
      error(0,0,gettext("%s:%u: malformed line"),a1,(uint8)v12);
      v2 = 1;
    }
    if (*(char *)*v18 != '+') { // branch-flip
      if (a2) {
        v7 = (*a2)(*v18);
        if (v7 == -1) {
          v5 = *v18;
          error(0,0,gettext("%s:%u: can\'t obtain %s of %s"),a1,(uint8)v12,a3,v5);
          v2 = 1;
          goto label_29135;
        }
      }
    }
    else {
      v5 = *v18;
      if (sub_28a83(&v7,v5 + 1,a3,a4,a1,v12)) {
        v2 = 1;
        goto label_29135;
      }
    }
    v16 = strchr((char *)v18[1],0x3a);
    if (v16) { // branch-flip
      if ((char *)v18[1] < v16)
        v13 = v18[1];
      v1 = &v16[1];
      *v16 = '\0';
      v16 = v1;
      if (sub_28a83(&v8,v1,a3,a4,a1,v12)) {
        v2 = 1;
        goto label_29135;
      }
    }
    else if (*(char *)v18[1] != '+') { // branch-flip
      v13 = v18[1];
      v8 = (*a2)(v18[1]);
      if (v8 == -1) {
        v5 = v18[1];
        error(0,0,gettext("%s:%u: can\'t obtain %s of %s"),a1,(uint8)v12,a3,v5);
        v2 = 1;
        goto label_29135;
      }
    }
    else {
      v5 = v18[1];
      if (sub_28a83(&v8,v5,a3,a4,a1,v12)) {
        v2 = 1;
        goto label_29135;
      }
    }
    v17 = (int8 *)sub_60b5b(0x18);
    *v17 = v7;
    v17[1] = v8;
    if (v13) // branch-flip
      v5 = sub_60cc6(v13);
    else {
      v5 = 0;
    }
    v17[2] = v5;
    if (!*a0) {
      *a0 = sub_52c1c(0,0,sub_28a20,sub_28a4d,0);
      if (!*a0) {
        sub_1783c(); // no-return, return-dupe
      }
    }
    v5 = sub_5395d(*a0,v17);
    if (!v5)
      sub_1783c();
  } while( true );
}


// Function: sub_291f9 @ 0x291f9
uint8 sub_291f9(char *a0) // early-return
{
  void *v1; // rax
  
  v1 = getpwnam(a0);
  if (v1)
    return (uint8)*(uint4 *)((int8)v1 + 0x10);
  return 0xffffffffffffffff;
}


// Function: sub_29234 @ 0x29234
void sub_29234(unsigned long a0)
{
  sub_28bbb(0x9f5e8,a0,sub_291f9,0x8295a,0xffffffff);
  return;
}


// Function: sub_29274 @ 0x29274
bool sub_29274(uint4 a0,int4 *a1,int8 *a2)
{
  int8 v1; // rax
  uint8 v2 [3]; // stack - 0x28
  
  if (dat_9f5e8) {
    v2[0] = (uint8)a0;
    v1 = sub_52567(dat_9f5e8,v2);
    if (v1) {
      *a1 = (int4)*(unsigned long *)(v1 + 8);
      *a2 = *(int8 *)(v1 + 0x10);
      return 0;
    }
  }
  if (dat_9f948 != -1)
    *a1 = dat_9f948;
  if (dat_9f940)
    *a2 = dat_9f940;
  return !dat_9f940 && dat_9f948 == -1;
}


// Function: sub_29353 @ 0x29353
uint8 sub_29353(char *a0) // early-return
{
  void *v1; // rax
  
  v1 = getgrnam(a0);
  if (v1)
    return (uint8)*(uint4 *)((int8)v1 + 0x10);
  return 0xffffffffffffffff;
}


// Function: sub_2938e @ 0x2938e
void sub_2938e(unsigned long a0)
{
  sub_28bbb(0x9f5f0,a0,sub_29353,0x8295e,0xffffffff);
  return;
}


// Function: sub_293ce @ 0x293ce
bool sub_293ce(uint4 a0,int4 *a1,int8 *a2)
{
  int8 v1; // rax
  uint8 v2 [3]; // stack - 0x28
  
  if (dat_9f5f0) {
    v2[0] = (uint8)a0;
    v1 = sub_52567(dat_9f5f0,v2);
    if (v1) {
      *a1 = (int4)*(unsigned long *)(v1 + 8);
      *a2 = *(int8 *)(v1 + 0x10);
      return 0;
    }
  }
  if (dat_9f8b8 != -1)
    *a1 = dat_9f8b8;
  if (dat_9f8b0)
    *a2 = dat_9f8b0;
  return !dat_9f8b0 && dat_9f8b8 == -1;
}


// Function: sub_29516 @ 0x29516
void sub_29516(unsigned int a0,unsigned long a1)
{
  sub_5e599(a0,sub_5c97e(0),a1);
  return;
}


// Function: sub_29547 @ 0x29547
void sub_29547(unsigned long *a0,int8 a1)
{
  unsigned long v1; // rax
  
  free((void *)*a0);
  if (a1) // branch-flip
    v1 = sub_60cc6(a1);
  else {
    v1 = 0;
  }
  *a0 = v1;
  return;
}


// Function: sub_29591 @ 0x29591
void sub_29591(unsigned long *a0,char *a1,uint8 a2) // return-dupe
{
  uint8 v1; // rax
  void *v2; // rax
  
  free((void *)*a0);
  if (!a1)
    return;
  v1 = strnlen(a1,a2);
  v2 = (void *)sub_60b5b(v1 + 1);
  memcpy(v2,a1,v1);
  *(char *)(v1 + (int8)v2) = 0;
  *a0 = v2;
  return;
}


// Function: sub_2961d @ 0x2961d
unsigned int sub_2961d(char *a0)
{
  char *v1;
  char *v2;
  char *v3;
  char v4; // al
  unsigned int v5; // stack - 0x20
  char *v6; // stack - 0x18
  char *v7; // stack - 0x10
  
  v5 = 1;
  v3 = a0;
  v1 = a0;
  while (v7 = v1, v6 = v3, *v6) {
    if (*v6 != '\\') { // branch-flip
      if (v6 != v7) { // branch-flip
        *v7 = *v6;
        v3 = &v6[1];
        v1 = &v7[1];
      }
      else {
        v3 = &v6[1];
        v1 = &v7[1];
      }
    }
    else {
      v3 = &v6[1];
      switch(*v3) {
        case 0x30:
        case 0x31:
        case 0x32:
        case 0x33:
        case 0x34:
        case 0x35:
        case 0x36:
        case 0x37:
          v1 = &v6[2];
          if (('0' <= *v1) && (*v1 <= '7')) {
            v2 = &v6[3];
            v4 = *v1 + (*v3 + '\xd0') * '\b' + '\xd0';
            if (('0' <= *v2) && (*v2 <= '7')) {
              *v7 = *v2 + v4 * '\b' + '\xd0';
              v3 = &v6[4];
              v1 = &v7[1];
            }
            else {
              *v7 = v4;
              v3 = v2;
              v1 = &v7[1];
            }
          }
          else {
            *v7 = *v3 + '\xd0';
            v3 = v1;
            v1 = &v7[1];
          }
          break;
        default:
          v5 = 0;
          *v7 = '\\';
          v1 = &v7[1];
          if (*v3) {
            v7[1] = *v3;
            v3 = &v6[2];
            v1 = &v7[2];
          }
          break;
        case 0x3f:
          *v7 = '\x7f';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x5c:
          *v7 = '\\';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x61:
          *v7 = '\a';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x62:
          *v7 = '\b';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x66:
          *v7 = '\f';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x6e:
          *v7 = '\n';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x72:
          *v7 = '\r';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x74:
          *v7 = '\t';
          v3 = &v6[2];
          v1 = &v7[1];
          break;
        case 0x76:
          *v7 = '\v';
          v3 = &v6[2];
          v1 = &v7[1];
        
      }
    }
  }
  if (v6 != v7)
    *v7 = '\0';
  return v5;
}


// Function: sub_298d7 @ 0x298d7
char * sub_298d7(char *a0)
{
  char *v1;
  char *v2; // stack - 0x10
  
  if ((a0) && (*a0)) {
    v2 = &a0[strlen(a0) - 1];
    while ((a0 < v2 && (*v2 == '/'))) {
      v1 = &v2[-1];
      *v2 = '\0';
      v2 = v1;
    }
  }
  return a0;
}


// Function: sub_29946 @ 0x29946
void sub_29946(char *a0) // return-dupe, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_29946
{
  char *v1;
  char *v2;
  char v3;
  int4 v4; // eax
  int4 v5; // eax
  int8 v6; // rax
  char *v7; // stack - 0x20
  char *v8; // stack - 0x18
  
  v7 = a0;
  v8 = a0;
  while ((*v7 = *v8, *v7 == '.' && (v8[1] == '/'))) {
    for (v8 = &v8[2]; *v8 == '/'; v8 = &v8[1]) {
    }
    v7 = &v7[*v8 == '\0'];
  }
  while( true ) {
    v1 = &v8[1];
    v3 = *v8;
    v2 = &v7[1];
    *v7 = v3;
    if (!*v7) break;
    v7 = v2;
    v8 = v1;
    if (v3 == '/') {
      for (; v8[*v8 == '.'] == '/'; v8 = &v8[v6]) {
        v6 = (*v8 != '.') ? 1 : 2; // branch-flip
      }
    }
  }
  if ((int8)v2 - (int8)a0 <= 2)
    return;
  v4 = !((v7[-1] != '.') || (v7[-2] != '/'));
  v5 = ((3 <= (int8)&v2[-(int8)a0 - (int8)v4]) && (v2[0xfffffffffffffffe - (int8)v4] == '/'));
  v2[(-(int8)v5 - (int8)v4) + -1] = '\0';
  return;
}


// Function: sub_29aac @ 0x29aac
char * sub_29aac(unsigned int a0,char *a1)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3; // stack - 0x30
  
  v3 = NULL;
  if (*a1 != '/') {
    v1 = (char *)sub_2afed(a0);
    v2 = strlen(v1);
    v3 = (char *)sub_60b5b(strlen(a1) + v2 + 2);
    strcpy(v3,v1);
    v3[v2] = '/';
    strcpy(&v3[v2 + 1],a1);
  }
  if (!v3)
    v3 = (char *)sub_60cc6(a1);
  sub_29946(v3);
  return v3;
}


// Function: sub_29b93 @ 0x29b93
void sub_29b93(unsigned long *a0,void *a1,uint8 a2,void *a3,uint8 a4)
{
  uint8 v1; // rax
  char *v2; // stack - 0x18
  
  v2 = (char *)*a0;
  v1 = strlen(v2);
  if (a2 < v1) {
    if ((!memcmp(v2,a1,a2)) && (v2[a2] == '/')) {
      if (a2 < a4) {
        v2 = (char *)sub_60b8d(v2,a4 + (v1 - a2) + 1);
        *a0 = v2;
      }
      memmove(&v2[a4],&v2[a2],(v1 - a2) + 1);
      memcpy(v2,a3,a4);
    }
  }
  return;
}


// Function: sub_29c96 @ 0x29c96
void sub_29c96(uint8 a0,unsigned long a1,uint8 a2,unsigned long a3) // return-dupe
{
  if (a0 <= a2) {
    sub_54e5a(a0,a3);
    return;
  }
  sub_54bde(a0,a3);
  return;
}


// Function: sub_29cfc @ 0x29cfc
uint8 sub_29cfc(char *a0,unsigned long a1,uint8 a2,uint8 a3)
{
  uint8 v1; // rax
  
  *__errno_location() = 0;
  if (0 <= (int8)a3) { // branch-flip
    if ((uint4)((int4)a0[*a0 == '-'] - 0x30U) <= 9) {
      v1 = strtoimax(a0,a1,10);
      if (((int8)a2 <= (int8)v1) && ((int8)v1 <= (int8)a3))
        return v1;
      *__errno_location() = 0x22;
      if ((int8)v1 < (int8)a2)
        return a2;
      return a3;
    }
  }
  else if ((uint4)((int4)*a0 - 0x30U) <= 9) {
    v1 = strtoumax(a0,a1,10);
    if (v1 <= a3)
      return sub_294ad(v1);
    *__errno_location() = 0x22;
    return a3;
  }
  *__errno_location() = 0x16;
  return 0;
}


// Function: sub_29e23 @ 0x29e23
void sub_29e23(int4 a0,char *a1) // return-dupe
{
  int4 v1; // stack - 0x1c
  int4 v2; // stack - 0xc
  
  if (!a0) {
    *a1 = 0;
    return;
  }
  v2 = 9;
  *a1 = 0x2e;
  for (v1 = a0; !(v1 % 10); v1 = v1 / 10) {
    v2 -= 1;
  }
  a1[(int8)v2 + 1] = 0;
  while( true ) {
    v2 -= 1;
    a1[(int8)v2 + 1] = (char)v1 + (char)(v1 / 10) * '\xf6' + '0';
    if (!v2) break;
    v1 /= 10;
  }
  return;
}


// Function: sub_29f2e @ 0x29f2e
char * sub_29f2e(int8 a0,int4 a1,int8 a2)
{
  int4 v1; // stack - 0x2c
  int8 v2; // stack - 0x28
  char *v3; // stack - 0x20
  
  if ((999999999 < a1) || (v1 = a1, a1 <= -1))
    v1 = 0;
  v2 = a0;
  if ((a0 < 0) && (v1)) {
    v2 = a0 + 1;
    v1 = 1000000000 - v1;
  }
  if (a0 <= -1)
    v2 = -v2;
  v3 = (char *)sub_54e5a(v2,a2 + 1);
  if (a0 < 0) {
    v3 = &v3[-1];
    *v3 = 0x2d;
  }
  sub_29e23(v1,a2 + 0x15);
  return v3;
}


// Function: sub_29ff7 @ 0x29ff7
undefined16 sub_29ff7(char *a0,int8 *a1,char a2)
{
  char v1 [16];
  int8 v2; // rax
  bool v3; // zf
  bool v4; // stack - 0x42
  int4 v5; // stack - 0x40
  int4 v6; // stack - 0x3c
  int8 v7; // stack - 0x38
  char *v8; // stack - 0x30
  
  v7 = -0x8000000000000000;
  v5 = -1;
  v3 = *a0 == '-';
  if (10 <= (uint4)((int4)a0[v3] - 0x30U)) { // branch-flip
    *__errno_location() = 0x16;
    v8 = a0;
  }
  else {
    *__errno_location() = 0;
    if (v3)
      v7 = strtoimax(a0,a1,10);
    else {
      v2 = strtoumax(a0,a1,10);
      if (0 <= v2) // branch-flip
        v7 = v2;
      else {
        *__errno_location() = 0x22;
      }
    }
    v8 = (char *)*a1;
    v5 = 0;
    if ((a2) && (*v8 == '.')) {
      v6 = 0;
      v4 = 0;
      while (v8 = &v8[1], (uint4)((int4)*v8 - 0x30U) <= 9) {
        if (9 <= v6) // branch-flip
          v4 = *v8 != '0' || v4;
        else {
          v6 += 1;
          v5 = *v8 + -0x30 + v5 * 10;
        }
      }
      for (; v6 <= 8; v6 = v6 + 1) {
        v5 *= 10;
      }
      if ((v3) && (v5 = v5 + (uint4)v4, v5)) {
        if (v7 != -0x8000000000000000) { // branch-flip
          v7 -= 1;
          v5 = 1000000000 - v5;
        }
        else {
          v5 = -1;
        }
      }
    }
    if (*__errno_location() == 0x22)
      v5 = -1;
  }
  *a1 = (int8)v8;
  v1._8_8_ = (int8)v5;
  v1._0_8_ = v7;
  return v1._0_16_;
}


// Function: sub_2a1fe @ 0x2a1fe
bool sub_2a1fe(char *a0) // early-return, ternary x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2a1fe
{
  bool v1; // al
  int8 v2; // rax
  char *v3; // stack - 0x10
  
  v3 = a0;
  if (*a0 != '/') {
    while ((*v3 == '.' && (v3[1] == '/'))) {
      for (v3 = &v3[2]; *v3 == '/'; v3 = &v3[1]) {
      }
    }
    v1 = !((*v3) && ((*v3 != '.' || (v3[1]))));
    return v1;
  }
  while( true ) {
    for (; v3[1] == '/'; v3 = &v3[1]) {
    }
    if (v3[1] != '.') break;
    v2 = (v3[2] != '.') ? 2 : 3; // branch-flip
    if (v3[v2] != '/') break;
    v2 = (v3[2] != '.') ? 2 : 3; // branch-flip
    v3 = &v3[v2];
  }
  return v3[1] == '\0';
}


// Function: sub_2a306 @ 0x2a306
unsigned long sub_2a306(char *a0) // return-dupe x2
{
  int4 v1; // eax
  
  if (sub_2a1fe(a0)) {
    *__errno_location() = 0;
    return 0xffffffff;
  }
  v1 = unlinkat(dat_9c268,a0,0x200);
  if (!v1) {
    sub_19454(a0);
    return 0;
  }
  return 0xffffffff;
}


// Function: sub_2a36f @ 0x2a36f
uint8 sub_2a36f(char *a0,uint4 a1) // early-return x8
{
  int4 v1;
  int4 v2; // eax
  char *v3; // rax
  void *v4; // rax
  uint8 v5; // rax
  char *v6; // stack - 0x28
  
  v1 = unlinkat(dat_9c268,a0,0);
  if (!v1)
    return 1;
  if ((*__errno_location() != 1) && (*__errno_location() != 0x15))
    return 0;
  if (!sub_2a306(a0))
    return 1;
  v1 = *__errno_location();
  if (v1 != 0x27) {
    if (0x28 <= v1)
      return 0;
    if (v1 == 0x14)
      return 0;
    if (0x15 <= v1)
      return 0;
    if ((v1) && (v1 != 0x11))
      return 0;
  }
  if (a1 == 2)
    return 0xffffffff;
  if (((a1 <= 2) && (a1)) && (a1 == 1)) {
    v3 = (char *)sub_2b8e1(a0,0);
    if (!v3)
      return 0;
    v6 = v3;
    while( true ) {
      v5 = strlen(v6);
      if (!v5) {
        free(v3);
        return (uint8)(sub_2a306(a0) == 0);
      }
      v4 = (void *)sub_2ee85(a0,v6);
      v2 = sub_2a36f(v4,1);
      v1 = *__errno_location();
      free(v4);
      if (!v2) break;
      v6 = &v6[v5 + 1];
    }
    free(v3);
    *__errno_location() = v1;
    return 0;
  }
  return 0;
}


// Function: sub_2a571 @ 0x2a571
unsigned long sub_2a571(char *a0,char a1) // return-dupe x6
{
  int4 v1;
  uint8 v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char v5 [24];
  uint4 v6; // stack - 0xa0
  
  sub_29547(0x9f608,a0);
  sub_29547(0x9f610,0);
  if ((((a1) && (dat_9fdf8 != '\x01')) && (dat_9fdf0 = strchr(a0,0x3a), dat_9fdf0)) && ((a0 < dat_9fdf0 && (v2 = (int8)dat_9fdf0 - (int8)a0, !memchr(a0,0x2f,v2)))))
    return 1;
  if (sub_2a9b8(a0,v5)) {
    if (*__errno_location() == 2)
      return 1;
    sub_3ea03(a0);
    return 0;
  }
  if ((v6 & 0xf000) == 0x4000)
    return 1;
  if ((a1) && (((v6 & 0xf000) == 0x6000 || ((v6 & 0xf000) == 0x2000))))
    return 1;
  dat_9f610 = sub_4de0e(dat_9c268,a0,dat_9f888);
  if (!dat_9f610)
    sub_1783c(); // no-return
  v1 = renameat(dat_9c268,dat_9f608,dat_9c268,dat_9f610);
  if (!v1) {
    if (dat_9f9ac) {
      v3 = sub_5e821(1,dat_9f610);
      v4 = sub_5e821(0,dat_9f608);
      fprintf(dat_9efd0,gettext("Renaming %s to %s\n"),v4,v3);
    }
    return 1;
  }
  v1 = *__errno_location();
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v3 = sub_5e821(1,dat_9f610);
  v4 = sub_5e548(dat_9f608);
  error(0,v1,gettext("%s: Cannot rename to %s"),v4,v3);
  dat_9fdd0 = 2;
  sub_29547(0x9f610,0);
  return 0;
}


// Function: sub_2a874 @ 0x2a874
void sub_2a874(void) // return-dupe
{
  int4 v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  if (!dat_9f610)
    return;
  v1 = renameat(dat_9c268,dat_9f610,dat_9c268,dat_9f608);
  if (v1) {
    v1 = *__errno_location();
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_5e821(1,dat_9f608);
    v3 = sub_5e548(dat_9f610);
    error(0,v1,gettext("%s: Cannot rename to %s"),v3,v2);
    dat_9fdd0 = 2;
  }
  if (dat_9f9ac) {
    v2 = sub_5e821(1,dat_9f608);
    v3 = sub_5e821(0,dat_9f610);
    fprintf(dat_9efd0,gettext("Renaming %s back to %s\n"),v3,v2);
  }
  sub_29547(0x9f610,0);
  return;
}


// Function: sub_2a9b8 @ 0x2a9b8
void sub_2a9b8(char *a0,void *a1)
{
  fstatat(dat_9c268,a0,a1,dat_9fc10);
  return;
}


// Function: sub_2a9e9 @ 0x2a9e9
int8 sub_2a9e9(unsigned int a0,unsigned long a1,unsigned long a2)
{
  uint4 v1; // eax
  int8 v2; // stack - 0x10
  
  v2 = sub_5ec70(a0,a1,a2);
  if (v2 == -1) {
    if (*__errno_location() == 0xb) {
      v1 = sub_617eb(a0,3);
      if ((0 <= (int4)v1) && (v1 & 0x800)) {
        if (sub_617eb(a0,4,v1 & 0xfffff7ff) != -1)
          v2 = sub_5ec70(a0,a1,a2);
      }
    }
  }
  return v2;
}


// Function: sub_2aa95 @ 0x2aa95
uint8 sub_2aa95(unsigned int a0,int8 a1,uint8 a2)
{
  uint4 v1; // eax
  uint8 v2; // stack - 0x18
  
  v2 = sub_52175(a0,a1,a2);
  if (v2 < a2) {
    if (*__errno_location() == 0xb) {
      v1 = sub_617eb(a0,3);
      if ((0 <= (int4)v1) && (v1 & 0x800)) {
        if (sub_617eb(a0,4,v1 & 0xfffff7ff) != -1)
          v2 += sub_52175(a0,a1 + v2,a2 - v2);
      }
    }
  }
  return v2;
}


// Function: sub_2ab5e @ 0x2ab5e
void sub_2ab5e(unsigned int a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x38
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x20
  
  v3 = 0x3ffffffe;
  v1 = a3;
  v2 = a4;
  sub_4f4ec(a0,a1,a2,&v1,dat_9fc10);
  return;
}


// Function: sub_2abe0 @ 0x2abe0
int4 sub_2abe0(void) // early-return
{
  if (!dat_9f620)
    return 0;
  return (int4)dat_9f620 + -1;
}


// Function: sub_2ac09 @ 0x2ac09
uint8 sub_2ac09(char *a0)
{
  uint8 v1;
  char *v2; // stack - 0x10
  
  if (dat_9f620 == dat_9f628) {
    if (!dat_9f628)
      dat_9f628 = 2;
    dat_9f618 = (unsigned long *)sub_60a5f(dat_9f618,0x9f628,0x18);
    if (!dat_9f620) {
      *dat_9f618 = 0x82ac7;
      dat_9f618[1] = 0;
      *(unsigned int *)&dat_9f618[2] = 0xffffff9c;
      dat_9f620 = 1;
    }
  }
  v1 = dat_9f620;
  v2 = a0;
  if (*a0) {
    while ((*v2 == '.' && (v2[1] == '/'))) {
      for (v2 = &v2[2]; *v2 == '/'; v2 = &v2[1]) {
      }
    }
    if (!v2[*v2 == '.'])
      return (uint8)(uint4)((int4)dat_9f620 - 1);
  }
  dat_9f618[dat_9f620 * 3] = v2;
  dat_9f618[dat_9f620 * 3 + 1] = 0;
  *(unsigned int *)&dat_9f618[dat_9f620 * 3 + 2] = 0;
  dat_9f620 += 1;
  return v1;
}


// Function: sub_2adf1 @ 0x2adf1
void sub_2adf1(int4 a0) // return-dupe
{
  uint8 v1;
  int4 v2;
  unsigned long *v3; // rax
  int4 v4; // stack - 0x2c
  int4 v5; // stack - 0x28
  int8 v6; // stack - 0x20
  
  if (a0 == dat_9f600)
    return;
  v3 = (unsigned long *)((int8)a0 * 0x18 + dat_9f618);
  v4 = *(int4 *)&v3[2];
  if (!v4) {
    if (*(char *)*v3 != '/')
      sub_2adf1(a0 + -1);
    v4 = openat(dat_9c268,(char *)*v3,dat_9fc0c & 0xfffdffff);
    if (v4 <= -1)
      sub_3e3b4(*v3); // no-return
    *(int4 *)&v3[2] = v4;
    if (0x10 <= dat_9f680) { // branch-flip
      v3 = (unsigned long *)((int8)dat_9f67c * 0x18 + dat_9f618);
      v2 = *(int4 *)&v3[2];
      if (close(v2))
        sub_2b26a(*v3);
      *(unsigned int *)&v3[2] = 0;
      dat_9f67c = a0;
    }
    else {
      v1 = dat_9f680 + 1;
      *(int4 *)(dat_9f680 * 4 + 0x9f640) = a0;
      dat_9f680 = v1;
    }
  }
  if (v4 <= 0) {
    dat_9c268 = v4;
    dat_9f600 = a0;
    return;
  }
  v5 = dat_9f640;
  v6 = 1;
  while( true ) {
    if (v5 == a0) {
      dat_9c268 = v4;
      dat_9f600 = a0;
      dat_9f640 = a0;
      return;
    }
    v2 = *(int4 *)(v6 * 4 + 0x9f640);
    *(int4 *)(v6 * 4 + 0x9f640) = v5;
    if (v2 == a0) break;
    v6 += 1;
    v5 = v2;
  }
  dat_9c268 = v4;
  dat_9f600 = a0;
  dat_9f640 = a0;
  return;
}


// Function: sub_2afc0 @ 0x2afc0
unsigned long sub_2afc0(void)
{
  return *(unsigned long *)((int8)dat_9f600 * 0x18 + dat_9f618);
}


// Function: sub_2afed @ 0x2afed
int8 sub_2afed(int4 a0)
{
  unsigned int v1;
  unsigned long v2;
  int8 v3;
  int4 v4; // stack - 0x28
  
  v1 = dat_9f600;
  if (dat_9f618) { // branch-flip
    if (!*(int8 *)((int8)a0 * 0x18 + dat_9f618 + 8)) {
      v4 = a0;
      while ((0 <= v4 && (!*(int8 *)((int8)v4 * 0x18 + dat_9f618 + 8)))) {
        v4 -= 1;
      }
      while (v4 = v4 + 1, v4 <= a0) {
        sub_2adf1(v4);
        v3 = dat_9f618;
        if (v4) { // branch-flip
          if (**(char **)((int8)v4 * 0x18 + dat_9f618) != '/') { // branch-flip
            v2 = sub_2b66c(*(unsigned long *)((int8)v4 * 0x18 + dat_9f618 + -0x10));
            sub_2b7ec(v2,*(unsigned long *)((int8)v4 * 0x18 + dat_9f618));
            v3 = dat_9f618 + (int8)v4 * 0x18;
            *(unsigned long *)(v3 + 8) = sub_2b882(v2);
          }
          else {
            v2 = *(unsigned long *)((int8)v4 * 0x18 + dat_9f618);
            v3 = dat_9f618 + (int8)v4 * 0x18;
            *(unsigned long *)(v3 + 8) = sub_60cc6(v2);
          }
        }
        else {
          *(unsigned long *)(v3 + 8) = sub_60cf7();
          if (!*(int8 *)(v3 + 8))
            sub_3dfc0("getcwd",0x82ac7); // no-return
        }
      }
      sub_2adf1(v1);
    }
    v3 = *(int8 *)((int8)a0 * 0x18 + dat_9f618 + 8);
  }
  else {
    if ((!dat_9f688) && (dat_9f688 = sub_60cf7(), !dat_9f688))
      sub_3dfc0("getcwd",0x82ac7); // no-return
    v3 = dat_9f688;
  }
  return v3;
}


// Function: sub_2b26a @ 0x2b26a
void sub_2b26a(unsigned long a0) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e1ff(a0);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e228(a0);
  return;
}


// Function: sub_2b2b1 @ 0x2b2b1
void sub_2b2b1(unsigned long a0) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e38b(a0);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e3da(a0);
  return;
}


// Function: sub_2b2f8 @ 0x2b2f8
void sub_2b2f8(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e42c(a0,a1,a2);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e506(a0,a1,a2);
  return;
}


// Function: sub_2b35d @ 0x2b35d
void sub_2b35d(unsigned long a0) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e6b3(a0);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e6dc(a0);
  return;
}


// Function: sub_2b3a4 @ 0x2b3a4
void sub_2b3a4(unsigned long a0) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e72e(a0);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e757(a0);
  return;
}


// Function: sub_2b3eb @ 0x2b3eb
void sub_2b3eb(unsigned long a0,unsigned long a1) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3e7a9(a0,a1);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3e890(a0,a1);
  return;
}


// Function: sub_2b444 @ 0x2b444
void sub_2b444(unsigned long a0) // return-dupe x2
{
  if (!dat_9f8bc) {
    sub_3ea03(a0);
    return;
  }
  if (!(dat_9ec84 & 0x800000))
    return;
  sub_3ea2c(a0);
  return;
}


// Function: sub_2b48b @ 0x2b48b
void sub_2b48b(unsigned long a0,char a1,code *a2)
{
  unsigned long v1; // rax
  
  if ((a1 != '\x01') && (*__errno_location() == 2)) {
    if (dat_9ec84 & 0x40) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = sub_5e548(a0);
      error(0,0,gettext("%s: File removed before we read it"),v1);
    }
    sub_39e98(1);
    return;
  }
  (*a2)(a0);
  return;
}


// Function: sub_2b537 @ 0x2b537
int4 sub_2b537(void)
{
  int4 v1; // eax
  
  v1 = fork();
  if (v1 == -1)
    sub_3dfc0(0x82b01,gettext("child process")); // no-return
  return v1;
}


// Function: sub_2b577 @ 0x2b577
void sub_2b577(int4 *a0)
{
  if (pipe(a0) <= -1)
    sub_3dfc0(0x82b1b,gettext("interprocess channel")); // no-return
  return;
}


// Function: sub_2b5bb @ 0x2b5bb
int8 sub_2b5bb(int8 a0,uint8 a1)
{
  uint8 v1; // rax
  
  v1 = a0 + (a1 - 1);
  return v1 - v1 % a1;
}


// Function: sub_2b601 @ 0x2b601
void sub_2b601(unsigned long *a0,uint8 a1)
{
  int8 v1; // rax
  uint8 v2; // rax
  
  v1 = (int8)getpagesize();
  v2 = v1 + a1;
  if (v2 < a1)
    sub_1783c(); // no-return
  *a0 = sub_60b5b(v2);
  sub_2b5bb(*a0,v1);
  return;
}


// Function: sub_2b66c @ 0x2b66c
int8 * sub_2b66c(char *a0)
{
  int8 v1;
  char *v2;
  int8 *v3; // rax
  
  v3 = (int8 *)sub_60b5b(0x18);
  v3[1] = strlen(a0) + 2;
  v1 = v3[1];
  *v3 = sub_60b5b(v1);
  strcpy((char *)*v3,a0);
  v2 = (char *)*v3;
  v3[2] = strlen(v2);
  if (*(char *)(v3[2] + -1 + *v3) != '/') {
    v1 = v3[2];
    v3[2] = v1 + 1;
    *(char *)(v1 + *v3) = 0x2f;
  }
  return v3;
}


// Function: sub_2b72d @ 0x2b72d
void sub_2b72d(unsigned long *a0)
{
  free((void *)*a0);
  free(a0);
  return;
}


// Function: sub_2b75b @ 0x2b75b
int8 sub_2b75b(int8 *a0,char *a1)
{
  int8 v1;
  uint8 v2; // rax
  
  v2 = strlen(a1);
  while ((uint8)a0[1] <= v2 + a0[2] + 1) {
    v1 = *a0;
    *a0 = sub_60bd8(v1,&a0[1]);
  }
  strcpy((char *)(*a0 + a0[2]),a1);
  return *a0;
}


// Function: sub_2b7ec @ 0x2b7ec
void sub_2b7ec(int8 *a0,char *a1)
{
  int8 v1;
  
  if (*(char *)(a0[2] + -1 + *a0) != '/') {
    sub_2b75b(a0,0x9c26c);
    a0[2] = a0[2] + 1;
  }
  sub_2b75b(a0,a1);
  v1 = a0[2];
  a0[2] = v1 + strlen(a1);
  return;
}


// Function: sub_2b882 @ 0x2b882
int8 sub_2b882(int8 *a0)
{
  int8 v1;
  
  v1 = *a0;
  if (*(char *)(a0[2] + -1 + *a0) == '/')
    *(char *)(a0[2] + *a0) = 0;
  free(a0);
  return v1;
}


// Function: sub_2b8e1 @ 0x2b8e1
int8 sub_2b8e1(char *a0,int4 a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2b8e1
{
  int4 v1; // eax
  bool v2;
  int8 v3; // stack - 0x18
  void *v4; // stack - 0x10
  
  v3 = 0;
  v4 = NULL;
  v1 = openat(dat_9c268,a0,dat_9fc08 | 0x10000);
  if (0 <= v1) { // branch-flip
    v4 = fdopendir(v1);
    if (v4) {
      v3 = sub_5eee2(v4,dat_9fc2c);
      if (v3) goto label_2b99f;
    }
    sub_3e72e(a0);
  }
  else {
    if ((!a1) && (*__errno_location() == 2))
      return 0;
    sub_3e38b(a0);
  }
label_2b99f:
  if (v4) // branch-flip
    v2 = closedir(v4) != 0;
  else {
    v2 = ((0 <= v1) && (close(v1)));
  }
  if (v2)
    sub_3e72e(a0);
  return v3;
}


// Function: sub_2b9f2 @ 0x2b9f2
int8 * sub_2b9f2(int4 a0)
{
  int8 *v1; // stack - 0x10
  
  for (v1 = (int8 *)0x9c280; ((*v1 || ((int4)v1[1])) || (v1[4])); v1 = &v1[6]) {
    if (a0 == (int4)v1[1])
      return v1;
  }
  return NULL;
}


// Function: sub_2ba4c @ 0x2ba4c
unsigned long sub_2ba4c(unsigned int a0) // early-return
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_2b9f2(a0);
  if (v1)
    return *v1;
  return 0;
}


// Function: sub_2ba80 @ 0x2ba80
unsigned long sub_2ba80(unsigned int a0)
{
  int8 v1; // rax
  
  v1 = sub_2b9f2(a0);
  return CONCAT71((undefined7)((uint8)v1 >> 8),v1 != 0);
}


// Function: sub_2baa1 @ 0x2baa1
unsigned long sub_2baa1(unsigned int a0,unsigned long a1,int8 a2) // return-dupe x2, return-dupe, return-dupe x4
{
  int8 *v1;
  unsigned long v2;
  unsigned long v3;
  
  switch(a0) {
    default:
      if (sub_2ba80(a0)) {
        sub_2c884(a0,a1);
        return 0;
      }
      return 7;
    case 0x43:
      sub_2c8c4(a1);
      return 0;
    case 0x54:
      sub_2c8f7(a1);
      return 0;
    case 0x100:
      sub_2c829(a1);
      return 0;
    case 0x1000005:
      goto label_2bb2b;
    
  }
label_2bb2b:
  v1 = *(int8 **)(a2 + 0x28);
  if (*(int4 *)*v1 != 2)
    return 7;
  v2 = *(unsigned long *)(*v1 + 0x10);
  v3 = *(unsigned long *)(*v1 + 8);
  error(0,0,gettext("%s:%lu: unrecognized option"),v3,v2);
  sub_39e98(2);
  return 7;
}


// Function: sub_2bbce @ 0x2bbce
void sub_2bbce(int8 a0,unsigned int a1)
{
  int4 v1; // stack - 0xc
  
  v1 = 0;
  while (*(int8 *)(a0 + (int8)v1 * 8)) {
    sub_4ee5d(dat_9f8a8,*(unsigned long *)(a0 + (int8)v1 * 8),a1);
    v1 += 1;
  }
  return;
}


// Function: sub_2bc3a @ 0x2bc3a
void sub_2bc3a(uint4 a0,unsigned long a1) // ternary x2
{
  int4 v1;
  uint4 v2; // edx
  
  switch(a0) {
    case 0x58:
      v2 = (dat_9f6bc != 1) ? 0x10000000 : 0; // branch-flip
      v1 = sub_4f407(sub_4ee5d,dat_9f8a8,a1,v2 | dat_9f6c0 | dat_9f930,10);
      if (v1) {
        v1 = *__errno_location();
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,v1,"%s",sub_5e548(a1));
        sub_177f4(); // no-return
      }
      break;
    default:
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,"unhandled positional option %d",(uint8)a0);
      sub_177f4(); // no-return
    case 0x101:
      sub_2bbce(0x9a7a0,0x10000000);
      break;
    case 0x102:
      sub_11ea1("CACHEDIR.TAG",1,sub_12087);
      break;
    case 0x103:
      sub_11ea1("CACHEDIR.TAG",2,sub_12087);
      break;
    case 0x104:
      sub_11ea1("CACHEDIR.TAG",3,sub_12087);
      break;
    case 0x105:
      v2 = (dat_9f6bc != 1) ? 0x10000000 : 0; // branch-flip
      sub_4ee5d(dat_9f8a8,a1,v2 | dat_9f6c0 | dat_9f930);
      break;
    case 0x106:
      sub_17879(a1,2);
      break;
    case 0x107:
      sub_17879(a1,1);
      break;
    case 0x108:
      sub_11ea1(a1,1,0);
      break;
    case 0x109:
      sub_11ea1(a1,2,0);
      break;
    case 0x10a:
      sub_11ea1(a1,3,0);
      break;
    case 0x10b:
      sub_2bbce(0x9a6e0,0);
      break;
    case 0x10c:
      sub_18293();
      break;
    case 0x10d:
      dat_9f6c0 |= 0x10;
      break;
    case 0x10e:
      dat_9f6c0 &= 0xffffffef;
      break;
    case 0x10f:
      dat_9f6c0 |= 0x40000000;
      break;
    case 0x110:
      dat_9c274 = 0;
      dat_9f6c0 &= 0xbfffffff;
      break;
    case 0x111:
      dat_9f930 = 8;
      break;
    case 0x112:
      dat_9f930 = 0;
      break;
    case 0x113:
      dat_9fc28 = 1;
      break;
    case 0x114:
      dat_9fc28 = 0;
      break;
    case 0x115:
      dat_9f6b8 = 0;
      break;
    case 0x116:
      dat_9f6c0 |= 1;
      break;
    case 0x117:
      dat_9f6bc = 1;
      break;
    case 0x118:
      dat_9c270 = 0;
      dat_9f6b8 = 1;
      break;
    case 0x119:
      dat_9c270 = 10;
      dat_9f6b8 = 0;
      break;
    case 0x11a:
      dat_9f6b8 = 1;
      break;
    case 0x11b:
      dat_9f6c0 &= 0xfffffffe;
      break;
    case 0x11c:
      dat_9f6bc = 2;
    
  }
  return;
}


// Function: sub_2bfd1 @ 0x2bfd1
void sub_2bfd1(uint4 a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  if ((a0) && (a0 == dat_9f6f0)) {
    *a1 = sub_60cc6(0x83794);
    return;
  }
  if ((!dat_9f6c8) || (a0 != dat_9f6d8)) {
    v1 = getpwuid(a0);
    if (!v1) {
      dat_9f6f0 = a0;
      *a1 = sub_60cc6(0x83794);
      return;
    }
    dat_9f6d8 = a0;
    sub_29547(0x9f6c8,*v1);
  }
  v2 = sub_60cc6(dat_9f6c8);
  *a1 = v2;
  return;
}


// Function: sub_2c0a2 @ 0x2c0a2
void sub_2c0a2(uint4 a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  if ((a0) && (a0 == dat_9f6f4)) {
    *a1 = sub_60cc6(0x83794);
    return;
  }
  if ((!dat_9f6d0) || (a0 != dat_9f6dc)) {
    v1 = getgrgid(a0);
    if (!v1) {
      dat_9f6f4 = a0;
      *a1 = sub_60cc6(0x83794);
      return;
    }
    dat_9f6dc = a0;
    sub_29547(0x9f6d0,*v1);
  }
  v2 = sub_60cc6(dat_9f6d0);
  *a1 = v2;
  return;
}


// Function: sub_2c173 @ 0x2c173
unsigned long sub_2c173(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  unsigned long *v2; // rax
  
  if ((dat_9f6e0) && (v1 = strcmp(a0,dat_9f6e0), !v1))
    return 0;
  if (((!dat_9f6c8) || (*a0 != *dat_9f6c8)) || (v1 = strcmp(a0,dat_9f6c8), v1)) {
    v2 = getpwnam(a0);
    if (!v2) {
      sub_29547(0x9f6e0,a0);
      return 0;
    }
    dat_9f6d8 = *(unsigned int *)&v2[2];
    sub_29547(0x9f6c8,*v2);
  }
  *a1 = dat_9f6d8;
  return 1;
}


// Function: sub_2c261 @ 0x2c261
unsigned long sub_2c261(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  void *v2; // rax
  
  if ((dat_9f6e8) && (v1 = strcmp(a0,dat_9f6e8), !v1))
    return 0;
  if (((!dat_9f6d0) || (*a0 != *dat_9f6d0)) || (v1 = strcmp(a0,dat_9f6d0), v1)) {
    v2 = getgrnam(a0);
    if (!v2) {
      sub_29547(0x9f6e8,a0);
      return 0;
    }
    dat_9f6dc = *(unsigned int *)((int8)v2 + 0x10);
    sub_29547(0x9f6d0,a0);
  }
  *a1 = dat_9f6dc;
  return 1;
}


// Function: sub_2c34c @ 0x2c34c
int8 sub_2c34c(int8 a0)
{
  char *v1;
  int8 v2; // rax
  int8 v3; // stack - 0x20
  
  v2 = sub_60c06(0x60);
  v3 = a0;
  if (!a0)
    v3 = 0x83794;
  *(unsigned long *)(v2 + 0x10) = sub_60cc6(v3);
  v1 = *(char **)(v2 + 0x10);
  *(uint8 *)(v2 + 0x18) = strlen(v1);
  return v2;
}


// Function: sub_2c3b1 @ 0x2c3b1
void sub_2c3b1(void *a0) // return-dupe
{
  if (!a0)
    return;
  free(*(void **)((int8)a0 + 0x10));
  free(*(void **)((int8)a0 + 0x58));
  free(a0);
  return;
}


// Function: sub_2c3f7 @ 0x2c3f7
unsigned long * sub_2c3f7(void)
{
  unsigned long *v1; // stack - 0x10
  
  v1 = (unsigned long *)sub_60b5b(0x38);
  if (!dat_9f708) {
    *v1 = 0;
    v1[1] = *v1;
    *(unsigned int *)&v1[2] = 3;
    dat_9f708 = v1;
    v1 = (unsigned long *)sub_60b5b(0x38);
  }
  v1[1] = dat_9f708[1];
  if (dat_9f708[1])
    *(unsigned long **)dat_9f708[1] = v1;
  *v1 = dat_9f708;
  dat_9f708[1] = v1;
  return v1;
}


// Function: sub_2c4b8 @ 0x2c4b8
void sub_2c4b8(void) // return-dupe
{
  if (!dat_9f708)
    return;
  for (; *(int8 *)(dat_9f708 + 8); dat_9f708 = *(int8 *)(dat_9f708 + 8)) {
  }
  return;
}


// Function: sub_2c4f3 @ 0x2c4f3
void sub_2c4f3(unsigned long *a0) // return-dupe
{
  *a0 = 0;
  a0[1] = dat_9f6a8;
  if (!dat_9f6a8) {
    dat_9f6a8 = a0;
    return;
  }
  *dat_9f6a8 = a0;
  dat_9f6a8 = a0;
  return;
}


// Function: sub_2c541 @ 0x2c541
void sub_2c541(void)
{
  void *v1;
  
  while (v1 = dat_9f6a8, dat_9f6a8) {
    dat_9f6a8 = *(void **)((int8)dat_9f6a8 + 8);
    free(v1);
  }
  return;
}


// Function: sub_2c588 @ 0x2c588
void sub_2c588(void) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long *v3; // stack - 0x20
  
  if (!dat_9f6a8)
    return;
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly."));
  for (v3 = dat_9f6a8; v3[1]; v3 = (unsigned long *)v3[1]) {
  }
  for (; dat_9fdd0 = 2, v3; v3 = (unsigned long *)*v3) {
    if (*(int4 *)&v3[2] != 1) { // branch-flip
      if (*(int4 *)&v3[2] == 4) {
        if (v3[4]) { // branch-flip
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v1 = sub_5e84e(v3[4]);
          v2 = sub_2ba4c(*(unsigned int *)&v3[3]);
          error(0,0,gettext("--%s %s has no effect"),v2,v1);
        }
        else {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v1 = sub_2ba4c(*(unsigned int *)&v3[3]);
          error(0,0,gettext("--%s has no effect"),v1);
        }
      }
    }
    else {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = sub_5e84e(v3[3]);
      error(0,0,gettext("-C %s has no effect"),v1);
    }
  }
  sub_2c541();
  return;
}


// Function: sub_2c78a @ 0x2c78a
void sub_2c78a(void) // return-dupe x2
{
  int8 *v1;
  int8 v2;
  int8 *v3;
  
  v3 = dat_9f708;
  v2 = *dat_9f708;
  if (v2)
    *(unsigned long *)(v2 + 8) = 0;
  if (((int4)dat_9f708[2] != 4) && ((int4)dat_9f708[2] != 1)) {
    v1 = &dat_9f708[2];
    dat_9f708 = (int8 *)v2;
    if ((int4)*v1 != 3)
      sub_2c541();
    free(v3);
    return;
  }
  if ((dat_9f860 != 3) && (dat_9f860 != 8)) {
    dat_9f708 = (int8 *)v2;
    return;
  }
  dat_9f708 = (int8 *)v2;
  sub_2c4f3(v3);
  return;
}


// Function: sub_2c829 @ 0x2c829
void sub_2c829(unsigned long a0) // return-dupe x2
{
  int8 v1; // rax
  
  v1 = sub_2c3f7();
  *(unsigned int *)(v1 + 0x10) = 0;
  *(unsigned long *)(v1 + 0x18) = a0;
  if (!dat_9f6a0) {
    dat_9f6a0 = 1;
    return;
  }
  if (dat_9f6a0 != 1)
    return;
  dat_9f6a0 = 2;
  return;
}


// Function: sub_2c884 @ 0x2c884
void sub_2c884(unsigned int a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_2c3f7();
  *(unsigned int *)(v1 + 0x10) = 4;
  *(unsigned int *)(v1 + 0x18) = a0;
  *(unsigned long *)(v1 + 0x20) = a1;
  return;
}


// Function: sub_2c8c4 @ 0x2c8c4
void sub_2c8c4(unsigned long a0)
{
  int8 v1; // rax
  
  v1 = sub_2c3f7();
  *(unsigned int *)(v1 + 0x10) = 1;
  *(unsigned long *)(v1 + 0x18) = a0;
  return;
}


// Function: sub_2c8f7 @ 0x2c8f7
void sub_2c8f7(unsigned long a0)
{
  int8 v1; // rax
  
  v1 = sub_2c3f7();
  *(unsigned int *)(v1 + 0x10) = 2;
  *(unsigned long *)(v1 + 0x18) = a0;
  *(unsigned long *)(v1 + 0x20) = 0;
  *(unsigned long *)(v1 + 0x30) = 0;
  dat_9f6a0 = 2;
  return;
}


// Function: sub_2c94c @ 0x2c94c
void sub_2c94c(void)
{
  dat_9f710 = sub_60b5b(0x66);
  dat_9f718 = 100;
  sub_2c4b8();
  return;
}


// Function: sub_2c978 @ 0x2c978
void sub_2c978(void)
{
  free(dat_9f710);
  return;
}


// Function: sub_2c992 @ 0x2c992
char * sub_2c992(void)
{
  char *v1; // rax
  unsigned long *v2; // stack - 0x10
  
  v2 = dat_9f708;
  while( true ) {
    if (!v2) {
      v1 = gettext("command line");
      return v1;
    }
    if ((*(int4 *)&v2[2] == 2) && (v2[6])) break;
    v2 = (unsigned long *)*v2;
  }
  return (char *)v2[3];
}


// Function: sub_2c9f1 @ 0x2c9f1
unsigned long sub_2c9f1(char *a0)
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long *v5; // rax
  int8 v6; // stack - 0xb8
  unsigned long *v7; // stack - 0xc8
  int8 v8; // stack - 0xb0
  
  if (stat(a0,&v6))
    sub_3e9dd(a0); // no-return
  v3 = sub_2c992();
  v7 = dat_9f720;
  while( true ) {
    if (!v7) {
      v5 = (unsigned long *)sub_60b5b(0x20);
      *v5 = dat_9f720;
      v5[1] = v8;
      v5[2] = v6;
      v5[3] = v3;
      dat_9f720 = v5;
      return 0;
    }
    if ((v7[1] == v8) && (v7[2] == v6)) break;
    v7 = (unsigned long *)*v7;
  }
  v2 = sub_5c9cd(0,0x3a,1);
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v1 = v7[3];
  v4 = sub_5e285(0,a0);
  error(0,0,gettext("%s: file list requested from %s already read from %s"),v4,v3,v1);
  dat_9fdd0 = 2;
  sub_5c9cd(0,0x3a,v2);
  return 1;
}


// Function: sub_2cbf6 @ 0x2cbf6
void sub_2cbf6(char *a0)
{
  char *v1;
  char *v2; // stack - 0x10
  
  v2 = &a0[strlen(a0) - 1];
  while ((a0 < v2 && (*v2 == '/'))) {
    v1 = &v2[-1];
    *v2 = '\0';
    v2 = v1;
  }
  return;
}


// Function: sub_2cc4a @ 0x2cc4a
unsigned long sub_2cc4a(int8 a0) // return-dupe x2
{
  int8 v1;
  int4 v2;
  void *v3;
  int4 v4; // stack - 0x20
  int8 v5; // stack - 0x18
  
  v5 = 0;
  v3 = *(void **)(a0 + 0x30);
  v2 = *(int4 *)(a0 + 0x28);
  *(int8 *)(a0 + 0x20) = *(int8 *)(a0 + 0x20) + 1;
  v4 = getc_unlocked(v3);
  while ((v4 != -1 && (v4 != v2))) {
    if (v5 == dat_9f718)
      dat_9f710 = sub_60bd8(dat_9f710,0x9f718);
    v1 = v5 + 1;
    *(char *)(v5 + dat_9f710) = (char)v4;
    if (!v4)
      return 2;
    v4 = getc_unlocked(v3);
    v5 = v1;
  }
  if ((!v5) && (v4 != -1))
    return 3;
  if (v5 == dat_9f718)
    dat_9f710 = sub_60bd8(dat_9f710,0x9f718);
  *(char *)(v5 + dat_9f710) = 0;
  sub_2cbf6(dat_9f710);
  if ((!v5) && (v4 == -1))
    return 1;
  return 0;
}


// Function: sub_2cd8e @ 0x2cd8e
unsigned long sub_2cd8e(char *a0,int8 a1) // return-dupe
{
  unsigned long v1; // rax
  int8 v2; // stack - 0x108
  unsigned int v3 [2]; // stack - 0x128
  char *v4; // stack - 0x140
  int4 v5; // stack - 0x12c
  unsigned long v6; // stack - 0x120
  unsigned long v7; // stack - 0x118
  unsigned long *v8; // stack - 0x100
  int8 v9; // stack - 0xf8
  
  v4 = a0;
  while ((*v4 && (*(uint2 *)((int8)*v4 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000))) {
    v4 = &v4[1];
  }
  if (*v4 != '-')
    return 1;
  v9 = 1;
  if (!sub_461a6(v4,&v2,0x2000e46)) {
    *v8 = dat_9fe48;
    v3[0] = 2;
    v6 = *(unsigned long *)(a1 + 0x18);
    v7 = *(unsigned long *)(a1 + 0x20);
    sub_3828e((int4)v9 + (int4)v2,v8,v3);
    for (v5 = 0; (uint8)(int8)v5 < (uint8)(v9 + v2); v5 = v5 + 1) {
      v8[v5] = 0;
    }
    sub_46399(&v2);
    return 0;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v1 = sub_4647f(&v2);
  error(0,0,gettext("cannot split string \'%s\': %s"),v4,v1);
  sub_177f4(); // no-return
}


// Function: sub_2cf89 @ 0x2cf89
unsigned long sub_2cf89(int8 a0,int8 a1) // return-dupe x3
{
  char *v1;
  unsigned long v2;
  int4 v3; // eax
  
  if (!*(int8 *)(a0 + 0x30)) {
    v1 = *(char **)(a0 + 0x18);
    if (strcmp(v1,"-")) { // branch-flip
      v2 = *(unsigned long *)(a0 + 0x18);
      if (sub_2c9f1(v2)) {
        sub_2c78a();
        return 1;
      }
      v1 = *(char **)(a0 + 0x18);
      *(void **)(a0 + 0x30) = fopen(v1,"r");
      if (!*(int8 *)(a0 + 0x30))
        sub_3e3b4(*(unsigned long *)(a0 + 0x18)); // no-return
    }
    else {
      sub_34a6f(0x8390c);
      *(unsigned long *)(a0 + 0x30) = stdin;
    }
    *(int4 *)(a0 + 0x28) = (int4)dat_9c270;
    *(char *)(a0 + 0x2c) = dat_9f6b8;
  }
  do {
    do {
      v3 = sub_2cc4a(a0);
    } while (v3 == 3);
    switch(v3) {
      case 0:
label_2d103:
        if (*(char *)(a0 + 0x2c) != '\x01') {
          if (dat_9fc28)
            sub_2961d(dat_9f710);
          v3 = sub_2cd8e(dat_9f710,a0);
          if (!v3) {
            sub_2c4b8();
            return 1;
          }
        }
        *(unsigned int *)(a1 + 0x10) = 0;
        *(unsigned long *)(a1 + 0x18) = dat_9f710;
        return 0;
      case 1:
        v1 = *(char **)(a0 + 0x18);
        if (strcmp(v1,"-"))
          fclose(*(void **)(a0 + 0x30));
        *(unsigned long *)(a0 + 0x30) = 0;
        sub_2c78a();
        return 1;
      case 2:
        if (dat_9ec84 & 0x200) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v2 = *(unsigned long *)(a0 + 0x18);
          error(0,0,"%s: file name read contains nul character",sub_5e548(v2));
        }
        *(unsigned int *)(a0 + 0x28) = 0;
        goto label_2d103;
      
    }
  } while( true );
}


// Function: sub_2d1c1 @ 0x2d1c1
void sub_2d1c1(int8 a0)
{
  char *v1;
  uint8 v2; // rax
  
  v1 = *(char **)(a0 + 0x18);
  v2 = strlen(v1);
  while (dat_9f718 <= v2) {
    dat_9f710 = (char *)sub_60bd8(dat_9f710,0x9f718);
  }
  strcpy(dat_9f710,v1);
  sub_2cbf6(dat_9f710);
  return;
}


// Function: sub_2d244 @ 0x2d244
unsigned long sub_2d244(int4 a0) // return-dupe x2
{
  int8 v1;
  int4 v2; // eax
  unsigned long v3; // rax
  
  do {
    v1 = dat_9f708;
    if (!dat_9f708) {
      sub_2c588();
      return 0;
    }
    switch(*(unsigned int *)(dat_9f708 + 0x10)) {
      case 0:
label_2d2ec:
        sub_2d1c1(dat_9f708);
        if (dat_9fc28)
          sub_2961d(dat_9f710);
        dat_9f750 = *(unsigned int *)(v1 + 0x10);
        dat_9f758 = dat_9f710;
        sub_2c78a();
        return 0x9f740;
      case 1:
        if (!a0) goto label_2d2ec;
        v3 = sub_60cc6(*(unsigned long *)(dat_9f708 + 0x18));
        sub_2adf1(sub_2ac09(v3));
        sub_2c78a();
        break;
      case 2:
        v2 = sub_2cf89(dat_9f708,0x9f740);
        if (!v2)
          return 0x9f740;
        break;
      case 3:
        sub_2c78a();
        break;
      case 4:
        sub_2bc3a(*(unsigned int *)(dat_9f708 + 0x18),*(unsigned long *)(dat_9f708 + 0x20));
        sub_2c78a();
      
    }
  } while( true );
}


// Function: sub_2d37c @ 0x2d37c
unsigned long sub_2d37c(unsigned int a0) // early-return
{
  int8 v1; // rax
  
  v1 = sub_2d244(a0);
  if (v1)
    return *(unsigned long *)(v1 + 0x18);
  return 0;
}


// Function: sub_2d3b1 @ 0x2d3b1
void sub_2d3b1(void) // return-dupe x4, ternary
{
  int4 v1;
  int8 v2; // rax
  unsigned long v3;
  uint4 v4; // edx
  int4 v5; // stack - 0x18
  
  if (dat_9f958) {
    while ((v2 = sub_2d244(0), v2 && (*(int4 *)(v2 + 0x10) == 1))) {
      v3 = *(unsigned long *)(v2 + 0x18);
      dat_9f778 = sub_2ac09(sub_60cc6(v3));
    }
    if (v2) {
      sub_2c3b1(dat_9f780);
      dat_9f780 = (unsigned long *)sub_2c34c(*(unsigned long *)(v2 + 0x18));
      *(int4 *)&dat_9f780[5] = dat_9f778;
      *dat_9f780 = 0;
      dat_9f780[6] = 0;
      v4 = (dat_9f6bc != 2) ? 0 : 0x10000000; // branch-flip
      *(uint4 *)&dat_9f780[4] = dat_9f930 | v4 | dat_9c274 | dat_9f6c0;
      dat_9f780[7] = 0;
      dat_9f780[8] = 0;
      *(char *)((int8)dat_9f780 + 0x24) = 1;
      dat_9f700 = dat_9f780;
      dat_9f6f8 = dat_9f780;
      return;
    }
    if (!dat_9f778)
      return;
    sub_2d5b5(0,dat_9f778,0,0);
    return;
  }
  v5 = 0;
  v1 = v5;
  while( true ) {
    while ((v2 = sub_2d244(0), v2 && (*(int4 *)(v2 + 0x10) == 1))) {
      v3 = sub_60cc6(*(unsigned long *)(v2 + 0x18));
      v5 = sub_2ac09(v3);
    }
    if (!v2) break;
    sub_2d5b5(*(unsigned long *)(v2 + 0x18),v5,1,0);
    v1 = v5;
  }
  if (v5 == v1)
    return;
  sub_2d5b5(0,v5,0,0);
  return;
}


// Function: sub_2d5b5 @ 0x2d5b5
unsigned long * sub_2d5b5(unsigned long a0,unsigned int a1,char a2,unsigned long a3) // ternary
{
  unsigned long *v1; // rax
  uint4 v2; // edx
  
  v1 = (unsigned long *)sub_2c34c(a0);
  v1[1] = dat_9f700;
  *v1 = 0;
  v1[6] = 0;
  v2 = (dat_9f6bc != 2) ? 0 : 0x10000000; // branch-flip
  *(uint4 *)&v1[4] = v2 | dat_9c274 | dat_9f6c0 | dat_9f930;
  *(unsigned int *)&v1[5] = a1;
  v1[7] = 0;
  v1[8] = a3;
  *(char *)((int8)v1 + 0x24) = a2;
  if (dat_9f700) // branch-flip
    *dat_9f700 = v1;
  else {
    dat_9f6f8 = v1;
  }
  dat_9f700 = v1;
  return v1;
}


// Function: sub_2d6a2 @ 0x2d6a2
void sub_2d6a2(unsigned long a0) // ternary
{
  unsigned long v1;
  unsigned long *v2; // rax
  uint4 v3; // edx
  
  v2 = (unsigned long *)sub_2c34c(a0);
  v1 = dat_9f6f8;
  if (dat_9f988) {
    sub_2d817(dat_9f6f8);
    sub_2c3b1(v1);
  }
  v2[1] = 0;
  *v2 = dat_9f6f8;
  dat_9f6f8 = v2;
  if (!dat_9f700)
    dat_9f700 = v2;
  v2[6] = 0;
  v3 = (dat_9f6bc != 2) ? 0 : 0x10000000; // branch-flip
  *(uint4 *)&v2[4] = v3 | dat_9c274 | dat_9f6c0 | dat_9f930;
  *(unsigned int *)&v2[5] = 0;
  v2[7] = 0;
  v2[8] = 0;
  *(char *)((int8)v2 + 0x24) = 1;
  dat_9f988 = 1;
  return;
}


// Function: sub_2d7a6 @ 0x2d7a6
unsigned long * sub_2d7a6(unsigned long a0)
{
  unsigned int v1;
  unsigned long v2;
  unsigned long *v3; // stack - 0x10
  
  v3 = dat_9f6f8;
  while( true ) {
    if (!v3)
      return NULL;
    if ((*(char *)v3[2]) && (v1 = *(unsigned int *)&v3[4], v2 = v3[2], sub_4ead5(v2,a0,v1))) break;
    v3 = (unsigned long *)*v3;
  }
  return v3;
}


// Function: sub_2d817 @ 0x2d817
void sub_2d817(int8 *a0) // return-dupe
{
  if ((int8 *)a0[1]) // branch-flip
    *(int8 *)a0[1] = *a0;
  else {
    dat_9f6f8 = *a0;
  }
  if (*a0) {
    *(int8 *)(*a0 + 8) = a0[1];
    return;
  }
  dat_9f700 = a0[1];
  return;
}


// Function: sub_2d88a @ 0x2d88a
unsigned long sub_2d88a(char *a0) // early-return x4
{
  uint8 v1; // rax
  unsigned long v2; // rax
  int8 v3; // stack - 0x18
  
  v1 = strlen(a0);
  while( true ) {
    if (!dat_9f6f8)
      return 1;
    if (!**(char **)(dat_9f6f8 + 0x10)) break;
    v3 = sub_2d7a6(a0,v1);
    if (dat_9f988) {
      if (v3 != dat_9f6f8) // branch-flip
        v3 = 0;
      else {
        dat_9f988 = '\0';
      }
    }
    if (v3) {
      if (((a0[*(int8 *)(v3 + 0x18)] != '/') || (!dat_9f930)) || (!*(int8 *)(v3 + 0x30)))
        *(int8 *)(v3 + 0x30) = *(int8 *)(v3 + 0x30) + 1;
      sub_2adf1(*(unsigned int *)(v3 + 0x28));
      if (dat_9f8d0) // branch-flip
        v2 = CONCAT71((undefined7)((uint8)dat_9f8d0 >> 8),*(int8 *)(v3 + 0x30) == dat_9f8d0);
      else {
        v2 = CONCAT71((undefined7)((uint8)*(int8 *)(v3 + 0x30) >> 8),*(int8 *)(v3 + 0x30) != 0);
      }
      return v2;
    }
    if ((!dat_9f958) || (!*(int8 *)(dat_9f6f8 + 0x30)))
      return 0;
    sub_2d3b1();
    if (*(int8 *)(dat_9f6f8 + 0x30))
      return 0;
  }
  sub_2adf1(*(unsigned int *)(dat_9f6f8 + 0x28));
  dat_9f6f8 = 0;
  dat_9f700 = 0;
  return 1;
}


// Function: sub_2da15 @ 0x2da15
unsigned long sub_2da15(int8 a0) // return-dupe x2
{
  uint8 v1; // rax
  unsigned long *v2; // stack - 0x18
  
  if (((*(int8 *)(a0 + 8)) && (dat_9f8d0)) && (!*(char *)(a0 + 0x10))) {
    v1 = strlen(*(char **)(a0 + 8));
    v2 = dat_9f6f8;
    while( true ) {
      if (!v2)
        return 1;
      if (*(char *)v2[2]) {
        if (dat_9f8d0) { // branch-flip
          if ((uint8)v2[6] < dat_9f8d0)
            return 0;
        }
        else if (!v2[6])
          return 0;
      }
      if (((uint8)v2[3] <= v1) && (*(char *)(v2[3] + *(int8 *)(a0 + 8)) == '/')) break;
      v2 = (unsigned long *)*v2;
    }
    return 0;
  }
  return 0;
}


// Function: sub_2dafb @ 0x2dafb
unsigned int sub_2dafb(unsigned long a0)
{
  if (!dat_9f6bc) {
    if (sub_4e397(a0,0)) {
      dat_9f788 = 1;
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Pattern matching characters used in file names"));
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning"));
    }
  }
  return dat_9f788;
}


// Function: sub_2dbba @ 0x2dbba
void sub_2dbba(void) // return-dupe
{
  char *v1; // rax
  unsigned long v2; // rax
  int8 v3; // rax
  bool v4;
  unsigned long *v5; // stack - 0x28
  
  for (v5 = dat_9f6f8; v5; v5 = (unsigned long *)*v5) {
    if (dat_9f8d0) // branch-flip
      v4 = (uint8)v5[6] < dat_9f8d0;
    else {
      v4 = v5[6] == 0;
    }
    if ((v4) && (*(char *)v5[2])) {
      sub_2dafb(v5[2]);
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v2 = sub_5e548(v5[2]);
      if (v5[6]) // branch-flip
        v1 = gettext("%s: Required occurrence not found in archive");
      else {
        v1 = gettext("%s: Not found in archive");
      }
      error(0,0,v1,v2);
      dat_9fdd0 = 2;
    }
  }
  dat_9f6f8 = NULL;
  dat_9f700 = 0;
  if (!dat_9f958) {
    dat_9f700 = 0;
    dat_9f6f8 = NULL;
    return;
  }
  while (v3 = sub_2d37c(1), v3) {
    sub_2dafb(v3);
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = sub_5e548(v3);
    error(0,0,gettext("%s: Not found in archive"),v2);
    dat_9fdd0 = 2;
  }
  return;
}


// Function: sub_2dd64 @ 0x2dd64
void sub_2dd64(void) // return-dupe x4
{
  unsigned long v1;
  int8 v2; // rax
  bool v3;
  unsigned long *v4; // stack - 0x18
  
  if (!dat_9f6f8)
    return;
  for (v4 = dat_9f6f8; v4; v4 = (unsigned long *)*v4) {
    if (dat_9f8d0) // branch-flip
      v3 = dat_9f8d0 <= (uint8)v4[6];
    else {
      v3 = v4[6] != 0;
    }
    if (v3)
      return;
  }
  if (dat_9f9ac)
    error(0,0,gettext("Archive label mismatch"));
  sub_39e98(1);
  v4 = dat_9f6f8;
  while ((v4 && (v1 = v4[2], !sub_2dafb(v1)))) {
    v4 = (unsigned long *)*v4;
  }
  dat_9f6f8 = NULL;
  dat_9f700 = 0;
  if (!dat_9f958) {
    dat_9f700 = 0;
    dat_9f6f8 = NULL;
    return;
  }
  do {
    v2 = sub_2d37c(1);
    if (!v2)
      return;
  } while (!sub_2dafb(v2));
  return;
}


// Function: sub_2de97 @ 0x2de97
unsigned long *** sub_2de97(unsigned long ***a0,int4 a1,code *a2)
{
  unsigned long ***v1;
  unsigned long ***v10; // stack - 0x20
  unsigned long ***v11; // stack - 0x18
  int4 v2;
  int4 v3;
  int4 v4; // eax
  unsigned long ***v5; // rax
  unsigned long ***v6; // stack - 0x38
  int4 v7; // stack - 0x44
  unsigned long ***v8; // stack - 0x30
  unsigned long ***v9; // stack - 0x28
  
  v5 = a0;
  if (a1 != 1) {
    if (a1 != 2) { // branch-flip
      v4 = a1 + 1;
      v4 /= 2;
      v2 = v4;
      v4 = a1 / 2;
      v3 = v4;
      v11 = a0;
      v7 = v2;
      while (v7 = v7 + -1, v7) {
        v11 = (unsigned long ***)*v11;
      }
      v9 = (unsigned long **)*v11;
      *v11 = NULL;
      v8 = a0;
      v5 = (unsigned long ***)sub_2de97(a0,v2,a2);
      v8 = v5;
      v5 = (unsigned long ***)sub_2de97(v9,v3,a2);
      v9 = v5;
      v10 = &v6;
      while ((v8 && (v9))) {
        v4 = (*a2)(v8,v9);
        if (0 <= v4) { // branch-flip
          v1 = (unsigned long ***)*v9;
          *v10 = v9;
          v10 = v9;
          v9 = v1;
          v11 = v1;
        }
        else {
          v1 = (unsigned long ***)*v8;
          *v10 = v8;
          v10 = v8;
          v8 = v1;
          v11 = v1;
        }
      }
      if (v8) // branch-flip
        *v10 = v8;
      else {
        *v10 = v9;
      }
      v5 = v6;
    }
    else {
      v4 = (*a2)(a0,*a0);
      if (v4 > 0) {
        v5 = (unsigned long ***)*a0;
        *v5 = a0;
        *a0 = NULL;
      }
    }
  }
  return v5;
}


// Function: sub_2e071 @ 0x2e071
unsigned long * sub_2e071(unsigned long a0,unsigned int a1,unsigned long a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  v1 = (unsigned long *)sub_2de97(a0,a1,a2);
  v3 = NULL;
  for (v2 = v1; v2; v2 = (unsigned long *)*v2) {
    v2[1] = v3;
    v3 = v2;
  }
  return v1;
}


// Function: sub_2e0df @ 0x2e0df
uint8 sub_2e0df(int8 a0,int8 a1)
{
  uint4 v1; // eax
  uint8 v2; // rax
  bool v3;
  
  if (dat_9f8d0) // branch-flip
    v3 = dat_9f8d0 <= *(uint8 *)(a1 + 0x30);
  else {
    v3 = *(int8 *)(a1 + 0x30) != 0;
  }
  v1 = (uint4)v3;
  if (dat_9f8d0) // branch-flip
    v3 = dat_9f8d0 <= *(uint8 *)(a0 + 0x30);
  else {
    v3 = *(int8 *)(a0 + 0x30) != 0;
  }
  v1 -= v3;
  if (v1) // branch-flip
    v2 = (uint8)v1;
  else {
    v1 = strcmp(*(char **)(a0 + 0x10),*(char **)(a1 + 0x10));
  }
  return v2;
}


// Function: sub_2e18e @ 0x2e18e
void sub_2e18e(int8 a0,int8 a1)
{
  strcmp(*(char **)(a0 + 0x10),*(char **)(a1 + 0x10));
  return;
}


// Function: sub_2e1bf @ 0x2e1bf
void sub_2e1bf(int8 a0,int8 a1) // return-dupe
{
  char *v1;
  int8 v10; // stack - 0x210
  uint8 v11; // stack - 0x208
  char *v12; // stack - 0x200
  char *v13; // stack - 0x1f8
  char *v14; // stack - 0x1f0
  uint8 v15; // stack - 0x1e8
  int8 v16; // stack - 0x1e0
  int8 v17; // stack - 0x40
  int4 v18; // stack - 0x30
  unsigned int v2;
  int4 v3;
  uint4 v4; // eax
  uint8 v5; // stack - 0x220
  unsigned long v6 [11]; // stack - 0x1d8
  char v7 [320];
  int4 v8; // stack - 0x228
  int8 v9; // stack - 0x218
  
  *(unsigned long *)(a1 + 0x38) = sub_21eb9(a0);
  v14 = (char *)sub_2241b(*(unsigned long *)(a1 + 0x38));
  if (!v14)
    return;
  v9 = 0;
  v10 = 0;
  v11 = *(uint8 *)(a1 + 0x18);
  if (100 <= v11) // branch-flip
    v5 = v11 + 0x66;
  else {
    v5 = 0x66;
  }
  v12 = (char *)sub_60b5b(v5);
  v2 = *(unsigned int *)(a1 + 0x28);
  strcpy(v12,*(char **)(a1 + 0x10));
  if (v12[v11 - 1] != '/') {
    v12[v11] = '/';
    v12[v11 + 1] = '\0';
    v11 += 1;
  }
  for (v13 = v14; *v13; v13 = &v13[v15 + 1]) {
    v15 = strlen(v13);
    if (*v13 == 'D') {
      while (v5 < v11 + v15) {
        v12 = (char *)sub_60bd8(v12,&v5);
      }
      strcpy(&v12[v11],&v13[1]);
      v16 = sub_2d5b5(v12,v2,0,a1);
      if (v9) // branch-flip
        *(int8 *)(v10 + 0x50) = v16;
      else {
        v9 = v16;
      }
      v10 = v16;
      sub_39c55(v6);
      v17 = a0;
      if (0 <= *(int4 *)(a0 + 0x1a8)) { // branch-flip
        v4 = dat_9fc08 | 0x10000;
        v1 = &v13[1];
        v8 = sub_157f9(a0,v1,v4);
      }
      else {
        v8 = -1;
        v3 = *(int4 *)(a0 + 0x1a8);
        *__errno_location() = -v3;
      }
      if (0 <= v8) { // branch-flip
        v18 = v8;
        if (fstat(v8,v7)) // branch-flip
          sub_2b444(v12);
        else {
          v6[0] = sub_60cc6(v12);
          sub_2e1bf(v6,v16);
          sub_1588e(v6);
        }
      }
      else {
        sub_2b2b1(v12);
      }
      sub_39d1c(v6);
    }
  }
  free(v12);
  *(int8 *)(a1 + 0x48) = v9;
  return;
}


// Function: sub_2e590 @ 0x2e590
void sub_2e590(int8 a0,unsigned long a1)
{
  sub_52834(*(unsigned long *)(a0 + 0x58),a1);
  return;
}


// Function: sub_2e5c5 @ 0x2e5c5
unsigned long sub_2e5c5(int8 a0,int8 a1)
{
  int4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x58),*(char **)(a1 + 0x58));
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_2e60b @ 0x2e60b
void sub_2e60b(int8 a0,int8 a1)
{
  int8 v1;
  int8 v2;
  char *v3;
  int8 v4; // rax
  char *v5; // rax
  int8 v6; // stack - 0x40
  
  v1 = *(int8 *)(*(int8 *)(a0 + 0x40) + 0x18);
  v2 = *(int8 *)(a1 + 0x18);
  v3 = *(char **)(a1 + 0x10);
  for (v6 = a0; v6; v6 = *(int8 *)(v6 + 0x50)) {
    v4 = v2 + (*(int8 *)(v6 + 0x18) - v1);
    v5 = (char *)sub_60b5b(v4 + 1);
    strcpy(v5,v3);
    strcat(v5,(char *)(*(int8 *)(v6 + 0x10) + v1));
    free(*(void **)(v6 + 0x10));
    *(char **)(v6 + 0x10) = v5;
    *(int8 *)(v6 + 0x18) = v4;
    sub_21244(*(unsigned long *)(v6 + 0x38),*(unsigned long *)(*(int8 *)(v6 + 0x40) + 0x10),v1,v3,v2);
  }
  return;
}


// Function: sub_2e719 @ 0x2e719
void sub_2e719(void) // return-dupe x2
{
  unsigned int v1;
  unsigned long *v10; // stack - 0x1f0
  unsigned long *v11; // stack - 0x1e8
  uint4 v12; // stack - 0x158
  int4 v13; // stack - 0x20
  unsigned long *v2;
  unsigned long v3;
  int4 v4; // eax
  unsigned long v5;
  int8 v6; // rax
  unsigned long v7 [11]; // stack - 0x1c8
  char v8 [24];
  int4 v9; // stack - 0x1f8
  
  v11 = NULL;
  sub_2d3b1();
  if (!dat_9f6f8)
    sub_2d5b5(0x83a3c,0,0,0);
  if (dat_9f8e0) {
    v4 = sub_2abe0();
    if (v4) {
      if (v4 != 1) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Only one -C option is allowed with --listed-incremental"));
        sub_3814c(2); // no-return
      }
      if (!*(int4 *)&dat_9f6f8[5]) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Using -C option inside file list is not allowed with --listed-incremental"));
        sub_3814c(2); // no-return
      }
    }
    sub_24090();
  }
  v9 = 0;
  for (v10 = dat_9f6f8; v10; v10 = (unsigned long *)*v10) {
    if ((((!v10[6]) && (!v10[7])) && (!(*(uint4 *)&v10[4] & 0x10000000))) && (sub_2adf1(*(unsigned int *)&v10[5]), *(char *)v10[2])) {
      sub_39c55(v7);
      v5 = v10[2];
      if (sub_2a9b8(v5,v8)) // branch-flip
        sub_2b444(v10[2]);
      else {
        if ((v12 & 0xf000) == 0x4000) {
          v4 = openat(dat_9c268,(char *)v10[2],dat_9fc08 | 0x10000);
          if (0 <= v4) { // branch-flip
            v13 = v4;
            if (fstat(v4,v8)) // branch-flip
              sub_2b444(v10[2]);
            else {
              v5 = v10[2];
              v7[0] = sub_60cc6(v5);
              v10[6] = v10[6] + 1;
              sub_2e1bf(v7,v10);
            }
          }
          else {
            sub_2b2b1(v10[2]);
          }
        }
        sub_39d1c(v7);
      }
    }
    v9 += 1;
  }
  dat_9f6f8 = (unsigned long *)sub_2e071(dat_9f6f8,v9,sub_2e18e);
  v9 = 0;
  v5 = sub_52c1c(0,0,sub_2e590,sub_2e5c5,0);
  v10 = dat_9f6f8;
  do {
    if (!v10) {
      dat_9f700 = v11;
      sub_52e9c(v5);
      dat_9f6f8 = (unsigned long *)sub_2e071(dat_9f6f8,v9,sub_2e0df);
      if (!dat_9f8e0)
        return;
      for (v10 = dat_9f6f8; (v10 && (!*(char *)v10[2])); v10 = (unsigned long *)*v10) {
      }
      if (!v10)
        return;
      sub_228cd(v10[7]);
      return;
    }
    v2 = (unsigned long *)*v10;
    v3 = v10[2];
    v1 = *(unsigned int *)&v10[5];
    v10[0xb] = sub_29aac(v1,v3);
    if ((v11) && (v6 = sub_52567(v5,v10), v6)) {
      if (!v10[8]) {
        if (*(int8 *)(v6 + 0x48))
          sub_2e60b(*(unsigned long *)(v6 + 0x48),v10);
        sub_539cd(v5,v10);
        sub_2d817(v6);
        sub_2c3b1(v6);
        v9 -= 1;
        goto label_2ebec;
      }
      if (v10[9])
        sub_2e60b(v10[9],v6);
      sub_2d817(v10);
      sub_2c3b1(v10);
    }
    else {
label_2ebec:
      v10[6] = 0;
      if (!sub_5395d(v5,v10))
        sub_1783c(); // no-return
      v11 = v10;
      v9 += 1;
    }
    v10 = v2;
  } while( true );
}


// Function: sub_2ed0e @ 0x2ed0e
int8 sub_2ed0e(char *a0) // early-return x2
{
  uint8 v1; // rax
  int8 v2; // rax
  
  v1 = strlen(a0);
  while( true ) {
    v2 = sub_2d7a6(a0,v1);
    if (v2)
      return v2;
    if (((!dat_9f958) || (!dat_9f6f8)) || (!*(int8 *)(dat_9f6f8 + 0x30))) break;
    sub_2d3b1();
    if (*(int8 *)(dat_9f6f8 + 0x30))
      return 0;
  }
  return 0;
}


// Function: sub_2eda0 @ 0x2eda0
unsigned long * sub_2eda0(void) // early-return
{
  if (!dat_9f6b0)
    dat_9f6b0 = dat_9f6f8;
  for (; (dat_9f6b0 && ((dat_9f6b0[6] || (!*(char *)dat_9f6b0[2])))); dat_9f6b0 = (unsigned long *)*dat_9f6b0) {
  }
  if (dat_9f6b0) {
    dat_9f6b0[6] = dat_9f6b0[6] + 1;
    sub_2adf1(*(unsigned int *)&dat_9f6b0[5]);
    return dat_9f6b0;
  }
  return NULL;
}


// Function: sub_2ee43 @ 0x2ee43
void sub_2ee43(void)
{
  unsigned long *v1; // stack - 0x10
  
  dat_9f6b0 = 0;
  for (v1 = dat_9f6f8; v1; v1 = (unsigned long *)*v1) {
    v1[6] = 0;
  }
  return;
}


// Function: sub_2ee85 @ 0x2ee85
void * sub_2ee85(char *a0,char *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_2ee85
{
  int4 v1; // eax
  uint8 v2; // rax
  uint8 v3; // rax
  void *v4; // rax
  
  v2 = strlen(a0);
  v3 = strlen(a1) + 1;
  v1 = ((v2) && (a0[v2 - 1] != '/'));
  v4 = (void *)sub_60b5b(v3 + (int8)v1 + v2);
  memcpy(v4,a0,v2);
  *(char *)(v2 + (int8)v4) = 0x2f;
  memcpy((void *)((int8)v1 + v2 + (int8)v4),a1,v3);
  return v4;
}


// Function: sub_2ef5c @ 0x2ef5c
int8 sub_2ef5c(char *a0,int8 a1) // early-return
{
  char v1;
  int8 v2; // stack - 0x28
  char *v3; // stack - 0x10
  
  for (v3 = a0; v2 = a1, *v3 == '/'; v3 = &v3[1]) {
  }
  while( true ) {
    do {
      if (!*v3)
        return -1;
      v1 = *v3;
      v3 = &v3[1];
    } while (v1 != '/');
    v2 -= 1;
    if (!v2) break;
    for (; *v3 == '/'; v3 = &v3[1]) {
    }
  }
  return (int8)v3 - (int8)a0;
}


// Function: sub_2efdc @ 0x2efdc
unsigned long sub_2efdc(char *a0)
{
  char *v1;
  char v2;
  char *v3; // stack - 0x10
  
  for (v3 = a0; ((*v3 != '.' || (v3[1] != '.')) || ((v3[2] != '/' && (v3[2])))); v3 = &v3[1]) {
    while (*v3 != '/') {
      v1 = &v3[1];
      v2 = *v3;
      v3 = v1;
      if (!v2)
        return 0;
    }
  }
  return 1;
}


// Function: sub_2f05d @ 0x2f05d
unsigned long sub_2f05d(int4 *a0,int8 a1) // return-dupe
{
  uint8 v1; // rax
  int8 v2; // rax
  
  if (a1 < *(int8 *)&a0[2]) {
    *__errno_location() = 0x16;
    return 0;
  }
  while( true ) {
    if (a1 <= *(int8 *)&a0[2])
      return 1;
    v1 = a1 - *(int8 *)&a0[2];
    if (0x201 <= (int8)v1)
      v1 = 0x200;
    v2 = write(*a0,(void *)0x83d00,v1);
    if (v2 <= 0) break;
    *(int8 *)&a0[2] = *(int8 *)&a0[2] + v2;
  }
  if (v2)
    return 0;
  *__errno_location() = 0x16;
  return 0;
}


// Function: sub_2f126 @ 0x2f126
unsigned long sub_2f126(int8 a0) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x10)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x10);
    return (*v1)(a0);
  }
  return 0;
}


// Function: sub_2f165 @ 0x2f165
unsigned long sub_2f165(void *a0) // early-return x2
{
  code *v1;
  
  memset(a0,0,0x30);
  if (sub_2fa5c(a0) != '\x01')
    return 0;
  if (**(int8 **)((int8)a0 + 0x20)) {
    v1 = (code *)**(unsigned long **)((int8)a0 + 0x20);
    return (*v1)(a0);
  }
  return 1;
}


// Function: sub_2f1d2 @ 0x2f1d2
unsigned long sub_2f1d2(int8 a0) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 8)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 8);
    return (*v1)(a0);
  }
  return 1;
}


// Function: sub_2f211 @ 0x2f211
unsigned long sub_2f211(int8 a0,unsigned int a1,unsigned long a2) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x30)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x30);
    return (*v1)(a0,a1,a2);
  }
  return 1;
}


// Function: sub_2f261 @ 0x2f261
unsigned long sub_2f261(int8 a0,unsigned long a1) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x38)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x38);
    return (*v1)(a0,a1);
  }
  return 0;
}


// Function: sub_2f2ab @ 0x2f2ab
unsigned long sub_2f2ab(int8 a0,unsigned long a1) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x40)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x40);
    return (*v1)(a0,a1);
  }
  return 0;
}


// Function: sub_2f2f5 @ 0x2f2f5
unsigned long sub_2f2f5(int8 a0) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x18)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x18);
    return (*v1)(a0);
  }
  return 0;
}


// Function: sub_2f334 @ 0x2f334
unsigned long sub_2f334(int8 a0) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x28)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x28);
    return (*v1)(a0);
  }
  return 1;
}


// Function: sub_2f373 @ 0x2f373
unsigned long sub_2f373(int8 a0) // early-return
{
  code *v1;
  
  if (*(int8 *)(*(int8 *)(a0 + 0x20) + 0x20)) {
    v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x20);
    return (*v1)(a0);
  }
  return 1;
}


// Function: sub_2f3b2 @ 0x2f3b2
bool sub_2f3b2(unsigned int *a0,unsigned long a1)
{
  uint1 v1; // al
  
  if (*(char *)&a0[1]) { // branch-flip
    lseek(*a0,a1,0);
    v1 = dat_7 >> 7;
  }
  else {
    v1 = sub_2f05d(a0,a1) ^ 1;
  }
  if (v1)
    sub_2b3eb(**(unsigned long **)&a0[6],a1);
  return v1 == 0;
}


// Function: sub_2f433 @ 0x2f433
unsigned long sub_2f433(char *a0,int8 a1)
{
  char *v1;
  int8 v2;
  char v3;
  int8 v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  v4 = a1;
  v5 = a0;
  do {
    v2 = v4 + -1;
    if (!v4)
      return 1;
    v1 = &v5[1];
    v3 = *v5;
    v4 = v2;
    v5 = v1;
  } while (!v3);
  return 0;
}


// Function: sub_2f477 @ 0x2f477
void sub_2f477(int8 a0,unsigned long *a1)
{
  unsigned long *v1;
  int8 v2;
  unsigned long v3;
  int8 v4; // stack - 0x18
  
  v4 = *(int8 *)(a0 + 0x140);
  v2 = *(int8 *)(a0 + 0x130);
  if (v2 == *(int8 *)(a0 + 0x138)) {
    v4 = sub_60a5f(v4,a0 + 0x138,0x10);
    *(int8 *)(a0 + 0x140) = v4;
  }
  v1 = (unsigned long *)(v2 * 0x10 + v4);
  v3 = a1[1];
  *v1 = *a1;
  v1[1] = v3;
  *(int8 *)(a0 + 0x130) = v2 + 1;
  return;
}


// Function: sub_2f527 @ 0x2f527
unsigned long sub_2f527(unsigned int *a0) // early-return
{
  unsigned int v1;
  int8 v2;
  int8 v3; // rax
  char v4 [520];
  int8 v5; // stack - 0x228
  int8 v6; // stack - 0x240
  int8 v7; // stack - 0x220
  
  v2 = *(int8 *)&a0[6];
  v1 = *a0;
  v6 = 0;
  v5 = 0;
  v7 = 0;
  *(unsigned long *)(v2 + 0x118) = 0;
  if (sub_2f211(a0,0,0) != '\x01')
    return 0;
  do {
    v3 = sub_2a9e9(v1,v4,0x200);
    if ((!v3) || (v3 == -1)) {
      if (!v7)
        v5 = v6;
      sub_2f477(v2,&v5);
      *(int8 *)(v2 + 0x118) = v3 + *(int8 *)(v2 + 0x118);
      return sub_2f211(a0,2,0);
    }
    if (sub_2f433(v4,v3)) { // branch-flip
      if (v7) {
        sub_2f477(v2,&v5);
        v7 = 0;
        if (sub_2f211(a0,1,0) != '\x01')
          return 0;
      }
    }
    else {
      if (!v7)
        v5 = v6;
      v7 = v3 + v7;
      *(int8 *)(v2 + 0x118) = v3 + *(int8 *)(v2 + 0x118);
      if (sub_2f211(a0,1,v4) != '\x01')
        return 0;
    }
    v6 = v3 + v6;
  } while( true );
}


// Function: sub_2f7c6 @ 0x2f7c6
bool sub_2f7c6(int8 a0)
{
  int8 v1;
  unsigned long v2; // stack - 0x28
  bool v3; // zf
  unsigned long v4; // stack - 0x20
  
  v1 = *(int8 *)(a0 + 0x18);
  v4 = 0;
  v3 = *(int8 *)(v1 + 0x98) == 0;
  if (v3) {
    *(unsigned long *)(v1 + 0x118) = 0;
    v2 = *(unsigned long *)(v1 + 0x88);
    sub_2f477(v1,&v2);
  }
  return v3;
}


// Function: sub_2f864 @ 0x2f864
unsigned long sub_2f864(unsigned int *a0) // return-dupe
{
  unsigned int v1;
  int8 v2;
  int8 v3; // rax
  int8 v4; // rax
  int8 v5; // stack - 0x28
  int8 v6; // stack - 0x48
  int8 v7; // stack - 0x20
  
  v2 = *(int8 *)&a0[6];
  v1 = *a0;
  v5 = 0;
  v7 = 0;
  v6 = 0;
  *(unsigned long *)(v2 + 0x118) = 0;
  while( true ) {
    v3 = lseek(v1,v6,3);
    if (v3 == -1) {
      if (*__errno_location() == 6) {
        v7 = 0;
        v5 = *(int8 *)(v2 + 0x88);
        sub_2f477(v2,&v5);
        return 1;
      }
      return 0;
    }
    v4 = lseek(v1,v3,4);
    if (((!v6) && (!v3)) && (v4 == *(int8 *)(v2 + 0x88))) break;
    v5 = v3;
    v7 = v4 - v3;
    sub_2f477(v2,&v5);
    *(int8 *)(v2 + 0x118) = *(int8 *)(v2 + 0x118) + v7;
    v6 = v4;
  }
  lseek(v1,0,0);
  return 0;
}


// Function: sub_2f9e5 @ 0x2f9e5
unsigned long sub_2f9e5(unsigned long a0) // return-dupe x3, return-dupe
{
  if (sub_2f7c6(a0))
    return 1;
  switch(dat_9f984) {
    default:
      return 0;
    case 0:
    case 2:
      goto label_2fa27;
    case 1:
      goto label_2fa3e;
    
  }
label_2fa27:
  if (sub_2f864(a0))
    return 1;
label_2fa3e:
  if (!sub_2f527(a0))
    return 0;
  return 1;
}


// Function: sub_2fa5c @ 0x2fa5c
unsigned long sub_2fa5c(int8 a0) // return-dupe x2
{
  int4 v1;
  
  if (dat_9f468) // branch-flip
    v1 = dat_9f468;
  else {
    v1 = dat_9f864;
  }
  switch(v1) {
    default:
      return 0;
    case 1:
    case 3:
      return 0;
    case 2:
    case 6:
      *(unsigned long *)(a0 + 0x20) = 0x9a7c0;
      return 1;
    case 4:
      *(unsigned long *)(a0 + 0x20) = 0x9a880;
      return 1;
    case 5:
      *(unsigned long *)(a0 + 0x20) = 0x9a820;
      return 1;
    
  }
}


// Function: sub_2faf0 @ 0x2faf0
unsigned long sub_2faf0(int4 *a0,int8 a1) // return-dupe
{
  int4 v1;
  int8 v10; // stack - 0xe8
  int8 v11; // stack - 0x98
  int8 v2;
  int8 v3; // rax
  int8 v4; // rax
  unsigned long v5;
  unsigned long v6; // rax
  char v7 [48];
  char v8 [24];
  int8 v9; // stack - 0xf0
  
  v9 = *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8);
  v5 = *(unsigned long *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140));
  if (sub_2f3b2(a0,v5) != '\x01')
    return 0;
  while( true ) {
    if (v9 <= 0)
      return 1;
    v2 = v9;
    if (0x201 <= v9)
      v2 = 0x200;
    v3 = sub_bb4b();
    v4 = sub_5ec70(*a0,v3,v2);
    if (v4 == -1) break;
    if (!v4) {
      v1 = *a0;
      if (fstat(v1,v7)) // branch-flip
        v10 = *(int8 *)(*(int8 *)&a0[6] + 0x88) - ((*(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8) + *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140))) - v9);
      else {
        v10 = *(int8 *)(*(int8 *)&a0[6] + 0x88) - v11;
      }
      if (dat_9ec84 & 0x80) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = sub_54e5a(v10,v8);
        v6 = sub_5e548(**(unsigned long **)&a0[6]);
        error(0,0,(char *)ngettext("%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v10),v6,v5);
      }
      if (dat_9f8bc == '\x01')
        return 0;
      sub_39e98(1);
      return 0;
    }
    memset((void *)(v3 + v4),0,0x200 - v4);
    v9 -= v4;
    *(int8 *)&a0[4] = v4 + *(int8 *)&a0[4];
    sub_bba7(v3);
  }
  sub_2b2f8(**(unsigned long **)&a0[6],(*(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8) + *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140))) - v9,v2);
  return 0;
}


// Function: sub_2fe94 @ 0x2fe94
unsigned long sub_2fe94(unsigned int *a0,int8 a1) // return-dupe x2
{
  unsigned int v1;
  unsigned long v2;
  int8 v3;
  int8 v4; // rax
  int8 v5; // stack - 0x28
  
  v2 = *(unsigned long *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140));
  if (sub_2f3b2(a0,v2) != '\x01')
    return 0;
  v5 = *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8);
  if (v5) { // branch-flip
    while (1 <= v5) {
      v3 = v5;
      if (0x201 <= v5)
        v3 = 0x200;
      v4 = sub_bb4b();
      if (!v4) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Unexpected EOF in archive"));
        dat_9fdd0 = 2;
        return 0;
      }
      sub_bba7(v4);
      *(int8 *)&a0[4] = *(int8 *)&a0[4] + 0x200;
      v4 = sub_2aa95(*a0,v4,v3);
      v5 -= v4;
      sub_ad86(*(int8 *)(*(int8 *)&a0[6] + 0x118) - *(int8 *)&a0[4]);
      *(int8 *)&a0[2] = v4 + *(int8 *)&a0[2];
      if (v4 != v3) {
        sub_3eb4b(**(unsigned long **)&a0[6],v4,v3);
        return 0;
      }
    }
  }
  else if ((*(char *)&a0[1]) && (v1 = *a0, sub_32f52(v1)))
    sub_3ea7e(**(unsigned long **)&a0[6]);
  return 1;
}


// Function: sub_30090 @ 0x30090
unsigned long sub_30090(int4 a0,int8 a1) // return-dupe x2
{
  int4 v1; // stack - 0x48
  char v2; // stack - 0x51
  uint8 v3; // stack - 0x50
  char v4; // stack - 0x44
  int8 v5; // stack - 0x38
  int8 v6; // stack - 0x30
  int8 v7; // stack - 0x28
  
  if (sub_2f165(&v1) != '\x01')
    return 3;
  v4 = 1;
  v1 = a0;
  v6 = a1;
  v2 = sub_2f9e5(&v1);
  if (((v2) && (*(int8 *)(v7 + 0x38))) && (sub_2f2f5(&v1), 0 <= a0)) {
    sub_ab09(*(unsigned long *)(v6 + 8),*(unsigned long *)(v6 + 0x88),*(int8 *)(v6 + 0x118) - v5);
    for (v3 = 0; (v2 && (v3 < *(uint8 *)(v6 + 0x130))); v3 = v3 + 1) {
      v2 = sub_2f261(&v1,v3);
    }
  }
  sub_142d7(*(int8 *)(v6 + 0x118) - v5);
  if ((sub_2f1d2(&v1)) && (v2))
    return 0;
  return 1;
}


// Function: sub_301de @ 0x301de
unsigned long sub_301de(unsigned long a0) // early-return
{
  char v1 [24];
  unsigned long v2; // stack - 0x30
  
  if (sub_2f165(v1) != '\x01')
    return 0;
  v2 = a0;
  return sub_2f126(v1);
}


// Function: sub_30241 @ 0x30241
unsigned long sub_30241(unsigned long a0) // early-return
{
  char v1 [24];
  unsigned long v2; // stack - 0x30
  
  if (sub_2f165(v1) != '\x01')
    return 0;
  v2 = a0;
  return sub_2f373(v1);
}


// Function: sub_302a4 @ 0x302a4
unsigned long sub_302a4(unsigned int a0,int8 a1,int8 *a2) // return-dupe x2
{
  unsigned int v1; // stack - 0x48
  char v2; // stack - 0x51
  uint8 v3; // stack - 0x50
  char v4; // stack - 0x44
  unsigned long v5; // stack - 0x40
  int8 v6; // stack - 0x38
  int8 v7; // stack - 0x30
  
  if (sub_2f165(&v1) != '\x01')
    return 3;
  v1 = a0;
  v7 = a1;
  v4 = lseek(a0,0,0) == 0;
  v5 = 0;
  v2 = sub_2f334(&v1);
  for (v3 = 0; (v2 && (v3 < *(uint8 *)(v7 + 0x130))); v3 = v3 + 1) {
    v2 = sub_2f2ab(&v1,v3);
  }
  *a2 = *(int8 *)(v7 + 0x118) - v6;
  if ((sub_2f1d2(&v1)) && (v2))
    return 0;
  return 1;
}


// Function: sub_303bd @ 0x303bd
unsigned long sub_303bd(int8 a0) // return-dupe x2
{
  char v1; // al
  unsigned int v2 [4]; // stack - 0x48
  int8 v3; // stack - 0x38
  int8 v4; // stack - 0x30
  
  if (sub_2f165(v2) != '\x01')
    return 3;
  v2[0] = 0xffffffff;
  v4 = a0;
  v1 = sub_2f334(v2);
  sub_287c1(*(int8 *)(v4 + 0x118) - v3);
  if ((sub_2f1d2(v2)) && (v1))
    return 0;
  return 1;
}


// Function: sub_3046a @ 0x3046a
unsigned long sub_3046a(unsigned int *a0,int8 a1,int8 a2) // return-dupe
{
  int8 v1; // rax
  int8 v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  char v5 [520];
  char v6 [32];
  int8 v7; // stack - 0x268
  
  v7 = a1;
  if (sub_2f3b2(a0,a1) != '\x01')
    return 0;
  while( true ) {
    if (a2 <= v7)
      return 1;
    v1 = a2 - v7;
    if (0x201 <= v1)
      v1 = 0x200;
    v2 = sub_5ec70(*a0,v5,v1);
    if (v2 == -1) {
      sub_2b2f8(**(unsigned long **)&a0[6],v7,v1);
      return 0;
    }
    if (!v2) {
      v3 = gettext("Size differs");
      sub_107be(*(unsigned long *)&a0[6],v3);
      return 0;
    }
    if (sub_2f433(v5,v2) != '\x01') break;
    v7 = v2 + v7;
  }
  v4 = sub_54d1c(v7,v6);
  v3 = gettext("File fragment at %s is not a hole");
  sub_107be(*(unsigned long *)&a0[6],v3,v4);
  return 0;
}


// Function: sub_3064f @ 0x3064f
unsigned long sub_3064f(unsigned int *a0,int8 a1) // return-dupe
{
  unsigned long v1;
  int8 v2;
  void *v3; // rax
  uint8 v4; // rax
  char *v5; // rax
  char v6 [520];
  int8 v7; // stack - 0x238
  
  v1 = *(unsigned long *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140));
  if (sub_2f3b2(a0,v1) != '\x01')
    return 0;
  v7 = *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8);
  sub_ad86(*(int8 *)(*(int8 *)&a0[6] + 0x118) - *(int8 *)&a0[4]);
  while( true ) {
    if (v7 <= 0)
      return 1;
    v2 = v7;
    if (0x201 <= v7)
      v2 = 0x200;
    v3 = (void *)sub_bb4b();
    if (!v3) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Unexpected EOF in archive"));
      dat_9fdd0 = 2;
      return 0;
    }
    sub_bba7(v3);
    *(int8 *)&a0[4] = *(int8 *)&a0[4] + 0x200;
    v4 = sub_5ec70(*a0,v6,v2);
    if (v4 == 0xffffffffffffffff) {
      sub_2b2f8(**(unsigned long **)&a0[6],(*(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140) + 8) + *(int8 *)(a1 * 0x10 + *(int8 *)(*(int8 *)&a0[6] + 0x140))) - v7,v2);
      return 0;
    }
    if (!v4) break;
    v7 -= v4;
    sub_ad86(*(int8 *)(*(int8 *)&a0[6] + 0x118) - *(int8 *)&a0[4]);
    if (memcmp(v3,v6,v4)) {
      v5 = gettext("Contents differ");
      sub_107be(*(unsigned long *)&a0[6],v5);
      return 0;
    }
  }
  sub_107be(0x9fa20,gettext("Size differs"));
  return 0;
}


// Function: sub_30972 @ 0x30972
char sub_30972(unsigned int a0,int8 a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_30972
{
  int8 v1;
  int8 v10; // stack - 0x30
  char v2; // al
  int8 *v3; // rax
  unsigned int v4; // stack - 0x48
  char v5; // stack - 0x59
  uint8 v6; // stack - 0x58
  int8 v7; // stack - 0x50
  char v8; // stack - 0x44
  int8 v9; // stack - 0x38
  
  v7 = 0;
  v2 = sub_2f165(&v4);
  if (v2 != '\x01') {
    v2 = '\0';
    return v2;
  }
  v8 = 1;
  v4 = a0;
  v10 = a1;
  v2 = sub_2f334(&v4);
  v5 = v2;
  sub_ad2b(a1);
  v6 = 0;
  while ((v5 && (v6 < *(uint8 *)(v10 + 0x130)))) {
    v3 = (int8)(v6 * 0x10);
    v3 = (unsigned long *)((int8)v3 + *(int8 *)(v10 + 0x140));
    v2 = sub_3046a(&v4,v7,*v3);
    v2 = ((v2) && (v2 = sub_3064f(&v4,v6), v2));
    v5 = v2;
    v3 = (int8)(v6 * 0x10);
    v3 = (int8 *)((int8)v3 + *(int8 *)(v10 + 0x140));
    v1 = *v3;
    v3 = (int8)(v6 * 0x10);
    v3 = (int8)((int8)v3 + *(int8 *)(v10 + 0x140));
    v3 = (int8)(*(int8 *)((int8)v3 + 8) + v1);
    v6 += 1;
    v7 = (int8)v3;
  }
  if (v5 != '\x01') {
    v3 = (int8)(*(int8 *)(v10 + 0x118) - v9);
    sub_287c1(v3);
  }
  sub_ad66();
  sub_2f1d2(&v4);
  v2 = v5;
  return v2;
}


// Function: sub_30afb @ 0x30afb
bool sub_30afb(void)
{
  return *(char *)(dat_9f460 + 0x9c) == 'S';
}


// Function: sub_30b1c @ 0x30b1c
unsigned long sub_30b1c(int8 a0,int8 a1) // return-dupe x2
{
  int8 v1; // stack - 0x28
  int8 v2; // stack - 0x20
  
  if (!*(char *)(a1 + 0xc))
    return 1;
  v1 = sub_2798b(a1,0xc);
  v2 = sub_2798b(a1 + 0xc,0xc);
  if ((((0 <= v1) && (0 <= v2)) && (!SCARRY8(v2,v1))) && ((v2 + v1 <= *(int8 *)(*(int8 *)(a0 + 0x18) + 0x88) && (0 <= *(int8 *)(*(int8 *)(a0 + 0x18) + 0x118))))) {
    sub_2f477(*(unsigned long *)(a0 + 0x18),&v1);
    return 0;
  }
  return 2;
}


// Function: sub_30c22 @ 0x30c22
uint8 sub_30c22(int8 a0)
{
  uint8 v1; // rax
  uint8 v2;
  
  v1 = sub_2798b(dat_9f460 + 0x1e3,0xc);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x118) = *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x88);
  v2 = v1;
  if ((int8)v1 <= -1)
    v2 = 0;
  *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x88) = v2;
  return ~v1 >> 0x3f;
}


// Function: sub_30c9a @ 0x30c9a
unsigned long sub_30c9a(int8 a0) // return-dupe x2
{
  char v1;
  unsigned long v2;
  int8 v3;
  int8 v4; // rax
  int4 v5; // stack - 0x2c
  uint8 v6; // stack - 0x28
  
  v3 = dat_9f460;
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  for (v6 = 0; v6 <= 3; v6 = v6 + 1) {
    v5 = sub_30b1c(a0,v3 + v6 * 0x18 + 0x182);
    if (v5) break;
  }
  v1 = *(char *)(v3 + 0x1e2);
  while ((!v5 && (v1))) {
    v3 = sub_bb4b();
    if (!v3) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Unexpected EOF in archive"));
      dat_9fdd0 = 2;
      return 0;
    }
    sub_bba7(v3);
    v6 = 0;
    while ((v6 <= 0x14 && (!v5))) {
      v4 = v6 * 0x18;
      v6 += 1;
      v5 = sub_30b1c(a0,v3 + v4);
    }
    v1 = *(char *)(v3 + 0x1f8);
  }
  if (v5 == 2) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = **(unsigned long **)(a0 + 0x18);
    error(0,0,gettext("%s: invalid sparse archive member"),v2);
    dat_9fdd0 = 2;
    return 0;
  }
  return 1;
}


// Function: sub_30e70 @ 0x30e70
void sub_30e70(int8 a0,uint8 *a1,int8 a2,int8 a3)
{
  int8 v1; // stack - 0x28
  int8 v2; // stack - 0x20
  
  v1 = a3;
  v2 = a2;
  while ((*a1 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130) && (v1))) {
    sub_12a9a(*(unsigned long *)(*a1 * 0x10 + *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140)),v2,0xc);
    sub_12a9a(*(unsigned long *)(*a1 * 0x10 + *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140) + 8),v2 + 0xc,0xc);
    v1 -= 1;
    v2 += 0x18;
    *a1 = *a1 + 1;
  }
  return;
}


// Function: sub_30f43 @ 0x30f43
unsigned long sub_30f43(int8 a0)
{
  uint8 v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  void *v3; // stack - 0x18
  
  v2 = sub_bacd();
  v3 = (void *)sub_1381d(*(unsigned long *)(a0 + 0x18));
  *(char *)((int8)v3 + 0x9c) = 0x53;
  if (5 <= *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130))
    *(char *)((int8)v3 + 0x1e2) = 1;
  sub_12a9a(*(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x88),(int8)v3 + 0x1e3,0xc);
  sub_12a9a(*(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x118),(int8)v3 + 0x7c,0xc);
  v1 = 0;
  sub_30e70(a0,&v1,(int8)v3 + 0x182,4);
  *(bool *)((int8)v3 + 0x1e2) = v1 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130);
  sub_1422e(*(unsigned long *)(a0 + 0x18),v3,v2);
  while (v1 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130)) {
    v3 = (void *)sub_bb4b();
    memset(v3,0,0x200);
    sub_30e70(a0,&v1,v3,0x15);
    if (v1 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130))
      *(char *)((int8)v3 + 0x1f8) = 1;
    sub_bba7(v3);
  }
  return 1;
}


// Function: sub_310ff @ 0x310ff
bool sub_310ff(void)
{
  return *(char *)(dat_9f460 + 0x9c) == 'S';
}


// Function: sub_31120 @ 0x31120
uint8 sub_31120(int8 a0)
{
  uint8 v1; // rax
  uint8 v2;
  
  v1 = sub_2798b(dat_9f460 + 0x1c4,0xc);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x118) = *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x88);
  v2 = v1;
  if ((int8)v1 <= -1)
    v2 = 0;
  *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x88) = v2;
  return ~v1 >> 0x3f;
}


// Function: sub_31198 @ 0x31198
unsigned long sub_31198(int8 a0) // return-dupe x2
{
  char v1;
  unsigned long v2;
  int8 v3;
  int8 v4; // rax
  int4 v5; // stack - 0x2c
  uint8 v6; // stack - 0x28
  
  v3 = dat_9f460;
  v5 = 0;
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  if ((*(char *)(dat_9f460 + 0x159)) || (!*(char *)(dat_9f460 + 0x16e)))
    v1 = '\x01';
  else {
    for (v6 = 0; v6 <= 3; v6 = v6 + 1) {
      v5 = sub_30b1c(a0,v3 + v6 * 0x18 + 0x164);
      if (v5) break;
    }
    v1 = *(char *)(v3 + 0x163);
  }
  while ((!v5 && (v1))) {
    v3 = sub_bb4b();
    if (!v3) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Unexpected EOF in archive"));
      dat_9fdd0 = 2;
      return 0;
    }
    sub_bba7(v3);
    v6 = 0;
    while ((v6 <= 0x14 && (!v5))) {
      v4 = v6 * 0x18;
      v6 += 1;
      v5 = sub_30b1c(a0,v3 + v4);
    }
    *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + 0x200;
    v1 = *(char *)(v3 + 0x1f8);
  }
  if (v5 == 2) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v2 = **(unsigned long **)(a0 + 0x18);
    error(0,0,gettext("%s: invalid sparse archive member"),v2);
    dat_9fdd0 = 2;
    return 0;
  }
  return 1;
}


// Function: sub_313b3 @ 0x313b3
unsigned int sub_313b3(int8 a0) // return-dupe
{
  if ((!*(int8 *)(*(int8 *)(a0 + 0x18) + 0x130)) && (!*(int4 *)(*(int8 *)(a0 + 0x18) + 0x124)))
    return 0;
  return 1;
}


// Function: sub_313f6 @ 0x313f6
unsigned long sub_313f6(int8 a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v1 = *(unsigned long *)(a0 + 0x88);
  *(unsigned long *)(a0 + 0x88) = *(unsigned long *)(a0 + 0x118);
  v2 = sub_1381d(a0);
  *(unsigned long *)(a0 + 0x88) = v1;
  return v2;
}


// Function: sub_31450 @ 0x31450
unsigned long sub_31450(int8 a0) // return-dupe
{
  int8 v1;
  unsigned long v2;
  uint8 v3; // stack - 0x60
  char v4 [24];
  int8 v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  int8 v7; // stack - 0x48
  unsigned long v8; // stack - 0x40
  
  v6 = sub_bacd();
  v7 = *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140);
  v5 = 0;
  sub_1e1d3("GNU.sparse.size",*(unsigned long *)(a0 + 0x18),0);
  sub_1e1d3("GNU.sparse.numblocks",*(unsigned long *)(a0 + 0x18),0);
  if ((sub_1c63f("GNU.sparse.map")) || (!dat_9f980)) {
    for (v3 = 0; v3 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130); v3 = v3 + 1) {
      sub_1e1d3("GNU.sparse.offset",*(unsigned long *)(a0 + 0x18),&v3);
      sub_1e1d3("GNU.sparse.numbytes",*(unsigned long *)(a0 + 0x18),&v3);
    }
  }
  else {
    sub_1e1d3("GNU.sparse.name",*(unsigned long *)(a0 + 0x18),0);
    v5 = *(int8 *)(*(int8 *)(a0 + 0x18) + 8);
    v2 = *(unsigned long *)(a0 + 0x18);
    v1 = *(int8 *)(a0 + 0x18);
    *(unsigned long *)(v1 + 8) = sub_1cd02(v2,"%d/GNUSparseFile.%p/%f",0);
    sub_1e848(*(int8 *)(a0 + 0x18) + 0x168);
    for (v3 = 0; v3 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130); v3 = v3 + 1) {
      if (v3)
        sub_1e863(*(int8 *)(a0 + 0x18) + 0x168,0x83c63);
      v2 = sub_54e5a(*(unsigned long *)(v7 + v3 * 0x10),v4);
      sub_1e863(*(int8 *)(a0 + 0x18) + 0x168,v2);
      sub_1e863(*(int8 *)(a0 + 0x18) + 0x168,0x83c63);
      v2 = sub_54e5a(*(unsigned long *)(v7 + v3 * 0x10 + 8),v4);
      sub_1e863(*(int8 *)(a0 + 0x18) + 0x168,v2);
    }
    v1 = *(int8 *)(a0 + 0x18);
    if (sub_1e8dc(v1 + 0x168,"GNU.sparse.map") != '\x01') {
      free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
      *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v5;
      return 0;
    }
  }
  v8 = sub_313f6(*(unsigned long *)(a0 + 0x18));
  sub_1422e(*(unsigned long *)(a0 + 0x18),v8,v6);
  if (!v5)
    return 1;
  free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
  *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v5;
  return 1;
}


// Function: sub_3179e @ 0x3179e
unsigned long sub_3179e(int8 a0)
{
  char *v1;
  char *v10; // stack - 0xd8
  char *v11; // stack - 0xd0
  uint8 v12; // stack - 0xc8
  int8 v13; // stack - 0xc0
  char *v14; // stack - 0xb8
  char *v15; // stack - 0xb0
  char *v16; // stack - 0xa8
  char *v17; // stack - 0xa0
  char *v18; // stack - 0x98
  char *v19; // stack - 0x90
  int8 v2;
  char *v20; // stack - 0x88
  char *v21; // stack - 0x80
  char *v22; // stack - 0x78
  char *v23; // stack - 0x70
  char *v24; // stack - 0x68
  char *v25; // stack - 0x60
  unsigned long v3;
  unsigned long v4; // rax
  char *v5;
  uint8 v6; // rax
  int8 v7;
  unsigned long v8;
  char v9 [24];
  
  v4 = sub_bacd();
  v2 = *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140);
  v3 = *(unsigned long *)(*(int8 *)(a0 + 0x18) + 8);
  v5 = (char *)sub_54e5a(*(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130),v9);
  v13 = strlen(v5) + 1;
  for (v12 = 0; v12 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130); v12 = v12 + 1) {
    v8 = *(unsigned long *)(v2 + v12 * 0x10);
    v6 = strlen((char *)sub_54e5a(v8,v9));
    v5 = (char *)sub_54e5a(*(unsigned long *)(v2 + v12 * 0x10 + 8),v9);
    v13 = strlen(v5) + v6 + v13 + 2;
  }
  v7 = v13 + 0x1ff;
  if (v13 + 0x1ff <= -1)
    v7 = v13 + 0x3fe;
  *(int8 *)(*(int8 *)(a0 + 0x18) + 0x118) = *(int8 *)(*(int8 *)(a0 + 0x18) + 0x118) + (v7 >> 9) * 0x200;
  *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + (v7 >> 9) * 0x200;
  sub_1e1d3("GNU.sparse.major",*(unsigned long *)(a0 + 0x18),0);
  sub_1e1d3("GNU.sparse.minor",*(unsigned long *)(a0 + 0x18),0);
  sub_1e1d3("GNU.sparse.name",*(unsigned long *)(a0 + 0x18),0);
  sub_1e1d3("GNU.sparse.realsize",*(unsigned long *)(a0 + 0x18),0);
  v8 = *(unsigned long *)(a0 + 0x18);
  v7 = *(int8 *)(a0 + 0x18);
  *(unsigned long *)(v7 + 8) = sub_1cd02(v8,"%d/GNUSparseFile.%p/%f",0);
  v5 = *(char **)(*(int8 *)(a0 + 0x18) + 8);
  if (0x65 <= strlen(v5))
    *(char *)(*(int8 *)(*(int8 *)(a0 + 0x18) + 8) + 100) = 0;
  v8 = sub_313f6(*(unsigned long *)(a0 + 0x18));
  sub_1422e(*(unsigned long *)(a0 + 0x18),v8,v4);
  free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 8) = v3;
  v10 = (char *)sub_bb4b();
  v11 = v10;
  v3 = *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130);
  v14 = &v10[0x200];
  v15 = (char *)sub_54e5a(v3,v9);
  while (*v15) {
    if (v11 == v14) {
      sub_bba7(v10);
      v10 = (char *)sub_bb4b();
      v11 = v10;
      v14 = &v10[0x200];
    }
    v5 = &v15[1];
    v1 = &v11[1];
    *v11 = *v15;
    v11 = v1;
    v15 = v5;
  }
  v16 = &v10[0x200];
  v17 = "\n";
  while (*v17) {
    if (v11 == v16) {
      sub_bba7(v10);
      v10 = (char *)sub_bb4b();
      v11 = v10;
      v16 = &v10[0x200];
    }
    v5 = &v17[1];
    v1 = &v11[1];
    *v11 = *v17;
    v11 = v1;
    v17 = v5;
  }
  for (v12 = 0; v12 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130); v12 = v12 + 1) {
    v3 = *(unsigned long *)(v2 + v12 * 0x10);
    v18 = &v10[0x200];
    v19 = (char *)sub_54e5a(v3,v9);
    while (*v19) {
      if (v11 == v18) {
        sub_bba7(v10);
        v10 = (char *)sub_bb4b();
        v11 = v10;
        v18 = &v10[0x200];
      }
      v5 = &v19[1];
      v1 = &v11[1];
      *v11 = *v19;
      v11 = v1;
      v19 = v5;
    }
    v20 = &v10[0x200];
    v21 = "\n";
    while (*v21) {
      if (v11 == v20) {
        sub_bba7(v10);
        v10 = (char *)sub_bb4b();
        v11 = v10;
        v20 = &v10[0x200];
      }
      v5 = &v21[1];
      v1 = &v11[1];
      *v11 = *v21;
      v11 = v1;
      v21 = v5;
    }
    v3 = *(unsigned long *)(v2 + v12 * 0x10 + 8);
    v22 = &v10[0x200];
    v23 = (char *)sub_54e5a(v3,v9);
    while (*v23) {
      if (v11 == v22) {
        sub_bba7(v10);
        v10 = (char *)sub_bb4b();
        v11 = v10;
        v22 = &v10[0x200];
      }
      v5 = &v23[1];
      v1 = &v11[1];
      *v11 = *v23;
      v11 = v1;
      v23 = v5;
    }
    v24 = &v10[0x200];
    v25 = "\n";
    while (*v25) {
      if (v11 == v24) {
        sub_bba7(v10);
        v10 = (char *)sub_bb4b();
        v11 = v10;
        v24 = &v10[0x200];
      }
      v5 = &v25[1];
      v1 = &v11[1];
      *v11 = *v25;
      v11 = v1;
      v25 = v5;
    }
  }
  memset(v11,0,0x200 - ((int8)v11 - (int8)v10));
  sub_bba7(v10);
  return 1;
}


// Function: sub_31fd5 @ 0x31fd5
char sub_31fd5(int8 a0)
{
  char v1; // al
  
  *(unsigned int *)(*(int8 *)(a0 + 0x18) + 0x124) = dat_9f97c;
  *(unsigned int *)(*(int8 *)(a0 + 0x18) + 0x128) = dat_9f980;
  if (*(int4 *)(*(int8 *)(a0 + 0x18) + 0x124)) // branch-flip
    v1 = sub_3179e(a0);
  else {
    v1 = sub_31450(a0);
  }
  v1 = v1 != '\0';
  return v1;
}


// Function: sub_3204b @ 0x3204b
unsigned long sub_3204b(uint8 *a0,char *a1,uint8 a2) // return-dupe x2
{
  char *v1; // stack - 0x20
  uint8 v2; // stack - 0x18
  
  if (10 <= (uint4)((int4)*a1 - 0x30U))
    return 0;
  *__errno_location() = 0;
  v2 = strtoumax(a1,&v1,10);
  if (((v2 <= a2) && (*__errno_location() != 0x22)) && (!*v1)) {
    *a0 = v2;
    return 1;
  }
  return 0;
}


// Function: sub_32100 @ 0x32100
unsigned long sub_32100(int8 a0) // return-dupe
{
  char *v1;
  char *v10; // stack - 0x90
  uint8 v11; // stack - 0x88
  char *v12; // stack - 0x80
  char *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  char *v17; // stack - 0x58
  int8 v18; // stack - 0x50
  unsigned long v19; // stack - 0x40
  char *v2;
  unsigned long v3;
  int8 v4;
  char v5 [20];
  char v6 [4];
  unsigned long v7; // stack - 0xa0
  unsigned long v8; // stack - 0x48
  char *v9; // stack - 0x98
  
  if (!*(int4 *)(*(int8 *)(a0 + 0x18) + 0x124))
    return 1;
  v18 = sub_bacd();
  sub_bba7(dat_9f460);
  v9 = (char *)sub_bb4b();
  if (!v9) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Unexpected EOF in archive"));
    sub_177f4(); // no-return
  }
  v10 = v9;
  v12 = &v9[0x200];
  v2 = v5;
  do {
    v13 = v2;
    if (v13 == v6) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v3 = **(unsigned long **)(a0 + 0x18);
      error(0,0,gettext("%s: numeric overflow in sparse archive member"),v3);
      dat_9fdd0 = 2;
      return 0;
    }
    if (v10 == v12) {
      sub_bba7(v9);
      v9 = (char *)sub_bb4b();
      if (!v9) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Unexpected EOF in archive"));
        sub_177f4(); // no-return
      }
      v10 = v9;
      v12 = &v9[0x200];
    }
    v1 = &v10[1];
    *v13 = *v10;
    v2 = &v13[1];
    v10 = v1;
  } while (*v13 != '\n');
  *v13 = '\0';
  v13 = v2;
  if (sub_3204b(&v7,v5,0xffffffffffffffff) == '\x01') {
    *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x138) = v7;
    v3 = *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x138);
    v4 = *(int8 *)(a0 + 0x18);
    *(unsigned long *)(v4 + 0x140) = sub_60c29(v3,0x10);
    *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
    v11 = 0;
    while( true ) {
      if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x138) <= v11) {
        sub_bba7(v9);
        *(int8 *)(a0 + 0x10) = (sub_bacd() - v18) * 0x200 + *(int8 *)(a0 + 0x10);
        return 1;
      }
      v14 = &v9[0x200];
      v2 = v5;
      do {
        v15 = v2;
        if (v15 == v6) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v3 = **(unsigned long **)(a0 + 0x18);
          error(0,0,gettext("%s: numeric overflow in sparse archive member"),v3);
          dat_9fdd0 = 2;
          return 0;
        }
        if (v10 == v14) {
          sub_bba7(v9);
          v9 = (char *)sub_bb4b();
          if (!v9) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Unexpected EOF in archive"));
            sub_177f4(); // no-return
          }
          v10 = v9;
          v14 = &v9[0x200];
        }
        v1 = &v10[1];
        *v15 = *v10;
        v2 = &v15[1];
        v10 = v1;
      } while (*v15 != '\n');
      *v15 = '\0';
      v15 = v2;
      if (sub_3204b(&v7,v5,0x7fffffffffffffff) != '\x01') {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v3 = **(unsigned long **)(a0 + 0x18);
        error(0,0,gettext("%s: malformed sparse archive member"),v3);
        dat_9fdd0 = 2;
        return 0;
      }
      v8 = v7;
      v16 = &v9[0x200];
      v2 = v5;
      do {
        v17 = v2;
        if (v17 == v6) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v3 = **(unsigned long **)(a0 + 0x18);
          error(0,0,gettext("%s: numeric overflow in sparse archive member"),v3);
          dat_9fdd0 = 2;
          return 0;
        }
        if (v10 == v16) {
          sub_bba7(v9);
          v9 = (char *)sub_bb4b();
          if (!v9) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Unexpected EOF in archive"));
            sub_177f4(); // no-return
          }
          v10 = v9;
          v16 = &v9[0x200];
        }
        v1 = &v10[1];
        *v17 = *v10;
        v2 = &v17[1];
        v10 = v1;
      } while (*v17 != '\n');
      *v17 = '\0';
      v17 = v2;
      if (sub_3204b(&v7,v5,0x7fffffffffffffff) != '\x01') break;
      v19 = v7;
      sub_2f477(*(unsigned long *)(a0 + 0x18),&v8);
      v11 += 1;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v3 = **(unsigned long **)(a0 + 0x18);
    error(0,0,gettext("%s: malformed sparse archive member"),v3);
    dat_9fdd0 = 2;
    return 0;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v3 = **(unsigned long **)(a0 + 0x18);
  error(0,0,gettext("%s: malformed sparse archive member"),v3);
  dat_9fdd0 = 2;
  return 0;
}


// Function: sub_3286e @ 0x3286e
int8 * sub_3286e(char *a0,int8 *a1) // return-dupe
{
  void *v1;
  char *v2; // rax
  uint8 v3; // rax
  int8 *v4; // stack - 0x20
  
  v2 = strrchr(a0,0x2e);
  if (!v2)
    return NULL;
  v3 = strlen(&v2[1]);
  v4 = (int8 *)0x9c960;
  while( true ) {
    if (!*v4)
      return NULL;
    if ((v3 == v4[1]) && (v1 = (void *)*v4, !memcmp(v1,&v2[1],v3))) break;
    v4 = &v4[3];
  }
  if (a1)
    *a1 = (strlen(a0) - v3) + -1;
  return v4;
}


// Function: sub_32930 @ 0x32930
int8 sub_32930(unsigned long a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_3286e(a0,0);
  if (v1) // branch-flip
    v1 = *(unsigned long *)(v1 + 0x10);
  else {
    v1 = a1;
  }
  return v1;
}


// Function: sub_32970 @ 0x32970
void sub_32970(unsigned long a0,unsigned long a1) // return-dupe
{
  int8 v1; // rax
  
  v1 = sub_32930(a0,a1);
  if (!v1)
    return;
  dat_9f898 = v1;
  return;
}


// Function: sub_329b0 @ 0x329b0
void * sub_329b0(void *a0)
{
  int8 v1;
  uint8 v2; // stack - 0x28
  void *v3; // stack - 0x20
  unsigned long *v4; // stack - 0x18
  
  v3 = NULL;
  v4 = (unsigned long *)sub_3286e(a0,&v2);
  if (v4) {
    if (((5 <= v2) && (v1 = v2 - 4, !strncmp((char *)((int8)a0 + v1),".tar",4))) && (*(char *)*v4 != 't'))
      v2 -= 4;
    if (!v2)
      return NULL;
    v3 = (void *)sub_60b5b(v2 + 1);
    memcpy(v3,a0,v2);
    *(char *)((int8)v3 + v2) = 0;
  }
  return v3;
}


// Function: sub_32aaa @ 0x32aaa
void sub_32aaa(unsigned long a0)
{
  char *v1; // stack - 0x38
  int8 v2; // fs_offset
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x10
  
  v6 = *(unsigned long *)(v2 + 0x28);
  v1 = "/bin/sh";
  v3 = 0x83f78;
  v5 = 0;
  v4 = a0;
  execv("/bin/sh",&v1);
  sub_3e251(a0); // no-return
}


// Function: sub_32b11 @ 0x32b11
unsigned long sub_32b11(void)
{
  int4 v1; // eax
  
  v1 = fstat(dat_9f9cc,(void *)0x9f7a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_32b37 @ 0x32b37
unsigned int sub_32b37(int8 a0) // return-dupe
{
  if (((dat_9fbf8) && (*(int8 *)(a0 + 0x58) == dat_9fbf8)) && (*(int8 *)(a0 + 0x60) == dat_9fc00))
    return 1;
  return 0;
}


// Function: sub_32b88 @ 0x32b88
void sub_32b88(void) // return-dupe
{
  if ((dat_9f9cc <= 0x3fffffff) && ((dat_9f7b8 & 0xf000) == 0x8000)) {
    dat_9fc00 = dat_9f7a8;
    dat_9fbf8 = dat_9f7a0;
    return;
  }
  dat_9fbf8 = 0;
  return;
}


// Function: sub_32bdc @ 0x32bdc
void sub_32bdc(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_32bdc
{
  char *v1;
  int8 v2; // stack - 0xa8
  int8 v3; // stack - 0xa0
  
  v1 = (char *)*dat_9fbd8;
  dat_9f9d0 = !((strcmp(v1,"/dev/null")) && ((((0x40000000 <= dat_9f9cc || ((dat_9f7b8 & 0xf000) != 0x2000)) || ((stat("/dev/null",&v2) || (dat_9f7a0 != v2)))) || (dat_9f7a8 != v3))));
  return;
}


// Function: sub_32ca8 @ 0x32ca8
void sub_32ca8(int4 a0,char a1) // return-dupe x2
{
  char *v1; // rax
  uint4 v2; // stack - 0x28
  uint4 v3; // stack - 0x24
  
  if (!a0)
    return;
  do {
    if (waitpid(a0,(int4 *)&v2,0) != -1) goto label_32d10;
  } while (*__errno_location() == 4);
  sub_3eaf9(dat_9f898);
label_32d10:
  if ((char)(((uint1)v2 & 0x7f) + 1) >> 1 <= '\0') {
    if (!((int4)v2 >> 8 & 0xffU))
      return;
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Child returned status %d"),(uint8)((int4)v2 >> 8 & 0xff));
    sub_177f4(); // no-return
  }
  v3 = v2 & 0x7f;
  if ((!a1) && (v3 == 0xd))
    return;
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v1 = gettext("Child died with signal %d");
  error(0,0,v1,(uint8)v3);
  sub_177f4(); // no-return
}


// Function: sub_32df0 @ 0x32df0
void sub_32df0(void)
{
  int4 v1; // eax
  int8 v2;
  int4 v3; // stack - 0x1c
  char *v4; // stack - 0x18
  
  v4 = getenv("SHELL");
  if (!v4)
    v4 = "/bin/sh";
  v3 = sub_2b537();
  if (v3) {
    do {
      v1 = waitpid(v3,&v2,0);
      if (v1 != -1)
        return;
    } while (*__errno_location() == 4);
    sub_3eaf9(v4);
    return;
  }
  sub_5c835();
  execlp(v4,"-sh",0x83fb4,0);
  sub_3e251(v4); // no-return
}


// Function: sub_32ebf @ 0x32ebf
unsigned int sub_32ebf(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x1c) >> 8),*(int4 *)(a0 + 0x1c) == *(int4 *)(a1 + 0x1c));
}


// Function: sub_32ee4 @ 0x32ee4
unsigned int sub_32ee4(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x20) >> 8),*(int4 *)(a0 + 0x20) == *(int4 *)(a1 + 0x20));
}


// Function: sub_32f09 @ 0x32f09
unsigned int sub_32f09(int8 *a0,int8 *a1) // return-dupe
{
  if ((*a1 == *a0) && (a1[1] == a0[1]))
    return 1;
  return 0;
}


// Function: sub_32f52 @ 0x32f52
unsigned long sub_32f52(unsigned int a0) // early-return
{
  int8 v1; // rax
  
  v1 = lseek(a0,0,1);
  if (0 <= v1)
    return ftruncate(a0,v1);
  return 0xffffffff;
}


// Function: sub_32f9a @ 0x32f9a
bool sub_32f9a(char *a0)
{
  char v1 [24];
  bool v2; // zf
  uint4 v3; // stack - 0x90
  
  if (stat(a0,v1)) // branch-flip
    v2 = *__errno_location() == 2;
  else {
    v2 = (v3 & 0xf000) == 0x8000;
  }
  return v2;
}


// Function: sub_3301a @ 0x3301a
void sub_3301a(void) // return-dupe
{
  if (0x40000000 <= dat_9f9cc) {
    sub_3ff8c(dat_9f9cc + -0x40000000,dat_9efa0,dat_9f870);
    return;
  }
  sub_52175(dat_9f9cc,dat_9efa0,dat_9f870);
  return;
}


// Function: sub_3307b @ 0x3307b
void sub_3307b(int4 a0,int4 a1) // return-dupe
{
  int4 v1;
  
  if (a0 == a1)
    return;
  if ((close(a1)) && (*__errno_location() != 9)) {
    v1 = *__errno_location();
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,v1,gettext("Cannot close"));
    sub_177f4(); // no-return
  }
  v1 = dup(a0);
  if (v1 == a1) {
    sub_bc11(a0);
    return;
  }
  if (0 <= v1)
    abort(); // no-return
  v1 = *__errno_location();
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,v1,gettext("Cannot dup"));
  sub_177f4(); // no-return
}


// Function: sub_33179 @ 0x33179
void sub_33179(int4 a0)
{
  uint4 v1; // stack - 0x18
  int8 v2; // fs_offset
  uint4 v3; // stack - 0x14
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v2 + 0x28);
  v3 = 0;
  do {
    if (waitpid(a0,(int4 *)&v1,0) != -1) goto label_331d8;
  } while (*__errno_location() == 4);
  sub_3eaf9(dat_9f898);
label_331d8:
  if ('\x01' <= (char)(((uint1)v1 & 0x7f) + 1) >> 1) // branch-flip
    raise(v1 & 0x7f);
  else if ((int4)v1 >> 8 & 0xffU)
    v3 = (int4)v1 >> 8 & 0xff;
  exit(v3); // no-return
}


// Function: sub_3321b @ 0x3321b
int4 sub_3321b(void)
{
  void *v1;
  uint8 v10; // stack - 0x30
  unsigned int v11; // stack - 0x1c
  unsigned int v12; // stack - 0x14
  char *v2;
  int4 v3;
  uint8 v4;
  unsigned int v5; // stack - 0x20
  unsigned int v6; // stack - 0x18
  int4 v7; // stack - 0x44
  int8 v8; // stack - 0x40
  int8 v9; // stack - 0x38
  
  signal(0xd,1);
  sub_2b577(&v5);
  v3 = sub_2b537();
  if (1 <= v3) {
    dat_9f9cc = v11;
    sub_bc11(v5);
    return v3;
  }
  sub_5c844(gettext("tar (child)"));
  signal(0xd,0);
  sub_3307b(v5,0);
  sub_bc11(v11);
  if (((((dat_9fdf8) || (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), !dat_9fdf0)) || (dat_9fdf0 <= (char *)*dat_9fbd8)) || (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v1 = (void *)*dat_9fbd8, memchr(v1,0x2f,v4))) && (v4 = *dat_9fbd8, sub_32f9a(v4))) {
    if (dat_9f884)
      sub_2a571(*dat_9fbd8,1);
    v2 = (char *)*dat_9fbd8;
    if (strcmp(v2,"-")) {
      dat_9f9cc = creat(*dat_9fbd8,0x1b6);
      if (dat_9f9cc <= -1) {
        v3 = *__errno_location();
        if (dat_9f884)
          sub_2a874();
        *__errno_location() = v3;
        sub_3e3b4(*dat_9fbd8); // no-return
      }
      sub_3307b(dat_9f9cc,1);
    }
    sub_5c835();
    sub_32aaa(dat_9f898); // no-return
  }
  sub_2b577(&v6);
  v7 = sub_2b537();
  if (!v7) {
    sub_5c844(gettext("tar (grandchild)"));
    sub_3307b(v12,1);
    sub_bc11(v6);
    sub_5c835();
    sub_32aaa(dat_9f898); // no-return
  }
  sub_3307b(v6,0);
  sub_bc11(v12);
  v2 = (char *)*dat_9fbd8;
  if (strcmp(v2,"-")) { // branch-flip
    if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbd8 < dat_9fdf0 && (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v1 = (void *)*dat_9fbd8, !memchr(v1,0x2f,v4)))))
      dat_9f9cc = sub_3f860(*dat_9fbd8,0x41,0x40000000,dat_9f950);
    else {
      dat_9f9cc = creat(*dat_9fbd8,0x1b6);
    }
    if (dat_9f9cc <= -1)
      sub_3e3b4(*dat_9fbd8); // no-return
  }
  else {
    dat_9f9cc = 1;
  }
  do {
    v8 = 0;
    v9 = dat_9efa0;
    for (v10 = 0; v10 < dat_9f870; v10 = v10 + v8) {
      v8 = sub_5ec70(0,v9,dat_9f870 - v10);
      if (v8 == -1)
        sub_3e5d6(dat_9f898); // no-return
      if (!v8) break;
      v9 += v8;
    }
    if (!v8) {
      if (v10) {
        memset((void *)(dat_9efa0 + v10),0,dat_9f870 - v10);
        v4 = sub_3301a();
        if (v4 != dat_9f870)
          sub_c6d5(v4); // no-return
      }
      sub_33179(v7); // no-return
    }
    v4 = sub_3301a();
    if (v4 != dat_9f870)
      sub_c6d5(v4); // no-return
  } while( true );
}


// Function: sub_336d5 @ 0x336d5
void sub_336d5(void)
{
  int4 v1; // eax
  unsigned long *v10; // stack - 0x100
  int8 v11; // stack - 0xf8
  unsigned long v12; // stack - 0x98
  unsigned long v13; // stack - 0x20
  char *v2; // rax
  unsigned long v3; // rax
  char v4 [4];
  int8 v5; // stack - 0x108
  int8 v6; // fs_offset
  uint4 v7; // stack - 0x11c
  int8 v8; // stack - 0x118
  int8 v9; // stack - 0x110
  
  v13 = *(unsigned long *)(v6 + 0x28);
  v9 = 0;
  v7 = 0x2080e06;
  v12 = __environ;
  v11 = 1;
  v8 = sub_afa2(v4);
  while( true ) {
    if (!v8) {
      if (!v9) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("unable to run decompression program"));
        sub_177f4(); // no-return
      }
      sub_3e251(v9); // no-return
    }
    if (v9) {
      if (dat_9ec84 & 0x80000) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v2 = gettext("cannot run %s");
        error(0,*__errno_location(),v2,v9);
      }
      if (dat_9ec84 & 0x80000) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("trying %s"),v8);
      }
    }
    v1 = sub_461a6(v8,&v5,v7);
    if (v1) break;
    v7 |= 8;
    memmove(v10,&v10[v11],v5 * 8);
    v10[v5] = 0x84027;
    v9 = v8;
    execvp(*v10,v10);
    v10[v5] = 0;
    v8 = sub_b012(v4);
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v3 = sub_4647f(&v5);
  error(0,0,gettext("cannot split string \'%s\': %s"),v8,v3);
  sub_177f4(); // no-return
}


// Function: sub_3399b @ 0x3399b
int4 sub_3399b(void)
{
  char *v1;
  unsigned int v10; // stack - 0x14
  void *v2;
  int4 v3; // eax
  uint8 v4;
  unsigned int v5; // stack - 0x20
  unsigned int v6; // stack - 0x18
  int8 v7; // stack - 0x40
  uint8 v8; // stack - 0x38
  unsigned int v9; // stack - 0x1c
  
  sub_2b577(&v5);
  v3 = sub_2b537();
  if (1 <= v3) {
    dat_9f9cc = v5;
    sub_bc11(v9);
    return v3;
  }
  sub_5c844(gettext("tar (child)"));
  signal(0xd,0);
  sub_3307b(v9,1);
  sub_bc11(v5);
  v1 = (char *)*dat_9fbd8;
  if (((strcmp(v1,"-")) && ((((dat_9fdf8 || (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), !dat_9fdf0)) || (dat_9fdf0 <= (char *)*dat_9fbd8)) || (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v2 = (void *)*dat_9fbd8, memchr(v2,0x2f,v4))))) && (v4 = *dat_9fbd8, sub_32f9a(v4))) {
    dat_9f9cc = open((char *)*dat_9fbd8,0,0x1b6);
    if (0 > dat_9f9cc)
      sub_3e3b4(*dat_9fbd8); // no-return
    sub_3307b(dat_9f9cc,0);
    sub_5c835();
    sub_336d5(); // no-return
  }
  sub_2b577(&v6);
  v3 = sub_2b537();
  if (!v3) {
    sub_5c844(gettext("tar (grandchild)"));
    sub_3307b(v6,0);
    sub_bc11(v10);
    sub_5c835();
    sub_336d5(); // no-return
  }
  sub_3307b(v10,1);
  sub_bc11(v6);
  v1 = (char *)*dat_9fbd8;
  if (strcmp(v1,"-")) { // branch-flip
    if (((dat_9fdf8 != '\x01') && (dat_9fdf0 = strchr((char *)*dat_9fbd8,0x3a), dat_9fdf0)) && (((char *)*dat_9fbd8 < dat_9fdf0 && (v4 = (int8)dat_9fdf0 - *dat_9fbd8, v2 = (void *)*dat_9fbd8, !memchr(v2,0x2f,v4)))))
      dat_9f9cc = sub_3f860(*dat_9fbd8,0,0x40000000,dat_9f950);
    else {
      dat_9f9cc = open((char *)*dat_9fbd8,0,0x1b6);
    }
  }
  else {
    dat_9f9cc = 0;
  }
  if (0 > dat_9f9cc)
    sub_3e3b4(*dat_9fbd8); // no-return
  while( true ) {
    sub_adb0();
    while( true ) {
      if (0x40000000 <= dat_9f9cc) // branch-flip
        v8 = sub_3fe62(dat_9f9cc + -0x40000000,dat_9efa0,dat_9f870);
      else {
        v8 = sub_5ec70(dat_9f9cc,dat_9efa0,dat_9f870);
      }
      if (v8 != 0xffffffffffffffff) break;
      sub_c729();
    }
    if (!v8) break;
    v7 = dat_9efa0;
    for (; v8; v8 = v8 - v4) {
      v4 = v8;
      if (0x201 <= v8)
        v4 = 0x200;
      if (v4 != sub_52175(1,v7,v4))
        sub_3eb22(dat_9f898);
      v7 += v4;
    }
  }
  sub_bc11(1);
  sub_33179(v3); // no-return
}


// Function: sub_33de6 @ 0x33de6
void sub_33de6(char *a0,unsigned long a1)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_54e5a(a1,v2);
  if (!setenv(a0,v1,1))
    return;
  sub_1783c(); // no-return
}


// Function: sub_33e58 @ 0x33e58
void sub_33e58(char *a0,unsigned long a1,unsigned long a2)
{
  char *v1; // rax
  char v2 [40];
  
  v1 = (char *)sub_29f2e(a1,a2,v2);
  if (!setenv(a0,v1,1))
    return;
  sub_1783c(); // no-return
}


// Function: sub_33ed9 @ 0x33ed9
void sub_33ed9(char *a0,unsigned long a1)
{
  char v1 [24];
  
  snprintf(v1,0x18,"0%lo",a1);
  if (!setenv(a0,v1,1))
    return;
  sub_1783c(); // no-return
}


// Function: sub_33f58 @ 0x33f58
void sub_33f58(char *a0,char *a1) // return-dupe
{
  if (!a1) {
    unsetenv(a0);
    return;
  }
  if (!setenv(a0,a1,1))
    return;
  sub_1783c(); // no-return
}


// Function: sub_33fa3 @ 0x33fa3
void sub_33fa3(char *a0,char a1)
{
  char v1 [2]; // stack - 0x12
  
  v1[1] = 0;
  v1[0] = a1;
  if (!setenv(a0,v1,1))
    return;
  sub_1783c(); // no-return
}


// Function: sub_3400a @ 0x3400a
void sub_3400a(unsigned long a0,char a1,int8 a2) // return-dupe x2
{
  unsigned long v1;
  int4 v2;
  
  sub_33f58("TAR_VERSION",0x84059);
  sub_33f58("TAR_ARCHIVE",*dat_9fbe8);
  sub_33de6("TAR_VOLUME",((int8)dat_9fbe8 - dat_9fbd8 >> 3) + 1);
  sub_33de6("TAR_BLOCKING_FACTOR",(int8)dat_9f868);
  if (dat_9f468) // branch-flip
    v2 = dat_9f468;
  else {
    v2 = dat_9f864;
  }
  sub_33f58("TAR_FORMAT",sub_34d66(v2));
  sub_33fa3("TAR_FILETYPE",(int4)a1);
  sub_33ed9("TAR_MODE",*(unsigned int *)(a2 + 0x70));
  sub_33f58("TAR_FILENAME",a0);
  sub_33f58("TAR_REALNAME",*(unsigned long *)(a2 + 8));
  sub_33f58("TAR_UNAME",*(unsigned long *)(a2 + 0x20));
  sub_33f58("TAR_GNAME",*(unsigned long *)(a2 + 0x28));
  sub_33e58("TAR_ATIME",*(unsigned long *)(a2 + 0xe8),*(unsigned long *)(a2 + 0xf0));
  sub_33e58("TAR_MTIME",*(unsigned long *)(a2 + 0xf8),*(unsigned long *)(a2 + 0x100));
  sub_33e58("TAR_CTIME",*(unsigned long *)(a2 + 0x108),*(unsigned long *)(a2 + 0x110));
  sub_33de6("TAR_SIZE",*(unsigned long *)(a2 + 0x88));
  sub_33de6("TAR_UID",*(unsigned int *)(a2 + 0x74));
  sub_33de6("TAR_GID",*(unsigned int *)(a2 + 0x78));
  if (a1 != 'l') {
    if (a1 <= 'l') {
      if ('d' <= a1) { // branch-flip
        if (a1 == 'h') goto label_342b8;
      }
      else if ('b' <= a1) {
        v1 = *(unsigned long *)(a2 + 0x80);
        sub_33de6("TAR_MINOR",gnu_dev_minor(v1));
        v1 = *(unsigned long *)(a2 + 0x80);
        sub_33de6("TAR_MAJOR",gnu_dev_major(v1));
        unsetenv("TAR_LINKNAME");
        return;
      }
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    unsetenv("TAR_LINKNAME");
    return;
  }
label_342b8:
  unsetenv("TAR_MINOR");
  unsetenv("TAR_MAJOR");
  sub_33f58("TAR_LINKNAME",*(unsigned long *)(a2 + 0x18));
  return;
}


// Function: sub_34323 @ 0x34323
unsigned int sub_34323(unsigned long a0,char a1,unsigned long a2)
{
  unsigned int v1; // stack - 0x18
  unsigned int v2; // stack - 0x14
  
  sub_2b577(&v1);
  dat_9f838 = signal(0xd,1);
  dat_9f830 = sub_2b537();
  if (dat_9f830) {
    sub_bc11(v1);
    return v2;
  }
  sub_3307b(v1,0);
  sub_bc11(v2);
  sub_3400a(a0,(int4)a1,a2);
  sub_5c835();
  sub_32aaa(dat_9f9a0); // no-return
}


// Function: sub_343ea @ 0x343ea
void sub_343ea(void) // return-dupe
{
  int4 v1; // eax
  int8 v2; // rbx
  uint4 v3; // stack - 0x24
  
  if (dat_9f830 <= -1)
    return;
  signal(0xd,dat_9f838);
  do {
    v1 = waitpid(dat_9f830,(int4 *)&v3,0);
    if (v1 != -1) {
      if (v3 & 0x7f) { // branch-flip
        if ('\x01' <= (char)(((uint1)v3 & 0x7f) + 1) >> 1) { // branch-flip
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v2 = (int8)dat_9f830;
          error(0,0,gettext("%lu: Child terminated on signal %d"),v2,(uint8)(v3 & 0x7f));
        }
        else {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v2 = (int8)dat_9f830;
          error(0,0,gettext("%lu: Child terminated on unknown reason"),v2);
          dat_9fdd0 = 2;
        }
      }
      else if ((dat_9f9a8 != '\x01') && ((int4)v3 >> 8 & 0xffU)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v2 = (int8)dat_9f830;
        error(0,0,gettext("%lu: Child returned status %d"),v2,(uint8)((int4)v3 >> 8 & 0xff));
        dat_9fdd0 = 2;
      }
      dat_9f830 = 0xffffffff;
      return;
    }
  } while (*__errno_location() == 4);
  dat_9f830 = 0xffffffff;
  sub_3eaf9(dat_9f9a0);
  return;
}


// Function: sub_345e7 @ 0x345e7
uint4 sub_345e7(unsigned long *a0,int4 a1)
{
  int4 v1;
  int4 v10; // stack - 0x50
  int4 v11; // stack - 0x4c
  void *v12; // stack - 0x38
  int4 v13; // stack - 0x2c
  void *v2; // rax
  int8 v3; // rax
  char *v4; // rax
  int4 v5; // stack - 0x30
  unsigned long v6; // stack - 0x40
  char *v7; // stack - 0x48
  uint4 v8; // stack - 0x54
  char v9 [24];
  
  sub_2b577(&v5);
  dat_9f840 = signal(0xd,1);
  v11 = sub_2b537();
  if (!v11) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*a0,1);
    setenv("TAR_VOLUME",(char *)sub_54e5a((int8)a1,v9),1);
    v3 = (int8)dat_9f868;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_54e5a(v3,v9),1);
    v4 = (char *)sub_34e1c(dat_9f860);
    setenv("TAR_SUBCOMMAND",v4,1);
    if (dat_9f468) // branch-flip
      v1 = dat_9f468;
    else {
      v1 = dat_9f864;
    }
    setenv("TAR_FORMAT",(char *)sub_34d66(v1),1);
    setenv("TAR_FD",(char *)sub_54e5a((int8)v13,v9),1);
    sub_bc11(v5);
    sub_5c835();
    sub_32aaa(dat_9f8c0); // no-return
  }
  v7 = NULL;
  v6 = 0;
  sub_bc11(v13);
  v2 = fdopen(v5,"r");
  v12 = v2;
  v10 = (int4)getline(&v7,&v6,v2);
  fclose(v12);
  if ((1 <= v10) && (v7[(int8)v10 + -1] == '\n')) {
    v10 -= 1;
    v7[v10] = '\0';
  }
  do {
    v1 = waitpid(v11,(int4 *)&v8,0);
    if (v1 != -1) {
      signal(0xd,dat_9f840);
      if (!(v8 & 0x7f)) {
        if (((int4)v8 >> 8 & 0xffU) || (v10 <= 0))
          free(v7);
        else {
          *a0 = v7;
        }
        return (int4)v8 >> 8 & 0xff;
      }
      free(v7);
      return 0xffffffff;
    }
  } while (*__errno_location() == 4);
  signal(0xd,dat_9f840);
  sub_3eaf9(dat_9f8c0);
  return 0xffffffff;
}


// Function: sub_348d7 @ 0x348d7
void sub_348d7(unsigned long a0,char *a1,int4 a2)
{
  int4 v1;
  int8 v2; // rax
  char *v3; // rax
  int8 v4;
  char v5 [24];
  int4 v6; // stack - 0x2c
  
  v6 = sub_2b537();
  if (!v6) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",a1,1);
    setenv("TAR_CHECKPOINT",(char *)sub_54e5a((int8)a2,v5),1);
    v2 = (int8)dat_9f868;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_54e5a(v2,v5),1);
    v3 = (char *)sub_34e1c(dat_9f860);
    setenv("TAR_SUBCOMMAND",v3,1);
    if (dat_9f468) // branch-flip
      v1 = dat_9f468;
    else {
      v1 = dat_9f864;
    }
    setenv("TAR_FORMAT",(char *)sub_34d66(v1),1);
    sub_5c835();
    sub_32aaa(a0); // no-return
  }
  do {
    v1 = waitpid(v6,&v4,0);
    if (v1 != -1)
      return;
  } while (*__errno_location() == 4);
  sub_3eaf9(a0);
  return;
}


// Function: sub_34a6f @ 0x34a6f
void sub_34a6f(unsigned long a0)
{
  int8 v1;
  
  if (dat_9fc40) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v1 = dat_9fc40;
    error(0,0,gettext("Options \'%s\' and \'%s\' both want standard input"),v1,a0);
    sub_3814c(2); // no-return
  }
  dat_9fc40 = a0;
  return;
}


// Function: sub_34af4 @ 0x34af4
bool sub_34af4(unsigned long a0,unsigned long a1)
{
  int4 v1; // eax
  int8 v2; // rax
  unsigned long v3; // stack - 0x18
  char *v4; // stack - 0x20
  bool v5; // stack - 0x21
  
  v5 = 0;
  if (!dat_9fcb0) {
    if ((dat_9f9cc) && (!dat_9fc40)) {
      sub_34a6f(0x8423a);
      dat_9fcb0 = stdin;
    }
    else {
      dat_9fcb0 = fopen("/dev/tty","r");
      if (!dat_9fcb0)
        sub_3e3b4("/dev/tty"); // no-return
    }
  }
  fprintf(dat_9efd0,"%s %s?",a0,sub_5e84e(a1));
  fflush_unlocked(dat_9efd0);
  if (!dat_9fcb8) {
    v4 = NULL;
    v3 = 0;
    v2 = getline(&v4,&v3,dat_9fcb0);
    if (0 <= v2) { // branch-flip
      v1 = rpmatch(v4);
      v5 = 0 < v1;
    }
    else {
      dat_9fcb8 = 1;
    }
    free(v4);
  }
  if (dat_9fcb8) {
    fputc_unlocked(10,dat_9efd0);
    fflush_unlocked(dat_9efd0);
  }
  return v5;
}


// Function: sub_34c7e @ 0x34c7e
void sub_34c7e(char *a0)
{
  char *v1;
  unsigned long v2; // rax
  int8 *v3; // stack - 0x20
  
  v3 = (int8 *)0x9a8e0;
  do {
    v1 = (char *)*v3;
    if (!strcmp(v1,a0)) {
      dat_9f864 = (int4)v3[1];
      return;
    }
    v3 = &v3[2];
  } while (*v3);
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = sub_5e548(a0);
  error(0,0,gettext("%s: Invalid archive format"),v2);
  sub_3814c(2); // no-return
}


// Function: sub_34d36 @ 0x34d36
void sub_34d36(int4 a0) // return-dupe
{
  if (a0 != 1) {
    dat_9f96c = a0;
    return;
  }
  sub_34c7e("posix");
  dat_9f96c = a0;
  return;
}


// Function: sub_34d66 @ 0x34d66
char * sub_34d66(int4 a0)
{
  int8 *v1; // stack - 0x10
  
  v1 = (int8 *)0x9a8e0;
  while( true ) {
    if (!*v1)
      return "unknown?";
    if (a0 == (int4)v1[1]) break;
    v1 = &v1[2];
  }
  return (char *)*v1;
}


// Function: sub_34dad @ 0x34dad
void sub_34dad(uint4 a0)
{
  if (!(1 << ((uint1)dat_9f864 & 0x1f) & a0)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("GNU features wanted on incompatible archive format"));
    sub_3814c(2); // no-return
  }
  return;
}


// Function: sub_34e1c @ 0x34e1c
char * sub_34e1c(unsigned int a0) // return-dupe x9
{
  switch(a0) {
    case 0:
      return "unknown?";
    case 1:
      return "-r";
    case 2:
      return "-A";
    case 3:
      return "-c";
    case 4:
      return "-D";
    case 5:
      return "-d";
    case 6:
      return "-x";
    case 7:
      return "-t";
    case 8:
      return "-u";
    case 9:
      return "--test-label";
    default:
      abort(); // no-return
    
  }
}


// Function: sub_34eb6 @ 0x34eb6
void sub_34eb6(int8 a0,char *a1)
{
  char *v1;
  uint8 v2; // rax
  uint8 v3; // rax
  int4 v4; // stack - 0x54
  
  v2 = strlen(a1);
  v4 = 0;
  while (*(int8 *)((int8)v4 * 8 + 0x9b6c0)) {
    if ((uint8)(*(int8 *)(a0 + 0x20) - *(int8 *)(a0 + 0x18)) < v2)
      _obstack_newchunk(a0,v2);
    memcpy(*(void **)(a0 + 0x18),a1,v2);
    *(uint8 *)(a0 + 0x18) = *(int8 *)(a0 + 0x18) + v2;
    v3 = strlen(*(char **)((int8)v4 * 8 + 0x9b6c0));
    if ((uint8)(*(int8 *)(a0 + 0x20) - *(int8 *)(a0 + 0x18)) < v3)
      _obstack_newchunk(a0,v3);
    memcpy(*(void **)(a0 + 0x18),*(void **)((int8)v4 * 8 + 0x9b6c0),v3);
    *(uint8 *)(a0 + 0x18) = *(int8 *)(a0 + 0x18) + v3;
    if (*(int8 *)(a0 + 0x20) == *(int8 *)(a0 + 0x18))
      _obstack_newchunk(a0,1);
    v1 = *(char **)(a0 + 0x18);
    *(char **)(a0 + 0x18) = &v1[1];
    *v1 = 10;
    v4 += 1;
  }
  return;
}


// Function: sub_35084 @ 0x35084
void sub_35084(char *a0)
{
  char *v1;
  unsigned long v2;
  int4 v3; // stack - 0x1c
  
  v3 = 0;
  while( true ) {
    if (!*(int8 *)((int8)v3 * 8 + 0x9b6c0)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v2 = dat_9fe48;
      error(0,0,gettext("Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list."),a0,v2);
      sub_177f4(); // no-return
    }
    v1 = *(char **)((int8)v3 * 8 + 0x9b6c0);
    if (!strcmp(a0,v1)) break;
    v3 += 1;
  }
  sub_5c9a2(0,v3);
  return;
}


// Function: sub_35152 @ 0x35152
void sub_35152(void)
{
  sub_61277("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",sub_34d66(6),0x867e7,0x14,"escape","/usr/local/libexec/rmt","/usr/bin/rsh");
  return;
}


// Function: sub_351b6 @ 0x351b6
void sub_351b6(unsigned long a0,unsigned long a1)
{
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("\'%s\' cannot be used with \'%s\'"),a0,a1);
  sub_3814c(2); // no-return
}


// Function: sub_3521d @ 0x3521d
unsigned long sub_3521d(uint4 a0,unsigned int *a1)
{
  char *v1;
  unsigned int *v2; // rax
  int8 v3; // stack - 0x20
  
  if (10 <= a0)
    abort(); // no-return
  v3 = 0x20;
  if (*(int8 *)&a1[2]) {
    v1 = *(char **)&a1[2];
    v3 = strlen(v1) + 0x21;
  }
  v2 = (unsigned int *)sub_60b5b(v3);
  if (*(int8 *)&a1[2]) { // branch-flip
    strcpy((char *)&v2[8],*(char **)&a1[2]);
    *(unsigned int **)&v2[2] = &v2[8];
  }
  else {
    *(unsigned long *)&v2[2] = 0;
  }
  *v2 = *a1;
  *(unsigned long *)&v2[4] = *(unsigned long *)&a1[4];
  *(unsigned long *)&v2[6] = *(unsigned long *)((uint8)a0 * 8 + 0x9fc60);
  *(unsigned int **)((uint8)a0 * 8 + 0x9fc60) = v2;
  return *(unsigned long *)&v2[6];
}


// Function: sub_35327 @ 0x35327
unsigned long sub_35327(int4 a0)
{
  return *(unsigned long *)((int8)a0 * 8 + 0x9fc60);
}


// Function: sub_3534c @ 0x3534c
bool sub_3534c(unsigned int a0) // early-return
{
  int4 *v1; // rax
  
  v1 = (int4 *)sub_35327(a0);
  if (!v1)
    return 0;
  return *v1 == 1;
}


// Function: sub_35388 @ 0x35388
bool sub_35388(int4 *a0,int4 *a1) // early-return x2
{
  char *v1;
  char *v2;
  
  if (*a0 != *a1)
    return 0;
  if (*a0 == 1)
    return 1;
  v1 = *(char **)&a1[2];
  v2 = *(char **)&a0[2];
  return strcmp(v2,v1) == 0;
}


// Function: sub_353ea @ 0x353ea
void sub_353ea(int4 a0)
{
  if ((dat_9f860) && (a0 != dat_9f860)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("You may not specify more than one \'-Acdtrux\', \'--delete\' or  \'--test-label\' option"));
    sub_3814c(2); // no-return
  }
  dat_9f860 = a0;
  return;
}


// Function: sub_3545f @ 0x3545f
void sub_3545f(char *a0,unsigned long a1)
{
  int4 v1; // eax
  int4 *v2; // rax
  
  v2 = (int4 *)sub_3521d(0,a1);
  if (dat_9f898) {
    v1 = strcmp(dat_9f898,a0);
    if ((v1) && (*v2 == 1)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Conflicting compression options"));
      sub_3814c(2); // no-return
    }
  }
  dat_9f898 = a0;
  return;
}


// Function: sub_3550c @ 0x3550c
void sub_3550c(unsigned int a0)
{
  sub_ae4d();
  sub_baa2();
  signal(a0,sub_3550c);
  return;
}


// Function: sub_3553c @ 0x3553c
void sub_3553c(unsigned int a0)
{
  signal(a0,sub_3550c);
  return;
}


// Function: sub_35562 @ 0x35562
unsigned int sub_35562(char *a0)
{
  char *v1;
  unsigned long *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v3 = a0;
  if (!strncmp(a0,"SIG",3))
    v3 = &a0[3];
  v2 = (unsigned long *)0x9a9c0;
  while( true ) {
    if ((unsigned long *)0x9aa10 <= v2) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Unknown signal name: %s"),a0);
      sub_177f4(); // no-return
    }
    v1 = (char *)*v2;
    if (!strcmp(v1,v3)) break;
    v2 = &v2[2];
  }
  return *(unsigned int *)&v2[1];
}


// Function: sub_35629 @ 0x35629
void sub_35629(unsigned long a0)
{
  sub_3553c(sub_35562(a0));
  return;
}


// Function: sub_3564f @ 0x3564f
unsigned long sub_3564f(int8 a0,unsigned long a1,char *a2,unsigned long *a3) // return-dupe
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long *v3; // rax
  char v4 [152];
  char v5 [16];
  
  if ((*a2 != '/') && (*a2 != '.')) {
    if (sub_5a8ed(a3,a2,0) == '\x01') {
      v3 = (unsigned long *)sub_60b5b(0x28);
      v1 = a3[1];
      v3[1] = *a3;
      v3[2] = v1;
      v3[3] = a1;
      v3[4] = sub_60cc6(a2);
      *v3 = *(unsigned long *)(a0 + 8);
      *(unsigned long **)(a0 + 8) = v3;
      return 0;
    }
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v1 = sub_5e84e(a2);
    v2 = sub_27abc(*a3,a3[1],0);
    error(0,0,gettext("Substituting %s for unknown date format %s"),v2,v1);
    a3[1] = 0;
    return 1;
  }
  if (!stat(a2,v4)) {
    v5 = sub_5fd9e(v4);
    *a3 = SUB168(v5,0);
    a3[1] = SUB168(v5,8);
    return 0;
  }
  sub_3ea03(a2);
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("Date sample file not found"));
  sub_3814c(2); // no-return
}


// Function: sub_3589e @ 0x3589e
void sub_3589e(int8 a0)
{
  unsigned long *v1;
  char *v2;
  unsigned long v3;
  unsigned long v4;
  char *v5; // rax
  unsigned long *v6; // stack - 0x30
  
  v6 = *(unsigned long **)(a0 + 8);
  while (v6) {
    v1 = (unsigned long *)*v6;
    if (dat_9f9ac) {
      v5 = (char *)sub_27abc(v6[1],v6[2],1);
      v2 = (char *)v6[4];
      if (strcmp(v2,v5)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v3 = v6[4];
        v4 = v6[3];
        error(0,0,gettext("Option %s: Treating date \'%s\' as %s"),v4,v3,v5);
      }
    }
    free((void *)v6[4]);
    free(v6);
    v6 = v1;
  }
  return;
}


// Function: sub_359a8 @ 0x359a8
unsigned long sub_359a8(int4 a0,unsigned long a1)
{
  char *v1;
  uint8 v10; // stack - 0x38
  uint1 v11; // stack - 0x18
  char *v2; // rax
  uint8 v3; // rax
  char v4 [8];
  unsigned long v5; // stack - 0x118
  int8 v6; // stack - 0x60
  char *v7; // stack - 0x58
  char *v8; // stack - 0x50
  char *v9; // stack - 0x48
  
  if (a0 != 0x2000004) { // branch-flip
    switch(a0) {
      default:
        v5 = a1;
        break;
      case 0x4a:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e61);
        break;
      case 0x5a:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,"compress");
        break;
      case 0x6a:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,"bzip2");
        break;
      case 0x7a:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e2b);
        break;
      case 0x94:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e52);
        break;
      case 0x95:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e57);
        break;
      case 0x96:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e5c);
        break;
      case 0xcf:
        v2 = gettext("filter the archive through %s");
        v5 = sub_61277(v2,0x85e64);
      
    }
  }
  else {
    _obstack_begin(v4,0,0,sub_60b5b,dat_9bfc0);
    v2 = gettext("Valid arguments for the --quoting-style option are:");
    v3 = strlen(v2);
    if ((uint8)((int8)v9 - (int8)v8) < v3)
      _obstack_newchunk(v4,v3);
    memcpy(v8,v2,v3);
    v8 = (void *)((int8)v8 + v3);
    if ((uint8)((int8)v9 - (int8)v8) < 2)
      _obstack_newchunk(v4,2);
    memcpy(v8,(void *)0x869c4,2);
    v8 = (void *)((int8)v8 + 2);
    sub_34eb6(v4,0x869c7);
    v2 = gettext("\n*This* tar defaults to:\n");
    v3 = strlen(v2);
    if ((uint8)((int8)v9 - (int8)v8) < v3)
      _obstack_newchunk(v4,v3);
    memcpy(v8,v2,v3);
    v8 = (void *)((int8)v8 + v3);
    v2 = (char *)sub_35152();
    v3 = strlen(v2);
    if ((uint8)((int8)v9 - (int8)v8) < v3)
      _obstack_newchunk(v4,v3);
    memcpy(v8,v2,v3);
    v8 = (char *)((int8)v8 + v3);
    if (v9 == v8)
      _obstack_newchunk(v4,1);
    v1 = &v8[1];
    *v8 = 10;
    v8 = v1;
    if (v9 == v1)
      _obstack_newchunk(v4,1);
    v1 = v7;
    *v8 = 0;
    if (v7 == &v8[1])
      v11 |= 2;
    v8 = (char *)(~v10 & (uint8)&(&v8[1])[v10]);
    if ((uint8)((int8)v9 - v6) < (uint8)((int8)v8 - v6))
      v8 = v9;
    v7 = v8;
    v5 = sub_60cc6(v1);
    _obstack_free(v4,0);
  }
  return v5;
}


// Function: sub_361aa @ 0x361aa
unsigned long sub_361aa(unsigned long a0,char *a1)
{
  uint8 v1; // rax
  char v10 [40];
  char *v11; // stack - 0x1a8
  uint1 *v12; // stack - 0x190
  uint8 v13; // stack - 0x188
  unsigned long v14; // stack - 0xa0
  int8 v15; // stack - 0x90
  char *v16; // stack - 0x88
  char *v17; // stack - 0x80
  char *v18; // stack - 0x78
  uint8 v19; // stack - 0x68
  uint1 *v2;
  uint1 v20; // stack - 0x48
  uint1 *v3; // rax
  void *v4; // rax
  char *v5;
  uint8 v6; // rax
  unsigned long v7; // rax
  char v8 [8];
  unsigned long v9; // stack - 0xa8
  
  _obstack_begin(v8,0,0,sub_60b5b,dat_9bfc0);
  v11 = a1;
  while (*v11) {
    v1 = strcspn(v11,",");
    v2 = memchr(v11,0x3d,v1);
    if (v2) { // branch-flip
      v3 = &v2[1 - (int8)v11];
      if (&v18[-(int8)v17] < v3)
        _obstack_newchunk(v8,v3);
      memcpy(v17,v11,(uint8)v3);
      v17 = &v17[(int8)v3];
      v13 = v1 - (int8)v3;
      while ((v12 = &v2[1], *v12 && (*(uint2 *)((uint8)*v12 * 2 + *(int8 *)__ctype_b_loc()) & 0x2000))) {
        v13 -= 1;
        v2 = v12;
      }
      if ((*v12 != 0x7b) || (v12[v13 - 1] != 0x7d)) {
        if ((uint8)((int8)v18 - (int8)v17) < v13)
          _obstack_newchunk(v8,v13);
        memcpy(v17,v12,v13);
        v17 = &v17[v13];
      }
      else {
        v4 = (void *)sub_60b5b(v13);
        memcpy(v4,&v2[2],v13 - 2);
        *(char *)((int8)v4 + (v13 - 2)) = 0;
        if (sub_3564f(a0,"--pax-option",v4,&v9)) { // branch-flip
          if ((uint8)((int8)v18 - (int8)v17) < v13)
            _obstack_newchunk(v8,v13);
          memcpy(v17,v12,v13);
          v17 = &v17[v13];
        }
        else {
          v5 = (char *)sub_29f2e(v9,v14,v10);
          v6 = strlen(v5);
          if ((uint8)((int8)v18 - (int8)v17) < v6)
            _obstack_newchunk(v8,v6);
          memcpy(v17,v5,v6);
          v17 = &v17[v6];
        }
        free(v4);
      }
    }
    else {
      if ((uint8)((int8)v18 - (int8)v17) < v1)
        _obstack_newchunk(v8,v1);
      memcpy(v17,v11,v1);
      v17 = &v17[v1];
    }
    v11 = &v11[v1];
    if (*v11) {
      if (v18 == v17)
        _obstack_newchunk(v8,1);
      v5 = &v17[1];
      *v17 = *v11;
      v11 = &v11[1];
      v17 = v5;
    }
  }
  if (v18 == v17)
    _obstack_newchunk(v8,1);
  v5 = v16;
  *v17 = '\0';
  if (v16 == &v17[1])
    v20 |= 2;
  v17 = (char *)(~v19 & (uint8)&(&v17[1])[v19]);
  if ((uint8)((int8)v18 - v15) < (uint8)((int8)v17 - v15))
    v17 = v18;
  v16 = v17;
  v7 = sub_60cc6(v5);
  _obstack_free(v8,0);
  return v7;
}


// Function: sub_369f0 @ 0x369f0
uint8 sub_369f0(char *a0,uint8 a1,unsigned long *a2)
{
  int4 v1; // eax
  char *v10; // stack - 0x28
  char *v2; // rax
  unsigned long v3; // rax
  uint8 v4; // stack - 0x58
  char v5 [8];
  uint8 v6; // stack - 0x48
  char *v7; // stack - 0x40
  char *v8; // stack - 0x38
  char *v9; // stack - 0x30
  
  v4 = 0xffffffffffffffff;
  v7 = NULL;
  v8 = NULL;
  v9 = strchr(a0,0x3a);
  if (v9) { // branch-flip
    v10 = &v9[1];
    *v9 = '\0';
    if (*a0)
      v7 = a0;
    if ((v10) && ((v1 = sub_60e19(v10,v5,10,&v4,0x869f3), v1 || (a1 < v4))))
      v8 = v10;
  }
  else {
    if (('0' <= *a0) && (*a0 <= '9'))
      v1 = sub_60e19(a0,v5,10,&v6,0x869f3);
    else {
      v1 = 4;
    }
    if (v1) { // branch-flip
      if (v1 != 1) {
        v7 = a0;
        goto label_36b28;
      }
    }
    else if (v6 <= a1) {
      v4 = v6;
      goto label_36b28;
    }
    v8 = a0;
  }
label_36b28:
  if (!v8) {
    if (v7)
      *a2 = v7;
    return v4;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  v2 = gettext("Invalid owner or group ID");
  v3 = sub_5e548(v8);
  error(0,0,"%s: %s",v3,v2);
  sub_177f4(); // no-return
}


// Function: sub_36bbb @ 0x36bbb
void sub_36bbb(uint4 a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_3521d(9,a1);
  if (v1) {
    if ((sub_35388(a1,v1)) && (a0 != dat_9f8d8))
      sub_351b6(*(unsigned long *)((int8)(int4)a0 * 8 + 0x9aa20),*(unsigned long *)((uint8)dat_9f8d8 * 8 + 0x9aa20)); // no-return
  }
  dat_9f8d8 = a0;
  return;
}


// Function: sub_36c56 @ 0x36c56
unsigned long sub_36c56(int4 a0,char *a1,int8 *a2) // return-dupe, return-dupe x2
{
  char v1;
  int8 *v10; // stack - 0x48
  void *v11; // stack - 0x40
  void *v12; // stack - 0x38
  int8 v13; // stack - 0x30
  int8 v14; // stack - 0x28
  unsigned long v2;
  unsigned long v3;
  int8 v4;
  char *v5;
  char *v6; // stack - 0x50
  uint8 v7; // stack - 0x58
  char *v8; // stack - 0x78
  int4 v9; // stack - 0x5c
  
  v10 = (int8 *)a2[5];
  if (a0 == 0x1000005) {
    if (*(int4 *)*v10 != 2) { // branch-flip
      if (!*(int4 *)*v10) {
        v2 = *(unsigned long *)(*v10 + 8);
        error(0,0,gettext("error parsing %s"),v2);
      }
    }
    else {
      v2 = *(unsigned long *)(*v10 + 0x10);
      v3 = *(unsigned long *)(*v10 + 8);
      error(0,0,gettext("%s:%lu: location of the error"),v3,v2);
    }
    exit(0x40); // no-return
  }
  if (0x1000006 <= a0)
    return 7;
  if (0xd0 <= a0) {
    if (a0 != 0x1000003)
      return 7;
    if (!*(int8 *)(*a2 + 0x20))
      return 0;
    v9 = 0;
    while (*(int8 *)((int8)v9 * 0x20 + *(int8 *)(*a2 + 0x20))) {
      *(int8 *)(a2[6] + (int8)v9 * 8) = a2[5];
      v9 += 1;
    }
    return 0;
  }
  v8 = a1;
  switch(a0) {
    case 0:
      sub_2c829(a1);
      break;
    default:
      return 7;
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
      argp_error(a2,gettext("Options \'-[0-7][lmh]\' not supported by *this* tar"));
      exit(0x40); // no-return
    case 0x41:
      sub_353ea(2);
      break;
    case 0x42:
      dat_9f94d = 1;
      break;
    case 0x46:
      dat_9f8fc = 1;
      dat_9f8c0 = a1;
      break;
    case 0x48:
      sub_34c7e(a1);
      break;
    case 0x49:
      sub_3545f(a1,*v10);
      break;
    case 0x4a:
      sub_3545f(0x85e61,*v10);
      break;
    case 0x4b:
      sub_3521d(5,*v10);
      sub_2d6a2(a1);
      break;
    case 0x4c:
      if (sub_60e19(a1,&v6,10,&v7,"bBcGgkKMmPTtw")) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid tape length");
        error(0,0,"%s: %s",sub_5e548(a1),v5);
        sub_3814c(2); // no-return
      }
      if ((v6 <= a1) || (v1 = v6[-1], strchr("bBcGgkKMmPTtw",(int4)v1)))
        dat_9f990 = (float8)v7;
      else {
        dat_9f990 = (float8)v7 * dat_87670;
      }
      dat_9f8fc = 1;
      break;
    case 0x4d:
      dat_9f8fc = 1;
      break;
    case 0x4e:
      dat_9f87c = 1;
      goto label_372f8;
    case 0x4f:
      dat_9f998 = 1;
      break;
    case 0x50:
      sub_3521d(8,*v10);
      dat_9f878 = 1;
      break;
    case 0x52:
      dat_9f88c = 1;
      break;
    case 0x53:
      dat_9f979 = 1;
      break;
    case 0x55:
      sub_36bbb(3,*v10);
      break;
    case 0x56:
      dat_9f9c0 = a1;
      break;
    case 0x57:
      sub_3521d(4,*v10);
      dat_9f9b0 = 1;
      break;
    case 0x5a:
      sub_3545f("compress",*v10);
      break;
    case 0x61:
      *(char *)((int8)v10 + 0x12) = 1;
      break;
    case 0x62:
      if ((((sub_60e19(a1,0,10,&v6,0x869f3)) || (dat_9f868 = (int4)v6, (char *)(int8)dat_9f868 != v6)) || (dat_9f868 <= 0)) || (dat_9f870 = (char *)((int8)v6 << 9), (char *)((uint8)v6 & 0x7fffffffffffff) != v6)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid blocking factor");
        error(0,0,"%s: %s",sub_5e548(a1),v5);
        sub_3814c(2); // no-return
      }
      break;
    case 99:
      sub_353ea(3);
      break;
    case 100:
      sub_353ea(5);
      break;
    case 0x66:
      if (dat_9fbe0 == dat_9fc38)
        dat_9fbd8 = sub_60a5f(dat_9fbd8,0x9fc38,8);
      v4 = dat_9fbe0 + 1;
      *(char **)(dat_9fbd8 + dat_9fbe0 * 8) = a1;
      dat_9fbe0 = v4;
      break;
    case 0x67:
      sub_3521d(2,*v10);
      dat_9f87c = 1;
      dat_9f8e0 = a1;
      dat_9f8be = 1; // return-dupe
      return 0;
    case 0x47:
      dat_9f8be = 1;
      break;
    case 0x68:
      dat_9f8a0 = 1;
      break;
    case 0x69:
      dat_9f8bd = 1;
      break;
    case 0x6a:
      sub_3545f("bzip2",*v10);
      break;
    case 0x6b:
      sub_36bbb(4,*v10);
      break;
    case 0x6c:
      dat_9fc34 = 1;
      break;
    case 0x6d:
      dat_9f99a = 1;
      break;
    case 0x6e:
      dat_9fc14 = 1;
      break;
    case 0x6f:
      *(char *)&v10[2] = 1;
      break;
    case 0x70:
      dat_9f960 = 1;
      break;
    case 0x72:
      sub_353ea(1);
      break;
    case 0x73:
      sub_3521d(6,*v10);
      dat_9f958 = 1;
      break;
    case 0x74:
      sub_353ea(7);
      dat_9f9ac += 1;
      break;
    case 0x75:
      sub_353ea(8);
      break;
    case 0x76:
      dat_9f9ac += 1;
      dat_9ec84 |= 0x583000;
      break;
    case 0x77:
      dat_9f8c8 = 1;
      break;
    case 0x78:
      sub_353ea(6);
      break;
    case 0x7a:
      sub_3545f(0x85e2b,*v10);
      break;
    case 0x80:
      sub_34c7e("posix");
      dat_9f968 = 1;
      break;
    case 0x81:
      if (a1) { // branch-flip
        v4 = sub_47c61("--atime-preserve",a1,0x9a950,0x867c8,4,dat_9ee48);
        dat_9f880 = *(unsigned int *)(v4 * 4 + 0x867c8);
      }
      else {
        dat_9f880 = 1;
      }
      break;
    case 0x82:
      dat_9f884 = 1;
      if (a1)
        v10[4] = (int8)a1;
      break;
    case 0x83:
      dat_9f8ec = 1;
      break;
    case 0x84:
      if (a1) { // branch-flip
        if (*a1 == '.') {
          sub_f825(0x86b47);
          v8 = &a1[1];
        }
        dat_9f890 = (unsigned int)strtoul(v8,&v6,0);
        if (*v6) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("--checkpoint value is not an integer"));
          sub_177f4(); // no-return
        }
      }
      else {
        dat_9f890 = 10;
      }
      break;
    case 0x85:
      sub_f825(a1);
      break;
    case 0x86:
      dat_9f910 = 2;
      break;
    case 0x87:
      dat_9fc31 = 1;
      break;
    case 0x88:
      dat_9f8a1 = 1;
      break;
    case 0x89:
      sub_353ea(4);
      break;
    case 0x8a:
      dat_9fdf8 = 1;
      break;
    case 0x8b:
      dat_9f87a = 1;
      break;
    case 0x8c:
      v14 = sub_369f0(a1,0xffffffff,0x9f8b0);
      if (v14 != -1) // branch-flip
        dat_9f8b8 = (unsigned int)v14;
      else {
        dat_9f8b8 = 0xffffffff;
        if (dat_9f8b0)
          sub_2c261(dat_9f8b0,0x9f8b8);
      }
      break;
    case 0x8d:
      sub_2938e(a1);
      break;
    case 0x8e:
      dat_9f9a8 = 1;
      break;
    case 0x8f:
      dat_9f8bc = 1;
      break;
    case 0x90:
      dat_9fbf0 = a1;
      break;
    case 0x91:
      dat_9f8dc = 1;
      break;
    case 0x92:
      sub_36bbb(6,*v10);
      break;
    case 0x93:
      dat_9f8e8 = (unsigned int)strtoul(a1,&v6,10);
      if (*v6) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Invalid incremental level value"));
        sub_3814c(2); // no-return
      }
      break;
    case 0x94:
      sub_3545f(0x85e52,*v10);
      break;
    case 0x95:
      sub_3545f(0x85e57,*v10);
      break;
    case 0x96:
      sub_3545f(0x85e5c,*v10);
      break;
    case 0x97:
      dat_9f8f0 = sub_55658(a1);
      if (!dat_9f8f0) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Invalid mode given on option"));
        sub_177f4(); // no-return
      }
      dat_9f8f8 = umask(0);
      umask(dat_9f8f8);
      break;
    case 0x98:
      sub_3564f(v10,"--mtime",a1,0x9f920);
      if (!dat_9f910)
        dat_9f910 = 1;
      break;
    case 0x99:
label_372f8:
      if (0 <= dat_9f908) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("More than one threshold date"));
        sub_3814c(2); // no-return
      }
      if (a0 != 0x99) // branch-flip
        v5 = "--after-date";
      else {
        v5 = "--newer-mtime";
      }
      sub_3564f(v10,v5,a1,0x9f900);
      sub_3521d(3,*v10);
      break;
    case 0x9a:
      dat_9f968 = 0xffffffff;
      break;
    case 0x9b:
      *(char *)((int8)v10 + 0x12) = 0;
      break;
    case 0x9c:
      dat_9f8ec = 0;
      break;
    case 0x9d:
      dat_9fc31 = 0;
      break;
    case 0x9e:
      dat_9f9a8 = 0;
      break;
    case 0x9f:
      sub_36bbb(1,*v10);
      break;
    case 0xa0:
      for (; *v8; v8 = &v8[1]) {
        sub_5c9cd(0,(int4)*v8,0);
      }
      break;
    case 0xa1:
      dat_9f95c = 0xffffffff;
      break;
    case 0xa2:
      dat_9f960 = 0xffffffff;
      break;
    case 0xa3:
      dat_9fc14 = 0;
      break;
    case 0xa4:
      dat_9f964 = 0xffffffff;
      break;
    case 0xa5:
      sub_34d36(0xffffffff);
      break;
    case 0xa6:
      dat_9f934 = 1;
      break;
    case 0xa7:
      sub_3521d(1,*v10);
      if (a1) { // branch-flip
        if (sub_60e19(a1,0,10,&v6,0x869f3)) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          v5 = gettext("Invalid number");
          error(0,0,"%s: %s",sub_5e548(a1),v5);
          sub_177f4(); // no-return
        }
        dat_9f8d0 = v6;
      }
      else {
        dat_9f8d0 = (char *)0x1;
      }
      break;
    case 0xa8:
      sub_34c7e(0x84244);
      break;
    case 0xa9:
      dat_9f935 = 1;
      break;
    case 0xaa:
      sub_3521d(7,*v10);
      dat_9f936 = 1;
      dat_9f938 = a1;
      break;
    case 0xab:
      sub_36bbb(0,*v10);
      break;
    case 0xac:
      sub_36bbb(2,*v10);
      break;
    case 0xad:
      v13 = sub_369f0(a1,0xffffffff,0x9f940);
      if (v13 != -1) // branch-flip
        dat_9f948 = (unsigned int)v13;
      else {
        dat_9f948 = 0xffffffff;
        if (dat_9f940)
          sub_2c173(dat_9f940,0x9f948);
      }
      break;
    case 0xae:
      sub_29234(a1);
      break;
    case 0xaf:
      v12 = (void *)sub_361aa(v10,a1);
      *(char *)((int8)v10 + 0x11) = 1;
      sub_1cc78(v12);
      free(v12);
      break;
    case 0xb0:
      sub_34c7e("posix");
      break;
    case 0xb1:
      for (; *v8; v8 = &v8[1]) {
        sub_5c9cd(0,(int4)*v8,1);
      }
      break;
    case 0xb2:
      sub_35084(a1);
      break;
    case 0xb3:
      if (sub_60e19(a1,0,10,&v6,"bBcGgkKMmPTtw")) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid record size");
        error(0,0,"%s: %s",sub_5e548(a1),v5);
        sub_3814c(2); // no-return
      }
      dat_9f870 = v6;
      if ((uint8)v6 & 0x1ff) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Record size must be a multiple of %d."),0x200);
        sub_3814c(2); // no-return
      }
      dat_9f868 = (int4)((uint8)v6 >> 9);
      break;
    case 0xb4:
      dat_9f94c = 1;
      break;
    case 0xb5:
      dat_9f94e = 1;
      break;
    case 0xb6:
      dat_9f9a9 = 1;
      break;
    case 0xb7:
      dat_9ece0 = a1;
      break;
    case 0xb8:
      dat_9f950 = a1;
      break;
    case 0xb9:
      dat_9f95c = 1;
      break;
    case 0xba:
      sub_34c7e("posix");
      dat_9f964 = 1;
      break;
    case 0xbb:
      v11 = (void *)sub_35152();
      printf("%s\n",v11);
      sub_4e014();
      free(v11);
      exit(0); // no-return
    case 0xbc:
      dat_9f978 = 1;
      break;
    case 0xbd:
      sub_23f6b();
      sub_4e014();
      exit(0); // no-return
    case 0xbe:
      dat_9fc30 = 1;
      break;
    case 0xbf:
      sub_36bbb(5,*v10);
      break;
    case 0xc0:
      v4 = sub_47c61("--sort",a1,0x9a980,0x9cb40,4,dat_9ee48);
      dat_9fc2c = *(unsigned int *)(v4 * 4 + 0x9cb40);
      break;
    case 0xc1:
      v4 = sub_47c61("--hole-detection",a1,0x9a9a0,0x86a30,4,dat_9ee48);
      dat_9f984 = *(unsigned int *)(v4 * 4 + 0x86a30);
      dat_9f979 = 1;
      break;
    case 0xc2:
      dat_9f979 = 1;
      dat_9f97c = (unsigned int)strtoul(a1,&v6,10);
      if (*v6) {
        if (*v6 != '.') {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Invalid sparse version value"));
          sub_3814c(2); // no-return
        }
        v5 = &v6[1];
        dat_9f980 = (unsigned int)strtoul(v5,&v6,10);
        if (*v6) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Invalid sparse version value"));
          sub_3814c(2); // no-return
        }
      }
      break;
    case 0xc3:
      if (sub_60e19(a1,0,10,&v6,0x869f3)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = gettext("Invalid number of elements");
        error(0,0,"%s: %s",sub_5e548(a1),v5);
        sub_3814c(2); // no-return
      }
      dat_9f970 = v6;
      break;
    case 0xc4:
      dat_9f884 = 1;
      v10[3] = (int8)a1;
      break;
    case 0xc5:
      sub_353ea(9);
      break;
    case 0xc6:
      if (a1) // branch-flip
        sub_35629(a1);
      else {
        dat_9f999 = 1;
      }
      break;
    case 199:
      if (dat_9f9a0) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Only one --to-command option allowed"));
        sub_3814c(2); // no-return
      }
      dat_9f9a0 = a1;
      break;
    case 200:
      sub_3ae0b(a1);
      break;
    case 0xc9:
      dat_9f879 = 1;
      break;
    case 0xca:
      dat_9f9b8 = a1;
      break;
    case 0xcb:
      sub_3c9ee(a1);
      break;
    case 0xcc:
      sub_34d36(1);
      break;
    case 0xcd:
    case 0xce:
      sub_34d36(1);
      sub_3d0e5(a1,a0 == 0xce);
      break;
    case 0xcf:
      sub_3545f(0x85e64,*v10);
    
  }
  return 0;
}


// Function: sub_3814c @ 0x3814c
void sub_3814c(int4 a0)
{
  argp_help(0x9eb60,stderr,4,dat_9fe48);
  sub_4e014();
  exit(a0); // no-return
}


// Function: sub_38192 @ 0x38192
int8 * sub_38192(int8 *a0,int4 a1)
{
  int8 *v1; // stack - 0x10
  
  for (v1 = a0; (((*v1 || ((int4)v1[1])) || (v1[2])) || (((int4)v1[3] || (v1[4])))); v1 = &v1[6]) {
    if (a1 == (int4)v1[1])
      return v1;
  }
  return NULL;
}


// Function: sub_381fd @ 0x381fd
int8 sub_381fd(unsigned long *a0,unsigned int a1)
{
  int8 v1; // rax
  int8 v2; // stack - 0x18
  int8 *v3; // stack - 0x10
  
  v2 = sub_38192(*a0,a1);
  if ((!v2) && (a0[4])) {
    for (v3 = (int8 *)a0[4]; *v3; v3 = &v3[4]) {
      v1 = sub_38192(*(unsigned long *)*v3,a1);
      if (v1)
        return v1;
      v2 = 0;
    }
  }
  return v2;
}


// Function: sub_3828e @ 0x3828e
void sub_3828e(unsigned int a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x38
  unsigned long v2; // stack - 0x30
  char v3; // stack - 0x28
  char v4; // stack - 0x27
  char v5; // stack - 0x26
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v2 = 0;
  v3 = 0;
  v4 = 0;
  v5 = 0;
  v6 = 0;
  v7 = 0;
  v1 = a2;
  argp_parse(0x9c920,a0,a1,0x2a,0,&v1);
  return;
}


// Function: sub_38321 @ 0x38321
void sub_38321(int8 *a0) // return-dupe
{
  char *v1;
  unsigned long v10; // stack - 0x118
  unsigned long v11; // stack - 0x110
  unsigned long *v12; // stack - 0x100
  unsigned long v13; // stack - 0xf8
  int4 v2; // eax
  unsigned long v3; // rax
  int8 v4; // stack - 0x108
  unsigned int v5 [2]; // stack - 0x128
  char v6 [4];
  char *v7; // stack - 0x138
  int8 v8; // stack - 0x130
  char *v9; // stack - 0x120
  
  v7 = getenv("TAR_OPTIONS");
  v5[0] = 0;
  v9 = "TAR_OPTIONS";
  v10 = 0;
  v11 = 0;
  if (!v7)
    return;
  v13 = 1;
  v2 = sub_461a6(v7,&v4,0x2000e46);
  if (v2) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    v3 = sub_4647f(&v4);
    error(0,0,gettext("cannot split TAR_OPTIONS: %s"),v3);
    sub_177f4(); // no-return
  }
  if (v4) {
    *v12 = dat_9fe48;
    v8 = *a0;
    *a0 = (int8)v5;
    v2 = (int4)v13;
    if (argp_parse(0x9eb60,(int4)v4 + v2,v12,0x28,v6,a0))
      abort(); // no-return
    *a0 = v8;
    if (sub_34a5e()) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = v9;
      error(0,0,gettext("non-option arguments in %s"),v1);
      sub_3814c(2); // no-return
    }
    v4 = 0;
  }
  sub_46399(&v4);
  return;
}


// Function: sub_3854d @ 0x3854d
void sub_3854d(int4 a0,unsigned long *a1) // ternary x5
{
  unsigned long *v1;
  unsigned int v10 [2]; // stack - 0x78
  char v11; // stack - 0x23
  unsigned int *v12; // stack - 0x58
  int4 v13; // stack - 0xec
  unsigned long *v14; // stack - 0x108
  int4 v15; // stack - 0xfc
  int4 v16; // stack - 0xe8
  uint4 v17; // stack - 0xe4
  unsigned long *v18; // stack - 0xe0
  unsigned long *v19; // stack - 0xd8
  unsigned long *v2;
  char *v20; // stack - 0xd0
  unsigned long *v21; // stack - 0xc8
  int8 v22; // stack - 0xc0
  int4 *v23; // stack - 0xb8
  unsigned long v24; // stack - 0xb0
  uint8 v25; // stack - 0xa8
  int4 *v26; // stack - 0xa0
  unsigned long v27; // stack - 0x98
  void *v28; // stack - 0x90
  int4 *v29; // stack - 0x88
  char v3;
  unsigned long v30; // stack - 0x80
  unsigned long v31; // stack - 0x70
  unsigned long v32; // stack - 0x68
  unsigned long v33; // stack - 0x60
  unsigned long v34; // stack - 0x50
  char v35; // stack - 0x48
  char v36; // stack - 0x47
  char v37; // stack - 0x46
  char *v38; // stack - 0x40
  unsigned long v39; // stack - 0x38
  int8 *v4;
  char v40; // stack - 0x22
  char v41; // stack - 0x21
  int4 v5; // eax
  unsigned long v6; // rax
  uint8 v7; // rax
  char *v8;
  uint4 v9; // edx
  
  v10[0] = 1;
  v31 = 0;
  v32 = 0;
  v33 = 0;
  v12 = v10;
  v34 = 0;
  v35 = '\0';
  v36 = '\0';
  v37 = '\0';
  v38 = NULL;
  v39 = 0;
  sub_4d302(0x87004,0x9eba0);
  v38 = getenv("SIMPLE_BACKUP_SUFFIX");
  dat_9f9c8 = getenv("POSIXLY_CORRECT") != NULL;
  dat_9f860 = 0;
  dat_9f864 = 0;
  dat_9f868 = 0x14;
  dat_9f870 = 0x2800;
  dat_9f8a8 = sub_4e53e();
  dat_9f984 = 0;
  dat_9f900 = 0x8000000000000000;
  dat_9f908 = -1;
  dat_9f920 = 0x8000000000000000;
  dat_9f928 = -1;
  dat_9f930 = 8;
  dat_9fc28 = 1;
  dat_9f97c = 1;
  dat_9f980 = 0;
  dat_9fc2c = 0;
  dat_9f948 = 0xffffffff;
  dat_9f940 = 0;
  dat_9f8b8 = 0xffffffff;
  dat_9f8b0 = 0;
  dat_9f8ec = 1;
  dat_9f8e8 = -1;
  dat_9fc14 = 0xffffffff;
  v14 = a1;
  v15 = a0;
  if ((2 <= a0) && (*(char *)a1[1] != '-')) {
    v11 = 0x2d;
    v41 = 0;
    v8 = (char *)a1[1];
    v5 = a0 + (int4)strlen(v8);
    v16 = v5 + -1;
    v21 = (unsigned long *)sub_60b5b((int8)v5 << 3);
    v19 = &v21[1];
    *v21 = *a1;
    v18 = &a1[2];
    for (v20 = (char *)a1[1]; v1 = v19, *v20; v20 = &v20[1]) {
      v40 = *v20;
      v19 = &v19[1];
      *v1 = sub_60cc6(&v11);
      v22 = sub_381fd(0x9eb60,(int4)*v20);
      if (v22) {
        if (*(int8 *)(v22 + 0x10)) {
          if (&a1[a0] <= v18) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v3 = *v20;
            error(0,0,gettext("Old option \'%c\' requires an argument."),(uint8)(uint4)(int4)v3);
            sub_3814c(2); // no-return
          }
          v1 = &v18[1];
          v2 = &v19[1];
          *v19 = *v18;
          v18 = v1;
          v19 = v2;
        }
      }
    }
    while (v18 < &a1[a0]) {
      v1 = &v18[1];
      v2 = &v19[1];
      *v19 = *v18;
      v18 = v1;
      v19 = v2;
    }
    *v19 = 0;
    v15 = v16;
    v14 = v21;
  }
  sub_38321(&v12);
  if (argp_parse(0x9eb60,v15,v14,8,&v13,&v12))
    exit(2); // no-return
  if (v35) {
    if (dat_9f860 != 3) // branch-flip
      dat_9f95c = 0xffffffff;
    else {
      sub_34c7e(0x84244);
    }
  }
  for (; v13 < v15; v13 = v13 + 1) {
    sub_2c829(v14[v13]);
  }
  if (!dat_9f864) {
    dat_9f864 = (v36) ? 4 : 6; // branch-flip
  }
  if ((((dat_9f9c0) && (dat_9f860 == 3)) || (dat_9f8be)) || ((dat_9f8fc || (dat_9f979))))
    sub_34dad(0x54);
  if (dat_9f8d0) {
    if (sub_34a5e() != '\x01') {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("--occurrence is meaningless without a file list"));
      sub_3814c(2); // no-return
    }
    if (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 0x10)) {
      if (sub_3534c(1)) {
        v6 = sub_34e1c(dat_9f860);
        sub_351b6("--occurrence",v6); // no-return
      }
      dat_9f8d0 = 0;
    }
  }
  v4 = dat_9fbd8;
  if (!dat_9fbe0) {
    dat_9fbe0 = 1;
    *v4 = (int8)getenv("TAPE");
    if (!*dat_9fbd8)
      *dat_9fbd8 = 0x867e7;
  }
  if ((2 <= dat_9fbe0) && (dat_9f8fc != '\x01')) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Multiple archive files require \'-M\' option"));
    sub_3814c(2); // no-return
  }
  if ((dat_9f8e0) && (0 <= dat_9f908)) {
    v23 = (int4 *)sub_35327(2);
    v24 = sub_35327(3);
    v5 = sub_35388(v23,v24);
    if (v5)
      sub_351b6("--listed-incremental","--newer"); // no-return
    if (*v23 != 1) // branch-flip
      memset((void *)0x9f900,0,0x10);
    else {
      dat_9f8e0 = 0;
    }
  }
  if ((dat_9f8e8 != -1) && (!dat_9f8e0)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--level is meaningless without --listed-incremental"));
  }
  if ((dat_9f9c0) && ((dat_9f864 == 6 || (dat_9f864 == 2)))) {
    v25 = (dat_9f8fc) ? 0x51 : 99; // branch-flip
    v7 = strlen(dat_9f9c0);
    if (v25 < v7) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v6 = sub_5e548(dat_9f9c0);
      v8 = (char *)ngettext("%s: Volume label is too long (limit is %lu byte)","%s: Volume label is too long (limit is %lu bytes)",v25);
      error(0,0,v8,v6,v25);
      sub_3814c(2); // no-return
    }
  }
  if (dat_9f9b0) {
    if (dat_9f8fc) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Cannot verify multi-volume archives"));
      sub_3814c(2); // no-return
    }
    if (dat_9f898) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Cannot verify compressed archives"));
      sub_3814c(2); // no-return
    }
    if (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 2)) {
      if (sub_3534c(4)) {
        v6 = sub_34e1c(dat_9f860);
        sub_351b6("--verify",v6); // no-return
      }
      dat_9f9b0 = '\0';
    }
  }
  if (dat_9f898) {
    if (dat_9f8fc) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Cannot use multi-volume compressed archives"));
      sub_3814c(2); // no-return
    }
    if (*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 4) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Cannot update compressed archives"));
      sub_3814c(2); // no-return
    }
    if (dat_9f860 == 2) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Cannot concatenate compressed archives"));
      sub_3814c(2); // no-return
    }
  }
  if ((dat_9f910 == 2) && (dat_9f928 <= -1)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--clamp-mtime needs a date specified using --mtime"));
    sub_3814c(2); // no-return
  }
  if (((v36) && (dat_9f864 != 4)) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--pax-option can be used only on POSIX archives"));
    sub_3814c(2); // no-return
  }
  if (((1 <= dat_9f968) && (dat_9f864 != 4)) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--acls can be used only on POSIX archives"));
    sub_3814c(2); // no-return
  }
  if (((1 <= dat_9f964) && (dat_9f864 != 4)) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--selinux can be used only on POSIX archives"));
    sub_3814c(2); // no-return
  }
  if (((1 <= dat_9f96c) && (dat_9f864 != 4)) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("--xattrs can be used only on POSIX archives"));
    sub_3814c(2); // no-return
  }
  if ((dat_9f988) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (sub_3534c(5)) {
      v6 = sub_34e1c(dat_9f860);
      sub_351b6("--starting-file",v6); // no-return
    }
    dat_9f988 = '\0';
  }
  if ((dat_9f958) && (!(*(uint4 *)((uint8)dat_9f860 * 4 + 0x9cb60) & 1))) {
    if (sub_3534c(6)) {
      v6 = sub_34e1c(dat_9f860);
      sub_351b6("--same-order",v6); // no-return
    }
    dat_9f958 = '\0';
  }
  if (dat_9f936) {
    if (dat_9f878) {
      v26 = (int4 *)sub_35327(7);
      v27 = sub_35327(8);
      v5 = sub_35388(v26,v27);
      if (v5)
        sub_351b6("--one-top-level","--absolute-names"); // no-return
      if (*v26 != 1) // branch-flip
        dat_9f936 = '\0';
      else {
        dat_9f878 = '\0';
      }
    }
    if ((dat_9f936) && (!dat_9f938)) {
      v28 = (void *)sub_4e13e(*dat_9fbd8);
      dat_9f938 = sub_329b0(v28);
      free(v28);
      if (!dat_9f938) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR"));
        sub_3814c(2); // no-return
      }
    }
  }
  if (dat_9f94c)
    dat_9f8d8 = 3;
  v9 = (dat_9f8a0) ? 0x80900 : 0xa0900; // branch-flip
  dat_9fc08 = (dat_9f880 != 2) ? 0 : 0x40000; // branch-flip
  dat_9fc08 |= v9;
  dat_9fc0c = dat_9fc08 | 0x10000;
  dat_9fc10 = (dat_9f8a0) ? 0 : 0x100; // branch-flip
  v17 = dat_9fc08;
  if (dat_9f860 != 9) { // branch-flip
    if (dat_9f879)
      dat_9f9ac = 2;
  }
  else if (sub_34a5e() != '\x01')
    dat_9f9ac += 1;
  if ((dat_9f990 != 0.0) && (dat_9f990 < (float8)dat_9f870)) {
    if (dat_9fdc8)
      (*dat_9fdc8)();
    error(0,0,gettext("Volume length cannot be less than record size"));
    sub_3814c(2); // no-return
  }
  if ((dat_9f958) && (dat_9f8e0)) {
    v29 = (int4 *)sub_35327(6);
    v30 = sub_35327(2);
    v5 = sub_35388(v29,v30);
    if (v5)
      sub_351b6("--preserve-order","--listed-incremental"); // no-return
    if (*v29 != 1) // branch-flip
      dat_9f958 = '\0';
    else {
      dat_9f8e0 = 0;
    }
  }
  switch(dat_9f860) {
    case 1:
    case 2:
    case 8:
      for (dat_9fbe8 = dat_9fbd8; dat_9fbe8 < &dat_9fbd8[dat_9fbe0]; dat_9fbe8 = &dat_9fbe8[1]) {
        v8 = (char *)*dat_9fbe8;
        if (!strcmp(v8,"-")) {
          if (dat_9fdc8)
            (*dat_9fdc8)();
          error(0,0,gettext("Options \'-Aru\' are incompatible with \'-f -\'"));
          sub_3814c(2); // no-return
        }
      }
      goto label_39837;
    default:
      break;
    case 3:
      if (sub_34a5e() != '\x01') {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("Cowardly refusing to create an empty archive"));
        sub_3814c(2); // no-return
      }
      if (((v37) && (dat_9fbe0)) && (v8 = (char *)*dat_9fbd8, strcmp(v8,"-")))
        sub_32970(*dat_9fbd8,dat_9f898);
      break;
    case 5:
    case 6:
    case 7:
    case 9:
      for (dat_9fbe8 = dat_9fbd8; dat_9fbe8 < &dat_9fbd8[dat_9fbe0]; dat_9fbe8 = &dat_9fbe8[1]) {
        v8 = (char *)*dat_9fbe8;
        if (!strcmp(v8,"-"))
          sub_34a6f(0x8746d);
      }
    
  }
  if (dat_9fbf0) { // branch-flip
    dat_9efd0 = fopen(dat_9fbf0,"w");
    if (!dat_9efd0)
      sub_3e3b4(dat_9fbf0); // no-return
  }
  else if (dat_9f998) // branch-flip
    dat_9efd0 = stderr;
  else {
    dat_9efd0 = stdout;
  }
  dat_9fbe8 = dat_9fbd8;
  if (v38)
    dat_9fe30 = sub_60cc6(v38);
  if ((dat_9f884) && (((dat_9f888 = sub_4dec2("--backup",v39), !dat_9f888 || (dat_9f998)) || (dat_9f9a0))))
    dat_9f884 = '\0';
  sub_fb72();
  sub_3589e(&v12);
  return;
}


// Function: sub_39948 @ 0x39948
int4 sub_39948(unsigned int a0,unsigned long *a1)
{
  int4 v1; // eax
  
  sub_adc5();
  sub_5c844(*a1);
  setlocale(6,"");
  bindtextdomain("tar","/usr/local/share/locale");
  textdomain("tar");
  dat_9ee88 = 2;
  dat_9fdd0 = 0;
  dat_9fdc8 = sub_10622;
  sub_5c9a2(0,7);
  sub_4dfe0(gettext("stdout"));
  if (!sub_5fe52()) {
    dat_9fc38 = 10;
    dat_9fbd8 = (void *)sub_60b5b(0x50);
    dat_9fbe0 = 0;
    signal(0x11,0);
    sub_5c826();
    sub_3854d(a0,a1);
    sub_2c94c();
    if (dat_9f9b8)
      sub_d1c6();
    switch(dat_9f860) {
      case 0:
        if (dat_9fdc8)
          (*dat_9fdc8)();
        error(0,0,gettext("You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options"));
        sub_3814c(2); // no-return
      case 1:
      case 2:
      case 8:
        sub_3c325();
        break;
      case 3:
        sub_14e23();
        break;
      case 4:
        sub_17069();
        break;
      case 5:
        sub_1075e();
        sub_257b8(sub_11897);
        break;
      case 6:
        sub_182fd();
        sub_257b8(sub_1bbcb);
        sub_1c2ee();
        break;
      case 7:
        sub_257b8(sub_25cf3);
        break;
      case 9:
        sub_28913();
      
    }
    sub_10729();
    if (dat_9f999)
      sub_baa2();
    if (dat_9fc34)
      sub_1573f();
    if (dat_9f9b8)
      sub_d2e3();
    free(dat_9fbd8);
    sub_3d17c();
    sub_2c978();
    if (dat_9fdd0 == 2)
      error(0,0,gettext("Exiting with failure status due to previous errors"));
    if (dat_9efd0 != stdout) { // branch-flip
      v1 = ferror_unlocked(stderr);
      if ((v1) || (v1 = fclose(stderr), v1))
        sub_39e98(2);
    }
    else {
      sub_4e014();
    }
    return dat_9fdd0;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,"%s",gettext("failed to assert availability of the standard file descriptors"));
  sub_177f4(); // no-return
}


// Function: sub_39c55 @ 0x39c55
void sub_39c55(void *a0)
{
  memset(a0,0,0x1b8);
  return;
}


// Function: sub_39c7e @ 0x39c7e
bool sub_39c7e(unsigned long *a0)
{
  int4 v1; // eax
  
  if (a0[0x34]) // branch-flip
    v1 = closedir((void *)a0[0x34]);
  else if (1 <= *(int4 *)&a0[0x35]) // branch-flip
    v1 = close(*(int4 *)&a0[0x35]);
  else {
    v1 = 0;
  }
  a0[0x34] = 0;
  *(unsigned int *)&a0[0x35] = 0;
  if (v1)
    sub_2b26a(*a0);
  return v1 == 0;
}


// Function: sub_39d1c @ 0x39d1c
void sub_39d1c(unsigned long *a0)
{
  sub_39c7e(a0);
  sub_1d55d(a0[0x2c],a0[0x2b]);
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[3]);
  free((void *)a0[4]);
  free((void *)a0[5]);
  free((void *)a0[6]);
  free((void *)a0[7]);
  free((void *)a0[9]);
  free((void *)a0[0x28]);
  free((void *)a0[0x32]);
  sub_1e785(&a0[0x2d]);
  sub_17bda(a0);
  memset(a0,0,0x1b8);
  return;
}


// Function: sub_39e35 @ 0x39e35
void sub_39e35(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v1 = a3;
  v2 = a1;
  if (dat_9f468 != 4) {
    v1 = 0;
    v2 = 0;
  }
  sub_5ff90(a0,v2,a2,v1);
  return;
}


// Function: sub_39e98 @ 0x39e98
void sub_39e98(int4 a0) // return-dupe
{
  if (a0 <= dat_9fdd0)
    return;
  dat_9fdd0 = a0;
  return;
}


// Function: sub_39eba @ 0x39eba
unsigned long sub_39eba(void)
{
  unsigned long v1; // rax
  
  v1 = sub_60c06(0x70);
  if (dat_9fcc8) // branch-flip
    *dat_9fcc8 = v1;
  else {
    dat_9fcc0 = v1;
  }
  dat_9fcc8 = (unsigned long *)v1;
  return v1;
}


// Function: sub_39f0c @ 0x39f0c
unsigned long * sub_39f0c(int8 a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_60b5b(0x20);
  *v1 = 0;
  if (*(int8 *)(a0 + 0x60)) // branch-flip
    **(unsigned long **)(a0 + 0x60) = v1;
  else {
    *(unsigned long **)(a0 + 0x58) = v1;
  }
  *(unsigned long **)(a0 + 0x60) = v1;
  *(int8 *)(a0 + 0x68) = *(int8 *)(a0 + 0x68) + 1;
  return v1;
}


// Function: sub_39f85 @ 0x39f85
void sub_39f85(unsigned long a0,void *a1,int8 a2) // return-dupe
{
  uint8 v1; // rax
  int8 v2; // rax
  
  v1 = a2 - (int8)a1;
  if (!v1)
    return;
  v2 = sub_39f0c(a0);
  *(unsigned int *)(v2 + 8) = 0;
  *(unsigned long *)(v2 + 0x10) = sub_60b5b(v1 + 1);
  memcpy(*(void **)(v2 + 0x10),a1,v1);
  *(char *)(v1 + *(int8 *)(v2 + 0x10)) = 0;
  *(uint8 *)(v2 + 0x18) = v1;
  return;
}


// Function: sub_3a022 @ 0x3a022
void sub_3a022(unsigned long a0,char a1)
{
  int8 v1; // rax
  
  v1 = sub_39f0c(a0);
  *(unsigned int *)(v1 + 8) = 0;
  *(unsigned long *)(v1 + 0x10) = sub_60b5b(2);
  **(char **)(v1 + 0x10) = a1;
  *(char *)(*(int8 *)(v1 + 0x10) + 1) = 0;
  *(unsigned long *)(v1 + 0x18) = 1;
  return;
}


// Function: sub_3a090 @ 0x3a090
void sub_3a090(unsigned long a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_39f0c(a0);
  *(unsigned int *)(v1 + 8) = 1;
  *(unsigned long *)(v1 + 0x10) = a1;
  return;
}


// Function: sub_3a0ce @ 0x3a0ce
unsigned long sub_3a0ce(uint4 *a0,unsigned int a1) // return-dupe, return-dupe x5
{
  switch(a1) {
    case 0x48:
      *a0 = *a0 & 0xfffffffd;
      return 0;
    default:
      return 1;
    case 0x52:
      *a0 = *a0 & 0xfffffffe;
      return 0;
    case 0x53:
      *a0 = *a0 & 0xfffffffb;
      return 0;
    case 0x68:
      *a0 = *a0 | 2;
      return 0;
    case 0x72:
      *a0 = *a0 | 1;
      return 0;
    case 0x73:
      *a0 = *a0 | 4;
      return 0;
    
  }
}


// Function: sub_3a18f @ 0x3a18f
void sub_3a18f(unsigned long a0,unsigned int a1)
{
  int8 v1; // rax
  
  v1 = sub_39f0c(a0);
  *(unsigned int *)(v1 + 8) = 2;
  *(unsigned int *)(v1 + 0x10) = a1;
  return;
}


// Function: sub_3a1ca @ 0x3a1ca
char * sub_3a1ca(char *a0)
{
  char v1;
  char *v10; // stack - 0x280
  int4 v11; // stack - 0x26c
  int4 v12; // stack - 0x268
  uint4 v13; // stack - 0x264
  char *v14; // stack - 0x248
  int8 v15; // stack - 0x240
  char *v16; // stack - 0x238
  uint8 v17; // stack - 0x230
  char v18; // stack - 0x227
  int4 v2; // eax
  int4 v3;
  uint8 v4; // rax
  int8 v5; // rdx
  char *v6; // stack - 0x250
  char v7; // stack - 0x228
  char *v8; // stack - 0x258
  char v9 [518];
  
  v13 = 0;
  v15 = sub_39eba();
  if (*a0 != 's') { // branch-flip
    if (strncmp(a0,"flags=",6)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Invalid transform expression"));
      sub_3814c(2); // no-return
    }
    dat_9ec00 = 0;
    for (v10 = &a0[6]; *v10; v10 = &v10[1]) {
      if (*v10 == ';') {
        v10 = &v10[1];
        break;
      }
      v1 = *v10;
      if (sub_3a0ce(0x9ec00,(int4)v1)) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v1 = *v10;
        error(0,0,gettext("Unknown transform flag: %c"),(uint8)(uint4)(int4)v1);
        sub_3814c(2); // no-return
      }
    }
  }
  else {
    v2 = (int4)a0[1];
    if (!v2) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Invalid transform expression"));
      sub_3814c(2); // no-return
    }
    v11 = 2;
    while ((a0[v11] && (v2 != a0[v11]))) {
      if ((a0[v11] == '\\') && (a0[(int8)v11 + 1]))
        v11 += 1;
      v11 += 1;
    }
    v12 = v11;
    if (v2 != a0[v11]) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Invalid transform expression"));
      sub_3814c(2); // no-return
    }
    while( true ) {
      v3 = v12;
      v12 = v3 + 1;
      if ((!a0[v12]) || (v2 == a0[v12])) break;
      if ((a0[v12] == '\\') && (a0[(int8)v12 + 1]))
        v12 = v3 + 2;
    }
    if (v2 != a0[v12]) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Invalid transform expression"));
      sub_3814c(2); // no-return
    }
    *(unsigned int *)(v15 + 8) = 0;
    *(unsigned int *)(v15 + 0xc) = dat_9ec00;
    for (v6 = &a0[(int8)v12 + 1]; (*v6 && (*v6 != ';')); v6 = &v6[1]) {
      v1 = *v6;
      if (v1 != 'x') { // branch-flip
        if ('y' <= v1) { // branch-flip
label_3a67e:
          v1 = *v6;
          v5 = v15 + 0xc;
          if (sub_3a0ce(v5,(int4)v1)) {
            if (dat_9fdc8)
              (*dat_9fdc8)();
            v1 = *v6;
            error(0,0,gettext("Unknown flag in transform expression: %c"),(uint8)(uint4)(int4)v1);
            sub_3814c(2); // no-return
          }
        }
        else if (v1 != 'i') { // branch-flip
          if ('i' < v1) goto label_3a67e;
          if (':' <= v1) { // branch-flip
            if (v1 != 'g') goto label_3a67e;
            *(unsigned int *)(v15 + 8) = 1;
          }
          else {
            if (v1 <= '/') goto label_3a67e;
            v4 = strtoul(v6,&v6,0);
            *(int4 *)(v15 + 0x10) = (int4)v4;
            v6 = &v6[-1];
          }
        }
        else {
          v13 |= 2;
        }
      }
      else {
        v13 |= 1;
      }
    }
    if (*v6 == ';')
      v6 = &v6[1];
    v16 = (char *)sub_60b5b((int8)(v11 + -1));
    memcpy(v16,&a0[2],(int8)(v11 + -2));
    v16[(int8)v11 + -2] = '\0';
    v3 = sub_67ccd(v15 + 0x18,v16,v13);
    if (v3) {
      sub_67e43(v3,v15 + 0x18,&v7,0x200);
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("Invalid transform expression: %s"),&v7);
      sub_3814c(2); // no-return
    }
    if ((*v16 == '^') || ((3 <= v11 && (v16[(int8)v11 + -3] == '$'))))
      *(unsigned int *)(v15 + 8) = 0;
    free(v16);
    v11 += 1;
    v16 = (char *)sub_60b5b((int8)((v12 - v11) + 1));
    memcpy(v16,&a0[v11],(int8)(v12 - v11));
    v16[v12 - v11] = '\0';
    v14 = v16;
    v8 = v16;
    while (*v8) {
      if (*v8 != '\\') { // branch-flip
        if (*v8 != '&') // branch-flip
          v8 = &v8[1];
        else {
          sub_39f85(v15,v14,v8);
          sub_3a090(v15,0);
          v8 = &v8[1];
          v14 = v8;
        }
      }
      else {
        sub_39f85(v15,v14,v8);
        v8 = &v8[1];
        switch(*v8) {
          case 0x26:
            sub_3a022(v15,0x26);
            v8 = &v8[1];
            break;
          default:
            if (v2 != *v8) { // branch-flip
              v7 = 0x5c;
              v18 = *v8;
              sub_39f85(v15,&v7,v9);
            }
            else {
              sub_3a022(v15,v2);
            }
            v8 = &v8[1];
            break;
          case 0x30:
          case 0x31:
          case 0x32:
          case 0x33:
          case 0x34:
          case 0x35:
          case 0x36:
          case 0x37:
          case 0x38:
          case 0x39:
            v17 = strtoul(v8,&v8,10);
            if (*(uint8 *)(v15 + 0x48) < v17) {
              if (dat_9fdc8)
                (*dat_9fdc8)();
              error(0,0,gettext("Invalid transform replacement: back reference out of range"));
              sub_3814c(2); // no-return
            }
            sub_3a090(v15,v17);
            break;
          case 0x45:
            sub_3a18f(v15,0);
            v8 = &v8[1];
            break;
          case 0x4c:
            sub_3a18f(v15,4);
            v8 = &v8[1];
            break;
          case 0x55:
            sub_3a18f(v15,3);
            v8 = &v8[1];
            break;
          case 0x5c:
            sub_3a022(v15,0x5c);
            v8 = &v8[1];
            break;
          case 0x61:
            sub_3a022(v15,7);
            v8 = &v8[1];
            break;
          case 0x62:
            sub_3a022(v15,8);
            v8 = &v8[1];
            break;
          case 0x66:
            sub_3a022(v15,0xc);
            v8 = &v8[1];
            break;
          case 0x6c:
            sub_3a18f(v15,2);
            v8 = &v8[1];
            break;
          case 0x6e:
            sub_3a022(v15,10);
            v8 = &v8[1];
            break;
          case 0x72:
            sub_3a022(v15,0xd);
            v8 = &v8[1];
            break;
          case 0x74:
            sub_3a022(v15,9);
            v8 = &v8[1];
            break;
          case 0x75:
            sub_3a18f(v15,1);
            v8 = &v8[1];
            break;
          case 0x76:
            sub_3a022(v15,0xb);
            v8 = &v8[1];
          
        }
        v14 = v8;
      }
    }
    sub_39f85(v15,v14,v8);
    free(v16);
    v10 = v6;
  }
  return v10;
}


// Function: sub_3ae0b @ 0x3ae0b
void sub_3ae0b(char *a0)
{
  char *v1; // stack - 0x10
  
  for (v1 = a0; *v1; v1 = (char *)sub_3a1ca(v1)) {
  }
  return;
}


// Function: sub_3ae3c @ 0x3ae3c
uint1 * sub_3ae3c(unsigned int a0,void *a1,uint8 a2)
{
  uint1 v1;
  uint1 *v2; // stack - 0x10
  
  if (dat_9fd40 < a2) {
    dat_9fd40 = a2;
    dat_9fcd0 = (uint1 *)sub_60b8d(dat_9fcd0,a2);
  }
  memcpy(dat_9fcd0,a1,a2);
  switch(a0) {
    case 1:
      v1 = *dat_9fcd0;
      *dat_9fcd0 = (uint1)toupper((uint4)v1);
      break;
    case 2:
      v1 = *dat_9fcd0;
      *dat_9fcd0 = (uint1)tolower((uint4)v1);
      break;
    case 3:
      for (v2 = dat_9fcd0; v2 < &dat_9fcd0[a2]; v2 = &v2[1]) {
        v1 = *v2;
        *v2 = (uint1)toupper((uint4)v1);
      }
      break;
    case 4:
      for (v2 = dat_9fcd0; v2 < &dat_9fcd0[a2]; v2 = &v2[1]) {
        v1 = *v2;
        *v2 = (uint1)tolower((uint4)v1);
      }
    
  }
  return dat_9fcd0;
}


// Function: sub_3afa7 @ 0x3afa7
void sub_3afa7(int8 a0,char *a1)
{
  char *v1;
  uint8 v10; // stack - 0xd0
  char *v11; // stack - 0xc8
  unsigned long *v12; // stack - 0xc0
  uint4 v2;
  int8 v3;
  uint8 *v4; // rax
  uint8 v5;
  char *v6; // stack - 0xf8
  int4 v7; // stack - 0xe4
  int4 v8; // stack - 0xe0
  uint8 v9; // stack - 0xd8
  
  v9 = 0;
  v7 = 0;
  v8 = 0;
  v4 = (uint8 *)sub_60b5b((*(int8 *)(a0 + 0x48) + 1) * 0x10);
  v6 = a1;
label_3b6ba:
  do {
    if (!*v6) goto label_3b6cc;
    v3 = *(int8 *)(a0 + 0x48);
    if (sub_6f294(a0 + 0x18,v6,v3 + 1,v4,0)) { // branch-flip
      v10 = strlen(v6);
      if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v10)
        _obstack_newchunk(0x9fce0,v10);
      memcpy(dat_9fcf8,v6,v10);
      dat_9fcf8 = &dat_9fcf8[v10];
    }
    else {
      v10 = v4[1];
      if (*v4) {
        v5 = *v4;
        if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v5)
          _obstack_newchunk(0x9fce0,v5);
        memcpy(dat_9fcf8,v6,v5);
        dat_9fcf8 = &dat_9fcf8[v5];
      }
      v9 += 1;
      if ((*(int4 *)(a0 + 0x10)) && (v9 < *(uint4 *)(a0 + 0x10))) {
        if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v10)
          _obstack_newchunk(0x9fce0,v10);
        memcpy(dat_9fcf8,v6,v10);
        dat_9fcf8 = &dat_9fcf8[v10];
        v6 = &v6[v10];
        goto label_3b6ba;
      }
      for (v12 = *(unsigned long **)(a0 + 0x58); v12; v12 = (unsigned long *)*v12) {
        v2 = *(uint4 *)&v12[1];
        if (v2 != 2) { // branch-flip
          if (v2 <= 2) {
            if (v2) { // branch-flip
              if (((v2 == 1) && (v4[v12[2] * 2] != 0xffffffffffffffff)) && (v4[v12[2] * 2 + 1] != 0xffffffffffffffff)) {
                v5 = v4[v12[2] * 2 + 1] - v4[v12[2] * 2];
                v11 = &v6[v4[v12[2] * 2]];
                if (v7) {
                  v11 = (char *)sub_3ae3c(v7,v11,v5);
                  if ((v7 == 1) || (v7 == 2)) {
                    v7 = v8;
                    v8 = 0;
                  }
                }
                if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v5)
                  _obstack_newchunk(0x9fce0,v5);
                memcpy(dat_9fcf8,v11,v5);
                dat_9fcf8 = &dat_9fcf8[v5];
              }
            }
            else {
              if (v7) { // branch-flip
                v11 = (void *)sub_3ae3c(v7,v12[2],v12[3]);
                if ((v7 == 1) || (v7 == 2)) {
                  v7 = v8;
                  v8 = 0;
                }
              }
              else {
                v11 = (void *)v12[2];
              }
              v5 = v12[3];
              if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v5)
                _obstack_newchunk(0x9fce0,v5);
              memcpy(dat_9fcf8,v11,v5);
              dat_9fcf8 = &dat_9fcf8[v5];
            }
          }
        }
        else {
          v2 = *(uint4 *)&v12[2];
          if (v2 <= 4) {
            if ((v2 <= 2) && (v2)) {
              if (2 <= v2 - 1) goto label_3b528;
              if ((!v8) || ((uint4)(v8 - 3U) <= 1))
                v8 = v7;
            }
            v7 = *(int4 *)&v12[2];
          }
        }
label_3b528:
      }
    }
    v6 = &v6[v10];
    if (!*(int4 *)(a0 + 8)) {
      v5 = strlen(v6);
      if ((uint8)((int8)dat_9fd00 - (int8)dat_9fcf8) < v5)
        _obstack_newchunk(0x9fce0,v5);
      memcpy(dat_9fcf8,v6,v5);
      dat_9fcf8 = &dat_9fcf8[v5];
label_3b6cc:
      if (dat_9fd00 == dat_9fcf8)
        _obstack_newchunk(0x9fce0,1);
      v1 = &dat_9fcf8[1];
      *dat_9fcf8 = 0;
      dat_9fcf8 = v1;
      free(v4);
      return;
    }
  } while( true );
}


// Function: sub_3b734 @ 0x3b734
char sub_3b734(uint4 a0,uint8 a1,uint8 *a2)
{
  uint8 v1; // stack - 0x38
  char v2; // stack - 0x21
  unsigned long *v3; // stack - 0x20
  
  v2 = 0;
  if (dat_9fd38 != '\x01') {
    _obstack_begin(0x9fce0,0,0,sub_60b5b,dat_9bfc0);
    dat_9fd38 = '\x01';
  }
  v1 = a1;
  for (v3 = dat_9fcc0; v3; v3 = (unsigned long *)*v3) {
    if (*(uint4 *)((int8)v3 + 0xc) & a0) {
      sub_3afa7(v3,v1);
      v1 = dat_9fcf0;
      if (dat_9fcf0 == dat_9fcf8)
        dat_9fd30 |= 2;
      dat_9fcf8 = ~dat_9fd10 & dat_9fcf8 + dat_9fd10;
      if (dat_9fd00 - dat_9fce8 < dat_9fcf8 - dat_9fce8)
        dat_9fcf8 = dat_9fd00;
      dat_9fcf0 = dat_9fcf8;
      v2 = 1;
    }
  }
  *a2 = v1;
  return v2;
}


// Function: sub_3b8af @ 0x3b8af
char sub_3b8af(unsigned long *a0,unsigned int a1,code *a2,unsigned long a3)
{
  void *v1;
  unsigned long v2; // rax
  void *v3; // stack - 0x28
  char v4; // stack - 0x29
  unsigned long v5; // stack - 0x20
  void *v6; // stack - 0x18
  
  v4 = sub_3b734(a1,*a0,&v3);
  if (v4) { // branch-flip
    if (a2) // branch-flip
      v1 = (void *)(*a2)(v3,a3);
    else {
      v1 = v3;
    }
    sub_29547(a0,v1);
    v5 = 0x9fce0;
    v6 = v3;
    if ((v3 <= dat_9fce8) || (dat_9fd00 <= v3))
      _obstack_free(0x9fce0,v3);
    else {
      dat_9fcf0 = v3;
      dat_9fcf8 = v3;
    }
  }
  else if (a2) {
    *a0 = 0;
    v2 = (*a2)(v3,a3);
    sub_29547(a0,v2);
    free(v3);
    v4 = '\x01';
  }
  return v4;
}


// Function: sub_3b9f2 @ 0x3b9f2
void sub_3b9f2(unsigned long a0,unsigned int a1)
{
  sub_3b8af(a0,a1,0,0);
  return;
}


// Function: sub_3ba20 @ 0x3ba20
unsigned long sub_3ba20(void)
{
  return CONCAT71((undefined7)((uint8)dat_9fcc0 >> 8),dat_9fcc0 != 0);
}


// Function: sub_3ba37 @ 0x3ba37
unsigned long * sub_3ba37(void)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x10
  
  if (dat_9fd60) { // branch-flip
    v2 = dat_9fd60;
    v1 = (unsigned long *)*dat_9fd60;
    *dat_9fd60 = 0;
    dat_9fd60 = v1;
  }
  else {
    v2 = (unsigned long *)sub_60b5b(0x28);
  }
  return v2;
}


// Function: sub_3ba89 @ 0x3ba89
void sub_3ba89(int8 *a0,int8 *a1)
{
  if (a0) { // branch-flip
    *a1 = *a0;
    *a0 = (int8)a1;
  }
  else {
    *a1 = (int8)dat_9fd48;
    dat_9fd48 = a1;
  }
  if (!*a1)
    dat_9fd50 = a1;
  dat_9fd58 += 1;
  return;
}


// Function: sub_3bb00 @ 0x3bb00
void sub_3bb00(unsigned long *a0)
{
  free((void *)a0[2]);
  *a0 = dat_9fd60;
  dat_9fd60 = a0;
  return;
}


// Function: sub_3bb3c @ 0x3bb3c
void sub_3bb3c(char a0)
{
  unsigned long *v1;
  char *v2;
  unsigned int v3;
  int4 v4;
  unsigned long *v5; // stack - 0x38
  unsigned long *v6; // stack - 0x30
  char *v7; // stack - 0x28
  
  v3 = dat_9f600;
  v6 = NULL;
  v5 = dat_9fd48;
label_3bd19:
  if (v5) {
    v1 = (unsigned long *)*v5;
    if ((a0) || ((uint8)(v5[4] + dat_9fd68) < dat_9efc8)) {
      sub_2adf1(*(unsigned int *)&v5[1]);
      if (*(char *)&v5[3]) { // branch-flip
        if ((*(int4 *)&v5[1]) && (*(char *)&v5[3])) {
          if (*(char *)v5[2]) {
            v2 = (char *)v5[2];
            if (strcmp(v2,".")) goto label_3bc1b;
          }
          v6 = v5;
          v5 = v1;
          goto label_3bd19;
        }
label_3bc1b:
        v2 = (char *)v5[2];
        v4 = unlinkat(dat_9c268,v2,0x200);
        if (v4) {
          v4 = *__errno_location();
          if (v4 == 0x27) {
label_3bc5f:
            v6 = v5;
            v5 = v1;
            goto label_3bd19;
          }
          if (v4 <= 0x27) {
            if (v4 == 2) goto label_3bcc2;
            if (v4 == 0x11) goto label_3bc5f;
          }
          sub_3e705(v2);
        }
      }
      else {
        v4 = unlinkat(dat_9c268,(char *)v5[2],0);
        if (v4) {
          if (*__errno_location() != 2)
            sub_3eaa7(v5[2]);
        }
      }
label_3bcc2:
      sub_3bb00(v5);
      dat_9fd58 -= 1;
      if (v6) { // branch-flip
        *v6 = v1;
        v5 = v1;
      }
      else {
        dat_9fd48 = v1;
        v5 = v1;
      }
    }
    else {
      v6 = v5;
      v5 = v1;
    }
    goto label_3bd19;
  }
  if (dat_9fd48) { // branch-flip
    if (a0) {
      v5 = dat_9fd48;
      while (v5) {
        v1 = (unsigned long *)*v5;
        sub_2adf1(*(unsigned int *)&v5[1]);
        if ((*(int4 *)&v5[1]) && (*(char *)&v5[3])) {
          if (*(char *)v5[2]) {
            v2 = (char *)v5[2];
            if (strcmp(v2,".")) goto label_3bdd3;
          }
          v7 = (char *)sub_2afc0();
          sub_2adf1(*(int4 *)&v5[1] + -1);
        }
        else {
label_3bdd3:
          v7 = (char *)v5[2];
        }
        v4 = unlinkat(dat_9c268,v7,0x200);
        if (v4) {
          if (*__errno_location() != 2)
            sub_3e705(v7);
        }
        sub_3bb00(v5);
        dat_9fd58 -= 1;
        v5 = v1;
      }
      dat_9fd50 = 0;
      dat_9fd48 = NULL;
    }
  }
  else {
    dat_9fd50 = 0;
  }
  sub_2adf1(v3);
  return;
}


// Function: sub_3be6b @ 0x3be6b
void sub_3be6b(void)
{
  unsigned long *v1;
  
  sub_3bb3c(1);
  while (dat_9fd60) {
    v1 = (unsigned long *)*dat_9fd60;
    free(dat_9fd60);
    dat_9fd60 = v1;
  }
  return;
}


// Function: sub_3bebb @ 0x3bebb
void sub_3bebb(unsigned long a0,char a1) // return-dupe x2
{
  char *v1;
  unsigned long *v2; // rax
  unsigned long *v3; // stack - 0x20
  unsigned long *v4; // stack - 0x18
  
  if ((dat_9fd48) && ((uint8)(dat_9fd48[4] + dat_9fd68) < dat_9efc8))
    sub_3bb3c(0);
  v2 = (unsigned long *)sub_3ba37();
  *v2 = 0;
  *(unsigned int *)&v2[1] = dat_9f600;
  v2[2] = sub_60cc6(a0);
  sub_29946(v2[2]);
  *(char *)&v2[3] = a1;
  v2[4] = dat_9efc8;
  if ((!*(char *)&v2[3]) || ((*(char *)v2[2] && (v1 = (char *)v2[2], strcmp(v1,"."))))) {
    sub_3ba89(dat_9fd50,v2);
    return;
  }
  v4 = NULL;
  for (v3 = dat_9fd48; (v3 && ((!*(char *)&v3[3] || (((*(char *)v3[2] && (v1 = (char *)v3[2], strcmp(v1,"."))) || (*(int4 *)&v2[1] <= *(int4 *)&v3[1])))))); v3 = (unsigned long *)*v3) {
    v4 = v3;
  }
  if (v3) {
    sub_3ba89(v4,v2);
    return;
  }
  sub_3ba89(dat_9fd50,v2);
  return;
}


// Function: sub_3c078 @ 0x3c078
void sub_3c078(char *a0) // return-dupe x2
{
  int4 v1; // eax
  uint8 v10; // stack - 0xe0
  uint8 v11; // stack - 0x98
  int8 v2; // rax
  uint8 v3; // rax
  int8 v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  char v7 [48];
  char v8 [24];
  uint8 v9; // stack - 0xe8
  
  v1 = openat(dat_9c268,a0,0);
  if (v1 <= -1) {
    sub_3e38b(a0);
    return;
  }
  if (fstat(v1,v7)) // branch-flip
    sub_3ea03(a0);
  else {
    v9 = v11;
    while (0 < (int8)v9) {
      v2 = sub_bb4b();
      v10 = sub_bbf5(v2);
      if (v9 < v10) {
        v10 = v9;
        v3 = (uint8)((uint4)v9 & 0x1ff);
        if (v3)
          memset((void *)(v2 + v9),0,0x200 - v3);
      }
      v4 = sub_5ec70(v1,v2,v10);
      if (v4 == -1)
        sub_3e5fc(a0,v11 - v9,v10); // no-return
      if (!v4) {
        if (dat_9fdc8)
          (*dat_9fdc8)();
        v5 = sub_54e5a(v9,v8);
        v6 = sub_5e548(a0);
        error(0,0,(char *)ngettext("%s: File shrank by %s byte","%s: File shrank by %s bytes",v9),v6,v5);
        sub_177f4(); // no-return
      }
      v9 -= v4;
      sub_bba7(v2 + (v4 - 1U & 0xfffffffffffffe00U));
    }
  }
  if (!close(v1))
    return;
  sub_3e1ff(a0);
  return;
}


// Function: sub_3c325 @ 0x3c325
void sub_3c325(void)
{
  unsigned int v1;
  unsigned int v10; // stack - 0xf0
  char *v11; // stack - 0xe8
  uint4 v12; // stack - 0xa0
  bool v2;
  unsigned int v3; // eax
  int4 v4; // eax
  char *v5; // rax
  unsigned long v6;
  int8 v7; // rax
  char v8 [24];
  char v9 [16];
  
  v10 = 0;
  v2 = 0;
  sub_2d3b1();
  sub_f694(2);
  sub_1d498();
  do {
    if (v2) {
      sub_baf9();
      dat_9fd70 = 1;
      dat_9fd78 = dat_9efb0;
      while (v7 = sub_2eda0(), v7) {
        v6 = *(unsigned long *)(v7 + 0x10);
        if ((!sub_17c43(v6,0)) && ((!dat_9f8c8 || (sub_34af4(0x879ce,v6))))) {
          if (dat_9f860 != 2) // branch-flip
            sub_16a25(0,v6,v6);
          else {
            sub_3c078(v6);
          }
        }
      }
      sub_12da0();
      sub_d046();
      sub_3be6b();
      sub_2dbba();
      return;
    }
    v3 = sub_25ecf(0x9f460,0x9fa20,0);
    switch(v3) {
      case 0:
      case 2:
        abort(); // no-return
      case 1:
        sub_26747(dat_9f460,0x9fa20,0x9f468,0);
        sub_25733((int4)*(char *)(dat_9f460 + 0x9c),0x9fa20);
        dat_9f864 = dat_9f468;
        if ((dat_9f860 == 8) && (v7 = sub_2ed0e(dat_9fa28), v7)) {
          sub_2adf1(*(unsigned int *)(v7 + 0x28));
          v4 = sub_2a9b8(dat_9fa28,v8);
          if (!v4) {
            if ((v12 & 0xf000) != 0x4000) { // branch-flip
              v9 = sub_5fd9e(v8);
              v4 = sub_39e35(SUB168(v9,0),SUB168(v9,8),dat_9fb18,dat_9fb20);
              if (v4 <= 0)
                sub_2d817(v7);
            }
            else {
              v5 = (char *)sub_2b8e1(*(unsigned long *)(v7 + 0x10),1);
              if (v5) {
                v6 = sub_2b66c(*(unsigned long *)(v7 + 0x10));
                for (v11 = v5; *v11; v11 = &v11[strlen(v11) + 1]) {
                  v1 = *(unsigned int *)(v7 + 0x28);
                  sub_2d5b5(sub_2b75b(v6,v11),v1,0,0);
                }
                sub_2b72d(v6);
                free(v5);
                sub_2d817(v7);
              }
            }
          }
        }
        sub_28891();
        break;
      case 3:
        dat_9efb0 = dat_9f460;
        v2 = 1;
        break;
      case 4:
        v2 = 1;
        break;
      case 5:
        sub_bba7(dat_9f460);
        switch(v10) {
          case 0:
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("This does not look like a tar archive"));
            goto label_3c673;
          case 1:
          case 3:
            if (dat_9fdc8)
              (*dat_9fdc8)();
            error(0,0,gettext("Skipping to next header"));
            dat_9fdd0 = 2;
            break;
          case 2:
          case 4:
            abort(); // no-return
          
        }
      
    }
    sub_39d1c(0x9fa20);
    v10 = v3;
  } while( true );
}


// Function: sub_3c7e3 @ 0x3c7e3
unsigned long sub_3c7e3(uint1 a0)
{
  unsigned long v1; // rax
  
  if (*(int8 *)((int8)(int4)(uint4)a0 * 8 + 0x9ec10) == -1) {
    if (a0) { // branch-flip
      v1 = sub_54ee3();
      *(unsigned long *)((int8)(int4)(uint4)a0 * 8 + 0x9ec10) = iconv_open("UTF-8",v1);
    }
    else {
      dat_9ec10 = iconv_open(sub_54ee3(),"UTF-8");
    }
  }
  return *(unsigned long *)((int8)(int4)(uint4)a0 * 8 + 0x9ec10);
}


// Function: sub_3c8a0 @ 0x3c8a0
unsigned long sub_3c8a0(char a0,char *a1,unsigned long *a2) // return-dupe x3
{
  int8 v1; // rax
  int8 v2; // stack - 0x28
  char *v3; // stack - 0x38
  int8 v4; // stack - 0x30
  char *v5; // stack - 0x40
  int8 v6; // stack - 0x20
  char *v7; // stack - 0x18
  
  v6 = sub_3c7e3(a0);
  if (!v6) {
    *a2 = sub_60cc6(a1);
    return 1;
  }
  if (v6 == -1)
    return 0;
  v4 = strlen(a1) + 1;
  v2 = v4 * 0x10 + 1;
  v7 = (char *)sub_60b5b(v2);
  v3 = v7;
  v5 = a1;
  v1 = iconv(v6,&v5,&v4,&v3,&v2);
  if (v1) {
    free(v7);
    return 0;
  }
  *v3 = 0;
  *a2 = v7;
  return 1;
}


// Function: sub_3c9b7 @ 0x3c9b7
unsigned long sub_3c9b7(char *a0)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (!*v1)
      return 1;
    if (*v1 <= '\xff') break;
    v1 = &v1[1];
  }
  return 0;
}


// Function: sub_3c9ee @ 0x3c9ee
void sub_3c9ee(char *a0) // return-dupe x2
{
  uint4 v1;
  bool v2;
  int8 v3; // rax
  char *v4; // stack - 0x20
  
  v2 = 0;
  if (!strcmp(a0,"none")) {
    dat_9ec84 = 0;
    return;
  }
  v4 = a0;
  if ((3 <= strlen(a0)) && (!memcmp(a0,(void *)0x87b57,3))) {
    v2 = 1;
    v4 = &a0[3];
  }
  v3 = sub_47c61("--warning",v4,0x9aa60,0x9ec20,4,dat_9ee48);
  v1 = *(uint4 *)(v3 * 4 + 0x9ec20);
  if (v2) {
    dat_9ec84 &= ~v1;
    return;
  }
  dat_9ec84 |= v1;
  return;
}


// Function: sub_3cae5 @ 0x3cae5
void sub_3cae5(char *a0,char a1,char *a2,uint8 a3)
{
  char *v1;
  char *v10; // stack - 0x50
  char *v11; // stack - 0x48
  uint8 v12; // stack - 0x38
  uint1 v13; // stack - 0x18
  int4 v2; // eax
  uint8 v3; // rax
  uint8 v4; // rax
  char v5 [8];
  char *v6; // stack - 0x110
  int4 v7; // stack - 0xec
  int8 v8; // stack - 0x60
  char *v9; // stack - 0x58
  
  v3 = strlen(a0);
  v7 = 0;
  if ((a2) && (a3)) {
    _obstack_begin(v5,0,0,sub_60b5b,dat_9bfc0);
    v6 = a2;
    for (; (uint8)(int8)v7 <= a3; v7 = v7 + v2 + 1) {
      v2 = (int4)strcspn(v6,",\n");
      if (!v2) break;
      if (a2 != v6) {
        if (v11 == v10)
          _obstack_newchunk(v5,1);
        v1 = &v10[1];
        *v10 = a1;
        v10 = v1;
      }
      v4 = (uint8)(int4)v3;
      if ((uint8)((int8)v11 - (int8)v10) < v4)
        _obstack_newchunk(v5,v4);
      memcpy(v10,a0,v4);
      v10 = &v10[v4];
      v4 = (uint8)v2;
      if ((uint8)((int8)v11 - (int8)v10) < v4)
        _obstack_newchunk(v5,v4);
      memcpy(v10,v6,v4);
      v10 = &v10[v4];
      v6 = &v6[(int8)v2 + 1];
    }
    if (v11 == v10)
      _obstack_newchunk(v5,1);
    v1 = v9;
    *v10 = 0;
    if (v9 == &v10[1])
      v13 |= 2;
    v10 = (char *)(~v12 & (uint8)&(&v10[1])[v12]);
    if ((uint8)((int8)v11 - v8) < (uint8)((int8)v10 - v8))
      v10 = v11;
    v9 = v10;
    fprintf(dat_9efd0,"%s",v1);
    _obstack_free(v5,0);
  }
  return;
}


// Function: sub_3cf89 @ 0x3cf89
void sub_3cf89(void) // return-dupe
{
  if (dat_9f968 <= 0)
    return;
  if (dat_9fdb0) {
    dat_9fdb0 = 1;
    return;
  }
  if (dat_9fdc8)
    (*dat_9fdc8)();
  error(0,0,gettext("POSIX ACL support is not available"));
  dat_9fdb0 = 1;
  return;
}


// Function: sub_3d003 @ 0x3d003
void sub_3d003(unsigned long a0,unsigned long a1,char a2)
{
  if ((1 <= dat_9f968) && (a2 != '2')) {
    if (!dat_9fdb4) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      error(0,0,gettext("POSIX ACL support is not available"));
    }
    dat_9fdb4 = 1;
  }
  return;
}


// Function: sub_3d07e @ 0x3d07e
void sub_3d07e(unsigned long *a0) // return-dupe
{
  unsigned long v1;
  
  if (a0[2] != a0[1])
    return;
  if (!a0[1])
    a0[1] = 4;
  v1 = *a0;
  *a0 = sub_60a5f(v1,&a0[1],8);
  return;
}


// Function: sub_3d0e5 @ 0x3d0e5
void sub_3d0e5(unsigned long a0,char a1) // ternary
{
  int8 v1;
  int8 *v2; // rax
  
  v2 = (a1) ? (int8 *)0x9fd80 : (int8 *)0x9fd98; // branch-flip
  sub_3d07e(v2);
  v1 = v2[2];
  v2[2] = v1 + 1;
  *(unsigned long *)(*v2 + v1 * 8) = a0;
  return;
}


// Function: sub_3d14d @ 0x3d14d
void sub_3d14d(unsigned long *a0) // return-dupe
{
  if (!a0[1])
    return;
  free((void *)*a0);
  return;
}


// Function: sub_3d17c @ 0x3d17c
void sub_3d17c(void)
{
  sub_3d14d(0x9fd80);
  sub_3d14d(0x9fd98);
  return;
}


// Function: sub_3d1a5 @ 0x3d1a5
void sub_3d1a5(unsigned int a0,unsigned long a1,unsigned long a2,int4 a3) // return-dupe x2
{
  int8 v1;
  int8 v2;
  char *v3; // rax
  uint8 v4; // rax
  int8 v5; // stack - 0x28
  char *v6; // stack - 0x20
  int8 v7; // stack - 0x18
  
  if (dat_9f96c <= 0)
    return;
  if (!dat_9fdb8)
    dat_9fdb8 = (char *)sub_60a5f(0,0x9ec88,1);
  while( true ) {
    if (a3) { // branch-flip
      v5 = flistxattr(a3,dat_9fdb8,dat_9ec88);
      v1 = v5;
      v2 = v5;
    }
    else {
      v1 = sub_47394(a0,a1,dat_9fdb8,dat_9ec88);
      v5 = v1;
      v2 = v1;
    }
    if ((v2 != -1) || (v5 = v1, *__errno_location() != 0x22)) break;
    dat_9fdb8 = (char *)sub_60a5f(dat_9fdb8,0x9ec88,1);
  }
  if (v5 != -1) {
    v6 = dat_9fdb8;
    if (!dat_9fdc0)
      dat_9fdc0 = sub_60a5f(0,0x9ec90,1);
    for (; 1 <= v5; v5 = (v5 - v4) + -1) {
      v4 = strlen(v6);
      while( true ) {
        if (a3) { // branch-flip
          v7 = fgetxattr(a3,v6,dat_9fdc0,dat_9ec90);
          v1 = v7;
          v2 = v7;
        }
        else {
          v1 = sub_46e17(a0,a1,v6,dat_9fdc0,dat_9ec90);
          v7 = v1;
          v2 = v1;
        }
        if ((v2 != -1) || (v7 = v1, *__errno_location() != 0x22)) break;
        dat_9fdc0 = sub_60a5f(dat_9fdc0,0x9ec90,1);
      }
      if (v7 != -1) { // branch-flip
        if (sub_3d84f(v6,1) != '\x01')
          sub_1d78e(a2,v6,dat_9fdc0,v7);
      }
      else if (*__errno_location() != 0x3d) {
        if (a3) // branch-flip
          v3 = "fgetxattr";
        else {
          v3 = "lgetxattrat";
        }
        sub_3e038(v3,a1);
      }
      v6 = &v6[v4 + 1];
    }
    return;
  }
  if (a3) // branch-flip
    v3 = "flistxattr";
  else {
    v3 = "llistxattrat";
  }
  sub_3e038(v3,a1);
  return;
}


// Function: sub_3d44b @ 0x3d44b
void sub_3d44b(unsigned long a0,unsigned long a1,char a2,unsigned long a3,int8 a4,unsigned long a5)
{
  char *v1; // rax
  int4 v2; // stack - 0x24
  char *v3; // stack - 0x20
  
  if (a4) {
    v3 = "setxattrat";
    if (a2 != '2') // branch-flip
      v2 = sub_46591(dat_9c268,a1,a3,a4,a5,0);
    else {
      v3 = "lsetxattr";
      v2 = sub_46870(dat_9c268,a1,a3,a4,a5,0);
    }
    if ((v2 == -1) && (dat_9ec84 & 0x200000)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = gettext("%s: Cannot set \'%s\' extended attribute for file \'%s\'");
      error(0,*__errno_location(),v1,v3,a3,a1);
    }
  }
  return;
}


// Function: sub_3d566 @ 0x3d566
void sub_3d566(unsigned int a0,unsigned long a1,int8 a2,int4 a3)
{
  int4 v1; // eax
  char *v2; // rax
  
  if (1 <= dat_9f964) {
    if (a3) // branch-flip
      v1 = sub_61d8a(a3,a2 + 0x30);
    else {
      v1 = sub_5f56b(a0,a1,a2 + 0x30);
    }
    if (v1 == -1) {
      if (*__errno_location() != 0x3d) {
        if (*__errno_location() != 0x5f) {
          if (a3) // branch-flip
            v2 = "fgetfilecon";
          else {
            v2 = "lgetfileconat";
          }
          sub_3e038(v2,a1);
        }
      }
    }
  }
  return;
}


// Function: sub_3d609 @ 0x3d609
void sub_3d609(int8 a0,unsigned long a1,char a2)
{
  char *v1; // rax
  int4 v2; // stack - 0x24
  char *v3; // stack - 0x20
  
  if ((1 <= dat_9f964) && (*(int8 *)(a0 + 0x30))) {
    if (a2 != '2') { // branch-flip
      v2 = sub_5f7f9(dat_9c268,a1,*(unsigned long *)(a0 + 0x30));
      v3 = "setfileconat";
    }
    else {
      v2 = sub_5fa87(dat_9c268,a1,*(unsigned long *)(a0 + 0x30));
      v3 = "lsetfileconat";
    }
    if ((v2 == -1) && (dat_9ec84 & 0x200000)) {
      if (dat_9fdc8)
        (*dat_9fdc8)();
      v1 = gettext("%s: Cannot set SELinux context for file \'%s\'");
      error(0,*__errno_location(),v1,v3,a1);
    }
  }
  return;
}


// Function: sub_3d717 @ 0x3d717
unsigned long sub_3d717(char *a0,int8 *a1) // return-dupe
{
  char *v1;
  int4 v2; // stack - 0xc
  
  if (!a1[1])
    return 0;
  v2 = 0;
  while( true ) {
    if ((uint8)a1[2] <= (uint8)(int8)v2)
      return 0;
    v1 = *(char **)((int8)v2 * 8 + *a1);
    if (!fnmatch(v1,a0,0)) break;
    v2 += 1;
  }
  return 1;
}


// Function: sub_3d79b @ 0x3d79b
unsigned long sub_3d79b(char *a0,char a1) // early-return
{
  uint8 v1;
  int4 v2; // eax
  unsigned long v3; // rax
  bool v4; // zf
  
  if (dat_9fd88) // branch-flip
    v3 = sub_3d717(a0,0x9fd80);
  else {
    if (a1) {
      v3 = 1;
      return v3;
    }
    v2 = strncmp(a0,"user.",5);
    v4 = v2 == 0;
    v1 = (uint8)v3 >> 8;
    v3 = CONCAT71((undefined7)v1,v4);
  }
  return v3;
}


// Function: sub_3d803 @ 0x3d803
unsigned long sub_3d803(unsigned long a0)
{
  if ((dat_9fda0) && (sub_3d717(a0,0x9fd98)))
    return 1;
  return 0;
}


// Function: sub_3d84f @ 0x3d84f
unsigned long sub_3d84f(unsigned long a0,char a1)
{
  if ((sub_3d79b(a0,a1)) && (!sub_3d803(a0,a1)))
    return 0;
  return 1;
}


// Function: sub_3d8a1 @ 0x3d8a1
void sub_3d8a1(int8 a0,unsigned long a1,char a2,uint4 a3)
{
  char *v1;
  int8 v2;
  uint8 v3; // stack - 0x18
  
  if ((1 <= dat_9f96c) && (v3 = 0, *(int8 *)(a0 + 0x158))) {
    for (; v3 < *(uint8 *)(a0 + 0x158); v3 = v3 + 1) {
      v2 = *(int8 *)(v3 * 0x18 + *(int8 *)(a0 + 0x160));
      v1 = (char *)(v2 + strlen("SCHILY.xattr."));
      if (((a2 != '0') || (a3 != (strcmp(v1,"security.capability") != 0))) && (!sub_3d84f(v1,0)))
        sub_3d44b(a0,a1,(int4)a2,v1,*(unsigned long *)(v3 * 0x18 + *(int8 *)(a0 + 0x160) + 8),*(unsigned long *)(v3 * 0x18 + *(int8 *)(a0 + 0x160) + 0x10));
    }
  }
  return;
}


// Function: sub_3d9ec @ 0x3d9ec
void sub_3d9ec(int8 a0,char *a1)
{
  int8 v1;
  uint8 v2; // rax
  int4 v3; // stack - 0x24
  
  if (2 <= dat_9f9ac) { // branch-flip
    if (((0 < dat_9f96c) || (0 < dat_9f964)) || (1 <= dat_9f968)) {
      *a1 = 0x20;
      a1[1] = 0;
    }
    if ((1 <= dat_9f96c) && (*(int8 *)(a0 + 0x158))) {
      for (v3 = 0; (uint8)(int8)v3 < *(uint8 *)(a0 + 0x158); v3 = v3 + 1) {
        v1 = *(int8 *)((int8)v3 * 0x18 + *(int8 *)(a0 + 0x160));
        v2 = strlen("SCHILY.xattr.");
        if (sub_3d84f(v2 + v1,0) != '\x01') {
          *a1 = 0x2a;
          break;
        }
      }
    }
    if ((1 <= dat_9f964) && (*(int8 *)(a0 + 0x30)))
      *a1 = 0x2e;
    if ((1 <= dat_9f968) && ((*(int8 *)(a0 + 0x40) || (*(int8 *)(a0 + 0x50)))))
      *a1 = 0x2b;
  }
  else {
    *a1 = 0;
  }
  return;
}


// Function: sub_3db33 @ 0x3db33
void sub_3db33(int8 a0)
{
  int8 v1;
  int4 v2; // stack - 0x24
  
  if (3 <= dat_9f9ac) {
    if ((1 <= dat_9f964) && (*(int8 *)(a0 + 0x30)))
      fprintf(dat_9efd0,"  s: %s\n",*(unsigned long *)(a0 + 0x30));
    if ((1 <= dat_9f968) && ((*(int8 *)(a0 + 0x40) || (*(int8 *)(a0 + 0x50))))) {
      fprintf(dat_9efd0,"  a: ");
      sub_3cae5(0x87cb4,0x2c,*(unsigned long *)(a0 + 0x38),*(unsigned long *)(a0 + 0x40));
      if ((*(int8 *)(a0 + 0x40)) && (*(int8 *)(a0 + 0x50)))
        fprintf(dat_9efd0,",");
      sub_3cae5("default:",0x2c,*(unsigned long *)(a0 + 0x48),*(unsigned long *)(a0 + 0x50));
      fprintf(dat_9efd0,"\n");
    }
    if ((1 <= dat_9f96c) && (*(int8 *)(a0 + 0x158))) {
      for (v2 = 0; (uint8)(int8)v2 < *(uint8 *)(a0 + 0x158); v2 = v2 + 1) {
        v1 = *(int8 *)((int8)v2 * 0x18 + *(int8 *)(a0 + 0x160));
        v1 = strlen("SCHILY.xattr.") + v1;
        if (sub_3d84f(v1,0) != '\x01')
          fprintf(dat_9efd0,"  x: %lu %s\n",*(unsigned long *)((int8)v2 * 0x18 + *(int8 *)(a0 + 0x160) + 0x10),v1);
      }
    }
  }
  return;
}

