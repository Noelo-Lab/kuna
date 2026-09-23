// Function: main @ 0x28a0
int main(int argc,char **argv,char **envp)
{
  unsigned char v1; // al
  char *v10; // rax
  FILE *v11;
  char *v12; // rax
  unsigned long v13; // rax
  unsigned long v14; // rax
  int *v15;
  stat v16; // stack - 0xd8
  int v17; // stack - 0x1fc
  long v18; // stack - 0x1f8
  int *v19;
  unsigned char v2;
  long v20; // fs_offset
  double v21;
  long v22; // stack - 0x228
  bool v23; // stack - 0x209
  char **v24; // stack - 0x1f0
  long v25; // stack - 0x40
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  char *v7;
  struct_4 *v8; // rax
  int *v9; // rax
  
  v25 = *(long *)(v20 + 0x28);
  sub_4ea0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8cd0(sub_4870);
  dat_e120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  v7 = getenv("POSIXLY_CORRECT");
  dat_e131 = '\0';
  dat_e132 = '\0';
  dat_e118 = v7 != NULL;
  dat_e133 = '\0';
  dat_e134 = '\0';
  dat_e130 = '\0';
  dat_e138 = 0;
  dat_e140 = 0;
  dat_e148 = 0;
  dat_e150 = 0;
  dat_e158 = 0;
  v7 = NULL;
  while( true ) {
    v3 = getopt_long(argc,argv,"clLmw",(option *)0xda20,NULL);
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
        v7 = optarg;
      
    }
  }
  if ((((!dat_e134) && (!dat_e133)) && (!dat_e132)) && ((!dat_e131 && (!dat_e130)))) {
    dat_e131 = '\x01';
    dat_e133 = '\x01';
    dat_e134 = '\x01';
  }
  if (v7) { // branch-flip
    if (optind < argc) {
      v7 = (char *)sub_7110(4,argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v7);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
label_3048:
      sub_45c0(1);
    }
    if (strcmp(v7,"-")) { // branch-flip
      v11 = fopen(v7,"r");
      if (!v11) {
        v14 = sub_7110(4,v7);
        v7 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v7,v14);
        return v4;
      }
    }
    else {
      v11 = stdin;
    }
    v3 = fileno(v11);
    if ((!fstat(v3,&v16)) && ((v16._24_4_ & 0xf000) == 0x8000)) {
      v21 = dat_99c0;
      if ((double)sub_4d60() * dat_99c8 <= dat_99c0)
        v21 = dat_99c8 * (double)sub_4d60();
      if ((double)(long)v16._48_8_ <= v21) {
        sub_74d0(&v18);
        v23 = sub_7580(v11,&v18);
        if ((!v23) || (sub_4960(v11))) {
          v7 = (char *)sub_7110(4,v7);
          error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v7);
          return v6;
        }
        v22 = v18;
        v8 = (struct_4 *)sub_4700(v24);
        argv = v24;
        goto label_2b30;
      }
    }
    v8 = (struct_4 *)sub_4730(v11);
    v23 = 0;
    v22 = 0;
    argv = NULL;
  }
  else {
    if (argc <= optind) { // branch-flip
      v22 = 1;
      argv = (char **)0xe110;
    }
    else {
      v22 = (long)(argc - optind);
      argv = &argv[optind];
    }
    v8 = (struct_4 *)sub_4700(argv);
    v23 = 0;
  }
