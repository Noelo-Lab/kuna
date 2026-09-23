// Function: main @ 0x3ec0
int main(int argc,char **argv,char **envp) // ternary x3
{
  unsigned char *v1;
  unsigned char v10;
  bool v100; // stack - 0x3b2
  long v101; // stack - 0x390
  long v102; // stack - 0x388
  long v103; // stack - 0x380
  char v104; // stack - 0x33f
  char v105 [16]; // stack - 0x320
  char v106 [16]; // stack - 0x310
  char v107 [16]; // stack - 0x300
  char v108 [16]; // stack - 0x2f0
  unsigned long v109; // stack - 0x2d0
  unsigned char v11;
  unsigned long v110; // stack - 0x2c8
  long v111; // stack - 0x2c0
  unsigned long v112; // stack - 0x2b8
  unsigned long v113; // stack - 0x2b0
  unsigned char v114; // stack - 0x2a8
  unsigned char v115; // stack - 0x2a7
  unsigned char v116; // stack - 0x2a6
  unsigned char v117; // stack - 0x2a5
  unsigned char v118; // stack - 0x2a4
  unsigned char v119; // stack - 0x2a3
  char v12;
  unsigned char v120; // stack - 0x2a2
  unsigned char v121; // stack - 0x2a1
  unsigned char v122; // stack - 0x2a0
  unsigned int v123; // stack - 0x284
  unsigned long *v124; // stack - 0x280
  undefined4 v125; // stack - 0xe0
  long v126; // stack - 0x40
  char v13;
  unsigned char v14;
  int v15;
  int v16; // eax
  unsigned int v17; // eax
  int v18; // eax
  int v19; // eax
  unsigned long *v2;
  int v20; // eax
  int v21; // eax
  int v22; // eax
  int v23; // eax
  int v24; // eax
  int v25; // eax
  lconv *v26; // rax
  long *v27;
  long *v28;
  long v29;
  void **v3;
  unsigned char *v30;
  unsigned long v31;
  long v32;
  unsigned long *v33;
  void *v34;
  unsigned long *v35; // rax
  unsigned long *v36;
  unsigned long *v37; // rax
  void *v38;
  unsigned long v39;
  unsigned short v4;
  void *v40; // rax
  char *v41;
  FILE *v42; // rax
  unsigned long *v43;
  unsigned long v44; // rcx
  unsigned char v45;
  unsigned char v46;
  unsigned char v47;
  unsigned int v48;
  unsigned long *v49;
  unsigned int v5;
  unsigned long *v50;
  void *v51; // stack - 0x168
  void *v52; // stack - 0x398
  unsigned long v53; // stack - 0x328
  unsigned int v54; // stack - 0x78
  void *v55; // stack - 0x2d8
  char v56 [32];
  char v57 [47];
  char v58; // stack - 0xa8
  int v59; // stack - 0x288
  long v6;
  long v60; // stack - 0x378
  unsigned long v61 [11]; // stack - 0x138
  long v62; // stack - 0x368
  long v63; // stack - 0x370
  long v64 [4]; // stack - 0x360
  pthread_mutex_t v65; // stack - 0x160
  unsigned char v66;
  char v67;
  unsigned long *v68;
  char *v69;
  bool v7;
  char v70; // dil
  void **v71;
  unsigned int *v72;
  int *v73; // r12
  char *v74;
  unsigned char v75;
  int *v76; // r13
  long *v77;
  unsigned char v78;
  unsigned long v79;
  bool v8;
  unsigned char v80;
  unsigned long v81;
  long v82;
  bool v83;
  bool v84;
  bool v85;
  double v86; // xmm0_qa
  double v87; // xmm0_qa
  double v88; // xmm0_qa
  double v89; // xmm0_qa
  unsigned char v9;
  double v90;
  unsigned long v91; // stack - 0x458
  char *v92; // stack - 0x438
  char *v93; // stack - 0x430
  void **v94; // stack - 0x420
  unsigned long v95; // stack - 0x408
  char *v96; // stack - 0x3f8
  long v97; // stack - 0x3e0
  char *v98; // stack - 0x3d8
  unsigned long v99; // stack - 0x3d0
  
  v126 = *(long *)(v82 + 0x28);
  v94 = (char *)getenv("POSIXLY_CORRECT");
  v83 = v94 == NULL;
  v15 = sub_eb70();
  v85 = 0x2b8 < (unsigned int)(v15 - 0x30db0U);
  sub_ec00(*argv);
  v93 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  dat_1d130 = 2;
  dat_1d861 = sub_cc40(3);
  v12 = sub_cc40(2);
  dat_1d860 = v12;
  v26 = localeconv();
  dat_1d868 = **(unsigned char **)v26;
  if ((!dat_1d868) || ((*(unsigned char **)v26)[1]))
    dat_1d868 = 0x2e;
  v13 = **(char **)&v26->field_0x8;
  dat_1d864 = (unsigned int)v13;
  if (v13) {
    if (!(*(char **)&v26->field_0x8)[1]) goto label_3fc7;
    dat_1d862 = '\x01';
  }
  dat_1d864 = 0x80;
label_3fc7:
  dat_1d418 = '\0';
  v27 = __ctype_b_loc();
  v28 = __ctype_toupper_loc();
  v29 = 0;
  do {
    v4 = *(unsigned short *)(*v27 + v29 * 2);
    if (v4 & 1) { // branch-flip
      v70 = 1;
      v47 = 0;
label_402e:
      *(unsigned char *)(v29 + 0x1d540) = v47;
      v32 = *v28;
      *(char *)(v29 + 0x1d740) = v70;
      v5 = *(unsigned int *)(v32 + v29 * 4);
      *(unsigned char *)(v29 + 0x1d640) = ((unsigned char)(v4 >> 0xe) ^ 1) & 1;
      *(char *)(v29 + 0x1d440) = (char)v5;
      if (v29 == 0xff) goto label_40b8;
    }
    else {
      if (v29 != 10) {
        v70 = 0;
        v47 = ((unsigned char)(v4 >> 3) ^ 1) & 1;
        goto label_402e;
      }
      dat_1d74a = 1;
      dat_1d54a = 0;
      dat_1d64a = ((unsigned char)((unsigned short)*(unsigned short *)(*v27 + 0x14) >> 0xe) ^ 1) & 1;
      dat_1d44a = (char)*(unsigned int *)(*v28 + 0x28);
    }
    v29 += 1;
  } while( true );
label_60e5:
  sub_7790(&v55,v97,v38);
label_5385:
  v94 = &v55;
  v95 -= 1;
  v114 = 0;
  while( true ) {
    v12 = sub_7ef0(v94,v42,v74);
    v47 = v114;
    v29 = v111;
    v79 = v110;
    v34 = v55;
    if (!v12) break;
    if (v114) { // branch-flip
      if (v95) {
        if ((v111 - v109) - v97 * v110 <= v97 + 1U) goto label_5402;
        v112 = v109;
        sub_7660(v42,v74);
        goto label_5d45;
      }
      dat_1d840 = 0;
      if (v91 || v112) goto label_5402;
      sub_7660(v42,v74);
      v38 = (void *)sub_74e0(v96,"w");
      if (!v38) {
        sub_7480(dcgettext(NULL,"open failed",5),v96); // return-dupe, no-return
      }
      v52 = v38;
      v91 = 0;
      v69 = v96;
      v79 = v110;
    }
    else {
label_5402:
      dat_1d840 = 0;
      v91 += 1;
      v47 = 0;
      v69 = (char *)(sub_8c20(&v52,0) + 0xd);
    }
    v29 += (long)v34;
    if (2 <= v79) { // branch-flip
      v51 = (void *)sub_deb0(sub_7c20,v81);
      pthread_mutex_init(&v65,NULL);
      pthread_cond_init(v61,NULL);
      v28 = (long *)sub_12ab0(v31 << 8);
      v28[3] = 0;
      v28[2] = 0;
      v28[1] = 0;
      *v28 = 0;
      v28[4] = 0;
      v28[6] = v79;
      v28[5] = v79;
      v28[7] = 0;
      *(unsigned int *)&v28[10] = 0;
      *(char *)((long)v28 + 0x54) = 0;
      pthread_mutex_init((pthread_mutex_t *)&v28[0xb],NULL);
      sub_7af0(v28,(struct_14 *)&v28[0x10],v29,v31,v79,0);
      sub_b220(v29,v31,v79,(struct_14 *)&v28[0x10],(sigaction *)&v51,v52,v69);
      if (v81) {
        v27 = &v28[0xb];
        v29 = v81 - 1;
        do {
          v29 -= 1;
          v77 = &v27[0x10];
          pthread_mutex_destroy(v27);
          v27 = v77;
        } while (v29 != -1);
      }
      free(v28);
      sub_df20(v51);
      pthread_cond_destroy(v61);
      pthread_mutex_destroy(&v65);
    }
    else {
      sub_a3c0((long *)(v29 + -0x20),v52,v69);
    }
    sub_7660(v52,v69);
    if (v47) {
      free(v55);
      goto label_5599;
    }
  }
  sub_7660(v42,v74);
  if (!v95) goto label_64ce;
label_5d45:
  v33 = &v33[1];
  goto label_5328;
label_64ce:
  free(v55);
  v33 = dat_1d368;
  v27 = (long *)sub_12bf0(v91,0x10);
  v28 = v27;
  while (v33) {
    v28[1] = (long)v33;
    v29 = (long)v33 + 0xd;
    v33 = (unsigned long *)*v33;
    v77 = &v28[2];
    *v28 = v29;
    v28 = v77;
  }
  sub_ac90(v27,v91,v91,v96);
  free(v27);
label_5599:
  while (1 <= dat_1d35c) {
    sub_78c0(0xffffffff);
  }
  goto label_55b4;
label_40b8:
  if (v12) {
    v29 = 1;
    v28 = (long *)0x1d060;
    do {
      v30 = (unsigned char *)nl_langinfo((int)v29 + 0x2000d);
      v31 = strlen((char *)v30);
      v32 = sub_12ab0(v31 + 1);
      *(int *)&v28[1] = (int)v29;
      *v28 = v32;
      if (v31) {
        v6 = *v27;
        v1 = &v30[v31];
        v31 = 0;
        do {
          if (!(*(unsigned char *)(v6 + (unsigned long)*v30 * 2) & 1)) {
            *(char *)(v32 + v31) = *(char *)((unsigned long)*v30 + 0x1d440);
            v31 += 1;
          }
          v30 = &v30[1];
        } while (v1 != v30);
      }
      v29 += 1;
      *(char *)(v32 + v31) = 0;
      v28 = &v28[2];
    } while (v29 != 0xd);
    qsort(0x1d060,0xc,0x10,sub_7300);
  }
  v76 = (int *)0x15324;
  v73 = (int *)0x15324;
  sigemptyset((sigset_t *)0x1d380);
  v15 = 0xe;
  while( true ) {
    sigaction(v15,NULL,(sigaction *)&v51);
    if (v51 != (void *)0x1)
      sigaddset((sigset_t *)0x1d380,v15);
    if (v73 == (int *)0x1534c) break;
    v15 = *v73;
    v73 = &v73[1];
  }
  v125 = 0;
  v51 = sub_8270;
  v15 = 0xe;
  v65._0_8_ = dat_1d380;
  v65._8_8_ = dat_1d388;
  v65._16_8_ = dat_1d390;
  v65._24_8_ = dat_1d398;
  v65._32_8_ = dat_1d3a0;
  v61[0] = dat_1d3a8;
  v61[1] = dat_1d3b0;
  v61[2] = dat_1d3b8;
  v61[3] = dat_1d3c0;
  v61[4] = dat_1d3c8;
  v61[5] = dat_1d3d0;
  v61[6] = dat_1d3d8;
  v61[7] = dat_1d3e0;
  v61[8] = dat_1d3e8;
  v61[9] = dat_1d3f0;
  v61[10] = dat_1d3f8;
  while( true ) {
    if (sigismember((void *)0x1d380,v15))
      sigaction(v15,(sigaction *)&v51,NULL);
    if (v76 == (int *)0x1534c) break;
    v15 = *v76;
    v76 = &v76[1];
  }
  v81 = 0;
  signal(0x11,0);
  sub_142c0(sub_8ed0);
  v74 = "-bcCdfghik:mMno:rRsS:t:T:uVy:z";
  v105._0_16_ = ZEXT816(0xffffffffffffffff) << 0x40;
  v53 = 0xffffffffffffffff;
  v106._0_16_ = 0;
  v107._0_16_ = 0;
  v108._0_16_ = 0;
  v33 = (unsigned long *)sub_12bf0((long)argc,8);
  v8 = 0;
  v31 = 0;
  v96 = NULL;
  v92 = NULL;
  v99 = 0;
  v98 = NULL;
  v12 = '\0';
  v91._0_4_ = argc;
label_43e0:
  v52 = (void *)CONCAT44(v52._4_4_,0xffffffff);
  if ((int)v31 == -1) {
label_43f1:
    v29 = (long)(int)optind;
    goto label_43c0;
  }
  if ((!v81) || (v83)) {
label_4410:
    v71 = &v52;
    v48 = getopt_long((unsigned int)v91,argv,v74,(option *)0x1c5e0,(int *)v71);
    v34 = v52;
    v69 = optarg;
    v31 = (unsigned long)v48;
    if (v48 == 0xffffffff) goto label_43f1;
    if (0x87 < (int)v48) {
      sub_bab0(2); // return-dupe, no-return
    }
    if ((int)v48 <= 0) {
      if (v48 == 0xffffff7d) {
        sub_128e0(stdout,"sort","GNU coreutils",dat_1d120,"Mike Haertel","Paul Eggert",0,v71);
        exit(0); // no-return
      }
      if (v48 == 0xffffff7e)
        sub_bab0(0); // no-return
      sub_bab0(2);
    }
    if (0x87 < v48) {
      sub_bab0(2);
    }
    v15 = (int)v52;
    switch(v48) {
      default:
        goto label_661d;
      case 1:
        if (*optarg == '+') {
          if (optind != (unsigned int)v91) { // branch-flip
            v84 = 0;
            if (*argv[(int)optind] == '-')
              v84 = (unsigned int)((int)argv[(int)optind][1] - 0x30U) <= 9;
          }
          else {
            v84 = 0;
          }
          v85 = (bool)(v85 | v94 == NULL & v84);
          if (v85) {
            v29 = 0x12;
            v28 = &v60;
            while (v29) {
              v77 = (long *)((long)v28 + 4);
              *(unsigned int *)v28 = 0;
              v29 -= 1;
              v28 = v77;
            }
            v62 = -1;
            v69 = (char *)sub_7390(&optarg[1],&v60,NULL);
            if (v69) { // branch-flip
              if (*v69 != '.') { // branch-flip
                if (!v63 && !v60) {
label_61b5:
                  v60 = -1;
                  goto label_61c1;
                }
              }
              else {
                v69 = (char *)sub_7390(&v69[1],&v63,NULL);
                if (!v60 && !v63) goto label_61b5;
label_61c1:
                if (!v69) goto label_67de;
              }
              if (!*(char *)sub_71f0(v69,&v60,0)) {
                if (v84) {
                  v29 = (long)(int)optind;
                  optind += 1;
                  v69 = argv[v29];
                  v41 = (char *)sub_7390(&v69[1],&v62,"invalid number after \'-\'");
                  if (!v41)
                    __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                  if (*v41 == '.')
                    v41 = (char *)sub_7390(&v41[1],v64,"invalid number after \'.\'");
                  if ((!v64[0]) && (v62))
                    v62 -= 1;
                  if (*(char *)sub_71f0(v41,&v60,1))
                    sub_7340(v69,"stray character in field spec"); // no-return
                }
                v104 = 1;
                sub_76e0(&v60);
                v31 = (unsigned long)v48;
                break;
              }
            }
            else if (!v63 && !v60)
              v60 = -1;
label_67de:
            v69 = optarg;
          }
        }
        v31 = (unsigned long)v48;
        v33[v81] = v69;
        v81 += 1;
        break;
      case 0x43:
      case 99:
label_4527:
        if ((v12) && ((int)v12 != (int)v31))
          sub_7310("cC"); // no-return
        v12 = (char)v31;
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
        goto label_44a1;
      case 0x53:
        v15 = sub_132f0(optarg,&v59,10,&v55,"EgGkKmMPtTYZ");
        if (v15) { // branch-flip
          if (((v15 != 2) || (v41 = (char *)CONCAT44(v123,v59), 10 <= (unsigned int)((int)v41[-1] - 0x30U))) || (v41[1])) {
            sub_13230(v15,(unsigned long)v34 & 0xffffffff,0x53,0x1c5e0,v69); // return-dupe, no-return
          }
          if (*v41 != '%') { // branch-flip
            if (*v41 != 'b') {
              sub_13230(v15,(unsigned long)v34 & 0xffffffff,0x53,0x1c5e0,v69);
            }
          }
          else {
            sub_e760();
            v90 = ((double)v55 * v89) / dat_170f0;
            if (dat_170f8 <= v90) goto label_6691;
            if (v90 < dat_17100) {
              v38 = (void *)(long)v90;
              v55 = (void *)(long)v90;
              goto label_56b9;
            }
            v55 = (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000);
          }
          v38 = v55;
        }
        else {
          v38 = v55;
          if ((unsigned int)((int)*(char *)(CONCAT44(v123,v59) + -1) - 0x30U) <= 9) {
            if ((void *)0x3fffffffffffff < v55) {
label_6691:
              v15 = 1;
              sub_13230(v15,(unsigned long)v34 & 0xffffffff,0x53,0x1c5e0,v69);
            }
            v38 = (void *)((long)v55 << 10);
            v55 = v38;
          }
        }
label_56b9:
        v31 = (unsigned long)v48;
        if ((dat_1d438 <= v38) && (dat_1d438 = (void *)((unsigned long)dat_1d024 * 0x22), (void *)((unsigned long)dat_1d024 * 0x22) < v38))
          dat_1d438 = v38;
        break;
      case 0x54:
        sub_7730(optarg);
        v31 = (unsigned long)v48;
        break;
      case 0x6b:
        v29 = 0x12;
        v28 = &v60;
        while (v29) {
          v77 = (long *)((long)v28 + 4);
          *(unsigned int *)v28 = 0;
          v29 -= 1;
          v28 = v77;
        }
        v62 = -1;
        v69 = (char *)sub_7390(optarg,&v60,"invalid number at field start");
        v29 = v60 + -1;
        v84 = v60 == 0;
        v60 = v29;
        if (v84) {
          sub_7340(optarg,"field number is zero"); // return-dupe, no-return
        }
        if (*v69 == '.') {
          v69 = (char *)sub_7390(&v69[1],&v63,"invalid number after \'.\'");
          v29 = v63 + -1;
          v84 = v63 == 0;
          v63 = v29;
          if (v84)
            sub_7340(optarg,"character offset is zero"); // no-return
        }
        if (!v60 && !v63)
          v60 = -1;
        v69 = (char *)sub_71f0(v69,&v60,0);
        if (*v69 != ',') { // branch-flip
          v62 = -1;
          v64[0] = 0;
        }
        else {
          v69 = (char *)sub_7390(&v69[1],&v62,"invalid number after \',\'");
          v29 = v62 + -1;
          v84 = v62 == 0;
          v62 = v29;
          if (v84) {
            sub_7340(optarg,"field number is zero");
          }
          if (*v69 == '.')
            v69 = (char *)sub_7390(&v69[1],v64,"invalid number after \'.\'");
          v69 = (char *)sub_71f0(v69,&v60,1);
        }
        if (*v69) {
          sub_7340(optarg,"stray character in field spec"); // return-dupe, no-return
        }
        sub_76e0(&v60);
        v31 = (unsigned long)v48;
        break;
      case 0x6d:
        v8 = 1;
        break;
      case 0x6f:
        if ((v96) && (strcmp(v96,optarg))) goto label_6dbc;
        v31 = (unsigned long)v48;
        v96 = v69;
        break;
      case 0x73:
        dat_1d41a = '\x01';
        break;
      case 0x74:
        v17 = (unsigned int)*optarg;
        if (!*optarg) {
          error(2,0,dcgettext(NULL,"empty tab",5));
          return v18;
        }
        if (optarg[1]) {
          if (strcmp(optarg,"\\0")) {
            v74 = (char *)sub_10fd0(v69);
            error(2,0,dcgettext(NULL,"multi-character tab %s",5),v74);
            return v19;
          }
          v17 = 0;
        }
        v31 = (unsigned long)v48;
        if ((dat_1d028 != 0x80) && (dat_1d028 != v17)) {
          error(2,0,dcgettext(NULL,"incompatible tabs",5));
label_6dbc:
          error(2,0,dcgettext(NULL,"multiple output files specified",5));
          sub_7340(optarg,"field number is zero");
        }
        dat_1d028 = v17;
        break;
      case 0x75:
        dat_1d419 = 1;
        break;
      case 0x79:
        if (argv[(long)(int)optind + -1] == optarg) {
          for (; (unsigned int)((int)*v69 - 0x30U) <= 9; v69 = &v69[1]) {
          }
          optind = (optind - 1) + (unsigned int)(*v69 == '\0');
        }
        break;
      case 0x7a:
        dat_1d038 = 0;
        break;
      case 0x80:
        v31 = 99;
        if (optarg) {
          v29 = sub_c370("--check",optarg,(long *)0x1c5c0,"CCc",1,dat_1d128,1,99);
          v31 = (unsigned long)(unsigned int)(int)"CCc"[v29];
        }
        goto label_4527;
      case 0x81:
        if ((dat_1d408) && (strcmp(dat_1d408,optarg))) {
label_699a:
          error(2,0,dcgettext(NULL,"multiple compress programs specified",5));
          sub_7340(optarg,"stray character in field spec");
        }
        v31 = (unsigned long)v48;
        dat_1d408 = v69;
        break;
      case 0x82:
        dat_1d400 = '\x01';
        break;
      case 0x83:
        goto label_4ef5;
      case 0x84:
        v16 = sub_132f0(optarg,NULL,10,&v55,"");
        v31 = (unsigned long)v48;
        v48 = 0x11;
        if (!getrlimit(7,&v59))
          v48 = v59 - 3;
        if (v16) { // branch-flip
          if (v16 != 1)
            sub_13230(v16,(unsigned long)v34 & 0xffffffff,0xffffff84,0x1c5e0,v69); // no-return
        }
        else {
          dat_1d024 = (unsigned int)v55;
          if ((void *)((unsigned long)v55 & 0xffffffff) == v55) {
            if (dat_1d024 < 2) {
              v69 = (char *)sub_10fd0(v69);
              v74 = *(char **)((long)v15 * 0x20 + 0x1c5e0);
              error(0,0,dcgettext(NULL,"invalid --%s argument %s",5),v74,v69);
              v69 = (char *)sub_10fd0("2");
              error(2,0,dcgettext(NULL,"minimum --%s argument is %s",5),v74,v69);
              return v20;
            }
            if (dat_1d024 <= v48) break;
          }
        }
        v39 = sub_10fd0(v69);
        v74 = *(char **)((long)v15 * 0x20 + 0x1c5e0);
        error(0,0,dcgettext(NULL,"--%s argument %s too large",5),v74,v39);
        v27 = (long *)sub_e1b0(v48,&v54);
        error(2,0,dcgettext(NULL,"maximum --%s argument with current rlimit is %s",5),v74,v27);
        v91._0_4_ = v48;
label_4ef5:
        v92 = optarg;
        break;
      case 0x85:
label_4d92:
        v69 = optarg;
        if (v98) {
          v31 &= 0xffffffff;
          if (strcmp(v98,optarg)) {
            error(2,0,dcgettext(NULL,"multiple random sources specified",5));
            goto label_699a;
          }
        }
        v98 = v69;
        break;
      case 0x86:
        v29 = sub_c370("--sort",optarg,(long *)0x1c580,"ghMnRVCCc",1,dat_1d128,1,(long)*(int *)((unsigned long)v48 * 4 + 0x150cc) + 0x150cc);
        v48 = (unsigned int)"ghMnRVCCc"[v29];
label_44a1:
        v54._0_2_ = (unsigned short)(unsigned char)v48;
        sub_71f0(&v54,&v53,2);
        v31 = (unsigned long)v48;
        break;
      case 0x87:
        v15 = sub_132f0(optarg,NULL,10,&v59,"");
        v31 = (unsigned long)v48;
        if (v15 != 1) { // branch-flip
          if (v15)
            sub_13230(v15,(unsigned long)v34 & 0xffffffff,0xffffff87,0x1c5e0,v69); // no-return
          v99 = CONCAT44(v123,v59);
          if (!v99) {
            error(2,0,dcgettext(NULL,"number in parallel must be nonzero",5));
            goto label_4d92;
          }
        }
        else {
          v99 = 0xffffffffffffffff;
        }
      
    }
    goto label_43e0;
  }
  v29 = (long)(int)optind;
  if ((v85) && (!v12)) {
    if ((unsigned int)v91 != optind) {
      v69 = argv[(int)optind];
      if (((*v69 == '-') && (v69[1] == 'o')) && ((v69[2] || (optind + 1 != (unsigned int)v91)))) goto label_4410;
      goto label_43c0;
    }
    if (!v92) {
      v12 = '\0';
      goto label_4571;
    }
label_65b8:
    v74 = (char *)sub_108e0(4,(char *)*v33);
    error(0,0,dcgettext(NULL,"extra operand %s",5),v74);
    __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
label_661d:
    sub_bab0(2);
  }
label_43c0:
  if ((int)v29 < (int)(unsigned int)v91) {
    optind = (int)v29 + 1;
    v33[v81] = argv[v29];
    v81 += 1;
    goto label_43e0;
  }
  if (!v92) goto label_4571;
  if (v81) goto label_65b8;
  v42 = (FILE *)sub_74e0(v92,"r");
  if (!v42)
    sub_7c60(v92); // no-return
  sub_119f0(&v59);
  if (!sub_11aa0(v42,&v59)) {
    v74 = (char *)sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"cannot read file names from %s",5),v74);
    return v24;
  }
  sub_7660(v42,v92);
  if (!CONCAT44(v123,v59)) {
    v74 = (char *)sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"no input from %s",5),v74);
    return v23;
  }
  free(v33);
  v33 = v124;
  if (CONCAT44(v123,v59)) {
    v81 = 0;
    do {
      v74 = (char *)v124[v81];
      if (!strcmp(v74,"-")) {
        v74 = (char *)sub_108e0(4,v74);
        error(2,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v74);
        return v21;
      }
      v81 += 1;
      if (!*v74) {
        v74 = (char *)sub_10c60(0,3,v92);
        error(2,0,dcgettext(NULL,"%s:%lu: invalid zero-length file name",5),v74,v81);
        return v22;
      }
    } while (CONCAT44(v123,v59) != v81);
  }
