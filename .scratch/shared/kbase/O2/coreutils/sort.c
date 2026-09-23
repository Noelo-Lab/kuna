// Function: sub_3ec0 @ 0x3ec0
unsigned long sub_3ec0(unsigned int a0,char **a1) // ternary x3
{
  unsigned char *v1;
  unsigned char v10;
  unsigned char v100; // stack - 0x3b2
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
  unsigned char v12;
  unsigned char v120; // stack - 0x2a2
  unsigned char v121; // stack - 0x2a1
  unsigned char v122; // stack - 0x2a0
  unsigned int v123; // stack - 0x284
  unsigned long *v124; // stack - 0x280
  unsigned long v125; // stack - 0x158
  unsigned long v126; // stack - 0x150
  unsigned long v127; // stack - 0x148
  unsigned long v128; // stack - 0x140
  unsigned long v129; // stack - 0x130
  unsigned char v13;
  unsigned long v130; // stack - 0x128
  unsigned long v131; // stack - 0x120
  unsigned long v132; // stack - 0x118
  unsigned long v133; // stack - 0x110
  unsigned long v134; // stack - 0x108
  unsigned long v135; // stack - 0x100
  unsigned long v136; // stack - 0xf8
  unsigned long v137; // stack - 0xf0
  unsigned long v138; // stack - 0xe8
  unsigned int v139; // stack - 0xe0
  char v14;
  long v140; // stack - 0x40
  char v15;
  unsigned char v16;
  int v17;
  int v18; // eax
  unsigned int v19; // eax
  unsigned long *v2;
  unsigned long *v20;
  long *v21;
  long *v22;
  long v23;
  unsigned char *v24;
  unsigned long v25;
  long v26;
  char *v27;
  char *v28;
  unsigned long v29;
  void **v3;
  unsigned long *v30;
  void *v31;
  unsigned long *v32; // rax
  unsigned long *v33;
  unsigned long *v34; // rax
  void *v35;
  void *v36; // rax
  long *v37;
  unsigned long v38; // rax
  unsigned long v39; // rax
  unsigned short v4;
  unsigned long v40; // rax
  unsigned long v41; // rax
  unsigned long v42;
  unsigned long v43; // rax
  unsigned long v44; // rax
  unsigned long v45; // rax
  unsigned long v46; // rax
  unsigned long v47; // rax
  unsigned long *v48;
  unsigned long v49;
  unsigned int v5;
  unsigned char v50;
  unsigned int v51;
  unsigned long *v52;
  void *v53; // stack - 0x168
  void *v54; // stack - 0x398
  unsigned long v55; // stack - 0x328
  unsigned int v56; // stack - 0x78
  void *v57; // stack - 0x2d8
  char v58 [32];
  char v59 [47];
  long v6;
  char v60; // stack - 0xa8
  int v61; // stack - 0x288
  long v62; // stack - 0x378
  unsigned long v63; // stack - 0x138
  unsigned long v64; // stack - 0x160
  long v65; // stack - 0x368
  long v66; // stack - 0x370
  long v67 [4]; // stack - 0x360
  unsigned char v68;
  char v69;
  bool v7;
  unsigned long *v70;
  char v71; // dil
  void **v72;
  unsigned int *v73;
  int *v74; // r12
  char *v75;
  void *v76;
  unsigned char v77;
  int *v78; // r13
  unsigned char v79;
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
  long v91; // stack - 0x458
  char *v92; // stack - 0x438
  char *v93; // stack - 0x430
  void **v94; // stack - 0x420
  unsigned long v95; // stack - 0x408
  char *v96; // stack - 0x3f8
  long v97; // stack - 0x3e0
  char *v98; // stack - 0x3d8
  unsigned long v99; // stack - 0x3d0
  
  v140 = *(long *)(v82 + 0x28);
  v94 = (char *)getenv("POSIXLY_CORRECT");
  v83 = v94 == NULL;
  v17 = sub_eb70();
  v85 = 0x2b8 < (unsigned int)(v17 - 0x30db0U);
  sub_ec00(*a1);
  v93 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  dat_1d130 = 2;
  dat_1d861 = sub_cc40(3);
  v14 = sub_cc40(2);
  dat_1d860 = v14;
  v20 = (unsigned long *)localeconv();
  dat_1d868 = *(unsigned char *)*v20;
  if ((!dat_1d868) || (((unsigned char *)*v20)[1]))
    dat_1d868 = 0x2e;
  v15 = *(char *)v20[1];
  dat_1d864 = (unsigned int)v15;
  if (v15) {
    if (!((char *)v20[1])[1]) goto label_3fc7;
    dat_1d862 = '\x01';
  }
  dat_1d864 = 0x80;
label_3fc7:
  dat_1d418 = '\0';
  v21 = __ctype_b_loc();
  v22 = __ctype_toupper_loc();
  v23 = 0;
  do {
    v4 = *(unsigned short *)(*v21 + v23 * 2);
    if (v4 & 1) { // branch-flip
      v71 = 1;
      v50 = 0;
label_402e:
      *(unsigned char *)(v23 + 0x1d540) = v50;
      v26 = *v22;
      *(char *)(v23 + 0x1d740) = v71;
      v5 = *(unsigned int *)(v26 + v23 * 4);
      *(unsigned char *)(v23 + 0x1d640) = ((unsigned char)(v4 >> 0xe) ^ 1) & 1;
      *(char *)(v23 + 0x1d440) = (char)v5;
      if (v23 == 0xff) goto label_40b8;
    }
    else {
      if (v23 != 10) {
        v71 = 0;
        v50 = ((unsigned char)(v4 >> 3) ^ 1) & 1;
        goto label_402e;
      }
      dat_1d74a = 1;
      dat_1d54a = 0;
      dat_1d64a = ((unsigned char)((unsigned short)*(unsigned short *)(*v21 + 0x14) >> 0xe) ^ 1) & 1;
      dat_1d44a = (char)*(unsigned int *)(*v22 + 0x28);
    }
    v23 += 1;
  } while( true );
label_60e5:
  sub_7790(&v57,v97,v35);
label_5385:
  v94 = &v57;
  v95 -= 1;
  v114 = 0;
  while( true ) {
    v14 = sub_7ef0(v94,v31,v42);
    v50 = v114;
    v26 = v111;
    v81 = v110;
    v76 = v57;
    if (!v14) break;
    if (v114) { // branch-flip
      if (v95) {
        if ((v111 - v109) - v97 * v110 <= v97 + 1U) goto label_5402;
        v112 = v109;
        sub_7660(v31,v42);
        goto label_5d45;
      }
      dat_1d840 = 0;
      if (v91 || v112) goto label_5402;
      sub_7660(v31,v42);
      v35 = (void *)sub_74e0(v96,"w");
      if (!v35) {
        sub_7480(dcgettext(NULL,"open failed",5),v96); // return-dupe, no-return
      }
      v54 = v35;
      v91 = 0;
      v75 = v96;
      v81 = v110;
    }
    else {
label_5402:
      dat_1d840 = 0;
      v91 += 1;
      v50 = 0;
      v75 = (char *)(sub_8c20(&v54,0) + 0xd);
    }
    v26 += (long)v76;
    if (2 <= v81) { // branch-flip
      v53 = (void *)sub_deb0(sub_7c20,v23);
      pthread_mutex_init(&v64,0);
      pthread_cond_init(&v63,0);
      v30 = (unsigned long *)sub_12ab0(v25 << 8);
      v30[3] = 0;
      v30[2] = 0;
      v30[1] = 0;
      *v30 = 0;
      v30[4] = 0;
      v30[6] = v81;
      v30[5] = v81;
      v30[7] = 0;
      *(unsigned int *)&v30[10] = 0;
      *(char *)((long)v30 + 0x54) = 0;
      pthread_mutex_init(&v30[0xb],0);
      sub_7af0(v30,&v30[0x10],v26,v25,v81,0);
      sub_b220(v26,v25,v81,&v30[0x10],&v53,v54);
      if (v23) {
        v33 = &v30[0xb];
        v26 = v23 + -1;
        do {
          v26 -= 1;
          v32 = &v33[0x10];
          pthread_mutex_destroy(v33);
          v33 = v32;
        } while (v26 != -1);
      }
      free(v30);
      sub_df20(v53);
      pthread_cond_destroy(&v63);
      pthread_mutex_destroy(&v64);
    }
    else {
      sub_a3c0(v26 + -0x20,v54,v75);
    }
    sub_7660(v54,v75);
    if (v50) {
      free(v57);
      goto label_5599;
    }
  }
  sub_7660(v31,v42);
  if (!v95) goto label_64ce;
label_5d45:
  v20 = &v20[1];
  goto label_5328;
label_64ce:
  free(v57);
  v20 = dat_1d368;
  v21 = (long *)sub_12bf0(v91,0x10);
  v22 = v21;
  while (v20) {
    v22[1] = (long)v20;
    v23 = (long)v20 + 0xd;
    v20 = (unsigned long *)*v20;
    v37 = &v22[2];
    *v22 = v23;
    v22 = v37;
  }
  sub_ac90(v21,v91,v91,v96);
  free(v21);
label_5599:
  while (1 <= dat_1d35c) {
    sub_78c0(0xffffffff);
  }
  goto label_55b4;
label_40b8:
  if (v14) {
    v23 = 1;
    v22 = (long *)0x1d060;
    do {
      v24 = (unsigned char *)nl_langinfo((int)v23 + 0x2000d);
      v25 = strlen((char *)v24);
      v26 = sub_12ab0(v25 + 1);
      *(int *)&v22[1] = (int)v23;
      *v22 = v26;
      if (v25) {
        v6 = *v21;
        v1 = &v24[v25];
        v25 = 0;
        do {
          if (!(*(unsigned char *)(v6 + (unsigned long)*v24 * 2) & 1)) {
            *(char *)(v26 + v25) = *(char *)((unsigned long)*v24 + 0x1d440);
            v25 += 1;
          }
          v24 = &v24[1];
        } while (v1 != v24);
      }
      v23 += 1;
      *(char *)(v26 + v25) = 0;
      v22 = &v22[2];
    } while (v23 != 0xd);
    qsort(0x1d060,0xc,0x10,sub_7300);
  }
  v78 = (int *)0x15324;
  v74 = (int *)0x15324;
  sigemptyset((void *)0x1d380);
  v17 = 0xe;
  while( true ) {
    sigaction(v17,NULL,&v53);
    if (v53 != (void *)0x1)
      sigaddset((void *)0x1d380,v17);
    if (v74 == (int *)0x1534c) break;
    v17 = *v74;
    v74 = &v74[1];
  }
  v139 = 0;
  v53 = sub_8270;
  v17 = 0xe;
  v64 = dat_1d380;
  v125 = dat_1d388;
  v126 = dat_1d390;
  v127 = dat_1d398;
  v128 = dat_1d3a0;
  v63 = dat_1d3a8;
  v129 = dat_1d3b0;
  v130 = dat_1d3b8;
  v131 = dat_1d3c0;
  v132 = dat_1d3c8;
  v133 = dat_1d3d0;
  v134 = dat_1d3d8;
  v135 = dat_1d3e0;
  v136 = dat_1d3e8;
  v137 = dat_1d3f0;
  v138 = dat_1d3f8;
  while( true ) {
    if (sigismember(0x1d380,v17))
      sigaction(v17,&v53,NULL);
    if (v78 == (int *)0x1534c) break;
    v17 = *v78;
    v78 = &v78[1];
  }
  v81 = 0;
  signal(0x11,0);
  sub_142c0(sub_8ed0);
  v75 = "-bcCdfghik:mMno:rRsS:t:T:uVy:z";
  v105._0_16_ = ZEXT816(0xffffffffffffffff) << 0x40;
  v55 = 0xffffffffffffffff;
  v106._0_16_ = 0;
  v107._0_16_ = 0;
  v108._0_16_ = 0;
  v20 = (unsigned long *)sub_12bf0((long)(int)a0,8);
  v8 = 0;
  v25 = 0;
  v96 = NULL;
  v92 = NULL;
  v99 = 0;
  v98 = NULL;
  v14 = '\0';
  v91._0_4_ = a0;
label_43e0:
  v54 = (void *)CONCAT44(v54._4_4_,0xffffffff);
  if ((int)v25 == -1) {
label_43f1:
    v23 = (long)(int)optind;
    goto label_43c0;
  }
  if ((!v81) || (v83)) {
label_4410:
    v72 = &v54;
    v51 = getopt_long((unsigned int)v91,a1,v75,(void *)0x1c5e0,(int *)v72);
    v31 = v54;
    v27 = optarg;
    v25 = (unsigned long)v51;
    if (v51 == 0xffffffff) goto label_43f1;
    if (0x87 < (int)v51) {
      sub_bab0(2); // return-dupe, no-return
    }
    if ((int)v51 <= 0) {
      if (v51 == 0xffffff7d) {
        sub_128e0(stdout,"sort","GNU coreutils",dat_1d120,"Mike Haertel","Paul Eggert",0,v72);
        exit(0); // no-return
      }
      if (v51 == 0xffffff7e)
        sub_bab0(0); // no-return
      sub_bab0(2);
    }
    if (0x87 < v51) {
      sub_bab0(2);
    }
    v17 = (int)v54;
    switch(v51) {
      default:
        goto label_661d;
      case 1:
        if (*optarg == '+') {
          if (optind != (unsigned int)v91) { // branch-flip
            v84 = 0;
            if (*a1[(int)optind] == '-')
              v84 = (unsigned int)((int)a1[(int)optind][1] - 0x30U) <= 9;
          }
          else {
            v84 = 0;
          }
          v85 = (bool)(v85 | v94 == NULL & v84);
          if (v85) {
            v23 = 0x12;
            v22 = &v62;
            while (v23) {
              v37 = (long *)((long)v22 + 4);
              *(unsigned int *)v22 = 0;
              v23 -= 1;
              v22 = v37;
            }
            v65 = -1;
            v27 = (char *)sub_7390(&optarg[1],&v62,0);
            if (v27) { // branch-flip
              if (*v27 != '.') { // branch-flip
                if (!v66 && !v62) {
label_61b5:
                  v62 = -1;
                  goto label_61c1;
                }
              }
              else {
                v27 = (char *)sub_7390(&v27[1],&v66,0);
                if (!v62 && !v66) goto label_61b5;
label_61c1:
                if (!v27) goto label_67de;
              }
              if (!*(char *)sub_71f0(v27,&v62,0)) {
                if (v84) {
                  v23 = (long)(int)optind;
                  optind += 1;
                  v27 = a1[v23];
                  v28 = (char *)sub_7390(&v27[1],&v65,"invalid number after \'-\'");
                  if (!v28)
                    __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                  if (*v28 == '.')
                    v28 = (char *)sub_7390(&v28[1],v67,"invalid number after \'.\'");
                  if ((!v67[0]) && (v65))
                    v65 -= 1;
                  if (*(char *)sub_71f0(v28,&v62,1))
                    sub_7340(v27,"stray character in field spec"); // no-return
                }
                v104 = 1;
                sub_76e0(&v62);
                v25 = (unsigned long)v51;
                break;
              }
            }
            else if (!v66 && !v62)
              v62 = -1;
label_67de:
            v27 = optarg;
          }
        }
        v25 = (unsigned long)v51;
        v20[v81] = v27;
        v81 += 1;
        break;
      case 0x43:
      case 99:
label_4527:
        if ((v14) && ((int)v14 != (int)v25))
          sub_7310("cC"); // no-return
        v14 = (char)v25;
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
        v17 = sub_132f0(optarg,&v61,10,&v57,"EgGkKmMPtTYZ");
        if (v17) { // branch-flip
          if (((v17 != 2) || (v28 = (char *)CONCAT44(v123,v61), 10 <= (unsigned int)((int)v28[-1] - 0x30U))) || (v28[1])) {
            sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27); // return-dupe, no-return
          }
          if (*v28 != '%') { // branch-flip
            if (*v28 != 'b') {
              sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27);
            }
          }
          else {
            sub_e760();
            v90 = ((double)v57 * v89) / dat_170f0;
            if (dat_170f8 <= v90) goto label_6691;
            if (v90 < dat_17100) {
              v76 = (void *)(long)v90;
              v57 = (void *)(long)v90;
              goto label_56b9;
            }
            v57 = (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000);
          }
          v76 = v57;
        }
        else {
          v76 = v57;
          if ((unsigned int)((int)*(char *)(CONCAT44(v123,v61) + -1) - 0x30U) <= 9) {
            if ((void *)0x3fffffffffffff < v57) {
label_6691:
              v17 = 1;
              sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27);
            }
            v76 = (void *)((long)v57 << 10);
            v57 = v76;
          }
        }
