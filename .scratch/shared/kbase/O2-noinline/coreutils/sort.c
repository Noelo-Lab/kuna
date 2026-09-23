// Function: sub_3f80 @ 0x3f80
unsigned long sub_3f80(int a0,char **a1)
{
  unsigned long *v1;
  long v10;
  long *v11; // rax
  unsigned long v12;
  unsigned long *v13; // rax
  unsigned long *v14;
  unsigned long *v15; // rax
  char *v16; // rax
  unsigned long v17;
  unsigned long v18; // rax
  unsigned long v19; // rax
  bool v2;
  unsigned long v20; // rax
  unsigned long v21; // rax
  unsigned long v22; // rax
  unsigned long v23; // rax
  unsigned long v24; // rax
  unsigned long v25; // rax
  unsigned long v26; // rax
  unsigned long v27; // rax
  unsigned long v28; // rax
  unsigned long v29; // rdx
  bool v3;
  long v30; // rdx
  char *v31;
  unsigned char v32;
  void *v33; // stack - 0xe8
  unsigned long v34 [4]; // stack - 0x258
  int v35; // stack - 0x208
  char v36; // stack - 0x42
  char v37 [80];
  char v38; // r12b
  char v39;
  bool v4;
  int *v40;
  long v41;
  long v42;
  char *v43; // stack - 0x2f8
  char *v44; // stack - 0x2f0
  char *v45; // stack - 0x2e0
  unsigned long v46; // stack - 0x2d8
  int v47; // stack - 0x2d0
  unsigned long v48; // stack - 0x238
  unsigned long v49; // stack - 0x230
  int v5;
  char v50; // stack - 0x228
  char v51; // stack - 0x227
  char v52; // stack - 0x226
  unsigned char v53; // stack - 0x225
  char v54; // stack - 0x224
  char v55; // stack - 0x223
  char v56; // stack - 0x222
  char v57; // stack - 0x221
  char v58; // stack - 0x220
  unsigned int v59; // stack - 0x204
  int v6; // eax
  unsigned long *v60; // stack - 0x200
  unsigned long v61; // stack - 0xe0
  unsigned long v62; // stack - 0xd8
  unsigned long v63; // stack - 0xd0
  unsigned long v64; // stack - 0xc8
  unsigned long v65; // stack - 0xc0
  unsigned long v66; // stack - 0xb8
  unsigned long v67; // stack - 0xb0
  unsigned long v68; // stack - 0xa8
  unsigned long v69; // stack - 0xa0
  char *v7;
  unsigned long v70; // stack - 0x98
  unsigned long v71; // stack - 0x90
  unsigned long v72; // stack - 0x88
  unsigned long v73; // stack - 0x80
  unsigned long v74; // stack - 0x78
  unsigned long v75; // stack - 0x70
  unsigned long v76; // stack - 0x68
  unsigned int v77; // stack - 0x60
  char v78; // stack - 0x41
  long v79; // stack - 0x40
  char *v8; // rax
  unsigned long *v9;
  
  v79 = *(long *)(v42 + 0x28);
  v7 = getenv("POSIXLY_CORRECT");
  v5 = sub_ee70();
  v3 = 0x2b8 < (unsigned int)(v5 - 0x30db0U);
  sub_ef00(*a1);
  v8 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8f60(3);
  dat_1e861 = sub_cfb0();
  dat_1e860 = sub_cfb0(2);
  v9 = (unsigned long *)localeconv();
  dat_1e868 = *(char *)*v9;
  if ((!dat_1e868) || (((char *)*v9)[1]))
    dat_1e868 = '.';
  v39 = *(char *)v9[1];
  dat_1e864 = (int)v39;
  if (v39) { // branch-flip
    if (((char *)v9[1])[1]) {
      dat_1e862 = 1;
      dat_1e864 = 0x80; // crossjump-dupe
    }
  }
  else {
    dat_1e864 = 0x80;
  }
  dat_1e418 = '\0';
  sub_5b80();
  v40 = (int *)0x15324;
  sigemptyset((void *)0x1e380);
  v5 = 0xe;
  while( true ) {
    sigaction(v5,NULL,&v33);
    if (v33 != (void *)0x1)
      sigaddset((void *)0x1e380,v5);
    if (v40 == (int *)0x1534c) break;
    v5 = *v40;
    v40 = &v40[1];
  }
  v77 = 0;
  v33 = sub_5b40;
  v5 = 0xe;
  v61 = dat_1e380;
  v62 = dat_1e388;
  v63 = dat_1e390;
  v64 = dat_1e398;
  v40 = (int *)0x15324;
  v65 = dat_1e3a0;
  v66 = dat_1e3a8;
  v67 = dat_1e3b0;
  v68 = dat_1e3b8;
  v69 = dat_1e3c0;
  v70 = dat_1e3c8;
  v71 = dat_1e3d0;
  v72 = dat_1e3d8;
  v73 = dat_1e3e0;
  v74 = dat_1e3e8;
  v75 = dat_1e3f0;
  v76 = dat_1e3f8;
  while( true ) {
    if (sigismember(0x1e380,v5))
      sigaction(v5,&v33,NULL);
    if (v40 == (int *)0x1534c) break;
    v5 = *v40;
    v40 = &v40[1];
  }
  v41 = 0;
  v5 = 0;
  signal(0x11,0);
  sub_14980(sub_5ad0);
  sub_5cf0(v34,8);
  v34[0] = 0xffffffffffffffff;
  v9 = (unsigned long *)sub_12fd0((long)a0);
  v43 = NULL;
  v47 = 0;
  v44 = NULL;
  v46 = 0;
  v45 = NULL;
  v4 = 0;
label_42ce:
  v35 = -1;
  if (v47 == -1) {
label_42de:
    v10 = (long)optind;
    goto label_42b0;
  }
  if ((!v41) || (!v7)) {
label_4310:
    v40 = &v35;
    v47 = getopt_long(a0,a1,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",(void *)0x1d5a0,v40);
    v31 = optarg;
    if (v47 == -1) goto label_42de;
    if (0x87 < v47) {
      sub_be50(2); // return-dupe, no-return
    }
    if (v47 <= 0) {
      if (v47 == -0x83) {
        sub_12d40(stdout,"sort","GNU coreutils",dat_1e120,"Mike Haertel","Paul Eggert",0,CONCAT44(dat_4,v47));
        exit(0); // no-return
      }
      if (v47 == -0x82)
        sub_be50(0); // no-return
      sub_be50(2);
    }
    switch(v47) {
      default:
        goto label_4db0;
      case 1:
        if (*optarg == '+') {
          if (optind != a0) { // branch-flip
            v2 = 0;
            if (*a1[optind] == '-')
              v2 = (unsigned int)((int)a1[optind][1] - 0x30U) <= 9;
          }
          else {
            v2 = 0;
          }
          v3 = (bool)(v3 | v7 == NULL & v2);
          if (v3) {
            v11 = (long *)sub_5cf0(v37);
            v31 = (char *)sub_5d20(&v31[1],v11,0);
            if (v31) { // branch-flip
              if (*v31 != '.') { // branch-flip
                if ((!*v11) && (!v11[1])) {
label_4bda:
                  *v11 = -1;
                  goto label_4be1;
                }
              }
              else {
                v31 = (char *)sub_5d20(&v31[1],&v11[1],0);
                if ((!*v11) && (!v11[1])) goto label_4bda;
label_4be1:
                if (!v31) goto label_4bea;
              }
              if (!*(char *)sub_57c0(v31,v11,0)) {
                if (v2) {
                  v10 = (long)optind;
                  optind += 1;
                  v31 = a1[v10];
                  v16 = (char *)sub_5d20(&v31[1],&v11[2],"invalid number after \'-\'");
                  if (!v16)
                    __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                  if (*v16 == '.')
                    v16 = (char *)sub_5d20(&v16[1],&v11[3],"invalid number after \'.\'");
                  if ((!v11[3]) && (v11[2]))
                    v11[2] = v11[2] + -1;
                  if (*(char *)sub_57c0(v16,v11,1))
                    sub_5a10(v31,"stray character in field spec"); // no-return
                }
                *(char *)((long)v11 + 0x39) = 1;
                sub_60c0(v11);
                break;
              }
            }
            else if ((!*v11) && (!v11[1]))
              *v11 = -1;
label_4bea:
            v31 = optarg;
          }
        }
        v9[v41] = v31;
        v41 += 1;
        break;
      case 0x43:
      case 99:
label_4420:
        if (((char)v5) && (v5 != v47))
          sub_5940("cC"); // no-return
        v5 = (int)(char)v47;
        break;
      case 0x4d:
      case 0x52:
      case 0x56:
      case 0x62:
      case 100:
      case 0x66:
      case 0x67:
      case 0x68:
      case 0x69:
      case 0x6e:
      case 0x72:
        goto label_43a5;
      case 0x53:
        sub_89b0(v35,optarg);
        break;
      case 0x54:
        sub_61e0(optarg);
        break;
      case 0x6b:
        v11 = (long *)sub_5cf0(v37);
        v31 = (char *)sub_5d20(optarg,v11,"invalid number at field start");
        v10 = *v11;
        *v11 = v10 + -1;
        if (!v10)
          sub_5a10(optarg,"field number is zero"); // return-dupe, no-return
        if (*v31 == '.') {
          v31 = (char *)sub_5d20(&v31[1],&v11[1],"invalid number after \'.\'");
          v10 = v11[1];
          v11[1] = v10 + -1;
          if (!v10)
            sub_5a10(optarg,"character offset is zero"); // no-return
        }
        if ((!*v11) && (!v11[1]))
          *v11 = -1;
        v31 = (char *)sub_57c0(v31,v11,0);
        if (*v31 != ',') { // branch-flip
          v11[2] = -1;
          v11[3] = 0;
        }
        else {
          v31 = (char *)sub_5d20(&v31[1],&v11[2],"invalid number after \',\'");
          v10 = v11[2];
          v11[2] = v10 + -1;
          if (!v10) {
            sub_5a10(optarg,"field number is zero");
          }
          if (*v31 == '.')
            v31 = (char *)sub_5d20(&v31[1],&v11[3],"invalid number after \'.\'");
          v31 = (char *)sub_57c0(v31,v11,1);
        }
        if (*v31)
          sub_5a10(optarg,"stray character in field spec"); // no-return
        sub_60c0(v11);
        break;
      case 0x6d:
        v4 = 1;
        break;
      case 0x6f:
        if ((v44) && (v6 = strcmp(v44,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple output files specified",5));
          return v27;
        }
        v44 = v31;
        break;
      case 0x73:
        dat_1e41a = 1;
        break;
      case 0x74:
        v6 = (int)*optarg;
        if (!*optarg) {
          error(2,0,dcgettext(NULL,"empty tab",5));
          return v24;
        }
        if (optarg[1]) {
          v6 = strcmp(optarg,"\\0");
          if (v6) {
            v17 = sub_11430(v31);
            error(2,0,dcgettext(NULL,"multi-character tab %s",5),v17);
            return v25;
          }
          v6 = 0;
        }
        if ((dat_1e028 != 0x80) && (dat_1e028 != v6)) {
          error(2,0,dcgettext(NULL,"incompatible tabs",5));
          return v26;
        }
        dat_1e028 = v6;
        break;
      case 0x75:
        dat_1e419 = 1;
        break;
      case 0x79:
        if (a1[(long)optind + -1] == optarg) {
          for (; (unsigned int)((int)*v31 - 0x30U) <= 9; v31 = &v31[1]) {
          }
          optind = optind + -1 + (unsigned int)(*v31 == '\0');
        }
        break;
      case 0x7a:
        dat_1e038 = 0;
        break;
      case 0x80:
        v47 = 99;
        if (optarg) {
          v10 = sub_c4b0("--check",optarg,0x1d580,"CCc",1,dat_1e128,1,v29);
          v47 = (int)"CCc"[v10];
        }
        goto label_4420;
      case 0x81:
        if ((dat_1e408) && (v6 = strcmp(dat_1e408,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple compress programs specified",5));
          return v23;
        }
        dat_1e408 = v31;
        break;
      case 0x82:
        dat_1e400 = '\x01';
        break;
      case 0x83:
        v43 = optarg;
        break;
      case 0x84:
        sub_8b60(v35,optarg);
        break;
      case 0x85:
        if ((v45) && (v6 = strcmp(v45,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple random sources specified",5));
          return v28;
        }
        v45 = v31;
        break;
      case 0x86:
        v10 = sub_c4b0("--sort",optarg,0x1d540,"ghMnRVCCc",1,dat_1e128,1,v40);
        v47 = (int)"ghMnRVCCc"[v10];
label_43a5:
        v36 = (char)v47;
        v78 = 0;
        sub_57c0(&v36,v34,2);
        break;
      case 0x87:
        goto label_481b;
      
    }
    goto label_42ce;
  }
  v10 = (long)optind;
  if ((v3) && (!(char)v5)) {
    if (optind != a0) {
      v31 = a1[optind];
      if (((*v31 == '-') && (v31[1] == 'o')) && ((v31[2] || (optind + 1 != a0)))) goto label_4310;
      goto label_42b0;
    }
    if (!v43) {
      v5 = 0;
      goto label_4466;
    }
label_4d4b:
    v17 = sub_11170(4,*v9);
    error(0,0,dcgettext(NULL,"extra operand %s",5),v17);
    __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
    sub_be50(2);
  }
label_42b0:
  if ((int)v10 < a0) {
    optind = (int)v10 + 1;
    v9[v41] = a1[v10];
    v41 += 1;
    goto label_42ce;
  }
  if (!v43) goto label_4466;
  if (v41) goto label_4d4b;
  v17 = sub_6010(v43,"r");
  sub_11ee0(&v35);
  if (!sub_11f90(v17,&v35)) {
    v17 = sub_11170(4,v43);
    error(2,0,dcgettext(NULL,"cannot read file names from %s",5),v17);
    return v22;
  }
  sub_6040(v17);
  if (!CONCAT44(v59,v35)) {
    v17 = sub_11170(4,v43);
    error(2,0,dcgettext(NULL,"no input from %s",5),v17);
    return v21;
  }
  free(v9);
  v10 = CONCAT44(v59,v35);
  v9 = v60;
  if (v10) {
    v41 = 0;
    do {
      v7 = (char *)v60[v41];
      if (!strcmp(v7,"-")) {
        v17 = sub_11170(4,v7);
        error(2,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v17);
        return v18;
      }
      v41 += 1;
      if (!*v7) {
        v17 = sub_11270(0,3,v43);
        error(2,0,dcgettext(NULL,"%s:%lu: invalid zero-length file name",5),v17,v41);
        return v19;
      }
    } while (v10 != v41);
  }
label_4466:
  v39 = (char)v5;
  if (dat_1e410) { // branch-flip
    v32 = 0;
    v10 = dat_1e410;
    do {
      while ((sub_5680(v10) && (!*(char *)(v30 + 0x37)))) {
        *(unsigned long *)(v30 + 0x20) = v48;
        *(unsigned long *)(v30 + 0x28) = v49;
        *(char *)(v30 + 0x30) = v50;
        *(char *)(v30 + 0x31) = v51;
        *(char *)(v30 + 0x36) = v56;
        *(char *)(v30 + 0x32) = v52;
        *(char *)(v30 + 0x34) = v54;
        *(char *)(v30 + 0x35) = v55;
        *(char *)(v30 + 0x37) = v57;
        *(char *)(v30 + 0x38) = v58;
        *(unsigned char *)(v30 + 0x33) = v53;
        v10 = *(long *)(v30 + 0x40);
        v32 |= v53;
        if (!v10) goto label_452f;
      }
      v10 = *(long *)(v30 + 0x40);
      v32 |= *(unsigned char *)(v30 + 0x33);
    } while (v10);
label_452f:
    v38 = 0;
label_4532:
    sub_5970();
    if (dat_1e400) goto label_461e;
  }
  else {
    if (!sub_5680(v34)) {
      v38 = 1;
      sub_60c0(v34);
      v32 = v53;
      goto label_4532;
    }
    sub_5970();
    if (!dat_1e400) {
      dat_1e41b = v57;
      goto label_4560;
    }
    v32 = 0;
    v38 = 0;
label_461e:
    if (v39) {
      s_1e018[0] = v39; // return-dupe
      sub_5940(s_1e018); // no-return
    }
    if (v44) {
      v39 = 'o';
      s_1e018[0] = v39;
      sub_5940(s_1e018);
    }
    if ((!v8) || (!setlocale(3,"")))
      error(0,0,"%s",dcgettext(NULL,"failed to set locale",5));
    if (dat_1e861) { // branch-flip
      v17 = sub_11430(setlocale(3,NULL));
      error(0,0,dcgettext(NULL,"text ordering performed using %s sorting rules",5),v17);
    }
    else {
      error(0,0,"%s",dcgettext(NULL,"text ordering performed using simple byte comparison",5));
    }
    sub_7d30(v34,v38);
  }
  dat_1e41b = v57;
  if (v32)
    sub_6110(v45);
label_4560:
  if (!dat_1e428) {
    v7 = getenv("TMPDIR");
    if (!v7)
      v7 = "/tmp";
    sub_61e0(v7);
  }
  if (!v41) {
    v41 = 1;
    free(v9);
    v9 = (unsigned long *)sub_12eb0(8);
    *v9 = "-";
  }
  if (dat_1e438) {
    v12 = (unsigned long)dat_1e024 * 0x22;
    if ((unsigned long)dat_1e024 * 0x22 < dat_1e438)
      v12 = dat_1e438;
    dat_1e438 = v12;
  }
  if (!v39) {
    sub_6c00(v9,v41);
    sub_6ca0(v44);
    if (v4) {
      v13 = (unsigned long *)sub_13150(v41,0x10);
      v1 = &v9[v41];
      v14 = v13;
      do {
        v17 = *v9;
        v9 = &v9[1];
        v15 = &v14[2];
        *v14 = v17;
        v14 = v15;
      } while (v9 != v1);
      sub_b720(v13,0,v41,v44);
    }
    else {
      if (!v46) {
        v12 = sub_e9d0(2);
        v46 = 8;
        if (v12 <= 8)
          v46 = v12;
      }
      if (0x100000000000000 <= v46)
        v46 = 0xffffffffffffff;
      sub_bac0(v9,v41,v44,v46);
    }
    if ((dat_1e418) && (v5 = sub_c900(stdin), v5 == -1))
      sub_5e10(dcgettext(NULL,"close failed",5),"-"); // no-return
    if (v79 == *(long *)(v42 + 0x28))
      return 0;
    __stack_chk_fail(); // no-return
  }
  if (v41 != 1) {
    v17 = sub_11170(4,v9[1]);
    error(2,0,dcgettext(NULL,"extra operand %s not allowed with -%c",5),v17,(unsigned long)(unsigned int)(int)v39);
    return v20;
  }
  if (v44) {
    dat_1e010 = v39;
    sub_5940(0x1e010); // no-return
  }
  v17 = *v9;
  exit((sub_aac0(v17,(int)v39) ^ 1) & 0xff); // no-return
label_481b:
  v46 = sub_8900(v35,optarg);
  goto label_42ce;
}


// Function: sub_5400 @ 0x5400
unsigned long sub_5400(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}


// Function: sub_5420 @ 0x5420
unsigned int sub_5420(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}


// Function: sub_5430 @ 0x5430
int sub_5430(long *a0)
{
  char *v1;
  char *v2;
  int v3; // ecx
  char v4;
  char *v5;
  int v6;
  
  v2 = (char *)*a0;
  v4 = *v2;
  v3 = (int)v4;
  v5 = &v2[1];
  if (10 <= (unsigned int)(v3 - 0x30U)) // branch-flip
    v6 = 0;
  else {
    v6 = 0;
    do {
      if ((char)v6 < (char)v3)
        v6 = v3;
      v1 = &v2[1];
      v2 = &v2[2];
      if (*v1 != dat_1e864)
        v2 = v5;
      v4 = *v2;
      v3 = (int)v4;
      v5 = &v2[1];
    } while ((unsigned int)(v3 - 0x30U) <= 9);
    if (*v1 == dat_1e864) {
      *a0 = (long)&v2[-1];
      return v6;
    }
  }
  if (dat_1e868 != v4) // branch-flip
    v5 = &v5[-1];
  else {
    v4 = *v5;
    while ((unsigned int)((int)v4 - 0x30U) <= 9) {
      v5 = &v5[1];
      if ((char)v6 < v4)
        v6 = (int)v4;
      v4 = *v5;
    }
  }
  *a0 = (long)v5;
  return v6;
}


// Function: sub_54f0 @ 0x54f0
int sub_54f0(char *a0) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  int v3; // edx
  unsigned char *v4; // stack - 0x18
  char v5;
  
  v5 = *a0;
  v2 = (unsigned long)(v5 == '-');
  v4 = (unsigned char *)&a0[v2];
  if ((char)sub_5430(&v4) <= '0') {
    v1 = 0;
    return v1;
  }
  v2 = (unsigned long)*v4;
  v1 = (int)*(char *)(v2 + 0x15360);
  v3 = -v1;
  if (v5 == '-')
    v1 = v3;
  return v1;
}


// Function: sub_5570 @ 0x5570
unsigned int sub_5570(char *a0,unsigned long *a1)
{
  char *v1;
  unsigned long v10;
  unsigned long v11; // r8
  char *v12;
  long v13; // r10
  unsigned long v14; // r11
  unsigned char v2; // al
  unsigned char v3; // al
  unsigned int v4; // eax
  unsigned long v5; // rax
  char *v6;
  char v7; // dl
  long v8; // rdx
  unsigned long v9;
  
  while( true ) {
    v4 = sub_53e0((int)*a0);
    v11 = (unsigned long)v4;
    if (!*(char *)(v8 + (unsigned long)(v4 & 0xff))) break;
    a0 = &a0[1];
  }
  v9 = 0xc;
  v10 = 0;
  v13 = 0x1e440;
  do {
    v14 = v10 + v9 >> 1;
    v6 = *(char **)(v14 * 0x10 + 0x1e060);
    if (!*v6) {
label_5630:
      if (a1)
        *a1 = a0;
      return *(unsigned int *)(v14 * 0x10 + 0x1e068);
    }
    v5 = v11 & 0xffffffff;
    v12 = a0;
    while( true ) {
      v2 = sub_53e0((int)*(char *)(v13 + (v5 & 0xff)));
      v3 = sub_53e0((int)v7);
      if (v2 < v3) break;
      if (v3 < v2) {
        v10 = v14 + 1;
        goto label_561a;
      }
      v1 = &v6[1];
      v6 = &v6[1];
      a0 = &a0[1];
      if (!*v1) goto label_5630;
      v5 = sub_53e0((int)*a0);
    }
    v9 = v14;
label_561a:
    a0 = v12;
    if (v9 <= v10)
      return 0;
  } while( true );
}


// Function: sub_5660 @ 0x5660
unsigned long sub_5660(long a0)
{
  unsigned long v1; // rax
  
  v1 = *(unsigned long *)(a0 + 0x30) & 0xffff00ff0000;
  return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
}


// Function: sub_5680 @ 0x5680
unsigned long sub_5680(long a0) // early-return x3
{
  unsigned long v1; // rax
  
  if (*(long *)(a0 + 0x20))
    return 0;
  if (*(long *)(a0 + 0x28))
    return 0;
  if (*(char *)(a0 + 0x30))
    return 0;
  if (*(char *)(a0 + 0x31))
    return 0;
  v1 = sub_5660();
  if ((char)v1)
    return 0;
  if ((!*(char *)(a0 + 0x36)) && (!*(char *)(a0 + 0x38)))
    return (unsigned long)(*(unsigned char *)(a0 + 0x33) ^ 1);
  return v1;
}


// Function: sub_56d0 @ 0x56d0
void sub_56d0(long a0,char *a1)
{
  if (*(short *)(a0 + 0x30)) {
    *a1 = 0x62;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1e540) {
    *a1 = 100;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x28)) {
    *a1 = 0x66;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x34)) {
    *a1 = 0x67;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x35)) {
    *a1 = 0x68;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1e640) {
    *a1 = 0x69;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x36)) {
    *a1 = 0x4d;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x32)) {
    *a1 = 0x6e;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x33)) {
    *a1 = 0x52;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x37)) {
    *a1 = 0x72;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x38)) {
    *a1 = 0x56;
    a1 = &a1[1];
  }
  *a1 = 0;
}


