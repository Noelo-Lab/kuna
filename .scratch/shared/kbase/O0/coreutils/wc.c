// Function: sub_2daf @ 0x2daf
unsigned long sub_2daf(void) // return-dupe x2, return-dupe x3
{
  bool v1;
  bool v2;
  unsigned int v3 [2]; // stack - 0x18
  unsigned int v4; // stack - 0x1c
  unsigned int v5; // stack - 0x20
  
  v5 = 0;
  v4 = 0;
  v3[0] = 0;
  v3[1] = 0;
  v2 = 0;
  v1 = 0;
  if ((sub_2cb7(1,&v5,&v4,v3,&v3[1])) && (v2 = 1, v3[0] & 0x8000000))
    v1 = 1;
  if (v1) {
    v3[1] = 0;
    v3[0] = 0;
    v4 = 0;
    v5 = 0;
    if (sub_2d2f(7,0,&v5,&v4,v3,&v3[1])) { // branch-flip
      if (!(v4 & 0x20))
        v1 = 0;
    }
    else {
      v2 = 0;
    }
  }
  if (!v2) {
    if (!dat_f110)
      return 0;
    error(0,0,"%s",gettext("failed to get cpuid"));
    return 0;
  }
  if (!v1) {
    if (!dat_f110)
      return 0;
    error(0,0,"%s",gettext("avx2 support not detected"));
    return 0;
  }
  if (!dat_f110)
    return 1;
  error(0,0,"%s",gettext("using avx2 hardware support"));
  return 1;
}


