// Function: main @ 0x4ff0
int main(int argc,char **argv,char **envp)
{
  unsigned int *v1;
  int v10;
  int v11; // eax
  int v12; // eax
  char *v13;
  FILE *v14;
  void *v15; // rax
  unsigned long v16;
  void *v17;
  char *v18;
  char *v19;
  char v2;
  long v20; // rax
  long v21;
  long *v22;
  char *v23;
  long v24; // stack - 0x1a8
  char v25; // stack - 0x68
  long v26; // stack - 0x1a0
  char v27 [3];
  char v28 [16];
  char v29 [8];
  int v3;
  char v30; // stack - 0x67
  stat v31; // stack - 0x188
  long *v32; // stack - 0x1b0
  stat v33; // stack - 0xf8
  mbstate_t v34; // stack - 0x190
  unsigned int v35; // eax
  stat *v36;
  unsigned int *v37;
  unsigned int v38; // eax
  long *v39;
  char v4 [16];
  char *v40; // r12
  int *v41; // rax
  char *v42;
  unsigned char *v43; // r15
  char *v44;
  unsigned char *v45;
  int v46; // stack - 0x1f8
  int v47; // stack - 0x1e4
  unsigned char *v48; // stack - 0x1e0
  long *v49; // stack - 0x1d8
  bool v5;
  int v50; // stack - 0x1d0
  unsigned int v51; // stack - 0x1cc
  char v52; // stack - 0x66
  unsigned int v6;
  unsigned char v7;
  char v8; // al
  int v9;
  
  dat_2b684 = 10;
  v32 = NULL;
  v24 = 0;
  dat_2b080 = 2;
  dat_2b804 = 0xffffffff;
  dat_2b7e0 = 0x7fffffffffffffff;
  dat_2b7f8 = -1;
  dat_2b7f0 = -1;
  v26 = -1;
  dat_2b8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_16210(0x2b180);
  sub_20120(sub_7a00);
  sub_f6a0(NULL);
  dat_2b8c8 = sub_1a2d0(0,0,sub_7850,sub_78a0,0);
  if (!dat_2b8c8)
    sub_1f410(); // no-return
  v49 = &v24;
  v51 = 0;
  v50 = 0;
  v47 = -1;
  v48 = NULL;
label_5148:
  v9 = optind;
  v5 = 0;
  v18 = &v25;
  while( true ) {
    v10 = optind;
    v46 = getopt_long(argc,argv,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(option *)0x29fc0,NULL);
    if ((unsigned int)(v46 - 0x30U) > 9) break;
    if ((dat_2b010 != v10) || (!v5))
      v13 = &v25;
    else {
      v13 = &v18[-(unsigned long)(v25 == '0')];
      if (v13 == v27) {
        builtin_strncpy(&v18[-(unsigned long)(v25 == '0')],"...",4);
        v18 = v28;
        goto label_51b5;
      }
    }
    dat_2b010 = v10;
    v18 = &v13[1];
    v5 = 1;
    *v13 = (char)v46;
  }
  if (v18 != &v25) {
label_51b5:
    *v18 = '\0';
    sub_7d10(&v25,&v26);
  }
  v18 = optarg;
  if (v46 != -1) {
    switch(v46) {
      case 0:
      case 0x55:
        break;
      default:
        sub_a170(2); // no-return, return-dupe
      case 0x41:
        sub_7d10(optarg,(long *)0x2b7f0);
        break;
      case 0x42:
        sub_7d10(optarg,(long *)0x2b7f8);
        break;
      case 0x43:
        sub_7d10(optarg,&v26);
        break;
      case 0x44:
        if (strcmp(optarg,"read")) { // branch-flip
label_5a31:
          if (strcmp(v18,"skip")) {
            error(2,0,dcgettext(NULL,"unknown devices method",5));
            goto label_666b;
          }
          dat_2b884 = 2;
        }
        else {
          dat_2b884 = 1;
        }
        break;
      case 0x45:
        v47 = sub_7950("egrep",v47);
        break;
      case 0x46:
        v47 = sub_7950("fgrep",v47);
        break;
      case 0x47:
        v47 = sub_7950("grep",v47);
        break;
      case 0x48:
        v51 = 1;
        break;
      case 0x49:
label_52b7:
        dat_2b810 = 2;
        break;
      case 0x4c:
        dat_2b80c = 2;
        break;
      case 0x50:
        v47 = sub_7950("perl",v47);
        break;
      case 0x52:
        dat_2b014 = 0x30a;
        goto label_5267;
      case 0x54:
        dat_2b8fc = 1;
        break;
      case 0x56:
        dat_2b905 = '\x01';
        break;
      case 0x58:
        v47 = sub_7950(optarg,v47);
        break;
      case 0x5a:
        dat_2b804 = 0;
        break;
      case 0x61:
label_5291:
        dat_2b810 = 1;
        break;
      case 0x62:
        dat_2b800 = 1;
        break;
      case 99:
        dat_2b7e8 = 1;
        break;
      case 100:
        dat_2b018 = *(int *)(sub_f3f0("--directories",optarg,(long *)0x29fa0,(void *)0x22788,4,dat_2b078,1) * 4 + 0x22788);
        if (dat_2b018 != 3)
          v9 = v50;
        v50 = v9;
        break;
      case 0x65:
        v16 = strlen(optarg);
        v22 = v32;
        v21 = (long)v32 + ((v16 + 1) - v24);
        if (1 <= v21) {
          dat_2b8d8 = (unsigned char *)sub_1f1f0(v48,v49,v21,-1,1);
          v48 = dat_2b8d8;
          v18 = optarg;
        }
        *(char *)mempcpy(&v48[(long)v22],v18,v16) = 10;
        v32 = (long *)sub_7a20(v48,v22,v16 + 1 + (long)v22,"");
        break;
      case 0x66:
        if (strcmp(optarg,"-")) { // branch-flip
          v14 = fopen(v18,"r");
          if (!v14) {
            error(2,*__errno_location(),"%s",optarg);
            v18 = NULL;
            goto label_5a31;
          }
        }
        else {
          v14 = stdin;
        }
        v22 = v32;
        v39 = v32;
        while( true ) {
          v21 = (long)v39 + (2U - v24);
          if (1 <= v21) {
            v48 = (unsigned char *)sub_1f1f0(v48,v49,v21,-1,1);
            dat_2b8d8 = v48;
          }
          v16 = fread_unlocked(&v48[(long)v39],1,v24 - (long)(long *)((long)v39 + 1U),v14);
          if (!v16) break;
          v39 = (long *)((long)v39 + v16);
        }
        v41 = __errno_location();
        v9 = *v41;
        if (*(unsigned char *)v14 & 0x20) { // branch-flip
label_5749:
          if (v9) {
            error(2,v9,"%s",optarg);
            return v11;
          }
        }
        else if (stdin != v14) { // branch-flip
          if (fclose(v14)) {
            v9 = *v41;
            goto label_5749;
          }
        }
        else {
          clearerr_unlocked(v14);
        }
        if ((v39 != v22) && ((&v48[-1])[(long)v39] != 10)) {
          v48[(long)v39] = 10;
          v39 = (long *)((long)v39 + 1U);
        }
        v32 = (long *)sub_7a20(v48,v22,v39,optarg);
        break;
      case 0x68:
        v51 = 0xffffffff;
        break;
      case 0x69:
      case 0x79:
        dat_2b687 = '\x01';
        break;
      case 0x6c:
        goto label_5660;
      case 0x6d:
        v38 = sub_1f450(optarg,NULL,10,(long *)0x2b7e0,"");
        if (2 <= v38) {
          error(2,0,dcgettext(NULL,"invalid max count",5));
label_5660:
          dat_2b80c = 1;
        }
        break;
      case 0x6e:
        dat_2b801 = 1;
        break;
      case 0x6f:
        dat_2b8fd = '\x01';
        break;
      case 0x71:
        goto label_5540;
      case 0x72:
label_5267:
        dat_2b018 = 3;
        v50 = v9;
        break;
      case 0x73:
        dat_2b904 = 1;
        break;
      case 0x75:
        error(0,0,dcgettext(NULL,"warning: --unix-byte-offsets (-u) is obsolete",5));
        break;
      case 0x76:
        dat_2b802 = 1;
        break;
      case 0x77:
        sub_e570();
        dat_2b686 = 1;
        break;
      case 0x78:
        dat_2b685 = '\x01';
        break;
      case 0x7a:
        dat_2b684 = 0;
        break;
      case 0x80:
        if (!strcmp(optarg,"binary")) {
          dat_2b810 = 0;
          break;
        }
        if (!strcmp(v18,"text")) goto label_5291;
        if (strcmp(v18,"without-match")) {
          error(2,0,dcgettext(NULL,"unknown binary-files type",5));
          return v12;
        }
        goto label_52b7;
      case 0x81:
        if (optarg) {
          v9 = sub_f750(optarg,"always");
          if (((!v9) || (!sub_f750(v18,"yes"))) || (!sub_f750(v18,"force"))) {
            dat_2b900 = 1;
            break;
          }
          if (((!sub_f750(v18,"never")) || (!sub_f750(v18,"no"))) || (!sub_f750(v18,"none"))) {
            dat_2b900 = 0;
            break;
          }
          if (((sub_f750(v18,"auto")) && (sub_f750(v18,"tty"))) && (sub_f750(v18,"if-tty"))) {
            dat_2b908 = 1;
            break;
          }
        }
        dat_2b900 = 2;
        break;
      case 0x82:
        sub_16480(optarg);
        v21 = dat_2b8a0;
        if (!dat_2b8a0) {
          v21 = sub_169d0();
          dat_2b8a0 = v21;
        }
        sub_16d40(v21,optarg,0x50000000);
        v21 = dat_2b8a8;
        if (!dat_2b8a8) {
          v21 = sub_169d0();
          dat_2b8a8 = v21;
        }
        sub_16d40(v21,optarg,0x10000000);
        break;
      case 0x83:
      case 0x86:
        v21 = dat_2b8b0;
        if (!dat_2b8b0) {
          v21 = sub_169d0();
          dat_2b8b0 = v21;
        }
        v38 = (unsigned int)(v46 == 0x86) << 0x1d;
        sub_16d40(v21,optarg,v38 | 0x50000000);
        v21 = dat_2b8b8;
        if (!dat_2b8b8) {
          v21 = sub_169d0();
          dat_2b8b8 = v21;
        }
        sub_16d40(v21,optarg,v38 | 0x10000000);
        break;
      case 0x84:
        v21 = dat_2b8b0;
        if (!dat_2b8b0) {
          v21 = sub_169d0();
          dat_2b8b0 = v21;
        }
        v9 = sub_17290(sub_16d40,v21,optarg,0x50000000,10);
        if (!v9) {
          v21 = dat_2b8b8;
          if (!dat_2b8b8) {
            v21 = sub_169d0();
            dat_2b8b8 = v21;
          }
          v9 = sub_17290(sub_16d40,v21,optarg,0x10000000,10);
          if (!v9) break;
        }
        error(2,*__errno_location(),"%s",optarg);
label_5540:
        dat_2b7a1 = '\x01';
        dat_2b080 = 0;
        break;
      case 0x85:
        dat_2b070 = optarg;
        break;
      case 0x87:
        dat_2b7d8 = 1;
        break;
      case 0x88:
        dat_2b7d0 = optarg;
        break;
      case 0x89:
        dat_2b687 = '\0';
      
    }
    goto label_5148;
  }
  if (dat_2b905) {
    sub_1ed40(stdout,(char *)sub_19950(),"GNU grep","3.8",0);
    puts(dcgettext(NULL,"Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",5));
    return 0;
  }
  if (dat_2b908)
    sub_a170(0); // no-return
  if (v48) { // branch-flip
    if (!v32) {
      dat_2b686 = 0;
      dat_2b685 = '\0';
      dat_2b802 ^= 1;
      v32 = (long *)0x1;
      *v48 = 10;
    }
  }
  else {
    v21 = (long)optind;
    if (argc <= optind) {
      sub_a170(2);
    }
    optind += 1;
    v16 = 0;
    v18 = argv[v21];
    if ((v47 != 2) && (*v18 == '\\'))
      v16 = (unsigned long)(v18[1] == '-');
    v48 = (unsigned char *)sub_1f3f0(&v18[v16]);
    dat_2b8d8 = v48;
    v16 = strlen((char *)v48);
    v48[v16] = 10;
    v32 = (long *)sub_7a20(v48,0,v16 + 1,"");
  }
  v22 = v32;
  v49 = (long *)((long)v32 - 1);
  v32 = v49;
  sub_1a490(dat_2b8c8);
  if (dat_2b7a1) {
    v5 = 0;
    goto label_5965;
  }
  v5 = 0;
  if (!fstat(1,&v31)) {
    if ((v31._24_4_ & 0xf000) != 0x8000) { // branch-flip
      if ((v31._24_4_ & 0xf000) == 0x2000) {
        if (((stat("/dev/null",&v33)) || (v31._8_8_ != v33._8_8_)) || (v31._0_8_ != v33._0_8_))
          v5 = 1;
        else {
          dat_2b7a0 = '\x01';
        }
      }
    }
    else {
      v21 = 0x24;
      v36 = &v31;
      v37 = (unsigned int *)0x2b920;
      while (v21) {
        v1 = &v37[1];
        v42 = &v36->field_0x4;
        *v37 = *(unsigned int *)v36;
        v21 -= 1;
        v36 = (stat *)v42;
        v37 = v1;
      }
    }
  }
  if (dat_2b7a0) { // branch-flip
label_5965:
    dat_2b80c = 0;
label_596f:
    dat_2b7e8 = 0;
    dat_2b7a2 = 1;
  }
  else if (dat_2b80c) goto label_596f;
  dat_2b803 = dat_2b7e8 | dat_2b7a2;
  if (dat_2b7f0 < 0)
    dat_2b7f0 = v26;
  if (dat_2b7f8 < 0)
    dat_2b7f8 = v26;
  if (((!dat_2b7e0) || (((!v49 && (dat_2b802)) && ((!dat_2b685 && (!dat_2b686)))))) && (dat_2b80c != 2))
    return 1;
  if (dat_2b900 == 2) {
    if ((!v5) || (!sub_1fa50()))
      dat_2b900 = 0;
    else {
      dat_2b900 = (unsigned int)(isatty(1) != 0);
    }
  }
  sub_1fa90();
  if (dat_2b900) {
    v18 = getenv("GREP_COLOR");
    if (v18) {
      if (*v18) {
        v13 = v18;
        while( true ) {
          if (*v13 != ';') {
            if (9 < (unsigned int)((int)*v13 - 0x30U)) goto label_62a1;
          }
          if (!v13[1]) break;
          v13 = &v13[1];
        }
        dat_2b060 = v18;
        dat_2b068 = v18;
      }
    }
label_62a1:
    v13 = getenv("GREP_COLORS");
    if ((v13) && (*v13)) {
      v19 = (char *)sub_1f3f0(v13);
      v23 = NULL;
      v13 = v19;
label_62d6:
      do {
        do {
          do {
            while( true ) {
              v40 = &v13[1];
              v2 = *v13;
              if ((v2 != ':') && (v2)) break;
              *v13 = '\0';
              v21 = 0x2a620;
              v13 = "mt";
              do {
                if (!strcmp(v13,v19)) break;
                v13 = *(char **)(v21 + 0x18);
                v21 += 0x18;
              } while (v13);
              if ((*(unsigned long **)(v21 + 8)) && (v23))
                **(unsigned long **)(v21 + 8) = v23;
              if (*(void **)(v21 + 0x10))
                (**(void **)(v21 + 0x10))();
              if (!v2) goto label_631c;
              v23 = NULL;
              v19 = v40;
              v13 = v40;
            }
            if (v2 == '=') {
              if ((v19 == v13) || (v23)) goto label_631c;
              *v13 = '\0';
              v23 = v40;
              v13 = v40;
              goto label_62d6;
            }
            v13 = v40;
          } while (!v23);
        } while (v2 == ';');
      } while ((unsigned int)((int)v2 - 0x30U) <= 9);
    }
label_631c:
    if ((dat_2b068 == v18) || (dat_2b060 == v18))
      error(0,0,dcgettext(NULL,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5),v18,v18);
  }
  v21 = 1;
  v38 = 1;
  v35 = 0;
  do {
    while (*(char *)(v21 + 0x2b183) == '\x01') {
label_5aa0:
      v21 += 1;
      if (v21 == 0x100) goto label_5ad9;
    }
    v6 = v38;
    if (v35 & 0xff & (unsigned int)v21) goto label_5aa0;
    do {
      v38 = v6;
      v6 = v38 * 2;
    } while ((int)(v38 * 2) <= (int)(unsigned int)v21);
    v21 += 1;
    v35 |= v38;
  } while (v21 != 0x100);
label_5ad9:
  dat_2b868 = (unsigned long)(v35 & 0xff) * 0x101010101010101;
  v9 = 0;
  if (0 <= v47)
    v9 = v47;
  if (3 <= v47) goto label_5d5b;
  if (dat_2b687) {
    v41 = (int *)0x2b284;
    v42 = (char *)0x2b6a0;
    do {
      if (*v41 != -1) {
        v10 = sub_16380(*v41,&v33);
        if (1 <= v10) {
          v44 = &(&v33.field_0x0)[(long)v10 * 4];
          do {
            v3 = *(int *)&v44[-4];
            v34 = 0;
            if (wcrtomb(&v25,v3,&v34) != 1) {
              v8 = 0xff;
              goto label_619c;
            }
            v44 = &v44[-4];
          } while ((char *)((long)&v31 + (unsigned long)(unsigned int)(v10 - 1) * -4 + (long)v10 * 4 + 0x8c) != v44);
        }
        v8 = 1;
label_619c:
        *v42 = v8;
      }
      v41 = &v41[1];
      v42 = &v42[1];
    } while (v41 != (int *)0x2b684);
  }
  if (v47 != 2) {
    if (2 <= dat_2b8d0) {
      v15 = (void *)sub_1ef30(v22);
      v33._0_8_ = 0;
      v22 = v49;
      v17 = v15;
      v45 = v48;
      while (v22) {
        v7 = *v45;
        if (v7 == 0x5b) goto label_5d53;
        if ('\\' <= (char)v7) { // branch-flip
          if (v7 != 0x5c) { // branch-flip
            if (v7 == 0x5e) goto label_5d53;
            if ((unsigned char)(v7 + 0x85) <= 1) goto label_61c8;
          }
          else if (2 <= (long)v22) {
            v7 = v45[1];
            if ('=' <= (char)v7) { // branch-flip
              if ((unsigned char)(v7 - 0x3e) <= 0x3e) {
                v16 = 1L << (v7 - 0x3e & 0x3f);
                if (!(v16 & 0x220001402200011)) {
                  v16 &= 0x6000000000000002;
label_6596:
                  if ((!v16) || (v47 == 1)) goto label_622b;
                  v9 = 0;
                }
                goto label_5d53;
              }
            }
            else if ('\n' <= (char)v7) {
              v16 = 1L << (v7 & 0x3f);
              if (!(v16 & 0x13fe008000000400)) {
                v16 &= 0xb0000000000;
                goto label_6596;
              }
              goto label_5d53;
            }
label_622b:
            v45 = &v45[1];
            v22 = (long *)((long)v22 - 1);
          }
        }
        else if ('/' <= (char)v7) { // branch-flip
          if (v7 == 0x3f) {
label_61c8:
            if (v47 == 1) goto label_5d53;
          }
        }
        else if ('$' <= (char)v7) {
          v16 = 1L << (v7 & 0x3f);
          if (v16 & 0x441000000000) goto label_5d53;
          if (v16 & 0x90000000000) goto label_61c8;
        }
        if (dat_2b687) { // branch-flip
          if (*(int *)((unsigned long)*v45 * 4 + 0x2b284) != -1) // branch-flip
            v16 = (unsigned long)*(char *)((unsigned long)*v45 + 0x2b6a0);
          else {
            v16 = sub_7d80(v45,v22,&v33);
          }
        }
        else {
          v16 = sub_e540(v45,v22,&v33);
        }
        if ((long)v16 < 0) goto label_5d53;
        v43 = &v45[v16];
        v17 = mempcpy(v17,v45,v16);
        v22 = (long *)((long)v22 - v16);
        v45 = v43;
      }
      v9 = 2;
      v22 = (long *)((long)v17 - (long)v15);
      if (v49 != v22) {
        v32 = v22;
        *(char *)mempcpy(v48,v15,(unsigned long)v22) = 10;
      }
label_5d53:
      free(v15);
    }
    goto label_5d5b;
  }
  if (dat_2b180) { // branch-flip
    v33._0_8_ = 0;
    for (v21 = 0; v21 < (long)v49; v21 = v21 + v16) {
      v2 = *(char *)((unsigned long)v48[v21] + 0x2b183);
      v16 = (unsigned long)v2;
      if (((v2 != '\xfe') || (v16 = sub_1ac90(&v48[v21],(long)v49 - v21,&v33), 0x11 <= v16)) && ((long)v16 < 0)) goto label_663d;
    }
    if (dat_2b687) {
label_666b:
      v33._0_8_ = 0;
      for (v21 = 0; v21 < (long)v49; v21 = v21 + v20) {
        v16 = (unsigned long)v48[v21];
        if (*(int *)(v16 * 4 + 0x2b284) != -1) // branch-flip
          v20 = (long)*(char *)(v16 + 0x2b6a0);
        else {
          v20 = sub_7d80(&v48[v21],(long)v49 - v21,&v33);
        }
        if (v20 < 0) goto label_663d;
      }
    }
    v7 = 0;
  }
  else {
    v7 = 0;
    if (dat_2b8d0 == 1)
      v7 = dat_2b686;
  }
label_60b9:
  v9 = 2;
  if (v7 & 1) {
    sub_a3e0((unsigned long *)0x2b8d8,&v32);
    v48 = dat_2b8d8;
    v9 = 0;
  }
label_5d5b:
  v21 = (long)v9 * 0x20;
  dat_2b878 = *(void **)(v21 + 0x2a7b8);
  dat_2b870 = (**(void **)(v21 + 0x2a7b0))(v48,v32,(long)*(int *)(v21 + 0x2a7ac),dat_2b8fd || dat_2b900);
  v30 = dat_2b684;
  v25 = '\0';
  v52 = 0;
  v21 = (*dat_2b878)(dat_2b870,&v30,1,v29,0);
  dat_2b820 = (v21 == 0) == (bool)dat_2b802;
  argc -= optind;
  if ((v51) || (2 <= argc))
    dat_2b808 = ~v51 >> 0x1f;
  else {
    dat_2b808 = -(unsigned int)(dat_2b018 == 3);
  }
  v16 = sysconf(0x1e);
  if (0x3ffffffffffffffa < v16 - 1)
    abort(); // no-return
  v4._8_8_ = 0;
  v4._0_8_ = v16;
  v21 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v4._0_16_,0);
  dat_2b858 = 0x18000;
  if (v21)
    dat_2b858 = (v16 + 0x18000) - v21;
  dat_2b858 = v16 + 8 + dat_2b858;
  dat_2b838 = v16;
  dat_2b860 = sub_1ef30(dat_2b858);
  if ((dat_2b014 & 2) && (!dat_2b884))
    dat_2b884 = 1;
  if (1 <= argc) // branch-flip
    argv = &argv[optind];
  else if ((dat_2b018 != 3) || (v50 <= 0))
    argv = (char **)0x29f80;
  else {
    dat_2b88a = 1;
    argv = (char **)0x29f90;
  }
  v18 = *argv;
  v38 = 1;
  do {
    while( true ) {
      argv = &argv[1];
      if (!strcmp(v18,"-")) break;
      dat_2b890 = v18;
      v35 = sub_a0f0(0xffffff9c,v18,1,1);
      v18 = *argv;
      v38 &= v35;
      if (!v18) goto label_5f29;
    }
    dat_2b890 = dat_2b7d0;
    v35 = sub_9130(0,1);
    v18 = *argv;
    v38 &= v35;
  } while (v18);
