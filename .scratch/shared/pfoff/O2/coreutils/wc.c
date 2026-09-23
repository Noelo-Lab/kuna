// Function: main @ 0x2860
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char *v10;
  FILE *v11;
  char *v12; // rax
  unsigned long v13;
  unsigned long v14; // rax
  int *v15;
  unsigned long v16; // rcx
  long v17;
  unsigned long v18;
  stat v19; // stack - 0xd8
  unsigned char v2; // al
  int v20; // stack - 0x1fc
  unsigned long v21; // stack - 0x1f8
  long v22; // fs_offset
  double v23;
  long v24; // stack - 0x230
  int *v25; // stack - 0x220
  unsigned char v26; // stack - 0x218
  bool v27; // stack - 0x20d
  char **v28; // stack - 0x1f0
  long v29; // stack - 0x40
  unsigned char v3;
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  char *v8;
  struct_3 *v9; // rax
  
  v29 = *(long *)(v22 + 0x28);
  sub_4d10(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_88e0(sub_46f0);
  dat_d120 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  v8 = getenv("POSIXLY_CORRECT");
  dat_d131 = 0;
  dat_d132 = 0;
  dat_d118 = v8 != NULL;
  dat_d133 = 0;
  dat_d134 = 0;
  dat_d130 = 0;
  dat_d138 = 0;
  dat_d140 = 0;
  dat_d148 = 0;
  dat_d150 = 0;
  dat_d158 = 0;
  v8 = NULL;
  while( true ) {
    v14 = 0;
    v4 = getopt_long(argc,argv,"clLmw",(option *)0xca20,NULL);
    if (v4 == -1) break;
    if (0x81 < v4) {
      sub_4230(1); // return-dupe, no-return
    }
    if (v4 <= 0x4b) {
      if (v4 == -0x83) {
        sub_7a40(stdout,"wc","GNU coreutils",dat_d018,"Paul Rubin","David MacKenzie",0,v14);
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
        v8 = optarg;
      
    }
  }
  if ((((!dat_d134) && (!dat_d133)) && (!dat_d132)) && ((!dat_d131 && (!dat_d130)))) {
    dat_d131 = 1;
    dat_d133 = 1;
    dat_d134 = 1;
  }
  if (v8) { // branch-flip
    if (optind < argc) {
      v8 = (char *)sub_69f0(4,argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v8);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
label_3299:
      sub_4230(1);
    }
    if (strcmp(v8,"-")) { // branch-flip
      v11 = fopen(v8,"r");
      if (!v11) {
        v14 = sub_69f0(4,v8);
        v8 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v8,v14);
        return v5;
      }
    }
    else {
      v11 = stdin;
    }
    v4 = fileno(v11);
    if ((fstat(v4,&v19)) || ((v19._24_4_ & 0xf000) != 0x8000)) {
label_2ad5:
      v9 = (struct_3 *)sub_45b0(v11);
      if (!v9) {
        sub_8110(); // no-return, return-dupe
      }
      v25 = (int *)sub_7d50(1,0x98);
      v27 = 0;
    }
    else {
      v23 = dat_99b8;
      if ((double)sub_4bd0() * dat_99c0 <= dat_99b8)
        v23 = dat_99c0 * (double)sub_4bd0();
      if (v23 < (double)(long)v19._48_8_) goto label_2ad5;
      sub_71f0(&v21);
      v27 = sub_72a0(v11,&v21);
      if ((!v27) || (sub_47e0(v11))) {
        v8 = (char *)sub_69f0(4,v8);
        error(1,0,dcgettext(NULL,"cannot read file names from %s",5),v8);
        return v7;
      }
      v9 = (struct_3 *)sub_4580(v28);
      if (!v9) {
        sub_8110();
      }
      v18 = 1;
      if (v21)
        v18 = v21;
      v25 = (int *)sub_7d50(v18,0x98);
      v18 = v21;
      argv = v28;
      if (v21) goto label_3024;
    }
    v18 = 0;
    *v25 = 1;
    dat_d12c = 1;
  }
  else {
    if (argc <= optind) { // branch-flip
      v18 = 1;
      argv = (char **)0xd110;
    }
    else {
      argv = &argv[optind];
      v18 = (unsigned long)(argc - optind);
    }
    v9 = (struct_3 *)sub_4580(argv);
    if (!v9) {
      sub_8110();
    }
    v25 = (int *)sub_7d50(v18,0x98);
    v27 = 0;
label_3024:
    if ((v18 != 1) || ((unsigned int)dat_d134 + (unsigned int)dat_d133 + (unsigned int)dat_d132 + (unsigned int)dat_d131 + (unsigned int)dat_d130 != 1)) {
      v13 = 0;
      v15 = &v25[2];
      do {
        v10 = argv[v13];
        if ((v10) && (strcmp(v10,"-")))
          v4 = stat(v10,(stat *)v15);
        else {
          v4 = fstat(0,(stat *)v15);
        }
        v13 += 1;
        v15[-2] = v4;
        v15 = &v15[0x26];
      } while (v13 < v18);
      dat_d12c = 1;
      if (*v25 <= 0) {
        v4 = 1;
        v13 = 0;
        v16 = 0;
        v15 = v25;
        do {
          if (!*v15) {
            if ((v15[8] & 0xf000U) != 0x8000) // branch-flip
              v4 = 7;
            else {
              v13 += *(long *)&v15[0xe];
            }
          }
          v16 += 1;
          v15 = &v15[0x26];
        } while (v16 < v18);
        dat_d12c = 1;
        if (10 <= v13) {
          do {
            dat_d12c += 1;
            v16 = v13 / 10;
            v1 = 100 <= v13;
            v13 = v16;
          } while (v1);
        }
        if (dat_d12c <= v4)
          dat_d12c = v4;
      }
    }
    else {
      *v25 = 1;
      dat_d12c = 1;
    }
  }
  v24 = 0;
  v3 = 1;
  while (v10 = (char *)sub_45f0(v9,&v20), v10) {
    if (v8) { // branch-flip
      if (strcmp(v8,"-")) { // branch-flip
        if (*v10) goto label_2c10;
      }
      else if (strcmp(v10,"-")) { // branch-flip
        if (*v10) {
          if (v18) {
            v17 = v24;
            goto label_2c1b;
          }
          v15 = v25;
          goto label_2b6c;
        }
      }
      else {
        v12 = (char *)sub_69f0(4,v10);
        error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v12);
        if (*v10) goto label_2de1;
      }
      v13 = sub_4690(v9);
      v10 = dcgettext(NULL,"invalid zero-length file name",5);
      error(0,0,"%s:%lu: %s",(char *)sub_6d70(0,3,v8),v13,v10);
      v3 = 0;
    }
    else if (*v10) { // branch-flip
label_2c10:
      v17 = 0;
      if (v18)
        v17 = v24;
label_2c1b:
      v15 = (int *)(v17 + (long)v25);
      if (strcmp(v10,"-")) { // branch-flip
label_2b6c:
        v4 = open(v10,0);
        if (v4 != -1) {
          v26 = sub_3820(v4,v10,v15,0);
          v4 = close(v4);
          if (!v4) goto label_2bb5;
        }
        v10 = (char *)sub_6d70(0,3,v10);
        error(0,*__errno_location(),"%s",v10);
        v26 = 0;
      }
      else {
        dat_d128 = '\x01';
        v26 = sub_3820(0,v10,(int *)(v17 + (long)v25),-1);
      }
label_2bb5:
      v3 &= v26;
    }
    else {
      error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
label_2de1:
      v3 = 0;
    }
    if (!v18)
      *v25 = 1;
    v24 += 0x98;
  }
  if (v20 != 3) {
    if (v20 != 4) { // branch-flip
      if (v20 != 2)
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
      v2 = v8 == NULL & v3;
      if (v2) {
        v3 = v2;
        if (!sub_4690(v9)) {
          dat_d128 = '\x01';
          v3 = sub_3820(0,NULL,v25,-1);
        }
      }
    }
    else {
      v14 = sub_6d70(0,3,v8);
      v8 = dcgettext(NULL,"%s: read error",5);
      error(0,*__errno_location(),v8,v14);
      v3 = 0;
    }
    if (v27)
      sub_7270(&v21);
    if (2 <= (unsigned long)sub_4690(v9))
      sub_3430(dat_d158,dat_d150,dat_d148,dat_d140,dat_d138,dcgettext(NULL,"total",5));
    sub_46b0(v9);
    free(v25);
    if ((dat_d128) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v6;
    }
    if (v29 != *(long *)(v22 + 0x28))
      __stack_chk_fail(); // no-return
    return (int)(v3 ^ 1);
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
  v1 = *(char **)&stdout->field_0x28;
  if (v1 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v1[1];
    *v1 = 10;
    return;
  }
  __overflow(stdout,10);
}