// Function: sub_5780 @ 0x5780
bool sub_5780(long a0,long a1)
{
  if (*(unsigned int *)(a0 + 0x50) != *(unsigned int *)(a1 + 0x50))
    return *(unsigned int *)(a0 + 0x50) < *(unsigned int *)(a1 + 0x50);
  return (unsigned long)(*(long *)(a0 + 0x30) + *(long *)(a0 + 0x28)) < (unsigned long)(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28));
}


// Function: sub_57c0 @ 0x57c0
char * sub_57c0(char *a0,long a1,unsigned int a2)
{
  char v1;
  
  v1 = *a0;
  if (v1) {
    do {
      switch(v1) {
        case 0x4d:
          *(char *)(a1 + 0x36) = 1;
          break;
        default:
          goto label_581d;
        case 0x52:
          *(char *)(a1 + 0x33) = 1;
          break;
        case 0x56:
          *(char *)(a1 + 0x38) = 1;
          break;
        case 0x62:
          if (!(a2 & 0xfffffffd))
            *(char *)(a1 + 0x30) = 1;
          if (a2 - 1 <= 1)
            *(char *)(a1 + 0x31) = 1;
          break;
        case 100:
          *(unsigned long *)(a1 + 0x20) = 0x1e540;
          break;
        case 0x66:
          *(unsigned long *)(a1 + 0x28) = 0x1e440;
          break;
        case 0x67:
          *(char *)(a1 + 0x34) = 1;
          break;
        case 0x68:
          *(char *)(a1 + 0x35) = 1;
          break;
        case 0x69:
          if (!*(long *)(a1 + 0x20))
            *(unsigned long *)(a1 + 0x20) = 0x1e640;
          break;
        case 0x6e:
          *(char *)(a1 + 0x32) = 1;
          break;
        case 0x72:
          *(char *)(a1 + 0x37) = 1;
        
      }
      v1 = a0[1];
      a0 = &a0[1];
    } while (v1);
  }
label_581d:
  return a0;
}


// Function: sub_5930 @ 0x5930
void sub_5930(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}


// Function: sub_5940 @ 0x5940
void sub_5940(unsigned long a0)
{
  error(2,0,dcgettext(NULL,"options \'-%s\' are incompatible",5),a0);
}


// Function: sub_5970 @ 0x5970
void sub_5970(void)
{
  long v1;
  char v2 [40];
  char *v3; // rsp
  long v4; // fs_offset
  unsigned long v5; // stack - 0x10
  
  v3 = v2;
  v5 = *(unsigned long *)(v4 + 0x28);
  v1 = dat_1e410;
  while( true ) {
    if (!v1)
      return;
    if (1 < (unsigned int)*(unsigned char *)(v1 + 0x32) + (unsigned int)*(unsigned char *)(v1 + 0x34) + (unsigned int)*(unsigned char *)(v1 + 0x35) + (unsigned int)*(unsigned char *)(v1 + 0x36) + (unsigned int)(unsigned char)(*(unsigned char *)(v1 + 0x38) | *(unsigned char *)(v1 + 0x33) | *(long *)(v1 + 0x20) != 0)) break;
    v1 = *(long *)(v1 + 0x40);
  }
  *(char *)(v1 + 0x37) = 0;
  *(unsigned short *)(v1 + 0x30) = 0;
  sub_56d0(v1,v2);
  sub_5940(v3); // no-return
}


// Function: sub_5a10 @ 0x5a10
void sub_5a10(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_11430(a0);
  v2 = dcgettext(NULL,a1,5);
  error(2,0,dcgettext(NULL,"%s: invalid field specification %s",5),v2,v1);
}


// Function: sub_5a60 @ 0x5a60
void sub_5a60(long a0)
{
  *(bool *)a0 = pthread_sigmask(0,0x1e380,a0 + 8) == 0;
}


// Function: sub_5a80 @ 0x5a80
void sub_5a80(char *a0)
{
  if (!*a0)
    return;
  pthread_sigmask(2,&a0[8],0); // tail-call
}


// Function: sub_5aa0 @ 0x5aa0
void sub_5aa0(void)
{
  unsigned long *v1;
  
  for (v1 = dat_1e368; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_1e368 = NULL;
}


// Function: sub_5ad0 @ 0x5ad0
void sub_5ad0(void)
{
  char v1 [136];
  
  if (dat_1e368) {
    sub_5a60(v1);
    sub_5aa0();
    sub_5a80(v1);
  }
  sub_c5c0(); // tail-call
}


// Function: sub_5b40 @ 0x5b40
void sub_5b40(int a0)
{
  sub_5aa0();
  signal(a0,0);
  raise(a0); // tail-call
}


// Function: sub_5b80 @ 0x5b80
unsigned long sub_5b80(void)
{
  char v1;
  char *v10;
  unsigned long v11; // rdi
  long v12;
  long v13; // r13
  long *v14; // r14
  unsigned long v15; // stack - 0x38
  unsigned short v2;
  unsigned char v3; // al
  long *v4; // rax
  char *v5; // rax
  long v6; // rax
  unsigned long v7; // rax
  long v8; // rdx
  unsigned long v9; // rax
  
  v9 = 0;
  v4 = __ctype_b_loc();
  do {
    *(char *)(v9 + 0x1e740) = sub_5b60(v9 & 0xffffffff);
    v2 = *(unsigned short *)(*v4 + v9 * 2);
    *(unsigned char *)(v9 + 0x1e640) = ((unsigned char)(v2 >> 0xe) ^ 1) & 1;
    v3 = 0;
    if (!(v2 & 8))
      v3 = sub_5b60(v9 & 0xffffffff) ^ 1;
    *(unsigned char *)(v9 + 0x1e540) = v3;
    *(char *)(v9 + 0x1e440) = (char)toupper((int)v9);
    v9 += 1;
  } while (v9 != 0x100);
  if (dat_1e860) {
    v14 = (long *)0x1e060;
    v13 = 1;
    do {
      v5 = nl_langinfo((int)v13 + 0x2000d);
      v9 = strlen(v5);
      v6 = sub_12eb0(v9 + 1);
      *(int *)&v14[1] = (int)v13;
      *v14 = v6;
      if (v9) {
        v12 = *v4;
        v10 = &v5[v9];
        v9 = 0;
        do {
          v1 = *v5;
          v11 = (unsigned long)(unsigned char)sub_53e0((int)v1);
          if (!(*(unsigned char *)(v12 + v11 * 2) & 1)) {
            *(char *)(v6 + v9) = *(char *)(v11 + 0x1e440);
            v9 += 1;
          }
          v5 = (char *)(v8 + 1);
        } while (v10 != v5);
      }
      v13 += 1;
      *(char *)(v6 + v9) = 0;
      v14 = &v14[2];
    } while (v13 != 0xd);
    v7 = qsort(0x1e060,0xc,0x10,sub_5930,v15); // tail-call
    return v7;
  }
  return v15;
}


// Function: sub_5cf0 @ 0x5cf0
char (* sub_5cf0(char (*a0)[16]))[16]
{
  *(unsigned long *)a0[4] = 0;
  a0[1] = 0;
  *a0 = 0;
  *(unsigned long *)a0[1] = 0xffffffffffffffff;
  a0[2] = 0;
  a0[3] = 0;
  return a0;
}


// Function: sub_5d20 @ 0x5d20
char * sub_5d20(unsigned long a0,unsigned long *a1,char *a2)
{
  unsigned long v1;
  char *v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  
  switch(sub_13650(a0,&v5,10,&v4,"")) {
    case 0:
    case 2:
      *a1 = v4;
      break;
    case 1:
    case 3:
      *a1 = 0xffffffffffffffff;
      break;
    case 4:
      if (!a2) {
        v3 = NULL;
        return v3;
      }
      v3 = (unsigned long)sub_11430(a0);
      v1 = v3;
      v3 = dcgettext(NULL,a2,5);
      v2 = v3;
      v3 = dcgettext(NULL,"%s: invalid count at start of %s",5);
      error(2,0,v3,v2,v1);
      return v3;
    
  }
  v3 = (char *)v5;
  return v3;
}


// Function: sub_5e10 @ 0x5e10
void sub_5e10(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = dcgettext(NULL,"standard output",5);
  v1 = sub_11270(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1);
}


// Function: sub_5e70 @ 0x5e70
unsigned long sub_5e70(void) // return-dupe x2, return-dupe
{
  if (dat_1e2f0) {
    if (dat_1e2f0 <= -1)
      return 0x1e260;
    return 0;
  }
  if (!fstat(1,(void *)0x1e260)) {
    dat_1e2f0 = 0xffffffff;
    return 0x1e260;
  }
  dat_1e2f0 = *__errno_location();
  if (dat_1e2f0 <= -1)
    return 0x1e260;
  return 0;
}


// Function: sub_5ee0 @ 0x5ee0
int * sub_5ee0(char *a0,char *a1)
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  void *v5;
  
  if (*a1 == 'r') {
    v3 = strcmp(a0,"-");
    if (v3) { // branch-flip
      v5 = NULL;
      v3 = open(a0,0x80000);
      if (0 <= (int)v3) {
        v4 = fdopen(v3,a1);
        v5 = v4;
      }
    }
    else {
      dat_1e418 = 1;
      v5 = stdin;
    }
    sub_c8d0(v5,2);
    v4 = v5;
    return v4;
  }
  if (*a1 == 'w') {
    if (a0) {
      v3 = ftruncate(1,0);
      if (v3) {
        v4 = __errno_location();
        v1 = *v4;
        v4 = (long)sub_5e70();
        if ((!v4) || (v3 = *(unsigned int *)((long)v4 + 0x18) & 0xf000, v3 == 0x8000)) {
          v4 = (unsigned long)sub_11270(0,3,a0);
          v2 = v4;
          v4 = (char *)dcgettext(NULL,"%s: error truncating",5);
          error(2,v1,(char *)v4,v2);
          return v4;
        }
      }
    }
    v4 = stdout;
    return v4;
  }
  __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
}


// Function: sub_6010 @ 0x6010
void sub_6010(unsigned long a0,unsigned long a1)
{
  if (sub_5ee0(a0,a1))
    return;
  sub_5e10(dcgettext(NULL,"open failed",5),a0); // no-return
}


// Function: sub_6040 @ 0x6040
void sub_6040(void *a0,unsigned long a1) // return-dupe
{
  int v1; // eax
  char *v2; // rsi
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0); // tail-call
    return;
  }
  if (v1 != 1) { // branch-flip
    if (!sub_c900(a0))
      return;
    v2 = "close failed";
  }
  else {
    if (!fflush_unlocked(a0))
      return;
    v2 = "fflush failed";
  }
  sub_5e10(dcgettext(NULL,v2,5),a1); // no-return
}


// Function: sub_60c0 @ 0x60c0
void sub_60c0(unsigned long a0)
{
  long v1;
  long v2; // rax
  long *v3; // rcx
  long v4; // rdx
  
  v2 = sub_131b0(a0,0x48);
  v3 = (long *)0x1e410;
  v1 = dat_1e410;
  if (dat_1e410) {
    do {
      v4 = v1;
      v1 = *(long *)(v4 + 0x40);
    } while (v1);
    v3 = (long *)(v4 + 0x40);
  }
  *v3 = v2;
  *(unsigned long *)(v2 + 0x40) = 0;
}