label_5f29:
  if (dat_2b889)
    v38 = 2;
  return v38;
label_663d:
  v7 = 1;
  goto label_60b9;
}


// Function: sub_6840 @ 0x6840
unsigned int sub_6840(long a0,char *a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe
{
  unsigned int v1;
  undefined8 v10; // stack - 0x50
  char *v2; // rax
  char *v3; // rax
  unsigned long *v4; // rbx
  char v5 [32]; // stack - 0x88
  long v6; // stack - 0x90
  void *v7; // stack - 0x68
  undefined8 v8; // stack - 0x60
  undefined8 v9; // stack - 0x58
  
  v1 = a6;
  v5._0_8_ = 0;
  v5._8_8_ = 0;
  if ((dat_2b687) || ((char)a6)) {
    v7 = NULL;
    v8 = 0;
    if ((char)a6) {
      re_set_syntax(a5 | 0x2000000);
      v2 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v5);
      if (!v2) {
        regfree((re_pattern_buffer *)v5);
        return v1;
      }
      goto label_68e7;
    }
  }
  else {
    v7 = (void *)sub_1ef30(0x100);
  }
  v8 = 0;
  re_set_syntax(a5);
  v2 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v5);
  if (!v2) {
    v4 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
    *v4 = v5._0_8_;
    v4[1] = v5._8_8_;
    v4[2] = v5._16_8_;
    v4[3] = v5._24_8_;
    v4[4] = v7;
    v4[5] = v8;
    v4[6] = v9;
    v4[7] = v10;
    return 1;
  }
label_68e7:
  free(v7);
  if ((a4 != -1) && (v3 = (char *)sub_9080(a4,&v6), *v3)) {
    error(0,0,"%s:%td: %s",v3,v6,v2);
    return 0;
  }
  error(0,0,"%s",v2);
  return 0;
}


// Function: sub_6a10 @ 0x6a10
void sub_6a10(char *a0)
{
  error(2,0,"%s",a0);
}


// Function: sub_6a40 @ 0x6a40
void sub_6a40(char *a0)
{
  error(0,0,dcgettext(NULL,"warning: %s",5),a0); // tail-call
}


// Function: sub_6a70 @ 0x6a70
unsigned long * sub_6a70(char *a0,unsigned long a1,unsigned long a2,bool a3)
{
  unsigned char *v1;
  unsigned long *v10; // rax
  char *v11; // rax
  long v12;
  unsigned long v13;
  char *v14;
  void *v15; // rax
  unsigned long v16; // rax
  unsigned char *v17; // rax
  long v18;
  long v19; // stack - 0x50
  char v2;
  long v20; // stack - 0x48
  char *v21;
  unsigned long v22; // stack - 0xb0
  void *v23; // stack - 0xa0
  char *v24; // stack - 0x90
  unsigned long v25; // stack - 0x88
  char *v26; // stack - 0x80
  char *v27; // stack - 0x70
  unsigned long v28; // stack - 0x68
  unsigned char v3;
  long v4;
  bool v5;
  bool v6;
  char v7;
  unsigned char v8;
  unsigned long *v9; // rax
  
  v9 = (unsigned long *)sub_1f2f0(1,0x48);
  v10 = (unsigned long *)sub_15a90();
  v9[1] = v10;
  if (dat_2b687)
    a2 |= 0x400000;
  sub_15fa0(v10,(unsigned char *)0x2b180,a2,(unsigned int)(dat_2b684 == 0) * 2 | (unsigned int)a2 & 0x10 | 0x2c);
  v7 = dat_2b182;
  v2 = dat_2b180;
  v11 = &a0[a1];
  v12 = sub_1ef10(0x40);
  v6 = 0;
  v9[3] = 0;
  v9[2] = v12 + 0x40;
  v12 = 0;
  v19 = 1;
  v20 = 0;
  v25 = 0;
  v26 = NULL;
  v21 = a0;
  v24 = a0;
  do {
    v23 = rawmemchr(v21,10);
    v13 = (long)v23 - (long)v21;
    v18 = v13 - 1;
    if (0 <= v18) {
      v14 = v21;
      while (v14 = memchr(v14,0x5c,(long)&v21[v18] - (long)v14), v14) {
        if ((unsigned char)(v14[1] - 0x31U) <= 8) {
          if (v21 <= v24) { // branch-flip
            v22 = 0;
            v5 = 1;
          }
          else {
            v16 = (long)v21 - (long)v24;
            v18 = (v25 - v20) + v16;
            if (1 <= v18)
              v26 = (char *)sub_1f1f0(v26,&v20,v18,-1,1);
            memcpy(&v26[v25],v24,v16);
            v25 += v16;
            v5 = 1;
            v22 = 0;
          }
          goto label_6bfa;
        }
        if (((int)v14[1] == (-(unsigned int)(v2 == '\x01' && !v7) & 0x24) + 0x5c) && (v14 = &v14[1], v14 == &v21[v18])) break;
        v14 = &v14[1];
      }
    }
    v22 = 1;
    v5 = 0;
label_6bfa:
    v18 = (v9[3] - v19) + 2;
    if (1 <= v18) {
      v4 = v9[2];
      v9[2] = sub_1f1f0((void *)(v4 + -0x40),&v19,v18,-1,0x40) + 0x40;
    }
    re_set_syntax(a2);
    v18 = v9[3];
    if (!sub_6840(v9,v21,v13,v18,v12,a2,v22))
      v6 = 1;
    v21 = (char *)((long)v23 + 1);
    v12 += 1;
    if (v5) {
      v9[3] = v9[3] + 1;
      v24 = v21;
    }
    if (v11 < v21) {
      if (v6)
        exit(2); // no-return
      if (v24 <= v11) {
        if (v24 <= a0) { // branch-flip
          v25 = a1;
          v26 = a0;
        }
        else {
          v13 = v25 + ((long)v11 - (long)v24);
          v26 = (char *)sub_1efa0(v26,v13);
          memcpy(&v26[v25],v24,(long)v11 - (long)v24);
          v25 = v13;
        }
      }
      if ((dat_2b686) || (v21 = NULL, v27 = a0, v28 = a1, dat_2b685)) {
        v21 = (char *)sub_1ef30(a1 + 0x2d);
        if (dat_2b685) { // branch-flip
          if (a2 & 0x2000) { // branch-flip
            *(unsigned short *)v21 = dat_21097;
            v21[2] = dat_21099;
            v13 = strlen(v21);
            v12 = v13 + a1;
            memcpy(&v21[v13],a0,a1);
            v11 = ")$";
          }
          else {
            *(unsigned int *)v21 = dat_2109a;
            v13 = strlen(v21);
            v12 = v13 + a1;
            memcpy(&v21[v13],a0,a1);
            v11 = "\\)$";
          }
        }
        else if (a2 & 0x2000) { // branch-flip
          *(unsigned long *)v21 = s_21060._0_8_;
          *(unsigned long *)&v21[8] = s_21060._8_8_;
          *(unsigned short *)&v21[0x10] = s_21060._16_2_;
          v21[0x12] = s_21060[0x12];
          v13 = strlen(v21);
          v12 = v13 + a1;
          memcpy(&v21[v13],a0,a1);
          v11 = ")([^[:alnum:]_]|$)";
        }
        else {
          *(unsigned long *)v21 = s_21080._0_8_;
          *(unsigned long *)&v21[8] = s_21080._8_8_;
          *(unsigned int *)&v21[0x10] = s_21080._16_4_;
          *(unsigned short *)&v21[0x14] = s_21080._20_2_;
          v21[0x16] = s_21080[0x16];
          v13 = strlen(v21);
          v12 = v13 + a1;
          memcpy(&v21[v13],a0,a1);
          v11 = "\\)\\([^[:alnum:]_]\\|$\\)";
        }
        v28 = strlen(strcpy(&v21[v12],v11));
        v27 = v21;
        v28 += v12;
      }
      sub_14cf0(v27,v28,v9[1]);
      v11 = (char *)sub_15020((struct_25 *)v9[1]);
      if (v11) {
        v1 = (unsigned char *)&v11[3];
        v15 = (void *)sub_e5c0(0);
        v2 = *v11;
        *v9 = v15;
        if (v2) { // branch-flip
          v9[7] = v9[7] + 1;
          v16 = strlen((char *)v1);
          v13 = (unsigned char)v11[1] + v16 + (unsigned long)(unsigned char)v11[2];
          v17 = (unsigned char *)sub_1ef30(v13);
          v8 = dat_2b684;
          *v17 = dat_2b684;
          v3 = v11[1];
          *(unsigned char *)&v9[8] = *(unsigned char *)&v9[8] | v3;
          v15 = memcpy(&v17[v3],v1,v16);
          if (v11[2])
            *(unsigned char *)((long)v15 + v16) = v8;
          sub_c9d0((void *)*v9,v17,v13);
          free(v17);
        }
        else {
          sub_c9d0(v15,v1,strlen((char *)v1));
        }
        sub_cec0((unsigned long *)*v9);
        sub_15a80(v11);
      }
      sub_15aa0(0,0,(unsigned long *)v9[1],1);
      if (v26) {
        if ((a3) || (v12 = v9[1], !sub_14e30(v12))) {
          v9[2] = v9[2] + -0x40;
          v9[3] = v9[3] + 1;
          if (!sub_6840(v9,v26,v25,0,-1,a2,0))
            abort(); // no-return
        }
        if (v27 != v26)
          free(v26);
      }
      free(v21);
      return v9;
    }
  } while( true );
}


