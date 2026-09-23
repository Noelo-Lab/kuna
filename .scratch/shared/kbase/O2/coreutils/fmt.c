// Function: sub_26a0 @ 0x26a0
unsigned long sub_26a0(int a0,char **a1)
{
  bool v1;
  unsigned long v10;
  unsigned long v11; // rdx
  unsigned int v12; // eax
  char **v13;
  char *v14; // stack - 0x48
  char *v15; // stack - 0x40
  int v2;
  unsigned int v3; // eax
  char *v4; // rax
  void *v5; // rax
  char *v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rdx
  
  sub_4350(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7b90(sub_4100);
  dat_17130 = 0;
  dat_17131 = 0;
  dat_17132 = 0;
  dat_17133 = 0;
  dat_17120 = 0x4b;
  dat_17128 = "";
  dat_1711c = 0;
  dat_17118 = 0;
  dat_17114 = 0;
  v15 = NULL;
  v13 = a1;
  if (((2 <= a0) && (v6 = a1[1], *v6 == '-')) && ((unsigned int)((int)v6[1] - 0x30U) <= 9)) {
    v15 = &v6[1];
    v13 = &a1[1];
    a0 -= 1;
    *v13 = *a1;
  }
  v14 = NULL;
  while (v2 = getopt_long(a0,v13,"0123456789cstuw:p:g:",(void *)0xbaa0,NULL), v2 != -1) {
    if (0x77 < v2) {
label_2acd:
      if ((unsigned int)(v2 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),v2);
      sub_3d70(1); // no-return
    }
    if (v2 <= 0x62) {
      if (v2 == -0x83) {
        sub_6c80(stdout,"fmt","GNU coreutils",dat_c010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_3d70(0); // no-return
      goto label_2acd;
    }
    switch(v2) {
      case 99:
        dat_17133 = 1;
        break;
      default:
        goto label_2acd;
      case 0x67:
        v14 = optarg;
        break;
      case 0x70:
        dat_17118 = 0;
        v6 = optarg;
        if (*optarg == ' ') {
          do {
            dat_17118 = (1U - (int)optarg) + (int)v6;
            v6 = &v6[1];
          } while (*v6 == ' ');
        }
        dat_17128 = v6;
        dat_1711c = (int)strlen(v6);
        v4 = &v6[dat_1711c];
        if (v6 < v4) {
          do {
            if (v4[-1] != ' ') break;
            v4 = &v4[-1];
          } while (v6 != v4);
        }
        *v4 = '\0';
        dat_17114 = (int)v4 - (int)v6;
        break;
      case 0x73:
        dat_17131 = 1;
        break;
      case 0x74:
        dat_17132 = 1;
        break;
      case 0x75:
        dat_17130 = 1;
        break;
      case 0x77:
        v15 = optarg;
      
    }
  }
  if (v15) { // branch-flip
    v2 = sub_74a0(v15,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = v2;
    if (v14) { // branch-flip
      dat_17110 = sub_74a0(v14,0,(long)v2,"",dcgettext(NULL,"invalid width",5),0);
      v10 = v9;
    }
    else {
label_2a19:
      dat_17110 = (dat_17120 * 0xbb) / 200;
      v10 = (long)(dat_17120 * 0xbb) % 200 & 0xffffffff;
    }
  }
  else {
    if (!v14) goto label_2a19;
    dat_17110 = sub_74a0(v14,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = dat_17110 + 10;
    v10 = v11;
  }
  if (optind != a0) { // branch-flip
    v1 = 0;
    v12 = 1;
    v2 = optind;
    if (a0 <= optind) {
      return (unsigned long)((v12 ^ 1) & 0xff); // return-dupe
    }
    do {
      v6 = v13[v2];
      if (strcmp(v6,"-")) { // branch-flip
        v5 = fopen(v6,"r");
        if (v5) // branch-flip
          v12 &= sub_3700(v5);
        else {
          v8 = sub_6030(4,v6);
          v6 = dcgettext(NULL,"cannot open %s for reading",5);
          v12 = 0;
          error(0,*__errno_location(),v6,v8);
        }
      }
      else {
        v3 = sub_3700(stdin,v6);
        v1 = 1;
        v12 &= v3;
      }
      v2 = optind + 1;
      optind = v2;
    } while (v2 < a0);
    if (!v1) {
      return (unsigned long)((v12 ^ 1) & 0xff);
    }
  }
  else {
    v12 = sub_3700(stdin,"-",v10);
  }
  v2 = sub_41f0(stdin);
  if (v2) {
    v6 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v6);
    return v7;
  }
  return (unsigned long)((v12 ^ 1) & 0xff);
}


// Function: sub_2cb0 @ 0x2cb0
void sub_2cb0(void)
{
  unsigned char v1;
  unsigned long v10;
  long v11;
  long v12; // r14
  int v13;
  unsigned int v2;
  int v3;
  char v4 [16];
  char v5 [16];
  unsigned long v6;
  unsigned long v7;
  long v8;
  unsigned long v9; // rdx
  
  v2 = *(unsigned int *)(dat_c100 + 8);
  *(unsigned long *)(dat_c100 + 0x18) = 0;
  *(int *)(dat_c100 + 8) = dat_17120;
  v7 = dat_c100;
  while (v6 = v7 - 0x28, 0xc120 <= v6) {
    v3 = *(int *)(v7 - 0x20);
    v13 = dat_c0ec;
    if (v6 == 0xc120)
      v13 = dat_c0f0;
    v11 = 0x7fffffffffffffff;
    v13 += v3;
    v10 = v6;
    do {
      v9 = v10 + 0x28;
      if (dat_c100 != v9) { // branch-flip
        v8 = (long)((dat_17110 - v13) * 10);
        v8 *= v8;
        if (dat_c100 != *(unsigned long *)(v10 + 0x48)) {
          v12 = (long)((v13 - *(int *)(v10 + 0x3c)) * 10);
          v8 += v12 * v12 >> 1;
        }
      }
      else {
        v8 = 0;
      }
      v8 += *(long *)(v10 + 0x40);
      if ((v6 == 0xc120) && (1 <= dat_c0e0)) {
        v12 = (long)((v13 - dat_c0e0) * 10);
        v8 += v12 * v12 >> 1;
      }
      if (v8 < v11) {
        *(unsigned long *)(v7 - 8) = v9;
        *(int *)(v7 - 0x14) = v13;
        v11 = v8;
      }
    } while ((dat_c100 != v9) && (v13 = v13 + *(int *)(v10 + 0x30) + *(int *)(v10 + 0xc), v10 = v9, v13 < dat_17120));
    v8 = 0x1324;
    if (0xc121 <= v6) {
      v1 = *(unsigned char *)(v7 - 0x40);
      if (v1 & 2) // branch-flip
        v8 = (unsigned long)(-(unsigned int)((v1 & 8) == 0) & 0x58804) + 0x960;
      else {
        v8 = 0xce4;
        if (((!(v1 & 4)) && (v8 = 0x1324, 0xc149 <= v6)) && (*(unsigned char *)(v7 - 0x68) & 8)) {
          v5._8_8_ = 0;
          v5._0_8_ = SUB168(SEXT816(40000),8);
          v8 = SUB168((v5._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)(v7 - 0x48) + 2)),0) + 0x1324;
        }
      }
    }
    if (*(unsigned char *)(v7 - 0x18) & 1) // branch-flip
      v8 -= 0x640;
    else if (*(unsigned char *)(v7 - 0x18) & 8) {
      v4._8_8_ = 0;
      v4._0_8_ = SUB168(SEXT816(0x57e4),8);
      v8 += SUB168((v4._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(v3 + 2)),0);
    }
    *(long *)(v7 - 0x10) = v11 + v8;
    v7 = v6;
  }
  *(unsigned int *)(dat_c100 + 8) = v2;
}


// Function: sub_2ea0 @ 0x2ea0
void sub_2ea0(long a0,unsigned int a1)
{
  int v1;
  unsigned char *v2;
  unsigned long v3; // rax
  
  v3 = (unsigned long)a1;
  do {
    while ((int)v3 != 0x20) {
      if ((int)v3 != 9)
        return;
      dat_c0f8 = 1;
      v1 = dat_1710c + 7;
      if (0 <= dat_1710c)
        v1 = dat_1710c;
      dat_1710c = (v1 >> 3) * 8 + 8;
      v2 = *(unsigned char **)(a0 + 8);
      if (v2 < *(unsigned char **)(a0 + 0x10)) goto label_2ee4;
label_2f0d:
      v3 = __uflow(a0);
    }
    dat_1710c += 1;
    v2 = *(unsigned char **)(a0 + 8);
    if (*(unsigned char **)(a0 + 0x10) <= v2) goto label_2f0d;
label_2ee4:
    *(unsigned char **)(a0 + 8) = &v2[1];
    v3 = (unsigned long)*v2;
  } while( true );
}


// Function: sub_2f30 @ 0x2f30
void sub_2f30(long a0) // return-dupe
{
  unsigned char v1;
  unsigned char *v2;
  unsigned int v3;
  unsigned int v4; // eax
  unsigned long v5;
  unsigned char *v6;
  
  v6 = *(unsigned char **)(a0 + 8);
  dat_1710c = 0;
  if (*(unsigned char **)(a0 + 0x10) <= v6) // branch-flip
    v4 = __uflow(a0);
  else {
    *(unsigned char **)(a0 + 8) = &v6[1];
    v4 = (unsigned int)*v6;
  }
  v5 = sub_2ea0(a0,v4);
  v3 = (unsigned int)v5;
  if (!dat_17114) {
    if (dat_1710c <= dat_17118) {
      dat_c0e4 = dat_1710c;
      return;
    }
    dat_c0e4 = dat_17118;
    return;
  }
  v1 = *dat_17128;
  dat_c0e4 = dat_1710c;
  v6 = dat_17128;
  while( true ) {
    if (!v1) {
      sub_2ea0(a0,v3); // tail-call
      return;
    }
    if ((unsigned int)v1 != (unsigned int)v5) break;
    dat_1710c += 1;
    v2 = *(unsigned char **)(a0 + 8);
    if (*(unsigned char **)(a0 + 0x10) <= v2) // branch-flip
      v5 = __uflow(a0);
    else {
      *(unsigned char **)(a0 + 8) = &v2[1];
      v5 = (unsigned long)*v2;
    }
    v3 = (unsigned int)v5;
    v1 = v6[1];
    v6 = &v6[1];
  }
}


// Function: sub_3000 @ 0x3000
void sub_3000(unsigned long *a0)
{
  unsigned char v1;
  unsigned int v2;
  unsigned char *v3;
  unsigned char *v4;
  unsigned char *v5; // rbx
  int v6;
  
  v2 = *(unsigned int *)&a0[1];
  v3 = (unsigned char *)*a0;
  v6 = 0;
  if (v2) {
    v4 = v3;
    do {
      v5 = &v4[1];
      v1 = *v4;
      v4 = *(unsigned char **)((long)stdout + 0x28);
      if (*(unsigned char **)((long)stdout + 0x30) <= v4) // branch-flip
        __overflow(stdout,(unsigned int)v1);
      else {
        *(unsigned char **)((long)stdout + 0x28) = &v4[1];
        *v4 = v1;
      }
      v4 = v5;
    } while (v5 != &v3[v2]);
    v6 = *(int *)&a0[1];
  }
  dat_17108 += v6;
}


// Function: sub_3060 @ 0x3060
void sub_3060(int a0)
{
  unsigned int v1;
  char *v2;
  unsigned int v3;
  int v4;
  int v5; // eax
  
  v1 = dat_17108 + a0;
  v4 = dat_17108;
  if (dat_c0f8) {
    v3 = v1 + 7;
    if (0 <= (int)v1)
      v3 = v1;
    if (((int)(v3 & 0xfffffff8) <= dat_17108 + 1) || ((int)(v3 & 0xfffffff8) <= dat_17108)) goto label_30df;
    do {
      v2 = *(char **)((long)stdout + 0x28);
      if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
        __overflow(stdout,9);
      else {
        *(char **)((long)stdout + 0x28) = &v2[1];
        *v2 = 9;
      }
      v4 = dat_17108 + 7;
      if (0 <= dat_17108)
        v4 = dat_17108;
      v5 = (v4 >> 3) + 1;
      dat_17108 = v5 * 8;
      v4 = dat_17108;
    } while (v5 < (int)v3 >> 3);
  }
label_30df:
  while (v4 < (int)v1) {
    v2 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)((long)stdout + 0x28) = &v2[1];
      *v2 = 0x20;
    }
    dat_17108 += 1;
    v4 = dat_17108;
  }
}