// Function: sub_6110 @ 0x6110
void sub_6110(char *a0)
{
  long v1; // rax
  char v2 [24];
  char *v3; // rsi
  
  v1 = sub_11710(a0,0x10);
  if (v1) { // branch-flip
    sub_11830(v1,v2,0x10);
    if (!sub_11850(v1)) {
      sub_c680(0x1e300);
      sub_c690(v2,0x10,0x1e300);
      return;
    }
    v3 = "close failed";
  }
  else {
    if (!a0)
      a0 = "getrandom";
    v3 = "open failed";
  }
  sub_5e10(dcgettext(NULL,v3,5),a0); // no-return
}


// Function: sub_61e0 @ 0x61e0
void sub_61e0(unsigned long a0)
{
  long v1;
  
  v1 = dat_1e430;
  if (dat_1e428 == dat_1e420) {
    v1 = sub_12ff0(dat_1e430,0x1e420,8);
    dat_1e430 = v1;
  }
  *(unsigned long *)(v1 + dat_1e428 * 8) = a0;
  dat_1e428 += 1;
}


// Function: sub_6240 @ 0x6240
void sub_6240(unsigned long *a0,long a1,unsigned long a2)
{
  void *v1; // rax
  
  do {
    a2 = (a2 & 0xffffffffffffffe0) + 0x20;
    v1 = malloc(a2);
    *a0 = v1;
    if (v1) {
      a0[5] = a1;
      a0[3] = a2;
      a0[2] = 0;
      a0[4] = 0;
      a0[1] = 0;
      *(char *)&a0[6] = 0;
      return;
    }
    a2 >>= 1;
  } while (a1 + 1U < a2);
  sub_13270(); // no-return
}


// Function: sub_62b0 @ 0x62b0
long sub_62b0(char *a0,char *a1)
{
  long v1; // rax
  char *v2; // rbx
  
  v1 = (long)(int)sub_e580(a0,(long)a1 - (long)a0,0);
  if (a0 < a1) {
    do {
      v2 = &a0[1];
      v1 += (unsigned long)(*a0 == '\t');
      a0 = v2;
    } while (a1 != v2);
  }
  return v1;
}


// Function: sub_62f0 @ 0x62f0
void sub_62f0(long a0,long a1)
{
  for (; a0; a0 = a0 + -1) {
    putchar_unlocked(0x20);
  }
  if (a1) {
    do {
      putchar_unlocked(0x5f);
      a1 -= 1;
    } while (a1);
    putchar_unlocked(10); // tail-call
    return;
  }
  __printf_chk(1,dcgettext(NULL,"^ no match for key\n",5)); // tail-call
}


// Function: sub_6370 @ 0x6370
void sub_6370(unsigned long a0,unsigned long a1)
{
  undefined10 v1; // stack - 0x38
  undefined10 v2; // stack - 0x28
  undefined10 v3; // st0
  undefined10 v4; // st0
  
  strtold(a0,0);
  v1 = v3;
  strtold(a1,0);
  v2 = v4;
  memcmp(&v1,&v2,0x10);
}


// Function: sub_63d0 @ 0x63d0
unsigned long sub_63d0(long a0,long a1) // return-dupe
{
  long v1; // stack - 0x30
  long v2; // stack - 0x28
  long double v3; // st0
  long double v4; // st0
  
  strtold(a0,&v1);
  strtold(a1,&v2);
  if (v1 == a0)
    return (unsigned long)-(unsigned int)(v2 != a1);
  if (v2 == a1)
    return 1;
  if (v3 < v4)
    return 0xffffffff;
  if (v3 <= v4) {
    if (v3 == v4)
      return 0;
    if (!NAN(v4))
      return 0xffffffff;
    if (!NAN(v3))
      return 1;
    return sub_6370(a0,a1);
  }
  return 1;
}