// Function: sub_7110 @ 0x7110
long sub_7110(struct_28 *a0,char *a1,long a2,long *a3,char *a4) // return-dupe
{
  char *v1;
  long v10;
  char *v11;
  char *v12;
  int v13;
  long v14;
  char *v15; // rbx
  long v16 [3]; // stack - 0x58
  long v17; // stack - 0x60
  char v18; // stack - 0x69
  char *v19; // stack - 0x68
  char *v2;
  long v20;
  int v21; // esi
  re_pattern_buffer *v22; // rdi
  char *v23;
  bool v24; // zf
  char *v25; // stack - 0xf8
  char *v26; // stack - 0xe0
  long v27; // stack - 0xd8
  char *v28; // stack - 0xb0
  long v29; // stack - 0xa0
  unsigned char *v3;
  char v4;
  char v5; // al
  int v6;
  long v7; // rax
  long v8;
  void *v9; // rax
  
  v4 = dat_2b684;
  v15 = &a1[a2];
  v8 = a0->field_0x8;
  v7 = sub_14e10(v8);
  v5 = sub_14e20(v8);
  if (v15 <= a1)
    return -1;
  v26 = a1;
  v19 = a1;
  do {
    while (a4) {
      v11 = v26;
      v26 = v15;
      v28 = a4;
label_73d9:
      v29 = (long)v26 - (long)v11;
      if (0x80000000 < v29)
        sub_1f410(); // no-return, return-dupe
      if (1 <= a0->field_0x18) {
        v1 = a0->field_0x20;
        v27 = 0;
        v25 = v26;
        v12 = &v26[-1];
        v10 = (long)v26 - (long)v28;
        v8 = 0;
        v21 = (int)v11;
        do {
          while( true ) {
            v20 = v8 * 0x40;
            v22 = (re_pattern_buffer *)(a0->field_0x10 + v20);
            v22->field_0x38 = (dat_2b684 == '\n') << 7 | v22->field_0x38 & 0x3f;
            v6 = re_search(v22,v11,(int)v29 + -1,(int)v28 - v21,(int)v10 + -1,v1);
            if (v6 < -1) {
              sub_1f410();
            }
            if (v6 != -1) break;
label_7528:
            v8 += 1;
            if (a0->field_0x18 <= v8) goto label_75d0;
          }
          v23 = &v11[v6];
          if (v25 < v23) goto label_7528;
          v14 = (long)(*a0->field_0x30 - v6);
          if (a4) {
            if (dat_2b686) {
              if (!dat_2b685) goto label_748d;
              v24 = v10 + -1 != v14;
              v14 = v10;
              v23 = v28;
              if (v24) goto label_7528;
            }
label_761a:
            if (v25 <= v23) { // branch-flip
              if (v25 == v23) {
                if (v14 <= v27)
                  v14 = v27;
                v27 = v14;
              }
            }
            else {
              v25 = v23;
              v27 = v14;
            }
            goto label_7528;
          }
          if (!dat_2b685) {
            if (dat_2b686) {
label_748d:
              do {
                if ((!sub_e820(&v23[v14],v12)) && (!sub_e830(v11,v23,v12))) {
                  if (a4) goto label_761a;
                  goto label_7690;
                }
                v6 = (int)v23; // branch-flip
                if (1 <= v14) {
                  v22 = (re_pattern_buffer *)(v20 + a0->field_0x10);
                  v2 = &v22->field_0x38;
                  *v2 = *v2 | 0x40;
                  v13 = re_match(v22,v11,(v6 + -1 + (int)v14) - (int)v28,v6 - v21,v1);
                  if (v13 < -1) {
                    sub_1f410();
                  }
                  if (v13 <= 0) goto label_74c2;
                }
                else {
label_74c2:
                  if (v12 == v23) break;
                  v22 = (re_pattern_buffer *)(v20 + a0->field_0x10);
                  v2 = &v22->field_0x38;
                  *v2 = *v2 & 0xbf;
                  v6 = re_search(v22,v11,(int)v29 + -1,(v6 + 1) - v21,((int)v26 - (v6 + 1)) + -1,v1);
                  if (v6 < 0) {
                    if (v6 != -1) {
                      sub_1f410();
                    }
                    goto label_7528;
                  }
                  v13 = *a0->field_0x30 - v6;
                  v23 = &v11[v6];
                }
                v14 = (long)v13;
              } while (v23 <= v25);
              goto label_7528;
            }
            goto label_7690;
          }
          if (v14 == v10 + -1) goto label_7690;
          v8 += 1;
        } while (v8 < a0->field_0x18);
label_75d0:
        if (v25 < v26) {
          v29 = v27;
          v11 = v25;
          goto label_7690;
        }
      }
label_75e2:
      if (v15 <= v26)
        return 0xffffffffffffffff;
    }
    v18 = '\0';
    v17 = 0;
    v12 = v15;
    if (a0->field_0x0) {
      v8 = sub_e3e0(a0->field_0x0,(long)v26 - (unsigned long)a0->field_0x40,(unsigned long)a0->field_0x40 + ((long)v15 - (long)v26),v16,1);
      if (v8 <= -1)
        return v8;
      v3 = (unsigned char *)&v26[v8];
      v21 = (int)v4;
      v9 = memrchr(a1,v21,(long)v3 - (long)a1);
      v10 = v16[0];
      v11 = (char *)((long)v9 + 1);
      if (!v9)
        v11 = a1;
      v20 = a0->field_0x38;
      if ((v5 == '\x01') && (v20 <= v16[0])) {
        v10 = (long)v3 - (long)v11;
        if (v10 <= 0xf)
          v10 = 0x10;
        if (v8 >> 2 <= v10) { // branch-flip
          if ((long)v15 - (long)v26 >> 2 <= v10) // branch-flip
            v26 = v11;
          else {
            v12 = (char *)((long)rawmemchr(&v26[v10 * 4],v21) + 1);
            v26 = v11;
          }
        }
        else {
          v12 = (char *)((long)rawmemchr(v3,v21) + 1);
          v26 = v11;
        }
        goto label_72a0;
      }
      v12 = (char *)((long)rawmemchr(v3,v21) + 1);
      if (v20 <= v10) {
        v26 = v11;
        goto label_72a0;
      }
      v26 = v12;
      if ((dat_2b180 == '\x01') && (!dat_2b182)) {
        if (v19 < v11)
          v19 = v11;
        if (!sub_e640(&v19,NULL,v3,v15)) goto label_782b;
        v23 = v19;
        goto label_7335;
      }
label_782b:
      v29 = (long)v26 - (long)v11;
label_7690:
      *a3 = v29;
      return (long)v11 - (long)a1;
    }
label_72a0:
    if (!v7) {
      v23 = v26;
      v11 = v26;
      v26 = v12;
label_7335:
      v12 = (char *)sub_14e00(a0->field_0x8,v23,v26,0,&v17,&v18);
      if ((!v12) || (v26 == v12)) goto label_75e2;
      if (v17)
        v11 = (char *)((long)memrchr(a1,(int)v4,(long)v12 - (long)a1) + 1);
      v26 = (char *)((long)rawmemchr(v12,(int)v4) + 1);
      if (v18) {
        v28 = v11;
        goto label_73d9;
      }
      goto label_782b;
    }
    v11 = (char *)sub_14e00(v7,v26,v12,0,&v17,0);
    if ((v11) && (v12 != v11)) {
      if (v17)
        v26 = (char *)((long)memrchr(a1,(int)v4,(long)v11 - (long)a1) + 1);
      v9 = rawmemchr(v11,(int)v4);
      v17 = 0;
      v23 = v26;
      v11 = v26;
      v26 = (char *)((long)v9 + 1);
      goto label_7335;
    }
    v26 = v12;
    if (v15 <= v12)
      return 0xffffffffffffffff;
  } while( true );
}


// Function: sub_7850 @ 0x7850
unsigned long sub_7850(long a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  unsigned char *v3;
  
  v3 = (unsigned char *)(dat_2b8d8 + -1 + a0);
  v2 = 0x3938a0565d85cb03;
  v1 = *v3;
  while (v1 != 10) {
    v3 = &v3[1];
    v2 = v2 * 0x21 ^ (unsigned long)v1;
    v1 = *v3;
  }
  return v2 % a1;
}


// Function: sub_78a0 @ 0x78a0
unsigned long sub_78a0(long a0,long a1) // return-dupe
{
  char *v1;
  char *v2;
  char *v3;
  long v4; // rax
  long v5;
  char v6;
  
  v1 = (char *)(dat_2b8d8 + -1 + a0);
  v2 = (char *)(dat_2b8d8 + -1 + a1);
  v6 = *v1;
  if (v6 != *v2)
    return 0;
  v5 = 1;
  do {
    if (v6 == '\n')
      return 1;
    v6 = v1[v5];
    v4 = v5 + 1;
    v3 = &v2[v5];
    v5 = v4;
  } while (v6 == *v3);
  return 0;
}


// Function: sub_78f0 @ 0x78f0
void sub_78f0(void)
{
  dat_2b060 = dat_2b068;
}


// Function: sub_7910 @ 0x7910
void sub_7910(void)
{
  dat_2b900 = 0xffffffff;
}


// Function: sub_7920 @ 0x7920
void sub_7920(void)
{
  dat_2b028 = 0x2109e;
  dat_2b020 = 0x210a4;
}


// Function: sub_7950 @ 0x7950
char * sub_7950(char *a0,int a1)
{
  char *v1; // rax
  unsigned long v2; // rbx
  
  v2 = 0;
  while( true ) {
    if (!strcmp(a0,(char *)(v2 * 0x20 + 0x2a7a0))) break;
    v2 += 1;
    if (v2 == 7) {
      v1 = dcgettext(NULL,"invalid matcher %s",5);
      error(2,0,v1,a0);
      return v1;
    }
  }
  if ((0 <= a1) && ((int)v2 != a1)) {
    v1 = dcgettext(NULL,"conflicting matchers specified",5);
    error(2,0,v1);
    return v1;
  }
  v1 = (char *)(v2 & 0xffffffff);
  return v1;
}


// Function: sub_7a20 @ 0x7a20
long sub_7a20(long a0,long a1,long a2,long a3)
{
  void *v1;
  long v2;
  long *v3;
  int v4; // eax
  long v5;
  void *v6;
  long v7; // rbp
  void *v8;
  int v9;
  
  v6 = (void *)(a0 + a1);
  if (v6 < (void *)(a2 + a0)) {
    v7 = 1;
    v8 = v6;
    v9 = 0;
    do {
      v1 = (void *)((long)rawmemchr(v8,10) + 1);
      memmove(v6,v8,(long)v1 - (long)v8);
      a1 = (long)v6 - a0;
      v4 = sub_1a700(dat_2b8c8,a1 + 1,NULL);
      if (v4) {
        if (v4 < 0)
          sub_1f410(); // no-return
        v6 = (void *)((long)v6 + ((long)v1 - (long)v8));
        if (!v9) {
          v5 = dat_2b8f0;
          if (dat_2b8e0 == dat_2b8e8) {
            v5 = sub_1f1f0(dat_2b8f0,(long *)0x2b8e8,1,-1,0x18);
            dat_2b8f0 = v5;
          }
          v2 = dat_2b8e0 + 1;
          v3 = (long *)(v5 + dat_2b8e0 * 0x18);
          v3[1] = a3;
          *v3 = dat_2b8d0;
          v3[2] = v7;
          dat_2b8e0 = v2;
        }
        dat_2b8d0 += 1;
        a1 = (long)v6 - a0;
      }
      v7 += 1;
      v8 = v1;
      v9 = v4;
    } while (v1 < (void *)(a2 + a0));
  }
  return a1;
}


// Function: sub_7b80 @ 0x7b80
void sub_7b80(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  char v1; // al
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
  unsigned int v2; // stack - 0xd8
  char v3 [8];
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xb0
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v1) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v5 = &Stack0000000000000008;
  v2 = 8;
  v6 = v3;
  v4 = 0x30;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  if (0 <= __vfprintf_chk(stdout,1,a8,&v2))
    return;
  dat_2b8c0 = *__errno_location();
}


// Function: sub_7c90 @ 0x7c90
void sub_7c90(void *a0) // return-dupe
{
  long v1;
  void *v2;
  long v3; // rbx
  int v4; // r12d
  
  v1 = dat_2b818;
  if (a0 <= dat_2b7c0) {
    dat_2b7c0 = a0;
    return;
  }
  v4 = (int)dat_2b684;
  v3 = 0;
  v2 = dat_2b7c0;
  do {
    v2 = memchr(v2,v4,(long)a0 - (long)v2);
    if (!v2) break;
    v2 = (void *)((long)v2 + 1);
    v3 += 1;
  } while (v2 < a0);
  if (!SCARRY8(v3,v1)) {
    dat_2b7c0 = a0;
    dat_2b818 = v3 + v1;
    return;
  }
  sub_7c60(); // no-return
}


// Function: sub_7ec0 @ 0x7ec0
void sub_7ec0(char a0)
{
  if ((dat_2b900) && (*dat_2b040))
    sub_1faa0(dat_2b028);
  sub_7e70((int)a0);
  if ((dat_2b900) && (*dat_2b040)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}


// Function: sub_7f20 @ 0x7f20
void sub_7f20(unsigned long a0,char *a1)
{
  if ((dat_2b900) && (*a1))
    sub_1faa0(dat_2b028,a1,a0);
  sub_7b80("%*ld",dat_2b8f8,a0);
  if ((dat_2b900) && (*a1)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}


// Function: sub_7fa0 @ 0x7fa0
void sub_7fa0(void)
{
  char *v1;
  
  v1 = dat_2b890;
  if ((dat_2b900) && (*dat_2b058)) {
    sub_1faa0(dat_2b028);
    v1 = dat_2b890;
  }
  dat_2b890 = v1;
  if (!v1) {
    v1 = dcgettext(NULL,"(standard input)",5);
    dat_2b890 = v1;
  }
  if (fputs_unlocked(v1,stdout) <= -1)
    dat_2b8c0 = *__errno_location();
  if ((dat_2b900) && (*dat_2b058)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}


// Function: sub_8050 @ 0x8050
char sub_8050(unsigned long *a0,long a1,unsigned long a2,char a3) // early-return
{
  unsigned long *v1;
  unsigned long *v10;
  bool v2;
  bool v3;
  unsigned long v4;
  char v5; // al
  unsigned long v6; // rax
  unsigned long v7;
  unsigned long v8; // stack - 0x48
  long v9;
  
  if (dat_2b810 != 1) {
    v1 = (unsigned long *)((long)a0 + a1);
    v4 = *v1;
    if (dat_2b868) {
      *(unsigned char *)v1 = 0xff;
      v8 = 0;
      v7 = dat_2b868;
      v10 = a0;
      do {
        for (; (unsigned long)v10 & 7; v10 = (unsigned long *)((long)v10 + 1)) {
          v6 = (unsigned long)(unsigned char)*v10;
          if (v6 & v7) goto label_81ff;
        }
        for (; v6 = v7 & *v10, !v6; v10 = &v10[1]) {
        }
        while (v6 = (unsigned long)(unsigned char)*v10, !(v6 & v7)) {
          v10 = (unsigned long *)((long)v10 + 1);
        }
label_81ff:
        if (v1 <= v10) {
          *(unsigned char *)v1 = (unsigned char)v4;
          break;
        }
        v6 = sub_1ac90(v10,(long)v1 - (long)v10,&v8);
        if ((0x11 <= v6) && ((long)v6 <= -1)) {
          *(unsigned char *)v1 = (unsigned char)v4;
          v5 = '\0';
          dat_2b888 = 1;
          return v5;
        }
        v10 = (unsigned long *)((long)v10 + v6);
        v7 = dat_2b868;
      } while( true );
    }
  }
  if (dat_2b808) {
    sub_7fa0();
    if (dat_2b804) // branch-flip
      sub_7ec0((int)a3);
    else {
      sub_7e70(0);
    }
  }
  if (dat_2b801) {
    v9 = dat_2b818;
    if (dat_2b7c0 < a2) {
      sub_7c90(a0);
      v9 = dat_2b818 + 1;
      if (SCARRY8(dat_2b818,1)) {
        sub_7c60(v9); // no-return, return-dupe
      }
      dat_2b7c0 = a2;
      dat_2b818 = v9;
    }
    sub_7f20(v9,dat_2b050);
    sub_7ec0((int)a3);
  }
  if (dat_2b800) {
    v9 = ((long)a0 - dat_2b848) + dat_2b7c8;
    if (SCARRY8((long)a0 - dat_2b848,dat_2b7c8))
      sub_7c60(v9);
    sub_7f20(v9,dat_2b048);
    sub_7ec0((int)a3);
  }
  if (!dat_2b8fc) {
    v5 = '\x01';
    return v5;
  }
  v2 = dat_2b801 || dat_2b800;
  v3 = dat_2b808 != 0;
  v5 = dat_2b8fc;
  if ((v2 || v3) && a1) {
    sub_7e70(9);
    v5 = (v2 || v3) && a1;
  }
  return v5;
}


// Function: sub_82a0 @ 0x82a0
void sub_82a0(unsigned long *a0,unsigned long *a1,char a2)
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long *v11;
  bool v12; // zf
  char *v13; // stack - 0x58
  char *v14; // stack - 0x50
  unsigned char v2;
  char v3;
  long v4;
  unsigned long v5; // rax
  unsigned char v6; // cl
  long v7; // stack - 0x48
  int v8;
  unsigned long *v9;
  
  if ((!dat_2b8fd) && (!sub_8050(a0,(long)a1 + (0xffffffffffffffffU - (long)a0),a1,(int)a2)))
    return;
  v12 = a2 == ':';
  v6 = (v12 ^ dat_2b802) & dat_2b8fd;
  v2 = dat_2b8fd;
  if (dat_2b900) { // branch-flip
    v14 = dat_2b038;
    if (v12 == (bool)((unsigned char)((unsigned int)dat_2b900 >> 0x1f) & dat_2b802))
      v14 = dat_2b030;
    v13 = dat_2b068;
    if (a2 != ':')
      v13 = dat_2b060;
    if (v6) goto label_84c2;
    v9 = a0;
    if (!*v14) {
      if (!*v13) goto label_82fd;
      if (v12 == (bool)dat_2b802) goto label_83d0;
label_83b0:
      if (a0 < a1) goto label_84cb;
      goto label_83c0;
    }
    if (v12 == (bool)dat_2b802) goto label_83d0;
    if (*v13) goto label_83b0;
label_83f7:
    a0 = a1;
    if (v9 < a1) {
      v5 = 0;
      if (*(char *)((long)a1 + -1) == dat_2b684) {
        a0 = (unsigned long *)((long)a1 - 1);
        if (a0 <= v9) goto label_8430;
        v5 = 1;
      }
      if (*(char *)((long)a1 + ~v5) == '\r')
        a0 = (unsigned long *)((long)a1 - (v5 + 1));
    }
label_8430:
    if (1 <= (long)a0 - (long)v9) { // branch-flip
      sub_1faa0(dat_2b028,v14);
      sub_7e40(v9,(long)a0 - (long)v9);
      if (*v14)
        sub_1fac0(dat_2b020);
      v2 = dat_2b8fd;
    }
    else {
      a0 = v9; // crossjump-dupe
      v2 = dat_2b8fd;
    }
  }
  else if (v6) {
    v13 = NULL;
    v14 = NULL;
label_84c2:
    if (a1 <= a0) { // branch-flip
label_86b0:
      a0 = a1;
label_83c0:
      if (!a0)
        return;
      v9 = a0;
      v2 = dat_2b8fd;
label_83d0:
      if (v2) goto label_8318;
      v3 = *v14;
    }
    else {
label_84cb:
      v9 = NULL;
      v11 = a0;
      do {
        v4 = (*dat_2b878)(dat_2b870,a0,(long)a1 - (long)a0,&v7,v11);
        if ((v4 < 0) || (v1 = (unsigned long *)((long)a0 + v4), a1 == v1)) break;
        if (v7) { // branch-flip
          if (dat_2b8fd) { // branch-flip
            v3 = sub_8050(v1,v7,a1,(-(dat_2b802 == 0) & 0xdU) + 0x2d);
            if (!v3)
              return;
          }
          else {
            if (*v14)
              sub_1faa0(dat_2b028,v14);
            v10 = v9;
            if (!v9)
              v10 = v11;
            v9 = NULL;
            sub_7e40(v10,(long)v1 - (long)v10);
          }
          if ((dat_2b900) && (*v13))
            sub_1faa0(dat_2b028,v13);
          sub_7e40(v1,v7);
          if ((dat_2b900) && (*v13))
            sub_1fac0(dat_2b020);
          if (dat_2b8fd)
            sub_7e70((int)dat_2b684);
          v4 = v7;
        }
        else {
          v4 = 1;
          v7 = 1;
          if (!v9)
            v9 = v11;
        }
        v11 = (unsigned long *)((long)v1 + v4);
      } while (v11 < a1);
      if (dat_2b8fd) goto label_86b0;
      a0 = v11;
      if (!v9) goto label_83c0;
      v3 = *v14;
    }
    if (v3) goto label_83f7;
    a0 = v9;
    v2 = dat_2b8fd;
  }
label_82fd:
  if ((a0 < a1) && (v2 != 1))
    sub_7e40(a0,(long)a1 - (long)a0);
label_8318:
  if ((dat_2b7d8) && (fflush_unlocked(stdout))) {
    v8 = *__errno_location();
    dat_2b8c0 = v8;
  }
  else {
    v8 = dat_2b8c0;
  }
  if (!v8) {
    dat_2b7b8 = a1;
    return;
  }
  error(2,v8,dcgettext(NULL,"write error",5));
}


// Function: sub_8710 @ 0x8710
void sub_8710(void *a0) // return-dupe x2
{
  void *v1;
  int v2; // esi
  
  if (!dat_2b7b8)
    dat_2b7b8 = dat_2b848;
  if (dat_2b7a8 <= 0)
    return;
  do {
    v1 = dat_2b7b8;
    if (a0 <= dat_2b7b8)
      return;
    v2 = (int)dat_2b684;
    sub_82a0(v1,(unsigned long *)((long)rawmemchr(dat_2b7b8,v2) + 1),0x2d);
    dat_2b7a8 -= 1;
  } while (1 <= dat_2b7a8);
}


// Function: sub_8790 @ 0x8790
long sub_8790(void *a0,void *a1) // early-return
{
  char v1;
  char *v10;
  char v11;
  void *v12;
  long v13;
  void *v14; // stack - 0x70
  long v2;
  long v3; // rax
  void *v4;
  unsigned long v5; // rax
  void *v6;
  void *v7;
  long v8; // stack - 0x48
  int v9;
  
  v2 = dat_2b7b0;
  if (a1 <= a0)
    return 0;
  do {
    v3 = (*dat_2b878)(dat_2b870,a0,(long)a1 - (long)a0,&v8,0);
    v1 = dat_2b684;
    v13 = dat_2b7b0;
    if (0 <= v3) { // branch-flip
      v14 = (void *)((long)a0 + v3);
      v4 = (void *)(v8 + (long)v14);
      if (!dat_2b802) {
        if (a1 != v14) {
          if (!dat_2b803) {
            v6 = v14;
            v14 = v4;
            goto label_8847;
          }
          v14 = v4;
label_8a78:
          dat_2b7b0 -= 1;
          dat_2b7a8 = 0;
          dat_2b828 = (long)v14 + (dat_2b830 - dat_2b840);
          goto label_89b3;
        }
        return v2 - v13; // return-dupe
      }
      if (a0 < v14) {
        goto label_88dc;
      }
    }
    else {
      if ((!dat_2b802) || (v8 = 0, a1 <= a0)) {
        return v2 - v13;
      }
      v4 = a1;
      v14 = a1;
label_88dc:
      if (dat_2b803) { // branch-flip
label_88f3:
        v11 = dat_2b802;
        if (!dat_2b802) goto label_8a78;
label_8911:
        v3 = 0;
        v13 = dat_2b7b0;
        if (a0 < v14) {
          for (; v3 < v13; v3 = v3 + 1) {
            v6 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
            if (!v11) {
              sub_82a0(a0,v6,0x3a);
              v13 = dat_2b7b0;
              v11 = dat_2b803;
            }
            a0 = v6;
            if (v14 <= v6) {
              dat_2b7b0 = v13 - (v3 + 1);
              goto label_8984;
            }
          }
          dat_2b7b0 = v13 - v3;
        }
      }
      else {
        v6 = a0;
label_8847:
        a0 = v6;
        if ((0 < (long)dat_2b7a8) && (sub_8710(v6), dat_2b803)) goto label_88f3;
        v7 = dat_2b7b8;
        if (!dat_2b7b8)
          v7 = dat_2b848;
        v5 = 0;
        v12 = v6; // branch-flip
        if (1 <= (long)dat_2b7f8) {
          do {
            a0 = v12;
            if (v7 < v12) {
              do {
                a0 = (void *)((long)v12 - 1);
                if (v1 == *(char *)((long)v12 - 2)) break;
                a0 = (void *)((long)v12 - 2);
                v10 = (char *)((long)v12 - 3);
                v12 = a0;
              } while (v1 != *v10);
            }
            v5 += 1;
            v12 = a0;
          } while (v5 != dat_2b7f8);
          if (((dat_2b688) && (dat_2b7b8 != a0)) && (dat_2b070)) {
label_8b46:
            v10 = dat_2b070;
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1faa0(dat_2b028);
              v10 = dat_2b070;
            }
            if (fputs_unlocked(v10,stdout) <= -1)
              dat_2b8c0 = *__errno_location();
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1fac0(dat_2b020);
              sub_7e70(10);
            }
            else {
              sub_7e70(10);
            }
          }
          if (a0 < v6) {
            do {
              v7 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
              sub_82a0(a0,v7,0x2d);
              a0 = v7;
            } while (v7 < v6);
          }
        }
        else if ((((0 <= (long)(dat_2b7f8 & dat_2b7f0)) && (dat_2b7b8 != v6)) && (dat_2b688)) && (dat_2b070)) goto label_8b46;
        v11 = dat_2b803;
        if (dat_2b802) goto label_8911;
        if (dat_2b803) goto label_8a78;
        sub_82a0(v6,v14,0x3a);
        dat_2b7b0 -= 1;
        a0 = v14;
        v11 = dat_2b803;
      }
label_8984:
      dat_2b828 = (long)a0 + (dat_2b830 - dat_2b840);
      dat_2b7a8 = 0;
      if (!v11) {
        dat_2b7a8 = dat_2b7f0;
        if ((long)dat_2b7f0 <= -1)
          dat_2b7a8 = 0;
      }
label_89b3:
      dat_2b688 = '\x01';
      if ((!dat_2b7b0) || (dat_2b7a2)) {
        v13 = dat_2b7b0;
        if (dat_2b7a1) {
          v9 = 0;
          if (dat_2b889)
            v9 = dat_2b080;
          exit(v9); // no-return
        }
        return v2 - v13;
      }
    }
    a0 = v4;
    v13 = dat_2b7b0;
    if (a1 <= v4)
      return v2 - v13;
  } while( true );
}