// Function: sub_3140 @ 0x3140
void sub_3140(long a0,int a1)
{
  long v1;
  char *v2;
  
  dat_17108 = 0;
  sub_3060(dat_c0f4);
  fputs_unlocked(dat_17128,stdout);
  dat_17108 = dat_17114 + dat_17108;
  sub_3060(a1 - dat_17108);
  v1 = *(long *)(a0 + 0x20);
  for (; a0 != v1 + -0x28; a0 = a0 + 0x28) {
    sub_3000(a0);
    sub_3060(*(unsigned int *)(a0 + 0xc));
  }
  sub_3000(a0);
  dat_c0e0 = dat_17108;
  v2 = *(char **)((long)stdout + 0x28);
  if (v2 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}


// Function: sub_3200 @ 0x3200
void sub_3200(void)
{
  long *v1;
  long v2;
  void *v3; // rax
  long v4; // rax
  long *v5;
  long v6;
  long *v7;
  long *v8;
  
  v5 = dat_c100;
  if (dat_c100 == (long *)0xc120) {
    fwrite_unlocked((void *)0x15d80,1,dat_15d60 - 0x15d80,stdout);
    dat_15d60 = 0x15d80;
    return;
  }
  sub_2cb0();
  v8 = v5;
  if (v5 != dat_c140) {
    v2 = 0x7fffffffffffffff;
    v7 = dat_c140;
    v6 = dat_c140[3];
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
  sub_3140(0xc120,dat_c0f0);
  v5 = dat_c140;
  v2 = dat_15d60;
  while (dat_15d60 = v2, v8 != v5) {
    sub_3140(v5,dat_c0ec);
    v5 = (long *)v5[4];
    v2 = dat_15d60;
  }
  v3 = __memmove_chk((void *)0x15d80,(void *)*v8,v2 - *v8,5000);
  v6 = *v8;
  v4 = (long)((int)v6 - (int)v3);
  dat_15d60 = v2 - v4;
  v5 = v8;
  if (v8 <= dat_c100) {
    while( true ) {
      v7 = &v5[5];
      *v5 = v6 - v4;
      if (dat_c100 < v7) break;
      v6 = *v7;
      v5 = v7;
    }
  }
  v2 = (long)dat_c100 - (long)&v8[-0x1824];
  __memmove_chk((void *)0xc120,v8,(long)dat_c100 + (0x28U - (long)v8),40000);
  dat_c100 = (long *)v2;
}


// Function: sub_3420 @ 0x3420
void sub_3420(long a0,unsigned int a1)
{
  unsigned char *v1;
  unsigned char *v10;
  char *v2;
  bool v3; // al
  int v4; // eax
  long *v5; // rax
  char *v6;
  char *v7; // rax
  int v8; // eax
  long *v9;
  
  v5 = __ctype_b_loc();
  v9 = dat_c100;
label_3450:
  do {
    *v9 = (long)dat_15d60;
    v6 = dat_15d60;
    v2 = dat_15d60;
    do {
      while( true ) {
        dat_15d60 = v6;
        if (dat_15d60 == (char *)0x17108) {
          dat_15d60 = v2;
          sub_3390(1);
          sub_3200();
        }
        v6 = &dat_15d60[1];
        *dat_15d60 = (char)a1;
        v10 = *(unsigned char **)(a0 + 8);
        dat_15d60 = v6;
        if (*(unsigned char **)(a0 + 0x10) <= v10) { // branch-flip
          a1 = __uflow(a0);
          v6 = dat_15d60;
          if (a1 == 0xffffffff) goto label_34a7;
        }
        else {
          *(unsigned char **)(a0 + 8) = &v10[1];
          a1 = (unsigned int)*v10;
        }
        v2 = dat_15d60;
        if ((int)a1 <= 0xd) break;
        if (a1 == 0x20) goto label_34a7;
      }
    } while ((int)a1 <= 8);
label_34a7:
    v9 = dat_c100;
    v1 = (unsigned char *)*dat_c100;
    v4 = (int)v6 - (int)v1;
    *(int *)&dat_c100[1] = v4;
    v8 = dat_1710c + v4;
    v10 = &v1[(long)v4 + -1];
    dat_1710c = v8;
    v7 = strchr("([\'`\"",(int)(char)*v1);
    *(unsigned char *)&v9[2] = *(unsigned char *)(*v5 + (unsigned long)*v10 * 2) & 4 | v7 != NULL | *(unsigned char *)&v9[2] & 0xfa;
    if (v10 <= v1) // branch-flip
      v4 = (int)(char)*v10;
    else {
      while( true ) {
        v4 = (int)(char)*v10;
        if (!strchr(")]\'\"",v4)) break;
        if (v1 == &v10[-1]) {
          v4 = (int)(char)v10[-1];
          break;
        }
        v10 = &v10[-1];
      }
    }
    *(unsigned char *)&v9[2] = *(unsigned char *)&v9[2] & 0xfd | (strchr(".?!",v4) != NULL) * '\x02';
    a1 = sub_2ea0(a0,a1);
    v8 = dat_1710c - v8;
    *(int *)((long)dat_c100 + 0xc) = v8;
    if (a1 != 0xffffffff) { // branch-flip
      if (*(unsigned char *)&dat_c100[2] & 2) { // branch-flip
        v3 = 1 < v8 || a1 == 10;
        if (1 < v8 || a1 == 10) goto label_35b7;
        *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] & 0xf7;
      }
      else {
        v3 = 0;
label_35b7:
        *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] & 0xf7 | v3 * '\b';
        if (a1 == 10) goto label_363c;
      }
      if (dat_17130) goto label_363c;
      if (dat_c100 != (long *)0x15d10) {
        v9 = &dat_c100[5];
        dat_c100 = v9;
        goto label_3450;
      }
label_36d0:
      sub_3390(1);
      sub_3200();
    }
    else {
      *(unsigned char *)&dat_c100[2] = *(unsigned char *)&dat_c100[2] | 8;
label_363c:
      *(unsigned int *)((long)dat_c100 + 0xc) = ((*(unsigned char *)&dat_c100[2] & 8) != 0) + 1;
      if (dat_c100 == (long *)0x15d10) goto label_36d0;
    }
    v9 = &dat_c100[5];
    dat_c100 = v9;
    if ((a1 == 10) || (a1 == 0xffffffff)) {
      sub_2f30(a0); // tail-call
      return;
    }
  } while( true );
}