label_56b9:
        v25 = (unsigned long)v51;
        if ((dat_1d438 <= v76) && (dat_1d438 = (void *)((unsigned long)dat_1d024 * 0x22), (void *)((unsigned long)dat_1d024 * 0x22) < v76))
          dat_1d438 = v76;
        break;
      case 0x54:
        sub_7730(optarg);
        v25 = (unsigned long)v51;
        break;
      case 0x6b:
        v23 = 0x12;
        v22 = &v62;
        while (v23) {
          v37 = (long *)((long)v22 + 4);
          *(unsigned int *)v22 = 0;
          v23 -= 1;
          v22 = v37;
        }
        v65 = -1;
        v27 = (char *)sub_7390(optarg,&v62,"invalid number at field start");
        v23 = v62 + -1;
        v84 = v62 == 0;
        v62 = v23;
        if (v84) {
          sub_7340(optarg,"field number is zero"); // return-dupe, no-return
        }
        if (*v27 == '.') {
          v27 = (char *)sub_7390(&v27[1],&v66,"invalid number after \'.\'");
          v23 = v66 + -1;
          v84 = v66 == 0;
          v66 = v23;
          if (v84)
            sub_7340(optarg,"character offset is zero"); // no-return
        }
        if (!v62 && !v66)
          v62 = -1;
        v27 = (char *)sub_71f0(v27,&v62,0);
        if (*v27 != ',') { // branch-flip
          v65 = -1;
          v67[0] = 0;
        }
        else {
          v27 = (char *)sub_7390(&v27[1],&v65,"invalid number after \',\'");
          v23 = v65 + -1;
          v84 = v65 == 0;
          v65 = v23;
          if (v84) {
            sub_7340(optarg,"field number is zero");
          }
          if (*v27 == '.')
            v27 = (char *)sub_7390(&v27[1],v67,"invalid number after \'.\'");
          v27 = (char *)sub_71f0(v27,&v62,1);
        }
        if (*v27) {
          sub_7340(optarg,"stray character in field spec"); // return-dupe, no-return
        }
        sub_76e0(&v62);
        v25 = (unsigned long)v51;
        break;
      case 0x6d:
        v8 = 1;
        break;
      case 0x6f:
        if ((v96) && (v17 = strcmp(v96,optarg), v17)) goto label_6dbc;
        v25 = (unsigned long)v51;
        v96 = v27;
        break;
      case 0x73:
        dat_1d41a = '\x01';
        break;
      case 0x74:
        v19 = (unsigned int)*optarg;
        if (!*optarg) {
          error(2,0,dcgettext(NULL,"empty tab",5));
          return v39;
        }
        if (optarg[1]) {
          v17 = strcmp(optarg,"\\0");
          if (v17) {
            v42 = sub_10fd0(v27);
            error(2,0,dcgettext(NULL,"multi-character tab %s",5),v42);
            return v40;
          }
          v19 = 0;
        }
        v25 = (unsigned long)v51;
        if ((dat_1d028 != 0x80) && (dat_1d028 != v19)) {
          error(2,0,dcgettext(NULL,"incompatible tabs",5));
label_6dbc:
          error(2,0,dcgettext(NULL,"multiple output files specified",5));
          sub_7340(optarg,"field number is zero");
        }
        dat_1d028 = v19;
        break;
      case 0x75:
        dat_1d419 = 1;
        break;
      case 0x79:
        if (a1[(long)(int)optind + -1] == optarg) {
          for (; (unsigned int)((int)*v27 - 0x30U) <= 9; v27 = &v27[1]) {
          }
          optind = (optind - 1) + (unsigned int)(*v27 == '\0');
        }
        break;
      case 0x7a:
        dat_1d038 = 0;
        break;
      case 0x80:
        v25 = 99;
        if (optarg) {
          v23 = sub_c370("--check",optarg,0x1c5c0,"CCc",1,dat_1d128,1,99);
          v25 = (unsigned long)(unsigned int)(int)"CCc"[v23];
        }
        goto label_4527;
      case 0x81:
        if ((dat_1d408) && (v17 = strcmp(dat_1d408,optarg), v17)) {
label_699a:
          error(2,0,dcgettext(NULL,"multiple compress programs specified",5));
          sub_7340(optarg,"stray character in field spec");
        }
        v25 = (unsigned long)v51;
        dat_1d408 = v27;
        break;
      case 0x82:
        dat_1d400 = '\x01';
        break;
      case 0x83:
        goto label_4ef5;
      case 0x84:
        v18 = sub_132f0(optarg,0,10,&v57,"");
        v25 = (unsigned long)v51;
        v51 = 0x11;
        if (!getrlimit(7,&v61))
          v51 = v61 - 3;
        if (v18) { // branch-flip
          if (v18 != 1)
            sub_13230(v18,(unsigned long)v31 & 0xffffffff,0xffffff84,0x1c5e0,v27); // no-return
        }
        else {
          dat_1d024 = (unsigned int)v57;
          if ((void *)((unsigned long)v57 & 0xffffffff) == v57) {
            if (dat_1d024 < 2) {
              v29 = sub_10fd0(v27);
              v42 = *(unsigned long *)((long)v17 * 0x20 + 0x1c5e0);
              error(0,0,dcgettext(NULL,"invalid --%s argument %s",5),v42,v29);
              v29 = sub_10fd0("2");
              error(2,0,dcgettext(NULL,"minimum --%s argument is %s",5),v42,v29);
              return v41;
            }
            if (dat_1d024 <= v51) break;
          }
        }
        v42 = sub_10fd0(v27);
        v75 = *(char **)((long)v17 * 0x20 + 0x1c5e0);
        error(0,0,dcgettext(NULL,"--%s argument %s too large",5),v75,v42);
        v21 = (long *)sub_e1b0(v51,&v56);
        error(2,0,dcgettext(NULL,"maximum --%s argument with current rlimit is %s",5),v75,v21);
        v91._0_4_ = v51;
label_4ef5:
        v92 = optarg;
        break;
      case 0x85:
label_4d92:
        v27 = optarg;
        if (v98) {
          v17 = strcmp(v98,optarg);
          v25 &= 0xffffffff;
          if (v17) {
            error(2,0,dcgettext(NULL,"multiple random sources specified",5));
            goto label_699a;
          }
        }
        v98 = v27;
        break;
      case 0x86:
        v23 = sub_c370("--sort",optarg,0x1c580,"ghMnRVCCc",1,dat_1d128,1,(long)*(int *)((unsigned long)v51 * 4 + 0x150cc) + 0x150cc);
        v51 = (unsigned int)"ghMnRVCCc"[v23];