// Function: sub_64d0 @ 0x64d0
void sub_64d0(char *a0,char *a1)
{
  char v1;
  long v2; // rdx
  long v3; // rdx
  
  while (v1 = *a0, *(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a0 = &a0[1];
  }
  while (v1 = *a1, *(char *)(v3 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a1 = &a1[1];
  }
  sub_12790(a0); // tail-call
}


// Function: sub_6530 @ 0x6530
void sub_6530(char *a0,char *a1)
{
  char v1;
  int v2; // eax
  long v3; // rdx
  long v4; // rdx
  
  while (v1 = *a0, *(char *)(v3 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a0 = &a0[1];
  }
  while (v1 = *a1, *(char *)(v4 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a1 = &a1[1];
  }
  v2 = sub_54f0(a0);
  if (v2 != sub_54f0(a1))
    return;
  sub_12790(a0,a1,(int)dat_1e868,dat_1e864); // tail-call
}


// Function: sub_65c0 @ 0x65c0
void sub_65c0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int *v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  strxfrm(a0,a1,a2);
  if (!*v1)
    return;
  v2 = dcgettext(NULL,"string transformation failed",5);
  error(0,*v1,v2);
  error(0,0,dcgettext(NULL,"set LC_ALL=\'C\' to work around the problem",5));
  v3 = sub_110a0(0,8,a1);
  error(2,0,dcgettext(NULL,"the untransformed string was %s",5),v3);
}


// Function: sub_6680 @ 0x6680
int sub_6680(char *a0,unsigned long a1,char *a2,unsigned long a3)
{
  char *v1;
  char v10 [16];
  char v11 [16];
  char *v12;
  unsigned long v13;
  char *v14;
  unsigned long v15;
  char *v16; // stack - 0x1118
  char *v17; // stack - 0x1108
  unsigned long v18; // stack - 0x10f8
  int v19; // stack - 0x10ec
  char *v2;
  unsigned long v20; // stack - 0x10a0
  unsigned long v21; // stack - 0x1098
  unsigned long v22; // stack - 0x1090
  unsigned long v23; // stack - 0x1088
  unsigned long v24; // stack - 0x1080
  unsigned long v25; // stack - 0x1078
  unsigned long v26; // stack - 0x1070
  unsigned long v27; // stack - 0x1068
  unsigned long v28; // stack - 0x1060
  unsigned long v29; // stack - 0x1058
  int v3; // eax
  unsigned int v30; // stack - 0x1050
  unsigned long v31; // stack - 0x1044
  unsigned long v32; // stack - 0x103c
  unsigned int v33; // stack - 0x1034
  unsigned int v34; // stack - 0x1030
  unsigned int v35; // stack - 0x102c
  unsigned int v36; // stack - 0x1028
  unsigned long v37; // stack - 0x1024
  unsigned long v38; // stack - 0x101c
  unsigned long v39; // stack - 0x1014
  char *v4;
  unsigned long v40; // stack - 0x100c
  unsigned long v41; // stack - 0x1004
  unsigned long v42; // stack - 0xffc
  unsigned int v43; // stack - 0xff4
  unsigned long v5;
  unsigned long v6;
  char v7 [4008];
  unsigned long v8; // stack - 0x10a8
  unsigned long v9; // stack - 0x104c
  
  v43 = dat_1e358;
  v42 = dat_1e350;
  v29 = dat_1e350;
  v30 = dat_1e358;
  v9 = dat_1e300;
  v31 = dat_1e308;
  v32 = dat_1e310;
  v34 = (unsigned int)((unsigned long)dat_1e318 >> 0x20);
  v33 = (unsigned int)dat_1e318;
  v36 = (unsigned int)((unsigned long)dat_1e320 >> 0x20);
  v35 = (unsigned int)dat_1e320;
  v37 = dat_1e328;
  v38 = dat_1e330;
  v39 = dat_1e338;
  v40 = dat_1e340;
  v41 = dat_1e348;
  v8 = dat_1e300;
  v20 = dat_1e308;
  v21 = dat_1e310;
  v22 = dat_1e318;
  v23 = dat_1e320;
  v24 = dat_1e328;
  v25 = dat_1e330;
  v26 = dat_1e338;
  v27 = dat_1e340;
  v28 = dat_1e348;
  if (dat_1e861) { // branch-flip
    v1 = &a0[a1];
    v14 = NULL;
    v13 = 4000;
    v2 = &a2[a3];
    v6 = (a1 + a3) * 3 + 2;
    v19 = 0;
    v16 = v7;
label_67c8:
    if (v13 < v6) goto label_6899;
label_67d3:
    v5 = v13;
    v4 = v14;
    if (a0 < v1) goto label_68d7;
label_67de:
    v14 = v4;
    if (a2 < v2) {
      a1 = 0;
      v12 = v16;
      v5 = v13;
      goto label_6902;
    }
    v18 = 0;
    a1 = 0;
    v17 = v16;
label_67fd:
    if (a2 < v2) {
label_6808:
      do {
        v4 = (unsigned long)strlen(a2);
        a2 = &a2[(long)v4 + 1];
        do {
          while( true ) {
            if ((v1 <= a0) && (v2 <= a2)) goto label_6bee;
            sub_c690(v16,a1,&v8);
            sub_c690(v17,v18,&v9);
            if (v19) goto label_67c8;
            v5 = v18;
            if (a1 <= v18)
              v5 = a1;
            v3 = memcmp(v16,v17,v5);
            v19 = v3;
            if (v3) goto label_67c8;
            v3 = (unsigned int)(a1 >= v18 && a1 != v18) - (unsigned int)(a1 < v18);
            v19 = v3;
            if (v6 <= v13) goto label_67d3;
label_6899:
            v13 = v13 * 3 >> 1;
            if (v13 < v6)
              v13 = v6;
            free(v14);
            v4 = malloc(v13);
            v14 = v4;
            if (!v4) {
              v13 = 4000;
              v16 = v7;
              goto label_67d3;
            }
            v16 = v4;
            v5 = v13;
            if (v1 <= a0) goto label_67de;
label_68d7:
            v4 = (long)sub_65c0(v16,a0,v5);
            a1 = (long)v4 + 1;
            if (v2 > a2) break;
            v18 = 0;
            v13 = v5;
            v15 = a1;
            if (v5 >= a1) goto label_6b78;
label_6920:
            v13 = v15;
            if (v15 <= 0x5555555555555554)
              v13 = v15 * 3 >> 1;
            free(v14);
            v4 = (char *)sub_12eb0(v13);
            v14 = v4;
            if (v1 <= a0) { // branch-flip
              v17 = &v4[a1];
              if (a2 < v2) {
                strxfrm(v17,a2,v18);
                v16 = v4;
                goto label_6808;
              }
            }
            else {
              strxfrm(v4,a0,a1);
              v17 = &v4[a1];
              if (a2 < v2) {
                strxfrm(v17,a2,v18);
                v4 = (unsigned long)strlen(a0);
                v16 = v14;
                a0 = &a0[(long)v4 + 1];
                goto label_6808;
              }
            }
            v17 = &v4[a1];
            if (v1 <= a0) {
              a0 = v4;
              goto label_697c;
            }
            v4 = (unsigned long)strlen(a0);
            v16 = v14;
            a0 = &a0[(long)v4 + 1];
          }
          if (a1 <= v5) { // branch-flip
            v13 = v5 - a1;
            v12 = &v16[a1];
          }
          else {
            v13 = 0;
            v12 = NULL;
          }
label_6902:
          v4 = (long)sub_65c0(v12,a2,v13);
          v4 = (unsigned long)((long)v4 + 1);
          v13 = v5;
          v15 = (long)v4 + a1;
          v18 = (unsigned long)v4;
          if (a1 > v5) goto label_6920;
label_6b78:
          if (v13 < v15) goto label_6920;
          v4 = &v16[a1];
          v17 = v4;
          if (v1 <= a0) goto label_67fd;
          v4 = (unsigned long)strlen(a0);
          a0 = &a0[(long)v4 + 1];
        } while (v2 <= a2);
      } while( true );
    }
label_6bee:
    a0 = v16;
label_697c:
    sub_c690(a0,a1,&v8);
    sub_c6c0(&v8,v10);
    sub_c690(v17,v18,&v9);
    sub_c6c0(&v9,v11);
    v3 = memcmp(v10,v11,0x10);
    if (!v3) {
      if (v19) {
        free(v14); // return-dupe
        v3 = v19;
        return v3;
      }
      goto label_6b27;
    }
  }
  else {
    sub_c690(a0,a1,&v8);
    sub_c6c0(&v8,v10);
    sub_c690(a2,a3,&v9);
    sub_c6c0(&v9,v11);
    v3 = memcmp(v10,v11,0x10);
    v14 = NULL;
    v19 = v3;
    v18 = a3;
    v17 = a2;
    if (v3) {
      free(v14);
      v3 = v19;
      return v3;
    }
label_6b27:
    v6 = v18;
    if (a1 <= v18)
      v6 = a1;
    v3 = memcmp(a0,v17,v6);
    v19 = v3;
    if (v3) {
      free(v14);
      v3 = v19;
      return v3;
    }
    v3 = (unsigned int)(v18 < a1) - (unsigned int)(a1 < v18);
  }
  v19 = v3;
  free(v14);
  v3 = v19;
  return v3;
}


// Function: sub_6c00 @ 0x6c00
void sub_6c00(unsigned long *a0,long a1)
{
  char *v1;
  unsigned long v2;
  long v3; // rbp
  
  if (!a1)
    return;
  v3 = 0;
  do {
    v1 = (char *)*a0;
    if (strcmp(v1,"-")) {
      if (euidaccess(v1,4)) {
        v2 = *a0;
        sub_5e10(dcgettext(NULL,"cannot read",5),v2); // no-return
      }
    }
    v3 += 1;
    a0 = &a0[1];
  } while (a1 != v3);
}


// Function: sub_6c80 @ 0x6c80
void sub_6c80(int a0,int a1)
{
  if (a0 == a1)
    return;
  dup2(a0,a1);
  close(a0); // tail-call
}


// Function: sub_6ca0 @ 0x6ca0
void sub_6ca0(char *a0)
{
  int v1; // eax
  
  if (!a0)
    return;
  v1 = open(a0,0x80041,0x1b6);
  if (0 <= v1) {
    sub_6c80(v1,1); // tail-call
    return;
  }
  sub_5e10(dcgettext(NULL,"open failed",5),a0); // no-return
}


// Function: sub_6cf0 @ 0x6cf0
bool sub_6cf0(unsigned int a0)
{
  long v1; // rax
  char v2 [8];
  unsigned int v3; // stack - 0x20
  
  v3 = a0;
  v1 = sub_dff0(dat_1e360,v2);
  if (v1)
    *(char *)(v1 + 0xc) = 2;
  return v1 != 0;
}


// Function: sub_6d50 @ 0x6d50
unsigned long sub_6d50(int a0)
{
  unsigned int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned int v6; // stack - 0x24
  int v7;
  
  v7 = -1;
  if (a0)
    v7 = a0;
  v1 = waitpid(v7,(int *)&v6,(unsigned int)(a0 == 0));
  if (0 <= (int)v1) {
    if ((v1) && ((1 <= a0 || (sub_6cf0(v1))))) {
      if (v6 >> 8 & 0xff || v6 & 0x7f) {
        v4 = sub_11170(4,dat_1e408);
        error(2,0,dcgettext(NULL,"%s [-d] terminated abnormally",5),v4);
        return v5;
      }
      dat_1e35c -= 1;
    }
    return (unsigned long)v1;
  }
  v4 = sub_11170(4,dat_1e408);
  v2 = dcgettext(NULL,"waiting for %s [-d]",5);
  error(2,*__errno_location(),v2,v4);
  return v3;
}


// Function: sub_6e60 @ 0x6e60
void sub_6e60(unsigned long a0)
{
  if (!sub_6cf0(a0))
    return;
  sub_6d50(a0 & 0xffffffff); // tail-call
}


// Function: sub_6e80 @ 0x6e80
void sub_6e80(char *a0)
{
  int v1;
  long v2;
  long *v3;
  long *v4;
  int v5; // eax
  unsigned long v6; // rax
  char v7 [136];
  
  v3 = (long *)0x1e368;
  for (v4 = dat_1e368; a0 != (char *)((long)v4 + 0xd); v4 = (long *)*v4) {
    v3 = v4;
  }
  if (*(char *)((long)v4 + 0xc) == '\x01')
    sub_6e60((int)v4[1]);
  v2 = *v4;
  sub_5a60(v7);
  v5 = unlink(a0);
  v1 = *__errno_location();
  *v3 = v2;
  sub_5a80(v7);
  if (v5) {
    v6 = sub_11270(0,3,a0);
    error(0,v1,dcgettext(NULL,"warning: cannot remove: %s",5),v6);
  }
  if (!v2)
    dat_1e040 = v3;
  free(v4);
}


// Function: sub_6fb0 @ 0x6fb0
void sub_6fb0(void) // return-dupe
{
  if (dat_1e35c <= 0)
    return;
  do {
    if (!sub_6d50(0))
      return;
  } while (0 < dat_1e35c);
}


// Function: sub_6ff0 @ 0x6ff0
void sub_6ff0(void)
{
  sub_6d50(0xffffffff);
  sub_6fb0(); // tail-call
}


// Function: sub_7010 @ 0x7010
void sub_7010(void)
{
  if (dat_1e35c <= 0)
    return;
  do {
    sub_6d50(0xffffffff);
  } while (0 < dat_1e35c);
}


// Function: sub_7040 @ 0x7040
int sub_7040(int *a0,long a1)
{
  unsigned long v1;
  int v2; // eax
  int *v3; // rax
  char v4 [136];
  int v5;
  double v6; // stack - 0xd8
  
  if (0 <= (int)sub_ec90(a0,0x80000)) { // branch-flip
    if (dat_1e024 + 1U < dat_1e35c)
      sub_6ff0();
    v3 = __errno_location();
    v6 = dat_170f0;
    do {
      a1 -= 1;
      sub_5a60(v4);
      v1 = dat_1e368;
      dat_1e368 = 0;
      v2 = fork();
      v5 = *v3;
      if (!v2) {
        sub_5a80(v4);
        *v3 = v5;
        close(0);
        close(1);
        return 0;
      }
      dat_1e368 = v1;
      sub_5a80(v4);
      *v3 = v5;
      if (0 <= v2) {
        dat_1e35c += 1;
        return v2;
      }
      if (v5 != 0xb) goto label_717d;
      sub_13440(v6);
      v6 = v6 + v6;
      sub_6fb0();
    } while (a1);
    v5 = *v3;
label_717d:
    close(*a0);
    close(a0[1]);
    *v3 = v5;
  }
  else {
    v2 = -1;
  }
  return v2;
}


// Function: sub_71d0 @ 0x71d0
void sub_71d0(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_e440(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}


// Function: sub_7270 @ 0x7270
void sub_7270(long a0)
{
  long v1;
  
  v1 = dat_1e360;
  if (!dat_1e360) {
    v1 = sub_d970(0x2f,0,sub_5400,sub_5420,0);
    dat_1e360 = v1;
    if (!v1) {
      sub_13270(); // no-return, return-dupe
    }
  }
  *(char *)(a0 + 0xc) = 1;
  if (sub_df90(v1,a0))
    return;
  sub_13270();
}


// Function: sub_72d0 @ 0x72d0
int * sub_72d0(long a0)
{
  int v1;
  int *v2;
  unsigned long v3;
  int v4; // eax
  int *v5; // rax
  int v6; // stack - 0x28
  void *v7;
  int v8; // stack - 0x24
  
  if (*(char *)(a0 + 0xc) == '\x01')
    sub_6e60(*(unsigned int *)(a0 + 8));
  v4 = open((char *)(a0 + 0xd),0);
  v1 = v4;
  if (0 <= v4) { // branch-flip
    v4 = sub_7040(&v6,9);
    if (v4 != -1) { // branch-flip
      if (!v4) {
        close(v6);
        sub_6c80(v1,0);
        sub_6c80(v8,1);
        execlp(dat_1e408,dat_1e408,"-d",0);
        v5 = __errno_location();
        sub_71d0(*v5,"couldn\'t execute compress program (with -d)"); // no-return
      }
      *(int *)(a0 + 8) = v4;
      sub_7270(a0);
      close(v1);
      close(v8);
      v5 = fdopen(v6,"r");
      v7 = v5;
      if (!v5) {
        v5 = __errno_location();
        v1 = *v5;
        close(v6);
        *v5 = v1;
      }
    }
    else {
      v5 = __errno_location();
      v2 = v5;
      if (*v5 != 0x18) {
        v5 = (unsigned long)sub_11170(4,dat_1e408);
        v3 = v5;
        v5 = (char *)dcgettext(NULL,"couldn\'t create process for %s -d",5);
        error(2,*v2,(char *)v5,v3);
        return v5;
      }
      v7 = NULL;
      close(v1);
      *v5 = 0x18;
    }
  }
  else {
    v7 = NULL;
  }
  v5 = v7;
  return v5;
}


// Function: sub_7470 @ 0x7470
long sub_7470(unsigned long *a0,long a1,long *a2)
{
  long v1;
  long v2;
  long v3; // rax
  long v4; // r13
  
  v3 = sub_12fd0(a1,8);
  v2 = v3;
  *a2 = v3;
  if (a1) { // branch-flip
    v4 = 0;
    do {
      v1 = a0[1];
      if ((v1) && (*(char *)(v1 + 0xc))) {
        v3 = sub_72d0(v1);
        *(long *)(v2 + v4 * 8) = v3;
      }
      else {
        v3 = sub_5ee0(*a0,"r");
        *(long *)(v2 + v4 * 8) = v3;
      }
      if (!v3) {
        v3 = v4;
        return v3;
      }
      v4 += 1;
      a0 = &a0[2];
    } while (a1 != v4);
  }
  else {
    v4 = 0;
  }
  v3 = v4;
  return v3;
}


// Function: sub_7510 @ 0x7510
int * sub_7510(int *a0,char a1)
{
  void *v1;
  char v10 [136];
  unsigned long *v11;
  int v2;
  char *v3;
  int v4;
  unsigned long v5;
  unsigned long *v6;
  int *v7;
  unsigned long v8;
  int *v9; // rax
  
  v3 = *(char **)(dat_1e430 + dat_1e248 * 8);
  v9 = (unsigned long)strlen(v3);
  v5 = (unsigned long)v9;
  v9 = (unsigned long *)sub_12eb0((long)v9 + 0x20U & 0xfffffffffffffff8);
  v6 = (unsigned long *)v9;
  v1 = (void *)((long)v9 + 0xd);
  memcpy(v1,v3,v5);
  *(unsigned long *)((long)v9 + v5 + 0xd) = s_152f0._0_8_;
  *(unsigned int *)((long)v9 + v5 + 0x15) = s_152f0._8_4_;
  *(unsigned long *)v9 = 0;
  v9 = (long)(dat_1e248 + 1);
  dat_1e248 = (long)v9;
  if (v9 == (int *)dat_1e428)
    dat_1e248 = 0;
  sub_5a60(v10);
  v4 = sub_12280(v1,0x80000);
  v9 = __errno_location();
  v7 = v9;
  if (0 <= v4) { // branch-flip
    v2 = *v9;
    *dat_1e040 = v6;
    dat_1e040 = v6;
    sub_5a80(v10);
    *v9 = v2;
    v11 = v6;
  }
  else {
    v2 = *v9;
    sub_5a80(v10);
    *v9 = v2;
    if ((a1 != '\x01') || (v2 != 0x18)) {
      v9 = (unsigned long)sub_11170(4,v3);
      v8 = v9;
      v9 = (char *)dcgettext(NULL,"cannot create temporary file in %s",5);
      error(2,*v7,(char *)v9,v8);
      return v9;
    }
    v11 = NULL;
    free(v6);
  }
  *a0 = v4;
  v9 = (int *)v11;
  return v9;
}


// Function: sub_76b0 @ 0x76b0
long sub_76b0(unsigned long *a0,char a1)
{
  int v1; // eax
  long v2; // rax
  void *v3; // rax
  int v4; // stack - 0x2c
  int v5; // stack - 0x28
  int v6; // stack - 0x24
  
  v2 = sub_7510(&v4,a1);
  if (v2) {
    *(char *)(v2 + 0xc) = 0;
    if (dat_1e408) {
      v1 = sub_7040(&v5,4);
      *(int *)(v2 + 8) = v1;
      if (1 <= v1) { // branch-flip
        close(v4);
        close(v5);
        v4 = v6;
        sub_7270(v2);
      }
      else if (!v1) {
        close(v6);
        sub_6c80(v4,1);
        sub_6c80(v5,0);
        execlp(dat_1e408,dat_1e408,0);
        sub_71d0(*__errno_location(),"couldn\'t execute compress program"); // no-return
      }
    }
    v3 = fdopen(v4,"w");
    *a0 = v3;
    if (!v3)
      sub_5e10(dcgettext(NULL,"couldn\'t create temporary file",5),v2 + 0xd); // no-return
  }
  return v2;
}


// Function: sub_77e0 @ 0x77e0
void sub_77e0(unsigned long a0)
{
  sub_76b0(a0,0); // tail-call
}


// Function: sub_77f0 @ 0x77f0
unsigned long sub_77f0(void) // ternary x2
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3 [3]; // stack - 0x38
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6; // xmm0_qa
  double v7;
  
  v2 = 0xffffffffffffffff;
  if (!getrlimit(2,v3))
    v2 = v3[0];
  if ((!getrlimit(9,v3)) && (v3[0] < v2))
    v2 = v3[0];
  v2 >>= 1;
  if ((!getrlimit(5,v3)) && (v1 = (v3[0] & 0xfffffffffffffff0) - (v3[0] >> 4), v1 < v2))
    v2 = v1;
  sub_eb50();
  sub_ea60();
  v6 = v5 * dat_17100;
  v7 = v4;
  if (v4 <= dat_170f8 * v5)
    v7 = dat_170f8 * v5;
  if (v6 < (double)(long)v2) {
    v2 = (v6 < dat_17108) ? (unsigned long)v6 : (long)(v6 - dat_17108) ^ 0x8000000000000000;
  }
  if (v7 < (double)v2) {
    v2 = (v7 < dat_17108) ? (unsigned long)v7 : (long)(v7 - dat_17108) ^ 0x8000000000000000;
  }
  v1 = (unsigned long)dat_1e024 * 0x22;
  if ((unsigned long)dat_1e024 * 0x22 < v2)
    v1 = v2;
  return v1;
}


// Function: sub_7970 @ 0x7970
long * sub_7970(long *a0,long *a1,long a2,unsigned long a3,long a4,char a5)
{
  long *v1;
  long v2;
  unsigned long v3; // rcx
  long v4; // rdx
  long *v5; // rsi
  long *v6;
  long v7; // r12
  
  while( true ) {
    v5 = a1;
    v7 = a2 + a4 * -0x20;
    if (a5) { // branch-flip
      v3 = (unsigned long)a0[5] >> 1;
      v4 = a0[5] - v3;
      v6 = &a0[2];
    }
    else {
      v3 = (unsigned long)a0[6] >> 1;
      v4 = a0[6] - v3;
      v6 = &a0[3];
    }
    a2 = v7 + v3 * -0x20;
    v5[7] = (long)a0;
    v2 = a0[10];
    v1 = &v5[0x10];
    v5[4] = (long)v6;
    v5[2] = v7;
    *v5 = v7;
    v5[3] = a2;
    v5[1] = a2;
    v5[5] = v3;
    v5[6] = v4;
    *(int *)&v5[10] = (int)v2 + 1;
    *(char *)((long)v5 + 0x54) = 0;
    pthread_mutex_init(&v5[0xb],0);
    if (a3 <= 1) break;
    v5[8] = (long)v1;
    v3 = a3 >> 1;
    a3 -= v3;
    a1 = (long *)sub_7970(v5,v1,v7,v3,a4,1);
    v5[9] = (long)a1;
    a5 = '\0';
    a0 = v5;
  }
  v5[8] = 0;
  v5[9] = 0;
  return v1;
}


// Function: sub_7aa0 @ 0x7aa0
unsigned long * sub_7aa0(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_12eb0(a0 << 8);
  v1[6] = a1;
  v1[5] = a1;
  v1[3] = 0;
  v1[2] = 0;
  v1[1] = 0;
  *v1 = 0;
  v1[4] = 0;
  v1[7] = 0;
  *(unsigned int *)&v1[10] = 0;
  *(char *)((long)v1 + 0x54) = 0;
  pthread_mutex_init(&v1[0xb],0);
  sub_7970(v1,&v1[0x10],a2,a0,a1,0);
  return v1;
}


// Function: sub_7b40 @ 0x7b40
void sub_7b40(unsigned long *a0,long a1)
{
  *a0 = sub_e300(sub_5780,a1 * 2);
  pthread_mutex_init(&a0[1],0);
  pthread_cond_init(&a0[6],0); // tail-call
}


// Function: sub_7b70 @ 0x7b70
void sub_7b70(long a0)
{
  pthread_mutex_lock((void *)(a0 + 0x58)); // tail-call
}


// Function: sub_7b80 @ 0x7b80
void sub_7b80(unsigned long *a0,long a1)
{
  pthread_mutex_lock(&a0[1]);
  sub_e390(*a0,a1);
  *(char *)(a1 + 0x54) = 1;
  pthread_cond_signal(&a0[6]);
  pthread_mutex_unlock(&a0[1]); // tail-call
}


// Function: sub_7bc0 @ 0x7bc0
void sub_7bc0(unsigned long a0,long *a1) // return-dupe
{
  long v1;
  
  if (*(char *)((long)a1 + 0x54))
    return;
  if (*a1 != a1[2]) { // branch-flip
    if (a1[1] != a1[3]) {
      sub_7b80(); // tail-call, return-dupe
      return;
    }
    v1 = a1[6];
  }
  else {
    if (a1[1] == a1[3])
      return;
    v1 = a1[5];
  }
  if (v1)
    return;
  sub_7b80();
  return;
}


// Function: sub_7c00 @ 0x7c00
void sub_7c00(long a0)
{
  pthread_mutex_unlock((void *)(a0 + 0x58)); // tail-call
}


// Function: sub_7c10 @ 0x7c10
void sub_7c10(unsigned long a0,long a1)
{
  if (2 <= *(unsigned int *)(a1 + 0x50)) {
    sub_7b70(*(unsigned long *)(a1 + 0x38));
    sub_7bc0(a0,*(unsigned long *)(a1 + 0x38));
    sub_7c00(*(unsigned long *)(a1 + 0x38)); // tail-call
    return;
  }
  if (*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28))
    return;
  sub_7b80(a0,*(unsigned long *)(a1 + 0x38)); // tail-call
}


// Function: sub_7c70 @ 0x7c70
long sub_7c70(unsigned long *a0)
{
  unsigned long *v1;
  long v2; // rax
  
  v1 = &a0[1];
  pthread_mutex_lock(v1);
  while( true ) {
    v2 = sub_e400(*a0);
    if (v2) break;
    pthread_cond_wait(&a0[6],v1);
  }
  pthread_mutex_unlock(v1);
  sub_7b70(v2);
  *(char *)(v2 + 0x54) = 0;
  return v2;
}


// Function: sub_7cd0 @ 0x7cd0
void sub_7cd0(unsigned long a0,void *a1)
{
  long v1;
  long v2; // rbx
  
  if (a0 & 0x7fffffffffffffff) {
    v1 = (long)a1 + 0x58;
    do {
      v2 = v1 + 0x80;
      pthread_mutex_destroy(v1);
      v1 = v2;
    } while (v2 != a0 * 0x100 + (long)a1 + 0x58);
  }
  free(a1); // tail-call
}


// Function: sub_7d10 @ 0x7d10
void sub_7d10(unsigned long *a0)
{
  sub_e370(*a0);
  pthread_cond_destroy(&a0[6]);
  pthread_mutex_destroy(&a0[1]); // tail-call
}


// Function: sub_7d30 @ 0x7d30
void sub_7d30(unsigned long *a0,char a1)
{
  unsigned char v1;
  char *v10; // rax
  char *v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14;
  char v15; // dl
  long *v16; // rdx
  long *v17; // rdx
  long v18;
  char v19 [32];
  unsigned long v2;
  char v20 [47];
  unsigned int v21; // stack - 0x78
  char v22; // stack - 0xa8
  unsigned long v23; // stack - 0x118
  unsigned char v24;
  unsigned long *v25;
  unsigned char v26;
  unsigned char v27; // stack - 0x14a
  unsigned char v28; // stack - 0x149
  long v29; // stack - 0x148
  unsigned short v3;
  unsigned char v30; // stack - 0x120
  unsigned char v31; // stack - 0x11f
  unsigned char v32; // stack - 0x11e
  char v33; // stack - 0x11c
  char v34; // stack - 0x11b
  char v35; // stack - 0x11a
  unsigned char v36; // stack - 0x119
  unsigned long v37; // stack - 0x110
  unsigned long v38; // stack - 0x108
  unsigned long v39; // stack - 0x100
  undefined3 v4;
  unsigned long v40; // stack - 0xf8
  unsigned long v41; // stack - 0xf0
  long v42; // stack - 0xe8
  unsigned long v43; // stack - 0xe0
  unsigned long v44; // stack - 0xd8
  char v5;
  char v6;
  unsigned char v7;
  unsigned char v8;
  char *v9; // rax
  
  v23 = *a0;
  v37 = a0[1];
  v38 = a0[2];
  v39 = a0[3];
  v40 = a0[4];
  v41 = a0[5];
  v14 = a0[6];
  v43 = a0[7];
  v44 = a0[8];
  v42 = v14;
  if (dat_1e410) {
    v42._1_1_ = (unsigned char)(v14 >> 8);
    v36 = 0;
    v35 = '\0';
    v42._0_1_ = (unsigned char)v14;
    v27 = v42._1_1_;
    v42._6_1_ = (unsigned char)(v14 >> 0x30);
    v42._4_1_ = (unsigned char)(v14 >> 0x20);
    v34 = '\0';
    v28 = v42._6_1_;
    v42._2_1_ = (unsigned char)(v14 >> 0x10);
    v33 = '\0';
    v42._3_1_ = (unsigned char)(v14 >> 0x18);
    v30 = v42._2_1_;
    v42._5_1_ = (unsigned char)(v14 >> 0x28);
    v29 = 1;
    v31 = v42._5_1_;
    v42._7_1_ = (unsigned char)(v14 >> 0x38);
    v32 = v42._7_1_;
    v25 = dat_1e410;
    v8 = (unsigned char)v42;
    v7 = (unsigned char)v43;
    v26 = v42._3_1_;
    v24 = v42._4_1_;
    do {
      v42 = v14;
      v5 = sub_5660(v25);
      if (v5) {
        v6 = *(char *)((long)v25 + 0x34);
        if (!v6) {
          v6 = v34;
          v33 = v5;
        }
        v34 = v6;
      }
      if (*(char *)((long)v25 + 0x39)) {
        v14 = *v25;
        v2 = v25[2];
        v18 = v14 + 1;
        if (v14 == 0xffffffffffffffff) {
          v18 = 1;
          v14 = 0;
        }
        v22 = 0x2b;
        v9 = __stpcpy_chk(v20,(char *)sub_e520(v14,v19),0x2d);
        v21 = 0x206b2d;
        v10 = __stpcpy_chk((char *)((long)&v21 + 3),(char *)sub_e520(v18,v19),0x2c);
        if (v25[2] != 0xffffffffffffffff) {
          v11 = (char *)sub_e520(v2 + 1,v19);
          v9[0] = ' ';
          v9[1] = '-';
          strcpy(&v9[2],v11);
          v9 = (char *)sub_e520((unsigned long)(v25[3] == 0xffffffffffffffff) + v2 + 1,v19);
          *v10 = ',';
          strcpy(&v10[1],v9);
        }
        v12 = sub_11420(1,&v21);
        v13 = sub_11420(0,&v22);
        error(0,0,dcgettext(NULL,"obsolescent key %s used; consider %s instead",5),v13,v12);
      }
      if ((*v25 != 0xffffffffffffffff) && (v25[2] < *v25))
        error(0,0,dcgettext(NULL,"key %lu has zero width and will be ignored",5),v29);
      v6 = sub_5660(v25);
      v5 = '\x01';
      if (!v6)
        v5 = *(char *)((long)v25 + 0x36);
      if ((v25[2]) || (!v25[3])) {
        if (v15) { // branch-flip
label_7e17:
          if (a1) goto label_7e22;
        }
        else if ((dat_1e028 == 0x80) && (((!(char)v25[6] && ((!v5 || (v25[1])))) || ((!*(char *)((long)v25 + 0x31) && (v25[3])))))) {
          error(0,0,dcgettext(NULL,"leading blanks are significant in key %lu; consider also specifying \'b\'",5),v29);
          v6 = sub_5660(v25);
        }
        if (!v6) goto label_7e22;
        v14 = *v25 + 1;
        if (!v14)
          v14 = 1;
        if (v25[2] < v14) goto label_7e22;
        error(0,0,dcgettext(NULL,"key %lu is numeric and spans multiple fields",5),v29);
        v1 = *(unsigned char *)((long)v25 + 0x34);
        if (v1) {
          v36 = v1;
          v6 = v35;
        }
        v35 = v6;
      }
      else {
        if (v15) goto label_7e17;
label_7e22:
        v1 = *(unsigned char *)((long)v25 + 0x34);
      }
      if ((v40) && (v40 == v25[4]))
        v40 = 0;
      if ((v41) && (v41 == v25[5]))
        v41 = 0;
      v24 &= v1 ^ 1;
      v42 = (unsigned long)v24 << 0x20;
      v8 &= (unsigned char)v25[6] ^ 1;
      v27 &= *(unsigned char *)((long)v25 + 0x31) ^ 1;
      v28 &= *(unsigned char *)((long)v25 + 0x36) ^ 1;
      v42._0_2_ = CONCAT11(v27,v8);
      v3 = (unsigned short)v42;
      v30 &= *(unsigned char *)((long)v25 + 0x32) ^ 1;
      v42._0_7_ = CONCAT16(v28,(undefined6)v42);
      v31 &= *(unsigned char *)((long)v25 + 0x35) ^ 1;
      v42._0_3_ = CONCAT12(v30,v3);
      v4 = (undefined3)v42;
      v26 &= *(unsigned char *)((long)v25 + 0x33) ^ 1;
      v42._0_5_ = (undefined5)CONCAT43((int)((undefined7)(undefined7)v42 >> 0x18),(undefined3)v42);
      v42._0_6_ = CONCAT15(v31,(undefined5)v42);
      v42._0_4_ = CONCAT13(v26,v4);
      v7 &= (unsigned char)v25[7] ^ 1;
      v43 = CONCAT71(v43._1_7_,v7);
      v32 &= *(unsigned char *)((long)v25 + 0x37) ^ 1;
      v42 = CONCAT17(v32,CONCAT34((undefined3)(CONCAT16(v28,(undefined6)v42) >> 0x20),(unsigned int)v42));
      v25 = (unsigned long *)v25[8];
      v29 += 1;
      v14 = v42;
    } while (v25);
    if (v35) { // branch-flip
      v8 = (unsigned char)dat_1e864;
      if (dat_1e028 != 0x80) { // branch-flip
        if (dat_1e028 == dat_1e864) {
label_8667:
          v21._0_2_ = (unsigned short)v8;
          v12 = sub_11430(&v21);
          error(0,0,dcgettext(NULL,"field separator %s is treated as a group separator in numbers",5),v12);
          goto label_8580;
        }
        v35 = '\0';
label_843e:
        if (dat_1e868 != dat_1e028) {
          if (dat_1e028 != 0x2d) { // branch-flip
            if ((dat_1e028 != 0x2b) || (!v36)) goto label_8466;
            v21 = CONCAT22(v21._2_2_,0x2b);
            v12 = sub_11430(&v21);
            v9 = "field separator %s is treated as a plus sign in numbers";
          }
          else {
            v21 = CONCAT22(v21._2_2_,0x2d);
            v12 = sub_11430(&v21);
            v9 = "field separator %s is treated as a minus sign in numbers";
          }
          error(0,0,dcgettext(NULL,v9,5),v12);
label_8466:
          if ((!v33) && (!v34)) goto label_8298;
          if (!v35) goto label_8483;
          goto label_8261;
        }
      }
      else {
        if (dat_1e864 == 0x80) {
          v35 = '\0';
          goto label_86b4;
        }
        __ctype_b_loc();
        v7 = sub_53e0((int)(char)v8);
        if (*(unsigned char *)(*v16 + (unsigned long)v7 * 2) & 1) goto label_8667;
        v35 = '\0';
label_81f3:
        __ctype_b_loc();
        v8 = sub_53e0((int)dat_1e868);
        if (!(*(unsigned char *)(*v17 + (unsigned long)v8 * 2) & 1)) goto label_8466;
      }
      v21 = CONCAT31((undefined3)(v21 >> 8),dat_1e868) & 0xffff00ff;
      v12 = sub_11430(&v21);
      error(0,0,dcgettext(NULL,"field separator %s is treated as a decimal point in numbers",5),v12);
    }
    else {
      if (v36) {
label_8580:
        if (dat_1e028 == 0x80) {
label_86b4:
          if (dat_1e864 != 0x80) goto label_81f3;
          goto label_8466;
        }
        goto label_843e;
      }
      if ((!v33) && (!v34)) goto label_8298;
label_8483:
      v9 = "";
      v21 = CONCAT31((undefined3)(v21 >> 8),dat_1e868) & 0xffff00ff;
      v12 = sub_11430(&v21);
      if (dat_1e868 != dat_1e028)
        v9 = dcgettext(NULL,"note ",5);
      error(0,0,dcgettext(NULL,"%snumbers use %s as a decimal point in this locale",5),v9,v12);
    }
label_8261:
    if ((v33) && (dat_1e862))
      error(0,0,dcgettext(NULL,"the multi-byte number group separator in this locale is not supported",5));
  }
label_8298:
  v6 = sub_5680(&v23);
  v5 = v42._7_1_;
  if (v6) { // branch-flip
    if (!v42._7_1_)
      return;
    if (dat_1e41a) { // branch-flip
      if (!dat_1e410)
        return;
      sub_56d0();
      sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
      error(0,0,(char *)dcngettext(0),&v21);
      v42 = CONCAT17(1,(undefined7)v42);
    }
    else {
      if ((!dat_1e419) || (!dat_1e410)) goto label_83b5;
      sub_56d0();
      sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
      error(0,0,(char *)dcngettext(0),&v21);
      v42 = CONCAT17(v5,(undefined7)v42);
    }
  }
  else {
    if ((!dat_1e41a) && (!dat_1e419))
      v42 &= 0xffffffffffffff;
    sub_56d0();
    sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
    error(0,0,(char *)dcngettext(0),&v21);
    v42 = CONCAT17(v5,(undefined7)v42);
    if (!v5)
      return;
  }
  if (dat_1e41a)
    return;
label_83b5:
  if ((!dat_1e419) && (dat_1e410)) {
    error(0,0,dcgettext(NULL,"option \'-r\' only applies to last-resort comparison",5)); // tail-call
    return;
  }
}


// Function: sub_9400 @ 0x9400
unsigned long sub_9400(unsigned long *a0)
{
  unsigned long v1; // rax
  long v2;
  unsigned long v3; // stack - 0x18
  
  v2 = dat_1e410;
  do {
    sub_9230(*a0,a0[1],v2);
    if (!v2)
      return v3;
    v2 = *(long *)(v2 + 0x40);
  } while (v2);
  if ((!dat_1e419) && (!dat_1e41a)) {
    v1 = sub_9230(*a0,a0[1],0); // tail-call
    return v1;
  }
  return v3;
}


// Function: sub_9460 @ 0x9460
void sub_9460(unsigned long *a0,void *a1,long a2)
{
  char *v1;
  char v2;
  unsigned long v3;
  char *v4;
  int v5;
  
  v4 = (char *)*a0;
  v3 = a0[1];
  v1 = &v4[v3];
  if ((!a2) && (dat_1e400)) {
    do {
      if (v1 <= v4) {
        sub_9400(a0); // tail-call
        return;
      }
      v2 = *v4;
      v4 = &v4[1];
      v5 = 0x3e;
      if ((v2 != '\t') && (v5 = 10, v1 != v4))
        v5 = (int)v2;
    } while (fputc_unlocked(v5,a1) != -1);
    sub_5e10(dcgettext(NULL,"write failed",5),0); // no-return
  }
  v1[-1] = dat_1e038;
  if (fwrite_unlocked(v4,1,v3,a1) != v3)
    sub_5e10(dcgettext(NULL,"write failed",5),a2); // no-return
  v1[-1] = '\0';
}


// Function: sub_9550 @ 0x9550
unsigned long sub_9550(unsigned long *a0,long *a1)
{
  long v1;
  char v10; // dl
  char v11;
  char v12; // dl
  int v13; // edx
  unsigned int v14; // edx
  long v15; // rdx
  long v16;
  char v17 [4007];
  long *v18;
  long v19;
  unsigned char v2; // al
  long v20;
  unsigned long v21; // r12
  unsigned long v22; // r13
  char *v23;
  bool v24; // cf
  char v25; // stack - 0x1002
  char v26; // stack - 0x1001
  char *v27; // stack - 0xff8
  unsigned int v3; // eax
  int v4; // eax
  char *v5;
  char *v6;
  char *v7;
  char *v8;
  unsigned long v9;
  
  v7 = (char *)a0[2];
  v8 = (char *)a1[2];
  v5 = (char *)a0[3];
  v6 = (char *)a1[3];
  v18 = dat_1e410;
label_95a0:
  v19 = v18[5];
  v1 = v18[4];
  if (v5 <= v7)
    v5 = v7;
  if (v6 <= v8)
    v6 = v8;
  v22 = (long)v5 - (long)v7;
  v21 = (long)v6 - (long)v8;
  if ((((dat_1e861) || (sub_5660(v18))) || (v18[6] & 0xff0000ff000000U)) || ((char)v18[7])) {
    if (v19 || v1) { // branch-flip
      v27 = NULL;
      if (0xfa1 <= v22 + 2 + v21) { // branch-flip
        v6 = (char *)sub_12eb0();
        v27 = v6;
        v5 = &v6[v22 + 1];
        if (!v22) goto label_9abb;
label_9986:
        v23 = &v7[v22];
        v22 = 0;
        v20 = v19;
        do {
          v11 = *v7;
          if (v1) { // branch-flip
            if (!*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
              v11 = v10;
              if (v20) goto label_99c8;
              goto label_99ab;
            }
          }
          else {
label_99c8:
            v11 = *(char *)(v20 + (unsigned long)(unsigned char)sub_53e0());
label_99ab:
            v6[v22] = v11;
            v22 += 1;
          }
          v7 = &v7[1];
        } while (v23 != v7);
        v6[v22] = '\0';
        v7 = v5;
        if (v21) goto label_99f0;
label_9ad0:
        v8 = v5;
      }
      else {
        v5 = &v17[v22];
        v6 = &v16;
        if (v22) goto label_9986;
label_9abb:
        *v6 = '\0';
        v7 = v5;
        if (!v21) goto label_9ad0;
label_99f0:
        v5 = &v8[v21];
        v21 = 0;
        v20 = v19;
        do {
          v11 = *v8;
          if (v1) { // branch-flip
            if (!*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
              v11 = v12;
              if (v20) goto label_9a30;
              goto label_9a13;
            }
          }
          else {
label_9a30:
            v11 = *(char *)(v20 + (unsigned long)(unsigned char)sub_53e0());
label_9a13:
            v7[v21] = v11;
            v21 += 1;
          }
          v8 = &v8[1];
        } while (v5 != v8);
        v5 = &v7[v21];
        v8 = v7;
      }
      *v5 = '\0';
    }
    else {
      v25 = *v5;
      *v5 = '\0';
      v26 = *v6;
      *v6 = '\0';
      v6 = v7;
    }
    if (*(char *)((long)v18 + 0x32)) { // branch-flip
      v9 = sub_64d0(v6,v8);
label_977e:
      if (!v19 && !v1) {
label_978a:
        v6[v22] = v25;
        v8[v21] = v26;
        goto label_979c;
      }
    }
    else {
      if (*(char *)((long)v18 + 0x34)) {
        v9 = sub_63d0(v6,v8);
        goto label_977e;
      }
      if (!*(char *)((long)v18 + 0x35)) {
        if (*(char *)((long)v18 + 0x36)) { // branch-flip
          v4 = sub_5570(v6,0);
          v9 = (unsigned long)(unsigned int)(v4 - sub_5570(v8,0));
        }
        else if (*(char *)((long)v18 + 0x33)) // branch-flip
          v9 = sub_6680(v6,v22,v8,v21);
        else if ((char)v18[7]) // branch-flip
          v9 = sub_cd70(v6,v22,v8,v21);
        else if (v22) { // branch-flip
          v9 = 1;
          if (v21)
            v9 = sub_133e0(v6,v22 + 1,v8,v21 + 1);
        }
        else {
          v9 = (unsigned long)-(unsigned int)(v21 != 0);
        }
        goto label_977e;
      }
      v9 = sub_6530(v6,v8);
      if (!v19 && !v1) goto label_978a;
    }
    v4 = (int)v9;
    free(v27);
    v9 &= 0xffffffff;
    goto label_9874;
  }
  if (v1) {
    if (!v19) {
      do {
        if ((v5 <= v7) || (v11 = *v7, !*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11)))) {
          if (v6 <= v8) goto label_9c50;
          while (v11 = *v8, *(char *)(v1 + (sub_53e0((int)v11) & 0xff))) {
            v8 = &v8[1];
            if (v6 == v8) goto label_9c50;
          }
          if ((v5 <= v7) || (v6 <= v8)) goto label_9c50;
          v11 = *v7;
          v3 = (unsigned int)(unsigned char)sub_53e0((int)v11) - v13;
          v9 = (unsigned long)v3;
          if (v3) goto label_9880;
          v8 = &v8[1];
        }
        v7 = &v7[1];
      } while( true );
    }
    do {
      if ((v5 <= v7) || (v11 = *v7, !*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11)))) {
        if (v6 <= v8) goto label_9bc0;
        while (v11 = *v8, *(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
          v8 = &v8[1];
          if (v6 == v8) goto label_9bc0;
        }
        if ((v5 <= v7) || (v6 <= v8)) goto label_9bc0;
        v11 = *v7;
        v3 = sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
        v2 = sub_53e0((int)*(char *)(v19 + v15));
        v3 = (v3 & 0xff) - (unsigned int)v2;
        if (v3) {
          v11 = *(char *)((long)v18 + 0x37);
          v9 = (unsigned long)v3;
          goto label_9be0;
        }
        v8 = &v8[1];
      }
      v7 = &v7[1];
    } while( true );
  }
  if (!v22) {
    v9 = (unsigned long)-(unsigned int)(v21 != 0);
    goto label_979c;
  }
  if (!v21) {
label_9baf:
    v9 = 1;
    goto label_9880;
  }
  if (v19) { // branch-flip
    while ((v7 < v5 && (v8 < v6))) {
      v11 = *v7;
      v23 = &v8[1];
      v7 = &v7[1];
      sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
      v11 = *v8;
      v3 = sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
      v3 = (v14 & 0xff) - (v3 & 0xff);
      v9 = (unsigned long)v3;
      v8 = v23;
      if (v3) goto label_9880;
    }
  }
  else {
    v9 = v21;
    if (v22 <= v21)
      v9 = v22;
    v4 = memcmp(v7,v8,v9);
    v9 = CONCAT44(dat_4,v4);
    if (v4) goto label_9880;
  }
  if (v22 < v21) {
    v9 = 0xffffffff;
    goto label_9880;
  }
  if (v22 != v21) goto label_9baf;
  goto label_97a4;