label_4571:
  if (dat_1d410) { // branch-flip
    v47 = 0;
    v43 = dat_1d410;
    v49 = dat_1d410;
    do {
      while ((v13 = sub_70e0(v43), !v13 || (*(char *)((long)v43 + 0x37)))) {
        v30 = (unsigned char *)((long)v43 + 0x33);
        v43 = (unsigned long *)v43[8];
        v47 |= *v30;
        if (!v43) goto label_463f;
      }
      v43[4] = v106._8_8_;
      v43[5] = v107._0_8_;
      *(char *)&v43[6] = v107[8];
      *(char *)((long)v43 + 0x31) = v107[9];
      *(char *)((long)v43 + 0x36) = v107[0xe];
      *(char *)((long)v43 + 0x32) = v107[10];
      *(char *)((long)v43 + 0x34) = v107[0xc];
      *(char *)((long)v43 + 0x35) = v107[0xd];
      *(char *)((long)v43 + 0x37) = v107[0xf];
      *(char *)&v43[7] = v108[0];
      *(unsigned char *)((long)v43 + 0x33) = v107[0xb];
      v43 = (unsigned long *)v43[8];
      v47 |= v107[0xb];
    } while (v43);
label_463f:
    v85 = 0;
    do {
      if (1 < (unsigned int)*(unsigned char *)((long)v49 + 0x32) + (unsigned int)*(unsigned char *)((long)v49 + 0x34) + (unsigned int)*(unsigned char *)((long)v49 + 0x35) + (unsigned int)*(unsigned char *)((long)v49 + 0x36) + (unsigned int)(unsigned char)((unsigned char)v49[7] | *(unsigned char *)((long)v49 + 0x33) | v49[4] != 0)) {
        *(char *)((long)v49 + 0x37) = 0;
        v72 = &v54;
        *(unsigned short *)&v49[6] = 0;
        sub_7140(v49,v72);
        sub_7310(v72); // no-return
      }
      v49 = (unsigned long *)v49[8];
label_58ff:
    } while (v49);
    if (dat_1d400) {
label_4775:
      if (v12 || v96) {
        s_1d018[0] = v12;
        if (!v12)
          s_1d018[0] = 'o';
        sub_7310(s_1d018); // no-return
      }
      if ((!v93) || (!setlocale(3,"")))
        error(0,0,"%s",dcgettext(NULL,"failed to set locale",5));
      if (dat_1d861) { // branch-flip
        v74 = (char *)sub_10fd0(setlocale(3,NULL));
        error(0,0,dcgettext(NULL,"text ordering performed using %s sorting rules",5),v74);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"text ordering performed using simple byte comparison",5));
      }
      v29 = 0x12;
      v68 = &v53;
      v71 = &v55;
      while (v29) {
        v3 = (void **)((long)v71 + 4);
        v2 = (unsigned long *)((long)v68 + 4);
        *(unsigned int *)v71 = *(unsigned int *)v68;
        v29 -= 1;
        v68 = v2;
        v71 = v3;
      }
      if (dat_1d410) {
        v100 = 0;
        v84 = 0;
        v13 = '\0';
        v83 = 0;
        v94 = (void **)0x1;
        v49 = dat_1d410;
        v80 = v115;
        v78 = v119;
        v75 = v121;
        v66 = v122;
        do {
          v11 = v120;
          v45 = v118;
          v10 = v117;
          v9 = v116;
          v46 = v114;
          if (v49[6] & 0xffff00ff0000) {
            v67 = *(char *)((long)v49 + 0x34);
            if (!v67) {
              v83 = 1;
              v67 = v13;
            }
            v13 = v67;
          }
          if (*(char *)((long)v49 + 0x39)) {
            v31 = *v49;
            if (*v49 == 0xffffffffffffffff)
              v31 = 0;
            v79 = v49[2];
            v74 = (char *)sub_e200(v31,v56);
            v58 = 0x2b;
            v74 = __stpcpy_chk(v57,v74,0x2d);
            v69 = (char *)sub_e200(v31 + 1,v56);
            v54 = 0x206b2d;
            v69 = __stpcpy_chk((char *)((long)&v54 + 3),v69,0x2c);
            if (v49[2] != 0xffffffffffffffff) {
              v41 = (char *)sub_e200(v79 + 1,v56);
              v74[0] = ' ';
              v74[1] = '-';
              strcpy(&v74[2],v41);
              v74 = (char *)sub_e200((unsigned long)(v49[3] == 0xffffffffffffffff) + v79 + 1,v56);
              *v69 = ',';
              strcpy(&v69[1],v74);
            }
            v74 = (char *)sub_10fb0(1,&v54);
            v69 = (char *)sub_10fb0(0,&v58);
            error(0,0,dcgettext(NULL,"obsolescent key %s used; consider %s instead",5),v69,v74);
          }
          v31 = v49[2];
          v7 = v85;
          if ((*v49 != 0xffffffffffffffff) && (v31 < *v49)) {
            error(0,0,dcgettext(NULL,"key %lu has zero width and will be ignored",5),(unsigned long)v94);
            v31 = v49[2];
            v7 = 1;
          }
          v67 = 1;
          v79 = v49[6] & 0xffff00ff0000;
          if (!v79)
            v67 = *(char *)((long)v49 + 0x36);
          if ((v31) || (!v49[3])) {
            if (!v7) {
              if ((dat_1d028 == 0x80) && (((!(char)v49[6] && ((!v67 || (v49[1])))) || ((!*(char *)((long)v49 + 0x31) && (v49[3])))))) {
                error(0,0,dcgettext(NULL,"leading blanks are significant in key %lu; consider also specifying \'b\'",5),(unsigned long)v94);
                v79 = v49[6] & 0xffff00ff0000;
              }
              goto label_48e3;
            }
label_5a20:
            if (!v85) goto label_48e3;
label_4908:
            v14 = *(unsigned char *)((long)v49 + 0x34);
          }
          else {
            if (v7) goto label_5a20;
label_48e3:
            if (!v79) goto label_4908;
            v31 = *v49 + 1;
            if (!v31)
              v31 = 1;
            if (v49[2] < v31) goto label_4908;
            error(0,0,dcgettext(NULL,"key %lu is numeric and spans multiple fields",5),(unsigned long)v94);
            v14 = *(unsigned char *)((long)v49 + 0x34);
            if (v14) // branch-flip
              v100 = v14;
            else {
              v84 = 1;
            }
          }
          if ((v112) && (v112 == v49[4]))
            v112 = 0;
          if ((v113) && (v113 == v49[5]))
            v113 = 0;
          v118 = v45 & (v14 ^ 1);
          v45 = (unsigned char)v49[6] ^ 1;
          v114 = v46 & v45;
          v46 = *(unsigned char *)((long)v49 + 0x31) ^ 1;
          v78 &= *(unsigned char *)((long)v49 + 0x35) ^ 1;
          v80 &= v46;
          v117 = v10 & (*(unsigned char *)((long)v49 + 0x33) ^ 1);
          v46 = *(unsigned char *)((long)v49 + 0x36) ^ 1;
          v120 = v11 & v46;
          v122 = v66 & ((unsigned char)v49[7] ^ 1);
          v66 = *(unsigned char *)((long)v49 + 0x32) ^ 1;
          v116 = v9 & v66;
          v75 &= *(unsigned char *)((long)v49 + 0x37) ^ 1;
          v49 = (unsigned long *)v49[8];
          v94 = (unsigned long)((long)v94 + 1);
          v115 = v80;
          v119 = v78;
          v121 = v75;
          v66 = v122;
        } while (v49);
        if (v84) {
          if (dat_1d028 != 0x80) {
            if (dat_1d028 == dat_1d864) {
label_6381:
              v54._0_2_ = (unsigned short)(unsigned char)dat_1d864;
              v74 = (char *)sub_10fd0(&v54);
              error(0,0,dcgettext(NULL,"field separator %s is treated as a group separator in numbers",5),v74);
              goto label_63c4;
            }
            v84 = 0;
            goto label_5aa7;
          }
          if (dat_1d864 != 0x80) { // branch-flip
            v29 = *v27;
            if (*(unsigned char *)(v29 + ((unsigned long)dat_1d864 & 0xff) * 2) & 1) goto label_6381;
            v84 = 0;
label_640e:
            if (*(unsigned char *)(v29 + (unsigned long)dat_1d868 * 2) & 1) goto label_641f;
          }
          else {
            v84 = 0;
          }
          goto label_5b0c;
        }
        if (v100) { // branch-flip
label_63c4:
          if (dat_1d028 == 0x80) {
            if (dat_1d864 != 0x80) {
              v29 = *v27;
              goto label_640e;
            }
            goto label_5b0c;
          }
label_5aa7:
          if (dat_1d028 != (int)(char)dat_1d868) { // branch-flip
            if (dat_1d028 != 0x2d) { // branch-flip
              if (dat_1d028 == 0x2b) {
                if (v100) {
                  v54 = CONCAT22(v54._2_2_,0x2b);
                  v39 = sub_10fd0(&v54);
                  v74 = "field separator %s is treated as a plus sign in numbers";
                  goto label_5af4;
                }
              }
            }
            else {
              v54 = CONCAT22(v54._2_2_,0x2d);
              v39 = sub_10fd0(&v54);
              v74 = "field separator %s is treated as a minus sign in numbers";
label_5af4:
              error(0,0,dcgettext(NULL,v74,5),v39);
            }
label_5b0c:
            if (!v83) {
              if (!v13) goto label_5b59;
            }
            if (!v84) goto label_5d6d;
          }
          else {
label_641f:
            v54 = CONCAT31((undefined3)(v54 >> 8),dat_1d868) & 0xffff00ff;
            v74 = (char *)sub_10fd0(&v54);
            error(0,0,dcgettext(NULL,"field separator %s is treated as a decimal point in numbers",5),v74);
          }
        }
        else {
          if (!v83 && !v13) goto label_5b59;
label_5d6d:
          v69 = "";
          v54 = CONCAT31((undefined3)(v54 >> 8),dat_1d868) & 0xffff00ff;
          v74 = (char *)sub_10fd0(&v54);
          if ((int)(char)dat_1d868 != dat_1d028)
            v69 = dcgettext(NULL,"note ",5);
          error(0,0,dcgettext(NULL,"%snumbers use %s as a decimal point in this locale",5),v69,v74);
        }
        if (v83) {
          if (dat_1d862)
            error(0,0,dcgettext(NULL,"the multi-byte number group separator in this locale is not supported",5));
        }
      }
