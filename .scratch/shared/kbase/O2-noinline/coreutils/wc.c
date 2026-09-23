// Function: sub_28a0 @ 0x28a0
unsigned long sub_28a0(int a0,char **a1)
{
  unsigned char v1; // al
  char *v10;
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned int *v13;
  char v14 [24];
  int v15; // stack - 0x1fc
  long v16; // stack - 0x1f8
  unsigned int *v17;
  long v18; // fs_offset
  double v19;
  unsigned char v2;
  long v20; // stack - 0x228
  char v21; // stack - 0x209
  char **v22; // stack - 0x1f0
  unsigned int v23; // stack - 0xc0
  long v24; // stack - 0xa8
  long v25; // stack - 0x40
  int v3; // eax
  long v4; // rax
  unsigned int *v5; // rax
  char *v6; // rax
  void *v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v25 = *(long *)(v18 + 0x28);
  sub_4ea0(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8cd0(sub_4870);
  dat_e120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  dat_e131 = '\0';
  dat_e132 = '\0';
  dat_e118 = getenv("POSIXLY_CORRECT") != NULL;
  dat_e133 = '\0';
  dat_e134 = '\0';
  dat_e130 = '\0';
  dat_e138 = 0;
  dat_e140 = 0;
  dat_e148 = 0;
  dat_e150 = 0;
  dat_e158 = 0;
  v10 = NULL;
  while( true ) {
    v3 = getopt_long(a0,a1,"clLmw",(void *)0xda20,NULL);
    if (v3 == -1) break;
    if (0x81 < v3) {
      sub_45c0(1); // return-dupe, no-return
    }
    if (v3 <= 0x4b) {
      if (v3 == -0x83) {
        sub_7d20(stdout,"wc","GNU coreutils",dat_e018,"Paul Rubin","David MacKenzie",0,CONCAT44(dat_4,v3));
        exit(0); // no-return
      }
      if (v3 == -0x82)
        sub_45c0(0); // no-return
      sub_45c0(1);
    }
    switch(v3) {
      case 0x4c:
        dat_e130 = '\x01';
        break;
      default:
        goto label_3048;
      case 99:
        dat_e131 = '\x01';
        break;
      case 0x6c:
        dat_e134 = '\x01';
        break;
      case 0x6d:
        dat_e132 = '\x01';
        break;
      case 0x77:
        dat_e133 = '\x01';
        break;
      case 0x80:
        dat_e160 = 1;
        break;
      case 0x81:
        v10 = optarg;
      
    }
  }
  if ((((!dat_e134) && (!dat_e133)) && (!dat_e132)) && ((!dat_e131 && (!dat_e130)))) {
    dat_e131 = '\x01';
    dat_e133 = '\x01';
    dat_e134 = '\x01';
  }
  if (v10) { // branch-flip
    if (optind < a0) {
      v9 = sub_7110(4,a1[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v9);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_45c0(1);
    }
    if (strcmp(v10,"-")) { // branch-flip
      v7 = fopen(v10,"r");
      if (!v7) {
        v9 = sub_7110(4,v10);
        v10 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v10,v9);
        return v8;
      }
    }
    else {
      v7 = stdin;
    }
    v3 = fileno(v7);
    if ((!fstat(v3,v14)) && ((v23 & 0xf000) == 0x8000)) {
      v19 = dat_99c0;
      if ((double)sub_4d60() * dat_99c8 <= dat_99c0)
        v19 = dat_99c8 * (double)sub_4d60();
      if ((double)v24 <= v19) {
        sub_74d0(&v16);
        v21 = sub_7580(v7,&v16);
        if ((!v21) || (sub_4960(v7))) {
          v9 = sub_7110(4,v10);
          error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v9);
          return v12;
        }
        v20 = v16;
        v4 = sub_4700(v22);
        a1 = v22;
        goto label_2b30;
      }
    }
    v4 = sub_4730(v7);
    v21 = '\0';
    v20 = 0;
    a1 = NULL;
  }
  else {
    if (a0 <= optind) { // branch-flip
      v20 = 1;
      a1 = (char **)0xe110;
    }
    else {
      v20 = (long)(a0 - optind);
      a1 = &a1[optind];
    }
    v4 = sub_4700(a1);
    v21 = '\0';
  }
label_2b30:
  if (v4) {
    v5 = (unsigned int *)sub_32f0(v20,a1);
    dat_e12c = sub_3230(v20,v5);
    v2 = 1;
    v13 = v5;
    while (v6 = (char *)sub_4770(v4,&v15), v6) {
      if (v10) { // branch-flip
        if ((strcmp(v10,"-")) || (strcmp(v6,"-"))) {
          if (*v6) goto label_2ba5;
        }
        else {
          v9 = sub_7110(4,v6);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v9);
          if (*v6) goto label_2c21;
        }
        v9 = sub_4810(v4);
        v6 = dcgettext(NULL,"invalid zero-length file name",5);
        error(0,0,"%s:%lu: %s",sub_7210(0,3,v10),v9,v6);
        v2 = 0;
      }
      else if (*v6) { // branch-flip
label_2ba5:
        v17 = v5;
        if (v20)
          v17 = v13;
        v2 &= sub_4500(v6,v17);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
label_2c21:
        v2 = 0;
      }
      if (!v20)
        *v5 = 1;
      v13 = &v13[0x26];
    }
    if (v15 != 3) {
      if (v15 != 4) { // branch-flip
        if (v15 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
        v1 = v10 == NULL & v2;
        if ((v1) && (v2 = v1, !sub_4810(v4)))
          v2 = sub_4500(0,v5);
      }
      else {
        v9 = sub_7210(0,3,v10);
        v10 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v10,v9);
        v2 = 0;
      }
      if (v21)
        sub_7550(&v16);
      if (2 <= (unsigned long)sub_4810(v4))
        sub_35d0(dat_e158,dat_e150,dat_e148,dat_e140,dat_e138,dcgettext(NULL,"total",5));
      sub_4830(v4);
      free(v5);
      if ((dat_e128) && (close(0))) {
        error(1,*__errno_location(),"-");
        return v11;
      }
      if (v25 == *(long *)(v18 + 0x28))
        return (unsigned long)(v2 ^ 1);
      __stack_chk_fail(); // no-return
    }
  }
  sub_8250(); // no-return
}