// Function: sub_8cb0 @ 0x8cb0
void sub_8cb0(int a0)
{
  char *v1;
  
  if (dat_2b904) {
    dat_2b889 = 1;
    return;
  }
  v1 = dat_2b890;
  if (!dat_2b890) {
    v1 = dcgettext(NULL,"(standard input)",5);
    dat_2b890 = v1;
  }
  error(0,a0,"%s",v1);
  dat_2b889 = 1;
}


// Function: sub_8d20 @ 0x8d20
unsigned long sub_8d20(unsigned long a0,long a1)
{
  unsigned char v1; // al
  char *v10;
  long v11; // rsi
  long v12;
  char *v13;
  long v14;
  unsigned int v15;
  unsigned long v16; // r15
  void *v2;
  unsigned long v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  long v6;
  char *v7;
  long v8; // rbx
  long v9;
  
  v2 = dat_2b860;
  v14 = dat_2b838;
  v9 = dat_2b838 + 8;
  v6 = dat_2b858;
  v13 = dat_2b840;
  if ((long)dat_2b860 + (dat_2b858 - (long)dat_2b840) < v9) {
    v3 = a0 + 1;
    v16 = ~a0;
    v6 = ((dat_2b838 + a0) - dat_2b858) + v9;
    if (1 <= v6) { // branch-flip
      v12 = -1;
      if (((((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000) && (v8 = *(long *)(a1 + 0x30) - dat_2b830, 0 <= v8)) && (v11 = v8 + v9 + a0, !SCARRY8(v8,v9 + a0))) && (v12 = dat_2b858 + v6, dat_2b858 + v6 < v11))
        v12 = v11;
      v2 = (void *)sub_1f1f0(NULL,(long *)0x2b858,v6,v12,1);
      v14 = dat_2b838;
      v13 = (char *)((long)v2 + v3);
      if ((unsigned long)v13 % dat_2b838) { // branch-flip
        v13 = (char *)(((dat_2b838 + 1 + a0) - (unsigned long)v13 % dat_2b838) + (long)v2);
        memmove(&v13[v16],&dat_2b840[v16],v3);
      }
      else {
        memmove(v2,&dat_2b840[v16],v3);
      }
      free(dat_2b860);
      v6 = dat_2b858;
      dat_2b860 = v2;
    }
    else {
      v13 = (char *)((long)dat_2b860 + v3);
      if ((unsigned long)v13 % dat_2b838) { // branch-flip
        v13 = (char *)(((dat_2b838 + 1 + a0) - (unsigned long)v13 % dat_2b838) + (long)dat_2b860);
        memmove(&v13[v16],&dat_2b840[v16],v3);
        v6 = dat_2b858;
      }
      else {
        memmove(dat_2b860,&dat_2b840[v16],v3);
        v6 = dat_2b858;
      }
    }
  }
  v9 = (long)v2 + ((v6 + -8) - (long)v13);
  dat_2b848 = (long)v13 - a0;
  v15 = dat_2b850;
  while (v6 = sub_1da00(v15,v13,v9 - v9 % v14), v15 = dat_2b850, v6 != -1) {
    v10 = &v13[v6];
    v12 = dat_2b830 + v6;
    v1 = dat_2b821 ^ 1 | v6 == 0;
    v3 = (unsigned long)v1;
    dat_2b830 = v12;
    if (v1) goto label_8ea3;
    if (v13 < v10) {
      v7 = v13;
      do {
        if (*v7) {
          v3 = 1;
          goto label_8ea3;
        }
        v7 = &v7[1];
      } while (v10 != v7);
    }
    v8 = v6 + dat_2b818;
    if (SCARRY8(v6,dat_2b818)) {
      error(2,0,dcgettext(NULL,"input is too large to count",5));
      return v4;
    }
    dat_2b818 = v8;
    if (!dat_2b880) {
      v6 = lseek(dat_2b850,v12,3);
      if ((0 <= v6) || (((*__errno_location() == 6 && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000)) && ((v12 < *(long *)(a1 + 0x30) && (v6 = lseek(v15,0,2), 0 <= v6)))))) {
        if (SCARRY8(v6 - v12,v8)) {
          error(2,0,dcgettext(NULL,"input is too large to count",5));
          return v5;
        }
        dat_2b818 = (v6 - v12) + v8;
        dat_2b830 = v6;
      }
      else {
        dat_2b880 = '\x01';
      }
    }
  }
  v3 = 0;
  v10 = v13;
label_8ea3:
  v10[0] = '\0';
  v10[1] = '\0';
  v10[2] = '\0';
  v10[3] = '\0';
  v10[4] = '\0';
  v10[5] = '\0';
  v10[6] = '\0';
  v10[7] = '\0';
  dat_2b840 = v10;
  return v3;
}


// Function: sub_9080 @ 0x9080
long sub_9080(long a0,long *a1)
{
  long v1; // rax
  long *v2;
  long v3; // rdx
  
  v2 = dat_2b8f0;
  if (2 <= dat_2b8e0) {
    v1 = 0x18;
    v3 = 1;
    while (*(long *)((long)dat_2b8f0 + v1) <= a0) {
      v3 += 1;
      v1 += 0x18;
      if (v3 == dat_2b8e0) {
        v2 = &dat_2b8f0[v3 * 3 + -3];
        *a1 = (a0 - *v2) + v2[2];
        return v2[1];
      }
    }
    v2 = (long *)((long)dat_2b8f0 + v1 + -0x18);
  }
  *a1 = (a0 - *v2) + v2[2];
  return v2[1];
}