label_5b59:
      v13 = sub_70e0(&v55);
      v80 = v121;
      if (v13) { // branch-flip
        v121 = v80;
        if (v121) {
          if (dat_1d41a) { // branch-flip
            if (dat_1d410) {
              sub_7140(&v55,&v54);
              v31 = strlen((char *)&v54);
              error(0,0,dcngettext(NULL,"option \'-%s\' is ignored","options \'-%s\' are ignored",v31,5),&v54);
              v121 = 1;
label_5cd4:
              if (!dat_1d41a) goto label_5ce1;
            }
          }
          else {
            if ((dat_1d419) && (dat_1d410)) {
              sub_7140(&v55,&v54);
              v31 = strlen((char *)&v54);
              error(0,0,dcngettext(NULL,"option \'-%s\' is ignored","options \'-%s\' are ignored",v31,5),&v54);
              v121 = v80;
              goto label_5cd4;
            }
label_5ce1:
            if ((!dat_1d419) && (dat_1d410))
              error(0,0,dcgettext(NULL,"option \'-r\' only applies to last-resort comparison",5));
          }
        }
      }
      else {
        if ((!dat_1d41a) && (!dat_1d419))
          v121 = 0;
        sub_7140(&v55,&v54);
        v31 = strlen((char *)&v54);
        error(0,0,dcngettext(NULL,"option \'-%s\' is ignored","options \'-%s\' are ignored",v31,5),&v54);
        v121 = v80;
        if (v80) goto label_5cd4;
      }
    }
    dat_1d41b = v107[0xf];
    if (v47) {
      v28 = (long *)sub_11060(v98,0x10);
      if (!v28) {
        if (!v98)
          v98 = "getrandom";
        v74 = "open failed";
        sub_7480(dcgettext(NULL,v74,5),v98); // return-dupe, no-return
      }
      sub_11220(v28,&v54,0x10);
      if (sub_113c0(v28)) {
        v74 = "close failed";
        sub_7480(dcgettext(NULL,v74,5),v98);
      }
      MD5_Init(0x1d300);
      MD5_Update(0x1d300,&v54,0x10);
    }
  }
  else {
    if (!sub_70e0(&v53)) {
      sub_76e0();
      v85 = 1;
      v47 = v107[0xb];
      v49 = dat_1d410;
      goto label_58ff;
    }
    if (dat_1d400) {
      v85 = 0;
      v47 = 0;
      goto label_4775;
    }
    dat_1d41b = v107[0xf];
  }
  if (!dat_1d428) {
    v74 = getenv("TMPDIR");
    if (!v74)
      v74 = "/tmp";
    sub_7730(v74);
  }
  if (!v81) {
    v81 = 1;
    free(v33);
    v33 = (unsigned long *)sub_12ab0(8);
    *v33 = "-";
  }
  if (dat_1d438) {
    v34 = (void *)((unsigned long)dat_1d024 * 0x22);
    if ((void *)((unsigned long)dat_1d024 * 0x22) < dat_1d438)
      v34 = dat_1d438;
    dat_1d438 = v34;
  }
  if (v12) {
    if (v81 != 1) {
      v74 = (char *)sub_108e0(4,(char *)v33[1]);
      error(2,0,dcgettext(NULL,"extra operand %s not allowed with -%c",5),v74,v12);
      return v25;
    }
    if (v96) {
      dat_1d010 = v12;
      sub_7310((char *)0x1d010); // no-return
    }
    v74 = (char *)*v33;
    v42 = (FILE *)sub_74e0(v74,"r");
    v49 = dat_1d410;
    if (!v42)
      sub_7c60(v74); // no-return
    v48 = (unsigned int)dat_1d419;
    v34 = dat_1d030;
    if (dat_1d030 <= dat_1d438)
      v34 = dat_1d438;
    v31 = 0;
    sub_7790(&v55,0x20,v34);
    v52 = NULL;
    v92 = NULL;
    do {
      v13 = sub_7ef0(&v55,v42,v74);
      v81 = v110;
      if (!v13) {
        v48 = 1;
label_5e04:
        sub_7660(v42,v74);
        free(v55);
        free(v52);
        exit(v48 ^ 1); // no-return
      }
      v28 = (long *)(v111 + (long)v55);
      v29 = v110 * -4;
      if ((v31) && ((int)(v48 ^ 1) <= (int)sub_a0b0(&v52,(struct_11 *)&v28[-4]))) {
        v27 = v28;
label_5df7:
        v48 = 0;
        if (v12 == 'c') {
          v41 = (char *)sub_e200(((long)v28 - (long)&v27[-4] >> 5) + (long)v92,&v54);
          v69 = dat_1d888;
          __fprintf_chk(stderr,1,dcgettext(NULL,"%s: %s:%s: disorder: ",5),v69,v74,v41);
          sub_82d0(&v27[-4],stderr,dcgettext(NULL,"standard error",5));
        }
        goto label_5e04;
      }
      v77 = &v28[-4];
      while (v27 = v77, &v28[v29] < v27) {
        v77 = &v27[-4];
        if ((int)(v48 ^ 1) <= (int)sub_a0b0(v27,&v27[-4])) goto label_5df7;
      }
      v79 = v27[1];
      v92 = (long)((long)v92 + v81);
      if (v31 < v79) {
        do {
          v31 *= 2;
          if (!v31) {
            v31 = v79;
            break;
          }
        } while (v31 < v79);
        free(v52);
        v52 = (void *)sub_12ab0(v31);
        v79 = v27[1];
      }
      v34 = memcpy(v52,(void *)*v27,v79);
      v101 = v27[1];
      if (v49) {
        v102 = (long)v34 + (v27[2] - *v27);
        v103 = (long)v34 + (v27[3] - *v27);
      }
    } while( true );
  }
  v95 = 0;
  v50 = v33;
  do {
    v74 = (char *)*v50;
    if ((strcmp(v74,"-")) && (euidaccess(v74,4))) {
      v74 = (char *)*v50;
      v69 = "cannot read";
      sub_7480(dcgettext(NULL,v69,5),v74); // return-dupe, no-return
    }
    v95 += 1;
    v50 = &v50[1];
  } while (v81 != v95);
  if (v96) {
    v15 = open(v96,0x80041,0x1b6);
    if (v15 < 0)
      sub_7480(dcgettext(NULL,"open failed",5),v96);
    if (v15 != 1)
      sub_7c90(v15,1);
  }
  if (!v8) {
    if (!v99) {
      v31 = sub_e570(2);
      v99 = 8;
      if (v31 <= 8)
        v99 = v31;
    }
    v91 = 0;
    v111 = 0;
    v31 = 0xffffffffffffff;
    if (v99 <= 0xffffffffffffff)
      v31 = v99;
    v81 = v31 * 2;
label_5328:
    v74 = (char *)*v33;
    v42 = (FILE *)sub_74e0(v74,"r");
    if (!v42)
      sub_7c60(v74); // no-return
    v97 = 0x30;
    if (2 <= v99) {
      v97 = 1;
      v79 = 1;
      do {
        v79 *= 2;
        v97 += 1;
      } while (v79 < v31);
      v97 *= 0x20;
    }
    if (!v111) {
      v34 = (void *)(v97 + 2);
      v79 = 0;
      do {
        if (v79) { // branch-flip
          v69 = (char *)v33[v79];
          v85 = (strcmp(v69,"-")) ? stat(v69,(stat *)(sigaction *)&v51) != 0 : fstat(0,(stat *)(sigaction *)&v51) != 0; // branch-flip
        }
        else {
          v15 = fileno(v42);
          v85 = fstat(v15,(stat *)(sigaction *)&v51) != 0;
        }
        if (v85) {
          v74 = (char *)v33[v79];
          v69 = "stat failed";
          sub_7480(dcgettext(NULL,v69,5),v74);
        }
        if ((v65._16_4_ & 0xf000) != 0x8000) { // branch-flip
          v38 = dat_1d438;
          if (dat_1d438) goto label_60e5;
          v93 = (char *)0x20000;
          v38 = dat_1d240;
          if (!dat_1d240) {
            v93 = (char *)0x20000;
label_5f41:
            v38 = (void *)0xffffffffffffffff;
            if (!getrlimit(2))
              v38 = v52;
            if ((!getrlimit(9)) && (v52 < v38))
              v38 = v52;
            v38 = (void *)((unsigned long)v38 >> 1);
            if (!getrlimit(5,&v52)) {
              v40 = (void *)(((unsigned long)v52 >> 4) * 0xf);
              if (v40 < v38)
                v38 = v40;
            }
            sub_e850();
            sub_e760();
            v88 = v87 * dat_17110;
            v90 = v86;
            if (v86 <= dat_17108 * v87)
              v90 = dat_17108 * v87;
            if (v88 < (double)(long)v38) {
              v38 = (dat_17100 <= v88) ? (void *)((long)(v88 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v88;
            }
            if (v90 < (double)v38) {
              v38 = (dat_17100 <= v90) ? (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v90;
            }
            dat_1d240 = (void *)((unsigned long)dat_1d024 * 0x22);
            if ((void *)((unsigned long)dat_1d024 * 0x22) < v38)
              dat_1d240 = v38;
            v38 = dat_1d240;
          }
        }
        else {
          v93 = (char *)v61[0];
          v38 = dat_1d240;
          if (!dat_1d240) {
            dat_1d240 = dat_1d438;
            v38 = dat_1d438;
            if (!dat_1d438) goto label_5f41;
          }
        }
        v44 = (long)v93 * (v97 + 1U) + 1;
        if ((v93 != (char *)(v44 / (v97 + 1U))) || ((unsigned long)((long)v38 - (long)v34) <= v44)) goto label_60e5;
        v34 = (void *)((long)v34 + v44);
        v79 += 1;
        v38 = v34;
        if (v79 >= v95) goto label_60e5;
      } while( true );
    }
    goto label_5385;
  }
  v35 = (unsigned long *)sub_12e90(v95,0x10);
  v50 = &v33[v95];
  v36 = v35;
  do {
    v39 = *v33;
    v33 = &v33[1];
    v37 = &v36[2];
    *v36 = v39;
    v36 = v37;
  } while (v50 != v33);
  sub_ac90(v35,0,v95,v96);
label_55b4:
  if ((dat_1d418) && (v15 = sub_c5b0(stdin), v15 == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  if (v126 != *(long *)(v82 + 0x28))
    __stack_chk_fail(); // no-return
  return 0;
}


// Function: sub_6f40 @ 0x6f40
unsigned long sub_6f40(void *a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)((long)a0 + 8) % a1;
}


// Function: sub_6f60 @ 0x6f60
unsigned int sub_6f60(void *a0,void *a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 8) >> 8),*(int *)((long)a0 + 8) == *(int *)((long)a1 + 8));
}


// Function: sub_6f70 @ 0x6f70
int sub_6f70(long *a0)
{
  char *v1;
  char *v2;
  unsigned int v3; // ecx
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
      if (*v1 != dat_1d864)
        v2 = v5;
      v4 = *v2;
      v3 = (int)v4;
      v5 = &v2[1];
    } while ((unsigned int)(v3 - 0x30U) <= 9);
    if (*v1 == dat_1d864) {
      *a0 = (long)&v2[-1];
      return v6;
    }
  }
  if (dat_1d868 != v4) // branch-flip
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


// Function: sub_7030 @ 0x7030
unsigned int sub_7030(unsigned char *a0,unsigned long *a1) // early-return
{
  char v1;
  unsigned char v2;
  unsigned char *v3;
  unsigned char *v4;
  unsigned long v5; // r9
  unsigned long v6;
  unsigned long v7;
  
  v1 = *(char *)((unsigned long)*a0 + 0x1d740);
  while (v1) {
    v4 = &a0[1];
    a0 = &a0[1];
    v1 = *(char *)((unsigned long)*v4 + 0x1d740);
  }
  v6 = 0xc;
  v7 = 0;
  do {
    v5 = v7 + v6 >> 1;
    v4 = *(unsigned char **)(v5 * 0x10 + 0x1d060);
    v2 = *v4;
    v3 = a0;
    while( true ) {
      if (!v2) {
        if (a1)
          *a1 = v3;
        return *(unsigned int *)(v5 * 0x10 + 0x1d068);
      }
      if (*(unsigned char *)((unsigned long)*v3 + 0x1d440) < v2) {
        v6 = v5;
        goto label_70ae;
      }
      if (v2 < *(unsigned char *)((unsigned long)*v3 + 0x1d440)) break;
      v2 = v4[1];
      v4 = &v4[1];
      v3 = &v3[1];
    }
    v7 = v5 + 1;
label_70ae:
    if (v6 <= v7)
      return 0;
  } while( true );
}


// Function: sub_70e0 @ 0x70e0
unsigned char sub_70e0(void *a0)
{
  if ((!*(long *)((long)a0 + 0x20)) && (!*(long *)((long)a0 + 0x28))) {
    if (*(char *)((long)a0 + 0x30))
      return 0;
    if ((!*(char *)((long)a0 + 0x31)) && (!(*(unsigned long *)((long)a0 + 0x30) & 0xffff00ff0000))) {
      if (*(char *)((long)a0 + 0x36))
        return 0;
      if (!*(char *)((long)a0 + 0x38))
        return *(unsigned char *)((long)a0 + 0x33) ^ 1;
    }
  }
  return 0;
}


// Function: sub_7140 @ 0x7140
void sub_7140(void *a0,char *a1)
{
  if (*(short *)((long)a0 + 0x30)) {
    *a1 = 0x62;
    a1 = &a1[1];
  }
  if (*(long *)((long)a0 + 0x20) == 0x1d540) {
    *a1 = 100;
    a1 = &a1[1];
  }
  if (*(long *)((long)a0 + 0x28)) {
    *a1 = 0x66;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x34)) {
    *a1 = 0x67;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x35)) {
    *a1 = 0x68;
    a1 = &a1[1];
  }
  if (*(long *)((long)a0 + 0x20) == 0x1d640) {
    *a1 = 0x69;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x36)) {
    *a1 = 0x4d;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x32)) {
    *a1 = 0x6e;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x33)) {
    *a1 = 0x52;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x37)) {
    *a1 = 0x72;
    a1 = &a1[1];
  }
  if (*(char *)((long)a0 + 0x38)) {
    *a1 = 0x56;
    a1 = &a1[1];
  }
  *a1 = 0;
}


// Function: sub_71f0 @ 0x71f0
char * sub_71f0(char *a0,void *a1,unsigned int a2)
{
  char v1;
  
  v1 = *a0;
  if (v1) {
    do {
      switch(v1) {
        case 0x4d:
          *(char *)((long)a1 + 0x36) = 1;
          break;
        default:
          goto label_724d;
        case 0x52:
          *(char *)((long)a1 + 0x33) = 1;
          break;
        case 0x56:
          *(char *)((long)a1 + 0x38) = 1;
          break;
        case 0x62:
          if (!(a2 & 0xfffffffd))
            *(char *)((long)a1 + 0x30) = 1;
          if (a2 - 1 <= 1)
            *(char *)((long)a1 + 0x31) = 1;
          break;
        case 100:
          *(unsigned long *)((long)a1 + 0x20) = 0x1d540;
          break;
        case 0x66:
          *(unsigned long *)((long)a1 + 0x28) = 0x1d440;
          break;
        case 0x67:
          *(char *)((long)a1 + 0x34) = 1;
          break;
        case 0x68:
          *(char *)((long)a1 + 0x35) = 1;
          break;
        case 0x69:
          if (!*(long *)((long)a1 + 0x20))
            *(unsigned long *)((long)a1 + 0x20) = 0x1d640;
          break;
        case 0x6e:
          *(char *)((long)a1 + 0x32) = 1;
          break;
        case 0x72:
          *(char *)((long)a1 + 0x37) = 1;
        
      }
      v1 = a0[1];
      a0 = &a0[1];
    } while (v1);
  }
label_724d:
  return a0;
}