label_9c50:
  v24 = v7 < v5;
  goto label_9bc3;
label_9bc0:
  v24 = v7 < v5;
label_9bc3:
  v9 = (unsigned long)((unsigned int)v24 - (unsigned int)(v8 < v6));
label_979c:
  v4 = (int)v9;
label_9874:
  if (v4) {
label_9880:
    v11 = *(char *)((long)v18 + 0x37);
label_9be0:
    if (v11)
      return (unsigned long)(unsigned int)-(int)v9;
    return v9;
  }
label_97a4:
  v18 = (long *)v18[8];
  if (!v18)
    return 0;
  v7 = (char *)*a0;
  v19 = a0[1];
  if (v18[2] != -1) { // branch-flip
    v5 = (char *)sub_8f70(v7,v19,v18);
    v6 = (char *)sub_8f70(*a1,a1[1],v18);
  }
  else {
    v5 = &v7[v19 + -1];
    v6 = (char *)(*a1 + -1 + a1[1]);
  }
  if (*v18 != -1) { // branch-flip
    v7 = (char *)sub_90d0(v7,v19,v18);
    v8 = (char *)sub_90d0(*a1,a1[1],v18);
  }
  else {
    v8 = (char *)*a1;
    if ((char)v18[6]) {
      if (v7 < v5) {
        do {
          v11 = *v7;
          v19 = 0x1e740;
          if (!*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11))) break;
          v7 = &v7[1];
        } while (v5 != v7);
      }
      if (v8 < v6) {
        do {
          v11 = *v8;
          v19 = 0x1e740;
          if (!*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11))) goto label_95a0;
          v8 = &v8[1];
        } while (v6 != v8);
      }
    }
  }
  goto label_95a0;
}