// Function: sub_31f0 @ 0x31f0
bool sub_31f0(int a0) // return-dupe
{
  if (dat_e118)
    return 0;
  if ((a0 != 0xa0) && (a0 != 0x2007))
    return a0 == 0x202f || a0 == 0x2060;
  return 1;
}


// Function: sub_3230 @ 0x3230
int sub_3230(long a0,int *a1)
{
  bool v1;
  int v2; // ecx
  unsigned long v3;
  unsigned long v4; // rdx
  int *v5; // rdi
  int v6;
  
  if ((a0) && (*a1 <= 0)) {
    v3 = 0;
    v6 = 1;
    v5 = &a1[a0 * 0x26];
    do {
      if (!*a1) {
        if ((a1[8] & 0xf000U) != 0x8000) // branch-flip
          v6 = 7;
        else {
          v3 += *(long *)&a1[0xe];
        }
      }
      a1 = &a1[0x26];
    } while (a1 != v5);
    v2 = 1;
    if (10 <= v3) {
      do {
        v2 += 1;
        v4 = v3 / 10;
        v1 = 100 <= v3;
        v3 = v4;
      } while (v1);
    }
    if (v6 <= v2)
      v6 = v2;
    return v6;
  }
  return 1;
}


// Function: sub_32f0 @ 0x32f0
unsigned int * sub_32f0(long a0,long a1)
{
  char *v1;
  int v2; // eax
  unsigned int *v3; // rax
  long v4;
  unsigned int *v5;
  
  v4 = 1;
  if (a0)
    v4 = a0;
  v3 = (unsigned int *)sub_7fb0(v4,0x98);
  if ((a0) && ((a0 != 1 || ((unsigned int)dat_e134 + (unsigned int)dat_e133 + (unsigned int)dat_e132 + (unsigned int)dat_e131 + (unsigned int)dat_e130 != 1)))) {
    v4 = 0;
    v5 = &v3[2];
    do {
      v1 = *(char **)(a1 + v4 * 8);
      if (v1) { // branch-flip
        if (!strcmp(v1,"-")) goto label_3348;
        v2 = stat(v1,v5);
      }
      else {
label_3348:
        v2 = fstat(0,v5);
      }
      v4 += 1;
      v5[-2] = v2;
      v5 = &v5[0x26];
    } while (a0 != v4);
  }
  else {
    *v3 = 1;
  }
  return v3;
}