// Function: sub_7300 @ 0x7300
void sub_7300(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}


// Function: sub_7310 @ 0x7310
void sub_7310(char *a0)
{
  error(2,0,dcgettext(NULL,"options \'-%s\' are incompatible",5),a0);
}


// Function: sub_7340 @ 0x7340
void sub_7340(char *a0,char *a1)
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = (char *)sub_10fd0(a0);
  v2 = dcgettext(NULL,a1,5);
  error(2,0,dcgettext(NULL,"%s: invalid field specification %s",5),v2,v1);
}


// Function: sub_7390 @ 0x7390
char * sub_7390(unsigned char *a0,unsigned long *a1,char *a2)
{
  char *v1;
  char *v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  
  switch(sub_132f0(a0,&v5,10,&v4,"")) {
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
      v3 = (char *)sub_10fd0(a0);
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


// Function: sub_7480 @ 0x7480
void sub_7480(char *a0,char *a1)
{
  char *v1; // rax
  
  if (!a1)
    a1 = dcgettext(NULL,"standard output",5);
  v1 = (char *)sub_10c60(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1);
}


// Function: sub_74e0 @ 0x74e0
FILE * sub_74e0(char *a0,char *a1)
{
  int v1;
  int *v2;
  char *v3;
  int v4;
  FILE *v5; // rax
  FILE *v6;
  
  if (*a1 == 'r') {
    v4 = strcmp(a0,"-");
    if (v4) { // branch-flip
      v6 = NULL;
      v4 = open(a0,0x80000);
      if (0 <= v4) {
        v5 = fdopen(v4,a1);
        v6 = v5;
      }
    }
    else {
      dat_1d418 = 1;
      v6 = stdin;
    }
    sub_c580(v6,2);
    v5 = v6;
    return v5;
  }
  if (*a1 != 'w')
    __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
  if (!a0) {
    v5 = stdout;
    return v5;
  }
  v4 = ftruncate(1,0);
  if (!v4) {
    v5 = stdout;
    return v5;
  }
  v5 = (int *)__errno_location();
  v2 = (int *)v5;
  v1 = *(int *)v5;
  v4 = dat_1d2f0;
  if (dat_1d2f0) { // branch-flip
label_7579:
    if (0 <= v4) goto label_7593;
  }
  else {
    v4 = fstat(1,(stat *)0x1d260);
    if (v4) {
      v4 = *v2;
      dat_1d2f0 = v4;
      goto label_7579;
    }
    dat_1d2f0 = -1;
  }
  v4 = dat_1d278 & 0xf000;
  if (v4 != 0x8000) {
    v5 = stdout;
    return v5;
  }
label_7593:
  v5 = (char *)sub_10c60(0,3,a0);
  v3 = (char *)v5;
  v5 = (char *)dcgettext(NULL,"%s: error truncating",5);
  error(2,v1,(char *)v5,v3);
  return v5;
}


// Function: sub_7660 @ 0x7660
void sub_7660(FILE *a0,char *a1) // return-dupe
{
  int v1; // eax
  char *v2; // rsi
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0); // tail-call
    return;
  }
  if (v1 != 1) { // branch-flip
    if (!sub_c5b0(a0))
      return;
    v2 = "close failed";
  }
  else {
    if (!fflush_unlocked(a0))
      return;
    v2 = "fflush failed";
  }
  sub_7480(dcgettext(NULL,v2,5),a1); // no-return
}


// Function: sub_76e0 @ 0x76e0
void sub_76e0(void *a0)
{
  long v1;
  long v2; // rax
  long *v3; // rcx
  long v4; // rdx
  
  v2 = sub_12ef0(a0,0x48);
  v3 = (long *)0x1d410;
  v1 = dat_1d410;
  if (dat_1d410) {
    do {
      v4 = v1;
      v1 = *(long *)(v4 + 0x40);
    } while (v1);
    v3 = (long *)(v4 + 0x40);
  }
  *v3 = v2;
  *(unsigned long *)(v2 + 0x40) = 0;
}


// Function: sub_7730 @ 0x7730
void sub_7730(char *a0)
{
  long v1;
  
  v1 = dat_1d430;
  if (dat_1d428 == dat_1d420) {
    v1 = sub_12d00(dat_1d430,(unsigned long *)0x1d420,8);
    dat_1d430 = v1;
  }
  *(char **)(v1 + dat_1d428 * 8) = a0;
  dat_1d428 += 1;
}


// Function: sub_7800 @ 0x7800
void sub_7800(char *a0,char *a1,unsigned long a2)
{
  int *v1; // rax
  char *v2; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  strxfrm(a0,a1,a2);
  if (!*v1)
    return;
  v2 = dcgettext(NULL,"string transformation failed",5);
  error(0,*v1,v2);
  error(0,0,dcgettext(NULL,"set LC_ALL=\'C\' to work around the problem",5));
  v2 = (char *)sub_107b0(0,8,a1);
  error(2,0,dcgettext(NULL,"the untransformed string was %s",5),v2);
}


// Function: sub_78c0 @ 0x78c0
unsigned long sub_78c0(int a0)
{
  unsigned int v1; // eax
  unsigned int v10; // stack - 0x30
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned int v7; // stack - 0x3c
  char v8 [8];
  int v9;
  
  v9 = -1;
  if (a0)
    v9 = a0;
  v1 = waitpid(v9,(int *)&v7,(unsigned int)(a0 == 0));
  if ((int)v1 < 0) {
    v3 = sub_108e0(4,dat_1d408);
    v5 = dcgettext(NULL,"waiting for %s [-d]",5);
    error(2,*__errno_location(),v5,v3);
    return v4;
  }
  if (v1) {
    if (a0 <= 0) {
      v10 = v1;
      v2 = sub_dd00(dat_1d360,v8);
      if (!v2) {
        return (unsigned long)v1; // return-dupe
      }
      *(char *)(v2 + 0xc) = 2;
    }
    if (v7 >> 8 & 0xff || v7 & 0x7f) {
      v5 = (char *)sub_108e0(4,dat_1d408);
      error(2,0,dcgettext(NULL,"%s [-d] terminated abnormally",5),v5);
      return v6;
    }
    dat_1d35c -= 1;
  }
  return (unsigned long)v1;
}


// Function: sub_79f0 @ 0x79f0
void sub_79f0(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_e120(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}


// Function: sub_7a90 @ 0x7a90
void sub_7a90(void *a0)
{
  long v1;
  
  v1 = dat_1d360;
  if (!dat_1d360) {
    v1 = sub_d640(0x2f,NULL,sub_6f40,sub_6f60,0);
    dat_1d360 = v1;
    if (!v1) {
      sub_12fb0(); // no-return, return-dupe
    }
  }
  *(char *)((long)a0 + 0xc) = 1;
  if (sub_dca0(v1,a0))
    return;
  sub_12fb0();
}


// Function: sub_7af0 @ 0x7af0
long * sub_7af0(long *a0,long *a1,long a2,unsigned long a3,long a4,char a5)
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
    pthread_mutex_init((pthread_mutex_t *)&v5[0xb],NULL);
    if (a3 <= 1) break;
    v5[8] = (long)v1;
    v3 = a3 >> 1;
    a3 -= v3;
    a1 = (long *)sub_7af0(v5,v1,v7,v3,a4,1);
    v5[9] = (long)a1;
    a5 = 0;
    a0 = v5;
  }
  v5[8] = 0;
  v5[9] = 0;
  return v1;
}


// Function: sub_7ef0 @ 0x7ef0
unsigned long sub_7ef0(struct_8 *a0,FILE *a1,char *a2)
{
  long v1;
  unsigned long v10;
  unsigned char *v11;
  unsigned char *v12;
  unsigned char *v13;
  long v14;
  unsigned char *v15; // r15
  unsigned char *v16; // stack - 0x88
  unsigned char v2;
  long *v3;
  unsigned long v4;
  unsigned long v5; // rax
  char *v6; // rax
  long v7;
  unsigned char *v8;
  unsigned long v9; // stack - 0x48
  
  v3 = dat_1d410;
  v2 = dat_1d038;
  v14 = dat_1d030;
  v1 = a0->field_0x28;
  if (a0->field_0x30)
    return 0;
  v4 = a0->field_0x8;
  v10 = a0->field_0x20;
  if (v4 != v10) { // branch-flip
    memmove(a0->field_0x0,&a0->field_0x0[v4 - v10],v10);
    v4 = a0->field_0x20;
    a0->field_0x10 = 0;
    v7 = 0;
    a0->field_0x8 = v4;
  }
  else {
    v7 = a0->field_0x10;
  }
  v10 = v14 - 0x22;
  v11 = a0->field_0x0;
  v14 = a0->field_0x18;
  do {
    v12 = &v11[v4];
    v13 = &v11[v7 * -0x20 + v14];
    v8 = &v11[(v14 - v1 * v7) - (long)v12];
    if (v7)
      v11 = (unsigned char *)(*(long *)&v13[8] + *(long *)v13);
    do {
      if (v8 <= (unsigned char *)(v1 + 1)) {
        v16 = v12;
        break;
      }
      v4 = (unsigned long)&v8[-1] / (unsigned long)(unsigned char *)(v1 + 1);
      v5 = fread_unlocked(v12,1,v4,a1);
      v8 = &v8[-v5];
      v16 = &v12[v5];
      if (v4 != v5) {
        if (*(unsigned int *)a1 & 0x20)
          sub_7480(dcgettext(NULL,"read failed",5),a2); // no-return
        if (*(unsigned int *)a1 & 0x10) {
          a0->field_0x30 = 1;
          if (a0->field_0x0 == v16)
            return 0;
          if ((v11 != v16) && (v16[-1] != v2)) {
            *v16 = v2;
            v16 = &v16[1];
          }
        }
      }
      while (v6 = memchr(v12,(int)(char)v2,(long)v16 - (long)v12), v6) {
        v12 = &v6[1];
        v15 = &v13[-0x20];
        *v6 = 0;
        *(unsigned char **)v15 = v11;
        v4 = (long)v12 - (long)v11;
        *(unsigned long *)&v13[-0x18] = v4;
        if (v10 < v4)
          v10 = v4;
        v8 = &v8[-v1];
        if (v3) {
          if (v3[2] != -1)
            v6 = (char *)sub_7ca0(v11,v4,v3);
          v14 = *v3;
          *(char **)&v13[-8] = v6;
          if (v14 != -1) // branch-flip
            *(unsigned long *)&v13[-0x10] = sub_7dd0(v11,v4,v3);
          else {
            if ((char)v3[6]) {
              while (*(char *)((unsigned long)*v11 + 0x1d740)) {
                v11 = &v11[1];
              }
            }
            *(unsigned char **)&v13[-0x10] = v11;
          }
        }
        v11 = v12;
        v13 = v15;
      }
      v12 = v16;
    } while (!a0->field_0x30);
    v12 = a0->field_0x0;
    a0->field_0x8 = (long)v16 - (long)v12;
    a0->field_0x10 = (long)&v12[a0->field_0x18 - (long)v13] >> 5;
    if ((long)&v12[a0->field_0x18 - (long)v13] >> 5) {
      a0->field_0x20 = (long)v16 - (long)v11;
      dat_1d030 = v10 + 0x22;
      return 1;
    }
    v9 = (unsigned long)a0->field_0x18 >> 5;
    v11 = (unsigned char *)sub_12d00(v12,&v9,0x20);
    v14 = v9 << 5;
    a0->field_0x0 = v11;
    v4 = a0->field_0x8;
    a0->field_0x18 = v14;
    v7 = a0->field_0x10;
  } while( true );
}