// Function: sub_9c70 @ 0x9c70
unsigned long sub_9c70(unsigned long *a0,unsigned long *a1)
{
  long v1;
  long v2;
  int v3; // eax
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r15
  
  if (dat_1e410) {
    v4 = sub_9550();
    if ((int)v4)
      return v4;
    if (dat_1e419)
      return v4;
    if (dat_1e41a)
      return v4;
  }
  v1 = a0[1];
  v2 = a1[1];
  v4 = v2 - 1;
  v6 = v1 - 1;
  if (v6) { // branch-flip
    v5 = 1;
    if (v4) {
      if (dat_1e861) // branch-flip
        v5 = sub_133e0((void *)*a0,v1,(void *)*a1,v2);
      else {
        v5 = v4;
        if (v6 <= v4)
          v5 = v6;
        v3 = memcmp((void *)*a0,(void *)*a1,v5);
        v5 = CONCAT44(dat_4,v3);
        if ((!v3) && (v5 = 0xffffffff, v4 <= v6))
          v5 = (unsigned long)(v1 != v2);
      }
    }
  }
  else {
    v5 = (unsigned long)-(unsigned int)(v4 != 0);
  }
  if (dat_1e41b)
    v5 = (unsigned long)(unsigned int)-(int)v5;
  return v5;
}


// Function: sub_9d40 @ 0x9d40
void sub_9d40(unsigned long *a0,unsigned long a1,long a2)
{
  unsigned long v1;
  long v10; // stack - 0x40
  unsigned long v2;
  unsigned long v3;
  long v4;
  unsigned long *v5;
  unsigned long *v6; // rbp
  unsigned long v7; // r12
  long v8; // r13
  unsigned long *v9;
  
  v7 = a1 >> 1;
  v10 = a1 - v7;
  v5 = &(&a0[v7 * -4])[-4];
  v4 = a2 + -0x20;
  v9 = &a0[v7 * -4];
  while( true ) {
    while( true ) {
      v8 = v4;
      v6 = v5;
      v5 = &a0[-4];
      if ((int)sub_9c70(v8,v6) <= 0) break;
      v1 = v9[-3];
      v10 -= 1;
      *v5 = v9[-4];
      a0[-3] = v1;
      v1 = v9[-1];
      a0[-2] = v9[-2];
      a0[-1] = v1;
      if (!v10) {
        v4 = -0x20;
        do {
          v1 = ((unsigned long *)(a2 + v4))[1];
          v9 = (unsigned long *)(a2 + 0x10 + v4);
          v2 = *v9;
          v3 = v9[1];
          *(unsigned long *)((long)v5 + v4) = *(unsigned long *)(a2 + v4);
          ((unsigned long *)((long)v5 + v4))[1] = v1;
          v9 = (unsigned long *)((long)a0 + v4 + -0x10);
          *v9 = v2;
          v9[1] = v3;
          v4 -= 0x20;
          v7 -= 1;
        } while (v7);
        return;
      }
      a0 = v5;
      v5 = &v6[-4];
      v4 = v8;
      v9 = v6;
    }
    v1 = *(unsigned long *)(a2 + -0x18);
    v2 = *(unsigned long *)(a2 + -0x10);
    v3 = *(unsigned long *)(a2 + -8);
    *v5 = *(unsigned long *)(a2 + -0x20);
    a0[-3] = v1;
    a0[-2] = v2;
    a0[-1] = v3;
    v7 -= 1;
    if (!v7) break;
    a0 = v5;
    v5 = v6;
    v4 = v8 + -0x20;
    a2 = v8;
  }
}


// Function: sub_9e20 @ 0x9e20
void sub_9e20(long a0,unsigned long a1,long a2,char a3) // return-dupe
{
  unsigned long v1;
  unsigned long *v10; // rax
  unsigned long *v11; // rdx
  long v12;
  unsigned long v13; // r14
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  int v9; // eax
  
  if (a1 != 2) {
    v13 = a1 >> 1;
    v12 = a0 + v13 * -0x20;
    if (a3) { // branch-flip
      sub_9e20(v12,a1 - v13,v13 * -0x20 + a2,1);
      v12 = a0;
      if (4 <= a1)
        sub_9e20(a0,v13,a2,0);
    }
    else {
      sub_9e20(v12,a1 - v13,a2,0);
      v12 = a2; // branch-flip
      if (4 <= a1) {
        sub_9e20(a0,v13,a2,1);
        a2 = a0;
      }
      else {
        v1 = *(unsigned long *)(a0 + -0x18);
        v2 = *(unsigned long *)(a0 + -0x10);
        v3 = *(unsigned long *)(a0 + -8);
        *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
        *(unsigned long *)(a2 + -0x18) = v1;
        *(unsigned long *)(a2 + -0x10) = v2;
        *(unsigned long *)(a2 + -8) = v3;
        a2 = a0;
      }
    }
    sub_9d40(a2,a1,v12); // tail-call
    return;
  }
  v9 = sub_9c70(a0 + -0x20,a0 + -0x40);
  if (a3) {
    v10 = (unsigned long *)((long)(int)((unsigned int)(0 < v9) - 2) * 0x20 + a0);
    v11 = (unsigned long *)((long)(int)~(unsigned int)(0 < v9) * 0x20 + a0);
    v1 = v10[1];
    v2 = v10[2];
    v3 = v10[3];
    v4 = *v11;
    v5 = v11[1];
    v6 = v11[2];
    v7 = v11[3];
    *(unsigned long *)(a2 + -0x40) = *v10;
    *(unsigned long *)(a2 + -0x38) = v1;
    *(unsigned long *)(a2 + -0x20) = v4;
    *(unsigned long *)(a2 + -0x18) = v5;
    *(unsigned long *)(a2 + -0x10) = v6;
    *(unsigned long *)(a2 + -8) = v7;
    *(unsigned long *)(a2 + -0x30) = v2;
    *(unsigned long *)(a2 + -0x28) = v3;
    return;
  }
  if (v9 <= 0)
    return;
  v1 = *(unsigned long *)(a0 + -0x20);
  v2 = *(unsigned long *)(a0 + -0x18);
  v3 = *(unsigned long *)(a0 + -0x10);
  v4 = *(unsigned long *)(a0 + -8);
  v5 = *(unsigned long *)(a0 + -0x40);
  v6 = *(unsigned long *)(a0 + -0x38);
  v7 = *(unsigned long *)(a0 + -0x30);
  v8 = *(unsigned long *)(a0 + -0x28);
  *(unsigned long *)(a2 + -0x20) = v1;
  *(unsigned long *)(a2 + -0x18) = v2;
  *(unsigned long *)(a2 + -0x10) = v3;
  *(unsigned long *)(a2 + -8) = v4;
  *(unsigned long *)(a0 + -0x20) = v5;
  *(unsigned long *)(a0 + -0x18) = v6;
  *(unsigned long *)(a0 + -0x10) = v7;
  *(unsigned long *)(a0 + -8) = v8;
  *(unsigned long *)(a0 + -0x40) = v1;
  *(unsigned long *)(a0 + -0x38) = v2;
  *(unsigned long *)(a0 + -0x30) = v3;
  *(unsigned long *)(a0 + -0x28) = v4;
}


// Function: sub_9fb0 @ 0x9fb0
void sub_9fb0(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_1e419) {
    if (dat_1e840) {
      if (!sub_9c70(a0,0x1e840))
        return;
    }
    dat_1e840 = *a0;
    dat_1e848 = a0[1];
    dat_1e850 = a0[2];
    dat_1e858 = a0[3];
  }
  sub_9460(a0,a1,a2); // tail-call
}


// Function: sub_a020 @ 0xa020
void sub_a020(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long *v1;
  long v10;
  long v11;
  long v12;
  long v13; // r8
  unsigned long *v14;
  long v15;
  unsigned long *v16; // rdx
  unsigned long *v17;
  unsigned long *v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned long *v6;
  unsigned long *v7;
  long v8;
  unsigned long *v9;
  
  v1 = (unsigned long *)*a0;
  v2 = (unsigned long *)a0[1];
  v7 = (unsigned long *)a0[2];
  v10 = (a1 >> ((char)*(unsigned int *)&a0[10] * '\x02' + 2U & 0x3f)) + 1;
  v6 = v1;
  if (*(unsigned int *)&a0[10] < 2) {
    v12 = 0;
    v11 = 0;
    v9 = v2;
    v15 = v10;
    if (v1 != v7) { // branch-flip
      while ((unsigned long *)a0[3] != v9) {
        v10 = v15 + -1;
        if (!v15) {
          v15 = -1;
          break;
        }
        if (1 <= (int)sub_9c70(&v6[-4],&v9[-4])) { // branch-flip
          v12 = a0[1];
          a0[1] = v12 + -0x20;
          sub_9fb0(v12 + -0x20,a2,a3);
          v6 = (unsigned long *)*a0;
          v9 = (unsigned long *)a0[1];
          if (v6 == (unsigned long *)a0[2]) goto label_a1d9;
        }
        else {
          v12 = *a0;
          *a0 = v12 + -0x20;
          sub_9fb0(v12 + -0x20,a2,a3);
          v6 = (unsigned long *)*a0;
          v9 = (unsigned long *)a0[1];
          if (v6 == (unsigned long *)a0[2]) {
label_a1d9:
            v11 = (long)v1 - (long)v6 >> 5;
            v12 = (long)v2 - (long)v9 >> 5;
            goto label_a1f2;
          }
        }
        v15 = v10;
      }
      v12 = (long)v2 - (long)v9 >> 5;
      if (a0[6] == v12) {
        if (((unsigned long *)a0[2] != v6) && (v15)) {
          do {
            v15 -= 1;
            *a0 = (long)&v6[-4];
            sub_9fb0(&v6[-4],a2,a3);
            v6 = (unsigned long *)*a0;
            if (v6 == (unsigned long *)a0[2]) break;
          } while (v15);
          v8 = a0[5];
          v12 = a0[6];
          v15 = (long)v2 - a0[1] >> 5;
        }
        else {
          v8 = a0[5];
          v15 = v12;
        }
        goto label_a13a;
      }
      v8 = a0[5];
      v11 = (long)v1 - (long)v6 >> 5;
    }
    else {
label_a1f2:
      v8 = a0[5];
      v15 = v10;
      if (a0[6] == v12) {
        v15 = v12;
        goto label_a13a;
      }
    }
    if (((v8 != v11) || (v9 == (unsigned long *)a0[3])) || (!v15)) {
      v12 = a0[6];
      v15 = (long)v2 - a0[1] >> 5;
    }
    else {
      do {
        v15 -= 1;
        a0[1] = (long)&v9[-4];
        sub_9fb0(&v9[-4],a2,a3);
        v9 = (unsigned long *)a0[1];
        if (v9 == (unsigned long *)a0[3]) break;
      } while (v15);
      v6 = (unsigned long *)*a0;
      v8 = a0[5];
      v12 = a0[6];
      v15 = (long)v2 - (long)v9 >> 5;
    }
    goto label_a13a;
  }
  v14 = (unsigned long *)a0[4];
  v15 = 0;
  v13 = 0;
  v17 = (unsigned long *)*v14;
  v5 = v1;
  v9 = v2;
  v11 = v10;
  if (v1 != v7) { // branch-flip
    while ((unsigned long *)a0[3] != v9) {
      v10 = v11 + -1;
      if (!v11) {
        v14 = (unsigned long *)a0[4];
        v11 = -1;
        v6 = (unsigned long *)*a0;
        goto label_a2dd;
      }
      v16 = &v17[-4];
      if (1 <= (int)sub_9c70(&v5[-4],&v9[-4])) { // branch-flip
        v12 = a0[1];
        v7 = (unsigned long *)a0[2];
        v3 = *(unsigned long *)(v12 + -0x20);
        v4 = *(unsigned long *)(v12 + -0x18);
        v9 = (unsigned long *)(v12 + -0x20);
        a0[1] = (long)v9;
        *v16 = v3;
        v17[-3] = v4;
        v3 = *(unsigned long *)(v12 + -8);
        v5 = (unsigned long *)*a0;
        v17[-2] = *(unsigned long *)(v12 + -0x10);
        v17[-1] = v3;
        if (v7 == v5) goto label_a104;
      }
      else {
        v12 = *a0;
        v7 = (unsigned long *)a0[2];
        v3 = *(unsigned long *)(v12 + -0x20);
        v4 = *(unsigned long *)(v12 + -0x18);
        v5 = (unsigned long *)(v12 + -0x20);
        *a0 = (long)v5;
        *v16 = v3;
        v17[-3] = v4;
        v3 = *(unsigned long *)(v12 + -8);
        v9 = (unsigned long *)a0[1];
        v17[-2] = *(unsigned long *)(v12 + -0x10);
        v17[-1] = v3;
        if (v7 == v5) {
label_a104:
          v14 = (unsigned long *)a0[4];
          v13 = (long)v1 - (long)v5 >> 5;
          v15 = (long)v2 - (long)v9 >> 5;
          v6 = v5;
          v17 = v16;
          goto label_a121;
        }
      }
      v11 = v10;
      v17 = v16;
    }
    v14 = (unsigned long *)a0[4];
    v6 = (unsigned long *)*a0;
label_a2dd:
    v15 = (long)v2 - (long)v9 >> 5;
    v13 = (long)v1 - (long)v5 >> 5;
    v12 = a0[6];
    v8 = a0[5];
    if (v12 != v15) goto label_a12e;
    if ((v7 != v6) && (v10 = v11 + -1, v9 = v17, v2 = v6, v11)) {
      do {
        v6 = &v2[-4];
        v3 = *v6;
        v4 = v2[-3];
        v17 = &v9[-4];
        *a0 = (long)v6;
        *v17 = v3;
        v9[-3] = v4;
        v3 = v2[-1];
        v9[-2] = v2[-2];
        v9[-1] = v3;
        if (v6 == v7) goto label_a137;
        v10 -= 1;
        v9 = v17;
        v2 = v6;
      } while (v10 != -1);
    }
  }
  else {
label_a121:
    v12 = a0[6];
    v8 = a0[5];
    v11 = v10;
    if (v15 != v12) {
label_a12e:
      if (((v13 == v8) && (v7 = (unsigned long *)a0[3], v7 != v9)) && (v10 = v11 + -1, v5 = v17, v11)) {
        do {
          v16 = &v9[-4];
          v3 = *v16;
          v4 = v9[-3];
          v17 = &v5[-4];
          a0[1] = (long)v16;
          *v17 = v3;
          v5[-3] = v4;
          v3 = v9[-1];
          v5[-2] = v9[-2];
          v5[-1] = v3;
          if (v16 == v7) {
            v15 = (long)v2 - (long)v16 >> 5;
            goto label_a137;
          }
          v10 -= 1;
          v5 = v17;
          v9 = v16;
        } while (v10 != -1);
        v15 = (long)v2 - (long)v16 >> 5;
      }
    }
  }
label_a137:
  *v14 = v17;
label_a13a:
  a0[6] = v12 - v15;
  a0[5] = v8 - ((long)v1 - (long)v6 >> 5);
  return;
}


// Function: sub_a460 @ 0xa460
void sub_a460(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  
  while( true ) {
    v1 = sub_7c70(a0);
    if (!*(int *)(v1 + 0x50)) break;
    sub_a020(v1,a1,a2,a3);
    sub_7bc0(a0,v1);
    sub_7c10(a0,v1);
    sub_7c00(v1);
  }
  sub_7c00(v1);
  sub_7b80(a0,v1); // tail-call
}


// Function: sub_a4e0 @ 0xa4e0
void sub_a4e0(long a0,unsigned long a1,long a2,long *a3,unsigned long a4,unsigned long a5,unsigned long a6)
{
  unsigned long v1;
  long v10; // stack - 0x68
  long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  unsigned long v14; // stack - 0x48
  long v2; // stack - 0x78
  unsigned long v3; // stack - 0x80
  unsigned long v4;
  long v5; // r10
  unsigned long v6; // r11
  long v7; // r11
  unsigned long v8;
  unsigned long v9; // stack - 0x70
  
  v1 = a6;
  v6 = a1 >> 1;
  v4 = a3[5];
  v8 = a3[6];
  v11 = a3[8];
  v14 = a6;
  v2 = a0;
  v9 = v6;
  v10 = a2;
  v12 = a4;
  v13 = a5;
  if ((0x20000 <= v4 + v8) && (2 <= a1)) {
    if (!pthread_create(&v3,0,sub_a710,&v2)) {
      sub_a4e0(a0 + a3[5] * -0x20,a1 - v6,a2,a3[9],a4,a5,v1);
      pthread_join(v3,0);
      return;
    }
    v4 = a3[5];
    v8 = a3[6];
  }
  v7 = a0 + a2 * -0x20;
  v5 = a0 + v4 * -0x20;
  if (2 <= v8)
    sub_9e20(v5,v8,v7 + (v4 >> 1) * -0x20,0);
  if (2 <= v4)
    sub_9e20(a0,v4,v7,0);
  *a3 = a0;
  a3[1] = v5;
  a3[2] = v5;
  a3[3] = a0 + v4 * -0x20 + v8 * -0x20;
  sub_7b80(a4,a3);
  sub_a460(a4,a2,a5,v1);
}


