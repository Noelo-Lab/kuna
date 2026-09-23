// Function: main @ 0x26e0
int main(int argc,char **argv,char **envp)
{
  char *v1;
  char *v10; // stack - 0x48
  bool v2;
  int v3;
  unsigned int v4; // eax
  int v5; // eax
  FILE *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned int v9; // eax
  
  v7 = NULL;
  sub_4350(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7c40(sub_40f0);
  dat_18130 = 0;
  dat_18131 = 0;
  dat_18132 = 0;
  dat_18133 = 0;
  dat_18120 = 0x4b;
  dat_18128 = "";
  dat_1811c = 0;
  dat_18118 = 0;
  dat_18114 = 0;
  if (((2 <= argc) && (v1 = argv[1], *v1 == '-')) && ((unsigned int)((int)v1[1] - 0x30U) <= 9)) {
    v7 = &v1[1];
    v1 = *argv;
    argc -= 1;
    argv = &argv[1];
    *argv = v1;
  }
  v10 = NULL;
  while (v3 = getopt_long(argc,argv,"0123456789cstuw:p:g:",(option *)0xca80,NULL), v3 != -1) {
    if (0x77 < v3) {
label_2a9d:
      if ((unsigned int)(v3 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),(char)v3);
      sub_3dc0(1); // no-return
    }
    if (v3 <= 0x62) {
      if (v3 == -0x83) {
        sub_6dd0(stdout,"fmt","GNU coreutils",dat_d010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v3 == -0x82)
        sub_3dc0(0); // no-return
      goto label_2a9d;
    }
    switch(v3) {
      case 99:
        dat_18133 = 1;
        break;
      default:
        goto label_2a9d;
      case 0x67:
        v10 = optarg;
        break;
      case 0x70:
        sub_2fd0(optarg);
        break;
      case 0x73:
        dat_18131 = 1;
        break;
      case 0x74:
        dat_18132 = 1;
        break;
      case 0x75:
        dat_18130 = 1;
        break;
      case 0x77:
        v7 = optarg;
      
    }
  }
  if (v7) { // branch-flip
    v3 = sub_7450(v7,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_18120 = v3;
    if (v10) // branch-flip
      dat_18110 = sub_7450(v10,0,(long)v3,"",dcgettext(NULL,"invalid width",5),0);
    else {
label_29f3:
      dat_18110 = (dat_18120 * 0xbb) / 200;
    }
  }
  else {
    if (!v10) goto label_29f3;
    dat_18110 = sub_7450(v10,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_18120 = dat_18110 + 10;
  }
  if (optind != argc) { // branch-flip
    v2 = 0;
    v9 = 1;
    v3 = optind;
    if (argc <= optind) {
      return (v9 ^ 1) & 0xff; // return-dupe
    }
    do {
      v7 = argv[v3];
      if (strcmp(v7,"-")) { // branch-flip
        v6 = fopen(v7,"r");
        if (v6) // branch-flip
          v9 &= sub_3a30(v6,v7);
        else {
          v8 = sub_65c0(4,v7);
          v7 = dcgettext(NULL,"cannot open %s for reading",5);
          v9 = 0;
          error(0,*__errno_location(),v7,v8);
        }
      }
      else {
        v4 = sub_3a30(stdin,v7);
        v2 = 1;
        v9 &= v4;
      }
      v3 = optind + 1;
      optind = v3;
    } while (v3 < argc);
    if (!v2) {
      return (v9 ^ 1) & 0xff;
    }
  }
  else {
    v9 = sub_3a30(stdin,"-");
  }
  v3 = sub_41e0(stdin);
  if (v3) {
    v7 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v7);
    return v5;
  }
  return (v9 ^ 1) & 0xff;
}


// Function: sub_2c80 @ 0x2c80
void sub_2c80(bool a0) // return-dupe x2
{
  if (dat_18131) {
    dat_d0ec = dat_d0f0;
    return;
  }
  if (dat_18133) {
    if (a0) {
      dat_d0ec = dat_1810c;
      return;
    }
    dat_d0ec = dat_d0f0;
    return;
  }
  if (!dat_18132) {
    dat_d0ec = dat_d0f0;
    return;
  }
  if ((a0) && (dat_1810c != dat_d0f0)) {
    dat_d0ec = dat_1810c;
    return;
  }
  if (dat_d0ec != dat_d0f0)
    return;
  dat_d0ec = -(unsigned int)(dat_d0f0 == 0) & 3;
}


// Function: sub_2d10 @ 0x2d10
bool sub_2d10(int a0)
{
  if ((dat_d0e4 == dat_d0f4) && (dat_d0e4 + dat_1811c <= dat_1810c))
    return a0 != 10 && a0 != -1;
  return 0;
}


// Function: sub_2d50 @ 0x2d50
long sub_2d50(unsigned long a0)
{
  unsigned char v1;
  char v2 [16];
  char v3 [16];
  long v4;
  
  v4 = 0x1324;
  if (0xd121 <= a0) {
    v1 = *(unsigned char *)(a0 - 0x18);
    if (v1 & 2) // branch-flip
      v4 = (unsigned long)(-(unsigned int)((v1 & 8) == 0) & 0x58804) + 0x960;
    else {
      v4 = 0xce4;
      if (((!(v1 & 4)) && (v4 = 0x1324, 0xd149 <= a0)) && (*(unsigned char *)(a0 - 0x40) & 8)) {
        v3._8_8_ = 0;
        v3._0_8_ = SUB168(SEXT816(40000),8);
        v4 = SUB168((v3._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)(a0 - 0x20) + 2)),0) + 0x1324;
      }
    }
  }
  if (*(unsigned char *)(a0 + 0x10) & 1) // branch-flip
    v4 -= 0x640;
  else if (*(unsigned char *)(a0 + 0x10) & 8) {
    v2._8_8_ = 0;
    v2._0_8_ = SUB168(SEXT816(0x57e4),8);
    return v4 + SUB168((v2._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(*(int *)(a0 + 8) + 2)),0);
  }
  return v4;
}


// Function: sub_2e00 @ 0x2e00
long sub_2e00(void *a0,int a1) // early-return
{
  long v1; // rax
  long v2; // rdx
  
  if (dat_d100 == a0)
    return 0;
  v1 = (long)((dat_18110 - a1) * 10);
  v1 *= v1;
  if (dat_d100 == *(void **)((long)a0 + 0x20))
    return v1;
  v2 = (long)((a1 - *(int *)((long)a0 + 0x14)) * 10);
  return v1 + (v2 * v2 >> 1);
}


// Function: sub_2e50 @ 0x2e50
void sub_2e50(void)
{
  unsigned int v1;
  long v10;
  unsigned long v11; // r10
  unsigned long v12;
  unsigned long v13;
  int v2;
  int v3;
  int v4;
  int v5;
  long v6; // rax
  unsigned long v7;
  long v8; // rsi
  int v9;
  
  v5 = dat_18120;
  v4 = dat_d0f0;
  v3 = dat_d0ec;
  v2 = dat_d0e0;
  v11 = 0xd120;
  v1 = *(unsigned int *)(dat_d100 + 8);
  v13 = dat_d100 - 0x28;
  *(unsigned long *)(dat_d100 + 0x18) = 0;
  *(int *)(dat_d100 + 8) = dat_18120;
  v12 = dat_d100;
  if (0xd120 <= v13) {
    do {
      v10 = 0x7fffffffffffffff;
      v9 = v3;
      if (v13 == v11)
        v9 = v4;
      v9 += *(int *)(v13 + 8);
      v7 = v13;
      do {
        v7 += 0x28;
        v6 = sub_2e00(v7,v9);
        v6 += *(long *)(v7 + 0x18);
        if ((v13 == v11) && (1 <= v2)) {
          v8 = (long)((v9 - v2) * 10);
          v6 += v8 * v8 >> 1;
        }
        if (v6 < v10) {
          *(unsigned long *)(v13 + 0x20) = v7;
          *(int *)(v13 + 0x14) = v9;
          v10 = v6;
        }
      } while ((v12 != v7) && (v9 = v9 + *(int *)(v7 + 8) + *(int *)(v7 - 0x1c), v9 < v5));
      v7 = v13 - 0x28;
      *(long *)(v13 + 0x18) = sub_2d50(v13) + v10;
      v13 = v7;
    } while (v11 <= v7);
  }
  *(unsigned int *)(v12 + 8) = v1;
}


// Function: sub_2fd0 @ 0x2fd0
void sub_2fd0(char *a0)
{
  char *v1; // rax
  int v2; // edi
  
  dat_18118 = 0;
  if (*a0 == ' ') {
    v2 = (int)a0;
    do {
      dat_18118 = (1U - v2) + (int)a0;
      a0 = &a0[1];
    } while (*a0 == ' ');
  }
  dat_18128 = a0;
  dat_1811c = (int)strlen(a0);
  v1 = &a0[dat_1811c];
  if (a0 < v1) {
    do {
      if (v1[-1] != ' ') break;
      v1 = &v1[-1];
    } while (v1 != a0);
  }
  *v1 = '\0';
  dat_18114 = (int)v1 - (int)a0;
}


// Function: sub_3050 @ 0x3050
void sub_3050(FILE *a0,int a1)
{
  int v1;
  
  do {
    if (a1 != 0x20) { // branch-flip
      if (a1 != 9)
        return;
      dat_d0f8 = 1;
      v1 = dat_1810c + 7;
      if (0 <= dat_1810c)
        v1 = dat_1810c;
      dat_1810c = (v1 >> 3) * 8 + 8;
    }
    else {
      dat_1810c += 1;
    }
    a1 = getc_unlocked(a0);
  } while( true );
}


// Function: sub_30b0 @ 0x30b0
void sub_30b0(FILE *a0) // return-dupe
{
  unsigned char v1;
  unsigned int v2;
  unsigned char *v3;
  
  dat_1810c = 0;
  v2 = sub_3050(a0,getc_unlocked(a0));
  if (!dat_18114) {
    if (dat_1810c <= dat_18118) {
      dat_d0e4 = dat_1810c;
      return;
    }
    dat_d0e4 = dat_18118;
    return;
  }
  v1 = *dat_18128;
  dat_d0e4 = dat_1810c;
  v3 = dat_18128;
  while( true ) {
    if (!v1) {
      sub_3050(a0,v2); // tail-call
      return;
    }
    if (v1 != v2) break;
    v3 = &v3[1];
    dat_1810c += 1;
    v2 = getc_unlocked(a0);
    v1 = *v3;
  }
}


// Function: sub_3150 @ 0x3150
void sub_3150(int a0) // return-dupe
{
  int v1;
  int v2;
  int v3; // eax
  int v4;
  
  v1 = dat_18108 + a0;
  v4 = dat_18108;
  if (dat_d0f8) {
    v2 = v1 + 7;
    if (0 <= v1)
      v2 = v1;
    if ((dat_18108 + 1 < (int)(v2 & 0xfffffff8)) && (dat_18108 < (int)(v2 & 0xfffffff8))) {
      do {
        putchar_unlocked(9);
        v4 = dat_18108 + 7;
        if (0 <= dat_18108)
          v4 = dat_18108;
        v3 = (v4 >> 3) + 1;
        v4 = v3 * 8;
        dat_18108 = v4;
      } while (v3 < v2 >> 3);
    }
  }
  if (v1 <= v4)
    return;
  do {
    putchar_unlocked(0x20);
    dat_18108 += 1;
  } while (dat_18108 < v1);
}


// Function: sub_31f0 @ 0x31f0
void sub_31f0(struct_0 *a0)
{
  char v1;
  char *v2;
  int v3;
  char *v4; // rbp
  
  v2 = a0->field_0x0;
  v3 = 0;
  if (a0->field_0x8) {
    v4 = &v2[(unsigned int)a0->field_0x8];
    do {
      v1 = *v2;
      v2 = &v2[1];
      putchar_unlocked((int)v1);
    } while (v4 != v2);
    v3 = a0->field_0x8;
  }
  dat_18108 += v3;
}


// Function: sub_3230 @ 0x3230
void sub_3230(void *a0,int a1)
{
  long v1;
  
  dat_18108 = 0;
  sub_3150(dat_d0f4);
  fputs_unlocked(dat_18128,stdout);
  dat_18108 = dat_18114 + dat_18108;
  sub_3150(a1 - dat_18108);
  v1 = *(long *)((long)a0 + 0x20);
  for (; a0 != (void *)(v1 + -0x28); a0 = (void *)((long)a0 + 0x28)) {
    sub_31f0(a0);
    sub_3150(*(unsigned int *)((long)a0 + 0xc));
  }
  sub_31f0(a0);
  dat_d0e0 = dat_18108;
  putchar_unlocked(10); // tail-call
}


// Function: sub_32d0 @ 0x32d0
void sub_32d0(long a0) // return-dupe
{
  long *v1;
  long v2;
  
  sub_3230((void *)0xd120,dat_d0f0);
  v2 = dat_d140;
  if (dat_d140 == a0)
    return;
  do {
    sub_3230(v2,dat_d0ec);
    v1 = (long *)(v2 + 0x20);
    v2 = *v1;
  } while (a0 != *v1);
}


// Function: sub_3320 @ 0x3320
int sub_3320(FILE *a0,int a1)
{
  int v1; // eax
  int v2;
  int v3; // edx
  char *v4;
  
  dat_18108 = 0;
  if (dat_1810c <= dat_d0e4) { // branch-flip
    if (a1 == 10 || a1 == -1) {
      v1 = a1;
      dat_18108 = 0;
      return v1;
    }
    sub_3150(dat_d0e4);
    v3 = 0;
    v4 = dat_18128;
    v1 = dat_18108;
    v2 = dat_1810c;
    if (dat_18108 == dat_1810c) goto label_33e4;
label_33a2:
    do {
      if (!*v4) goto label_33ac;
      putchar_unlocked((int)*v4);
      v4 = &v4[1];
      v1 = dat_18108 + 1;
      v2 = dat_1810c;
      dat_18108 = v1;
    } while (v1 != dat_1810c);
  }
  else {
    sub_3150(dat_d0e4);
    v4 = dat_18128;
    v1 = dat_18108;
    v2 = dat_1810c;
    if (dat_18108 != dat_1810c) goto label_33a2;
  }
label_33ac:
  if (a1 == 10 || a1 == -1) {
    if (a1 != -1) {
      v1 = a1;
      return v1;
    }
    v1 = dat_18114 + dat_d0e4;
    if (dat_1810c < v1) {
      v1 = -1;
      return v1;
    }
    putchar_unlocked(10);
    v1 = -1;
    return v1;
  }
  v3 = v2 - v1;
label_33e4:
  sub_3150(v3);
  do {
    putchar_unlocked(a1);
    v1 = getc_unlocked(a0);
    if (v1 == 10) {
      v1 = 10;
      return v1;
    }
    a1 = v1;
  } while (v1 != -1);
  v1 = -1;
  return v1;
}


// Function: sub_3460 @ 0x3460
void sub_3460(struct_1 *a0)
{
  char *v1;
  char *v2; // rax
  long *v3; // rax
  char *v4;
  int v5; // ebp
  
  v1 = a0->field_0x0;
  v4 = &v1[(long)a0->field_0x8 + -1];
  v2 = strchr("([\'`\"",(int)*v1);
  v3 = __ctype_b_loc();
  a0->field_0x10 = *(unsigned char *)(*v3 + (unsigned long)(unsigned char)*v4 * 2) & 4 | v2 != NULL | a0->field_0x10 & 0xfa;
  if (v4 <= v1) // branch-flip
    v5 = (int)*v4;
  else {
    while( true ) {
      v5 = (int)*v4;
      if (!strchr(")]\'\"",v5)) break;
      if (v1 == &v4[-1]) {
        v5 = (int)v4[-1];
        break;
      }
      v4 = &v4[-1];
    }
  }
  a0->field_0x10 = a0->field_0x10 & 0xfd | (strchr(".?!",v5) != NULL) * '\x02';
}


// Function: sub_3530 @ 0x3530
void sub_3530(void)
{
  long *v1;
  long v2;
  void *v3; // rax
  long v4; // rax
  long *v5;
  long v6;
  long *v7;
  long *v8;
  
  v5 = dat_d100;
  if (dat_d100 == (long *)0xd120) {
    fwrite_unlocked((void *)0x16d80,1,dat_16d60 - 0x16d80,stdout);
    dat_16d60 = 0x16d80;
    return;
  }
  sub_2e50();
  v8 = v5;
  if (v5 != dat_d140) {
    v2 = 0x7fffffffffffffff;
    v7 = dat_d140;
    v6 = dat_d140[3];
    do {
      v1 = (long *)v7[4];
      v6 -= v1[3];
      if (v6 < v2) {
        v2 = v6;
        v8 = v7;
      }
      if (v2 <= 0x7ffffffffffffff6)
        v2 += 9;
      v7 = v1;
      v6 = v1[3];
    } while (v5 != v1);
  }
  sub_32d0(v8);
  v6 = dat_16d60;
  v3 = __memmove_chk((void *)0x16d80,(void *)*v8,dat_16d60 - *v8,5000);
  v2 = *v8;
  v4 = (long)((int)v2 - (int)v3);
  dat_16d60 = v6 - v4;
  if (v8 <= dat_d100) {
    v5 = v8;
    while( true ) {
      v7 = &v5[5];
      *v5 = v2 - v4;
      if (dat_d100 < v7) break;
      v2 = *v7;
      v5 = v7;
    }
  }
  v2 = (long)dat_d100 - (long)&v8[-0x1a24];
  __memmove_chk((void *)0xd120,v8,(long)dat_d100 + (0x28U - (long)v8),40000);
  dat_d100 = (long *)v2;
}


// Function: sub_3690 @ 0x3690
void sub_3690(FILE *a0,int a1)
{
  bool v1; // dl
  int v2; // edx
  long *v3;
  
  v3 = dat_d100;
label_36b8:
  do {
    *v3 = (long)dat_16d60;
    do {
      if (dat_16d60 == (char *)0x18108) {
        sub_2c80(1);
        sub_3530();
      }
      *dat_16d60 = (char)a1;
      dat_16d60 = &dat_16d60[1];
      a1 = getc_unlocked(a0);
    } while ((a1 != -1) && (!sub_4030(a1)));
    v2 = (int)dat_16d60 - (int)*dat_d100;
    *(int *)&dat_d100[1] = v2;
    v2 = dat_1810c + v2;
    dat_1810c = v2;
    sub_3460(dat_d100);
    a1 = sub_3050(a0,a1);
    v2 = dat_1810c - v2;
    *(int *)((long)dat_d100 + 0xc) = v2;
    if (a1 != -1) { // branch-flip
      if (*(unsigned char *)&dat_d100[2] & 2) { // branch-flip
        v1 = 1 < v2 || a1 == 10;
        if (1 < v2 || a1 == 10) goto label_3791;
        *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] & 0xf7;
      }
      else {
        v1 = 0;
label_3791:
        *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] & 0xf7 | v1 * '\b';
        if (a1 == 10) goto label_37d4;
      }
      if (dat_18130) goto label_37d4;
      if (dat_d100 != (long *)0x16d10) {
        v3 = &dat_d100[5];
        dat_d100 = v3;
        goto label_36b8;
      }
label_3820:
      sub_2c80(1);
      sub_3530();
    }
    else {
      *(unsigned char *)&dat_d100[2] = *(unsigned char *)&dat_d100[2] | 8;
label_37d4:
      *(unsigned int *)((long)dat_d100 + 0xc) = ((*(unsigned char *)&dat_d100[2] & 8) != 0) + 1;
      if (dat_d100 == (long *)0x16d10) goto label_3820;
    }
    v3 = &dat_d100[5];
    dat_d100 = v3;
    if ((a1 == 10) || (a1 == -1)) {
      sub_30b0(a0); // tail-call
      return;
    }
  } while( true );
}