// Function: sub_8270 @ 0x8270
void sub_8270(int a0)
{
  unsigned long *v1;
  
  for (v1 = dat_1d368; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_1d368 = NULL;
  signal(a0,0);
  raise(a0); // tail-call
}


// Function: sub_82d0 @ 0x82d0
void sub_82d0(struct_9 *a0,FILE *a1,char *a2)
{
  char v1;
  long *v10;
  char *v11;
  long v12; // r14
  unsigned long v13;
  long v14;
  char *v2;
  char v3;
  char *v4;
  char *v5;
  unsigned int v6; // ecx
  char *v7;
  char *v8; // stack - 0x48
  char *v9; // stack - 0x50
  
  v7 = a0->field_0x0;
  v13 = a0->field_0x8;
  v5 = &v7[v13];
  if ((a2) || (!dat_1d400)) {
    v5[-1] = dat_1d038;
    if (v13 == fwrite_unlocked(v7,1,v13,a1)) {
      v5[-1] = '\0';
      return;
    }
    sub_7480(dcgettext(NULL,"write failed",5),a2); // no-return
  }
  if (v7 < v5) {
    do {
      v3 = *v7;
      v7 = &v7[1];
      if (v3 != '\t') { // branch-flip
        v6 = (unsigned int)v3;
        if (v5 == v7) {
          v6 = 10;
          v3 = '\n';
        }
        v4 = *(char **)&a1->field_0x28;
        if (*(char **)&a1->field_0x30 <= v4) goto label_83be;
label_838c:
        *(char **)&a1->field_0x28 = &v4[1];
        *v4 = v3;
      }
      else {
        v4 = *(char **)&a1->field_0x28;
        v6 = 0x3e;
        v3 = '>';
        if (v4 < *(char **)&a1->field_0x30) goto label_838c;
label_83be:
        if (__overflow(a1,v6 & 0xff) == -1)
          sub_7480(dcgettext(NULL,"write failed",5),NULL); // no-return
      }
    } while (v5 != v7);
    v7 = a0->field_0x0;
    v13 = a0->field_0x8;
  }
  v5 = &v7[v13 - 1];
  v10 = dat_1d410;
  v4 = v7;
  if (!dat_1d410) goto label_8642;
  do {
    v14 = *v10;
    if (v14 != -1) { // branch-flip
      v4 = (char *)sub_7dd0(v7,v13,v10);
      if (v10[2] != -1) {
label_844e:
        v5 = (char *)sub_7ca0(v7,v13,v10);
        goto label_845f;
      }
label_846b:
      if ((*(char *)((long)v10 + 0x36)) || (v10[6] & 0xffff00ff0000U)) goto label_8485;
    }
    else {
      v4 = v7;
      if (v10[2] != -1) goto label_844e;
label_845f:
      if ((v14 != -1) || (!(char)v10[6])) goto label_846b;
label_8485:
      v3 = *v5;
      *v5 = '\0';
      v1 = *(char *)((unsigned long)(unsigned char)*v4 + 0x1d740);
      while (v1) {
        v11 = &v4[1];
        v4 = &v4[1];
        v1 = *(char *)((unsigned long)(unsigned char)*v11 + 0x1d740);
      }
      if (v4 <= v5) { // branch-flip
        v9 = v4;
        if (*(char *)((long)v10 + 0x36)) // branch-flip
          sub_7030(v4,&v9);
        else if (*(char *)((long)v10 + 0x34)) // branch-flip
          strtold(v4,&v9);
        else {
          if (!(v10[6] & 0xff0000ff0000U)) goto label_8670;
          v8 = v4;
          if (v4 < v5)
            v8 = &v4[*v4 == '-'];
          if ('0' <= (char)sub_6f70(&v8)) {
            v9 = v8;
            if (*(char *)((long)v10 + 0x35))
              v9 = &v8[*(char *)((unsigned long)(unsigned char)*v8 + 0x15360) != '\0'];
          }
        }
      }
      else {
label_8670:
        v9 = v5;
      }
      *v5 = v3;
      v5 = v9;
    }
    while( true ) {
      while( true ) {
        v14 = (long)(int)sub_e260(v7,(long)v4 - (long)v7,0);
        if (v7 < v4) {
          do {
            v11 = &v7[1];
            v14 += (unsigned long)(*v7 == '\t');
            v7 = v11;
          } while (v4 != v11);
        }
        v12 = (long)(int)sub_e260(v4,(long)v5 - (long)v4,0);
        if (v4 < v5) {
          do {
            v7 = &v4[1];
            v12 += (unsigned long)(*v4 == '\t');
            v4 = v7;
          } while (v5 != v7);
        }
        while (v14) {
          v14 -= 1;
          v2 = *(char **)&stdout->field_0x28;
          if (*(char **)&stdout->field_0x30 <= v2) // branch-flip
            __overflow(stdout,0x20);
          else {
            *(char **)&stdout->field_0x28 = &v2[1];
            *v2 = 0x20;
          }
        }
        if (v12) { // branch-flip
          do {
            v2 = *(char **)&stdout->field_0x28;
            if (*(char **)&stdout->field_0x30 <= v2) // branch-flip
              __overflow(stdout,0x5f);
            else {
              *(char **)&stdout->field_0x28 = &v2[1];
              *v2 = 0x5f;
            }
            v12 -= 1;
          } while (v12);
          v2 = *(char **)&stdout->field_0x28;
          if (*(char **)&stdout->field_0x30 <= v2) // branch-flip
            __overflow(stdout,10);
          else {
            *(char **)&stdout->field_0x28 = &v2[1];
            *v2 = 10;
          }
        }
        else {
          __printf_chk(1,dcgettext(NULL,"^ no match for key\n",5));
        }
        if (!v10)
          return;
        v10 = (long *)v10[8];
        if (v10) break;
        if (dat_1d419)
          return;
        if (dat_1d41a)
          return;
        v4 = a0->field_0x0;
        v5 = &v4[a0->field_0x8 + -1];
        v7 = v4;
      }
      v7 = a0->field_0x0;
      v13 = a0->field_0x8;
      v5 = &v7[v13 - 1];
      v4 = v7;
      if (v10) break;
label_8642:
      v7 = v4;
    }
  } while( true );
}


// Function: sub_8770 @ 0x8770
int sub_8770(int *a0,long a1)
{
  unsigned long v1;
  int v2; // eax
  int v3; // eax
  int *v4; // rax
  int v5;
  char v6 [128];
  double v7; // stack - 0xe0
  
  if (0 <= (int)sub_e990(a0,0x80000)) { // branch-flip
    if (dat_1d024 + 1U < dat_1d35c) {
      sub_78c0(0xffffffff);
      do {
        if ((int)dat_1d35c <= 0) break;
      } while (sub_78c0(0));
    }
    v4 = __errno_location();
    v7 = dat_170e8;
    do {
      a1 -= 1;
      v2 = pthread_sigmask(0,(void *)0x1d380,v6);
      v1 = dat_1d368;
      dat_1d368 = 0;
      v3 = fork();
      v5 = *v4;
      if (v3) { // branch-flip
        dat_1d368 = v1;
        if (!v2) goto label_8895;
      }
      else {
        if (v2) {
          close(0); // return-dupe
          close(1);
          return 0;
        }
label_8895:
        pthread_sigmask(2,v6,NULL);
      }
      *v4 = v5;
      if (0 <= v3) {
        if (v3) {
          dat_1d35c += 1;
          return v3;
        }
        close(0);
        close(1);
        return 0;
      }
      if (v5 != 0xb) goto label_8943;
      sub_13180(v7);
      v7 = v7 + v7;
      do {
        if ((int)dat_1d35c <= 0) break;
      } while (sub_78c0(0));
    } while (a1);
    v5 = *v4;
label_8943:
    close(*a0);
    close(a0[1]);
    *v4 = v5;
  }
  else {
    v3 = -1;
  }
  return v3;
}


// Function: sub_8980 @ 0x8980
FILE * sub_8980(unsigned long *a0,unsigned long a1,unsigned long *a2)
{
  unsigned int v1;
  long *v10;
  unsigned long v11; // r12
  int v12; // stack - 0x54
  unsigned int v13; // stack - 0x50
  int v2;
  void *v3;
  long *v4;
  int *v5;
  char *v6;
  int v7; // eax
  FILE *v8; // rax
  int v9; // stack - 0x58
  
  v8 = (long *)sub_12bf0(a1,8);
  v4 = (long *)v8;
  *a2 = v8;
  if (a1) { // branch-flip
    v11 = 0;
    v10 = (long *)v8;
    do {
      v3 = (void *)a0[1];
      if ((v3) && (*(char *)((long)v3 + 0xc))) {
        if (*(char *)((long)v3 + 0xc) == '\x01') {
          v1 = *(unsigned int *)((long)v3 + 8);
          v13 = v1;
          v8 = (long)sub_dd00(dat_1d360,&v9);
          if (v8) {
            *(char *)((long)v8 + 0xc) = 2;
            sub_78c0(v1);
          }
        }
        v7 = open((char *)((long)v3 + 0xd),0);
        v2 = v7;
        if (v7 < 0) {
          v4[v11] = 0;
          v8 = (FILE *)v11;
          return v8;
        }
        v7 = sub_8770(&v9,9);
        if (v7 == -1) {
          v8 = (int *)__errno_location();
          v5 = (int *)v8;
          if (*(int *)v8 == 0x18) {
            close(v2);
            *(int *)v8 = 0x18;
            v4[v11] = 0;
            v8 = (FILE *)v11;
            return v8;
          }
          v8 = (char *)sub_108e0(4,dat_1d408);
          v6 = (char *)v8;
          v8 = (char *)dcgettext(NULL,"couldn\'t create process for %s -d",5);
          error(2,*v5,(char *)v8,v6);
          return v8;
        }
        if (!v7) {
          close(v9);
          if (v2)
            sub_7c90(v2,0);
          if (v12 != 1)
            sub_7c90(v12,1);
          execlp(dat_1d408,dat_1d408,"-d",0);
          v8 = (int *)__errno_location();
          sub_79f0(*(int *)v8,"couldn\'t execute compress program (with -d)"); // no-return
        }
        *(int *)((long)v3 + 8) = v7;
        sub_7a90(v3);
        close(v2);
        close(v12);
        v8 = fdopen(v9,"r");
        if (!v8) {
          v8 = (int *)__errno_location();
          v2 = *(int *)v8;
          close(v9);
          *v10 = 0;
          *(int *)v8 = v2;
          v8 = (FILE *)v11;
          return v8;
        }
        *v10 = (long)v8;
      }
      else {
        v8 = (long)sub_74e0((char *)*a0,"r");
        *v10 = (long)v8;
        if (!v8) {
          v8 = (FILE *)v11;
          return v8;
        }
      }
      v11 += 1;
      a0 = &a0[2];
      v10 = &v10[1];
    } while (a1 != v11);
  }
  else {
    v11 = 0;
  }
  v8 = (FILE *)v11;
  return v8;
}


// Function: sub_8c20 @ 0x8c20
FILE * sub_8c20(unsigned long *a0,char a1)
{
  char *v1;
  int v10; // stack - 0xc8
  int v11;
  unsigned long *v12;
  int v13; // stack - 0xc4
  int v2;
  char *v3;
  unsigned long v4;
  unsigned long *v5;
  int *v6;
  int v7; // eax
  FILE *v8; // rax
  char v9 [128];
  
  v3 = *(char **)(dat_1d430 + dat_1d248 * 8);
  v8 = (unsigned long)strlen(v3);
  v4 = (unsigned long)v8;
  v8 = (unsigned long *)sub_12ab0((long)v8 + 0x20U & 0xfffffffffffffff8);
  v5 = (unsigned long *)v8;
  v1 = (char *)((long)v8 + 0xd);
  memcpy(v1,v3,v4);
  *(unsigned long *)((long)v8 + v4 + 0xd) = s_152f0._0_8_;
  *(unsigned int *)((long)v8 + v4 + 0x15) = s_152f0._8_4_;
  *(unsigned long *)v8 = 0;
  v8 = (long)(dat_1d248 + 1);
  dat_1d248 = (long)v8;
  if (v8 == (FILE *)dat_1d428)
    dat_1d248 = 0;
  v7 = pthread_sigmask(0,(void *)0x1d380,v9);
  v10 = CONCAT31(v10._1_3_,v7 == 0);
  v7 = sub_11d70(v1,0x80000);
  v11 = v7;
  v8 = (int *)__errno_location();
  v6 = (int *)v8;
  if (0 <= v11) { // branch-flip
    *dat_1d040 = v5;
    dat_1d040 = v5;
    if ((char)v10) {
      v2 = *(int *)v8;
      pthread_sigmask(2,v9,NULL);
      *(int *)v8 = v2;
    }
    *(char *)((long)v5 + 0xc) = 0;
    if (dat_1d408) {
      v7 = sub_8770(&v10,4);
      *(int *)&v5[1] = v7;
      if (1 <= v7) { // branch-flip
        close(v11);
        close(v10);
        sub_7a90(v5);
        v11 = v13;
      }
      else if (!v7) {
        close(v13);
        if (v11 != 1)
          sub_7c90(v11,1);
        if (v10)
          sub_7c90(v10,0);
        execlp(dat_1d408,dat_1d408,0);
        sub_79f0(*v6,"couldn\'t execute compress program"); // no-return
      }
    }
    v8 = fdopen(v11,"w");
    *a0 = v8;
    v12 = v5;
    if (!v8) {
      v8 = (char *)dcgettext(NULL,"couldn\'t create temporary file",5);
      sub_7480(v8,v1); // no-return
    }
  }
  else {
    v11 = *(int *)v8;
    if ((char)v10) {
      pthread_sigmask(2,v9,NULL);
      *(int *)v8 = v11;
    }
    if ((v11 != 0x18) || (a1 != '\x01')) {
      v8 = (char *)sub_108e0(4,v3);
      v1 = (char *)v8;
      v8 = (char *)dcgettext(NULL,"cannot create temporary file in %s",5);
      error(2,*v6,(char *)v8,v1);
      return v8;
    }
    v12 = NULL;
    free(v5);
  }
  v8 = (FILE *)v12;
  return v8;
}


// Function: sub_8ed0 @ 0x8ed0
void sub_8ed0(void)
{
  unsigned long *v1;
  int v2; // eax
  char v3 [128];
  
  if (dat_1d368) {
    v2 = pthread_sigmask(0,(void *)0x1d380,v3);
    for (v1 = dat_1d368; v1; v1 = (unsigned long *)*v1) {
      unlink((char *)((long)v1 + 0xd));
    }
    dat_1d368 = NULL;
    if (!v2)
      pthread_sigmask(2,v3,NULL);
  }
  sub_c4c0(); // tail-call
}


// Function: sub_8f90 @ 0x8f90
void sub_8f90(char *a0)
{
  int v1;
  char v10 [8]; // stack - 0xc8
  unsigned int v2;
  long *v3;
  long *v4;
  int v5; // eax
  int *v6; // rax
  char *v7; // rax
  long v8;
  unsigned int v9 [32]; // stack - 0xc0
  
  v3 = (long *)0x1d368;
  for (v4 = dat_1d368; a0 != (char *)((long)v4 + 0xd); v4 = (long *)*v4) {
    v3 = v4;
  }
  if (*(char *)((long)v4 + 0xc) == '\x01') {
    v2 = *(unsigned int *)&v4[1];
    v9[0] = v2;
    v8 = sub_dd00(dat_1d360,v10);
    if (v8) {
      *(char *)(v8 + 0xc) = 2;
      sub_78c0(v2);
    }
  }
  v8 = *v4;
  v10[0] = pthread_sigmask(0,(void *)0x1d380,v9) == 0;
  v5 = unlink(a0);
  v6 = __errno_location();
  v1 = *v6;
  *v3 = v8;
  if (v10[0])
    pthread_sigmask(2,v9,NULL);
  if (v5) {
    v7 = (char *)sub_10c60(0,3,a0);
    error(0,v1,dcgettext(NULL,"warning: cannot remove: %s",5),v7);
  }
  if (!v8)
    dat_1d040 = v3;
  free(v4);
}


// Function: sub_9120 @ 0x9120
int sub_9120(unsigned long *a0,long *a1)
{
  unsigned char *v1;
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned char v14 [4008];
  long double v15; // stack - 0x2078
  long v16;
  unsigned char v17 [3999];
  unsigned char *v18; // stack - 0x2098
  unsigned char *v19; // stack - 0x2090
  char v2;
  unsigned long v20; // stack - 0x2048
  unsigned long v21; // stack - 0x1fec
  char v22 [16];
  char v23 [16];
  long double v24; // stack - 0x2088
  long *v25;
  unsigned char *v26;
  unsigned char *v27;
  unsigned char *v28;
  unsigned long v29; // r9
  unsigned char v3;
  unsigned long v30; // r10
  unsigned long v31;
  unsigned char *v32;
  long double v33;
  long double v34;
  long double v35;
  long double v36;
  long double v37;
  long double v38;
  long double v39;
  long v4;
  long double v40;
  long double v41;
  long double v42;
  long double v43;
  long double v44;
  unsigned char v45; // stack - 0x2132
  unsigned char v46; // stack - 0x2131
  unsigned long v47; // stack - 0x2118
  unsigned char *v48; // stack - 0x2110
  unsigned char *v49; // stack - 0x2100
  long v5;
  unsigned char *v50; // stack - 0x20f8
  unsigned long v51; // stack - 0x20f0
  unsigned long v52; // stack - 0x2040
  unsigned long v53; // stack - 0x2038
  unsigned long v54; // stack - 0x2030
  unsigned long v55; // stack - 0x2028
  unsigned long v56; // stack - 0x2020
  unsigned long v57; // stack - 0x2018
  unsigned long v58; // stack - 0x2010
  unsigned long v59; // stack - 0x2008
  unsigned char v6;
  unsigned long v60; // stack - 0x2000
  unsigned long v61; // stack - 0x1ff8
  unsigned int v62; // stack - 0x1ff0
  unsigned long v63; // stack - 0x1fe4
  unsigned long v64; // stack - 0x1fdc
  unsigned long v65; // stack - 0x1fd4
  unsigned long v66; // stack - 0x1fcc
  unsigned long v67; // stack - 0x1fc4
  unsigned long v68; // stack - 0x1fbc
  unsigned long v69; // stack - 0x1fb4
  int v7;
  unsigned long v70; // stack - 0x1fac
  unsigned long v71; // stack - 0x1fa4
  unsigned long v72; // stack - 0x1f9c
  unsigned int v73; // stack - 0x1f94
  int v8;
  unsigned char *v9;
  
  v26 = (unsigned char *)a0[2];
  v10 = (unsigned char *)a1[2];
  v9 = (unsigned char *)a0[3];
  v27 = (unsigned char *)a1[3];
  v25 = dat_1d410;
label_91a0:
  v4 = v25[5];
  v5 = v25[4];
  if (v9 <= v26)
    v9 = v26;
  if (v27 <= v10)
    v27 = v10;
  v29 = (long)v9 - (long)v26;
  v30 = (long)v27 - (long)v10;
  if (((dat_1d861) || (v25[6] & 0xffffffffff0000U)) || ((char)v25[7])) {
    if (v4 || v5) { // branch-flip
      v12 = v29 + 2 + v30;
      v9 = &v16;
      v48 = NULL;
      if (0xfa1 <= v12) { // branch-flip
        v9 = (unsigned char *)sub_12ab0(v12);
        v48 = v9;
        v27 = &v9[v29 + 1];
        if (!v29) goto label_9887;
label_95c1:
        v32 = &v26[v29];
        v29 = 0;
        do {
          v6 = *v26;
          if ((!v5) || (!*(char *)(v5 + (unsigned long)v6))) {
            if (v4)
              v6 = *(unsigned char *)(v4 + (unsigned long)v6);
            v9[v29] = v6;
            v29 += 1;
          }
          v26 = &v26[1];
        } while (v32 != v26);
        v9[v29] = 0;
        v26 = v27;
        if (v30) goto label_960d;
label_9896:
        v10 = v27;
      }
      else {
        v27 = &v17[v29];
        if (v29) goto label_95c1;
label_9887:
        *v9 = 0;
        v26 = v27;
        if (!v30) goto label_9896;
label_960d:
        v27 = &v10[v30];
        v30 = 0;
        do {
          v6 = *v10;
          if ((!v5) || (!*(char *)(v5 + (unsigned long)v6))) {
            if (v4)
              v6 = *(unsigned char *)(v4 + (unsigned long)v6);
            v26[v30] = v6;
            v30 += 1;
          }
          v10 = &v10[1];
        } while (v27 != v10);
        v27 = &v26[v30];
        v10 = v26;
      }
      *v27 = 0;
    }
    else {
      v45 = *v9;
      *v9 = 0;
      v46 = *v27;
      *v27 = 0;
      v9 = v26;
    }
    if (*(char *)((long)v25 + 0x32)) { // branch-flip
      v2 = *(char *)((unsigned long)*v9 + 0x1d740);
      v27 = v9;
      while (v2) {
        v26 = &v27[1];
        v27 = &v27[1];
        v2 = *(char *)((unsigned long)*v26 + 0x1d740);
      }
      v2 = *(char *)((unsigned long)*v10 + 0x1d740);
      v26 = v10;
      while (v2) {
        v32 = &v26[1];
        v26 = &v26[1];
        v2 = *(char *)((unsigned long)*v32 + 0x1d740);
      }
      v7 = sub_11d90(v27,v26,(int)dat_1d868,dat_1d864);
      v34 = v33;
      v35 = v36;
label_92a0:
      v33 = v34;
      v36 = v35;
      if (!v4 && !v5) {
label_92ac:
        v9[v29] = v45;
        v10[v30] = v46;
        goto label_92c1;
      }
    }
    else {
      if (*(char *)((long)v25 + 0x34)) {
        v34 = v37;
        v35 = v38;
        v37 = v39;
        v38 = v40;
        v39 = v42;
        v40 = v44;
        strtold(v9,&v18);
        strtold(v10,&v19);
        v42 = v40;
        v44 = v40;
        if (v9 != v18) { // branch-flip
          if (v10 != v19) { // branch-flip
            if (v33 < v36) {
label_a045:
              v7 = -1;
            }
            else {
              v7 = 1;
              if ((v36 >= v33) && (v7 = 0, v33 != v36)) {
                if (!NAN(v36)) goto label_a045;
                if (!NAN(v33)) goto label_9748;
                v43 = v40;
                v41 = v40;
                strtold(v9,0);
                v24 = v34;
                v34 = v37;
                v37 = v39;
                v39 = v43;
                v42 = v41;
                strtold(v10,0);
                v15 = v35;
                v44 = v42;
                v7 = memcmp(&v24,&v15,0x10);
                v35 = v38;
                v38 = v40;
                v40 = v41;
              }
            }
          }
          else {
label_9748:
            v7 = 1;
          }
        }
        else {
          v7 = -(unsigned int)(v19 != v10);
        }
        goto label_92a0;
      }
      if (!*(char *)((long)v25 + 0x35)) {
        if (*(char *)((long)v25 + 0x36)) { // branch-flip
          v7 = sub_7030(v9,NULL);
          v7 -= sub_7030(v10,NULL);
          v34 = v33;
          v35 = v36;
        }
        else if (*(char *)((long)v25 + 0x33)) { // branch-flip
          v21 = dat_1d300;
          v63 = dat_1d308;
          v64 = dat_1d310;
          v65 = dat_1d318;
          v72 = dat_1d350;
          v73 = dat_1d358;
          v61 = dat_1d350;
          v62 = dat_1d358;
          v66 = dat_1d320;
          v67 = dat_1d328;
          v68 = dat_1d330;
          v69 = dat_1d338;
          v70 = dat_1d340;
          v71 = dat_1d348;
          v20 = dat_1d300;
          v52 = dat_1d308;
          v53 = dat_1d310;
          v54 = dat_1d318;
          v55 = dat_1d320;
          v56 = dat_1d328;
          v57 = dat_1d330;
          v58 = dat_1d338;
          v59 = dat_1d340;
          v60 = dat_1d348;
          if (dat_1d861) {
            v27 = &v9[v29];
            v49 = NULL;
            v1 = &v10[v30];
            v12 = (v29 + v30) * 3 + 2;
            v47 = 4000;
            v7 = 0;
            v26 = v10;
            v11 = v14;
            v32 = v9;
label_9a20:
            v28 = v11;
            if (v47 < v12) goto label_9b04;
label_9a30:
            v11 = v28;
            if (v32 < v27) goto label_9b4f;
label_9a3b:
            v11 = v28;
            if (v26 < v1) {
              v13 = 0;
              v31 = v47;
              goto label_9b7e;
            }
            v51 = 0;
            v13 = 0;
label_9a51:
            v50 = &v11[v13];
            do {
              if (v27 <= v32) { // branch-flip
                if (v1 <= v26) {
label_9c1a:
                  MD5_Update(&v20,v11,v13);
                  MD5_Final(v22,&v20);
                  MD5_Update(&v21,v50,v51);
                  MD5_Final(v23,&v21);
                  v8 = memcmp(v22,v23,0x10);
                  if (!v8) {
                    if (v7) goto label_9cac;
                    goto label_9d87;
                  }
                  v7 = v8;
                  goto label_9cac;
                }
label_9a70:
                v26 = &v26[strlen((char *)v26) + 1];
              }
              else {
                v32 = &v32[strlen((char *)v32) + 1];
                if (v26 < v1) goto label_9a70;
              }
              while( true ) {
                if ((v27 <= v32) && (v1 <= v26)) goto label_9c1a;
                MD5_Update(&v20,v11,v13);
                MD5_Update(&v21,v50,v51);
                if (v7) goto label_9a20;
                v31 = v51;
                if (v13 <= v51)
                  v31 = v13;
                v7 = memcmp(v11,v50,v31);
                if (v7) goto label_9a20;
                v7 = (unsigned int)(v13 >= v51 && v13 != v51) - (unsigned int)(v13 < v51);
                v28 = v11;
                if (v12 <= v47) goto label_9a30;
label_9b04:
                v47 = v47 * 3 >> 1;
                if (v47 <= v12)
                  v47 = v12;
                free(v49);
                v11 = malloc(v47);
                v49 = v11;
                if (!v11) {
                  v47 = 4000;
                  v28 = v14;
                  goto label_9a30;
                }
                v28 = v49;
                if (v27 <= v32) goto label_9a3b;
label_9b4f:
                v13 = sub_7800(v11,v32,v47) + 1;
                if (v1 <= v26) { // branch-flip
                  v51 = 0;
                  v31 = v13;
                }
                else {
                  if (v13 <= v47) { // branch-flip
                    v28 = &v11[v13];
                    v31 = v47 - v13;
                  }
                  else {
                    v28 = NULL;
                    v31 = 0;
                  }
label_9b7e:
                  v51 = sub_7800(v28,v26,v31) + 1;
                  v31 = v51 + v13;
                }
                if ((v13 <= v47) && (v31 <= v47)) goto label_9a51;
                if (v31 <= 0x5555555555555554)
                  v31 = v31 * 3 >> 1;
                free(v49);
                v11 = (unsigned char *)sub_12ab0(v31);
                v47 = v31;
                if (v27 <= v32) break;
                strxfrm((char *)v11,(char *)v32,v13);
                if (v26 < v1) goto label_9e3c;
                v50 = &v11[v13];
                v31 = strlen((char *)v32);
                v49 = v11;
                v32 = &v32[v31 + 1];
              }
              if (v1 <= v26) {
                v50 = &v11[v13];
                v49 = v11;
                goto label_9c1a;
              }
label_9e3c:
              v50 = &v11[v13];
              strxfrm((char *)v50,(char *)v26,v51);
              v49 = v11;
            } while( true );
          }
          MD5_Update(&v20,v9,v29);
          MD5_Final(v22,&v20);
          MD5_Update(&v21,v10,v30);
          MD5_Final(v23,&v21);
          v7 = memcmp(v22,v23,0x10);
          if (v7) // branch-flip
            v49 = NULL;
          else {
            v49 = NULL;
            v13 = v29;
            v11 = v9;
            v50 = v10;
            v51 = v30;
label_9d87:
            v12 = v51;
            if (v13 <= v51)
              v12 = v13;
            v7 = memcmp(v11,v50,v12);
            if (!v7)
              v7 = (unsigned int)(v51 < v13) - (unsigned int)(v13 < v51);
          }
label_9cac:
          free(v49);
          v34 = v33;
          v35 = v36;
        }
        else if ((char)v25[7]) { // branch-flip
          v7 = sub_ca00(v9,v29,v10,v30);
          v34 = v33;
          v35 = v36;
        }
        else {
          v34 = v33;
          v35 = v36;
          if (v29) { // branch-flip
            v7 = 1;
            if (v30) {
              v7 = sub_13120(v9,v29 + 1,v10,v30 + 1);
              v34 = v33;
              v35 = v36;
            }
          }
          else {
            v7 = -(unsigned int)(v30 != 0);
          }
        }
        goto label_92a0;
      }
      v6 = *v9;
      v2 = *(char *)((unsigned long)v6 + 0x1d740);
      v27 = v9;
      while (v2) {
        v6 = v27[1];
        v27 = &v27[1];
        v2 = *(char *)((unsigned long)v6 + 0x1d740);
      }
      v3 = *v10;
      v2 = *(char *)((unsigned long)v3 + 0x1d740);
      v26 = v10;
      while (v2) {
        v3 = v26[1];
        v26 = &v26[1];
        v2 = *(char *)((unsigned long)v3 + 0x1d740);
      }
      v15._0_8_ = &v27[v6 == 0x2d];
      v8 = 0;
      if ('1' <= (char)sub_6f70(&v15)) {
        v8 = (int)*(char *)((unsigned long)*v15._0_8_ + 0x15360);
        if (v6 == 0x2d)
          v8 = -(int)*(char *)((unsigned long)*v15._0_8_ + 0x15360);
      }
      v15 = (long double)CONCAT28(v15._8_2_,&v26[v3 == 0x2d]);
      v7 = v8;
      if ('1' <= (char)sub_6f70(&v15)) {
        v7 = v8 - *(char *)((unsigned long)*v15._0_8_ + 0x15360);
        if (v3 == 0x2d)
          v7 = *(char *)((unsigned long)*v15._0_8_ + 0x15360) + v8;
      }
      v34 = v33;
      v35 = v36;
      if (v7) goto label_92a0;
      v7 = sub_11d90(v27,v26,(int)dat_1d868,dat_1d864);
      if (!v4 && !v5) goto label_92ac;
    }
    free(v48);
    goto label_92c1;
  }
  if (v5) {
    if (!v4) {
      do {
        if ((v9 <= v26) || (!*(char *)(v5 + (unsigned long)*v26))) {
          if (v27 <= v10) goto label_a018;
          while (*(char *)(v5 + (unsigned long)*v10)) {
            v10 = &v10[1];
            if (v27 == v10) goto label_a018;
          }
          if ((v9 <= v26) || (v27 <= v10)) goto label_a018;
          v7 = (unsigned int)*v26 - (unsigned int)*v10;
          if (v7) goto label_94dd;
          v10 = &v10[1];
        }
        v26 = &v26[1];
      } while( true );
    }
    do {
      if ((v9 <= v26) || (!*(char *)(v5 + (unsigned long)*v26))) {
        if (v27 <= v10) goto label_9f28;
        while (*(char *)(v5 + (unsigned long)*v10)) {
          v10 = &v10[1];
          if (v27 == v10) goto label_9f28;
        }
        if ((v9 <= v26) || (v27 <= v10)) goto label_9f28;
        v7 = (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v26) - (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v10);
        if (v7) goto label_94dd;
        v10 = &v10[1];
      }
      v26 = &v26[1];
    } while( true );
  }
  if (!v29) {
    v7 = -(unsigned int)(v30 != 0);
    goto label_92c1;
  }
  if (!v30) {
label_9f18:
    v7 = 1;
    goto label_94dd;
  }
  if (v4) { // branch-flip
    while ((v26 < v9 && (v10 < v27))) {
      v6 = *v26;
      v32 = &v10[1];
      v26 = &v26[1];
      v7 = (unsigned int)*(unsigned char *)(v4 + (unsigned long)v6) - (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v10);
      v10 = v32;
      if (v7) goto label_94dd;
    }
  }
  else {
    v12 = v30;
    if (v29 <= v30)
      v12 = v29;
    v7 = memcmp(v26,v10,v12);
    if (v7) goto label_94dd;
  }
  if (v29 < v30) {
    v7 = -1;
    goto label_94dd;
  }
  if (v29 != v30) goto label_9f18;
  goto label_92c7;
label_a018:
  v7 = (unsigned int)(v26 < v9) - (unsigned int)(v10 < v27);
  goto label_92c1;
label_9f28:
  v7 = (unsigned int)(v26 < v9) - (unsigned int)(v10 < v27);
label_92c1:
  if (v7) {
label_94dd:
    if (*(char *)((long)v25 + 0x37))
      return -v7;
    return v7;
  }
label_92c7:
  v25 = (long *)v25[8];
  if (!v25)
    return 0;
  v26 = (unsigned char *)*a0;
  v4 = a0[1];
  if (v25[2] != -1) { // branch-flip
    v9 = (unsigned char *)sub_7ca0(v26,v4,v25);
    v27 = (unsigned char *)sub_7ca0((char *)*a1,a1[1],v25);
  }
  else {
    v9 = &v26[v4 + -1];
    v27 = (unsigned char *)(*a1 + -1 + a1[1]);
  }
  if (*v25 != -1) { // branch-flip
    v26 = (unsigned char *)sub_7dd0(v26,v4,v25);
    v10 = (unsigned char *)sub_7dd0((char *)*a1,a1[1],v25);
  }
  else {
    v10 = (unsigned char *)*a1;
    if ((char)v25[6]) {
      if (v26 < v9) {
        do {
          if (!*(char *)((unsigned long)*v26 + 0x1d740)) break;
          v26 = &v26[1];
        } while (v9 != v26);
      }
      if (v10 < v27) {
        do {
          if (!*(char *)((unsigned long)*v10 + 0x1d740)) goto label_91a0;
          v10 = &v10[1];
        } while (v27 != v10);
      }
    }
  }
  goto label_91a0;
}


// Function: sub_a180 @ 0xa180
void sub_a180(unsigned long *a0,unsigned long a1,unsigned long *a2,bool a3) // return-dupe x2
{
  unsigned long v1;
  long v10; // rax
  unsigned long *v11;
  unsigned long *v12; // rbp
  unsigned long *v13;
  unsigned long v14; // r12
  unsigned long *v15;
  unsigned long *v16;
  long v17; // stack - 0x48
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  int v9; // eax
  
  if (a1 != 2) {
    v14 = a1 >> 1;
    v17 = a1 - v14;
    v13 = &a0[v14 * -4];
    if (a3) { // branch-flip
      v16 = &a2[v14 * -4];
      sub_a180(v13,v17,v16,1);
      v11 = a2;
      if (4 <= a1)
        sub_a180(a0,v14,a2,0);
    }
    else {
      sub_a180(v13,v17,a2,0);
      v11 = a0; // branch-flip
      v16 = v13;
      if (4 <= a1) {
        sub_a180(a0,v14,a2,1);
        a0 = a2;
      }
      else {
        v1 = a0[-3];
        v2 = a0[-2];
        v3 = a0[-1];
        a2[-4] = a0[-4];
        a2[-3] = v1;
        a2[-2] = v2;
        a2[-1] = v3;
        a0 = a2;
      }
    }
    v13 = &v16[-4];
    v15 = &a0[-4];
    while( true ) {
      while( true ) {
        v12 = v13;
        v13 = &v11[-4];
        if (1 <= (int)sub_a0b0(v15,v12)) break;
        v1 = a0[-3];
        v2 = a0[-2];
        v3 = a0[-1];
        *v13 = a0[-4];
        v11[-3] = v1;
        v11[-2] = v2;
        v11[-1] = v3;
        v14 -= 1;
        if (!v14)
          return;
        v11 = v13;
        v13 = v12;
        a0 = v15;
        v15 = &v15[-4];
      }
      v1 = v16[-3];
      v17 -= 1;
      *v13 = v16[-4];
      v11[-3] = v1;
      v1 = v16[-1];
      v11[-2] = v16[-2];
      v11[-1] = v1;
      if (!v17) break;
      v11 = v13;
      v13 = &v12[-4];
      v16 = v12;
    }
    v10 = -0x20;
    do {
      v1 = ((unsigned long *)((long)a0 + v10))[1];
      v16 = (unsigned long *)((long)a0 + v10 + 0x10);
      v2 = *v16;
      v3 = v16[1];
      *(unsigned long *)((long)v13 + v10) = *(unsigned long *)((long)a0 + v10);
      ((unsigned long *)((long)v13 + v10))[1] = v1;
      v16 = (unsigned long *)((long)v11 + v10 + -0x10);
      *v16 = v2;
      v16[1] = v3;
      v10 -= 0x20;
      v14 -= 1;
    } while (v14);
    return;
  }
  v9 = sub_a0b0((struct_11 *)&a0[-4],(struct_11 *)&a0[-8]);
  if (a3) {
    v13 = &a0[(long)(int)((unsigned int)(0 < v9) - 2) * 4];
    a0 = &a0[(long)(int)~(unsigned int)(0 < v9) * 4];
    v1 = a0[1];
    v2 = a0[2];
    v3 = a0[3];
    a2[-4] = *a0;
    a2[-3] = v1;
    v1 = *v13;
    v4 = v13[1];
    a2[-2] = v2;
    a2[-1] = v3;
    v2 = v13[2];
    v3 = v13[3];
    a2[-8] = v1;
    a2[-7] = v4;
    a2[-6] = v2;
    a2[-5] = v3;
    return;
  }
  if (v9 <= 0)
    return;
  v1 = a0[-4];
  v2 = a0[-3];
  v3 = a0[-2];
  v4 = a0[-1];
  v5 = a0[-8];
  v6 = a0[-7];
  v7 = a0[-6];
  v8 = a0[-5];
  a2[-4] = v1;
  a2[-3] = v2;
  a2[-2] = v3;
  a2[-1] = v4;
  a0[-4] = v5;
  a0[-3] = v6;
  a0[-2] = v7;
  a0[-1] = v8;
  a0[-8] = v1;
  a0[-7] = v2;
  a0[-6] = v3;
  a0[-5] = v4;
}


// Function: sub_a3c0 @ 0xa3c0
void sub_a3c0(long *a0,FILE *a1,char *a2)
{
  if (dat_1d419) {
    if (dat_1d840) {
      if (!sub_a0b0(a0,(struct_11 *)0x1d840))
        return;
    }
    dat_1d840 = *a0;
    dat_1d848 = a0[1];
    dat_1d850 = a0[2];
    dat_1d858 = a0[3];
  }
  sub_82d0(a0,a1,a2); // tail-call
}


// Function: sub_a430 @ 0xa430
void sub_a430(long a0,unsigned long a1,unsigned long a2,FILE *a3,char *a4,void *a5)
{
  unsigned long v1;
  long v10;
  unsigned long v11;
  unsigned long v12;
  unsigned long v13;
  unsigned long v14;
  unsigned long v15;
  unsigned long v16;
  unsigned long v17;
  unsigned long v18;
  unsigned long v19;
  struct_8 *v2;
  long v20;
  int v21; // eax
  void *v22; // rax
  unsigned long *v23; // rax
  unsigned char *v24;
  unsigned long *v25;
  void *v26;
  long v27;
  unsigned long *v28;
  unsigned long *v29; // rdx
  unsigned long *v3;
  unsigned long *v30;
  void *v31; // stack - 0x68
  unsigned long v32;
  unsigned long v33; // r14
  unsigned long v34;
  unsigned long v35;
  bool v36; // zf
  void *v37; // stack - 0xd0
  unsigned long v38; // stack - 0xc8
  void *v39; // stack - 0xc0
  FILE *v4;
  void *v40; // stack - 0xb8
  void **v41; // stack - 0xb0
  unsigned long v42; // stack - 0x88
  unsigned long v43; // stack - 0x80
  unsigned long v44; // stack - 0x60
  long v45; // stack - 0x58
  long v46; // stack - 0x50
  char *v5;
  struct_11 *v6;
  struct_11 *v7;
  long *v8;
  long v9;
  
  v40 = (void *)sub_12bf0(a2,0x38);
  v22 = (void *)sub_12bf0(a2,8);
  v39 = (void *)sub_12bf0(a2,8);
  v23 = (unsigned long *)sub_12bf0(a2,8);
  v20 = dat_1d410;
  v31 = NULL;
  v41 = (void **)v23;
  v37 = v22;
  if (a2) {
    v34 = 0;
    v38 = a2;
    v42 = a1;
    do {
      while( true ) {
        v32 = dat_1d438 / v38;
        if (dat_1d438 / v38 < dat_1d030)
          v32 = dat_1d030;
        v2 = (struct_8 *)((long)v40 + v34 * 0x38);
        while( true ) {
          v1 = (v32 & 0xffffffffffffffe0) + 0x20;
          v24 = malloc(v1);
          v2->field_0x0 = v24;
          if (v24) break;
          v32 = v1 >> 1;
          if (v1 <= 0x42)
            sub_12fb0(); // no-return
        }
        v2->field_0x18 = v1;
        v27 = v34 * 8;
        v2->field_0x28 = 0x20;
        v30 = (unsigned long *)(v34 * 0x10 + a0);
        v4 = *(FILE **)(unsigned long *)((long)a5 + v27);
        v2->field_0x10 = 0;
        v2->field_0x20 = 0;
        v5 = (char *)*v30;
        v2->field_0x8 = 0;
        v2->field_0x30 = 0;
        if (sub_7ef0(v2,v4,v5)) break;
        sub_7660(*(FILE **)(unsigned long *)((long)a5 + v27),(char *)*v30);
        if (v34 < v42) {
          v42 -= 1;
          sub_8f90((char *)*v30);
        }
        free(v2->field_0x0);
        v32 = v38 - 1;
        if (v32 <= v34) {
          if (!v32) goto label_aa9e;
          v38 = v32;
          goto label_a5e2;
        }
        do {
          v30 = (unsigned long *)(a0 + 0x10 + v27 * 2);
          v12 = v30[1];
          v11 = *(unsigned long *)((long)a5 + v27 + 8);
          v3 = (unsigned long *)(a0 + v27 * 2);
          *v3 = *v30;
          v3[1] = v12;
          *(unsigned long *)((long)a5 + v27) = v11;
          v27 += 8;
        } while (v38 * 8 + -8 != v27);
        v38 = v32;
      }
      v27 = v2->field_0x18;
      v24 = v2->field_0x0;
      *(unsigned char **)((long)v22 + v34 * 8) = &v24[v27 + -0x20];
      *(unsigned char **)((long)v39 + v34 * 8) = &v24[v2->field_0x10 * -0x20 + v27];
      v34 += 1;
    } while (v34 < v38);
label_a5e2:
    v34 = 0;
    do {
      v23[v34] = v34;
      v34 += 1;
    } while (v34 != v38);
    if (v38 != 1) {
      v34 = 1;
      do {
        while( true ) {
          v25 = &v23[v34];
          v28 = &v23[v34 - 1];
          v6 = *(struct_11 **)((long)v22 + *v25 * 8);
          v7 = *(struct_11 **)((long)v22 + *v28 * 8);
          if ((int)sub_a0b0(v7,v6) <= 0) break;
          v32 = *v28;
          v34 = 1;
          *v28 = *v25;
          *v25 = v32;
        }
        v34 += 1;
      } while (v34 < v38);
    }
    v43 = 0;
    v41 = NULL;
label_a680:
    v8 = *(long **)((long)v22 + *v23 * 8);
    if (dat_1d419) { // branch-flip
      if (v41) {
        if (!sub_a0b0(v41,v8)) goto label_a6b6;
        sub_82d0(&v31,a3,a4);
      }
      v34 = v8[1];
      v26 = v31;
      if (v43 < v34) {
        do {
          v32 = v34;
          if (!v43) goto label_ab1d;
          v43 *= 2;
        } while (v43 < v34);
        v32 = v43;
label_ab1d:
        free(v31);
        v26 = (void *)sub_12ab0(v32);
        v34 = v8[1];
        v31 = v26;
        v43 = v32;
      }
      v44 = v34;
      v26 = memcpy(v26,(void *)*v8,v34);
      v41 = &v31;
      if (v20) {
        v45 = (v8[2] - *v8) + (long)v26;
        v46 = (long)v26 + (v8[3] - *v8);
      }
    }
    else {
      sub_82d0(v8,a3,a4);
    }
label_a6b6:
    v34 = *v23;
    if (*(long **)((long)v39 + v34 * 8) < v8) {
      *(long **)((long)v22 + v34 * 8) = &v8[-4];
      v37 = (void *)v34;
      if (v38 != 1) goto label_a761;
label_a81c:
      *v23 = v34;
      goto label_a680;
    }
    v4 = *(FILE **)((long)a5 + v34 * 8);
    v5 = *(char **)(a0 + v34 * 0x10);
    if (sub_7ef0((struct_8 *)((long)v40 + v34 * 0x38),v4,v5)) {
      v34 = *v23;
      v8 = (long *)((long)v40 + v34 * 0x38);
      v27 = v8[3];
      v9 = *v8;
      v10 = v8[2];
      *(long *)((long)v22 + v34 * 8) = v27 + v9 + -0x20;
      *(long *)((long)v39 + v34 * 8) = v27 + v9 + v10 * -0x20;
      v37 = (void *)v34;
      if (v38 == 1) goto label_a81c;
label_a761:
      v32 = 1;
      v1 = 1;
      v35 = v38;
      do {
        while( true ) {
          v33 = v1;
          v21 = sub_a0b0(*(struct_11 **)((long)v22 + v34 * 8),*(struct_11 **)((long)v22 + v23[v33] * 8));
          if ((0 <= v21) && ((v21 || (v23[v33] <= v37)))) break;
          v1 = v32 + v33 >> 1;
          v35 = v33;
          if (v33 <= v32) goto label_a7c9;
        }
        v32 = v33 + 1;
        v1 = v32 + v35 >> 1;
      } while (v32 < v35);
label_a7c9:
      if (v32 - 1) {
        v28 = v23;
        do {
          v25 = &v28[1];
          *v28 = v28[1];
          v28 = v25;
        } while (v25 != &v23[v32 - 1]);
      }
      v23[v32 - 1] = (unsigned long)v37;
      goto label_a680;
    }
    v34 = *v23;
    v28 = &v23[1];
    if (v38 != 1) {
      do {
        if (v34 < *v28) {
          *v28 = *v28 - 1;
          v34 = *v23;
        }
        v28 = &v28[1];
      } while (&v23[v38] != v28);
    }
    v32 = v38 - 1;
    sub_7660(*(FILE **)((long)a5 + v34 * 8),*(char **)(a0 + v34 * 0x10));
    v34 = *v23;
    if (v34 < v42) {
      v42 -= 1;
      sub_8f90(*(char **)(a0 + v34 * 0x10));
      v34 = *v23;
    }
    free(*(void **)((long)v40 + v34 * 0x38));
    v34 = *v23;
    if (v34 < v32) {
      v27 = v34 * 8;
      v30 = (unsigned long *)((long)v40 + v34 * 0x38);
      do {
        v3 = (unsigned long *)(a0 + 0x10 + v27 * 2);
        v12 = *v3;
        v13 = v3[1];
        v29 = &v30[7];
        v14 = *v29;
        v15 = v30[8];
        v16 = v30[9];
        v17 = v30[10];
        *(unsigned long *)((long)a5 + v27) = *(unsigned long *)((long)a5 + v27 + 8);
        v11 = v30[0xd];
        v18 = v30[0xb];
        v19 = v30[0xc];
        v3 = (unsigned long *)(a0 + v27 * 2);
        *v3 = v12;
        v3[1] = v13;
        v30[6] = v11;
        v11 = *(unsigned long *)((long)v22 + v27 + 8);
        *v30 = v14;
        v30[1] = v15;
        *(unsigned long *)((long)v22 + v27) = v11;
        v11 = *(unsigned long *)((long)v39 + v27 + 8);
        v30[2] = v16;
        v30[3] = v17;
        *(unsigned long *)((long)v39 + v27) = v11;
        v27 += 8;
        v30[4] = v18;
        v30[5] = v19;
        v30 = v29;
      } while (v38 * 8 + -8 != v27);
    }
    if (v32) {
      v28 = v23;
      do {
        v25 = &v28[1];
        *v28 = v28[1];
        v28 = v25;
      } while (&v23[v38 - 1] != v25);
      v38 = v32;
      goto label_a680;
    }
    v36 = v41 != NULL;
    v41 = (void **)v23;
    v37 = v22;
    if (v36) {
      if (dat_1d419) {
        sub_82d0(&v31,a3,a4);
        free(v31);
      }
    }
  }
label_aa9e:
  sub_7660(a3,a4);
  free(a5);
  free(v40);
  free(v41);
  free(v39);
  free(v37); // tail-call
  return;
}


// Function: sub_abe0 @ 0xabe0
unsigned long sub_abe0(long a0,unsigned long a1,unsigned long a2,FILE *a3,char *a4)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x48
  
  v2 = sub_8980(a0,a2,&v3);
  if ((v2 < a2) && (v2 < 2)) {
    v1 = *(char **)(a0 + v2 * 0x10);
    sub_7480(dcgettext(NULL,"open failed",5),v1); // no-return
  }
  sub_a430(a0,a1,v2,a3,a4,v3);
  return v2;
}