// Function: sub_33e0 @ 0x33e0
char sub_33e0(unsigned long a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  char *v1;
  long v10; // stack - 0x4070
  bool v2;
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  unsigned long v6; // rax
  long v7;
  char v8 [16392];
  long v9;
  
  if (!a2 || !a3)
    return 0;
  v2 = 0;
  v10 = 0;
  v9 = 0;
  while( true ) {
    v3 = sub_7770(a1,v8,0x4000);
    if (!v3) {
      *a3 = v10;
      *a2 = v9;
      return 1;
    }
    if (v3 == 0xffffffffffffffff) break;
    v10 += v3;
    v1 = &v8[v3];
    if (v2) {
      *v1 = '\n';
      v7 = v9;
      v5 = v8;
      while (v5 = rawmemchr(v5,10), v5 < v1) {
        v5 = &v5[1];
        v7 += 1;
      }
      v2 = (unsigned long)(v7 - v9) <= v3 / 0xf;
      v9 = v7;
    }
    else if (v1 != v8) { // branch-flip
      v5 = v8;
      v7 = v9;
      do {
        v4 = &v5[1];
        v7 += (unsigned long)(*v5 == '\n');
        v5 = v4;
      } while (v1 != v4);
      v2 = (unsigned long)(v7 - v9) <= v3 / 0xf;
      v9 = v7;
    }
    else {
      v2 = 1;
    }
  }
  v6 = sub_7210(0,3,a0);
  error(0,*__errno_location(),"%s",v6);
  return 0;
}


// Function: sub_35b0 @ 0x35b0
void sub_35b0(void)
{
  sub_31f0(btowc()); // tail-call
}


// Function: sub_35d0 @ 0x35d0
void sub_35d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  unsigned long v1; // rax
  char v2 [24];
  char *v3;
  char *v4;
  
  v3 = "%*s";
  if (dat_e134) {
    v1 = sub_4ad0(a0,v2);
    v3 = " %*s";
    __printf_chk(1,"%*s",(unsigned long)dat_e12c,v1);
  }
  v4 = v3;
  if (dat_e133) {
    v1 = sub_4ad0(a1,v2);
    v4 = " %*s";
    __printf_chk(1,v3,(unsigned long)dat_e12c,v1);
  }
  v3 = v4;
  if (dat_e132) {
    v1 = sub_4ad0(a2,v2);
    v3 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_e12c,v1);
  }
  v4 = v3;
  if (dat_e131) {
    v1 = sub_4ad0(a3,v2);
    v4 = " %*s";
    __printf_chk(1,v3,(unsigned long)dat_e12c,v1);
  }
  if (dat_e130) {
    v1 = sub_4ad0(a4,v2);
    __printf_chk(1,v4,(unsigned long)dat_e12c,v1);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_7210(0,3,a5);
    __printf_chk(1," %s",a5);
  }
  putchar_unlocked(10);
}