// Function: sub_3820 @ 0x3820
unsigned int sub_3820(int a0,char *a1,struct_1 *a2,long a3) // ternary
{
  unsigned short v1;
  mbstate_t v10; // stack - 0x4050
  unsigned int v11; // stack - 0x4064
  unsigned long v12; // stack - 0x4058
  long v13; // stack - 0x4060
  unsigned long v14;
  char *v15;
  unsigned char v16;
  long v17;
  unsigned long v18;
  unsigned char *v19;
  mbstate_t v2;
  unsigned int v20;
  unsigned char *v21;
  unsigned int v22;
  bool v23; // sil
  long v24; // stack - 0x40a0
  unsigned long v25; // stack - 0x4090
  unsigned char v26; // stack - 0x4079
  char *v27; // stack - 0x4070
  unsigned char v3; // al
  unsigned int v4; // eax
  void *v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8; // rbx
  unsigned char v9 [16392];
  
  v27 = a1;
  if (!a1) {
    v5 = (char *)dcgettext(NULL,"standard input",5);
    v27 = (char *)v5;
  }
  v12 = 0;
  v13 = 0;
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  v16 = dat_d131; // branch-flip
  if (2 <= v5) {
    v3 = dat_d132;
    v26 = v3;
  }
  else {
    v26 = 0;
    if (!dat_d131)
      v16 = dat_d132;
  }
  if ((dat_d133) || (dat_d130))
    sub_47a0(a0,0,0,2);
  else {
    if ((v16 == 1) && (!v26)) {
      if (dat_d134) {
        sub_47a0(a0,0,0,2);
label_40e0:
        if (dat_d134) {
label_40f0:
          if (*(int *)cpuid_basic_info(0)) { // branch-flip
            if (!(*(unsigned int *)(cpuid_Version_info(1) + 0xc) & 0x8000000)) {
label_4120:
              v15 = "avx2 support not detected";
              if (dat_d160) goto label_417a;
              goto label_4135;
            }
            if (*(unsigned int *)cpuid_basic_info(0) <= 6) goto label_4165;
            if (!(*(unsigned int *)(cpuid_Extended_Feature_Enumeration_info(7) + 4) & 0x20)) goto label_4120;
            if (dat_d160) {
              v5 = (char *)dcgettext(NULL,"using avx2 hardware support",5);
              error(0,0,"%s",(char *)v5);
            }
            v5 = sub_86a0;
            dat_d010 = sub_86a0;
          }
          else {
label_4165:
            if (dat_d160) {
              v15 = "failed to get cpuid";
label_417a:
              v5 = (char *)dcgettext(NULL,v15,5);
              error(0,0,"%s",(char *)v5);
            }
label_4135:
            v5 = dat_d010;
          }
          v14 = 0;
          v18 = 0;
          v8 = 0;
          v4 = (*v5)(v27,a0,&v13,&v12);
          v7 = v12;
          v22 = v4;
          goto label_39f8;
        }
      }
      v4 = a2->field_0x0;
      if (0 < (int)v4) {
        v4 = fstat(a0,(stat *)&a2->field_0x4[4]);
        a2->field_0x0 = v4;
      }
      v18 = dat_d120;
      if (((!v4) && (v4 = a2->field_0x20 & 0xd000, v4 == 0x8000)) && (v7 = a2->field_0x38, 0 <= (long)v7)) {
        if (a3 != -1) { // branch-flip
          if (v7 % dat_d120) {
label_40b7:
            v14 = 0;
            v18 = 0;
            v8 = 0;
            v12 = v7;
            v22 = 1;
            goto label_39f8;
          }
          v23 = 1;
          v18 = 0;
        }
        else {
          v5 = (unsigned long)lseek(a0,0,1);
          if (v7 % v18) {
            v7 = (v5 <= v7) ? v7 - (long)v5 : 0; // branch-flip
            goto label_40b7;
          }
          v23 = (bool)((unsigned char)~(unsigned char)((unsigned long)v5 >> 0x38) >> 7);
          v18 = (unsigned long)v5;
        }
        v5 = (unsigned long)(a2->field_0x40 + 1);
        v14 = 0x201;
        if ((unsigned long)(a2->field_0x40 - 1U) <= 0x1fffffffffffffff)
          v14 = (unsigned long)v5;
        v17 = v7 - v7 % v14;
        if ((((long)v18 < v17) && (v23)) && (v5 = (long)lseek(a0,v17,1), 0 <= (long)v5))
          v12 = v17 - v18;
      }
      sub_47a0(a0,0,0,2);
      while (v5 = (long)sub_7490(a0,v9,0x4000), v5) {
        if (v5 == (void *)0xffffffffffffffff) {
          v14 = 0;
          v8 = 0;
          v5 = (char *)sub_6d70(0,3,v27);
          v15 = (char *)v5;
          v5 = (int *)__errno_location();
          v18 = 0;
          error(0,*(int *)v5,"%s",v15);
          v7 = v12;
          v22 = 0;
          goto label_39f8;
        }
        v12 += (long)v5;
      }
      v14 = 0;
      v8 = 0;
      v18 = 0;
      v7 = v12;
      v22 = 1;
      goto label_39f8;
    }
    sub_47a0(a0,0,0,2);
    if (v26 != 1) {
      if (v16) goto label_40e0;
      if (!v26) goto label_40f0;
    }
  }
  v5 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v5) { // branch-flip
    v7 = 0;
    v23 = 0;
    v14 = 0;
    v10 = 0;
    v8 = 0;
    v18 = 0;
    v25 = 0;
    v24 = 0;
label_3ae0:
    v5 = (long)sub_7490(a0,&v9[v7],0x4000 - v7);
    if (v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v5 = (char *)sub_6d70(0,3,v27);
        v15 = (char *)v5;
        v5 = (int *)__errno_location();
        v22 = 0;
        error(0,*(int *)v5,"%s",v15);
        goto label_3f68;
      }
      v12 += (long)v5;
      v7 += (long)v5;
      v19 = v9;
      do {
        v2 = v10;
        if (v23) {
label_3c30:
          v5 = (long)sub_4a50(&v11,v19,v7,&v10);
          v17 = (long)v5;
          if (v5 != (void *)0xfffffffffffffffe) {
            if (v5 != (void *)0xffffffffffffffff) {
              v4 = mbsinit(&v10);
              v23 = v4 == 0;
              if (v17) { // branch-flip
                switch(v11) {
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
                    v20 = v11;
                    break;
                  case 0x20:
                    goto label_3c06;
                  
                }
              }
              else {
                v11 = 0;
                v17 = 1;
                v20 = 0;
              }
              v4 = iswprint(v20);
              if (v4) {
                if (dat_d130) {
                  v4 = wcwidth(v11);
                  v6 = (long)(int)v4 + v14;
                  if (1 <= (int)v4)
                    v14 = v6;
                }
                v4 = iswspace(v11);
                if (v4) goto label_3be9;
                if (dat_d118) // branch-flip
                  v8 = (unsigned long)dat_d118;
                else {
                  if ((((v11 == 0xa0) || (v11 == 0x2007)) || (v11 == 0x202f)) || (v11 == 0x2060)) goto label_3be9;
                  v8 = 1;
                }
              }
              goto label_3ba8;
            }
            v19 = &v19[1];
            v17 = -1;
            v23 = 1;
            goto label_3bb2;
          }
          v10 = v2;
          if (v7) {
            if (v7 == 0x4000) {
              v19 = &v19[1];
              v7 = 0x3fff;
            }
            __memmove_chk(v9,v19,v7,0x4001);
          }
          v23 = 1;
          break;
        }
        v16 = *v19;
        v3 = v16 >> 5;
        v5 = (unsigned long)(unsigned long)v3;
        v4 = *(unsigned int *)((long)v5 * 4 + 0x99e0) >> (v16 & 0x1f);
        v20 = v4 & 1;
        if (!v20) goto label_3c30;
        v4 = (unsigned int)(char)v16;
        v11 = v4;
        switch(v16) {
          case 9:
            v17 = 1;
label_3c26:
            v14 = (v14 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v17 = 1;
label_3c16:
            v13 += 1;
            goto label_3bd6;
          case 0xb:
label_3bf8:
            v17 = 1;
            break;
          case 0xc:
          case 0xd:
            v17 = 1;
label_3bd6:
            v6 = v14;
            if (v14 <= v25)
              v6 = v25;
            v14 = 0;
            v25 = v6;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v16 * 2);
            if (v1 & 0x4000) { // branch-flip
              v14 += 1;
              if (v1 & 0x2000) goto label_3bf8;
              v8 = (unsigned long)v20;
              v17 = 1;
            }
            else {
              v17 = 1;
            }
            goto label_3ba8;
          case 0x20:
            v17 = 1;
label_3c06:
            v14 += 1;
          
        }
label_3be9:
        v24 += v8;
        v8 = 0;
label_3ba8:
        v19 = &v19[v17];
        v17 = -v17;
        v18 += 1;
label_3bb2:
        v7 += v17;
      } while (v7);
      goto label_3ae0;
    }
    v22 = 1;