// Function: sub_ac90 @ 0xac90
void sub_ac90(unsigned long *a0,unsigned long a1,unsigned long a2,char *a3)
{
  int v1; // eax
  long *v10;
  long *v11; // rbx
  unsigned long v12; // stack - 0xe0
  stat v13; // stack - 0xd8
  long v14; // stack - 0xe8
  unsigned long *v15;
  unsigned long v16;
  unsigned long v17; // r12
  char *v18;
  unsigned long *v19;
  int v2;
  unsigned long v20; // stack - 0x110
  unsigned long v21; // stack - 0x108
  long v3; // rax
  unsigned long v4;
  unsigned long v5;
  FILE *v6; // rax
  unsigned long v7; // rdx
  unsigned long v8;
  long v9; // rax
  
  v5 = (unsigned long)dat_1d024;
  v21 = a2;
  if (v5 < a2) {
    do {
      v4 = 0;
      v9 = 0;
      if (v5 <= v21) { // branch-flip
        do {
          v17 = v4;
          v3 = sub_8c20(&v12,0);
          v4 = (unsigned long)dat_1d024;
          v5 = a1;
          if (v4 <= a1)
            v5 = v4;
          v4 = sub_abe0(&a0[v9 * 2],v5,v4,v12,(char *)(v3 + 0xd));
          v8 = (unsigned long)dat_1d024;
          v5 = a1;
          if (v4 <= a1)
            v5 = v4;
          v9 += v4;
          v4 = v17 + 1;
          a0[v17 * 2] = (char *)(v3 + 0xd);
          a0[v17 * 2 + 1] = v3;
          a1 -= v5;
        } while (v8 <= v21 - v9);
        v7 = v4 % v8;
        v5 = v8 - v7;
        v16 = v21 - v9;
        v15 = &a0[v17 * 2 + 2];
        v19 = &a0[v9 * 2];
      }
      else {
        v7 = 0;
        v8 = v5;
        v16 = v21;
        v15 = a0;
        v19 = a0;
      }
      if (v5 < v16) {
        v8 = (v16 + 1 + v7) - v8;
        v3 = sub_8c20(&v12,0);
        v5 = a1;
        if (v8 <= a1)
          v5 = v8;
        v8 = sub_abe0(v19,v5,v8,v12,(char *)(v3 + 0xd));
        v5 = a1;
        if (v8 <= a1)
          v5 = v8;
        v4 += 1;
        v9 += v8;
        *v15 = (char *)(v3 + 0xd);
        v15[1] = v3;
        a1 -= v5;
        v19 = &a0[v9 * 2];
        v15 = &a0[v4 * 2];
      }
      a1 += v4;
      memmove(v15,v19,(v21 - v9) * 0x10);
      v5 = (unsigned long)dat_1d024;
      v21 += v4 - v9;
    } while (v5 < v21);
  }
  if (a1 < v21) {
    v9 = 0;
    v10 = &a0[a1 * 2];
    v5 = a1;
    do {
      while( true ) {
        v18 = (char *)*v10;
        v1 = strcmp(v18,"-");
        if (((!a3) || (strcmp(a3,v18))) || (!v1)) break;
label_ae9e:
        v18 = (char *)(v9 + 0xd);
        if (!v9) {
          v9 = sub_8c20(&v12,0);
          v18 = (char *)(v9 + 0xd);
          sub_abe0(v10,0,1,v12,v18);
        }
        *v10 = (long)v18;
        v5 += 1;
        v11 = &v10[2];
        v10[1] = v9;
        v10 = v11;
        if (v5 == v21) goto label_aecf;
      }
      v2 = dat_1d2f0;
      if (dat_1d2f0) { // branch-flip
label_ae64:
        if (0 <= v2) break;
      }
      else {
        if (fstat(1,(stat *)0x1d260)) {
          v2 = *__errno_location();
          dat_1d2f0 = v2;
          goto label_ae64;
        }
        dat_1d2f0 = -1;
      }
      if (v1) { // branch-flip
        v18 = (char *)*v10;
        if ((!stat(v18,&v13)) && (v13._8_8_ == dat_1d268)) {
label_ae8c:
          if (v13._0_8_ == dat_1d260) goto label_ae9e;
        }
      }
      else if ((!fstat(0,&v13)) && (v13._8_8_ == dat_1d268)) goto label_ae8c;
      v5 += 1;
      v10 = &v10[2];
    } while (v5 != v21);
  }
label_aecf:
  v20 = a1;
  do {
    v5 = sub_8980(a0,v21,&v14);
    if (v21 != v5) { // branch-flip
      if (v5 <= 2) {
        v18 = (char *)a0[v5 * 2];
        sub_7480(dcgettext(NULL,"open failed",5),v18); // no-return
      }
    }
    else {
      v6 = (FILE *)sub_74e0(a3,"w");
      if (v6) {
        sub_a430(a0,v20,v21,v6,a3,v14);
        return;
      }
      if ((*__errno_location() != 0x18) || (v21 <= 2))
        sub_7480(dcgettext(NULL,"open failed",5),a3); // no-return
    }
    v5 -= 1;
    v15 = &a0[v5 * 2];
    while( true ) {
      sub_7660(*(FILE **)(v14 + v5 * 8),(char *)*v15);
      v19 = &v15[-2];
      v9 = sub_8c20(&v12,3 <= v5);
      if (v9) break;
      v5 -= 1;
      v15 = v19;
    }
    v4 = v20;
    if (v5 < v20)
      v4 = v5;
    v3 = v21 - v5;
    sub_a430(a0,v4,v5,v12,(char *)(v9 + 0xd),v14);
    v21 = v3 + 1;
    *a0 = (char *)(v9 + 0xd);
    a0[1] = v9;
    memmove(&a0[2],v15,v3 * 0x10);
    v20 = (v20 - v4) + 1;
  } while( true );
}