label_44a1:
        v56._0_2_ = (unsigned short)(unsigned char)v51;
        sub_71f0(&v56,&v55,2);
        v25 = (unsigned long)v51;
        break;
      case 0x87:
        v17 = sub_132f0(optarg,0,10,&v61,"");
        v25 = (unsigned long)v51;
        if (v17 != 1) { // branch-flip
          if (v17)
            sub_13230(v17,(unsigned long)v31 & 0xffffffff,0xffffff87,0x1c5e0,v27); // no-return
          v99 = CONCAT44(v123,v61);
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
  v23 = (long)(int)optind;
  if ((v85) && (!v14)) {
    if ((unsigned int)v91 != optind) {
      v27 = a1[(int)optind];
      if (((*v27 == '-') && (v27[1] == 'o')) && ((v27[2] || (optind + 1 != (unsigned int)v91)))) goto label_4410;
      goto label_43c0;
    }
    if (!v92) {
      v14 = '\0';
      goto label_4571;
    }
label_65b8:
    v42 = sub_108e0(4,*v20);
    error(0,0,dcgettext(NULL,"extra operand %s",5),v42);
    __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
    sub_bab0(2);
  }
label_43c0:
  if ((int)v23 < (int)(unsigned int)v91) {
    optind = (int)v23 + 1;
    v20[v81] = a1[v23];
    v81 += 1;
    goto label_43e0;
  }
  if (!v92) goto label_4571;
  if (v81) goto label_65b8;
  v23 = sub_74e0(v92,"r");
  if (!v23)
    sub_7c60(v92); // no-return
  sub_119f0(&v61);
  if (!sub_11aa0(v23,&v61)) {
    v42 = sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"cannot read file names from %s",5),v42);
    return v46;
  }
  sub_7660(v23,v92);
  if (!CONCAT44(v123,v61)) {
    v42 = sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"no input from %s",5),v42);
    return v45;
  }
  free(v20);
  v20 = v124;
  if (CONCAT44(v123,v61)) {
    v81 = 0;
    do {
      v75 = (char *)v124[v81];
      if (!strcmp(v75,"-")) {
        v42 = sub_108e0(4,v75);
        error(2,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v42);
        return v43;
      }
      v81 += 1;
      if (!*v75) {
        v42 = sub_10c60(0,3,v92);
        error(2,0,dcgettext(NULL,"%s:%lu: invalid zero-length file name",5),v42,v81);
        return v44;
      }
    } while (CONCAT44(v123,v61) != v81);
  }
label_4571:
  if (dat_1d410) { // branch-flip
    v50 = 0;
    v48 = dat_1d410;
    v52 = dat_1d410;
    do {
      while ((v15 = sub_70e0(v48), !v15 || (*(char *)((long)v48 + 0x37)))) {
        v24 = (unsigned char *)((long)v48 + 0x33);
        v48 = (unsigned long *)v48[8];
        v50 |= *v24;
        if (!v48) goto label_463f;
      }
      v48[4] = v106._8_8_;
      v48[5] = v107._0_8_;
      *(char *)&v48[6] = v107[8];
      *(char *)((long)v48 + 0x31) = v107[9];
      *(char *)((long)v48 + 0x36) = v107[0xe];
      *(char *)((long)v48 + 0x32) = v107[10];
      *(char *)((long)v48 + 0x34) = v107[0xc];
      *(char *)((long)v48 + 0x35) = v107[0xd];
      *(char *)((long)v48 + 0x37) = v107[0xf];
      *(char *)&v48[7] = v108[0];
      *(unsigned char *)((long)v48 + 0x33) = v107[0xb];
      v48 = (unsigned long *)v48[8];
      v50 |= v107[0xb];
    } while (v48);
label_463f:
    v85 = 0;
    do {
      if (1 < (unsigned int)*(unsigned char *)((long)v52 + 0x32) + (unsigned int)*(unsigned char *)((long)v52 + 0x34) + (unsigned int)*(unsigned char *)((long)v52 + 0x35) + (unsigned int)*(unsigned char *)((long)v52 + 0x36) + (unsigned int)(unsigned char)((unsigned char)v52[7] | *(unsigned char *)((long)v52 + 0x33) | v52[4] != 0)) {
        *(char *)((long)v52 + 0x37) = 0;
        v73 = &v56;
        *(unsigned short *)&v52[6] = 0;
        sub_7140(v52,v73);
        sub_7310(v73); // no-return
      }
      v52 = (unsigned long *)v52[8];
label_58ff:
    } while (v52);
    if (dat_1d400) {
label_4775:
      if (v14 || v96) {
        s_1d018[0] = v14;
        if (!v14)
          s_1d018[0] = 'o';
        sub_7310(s_1d018); // no-return
      }
      if ((!v93) || (!setlocale(3,"")))
        error(0,0,"%s",dcgettext(NULL,"failed to set locale",5));
      if (dat_1d861) { // branch-flip
        v42 = sub_10fd0(setlocale(3,NULL));
        error(0,0,dcgettext(NULL,"text ordering performed using %s sorting rules",5),v42);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"text ordering performed using simple byte comparison",5));
      }
      v23 = 0x12;
      v70 = &v55;
      v72 = &v57;
      while (v23) {
        v3 = (void **)((long)v72 + 4);
        v2 = (unsigned long *)((long)v70 + 4);
        *(unsigned int *)v72 = *(unsigned int *)v70;
        v23 -= 1;
        v70 = v2;
        v72 = v3;
      }
      if (dat_1d410) {
        v100 = 0;
        v84 = 0;
        v15 = '\0';
        v83 = 0;
        v94 = (void **)0x1;
        v52 = dat_1d410;
        v80 = v115;
        v79 = v119;
        v77 = v121;
        v68 = v122;
        do {
          v13 = v120;
          v12 = v118;
          v11 = v117;
          v10 = v116;
          v9 = v114;
          if (v52[6] & 0xffff00ff0000) {
            v69 = *(char *)((long)v52 + 0x34);
            if (!v69) {
              v83 = 1;
              v69 = v15;
            }
            v15 = v69;
          }
          if (*(char *)((long)v52 + 0x39)) {
            v25 = *v52;
            if (*v52 == 0xffffffffffffffff)
              v25 = 0;
            v49 = v52[2];
            v60 = 0x2b;
            v75 = __stpcpy_chk(v59,(char *)sub_e200(v25,v58),0x2d);
            v56 = 0x206b2d;
            v27 = __stpcpy_chk((char *)((long)&v56 + 3),(char *)sub_e200(v25 + 1,v58),0x2c);
            if (v52[2] != 0xffffffffffffffff) {
              v28 = (char *)sub_e200(v49 + 1,v58);
              v75[0] = ' ';
              v75[1] = '-';
              strcpy(&v75[2],v28);
              v75 = (char *)sub_e200((unsigned long)(v52[3] == 0xffffffffffffffff) + v49 + 1,v58);
              *v27 = ',';
              strcpy(&v27[1],v75);
            }
            v42 = sub_10fb0(1,&v56);
            v29 = sub_10fb0(0,&v60);
            error(0,0,dcgettext(NULL,"obsolescent key %s used; consider %s instead",5),v29,v42);
          }
          v25 = v52[2];
          v7 = v85;
          if ((*v52 != 0xffffffffffffffff) && (v25 < *v52)) {
            error(0,0,dcgettext(NULL,"key %lu has zero width and will be ignored",5),v94);
            v25 = v52[2];
            v7 = 1;
          }
          v69 = '\x01';
          v49 = v52[6] & 0xffff00ff0000;
          if (!v49)
            v69 = *(char *)((long)v52 + 0x36);
          if ((v25) || (!v52[3])) {
            if (!v7) {
              if ((dat_1d028 == 0x80) && (((!(char)v52[6] && ((!v69 || (v52[1])))) || ((!*(char *)((long)v52 + 0x31) && (v52[3])))))) {
                error(0,0,dcgettext(NULL,"leading blanks are significant in key %lu; consider also specifying \'b\'",5),v94);
                v49 = v52[6] & 0xffff00ff0000;
              }
              goto label_48e3;
            }
label_5a20:
            if (!v85) goto label_48e3;
label_4908:
            v16 = *(unsigned char *)((long)v52 + 0x34);
          }
          else {
            if (v7) goto label_5a20;
label_48e3:
            if (!v49) goto label_4908;
            v25 = *v52 + 1;
            if (!v25)
              v25 = 1;
            if (v52[2] < v25) goto label_4908;
            error(0,0,dcgettext(NULL,"key %lu is numeric and spans multiple fields",5),v94);
            v16 = *(unsigned char *)((long)v52 + 0x34);
            if (v16) // branch-flip
              v100 = v16;
            else {
              v84 = 1;
            }
          }
          if ((v112) && (v112 == v52[4]))
            v112 = 0;
          if ((v113) && (v113 == v52[5]))
            v113 = 0;
          v118 = v12 & (v16 ^ 1);
          v114 = v9 & ((unsigned char)v52[6] ^ 1);
          v79 &= *(unsigned char *)((long)v52 + 0x35) ^ 1;
          v80 &= *(unsigned char *)((long)v52 + 0x31) ^ 1;
          v117 = v11 & (*(unsigned char *)((long)v52 + 0x33) ^ 1);
          v120 = v13 & (*(unsigned char *)((long)v52 + 0x36) ^ 1);
          v122 = v68 & ((unsigned char)v52[7] ^ 1);
          v116 = v10 & (*(unsigned char *)((long)v52 + 0x32) ^ 1);
          v77 &= *(unsigned char *)((long)v52 + 0x37) ^ 1;
          v52 = (unsigned long *)v52[8];
          v94 = (long)((long)v94 + 1);
          v115 = v80;
          v119 = v79;
          v121 = v77;
          v68 = v122;
        } while (v52);
        if (v84) {
          if (dat_1d028 != 0x80) {
            if (dat_1d028 == dat_1d864) {
label_6381:
              v56._0_2_ = (unsigned short)(unsigned char)dat_1d864;
              v42 = sub_10fd0(&v56);
              error(0,0,dcgettext(NULL,"field separator %s is treated as a group separator in numbers",5),v42);
              goto label_63c4;
            }
            v84 = 0;
            goto label_5aa7;
          }
          if (dat_1d864 != 0x80) { // branch-flip
            v23 = *v21;
            if (*(unsigned char *)(v23 + ((unsigned long)dat_1d864 & 0xff) * 2) & 1) goto label_6381;
            v84 = 0;
label_640e:
            if (*(unsigned char *)(v23 + (unsigned long)dat_1d868 * 2) & 1) goto label_641f;
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
              v23 = *v21;
              goto label_640e;
            }
            goto label_5b0c;
          }
label_5aa7:
          if (dat_1d028 != (int)(char)dat_1d868) { // branch-flip
            if (dat_1d028 != 0x2d) { // branch-flip
              if (dat_1d028 == 0x2b) {
                if (v100) {
                  v56 = CONCAT22(v56._2_2_,0x2b);
                  v42 = sub_10fd0(&v56);
                  v75 = "field separator %s is treated as a plus sign in numbers";
                  goto label_5af4;
                }
              }
            }
            else {
              v56 = CONCAT22(v56._2_2_,0x2d);
              v42 = sub_10fd0(&v56);
              v75 = "field separator %s is treated as a minus sign in numbers";
label_5af4:
              error(0,0,dcgettext(NULL,v75,5),v42);
            }
label_5b0c:
            if (!v83) {
              if (!v15) goto label_5b59;
            }
            if (!v84) goto label_5d6d;
          }
          else {
label_641f:
            v56 = CONCAT31((undefined3)(v56 >> 8),dat_1d868) & 0xffff00ff;
            v42 = sub_10fd0(&v56);
            error(0,0,dcgettext(NULL,"field separator %s is treated as a decimal point in numbers",5),v42);
          }
        }
        else {
          if (!v83 && !v15) goto label_5b59;
label_5d6d:
          v75 = "";
          v56 = CONCAT31((undefined3)(v56 >> 8),dat_1d868) & 0xffff00ff;
          v42 = sub_10fd0(&v56);
          if ((int)(char)dat_1d868 != dat_1d028)
            v75 = dcgettext(NULL,"note ",5);
          error(0,0,dcgettext(NULL,"%snumbers use %s as a decimal point in this locale",5),v75,v42);
        }
        if (v83) {
          if (dat_1d862)
            error(0,0,dcgettext(NULL,"the multi-byte number group separator in this locale is not supported",5));
        }
      }
