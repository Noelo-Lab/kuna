// Function: sub_2860 @ 0x2860
unsigned long sub_2860(int a0,char **a1)
{
  bool v1;
  char *v10;
  unsigned long v11; // rax
  int *v12;
  unsigned long v13; // rax
  unsigned long v14; // rcx
  long v15;
  unsigned long v16;
  char v17 [24];
  int v18; // stack - 0x1fc
  unsigned long v19; // stack - 0x1f8
  unsigned char v2; // al
  unsigned long v20;
  long v21; // fs_offset
  double v22;
  long v23; // stack - 0x230
  int *v24; // stack - 0x220
  unsigned char v25; // stack - 0x218
  char v26; // stack - 0x20d
  char **v27; // stack - 0x1f0
  unsigned int v28; // stack - 0xc0
  long v29; // stack - 0xa8
  unsigned char v3;
  long v30; // stack - 0x40
  int v4; // eax
  long v5; // rax
  char *v6;
  void *v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v30 = *(long *)(v21 + 0x28);
  sub_4d10(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_88e0(sub_46f0);
  dat_d120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  dat_d131 = 0;
  dat_d132 = 0;
  dat_d118 = getenv("POSIXLY_CORRECT") != NULL;
  dat_d133 = 0;
  dat_d134 = 0;
  dat_d130 = 0;
  dat_d138 = 0;
  dat_d140 = 0;
  dat_d148 = 0;
  dat_d150 = 0;
  dat_d158 = 0;
  v10 = NULL;
  while( true ) {
    v9 = 0;
    v4 = getopt_long(a0,a1,"clLmw",(void *)0xca20,NULL);
    if (v4 == -1) break;
    if (0x81 < v4) {
      sub_4230(1); // return-dupe, no-return
    }
    if (v4 <= 0x4b) {
      if (v4 == -0x83) {
        sub_7a40(stdout,"wc","GNU coreutils",dat_d018,"Paul Rubin","David MacKenzie",0,v9);
        exit(0); // no-return
      }
      if (v4 == -0x82)
        sub_4230(0); // no-return
      sub_4230(1);
    }
    switch(v4) {
      case 0x4c:
        dat_d130 = 1;
        break;
      default:
        goto label_3299;
      case 99:
        dat_d131 = 1;
        break;
      case 0x6c:
        dat_d134 = 1;
        break;
      case 0x6d:
        dat_d132 = 1;
        break;
      case 0x77:
        dat_d133 = 1;
        break;
      case 0x80:
        dat_d160 = 1;
        break;
      case 0x81:
        v10 = optarg;
      
    }
  }
  if ((((!dat_d134) && (!dat_d133)) && (!dat_d132)) && ((!dat_d131 && (!dat_d130)))) {
    dat_d131 = 1;
    dat_d133 = 1;
    dat_d134 = 1;
  }
  if (v10) { // branch-flip
    if (optind < a0) {
      v9 = sub_69f0(4,a1[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v9);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_4230(1);
    }
    if (strcmp(v10,"-")) { // branch-flip
      v7 = fopen(v10,"r");
      if (!v7) {
        v9 = sub_69f0(4,v10);
        v10 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v10,v9);
        return v8;
      }
    }
    else {
      v7 = stdin;
    }
    v4 = fileno(v7);
    if ((fstat(v4,v17)) || ((v28 & 0xf000) != 0x8000)) {
label_2ad5:
      v5 = sub_45b0(v7);
      if (!v5) {
        sub_8110(); // no-return, return-dupe
      }
      v26 = '\0';
      v24 = (int *)sub_7d50(1,0x98);
    }
    else {
      v22 = dat_99b8;
      if ((double)sub_4bd0() * dat_99c0 <= dat_99b8)
        v22 = dat_99c0 * (double)sub_4bd0();
      if (v22 < (double)v29) goto label_2ad5;
      sub_71f0(&v19);
      v26 = sub_72a0(v7,&v19);
      if ((!v26) || (sub_47e0(v7))) {
        v9 = sub_69f0(4,v10);
        error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v9);
        return v13;
      }
      v5 = sub_4580(v27);
      if (!v5) {
        sub_8110();
      }
      v16 = 1;
      if (v19)
        v16 = v19;
      v24 = (int *)sub_7d50(v16,0x98);
      v16 = v19;
      a1 = v27;
      if (v19) goto label_3024;
    }
    v16 = 0;
    *v24 = 1;
    dat_d12c = 1;
  }
  else {
    if (a0 <= optind) { // branch-flip
      v16 = 1;
      a1 = (char **)0xd110;
    }
    else {
      a1 = &a1[optind];
      v16 = (unsigned long)(a0 - optind);
    }
    v5 = sub_4580(a1);
    if (!v5) {
      sub_8110();
    }
    v26 = '\0';
    v24 = (int *)sub_7d50(v16,0x98);
label_3024:
    if ((v16 != 1) || ((unsigned int)dat_d134 + (unsigned int)dat_d133 + (unsigned int)dat_d132 + (unsigned int)dat_d131 + (unsigned int)dat_d130 != 1)) {
      v20 = 0;
      v12 = &v24[2];
      do {
        v6 = a1[v20];
        if ((v6) && (strcmp(v6,"-")))
          v4 = stat(v6,v12);
        else {
          v4 = fstat(0,v12);
        }
        v20 += 1;
        v12[-2] = v4;
        v12 = &v12[0x26];
      } while (v20 < v16);
      dat_d12c = 1;
      if (*v24 <= 0) {
        v4 = 1;
        v20 = 0;
        v14 = 0;
        v12 = v24;
        do {
          if (!*v12) {
            if ((v12[8] & 0xf000U) != 0x8000) // branch-flip
              v4 = 7;
            else {
              v20 += *(long *)&v12[0xe];
            }
          }
          v14 += 1;
          v12 = &v12[0x26];
        } while (v14 < v16);
        dat_d12c = 1;
        if (10 <= v20) {
          do {
            dat_d12c += 1;
            v14 = v20 / 10;
            v1 = 100 <= v20;
            v20 = v14;
          } while (v1);
        }
        if (dat_d12c <= v4)
          dat_d12c = v4;
      }
    }
    else {
      *v24 = 1;
      dat_d12c = 1;
    }
  }
  v23 = 0;
  v3 = 1;
  while (v6 = (char *)sub_45f0(v5,&v18), v6) {
    if (v10) { // branch-flip
      if (strcmp(v10,"-")) { // branch-flip
        if (*v6) goto label_2c10;
      }
      else if (strcmp(v6,"-")) { // branch-flip
        if (*v6) {
          if (v16) {
            v15 = v23;
            goto label_2c1b;
          }
          v12 = v24;
          goto label_2b6c;
        }
      }
      else {
        sub_69f0(4,v6);
        error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5));
        if (*v6) goto label_2de1;
      }
      v9 = sub_4690(v5);
      v6 = dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s:%lu: %s",sub_6d70(0,3,v10),v9,v6);
      v3 = 0;
    }
    else if (*v6) { // branch-flip
label_2c10:
      v15 = 0;
      if (v16)
        v15 = v23;
label_2c1b:
      v12 = (int *)(v15 + (long)v24);
      if (strcmp(v6,"-")) { // branch-flip
label_2b6c:
        v4 = open(v6,0);
        if (v4 != -1) {
          v25 = sub_3820(v4,v6,v12,0);
          if (!close(v4)) goto label_2bb5;
        }
        sub_6d70(0,3,v6);
        error(0,*__errno_location(),"%s");
        v25 = 0;
      }
      else {
        dat_d128 = '\x01';
        v25 = sub_3820(0,v6,(int *)(v15 + (long)v24),0xffffffffffffffff);
      }
label_2bb5:
      v3 &= v25;
    }
    else {
      dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s");
label_2de1:
      v3 = 0;
    }
    if (!v16)
      *v24 = 1;
    v23 += 0x98;
  }
  if (v18 != 3) {
    if (v18 != 4) { // branch-flip
      if (v18 != 2)
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
      v2 = v10 == NULL & v3;
      if (v2) {
        v3 = v2;
        if (!sub_4690(v5)) {
          dat_d128 = '\x01';
          v3 = sub_3820(0,0,v24,0xffffffffffffffff);
        }
      }
    }
    else {
      v9 = sub_6d70(0,3,v10);
      v10 = dcgettext(NULL,"%s: read error",5);
      error(0,*__errno_location(),v10,v9);
      v3 = 0;
    }
    if (v26)
      sub_7270(&v19);
    if (2 <= (unsigned long)sub_4690(v5))
      sub_3430(dat_d158,dat_d150,dat_d148,dat_d140,dat_d138,dcgettext(NULL,"total",5));
    sub_46b0(v5);
    free(v24);
    if ((dat_d128) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v11;
    }
    if (v30 != *(long *)(v21 + 0x28))
      __stack_chk_fail(); // no-return
    return (unsigned long)(v3 ^ 1);
  }
  sub_8110();
}