// Function: sub_2f7b @ 0x2f7b
void sub_2f7b(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_f178;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n"),v1);
    sub_2978();
    v1 = stdout;
    fputs_unlocked(gettext("\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_29a9("wc");
  }
  exit(a0); // no-return
}


// Function: sub_30be @ 0x30be
unsigned long sub_30be(int a0) // early-return
{
  if ((dat_f158 != '\x01') && ((((a0 == 0xa0 || (a0 == 0x2007)) || (a0 == 0x202f)) || (a0 == 0x2060))))
    return 1;
  return 0;
}


// Function: sub_3109 @ 0x3109
void sub_3109(unsigned int a0)
{
  sub_30be(btowc(a0));
}


// Function: sub_312b @ 0x312b
void sub_312b(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  unsigned long v1; // rax
  char v2 [24];
  char *v3; // stack - 0x30
  
  v3 = "%*s";
  if (dat_f140) {
    v1 = sub_5729(a0,v2);
    printf("%*s",(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f141) {
    v1 = sub_5729(a1,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f142) {
    v1 = sub_5729(a2,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f143) {
    v1 = sub_5729(a3,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f144) {
    v1 = sub_5729(a4,v2);
    printf(v3,(unsigned long)dat_f148,v1);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_79a7(0,3,a5);
    printf(" %s",a5);
  }
  putchar_unlocked(10);
}


// Function: sub_3328 @ 0x3328
unsigned long sub_3328(unsigned long a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  long v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  char v6 [16392];
  long v7; // stack - 0x4058
  long v8; // stack - 0x4050
  char *v9; // stack - 0x4048
  
  v1 = 0;
  if ((a2) && (a3)) {
    v8 = 0;
    v7 = 0;
    while( true ) {
      v2 = v7;
      v5 = sub_8412(a1,v6,0x4000);
      if (!v5) {
        *a3 = v8;
        *a2 = v7;
        return 1;
      }
      if (v5 == 0xffffffffffffffff) break;
      v8 += v5;
      v9 = v6;
      v4 = &v6[v5];
      if (v1) {
        *v4 = '\n';
        while (v9 = rawmemchr(v9,10), v9 < v4) {
          v9 = &v9[1];
          v7 += 1;
        }
      }
      else {
        for (; v9 != v4; v9 = &v9[1]) {
          v7 += (int)(unsigned int)(*v9 == '\n');
        }
      }
      if ((unsigned long)(v7 - v2) <= v5 / 0xf) // branch-flip
        v1 = 1;
      else {
        v1 = 0;
      }
    }
    v3 = sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v3);
    return 0;
  }
  return 0;
}


// Function: sub_3591 @ 0x3591
unsigned char sub_3591(int a0,char *a1,int *a2,unsigned long a3) // ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_3591
{
  char *v1;
  char v10 [16392];
  long v11; // stack - 0x40a8
  long v12; // stack - 0x40b0
  unsigned long v13; // stack - 0x4038
  unsigned int v14; // stack - 0x40b8
  bool v15;
  unsigned long v16; // stack - 0x40e8
  char v17; // stack - 0x40c1
  char v18; // stack - 0x40c0
  unsigned char v19; // stack - 0x40bf
  char v2;
  unsigned char v20; // stack - 0x40bd
  unsigned char v21; // stack - 0x40ba
  int v22; // stack - 0x40b4
  unsigned long v23; // stack - 0x40a0
  long v24; // stack - 0x4098
  long v25; // stack - 0x4090
  unsigned long v26; // stack - 0x4088
  unsigned long v27; // stack - 0x4080
  unsigned long v28; // stack - 0x4078
  char *v29; // stack - 0x4070
  long v3;
  long v30; // stack - 0x4068
  unsigned long v31; // stack - 0x4060
  char *v32; // stack - 0x4058
  char *v33; // stack - 0x4050
  unsigned long v34; // stack - 0x4048
  long v35; // stack - 0x4040
  unsigned long v36; // stack - 0x4030
  bool v4;
  unsigned long v5;
  unsigned long v6;
  unsigned char v7; // al
  unsigned int v8; // eax
  unsigned short *v9;
  
  v17 = 1;
  if (a1) // branch-flip
    v9 = (unsigned short *)a1;
  else {
    v9 = (char *)gettext("standard input");
  }
  v26 = 0;
  v11 = 0;
  v25 = 0;
  v24 = 0;
  v12 = 0;
  v33 = (char *)v9;
  v9 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v9) { // branch-flip
    v7 = dat_f143;
    v18 = v7;
    v7 = dat_f142;
    v19 = v7;
  }
  else {
    v7 = ((dat_f143) || (dat_f142));
    v19 = 0;
    v18 = v7;
  }
  v7 = ((dat_f141) || (dat_f144));
  if ((((v18 != '\x01') || (v19)) || (dat_f140)) || ((bool)v7))
    sub_54b7(a0,0,0,2);
  if (((!v18) || (v19 == 1)) || ((dat_f140 == '\x01' || ((bool)v7)))) {
    if ((v19 == 1) || ((bool)v7)) {
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      if (2 <= v9) { // branch-flip
        v20 = 0;
        v27 = 0;
        v13 = 0;
        v15 = 0;
        v28 = 0;
        while( true ) {
          v9 = (long)(0x4000 - v28);
          v9 = (unsigned long)sub_8412(a0,&v10[v28],v9);
          v6 = (unsigned long)v9;
          v23 = (unsigned long)v9;
          if (!v9) break;
          if (v9 == (unsigned short *)0xffffffffffffffff) {
            v9 = (unsigned long)sub_79a7(0,3,v33);
            v5 = v9;
            v9 = (int *)__errno_location();
            error(0,*(int *)v9,"%s",v5);
            v17 = 0;
            break;
          }
          v9 = (long)((long)v9 + v11);
          v29 = v10;
          v23 = v6 + v28;
          v11 = (long)v9;
          do {
            v4 = 1;
            if ((v15) || (v8 = (int)*v29, v7 = sub_5893(v8), !v7)) {
              v15 = 1;
              v36 = v13;
              v9 = (long)sub_58d8(&v14,v29,v23,&v13);
              v30 = (long)v9;
              if (v9 != (unsigned short *)0xfffffffffffffffe) {
                if (v9 != (unsigned short *)0xffffffffffffffff) {
                  v8 = mbsinit(&v13);
                  v15 = v8 == 0;
                  if (!v30) {
                    v14 = 0;
                    v30 = 1;
                  }
                  goto label_3c3e;
                }
                v29 = &v29[1];
                v23 -= 1;
                goto label_3e20;
              }
              v13 = v36;
              break;
            }
            v30 = 1;
            v8 = (unsigned int)*v29;
            v4 = 0;
            v14 = v8;
label_3c3e:
            switch(v14) {
              case 9:
                v9 = (unsigned long)(v27 & 0xfffffffffffffff8);
                v9 = (unsigned long)((long)v9 + 8);
                v27 = (unsigned long)v9;
                break;
              case 10:
                v9 = (long)(v12 + 1);
                v12 = (long)v9;
                goto label_3c85;
              case 0xb:
                break;
              case 0xc:
              case 0xd:
label_3c85:
                if (v26 < v27)
                  v26 = v27;
                v27 = 0;
                break;
              default:
                if ((v4) && (v8 = iswprint(v14), v8)) {
                  if (dat_f144) {
                    v8 = wcwidth(v14);
                    v22 = v8;
                    if (1 <= (int)v8) {
                      v9 = (long)(long)(int)v8;
                      v27 += (long)v9;
                    }
                  }
                  v8 = iswspace(v14);
                  if ((v8) || (v8 = sub_30be(v14), v8)) {
                    goto label_3cd6;
                  }
                  v20 = 1;
                }
                else if (!v4) {
                  v9 = __ctype_b_loc();
                  v3 = *(long *)v9;
                  v8 = (int)*v29;
                  v7 = sub_2969(v8);
                  v9 = (unsigned long)(unsigned long)v7;
                  v9 = (unsigned short *)((long)v9 * 2 + v3);
                  if (*v9 & 0x4000) {
                    v27 += 1;
                    v9 = __ctype_b_loc();
                    v3 = *(long *)v9;
                    v8 = (int)*v29;
                    v7 = sub_2969(v8);
                    v9 = (unsigned long)(unsigned long)v7;
                    v9 = (unsigned short *)((long)v9 * 2 + v3);
                    if (*v9 & 0x2000) goto label_3cd6;
                    v20 = 1;
                  }
                }
                goto label_3dfc;
              case 0x20:
                v27 += 1;
              
            }
label_3cd6:
            v9 = (unsigned long)(unsigned long)v20;
            v24 += (long)v9;
            v20 = 0;
label_3dfc:
            v29 = &v29[v30];
            v23 -= v30;
            v25 += 1;
label_3e20:
          } while (v23);
          if (v23) {
            if (v23 == 0x4000) {
              v29 = &v29[1];
              v23 = 0x3fff;
            }
            memmove(v10,v29,v23);
          }
          v28 = v23;
        }
        if (v26 < v27)
          v26 = v27;
        v9 = (unsigned long)(unsigned long)v20;
        v24 += (long)v9;
      }
      else {
        v21 = 0;
        v31 = 0;
        while( true ) {
          v9 = (unsigned long)sub_8412(a0,v10,0x4000);
          v23 = (unsigned long)v9;
          if (!v9) break;
          v32 = v10;
          if (v9 == (unsigned short *)0xffffffffffffffff) {
            v9 = (unsigned long)sub_79a7(0,3,v33);
            v5 = v9;
            v9 = (int *)__errno_location();
            error(0,*(int *)v9,"%s",v5);
            v17 = 0;
            break;
          }
          v9 = (long)((long)v9 + v11);
          v11 = (long)v9;
          do {
            v1 = &v32[1];
            v2 = *v32;
            v32 = v1;
            switch(v2) {
              case 9:
                v9 = (unsigned long)(v31 & 0xfffffffffffffff8);
                v9 = (unsigned long)((long)v9 + 8);
                v31 = (unsigned long)v9;
                break;
              case 10:
                v9 = (long)(v12 + 1);
                v12 = (long)v9;
                goto label_3fe3;
              case 0xb:
                break;
              case 0xc:
              case 0xd:
label_3fe3:
                if (v26 < v31)
                  v26 = v31;
                v31 = 0;
                break;
              default:
                v9 = __ctype_b_loc();
                v3 = *(long *)v9;
                v9 = (char *)&v32[-1];
                v8 = (int)*(char *)v9;
                v7 = sub_2969(v8);
                v9 = (unsigned long)(unsigned long)v7;
                v9 = (unsigned short *)((long)v9 * 2 + v3);
                if (*v9 & 0x4000) {
                  v31 += 1;
                  v9 = __ctype_b_loc();
                  v3 = *(long *)v9;
                  v9 = (char *)&v32[-1];
                  v8 = (int)*(char *)v9;
                  v7 = sub_2969(v8);
                  v9 = (unsigned long)(unsigned long)v7;
                  v9 = (unsigned short *)((long)v9 * 2 + v3);
                  if (!(*v9 & 0x2000)) {
                    v9 = (char *)&v32[-1];
                    v8 = (int)*(char *)v9;
                    v7 = sub_2969(v8);
                    v8 = sub_3109(v7);
                    if (!v8) {
                      v21 = 1;
                      goto label_40f9;
                    }
                  }
                  v9 = (unsigned long)(unsigned long)v21; // crossjump-dupe
                  v24 += (long)v9;
                  v21 = 0;
                }
                goto label_40f9;
              case 0x20:
                v31 += 1;
              
            }
            v9 = (unsigned long)(unsigned long)v21;
            v24 += (long)v9;
            v21 = 0;
label_40f9:
            v23 -= 1;
          } while (v23);
        }
        if (v26 < v31)
          v26 = v31;
        v9 = (unsigned long)(unsigned long)v21;
        v24 += (long)v9;
      }
    }
    else {
      v7 = sub_2daf();
      if (v7)
        dat_f010 = sub_a1b1;
      v7 = (*dat_f010)(v33,a0,&v12,&v11);
      v17 = v7;
    }
  }
  else {
    v15 = 0;
    if (1 <= *a2) {
      v8 = fstat(a0,&a2[2]);
      *a2 = v8;
    }
    if (((!*a2) && (v9 = (int *)&a2[2], v7 = sub_2c40(v9), v7)) && (0 <= *(long *)&a2[0xe])) {
      v34 = *(unsigned long *)&a2[0xe];
      v16 = a3;
      if ((long)a3 <= -1) {
        v9 = (unsigned long)lseek(a0,0,1);
        v16 = (unsigned long)v9;
      }
      if (v34 % dat_f150) { // branch-flip
        v9 = (v16 <= v34) ? (long)(v34 - v16) : NULL; // branch-flip
        v15 = 1;
        v11 = (long)v9;
      }
      else {
        if ((1 <= *(long *)&a2[0x10]) && (*(unsigned long *)&a2[0x10] <= 0x2000000000000000))
          v9 = (unsigned long)(*(long *)&a2[0x10] + 1);
        else {
          v9 = (unsigned short *)0x201;
        }
        v9 = (long)(v34 - v34 % (unsigned long)v9);
        v35 = (long)v9;
        if (((0 <= (long)v16) && ((long)v16 < (long)v9)) && (v9 = (long)lseek(a0,v9,1), 0 <= (long)v9)) {
          v9 = (long)(v35 - v16);
          v11 = (long)v9;
        }
      }
    }
    if (!v15) {
      sub_54b7(a0,0,0,2);
      while( true ) {
        v9 = (unsigned long)sub_8412(a0,v10,0x4000);
        v23 = (unsigned long)v9;
        if (!v9) break;
        if (v9 == (unsigned short *)0xffffffffffffffff) {
          v9 = (unsigned long)sub_79a7(0,3,v33);
          v5 = v9;
          v9 = (int *)__errno_location();
          error(0,*(int *)v9,"%s",v5);
          v17 = 0;
          break;
        }
        v9 = (long)((long)v9 + v11);
        v11 = (long)v9;
      }
    }
  }
  if (v19 < dat_f142)
    v25 = v11;
  sub_312b(v12,v24,v25,v11,v26,a1);
  v9 = (long)(v12 + dat_f118);
  dat_f118 = (long)v9;
  v9 = (long)(v24 + dat_f120);
  dat_f120 = (long)v9;
  v9 = (long)(v25 + dat_f128);
  dat_f128 = (long)v9;
  v9 = (long)(v11 + dat_f130);
  dat_f130 = (long)v9;
  if (dat_f138 < v26)
    dat_f138 = v26;
  v7 = v17;
  return v7;
}


// Function: sub_4263 @ 0x4263
unsigned long sub_4263(char *a0,unsigned long a1)
{
  unsigned char v1; // al
  int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  if ((!a0) || (!strcmp(a0,"-"))) {
    dat_f14c = 1;
    sub_945e(0,0);
    v3 = sub_3591(0,a0,a1,0xffffffffffffffff);
    return v3;
  }
  v2 = open(a0,0);
  if (v2 == -1) {
    v4 = sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v4);
    return 0;
  }
  v1 = sub_3591(v2,a0,a1,0);
  if (close(v2)) {
    v4 = sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v4);
    return 0;
  }
  return (unsigned long)v1;
}


// Function: sub_43a5 @ 0x43a5
unsigned int * sub_43a5(unsigned long a0,long a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3;
  unsigned int *v4; // rax
  unsigned long v5; // stack - 0x18
  
  if (a0) // branch-flip
    v3 = a0;
  else {
    v3 = 1;
  }
  v4 = (unsigned int *)sub_8f54(v3,0x98);
  if ((a0) && ((a0 != 1 || ((unsigned int)dat_f144 + (unsigned int)dat_f140 + (unsigned int)dat_f141 + (unsigned int)dat_f142 + (unsigned int)dat_f143 != 1)))) {
    for (v5 = 0; v5 < a0; v5 = v5 + 1) {
      if (*(long *)(a1 + v5 * 8)) { // branch-flip
        v1 = *(char **)(a1 + v5 * 8);
        if (!strcmp(v1,"-")) goto label_448c;
        v2 = stat(*(char **)(a1 + v5 * 8),&v4[v5 * 0x26 + 2]);
      }
      else {
label_448c:
        v2 = fstat(0,&v4[v5 * 0x26 + 2]);
      }
      v4[v5 * 0x26] = v2;
    }
  }
  else {
    *v4 = 1;
  }
  return v4;
}


// Function: sub_4549 @ 0x4549
int sub_4549(unsigned long a0,int *a1)
{
  int v1; // stack - 0x20
  int v2; // stack - 0x1c
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v1 = 1;
  if ((a0) && (*a1 <= 0)) {
    v2 = 1;
    v3 = 0;
    for (v4 = 0; v4 < a0; v4 = v4 + 1) {
      if (!a1[v4 * 0x26]) {
        if ((a1[v4 * 0x26 + 8] & 0xf000U) != 0x8000) // branch-flip
          v2 = 7;
        else {
          v3 += *(long *)&a1[v4 * 0x26 + 0xe];
        }
      }
    }
    for (; 10 <= v3; v3 = v3 / 10) {
      v1 += 1;
    }
    if (v1 < v2)
      v1 = v2;
  }
  return v1;
}


// Function: sub_4670 @ 0x4670
unsigned long sub_4670(int a0,char **a1)
{
  bool v1;
  long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char v13 [24];
  long v14; // stack - 0x1d8
  unsigned int v15; // stack - 0x224
  long v16; // fs_offset
  double v17; // xmm0_qa
  double v18; // xmm0_qa
  double v19;
  bool v2;
  bool v20; // stack - 0x227
  int v21; // stack - 0x220
  int v22; // stack - 0x21c
  long v23; // stack - 0x218
  char **v24; // stack - 0x210
  char *v25; // stack - 0x208
  long v26; // stack - 0x200
  void *v27; // stack - 0x1f8
  unsigned int *v28; // stack - 0x1f0
  char *v29; // stack - 0x1e8
  char v3; // al
  unsigned long v30; // stack - 0x1e0
  char **v31; // stack - 0x1d0
  unsigned int v32; // stack - 0xa0
  long v33; // stack - 0x88
  long v34; // stack - 0x20
  unsigned char v4; // al
  int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  char *v9; // rax
  
  v34 = *(long *)(v16 + 0x28);
  v25 = NULL;
  sub_5c1b(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_ac00(sub_53bf);
  dat_f150 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  dat_f158 = getenv("POSIXLY_CORRECT") != NULL;
  dat_f143 = '\0';
  dat_f142 = '\0';
  dat_f141 = '\0';
  dat_f140 = '\0';
  dat_f144 = '\0';
  dat_f138 = 0;
  dat_f130 = 0;
  dat_f128 = 0;
  dat_f120 = 0;
  dat_f118 = 0;
  while( true ) {
    v22 = getopt_long(a0,a1,"clLmw",(void *)0xea20,NULL);
    if (v22 == -1) break;
    if (0x81 < v22)
      sub_2f7b(1); // no-return, return-dupe
    if (v22 <= 0x4b) {
      if (v22 == -0x83) {
        sub_8bfb(stdout,"wc","GNU coreutils",dat_f018,"Paul Rubin","David MacKenzie",0);
        exit(0); // no-return
      }
      if (v22 == -0x82)
        sub_2f7b(0); // no-return
      sub_2f7b(1);
    }
    switch(v22) {
      case 0x4c:
        dat_f144 = '\x01';
        break;
      default:
        goto label_48e7;
      case 99:
        dat_f143 = '\x01';
        break;
      case 0x6c:
        dat_f140 = '\x01';
        break;
      case 0x6d:
        dat_f142 = '\x01';
        break;
      case 0x77:
        dat_f141 = '\x01';
        break;
      case 0x80:
        dat_f110 = 1;
        break;
      case 0x81:
        v25 = optarg;
      
    }
  }
  if ((((dat_f140 != '\x01') && (dat_f141 != '\x01')) && (dat_f142 != '\x01')) && ((dat_f143 != '\x01' && (dat_f144 != '\x01')))) {
    dat_f143 = '\x01';
    dat_f141 = '\x01';
    dat_f140 = '\x01';
  }
  v2 = 0;
  if (v25) { // branch-flip
    if (optind < a0) {
      v6 = sub_7815(4,a1[optind]);
      error(0,0,gettext("extra operand %s"),v6);
      fprintf(stderr,"%s\n",gettext("file operands cannot be combined with --files0-from"));
      sub_2f7b(1); // no-return
    }
    v5 = strcmp(v25,"-");
    if (v5) { // branch-flip
      v27 = fopen(v25,"r");
      if (!v27) {
        v6 = sub_7815(4,v25);
        v9 = gettext("cannot open %s for reading");
        error(1,*__errno_location(),v9,v6);
        return v7;
      }
    }
    else {
      v27 = stdin;
    }
    v5 = fileno(v27);
    if ((!fstat(v5,v13)) && ((v32 & 0xf000) == 0x8000)) {
      sub_5aa9();
      if (v17 / dat_b988 <= dat_b990) {
        sub_5aa9();
        v19 = v18 / dat_b988;
      }
      else {
        v19 = dat_b990;
      }
      if ((double)v33 <= v19) {
        v2 = 1;
        sub_7c7f(&v14);
        v3 = sub_809e(v27,&v14);
        if ((v3 != '\x01') || (v5 = sub_552b(v27), v5)) {
          v6 = sub_7815(4,v25);
          error(1,0,gettext("cannot read file names from %s"),v6);
          return v8;
        }
        v24 = v31;
        v23 = v14;
        v26 = sub_5165(v31);
        goto label_4d00;
      }
    }
    v24 = NULL;
    v23 = 0;
    v26 = sub_51ba(v27);
  }
  else {
    if (a0 <= optind) { // branch-flip
      v24 = (char **)0xf160;
      v23 = 1;
    }
    else {
      v24 = &a1[optind];
      v23 = (long)(a0 - optind);
    }
    v26 = sub_5165(v24);
  }
label_4d00:
  if (!v26)
    sub_9413(); // no-return
  v28 = (unsigned int *)sub_43a5(v23,v24);
  dat_f148 = sub_4549(v23,v28);
  v20 = 1;
  v21 = 0;
  while( true ) {
    v1 = 0;
    v29 = (char *)sub_5227(v26,&v15);
    if (!v29) break;
    if (((v25) && (v5 = strcmp(v25,"-"), !v5)) && (v5 = strcmp(v29,"-"), !v5)) {
      v6 = sub_7815(4,v29);
      error(0,0,gettext("when reading file names from stdin, no file name of %s allowed"),v6);
      v1 = 1;
    }
    if (!*v29) {
      if (v25) { // branch-flip
        v30 = sub_5312(v26);
        v9 = gettext("invalid zero-length file name");
        v6 = sub_79a7(0,3,v25);
        error(0,0,"%s:%lu: %s",v6,v30,v9);
      }
      else {
        error(0,0,"%s",gettext("invalid zero-length file name"));
      }
      v1 = 1;
    }
    if (v1)
      v20 = 0;
    else {
      if (v23) // branch-flip
        v10 = (long)v21 * 0x98;
      else {
        v10 = 0;
      }
      v4 = sub_4263(v29,v10 + (long)v28);
      v20 = (v4 & v20) != 0;
    }
    if (!v23)
      *v28 = 1;
    v21 += 1;
  }
  if (v15 == 4) {
    v6 = sub_79a7(0,3,v25);
    v9 = gettext("%s: read error");
    error(0,*__errno_location(),v9,v6);
    v20 = 0;
label_5026:
    if (((v20) && (!v25)) && (v10 = sub_5312(v26), !v10)) {
      v4 = sub_4263(0,v28);
      v20 = (v4 & v20) != 0;
    }
    if (v2)
      sub_7d41(&v14);
    v11 = sub_5312(v26);
    if (2 <= v11)
      sub_312b(dat_f118,dat_f120,dat_f128,dat_f130,dat_f138,gettext("total"));
    sub_5350(v26);
    free(v28);
    if ((dat_f14c) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v12;
    }
    if (v34 == *(long *)(v16 + 0x28))
      return (unsigned long)(v20 ^ 1);
    __stack_chk_fail(); // no-return
  }
  if (v15 <= 4) {
    if (v15 == 2) goto label_5026;
    if (v15 == 3)
      sub_9413(); // no-return
  }
  __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
}


// Function: sub_a1b1 @ 0xa1b1
unsigned long sub_a1b1(unsigned long a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  char v1 [16];
  char v10 [32];
  char v11 [32];
  unsigned long v12; // rax
  char (*v13)[32]; // rax
  char v14 [16344];
  char v15 [32]; // ymm0
  char (*v16)[32]; // stack - 0x4360
  long v17; // stack - 0x4358
  long v18; // stack - 0x4350
  unsigned long v19; // stack - 0x4348
  char v2 [16];
  char (*v20)[32]; // stack - 0x4340
  char v21 [32]; // stack - 0x42a0
  char v22 [608]; // stack - 0x4280
  char v3 [16];
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  v17 = 0;
  v18 = 0;
  if ((a2) && (a3)) {
    v1 = vpinsrb_avx(ZEXT116(10),10,1);
    v2 = vpinsrb_avx(ZEXT116(10),10,1);
    v3 = vpinsrb_avx(ZEXT116(10),10,1);
    v4 = vpinsrb_avx(ZEXT116(10),10,1);
    v5 = vpinsrb_avx(ZEXT116(10),10,1);
    v6 = vpinsrb_avx(ZEXT116(10),10,1);
    v7 = vpinsrb_avx(ZEXT116(10),10,1);
    v8 = vpinsrb_avx(ZEXT116(10),10,1);
    v1 = vpunpcklwd_avx(v1,v2);
    v2 = vpunpcklwd_avx(v3,v4);
    v3 = vpunpcklwd_avx(v5,v6);
    v4 = vpunpcklwd_avx(v7,v8);
    v1 = vpunpckldq_avx(v1,v2);
    v9 = vpunpcklqdq_avx(v1,vpunpckldq_avx(v3,v4));
    v1 = vpinsrb_avx(ZEXT116(10),10,1);
    v2 = vpinsrb_avx(ZEXT116(10),10,1);
    v3 = vpinsrb_avx(ZEXT116(10),10,1);
    v4 = vpinsrb_avx(ZEXT116(10),10,1);
    v5 = vpinsrb_avx(ZEXT116(10),10,1);
    v6 = vpinsrb_avx(ZEXT116(10),10,1);
    v7 = vpinsrb_avx(ZEXT116(10),10,1);
    v8 = vpinsrb_avx(ZEXT116(10),10,1);
    v1 = vpunpcklwd_avx(v1,v2);
    v2 = vpunpcklwd_avx(v3,v4);
    v3 = vpunpcklwd_avx(v5,v6);
    v4 = vpunpcklwd_avx(v7,v8);
    v1 = vpunpckldq_avx(v1,v2);
    v1 = vpunpcklqdq_avx(v1,vpunpckldq_avx(v3,v4));
    v15._0_16_ = ZEXT116(0) * v1 + ZEXT116(1) * v9;
    v15._16_16_ = ZEXT116(1) * v1;
    while( true ) {
      v22._0_32_ = ZEXT1632(ZEXT816(0));
      v21._0_32_ = ZEXT1632(ZEXT816(0));
      v19 = sub_8412(a1,v14,0x3fc0);
      if (!v19) {
        *a2 = v17;
        *a3 = v18;
        return 1;
      }
      if (v19 == 0xffffffffffffffff) break;
      v18 += v19;
      v16 = (char (*)[32])v14;
      v13 = (char (*)[32])&v14[v19];
      for (; 0x40 <= v19; v19 = v19 - 0x40) {
        v10 = vpcmpeqb_avx2(*v16,v15._0_32_);
        v11 = vpcmpeqb_avx2(v16[1],v15._0_32_);
        v21._0_32_ = vpsubb_avx2(v21._0_32_,v10);
        v22._0_32_ = vpsubb_avx2(v22._0_32_,v11);
        v16 = &v16[2];
      }
      v10 = vpsadbw_avx2(v21._0_32_,ZEXT1632(ZEXT816(0)));
      v21._16_16_ = SUB3216(v10,0x10);
      v11 = vpsadbw_avx2(v22._0_32_,ZEXT1632(ZEXT816(0)));
      v22._16_16_ = SUB3216(v11,0x10);
      v17 = v17 + (int)((vpextrw_avx(v21._16_16_,4) & 0xffff) + (vpextrw_avx(SUB3216(v10,0),0) & 0xffff) + (vpextrw_avx(SUB3216(v10,0),4) & 0xffff) + (vpextrw_avx(v21._16_16_,0) & 0xffff)) + (long)(int)((vpextrw_avx(v22._16_16_,4) & 0xffff) + (vpextrw_avx(SUB3216(v11,0),0) & 0xffff) + (vpextrw_avx(SUB3216(v11,0),4) & 0xffff) + (vpextrw_avx(v22._16_16_,0) & 0xffff));
      for (v20 = v16; v20 != v13; v20 = (char (*)[32])&(*v20)[1]) {
        v17 += (int)(unsigned int)((*v20)[0] == '\n');
      }
    }
    v12 = sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v12);
    return 0;
  }
  return 0;
}