// Function: sub_9130 @ 0x9130
unsigned long sub_9130(int a0,unsigned char a1)
{
  char *v1;
  void *v10; // rax
  long v11;
  char *v12;
  struct_10 *v13; // rax
  int *v14; // rax
  unsigned long v15; // rax
  int v16;
  stat v17; // stack - 0x168
  char *v18; // stack - 0x178
  stat v19; // stack - 0xd8
  char *v2;
  char v20;
  unsigned int v21; // edx
  long v22; // rax
  long v23;
  unsigned long v24; // r12
  unsigned char v25; // bl
  char v26;
  char *v27;
  char *v28;
  long v29; // stack - 0x1b8
  unsigned long v3;
  long v30; // stack - 0x1b0
  long v31; // stack - 0x1a8
  bool v32; // stack - 0x17b
  unsigned long v33; // stack - 0x170
  char v4;
  char v5;
  char v6;
  short v7;
  unsigned int v8;
  unsigned long v9; // rax
  
  if (fstat(a0,&v17)) { // branch-flip
    v9 = 1;
    sub_8cb0(*__errno_location());
label_98d2:
    if (!a0) {
      return v9 & 0xffffffff; // return-dupe
    }
label_9454:
    if (!close(a0)) {
      return v9 & 0xffffffff;
    }
  }
  else {
    if (a0) {
      if (dat_2b884 != 2) { // branch-flip
        if ((dat_2b884) || (a1 == 1)) {
label_94a4:
          if (a1) {
            if ((v17._24_4_ & 0xf000) != 0x4000) // branch-flip
              v11 = 0x2b8b0;
            else {
              if (dat_2b018 == 4) goto label_944e;
              v11 = 0x2b8a0;
              if (dat_2b88a) goto label_94e0;
            }
            if ((*(long **)(v11 + 8)) && (v6 = sub_16b90(*(long **)(v11 + 8),dat_2b890), v6)) goto label_944e;
          }
        }
        else if (((v17._24_4_ & 0xb000) == 0x2000 || (v17._24_4_ & 0xf000) == 0xc000) || ((v17._24_4_ & 0xf000) == 0x1000)) goto label_944e;
label_94e0:
        if (0 <= (int)dat_2b808) { // branch-flip
          if (dat_2b018 == 3) goto label_9a0e;
label_94fd:
          if ((dat_2b018 == 4) && ((v17._24_4_ & 0xf000) == 0x4000)) goto label_944e;
        }
        else {
          dat_2b808 = (unsigned int)((v17._24_4_ & 0xf000) == 0x4000);
          if (dat_2b018 != 3) goto label_94fd;
label_9a0e:
          if ((v17._24_4_ & 0xf000) == 0x4000) {
            v21 = ~(unsigned int)(a1 ^ 1) & dat_2b014;
            if (close(a0))
              sub_8cb0(*__errno_location());
            v33 = 0;
            v18 = dat_2b890;
            v13 = (struct_10 *)sub_189c0(&v18,v21,0);
            if (!v13)
              sub_1f410(); // no-return
            v9 = 1;
label_9a8d:
            do {
              v11 = sub_18fd0(v13);
              if (!v11) {
                v14 = __errno_location();
                if (*v14)
                  sub_8cb0(*v14);
                if (sub_18de0(v13))
                  sub_8cb0(*v14);
                return v9 & 0xffffffff;
              }
              v7 = *(short *)(v11 + 0x68);
              v25 = *(long *)(v11 + 0x58) == 0 & a1;
            } while (v7 == 6);
            if (!v25) {
              if (((unsigned short)(v7 - 1U) < 2) || (v23 = dat_2b8b0, v7 == 4)) {
                if (dat_2b018 != 4) {
                  v23 = dat_2b8a0;
                  goto label_9f2d;
                }
label_9f64:
                sub_19720(v13,v11,4);
                goto label_9a8d;
              }
label_9f2d:
              if (v23) {
                if (sub_16b90(v23,(char *)(v11 + 0x100))) goto label_9f64;
                v7 = *(short *)(v11 + 0x68);
              }
            }
            dat_2b890 = *(char **)(v11 + 0x38);
            if ((dat_2b88a) && (dat_2b890[1]))
              dat_2b890 = &dat_2b890[2];
            v12 = dat_2b890;
            v21 = 1;
            if (!(v13->field_0x48 & 2))
              v21 = v13->field_0x48 & (unsigned int)v25;
            switch(v7) {
              default:
                goto label_9f9b;
              case 1:
                if (dat_2b018 == 3) goto label_9a8d;
                sub_19720(v13,v11,4);
                break;
              case 2:
                if (!dat_2b904)
                  error(0,0,dcgettext(NULL,"%s: warning: recursive directory loop",5),v12);
                goto label_9a8d;
              case 3:
              case 0xb:
                if ((dat_2b884 == 2) || ((!v25 && (!dat_2b884)))) {
                  v8 = *(unsigned int *)(v11 + 0x88);
                  if (!*(unsigned int *)(v11 + 0x88)) {
                    v12 = *(char **)(v11 + 0x30);
                    v16 = v13->field_0x2c;
                    v8 = v19._24_4_;
                    if (fstatat(v16,v12,&v19,v21 << 8 ^ 0x100)) {
                      sub_8cb0(*__errno_location());
                      goto label_9a8d;
                    }
                  }
                  if (((v8 & 0xb000) == 0x2000 || (v8 & 0xf000) == 0xc000) || ((v8 & 0xf000) == 0x1000)) goto label_9a8d;
                }
                break;
              case 4:
              case 7:
              case 10:
                sub_8cb0(*(unsigned int *)(v11 + 0x40));
                goto label_9a8d;
              case 8:
              case 0xd:
                break;
              case 0xc:
              case 0xe:
                goto label_9a8d;
              
            }
            v3 = *(unsigned long *)(v11 + 0x30);
            v16 = v13->field_0x2c;
            v9 = (unsigned long)((unsigned int)v9 & sub_a0f0(v16,v3,v21,v25));
            goto label_9a8d;
          }
        }
        if (((dat_2b884 != 2) && ((dat_2b884 || (a1)))) || (((v17._24_4_ & 0xb000) != 0x2000 && (v17._24_4_ & 0xf000) != 0xc000 && ((v17._24_4_ & 0xf000) != 0x1000)))) goto label_9191;
      }
      else if (((v17._24_4_ & 0xb000) != 0x2000 && (v17._24_4_ & 0xf000) != 0xc000) && ((v17._24_4_ & 0xf000) != 0x1000)) goto label_94a4;
label_944e:
      v9 = 1;
      goto label_9454;
    }
    if ((int)dat_2b808 < 0)
      dat_2b808 = (unsigned int)((v17._24_4_ & 0xf000) == 0x4000);
label_9191:
    v5 = dat_2b803;
    v4 = dat_2b7a2;
    v6 = dat_2b684;
    if ((((!dat_2b803) && (!dat_2b80c)) && (2 <= dat_2b7e0)) && ((((v17._24_4_ & 0xf000) == 0x8000 && (v17._8_8_ == dat_2b928)) && (v17._0_8_ == dat_2b920)))) {
      if (!dat_2b904) {
        v12 = (char *)sub_90f0();
        error(0,0,dcgettext(NULL,"%s: input file is also the output",5),v12);
      }
      dat_2b889 = 1;
      v9 = 1;
      goto label_98d2;
    }
    dat_2b840 = (char *)(dat_2b860 + 1);
    if ((unsigned long)dat_2b840 % dat_2b838)
      dat_2b840 = (char *)(dat_2b860 + ((dat_2b838 + 1) - (unsigned long)dat_2b840 % dat_2b838));
    dat_2b840[-1] = dat_2b684;
    dat_2b848 = dat_2b840;
    dat_2b850 = a0;
    if (a0) { // branch-flip
      dat_2b830 = 0;
      dat_2b881 = 0;
      dat_2b880 = 0;
label_9228:
      dat_2b7c8 = NULL;
      dat_2b7b8 = NULL;
      dat_2b7b0 = dat_2b7e0;
      dat_2b821 = v6 == '\0' & dat_2b820;
      dat_2b818 = 0;
      dat_2b828 = 0;
      dat_2b7a8 = 0;
      dat_2b888 = '\0';
      v32 = sub_8d20(0,&v17);
      if (v32) { // branch-flip
        dat_2b8f8 = 0;
        if (dat_2b8fc) {
          v11 = 0x7fffffffffffffff;
          if (((v17._24_4_ & 0xf000) == 0x8000) && (v11 = v17._48_8_, dat_2b801))
            v11 = v17._48_8_ + (unsigned long)(v17._48_8_ != 0x7fffffffffffffff);
          dat_2b8f8 = 0;
          do {
            dat_2b8f8 += 1;
            v11 /= 10;
          } while (v11);
        }
        v30 = -1;
        v29 = 0;
        v31 = 0;
        v24 = 0;
        v20 = '\0';
        v26 = v32;
        do {
          v28 = dat_2b848;
          v12 = dat_2b840;
          v16 = dat_2b810;
          v25 = (unsigned char)((unsigned long)v30 >> 0x3f) & v6 != '\0';
          v9 = (unsigned long)v25;
          if (v25) {
            if (dat_2b810 != 1) {
              *dat_2b840 = '\0';
              v22 = (long)dat_2b840 - (long)dat_2b848;
              if (v22 != strlen(dat_2b848)) { // branch-flip
label_9355:
                if (v16 == 2) {
                  v32 = 0;
                  v31 = 0;
                  goto label_96ef;
                }
                if (!dat_2b7e8) {
                  dat_2b803 = '\x01';
                  dat_2b7a2 = '\x01';
                }
                dat_2b821 = dat_2b820;
                v30 = v31;
                v28 = dat_2b848;
                v12 = dat_2b840;
                v20 = v6;
              }
              else if (v26) {
                if (!dat_2b881) {
                  if ((v17._24_4_ & 0xf000) == 0x8000) {
                    if (v22 < (long)v17._48_8_) {
                      if (!a0) {
                        v22 = lseek(0,0,1);
                        if (0 > v22) goto label_939e;
                      }
                      v11 = lseek(a0,v22,4);
                      if (0 <= v11) {
                        if ((long)lseek(a0,v22,0) < 0)
                          sub_8cb0(*__errno_location());
                        if (v11 < (long)v17._48_8_) {
                          v16 = dat_2b810;
                          goto label_9355;
                        }
                        v28 = dat_2b848;
                        v12 = dat_2b840;
                      }
                    }
                  }
                }
              }
            }
          }
label_939e:
          if (dat_2b7b8)
            dat_2b7b8 = v28;
          v2 = &v28[v24];
          dat_2b7c0 = v28;
          if (v2 == v12) {
            if (v29) {
              dat_2b840 = &v12[1];
              *v12 = v6;
              if (dat_2b7b0)
                v31 += sub_8790(&v28[v24 - v29]);
              if (dat_2b7a8)
                sub_8710(dat_2b840);
            }
            goto label_96b9;
          }
          if (v20) {
            v27 = v2;
            while( true ) {
              *v12 = '\0';
              v9 = strlen(v27);
              *v12 = v20;
              v28 = &v27[v9];
              if (&v27[v9] == v12) break;
              do {
                v27 = &v28[1];
                *v28 = v20;
                v28 = v27;
              } while (!*v27);
            }
          }
          v26 = v2[-1];
          v2[-1] = v6;
          v10 = memrchr(&v2[-1],(int)v6,(unsigned long)&v12[1 - (long)v2]);
          v2[-1] = v26;
          v27 = &v2[-v29];
          v28 = (char *)((long)v10 + 1);
          if (v2 != v28) { // branch-flip
            v29 = (long)v12 - (long)v28;
            if (v27 < v28) {
              if (!dat_2b7b0) {
                if (dat_2b7a8) goto label_95c2;
label_97dd:
                v32 = 0;
                goto label_96b9;
              }
              v31 += sub_8790(v27,v28);
              if (dat_2b7a8) { // branch-flip
label_95c2:
                sub_8710(v28);
                if (!dat_2b7b0 && !dat_2b7a8) goto label_97dd;
              }
              else if (!dat_2b7b0) goto label_97dd;
              if (dat_2b7a2) {
                v11 = 0;
                if (0 <= v30)
                  v11 = v30;
                if (v11 < v31) goto label_97dd;
              }
            }
          }
          else {
            v29 = (long)v12 - (long)v27;
            v28 = v27;
          }
          if (1 <= dat_2b7f8) {
            v11 = 0;
            do {
              if (v28 <= dat_2b848) break;
              if (v28 == dat_2b7b8) goto label_964f;
              v11 += 1;
              v27 = v28;
              do {
                v28 = &v27[-1];
                v1 = &v27[-2];
                v27 = v28;
              } while (v6 != *v1);
            } while (v11 != dat_2b7f8);
          }
          if (v28 != dat_2b7b8)
            dat_2b7b8 = NULL;
label_964f:
          v24 = (long)v12 - (long)v28;
          if (dat_2b800) {
            v12 = &(&dat_2b840[-v24 - (long)dat_2b848])[(long)dat_2b7c8];
            if (SCARRY8((long)&dat_2b840[-v24 - (long)dat_2b848],(long)dat_2b7c8)) {
              error(2,0,dcgettext(NULL,"input is too large to count",5));
              return v15;
            }
            dat_2b7c8 = v12;
          }
          if (dat_2b801)
            sub_7c90(v28);
          v26 = 0;
        } while (sub_8d20(v24,&v17));
        sub_8cb0(*__errno_location());
        v32 = 0;
label_96b9:
        v9 = CONCAT71((undefined7)((unsigned long)v2 >> 8),v31 == 0);
        dat_2b7a2 = v4;
        dat_2b803 = v5;
        if (((!dat_2b810) && (!v5)) && ((dat_2b888 || ((0 <= v30 && (v30 < v31)))))) {
          v12 = dat_2b890;
          if (!dat_2b890) {
            v12 = dcgettext(NULL,"(standard input)",5);
            dat_2b890 = v12;
          }
          error(0,0,dcgettext(NULL,"%s: binary file matches",5),v12);
        }
      }
      else {
        v9 = 1;
        sub_8cb0(*__errno_location());
        v31 = 0;
      }
    }
    else {
      dat_2b830 = lseek(0,0,1);
      dat_2b880 = (unsigned char)((unsigned long)dat_2b830 >> 0x3f);
      dat_2b881 = dat_2b880;
      if (0 <= dat_2b830) goto label_9228;
      v16 = *__errno_location();
      if (v16 == 0x1d) {
        dat_2b830 = 0;
        goto label_9228;
      }
      sub_8cb0(v16);
      v9 = 1;
      v32 = 0;
      v31 = 0;
    }
label_96ef:
    if (dat_2b7e8) {
      if (dat_2b808) {
        sub_7fa0();
        if (dat_2b804) // branch-flip
          sub_7ec0(0x3a);
        else {
          sub_7e70(0);
        }
      }
      sub_7b80("%ld\n",v31);
      if ((dat_2b7d8) && (fflush_unlocked(stdout)))
        dat_2b8c0 = *__errno_location();
    }
    if (dat_2b80c) {
      if (dat_2b80c == (v31 == 0) + 1) {
        sub_7fa0();
        sub_7e70(dat_2b804 & 10);
        if ((dat_2b7d8) && (fflush_unlocked(stdout)))
          dat_2b8c0 = *__errno_location();
      }
      goto label_98d2;
    }
    if (a0) goto label_9454;
    if (!dat_2b7b0) {
      if (((dat_2b830 == dat_2b828) || (dat_2b881)) || (0 <= (long)lseek(0,dat_2b828,0))) {
        return v9 & 0xffffffff;
      }
      sub_8cb0(*__errno_location()); // return-dupe
      return v9 & 0xffffffff;
    }
    if ((v32) || ((!dat_2b881 && ((0 <= (long)lseek(0,0,2) || (*__errno_location() == 0x16)))))) {
      return v9 & 0xffffffff;
    }
    if (((v17._24_4_ & 0xf000) != 0x1000) || (!dat_2b7a0)) {
label_9b90:
      do {
        v11 = sub_1da00(0,dat_2b860,dat_2b858);
        if (!v11) {
          return v9 & 0xffffffff;
        }
      } while (v11 != -1);
    }
    else {
      v11 = splice(0,NULL,1,NULL,0x18000,1);
      if (0 <= v11) { // branch-flip
        if (!v11) {
          return v9 & 0xffffffff;
        }
        do {
          v11 = splice(0,NULL,1,NULL,0x18000,1);
        } while (0 < v11);
        if (!v11) {
          return v9 & 0xffffffff;
        }
      }
      else if (*__errno_location() == 0x16) goto label_9b90;
    }
  }
  sub_8cb0(*__errno_location());
  return v9 & 0xffffffff;
label_9f9b:
  abort(); // no-return
}


// Function: sub_a0f0 @ 0xa0f0
unsigned long sub_a0f0(unsigned long a0,char *a1,bool a2,char a3) // early-return
{
  int v1;
  unsigned long v2; // rax
  int *v3; // rax
  unsigned int v4; // edx
  
  v4 = (-(unsigned int)(a2 == 0) & 0x20000) + 0x100;
  if ((dat_2b884 == 2) || ((!dat_2b884 && (a3 != '\x01'))))
    v4 |= 0x800;
  v1 = sub_1b5d0(a0,a1,v4);
  if (v1 <= -1) {
    v3 = __errno_location();
    v1 = *v3;
    if ((!a2) && (v1 == 0x28 || v1 == 0x1f))
      return CONCAT71((undefined7)((unsigned long)v3 >> 8),v1 == 0x28 || v1 == 0x1f);
    sub_8cb0(v1);
    return 1;
  }
  v2 = sub_9130(v1,(unsigned char)a3); // tail-call
  return v2;
}


// Function: sub_a170 @ 0xa170
void sub_a170(int a0)
{
  char *v1; // rax
  
  v1 = (char *)sub_19950();
  if (a0) { // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    v1 = (char *)sub_19950();
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v1);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"Search for PATTERNS in each FILE.\n",5));
    v1 = (char *)sub_19950();
    __printf_chk(1,dcgettext(NULL,"Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n",5));
    __printf_chk(1,dcgettext(NULL,"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n",5));
    __printf_chk(1,dcgettext(NULL,"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n",5));
    __printf_chk(1,dcgettext(NULL,"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n",5));
    __printf_chk(1,dcgettext(NULL,"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n",5));
    __printf_chk(1,dcgettext(NULL,"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n",5));
    __printf_chk(1,dcgettext(NULL,"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n",5));
    __printf_chk(1,dcgettext(NULL,"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n",5));
    __printf_chk(1,dcgettext(NULL,"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n",5));
    __printf_chk(1,dcgettext(NULL,"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n",5));
    __printf_chk(1,dcgettext(NULL,"When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n",5));
    sub_1ee20();
  }
  exit(a0); // no-return
}


// Function: sub_a3e0 @ 0xa3e0
void sub_a3e0(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1;
  unsigned char v2;
  unsigned char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x48
  unsigned char *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned char *v9; // stack - 0x60
  
  v8 = *a1;
  v6 = (unsigned char *)*a0;
  v5 = 0;
  v3 = (unsigned char *)sub_1f050(v8 + 1,2);
  v9 = v3;
  if (v8) { // branch-flip
    do {
      while( true ) {
        v4 = (unsigned long)*(char *)((unsigned long)*v6 + 0x2b183);
        if (*(char *)((unsigned long)*v6 + 0x2b183) == '\xfe')
          v4 = sub_1ac90(v6,v8,&v5);
        if (v4 != 0xffffffffffffffff) break;
        v5 = 0;
label_a49c:
        v2 = *v6;
        if (((unsigned char)((unsigned int)v2 - 0x24) <= 0x3a) && (0x580000000000441U >> ((unsigned long)((unsigned int)v2 - 0x24) & 0x3f) & 1)) {
          *v9 = 0x5c;
          v2 = *v6;
          v9 = &v9[1];
        }
        v8 -= 1;
        *v9 = v2;
        v9 = &v9[1];
        v6 = &v6[1];
        if (!v8) goto label_a4de;
      }
      if (v4 == 1) goto label_a49c;
      v7 = v8;
      v1 = 0;
      if (v4 != 0xfffffffffffffffe) {
        v7 = v4;
        v1 = v8 - v4;
      }
      v8 = v1;
      v9 = mempcpy(v9,v6,v7);
      v6 = &v6[v7];
    } while (v8);
label_a4de:
    v8 = (long)v9 - (long)v3;
  }
  else {
    v8 = 0;
  }
  *v9 = 10;
  free((void *)*a0);
  *a0 = v3;
  *a1 = v8;
}


// Function: sub_a570 @ 0xa570
void sub_a570(char *a0,long a1)
{
  char v1;
  unsigned long v2; // rax
  void *v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  unsigned long v7; // stack - 0x48
  unsigned long v8; // r13
  char *v9;
  
  v4 = NULL;
  v7 = 0;
  v2 = sub_e5c0(1);
  v9 = a0;
  do {
    v3 = rawmemchr(v9,10);
    v8 = (long)v3 - (long)v9;
    if (dat_2b685) {
      if ((v9 <= a0) || (dat_2b684 != '\n')) {
        if ((long)v7 <= (long)(v8 + 1)) {
          free(v4);
          v7 = v8;
          v4 = (char *)sub_1f1f0(NULL,&v7,2,-1,1);
          *v4 = dat_2b684;
        }
        v1 = dat_2b684;
        memcpy(&v4[1],v9,v8);
        v4[v8 + 1] = v1;
        v9 = v4;
      }
      else {
        v9 = &v9[-1];
      }
      v8 += 2;
    }
    sub_c9d0(v2,v9,v8);
    v9 = (char *)((long)v3 + 1U);
  } while ((char *)((long)v3 + 1U) <= &a0[a1]);
  free(v4);
  v5 = sub_ceb0(v2);
  sub_cec0(v2);
  v6 = (unsigned long *)sub_1ef10(0x28);
  *v6 = v2;
  v6[2] = a0;
  v6[1] = v5;
  v6[3] = a1;
  v6[4] = 0;
}


// Function: sub_a710 @ 0xa710
long sub_a710(struct_29 *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  long v1;
  void *v10;
  unsigned char v11; // cl
  unsigned char v12; // cl
  unsigned int v13; // edx
  char v14 [16];
  char v15 [16];
  long v16; // stack - 0x80
  void *v17; // stack - 0x88
  void *v18;
  long v19;
  struct_28 *v2;
  long v20; // stack - 0x68
  long v21; // stack - 0x48
  unsigned long v3;
  char *v4;
  char v5;
  unsigned int v6; // eax
  long v7;
  void *v8;
  long v9; // rax
  
  v5 = dat_2b684;
  v18 = (void *)((long)a1 + a2);
  v1 = a0->field_0x0;
  v6 = (unsigned int)dat_2b685;
  v11 = dat_2b180 & ((dat_2b182 | dat_2b685) ^ 1);
  v12 = v11 | dat_2b686;
  v13 = v6 ^ 1;
  v10 = a1;
  if (a4)
    v10 = a4;
  if (v18 < v10)
    return -1;
  v17 = v10;
  while( true ) {
    v7 = sub_e3e0(v1,(long)v10 - (unsigned long)v6,(long)v18 + ((unsigned long)v6 - (long)v10),v14,(v12 | a4 != NULL) & v13);
    if (v7 <= -1)
      return -1;
    v7 = (long)v10 + v7;
    v16 = 0;
    v19 = v20 - (int)((unsigned int)dat_2b685 * 2);
    if ((!v11) || (!sub_e640(&v17,&v16,v7,v18))) break;
    v10 = v17;
    if (v18 < v17)
      return -1;
label_a98c:
    v6 = (unsigned int)dat_2b685;
  }
  v10 = v17;
  if ((dat_2b686 == 1) || (!a4)) {
    if (!dat_2b685) {
      if (!dat_2b686) {
        v10 = (void *)(v7 + v19);
label_aa4d:
        if (v10 < v18)
          v18 = (void *)((long)rawmemchr(v10,(int)v5) + 1);
label_aa67:
        v10 = memrchr(a1,(int)v5,v7 - (long)a1);
        if (v10) { // branch-flip
          v7 = ((long)v10 + 1) - (long)a1;
          a1 = (void *)((long)v10 + 1);
        }
        else {
          v7 = 0;
        }
        v19 = (long)v18 - (long)a1;
        goto label_aa18;
      }
      if (v16) { // branch-flip
        if (v16 <= 0) goto label_a894;
        v9 = sub_e820(v7 - v16,v18);
      }
      else {
        v8 = memrchr(v17,(int)v5,v7 - (long)v17);
        if (v8) {
          v10 = (void *)((long)v8 + 1);
          v17 = v10;
        }
label_a894:
        v9 = sub_e830(v10,v7,v18);
      }
      if (!v9) {
        while( true ) {
          v10 = (void *)(v7 + v19);
          if (!sub_e820(v10,v18)) {
            if (!a4) goto label_aa4d;
            goto label_aa10;
          }
          if ((!a4) && (!dat_2b180)) break;
          if ((!v19) || (sub_e3e0(v1,v7,v19 + -1,v15,1))) goto label_a970;
          v19 = v21;
        }
        if (!a0->field_0x20) {
          sub_a3e0(&a0->field_0x10,&a0->field_0x18);
          v3 = a0->field_0x18;
          v4 = a0->field_0x10;
          a0->field_0x20 = sub_6a70(v4,v3,0x10a46,0);
        }
        v8 = v18;
        if (v10 < v18)
          v8 = (void *)((long)rawmemchr(v10,(int)v5) + 1);
        v2 = (struct_28 *)a0->field_0x20;
        if (0 <= (long)sub_7110(v2,v7,(long)v8 - v7,a3,NULL)) {
          v18 = v8;
          goto label_aa67;
        }
        v7 = (long)v8 - 1;
      }
label_a970:
      v17 = (void *)(v7 + sub_e810(v7,v18));
      v10 = (void *)((long)v17 + 1);
      if (v18 < (void *)((long)v17 + 1))
        return -1;
      goto label_a98c;
    }
    v19 += (unsigned long)(a4 == NULL);
  }
label_aa10:
  v7 -= (long)a1;
label_aa18:
  *a3 = v19;
  return v7;
}