// Function: sub_a710 @ 0xa710
unsigned long sub_a710(unsigned long *a0)
{
  sub_a4e0(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}


// Function: sub_a750 @ 0xa750
unsigned long sub_a750(long *a0,void *a1,unsigned long a2) // return-dupe
{
  char v1;
  unsigned long v10;
  long v11;
  long *v12;
  long v13; // rdx
  long v14;
  unsigned long v15; // rdx
  long *v16;
  char *v17;
  char *v18;
  unsigned long v19;
  long v2;
  char *v20; // stack - 0x88
  char v3;
  long *v4;
  long v5; // rax
  unsigned long v6;
  unsigned long v7; // rax
  char *v8; // rax
  long v9; // rax
  
  v4 = dat_1e410;
  v3 = dat_1e038;
  v11 = dat_1e030;
  v2 = a0[5];
  if ((char)a0[6])
    return 0;
  v6 = a0[1];
  v10 = a0[4];
  if (v6 != v10) { // branch-flip
    memmove((void *)*a0,(void *)(*a0 + (v6 - v10)),v10);
    v6 = a0[4];
    a0[2] = 0;
    v14 = 0;
    a0[1] = v6;
  }
  else {
    v14 = a0[2];
  }
  v19 = v11 - 0x22;
  v17 = (char *)*a0;
  v10 = a0[3];
  do {
    v18 = &v17[v6];
    sub_a740(v17,v10,v14);
    v16 = (long *)(v5 + v13 * -0x20);
    v10 = (v5 - v2 * v13) - (long)v18;
    if (v13)
      v17 = (char *)(v16[1] + *v16);
    do {
      if (v10 <= v2 + 1U) {
        v20 = v18;
        break;
      }
      v6 = (v10 - 1) / (v2 + 1U);
      v7 = fread_unlocked(v18,1,v6,a1);
      v10 -= v7;
      v20 = &v18[v7];
      if (v6 != v7) {
        if (ferror_unlocked(a1))
          sub_5e10(dcgettext(NULL,"read failed",5),a2); // no-return
        if (feof_unlocked(a1)) {
          *(char *)&a0[6] = 1;
          if ((char *)*a0 == v20)
            return 0;
          if ((v17 != v20) && (v20[-1] != v3)) {
            *v20 = v3;
            v20 = &v20[1];
          }
        }
      }
      while (v8 = memchr(v18,(int)v3,(long)v20 - (long)v18), v8) {
        v18 = &v8[1];
        v12 = &v16[-4];
        *v8 = 0;
        *v12 = (long)v17;
        v6 = (long)v18 - (long)v17;
        v16[-3] = v6;
        if (v19 < v6)
          v19 = v6;
        v10 -= v2;
        if (v4) {
          if (v4[2] != -1)
            v8 = (char *)sub_8f70(v17,v6,v4);
          v11 = *v4;
          v16[-1] = (long)v8;
          if (v11 != -1) // branch-flip
            v16[-2] = sub_90d0(v17,v6,v4);
          else {
            if ((char)v4[6]) {
              while( true ) {
                v1 = *v17;
                v11 = 0x1e740;
                if (!*(char *)(v11 + (unsigned long)(unsigned char)sub_53e0((int)v1))) break;
                v17 = &v17[1];
              }
            }
            v16[-2] = (long)v17;
          }
        }
        v17 = v18;
        v16 = v12;
      }
      v18 = v20;
    } while (!(char)a0[6]);
    a0[1] = (long)v20 - *a0;
    v12 = a0;
    sub_a740(*a0,a0[3]);
    v11 = v9 - (long)v16 >> 5;
    v12[2] = v11;
    if (v11) {
      v12[4] = (long)v20 - (long)v17;
      dat_1e030 = v19 + 0x22;
      return 1;
    }
    v17 = (char *)sub_12ff0();
    v10 = v15 & 0xffffffffffffffe0;
    v6 = a0[1];
    v14 = a0[2];
    *a0 = (long)v17;
    a0[3] = v10;
  } while( true );
}


// Function: sub_aac0 @ 0xaac0
unsigned long sub_aac0(unsigned long a0,char a1)
{
  long v1;
  long *v10; // rdx
  unsigned long v11;
  unsigned int v12; // ebx
  void *v13 [2]; // stack - 0x98
  void *v14; // stack - 0xb8
  char v15 [24];
  unsigned long v16;
  unsigned long v17; // r12
  long *v18; // stack - 0x100
  long v19; // stack - 0xd8
  unsigned long v2;
  long v20; // stack - 0xb0
  long v21; // stack - 0xa8
  long v22; // stack - 0xa0
  long v23; // stack - 0x88
  unsigned long v24; // stack - 0x80
  long v3;
  long *v4;
  char v5; // al
  unsigned long v6; // rax
  long *v7; // rax
  void *v8;
  unsigned long v9; // rax
  
  v6 = sub_6010(a0,"r");
  v1 = dat_1e410;
  v12 = (unsigned int)dat_1e419;
  v11 = dat_1e438;
  if (dat_1e438 < dat_1e030)
    v11 = dat_1e030;
  v16 = 0;
  sub_6240(v13,0x20,v11);
  v14 = NULL;
  v19 = 0;
  do {
    v5 = sub_a750(v13,v6,a0);
    v8 = v13[0];
    if (!v5) {
      v17 = 1;
label_acaa:
      sub_6040(v6,a0);
      free(v8);
      free(v14);
      return v17;
    }
    v7 = (long *)sub_a740(v13[0],v24);
    v3 = v23;
    v18 = v7;
    v10 = &v7[v23 * -4];
    if ((v16) && ((int)(v12 ^ 1) <= (int)sub_9c70(&v14,&v7[-4]))) {
label_aca0:
      v17 = 0;
      if (a1 == 'c') {
        v9 = sub_e520(((long)v18 - (long)&v7[-4] >> 5) + v19,v15);
        v2 = dat_1e888;
        __fprintf_chk(stderr,1,dcgettext(NULL,"%s: %s:%s: disorder: ",5),v2,a0,v9);
        sub_9460(&v7[-4],stderr,dcgettext(NULL,"standard error",5));
      }
      goto label_acaa;
    }
    v4 = &v18[-4];
    while (v7 = v4, v10 < v7) {
      v4 = &v7[-4];
      if ((int)(v12 ^ 1) <= (int)sub_9c70(v7,&v7[-4])) goto label_aca0;
    }
    v11 = v7[1];
    v19 += v3;
    if (v16 < v11) {
      do {
        v16 *= 2;
        if (!v16) {
          v16 = v11;
          break;
        }
      } while (v16 < v11);
      free(v14);
      v14 = (void *)sub_12eb0(v16);
      v11 = v7[1];
    }
    v8 = memcpy(v14,(void *)*v7,v11);
    v20 = v7[1];
    if (v1) {
      v21 = (long)v8 + (v7[2] - *v7);
      v22 = (long)v8 + (v7[3] - *v7);
    }
  } while( true );
}


// Function: sub_ad90 @ 0xad90
void sub_ad90(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  unsigned long v1;
  unsigned long v10;
  long v11;
  unsigned long v12;
  int v13; // eax
  void *v14; // rax
  unsigned long *v15; // rax
  long v16;
  long v17; // rax
  unsigned long *v18;
  void *v19;
  unsigned long v2;
  unsigned long *v20;
  long v21; // rdx
  unsigned long *v22;
  unsigned long *v23; // rdx
  unsigned long v24;
  void *v25; // stack - 0x68
  unsigned long *v26;
  unsigned long v27;
  unsigned long v28; // r14
  unsigned long v29;
  long *v3;
  bool v30; // zf
  void *v31; // stack - 0xd0
  unsigned long v32; // stack - 0xc8
  void *v33; // stack - 0xc0
  void *v34; // stack - 0xb8
  void **v35; // stack - 0xb0
  unsigned long v36; // stack - 0x88
  unsigned long v37; // stack - 0x80
  unsigned long v38; // stack - 0x60
  long v39; // stack - 0x58
  unsigned long v4;
  long v40; // stack - 0x50
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  v34 = (void *)sub_12fd0(a2,0x38);
  v14 = (void *)sub_12fd0(a2,8);
  v33 = (void *)sub_12fd0(a2,8);
  v15 = (unsigned long *)sub_12fd0(a2,8);
  v11 = dat_1e410;
  v25 = NULL;
  v35 = (void **)v15;
  v31 = v14;
  if (a2) {
    v24 = 0;
    v32 = a2;
    v36 = a1;
    do {
      while( true ) {
        v16 = v24 * 8;
        v22 = (unsigned long *)((long)v34 + v24 * 0x38);
        v27 = dat_1e030;
        if (dat_1e030 <= dat_1e438 / v32)
          v27 = dat_1e438 / v32;
        v26 = (unsigned long *)(v24 * 0x10 + a0);
        sub_6240(v22,0x20,v27);
        v1 = *v26;
        v2 = *(unsigned long *)((long)a5 + v16);
        if (sub_a750(v22,v2,v1)) break;
        sub_6040(*(unsigned long *)((long)a5 + v16),*v26);
        if (v24 < v36) {
          v36 -= 1;
          sub_6e80(*v26);
        }
        free((void *)*v22);
        v27 = v32 - 1;
        if (v27 <= v24) {
          if (!v27) goto label_b3cd;
          v32 = v27;
          goto label_af06;
        }
        do {
          v26 = (unsigned long *)(a0 + 0x10 + v16 * 2);
          v2 = v26[1];
          v1 = *(unsigned long *)((long)a5 + v16 + 8);
          v22 = (unsigned long *)(a0 + v16 * 2);
          *v22 = *v26;
          v22[1] = v2;
          *(unsigned long *)((long)a5 + v16) = v1;
          v16 += 8;
        } while (v32 * 8 + -8 != v16);
        v32 = v27;
      }
      v16 = sub_a740(*v22,v22[3]);
      *(long *)((long)v14 + v24 * 8) = v16 + -0x20;
      *(long *)((long)v33 + v24 * 8) = v16 + v22[2] * -0x20;
      v24 += 1;
    } while (v24 < v32);
label_af06:
    v24 = 0;
    do {
      v15[v24] = v24;
      v24 += 1;
    } while (v24 != v32);
    if (v32 != 1) {
      v24 = 1;
      do {
        while( true ) {
          v18 = &v15[v24];
          v20 = &v15[v24 - 1];
          v1 = *(unsigned long *)((long)v14 + *v18 * 8);
          v2 = *(unsigned long *)((long)v14 + *v20 * 8);
          if ((int)sub_9c70(v2,v1) <= 0) break;
          v27 = *v20;
          v24 = 1;
          *v20 = *v18;
          *v18 = v27;
        }
        v24 += 1;
      } while (v24 < v32);
    }
    v37 = 0;
    v35 = NULL;
label_afa0:
    v3 = *(long **)((long)v14 + *v15 * 8);
    if (dat_1e419) { // branch-flip
      if (v35) {
        if (!sub_9c70(v35,v3)) goto label_afd6;
        sub_9460(&v25,a3,a4);
      }
      v24 = v3[1];
      v19 = v25;
      if (v37 < v24) {
        do {
          v27 = v24;
          if (!v37) goto label_b451;
          v37 *= 2;
        } while (v37 < v24);
        v27 = v37;
label_b451:
        free(v25);
        v19 = (void *)sub_12eb0(v27);
        v24 = v3[1];
        v25 = v19;
        v37 = v27;
      }
      v38 = v24;
      v19 = memcpy(v19,(void *)*v3,v24);
      v35 = &v25;
      if (v11) {
        v39 = (v3[2] - *v3) + (long)v19;
        v40 = (long)v19 + (v3[3] - *v3);
      }
    }
    else {
      sub_9460(v3,a3,a4);
    }
label_afd6:
    v24 = *v15;
    if (*(long **)((long)v33 + v24 * 8) < v3) {
      *(long **)((long)v14 + v24 * 8) = &v3[-4];
      v31 = (void *)v24;
      if (v32 != 1) goto label_b086;
label_b14c:
      *v15 = v24;
      goto label_afa0;
    }
    v1 = *(unsigned long *)((long)a5 + v24 * 8);
    v2 = *(unsigned long *)(a0 + v24 * 0x10);
    if (sub_a750((void *)((long)v34 + v24 * 0x38),v1,v2)) {
      v24 = *v15;
      v22 = (unsigned long *)((long)v34 + v24 * 0x38);
      sub_a740(*v22,v22[3]);
      v16 = *(long *)(v21 + 0x10);
      *(long *)((long)v14 + v24 * 8) = v17 + -0x20;
      *(long *)((long)v33 + v24 * 8) = v17 + v16 * -0x20;
      v31 = (void *)v24;
      if (v32 == 1) goto label_b14c;
label_b086:
      v27 = 1;
      v12 = 1;
      v29 = v32;
      do {
        while( true ) {
          v28 = v12;
          v13 = sub_9c70(*(unsigned long *)((long)v14 + v24 * 8),*(unsigned long *)((long)v14 + v15[v28] * 8));
          if ((0 <= v13) && ((v13 || (v15[v28] <= v31)))) break;
          v12 = v27 + v28 >> 1;
          v29 = v28;
          if (v28 <= v27) goto label_b0f1;
        }
        v27 = v28 + 1;
        v12 = v27 + v29 >> 1;
      } while (v27 < v29);
label_b0f1:
      if (v27 - 1) {
        v20 = v15;
        do {
          v18 = &v20[1];
          *v20 = v20[1];
          v20 = v18;
        } while (v18 != &v15[v27 - 1]);
      }
      v15[v27 - 1] = (unsigned long)v31;
      goto label_afa0;
    }
    v24 = *v15;
    v20 = &v15[1];
    if (v32 != 1) {
      do {
        if (v24 < *v20) {
          *v20 = *v20 - 1;
          v24 = *v15;
        }
        v20 = &v20[1];
      } while (&v15[v32] != v20);
    }
    v27 = v32 - 1;
    sub_6040(*(unsigned long *)((long)a5 + v24 * 8),*(unsigned long *)(a0 + v24 * 0x10));
    v24 = *v15;
    if (v24 < v36) {
      v36 -= 1;
      sub_6e80(*(unsigned long *)(a0 + v24 * 0x10));
      v24 = *v15;
    }
    free(*(void **)((long)v34 + v24 * 0x38));
    v24 = *v15;
    if (v24 < v27) {
      v16 = v24 * 8;
      v22 = (unsigned long *)((long)v34 + v24 * 0x38);
      do {
        v26 = (unsigned long *)(a0 + 0x10 + v16 * 2);
        v2 = *v26;
        v4 = v26[1];
        v23 = &v22[7];
        v5 = *v23;
        v6 = v22[8];
        v7 = v22[9];
        v8 = v22[10];
        *(unsigned long *)((long)a5 + v16) = *(unsigned long *)((long)a5 + v16 + 8);
        v1 = v22[0xd];
        v9 = v22[0xb];
        v10 = v22[0xc];
        v26 = (unsigned long *)(a0 + v16 * 2);
        *v26 = v2;
        v26[1] = v4;
        v22[6] = v1;
        v1 = *(unsigned long *)((long)v14 + v16 + 8);
        *v22 = v5;
        v22[1] = v6;
        *(unsigned long *)((long)v14 + v16) = v1;
        v1 = *(unsigned long *)((long)v33 + v16 + 8);
        v22[2] = v7;
        v22[3] = v8;
        *(unsigned long *)((long)v33 + v16) = v1;
        v16 += 8;
        v22[4] = v9;
        v22[5] = v10;
        v22 = v23;
      } while (v32 * 8 + -8 != v16);
    }
    if (v27) {
      v20 = v15;
      do {
        v18 = &v20[1];
        *v20 = v20[1];
        v20 = v18;
      } while (&v15[v32 - 1] != v18);
      v32 = v27;
      goto label_afa0;
    }
    v30 = v35 != NULL;
    v35 = (void **)v15;
    v31 = v14;
    if (v30) {
      if (dat_1e419) {
        sub_9460(&v25,a3,a4);
        free(v25);
      }
    }
  }
label_b3cd:
  sub_6040(a3,a4);
  free(a5);
  free(v34);
  free(v35);
  free(v33);
  free(v31); // tail-call
  return;
}


// Function: sub_b500 @ 0xb500
unsigned long sub_b500(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x48
  
  v2 = sub_7470(a0,a2,&v3);
  if ((v2 < a2) && (v2 < 2)) {
    v1 = *(unsigned long *)(a0 + v2 * 0x10);
    sub_5e10(dcgettext(NULL,"open failed",5),v1); // no-return
  }
  sub_ad90(a0,a1,v2,a3,a4,v3);
  return v2;
}


// Function: sub_b5b0 @ 0xb5b0
void sub_b5b0(long a0,unsigned long a1,unsigned long a2,char *a3) // return-dupe
{
  long *v1;
  long v10; // stack - 0xd0
  char *v2;
  int v3; // eax
  long *v4; // rax
  long v5; // rax
  long *v6;
  long v7; // stack - 0xd8
  unsigned long v8; // stack - 0xe0
  long v9;
  
  if (a2 <= a1)
    return;
  v5 = 0;
  v1 = (long *)(a0 + a2 * 0x10);
  v6 = (long *)(a0 + a1 * 0x10);
  do {
    while( true ) {
      v2 = (char *)*v6;
      v3 = strcmp(v2,"-");
      if ((a3) && (!strcmp(a3,v2))) break;
      v4 = (long *)sub_5e70();
      if (!v4)
        return;
      if (!v3) goto label_b629;
      v2 = (char *)*v6;
      if ((!stat(v2,&v7)) && (v10 == v4[1])) goto label_b6a3;
label_b648:
      v6 = &v6[2];
      if (v6 == v1)
        return;
    }
    if (!v3) {
      v4 = (long *)sub_5e70();
      if (!v4)
        return;
label_b629:
      if ((!fstat(0,&v7)) && (v10 == v4[1])) {
label_b6a3:
        if (v7 == *v4) goto label_b6b0;
      }
      goto label_b648;
    }
label_b6b0:
    v9 = v5 + 0xd;
    if (!v5) {
      v5 = sub_77e0(&v8);
      v9 = v5 + 0xd;
      sub_b500(v6,0,1,v8,v9);
    }
    *v6 = v9;
    v4 = &v6[2];
    v6[1] = v5;
    v6 = v4;
    if (v4 == v1)
      return;
  } while( true );
}


// Function: sub_b720 @ 0xb720
void sub_b720(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  unsigned long v10;
  unsigned long v11; // r13
  long *v12;
  unsigned long v13; // stack - 0x80
  unsigned long v14; // stack - 0x78
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  long v6;
  unsigned long v7; // stack - 0x48
  long v8; // stack - 0x50
  long *v9;
  
  v3 = (unsigned long)dat_1e024;
  v14 = a2;
  if (v3 < a2) {
    do {
      v2 = 0;
      v6 = 0;
      if (v3 <= v14) { // branch-flip
        do {
          v11 = v2;
          v1 = sub_77e0(&v7);
          v2 = (unsigned long)dat_1e024;
          v3 = a1;
          if (v2 <= a1)
            v3 = v2;
          v2 = sub_b500(&a0[v6 * 2],v3,v2,v7);
          v5 = (unsigned long)dat_1e024;
          v3 = a1;
          if (v2 <= a1)
            v3 = v2;
          v6 += v2;
          v2 = v11 + 1;
          a0[v11 * 2] = v1 + 0xd;
          a0[v11 * 2 + 1] = v1;
          a1 -= v3;
        } while (v5 <= v14 - v6);
        v4 = v2 % v5;
        v3 = v5 - v4;
        v10 = v14 - v6;
        v9 = &a0[v11 * 2 + 2];
        v12 = &a0[v6 * 2];
      }
      else {
        v4 = 0;
        v5 = v3;
        v10 = v14;
        v9 = a0;
        v12 = a0;
      }
      if (v3 < v10) {
        v5 = (v10 + 1 + v4) - v5;
        v1 = sub_77e0(&v7);
        v3 = a1;
        if (v5 <= a1)
          v3 = v5;
        v5 = sub_b500(v12,v3,v5,v7);
        v3 = a1;
        if (v5 <= a1)
          v3 = v5;
        v2 += 1;
        v6 += v5;
        *v9 = v1 + 0xd;
        v9[1] = v1;
        a1 -= v3;
        v12 = &a0[v6 * 2];
        v9 = &a0[v2 * 2];
      }
      a1 += v2;
      memmove(v9,v12,(v14 - v6) * 0x10);
      v3 = (unsigned long)dat_1e024;
      v14 += v2 - v6;
    } while (v3 < v14);
  }
  sub_b5b0(a0,a1,v14,a3);
  v13 = a1;
  do {
    v3 = sub_7470(a0,v14,&v8);
    if (v14 != v3) { // branch-flip
      if (v3 <= 2) {
        v6 = a0[v3 * 2];
        sub_5e10(dcgettext(NULL,"open failed",5),v6); // no-return
      }
    }
    else {
      v6 = sub_5ee0(a3,"w");
      if (v6) {
        sub_ad90(a0,v13,v14,v6,a3,v8);
        return;
      }
      if ((*__errno_location() != 0x18) || (v14 <= 2))
        sub_5e10(dcgettext(NULL,"open failed",5),a3); // no-return
    }
    v3 -= 1;
    v9 = &a0[v3 * 2];
    while( true ) {
      sub_6040(*(unsigned long *)(v8 + v3 * 8),*v9);
      v12 = &v9[-2];
      v6 = sub_76b0(&v7,3 <= v3);
      if (v6) break;
      v3 -= 1;
      v9 = v12;
    }
    v2 = v13;
    if (v3 < v13)
      v2 = v3;
    v1 = v14 - v3;
    sub_ad90(a0,v2,v3,v7,v6 + 0xd,v8);
    v14 = v1 + 1;
    *a0 = v6 + 0xd;
    a0[1] = v6;
    memmove(&a0[2],v9,v1 * 0x10);
    v13 = (v13 - v2) + 1;
  } while( true );
}


// Function: sub_bac0 @ 0xbac0
void sub_bac0(unsigned long *a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1;
  unsigned long v10; // rdx
  long *v11; // rdx
  char v12;
  void *v13; // stack - 0xe8
  char v14 [104];
  unsigned long v15; // stack - 0xf0
  unsigned long v16; // stack - 0xf8
  unsigned long v17;
  long v18;
  long v19; // stack - 0x138
  unsigned long *v2;
  long v20; // stack - 0x118
  long v21; // stack - 0x110
  unsigned long *v22; // stack - 0x108
  long v23; // stack - 0xe0
  unsigned long v24; // stack - 0xd8
  long v25; // stack - 0xd0
  long v26; // stack - 0xc8
  char v27; // stack - 0xb8
  long *v3;
  char v4; // al
  long v5; // rax
  long v6;
  unsigned long v7;
  long *v8; // rax
  unsigned long v9;
  
  v25 = 0;
  v20 = a1;
  if (a1) {
    v19 = 0;
    v6 = 0;
    v22 = a0;
    do {
      v1 = *v22;
      v16 = sub_6010(v1,"r");
      v21 = 0x30;
      v9 = v16;
      if (2 <= a3) {
        v21 = 1;
        v7 = 1;
        do {
          v7 *= 2;
          v21 += 1;
        } while (v7 < a3);
        v21 *= 0x20;
        v9 = v16;
      }
      v17 = v16;
      if (!v6) {
        sub_6240(&v13,v21,sub_8700(&v16,v22,v20,v21));
        v17 = v16;
        v9 = v16;
      }
      v16 = v9;
      v20 -= 1;
      v27 = '\0';
      while (v4 = sub_a750(&v13,v17,v1), v12 = v27, v7 = v24, v4) {
        if (v27) { // branch-flip
          if (v20) {
            if ((v25 - v23) - v21 * v24 <= v21 + 1U) goto label_bc43;
            v26 = v23;
            sub_6040(v16,v1);
            goto label_bd03;
          }
          dat_1e840 = 0;
          v6 = sub_a740(v13);
          if (v19 || v26) goto label_bc5b;
          sub_6040(v16,v1);
          v19 = 0;
          v15 = sub_6010(a2,"w");
          v18 = a2;
        }
        else {
label_bc43:
          dat_1e840 = 0;
          v6 = sub_a740(v13);
label_bc5b:
          v19 += 1;
          v12 = '\0';
          v18 = sub_77e0(&v15) + 0xd;
        }
        if (2 <= v7) { // branch-flip
          sub_7b40(v14,a3);
          v5 = sub_7aa0(a3,v7,v6);
          sub_a4e0(v6,a3,v7,v5 + 0x80,v14,v15,v18);
          sub_7cd0(a3,v5);
          sub_7d10(v14);
          v9 = 0xbb91;
        }
        else {
          sub_9fb0(v6 + -0x20,v15,v18);
          v9 = v10;
        }
        sub_6040(v15,v18,v9);
        if (v12) {
          free(v13);
          sub_7010(); // return-dupe, tail-call
          return;
        }
        v17 = v16;
      }
      sub_6040(v16,v1);
      if (!v20) goto label_bdb4;
label_bd03:
      v22 = &v22[1];
      v6 = v25;
    } while( true );
  }
label_bdbd:
  free(v13);
  v2 = dat_1e368;
  v8 = (long *)sub_12fd0(v20,0x10);
  v3 = v8;
  while (v2) {
    v3[1] = (long)v2;
    v6 = (long)v2 + 0xd;
    v2 = (unsigned long *)*v2;
    v11 = &v3[2];
    *v3 = v6;
    v3 = v11;
  }
  sub_b720(v8,v20,v20,a2);
  free(v8);
  sub_7010();
  return;
label_bdb4:
  v20 = v19;
  goto label_bdbd;
}


// Function: sub_be50 @ 0xbe50
void sub_be50(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_1e888;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Write sorted concatenation of all FILE(s) to standard output.\n",5),v1);
    sub_58d0();
    sub_5900();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Ordering options:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Other options:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n",5),"/tmp");
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -z, --zero-terminated     line delimiter is NUL, not newline\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n",5),v1);
    sub_8d30();
  }
  exit(a0); // no-return
}