// Function: sub_3430 @ 0x3430
void sub_3430(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5) // return-dupe
{
  char *v1;
  unsigned long v2; // rax
  char v3 [24];
  char *v4;
  char *v5;
  
  v4 = "%*s";
  if (dat_d134) {
    v2 = sub_4940(a0,v3);
    v4 = " %*s";
    __printf_chk(1,"%*s",(unsigned long)dat_d12c,v2);
  }
  v5 = v4;
  if (dat_d133) {
    v2 = sub_4940(a1,v3);
    v5 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_d12c,v2);
  }
  v4 = v5;
  if (dat_d132) {
    v2 = sub_4940(a2,v3);
    v4 = " %*s";
    __printf_chk(1,v5,(unsigned long)dat_d12c,v2);
  }
  v5 = v4;
  if (dat_d131) {
    v2 = sub_4940(a3,v3);
    v5 = " %*s";
    __printf_chk(1,v4,(unsigned long)dat_d12c,v2);
  }
  if (dat_d130) {
    v2 = sub_4940(a4,v3);
    __printf_chk(1,v5,(unsigned long)dat_d12c,v2);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_6d70(0,3,a5);
    __printf_chk(1," %s",a5);
  }
  v1 = *(char **)((long)stdout + 0x28);
  if (v1 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10);
}