// Function: sub_b220 @ 0xb220
void sub_b220(long a0,unsigned long a1,unsigned long a2,struct_14 *a3,unsigned long *a4,FILE *a5,char *a6)
{
  pthread_mutex_t *v1;
  long v10;
  long v11;
  unsigned long *v12;
  long v13;
  unsigned long *v14;
  unsigned long *v15; // rax
  long v16; // stack - 0x78
  unsigned long v17; // stack - 0x80
  unsigned long *v18;
  struct_12 *v19;
  pthread_mutex_t *v2;
  long v20;
  unsigned long *v21;
  char v22;
  unsigned long *v23;
  long v24;
  long v25;
  unsigned long *v26;
  unsigned long v27;
  unsigned long v28;
  unsigned long v29; // r15
  int v3;
  unsigned long *v30;
  unsigned long v31; // stack - 0x70
  unsigned long v32; // stack - 0x68
  long v33; // stack - 0x60
  unsigned long *v34; // stack - 0x58
  FILE *v35; // stack - 0x50
  char *v36; // stack - 0x48
  unsigned long *v4;
  unsigned long v5;
  unsigned long v6;
  char *v7;
  long *v8; // rax
  unsigned long *v9;
  
  v7 = a6;
  v29 = a1 >> 1;
  v28 = a3->field_0x28;
  v27 = a3->field_0x30;
  v33 = a3->field_0x40;
  v36 = a6;
  v16 = a0;
  v31 = v29;
  v32 = a2;
  v34 = a4;
  v35 = a5;
  if ((0x20000 <= v28 + v27) && (2 <= a1)) {
    if (!pthread_create(&v17,0,sub_ba80,&v16)) {
      sub_b220(a0 + a3->field_0x28 * -0x20,a1 - v29,a2,a3->field_0x48,a4,a5,v7);
      pthread_join(v17,0);
      return;
    }
    v28 = a3->field_0x28;
    v27 = a3->field_0x30;
  }
  v23 = (unsigned long *)(a0 + a2 * -0x20);
  v30 = (unsigned long *)(a0 + v28 * -0x20);
  if (2 <= v27)
    sub_a180(v30,v27,&v23[(v28 >> 1) * -4],0);
  if (2 <= v28)
    sub_a180(a0,v28,v23,0);
  a3->field_0x0 = a0;
  v23 = &a4[6];
  a3->field_0x8 = v30;
  a3->field_0x10 = v30;
  a3->field_0x18 = a0 + v28 * -0x20 + v27 * -0x20;
  v1 = (pthread_mutex_t *)&a4[1];
  pthread_mutex_lock(v1);
  sub_df40((long *)*a4,a3);
  a3->field_0x54 = 1;
  pthread_cond_signal(v23);
  pthread_mutex_unlock(v1);
  do {
    pthread_mutex_lock(v1);
    while (v8 = (long *)sub_e010((struct_13 *)*a4), !v8) {
      pthread_cond_wait(v23,v1);
    }
    pthread_mutex_unlock(v1);
    v2 = (pthread_mutex_t *)&v8[0xb];
    pthread_mutex_lock(v2);
    v3 = (int)v8[10];
    *(char *)((long)v8 + 0x54) = 0;
    if (!v3) {
      pthread_mutex_unlock(v2);
      pthread_mutex_lock(v1);
      sub_df40((long *)*a4,v8);
      *(char *)((long)v8 + 0x54) = 1;
      pthread_cond_signal(v23);
      pthread_mutex_unlock(v1);
      return;
    }
    v30 = (unsigned long *)*v8;
    v4 = (unsigned long *)v8[1];
    v18 = (unsigned long *)v8[2];
    v10 = (a2 >> ((char)v3 * '\x02' + 2U & 0x3f)) + 1;
    v21 = v30;
    if (v3 != 1) { // branch-flip
      v12 = (unsigned long *)v8[4];
      v14 = (unsigned long *)*v12;
      v9 = v4;
      if (v30 != v18) { // branch-flip
        v11 = v10;
        v26 = v30;
        while ((unsigned long *)v8[3] != v9) {
          v10 = v11 + -1;
          if (!v11) {
            v21 = (unsigned long *)*v8;
            v22 = *(char *)((long)v8 + 0x54);
            v11 = -1;
            v12 = (unsigned long *)v8[4];
            goto label_b66d;
          }
          v15 = &v14[-4];
          if (1 <= (int)sub_a0b0((struct_11 *)&v26[-4],(struct_11 *)&v9[-4])) { // branch-flip
            v25 = v8[1];
            v26 = (unsigned long *)*v8;
            v18 = (unsigned long *)v8[2];
            v5 = *(unsigned long *)(v25 + -0x20);
            v6 = *(unsigned long *)(v25 + -0x18);
            v9 = (unsigned long *)(v25 + -0x20);
            v8[1] = (long)v9;
            *v15 = v5;
            v14[-3] = v6;
            v5 = *(unsigned long *)(v25 + -8);
            v14[-2] = *(unsigned long *)(v25 + -0x10);
            v14[-1] = v5;
            if (v18 == v26) goto label_b4b8;
          }
          else {
            v25 = *v8;
            v18 = (unsigned long *)v8[2];
            v5 = *(unsigned long *)(v25 + -0x20);
            v6 = *(unsigned long *)(v25 + -0x18);
            v26 = (unsigned long *)(v25 + -0x20);
            *v8 = (long)v26;
            *v15 = v5;
            v14[-3] = v6;
            v5 = *(unsigned long *)(v25 + -8);
            v9 = (unsigned long *)v8[1];
            v14[-2] = *(unsigned long *)(v25 + -0x10);
            v14[-1] = v5;
            if (v18 == v26) {
label_b4b8:
              v22 = *(char *)((long)v8 + 0x54);
              v12 = (unsigned long *)v8[4];
              v24 = (long)v30 - (long)v26 >> 5;
              v25 = (long)v4 - (long)v9 >> 5;
              v21 = v26;
              v14 = v15;
              goto label_b4e8;
            }
          }
          v11 = v10;
          v14 = v15;
        }
        v21 = (unsigned long *)*v8;
        v22 = *(char *)((long)v8 + 0x54);
        v12 = (unsigned long *)v8[4];
label_b66d:
        v25 = (long)v4 - (long)v9 >> 5;
        v24 = (long)v30 - (long)v26 >> 5;
        v13 = v8[6];
        v20 = v8[5];
        if (v13 != v25) { // branch-flip
label_b4f5:
          if (((v24 == v20) && (v18 = (unsigned long *)v8[3], v18 != v9)) && (v10 = v11 + -1, v26 = v14, v11)) {
            do {
              v15 = &v9[-4];
              v5 = *v15;
              v6 = v9[-3];
              v14 = &v26[-4];
              v8[1] = (long)v15;
              *v14 = v5;
              v26[-3] = v6;
              v5 = v9[-1];
              v26[-2] = v9[-2];
              v26[-1] = v5;
              if (v15 == v18) {
                v25 = (long)v4 - (long)v15 >> 5;
                goto label_b4fe;
              }
              v10 -= 1;
              v26 = v14;
              v9 = v15;
            } while (v10 != -1);
            v25 = (long)v4 - (long)v15 >> 5;
          }
        }
        else if ((v18 != v21) && (v10 = v11 + -1, v9 = v14, v4 = v21, v11)) {
          do {
            v21 = &v4[-4];
            v5 = *v21;
            v6 = v4[-3];
            v14 = &v9[-4];
            *v8 = (long)v21;
            *v14 = v5;
            v9[-3] = v6;
            v5 = v4[-1];
            v9[-2] = v4[-2];
            v9[-1] = v5;
            if (v21 == v18) break;
            v10 -= 1;
            v9 = v14;
            v4 = v21;
          } while (v10 != -1);
        }
      }
      else {
        v22 = 0;
        v25 = 0;
        v24 = 0;
label_b4e8:
        v13 = v8[6];
        v20 = v8[5];
        v11 = v10;
        if (v25 != v13) goto label_b4f5;
      }
label_b4fe:
      *v12 = v14;
    }
    else {
      v9 = v4;
      if (v30 != v18) { // branch-flip
        v24 = v10;
        while ((unsigned long *)v8[3] != v9) {
          v10 = v24 + -1;
          if (!v24) {
            v24 = -1;
            break;
          }
          if (1 <= (int)sub_a0b0((struct_11 *)&v21[-4],(struct_11 *)&v9[-4])) { // branch-flip
            v25 = v8[1];
            v8[1] = v25 + -0x20;
            sub_a3c0((long *)(v25 + -0x20),a5,v7);
            v21 = (unsigned long *)*v8;
            v9 = (unsigned long *)v8[1];
            if (v21 == (unsigned long *)v8[2]) goto label_b5f5;
          }
          else {
            v25 = *v8;
            *v8 = v25 + -0x20;
            sub_a3c0((long *)(v25 + -0x20),a5,v7);
            v21 = (unsigned long *)*v8;
            v9 = (unsigned long *)v8[1];
            if (v21 == (unsigned long *)v8[2]) {
label_b5f5:
              v13 = (long)v30 - (long)v21 >> 5;
              v25 = (long)v4 - (long)v9 >> 5;
              goto label_b618;
            }
          }
          v24 = v10;
        }
        v25 = v8[6];
        if ((long)v4 - (long)v9 >> 5 != v25) { // branch-flip
          v20 = v8[5];
          v13 = (long)v30 - (long)v21 >> 5;
label_b8a8:
          if (((v20 != v13) || (v9 == (unsigned long *)v8[3])) || (!v24)) {
            v22 = *(char *)((long)v8 + 0x54);
            v25 = (long)v4 - v8[1] >> 5;
            v13 = v8[6];
          }
          else {
            do {
              v24 -= 1;
              v8[1] = (long)&v9[-4];
              sub_a3c0(&v9[-4],a5,v7);
              v9 = (unsigned long *)v8[1];
              if (v9 == (unsigned long *)v8[3]) break;
            } while (v24);
            v22 = *(char *)((long)v8 + 0x54);
            v21 = (unsigned long *)*v8;
            v20 = v8[5];
            v25 = (long)v4 - (long)v9 >> 5;
            v13 = v8[6];
          }
        }
        else if (((unsigned long *)v8[2] != v21) && (v24)) {
          do {
            v24 -= 1;
            *v8 = (long)&v21[-4];
            sub_a3c0(&v21[-4],a5,v7);
            v21 = (unsigned long *)*v8;
            if (v21 == (unsigned long *)v8[2]) break;
          } while (v24);
          v22 = *(char *)((long)v8 + 0x54);
          v25 = (long)v4 - v8[1] >> 5;
          v20 = v8[5];
          v13 = v8[6];
        }
        else {
          v22 = *(char *)((long)v8 + 0x54);
          v20 = v8[5];
          v13 = v25;
        }
      }
      else {
        v25 = 0;
        v13 = 0;
label_b618:
        v20 = v8[5];
        v24 = v10;
        if (v8[6] != v25) goto label_b8a8;
        v22 = *(char *)((long)v8 + 0x54);
        v13 = v25;
      }
    }
    v8[5] = v20 - ((long)v30 - (long)v21 >> 5);
    v8[6] = v13 - v25;
    if (!v22)
      sub_b1a0(a4,v8);
    if (2 <= *(unsigned int *)&v8[10]) { // branch-flip
      pthread_mutex_lock((pthread_mutex_t *)(v8[7] + 0x58));
      v19 = (struct_12 *)v8[7];
      if (!v19->field_0x54) {
        sub_b1a0(a4,v19);
        v19 = (struct_12 *)v8[7];
      }
      pthread_mutex_unlock((pthread_mutex_t *)&v19[1]);
    }
    else if (!(v8[6] + v8[5])) {
      v10 = v8[7];
      pthread_mutex_lock(v1);
      sub_df40((long *)*a4,v10);
      *(char *)(v10 + 0x54) = 1;
      pthread_cond_signal(v23);
      pthread_mutex_unlock(v1);
    }
    pthread_mutex_unlock(v2);
  } while( true );
}


