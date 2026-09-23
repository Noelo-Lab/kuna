// Function: sub_4ff0 @ 0x4ff0
unsigned long sub_4ff0(int a0,char **a1) // early-return
{
  long *v1;
  int v10;
  int v11;
  char *v12;
  void *v13; // rax
  unsigned long v14;
  void *v15;
  char *v16;
  char *v17;
  long v18; // rax
  unsigned long v19; // rax
  unsigned int *v2;
  unsigned long v20; // rax
  long v21;
  long *v22;
  char *v23;
  long v24; // stack - 0x1a8
  char v25; // stack - 0x68
  long v26; // stack - 0x1a0
  char v27 [3];
  char v28 [8];
  char v29; // stack - 0x67
  char v3;
  long v30; // stack - 0x188
  long *v31; // stack - 0x1b0
  long v32 [18]; // stack - 0xf8
  unsigned long v33; // stack - 0x190
  char v34 [4];
  unsigned int v35; // eax
  unsigned int *v36;
  unsigned int v37; // eax
  long *v38;
  char *v39; // r12
  unsigned int v4;
  int *v40; // rax
  char *v41; // r14
  unsigned char *v42; // r15
  char *v43;
  unsigned char *v44;
  int v45; // stack - 0x1f8
  int v46; // stack - 0x1e4
  unsigned char *v47; // stack - 0x1e0
  long *v48; // stack - 0x1d8
  int v49; // stack - 0x1d0
  char v5 [16];
  unsigned int v50; // stack - 0x1cc
  long v51; // stack - 0x180
  unsigned int v52; // stack - 0x170
  char v53; // stack - 0x66
  bool v6;
  unsigned int v7;
  unsigned char v8;
  char v9; // al
  
  dat_2b684 = 10;
  v31 = NULL;
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
  sub_f6a0(0);
  dat_2b8c8 = sub_1a2d0(0,0,sub_7850,sub_78a0,0);
  if (!dat_2b8c8)
    sub_1f410(); // no-return
  v48 = &v24;
  v50 = 0;
  v49 = 0;
  v46 = -1;
  v47 = NULL;
label_5148:
  v10 = optind;
  v6 = 0;
  v16 = &v25;
  while( true ) {
    v11 = optind;
    v45 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(void *)0x29fc0,NULL);
    if ((unsigned int)(v45 - 0x30U) > 9) break;
    if ((dat_2b010 != v11) || (!v6))
      v12 = &v25;
    else {
      v12 = &v16[-(unsigned long)(v25 == '0')];
      if (v12 == v27) {
        builtin_strncpy(&v16[-(unsigned long)(v25 == '0')],"...",4);
        v16 = &v27[3];
        goto label_51b5;
      }
    }
    dat_2b010 = v11;
    v16 = &v12[1];
    v6 = 1;
    *v12 = (char)v45;
  }
  if (v16 != &v25) {
label_51b5:
    *v16 = '\0';
    sub_7d10(&v25,&v26);
  }
  v16 = optarg;
  if (v45 != -1) {
    switch(v45) {
      case 0:
      case 0x55:
        break;
      default:
        sub_a170(2); // no-return, return-dupe
      case 0x41:
        sub_7d10(optarg,0x2b7f0);
        break;
      case 0x42:
        sub_7d10(optarg,0x2b7f8);
        break;
      case 0x43:
        sub_7d10(optarg,&v26);
        break;
      case 0x44:
        v10 = strcmp(optarg,"read");
        if (v10) { // branch-flip
label_5a31:
          if (strcmp(v16,"skip")) {
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
        v46 = sub_7950("egrep",v46);
        break;
      case 0x46:
        v46 = sub_7950("fgrep",v46);
        break;
      case 0x47:
        v46 = sub_7950("grep",v46);
        break;
      case 0x48:
        v50 = 1;
        break;
      case 0x49:
label_52b7:
        dat_2b810 = 2;
        break;
      case 0x4c:
        dat_2b80c = 2;
        break;
      case 0x50:
        v46 = sub_7950("perl",v46);
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
        v46 = sub_7950(optarg,v46);
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
        v21 = sub_f3f0("--directories",optarg,0x29fa0,0x22788,4,dat_2b078,1);
        dat_2b018 = *(int *)(v21 * 4 + 0x22788);
        if (dat_2b018 != 3)
          v10 = v49;
        v49 = v10;
        break;
      case 0x65:
        v14 = strlen(optarg);
        v22 = v31;
        v21 = (long)v31 + ((v14 + 1) - v24);
        if (1 <= v21) {
          dat_2b8d8 = (unsigned char *)sub_1f1f0(v47,v48,v21,0xffffffffffffffff,1);
          v47 = dat_2b8d8;
          v16 = optarg;
        }
        *(char *)mempcpy(&v47[(long)v22],v16,v14) = 10;
        v31 = (long *)sub_7a20(v47,v22,v14 + 1 + (long)v22,"");
        break;
      case 0x66:
        v10 = strcmp(optarg,"-");
        if (v10) { // branch-flip
          v44 = fopen(v16,"r");
          if (!v44) {
            error(2,*__errno_location(),"%s",optarg);
            v16 = NULL;
            goto label_5a31;
          }
        }
        else {
          v44 = stdin;
        }
        v22 = v31;
        v38 = v31;
        while( true ) {
          v21 = (long)v38 + (2U - v24);
          if (1 <= v21) {
            v47 = (unsigned char *)sub_1f1f0(v47,v48,v21,0xffffffffffffffff,1);
            dat_2b8d8 = v47;
          }
          v21 = fread_unlocked(&v47[(long)v38],1,v24 - (long)(long *)((long)v38 + 1U),v44);
          if (!v21) break;
          v38 = (long *)((long)v38 + v21);
        }
        v40 = __errno_location();
        v10 = *v40;
        if (*v44 & 0x20) { // branch-flip
label_5749:
          if (v10) {
            error(2,v10,"%s",optarg);
            return v19;
          }
        }
        else if (stdin != v44) { // branch-flip
          if (fclose(v44)) {
            v10 = *v40;
            goto label_5749;
          }
        }
        else {
          clearerr_unlocked(v44);
        }
        if ((v38 != v22) && ((&v47[-1])[(long)v38] != 10)) {
          v47[(long)v38] = 10;
          v38 = (long *)((long)v38 + 1U);
        }
        v31 = (long *)sub_7a20(v47,v22,v38,optarg);
        break;
      case 0x68:
        v50 = 0xffffffff;
        break;
      case 0x69:
      case 0x79:
        dat_2b687 = '\x01';
        break;
      case 0x6c:
        goto label_5660;
      case 0x6d:
        v37 = sub_1f450(optarg,0,10,0x2b7e0,"");
        if (2 <= v37) {
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
        v49 = v10;
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
        v10 = strcmp(optarg,"binary");
        if (!v10) {
          dat_2b810 = 0;
          break;
        }
        if (!strcmp(v16,"text")) goto label_5291;
        if (strcmp(v16,"without-match")) {
          error(2,0,dcgettext(NULL,"unknown binary-files type",5));
          return v20;
        }
        goto label_52b7;
      case 0x81:
        if (optarg) {
          v10 = sub_f750(optarg,"always");
          if (((!v10) || (!sub_f750(v16,"yes"))) || (!sub_f750(v16,"force"))) {
            dat_2b900 = 1;
            break;
          }
          if (((!sub_f750(v16,"never")) || (!sub_f750(v16,"no"))) || (!sub_f750(v16,"none"))) {
            dat_2b900 = 0;
            break;
          }
          if (((sub_f750(v16,"auto")) && (sub_f750(v16,"tty"))) && (sub_f750(v16,"if-tty"))) {
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
        v37 = (unsigned int)(v45 == 0x86) << 0x1d;
        sub_16d40(v21,optarg,v37 | 0x50000000);
        v21 = dat_2b8b8;
        if (!dat_2b8b8) {
          v21 = sub_169d0();
          dat_2b8b8 = v21;
        }
        sub_16d40(v21,optarg,v37 | 0x10000000);
        break;
      case 0x84:
        v21 = dat_2b8b0;
        if (!dat_2b8b0) {
          v21 = sub_169d0();
          dat_2b8b0 = v21;
        }
        v10 = sub_17290(sub_16d40,v21,optarg,0x50000000,10);
        if (!v10) {
          v21 = dat_2b8b8;
          if (!dat_2b8b8) {
            v21 = sub_169d0();
            dat_2b8b8 = v21;
          }
          v10 = sub_17290(sub_16d40,v21,optarg,0x10000000,10);
          if (!v10) break;
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
    sub_1ed40(stdout,sub_19950(),"GNU grep","3.8",0);
    puts(dcgettext(NULL,"Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",5));
    return 0;
  }
  if (dat_2b908)
    sub_a170(0); // no-return
  if (v47) { // branch-flip
    if (!v31) {
      dat_2b686 = 0;
      dat_2b685 = '\0';
      dat_2b802 ^= 1;
      v31 = (long *)0x1;
      *v47 = 10;
    }
  }
  else {
    v21 = (long)optind;
    if (a0 <= optind) {
      sub_a170(2);
    }
    optind += 1;
    v14 = 0;
    v16 = a1[v21];
    if ((v46 != 2) && (*v16 == '\\'))
      v14 = (unsigned long)(v16[1] == '-');
    v47 = (unsigned char *)sub_1f3f0(&v16[v14]);
    dat_2b8d8 = v47;
    v14 = strlen((char *)v47);
    v47[v14] = 10;
    v31 = (long *)sub_7a20(v47,0,v14 + 1,"");
  }
  v22 = v31;
  v48 = (long *)((long)v31 - 1);
  v31 = v48;
  sub_1a490(dat_2b8c8);
  if (dat_2b7a1) {
    v6 = 0;
    goto label_5965;
  }
  v6 = 0;
  if (!fstat(1,&v30)) {
    if ((v52 & 0xf000) != 0x8000) { // branch-flip
      if ((v52 & 0xf000) == 0x2000) {
        if (((stat("/dev/null",v32)) || (v51 != v32[1])) || (v30 != v32[0]))
          v6 = 1;
        else {
          dat_2b7a0 = '\x01';
        }
      }
    }
    else {
      v21 = 0x24;
      v38 = &v30;
      v36 = (unsigned int *)0x2b920;
      while (v21) {
        v2 = &v36[1];
        v1 = (long *)((long)v38 + 4);
        *v36 = (int)*v38;
        v21 -= 1;
        v38 = v1;
        v36 = v2;
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
  if (((!dat_2b7e0) || (((!v48 && (dat_2b802)) && ((!dat_2b685 && (!dat_2b686)))))) && (dat_2b80c != 2))
    return 1;
  if (dat_2b900 == 2) {
    if ((!v6) || (!sub_1fa50()))
      dat_2b900 = 0;
    else {
      dat_2b900 = (unsigned int)(isatty(1) != 0);
    }
  }
  sub_1fa90();
  if (dat_2b900) {
    v16 = getenv("GREP_COLOR");
    if (v16) {
      if (*v16) {
        v12 = v16;
        while( true ) {
          if (*v12 != ';') {
            if (9 < (unsigned int)((int)*v12 - 0x30U)) goto label_62a1;
          }
          if (!v12[1]) break;
          v12 = &v12[1];
        }
        dat_2b060 = v16;
        dat_2b068 = v16;
      }
    }
label_62a1:
    v12 = getenv("GREP_COLORS");
    if ((v12) && (*v12)) {
      v17 = (char *)sub_1f3f0(v12);
      v23 = NULL;
      v12 = v17;
label_62d6:
      do {
        do {
          do {
            while( true ) {
              v39 = &v12[1];
              v3 = *v12;
              if ((v3 != ':') && (v3)) break;
              *v12 = '\0';
              v21 = 0x2a620;
              v12 = "mt";
              do {
                if (!strcmp(v12,v17)) break;
                v12 = *(char **)(v21 + 0x18);
                v21 += 0x18;
              } while (v12);
              if ((*(unsigned long **)(v21 + 8)) && (v23))
                **(unsigned long **)(v21 + 8) = v23;
              if (*(void **)(v21 + 0x10))
                (**(void **)(v21 + 0x10))();
              if (!v3) goto label_631c;
              v23 = NULL;
              v17 = v39;
              v12 = v39;
            }
            if (v3 == '=') {
              if ((v17 == v12) || (v23)) goto label_631c;
              *v12 = '\0';
              v23 = v39;
              v12 = v39;
              goto label_62d6;
            }
            v12 = v39;
          } while (!v23);
        } while (v3 == ';');
      } while ((unsigned int)((int)v3 - 0x30U) <= 9);
    }
label_631c:
    if ((dat_2b068 == v16) || (dat_2b060 == v16))
      error(0,0,dcgettext(NULL,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5),v16,v16);
  }
  v21 = 1;
  v37 = 1;
  v35 = 0;
  do {
    while (*(char *)(v21 + 0x2b183) == '\x01') {
label_5aa0:
      v21 += 1;
      if (v21 == 0x100) goto label_5ad9;
    }
    v7 = v37;
    if (v35 & 0xff & (unsigned int)v21) goto label_5aa0;
    do {
      v37 = v7;
      v7 = v37 * 2;
    } while ((int)(v37 * 2) <= (int)(unsigned int)v21);
    v21 += 1;
    v35 |= v37;
  } while (v21 != 0x100);
label_5ad9:
  dat_2b868 = (unsigned long)(v35 & 0xff) * 0x101010101010101;
  v10 = 0;
  if (0 <= v46)
    v10 = v46;
  if (3 <= v46) goto label_5d5b;
  if (dat_2b687) {
    v40 = (int *)0x2b284;
    v41 = (char *)0x2b6a0;
    do {
      if (*v40 != -1) {
        v11 = sub_16380(*v40,v32);
        if (1 <= v11) {
          v43 = (char *)((long)v32 + (long)v11 * 4);
          do {
            v4 = *(unsigned int *)&v43[-4];
            v33 = 0;
            if (wcrtomb(&v25,v4,&v33) != 1) {
              v9 = 0xff;
              goto label_619c;
            }
            v43 = &v43[-4];
          } while (&v34[(unsigned long)(unsigned int)(v11 - 1) * -4 + (long)v11 * 4] != v43);
        }
        v9 = 1;
label_619c:
        *v41 = v9;
      }
      v40 = &v40[1];
      v41 = &v41[1];
    } while (v40 != (int *)0x2b684);
  }
  if (v46 != 2) {
    if (2 <= dat_2b8d0) {
      v13 = (void *)sub_1ef30(v22);
      v32[0] = 0;
      v22 = v48;
      v15 = v13;
      v44 = v47;
      while (v22) {
        v8 = *v44;
        if (v8 == 0x5b) goto label_5d53;
        if ('\\' <= (char)v8) { // branch-flip
          if (v8 != 0x5c) { // branch-flip
            if (v8 == 0x5e) goto label_5d53;
            if ((unsigned char)(v8 + 0x85) <= 1) goto label_61c8;
          }
          else if (2 <= (long)v22) {
            v8 = v44[1];
            if ('=' <= (char)v8) { // branch-flip
              if ((unsigned char)(v8 - 0x3e) <= 0x3e) {
                v14 = 1L << (v8 - 0x3e & 0x3f);
                if (!(v14 & 0x220001402200011)) {
                  v14 &= 0x6000000000000002;
label_6596:
                  if ((!v14) || (v46 == 1)) goto label_622b;
                  v10 = 0;
                }
                goto label_5d53;
              }
            }
            else if ('\n' <= (char)v8) {
              v14 = 1L << (v8 & 0x3f);
              if (!(v14 & 0x13fe008000000400)) {
                v14 &= 0xb0000000000;
                goto label_6596;
              }
              goto label_5d53;
            }
label_622b:
            v44 = &v44[1];
            v22 = (long *)((long)v22 - 1);
          }
        }
        else if ('/' <= (char)v8) { // branch-flip
          if (v8 == 0x3f) {
label_61c8:
            if (v46 == 1) goto label_5d53;
          }
        }
        else if ('$' <= (char)v8) {
          v14 = 1L << (v8 & 0x3f);
          if (v14 & 0x441000000000) goto label_5d53;
          if (v14 & 0x90000000000) goto label_61c8;
        }
        if (dat_2b687) { // branch-flip
          if (*(int *)((unsigned long)*v44 * 4 + 0x2b284) != -1) // branch-flip
            v14 = (unsigned long)*(char *)((unsigned long)*v44 + 0x2b6a0);
          else {
            v14 = sub_7d80(v44,v22,v32);
          }
        }
        else {
          v14 = sub_e540(v44,v22,v32);
        }
        if ((long)v14 < 0) goto label_5d53;
        v42 = &v44[v14];
        v15 = mempcpy(v15,v44,v14);
        v22 = (long *)((long)v22 - v14);
        v44 = v42;
      }
      v10 = 2;
      v22 = (long *)((long)v15 - (long)v13);
      if (v48 != v22) {
        v31 = v22;
        *(char *)mempcpy(v47,v13,(unsigned long)v22) = 10;
      }
label_5d53:
      free(v13);
    }
    goto label_5d5b;
  }
  if (dat_2b180) { // branch-flip
    v32[0] = 0;
    for (v21 = 0; v21 < (long)v48; v21 = v21 + v14) {
      v3 = *(char *)((unsigned long)v47[v21] + 0x2b183);
      v14 = (unsigned long)v3;
      if (((v3 != '\xfe') || (v14 = sub_1ac90(&v47[v21],(long)v48 - v21,v32), 0x11 <= v14)) && ((long)v14 < 0)) goto label_663d;
    }
    if (dat_2b687) {
label_666b:
      v32[0] = 0;
      for (v21 = 0; v21 < (long)v48; v21 = v21 + v18) {
        v14 = (unsigned long)v47[v21];
        if (*(int *)(v14 * 4 + 0x2b284) != -1) // branch-flip
          v18 = (long)*(char *)(v14 + 0x2b6a0);
        else {
          v18 = sub_7d80(&v47[v21],(long)v48 - v21,v32);
        }
        if (v18 < 0) goto label_663d;
      }
    }
    v8 = 0;
  }
  else {
    v8 = 0;
    if (dat_2b8d0 == 1)
      v8 = dat_2b686;
  }
label_60b9:
  v10 = 2;
  if (v8 & 1) {
    sub_a3e0(0x2b8d8,&v31);
    v47 = dat_2b8d8;
    v10 = 0;
  }
label_5d5b:
  v21 = (long)v10 * 0x20;
  dat_2b878 = *(void **)(v21 + 0x2a7b8);
  dat_2b870 = (**(void **)(v21 + 0x2a7b0))(v47,v31,(long)*(int *)(v21 + 0x2a7ac),dat_2b8fd || dat_2b900);
  v29 = dat_2b684;
  v25 = '\0';
  v53 = 0;
  v21 = (*dat_2b878)(dat_2b870,&v29,1,v28,0);
  dat_2b820 = (v21 == 0) == (bool)dat_2b802;
  a0 -= optind;
  if ((v50) || (2 <= a0))
    dat_2b808 = ~v50 >> 0x1f;
  else {
    dat_2b808 = -(unsigned int)(dat_2b018 == 3);
  }
  v14 = sysconf(0x1e);
  if (0x3ffffffffffffffa < v14 - 1)
    abort(); // no-return
  v5._8_8_ = 0;
  v5._0_8_ = v14;
  v21 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v5._0_16_,0);
  dat_2b858 = 0x18000;
  if (v21)
    dat_2b858 = (v14 + 0x18000) - v21;
  dat_2b858 = v14 + 8 + dat_2b858;
  dat_2b838 = v14;
  dat_2b860 = sub_1ef30(dat_2b858);
  if ((dat_2b014 & 2) && (!dat_2b884))
    dat_2b884 = 1;
  if (1 <= a0) // branch-flip
    a1 = &a1[optind];
  else if ((dat_2b018 != 3) || (v49 <= 0))
    a1 = (char **)0x29f80;
  else {
    dat_2b88a = 1;
    a1 = (char **)0x29f90;
  }
  v16 = *a1;
  v37 = 1;
  do {
    while( true ) {
      a1 = &a1[1];
      if (!strcmp(v16,"-")) break;
      dat_2b890 = v16;
      v35 = sub_a0f0(0xffffff9c,v16,1,1);
      v16 = *a1;
      v37 &= v35;
      if (!v16) goto label_5f29;
    }
    dat_2b890 = dat_2b7d0;
    v35 = sub_9130(0,1);
    v16 = *a1;
    v37 &= v35;
  } while (v16);
label_5f29:
  if (dat_2b889)
    return 2;
  return (unsigned long)v37;
label_663d:
  v8 = 1;
  goto label_60b9;
}


// Function: sub_6840 @ 0x6840
unsigned int sub_6840(long a0,unsigned long a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe
{
  unsigned int v1;
  void *v10; // stack - 0x68
  unsigned long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  long v2; // rax
  char *v3; // rax
  unsigned long *v4; // rbx
  unsigned long v5; // stack - 0x88
  unsigned long v6; // stack - 0x90
  unsigned long v7; // stack - 0x80
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v1 = a6;
  v5 = 0;
  v7 = 0;
  if ((dat_2b687) || ((char)a6)) {
    v10 = NULL;
    v11 = 0;
    if ((char)a6) {
      re_set_syntax(a5 | 0x2000000);
      v2 = re_compile_pattern(a1,a2,&v5);
      if (!v2) {
        regfree(&v5);
        return v1;
      }
      goto label_68e7;
    }
  }
  else {
    v10 = (void *)sub_1ef30(0x100);
  }
  v11 = 0;
  re_set_syntax(a5);
  v2 = re_compile_pattern(a1,a2,&v5);
  if (!v2) {
    v4 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
    *v4 = v5;
    v4[1] = v7;
    v4[2] = v8;
    v4[3] = v9;
    v4[4] = v10;
    v4[5] = v11;
    v4[6] = v12;
    v4[7] = v13;
    return 1;
  }
label_68e7:
  free(v10);
  if ((a4 != -1) && (v3 = (char *)sub_9080(a4,&v6), *v3)) {
    error(0,0,"%s:%td: %s",v3,v6,v2);
    return 0;
  }
  error(0,0,"%s",v2);
  return 0;
}


// Function: sub_6a10 @ 0x6a10
void sub_6a10(unsigned long a0)
{
  error(2,0,"%s",a0);
}


// Function: sub_6a40 @ 0x6a40
void sub_6a40(unsigned long a0)
{
  error(0,0,dcgettext(NULL,"warning: %s",5),a0); // tail-call
}


// Function: sub_6a70 @ 0x6a70
unsigned long * sub_6a70(char *a0,unsigned long a1,unsigned long a2,char a3)
{
  char v1;
  long v10; // rax
  char *v11;
  unsigned long v12;
  char *v13; // rax
  void *v14; // rax
  long v15; // rdx
  long v16; // stack - 0x50
  long v17; // stack - 0x48
  char *v18;
  long v19;
  unsigned char v2;
  unsigned long v20; // stack - 0xb0
  void *v21; // stack - 0xa0
  char *v22; // stack - 0x90
  unsigned long v23; // stack - 0x88
  char *v24; // stack - 0x80
  char *v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  long v3;
  bool v4;
  bool v5;
  char v6;
  unsigned long *v7; // rax
  unsigned long v8;
  char *v9; // rax
  
  v7 = (unsigned long *)sub_1f2f0(1,0x48);
  v8 = sub_15a90();
  v7[1] = v8;
  if (dat_2b687)
    a2 |= 0x400000;
  sub_15fa0(v8,0x2b180,a2,(unsigned int)(dat_2b684 == '\0') * 2 | (unsigned int)a2 & 0x10 | 0x2c);
  v6 = dat_2b182;
  v1 = dat_2b180;
  v9 = &a0[a1];
  v5 = 0;
  v7[3] = 0;
  v7[2] = sub_1ef10(0x40) + 0x40;
  v19 = 0;
  v16 = 1;
  v17 = 0;
  v23 = 0;
  v24 = NULL;
  v18 = a0;
  v22 = a0;
  do {
    v21 = rawmemchr(v18,10);
    v10 = (long)v21 - (long)v18;
    v15 = v10 + -1;
    if (0 <= v15) {
      v11 = v18;
      while (v11 = memchr(v11,0x5c,(long)&v18[v15] - (long)v11), v11) {
        if ((unsigned char)(v11[1] - 0x31U) <= 8) {
          if (v18 <= v22) { // branch-flip
            v20 = 0;
            v4 = 1;
          }
          else {
            v12 = (long)v18 - (long)v22;
            v15 = (v23 - v17) + v12;
            if (1 <= v15)
              v24 = (char *)sub_1f1f0(v24,&v17,v15,0xffffffffffffffff,1);
            memcpy(&v24[v23],v22,v12);
            v23 += v12;
            v4 = 1;
            v20 = 0;
          }
          goto label_6bfa;
        }
        if (((int)v11[1] == (-(unsigned int)(v1 == '\x01' && !v6) & 0x24) + 0x5c) && (v11 = &v11[1], v11 == &v18[v15])) break;
        v11 = &v11[1];
      }
    }
    v20 = 1;
    v4 = 0;
label_6bfa:
    v15 = (v7[3] - v16) + 2;
    if (1 <= v15) {
      v3 = v7[2];
      v7[2] = sub_1f1f0(v3 + -0x40,&v16,v15,0xffffffffffffffff,0x40) + 0x40;
    }
    re_set_syntax(a2);
    v8 = v7[3];
    if (!sub_6840(v7,v18,v10,v8,v19,a2,v20))
      v5 = 1;
    v18 = (char *)((long)v21 + 1);
    v19 += 1;
    if (v4) {
      v7[3] = v7[3] + 1;
      v22 = v18;
    }
    if (v9 < v18) {
      if (v5)
        exit(2); // no-return
      if (v22 <= v9) {
        if (v22 <= a0) { // branch-flip
          v23 = a1;
          v24 = a0;
        }
        else {
          v12 = v23 + ((long)v9 - (long)v22);
          v24 = (char *)sub_1efa0(v24,v12);
          memcpy(&v24[v23],v22,(long)v9 - (long)v22);
          v23 = v12;
        }
      }
      if ((dat_2b686) || (v18 = NULL, v25 = a0, v26 = a1, dat_2b685)) {
        v18 = (char *)sub_1ef30(a1 + 0x2d);
        if (dat_2b685) { // branch-flip
          if (a2 & 0x2000) { // branch-flip
            *(unsigned short *)v18 = dat_21097;
            v18[2] = dat_21099;
            v12 = strlen(v18);
            v19 = v12 + a1;
            memcpy(&v18[v12],a0,a1);
            v9 = ")$";
          }
          else {
            *(unsigned int *)v18 = dat_2109a;
            v12 = strlen(v18);
            v19 = v12 + a1;
            memcpy(&v18[v12],a0,a1);
            v9 = "\\)$";
          }
        }
        else if (a2 & 0x2000) { // branch-flip
          *(unsigned long *)v18 = s_21060._0_8_;
          *(unsigned long *)&v18[8] = s_21060._8_8_;
          *(unsigned short *)&v18[0x10] = s_21060._16_2_;
          v18[0x12] = s_21060[0x12];
          v12 = strlen(v18);
          v19 = v12 + a1;
          memcpy(&v18[v12],a0,a1);
          v9 = ")([^[:alnum:]_]|$)";
        }
        else {
          *(unsigned long *)v18 = s_21080._0_8_;
          *(unsigned long *)&v18[8] = s_21080._8_8_;
          *(unsigned int *)&v18[0x10] = s_21080._16_4_;
          *(unsigned short *)&v18[0x14] = s_21080._20_2_;
          v18[0x16] = s_21080[0x16];
          v12 = strlen(v18);
          v19 = v12 + a1;
          memcpy(&v18[v12],a0,a1);
          v9 = "\\)\\([^[:alnum:]_]\\|$\\)";
        }
        v9 = strcpy(&v18[v19],v9);
        v25 = v18;
        v26 = strlen(v9) + v19;
      }
      sub_14cf0(v25,v26,v7[1]);
      v9 = (char *)sub_15020(v7[1]);
      if (v9) {
        v11 = &v9[3];
        v8 = sub_e5c0(0);
        v1 = *v9;
        *v7 = v8;
        if (v1) { // branch-flip
          v7[7] = v7[7] + 1;
          v12 = strlen(v11);
          v19 = (unsigned char)v9[1] + v12 + (unsigned long)(unsigned char)v9[2];
          v13 = (char *)sub_1ef30(v19);
          v1 = dat_2b684;
          *v13 = dat_2b684;
          v2 = v9[1];
          *(unsigned char *)&v7[8] = *(unsigned char *)&v7[8] | v2;
          v14 = memcpy(&v13[v2],v11,v12);
          if (v9[2])
            *(char *)((long)v14 + v12) = v1;
          sub_c9d0(*v7,v13,v19);
          free(v13);
        }
        else {
          sub_c9d0(v8,v11,strlen(v11));
        }
        sub_cec0(*v7);
        sub_15a80(v9);
      }
      sub_15aa0(0,0,v7[1],1);
      if (v24) {
        if ((a3) || (v8 = v7[1], !sub_14e30(v8))) {
          v7[2] = v7[2] + -0x40;
          v7[3] = v7[3] + 1;
          if (!sub_6840(v7,v24,v23,0,0xffffffffffffffff,a2,0))
            abort(); // no-return
        }
        if (v25 != v24)
          free(v24);
      }
      free(v18);
      return v7;
    }
  } while( true );
}


// Function: sub_7110 @ 0x7110
long sub_7110(long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  long *v1;
  long v10;
  void *v11; // rax
  void *v12;
  int v13;
  void *v14; // rbx
  long v15 [3]; // stack - 0x58
  long v16; // stack - 0x60
  char v17; // stack - 0x69
  void *v18; // stack - 0x68
  int v19; // esi
  unsigned char *v2;
  long v20;
  void *v21;
  void *v22;
  bool v23; // zf
  void *v24; // stack - 0xf8
  void *v25; // stack - 0xe0
  long v26; // stack - 0xd8
  void *v27; // stack - 0xb0
  long v28; // stack - 0xa0
  long v3;
  char v4;
  char v5; // al
  int v6;
  long v7; // rax
  long v8;
  long v9;
  
  v4 = dat_2b684;
  v14 = (void *)(a2 + (long)a1);
  v8 = a0[1];
  v7 = sub_14e10(v8);
  v5 = sub_14e20(v8);
  if (v14 <= a1)
    return -1;
  v25 = a1;
  v18 = a1;
  do {
    while (a4) {
      v22 = v25;
      v25 = v14;
      v27 = a4;
label_73d9:
      v28 = (long)v25 - (long)v22;
      if (0x80000000 < v28)
        sub_1f410(); // no-return, return-dupe
      if (1 <= a0[3]) {
        v1 = &a0[4];
        v26 = 0;
        v24 = v25;
        v12 = (void *)((long)v25 - 1);
        v10 = (long)v25 - (long)v27;
        v8 = 0;
        v19 = (int)v22;
        do {
          while( true ) {
            v9 = v8 * 0x40;
            v20 = a0[2] + v9;
            *(unsigned char *)(v20 + 0x38) = (dat_2b684 == '\n') << 7 | *(unsigned char *)(v20 + 0x38) & 0x3f;
            v6 = re_search(v20,v22,(int)v28 + -1,(int)v27 - v19,(int)v10 + -1,v1);
            if (v6 < -1) {
              sub_1f410();
            }
            if (v6 != -1) break;
label_7528:
            v8 += 1;
            if (a0[3] <= v8) goto label_75d0;
          }
          v21 = (void *)((long)v6 + (long)v22);
          if (v24 < v21) goto label_7528;
          v20 = (long)(*(int *)a0[6] - v6);
          if (a4) {
            if (dat_2b686) {
              if (!dat_2b685) goto label_748d;
              v23 = v10 + -1 != v20;
              v20 = v10;
              v21 = v27;
              if (v23) goto label_7528;
            }
label_761a:
            if (v24 <= v21) { // branch-flip
              if (v24 == v21) {
                if (v20 <= v26)
                  v20 = v26;
                v26 = v20;
              }
            }
            else {
              v24 = v21;
              v26 = v20;
            }
            goto label_7528;
          }
          if (!dat_2b685) {
            if (dat_2b686) {
label_748d:
              do {
                if ((!sub_e820((long)v21 + v20,v12)) && (!sub_e830(v22,v21,v12))) {
                  if (a4) goto label_761a;
                  goto label_7690;
                }
                v6 = (int)v21; // branch-flip
                if (1 <= v20) {
                  v3 = a0[2];
                  v2 = (unsigned char *)(v9 + v3 + 0x38);
                  *v2 = *v2 | 0x40;
                  v13 = re_match(v9 + v3,v22,(v6 + -1 + (int)v20) - (int)v27,v6 - v19,v1);
                  if (v13 < -1) {
                    sub_1f410();
                  }
                  if (v13 <= 0) goto label_74c2;
                }
                else {
label_74c2:
                  if (v12 == v21) break;
                  v20 = a0[2];
                  v2 = (unsigned char *)(v9 + v20 + 0x38);
                  *v2 = *v2 & 0xbf;
                  v6 = re_search(v9 + v20,v22,(int)v28 + -1,(v6 + 1) - v19,((int)v25 - (v6 + 1)) + -1,v1);
                  if (v6 < 0) {
                    if (v6 != -1) {
                      sub_1f410();
                    }
                    goto label_7528;
                  }
                  v13 = *(int *)a0[6] - v6;
                  v21 = (void *)((long)v22 + (long)v6);
                }
                v20 = (long)v13;
              } while (v21 <= v24);
              goto label_7528;
            }
            goto label_7690;
          }
          if (v20 == v10 + -1) goto label_7690;
          v8 += 1;
        } while (v8 < a0[3]);
label_75d0:
        if (v24 < v25) {
          v28 = v26;
          v22 = v24;
          goto label_7690;
        }
      }
label_75e2:
      if (v14 <= v25)
        return 0xffffffffffffffff;
    }
    v17 = '\0';
    v16 = 0;
    v12 = v14;
    if (*a0) {
      v8 = sub_e3e0(*a0,(long)v25 - (unsigned long)*(unsigned char *)&a0[8],(unsigned long)*(unsigned char *)&a0[8] + ((long)v14 - (long)v25),v15,1);
      if (v8 <= -1)
        return v8;
      v21 = (void *)((long)v25 + v8);
      v19 = (int)v4;
      v9 = memrchr(a1,v19,(long)v21 - (long)a1);
      v10 = v15[0];
      v22 = (void *)(v9 + 1);
      if (!v9)
        v22 = a1;
      v9 = a0[7];
      if ((v5 == '\x01') && (v9 <= v15[0])) {
        v10 = (long)v21 - (long)v22;
        if (v10 <= 0xf)
          v10 = 0x10;
        if (v8 >> 2 <= v10) { // branch-flip
          if ((long)v14 - (long)v25 >> 2 <= v10) // branch-flip
            v25 = v22;
          else {
            v12 = (void *)((long)rawmemchr((void *)((long)v25 + v10 * 4),v19) + 1);
            v25 = v22;
          }
        }
        else {
          v12 = (void *)((long)rawmemchr(v21,v19) + 1);
          v25 = v22;
        }
        goto label_72a0;
      }
      v12 = (void *)((long)rawmemchr(v21,v19) + 1);
      if (v9 <= v10) {
        v25 = v22;
        goto label_72a0;
      }
      v25 = v12;
      if ((dat_2b180 == '\x01') && (!dat_2b182)) {
        if (v18 < v22)
          v18 = v22;
        if (!sub_e640(&v18,0,v21,v14)) goto label_782b;
        v21 = v18;
        goto label_7335;
      }
label_782b:
      v28 = (long)v25 - (long)v22;
label_7690:
      *a3 = v28;
      return (long)v22 - (long)a1;
    }
label_72a0:
    if (!v7) {
      v21 = v25;
      v22 = v25;
      v25 = v12;
label_7335:
      v12 = (void *)sub_14e00(a0[1],v21,v25,0,&v16,&v17);
      if ((!v12) || (v25 == v12)) goto label_75e2;
      if (v16)
        v22 = (void *)(memrchr(a1,(int)v4,(long)v12 - (long)a1) + 1);
      v25 = (void *)((long)rawmemchr(v12,(int)v4) + 1);
      if (v17) {
        v27 = v22;
        goto label_73d9;
      }
      goto label_782b;
    }
    v11 = (void *)sub_14e00(v7,v25,v12,0,&v16,0);
    if ((v11) && (v12 != v11)) {
      if (v16)
        v25 = (void *)(memrchr(a1,(int)v4,(long)v11 - (long)a1) + 1);
      v16 = 0;
      v21 = v25;
      v22 = v25;
      v25 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
      goto label_7335;
    }
    v25 = v12;
    if (v14 <= v12)
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
      v4 = sub_1a700(dat_2b8c8,a1 + 1,0);
      if (v4) {
        if (v4 < 0)
          sub_1f410(); // no-return
        v6 = (void *)((long)v6 + ((long)v1 - (long)v8));
        if (!v9) {
          v5 = dat_2b8f0;
          if (dat_2b8e0 == dat_2b8e8) {
            v5 = sub_1f1f0(dat_2b8f0,0x2b8e8,1,0xffffffffffffffff,0x18);
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
  unsigned long v10; // stack - 0xa0
  unsigned long v11; // stack - 0x98
  unsigned long v12; // stack - 0x90
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  int v2; // eax
  unsigned long v20; // stack - 0x18
  unsigned int v3; // stack - 0xd8
  char v4 [8];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xb0
  unsigned long v9; // stack - 0xa8
  
  if (v1) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v6 = &Stack0000000000000008;
  v3 = 8;
  v7 = v4;
  v5 = 0x30;
  v8 = a9;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  v2 = __vfprintf_chk(stdout,1,a8,&v3);
  if (0 <= v2)
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
  int v1; // eax
  char *v2;
  
  v2 = dat_2b890;
  if ((dat_2b900) && (*dat_2b058)) {
    sub_1faa0(dat_2b028);
    v2 = dat_2b890;
  }
  dat_2b890 = v2;
  if (!v2) {
    v2 = dcgettext(NULL,"(standard input)",5);
    dat_2b890 = v2;
  }
  v1 = fputs_unlocked(v2,stdout);
  if (v1 <= -1)
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
void sub_82a0(unsigned long a0,unsigned long a1,char a2)
{
  unsigned long v1;
  unsigned long v10;
  bool v11; // zf
  char *v12; // stack - 0x58
  char *v13; // stack - 0x50
  unsigned char v2;
  char v3;
  int v4;
  long v5;
  unsigned long v6;
  unsigned char v7; // cl
  long v8; // stack - 0x48
  unsigned long v9;
  
  if ((!dat_2b8fd) && (!sub_8050(a0,(a1 - a0) + -1,a1,(int)a2)))
    return;
  v11 = a2 == ':';
  v7 = (v11 ^ dat_2b802) & dat_2b8fd;
  v2 = dat_2b8fd;
  if (dat_2b900) { // branch-flip
    v13 = dat_2b038;
    if (v11 == (bool)((unsigned char)((unsigned int)dat_2b900 >> 0x1f) & dat_2b802))
      v13 = dat_2b030;
    v12 = dat_2b068;
    if (a2 != ':')
      v12 = dat_2b060;
    if (v7) goto label_84c2;
    v9 = a0;
    if (!*v13) {
      if (!*v12) goto label_82fd;
      if (v11 == (bool)dat_2b802) goto label_83d0;
label_83b0:
      if (a0 < a1) goto label_84cb;
      goto label_83c0;
    }
    if (v11 == (bool)dat_2b802) goto label_83d0;
    if (*v12) goto label_83b0;
label_83f7:
    a0 = a1;
    if (v9 < a1) {
      v6 = 0;
      if (*(char *)(a1 - 1) == dat_2b684) {
        a0 = a1 - 1;
        if (a0 <= v9) goto label_8430;
        v6 = 1;
      }
      if (*(char *)(a1 + ~v6) == '\r')
        a0 = a1 - (v6 + 1);
    }
label_8430:
    if (1 <= (long)(a0 - v9)) { // branch-flip
      sub_1faa0(dat_2b028,v13);
      sub_7e40(v9,a0 - v9);
      if (*v13)
        sub_1fac0(dat_2b020);
      v2 = dat_2b8fd;
    }
    else {
      a0 = v9; // crossjump-dupe
      v2 = dat_2b8fd;
    }
  }
  else if (v7) {
    v12 = NULL;
    v13 = NULL;
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
      v3 = *v13;
    }
    else {
label_84cb:
      v9 = 0;
      v6 = a0;
      do {
        v5 = (*dat_2b878)(dat_2b870,a0,a1 - a0,&v8,v6);
        if ((v5 < 0) || (v1 = a0 + v5, a1 == v1)) break;
        if (v8) { // branch-flip
          if (dat_2b8fd) { // branch-flip
            v3 = sub_8050(v1,v8,a1,(-(dat_2b802 == 0) & 0xdU) + 0x2d);
            if (!v3)
              return;
          }
          else {
            if (*v13)
              sub_1faa0(dat_2b028,v13);
            v10 = v9;
            if (!v9)
              v10 = v6;
            v9 = 0;
            sub_7e40(v10,v1 - v10);
          }
          if ((dat_2b900) && (*v12))
            sub_1faa0(dat_2b028,v12);
          sub_7e40(v1,v8);
          if ((dat_2b900) && (*v12))
            sub_1fac0(dat_2b020);
          if (dat_2b8fd)
            sub_7e70((int)dat_2b684);
          v5 = v8;
        }
        else {
          v5 = 1;
          v8 = 1;
          if (!v9)
            v9 = v6;
        }
        v6 = v1 + v5;
      } while (v6 < a1);
      if (dat_2b8fd) goto label_86b0;
      a0 = v6;
      if (!v9) goto label_83c0;
      v3 = *v13;
    }
    if (v3) goto label_83f7;
    a0 = v9;
    v2 = dat_2b8fd;
  }
label_82fd:
  if ((a0 < a1) && (v2 != 1))
    sub_7e40(a0,a1 - a0);
label_8318:
  if ((dat_2b7d8) && (v4 = fflush_unlocked(stdout), v4)) {
    v4 = *__errno_location();
    dat_2b8c0 = v4;
  }
  else {
    v4 = dat_2b8c0;
  }
  if (!v4) {
    dat_2b7b8 = a1;
    return;
  }
  error(2,v4,dcgettext(NULL,"write error",5));
}


// Function: sub_8710 @ 0x8710
void sub_8710(void *a0) // return-dupe x2
{
  void *v1;
  void *v2; // rax
  
  if (!dat_2b7b8)
    dat_2b7b8 = dat_2b848;
  if (dat_2b7a8 <= 0)
    return;
  do {
    v1 = dat_2b7b8;
    if (a0 <= dat_2b7b8)
      return;
    v2 = rawmemchr(dat_2b7b8,(int)dat_2b684);
    sub_82a0(v1,(long)v2 + 1,0x2d);
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
  int v3;
  long v4; // rax
  void *v5;
  unsigned long v6; // rax
  void *v7;
  void *v8;
  long v9; // stack - 0x48
  
  v2 = dat_2b7b0;
  if (a1 <= a0)
    return 0;
  do {
    v4 = (*dat_2b878)(dat_2b870,a0,(long)a1 - (long)a0,&v9,0);
    v1 = dat_2b684;
    v13 = dat_2b7b0;
    if (0 <= v4) { // branch-flip
      v14 = (void *)((long)a0 + v4);
      v5 = (void *)(v9 + (long)v14);
      if (!dat_2b802) {
        if (a1 != v14) {
          if (!dat_2b803) {
            v7 = v14;
            v14 = v5;
            goto label_8847;
          }
          v14 = v5;
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
      if ((!dat_2b802) || (v9 = 0, a1 <= a0)) {
        return v2 - v13;
      }
      v5 = a1;
      v14 = a1;
label_88dc:
      if (dat_2b803) { // branch-flip
label_88f3:
        v11 = dat_2b802;
        if (!dat_2b802) goto label_8a78;
label_8911:
        v4 = 0;
        v13 = dat_2b7b0;
        if (a0 < v14) {
          for (; v4 < v13; v4 = v4 + 1) {
            v7 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
            if (!v11) {
              sub_82a0(a0,v7,0x3a);
              v13 = dat_2b7b0;
              v11 = dat_2b803;
            }
            a0 = v7;
            if (v14 <= v7) {
              dat_2b7b0 = v13 - (v4 + 1);
              goto label_8984;
            }
          }
          dat_2b7b0 = v13 - v4;
        }
      }
      else {
        v7 = a0;
label_8847:
        a0 = v7;
        if ((0 < (long)dat_2b7a8) && (sub_8710(v7), dat_2b803)) goto label_88f3;
        v8 = dat_2b7b8;
        if (!dat_2b7b8)
          v8 = dat_2b848;
        v6 = 0;
        v12 = v7; // branch-flip
        if (1 <= (long)dat_2b7f8) {
          do {
            a0 = v12;
            if (v8 < v12) {
              do {
                a0 = (void *)((long)v12 - 1);
                if (v1 == *(char *)((long)v12 - 2)) break;
                a0 = (void *)((long)v12 - 2);
                v10 = (char *)((long)v12 - 3);
                v12 = a0;
              } while (v1 != *v10);
            }
            v6 += 1;
            v12 = a0;
          } while (v6 != dat_2b7f8);
          if (((dat_2b688) && (dat_2b7b8 != a0)) && (dat_2b070)) {
label_8b46:
            v10 = dat_2b070;
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1faa0(dat_2b028);
              v10 = dat_2b070;
            }
            v3 = fputs_unlocked(v10,stdout);
            if (v3 <= -1)
              dat_2b8c0 = *__errno_location();
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1fac0(dat_2b020);
              sub_7e70(10);
            }
            else {
              sub_7e70(10);
            }
          }
          if (a0 < v7) {
            do {
              v8 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
              sub_82a0(a0,v8,0x2d);
              a0 = v8;
            } while (v8 < v7);
          }
        }
        else if ((((0 <= (long)(dat_2b7f8 & dat_2b7f0)) && (dat_2b7b8 != v7)) && (dat_2b688)) && (dat_2b070)) goto label_8b46;
        v11 = dat_2b803;
        if (dat_2b802) goto label_8911;
        if (dat_2b803) goto label_8a78;
        sub_82a0(v7,v14,0x3a);
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
          v3 = 0;
          if (dat_2b889)
            v3 = dat_2b080;
          exit(v3); // no-return
        }
        return v2 - v13;
      }
    }
    a0 = v5;
    v13 = dat_2b7b0;
    if (a1 <= v5)
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
  unsigned long v14;
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
      v2 = (void *)sub_1f1f0(0,0x2b858,v6,v12,1);
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
  while (v6 = sub_1da00(v15,v13,v9 - v9 % (long)v14), v15 = dat_2b850, v6 != -1) {
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
  unsigned long v10; // rax
  unsigned long v11; // rax
  long v12;
  char *v13;
  long v14; // rax
  int *v15; // rax
  unsigned long v16;
  unsigned long v17; // rax
  long v18; // stack - 0x168
  char *v19; // stack - 0x178
  char *v2;
  char v20 [24];
  char v21;
  unsigned int v22; // edx
  unsigned long v23; // rax
  long v24;
  unsigned char v25; // bl
  char v26;
  char *v27;
  char *v28;
  long v29; // stack - 0x1b8
  unsigned int v3;
  long v30; // stack - 0x1b0
  long v31; // stack - 0x1a8
  char v32; // stack - 0x17b
  unsigned long v33; // stack - 0x170
  long v34; // stack - 0x160
  unsigned int v35; // stack - 0x150
  long v36; // stack - 0x138
  unsigned int v37; // stack - 0xc0
  char v4;
  char v5;
  char v6;
  short v7;
  int v8;
  unsigned int v9;
  
  if (fstat(a0,&v18)) { // branch-flip
    v11 = 1;
    sub_8cb0(*__errno_location());
label_98d2:
    if (!a0) {
      return v11 & 0xffffffff; // return-dupe
    }
label_9454:
    if (!close(a0)) {
      return v11 & 0xffffffff;
    }
  }
  else {
    if (a0) {
      if (dat_2b884 != 2) { // branch-flip
        if ((dat_2b884) || (a1 == 1)) {
label_94a4:
          if (a1) {
            if ((v35 & 0xf000) != 0x4000) // branch-flip
              v12 = 0x2b8b0;
            else {
              if (dat_2b018 == 4) goto label_944e;
              v12 = 0x2b8a0;
              if (dat_2b88a) goto label_94e0;
            }
            if ((*(long *)(v12 + 8)) && (v6 = sub_16b90(*(long *)(v12 + 8),dat_2b890), v6)) goto label_944e;
          }
        }
        else if (((v35 & 0xb000) == 0x2000 || (v35 & 0xf000) == 0xc000) || ((v35 & 0xf000) == 0x1000)) goto label_944e;
label_94e0:
        if (0 <= (int)dat_2b808) { // branch-flip
          if (dat_2b018 == 3) goto label_9a0e;
label_94fd:
          if ((dat_2b018 == 4) && ((v35 & 0xf000) == 0x4000)) goto label_944e;
        }
        else {
          dat_2b808 = (unsigned int)((v35 & 0xf000) == 0x4000);
          if (dat_2b018 != 3) goto label_94fd;
label_9a0e:
          if ((v35 & 0xf000) == 0x4000) {
            v22 = ~(unsigned int)(a1 ^ 1) & dat_2b014;
            if (close(a0))
              sub_8cb0(*__errno_location());
            v33 = 0;
            v19 = dat_2b890;
            v12 = sub_189c0(&v19,v22,0);
            if (!v12)
              sub_1f410(); // no-return
            v11 = 1;
label_9a8d:
            do {
              v14 = sub_18fd0(v12);
              if (!v14) {
                v15 = __errno_location();
                if (*v15)
                  sub_8cb0(*v15);
                if (sub_18de0(v12))
                  sub_8cb0(*v15);
                return v11 & 0xffffffff;
              }
              v7 = *(short *)(v14 + 0x68);
              v25 = *(long *)(v14 + 0x58) == 0 & a1;
            } while (v7 == 6);
            if (!v25) {
              if (((unsigned short)(v7 - 1U) < 2) || (v24 = dat_2b8b0, v7 == 4)) {
                if (dat_2b018 != 4) {
                  v24 = dat_2b8a0;
                  goto label_9f2d;
                }
label_9f64:
                sub_19720(v12,v14,4);
                goto label_9a8d;
              }
label_9f2d:
              if (v24) {
                if (sub_16b90(v24,v14 + 0x100)) goto label_9f64;
                v7 = *(short *)(v14 + 0x68);
              }
            }
            dat_2b890 = *(char **)(v14 + 0x38);
            if ((dat_2b88a) && (dat_2b890[1]))
              dat_2b890 = &dat_2b890[2];
            v13 = dat_2b890;
            v22 = 1;
            if (!(*(unsigned int *)(v12 + 0x48) & 2))
              v22 = *(unsigned int *)(v12 + 0x48) & (unsigned int)v25;
            switch(v7) {
              default:
                goto label_9f9b;
              case 1:
                if (dat_2b018 == 3) goto label_9a8d;
                sub_19720(v12,v14,4);
                break;
              case 2:
                if (!dat_2b904)
                  error(0,0,dcgettext(NULL,"%s: warning: recursive directory loop",5),v13);
                goto label_9a8d;
              case 3:
              case 0xb:
                if ((dat_2b884 == 2) || ((!v25 && (!dat_2b884)))) {
                  v9 = *(unsigned int *)(v14 + 0x88);
                  if (!*(unsigned int *)(v14 + 0x88)) {
                    v13 = *(char **)(v14 + 0x30);
                    v8 = *(int *)(v12 + 0x2c);
                    v9 = v37;
                    if (fstatat(v8,v13,v20,v22 << 8 ^ 0x100)) {
                      sub_8cb0(*__errno_location());
                      goto label_9a8d;
                    }
                  }
                  if (((v9 & 0xb000) == 0x2000 || (v9 & 0xf000) == 0xc000) || ((v9 & 0xf000) == 0x1000)) goto label_9a8d;
                }
                break;
              case 4:
              case 7:
              case 10:
                sub_8cb0(*(unsigned int *)(v14 + 0x40));
                goto label_9a8d;
              case 8:
              case 0xd:
                break;
              case 0xc:
              case 0xe:
                goto label_9a8d;
              
            }
            v16 = *(unsigned long *)(v14 + 0x30);
            v3 = *(unsigned int *)(v12 + 0x2c);
            v11 = (unsigned long)((unsigned int)v11 & sub_a0f0(v3,v16,v22,v25));
            goto label_9a8d;
          }
        }
        if (((dat_2b884 != 2) && ((dat_2b884 || (a1)))) || (((v35 & 0xb000) != 0x2000 && (v35 & 0xf000) != 0xc000 && ((v35 & 0xf000) != 0x1000)))) goto label_9191;
      }
      else if (((v35 & 0xb000) != 0x2000 && (v35 & 0xf000) != 0xc000) && ((v35 & 0xf000) != 0x1000)) goto label_94a4;
label_944e:
      v11 = 1;
      goto label_9454;
    }
    if ((int)dat_2b808 < 0)
      dat_2b808 = (unsigned int)((v35 & 0xf000) == 0x4000);
label_9191:
    v5 = dat_2b803;
    v4 = dat_2b7a2;
    v6 = dat_2b684;
    if ((((!dat_2b803) && (!dat_2b80c)) && (2 <= dat_2b7e0)) && ((((v35 & 0xf000) == 0x8000 && (v34 == dat_2b928)) && (v18 == dat_2b920)))) {
      if (!dat_2b904) {
        v16 = sub_90f0();
        error(0,0,dcgettext(NULL,"%s: input file is also the output",5),v16);
      }
      dat_2b889 = 1;
      v11 = 1;
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
      v32 = sub_8d20(0,&v18);
      if (v32) { // branch-flip
        dat_2b8f8 = 0;
        if (dat_2b8fc) {
          v12 = 0x7fffffffffffffff;
          if (((v35 & 0xf000) == 0x8000) && (v12 = v36, dat_2b801))
            v12 = v36 + (unsigned long)(v36 != 0x7fffffffffffffff);
          dat_2b8f8 = 0;
          do {
            dat_2b8f8 += 1;
            v12 /= 10;
          } while (v12);
        }
        v30 = -1;
        v29 = 0;
        v31 = 0;
        v12 = 0;
        v21 = '\0';
        v26 = v32;
        do {
          v28 = dat_2b848;
          v13 = dat_2b840;
          v8 = dat_2b810;
          v25 = (unsigned char)((unsigned long)v30 >> 0x3f) & v6 != '\0';
          v11 = (unsigned long)v25;
          if (v25) {
            if (dat_2b810 != 1) {
              *dat_2b840 = '\0';
              v23 = (long)dat_2b840 - (long)dat_2b848;
              v10 = strlen(dat_2b848);
              if (v23 != v10) { // branch-flip
label_9355:
                if (v8 == 2) {
                  v32 = '\0';
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
                v13 = dat_2b840;
                v21 = v6;
              }
              else if (v26) {
                if (!dat_2b881) {
                  if ((v35 & 0xf000) == 0x8000) {
                    if ((long)v23 < v36) {
                      if (!a0) {
                        v23 = lseek(0,0,1);
                        if (0 > (long)v23) goto label_939e;
                      }
                      v14 = lseek(a0,v23,4);
                      if (0 <= v14) {
                        if ((long)lseek(a0,v23,0) < 0)
                          sub_8cb0(*__errno_location());
                        if (v14 < v36) {
                          v8 = dat_2b810;
                          goto label_9355;
                        }
                        v28 = dat_2b848;
                        v13 = dat_2b840;
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
          v2 = &v28[v12];
          dat_2b7c0 = v28;
          if (v2 == v13) {
            if (v29) {
              dat_2b840 = &v13[1];
              *v13 = v6;
              if (dat_2b7b0)
                v31 += sub_8790(&v28[v12 - v29]);
              if (dat_2b7a8)
                sub_8710(dat_2b840);
            }
            goto label_96b9;
          }
          if (v21) {
            v27 = v2;
            while( true ) {
              *v13 = '\0';
              v11 = strlen(v27);
              *v13 = v21;
              v28 = &v27[v11];
              if (&v27[v11] == v13) break;
              do {
                v27 = &v28[1];
                *v28 = v21;
                v28 = v27;
              } while (!*v27);
            }
          }
          v26 = v2[-1];
          v2[-1] = v6;
          v12 = memrchr(&v2[-1],(int)v6,&v13[1 - (long)v2]);
          v2[-1] = v26;
          v27 = &v2[-v29];
          v28 = (char *)(v12 + 1);
          if (v2 != v28) { // branch-flip
            v29 = (long)v13 - (long)v28;
            if (v27 < v28) {
              if (!dat_2b7b0) {
                if (dat_2b7a8) goto label_95c2;
label_97dd:
                v32 = '\0';
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
                v12 = 0;
                if (0 <= v30)
                  v12 = v30;
                if (v12 < v31) goto label_97dd;
              }
            }
          }
          else {
            v29 = (long)v13 - (long)v27;
            v28 = v27;
          }
          if (1 <= dat_2b7f8) {
            v12 = 0;
            do {
              if (v28 <= dat_2b848) break;
              if (v28 == dat_2b7b8) goto label_964f;
              v12 += 1;
              v27 = v28;
              do {
                v28 = &v27[-1];
                v1 = &v27[-2];
                v27 = v28;
              } while (v6 != *v1);
            } while (v12 != dat_2b7f8);
          }
          if (v28 != dat_2b7b8)
            dat_2b7b8 = NULL;
label_964f:
          v12 = (long)v13 - (long)v28;
          if (dat_2b800) {
            v13 = &(&dat_2b840[-v12 - (long)dat_2b848])[(long)dat_2b7c8];
            if (SCARRY8((long)&dat_2b840[-v12 - (long)dat_2b848],(long)dat_2b7c8)) {
              error(2,0,dcgettext(NULL,"input is too large to count",5));
              return v17;
            }
            dat_2b7c8 = v13;
          }
          if (dat_2b801)
            sub_7c90(v28);
          v26 = '\0';
        } while (sub_8d20(v12,&v18));
        sub_8cb0(*__errno_location());
        v32 = '\0';
label_96b9:
        v11 = CONCAT71((undefined7)((unsigned long)v2 >> 8),v31 == 0);
        dat_2b7a2 = v4;
        dat_2b803 = v5;
        if (((!dat_2b810) && (!v5)) && ((dat_2b888 || ((0 <= v30 && (v30 < v31)))))) {
          v13 = dat_2b890;
          if (!dat_2b890) {
            v13 = dcgettext(NULL,"(standard input)",5);
            dat_2b890 = v13;
          }
          error(0,0,dcgettext(NULL,"%s: binary file matches",5),v13);
        }
      }
      else {
        v11 = 1;
        sub_8cb0(*__errno_location());
        v31 = 0;
      }
    }
    else {
      dat_2b830 = lseek(0,0,1);
      dat_2b880 = (unsigned char)((unsigned long)dat_2b830 >> 0x3f);
      dat_2b881 = dat_2b880;
      if (0 <= dat_2b830) goto label_9228;
      v8 = *__errno_location();
      if (v8 == 0x1d) {
        dat_2b830 = 0;
        goto label_9228;
      }
      sub_8cb0(v8);
      v11 = 1;
      v32 = '\0';
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
      if ((dat_2b7d8) && (v8 = fflush_unlocked(stdout), v8))
        dat_2b8c0 = *__errno_location();
    }
    if (dat_2b80c) {
      if (dat_2b80c == (v31 == 0) + 1) {
        sub_7fa0();
        sub_7e70(dat_2b804 & 10);
        if ((dat_2b7d8) && (v8 = fflush_unlocked(stdout), v8))
          dat_2b8c0 = *__errno_location();
      }
      goto label_98d2;
    }
    if (a0) goto label_9454;
    if (!dat_2b7b0) {
      if (((dat_2b830 == dat_2b828) || (dat_2b881)) || (v12 = lseek(0,dat_2b828,0), 0 <= v12)) {
        return v11 & 0xffffffff;
      }
      sub_8cb0(*__errno_location()); // return-dupe
      return v11 & 0xffffffff;
    }
    if ((v32) || ((!dat_2b881 && ((0 <= (long)lseek(0,0,2) || (*__errno_location() == 0x16)))))) {
      return v11 & 0xffffffff;
    }
    if (((v35 & 0xf000) != 0x1000) || (!dat_2b7a0)) {
label_9b90:
      do {
        v12 = sub_1da00(0,dat_2b860,dat_2b858);
        if (!v12) {
          return v11 & 0xffffffff;
        }
      } while (v12 != -1);
    }
    else {
      v12 = splice(0,0,1,0,0x18000,1);
      if (0 <= v12) { // branch-flip
        if (!v12) {
          return v11 & 0xffffffff;
        }
        do {
          v12 = splice(0,0,1,0,0x18000,1);
        } while (0 < v12);
        if (!v12) {
          return v11 & 0xffffffff;
        }
      }
      else if (*__errno_location() == 0x16) goto label_9b90;
    }
  }
  sub_8cb0(*__errno_location());
  return v11 & 0xffffffff;
label_9f9b:
  abort(); // no-return
}


// Function: sub_a0f0 @ 0xa0f0
unsigned long sub_a0f0(unsigned long a0,unsigned long a1,char a2,char a3) // early-return
{
  int v1;
  unsigned long v2; // rax
  int *v3; // rax
  unsigned int v4; // edx
  
  v4 = (-(unsigned int)(a2 == '\0') & 0x20000) + 0x100;
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
  v2 = sub_9130(v1,a3); // tail-call
  return v2;
}


// Function: sub_a170 @ 0xa170
void sub_a170(int a0)
{
  unsigned long v1; // rax
  
  v1 = sub_19950();
  if (a0) { // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    v1 = sub_19950();
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v1);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"Search for PATTERNS in each FILE.\n",5));
    v1 = sub_19950();
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
          v4 = (char *)sub_1f1f0(0,&v7,2,0xffffffffffffffff,1);
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
long sub_a710(unsigned long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  unsigned long v1;
  unsigned int v10; // edx
  void *v11;
  char v12 [16];
  char v13 [16];
  long v14; // stack - 0x80
  void *v15; // stack - 0x88
  void *v16;
  void *v17;
  long v18;
  long v19; // stack - 0x68
  unsigned long v2;
  long v20; // stack - 0x48
  unsigned long v3;
  char v4;
  unsigned int v5; // eax
  long v6;
  long v7; // rax
  unsigned char v8; // cl
  unsigned char v9; // cl
  
  v4 = dat_2b684;
  v16 = (void *)((long)a1 + a2);
  v1 = *a0;
  v5 = (unsigned int)dat_2b685;
  v8 = dat_2b180 & ((dat_2b182 | dat_2b685) ^ 1);
  v9 = v8 | dat_2b686;
  v10 = v5 ^ 1;
  v11 = a1;
  if (a4)
    v11 = a4;
  if (v16 < v11)
    return -1;
  v15 = v11;
  while( true ) {
    v6 = sub_e3e0(v1,(long)v11 - (unsigned long)v5,(long)v16 + ((unsigned long)v5 - (long)v11),v12,(v9 | a4 != NULL) & v10);
    if (v6 <= -1)
      return -1;
    v6 = (long)v11 + v6;
    v14 = 0;
    v18 = v19 - (int)((unsigned int)dat_2b685 * 2);
    if ((!v8) || (!sub_e640(&v15,&v14,v6,v16))) break;
    v11 = v15;
    if (v16 < v15)
      return -1;
label_a98c:
    v5 = (unsigned int)dat_2b685;
  }
  v11 = v15;
  if ((dat_2b686 == 1) || (!a4)) {
    if (!dat_2b685) {
      if (!dat_2b686) {
        v11 = (void *)(v6 + v18);
label_aa4d:
        if (v11 < v16)
          v16 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
label_aa67:
        v18 = memrchr(a1,(int)v4,v6 - (long)a1);
        if (v18) { // branch-flip
          v6 = (v18 + 1) - (long)a1;
          a1 = (void *)(v18 + 1);
        }
        else {
          v6 = 0;
        }
        v18 = (long)v16 - (long)a1;
        goto label_aa18;
      }
      if (v14) { // branch-flip
        if (v14 <= 0) goto label_a894;
        v7 = sub_e820(v6 - v14,v16);
      }
      else {
        v7 = memrchr(v15,(int)v4,v6 - (long)v15);
        if (v7) {
          v11 = (void *)(v7 + 1);
          v15 = v11;
        }
label_a894:
        v7 = sub_e830(v11,v6,v16);
      }
      if (!v7) {
        while( true ) {
          v11 = (void *)(v6 + v18);
          if (!sub_e820(v11,v16)) {
            if (!a4) goto label_aa4d;
            goto label_aa10;
          }
          if ((!a4) && (!dat_2b180)) break;
          if ((!v18) || (sub_e3e0(v1,v6,v18 + -1,v13,1))) goto label_a970;
          v18 = v20;
        }
        if (!a0[4]) {
          sub_a3e0(&a0[2],&a0[3]);
          v2 = a0[3];
          v3 = a0[2];
          a0[4] = sub_6a70(v3,v2,0x10a46,0);
        }
        v17 = v16;
        if (v11 < v16)
          v17 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
        v2 = a0[4];
        if (0 <= (long)sub_7110(v2,v6,(long)v17 - v6,a3,0)) {
          v16 = v17;
          goto label_aa67;
        }
        v6 = (long)v17 - 1;
      }
label_a970:
      v15 = (void *)(v6 + sub_e810(v6,v16));
      v11 = (void *)((long)v15 + 1);
      if (v16 < (void *)((long)v15 + 1))
        return -1;
      goto label_a98c;
    }
    v18 += (unsigned long)(a4 == NULL);
  }
label_aa10:
  v6 -= (long)a1;
label_aa18:
  *a3 = v18;
  return v6;
}


// Function: sub_ab10 @ 0xab10
void sub_ab10(unsigned long *a0,long a1,unsigned long a2,char a3)
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  long *v4;
  long *v5;
  unsigned long *v6;
  
  if (!a0)
    return;
  sub_ab10(*a0,a1,a2,a3);
  sub_ab10(a0[1],a1,a2,a3);
  do {
    if (!a1) {
      *(unsigned long *)(a0[2] + 0x20) = a2;
      return;
    }
    v6 = *(unsigned long **)(a1 + 8);
    if (v6) {
      do {
        v2 = (unsigned char *)&v6[3];
        if (*(unsigned char *)&a0[3] == *v2) {
          v4 = (long *)a0[2];
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
        if (*v2 <= *(unsigned char *)&a0[3])
          v6 = (unsigned long *)*v1;
      } while (v6);
    }
    a1 = *(long *)(a1 + 0x20);
  } while( true );
}


// Function: sub_abd0 @ 0xabd0
unsigned long sub_abd0(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  
  if (!a1)
    return 1;
  v4 = sub_abd0(a0,*a1);
  if (((char)v4) && (v4 = sub_abd0(a0,a1[1]), (char)v4)) {
    if (!a0)
      return 0;
    do {
      v2 = (unsigned char *)&a0[3];
      if (*(unsigned char *)&a1[3] == *v2)
        return v4;
      v3 = (unsigned long *)*a0;
      v1 = &a0[1];
      a0 = v3;
      if (*v2 <= *(unsigned char *)&a1[3])
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
void sub_af60(void)
{
  sub_ace0(); // tail-call
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
long sub_c880(unsigned long a0)
{
  long v1; // rax
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  
  v1 = sub_1ef10(0x9a0);
  _obstack_begin(v1,0,0,sub_1ef10,dat_2afb8);
  v4 = *(unsigned long *)(v1 + 0x20);
  *(unsigned long *)(v1 + 0x58) = 0;
  if (0x40 <= v4 - *(long *)(v1 + 0x18)) { // branch-flip
    v5 = *(unsigned long **)(v1 + 0x10);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  else {
    _obstack_newchunk(v1,0x40);
    v5 = *(unsigned long **)(v1 + 0x10);
    v4 = *(unsigned long *)(v1 + 0x20);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  *(unsigned char *)(v1 + 0x50) = *(unsigned char *)(v1 + 0x50) | 2;
label_c8ea:
  *(unsigned long **)(v1 + 0x60) = v5;
  v3 = (long)v2 + *(unsigned long *)(v1 + 0x30) & ~*(unsigned long *)(v1 + 0x30);
  if (v3 - *(long *)(v1 + 8) <= v4 - *(long *)(v1 + 8))
    v4 = v3;
  *(unsigned long *)(v1 + 0x18) = v4;
  *(unsigned long *)(v1 + 0x10) = v4;
  *v5 = 0;
  v5[1] = 0;
  v5[2] = 0;
  v5[3] = 0;
  v5[4] = 0;
  v5[5] = 0;
  v5[6] = 0;
  *(unsigned long *)(v1 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)(v1 + 0x980) = a0;
  *(void **)(v1 + 0x998) = sub_af60;
  *(unsigned long *)(v1 + 0x970) = 0;
  return v1;
}


// Function: sub_c9d0 @ 0xc9d0
void sub_c9d0(long a0,unsigned char *a1,long a2) // return-dupe
{
  unsigned char v1;
  long *v10;
  long *v11;
  long v12;
  int v13 [12]; // stack - 0xd8
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
  
  v2 = *(void **)(a0 + 0x998);
  v3 = *(long *)(a0 + 0x980);
  v7 = *(long **)(a0 + 0x60);
  if (v2 == sub_b3f0)
    a1 = &a1[a2];
label_ca25:
  do {
    while( true ) {
      if (!a2) {
        v3 = *(long *)(a0 + 0x58);
        if (!*v7)
          *v7 = v3 * 2 + 1;
        *(long *)(a0 + 0x58) = v3 + 1;
        if (*(long *)(a0 + 0x68) <= v7[5])
          return;
        *(long *)(a0 + 0x68) = v7[5];
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
      v13[0] = 0;
      v14[0] = &v7[1];
      v12 = 1;
      if (!v10) break;
      while (v1 = *(unsigned char *)&v10[3], v1 != v15) {
        v14[v12] = v10;
        v18 = v12 + 1;
        if (v15 <= v1) { // branch-flip
          v10 = (long *)*v10;
          v13[v12] = 0;
        }
        else {
          v10 = (long *)v10[1];
          v13[v12] = 1;
        }
        if (!v10) goto label_cabc;
        v12 = v18;
      }
      v7 = (long *)v10[2];
    }
    v12 = 0;
label_cabc:
    v16 = *(unsigned long *)(a0 + 0x20);
    if (0x20 <= v16 - *(long *)(a0 + 0x18)) { // branch-flip
      v19 = *(unsigned long **)(a0 + 0x10);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) {
label_cdc7:
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
    }
    else {
      _obstack_newchunk(a0,0x20);
      v19 = *(unsigned long **)(a0 + 0x10);
      v16 = *(unsigned long *)(a0 + 0x20);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) goto label_cdc7;
    }
    v20 = *(unsigned long *)(a0 + 0x30);
    v22 = ~v20;
    v10 = (long *)((long)v9 + v20 & v22);
    v18 = *(long *)(a0 + 8);
    v21 = v16 - v18;
    if ((unsigned long)((long)v10 - v18) <= v21) { // branch-flip
      *(long **)(a0 + 0x18) = v10;
      *(long **)(a0 + 0x10) = v10;
      *v19 = 0;
      v19[1] = 0;
      if (v16 - (long)v10 <= 0x3f) goto label_cb2f;
      v11 = &v10[8];
      if (v11 == v10) goto label_cd6f;
    }
    else {
      *(unsigned long *)(a0 + 0x18) = v16;
      *(unsigned long *)(a0 + 0x10) = v16;
      *v19 = 0;
      v19[1] = 0;
label_cb2f:
      _obstack_newchunk(a0,0x40);
      v20 = *(unsigned long *)(a0 + 0x30);
      v16 = *(unsigned long *)(a0 + 0x20);
      v22 = ~v20;
      v11 = (long *)(*(long *)(a0 + 0x18) + 0x40);
      v18 = *(long *)(a0 + 8);
      v21 = v16 - v18;
      v10 = *(long **)(a0 + 0x10);
      if (v11 == v10) {
label_cd6f:
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
    }
    v22 = (long)v11 + v20 & v22;
    v17 = v13[v12];
    if (v22 - v18 <= v21)
      v16 = v22;
    *(unsigned long *)(a0 + 0x18) = v16;
    v18 = v7[5];
    *(unsigned long *)(a0 + 0x10) = v16;
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
    v12 -= 1;
    if (!v12) goto label_ca25;
    v17 = v13[v12];
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
      if (v13[v12 + 1]) { // branch-flip
        if (v13[v12 + 1] != 1) {
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
    else if (v13[v12 + 1]) { // branch-flip
      if (v13[v12 + 1] != 1) {
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
    if (v13[v12 + -1]) // branch-flip
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
  char v10; // al
  unsigned int v11;
  unsigned long v12;
  long *v13;
  long *v14;
  long v15; // rax
  long v16;
  long v17;
  unsigned long *v18;
  unsigned long v19;
  unsigned long *v2;
  long v20;
  long *v21;
  unsigned long v22;
  unsigned long v23; // rcx
  long *v24;
  long *v25; // rdx
  long *v26; // rdx
  long *v27; // rdx
  long *v28; // rdx
  long *v29;
  long v3;
  unsigned long v30 [33];
  long *v31; // stack - 0x950
  unsigned long v32 [256];
  unsigned int v33;
  unsigned int v34;
  long v35;
  long *v36;
  unsigned long *v37;
  unsigned long *v38;
  long *v39;
  unsigned long v4;
  long *v40;
  unsigned int v41; // r10d
  long **v42;
  unsigned long *v43;
  unsigned char v44;
  unsigned long v5;
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  v3 = a0[0x130];
  v43 = v30;
  if (!v3)
    v43 = &a0[0xe];
  v16 = a0[0xb];
  if (v16 == 1) {
    v42 = &v31;
    v24 = (long *)a0[0xc];
    v39 = v24;
    v31 = v24;
    for (; v24; v24 = (long *)v24[3]) {
      v40 = (long *)v24[1];
      if (v40) {
        v21 = (long *)*v40;
        if (v21) {
          if (*v21)
            sub_b420(*v21,v42);
          if (v21[1])
            sub_b420(v21[1],v42);
          v21 = (long *)v21[2];
          v31[3] = (long)v21;
          v31 = v21;
        }
        v21 = (long *)v40[1];
        v29 = v31;
        if (v21) {
          if (*v21)
            sub_b420(*v21,v42);
          if (v21[1])
            sub_b420(v21[1],v42);
          v29 = (long *)v21[2];
          v31[3] = (long)v29;
        }
        v31 = (long *)v40[2];
        v29[3] = (long)v31;
      }
    }
    v12 = a0[4];
    v22 = a0[0xd];
    if (v22 <= v12 - a0[3]) { // branch-flip
      v35 = a0[2];
      v17 = a0[3] + v22;
      if (v17 == v35) {
label_e3d2:
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      }
    }
    else {
      _obstack_newchunk(a0,v22);
      v35 = a0[2];
      v12 = a0[4];
      v39 = (long *)a0[0xc];
      v17 = a0[3] + v22;
      v22 = a0[0xd];
      if (v17 == v35) goto label_e3d2;
    }
    a0[0x12e] = v35;
    v19 = v17 + a0[6] & ~a0[6];
    if (v19 - a0[1] <= v12 - a0[1])
      v12 = v19;
    a0[3] = v12;
    a0[2] = v12;
    if (1 <= (long)v22) {
      v17 = 0;
      while( true ) {
        *(char *)(v35 + v17) = *(char *)(v39[1] + 0x18);
        v17 += 1;
        v39 = (long *)v39[3];
        if ((long)a0[0xd] <= v17) break;
        v35 = a0[0x12e];
      }
    }
    v18 = (unsigned long *)sub_c880(a0[0x130]);
    v4 = a0[0xd];
    v5 = a0[0x12e];
    v18[0x133] = sub_b3f0;
    sub_c9d0(v18,v5,v4);
    _obstack_free(a0,0);
    v35 = 0x134;
    v37 = v18;
    v38 = a0;
    while (v35) {
      v2 = &v38[1];
      v1 = &v37[1];
      *v38 = *v37;
      v35 -= 1;
      v37 = v1;
      v38 = v2;
    }
    free(v18);
    v12 = a0[0xd];
    if (0x100 <= (long)v12)
      v12 = 0xff;
    v12 &= 0xff;
    v8._8_8_ = 0;
    v8._0_8_ = v12;
    *v43 = SUB168(v8._0_16_ * ZEXT816(0x101010101010101),0);
    v35 = v12 * 0x101010101010101 + SUB168(v8._0_16_ * ZEXT816(0x101010101010101),8);
    v43[1] = v35;
    v43[3] = v35;
    v43[2] = v12 * 0x101010101010101;
    v43[5] = v35;
    v43[7] = v35;
    v43[9] = v35;
    v43[4] = v12 * 0x101010101010101;
    v43[0xb] = v35;
    v43[0xd] = v35;
    v43[0xf] = v35;
    v43[6] = v12 * 0x101010101010101;
    v43[0x11] = v35;
    v43[0x13] = v35;
    v43[0x15] = v35;
    v43[8] = v12 * 0x101010101010101;
    v43[0x17] = v35;
    v43[0x19] = v35;
    v43[0x1b] = v35;
    v43[10] = v12 * 0x101010101010101;
    v43[0x1d] = v35;
    v43[0xc] = v12 * 0x101010101010101;
    v43[0xe] = v12 * 0x101010101010101;
    v43[0x10] = v12 * 0x101010101010101;
    v43[0x12] = v12 * 0x101010101010101;
    v43[0x14] = v12 * 0x101010101010101;
    v43[0x16] = v12 * 0x101010101010101;
    v43[0x18] = v12 * 0x101010101010101;
    v43[0x1a] = v12 * 0x101010101010101;
    v43[0x1c] = v12 * 0x101010101010101;
    v9._8_8_ = 0;
    v9._0_8_ = v12;
    v24 = SUB168(v9._0_16_ * ZEXT816(0x101010101010101),8);
    v43[0x1e] = SUB168(v9._0_16_ * ZEXT816(0x101010101010101),0);
    v43[0x1f] = v35;
    v39 = (long *)a0[0xc];
    if (v39) goto label_d084;
    v35 = 0;
    v31 = v39;
label_e1bb:
    for (v35 = *(long *)(v35 + 0x18); v35; v35 = *(long *)(v35 + 0x18)) {
      v17 = *(long *)(*(long *)(v35 + 0x10) + 0x38);
      v20 = *(long *)(v35 + 0x38);
      if (v17 < *(long *)(v35 + 0x38)) {
        *(long *)(v35 + 0x38) = v17;
        v20 = v17;
      }
      if (v20 < *(long *)(v35 + 0x30))
        *(long *)(v35 + 0x30) = v20;
    }
  }
  else {
    v12 = a0[0xd];
    if (0x100 <= (long)v12)
      v12 = 0xff;
    v12 &= 0xff;
    v6._8_8_ = 0;
    v6._0_8_ = v12;
    *v43 = SUB168(v6._0_16_ * ZEXT816(0x101010101010101),0);
    v35 = v12 * 0x101010101010101 + SUB168(v6._0_16_ * ZEXT816(0x101010101010101),8);
    v43[1] = v35;
    v43[3] = v35;
    v43[2] = v12 * 0x101010101010101;
    v43[5] = v35;
    v43[7] = v35;
    v43[9] = v35;
    v43[4] = v12 * 0x101010101010101;
    v43[0xb] = v35;
    v43[0xd] = v35;
    v43[0xf] = v35;
    v43[6] = v12 * 0x101010101010101;
    v43[0x11] = v35;
    v43[0x13] = v35;
    v43[0x15] = v35;
    v43[8] = v12 * 0x101010101010101;
    v43[0x17] = v35;
    v43[0x19] = v35;
    v43[0x1b] = v35;
    v43[10] = v12 * 0x101010101010101;
    v43[0x1d] = v35;
    v43[0xc] = v12 * 0x101010101010101;
    v43[0xe] = v12 * 0x101010101010101;
    v43[0x10] = v12 * 0x101010101010101;
    v43[0x12] = v12 * 0x101010101010101;
    v43[0x14] = v12 * 0x101010101010101;
    v43[0x16] = v12 * 0x101010101010101;
    v43[0x18] = v12 * 0x101010101010101;
    v43[0x1a] = v12 * 0x101010101010101;
    v43[0x1c] = v12 * 0x101010101010101;
    v7._8_8_ = 0;
    v7._0_8_ = v12;
    v24 = SUB168(v7._0_16_ * ZEXT816(0x101010101010101),8);
    v43[0x1e] = SUB168(v7._0_16_ * ZEXT816(0x101010101010101),0);
    v43[0x1f] = v35;
    v39 = (long *)a0[0xc];
    v31 = v39;
    if (v39) {
label_d084:
      v31 = v39;
label_dc63:
      do {
        v37 = (unsigned long *)v39[1];
        if (v37) {
          v40 = (long *)*v37;
          if (v40) { // branch-flip
            v21 = (long *)*v40;
            if (v21) {
              v29 = (long *)*v21;
              if (v29) {
                v13 = (long *)*v29;
                if (v13) {
                  if (*v13) {
                    sub_b420(*v13,&v31,v24);
                    v24 = v25;
                  }
                  if (v13[1])
                    sub_b420(v13[1],&v31,v24);
                  v24 = (long *)v13[2];
                  v31[3] = (long)v24;
                  v31 = v24;
                }
                v24 = (long *)v29[1];
                v13 = v31;
                if (v24) {
                  if (*v24)
                    sub_b420(*v24,&v31);
                  if (v24[1])
                    sub_b420(v24[1],&v31);
                  v13 = (long *)v24[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v29[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[2];
              v31[3] = (long)v24;
              v31 = v24;
            }
            v21 = (long *)v40[1];
            v24 = v31;
            if (v21) {
              v24 = (long *)*v21;
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[2];
              v31[3] = (long)v24;
            }
            v21 = (long *)v40[2];
            v29 = (long *)v37[1];
            v24[3] = (long)v21;
            v31 = v21;
            if (v29) { // branch-flip
label_d327:
              v21 = (long *)*v29;
              if (v21) {
                v13 = (long *)*v21;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14) {
                      sub_b420(*v14,&v31,v24);
                      v24 = v26;
                    }
                    if (v14[1])
                      sub_b420(v14[1],&v31,v24);
                    v24 = (long *)v14[2];
                    v31[3] = (long)v24;
                    v31 = v24;
                  }
                  v24 = (long *)v13[1];
                  v14 = v31;
                  if (v24) {
                    if (*v24)
                      sub_b420(*v24,&v31);
                    if (v24[1])
                      sub_b420(v24[1],&v31);
                    v14 = (long *)v24[2];
                    v31[3] = (long)v14;
                  }
                  v31 = (long *)v13[2];
                  v14[3] = (long)v31;
                }
                v24 = (long *)v21[1];
                if (v24) {
                  v13 = (long *)*v24;
                  if (v13) {
                    if (*v13)
                      sub_b420(*v13,&v31);
                    if (v13[1])
                      sub_b420(v13[1],&v31);
                    v13 = (long *)v13[2];
                    v31[3] = (long)v13;
                    v31 = v13;
                  }
                  v13 = (long *)v24[1];
                  v14 = v31;
                  if (v13) {
                    if (*v13)
                      sub_b420(*v13,&v31);
                    if (v13[1])
                      sub_b420(v13[1],&v31);
                    v14 = (long *)v13[2];
                    v31[3] = (long)v14;
                  }
                  v31 = (long *)v24[2];
                  v14[3] = (long)v31;
                }
                v24 = (long *)v21[2];
                v31[3] = (long)v24;
                v31 = v24;
              }
              v24 = (long *)v29[1];
              if (v24) {
                v13 = (long *)*v24;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v14 = (long *)v13[1];
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v13 = (long *)v13[2];
                  v31[3] = (long)v13;
                  v31 = v13;
                }
                v13 = (long *)v24[1];
                v14 = v31;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v14 = (long *)v13[1];
                  v36 = v31;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v36 = (long *)v14[2];
                    v31[3] = (long)v36;
                  }
                  v14 = (long *)v13[2];
                  v36[3] = (long)v14;
                }
                v31 = (long *)v24[2];
                v14[3] = (long)v31;
              }
              v17 = v29[2];
              v35 = v39[5];
              v31[3] = v17;
              v31 = (long *)v37[2];
              *(long **)(v17 + 0x18) = v31;
              if (v40) goto label_d613;
            }
            else {
              v31 = (long *)v37[2];
              v35 = v39[5];
              v21[3] = (long)v31;
label_d613:
              v24 = (long *)*v40;
              v44 = (unsigned char)v35;
              if (v24) {
                v21 = (long *)*v24;
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                v21 = (long *)v24[1];
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              v24 = (long *)v40[1];
              if (v24) {
                v21 = (long *)*v24;
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                v21 = (long *)v24[1];
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              v29 = (long *)v37[1];
              if (!v29) goto label_dc21;
              v21 = (long *)*v29;
            }
            v44 = (unsigned char)v35;
            if (v21) {
              v24 = (long *)*v21;
              if (v24) {
                v38 = (unsigned long *)*v24;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v24[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v38 = (unsigned long *)*v24;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v24[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
            }
            v24 = (long *)v29[1];
            if (v24) {
              v40 = (long *)*v24;
              if (v40) {
                v38 = (unsigned long *)*v40;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v40[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              }
              v40 = (long *)v24[1];
              if (v40) {
                v38 = (unsigned long *)*v40;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v40[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
            }
            if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v29[3] + (long)v43))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v29[3] + (long)v43) = v44;
          }
          else {
            v29 = (long *)v37[1];
            if (v29) goto label_d327;
            v24 = (long *)v37[2];
            v35 = v39[5];
            v31[3] = (long)v24;
            v31 = v24;
          }
label_dc21:
          if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v37[3] + (long)v43))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v37[3] + (long)v43) = (unsigned char)v35;
          v37 = (unsigned long *)v39[1];
        }
        v35 = a0[0xc];
        sub_ab10(v37,v39[4],v35,v16 == 1);
        if (v16 == 1) {
          v17 = a0[0xd];
          v20 = v39[4];
          v39[6] = v17;
          v39[7] = v17;
          if (v20) {
            v17 = *v39;
            do {
              v10 = sub_abd0(*(unsigned long *)(v20 + 8),v37);
              if ((!v10) && (v15 = v39[5] - *(long *)(v20 + 0x28), v15 < *(long *)(v20 + 0x30)))
                *(long *)(v20 + 0x30) = v15;
              if ((v17) && (v15 = v39[5] - *(long *)(v20 + 0x28), v15 < *(long *)(v20 + 0x38)))
                *(long *)(v20 + 0x38) = v15;
              v20 = *(long *)(v20 + 0x20);
            } while (v20);
            v39 = (long *)v39[3];
            v24 = v28;
            if (!v39) break;
            goto label_dc63;
          }
        }
        v39 = (long *)v39[3];
        v24 = v27;
      } while (v39);
      if (v16 == 1) goto label_e1bb;
    }
  }
  v12 = 0xffffffff;
  v37 = v32;
  if (!v3)
    v37 = &a0[0x2e];
  v41 = 0xffffffff;
  *v37 = 0;
  v37[0xff] = 0;
  v22 = (unsigned long)(((int)v37 - (int)(unsigned long *)((unsigned long)&v37[1] & 0xfffffffffffffff8)) + 0x800U >> 3);
  v38 = (unsigned long *)((unsigned long)&v37[1] & 0xfffffffffffffff8);
  while (v22) {
    v18 = &v38[1];
    *v38 = 0;
    v22 -= 1;
    v38 = v18;
  }
  sub_ba10(*(unsigned long *)(a0[0xc] + 8),v37);
  v22 = 0;
  v33 = 0xfffffffe;
  do {
    while( true ) {
      v11 = (unsigned int)v22;
      v19 = v22 & 0xffffffff;
      if (v3) { // branch-flip
        v23 = (unsigned long)*(unsigned char *)(v3 + v22);
        v35 = v37[v23];
        a0[v22 + 0x2e] = v35;
      }
      else {
        v35 = a0[v22 + 0x2e];
        v23 = v22 & 0xffffffff;
      }
      v34 = v33;
      if (v35) break;
label_ddd0:
      v11 = (unsigned int)v12;
      v22 += 1;
      v33 = v34;
      if (v22 == 0x100) goto label_de21;
    }
    v34 = (unsigned int)v23;
    if (v33 != 0xfffffffe) {
      if (v34 != v33) { // branch-flip
        if ((v34 != v11) || (v33 != (unsigned int)v12))
          v34 = 0xffffffff;
        else {
          v12 = v19;
          v34 = v33;
        }
      }
      else {
        if (v33 != (unsigned int)v12)
          v11 = v41;
        v12 = (unsigned long)v11;
        v34 = v33;
      }
      goto label_ddd0;
    }
    v22 += 1;
    v12 = v19;
    v33 = v34;
  } while (v22 != 0x100);
label_de21:
  *(unsigned int *)&a0[0x131] = v34;
  *(unsigned int *)((long)a0 + 0x98c) = v11;
  if (v16 != 1) goto label_de3a;
  v12 = a0[4];
  v22 = a0[0xd];
  if (v22 <= v12 - a0[3]) { // branch-flip
    v16 = a0[2];
    v35 = a0[3] + v22;
    v19 = v22;
    if (v35 == v16) {
label_e39e:
      *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
    }
  }
  else {
    _obstack_newchunk(a0);
    v16 = a0[2];
    v12 = a0[4];
    v19 = a0[0xd];
    v35 = a0[3] + v22;
    if (v35 == v16) goto label_e39e;
  }
  a0[0x12e] = v16;
  v22 = v35 + a0[6] & ~a0[6];
  if (v22 - a0[1] <= v12 - a0[1])
    v12 = v22;
  a0[3] = v12;
  a0[2] = v12;
  v35 = a0[0xc];
  if (1 <= (long)v19) {
    while( true ) {
      v19 -= 1;
      *(char *)(v16 + v19) = *(char *)(*(long *)(v35 + 8) + 0x18);
      v35 = *(long *)(v35 + 0x18);
      if (!v19) break;
      v16 = a0[0x12e];
    }
    v16 = a0[0xd];
    if (2 <= v16) {
      v22 = a0[4];
      v35 = a0[3];
      v12 = v16 * 8 - 8;
      if (v22 - v35 < v12) {
        _obstack_newchunk(a0,v12);
        v35 = a0[3];
        v22 = a0[4];
        v16 = a0[0xd];
      }
      v17 = a0[2];
      if (v35 + v12 == v17)
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      a0[0x12f] = v17;
      v12 = v35 + v12 + a0[6] & ~a0[6];
      if (v12 - a0[1] <= v22 - a0[1])
        v22 = v12;
      a0[3] = v22;
      a0[2] = v22;
      v35 = *(long *)(a0[0xc] + 0x18);
      if (2 <= v16) {
        v20 = 0;
        do {
          v37 = (unsigned long *)(v35 + 0x30);
          v35 = *(long *)(v35 + 0x18);
          *(unsigned long *)(v17 + v20 * 8) = *v37;
          v16 = a0[0xd];
          v20 += 1;
        } while (v20 < v16 + -1);
      }
      v44 = *(unsigned char *)(a0[0x12e] + -2 + v16);
      if (v3)
        v44 = *(unsigned char *)(v3 + (unsigned long)v44);
      *(unsigned char *)&a0[0x132] = v44;
    }
  }
label_de3a:
  v16 = 0;
  if (v3) {
    do {
      *(char *)((long)a0 + v16 + 0x70) = *(char *)((long)v43 + (unsigned long)*(unsigned char *)(v3 + v16));
      v16 += 1;
    } while (v16 != 0x100);
    return;
  }
}


// Function: sub_e3e0 @ 0xe3e0
void sub_e3e0(long a0)
{
  (**(void **)(a0 + 0x998))(); // jump-as-call
}


// Function: sub_e420 @ 0xe420
long sub_e420(long a0,long a1,char a2)
{
  unsigned long v1; // rax
  long v2; // rax
  int v3; // stack - 0x4c
  unsigned long v4; // stack - 0x48
  long v5; // r14
  
  v5 = 0;
  v4 = 0;
  do {
    if (a1 - a0 <= v5)
      return v5;
    v1 = (unsigned long)*(unsigned char *)(a0 + v5);
    if (*(char *)(v1 + 0x2b9c0)) // branch-flip
      v5 += 1;
    else {
      if (*(char *)(v1 + 0x2b183) != '\xfe')
        return v5;
      v3 = 0;
      v2 = sub_1acc0(&v3,(unsigned char *)(a0 + v5),(a1 - a0) - v5,&v4);
      if ((v3 != 0x5f) && (!iswalnum()))
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
  int *v1; // rbx
  char *v2; // rbp
  bool v3; // al
  
  v2 = (char *)0x2b9c0;
  v1 = (int *)0x2b284;
  do {
    v3 = 1;
    if (*v1 != 0x5f)
      v3 = iswalnum() != 0;
    v1 = &v1[1];
    *v2 = v3;
    v2 = &v2[1];
  } while (v1 != (int *)0x2b684);
}


// Function: sub_e5c0 @ 0xe5c0
void sub_e5c0(char a0)
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
  unsigned int v1; // eax
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
void sub_e810(unsigned long a0,unsigned long a1)
{
  sub_e420(a0,a1,1); // tail-call
}


// Function: sub_e830 @ 0xe830
unsigned long sub_e830(long a0,long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x28
  
  if (a0 == a1)
    return 0;
  if ((dat_2b180) && (!(~(*(unsigned char *)(a1 + -1) >> 7) & dat_2b182))) {
    v2 = a0;
    v1 = sub_e420((a1 + -1) - sub_e640(&v2,0,a1 + -1,a2),a2,0); // tail-call
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
  unsigned long *v1; // rax
  unsigned long *v10; // rax
  int v11; // stack - 0x154
  char v12 [8];
  char v13 [264];
  unsigned int v14; // r13d
  bool v15; // cf
  unsigned long v16; // stack - 0x168
  char *v17; // stack - 0x160
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long *v7; // rax
  unsigned long *v8; // rax
  unsigned long *v9; // rax
  
  v15 = dat_2b687 == '\0';
  v1 = (unsigned long *)sub_1ef30(0x38);
  v14 = (-(unsigned int)v15 & 0xfffffff8) + 0x18;
  v2 = pcre2_general_context_create_8(sub_e900,sub_e8f0,0);
  *v1 = v2;
  v3 = pcre2_compile_context_create_8(v2);
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
    pcre2_set_compile_extra_options_8(v3,8);
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
  pcre2_set_character_tables_8(v3,pcre2_maketables_8(v2));
  v4 = pcre2_compile_8(a0,v16,v14,&v11,v12,v3);
  v1[1] = v4;
  if (!v4) {
    pcre2_get_error_message_8(v11,v13,0x100);
    error(2,0,"%s",v13);
    return v9;
  }
  free(v17);
  pcre2_compile_context_free_8(v3);
  v1[2] = 0;
  v2 = pcre2_match_data_create_from_pattern_8(v1[1],v2);
  v1[3] = v2;
  v5 = pcre2_jit_compile_8(v1[1],1);
  v11 = (int)v5;
  if ((v11 + 0x30U <= 0x30) && (!(0xfffefffffffffff6U >> ((unsigned long)(v11 + 0x30U) & 0x3f) & 1))) {
    v1[4] = 0;
    v1[5] = 0x8000;
    *(unsigned int *)&v1[6] = sub_e920(v1,"",0,0,1);
    *(unsigned int *)((long)v1 + 0x34) = sub_e920(v1,"",0,0,0);
    return v1;
  }
  error(2,0,dcgettext(NULL,"JIT internal error: %d",5),v5 & 0xffffffff);
  return v7;
}


// Function: sub_eda0 @ 0xeda0
unsigned char * sub_eda0(long a0,unsigned char *a1,long a2,long *a3,unsigned char *a4)
{
  int v1;
  unsigned char *v10; // stack - 0x68
  long *v2;
  unsigned long v3;
  unsigned char *v4; // rax
  long v5; // rcx
  unsigned char *v6;
  unsigned int v7; // ebp
  unsigned char *v8;
  unsigned char *v9;
  
  v6 = a1;
  if (a4)
    v6 = a4;
  v7 = (unsigned int)(v6[-1] == dat_2b684);
  v4 = (long *)pcre2_get_ovector_pointer_8(*(unsigned long *)(a0 + 0x18));
  v2 = (long *)v4;
  v10 = a1;
  while( true ) {
    v4 = rawmemchr(v6,(int)(char)dat_2b684);
    v8 = v4;
    v4 = (unsigned long)(unsigned long)*v6;
    if (*(char *)((long)v4 + 0x2b183) != '\xff') { // branch-flip
      v5 = (long)v6 - (long)v10;
      v9 = v10;
    }
    else {
      do {
        v4 = (unsigned long)(unsigned long)v6[1];
        v6 = &v6[1];
      } while (*(char *)((long)v4 + 0x2b183) == '\xff');
      v5 = 0;
      v7 = 0;
      v9 = v6;
    }
    if (v8 != v6) // branch-flip
      v1 = sub_e920(a0,v9,(long)v8 - (long)v9,v5,v7 ^ 1);
    else {
      v1 = *(int *)(a0 + 0x30 + (unsigned long)v7 * 4);
      v2[1] = v5;
      *v2 = v5;
    }
    if (v1 != -1) break;
    v6 = &v8[1];
    v4 = &a1[a2];
    v7 = 1;
    if (v4 <= v6) {
      v4 = (unsigned char *)0xffffffffffffffff;
      return v4;
    }
    v10 = v6;
  }
  if (1 <= v1) {
    v8 = &v8[1];
    if (a4) {
      v10 = &v9[*v2];
      v8 = &v9[v2[1]];
    }
    v4 = (long)((long)v10 - (long)a1);
    *a3 = (long)v8 - (long)v10;
    return v4;
  }
  if ((v1 < -0x2d) && (-0x3f <= v1)) {
    switch(v1) {
      case 0xffffffcb:
label_f044:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s nested backtracking limit",5);
        error(2,0,(char *)v4,v3);
        goto label_f073;
      case 0xffffffc1:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s heap limit",5);
        error(2,0,(char *)v4,v3);
      case 0xffffffcc:
label_f015:
        v4 = (unsigned long)sub_90f0(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: PCRE detected recurse loop",5);
        error(2,0,(char *)v4,v3);
        goto label_f044;
      case 0xffffffd0:
label_efe6:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: memory exhausted",5);
        error(2,0,(char *)v4,v3);
        goto label_f015;
      case 0xffffffd1:
label_efb7:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s backtracking limit",5);
        error(2,0,(char *)v4,v3);
        goto label_efe6;
      case 0xffffffd2:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exhausted PCRE JIT stack",5);
        error(2,0,(char *)v4,v3);
        goto label_efb7;
      
    }
  }
  v4 = (unsigned long)sub_90f0();
  v3 = v4;
  v4 = (char *)dcgettext(NULL,"%s: internal PCRE error: %d",5);
  error(2,0,(char *)v4,v3,v1);
  return v4;
}