// Function: sub_ab10 @ 0xab10
void sub_ab10(struct_21 *a0,long a1,long a2,char a3)
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  long *v4;
  long *v5;
  unsigned long *v6;
  
  if (!a0)
    return;
  sub_ab10(a0->field_0x0,a1,a2,(unsigned char)a3);
  sub_ab10(a0->field_0x8,a1,a2,(unsigned char)a3);
  do {
    if (!a1) {
      a0->field_0x10[4] = a2;
      return;
    }
    v6 = *(unsigned long **)(a1 + 8);
    if (v6) {
      do {
        v2 = (unsigned char *)&v6[3];
        if (a0->field_0x18 == *v2) {
          v4 = a0->field_0x10;
          v5 = (long *)v6[2];
          v4[4] = (long)v5;
          if (a3)
            return;
          if (!*v5)
            return;
          if (*v4)
            return;
          *v4 = -1;
          return;
        }
        v3 = (unsigned long *)*v6;
        v1 = &v6[1];
        v6 = v3;
        if (*v2 <= a0->field_0x18)
          v6 = (unsigned long *)*v1;
      } while (v6);
    }
    a1 = *(long *)(a1 + 0x20);
  } while( true );
}


// Function: sub_abd0 @ 0xabd0
unsigned long sub_abd0(unsigned long *a0,struct_21 *a1) // return-dupe
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  
  if (!a1)
    return 1;
  v4 = sub_abd0(a0,a1->field_0x0);
  if (((char)v4) && (v4 = sub_abd0(a0,a1->field_0x8), (char)v4)) {
    if (!a0)
      return 0;
    do {
      v2 = (unsigned char *)&a0[3];
      if (a1->field_0x18 == *v2)
        return v4;
      v3 = (unsigned long *)*a0;
      v1 = &a0[1];
      a0 = v3;
      if (*v2 <= a1->field_0x18)
        a0 = (unsigned long *)*v1;
    } while (a0);
    return 0;
  }
  return v4;
}


// Function: sub_ac50 @ 0xac50
unsigned char * sub_ac50(unsigned char *a0,long a1,long a2) // return-dupe
{
  unsigned char *v1;
  int v2;
  unsigned int v3;
  
  v2 = *(int *)(a2 + 0x98c);
  v1 = &a0[a1];
  if (0 <= v2) { // branch-flip
    while( true ) {
      if (v1 <= a0)
        return NULL;
      if (*(long *)(a2 + 0x170 + (unsigned long)*a0 * 8)) break;
      a0 = &a0[1];
      if (!((unsigned long)a0 & 0xf)) {
        v3 = *(unsigned int *)(a2 + 0x988);
        return (unsigned char *)sub_1b140(a0,v3,v2,(long)v1 - (long)a0); // tail-call
      }
    }
  }
  else {
    if (v1 <= a0)
      return NULL;
    while (!*(long *)(a2 + 0x170 + (unsigned long)*a0 * 8)) {
      a0 = &a0[1];
      if (v1 == a0)
        return NULL;
    }
  }
  return a0;
}


// Function: sub_af60 @ 0xaf60
long sub_af60(void)
{
  return sub_ace0(); // tail-call
}


// Function: sub_af70 @ 0xaf70
long sub_af70(long a0,long a1,long a2) // return-dupe
{
  long v1;
  long v10; // rbp
  unsigned char v11;
  long v12; // rdi
  long v13;
  unsigned long v14; // rcx
  unsigned long v15;
  unsigned char v2;
  unsigned char v3;
  long v4;
  long v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned long v9;
  
  v7 = *(long *)(a0 + 0x68);
  if (!v7)
    return 0;
  v4 = *(long *)(a0 + 0x980);
  if (a2 < v7)
    return -1;
  if (v7 == 1) {
    v7 = sub_ac50(a1,a2,a0);
    if (v7)
      return v7 - a1;
    return -1;
  }
  v1 = a0 + 0x70;
  v2 = *(unsigned char *)(a0 + 0x988);
  v3 = *(unsigned char *)(a0 + 0x990);
  v10 = *(long *)(a0 + 0x970) + v7;
  v9 = a1 + v7;
  if ((SEXT816(v7 * 0xc) == SEXT816(v7) * SEXT816(0xc)) && (v7 * 0xc < a2)) {
    v14 = a2 + v7 * -0xb + a1;
    v15 = v9;
    if (v9 <= v14) {
label_b168:
      v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v15 - 1)) + v15;
      v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1));
      v9 = v6 + (unsigned long)v8;
      if (((!v8) || (v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1)) + v9, v6 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v9 = (unsigned long)v8 + v6, !v8)) || (v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1)) + v9, v6 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v9 = (unsigned long)v8 + v6, !v8)) {
label_b260:
        v6 = 0;
        v13 = v7;
label_b2ae:
        v8 = *(unsigned char *)(v9 - 2);
        if (v4)
          v8 = *(unsigned char *)(v4 + (unsigned long)v8);
        if (v3 == v8) {
          v12 = -3;
          v5 = 2;
          do {
            v5 += 1;
            if (v13 < v5) {
              v5 = v13 + v6 + 1;
              if (v7 < v5) {
                return (v9 - v7) - a1;
              }
              v15 = ~(v13 + v6);
              goto label_b321;
            }
            v8 = *(unsigned char *)(v9 + v12);
            v11 = *(unsigned char *)(v10 + v12);
            if (v4) {
              v8 = *(unsigned char *)(v4 + (unsigned long)v8);
              v11 = *(unsigned char *)(v4 + (unsigned long)v11);
            }
            v12 -= 1;
          } while (v11 == v8);
          goto label_b347;
        }
        v6 = 0;
        v5 = 2;
        goto label_b277;
      }
      v6 = v9 + *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1));
      v9 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1));
      if (0x7f < (long)(v9 - v15)) goto label_b378;
      v6 = sub_ac50(v9 - 1,(a1 + a2) - (v9 - 1),a0);
      if (!v6)
        return -1;
      v9 = v6 + 1;
      if (v9 < v14) goto label_b260;
      goto label_affe;
    }
  }
label_affe:
  v15 = (unsigned long)*(unsigned char *)(a0 + 0x70 + (unsigned long)*(unsigned char *)(v9 - 1));
label_b010:
  do {
    if ((long)((a2 + a1) - v9) < (long)v15)
      return -1;
    v9 += v15;
    v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1));
    v15 = (unsigned long)v8;
  } while (v8);
  v6 = 0;
  v13 = v7;
label_b071:
  v8 = *(unsigned char *)(v9 - 2);
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v3 == v8) {
    v12 = -3;
    v5 = 2;
    do {
      v5 += 1;
      if (v13 < v5) {
        v5 = v13 + v6 + 1;
        if (v7 < v5) {
          return (v9 - v7) - a1;
        }
        v14 = ~(v13 + v6);
        goto label_b0e9;
      }
      v8 = *(unsigned char *)(v9 + v12);
      v11 = *(unsigned char *)(v10 + v12);
      if (v4) {
        v8 = *(unsigned char *)(v4 + (unsigned long)v8);
        v11 = *(unsigned char *)(v4 + (unsigned long)v11);
      }
      v12 -= 1;
    } while (v11 == v8);
    goto label_b10f;
  }
  v6 = 0;
  v5 = 2;
  goto label_b047;
label_b321:
  v8 = *(unsigned char *)(v9 + v15);
  v11 = *(unsigned char *)(v10 + v15);
  if (v4) {
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
    v11 = *(unsigned char *)(v4 + (unsigned long)v11);
  }
  if (v11 != v8) goto label_b33e;
  v5 += 1;
  v15 -= 1;
  if (v7 < v5) {
    return (v9 - v7) - a1;
  }
  goto label_b321;
label_b33e:
  if (v7 < v5)
    return (v9 - v7) - a1;
label_b347:
  v6 = v5 * 8 + -0x10;
label_b277:
  v13 = *(long *)(*(long *)(a0 + 0x978) + v6);
  v9 += v13;
  v8 = *(unsigned char *)(v9 - 1);
  v15 = (unsigned long)v8;
  if (v14 < v9) goto label_affe;
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v2 != v8) goto label_b370;
  v6 = v5 + -1;
  goto label_b2ae;
label_b370:
  v9 = *(unsigned char *)(v1 + v15) + v9;
label_b378:
  v15 = v9;
  if (v14 < v9) goto label_affe;
  goto label_b168;
label_b0e9:
  v8 = *(unsigned char *)(v9 + v14);
  v11 = *(unsigned char *)(v10 + v14);
  if (v4) {
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
    v11 = *(unsigned char *)(v4 + (unsigned long)v11);
  }
  if (v11 != v8) goto label_b106;
  v5 += 1;
  v14 -= 1;
  if (v7 < v5) {
    return (v9 - v7) - a1;
  }
  goto label_b0e9;
label_b106:
  if (v7 < v5)
    return (v9 - v7) - a1;
label_b10f:
  v6 = v5 * 8 + -0x10;
label_b047:
  v13 = *(long *)(*(long *)(a0 + 0x978) + v6);
  v9 += v13;
  if ((unsigned long)(a2 + a1) < v9) goto label_b010;
  v8 = *(unsigned char *)(v9 - 1);
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v2 != v8) goto label_b010;
  v6 = v5 + -1;
  goto label_b071;
}


// Function: sub_b3f0 @ 0xb3f0
void sub_b3f0(long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = sub_af70();
  v1 = *(unsigned long *)(a0 + 0x68);
  *a3 = 0;
  a3[1] = v2;
  a3[2] = v1;
}