// Function: sub_3850 @ 0x3850
unsigned long sub_3850(FILE *a0)
{
  char v1; // al
  int v2;
  unsigned int v3; // eax
  unsigned int v4; // eax
  unsigned long v5; // rsi
  
  dat_d0e0 = 0;
  v2 = dat_d0e8;
  while ((((v2 == 10 || (v2 == -1)) || (dat_d0e4 < dat_18118)) || (dat_1810c < dat_1811c + dat_d0e4))) {
    v2 = sub_3320(a0);
    if (v2 == -1) {
      dat_d0e8 = v2;
      return 0;
    }
    putchar_unlocked(10);
    v2 = sub_30b0(a0);
  }
  dat_d0f4 = dat_d0e4;
  dat_d0f0 = dat_1810c;
  dat_16d60 = 0x16d80;
  dat_d100 = 0xd120;
  v3 = sub_3690(a0);
  v5 = (unsigned long)v3;
  sub_2c80((unsigned char)sub_2d10(v3));
  v4 = (unsigned int)v5;
  if (!dat_18131) {
    v1 = sub_2d10(v5 & 0xffffffff);
    v4 = (unsigned int)v5;
    if (dat_18133) { // branch-flip
      if (v1) {
        do {
          v4 = sub_3690(a0);
          v1 = sub_2d10(v4);
          if (!v1) break;
        } while (dat_1810c == dat_d0ec);
      }
    }
    else if (dat_18132) { // branch-flip
      if ((v1) && (dat_1810c != dat_d0f0)) {
        do {
          v4 = sub_3690(a0);
          v1 = sub_2d10(v4);
          if (!v1) break;
        } while (dat_1810c == dat_d0ec);
      }
    }
    else {
      while( true ) {
        v4 = (unsigned int)v5;
        if ((!sub_2d10(v5 & 0xffffffff)) || (dat_1810c != dat_d0ec)) break;
        v5 = (unsigned long)(unsigned int)sub_3690(a0);
      }
    }
  }
  if (dat_d100 <= 0xd120)
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph"); // no-return
  *(unsigned char *)(dat_d100 - 0x18) = *(unsigned char *)(dat_d100 - 0x18) | 10;
  dat_d0e8 = v4;
  return 1;
}