label_5b59:
      v15 = sub_70e0(&v57);
      v80 = v121;
      if (v15) { // branch-flip
        v121 = v80;
        if (v121) {
          if (dat_1d41a) { // branch-flip
            if (dat_1d410) {
              sub_7140(&v57,&v56);
              v25 = strlen((char *)&v56);
              error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
              v121 = 1;
label_5cd4:
              if (!dat_1d41a) goto label_5ce1;
            }
          }
          else {
            if ((dat_1d419) && (dat_1d410)) {
              sub_7140(&v57,&v56);
              v25 = strlen((char *)&v56);
              error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
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
        sub_7140(&v57,&v56);
        v25 = strlen((char *)&v56);
        error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
        v121 = v80;
        if (v80) goto label_5cd4;
      }
    }
    dat_1d41b = v107[0xf];
    if (v50) {
      v23 = sub_11060(v98,0x10);
      if (!v23) {
        if (!v98)
          v98 = "getrandom";
        v75 = "open failed";
        sub_7480(dcgettext(NULL,v75,5),v98); // return-dupe, no-return
      }
      sub_11220(v23,&v56,0x10);
      if (sub_113c0(v23)) {
        v75 = "close failed";
        sub_7480(dcgettext(NULL,v75,5),v98);
      }
      MD5_Init(0x1d300);
      MD5_Update(0x1d300,&v56,0x10);
    }
  }
  else {
    if (!sub_70e0(&v55)) {
      sub_76e0();
      v85 = 1;
      v50 = v107[0xb];
      v52 = dat_1d410;
      goto label_58ff;
    }
    if (dat_1d400) {
      v85 = 0;
      v50 = 0;
      goto label_4775;
    }
    dat_1d41b = v107[0xf];
  }
  if (!dat_1d428) {
    v75 = getenv("TMPDIR");
    if (!v75)
      v75 = "/tmp";
    sub_7730(v75);
  }
  if (!v81) {
    v81 = 1;
    free(v20);
    v20 = (unsigned long *)sub_12ab0(8);
    *v20 = "-";
  }
  if (dat_1d438) {
    v31 = (void *)((unsigned long)dat_1d024 * 0x22);
    if ((void *)((unsigned long)dat_1d024 * 0x22) < dat_1d438)
      v31 = dat_1d438;
    dat_1d438 = v31;
  }
  if (v14) {
    if (v81 != 1) {
      v42 = sub_108e0(4,v20[1]);
      error(2,0,dcgettext(NULL,"extra operand %s not allowed with -%c",5),v42,(unsigned long)(unsigned int)(int)v14);
      return v47;
    }
    if (v96) {
      dat_1d010 = v14;
      sub_7310(0x1d010); // no-return
    }
    v42 = *v20;
    v23 = sub_74e0(v42,"r");
    v52 = dat_1d410;
    if (!v23)
      sub_7c60(v42); // no-return
    v51 = (unsigned int)dat_1d419;
    v31 = dat_1d030;
    if (dat_1d030 <= dat_1d438)
      v31 = dat_1d438;
    v25 = 0;
    sub_7790(&v57,0x20,v31);
    v54 = NULL;
    v92 = NULL;
    do {
      v15 = sub_7ef0(&v57,v23,v42);
      v81 = v110;
      if (!v15) {
        v51 = 1;
label_5e04:
        sub_7660(v23,v42);
        free(v57);
        free(v54);
        exit(v51 ^ 1); // no-return
      }
      v22 = (long *)(v111 + (long)v57);
      v26 = v110 * -4;
      if ((v25) && ((int)(v51 ^ 1) <= (int)sub_a0b0(&v54,&v22[-4]))) {
        v21 = v22;
label_5df7:
        v51 = 0;
        if (v14 == 'c') {
          v38 = sub_e200(((long)v22 - (long)&v21[-4] >> 5) + (long)v92,&v56);
          v29 = dat_1d888;
          __fprintf_chk(stderr,1,dcgettext(NULL,"%s: %s:%s: disorder: ",5),v29,v42,v38);
          sub_82d0(&v21[-4],stderr,dcgettext(NULL,"standard error",5));
        }
        goto label_5e04;
      }
      v37 = &v22[-4];
      while (v21 = v37, &v22[v26] < v21) {
        v37 = &v21[-4];
        if ((int)(v51 ^ 1) <= (int)sub_a0b0(v21,&v21[-4])) goto label_5df7;
      }
      v49 = v21[1];
      v92 = (long)((long)v92 + v81);
      if (v25 < v49) {
        do {
          v25 *= 2;
          if (!v25) {
            v25 = v49;
            break;
          }
        } while (v25 < v49);
        free(v54);
        v54 = (void *)sub_12ab0(v25);
        v49 = v21[1];
      }
      v31 = memcpy(v54,(void *)*v21,v49);
      v101 = v21[1];
      if (v52) {
        v102 = (long)v31 + (v21[2] - *v21);
        v103 = (long)v31 + (v21[3] - *v21);
      }
    } while( true );
  }
  v95 = 0;
  v30 = v20;
  do {
    v75 = (char *)*v30;
    if ((strcmp(v75,"-")) && (euidaccess(v75,4))) {
      v42 = *v30;
      v75 = "cannot read";
      sub_7480(dcgettext(NULL,v75,5),v42); // return-dupe, no-return
    }
    v95 += 1;
    v30 = &v30[1];
  } while (v81 != v95);
  if (v96) {
    v17 = open(v96,0x80041,0x1b6);
    if (v17 < 0)
      sub_7480(dcgettext(NULL,"open failed",5),v96);
    if (v17 != 1)
      sub_7c90(v17,1);
  }
  if (!v8) {
    if (!v99) {
      v25 = sub_e570(2);
      v99 = 8;
      if (v25 <= 8)
        v99 = v25;
    }
    v91 = 0;
    v111 = 0;
    v25 = 0xffffffffffffff;
    if (v99 <= 0xffffffffffffff)
      v25 = v99;
    v23 = v25 * 2;
label_5328:
    v42 = *v20;
    v31 = (void *)sub_74e0(v42,"r");
    if (!v31)
      sub_7c60(v42); // no-return
    v97 = 0x30;
    if (2 <= v99) {
      v97 = 1;
      v81 = 1;
      do {
        v81 *= 2;
        v97 += 1;
      } while (v81 < v25);
      v97 *= 0x20;
    }
    if (!v111) {
      v76 = (void *)(v97 + 2);
      v81 = 0;
      do {
        if (v81) { // branch-flip
          v75 = (char *)v20[v81];
          v85 = (strcmp(v75,"-")) ? stat(v75,&v53) != 0 : fstat(0,&v53) != 0; // branch-flip
        }
        else {
          v17 = fileno(v31);
          v85 = fstat(v17,&v53) != 0;
        }
        if (v85) {
          v42 = v20[v81];
          v75 = "stat failed";
          sub_7480(dcgettext(NULL,v75,5),v42);
        }
        if (((unsigned int)v126 & 0xf000) != 0x8000) { // branch-flip
          v35 = dat_1d438;
          if (dat_1d438) goto label_60e5;
          v93 = (char *)0x20000;
          v35 = dat_1d240;
          if (!dat_1d240) {
            v93 = (char *)0x20000;
label_5f41:
            v35 = (void *)0xffffffffffffffff;
            if (!getrlimit(2))
              v35 = v54;
            if ((!getrlimit(9)) && (v54 < v35))
              v35 = v54;
            v35 = (void *)((unsigned long)v35 >> 1);
            if (!getrlimit(5,&v54)) {
              v36 = (void *)(((unsigned long)v54 >> 4) * 0xf);
              if (v36 < v35)
                v35 = v36;
            }
            sub_e850();
            sub_e760();
            v88 = v87 * dat_17110;
            v90 = v86;
            if (v86 <= dat_17108 * v87)
              v90 = dat_17108 * v87;
            if (v88 < (double)(long)v35) {
              v35 = (dat_17100 <= v88) ? (void *)((long)(v88 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v88;
            }
            if (v90 < (double)v35) {
              v35 = (dat_17100 <= v90) ? (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v90;
            }
            dat_1d240 = (void *)((unsigned long)dat_1d024 * 0x22);
            if ((void *)((unsigned long)dat_1d024 * 0x22) < v35)
              dat_1d240 = v35;
            v35 = dat_1d240;
          }
        }
        else {
          v93 = (char *)v63;
          v35 = dat_1d240;
          if (!dat_1d240) {
            dat_1d240 = dat_1d438;
            v35 = dat_1d438;
            if (!dat_1d438) goto label_5f41;
          }
        }
        v49 = (long)v93 * (v97 + 1U) + 1;
        if ((v93 != (char *)(v49 / (v97 + 1U))) || ((unsigned long)((long)v35 - (long)v76) <= v49)) goto label_60e5;
        v76 = (void *)((long)v76 + v49);
        v81 += 1;
        v35 = v76;
        if (v81 >= v95) goto label_60e5;
      } while( true );
    }
    goto label_5385;
  }
  v32 = (unsigned long *)sub_12e90(v95,0x10);
  v30 = &v20[v95];
  v33 = v32;
  do {
    v42 = *v20;
    v20 = &v20[1];
    v34 = &v33[2];
    *v33 = v42;
    v33 = v34;
  } while (v30 != v20);
  sub_ac90(v32,0,v95,v96);
label_55b4:
  if ((dat_1d418) && (v17 = sub_c5b0(stdin), v17 == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  if (v140 != *(long *)(v82 + 0x28))
    __stack_chk_fail(); // no-return
  return 0;
}


// Function: sub_6f40 @ 0x6f40
unsigned long sub_6f40(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}


// Function: sub_6f60 @ 0x6f60
unsigned int sub_6f60(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}


// Function: sub_6f70 @ 0x6f70
int sub_6f70(long *a0)
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
unsigned char sub_70e0(long a0)
{
  if ((!*(long *)(a0 + 0x20)) && (!*(long *)(a0 + 0x28))) {
    if (*(char *)(a0 + 0x30))
      return 0;
    if ((!*(char *)(a0 + 0x31)) && (!(*(unsigned long *)(a0 + 0x30) & 0xffff00ff0000))) {
      if (*(char *)(a0 + 0x36))
        return 0;
      if (!*(char *)(a0 + 0x38))
        return *(unsigned char *)(a0 + 0x33) ^ 1;
    }
  }
  return 0;
}


// Function: sub_7140 @ 0x7140
void sub_7140(long a0,char *a1)
{
  if (*(short *)(a0 + 0x30)) {
    *a1 = 0x62;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1d540) {
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
  if (*(long *)(a0 + 0x20) == 0x1d640) {
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


// Function: sub_71f0 @ 0x71f0
char * sub_71f0(char *a0,long a1,unsigned int a2)
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
          goto label_724d;
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
          *(unsigned long *)(a1 + 0x20) = 0x1d540;
          break;
        case 0x66:
          *(unsigned long *)(a1 + 0x28) = 0x1d440;
          break;
        case 0x67:
          *(char *)(a1 + 0x34) = 1;
          break;
        case 0x68:
          *(char *)(a1 + 0x35) = 1;
          break;
        case 0x69:
          if (!*(long *)(a1 + 0x20))
            *(unsigned long *)(a1 + 0x20) = 0x1d640;
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
label_724d:
  return a0;
}


// Function: sub_7300 @ 0x7300
void sub_7300(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}


// Function: sub_7310 @ 0x7310
void sub_7310(unsigned long a0)
{
  error(2,0,dcgettext(NULL,"options \'-%s\' are incompatible",5),a0);
}


// Function: sub_7340 @ 0x7340
void sub_7340(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_10fd0(a0);
  v2 = dcgettext(NULL,a1,5);
  error(2,0,dcgettext(NULL,"%s: invalid field specification %s",5),v2,v1);
}


// Function: sub_7390 @ 0x7390
char * sub_7390(unsigned long a0,unsigned long *a1,char *a2)
{
  unsigned long v1;
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
      v3 = (unsigned long)sub_10fd0(a0);
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
void sub_7480(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = dcgettext(NULL,"standard output",5);
  v1 = sub_10c60(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1);
}


// Function: sub_74e0 @ 0x74e0
int * sub_74e0(char *a0,char *a1)
{
  int v1;
  int *v2;
  unsigned long v3;
  unsigned int v4;
  int *v5; // rax
  void *v6;
  
  if (*a1 == 'r') {
    v4 = strcmp(a0,"-");
    if (v4) { // branch-flip
      v6 = NULL;
      v4 = open(a0,0x80000);
      if (0 <= (int)v4) {
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
  v5 = __errno_location();
  v2 = v5;
  v1 = *v5;
  v4 = dat_1d2f0;
  if (dat_1d2f0) { // branch-flip
label_7579:
    if (0 <= (int)v4) goto label_7593;
  }
  else {
    v4 = fstat(1,(void *)0x1d260);
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
  v5 = (unsigned long)sub_10c60(0,3,a0);
  v3 = v5;
  v5 = (char *)dcgettext(NULL,"%s: error truncating",5);
  error(2,v1,(char *)v5,v3);
  return v5;
}


// Function: sub_7660 @ 0x7660
void sub_7660(void *a0,unsigned long a1) // return-dupe
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
void sub_76e0(unsigned long a0)
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
void sub_7730(unsigned long a0)
{
  long v1;
  
  v1 = dat_1d430;
  if (dat_1d428 == dat_1d420) {
    v1 = sub_12d00(dat_1d430,0x1d420,8);
    dat_1d430 = v1;
  }
  *(unsigned long *)(v1 + dat_1d428 * 8) = a0;
  dat_1d428 += 1;
}


// Function: sub_7800 @ 0x7800
void sub_7800(unsigned long a0,unsigned long a1,unsigned long a2)
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
  v3 = sub_107b0(0,8,a1);
  error(2,0,dcgettext(NULL,"the untransformed string was %s",5),v3);
}


// Function: sub_78c0 @ 0x78c0
unsigned long sub_78c0(int a0)
{
  unsigned int v1; // eax
  unsigned int v10; // stack - 0x30
  long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned int v7; // stack - 0x3c
  char v8 [8];
  int v9;
  
  v9 = -1;
  if (a0)
    v9 = a0;
  v1 = waitpid(v9,(int *)&v7,(unsigned int)(a0 == 0));
  if ((int)v1 < 0) {
    v5 = sub_108e0(4,dat_1d408);
    v3 = dcgettext(NULL,"waiting for %s [-d]",5);
    error(2,*__errno_location(),v3,v5);
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
      v5 = sub_108e0(4,dat_1d408);
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
void sub_7a90(long a0)
{
  long v1;
  
  v1 = dat_1d360;
  if (!dat_1d360) {
    v1 = sub_d640(0x2f,0,sub_6f40,sub_6f60,0);
    dat_1d360 = v1;
    if (!v1) {
      sub_12fb0(); // no-return, return-dupe
    }
  }
  *(char *)(a0 + 0xc) = 1;
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
    pthread_mutex_init(&v5[0xb],0);
    if (a3 <= 1) break;
    v5[8] = (long)v1;
    v3 = a3 >> 1;
    a3 -= v3;
    a1 = (long *)sub_7af0(v5,v1,v7,v3,a4,1);
    v5[9] = (long)a1;
    a5 = '\0';
    a0 = v5;
  }
  v5[8] = 0;
  v5[9] = 0;
  return v1;
}


// Function: sub_7ef0 @ 0x7ef0
unsigned long sub_7ef0(long *a0,unsigned int *a1,unsigned long a2)
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
  v1 = a0[5];
  if ((char)a0[6])
    return 0;
  v4 = a0[1];
  v10 = a0[4];
  if (v4 != v10) { // branch-flip
    memmove((void *)*a0,(void *)(*a0 + (v4 - v10)),v10);
    v4 = a0[4];
    a0[2] = 0;
    v7 = 0;
    a0[1] = v4;
  }
  else {
    v7 = a0[2];
  }
  v10 = v14 - 0x22;
  v11 = (unsigned char *)*a0;
  v14 = a0[3];
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
        if (*a1 & 0x20)
          sub_7480(dcgettext(NULL,"read failed",5),a2); // no-return
        if (*a1 & 0x10) {
          *(char *)&a0[6] = 1;
          if ((unsigned char *)*a0 == v16)
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
    } while (!(char)a0[6]);
    v14 = *a0;
    a0[1] = (long)v16 - v14;
    v7 = (v14 + a0[3]) - (long)v13 >> 5;
    a0[2] = v7;
    if (v7) {
      a0[4] = (long)v16 - (long)v11;
      dat_1d030 = v10 + 0x22;
      return 1;
    }
    v9 = (unsigned long)a0[3] >> 5;
    v11 = (unsigned char *)sub_12d00(v14,&v9,0x20);
    v14 = v9 << 5;
    *a0 = (long)v11;
    v4 = a0[1];
    a0[3] = v14;
    v7 = a0[2];
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
void sub_82d0(unsigned long *a0,void *a1,long a2)
{
  char v1;
  long *v10;
  unsigned char *v11;
  long v12; // r14
  unsigned long v13;
  long v14;
  char *v2;
  unsigned char v3;
  unsigned char *v4;
  unsigned char *v5;
  unsigned int v6; // ecx
  unsigned char *v7;
  unsigned char *v8; // stack - 0x48
  unsigned char *v9; // stack - 0x50
  
  v7 = (unsigned char *)*a0;
  v13 = a0[1];
  v5 = &v7[v13];
  if ((a2) || (!dat_1d400)) {
    v5[-1] = dat_1d038;
    if (v13 == fwrite_unlocked(v7,1,v13,a1)) {
      v5[-1] = 0;
      return;
    }
    sub_7480(dcgettext(NULL,"write failed",5),a2); // no-return
  }
  if (v7 < v5) {
    do {
      v3 = *v7;
      v7 = &v7[1];
      if (v3 != 9) { // branch-flip
        v6 = (unsigned int)(char)v3;
        if (v5 == v7) {
          v6 = 10;
          v3 = 10;
        }
        v4 = *(unsigned char **)((long)a1 + 0x28);
        if (*(unsigned char **)((long)a1 + 0x30) <= v4) goto label_83be;
label_838c:
        *(unsigned char **)((long)a1 + 0x28) = &v4[1];
        *v4 = v3;
      }
      else {
        v4 = *(unsigned char **)((long)a1 + 0x28);
        v6 = 0x3e;
        v3 = 0x3e;
        if (v4 < *(unsigned char **)((long)a1 + 0x30)) goto label_838c;
label_83be:
        if (__overflow(a1,v6 & 0xff) == -1)
          sub_7480(dcgettext(NULL,"write failed",5),0); // no-return
      }
    } while (v5 != v7);
    v7 = (unsigned char *)*a0;
    v13 = a0[1];
  }
  v5 = &v7[v13 - 1];
  v10 = dat_1d410;
  v4 = v7;
  if (!dat_1d410) goto label_8642;
  do {
    v14 = *v10;
    if (v14 != -1) { // branch-flip
      v4 = (unsigned char *)sub_7dd0(v7,v13,v10);
      if (v10[2] != -1) {
label_844e:
        v5 = (unsigned char *)sub_7ca0(v7,v13,v10);
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
      *v5 = 0;
      v1 = *(char *)((unsigned long)*v4 + 0x1d740);
      while (v1) {
        v11 = &v4[1];
        v4 = &v4[1];
        v1 = *(char *)((unsigned long)*v11 + 0x1d740);
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
            v8 = &v4[*v4 == 0x2d];
          if ('0' <= (char)sub_6f70(&v8)) {
            v9 = v8;
            if (*(char *)((long)v10 + 0x35))
              v9 = &v8[*(char *)((unsigned long)*v8 + 0x15360) != '\0'];
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
            v14 += (unsigned long)(*v7 == 9);
            v7 = v11;
          } while (v4 != v11);
        }
        v12 = (long)(int)sub_e260(v4,(long)v5 - (long)v4,0);
        if (v4 < v5) {
          do {
            v7 = &v4[1];
            v12 += (unsigned long)(*v4 == 9);
            v4 = v7;
          } while (v5 != v7);
        }
        while (v14) {
          v14 -= 1;
          v2 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
            __overflow(stdout,0x20);
          else {
            *(char **)((long)stdout + 0x28) = &v2[1];
            *v2 = 0x20;
          }
        }
        if (v12) { // branch-flip
          do {
            v2 = *(char **)((long)stdout + 0x28);
            if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
              __overflow(stdout,0x5f);
            else {
              *(char **)((long)stdout + 0x28) = &v2[1];
              *v2 = 0x5f;
            }
            v12 -= 1;
          } while (v12);
          v2 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
            __overflow(stdout,10);
          else {
            *(char **)((long)stdout + 0x28) = &v2[1];
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
        v4 = (unsigned char *)*a0;
        v5 = &v4[a0[1] + -1];
        v7 = v4;
      }
      v7 = (unsigned char *)*a0;
      v13 = a0[1];
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
      v2 = pthread_sigmask(0,0x1d380,v6);
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
        pthread_sigmask(2,v6,0);
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
long * sub_8980(unsigned long *a0,long a1,unsigned long *a2)
{
  unsigned int v1;
  long *v10;
  long v11; // r12
  int v12; // stack - 0x54
  unsigned int v13; // stack - 0x50
  int v2;
  long v3;
  long *v4;
  int *v5;
  unsigned long v6;
  int v7; // eax
  long *v8; // rax
  int v9; // stack - 0x58
  
  v8 = (long *)sub_12bf0(a1,8);
  v4 = v8;
  *a2 = v8;
  if (a1) { // branch-flip
    v11 = 0;
    v10 = v8;
    do {
      v3 = a0[1];
      if ((v3) && (*(char *)(v3 + 0xc))) {
        if (*(char *)(v3 + 0xc) == '\x01') {
          v1 = *(unsigned int *)(v3 + 8);
          v13 = v1;
          v8 = (long)sub_dd00(dat_1d360,&v9);
          if (v8) {
            *(char *)((long)v8 + 0xc) = 2;
            sub_78c0(v1);
          }
        }
        v7 = open((char *)(v3 + 0xd),0);
        v2 = v7;
        if (v7 < 0) {
          v4[v11] = 0;
          v8 = (long *)v11;
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
            v8 = (long *)v11;
            return v8;
          }
          v8 = (unsigned long)sub_108e0(4,dat_1d408);
          v6 = v8;
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
        *(int *)(v3 + 8) = v7;
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
          v8 = (long *)v11;
          return v8;
        }
        *v10 = (long)v8;
      }
      else {
        v8 = (long)sub_74e0(*a0,"r");
        *v10 = (long)v8;
        if (!v8) {
          v8 = (long *)v11;
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
  v8 = (long *)v11;
  return v8;
}


// Function: sub_8c20 @ 0x8c20
int * sub_8c20(unsigned long *a0,char a1)
{
  void *v1;
  char v10 [128];
  int v11; // stack - 0xc8
  int v12;
  unsigned long *v13;
  int v14; // stack - 0xc4
  int v2;
  char *v3;
  unsigned long v4;
  unsigned long *v5;
  int *v6;
  unsigned long v7;
  int v8; // eax
  int *v9; // rax
  
  v3 = *(char **)(dat_1d430 + dat_1d248 * 8);
  v9 = (unsigned long)strlen(v3);
  v4 = (unsigned long)v9;
  v9 = (unsigned long *)sub_12ab0((long)v9 + 0x20U & 0xfffffffffffffff8);
  v5 = (unsigned long *)v9;
  v1 = (void *)((long)v9 + 0xd);
  memcpy(v1,v3,v4);
  *(unsigned long *)((long)v9 + v4 + 0xd) = s_152f0._0_8_;
  *(unsigned int *)((long)v9 + v4 + 0x15) = s_152f0._8_4_;
  *(unsigned long *)v9 = 0;
  v9 = (long)(dat_1d248 + 1);
  dat_1d248 = (long)v9;
  if (v9 == (int *)dat_1d428)
    dat_1d248 = 0;
  v8 = pthread_sigmask(0,0x1d380,v10);
  v11 = CONCAT31(v11._1_3_,v8 == 0);
  v8 = sub_11d70(v1,0x80000);
  v12 = v8;
  v9 = __errno_location();
  v6 = v9;
  if (0 <= v12) { // branch-flip
    *dat_1d040 = v5;
    dat_1d040 = v5;
    if ((char)v11) {
      v2 = *v9;
      pthread_sigmask(2,v10,0);
      *v9 = v2;
    }
    *(char *)((long)v5 + 0xc) = 0;
    if (dat_1d408) {
      v8 = sub_8770(&v11,4);
      *(int *)&v5[1] = v8;
      if (1 <= v8) { // branch-flip
        close(v12);
        close(v11);
        sub_7a90(v5);
        v12 = v14;
      }
      else if (!v8) {
        close(v14);
        if (v12 != 1)
          sub_7c90(v12,1);
        if (v11)
          sub_7c90(v11,0);
        execlp(dat_1d408,dat_1d408,0);
        sub_79f0(*v6,"couldn\'t execute compress program"); // no-return
      }
    }
    v9 = fdopen(v12,"w");
    *a0 = v9;
    v13 = v5;
    if (!v9) {
      v9 = (char *)dcgettext(NULL,"couldn\'t create temporary file",5);
      sub_7480(v9,v1); // no-return
    }
  }
  else {
    v12 = *v9;
    if ((char)v11) {
      pthread_sigmask(2,v10,0);
      *v9 = v12;
    }
    if ((v12 != 0x18) || (a1 != '\x01')) {
      v9 = (unsigned long)sub_108e0(4,v3);
      v7 = v9;
      v9 = (char *)dcgettext(NULL,"cannot create temporary file in %s",5);
      error(2,*v6,(char *)v9,v7);
      return v9;
    }
    v13 = NULL;
    free(v5);
  }
  v9 = (int *)v13;
  return v9;
}


// Function: sub_8ed0 @ 0x8ed0
void sub_8ed0(void)
{
  unsigned long *v1;
  int v2; // eax
  char v3 [128];
  
  if (dat_1d368) {
    v2 = pthread_sigmask(0,0x1d380,v3);
    for (v1 = dat_1d368; v1; v1 = (unsigned long *)*v1) {
      unlink((char *)((long)v1 + 0xd));
    }
    dat_1d368 = NULL;
    if (!v2)
      pthread_sigmask(2,v3,0);
  }
  sub_c4c0(); // tail-call
}


// Function: sub_8f90 @ 0x8f90
void sub_8f90(char *a0)
{
  int v1;
  char v10 [8]; // stack - 0xc8
  long v2;
  long *v3;
  long *v4;
  int v5; // eax
  int *v6; // rax
  unsigned long v7; // rax
  long v8; // rax
  unsigned int v9 [32]; // stack - 0xc0
  
  v3 = (long *)0x1d368;
  for (v4 = dat_1d368; a0 != (char *)((long)v4 + 0xd); v4 = (long *)*v4) {
    v3 = v4;
  }
  if (*(char *)((long)v4 + 0xc) == '\x01') {
    v2 = v4[1];
    v9[0] = (int)v2;
    v8 = sub_dd00(dat_1d360,v10);
    if (v8) {
      *(char *)(v8 + 0xc) = 2;
      sub_78c0((int)v2);
    }
  }
  v2 = *v4;
  v10[0] = pthread_sigmask(0,0x1d380,v9) == 0;
  v5 = unlink(a0);
  v6 = __errno_location();
  v1 = *v6;
  *v3 = v2;
  if (v10[0])
    pthread_sigmask(2,v9,0);
  if (v5) {
    v7 = sub_10c60(0,3,a0);
    error(0,v1,dcgettext(NULL,"warning: cannot remove: %s",5),v7);
  }
  if (!v2)
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
          v7 = sub_7030(v9,0);
          v7 -= sub_7030(v10,0);
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
                strxfrm(v11,v32,v13);
                if (v26 < v1) goto label_9e3c;
                v50 = &v11[v13];
                v49 = v11;
                v32 = &v32[strlen((char *)v32) + 1];
              }
              if (v1 <= v26) {
                v50 = &v11[v13];
                v49 = v11;
                goto label_9c1a;
              }
label_9e3c:
              v50 = &v11[v13];
              strxfrm(v50,v26,v51);
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
    v27 = (unsigned char *)sub_7ca0(*a1,a1[1],v25);
  }
  else {
    v9 = &v26[v4 + -1];
    v27 = (unsigned char *)(*a1 + -1 + a1[1]);
  }
  if (*v25 != -1) { // branch-flip
    v26 = (unsigned char *)sub_7dd0(v26,v4,v25);
    v10 = (unsigned char *)sub_7dd0(*a1,a1[1],v25);
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
void sub_a180(unsigned long *a0,unsigned long a1,unsigned long *a2,char a3) // return-dupe x2
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
  v9 = sub_a0b0(&a0[-4],&a0[-8]);
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
void sub_a3c0(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_1d419) {
    if (dat_1d840) {
      if (!sub_a0b0(a0,0x1d840))
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
void sub_a430(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  unsigned long v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long v12;
  unsigned long v13;
  unsigned long v14;
  long v15;
  int v16; // eax
  void *v17; // rax
  unsigned long *v18; // rax
  void *v19;
  long *v2;
  unsigned long *v20;
  long v21;
  unsigned long *v22;
  unsigned long *v23; // rdx
  unsigned long *v24;
  void *v25; // stack - 0x68
  unsigned long v26;
  unsigned long v27; // r14
  unsigned long v28;
  unsigned long v29;
  unsigned long *v3;
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
  long v6;
  long v7;
  unsigned long v8;
  unsigned long v9;
  
  v34 = (void *)sub_12bf0(a2,0x38);
  v17 = (void *)sub_12bf0(a2,8);
  v33 = (void *)sub_12bf0(a2,8);
  v18 = (unsigned long *)sub_12bf0(a2,8);
  v15 = dat_1d410;
  v25 = NULL;
  v35 = (void **)v18;
  v31 = v17;
  if (a2) {
    v28 = 0;
    v32 = a2;
    v36 = a1;
    do {
      while( true ) {
        v26 = dat_1d438 / v32;
        if (dat_1d438 / v32 < dat_1d030)
          v26 = dat_1d030;
        v2 = (long *)((long)v34 + v28 * 0x38);
        while( true ) {
          v1 = (v26 & 0xffffffffffffffe0) + 0x20;
          v19 = malloc(v1);
          *v2 = (long)v19;
          if (v19) break;
          v26 = v1 >> 1;
          if (v1 <= 0x42)
            sub_12fb0(); // no-return
        }
        v2[3] = v1;
        v21 = v28 * 8;
        v2[5] = 0x20;
        v24 = (unsigned long *)(v28 * 0x10 + a0);
        v4 = *(unsigned long *)((long)a5 + v21);
        v2[2] = 0;
        v2[4] = 0;
        v5 = *v24;
        v2[1] = 0;
        *(char *)&v2[6] = 0;
        if (sub_7ef0(v2,v4,v5)) break;
        sub_7660(*(unsigned long *)((long)a5 + v21),*v24);
        if (v28 < v36) {
          v36 -= 1;
          sub_8f90(*v24);
        }
        free((void *)*v2);
        v26 = v32 - 1;
        if (v26 <= v28) {
          if (!v26) goto label_aa9e;
          v32 = v26;
          goto label_a5e2;
        }
        do {
          v24 = (unsigned long *)(a0 + 0x10 + v21 * 2);
          v5 = v24[1];
          v4 = *(unsigned long *)((long)a5 + v21 + 8);
          v3 = (unsigned long *)(a0 + v21 * 2);
          *v3 = *v24;
          v3[1] = v5;
          *(unsigned long *)((long)a5 + v21) = v4;
          v21 += 8;
        } while (v32 * 8 + -8 != v21);
        v32 = v26;
      }
      v21 = v2[3];
      v6 = *v2;
      *(long *)((long)v17 + v28 * 8) = v21 + v6 + -0x20;
      *(long *)((long)v33 + v28 * 8) = v21 + v6 + v2[2] * -0x20;
      v28 += 1;
    } while (v28 < v32);
label_a5e2:
    v28 = 0;
    do {
      v18[v28] = v28;
      v28 += 1;
    } while (v28 != v32);
    if (v32 != 1) {
      v28 = 1;
      do {
        while( true ) {
          v20 = &v18[v28];
          v22 = &v18[v28 - 1];
          v4 = *(unsigned long *)((long)v17 + *v20 * 8);
          v5 = *(unsigned long *)((long)v17 + *v22 * 8);
          if ((int)sub_a0b0(v5,v4) <= 0) break;
          v26 = *v22;
          v28 = 1;
          *v22 = *v20;
          *v20 = v26;
        }
        v28 += 1;
      } while (v28 < v32);
    }
    v37 = 0;
    v35 = NULL;
label_a680:
    v2 = *(long **)((long)v17 + *v18 * 8);
    if (dat_1d419) { // branch-flip
      if (v35) {
        if (!sub_a0b0(v35,v2)) goto label_a6b6;
        sub_82d0(&v25,a3,a4);
      }
      v28 = v2[1];
      v19 = v25;
      if (v37 < v28) {
        do {
          v26 = v28;
          if (!v37) goto label_ab1d;
          v37 *= 2;
        } while (v37 < v28);
        v26 = v37;
label_ab1d:
        free(v25);
        v19 = (void *)sub_12ab0(v26);
        v28 = v2[1];
        v25 = v19;
        v37 = v26;
      }
      v38 = v28;
      v19 = memcpy(v19,(void *)*v2,v28);
      v35 = &v25;
      if (v15) {
        v39 = (v2[2] - *v2) + (long)v19;
        v40 = (long)v19 + (v2[3] - *v2);
      }
    }
    else {
      sub_82d0(v2,a3,a4);
    }
label_a6b6:
    v28 = *v18;
    if (*(long **)((long)v33 + v28 * 8) < v2) {
      *(long **)((long)v17 + v28 * 8) = &v2[-4];
      v31 = (void *)v28;
      if (v32 != 1) goto label_a761;
label_a81c:
      *v18 = v28;
      goto label_a680;
    }
    v4 = *(unsigned long *)((long)a5 + v28 * 8);
    v5 = *(unsigned long *)(a0 + v28 * 0x10);
    if (sub_7ef0((void *)((long)v34 + v28 * 0x38),v4,v5)) {
      v28 = *v18;
      v2 = (long *)((long)v34 + v28 * 0x38);
      v21 = v2[3];
      v6 = *v2;
      v7 = v2[2];
      *(long *)((long)v17 + v28 * 8) = v21 + v6 + -0x20;
      *(long *)((long)v33 + v28 * 8) = v21 + v6 + v7 * -0x20;
      v31 = (void *)v28;
      if (v32 == 1) goto label_a81c;
label_a761:
      v26 = 1;
      v1 = 1;
      v29 = v32;
      do {
        while( true ) {
          v27 = v1;
          v16 = sub_a0b0(*(unsigned long *)((long)v17 + v28 * 8),*(unsigned long *)((long)v17 + v18[v27] * 8));
          if ((0 <= v16) && ((v16 || (v18[v27] <= v31)))) break;
          v1 = v26 + v27 >> 1;
          v29 = v27;
          if (v27 <= v26) goto label_a7c9;
        }
        v26 = v27 + 1;
        v1 = v26 + v29 >> 1;
      } while (v26 < v29);
label_a7c9:
      if (v26 - 1) {
        v22 = v18;
        do {
          v20 = &v22[1];
          *v22 = v22[1];
          v22 = v20;
        } while (v20 != &v18[v26 - 1]);
      }
      v18[v26 - 1] = (unsigned long)v31;
      goto label_a680;
    }
    v28 = *v18;
    v22 = &v18[1];
    if (v32 != 1) {
      do {
        if (v28 < *v22) {
          *v22 = *v22 - 1;
          v28 = *v18;
        }
        v22 = &v22[1];
      } while (&v18[v32] != v22);
    }
    v26 = v32 - 1;
    sub_7660(*(unsigned long *)((long)a5 + v28 * 8),*(unsigned long *)(a0 + v28 * 0x10));
    v28 = *v18;
    if (v28 < v36) {
      v36 -= 1;
      sub_8f90(*(unsigned long *)(a0 + v28 * 0x10));
      v28 = *v18;
    }
    free(*(void **)((long)v34 + v28 * 0x38));
    v28 = *v18;
    if (v28 < v26) {
      v21 = v28 * 8;
      v24 = (unsigned long *)((long)v34 + v28 * 0x38);
      do {
        v3 = (unsigned long *)(a0 + 0x10 + v21 * 2);
        v5 = *v3;
        v8 = v3[1];
        v23 = &v24[7];
        v9 = *v23;
        v10 = v24[8];
        v11 = v24[9];
        v12 = v24[10];
        *(unsigned long *)((long)a5 + v21) = *(unsigned long *)((long)a5 + v21 + 8);
        v4 = v24[0xd];
        v13 = v24[0xb];
        v14 = v24[0xc];
        v3 = (unsigned long *)(a0 + v21 * 2);
        *v3 = v5;
        v3[1] = v8;
        v24[6] = v4;
        v4 = *(unsigned long *)((long)v17 + v21 + 8);
        *v24 = v9;
        v24[1] = v10;
        *(unsigned long *)((long)v17 + v21) = v4;
        v4 = *(unsigned long *)((long)v33 + v21 + 8);
        v24[2] = v11;
        v24[3] = v12;
        *(unsigned long *)((long)v33 + v21) = v4;
        v21 += 8;
        v24[4] = v13;
        v24[5] = v14;
        v24 = v23;
      } while (v32 * 8 + -8 != v21);
    }
    if (v26) {
      v22 = v18;
      do {
        v20 = &v22[1];
        *v22 = v22[1];
        v22 = v20;
      } while (&v18[v32 - 1] != v20);
      v32 = v26;
      goto label_a680;
    }
    v30 = v35 != NULL;
    v35 = (void **)v18;
    v31 = v17;
    if (v30) {
      if (dat_1d419) {
        sub_82d0(&v25,a3,a4);
        free(v25);
      }
    }
  }
label_aa9e:
  sub_7660(a3,a4);
  free(a5);
  free(v34);
  free(v35);
  free(v33);
  free(v31); // tail-call
  return;
}


// Function: sub_abe0 @ 0xabe0
unsigned long sub_abe0(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x48
  
  v2 = sub_8980(a0,a2,&v3);
  if ((v2 < a2) && (v2 < 2)) {
    v1 = *(unsigned long *)(a0 + v2 * 0x10);
    sub_7480(dcgettext(NULL,"open failed",5),v1); // no-return
  }
  sub_a430(a0,a1,v2,a3,a4,v3);
  return v2;
}


// Function: sub_ac90 @ 0xac90
void sub_ac90(long *a0,unsigned long a1,unsigned long a2,char *a3)
{
  char *v1;
  long *v10;
  unsigned long v11; // stack - 0xe0
  long v12; // stack - 0xd8
  long v13; // stack - 0xe8
  unsigned long v14;
  unsigned long v15; // r12
  long *v16;
  unsigned long v17; // stack - 0x110
  unsigned long v18; // stack - 0x108
  long v19; // stack - 0xd0
  int v2; // eax
  int v3;
  long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7; // rdx
  unsigned long v8;
  long v9;
  
  v6 = (unsigned long)dat_1d024;
  v18 = a2;
  if (v6 < a2) {
    do {
      v5 = 0;
      v9 = 0;
      if (v6 <= v18) { // branch-flip
        do {
          v15 = v5;
          v4 = sub_8c20(&v11,0);
          v5 = (unsigned long)dat_1d024;
          v6 = a1;
          if (v5 <= a1)
            v6 = v5;
          v5 = sub_abe0(&a0[v9 * 2],v6,v5,v11,v4 + 0xd);
          v8 = (unsigned long)dat_1d024;
          v6 = a1;
          if (v5 <= a1)
            v6 = v5;
          v9 += v5;
          v5 = v15 + 1;
          a0[v15 * 2] = v4 + 0xd;
          a0[v15 * 2 + 1] = v4;
          a1 -= v6;
        } while (v8 <= v18 - v9);
        v7 = v5 % v8;
        v6 = v8 - v7;
        v14 = v18 - v9;
        v10 = &a0[v15 * 2 + 2];
        v16 = &a0[v9 * 2];
      }
      else {
        v7 = 0;
        v8 = v6;
        v14 = v18;
        v10 = a0;
        v16 = a0;
      }
      if (v6 < v14) {
        v8 = (v14 + 1 + v7) - v8;
        v4 = sub_8c20(&v11,0);
        v6 = a1;
        if (v8 <= a1)
          v6 = v8;
        v8 = sub_abe0(v16,v6,v8,v11,v4 + 0xd);
        v6 = a1;
        if (v8 <= a1)
          v6 = v8;
        v5 += 1;
        v9 += v8;
        *v10 = v4 + 0xd;
        v10[1] = v4;
        a1 -= v6;
        v16 = &a0[v9 * 2];
        v10 = &a0[v5 * 2];
      }
      a1 += v5;
      memmove(v10,v16,(v18 - v9) * 0x10);
      v6 = (unsigned long)dat_1d024;
      v18 += v5 - v9;
    } while (v6 < v18);
  }
  if (a1 < v18) {
    v9 = 0;
    v10 = &a0[a1 * 2];
    v6 = a1;
    do {
      while( true ) {
        v1 = (char *)*v10;
        v2 = strcmp(v1,"-");
        if (((!a3) || (strcmp(a3,v1))) || (!v2)) break;
label_ae9e:
        v4 = v9 + 0xd;
        if (!v9) {
          v9 = sub_8c20(&v11,0);
          v4 = v9 + 0xd;
          sub_abe0(v10,0,1,v11,v4);
        }
        *v10 = v4;
        v6 += 1;
        v16 = &v10[2];
        v10[1] = v9;
        v10 = v16;
        if (v6 == v18) goto label_aecf;
      }
      v3 = dat_1d2f0;
      if (dat_1d2f0) { // branch-flip
label_ae64:
        if (0 <= v3) break;
      }
      else {
        if (fstat(1,(void *)0x1d260)) {
          v3 = *__errno_location();
          dat_1d2f0 = v3;
          goto label_ae64;
        }
        dat_1d2f0 = -1;
      }
      if (v2) { // branch-flip
        v1 = (char *)*v10;
        if ((!stat(v1,&v12)) && (v19 == dat_1d268)) {
label_ae8c:
          if (v12 == dat_1d260) goto label_ae9e;
        }
      }
      else if ((!fstat(0,&v12)) && (v19 == dat_1d268)) goto label_ae8c;
      v6 += 1;
      v10 = &v10[2];
    } while (v6 != v18);
  }
label_aecf:
  v17 = a1;
  do {
    v6 = sub_8980(a0,v18,&v13);
    if (v18 != v6) { // branch-flip
      if (v6 <= 2) {
        v9 = a0[v6 * 2];
        sub_7480(dcgettext(NULL,"open failed",5),v9); // no-return
      }
    }
    else {
      v9 = sub_74e0(a3,"w");
      if (v9) {
        sub_a430(a0,v17,v18,v9,a3,v13);
        return;
      }
      if ((*__errno_location() != 0x18) || (v18 <= 2))
        sub_7480(dcgettext(NULL,"open failed",5),a3); // no-return
    }
    v6 -= 1;
    v10 = &a0[v6 * 2];
    while( true ) {
      sub_7660(*(unsigned long *)(v13 + v6 * 8),*v10);
      v16 = &v10[-2];
      v9 = sub_8c20(&v11,3 <= v6);
      if (v9) break;
      v6 -= 1;
      v10 = v16;
    }
    v5 = v17;
    if (v6 < v17)
      v5 = v6;
    v4 = v18 - v6;
    sub_a430(a0,v5,v6,v11,v9 + 0xd,v13);
    v18 = v4 + 1;
    *a0 = v9 + 0xd;
    a0[1] = v9;
    memmove(&a0[2],v10,v4 * 0x10);
    v17 = (v17 - v5) + 1;
  } while( true );
}


// Function: sub_b220 @ 0xb220
void sub_b220(long a0,unsigned long a1,unsigned long a2,long *a3,unsigned long *a4,unsigned long a5,unsigned long a6)
{
  unsigned long *v1;
  long *v10; // rax
  unsigned long *v11;
  long v12;
  unsigned long *v13;
  long v14;
  unsigned long *v15;
  unsigned long *v16; // rax
  long v17; // stack - 0x78
  unsigned long v18; // stack - 0x80
  unsigned long *v19;
  unsigned long *v2;
  long v20;
  unsigned long *v21;
  char v22;
  long v23;
  long v24;
  unsigned long *v25;
  unsigned long v26;
  unsigned long v27;
  unsigned long v28; // r15
  long v29;
  long *v3;
  unsigned long v30; // stack - 0x70
  unsigned long v31; // stack - 0x68
  long v32; // stack - 0x60
  unsigned long *v33; // stack - 0x58
  unsigned long v34; // stack - 0x50
  unsigned long v35; // stack - 0x48
  int v4;
  unsigned long *v5;
  unsigned long *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  v9 = a6;
  v28 = a1 >> 1;
  v27 = a3[5];
  v26 = a3[6];
  v32 = a3[8];
  v35 = a6;
  v17 = a0;
  v30 = v28;
  v31 = a2;
  v33 = a4;
  v34 = a5;
  if ((0x20000 <= v27 + v26) && (2 <= a1)) {
    if (!pthread_create(&v18,0,sub_ba80,&v17)) {
      sub_b220(a0 + a3[5] * -0x20,a1 - v28,a2,a3[9],a4,a5,v9);
      pthread_join(v18,0);
      return;
    }
    v27 = a3[5];
    v26 = a3[6];
  }
  v23 = a0 + a2 * -0x20;
  v29 = a0 + v27 * -0x20;
  if (2 <= v26)
    sub_a180(v29,v26,v23 + (v27 >> 1) * -0x20,0);
  if (2 <= v27)
    sub_a180(a0,v27,v23,0);
  *a3 = a0;
  v1 = &a4[6];
  a3[1] = v29;
  a3[2] = v29;
  a3[3] = a0 + v27 * -0x20 + v26 * -0x20;
  v2 = &a4[1];
  pthread_mutex_lock(v2);
  sub_df40(*a4,a3);
  *(char *)((long)a3 + 0x54) = 1;
  pthread_cond_signal(v1);
  pthread_mutex_unlock(v2);
  do {
    pthread_mutex_lock(v2);
    while (v10 = (long *)sub_e010(*a4), !v10) {
      pthread_cond_wait(v1,v2);
    }
    pthread_mutex_unlock(v2);
    v3 = &v10[0xb];
    pthread_mutex_lock(v3);
    v4 = (int)v10[10];
    *(char *)((long)v10 + 0x54) = 0;
    if (!v4) {
      pthread_mutex_unlock(v3);
      pthread_mutex_lock(v2);
      sub_df40(*a4,v10);
      *(char *)((long)v10 + 0x54) = 1;
      pthread_cond_signal(v1);
      pthread_mutex_unlock(v2);
      return;
    }
    v5 = (unsigned long *)*v10;
    v6 = (unsigned long *)v10[1];
    v19 = (unsigned long *)v10[2];
    v23 = (a2 >> ((char)v4 * '\x02' + 2U & 0x3f)) + 1;
    v21 = v5;
    if (v4 != 1) { // branch-flip
      v13 = (unsigned long *)v10[4];
      v15 = (unsigned long *)*v13;
      v11 = v6;
      if (v5 != v19) { // branch-flip
        v12 = v23;
        v25 = v5;
        while ((unsigned long *)v10[3] != v11) {
          v23 = v12 + -1;
          if (!v12) {
            v21 = (unsigned long *)*v10;
            v22 = *(char *)((long)v10 + 0x54);
            v12 = -1;
            v13 = (unsigned long *)v10[4];
            goto label_b66d;
          }
          v16 = &v15[-4];
          if (1 <= (int)sub_a0b0(&v25[-4],&v11[-4])) { // branch-flip
            v29 = v10[1];
            v25 = (unsigned long *)*v10;
            v19 = (unsigned long *)v10[2];
            v7 = *(unsigned long *)(v29 + -0x20);
            v8 = *(unsigned long *)(v29 + -0x18);
            v11 = (unsigned long *)(v29 + -0x20);
            v10[1] = (long)v11;
            *v16 = v7;
            v15[-3] = v8;
            v7 = *(unsigned long *)(v29 + -8);
            v15[-2] = *(unsigned long *)(v29 + -0x10);
            v15[-1] = v7;
            if (v19 == v25) goto label_b4b8;
          }
          else {
            v29 = *v10;
            v19 = (unsigned long *)v10[2];
            v7 = *(unsigned long *)(v29 + -0x20);
            v8 = *(unsigned long *)(v29 + -0x18);
            v25 = (unsigned long *)(v29 + -0x20);
            *v10 = (long)v25;
            *v16 = v7;
            v15[-3] = v8;
            v7 = *(unsigned long *)(v29 + -8);
            v11 = (unsigned long *)v10[1];
            v15[-2] = *(unsigned long *)(v29 + -0x10);
            v15[-1] = v7;
            if (v19 == v25) {
label_b4b8:
              v22 = *(char *)((long)v10 + 0x54);
              v13 = (unsigned long *)v10[4];
              v24 = (long)v5 - (long)v25 >> 5;
              v29 = (long)v6 - (long)v11 >> 5;
              v21 = v25;
              v15 = v16;
              goto label_b4e8;
            }
          }
          v12 = v23;
          v15 = v16;
        }
        v21 = (unsigned long *)*v10;
        v22 = *(char *)((long)v10 + 0x54);
        v13 = (unsigned long *)v10[4];
label_b66d:
        v29 = (long)v6 - (long)v11 >> 5;
        v24 = (long)v5 - (long)v25 >> 5;
        v14 = v10[6];
        v20 = v10[5];
        if (v14 != v29) { // branch-flip
label_b4f5:
          if (((v24 == v20) && (v19 = (unsigned long *)v10[3], v19 != v11)) && (v23 = v12 + -1, v25 = v15, v12)) {
            do {
              v16 = &v11[-4];
              v7 = *v16;
              v8 = v11[-3];
              v15 = &v25[-4];
              v10[1] = (long)v16;
              *v15 = v7;
              v25[-3] = v8;
              v7 = v11[-1];
              v25[-2] = v11[-2];
              v25[-1] = v7;
              if (v16 == v19) {
                v29 = (long)v6 - (long)v16 >> 5;
                goto label_b4fe;
              }
              v23 -= 1;
              v25 = v15;
              v11 = v16;
            } while (v23 != -1);
            v29 = (long)v6 - (long)v16 >> 5;
          }
        }
        else if ((v19 != v21) && (v23 = v12 + -1, v11 = v15, v6 = v21, v12)) {
          do {
            v21 = &v6[-4];
            v7 = *v21;
            v8 = v6[-3];
            v15 = &v11[-4];
            *v10 = (long)v21;
            *v15 = v7;
            v11[-3] = v8;
            v7 = v6[-1];
            v11[-2] = v6[-2];
            v11[-1] = v7;
            if (v21 == v19) break;
            v23 -= 1;
            v11 = v15;
            v6 = v21;
          } while (v23 != -1);
        }
      }
      else {
        v22 = '\0';
        v29 = 0;
        v24 = 0;
label_b4e8:
        v14 = v10[6];
        v20 = v10[5];
        v12 = v23;
        if (v29 != v14) goto label_b4f5;
      }
label_b4fe:
      *v13 = v15;
    }
    else {
      v11 = v6;
      if (v5 != v19) { // branch-flip
        v24 = v23;
        while ((unsigned long *)v10[3] != v11) {
          v23 = v24 + -1;
          if (!v24) {
            v24 = -1;
            break;
          }
          if (1 <= (int)sub_a0b0(&v21[-4],&v11[-4])) { // branch-flip
            v29 = v10[1];
            v10[1] = v29 + -0x20;
            sub_a3c0(v29 + -0x20,a5,v9);
            v21 = (unsigned long *)*v10;
            v11 = (unsigned long *)v10[1];
            if (v21 == (unsigned long *)v10[2]) goto label_b5f5;
          }
          else {
            v29 = *v10;
            *v10 = v29 + -0x20;
            sub_a3c0(v29 + -0x20,a5,v9);
            v21 = (unsigned long *)*v10;
            v11 = (unsigned long *)v10[1];
            if (v21 == (unsigned long *)v10[2]) {
label_b5f5:
              v14 = (long)v5 - (long)v21 >> 5;
              v29 = (long)v6 - (long)v11 >> 5;
              goto label_b618;
            }
          }
          v24 = v23;
        }
        v29 = v10[6];
        if ((long)v6 - (long)v11 >> 5 != v29) { // branch-flip
          v20 = v10[5];
          v14 = (long)v5 - (long)v21 >> 5;
label_b8a8:
          if (((v20 != v14) || (v11 == (unsigned long *)v10[3])) || (!v24)) {
            v22 = *(char *)((long)v10 + 0x54);
            v29 = (long)v6 - v10[1] >> 5;
            v14 = v10[6];
          }
          else {
            do {
              v24 -= 1;
              v10[1] = (long)&v11[-4];
              sub_a3c0(&v11[-4],a5,v9);
              v11 = (unsigned long *)v10[1];
              if (v11 == (unsigned long *)v10[3]) break;
            } while (v24);
            v22 = *(char *)((long)v10 + 0x54);
            v21 = (unsigned long *)*v10;
            v20 = v10[5];
            v29 = (long)v6 - (long)v11 >> 5;
            v14 = v10[6];
          }
        }
        else if (((unsigned long *)v10[2] != v21) && (v24)) {
          do {
            v24 -= 1;
            *v10 = (long)&v21[-4];
            sub_a3c0(&v21[-4],a5,v9);
            v21 = (unsigned long *)*v10;
            if (v21 == (unsigned long *)v10[2]) break;
          } while (v24);
          v22 = *(char *)((long)v10 + 0x54);
          v29 = (long)v6 - v10[1] >> 5;
          v20 = v10[5];
          v14 = v10[6];
        }
        else {
          v22 = *(char *)((long)v10 + 0x54);
          v20 = v10[5];
          v14 = v29;
        }
      }
      else {
        v29 = 0;
        v14 = 0;
label_b618:
        v20 = v10[5];
        v24 = v23;
        if (v10[6] != v29) goto label_b8a8;
        v22 = *(char *)((long)v10 + 0x54);
        v14 = v29;
      }
    }
    v10[5] = v20 - ((long)v5 - (long)v21 >> 5);
    v10[6] = v14 - v29;
    if (!v22)
      sub_b1a0(a4,v10);
    if (2 <= *(unsigned int *)&v10[10]) { // branch-flip
      pthread_mutex_lock((void *)(v10[7] + 0x58));
      v23 = v10[7];
      if (!*(char *)(v23 + 0x54)) {
        sub_b1a0(a4,v23);
        v23 = v10[7];
      }
      pthread_mutex_unlock((void *)(v23 + 0x58));
    }
    else if (!(v10[6] + v10[5])) {
      v23 = v10[7];
      pthread_mutex_lock(v2);
      sub_df40(*a4,v23);
      *(char *)(v23 + 0x54) = 1;
      pthread_cond_signal(v1);
      pthread_mutex_unlock(v2);
    }
    pthread_mutex_unlock(v3);
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
  
  v2 = dat_1d888;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
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
      if (!strcmp("sort",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "sort";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "sort";
    if (!strcmp("sort","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "sort")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
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
unsigned long sub_cd00(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x17200)
    return 1;
  v1 = v2[2];
  if ((((dat_17214 < v1) && (v1 < dat_17218)) && (dat_1721c < v2[3])) && (dat_170f0 <= *v2)) {
    v3 = *v2 + dat_17214;
    if (((v3 < v2[1]) && (v2[1] <= dat_17220)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x17200;
  return 0;
}


// Function: sub_cd90 @ 0xcd90
unsigned long sub_cd90(long *a0,long a1,unsigned long *a2,char a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + *a0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*(void *)a0[7])(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*(void *)a0[7])(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0[9];
      a0[9] = (long)v1;
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
      v1[1] = a0[9];
      a0[9] = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}


// Function: sub_cfe0 @ 0xcfe0
unsigned long sub_cfe0(long *a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = (long *)*a1;
  if ((long *)a1[1] <= v6)
    return 1;
label_d016:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0[2];
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*(void *)a0[6])(v8,v4);
          v4 = a0[2];
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + *a0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0[9];
            a0[3] = a0[3] + 1;
            *v7 = 0;
            v7[1] = v8;
            a0[9] = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*(void *)a0[6])(v8,a0[2]);
        if ((unsigned long)a0[2] <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + *a0);
        if (*v7) { // branch-flip
          v5 = (long *)a0[9];
          if (v5) // branch-flip
            a0[9] = v5[1];
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
          a0[3] = a0[3] + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1[3] = a1[3] + -1;
        if ((long *)a1[1] <= v6)
          return 1;
        goto label_d016;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}


// Function: sub_d250 @ 0xd250
void sub_d250(unsigned long *a0,void *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_d3b0 @ 0xd3b0
long sub_d3b0(long *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = (void *)a0[7];
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
long sub_d470(long *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
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
    if ((long *)a0[1] <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}


// Function: sub_d550 @ 0xd550
long sub_d550(unsigned long *a0,void *a1,unsigned long a2)
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


// Function: sub_d640 @ 0xd640
unsigned long * sub_d640(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = 0x17200;
  v1[5] = a1;
  if (sub_cd00(v1)) {
    v2 = sub_cea0(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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
void sub_d750(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_d7eb;
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
label_d7eb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_d800 @ 0xd800
void sub_d800(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_d8ac;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_d86f;
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
label_d86f:
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
label_d8ac:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_d8f0 @ 0xd8f0
unsigned int sub_d8f0(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_cea0(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_cfe0(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_cfe0(a0,&v2,1)) && (sub_cfe0(a0,&v2,0))) {
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


// Function: sub_da70 @ 0xda70
long * sub_da70(long a0,long a1,long *a2) // return-dupe, ternary
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
  v4 = *(unsigned long *)(a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)(a0 + 0x10);
    if ((long)v2 > -1) goto label_daff;
label_db86:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_db86;
label_daff:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_cd00(a0);
    v1 = *(long *)(a0 + 0x28);
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
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)(a0 + 0x48);
  if (v2) // branch-flip
    *(long *)(a0 + 0x48) = v2[1];
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}


// Function: sub_dca0 @ 0xdca0
unsigned long sub_dca0(unsigned long a0,unsigned long a1) // early-return
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
long sub_dd00(long a0,unsigned long a1) // ternary x2
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_cd00(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_17228 <= v8) ? (long)(v8 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_d8f0(a0,v6)) {
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
        v3 = (unsigned long)(int)__sched_cpucount(0x80,v4);
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
    v1 = (unsigned long)(int)__sched_cpucount(0x80,v4);
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