// Function: sub_c880 @ 0xc880
void * sub_c880(unsigned long a0)
{
  void *v1; // rax
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  
  v1 = (void *)sub_1ef10(0x9a0);
  _obstack_begin(v1,0,0,sub_1ef10,dat_2afb8);
  v4 = *(unsigned long *)((long)v1 + 0x20);
  *(unsigned long *)((long)v1 + 0x58) = 0;
  if (0x40 <= v4 - *(long *)((long)v1 + 0x18)) { // branch-flip
    v5 = *(unsigned long **)((long)v1 + 0x10);
    v2 = (unsigned long *)(*(long *)((long)v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  else {
    _obstack_newchunk(v1,0x40);
    v5 = *(unsigned long **)((long)v1 + 0x10);
    v4 = *(unsigned long *)((long)v1 + 0x20);
    v2 = (unsigned long *)(*(long *)((long)v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  *(unsigned char *)((long)v1 + 0x50) = *(unsigned char *)((long)v1 + 0x50) | 2;
label_c8ea:
  *(unsigned long **)((long)v1 + 0x60) = v5;
  v3 = (long)v2 + *(unsigned long *)((long)v1 + 0x30) & ~*(unsigned long *)((long)v1 + 0x30);
  if (v3 - *(long *)((long)v1 + 8) <= v4 - *(long *)((long)v1 + 8))
    v4 = v3;
  *(unsigned long *)((long)v1 + 0x18) = v4;
  *(unsigned long *)((long)v1 + 0x10) = v4;
  *v5 = 0;
  v5[1] = 0;
  v5[2] = 0;
  v5[3] = 0;
  v5[4] = 0;
  v5[5] = 0;
  v5[6] = 0;
  *(unsigned long *)((long)v1 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)((long)v1 + 0x980) = a0;
  *(void **)((long)v1 + 0x998) = sub_af60;
  *(unsigned long *)((long)v1 + 0x970) = 0;
  return v1;
}


// Function: sub_c9d0 @ 0xc9d0
void sub_c9d0(void *a0,unsigned char *a1,long a2) // return-dupe
{
  unsigned char v1;
  long *v10;
  long *v11;
  long v12;
  int v13 [13]; // stack - 0xdc
  long *v14 [13]; // stack - 0xa8
  unsigned char v15;
  unsigned long v16;
  int v17;
  long v18;
  unsigned long *v19;
  void *v2;
  unsigned long v20;
  unsigned long v21; // stack - 0xf8
  unsigned long v22; // stack - 0xf0
  long v3;
  long *v4;
  long v5;
  long v6;
  long *v7;
  char v8;
  unsigned long *v9; // rax
  
  v2 = *(void **)((long)a0 + 0x998);
  v3 = *(long *)((long)a0 + 0x980);
  v7 = *(long **)((long)a0 + 0x60);
  if (v2 == sub_b3f0)
    a1 = &a1[a2];
label_ca25:
  do {
    while( true ) {
      if (!a2) {
        v3 = *(long *)((long)a0 + 0x58);
        if (!*v7)
          *v7 = v3 * 2 + 1;
        *(long *)((long)a0 + 0x58) = v3 + 1;
        if (*(long *)((long)a0 + 0x68) <= v7[5])
          return;
        *(long *)((long)a0 + 0x68) = v7[5];
        return;
      }
      a2 -= 1;
      if (v2 != sub_b3f0) { // branch-flip
        v15 = *a1;
        a1 = &a1[1];
      }
      else {
        v15 = a1[-1];
        a1 = &a1[-1];
      }
      if (v3)
        v15 = *(unsigned char *)(v3 + (unsigned long)v15);
      v10 = (long *)v7[1];
      v13[1] = 0;
      v14[0] = &v7[1];
      v12 = 1;
      if (!v10) break;
      while (v1 = *(unsigned char *)&v10[3], v1 != v15) {
        v14[v12] = v10;
        v18 = v12 + 1;
        if (v15 <= v1) { // branch-flip
          v10 = (long *)*v10;
          v13[v18] = 0;
        }
        else {
          v10 = (long *)v10[1];
          v13[v18] = 1;
        }
        if (!v10) goto label_cabc;
        v12 = v18;
      }
      v7 = (long *)v10[2];
    }
    v12 = 0;
label_cabc:
    v16 = *(unsigned long *)((long)a0 + 0x20);
    if (0x20 <= v16 - *(long *)((long)a0 + 0x18)) { // branch-flip
      v19 = *(unsigned long **)((long)a0 + 0x10);
      v9 = (unsigned long *)(*(long *)((long)a0 + 0x18) + 0x20);
      if (v9 == v19) {
label_cdc7:
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      }
    }
    else {
      _obstack_newchunk(a0,0x20);
      v19 = *(unsigned long **)((long)a0 + 0x10);
      v16 = *(unsigned long *)((long)a0 + 0x20);
      v9 = (unsigned long *)(*(long *)((long)a0 + 0x18) + 0x20);
      if (v9 == v19) goto label_cdc7;
    }
    v20 = *(unsigned long *)((long)a0 + 0x30);
    v22 = ~v20;
    v10 = (long *)((long)v9 + v20 & v22);
    v18 = *(long *)((long)a0 + 8);
    v21 = v16 - v18;
    if ((unsigned long)((long)v10 - v18) <= v21) { // branch-flip
      *(long **)((long)a0 + 0x18) = v10;
      *(long **)((long)a0 + 0x10) = v10;
      *v19 = 0;
      v19[1] = 0;
      if (v16 - (long)v10 <= 0x3f) goto label_cb2f;
      v11 = &v10[8];
      if (v11 == v10) goto label_cd6f;
    }
    else {
      *(unsigned long *)((long)a0 + 0x18) = v16;
      *(unsigned long *)((long)a0 + 0x10) = v16;
      *v19 = 0;
      v19[1] = 0;
label_cb2f:
      _obstack_newchunk(a0,0x40);
      v20 = *(unsigned long *)((long)a0 + 0x30);
      v16 = *(unsigned long *)((long)a0 + 0x20);
      v22 = ~v20;
      v11 = (long *)(*(long *)((long)a0 + 0x18) + 0x40);
      v18 = *(long *)((long)a0 + 8);
      v21 = v16 - v18;
      v10 = *(long **)((long)a0 + 0x10);
      if (v11 == v10) {
label_cd6f:
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      }
    }
    v22 = (long)v11 + v20 & v22;
    v17 = v13[v12 + 1];
    if (v22 - v18 <= v21)
      v16 = v22;
    *(unsigned long *)((long)a0 + 0x18) = v16;
    v18 = v7[5];
    *(unsigned long *)((long)a0 + 0x10) = v16;
    v19[2] = v10;
    v10[5] = v18 + 1;
    v11 = v14[v12];
    *v10 = 0;
    v10[1] = 0;
    v10[2] = (long)v7;
    v10[3] = 0;
    v10[4] = 0;
    v10[6] = 0;
    *(unsigned char *)&v19[3] = v15;
    *(char *)((long)v19 + 0x19) = 0;
    if (v17) // branch-flip
      v11[1] = (long)v19;
    else {
      *v11 = (long)v19;
    }
    v7 = v10;
  } while (!v12);
  while( true ) {
    v10 = v14[v12];
    v8 = *(char *)((long)v10 + 0x19);
    if (v8) break;
    *(unsigned char *)((long)v10 + 0x19) = -(v17 == 0) | 1;
    if (!(v12 + -1)) goto label_ca25;
    v17 = v13[v12];
    v12 -= 1;
  }
  if (v17) { // branch-flip
    if (v17 != 1) goto label_ca25;
    v8 += '\x01';
    *(char *)((long)v10 + 0x19) = v8;
  }
  else {
    v8 += '\xff';
    *(char *)((long)v10 + 0x19) = v8;
  }
  if (v8) {
    if (v8 != '\xfe') { // branch-flip
      if (v8 != '\x02')
        abort(); // no-return, return-dupe
      if (v13[v12 + 2]) { // branch-flip
        if (v13[v12 + 2] != 1) {
          abort();
        }
        v11 = (long *)v10[1];
        v18 = *v11;
        *v11 = (long)v10;
        *(char *)((long)v10 + 0x19) = 0;
        v10[1] = v18;
        *(char *)((long)v11 + 0x19) = 0;
      }
      else {
        v4 = (long *)v10[1];
        v11 = (long *)*v4;
        v18 = v11[1];
        v5 = *v11;
        *v11 = (long)v10;
        v10[1] = v5;
        v11[1] = (long)v4;
        *v4 = v18;
        *(char *)((long)v10 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
        *(bool *)((long)v4 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
        *(char *)((long)v11 + 0x19) = 0;
      }
    }
    else if (v13[v12 + 2]) { // branch-flip
      if (v13[v12 + 2] != 1) {
        abort();
      }
      v18 = *v10;
      v11 = *(long **)(v18 + 8);
      v5 = v11[1];
      v6 = *v11;
      *v11 = v18;
      *(long *)(v18 + 8) = v6;
      v11[1] = (long)v10;
      *v10 = v5;
      *(char *)(v18 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
      *(bool *)((long)v10 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
      *(char *)((long)v11 + 0x19) = 0;
    }
    else {
      v11 = (long *)*v10;
      v18 = v11[1];
      v11[1] = (long)v10;
      *(char *)((long)v10 + 0x19) = 0;
      *v10 = v18;
      *(char *)((long)v11 + 0x19) = 0;
    }
    if (v13[v12]) // branch-flip
      v14[v12 + -1][1] = (long)v11;
    else {
      *v14[v12 + -1] = (long)v11;
    }
  }
  goto label_ca25;
}


// Function: sub_ceb0 @ 0xceb0
unsigned long sub_ceb0(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}


// Function: sub_cec0 @ 0xcec0
void sub_cec0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  long v10; // rax
  long v11;
  long v12;
  unsigned long *v13;
  unsigned long v14;
  long v15;
  long *v16;
  unsigned long v17;
  unsigned long v18; // rcx
  long *v19;
  unsigned long *v2;
  long *v20; // rdx
  long *v21; // rdx
  long *v22;
  long *v23; // rdx
  long *v24; // rdx
  unsigned long v25 [33];
  long *v26; // stack - 0x950
  unsigned long v27 [256];
  unsigned int v28;
  unsigned int v29;
  long v3;
  long v30;
  long *v31;
  unsigned long *v32;
  unsigned long *v33;
  long *v34;
  long *v35;
  struct_21 *v36;
  unsigned int v37; // r10d
  long **v38;
  unsigned long *v39;
  char v4; // al
  unsigned char v40;
  unsigned int v5;
  unsigned long v6;
  long *v7;
  long *v8;
  unsigned char *v9;
  
  v3 = a0[0x130];
  v39 = v25;
  if (!v3)
    v39 = &a0[0xe];
  v11 = a0[0xb];
  if (v11 == 1) {
    v38 = &v26;
    v19 = (long *)a0[0xc];
    v34 = v19;
    v26 = v19;
    for (; v19; v19 = (long *)v19[3]) {
      v35 = (long *)v19[1];
      if (v35) {
        v32 = (unsigned long *)*v35;
        if (v32) {
          if ((long *)*v32)
            sub_b420((long *)*v32,v38);
          if ((long *)v32[1])
            sub_b420((long *)v32[1],v38);
          v16 = (long *)v32[2];
          v26[3] = (long)v16;
          v26 = v16;
        }
        v32 = (unsigned long *)v35[1];
        v16 = v26;
        if (v32) {
          if ((long *)*v32)
            sub_b420((long *)*v32,v38);
          if ((long *)v32[1])
            sub_b420((long *)v32[1],v38);
          v16 = (long *)v32[2];
          v26[3] = (long)v16;
        }
        v26 = (long *)v35[2];
        v16[3] = (long)v26;
      }
    }
    v6 = a0[4];
    v17 = a0[0xd];
    if (v17 <= v6 - a0[3]) { // branch-flip
      v30 = a0[2];
      v12 = a0[3] + v17;
      if (v12 == v30) {
label_e3d2:
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      }
    }
    else {
      _obstack_newchunk((obstack *)a0,v17);
      v30 = a0[2];
      v6 = a0[4];
      v34 = (long *)a0[0xc];
      v12 = a0[3] + v17;
      v17 = a0[0xd];
      if (v12 == v30) goto label_e3d2;
    }
    a0[0x12e] = v30;
    v14 = v12 + a0[6] & ~a0[6];
    if (v14 - a0[1] <= v6 - a0[1])
      v6 = v14;
    a0[3] = v6;
    a0[2] = v6;
    if (1 <= (long)v17) {
      v12 = 0;
      while( true ) {
        *(char *)(v30 + v12) = *(char *)(v34[1] + 0x18);
        v12 += 1;
        v34 = (long *)v34[3];
        if ((long)a0[0xd] <= v12) break;
        v30 = a0[0x12e];
      }
    }
    v13 = (unsigned long *)sub_c880(a0[0x130]);
    v30 = a0[0xd];
    v9 = (unsigned char *)a0[0x12e];
    v13[0x133] = sub_b3f0;
    sub_c9d0(v13,v9,v30);
    _obstack_free((obstack *)a0,NULL);
    v30 = 0x134;
    v32 = v13;
    v33 = a0;
    while (v30) {
      v2 = &v33[1];
      v1 = &v32[1];
      *v33 = *v32;
      v30 -= 1;
      v32 = v1;
      v33 = v2;
    }
    free(v13);
    v6 = a0[0xd];
    if (0x100 <= (long)v6)
      v6 = 0xff;
    v6 &= 0xff;
    *v39 = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),0);
    v30 = v6 * 0x101010101010101 + SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),8);
    v39[1] = v30;
    v39[3] = v30;
    v39[2] = v6 * 0x101010101010101;
    v39[5] = v30;
    v39[7] = v30;
    v39[9] = v30;
    v39[4] = v6 * 0x101010101010101;
    v39[0xb] = v30;
    v39[0xd] = v30;
    v39[0xf] = v30;
    v39[6] = v6 * 0x101010101010101;
    v39[0x11] = v30;
    v39[0x13] = v30;
    v39[0x15] = v30;
    v39[8] = v6 * 0x101010101010101;
    v39[0x17] = v30;
    v39[0x19] = v30;
    v39[0x1b] = v30;
    v39[10] = v6 * 0x101010101010101;
    v39[0x1d] = v30;
    v39[0xc] = v6 * 0x101010101010101;
    v39[0xe] = v6 * 0x101010101010101;
    v39[0x10] = v6 * 0x101010101010101;
    v39[0x12] = v6 * 0x101010101010101;
    v39[0x14] = v6 * 0x101010101010101;
    v39[0x16] = v6 * 0x101010101010101;
    v39[0x18] = v6 * 0x101010101010101;
    v39[0x1a] = v6 * 0x101010101010101;
    v39[0x1c] = v6 * 0x101010101010101;
    v19 = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),8);
    v39[0x1e] = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),0);
    v39[0x1f] = v30;
    v34 = (long *)a0[0xc];
    if (v34) goto label_d084;
    v30 = 0;
    v26 = v34;
label_e1bb:
    for (v30 = *(long *)(v30 + 0x18); v30; v30 = *(long *)(v30 + 0x18)) {
      v12 = *(long *)(*(long *)(v30 + 0x10) + 0x38);
      v15 = *(long *)(v30 + 0x38);
      if (v12 < *(long *)(v30 + 0x38)) {
        *(long *)(v30 + 0x38) = v12;
        v15 = v12;
      }
      if (v15 < *(long *)(v30 + 0x30))
        *(long *)(v30 + 0x30) = v15;
    }
  }
  else {
    v6 = a0[0xd];
    if (0x100 <= (long)v6)
      v6 = 0xff;
    v6 &= 0xff;
    *v39 = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),0);
    v30 = v6 * 0x101010101010101 + SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),8);
    v39[1] = v30;
    v39[3] = v30;
    v39[2] = v6 * 0x101010101010101;
    v39[5] = v30;
    v39[7] = v30;
    v39[9] = v30;
    v39[4] = v6 * 0x101010101010101;
    v39[0xb] = v30;
    v39[0xd] = v30;
    v39[0xf] = v30;
    v39[6] = v6 * 0x101010101010101;
    v39[0x11] = v30;
    v39[0x13] = v30;
    v39[0x15] = v30;
    v39[8] = v6 * 0x101010101010101;
    v39[0x17] = v30;
    v39[0x19] = v30;
    v39[0x1b] = v30;
    v39[10] = v6 * 0x101010101010101;
    v39[0x1d] = v30;
    v39[0xc] = v6 * 0x101010101010101;
    v39[0xe] = v6 * 0x101010101010101;
    v39[0x10] = v6 * 0x101010101010101;
    v39[0x12] = v6 * 0x101010101010101;
    v39[0x14] = v6 * 0x101010101010101;
    v39[0x16] = v6 * 0x101010101010101;
    v39[0x18] = v6 * 0x101010101010101;
    v39[0x1a] = v6 * 0x101010101010101;
    v39[0x1c] = v6 * 0x101010101010101;
    v19 = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),8);
    v39[0x1e] = SUB168(ZEXT816(v6) * ZEXT816(0x101010101010101),0);
    v39[0x1f] = v30;
    v34 = (long *)a0[0xc];
    v26 = v34;
    if (v34) {
label_d084:
      v26 = v34;
label_dc63:
      do {
        v36 = (struct_21 *)v34[1];
        if (v36) {
          v35 = (long *)v36->field_0x0;
          if (v35) { // branch-flip
            v16 = (long *)*v35;
            if (v16) {
              v7 = (long *)*v16;
              if (v7) {
                v32 = (unsigned long *)*v7;
                if (v32) {
                  if ((long *)*v32) {
                    sub_b420((long *)*v32,&v26,v19);
                    v19 = v20;
                  }
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26,v19);
                  v19 = (long *)v32[2];
                  v26[3] = (long)v19;
                  v26 = v19;
                }
                v32 = (unsigned long *)v7[1];
                v19 = v26;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v19 = (long *)v32[2];
                  v26[3] = (long)v19;
                }
                v26 = (long *)v7[2];
                v19[3] = (long)v26;
              }
              v19 = (long *)v16[1];
              if (v19) {
                v32 = (unsigned long *)*v19;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                  v26 = v7;
                }
                v32 = (unsigned long *)v19[1];
                v7 = v26;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                }
                v26 = (long *)v19[2];
                v7[3] = (long)v26;
              }
              v19 = (long *)v16[2];
              v26[3] = (long)v19;
              v26 = v19;
            }
            v16 = (long *)v35[1];
            v19 = v26;
            if (v16) {
              v19 = (long *)*v16;
              if (v19) {
                v32 = (unsigned long *)*v19;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                  v26 = v7;
                }
                v32 = (unsigned long *)v19[1];
                v7 = v26;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                }
                v26 = (long *)v19[2];
                v7[3] = (long)v26;
              }
              v19 = (long *)v16[1];
              if (v19) {
                v32 = (unsigned long *)*v19;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                  v26 = v7;
                }
                v32 = (unsigned long *)v19[1];
                v7 = v26;
                if (v32) {
                  if ((long *)*v32)
                    sub_b420((long *)*v32,&v26);
                  if ((long *)v32[1])
                    sub_b420((long *)v32[1],&v26);
                  v7 = (long *)v32[2];
                  v26[3] = (long)v7;
                }
                v26 = (long *)v19[2];
                v7[3] = (long)v26;
              }
              v19 = (long *)v16[2];
              v26[3] = (long)v19;
            }
            v16 = (long *)v35[2];
            v7 = (long *)v36->field_0x8;
            v19[3] = (long)v16;
            v26 = v16;
            if (v7) { // branch-flip
label_d327:
              v16 = (long *)*v7;
              if (v16) {
                v8 = (long *)*v16;
                if (v8) {
                  v32 = (unsigned long *)*v8;
                  if (v32) {
                    if ((long *)*v32) {
                      sub_b420((long *)*v32,&v26,v19);
                      v19 = v21;
                    }
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26,v19);
                    v19 = (long *)v32[2];
                    v26[3] = (long)v19;
                    v26 = v19;
                  }
                  v32 = (unsigned long *)v8[1];
                  v19 = v26;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v19 = (long *)v32[2];
                    v26[3] = (long)v19;
                  }
                  v26 = (long *)v8[2];
                  v19[3] = (long)v26;
                }
                v19 = (long *)v16[1];
                if (v19) {
                  v32 = (unsigned long *)*v19;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v8 = (long *)v32[2];
                    v26[3] = (long)v8;
                    v26 = v8;
                  }
                  v32 = (unsigned long *)v19[1];
                  v8 = v26;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v8 = (long *)v32[2];
                    v26[3] = (long)v8;
                  }
                  v26 = (long *)v19[2];
                  v8[3] = (long)v26;
                }
                v19 = (long *)v16[2];
                v26[3] = (long)v19;
                v26 = v19;
              }
              v19 = (long *)v7[1];
              if (v19) {
                v8 = (long *)*v19;
                if (v8) {
                  v32 = (unsigned long *)*v8;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v22 = (long *)v32[2];
                    v26[3] = (long)v22;
                    v26 = v22;
                  }
                  v32 = (unsigned long *)v8[1];
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v22 = (long *)v32[2];
                    v26[3] = (long)v22;
                    v26 = v22;
                  }
                  v8 = (long *)v8[2];
                  v26[3] = (long)v8;
                  v26 = v8;
                }
                v8 = (long *)v19[1];
                v22 = v26;
                if (v8) {
                  v32 = (unsigned long *)*v8;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v22 = (long *)v32[2];
                    v26[3] = (long)v22;
                    v26 = v22;
                  }
                  v32 = (unsigned long *)v8[1];
                  v31 = v26;
                  if (v32) {
                    if ((long *)*v32)
                      sub_b420((long *)*v32,&v26);
                    if ((long *)v32[1])
                      sub_b420((long *)v32[1],&v26);
                    v31 = (long *)v32[2];
                    v26[3] = (long)v31;
                  }
                  v22 = (long *)v8[2];
                  v31[3] = (long)v22;
                }
                v26 = (long *)v19[2];
                v22[3] = (long)v26;
              }
              v12 = v7[2];
              v30 = v34[5];
              v26[3] = v12;
              v26 = v36->field_0x10;
              *(long **)(v12 + 0x18) = v26;
              if (v35) goto label_d613;
            }
            else {
              v26 = v36->field_0x10;
              v30 = v34[5];
              v16[3] = (long)v26;
label_d613:
              v19 = (long *)*v35;
              v40 = (unsigned char)v30;
              if (v19) {
                v16 = (long *)*v19;
                if (v16) {
                  v32 = (unsigned long *)*v16;
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  v32 = (unsigned long *)v16[1];
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39) = v40;
                }
                v16 = (long *)v19[1];
                if (v16) {
                  v32 = (unsigned long *)*v16;
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  v32 = (unsigned long *)v16[1];
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39) = v40;
              }
              v19 = (long *)v35[1];
              if (v19) {
                v16 = (long *)*v19;
                if (v16) {
                  v32 = (unsigned long *)*v16;
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  v32 = (unsigned long *)v16[1];
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39) = v40;
                }
                v16 = (long *)v19[1];
                if (v16) {
                  v32 = (unsigned long *)*v16;
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  v32 = (unsigned long *)v16[1];
                  if (v32) {
                    sub_c100((struct_22 *)*v32,v30,v39);
                    sub_c100((struct_22 *)v32[1],v30,v39);
                    if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                  }
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39) = v40;
              }
              if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39) = v40;
              v7 = (long *)v36->field_0x8;
              if (!v7) goto label_dc21;
              v16 = (long *)*v7;
            }
            v40 = (unsigned char)v30;
            if (v16) {
              v19 = (long *)*v16;
              if (v19) {
                v32 = (unsigned long *)*v19;
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                v32 = (unsigned long *)v19[1];
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39) = v40;
              }
              v19 = (long *)v16[1];
              if (v19) {
                v32 = (unsigned long *)*v19;
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                v32 = (unsigned long *)v19[1];
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39) = v40;
              }
              if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v16[3] + (long)v39) = v40;
            }
            v19 = (long *)v7[1];
            if (v19) {
              v35 = (long *)*v19;
              if (v35) {
                v32 = (unsigned long *)*v35;
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                v32 = (unsigned long *)v35[1];
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39) = v40;
              }
              v35 = (long *)v19[1];
              if (v35) {
                v32 = (unsigned long *)*v35;
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                v32 = (unsigned long *)v35[1];
                if (v32) {
                  sub_c100((struct_22 *)*v32,v30,v39);
                  sub_c100((struct_22 *)v32[1],v30,v39);
                  if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v32[3] + (long)v39) = v40;
                }
                if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v35[3] + (long)v39) = v40;
              }
              if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v19[3] + (long)v39) = v40;
            }
            if (v30 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v7[3] + (long)v39))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v7[3] + (long)v39) = v40;
          }
          else {
            v7 = (long *)v36->field_0x8;
            if (v7) goto label_d327;
            v19 = v36->field_0x10;
            v30 = v34[5];
            v26[3] = (long)v19;
            v26 = v19;
          }
label_dc21:
          v9 = (unsigned char *)((unsigned long)v36->field_0x18 + (long)v39);
          if (v30 < (long)(unsigned long)*v9)
            *v9 = (unsigned char)v30;
          v36 = (struct_21 *)v34[1];
        }
        v30 = a0[0xc];
        sub_ab10(v36,v34[4],v30,v11 == 1);
        if (v11 == 1) {
          v12 = a0[0xd];
          v15 = v34[4];
          v34[6] = v12;
          v34[7] = v12;
          if (v15) {
            v12 = *v34;
            do {
              v4 = sub_abd0(*(unsigned long **)(v15 + 8),v36);
              if ((!v4) && (v10 = v34[5] - *(long *)(v15 + 0x28), v10 < *(long *)(v15 + 0x30)))
                *(long *)(v15 + 0x30) = v10;
              if ((v12) && (v10 = v34[5] - *(long *)(v15 + 0x28), v10 < *(long *)(v15 + 0x38)))
                *(long *)(v15 + 0x38) = v10;
              v15 = *(long *)(v15 + 0x20);
            } while (v15);
            v34 = (long *)v34[3];
            v19 = v24;
            if (!v34) break;
            goto label_dc63;
          }
        }
        v34 = (long *)v34[3];
        v19 = v23;
      } while (v34);
      if (v11 == 1) goto label_e1bb;
    }
  }
  v6 = 0xffffffff;
  v32 = v27;
  if (!v3)
    v32 = &a0[0x2e];
  v37 = 0xffffffff;
  *v32 = 0;
  v32[0xff] = 0;
  v17 = (unsigned long)(((int)v32 - (int)(unsigned long *)((unsigned long)&v32[1] & 0xfffffffffffffff8)) + 0x800U >> 3);
  v33 = (unsigned long *)((unsigned long)&v32[1] & 0xfffffffffffffff8);
  while (v17) {
    v13 = &v33[1];
    *v33 = 0;
    v17 -= 1;
    v33 = v13;
  }
  sub_ba10(*(struct_22 **)(a0[0xc] + 8),v32);
  v17 = 0;
  v28 = 0xfffffffe;
  do {
    while( true ) {
      v5 = (unsigned int)v17;
      v14 = v17 & 0xffffffff;
      if (v3) { // branch-flip
        v18 = (unsigned long)*(unsigned char *)(v3 + v17);
        v30 = v32[v18];
        a0[v17 + 0x2e] = v30;
      }
      else {
        v30 = a0[v17 + 0x2e];
        v18 = v17 & 0xffffffff;
      }
      v29 = v28;
      if (v30) break;
label_ddd0:
      v5 = (unsigned int)v6;
      v17 += 1;
      v28 = v29;
      if (v17 == 0x100) goto label_de21;
    }
    v29 = (unsigned int)v18;
    if (v28 != 0xfffffffe) {
      if (v29 != v28) { // branch-flip
        if ((v29 != v5) || (v28 != (unsigned int)v6))
          v29 = 0xffffffff;
        else {
          v6 = v14;
          v29 = v28;
        }
      }
      else {
        if (v28 != (unsigned int)v6)
          v5 = v37;
        v6 = (unsigned long)v5;
        v29 = v28;
      }
      goto label_ddd0;
    }
    v17 += 1;
    v6 = v14;
    v28 = v29;
  } while (v17 != 0x100);