// Function: sub_3830 @ 0x3830
char sub_3830(void)
{
  char v1;
  int v2; // eax
  unsigned int v3 [2]; // stack - 0x38
  unsigned int v4; // stack - 0x3c
  unsigned int v5; // stack - 0x40
  char *v6; // rsi
  unsigned int *v7;
  
  v7 = &v5;
  v5 = 0;
  v4 = 0;
  v3[0] = 0;
  v3[1] = 0;
  v2 = sub_37b0(v7,&v4,v3,&v3[1]);
  v1 = dat_e160;
  if (v2) { // branch-flip
    if (v3[0] & 0x8000000) {
      v3[1] = 0;
      v3[0] = 0;
      v4 = 0;
      v5 = 0;
      if (!sub_37f0(v7,&v4,v3,&v3[1])) goto label_38d8;
      if (v4 & 0x20) {
        if (v1) {
          error(0,0,"%s",dcgettext(NULL,"using avx2 hardware support",5));
          return v1;
        }
        return '\x01';
      }
    }
    if (!v1)
      return '\0';
    v6 = "avx2 support not detected";
  }
  else {
label_38d8:
    if (!v1)
      return '\0';
    v6 = "failed to get cpuid";
  }
  error(0,0,"%s",dcgettext(NULL,v6,5));
  return '\0';
}