label_2b30:
  if (v8) {
    v9 = (int *)sub_32f0(v22,argv);
    dat_e12c = sub_3230(v22,v9);
    v2 = 1;
    v15 = v9;
    while (v10 = (char *)sub_4770(v8,&v17), v10) {
      if (v7) { // branch-flip
        if ((strcmp(v7,"-")) || (strcmp(v10,"-"))) {
          if (*v10) goto label_2ba5;
        }
        else {
          v12 = (char *)sub_7110(4,v10);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v12);
          if (*v10) goto label_2c21;
        }
        v13 = sub_4810(v8);
        v10 = dcgettext(NULL,"invalid zero-length file name",5);
        error(0,0,"%s:%lu: %s",(char *)sub_7210(0,3,v7),v13,v10);
        v2 = 0;
      }
      else if (*v10) { // branch-flip
label_2ba5:
        v19 = v9;
        if (v22)
          v19 = v15;
        v2 &= sub_4500(v10,v19);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
label_2c21:
        v2 = 0;
      }
      if (!v22)
        *v9 = 1;
      v15 = &v15[0x26];
    }
    if (v17 != 3) {
      if (v17 != 4) { // branch-flip
        if (v17 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
        v1 = v7 == NULL & v2;
        if ((v1) && (v2 = v1, !sub_4810(v8)))
          v2 = sub_4500(NULL,v9);
      }
      else {
        v14 = sub_7210(0,3,v7);
        v7 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v7,v14);
        v2 = 0;
      }
      if (v23)
        sub_7550(&v18);
      if (2 <= (unsigned long)sub_4810(v8))
        sub_35d0(dat_e158,dat_e150,dat_e148,dat_e140,dat_e138,dcgettext(NULL,"total",5));
      sub_4830(v8);
      free(v9);
      if ((dat_e128) && (close(0))) {
        error(1,*__errno_location(),"-");
        return v5;
      }
      if (v25 == *(long *)(v20 + 0x28))
        return (int)(v2 ^ 1);
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
unsigned int * sub_32f0(unsigned long a0,long a1)
{
  char *v1;
  int v2; // eax
  unsigned int *v3; // rax
  unsigned long v4;
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
        v2 = stat(v1,(stat *)v5);
      }
      else {
label_3348:
        v2 = fstat(0,(stat *)v5);
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
char sub_33e0(char *a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  unsigned long v2; // rax
  char *v3; // rax
  char *v4;
  char *v5;
  unsigned long v6;
  char v7 [16392];
  unsigned long v8;
  long v9; // stack - 0x4070
  
  if (!a2 || !a3)
    return 0;
  v1 = 0;
  v9 = 0;
  v8 = 0;
  while( true ) {
    v2 = sub_7770(a1,v7,0x4000);
    if (!v2) {
      *a3 = v9;
      *a2 = v8;
      return 1;
    }
    if (v2 == 0xffffffffffffffff) break;
    v9 += v2;
    v5 = &v7[v2];
    if (v1) {
      *v5 = '\n';
      v6 = v8;
      v4 = v7;
      while (v4 = rawmemchr(v4,10), v4 < v5) {
        v4 = &v4[1];
        v6 += 1;
      }
      v1 = (unsigned long)(v6 - v8) <= v2 / 0xf;
      v8 = v6;
    }
    else if (v5 != v7) { // branch-flip
      v4 = v7;
      v6 = v8;
      do {
        v3 = &v4[1];
        v6 += (unsigned long)(*v4 == '\n');
        v4 = v3;
      } while (v5 != v3);
      v1 = (unsigned long)(v6 - v8) <= v2 / 0xf;
      v8 = v6;
    }
    else {
      v1 = 1;
    }
  }
  v5 = (char *)sub_7210(0,3,a0);
  error(0,*__errno_location(),"%s",v5);
  return 0;
}


// Function: sub_35b0 @ 0x35b0
void sub_35b0(int a0)
{
  sub_31f0(btowc(a0)); // tail-call
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
int sub_3bf0(int a0,char *a1,struct_2 *a2,long a3) // ternary
{
  char v1;
  char v10 [16392];
  mbstate_t v11; // stack - 0x4050
  unsigned int v12; // stack - 0x4064
  unsigned long v13; // stack - 0x4058
  long v14; // stack - 0x4060
  long *v15;
  unsigned long v16;
  unsigned int v17;
  unsigned int v18;
  char *v19;
  unsigned short v2;
  char *v20;
  unsigned char v21;
  unsigned long v22;
  long v23;
  bool v24;
  unsigned long v25; // stack - 0x40a8
  long v26; // stack - 0x4098
  unsigned long v27; // stack - 0x4090
  unsigned char v28; // stack - 0x4079
  char *v29; // stack - 0x4070
  unsigned long v3;
  mbstate_t v4;
  unsigned char v5; // al
  int v6; // eax
  void *v7;
  unsigned long v8;
  unsigned long v9;
  
  v29 = a1;
  if (!a1) {
    v7 = (char *)dcgettext(NULL,"standard input",5);
    v29 = (char *)v7;
  }
  v13 = 0;
  v14 = 0;
  v7 = (unsigned long)__ctype_get_mb_cur_max();
  v21 = dat_e131; // branch-flip
  if (2 <= v7) {
    v5 = dat_e132;
    v28 = v5;
  }
  else {
    v28 = 0;
    if (!dat_e131)
      v21 = dat_e132;
  }
  if ((dat_e133) || (dat_e130))
    sub_4920(a0,0,0,2);
  else {
    if ((v21 == 1) && (!v28)) {
      if (dat_e134) {
        sub_4920(a0,0,0,2);
label_446e:
        if (dat_e134) {
label_4480:
          v5 = sub_3830();
          if (v5) { // branch-flip
            v7 = sub_8a90;
            dat_e010 = sub_8a90;
          }
          else {
            v7 = dat_e010;
          }
          v25 = 0;
          v22 = 0;
          v9 = 0;
          v6 = (*v7)(v29,a0,&v14,&v13);
          v8 = v13;
          v18 = v6;
          goto label_3db8;
        }
      }
      v6 = a2->field_0x0;
      if (0 < v6) {
        v6 = fstat(a0,(stat *)&a2->field_0x4[4]);
        a2->field_0x0 = v6;
      }
      if (!v6) {
        v6 = sub_3be0(a2->field_0x20);
        v18 = v6;
        v22 = dat_e120;
        v21 = (unsigned char)v6;
        if ((v21) && (v8 = a2->field_0x38, 0 <= (long)v8)) {
          if (a3 != -1) { // branch-flip
            v22 = 0;
            if (v8 % dat_e120) {
label_442f:
              v25 = 0;
              v22 = 0;
              v9 = 0;
              v13 = v8;
              goto label_3db8;
            }
          }
          else {
            v7 = (unsigned long)lseek(a0,0,1);
            if (v8 % v22) {
              v8 = (v7 <= v8) ? v8 - (long)v7 : 0; // branch-flip
              goto label_442f;
            }
            v21 = (unsigned char)~(unsigned char)((unsigned long)v7 >> 0x38) >> 7;
            v22 = (unsigned long)v7;
          }
          v7 = (unsigned long)(a2->field_0x40 + 1);
          v9 = 0x201;
          if ((unsigned long)(a2->field_0x40 - 1U) <= 0x1fffffffffffffff)
            v9 = (unsigned long)v7;
          v23 = v8 - v8 % v9;
          if ((((long)v22 < v23) && (v21)) && (v7 = (long)lseek(a0,v23,1), 0 <= (long)v7))
            v13 = v23 - v22;
        }
      }
      sub_4920(a0,0,0,2);
      while (v7 = (long)sub_7770(a0,v10,0x4000), v7) {
        if (v7 == (void *)0xffffffffffffffff) {
          v25 = 0;
          v9 = 0;
          v7 = (unsigned long)sub_7210(0,3,v29);
          v3 = v7;
          v7 = (int *)__errno_location();
          v22 = 0;
          error(0,*(int *)v7,"%s",v3);
          v8 = v13;
          v18 = 0;
          goto label_3db8;
        }
        v13 += (long)v7;
      }
      v25 = 0;
      v9 = 0;
      v22 = 0;
      v8 = v13;
      v18 = 1;
      goto label_3db8;
    }
    sub_4920(a0,0,0,2);
    if (v28 != 1) {
      if (v21) goto label_446e;
      if (!v28) goto label_4480;
    }
  }
  v7 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v7) { // branch-flip
    v8 = 0;
    v24 = 0;
    v9 = 0;
    v11 = 0;
    v22 = 0;
    v25 = 0;
    v27 = 0;
    v26 = 0;
label_3ea8:
    v7 = (long)sub_7770(a0,&v10[v8],0x4000 - v8);
    if (v7) { // branch-flip
      if (v7 != (void *)0xffffffffffffffff) {
        v13 += (long)v7;
        v8 += (long)v7;
        v19 = v10;
        if (v24) goto label_3f9c;
label_3ef0:
        v1 = *v19;
        v17 = (unsigned int)v1;
        v5 = sub_4bc0(v17);
        v21 = v5;
        if (!v5) goto label_3f9c;
        v12 = v17;
        switch(v1) {
          case 9:
            v23 = 1;
label_40b5:
            v7 = (unsigned long)(v25 & 0xfffffffffffffff8);
            v7 = (unsigned long)((long)v7 + 8);
            v25 = (unsigned long)v7;
label_4072:
            v26 += v9;
            v9 = 0;
label_3f80:
            do {
              v19 = &v19[v23];
              v8 -= v23;
              v22 += 1;
              while( true ) {
                if (!v8) goto label_3ea8;
                if (!v24) goto label_3ef0;
label_3f9c:
                v4 = v11;
                v7 = (long)sub_4be0(&v12,v19,v8,&v11);
                v23 = (long)v7;
                if (v7 == (void *)0xfffffffffffffffe) {
                  v11 = v4;
                  if (v8) {
                    if (v8 == 0x4000) {
                      v19 = &v19[1];
                      v8 = 0x3fff;
                    }
                    __memmove_chk(v10,v19,v8,0x4001);
                  }
                  v24 = 1;
                  goto label_3ea8;
                }
                if (v7 != (void *)0xffffffffffffffff) break;
                v19 = &v19[1];
                v8 -= 1;
                v24 = 1;
              }
              v6 = mbsinit(&v11);
              v24 = v6 == 0;
              if (v23) { // branch-flip
                switch(v12) {
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
                    v17 = v12;
                    break;
                  case 0x20:
                    goto label_4095;
                  
                }
              }
              else {
                v12 = 0;
                v23 = 1;
                v17 = 0;
              }
              v6 = iswprint(v17);
              if (v6) {
                if (dat_e130) {
                  v6 = wcwidth(v12);
                  v16 = (long)v6 + v25;
                  if (v6 <= 0)
                    v16 = v25;
                  v25 = v16;
                }
                v6 = iswspace(v12);
                if ((v6) || (v6 = sub_31f0(v12), v6)) goto label_4072;
                v9 = 1;
              }
            } while( true );
          case 10:
            v23 = 1;
            goto label_40a5;
          case 0xb:
label_4080:
            v23 = 1;
            goto label_4072;
          case 0xc:
          case 0xd:
            v23 = 1;
            goto label_4055;
          case 0x20:
            v23 = 1;
            goto label_4095;
          
        }
        v7 = __ctype_b_loc();
        v15 = (long *)v7;
        v5 = sub_31e0(v17);
        v7 = (unsigned long)(unsigned long)v5;
        v2 = *(unsigned short *)(*v15 + (long)v7 * 2);
        if (v2 & 0x4000) {
          v25 += 1;
          if (v2 & 0x2000) goto label_4080;
          v9 = (unsigned long)v21;
        }
        v23 = 1;
        goto label_3f80;
      }
      v7 = (unsigned long)sub_7210(0,3,v29);
      v3 = v7;
      v7 = (int *)__errno_location();
      v18 = 0;
      error(0,*(int *)v7,"%s",v3);
    }
    else {
      v18 = 1;
    }
    if (v25 <= v27)
      v25 = v27;
    v9 += v26;
    v8 = v13;
  }
  else {
    v22 = 0;
    v25 = 0;
    v23 = 0;
    v9 = v25;
    while (v7 = (long)sub_7770(a0,v10,0x4000), v7) {
      if (v7 == (void *)0xffffffffffffffff) {
        v7 = (unsigned long)sub_7210(0,3,v29);
        v3 = v7;
        v7 = (int *)__errno_location();
        v18 = 0;
        error(0,*(int *)v7,"%s",v3);
        goto label_4347;
      }
      v13 += (long)v7;
      v19 = &v10[(long)v7];
      v20 = v10;
      do {
        v1 = *v20;
        v20 = &v20[1];
        switch(v1) {
          case 9:
            v22 = (v22 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v14 += 1;
            goto label_4266;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_4266:
            if (v25 < v22)
              v25 = v22;
            v22 = 0;
            break;
          default:
            v7 = __ctype_b_loc();
            v15 = (long *)v7;
            v5 = sub_31e0((int)v1);
            v2 = *(unsigned short *)(*v15 + (unsigned long)v5 * 2);
            if (v2 & 0x4000) {
              v22 += 1;
              if ((v2 & 0x2000) || (v6 = sub_35b0(v5), v6)) goto label_426f;
              v23 = 1;
            }
            goto label_4250;
          case 0x20:
            v22 += 1;
          
        }
label_426f:
        v9 += v23;
        v23 = 0;
label_4250:
      } while (v19 != v20);
    }
    v18 = 1;
label_4347:
    if (v25 < v22)
      v25 = v22;
    v9 = v23 + v9;
    v22 = 0;
    v8 = v13;
  }
label_3db8:
  if (v28 < dat_e132)
    v22 = v8;
  sub_35d0(v14,v9,v22,v8,v25,a1);
  dat_e150 += v9;
  dat_e158 += v14;
  dat_e148 += v22;
  dat_e140 += v13;
  if (dat_e138 < v25)
    dat_e138 = v25;
  v6 = v18;
  return v6;
label_4095:
  v25 += 1;
  goto label_4072;
label_40a5:
  v14 += 1;
  goto label_4055;
label_4055:
  v7 = (void *)v25;
  v25 = 0;
  if (v7 <= v27)
    v7 = (void *)v27;
  v27 = (unsigned long)v7;
  goto label_4072;
}


// Function: sub_4500 @ 0x4500
unsigned long sub_4500(char *a0,struct_2 *a1)
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  
  if (a0) {
    if (strcmp(a0,"-")) {
      v1 = open(a0,0);
      if (v1 != -1) {
        v2 = sub_3bf0(v1,a0,a1,0);
        if (!close(v1))
          return (unsigned long)v2;
      }
      v4 = (char *)sub_7210(0,3,a0);
      error(0,*__errno_location(),"%s",v4);
      return 0;
    }
  }
  dat_e128 = 1;
  v3 = sub_3bf0(0,a0,a1,-1); // tail-call
  return v3;
}


// Function: sub_45c0 @ 0x45c0
void sub_45c0(int a0)
{
  FILE *v1;
  char *v2;
  
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
unsigned long sub_8a90(char *a0,unsigned int a1,long *a2,long *a3)
{
  char v1 [32];
  int v10; // r8d
  int v11; // r8d
  int v12; // r8d
  int v13; // r8d
  int v14; // r9d
  int v15; // r9d
  bool v16; // r13b
  unsigned long v17; // r13
  unsigned long v18; // r13
  long v19;
  unsigned long v2; // rax
  char v20 [32];
  char v21 [32];
  char v22 [64]; // zmm3
  char (*v3)[32];
  char *v4; // rax
  int v5; // edx
  char (*v6)[32];
  long v7; // rbx
  char v8 [64];
  char v9 [16280];
  
  v16 = !a2 || !a3;
  v18 = CONCAT71((undefined7)((unsigned long)v17 >> 8),v16);
  if (v16)
    v18 = 0;
  else {
    v7 = 0;
    v19 = 0;
    while (v2 = sub_7770(a1,(char (*)[32])v8,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = (char *)sub_7210(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v18 & 0xffffffff; // return-dupe
      }
      v7 += v2;
      if (0x40 <= v2) { // branch-flip
        v22 = ZEXT1664(0);
        v21 = SUB6432(v22,0);
        v6 = (char (*)[32])v8;
        do {
          v20 = vpcmpeqb_avx2(dat_a200,*v6);
          v1 = vpcmpeqb_avx2(dat_a200,v6[1]);
          v6 = &v6[2];
          v21 = vpsubb_avx2(v21,v20);
          v20 = vpsubb_avx2(SUB6432(v22,0),v1);
          v22 = ZEXT3264(v20);
          v3 = (char (*)[32])&v9[v2 - 0x40 & 0xffffffffffffffc0];
        } while (v6 != (char (*)[32])&v9[v2 - 0x40 & 0xffffffffffffffc0]);
      }
      else {
        v21 = SUB6432(ZEXT1664(0),0);
        v3 = (char (*)[32])v8;
        v20 = v21;
      }
      v21 = vpsadbw_avx2(v21,SUB6432(ZEXT1664(0),0));
      v20 = vpsadbw_avx2(v20,SUB6432(ZEXT1664(0),0));
      v10 = vpextrw_avx(SUB3216(v21,0),0);
      v5 = vpextrw_avx(SUB3216(v21,0),4);
      v11 = vpextrw_avx(SUB3216(v21,0x10),0);
      v14 = vpextrw_avx(SUB3216(v20,0),0);
      v12 = vpextrw_avx(SUB3216(v21,0x10),4);
      v13 = vpextrw_avx(SUB3216(v20,0),4);
      v15 = vpextrw_avx(SUB3216(v20,0x10),0);
      v19 += (long)(v5 + v10 + v11 + v12) + (long)(v13 + v14 + v15 + vpextrw_avx(SUB3216(v20,0x10),4));
      for (; (char (*)[32])&(*(char (*)[32])v8)[v2] != v3; v3 = (char (*)[32])&(*v3)[1]) {
        v19 += (unsigned long)((*v3)[0] == '\n');
      }
    }
    v18 = 1;
    *a2 = v19;
    *a3 = v7;
  }
  return v18 & 0xffffffff;
}