label_3f68:
    if (v14 <= v25)
      v14 = v25;
    v8 += v24;
    v7 = v12;
  }
  else {
    v14 = 0;
    v7 = 0;
    v8 = 0;
    v18 = v14;
    while (v5 = (long)sub_7490(a0,v9,0x4000), v5) {
      if (v5 == (void *)0xffffffffffffffff) {
        v22 = 0;
        v5 = (char *)sub_6d70(0,3,v27);
        v15 = (char *)v5;
        v5 = (int *)__errno_location();
        error(0,*(int *)v5,"%s",v15);
        goto label_3fc8;
      }
      v12 += (long)v5;
      v19 = &v9[(long)v5];
      v21 = v9;
      do {
        v16 = *v21;
        v21 = &v21[1];
        switch(v16) {
          case 9:
            v7 = (v7 & 0xfffffffffffffff8) + 8;
            break;
          case 10:
            v13 += 1;
            goto label_3ee6;
          case 0xb:
            break;
          case 0xc:
          case 0xd:
label_3ee6:
            if (v14 < v7)
              v14 = v7;
            v7 = 0;
            break;
          default:
            v5 = __ctype_b_loc();
            v1 = *(unsigned short *)(*(long *)v5 + (unsigned long)v16 * 2);
            if (v1 & 0x4000) {
              v7 += 1;
              if (v1 & 0x2000) goto label_3ef0;
              v4 = btowc((unsigned int)v16);
              if (dat_d118) // branch-flip
                v8 = (unsigned long)dat_d118;
              else {
                if (((v4 == 0xa0) || (v4 == 0x2007)) || ((v4 == 0x202f || (v4 == 0x2060)))) goto label_3ef0;
                v8 = 1;
              }
            }
            goto label_3ed0;
          case 0x20:
            v7 += 1;
          
        }
label_3ef0:
        v18 += v8;
        v8 = 0;
label_3ed0:
      } while (v19 != v21);
    }
    v22 = 1;
label_3fc8:
    if (v14 < v7)
      v14 = v7;
    v8 += v18;
    v18 = 0;
    v7 = v12;
  }