// Function: sub_3700 @ 0x3700
unsigned long sub_3700(unsigned char *a0,unsigned long a1) // return-dupe
{
  unsigned char v1;
  unsigned long v10;
  int v11;
  unsigned long v12;
  unsigned char *v13;
  bool v14;
  unsigned char *v2;
  char *v3;
  unsigned int v4;
  int v5;
  unsigned int v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  unsigned int v9; // ecx
  
  sub_41c0(a0,2);
  dat_c0f8 = 0;
  dat_c0ec = 0;
  v4 = sub_2f30(a0);
  dat_c0e8 = v4;
label_3746:
  dat_c0e0 = 0;
  do {
    if ((v4 != 10) && (v4 != 0xffffffff)) {
      if ((dat_17118 <= (int)dat_c0e4) && ((int)(dat_1711c + dat_c0e4) <= dat_1710c)) goto label_3a07;
      dat_17108 = 0;
      if (dat_1710c <= (int)dat_c0e4) {
        sub_3060(dat_c0e4);
        v11 = dat_1710c;
        if (dat_17108 != dat_1710c) {
label_393a:
          v14 = 1;
          v13 = dat_17128;
          v5 = dat_17108;
          goto label_37df;
        }
        sub_3060(0);
label_3981:
        do {
          v3 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
            __overflow(stdout,v4 & 0xff);
          else {
            *(char **)((long)stdout + 0x28) = &v3[1];
            *v3 = (char)v4;
          }
          v13 = *(unsigned char **)&a0[8];
          if (*(unsigned char **)&a0[0x10] <= v13) { // branch-flip
            v4 = __uflow(a0);
            v14 = v4 != 0xffffffff;
          }
          else {
            *(unsigned char **)&a0[8] = &v13[1];
            v4 = (unsigned int)*v13;
            v14 = 1;
          }
        } while ((v4 != 10) && (v14));
        goto label_38db;
      }
      v14 = 1;
label_379e:
      dat_17108 = 0;
      sub_3060(dat_c0e4);
      v13 = dat_17128;
      v5 = dat_17108;
      v11 = dat_1710c;
      if (dat_1710c != dat_17108) {
label_37df:
        do {
          v1 = *v13;
          if (!v1) break;
          v13 = &v13[1];
          v2 = *(unsigned char **)((long)stdout + 0x28);
          if (*(unsigned char **)((long)stdout + 0x30) <= v2) { // branch-flip
            __overflow(stdout,(unsigned int)v1);
            v11 = dat_1710c;
          }
          else {
            *(unsigned char **)((long)stdout + 0x28) = &v2[1];
            *v2 = v1;
          }
          dat_17108 += 1;
          v5 = dat_17108;
        } while (dat_17108 != v11);
      }
      if (v14) {
        sub_3060(v11 - v5);
label_3955:
        if (v4 != 0xffffffff) goto label_3981;
label_3822:
        if ((int)(dat_17114 + dat_c0e4) <= dat_1710c) {
          v3 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
            __overflow(stdout,10);
          else {
            *(char **)((long)stdout + 0x28) = &v3[1];
            *v3 = 10;
          }
        }
        goto label_383a;
      }
      if (v4 == 0xffffffff) goto label_3822;
    }
    else {
      dat_17108 = 0;
      v14 = v4 != 10 && v4 != 0xffffffff;
      if ((int)dat_c0e4 < dat_1710c) goto label_379e;
      if (v14) {
        sub_3060(dat_c0e4);
        v11 = dat_1710c;
        if (dat_17108 != dat_1710c) goto label_393a;
        sub_3060(0);
        goto label_3955;
      }
label_38db:
      if (v4 == 0xffffffff) {
label_383a:
        dat_c0e8 = 0xffffffff;
        if (*a0 & 0x20) { // branch-flip
          if (a0 != stdin) // branch-flip
            sub_41f0(a0);
          else {
            clearerr_unlocked(a0);
          }
          v7 = sub_63b0(0,3,a1);
        }
        else {
          if (a0 == stdin) {
            clearerr_unlocked(a0);
            return 1;
          }
          if (!sub_41f0(a0))
            return 1;
          v11 = *__errno_location();
          if (v11 < 0)
            return 1;
          v7 = sub_63b0(0,3,a1);
          if (v11) {
            v8 = "%s";
            error(0,v11,v8,v7); // return-dupe
            return 0;
          }
        }
        v11 = 0;
        v8 = dcgettext(NULL,"read error",5);
        error(0,v11,v8,v7);
        return 0;
      }
    }
    v3 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
      __overflow(stdout,10);
    else {
      *(char **)((long)stdout + 0x28) = &v3[1];
      *v3 = 10;
    }
    v4 = sub_2f30(a0);
  } while( true );
label_3a07:
  dat_c0f4 = dat_c0e4;
  dat_c0f0 = dat_1710c;
  dat_15d60 = 0x15d80;
  dat_c100 = 0xc120;
  v4 = sub_3420(a0,v4);
  v10 = (unsigned long)dat_c0e4;
  v14 = 0;
  v12 = (unsigned long)v4;
  if ((dat_c0e4 == dat_c0f4) && ((int)(dat_1711c + dat_c0e4) <= dat_1710c))
    v14 = v4 != 0xffffffff && v4 != 10;
  v4 = dat_c0f4;
  sub_3390(v14);
  v6 = (unsigned int)v12;
  if (!dat_17131) {
    v9 = (unsigned int)v10;
    if (dat_17133) { // branch-flip
      if (((v9 == v4) && ((int)(v9 + dat_1711c) <= dat_1710c)) && ((v6 != 10 && (v6 != 0xffffffff)))) {
        while (((v6 = sub_3420(a0), dat_c0e4 == dat_c0f4 && ((int)(dat_c0e4 + dat_1711c) <= dat_1710c)) && (v6 != 10))) {
          if ((v6 == 0xffffffff) || (dat_c0ec != dat_1710c)) break;
        }
      }
    }
    else if (dat_17132) { // branch-flip
      if (((v9 == v4) && ((int)(v9 + dat_1711c) <= dat_1710c)) && ((v6 != 10 && ((v6 != 0xffffffff && (dat_c0f0 != dat_1710c)))))) {
        while (v6 = sub_3420(a0), dat_c0e4 == dat_c0f4) {
          if ((((dat_1710c < (int)(dat_c0e4 + dat_1711c)) || (v6 == 10)) || (v6 == 0xffffffff)) || (dat_c0ec != dat_1710c)) break;
        }
      }
    }
    else if (v9 == v4) {
      do {
        v6 = (unsigned int)v12;
        if ((((dat_1710c < (int)v10 + dat_1711c) || (v6 == 10)) || (v6 == 0xffffffff)) || (dat_c0ec != dat_1710c)) break;
        v6 = sub_3420(a0);
        v10 = (unsigned long)dat_c0e4;
        v12 = (unsigned long)v6;
      } while (dat_c0e4 == dat_c0f4);
    }
  }
  v10 = dat_c100;
  if (dat_c100 <= 0xc120)
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph"); // no-return
  *(unsigned char *)(dat_c100 - 0x18) = *(unsigned char *)(dat_c100 - 0x18) | 10;
  dat_c0e8 = v6;
  sub_2cb0();
  sub_3140(0xc120,dat_c0f0);
  v4 = dat_c0e8;
  v12 = dat_c140;
  while (dat_c0e8 = v4, v12 != v10) {
    sub_3140(v12,dat_c0ec);
    v12 = *(unsigned long *)(v12 + 0x20);
    v4 = dat_c0e8;
  }
  goto label_3746;
}


// Function: sub_3d70 @ 0x3d70
void sub_3d70(int a0)
{
  void *v1;
  char *v10; // stack - 0xb0
  char *v11; // stack - 0x88
  char *v12; // stack - 0x80
  char *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x50
  unsigned long v19; // stack - 0x40
  unsigned long v2;
  char *v20 [4]; // stack - 0xa8
  int v3; // eax
  char *v4; // rax
  char **v5; // rbx
  char *v6; // stack - 0xb8
  char *v7;
  char *v8; // r13
  long v9; // fs_offset
  
  v2 = dat_17148;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v6 = "[";
    v7 = "[";
    v10 = "test invocation";
    v20[0] = "coreutils";
    v20[1] = "Multi-call invocation";
    v11 = "sha256sum";
    v20[2] = "sha224sum";
    v13 = "sha384sum";
    v20[3] = "sha2 utilities";
    v12 = "sha2 utilities";
    v14 = "sha2 utilities";
    v15 = "sha512sum";
    v16 = "sha2 utilities";
    v17 = 0;
    v18 = 0;
    do {
      if (!strcmp("fmt",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "fmt";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "fmt";
    if (!strcmp("fmt","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "fmt")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}