label_de21:
  *(unsigned int *)&a0[0x131] = v29;
  *(unsigned int *)((long)a0 + 0x98c) = v5;
  if (v11 != 1) goto label_de3a;
  v6 = a0[4];
  v17 = a0[0xd];
  if (v17 <= v6 - a0[3]) { // branch-flip
    v11 = a0[2];
    v30 = a0[3] + v17;
    v14 = v17;
    if (v30 == v11) {
label_e39e:
      *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
    }
  }
  else {
    _obstack_newchunk((obstack *)a0,v17);
    v11 = a0[2];
    v6 = a0[4];
    v14 = a0[0xd];
    v30 = a0[3] + v17;
    if (v30 == v11) goto label_e39e;
  }
  a0[0x12e] = v11;
  v17 = v30 + a0[6] & ~a0[6];
  if (v17 - a0[1] <= v6 - a0[1])
    v6 = v17;
  a0[3] = v6;
  a0[2] = v6;
  v30 = a0[0xc];
  if (1 <= (long)v14) {
    while( true ) {
      v14 -= 1;
      *(char *)(v11 + v14) = *(char *)(*(long *)(v30 + 8) + 0x18);
      v30 = *(long *)(v30 + 0x18);
      if (!v14) break;
      v11 = a0[0x12e];
    }
    v11 = a0[0xd];
    if (2 <= v11) {
      v17 = a0[4];
      v30 = a0[3];
      v6 = v11 * 8 - 8;
      if (v17 - v30 < v6) {
        _obstack_newchunk((obstack *)a0,v6);
        v30 = a0[3];
        v17 = a0[4];
        v11 = a0[0xd];
      }
      v12 = a0[2];
      if (v30 + v6 == v12)
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      a0[0x12f] = v12;
      v6 = v30 + v6 + a0[6] & ~a0[6];
      if (v6 - a0[1] <= v17 - a0[1])
        v17 = v6;
      a0[3] = v17;
      a0[2] = v17;
      v30 = *(long *)(a0[0xc] + 0x18);
      if (2 <= v11) {
        v15 = 0;
        do {
          v32 = (unsigned long *)(v30 + 0x30);
          v30 = *(long *)(v30 + 0x18);
          *(unsigned long *)(v12 + v15 * 8) = *v32;
          v11 = a0[0xd];
          v15 += 1;
        } while (v15 < v11 + -1);
      }
      v40 = *(unsigned char *)(a0[0x12e] + -2 + v11);
      if (v3)
        v40 = *(unsigned char *)(v3 + (unsigned long)v40);
      *(unsigned char *)&a0[0x132] = v40;
    }
  }
label_de3a:
  v11 = 0;
  if (v3) {
    do {
      *(char *)((long)a0 + v11 + 0x70) = *(char *)((long)v39 + (unsigned long)*(unsigned char *)(v3 + v11));
      v11 += 1;
    } while (v11 != 0x100);
    return;
  }
}


// Function: sub_e3e0 @ 0xe3e0
void sub_e3e0(long a0)
{
  (**(void **)(a0 + 0x998))(); // jump-as-call
}


// Function: sub_e420 @ 0xe420
long sub_e420(long a0,long a1,bool a2)
{
  unsigned long v1; // rax
  long v2; // rax
  unsigned int v3; // stack - 0x4c
  unsigned long v4; // stack - 0x48
  long v5; // r14
  
  v5 = 0;
  v4 = 0;
  do {
    if (a1 - a0 <= v5)
      return v5;
    v1 = (unsigned long)(unsigned char)*(char *)(a0 + v5);
    if (*(char *)(v1 + 0x2b9c0)) // branch-flip
      v5 += 1;
    else {
      if (*(char *)(v1 + 0x2b183) != '\xfe')
        return v5;
      v3 = 0;
      v2 = sub_1acc0(&v3,(char *)(a0 + v5),(a1 - a0) - v5,&v4);
      if ((v3 != 0x5f) && (!iswalnum(v3)))
        return v5;
      v5 = v5 + v2 + (unsigned long)(v2 == 0);
    }
    if (!a2)
      return v5;
  } while( true );
}


// Function: sub_e570 @ 0xe570
void sub_e570(void)
{
  unsigned int v1;
  unsigned int *v2; // rbx
  char *v3; // rbp
  bool v4; // al
  
  v3 = (char *)0x2b9c0;
  v2 = (unsigned int *)0x2b284;
  do {
    v1 = *v2;
    v4 = 1;
    if (v1 != 0x5f)
      v4 = iswalnum(v1) != 0;
    v2 = &v2[1];
    *v3 = v4;
    v3 = &v3[1];
  } while (v2 != (unsigned int *)0x2b684);
}


// Function: sub_e5c0 @ 0xe5c0
void sub_e5c0(bool a0)
{
  long v1;
  long v2; // rax
  long v3; // rax
  
  if (!dat_2b687) {
    sub_c880(0); // tail-call
    return;
  }
  if ((__ctype_get_mb_cur_max() != 1) && (!a0)) {
    sub_c880(0); // tail-call
    return;
  }
  v2 = sub_1ef30(0x100);
  v1 = *(long *)__ctype_toupper_loc();
  v3 = 0;
  do {
    *(char *)(v2 + v3) = (char)*(unsigned int *)(v1 + v3 * 4);
    v3 += 1;
  } while (v3 != 0x100);
  sub_c880(v2); // tail-call
}


// Function: sub_e640 @ 0xe640
long sub_e640(unsigned long *a0,unsigned long *a1,unsigned char *a2,long a3) // early-return, return-dupe
{
  int v1; // eax
  unsigned char *v2;
  unsigned long v3; // rax
  unsigned long v4;
  long v5; // rdx
  unsigned char *v6;
  unsigned long v7; // stack - 0x48
  unsigned char *v8; // r14
  
  v6 = (unsigned char *)*a0;
  if (v6 < a2) {
    if (dat_2b182) { // branch-flip
      if (((int)(char)*a2 & 0xc0U) != 0x80) {
label_e6ea:
        *a0 = a2;
        return 0;
      }
      v1 = (unsigned int)(char)a2[-1];
      if ((v1 & 0xc0) != 0x80) // branch-flip
        v5 = 1;
      else {
        v1 = (unsigned int)(char)a2[-2];
        if ((v1 & 0xc0) != 0x80) // branch-flip
          v5 = 2;
        else {
          v1 = (unsigned int)(char)a2[-3];
          if ((v1 & 0xc0) == 0x80) goto label_e6ea;
          v5 = 3;
        }
      }
      if ((int)(~v1 & 0xff) >> (7U - (char)v5 & 0x1fU)) goto label_e6ea;
      v7 = 0;
      v8 = &a2[-v5];
      v3 = sub_1ac90(v8,a3 - (long)v8,&v7);
      if ((v3 <= 0x10) || (v2 = a2, 0 <= (long)v3)) {
        v2 = &v8[v3];
        v6 = v8;
      }
    }
    else {
      v7 = 0;
      while( true ) {
        v3 = (unsigned long)*(char *)((unsigned long)*v6 + 0x2b183);
        if (((*(char *)((unsigned long)*v6 + 0x2b183) != '\xfe') || (v3 = sub_1ac90(v6,a3 - (long)v6,&v7), v4 = v3, 0x11 <= v3)) && (v4 = v3, (long)v3 <= -1)) {
          v7 = 0;
          v4 = 1;
          v2 = &v6[1];
        }
        else {
          v2 = &v6[v3];
        }
        if (a2 <= v2) break;
        v6 = v2;
      }
      if (a1)
        *a1 = v4;
    }
    *a0 = v2;
    if (v2 == a2)
      return 0;
  }
  return (long)a2 - (long)v6;
}


// Function: sub_e810 @ 0xe810
long sub_e810(long a0,long a1)
{
  return sub_e420(a0,a1,1); // tail-call
}


// Function: sub_e830 @ 0xe830
unsigned long sub_e830(long a0,long a1,long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x28
  
  if (a0 == a1)
    return 0;
  if ((dat_2b180) && (!(~(*(unsigned char *)(a1 + -1) >> 7) & dat_2b182))) {
    v2 = a0;
    v1 = sub_e420((a1 + -1) - sub_e640(&v2,NULL,(unsigned char *)(a1 + -1),a2),a2,0); // tail-call
    return v1;
  }
  return (unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)(a1 + -1) + 0x2b9c0);
}


// Function: sub_e900 @ 0xe900
void sub_e900(long a0)
{
  if (0 <= a0) {
    sub_1ef30(); // tail-call
    return;
  }
  sub_1f410(); // no-return
}


// Function: sub_e920 @ 0xe920
int sub_e920(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
{
  int v1;
  long v2;
  long v3;
  int v4; // stack - 0x44
  bool v5; // sf
  unsigned long v6; // stack - 0x60
  
  while( true ) {
    while (v1 = pcre2_match_8(a0[1],a1,a2,a3,a4,a0[3],a0[2]), v1 == -0x2e) {
      v2 = a0[5];
      if (0x4000000000000000 <= v2)
        return -0x2e;
      a0[5] = v2 * 2;
      pcre2_jit_stack_free_8(a0[4]);
      v2 = pcre2_jit_stack_create_8(v2,v2 * 2,*a0);
      a0[4] = v2;
      if (!v2)
        sub_1f410(); // no-return
      v3 = a0[2];
      if (!v3) {
        v3 = pcre2_match_context_create_8(*a0);
        v2 = a0[4];
        a0[2] = v3;
      }
      v6 = 0xea52;
      pcre2_jit_stack_assign_8(v3,0,v2);
    }
    if (v1 != -0x35)
      return v1;
    pcre2_config_8(7,&v4,v6);
    v5 = v4 <= -1;
    v4 *= 2;
    if (v5) break;
    v2 = a0[2];
    v1 = v4;
    if (!v2) {
      v2 = pcre2_match_context_create_8(*a0);
      a0[2] = v2;
      v1 = v4;
    }
    v6 = 0xe988;
    pcre2_set_depth_limit_8(v2,v1);
  }
  return 0xffffffcb;
}


// Function: sub_ea80 @ 0xea80
unsigned long * sub_ea80(char *a0,unsigned long a1)
{
  unsigned int v1; // eax
  unsigned long *v10; // rax
  int v11; // stack - 0x154
  char v12 [8];
  char v13 [264];
  unsigned int v14; // r13d
  bool v15; // cf
  unsigned long v16; // stack - 0x168
  char *v17; // stack - 0x160
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  char *v6; // rax
  unsigned long *v7; // rax
  unsigned long *v8; // rax
  unsigned long *v9; // rax
  
  v15 = dat_2b687 == '\0';
  v2 = (unsigned long *)sub_1ef30(0x38);
  v14 = (-(unsigned int)v15 & 0xfffffff8) + 0x18;
  v3 = pcre2_general_context_create_8(sub_e900,sub_e8f0,0);
  *v2 = v3;
  v4 = pcre2_compile_context_create_8(v3);
  if (dat_2b180) {
    if (!dat_2b182) {
      error(2,0,dcgettext(NULL,"-P supports only unibyte and UTF-8 locales",5));
      return v8;
    }
    v14 |= 0x4080000;
  }
  if (&a0[a1] != rawmemchr(a0,10)) {
    error(2,0,dcgettext(NULL,"the -P option only supports a single pattern",5));
    return v10;
  }
  v16 = a1;
  if (dat_2b685) { // branch-flip
    pcre2_set_compile_extra_options_8(v4,8);
    v17 = NULL;
  }
  else {
    v17 = NULL;
    if (dat_2b686) {
      v16 = a1 + 0x11;
      v17 = (char *)sub_1ef30(v16);
      *(unsigned long *)v17 = s_22990._0_8_;
      *(unsigned short *)&v17[8] = s_22990._8_2_;
      v6 = mempcpy(&v17[10],a0,a1);
      *(unsigned int *)v6 = s_22988._0_4_;
      *(unsigned short *)&v6[4] = s_22988._4_2_;
      v6[6] = s_22988[6];
      a0 = v17;
    }
  }
  pcre2_set_character_tables_8(v4,pcre2_maketables_8(v3));
  v5 = pcre2_compile_8(a0,v16,v14,&v11,v12,v4);
  v2[1] = v5;
  if (!v5) {
    pcre2_get_error_message_8(v11,v13,0x100);
    error(2,0,"%s",v13);
    return v9;
  }
  free(v17);
  pcre2_compile_context_free_8(v4);
  v2[2] = 0;
  v3 = pcre2_match_data_create_from_pattern_8(v2[1],v3);
  v2[3] = v3;
  v1 = pcre2_jit_compile_8(v2[1],1);
  v11 = v1;
  if ((v1 + 0x30U <= 0x30) && (!(0xfffefffffffffff6U >> ((unsigned long)(v1 + 0x30U) & 0x3f) & 1))) {
    v2[4] = 0;
    v2[5] = 0x8000;
    *(unsigned int *)&v2[6] = sub_e920(v2,"",0,0,1);
    *(unsigned int *)((long)v2 + 0x34) = sub_e920(v2,"",0,0,0);
    return v2;
  }
  error(2,0,dcgettext(NULL,"JIT internal error: %d",5),v1);
  return v7;
}


// Function: sub_eda0 @ 0xeda0
unsigned char * sub_eda0(unsigned long *a0,unsigned char *a1,long a2,long *a3,unsigned char *a4)
{
  int v1;
  unsigned char *v10;
  unsigned char *v11; // stack - 0x68
  long *v2;
  char *v3;
  unsigned long v4;
  unsigned char *v5; // rax
  long v6; // rcx
  unsigned char *v7;
  unsigned int v8; // ebp
  unsigned char *v9;
  
  v7 = a1;
  if (a4)
    v7 = a4;
  v8 = (unsigned int)(v7[-1] == dat_2b684);
  v5 = (long *)pcre2_get_ovector_pointer_8(a0[3]);
  v2 = (long *)v5;
  v11 = a1;
  while( true ) {
    v5 = rawmemchr(v7,(int)(char)dat_2b684);
    v9 = v5;
    v5 = (unsigned long)(unsigned long)*v7;
    if (*(char *)((long)v5 + 0x2b183) != '\xff') { // branch-flip
      v6 = (long)v7 - (long)v11;
      v10 = v11;
    }
    else {
      do {
        v5 = (unsigned long)(unsigned long)v7[1];
        v7 = &v7[1];
      } while (*(char *)((long)v5 + 0x2b183) == '\xff');
      v6 = 0;
      v8 = 0;
      v10 = v7;
    }
    if (v9 != v7) // branch-flip
      v1 = sub_e920(a0,v10,(long)v9 - (long)v10,v6,v8 ^ 1);
    else {
      v1 = *(int *)((long)a0 + (unsigned long)v8 * 4 + 0x30);
      v2[1] = v6;
      *v2 = v6;
    }
    if (v1 != -1) break;
    v7 = &v9[1];
    v5 = &a1[a2];
    v8 = 1;
    if (v5 <= v7) {
      v5 = (unsigned char *)0xffffffffffffffff;
      return v5;
    }
    v11 = v7;
  }
  if (1 <= v1) {
    v9 = &v9[1];
    if (a4) {
      v11 = &v10[*v2];
      v9 = &v10[v2[1]];
    }
    v5 = (long)((long)v11 - (long)a1);
    *a3 = (long)v9 - (long)v11;
    return v5;
  }
  if ((v1 < -0x2d) && (-0x3f <= v1)) {
    switch(v1) {
      case 0xffffffcb:
label_f044:
        v5 = (unsigned long)sub_90f0();
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s nested backtracking limit",5);
        error(2,0,(char *)v5,v4);
        goto label_f073;
      case 0xffffffc1:
label_f073:
        v5 = (unsigned long)sub_90f0();
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s heap limit",5);
        error(2,0,(char *)v5,v4);
      case 0xffffffcc:
label_f015:
        v5 = (unsigned long)sub_90f0(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: PCRE detected recurse loop",5);
        error(2,0,(char *)v5,v4);
        goto label_f044;
      case 0xffffffd0:
label_efe6:
        v5 = (unsigned long)sub_90f0();
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: memory exhausted",5);
        error(2,0,(char *)v5,v4);
        goto label_f015;
      case 0xffffffd1:
label_efb7:
        v5 = (unsigned long)sub_90f0();
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s backtracking limit",5);
        error(2,0,(char *)v5,v4);
        goto label_efe6;
      case 0xffffffd2:
        v5 = (unsigned long)sub_90f0();
        v4 = v5;
        v5 = (char *)dcgettext(NULL,"%s: exhausted PCRE JIT stack",5);
        error(2,0,(char *)v5,v4);
        goto label_efb7;
      
    }
  }
  v5 = (char *)sub_90f0();
  v3 = (char *)v5;
  v5 = (char *)dcgettext(NULL,"%s: internal PCRE error: %d",5);
  error(2,0,(char *)v5,v3,v1);
  return v5;
}