label_39f8:
  if (v26 < dat_d132)
    v18 = v7;
  sub_3430(v13,v8,v18,v7,v14,a1);
  dat_d150 += v8;
  dat_d158 += v13;
  dat_d148 += v18;
  dat_d140 += v12;
  if (dat_d138 < v14)
    dat_d138 = v14;
  v4 = v22;
  return v4;
}


// Function: sub_4230 @ 0x4230
void sub_4230(int a0)
{
  FILE *v1;
  char *v10; // stack - 0x88
  char *v11; // stack - 0x80
  char *v12; // stack - 0x78
  char *v13; // stack - 0x70
  char *v14; // stack - 0x68
  char *v15; // stack - 0x60
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x50
  unsigned long v18; // stack - 0x40
  char *v19 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // r13
  long v8; // fs_offset
  char *v9; // stack - 0xb0
  
  v6 = dat_d178;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v6,v6);
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
    v5 = "[";
    v6 = "[";
    v9 = "test invocation";
    v19[0] = "coreutils";
    v19[1] = "Multi-call invocation";
    v10 = "sha256sum";
    v19[2] = "sha224sum";
    v12 = "sha384sum";
    v19[3] = "sha2 utilities";
    v11 = "sha2 utilities";
    v13 = "sha2 utilities";
    v14 = "sha512sum";
    v15 = "sha2 utilities";
    v16 = 0;
    v17 = 0;
    do {
      if (!strcmp("wc",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "wc";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "wc";
    if (!strcmp("wc","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "wc")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}


// Function: sub_86a0 @ 0x86a0
unsigned long sub_86a0(char *a0,unsigned int a1,long *a2,long *a3)
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
    while (v2 = sub_7490(a1,(char (*)[32])v8,0x3fc0), v2) {
      if (v2 == 0xffffffffffffffff) {
        v4 = (char *)sub_6d70(0,3,a0);
        error(0,*__errno_location(),"%s",v4);
        return v18 & 0xffffffff; // return-dupe
      }
      v7 += v2;
      if (0x40 <= v2) { // branch-flip
        v22 = ZEXT1664(0);
        v21 = SUB6432(v22,0);
        v6 = (char (*)[32])v8;
        do {
          v20 = vpcmpeqb_avx2(dat_a1c0,*v6);
          v1 = vpcmpeqb_avx2(dat_a1c0,v6[1]);
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