// Function: sub_d040 @ 0xd040
unsigned long sub_d040(unsigned long a0)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5; // rsi
  long v6; // r8
  
  v1._8_8_ = 0;
  v1._0_8_ = a0;
  v2 = SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v4 = a0 % 3;
  if ((10 <= a0) && (v4)) {
    v6 = 0x10;
    v5 = 9;
    v3 = 3;
    do {
      v3 += 2;
      v5 += v6;
      v2 = a0 / v3;
      v4 = a0 % v3;
      if (a0 <= v5) break;
      v6 += 8;
    } while (v4);
  }
  v3 = v2 >> 8;
  v2 = CONCAT71((undefined7)v3,v4 != 0);
  return v2;
}


// Function: sub_d0b0 @ 0xd0b0
unsigned long sub_d0b0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_d040(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}


// Function: sub_d0f0 @ 0xd0f0
unsigned long sub_d0f0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_13a80(a0,3) % a1;
}


// Function: sub_d110 @ 0xd110
bool sub_d110(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_d120 @ 0xd120
unsigned long sub_d120(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x17200)
    return 1;
  v1 = v2[2];
  if ((((dat_17214 < v1) && (v1 < dat_17218)) && (dat_1721c < v2[3])) && (dat_17110 <= *v2)) {
    v3 = *v2 + dat_17214;
    if (((v3 < v2[1]) && (v2[1] <= dat_17220)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x17200;
  return 0;
}


// Function: sub_d1b0 @ 0xd1b0
void sub_d1b0(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_d1d0 @ 0xd1d0
long sub_d1d0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}


// Function: sub_d200 @ 0xd200
long sub_d200(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_d1d0(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_d1b0(a0);
      return v5;
    }
    v5 = *v4;
  }
  if (a3) {
    v2 = (long *)v4[1];
    if (v2) {
      v3 = v2[1];
      *v4 = *v2;
      v4[1] = v3;
      sub_d1b0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}


// Function: sub_d300 @ 0xd300
void sub_d300(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_d330 @ 0xd330
unsigned long sub_d330(long a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  long *v2; // rax
  long *v3;
  long *v4;
  long *v5;
  long v6;
  
  v5 = (long *)*a1;
  if ((long *)a1[1] <= v5)
    return 1;
  do {
    while (v6 = *v5, !v6) {
label_d358:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_d1d0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_d1b0(a0);
          v3 = v4;
          if (!v4) goto label_d3c8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_d3c8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_d358;
    v3 = (long *)sub_d1d0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_d300(a0);
      if (!v4)
        return 0;
      v1 = v3[1];
      *v4 = v6;
      v4[1] = v1;
      v3[1] = (long)v4;
    }
    else {
      *v3 = v6;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1[3] = a1[3] + -1;
    if ((long *)a1[1] <= v5)
      return 1;
  } while( true );
}


// Function: sub_d4f0 @ 0xd4f0
unsigned long sub_d4f0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_d500 @ 0xd500
unsigned long sub_d500(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_d510 @ 0xd510
unsigned long sub_d510(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_d520 @ 0xd520
unsigned long sub_d520(unsigned long *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = (long *)*a0;
  v4 = 0;
  do {
    if ((long *)a0[1] <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if ((long *)a0[1] <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_d5f0 @ 0xd5f0
void sub_d5f0(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_d510(a0);
  v2 = sub_d4f0();
  v3 = sub_d500();
  v4 = sub_d520();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_d710 @ 0xd710
long sub_d710(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_d1d0(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_d7c0 @ 0xd7c0
long sub_d7c0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_d1d0(a0,a1);
  v3 = v2;
  do {
    v1 = *v3;
    v3 = (long *)v3[1];
    if (v1 == a1) {
      if (v3)
        return *v3;
      break;
    }
  } while (v3);
  do {
    v2 = &v2[2];
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_d880 @ 0xd880
long sub_d880(unsigned long *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = (long *)*a0;
  if ((long *)a0[1] <= v4) // branch-flip
    v3 = 0;
  else {
    v3 = 0;
    do {
      v2 = *v4;
      if (v2) {
        v1 = v4;
        while( true ) {
          if (!(*a1)(v2,a2))
            return v3;
          v1 = (long *)v1[1];
          v3 += 1;
          if (!v1) break;
          v2 = *v1;
        }
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
  return v3;
}


// Function: sub_d970 @ 0xd970
unsigned long * sub_d970(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_d0f0;
  if (!a3)
    a3 = sub_d110;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x17200;
  v1[5] = a1;
  if (sub_d120(v1)) {
    v2 = sub_d460(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
    v1[2] = v2;
    if (v2) {
      v3 = calloc(v2,0x10);
      *v1 = v3;
      if (v3) {
        v1[6] = a2;
        v1[3] = 0;
        v1[7] = a3;
        v1[8] = a4;
        v1[1] = (void *)((long)v3 + v1[2] * 0x10);
        v1[4] = 0;
        v1[9] = 0;
        return v1;
      }
    }
  }
  free(v1);
  return NULL;
}


// Function: sub_da80 @ 0xda80
void sub_da80(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = (long *)*a0;
  if ((long *)*a0 < (long *)a0[1]) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if ((long *)a0[1] <= v5) goto label_db1b;
      }
      v3 = (void *)a0[8];
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = (void *)a0[8];
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0[9];
          *v4 = 0;
          v4[1] = v2;
          a0[9] = v4;
          if (!v1) break;
          v4 = v1;
        }
      }
      if (v3)
        (*v3)(*v5);
      *v5 = 0;
      v6 = &v5[2];
      v5[1] = 0;
      v5 = v6;
    } while (v6 < (long *)a0[1]);
  }
label_db1b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_db30 @ 0xdb30
void sub_db30(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_dbdc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_db9f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0[8])(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = (long *)a0[1];
      v4 = &v4[2];
    } while (v4 < v3);
label_db9f:
    v4 = (long *)*a0;
  }
  if (v4 < v3) {
    do {
      v2 = (void *)v4[1];
      while (v2) {
        v1 = *(void **)((long)v2 + 8);
        free(v2);
        v2 = v1;
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
label_dbdc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_dc20 @ 0xdc20
unsigned int sub_dc20(unsigned long *a0,unsigned long a1) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  void *v2; // stack - 0x88
  void *v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v4 = sub_d460(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
  if (!v4)
    return 0;
  if (a0[2] == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0[5];
  v6 = 0;
  v8 = a0[6];
  v9 = a0[7];
  v10 = a0[8];
  v11 = a0[9];
  v1 = sub_d330(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_d330(a0,&v2,1)) && (sub_d330(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free((void *)*a0);
  *a0 = v2;
  a0[1] = v3;
  a0[2] = v4;
  a0[3] = v5;
  a0[9] = v11;
  return v1;
}


// Function: sub_dda0 @ 0xdda0
unsigned long sub_dda0(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_d200(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_d120(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_17224 <= v5)
          return 0xffffffff;
        v4 = (dat_17228 <= v5) ? (long)(v5 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_dc20(a0,v4))
          return 0xffffffff;
        if (sub_d200(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_d300(a0);
    if (!v2)
      return 0xffffffff;
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_df90 @ 0xdf90
unsigned long sub_df90(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_dda0(a0,a1,&v3);
  if (v1 != -1) {
    v4 = v1 == 0;
    v2 = a1;
    if (v4)
      v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_dff0 @ 0xdff0
long sub_dff0(long a0,unsigned long a1) // ternary x2
{
  unsigned long v1;
  float *v2;
  void *v3;
  void *v4;
  long v5; // rax
  unsigned long v6; // rax
  long *v7; // stack - 0x28
  float v8; // xmm4_da
  float v9;
  
  v5 = sub_d200(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_d120(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_17228 <= v8) ? (long)(v8 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_dc20(a0,v6)) {
          v4 = *(void **)(a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)(a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}


// Function: sub_e7d0 @ 0xe7d0
unsigned long sub_e7d0(char *a0) // return-dupe x2
{
  char v1;
  unsigned long v2; // rax
  char *v3; // stack - 0x28
  char *v4; // rbp
  
  if (!a0)
    return 0;
  v1 = *a0;
  do {
    if (!v1) {
      v1 = sub_13b70(0);
label_e82a:
      if (!v1)
        return 0;
      v3 = NULL;
      v2 = strtoul(a0,&v3,10);
      if (!v3)
        return 0;
      v1 = *v3;
      while( true ) {
        if (!v1)
          return v2;
        v4 = &v3[1];
        if (!sub_13c00((int)v1)) break;
        v1 = *v4;
        v3 = v4;
      }
      if (v1 != ',')
        return 0;
      return v2;
    }
    if (!sub_13c00((int)v1)) {
      v1 = sub_13b70((int)v1);
      goto label_e82a;
    }
    v1 = a0[1];
    a0 = &a0[1];
  } while( true );
}


// Function: sub_e8d0 @ 0xe8d0
long sub_e8d0(void) // return-dupe
{
  long v1; // rax
  char v2 [136];
  
  if (sched_getaffinity(0,0x80,v2))
    return 0;
  v1 = (long)(int)__sched_cpucount(0x80,v2);
  if (!v1)
    return 0;
  return v1;
}


// Function: sub_e940 @ 0xe940
unsigned long sub_e940(int a0) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if (a0 != 1) {
    v1 = sysconf(0x53);
    v2 = v1;
    if ((v1 - 1 < 2) && (v2 = sub_e8d0(), v2 <= v1))
      return v1;
    if ((long)v2 > 0)
      return v2;
    return 1;
  }
  v2 = sub_e8d0();
  if (v2)
    return v2;
  v2 = sysconf(0x54);
  if ((long)v2 > 0)
    return v2;
  return 1;
}


// Function: sub_e9d0 @ 0xe9d0
unsigned long sub_e9d0(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = 0xffffffffffffffff;
  if ((int)a0 == 2) {
    v1 = sub_e7d0(getenv("OMP_NUM_THREADS"));
    v2 = sub_e7d0(getenv("OMP_THREAD_LIMIT"));
    a0 = 1;
    if (!v2)
      v2 = 0xffffffffffffffff;
    if (v1) {
      if (v2 <= v1)
        v1 = v2;
      return v1;
    }
  }
  v1 = sub_e940(a0);
  if (v2 <= v1)
    v1 = v2;
  return v1;
}