// Function: sub_3820 @ 0x3820
unsigned int sub_3820(int a0,char *a1,int *a2,long a3) // ternary
{
  unsigned short v1;
  unsigned int v10; // stack - 0x4064
  long v11; // stack - 0x4060
  unsigned long v12;
  char *v13; // rsi
  unsigned char v14;
  long v15;
  unsigned long v16;
  unsigned char *v17;
  unsigned int v18;
  unsigned long v19; // r13
  unsigned long v2;
  unsigned char *v20;
  unsigned int v21;
  bool v22; // r9b
  long v23; // stack - 0x40a0
  unsigned long v24; // stack - 0x4090
  unsigned char v25; // stack - 0x4079
  char *v26; // stack - 0x4070
  unsigned char v3; // al
  unsigned int v4; // eax
  void *v5;
  unsigned long v6;
  unsigned long v7;
  unsigned char v8 [16392];
  unsigned long v9 [2]; // stack - 0x4058
  
  v26 = a1;
  if (!a1) {
    v5 = (char *)dcgettext(NULL,"standard input",5);
    v26 = (char *)v5;
  }
  v9[0] = 0;
  v11 = 0;
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  v14 = dat_d131; // branch-flip
  if (2 <= v5) {
    v3 = dat_d132;
    v25 = v3;
  }
  else {
    v25 = 0;
    if (!dat_d131)
      v14 = dat_d132;
  }
  if ((dat_d133) || (dat_d130))
    sub_47a0(a0,0,0,2);
  else {
    if ((v14 == 1) && (!v25)) {
      if (dat_d134) {
        sub_47a0(a0,0,0,2);
label_40e0:
        if (dat_d134) {
label_40f0:
          if (*(int *)cpuid_basic_info(0)) { // branch-flip
            if (!(*(unsigned int *)(cpuid_Version_info(1) + 0xc) & 0x8000000)) {
label_4120:
              v13 = "avx2 support not detected";
              if (dat_d160) goto label_417a;
              goto label_4135;
            }
            if (*(unsigned int *)cpuid_basic_info(0) <= 6) goto label_4165;
            if (!(*(unsigned int *)(cpuid_Extended_Feature_Enumeration_info(7) + 4) & 0x20)) goto label_4120;
            if (dat_d160) {
              v5 = (char *)dcgettext(NULL,"using avx2 hardware support",5);
              error(0,0,"%s",v5);
            }
            v5 = sub_86a0;
            dat_d010 = sub_86a0;
          }
          else {
label_4165:
            if (dat_d160) {
              v13 = "failed to get cpuid";
label_417a:
              v5 = (char *)dcgettext(NULL,v13,5);
              error(0,0,"%s",v5);
            }
label_4135:
            v5 = dat_d010;
          }
          v12 = 0;
          v16 = 0;
          v23 = 0;
          v4 = (*v5)(v26,a0,&v11,v9);
          v7 = v9[0];
          v21 = v4;
          goto label_39f8;
        }
      }
      v4 = *a2;
      if (0 < (int)v4) {
        v4 = fstat(a0,&a2[2]);
        *a2 = v4;
      }
      v16 = dat_d120;
      if (((!v4) && (v4 = a2[8] & 0xd000, v4 == 0x8000)) && (v7 = *(unsigned long *)&a2[0xe], 0 <= (long)v7)) {
        if (a3 != -1) { // branch-flip
          if (v7 % dat_d120) {
label_40b7:
            v12 = 0;
            v16 = 0;
            v23 = 0;
            v9[0] = v7;
            v21 = 1;
            goto label_39f8;
          }
          v14 = 1;
          v16 = 0;
        }
        else {
          v5 = (unsigned long)lseek(a0,0,1);
          if (v7 % v16) {
            v7 = (v5 <= v7) ? v7 - (long)v5 : 0; // branch-flip
            goto label_40b7;
          }
          v14 = (unsigned char)~(unsigned char)((unsigned long)v5 >> 0x38) >> 7;
          v16 = (unsigned long)v5;
        }
        v5 = (unsigned long)(*(long *)&a2[0x10] + 1);
        v12 = 0x201;
        if ((unsigned long)(*(long *)&a2[0x10] - 1U) <= 0x1fffffffffffffff)
          v12 = (unsigned long)v5;
        v15 = v7 - v7 % v12;
        if ((((long)v16 < v15) && (v14)) && (v5 = (long)lseek(a0,v15,1), 0 <= (long)v5))
          v9[0] = v15 - v16;
      }
      sub_47a0(a0,0,0,2);
      while (v5 = (long)sub_7490(a0,v8,0x4000), v5) {
        if (v5 == (void *)0xffffffffffffffff) {
          v12 = 0;
          v23 = 0;
          v5 = (unsigned long)sub_6d70(0,3,v26);
          v2 = v5;
          v5 = (int *)__errno_location();
          v16 = 0;
          error(0,*(int *)v5,"%s",v2);
          v7 = v9[0];
          v21 = 0;
          goto label_39f8;
        }
        v9[0] += (long)v5;
      }
      v12 = 0;
      v23 = 0;
      v16 = 0;
      v7 = v9[0];
      v21 = 1;
      goto label_39f8;
    }
    sub_47a0(a0,0,0,2);
    if (v25 != 1) {
      if (v14) goto label_40e0;
      if (!v25) goto label_40f0;
    }
  }
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v5) { // branch-flip
    v19 = 0;
    v22 = 0;
    v12 = 0;
    v9[1] = 0;
    v7 = 0;
    v16 = 0;
    v24 = 0;
    v23 = 0;
label_3ae0:
    v5 = (long)sub_7490(a0,&v8[v19],0x4000 - v19);
    if (v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v5 = (unsigned long)sub_6d70(0,3,v26);
        v2 = v5;
        v5 = (int *)__errno_location();
        v21 = 0;
        error(0,*(int *)v5,"%s",v2);
        goto label_3f68;
      }
      v9[0] += (long)v5;
      v19 += (long)v5;
      v17 = v8;
      do {
        v6 = v9[1];
        if (v22) {
label_3c30:
          v5 = (long)sub_4a50(&v10,v17,v19,&v9[1]);
          v15 = (long)v5;
          if (v5 != (void *)0xfffffffffffffffe) {
            if (v5 != (void *)0xffffffffffffffff) {
              v4 = mbsinit(&v9[1]);
              v22 = v4 == 0;
              if (v15) { // branch-flip
                switch(v10) {
                  case 9:
                    goto label_3c26;
                  case 10:
                    goto label_3c16;
                  case 0xb:
                    goto label_3be9;
                  case 0xc:
                  case 0xd:
                    goto label_3bd6;
                  default:
                    v18 = v10;
                    break;
                  case 0x20:
                    goto label_3c06;
                  
                }
              }
              else {
                v10 = 0;
                v15 = 1;
                v18 = 0;
              }
              v4 = iswprint(v18);
              if (v4) {
                if (dat_d130) {
                  v4 = wcwidth(v10);
                  v6 = (long)(int)v4 + v12;
                  if (1 <= (int)v4)
                    v12 = v6;
                }
                v4 = iswspace(v10);
                if (v4) goto label_3be9;
                if (dat_d118) // branch-flip
                  v7 = (unsigned long)dat_d118;
                else {
                  if ((((v10 == 0xa0) || (v10 == 0x2007)) || (v10 == 0x202f)) || (v10 == 0x2060)) goto label_3be9;
                  v7 = 1;
                }
              }
              goto label_3ba8;
            }
            v17 = &v17[1];
            v15 = -1;
            v22 = 1;
            goto label_3bb2;
          }
          v9[1] = v6;
          if (v19) {
            if (v19 == 0x4000) {
              v17 = &v17[1];
              v19 = 0x3fff;
            }
            __memmove_chk(v8,v17,v19,0x4001);
          }
          v22 = 1;
          break;
        }
        v14 = *v17;
        v3 = v14 >> 5;
        v5 = (unsigned long)(unsigned long)v3;
        v4 = *(unsigned int *)((long)v5 * 4 + 0x99e0) >> (v14 & 0x1f);
        v18 = v4 & 1;
        if (!v18) goto label_3c30;
        v4 = (unsigned int)(char)v14;
        v10 = v4;
        switch(v14) {
          case 9:
            v15 = 1;
label_3c26:
            v12 = (v12 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v15 = 1;
label_3c16:
            v11 += 1;
            goto label_3bd6;
          case 0xb:
label_3bf8:
            v15 = 1;
            break;
          case 0xc:
          case 0xd:
            v15 = 1;
label_3bd6:
            v6 = v12;
            if (v12 <= v24)
              v6 = v24;
            v12 = 0;
            v24 = v6;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v14 * 2);
            if (v1 & 0x4000) { // branch-flip
              v12 += 1;
              if (v1 & 0x2000) goto label_3bf8;
              v7 = (unsigned long)v18;
              v15 = 1;
            }
            else {
              v15 = 1;
            }
            goto label_3ba8;
          case 0x20:
            v15 = 1;
label_3c06:
            v12 += 1;
          
        }
label_3be9:
        v23 += v7;
        v7 = 0;
label_3ba8:
        v17 = &v17[v15];
        v15 = -v15;
        v16 += 1;
label_3bb2:
        v19 += v15;
      } while (v19);
      goto label_3ae0;
    }
    v21 = 1;
