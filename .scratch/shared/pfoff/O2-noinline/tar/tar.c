// Function: main @ 0xbb10
int4 main(int4 argc,char **argv,char **envp)
{
  sub_cd20();
  sub_48760(*argv);
  setlocale(6,"");
  bindtextdomain("tar","/usr/local/share/locale");
  textdomain("tar");
  dat_83e08 = 2;
  dat_84d50 = sub_10820;
  dat_84d58 = 0;
  sub_4a590(NULL,7);
  sub_3e420(dcgettext(NULL,"stdout",5));
  if (!sub_4bd80()) {
    dat_84c18 = 10;
    dat_84838 = (void *)sub_4c5b0(0x50);
    dat_84830 = 0;
    signal(0x11,0);
    sub_2e690(argc,argv);
    sub_26c90();
    if (dat_84a48)
      sub_d850();
    switch(dat_84ba8) {
      case 0:
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options",5));
        sub_2c970(2); // no-return
      case 1:
      case 2:
      case 8:
        sub_30ef0();
        break;
      case 3:
        sub_143d0();
        break;
      case 4:
        sub_15dc0();
        break;
      case 5:
        sub_10af0();
        sub_22c60(sub_11730);
        break;
      case 6:
        sub_19b30();
        sub_22c60(sub_19c10);
        sub_18640();
        break;
      case 7:
        sub_22c60(sub_23260);
        break;
      case 9:
        sub_232e0();
      
    }
    sub_10910();
    if (dat_84a69)
      sub_d150();
    if (dat_84c20)
      sub_14030();
    if (dat_84a48)
      sub_d960();
    free(dat_84838);
    sub_31b40();
    sub_26cc0();
    if (dat_84d58 == 2)
      error(0,0,dcgettext(NULL,"Exiting with failure status due to previous errors",5));
    if (dat_83f28 != stdout) { // branch-flip
      if ((ferror_unlocked(stderr)) || (fclose(stderr)))
        sub_2f7d0(2);
    }
    else {
      sub_3e440();
    }
    return dat_84d58;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,"%s",dcgettext(NULL,"failed to assert availability of the standard file descriptors",5));
  sub_163d0(); // no-return
}


// Function: sub_bed0 @ 0xbed0
int8 * sub_bed0(uint8 a0)
{
  int8 *v1;
  int8 *v2;
  
  v2 = dat_84148;
  while (((v2 && (v1 = (int8 *)*v2, v1)) && ((uint8)(v1[1] << 9) <= a0))) {
    v2 = v1;
  }
  return v2;
}


// Function: sub_bf10 @ 0xbf10
int8 sub_bf10(int4 a0,uint4 *a1)
{
  int4 v1;
  int4 v2; // eax
  uint4 v3;
  uint8 v4;
  int4 *v5;
  
  v3 = *a1;
  v4 = (uint8)(int4)v3;
  v5 = (int4 *)(v4 * 0x18 + 0x81020);
  v1 = *v5;
  while( true ) {
    if (!v1) {
      *a1 = v3;
      return 0;
    }
    v2 = (int4)v4;
    v3 = v2 + 1;
    v4 = (uint8)v3;
    if (a0 == v1) break;
    v1 = v5[6];
    v5 = &v5[6];
  }
  *a1 = v3;
  return (int8)v2 * 0x18 + 0x81020;
}


// Function: sub_bf70 @ 0xbf70
unsigned long sub_bf70(unsigned long a0) // early-return
{
  int8 v1; // rax
  unsigned int v2; // stack - 0x14
  
  v2 = 0;
  v1 = sub_bf10(a0,&v2);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x10);
}


// Function: sub_bfc0 @ 0xbfc0
void sub_bfc0(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = dat_84148;
  while ((v2 && (v2 != a0))) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[2]);
    free(v2);
    v2 = v1;
  }
  dat_84148 = v2;
  if (v2)
    return;
  dat_84140 = 0;
  return;
}


// Function: sub_c030 @ 0xc030
void sub_c030(int8 *a0,int8 a1) // return-dupe
{
  sub_bfc0(a0);
  if (!a0)
    return;
  do {
    a0[1] = a0[1] + a1;
    a0[5] = 0;
    a0 = (int8 *)*a0;
  } while (a0);
  return;
}


// Function: sub_c070 @ 0xc070
void sub_c070(void)
{
  sub_411a0((timespec *)0x84a10);
  dat_84a00 = dat_84a10;
  dat_84a08 = dat_84a18;
  return;
}


// Function: sub_c0a0 @ 0xc0a0
int4 sub_c0a0(float8 a0,FILE *a1,char *a2)
{
  float8 v1;
  int4 v2;
  char *v3; // rax
  char v4 [656];
  char v5 [664];
  uint8 v6; // rdi
  char *v7; // rax
  float8 v8; // xmm2_qa
  
  v2 = 0;
  if ((a2) && (v2 = 0, *a2))
    v2 = __fprintf_chk(a1,1,"%s: ",dcgettext(NULL,a2,5));
  v1 = dat_60758;
  if ((dat_83f78 <= dat_60748) || (v8 = a0 / dat_83f78, dat_60750 <= v8))
    v7 = "?";
  else {
    if (dat_60758 <= v8)
      v6 = (int8)(v8 - dat_60758) ^ 0x8000000000000000;
    else {
      v6 = (uint8)v8;
    }
    v7 = (char *)sub_426b0(v6,v5,0x1b0,1,1);
  }
  if (v1 <= a0)
    v6 = (int8)(a0 - v1) ^ 0x8000000000000000;
  else {
    v6 = (uint8)a0;
  }
  v3 = (char *)sub_426b0(v6,v4,0x1b0,1,1);
  return __fprintf_chk(a1,1,"%.0f (%s, %s/s)",a0,v3,v7) + v2;
}


// Function: sub_c240 @ 0xc240
void sub_c240(void)
{
  dat_81010 += 1;
  if (0 <= dat_81010) {
    dat_81014 += 1;
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Volume number overflow",5));
  sub_163d0(); // no-return
}


// Function: sub_c290 @ 0xc290
void sub_c290(void) // return-dupe x3
{
  char v1;
  int8 v2; // rax
  uint8 v3; // rax
  int8 v4; // rbx
  unsigned short v5 [2]; // stack - 0x28
  unsigned int v6; // stack - 0x24
  
  v6 = 1;
  v5[0] = 4;
  if (0x40000000 <= dat_84a34) { // branch-flip
    sub_34080(dat_84a34 + -0x40000000,0x40086d01,v5);
    v1 = dat_3;
  }
  else {
    v1 = (char)((uint4)ioctl(dat_84a34,0x40086d01,v5) >> 0x18);
  }
  if ('\0' <= v1)
    return;
  if (*__errno_location() == 5) {
    if (0x40000000 <= dat_84a34) { // branch-flip
      sub_34080(dat_84a34 + -0x40000000,0x40086d01,v5);
      v1 = dat_3;
    }
    else {
      v1 = (char)((uint4)ioctl(dat_84a34,0x40086d01,v5) >> 0x18);
    }
    if ('\0' <= v1)
      return;
  }
  if (0x40000000 <= dat_84a34) // branch-flip
    v2 = sub_33f90(dat_84a34 + -0x40000000,0,1);
  else {
    v2 = lseek(dat_84a34,0,1);
  }
  v4 = v2 - (dat_83f50 - (int8)dat_83f58);
  if (v4 <= -1)
    v4 = 0;
  if (0x40000000 <= dat_84a34) // branch-flip
    v3 = sub_33f90(dat_84a34 + -0x40000000,v4,0);
  else {
    v3 = lseek(dat_84a34,v4,0);
  }
  if (v3 == v4)
    return;
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Cannot backspace archive file; it may be unreadable without -i",5));
  if (dat_83f58 == dat_84cf0)
    return;
  memset(dat_83f58,0,(int8)dat_84cf0 - (int8)dat_83f58); // tail-call
  return;
}


// Function: sub_c490 @ 0xc490
int8 sub_c490(void)
{
  uint8 v1;
  unsigned long *v2;
  int8 v3;
  int8 v4; // rcx
  uint8 v5;
  int8 v6;
  
  sub_108d0(1);
  if ((dat_84a70 != dat_60748) && (dat_84a70 <= dat_841a0)) {
    *__errno_location() = 0x1c;
    return 0;
  }
  v3 = dat_84b98;
  if (!dat_84a30)
    v3 = sub_2af00();
  if ((((v3) && (dat_84b10)) && (!dat_84138)) && (v2 = (unsigned long *)sub_bed0(v3), v2)) {
    v6 = v2[1];
    v5 = v3 + v6 * -0x200;
    v1 = v2[4];
    v4 = (v5 >> 9) + v2[5];
    if (v1 <= v5)
      v5 = v1;
    v2[5] = v4;
    v2[4] = v1 - v5;
    if (!(v1 - v5)) {
      v2 = (unsigned long *)*v2;
      v6 += v4;
    }
    sub_c030(v2,-v6);
  }
  return v3;
}


// Function: sub_c580 @ 0xc580
void sub_c580(FILE *a0)
{
  uint4 v1;
  char *v10; // rsi
  uint1 *v11;
  uint1 *v2;
  unsigned long *v3;
  uint1 v4;
  unsigned long v5; // rax
  int8 v6; // rax
  uint1 *v7;
  unsigned long v8; // stack - 0x48
  uint1 *v9; // stack - 0x50
  
  v9 = NULL;
  v8 = 0;
label_c5c8:
  do {
    fputc_unlocked(7,stderr);
    v5 = sub_4ac50((char *)*dat_84828);
    v1 = dat_81010 + 1;
    __fprintf_chk(stderr,1,dcgettext(NULL,"Prepare volume #%d for %s and hit return: ",5),(uint8)v1,v5);
    fflush_unlocked(stderr);
    v6 = getline((char **)&v9,&v8,a0);
    v3 = dat_84828;
    if (v6 <= 0) {
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "EOF where user reply was expected";
label_c772:
      error(0,0,dcgettext(NULL,v10,5));
      if (3 <= (uint4)(dat_84ba8 - 5U)) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"WARNING: Archive is incomplete",5));
      }
      sub_163d0(); // no-return
    }
    v4 = *v9;
    v11 = v9;
    if (((v4 & 0xdf) == 0x59) || (v4 == 10)) {
      free(v11); // return-dupe
      return;
    }
    if (v4 == 0x3f) {
      __fprintf_chk(stderr,1,dcgettext(NULL," n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n",5));
      if (!dat_84a58)
        __fprintf_chk(stderr,1,dcgettext(NULL," !             Spawn a subshell\n",5));
      v10 = " ?             Print this list\n";
label_c68c:
      __fprintf_chk(stderr,1,dcgettext(NULL,v10,5));
      goto label_c5c8;
    }
    if ('@' <= (char)v4) { // branch-flip
      if (v4 != 0x6e) {
        if (v4 != 0x71) {
label_c680:
          v10 = "Invalid input. Type ? for help.\n";
          goto label_c68c;
        }
        if (dat_84d50)
          (*dat_84d50)();
        v10 = "No new volume; exiting.\n";
        goto label_c772;
      }
      v4 = v9[1];
      v11 = &v9[1];
      if ((v4 == 9) || (v4 == 0x20)) {
        do {
          do {
            v4 = v11[1];
            v11 = &v11[1];
          } while (v4 == 0x20);
        } while (v4 == 9);
      }
      v7 = v11;
      if ((v4 != 10) && (v4)) {
        do {
          v2 = &v7[1];
          v7 = &v7[1];
          if (!*v2) break;
        } while (*v2 != 10);
      }
      *v7 = 0;
      if (*v11) {
        *v3 = sub_4c7a0(v11);
        v11 = v9;
        free(v11);
        return;
      }
      __fprintf_chk(stderr,1,"%s",dcgettext(NULL,"File name not specified. Try again.\n",5));
    }
    else {
      if ((v4 != 0x21) || (dat_84a58)) goto label_c680;
      sub_2add0();
    }
  } while( true );
}


// Function: sub_c8d0 @ 0xc8d0
void sub_c8d0(void) // return-dupe x2
{
  stat v1; // stack - 0xa8
  
  if (dat_84ba8 == 4)
    dat_847f8 = 0;
  if (dat_847fc != -1) {
    dat_847f8 = dat_847fc != 0;
    return;
  }
  if (((!dat_84b10) && (!dat_84b70)) && (!fstat(dat_84a34,&v1))) {
    dat_847f8 = (v1._24_4_ & 0xf000) == 0x8000;
    return;
  }
  dat_847f8 = 0;
  return;
}


// Function: sub_c980 @ 0xc980
void sub_c980(void)
{
  int8 v1;
  int8 v2; // rbp
  
  v2 = (int8)dat_84178;
  v1 = *(int8 *)(v2 * 8 + 0x84180);
  if (!v1) {
    *(unsigned long *)(v2 * 8 + 0x84180) = sub_25000((int8 *)(v2 * 8 + 0x84190),dat_84b98);
    v1 = *(int8 *)((int8)dat_84178 * 8 + 0x84180);
  }
  dat_83f58 = v1;
  dat_83f48 = v1;
  dat_83f50 = v1 + (int8)dat_84ba0 * 0x200;
  return;
}


// Function: sub_ca00 @ 0xca00
unsigned int sub_ca00(void)
{
  unsigned int v1;
  stat v2; // stack - 0xa8
  
  if (fstat(dat_84a34,&v2)) { // branch-flip
    sub_24ec0((char *)*dat_84828);
    v1 = 0;
  }
  else {
    v1 = CONCAT31((undefined3)((v2._24_4_ & 0xb000) >> 8),(v2._24_4_ & 0xb000) == 0x2000);
  }
  return v1;
}


// Function: sub_ca80 @ 0xca80
void sub_ca80(int4 a0)
{
  char *v1;
  
  v1 = (char *)*dat_84838;
  if (!strcmp(v1,"-")) {
    if (isatty((uint4)(a0 != 0))) {
      if (dat_84d50)
        (*dat_84d50)();
      if (a0) // branch-flip
        v1 = dcgettext(NULL,"Refusing to write archive contents to terminal (missing -f option?)",5);
      else {
        v1 = dcgettext(NULL,"Refusing to read archive contents from terminal (missing -f option?)",5);
      }
      error(0,0,v1);
      sub_163d0(); // no-return
    }
  }
  return;
}


// Function: sub_cb00 @ 0xcb00
void sub_cb00(int8 a0) // return-dupe
{
  unsigned long *v1;
  int8 v2; // rcx
  void *v3 [14]; // stack - 0x1d8
  void **v4;
  unsigned int v5; // stack - 0x168
  uint4 v6; // stack - 0x164
  uint4 v7; // stack - 0x160
  unsigned long v8; // stack - 0x150
  unsigned long v9; // stack - 0xc0
  
  if (dat_84ba4 != 4)
    return;
  v2 = 0x37;
  v4 = v3;
  while (v2) {
    v1 = &v4[1];
    *v4 = NULL;
    v2 -= 1;
    v4 = (void **)v1;
  }
  v3[0] = *(void **)(a0 + 0x10);
  v5 = 0x81a4;
  v3[1] = v3[0];
  v6 = getuid();
  v7 = getgid();
  v3[0] = (void *)sub_1c080(v3,"%d/GNUFileParts/%f.%n",(int8)dat_81014);
  v3[1] = v3[0];
  v8 = *(unsigned long *)(a0 + 0x20);
  v9 = v8;
  v1 = (unsigned long *)sub_132d0(v3);
  if (v1) {
    sub_12f00((uint1 *)sub_12da0(0,v3,v1));
    free(v3[0]);
    return;
  }
  abort(); // no-return
}


// Function: sub_cc00 @ 0xcc00
void sub_cc00(char *a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  if (!dat_84b10)
    return;
  v1 = (unsigned long *)sub_4c5b0(0x30);
  if (dat_84140) // branch-flip
    *dat_84140 = v1;
  else {
    dat_84148 = v1;
  }
  *v1 = 0;
  v1[1] = dat_83f48 - dat_83f58 >> 9;
  dat_84140 = v1;
  v2 = sub_4c7a0(a0);
  v1[3] = a1;
  v1[4] = a2;
  v1[2] = v2;
  v1[5] = 0;
  return;
}


// Function: sub_cca0 @ 0xcca0
void sub_cca0(void)
{
  sub_1ceb0((struct_6 *)0x840e8); // tail-call
  return;
}


// Function: sub_ccb0 @ 0xccb0
void sub_ccb0(struct_18 *a0)
{
  sub_cc00(a0->field_0x0,a0->field_0x88,a0->field_0x88); // tail-call
  return;
}


// Function: sub_ccd0 @ 0xccd0
void sub_ccd0(void)
{
  if (!dat_84b10)
    return;
  sub_bfc0(NULL); // tail-call
  return;
}


// Function: sub_ccf0 @ 0xccf0
void sub_ccf0(unsigned long a0) // return-dupe
{
  if (!dat_84148)
    return;
  *(unsigned long *)(dat_84148 + 0x20) = a0;
  return;
}


// Function: sub_cd10 @ 0xcd10
void sub_cd10(void)
{
  dat_84164 = 0;
  return;
}


// Function: sub_cd20 @ 0xcd20
void sub_cd20(void)
{
  sub_411a0((timespec *)0x84a20);
  dat_84a10 = dat_84a20;
  dat_84a18 = dat_84a28;
  dat_84a00 = dat_84a20;
  dat_84a08 = dat_84a28;
  return;
}


// Function: sub_cd50 @ 0xcd50
float8 sub_cd50(void)
{
  int8 v1; // stack - 0x28
  int8 v2; // stack - 0x20
  
  sub_411a0(&v1);
  dat_83f78 = (float8)(v2 - dat_84a08) / dat_60760 + (float8)(v1 - dat_84a00) + dat_83f78;
  sub_411a0((timespec *)0x84a00);
  return dat_83f78;
}


// Function: sub_cdf0 @ 0xcdf0
int8 sub_cdf0(uint4 *a0)
{
  int8 v1; // rax
  int8 v2;
  
  v2 = dat_84b70;
  if ((!dat_84b70) && (dat_83f70)) {
    *a0 = 0;
    v1 = sub_bf10(dat_83f70,a0);
    if (v1)
      return *(int8 *)(v1 + 8);
  }
  return v2;
}


// Function: sub_ce40 @ 0xce40
unsigned long sub_ce40(uint4 *a0)
{
  int8 v1; // rax
  
  if (!dat_84b70) {
    v1 = sub_bf10(dat_83f70,a0);
    if (v1)
      return *(unsigned long *)(v1 + 8);
  }
  return 0;
}


// Function: sub_ce80 @ 0xce80
int4 sub_ce80(FILE *a0,unsigned long *a1,uint8 a2,int4 a3)
{
  int4 v1;
  int4 v2; // eax
  uint8 v3; // rax
  char *v4;
  char v5 [24];
  uint8 v6;
  float8 v7; // xmm0_qa
  
  if (8 <= dat_84ba8) { // branch-flip
    if (dat_84ba8 != 8)
      abort(); // no-return, return-dupe
  }
  else {
    if (4 < dat_84ba8) {
      v4 = (char *)*a1;
      v1 = sub_c0a0(a0,dcgettext(NULL,v4,5));
      goto label_d017;
    }
    if (dat_84ba8 == 4) {
      v3 = dat_83f38 * dat_84b98;
      v6 = a2;
      if ((int8)v3 < 0)
        v6 = v3 >> 1 | (uint8)((uint4)v3 & 1);
      v1 = sub_c0a0(a0,*a1,v6);
      fputc_unlocked((int4)a2,a0);
      v2 = sub_c0a0(a0,a1[1]);
      fputc_unlocked((int4)a2,a0);
      v4 = (char *)a1[2];
      v1 = v1 + v2 + 2;
      if ((v4) && (*v4)) {
        v4 = dcgettext(NULL,v4,5);
        v1 += __fprintf_chk(a0,1,"%s: ",v4);
      }
      v7 = (float8)(uint8)((dat_83f38 - dat_842d8) * dat_84b98) - (dat_841a8 + dat_841a0);
      if (dat_60758 <= v7)
        v6 = (int8)(v7 - dat_60758) ^ 0x8000000000000000;
      else {
        v6 = (uint8)v7;
      }
      v4 = (char *)sub_43090(v6,v5);
      v1 += __fprintf_chk(a0,1,"%s",v4);
      goto label_d017;
    }
    if (!dat_84ba8) {
      abort();
    }
  }
  v1 = sub_c0a0(a0,a1[1]);
label_d017:
  if (a3) {
    v1 += 1;
    fputc_unlocked(a3,a0);
  }
  return v1;
}


// Function: sub_d150 @ 0xd150
int4 sub_d150(void)
{
  int4 v1; // eax
  
  v1 = sub_ce80(stderr,(unsigned long *)0x7f0e0,10,10); // tail-call
  return v1;
}


// Function: sub_d180 @ 0xd180
int8 sub_d180(void)
{
  return (dat_83f48 - dat_83f58 >> 9) + dat_84170;
}


// Function: sub_d1a0 @ 0xd1a0
void sub_d1a0(void) // return-dupe
{
  if (!dat_84161)
    return;
  dat_83f40 = 1;
  dat_83f48 = dat_83f58;
  dat_83f50 = dat_83f58 + (int8)dat_84ba0 * 0x200;
  dat_84161 = 0;
  return;
}


// Function: sub_d1f0 @ 0xd1f0
void sub_d1f0(uint8 a0)
{
  uint8 v1;
  
  v1 = dat_83f48;
  if (dat_83f48 <= a0) {
    v1 = dat_83f48 + 0x200 + (a0 - dat_83f48 & 0xfffffffffffffe00);
    dat_83f48 = v1;
  }
  if (v1 <= dat_83f50)
    return;
  abort(); // no-return
}


// Function: sub_d2b0 @ 0xd2b0
int8 sub_d2b0(int8 a0)
{
  return dat_83f50 - a0;
}


// Function: sub_d2c0 @ 0xd2c0
void sub_d2c0(int4 a0)
{
  if (!close(a0))
    return;
  sub_32680(dcgettext(NULL,"(pipe)",5)); // tail-call
  return;
}


// Function: sub_d300 @ 0xd300
void sub_d300(void)
{
  int4 v1;
  char *v2; // rsi
  
  sub_32820((char *)*dat_84828);
  v1 = dat_84164;
  if (dat_84170) { // branch-flip
    dat_84164 += 1;
    if (v1 <= 10)
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v2 = "Too many errors, quitting";
  }
  else {
    if (dat_84d50)
      (*dat_84d50)();
    v2 = "At beginning of tape, quitting now";
  }
  error(0,0,dcgettext(NULL,v2,5));
  sub_163d0(); // no-return
}


// Function: sub_d390 @ 0xd390
void sub_d390(uint8 a0)
{
  uint8 v1;
  void *v2;
  
  v2 = (void *)(dat_83f58 + a0);
  v1 = dat_84b98 - a0;
  if ((((v1) && (dat_83b82 & 0x40)) && (!(v1 & 0x1ff) && !dat_84170)) && ((a0 && (sub_ca00())))) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcngettext(NULL,"Record size = %lu block","Record size = %lu blocks",a0 >> 9,5),a0 >> 9);
  }
  while( true ) {
    if (v1 & 0x1ff) { // branch-flip
      if (!a0) break;
    }
    else if (((!v1) || (!a0)) || (!dat_84160)) break;
    while( true ) {
      if (0x40000000 <= dat_84a34) // branch-flip
        a0 = sub_33da0(dat_84a34 + -0x40000000,v2,v1);
      else {
        a0 = sub_4af10(dat_84a34,v2,v1);
      }
      if (a0 != 0xffffffffffffffff) break;
      sub_d300();
    }
    if (!a0) break;
    if (!dat_84160) {
      v1 = dat_84b98 - v1;
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcngettext(NULL,"Unaligned block (%lu byte) in archive","Unaligned block (%lu bytes) in archive",v1,5),v1);
      sub_163d0(); // no-return
    }
    v1 -= a0;
    v2 = (void *)((int8)v2 + a0);
  }
  dat_83f38 += 1;
  dat_83f50 = (dat_84b98 - v1 & 0xfffffffffffffe00) + dat_83f58;
  return;
}


// Function: sub_d540 @ 0xd540
uint8 sub_d540(int8 a0)
{
  char v1 [16];
  char v2 [16];
  int8 v3; // rax
  int8 v4; // rax
  unsigned long v5; // rdx
  uint8 v6;
  uint8 v7; // rax
  
  v3 = (int8)dat_84ba0 - (dat_83f48 - dat_83f58 >> 9);
  if (v3 * 0x200 < a0) {
    v7 = a0 + v3 * -0x200;
    if (dat_84b98 <= v7) {
      v6 = dat_84b98;
      sub_d180(v7,dat_84b98,0);
      v1._8_8_ = 0;
      v1._0_8_ = v6;
      v2._8_8_ = v5;
      v2._0_8_ = v7;
      v6 *= SUB168(v2._0_16_ / v1._0_16_,0);
      if (0x40000000 <= dat_84a34) // branch-flip
        v7 = sub_33f90(dat_84a34 + -0x40000000,v6,1);
      else {
        v7 = lseek(dat_84a34,v6,1);
      }
      if (0 <= (int8)v7) {
        if (v7 % dat_84b98) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"rmtlseek not stopped at a record boundary",5));
          sub_163d0(); // no-return
        }
        dat_84170 = ((int8)v7 >> 9) - (int8)dat_84ba0;
        v7 = ((int8)v7 >> 9) - v4;
        dat_83f38 += (int8)v7 / (int8)dat_84ba0;
        dat_83f48 = dat_83f50;
      }
      return v7;
    }
  }
  return 0;
}


// Function: sub_d670 @ 0xd670
void sub_d670(void)
{
  int4 v1; // eax
  
  sub_32e30();
  if (0x40000000 <= dat_84a34) // branch-flip
    v1 = sub_33d60(dat_84a34 + -0x40000000);
  else {
    v1 = close(dat_84a34);
  }
  if (v1)
    sub_32680((char *)*dat_84828);
  sub_2aca0(dat_84168,0);
  sub_163d0(); // no-return
}


// Function: sub_d6d0 @ 0xd6d0
void sub_d6d0(unsigned long a0)
{
  int4 v1;
  int4 *v2; // rax
  
  if (dat_84a69) {
    v2 = __errno_location();
    v1 = *v2;
    sub_d150();
    *v2 = v1;
  }
  sub_d670(*dat_84828,a0,dat_84b98); // no-return
}


// Function: sub_d720 @ 0xd720
void sub_d720(void)
{
  int8 v1; // rax
  
  v1 = sub_c490();
  if (dat_84b98 == v1) {
    dat_83f30 += 1;
    dat_841a0 = (float8)v1 + dat_841a0;
    return;
  }
  sub_d6d0(v1); // no-return
}


// Function: sub_d770 @ 0xd770
void sub_d770(void)
{
  uint8 v1;
  int4 v2;
  
  sub_108d0(0);
  dat_84164 = 0;
  if ((dat_83f20) && (dat_84170)) {
    dat_84a34 = 1;
    v1 = sub_2af00();
    dat_84a34 = 0;
    if (dat_84b98 == v1) {
      v2 = 0;
      goto label_d7a9;
    }
    sub_d6d0(v1); // no-return
  }
  while( true ) {
    v1 = dat_84b98;
    v2 = dat_84a34;
label_d7a9:
    if (0x40000000 <= v2) // branch-flip
      v1 = sub_33da0(v2 + -0x40000000);
    else {
      v1 = sub_4af10(v2,dat_83f58,v1);
    }
    if (dat_84b98 == v1) break;
    if (v1 != 0xffffffffffffffff) {
      sub_d390(v1); // tail-call
      return;
    }
    sub_d300();
  }
  dat_83f38 += 1;
  return;
}


// Function: sub_d850 @ 0xd850
void sub_d850(void) // return-dupe
{
  int4 v1; // eax
  FILE *v2; // rax
  char *v3; // rax
  
  v2 = fopen(dat_84a48,"r");
  if (!v2) {
    if (*__errno_location() == 2)
      return;
    sub_327c0(dat_84a48); // tail-call
    return;
  }
  if ((__isoc99_fscanf(v2,"%d",(int4 *)0x81010) == 1) && (0 <= dat_81010)) {
    if (ferror_unlocked(v2)) { // branch-flip
      sub_32820(dat_84a48);
      v1 = fclose(v2);
    }
    else {
      v1 = fclose(v2);
    }
    if (!v1)
      return;
    sub_32680(dat_84a48); // tail-call
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v3 = (char *)sub_4aa70(dat_84a48);
  error(0,0,dcgettext(NULL,"%s: contains invalid volume number",5),v3);
  sub_163d0(); // no-return
}


// Function: sub_d960 @ 0xd960
void sub_d960(void)
{
  int4 v1; // eax
  FILE *v2; // rax
  
  v2 = fopen(dat_84a48,"w");
  if (!v2) {
    sub_327c0(dat_84a48); // tail-call
    return;
  }
  __fprintf_chk(v2,1,"%d\n",dat_81010);
  if (ferror_unlocked(v2)) { // branch-flip
    sub_32e10(dat_84a48);
    v1 = fclose(v2);
  }
  else {
    v1 = fclose(v2);
  }
  if (!v1)
    return;
  sub_32680(dat_84a48); // tail-call
  return;
}


// Function: sub_da00 @ 0xda00
unsigned long sub_da00(int4 a0)
{
  int4 v1;
  uint8 v2; // rax
  FILE *v3;
  char *v4;
  unsigned long *v5;
  
  if ((!dat_83f68) && (!dat_84b48)) {
    v3 = stdin;
    if (!dat_84a34)
      v3 = fopen("/dev/tty","r");
    dat_83f68 = v3;
  }
  if (dat_84280)
    return 0;
  if (dat_84a50)
    sub_118d0();
  sub_23be0((unsigned long *)0x83f18,NULL);
  sub_23be0((unsigned long *)0x83f10,NULL);
  dat_83f00 = 0;
  dat_83f08 = 0;
  dat_83f48 = dat_83f58;
  if (0x40000000 <= dat_84a34) // branch-flip
    v1 = sub_33d60(dat_84a34 + -0x40000000);
  else {
    v1 = close(dat_84a34);
  }
  if (v1)
    sub_32680((char *)*dat_84828);
  dat_84828 = &dat_84828[1];
  if (dat_84828 == &dat_84838[dat_84830]) {
    dat_84828 = dat_84838;
    dat_83f60 = 1;
    goto label_db51;
  }
  v5 = dat_84828;
  if (dat_83f60) goto label_db51;
  do {
    v4 = (char *)*v5;
    if (!strcmp(v4,"-")) {
      dat_84a34 = 0;
      dat_84160 = 1;
      return 1;
    }
    if (dat_84a50) { // branch-flip
label_dc38:
      if (!dat_84d70) {
        dat_84d78 = strchr(v4,0x3a);
        v4 = (char *)*v5;
        if (((dat_84d78) && (v4 < dat_84d78)) && (v2 = (int8)dat_84d78 - (int8)v4, !memchr(v4,0x2f,v2))) {
          dat_84a34 = sub_338d0(v4,0x42,0x40000000,dat_84ab0);
          goto label_dc7c;
        }
      }
      dat_84a34 = open(v4,0x42,0x1b6);
label_dc7c:
      v1 = dat_84a34;
    }
    else {
      if (a0 == 1) {
        if (dat_84b84) {
          sub_24710(v4,1);
          v4 = (char *)*dat_84828;
          v5 = dat_84828;
        }
        if (!dat_84d70) {
          dat_84d78 = strchr(v4,0x3a);
          v4 = (char *)*v5;
          if (((dat_84d78) && (v4 < dat_84d78)) && (v2 = (int8)dat_84d78 - (int8)v4, !memchr(v4,0x2f,v2))) {
            dat_84a34 = sub_338d0(v4,0x41,0x40000000,dat_84ab0);
            goto label_dc7c;
          }
        }
        dat_84a34 = creat(v4,0x1b6);
        goto label_dc7c;
      }
      if (a0 == 2) goto label_dc38;
      if (dat_84d70) { // branch-flip
label_dc0f:
        dat_84a34 = open(v4,0,0x1b6);
      }
      else {
        dat_84d78 = strchr(v4,0x3a);
        v4 = (char *)*v5;
        if (((!dat_84d78) || (dat_84d78 <= v4)) || (v2 = (int8)dat_84d78 - (int8)v4, memchr(v4,0x2f,v2))) goto label_dc0f;
        dat_84a34 = sub_338d0(v4,0,0x40000000,dat_84ab0);
      }
      sub_c8d0();
      v1 = dat_84a34;
    }
    if (0 <= v1)
      return 1;
    sub_32800((char *)*dat_84828);
    if (((dat_84a50 != '\x01') && (a0 == 1)) && (dat_84b84))
      sub_245d0();
label_db51:
    if (dat_84b48) { // branch-flip
      if (dat_84a48)
        sub_d960();
      v1 = sub_2b8b0(dat_84828,dat_81010 + 1);
      if (v1) {
        if (dat_84d50)
          (*dat_84d50)();
        v4 = (char *)sub_4ac50(dat_84b48);
        error(0,0,dcgettext(NULL,"%s command failed",5),v4);
        sub_163d0(); // no-return
      }
    }
    else {
      sub_c580(dat_83f68);
    }
    v5 = dat_84828;
  } while( true );
}


// Function: sub_de80 @ 0xde80
void * sub_de80(uint1 *a0)
{
  uint8 v1; // rax
  int8 *v2; // rax
  void *v3; // rax
  uint1 *v4;
  
  v1 = strlen((char *)a0);
  if ((v1) && (v4 = &a0[v1 - 1], a0 < v4)) {
    v2 = __ctype_b_loc();
    do {
      if (!(*(uint1 *)(*v2 + 1 + (uint8)*v4 * 2) & 8)) {
        if (v4 <= a0)
          return NULL;
        v4 = &v4[-7];
        if ((a0 < v4) && (!memcmp(v4," Volume ",8))) {
          v1 = (int8)v4 - (int8)a0;
          v3 = (void *)sub_4c5b0(v1 + 1);
          memcpy(v3,a0,v1);
          *(char *)((int8)v3 + v1) = 0;
          return v3;
        }
        return NULL;
      }
      v4 = &v4[-1];
    } while (a0 != v4);
  }
  return NULL;
}


// Function: sub_df40 @ 0xdf40
bool sub_df40(uint1 *a0) // return-dupe
{
  int4 v1; // eax
  char *v2; // rax
  
  if (!fnmatch(dat_84a40,(char *)a0,0))
    return 1;
  if ((dat_84b10) && (v2 = (char *)sub_de80(a0), v2)) {
    v1 = fnmatch(v2,dat_84a40,0);
    free(v2);
    return v1 == 0;
  }
  return 0;
}


// Function: sub_dfd0 @ 0xdfd0
void sub_dfd0(void) // return-dupe
{
  int8 v1;
  int8 v2; // rax
  int8 v3; // rdx
  
  if (dat_83f40) { // branch-flip
    v3 = (int8)dat_84ba0;
    v2 = (int8)dat_83f50 - dat_83f58 >> 9;
  }
  else {
    if (!dat_84cf8) {
      dat_83f48 = dat_83f58;
      dat_84170 += (int8)dat_83f50 - dat_83f58 >> 9;
      dat_83f50 = (int8)(dat_83f58 + (int8)dat_84ba0 * 0x200);
      sub_dfc0(); // return-dupe, tail-call
      return;
    }
    dat_83f40 = 1;
    dat_84cf8 = '\0';
    sub_c290();
    v3 = (int8)dat_84ba0;
    v2 = (int8)dat_83f50 - dat_83f58 >> 9;
    if (v2 < v3) {
      memset(dat_83f50,0,(v3 - v2) * 0x200);
      dat_83f50 = (void *)((int8)dat_84ba0 * 0x200 + dat_83f58);
      return;
    }
  }
  v1 = dat_83f48;
  dat_84170 += v2;
  dat_83f50 = (int8)(v3 * 0x200 + dat_83f58);
  dat_83f48 = dat_83f58;
  if (dat_83f40 == 1) {
    (*dat_84158)(v1 - dat_83f58); // jump-as-call
    return;
  }
  if (dat_83f40 == 2)
    abort(); // no-return
  if (dat_83f40) {
    dat_83f48 = dat_83f58;
    return;
  }
  sub_dfc0();
  return;
}


// Function: sub_e130 @ 0xe130
int8 sub_e130(void) // early-return x2
{
  if (dat_83f48 != dat_83f50)
    return dat_83f48;
  if (dat_84161)
    return 0;
  sub_dfd0();
  if (dat_83f48 != dat_83f50)
    return dat_83f48;
  dat_84161 = 1;
  return 0;
}


// Function: sub_e190 @ 0xe190
void sub_e190(char *a0)
{
  unsigned long *v1;
  char *v2; // rax
  uint8 v3; // rcx
  unsigned long *v4;
  
  if (dat_84ba4 != 4) {
    v2 = (char *)sub_e130();
    v2[0] = '\0';
    v2[1] = '\0';
    v2[2] = '\0';
    v2[3] = '\0';
    v2[4] = '\0';
    v2[5] = '\0';
    v2[6] = '\0';
    v2[7] = '\0';
    v2[0x1f8] = '\0';
    v2[0x1f9] = '\0';
    v2[0x1fa] = '\0';
    v2[0x1fb] = '\0';
    v2[0x1fc] = '\0';
    v2[0x1fd] = '\0';
    v2[0x1fe] = '\0';
    v2[0x1ff] = '\0';
    v3 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[8] & 0xfffffffffffffff8)) + 0x200U >> 3);
    v4 = (unsigned long *)((uint8)&v2[8] & 0xfffffffffffffff8);
    while (v3) {
      v1 = &v4[1];
      *v4 = 0;
      v3 -= 1;
      v4 = v1;
    }
    __strcpy_chk(v2,a0,100);
    sub_23be0((unsigned long *)0x84848,v2);
    dat_84850 = sub_3e630(dat_84848);
    v2[0x9c] = 'V';
    sub_12c10(dat_84a20,&v2[0x88],0xc);
    sub_12f60((struct_9 *)0x84840,v2,-1);
    sub_d1f0(v2); // tail-call
    return;
  }
  sub_1ca30("GNU.volume.label",0x83f80,a0); // tail-call
  return;
}


// Function: sub_e280 @ 0xe280
void sub_e280(void)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3; // rax
  char v4 [24];
  
  v1 = (char *)sub_43090((int8)dat_81014,v4);
  v2 = strlen(dat_84a40);
  v3 = (char *)sub_4c5b0(v2 + 9 + strlen(v1));
  __sprintf_chk(v3,1,0xffffffffffffffff,"%s %s %s",dat_84a40,"Volume",v1);
  sub_e190(v3);
  free(v3);
  return;
}


// Function: sub_e330 @ 0xe330
void sub_e330(void)
{
  if (dat_84b10) {
    sub_e280(); // tail-call
    return;
  }
  sub_e190(dat_84a40); // tail-call
  return;
}


// Function: sub_e350 @ 0xe350
void sub_e350(int8 a0)
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long *v11;
  unsigned long *v2;
  unsigned int v3;
  unsigned long *v4; // rax
  uint8 v5; // rax
  char *v6;
  uint8 v7; // rcx
  int8 v8; // rcx
  uint4 v9; // edx
  
  v4 = (unsigned long *)sub_e130();
  v6 = *(char **)(a0 + 0x10);
  v5 = strlen(v6);
  if (0x65 <= v5) {
    if (dat_84d50) {
      (*dat_84d50)();
      v6 = *(char **)(a0 + 0x10);
    }
    v6 = (char *)sub_4aa70(v6);
    error(0,0,dcgettext(NULL,"%s: file name too long to be stored in a GNU multivolume header, truncated",5),v6);
    v5 = 100;
  }
  *v4 = 0;
  v4[0x3f] = 0;
  v10 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
  v7 = (uint8)(((int4)v4 - (int4)v10) + 0x200U >> 3);
  while (v7) {
    v11 = &v10[1];
    *v10 = 0;
    v7 -= 1;
    v10 = v11;
  }
  v9 = (uint4)v5;
  v7 = v5 & 0xffffffff;
  v10 = *(unsigned long **)(a0 + 0x10);
  if (8 <= v9) { // branch-flip
    v11 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
    *v4 = *v10;
    *(unsigned long *)((int8)v4 + ((v5 & 0xffffffff) - 8)) = *(unsigned long *)((int8)v10 + ((v5 & 0xffffffff) - 8));
    v8 = (int8)v4 - (int8)v11;
    v5 = (uint8)((int4)v8 + v9 >> 3);
    v10 = (unsigned long *)((int8)v10 - v8);
    while (v5) {
      v2 = &v11[1];
      v1 = &v10[1];
      *v11 = *v10;
      v5 -= 1;
      v10 = v1;
      v11 = v2;
    }
  }
  else if (v5 & 4) { // branch-flip
    *(unsigned int *)v4 = *(unsigned int *)v10;
    *(unsigned int *)((int8)v4 + (v7 - 4)) = *(unsigned int *)((int8)v10 + (v7 - 4));
  }
  else if ((v9) && (*(char *)v4 = *(char *)v10, v5 & 2))
    *(unsigned short *)((int8)v4 + (v7 - 2)) = *(unsigned short *)((int8)v10 + (v7 - 2));
  *(char *)((int8)v4 + 0x9c) = 0x4d;
  sub_12be0(*(uint8 *)(a0 + 0x20),(char *)((int8)v4 + 0x7c),0xc);
  sub_12be0(*(int8 *)(a0 + 0x18) - *(int8 *)(a0 + 0x20),(char *)((int8)v4 + 0x171),0xc);
  v3 = dat_84a54;
  dat_84a54 = 0;
  sub_12f60((struct_9 *)0x84840,v4,-1);
  dat_84a54 = v3;
  sub_d1f0(v4); // tail-call
  return;
}


// Function: sub_e4d0 @ 0xe4d0
void sub_e4d0(int8 a0)
{
  int8 v1; // stack - 0x28
  
  if (dat_84ba4 != 4) {
    sub_e350(a0); // tail-call
    return;
  }
  v1 = *(int8 *)(a0 + 0x18) - *(int8 *)(a0 + 0x20);
  sub_1ca30("GNU.volume.filename",0x83f80,*(unsigned long *)(a0 + 0x10));
  sub_1ca30("GNU.volume.size",0x83f80,a0 + 0x20);
  sub_1ca30("GNU.volume.offset",0x83f80,&v1);
  return;
}


// Function: sub_e580 @ 0xe580
void sub_e580(int8 a0)
{
  uint4 v1;
  uint8 v2; // rax
  int8 v3; // rax
  void *v4; // rax
  uint8 v5; // rbp
  void *v6; // r12
  void *v7;
  
  v2 = sub_c490();
  if (dat_84b98 != v2) { // branch-flip
    if (!dat_84b10)
      sub_d6d0(v2); // return-dupe, no-return
    dat_841a0 = (float8)(int8)v2 + dat_841a0;
    if (v2) {
      dat_83f30 += 1;
      v3 = sub_bed0(v2);
      if (v2 & 0x1ff) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"write did not end on a block boundary",5));
        dat_84d58 = 2;
        sub_d6d0(v2);
      }
      if ((int8)v2 <= -1) {
        v1 = *__errno_location();
        if ((2 <= (uint4)(v1 - 5U)) && (v1 != 0x1c)) {
          sub_d6d0(v2);
        }
      }
      goto label_e647;
    }
  }
  else {
    dat_841a0 = (float8)(int8)dat_84b98 + dat_841a0;
    if (dat_84b98) {
      dat_83f30 += 1;
      return;
    }
  }
  if (!dat_84b98)
    return;
  v2 = 0;
  v3 = sub_bed0(0);
label_e647:
  if (!sub_da00(1))
    return;
  v5 = a0 - v2;
  sub_2f6f0((unsigned long *)0x83f80);
  sub_c240();
  dat_841a8 = dat_841a8 + dat_841a0;
  dat_841a0 = 0.0;
  v6 = (void *)(dat_83f58 + v2);
  dat_84178 = (uint4)(dat_84178 == 0);
  sub_c980();
  dat_84138 = 1;
  if (dat_84a40)
    sub_e280();
  if (v3) { // branch-flip
    sub_e4d0(v3);
    sub_12da0(1,(unsigned long *)0x83f80,(unsigned long *)sub_e130());
    sub_2f6f0((unsigned long *)0x83f80);
    sub_cb00(v3);
  }
  else {
    sub_12da0(1,(unsigned long *)0x83f80,(unsigned long *)sub_e130());
    sub_2f6f0((unsigned long *)0x83f80);
  }
  v4 = (void *)sub_e130();
  sub_c030(v3,(int8)v4 - dat_83f58 >> 9);
  v2 = sub_d2b0(v4);
  dat_84138 = 0;
  v7 = v6;
  if (v2 < v5) {
    do {
      v6 = (void *)((int8)v7 + v2);
      memcpy(v4,v7,v2);
      v5 -= v2;
      sub_d1f0((int8)v4 + (v2 - 1 & 0xfffffffffffffe00));
      v4 = (void *)sub_e130();
      v2 = sub_d2b0(v4);
      v7 = v6;
    } while (v2 < v5);
  }
  memcpy(v4,v6,v5);
  memset((void *)((int8)v4 + v5),0,v2 - v5);
  sub_d1f0((int8)v4 + (v5 - 1 & 0xfffffffffffffe00));
  sub_e130(); // tail-call
  return;
}


// Function: sub_e850 @ 0xe850
void sub_e850(int8 a0)
{
  dat_84158 = sub_d720;
  sub_e580(a0);
  dat_84158 = sub_e850;
  return;
}


// Function: sub_e880 @ 0xe880
unsigned int sub_e880(void) // return-dupe
{
  code *v1;
  int8 v10;
  char v11 [24];
  char v12 [32];
  char v13 [32];
  int8 v14; // stack - 0x90
  char v2;
  unsigned int v3;
  int4 v4; // eax
  uint8 v5;
  char *v6;
  char *v7;
  char *v8; // rax
  char *v9; // rax
  
  if (3 <= dat_84ba8) { // branch-flip
    v3 = sub_da00((dat_84ba8 == 8) * '\x02');
    v2 = (char)v3;
  }
  else {
    v3 = sub_da00((dat_84ba8 != 0) * '\x02');
    v2 = (char)v3;
  }
  if (!v2)
    return 1;
  while( true ) {
    if (0x40000000 <= dat_84a34) // branch-flip
      v5 = sub_33da0(dat_84a34 + -0x40000000,dat_83f58,dat_84b98);
    else {
      v5 = sub_4af10(dat_84a34,dat_83f58,dat_84b98);
    }
    if (v5 != 0xffffffffffffffff) break;
    sub_d300();
  }
  if (dat_84b98 != v5) { // branch-flip
    sub_d390(v5);
    v14 = sub_e130();
  }
  else {
    v14 = sub_e130();
  }
  if (!v14) goto label_eb23;
  v2 = *(char *)(v14 + 0x9c);
  if (v2 != 'V') { // branch-flip
    v10 = v14;
    if (v2 != 'g') goto label_ebac;
    sub_2f630((unsigned long *)0x83f80);
    if (sub_219e0(&v14,(unsigned long *)0x83f80,2) != 2) {
label_eb23:
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
      return 0;
    }
    sub_1c8c0(0x83f80);
    sub_2f6f0((unsigned long *)0x83f80);
    v4 = sub_219e0(&v14,(unsigned long *)0x83f80,0);
    if (v4 != 1) { // branch-flip
      v10 = v14;
      if (v4 != 5) goto label_eb23;
    }
    else {
      sub_d1f0(v14);
      v10 = v14;
    }
  }
  else {
    if (!sub_d230())
      return 0;
    sub_2f6f0((unsigned long *)0x83f80);
    sub_23c20((unsigned long *)0x83f18,dat_84408,100);
    sub_d1f0(v14);
    v14 = sub_e130();
    v2 = *(char *)(v14 + 0x9c);
    v10 = v14;
label_ebac:
    if (v2 == 'M') {
      if (!sub_d230())
        return 0;
      sub_2f6f0((unsigned long *)0x83f80);
      sub_23c20((unsigned long *)0x83f10,dat_84408,100);
      dat_83f08 = sub_22050((uint1 *)(dat_84408 + 0x7c),0xc);
      dat_83f00 = sub_22050((uint1 *)(dat_84408 + 0x171),0xc);
      v10 = v14;
    }
  }
  v14 = v10;
  v10 = dat_84148;
  v6 = dat_83f10;
  if (dat_84148) {
    if (!dat_83f10) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = *(char **)(dat_84148 + 0x10);
label_eae1:
      v6 = (char *)sub_4ac50(v7);
      error(0,0,dcgettext(NULL,"%s is not continued on this volume",5),v6);
      return 0;
    }
    v7 = *(char **)(dat_84148 + 0x10);
    v4 = strcmp(dat_83f10,v7);
    v1 = dat_84d50;
    if (v4) {
      if ((((dat_84ba4 & 0xfffffffb) != 2) || (strlen(v7) <= 99)) || (strncmp(v6,v7,100))) {
        if (v1) {
          (*v1)();
          v7 = *(char **)(dat_84148 + 0x10);
        }
        goto label_eae1;
      }
      if (v1) {
        (*v1)();
        v7 = *(char **)(dat_84148 + 0x10);
      }
      v6 = (char *)sub_4ac50(v7);
      error(0,0,dcgettext(NULL,"%s is possibly continued on this volume: header contains truncated name",5),v6);
      v10 = dat_84148;
    }
    if (CARRY8(dat_83f00,dat_83f08) || dat_83f00 + dat_83f08 != *(int8 *)(v10 + 0x18)) {
      if (dat_84d50)
        (*dat_84d50)();
      v6 = (char *)sub_43090(dat_83f00,v11);
      v7 = (char *)sub_43090(dat_83f08,v12);
      v8 = (char *)sub_43090(*(uint8 *)(dat_84148 + 0x18),v13);
      v9 = (char *)sub_4ac50(dat_83f10);
      error(0,0,dcgettext(NULL,"%s is the wrong size (%s != %s + %s)",5),v9,v8,v7,v6);
      return 0;
    }
    if (*(int8 *)(v10 + 0x18) - *(int8 *)(v10 + 0x20) != dat_83f00) {
      if (dat_84d50)
        (*dat_84d50)();
      v6 = (char *)sub_43090(dat_83f00,v11);
      v7 = (char *)sub_43090(*(uint8 *)(dat_84148 + 0x20),v12);
      v8 = (char *)sub_43090(*(uint8 *)(dat_84148 + 0x18),v13);
      error(0,0,dcgettext(NULL,"This volume is out of sequence (%s - %s != %s)",5),v8,v7,v6);
      return 0;
    }
  }
  sub_c240();
  return v3;
}


// Function: sub_edb0 @ 0xedb0
uint8 sub_edb0(char *a0)
{
  char *v1;
  char v2;
  int8 v3; // rax
  uint8 v4;
  uint4 *v5;
  int8 v6;
  void *v7;
  void *v8;
  
  v2 = dat_84160;
  dat_84160 = 1;
  if (!a0)
    a0 = &v6;
  dat_83f50 = dat_83f58;
  v3 = sub_e130();
  v8 = dat_83f58;
  *a0 = v3 == 0;
  v7 = dat_83f50;
  dat_84160 = v2;
  if (dat_83f58 != dat_83f50) {
    v1 = (char *)((int8)dat_83f58 + 0x101);
    if ((strcmp(v1,"ustar")) && (strcmp(v1,"ustar  ")))
      v7 = v8;
    else {
      v4 = sub_214d0(v8,1);
      if ((int4)v4 == 1)
        return v4;
      v7 = dat_83f58;
    }
  }
  v8 = (void *)0x60127;
  v4 = 2;
  v5 = (uint4 *)0x7f130;
  while( true ) {
    if (!memcmp(v7,v8,v4))
      return (uint8)*v5;
    if (&v5[6] == (uint4 *)0x7f1f0) break;
    v4 = *(uint8 *)&v5[8];
    v8 = *(void **)&v5[10];
    v5 = &v5[6];
  }
  return 0;
}


// Function: sub_eed0 @ 0xeed0
uint8 sub_eed0(void)
{
  unsigned long *v1;
  int4 v2; // eax
  uint8 v3; // rax
  char v4; // stack - 0x11
  char *v5;
  
  v1 = dat_84838;
  v5 = (char *)*dat_84838;
  if (dat_84d70) { // branch-flip
label_ef39:
    v3 = CONCAT44(dat_4,open(v5,0,0x1b6));
  }
  else {
    dat_84d78 = strchr(v5,0x3a);
    v5 = (char *)*v1;
    if (((!dat_84d78) || (dat_84d78 <= v5)) || (v3 = (int8)dat_84d78 - (int8)v5, memchr(v5,0x2f,v3))) goto label_ef39;
    v3 = sub_338d0(v5,0,0x40000000,dat_84ab0);
  }
  dat_84a34 = (uint4)v3;
  if (dat_84a34 == 0xffffffff)
    return v3;
  if (!dat_84b10) {
    if (!dat_84b70) {
      v2 = sub_edb0(&v4);
      if (v2) { // branch-flip
        if (v2 == 1) {
          if (v4) {
            if (dat_84d50)
              (*dat_84d50)();
            error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
            dat_84d58 = 2;
          }
          return (uint8)dat_84a34; // return-dupe
        }
        dat_83f70 = v2;
      }
      else {
        if (v4) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
          dat_84d58 = 2;
        }
        sub_2a260(*dat_84838,0);
        if (!dat_84b70) {
          return (uint8)dat_84a34;
        }
      }
      v3 = (uint8)dat_84a34;
    }
    v2 = (int4)v3;
    if (0x40000000 <= v2) // branch-flip
      sub_33d60(v2 + -0x40000000);
    else {
      close(v2);
    }
    dat_84161 = 0;
    dat_84168 = sub_2b310();
    dat_84160 = 1;
    v3 = (uint8)dat_84a34;
  }
  dat_83f50 = dat_83f58;
  dat_83f38 = 0;
  return v3;
}


// Function: sub_f0f0 @ 0xf0f0
void sub_f0f0(int4 a0)
{
  int4 *v1; // rax
  uint8 v2; // rax
  char *v3;
  char v4; // stack - 0x21
  int4 v5;
  unsigned long *v6;
  bool v7; // zf
  uint4 v8; // stack - 0x30
  
  if (!dat_84b98) {
    if (dat_84d50)
      (*dat_84d50)();
    v3 = "Invalid value for record_size";
    goto label_f3ab;
  }
  if (!dat_84830) {
    if (dat_84d50)
      (*dat_84d50)();
    v3 = "No archive name given";
    goto label_f3ab;
  }
  sub_2f6f0((unsigned long *)0x84840);
  dat_84178 = 0;
  sub_c980();
  if (a0 != 2) { // branch-flip
    dat_83f40 = a0;
    sub_ca80(a0);
    v6 = dat_84838;
    dat_84160 = dat_84ab9;
    dat_83f38 = 0;
    if (dat_84b70) { // branch-flip
      if (a0 != 1) { // branch-flip
        if (!a0) {
          dat_84168 = sub_2b310();
          dat_84160 = 1;
          dat_83f50 = dat_83f58;
        }
      }
      else {
        dat_84168 = sub_2af40();
        if ((!dat_84820) && (v3 = (char *)*dat_84838, !strcmp(v3,"-")))
          dat_83f28 = stderr;
      }
    }
    else {
      v3 = (char *)*dat_84838;
      if (!strcmp(v3,"-")) goto label_f2d9;
      if (a0 == 1) {
        v7 = dat_84b84 != '\0';
        if (v7) {
          sub_24710(v3,1);
          v3 = (char *)*dat_84838;
          v6 = dat_84838;
        }
        if (dat_84a50) { // branch-flip
          if (!dat_84d70) {
            dat_84d78 = strchr(v3,0x3a);
            v3 = (char *)*v6;
            if (((dat_84d78) && (v3 < dat_84d78)) && (v2 = (int8)dat_84d78 - (int8)v3, !memchr(v3,0x2f,v2))) {
              dat_84a34 = sub_338d0(v3,0x42,0x40000000,dat_84ab0);
              goto label_f60a;
            }
          }
          dat_84a34 = open(v3,0x42,0x1b6);
        }
        else {
          if (!dat_84d70) {
            dat_84d78 = strchr(v3,0x3a);
            v3 = (char *)*v6;
            if (((dat_84d78) && (v3 < dat_84d78)) && (v2 = (int8)dat_84d78 - (int8)v3, !memchr(v3,0x2f,v2))) {
              dat_84a34 = sub_338d0(v3,0x41,0x40000000,dat_84ab0);
              goto label_f60a;
            }
          }
          dat_84a34 = creat(v3,0x1b6);
        }
label_f60a:
        if (0 <= dat_84a34) {
          if (0x40000000 <= dat_84a34) {
            sub_2abd0();
            sub_2ab80();
            dat_83f30 = 0;
            return;
          }
          if (sub_2ab10()) goto label_f4a8;
        }
        v1 = __errno_location();
        v5 = *v1;
        if (v7)
          sub_245d0();
        goto label_f218;
      }
      if (!a0) {
        dat_84a34 = sub_eed0();
        if (dat_84a34 < 0) goto label_f210;
        sub_c8d0();
      }
    }
label_f1aa:
    if (0 <= dat_84a34) {
      if (dat_84a34 <= 0x3fffffff) {
label_f200:
        if (!sub_2ab10()) goto label_f210;
      }
      sub_2abd0();
      sub_2ab80();
      if (!a0) {
        sub_e130(); // tail-call
        return;
      }
label_f1cd:
      if ((uint4)(a0 - 1U) <= 1) {
        dat_83f30 = 0;
        return;
      }
      return;
    }
  }
  else {
    dat_83f40 = 0;
    sub_ca80(0);
    v6 = dat_84838;
    dat_84160 = dat_84ab9;
    dat_83f38 = 0;
    if (dat_84b70)
      abort(); // no-return
    v3 = (char *)*dat_84838;
    if (strcmp(v3,"-")) {
      if (dat_84d70) { // branch-flip
label_f40f:
        dat_84a34 = open(v3,0x42,0x1b6);
      }
      else {
        dat_84d78 = strchr(v3,0x3a);
        v3 = (char *)*v6;
        if (((!dat_84d78) || (dat_84d78 <= v3)) || (v2 = (int8)dat_84d78 - (int8)v3, memchr(v3,0x2f,v2))) goto label_f40f;
        dat_84a34 = sub_338d0(v3,0x42,0x40000000,dat_84ab0);
      }
      if (2 <= (uint4)sub_edb0(NULL)) {
        if (dat_84d50)
          (*dat_84d50)();
        v3 = "Cannot update compressed archives";
label_f3ab:
        error(0,0,dcgettext(NULL,v3,5));
        sub_163d0(); // no-return
      }
      goto label_f1aa;
    }
label_f2d9:
    dat_84160 = 1;
    if (dat_84a50) {
      if (dat_84d50)
        (*dat_84d50)();
      v3 = "Cannot verify stdin/stdout archive";
      goto label_f3ab;
    }
    if (a0 != 1) {
      if (a0 != 2) {
        if (!a0) {
          dat_84a34 = 0;
          v8 = sub_edb0(&v4);
          if (2 <= v8) {
            if (dat_84d50)
              (*dat_84d50)();
            v3 = (char *)sub_bf70(v8);
            error(0,0,dcgettext(NULL,"Archive is compressed. Use %s option",5),v3);
            sub_163d0(); // no-return
          }
          if (v4) {
            if (dat_84d50)
              (*dat_84d50)();
            error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
            dat_84d58 = 2;
          }
        }
        goto label_f1aa;
      }
      dat_83f50 = dat_83f58;
      dat_84a34 = 0;
      dat_83f20 = 1;
      if (!dat_84820) goto label_f48d;
      goto label_f200;
    }
    dat_84a34 = 1;
    if (!dat_84820) {
label_f48d:
      dat_83f28 = stderr;
    }
    if (sub_2ab10()) {
label_f4a8:
      sub_2abd0();
      sub_2ab80();
      goto label_f1cd;
    }
  }
label_f210:
  v1 = __errno_location();
  v5 = *v1;
label_f218:
  *v1 = v5;
  sub_327e0((char *)*dat_84838); // no-return
}


// Function: sub_f760 @ 0xf760
void sub_f760(void)
{
  char v1; // al
  char *v2; // rax
  char *v3; // rax
  char v4 [360];
  char v5 [80];
  
  if (!dat_83f18) {
    v2 = (char *)sub_e130();
    if (v2) {
      if (v2[0x9c] != 'V') { // branch-flip
        if (v2[0x9c] == 'g') {
          sub_2f630(v4);
          sub_1cac0(v5,v2,sub_219b0(&v2[0x7c],0xc));
          sub_1c8c0(v4);
          sub_2f6f0(v4);
        }
      }
      else {
        sub_23c20((unsigned long *)0x83f18,v2,100);
      }
      if (dat_83f18) goto label_f789;
    }
    if (dat_84d50)
      (*dat_84d50)();
    v2 = (char *)sub_4ac50(dat_84a40);
    error(0,0,dcgettext(NULL,"Archive not labeled to match %s",5),v2);
    sub_163d0(); // no-return
  }
label_f789:
  v1 = sub_df40(dat_83f18);
  if (v1)
    return;
  if (dat_84d50)
    (*dat_84d50)();
  v2 = (char *)sub_4ac40(1,dat_84a40);
  v3 = (char *)sub_4ac40(0,dat_83f18);
  error(0,0,dcgettext(NULL,"Volume %s does not match %s",5),v3,v2);
  sub_163d0(); // no-return
}


// Function: sub_f900 @ 0xf900
void sub_f900(void)
{
  int4 v1; // eax
  
  if ((dat_84cf8) || (dat_83f40 == 1)) {
    do {
      sub_dfd0();
    } while (dat_83f58 < dat_83f48);
  }
  sub_cd50();
  if (dat_84a50)
    sub_118d0();
  if (0x40000000 <= dat_84a34) // branch-flip
    v1 = sub_33d60(dat_84a34 + -0x40000000);
  else {
    v1 = close(dat_84a34);
  }
  if (v1)
    sub_32680((char *)*dat_84828);
  sub_2aca0(dat_84168,(uint1)dat_84161);
  sub_2f6f0((unsigned long *)0x84840);
  free(dat_84190);
  free(dat_84198);
  sub_bfc0(NULL); // tail-call
  return;
}


// Function: sub_f9e0 @ 0xf9e0
void sub_f9e0(void) // return-dupe
{
  uint8 v1;
  int4 v2;
  
  sub_108d0(0);
  dat_84164 = 0;
  if ((dat_83f20) && (dat_84170)) {
    dat_84a34 = 1;
    v1 = sub_2af00();
    dat_84a34 = 0;
    if (dat_84b98 != v1)
      sub_d6d0(v1); // no-return
    v2 = 0;
    goto label_fa19;
  }
  while( true ) {
    v1 = dat_84b98;
    v2 = dat_84a34;
label_fa19:
    if (0x40000000 <= v2) // branch-flip
      v1 = sub_33da0(v2 + -0x40000000);
    else {
      v1 = sub_4af10(v2,dat_83f58,v1);
    }
    if (dat_84b98 == v1) {
      dat_83f38 += 1;
      return;
    }
    if (!v1) break;
    if (v1 != 0xffffffffffffffff) {
      sub_d390(v1); // tail-call, return-dupe
      return;
    }
    if ((*__errno_location() == 0x1c) && (dat_84b10)) goto label_fa79;
    sub_d300();
  }
  if (!dat_84b10) {
    sub_d390(v1);
    return;
  }
label_fa79:
  do {
  } while (!sub_e880());
  if (dat_83f48 != dat_83f50)
    return;
  sub_dfd0(); // tail-call
  return;
}


// Function: sub_fb20 @ 0xfb20
void sub_fb20(void)
{
  dat_84150 = sub_d770;
  sub_f9e0();
  dat_84150 = sub_fb20;
  return;
}


// Function: sub_fb50 @ 0xfb50
void sub_fb50(void)
{
  (*dat_84158)(dat_84b98); // jump-as-call
  return;
}


// Function: sub_fb70 @ 0xfb70
void sub_fb70(uint4 a0)
{
  dat_84150 = sub_fb20;
  dat_84158 = sub_e850;
  sub_f0f0(a0);
  if (a0 != 1) { // branch-flip
    if ((!(a0 & 0xfffffffd)) && (dat_84a40)) {
      sub_f760();
      sub_c070(); // tail-call
      return;
    }
  }
  else {
    dat_83f30 = 0;
    if (dat_84a40) {
      sub_e330();
      sub_c070(); // tail-call
      return;
    }
  }
  sub_c070(); // tail-call
  return;
}


// Function: sub_fbf0 @ 0xfbf0
char * sub_fbf0(char *a0)
{
  char v1;
  char *v2; // rax
  uint8 v3; // rax
  
  v2 = (char *)sub_4c7a0(a0);
  v1 = *v2;
  if ((v1 == '\"') || (v1 == '\'')) {
    v3 = strlen(v2);
    if (v2[v3 - 1] == v1) {
      memmove(v2,&v2[1],v3 - 2);
      v2[v3 - 2] = '\0';
      sub_23c70(v2);
      return v2;
    }
  }
  sub_23c70(v2);
  return v2;
}


// Function: sub_fc60 @ 0xfc60
void sub_fc60(unsigned int a0)
{
  int8 v1; // rax
  
  v1 = sub_4c760(0x18);
  if (dat_84268) {
    *dat_84268 = v1;
    *(unsigned int *)(v1 + 8) = a0;
    dat_84268 = (int8 *)v1;
    return;
  }
  *(unsigned int *)(v1 + 8) = a0;
  dat_84270 = v1;
  dat_84268 = (int8 *)v1;
  return;
}


// Function: sub_fcb0 @ 0xfcb0
uint8 sub_fcb0(FILE *a0) // early-return
{
  int4 v1; // eax
  char *v2; // rax
  int8 v3; // rax
  char v4 [2];
  uint2 v5; // stack - 0x16
  
  v1 = fileno(a0);
  if (((ioctl(v1,0x5413,v4)) || (v3 = (uint8)v5, !v5)) && ((v2 = getenv("COLUMNS"), !v2 || (v3 = strtol(v2,NULL,10), v3 <= 0))))
    return 0x50;
  return v3;
}


// Function: sub_fdf0 @ 0xfdf0
uint8 sub_fdf0(FILE *a0,uint8 a1,char *a2,char a3,unsigned int a4)
{
  char v1;
  timeval v10; // stack - 0x178
  int8 v11; // stack - 0x148
  unsigned long v12 [4];
  unsigned long *v13;
  int4 v14; // r8d
  char *v15; // stack - 0x1a0
  int8 v16; // stack - 0x140
  unsigned long v17; // stack - 0x110
  char *v2; // rax
  char *v3; // rax
  tm *v4; // rax
  int8 v5; // rax
  unsigned long v6; // rax
  uint8 v7; // rax
  char v8 [24];
  char *v9; // stack - 0x180
  
  if (a3) { // branch-flip
    v15 = dcgettext(NULL,"write",5);
    v2 = (char *)sub_43090(a4,v8);
    if (!a2)
      a2 = dcgettext(NULL,"Write checkpoint %u",5);
  }
  else {
    v15 = dcgettext(NULL,"read",5);
    v2 = (char *)sub_43090(a4,v8);
    if (!a2)
      a2 = dcgettext(NULL,"Read checkpoint %u",5);
  }
  v1 = *a2;
  v9 = a2;
  if (v1) {
    do {
      if (v1 != '%') { // branch-flip
        a1 += 1;
        v9 = a2;
        fputc_unlocked((int4)v1,a0);
        if (*v9 == '\r') {
          dat_841d8 = 1;
          a1 = 0;
        }
      }
      else {
        v9 = &a2[1];
        v1 = a2[1];
        if (v1 != '{') { // branch-flip
          if (v1 != '*') { // branch-flip
            switch(v1) {
              case 0x54:
                sub_cd50();
                a1 += (int8)(int4)sub_ce80(a0,(unsigned long *)0x81140,0x2c,0);
                break;
              default:
                goto label_ff80;
              case 99:
label_10028:
                a1 += (int8)(int4)sub_fdf0(a0,a1,"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r",(uint1)a3,a4);
                break;
              case 100:
label_fff8:
                sub_cd50();
                a1 += (int8)__fprintf_chk(a0,1,"%.0f");
                break;
              case 0x73:
label_ffd0:
                fputs_unlocked(v15,a0);
                a1 += strlen(v15);
                break;
              case 0x74:
                v3 = "%c";
label_100a7:
                gettimeofday(&v10,NULL);
                v4 = localtime(&v10);
                v14 = v10._8_4_ * 1000;
                a1 += sub_41100(a0,v3,v4,0,v14);
                break;
              case 0x75:
label_ffb0:
                fputs_unlocked(v2,a0);
                a1 += strlen(v2);
              
            }
          }
          else {
            v7 = sub_fcb0(a0);
label_101de:
            if (a1 < v7) {
              do {
                a1 += 1;
                fputc_unlocked(0x20,a0);
              } while (v7 != a1);
            }
          }
        }
        else {
          v3 = (char *)sub_fd40(v9,&v9);
          if (v3) {
            if (*v9 == '*') {
              v7 = strtol(v3,NULL,10);
              goto label_101de;
            }
            switch(*v9) {
              case 0x54:
                sub_cd50();
                v17 = 0x6079c;
                v5 = sub_382c0(v3,&v11,0x4644);
                if ((int4)v5) { // branch-flip
                  if (dat_84d50)
                    (*dat_84d50)();
                  v6 = sub_354b0(&v11);
                  error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),v3,v6);
                  v13 = (unsigned long *)0x81140;
                  dat_84d58 = 2;
                }
                else {
                  if (v11) {
                    v5 = 0;
                    v13 = v12;
                    do {
                      v13[v5] = *(unsigned long *)(v16 + v5 * 8);
                      v5 += 1;
                    } while (v5 != v11);
                    if ((int4)v5 == 3) goto label_101ab;
                  }
                  v13 = v12;
                  v5 = (int8)(int4)v5;
                  do {
                    v13[v5] = 0;
                    v5 += 1;
                  } while ((int4)v5 != 3);
                }
label_101ab:
                a1 += (int8)(int4)sub_ce80(a0,v13,0x2c,0);
                sub_35420(&v11);
                goto label_fee8;
              case 99:
                goto label_10028;
              case 100:
                goto label_fff8;
              case 0x73:
                goto label_ffd0;
              case 0x74:
                goto label_100a7;
              case 0x75:
                goto label_ffb0;
              
            }
          }
label_ff80:
          a1 += 2;
          fputc_unlocked(0x25,a0);
          fputc_unlocked((int4)*v9,a0);
        }
      }
label_fee8:
      a2 = &v9[1];
      v1 = v9[1];
      v9 = a2;
    } while (v1);
  }
  fflush_unlocked(a0);
  return a1 & 0xffffffff;
}


// Function: sub_10270 @ 0x10270
void sub_10270(char a0)
{
  unsigned long *v1;
  int4 v2; // eax
  FILE *v3;
  int8 v4;
  
  v1 = dat_84270;
  v3 = dat_841d0;
  while (v1) {
    dat_841d0 = v3;
    switch(*(unsigned int *)&v1[1]) {
      case 0:
        fputc_unlocked(0x2e,dat_83f28);
        fflush_unlocked(dat_83f28);
        break;
      case 1:
        if ((v3) || (v3 = fopen("/dev/tty","w"), dat_841d0 = v3, v3)) {
          fputc_unlocked(7,v3);
          fflush_unlocked(dat_841d0);
        }
        break;
      case 2:
        v2 = __fprintf_chk(stderr,1,"%s: ",dat_84dd0);
        sub_fdf0(stderr,(int8)v2,v1[2],(uint1)a0,dat_84278);
        fputc_unlocked(10,stderr);
        break;
      case 3:
        if ((v3) || (v3 = fopen("/dev/tty","w"), dat_841d0 = v3, v3))
          sub_fdf0(v3,0,v1[2],(uint1)a0,dat_84278);
        break;
      case 4:
        sleep(*(uint4 *)&v1[2]);
        break;
      case 5:
        sub_2bb40(v1[2],*dat_84828,dat_84278);
        break;
      case 6:
        sub_cd50();
        sub_d150();
        break;
      case 7:
        sigwait((void *)0x841e0,&v4);
      
    }
    v1 = (unsigned long *)*v1;
    v3 = dat_841d0;
  }
  dat_841d0 = v3;
  return;
}


// Function: sub_10490 @ 0x10490
void sub_10490(char *a0)
{
  int4 v1; // eax
  int8 v2; // rax
  uint8 v3; // rax
  char *v4; // stack - 0x28
  char *v5; // rsi
  unsigned long v6; // rdi
  
  if (!dat_84260) {
    sigemptyset((sigset_t *)0x841e0);
    dat_84260 = 1;
  }
  if ((!strcmp(a0,".")) || (!strcmp(a0,"dot"))) {
    sub_fc60(0); // tail-call
    return;
  }
  if (!strcmp(a0,"bell")) {
    sub_fc60(1); // tail-call
    return;
  }
  if (strcmp(a0,"echo")) {
    v6 = 2;
    if (strncmp(a0,"echo=",5)) {
      if (strncmp(a0,"exec=",5)) {
        if (!strncmp(a0,"ttyout=",7)) {
          v2 = sub_fc60(3);
          *(unsigned long *)(v2 + 0x10) = sub_fbf0(&a0[7]);
          return;
        }
        if (strncmp(a0,"sleep=",6)) { // branch-flip
          if (!strcmp(a0,"totals")) {
            sub_fc60(6); // tail-call
            return;
          }
          if (!strncmp(a0,"wait=",5)) {
            v2 = sub_fc60(7);
            v1 = sub_2c8a0(&a0[5]);
            *(int4 *)(v2 + 0x10) = v1;
            sigaddset((sigset_t *)0x841e0,v1); // tail-call
            return;
          }
          if (dat_84d50)
            (*dat_84d50)();
          v5 = "%s: unknown checkpoint action";
        }
        else {
          v3 = strtoul(&a0[6],&v4,10);
          if (!*v4) {
            *(uint8 *)(sub_fc60(4) + 0x10) = v3;
            return;
          }
          if (dat_84d50)
            (*dat_84d50)();
          v5 = "%s: not a valid timeout";
        }
        error(0,0,dcgettext(NULL,v5,5),a0);
        sub_163d0(); // no-return
      }
      v6 = 5;
    }
    v2 = sub_fc60(v6);
    *(unsigned long *)(v2 + 0x10) = sub_fbf0(&a0[5]);
    return;
  }
  sub_fc60(2); // tail-call
  return;
}


// Function: sub_107a0 @ 0x107a0
void sub_107a0(void) // return-dupe
{
  if (!dat_84260) {
    if (!dat_84b78)
      return;
    if (dat_84270)
      return;
    sub_10490("echo");
  }
  if (dat_84260 != 1)
    return;
  sigprocmask(0,(sigset_t *)0x841e0,NULL);
  if (dat_84b78) {
    dat_84260 = 2;
    return;
  }
  dat_84260 = 2;
  dat_84b78 = 10;
  return;
}


// Function: sub_10820 @ 0x10820
void sub_10820(void) // return-dupe x2
{
  int8 v1;
  int8 *v2;
  FILE *v3;
  
  v2 = dat_84270;
  if (!dat_84270)
    return;
  do {
    while (((v3 = dat_841d0, (int4)v2[1] != 3 || (!dat_841d0)) || (!dat_841d8))) {
      v2 = (int8 *)*v2;
      if (!v2)
        return;
    }
    v1 = sub_fcb0(dat_841d0);
    if (v1) {
      for (v1 = v1 + -2; fputc_unlocked(0x20,v3), v3 = dat_841d0, v1 != -1; v1 = v1 + -1) {
      }
    }
    fputc_unlocked(0xd,v3);
    fflush_unlocked(dat_841d0);
    v2 = (int8 *)*v2;
  } while (v2);
  return;
}


// Function: sub_108d0 @ 0x108d0
void sub_108d0(char a0)
{
  if (dat_84b78) {
    dat_84278 += 1;
    if (!(dat_84278 % dat_84b78)) {
      sub_10270((uint1)a0); // tail-call
      return;
    }
  }
  return;
}


// Function: sub_10910 @ 0x10910
void sub_10910(void)
{
  if (!dat_84b78)
    return;
  sub_10820();
  if (dat_841d0) {
    fclose(dat_841d0); // tail-call
    return;
  }
  return;
}


// Function: sub_10950 @ 0x10950
unsigned long sub_10950(void)
{
  return 1;
}


// Function: sub_10960 @ 0x10960
int4 sub_10960(char *a0,char *a1)
{
  uint1 v1;
  char v2;
  int4 v3; // eax
  uint8 v4; // rax
  int8 v5; // rax
  
  do {
    v1 = *a0 + 0xbc;
    v4 = 1L << (v1 & 0x3f);
    do {
      do {
        if (!*a0)
          return (int4)*a1;
      } while (0x15 < v1);
      if (v4 & 0x114000) {
        return (int4)*a1;
      }
      if (v4 & 0x200400) {
        v2 = *a1;
        if (!strchr("YN",(int4)v2))
          return 1;
        v3 = strcmp(&a0[1],&a1[1]);
        goto label_10a10;
      }
    } while (!(v4 & 1));
    v3 = strcmp(a0,a1);
label_10a10:
    if (v3)
      return 1;
    v5 = strlen(a0) + 1;
    a0 = &a0[v5];
    a1 = &a1[v5];
  } while( true );
}


// Function: sub_10af0 @ 0x10af0
void sub_10af0(void) // return-dupe
{
  char v1 [8];
  
  dat_84288 = sub_25000(v1,dat_84b98);
  if (!dat_84b28)
    return;
  sub_1fd80();
  return;
}


// Function: sub_10b50 @ 0x10b50
void sub_10b50(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,int8 a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char *v1;
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
  unsigned int v3; // stack - 0xd8
  char v4 [16];
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
    v1 = *(char **)(a8 + 8);
    __fprintf_chk(dat_83f28,1,"%s: ",(char *)sub_23bb0(1,v1));
    v6 = &Stack0000000000000008;
    v3 = 0x10;
    v7 = v4;
    v5 = 0x30;
    __vfprintf_chk(dat_83f28,1,a9,&v3);
    __fprintf_chk(dat_83f28,1,"\n");
  }
  sub_2f7d0(1);
  return;
}


// Function: sub_10c80 @ 0x10c80
unsigned long sub_10c80(uint8 a0,void *a1) // return-dupe
{
  uint8 v1; // rax
  
  v1 = sub_24960(dat_84290,dat_84288,a0);
  if (a0 == v1) {
    if (!memcmp(a1,dat_84288,a0))
      return 1;
    sub_10b50(0x84840,dcgettext(NULL,"Contents differ",5));
    return 0;
  }
  if (v1 != 0xffffffffffffffff) {
    sub_10b50(0x84840,dcngettext(NULL,"Could only read %lu of %lu byte","Could only read %lu of %lu bytes",a0,5),v1,a0);
    return 0;
  }
  sub_32820(dat_84848);
  sub_10b50(0x84840,0);
  return 0;
}


// Function: sub_10d70 @ 0x10d70
unsigned long sub_10d70(char *a0,stat *a1)
{
  if (!sub_246f0(a0,a1))
    return 1;
  if (*__errno_location() != 2) // branch-flip
    sub_32d30(a0);
  else {
    sub_32d50(a0);
  }
  sub_10b50(0x84840,0);
  return 0;
}


// Function: sub_10dd0 @ 0x10dd0
void sub_10dd0(void)
{
  char *v1;
  int4 v2; // eax
  int8 v3; // rax
  char v4 [24];
  char *v5; // rsi
  char v6 [16];
  uint4 v7; // stack - 0xb0
  int8 v8; // stack - 0x98
  
  v1 = dat_84848;
  v2 = sub_10d70(dat_84848,v4);
  if (v2) {
    if ((v7 & 0xf000) != 0x8000) // branch-flip
      v5 = "File type differs";
    else {
      if ((v7 ^ dat_848b0) & 0xfff)
        sub_10b50(0x84840,dcgettext(NULL,"Mode differs",5));
      if (!sub_2ae90(v4,0x84898))
        sub_10b50(0x84840,dcgettext(NULL,"Uid differs",5));
      if (!sub_2aea0(v4,0x84898))
        sub_10b50(0x84840,dcgettext(NULL,"Gid differs",5));
      v3 = sub_4bd20(v4);
      if (sub_2f7b0(v3))
        sub_10b50(0x84840,dcgettext(NULL,"Mod time differs",5));
      if ((*(char *)(dat_84408 + 0x9c) == 'S') || (v5 = "Size differs", v8 == dat_848c8)) {
        dat_84290 = __openat_2(dat_81234,v1,dat_84808);
        if (dat_84290 < 0) {
          sub_327c0(v1);
          sub_22bd0();
          sub_10b50(0x84840,0);
          return;
        }
        if (dat_84960) // branch-flip
          sub_2a070(dat_84290,0x84840);
        else {
          sub_10a20(sub_10c80);
        }
        if ((dat_84b88 == 1) && (v8)) {
          v6 = sub_4bd00(v4);
          v2 = sub_24a80(dat_84290,dat_81234,v1,SUB168(v6,0),SUB168(v6,8));
          if (v2)
            sub_32dd0(v1);
        }
        if (!close(dat_84290))
          return;
        sub_32680(v1);
        return;
      }
    }
    sub_10b50(0x84840,dcgettext(NULL,v5,5));
  }
  sub_22bd0();
  return;
}


// Function: sub_11090 @ 0x11090
void sub_11090(void)
{
  int4 v1; // eax
  unsigned long v2; // rax
  char v3 [144];
  char v4 [152];
  
  v1 = sub_10d70(dat_84848,v3);
  if (v1) {
    v1 = sub_10d70(dat_84858,v4);
    if ((v1) && (!sub_2aeb0(v3,v4))) {
      v2 = sub_23bb0(0,dat_84858);
      sub_10b50(0x84840,dcgettext(NULL,"Not linked to %s",5),v2);
    }
  }
  return;
}


// Function: sub_11160 @ 0x11160
void sub_11160(void) // return-dupe
{
  uint8 v1; // rax
  int8 v2; // rax
  char *v3; // rax
  char v4 [1032];
  
  v3 = v4;
  v1 = strlen(dat_84858);
  if (0x400 <= v1)
    v3 = (char *)sub_4c5b0(v1 + 1);
  v2 = readlinkat(dat_81234,dat_84848,v3,v1 + 1);
  if (0 <= v2) { // branch-flip
    if ((v1 != v2) || (memcmp(dat_84858,v3,v1)))
      sub_10b50(0x84840,dcgettext(NULL,"Symlink differs",5));
  }
  else {
    if (*__errno_location() != 2) // branch-flip
      sub_32a60(dat_84848);
    else {
      sub_32a80(dat_84848);
    }
    sub_10b50(0x84840,0);
  }
  if (v3 == v4)
    return;
  free(v3);
  return;
}


// Function: sub_11280 @ 0x11280
void sub_11280(void)
{
  int4 v1; // eax
  uint4 v2; // ecx
  char v3 [24];
  char *v4; // rsi
  uint4 v5; // stack - 0x90
  int8 v6; // stack - 0x80
  
  v1 = sub_10d70(dat_84848,v3);
  if (!v1)
    return;
  v2 = v5 & 0xf000;
  if (*(char *)(dat_84408 + 0x9c) != '3') { // branch-flip
    if (*(char *)(dat_84408 + 0x9c) == '4') {
      if (v2 != 0x6000) {
        sub_10b50(0x84840,dcgettext(NULL,"File type differs",5)); // return-dupe
        return;
      }
      goto label_11328;
    }
    if (v2 != 0x1000) {
      sub_10b50(0x84840,dcgettext(NULL,"File type differs",5));
      return;
    }
  }
  else {
    if (v2 != 0x2000) {
      sub_10b50(0x84840,dcgettext(NULL,"File type differs",5));
      return;
    }
label_11328:
    v4 = "Device number differs";
    if (dat_848c0 != v6) {
      sub_10b50(0x84840,dcgettext(NULL,v4,5)); // return-dupe
      return;
    }
  }
  if (!((v5 ^ dat_848b0) & 0xfff))
    return;
  v4 = "Mode differs";
  sub_10b50(0x84840,dcgettext(NULL,v4,5));
  return;
}


// Function: sub_113b0 @ 0x113b0
void sub_113b0(void) // return-dupe
{
  int4 v1; // eax
  char v2 [24];
  char *v3; // rsi
  uint4 v4; // stack - 0x90
  
  v1 = sub_10d70(dat_84848,v2);
  if (!v1)
    return;
  if ((v4 & 0xf000) != 0x4000) // branch-flip
    v3 = "File type differs";
  else {
    if (!((v4 ^ dat_848b0) & 0xfff))
      return;
    v3 = "Mode differs";
  }
  sub_10b50(0x84840,dcgettext(NULL,v3,5));
  return;
}


// Function: sub_11470 @ 0x11470
void sub_11470(void) // return-dupe
{
  int4 v1; // eax
  int8 v2; // rax
  char v3 [24];
  char *v4; // rsi
  uint4 v5; // stack - 0xa0
  int8 v6; // stack - 0x88
  
  if (dat_84850) {
    sub_113b0(); // tail-call
    return;
  }
  v1 = sub_10d70(dat_84848,v3);
  if (!v1)
    return;
  v4 = "File type differs";
  if ((v5 & 0xf000) == 0x8000) {
    v2 = sub_219b0((uint1 *)(dat_84408 + 0x171),0xc);
    if (((0 <= v2) && (!SCARRY8(v2,dat_848c8))) && (v6 == v2 + dat_848c8)) {
      v1 = __openat_2(dat_81234,dat_84848,dat_84808);
      if (v1 < 0) {
        sub_327c0(dat_84848);
        sub_10b50(0x84840,0);
        sub_22bd0();
        return;
      }
      if (0 <= (int8)lseek(v1,v2,0)) // branch-flip
        sub_10a20(sub_10c80);
      else {
        sub_32b20(dat_84848,v2);
        sub_10b50(0x84840,0);
      }
      if (!close(v1))
        return;
      sub_32680(dat_84848);
      return;
    }
    v4 = "Size differs";
  }
  sub_10b50(0x84840,dcgettext(NULL,v4,5));
  sub_22bd0();
  return;
}


// Function: sub_11730 @ 0x11730
void sub_11730(void)
{
  char *v1; // rax
  char v2;
  
  sub_d1f0(dat_84408);
  if (dat_84a54) {
    if (dat_84280)
      __fprintf_chk(dat_83f28,1,dcgettext(NULL,"Verify ",5));
    sub_22990((struct_9 *)0x84840,dat_84408,-1);
  }
  v2 = *(char *)(dat_84408 + 0x9c);
  switch(v2) {
    case 0:
    case 0x30:
    case 0x37:
    case 0x53:
label_117fd:
      if (!dat_84850) {
        sub_10dd0(); // tail-call
        return;
      }
      break;
    default:
      if (dat_84d50) {
        (*dat_84d50)();
        v2 = *(char *)(dat_84408 + 0x9c);
      }
      v1 = (char *)sub_4aa70(dat_84848);
      error(0,0,dcgettext(NULL,"%s: Unknown file type \'%c\', diffed as normal file",5),v1,v2);
      dat_84d58 = 2;
      goto label_117fd;
    case 0x31:
      sub_11090(); // tail-call
      return;
    case 0x32:
      sub_11160(); // tail-call
      return;
    case 0x33:
    case 0x34:
    case 0x36:
      sub_11280(); // tail-call
      return;
    case 0x35:
    case 0x44:
      if (sub_201b0(0x84840))
        sub_11650();
      break;
    case 0x4d:
      sub_11470(); // tail-call
      return;
    case 0x56:
      return;
    
  }
  sub_113b0(); // tail-call
  return;
}


// Function: sub_118d0 @ 0x118d0
void sub_118d0(void) // return-dupe
{
  int4 v1;
  int8 v2; // rax
  char *v3; // rax
  unsigned short v4 [2]; // stack - 0x60
  char v5 [24];
  uint4 v6; // r12d
  unsigned int v7; // stack - 0x5c
  
  if (sub_32fd0()) { // branch-flip
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"Archive contains file names with leading prefixes removed.",5));
    if (sub_30840()) goto label_11a80;
label_11aaf:
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"Verification may fail to locate original files.",5));
    sub_1f510();
  }
  else {
    if (sub_30840()) {
label_11a80:
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Archive contains transformed file names.",5));
      goto label_11aaf;
    }
    sub_1f510();
  }
  if (!dat_84288)
    sub_10af0();
  fsync(dat_84a34);
  ioctl(dat_84a34,0x24b);
  v7 = 1;
  v4[0] = 2;
  if (0x40000000 <= dat_84a34) // branch-flip
    v1 = sub_34080(dat_84a34 + -0x40000000,0x40086d01,v4);
  else {
    v1 = ioctl(dat_84a34,0x40086d01,v4);
  }
  if (0 <= v1) goto label_119b4;
  if (*__errno_location() != 5) { // branch-flip
    if (0x40000000 <= dat_84a34) goto label_1199c;
label_11c58:
    v2 = lseek(dat_84a34,0,0);
  }
  else {
    if (0x40000000 <= dat_84a34) // branch-flip
      v1 = sub_34080(dat_84a34 + -0x40000000,0x40086d01,v4);
    else {
      v1 = ioctl(dat_84a34,0x40086d01,v4);
    }
    if (0 <= v1) goto label_119b4;
    if (dat_84a34 <= 0x3fffffff) goto label_11c58;
label_1199c:
    v2 = sub_33f90(dat_84a34 + -0x40000000,0,0);
  }
  if (v2) {
    sub_32bd0((char *)*dat_84838);
    return;
  }
label_119b4:
  dat_84280 = 1;
  dat_83f40 = 0;
  sub_dfc0();
  while( true ) {
    v1 = sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0);
    if (v1 == 5) {
      v6 = 0;
      do {
        v6 += 1;
        sub_d1f0(dat_84408);
        v1 = sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0);
      } while (v1 == 5);
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcngettext(NULL,"VERIFY FAILURE: %d invalid header detected","VERIFY FAILURE: %d invalid headers detected",(int8)(int4)v6,5),(uint8)v6);
      dat_84d58 = 2;
    }
    if (v1 == 4) break;
    if (v1 != 3) { // branch-flip
      sub_215c0(dat_84408,0x84840,(int4 *)0x84400,1);
      sub_11730();
      sub_2f6f0((unsigned long *)0x84840);
    }
    else {
      sub_d1f0(dat_84408);
      if (!dat_84b51) {
        if (sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0) == 3) {
          dat_83f40 = 1;
          dat_84280 = 0;
          return;
        }
        if (dat_83b80 & 1) {
          if (dat_84d50)
            (*dat_84d50)();
          v3 = (char *)sub_43090(sub_d180(),v5);
          error(0,0,dcgettext(NULL,"A lone zero block at %s",5),v3);
        }
      }
    }
  }
  dat_84280 = 0;
  dat_83f40 = 1;
  return;
}


// Function: sub_11ce0 @ 0x11ce0
void sub_11ce0(uint8 a0,int8 a1,int8 a2)
{
  uint8 v1; // rdi
  
  do {
    a2 -= 1;
    v1 = a0 >> 3;
    *(uint1 *)(a1 + a2) = ((uint1)a0 & 7) + 0x30;
    a0 = v1;
  } while (a2);
  return;
}


// Function: sub_11d00 @ 0x11d00
void sub_11d00(int8 a0,int8 a1,int8 a2)
{
  char v1;
  int8 v2; // rax
  
  v2 = 0;
  do {
    v1 = *(char *)(a1 + v2);
    *(char *)(a0 + v2) = v1;
    if (!v1)
      return;
    v2 += 1;
  } while (a2 != v2);
  return;
}


// Function: sub_11d20 @ 0x11d20
void sub_11d20(int4 a0,uint8 a1,int8 a2,int8 a3)
{
  do {
    a3 -= 1;
    *(char *)(a2 + a3) = (char)a1;
    a1 = a1 >> 8 | (uint8)(uint4)-a0 << 0x38;
  } while (a3);
  return;
}


// Function: sub_11d50 @ 0x11d50
uint8 sub_11d50(int8 a0) // early-return x3
{
  uint8 v1;
  int8 v2;
  int8 v3;
  uint4 v4; // edx
  
  v4 = *(uint4 *)(a0 + 0x18) & 0xf000;
  if (v4 == 0x4000)
    return 1;
  if (v4 != 0x8000)
    return 0;
  if (!dat_84a30) {
    if (*(int8 *)(a0 + 0x30))
      return 1;
    return (uint8)((*(uint4 *)(a0 + 0x18) & 0x124) != 0x124);
  }
  if ((dat_84a69) && (dat_84a88)) {
    v1 = *(uint8 *)(a0 + 0x30);
    v3 = v1;
    if ((int8)v1 <= -1)
      v3 = v1 + 0x1ff;
    v2 = v3 >> 9;
    if ((v1 & 0x1ff) && (0x3ff <= v1 + 0x1ff))
      v2 += 1;
    return CONCAT71((undefined7)((uint8)v2 >> 8),*(int8 *)(a0 + 0x40) < v2);
  }
  return 0;
}


// Function: sub_11e00 @ 0x11e00
void sub_11e00(char *a0,uint8 a1) // return-dupe x2
{
  int8 v1;
  
  if (0x9d <= a1) // branch-flip
    v1 = 0x9b;
  else {
    v1 = a1 - 1;
    if (a0[a1 - 1] != '/') { // branch-flip
      if (!v1)
        return;
    }
    else {
      v1 = a1 - 2;
      if (!v1)
        return;
    }
  }
  do {
    if (a0[v1] == '/')
      return;
    v1 -= 1;
  } while (v1);
  return;
}


// Function: sub_11e50 @ 0x11e50
uint8 sub_11e50(struct_49 *a0,uint8 a1)
{
  return (a0->field_0x0 ^ a0->field_0x8) % a1;
}


// Function: sub_11e70 @ 0x11e70
unsigned long sub_11e70(struct_49 *a0,struct_49 *a1)
{
  uint8 v1; // rax
  
  v1 = a0->field_0x0 ^ a1->field_0x0 | a0->field_0x8 ^ a1->field_0x8;
  return CONCAT71((undefined7)(v1 >> 8),v1 == 0);
}


// Function: sub_11e90 @ 0x11e90
unsigned long sub_11e90(int4 a0,int4 a1,uint8 a2,unsigned long a3,code *a4,unsigned long a5,int8 a6,char *a7) // early-return
{
  char *v1;
  uint4 v10; // stack - 0xbc
  char v11 [23];
  int8 v12;
  uint8 v13;
  uint8 v14; // r15
  char *v15;
  char *v2;
  int8 v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  char *v6;
  char v7 [32];
  char v8 [32];
  char v9 [32];
  
  v1 = a7;
  if (a1) { // branch-flip
    if (0x40 <= (uint8)((a6 + -1) * 8)) { // branch-flip
      v13 = 0xffffffffffffffff;
      v14 = 0x8000000000000000;
      v2 = (char *)sub_43090(0xffffffffffffffff,v7);
    }
    else {
      v14 = 1L << ((uint1)((int4)(a6 + -1) << 3) & 0x3f);
      v13 = v14 - 1;
      v2 = (char *)sub_43090(v13,v7);
    }
    v3 = sub_43090(v14,v8);
    *(char *)(v3 + -1) = 0x2d;
    v15 = (char *)(v3 + -1);
  }
  else if (0x40 <= (uint8)(a6 * 3 - 3U)) { // branch-flip
    v13 = 0xffffffffffffffff;
    v2 = (char *)sub_43090(0xffffffffffffffff,v7);
    v15 = "0";
  }
  else {
    v15 = "0";
    v13 = (1L << ((char)a6 * '\x03' - 3U & 0x3fU)) - 1;
    v2 = (char *)sub_43090(v13,v7);
  }
  if (a0) { // branch-flip
    v3 = sub_43090(-a2,v9);
    *(char *)(v3 + -1) = 0x2d;
    v6 = (char *)(v3 + -1);
  }
  else {
    v6 = (char *)sub_43090(a2,&v12);
  }
  if (a4) {
    v13 &= (*a4)(&v10);
    v10 = dat_84ba4 == 6 & v10;
    if (v10)
      v13 = -v13;
    v4 = (char *)sub_43090(v13,v11);
    if (v10) {
      v4[-1] = '-';
      v4 = &v4[-1];
    }
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"value %s out of %s range %s..%s; substituting %s",5),v6,v1,v15,v2,v4);
    v5 = sub_12150(v10,v13,a3,0,a5,a6,v1);
    return v5;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"value %s out of %s range %s..%s",5),v6,v1,v15,v2);
  dat_84d58 = 2;
  return 0;
}


// Function: sub_12150 @ 0x12150
unsigned long sub_12150(unsigned long a0,uint8 a1,int8 a2,code *a3,char *a4,int8 a5,char *a6) // early-return
{
  uint8 v1;
  uint8 v2;
  uint8 v3;
  char v4; // al
  uint4 v5; // eax
  unsigned long v6; // rax
  
  v5 = dat_84ba4 & 0xfffffffb;
  if ((int4)a0) { // branch-flip
    if (v5 == 2) {
      v1 = a5 + -1;
      if ((0x3f < (uint8)(v1 * 8)) || (~a1 <= (uint8)((1L << ((char)v1 * '\b' & 0x3fU)) - 1U))) {
        v4 = 0xff;
        goto label_121e5;
      }
      v2 = a2 * 8;
      v3 = a5 * 3 - 3;
      if (v2 < v3 || !(v2 - v3)) {
        if (!dat_842b8) {
          dat_842b8 = 1;
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"Generating negative octal headers",5));
        }
        a4[a5 + -1] = 0;
        if (v2 <= 0x3f)
          a1 &= ~(-1L << ((char)a2 * '\b' & 0x3fU));
        sub_11ce0(a1,a4,v1);
        return 1;
      }
      v6 = sub_11e90(a0,v5 == 2,a1,a2,a3,a4,a5,a6); // return-dupe
      return v6;
    }
  }
  else {
    if ((0x40 <= (uint8)(a5 * 3 - 3U)) || (a1 <= (uint8)((1L << ((char)a5 * '\x03' - 3U & 0x3fU)) - 1U))) {
      a4[a5 + -1] = 0;
      sub_11ce0(a1,a4,a5 + -1);
      return 1;
    }
    if (v5 == 2) {
      if ((0x40 <= (uint8)((a5 + -1) * 8)) || (a1 <= (uint8)((1L << ((char)(a5 + -1) * '\b' & 0x3fU)) - 1U))) {
        v4 = 0x80;
label_121e5:
        *a4 = v4;
        sub_11d20(a0,a1,&a4[1],a5 + -1);
        return 1;
      }
      v6 = sub_11e90(a0,v5 == 2,a1,a2,a3,a4,a5,a6);
      return v6;
    }
  }
  a3 = NULL;
  v6 = sub_11e90(a0,v5 == 2,a1,a2,a3,a4,a5,a6);
  return v6;
}


// Function: sub_12380 @ 0x12380
int4 sub_12380(unsigned int *a0)
{
  if (dat_842b4) {
    *a0 = 0;
    return dat_842b4;
  }
  if (!sub_269f0("nobody",(unsigned int *)0x842b4)) {
    dat_842b4 = 0xfffffffe;
    *a0 = 0;
    return -2;
  }
  *a0 = 0;
  return dat_842b4;
}


// Function: sub_123e0 @ 0x123e0
int4 sub_123e0(unsigned int *a0)
{
  if (dat_842b0) {
    *a0 = 0;
    return dat_842b0;
  }
  if (!sub_26aa0("nobody",(unsigned int *)0x842b0)) {
    dat_842b0 = 0xfffffffe;
    *a0 = 0;
    return -2;
  }
  *a0 = 0;
  return dat_842b0;
}


// Function: sub_12440 @ 0x12440
unsigned long * sub_12440(char *a0)
{
  unsigned long *v1;
  uint8 v2; // rax
  uint8 v3; // rax
  unsigned long *v4; // rax
  char *v5; // rax
  uint8 v6; // rcx
  unsigned long *v7;
  
  v2 = strlen(a0);
  if (0x101 <= v2) {
    if (dat_84d50)
      (*dat_84d50)();
    v5 = (char *)sub_4aa70(a0);
    error(0,0,dcgettext(NULL,"%s: file name is too long (max %d); not dumped",5),v5,0x100);
    dat_84d58 = 2;
    return NULL;
  }
  v3 = sub_11e00(a0,v2);
  if ((v3) && (v2 = (v2 - 1) - v3, v2 - 1 <= 99)) {
    v4 = (unsigned long *)sub_e130();
    *v4 = 0;
    v4[0x3f] = 0;
    v6 = (uint8)(((int4)v4 - (int4)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
    v7 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
    while (v6) {
      v1 = &v7[1];
      *v7 = 0;
      v6 -= 1;
      v7 = v1;
    }
    memcpy((void *)((int8)v4 + 0x159),a0,v3);
    memcpy(v4,&a0[v3 + 1],v2);
    return v4;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v5 = (char *)sub_4aa70(a0);
  error(0,0,dcgettext(NULL,"%s: file name is too long (cannot be split); not dumped",5),v5);
  dat_84d58 = 2;
  return NULL;
}


// Function: sub_125b0 @ 0x125b0
uint8 sub_125b0(int8 a0) // return-dupe
{
  int4 *v1; // rax
  int8 v2;
  int8 v3; // rax
  uint8 v4; // r12
  
  v1 = __errno_location();
  v4 = CONCAT71((undefined7)((uint8)v1 >> 8),*v1 == 0x18 && a0) & 0xffffffff;
  if (*v1 != 0x18 || !a0)
    return v4;
  if (!*(int8 *)(a0 + 0x198))
    return 0;
  v2 = *(int8 *)(*(int8 *)(a0 + 0x198) + 0x198);
  do {
    if (!v2)
      return 0;
    while (v3 = v2, v2 = *(int8 *)(v3 + 0x198), 1 <= *(int4 *)(v3 + 0x1a8)) {
      if ((!v2) || (*(int4 *)(v2 + 0x1a8) <= 0)) {
        sub_2f670(v3);
        return v4;
      }
    }
  } while( true );
}


// Function: sub_12640 @ 0x12640
void sub_12640(char *a0)
{
  char *v1; // rax
  
  if (!(dat_83b80 & 0x20)) {
    if (dat_84b52)
      return;
    sub_2f7d0(2); // tail-call
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v1 = (char *)sub_4aa70(a0);
  error(0,0,dcgettext(NULL,"%s: Unknown file type; file ignored",5),v1);
  if (dat_84b52)
    return;
  sub_2f7d0(2); // tail-call
  return;
}


// Function: sub_126e0 @ 0x126e0
void sub_126e0(int8 *a0)
{
  uint8 v1;
  char *v2;
  int8 v3; // rax
  char *v4;
  uint8 v5;
  int8 v6;
  
  v2 = (char *)*a0;
  v5 = strlen(v2);
  do {
    if (!v5) {
      v6 = 1;
      v4 = v2;
label_1271c:
      if (*v4 != '/') {
        v3 = sub_4c610(v2,v5 + 2);
        *a0 = v3;
        v4 = (char *)(v3 + v5);
      }
      *v4 = '/';
      *(char *)(*a0 + v6) = 0;
      return;
    }
    v1 = v5 - 1;
    if (v2[v5 - 1] != '/') {
      v4 = &v2[v5];
      v6 = v5 + 1;
      goto label_1271c;
    }
    v5 = v1;
  } while( true );
}


// Function: sub_12760 @ 0x12760
void sub_12760(struct_53 *a0)
{
  char *v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  int8 v4;
  char *v5; // stack - 0x28
  
  if ((!dat_84b68) && (dat_842c8 < a0->field_0x68)) {
    v1 = a0->field_0x0;
    v5 = NULL;
    sub_23be0(&v5,(char *)sub_33020(v1,1,(uint1)dat_84b92));
    sub_30830(&v5,2);
    v2 = (unsigned long *)sub_4c5b0(strlen(v5) + 0x19);
    v1 = v5;
    v2[1] = a0->field_0x60;
    *v2 = a0->field_0x58;
    v2[2] = a0->field_0x68;
    strcpy((char *)&v2[3],v5);
    free(v1);
    v4 = dat_842c0;
    if (((!dat_842c0) && (v4 = sub_41b30(0,0,sub_11e50,sub_11e70,0), dat_842c0 = v4, !v4)) || (v3 = (unsigned long *)sub_42140(v4,v2), !v3))
      sub_16420(); // no-return
    if (v2 != v3)
      abort(); // no-return
    v2[2] = v2[2] + -1;
  }
  return;
}


// Function: sub_12990 @ 0x12990
unsigned long * sub_12990(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  uint8 v3; // rcx
  unsigned long *v4;
  
  v2 = (unsigned long *)sub_e130();
  *v2 = 0;
  v2[0x3f] = 0;
  v3 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v4 = (unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8);
  while (v3) {
    v1 = &v4[1];
    *v4 = 0;
    v3 -= 1;
    v4 = v1;
  }
  sub_12970(v2,*(int8 *)(a0 + 8));
  return v2;
}


// Function: sub_12a70 @ 0x12a70
void sub_12a70(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long *v1; // rax
  uint8 v2; // rax
  
  v1 = (unsigned long *)sub_4c5b0(0x28);
  *v1 = a0;
  *(unsigned int *)&v1[2] = a1;
  v1[3] = a2;
  v1[4] = dat_842d0;
  v2 = strlen(a0);
  dat_842d0 = v1;
  v1[1] = v2;
  return;
}


// Function: sub_12ad0 @ 0x12ad0
void sub_12ad0(char *a0,char *a1,char *a2)
{
  char *v1; // rax
  char *v2; // rax
  
  if ((dat_84a54) && (dat_83b80 & 4)) {
    if (dat_84d50)
      (*dat_84d50)();
    v1 = (char *)sub_4a870(1,a1);
    v2 = (char *)sub_4aa70(a0);
    error(0,0,dcgettext(NULL,"%s: contains a cache directory tag %s; %s",5),v2,v1,a2); // tail-call
    return;
  }
  return;
}


// Function: sub_12b70 @ 0x12b70
unsigned long sub_12b70(int4 a0) // early-return
{
  int4 v1; // eax
  char v2 [56];
  
  if (read(a0,v2,0x2b) != 0x2b)
    return 0;
  v1 = memcmp(v2,"Signature: 8a477f597d28d172789f06886806bc55",0x2b);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_12be0 @ 0x12be0
void sub_12be0(uint8 a0,char *a1,int8 a2)
{
  sub_12150(a0 >> 0x3f,a0,8,NULL,a1,a2,"off_t");
  return;
}


// Function: sub_12c10 @ 0x12c10
void sub_12c10(uint8 a0,char *a1,int8 a2)
{
  sub_12150(a0 >> 0x3f,a0,8,NULL,a1,a2,"time_t");
  return;
}


// Function: sub_12c40 @ 0x12c40
void sub_12c40(void)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  void *v3; // rax
  uint8 v4; // rcx
  unsigned long *v5;
  
  v2 = (unsigned long *)sub_e130();
  *v2 = 0;
  v2[0x3f] = 0;
  v4 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v5 = (unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8);
  while (v4) {
    v1 = &v5[1];
    *v5 = 0;
    v4 -= 1;
    v5 = v1;
  }
  sub_d1f0(v2);
  v3 = (void *)sub_e130();
  memset(v3,0,sub_d2b0(v3));
  sub_d1f0(v3); // tail-call
  return;
}


// Function: sub_12cb0 @ 0x12cb0
unsigned long * sub_12cb0(int8 a0,uint8 a1,uint8 a2)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  uint8 v3;
  unsigned long *v4;
  
  v2 = (unsigned long *)sub_e130();
  *v2 = 0;
  v2[0x3f] = 0;
  v3 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
  v4 = (unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8);
  while (v3) {
    v1 = &v4[1];
    *v4 = 0;
    v3 -= 1;
    v4 = v1;
  }
  sub_12970(v2,a0);
  sub_12be0(a1,(char *)((int8)v2 + 0x7c),0xc);
  v3 = 0x1ffffffff;
  if (a2 <= 0x1ffffffff)
    v3 = a2;
  if ((int8)a2 <= -1)
    v3 = 0;
  sub_12c10(v3,&v2[0x11],0xc);
  sub_129e0(0x81a4,(char *)((int8)v2 + 100));
  sub_128b0(0,(char *)((int8)v2 + 0x6c));
  sub_12940(0,(char *)((int8)v2 + 0x74));
  *(unsigned int *)((int8)v2 + 0x101) = 0x61747375;
  *(unsigned short *)((int8)v2 + 0x105) = 0x72;
  *(unsigned short *)((int8)v2 + 0x107) = 0x3030;
  return v2;
}


// Function: sub_12da0 @ 0x12da0
unsigned long * sub_12da0(bool a0,unsigned long *a1,unsigned long *a2)
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long v11; // stack - 0x40
  unsigned long *v2;
  void *v3; // rax
  int8 v4; // rcx
  uint8 v5; // rcx
  unsigned long v6;
  unsigned long v7 [63]; // stack - 0x238
  unsigned long *v8;
  unsigned long v9; // rdi
  
  if ((!a1[0x2f]) && (a1[0x2d])) {
    sub_1cc10((struct_6 *)&a1[0x2d]);
    v4 = 0x40;
    v8 = v7;
    while (v4) {
      v1 = &v8[1];
      v10 = &a2[1];
      *v8 = *a2;
      v4 -= 1;
      a2 = v10;
      v8 = v1;
    }
    if (a0) { // branch-flip
      v3 = (void *)sub_1c4b0();
      v9 = 0x67;
      v6 = dat_84a20;
    }
    else {
      v3 = (void *)sub_1c460(a1);
      if (dat_84af0) { // branch-flip
        v9 = 0x78;
        v6 = dat_84ae0;
      }
      else {
        v6 = a1[0x16];
        v9 = 0x78;
      }
    }
    sub_1ccf0(v9,v3,v6,(struct_6 *)&a1[0x2d]);
    free(v3);
    a2 = (unsigned long *)sub_e130();
    *a2 = v7[0];
    v4 = (int8)a2 - (int8)(unsigned long *)((uint8)&a2[1] & 0xfffffffffffffff8);
    a2[0x3f] = v11;
    v5 = (uint8)((int4)v4 + 0x200U >> 3);
    v8 = (unsigned long *)((int8)v7 - v4);
    v10 = (unsigned long *)((uint8)&a2[1] & 0xfffffffffffffff8);
    while (v5) {
      v2 = &v10[1];
      v1 = &v8[1];
      *v10 = *v8;
      v5 -= 1;
      v8 = v1;
      v10 = v2;
    }
  }
  return a2;
}


// Function: sub_12f00 @ 0x12f00
void sub_12f00(uint1 *a0)
{
  uint1 v1;
  uint1 *v2;
  int4 v3; // edx
  
  v3 = 0;
  a0[0x94] = 0x20;
  a0[0x95] = 0x20;
  a0[0x96] = 0x20;
  a0[0x97] = 0x20;
  a0[0x98] = 0x20;
  a0[0x99] = 0x20;
  a0[0x9a] = 0x20;
  a0[0x9b] = 0x20;
  v2 = a0;
  do {
    v1 = *v2;
    v2 = &v2[1];
    v3 += (uint4)v1;
  } while (&a0[0x200] != v2);
  sub_12a40((int8)v3,&a0[0x94],v3);
  sub_d1f0(a0); // tail-call
  return;
}


// Function: sub_12f60 @ 0x12f60
void sub_12f60(struct_9 *a0,int8 a1,int8 a2)
{
  uint4 v1; // eax
  
  if (((dat_84a54) && (*(uint1 *)(a1 + 0x9c) != 0x4b)) && ((v1 = (uint4)*(uint1 *)(a1 + 0x9c) - 0x4c, 0x2d <= (uint1)v1 || (!(0x100008000001U >> ((uint8)v1 & 0x3f) & 1))))) {
    dat_84400 = dat_84ba4;
    sub_22990(a0,a1,a2);
  }
  sub_12f00((uint1 *)sub_12da0(0,a0,a1)); // tail-call
  return;
}


// Function: sub_12fd0 @ 0x12fd0
void sub_12fd0(struct_9 *a0,char *a1,char a2)
{
  uint8 v1;
  int8 v2; // rax
  void *v3; // rax
  uint8 v4; // rax
  uint8 v5;
  char *v6;
  
  v1 = strlen(a1);
  v5 = v1 + 1;
  v2 = sub_12cb0("././@LongLink",v5,0);
  if (!dat_84ad2) {
    if (!dat_842a8) {
      sub_268d0(0,(unsigned long *)0x842a8);
      sub_26960(0,(unsigned long *)0x842a0);
    }
    sub_12890(dat_842a8,v2 + 0x109);
    sub_12890(dat_842a0,v2 + 0x129);
  }
  *(char *)(v2 + 0x9c) = a2;
  *(unsigned long *)(v2 + 0x101) = 0x20207261747375;
  sub_12f60(a0,v2,-1);
  v3 = (void *)sub_e130();
  v4 = sub_d2b0(v3);
  v6 = a1;
  if (v4 < v5) {
    do {
      v6 = &a1[v4];
      memcpy(v3,a1,v4);
      v5 -= v4;
      sub_d1f0((int8)v3 + (v4 - 1 & 0xfffffffffffffe00));
      v3 = (void *)sub_e130();
      v4 = sub_d2b0(v3);
      a1 = v6;
    } while (v4 < v5);
    v1 = v5 - 1;
  }
  memcpy(v3,v6,v5);
  memset((void *)((int8)v3 + v5),0,v4 - v5);
  sub_d1f0((int8)v3 + (v1 & 0xfffffffffffffe00)); // tail-call
  return;
}


// Function: sub_13140 @ 0x13140
unsigned long sub_13140(int8 a0)
{
  unsigned long v1; // rax
  char *v2;
  
  switch(dat_84ba4) {
    default:
      abort(); // no-return
    case 1:
      v2 = *(char **)(a0 + 8);
      if (100 <= strlen(v2)) {
        if (dat_84d50) {
          (*dat_84d50)();
          v2 = *(char **)(a0 + 8);
        }
        v1 = sub_4aa70(v2);
        error(0,0,dcgettext(NULL,"%s: file name is too long (max %d); not dumped",5),v1,99);
        dat_84d58 = 2;
        return 0;
      }
      break;
    case 2:
    case 6:
      sub_12fd0(a0,*(char **)(a0 + 8),0x4c);
      break;
    case 3:
    case 5:
      v1 = sub_12440(*(char **)(a0 + 8)); // tail-call
      return v1;
    case 4:
      sub_1ca30("path",a0,0);
      v1 = sub_12990(a0); // tail-call
      return v1;
    
  }
  v1 = sub_12990(a0); // tail-call
  return v1;
}


// Function: sub_13240 @ 0x13240
void sub_13240(int8 a0)
{
  char *v1;
  bool v2; // zf
  
  v1 = *(char **)(a0 + 8);
  if (dat_84ba4 == 4) {
    if (!sub_31440(v1)) {
      sub_1ca30("path",a0,0);
      sub_12990(a0); // tail-call
      return;
    }
    v1 = *(char **)(a0 + 8);
  }
  v2 = dat_84ba4 != 2;
  if ((uint8)v2 + 99 < strlen(v1)) {
    sub_13140(a0); // tail-call
    return;
  }
  sub_12990(a0); // tail-call
  return;
}


// Function: sub_132d0 @ 0x132d0
int8 sub_132d0(int8 a0)
{
  int8 v1;
  int8 v10; // stack - 0x68
  int8 v11; // stack - 0x60
  uint8 v12; // stack - 0x58
  char *v13;
  int8 v14; // stack - 0x50
  unsigned long v2;
  char v3; // al
  uint4 v4;
  uint4 v5;
  uint4 v6; // eax
  int4 v7; // eax
  int8 v8; // rax
  uint8 v9;
  
  v10 = 0;
  v11 = 0;
  v8 = sub_13240(a0);
  if (!v8)
    return 0;
  sub_239d0(*(unsigned int *)(a0 + 0x74),(int4 *)(a0 + 0x74),&v10);
  sub_23aa0(*(unsigned int *)(a0 + 0x78),(int4 *)(a0 + 0x78),&v11);
  if (dat_84b18) { // branch-flip
    v5 = *(uint4 *)(a0 + 0x70);
    v4 = sub_43970(v5,(v5 & 0xf000) == 0x4000,dat_84b14,dat_84b18,NULL);
    v4 = v5 & 0xfffff000 | v4;
    *(uint4 *)(a0 + 0x70) = v4;
  }
  else {
    v4 = *(uint4 *)(a0 + 0x70);
  }
  if ((dat_84ba4 & 0xfffffffd) != 1) // branch-flip
    sub_129e0(v4,(char *)(v8 + 100));
  else {
    sub_129e0(v4 & 0xfff,(char *)(v8 + 100));
  }
  v4 = *(uint4 *)(a0 + 0x74);
  if ((dat_84ba4 == 4) && (0x200000 <= v4)) {
    sub_1ca30("uid",a0,0);
    v4 = 0;
  }
  if (!sub_128b0(v4,(char *)(v8 + 0x6c)))
    return 0;
  v4 = *(uint4 *)(a0 + 0x78);
  if ((dat_84ba4 == 4) && (0x200000 <= v4)) {
    sub_1ca30("gid",a0,0);
    v4 = 0;
  }
  if (!sub_12940(v4,(char *)(v8 + 0x74)))
    return 0;
  v9 = *(uint8 *)(a0 + 0x88);
  if ((dat_84ba4 == 4) && (v9 >> 0x21)) {
    sub_1ca30("size",a0,0);
    v9 = 0;
  }
  v3 = sub_12be0(v9,(char *)(v8 + 0x7c),0xc);
  v1 = dat_84ae8;
  v9 = dat_84ae0;
  if (!v3)
    return 0;
  if (dat_84af0 != 1) { // branch-flip
    if (dat_84af0 != 2) { // branch-flip
      if (!dat_84af0) {
        v12 = *(uint8 *)(a0 + 0xf8);
        v14 = *(int8 *)(a0 + 0x100);
      }
    }
    else {
      v7 = sub_4be80(*(int8 *)(a0 + 0xf8),*(int8 *)(a0 + 0x100),dat_84ae0,dat_84ae8);
      if (1 <= v7) { // branch-flip
        v12 = v9;
        v14 = v1;
      }
      else {
        v12 = *(uint8 *)(a0 + 0xf8);
        v14 = *(int8 *)(a0 + 0x100);
      }
    }
  }
  else {
    v12 = dat_84ae0;
    v14 = dat_84ae8;
  }
  v9 = v12;
  if ((dat_84ba4 == 4) && (((v12 >> 0x21 || (v14)) && (sub_1ca30("mtime",a0,&v12), v9 = v12, v12 >> 0x21)))) {
    v12 = 0;
    v9 = 0;
  }
  v3 = sub_12c10(v9,(char *)(v8 + 0x88),0xc);
  v4 = dat_84ba4;
  if (!v3)
    return 0;
  if ((*(uint4 *)(a0 + 0x70) & 0xb000) != 0x2000) { // branch-flip
    if ((dat_84ba4 & 0xfffffffb) != 2) {
      if (!sub_12910(0,(char *)(v8 + 0x149)))
        return 0;
      v3 = sub_128e0(0,(char *)(v8 + 0x151));
      goto label_1370f;
    }
  }
  else {
    v2 = *(unsigned long *)(a0 + 0x80);
    v5 = gnu_dev_major(v2);
    v6 = gnu_dev_minor(v2);
    if ((0x200000 <= v5) && (v4 == 4)) {
      v5 = 0;
      sub_1ca30("devmajor",a0,0);
    }
    if (!sub_12910(v5,(char *)(v8 + 0x149)))
      return 0;
    if ((dat_84ba4 == 4) && (0x200000 <= v6)) {
      sub_1ca30("devminor",a0,0);
      v6 = 0;
    }
    v3 = sub_128e0(v6,(char *)(v8 + 0x151));
label_1370f:
    if (!v3)
      return 0;
  }
  if (dat_84ba4 != 4) { // branch-flip
    v4 = dat_84ba4;
    if ((dat_84b50) && ((dat_84ba4 & 0xfffffffb) == 2)) {
      sub_12c10(*(uint8 *)(a0 + 0xe8),(char *)(v8 + 0x159),0xc);
      sub_12c10(*(uint8 *)(a0 + 0x108),(char *)(v8 + 0x165),0xc);
      v4 = dat_84ba4;
    }
  }
  else {
    sub_1ca30("atime",a0,0);
    sub_1ca30("ctime",a0,0);
    v4 = dat_84ba4;
  }
  if (v4 == 1) {
    *(char *)(v8 + 0x9c) = 0;
    return v8;
  }
  *(char *)(v8 + 0x9c) = 0x30;
  if (5 <= v4) { // branch-flip
    if (v4 != 6)
      abort(); // no-return, return-dupe
  }
  else {
    if (3 <= v4) {
      *(unsigned int *)(v8 + 0x101) = 0x61747375;
      *(unsigned short *)(v8 + 0x105) = 0x72;
      *(unsigned short *)(v8 + 0x107) = 0x3030;
      goto label_134f9;
    }
    if (v4 != 2) {
      abort();
    }
  }
  *(unsigned long *)(v8 + 0x101) = 0x20207261747375;
label_134f9:
  if (!dat_84ad2) {
    if (v10) // branch-flip
      *(unsigned long *)(a0 + 0x20) = sub_4c7a0(v10);
    else {
      sub_268d0(*(unsigned int *)(a0 + 0x74),(unsigned long *)(a0 + 0x20));
    }
    if (v11) // branch-flip
      *(unsigned long *)(a0 + 0x28) = sub_4c7a0(v11);
    else {
      sub_26960(*(unsigned int *)(a0 + 0x78),(unsigned long *)(a0 + 0x28));
    }
    v13 = *(char **)(a0 + 0x20);
    if (dat_84ba4 == 4) {
      if ((0x21 <= strlen(v13)) || (!sub_31440(v13))) {
        sub_1ca30("uname",a0,0);
        v13 = *(char **)(a0 + 0x20);
      }
      else {
        v13 = *(char **)(a0 + 0x20);
      }
    }
    sub_12890(v13,v8 + 0x109);
    if (dat_84ba4 == 4) {
      v13 = *(char **)(a0 + 0x28);
      if ((0x21 <= strlen(v13)) || (!sub_31440(v13)))
        sub_1ca30("gname",a0,0);
    }
    sub_12890(*(int8 *)(a0 + 0x28),v8 + 0x129);
    v4 = dat_84ba4;
  }
  if (v4 == 4) {
    if (1 <= dat_84a9c) {
      if (*(int8 *)(a0 + 0x38))
        sub_1ca30("SCHILY.acl.access",a0,0);
      if (*(int8 *)(a0 + 0x48))
        sub_1ca30("SCHILY.acl.default",a0,0);
    }
    if ((1 <= dat_84aa0) && (*(int8 *)(a0 + 0x30)))
      sub_1ca30("RHT.security.selinux",a0,0);
    if (1 <= dat_84a98) {
      v9 = 0;
      v12 = 0;
      v1 = *(int8 *)(a0 + 0x160);
      if (*(int8 *)(a0 + 0x158)) {
        do {
          sub_1ca30(*(char **)(v1 + v9 * 0x18),a0,&v12);
          v9 = v12 + 1;
          v12 = v9;
        } while (v9 < *(uint8 *)(a0 + 0x158));
        return v8;
      }
      return v8;
    }
    return v8;
  }
  return v8;
}


// Function: sub_13a40 @ 0x13a40
void sub_13a40(int8 a0)
{
  uint8 v1; // rax
  char *v2; // rax
  
  if (dat_84ba4 <= 6) {
    v1 = 1L << ((uint1)dat_84ba4 & 0x3f);
    if (v1 & 0x2a) {
      if (dat_84d50)
        (*dat_84d50)();
      v2 = (char *)sub_4aa70(*(char **)(a0 + 0x18));
      error(0,0,dcgettext(NULL,"%s: link name is too long; not dumped",5),v2);
      dat_84d58 = 2;
      return;
    }
    if (v1 & 0x44) {
      sub_12fd0(a0,*(char **)(a0 + 0x18),0x4b); // tail-call
      return;
    }
    if (dat_84ba4 == 4) {
      sub_1ca30("linkpath",a0,0); // tail-call
      return;
    }
  }
  abort(); // no-return
}


// Function: sub_13b00 @ 0x13b00
char sub_13b00(struct_53 *a0) // early-return x4
{
  char v1;
  int8 v2; // rax
  char *v3; // rax
  int8 v4; // rax
  unsigned long v5; // stack - 0x58
  bool v6; // zf
  unsigned long v7; // stack - 0x50
  
  if (!dat_842c0)
    return '\0';
  if ((dat_842c8 < a0->field_0x68) || (v1 = dat_84ab8, dat_84ab8)) {
    v7 = a0->field_0x60;
    v5 = a0->field_0x58;
    v2 = sub_418d0(dat_842c0,&v5);
    if (!v2)
      return '\0';
    v3 = (char *)sub_33020((char *)(v2 + 0x18),1,(uint1)dat_84b92);
    if (*(int8 *)(v2 + 0x10))
      *(int8 *)(v2 + 0x10) = *(int8 *)(v2 + 0x10) + -1;
    v2 = sub_d180();
    sub_23be0(&a0->field_0x8[0x10],v3);
    v6 = dat_84ba4 != 2;
    if ((uint8)v6 + 99 < strlen(v3))
      sub_13a40(a0);
    a0->field_0x88 = 0;
    v4 = sub_132d0(a0);
    if (!v4)
      return '\0';
    sub_11d00(v4 + 0x9d,v3,100);
    *(char *)(v4 + 0x9c) = 0x31;
    sub_12f60(a0,v4,v2);
    v1 = dat_84ab8;
    if (!dat_84ab8)
      return '\x01';
    sub_30b70(a0->field_0x0,0);
  }
  return v1;
}


// Function: sub_13c80 @ 0x13c80
void sub_13c80(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  uint8 v3; // rcx
  int8 v4; // rbp
  unsigned long *v5;
  int8 v6;
  
  if (1 <= a0) {
    v6 = a0 + -0x200;
    v4 = v6 - (a0 - 1U & 0xfffffffffffffe00U);
    while( true ) {
      v2 = (unsigned long *)sub_e130();
      *v2 = 0;
      v2[0x3f] = 0;
      v3 = (uint8)(((int4)v2 - (int4)(unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
      v5 = (unsigned long *)((uint8)&v2[1] & 0xfffffffffffffff8);
      while (v3) {
        v1 = &v5[1];
        *v5 = 0;
        v3 -= 1;
        v5 = v1;
      }
      sub_d1f0(v2);
      if (v6 == v4) break;
      v6 -= 0x200;
    }
    return;
  }
  return;
}


// Function: sub_13d10 @ 0x13d10
unsigned long sub_13d10(int4 a0,struct_75 *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v10;
  uint8 v11; // r14
  uint8 v12;
  uint4 v2; // eax
  int8 v3; // rax
  int8 v4; // rax
  uint8 v5;
  uint8 v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  char v9 [24];
  
  v12 = a1->field_0x88;
  v3 = sub_d180();
  v4 = sub_132d0(a1);
  if (!v4)
    return 2;
  sub_12f60(a1,v4,v3);
  sub_cc00(a1->field_0x8,a1->field_0x88,a1->field_0x88);
label_13d78:
  do {
    v11 = v12;
    if ((int8)v11 <= 0)
      return 0;
    v3 = sub_e130();
    v5 = sub_d2b0(v3);
    if (v5 <= v11) { // branch-flip
      if (a0 <= 0) {
        sub_d1f0(v3 + (v5 - 1 & 0xfffffffffffffe00));
        v12 = v11 - v5;
        goto label_13d78;
      }
    }
    else {
      if (v11 & 0x1ff) {
        v2 = 0x200 - ((uint4)v11 & 0x1ff);
        v10 = (unsigned long *)(v3 + v11);
        if (8 <= v2) { // branch-flip
          *v10 = 0;
          *(unsigned long *)((int8)v10 + ((uint8)v2 - 8)) = 0;
          v12 = (uint8)(v2 + ((int4)v10 - (int4)(unsigned long *)((uint8)&v10[1] & 0xfffffffffffffff8)) >> 3);
          v10 = (unsigned long *)((uint8)&v10[1] & 0xfffffffffffffff8);
          while (v12) {
            v1 = &v10[1];
            *v10 = 0;
            v12 -= 1;
            v10 = v1;
          }
        }
        else if (v2 & 4) { // branch-flip
          *(unsigned int *)v10 = 0;
          *(unsigned int *)((int8)v10 + ((uint8)v2 - 4)) = 0;
        }
        else if ((v2) && (*(char *)v10 = 0, v2 & 2))
          *(unsigned short *)((int8)v10 + ((uint8)v2 - 2)) = 0;
      }
      if (a0 <= 0) {
        sub_d1f0(v3 + (v11 - 1 & 0xfffffffffffffe00));
        return 0;
      }
      v5 = v11;
    }
    v6 = sub_24960(a0,v3,v5);
    if (v6 == 0xffffffffffffffff) {
      sub_24e00(a1->field_0x0,a1->field_0x88 - v11,v5);
      sub_13c80(v11);
      return 1;
    }
    v12 = v11 - v6;
    sub_d1f0((v5 - 1 & 0xfffffffffffffe00) + v3);
    if (v6 != v5) {
      memset((void *)(v3 + v6),0,v5 - v6);
      if (dat_83b80 & 0x80) {
        if (dat_84d50)
          (*dat_84d50)();
        v7 = sub_43090(v12,v9);
        v8 = sub_4aa70(a1->field_0x0);
        error(0,0,dcngettext(NULL,"%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v12,5),v8,v7);
      }
      if (!dat_84b52)
        sub_2f7d0(1);
      sub_13c80(v11 - v5);
      return 1;
    }
  } while( true );
}


// Function: sub_13fe0 @ 0x13fe0
unsigned long sub_13fe0(int8 a0)
{
  DIR *v1; // rax
  unsigned long v2; // rax
  
  do {
    v1 = fdopendir(*(int4 *)(a0 + 0x1a8));
    *(DIR **)(a0 + 0x1a0) = v1;
    if (v1) {
      v2 = sub_4b0d0(v1,dat_847e4); // tail-call
      return v2;
    }
  } while (sub_125b0(a0));
  return 0;
}


// Function: sub_14030 @ 0x14030
void sub_14030(void) // return-dupe
{
  int8 v1; // rax
  unsigned long v2; // rax
  
  if (!dat_842c0)
    return;
  v1 = sub_41930(dat_842c0);
  if (!v1)
    return;
  do {
    if (*(int8 *)(v1 + 0x10)) {
      if (dat_84d50)
        (*dat_84d50)();
      v2 = sub_4ac50((char *)(v1 + 0x18));
      error(0,0,dcgettext(NULL,"Missing links to %s.",5),v2);
    }
    v1 = sub_41980(dat_842c0,v1);
  } while (v1);
  return;
}


// Function: sub_140d0 @ 0x140d0
int4 sub_140d0(int8 a0,char *a1,int4 a2)
{
  int4 v1;
  
  if (!dat_84298) {
    dat_84298 = '\x01';
    strerror(2);
    dcgettext(NULL,"",5);
  }
  while( true ) {
    v1 = dat_81234;
    if (a0)
      v1 = *(int4 *)(a0 + 0x1a8);
    v1 = __openat_2(v1,a1,a2);
    if (0 <= v1) break;
    if (!sub_125b0(a0))
      return v1;
  }
  return v1;
}


// Function: sub_14160 @ 0x14160
unsigned int sub_14160(int8 a0,unsigned long *a1) // return-dupe
{
  int8 *v1;
  code *v2;
  int4 v3; // eax
  uint8 v4; // rax
  unsigned long *v5;
  
  v5 = dat_842d0;
  if (!dat_842d0)
    return 0;
  do {
    v4 = sub_140d0(a0,(char *)*v5,dat_84808);
    v3 = (int4)v4;
    if (0 <= v3) {
      v2 = (code *)v5[3];
      if ((!v2) || ((*v2)(v4 & 0xffffffff))) {
        close(v3);
        if (a1)
          *a1 = *v5;
        return *(unsigned int *)&v5[2];
      }
      close(v3);
    }
    v1 = &v5[4];
    v5 = (unsigned long *)*v1;
    if (!(unsigned long *)*v1)
      return 0;
  } while( true );
}


// Function: sub_14200 @ 0x14200
void sub_14200(int8 a0)
{
  unsigned long *v1;
  int4 v2;
  int4 v3;
  stat v4; // stack - 0xb8
  
  v1 = *(unsigned long **)(a0 + 0x198);
  if ((v1) && (!*(int4 *)&v1[0x35])) {
    v2 = __openat_2(*(int4 *)(a0 + 0x1a8),"..",dat_84804);
    if (0 <= v2) { // branch-flip
      if (((fstat(v2,&v4)) || (v1[0xc] != v4._8_8_)) || (v1[0xb] != v4._0_8_)) {
        close(v2);
        *(unsigned int *)&v1[0x35] = 2;
        return;
      }
    }
    else {
      v3 = *__errno_location();
      v2 = -v3;
      if ((0 < v3) && (v3 = __openat_2(dat_81234,(char *)*v1,dat_84804), 0 <= v3)) {
        if ((fstat(v2,&v4)) || ((v1[0xc] != v4._8_8_ || (v1[0xb] != v4._0_8_))))
          close(v3);
        else {
          v2 = v3;
        }
      }
    }
    *(int4 *)&v1[0x35] = v2;
  }
  return;
}


// Function: sub_14330 @ 0x14330
void sub_14330(struct_74 *a0,char *a1,char *a2)
{
  char v1 [408];
  struct_74 *v2; // stack - 0x50
  
  sub_2f630(v1);
  v2 = a0;
  sub_14ca0(v1,a1,a2);
  if ((a0) && (dat_84b28))
    sub_1f580(a0);
  sub_2f6f0(v1);
  return;
}


// Function: sub_143d0 @ 0x143d0
void sub_143d0(void)
{
  char v1;
  stat v10 [2];
  uint8 v11; // stack - 0x218
  int4 v12; // stack - 0x50
  char *v2;
  int4 v3; // eax
  int8 v4; // rax
  char *v5;
  uint8 v6; // rax
  char *v7;
  uint8 v8; // stack - 0x200
  int8 v9 [11]; // stack - 0x1f8
  
  dat_842c8 = 0;
  if (dat_84630 != 2)
    dat_842c8 = (uint8)(dat_84b69 ^ 1);
  sub_fb70(1);
  sub_cca0();
  if (dat_84b50) { // branch-flip
    v8 = 0;
    sub_276a0();
    while (v4 = sub_27b00(), v4) {
      v5 = *(char **)(v4 + 0x10);
      if (!sub_16b00(v5,0))
        sub_14330(NULL,*(char **)(v4 + 0x10),*(char **)(v4 + 0x10));
    }
    sub_27b80();
    v5 = NULL;
    while (v4 = sub_27b00(), v4) {
      v7 = *(char **)(v4 + 0x10);
      if (!sub_16b00(v7,0)) {
        v7 = *(char **)(v4 + 0x10);
        v11 = strlen(v7);
        if (v8 <= v11) {
          do {
            v5 = (char *)sub_4c710(v5,&v8);
          } while (v8 <= v11);
          v7 = *(char **)(v4 + 0x10);
        }
        memcpy(v5,v7,v11);
        if (v5[v11 - 1] != '/') {
          v6 = v11 + 1;
          v5[v11] = '/';
          v11 = v6;
        }
        sub_2f630(v9);
        v7 = (char *)sub_1fa60(*(int8 *)(v4 + 0x38));
        if (v7) {
          for (; v1 = *v7, v1; v7 = &v7[v6 + 1]) {
            v6 = strlen(v7);
            if (v1 == 'Y') {
              if (!v9[0]) {
                v3 = __openat_2(dat_81234,*(char **)(v4 + 0x10),dat_84804);
                if (v3 < 0) {
                  sub_24ef0(*(char **)(v4 + 0x10),*(int8 *)(v4 + 0x40) == 0,sub_24dd0);
                  break;
                }
                v12 = v3;
                if (fstat(v3,v10)) {
                  sub_24ef0(*(char **)(v4 + 0x10),*(int8 *)(v4 + 0x40) == 0,sub_24ec0);
                  break;
                }
                v2 = *(char **)(v4 + 0x10);
                v9[0] = sub_4c7a0(v2);
              }
              while (v8 < v11 + v6) {
                v5 = (char *)sub_4c710(v5,&v8);
              }
              strcpy(&v5[v11],&v7[1]);
              sub_14330(v9,&v7[1],v5);
            }
          }
        }
        sub_2f6f0(v9);
      }
    }
    free(v5);
  }
  else {
    while (v5 = (char *)sub_26cd0(1), v5) {
      if (!sub_16b00(v5,0))
        sub_14330(NULL,v5,v5);
    }
  }
  sub_12c40();
  sub_f900();
  sub_30b30();
  if (dat_84b28) {
    sub_20010(); // tail-call
    return;
  }
  return;
}


// Function: sub_14710 @ 0x14710
void sub_14710(struct_75 *a0,char *a1)
{
  uint8 v1;
  void *v10; // rax
  uint8 v11; // rax
  uint8 v12; // rax
  uint8 v13;
  char *v14; // stack - 0x48
  unsigned long *v15;
  unsigned long *v2;
  int8 v3;
  int4 v4; // eax
  uint4 v5; // eax
  int8 v6; // rax
  int8 v7; // rax
  char *v8;
  int8 v9;
  
  v9 = a0->field_0x198;
  v6 = sub_d180();
  a0->field_0x88 = 0;
  v7 = sub_132d0(a0);
  if (v7) {
    sub_16890(a0);
    if (dat_84b50) { // branch-flip
      v3 = *(int8 *)(dat_84620 + 0x38);
      if (dat_84ba4 != 4) { // branch-flip
        *(char *)(v7 + 0x9c) = 0x44;
        if (v3) {
          v9 = sub_d180();
          v8 = (char *)sub_1fa90(*(int8 *)(dat_84620 + 0x38));
          v13 = sub_1f4c0(v8);
          sub_12be0(v13,(char *)(v7 + 0x7c),0xc);
          sub_12f60(a0,v7,v9);
          sub_cc00(a0->field_0x8,v13,v13);
          while( true ) {
            if ((int8)v13 <= 0)
              return;
            v10 = (void *)sub_e130();
            v11 = sub_d2b0(v10);
            if (v13 < v11) break;
            v13 -= v11;
            memcpy(v10,v8,v11);
            v8 = &v8[v11];
            sub_d1f0((int8)v10 + (v11 - 1 & 0xfffffffffffffe00));
          }
          if (v13 & 0x1ff) {
            v5 = 0x200 - ((uint4)v13 & 0x1ff);
            v15 = (unsigned long *)((int8)v10 + v13);
            if (8 <= v5) { // branch-flip
              *v15 = 0;
              *(unsigned long *)((int8)v15 + ((uint8)v5 - 8)) = 0;
              v11 = (uint8)(v5 + ((int4)v15 - (int4)(unsigned long *)((uint8)&v15[1] & 0xfffffffffffffff8)) >> 3);
              v15 = (unsigned long *)((uint8)&v15[1] & 0xfffffffffffffff8);
              while (v11) {
                v2 = &v15[1];
                *v15 = 0;
                v11 -= 1;
                v15 = v2;
              }
            }
            else if (v5 & 4) { // branch-flip
              *(unsigned int *)v15 = 0;
              *(unsigned int *)((int8)v15 + ((uint8)v5 - 4)) = 0;
            }
            else if ((v5) && (*(char *)v15 = 0, v5 & 2))
              *(unsigned short *)((int8)v15 + ((uint8)v5 - 2)) = 0;
          }
          memcpy(v10,v8,v13);
          sub_d1f0((int8)v10 + (v13 - 1 & 0xfffffffffffffe00)); // tail-call
          return;
        }
      }
      else {
        *(char *)(v7 + 0x9c) = 0x35;
        if (v3) {
          sub_1ca30("GNU.dumpdir",a0,sub_1fa90(v3));
          sub_12f60(a0,v7,v6); // tail-call
          return;
        }
      }
    }
    else {
      *(char *)(v7 + 0x9c) = 0x35;
      sub_12f60(a0,v7,v6);
    }
    if (dat_84ad4) {
      if (((!dat_84ad1) || (!v9)) || (*(int8 *)(a0->field_0x198 + 0x58) == a0->field_0x58)) {
        v4 = sub_14160(a0,&v14);
        if (v4 != 1) { // branch-flip
          if (v4 == 2) {
            v8 = dcgettext(NULL,"contents not dumped",5);
            sub_12ad0(a0->field_0x0,v14,v8); // tail-call
            return;
          }
          if (v4)
            return;
          v8 = (char *)sub_4c7a0(a0->field_0x0);
          v11 = strlen(v8);
          v13 = v11;
          while (v12 = strlen(a1), v12) {
            v1 = v11 + v12;
            if (v13 < v1) {
              v8 = (char *)sub_4c610(v8,v1 + 1);
              v13 = v1;
            }
            strcpy(&v8[v11],a1);
            if (!sub_16b00(v8,a0))
              sub_14330(a0,a1,v8);
            a1 = &a1[v12 + 1];
          }
        }
        else {
          v8 = dcgettext(NULL,"contents not dumped",5);
          sub_12ad0(a0->field_0x0,v14,v8);
          v13 = strlen(a0->field_0x0);
          v8 = (char *)sub_4c5b0(v13 + 1 + strlen(v14));
          strcpy(v8,a0->field_0x0);
          strcat(v8,v14);
          sub_14330(a0,v14,v8);
        }
        free(v8); // tail-call
        return;
      }
      if ((dat_84a54) && (dat_83b82 & 4)) {
        if (dat_84d50)
          (*dat_84d50)();
        v8 = (char *)sub_4aa70(a0->field_0x0);
        error(0,0,dcgettext(NULL,"%s: file is on a different filesystem; not dumped",5),v8); // tail-call
        return;
      }
    }
  }
  return;
}


// Function: sub_14c40 @ 0x14c40
unsigned long sub_14c40(unsigned long *a0)
{
  char *v1; // rax
  
  v1 = (char *)sub_13fe0(a0);
  if (v1) {
    sub_14710(a0,v1);
    sub_14200(a0);
    free(v1);
    return 1;
  }
  sub_24e60((char *)*a0);
  return 0;
}


// Function: sub_14ca0 @ 0x14ca0
void sub_14ca0(struct_76 *a0,char *a1,char *a2)
{
  stat *v1;
  unsigned long v10;
  int8 v11;
  int8 v12;
  int8 v13;
  char *v14;
  code *v15; // rdx
  unsigned long v16; // stack - 0xe0
  stat v17; // stack - 0xd8
  int4 v18;
  bool v19; // zf
  uint8 v2;
  char v20 [16];
  char v21 [16];
  bool v22; // stack - 0x122
  int4 v23; // stack - 0x100
  int8 v3;
  int8 v4;
  unsigned long v5;
  char v6;
  int4 v7; // eax
  uint4 v8; // eax
  uint4 v9; // eax
  
  v18 = dat_81234;
  v12 = a0->field_0x198;
  v19 = v12 == 0;
  v22 = (bool)dat_84b40;
  if (v19) {
    if (dat_84b40) { // branch-flip
      if (!sub_2ca20("add",a2))
        return;
      sub_23be0(a0,a2);
      sub_23be0(a0->field_0x8,(char *)sub_33020(a2,0,(uint1)dat_84b92));
      sub_30830(a0->field_0x8,1);
      v22 = 0;
    }
    else {
      sub_23be0(a0,a2);
      sub_23be0(a0->field_0x8,(char *)sub_33020(a2,0,(uint1)dat_84b92));
      sub_30830(a0->field_0x8,1);
    }
  }
  else {
    v18 = *(int4 *)(v12 + 0x1a8);
    if ((dat_84b40) && (!sub_2ca20("add",a2)))
      return;
    sub_23be0(a0,a2);
    sub_23be0(a0->field_0x8,(char *)sub_33020(a2,0,(uint1)dat_84b92));
    sub_30830(a0->field_0x8,1);
    if (v18 <= -1) {
      v15 = sub_24dd0;
      *__errno_location() = -v18;
      sub_24ef0(a2,v19,v15); // return-dupe, tail-call
      return;
    }
    v22 = 1;
  }
  v1 = (stat *)&a0->field_0x20[0x38];
  if (fstatat(v18,a1,v1,dat_84800)) {
    v15 = sub_24ec0; // return-dupe
    sub_24ef0(a2,v19,v15);
    return;
  }
  v23 = 0;
  if (sub_11d50(v1)) {
    v23 = sub_140d0(v12,a1,dat_84808);
    if (v23 < 0) {
      v15 = sub_24dd0;
      sub_24ef0(a2,v19,v15);
      return;
    }
    a0->field_0x1a8 = v23;
    if (fstat(v23,v1)) {
      v15 = sub_24ec0;
      sub_24ef0(a2,v19,v15);
      return;
    }
  }
  v13 = a0->field_0x88;
  a0->field_0x118 = v13;
  v20 = sub_4bd00(v1);
  a0->field_0xe8 = SUB168(v20,0);
  a0->field_0xf0 = SUB168(v20,8);
  v20 = sub_4bd20(v1);
  a0->field_0xf8 = SUB168(v20,0);
  a0->field_0x100 = SUB168(v20,8);
  v20 = sub_4bd10(v1);
  v6 = dat_84b50;
  v5 = dat_84b08;
  v10 = dat_84b00;
  a0->field_0x108 = SUB168(v20,0);
  a0->field_0x110 = SUB168(v20,8);
  if (((((dat_84b50 != '\x01') || (v19)) && ((a0->field_0x70 & 0xf000) != 0x4000)) && (v7 = sub_4be80(a0->field_0xf8,a0->field_0x100,dat_84b00,dat_84b08), v7 <= -1)) && ((!dat_84b8c || (v3 = a0->field_0x108, v4 = a0->field_0x110, (int4)sub_4be80(v3,v4,v10,v5) <= -1)))) {
    if (v6)
      return;
    if (!dat_84a54)
      return;
    if (!(dat_83b81 & 1))
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v10 = sub_4aa70(a2);
    v14 = "%s: file is unchanged; not dumped";
    error(0,0,dcgettext(NULL,v14,5),v10); // return-dupe, tail-call
    return;
  }
  if (sub_2ab40(a0)) {
    if (!(dat_83b81 & 4))
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v10 = sub_4aa70(a2);
    v14 = "%s: file is the archive; not dumped";
    error(0,0,dcgettext(NULL,v14,5),v10);
    return;
  }
  v8 = a0->field_0x70 & 0xf000;
  if (v8 != 0x4000) { // branch-flip
    if (sub_13b00(a0))
      return;
    v9 = a0->field_0x70 & 0xf000;
    if (v9 != 0x8000) {
      if (v9 == 0xa000) {
        v14 = (char *)sub_39200(v18,a1,a0->field_0x88);
        a0->field_0x18 = v14;
        if (v14) {
          sub_30830(&a0->field_0x18,4);
          v19 = dat_84ba4 != 2;
          v14 = a0->field_0x18;
          if ((uint8)v19 + 99 < strlen(v14))
            sub_13a40(a0);
          sub_31de0(v18,a1,a0,0);
          sub_31b80(v18,a1,a0,0);
          v12 = sub_d180();
          a0->field_0x88 = 0;
          v13 = sub_132d0(a0);
          if (!v13)
            return;
          sub_11d00(v13 + 0x9d,a0->field_0x18,100);
          *(char *)(v13 + 0x9c) = 0x32;
          sub_12f60(a0,v13,v12);
          if (dat_84ab8)
            sub_30b70(a2,0);
          sub_12760(a0); // tail-call
          return;
        }
        if (*__errno_location() == 0xc)
          sub_16420(); // no-return
        v15 = sub_24e30;
        sub_24ef0(a2,v19,v15);
        return;
      }
      if (v9 != 0x2000) { // branch-flip
        if (v9 != 0x6000) { // branch-flip
          if (v9 != 0x1000) {
            if (v9 == 0xc000) {
              if (!(dat_83b80 & 0x20))
                return;
              if (dat_84d50)
                (*dat_84d50)();
              v10 = sub_4aa70(a2);
              v14 = "%s: socket ignored";
              error(0,0,dcgettext(NULL,v14,5),v10);
              return;
            }
            sub_12640(a2); // return-dupe, tail-call
            return;
          }
          sub_319f0(v18,a1,a0,0,1);
          sub_31de0(v18,a1,a0,0);
          v6 = '6';
          sub_31b80(v18,a1,a0,0);
        }
        else {
          sub_319f0(v18,a1,a0,0,1);
          sub_31de0(v18,a1,a0,0);
          v6 = '4';
          sub_31b80(v18,a1,a0,0);
        }
      }
      else {
        sub_319f0(v18,a1,a0,0,1);
        sub_31de0(v18,a1,a0,0);
        v6 = '3';
        sub_31b80(v18,a1,a0,0);
      }
      if (dat_84ba4 != 1) {
        v12 = sub_d180();
        a0->field_0x88 = 0;
        v13 = sub_132d0(a0);
        if (!v13)
          return;
        *(char *)(v13 + 0x9c) = v6;
        if (v6 != '6') {
          v10 = a0->field_0x80;
          sub_12910(gnu_dev_major(v10),(char *)(v13 + 0x149));
          v10 = a0->field_0x80;
          sub_128e0(gnu_dev_minor(v10),(char *)(v13 + 0x151));
        }
        sub_12f60(a0,v13,v12);
        if (!dat_84ab8)
          return;
        sub_30b70(a2,0); // tail-call
        return;
      }
      sub_12640(a2);
      return;
    }
    sub_319f0(v18,a1,a0,0,1);
    sub_31de0(v18,a1,a0,v23);
    sub_31b80(v18,a1,a0,v23);
    if ((v23) && (dat_84a88)) {
      v2 = a0->field_0x88;
      v11 = v2;
      if ((int8)v2 <= -1)
        v11 = v2 + 0x1ff;
      v12 = v11 >> 9;
      if ((v2 & 0x1ff) && (0x3ff <= v2 + 0x1ff))
        v12 += 1;
      if ((v12 <= a0->field_0x98) || (v9 = sub_29cd0(v23,a0), v9 == 3)) goto label_153a7;
      if (2 <= v9) {
        sub_2f670(a0); // return-dupe
        return;
      }
    }
    else {
label_153a7:
      v9 = sub_13d10(v23,a0);
      if (2 <= v9) {
        if (v9 == 3)
          abort(); // no-return
        sub_2f670(a0);
        return;
      }
    }
    sub_12760(a0);
    if (v9) {
      sub_2f670(a0);
      return;
    }
label_153ca:
    if (v23) // branch-flip
      v7 = fstat(v23,&v17);
    else {
      if ((v18 <= -1) && (v22)) {
        *__errno_location() = -v18;
        goto label_1500a;
      }
      v7 = fstatat(v18,a1,&v17,dat_84800);
    }
    if (!v7) {
      v21 = sub_4bd10(&v17);
      if (((sub_4be80(SUB168(v21,0),SUB168(v21,8),SUB168(v20,0),SUB168(v20,8))) && ((dat_84ab8 != '\x01' || (v8 != 0x4000)))) || (v13 < v17._48_8_)) {
        if (dat_83b80 & 0x10) {
          if (dat_84d50)
            (*dat_84d50)();
          v14 = (char *)sub_4aa70(a2);
          error(0,0,dcgettext(NULL,"%s: file changed as we read it",5),v14);
        }
        sub_2f7d0(1);
      }
      else if ((((dat_84b88 == 1) && (v23)) && ((v13 || (v8 == 0x4000)))) && (v10 = a0->field_0xe8, v5 = a0->field_0xf0, sub_24a80(v23,v18,a1,v10,v5)))
        sub_32dd0(a2);
      if (!sub_2f670(a0))
        return;
      if (!dat_84ab8)
        return;
      sub_30b70(a2,v8 == 0x4000);
      return;
    }
  }
  else {
    sub_319f0(v18,a1,a0,0,0);
    sub_31de0(v18,a1,a0,v23);
    sub_31b80(v18,a1,a0,v23);
    sub_126e0(a0);
    sub_126e0(a0->field_0x8);
    if (sub_14160(a0,&v16) == 3) {
      v14 = dcgettext(NULL,"directory not dumped",5);
      sub_12ad0(a0->field_0x0,v16,v14); // tail-call
      return;
    }
    v6 = sub_14c40(a0);
    v23 = a0->field_0x1a8;
    v18 = dat_81234;
    if (v12)
      v18 = *(int4 *)(v12 + 0x1a8);
    if (!v6) {
      sub_2f670(a0);
      return;
    }
    if (0 <= v23) goto label_153ca;
    *__errno_location() = -v23;
  }
label_1500a:
  sub_24ef0(a2,v19,sub_24ec0);
  sub_2f670(a0);
  return;
}


// Function: sub_15860 @ 0x15860
void sub_15860(void)
{
  int8 v1; // rax
  int8 v2;
  
  sub_d1f0(dat_84408);
  v2 = dat_848c8 + 0x3fe;
  if (0 <= dat_848c8 + 0x1ff)
    v2 = dat_848c8 + 0x1ff;
  v1 = dat_83f50 - dat_83f48;
  v2 >>= 9;
  if (v1 >> 9 <= v2) {
    do {
      v2 -= v1 >> 9;
      sub_dfd0();
      v1 = dat_83f50 - dat_83f48;
    } while (v1 >> 9 <= v2);
  }
  dat_83f48 += v2 * 0x200;
  return;
}


// Function: sub_158f0 @ 0x158f0
void sub_158f0(int8 a0)
{
  int4 v1;
  int8 v2; // rax
  uint8 v3; // rax
  int8 v4; // rbx
  unsigned short v5 [2]; // stack - 0x28
  int8 v6;
  bool v7;
  int4 v8; // stack - 0x24
  
  if (!a0)
    return;
  v8 = (int4)a0;
  if (0 <= a0) { // branch-flip
    v5[0] = 3;
    v7 = v8 == a0;
  }
  else {
    v8 = -v8;
    v5[0] = 4;
    v7 = v8 + a0 == 0;
  }
  v1 = dat_84a34;
  if (v7) {
    if (0x40000000 <= dat_84a34) // branch-flip
      v1 = sub_34080(dat_84a34 + -0x40000000,0x40086d01,v5);
    else {
      v1 = ioctl(dat_84a34,0x40086d01,v5);
    }
    if (0 <= v1)
      return;
    v1 = dat_84a34;
    if (*__errno_location() == 5) {
      if (0x40000000 <= dat_84a34) // branch-flip
        v1 = sub_34080(dat_84a34 + -0x40000000,0x40086d01,v5);
      else {
        v1 = ioctl(dat_84a34,0x40086d01,v5);
      }
      if (0 <= v1)
        return;
      v1 = dat_84a34;
    }
  }
  if (0x40000000 <= v1) // branch-flip
    v2 = sub_33f90(v1 + -0x40000000,0,1);
  else {
    v2 = lseek(v1,0,1);
  }
  v4 = a0 * dat_84b98;
  v6 = v2 + v4;
  if ((SEXT816(v4) == SEXT816(a0) * SEXT816(dat_84b98)) && (v6 < v2 == (bool)-(char)(v4 >> 0x3f))) {
    if (v6 <= -1)
      v6 = 0;
    if (0x40000000 <= dat_84a34) // branch-flip
      v3 = sub_33f90(dat_84a34 + -0x40000000,v6,0);
    else {
      v3 = lseek(dat_84a34,v6,0);
    }
    if (v3 == v6)
      return;
  }
  sub_32b20((char *)*dat_84838); // tail-call
  return;
}


// Function: sub_15b10 @ 0x15b10
void sub_15b10(int4 a0)
{
  unsigned long v1;
  
  v1 = dat_83f58;
  dat_83f58 = dat_842e8;
  if (dat_842e0) { // branch-flip
    dat_84a34 = 1;
    sub_fb50();
    dat_84a34 = 0;
  }
  else {
    sub_158f0((dat_842d8 + dat_83f30) - dat_83f38);
    sub_fb50();
  }
  dat_83f58 = v1;
  if ((a0) && (!dat_842e0))
    sub_158f0(dat_83f38 - (dat_842d8 + dat_83f30));
  dat_842e4 = 0;
  return;
}


// Function: sub_15bc0 @ 0x15bc0
void sub_15bc0(int8 a0,int8 a1)
{
  unsigned long *v1;
  unsigned long *v2;
  int8 v3; // rcx
  unsigned long *v4;
  uint8 v5; // rcx
  int8 v6; // rbx
  unsigned long *v7;
  
  if (!a1)
    return;
  v6 = 0;
  do {
    while( true ) {
      v3 = (int8)dat_842e4;
      v7 = (unsigned long *)(v6 * 0x200 + a0);
      dat_842e4 += 1;
      v4 = (unsigned long *)(v3 * 0x200 + dat_842e8);
      *v4 = *v7;
      v4[0x3f] = v7[0x3f];
      v3 = (int8)v4 - (int8)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      v5 = (uint8)((int4)v3 + 0x200U >> 3);
      v7 = (unsigned long *)((int8)v7 - v3);
      v4 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      while (v5) {
        v2 = &v4[1];
        v1 = &v7[1];
        *v4 = *v7;
        v5 -= 1;
        v7 = v1;
        v4 = v2;
      }
      if (dat_842e4 == dat_84ba0) break;
      v6 += 1;
      if (a1 == v6)
        return;
    }
    v6 += 1;
    sub_15b10(1);
  } while (a1 != v6);
  return;
}


// Function: sub_15c70 @ 0x15c70
void sub_15c70(int8 a0,uint8 a1)
{
  unsigned long *v1;
  unsigned long *v2;
  unsigned long *v3;
  int8 v4; // rcx
  uint8 v5; // rcx
  uint8 v6; // rcx
  unsigned long *v7;
  unsigned long *v8;
  uint4 v9; // ecx
  
  v9 = (uint4)a1 & 0x1ff;
  v6 = (uint8)v9;
  sub_15bc0(a0,a1 >> 9);
  v7 = (unsigned long *)(a0 + (a1 & 0xfffffffffffffe00));
  v3 = (unsigned long *)((int8)dat_842e4 * 0x200 + dat_842e8);
  if (8 <= v9) { // branch-flip
    *v3 = *v7;
    *(unsigned long *)((int8)v3 + (v6 - 8)) = *(unsigned long *)((v6 - 8) + (int8)v7);
    v4 = (int8)v3 - (int8)(unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8);
    v5 = (uint8)((int4)v4 + v9 >> 3);
    v7 = (unsigned long *)((int8)v7 - v4);
    v8 = (unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8);
    while (v5) {
      v2 = &v8[1];
      v1 = &v7[1];
      *v8 = *v7;
      v5 -= 1;
      v7 = v1;
      v8 = v2;
    }
  }
  else if (a1 & 4) { // branch-flip
    *(unsigned int *)v3 = *(unsigned int *)v7;
    *(unsigned int *)((int8)v3 + (v6 - 4)) = *(unsigned int *)((v6 - 4) + (int8)v7);
  }
  else if ((a1 & 0x1ff) && (*(char *)v3 = *(char *)v7, a1 & 2))
    *(unsigned short *)((int8)v3 + (v6 - 2)) = *(unsigned short *)((v6 - 2) + (int8)v7);
  v3 = (unsigned long *)((int8)v3 + v6);
  v6 = 0x200 - v6;
  v9 = (uint4)v6;
  if (8 <= v9) { // branch-flip
    *v3 = 0;
    *(unsigned long *)((int8)v3 + ((v6 & 0xffffffff) - 8)) = 0;
    v6 = (uint8)(v9 + ((int4)v3 - (int4)(unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8)) >> 3);
    v7 = (unsigned long *)((uint8)&v3[1] & 0xfffffffffffffff8);
    while (v6) {
      v3 = &v7[1];
      *v7 = 0;
      v6 -= 1;
      v7 = v3;
    }
  }
  else if (v6 & 4) { // branch-flip
    *(unsigned int *)v3 = 0;
    *(unsigned int *)((int8)v3 + ((v6 & 0xffffffff) - 4)) = 0;
  }
  else if ((v9) && (*(char *)v3 = 0, v6 & 2))
    *(unsigned short *)((int8)v3 + ((v6 & 0xffffffff) - 2)) = 0;
  dat_842e4 += 1;
  if (dat_842e4 != dat_84ba0)
    return;
  sub_15b10(1); // tail-call
  return;
}


// Function: sub_163d0 @ 0x163d0
void sub_163d0(void)
{
  if (dat_842f0)
    (*dat_842f0)();
  error(2,0,dcgettext(NULL,"Error is not recoverable: exiting now",5));
  return;
}


// Function: sub_16420 @ 0x16420
void sub_16420(void)
{
  error(0,0,"%s",dcgettext(NULL,"memory exhausted",5));
  sub_163d0(); // no-return
}


// Function: sub_16460 @ 0x16460
void sub_16460(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x842f8;
  *a0 = 0x8000000;
  return;
}


// Function: sub_16480 @ 0x16480
int8 sub_16480(char *a0)
{
  char *v1;
  int8 v2; // r12
  
  v2 = 0x81160;
  v1 = dat_81160;
  if (dat_81160) {
    while (strcmp(v1,a0)) {
      v1 = *(char **)(v2 + 0x28);
      v2 += 0x28;
      if (!v1)
        return v2;
    }
  }
  return v2;
}


// Function: sub_164d0 @ 0x164d0
void sub_164d0(int8 *a0,char *a1,uint4 a2)
{
  char v1;
  int8 *v2; // rax
  
  v2 = __ctype_b_loc();
  while (v1 = *a1, *(uint1 *)(*v2 + 1 + (int8)v1 * 2) & 0x20) {
    a1 = &a1[1];
  }
  if ((v1) && (v1 != '#')) {
    if (v1 == '!') {
      if (a1[1] != '!') { // branch-flip
        a1 = &a1[1];
        a2 |= 0x20000000;
      }
      else {
        a1 = &a1[2];
      }
    }
    if (!strncmp(a1,"RE:",3)) {
      a1 = &a1[3];
      a2 = a2 & 0xefffffff | 0x8000000;
    }
    sub_3ee30(a0,a1,a2); // tail-call
    return;
  }
  return;
}


// Function: sub_16580 @ 0x16580
void sub_16580(int8 *a0,char *a1,unsigned int a2)
{
  char v1;
  int8 *v2; // rax
  
  v2 = __ctype_b_loc();
  while( true ) {
    v1 = *a1;
    if (!(*(uint1 *)(*v2 + 1 + (int8)v1 * 2) & 0x20)) break;
    a1 = &a1[1];
  }
  if ((v1) && (v1 != '#')) {
    if (v1 == '\\')
      a1 = &a1[a1[1] == '#'];
    sub_3ee30(a0,a1,a2); // tail-call
    return;
  }
  return;
}


// Function: sub_16600 @ 0x16600
void sub_16600(unsigned long a0,char *a1,uint4 a2,uint4 *a3)
{
  char *v1;
  char v2;
  uint1 v3;
  int8 v4;
  uint4 v5;
  uint8 v6; // rax
  char *v7;
  
  v4 = *(int8 *)__ctype_b_loc();
  while (v2 = *a1, *(uint1 *)(v4 + 1 + (int8)v2 * 2) & 0x20) {
    a1 = &a1[1];
  }
  if ((v2) && (v2 != '#')) {
    if (strncmp(a1,"syntax:",7)) {
      v6 = strlen(a1);
      v7 = a1;
      if (a1[v6 - 1] == '/') {
        a2 |= 0x4000008;
        v7 = (char *)sub_4c5b0(v6);
        memcpy(v7,a1,v6 - 1);
        v7[v6 - 1] = '\0';
        sub_3eaa0(a0,v7);
      }
      v5 = a2 & 0xf7ffffff;
      if (*a3 == 0x8000000)
        v5 = a2 & 0xefffffff;
      sub_3ee30(a0,v7,*a3 | v5); // tail-call
      return;
    }
    v7 = &a1[7];
    v3 = *(uint1 *)(v4 + 1 + (int8)a1[7] * 2);
    while (v3 & 0x20) {
      v1 = &v7[1];
      v7 = &v7[1];
      v3 = *(uint1 *)(v4 + 1 + (int8)*v1 * 2);
    }
    if (strcmp(v7,"regexp")) { // branch-flip
      if (!strcmp(v7,"glob"))
        *a3 = 0x10000000;
    }
    else {
      *a3 = 0x8000000;
    }
  }
  return;
}


// Function: sub_16780 @ 0x16780
void sub_16780(int8 *a0,char *a1,unsigned int a2) // return-dupe
{
  int8 v1;
  uint8 v2; // rbx
  uint8 v3; // stack - 0x118
  int8 v4; // stack - 0x110
  
  if (sub_382c0(a1,&v3,0x844))
    return;
  if (v3) {
    v2 = 0;
    do {
      v1 = v2 * 8;
      v2 += 1;
      sub_3ee30(a0,*(char **)(v4 + v1),a2);
    } while (v2 < v3);
  }
  sub_35420(&v3);
  return;
}


// Function: sub_16820 @ 0x16820
void sub_16820(char *a0,unsigned int a1)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_4c5b0(strlen(a0) + 0x10);
  *v1 = 0;
  *(unsigned int *)&v1[1] = a1;
  strcpy((char *)((int8)v1 + 0xc),a0);
  if (dat_84300) {
    *dat_84300 = v1;
    dat_84300 = v1;
    return;
  }
  dat_84308 = v1;
  dat_84300 = v1;
  return;
}


// Function: sub_16890 @ 0x16890
void sub_16890(int8 a0)
{
  char *v1;
  char *v10; // rax
  unsigned int v11;
  int8 *v12;
  unsigned long *v13;
  unsigned long *v14; // stack - 0x40
  code *v2;
  int8 v3;
  int4 v4;
  FILE *v5; // rax
  int8 v6; // rax
  int8 v7; // rax
  unsigned long v8;
  unsigned long *v9; // rax
  
  if (*(int8 *)(a0 + 0x1b0))
    return;
  v14 = NULL;
  if (dat_84308) {
    v12 = dat_84308;
    v13 = NULL;
label_168ec:
    do {
      v1 = (char *)((int8)v12 + 0xc);
      v4 = *(int4 *)(a0 + 0x1a8);
      if (!faccessat(v4,v1,0,0)) {
        v4 = sub_140d0(a0,v1,0);
        if (v4 != -1) { // branch-flip
          v5 = fdopen(v4,"r");
          if (v5) { // branch-flip
            v6 = sub_3eba0();
            v7 = sub_16480(v1);
            if (*(code **)(v7 + 0x18)) { // branch-flip
              v8 = (**(code **)(v7 + 0x18))(*(unsigned long *)(v7 + 0x20));
              *(unsigned long *)(v7 + 0x20) = v8;
            }
            else {
              v8 = *(unsigned long *)(v7 + 0x20);
            }
            v2 = *(code **)(v7 + 0x10);
            if (sub_3f0c0(v2,v6,v5,0x50000001,10,v8)) {
              v4 = *__errno_location();
              if (dat_84d50)
                (*dat_84d50)();
              error(0,v4,"%s",(char *)sub_4aa70(v1));
              sub_163d0(); // no-return
            }
            fclose(v5);
            v9 = (unsigned long *)sub_4c5b0(0x20);
            v3 = v12[1];
            v9[3] = v6;
            v11 = 0;
            if ((int4)v3)
              v11 = *(unsigned int *)(v7 + 8);
            *(unsigned int *)&v9[2] = v11;
            v9[1] = v13;
            *v9 = 0;
            if (v13) {
              v12 = (int8 *)*v12;
              *v13 = v9;
              v13 = v9;
              if (!v12) break;
              goto label_168ec;
            }
            v14 = v9;
            v13 = v9;
          }
          else {
            if (dat_84d50)
              (*dat_84d50)();
            v10 = dcgettext(NULL,"%s: fdopen failed",5);
            error(0,*__errno_location(),v10,v1);
            dat_84d58 = 2;
            close(v4);
          }
        }
        else {
          sub_327c0(v1);
        }
      }
      v12 = (int8 *)*v12;
    } while (v12);
  }
  *(unsigned long **)(a0 + 0x1b0) = v14;
  return;
}


// Function: sub_16ab0 @ 0x16ab0
void sub_16ab0(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 0x1b0);
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_3ebb0((struct_15 *)v2[3]);
    free(v2);
    v2 = v1;
  }
  *(unsigned long *)(a0 + 0x1b0) = 0;
  return;
}


// Function: sub_16b00 @ 0x16b00
char sub_16b00(char *a0,int8 a1)
{
  char v1;
  unsigned long *v2;
  char v3; // al
  char *v4; // rax
  char *v5;
  uint4 v6; // r12d
  char *v7;
  char v8; // stack - 0x39
  
  v3 = sub_3ed70(dat_84b60,a0);
  v8 = v3;
  if ((!v3) && (a1)) {
    v6 = 0;
    v5 = NULL;
    v7 = NULL;
    do {
      for (v2 = *(unsigned long **)(a1 + 0x1b0); v2; v2 = (unsigned long *)*v2) {
        if (!(v6 & *(uint4 *)&v2[2])) {
          v3 = sub_3ed70((int8 *)v2[3],a0);
          if (!v3) {
            if (!v7) {
              v1 = *a0;
              v7 = a0;
              while ((v1 == '.' && (v7[1] == '/'))) {
                v7 = &v7[2];
                v1 = *v7;
              }
            }
            v3 = sub_3ed70((int8 *)v2[3],v7);
            if (!v3) {
              if (v5) // branch-flip
                v3 = sub_3ed70((int8 *)v2[3],v5);
              else {
                v4 = (char *)sub_3e510(a0);
                v5 = v4;
                v3 = sub_3ed70((int8 *)v2[3],v4);
              }
              if (!v3) goto label_16b70;
            }
          }
          v8 = '\x01';
          free(v5); // return-dupe
          v3 = v8;
          return v3;
        }
label_16b70:
      }
      a1 = *(int8 *)(a1 + 0x198);
      v6 = 2;
    } while (a1);
    free(v5);
  }
  v3 = v8;
  return v3;
}


// Function: sub_16c40 @ 0x16c40
void sub_16c40(void)
{
  int8 *v1; // rbx
  int8 v2;
  
  if (dat_81160) {
    v1 = (int8 *)0x81160;
    v2 = dat_81160;
    do {
      v1 = &v1[5];
      sub_16820(v2,0);
      v2 = *v1;
    } while (v2);
    return;
  }
  return;
}


// Function: sub_16c80 @ 0x16c80
uint4 sub_16c80(int4 a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (uint4)CONCAT71((undefined7)((uint8)v1 >> 8),a0 != 0x26) & CONCAT31((undefined3)((uint4)a2 >> 8),a0 != 0x5f);
}


// Function: sub_16c90 @ 0x16c90
void sub_16c90(void *a0)
{
  free(*(void **)((int8)a0 + 0xa0));
  sub_1c600(*(unsigned long **)((int8)a0 + 0x90),*(int8 *)((int8)a0 + 0x88));
  free(*(void **)((int8)a0 + 0x60));
  free(*(void **)((int8)a0 + 0x68));
  free(*(void **)((int8)a0 + 0x78));
  free(a0); // tail-call
  return;
}


// Function: sub_16ce0 @ 0x16ce0
unsigned int sub_16ce0(char *a0)
{
  int4 v1;
  unsigned int v2;
  int4 *v3; // rax
  stat v4; // stack - 0xc8
  
  v3 = __errno_location();
  v1 = *v3;
  if (((fstatat(dat_81234,a0,&v4,0x100)) || ((v4._24_4_ & 0xf000) != 0xa000)) || (fstatat(dat_81234,a0,&v4,0)))
    v2 = 0;
  else {
    v2 = CONCAT31((undefined3)((v4._24_4_ & 0xf000) >> 8),(v4._24_4_ & 0xf000) == 0x4000);
  }
  *v3 = v1;
  return v2;
}


// Function: sub_16da0 @ 0x16da0
int8 * sub_16da0(char *a0) // return-dupe x2
{
  stat v1; // stack - 0xb8
  int8 *v2;
  
  v2 = dat_84318;
  if (!dat_84318)
    return NULL;
  if (!fstatat(dat_81234,a0,&v1,0x100)) {
    while ((v2[1] != v1._0_8_ || (v2[2] != v1._8_8_))) {
      v2 = (int8 *)*v2;
      if (!v2)
        return NULL;
    }
    return v2;
  }
  if (*__errno_location() == 2)
    return NULL;
  sub_32d30(a0);
  return NULL;
}


// Function: sub_16e60 @ 0x16e60
void sub_16e60(char *a0,struct_3 *a1)
{
  int4 v1;
  unsigned long *v2;
  char *v3;
  stat v4; // stack - 0xc8
  
  v2 = dat_84320;
  while( true ) {
    if (!v2) {
      if (dat_84d50)
        (*dat_84d50)();
      v3 = (char *)sub_4aa70(a0);
      error(0,0,dcgettext(NULL,"%s: Unexpected inconsistency when making directory",5),v3);
      dat_84d58 = 2;
      return;
    }
    v1 = *(int4 *)((int8)v2 + 0x54);
    v3 = (char *)v2[0x14];
    if (fstatat(dat_81234,v3,&v4,v1)) break;
    if ((v4._0_8_ == a1->field_0x0) && (v4._8_8_ == a1->field_0x8)) {
      v2[1] = dat_84898;
      v2[2] = dat_848a0;
      *(unsigned int *)&v2[3] = dat_848b0;
      *(unsigned int *)((int8)v2 + 0x1c) = dat_848b4;
      *(unsigned int *)((int8)v2 + 0x4c) = 0xffffffff;
      *(unsigned int *)&v2[4] = dat_848b8;
      *(char *)&v2[10] = 0;
      *(unsigned int *)&v2[9] = v4._24_4_;
      v2[5] = dat_84928;
      v2[6] = dat_84930;
      v2[7] = dat_84938;
      v2[8] = dat_84940;
      return;
    }
    v2 = (unsigned long *)*v2;
  }
  sub_32d30((char *)v2[0x14]);
  return;
}


// Function: sub_16fd0 @ 0x16fd0
void sub_16fd0(char *a0,int8 a1,int8 a2) // return-dupe x4
{
  int4 v1; // eax
  char *v2; // rax
  char *v3; // rax
  int8 v4; // stack - 0x68
  char v5 [40];
  int8 v6; // r14
  uint8 v7; // r15
  int8 v8; // stack - 0x60
  
  if (a1 < 0) {
    if (!(dat_83b81 & 0x80))
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v2 = (char *)sub_22080(a1,a2,1);
    error(0,0,dcgettext(NULL,"%s: implausibly old time stamp %s",5),a0,v2); // tail-call
    return;
  }
  v1 = sub_4be80(dat_84a10,dat_84a18,a1,a2);
  if (0 <= v1)
    return;
  sub_411a0(&v4);
  v1 = sub_4be80(v4,v8,a1,a2);
  if (0 <= v1)
    return;
  v7 = a1 - v4;
  v6 = a2 - v8;
  if (v6 < 0) {
    v6 += 1000000000;
    v7 -= 1;
  }
  if (!(dat_83b81 & 0x80))
    return;
  if (dat_84d50)
    (*dat_84d50)();
  v2 = (char *)sub_24270(v7,v6,v5);
  v3 = (char *)sub_22080(a1,a2,1);
  error(0,0,dcgettext(NULL,"%s: time stamp %s is %s s in the future",5),a0,v3,v2);
  return;
}


// Function: sub_17180 @ 0x17180
int4 sub_17180(int4 a0,char *a1,uint4 a2,uint4 a3,int4 a4)
{
  uint4 v1; // eax
  int4 v2;
  
  if (0 <= a0) {
    v1 = fchown(a0,a2,a3);
    if (v1) {
      v2 = *__errno_location();
      if (!sub_16c80(v2)) {
        v2 = fchownat(dat_81234,a1,a2,a3,a4); // return-dupe, tail-call
        return v2;
      }
    }
    return (uint8)v1;
  }
  v2 = fchownat(dat_81234,a1,a2,a3,a4);
  return v2;
}


// Function: sub_171f0 @ 0x171f0
void sub_171f0(int4 a0,char *a1,stat *a2,int4 a3)
{
  if (0 <= a0) {
    fstat(a0,a2); // tail-call
    return;
  }
  fstatat(dat_81234,a1,a2,a3); // tail-call
  return;
}


// Function: sub_17210 @ 0x17210
int4 sub_17210(int4 a0,char *a1,uint4 a2,int4 a3)
{
  uint4 v1; // eax
  int4 v2;
  
  if (0 <= a0) {
    v1 = fchmod(a0,a2);
    if (v1) {
      v2 = *__errno_location();
      if (!sub_16c80(v2)) {
        v2 = fchmodat(dat_81234,a1,a2,a3); // return-dupe, tail-call
        return v2;
      }
    }
    return (uint8)v1;
  }
  v2 = fchmodat(dat_81234,a1,a2,a3);
  return v2;
}


// Function: sub_17280 @ 0x17280
unsigned long sub_17280(uint8 a0,char *a1,uint8 a2,uint8 a3,int4 a4) // return-dupe
{
  char v1; // al
  int4 *v2; // rax
  int4 v3;
  bool v4; // zf
  bool v5; // zf
  
  v4 = a4 != 0x32;
  v5 = (int4)a3 != 0;
  if (sub_17210(a0,a1,a2,a3)) { // branch-flip
    v2 = __errno_location();
    v3 = *v2;
    if (v3 != 1) {
label_172e3:
      if (v4 && v5) {
        v1 = sub_16c80(v3);
        if (v1) {
          if (!v3)
            return 0;
          goto label_172bc;
        }
        goto label_1732b;
      }
      goto label_172e8;
    }
    if ((a2 & 0x800) && (!sub_48750())) {
      if (!sub_17210(a0 & 0xffffffff,a1,a2 & 0xffffffff,a3 & 0xffffffff)) goto label_172b5;
      v3 = *v2;
      goto label_172e3;
    }
    if (v4 && v5) {
      if (sub_16c80(1)) goto label_172bc;
      goto label_1732b;
    }
  }
  else {
label_172b5:
    if (!v4 || !v5)
      return 0;
    if (sub_16c80(0))
      return 0;
label_1732b:
    if (!sub_17210(a0 & 0xffffffff,a1,a2 & 0xffffffff,0))
      return 0;
    v3 = *__errno_location();
label_172e8:
    if (!v3)
      return 0;
  }
  if (a4 != 0x32) // branch-flip
    v2 = __errno_location();
  else {
    if (!sub_16c80(v3))
      return 0;
    v2 = __errno_location();
  }
label_172bc:
  *v2 = v3;
  return 0xffffffff;
}


// Function: sub_17410 @ 0x17410
void sub_17410(char *a0,uint4 a1,uint4 a2,unsigned int a3,uint4 a4,uint4 a5,unsigned int a6,unsigned int a7) // return-dupe x3
{
  unsigned int v1;
  unsigned int v2;
  int4 v3; // eax
  char v4 [24];
  uint4 v5; // stack - 0xc0
  
  v2 = a7;
  v1 = a6;
  if (!((a4 ^ a1 | ~a5) & a2))
    return;
  if ((~(a5 | a2) & 0xfff) && (v3 = sub_171f0(a3,a0,v4,a7), a4 = v5, v3)) {
    sub_32d30(a0);
    return;
  }
  a2 = (a1 ^ a4 & 0xfff) & a2;
  if (!a2)
    return;
  a2 = a4 & 0xfff ^ a2;
  if (!sub_17280(a3,a0,a2,v2,(int4)(char)v1))
    return;
  sub_32550(a0,a2); // tail-call
  return;
}


// Function: sub_17530 @ 0x17530
void sub_17530(char *a0,int8 a1,unsigned int a2,uint4 a3,uint4 a4,char a5,unsigned int a6,unsigned int a7)
{
  unsigned int v1;
  int8 v10; // stack - 0x58
  int8 v11; // stack - 0x50
  unsigned int v2;
  unsigned int v3;
  int4 v4;
  unsigned long v5; // rdx
  unsigned long v6; // stack - 0x68
  char v7; // r10b
  int4 v8; // r13d
  int8 v9; // stack - 0x60
  
  v3 = a7;
  v8 = (int4)a5;
  v7 = (char)a6;
  if (dat_84a68) { // branch-flip
    if (dat_84aa8 <= 0) goto label_17738;
    if (!v7) goto label_176aa;
  }
  else {
    v5 = 0x1ff;
    if (v7) goto label_17588;
    if (dat_84b50) { // branch-flip
      v6 = *(unsigned long *)(a1 + 0xe8);
      v9 = *(int8 *)(a1 + 0xf0);
    }
    else {
      v9 = 0x3ffffffe;
    }
    v10 = *(int8 *)(a1 + 0xf8);
    v11 = *(int8 *)(a1 + 0x100);
    v4 = sub_3f360(a2,dat_81234,a0,&v6);
    if (v4) { // branch-flip
      if (a5 == '2') {
        v4 = *__errno_location();
        if (!sub_16c80(v4)) goto label_1769a;
      }
      sub_32dd0(a0);
    }
    else {
      if (dat_84b50)
        sub_16fd0(a0,v6,v9);
      sub_16fd0(a0,v10,v11);
    }
label_1769a:
    if (1 <= dat_84aa8) {
label_176aa:
      v1 = *(unsigned int *)(a1 + 0x74);
      v2 = *(unsigned int *)(a1 + 0x78);
      if (sub_17180(a2,a0,v1,v2)) { // branch-flip
        if (a5 == '2') {
          v4 = *__errno_location();
          if (!sub_16c80(v4)) goto label_176fb;
        }
        sub_32600(a0,v1,v2);
      }
      else if ((~a4 | a3) & 0x49)
        a4 &= ~(a3 & 0xc00);
label_176fb:
      v5 = 0xfff;
      if (dat_84aa4 <= 0)
        v5 = 0x1ff;
      goto label_17588;
    }
label_17738:
    if ((1 <= dat_84aa4) && (!v7)) {
      v5 = 0xfff;
      goto label_17588;
    }
  }
  v5 = 0x1ff;
label_17588:
  sub_17410(a0,~dat_84328 & *(uint4 *)(a1 + 0x70),v5,a2,a3,a4,v8);
  sub_31f20(a1,a0,v8,1);
  sub_31a70(a1,a0,v8);
  sub_31e70(a1,a0,v8,v3); // tail-call
  return;
}


// Function: sub_17870 @ 0x17870
void sub_17870(char *a0,uint1 a1) // return-dupe
{
  int4 v1;
  undefined4 v10;
  unsigned int v11;
  unsigned long v12; // stack - 0x1c8
  unsigned long v13; // stack - 0x1c0
  unsigned long v14; // stack - 0x1b8
  unsigned long v15; // stack - 0x1b0
  unsigned long v16; // stack - 0x1a8
  unsigned int v17; // stack - 0x188
  unsigned int v18; // stack - 0x184
  unsigned int v19; // stack - 0x180
  uint8 v2;
  unsigned long v20; // stack - 0x110
  unsigned long v21; // stack - 0x108
  unsigned long v22; // stack - 0x100
  unsigned long v23; // stack - 0xf8
  unsigned long v24; // stack - 0xa0
  unsigned long v25; // stack - 0x98
  void *v3;
  unsigned long *v4;
  uint8 v5; // rax
  char *v6;
  uint1 v7; // bl
  char v8 [48];
  stat v9; // stack - 0x288
  
  v5 = strlen(a0);
  if (!dat_84320)
    return;
  v7 = 0;
  do {
    v4 = dat_84320;
    v10 = *(unsigned int *)&dat_84320[9];
    v11 = *(unsigned int *)((int8)dat_84320 + 0x4c);
    v7 |= *(uint1 *)&dat_84320[0xb];
    if (a1 < *(uint1 *)&dat_84320[0xb])
      return;
    v2 = dat_84320[0x13];
    if ((((v2 < v5) && (a0[v2])) && ((a0[v2] == '/' || (a0[v2 - 1] == '/')))) && (v3 = (void *)dat_84320[0x14], !memcmp(a0,v3,v2)))
      return;
    sub_24c80(*(unsigned int *)((int8)v4 + 0x5c));
    if (v7) { // branch-flip
      v1 = *(int4 *)((int8)v4 + 0x54);
      v6 = (char *)v4[0x14];
      if (fstatat(dat_81234,v6,&v9,v1)) // branch-flip
        sub_32d30((char *)v4[0x14]);
      else {
        if ((v9._0_8_ == v4[1]) && (v9._8_8_ == v4[2])) {
          v11 = 0xffffffff;
          v10 = v9._24_4_;
          goto label_178c0;
        }
        if (dat_84d50)
          (*dat_84d50)();
        v6 = (char *)sub_4aa70((char *)v4[0x14]);
        error(0,0,dcgettext(NULL,"%s: Directory renamed before its status could be extracted",5),v6);
        dat_84d58 = 2;
      }
    }
    else {
label_178c0:
      v17 = *(unsigned int *)&v4[3];
      v20 = v4[5];
      v21 = v4[6];
      v22 = v4[7];
      v23 = v4[8];
      v18 = *(unsigned int *)((int8)v4 + 0x1c);
      v19 = *(unsigned int *)&v4[4];
      v12 = v4[0xc];
      v13 = v4[0xd];
      v14 = v4[0xe];
      v15 = v4[0xf];
      v16 = v4[0x10];
      v25 = v4[0x12];
      v24 = v4[0x11];
      sub_17530((char *)v4[0x14],v8,0xffffffff,v10,v11,0x35,(uint1)*(char *)&v4[10],*(unsigned int *)((int8)v4 + 0x54));
    }
    dat_84320 = (unsigned long *)*v4;
    sub_16c90(v4);
    if (!dat_84320)
      return;
  } while( true );
}


// Function: sub_17b10 @ 0x17b10
void sub_17b10(unsigned long *a0)
{
  char *v1;
  int8 v2; // stack - 0xb8
  unsigned long v3; // stack - 0xb0
  
  do {
    *(char *)&a0[0xb] = 1;
    v1 = (char *)a0[0x14];
    if (sub_246f0(v1,&v2)) { // branch-flip
      sub_32d30((char *)a0[0x14]);
      a0 = (unsigned long *)*a0;
    }
    else {
      a0[1] = v2;
      a0[2] = v3;
      a0 = (unsigned long *)*a0;
    }
  } while ((a0) && (!*(char *)&a0[0xb]));
  return;
}


// Function: sub_17bb0 @ 0x17bb0
void sub_17bb0(char *a0,char *a1) // return-dupe x2
{
  char *v1;
  int4 v2;
  int8 *v3;
  
  v2 = dat_84580;
  v3 = dat_84320;
  if (!dat_84320)
    return;
  do {
    if (*(int4 *)((int8)v3 + 0x5c) == v2) {
      v1 = (char *)v3[0x14];
      if (!strcmp(v1,a0)) {
        free(v1);
        v3[0x14] = sub_4c7a0(a1);
        v3[0x13] = strlen(a1);
        return;
      }
    }
    v3 = (int8 *)*v3;
  } while (v3);
  return;
}


// Function: sub_17c30 @ 0x17c30
void sub_17c30(char *a0,int8 a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int a5)
{
  unsigned int v1;
  unsigned long *v10;
  unsigned long v11;
  stat v12; // stack - 0xd8
  void *v13;
  int4 v2;
  char *v3;
  int8 v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  char v8; // al
  uint8 v9; // rax
  
  v9 = strlen(a0);
  for (v10 = dat_84320; v10; v10 = (unsigned long *)*v10) {
    v3 = (char *)v10[0x14];
    if (!strcmp(v3,a0)) {
      if (*(char *)&v10[10]) {
        v2 = *(int4 *)((int8)v10 + 0x54);
        if (fstatat(dat_81234,v3,&v12,v2)) // branch-flip
          sub_32d30((char *)v10[0x14]);
        else {
          v10[1] = v12._0_8_;
          v10[2] = v12._8_8_;
        }
      }
      *(unsigned int *)&v10[3] = a4;
      if (a1) goto label_17cc6;
      goto label_17e08;
    }
  }
  v10 = (unsigned long *)sub_4c5b0(0xa8);
  *v10 = dat_84320;
  v10[0x13] = v9;
  dat_84320 = v10;
  v11 = sub_4c7a0(a0);
  *(char *)&v10[0xb] = 0;
  v10[0x14] = v11;
  if (!a1) {
    *(unsigned int *)&v10[3] = a4;
label_17e08:
    *(unsigned int *)&v10[9] = a2;
    *(char *)&v10[10] = 1;
    *(unsigned int *)((int8)v10 + 0x4c) = a3;
    *(unsigned int *)((int8)v10 + 0x54) = a5;
    *(unsigned int *)((int8)v10 + 0x5c) = dat_84580;
    v10[0xc] = 0;
    v10[0xd] = 0;
    v10[0xe] = 0;
    v10[0xf] = 0;
    v10[0x10] = 0;
    v10[0x12] = 0;
    v10[0x11] = 0;
    v8 = sub_244e0(a0);
    goto label_17e71;
  }
  v10[1] = *(unsigned long *)(a1 + 0x58);
  v10[2] = *(unsigned long *)(a1 + 0x60);
  *(unsigned int *)&v10[3] = a4;
label_17cc6:
  v3 = *(char **)(a1 + 0x30);
  v11 = *(unsigned long *)(a1 + 0xe8);
  v5 = *(unsigned long *)(a1 + 0xf0);
  v6 = *(unsigned long *)(a1 + 0xf8);
  v7 = *(unsigned long *)(a1 + 0x100);
  *(unsigned int *)((int8)v10 + 0x1c) = *(unsigned int *)(a1 + 0x74);
  v1 = *(unsigned int *)(a1 + 0x78);
  *(unsigned int *)&v10[9] = a2;
  *(unsigned int *)&v10[4] = v1;
  *(char *)&v10[10] = 0;
  *(unsigned int *)((int8)v10 + 0x4c) = a3;
  *(unsigned int *)((int8)v10 + 0x54) = a5;
  *(unsigned int *)((int8)v10 + 0x5c) = dat_84580;
  v10[0xc] = 0;
  v10[5] = v11;
  v10[6] = v5;
  v10[7] = v6;
  v10[8] = v7;
  sub_23be0(&v10[0xc],v3);
  if (*(void **)(a1 + 0x38)) { // branch-flip
    v11 = sub_4c770(*(void **)(a1 + 0x38),*(int8 *)(a1 + 0x40) + 1);
    v13 = *(void **)(a1 + 0x48);
    v10[0xd] = v11;
    v10[0xe] = *(unsigned long *)(a1 + 0x40);
    if (v13) goto label_17d61;
label_17dee:
    v10[0xf] = 0;
    v11 = 0;
  }
  else {
    v13 = *(void **)(a1 + 0x48);
    v10[0xd] = 0;
    v10[0xe] = 0;
    if (!v13) goto label_17dee;
label_17d61:
    v4 = *(int8 *)(a1 + 0x50);
    v10[0xf] = sub_4c770(v13,v4 + 1);
    v11 = *(unsigned long *)(a1 + 0x50);
  }
  v10[0x10] = v11;
  sub_1c840(a1,&v10[0x12],&v10[0x11]);
  v8 = sub_244e0(a0);
label_17e71:
  if (!v8)
    return;
  sub_17b10(v10); // tail-call
  return;
}


// Function: sub_17f70 @ 0x17f70
int4 sub_17f70(char *a0,char *a1) // return-dupe
{
  int4 v1;
  char v2;
  int4 v3; // eax
  int4 *v4; // rax
  char *v5;
  char *v6;
  stat v7;
  uint4 v8; // r12d
  uint4 v9;
  
  v5 = a0;
  if (!*a0)
    return 0;
label_17fb0:
  do {
    v2 = v5[1];
    v6 = v5;
    if (!v2)
      return 0;
    while( true ) {
      v5 = &v6[1];
      if ((((a0 == v5) || (v2 != '/')) || (*v6 == '/')) || ((*v6 == '.' && (((v5 == &a0[1] || (v6[-1] == '/')) || ((v6[-1] == '.' && ((v5 == &a0[2] || (v6[-2] == '/')))))))))) goto label_17fb0;
      *v5 = '\0';
      v8 = ~dat_8432c & 0x1ff;
      v9 = v8;
      if (!dat_84330)
        v9 = v8 | 0xc0;
      if (!mkdirat(dat_81234,a0,v9)) break;
      v4 = __errno_location();
      v1 = *v4;
      if ((v1 != 0x11) && (v3 = fstatat(dat_81234,a0,&v7,0), v3)) {
        *v4 = v1;
        sub_32760(a0);
        *v5 = '/';
        return v3;
      }
      *v5 = '/';
      v2 = v6[2];
      v6 = v5;
      if (!v2)
        return 0;
    }
    sub_17c30(a0,0,~dat_84328 & v9,0x1ff,v8,0x100);
    sub_22a00(a0,(int4)v5 - (int4)a0,v8);
    *a1 = 1;
    *v5 = '/';
  } while( true );
}


// Function: sub_18140 @ 0x18140
int4 sub_18140(char *a0,int4 a1,uint4 a2,int4 a3,uint4 *a4,unsigned int *a5)
{
  int4 v1;
  int4 v2; // eax
  int4 *v3; // rax
  stat v4; // stack - 0xd8
  uint4 v5;
  
  v1 = dat_84b34;
  v5 = 0x809c1;
  if (dat_84b34 == 2)
    v5 = (-(uint4)(dat_84b69 == '\0') & 0x20000) + 0x80b41;
  if (a3)
    v5 &= 0xffffff7f;
  if (((a1 == 0x37) && (!dat_84310)) && (dat_84310 = 1, dat_83b80 & 8)) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"Extracting contiguous files as regular files",5));
  }
  v2 = openat(dat_81234,a0,v5,(uint8)a2);
  if (0 <= v2) {
    if (v1 != 2) { // branch-flip
      *a4 = ~dat_84328 & a2;
      *a5 = 0x1ff;
    }
    else {
      if (fstat(v2,&v4)) { // branch-flip
        v3 = __errno_location();
        v1 = *v3;
        close(v2);
        *v3 = v1;
      }
      else {
        if ((v4._24_4_ & 0xf000) == 0x8000) {
          *a4 = v4._24_4_;
          *a5 = 0xffffffff;
          return v2;
        }
        close(v2);
        *__errno_location() = 0x11;
      }
      v2 = -1;
    }
  }
  return v2;
}


// Function: sub_182e0 @ 0x182e0
unsigned long * sub_182e0(char *a0)
{
  unsigned long *v1;
  uint8 v2;
  char *v3;
  
  v1 = dat_84320;
  do {
    if (!v1)
      return NULL;
    if (!*(char *)&v1[0xb]) {
      v2 = v1[0x13];
      v3 = (char *)v1[0x14];
      if (((!strncmp(a0,v3,v2)) && (a0[v2] == '/')) && ((char *)sub_3e390(a0) == &a0[v2 + 1]))
        return v1;
    }
    v1 = (unsigned long *)*v1;
  } while( true );
}


// Function: sub_18350 @ 0x18350
void sub_18350(void)
{
  int8 *v1;
  int8 *v10;
  int8 *v11;
  char v12 [16];
  int8 v13; // stack - 0x1c8
  int8 v14; // stack - 0x1c0
  int8 v15; // stack - 0x1b8
  int8 v16; // stack - 0x1b0
  int8 v17; // stack - 0x1a8
  unsigned int v18; // stack - 0x188
  unsigned int v19; // stack - 0x184
  int8 v2;
  unsigned int v20; // stack - 0x180
  int8 v21; // stack - 0x110
  int8 v22; // stack - 0x108
  int8 v23; // stack - 0x100
  int8 v24; // stack - 0xf8
  int8 v25; // stack - 0xa0
  int8 v26; // stack - 0x98
  int8 v3;
  unsigned long *v4;
  int8 *v5;
  unsigned long *v6;
  int8 *v7;
  stat v8; // stack - 0x288
  char v9 [48];
  
  v10 = dat_84318;
  if (!dat_84318) {
    dat_84318 = NULL;
    return;
  }
  do {
    v11 = NULL;
    v5 = &v10[0x14];
    sub_24c80(*(unsigned int *)&v10[0xb]);
    v7 = (int8 *)v10[0xc];
    if (v7) {
      do {
        while( true ) {
          v1 = &v7[1];
          if (((!fstatat(dat_81234,(char *)v1,&v8,0x100)) && (v8._0_8_ == v10[1])) && (v8._8_8_ == v10[2])) break;
label_183b0:
          v7 = (int8 *)*v7;
          if (!v7) goto label_18490;
        }
        v12 = sub_4bd30(&v8);
        v2 = v10[4];
        v3 = v10[3];
        if (sub_4be80(SUB168(v12,0),SUB168(v12,8),v3,v2)) goto label_183b0;
        if (unlinkat(dat_81234,(char *)v1,0)) {
          sub_32db0(v1);
          goto label_183b0;
        }
        if ((v11) && (!linkat(dat_81234,(char *)v11,dat_81234,(char *)v1,0))) goto label_183b0;
        if ((char)v10[5]) {
          if (symlinkat((char *)v5,dat_81234,(char *)v1)) // branch-flip
            sub_32c90(v5,v1);
          else {
            v18 = *(unsigned int *)((int8)v10 + 0x2c);
            v21 = v10[7];
            v22 = v10[8];
            v23 = v10[9];
            v24 = v10[10];
            v19 = *(unsigned int *)&v10[6];
            v20 = *(unsigned int *)((int8)v10 + 0x34);
            v13 = v10[0xd];
            v14 = v10[0xe];
            v15 = v10[0xf];
            v16 = v10[0x10];
            v17 = v10[0x11];
            v26 = v10[0x13];
            v25 = v10[0x12];
            sub_17530(v1,v9,0xffffffff,0,0,0x32,0,0x100);
            v11 = v1;
          }
          goto label_183b0;
        }
        if (!linkat(dat_81234,(char *)v5,dat_81234,(char *)v1,0)) goto label_183b0;
        sub_326e0(v5,v1);
        v7 = (int8 *)*v7;
      } while (v7);
label_18490:
      v6 = (unsigned long *)v10[0xc];
      while (v6) {
        v4 = (unsigned long *)*v6;
        free(v6);
        v6 = v4;
      }
    }
    sub_1c600((unsigned long *)v10[0x13],v10[0x12]);
    free((void *)v10[0xd]);
    v5 = (int8 *)*v10;
    free(v10);
    if (!v5) {
      dat_84318 = NULL;
      return;
    }
    v10 = v5;
  } while( true );
}


// Function: sub_18640 @ 0x18640
void sub_18640(void)
{
  sub_17870("",0);
  sub_18350();
  sub_17870("",1); // tail-call
  return;
}


// Function: sub_18750 @ 0x18750
unsigned long sub_18750(char *a0,bool a1,char *a2) // return-dupe
{
  int4 v1;
  uint4 *v2; // rax
  char v3 [152];
  
  if (*a2)
    return 0;
  v2 = (uint4 *)__errno_location();
  v1 = *v2;
  if (v1 != 0x11) { // branch-flip
    if (0x12 <= v1) {
      if ((((v1 != 0x28) && ((v1 & 0xffffffbf) != 0x1f)) || (!a1)) || ((dat_84b34 != 2 || (dat_84b69)))) goto label_187c0;
      if (strchr(a0,0x2f)) {
        if (sub_246f0(a0,v3)) goto label_187c0;
        goto label_18812;
      }
      goto label_18850;
    }
    if (v1 != 2) goto label_187c0;
  }
  else {
label_18812:
    if (dat_84b34 == 5) {
      if (dat_83b82 & 0x10) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"%s: skipping existing file",5),a0);
        return 2;
      }
      return 2;
    }
    if (6 <= dat_84b34) { // branch-flip
      if ((dat_84b34 == 6) && (!sub_18670(a0))) goto label_18850;
    }
    else if (3 <= dat_84b34) { // branch-flip
      if (dat_84b34 == 4)
        return 0;
    }
    else {
label_18850:
      if (1 <= (int4)sub_25490(a0,0))
        return 1;
    }
  }
  if ((!sub_17f70(a0,a2)) && (*a2))
    return 1;
label_187c0:
  *v2 = v1;
  return 0;
}


// Function: sub_18900 @ 0x18900
unsigned int sub_18900(char *a0,char a1)
{
  int4 v1; // eax
  uint4 v2;
  char v3; // stack - 0x31
  
  v3 = 0;
  v2 = dat_848b0;
  if (1 <= dat_84aa8)
    v2 = dat_848b0 & 0xffffffc0;
  while( true ) {
    if (!mkfifoat(dat_81234,a0,v2 & 0x1ff)) {
      sub_17530(a0,0x84840,0xffffffff,~dat_84328 & v2 & 0x1ff,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v1 = sub_18750(a0,0,&v3);
    if (!v1) break;
    if (v1 == 2)
      return 0;
  }
  sub_32780(a0);
  return 0xffffffff;
}


// Function: sub_189e0 @ 0x189e0
unsigned int sub_189e0(char *a0,char a1)
{
  int4 v1; // eax
  uint4 v2;
  char v3; // stack - 0x31
  
  v3 = 0;
  v2 = dat_848b0;
  if (1 <= dat_84aa8)
    v2 = dat_848b0 & 0xffffffc0;
  while( true ) {
    v1 = mknodat(dat_81234,a0,v2 & 0x61ff,dat_848c0);
    if (!v1) {
      sub_17530(a0,0x84840,0xffffffff,~dat_84328 & v2 & 0x61ff,0x1ff,(int4)a1,0,0x100);
      return 0;
    }
    v1 = sub_18750(a0,0,&v3);
    if (!v1) break;
    if (v1 == 2)
      return 0;
  }
  sub_327a0(a0);
  return 0xffffffff;
}


// Function: sub_18ad0 @ 0x18ad0
unsigned int sub_18ad0(char *a0,char a1,char *a2,unsigned long *a3)
{
  unsigned long v1;
  int4 v2; // eax
  unsigned long *v3; // rax
  unsigned long *v4; // rax
  stat v5; // stack - 0xd8
  char v6 [16];
  
  do {
    v2 = openat(dat_81234,a0,0xc1,0);
    if (0 <= v2) {
      if (fstat(v2,&v5)) {
        sub_32d30(a0);
        close(v2);
        return 0xffffffff;
      }
      if (!close(v2)) {
        v3 = (unsigned long *)sub_4c5b0(strlen(dat_84858) + 0xa1);
        if (a3) { // branch-flip
          v1 = *a3;
          *a3 = v3;
          *v3 = v1;
        }
        else {
          *v3 = dat_84318;
          dat_84318 = v3;
        }
        v3[1] = v5._0_8_;
        v3[2] = v5._8_8_;
        v6 = sub_4bd30(&v5);
        *(char *)&v3[5] = a1;
        v3[3] = SUB168(v6,0);
        v3[4] = SUB168(v6,8);
        if (a1) {
          *(unsigned int *)((int8)v3 + 0x2c) = dat_848b0;
          v3[7] = dat_84928;
          v3[8] = dat_84930;
          v3[6] = dat_848b4;
          v3[9] = dat_84938;
          v3[10] = dat_84940;
        }
        *(unsigned int *)&v3[0xb] = dat_84580;
        v4 = (unsigned long *)sub_4c5b0(strlen(a0) + 9);
        v3[0xc] = v4;
        *v4 = 0;
        strcpy((char *)&v4[1],a0);
        v3[0xd] = 0;
        sub_23be0(&v3[0xd],dat_84870);
        v3[0xe] = 0;
        v3[0xf] = 0;
        v3[0x10] = 0;
        v3[0x11] = 0;
        sub_1c840(0x84840,&v3[0x13],&v3[0x12]);
        strcpy((char *)&v3[0x14],dat_84858);
        v3 = (unsigned long *)sub_182e0(a0);
        if (!v3)
          return 0;
        sub_17b10(v3);
        return 0;
      }
      sub_32680(a0);
      return 0xffffffff;
    }
    if ((*__errno_location() == 0x11) && (sub_16da0(a0)))
      return 0;
    v2 = sub_18750(a0,0,a2);
    if (!v2) {
      sub_327c0(a0);
      return 0xffffffff;
    }
  } while (v2 != 2);
  return 0;
}


// Function: sub_18d40 @ 0x18d40
unsigned int sub_18d40(char *a0) // return-dupe x2
{
  int8 v1;
  int8 *v10;
  stat v11; // stack - 0x158
  char v12; // stack - 0x159
  stat v13; // stack - 0xc8
  char v14 [16];
  int8 v2;
  char *v3;
  char v4; // al
  int4 v5;
  int4 v6; // eax
  unsigned long *v7; // rax
  int4 *v8; // rax
  int8 *v9; // rax
  
  v3 = dat_84858;
  v12 = 0;
  if ((!dat_84b92) && (v4 = sub_27cc0(dat_84858), v4))
    return sub_18ad0(a0,0,&v12,NULL);
  v7 = (unsigned long *)sub_16da0(v3);
  if (v7)
    return sub_18ad0(a0,0,&v12,v7);
  v8 = __errno_location();
  while( true ) {
    v6 = linkat(dat_81234,v3,dat_81234,a0,0);
    v10 = dat_84318;
    v5 = *v8;
    if (!v6) {
      if (!dat_84318)
        return 0;
      v6 = fstatat(dat_81234,v3,&v11,0x100);
      v5 = dat_84580;
      if (v6)
        return 0;
      do {
        if ((((int4)v10[0xb] == v5) && (v10[1] == v11._0_8_)) && (v10[2] == v11._8_8_)) {
          v14 = sub_4bd30(&v11);
          v1 = v10[3];
          v2 = v10[4];
          if (!sub_4be80(v1,v2,SUB168(v14,0),SUB168(v14,8))) {
            v9 = (int8 *)sub_4c5b0(strlen(a0) + 9);
            strcpy((char *)&v9[1],a0);
            *v9 = v10[0xc];
            v10[0xc] = (int8)v9;
            return 0;
          }
        }
        v10 = (int8 *)*v10;
      } while (v10);
      return 0;
    }
    if ((v5 == 0x11) && (!strcmp(v3,a0)))
      return 0;
    if (((!fstatat(dat_81234,v3,&v11,0x100)) && (!fstatat(dat_81234,a0,&v13,0x100))) && ((v11._0_8_ == v13._0_8_ && (v11._8_8_ == v13._8_8_)))) break;
    *v8 = v5;
    v5 = sub_18750(a0,0,&v12);
    if (v5 != 1) {
      if (v5 == 2)
        return 0;
      if ((dat_84b50) && (*v8 == 0x11))
        return 0;
      sub_326e0(v3,a0);
      return 1;
    }
  }
  return 0;
}


// Function: sub_18fc0 @ 0x18fc0
unsigned long sub_18fc0(char *a0) // early-return
{
  char v1; // al
  int4 v2; // eax
  unsigned long v3; // rax
  char v4; // stack - 0x21
  char *v5;
  
  v4 = 0;
  if ((!dat_84b92) && ((*dat_84858 == '/' || (v1 = sub_27cc0(dat_84858), v1)))) {
    v3 = sub_18ad0(a0,1,&v4,NULL);
    return v3;
  }
  v5 = dat_84858;
  while( true ) {
    if (!symlinkat(v5,dat_81234,a0)) {
      sub_17530(a0,0x84840,0xffffffff,0,0,0x32,0,0x100);
      return 0;
    }
    v2 = sub_18750(a0,0,&v4);
    if (!v2) break;
    if (v2 == 2)
      return 0;
    v5 = dat_84858;
  }
  sub_32c90(dat_84858,a0);
  return 0xffffffff;
}


// Function: sub_191d0 @ 0x191d0
int4 sub_191d0(char *a0,unsigned int a1)
{
  int4 v1; // eax
  uint4 v10; // eax
  char v11; // stack - 0x6c
  int4 v2;
  int8 v3; // rax
  uint8 v4;
  uint8 v5; // rax
  uint8 v6; // stack - 0x48
  unsigned int v7; // stack - 0x4c
  unsigned int v8; // stack - 0x50
  char v9; // stack - 0x51
  
  v9 = 0;
  v10 = dat_848b0 & 0x1c0;
  if (dat_84aa8 <= 0)
    v10 = dat_848b0 & 0x1ff;
  v8 = 0;
  v7 = 0;
  v11 = (char)a1;
  if (dat_84a6a) // branch-flip
    v1 = 1;
  else if (dat_84a60) { // branch-flip
    v1 = sub_2b640(a0,0x66,0x84840);
    if (v1 < 0) {
      sub_22bd0();
      return 0;
    }
  }
  else {
    v6 &= 0xffffffff00000000;
    v1 = sub_190d0(a0,0,(int4)v11,&v6);
    v4 = v6;
    if (v1)
      return 1;
    while (v1 = sub_18140(a0,a1,v10,v4 & 0xffffffff,&v8,&v7), v1 <= -1) {
      v1 = sub_18750(a0,1,&v9);
      if (v1 != 1) {
        sub_22bd0();
        if (v1 == 2)
          return 0;
        sub_327c0(a0);
        return 1;
      }
    }
  }
  sub_ccb0((struct_18 *)0x84840);
  if (dat_84960) { // branch-flip
    sub_29ed0(v1,0x84840,&v6);
    v4 = v6;
  }
  else {
    v6 = dat_848c8;
    v4 = dat_848c8;
    while (0 < (int8)v4) {
      sub_ccf0(v4);
      v3 = sub_e130();
      if (!v3) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
        dat_84d58 = 2;
        v4 = v6;
        break;
      }
      v4 = sub_d2b0(v3);
      if (v6 < v4)
        v4 = v6;
      *__errno_location() = 0;
      v5 = sub_249f0(v1,v3,v4);
      v6 -= v4;
      sub_d1f0(v3 + -1 + v4);
      if (v4 != v5) {
        if (!dat_84a60)
          sub_32e30(a0,v5,v4);
        v4 = v6;
        break;
      }
      v4 = v6;
    }
  }
  sub_22b10(v4);
  v2 = 0;
  sub_ccd0();
  if (!dat_84a6a) {
    if (dat_84a60) // branch-flip
      v2 = close(v1);
    else {
      sub_17530(a0,0x84840,v1,v8,v7,(int4)v11,0,(uint8)(dat_84b34 != 2) << 8);
      v2 = close(v1);
    }
    if (v2 <= -1)
      sub_32680(a0);
    if (dat_84a60)
      sub_2b6f0();
  }
  return v2;
}


// Function: sub_19540 @ 0x19540
unsigned long sub_19540(char *a0,unsigned int a1,unsigned long *a2) // return-dupe x3, return-dupe
{
  int4 v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  code *v4; // rbx
  
  switch(a1) {
    case 0:
    case 0x30:
    case 0x37:
      if (!dat_84850) goto label_195f9;
      v4 = sub_19850;
      break;
    default:
      if (dat_83b82 & 1) {
        if (dat_84d50)
          (*dat_84d50)();
        v2 = (char *)sub_4aa70(a0);
        error(0,0,dcgettext(NULL,"%s: Unknown file type \'%c\', extracted as normal file",5),v2,(char)a1);
      }
      goto label_195f9;
    case 0x31:
      v4 = sub_18d40;
      break;
    case 0x32:
      v4 = sub_18fc0;
      break;
    case 0x33:
      dat_848b0 |= 0x2000;
      v4 = sub_189e0;
      break;
    case 0x34:
      dat_848b0 |= 0x6000;
      v4 = sub_189e0;
      break;
    case 0x35:
    case 0x44:
      v4 = sub_19850;
      if (dat_849c8)
        dat_847e0 = 1;
      break;
    case 0x36:
      v4 = sub_18900;
      break;
    case 0x4b:
    case 0x4c:
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected long name header",5));
      dat_84d58 = 2;
      return 0;
    case 0x4d:
      if (dat_84d50)
        (*dat_84d50)();
      v3 = sub_4aa70(dat_84848);
      error(0,0,dcgettext(NULL,"%s: Cannot extract -- file is continued from another volume",5),v3);
      dat_84d58 = 2;
      return 0;
    case 0x53:
label_195f9:
      if ((!dat_84a6a) && (v4 = sub_191d0, !dat_84a60)) goto label_19631;
      goto label_19602;
    case 0x56:
      return 0;
    
  }
  if (dat_84a6a)
    return 0;
  if (dat_84a60) { // branch-flip
    if (v4 != sub_191d0)
      return 0;
label_19602:
    v4 = sub_191d0;
  }
  else {
label_19631:
    if (dat_84b34 != 3) { // branch-flip
      if ((dat_84b34 == 6) && (sub_18670(a0,NULL))) {
        if (!(dat_83b81 & 8))
          return 0;
        if (dat_84d50)
          (*dat_84d50)();
        v2 = (char *)sub_4ac50(a0);
        error(0,0,dcgettext(NULL,"Current %s is newer or same age",5),v2);
        return 0;
      }
    }
    else {
      v1 = sub_25490(a0,(uint1)dat_84aba);
      if ((!v1) && (*__errno_location() & 0xfffffffdU)) {
        sub_32db0(a0);
        return 0;
      }
    }
  }
  *a2 = v4;
  return 1;
}


// Function: sub_19850 @ 0x19850
int4 sub_19850(char *a0,int4 a1)
{
  char *v1;
  char v10; // stack - 0xd9
  stat *v11;
  unsigned int *v12;
  unsigned long v13; // r9
  uint4 v14;
  unsigned long v15; // r15
  unsigned int *v2;
  char v3;
  uint4 v4; // eax
  int4 v5; // eax
  int4 v6; // eax
  int4 *v7; // rax
  int8 v8; // rcx
  stat v9; // stack - 0xd8
  
  v10 = '\0';
  if ((dat_84ad1) && (!dat_847f0)) {
    if (fstatat(dat_81234,".",&v9,0)) // branch-flip
      sub_24ec0(".");
    else {
      dat_847f0 = v9._0_8_;
    }
  }
  if (dat_84b50) // branch-flip
    sub_206e0(a0);
  else if (a1 == 0x44)
    sub_22bd0();
  v15 = 0;
  v14 = 0;
  v4 = sub_19820(dat_848b0);
label_198cc:
  v5 = mkdirat(dat_81234,a0,v4);
  if (!v5) goto label_19ac0;
  v7 = __errno_location();
  v3 = v10;
  if (*v7 == 0x11) {
    if (!v10) {
      if (dat_84b30) goto label_199c4;
      if (dat_84b34 <= 2) goto label_19915;
      goto label_19949;
    }
    if (dat_84b30) {
label_199c4:
      if (sub_16ce0(a0))
        return 0;
    }
label_19915:
    if (sub_246f0(a0,&v9)) {
label_19943:
      *v7 = 0x11;
      goto label_19949;
    }
    if ((v9._24_4_ & 0xf000) != 0x4000) {
      v15 = 0xffffffff;
      v14 = v9._24_4_;
      goto label_19943;
    }
    if (v3) {
      sub_16e60(a0,&v9);
      return 0;
    }
    if (dat_84b34 != 1) {
label_19af0:
      v15 = 0xffffffff;
      v14 = dat_84b34;
      goto label_1996f;
    }
    v4 = sub_19820(v9._24_4_,a0);
    v5 = sub_17280(0xffffffff);
    if (v5) {
      sub_32550(a0,v4);
      goto label_19af0;
    }
    v8 = 0x24;
    v11 = &v9;
    v12 = (unsigned int *)0x84898;
    while (v8) {
      v2 = &v12[1];
      v1 = &v11->field_0x4;
      *v12 = *(unsigned int *)v11;
      v8 -= 1;
      v11 = (stat *)v1;
      v12 = v2;
    }
label_19ac0:
    v13 = 0x100;
    v15 = 0x1ff;
    v9._24_4_ = ~dat_84328 & v4;
    sub_17c30(a0,0x84840,v9._24_4_,v15,dat_848b0,v13); // return-dupe
    return v5;
  }
label_19949:
  v6 = sub_18750(a0,0,&v10);
  if (v6) goto label_19960;
  if (*v7 != 0x11) {
    sub_32760(a0);
    return 1;
  }
  goto label_19969;
label_19960:
  if (v6 != 1) {
label_19969:
    v9._24_4_ = v14;
    v14 = dat_84b34;
label_1996f:
    if (!(v14 & 0xfffffffd)) {
      v13 = 0;
      sub_17c30(a0,0x84840,v9._24_4_,v15,dat_848b0,v13);
    }
    return v5;
  }
  goto label_198cc;
}


// Function: sub_19b30 @ 0x19b30
void sub_19b30(void)
{
  uint4 v1; // eax
  
  dat_84330 = geteuid() == 0;
  dat_84aa4 += (uint4)(uint1)dat_84330;
  dat_84aa8 += (uint4)(uint1)dat_84330;
  v1 = umask(0);
  dat_8432c = v1;
  if (1 <= dat_84aa4) {
    dat_84328 = 0;
    return;
  }
  umask(v1);
  dat_84328 = v1;
  return;
}


// Function: sub_19b90 @ 0x19b90
void sub_19b90(char *a0)
{
  int8 *v1;
  char *v2;
  int4 v3;
  int8 *v4; // rbp
  int8 *v5;
  
  v3 = dat_84580;
  if (!dat_84320)
    return;
  v1 = dat_84320;
  v5 = NULL;
  while ((v4 = v1, v1 = (int8 *)*v4, *(int4 *)((int8)v4 + 0x5c) != v3 || (v2 = (char *)v4[0x14], strcmp(v2,a0)))) {
    if (!v1)
      return;
    v5 = v4;
  }
  sub_16c90(v4);
  if (v5) {
    *v5 = (int8)v1;
    return;
  }
  dat_84320 = v1;
  return;
}


// Function: sub_19c10 @ 0x19c10
void sub_19c10(void) // return-dupe
{
  unsigned int v1;
  char v2;
  int4 v3;
  char *v4; // rax
  code *v5; // stack - 0x28
  
  dat_842f0 = sub_18640;
  sub_d1f0(dat_84408);
  if ((dat_84b92) || (v2 = sub_27cc0(dat_84840), !v2)) {
    if ((*dat_84848) && ((!dat_84b40 || (sub_2ca20("extract"))))) {
      if (dat_84a54) { // branch-flip
        sub_22990((struct_9 *)0x84840,dat_84408,-1);
        v1 = dat_84580;
        v2 = dat_847e0;
      }
      else {
        v1 = dat_84580;
        v2 = dat_847e0;
      }
      dat_84580 = v1;
      if (!v2) {
        sub_17870(dat_84848,0);
        sub_24c80(v1);
      }
      if ((!dat_84b84) || (v2 = sub_24710(dat_84848,0), v2)) {
        v3 = 0x53;
        if (!sub_29e10(0x84840))
          v3 = (int4)*(char *)(dat_84408 + 0x9c);
        v2 = sub_19540(dat_84848,v3,&v5);
        if (v2) { // branch-flip
          v3 = (*v5)(dat_84848,v3);
          if (!v3)
            return;
        }
        else {
          sub_22bd0();
        }
        if (!dat_84b84)
          return;
        sub_245d0(); // tail-call
        return;
      }
      v3 = *__errno_location();
      if (dat_84d50)
        (*dat_84d50)();
      v4 = (char *)sub_4aa70(dat_84848);
      error(0,v3,dcgettext(NULL,"%s: Was unable to backup this file",5),v4);
      dat_84d58 = 2;
    }
  }
  else {
    if (dat_84d50)
      (*dat_84d50)();
    v4 = (char *)sub_4aa70(dat_84840);
    error(0,0,dcgettext(NULL,"%s: Member name contains \'..\'",5),v4);
    dat_84d58 = 2;
  }
  sub_22bd0(); // tail-call
  return;
}


// Function: sub_19ec0 @ 0x19ec0
unsigned long sub_19ec0(char *a0,char *a1) // return-dupe
{
  int4 *v1; // rax
  char *v2; // rax
  char *v3; // rax
  int8 v4;
  int4 v5;
  
  if (!renameat(dat_81234,a0,dat_81234,a1)) {
    sub_17bb0(a0,a1);
    return 1;
  }
  v1 = __errno_location();
  v5 = *v1;
  if ((v5 == 2) && (!sub_17f70(a1,&v4))) {
    if (!renameat(dat_81234,a0,dat_81234,a1))
      return 1;
    v5 = *v1;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v2 = (char *)sub_4ac40(1,a1);
  v3 = (char *)sub_4ac40(0,a0);
  error(0,v5,dcgettext(NULL,"Cannot rename %s to %s",5),v3,v2);
  dat_84d58 = 2;
  return 0;
}


// Function: sub_19ff0 @ 0x19ff0
void sub_19ff0(char *a0)
{
  char v1;
  char *v2;
  char *v3; // rdx
  
  v2 = a0;
label_1a00f:
  do {
    while( true ) {
      v3 = &v2[1];
      v1 = *a0;
      if (v1 == '%') break;
      *v2 = v1;
      if (!v1)
        return;
label_1a007:
      a0 = &a0[1];
      v2 = v3;
    }
    if (a0[1] != '3') {
      if ((a0[1] != '2') || (a0[2] != '5')) {
label_1a028:
        *v2 = '%';
        goto label_1a007;
      }
      *v2 = '%';
      a0 = &a0[3];
      v2 = v3;
      goto label_1a00f;
    }
    if (a0[2] != 'D') goto label_1a028;
    *v2 = '=';
    a0 = &a0[3];
    v2 = v3;
  } while( true );
}


// Function: sub_1a060 @ 0x1a060
void sub_1a060(void)
{
  return;
}


// Function: sub_1a070 @ 0x1a070
void sub_1a070(void)
{
  return;
}


// Function: sub_1a080 @ 0x1a080
unsigned long sub_1a080(char *a0)
{
  int8 v1; // rbx
  char *v2;
  
  v2 = "atime";
  v1 = 0x7f200;
  do {
    if ((!*(char *)(v1 + 0x1c)) && (*(uint1 *)(v1 + 0x18) & 1)) {
      if (!fnmatch(a0,v2,0))
        return 1;
    }
    v2 = *(char **)(v1 + 0x20);
    v1 += 0x20;
  } while (v2);
  return 0;
}


// Function: sub_1a0f0 @ 0x1a0f0
void sub_1a0f0(char *a0)
{
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Keyword %s is unknown or not yet implemented",5),a0);
  sub_2c970(2); // no-return
}


// Function: sub_1a140 @ 0x1a140
unsigned long sub_1a140(char *a0)
{
  int8 v1; // rbx
  char *v2;
  
  v2 = "atime";
  v1 = 0x7f200;
  do {
    if ((!*(char *)(v1 + 0x1c)) && (*(uint1 *)(v1 + 0x18) & 1)) {
      if (!strcmp(v2,a0))
        return 1;
    }
    v2 = *(char **)(v1 + 0x20);
    v1 += 0x20;
  } while (v2);
  return 0;
}


// Function: sub_1a1a0 @ 0x1a1a0
unsigned long sub_1a1a0(char *a0) // return-dupe
{
  char *v1;
  int8 *v2;
  
  v2 = dat_84380;
  if (!dat_84380)
    return 0;
  do {
    v1 = (char *)v2[1];
    if (!strcmp(v1,a0))
      return 1;
    v2 = (int8 *)*v2;
  } while (v2);
  return 0;
}


// Function: sub_1a200 @ 0x1a200
void sub_1a200(unsigned long *a0,unsigned long *a1,char *a2) // return-dupe
{
  char *v1; // stack - 0x38
  char v2 [16];
  
  v2 = sub_242f0(a2,&v1,0);
  if ((sub_23ba0(SUB168(v2,0),SUB168(v2,8))) && (!*v1)) {
    *a1 = SUB168(v2,0);
    sub_23be0(a0,a2);
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Time stamp is out of allowed range",5));
  dat_84d58 = 2;
  return;
}


// Function: sub_1a2c0 @ 0x1a2c0
void sub_1a2c0(unsigned long *a0,char *a1,char *a2)
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long v3;
  
  v2 = (unsigned long *)sub_4c5b0(0x18);
  v2[1] = sub_4c7a0(a1);
  v3 = 0;
  if (a2)
    v3 = sub_4c7a0(a2);
  v1 = *a0;
  v2[2] = v3;
  *a0 = v2;
  *v2 = v1;
  return;
}


// Function: sub_1a320 @ 0x1a320
void sub_1a320(uint1 *a0,uint1 *a1)
{
  uint1 v1;
  uint1 v2;
  int8 *v3; // rax
  char *v4; // rax
  uint1 *v5;
  unsigned long *v6; // rsi
  unsigned long *v7; // rdi
  
  if (a1 != a0) { // branch-flip
    v1 = a1[-1];
    v5 = a1;
    if (v1 == 0x3a)
      v5 = &a1[-1];
    if (a0 < v5) {
      v3 = __ctype_b_loc();
      do {
        if (!(*(uint1 *)(*v3 + 1 + (uint8)*v5 * 2) & 0x20)) break;
        v5 = &v5[-1];
      } while (a0 != v5);
    }
    *v5 = 0;
    v2 = a1[1];
    a1 = &a1[1];
    if (v2) {
      v3 = __ctype_b_loc();
      do {
        if (!(*(uint1 *)(*v3 + 1 + (uint8)v2 * 2) & 0x20)) break;
        v2 = a1[1];
        a1 = &a1[1];
      } while (v2);
    }
    if (strcmp((char *)a0,"delete")) {
      v6 = (unsigned long *)0x84370;
      if (strcmp((char *)a0,"exthdr.name")) {
        if (strcmp((char *)a0,"globexthdr.name")) {
          v6 = (unsigned long *)0x84360;
          v7 = (unsigned long *)0x84368;
          if (strcmp((char *)a0,"exthdr.mtime")) {
            if (strcmp((char *)a0,"globexthdr.mtime")) {
              if (sub_1a140(a0)) {
                if (dat_84d50)
                  (*dat_84d50)();
                v4 = dcgettext(NULL,"Keyword %s cannot be overridden",5);
                error(0,0,v4,a0); // return-dupe
                sub_2c970(2); // no-return
              }
              v6 = (unsigned long *)0x84388;
              v5 = a1;
              if (v1 == 0x3a)
                v6 = (unsigned long *)0x84380;
              sub_1a2c0(v6,a0,v5); // return-dupe, tail-call
              return;
            }
            v6 = (unsigned long *)0x84348;
            v7 = (unsigned long *)0x84350;
          }
          sub_1a200(v7,v6,a1); // tail-call
          return;
        }
        v6 = (unsigned long *)0x84358;
      }
      sub_23be0(v6,a1); // tail-call
      return;
    }
    if (!sub_1a080(a1)) {
      v5 = NULL;
      v6 = (unsigned long *)0x84390;
      a0 = a1;
      sub_1a2c0(v6,a0,v5);
      return;
    }
    if (dat_84d50)
      (*dat_84d50)();
    a0 = (uint1 *)sub_4ac50(a1);
    v4 = dcgettext(NULL,"Pattern %s cannot be used",5);
  }
  else {
    if (dat_84d50)
      (*dat_84d50)();
    a0 = (uint1 *)sub_4ac50(a0);
    v4 = dcgettext(NULL,"Malformed pax option: %s",5);
  }
  error(0,0,v4,a0);
  sub_2c970(2);
}


// Function: sub_1a560 @ 0x1a560
void sub_1a560(int8 a0,unsigned long a1,void *a2,uint8 a3)
{
  void *v1; // rax
  
  v1 = (void *)sub_4c5b0(a3);
  *(void **)(a0 + 400) = v1;
  memcpy(v1,a2,a3); // tail-call
  return;
}


// Function: sub_1a5a0 @ 0x1a5a0
void sub_1a5a0(int8 *a0)
{
  obstack *v1; // rax
  
  if (*a0)
    return;
  v1 = (obstack *)sub_4c5b0(0x58);
  *a0 = (int8)v1;
  _obstack_begin(v1,0,0,sub_4c5b0,dat_80fb8); // tail-call
  return;
}


// Function: sub_1a5e0 @ 0x1a5e0
void sub_1a5e0(unsigned long *a0,char *a1)
{
  if ((void *)*a0) {
    free((void *)*a0);
    *a0 = 0;
  }
  if (sub_31340(0,a1,a0))
    return;
  sub_23be0(a0,a1); // tail-call
  return;
}


// Function: sub_1a640 @ 0x1a640
void sub_1a640(int8 a0,unsigned long a1,char *a2)
{
  sub_1a5e0((unsigned long *)(a0 + 0x30),a2); // tail-call
  return;
}


// Function: sub_1a6b0 @ 0x1a6b0
char * sub_1a6b0(char *a0)
{
  char *v1;
  char *v2; // rax
  uint8 v3;
  uint8 v4; // rbx
  char v5;
  char *v6;
  
  if (dat_84340) { // branch-flip
    *dat_84340 = 0;
    v6 = dat_84340;
  }
  else {
    dat_84338 = 0x100;
    v2 = (char *)sub_4c5b0(0x100);
    v6 = v2;
    dat_84340 = v2;
  }
  v5 = *a0;
  v4 = 0;
  if (!v5) {
    v2 = v6;
    *v6 = 0;
    return v2;
  }
  do {
    v3 = v4 + 2;
    if (dat_84338 <= v3) { // branch-flip
      v2 = (char *)sub_4c710(v6,(uint8 *)0x84338);
      v1 = &v2[v4];
      v6 = v2;
      dat_84340 = v2;
      if (v5 == '%') goto label_1a73f;
label_1a6fa:
      v2 = v1;
      if (v5 != '=') { // branch-flip
        *v2 = v5;
        v3 = v4;
      }
      else {
        builtin_strncpy(v2,"%3D",4);
      }
    }
    else {
      v1 = &v6[v4];
      if (v5 != '%') goto label_1a6fa;
label_1a73f:
      v2 = v1;
      builtin_strncpy(v2,"%25",4);
    }
    v5 = a0[1];
    a0 = &a0[1];
    v4 = v3 + 1;
    if (!v5) {
      v2 = v6;
      v6[v4] = 0;
      return v2;
    }
  } while( true );
}


// Function: sub_1a7c0 @ 0x1a7c0
void sub_1a7c0(struct_5 *a0,void *a1,uint8 a2)
{
  obstack *v1;
  void *v2;
  
  v1 = a0->field_0x0;
  v2 = *(void **)&v1->field_0x18;
  if ((uint8)(*(int8 *)&v1->field_0x20 - (int8)v2) < a2) {
    _obstack_newchunk(v1,a2);
    v2 = *(void **)&v1->field_0x18;
  }
  memcpy(v2,a1,a2);
  *(uint8 *)&v1->field_0x18 = *(int8 *)&v1->field_0x18 + a2;
  a0->field_0x8 = a0->field_0x8 + a2;
  return;
}


// Function: sub_1a830 @ 0x1a830
void sub_1a830(struct_5 *a0,char a1)
{
  obstack *v1;
  char *v2;
  
  v1 = a0->field_0x0;
  v2 = *(char **)&v1->field_0x18;
  if (*(char **)&v1->field_0x20 == v2) {
    _obstack_newchunk(v1,1);
    v2 = *(char **)&v1->field_0x18;
  }
  *(char **)&v1->field_0x18 = &v2[1];
  *v2 = a1;
  a0->field_0x8 = a0->field_0x8 + 1;
  return;
}


// Function: sub_1a880 @ 0x1a880
void sub_1a880(struct_5 *a0,char *a1,void *a2,uint8 a3)
{
  char *v1; // rax
  uint8 v2; // rax
  int8 v3; // rax
  char v4 [20];
  char v5 [4];
  char *v6;
  char *v7; // r15
  bool v8; // zf
  
  v1 = (char *)sub_1a6b0(a1);
  v2 = strlen(v1);
  v6 = NULL;
  do {
    v3 = sub_43090(&v6[v2 + 3 + a3],v4);
    v7 = &v5[-v3];
    v8 = v6 != v7;
    v6 = v7;
  } while (v8);
  sub_1a7c0(a0,v3,v7);
  sub_1a830(a0,0x20);
  sub_1a7c0(a0,v1,v2);
  sub_1a830(a0,0x3d);
  sub_1a7c0(a0,a2,a3);
  sub_1a830(a0,10);
  return;
}


// Function: sub_1a980 @ 0x1a980
void sub_1a980(struct_5 *a0,char *a1,char *a2)
{
  sub_1a880(a0,a1,a2,strlen(a2)); // tail-call
  return;
}


// Function: sub_1a9b0 @ 0x1a9b0
void sub_1a9b0(char *a0,char *a1,struct_5 *a2)
{
  void *v1;
  void *v2; // stack - 0x28
  
  v1 = v2;
  if (!sub_31340(1,a0,&v2)) {
    v1 = (void *)sub_4c7a0(a0);
    v2 = v1;
  }
  sub_1a980(a2,a1,v1);
  free(v2);
  return;
}


// Function: sub_1aa40 @ 0x1aa40
void sub_1aa40(int8 a0,char *a1,struct_5 *a2)
{
  sub_1a9b0(*(char **)(a0 + 0x30),a1,a2); // tail-call
  return;
}


// Function: sub_1ab10 @ 0x1ab10
void sub_1ab10(struct_5 *a0,uint8 a1)
{
  obstack *v1;
  uint8 v2;
  
  v1 = a0->field_0x0;
  v2 = *(int8 *)&v1->field_0x18;
  if ((uint8)(*(int8 *)&v1->field_0x20 - v2) < a1) {
    _obstack_newchunk(v1,a1);
    v2 = *(int8 *)&v1->field_0x18;
  }
  *(uint8 *)&v1->field_0x18 = v2 + a1;
  a0->field_0x8 = a0->field_0x8 + a1;
  return;
}


// Function: sub_1ab50 @ 0x1ab50
void sub_1ab50(int8 *a0,int8 *a1,char *a2,void *a3,int8 a4)
{
  void *v1;
  int8 v2;
  uint8 v3; // rbx
  int8 v4; // rbx
  
  v2 = *a1;
  v1 = (void *)*a0;
  *a1 = v2 + 1;
  v3 = (v2 + 1) * 0x18;
  v4 = v3 - 0x18;
  v2 = sub_4c610(v1,v3);
  *a0 = v2;
  *(unsigned long *)(v2 + v4) = sub_4c7a0(a2);
  v2 = *a0;
  *(unsigned long *)(v2 + v4 + 8) = sub_4c770(a3,a4 + 1);
  *(int8 *)(*a0 + 0x10 + v4) = a4;
  return;
}


// Function: sub_1abd0 @ 0x1abd0
void sub_1abd0(int8 a0,unsigned long a1,void *a2,int8 a3)
{
  unsigned long v1; // rax
  
  v1 = sub_4c770(a2,a3 + 1);
  *(int8 *)(a0 + 0x50) = a3;
  *(unsigned long *)(a0 + 0x48) = v1;
  return;
}


// Function: sub_1ac00 @ 0x1ac00
void sub_1ac00(int8 a0,unsigned long a1,void *a2,int8 a3)
{
  unsigned long v1; // rax
  
  v1 = sub_4c770(a2,a3 + 1);
  *(int8 *)(a0 + 0x40) = a3;
  *(unsigned long *)(a0 + 0x38) = v1;
  return;
}


// Function: sub_1ac30 @ 0x1ac30
int8 sub_1ac30(char *a0)
{
  int4 v1; // eax
  char *v2;
  int8 v3; // r12
  
  v3 = 0x7f200;
  v2 = "atime";
  while( true ) {
    if (*(char *)(v3 + 0x1c)) // branch-flip
      v1 = strncmp(v2,a0,strlen(v2));
    else {
      v1 = strcmp(v2,a0);
    }
    if (!v1) break;
    v2 = *(char **)(v3 + 0x20);
    v3 += 0x20;
    if (!v2)
      return 0;
  }
  return v3;
}


// Function: sub_1acb0 @ 0x1acb0
void sub_1acb0(int8 *a0,unsigned long a1)
{
  char *v1;
  struct_10 *v2; // rax
  uint8 v3; // rax
  
  if (!a0)
    return;
  do {
    v2 = (struct_10 *)sub_1ac30((char *)a0[1]);
    if (v2) {
      v1 = (char *)a0[2];
      v3 = strlen(v1);
      (*v2->field_0x10)(a1,v2->field_0x0,v1,v3);
    }
    a0 = (int8 *)*a0;
  } while (a0);
  return;
}


// Function: sub_1ad20 @ 0x1ad20
void sub_1ad20(unsigned long *a0,char *a1,char *a2,unsigned long a3)
{
  int8 v1; // rax
  
  v1 = sub_1ac30(a1);
  if ((v1) && (*(uint1 *)(v1 + 0x18) & 2)) {
    (**(code **)(v1 + 0x10))(a0,a1,a2,a3); // jump-as-call
    return;
  }
  sub_1a2c0(a0,a1,a2); // tail-call
  return;
}


// Function: sub_1ad90 @ 0x1ad90
void sub_1ad90(char *a0,char *a1,uint8 a2,uint8 a3)
{
  char *v1; // rax
  char *v2; // rax
  char v3 [32];
  char v4 [24];
  
  v1 = (char *)sub_42f50(a2,v3);
  v2 = (char *)sub_43090(a3,v4);
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Extended header %s=%s is out of range %s..%s",5),a0,a1,v1,v2);
  dat_84d58 = 2;
  return;
}


// Function: sub_1ae40 @ 0x1ae40
unsigned int sub_1ae40(struct_99 *a0,char *a1,char *a2)
{
  unsigned int v1; // eax
  char *v2; // stack - 0x48
  char v3 [16];
  
  v3 = sub_242f0(a1,&v2,1);
  v1 = sub_23ba0(SUB168(v3,0),SUB168(v3,8));
  if ((char)v1) { // branch-flip
    a0->field_0x0 = SUB168(v3,0);
    a0->field_0x8 = SUB168(v3,8);
  }
  else if ((v2 <= a1) || (*v2)) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s=%s",5),a2,a1);
    dat_84d58 = 2;
  }
  else {
    sub_1ad90(a2,a1,0x8000000000000000,0x7fffffffffffffff);
  }
  return v1;
}


// Function: sub_1af40 @ 0x1af40
void sub_1af40(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ae40(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xf8) = v1;
  *(unsigned long *)(a0 + 0x100) = v2;
  return;
}


// Function: sub_1afa0 @ 0x1afa0
void sub_1afa0(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ae40(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0x108) = v1;
  *(unsigned long *)(a0 + 0x110) = v2;
  return;
}


// Function: sub_1b000 @ 0x1b000
void sub_1b000(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  if (!sub_1ae40(&v1,a2,a1))
    return;
  *(unsigned long *)(a0 + 0xe8) = v1;
  *(unsigned long *)(a0 + 0xf0) = v2;
  return;
}


// Function: sub_1b060 @ 0x1b060
void sub_1b060(unsigned long a0,char *a1,struct_5 *a2,char *a3)
{
  sub_1a880(a2,a1,a3,sub_1f4c0(a3)); // tail-call
  return;
}


// Function: sub_1b090 @ 0x1b090
void sub_1b090(int8 a0,char *a1,char *a2)
{
  uint8 v1;
  unsigned long v10;
  bool v2;
  bool v3;
  int4 *v4; // rax
  unsigned long v5;
  unsigned long *v6; // rdx
  char *v7; // stack - 0x48
  char *v8; // rsi
  char v9;
  
  *(unsigned long *)(a0 + 0x130) = 0;
  if ((uint4)((int4)*a2 - 0x30U) <= 9) {
    v4 = __errno_location();
    v2 = 1;
    do {
      *v4 = 0;
      v5 = strtoimax(a2,&v7,10);
      if (v2) {
        if (*v4 == 0x22) {
          sub_1ad90(a1,a2,0,0x7fffffffffffffff); // return-dupe
          return;
        }
      }
      else {
        if (*v4 == 0x22) {
          sub_1ad90(a1,a2,0,0x7fffffffffffffff);
          return;
        }
        v1 = *(uint8 *)(a0 + 0x130);
        if (*(uint8 *)(a0 + 0x138) <= v1) {
          if (dat_84d50)
            (*dat_84d50)();
          v8 = "Malformed extended header: excess %s=%s";
          error(0,0,dcgettext(NULL,v8,5),a1,a2); // return-dupe
          dat_84d58 = 2;
          return;
        }
        v6 = (unsigned long *)(v1 * 0x10 + *(int8 *)(a0 + 0x140));
        *(uint8 *)(a0 + 0x130) = v1 + 1;
        *v6 = v10;
        v6[1] = v5;
        v5 = v10;
      }
      v3 = (bool)(v2 ^ 1);
      v9 = *v7;
      if (!v9) {
        if (v2) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s: odd number of values",5),a1);
          dat_84d58 = 2;
          return;
        }
        return;
      }
      if (v9 != ',') {
        if (dat_84d50) {
          (*dat_84d50)();
          v9 = *v7;
        }
        error(0,0,dcgettext(NULL,"Malformed extended header: invalid %s: unexpected delimiter %c",5),a1,v9);
        dat_84d58 = 2;
        return;
      }
      a2 = &v7[1];
      v10 = v5;
      v2 = v3;
    } while ((uint4)((int4)v7[1] - 0x30U) <= 9);
  }
  if (dat_84d50)
    (*dat_84d50)();
  v8 = "Malformed extended header: invalid %s=%s";
  error(0,0,dcgettext(NULL,v8,5),a1,a2);
  dat_84d58 = 2;
  return;
}


// Function: sub_1b2e0 @ 0x1b2e0
void sub_1b2e0(unsigned long *a0,char *a1)
{
  char *v1;
  
  sub_1a5e0(a0,a1);
  sub_1a5e0(&a0[1],a1);
  v1 = (char *)a0[1];
  *(char *)&a0[2] = sub_3e630(v1);
  return;
}


// Function: sub_1b310 @ 0x1b310
void sub_1b310(unsigned long *a0,unsigned long a1,char *a2)
{
  *(char *)((int8)a0 + 0x151) = 1;
  sub_1b2e0(a0,a2); // tail-call
  return;
}


// Function: sub_1b340 @ 0x1b340
void sub_1b340(uint8 a0,unsigned long a1,char *a2,struct_5 *a3)
{
  char v1 [40];
  
  sub_1a980(a3,a2,(char *)sub_24270(a0,a1,v1));
  return;
}


// Function: sub_1b390 @ 0x1b390
void sub_1b390(int8 a0,char *a1,struct_5 *a2,struct_49 *a3)
{
  if (a3) {
    sub_1b340(a3->field_0x0,a3->field_0x8,a1,a2); // tail-call
    return;
  }
  sub_1b340(*(uint8 *)(a0 + 0xf8),*(unsigned long *)(a0 + 0x100),a1,a2); // tail-call
  return;
}


// Function: sub_1b530 @ 0x1b530
void sub_1b530(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b440(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x78) = v1[0];
  return;
}


// Function: sub_1b590 @ 0x1b590
void sub_1b590(int8 a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b440(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x74) = v1[0];
  return;
}


// Function: sub_1b640 @ 0x1b640
void sub_1b640(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b5f0(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_83f00 = v1;
  return;
}


// Function: sub_1b6a0 @ 0x1b6a0
void sub_1b6a0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b5f0(&v1,a2,0xffffffffffffffff,a1))
    return;
  dat_83f08 = v1;
  return;
}


// Function: sub_1b700 @ 0x1b700
void sub_1b700(int8 a0,char *a1,char *a2) // return-dupe x2
{
  uint8 v1;
  unsigned long v2; // stack - 0x28
  
  if (!sub_1b5f0(&v2,a2,0x7fffffffffffffff,a1))
    return;
  v1 = *(uint8 *)(a0 + 0x130);
  if (v1 < *(uint8 *)(a0 + 0x138)) {
    *(uint8 *)(a0 + 0x130) = v1 + 1;
    *(unsigned long *)(v1 * 0x10 + *(int8 *)(a0 + 0x140) + 8) = v2;
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Malformed extended header: excess %s=%s",5),a1,a2);
  dat_84d58 = 2;
  return;
}


// Function: sub_1b7e0 @ 0x1b7e0
void sub_1b7e0(int8 a0,unsigned long a1,char *a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  
  if (!sub_1b5f0(&v1,a2,0x7fffffffffffffff,a1))
    return;
  if (*(uint8 *)(a0 + 0x130) < *(uint8 *)(a0 + 0x138)) {
    *(unsigned long *)(*(uint8 *)(a0 + 0x130) * 0x10 + *(int8 *)(a0 + 0x140)) = v1;
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Malformed extended header: excess %s=%s",5),"GNU.sparse.offset",a2);
  dat_84d58 = 2;
  return;
}


// Function: sub_1b8b0 @ 0x1b8b0
void sub_1b8b0(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b5f0(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(char *)(a0 + 0x150) = 1;
  *(unsigned long *)(a0 + 0x148) = v1;
  return;
}


// Function: sub_1b920 @ 0x1b920
void sub_1b920(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b5f0(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x128) = v1[0];
  return;
}


// Function: sub_1b980 @ 0x1b980
void sub_1b980(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned int v1 [2]; // stack - 0x18
  
  if (!sub_1b5f0(v1,a2,0xffffffff,a1))
    return;
  *(unsigned int *)(a0 + 0x124) = v1[0];
  return;
}


// Function: sub_1b9e0 @ 0x1b9e0
void sub_1b9e0(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_1b5f0(&v1,a2,0x7fffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x88) = v1;
  return;
}


// Function: sub_1ba40 @ 0x1ba40
void sub_1ba40(int8 a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (!sub_1b5f0(&v2,a2,0xffffffffffffffff,a1))
    return;
  *(unsigned long *)(a0 + 0x138) = v2;
  v1 = sub_4c720(v2,0x10);
  *(unsigned long *)(a0 + 0x130) = 0;
  *(unsigned long *)(a0 + 0x140) = v1;
  return;
}


// Function: sub_1bb20 @ 0x1bb20
void sub_1bb20(int8 a0,char *a1,struct_5 *a2)
{
  sub_1bac0(*(unsigned int *)(a0 + 0x78),a1,0xffffffff,a2); // tail-call
  return;
}


// Function: sub_1bb40 @ 0x1bb40
void sub_1bb40(int8 a0,char *a1,struct_5 *a2)
{
  sub_1bac0(*(unsigned int *)(a0 + 0x74),a1,0xffffffff,a2); // tail-call
  return;
}


// Function: sub_1bef0 @ 0x1bef0
unsigned long sub_1bef0(char *a0) // return-dupe
{
  char *v1;
  int8 *v2;
  
  v2 = dat_84390;
  if (!dat_84390)
    return 0;
  do {
    v1 = (char *)v2[1];
    if (!fnmatch(v1,a0,0))
      return 1;
    v2 = (int8 *)*v2;
  } while (v2);
  return 0;
}


// Function: sub_1bf50 @ 0x1bf50
void sub_1bf50(unsigned long a0,char *a1,unsigned long a2,unsigned long a3)
{
  int8 v1; // rax
  
  if (!sub_1bef0(a1)) {
    if (!sub_1a1a0(a1)) {
      v1 = sub_1ac30(a1);
      if (v1) {
        (**(code **)(v1 + 0x10))(a0,a1,a2,a3); // jump-as-call
        return;
      }
      if (dat_83b82 & 2) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"Ignoring unknown extended header keyword \'%s\'",5),a1); // tail-call
        return;
      }
    }
  }
  return;
}


// Function: sub_1c010 @ 0x1c010
void sub_1c010(char *a0) // return-dupe
{
  uint1 *v1; // rax
  uint1 *v2; // rax
  
  v1 = (uint1 *)strtok(a0,",");
  if (!v1)
    return;
  do {
    v2 = (uint1 *)strchr((char *)v1,0x3d);
    if (!v2)
      sub_1a0f0(v1,0); // no-return
    sub_1a320(v1,v2);
    v1 = (uint1 *)strtok(NULL,",");
  } while (v1);
  return;
}


// Function: sub_1c080 @ 0x1c080
char * sub_1c080(unsigned long *a0,char *a1,uint8 a2)
{
  char *v1;
  char v10 [20];
  char v11 [12];
  char v12 [4];
  char *v13;
  char *v14;
  char *v15; // stack - 0xa8
  char *v16; // stack - 0xa0
  char v2;
  char v3;
  char *v4; // rax
  char *v5;
  char *v6;
  char *v7; // rax
  char *v8; // rax
  char v9 [20];
  
  v3 = *a1;
  if (v3) { // branch-flip
    v7 = NULL;
    v15 = NULL;
    v4 = NULL;
    v8 = NULL;
    v14 = NULL;
    v16 = NULL;
    v13 = a1;
    do {
      while( true ) {
        v2 = v13[1];
        v5 = &v13[1];
        if ((v3 == '%') && (v2)) break;
        v14 = &v14[1];
        v13 = v5;
        v3 = v2;
label_1c0ee:
        if (!v3) goto label_1c188;
      }
      if (v2 == 'f') {
        if (a0) { // branch-flip
          v7 = (char *)sub_3e390((char *)*a0);
          v5 = &v13[2];
          v14 = &v14[strlen(v7)];
          v1 = &v13[2];
          v13 = v5;
          v3 = *v1;
        }
        else {
label_1c3e8:
          v5 = &v13[2];
          v13 = &v13[2];
          v3 = *v5;
        }
        goto label_1c0ee;
      }
      if ('f' < v2) {
        if (v2 != 'n') { // branch-flip
          if (v2 != 'p') goto label_1c33a;
          v5 = (char *)sub_43090((int8)getpid(),v10);
          v15 = v5;
          v6 = v11;
        }
        else {
          v5 = (char *)sub_43090(a2,v9);
          v16 = v5;
          v6 = v12;
        }
        v1 = &v13[2];
        v14 = &v6[(int8)v14 - (int8)v5];
        v5 = &v13[2];
        v13 = v1;
        v3 = *v5;
        goto label_1c0ee;
      }
      if (v2 == '%') {
        v5 = &v13[2];
        v14 = &v14[1];
        v13 = &v13[2];
        v3 = *v5;
        goto label_1c0ee;
      }
      if (v2 != 'd') {
label_1c33a:
        v5 = &v13[2];
        v14 = &v14[2];
        v13 = &v13[2];
        v3 = *v5;
        goto label_1c0ee;
      }
      if (!a0) goto label_1c3e8;
      if (!v8)
        v8 = (char *)sub_3e4f0((char *)*a0);
      v4 = (char *)sub_33020(v8,0,(uint1)dat_84b92);
      v14 = &v14[strlen(v4)];
      v3 = v13[2];
      v13 = &v13[2];
    } while (v3);
label_1c188:
    v14 = &v14[1];
  }
  else {
    v15 = NULL;
    v7 = NULL;
    v4 = NULL;
    v8 = NULL;
    v16 = NULL;
    v14 = (char *)0x1;
  }
  v5 = (char *)sub_4c5b0(v14);
  v3 = *a1;
  v13 = v5;
  if (v3) {
label_1c1c0:
    v2 = a1[1];
    if (v3 != '%') { // branch-flip
      *v13 = v3;
      a1 = &a1[1];
      v13 = &v13[1];
      v3 = v2;
    }
    else {
      if (v2 != 'f') { // branch-flip
        if (v2 <= 'f') {
          if (v2 != '%') {
            if (v2 == 'd') {
              if (v4)
                v13 = stpcpy(v13,v4);
              goto label_1c1fd;
            }
            *v13 = '%';
            if (v2) {
label_1c350:
              v13[1] = v2;
              v1 = &a1[2];
              v13 = &v13[2];
              a1 = &a1[2];
              v3 = *v1;
              goto label_1c208;
            }
            v13 = &v13[1];
            goto label_1c210;
          }
          *v13 = '%';
          v1 = &a1[2];
          v13 = &v13[1];
          a1 = &a1[2];
          v3 = *v1;
          goto label_1c208;
        }
        if (v2 != 'n') { // branch-flip
          if (v2 != 'p') {
            *v13 = '%';
            goto label_1c350;
          }
          v13 = stpcpy(v13,v15);
        }
        else {
          v13 = stpcpy(v13,v16);
        }
label_1c2cd:
        v3 = a1[2];
      }
      else {
        if (v7) {
          v13 = stpcpy(v13,v7);
          goto label_1c2cd;
        }
label_1c1fd:
        v3 = a1[2];
      }
      a1 = &a1[2];
    }
label_1c208:
    if (!v3) goto label_1c210;
    goto label_1c1c0;
  }
  free(v8);
  goto label_1c22f;
label_1c210:
  free(v8);
  if (v5 < v13) {
    do {
      if (v13[-1] != '/') break;
      v13 = &v13[-1];
    } while (v5 != v13);
  }
label_1c22f:
  *v13 = '\0';
  return v5;
}


// Function: sub_1c460 @ 0x1c460
void sub_1c460(unsigned long *a0)
{
  if (dat_84370) {
    sub_1c080(a0,dat_84370,0); // tail-call
    return;
  }
  sub_23be0((unsigned long *)0x84370,*(char **)((uint8)dat_84a38 * 8 + 0x7f5e0));
  sub_1c080(a0,dat_84370,0); // tail-call
  return;
}


// Function: sub_1c4b0 @ 0x1c4b0
void sub_1c4b0(void)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3;
  
  v3 = dat_84358;
  if (!dat_84358) {
    v3 = *(char **)((uint8)dat_84a38 * 8 + 0x7f5f0);
    v1 = getenv("TMPDIR");
    if (!v1)
      v1 = "/tmp";
    v2 = strlen(v1);
    dat_84358 = (char *)sub_4c5b0(v2 + 1 + strlen(v3));
    v3 = strcat(strcpy(dat_84358,v1),v3);
  }
  sub_1c080(NULL,v3,dat_84398 + 1); // tail-call
  return;
}


// Function: sub_1c560 @ 0x1c560
void sub_1c560(void)
{
  if (!dat_84388)
    return;
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"can\'t update global extended header record",5));
  sub_2c970(2); // no-return
}


// Function: sub_1c5b0 @ 0x1c5b0
void sub_1c5b0(int8 a0)
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


// Function: sub_1c600 @ 0x1c600
void sub_1c600(unsigned long *a0,int8 a1)
{
  unsigned long *v1;
  unsigned long *v2; // rbx
  int8 v3; // rbp
  
  if (a1) {
    v3 = 0;
    v1 = a0;
    do {
      v3 += 1;
      v2 = &v1[3];
      free((void *)*v1);
      free((void *)v1[1]);
      v1 = v2;
    } while (a1 != v3);
  }
  free(a0); // tail-call
  return;
}


// Function: sub_1c650 @ 0x1c650
void sub_1c650(int8 a0,char *a1,void *a2,int8 a3)
{
  uint8 v1; // rax
  char *v2; // rax
  
  v1 = strlen(a1);
  v2 = (char *)sub_4c5b0(v1 + 1 + strlen("SCHILY.xattr."));
  builtin_strncpy(v2,"SCHILY.xattr.",0xd);
  strcpy(&v2[0xd],a1);
  sub_1ab50((int8 *)(a0 + 0x160),(int8 *)(a0 + 0x158),v2,a2,a3);
  free(v2); // tail-call
  return;
}


// Function: sub_1c6e0 @ 0x1c6e0
void sub_1c6e0(int8 a0,char *a1,void *a2,int8 a3)
{
  int8 v1;
  char *v10; // rsp
  char *v11;
  char *v12; // rsp
  char *v2;
  uint8 v3; // rax
  uint8 v4; // rdx
  void *v5; // rbx
  char v6 [8];
  char *v7;
  char *v8; // rsp
  char *v9; // rsp
  
  v7 = v6;
  v3 = strlen(a1);
  v9 = v6;
  v11 = v6;
  while (v9 != &v6[-(v3 + 0x18 & 0xfffffffffffff000)]) {
    v7 = &v11[-0x1000];
    v8 = &v11[-0x1000];
    *(unsigned long *)&v11[-8] = *(unsigned long *)&v11[-8];
    v9 = &v11[-0x1000];
    v11 = v8;
  }
  v4 = (uint8)((uint4)(v3 + 0x18) & 0xff0);
  v1 = -v4;
  v10 = &v7[v1];
  v11 = &v7[v1];
  if (v4)
    *(unsigned long *)&v7[-8] = *(unsigned long *)&v7[-8];
  v5 = (void *)((uint8)&v7[v1 + 0xf] & 0xfffffffffffffff0);
  *(unsigned long *)&v7[v1 + -8] = 0x1c776;
  memcpy(v5,a1,v3 + 1);
  v2 = &v7[v1];
  while (v10 != &v7[v1 - (a3 + 0x18U & 0xfffffffffffff000)]) {
    v11 = &v2[-0x1000];
    v12 = &v2[-0x1000];
    *(unsigned long *)&v2[-8] = *(unsigned long *)&v2[-8];
    v10 = &v2[-0x1000];
    v2 = v12;
  }
  v3 = (uint8)((uint4)(a3 + 0x18U) & 0xff0);
  v1 = -v3;
  if (v3)
    *(unsigned long *)&v11[-8] = *(unsigned long *)&v11[-8];
  *(unsigned long *)&v11[v1 + -8] = 0x1c7d2;
  memcpy((void *)((uint8)&v11[v1 + 0xf] & 0xfffffffffffffff0),a2,a3 + 1);
  *(unsigned long *)&v11[v1 + -8] = 0x1c7da;
  sub_19ff0(v5);
  *(unsigned long *)&v11[v1 + -8] = 0x1c7e6;
  v3 = strlen("SCHILY.xattr.");
  *(unsigned long *)&v11[v1 + -8] = 0x1c7f8;
  sub_1c650(a0,(char *)((int8)v5 + v3),(void *)((uint8)&v11[v1 + 0xf] & 0xfffffffffffffff0),a3);
  return;
}


// Function: sub_1c840 @ 0x1c840
void sub_1c840(int8 a0,int8 *a1,int8 *a2)
{
  unsigned long *v1;
  uint8 v2;
  uint8 v3; // rbx
  
  *a1 = 0;
  *a2 = 0;
  if (*(int8 *)(a0 + 0x158)) {
    v2 = 0;
    do {
      v3 = v2 + 1;
      v1 = (unsigned long *)(*(int8 *)(a0 + 0x160) + v2 * 0x18);
      sub_1ab50(a1,a2,(char *)*v1,(void *)v1[1],v1[2]);
      v2 = v3;
    } while (v3 < *(uint8 *)(a0 + 0x158));
    return;
  }
  return;
}


// Function: sub_1c8c0 @ 0x1c8c0
void sub_1c8c0(int8 a0) // return-dupe
{
  int8 v1; // stack - 0x28
  int8 v2;
  uint8 v3;
  
  sub_1acb0(dat_84388,a0);
  sub_1acb0(dat_84378,a0);
  v3 = *(uint8 *)(a0 + 0x170);
  if (v3) {
    v2 = *(int8 *)(a0 + 0x178);
    v1 = v2 + 0x200;
    while (sub_1bc70(v3,v2,&v1,sub_1bf50,a0)) {
      v3 = *(uint8 *)(a0 + 0x170);
      v2 = *(int8 *)(a0 + 0x178);
    }
  }
  sub_1acb0(dat_84380,a0);
  *(unsigned long *)(a0 + 0x118) = *(unsigned long *)(a0 + 0x88);
  if (!*(char *)(a0 + 0x150))
    return;
  *(unsigned long *)(a0 + 0x88) = *(unsigned long *)(a0 + 0x148);
  return;
}


// Function: sub_1c9a0 @ 0x1c9a0
void sub_1c9a0(int8 a0) // return-dupe
{
  int8 v1;
  uint8 v2;
  int8 v3; // stack - 0x38
  
  if (!*(int8 *)(a0 + 8))
    return;
  v3 = *(int8 *)(a0 + 0x10) + 0x200;
  sub_1bc20();
  do {
    v1 = *(int8 *)(a0 + 0x10);
    v2 = *(uint8 *)(a0 + 8);
  } while (sub_1bc70(v2,v1,&v3,sub_1ad20,0x84378));
  return;
}


// Function: sub_1ca30 @ 0x1ca30
void sub_1ca30(char *a0,int8 a1,unsigned long a2)
{
  int8 v1; // rax
  
  if (*(int8 *)(a1 + 0x178))
    return;
  v1 = sub_1ac30(a0);
  if ((v1) && (*(int8 *)(v1 + 8))) {
    if (!sub_1bef0(a0)) {
      sub_1a5a0((int8 *)(a1 + 0x168));
      if (!sub_1a1a0(a0)) {
        (**(code **)(v1 + 8))(a1,a0,(int8 *)(a1 + 0x168),a2); // jump-as-call
        return;
      }
    }
  }
  return;
}


// Function: sub_1cac0 @ 0x1cac0
void sub_1cac0(int8 a0,unsigned long *a1,int8 a2)
{
  unsigned long *v1;
  uint8 v10;
  int8 v11; // r14
  unsigned long *v2;
  int8 v3; // rax
  unsigned long *v4;
  int8 v5;
  uint8 v6; // rcx
  uint4 v7; // ebp
  uint8 v8;
  unsigned long *v9;
  
  v5 = 0;
  if (0 <= a2)
    v5 = a2;
  v11 = 0;
  v10 = v5 + 0x200;
  *(uint8 *)(a0 + 8) = v10;
  v3 = sub_4c5b0(v5 + 0x201);
  *(int8 *)(a0 + 0x10) = v3;
  *(char *)(v3 + 0x200 + v5) = 0;
  do {
    v8 = 0x200;
    if (v10 <= 0x200)
      v8 = v10;
    if (!a1) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      sub_163d0(); // no-return
    }
    v4 = (unsigned long *)(*(int8 *)(a0 + 0x10) + v11);
    v7 = (uint4)v8;
    if (8 <= v7) { // branch-flip
      *v4 = *a1;
      *(unsigned long *)((int8)v4 + ((v8 & 0xffffffff) - 8)) = *(unsigned long *)((int8)a1 + ((v8 & 0xffffffff) - 8));
      v5 = (int8)v4 - (int8)(unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      v6 = (uint8)((int4)v5 + v7 >> 3);
      v9 = (unsigned long *)((int8)a1 - v5);
      v4 = (unsigned long *)((uint8)&v4[1] & 0xfffffffffffffff8);
      while (v6) {
        v2 = &v4[1];
        v1 = &v9[1];
        *v4 = *v9;
        v6 -= 1;
        v9 = v1;
        v4 = v2;
      }
    }
    else if (v8 & 4) { // branch-flip
      *(unsigned int *)v4 = *(unsigned int *)a1;
      *(unsigned int *)((int8)v4 + ((v8 & 0xffffffff) - 4)) = *(unsigned int *)((int8)a1 + ((v8 & 0xffffffff) - 4));
    }
    else if ((v7) && (*(char *)v4 = *(char *)a1, v8 & 2))
      *(unsigned short *)((int8)v4 + ((v8 & 0xffffffff) - 2)) = *(unsigned short *)((int8)a1 + ((v8 & 0xffffffff) - 2));
    v10 -= v8;
    v11 += v8;
    sub_d1f0(a1);
    a1 = (unsigned long *)sub_e130();
  } while (1 <= (int8)v10);
  return;
}


// Function: sub_1cc10 @ 0x1cc10
void sub_1cc10(struct_6 *a0)
{
  unsigned long *v1;
  int8 v2;
  int8 v3;
  uint8 v4;
  uint8 v5; // rdx
  
  for (v1 = dat_84380; v1; v1 = (unsigned long *)*v1) {
    sub_1a9b0((char *)v1[2],(char *)v1[1],a0);
  }
  v2 = a0->field_0x0;
  v3 = *(int8 *)(v2 + 0x10);
  if (*(int8 *)(v2 + 0x18) == v3)
    *(uint1 *)(v2 + 0x50) = *(uint1 *)(v2 + 0x50) | 2;
  v5 = *(int8 *)(v2 + 0x18) + *(uint8 *)(v2 + 0x30) & ~*(uint8 *)(v2 + 0x30);
  v4 = *(uint8 *)(v2 + 0x20);
  if (v5 - *(int8 *)(v2 + 8) <= *(uint8 *)(v2 + 0x20) - *(int8 *)(v2 + 8))
    v4 = v5;
  *(uint8 *)(v2 + 0x18) = v4;
  *(uint8 *)(v2 + 0x10) = v4;
  a0->field_0x10 = v3;
  return;
}


// Function: sub_1cca0 @ 0x1cca0
void sub_1cca0(unsigned long *a0)
{
  if ((obstack *)*a0) { // branch-flip
    _obstack_free((obstack *)*a0,NULL);
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


// Function: sub_1ccf0 @ 0x1ccf0
void sub_1ccf0(uint1 a0,int8 a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  unsigned long *v1;
  int8 v10; // rbx
  unsigned long *v11;
  unsigned long *v12;
  unsigned long *v2;
  uint4 v3; // eax
  uint1 *v4; // rax
  unsigned long *v5;
  void *v6; // rax
  uint8 v7; // rax
  int8 v8; // rcx
  uint8 v9;
  
  v9 = a3[1];
  if (a0 != 0x67) { // branch-flip
    if ((a0 == 0x78) && (dat_84368))
      a2 = dat_84360;
  }
  else if (dat_84350)
    a2 = dat_84348;
  v4 = (uint1 *)sub_12cb0(a1,v9,a2);
  v3 = (uint4)v9;
  v4[0x9c] = a0;
  sub_12f00(v4);
  v10 = a3[2] + v9;
  while( true ) {
    v5 = (unsigned long *)sub_e130();
    v12 = (unsigned long *)(v10 - v9);
    if (v9 == (v3 & 0x1ff)) break;
    v9 -= 0x200;
    v8 = (int8)v5 - (int8)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    *v5 = *v12;
    v5[0x3f] = v12[0x3f];
    v7 = (uint8)((int4)v8 + 0x200U >> 3);
    v12 = (unsigned long *)((int8)v12 - v8);
    v11 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v7) {
      v2 = &v11[1];
      v1 = &v12[1];
      *v11 = *v12;
      v7 -= 1;
      v12 = v1;
      v11 = v2;
    }
    sub_d1f0(v5);
    if (!v9) goto label_1cdb8;
  }
  v6 = memcpy(v5,v12,v9);
  v7 = 0x200 - v9;
  v5 = (unsigned long *)(v9 + (int8)v6);
  v3 = (uint4)v7;
  if (8 <= v3) { // branch-flip
    *v5 = 0;
    *(unsigned long *)((int8)v5 + ((v7 & 0xffffffff) - 8)) = 0;
    v9 = (uint8)(v3 + ((int4)v5 - (int4)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8)) >> 3);
    v5 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v9) {
      v12 = &v5[1];
      *v5 = 0;
      v9 -= 1;
      v5 = v12;
    }
  }
  else if (v7 & 4) { // branch-flip
    *(unsigned int *)v5 = 0;
    *(unsigned int *)((int8)v5 + ((v7 & 0xffffffff) - 4)) = 0;
  }
  else if ((v3) && (*(char *)v5 = 0, v7 & 2))
    *(unsigned short *)((int8)v5 + ((v7 & 0xffffffff) - 2)) = 0;
  sub_d1f0(v6);
label_1cdb8:
  sub_1cca0(a3);
  if (a0 != 0x67)
    return;
  dat_84398 += 1;
  return;
}


// Function: sub_1ceb0 @ 0x1ceb0
void sub_1ceb0(struct_6 *a0)
{
  unsigned long *v1;
  void *v2; // rax
  
  if (dat_84388) {
    sub_1a5a0(a0);
    for (v1 = dat_84388; v1; v1 = (unsigned long *)*v1) {
      sub_1a9b0((char *)v1[2],(char *)v1[1],a0);
    }
  }
  if (a0->field_0x0) {
    sub_1cc10(a0);
    v2 = (void *)sub_1c4b0();
    sub_1ccf0(0x67,v2,dat_84a20,a0);
    free(v2); // tail-call
    return;
  }
  return;
}


// Function: sub_1cf40 @ 0x1cf40
void sub_1cf40(int8 a0)
{
  *(unsigned long *)(a0 + 0x18) = 0;
  return;
}


// Function: sub_1cf50 @ 0x1cf50
void sub_1cf50(int8 *a0,char *a1)
{
  uint8 v1; // rax
  
  if (a0[2])
    return;
  sub_1a5a0(a0);
  v1 = strlen(a1);
  a0[3] = a0[3] + v1;
  sub_1a7c0(a0,a1,strlen(a1)); // tail-call
  return;
}


// Function: sub_1cfa0 @ 0x1cfa0
unsigned long sub_1cfa0(struct_100 *a0,char *a1) // return-dupe
{
  int8 v1;
  uint8 v2; // rax
  char *v3; // rax
  uint8 v4; // rax
  char *v5;
  char v6 [20];
  char v7 [4];
  char *v8; // rbp
  bool v9; // zf
  
  if (a0->field_0x10)
    return 0;
  sub_1a5a0(a0);
  v2 = strlen(a1);
  v1 = a0->field_0x18;
  v5 = NULL;
  do {
    v3 = (char *)sub_43090(&v5[v1 + 3 + v2],v6);
    v9 = v5 != &v7[-(int8)v3];
    v5 = &v7[-(int8)v3];
  } while (v9);
  v2 = strlen(a1);
  sub_1ab10(a0,&v7[v2 + -(int8)v3 + 2]);
  sub_1a830(a0,10);
  v1 = *(int8 *)(a0->field_0x0 + 0x18);
  v4 = ~a0->field_0x18;
  v8 = (char *)((v4 - (int8)&v7[v2 + -(int8)v3 + 2]) + v1);
  memmove((void *)(v1 + v4),v8,a0->field_0x18);
  v3 = stpcpy(v8,v3);
  *v3 = ' ';
  *stpcpy(&v3[1],a1) = '=';
  return 1;
}


// Function: sub_1d0c0 @ 0x1d0c0
uint8 sub_1d0c0(int8 a0,uint8 a1)
{
  return (uint8)(*(int8 *)(a0 + 0x18) + *(int8 *)(a0 + 0x20)) % a1;
}


// Function: sub_1d0e0 @ 0x1d0e0
unsigned long sub_1d0e0(int8 a0,int8 a1)
{
  if (*(int8 *)(a0 + 0x18) != *(int8 *)(a1 + 0x18))
    return 0;
  return CONCAT71((undefined7)((uint8)*(int8 *)(a1 + 0x20) >> 8),*(int8 *)(a0 + 0x20) == *(int8 *)(a1 + 0x20));
}


// Function: sub_1d110 @ 0x1d110
char * sub_1d110(struct_41 *a0)
{
  uint8 v1;
  int8 *v2;
  int8 v3;
  char *v4; // r12
  
  v1 = a0->field_0x10;
  v2 = a0->field_0x0;
  if (a0->field_0x8) { // branch-flip
    v4 = (char *)(*v2 + v1);
    if (*v4) // branch-flip
      a0->field_0x10 = v1 + 1 + strlen(v4);
    else {
      v4 = NULL;
    }
  }
  else {
    v4 = NULL;
    if (v1 < (uint8)v2[2]) {
      v3 = *(int8 *)(v2[3] + v1 * 8);
      a0->field_0x10 = v1 + 1;
      return (char *)(v3 + -1);
    }
  }
  return v4;
}


// Function: sub_1d180 @ 0x1d180
void sub_1d180(void *a0)
{
  free(*(void **)((int8)a0 + 0x50));
  free(*(void **)((int8)a0 + 0x58));
  free(a0); // tail-call
  return;
}


// Function: sub_1d1a0 @ 0x1d1a0
void sub_1d1a0(struct_42 *a0)
{
  free(a0->field_0x18);
  free(a0); // tail-call
  return;
}


// Function: sub_1d1c0 @ 0x1d1c0
void sub_1d1c0(char *a0,char *a1) // return-dupe
{
  uint8 v1; // rax
  uint8 v2; // rax
  int8 *v3;
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3 = dat_843c0;
  if (!dat_843c0)
    return;
  do {
    sub_23fc0(&v3[0xb],a0,v1,a1,v2);
    v3 = (int8 *)*v3;
  } while (v3);
  return;
}


// Function: sub_1d220 @ 0x1d220
void sub_1d220(int8 *a0,int4 a1,unsigned long *a2)
{
  struct_41 *v1; // rax
  
  v1 = (struct_41 *)sub_4c5b0(0x18);
  v1->field_0x0 = a0;
  v1->field_0x8 = a1;
  v1->field_0x10 = 0;
  *a2 = v1;
  sub_1d110(v1); // tail-call
  return;
}


// Function: sub_1d260 @ 0x1d260
struct_17 * sub_1d260(char *a0,unsigned long a1)
{
  uint8 v1;
  struct_17 *v2; // rax
  void *v3; // rax
  uint8 v4;
  
  v1 = strlen(a0);
  v2 = (struct_17 *)sub_4c5b0(0x60);
  v2->field_0x0 = 0;
  v2->field_0x30 = 0;
  v2->field_0x28 = 0;
  v2->field_0x40 = 0;
  v2->field_0x3c = 0;
  if ((2 <= v1) && (v4 = v1 - 1, a0[v1 - 1] == '/')) {
  }
  else {
    v4 = v1;
    v1 += 1;
  }
  v3 = (void *)sub_4c5b0(v1);
  v2->field_0x58 = v3;
  v3 = memcpy(v3,a0,v4);
  v2->field_0x50 = a1;
  *(char *)((int8)v3 + v4) = 0;
  v2->field_0x48 = 0;
  return v2;
}


// Function: sub_1d310 @ 0x1d310
unsigned long sub_1d310(char *a0)
{
  unsigned long v1; // rax
  unsigned long v2;
  
  v2 = 0;
  if (dat_843b0) {
    v1 = sub_1d260(a0,sub_25340(dat_84580,a0));
    v2 = sub_418d0(dat_843b0,v1);
    sub_1d180(v1);
  }
  return v2;
}


// Function: sub_1d370 @ 0x1d370
unsigned long sub_1d370(unsigned long a0,unsigned long a1)
{
  int8 v1; // rax
  unsigned long v2;
  
  v2 = 0;
  if (dat_843a8) {
    v1 = sub_1d260("",0);
    *(unsigned long *)(v1 + 0x18) = a0;
    *(unsigned long *)(v1 + 0x20) = a1;
    v2 = sub_418d0(dat_843a8,v1);
    sub_1d180(v1);
  }
  return v2;
}


// Function: sub_1d3d0 @ 0x1d3d0
void sub_1d3d0(char *a0)
{
  unsigned long v1; // rax
  
  v1 = sub_1d260(a0,sub_25340(dat_84580,a0));
  if (dat_843b8) {
    *dat_843b8 = v1;
    dat_843b8 = (unsigned long *)v1;
    return;
  }
  dat_843c0 = v1;
  dat_843b8 = (unsigned long *)v1;
  return;
}


// Function: sub_1d420 @ 0x1d420
void sub_1d420(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
  return;
}


// Function: sub_1d430 @ 0x1d430
unsigned int sub_1d430(int8 a0,int8 a1)
{
  uint4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x50),*(char **)(a1 + 0x50));
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_1d450 @ 0x1d450
int8 sub_1d450(int8 a0,unsigned long a1)
{
  int8 *v1; // rax
  unsigned long v2 [2]; // stack - 0x10
  
  if (a0) {
    v2[0] = a1;
    v1 = (int8 *)bsearch(v2,*(unsigned long *)(a0 + 0x18),*(unsigned long *)(a0 + 0x10),8,sub_1d420);
    if (v1)
      return *v1 + -1;
  }
  return 0;
}


// Function: sub_1d4a0 @ 0x1d4a0
unsigned long * sub_1d4a0(char *a0,char *a1)
{
  int8 v1;
  int8 v10; // stack - 0x50
  unsigned long *v2; // rax
  unsigned long *v3;
  int8 v4; // rbx
  uint8 v5;
  uint8 v6;
  uint8 v7; // r12
  char v8;
  char *v9;
  
  v8 = *a0;
  if (v8) { // branch-flip
    v10 = 0;
    v7 = 1;
    v4 = 0;
    v9 = a0;
    do {
      v1 = strlen(v9) + 1;
      v7 += v1;
      if (a1) { // branch-flip
        if (strchr(a1,(int4)v8)) goto label_1d505;
      }
      else {
label_1d505:
        v10 += 1;
      }
      v9 = &v9[v1];
      v4 += 1;
      v8 = *v9;
    } while (v8);
    v5 = v7 + 0x20;
    v6 = v10 + 1;
  }
  else {
    v10 = 0;
    v6 = 1;
    v5 = 0x21;
    v4 = 0;
    v7 = 1;
  }
  v2 = (unsigned long *)sub_4c5b0(v5);
  *v2 = &v2[4];
  memcpy(&v2[4],a0,v7);
  v2[1] = v4;
  v2[2] = v10;
  v3 = (unsigned long *)sub_4c720(v6,8);
  v9 = (char *)*v2;
  v2[3] = v3;
  v8 = *v9;
  if (v8) {
    v4 = 0;
    do {
      if (a1) { // branch-flip
        if (strchr(a1,(int4)v8)) goto label_1d592;
      }
      else {
label_1d592:
        v3[v4] = &v9[1];
        v4 += 1;
      }
      v9 = &v9[strlen(v9) + 1];
      v8 = *v9;
    } while (v8);
    v3 = &v3[v4];
  }
  *v3 = 0;
  return v2;
}


// Function: sub_1d600 @ 0x1d600
void sub_1d600(int8 a0,char *a1)
{
  char v1;
  int8 v10;
  int8 v11;
  unsigned long *v12;
  char *v13; // stack - 0x40
  unsigned long v2;
  uint1 v3;
  uint8 v4; // rax
  unsigned long *v5; // rax
  char *v6;
  char *v7;
  char *v8;
  uint8 v9; // rbp
  
  v10 = 0;
  if (*(int4 *)(a0 + 0x38) != 2) { // branch-flip
    v11 = *(int8 *)(a0 + 0x40);
    if (v11) {
      v10 = *(int8 *)(v11 + 0x30);
      if (!v10)
        v10 = *(int8 *)(v11 + 0x28);
      goto label_1d637;
    }
    v10 = *(int8 *)(a0 + 0x28);
    if (!*a1) goto label_1d78c;
label_1d640:
    v11 = 0;
    v9 = 0;
    v8 = a1;
    do {
      v9 += 1;
      v4 = strlen(v8);
      v8 = &v8[v4 + 1];
      v11 = v4 + 2 + v11;
    } while (*v8);
    v13 = (uint8)(v11 + 1);
    v5 = (unsigned long *)sub_4c720(v9,8);
    if (*a1) goto label_1d68b;
  }
  else {
label_1d637:
    if (*a1) goto label_1d640;
label_1d78c:
    v9 = 0;
    v5 = (unsigned long *)sub_4c720(0,8);
    v13 = (char *)0x1;
    if (!*a1) {
      qsort(v5,0,8,sub_1d420);
      v8 = (char *)sub_4c5b0(1);
      v13 = v8;
      goto label_1d72b;
    }
label_1d68b:
    v12 = v5;
    do {
      *v12 = a1;
      v12 = &v12[1];
      a1 = &a1[strlen(a1) + 1];
    } while (*a1);
  }
  v4 = 0;
  qsort(v5,v9,8,sub_1d420);
  v13 = (char *)sub_4c5b0(v13);
  v8 = v13;
  if (v9) {
    do {
      v2 = v5[v4];
      if (sub_1d450(v10,v2)) // branch-flip
        v3 = -(*(int8 *)(a0 + 0x48) == 0) & 0xd7;
      else {
        v3 = -(*(int8 *)(a0 + 0x48) == 0) & 0x10;
      }
      *v8 = v3 + 0x49;
      v6 = (char *)v5[v4];
      v7 = &v8[1];
      do {
        v1 = *v6;
        v6 = &v6[1];
        v8 = &v7[1];
        *v7 = v1;
        v7 = v8;
      } while (v1);
      v4 += 1;
    } while (v9 != v4);
  }
label_1d72b:
  v2 = *(unsigned long *)(a0 + 0x28);
  *v8 = '\0';
  *(unsigned long *)(a0 + 0x30) = v2;
  *(unsigned long *)(a0 + 0x28) = sub_1d4a0(v13,NULL);
  free(v13);
  free(v5); // tail-call
  return;
}


// Function: sub_1d7f0 @ 0x1d7f0
unsigned long * sub_1d7f0(char *a0)
{
  return sub_1d4a0(a0,"YND"); // tail-call
}


// Function: sub_1d800 @ 0x1d800
void sub_1d800(int8 a0)
{
  void *v1;
  unsigned long v2;
  uint8 v3; // rax
  char *v4; // rax
  
  v3 = strlen(*(char **)(a0 + 0x48));
  v4 = (char *)sub_4c5b0(v3 + 3);
  v1 = *(void **)(a0 + 0x48);
  *v4 = 'Y';
  memcpy(&v4[1],v1,v3 + 1);
  v2 = *(unsigned long *)(a0 + 0x28);
  v4[v3 + 2] = '\0';
  *(unsigned long *)(a0 + 0x30) = v2;
  *(unsigned long *)(a0 + 0x28) = sub_1d4a0(v4,NULL);
  free(v4); // tail-call
  return;
}


// Function: sub_1d860 @ 0x1d860
unsigned long sub_1d860(uint1 *a0)
{
  bool v1;
  uint1 v2;
  char *v3; // rsi
  uint1 v4; // r12b
  
  v2 = *a0;
  if (v2) {
    v4 = 0;
    v1 = 0;
    do {
      if (v4) { // branch-flip
        if (v2 != v4) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"Malformed dumpdir: expected \'%c\' but found %#3o",5),v4,(uint4)v2);
          dat_84d58 = 2;
          return 0;
        }
        if (v2 != 0x54) goto label_1d8d3;
        if (v4 != 0x54) goto label_1da57;
        if ((!a0[1]) && (!v1)) {
          if (dat_84d50)
            (*dat_84d50)();
          v3 = "Malformed dumpdir: empty name in \'T\'";
          error(0,0,dcgettext(NULL,v3,5)); // return-dupe
          dat_84d58 = 2;
          return 0;
        }
        v4 = 0;
      }
      else {
        if (v2 == 0x54) {
label_1da57:
          if (dat_84d50)
            (*dat_84d50)();
          v3 = "Malformed dumpdir: \'T\' not preceded by \'R\'";
          error(0,0,dcgettext(NULL,v3,5));
          dat_84d58 = 2;
          return 0;
        }
label_1d8d3:
        if (v2 != 0x58) { // branch-flip
          if (v2 == 0x52) {
            if (a0[1]) // branch-flip
              v4 = 0x54;
            else {
              if (!v1) {
                if (dat_84d50)
                  (*dat_84d50)();
                v3 = "Malformed dumpdir: empty name in \'R\'";
                error(0,0,dcgettext(NULL,v3,5));
                dat_84d58 = 2;
                return 0;
              }
              v4 = 0x54;
              v1 = 0;
            }
          }
        }
        else {
          if (v1) {
            if (dat_84d50)
              (*dat_84d50)();
            v3 = "Malformed dumpdir: \'X\' duplicated";
            error(0,0,dcgettext(NULL,v3,5));
            dat_84d58 = 2;
            return 0;
          }
          v1 = 1;
        }
      }
      a0 = &a0[strlen((char *)a0) + 1];
      v2 = *a0;
    } while (v2);
    if (v4) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Malformed dumpdir: expected \'%c\' but found end of data",5),v4);
      dat_84d58 = 2;
      return 0;
    }
    if ((v1) && (dat_83b80 & 2)) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Malformed dumpdir: \'X\' never used",5));
    }
  }
  return 1;
}


// Function: sub_1dac0 @ 0x1dac0
int8 sub_1dac0(char *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,bool a5,unsigned int a6,char *a7)
{
  unsigned int v1;
  char *v2;
  int8 v3; // rax
  unsigned long v4;
  int8 v5;
  
  v2 = a7;
  v1 = a6;
  v3 = sub_1d3d0(a0);
  *(unsigned long *)(v3 + 0x10) = a2;
  *(unsigned long *)(v3 + 8) = a1;
  *(unsigned long *)(v3 + 0x18) = a3;
  *(unsigned long *)(v3 + 0x20) = a4;
  *(unsigned int *)(v3 + 0x38) = 1;
  if (a5)
    *(uint4 *)(v3 + 0x3c) = *(uint4 *)(v3 + 0x3c) | 2;
  if ((char)v1)
    *(uint4 *)(v3 + 0x3c) = *(uint4 *)(v3 + 0x3c) | 4;
  v4 = 0;
  if (v2)
    v4 = sub_1d7f0(v2);
  *(unsigned long *)(v3 + 0x28) = v4;
  v5 = dat_843b0;
  if ((((dat_843b0) || (v5 = sub_41b30(0,0,sub_1e1a0,sub_1d430,0), dat_843b0 = v5, v5)) && (sub_42140(v5,v3))) && (((v5 = dat_843a8, dat_843a8 || (v5 = sub_41b30(0,0,sub_1d0c0,sub_1d0e0,0), dat_843a8 = v5, v5)) && (sub_42140(v5,v3)))))
    return v3;
  sub_16420(); // no-return
}


// Function: sub_1dbe0 @ 0x1dbe0
int8 sub_1dbe0(char *a0,int8 a1,uint4 a2,char *a3)
{
  uint8 v1;
  char *v10;
  char *v11; // rax
  unsigned long v12; // stack - 0x48
  char v13 [16];
  unsigned long v2;
  unsigned long v3;
  bool v4;
  uint4 v5;
  int4 v6;
  unsigned int v7; // eax
  int8 v8; // rax
  int8 v9;
  
  v1 = *(uint8 *)(a1 + 0x58);
  v8 = sub_1d310(a0);
  if (v8) { // branch-flip
    if (*(uint1 *)(v8 + 0x3c) & 1) { // branch-flip
      if (a2 & 0x20) {
        sub_23be0((unsigned long *)(v8 + 0x58),a0);
        goto label_1dc3b;
      }
    }
    else {
label_1dc3b:
      v10 = *(char **)(v8 + 0x58);
      if (!strcmp(v10,a0)) {
        if ((dat_84b20) && (((!(*(uint1 *)(v8 + 0x3c) & 2) || (0 <= (int8)v1)) && (v9 = *(int8 *)(a1 + 0x58), *(int8 *)(v8 + 0x18) != v9)))) {
label_1dc86:
          v9 = sub_1d370(v9,*(int8 *)(a1 + 0x60));
          if (v9) { // branch-flip
            v10 = *(char **)(v9 + 0x58);
            if (strcmp(v10,a0)) {
              if (dat_83b81 & 0x20) {
                if (dat_84d50) {
                  (*dat_84d50)();
                  v10 = *(char **)(v9 + 0x58);
                }
                v10 = (char *)sub_4ac40(1,v10);
                v11 = (char *)sub_4aa70(a0);
                error(0,0,dcgettext(NULL,"%s: Directory has been renamed from %s",5),v11,v10);
                v10 = *(char **)(v9 + 0x58);
              }
              *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) | 0x10;
              *(int8 *)(v8 + 0x40) = v9;
              *(uint4 *)(v9 + 0x3c) = *(uint4 *)(v9 + 0x3c) & 0xffffffef;
              sub_1d1c0(v10,a0);
            }
            *(unsigned int *)(v8 + 0x38) = 1;
            v4 = 0;
          }
          else {
            v3 = *(unsigned long *)(a1 + 0x58);
            *(unsigned int *)(v8 + 0x38) = 2;
            v4 = 1;
            *(unsigned long *)(v8 + 0x18) = v3;
            *(unsigned long *)(v8 + 0x20) = *(unsigned long *)(a1 + 0x60);
          }
          v5 = *(uint4 *)(v8 + 0x3c);
          if ((int8)v1 <= -1)
            v5 = *(uint4 *)(v8 + 0x3c) | 2;
        }
        else {
          if (*(int8 *)(v8 + 0x20) != *(int8 *)(a1 + 0x60)) {
            v9 = *(int8 *)(a1 + 0x58);
            goto label_1dc86;
          }
          *(unsigned int *)(v8 + 0x38) = 1;
          v5 = *(uint4 *)(v8 + 0x3c);
          v4 = 0;
        }
        *(uint4 *)(v8 + 0x3c) = v5 | 4;
        goto label_1dd95;
      }
    }
    *a3 = 0x4e;
  }
  else {
    v9 = sub_1d370(*(unsigned long *)(a1 + 0x58),*(unsigned long *)(a1 + 0x60));
    v3 = *(unsigned long *)(a1 + 0x60);
    v2 = *(unsigned long *)(a1 + 0x58);
    v13 = sub_4bd20(a1 + 0x58);
    v8 = sub_1dac0(a0,SUB168(v13,0),SUB168(v13,8),v2,v3,v1 >> 0x3f);
    if (v9) { // branch-flip
      v10 = *(char **)(v9 + 0x58);
      if (strcmp(v10,a0)) {
        if (dat_83b81 & 0x20) {
          if (dat_84d50) {
            (*dat_84d50)();
            v10 = *(char **)(v9 + 0x58);
          }
          v10 = (char *)sub_4ac40(1,v10);
          v11 = (char *)sub_4aa70(a0);
          error(0,0,dcgettext(NULL,"%s: Directory has been renamed from %s",5),v11,v10);
          v10 = *(char **)(v9 + 0x58);
        }
        *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) | 0x10;
        *(int8 *)(v8 + 0x40) = v9;
        *(uint4 *)(v9 + 0x3c) = *(uint4 *)(v9 + 0x3c) & 0xffffffef;
        sub_1d1c0(v10,a0);
      }
      *(unsigned int *)(v8 + 0x38) = 1;
      v4 = 0;
    }
    else {
      *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) | 8;
      if (dat_83b81 & 0x10) {
        if (dat_84d50)
          (*dat_84d50)();
        v10 = (char *)sub_4aa70(a0);
        error(0,0,dcgettext(NULL,"%s: Directory is new",5),v10);
      }
      if (dat_84b28) { // branch-flip
label_1e09c:
        v7 = 2;
      }
      else {
        v13 = sub_4bd20(a1 + 0x58);
        v2 = dat_84b08;
        v3 = dat_84b00;
        v6 = sub_4be80(SUB168(v13,0),SUB168(v13,8),dat_84b00,dat_84b08);
        if (v6 < 0) goto label_1e09c;
        if (dat_84b8c) {
          v13 = sub_4bd10(a1 + 0x58);
          if ((int4)sub_4be80(SUB168(v13,0),SUB168(v13,8),v3,v2) < 0) goto label_1e09c;
        }
        v7 = 1;
      }
      *(unsigned int *)(v8 + 0x38) = v7;
      v4 = 0;
    }
label_1dd95:
    if (((dat_84ad1) && (*(int8 *)(a1 + 0x198))) && (*(int8 *)(a1 + 0x58) != *(int8 *)(*(int8 *)(a1 + 0x198) + 0x58))) {
      if (dat_83b82 & 4) {
        if (dat_84d50)
          (*dat_84d50)();
        v10 = (char *)sub_4aa70(*(char **)(v8 + 0x58));
        error(0,0,dcgettext(NULL,"%s: directory is on a different filesystem; not dumped",5),v10);
      }
      *(unsigned int *)(v8 + 0x38) = 0;
      if (!*(struct_42 **)(v8 + 0x28)) {
        *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) | 1;
        return v8;
      }
      sub_1d1a0(*(struct_42 **)(v8 + 0x28));
      *(unsigned long *)(v8 + 0x28) = 0;
      v6 = *(int4 *)(v8 + 0x38);
    }
    else {
      if ((a2 & 0x10) && (*(uint4 *)(v8 + 0x38) = a2 & 3, !(a2 & 3)))
        *a3 = 0x4e;
      if ((v4) && (dat_83b81 & 0x20)) {
        if (dat_84d50)
          (*dat_84d50)();
        v10 = (char *)sub_4aa70(a0);
        error(0,0,dcgettext(NULL,"%s: Directory has been renamed",5),v10);
      }
      v6 = *(int4 *)(v8 + 0x38);
    }
    *(uint4 *)(v8 + 0x3c) = *(uint4 *)(v8 + 0x3c) | 1;
    if (v6) {
      v6 = sub_14160(a1,&v12);
      if (v6 != 2) { // branch-flip
        if (v6 != 3) { // branch-flip
          if (v6 == 1) {
            sub_12ad0(a0,v12,dcgettext(NULL,"contents not dumped",5));
            *(unsigned int *)(v8 + 0x38) = 0;
            *(unsigned long *)(v8 + 0x48) = v12;
          }
        }
        else {
          sub_12ad0(a0,v12,dcgettext(NULL,"directory not dumped",5));
          *a3 = 0x4e;
          *(unsigned int *)(v8 + 0x38) = 0;
        }
      }
      else {
        sub_12ad0(a0,v12,dcgettext(NULL,"contents not dumped",5));
        *(unsigned long *)(v8 + 0x48) = v12;
      }
    }
  }
  return v8;
}


// Function: sub_1e1a0 @ 0x1e1a0
uint8 sub_1e1a0(int8 a0,uint8 a1)
{
  uint1 *v1;
  
  v1 = *(uint1 **)(a0 + 0x50);
  return sub_41ac0(v1,a1); // tail-call
}


// Function: sub_1e1b0 @ 0x1e1b0
void sub_1e1b0(obstack *a0,char *a1,char *a2)
{
  uint8 v1;
  char *v2;
  void *v3;
  
  if (*a1) { // branch-flip
    a1 = (char *)sub_33020(a1,0,(uint1)dat_84b92);
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 == v2) goto label_1e29a;
  }
  else {
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 == v2) {
label_1e29a:
      _obstack_newchunk(a0,1);
      v2 = *(char **)&a0->field_0x18;
    }
  }
  *(char **)&a0->field_0x18 = &v2[1];
  *v2 = 0x52;
  v1 = strlen(a1);
  v3 = *(void **)&a0->field_0x18;
  v1 += 1;
  if ((uint8)(*(int8 *)&a0->field_0x20 - (int8)v3) < v1) {
    _obstack_newchunk(a0,v1);
    v3 = *(void **)&a0->field_0x18;
  }
  memcpy(v3,a1,v1);
  v2 = (char *)(*(int8 *)&a0->field_0x18 + v1);
  *(char **)&a0->field_0x18 = v2;
  if (*a2) { // branch-flip
    a2 = (char *)sub_33020(a2,0,(uint1)dat_84b92);
    v2 = *(char **)&a0->field_0x18;
    if (*(char **)&a0->field_0x20 != v2) goto label_1e230;
  }
  else if (*(char **)&a0->field_0x20 != v2) goto label_1e230;
  _obstack_newchunk(a0,1);
  v2 = *(char **)&a0->field_0x18;
label_1e230:
  *(char **)&a0->field_0x18 = &v2[1];
  *v2 = 0x54;
  v1 = strlen(a2);
  v3 = *(void **)&a0->field_0x18;
  v1 += 1;
  if ((uint8)(*(int8 *)&a0->field_0x20 - (int8)v3) < v1) {
    _obstack_newchunk(a0,v1);
    v3 = *(void **)&a0->field_0x18;
  }
  memcpy(v3,a2,v1);
  *(uint8 *)&a0->field_0x18 = *(int8 *)&a0->field_0x18 + v1;
  return;
}


// Function: sub_1e320 @ 0x1e320
void sub_1e320(int8 a0,obstack *a1)
{
  char *v1; // rax
  char *v2;
  uint8 v3;
  int8 v4;
  void *v5;
  
  v4 = a0;
  while( true ) {
    if (!v4) {
      for (; (a0 && (*(int8 *)(a0 + 0x40))); a0 = *(int8 *)(a0 + 0x40)) {
        sub_1e1b0(a1,*(char **)(*(int8 *)(a0 + 0x40) + 0x58),*(char **)(a0 + 0x58));
      }
      return;
    }
    if (*(int8 *)(v4 + 0x40) == a0) break;
    v4 = *(int8 *)(v4 + 0x40);
  }
  v1 = (char *)sub_3e4f0(*(char **)(a0 + 0x58));
  v2 = *(char **)&a1->field_0x18;
  if (*(char **)&a1->field_0x20 == v2) {
    _obstack_newchunk(a1,1);
    v2 = *(char **)&a1->field_0x18;
  }
  *(char **)&a1->field_0x18 = &v2[1];
  *v2 = 0x58;
  v3 = strlen(v1);
  v5 = *(void **)&a1->field_0x18;
  v3 += 1;
  if ((uint8)(*(int8 *)&a1->field_0x20 - (int8)v5) < v3) {
    _obstack_newchunk(a1,v3);
    v5 = *(void **)&a1->field_0x18;
  }
  memcpy(v5,v1,v3);
  *(uint8 *)&a1->field_0x18 = *(int8 *)&a1->field_0x18 + v3;
  sub_1e1b0(a1,*(char **)(a0 + 0x58),"");
  for (; v4 != a0; a0 = *(int8 *)(a0 + 0x40)) {
    sub_1e1b0(a1,*(char **)(*(int8 *)(a0 + 0x40) + 0x58),*(char **)(a0 + 0x58));
  }
  sub_1e1b0(a1,"",*(char **)(v4 + 0x58));
  free(v1); // tail-call
  return;
}


// Function: sub_1e470 @ 0x1e470
void sub_1e470(int4 a0,char *a1)
{
  char v1;
  int8 v10; // stack - 0x50
  char *v11; // stack - 0x58
  unsigned long v12; // rax
  int8 v13; // rbp
  char v14 [16];
  char **v15; // stack - 0x70
  int8 v16; // stack - 0x68
  int4 v2; // eax
  char *v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  int4 *v6; // rax
  uint8 v7; // rax
  char *v8; // rax
  char *v9; // stack - 0x48
  
  v11 = NULL;
  v10 = 0;
  if (a0 != 1) { // branch-flip
    v3 = strdup(a1);
    v11 = v3;
    v10 = strlen(v3) + 1;
    v14 = sub_242f0(v3,&v9,0);
    dat_84b08 = SUB168(v14,8);
    dat_84b00 = SUB168(v14,0);
    v1 = sub_23ba0(dat_84b00,dat_84b08);
    v16 = 1;
    if (!v1) {
      v13 = 1;
label_1e88b:
      if (dat_84d50)
        (*dat_84d50)();
      v3 = dcgettext(NULL,"Invalid time stamp",5);
      v8 = (char *)sub_4aa70(dat_84b28);
      error(0,*__errno_location(),"%s:%ld: %s",v8,v13,v3);
      sub_163d0(); // no-return
    }
  }
  else {
    if (getline(&v11,&v10,dat_843a0) <= 0) {
      sub_32820(dat_84b28);
      free(v11);
      return;
    }
    v14 = sub_242f0(v11,&v9,0);
    dat_84b08 = SUB168(v14,8);
    dat_84b00 = SUB168(v14,0);
    v1 = sub_23ba0(dat_84b00,dat_84b08);
    if (!v1) {
      v13 = 2;
      goto label_1e88b;
    }
    v16 = 2;
    if (*v9) {
      v3 = &v9[1];
      v6 = __errno_location();
      *v6 = 0;
      v7 = strtoumax(v3,&v9,10);
      if (!*v6) {
        if (1000000000 <= v7) // branch-flip
          *v6 = 0x22;
        else if (v9 != v3) {
          v16 = 2;
          dat_84b08 = v7;
          goto label_1e5ad;
        }
      }
      if (dat_84d50)
        (*dat_84d50)();
      v3 = dcgettext(NULL,"Invalid time stamp",5);
      v8 = (char *)sub_4aa70(dat_84b28);
      error(0,*v6,"%s:%ld: %s",v8,2,v3);
      dat_84d58 = 2;
      dat_84b00 = 0x8000000000000000;
      dat_84b08 = 0xffffffffffffffff;
      v16 = 2;
    }
  }
label_1e5ad:
  while( true ) {
    v15 = &v11;
    v2 = (int4)getline(v15,&v10,dat_843a0);
    if (v2 <= 0) {
      free(v11);
      return;
    }
    v1 = *v11;
    v16 += 1;
    v3 = &v11[v1 == '+'];
    if (v11[(int8)v2 + -1] == '\n')
      v11[(int8)v2 + -1] = '\0';
    v6 = __errno_location();
    v12 = 0;
    v7 = 0;
    if (a0 == 1) break;
label_1e520:
    v4 = sub_240a0(v3,&v9,0,0xffffffffffffffff);
    if ((*v6) || (*v9 != ' ')) {
      if (dat_84d50)
        (*dat_84d50)();
      v3 = "Invalid device number";
      goto label_1e697;
    }
    v5 = sub_240a0(v9,&v9,0,0xffffffffffffffff);
    if ((*v6) || (*v9 != ' ')) {
      if (dat_84d50)
        (*dat_84d50)();
      v3 = "Invalid inode number";
      goto label_1e697;
    }
    v3 = &v9[1];
    sub_23c70(v3);
    sub_1dac0(v3,v12,v7,v4,v5,v1 == '+',0,NULL);
  }
  v14 = sub_242f0(v3,&v9,0);
  v3 = v9;
  v12 = SUB168(v14,0);
  if ((!sub_23ba0(v12,SUB168(v14,8))) || (*v3 != ' ')) {
    if (dat_84d50)
      (*dat_84d50)();
    v3 = "Invalid modification time";
    goto label_1e697;
  }
  *v6 = 0;
  v7 = strtoumax(v3,&v9,10);
  if (1000000000 <= v7) { // branch-flip
    if (!*v6)
      *v6 = 0x22;
  }
  else if (((!*v6) && (v9 != v3)) && (v3 = v9, *v9 == ' ')) goto label_1e520;
  if (dat_84d50)
    (*dat_84d50)();
  v3 = "Invalid modification time (nanoseconds)";
label_1e697:
  v3 = dcgettext(NULL,v3,5);
  v8 = (char *)sub_4aa70(dat_84b28);
  error(0,*v6,"%s:%ld: %s",v8,v16,v3);
  sub_163d0(); // no-return
}


// Function: sub_1e960 @ 0x1e960
int4 sub_1e960(FILE *a0,obstack *a1,int8 *a2)
{
  uint4 v1;
  char *v2;
  int8 v3;
  
  v3 = 0;
  v1 = getc_unlocked(a0);
  while (2 <= v1 + 1U) {
    v2 = *(char **)&a1->field_0x18;
    if (*(char **)&a1->field_0x20 == v2) {
      _obstack_newchunk(a1,1);
      v2 = *(char **)&a1->field_0x18;
    }
    v3 += 1;
    *(char **)&a1->field_0x18 = &v2[1];
    *v2 = (char)v1;
    v1 = getc_unlocked(a0);
  }
  v2 = *(char **)&a1->field_0x18;
  if (*(char **)&a1->field_0x20 == v2) {
    _obstack_newchunk(a1,1);
    v2 = *(char **)&a1->field_0x18;
  }
  *(char **)&a1->field_0x18 = &v2[1];
  *v2 = 0;
  *a2 = v3;
  return v1;
}


// Function: sub_1ea10 @ 0x1ea10
bool sub_1ea10(FILE *a0,char *a1,uint8 a2,uint8 a3,unsigned long *a4)
{
  bool v1; // al
  char v10 [32]; // stack - 0xb8
  char v11 [32];
  char v12 [24];
  char v13 [32];
  bool v14; // zf
  char *v15;
  uint4 v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  char *v7; // rax
  int4 v8;
  int8 v9; // rbx
  
  v2 = getc_unlocked(a0);
  v14 = v2 == 0x2d;
  v9 = 0;
  do {
    v8 = (int4)v9;
    v1 = !v8 && v14;
    if (!v8 && v14)
      v10[0] = (char)v2;
    else {
      if (9 < v2 - 0x30) {
        v10[v8] = '\0';
        if (0 <= (int4)v2) { // branch-flip
          if (v2) {
            if (dat_84d50)
              (*dat_84d50)();
            v3 = sub_42ff0(ftello(a0),v11);
            v4 = sub_4aa70(dat_84b28);
            v5 = dcgettext(NULL,"%s: byte %s: %s %s followed by invalid byte 0x%02x",5);
            v6 = (char *)(uint8)v2;
            v15 = v10;
            error(0,0,v5,v4,v3,a1,v15,v6); // return-dupe
            sub_163d0(); // no-return
          }
          *a4 = sub_240a0(v10,0,a2,a3);
          v8 = *__errno_location();
          if (v8) {
            if (v8 == 0x22) {
              if (dat_84d50)
                (*dat_84d50)();
              v6 = (char *)sub_43090(a3,v12);
              v15 = (char *)sub_42f50(a2,v13);
              v5 = (char *)sub_42ff0(ftello(a0),v11);
              v7 = (char *)sub_4aa70(dat_84b28);
              error(0,0x22,dcgettext(NULL,"%s: byte %s: (valid range %s..%s)\n\t%s %s",5),v7,v5,v15,v6,a1,v10);
              sub_163d0(); // no-return
            }
            if (dat_84d50)
              (*dat_84d50)();
            v6 = (char *)sub_42ff0(ftello(a0),v11);
            v15 = (char *)sub_4aa70(dat_84b28);
            error(0,v8,dcgettext(NULL,"%s: byte %s: %s %s",5),v15,v6,a1,v10);
            sub_163d0(); // no-return
          }
          v1 = 1;
        }
        else {
          if (ferror_unlocked(a0))
            sub_329b0(dat_84b28); // no-return
          if (v8) {
            if (dat_84d50)
              (*dat_84d50)();
            v6 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
            error(0,0,"%s: %s",(char *)sub_4aa70(dat_84b28),v6);
            sub_163d0(); // no-return
          }
        }
        return v1;
      }
      v10[v9] = (char)v2;
      v6 = v10;
      if (v9 == 0x14) {
        if (dat_84d50)
          (*dat_84d50)();
        v3 = sub_42ff0(ftello(a0),v11);
        v4 = sub_4aa70(dat_84b28);
        v5 = dcgettext(NULL,"%s: byte %s: %s %.*s... too long",5);
        v15 = (char *)0x15;
        error(0,0,v5,v4,v3,a1,v15,v6);
        sub_163d0();
      }
    }
    v9 += 1;
    v2 = getc_unlocked(a0);
  } while( true );
}


// Function: sub_1ed70 @ 0x1ed70
void sub_1ed70(FILE *a0,struct_0 *a1)
{
  char *v1; // rax
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  
  if ((sub_1ea10(a0,"sec",0x8000000000000000,0x7fffffffffffffff,&v2)) && (sub_1ea10(a0,"nsec",0,999999999,&v3))) {
    a1->field_0x0 = v2;
    a1->field_0x8 = v3;
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v1 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
  error(0,0,"%s: %s",(char *)sub_4aa70(dat_84b28),v1);
  sub_163d0(); // no-return
}


// Function: sub_1ee50 @ 0x1ee50
void sub_1ee50(void)
{
  int8 v1;
  char *v10; // rax
  char v11 [16]; // stack - 0xb8
  int8 v12; // stack - 0xd8
  uint8 v13; // stack - 0xd0
  unsigned long v14; // stack - 0xc8
  char v15 [24];
  bool v16; // zf
  unsigned long v17; // stack - 0xc0
  void *v18; // stack - 0xa8
  void *v19; // stack - 0xa0
  int8 v2;
  void *v20; // stack - 0x98
  uint8 v21; // stack - 0x88
  uint1 v22; // stack - 0x68
  void *v3;
  char v4; // al
  int4 v5; // eax
  void *v6;
  void *v7; // rax
  char *v8; // rax
  char *v9; // rax
  
  _obstack_begin((obstack *)v11,0,0,sub_4c5b0,dat_80fb8);
  sub_1ed70(dat_843a0,(struct_0 *)0x84b00);
  do {
    v4 = sub_1ea10(dat_843a0,"nfs",0,1,&v12);
    if (!v4)
      return;
    v16 = v12 != 0;
    sub_1ed70(dat_843a0,&v14);
    v4 = sub_1ea10(dat_843a0,"dev",0,0xffffffffffffffff,&v12);
    v1 = v12;
    if (((!v4) || (v4 = sub_1ea10(dat_843a0,"ino",0,0xffffffffffffffff,&v12), v2 = v12, !v4)) || (v5 = sub_1e960(dat_843a0,(obstack *)v11,&v13), v3 = v18, v5)) {
      if (dat_84d50)
        (*dat_84d50)();
      v8 = dcgettext(NULL,"Unexpected EOF in snapshot file",5);
      error(0,0,"%s: %s",(char *)sub_4aa70(dat_84b28),v8);
      sub_163d0(); // no-return
    }
    if (v19 == v18)
      v22 |= 2;
    v6 = (void *)((int8)v19 + v21 & ~v21);
    v18 = v20;
    if ((uint8)((int8)v6 - v11._8_8_) <= (uint8)((int8)v20 - v11._8_8_))
      v18 = v6;
    v19 = v18;
    do {
      v5 = sub_1e960(dat_843a0,(obstack *)v11,&v13);
      if (v5) break;
    } while (2 <= v13);
    v5 = getc_unlocked(dat_843a0);
    v6 = v18;
    if (v5) {
      if (dat_84d50)
        (*dat_84d50)();
      v8 = dcgettext(NULL,"Missing record terminator",5);
      v9 = (char *)sub_42ff0(ftello(dat_843a0),v15);
      v10 = (char *)sub_4aa70(dat_84b28);
      error(0,0,dcgettext(NULL,"%s: byte %s: %s",5),v10,v9,v8);
      sub_163d0(); // no-return
    }
    if (v19 == v18)
      v22 |= 2;
    v7 = (void *)((int8)v19 + v21 & ~v21);
    v18 = v20;
    if ((uint8)((int8)v7 - v11._8_8_) <= (uint8)((int8)v20 - v11._8_8_))
      v18 = v7;
    v19 = v18;
    sub_1dac0(v3,v14,v17,v1,v2,v16,0,v6);
    if ((v6 <= (uint8)v11._8_8_) || (v20 <= v6))
      _obstack_free((obstack *)v11,v6);
    else {
      v18 = v6;
      v19 = v6;
    }
  } while( true );
}


// Function: sub_1f1c0 @ 0x1f1c0
unsigned long sub_1f1c0(int8 a0,FILE *a1)
{
  int4 v1; // eax
  char v2 [24];
  void *v3; // stack - 0x50
  char *v4;
  void *v5;
  
  if (*(uint4 *)(a0 + 0x3c) & 4) {
    v4 = "1";
    if (!(*(uint4 *)(a0 + 0x3c) & 2))
      v4 = "0";
    fwrite_unlocked(v4,2,1,a1);
    v4 = (char *)sub_24080(*(uint8 *)(a0 + 8),0x8000000000000000,0x7fffffffffffffff,v2);
    fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
    v4 = (char *)sub_42f50(*(uint8 *)(a0 + 0x10),v2);
    fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
    v4 = (char *)sub_24080(*(uint8 *)(a0 + 0x18),0,0xffffffffffffffff,v2);
    fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
    v4 = (char *)sub_24080(*(uint8 *)(a0 + 0x20),0,0xffffffffffffffff,v2);
    fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
    v4 = *(char **)(a0 + 0x58);
    fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
    if (*(int8 **)(a0 + 0x28)) {
      v4 = (char *)sub_1d220(*(int8 **)(a0 + 0x28),0,&v3);
      if (v4) { // branch-flip
        do {
          fwrite_unlocked(v4,strlen(v4) + 1,1,a1);
          v5 = v3;
          v4 = (char *)sub_1d110(v3);
        } while (v4);
      }
      else {
        v5 = v3;
      }
      free(v5);
    }
    fwrite_unlocked((void *)0x61eec,2,1,a1);
  }
  v1 = ferror_unlocked(a1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_1f3c0 @ 0x1f3c0
void sub_1f3c0(int8 a0)
{
  uint8 v1;
  void *v2;
  uint8 v3;
  void *v4;
  void *v5; // stack - 0x40
  
  v3 = *(uint8 *)(a0 + 0x88);
  v5 = (void *)sub_4c5b0(v3);
  sub_d1f0(dat_84408);
  sub_ccb0(a0);
  v4 = v5;
  v2 = v5;
  while( true ) {
    if (!v3) {
      sub_ccd0();
      *(char *)(a0 + 0x189) = 1;
      *(void **)(a0 + 400) = v2;
      return;
    }
    sub_ccf0(v3);
    v2 = (void *)sub_e130();
    if (!v2) break;
    v1 = sub_d2b0(v2);
    if (v3 < v1)
      v1 = v3;
    memcpy(v4,v2,v1);
    v4 = (void *)((int8)v4 + v1);
    sub_d1f0((int8)v2 + (v1 - 1));
    v3 -= v1;
    v2 = v5;
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(1,0,dcgettext(NULL,"Unexpected EOF in archive",5));
  return;
}


// Function: sub_1f4c0 @ 0x1f4c0
int8 sub_1f4c0(char *a0)
{
  int8 v1; // rax
  int8 v2; // rbp
  
  if (*a0) {
    v2 = 0;
    do {
      v1 = strlen(a0) + 1;
      a0 = &a0[v1];
      v2 += v1;
    } while (*a0);
    return v2 + 1;
  }
  return 1;
}


// Function: sub_1f510 @ 0x1f510
void sub_1f510(void) // return-dupe
{
  int8 *v1;
  int8 *v2;
  
  if (dat_843b0)
    sub_41c30(dat_843b0);
  if (dat_843a8)
    sub_41c30(dat_843a8);
  v2 = dat_843c0;
  if (!dat_843c0) {
    dat_843b8 = 0;
    dat_843c0 = NULL;
    return;
  }
  do {
    v1 = (int8 *)*v2;
    sub_1d180(v2);
    v2 = v1;
  } while (v1);
  dat_843b8 = 0;
  dat_843c0 = NULL;
  return;
}


// Function: sub_1f570 @ 0x1f570
void sub_1f570(int8 a0,void *a1,uint8 a2,char *a3,uint8 a4)
{
  sub_23fc0((unsigned long *)(a0 + 0x58),a1,a2,a3,a4); // tail-call
  return;
}


// Function: sub_1f580 @ 0x1f580
void sub_1f580(struct_74 *a0) // return-dupe x2
{
  int4 v1;
  int8 v2; // rax
  stat v3;
  char v4 [16];
  
  v2 = sub_1d310(a0->field_0x0);
  if (!v2)
    return;
  v1 = a0->field_0x1a8;
  if (!fstat(v1,&v3)) {
    v4 = sub_4bd20(&v3);
    *(int8 *)(v2 + 8) = SUB168(v4,0);
    *(int8 *)(v2 + 0x10) = SUB168(v4,8);
    return;
  }
  sub_24ec0(*(char **)(v2 + 0x58));
  return;
}


// Function: sub_1f610 @ 0x1f610
int8 sub_1f610(unsigned long *a0)
{
  int8 v1;
  code *v10; // rdx
  int8 v11;
  void *v12; // stack - 0x200
  char v13 [88];
  stat v14 [2]; // stack - 0x1a0
  unsigned long v15;
  void *v16;
  char v17 [16];
  unsigned long *v18; // stack - 0x60
  int4 v19; // stack - 0x50
  unsigned long v2;
  int4 v3;
  char *v4; // rax
  char *v5;
  int8 v6;
  unsigned long *v7; // rax
  char *v8;
  char *v9; // rax
  
  v8 = (char *)*a0;
  v4 = (char *)sub_13fe0(a0);
  v6 = a0[0x33];
  v1 = a0[0xb];
  if (v4) { // branch-flip
    sub_16890(a0);
    v5 = (char *)sub_4c7a0(v8);
    sub_23e60(v5);
    v15 = 0;
    if (!v6)
      v15 = 0x20;
    v6 = sub_1dbe0(v5,a0,v15,&v11);
    free(v5);
    v7 = (unsigned long *)sub_25040(v8);
    if (*(int4 *)(v6 + 0x38)) { // branch-flip
      sub_1d600(v6,v4);
      v8 = (char *)sub_1d220(*(int8 **)(v6 + 0x28),1,&v12);
      if (v8) { // branch-flip
        do {
          v5 = &v8[1];
          v9 = (char *)sub_250d0(v7,v5);
          if ((*v8 != 'I') && (!sub_16b00(v9,a0))) {
            v3 = *(int4 *)&a0[0x35];
            sub_2f630(v13);
            if (0 <= v3) { // branch-flip
              if (fstatat(v3,v5,v14,dat_84800)) {
label_1f84a:
                v10 = sub_24ec0;
                goto label_1f7d3;
              }
              if ((v14[0]._24_4_ & 0xf000) == 0x4000) {
                v3 = sub_140d0(a0,v5,dat_84808);
                if (v3 <= -1) {
                  v10 = sub_24dd0;
                  goto label_1f7d3;
                }
                v19 = v3;
                if (!fstat(v3,v14)) {
                  if ((v14[0]._24_4_ & 0xf000) != 0x4000) goto label_1f8cb;
                  if (dat_84ad4) { // branch-flip
                    v15 = 0;
                    if (*(int4 *)(v6 + 0x38) == 2)
                      v15 = 0x12;
                  }
                  else {
                    v15 = 0x10;
                  }
                  *v8 = 'D';
                  v18 = a0;
                  sub_1dbe0(v9,v13,v15,v8);
                  sub_14200(v13);
                  goto label_1f7e0;
                }
                goto label_1f84a;
              }
label_1f8cb:
              if ((dat_84ad1) && (v14[0]._0_8_ != v1)) goto label_1f8e3;
              if (*v8 != 'Y') {
                v17 = sub_4bd20(v14);
                v2 = dat_84b08;
                v15 = dat_84b00;
                v3 = sub_4be80(SUB168(v17,0),SUB168(v17,8),dat_84b00,dat_84b08);
                if (0 <= v3) { // branch-flip
label_1f972:
                  *v8 = 'Y';
                }
                else {
                  if (dat_84b8c) {
                    v17 = sub_4bd10(v14);
                    if (0 <= (int4)sub_4be80(SUB168(v17,0),SUB168(v17,8),v15,v2)) goto label_1f972;
                  }
label_1f8e3:
                  *v8 = 'N';
                }
              }
            }
            else {
              v10 = sub_24dd0;
              *__errno_location() = -v3;
label_1f7d3:
              sub_24ef0(v9,0,v10);
              *v8 = 'N';
            }
label_1f7e0:
            sub_2f6f0(v13);
          }
          else {
            *v8 = 'N';
          }
          v16 = v12;
          v8 = (char *)sub_1d110(v12);
        } while (v8);
      }
      else {
        v16 = v12;
      }
      free(v16);
    }
    else if (*(int8 *)(v6 + 0x48))
      sub_1d800(v6);
  }
  else {
    sub_32ac0(v8);
    sub_16890(a0);
    v5 = (char *)sub_4c7a0(v8);
    sub_23e60(v5);
    v15 = 0;
    if (!v6)
      v15 = 0x20;
    v6 = sub_1dbe0(v5,a0,v15,&v11);
    free(v5);
    v7 = (unsigned long *)sub_25040(v8);
  }
  sub_250b0(v7);
  free(v4);
  return v6;
}


// Function: sub_1fa60 @ 0x1fa60
unsigned long sub_1fa60(int8 a0)
{
  if ((a0) && (*(unsigned long **)(a0 + 0x28)))
    return **(unsigned long **)(a0 + 0x28);
  return 0;
}


// Function: sub_1fa90 @ 0x1fa90
int8 sub_1fa90(int8 a0) // early-return
{
  int8 v1; // rax
  
  v1 = sub_1fa60(a0);
  if (v1)
    return v1;
  return 0x61eef;
}


// Function: sub_1fab0 @ 0x1fab0
void sub_1fab0(int8 a0) // return-dupe
{
  char *v1;
  uint1 v10; // stack - 0x38
  undefined8 v2;
  char *v3; // rax
  int8 *v4;
  char v5 [24]; // stack - 0x88
  uint8 v6;
  char *v7; // stack - 0x70
  char *v8; // stack - 0x68
  uint8 v9; // stack - 0x58
  
  if (!dat_843c0)
    return;
  v6 = 0;
  _obstack_begin((obstack *)v5,0,0,sub_4c5b0,dat_80fb8);
  v3 = (char *)sub_1fa60(a0);
  v4 = dat_843c0;
  if (v3) {
    v6 = sub_1f4c0(v3) - 1;
    if ((uint8)((int8)v8 - (int8)v7) < v6)
      _obstack_newchunk((obstack *)v5,v6);
    memcpy(v7,v3,v6);
    v7 = &v7[v6];
    v4 = dat_843c0;
  }
  for (; v4; v4 = (int8 *)*v4) {
    while (!(*(uint1 *)((int8)v4 + 0x3c) & 0x10)) {
      v4 = (int8 *)*v4;
      if (!v4) goto label_1fb79;
    }
    sub_1e320(v4,v5);
  }
label_1fb79:
  if ((a0) && (v6 != (int8)v7 - (int8)v5._16_8_)) {
    if (v7 == v8)
      _obstack_newchunk((obstack *)v5,1);
    v1 = &v7[1];
    *v7 = 0;
    v7 = v1;
    sub_1d1a0(*(struct_42 **)(a0 + 0x28));
    v2 = v5._16_8_;
    if (v7 == v5._16_8_)
      v10 |= 2;
    v5._16_8_ = v8;
    if ((uint8)((int8)(char *)((uint8)&v7[v9] & ~v9) - v5._8_8_) <= (uint8)((int8)v8 - v5._8_8_))
      v5._16_8_ = (char *)((uint8)&v7[v9] & ~v9);
    v7 = v5._16_8_;
    *(unsigned long *)(a0 + 0x28) = sub_1d7f0(v2);
  }
  _obstack_free((obstack *)v5,NULL);
  return;
}


// Function: sub_1fc70 @ 0x1fc70
void sub_1fc70(void)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  int8 *v3;
  char v4 [32];
  char v5 [24];
  int8 v6;
  uint8 v7;
  
  __printf_chk(1,"This tar\'s snapshot file field ranges are\n");
  __printf_chk(1,"   (%-15s => [ %s, %s ]):\n\n","field name","min","max");
  v6 = 0;
  v7 = 1;
  v3 = (int8 *)0x7f600;
  while( true ) {
    v1 = sub_24080(v7,v6,v7,v5);
    v2 = sub_24080(v3[1],v3[1],v3[2],v4);
    __printf_chk(1,"    %-15s => [ %s, %s ],\n",*v3,v2,v1);
    if (!v3[3]) break;
    v7 = v3[5];
    v6 = v3[4];
    v3 = &v3[3];
  }
  __printf_chk(1,"\n");
  return;
}


// Function: sub_1fd80 @ 0x1fd80
void sub_1fd80(void)
{
  char v1;
  int4 v2; // eax
  int8 v3; // rax
  uint8 v4; // rax
  char *v5;
  unsigned long v6; // stack - 0x28
  char *v7; // stack - 0x30
  char *v8;
  
  v7 = NULL;
  v6 = 0;
  v2 = open(dat_84b28,(-(uint4)(dat_84b24 == 0) & 0x200) + 0x42,0x1b6);
  if (v2 < 0) {
    sub_327c0(dat_84b28);
    return;
  }
  dat_843a0 = fdopen(v2,"r+");
  if (!dat_843a0) {
    sub_327c0(dat_84b28);
    close(v2);
    return;
  }
  sub_27b00();
  sub_27b80();
  v3 = getline(&v7,&v6,dat_843a0);
  v8 = v7;
  if (1 <= v3) { // branch-flip
    if (strncmp(v7,"GNU tar",7)) // branch-flip
      v4 = 0;
    else {
      v5 = &v8[8];
      if (v8[7] != '-') {
        if (dat_84d50)
          (*dat_84d50)();
        error(1,0,dcgettext(NULL,"Bad incremental file format",5));
        return;
      }
      v1 = v8[8];
      while (v1 != '-') {
        if (!v1) {
          if (dat_84d50)
            (*dat_84d50)();
          error(1,0,dcgettext(NULL,"Bad incremental file format",5));
          return;
        }
        v8 = &v5[1];
        v5 = &v5[1];
        v1 = *v8;
      }
      v4 = strtoumax(&v5[1],0,10);
      if (2 <= v4) {
        if (v4 != 2) {
          if (dat_84d50)
            (*dat_84d50)();
          error(1,0,dcgettext(NULL,"Unsupported incremental format version: %lu",5),v4);
          return;
        }
        sub_1ee50();
        goto label_1ff10;
      }
      v4 &= 0xffffffff;
      v8 = v7;
    }
    sub_1e470(v4,v8);
    v2 = ferror_unlocked(dat_843a0);
  }
  else {
label_1ff10:
    v2 = ferror_unlocked(dat_843a0);
  }
  if (v2)
    sub_32820(dat_84b28);
  free(v7);
  return;
}


// Function: sub_20010 @ 0x20010
void sub_20010(void) // return-dupe x2
{
  FILE *v1;
  int4 v2; // eax
  char *v3; // rax
  char v4 [24];
  
  v1 = dat_843a0;
  if (!dat_843a0)
    return;
  if (fseeko(dat_843a0,0,0))
    sub_32b00(dat_84b28);
  v2 = fileno(v1);
  if (sub_2aed0(v2))
    sub_32d70(dat_84b28);
  __fprintf_chk(v1,1,"%s-%s-%d\n","GNU tar","1.34",2);
  v3 = (char *)sub_42f50(dat_84a20,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  v3 = (char *)sub_43090(dat_84a28,v4);
  fwrite_unlocked(v3,strlen(v3) + 1,1,v1);
  if ((!ferror_unlocked(v1)) && (dat_843b0))
    sub_41a40(dat_843b0,sub_1f1c0,v1);
  if (ferror_unlocked(v1)) { // branch-flip
    sub_32e10(dat_84b28);
    v2 = fclose(v1);
  }
  else {
    v2 = fclose(v1);
  }
  if (!v2)
    return;
  sub_32680(dat_84b28);
  return;
}


// Function: sub_201b0 @ 0x201b0
char sub_201b0(int8 a0)
{
  if ((*(char *)(a0 + 0x188)) && (!*(int8 *)(a0 + 400))) {
    sub_1f3c0(a0);
    return *(char *)(a0 + 0x188);
  }
  return *(char *)(a0 + 0x188);
}


// Function: sub_201f0 @ 0x201f0
uint4 sub_201f0(char *a0)
{
  char *v1;
  char *v10;
  int8 v11 [3]; // stack - 0xd8
  char v12;
  char *v13; // stack - 0xf0
  uint4 v14; // stack - 0xc0
  int4 v2;
  uint4 v3; // eax
  uint8 v4; // rax
  char *v5;
  unsigned long v6; // rax
  char *v7;
  char *v8;
  unsigned long v9; // rax
  
  if (sub_201b0(0x84840)) {
    v13 = (char *)sub_253c0(a0,0);
    if ((v13) && (v3 = sub_1d860(dat_849d0), (char)v3)) {
      v5 = NULL;
      v12 = *dat_849d0;
      if (v12) {
        v7 = dat_849d0;
        do {
          if (v12 != 'X') { // branch-flip
            v4 = strlen(v7);
            if (v12 == 'R') {
              v1 = &v7[v4 + 1];
              v8 = &v7[1];
              v10 = &v1[1];
              if (v7[1])
                v8 = (char *)sub_33020(v8,0,(uint1)dat_84b92);
              if (v1[1])
                v10 = (char *)sub_33020(v10,0,(uint1)dat_84b92);
              if (*v8) { // branch-flip
                if (!*v10)
                  v10 = v5;
              }
              else {
                v8 = v5;
              }
              if (!sub_19ec0(v8,v10)) {
                free(v5); // return-dupe
                free(v13);
                return 0;
              }
              v7 = v1;
            }
          }
          else {
            v4 = strlen(&v7[1]);
            v5 = (char *)sub_4c610(v5,v4 + 0xc);
            memcpy(v5,&v7[1],v4);
            builtin_strncpy(&v5[v4],"/tar.XXXXXX",0xc);
            if (!mkdtemp(v5)) {
              if (dat_84d50)
                (*dat_84d50)();
              v6 = sub_4ac50(v5);
              v7 = dcgettext(NULL,"Cannot create temporary directory using template %s",5);
              error(0,*__errno_location(),v7,v6);
              dat_84d58 = 2;
              free(v5);
              free(v13);
              return 0;
            }
          }
          v7 = &v7[strlen(v7) + 1];
          v12 = *v7;
        } while (v12);
      }
      v7 = NULL;
      free(v5);
      v6 = sub_1d7f0(dat_849d0);
      if (*v13) {
        v5 = v13;
        do {
          free(v7);
          v7 = (char *)sub_27bc0(a0,v5);
          if (sub_246f0(v7,v11)) { // branch-flip
            if (*__errno_location() == 2) goto label_204cf;
            sub_24ec0(v7);
            if (dat_84d50)
              (*dat_84d50)();
            v9 = sub_4aa70(v7);
            v8 = "%s: Not purging directory: unable to stat";
label_204b7:
            error(0,0,dcgettext(NULL,v8,5),v9);
          }
          else {
            v8 = (char *)sub_1d450(v6,v5);
            if (v8) { // branch-flip
              if (*v8 != 'D') { // branch-flip
                if ((*v8 == 'Y') && ((v14 & 0xf000) == 0x4000)) goto label_20540;
              }
              else if ((v14 & 0xf000) != 0x4000) goto label_20540;
            }
            else {
label_20540:
              if ((dat_84ad1) && (v11[0] != dat_847f0)) {
                if (dat_84d50)
                  (*dat_84d50)();
                v9 = sub_4aa70(v7);
                v8 = "%s: directory is on a different device: not purging";
                goto label_204b7;
              }
              if ((!dat_84b40) || (sub_2ca20("delete",v7))) {
                if (dat_84a54) {
                  v10 = (char *)sub_4ac50(v7);
                  v8 = dat_84dd0;
                  __fprintf_chk(dat_83f28,1,dcgettext(NULL,"%s: Deleting %s\n",5),v8,v10);
                }
                if (!sub_25490(v7,1)) {
                  v2 = *__errno_location();
                  if (dat_84d50)
                    (*dat_84d50)();
                  v8 = (char *)sub_4aa70(v7);
                  error(0,v2,dcgettext(NULL,"%s: Cannot remove",5),v8);
                  dat_84d58 = 2;
                }
              }
            }
          }
label_204cf:
          v5 = &v5[strlen(v5) + 1];
        } while (*v5);
        v3 &= 0xff;
      }
      free(v7);
      sub_1d1a0(v6);
      free(v13);
      return v3;
    }
  }
  return 0;
}


// Function: sub_206e0 @ 0x206e0
void sub_206e0(char *a0)
{
  if (sub_201f0(a0))
    return;
  sub_22bd0(); // tail-call
  return;
}


// Function: sub_20710 @ 0x20710
void sub_20710(char *a0,int8 a1) // return-dupe
{
  char *v1;
  char v2;
  bool v3;
  uint4 v4; // eax
  
  v1 = &a0[a1];
  if (!a1)
    return;
  v3 = 0;
  do {
    while( true ) {
      v2 = *a0;
      a0 = &a0[1];
      if (v2) break;
      v3 = 0;
      fputc_unlocked(10,dat_83f28);
      if (a0 == v1)
        return;
    }
    v4 = (int4)v2 - 0x44;
    if ((0x16 <= (uint1)v4) || (!(0x314401UL >> ((uint8)v4 & 0x3f) & 1)))
      fputc_unlocked((int4)v2,dat_83f28);
    else {
      __fprintf_chk(dat_83f28,1,"%c");
      if (!v3)
        __fprintf_chk(dat_83f28,1," ");
      v3 = 1;
    }
  } while (a0 != v1);
  return;
}


// Function: sub_207f0 @ 0x207f0
char * sub_207f0(char *a0,int4 *a1)
{
  int4 v1;
  char *v2;
  char *v3; // rax
  
  v1 = *a1;
  if (v1 != 2) { // branch-flip
    if (v1 == 4) {
      v3 = a0;
      return v3;
    }
    v2 = a0;
    if (v1 == 1) {
      v3 = (char *)sub_33020(a0,0,(uint1)dat_84b92);
      v2 = v3;
    }
  }
  else {
    v3 = (char *)sub_33020(a0,1,(uint1)dat_84b92);
    v2 = v3;
  }
  if (!dat_84a90) {
    v3 = v2;
    return v3;
  }
  v3 = (uint8)sub_27c50(v2);
  if (v3 == (char *)0xffffffffffffffff)
    v3 = (uint8)strlen(v2);
  v3 = &v2[(int8)v3];
  return v3;
}


// Function: sub_20890 @ 0x20890
unsigned long sub_20890(uint1 *a0,uint8 a1,char *a2,int8 a3,uint1 *a4,unsigned int a5,unsigned int a6)
{
  uint1 v1;
  uint8 v10; // rax
  uint1 *v11;
  uint4 v12; // edx
  uint1 *v13;
  char v14 [8];
  char v15 [31];
  char v16 [999];
  char v17 [33];
  int8 v18;
  char *v19;
  char v2;
  char *v20; // rsp
  char *v21; // rsp
  uint1 *v22;
  uint1 *v23;
  uint1 *v24;
  void *v25; // r14
  uint1 *v26;
  bool v27; // zf
  bool v28;
  uint1 *v29; // stack - 0x490
  char *v3;
  uint1 *v30; // stack - 0x488
  uint1 *v31; // stack - 0x480
  unsigned int v32; // stack - 0x474
  uint1 *v33; // stack - 0x470
  uint1 v4;
  uint1 *v5;
  int8 v6;
  char *v7; // rax
  char *v8; // rax
  char *v9;
  
  v13 = &a0[a1];
  v21 = v14;
  v19 = v14;
  v30 = (uint1 *)CONCAT44(v30._4_4_,a5);
  v32 = a6;
  v31 = a4;
  if (v13 != &a0[*a0 == 0]) {
    v33 = &a0[*a0 == 0];
    v5 = __ctype_b_loc();
    v11 = v33;
    do {
      v4 = *v11;
      if (!(*(uint1 *)(*(int8 *)v5 + 1 + (uint8)v4 * 2) & 0x20)) {
        v33 = (uint1 *)-a3;
        if ((uint4)((int4)(char)v4 - 0x30U) <= 7) {
          v22 = &v11[1];
          v24 = (uint1 *)(int8)((int4)(char)v4 - 0x30U);
          if (v13 != v22) {
            v28 = 0;
            v26 = v22;
            goto label_20994;
          }
          v28 = 0;
          v26 = v13;
          goto label_20cc0;
        }
        if (!(char)v30) {
          if (!(v4 - 0x2b & 0xfd)) {
            if ((!(char)v32) && (!dat_84450)) {
              dat_84450 = '\x01';
              v29 = v11;
              v30 = v5;
              if (dat_84d50)
                (*dat_84d50)();
              error(0,0,dcgettext(NULL,"Archive contains obsolescent base-64 headers",5));
              v4 = *v29;
              v11 = v29;
              v5 = v30;
            }
            v11 = &v11[1];
            v28 = v4 == 0x2d;
            v24 = NULL;
            if (v13 == v11) {
              return sub_23b90(v24); // return-dupe
            }
            goto label_20baf;
          }
          if ((v4 == 0x80) || (v4 == 0xff)) {
            v11 = &v11[1];
            v24 = (uint1 *)(int8)(int4)((v4 & 0x3f) - (v4 & 0x40));
            goto label_20eb2;
          }
        }
        if (v4) goto label_20a91;
        v24 = NULL;
        return sub_23b90(v24);
      }
      v11 = &v11[1];
    } while (v13 != v11);
  }
  if (!a2)
    return 0xffffffffffffffff;
  if ((char)v32 == '\x01')
    return 0xffffffffffffffff;
  if (dat_84d50)
    (*dat_84d50)();
  v9 = "Blanks in header where numeric %s value expected";
  error(0,0,dcgettext(NULL,v9,5),a2); // return-dupe
  dat_84d58 = 2;
  return 0xffffffffffffffff;
  while( true ) {
    v27 = (uint1 *)((uint8)v24 & 0x1fffffffffffffff) != v24;
    v24 = (uint1 *)((int8)(int4)v12 + (int8)v24 * 8);
    v26 = &v26[1];
    v28 = (bool)(v28 | v27);
    if (v13 == v26) break;
label_20994:
    v1 = *v26;
    v10 = (uint8)(uint4)(int4)(char)v1;
    v12 = (int4)(char)v1 - 0x30;
    if (8 <= v12) {
      if ((v31 < v24) || (v28)) goto label_20cce;
      if ((v13 == v26) || (!v1)) {
        return sub_23b90(v24);
      }
      goto label_209ca;
    }
  }
label_20cc0:
  if ((v24 <= v31) && (!v28)) {
    return sub_23b90(v24);
  }
label_20cce:
  if ('2' <= (char)v4) { // branch-flip
    if (a2) {
      v23 = (uint1 *)(int8)(7 - (int4)(char)(v4 - 0x30 | 4));
      if (v13 != v22) { // branch-flip
        v4 = 0;
        do {
          v1 = *v22;
          if (8 <= (uint4)((int4)(char)v1 - 0x30U)) {
            v26 = v22;
            goto label_20d4e;
          }
          v22 = &v22[1];
          v4 |= (uint1 *)((uint8)v23 & 0x1fffffffffffffff) != v23;
          v23 = (uint1 *)((int8)(7 - ((int4)(char)v1 - 0x30U)) + (int8)v23 * 8);
        } while (v13 != v22);
        v26 = v13;
      }
      else {
        v4 = 0;
        v26 = v13;
      }
label_20d4e:
      v24 = &v23[1];
      if ((bool)(v24 == NULL | v4)) goto label_210d1;
      v28 = 0;
      if (v24 <= v33) {
        if (!(char)v32) {
          v30 = v11;
          v31 = v23;
          v33 = v5;
          if (dat_84d50)
            (*dat_84d50)();
          v9 = dcgettext(NULL,"Archive octal value %.*s is out of %s range; assuming two\'s complement",5);
          error(0,0,v9,(int4)v26 - (int4)v30,(char *)v30,a2);
          v23 = v31;
          v5 = v33;
        }
        if ((v13 == v26) || (!*v26)) {
          v24 = (uint1 *)~(uint8)v23;
          return sub_23b90(v24);
        }
        if (!(*(uint1 *)(*(int8 *)v5 + 1 + (uint8)*v26 * 2) & 0x20)) goto label_20a9a;
        v24 = (uint1 *)-(int8)v24; // return-dupe
        return sub_23b90(v24);
      }
      goto label_20e3b;
    }
    if (v28)
      return 0xffffffffffffffff;
  }
  else {
label_20e3b:
    if (v28) {
label_210d1:
      if (!a2)
        return 0xffffffffffffffff;
      if ((char)v32 == '\x01')
        return 0xffffffffffffffff;
      v33 = v11;
      if (dat_84d50)
        (*dat_84d50)();
      v9 = dcgettext(NULL,"Archive octal value %.*s is out of %s range",5);
      error(0,0,v9,(int4)v26 - (int4)v33,(char *)v33,a2);
      dat_84d58 = 2;
      return 0xffffffffffffffff;
    }
  }
  if ((v13 != v26) && (v10 = (uint8)*v26, *v26)) {
label_209ca:
    if (!(*(uint1 *)(*(int8 *)v5 + 1 + (v10 & 0xff) * 2) & 0x20)) {
label_20a91:
      if (!a2)
        return 0xffffffffffffffff;
label_20a9a:
      if (!dat_84448) {
        dat_84448 = sub_4a530(NULL);
        sub_4a590(dat_84448,8);
      }
      for (; (a0 != v13 && (!v13[-1])); v13 = &v13[-1]) {
      }
      sub_4a650(&v18,1000,a0,(int8)v13 - (int8)a0,dat_84448);
      if ((char)v32)
        return 0xffffffffffffffff;
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Archive contains %.*s where numeric %s value expected",5),1000,&v18,a2);
      dat_84d58 = 2;
      return 0xffffffffffffffff;
    }
  }
  goto label_209e0;
  while ((uint1 *)((uint8)v24 & 0xffffffffffffff | (uint8)-(v4 & 0x40) << 0x32) == v24) {
label_20eb2:
    v1 = *v11;
    v11 = &v11[1];
    v24 = (uint1 *)((int8)v24 * 0x100 + (uint8)v1);
    if (v13 == v11) {
      v28 = (v4 & 0x40) != 0;
      if (v4 & 0x40)
        v24 = (uint1 *)-(int8)v24;
      goto label_20bd5;
    }
  }
  if (!a2)
    return 0xffffffffffffffff;
  if ((char)v32 == '\x01')
    return 0xffffffffffffffff;
  if (dat_84d50)
    (*dat_84d50)();
  v9 = "Archive base-256 value is out of %s range";
  error(0,0,dcgettext(NULL,v9,5),a2);
  dat_84d58 = 2;
  return 0xffffffffffffffff;
  while( true ) {
    if ((uint1 *)((uint8)v24 & 0x3ffffffffffffff) != v24) {
      v3 = v14;
      while (v21 != &v14[-(a1 + 0x18 & 0xfffffffffffff000)]) {
        v19 = &v3[-0x1000];
        v20 = &v3[-0x1000];
        *(unsigned long *)&v3[-8] = *(unsigned long *)&v3[-8];
        v21 = &v3[-0x1000];
        v3 = v20;
      }
      v10 = (uint8)((uint4)(a1 + 0x18) & 0xff0);
      v6 = -v10;
      if (v10)
        *(unsigned long *)&v19[-8] = *(unsigned long *)&v19[-8];
      v25 = (void *)((uint8)&v19[v6 + 0xf] & 0xfffffffffffffff0);
      *(unsigned long *)&v19[v6 + -8] = 0x21016;
      memcpy(v25,a0,a1);
      *(char *)((int8)v25 + a1) = 0;
      if ((char)v32 == '\x01')
        return 0xffffffffffffffff;
      if (!a2)
        return 0xffffffffffffffff;
      if (dat_84d50) {
        *(unsigned long *)&v19[v6 + -8] = 0x2103f;
        (*dat_84d50)();
      }
      *(unsigned long *)&v19[v6 + -8] = 0x21047;
      v9 = (char *)sub_4ac50(v25);
      *(unsigned long *)&v19[v6 + -8] = 0x2105d;
      v8 = dcgettext(NULL,"Archive signed base-64 string %s is out of %s range",5);
      *(unsigned long *)&v19[v6 + -8] = 0x21071;
      error(0,0,v8,v9,a2);
      dat_84d58 = 2;
      return 0xffffffffffffffff;
    }
    v11 = &v11[1];
    v24 = (uint1 *)((int8)v24 << 6 | (int8)v2);
    if (v13 == v11) break;
label_20baf:
    v4 = *v11;
    v2 = *(char *)((uint8)v4 + 0x84460);
    if ('@' <= v2) {
      if (((v11 != v13) && (v4)) && (!(*(uint1 *)(*(int8 *)v5 + 1 + (uint8)v4 * 2) & 0x20))) goto label_20a91;
      break;
    }
  }
label_20bd5:
  if (v28) {
    if (v33 < v24) {
      if ((char)v32 == '\x01')
        return 0xffffffffffffffff;
      if (!a2)
        return 0xffffffffffffffff;
      v8 = (char *)sub_43090(v33,v15);
      v6 = sub_43090(v24,v16);
      *(char *)(v6 + -1) = 0x2d;
      v9 = (char *)(v6 + -1);
      goto label_20c2e;
    }
    v24 = (uint1 *)-(int8)v24;
    return sub_23b90(v24);
  }
label_209e0:
  if (v31 < v24) {
    if (!a2)
      return 0xffffffffffffffff;
    if ((char)v32 == '\x01')
      return 0xffffffffffffffff;
    v8 = (char *)sub_43090(v33,v15);
    v9 = (char *)sub_43090(v24,v16);
label_20c2e:
    if (v33) {
      v8[-1] = '-';
      v8 = &v8[-1];
    }
    if (dat_84d50)
      (*dat_84d50)();
    v7 = (char *)sub_43090(v31,v17);
    error(0,0,dcgettext(NULL,"Archive value %s is out of %s range %s..%s",5),v9,a2,v8,v7);
    dat_84d58 = 2;
    return 0xffffffffffffffff;
  }
  return sub_23b90(v24);
}


// Function: sub_21160 @ 0x21160
void sub_21160(void)
{
  int8 v1;
  int8 v2; // rdx
  
  v2 = 0x41;
  dat_84460 = s_62520._64_8_;
  dat_84468 = s_62520._72_8_;
  dat_84470 = s_62520._64_8_;
  dat_84478 = s_62520._72_8_;
  dat_84480 = s_62520._64_8_;
  dat_84488 = s_62520._72_8_;
  dat_84490 = s_62520._64_8_;
  dat_84498 = s_62520._72_8_;
  dat_844a0 = s_62520._64_8_;
  dat_844a8 = s_62520._72_8_;
  dat_844b0 = s_62520._64_8_;
  dat_844b8 = s_62520._72_8_;
  dat_844c0 = s_62520._64_8_;
  dat_844c8 = s_62520._72_8_;
  dat_844d0 = s_62520._64_8_;
  dat_844d8 = s_62520._72_8_;
  dat_844e0 = s_62520._64_8_;
  dat_844e8 = s_62520._72_8_;
  dat_844f0 = s_62520._64_8_;
  dat_844f8 = s_62520._72_8_;
  dat_84500 = s_62520._64_8_;
  dat_84508 = s_62520._72_8_;
  dat_84510 = s_62520._64_8_;
  dat_84518 = s_62520._72_8_;
  dat_84520 = s_62520._64_8_;
  dat_84528 = s_62520._72_8_;
  dat_84530 = s_62520._64_8_;
  dat_84538 = s_62520._72_8_;
  dat_84540 = s_62520._64_8_;
  dat_84548 = s_62520._72_8_;
  dat_84550 = s_62520._64_8_;
  dat_84558 = s_62520._72_8_;
  v1 = 0;
  while( true ) {
    *(char *)(v2 + 0x84460) = (char)v1;
    if (v1 + 1 == 0x40) break;
    v2 = (int8)"BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"[v1];
    v1 += 1;
  }
  return;
}


// Function: sub_21440 @ 0x21440
void sub_21440(int4 a0,int8 a1) // return-dupe
{
  if (a0 == 0x56)
    return;
  sub_21420((uint8 *)(a1 + 8),1);
  if (a0 != 0x31) { // branch-flip
    if (a0 == 0x32) {
      sub_21420((uint8 *)(a1 + 0x18),4);
      if (!dat_84ad0)
        return;
      sub_21360(); // return-dupe, tail-call
      return;
    }
  }
  else {
    sub_21420((uint8 *)(a1 + 0x18),2);
  }
  if (!dat_84ad0)
    return;
  sub_21360();
  return;
}


// Function: sub_214d0 @ 0x214d0
undefined16 sub_214d0(uint1 *a0,uint1 a1)
{
  uint1 v1;
  int4 v2; // eax
  char v3 [16];
  char v4 [16];
  char v5 [16];
  int4 v6; // ecx
  uint1 *v7;
  int4 v8; // esi
  unsigned long v9; // r8
  
  v8 = 0;
  v6 = 0;
  v7 = a0;
  do {
    v4._8_8_ = &v7[1];
    v6 += (uint4)*v7;
    v8 += (char)*v7;
    v7 = v4._8_8_;
  } while (&a0[0x200] != v4._8_8_);
  if (!v6) {
    v4._0_8_ = 3;
    return v4._0_16_;
  }
  v7 = &a0[0x9b];
  do {
    v1 = *v7;
    v7 = &v7[-1];
    v6 -= (uint4)v1;
    v8 -= (char)v1;
  } while (&a0[0x93] != v7);
  v5._9_7_ = 0;
  v5[8] = a1;
  v2 = sub_20890(&a0[0x94],8,NULL,0,(uint1 *)0x7fffffff,1);
  if (0 <= v2) {
    if ((v6 + 0x100 == v2) || (v9 = 5, v8 + 0x100 == v2))
      v9 = 1;
    v3[8] = a1;
    v3._0_8_ = v9;
    v3._9_7_ = 0;
    return v3._0_16_;
  }
  v5._0_8_ = 5;
  return v5._0_16_;
}


// Function: sub_215c0 @ 0x215c0
void sub_215c0(int8 a0,int8 a1,int4 *a2,int4 a3) // return-dupe x2
{
  char v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  char v4; // stack - 0x41
  char *v5;
  int4 v6; // r12d
  
  v2 = sub_21210((uint1 *)(a0 + 100),&v4);
  if (strcmp((char *)(a0 + 0x101),"ustar")) { // branch-flip
    v6 = 1;
    if (!strcmp((char *)(a0 + 0x101),"ustar  "))
      v6 = (-(uint4)(v4 == '\0') & 4) + 2;
  }
  else if ((((*(char *)(a0 + 0x1db)) || (8 <= (uint1)(*(char *)(a0 + 0x1dc) - 0x30U))) || (*(char *)(a0 + 0x1e7) != ' ')) || ((8 <= (uint1)(*(char *)(a0 + 0x1e8) - 0x30U) || (v6 = 5, *(char *)(a0 + 499) != ' '))))
    v6 = 4 - (uint4)(*(int8 *)(a1 + 0x170) == 0);
  *a2 = v6;
  *(unsigned int *)(a1 + 0x70) = v2;
  v3 = sub_21320((uint1 *)(a0 + 0x88));
  v5 = NULL;
  v1 = *(char *)(a0 + 0x109);
  *(unsigned long *)(a1 + 0x100) = 0;
  *(unsigned long *)(a1 + 0xf8) = v3;
  if (v1)
    v5 = (char *)(a0 + 0x109);
  sub_23c20((unsigned long *)(a1 + 0x20),v5,0x20);
  v5 = NULL;
  if (*(char *)(a0 + 0x129))
    v5 = (char *)(a0 + 0x129);
  sub_23c20((unsigned long *)(a1 + 0x28),v5,0x20);
  sub_1c5b0(a1);
  if (v6 != 2) { // branch-flip
    if (v6 != 5) { // branch-flip
      *(unsigned long *)(a1 + 0x108) = dat_84a20;
      *(unsigned long *)(a1 + 0x110) = dat_84a28;
      *(unsigned long *)(a1 + 0xe8) = dat_84a20;
      *(unsigned long *)(a1 + 0xf0) = dat_84a28;
      if (v6 == 1) {
        *(unsigned int *)(a1 + 0x74) = sub_21260((uint1 *)(a0 + 0x6c));
        *(unsigned int *)(a1 + 0x78) = sub_21290((uint1 *)(a0 + 0x74));
        v3 = 0;
        goto label_2173b;
      }
    }
    else {
      *(unsigned long *)(a1 + 0xe8) = sub_21320((uint1 *)(a0 + 0x1dc));
      v3 = sub_21320((uint1 *)(a0 + 0x1e8));
      *(unsigned long *)(a1 + 0x110) = 0;
      *(unsigned long *)(a1 + 0x108) = v3;
      *(unsigned long *)(a1 + 0xf0) = 0;
    }
  }
  else if (dat_84b50) { // branch-flip
    *(unsigned long *)(a1 + 0xe8) = sub_21320((uint1 *)(a0 + 0x159));
    v3 = sub_21320((uint1 *)(a0 + 0x165));
    *(unsigned long *)(a1 + 0x110) = 0;
    *(unsigned long *)(a1 + 0x108) = v3;
    *(unsigned long *)(a1 + 0xf0) = 0;
  }
  else {
    *(unsigned long *)(a1 + 0x108) = dat_84a20;
    *(unsigned long *)(a1 + 0x110) = dat_84a28;
    *(unsigned long *)(a1 + 0xe8) = dat_84a20;
    *(unsigned long *)(a1 + 0xf0) = dat_84a28;
  }
  if (a3) {
    if (((dat_84ad2) || (!*(char *)(a0 + 0x109))) || (!sub_269f0((char *)(a0 + 0x109),(unsigned int *)(a1 + 0x74))))
      *(unsigned int *)(a1 + 0x74) = sub_21260((uint1 *)(a0 + 0x6c));
    if (((dat_84ad2) || (!*(char *)(a0 + 0x129))) || (!sub_26aa0((char *)(a0 + 0x129),(unsigned int *)(a1 + 0x78))))
      *(unsigned int *)(a1 + 0x78) = sub_21290((uint1 *)(a0 + 0x74));
  }
  v3 = 0;
  if ((uint4)((int4)*(char *)(a0 + 0x9c) - 0x33U) < 2) {
    v2 = sub_212c0((uint1 *)(a0 + 0x151));
    v3 = gnu_dev_makedev(sub_212f0((uint1 *)(a0 + 0x149)),v2);
  }
label_2173b:
  *(unsigned long *)(a1 + 0x80) = v3;
  sub_1c8c0(a1);
  if (sub_29e10(a1)) {
    sub_29e70(a1);
    *(char *)(a1 + 0x120) = 1;
    return;
  }
  *(char *)(a1 + 0x120) = 0;
  if ((((dat_84400 & 0xfffffffb) != 2) || (*(char *)(dat_84408 + 0x9c) != 'D')) && (!*(int8 *)(a1 + 400)))
    return;
  *(char *)(a1 + 0x188) = 1;
  return;
}


// Function: sub_219b0 @ 0x219b0
unsigned long sub_219b0(uint1 *a0,uint8 a1)
{
  return sub_20890(a0,a1,"off_t",0,(uint1 *)0x7fffffffffffffff,0,0);
}


// Function: sub_219e0 @ 0x219e0
int4 sub_219e0(unsigned long *a0,unsigned long *a1,int4 a2)
{
  uint1 *v1;
  void *v10; // rax
  char *v11;
  uint8 v12;
  int8 v13; // rax
  uint8 v14;
  char v15 [16]; // stack - 0x168
  unsigned long v16; // stack - 0x148
  unsigned long *v17;
  unsigned long *v18; // stack - 0x188
  unsigned long *v19; // stack - 0x180
  unsigned long *v2;
  uint8 v20; // stack - 0x178
  uint8 v21; // stack - 0x170
  char v22 [16]; // stack - 0x158
  unsigned long v23; // stack - 0x140
  unsigned long v24; // stack - 0x138
  unsigned long v25; // stack - 0x130
  unsigned long v26; // stack - 0x128
  unsigned long v27; // stack - 0x120
  unsigned long v28; // stack - 0x118
  unsigned long v29; // stack - 0x110
  unsigned long *v3;
  unsigned long v30; // stack - 0x108
  unsigned long v31; // stack - 0x100
  unsigned long v32; // stack - 0xf8
  unsigned long v33; // stack - 0xf0
  uint8 v34; // stack - 0xe8
  unsigned long v35; // stack - 0xe0
  unsigned long v36; // stack - 0xd8
  unsigned long v37; // stack - 0xd0
  unsigned long v38; // stack - 0xc8
  unsigned long v39; // stack - 0xc0
  char v4;
  unsigned long v40; // stack - 0xb8
  unsigned short v41; // stack - 0xb0
  char v42; // stack - 0xae
  char v43; // stack - 0xad
  unsigned int v5;
  unsigned long v6;
  int4 v7; // eax
  unsigned long *v8;
  unsigned long *v9;
  
  v20 = 0;
  v21 = 0;
  v18 = NULL;
  v19 = NULL;
label_21a40:
  v8 = (unsigned long *)sub_e130();
  *a0 = v8;
label_21a4e:
  if (!v8) {
    v7 = 4;
    free(v19); // return-dupe
    free(v18);
    return v7;
  }
  v7 = sub_214d0(v8,0);
  if (v7 != 1) {
    free(v19);
    free(v18);
    return v7;
  }
  if (*(char *)((int8)v8 + 0x9c) == '1') {
    a1[0x11] = 0;
label_21a83:
    free(dat_843f8);
    v9 = &v19[0x40];
    if (!v19) {
      if (*(char *)((int8)v8 + 0x159)) { // branch-flip
        v11 = (char *)&v16;
        if (!strcmp((char *)((int8)v8 + 0x101),"ustar")) {
          v16 = *(unsigned long *)((int8)v8 + 0x159);
          v23 = *(unsigned long *)((int8)v8 + 0x161);
          v40 = *(unsigned long *)((int8)v8 + 0x1e9);
          v43 = 0;
          v24 = *(unsigned long *)((int8)v8 + 0x169);
          v25 = *(unsigned long *)((int8)v8 + 0x171);
          v26 = *(unsigned long *)((int8)v8 + 0x179);
          v27 = *(unsigned long *)((int8)v8 + 0x181);
          v28 = *(unsigned long *)((int8)v8 + 0x189);
          v29 = *(unsigned long *)((int8)v8 + 0x191);
          v30 = *(unsigned long *)((int8)v8 + 0x199);
          v31 = *(unsigned long *)((int8)v8 + 0x1a1);
          v32 = *(unsigned long *)((int8)v8 + 0x1a9);
          v33 = *(unsigned long *)((int8)v8 + 0x1b1);
          v42 = *(char *)((int8)v8 + 499);
          v34 = *(uint8 *)((int8)v8 + 0x1b9);
          v35 = *(unsigned long *)((int8)v8 + 0x1c1);
          v41 = *(unsigned short *)((int8)v8 + 0x1f1);
          v36 = *(unsigned long *)((int8)v8 + 0x1c9);
          v37 = *(unsigned long *)((int8)v8 + 0x1d1);
          v38 = *(unsigned long *)((int8)v8 + 0x1d9);
          v39 = *(unsigned long *)((int8)v8 + 0x1e1);
          v12 = strlen(v11);
          v11[v12] = '/';
          v11 = (char *)((int8)&v16 + v12 + 1);
        }
      }
      else {
        v11 = (char *)&v16;
      }
      v9 = &v16;
      v21 = 0;
      v6 = v8[1];
      *(unsigned long *)v11 = *v8;
      *(unsigned long *)&v11[8] = v6;
      v6 = v8[3];
      *(unsigned long *)&v11[0x10] = v8[2];
      *(unsigned long *)&v11[0x18] = v6;
      v6 = v8[5];
      *(unsigned long *)&v11[0x20] = v8[4];
      *(unsigned long *)&v11[0x28] = v6;
      v6 = v8[7];
      *(unsigned long *)&v11[0x30] = v8[6];
      *(unsigned long *)&v11[0x38] = v6;
      v6 = v8[9];
      *(unsigned long *)&v11[0x40] = v8[8];
      *(unsigned long *)&v11[0x48] = v6;
      v6 = v8[0xb];
      *(unsigned long *)&v11[0x50] = v8[10];
      *(unsigned long *)&v11[0x58] = v6;
      v5 = *(unsigned int *)&v8[0xc];
      v11[100] = '\0';
      *(unsigned int *)&v11[0x60] = v5;
    }
    dat_843f8 = v19;
    dat_843e8 = v21;
    sub_23be0(a1,v9);
    sub_23be0(&a1[1],v9);
    v11 = (char *)a1[1];
    *(char *)&a1[2] = sub_3e630(v11);
    free(dat_843f0);
    if (v18) // branch-flip
      v9 = &v18[0x40];
    else {
      v16 = *(unsigned long *)((int8)v8 + 0x9d);
      v23 = *(unsigned long *)((int8)v8 + 0xa5);
      v9 = &v16;
      v24 = *(unsigned long *)((int8)v8 + 0xad);
      v25 = *(unsigned long *)((int8)v8 + 0xb5);
      v26 = *(unsigned long *)((int8)v8 + 0xbd);
      v27 = *(unsigned long *)((int8)v8 + 0xc5);
      v20 = 0;
      v28 = *(unsigned long *)((int8)v8 + 0xcd);
      v29 = *(unsigned long *)((int8)v8 + 0xd5);
      v34 = CONCAT44((int4)(v34 >> 0x20),*(unsigned int *)((int8)v8 + 0xfd)) & 0xffffff00ffffffff;
      v30 = *(unsigned long *)((int8)v8 + 0xdd);
      v31 = *(unsigned long *)((int8)v8 + 0xe5);
      v32 = *(unsigned long *)((int8)v8 + 0xed);
      v33 = *(unsigned long *)((int8)v8 + 0xf5);
    }
    dat_843f0 = v18;
    dat_843e0 = v20;
    sub_23be0(&a1[3],v9);
    v18 = NULL;
    v19 = NULL;
    free(v19);
    free(v18);
    return v7;
  }
  v1 = (uint1 *)((int8)v8 + 0x7c);
  v12 = sub_219b0(v1,0xc);
  a1[0x11] = v12;
  if ((int8)v12 < 0) {
    v7 = 5;
    free(v19);
    free(v18);
    return v7;
  }
  v4 = *(char *)((int8)v8 + 0x9c);
  if ((2 <= (uint1)(v4 + 0xb5U)) && (v4 != 'x')) {
    if ((v4 != 'g') && (v4 != 'X')) goto label_21a83;
    if (a2 == 1) {
      v7 = 2; // return-dupe
      free(v19);
      free(v18);
      return v7;
    }
    if (v4 != 'X') goto label_21c2e;
  }
  else {
    if (a2 == 1) {
      v7 = 2;
      free(v19);
      free(v18);
      return v7;
    }
    if ((uint1)(v4 + 0xb5U) < 2) goto label_21cd0;
  }
  sub_1cac0(&a1[0x2d],v8,sub_219b0(v1,0xc));
  v8 = (unsigned long *)sub_e130();
  *a0 = v8;
  goto label_21a4e;
label_21cd0:
  v14 = v12 + 0x200;
  if ((v12 & 0x1ff) && (v14 = (v12 + 0x400) - (uint8)((uint4)v12 & 0x1ff), v14 < v12))
    sub_16420(); // no-return
  v9 = (unsigned long *)sub_4c5b0(v14 + 1);
  if (*(char *)((int8)v8 + 0x9c) != 'L') { // branch-flip
    free(v18);
    v18 = v9;
    v20 = v14 >> 9;
  }
  else {
    free(v19);
    v19 = v9;
    v21 = v14 >> 9;
  }
  sub_d1f0(v8);
  v17 = &v9[0x40];
  v13 = (int8)v9 - (int8)(unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
  *v9 = *v8;
  v9[0x3f] = v8[0x3f];
  v12 = (uint8)((int4)v13 + 0x200U >> 3);
  v8 = (unsigned long *)((int8)v8 - v13);
  v9 = (unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
  while (v12) {
    v3 = &v9[1];
    v2 = &v8[1];
    *v9 = *v8;
    v12 -= 1;
    v8 = v2;
    v9 = v3;
  }
  for (v14 = v14 - 0x200; v14; v14 = v14 - v12) {
    v10 = (void *)sub_e130();
    if (!v10) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      dat_84d58 = 2;
      break;
    }
    v12 = sub_d2b0(v10);
    if (v14 <= v12)
      v12 = v14;
    memcpy(v17,v10,v12);
    v17 = (unsigned long *)((int8)v17 + v12);
    sub_d1f0((int8)v10 + (v12 - 1));
  }
  *(char *)v17 = 0;
  goto label_21a40;
label_21c2e:
  v9 = dat_84560;
  if (!dat_84560) {
    v9 = (unsigned long *)sub_4c5b0(0x200);
    dat_84560 = v9;
  }
  *v9 = *v8;
  v9[0x3f] = v8[0x3f];
  v13 = (int8)v9 - (int8)(unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
  v12 = (uint8)((int4)v13 + 0x200U >> 3);
  v17 = (unsigned long *)((int8)v8 - v13);
  v9 = (unsigned long *)((uint8)&v9[1] & 0xfffffffffffffff8);
  while (v12) {
    v3 = &v9[1];
    v2 = &v17[1];
    *v9 = *v17;
    v12 -= 1;
    v17 = v2;
    v9 = v3;
  }
  v15._0_16_ = 0;
  v22._0_16_ = 0;
  sub_1cac0(v15,v8,sub_219b0(v1,0xc));
  sub_1c9a0(v15);
  sub_1cca0(v15);
  if (a2 == 2) {
    v7 = 2;
    free(v19);
    free(v18);
    return v7;
  }
  goto label_21a40;
}


// Function: sub_22050 @ 0x22050
unsigned long sub_22050(uint1 *a0,uint8 a1)
{
  return sub_20890(a0,a1,"uintmax_t",0,(uint1 *)0xffffffffffffffff,0,0);
}


// Function: sub_22080 @ 0x22080
int8 sub_22080(int8 a0,int4 a1,bool a2) // early-return x2
{
  tm *v1;
  uint8 v2;
  uint8 v3;
  uint8 v4;
  int8 v5; // rax
  int8 v6; // stack - 0x28
  
  v6 = a0;
  if ((a1) && (a0 <= -1)) {
    v6 = a0 + 1;
    a1 = 1000000000 - a1;
  }
  if (dat_84b91) // branch-flip
    v1 = gmtime(&v6);
  else {
    v1 = localtime(&v6);
  }
  if (v1) {
    if (!a2) {
      strftime((char *)0x84420,0x25,"%Y-%m-%d %H:%M",v1);
      return 0x84420;
    }
    strftime((char *)0x84420,0x25,"%Y-%m-%d %H:%M:%S",v1);
    sub_24170(a1,(char *)(strlen((char *)0x84420) + 0x84420));
    return 0x84420;
  }
  if (0 <= a0) // branch-flip
    v2 = sub_43090(v6,0x84426);
  else {
    v5 = sub_43090(-v6,0x84426);
    v2 = v5 - 1;
    *(char *)(v5 + -1) = 0x2d;
  }
  v3 = 0x84441;
  if (!a2)
    v3 = 0x84434;
  for (v4 = v2; v3 < v4; v4 = v4 - 1) {
    *(char *)(v4 - 1) = 0x20;
  }
  v5 = v3 - v2;
  if (v2 < v3)
    v5 = 0;
  if (a2)
    sub_24170(a1,(char *)0x8443a);
  return v2 + v5;
}


// Function: sub_22220 @ 0x22220
void sub_22220(struct_9 *a0,int8 a1,int8 a2)
{
  FILE *v1;
  char v10 [2];
  char v11 [32];
  char v12 [32];
  char v13 [32];
  char v14; // stack - 0xe4
  char v15; // stack - 0xe6
  int4 v16;
  char *v17;
  char *v18; // stack - 0x110
  char *v19; // stack - 0xf0
  int4 v2; // eax
  char v20; // stack - 0xe5
  unsigned int v3; // eax
  char *v4;
  char *v5;
  uint8 v6;
  unsigned long v7; // rax
  char v8 [56];
  char v9 [9];
  
  if (dat_847e1) { // branch-flip
    v17 = a0->field_0x8;
    if (!v17)
      v17 = a0->field_0x0;
  }
  else {
    v17 = a0->field_0x0;
    if (!v17)
      v17 = a0->field_0x8;
  }
  if (dat_84b7c) {
    if (a2 < 0)
      a2 = sub_d180();
    v4 = (char *)sub_43090((a2 - dat_843e0) - dat_843e8,v8);
    __fprintf_chk(dat_83f28,1,dcgettext(NULL,"block %s: ",5),v4);
  }
  v1 = dat_83f28;
  if (dat_84a54 <= 1) {
    fputs_unlocked((char *)sub_4a8a0(v17),v1);
    if ((dat_847e1) && (a0->field_0x10))
      fputc_unlocked(0x2f,dat_83f28);
    fputc_unlocked(10,dat_83f28);
    fflush_unlocked(dat_83f28); // return-dupe
    sub_32150(a0);
    return;
  }
  v14 = '?';
  switch(*(char *)(a1 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x53:
      v14 = (-(a0->field_0x10 == '\0') & 0xc9U) + 100;
      break;
    case 0x31:
      v14 = 'h';
      break;
    case 0x32:
      v14 = 'l';
      break;
    case 0x33:
      v14 = 'c';
      break;
    case 0x34:
      v14 = 'b';
      break;
    case 0x35:
    case 0x44:
      v14 = 'd';
      break;
    case 0x36:
      v14 = 'p';
      break;
    case 0x37:
      v14 = 'C';
      break;
    case 0x4b:
    case 0x4c:
      v14 = 'L';
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected long name header",5));
      dat_84d58 = 2;
      break;
    case 0x4d:
      v14 = 'M';
      break;
    case 0x56:
      dat_84451 = 1;
      v14 = 'V';
    
  }
  sub_32310(a0->field_0x70,v9);
  sub_32010(a0,v10);
  v18 = (char *)sub_22080(a0->field_0xf8,a0->field_0x100,(uint1)dat_84b90);
  v16 = (int4)strlen(v18);
  if (dat_81228 < v16)
    dat_81228 = v16;
  v4 = a0->field_0x20;
  if ((((!v4) || (!*v4)) || (dat_84400 == 1)) || (dat_84ad2))
    v4 = (char *)sub_43090(a0->field_0x74,v11);
  v5 = a0->field_0x28;
  if (((!v5) || (!*v5)) || ((dat_84400 == 1 || (dat_84ad2))))
    v5 = (char *)sub_43090(a0->field_0x78,v12);
  if (2 <= (uint4)((int4)*(char *)(a1 + 0x9c) - 0x33U)) { // branch-flip
    v6 = a0->field_0x88;
    __strcpy_chk(v8,(char *)sub_43090(v6,v13),0x2a);
  }
  else {
    v3 = gnu_dev_major(a0->field_0x80);
    __strcpy_chk(v8,(char *)sub_43090(v3,v13),0x2a);
    __strcat_chk(v8,",",0x2a);
    v3 = gnu_dev_minor(a0->field_0x80);
    __strcat_chk(v8,(char *)sub_43090(v3,v13),0x2a);
  }
  v19 = v13;
  v16 = (int4)strlen(v8);
  v6 = strlen(v4);
  v2 = (int4)v6 + 2 + (int4)strlen(v5) + v16;
  if (v2 <= dat_8122c) // branch-flip
    v16 = (dat_8122c - v2) + v16;
  else {
    dat_8122c = v2;
  }
  __fprintf_chk(dat_83f28,1,"%s %s/%s %*s %-*s",&v14,v4,v5,v16,v8,dat_81228,v18);
  __fprintf_chk(dat_83f28,1," %s",(char *)sub_4a8a0(v17));
  if ((dat_847e1) && (a0->field_0x10))
    fputc_unlocked(0x2f,dat_83f28);
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
      putc_unlocked(10,dat_83f28);
      break;
    default:
      v20 = 0;
      v15 = *(char *)(a1 + 0x9c);
      v7 = sub_4ac50(&v15);
      v17 = " unknown file type %s\n";
      goto label_22652;
    case 0x31:
      v7 = sub_4a8a0(a0->field_0x18);
      v17 = " link to %s\n";
label_22652:
      __fprintf_chk(dat_83f28,1,dcgettext(NULL,v17,5),v7);
      break;
    case 0x32:
      v17 = a0->field_0x18;
      __fprintf_chk(dat_83f28,1," -> %s\n",sub_4a8a0(v17));
      break;
    case 0x4b:
      v17 = "--Long Link--\n";
      goto label_22764;
    case 0x4c:
      v17 = "--Long Name--\n";
      goto label_22764;
    case 0x4d:
      v6 = sub_22050((uint1 *)(a1 + 0x171),0xc);
      __strcpy_chk(v8,(char *)sub_43090(v6,v19),0x2a);
      __fprintf_chk(dat_83f28,1,dcgettext(NULL,"--Continued at byte %s--\n",5),v8);
      break;
    case 0x56:
      v17 = "--Volume Header--\n";
label_22764:
      __fprintf_chk(dat_83f28,1,dcgettext(NULL,v17,5));
    
  }
  fflush_unlocked(dat_83f28);
  sub_32150(a0);
  return;
}


// Function: sub_228b0 @ 0x228b0
void sub_228b0(void)
{
  unsigned long *v1;
  char v10; // stack - 0x18c
  int8 v2; // rcx
  unsigned long v3 [17];
  char v4 [8];
  char v5 [440];
  char v6 [4];
  unsigned long *v7;
  unsigned long v8; // stack - 0x1a0
  unsigned int v9; // stack - 0x198
  
  v2 = 0x40;
  v7 = v3;
  while (v2) {
    v1 = &v7[1];
    *v7 = 0;
    v2 -= 1;
    v7 = v1;
  }
  v10 = 0x56;
  if (dat_84560) {
    v8 = *(unsigned long *)(dat_84560 + 0x88);
    v9 = *(unsigned int *)(dat_84560 + 0x90);
  }
  sub_2f630(v4);
  sub_23be0(v5,".");
  sub_215c0(v3,v4,v6,0);
  sub_23be0(v5,dat_83f18);
  sub_22220(v4,v3,0);
  sub_2f6f0(v4);
  return;
}


// Function: sub_22990 @ 0x22990
void sub_22990(struct_9 *a0,int8 a1,int8 a2)
{
  if (((dat_84400 == 4) && (!dat_84451)) && (dat_83f18)) {
    sub_228b0();
    dat_84451 = 1;
    sub_22220(a0,a1,a2); // tail-call
    return;
  }
  sub_22220(a0,a1,a2); // tail-call
  return;
}


// Function: sub_22a00 @ 0x22a00
void sub_22a00(char *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  char v3 [10];
  char v4; // stack - 0x43
  char v5 [24];
  
  if (dat_84a54 <= 1)
    return;
  v4 = 'd';
  sub_32310(a2,v3);
  if (dat_84b7c) {
    v1 = (char *)sub_43090(sub_d180(),v5);
    __fprintf_chk(dat_83f28,1,dcgettext(NULL,"block %s: ",5),v1);
  }
  v1 = (char *)sub_4a8a0(a0);
  v2 = dcgettext(NULL,"Creating directory:",5);
  __fprintf_chk(dat_83f28,1,"%s %*s %s\n",&v4,dat_8122c + 1 + dat_81228,v2,v1);
  return;
}


// Function: sub_22b10 @ 0x22b10
void sub_22b10(int8 a0) // return-dupe
{
  uint8 v1;
  uint8 v2; // rax
  int8 v3;
  
  if (dat_847f8) {
    v3 = sub_d540(a0);
    if (0 <= v3) // branch-flip
      a0 += v3 * -0x200;
    else {
      dat_847f8 = '\0';
    }
  }
  sub_ccf0(a0);
  if (a0 <= 0)
    return;
  v1 = a0 - 1;
  v3 = a0 + -0x200;
  do {
    v2 = sub_e130();
    if (!v2) {
      if (dat_84d50)
        (*dat_84d50)(0);
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      sub_163d0(); // no-return
    }
    sub_d1f0(v2);
    a0 -= 0x200;
    sub_ccf0(a0);
  } while (a0 != v3 - (v1 & 0xfffffffffffffe00));
  return;
}


// Function: sub_22bd0 @ 0x22bd0
void sub_22bd0(void)
{
  char v1;
  
  if (dat_849c9)
    return;
  v1 = *(char *)(dat_84408 + 0x9c);
  sub_d1f0(dat_84408);
  sub_ccb0((struct_18 *)0x84840);
  if (!dat_84960) {
    if (v1 == '5') {
      sub_ccd0(); // tail-call
      return;
    }
    sub_22b10(dat_848c8);
    sub_ccd0(); // tail-call
    return;
  }
  sub_29fe0(0x84840);
  sub_ccd0(); // tail-call
  return;
}


// Function: sub_22c60 @ 0x22c60
void sub_22c60(code *a0)
{
  char v1;
  unsigned long v2; // rax
  char v3 [24];
  int4 v4; // eax
  
  v4 = 0;
  sub_21160();
  sub_26d90();
  sub_fb70(0);
label_22cbd:
  sub_2f6f0((unsigned long *)0x84840);
  switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
    case 0:
    case 2:
label_231f0:
      abort(); // no-return
    case 1:
label_22dc0:
label_22dc7:
      sub_215c0(dat_84408,0x84840,(int4 *)0x84400,1);
      v1 = sub_27250(dat_84848);
      if (v1) {
        if (0 <= dat_84b08) {
          dat_84938 = sub_21320((uint1 *)(dat_84408 + 0x88));
          dat_84940 = 0;
          v4 = sub_4be80(dat_84938,0,dat_84b00,dat_84b08);
          if (v4 < 0) goto label_22df2;
        }
        v1 = sub_16b00(dat_84848,dat_849d8);
        if (v1) goto label_22df2;
        v4 = (int4)*(char *)(dat_84408 + 0x9c);
label_23000:
        sub_21440(v4,0x84840);
        (*a0)();
        if (!sub_273a0(0x84840)) goto label_2301e;
        break;
      }
label_22df2:
      v1 = *(char *)(dat_84408 + 0x9c);
      v4 = (int4)v1;
      if ((v1 == 'M') || (v1 == 'V')) goto label_23000;
      if ((v1 == '5') && (dat_84a89)) {
        if (dat_84d50)
          (*dat_84d50)();
        v2 = sub_4aa70(dat_84848);
        error(0,0,dcgettext(NULL,"%s: Omitting",5),v2);
      }
      sub_22bd0();
      if (sub_273a0(0x84840)) break;
      sub_2f6f0((unsigned long *)0x84840);
      switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
        case 0:
        case 2:
          goto label_231f0;
        case 1:
          goto label_22dc7;
        case 3:
          goto label_23138;
        case 4:
          goto label_22ce8;
        case 5:
          goto label_23120;
        default:
          break;
        
      }
    case 3:
      goto label_22d2e;
    case 4:
label_22ce8:
      if (dat_84b7c) {
        v2 = sub_43090(sub_d180(),v3);
        __fprintf_chk(dat_83f28,1,dcgettext(NULL,"block %s: ** End of File **\n",5),v2);
      }
      break;
    case 5:
      sub_d1f0(dat_84408);
      if (v4) {
        if (v4 == 1) goto label_22eae;
        goto label_22f49;
      }
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
      dat_84d58 = 2;
      goto label_22eae;
    
  }
  sub_f900(); // return-dupe
  sub_27430(); // tail-call
  return;
label_2301e:
  sub_2f6f0((unsigned long *)0x84840);
  switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
    case 0:
    case 2:
      goto label_231f0;
    case 1:
      goto label_22dc7;
    case 3:
label_23138:
      v4 = 1;
      goto label_22d2e;
    case 4:
      goto label_22ce8;
    case 5:
      goto label_23120;
    default:
      goto label_22cf5;
    
  }
label_23120:
  sub_d1f0(dat_84408);
label_22eae:
  if (dat_84b7c) {
    v2 = sub_43090((sub_d180() - dat_843e0) - dat_843e8,v3);
    __fprintf_chk(dat_83f28,1,dcgettext(NULL,"block %s: ",5),v2);
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Skipping to next header",5));
  dat_84d58 = 2;
  if (sub_273a0(0x84840)) {
    sub_f900();
    sub_27430();
    return;
  }
  sub_2f6f0((unsigned long *)0x84840);
  switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
    case 0:
    case 2:
      goto label_231f0;
    case 1:
      goto label_22dc0;
    case 3:
      goto label_22d28;
    case 4:
      goto label_22ce8;
    case 5:
      sub_d1f0(dat_84408);
label_22f49:
      do {
        if (sub_273a0(0x84840)) {
          sub_f900();
          sub_27430();
          return;
        }
        sub_2f6f0((unsigned long *)0x84840);
        switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
          case 0:
          case 2:
            goto label_231f0;
          case 1:
            goto label_22dc0;
          case 3:
            goto label_22d28;
          case 4:
            goto label_22ce8;
          case 5:
            sub_d1f0(dat_84408);
            if (sub_273a0(0x84840)) {
              sub_f900();
              sub_27430();
              return;
            }
            sub_2f6f0((unsigned long *)0x84840);
            switch(sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0)) {
              case 0:
              case 2:
                goto label_231f0;
              case 1:
                goto label_22dc0;
              case 3:
                goto label_22d28;
              case 4:
                goto label_22ce8;
              case 5:
                sub_d1f0(dat_84408);
                break;
              default:
                goto label_22cf5;
              
            }
            break;
          default:
            goto label_22cf5;
          
        }
      } while( true );
    default:
      goto label_22cf5;
    
  }
label_22d28:
  v4 = 5;
label_22d2e:
  if (dat_84b7c) {
    v2 = sub_43090(sub_d180(),v3);
    __fprintf_chk(dat_83f28,1,dcgettext(NULL,"block %s: ** Block of NULs **\n",5),v2);
  }
  sub_d1f0(dat_84408);
  if (!dat_84b51) {
    if ((sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0) != 3) && (dat_83b80 & 1)) {
      if (dat_84d50)
        (*dat_84d50)();
      v2 = sub_43090(sub_d180(),v3);
      error(0,0,dcgettext(NULL,"A lone zero block at %s",5),v2);
    }
    sub_f900();
    sub_27430();
    return;
  }
  if (sub_273a0(0x84840)) {
label_22cf5:
    sub_f900();
    sub_27430();
    return;
  }
  goto label_22cbd;
}


// Function: sub_23260 @ 0x23260
void sub_23260(void)
{
  int8 v1; // rax
  
  v1 = sub_d180();
  if (dat_84a54) {
    sub_22990((struct_9 *)0x84840,dat_84408,v1);
    if ((dat_84b50) && (3 <= dat_84a54)) {
      if (sub_201b0(0x84840)) {
        v1 = sub_1f4c0(dat_849d0);
        sub_20710(dat_849d0,v1);
      }
    }
  }
  sub_22bd0(); // tail-call
  return;
}


// Function: sub_232e0 @ 0x232e0
void sub_232e0(void)
{
  char v1; // al
  char *v2; // rax
  
  sub_21160();
  sub_26d90();
  sub_fb70(0);
  if (sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0) == 1) {
    sub_215c0(dat_84408,0x84840,(int4 *)0x84400,0);
    if (*(char *)(dat_84408 + 0x9c) == 'V')
      sub_23c20((unsigned long *)0x83f18,dat_84408,100);
    if (dat_83f18) {
      if (dat_84a54)
        sub_228b0();
      v1 = sub_27250(dat_83f18);
      if ((!v1) && (dat_84b10)) {
        v2 = (char *)sub_de80(dat_83f18);
        sub_27250(v2);
        free(v2);
      }
    }
  }
  sub_f900();
  sub_275b0(); // tail-call
  return;
}


// Function: sub_233c0 @ 0x233c0
uint8 sub_233c0(uint8 *a0,uint8 a1)
{
  return *a0 % a1;
}


// Function: sub_233d0 @ 0x233d0
unsigned long sub_233d0(int8 *a0,int8 *a1)
{
  return CONCAT71((undefined7)((uint8)*a1 >> 8),*a0 == *a1);
}


// Function: sub_233e0 @ 0x233e0
uint8 sub_233e0(char *a0)
{
  passwd *v1; // rax
  
  v1 = getpwnam(a0);
  if (v1)
    return (uint8)*(uint4 *)&v1->field_0x10;
  return 0xffffffffffffffff;
}


// Function: sub_23410 @ 0x23410
uint8 sub_23410(char *a0)
{
  group *v1; // rax
  
  v1 = getgrnam(a0);
  if (v1)
    return (uint8)*(uint4 *)&v1->field_0x10;
  return 0xffffffffffffffff;
}


// Function: sub_239a0 @ 0x239a0
void sub_239a0(char *a0)
{
  sub_23520((int8 *)0x84570,a0,sub_233e0,"UID"); // tail-call
  return;
}


// Function: sub_239d0 @ 0x239d0
bool sub_239d0(uint8 a0,int4 *a1,int8 *a2) // early-return
{
  int8 v1;
  int8 v2; // rax
  uint8 v3 [3]; // stack - 0x38
  
  if (dat_84570) {
    v3[0] = a0 & 0xffffffff;
    v2 = sub_418d0(dat_84570,v3);
    if (v2) {
      v1 = *(int8 *)(v2 + 0x10);
      *a1 = (int4)*(unsigned long *)(v2 + 8);
      *a2 = v1;
      return 0;
    }
  }
  if (dat_84abc != -1)
    *a1 = dat_84abc;
  if (!dat_84ac0)
    return dat_84abc == -1;
  *a2 = dat_84ac0;
  return 0;
}


// Function: sub_23a70 @ 0x23a70
void sub_23a70(char *a0)
{
  sub_23520((int8 *)0x84568,a0,sub_23410,"GID"); // tail-call
  return;
}


// Function: sub_23aa0 @ 0x23aa0
bool sub_23aa0(uint8 a0,int4 *a1,int8 *a2) // early-return
{
  int8 v1;
  int8 v2; // rax
  uint8 v3 [3]; // stack - 0x38
  
  if (dat_84568) {
    v3[0] = a0 & 0xffffffff;
    v2 = sub_418d0(dat_84568,v3);
    if (v2) {
      v1 = *(int8 *)(v2 + 0x10);
      *a1 = (int4)*(unsigned long *)(v2 + 8);
      *a2 = v1;
      return 0;
    }
  }
  if (dat_84b54 != -1)
    *a1 = dat_84b54;
  if (!dat_84b58)
    return dat_84b54 == -1;
  *a2 = dat_84b58;
  return 0;
}


// Function: sub_23b40 @ 0x23b40
int8 sub_23b40(int8 a0,uint8 a1)
{
  uint8 v1;
  
  v1 = (a1 - 1) + a0;
  return v1 - v1 % a1;
}


// Function: sub_23b60 @ 0x23b60
int8 sub_23b60(struct_6 *a0)
{
  int8 v1;
  
  v1 = a0->field_0x0;
  if (*(char *)(v1 + -1 + a0->field_0x10) == '/')
    *(char *)(v1 + a0->field_0x10) = 0;
  free(a0);
  return v1;
}


// Function: sub_23bb0 @ 0x23bb0
void sub_23bb0(unsigned int a0,char *a1)
{
  sub_4aa90(a0,sub_4a570(NULL),a1); // tail-call
  return;
}


// Function: sub_23be0 @ 0x23be0
void sub_23be0(unsigned long *a0,char *a1)
{
  unsigned long v1;
  
  free((void *)*a0);
  v1 = 0;
  if (a1)
    v1 = sub_4c7a0(a1);
  *a0 = v1;
  return;
}


// Function: sub_23c20 @ 0x23c20
void sub_23c20(unsigned long *a0,char *a1,uint8 a2) // return-dupe
{
  uint8 v1; // rax
  void *v2; // rax
  
  free((void *)*a0);
  if (!a1)
    return;
  v1 = strnlen(a1,a2);
  v2 = memcpy((void *)sub_4c5b0(v1 + 1),a1,v1);
  *(char *)((int8)v2 + v1) = 0;
  *a0 = v2;
  return;
}


// Function: sub_23c70 @ 0x23c70
unsigned long sub_23c70(char *a0)
{
  char *v1;
  char v2;
  char *v3;
  char *v4;
  unsigned long v5; // r8
  
  v2 = *a0;
  if (!v2)
    return 1;
  v5 = 1;
  v3 = a0;
  do {
    while( true ) {
      v4 = &v3[1];
      v1 = &a0[1];
      if (v2 != '\\') break;
      switch(v3[1]) {
        case 0x30:
        case 0x31:
        case 0x32:
        case 0x33:
        case 0x34:
        case 0x35:
        case 0x36:
        case 0x37:
          v2 = v3[1] + '\xd0';
          if (8 <= (uint1)(v3[2] - 0x30U)) { // branch-flip
            *a0 = v2;
            v4 = &v3[2];
            v2 = *v4;
          }
          else {
            v2 = v3[2] + '\xd0' + v2 * '\b';
            if (8 <= (uint1)(v3[3] - 0x30U)) { // branch-flip
              *a0 = v2;
              v4 = &v3[3];
              v2 = *v4;
            }
            else {
              v4 = &v3[4];
              *a0 = v3[3] + '\xd0' + v2 * '\b';
              v2 = *v4;
            }
          }
          break;
        default:
          *a0 = '\\';
          if (!v3[1]) {
            v5 = 0;
            goto label_23cf0;
          }
          a0[1] = v3[1];
          v4 = &v3[2];
          v2 = *v4;
          v1 = &a0[2];
          v5 = 0;
          break;
        case 0x3f:
          *a0 = '\x7f';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x5c:
          *a0 = '\\';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x61:
          *a0 = '\a';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x62:
          *a0 = '\b';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x66:
          *a0 = '\f';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x6e:
          *a0 = '\n';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x72:
          *a0 = '\r';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x74:
          *a0 = '\t';
          v4 = &v3[2];
          v2 = *v4;
          break;
        case 0x76:
          *a0 = '\v';
          v4 = &v3[2];
          v2 = *v4;
        
      }
label_23ca9:
      a0 = v1;
      v3 = v4;
      if (!v2) goto label_23cf0;
    }
    if (a0 != v3) {
      *a0 = v2;
      v2 = v3[1];
      goto label_23ca9;
    }
    v2 = a0[1];
    a0 = v1;
    v3 = v4;
  } while (v2);
label_23cf0:
  if (v4 != v1)
    *v1 = '\0';
  return v5;
}


// Function: sub_23e60 @ 0x23e60
char * sub_23e60(char *a0)
{
  uint8 v1; // rax
  char *v2; // rax
  char *v3;
  
  if ((a0) && (*a0)) {
    v1 = strlen(a0);
    v3 = &a0[v1 - 1];
    if (a0 < &a0[v1 - 1]) {
      do {
        if (*v3 != '/')
          return a0;
        v2 = &v3[-1];
        *v3 = '\0';
        v3 = v2;
      } while (a0 != v2);
    }
  }
  return a0;
}


// Function: sub_23ea0 @ 0x23ea0
void sub_23ea0(char *a0) // return-dupe
{
  char *v1;
  char *v2;
  char v3;
  char v4;
  char *v5;
  
  v3 = *a0;
  v2 = a0;
  v1 = a0;
  v5 = a0;
  if (v3 == '.') {
    do {
      v1 = v2;
      if (v2[1] != '/') break;
      v3 = v2[2];
      v1 = &v2[2];
      if (v3 == '/') {
        v1 = &v2[3];
        v3 = v2[3];
        while (v3 == '/') {
          v2 = &v1[1];
          v1 = &v1[1];
          v3 = *v2;
        }
      }
      v5 = &v5[v3 == '\0'];
      *v5 = v3;
      v2 = v1;
    } while (v3 == '.');
    v3 = *v1;
  }
  *v5 = v3;
  while (v2 = &v5[1], v5 = v2, v4 = v3, v3) {
    while( true ) {
      v1 = &v1[1];
      v3 = *v1;
      if ((v4 == '/') && (v1[v3 == '.'] == '/')) break;
      v2 = &v5[1];
      *v5 = v3;
      v5 = v2;
      v4 = v3;
      if (!v3) goto label_23f80;
    }
    do {
      v1 = &v1[(uint8)(v3 == '.') + 1];
      v3 = *v1;
    } while (v1[v3 == '.'] == '/');
    *v5 = v3;
  }
label_23f80:
  if ((int8)v2 - (int8)a0 <= 2)
    return;
  if (v2[-2] != '.') { // branch-flip
    if (v2[-2] != '/') goto label_23fa0;
  }
  else if ((v2[-3] != '/') || (v2 = &v2[-1], (int8)v2 - (int8)a0 <= 2)) goto label_23fa0;
  v2 = &v2[-1];
label_23fa0:
  v2[-1] = '\0';
  return;
}


// Function: sub_23fc0 @ 0x23fc0
void sub_23fc0(unsigned long *a0,void *a1,uint8 a2,char *a3,uint8 a4)
{
  uint8 v1; // rax
  char *v2;
  char *v3;
  
  v2 = (char *)*a0;
  v1 = strlen(v2);
  if (a2 < v1) {
    if ((!memcmp(v2,a1,a2)) && (v3 = &v2[a2], *v3 == '/')) {
      if (a2 < a4) {
        v2 = (char *)sub_4c610(v2,(v1 - a2) + 1 + a4);
        *a0 = v2;
        v3 = &v2[a2];
      }
      memmove(&v2[a4],v3,(v1 - a2) + 1);
      memcpy(v2,a3,a4); // tail-call
      return;
    }
  }
  return;
}


// Function: sub_24080 @ 0x24080
char * sub_24080(uint8 a0,unsigned long a1,uint8 a2,int8 a3)
{
  if (a0 <= a2)
    return sub_43090(a0,a3); // tail-call
  return sub_42f50(a0,a3); // tail-call
}


// Function: sub_240a0 @ 0x240a0
uint8 sub_240a0(char *a0,unsigned long a1,uint8 a2,uint8 a3) // early-return
{
  int4 *v1; // rax
  uint8 v2;
  uint8 v3; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  if (0 <= (int8)a3) { // branch-flip
    if ((uint4)((int4)a0[*a0 == '-'] - 0x30U) <= 9) {
      v3 = strtoimax(a0,a1,10);
      if ((((int8)v3 < (int8)a2) || (v2 = v3, (int8)a3 < (int8)v3)) && (*v1 = 0x22, v2 = a3, (int8)v3 < (int8)a2))
        v2 = a2;
      return v2;
    }
  }
  else if ((uint4)((int4)*a0 - 0x30U) <= 9) {
    v3 = strtoumax(a0,a1,10);
    if (a3 < v3) {
      *v1 = 0x22;
      return a3;
    }
    return sub_23b90(v3); // tail-call
  }
  *v1 = 0x16;
  return 0;
}


// Function: sub_24170 @ 0x24170
void sub_24170(int4 a0,char *a1)
{
  int8 v1; // rax
  int8 v2; // rdi
  int4 v3;
  int4 v4; // r8d
  
  if (!a0) {
    *a1 = 0;
    return;
  }
  v1 = (int8)a0;
  *a1 = 0x2e;
  if (a0 % 10) { // branch-flip
    a1[10] = 0;
    v2 = 8;
    a1[9] = (char)(a0 % 10) + '0';
  }
  else {
    v3 = 9;
    do {
      v4 = v3;
      v3 = v4 + -1;
      a0 = (int4)(v1 * 0x66666667 >> 0x22) - (a0 >> 0x1f);
      v1 = (int8)a0;
    } while (!(a0 % 10));
    v4 -= 2;
    v2 = (int8)v4;
    a1[(int8)v3 + 1] = 0;
    a1[v2 + 1] = (char)(a0 % 10) + '0';
    if (!v4)
      return;
  }
  do {
    a0 = (int4)(v1 * 0x66666667 >> 0x22) - (a0 >> 0x1f);
    v1 = (int8)a0;
    a1[v2] = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    v2 -= 1;
  } while ((int4)v2);
  return;
}


// Function: sub_24270 @ 0x24270
int8 sub_24270(uint8 a0,uint4 a1,int8 a2)
{
  int8 v1;
  
  if (1000000000 <= a1) { // branch-flip
    a1 = 0;
label_242d2:
    if (0 <= (int8)a0) {
      v1 = sub_43090(a0,a2 + 1);
      sub_24170(a1,(char *)(a2 + 0x15)); // return-dupe
      return v1;
    }
  }
  else {
    if ((!a1) || (0 <= (int8)a0)) goto label_242d2;
    a0 += 1;
    a1 = 1000000000 - a1;
  }
  v1 = sub_43090(-a0,a2 + 1);
  *(char *)(v1 + -1) = 0x2d;
  v1 -= 1;
  sub_24170(a1,(char *)(a2 + 0x15));
  return v1;
}


// Function: sub_242f0 @ 0x242f0
undefined16 sub_242f0(char *a0,unsigned long *a1,bool a2)
{
  char v1;
  char *v10;
  uint4 v11; // edx
  uint4 v12; // r9d
  int4 v13;
  int4 v2; // eax
  int4 v3;
  int4 *v4; // rax
  char v5 [16];
  int8 v6;
  char v7 [16];
  char v8 [16];
  int8 v9; // rdx
  
  v1 = *a0;
  v4 = __errno_location();
  if (10 <= (uint4)((int4)a0[v1 == '-'] - 0x30U)) {
    *v4 = 0x16;
    *a1 = a0;
    v5._8_8_ = 0xffffffffffffffff;
    v5._0_8_ = 0x8000000000000000;
    return v5._0_16_;
  }
  *v4 = 0;
  if (v1 != '-') { // branch-flip
    v6 = strtoumax();
    if (0 <= v6) {
      v10 = (char *)*a1;
      v13 = *v4;
      goto label_24474;
    }
    *v4 = 0x22;
    v10 = (char *)*a1;
    if ((!a2) || (*v10 != '.')) {
      *a1 = v10;
      v8._8_8_ = 0xffffffffffffffff;
      v8._0_8_ = 0x8000000000000000;
      return v8._0_16_;
    }
    v13 = 0x22;
    v6 = -0x8000000000000000;
  }
  else {
    v6 = strtoimax(a0,a1,10);
    v10 = (char *)*a1;
    v13 = *v4;
label_24474:
    v3 = 0;
    if ((!a2) || (v3 = 0, *v10 != '.')) goto label_24403;
  }
  v11 = (int4)v10[1];
  v10 = &v10[1];
  if (10 <= (uint4)(v11 - 0x30U)) { // branch-flip
    v3 = 0;
    v12 = 0;
    v2 = 0;
label_243f0:
    do {
      v2 += 1;
      v3 *= 10;
    } while (v2 != 9);
  }
  else {
    v12 = 0;
    v2 = 0;
    v3 = 0;
    do {
      if (9 <= v2) // branch-flip
        v12 |= (char)v11 != '0';
      else {
        v2 += 1;
        v3 = v11 + -0x30 + v3 * 10;
      }
      v11 = (int4)v10[1];
      v10 = &v10[1];
    } while ((uint4)(v11 - 0x30U) <= 9);
    if (v2 <= 8) goto label_243f0;
  }
  if (v1 == '-') {
    v11 = v3 + v12;
    v3 = 0;
    if (v11) {
      if (v6 != -0x8000000000000000) { // branch-flip
        v6 -= 1;
        v3 = 1000000000 - v11;
      }
      else {
        v3 = -1;
      }
    }
  }
label_24403:
  v9 = (int8)v3;
  *a1 = v10;
  if (v13 == 0x22)
    v9 = -1;
  v7._8_8_ = v9;
  v7._0_8_ = v6;
  return v7._0_16_;
}


// Function: sub_244e0 @ 0x244e0
char sub_244e0(char *a0)
{
  char v1; // al
  
  v1 = *a0;
  if (v1 != '/') { // branch-flip
    while (v1 == '.') {
      while( true ) {
        v1 = a0[1];
        if (v1 != '/') {
          v1 = v1 == '\0';
          return v1;
        }
        v1 = a0[2];
        a0 = &a0[2];
        if (v1 != '/') break;
        do {
          v1 = a0[1];
          a0 = &a0[1];
        } while (v1 == '/');
        if (v1 != '.') {
          v1 = v1 == '\0';
          return v1;
        }
      }
    }
  }
  else {
    while( true ) {
      for (; v1 = a0[1], v1 == '/'; a0 = &a0[1]) {
      }
      if (v1 != '.') break;
      a0 = &a0[(uint8)(a0[2] == '.') + 2];
      if (*a0 != '/') {
        v1 = 0;
        return v1;
      }
    }
  }
  v1 = v1 == '\0';
  return v1;
}


// Function: sub_24560 @ 0x24560
unsigned int sub_24560(char *a0) // return-dupe x2
{
  if (sub_244e0(a0)) {
    *__errno_location() = 0;
    return 0xffffffff;
  }
  if (!unlinkat(dat_81234,a0,0x200)) {
    sub_19b90(a0);
    return 0;
  }
  return 0xffffffff;
}


// Function: sub_245d0 @ 0x245d0
void sub_245d0(void)
{
  int4 v1;
  char *v2; // rax
  char *v3; // rax
  
  if (dat_845f8) {
    if (renameat(dat_81234,dat_845f8,dat_81234,dat_84600)) {
      v1 = *__errno_location();
      if (dat_84d50)
        (*dat_84d50)();
      v2 = (char *)sub_4ac40(1,dat_84600);
      v3 = (char *)sub_4aa70(dat_845f8);
      error(0,v1,dcgettext(NULL,"%s: Cannot rename to %s",5),v3,v2);
      dat_84d58 = 2;
    }
    if (dat_84a54) {
      v2 = (char *)sub_4ac40(1,dat_84600);
      v3 = (char *)sub_4ac40(0,dat_845f8);
      __fprintf_chk(dat_83f28,1,dcgettext(NULL,"Renaming %s back to %s\n",5),v3,v2);
    }
    sub_23be0((unsigned long *)0x845f8,NULL); // tail-call
    return;
  }
  return;
}


// Function: sub_246f0 @ 0x246f0
void sub_246f0(char *a0,stat *a1)
{
  fstatat(dat_81234,a0,a1,dat_84800); // tail-call
  return;
}


// Function: sub_24710 @ 0x24710
unsigned long sub_24710(char *a0,bool a1)
{
  int4 v1;
  uint8 v2; // rax
  char *v3; // rax
  char *v4; // rax
  char v5 [24];
  uint4 v6; // stack - 0xb0
  
  sub_23be0((unsigned long *)0x84600,a0);
  sub_23be0((unsigned long *)0x845f8,NULL);
  if ((((!a1) || (dat_84d70)) || (dat_84d78 = strchr(a0,0x3a), !dat_84d78)) || ((dat_84d78 <= a0 || (v2 = (int8)dat_84d78 - (int8)a0, memchr(a0,0x2f,v2))))) {
    if (sub_246f0(a0,v5)) { // branch-flip
      if (*__errno_location() != 2) {
        sub_32d30(a0);
        return 0;
      }
    }
    else if (((v6 & 0xf000) != 0x4000) && ((!a1 || ((v6 & 0xb000) != 0x2000)))) {
      dat_845f8 = (char *)sub_3e2f0(dat_81234,a0,dat_84b80);
      if (!dat_845f8)
        sub_16420(); // no-return
      if (renameat(dat_81234,dat_84600,dat_81234,dat_845f8)) {
        v1 = *__errno_location();
        if (dat_84d50)
          (*dat_84d50)();
        v3 = (char *)sub_4ac40(1,dat_845f8);
        v4 = (char *)sub_4aa70(dat_84600);
        error(0,v1,dcgettext(NULL,"%s: Cannot rename to %s",5),v4,v3);
        dat_84d58 = 2;
        sub_23be0((unsigned long *)0x845f8,NULL);
        return 0;
      }
      if (dat_84a54) {
        v3 = (char *)sub_4ac40(1,dat_845f8);
        v4 = (char *)sub_4ac40(0,dat_84600);
        __fprintf_chk(dat_83f28,1,dcgettext(NULL,"Renaming %s to %s\n",5),v4,v3);
      }
    }
  }
  return 1;
}


// Function: sub_24960 @ 0x24960
int8 sub_24960(uint8 a0,void *a1,uint8 a2)
{
  int8 v1; // rax
  uint8 v2; // rax
  
  v1 = sub_4af10(a0,a1,a2);
  if (v1 == -1) {
    if (*__errno_location() == 0xb) {
      v2 = sub_4d180(a0 & 0xffffffff,3);
      if ((0 <= (int4)(uint4)v2) && (v2 & 0x800)) {
        if (sub_4d180(a0 & 0xffffffff,4,(uint4)v2 & 0xfffff7ff) != -1)
          return sub_4af10(a0 & 0xffffffff,a1,a2); // tail-call
      }
    }
  }
  return v1;
}


// Function: sub_249f0 @ 0x249f0
uint8 sub_249f0(uint8 a0,void *a1,uint8 a2)
{
  uint8 v1; // rax
  uint8 v2; // rax
  
  v1 = sub_41120(a0,a1,a2);
  if (v1 < a2) {
    if (*__errno_location() == 0xb) {
      v2 = sub_4d180(a0 & 0xffffffff,3);
      if ((0 <= (int4)(uint4)v2) && (v2 & 0x800)) {
        if (sub_4d180(a0 & 0xffffffff,4,(uint4)v2 & 0xfffff7ff) != -1)
          v1 += sub_41120(a0 & 0xffffffff,(void *)((int8)a1 + v1),a2 - v1);
      }
    }
  }
  return v1;
}


// Function: sub_24a80 @ 0x24a80
void sub_24a80(void)
{
  sub_3f360();
  return;
}


// Function: sub_24ae0 @ 0x24ae0
int4 sub_24ae0(void) // early-return
{
  if (dat_845e8)
    return (int4)dat_845e8 + -1;
  return 0;
}


// Function: sub_24b00 @ 0x24b00
uint8 sub_24b00(char *a0)
{
  char *v1;
  int8 *v2;
  char v3;
  uint8 v4;
  
  v4 = dat_845e8;
  if (dat_845e8 == dat_845e0) {
    if (!dat_845e8)
      dat_845e0 = 2;
    dat_845f0 = (unsigned long *)sub_4c680(dat_845f0,(uint8 *)0x845e0,0x18);
    v4 = dat_845e8;
    if (!dat_845e8) {
      dat_845f0[1] = 0;
      *dat_845f0 = ".";
      v4 = 1;
      *(unsigned int *)&dat_845f0[2] = 0xffffff9c;
      dat_845e8 = 1;
    }
  }
  if (*a0 == '.') {
    do {
      while( true ) {
        if (a0[1] != '/') {
          v3 = a0[1];
          goto label_24c19;
        }
        v1 = &a0[2];
        a0 = &a0[2];
        if (*v1 == '/') break;
        if (*v1 != '.') goto label_24b87;
      }
      do {
        v1 = &a0[1];
        a0 = &a0[1];
      } while (*v1 == '/');
    } while (*v1 == '.');
label_24b87:
    v3 = *a0;
label_24c19:
    if (!v3)
      return (uint8)(uint4)((int4)v4 - 1);
  }
  v2 = &dat_845f0[v4 * 3];
  *v2 = (int8)a0;
  v2[1] = 0;
  *(unsigned int *)&v2[2] = 0;
  dat_845e8 = v4 + 1;
  return v4;
}


// Function: sub_24c30 @ 0x24c30
unsigned long sub_24c30(void)
{
  return *(unsigned long *)(dat_845f0 + (int8)dat_84580 * 0x18);
}


// Function: sub_24c50 @ 0x24c50
void sub_24c50(char *a0)
{
  if (!dat_84b52) {
    sub_32680(a0); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_326a0(a0); // tail-call
  return;
}


// Function: sub_24c80 @ 0x24c80
void sub_24c80(int4 a0) // return-dupe x2, return-dupe
{
  uint8 v1;
  unsigned long *v2;
  int4 v3;
  int4 v4;
  int4 *v5;
  int4 *v6; // rax
  char *v7;
  
  if (dat_84580 == a0)
    return;
  v2 = (unsigned long *)(dat_845f0 + (int8)a0 * 0x18);
  v4 = *(int4 *)&v2[2];
  if (!v4) {
    v7 = (char *)*v2;
    if (*v7 != '/') {
      sub_24c80(a0 + -1);
      v7 = (char *)*v2;
    }
    v4 = __openat_2(dat_81234,v7,dat_84804 & 0xfffdffff);
    if (v4 < 0)
      sub_327e0((char *)*v2); // no-return
    *(int4 *)&v2[2] = v4;
    if (0x10 <= dat_84590) {
      v2 = (unsigned long *)(dat_845f0 + (int8)dat_845dc * 0x18);
      v3 = *(int4 *)&v2[2];
      if (close(v3))
        sub_24c50((char *)*v2);
      *(unsigned int *)&v2[2] = 0;
      dat_845dc = a0;
      if (v4 <= 0) {
        dat_81234 = v4;
        dat_84580 = a0;
        return;
      }
      goto label_24d0a;
    }
    v1 = dat_84590 + 1;
    *(int4 *)(dat_84590 * 4 + 0x845a0) = a0;
    dat_84590 = v1;
  }
  if (v4 <= 0) {
    dat_84580 = a0;
    dat_81234 = v4;
    return;
  }
label_24d0a:
  if (a0 == dat_845a0) {
    dat_81234 = v4;
    dat_84580 = a0;
    dat_845a0 = a0;
    return;
  }
  v5 = (int4 *)0x845a4;
  do {
    v3 = *v5;
    v6 = &v5[1];
    *v5 = dat_845a0;
    v5 = v6;
    dat_845a0 = v3;
  } while (a0 != v3);
  dat_81234 = v4;
  dat_84580 = a0;
  dat_845a0 = a0;
  return;
}


// Function: sub_24dd0 @ 0x24dd0
void sub_24dd0(char *a0)
{
  if (!dat_84b52) {
    sub_327c0(a0); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32800(a0); // tail-call
  return;
}


// Function: sub_24e00 @ 0x24e00
void sub_24e00(char *a0,uint8 a1,uint8 a2)
{
  if (!dat_84b52) {
    sub_32840(a0,a1,a2); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32900(a0,a1,a2); // tail-call
  return;
}


// Function: sub_24e30 @ 0x24e30
void sub_24e30(char *a0)
{
  if (!dat_84b52) {
    sub_32a60(a0); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32a80(a0); // tail-call
  return;
}


// Function: sub_24e60 @ 0x24e60
void sub_24e60(char *a0)
{
  if (!dat_84b52) {
    sub_32ac0(a0); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32ae0(a0); // tail-call
  return;
}


// Function: sub_24e90 @ 0x24e90
void sub_24e90(char *a0,uint8 a1)
{
  if (!dat_84b52) {
    sub_32b20(a0,a1); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32bf0(a0,a1); // tail-call
  return;
}


// Function: sub_24ec0 @ 0x24ec0
void sub_24ec0(char *a0)
{
  if (!dat_84b52) {
    sub_32d30(a0); // tail-call
    return;
  }
  if (!(dat_83b82 & 0x80))
    return;
  sub_32d50(a0); // tail-call
  return;
}


// Function: sub_24ef0 @ 0x24ef0
void sub_24ef0(char *a0,bool a1,code *a2)
{
  char *v1; // rax
  
  if (!a1) {
    if (*__errno_location() == 2) {
      if (dat_83b80 & 0x40) {
        if (dat_84d50)
          (*dat_84d50)();
        v1 = (char *)sub_4aa70(a0);
        error(0,0,dcgettext(NULL,"%s: File removed before we read it",5),v1);
      }
      sub_2f7d0(1); // tail-call
      return;
    }
  }
  (*a2)(a0); // jump-as-call
  return;
}


// Function: sub_24f80 @ 0x24f80
void sub_24f80(void)
{
  if (fork() != -1)
    return;
  sub_32490("fork",dcgettext(NULL,"child process",5)); // no-return
}


// Function: sub_24fc0 @ 0x24fc0
void sub_24fc0(int4 *a0)
{
  if (0 <= pipe(a0))
    return;
  sub_32490("pipe",dcgettext(NULL,"interprocess channel",5)); // no-return
}


// Function: sub_25000 @ 0x25000
void sub_25000(int8 *a0,uint8 a1)
{
  int8 v1; // rax
  uint8 v2; // rbp
  
  v2 = (uint8)getpagesize();
  if (!CARRY8(a1,v2)) {
    v1 = sub_4c5b0(a1 + v2);
    *a0 = v1;
    sub_23b40(v1,v2); // tail-call
    return;
  }
  sub_16420(); // no-return
}


// Function: sub_25040 @ 0x25040
unsigned long * sub_25040(char *a0)
{
  char v1;
  unsigned long *v2; // rax
  char *v3; // rax
  uint8 v4;
  
  v2 = (unsigned long *)sub_4c5b0(0x18);
  v4 = strlen(a0) + 2;
  v2[1] = v4;
  v3 = (char *)sub_4c5b0(v4);
  *v2 = v3;
  strcpy(v3,a0);
  v4 = strlen(v3);
  v1 = v3[v4 - 1];
  v2[2] = v4;
  if (v1 != '/') {
    v3[v4] = '/';
    v2[2] = v4 + 1;
  }
  return v2;
}


// Function: sub_250b0 @ 0x250b0
void sub_250b0(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
  return;
}


// Function: sub_250d0 @ 0x250d0
void * sub_250d0(unsigned long *a0,char *a1)
{
  uint8 v1;
  int8 v2;
  void *v3; // rax
  void *v4;
  
  v3 = (uint8)strlen(a1);
  v1 = (uint8)v3;
  if ((uint8)a0[1] <= (uint8)(a0[2] + 1 + (int8)v3)) { // branch-flip
    v3 = (void *)*a0;
    do {
      v3 = (void *)sub_4c710(v3,&a0[1]);
      *a0 = v3;
      v2 = a0[2];
      v4 = v3;
    } while ((uint8)a0[1] <= a0[2] + 1 + v1);
  }
  else {
    v4 = (void *)*a0;
    v2 = a0[2];
  }
  v3 = (void *)v2;
  strcpy((char *)((int8)v4 + (int8)v3),a1);
  v3 = (void *)*a0;
  return v3;
}


// Function: sub_25150 @ 0x25150
void sub_25150(struct_6 *a0,char *a1)
{
  uint8 v1; // rax
  
  if (*(char *)(a0->field_0x0 + -1 + a0->field_0x10) != '/') {
    sub_250d0(a0,(char *)0x81230);
    a0->field_0x10 = a0->field_0x10 + 1;
  }
  sub_250d0(a0,a1);
  v1 = strlen(a1);
  a0->field_0x10 = a0->field_0x10 + v1;
  return;
}


// Function: sub_251a0 @ 0x251a0
int8 sub_251a0(int4 a0)
{
  unsigned long *v1;
  int8 v10;
  int8 v11; // r13
  char *v2;
  unsigned int v3;
  int4 v4;
  int8 *v5;
  int8 *v6; // rax
  struct_6 *v7; // rax
  int8 v8; // rax
  int4 v9;
  
  v3 = dat_84580;
  if (dat_845f0) { // branch-flip
    v11 = (int8)a0 * 0x18;
    v10 = *(int8 *)(dat_845f0 + 8 + v11);
    if (!v10) {
      if (0 <= a0) { // branch-flip
        v5 = (int8 *)(dat_845f0 + -0x10 + v11);
        v4 = a0;
        do {
          v9 = v4;
          if (!v9) goto label_252cc;
          v6 = &v5[-3];
          v10 = *v5;
          v5 = v6;
          v4 = v9 + -1;
        } while (!v10);
      }
      else {
        v9 = a0 + 1;
      }
      while (v9 <= a0) {
label_252cc:
        sub_24c80(v9);
        v10 = dat_845f0;
        if (!v9) {
          v8 = sub_4c7c0();
          *(int8 *)(v10 + 8) = v8;
          if (!v8) {
            sub_32490("getcwd","."); // return-dupe, no-return
          }
          if (a0 <= 0) break;
          v9 = 1;
          sub_24c80(1);
        }
        v8 = (int8)v9;
        v9 += 1;
        v10 = v8 * 0x18;
        v1 = (unsigned long *)(dat_845f0 + v10);
        v2 = (char *)*v1;
        if (*v2 != '/') { // branch-flip
          v7 = (struct_6 *)sub_25040(*(char **)(dat_845f0 + -0x10 + v10));
          sub_25150(v7,*(char **)(dat_845f0 + v8 * 0x18));
          v10 += dat_845f0;
          *(unsigned long *)(v10 + 8) = sub_23b60(v7);
        }
        else {
          v1[1] = sub_4c7a0(v2);
        }
      }
      sub_24c80(v3);
      return *(int8 *)(dat_845f0 + 8 + v11);
    }
  }
  else {
    v10 = dat_84588;
    if ((!dat_84588) && (v10 = sub_4c7c0(), dat_84588 = v10, !v10))
      sub_32490("getcwd",".");
  }
  return v10;
}


// Function: sub_25340 @ 0x25340
char * sub_25340(unsigned long a0,char *a1)
{
  char *v1; // rax
  uint8 v2; // rax
  char *v3; // rax
  
  if (*a1 != '/') { // branch-flip
    v1 = (char *)sub_251a0(a0);
    v2 = strlen(v1);
    v3 = (char *)sub_4c5b0(v2 + 2 + strlen(a1));
    strcpy(v3,v1);
    v3[v2] = '/';
    strcpy(&v3[v2 + 1],a1);
  }
  else {
    v3 = (char *)sub_4c7a0(a1);
  }
  sub_23ea0(v3);
  return v3;
}


// Function: sub_253c0 @ 0x253c0
int8 sub_253c0(char *a0,int4 a1) // return-dupe x2
{
  int4 v1; // eax
  DIR *v2; // rax
  int8 v3; // rax
  
  v1 = __openat_2(dat_81234,a0,dat_84808 | 0x10000);
  if (v1 <= -1) {
    if ((!a1) && (*__errno_location() == 2))
      return 0;
    sub_327c0(a0);
    return 0;
  }
  v2 = fdopendir(v1);
  if (!v2) {
    sub_32ac0(a0);
    if (!close(v1))
      return 0;
    sub_32ac0(a0);
    return 0;
  }
  v3 = sub_4b0d0(v2,dat_847e4);
  if (!v3)
    sub_32ac0(a0);
  if (closedir(v2)) {
    sub_32ac0(a0);
    return v3;
  }
  return v3;
}


// Function: sub_25490 @ 0x25490
uint4 sub_25490(char *a0,int4 a1) // return-dupe
{
  char *v1;
  uint4 v2;
  int4 v3; // eax
  uint4 *v4; // rax
  char *v5; // rax
  uint8 v6;
  void *v7; // rax
  
  if (!unlinkat(dat_81234,a0,0))
    return 1;
  v4 = (uint4 *)__errno_location();
  if ((*v4 != 1) && (*v4 != 0x15))
    return 0;
  if (!sub_24560(a0))
    return 1;
  if (*v4 <= 0x27) {
    if (!(0x8000020001U >> ((uint8)*v4 & 0x3f) & 1))
      return 0;
    if (a1 == 1) {
      v5 = (char *)sub_253c0(a0,0);
      if (!v5)
        return 0;
      v6 = strlen(v5);
      v1 = v5;
      while( true ) {
        if (!v6) {
          free(v5);
          return (uint4)(sub_24560(a0) == 0);
        }
        v7 = (void *)sub_27bc0(a0,v1);
        v3 = sub_25490(v7,1);
        v2 = *v4;
        free(v7);
        if (!v3) break;
        v1 = &v1[v6 + 1];
        v6 = strlen(v1);
      }
      free(v5);
      *v4 = v2;
      return 0;
    }
    return -(uint4)(a1 == 2);
  }
  return 0;
}


// Function: sub_255e0 @ 0x255e0
int8 * sub_255e0(int4 a0)
{
  int8 *v1; // rax
  
  for (v1 = (int8 *)0x81280; ((*v1 || ((int4)v1[1])) || (v1[4])); v1 = &v1[6]) {
    if (a0 == (int4)v1[1])
      return v1;
  }
  return NULL;
}


// Function: sub_25610 @ 0x25610
unsigned long sub_25610(int4 a0) // early-return
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_255e0(a0);
  if (!v1)
    return 0;
  return *v1;
}


// Function: sub_25620 @ 0x25620
bool sub_25620(int4 a0)
{
  return sub_255e0(a0) != 0;
}


// Function: sub_25630 @ 0x25630
void sub_25630(void)
{
  int8 v1;
  
  if ((dat_846b0) && (v1 = *(int8 *)(dat_846b0 + 8), *(int8 *)(dat_846b0 + 8))) {
    do {
      dat_846b0 = v1;
      v1 = *(int8 *)(dat_846b0 + 8);
    } while (*(int8 *)(dat_846b0 + 8));
  }
  return;
}


// Function: sub_25660 @ 0x25660
void sub_25660(struct_101 *a0) // return-dupe
{
  a0->field_0x0 = 0;
  a0->field_0x8 = &dat_84628->field_0x0;
  if (!dat_84628) {
    dat_84628 = a0;
    return;
  }
  *(struct_101 **)dat_84628 = a0;
  dat_84628 = a0;
  return;
}


// Function: sub_25690 @ 0x25690
unsigned long ** sub_25690(unsigned long *a0,int4 a1,code *a2)
{
  unsigned long v1;
  int4 v10; // r8d
  unsigned long **v11;
  unsigned long **v2;
  unsigned long **v3;
  unsigned long *v4;
  int4 v5;
  unsigned long **v6; // rax
  unsigned long **v7;
  unsigned long *v8; // stack - 0x38
  unsigned long **v9;
  
  if (a1 != 1) {
    if (a1 != 2) {
      v10 = (a1 + 1) / 2;
      v4 = a0;
      v5 = v10;
      while (v5 = v5 + -1, v5) {
        v4 = (unsigned long *)*v4;
      }
      v1 = *v4;
      *v4 = 0;
      v6 = (unsigned long **)sub_25690(a0,v10,a2);
      v9 = v6;
      v6 = (unsigned long **)sub_25690(v1,a1 / 2,a2);
      v7 = v6;
      if ((v9) && (v6)) {
        v11 = &v8;
        do {
          v5 = (*a2)(v9,v7);
          if (0 <= v5) { // branch-flip
            v2 = (unsigned long **)*v7;
            *v11 = v7;
            v11 = v7;
          }
          else {
            v3 = (unsigned long **)*v9;
            *v11 = v9;
            v2 = v7;
            v11 = v9;
            v9 = v3;
          }
          v7 = v2;
          if (!v9) break;
        } while (v7);
      }
      else {
        v11 = &v8;
      }
      v6 = v11;
      if (!v9)
        v9 = v7;
      *v6 = v9;
      v6 = (unsigned long **)v8;
      return v6;
    }
    v5 = (*a2)(a0,*a0);
    if (0 < v5) {
      v6 = (unsigned long *)*a0;
      *v6 = a0;
      *a0 = 0;
      return v6;
    }
  }
  v6 = (unsigned long **)a0;
  return v6;
}


// Function: sub_25800 @ 0x25800
void sub_25800(unsigned long *a0,int4 a1,code *a2) // return-dupe
{
  int8 *v1;
  int8 *v2; // rdx
  int8 *v3;
  
  v1 = (int8 *)sub_25690(a0,a1,a2);
  if (!v1)
    return;
  v3 = NULL;
  while( true ) {
    v2 = v1;
    v2[1] = (int8)v3;
    if (!(int8 *)*v2) break;
    v1 = (int8 *)*v2;
    v3 = v2;
  }
  return;
}


// Function: sub_25830 @ 0x25830
char * sub_25830(void)
{
  unsigned long *v1;
  char *v2; // rax
  
  v1 = dat_846b0;
  while( true ) {
    if (!v1) {
      v2 = dcgettext(NULL,"command line",5); // tail-call
      return v2;
    }
    if ((*(int4 *)&v1[2] == 2) && (v1[6])) break;
    v1 = (unsigned long *)*v1;
  }
  return (char *)v1[3];
}


// Function: sub_25880 @ 0x25880
void sub_25880(void)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_4c5b0(0x38);
  if (!dat_846b0) {
    *v2 = 0;
    v2[1] = 0;
    *(unsigned int *)&v2[2] = 3;
    dat_846b0 = v2;
    v2 = (unsigned long *)sub_4c5b0(0x38);
  }
  v1 = (unsigned long *)dat_846b0[1];
  v2[1] = v1;
  if (v1)
    *v1 = v2;
  *v2 = dat_846b0;
  dat_846b0[1] = v2;
  return;
}


// Function: sub_258f0 @ 0x258f0
void sub_258f0(unsigned long a0)
{
  int8 v1; // rax
  
  v1 = sub_25880();
  *(unsigned int *)(v1 + 0x10) = 1;
  *(unsigned long *)(v1 + 0x18) = a0;
  return;
}


// Function: sub_25910 @ 0x25910
void sub_25910(unsigned long a0)
{
  int8 v1; // rax
  
  v1 = sub_25880();
  dat_84630 = 2;
  *(unsigned int *)(v1 + 0x10) = 2;
  *(unsigned long *)(v1 + 0x18) = a0;
  *(unsigned long *)(v1 + 0x20) = 0;
  *(unsigned long *)(v1 + 0x30) = 0;
  return;
}


// Function: sub_25940 @ 0x25940
void sub_25940(unsigned int a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_25880();
  *(unsigned int *)(v1 + 0x10) = 4;
  *(unsigned int *)(v1 + 0x18) = a0;
  *(unsigned long *)(v1 + 0x20) = a1;
  return;
}


// Function: sub_25970 @ 0x25970
void sub_25970(int8 a0,int8 a1)
{
  if (dat_84b38) { // branch-flip
    if (dat_84b38 <= *(uint8 *)(a1 + 0x30) != dat_84b38 <= *(uint8 *)(a0 + 0x30))
      return;
  }
  else if ((*(uint8 *)(a1 + 0x30) != 0) != (*(uint8 *)(a0 + 0x30) != 0))
    return;
  strcmp(*(char **)(a0 + 0x10),*(char **)(a1 + 0x10)); // tail-call
  return;
}


// Function: sub_259d0 @ 0x259d0
unsigned int sub_259d0(int8 a0,int8 a1)
{
  uint4 v1; // eax
  
  v1 = strcmp(*(char **)(a0 + 0x58),*(char **)(a1 + 0x58));
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_259f0 @ 0x259f0
void sub_259f0(int8 a0,int8 a1)
{
  strcmp(*(char **)(a0 + 0x10),*(char **)(a1 + 0x10)); // tail-call
  return;
}


// Function: sub_25a10 @ 0x25a10
void sub_25a10(void)
{
  void *v1;
  
  if (!dat_84628)
    return;
  do {
    v1 = *(void **)((int8)dat_84628 + 8);
    free(dat_84628);
  } while (dat_84628);
  return;
}


// Function: sub_25a50 @ 0x25a50
void sub_25a50(void)
{
  int4 v1;
  int8 *v2;
  
  v2 = dat_846b0;
  dat_846b0 = (int8)*dat_846b0;
  if (dat_846b0)
    *(unsigned long *)((int8)dat_846b0 + 8) = 0;
  v1 = (int4)v2[2];
  if ((v1 != 4) && (v1 != 1)) {
    if (v1 == 3) {
      free(v2); // tail-call
      return;
    }
    sub_25a10();
    free(v2); // tail-call
    return;
  }
  if ((dat_84ba8 != 3) && (dat_84ba8 != 8))
    return;
  sub_25660(v2); // tail-call
  return;
}


// Function: sub_25ad0 @ 0x25ad0
void sub_25ad0(void *a0)
{
  if (a0) {
    free(*(void **)((int8)a0 + 0x10));
    free(*(void **)((int8)a0 + 0x58));
    free(a0); // tail-call
    return;
  }
  return;
}


// Function: sub_25b00 @ 0x25b00
unsigned int sub_25b00(char *a0)
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  char *v4; // rax
  unsigned long *v5;
  stat v6; // stack - 0xc8
  
  if (stat(a0,&v6))
    sub_32d10(a0); // no-return
  v3 = (char *)sub_25830();
  v5 = dat_84698;
  while( true ) {
    if (!v5) {
      v5 = (unsigned long *)sub_4c5b0(0x20);
      v5[3] = v3;
      *v5 = dat_84698;
      v5[1] = v6._8_8_;
      v5[2] = v6._0_8_;
      dat_84698 = v5;
      return 0;
    }
    if ((v5[1] == v6._8_8_) && (v5[2] == v6._0_8_)) break;
    v5 = (unsigned long *)*v5;
  }
  v2 = sub_4a5b0(0,0x3a,1);
  if (dat_84d50)
    (*dat_84d50)();
  v1 = (char *)v5[3];
  v4 = (char *)sub_4a870(0,a0);
  error(0,0,dcgettext(NULL,"%s: file list requested from %s already read from %s",5),v4,v3,v1);
  dat_84d58 = 2;
  sub_4a5b0(0,0x3a,v2);
  return 1;
}


// Function: sub_25c70 @ 0x25c70
void sub_25c70(char *a0) // return-dupe x2
{
  uint8 v1; // rax
  char *v2; // rax
  char *v3;
  
  v1 = strlen(a0);
  v3 = &a0[v1 - 1];
  if (&a0[v1 - 1] <= a0)
    return;
  do {
    if (*v3 != '/')
      return;
    v2 = &v3[-1];
    *v3 = '\0';
    v3 = v2;
  } while (a0 != v2);
  return;
}


// Function: sub_25ca0 @ 0x25ca0
uint1 sub_25ca0(int8 a0)
{
  int4 v1;
  FILE *v2;
  int4 v3; // eax
  int8 v4;
  uint1 v5; // r8b
  int8 v6; // rbp
  bool v7;
  bool v8; // bpl
  
  *(int8 *)(a0 + 0x20) = *(int8 *)(a0 + 0x20) + 1;
  v2 = *(FILE **)(a0 + 0x30);
  v1 = *(int4 *)(a0 + 0x28);
  v3 = getc_unlocked(v2);
  v7 = v3 != -1;
  if ((v1 == v3) || (!v7)) {
    v8 = 1;
    v6 = 0;
  }
  else {
    v6 = 0;
    do {
      if (dat_846a0 != v6) // branch-flip
        *(char *)(dat_846a8 + v6) = (char)v3;
      else {
        dat_846a8 = sub_4c710(dat_846a8,(uint8 *)0x846a0);
        *(char *)(dat_846a8 + v6) = (char)v3;
      }
      if (!v3)
        return 2;
      v6 += 1;
      v3 = getc_unlocked(v2);
    } while ((v1 != v3) && (v3 != -1));
    v8 = v6 == 0;
    v7 = v3 != -1 && v8;
  }
  v5 = 3;
  if (!v7) {
    v4 = dat_846a8;
    if (dat_846a0 == v6) {
      v4 = sub_4c710(dat_846a8,(uint8 *)0x846a0);
      dat_846a8 = v4;
    }
    *(char *)(v4 + v6) = 0;
    sub_25c70(v4);
    v5 = v3 == -1 & v8;
  }
  return v5;
}


// Function: sub_25dc0 @ 0x25dc0
unsigned int sub_25dc0(char *a0,int8 a1) // return-dupe
{
  int8 *v1;
  int8 *v10; // stack - 0x110
  int8 v11; // stack - 0x108
  int8 v2; // rax
  char *v3; // rax
  char v4;
  unsigned int v5 [2]; // stack - 0x138
  int4 v6; // stack - 0x118
  unsigned long v7; // stack - 0x130
  unsigned long v8; // stack - 0x128
  unsigned int v9; // stack - 0x114
  
  v4 = *a0;
  if (!v4)
    return 1;
  v1 = __ctype_b_loc();
  while (*(uint1 *)(*v1 + 1 + (int8)v4 * 2) & 0x20) {
    v4 = a0[1];
    a0 = &a0[1];
    if (!v4)
      return 1;
  }
  if (v4 != '-')
    return 1;
  v11 = 1;
  if (sub_382c0(a0,&v6,0x2000e46)) {
    if (dat_84d50)
      (*dat_84d50)();
    v3 = (char *)sub_354b0(&v6);
    error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),a0,v3);
    sub_163d0(); // no-return
  }
  *v10 = dat_84dd0;
  v7 = *(unsigned long *)(a1 + 0x18);
  v5[0] = 2;
  v8 = *(unsigned long *)(a1 + 0x20);
  sub_2f5b0((int4)v11 + v6,v10,v5);
  v2 = CONCAT44(v9,v6) + v11;
  if (v2) {
    v1 = &v10[v2];
    do {
      *v10 = 0;
      v10 = &v10[1];
    } while (v1 != v10);
  }
  sub_35420(&v6);
  return 0;
}


// Function: sub_25f40 @ 0x25f40
void sub_25f40(int8 *a0,unsigned int a1) // return-dupe
{
  char *v1;
  
  v1 = (char *)*a0;
  if (v1) {
    do {
      a0 = &a0[1];
      sub_3ee30(dat_84b60,v1,a1);
      v1 = (char *)*a0;
    } while (v1);
    return;
  }
  return;
}


// Function: sub_25f80 @ 0x25f80
void sub_25f80(int4 a0,char *a1)
{
  int4 v1;
  unsigned long v2; // rsi
  
  switch(a0) {
    case 0x58:
      v1 = sub_3f2a0(sub_3ee30,dat_84b60,a1,dat_846f8 | dat_84ad4 | (uint4)(dat_846fc != 1) << 0x1c,10);
      if (v1) {
        v1 = *__errno_location();
        if (dat_84d50)
          (*dat_84d50)();
        error(0,v1,"%s",sub_4aa70(a1));
        sub_163d0(); // no-return
      }
      break;
    default:
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,"unhandled positional option %d",a0);
      sub_163d0(); // no-return
    case 0x101:
      sub_25f40((int8 *)0x7f6a0,0x10000000); // tail-call
      return;
    case 0x102:
      v2 = 1;
      sub_12a70("CACHEDIR.TAG",v2,sub_12b70); // return-dupe, tail-call
      return;
    case 0x103:
      v2 = 2;
      sub_12a70("CACHEDIR.TAG",v2,sub_12b70);
      return;
    case 0x104:
      v2 = 3;
      sub_12a70("CACHEDIR.TAG",v2,sub_12b70);
      return;
    case 0x105:
      sub_3ee30(dat_84b60,a1,dat_846f8 | dat_84ad4 | (uint4)(dat_846fc != 1) << 0x1c); // tail-call
      return;
    case 0x106:
      v2 = 2;
      sub_16820(a1,v2); // return-dupe, tail-call
      return;
    case 0x107:
      v2 = 1;
      sub_16820(a1,v2);
      return;
    case 0x108:
      v2 = 1;
      sub_12a70(a1,v2,0); // return-dupe, tail-call
      return;
    case 0x109:
      v2 = 2;
      sub_12a70(a1,v2,0);
      return;
    case 0x10a:
      v2 = 3;
      sub_12a70(a1,v2,0);
      return;
    case 0x10b:
      sub_25f40((int8 *)0x7f6c0,0); // tail-call
      return;
    case 0x10c:
      sub_16c40(); // tail-call
      return;
    case 0x10d:
      dat_846f8 |= 0x10;
      break;
    case 0x10e:
      dat_846f8 &= 0xffffffef;
      break;
    case 0x10f:
      dat_846f8 |= 0x40000000;
      break;
    case 0x110:
      dat_846f8 &= 0xbfffffff;
      dat_81238 = 0;
      break;
    case 0x111:
      dat_84ad4 = 8;
      break;
    case 0x112:
      dat_84ad4 = 0;
      break;
    case 0x113:
      dat_847e8 = 1;
      break;
    case 0x114:
      dat_847e8 = 0;
      break;
    case 0x115:
      dat_84700 = 0;
      break;
    case 0x116:
      dat_846f8 |= 1;
      break;
    case 0x117:
      dat_846fc = 1;
      break;
    case 0x118:
      dat_8123c = 0;
      dat_84700 = 1;
      break;
    case 0x119:
      dat_8123c = 10;
      dat_84700 = 0;
      break;
    case 0x11a:
      dat_84700 = 1;
      break;
    case 0x11b:
      dat_846f8 &= 0xfffffffe;
      break;
    case 0x11c:
      dat_846fc = 2;
    
  }
  return;
}


// Function: sub_26210 @ 0x26210
void sub_26210(void)
{
  int8 *v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  int8 *v4;
  char *v5;
  
  if (!dat_84628)
    return;
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly.",5));
  v1 = dat_84628;
  do {
    v4 = v1;
    v1 = (int8 *)v4[1];
  } while ((int8 *)v4[1]);
  do {
    while( true ) {
      dat_84d58 = 2;
      if ((int4)v4[2] == 1) break;
      if ((int4)v4[2] == 4) {
        v5 = (char *)v4[4];
        if (!v5) {
          if (dat_84d50)
            (*dat_84d50)();
          v5 = "--%s has no effect";
          v3 = sub_25610(*(unsigned int *)&v4[3]);
          goto label_2632e;
        }
        if (dat_84d50) {
          (*dat_84d50)();
          v5 = (char *)v4[4];
        }
        v3 = sub_4ac50(v5);
        v5 = "--%s %s has no effect";
        v2 = sub_25610(*(unsigned int *)&v4[3]);
        error(0,0,dcgettext(NULL,v5,5),v2,v3);
      }
      v4 = (int8 *)*v4;
      if (!v4) {
        dat_84d58 = 2; // return-dupe
        sub_25a10(); // tail-call
        return;
      }
    }
    if (dat_84d50)
      (*dat_84d50)();
    v3 = sub_4ac50((char *)v4[3]);
    v5 = "-C %s has no effect";
label_2632e:
    error(0,0,dcgettext(NULL,v5,5),v3);
    v4 = (int8 *)*v4;
  } while (v4);
  dat_84d58 = 2;
  sub_25a10();
  return;
}


// Function: sub_263a0 @ 0x263a0
int8 sub_263a0(char *a0)
{
  int8 v1; // rax
  char *v2; // rax
  
  v1 = sub_4c760(0x60);
  if (!a0)
    a0 = "";
  v2 = (char *)sub_4c7a0(a0);
  *(char **)(v1 + 0x10) = v2;
  *(uint8 *)(v1 + 0x18) = strlen(v2);
  return v1;
}


// Function: sub_263f0 @ 0x263f0
unsigned int sub_263f0(char *a0)
{
  if (!dat_846fc) {
    if (sub_3ead0(a0,0)) {
      dat_84634 = 1;
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Pattern matching characters used in file names",5));
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning",5));
    }
    return dat_84634;
  }
  return dat_84634;
}


// Function: sub_26490 @ 0x26490
uint8 sub_26490(int8 a0,uint8 a1)
{
  uint1 *v1;
  
  v1 = *(uint1 **)(a0 + 0x58);
  return sub_41ac0(v1,a1); // tail-call
}


// Function: sub_267d0 @ 0x267d0
unsigned long sub_267d0(int4 a0) // return-dupe
{
  int8 v1;
  int4 v2; // eax
  char *v3; // rax
  
  do {
    v1 = dat_846b0;
    if (!dat_846b0) {
      sub_26210();
      return 0;
    }
    switch(*(unsigned int *)(dat_846b0 + 0x10)) {
      case 0:
label_26820:
        sub_26770(*(unsigned long *)(dat_846b0 + 0x18));
        if (dat_847e8)
          sub_23c70(dat_846a8);
        dat_84670 = *(unsigned int *)(v1 + 0x10);
        dat_84678 = dat_846a8;
        sub_25a50();
        return 0x84660;
      case 1:
        if (!a0) goto label_26820;
        v3 = (char *)sub_4c7a0(*(char **)(dat_846b0 + 0x18));
        sub_24c80(sub_24b00(v3));
        sub_25a50();
        break;
      case 2:
        v2 = sub_26500(dat_846b0);
        if (!v2)
          return 0x84660;
        break;
      case 4:
        sub_25f80(*(unsigned int *)(dat_846b0 + 0x18),*(unsigned long *)(dat_846b0 + 0x20));
        goto label_26817;
      case 3:
label_26817:
        sub_25a50();
      
    }
  } while( true );
}


// Function: sub_268d0 @ 0x268d0
void sub_268d0(uint4 a0,unsigned long *a1)
{
  passwd *v1; // rax
  
  if ((a0) && (dat_846cc == a0)) {
label_26946:
    *a1 = sub_4c7a0("");
    return;
  }
  if ((!dat_846f0) || (dat_846e4 != a0)) {
    v1 = getpwuid(a0);
    if (!v1) {
      dat_846cc = a0;
      goto label_26946;
    }
    dat_846e4 = a0;
    sub_23be0((unsigned long *)0x846f0,*(char **)v1);
  }
  *a1 = sub_4c7a0(dat_846f0);
  return;
}


// Function: sub_26960 @ 0x26960
void sub_26960(uint4 a0,unsigned long *a1)
{
  group *v1; // rax
  
  if ((a0) && (dat_846c8 == a0)) {
label_269d6:
    *a1 = sub_4c7a0("");
    return;
  }
  if ((!dat_846e8) || (dat_846e0 != a0)) {
    v1 = getgrgid(a0);
    if (!v1) {
      dat_846c8 = a0;
      goto label_269d6;
    }
    dat_846e0 = a0;
    sub_23be0((unsigned long *)0x846e8,*(char **)v1);
  }
  *a1 = sub_4c7a0(dat_846e8);
  return;
}


// Function: sub_269f0 @ 0x269f0
unsigned long sub_269f0(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  passwd *v2;
  
  if (dat_846d8) {
    v1 = strcmp(a0,dat_846d8);
    if (!v1)
      return CONCAT44(dat_4,v1);
  }
  if ((dat_846f0) && (*a0 == *dat_846f0)) {
    if (!strcmp(a0,dat_846f0)) goto label_26a4e;
    v2 = getpwnam(a0);
  }
  else {
    v2 = getpwnam(a0);
  }
  if (!v2) {
    sub_23be0((unsigned long *)0x846d8,a0);
    return 0;
  }
  dat_846e4 = *(unsigned int *)&v2->field_0x10;
  sub_23be0((unsigned long *)0x846f0,*(char **)v2);
label_26a4e:
  *a1 = dat_846e4;
  return 1;
}


// Function: sub_26aa0 @ 0x26aa0
unsigned long sub_26aa0(char *a0,unsigned int *a1)
{
  int4 v1; // eax
  group *v2;
  
  if (dat_846d0) {
    v1 = strcmp(a0,dat_846d0);
    if (!v1)
      return CONCAT44(dat_4,v1);
  }
  if ((dat_846e8) && (*a0 == *dat_846e8)) {
    if (!strcmp(a0,dat_846e8)) goto label_26afe;
    v2 = getgrnam(a0);
  }
  else {
    v2 = getgrnam(a0);
  }
  if (!v2) {
    sub_23be0((unsigned long *)0x846d0,a0);
    return 0;
  }
  dat_846e0 = *(unsigned int *)&v2->field_0x10;
  sub_23be0((unsigned long *)0x846e8,a0);
label_26afe:
  *a1 = dat_846e0;
  return 1;
}


// Function: sub_26b50 @ 0x26b50
void sub_26b50(unsigned long a0) // return-dupe
{
  int8 v1; // rax
  
  v1 = sub_25880();
  *(unsigned int *)(v1 + 0x10) = 0;
  *(unsigned long *)(v1 + 0x18) = a0;
  if (!dat_84630) {
    dat_84630 = 1;
    return;
  }
  if (dat_84630 != 1)
    return;
  dat_84630 = 2;
  return;
}


// Function: sub_26ba0 @ 0x26ba0
unsigned long sub_26ba0(unsigned int a0,unsigned long a1,int8 a2) // return-dupe x3
{
  int4 *v1;
  uint8 v2;
  char *v3;
  
  switch(a0) {
    default:
      if (!sub_25620())
        return 7;
      sub_25940();
      return 0;
    case 0x43:
      sub_258f0(a1);
      return 0;
    case 0x54:
      sub_25910(a1);
      return 0;
    case 0x100:
      sub_26b50(a1);
      return 0;
    case 0x1000005:
      goto label_26be8;
    
  }
label_26be8:
  v1 = (int4 *)**(unsigned long **)(a2 + 0x28);
  if (*v1 != 2)
    return 7;
  v2 = *(uint8 *)&v1[4];
  v3 = *(char **)&v1[2];
  error(0,0,dcgettext(NULL,"%s:%lu: unrecognized option",5),v3,v2);
  sub_2f7d0(2);
  return 7;
}


// Function: sub_26c90 @ 0x26c90
void sub_26c90(void)
{
  dat_846a8 = sub_4c5b0(0x66);
  dat_846a0 = 100;
  sub_25630(); // tail-call
  return;
}


// Function: sub_26cc0 @ 0x26cc0
void sub_26cc0(void)
{
  free(dat_846a8); // tail-call
  return;
}


// Function: sub_26cd0 @ 0x26cd0
unsigned long sub_26cd0(int4 a0) // early-return
{
  int8 v1; // rax
  
  v1 = sub_267d0(a0);
  if (!v1)
    return 0;
  return *(unsigned long *)(v1 + 0x18);
}


// Function: sub_26cf0 @ 0x26cf0
void sub_26cf0(char *a0,unsigned int a1,char a2,unsigned long a3)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_263a0(a0);
  *v1 = 0;
  *(unsigned int *)&v1[5] = a1;
  v1[6] = 0;
  v1[1] = dat_846b8;
  *(uint4 *)&v1[4] = dat_81238 | dat_846f8 | dat_84ad4 | (uint4)(dat_846fc == 2) << 0x1c;
  v1[7] = 0;
  v1[8] = a3;
  *(char *)((int8)v1 + 0x24) = a2;
  if (dat_846b8) {
    *dat_846b8 = v1;
    dat_846b8 = v1;
    return;
  }
  dat_846c0 = v1;
  dat_846b8 = v1;
  return;
}


// Function: sub_26d90 @ 0x26d90
void sub_26d90(void) // return-dupe
{
  char *v1;
  int4 v2;
  int8 v3; // rax
  int4 v4;
  
  v2 = 0;
  v4 = v2;
  if (dat_84aac) { // branch-flip
    while (v3 = sub_267d0(0), v3) {
      if (*(int4 *)(v3 + 0x10) != 1) {
        sub_25ad0(dat_84638);
        dat_84638 = (unsigned long *)sub_263a0(*(char **)(v3 + 0x18));
        *dat_84638 = 0;
        *(int4 *)&dat_84638[5] = dat_84640;
        dat_84638[6] = 0;
        dat_84638[7] = 0;
        dat_84638[8] = 0;
        *(char *)((int8)dat_84638 + 0x24) = 1;
        *(uint4 *)&dat_84638[4] = dat_81238 | dat_846f8 | dat_84ad4 | (uint4)(dat_846fc == 2) << 0x1c;
        dat_846b8 = dat_84638;
        dat_846c0 = dat_84638;
        return;
      }
      v1 = *(char **)(v3 + 0x18);
      dat_84640 = sub_24b00((char *)sub_4c7a0(v1));
    }
    v2 = dat_84640;
    if (!dat_84640)
      return;
  }
  else {
    while (v3 = sub_267d0(0), v3) {
      v1 = *(char **)(v3 + 0x18);
      if (*(int4 *)(v3 + 0x10) != 1) { // branch-flip
        sub_26cf0(v1,v2,1,0);
        v4 = v2;
      }
      else {
        v2 = sub_24b00((char *)sub_4c7a0(v1));
      }
    }
    if (v4 == v2)
      return;
  }
  sub_26cf0(NULL,v2,0,0); // tail-call
  return;
}


// Function: sub_26ee0 @ 0x26ee0
void sub_26ee0(int8 a0,int8 a1) // return-dupe
{
  unsigned int v1;
  char v10;
  uint8 v11; // stack - 0x238
  int8 v12; // stack - 0x230
  int8 v13; // stack - 0x218
  int8 v14; // stack - 0x60
  int4 v15; // stack - 0x50
  int4 v2;
  int8 v3; // rax
  char *v4;
  char *v5; // rax
  uint8 v6;
  unsigned long v7 [11]; // stack - 0x1f8
  uint8 v8; // stack - 0x200
  stat v9 [2];
  
  v3 = sub_1f610(a0);
  *(int8 *)(a1 + 0x38) = v3;
  v4 = (char *)sub_1fa60(v3);
  if (!v4)
    return;
  v6 = *(uint8 *)(a1 + 0x18);
  v8 = v6 + 0x66;
  if (v6 <= 99)
    v8 = 0x66;
  v5 = (char *)sub_4c5b0();
  v1 = *(unsigned int *)(a1 + 0x28);
  strcpy(v5,*(char **)(a1 + 0x10));
  v11 = v6;
  if (v5[v6 - 1] != '/') {
    v11 = v6 + 1;
    v5[v6] = '/';
    v5[v6 + 1] = '\0';
  }
  v12 = 0;
  v10 = *v4;
  if (v10) {
    v13 = 0;
    do {
      v6 = strlen(v4);
      if (v10 == 'D') {
        if (v8 < v11 + v6) {
          do {
            v5 = (char *)sub_4c710(v5,&v8);
          } while (v8 < v11 + v6);
        }
        strcpy(&v5[v11],&v4[1]);
        v3 = sub_26cf0(v5,v1,0,a1);
        if (v12) // branch-flip
          *(int8 *)(v13 + 0x50) = v3;
        else {
          v12 = v3;
        }
        sub_2f630(v7);
        v2 = *(int4 *)(a0 + 0x1a8);
        v14 = a0;
        if (0 <= v2) { // branch-flip
          v2 = sub_140d0(a0,&v4[1],dat_84808 | 0x10000);
          if (v2 < 0) goto label_270a5;
          v15 = v2;
          if (fstat(v2,v9)) // branch-flip
            sub_24ec0(v5);
          else {
            v7[0] = sub_4c7a0(v5);
            sub_26ee0(v7,v3);
            sub_14200(v7);
          }
        }
        else {
          *__errno_location() = -v2;
label_270a5:
          sub_24dd0(v5);
        }
        sub_2f6f0(v7);
        v13 = v3;
      }
      v4 = &v4[v6 + 1];
      v10 = *v4;
    } while (v10);
  }
  free(v5);
  *(int8 *)(a1 + 0x48) = v12;
  return;
}


// Function: sub_27150 @ 0x27150
void sub_27150(struct_45 *a0)
{
  int8 *v1;
  int8 v2;
  int8 v3;
  
  v1 = a0->field_0x8;
  v2 = a0->field_0x0;
  v3 = v2;
  if (v1) {
    *v1 = v2;
    v3 = dat_846c0;
  }
  if (!v2) {
    dat_846c0 = v3;
    dat_846b8 = v1;
    return;
  }
  dat_846c0 = v3;
  *(int8 **)(v2 + 8) = v1;
  return;
}


// Function: sub_27190 @ 0x27190
void sub_27190(char *a0)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_263a0(a0);
  if (dat_84a78) {
    sub_27150(dat_846c0);
    sub_25ad0();
  }
  v1 = dat_846c0;
  v2[1] = 0;
  dat_846c0 = v2;
  *v2 = v1;
  if (!dat_846b8)
    dat_846b8 = v2;
  v2[6] = 0;
  *(unsigned int *)&v2[5] = 0;
  v2[7] = 0;
  v2[8] = 0;
  *(char *)((int8)v2 + 0x24) = 1;
  *(uint4 *)&v2[4] = dat_81238 | dat_846f8 | dat_84ad4 | (uint4)(dat_846fc == 2) << 0x1c;
  dat_84a78 = 1;
  return;
}


// Function: sub_27250 @ 0x27250
unsigned long sub_27250(char *a0) // early-return
{
  int8 v1; // rax
  bool v2; // zf
  
  do {
    if (!dat_846c0)
      return 1;
    if (!**(char **)(dat_846c0 + 0x10)) {
      sub_24c80(*(unsigned int *)(dat_846c0 + 0x28));
      dat_846c0 = 0;
      dat_846b8 = 0;
      return 1;
    }
    v1 = sub_264a0(a0);
    if (dat_84a78) { // branch-flip
      if (dat_846c0 == v1) {
        dat_84a78 = '\0';
        goto label_2732a;
      }
    }
    else {
label_2732a:
      if (v1) {
        if (((a0[*(int8 *)(v1 + 0x18)] != '/') || (!dat_84ad4)) || (!*(int8 *)(v1 + 0x30)))
          *(int8 *)(v1 + 0x30) = *(int8 *)(v1 + 0x30) + 1;
        sub_24c80(*(unsigned int *)(v1 + 0x28));
        if (dat_84b38) // branch-flip
          v2 = *(int8 *)(v1 + 0x30) == dat_84b38;
        else {
          v2 = *(int8 *)(v1 + 0x30) != 0;
        }
        return CONCAT71((undefined7)((uint8)v1 >> 8),v2);
      }
    }
    if (((!dat_84aac) || (!*(int8 *)(dat_846c0 + 0x30))) || (sub_26d90(), *(int8 *)(dat_846c0 + 0x30)))
      return 0;
  } while( true );
}


// Function: sub_273a0 @ 0x273a0
unsigned long sub_273a0(int8 a0) // return-dupe
{
  char *v1;
  unsigned long *v2;
  uint8 v3;
  uint8 v4; // rax
  
  v3 = dat_84b38;
  v1 = *(char **)(a0 + 8);
  if (!v1)
    return 0;
  if (!dat_84b38)
    return 0;
  if (*(char *)(a0 + 0x10))
    return 0;
  v4 = strlen(v1);
  v2 = dat_846c0;
  while( true ) {
    if (!v2)
      return 1;
    if ((*(char *)v2[2]) && ((uint8)v2[6] < v3)) break;
    if (((uint8)v2[3] <= v4) && (v1[v2[3]] == '/'))
      return 0;
    v2 = (unsigned long *)*v2;
  }
  return 0;
}


// Function: sub_27430 @ 0x27430
void sub_27430(void)
{
  char v1;
  unsigned long *v2;
  unsigned long v3; // rax
  char *v4;
  
  v2 = dat_846c0;
label_27456:
  do {
    if (!v2) {
      dat_846c0 = NULL;
      dat_846b8 = 0;
      if (!dat_84aac)
        return;
      while (v4 = (char *)sub_26cd0(1), v4) {
        sub_263f0(v4);
        if (dat_84d50)
          (*dat_84d50)();
        v3 = sub_4aa70(v4);
        error(0,0,dcgettext(NULL,"%s: Not found in archive",5),v3);
        dat_84d58 = 2;
      }
      return;
    }
    if (dat_84b38) { // branch-flip
      if ((uint8)v2[6] < dat_84b38) {
        v4 = (char *)v2[2];
        v1 = *v4;
        goto label_27492;
      }
    }
    else if (!v2[6]) {
      v4 = (char *)v2[2];
      v1 = *v4;
label_27492:
      if (v1) {
        sub_263f0(v4);
        if (dat_84d50)
          (*dat_84d50)();
        v3 = sub_4aa70((char *)v2[2]);
        if (v2[6]) // branch-flip
          v4 = dcgettext(NULL,"%s: Required occurrence not found in archive",5);
        else {
          v4 = dcgettext(NULL,"%s: Not found in archive",5);
        }
        error(0,0,v4,v3);
        v2 = (unsigned long *)*v2;
        dat_84d58 = 2;
        goto label_27456;
      }
    }
    v2 = (unsigned long *)*v2;
  } while( true );
}


// Function: sub_275b0 @ 0x275b0
void sub_275b0(void) // return-dupe x2
{
  int8 *v1;
  char *v2;
  
  v1 = dat_846c0;
  if (!dat_846c0)
    return;
  do {
    if (dat_84b38) { // branch-flip
      if (dat_84b38 <= (uint8)v1[6])
        return;
    }
    else if (v1[6])
      return;
    v1 = (int8 *)*v1;
  } while (v1);
  if (dat_84a54)
    error(0,0,dcgettext(NULL,"Archive label mismatch",5));
  sub_2f7d0(1);
  v1 = dat_846c0;
  while ((v1 && (v2 = (char *)v1[2], !sub_263f0(v2)))) {
    v1 = (int8 *)*v1;
  }
  dat_846c0 = NULL;
  dat_846b8 = 0;
  if (dat_84aac) {
    do {
      v2 = (char *)sub_26cd0(1);
      if (!v2)
        return;
    } while (!sub_263f0(v2));
    return;
  }
  dat_846c0 = NULL;
  dat_846b8 = 0;
  return;
}


// Function: sub_276a0 @ 0x276a0
void sub_276a0(void) // return-dupe
{
  unsigned int v1;
  stat v10 [2]; // stack - 0x1a0
  int8 *v11;
  char *v12;
  int4 v13; // stack - 0x50
  int8 *v2;
  int4 v3; // eax
  int4 v4;
  unsigned long v5; // rax
  int8 v6; // rax
  unsigned long *v7;
  int8 *v8; // rbx
  unsigned long v9 [11]; // stack - 0x1f8
  
  sub_26d90();
  if (dat_846c0) { // branch-flip
    if (dat_84b28) {
label_276e7:
      v3 = sub_24ae0();
      if (v3) {
        if (v3 != 1) {
          if (dat_84d50)
            (*dat_84d50)();
          v12 = "Only one -C option is allowed with --listed-incremental";
label_27a54:
          error(0,0,dcgettext(NULL,v12,5));
          sub_2c970(2); // no-return
        }
        if (!(int4)dat_846c0[5]) {
          if (dat_84d50)
            (*dat_84d50)();
          v12 = "Using -C option inside file list is not allowed with --listed-incremental";
          goto label_27a54;
        }
      }
      sub_1fd80();
      goto label_27710;
    }
  }
  else {
    sub_26cf0(".",0,0,0);
    if (dat_84b28) goto label_276e7;
label_27710:
    if (!dat_846c0) {
      v3 = 0;
      goto label_277b7;
    }
  }
  v11 = dat_846c0;
  v4 = 0;
  do {
    while ((((v3 = v4, v11[6] || (v11[7])) || (*(uint1 *)((int8)v11 + 0x23) & 0x10)) || (sub_24c80(*(unsigned int *)&v11[5]), !*(char *)v11[2]))) {
label_27730:
      v11 = (int8 *)*v11;
      v4 = v3 + 1;
      if (!v11) goto label_277b0;
    }
    sub_2f630(v9);
    v12 = (char *)v11[2];
    if (sub_246f0(v12,v10)) {
      sub_24ec0((char *)v11[2]);
      goto label_27730;
    }
    if ((v10[0]._24_4_ & 0xf000) == 0x4000) {
      v4 = __openat_2(dat_81234,(char *)v11[2],dat_84808 | 0x10000);
      if (0 <= v4) { // branch-flip
        v13 = v4;
        v4 = fstat(v4,v10);
        if (v4) // branch-flip
          sub_24ec0((char *)v11[2]);
        else {
          v9[0] = sub_4c7a0((char *)v11[2]);
          v11[6] = v11[6] + 1;
          sub_26ee0(v9,v11);
        }
      }
      else {
        sub_24dd0((char *)v11[2]);
      }
    }
    sub_2f6f0(v9);
    v11 = (int8 *)*v11;
    v4 = v3 + 1;
  } while (v11);
label_277b0:
  v3 += 1;
label_277b7:
  dat_846c0 = (int8 *)sub_25800(dat_846c0,v3,sub_259f0);
  v5 = sub_41b30(0,0,sub_26490,sub_259d0,0);
  if (dat_846c0) { // branch-flip
    v3 = 0;
    v2 = dat_846c0;
    v11 = NULL;
    do {
      while( true ) {
        v8 = v2;
        v2 = (int8 *)*v8;
        v12 = (char *)v8[2];
        v1 = *(unsigned int *)&v8[5];
        v8[0xb] = sub_25340(v1,v12);
        if ((v11) && (v6 = sub_418d0(v5,v8), v6)) break;
label_27869:
        v8[6] = 0;
        if (!sub_42140(v5,v8))
          sub_16420(); // no-return
        v3 += 1;
        v11 = v8;
        if (!v2) goto label_27891;
      }
      if (!v8[8]) {
        if (*(int8 *)(v6 + 0x48))
          sub_266c0(*(int8 *)(v6 + 0x48),(char *)v8[2],v8[3]);
        v3 -= 1;
        sub_421a0(v5,v8);
        sub_27150(v6);
        sub_25ad0();
        goto label_27869;
      }
      if (v8[9])
        sub_266c0(v8[9],*(char **)(v6 + 0x10),*(uint8 *)(v6 + 0x18));
      sub_27150(v8);
      sub_25ad0();
    } while (v2);
  }
  else {
    v11 = NULL;
    v3 = 0;
  }
label_27891:
  dat_846b8 = v11;
  sub_41ce0(v5);
  dat_846c0 = (unsigned long *)sub_25800(dat_846c0,v3,sub_25970);
  v7 = dat_846c0;
  if (!dat_84b28)
    return;
  while( true ) {
    if (!v7)
      return;
    if (*(char *)v7[2]) break;
    v7 = (unsigned long *)*v7;
  }
  sub_1fab0(v7[7]); // tail-call
  return;
}


// Function: sub_27aa0 @ 0x27aa0
int8 sub_27aa0(char *a0)
{
  int8 v1; // rax
  
  while ((((v1 = sub_264a0(a0), !v1 && (dat_84aac)) && (dat_846c0)) && (*(int8 *)(dat_846c0 + 0x30)))) {
    sub_26d90();
    if (*(int8 *)(dat_846c0 + 0x30))
      return 0;
  }
  return v1;
}


// Function: sub_27b00 @ 0x27b00
int8 * sub_27b00(void) // return-dupe
{
  int8 *v1;
  
  v1 = dat_84620;
  if ((!dat_84620) && (dat_84620 = dat_846c0, v1 = dat_846c0, !dat_846c0))
    return NULL;
  while ((v1[6] || (!*(char *)v1[2]))) {
    v1 = (int8 *)*v1;
    dat_84620 = v1;
    if (!v1)
      return NULL;
  }
  v1[6] = 1;
  sub_24c80(*(unsigned int *)&v1[5]);
  return dat_84620;
}


// Function: sub_27b80 @ 0x27b80
void sub_27b80(void) // return-dupe
{
  int8 *v1;
  
  dat_84620 = 0;
  v1 = dat_846c0;
  if (!dat_846c0)
    return;
  do {
    v1[6] = 0;
    v1 = (int8 *)*v1;
  } while (v1);
  dat_84620 = 0;
  return;
}


// Function: sub_27bc0 @ 0x27bc0
void * sub_27bc0(char *a0,char *a1)
{
  uint8 v1;
  uint8 v2; // rax
  void *v3; // rax
  uint8 v4;
  
  v2 = strlen(a0);
  v1 = strlen(a1) + 1;
  v4 = 0;
  if ((v2) && (v4 = v2, a0[v2 - 1] != '/'))
    v4 = v2 + 1;
  v3 = (void *)sub_4c5b0(v1 + v4);
  memcpy(v3,a0,v2);
  *(char *)((int8)v3 + v2) = 0x2f;
  memcpy((void *)((int8)v3 + v4),a1,v1);
  return v3;
}


// Function: sub_27c50 @ 0x27c50
int8 sub_27c50(char *a0,int8 a1)
{
  char *v1;
  char v2;
  char *v3;
  
  v2 = *a0;
  v3 = a0;
  while (v2 == '/') {
    v1 = &v3[1];
    v3 = &v3[1];
    v2 = *v1;
  }
  while( true ) {
    while( true ) {
      if (!v2)
        return -1;
      v3 = &v3[1];
      if (v2 == '/') break;
      v2 = *v3;
    }
    a1 -= 1;
    if (!a1) break;
    for (; v2 = *v3, v2 == '/'; v3 = &v3[1]) {
    }
  }
  return (int8)v3 - (int8)a0;
}


// Function: sub_27cc0 @ 0x27cc0
unsigned long sub_27cc0(char *a0)
{
  char v1;
  unsigned long v2; // rax
  char v3;
  
  while (((v3 = *a0, v3 != '.' || (a0[1] != '.')) || (v1 = a0[2], v2 = CONCAT71((undefined7)((uint8)v2 >> 8),v1 == '/' || !v1), v1 != '/' && v1))) {
    while (a0 = &a0[1], v3 != '/') {
      if (!v3)
        return 0;
      v3 = *a0;
    }
  }
  return v2;
}


// Function: sub_27d10 @ 0x27d10
unsigned long sub_27d10(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x10);
  if (v1)
    return (*v1)(); // jump-as-call
  return 0;
}


// Function: sub_27d30 @ 0x27d30
unsigned long sub_27d30(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 8);
  if (v1)
    return (*v1)(); // jump-as-call
  return 1;
}


// Function: sub_27d50 @ 0x27d50
unsigned long sub_27d50(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x30);
  if (v1)
    return (*v1)(); // jump-as-call
  return 1;
}


// Function: sub_27d70 @ 0x27d70
unsigned long sub_27d70(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x38);
  if (v1)
    return (*v1)(); // jump-as-call
  return 0;
}


// Function: sub_27d90 @ 0x27d90
unsigned long sub_27d90(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x40);
  if (v1)
    return (*v1)(); // jump-as-call
  return 0;
}


// Function: sub_27db0 @ 0x27db0
unsigned long sub_27db0(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x28);
  if (v1)
    return (*v1)(); // jump-as-call
  return 1;
}


// Function: sub_27dd0 @ 0x27dd0
unsigned long sub_27dd0(int8 a0)
{
  code *v1;
  
  v1 = *(code **)(*(int8 *)(a0 + 0x20) + 0x20);
  if (v1)
    return (*v1)(); // jump-as-call
  return 1;
}


// Function: sub_27df0 @ 0x27df0
unsigned long sub_27df0(char *a0,int8 a1)
{
  char v1;
  char *v2; // rdi
  char *v3;
  
  v3 = a0;
  do {
    if (v3 == &a0[a1])
      return 1;
    v2 = &v3[1];
    v1 = *v3;
    v3 = v2;
  } while (!v1);
  return 0;
}


// Function: sub_27e20 @ 0x27e20
void sub_27e20(int8 a0,struct_0 *a1)
{
  unsigned long *v1;
  int8 v2;
  unsigned long v3;
  void *v4;
  
  v4 = *(void **)(a0 + 0x140);
  v2 = *(int8 *)(a0 + 0x130);
  if (*(int8 *)(a0 + 0x138) == v2) {
    v4 = (void *)sub_4c680(v4,(uint8 *)(a0 + 0x138),0x10);
    *(void **)(a0 + 0x140) = v4;
  }
  v3 = a1->field_0x8;
  v1 = (unsigned long *)((int8)v4 + v2 * 0x10);
  *v1 = a1->field_0x0;
  v1[1] = v3;
  *(int8 *)(a0 + 0x130) = v2 + 1;
  return;
}


// Function: sub_27e90 @ 0x27e90
unsigned long sub_27e90(int8 a0) // return-dupe x2
{
  uint4 v1;
  
  v1 = dat_84400;
  if (!dat_84400)
    v1 = dat_84ba4;
  if (v1 != 5) {
    if (6 <= v1) { // branch-flip
      if (v1 != 6)
        return 0;
    }
    else if (v1 != 2) {
      if (v1 == 4) {
        *(unsigned long *)(a0 + 0x20) = 0x7f780;
        return 1;
      }
      return 0;
    }
    *(unsigned long *)(a0 + 0x20) = 0x7f840;
    return 1;
  }
  *(unsigned long *)(a0 + 0x20) = 0x7f7e0;
  return 1;
}


// Function: sub_27f70 @ 0x27f70
void sub_27f70(char (*a0)[16])
{
  char v1; // al
  
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  v1 = sub_27e90(a0);
  if ((v1) && ((code *)**(unsigned long **)a0[2])) {
    (*(code *)**(unsigned long **)a0[2])(); // jump-as-call
    return;
  }
  return;
}


// Function: sub_27fb0 @ 0x27fb0
unsigned long sub_27fb0(struct_20 *a0,int8 a1) // return-dupe x2
{
  int8 v1;
  int8 v2; // rdx
  
  v1 = a0->field_0x8;
  if (a1 < v1) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (a1 <= v1)
    return 1;
  while( true ) {
    v2 = a1 - v1;
    if (0x201 <= v2)
      v2 = 0x200;
    v1 = write(a0->field_0x0,(void *)0x63760,v2);
    if (v1 <= 0) break;
    v1 += a0->field_0x8;
    a0->field_0x8 = v1;
    if (a1 <= v1)
      return 1;
  }
  if (v1)
    return 0;
  *__errno_location() = 0x16;
  return 0;
}


// Function: sub_28050 @ 0x28050
unsigned long sub_28050(struct_21 *a0,uint8 a1) // return-dupe
{
  unsigned int v1;
  
  if (a0->field_0x4) { // branch-flip
    v1 = a0->field_0x0;
    if (0 <= (int8)lseek(v1,a1,0))
      return 1;
  }
  else if (sub_27fb0(a0,a1))
    return 1;
  sub_24e90((char *)*a0->field_0x18,a1);
  return 0;
}


// Function: sub_280b0 @ 0x280b0
uint4 sub_280b0(struct_102 *a0,int8 a1)
{
  uint1 v1;
  unsigned int v2;
  uint4 v3; // eax
  int8 v4; // rax
  uint8 v5; // rax
  uint8 v6;
  int8 v7;
  
  v3 = sub_28050(a0,*(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + a1 * 0x10));
  if ((char)v3) {
    v7 = *(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + 8 + a1 * 0x10);
    if (v7) { // branch-flip
      while (0 < v7) {
        v6 = 0x200;
        if (v7 <= 0x200)
          v6 = v7;
        v4 = sub_e130();
        if (!v4) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
          dat_84d58 = 2;
          return 0;
        }
        sub_d1f0(v4);
        a0->field_0x10 = a0->field_0x10 + 0x200;
        v5 = sub_249f0(a0->field_0x0,v4,v6);
        v7 -= v5;
        sub_ccf0(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
        a0->field_0x8 = a0->field_0x8 + v5;
        if (v6 != v5) {
          sub_32e30(*(char **)a0->field_0x18,v5,v6);
          return 0;
        }
      }
    }
    else {
      v1 = a0->field_0x4;
      if ((v1) && (v2 = a0->field_0x0, sub_2aed0(v2))) {
        v3 = (uint4)v1;
        sub_32d90(*(char **)a0->field_0x18);
      }
    }
  }
  return v3;
}


// Function: sub_28220 @ 0x28220
uint8 sub_28220(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3;
  
  v2 = sub_219b0((uint1 *)(dat_84408 + 0x1c4),0xc);
  v1 = *(int8 *)(a0 + 0x18);
  *(unsigned long *)(v1 + 0x118) = *(unsigned long *)(v1 + 0x88);
  v3 = 0;
  if (0 <= (int8)v2)
    v3 = v2;
  *(uint8 *)(v1 + 0x88) = v3;
  return ~v2 >> 0x3f;
}


// Function: sub_28270 @ 0x28270
uint8 sub_28270(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3;
  
  v2 = sub_219b0((uint1 *)(dat_84408 + 0x1e3),0xc);
  v1 = *(int8 *)(a0 + 0x18);
  *(unsigned long *)(v1 + 0x118) = *(unsigned long *)(v1 + 0x88);
  v3 = 0;
  if (0 <= (int8)v2)
    v3 = v2;
  *(uint8 *)(v1 + 0x88) = v3;
  return ~v2 >> 0x3f;
}


// Function: sub_282c0 @ 0x282c0
unsigned long sub_282c0(int8 a0,uint1 *a1)
{
  int8 v1;
  uint8 v2; // stack - 0x38
  unsigned long v3; // r8
  uint8 v4; // stack - 0x30
  
  v3 = 1;
  if (a1[0xc]) {
    v2 = sub_219b0(a1,0xc);
    v4 = sub_219b0(&a1[0xc],0xc);
    v3 = 2;
    if ((((0 <= (int8)(v4 | v2)) && (!SCARRY8(v2,v4))) && (v1 = *(int8 *)(a0 + 0x18), (int8)(v2 + v4) <= *(int8 *)(v1 + 0x88))) && (0 <= *(int8 *)(v1 + 0x118))) {
      sub_27e20(v1,&v2);
      v3 = 0;
    }
  }
  return v3;
}


// Function: sub_28370 @ 0x28370
unsigned long sub_28370(int8 a0) // return-dupe
{
  uint1 *v1;
  char v2;
  char *v3;
  int4 v4; // eax
  int8 v5;
  uint8 v6; // rbx
  uint1 *v7;
  int8 v8;
  
  v5 = dat_84408;
  v2 = *(char *)(dat_84408 + 0x159);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  if (v2) goto label_283b0;
  if (!*(char *)(dat_84408 + 0x16e)) goto label_283b0;
  v7 = (uint1 *)(dat_84408 + 0x164);
  v1 = (uint1 *)(dat_84408 + 0x1c4);
  do {
    v4 = sub_282c0(a0,v7);
    if (v4) goto label_28409;
    v7 = &v7[0x18];
  } while (v1 != v7);
  if (!*(char *)(v5 + 0x163))
    return 1;
  v5 = sub_e130();
  do {
    if (!v5) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      dat_84d58 = 2;
      return 0;
    }
    v6 = 0;
    sub_d1f0(v5);
    v8 = v5;
    do {
      v6 += 1;
      v4 = sub_282c0(a0,v8);
      v8 += 0x18;
      if (0x15 <= v6) break;
    } while (!v4);
    *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + 0x200;
    if ((!*(char *)(v5 + 0x1f8)) || (v4)) {
label_28409:
      if (v4 == 2) {
        if (dat_84d50)
          (*dat_84d50)();
        v3 = (char *)**(unsigned long **)(a0 + 0x18);
        error(0,0,dcgettext(NULL,"%s: invalid sparse archive member",5),v3);
        dat_84d58 = 2;
        return 0;
      }
      return 1;
    }
label_283b0:
    v5 = sub_e130();
  } while( true );
}


// Function: sub_28500 @ 0x28500
unsigned long sub_28500(int8 a0) // return-dupe
{
  uint1 *v1;
  char *v2;
  int4 v3; // eax
  int8 v4;
  uint1 *v5;
  uint8 v6; // rbx
  int8 v7;
  
  v4 = dat_84408;
  v5 = (uint1 *)(dat_84408 + 0x182);
  v1 = (uint1 *)(dat_84408 + 0x1e2);
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 0x130) = 0;
  do {
    v3 = sub_282c0(a0,v5);
    if (v3) goto label_285b0;
    v5 = &v5[0x18];
  } while (v5 != v1);
  if (!*(char *)(v4 + 0x1e2))
    return 1;
  do {
    v4 = sub_e130();
    if (!v4) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
      dat_84d58 = 2;
      return 0;
    }
    v6 = 0;
    sub_d1f0(v4);
    v7 = v4;
    do {
      v6 += 1;
      v3 = sub_282c0(a0,v7);
      v7 += 0x18;
      if (0x15 <= v6) break;
    } while (!v3);
  } while ((*(char *)(v4 + 0x1f8)) && (!v3));
label_285b0:
  if (v3 != 2)
    return 1;
  if (dat_84d50)
    (*dat_84d50)();
  v2 = (char *)**(unsigned long **)(a0 + 0x18);
  error(0,0,dcgettext(NULL,"%s: invalid sparse archive member",5),v2);
  dat_84d58 = 2;
  return 0;
}


// Function: sub_28660 @ 0x28660
char sub_28660(struct_103 *a0,int8 a1) // return-dupe
{
  int4 v1;
  uint8 *v10; // rsi
  uint8 v11;
  int8 *v12; // r15
  char v13; // stack - 0xf1
  char v2; // al
  int8 v3; // rax
  int8 v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  int8 v7;
  stat v8; // stack - 0xe8
  char v9 [24];
  
  a1 *= 0x10;
  v10 = (uint8 *)(a1 + *(int8 *)(a0->field_0x18 + 0x140));
  v7 = v10[1];
  v2 = sub_28050(a0,*v10);
  v13 = v2;
  if (v2) {
    while (0 < v7) {
      v11 = 0x200;
      if (v7 <= 0x200)
        v11 = v7;
      v3 = sub_e130();
      v4 = sub_4af10(a0->field_0x0,v3,v11);
      if (v4 == -1) {
        v12 = (int8 *)(a1 + ((unsigned long *)a0->field_0x18)[0x28]);
        sub_24e00(*(char **)(unsigned long *)a0->field_0x18,(v12[1] + *v12) - v7,v11);
        return '\0';
      }
      if (!v4) {
        v1 = a0->field_0x0;
        if (fstat(v1,&v8)) { // branch-flip
          v12 = (int8 *)(a1 + *(int8 *)(a0->field_0x18 + 0x140));
          v7 = (v7 + *(int8 *)(a0->field_0x18 + 0x88)) - (v12[1] + *v12);
        }
        else {
          v7 = *(int8 *)(a0->field_0x18 + 0x88) - v8._48_8_;
        }
        if (dat_83b80 & 0x80) {
          if (dat_84d50)
            (*dat_84d50)();
          v5 = sub_43090(v7,v9);
          v6 = sub_4aa70(*(char **)a0->field_0x18);
          error(0,0,dcngettext(NULL,"%s: File shrank by %s byte; padding with zeros","%s: File shrank by %s bytes; padding with zeros",v7,5),v6,v5);
        }
        if (dat_84b52)
          return '\0';
        sub_2f7d0(1);
        return '\0';
      }
      memset((void *)(v3 + v4),0,0x200 - v4);
      a0->field_0x10 = a0->field_0x10 + v4;
      sub_d1f0(v3);
      v7 -= v4;
      v13 = v2;
    }
  }
  return v13;
}


// Function: sub_28870 @ 0x28870
unsigned long sub_28870(uint8 *a0,char *a1,uint8 a2)
{
  int4 *v1; // rax
  uint8 v2; // rax
  char *v3; // stack - 0x38
  
  if ((uint4)((int4)*a1 - 0x30U) <= 9) {
    v1 = __errno_location();
    *v1 = 0;
    v2 = strtoumax(a1,&v3,10);
    if (((v2 <= a2) && (*v1 != 0x22)) && (!*v3)) {
      *a0 = v2;
      return 1;
    }
  }
  return 0;
}


// Function: sub_28910 @ 0x28910
unsigned int sub_28910(int8 a0)
{
  char v1;
  char v10 [20];
  unsigned long v11; // stack - 0x70
  unsigned long v12; // stack - 0x68
  unsigned long v13;
  char *v14; // rdx
  uint8 v15; // stack - 0x88
  unsigned long v16; // stack - 0x60
  int8 v2;
  unsigned int v3; // eax
  int8 v4; // rax
  char *v5;
  char *v6;
  char *v7;
  char *v8;
  char *v9;
  
  if (!*(int4 *)(*(int8 *)(a0 + 0x18) + 0x124))
    return 1;
  v4 = sub_d180();
  sub_d1f0(dat_84408);
  v5 = (char *)sub_e130();
  if (!v5) {
label_28ca8:
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
    sub_163d0(); // no-return
  }
  v9 = &v5[0x200];
  v6 = v5;
  v8 = v10;
  do {
    if (v6 == v9) {
      sub_d1f0(v5);
      v6 = (char *)sub_e130();
      if (!v6) goto label_28ca8;
      v9 = &v6[0x200];
      v5 = v6;
    }
    v7 = &v6[1];
    v1 = *v6;
    v14 = &v8[1];
    *v8 = v1;
    if (v1 == '\n') {
      *v8 = '\0';
      v3 = sub_28870(&v11,v10,0xffffffffffffffff);
      if (!(char)v3) goto label_28a87;
      v2 = *(int8 *)(a0 + 0x18);
      *(unsigned long *)(v2 + 0x138) = v11;
      *(unsigned long *)(v2 + 0x140) = sub_4c720(v11,0x10);
      v2 = *(int8 *)(a0 + 0x18);
      *(unsigned long *)(v2 + 0x130) = 0;
      if (!*(int8 *)(v2 + 0x138)) goto label_28944;
      v15 = 0;
      goto label_28b1a;
    }
    v6 = v7;
    v8 = v14;
  } while (v14 != &v10[0x14]);
label_28a00:
  if (dat_84d50)
    (*dat_84d50)();
  v5 = "%s: numeric overflow in sparse archive member";
  v13 = **(unsigned long **)(a0 + 0x18);
  error(0,0,dcgettext(NULL,v5,5),v13); // return-dupe
  dat_84d58 = 2;
  return 0;
label_28b1a:
  v6 = v10;
  while( true ) {
    if (v7 == v9) {
      sub_d1f0(v5);
      v7 = (char *)sub_e130();
      if (!v7) goto label_28ca8;
      v9 = &v7[0x200];
      v5 = v7;
    }
    v8 = &v7[1];
    v1 = *v7;
    v14 = &v6[1];
    *v6 = v1;
    if (v1 == '\n') break;
    v6 = v14;
    v7 = v8;
    if (v14 == &v10[0x14]) goto label_28a00;
  }
  *v6 = '\0';
  if (!sub_28870(&v11,v10,0x7fffffffffffffff)) goto label_28a87;
  v12 = v11;
  v6 = v10;
  while( true ) {
    if (v8 == v9) {
      sub_d1f0(v5);
      v8 = (char *)sub_e130();
      if (!v8) goto label_28ca8;
      v9 = &v8[0x200];
      v5 = v8;
    }
    v7 = &v8[1];
    v1 = *v8;
    v14 = &v6[1];
    *v6 = v1;
    if (v1 == '\n') break;
    v6 = v14;
    v8 = v7;
    if (v14 == &v10[0x14]) goto label_28a00;
  }
  *v6 = '\0';
  if (!sub_28870(&v11,v10,0x7fffffffffffffff)) goto label_28a87;
  v16 = v11;
  sub_27e20(*(int8 *)(a0 + 0x18),&v12);
  v15 += 1;
  if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x138) <= v15) {
label_28944:
    sub_d1f0(v5);
    *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + (sub_d180() - v4) * 0x200;
    return v3;
  }
  goto label_28b1a;
label_28a87:
  if (dat_84d50)
    (*dat_84d50)();
  v5 = "%s: malformed sparse archive member";
  v13 = **(unsigned long **)(a0 + 0x18);
  error(0,0,dcgettext(NULL,v5,5),v13);
  dat_84d58 = 2;
  return 0;
}


// Function: sub_28ce0 @ 0x28ce0
void sub_28ce0(int8 a0)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x88);
  *(unsigned long *)(a0 + 0x88) = *(unsigned long *)(a0 + 0x118);
  sub_132d0(a0);
  *(unsigned long *)(a0 + 0x88) = v1;
  return;
}


// Function: sub_28d20 @ 0x28d20
unsigned int sub_28d20(int8 a0)
{
  unsigned long *v1;
  int8 v10;
  int8 v2;
  unsigned int v3; // eax
  int8 v4; // rax
  int8 v5;
  uint8 v6;
  char *v7; // rax
  uint8 v8; // stack - 0x60
  char v9 [24];
  
  v4 = sub_d180();
  v5 = *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140);
  sub_1ca30("GNU.sparse.size",*(int8 *)(a0 + 0x18),0);
  sub_1ca30("GNU.sparse.numblocks",*(int8 *)(a0 + 0x18),0);
  if ((sub_1bef0("GNU.sparse.map")) || (!dat_84a80)) {
    v8 = 0;
    v5 = *(int8 *)(a0 + 0x18);
    if (*(int8 *)(v5 + 0x130)) {
      do {
        sub_1ca30("GNU.sparse.offset",v5,&v8);
        sub_1ca30("GNU.sparse.numbytes",*(int8 *)(a0 + 0x18),&v8);
        v5 = *(int8 *)(a0 + 0x18);
        v8 += 1;
      } while (v8 < *(uint8 *)(v5 + 0x130));
    }
    v3 = 1;
    v5 = sub_28ce0(v5);
    sub_12f60(*(unsigned long *)(a0 + 0x18),v5,v4);
  }
  else {
    sub_1ca30("GNU.sparse.name",*(int8 *)(a0 + 0x18),0);
    v1 = *(unsigned long **)(a0 + 0x18);
    v2 = v1[1];
    v1[1] = sub_1c080(v1,"%d/GNUSparseFile.%p/%f",0);
    sub_1cf40(*(int8 *)(a0 + 0x18) + 0x168);
    v10 = *(int8 *)(a0 + 0x18);
    v8 = 0;
    v6 = 0;
    if (*(int8 *)(v10 + 0x130)) {
      while( true ) {
        v7 = (char *)sub_43090(*(uint8 *)(v5 + v6 * 0x10),v9);
        sub_1cf50((int8 *)(*(int8 *)(a0 + 0x18) + 0x168),v7);
        sub_1cf50((int8 *)(*(int8 *)(a0 + 0x18) + 0x168),",");
        v7 = (char *)sub_43090(*(uint8 *)(v5 + 8 + v8 * 0x10),v9);
        sub_1cf50((int8 *)(*(int8 *)(a0 + 0x18) + 0x168),v7);
        v10 = *(int8 *)(a0 + 0x18);
        v8 += 1;
        if (*(uint8 *)(v10 + 0x130) <= v8) break;
        v6 = v8;
        if (v8) {
          sub_1cf50((int8 *)(v10 + 0x168),",");
          v6 = v8;
        }
      }
    }
    v3 = sub_1cfa0((struct_100 *)(v10 + 0x168),"GNU.sparse.map");
    if ((char)v3) { // branch-flip
      v5 = sub_28ce0(*(int8 *)(a0 + 0x18));
      sub_12f60(*(unsigned long *)(a0 + 0x18),v5,v4);
      if (v2) {
        free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
        *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v2;
      }
    }
    else {
      free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
      *(int8 *)(*(int8 *)(a0 + 0x18) + 8) = v2;
    }
  }
  return v3;
}


// Function: sub_28fd0 @ 0x28fd0
unsigned long sub_28fd0(int8 a0)
{
  char *v1;
  char *v10;
  char *v11;
  char v12 [24];
  int8 v13;
  int8 v14;
  uint8 *v15;
  uint8 *v16;
  uint8 *v17; // r15
  int8 v18; // stack - 0x70
  char v2;
  unsigned long v3;
  unsigned long *v4;
  uint8 v5;
  uint8 v6; // rax
  uint8 v7;
  char *v8;
  char *v9;
  
  v18 = sub_d180();
  v13 = *(int8 *)(a0 + 0x18);
  v3 = *(unsigned long *)(v13 + 8);
  v5 = *(uint8 *)(v13 + 0x130);
  v15 = *(uint8 **)(v13 + 0x140);
  v5 = strlen((char *)sub_43090(v5,v12));
  v14 = *(int8 *)(a0 + 0x18);
  v13 = v5 + 1;
  if (*(int8 *)(v14 + 0x130)) {
    v5 = 0;
    v16 = v15;
    do {
      v7 = *v16;
      v5 += 1;
      v17 = &v16[2];
      v6 = strlen((char *)sub_43090(v7,v12));
      v7 = v16[1];
      v7 = strlen((char *)sub_43090(v7,v12));
      v14 = *(int8 *)(a0 + 0x18);
      v13 = v6 + 2 + v13 + v7;
      v16 = v17;
    } while (v5 < *(uint8 *)(v14 + 0x130));
  }
  v5 = v13 + 0x3fe;
  if (0 <= (int8)(v13 + 0x1ffU))
    v5 = v13 + 0x1ffU;
  *(int8 *)(v14 + 0x118) = *(int8 *)(v14 + 0x118) + (v5 & 0xfffffffffffffe00);
  *(int8 *)(a0 + 0x10) = *(int8 *)(a0 + 0x10) + (v5 & 0xfffffffffffffe00);
  sub_1ca30("GNU.sparse.major",v14,0);
  sub_1ca30("GNU.sparse.minor",*(int8 *)(a0 + 0x18),0);
  sub_1ca30("GNU.sparse.name",*(int8 *)(a0 + 0x18),0);
  sub_1ca30("GNU.sparse.realsize",*(int8 *)(a0 + 0x18),0);
  v4 = *(unsigned long **)(a0 + 0x18);
  v4[1] = sub_1c080(v4,"%d/GNUSparseFile.%p/%f",0);
  v13 = *(int8 *)(a0 + 0x18);
  v10 = *(char **)(v13 + 8);
  if (0x65 <= strlen(v10)) {
    v10[100] = '\0';
    v13 = *(int8 *)(a0 + 0x18);
  }
  v13 = sub_28ce0(v13);
  sub_12f60(*(unsigned long *)(a0 + 0x18),v13,v18);
  free(*(void **)(*(int8 *)(a0 + 0x18) + 8));
  *(unsigned long *)(*(int8 *)(a0 + 0x18) + 8) = v3;
  v8 = (char *)sub_e130();
  v9 = (char *)sub_43090(*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130),v12);
  v11 = &v8[0x200];
  v2 = *v9;
  v10 = v8;
  while (v2) {
    if (v10 == v11) {
      sub_d1f0(v8);
      v10 = (char *)sub_e130();
      v11 = &v10[0x200];
      v8 = v10;
    }
    v2 = *v9;
    v9 = &v9[1];
    v1 = &v10[1];
    *v10 = v2;
    v2 = *v9;
    v10 = v1;
  }
  if (v11 == v10) {
    sub_d1f0(v8);
    v8 = (char *)sub_e130();
    v10 = v8;
  }
  *v10 = '\n';
  v10 = &v10[1];
  if (*(int8 *)(*(int8 *)(a0 + 0x18) + 0x130)) {
    v5 = 0;
    do {
      v9 = (char *)sub_43090(*v15,v12);
      v11 = &v8[0x200];
      v2 = *v9;
      while (v2) {
        if (v10 == v11) {
          sub_d1f0(v8);
          v10 = (char *)sub_e130();
          v11 = &v10[0x200];
          v8 = v10;
        }
        v2 = *v9;
        v9 = &v9[1];
        v1 = &v10[1];
        *v10 = v2;
        v2 = *v9;
        v10 = v1;
      }
      if (v11 == v10) {
        sub_d1f0(v8);
        v10 = (char *)sub_e130();
        v11 = &v10[0x200];
        v8 = v10;
      }
      *v10 = '\n';
      v9 = (char *)sub_43090(v15[1],v12);
      v2 = *v9;
      while (v10 = &v10[1], v2) {
        if (v10 == v11) {
          sub_d1f0(v8);
          v10 = (char *)sub_e130();
          v11 = &v10[0x200];
          v8 = v10;
        }
        v2 = *v9;
        v9 = &v9[1];
        *v10 = v2;
        v2 = *v9;
      }
      if (v11 == v10) {
        sub_d1f0(v8);
        v8 = (char *)sub_e130();
        v10 = v8;
      }
      *v10 = '\n';
      v10 = &v10[1];
      v5 += 1;
      v15 = &v15[2];
    } while (v5 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130));
  }
  memset(v10,0,(uint8)&v8[0x200 - (int8)v10]);
  sub_d1f0(v8);
  return 1;
}


// Function: sub_29390 @ 0x29390
void sub_29390(int8 a0)
{
  int8 v1;
  
  v1 = *(int8 *)(a0 + 0x18);
  *(int4 *)(v1 + 0x124) = dat_84a84;
  *(unsigned int *)(v1 + 0x128) = dat_84a80;
  if (!dat_84a84) {
    sub_28d20(a0); // tail-call
    return;
  }
  sub_28fd0(a0); // tail-call
  return;
}


// Function: sub_293d0 @ 0x293d0
void sub_293d0(int8 a0,uint8 *a1,char *a2,int8 a3) // return-dupe
{
  char *v1;
  uint8 v2;
  int8 v3;
  
  v3 = *(int8 *)(a0 + 0x18);
  v2 = *a1;
  if (*(uint8 *)(v3 + 0x130) <= v2)
    return;
  do {
    a3 -= 1;
    sub_12be0(*(uint8 *)(v2 * 0x10 + *(int8 *)(v3 + 0x140)),a2,0xc);
    v1 = &a2[0xc];
    a2 = &a2[0x18];
    sub_12be0(*(uint8 *)(*a1 * 0x10 + *(int8 *)(*(int8 *)(a0 + 0x18) + 0x140) + 8),v1,0xc);
    v3 = *(int8 *)(a0 + 0x18);
    v2 = *a1 + 1;
    *a1 = v2;
    if (*(uint8 *)(v3 + 0x130) <= v2)
      return;
  } while (a3);
  return;
}


// Function: sub_29480 @ 0x29480
unsigned long sub_29480(int8 a0) // return-dupe
{
  unsigned long *v1;
  int8 v2;
  int8 v3; // rax
  int8 v4; // rax
  unsigned long *v5; // rax
  uint8 v6;
  uint8 v7; // stack - 0x38
  unsigned long *v8;
  
  v3 = sub_d180();
  v4 = sub_132d0(*(int8 *)(a0 + 0x18));
  *(char *)(v4 + 0x9c) = 0x53;
  v2 = *(int8 *)(a0 + 0x18);
  if (5 <= *(uint8 *)(v2 + 0x130))
    *(char *)(v4 + 0x1e2) = 1;
  sub_12be0(*(uint8 *)(v2 + 0x88),(char *)(v4 + 0x1e3),0xc);
  sub_12be0(*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x118),(char *)(v4 + 0x7c),0xc);
  v7 = 0;
  sub_293d0(a0,&v7,(char *)(v4 + 0x182),4);
  v6 = v7;
  v2 = *(int8 *)(a0 + 0x18);
  *(bool *)(v4 + 0x1e2) = v7 < *(uint8 *)(v2 + 0x130);
  sub_12f60(v2,v4,v3);
  if (*(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130) <= v6)
    return 1;
  do {
    v5 = (unsigned long *)sub_e130();
    *v5 = 0;
    v5[0x3f] = 0;
    v6 = (uint8)(((int4)v5 - (int4)(unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8)) + 0x200U >> 3);
    v8 = (unsigned long *)((uint8)&v5[1] & 0xfffffffffffffff8);
    while (v6) {
      v1 = &v8[1];
      *v8 = 0;
      v6 -= 1;
      v8 = v1;
    }
    sub_293d0(a0,&v7,v5,0x15);
    v6 = v7;
    if (v7 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130))
      *(char *)&v5[0x3f] = 1;
    sub_d1f0(v5);
  } while (v6 < *(uint8 *)(*(int8 *)(a0 + 0x18) + 0x130));
  return 1;
}


// Function: sub_29600 @ 0x29600
unsigned long sub_29600(struct_55 *a0) // early-return
{
  unsigned int v1;
  char v10 [16];
  int8 v11; // stack - 0x250
  int8 v2;
  int8 v3; // rax
  int8 v4; // rdx
  char *v5;
  int8 v6;
  char v7 [520];
  int8 v8; // stack - 0x258
  int8 v9; // rbp
  
  v9 = 0;
  v2 = a0->field_0x18;
  v1 = a0->field_0x0;
  v8 = 0;
  *(unsigned long *)(v2 + 0x118) = 0;
  v11 = 0;
  if (!sub_27d50(a0,0,0))
    return 0;
  do {
    v10 = sub_24960(v1,v7,0x200);
    v3 = SUB168(v10,0);
    v6 = SUB168(v10,8);
    if ((!v3) || (v6 = v11, v3 == -1)) {
      if (!v11)
        v8 = v9;
      sub_27e20(v2,&v8,v6);
      *(int8 *)(v2 + 0x118) = *(int8 *)(v2 + 0x118) + v3;
      return sub_27d50(a0,2,0);
    }
    if (sub_27df0(v7,v3)) { // branch-flip
      if (v4) {
        sub_27e20(v2,&v8);
        v5 = NULL;
        v11 = 0;
        goto label_296c7;
      }
    }
    else {
      if (!v4)
        v8 = v9;
      *(int8 *)(v2 + 0x118) = *(int8 *)(v2 + 0x118) + v3;
      v11 = v4 + v3;
      v5 = v7;
label_296c7:
      if (!sub_27d50(a0,1,v5))
        return 0;
    }
    v9 += v3;
  } while( true );
}


// Function: sub_29770 @ 0x29770
unsigned int sub_29770(struct_21 *a0,uint8 a1,int8 a2)
{
  unsigned int v1; // eax
  int8 v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  int8 v5; // rdx
  char v6 [32];
  char v7 [520];
  int8 v8; // r14
  
  v1 = sub_28050(a0,a1);
  if ((char)v1) {
    for (; (int8)a1 < a2; a1 = a1 + v5) {
      v8 = a2 - a1;
      if (0x201 <= v8)
        v8 = 0x200;
      v2 = sub_4af10(a0->field_0x0,v7,v8);
      if (v2 == -1) {
        sub_24e00((char *)*a0->field_0x18,a1,v8);
        return 0;
      }
      if (!v2) {
        v4 = dcgettext(NULL,"Size differs",5);
        sub_10b50(a0->field_0x18,v4);
        return 0;
      }
      if (!sub_27df0(v7,v2,v2)) {
        v3 = sub_42ff0(a1,v6);
        v4 = dcgettext(NULL,"File fragment at %s is not a hole",5);
        sub_10b50(a0->field_0x18,v4,v3);
        return 0;
      }
    }
  }
  return v1;
}


// Function: sub_298b0 @ 0x298b0
char sub_298b0(struct_22 *a0,int8 a1)
{
  uint8 v1; // rax
  void *v2; // rax
  char *v3; // rax
  char v4 [520];
  int8 v5;
  uint8 v6;
  int8 *v7; // r15
  char v8; // stack - 0x251
  
  a1 *= 0x10;
  v8 = sub_28050(a0,*(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + a1));
  if (v8) {
    v5 = *(uint8 *)(*(int8 *)(a0->field_0x18 + 0x140) + 8 + a1);
    sub_ccf0(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
    do {
      if (v5 <= 0)
        return v8;
      v6 = 0x200;
      if (v5 <= 0x200)
        v6 = v5;
      v2 = (void *)sub_e130();
      if (!v2) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"Unexpected EOF in archive",5));
        dat_84d58 = 2;
        v8 = '\0'; // return-dupe
        return v8;
      }
      sub_d1f0(v2);
      a0->field_0x10 = a0->field_0x10 + 0x200;
      v1 = sub_4af10(a0->field_0x0,v4,v6);
      if (v1 == 0xffffffffffffffff) {
        v7 = (int8 *)(a1 + ((unsigned long *)a0->field_0x18)[0x28]);
        sub_24e00(*(char **)(unsigned long *)a0->field_0x18,(v7[1] + *v7) - v5,v6);
        v8 = '\0';
        return v8;
      }
      if (!v1) {
        sub_10b50(0x84840,dcgettext(NULL,"Size differs",5));
        v8 = '\0';
        return v8;
      }
      v5 -= v1;
      sub_ccf0(*(int8 *)(a0->field_0x18 + 0x118) - a0->field_0x10);
    } while (!memcmp(v2,v4,v1));
    v3 = dcgettext(NULL,"Contents differ",5);
    sub_10b50(a0->field_0x18,v3);
    v8 = '\0';
  }
  return v8;
}


// Function: sub_29cd0 @ 0x29cd0
uint4 sub_29cd0(int4 a0,int8 a1) // early-return
{
  char v1; // al
  int8 v2;
  uint8 v3;
  uint8 v4; // rbx
  int4 v5; // stack - 0x58
  char v6; // stack - 0x54
  int8 v7; // stack - 0x48
  int8 v8; // stack - 0x40
  int8 v9; // stack - 0x38
  
  if (!sub_27f70(&v5))
    return 3;
  v6 = 1;
  v5 = a0;
  v8 = a1;
  if (sub_29c60(&v5)) {
    if (*(int8 *)(v9 + 0x38)) { // branch-flip
      sub_29cb0(&v5);
      v2 = v8;
      if (0 <= a0) {
        sub_cc00(*(char **)(v8 + 8),*(unsigned long *)(v8 + 0x88),*(int8 *)(v8 + 0x118) - v7);
        v3 = 0;
        do {
          v2 = v8;
          if (*(uint8 *)(v8 + 0x130) <= v3) {
            sub_13c80(*(int8 *)(v2 + 0x118) - v7); // return-dupe
            return (sub_27d30(&v5) ^ 1) & 0xff;
          }
          v4 = v3 + 1;
          v1 = sub_27d70(&v5,v3);
          v3 = v4;
        } while (v1);
        sub_13c80(*(int8 *)(v8 + 0x118) - v7); // return-dupe
        sub_27d30(&v5);
        return 1;
      }
    }
    else {
      v2 = v8;
    }
    sub_13c80(*(int8 *)(v2 + 0x118) - v7);
    return (sub_27d30(&v5) ^ 1) & 0xff;
  }
  sub_13c80(*(int8 *)(v8 + 0x118) - v7);
  sub_27d30(&v5);
  return 1;
}


// Function: sub_29e10 @ 0x29e10
void sub_29e10(unsigned long a0) // return-dupe
{
  char v1 [24];
  unsigned long v2; // stack - 0x40
  
  if (!sub_27f70(v1))
    return;
  v2 = a0;
  sub_27d10(v1);
  return;
}


// Function: sub_29e70 @ 0x29e70
void sub_29e70(unsigned long a0) // return-dupe
{
  char v1 [24];
  unsigned long v2; // stack - 0x40
  
  if (!sub_27f70(v1))
    return;
  v2 = a0;
  sub_27dd0(v1);
  return;
}


// Function: sub_29ed0 @ 0x29ed0
uint4 sub_29ed0(unsigned int a0,int8 a1,int8 *a2) // return-dupe
{
  uint8 v1;
  char v2; // al
  int8 v3; // rax
  uint8 v4; // rbx
  unsigned int v5; // stack - 0x68
  char v6; // stack - 0x64
  unsigned long v7; // stack - 0x60
  int8 v8; // stack - 0x58
  int8 v9; // stack - 0x50
  
  if (!sub_27f70(&v5))
    return 3;
  v5 = a0;
  v9 = a1;
  v3 = lseek(a0,0,0);
  v7 = 0;
  v6 = v3 == 0;
  v2 = sub_27db0(&v5);
  v1 = 0;
  while( true ) {
    if (!v2) {
      *a2 = *(int8 *)(v9 + 0x118) - v8;
      sub_27d30(&v5);
      return 1;
    }
    if (*(uint8 *)(v9 + 0x130) <= v1) break;
    v4 = v1 + 1;
    v2 = sub_27d90(&v5,v1);
    v1 = v4;
  }
  *a2 = *(int8 *)(v9 + 0x118) - v8;
  return (sub_27d30(&v5) ^ 1) & 0xff;
}


// Function: sub_29fe0 @ 0x29fe0
uint4 sub_29fe0(int8 a0) // early-return
{
  uint4 v1; // eax
  unsigned int v2 [4]; // stack - 0x58
  int8 v3; // stack - 0x48
  int8 v4; // stack - 0x40
  
  if (!sub_27f70(v2))
    return 3;
  v2[0] = 0xffffffff;
  v4 = a0;
  v1 = sub_27db0(v2);
  sub_22b10(*(int8 *)(v4 + 0x118) - v3);
  return (sub_27d30(v2) & v1 ^ 1) & 0xff;
}


// Function: sub_2a070 @ 0x2a070
unsigned int sub_2a070(unsigned int a0,int8 a1)
{
  int8 v1;
  int8 v10; // stack - 0x58
  int8 v11; // stack - 0x50
  char v2; // al
  unsigned int v3; // eax
  int8 *v4; // rbx
  unsigned int v5; // stack - 0x68
  uint8 v6;
  uint8 v7; // rbp
  uint8 v8; // rsi
  char v9; // stack - 0x64
  
  v3 = sub_27f70(&v5);
  if ((char)v3) {
    v9 = 1;
    v5 = a0;
    v11 = a1;
    v3 = sub_27db0(&v5);
    sub_ccb0(a1);
    if ((char)v3) {
      v8 = 0;
      v6 = 0;
      do {
        if (*(uint8 *)(v11 + 0x130) <= v6) {
          sub_ccd0(); // return-dupe
          sub_27d30(&v5);
          return v3;
        }
        v1 = *(int8 *)(*(int8 *)(v11 + 0x140) + v6 * 0x10);
        if (!sub_29770(&v5,v8,v1)) break;
        v7 = v6 + 1;
        v2 = sub_298b0(&v5,v6);
        v4 = (int8 *)(v6 * 0x10 + *(int8 *)(v11 + 0x140));
        v8 = v4[1] + *v4;
        v6 = v7;
      } while (v2);
    }
    v3 = 0;
    sub_22b10(*(int8 *)(v11 + 0x118) - v10);
    sub_ccd0();
    sub_27d30(&v5);
  }
  return v3;
}


// Function: sub_2a190 @ 0x2a190
int8 sub_2a190(char *a0,int8 *a1) // return-dupe x2
{
  char *v1; // rax
  uint8 v2; // rax
  int8 v3; // rbx
  void *v4;
  
  v1 = strrchr(a0,0x2e);
  if (!v1)
    return 0;
  v2 = strlen(&v1[1]);
  if (!dat_81920)
    return 0;
  v3 = 0x81920;
  v4 = dat_81920;
  while ((*(uint8 *)(v3 + 8) != v2 || (memcmp(v4,&v1[1],v2)))) {
    v4 = *(void **)(v3 + 0x18);
    v3 += 0x18;
    if (!v4)
      return 0;
  }
  if (!a1)
    return v3;
  *a1 = (strlen(a0) - v2) + -1;
  return v3;
}


// Function: sub_2a240 @ 0x2a240
int8 sub_2a240(char *a0,unsigned long a1)
{
  unsigned long v1;
  int8 v2; // rax
  
  v2 = sub_2a190(a0,NULL);
  v1 = a1;
  if (v2)
    v1 = *(unsigned long *)(v2 + 0x10);
  v2 = v1;
  return v2;
}


// Function: sub_2a260 @ 0x2a260
void sub_2a260(void) // return-dupe
{
  int8 v1; // rax
  
  v1 = sub_2a240();
  if (!v1)
    return;
  dat_84b70 = v1;
  return;
}


// Function: sub_2a280 @ 0x2a280
void * sub_2a280(char *a0)
{
  unsigned long *v1; // rax
  void *v2;
  uint8 v3; // stack - 0x38
  
  v1 = (unsigned long *)sub_2a190(a0,&v3);
  v2 = NULL;
  if (v1) {
    if (5 <= v3) { // branch-flip
      if ((!strncmp(&a0[v3 - 4],".tar",4)) && (*(char *)*v1 != 't'))
        v3 -= 4;
    }
    else if (!v3)
      return NULL;
    v2 = (void *)sub_4c5b0(v3 + 1);
    memcpy(v2,a0,v3);
    *(char *)((int8)v2 + v3) = 0;
  }
  return v2;
}


// Function: sub_2a340 @ 0x2a340
bool sub_2a340(char *a0)
{
  stat v1; // stack - 0xa8
  bool v2; // zf
  
  if (stat(a0,&v1)) // branch-flip
    v2 = *__errno_location() == 2;
  else {
    v2 = (v1._24_4_ & 0xf000) == 0x8000;
  }
  return v2;
}


// Function: sub_2a3b0 @ 0x2a3b0
void sub_2a3b0(char *a0)
{
  char *v1; // stack - 0x38
  int8 v2; // fs_offset
  unsigned long v3; // stack - 0x30
  char *v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x10
  
  v6 = *(unsigned long *)(v2 + 0x28);
  v1 = "/bin/sh";
  v3 = 0x63997;
  v5 = 0;
  v4 = a0;
  execv("/bin/sh",&v1);
  sub_326c0(a0); // no-return
}


// Function: sub_2a400 @ 0x2a400
void sub_2a400(int4 a0,int4 a1)
{
  int4 v1;
  
  if (a0 == a1)
    return;
  if (close(a1)) {
    v1 = *__errno_location();
    if (v1 != 9) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,v1,dcgettext(NULL,"Cannot close",5));
      sub_163d0(); // no-return
    }
  }
  v1 = dup(a0);
  if (a1 != v1) {
    if (v1 <= -1) {
      v1 = *__errno_location();
      if (dat_84d50)
        (*dat_84d50)();
      error(0,v1,dcgettext(NULL,"Cannot dup",5));
      sub_163d0(); // no-return
    }
    abort(); // no-return
  }
  sub_d2c0(a0); // tail-call
  return;
}


// Function: sub_2a4c0 @ 0x2a4c0
void sub_2a4c0(int4 a0)
{
  uint4 v1; // stack - 0x24
  uint4 v2; // edi
  int8 v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  do {
    if (waitpid(a0,(int4 *)&v1,0) != -1) goto label_2a4fb;
  } while (*__errno_location() == 4);
  sub_32df0(dat_84b70);
label_2a4fb:
  v2 = v1 >> 8 & 0xff;
  if ('\x02' <= (char)(((uint1)v1 & 0x7f) + 1)) {
    raise(v1 & 0x7f);
    v2 = 0;
  }
  exit(v2); // no-return
}


// Function: sub_2a530 @ 0x2a530
void sub_2a530(void)
{
  char *v1;
  unsigned long v10; // stack - 0xb8
  unsigned long v11; // stack - 0x40
  char *v2; // rax
  char *v3; // rax
  unsigned long v4; // rdx
  char v5 [4];
  int8 v6; // stack - 0x128
  int8 v7; // fs_offset
  char **v8; // stack - 0x120
  int8 v9; // stack - 0x118
  
  v11 = *(unsigned long *)(v7 + 0x28);
  v9 = 1;
  v10 = __environ;
  v1 = (char *)sub_cdf0(v5);
  if (!v1) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"unable to run decompression program",5));
    sub_163d0(); // no-return
  }
  v4 = 0x2080e06;
  while( true ) {
    if (sub_382c0(v1,&v6,v4)) {
      if (dat_84d50)
        (*dat_84d50)();
      v3 = (char *)sub_354b0(&v6);
      error(0,0,dcgettext(NULL,"cannot split string \'%s\': %s",5),v1,v3);
      sub_163d0(); // no-return
    }
    memmove(v8,&v8[v9],v6 * 8);
    v8[v6] = "-d";
    execvp(*v8,v8);
    v8[v6] = NULL;
    v3 = (char *)sub_ce40(v5);
    if (!v3) break;
    if (dat_83b82 & 8) {
      if (dat_84d50)
        (*dat_84d50)();
      v2 = dcgettext(NULL,"cannot run %s",5);
      error(0,*__errno_location(),v2,v1);
      if (dat_83b82 & 8) {
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"trying %s",5),v3);
      }
    }
    v4 = 0x2080e0e;
    v1 = v3;
  }
  sub_326c0(v1); // no-return
}


// Function: sub_2a720 @ 0x2a720
void sub_2a720(char *a0,char *a1)
{
  if (!a1) {
    unsetenv(a0); // tail-call
    return;
  }
  if (!setenv(a0,a1,1))
    return;
  sub_16420(); // no-return
}


// Function: sub_2a750 @ 0x2a750
void sub_2a750(char *a0,uint8 a1,unsigned long a2)
{
  char *v1; // rax
  char v2 [40];
  
  v1 = (char *)sub_24270(a1,a2,v2);
  if (!setenv(a0,v1,1))
    return;
  sub_16420(); // no-return
}


// Function: sub_2a7b0 @ 0x2a7b0
void sub_2a7b0(char *a0,uint8 a1)
{
  char *v1; // rax
  char v2 [24];
  
  v1 = (char *)sub_43090(a1,v2);
  if (!setenv(a0,v1,1))
    return;
  sub_16420(); // no-return
}


// Function: sub_2a8f0 @ 0x2a8f0
void sub_2a8f0(char *a0,char a1,int8 a2)
{
  unsigned long v1;
  int4 v2;
  
  sub_2a720("TAR_VERSION","1.34");
  sub_2a720("TAR_ARCHIVE",(char *)*dat_84828);
  sub_2a7b0("TAR_VOLUME",((int8)dat_84828 - dat_84838 >> 3) + 1);
  sub_2a7b0("TAR_BLOCKING_FACTOR",(int8)dat_84ba0);
  v2 = dat_84400;
  if (!dat_84400)
    v2 = dat_84ba4;
  sub_2a720("TAR_FORMAT",(char *)sub_2c230(v2));
  sub_2a810((int4)a1);
  sub_2a870(*(unsigned int *)(a2 + 0x70));
  sub_2a720("TAR_FILENAME",a0);
  sub_2a720("TAR_REALNAME",*(char **)(a2 + 8));
  sub_2a720("TAR_UNAME",*(char **)(a2 + 0x20));
  sub_2a720("TAR_GNAME",*(char **)(a2 + 0x28));
  sub_2a750("TAR_ATIME",*(uint8 *)(a2 + 0xe8),*(unsigned long *)(a2 + 0xf0));
  sub_2a750("TAR_MTIME",*(uint8 *)(a2 + 0xf8),*(unsigned long *)(a2 + 0x100));
  sub_2a750("TAR_CTIME",*(uint8 *)(a2 + 0x108),*(unsigned long *)(a2 + 0x110));
  sub_2a7b0("TAR_SIZE",*(uint8 *)(a2 + 0x88));
  sub_2a7b0("TAR_UID",*(unsigned int *)(a2 + 0x74));
  sub_2a7b0("TAR_GID",*(unsigned int *)(a2 + 0x78));
  if (a1 != 'h') {
    if ('i' <= a1) { // branch-flip
      if (a1 == 'l') goto label_2aae0;
    }
    else if ((uint1)(a1 + 0x9eU) <= 1) {
      v1 = *(unsigned long *)(a2 + 0x80);
      sub_2a7b0("TAR_MINOR",gnu_dev_minor(v1));
      v1 = *(unsigned long *)(a2 + 0x80);
      sub_2a7b0("TAR_MAJOR",gnu_dev_major(v1));
      unsetenv("TAR_LINKNAME"); // tail-call
      return;
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    unsetenv("TAR_LINKNAME"); // tail-call
    return;
  }
label_2aae0:
  unsetenv("TAR_MINOR");
  unsetenv("TAR_MAJOR");
  sub_2a720("TAR_LINKNAME",*(char **)(a2 + 0x18)); // tail-call
  return;
}


// Function: sub_2ab10 @ 0x2ab10
unsigned int sub_2ab10(void)
{
  uint4 v1; // eax
  
  v1 = fstat(dat_84a34,(stat *)0x84740);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}


// Function: sub_2ab40 @ 0x2ab40
bool sub_2ab40(int8 a0)
{
  if ((dat_84818) && (dat_84818 == *(int8 *)(a0 + 0x58)))
    return *(int8 *)(a0 + 0x60) == dat_84810;
  return 0;
}


// Function: sub_2ab80 @ 0x2ab80
void sub_2ab80(void)
{
  if ((dat_84a34 <= 0x3fffffff) && ((dat_84758 & 0xf000) == 0x8000)) {
    dat_84818 = dat_84740;
    dat_84810 = dat_84748;
    return;
  }
  dat_84818 = 0;
  return;
}


// Function: sub_2abd0 @ 0x2abd0
void sub_2abd0(void) // return-dupe x2
{
  char *v1;
  stat v2; // stack - 0xa8
  
  v1 = (char *)*dat_84838;
  if (!strcmp(v1,"/dev/null")) {
    dat_84a30 = 1;
    return;
  }
  if ((((dat_84a34 <= 0x3fffffff) && ((dat_84758 & 0xf000) == 0x2000)) && (!stat("/dev/null",&v2))) && (dat_84740 == v2._0_8_)) {
    dat_84a30 = dat_84748 == v2._8_8_;
    return;
  }
  dat_84a30 = 0;
  return;
}


// Function: sub_2aca0 @ 0x2aca0
void sub_2aca0(int4 a0,bool a1) // return-dupe x2
{
  uint1 v1;
  char *v2; // rax
  uint8 v3; // rcx
  uint4 v4; // stack - 0x24
  
  if (!a0)
    return;
  do {
    if (waitpid(a0,(int4 *)&v4,0) != -1) {
      v1 = (uint1)v4;
      goto label_2acfd;
    }
  } while (*__errno_location() == 4);
  sub_32df0(dat_84b70);
  v1 = (uint1)v4;
label_2acfd:
  if ('\x02' <= (char)((v1 & 0x7f) + 1)) { // branch-flip
    if (((v4 & 0x7f) == 0xd) && (!a1))
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v2 = dcgettext(NULL,"Child died with signal %d",5);
    v3 = (uint8)(v4 & 0x7f);
  }
  else {
    if (!(char)(v4 >> 8))
      return;
    if (dat_84d50)
      (*dat_84d50)();
    v3 = (uint8)(v4 >> 8) & 0xff;
    v2 = dcgettext(NULL,"Child returned status %d",5);
  }
  error(0,0,v2,v3);
  sub_163d0(); // no-return
}


// Function: sub_2add0 @ 0x2add0
void sub_2add0(void)
{
  int4 v1; // eax
  char *v2; // rax
  int8 v3;
  
  v2 = getenv("SHELL");
  if (!v2)
    v2 = "/bin/sh";
  v1 = sub_24f80();
  if (v1) {
    do {
      if (waitpid(v1,&v3,0) != -1)
        return;
    } while (*__errno_location() == 4);
    sub_32df0(v2);
    return;
  }
  execlp(v2,"-sh",0x63ad0,0);
  sub_326c0(v2); // no-return
}


// Function: sub_2ae90 @ 0x2ae90
unsigned int sub_2ae90(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x1c) >> 8),*(int4 *)(a0 + 0x1c) == *(int4 *)(a1 + 0x1c));
}


// Function: sub_2aea0 @ 0x2aea0
unsigned int sub_2aea0(int8 a0,int8 a1)
{
  return CONCAT31((undefined3)((uint4)*(int4 *)(a1 + 0x20) >> 8),*(int4 *)(a0 + 0x20) == *(int4 *)(a1 + 0x20));
}


// Function: sub_2aeb0 @ 0x2aeb0
unsigned long sub_2aeb0(struct_3 *a0,struct_3 *a1)
{
  if (a1->field_0x0 != a0->field_0x0)
    return 0;
  return CONCAT71((undefined7)((uint8)a0->field_0x8 >> 8),a1->field_0x8 == a0->field_0x8);
}


// Function: sub_2aed0 @ 0x2aed0
unsigned long sub_2aed0(uint8 a0)
{
  int8 v1; // rax
  
  v1 = lseek(a0,0,1);
  if (0 <= v1)
    return ftruncate(a0 & 0xffffffff,v1); // tail-call
  return 0xffffffff;
}


// Function: sub_2af00 @ 0x2af00
uint8 sub_2af00(void)
{
  uint8 v1; // rax
  
  if (0x40000000 <= dat_84a34) {
    v1 = sub_33ea0(dat_84a34 + -0x40000000); // tail-call
    return v1;
  }
  v1 = sub_41120(dat_84a34,dat_83f58,dat_84b98); // tail-call
  return v1;
}


// Function: sub_2af40 @ 0x2af40
int4 sub_2af40(void)
{
  unsigned long *v1;
  int8 v10;
  unsigned int v11; // stack - 0x2c
  unsigned int v12; // stack - 0x24
  int4 v2;
  int4 *v3; // rax
  int8 v4; // rax
  uint8 v5;
  uint8 v6; // rbx
  unsigned int v7; // stack - 0x30
  unsigned int v8; // stack - 0x28
  char *v9;
  
  signal(0xd,1);
  sub_24fc0(&v7);
  v2 = sub_24f80();
  if (0 < v2) {
    dat_84a34 = v11;
    sub_d2c0(v7);
    return v2;
  }
  sub_48760(dcgettext(NULL,"tar (child)",5));
  signal(0xd,0);
  sub_2a400(v7,0);
  sub_d2c0(v11);
  v1 = dat_84838;
  if (dat_84d70) { // branch-flip
    v9 = (char *)*dat_84838;
label_2aff6:
    if (sub_2a340(v9)) {
      if (dat_84b84)
        sub_24710((char *)*dat_84838,1);
      v9 = (char *)*dat_84838;
      if (strcmp(v9,"-")) {
        dat_84a34 = creat(v9,0x1b6);
        if (dat_84a34 <= -1) {
          v3 = __errno_location();
          v2 = *v3;
          if (dat_84b84)
            sub_245d0();
          *v3 = v2;
          sub_327e0((char *)*dat_84838); // no-return
        }
        sub_2a400(dat_84a34,1);
      }
      sub_2a3b0(dat_84b70); // return-dupe, no-return
    }
  }
  else {
    dat_84d78 = strchr((char *)*dat_84838,0x3a);
    v9 = (char *)*v1;
    if (((!dat_84d78) || (dat_84d78 <= v9)) || (v5 = (int8)dat_84d78 - (int8)v9, memchr(v9,0x2f,v5))) goto label_2aff6;
  }
  sub_24fc0(&v8);
  v2 = sub_24f80();
  if (!v2) {
    sub_48760(dcgettext(NULL,"tar (grandchild)",5));
    sub_2a400(v12,1);
    sub_d2c0(v8);
    sub_2a3b0(dat_84b70);
  }
  sub_2a400(v8,0);
  sub_d2c0(v12);
  v1 = dat_84838;
  v9 = (char *)*dat_84838;
  if (!strcmp(v9,"-")) {
    dat_84a34 = 1;
    goto label_2b127;
  }
  if (dat_84d70) { // branch-flip
label_2b25f:
    dat_84a34 = creat(v9,0x1b6);
  }
  else {
    dat_84d78 = strchr(v9,0x3a);
    v9 = (char *)*v1;
    if (((!dat_84d78) || (dat_84d78 <= v9)) || (v5 = (int8)dat_84d78 - (int8)v9, memchr(v9,0x2f,v5))) goto label_2b25f;
    dat_84a34 = sub_338d0(v9,0x41,0x40000000,dat_84ab0);
  }
  if (dat_84a34 <= -1)
    sub_327e0((char *)*dat_84838); // no-return
label_2b127:
  v5 = dat_84b98;
  do {
    if (!v5)
      sub_2a4c0(v2); // return-dupe, no-return
    v6 = 0;
    v10 = dat_83f58;
    do {
      v4 = sub_4af10(0,v10,v5 - v6);
      if (v4 == -1)
        sub_329b0(dat_84b70); // no-return
      if (!v4) {
        if (!v6) {
          sub_2a4c0(v2);
        }
        memset((void *)(dat_83f58 + v6),0,dat_84b98 - v6);
        v5 = sub_2af00();
        if (dat_84b98 != v5) {
          sub_d6d0(v5); // no-return, return-dupe
        }
        sub_2a4c0(v2);
      }
      v6 += v4;
      v10 += v4;
      v5 = dat_84b98;
    } while (v6 < dat_84b98);
    v5 = sub_2af00();
    if (dat_84b98 != v5)
      sub_d6d0(v5);
  } while( true );
}


// Function: sub_2b310 @ 0x2b310
int4 sub_2b310(void)
{
  unsigned long *v1;
  unsigned int v10; // stack - 0x34
  int4 v2; // eax
  uint8 v3;
  unsigned int v4; // stack - 0x40
  unsigned int v5; // stack - 0x38
  uint8 v6;
  char *v7;
  int8 v8;
  unsigned int v9; // stack - 0x3c
  
  sub_24fc0(&v4);
  v2 = sub_24f80();
  if (0 < v2) {
    dat_84a34 = v4;
    sub_d2c0(v9);
    return v2;
  }
  sub_48760(dcgettext(NULL,"tar (child)",5));
  signal(0xd,0);
  sub_2a400(v9,1);
  sub_d2c0(v4);
  v1 = dat_84838;
  v7 = (char *)*dat_84838;
  if (strcmp(v7,"-")) {
    if (!dat_84d70) {
      dat_84d78 = strchr(v7,0x3a);
      v7 = (char *)*v1;
      if (((dat_84d78) && (v7 < dat_84d78)) && (v3 = (int8)dat_84d78 - (int8)v7, !memchr(v7,0x2f,v3))) goto label_2b3f8;
    }
    if (sub_2a340(v7)) {
      dat_84a34 = open((char *)*dat_84838,0,0x1b6);
      if (0 <= dat_84a34) {
        sub_2a400(dat_84a34,0);
        sub_2a530(); // no-return
      }
      sub_327e0((char *)*dat_84838); // return-dupe, no-return
    }
  }
label_2b3f8:
  sub_24fc0(&v5);
  v2 = sub_24f80();
  if (!v2) {
    sub_48760(dcgettext(NULL,"tar (grandchild)",5));
    sub_2a400(v5,0);
    sub_d2c0(v10);
    sub_2a530(); // no-return
  }
  sub_2a400(v10,1);
  sub_d2c0(v5);
  v1 = dat_84838;
  v7 = (char *)*dat_84838;
  if (!strcmp(v7,"-")) {
    dat_84a34 = 0;
    goto label_2b450;
  }
  if (dat_84d70) { // branch-flip
label_2b584:
    dat_84a34 = open(v7,0,0x1b6);
  }
  else {
    dat_84d78 = strchr(v7,0x3a);
    v7 = (char *)*v1;
    if (((!dat_84d78) || (dat_84d78 <= v7)) || (v3 = (int8)dat_84d78 - (int8)v7, memchr(v7,0x2f,v3))) goto label_2b584;
    dat_84a34 = sub_338d0(v7,0,0x40000000,dat_84ab0);
  }
  if (dat_84a34 <= -1)
    sub_327e0((char *)*dat_84838);
label_2b450:
  while( true ) {
    sub_cd10();
    while( true ) {
      if (0x40000000 <= dat_84a34) { // branch-flip
        v3 = sub_33da0(dat_84a34 + -0x40000000,dat_83f58,dat_84b98);
        v8 = dat_83f58;
      }
      else {
        v3 = sub_4af10(dat_84a34,dat_83f58,dat_84b98);
        v8 = dat_83f58;
      }
      dat_83f58 = v8;
      if (v3 != 0xffffffffffffffff) break;
      sub_d300();
    }
    if (!v3) break;
    do {
      v6 = 0x200;
      if (v3 <= 0x200)
        v6 = v3;
      if (sub_41120(1,v8,v6) != v6)
        sub_32e10(dat_84b70);
      v8 += v6;
      v3 -= v6;
    } while (v3);
  }
  sub_d2c0(1);
  sub_2a4c0(v2); // no-return
}


// Function: sub_2b640 @ 0x2b640
unsigned int sub_2b640(char *a0,char a1,int8 a2)
{
  unsigned int v1; // stack - 0x28
  unsigned int v2; // stack - 0x24
  
  sub_24fc0(&v1);
  dat_84728 = signal(0xd,1);
  dat_84730 = sub_24f80();
  if (dat_84730) {
    sub_d2c0(v1);
    return v2;
  }
  sub_2a400(v1,0);
  sub_d2c0(v2);
  sub_2a8f0(a0,(int4)a1,a2);
  sub_2a3b0(dat_84a60); // no-return
}


// Function: sub_2b6f0 @ 0x2b6f0
void sub_2b6f0(void) // return-dupe
{
  uint4 v1;
  uint4 v2; // stack - 0x24
  int4 v3;
  uint8 v4; // r12
  
  v3 = dat_84730;
  if (dat_84730 <= -1)
    return;
  signal(0xd,dat_84728);
  while( true ) {
    if (waitpid(v3,(int4 *)&v2,0) != -1) {
      v1 = v2 & 0x7f;
      if (v1) { // branch-flip
        if ('\x02' <= (char)((char)v1 + '\x01')) { // branch-flip
          if (dat_84d50) {
            (*dat_84d50)();
            v1 = v2 & 0x7f;
          }
          v4 = (uint8)dat_84730;
          error(0,0,dcgettext(NULL,"%lu: Child terminated on signal %d",5),v4,v1);
        }
        else {
          if (dat_84d50)
            (*dat_84d50)();
          v4 = (uint8)dat_84730;
          error(0,0,dcgettext(NULL,"%lu: Child terminated on unknown reason",5),v4);
          dat_84d58 = 2;
        }
      }
      else if ((!dat_84a59) && (v1 = v2 >> 8, (char)(v2 >> 8))) {
        if (dat_84d50) {
          (*dat_84d50)();
          v1 = v2 >> 8;
        }
        v4 = (uint8)dat_84730;
        error(0,0,dcgettext(NULL,"%lu: Child returned status %d",5),v4,v1 & 0xff);
        dat_84d58 = 2;
      }
      dat_84730 = 0xffffffff;
      return;
    }
    if (*__errno_location() != 4) break;
    v3 = dat_84730;
  }
  dat_84730 = 0xffffffff;
  sub_32df0(dat_84a60);
  return;
}


// Function: sub_2b8b0 @ 0x2b8b0
uint4 sub_2b8b0(unsigned long *a0,int4 a1)
{
  int4 v1;
  int4 v10; // stack - 0x4c
  FILE *v2; // rax
  int4 v3; // stack - 0x50
  char *v4; // stack - 0x60
  unsigned long v5; // stack - 0x58
  uint4 v6; // stack - 0x64
  char v7 [24];
  uint8 v8; // rdi
  int4 v9; // eax
  
  sub_24fc0(&v3);
  dat_84720 = signal(0xd,1);
  v1 = sub_24f80();
  if (!v1) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*a0,1);
    setenv("TAR_VOLUME",(char *)sub_43090((int8)a1,v7),1);
    v8 = (uint8)dat_84ba0;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_43090(v8,v7),1);
    setenv("TAR_SUBCOMMAND",(char *)sub_2c7e0(dat_84ba8),1);
    v1 = dat_84400;
    if (!dat_84400)
      v1 = dat_84ba4;
    setenv("TAR_FORMAT",(char *)sub_2c230(v1),1);
    setenv("TAR_FD",(char *)sub_43090((int8)v10,v7),1);
    sub_d2c0(v3);
    sub_2a3b0(dat_84b48); // no-return
  }
  v4 = NULL;
  v5 = 0;
  sub_d2c0(v10);
  v2 = fdopen(v3,"r");
  v9 = (int4)getline(&v4,&v5,v2);
  fclose(v2);
  if ((1 <= v9) && (v4[(int8)v9 + -1] == '\n')) {
    v4[(int8)v9 + -1] = '\0';
    v9 -= 1;
  }
  while (waitpid(v1,(int4 *)&v6,0) == -1) {
    if (*__errno_location() != 4) {
      signal(0xd,dat_84720);
      sub_32df0(dat_84b48);
      return 0xffffffff;
    }
  }
  signal(0xd,dat_84720);
  if (v6 & 0x7f) {
    free(v4);
    return 0xffffffff;
  }
  if ((!(char)(v6 >> 8)) && (1 <= v9)) {
    *a0 = v4;
    return 0;
  }
  free(v4);
  return v6 >> 8 & 0xff;
}


// Function: sub_2bb40 @ 0x2bb40
void sub_2bb40(char *a0,char *a1,int4 a2)
{
  int4 v1;
  int8 v2;
  char v3 [24];
  uint8 v4; // rdi
  
  v1 = sub_24f80();
  if (!v1) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",a1,1);
    setenv("TAR_CHECKPOINT",(char *)sub_43090((int8)a2,v3),1);
    v4 = (uint8)dat_84ba0;
    setenv("TAR_BLOCKING_FACTOR",(char *)sub_43090(v4,v3),1);
    setenv("TAR_SUBCOMMAND",(char *)sub_2c7e0(dat_84ba8),1);
    v1 = dat_84400;
    if (!dat_84400)
      v1 = dat_84ba4;
    setenv("TAR_FORMAT",(char *)sub_2c230(v1),1);
    sub_2a3b0(a0); // no-return
  }
  do {
    if (waitpid(v1,&v2,0) != -1)
      return;
  } while (*__errno_location() == 4);
  sub_32df0(a0);
  return;
}


// Function: sub_2bcb0 @ 0x2bcb0
unsigned long sub_2bcb0(int4 a0)
{
  return *(unsigned long *)((int8)a0 * 8 + 0x84bc0);
}


// Function: sub_2bcc0 @ 0x2bcc0
bool sub_2bcc0(int4 a0)
{
  int4 *v1; // rax
  
  v1 = (int4 *)sub_2bcb0(a0);
  if (v1)
    return *v1 == 1;
  return 0;
}


// Function: sub_2bce0 @ 0x2bce0
int8 * sub_2bce0(int8 *a0,int4 a1)
{
  for (; (((*a0 || ((int4)a0[1])) || (a0[2])) || (((int4)a0[3] || (a0[4])))); a0 = &a0[6]) {
    if (a1 == (int4)a0[1])
      return a0;
  }
  return NULL;
}


// Function: sub_2bd20 @ 0x2bd20
bool sub_2bd20(struct_72 *a0,struct_72 *a1) // early-return
{
  char *v1;
  char *v2;
  
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  if (a0->field_0x0 == 1)
    return 1;
  v1 = a1->field_0x8;
  v2 = a0->field_0x8;
  return strcmp(v2,v1) == 0;
}


// Function: sub_2bd60 @ 0x2bd60
unsigned long sub_2bd60(uint4 a0,struct_104 *a1)
{
  unsigned int v1;
  unsigned long v2;
  unsigned int *v3; // rax
  char *v4;
  uint8 v5;
  
  v5 = 0x20;
  v4 = a1->field_0x8;
  if (v4)
    v5 = strlen(v4) + 0x21;
  v3 = (unsigned int *)sub_4c5b0(v5);
  v4 = NULL;
  if (a1->field_0x8)
    v4 = strcpy((char *)&v3[8],a1->field_0x8);
  v1 = a1->field_0x0;
  *(char **)&v3[2] = v4;
  *v3 = v1;
  *(unsigned long *)&v3[4] = a1->field_0x10;
  v2 = *(unsigned long *)((uint8)a0 * 8 + 0x84bc0);
  *(unsigned int **)((uint8)a0 * 8 + 0x84bc0) = v3;
  *(unsigned long *)&v3[6] = v2;
  return v2;
}


// Function: sub_2bde0 @ 0x2bde0
void sub_2bde0(char *a0)
{
  uint8 v1; // rbx
  char *v2;
  
  v2 = "literal";
  v1 = 0;
  do {
    if (!strcmp(a0,v2)) {
      sub_4a590(NULL,v1 & 0xffffffff); // tail-call
      return;
    }
    v1 += 1;
    v2 = *(char **)(v1 * 8 + 0x80680);
  } while (v2);
  if (dat_84d50)
    (*dat_84d50)();
  v2 = dat_84dd0;
  error(0,0,dcgettext(NULL,"Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list.",5),a0,v2);
  sub_163d0(); // no-return
}


// Function: sub_2be90 @ 0x2be90
void sub_2be90(unsigned int a0)
{
  sub_cd50();
  sub_d150();
  signal(a0,sub_2be90); // tail-call
  return;
}


// Function: sub_2c230 @ 0x2c230
char * sub_2c230(int4 a0)
{
  int8 v1; // rax
  int8 v2;
  int4 v3;
  char *v4;
  
  v3 = 1;
  v4 = "v7";
  v2 = 0x7f9a0;
  while( true ) {
    if (a0 == v3)
      return v4;
    v4 = *(char **)(v2 + 0x10);
    v1 = v2 + 0x10;
    if (!v4) break;
    v3 = *(int4 *)(v2 + 0x18);
    v2 = v1;
  }
  return "unknown?";
}


// Function: sub_2c280 @ 0x2c280
void sub_2c280(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_4cc50("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",sub_2c230(6,a1,a2,0x14),0x6863b);
  return;
}


// Function: sub_2c2d0 @ 0x2c2d0
unsigned long sub_2c2d0(int4 a0,unsigned long a1)
{
  char *v1; // rax
  char *v10; // stack - 0x58
  uint8 v11; // stack - 0x48
  uint1 v12; // stack - 0x28
  uint8 v2; // rax
  unsigned short *v3;
  char *v4; // rax
  undefined8 v5;
  char *v6;
  char v7 [24]; // stack - 0x78
  char *v8; // rsi
  unsigned short *v9; // stack - 0x60
  
  if (a0 != 0x94) { // branch-flip
    switch(a0) { // branch-flip
      default:
        return a1;
      case 0x4a:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "xz";
        break;
      case 0x5a:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "compress";
        break;
      case 0x6a:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "bzip2";
        break;
      case 0x7a:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "gzip";
        break;
      case 0x95:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "lzma";
        break;
      case 0x96:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "lzop";
        break;
      case 0xcf:
        v1 = dcgettext(NULL,"filter the archive through %s",5);
        v8 = "zstd";
        break;
      case 0x2000004:
        _obstack_begin((obstack *)v7,0,0,sub_4c5b0,dat_80fb8);
        v1 = dcgettext(NULL,"Valid arguments for the --quoting-style option are:",5);
        v2 = strlen(v1);
        if ((uint8)((int8)v10 - (int8)v9) < v2)
          _obstack_newchunk((obstack *)v7,v2);
        memcpy(v9,v1,v2);
        v9 = (unsigned short *)((int8)v9 + v2);
        v3 = v9;
        if ((uint8)((int8)v10 - (int8)v9) < 2) {
          _obstack_newchunk((obstack *)v7,2);
          v3 = v9;
        }
        *v3 = 0xa0a;
        v9 = &v9[1];
        sub_2c050(v7);
        v1 = dcgettext(NULL,"\n*This* tar defaults to:\n",5);
        v2 = strlen(v1);
        if ((uint8)((int8)v10 - (int8)v9) < v2)
          _obstack_newchunk((obstack *)v7,v2);
        memcpy(v9,v1,v2);
        v9 = (void *)((int8)v9 + v2);
        v1 = (char *)sub_2c280();
        v2 = strlen(v1);
        if ((uint8)((int8)v10 - (int8)v9) < v2)
          _obstack_newchunk((obstack *)v7,v2);
        memcpy(v9,v1,v2);
        v9 = (char *)((int8)v9 + v2);
        v3 = v9;
        if (v9 == (unsigned short *)v10) {
          _obstack_newchunk((obstack *)v7,1);
          v3 = v9;
        }
        v9 = (char *)((int8)v3 + 1);
        *(char *)v3 = 10;
        if ((unsigned short *)v10 == v9)
          _obstack_newchunk((obstack *)v7,1);
        v5 = v7._16_8_;
        *(char *)v9 = 0;
        if ((char *)((int8)v9 + 1) == v7._16_8_)
          v12 |= 2;
        v4 = (char *)((uint8)&((char *)((int8)v9 + 1))[v11] & ~v11);
        v6 = v10;
        if ((uint8)((int8)v4 - v7._8_8_) <= (uint8)((int8)v10 - v7._8_8_))
          v6 = v4;
        v7._16_8_ = v6;
        v9 = (unsigned short *)v6;
        v5 = sub_4c7a0(v5);
        _obstack_free((obstack *)v7,NULL);
        return v5;
      
    }
  }
  else {
    v1 = dcgettext(NULL,"filter the archive through %s",5);
    v8 = "lzip";
  }
  v5 = sub_4cc50(v1,v8); // tail-call
  return v5;
}


// Function: sub_2c7e0 @ 0x2c7e0
char * sub_2c7e0(unsigned int a0)
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


// Function: sub_2c8a0 @ 0x2c8a0
int4 sub_2c8a0(char *a0)
{
  int4 v1; // eax
  unsigned long *v2;
  char *v3;
  
  v2 = (unsigned long *)0x7f8e0;
  v1 = strncmp(a0,"SIG",3);
  v3 = &a0[3];
  if (v1)
    v3 = a0;
  v1 = strcmp("USR1",v3);
  while( true ) {
    if (!v1) {
      v1 = *(unsigned int *)&v2[1];
      return v1;
    }
    v2 = &v2[2];
    if (v2 == (unsigned long *)0x7f930) break;
    v1 = strcmp((char *)*v2,v3);
  }
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Unknown signal name: %s",5),a0);
  sub_163d0(); // no-return
}


// Function: sub_2c950 @ 0x2c950
void sub_2c950(char *a0)
{
  sub_2be80(sub_2c8a0(a0)); // tail-call
  return;
}


// Function: sub_2c970 @ 0x2c970
void sub_2c970(int4 a0)
{
  argp_help(0x81b20,stderr,4,dat_84dd0);
  sub_3e440();
  exit(a0); // no-return
}


// Function: sub_2c9b0 @ 0x2c9b0
void sub_2c9b0(char *a0)
{
  char *v1;
  
  if (!dat_84c10) {
    dat_84c10 = a0;
    return;
  }
  if (dat_84d50)
    (*dat_84d50)();
  v1 = dat_84c10;
  error(0,0,dcgettext(NULL,"Options \'%s\' and \'%s\' both want standard input",5),v1,a0);
  sub_2c970(2); // no-return
}


// Function: sub_2ca20 @ 0x2ca20
bool sub_2ca20(char *a0,char *a1)
{
  int4 v1;
  int4 v2; // eax
  unsigned long v3; // stack - 0x28
  char *v4; // stack - 0x30
  bool v5;
  
  if (!dat_84bb0) {
    if ((dat_84a34) && (!dat_84c10)) {
      sub_2c9b0("-w");
      dat_84bb0 = stdin;
    }
    else {
      dat_84bb0 = fopen("/dev/tty","r");
      if (!dat_84bb0)
        sub_327e0("/dev/tty"); // no-return
    }
  }
  __fprintf_chk(dat_83f28,1,"%s %s?",a0,(char *)sub_4ac50(a1));
  fflush_unlocked(dat_83f28);
  if (dat_84bac) // branch-flip
    v5 = 0;
  else {
    v4 = NULL;
    v3 = 0;
    if (0 <= getline(&v4,&v3,dat_84bb0)) { // branch-flip
      v2 = rpmatch(v4);
      v1 = dat_84bac;
      v5 = 0 < v2;
      free(v4);
      if (!v1)
        return v5;
    }
    else {
      dat_84bac = 1;
      free(v4);
      v5 = 0;
    }
  }
  fputc_unlocked(10,dat_83f28);
  fflush_unlocked(dat_83f28);
  return v5;
}


// Function: sub_2cbe0 @ 0x2cbe0
unsigned int sub_2cbe0(int8 a0,unsigned long a1,char *a2,struct_49 *a3) // return-dupe x2
{
  unsigned long v1;
  uint8 v2;
  uint8 v3;
  char *v4; // rax
  char *v5; // rax
  unsigned long *v6; // rax
  stat v7;
  char v8 [16];
  
  if (2 <= (uint1)(*a2 - 0x2eU)) {
    if (!sub_486d0(a3,a2,NULL)) {
      if (dat_84d50)
        (*dat_84d50)();
      v4 = (char *)sub_4ac50(a2);
      v5 = (char *)sub_22080(a3->field_0x0,a3->field_0x8,0);
      error(0,0,dcgettext(NULL,"Substituting %s for unknown date format %s",5),v5,v4);
      a3->field_0x8 = 0;
      return 1;
    }
    v6 = (unsigned long *)sub_4c5b0(0x28);
    v2 = a3->field_0x0;
    v3 = a3->field_0x8;
    v6[3] = a1;
    v6[1] = v2;
    v6[2] = v3;
    v6[4] = sub_4c7a0(a2);
    v1 = *(unsigned long *)(a0 + 8);
    *(unsigned long **)(a0 + 8) = v6;
    *v6 = v1;
    return 0;
  }
  if (!stat(a2,&v7)) {
    v8 = sub_4bd20(&v7);
    a3->field_0x0 = SUB168(v8,0);
    a3->field_0x8 = SUB168(v8,8);
    return 0;
  }
  sub_32d30(a2);
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"Date sample file not found",5));
  sub_2c970(2); // no-return
}


// Function: sub_2cd70 @ 0x2cd70
unsigned long sub_2cd70(int8 a0,char *a1)
{
  uint8 v1;
  unsigned long v10; // stack - 0xd8
  char v11 [40];
  uint8 v12; // rax
  void *v13;
  void *v14; // stack - 0x100
  unsigned long v15; // stack - 0xd0
  char *v16; // stack - 0xb0
  char *v17; // stack - 0xa8
  uint8 v18; // stack - 0x98
  uint1 v19; // stack - 0x78
  uint1 v2;
  char v3;
  uint8 v4; // rax
  char *v5;
  undefined8 v6;
  int8 *v7; // rax
  char *v8;
  char v9 [24]; // stack - 0xc8
  
  _obstack_begin((obstack *)v9,0,0,sub_4c5b0,dat_80fb8);
  do {
    v5 = v16;
    if (!*a1) {
label_2ce53:
      if (v5 == v17) {
        _obstack_newchunk((obstack *)v9,1);
        v5 = v16;
      }
      v6 = v9._16_8_;
      *v5 = '\0';
      if (&v5[1] == v9._16_8_)
        v19 |= 2;
      v5 = (char *)((uint8)&(&v5[1])[v18] & ~v18);
      v9._16_8_ = v17;
      if ((uint8)((int8)v5 - v9._8_8_) <= (uint8)((int8)v17 - v9._8_8_))
        v9._16_8_ = v5;
      v16 = v9._16_8_;
      v6 = sub_4c7a0(v6);
      _obstack_free((obstack *)v9,NULL);
      return v6;
    }
    v12 = (int8)v17 - (int8)v16;
    v4 = strcspn(a1,",");
    v14 = memchr(a1,0x3d,v4);
    if (v14) { // branch-flip
      v1 = (int8)v14 + (1U - (int8)a1);
      if (v12 < v1) {
        _obstack_newchunk((obstack *)v9,v1);
        v5 = v16;
      }
      memcpy(v5,a1,v1);
      v12 = v4 - v1;
      v5 = &v16[v1];
      v13 = (void *)((int8)v14 + 1);
      v2 = *(uint1 *)((int8)v14 + 1);
      v16 = v5;
      if (v2) {
        v7 = __ctype_b_loc();
        do {
          if (!(*(uint1 *)(*v7 + 1 + (uint8)v2 * 2) & 0x20)) {
            if ((v2 == 0x7b) && (*(char *)((int8)v13 + (v12 - 1)) == '}')) {
              v5 = (char *)sub_4c5b0(v12);
              memcpy(v5,(void *)((int8)v13 + 1),v12 - 2);
              v5[v12 - 2] = '\0';
              if (sub_2cbe0(a0,"--pax-option",v5,&v10)) { // branch-flip
                if ((uint8)((int8)v17 - (int8)v16) < v12)
                  _obstack_newchunk((obstack *)v9,v12);
                memcpy(v16,v13,v12);
                v16 = &v16[v12];
              }
              else {
                v8 = (char *)sub_24270(v10,v15,v11);
                v12 = strlen(v8);
                if ((uint8)((int8)v17 - (int8)v16) < v12)
                  _obstack_newchunk((obstack *)v9,v12);
                memcpy(v16,v8,v12);
                v16 = &v16[v12];
              }
              free(v5);
              v5 = v16;
              goto label_2ce3f;
            }
            break;
          }
          v2 = *(uint1 *)((int8)v13 + 1);
          v13 = (void *)((int8)v13 + 1);
          v12 -= 1;
        } while (v2);
      }
      if ((uint8)((int8)v17 - (int8)v5) < v12) {
        _obstack_newchunk((obstack *)v9,v12);
        v5 = v16;
      }
      memcpy(v5,v13,v12);
      v3 = a1[v4];
      v5 = &v16[v12];
      v8 = &v16[v12];
    }
    else {
      if (v12 < v4) {
        _obstack_newchunk((obstack *)v9,v4);
        v5 = v16;
      }
      memcpy(v5,a1,v4);
      v16 = &v16[v4];
      v5 = v16;
label_2ce3f:
      v3 = a1[v4];
      v8 = v16;
    }
    v16 = v5;
    if (!v3) {
      v5 = v16;
      v16 = v8;
      goto label_2ce53;
    }
    if (v17 == v16) {
      v16 = v8;
      _obstack_newchunk((obstack *)v9,1);
      v3 = a1[v4];
    }
    v5 = &v16[1];
    a1 = &(&a1[v4])[1];
    *v16 = v3;
    v16 = v5;
  } while( true );
}


// Function: sub_2d160 @ 0x2d160
void sub_2d160(int4 a0)
{
  if ((dat_84ba8) && (dat_84ba8 != a0)) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"You may not specify more than one \'-Acdtrux\', \'--delete\' or  \'--test-label\' option",5));
    sub_2c970(2); // no-return
  }
  dat_84ba8 = a0;
  return;
}


// Function: sub_2d1b0 @ 0x2d1b0
void sub_2d1b0(char *a0,char *a1)
{
  if (dat_84d50)
    (*dat_84d50)();
  error(0,0,dcgettext(NULL,"\'%s\' cannot be used with \'%s\'",5),a0,a1);
  sub_2c970(2); // no-return
}


// Function: sub_2d200 @ 0x2d200
void sub_2d200(uint4 a0,struct_104 *a1)
{
  struct_72 *v1; // rax
  
  v1 = (struct_72 *)sub_2bd60(9,a1);
  if (v1) {
    if ((sub_2bd20(a1,v1)) && (dat_84b34 != a0))
      sub_2d1b0(*(char **)((int8)(int4)a0 * 8 + 0x7f8a0),*(char **)((uint8)dat_84b34 * 8 + 0x7f8a0)); // no-return
  }
  dat_84b34 = a0;
  return;
}


// Function: sub_2d260 @ 0x2d260
void sub_2d260(char *a0,struct_104 *a1)
{
  int4 *v1; // rax
  
  v1 = (int4 *)sub_2bd60(0,a1);
  if (dat_84b70) {
    if ((strcmp(dat_84b70,a0)) && (*v1 == 1)) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"Conflicting compression options",5));
      sub_2c970(2); // no-return
    }
  }
  dat_84b70 = a0;
  return;
}


// Function: sub_2d2e0 @ 0x2d2e0
void sub_2d2e0(char *a0)
{
  int8 v1; // rbx
  char *v2;
  
  v2 = "v7";
  v1 = 0x7f9a0;
  do {
    if (!strcmp(v2,a0)) {
      dat_84ba4 = *(unsigned int *)(v1 + 8);
      return;
    }
    v2 = *(char **)(v1 + 0x10);
    v1 += 0x10;
  } while (v2);
  if (dat_84d50)
    (*dat_84d50)();
  v2 = (char *)sub_4aa70(a0);
  error(0,0,dcgettext(NULL,"%s: Invalid archive format",5),v2);
  sub_2c970(2); // no-return
}


// Function: sub_2d370 @ 0x2d370
void sub_2d370(int4 a0)
{
  if (a0 != 1) {
    dat_84a98 = a0;
    return;
  }
  sub_2d2e0("posix");
  dat_84a98 = a0;
  return;
}


// Function: sub_2d3a0 @ 0x2d3a0
int4 sub_2d3a0(int4 a0,char *a1,int8 *a2) // switch-return x115, return-dupe x3
{
  int8 *v1;
  char *v10;
  unsigned long *v11;
  char v2;
  int4 *v3;
  uint8 v4;
  int8 v5;
  void *v6; // rax
  int8 v7; // rdx
  char *v8; // stack - 0x38
  uint8 v9; // stack - 0x40
  
  v11 = (unsigned long *)a2[5];
  if (0xcf < a0) {
    if (a0 != 0x1000003) {
      if (a0 == 0x1000005) {
        v3 = (int4 *)*v11;
        if (*v3 != 2) { // branch-flip
          if (!*v3) {
            v10 = *(char **)&v3[2];
            error(0,0,dcgettext(NULL,"error parsing %s",5),v10);
          }
        }
        else {
          v4 = *(uint8 *)&v3[4];
          v10 = *(char **)&v3[2];
          error(0,0,dcgettext(NULL,"%s:%lu: location of the error",5),v10,v4);
        }
        exit(0x40); // no-return
      }
      return 7;
    }
    if (!*(int8 **)(*a2 + 0x20))
      return 0;
    if (!**(int8 **)(*a2 + 0x20))
      return 0;
    v7 = 0x20;
    v5 = 0;
    while( true ) {
      *(unsigned long **)(a2[6] + v5) = v11;
      v5 += 8;
      v1 = (int8 *)(*(int8 *)(*a2 + 0x20) + v7);
      v7 += 0x20;
      if (!*v1) break;
      v11 = (unsigned long *)a2[5];
    }
    return 0;
  }
  if (a0 < 0)
    return 7;
  switch(a0) {
    case 0:
      sub_26b50(a1);
      return a0;
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
      argp_error(a2,dcgettext(NULL,"Options \'-[0-7][lmh]\' not supported by *this* tar",5));
      exit(0x40); // no-return
    case 0x41:
      sub_2d160(2);
      return 0;
    case 0x42:
      dat_84ab9 = 1;
      return 0;
    case 0x46:
      dat_84b10 = 1;
      dat_84b48 = a1;
      return 0;
    case 0x47:
      dat_84b50 = 1;
      return 0;
    case 0x48:
      sub_2d2e0(a1);
      return 0;
    case 0x49:
      sub_2d260(a1,(struct_104 *)*v11);
      return 0;
    case 0x4a:
      sub_2d260("xz",(struct_104 *)*v11);
      return 0;
    case 0x4b:
      sub_2bd60(5,(struct_104 *)*v11);
      sub_27190(a1);
      return 0;
    case 0x4c:
      if (!sub_4c870(a1,&v8,10,&v9,"bBcGgkKMmPTtw")) {
        if ((a1 < v8) && (v2 = v8[-1], !strchr("bBcGgkKMmPTtw",(int4)v2))) {
          dat_84a70 = (float8)v9 * dat_66d60;
          dat_84b10 = 1;
          return 0;
        }
        if (0 <= (int8)v9) {
          dat_84a70 = (float8)(int8)v9;
          dat_84b10 = 1;
          return 0;
        }
        dat_84a70 = (float8)v9;
        dat_84b10 = 1;
        return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid tape length";
      break;
    case 0x4d:
      dat_84b10 = 1;
      return 0;
    case 0x4e:
      dat_84b8c = 1;
      if (dat_84b08 <= -1) {
        v10 = "--after-date";
        sub_2cbe0(v11,v10,a1,(struct_49 *)0x84b00); // return-dupe
        sub_2bd60(3,(struct_104 *)*v11);
        return 0;
      }
      goto label_2e380;
    case 0x4f:
      dat_84a6a = 1;
      return 0;
    case 0x50:
      sub_2bd60(8,(struct_104 *)*v11);
      dat_84b92 = 1;
      return 0;
    case 0x52:
      dat_84b7c = 1;
      return 0;
    case 0x53:
      dat_84a88 = 1;
      return 0;
    case 0x55:
      sub_2d200(3,(struct_104 *)*v11);
      return 0;
    case 0x56:
      dat_84a40 = a1;
      return 0;
    case 0x57:
      sub_2bd60(4,(struct_104 *)*v11);
      dat_84a50 = 1;
      return 0;
    case 0x5a:
      sub_2d260("compress",(struct_104 *)*v11);
      return 0;
    case 0x61:
      *(char *)((int8)v11 + 0x12) = 1;
      return 0;
    case 0x62:
      if ((((!sub_4c870(a1,NULL,10,&v8,"")) && (dat_84ba0 = (int4)v8, (char *)(int8)dat_84ba0 == v8)) && (1 <= dat_84ba0)) && (dat_84b98 = (int8)((int8)v8 << 9), v8 == (char *)((uint8)v8 & 0x7fffffffffffff)))
        return 0;
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid blocking factor";
      break;
    case 99:
      sub_2d160(3);
      return 0;
    case 100:
      sub_2d160(5);
      return 0;
    case 0x66:
      v5 = dat_84838;
      if (dat_84830 == dat_84c18) {
        v5 = sub_4c680(dat_84838,(uint8 *)0x84c18,8);
        dat_84838 = v5;
      }
      *(char **)(v5 + dat_84830 * 8) = a1;
      dat_84830 += 1;
      return 0;
    case 0x67:
      sub_2bd60(2,(struct_104 *)*v11);
      dat_84b28 = a1;
      dat_84b50 = 1;
      dat_84b8c = 1;
      return 0;
    case 0x68:
      dat_84b69 = 1;
      return 0;
    case 0x69:
      dat_84b51 = 1;
      return 0;
    case 0x6a:
      sub_2d260("bzip2",(struct_104 *)*v11);
      return 0;
    case 0x6b:
      sub_2d200(4,(struct_104 *)*v11);
      return 0;
    case 0x6c:
      dat_84c20 = 1;
      return 0;
    case 0x6d:
      dat_84a68 = 1;
      return 0;
    case 0x6e:
      dat_847fc = 1;
      return 0;
    case 0x6f:
      *(char *)&v11[2] = 1;
      return 0;
    case 0x70:
      dat_84aa4 = 1;
      return 0;
    case 0x72:
      sub_2d160(1);
      return 0;
    case 0x73:
      sub_2bd60(6,(struct_104 *)*v11);
      dat_84aac = 1;
      return 0;
    case 0x74:
      sub_2d160(7);
      dat_84a54 += 1;
      return 0;
    case 0x75:
      sub_2d160(8);
      return 0;
    case 0x76:
      dat_84a54 += 1;
      dat_83b80 |= 0x583000;
      return 0;
    case 0x77:
      dat_84b40 = 1;
      return 0;
    case 0x78:
      sub_2d160(6);
      return 0;
    case 0x7a:
      sub_2d260("gzip",(struct_104 *)*v11);
      return 0;
    case 0x80:
      sub_2d2e0("posix");
      dat_84a9c = 1;
      return 0;
    case 0x81:
      if (!a1) {
        dat_84b88 = 1;
        return 0;
      }
      v5 = sub_39670("--atime-preserve",a1,(int8 *)0x7f980,(void *)0x66a90,4,dat_83dd8);
      dat_84b88 = *(unsigned int *)(v5 * 4 + 0x66a90);
      return 0;
    case 0x82:
      if (!a1) {
        dat_84b84 = 1;
        return 0;
      }
      v11[4] = a1;
      dat_84b84 = 1;
      return 0;
    case 0x83:
      dat_84b20 = 1;
      return 0;
    case 0x84:
      if (!a1) {
        dat_84b78 = 10;
        return 0;
      }
      if (*a1 == '.') {
        a1 = &a1[1];
        sub_10490(".");
      }
      dat_84b78 = (unsigned int)strtoul(a1,&v8,0);
      if (!*v8)
        return 0;
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "--checkpoint value is not an integer";
      goto label_2e366;
    case 0x85:
      sub_10490(a1);
      return 0;
    case 0x86:
      dat_84af0 = 2;
      return 0;
    case 0x87:
      dat_847e0 = 1;
      return 0;
    case 0x88:
      dat_84b68 = 1;
      return 0;
    case 0x89:
      sub_2d160(4);
      return 0;
    case 0x8a:
      dat_84d70 = 1;
      return 0;
    case 0x8b:
      dat_84b90 = 1;
      return 0;
    case 0x8c:
      v5 = sub_2bf00(a1,(int8 *)0x84b58);
      if (v5 != -1) {
        dat_84b54 = (int4)v5;
        return 0;
      }
      dat_84b54 = 0xffffffff;
      if (!dat_84b58) {
        dat_84b54 = 0xffffffff;
        return 0;
      }
      sub_26aa0(dat_84b58,(unsigned int *)0x84b54);
      return 0;
    case 0x8d:
      sub_23a70(a1);
      return 0;
    case 0x8e:
      dat_84a59 = 1;
      return 0;
    case 0x8f:
      dat_84b52 = 1;
      return 0;
    case 0x90:
      dat_84820 = a1;
      return 0;
    case 0x91:
      dat_84b30 = 1;
      return 0;
    case 0x92:
      sub_2d200(6,(struct_104 *)*v11);
      return 0;
    case 0x93:
      dat_84b24 = (unsigned int)strtoul(a1,&v8,10);
      if (!*v8)
        return 0;
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid incremental level value";
      goto label_2e39a;
    case 0x94:
      sub_2d260("lzip",(struct_104 *)*v11);
      return 0;
    case 0x95:
      sub_2d260("lzma",(struct_104 *)*v11);
      return 0;
    case 0x96:
      sub_2d260("lzop",(struct_104 *)*v11);
      return 0;
    case 0x97:
      dat_84b18 = sub_435a0(a1);
      if (dat_84b18) {
        dat_84b14 = umask(0);
        umask(dat_84b14);
        return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid mode given on option";
label_2e366:
      error(0,0,dcgettext(NULL,v10,5));
      sub_163d0(); // no-return
    case 0x98:
      sub_2cbe0(v11,"--mtime",a1,(struct_49 *)0x84ae0);
      if (dat_84af0)
        return 0;
      dat_84af0 = 1;
      return 0;
    case 0x99:
      if (dat_84b08 <= -1) {
        v10 = "--newer-mtime";
        sub_2cbe0(v11,v10,a1,(struct_49 *)0x84b00);
        sub_2bd60(3,(struct_104 *)*v11);
        return 0;
      }
label_2e380:
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "More than one threshold date";
      goto label_2e39a;
    case 0x9a:
      dat_84a9c = 0xffffffff;
      return 0;
    case 0x9b:
      *(char *)((int8)v11 + 0x12) = 0;
      return 0;
    case 0x9c:
      dat_84b20 = 0;
      return 0;
    case 0x9d:
      dat_847e0 = 0;
      return 0;
    case 0x9e:
      dat_84a59 = 0;
      return 0;
    case 0x9f:
      sub_2d200(1,(struct_104 *)*v11);
      return 0;
    case 0xa0:
      for (; *a1; a1 = &a1[1]) {
        sub_4a5b0(0,(int4)*a1,0);
      }
      return 0;
    case 0xa1:
      dat_84aa8 = 0xffffffff;
      return 0;
    case 0xa2:
      dat_84aa4 = 0xffffffff;
      return 0;
    case 0xa3:
      dat_847fc = 0;
      return 0;
    case 0xa4:
      dat_84aa0 = 0xffffffff;
      return 0;
    case 0xa5:
      sub_2d370(0xffffffff);
      return 0;
    case 0xa6:
      dat_84ad2 = 1;
      return 0;
    case 0xa7:
      sub_2bd60(1,(struct_104 *)*v11);
      if (!a1) {
        dat_84b38 = (char *)0x1;
        return 0;
      }
      if (sub_4c870(a1,NULL,10,&v8,"")) {
        if (dat_84d50)
          (*dat_84d50)();
        v10 = dcgettext(NULL,"Invalid number",5);
        error(0,0,"%s: %s",sub_4aa70(a1),v10);
        sub_163d0(); // no-return
      }
      dat_84b38 = v8;
      return 0;
    case 0xa8:
      sub_2d2e0("v7");
      return 0;
    case 0xa9:
      dat_84ad1 = 1;
      return 0;
    case 0xaa:
      sub_2bd60(7,(struct_104 *)*v11);
      dat_84ac8 = a1;
      dat_84ad0 = 1;
      return 0;
    case 0xab:
      sub_2d200(0,(struct_104 *)*v11);
      return 0;
    case 0xac:
      sub_2d200(2,(struct_104 *)*v11);
      return 0;
    case 0xad:
      v5 = sub_2bf00(a1,(int8 *)0x84ac0);
      if (v5 != -1) {
        dat_84abc = (int4)v5;
        return 0;
      }
      dat_84abc = 0xffffffff;
      if (!dat_84ac0) {
        dat_84abc = 0xffffffff;
        return 0;
      }
      sub_269f0(dat_84ac0,(unsigned int *)0x84abc);
      return 0;
    case 0xae:
      sub_239a0(a1);
      return 0;
    case 0xaf:
      v10 = (char *)sub_2cd70(v11,a1);
      *(char *)((int8)v11 + 0x11) = 1;
      sub_1c010(v10);
      free(v10);
      return 0;
    case 0xb0:
      sub_2d2e0("posix");
      return 0;
    case 0xb1:
      for (; *a1; a1 = &a1[1]) {
        sub_4a5b0(0,(int4)*a1,1);
      }
      return 0;
    case 0xb2:
      sub_2bde0(a1);
      return 0;
    case 0xb3:
      if (!sub_4c870(a1,NULL,10,&v8,"bBcGgkKMmPTtw")) {
        dat_84b98 = v8;
        if ((uint8)v8 & 0x1ff) {
          if (dat_84d50)
            (*dat_84d50)();
          error(0,0,dcgettext(NULL,"Record size must be a multiple of %d.",5),0x200);
          sub_2c970(2); // no-return
        }
        dat_84ba0 = (int4)((uint8)v8 >> 9);
        return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid record size";
      break;
    case 0xb4:
      dat_84aba = 1;
      return 0;
    case 0xb5:
      dat_84ab8 = 1;
      return 0;
    case 0xb6:
      dat_84a58 = 1;
      return 0;
    case 0xb7:
      dat_83c60 = a1;
      return 0;
    case 0xb8:
      dat_84ab0 = a1;
      return 0;
    case 0xb9:
      dat_84aa8 = 1;
      return 0;
    case 0xba:
      sub_2d2e0("posix");
      dat_84aa0 = 1;
      return 0;
    case 0xbb:
      v6 = (void *)sub_2c280();
      __printf_chk(1,"%s\n",v6);
      sub_3e440();
      free(v6);
      exit(0); // no-return
    case 0xbc:
      dat_84a89 = 1;
      return 0;
    case 0xbd:
      sub_1fc70();
      sub_3e440();
      exit(0); // no-return
    case 0xbe:
      dat_847e1 = 1;
      return 0;
    case 0xbf:
      sub_2d200(5,(struct_104 *)*v11);
      return 0;
    case 0xc0:
      v5 = sub_39670("--sort",a1,(int8 *)0x7f960,(void *)0x81ae8,4,dat_83dd8);
      dat_847e4 = *(unsigned int *)(v5 * 4 + 0x81ae8);
      return 0;
    case 0xc1:
      v5 = sub_39670("--hole-detection",a1,(int8 *)0x7f930,(void *)0x66a88,4,dat_83dd8);
      dat_84a7c = *(unsigned int *)(v5 * 4 + 0x66a88);
      dat_84a88 = 1;
      return 0;
    case 0xc2:
      dat_84a88 = 1;
      dat_84a84 = (unsigned int)strtoul(a1,&v8,10);
      if (!*v8)
        return 0;
      if (*v8 == '.') {
        v10 = &v8[1];
        dat_84a80 = (unsigned int)strtoul(v10,&v8,10);
        if (!*v8)
          return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid sparse version value";
      goto label_2e39a;
    case 0xc3:
      if (!sub_4c870(a1,NULL,10,&v8,"")) {
        dat_84a90 = v8;
        return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Invalid number of elements";
      break;
    case 0xc4:
      v11[3] = a1;
      dat_84b84 = 1;
      return 0;
    case 0xc5:
      sub_2d160(9);
      return 0;
    case 0xc6:
      if (!a1) {
        dat_84a69 = 1;
        return 0;
      }
      sub_2c950(a1);
      return 0;
    case 199:
      if (!dat_84a60) {
        dat_84a60 = a1;
        return 0;
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = "Only one --to-command option allowed";
label_2e39a:
      error(0,0,dcgettext(NULL,v10,5));
      sub_2c970(2); // no-return
    case 200:
      sub_30710(a1);
      return 0;
    case 0xc9:
      dat_84b91 = 1;
      return 0;
    case 0xca:
      dat_84a48 = a1;
      return 0;
    case 0xcb:
      sub_31470(a1);
      return 0;
    case 0xcc:
      sub_2d370(1);
      return 0;
    case 0xcd:
    case 0xce:
      sub_2d370(1);
      sub_31af0(a1,a0 == 0xce);
      return 0;
    case 0xcf:
      sub_2d260("zstd",(struct_104 *)*v11);
      return 0;
    
  }
  v10 = dcgettext(NULL,v10,5);
  error(0,0,"%s: %s",sub_4aa70(a1),v10);
  sub_2c970(2); // no-return
}


// Function: sub_2e4b0 @ 0x2e4b0
void sub_2e4b0(int8 *a0) // return-dupe
{
  int8 v1;
  int8 *v10; // stack - 0x100
  unsigned long v11; // stack - 0xf8
  char *v2;
  int8 v3; // stack - 0x108
  char v4 [4];
  unsigned int v5 [2]; // stack - 0x128
  int4 v6; // esi
  char *v7; // stack - 0x120
  unsigned long v8; // stack - 0x118
  unsigned long v9; // stack - 0x110
  
  v2 = getenv("TAR_OPTIONS");
  v5[0] = 0;
  v7 = "TAR_OPTIONS";
  v8 = 0;
  v9 = 0;
  if (!v2)
    return;
  v11 = 1;
  if (sub_382c0(v2,&v3,0x2000e46)) {
    if (dat_84d50)
      (*dat_84d50)();
    v2 = (char *)sub_354b0(&v3);
    error(0,0,dcgettext(NULL,"cannot split TAR_OPTIONS: %s",5),v2);
    sub_163d0(); // no-return
  }
  if (v3) {
    v1 = *a0;
    v6 = (int4)v3 + (int4)v11;
    *v10 = dat_84dd0;
    *a0 = (int8)v5;
    if (argp_parse((void *)0x81b20,v6,v10,0x28,v4,a0))
      abort(); // no-return
    *a0 = v1;
    if (sub_2bca0()) {
      if (dat_84d50)
        (*dat_84d50)();
      v2 = v7;
      error(0,0,dcgettext(NULL,"non-option arguments in %s",5),v2);
      sub_2c970(2); // no-return
    }
    v3 = 0;
  }
  sub_35420(&v3);
  return;
}


// Function: sub_2e690 @ 0x2e690
void sub_2e690(int4 a0,int8 *a1) // ternary
{
  int8 *v1;
  unsigned long v10; // rax
  uint8 v11; // rdx
  uint8 v12; // rdx
  uint8 v13; // rdx
  uint4 v14;
  int8 *v15;
  unsigned int v16 [2]; // stack - 0x98
  unsigned int *v17; // stack - 0x78
  int4 v18; // stack - 0x9c
  char v19; // stack - 0x43
  int8 *v2;
  int8 v20;
  int8 *v21;
  int8 *v22;
  uint8 v23;
  int8 *v24;
  int8 *v25; // stack - 0xb0
  unsigned long v26; // stack - 0x90
  unsigned long v27; // stack - 0x88
  unsigned long v28; // stack - 0x80
  int8 *v29; // stack - 0x70
  int8 *v3;
  unsigned short v30; // stack - 0x68
  char v31; // stack - 0x66
  char *v32; // stack - 0x60
  char *v33; // stack - 0x58
  char v34; // stack - 0x42
  char v35; // stack - 0x41
  uint4 v4;
  char v5;
  int4 v6; // eax
  char *v7;
  struct_72 *v8; // rax
  struct_72 *v9; // rax
  
  v20 = (int8)a0;
  v17 = v16;
  v30 = 0;
  v16[0] = 1;
  v26 = 0;
  v27 = 0;
  v28 = 0;
  v29 = NULL;
  v31 = '\0';
  v32 = NULL;
  v33 = NULL;
  sub_3da30("tar",0x81b00);
  v32 = getenv("SIMPLE_BACKUP_SUFFIX");
  v7 = getenv("POSIXLY_CORRECT");
  dat_84ba8 = 0;
  dat_84ba4 = 0;
  dat_84a38 = v7 != NULL;
  dat_84ba0 = 0x14;
  dat_84b98 = 0x2800;
  dat_84b60 = sub_3eba0();
  dat_847e8 = 1;
  dat_84a7c = 0;
  dat_84b08 = 0xffffffffffffffff;
  dat_84b00 = 0x8000000000000000;
  dat_84ae0 = 0x8000000000000000;
  dat_84ae8 = -1;
  dat_84ad4 = 8;
  dat_84a84 = 1;
  dat_84a80 = 0;
  dat_847e4 = 0;
  dat_84abc = 0xffffffff;
  dat_84ac0 = 0;
  dat_84b54 = 0xffffffff;
  dat_84b58 = 0;
  dat_84b20 = 1;
  dat_84b24 = -1;
  dat_847fc = 0xffffffff;
  if ((2 <= a0) && (v7 = (char *)a1[1], *v7 != '-')) {
    v19 = 0x2d;
    v24 = &a1[2];
    v35 = 0;
    v6 = (int4)strlen(v7) + a0;
    a0 = v6 + -1;
    v25 = (int8 *)sub_4c5b0((int8)v6 * 8);
    v7 = (char *)a1[1];
    v15 = &v25[1];
    v1 = &a1[v20];
    *v25 = *a1;
    v5 = *v7;
    if (v5) {
      v21 = v15;
      do {
        while( true ) {
          v15 = &v21[1];
          v34 = v5;
          *v21 = sub_4c7a0(&v19);
          v20 = sub_2beb0((int4)*v7);
          if ((v20) && (*(int8 *)(v20 + 0x10))) break;
          v5 = v7[1];
          v7 = &v7[1];
          v21 = v15;
          if (!v5) goto label_2ede2;
        }
        if (v1 <= v24) {
          if (dat_84d50)
            (*dat_84d50)();
          v5 = *v7;
          error(0,0,dcgettext(NULL,"Old option \'%c\' requires an argument.",5),v5);
          sub_2c970(2); // no-return
        }
        v20 = *v24;
        v7 = &v7[1];
        v15 = &v21[2];
        v24 = &v24[1];
        v21[1] = v20;
        v5 = *v7;
        v21 = v15;
      } while (v5);
    }
label_2ede2:
    if (v24 < v1) {
      v21 = v24;
      v22 = v15;
      do {
        v3 = &v22[1];
        v2 = &v21[1];
        *v22 = *v21;
        v21 = v2;
        v22 = v3;
      } while (v2 < v1);
      v15 = (int8 *)((int8)v15 + ((int8)v1 + (0xffffffffffffffffU - (int8)v24) & 0xfffffffffffffff8U) + 8);
    }
    *v15 = 0;
    a1 = v25;
  }
  sub_2e4b0(&v17);
  if (argp_parse((void *)0x81b20,a0,a1,8,&v18,&v17))
    exit(2); // no-return
  if ((char)v30) {
    if (dat_84ba8 != 3) // branch-flip
      dat_84aa8 = 0xffffffff;
    else {
      sub_2d2e0("v7");
    }
  }
  while (v18 < a0) {
    sub_26b50(a1[v18]);
    v18 += 1;
  }
  if (!dat_84ba4) {
    v30._1_1_ = (char)((uint2)v30 >> 8);
    dat_84ba4 = (v30._1_1_) ? 4 : 6; // branch-flip
  }
  if ((((dat_84a40) && (dat_84ba8 == 3)) || (dat_84b50)) || ((dat_84b10 || (dat_84a88))))
    sub_2e640();
  if (dat_84b38) {
    if (!sub_2bca0()) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "--occurrence is meaningless without a file list";
      goto label_2f200;
    }
    if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 0x10)) {
      if (sub_2bcc0(1))
        sub_2d1b0("--occurrence",(char *)sub_2c7e0(v13 & 0xffffffff)); // no-return
      dat_84b38 = 0;
    }
  }
  v15 = dat_84838;
  if (!dat_84830) {
    dat_84830 = 1;
    *v15 = (int8)getenv("TAPE");
    if (!*dat_84838)
      *dat_84838 = 0x6863b;
  }
  if ((2 <= dat_84830) && (!dat_84b10)) {
    if (dat_84d50)
      (*dat_84d50)();
    v7 = "Multiple archive files require \'-M\' option";
    goto label_2f200;
  }
  if (dat_84b28) { // branch-flip
    if (0 <= dat_84b08) {
      v8 = (struct_72 *)sub_2bcb0(2);
      v9 = (struct_72 *)sub_2bcb0(3);
      if (sub_2bd20(v8,v9))
        sub_2d1b0("--listed-incremental","--newer"); // no-return
      if (v8->field_0x0 == 1) {
        dat_84b28 = 0;
        goto label_2e97a;
      }
      dat_84b00 = 0;
    }
  }
  else {
label_2e97a:
    if (dat_84b24 != -1) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"--level is meaningless without --listed-incremental",5));
    }
  }
  v7 = dat_84a40;
  if ((dat_84a40) && ((dat_84ba4 & 0xfffffffb) == 2)) {
    v23 = (uint8)(-(uint4)(dat_84b10 == '\0') & 0x12) + 0x51;
    if (v23 < strlen(dat_84a40)) {
      if (dat_84d50) {
        (*dat_84d50)();
        v7 = dat_84a40;
      }
      v10 = sub_4aa70(v7);
      error(0,0,dcngettext(NULL,"%s: Volume label is too long (limit is %lu byte)","%s: Volume label is too long (limit is %lu bytes)",v23,5),v10,v23);
      sub_2c970(2); // no-return
    }
  }
  if (dat_84a50) { // branch-flip
    if (dat_84b10) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "Cannot verify multi-volume archives";
      goto label_2f200;
    }
    if (dat_84b70) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "Cannot verify compressed archives";
      goto label_2f200;
    }
    if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 2)) {
      if (sub_2bcc0(4))
        sub_2d1b0("--verify",(char *)sub_2c7e0(v12 & 0xffffffff)); // no-return
      dat_84a50 = '\0';
    }
  }
  else if (dat_84b70) {
    if (dat_84b10) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "Cannot use multi-volume compressed archives";
      goto label_2f200;
    }
    if (*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 4) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "Cannot update compressed archives";
      goto label_2f200;
    }
    if (dat_84ba8 == 2) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "Cannot concatenate compressed archives";
      goto label_2f200;
    }
  }
  if ((dat_84af0 == 2) && (dat_84ae8 <= -1)) {
    if (dat_84d50)
      (*dat_84d50)();
    v7 = "--clamp-mtime needs a date specified using --mtime";
    goto label_2f200;
  }
  if ((v30._1_1_) && (dat_84ba4 != 4)) {
    if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 1)) {
      if (dat_84d50)
        (*dat_84d50)();
      v7 = "--pax-option can be used only on POSIX archives";
      goto label_2f200;
    }
    if (1 <= dat_84a9c) { // branch-flip
label_2ea51:
      if (dat_84aa0 <= 0) goto label_2ee71;
    }
    else {
      if (dat_84aa0 <= 0) goto label_2ee71;
label_2ee3f:
      if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 1)) {
        if (dat_84d50)
          (*dat_84d50)();
        v7 = "--selinux can be used only on POSIX archives";
        goto label_2f200;
      }
    }
label_2ea5e:
    if (dat_84a78) goto label_2ea9f;
label_2ea67:
    if ((dat_84aac) && (v23 = (uint8)dat_84ba8, !(*(uint1 *)(v23 * 4 + 0x66a60) & 1))) {
label_2ea86:
      if (sub_2bcc0(6))
        sub_2d1b0("--same-order",(char *)sub_2c7e0(v23 & 0xffffffff)); // no-return
      dat_84aac = '\0';
    }
  }
  else {
    if (1 <= dat_84a9c) { // branch-flip
      if (dat_84ba4 != 4) {
        if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 1)) {
          if (dat_84d50)
            (*dat_84d50)();
          v7 = "--acls can be used only on POSIX archives";
          goto label_2f200;
        }
        goto label_2ea51;
      }
      if (dat_84aa0 <= 0) goto label_2ee71;
    }
    else if (1 <= dat_84aa0) { // branch-flip
      if (dat_84ba4 != 4) goto label_2ee3f;
    }
    else {
label_2ee71:
      if ((1 <= dat_84a98) && (dat_84ba4 != 4)) {
        if (!(*(uint1 *)((uint8)dat_84ba8 * 4 + 0x66a60) & 1)) {
          if (dat_84d50)
            (*dat_84d50)();
          v7 = "--xattrs can be used only on POSIX archives";
          goto label_2f200;
        }
        goto label_2ea5e;
      }
    }
    if (!dat_84a78) goto label_2ea67;
    v23 = (uint8)dat_84ba8;
    if (!(*(uint1 *)(v23 * 4 + 0x66a60) & 1)) {
      if (sub_2bcc0(5))
        sub_2d1b0("--starting-file",(char *)sub_2c7e0(v11 & 0xffffffff)); // no-return
      dat_84a78 = '\0';
      if (!dat_84aac) goto label_2ea9f;
      goto label_2ea86;
    }
  }
label_2ea9f:
  if (dat_84ad0) {
    if (dat_84b92) {
      v8 = (struct_72 *)sub_2bcb0(7);
      v9 = (struct_72 *)sub_2bcb0(8);
      if (sub_2bd20(v8,v9))
        sub_2d1b0("--one-top-level","--absolute-names"); // no-return
      if (v8->field_0x0 != 1) {
        dat_84ad0 = '\0';
        goto label_2eaf0;
      }
      dat_84b92 = '\0';
    }
    if (!dat_84ac8) {
      v7 = (char *)sub_3e510((char *)*dat_84838);
      dat_84ac8 = sub_2a280(v7);
      free(v7);
      if (!dat_84ac8) {
        if (dat_84d50)
          (*dat_84d50)();
        v7 = "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
        goto label_2f200;
      }
    }
  }
label_2eaf0:
  v4 = dat_84ba8;
  if (dat_84aba)
    dat_84b34 = 3;
  if (dat_84b69) { // branch-flip
    dat_84808 = 0x80900;
    if (dat_84b88 != 2) { // branch-flip
      dat_84804 = 0x90900;
      dat_84800 = 0;
    }
    else {
      v14 = 0x40000;
label_2f08e:
      dat_84808 |= v14;
      dat_84800 = (dat_84b69 ^ 1) << 8;
      dat_84804 = dat_84808 | 0x10000;
    }
  }
  else {
    if (dat_84b88 != 2) {
      dat_84808 = 0xa0900;
      v14 = 0;
      goto label_2f08e;
    }
    dat_84808 = 0xe0900;
    dat_84804 = 0xf0900;
    dat_84800 = 0x100;
  }
  if (dat_84ba8 != 9) { // branch-flip
    if (dat_84b91)
      dat_84a54 = 2;
  }
  else if (!sub_2bca0())
    dat_84a54 += 1;
  if ((dat_84a70 != dat_60748) && (dat_84a70 < (float8)dat_84b98)) {
    if (dat_84d50)
      (*dat_84d50)();
    v7 = "Volume length cannot be less than record size";
label_2f200:
    error(0,0,dcgettext(NULL,v7,5));
    sub_2c970(2); // no-return
  }
  if ((dat_84aac) && (dat_84b28)) {
    v8 = (struct_72 *)sub_2bcb0(6);
    v9 = (struct_72 *)sub_2bcb0(2);
    if (sub_2bd20(v8,v9))
      sub_2d1b0("--preserve-order","--listed-incremental"); // no-return
    if (v8->field_0x0 != 1) // branch-flip
      dat_84aac = '\0';
    else {
      dat_84b28 = 0;
    }
  }
  switch(v4) {
    case 1:
    case 2:
    case 8:
      v15 = &dat_84838[dat_84830];
      dat_84828 = dat_84838;
      v1 = dat_84838;
      while (v1 < v15) {
        v7 = (char *)*v1;
        dat_84828 = v1;
        if (!strcmp(v7,"-")) {
          if (dat_84d50)
            (*dat_84d50)();
          v7 = "Options \'-Aru\' are incompatible with \'-f -\'";
          goto label_2f200;
        }
        dat_84828 = &v1[1];
        v1 = dat_84828;
      }
      break;
    case 3:
      if (!sub_2bca0()) {
        if (dat_84d50)
          (*dat_84d50)();
        v7 = "Cowardly refusing to create an empty archive";
        goto label_2f200;
      }
      if ((v31) && (dat_84830)) {
        v7 = (char *)*dat_84838;
        if (strcmp(v7,"-"))
          sub_2a260(v7,dat_84b70);
      }
      break;
    case 5:
    case 6:
    case 7:
    case 9:
      dat_84828 = dat_84838;
      v15 = dat_84838;
      v23 = dat_84830;
      if (dat_84838 < &dat_84838[dat_84830]) {
        do {
          v7 = (char *)*dat_84828;
          if (!strcmp(v7,"-")) {
            sub_2c9b0("-f");
            v15 = dat_84838;
            v23 = dat_84830;
          }
          dat_84828 = &dat_84828[1];
        } while (dat_84828 < &v15[v23]);
      }
    
  }
  if (dat_84820) { // branch-flip
    dat_83f28 = fopen(dat_84820,"w");
    if (!dat_83f28)
      sub_327e0(dat_84820); // no-return
  }
  else if (dat_84a6a) // branch-flip
    dat_83f28 = stderr;
  else {
    dat_83f28 = stdout;
  }
  dat_84828 = dat_84838;
  if (v32)
    dat_84db8 = sub_4c7a0(v32);
  if ((dat_84b84) && (((dat_84b80 = sub_3e360("--backup",v33), !dat_84b80 || (dat_84a6a)) || (dat_84a60))))
    dat_84b84 = '\0';
  sub_107a0();
  sub_2c160(v29);
  return;
}


// Function: sub_2f5b0 @ 0x2f5b0
void sub_2f5b0(unsigned int a0,int8 *a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x38
  unsigned long v2; // stack - 0x30
  unsigned short v3; // stack - 0x28
  char v4; // stack - 0x26
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  v3 = 0;
  v2 = 0;
  v4 = 0;
  v5 = 0;
  v6 = 0;
  v1 = a2;
  argp_parse((void *)0x81240,a0,a1,0x2a,NULL,&v1);
  return;
}


// Function: sub_2f630 @ 0x2f630
void sub_2f630(unsigned long *a0)
{
  unsigned long *v1;
  uint8 v2; // rcx
  unsigned long *v3;
  
  *a0 = 0;
  a0[0x36] = 0;
  v2 = (uint8)(((int4)a0 - (int4)(unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8)) + 0x1b8U >> 3);
  v3 = (unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8);
  while (v2) {
    v1 = &v3[1];
    *v3 = 0;
    v2 -= 1;
    v3 = v1;
  }
  return;
}


// Function: sub_2f670 @ 0x2f670
bool sub_2f670(struct_16 *a0)
{
  int4 v1; // eax
  
  if (a0->field_0x1a0) // branch-flip
    v1 = closedir(a0->field_0x1a0);
  else {
    if ((int4)a0->field_0x1a8 <= 0) {
      a0->field_0x1a8 = 0;
      return 1;
    }
    v1 = close(a0->field_0x1a8);
  }
  a0->field_0x1a8 = 0;
  a0->field_0x1a0 = NULL;
  if (v1)
    sub_24c50(a0->field_0x0);
  return v1 == 0;
}


// Function: sub_2f6f0 @ 0x2f6f0
void sub_2f6f0(unsigned long *a0)
{
  unsigned long *v1;
  uint8 v2; // rcx
  unsigned long *v3;
  
  sub_2f670(a0);
  sub_1c600((unsigned long *)a0[0x2c],a0[0x2b]);
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
  sub_1cca0(&a0[0x2d]);
  sub_16ab0(a0);
  *a0 = 0;
  a0[0x36] = 0;
  v2 = (uint8)(((int4)a0 - (int4)(unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8)) + 0x1b8U >> 3);
  v3 = (unsigned long *)((uint8)&a0[1] & 0xfffffffffffffff8);
  while (v2) {
    v1 = &v3[1];
    *v3 = 0;
    v2 -= 1;
    v3 = v1;
  }
  return;
}


// Function: sub_2f7b0 @ 0x2f7b0
void sub_2f7b0(int8 a0,int8 a1,int8 a2,int8 a3)
{
  if (dat_84400 != 4) {
    a3 = 0;
    a1 = 0;
  }
  sub_4be80(a0,a1,a2,a3); // tail-call
  return;
}


// Function: sub_2f7d0 @ 0x2f7d0
void sub_2f7d0(int4 a0) // return-dupe
{
  if (a0 <= dat_84d58)
    return;
  dat_84d58 = a0;
  return;
}


// Function: sub_2f7f0 @ 0x2f7f0
unsigned long sub_2f7f0(uint4 *a0,unsigned int a1) // return-dupe
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


// Function: sub_2f880 @ 0x2f880
void sub_2f880(int8 a0)
{
  unsigned long *v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_4c5b0(0x20);
  v1 = *(unsigned long **)(a0 + 0x60);
  *v2 = 0;
  if (v1) {
    *v1 = v2;
    *(int8 *)(a0 + 0x68) = *(int8 *)(a0 + 0x68) + 1;
    *(unsigned long **)(a0 + 0x60) = v2;
    return;
  }
  *(int8 *)(a0 + 0x68) = *(int8 *)(a0 + 0x68) + 1;
  *(unsigned long **)(a0 + 0x58) = v2;
  *(unsigned long **)(a0 + 0x60) = v2;
  return;
}


// Function: sub_2f8c0 @ 0x2f8c0
void sub_2f8c0(int8 a0,unsigned int a1)
{
  int8 v1; // rax
  
  v1 = sub_2f880(a0);
  *(unsigned int *)(v1 + 8) = 2;
  *(unsigned int *)(v1 + 0x10) = a1;
  return;
}


// Function: sub_2f8e0 @ 0x2f8e0
void sub_2f8e0(int8 a0,unsigned long a1)
{
  int8 v1; // rax
  
  v1 = sub_2f880(a0);
  *(unsigned int *)(v1 + 8) = 1;
  *(unsigned long *)(v1 + 0x10) = a1;
  return;
}


// Function: sub_2f900 @ 0x2f900
void sub_2f900(int8 a0,char a1)
{
  int8 v1; // rax
  char *v2; // rax
  
  v1 = sub_2f880(a0);
  *(unsigned int *)(v1 + 8) = 0;
  v2 = (char *)sub_4c5b0(2);
  *(char **)(v1 + 0x10) = v2;
  *v2 = a1;
  *(char *)(*(int8 *)(v1 + 0x10) + 1) = 0;
  *(unsigned long *)(v1 + 0x18) = 1;
  return;
}


// Function: sub_2f940 @ 0x2f940
void sub_2f940(void)
{
  unsigned long v1; // rax
  
  v1 = sub_4c760(0x70);
  if (dat_84cc0) {
    *dat_84cc0 = v1;
    dat_84cc0 = (unsigned long *)v1;
    return;
  }
  dat_84cc8 = v1;
  dat_84cc0 = (unsigned long *)v1;
  return;
}


// Function: sub_2f990 @ 0x2f990
void sub_2f990(int8 a0,void *a1,int8 a2)
{
  int8 v1; // rax
  void *v2; // rax
  uint8 v3; // rbx
  
  v3 = a2 - (int8)a1;
  if (!v3)
    return;
  v1 = sub_2f880(a0);
  *(unsigned int *)(v1 + 8) = 0;
  v2 = (void *)sub_4c5b0(v3 + 1);
  *(void **)(v1 + 0x10) = v2;
  memcpy(v2,a1,v3);
  *(char *)(*(int8 *)(v1 + 0x10) + v3) = 0;
  *(uint8 *)(v1 + 0x18) = v3;
  return;
}


// Function: sub_2f9f0 @ 0x2f9f0
char * sub_2f9f0(char *a0)
{
  uint4 v1;
  char v10;
  char *v11; // stack - 0x250
  char v12; // stack - 0x248
  char v13 [518];
  char *v14; // stack - 0x258
  char v15;
  char v16;
  unsigned long v17; // rsi
  char *v18;
  char *v19;
  uint8 v2;
  int4 v20;
  uint8 v21;
  int4 v22; // r14d
  int4 v23;
  char v24; // stack - 0x247
  int4 v3;
  int8 v4;
  char *v5;
  int4 v6; // eax
  char *v7; // rax
  uint4 *v8; // rcx
  int8 v9; // rdx
  
  v7 = (int8)sub_2f940();
  v4 = (int8)v7;
  if (*a0 != 's') { // branch-flip
    v6 = strncmp(a0,"flags=",6);
    if (!v6) {
      v10 = a0[6];
      a0 = &a0[6];
      dat_83b60 = 0;
      if (!v10) {
        v7 = a0;
        dat_83b60 = 0;
        return v7;
      }
      v8 = (uint4 *)0x83b60;
      while( true ) {
        if (v10 == ';') {
          v7 = &a0[1];
          return v7;
        }
        v6 = sub_2f7f0(v8);
        if (v6) break;
        v10 = a0[1];
        a0 = &a0[1];
        if (!v10) {
          v7 = a0;
          return v7;
        }
      }
      if (dat_84d50)
        (*dat_84d50)();
      v10 = *a0;
      v18 = "Unknown transform flag: %c";
label_2ff6e:
      v7 = dcgettext(NULL,v18,5);
      error(0,0,v7,(uint8)(uint4)(int4)v10);
      sub_2c970(2); // no-return
    }
  }
  else {
    v10 = a0[1];
    v6 = (int4)v10;
    v3 = v6;
    if (v10) {
      v21 = 2;
      v15 = a0[2];
      v2 = 2;
      while (v15) {
        v20 = (int4)v21;
        v22 = v20 + 1;
        if (v10 == v15) {
          v9 = (int8)v22;
          v18 = &a0[v9];
          v10 = *v18;
          v23 = v22;
          if (v10) goto label_2fb74;
          goto label_2ffb5;
        }
        if (v15 == '\\')
          v22 = (v20 + 2) - (uint4)(a0[v2 + 1] == '\0');
        v21 = (uint8)v22;
        v15 = a0[v21];
        v2 = v21;
      }
    }
  }
label_2ffb5:
  if (dat_84d50)
    (*dat_84d50)();
  v18 = "Invalid transform expression";
label_2ffcf:
  v7 = dcgettext(NULL,v18,5);
  error(0,0,v7);
  sub_2c970(2); // no-return
  while( true ) {
    if (v10 == '\\')
      v23 = (v23 + 1) - (uint4)(a0[v9 + 1] == '\0');
    v23 += 1;
    v9 = (int8)v23;
    v10 = a0[v9];
    if (!v10) break;
label_2fb74:
    if (v10 == v15) {
      v19 = &a0[v9 + 1];
      *(unsigned int *)(v4 + 8) = 0;
      *(unsigned int *)(v4 + 0xc) = dat_83b60;
      v15 = *v19;
      v1 = 0;
      v11 = v19;
      v16 = v15;
      if ((!v15) || (v15 == ';')) goto label_2fc30;
      goto label_2fbf3;
    }
  }
  goto label_2ffb5;
  while( true ) {
    v16 = '\0';
    v11 = v19;
    if (!v15) goto label_2fc30;
    v20 = (int4)v21;
    if (v15 == ';') break;
label_2fbf3:
    v20 = (int4)v21;
    v7 = v11;
    if (v15 != 'i') { // branch-flip
      if ('i' < v15) {
        if (v15 != 'x') goto label_2fbc2;
        v1 |= 1;
        v19 = &v11[1];
        goto label_2fbdb;
      }
      if (v15 <= '9') {
        if ('/' < v15) {
          v7 = (uint8)strtoul(v19,&v11,0);
          v21 &= 0xffffffff;
          *(int4 *)(v4 + 0x10) = v6;
          v7 = &v11[-1];
          v19 = v11;
          goto label_2fbdb;
        }
label_2fbc2:
        v6 = sub_2f7f0((uint4 *)(v4 + 0xc));
        if (!v6) {
          v7 = v11;
          v19 = &v11[1];
          goto label_2fbdb;
        }
        if (dat_84d50)
          (*dat_84d50)();
        v18 = "Unknown flag in transform expression: %c";
        v10 = *v11;
        goto label_2ff6e;
      }
      if (v15 != 'g') goto label_2fbc2;
      *(unsigned int *)(v4 + 8) = 1;
      v19 = &v11[1];
      v15 = v11[1];
    }
    else {
      v1 |= 2;
      v19 = &v11[1];
label_2fbdb:
      v20 = (int4)v21;
      v15 = v7[1];
    }
  }
label_2ff30:
  v11 = &v19[1];
label_2fc3a:
  v7 = (char *)sub_4c5b0((int8)(v20 + -1));
  v19 = v7;
  memcpy(v7,&a0[2],(int8)(v20 + -2));
  v7[v2 - 2] = '\0';
  v6 = sub_5ae70((struct_98 *)(v4 + 0x18),v7,v1);
  if (v6) {
    sub_5af90(v6,(struct_98 *)(v4 + 0x18),&v12,0x200);
    if (dat_84d50)
      (*dat_84d50)();
    v7 = dcgettext(NULL,"Invalid transform expression: %s",5);
    error(0,0,v7,&v12);
    sub_2c970(2); // no-return
  }
  if ((*v19 == '^') || ((v20 != 2 && (v19[v2 - 3] == '$'))))
    *(unsigned int *)(v4 + 8) = 0;
  v23 -= v22;
  free(v19);
  v7 = (char *)sub_4c5b0((int8)(v23 + 1));
  v5 = v7;
  memcpy(v7,v18,(int8)v23);
  v7[v23] = '\0';
  v15 = *v7;
  v14 = v7;
  v18 = v7;
  v19 = v14;
  do {
    while( true ) {
      if (!v15) {
        v14 = v19;
        sub_2f990(v4,v18,v7);
        free(v5);
        v7 = v11;
        return v7;
      }
      if (v15 == '\\') break;
      if (v15 != '&') // branch-flip
        v7 = &v7[1];
      else {
        sub_2f990(v4,v18,v7);
        sub_2f8e0(v4,0);
        v7 = &v14[1];
        v18 = v7;
      }
      v15 = *v7;
      v14 = v7;
      v19 = v7;
    }
    sub_2f990(v4,v18,v7);
    v18 = &v14[1];
    v15 = v14[1];
    v14 = v18;
    switch(v15) {
      case 0x26:
        v17 = 0x26;
        break;
      default:
        if (v15 != v10) { // branch-flip
          v12 = '\\';
          v24 = v15;
          sub_2f990(v4,&v12,v13);
        }
        else {
          sub_2f900(v4,v3);
        }
        v14 = &v14[1];
        v7 = v14;
        goto label_2fe28;
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
        v7 = (uint8)strtoul(v18,&v14,10);
        if (v7 <= *(uint8 *)(v4 + 0x48)) {
          sub_2f8e0(v4,v7);
          v7 = v14;
          goto label_2fe28;
        }
        if (dat_84d50)
          (*dat_84d50)();
        v18 = "Invalid transform replacement: back reference out of range";
        goto label_2ffcf;
      case 0x45:
        v17 = 0;
        goto label_2fe5f;
      case 0x4c:
        v17 = 4;
        goto label_2fe5f;
      case 0x55:
        v17 = 3;
        goto label_2fe5f;
      case 0x5c:
        v17 = 0x5c;
        break;
      case 0x61:
        v17 = 7;
        break;
      case 0x62:
        v17 = 8;
        break;
      case 0x66:
        v17 = 0xc;
        break;
      case 0x6c:
        v17 = 2;
        goto label_2fe5f;
      case 0x6e:
        v17 = 10;
        break;
      case 0x72:
        v17 = 0xd;
        break;
      case 0x74:
        v17 = 9;
        break;
      case 0x75:
        v17 = 1;
label_2fe5f:
        sub_2f8c0(v4,v17);
        v14 = &v14[1];
        v7 = v14;
        goto label_2fe28;
      case 0x76:
        v17 = 0xb;
      
    }
    sub_2f900(v4,v17);
    v14 = &v14[1];
    v7 = v14;
label_2fe28:
    v15 = *v7;
    v18 = v7;
    v19 = v14;
  } while( true );
label_2fc30:
  if (v16 != ';') goto label_2fc3a;
  goto label_2ff30;
}


// Function: sub_30070 @ 0x30070
uint1 * sub_30070(int4 a0,void *a1,uint8 a2)
{
  int8 *v1; // rax
  uint1 *v2;
  uint1 *v3;
  uint1 *v4; // rdx
  
  v2 = dat_84cb8;
  if (dat_84c40 < a2) {
    dat_84c40 = a2;
    v2 = (uint1 *)sub_4c610(dat_84cb8,a2);
    dat_84cb8 = v2;
  }
  memcpy(v2,a1,a2);
  if (a0 != 3) { // branch-flip
    switch(a0) { // branch-flip
      case 1:
        v1 = __ctype_toupper_loc();
        *v2 = (uint1)*(unsigned int *)(*v1 + (uint8)*v2 * 4);
        return v2;
      case 2:
        v1 = __ctype_tolower_loc();
        *v2 = (uint1)*(unsigned int *)(*v1 + (uint8)*v2 * 4);
        break;
      case 4:
        if (v2 < &v2[a2]) {
          v1 = __ctype_tolower_loc();
          v3 = v2;
          do {
            v4 = &v3[1];
            *v3 = (uint1)*(unsigned int *)(*v1 + (uint8)*v3 * 4);
            v3 = v4;
          } while (v4 != &v2[a2]);
          return v2;
        }
      
    }
  }
  else if (v2 < &v2[a2]) {
    v1 = __ctype_toupper_loc();
    v3 = v2;
    do {
      v4 = &v3[1];
      *v3 = (uint1)*(unsigned int *)(*v1 + (uint8)*v3 * 4);
      v3 = v4;
    } while (v4 != &v2[a2]);
    return v2;
  }
  return v2;
}


// Function: sub_301b0 @ 0x301b0
void sub_301b0(int8 a0,char *a1)
{
  int4 v1;
  char *v10;
  char *v11;
  uint4 v12;
  uint4 v13;
  uint8 v14; // stack - 0x68
  uint4 v15; // stack - 0x44
  uint4 v2;
  int8 v3;
  unsigned long *v4;
  uint4 v5;
  uint8 *v6; // rax
  uint8 v7;
  uint8 v8;
  uint8 v9;
  
  v6 = (uint8 *)sub_4c5b0((*(int8 *)(a0 + 0x48) + 1) * 0x10);
  if (*a1) {
    v15 = 0;
    v12 = 0;
    v14 = 0;
    do {
      v3 = *(int8 *)(a0 + 0x48);
      if (sub_5b070((struct_39 *)(a0 + 0x18),a1,v3 + 1,v6,0)) { // branch-flip
        v7 = strlen(a1);
        if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v7)
          _obstack_newchunk((obstack *)0x84c60,v7);
        memcpy(dat_84c78,a1,v7);
        dat_84c78 = &dat_84c78[v7];
label_302f9:
        v10 = &a1[v7];
        if (!*(int4 *)(a0 + 8)) {
          v8 = strlen(v10);
          if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v8)
            _obstack_newchunk((obstack *)0x84c60,v8);
          memcpy(dat_84c78,v10,v8);
          dat_84c78 = &dat_84c78[v8];
          if (dat_84c80 != dat_84c78) goto label_305a4;
          goto label_30352;
        }
      }
      else {
        v8 = *v6;
        v7 = v6[1];
        if (v8) {
          if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v8)
            _obstack_newchunk((obstack *)0x84c60,v8);
          memcpy(dat_84c78,a1,v8);
          dat_84c78 = &dat_84c78[v8];
        }
        v14 += 1;
        if ((!*(uint4 *)(a0 + 0x10)) || (*(uint4 *)(a0 + 0x10) <= v14)) {
          v4 = *(unsigned long **)(a0 + 0x58);
          v5 = v15;
          while (v15 = v5, v4) {
            v1 = *(int4 *)&v4[1];
            v13 = v12;
            if (v1 != 1) { // branch-flip
              if (v1 != 2) { // branch-flip
                if (!v1) {
                  if (v12) { // branch-flip
                    v10 = (char *)sub_30070(v12,(void *)v4[2],v4[3]);
                    if (v12 - 1 <= 1) {
                      v15 = 0;
                      v12 = v5;
                    }
                  }
                  else {
                    v10 = (char *)v4[2];
                  }
                  v9 = v4[3];
                  v11 = dat_84c78;
                  if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v9) goto label_3044f;
                  memcpy(v11,v10,v9); // crossjump-dupe
                  dat_84c78 = &dat_84c78[v9];
                  v13 = v12;
                }
              }
              else {
                v2 = *(uint4 *)&v4[2];
                v13 = v2; // branch-flip
                if (3 <= v2) {
                  v13 = v12;
                  if (v2 - 3 <= 1)
                    v13 = v2;
                }
                else if (v2) { // branch-flip
                  if ((!v5) || (v5 - 3 < 2))
                    v15 = v12;
                }
                else {
                  v13 = 0;
                }
              }
            }
            else {
              v8 = v6[v4[2] * 2];
              if ((v8 != 0xffffffffffffffff) && (v9 = (&v6[v4[2] * 2])[1], v9 != 0xffffffffffffffff)) {
                v9 -= v8;
                v10 = &a1[v8];
                if (v12) {
                  v10 = (char *)sub_30070(v12,v10,v9);
                  if (v12 - 1 <= 1) {
                    v15 = 0;
                    v12 = v5;
                  }
                }
                v11 = dat_84c78;
                if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v9) {
label_3044f:
                  _obstack_newchunk((obstack *)0x84c60,v9);
                  v11 = dat_84c78;
                }
                memcpy(v11,v10,v9);
                dat_84c78 = &dat_84c78[v9];
                v13 = v12;
              }
            }
            v4 = (unsigned long *)*v4;
            v12 = v13;
            v5 = v15;
          }
          goto label_302f9;
        }
        if ((uint8)((int8)dat_84c80 - (int8)dat_84c78) < v7)
          _obstack_newchunk((obstack *)0x84c60,v7);
        v10 = &a1[v7];
        memcpy(dat_84c78,a1,v7);
        dat_84c78 = &dat_84c78[v7];
      }
      a1 = v10;
    } while (*v10);
  }
  if (dat_84c80 == dat_84c78) {
label_30352:
    _obstack_newchunk((obstack *)0x84c60,1);
  }
label_305a4:
  v11 = &dat_84c78[1];
  *dat_84c78 = 0;
  dat_84c78 = v11;
  free(v6); // tail-call
  return;
}


// Function: sub_30600 @ 0x30600
unsigned long sub_30600(uint4 a0,uint8 a1,uint8 *a2)
{
  uint8 v1; // rax
  int8 *v2;
  unsigned long v3;
  
  if (!dat_84c48) {
    _obstack_begin((obstack *)0x84c60,0,0,sub_4c5b0,dat_80fb8);
    dat_84c48 = '\x01';
  }
  v3 = 0;
  v2 = dat_84cc8;
  while (v2) {
    while (!(a0 & *(uint4 *)((int8)v2 + 0xc))) {
      v2 = (int8 *)*v2;
      if (!v2) goto label_306b4;
    }
    sub_301b0(v2,a1);
    if (dat_84c78 == dat_84c70)
      dat_84cb0 |= 2;
    v3 = 1;
    v1 = dat_84c78 + dat_84c90 & ~dat_84c90;
    dat_84c78 = dat_84c80;
    if (v1 - dat_84c68 <= dat_84c80 - dat_84c68)
      dat_84c78 = v1;
    v2 = (int8 *)*v2;
    a1 = dat_84c70;
    dat_84c70 = dat_84c78;
  }
label_306b4:
  *a2 = a1;
  return v3;
}


// Function: sub_30710 @ 0x30710
void sub_30710(char *a0)
{
  if (!*a0)
    return;
  do {
    a0 = (char *)sub_2f9f0(a0);
  } while (*a0);
  return;
}


// Function: sub_30740 @ 0x30740
unsigned int sub_30740(uint8 *a0,unsigned int a1,code *a2,unsigned long a3)
{
  unsigned int v1; // eax
  void *v2;
  void *v3; // stack - 0x38
  
  v1 = sub_30600(a1,*a0,&v3);
  if ((char)v1) { // branch-flip
    v2 = v3;
    if (a2)
      v2 = (void *)(*a2)(v3,a3);
    sub_23be0(a0,v2);
    if ((v3 <= dat_84c68) || (dat_84c80 <= v3))
      _obstack_free((obstack *)0x84c60,v3);
    else {
      dat_84c70 = v3;
      dat_84c78 = v3;
    }
  }
  else if (a2) {
    *a0 = 0;
    sub_23be0(a0,(char *)(*a2)(v3,a3));
    v1 = 1;
    free(v3);
  }
  return v1;
}


// Function: sub_30830 @ 0x30830
unsigned int sub_30830(uint8 *a0,unsigned long a1)
{
  return sub_30740(a0,a1,NULL,0); // tail-call
}


// Function: sub_30840 @ 0x30840
bool sub_30840(void)
{
  return dat_84cc8 != 0;
}


// Function: sub_30850 @ 0x30850
void sub_30850(unsigned long *a0,unsigned long *a1)
{
  unsigned long *v1;
  unsigned long *v2;
  
  if (a0) { // branch-flip
    *a1 = *a0;
    *a0 = a1;
    v1 = (unsigned long *)*a1;
    v2 = dat_84ce8;
  }
  else {
    *a1 = dat_84ce8;
    v2 = a1;
    v1 = dat_84ce8;
  }
  if (v1) {
    dat_84ce8 = v2;
    dat_84cd8 += 1;
    return;
  }
  dat_84ce8 = v2;
  dat_84cd8 += 1;
  dat_84ce0 = a1;
  return;
}


// Function: sub_308a0 @ 0x308a0
void sub_308a0(struct_51 *a0)
{
  unsigned long v1;
  
  free(a0->field_0x10);
  v1 = dat_84cd0;
  dat_84cd0 = a0;
  a0->field_0x0 = v1;
  return;
}


// Function: sub_308c0 @ 0x308c0
void sub_308c0(bool a0)
{
  int4 v1;
  int8 *v2;
  unsigned int v3;
  char *v4;
  int8 *v5;
  int8 *v6;
  
  v3 = dat_84580;
  v5 = NULL;
  v6 = dat_84ce8;
  if (dat_84ce8) {
label_3090c:
    do {
      v2 = (int8 *)*v6;
      if ((a0) || (dat_83f30 > (uint8)v6[4])) {
        sub_24c80(*(unsigned int *)&v6[1]);
        if ((char)v6[3]) { // branch-flip
          v4 = (char *)v6[2];
          if (((int4)v6[1]) && ((!*v4 || (!strcmp(v4,"."))))) goto label_30903;
          if (unlinkat(dat_81234,v4,0x200)) {
            v1 = *__errno_location();
            if ((v1 == 0x11) || (v1 == 0x27)) goto label_30903;
            if (v1 != 2)
              sub_32aa0(v4);
          }
        }
        else {
          v4 = (char *)v6[2];
          if ((unlinkat(dat_81234,v4,0)) && (*__errno_location() != 2))
            sub_32db0((char *)v6[2]);
        }
        sub_308a0(v6);
        dat_84cd8 -= 1;
        if (v5) {
          *v5 = (int8)v2;
          v6 = v2;
          if (!v2) break;
          goto label_3090c;
        }
        v6 = v5;
        dat_84ce8 = v2;
      }
label_30903:
      v5 = v6;
      v6 = v2;
    } while (v2);
    if (dat_84ce8) {
      v6 = dat_84ce8;
      if (a0) {
        do {
          v5 = (int8 *)*v6;
          sub_24c80(*(unsigned int *)&v6[1]);
          v4 = (char *)v6[2];
          if ((((int4)v6[1]) && ((char)v6[3])) && ((!*v4 || (!strcmp(v4,"."))))) {
            v4 = (char *)sub_24c30();
            sub_24c80((int4)v6[1] + -1);
          }
          if ((unlinkat(dat_81234,v4,0x200)) && (*__errno_location() != 2))
            sub_32aa0(v4);
          sub_308a0(v6);
          dat_84cd8 -= 1;
          v6 = v5;
        } while (v5);
        dat_84ce0 = 0;
        dat_84ce8 = NULL;
      }
      sub_24c80(v3); // return-dupe, tail-call
      return;
    }
  }
  dat_84ce0 = 0;
  sub_24c80(v3);
  return;
}


// Function: sub_30b00 @ 0x30b00
void sub_30b00(void)
{
  unsigned long v1;
  
  if (dat_84cd0) {
    v1 = *dat_84cd0;
    *dat_84cd0 = 0;
    dat_84cd0 = (unsigned long *)v1;
    return;
  }
  sub_4c5b0(0x28); // tail-call
  return;
}


// Function: sub_30b30 @ 0x30b30
void sub_30b30(void) // return-dupe
{
  int8 *v1;
  int8 *v2;
  
  sub_308c0(1);
  v2 = dat_84cd0;
  if (!dat_84cd0)
    return;
  do {
    v1 = (int8 *)*v2;
    free(v2);
    v2 = v1;
    dat_84cd0 = v1;
  } while (v1);
  return;
}


// Function: sub_30b70 @ 0x30b70
void sub_30b70(char *a0,char a1)
{
  int8 *v1;
  struct_52 *v2; // rax
  char *v3;
  int8 *v4; // rbx
  int8 *v5;
  
  if ((dat_84ce8) && ((uint8)dat_84ce8[4] < dat_83f30))
    sub_308c0(0);
  v2 = (struct_52 *)sub_30b00();
  v2->field_0x0 = 0;
  v2->field_0x8 = dat_84580;
  v3 = (char *)sub_4c7a0(a0);
  v2->field_0x10 = v3;
  sub_23ea0(v3);
  v2->field_0x18 = a1;
  v2->field_0x20 = dat_83f30;
  if ((a1) && (((v3 = v2->field_0x10, !*v3 || (!strcmp(v3,"."))) && (dat_84ce8)))) {
    v1 = dat_84ce8;
    v5 = NULL;
    while( true ) {
      v4 = v1;
      if ((((char)v4[3]) && ((v3 = (char *)v4[2], !*v3 || (!strcmp(v3,"."))))) && ((int4)v4[1] < (int4)v2->field_0x8)) {
        sub_30850(v5,v2); // tail-call
        return;
      }
      if (!(int8 *)*v4) break;
      v1 = (int8 *)*v4;
      v5 = v4;
    }
  }
  sub_30850(dat_84ce0,v2); // tail-call
  return;
}


// Function: sub_30ca0 @ 0x30ca0
void sub_30ca0(char *a0) // return-dupe
{
  unsigned long *v1;
  stat v10; // stack - 0xe8
  char v11 [24];
  unsigned long *v12;
  uint8 v13;
  int4 v2; // eax
  uint4 v3; // eax
  int8 v4; // rax
  int8 v5; // rax
  uint8 v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  uint8 v9; // rcx
  
  v2 = openat(dat_81234,a0,0);
  if (v2 < 0) {
    sub_327c0(a0); // tail-call
    return;
  }
  v13 = v10._48_8_;
  if (fstat(v2,&v10)) // branch-flip
    sub_32d30(a0);
  else {
    for (; 0 < (int8)v13; v13 = v13 - v4) {
      v5 = sub_e130();
      v6 = sub_d2b0(v5);
      if (v13 < v6) {
        v6 = v13;
        if (v13 & 0x1ff) {
          v3 = 0x200 - ((uint4)v13 & 0x1ff);
          v12 = (unsigned long *)(v5 + v13);
          if (8 <= v3) { // branch-flip
            *v12 = 0;
            *(unsigned long *)((int8)v12 + ((uint8)v3 - 8)) = 0;
            v9 = (uint8)(v3 + ((int4)v12 - (int4)(unsigned long *)((uint8)&v12[1] & 0xfffffffffffffff8)) >> 3);
            v12 = (unsigned long *)((uint8)&v12[1] & 0xfffffffffffffff8);
            while (v9) {
              v1 = &v12[1];
              *v12 = 0;
              v9 -= 1;
              v12 = v1;
            }
          }
          else if (v3 & 4) { // branch-flip
            *(unsigned int *)v12 = 0;
            *(unsigned int *)((int8)v12 + ((uint8)v3 - 4)) = 0;
          }
          else if ((v3) && (*(char *)v12 = 0, v3 & 2))
            *(unsigned short *)((int8)v12 + ((uint8)v3 - 2)) = 0;
        }
      }
      v4 = sub_4af10(v2,v5,v6);
      if (v4 == -1)
        sub_329d0(a0,v10._48_8_ - v13,v6); // no-return
      if (!v4) {
        if (dat_84d50)
          (*dat_84d50)();
        v7 = sub_43090(v13,v11);
        v8 = sub_4aa70(a0);
        error(0,0,dcngettext(NULL,"%s: File shrank by %s byte","%s: File shrank by %s bytes",v13,5),v8,v7);
        sub_163d0(); // no-return
      }
      sub_d1f0(v5 + (v4 - 1U & 0xfffffffffffffe00U));
    }
  }
  if (close(v2)) {
    sub_32680(a0);
    return;
  }
  return;
}


// Function: sub_30ef0 @ 0x30ef0
void sub_30ef0(void)
{
  unsigned int v1;
  char *v10; // stack - 0xe8
  uint4 v11; // stack - 0xc0
  uint4 v2; // eax
  int4 v3; // eax
  int8 v4;
  int8 v5; // rax
  unsigned long *v6; // rax
  char v7 [24];
  uint4 v8;
  char *v9;
  
  sub_26d90();
  sub_fb70(2);
  sub_1c560();
  v8 = 0;
  do {
    v2 = sub_219e0((unsigned long *)0x84408,(unsigned long *)0x84840,0);
    switch(v2) {
      case 0:
      case 2:
        abort(); // no-return, return-dupe
      case 1:
        sub_215c0(dat_84408,0x84840,(int4 *)0x84400,0);
        sub_21440((int4)*(char *)(dat_84408 + 0x9c),0x84840);
        dat_84ba4 = dat_84400;
        if ((dat_84ba8 == 8) && (v4 = sub_27aa0(dat_84848), v4)) {
          sub_24c80(*(unsigned int *)(v4 + 0x28));
          v3 = sub_246f0(dat_84848,v7);
          if (!v3) {
            if ((v11 & 0xf000) != 0x4000) { // branch-flip
              v5 = sub_4bd20(v7);
              if ((int4)sub_2f7b0(v5) <= 0)
                sub_27150(v4);
            }
            else {
              v10 = (char *)sub_253c0(*(char **)(v4 + 0x10),1);
              if (v10) {
                v6 = (unsigned long *)sub_25040(*(char **)(v4 + 0x10));
                if (*v10) {
                  v9 = v10;
                  do {
                    v1 = *(unsigned int *)(v4 + 0x28);
                    sub_26cf0((char *)sub_250d0(v6,v9),v1,0,0);
                    v9 = &v9[strlen(v9) + 1];
                  } while (*v9);
                }
                sub_250b0(v6);
                free(v10);
                sub_27150(v4);
              }
            }
          }
        }
        sub_22bd0();
        sub_2f6f0((unsigned long *)0x84840);
        break;
      case 3:
        dat_83f48 = dat_84408;
        sub_2f6f0((unsigned long *)0x84840);
        goto label_30f88;
      case 4:
        sub_2f6f0((unsigned long *)0x84840);
label_30f88:
        sub_d1a0();
        dat_84cf8 = 1;
        dat_84cf0 = dat_83f48;
label_30fb0:
        do {
          v4 = sub_27b00();
          while( true ) {
            if (!v4) {
              sub_12c40();
              sub_f900();
              sub_30b30();
              sub_27430(); // tail-call
              return;
            }
            v9 = *(char **)(v4 + 0x10);
            if ((sub_16b00(v9,0)) || ((dat_84b40 && (!sub_2ca20("add",v9))))) goto label_30fb0;
            if (dat_84ba8 == 2) break;
            sub_14330(NULL,v9,v9);
            v4 = sub_27b00();
          }
          sub_30ca0(v9);
        } while( true );
      case 5:
        sub_d1f0(dat_84408);
        if (v8 != 1) {
          if (2 <= v8) { // branch-flip
            if (v8 != 3) {
              if (v8 != 4) goto label_31154;
              abort();
            }
          }
          else {
            if (dat_84d50)
              (*dat_84d50)();
            error(0,0,dcgettext(NULL,"This does not look like a tar archive",5));
          }
        }
        if (dat_84d50)
          (*dat_84d50)();
        error(0,0,dcgettext(NULL,"Skipping to next header",5));
        dat_84d58 = 2;
        sub_2f6f0((unsigned long *)0x84840);
        break;
      default:
label_31154:
        sub_2f6f0((unsigned long *)0x84840);
      
    }
    v8 = v2;
  } while( true );
}


// Function: sub_312d0 @ 0x312d0
void sub_312d0(uint1 a0)
{
  unsigned long v1; // rax
  
  if (*(int8 *)((uint8)a0 * 8 + 0x83b70) != -1)
    return;
  if (!a0) {
    v1 = sub_430f0();
    dat_83b70 = iconv_open(v1,"UTF-8");
    return;
  }
  v1 = sub_430f0();
  dat_83b78 = iconv_open("UTF-8",v1);
  return;
}


// Function: sub_31340 @ 0x31340
unsigned long sub_31340(char a0,char *a1,unsigned long *a2) // return-dupe x3
{
  int8 v1; // rax
  char *v2; // rax
  char *v3; // stack - 0x48
  int8 v4; // stack - 0x40
  char *v5; // stack - 0x50
  int8 v6; // stack - 0x38
  
  v1 = sub_312d0((uint1)a0);
  if (!v1) {
    *a2 = sub_4c7a0(a1);
    return 1;
  }
  if (v1 == -1)
    return 0;
  v4 = strlen(a1) + 1;
  v6 = v4 * 0x10 + 1;
  v2 = (char *)sub_4c5b0();
  v3 = v2;
  v5 = a1;
  if (!iconv(v1,&v5,&v4,&v3,&v6)) {
    *v3 = 0;
    *a2 = v2;
    return 1;
  }
  free(v2);
  return 0;
}


// Function: sub_31440 @ 0x31440
unsigned long sub_31440(char *a0)
{
  while( true ) {
    if (!*a0)
      return 1;
    if (*a0 < '\0') break;
    a0 = &a0[1];
  }
  return 0;
}


// Function: sub_31470 @ 0x31470
void sub_31470(char *a0)
{
  unsigned long v1;
  uint8 v2; // rax
  
  if (!strcmp(a0,"none")) {
    dat_83b80 = 0;
    return;
  }
  v2 = strlen(a0);
  v1 = dat_83dd8;
  if ((3 <= v2) && (!memcmp(a0,(void *)0x6708a,3))) {
    dat_83b80 = ~*(uint4 *)(sub_39670("--warning",&a0[3],(int8 *)0x7fa20,(void *)0x83ba0,4,v1) * 4 + 0x83ba0) & dat_83b80;
    return;
  }
  dat_83b80 |= *(uint4 *)(sub_39670("--warning",a0,(int8 *)0x7fa20,(void *)0x83ba0,4,v1) * 4 + 0x83ba0);
  return;
}


// Function: sub_31550 @ 0x31550
void sub_31550(unsigned long *a0)
{
  void *v1;
  
  if (a0[2] != a0[1])
    return;
  if (!a0[1])
    a0[1] = 4;
  v1 = (void *)*a0;
  *a0 = sub_4c680(v1,&a0[1],8);
  return;
}


// Function: sub_31600 @ 0x31600
uint8 sub_31600(char *a0,uint8 a1)
{
  int4 v1; // eax
  uint8 v2; // rax
  
  if (dat_84d28) {
    v2 = sub_31590(a0,(int8 *)0x84d20); // tail-call
    return v2;
  }
  if ((char)a1)
    return a1 & 0xffffffff;
  v1 = strncmp(a0,"user.",5);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_31730 @ 0x31730
unsigned long sub_31730(char *a0,char a1)
{
  if (!sub_31600(a0,(uint1)a1))
    return 1;
  return sub_31710(a0); // tail-call
}


// Function: sub_319f0 @ 0x319f0
void sub_319f0(void)
{
  if (dat_84a9c <= 0)
    return;
  if (!dat_84d14) {
    if (dat_84d50)
      (*dat_84d50)();
    error(0,0,dcgettext(NULL,"POSIX ACL support is not available",5));
    dat_84d14 = 1;
    return;
  }
  dat_84d14 = 1;
  return;
}


// Function: sub_31a70 @ 0x31a70
void sub_31a70(unsigned long a0,unsigned long a1,char a2)
{
  if ((1 <= dat_84a9c) && (a2 != '2')) {
    if (!dat_84d10) {
      if (dat_84d50)
        (*dat_84d50)();
      error(0,0,dcgettext(NULL,"POSIX ACL support is not available",5));
      dat_84d10 = 1;
      return;
    }
    dat_84d10 = 1;
    return;
  }
  return;
}


// Function: sub_31af0 @ 0x31af0
void sub_31af0(char *a0,bool a1)
{
  int8 v1;
  int8 *v2;
  
  v2 = (int8 *)0x84d20;
  if (!a1)
    v2 = (int8 *)0x84d38;
  sub_31550(v2);
  v1 = v2[2];
  v2[2] = v1 + 1;
  *(char **)(*v2 + v1 * 8) = a0;
  return;
}


// Function: sub_31b40 @ 0x31b40
void sub_31b40(void)
{
  sub_319d0(dat_84d20,dat_84d28);
  sub_319d0(dat_84d38,dat_84d40); // tail-call
  return;
}


// Function: sub_31b80 @ 0x31b80
void sub_31b80(unsigned int a0,char *a1,int8 a2,int4 a3) // return-dupe x2
{
  int4 v1;
  char *v2;
  int8 v3; // rax
  char *v4; // rax
  void *v5;
  int8 v6;
  uint8 v7; // stack - 0x58
  
  if (dat_84a98 <= 0)
    return;
  v2 = dat_84d08;
  if (!dat_84d08) {
    v2 = (char *)sub_4c680(NULL,(uint8 *)0x83c10,1);
    dat_84d08 = v2;
  }
  do {
    if (a3) { // branch-flip
      v3 = flistxattr(a3,v2,dat_83c10);
      if (v3 != -1) {
label_31c38:
        if (!dat_84d00)
          dat_84d00 = (void *)sub_4c680(NULL,(uint8 *)0x83c08,1);
        if (v3 <= 0)
          return;
        v4 = "fgetxattr";
        if (!a3)
          v4 = "lgetxattrat";
        do {
          v7 = strlen(v2);
          v5 = dat_84d00;
          while( true ) {
            if (a3) // branch-flip
              v6 = fgetxattr(a3,v2,v5,dat_83c08);
            else {
              v6 = sub_38a90(a0,a1,v2,v5);
            }
            if (v6 != -1) {
              if (!sub_31730(v2,1))
                sub_1c650(a2,v2,dat_84d00,v6);
              goto label_31d0c;
            }
            v1 = *__errno_location();
            if (v1 != 0x22) break;
            v5 = (void *)sub_4c680(dat_84d00,(uint8 *)0x83c08,1);
            dat_84d00 = v5;
          }
          if (v1 != 0x3d)
            sub_324f0(v4,a1);
label_31d0c:
          v2 = &v2[v7 + 1];
          v3 = (v3 - v7) + -1;
          if (v3 <= 0)
            return;
        } while( true );
      }
    }
    else {
      v3 = sub_38f90(a0,a1,v2,dat_83c10);
      if (v3 != -1) {
        v2 = dat_84d08;
        goto label_31c38;
      }
    }
    if (*__errno_location() != 0x22) {
      v2 = "llistxattrat";
      if (a3)
        v2 = "flistxattr";
      sub_324f0(v2,a1); // tail-call
      return;
    }
    v2 = (char *)sub_4c680(dat_84d08,(uint8 *)0x83c10,1);
    dat_84d08 = v2;
  } while( true );
}


// Function: sub_31de0 @ 0x31de0
void sub_31de0(int4 a0,char *a1,struct_76 *a2,int4 a3)
{
  int4 v1;
  char *v2; // rdi
  
  if (dat_84aa0 <= 0)
    return;
  if (a3) // branch-flip
    v1 = sub_4d390(a3,&a2->field_0x20[0x10]);
  else {
    v1 = sub_4b660(a0,a1);
  }
  if (v1 == -1) {
    v1 = *__errno_location();
    if ((v1 != 0x5f) && (v1 != 0x3d)) {
      v2 = "fgetfilecon";
      if (!a3)
        v2 = "lgetfileconat";
      sub_324f0(v2,a1); // tail-call
      return;
    }
  }
  return;
}


// Function: sub_31e70 @ 0x31e70
void sub_31e70(int8 a0,char *a1,char a2)
{
  int4 v1; // eax
  char *v2; // rax
  char *v3;
  
  if ((dat_84aa0 <= 0) || (v3 = *(char **)(a0 + 0x30), !v3))
    return;
  if (a2 != '2') { // branch-flip
    v1 = sub_4b880(dat_81234,a1,v3);
    v3 = "setfileconat";
  }
  else {
    v1 = sub_4baa0(dat_81234,a1,v3);
    v3 = "lsetfileconat";
  }
  if ((v1 == -1) && (dat_83b82 & 0x20)) {
    if (dat_84d50)
      (*dat_84d50)();
    v2 = dcgettext(NULL,"%s: Cannot set SELinux context for file \'%s\'",5);
    error(0,*__errno_location(),v2,v3,a1); // tail-call
    return;
  }
  return;
}


// Function: sub_31f20 @ 0x31f20
void sub_31f20(int8 a0,char *a1,char a2,uint4 a3)
{
  uint8 v1; // rbx
  int8 v2;
  char *v3; // r14
  
  if ((1 <= dat_84a98) && (*(int8 *)(a0 + 0x158))) {
    v1 = 0;
    do {
      v2 = *(int8 *)(*(int8 *)(a0 + 0x160) + v1 * 0x18);
      v3 = (char *)(v2 + strlen("SCHILY.xattr."));
      if (a2 != '0') { // branch-flip
label_31fb5:
        if (!sub_31730(v3,0)) {
          v2 = v1 * 0x18 + *(int8 *)(a0 + 0x160);
          sub_31650(a1,(int4)a2,v3,*(int8 *)(v2 + 8),*(unsigned long *)(v2 + 0x10));
        }
      }
      else if ((strcmp(v3,"security.capability") != 0) != a3) goto label_31fb5;
      v1 += 1;
    } while (v1 < *(uint8 *)(a0 + 0x158));
  }
  return;
}


// Function: sub_32010 @ 0x32010
void sub_32010(int8 a0,unsigned short *a1) // return-dupe x3
{
  int8 v1;
  uint8 v2; // rax
  uint8 v3; // r13
  
  if (dat_84a54 <= 1) {
    *(char *)a1 = 0;
    return;
  }
  if (((1 <= dat_84a98) || (0 < dat_84aa0)) || (0 < dat_84a9c)) {
    *a1 = 0x20;
    if ((1 <= dat_84a98) && (*(int8 *)(a0 + 0x158))) {
      v3 = 0;
      do {
        v2 = strlen("SCHILY.xattr.");
        v1 = *(int8 *)(*(int8 *)(a0 + 0x160) + v3 * 0x18);
        if (!sub_31730((char *)(v2 + v1),0)) {
          *(char *)a1 = 0x2a;
          if (dat_84aa0 <= 0) goto label_320cb;
          goto label_320c0;
        }
        v3 += 1;
      } while (v3 < *(uint8 *)(a0 + 0x158));
    }
    if (dat_84aa0 <= 0) {
      if (dat_84a9c <= 0)
        return;
      goto label_320d5;
    }
label_320c0:
    if (*(int8 *)(a0 + 0x30))
      *(char *)a1 = 0x2e;
  }
label_320cb:
  if (dat_84a9c <= 0)
    return;
label_320d5:
  if ((!*(int8 *)(a0 + 0x40)) && (!*(int8 *)(a0 + 0x50)))
    return;
  *(char *)a1 = 0x2b;
  return;
}


// Function: sub_32150 @ 0x32150
void sub_32150(int8 a0)
{
  int8 v1;
  uint8 v2; // rax
  char *v3; // rax
  uint8 v4;
  
  if (dat_84a54 <= 2)
    return;
  if ((1 <= dat_84aa0) && (*(char **)(a0 + 0x30)))
    __fprintf_chk(dat_83f28,1,"  s: %s\n",*(char **)(a0 + 0x30));
  if ((1 <= dat_84a9c) && ((*(int8 *)(a0 + 0x40) || (*(int8 *)(a0 + 0x50))))) {
    __fprintf_chk(dat_83f28,1,"  a: ");
    sub_31760("",*(char **)(a0 + 0x38),*(uint8 *)(a0 + 0x40));
    v4 = *(uint8 *)(a0 + 0x50);
    if ((*(int8 *)(a0 + 0x40)) && (v4)) {
      __fprintf_chk(dat_83f28,1,",");
      v4 = *(uint8 *)(a0 + 0x50);
    }
    sub_31760("default:",*(char **)(a0 + 0x48),v4);
    __fprintf_chk(dat_83f28,1,"\n");
  }
  if ((1 <= dat_84a98) && (*(int8 *)(a0 + 0x158))) {
    v4 = 0;
    do {
      while( true ) {
        v2 = strlen("SCHILY.xattr.");
        v1 = v4 * 0x18;
        v3 = (char *)(v2 + *(int8 *)(*(int8 *)(a0 + 0x160) + v4 * 0x18));
        if (sub_31730(v3,0)) break;
        v4 += 1;
        __fprintf_chk(dat_83f28,1,"  x: %lu %s\n",*(unsigned long *)(*(int8 *)(a0 + 0x160) + 0x10 + v1),v3);
        if (*(uint8 *)(a0 + 0x158) <= v4)
          return;
      }
      v4 += 1;
    } while (v4 < *(uint8 *)(a0 + 0x158));
  }
  return;
}