// Function: sub_3a30 @ 0x3a30
unsigned int sub_3a30(FILE *a0,char *a1)
{
  int v1;
  unsigned int v2; // eax
  char *v3; // rax
  
  sub_41b0(a0,2);
  dat_d0f8 = 0;
  dat_d0ec = 0;
  dat_d0e8 = sub_30b0(a0);
  while (v2 = sub_3850(a0), (char)v2) {
    sub_2e50();
    sub_32d0(dat_d100);
  }
  if (ferror_unlocked(a0)) { // branch-flip
    if (a0 != stdin) // branch-flip
      sub_41e0(a0);
    else {
      clearerr_unlocked(a0);
    }
    v3 = (char *)sub_66c0(0,3,a1);
  }
  else {
    if (a0 == stdin) {
      clearerr_unlocked(a0);
      return 1;
    }
    if (!sub_41e0(a0))
      return 1;
    v1 = *__errno_location();
    if (v1 < 0)
      return 1;
    v3 = (char *)sub_66c0(0,3,a1);
    if (v1) {
      error(0,v1,"%s",v3);
      return v2;
    }
  }
  error(0,0,dcgettext(NULL,"read error",5),v3);
  return v2;
}


// Function: sub_3dc0 @ 0x3dc0
void sub_3dc0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_18148;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n",5),v1);
    sub_2f70();
    sub_2fa0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_3b90();
  }
  exit(a0); // no-return
}