// Function: sub_3bf0 @ 0x3bf0
int sub_3bf0(int a0,char *a1,int *a2,long a3) // ternary
{
  char v1;
  unsigned int v10; // stack - 0x4064
  long v11; // stack - 0x4060
  long *v12;
  unsigned long v13;
  unsigned int v14;
  unsigned int v15;
  char *v16;
  char *v17;
  unsigned char v18;
  unsigned long v19;
  unsigned short v2;
  long v20;
  unsigned long v21;
  bool v22;
  unsigned long v23; // stack - 0x40a8
  long v24; // stack - 0x4098
  unsigned long v25; // stack - 0x4090
  unsigned char v26; // stack - 0x4079
  char *v27; // stack - 0x4070
  unsigned long v3;
  unsigned char v4; // al
  int v5; // eax
  void *v6;
  unsigned long v7;
  char v8 [16392];
  unsigned long v9 [2]; // stack - 0x4058
  
  v27 = a1;
  if (!a1) {
    v6 = (char *)dcgettext(NULL,"standard input",5);
    v27 = (char *)v6;
  }
  v9[0] = 0;
  v11 = 0;
  v6 = (unsigned long)__ctype_get_mb_cur_max();
  v18 = dat_e131; // branch-flip
  if (2 <= v6) {
    v4 = dat_e132;
    v26 = v4;
  }
  else {
    v26 = 0;
    if (!dat_e131)
      v18 = dat_e132;
  }
  if ((dat_e133) || (dat_e130))
    sub_4920(a0,0,0,2);
  else {
    if ((v18 == 1) && (!v26)) {
      if (dat_e134) {
        sub_4920(a0,0,0,2);
label_446e:
        if (dat_e134) {
label_4480:
          v4 = sub_3830();
          if (v4) { // branch-flip
            v6 = sub_8a90;
            dat_e010 = sub_8a90;
          }
          else {
            v6 = dat_e010;
          }
          v23 = 0;
          v19 = 0;
          v24 = 0;
          v5 = (*v6)(v27,a0,&v11,v9);
          v7 = v9[0];
          v15 = v5;
          goto label_3db8;
        }
      }
      v5 = *a2;
      if (0 < v5) {
        v5 = fstat(a0,&a2[2]);
        *a2 = v5;
      }
      if (!v5) {
        v5 = sub_3be0(a2[8]);
        v15 = v5;
        v19 = dat_e120;
        v18 = (unsigned char)v5;
        if ((v18) && (v7 = *(unsigned long *)&a2[0xe], 0 <= (long)v7)) {
          if (a3 != -1) { // branch-flip
            v19 = 0;
            if (v7 % dat_e120) {
label_442f:
              v23 = 0;
              v19 = 0;
              v24 = 0;
              v9[0] = v7;
              goto label_3db8;
            }
          }
          else {
            v6 = (unsigned long)lseek(a0,0,1);
            if (v7 % v19) {
              v7 = (v6 <= v7) ? v7 - (long)v6 : 0; // branch-flip
              goto label_442f;
            }
            v18 = (unsigned char)~(unsigned char)((unsigned long)v6 >> 0x38) >> 7;
            v19 = (unsigned long)v6;
          }
          v6 = (unsigned long)(*(long *)&a2[0x10] + 1);
          v21 = 0x201;
          if ((unsigned long)(*(long *)&a2[0x10] - 1U) <= 0x1fffffffffffffff)
            v21 = (unsigned long)v6;
          v20 = v7 - v7 % v21;
          if ((((long)v19 < v20) && (v18)) && (v6 = (long)lseek(a0,v20,1), 0 <= (long)v6))
            v9[0] = v20 - v19;
        }
      }
      sub_4920(a0,0,0,2);
      while (v6 = (long)sub_7770(a0,v8,0x4000), v6) {
        if (v6 == (void *)0xffffffffffffffff) {
          v23 = 0;
          v24 = 0;
          v6 = (unsigned long)sub_7210(0,3,v27);
          v3 = v6;
          v6 = (int *)__errno_location();
          v19 = 0;
          error(0,*(int *)v6,"%s",v3);
          v7 = v9[0];
          v15 = 0;
          goto label_3db8;
        }
        v9[0] += (long)v6;
      }
      v23 = 0;
      v24 = 0;
      v19 = 0;
      v7 = v9[0];
      v15 = 1;
      goto label_3db8;
    }
    sub_4920(a0,0,0,2);
    if (v26 != 1) {
      if (v18) goto label_446e;
      if (!v26) goto label_4480;
    }
  }
  v6 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v6) { // branch-flip
    v21 = 0;
    v22 = 0;
    v7 = 0;
    v9[1] = 0;
    v19 = 0;
    v23 = 0;
    v25 = 0;
    v24 = 0;
label_3ea8:
    v6 = (long)sub_7770(a0,&v8[v21],0x4000 - v21);
    if (v6) { // branch-flip
      if (v6 != (void *)0xffffffffffffffff) {
        v9[0] += (long)v6;
        v21 += (long)v6;
        v16 = v8;
        if (v22) goto label_3f9c;
label_3ef0:
        v1 = *v16;
        v14 = (unsigned int)v1;
        v4 = sub_4bc0(v14);
        v18 = v4;
        if (!v4) goto label_3f9c;
        v10 = v14;
        switch(v1) {
          case 9:
            v20 = 1;
label_40b5:
            v6 = (unsigned long)(v23 & 0xfffffffffffffff8);
            v6 = (unsigned long)((long)v6 + 8);
            v23 = (unsigned long)v6;
label_4072:
            v24 += v7;
            v7 = 0;
label_3f80:
            do {
              v16 = &v16[v20];
              v21 -= v20;
              v19 += 1;
              while( true ) {
                if (!v21) goto label_3ea8;
                if (!v22) goto label_3ef0;
label_3f9c:
                v13 = v9[1];
                v6 = (long)sub_4be0(&v10,v16,v21,&v9[1]);
                v20 = (long)v6;
                if (v6 == (void *)0xfffffffffffffffe) {
                  v9[1] = v13;
                  if (v21) {
                    if (v21 == 0x4000) {
                      v16 = &v16[1];
                      v21 = 0x3fff;
                    }
                    __memmove_chk(v8,v16,v21,0x4001);
                  }
                  v22 = 1;
                  goto label_3ea8;
                }
                if (v6 != (void *)0xffffffffffffffff) break;
                v16 = &v16[1];
                v21 -= 1;
                v22 = 1;
              }
              v5 = mbsinit(&v9[1]);
              v22 = v5 == 0;
              if (v20) { // branch-flip
                switch(v10) {
                  case 9:
                    goto label_40b5;
                  case 10:
                    goto label_40a5;
                  case 0xb:
                    goto label_4072;
                  case 0xc:
                  case 0xd:
                    goto label_4055;
                  default:
                    v14 = v10;
                    break;
                  case 0x20:
                    goto label_4095;
                  
                }
              }
              else {
                v10 = 0;
                v20 = 1;
                v14 = 0;
              }
              v5 = iswprint(v14);
              if (v5) {
                if (dat_e130) {
                  v5 = wcwidth(v10);
                  v13 = (long)v5 + v23;
                  if (v5 <= 0)
                    v13 = v23;
                  v23 = v13;
                }
                v5 = iswspace(v10);
                if ((v5) || (v5 = sub_31f0(v10), v5)) goto label_4072;
                v7 = 1;
              }
            } while( true );
          case 10:
            v20 = 1;
            goto label_40a5;
          case 0xb:
label_4080:
            v20 = 1;
            goto label_4072;
          case 0xc:
          case 0xd:
            v20 = 1;
            goto label_4055;
          case 0x20:
            v20 = 1;
            goto label_4095;
          
        }
        v6 = __ctype_b_loc();
        v12 = (long *)v6;
        v4 = sub_31e0(v14);
        v6 = (unsigned long)(unsigned long)v4;
        v2 = *(unsigned short *)(*v12 + (long)v6 * 2);
        if (v2 & 0x4000) {
          v23 += 1;
          if (v2 & 0x2000) goto label_4080;
          v7 = (unsigned long)v18;
        }
        v20 = 1;
        goto label_3f80;
      }
      v6 = (unsigned long)sub_7210(0,3,v27);
      v3 = v6;
      v6 = (int *)__errno_location();
      v15 = 0;
      error(0,*(int *)v6,"%s",v3);
    }
    else {
      v15 = 1;
    }
    if (v23 <= v25)
      v23 = v25;
    v24 = v7 + v24;
    v7 = v9[0];
  }
  else {
    v7 = 0;
    v23 = 0;
    v24 = 0;
    v19 = v23;
    while (v6 = (long)sub_7770(a0,v8,0x4000), v6) {
      if (v6 == (void *)0xffffffffffffffff) {
        v6 = (unsigned long)sub_7210(0,3,v27);
        v3 = v6;
        v6 = (int *)__errno_location();
        v15 = 0;
        error(0,*(int *)v6,"%s",v3);
        goto label_4347;
      }
      v9[0] += (long)v6;
      v16 = &v8[(long)v6];
      v17 = v8;
      do {
        v1 = *v17;
        v17 = &v17[1];
        switch(v1) {
          case 9:
            v7 = (v7 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v11 += 1;
            goto label_4266;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_4266:
            if (v23 < v7)
              v23 = v7;
            v7 = 0;
            break;
          default:
            v6 = __ctype_b_loc();
            v12 = (long *)v6;
            v4 = sub_31e0((int)v1);
            v2 = *(unsigned short *)(*v12 + (unsigned long)v4 * 2);
            if (v2 & 0x4000) {
              v7 += 1;
              if ((v2 & 0x2000) || (v5 = sub_35b0(v4), v5)) goto label_426f;
              v24 = 1;
            }
            goto label_4250;
          case 0x20:
            v7 += 1;
          
        }
label_426f:
        v19 += v24;
        v24 = 0;
label_4250:
      } while (v16 != v17);
    }
    v15 = 1;
label_4347:
    if (v23 < v7)
      v23 = v7;
    v24 += v19;
    v19 = 0;
    v7 = v9[0];
  }
label_3db8:
  if (v26 < dat_e132)
    v19 = v7;
  sub_35d0(v11,v24,v19,v7,v23,a1);
  dat_e150 += v24;
  dat_e158 += v11;
  dat_e148 += v19;
  dat_e140 += v9[0];
  if (dat_e138 < v23)
    dat_e138 = v23;
  v5 = v15;
  return v5;
label_4095:
  v23 += 1;
  goto label_4072;
label_40a5:
  v11 += 1;
  goto label_4055;
label_4055:
  v6 = (void *)v23;
  v23 = 0;
  if (v6 <= v25)
    v6 = (void *)v25;
  v25 = (unsigned long)v6;
  goto label_4072;
}