label_3f68:
    if (v12 <= v24)
      v12 = v24;
    v23 = v7 + v23;
    v7 = v9[0];
  }
  else {
    v12 = 0;
    v19 = 0;
    v7 = 0;
    v16 = v12;
    while (v5 = (long)sub_7490(a0,v8,0x4000), v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v21 = 0;
        v5 = (unsigned long)sub_6d70(0,3,v26);
        v2 = v5;
        v5 = (int *)__errno_location();
        error(0,*(int *)v5,"%s",v2);
        goto label_3fc8;
      }
      v9[0] += (long)v5;
      v17 = &v8[(long)v5];
      v20 = v8;
      do {
        v14 = *v20;
        v20 = &v20[1];
        switch(v14) {
          case 9:
            v19 = (v19 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v11 += 1;
            goto label_3ee6;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_3ee6:
            if (v12 < v19)
              v12 = v19;
            v19 = 0;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v14 * 2);
            if (v1 & 0x4000) {
              v19 += 1;
              if (v1 & 0x2000) goto label_3ef0;
              v4 = btowc((unsigned long)v14);
              if (dat_d118) // branch-flip
                v7 = (unsigned long)dat_d118;
              else {
                if (((v4 == 0xa0) || (v4 == 0x2007)) || ((v4 == 0x202f || (v4 == 0x2060)))) goto label_3ef0;
                v7 = 1;
              }
            }
            goto label_3ed0;
          case 0x20:
            v19 += 1;
          
        }
label_3ef0:
        v16 += v7;
        v7 = 0;
label_3ed0:
      } while (v17 != v20);
    }
    v21 = 1;