// Function: sub_ba80 @ 0xba80
unsigned long sub_ba80(unsigned long *a0)
{
  sub_b220(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}


// Function: sub_bab0 @ 0xbab0
void sub_bab0(int a0)
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
  
  v6 = dat_1d888;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v6,v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Write sorted concatenation of all FILE(s) to standard output.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
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
      if (!strcmp("sort",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "sort";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "sort";
    if (!strcmp("sort","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "sort")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}


// Function: sub_ccd0 @ 0xccd0
unsigned long sub_ccd0(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}


// Function: sub_ccf0 @ 0xccf0
bool sub_ccf0(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_cd00 @ 0xcd00
unsigned long sub_cd00(struct_2 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x17200)
    return 1;
  v1 = v2[2];
  if ((((dat_17214 < v1) && (v1 < dat_17218)) && (dat_1721c < v2[3])) && (dat_170f0 <= *v2)) {
    v3 = *v2 + dat_17214;
    if (((v3 < v2[1]) && (v2[1] <= dat_17220)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x17200;
  return 0;
}


// Function: sub_cd90 @ 0xcd90
unsigned long sub_cd90(struct_3 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + a0->field_0x0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*a0->field_0x38)(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*a0->field_0x38)(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    v4 = *v5;
  }
  if (a3) {
    v1 = (long *)v5[1];
    if (v1) {
      v2 = v1[1];
      *v5 = *v1;
      v5[1] = v2;
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}


// Function: sub_cfe0 @ 0xcfe0
unsigned long sub_cfe0(struct_4 *a0,struct_17 *a1,bool a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = a1->field_0x0;
  if (a1->field_0x8 <= v6)
    return 1;
label_d016:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0->field_0x10;
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*a0->field_0x30)(v8,v4);
          v4 = a0->field_0x10;
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + a0->field_0x0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0->field_0x48;
            a0->field_0x18 = a0->field_0x18 + 1;
            *v7 = 0;
            v7[1] = v8;
            a0->field_0x48 = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*a0->field_0x30)(v8,a0->field_0x10);
        if ((unsigned long)a0->field_0x10 <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + a0->field_0x0);
        if (*v7) { // branch-flip
          v5 = (long *)a0->field_0x48;
          if (v5) // branch-flip
            a0->field_0x48 = v5[1];
          else {
            v5 = malloc(0x10);
            if (!v5)
              return 0;
          }
          v1 = v7[1];
          *v5 = v8;
          v5[1] = v1;
          v7[1] = (long)v5;
        }
        else {
          *v7 = v8;
          a0->field_0x18 = a0->field_0x18 + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1->field_0x18 = a1->field_0x18 + -1;
        if (a1->field_0x8 <= v6)
          return 1;
        goto label_d016;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}


// Function: sub_d250 @ 0xd250
void sub_d250(unsigned long *a0,FILE *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long *v4;
  unsigned long v5;
  unsigned long v6;
  
  v6 = 0;
  v1 = a0[2];
  v2 = a0[3];
  for (v4 = (long *)*a0; v4 < (long *)a0[1]; v4 = &v4[2]) {
    while (!*v4) {
      v4 = &v4[2];
      if ((long *)a0[1] <= v4) goto label_d2bd;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_d2bd:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_170f0) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_d3b0 @ 0xd3b0
long sub_d3b0(struct_18 *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = a0->field_0x38;
      if ((*v1)(a1)) break;
      v3 = (long *)v3[1];
      if (!v3)
        return 0;
      v4 = *v3;
    }
    return *v3;
  }
  return 0;
}


// Function: sub_d470 @ 0xd470
long sub_d470(struct_19 *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = v3;
  do {
    v1 = *v4;
    v4 = (long *)v4[1];
    if (v1 == a1) {
      if (v4)
        return *v4;
      break;
    }
  } while (v4);
  do {
    v3 = &v3[2];
    if (a0->field_0x8 <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}


// Function: sub_d550 @ 0xd550
long sub_d550(struct_17 *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = a0->field_0x0;
  if (a0->field_0x8 <= v4) // branch-flip
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
    } while (v4 < a0->field_0x8);
  }
  return v3;
}


// Function: sub_d640 @ 0xd640
unsigned long * sub_d640(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_ccd0;
  if (!a3)
    a3 = sub_ccf0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x17200;
  v1[5] = a1;
  if (sub_cd00(v1)) {
    v2 = sub_cea0(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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


// Function: sub_d750 @ 0xd750
void sub_d750(struct_21 *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = a0->field_0x0;
  if (a0->field_0x0 < a0->field_0x8) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if (a0->field_0x8 <= v5) goto label_d7eb;
      }
      v3 = a0->field_0x40;
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = a0->field_0x40;
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0->field_0x48;
          *v4 = 0;
          v4[1] = v2;
          a0->field_0x48 = v4;
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
    } while (v6 < a0->field_0x8);
  }
label_d7eb:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_d800 @ 0xd800
void sub_d800(struct_22 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_d8ac;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_d86f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0->field_0x40)(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = a0->field_0x8;
      v4 = &v4[2];
    } while (v4 < v3);
label_d86f:
    v4 = a0->field_0x0;
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
    } while (v4 < a0->field_0x8);
  }
label_d8ac:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_d8f0 @ 0xd8f0
unsigned int sub_d8f0(struct_6 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_cea0(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
  if (!v4)
    return 0;
  if (a0->field_0x10 == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0->field_0x28;
  v6 = 0;
  v8 = a0->field_0x30;
  v9 = a0->field_0x38;
  v10 = a0->field_0x40;
  v11 = a0->field_0x48;
  v1 = sub_cfe0(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_cfe0(a0,&v2,1)) && (sub_cfe0(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free(a0->field_0x0);
  a0->field_0x0 = v2;
  a0->field_0x8 = v3;
  a0->field_0x10 = v4;
  a0->field_0x18 = v5;
  a0->field_0x48 = v11;
  return v1;
}


// Function: sub_da70 @ 0xda70
long * sub_da70(void *a0,long a1,long *a2) // return-dupe, ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4;
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (!a1) {
    abort(); // no-return, return-dupe
  }
  v2 = (long)sub_cd90(a0,a1,&v3,0);
  if (v2) {
    if (!a2) {
      v2 = NULL;
      return v2;
    }
    *a2 = (long)v2;
    v2 = NULL;
    return v2;
  }
  v4 = *(unsigned long *)((long)a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)((long)a0 + 0x10);
    if ((long)v2 > -1) goto label_daff;
label_db86:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)((long)a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_db86;
label_daff:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
    sub_cd00(a0);
    v1 = *(long *)((long)a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_17224 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_17228 <= v5) ? (long)(v5 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_d8f0(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_cd90(a0,a1,&v3,0);
      if (v2)
        abort();
    }
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)((long)a0 + 0x48);
  if (v2) // branch-flip
    *(long *)((long)a0 + 0x48) = v2[1];
  else {
    v2 = malloc(0x10);
    if (!v2) {
      v2 = (long *)0xffffffff;
      return v2;
    }
  }
  v1 = v3[1];
  *v2 = a1;
  v2[1] = v1;
  v3[1] = (long)v2;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}


// Function: sub_dca0 @ 0xdca0
unsigned long sub_dca0(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_da70(a0,a1,&v3);
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


// Function: sub_dd00 @ 0xdd00
long sub_dd00(void *a0,long a1) // ternary x2
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
  
  v5 = sub_cd90(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)((long)a0 + 0x18) - 1;
    *(unsigned long *)((long)a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)((long)a0 + 0x10) : *(unsigned long *)((long)a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)((long)a0 + 0x28) * v8) {
      sub_cd00(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_17228 <= v8) ? (long)(v8 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_d8f0(a0,v6)) {
          v4 = *(void **)((long)a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)((long)a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}


// Function: sub_e570 @ 0xe570
unsigned long sub_e570(int a0)
{
  unsigned long v1; // rax
  char *v2; // rax
  unsigned long v3;
  char v4 [136];
  
  if (a0 != 2) { // branch-flip
    if (a0 != 1) {
      v1 = sysconf(0x53);
      v3 = v1;
      if (v1 - 1 < 2) {
        if (sched_getaffinity(0,0x80,v4))
          return v1;
        v3 = (unsigned long)__sched_cpucount(0x80,v4);
        if (!v3)
          return v1;
        if (v3 <= v1)
          return v1;
      }
      if ((long)v3 > 0)
        return v3;
      return 1;
    }
  }
  else {
    v2 = getenv("OMP_NUM_THREADS");
    if (v2) {
      v1 = sub_e4b0(v2);
      v2 = getenv("OMP_THREAD_LIMIT");
      if (v2) { // branch-flip
        v3 = sub_e4b0(v2);
        if (!v3)
          v3 = 0xffffffffffffffff;
      }
      else {
        v3 = 0xffffffffffffffff;
      }
      if (v1) {
        if (v3 > v1)
          return v1;
        return v3;
      }
      goto label_e64f;
    }
    v2 = getenv("OMP_THREAD_LIMIT");
    if ((v2) && (v3 = sub_e4b0(v2), v3)) goto label_e64f;
  }
  v3 = 0xffffffffffffffff;
label_e64f:
  if (!sched_getaffinity(0,0x80,v4)) {
    v1 = (unsigned long)__sched_cpucount(0x80,v4);
    if (v1) {
      if (v3 > v1)
        return v1;
      return v3;
    }
  }
  v1 = sysconf(0x54);
  if (v1 <= v3)
    v3 = v1;
  if ((long)v1 <= 0)
    v3 = 1;
  return v3;
}