// Function: sub_4500 @ 0x4500
unsigned long sub_4500(char *a0,unsigned long a1)
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  if (a0) {
    if (strcmp(a0,"-")) {
      v1 = open(a0,0);
      if (v1 != -1) {
        v2 = sub_3bf0(v1,a0,a1,0);
        if (!close(v1))
          return (unsigned long)v2;
      }
      v4 = sub_7210(0,3,a0);
      error(0,*__errno_location(),"%s",v4);
      return 0;
    }
  }
  dat_e128 = 1;
  v3 = sub_3bf0(0,a0,a1,0xffffffffffffffff); // tail-call
  return v3;
}


// Function: sub_45c0 @ 0x45c0
void sub_45c0(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_e178;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n",5),v1);
    sub_32c0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_39b0();
  }
  exit(a0); // no-return
}


// Function: sub_8a90 @ 0x8a90
unsigned long sub_8a90(unsigned long a0,unsigned int a1,long *a2,long *a3)
{
  char v1 [32];
  int v10; // r8d
  int v11; // r9d
  bool v12; // r13b
  unsigned long v13; // r13
  unsigned long v14; // r13
  long v15;
  char v16 [32];
  char v17 [32];
  char v18 [64]; // zmm3
  unsigned long v2; // rax
  char (*v3)[32];
  unsigned long v4; // rax
  char (*v5)[32];
  long v6; // rbx
  char v7 [64];
  char v8 [16280];
  int v9; // r8d
  
  v12 = !a2 || !a3;
  v14 = CONCAT71((undefined7)((unsigned long)v13 >> 8),v12);
  if (v12)
    v14 = 0;
  else {
    v6 = 0;
    v15 = 0;
    while (v2 = sub_7770(a1,(char (*)[32])v7,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = sub_7210(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v14 & 0xffffffff; // return-dupe
      }
      v6 += v2;
      if (0x40 <= v2) { // branch-flip
        v18 = ZEXT1664(0);
        v17 = SUB6432(v18,0);
        v5 = (char (*)[32])v7;
        do {
          v16 = vpcmpeqb_avx2(dat_a200,*v5);
          v1 = vpcmpeqb_avx2(dat_a200,v5[1]);
          v5 = &v5[2];
          v17 = vpsubb_avx2(v17,v16);
          v16 = vpsubb_avx2(SUB6432(v18,0),v1);
          v18 = ZEXT3264(v16);
          v3 = (char (*)[32])&v8[v2 - 0x40 & 0xffffffffffffffc0];
        } while (v5 != (char (*)[32])&v8[v2 - 0x40 & 0xffffffffffffffc0]);
      }
      else {
        v17 = SUB6432(ZEXT1664(0),0);
        v3 = (char (*)[32])v7;
        v16 = v17;
      }
      v17 = vpsadbw_avx2(v17,SUB6432(ZEXT1664(0),0));
      v16 = vpsadbw_avx2(v16,SUB6432(ZEXT1664(0),0));
      v9 = vpextrw_avx(SUB3216(v17,0),0);
      v10 = vpextrw_avx(SUB3216(v17,0x10),0);
      v11 = vpextrw_avx(SUB3216(v16,0),0);
      v15 += (long)(vpextrw_avx(SUB3216(v17,0),4) + v9 + v10 + vpextrw_avx(SUB3216(v17,0x10),4)) + (long)(vpextrw_avx(SUB3216(v16,0),4) + v11 + vpextrw_avx(SUB3216(v16,0x10),0) + vpextrw_avx(SUB3216(v16,0x10),4));
      for (; (char (*)[32])&(*(char (*)[32])v7)[v2] != v3; v3 = (char (*)[32])&(*v3)[1]) {
        v15 += (unsigned long)((*v3)[0] == '\n');
      }
    }
    v14 = 1;
    *a2 = v15;
    *a3 = v6;
  }
  return v14 & 0xffffffff;
}