label_3fc8:
    if (v12 < v19)
      v12 = v19;
    v23 = v7 + v16;
    v16 = 0;
    v7 = v9[0];
  }
label_39f8:
  if (v25 < dat_d132)
    v16 = v7;
  sub_3430(v11,v23,v16,v7,v12,a1);
  dat_d150 += v23;
  dat_d158 += v11;
  dat_d148 += v16;
  dat_d140 += v9[0];
  if (dat_d138 < v12)
    dat_d138 = v12;
  v4 = v21;
  return v4;
}


// Function: sub_4230 @ 0x4230
void sub_4230(int a0)
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
  
  v2 = dat_d178;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n",5),v1);
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
      if (!strcmp("wc",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "wc";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "wc";
    if (!strcmp("wc","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "wc")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}


// Function: sub_86a0 @ 0x86a0
unsigned long sub_86a0(unsigned long a0,unsigned int a1,long *a2,long *a3)
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
    while (v2 = sub_7490(a1,(char (*)[32])v7,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = sub_6d70(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v14 & 0xffffffff; // return-dupe
      }
      v6 += v2;
      if (0x40 <= v2) { // branch-flip
        v18 = ZEXT1664(0);
        v17 = SUB6432(v18,0);
        v5 = (char (*)[32])v7;
        do {
          v16 = vpcmpeqb_avx2(dat_a1c0,*v5);
          v1 = vpcmpeqb_avx2(dat_a1c0,v5[1]);
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

