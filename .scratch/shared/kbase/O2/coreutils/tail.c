// Function: sub_2960 @ 0x2960
unsigned long sub_2960(int a0,unsigned long *a1)
{
  char v1;
  int v10;
  unsigned int v11;
  unsigned int v12; // eax
  char **v13;
  long *v14; // rax
  long v15; // rax
  void *v16;
  unsigned long v17;
  void *v18;
  double v19;
  unsigned int v2;
  double v20;
  char *v21;
  char *v22;
  unsigned long v23; // rax
  unsigned long v24; // rax
  unsigned long v25; // rax
  unsigned long v26; // rax
  int *v27;
  unsigned long v28; // rax
  unsigned long v29; // rax
  long v3;
  unsigned long v30; // rax
  unsigned long v31; // rax
  unsigned long v32; // rax
  long *v33;
  unsigned long v34; // rax
  unsigned long v35; // rax
  unsigned int *v36; // rax
  unsigned long v37; // rax
  unsigned long v38; // rax
  double v39;
  long v4;
  long v40;
  unsigned int v41;
  char **v42;
  char *v43;
  char v44;
  int v45;
  long v46;
  long *v47;
  double v48 [8]; // stack - 0x1c8
  int v49; // stack - 0xd8
  double v5;
  double v50; // stack - 0x1d8
  char v51 [24];
  long *v52; // stack - 0x1d0
  void *v53; // rbp
  unsigned long v54; // r10
  unsigned char v55;
  char **v56;
  void *v57;
  bool v58;
  unsigned char v59; // r14b
  char v6;
  long v60;
  long *v61; // stack - 0x260
  unsigned long v62; // stack - 0x258
  char *v63; // stack - 0x250
  void *v64; // stack - 0x248
  double v65; // stack - 0x240
  double v66; // stack - 0x238
  long v67; // stack - 0x230
  void *v68; // stack - 0x228
  long *v69; // stack - 0x220
  char v7;
  unsigned int v70; // stack - 0x210
  unsigned char v71; // stack - 0x209
  double v72; // stack - 0x208
  int v73; // stack - 0x184
  unsigned int v74; // stack - 0x150
  unsigned short v75; // stack - 0xd4
  unsigned short v76; // stack - 0xd2
  long v77; // stack - 0xd0
  unsigned int v78; // stack - 0xc0
  long v79; // stack - 0xa8
  char v8;
  long v80; // stack - 0xa0
  long v81; // stack - 0x80
  long v82; // stack - 0x78
  long v83; // stack - 0x40
  int v9;
  
  v83 = *(long *)(v60 + 0x28);
  v50 = 4.94065645841247e-323;
  sub_9150(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_cf70(sub_79b0);
  dat_1310e = '\0';
  dat_13119 = 1;
  dat_13115 = '\0';
  dat_13116 = 0;
  dat_13118 = '\0';
  dat_13114 = '\n';
  if (a0 != 2) { // branch-flip
    if (a0 == 3) {
      v22 = (char *)a1[2];
      if ((*v22 == '-') && (v22[1])) goto label_2a20;
      goto label_2a30;
    }
    if (a0 == 4) {
      v22 = (char *)a1[2];
label_2a20:
      if (!strcmp(v22,"--")) goto label_2a30;
    }
label_2a52:
    v15 = 0;
  }
  else {
label_2a30:
    v9 = sub_90c0();
    v43 = (char *)a1[1];
    v22 = &v43[1];
    if (*v43 == '+') {
      v55 = 1;
      if (0x2b8 < (unsigned int)(v9 - 0x30db0U)) goto label_38a5;
      goto label_2a52;
    }
    if ((*v43 != '-') || ((0x30db0 <= v9 && (!v43[(unsigned long)(v43[1] == 'c') + 1])))) goto label_2a52;
    v55 = 0;
label_38a5:
    v7 = v43[1];
    v43 = v22;
    while ((unsigned int)((int)v7 - 0x30U) <= 9) {
      v21 = &v43[1];
      v43 = &v43[1];
      v7 = *v21;
    }
    if (v7 != 'c') { // branch-flip
      if (v7 == 'l') {
        v59 = 1;
        v39 = 4.94065645841247e-323;
        goto label_41b9;
      }
      if (v7 == 'b') {
        v59 = 0;
        v39 = 2.52961610670718e-320;
        goto label_41b9;
      }
      v59 = 1;
      v39 = 4.94065645841247e-323;
      v21 = v43;
    }
    else {
      v59 = 0;
      v39 = 4.94065645841247e-323;
label_41b9:
      v21 = &v43[1];
    }
    v7 = *v21;
    if (v7 == 'f')
      v21 = &v21[1];
    if (*v21) goto label_2a52;
    if (v22 != v43) { // branch-flip
      if (sub_c520(v22,0,10,&v50,"b") & 0xfffffffd) {
        v25 = sub_b520(a1[1]);
        v22 = dcgettext(NULL,"invalid number",5);
        error(1,*__errno_location(),"%s: %s",v22,v25);
        return v32;
      }
    }
    else {
      v50 = v39;
    }
    v15 = 1;
    dat_13116 = v55;
    dat_13118 = v7 == 'f';
    dat_13119 = v59;
  }
  v9 = 0;
  v13 = (char **)&a1[v15];
  a0 -= (int)v15;
  v72 = dat_e4a0;
label_2a90:
  v10 = getopt_long(a0,v13,"c:n:fFqs:vz0123456789",(void *)0x12900,NULL);
  if (v10 != -1) {
    if (0x85 < v10)
      sub_6f70(1); // no-return, return-dupe
    if (v10 <= 0x2f) {
      if (v10 == -0x83) {
        sub_bb50(stdout,"tail","GNU coreutils",dat_13030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0,v54);
        exit(0); // no-return
      }
      if (v10 == -0x82)
        sub_6f70(0); // no-return
      sub_6f70(1);
    }
    switch(v10) {
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
        error(1,0,dcgettext(NULL,"option used in invalid context -- %c",5),v10);
        return v24;
      default:
        goto label_42ba;
      case 0x46:
        dat_13118 = '\x01';
        dat_13020 = 1;
        dat_1311a = 1;
        break;
      case 99:
      case 0x6e:
        dat_13119 = v10 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_13116 = 1;
        }
        if (v10 != 0x6e) // branch-flip
          v22 = dcgettext(NULL,"invalid number of bytes",5);
        else {
          v22 = dcgettext(NULL,"invalid number of lines",5);
        }
        v50 = (double)sub_c370(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v22,0);
        break;
      case 0x66:
      case 0x84:
        dat_13118 = '\x01';
        if (optarg) { // branch-flip
          v15 = sub_7710("--follow",optarg,0x12b00,0xe498,4,dat_13038,1);
          dat_13020 = *(int *)(v15 * 4 + 0xe498);
        }
        else {
          dat_13020 = 2;
        }
        break;
      case 0x71:
label_2cf0:
        v9 = 2;
        break;
      case 0x73:
        v7 = sub_c450(optarg,0,v48,sub_78d0);
        if (v7) {
          v72 = v48[0];
          if (0.0 <= v48[0]) break;
        }
        v13 = (char **)sub_b520(optarg);
        error(1,0,dcgettext(NULL,"invalid number of seconds: %s",5),v13);
        goto label_2cf0;
      case 0x76:
        v9 = 1;
        break;
      case 0x7a:
        dat_13114 = '\0';
        break;
      case 0x80:
        dat_1311a = 1;
        break;
      case 0x81:
        dat_13018 = sub_c370(optarg,0,0xffffffffffffffff,"",dcgettext(NULL,"invalid maximum number of unchanged stats between opens",5),0);
        break;
      case 0x82:
        dat_13110 = sub_c370(optarg,0,0x7fffffff,"",dcgettext(NULL,"invalid PID",5),0);
        break;
      case 0x83:
        dat_1310d = '\x01';
        break;
      case 0x85:
        goto label_2bd5;
      
    }
    goto label_2a90;
  }
  if (dat_1311a) { // branch-flip
    if (!dat_13118) {
      dat_1311a = 0;
      v22 = "warning: --retry ignored; --retry is useful only when following";
label_3152:
      error(0,0,dcgettext(NULL,v22,5));
      goto label_3167;
    }
    if (dat_13020 == 2) {
      v22 = "warning: --retry only effective for the initial open";
      goto label_3152;
    }
    if (dat_13110) goto label_2da1;
  }
  else {
label_3167:
    if (dat_13110) {
      if (dat_13118) { // branch-flip
label_2da1:
        v10 = kill(dat_13110,0);
        if ((v10) && (*__errno_location() == 0x26)) {
          error(0,0,dcgettext(NULL,"warning: --pid=PID is not supported on this system",5));
          dat_13110 = 0;
        }
      }
      else {
        error(0,0,dcgettext(NULL,"warning: PID ignored; --pid=PID is useful only when following",5));
      }
    }
  }
  if ((dat_13116) && (v50 != 0.0))
    v50 = (double)((long)v50 - 1);
  v42 = (char **)0x13028;
  v67 = 1;
  if (optind < a0) {
    v42 = &v13[optind];
    v67 = (long)(a0 - optind);
  }
  v58 = 0;
  v13 = &v42[v67];
  v56 = v42;
  do {
    v22 = *v56;
    if (!strcmp(v22,"-"))
      v58 = 1;
    v56 = &v56[1];
  } while (v56 != v13);
  if (v58) {
    if (dat_13020 == 1) {
      v25 = sub_ae30(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v25);
      return v31;
    }
    if (dat_13118) {
      if (((((dat_13110) || (v67 != 1)) || (dat_13020 != 2)) || ((fstat(0,&v49) || ((v78 & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2e85;
    }
    if (v50 == 0.0) goto label_3d8d;
  }
  else {
label_2e85:
    if ((v50 == 0.0) && (!dat_13118)) {
label_3d8d:
      v17 = 0;
      if (!dat_13116) goto label_3381;
    }
  }
  v14 = (long *)sub_be60(v67,0x60);
  v33 = v14;
  do {
    v22 = *v42;
    v42 = &v42[1];
    v47 = &v33[0xc];
    *v33 = (long)v22;
    v33 = v47;
  } while (v42 != v13);
  if ((v9 == 1) || ((!v9 && (v67 != 1))))
    dat_13115 = '\x01';
  v71 = 1;
  v61 = v14;
  v15 = v67 * 0x60;
  v33 = &v14[v67 * 0xc];
  do {
    v39 = v50;
    v65 = v50;
    v63 = (char *)*v61;
    v9 = strcmp(v63,"-");
    if (v9) { // branch-flip
      v10 = sub_7dd0(v63,0);
      v55 = dat_1311a;
      if (dat_1311a) { // branch-flip
        v63 = (char *)*v61;
        if (v10 != -1) goto label_2f60;
        *(char *)((long)v61 + 0x36) = 0;
      }
      else {
        v63 = (char *)*v61;
        *(char *)((long)v61 + 0x36) = 1;
        if (v10 != -1) goto label_2f69;
      }
      v27 = __errno_location();
      if (dat_13118) {
        v9 = *v27;
        *(unsigned int *)&v61[7] = 0xffffffff;
        *(int *)((long)v61 + 0x3c) = v9;
        *(unsigned char *)((long)v61 + 0x34) = v55 ^ 1;
        v61[5] = 0;
        v61[4] = 0;
      }
      if (!strcmp(v63,"-"))
        v63 = dcgettext(NULL,"standard input",5);
      v25 = sub_ae30(4,v63);
      v22 = dcgettext(NULL,"cannot open %s for reading",5);
      error(0,*v27,v22,v25);
      v71 = 0;
    }
    else {
      dat_1310e = '\x01';
      v10 = 0;
label_2f60:
      *(char *)((long)v61 + 0x36) = 1;
label_2f69:
      if (dat_13115) {
        if (!strcmp(v63,"-"))
          v63 = dcgettext(NULL,"standard input",5);
        v22 = "\n";
        if (dat_13010)
          v22 = "";
        __printf_chk(1,"%s==> %s <==\n",v22,v63);
        dat_13010 = '\0';
        v63 = (char *)*v61;
      }
      if (!strcmp(v63,"-"))
        v63 = dcgettext(NULL,"standard input",5);
      v55 = dat_13119;
      v48[0] = 0.0;
      if (dat_13119) { // branch-flip
        if (fstat(v10,&v49)) { // branch-flip
label_3590:
          v25 = sub_ae30(4,v63);
          v22 = dcgettext(NULL,"cannot fstat %s",5);
          error(0,*__errno_location(),v22,v25);
          v70 = 0;
        }
        else {
          v70 = (unsigned int)dat_13116;
          if (dat_13116) {
            v41 = sub_5ce0(v63,v10,v39,v48);
            if (v41) { // branch-flip
label_3df1:
              v70 = v41 >> 0x1f;
            }
            else {
              v48[0] = (double)((long)v48[0] + sub_6020(0,v63,v10,0xffffffffffffffff));
            }
            goto label_35da;
          }
          if (((!dat_1310d) && ((v78 & 0xf000) == 0x8000)) && (v46 = lseek(v10,0,1), v46 != -1)) {
            v19 = (double)lseek(v10,0,2);
            if (v46 < (long)v19) {
              v48[0] = v19;
              v70 = (unsigned int)v55;
              if (v19 != 0.0)
                v70 = (unsigned int)(unsigned char)sub_6190(v63,v10,v39,v46,v19,v48);
              goto label_35da;
            }
            if ((long)lseek(v10,v46,0) <= -1)
              sub_5f40(v46,0,v63); // no-return
          }
          v64 = (void *)sub_bd20(0x2018);
          *(unsigned long *)((long)v64 + 0x2008) = 0;
          *(unsigned long *)((long)v64 + 0x2000) = 0;
          *(unsigned long *)((long)v64 + 0x2010) = 0;
          v16 = (void *)sub_bd20(0x2018);
          v68 = v64;
          v66 = 0.0;
          v18 = v16;
          while( true ) {
            v57 = v18;
            v17 = sub_b540(v10,v16,_DT_INIT);
            if (v17 - 1 > 0xfffffffffffffffd) break;
            *(unsigned long *)((long)v57 + 0x2000) = v17;
            v45 = (int)dat_13114;
            v48[0] = (double)((long)v48[0] + v17);
            *(unsigned long *)((long)v57 + 0x2008) = 0;
            *(unsigned long *)((long)v57 + 0x2010) = 0;
            v18 = v16;
            while (v18 = memchr(v18,v45,(long)v16 + (v17 - (long)v18)), v18) {
              *(long *)((long)v57 + 0x2008) = *(long *)((long)v57 + 0x2008) + 1;
              v18 = (void *)((long)v18 + 1);
            }
            v66 = (double)((long)v66 + *(long *)((long)v57 + 0x2008));
            if (_DT_INIT <= (void *)(v17 + *(long *)((long)v64 + 0x2000))) { // branch-flip
              *(void **)((long)v64 + 0x2010) = v57;
              v19 = (double)((long)v66 - *(long *)((long)v68 + 0x2008));
              v64 = v57;
              if ((unsigned long)v19 <= (unsigned long)v39) { // branch-flip
                v16 = (void *)sub_bd20(0x2018);
                v18 = v16;
              }
              else {
                v16 = v68;
                v18 = v68;
                v66 = v19;
                v68 = *(void **)((long)v68 + 0x2010);
              }
            }
            else {
              memcpy((void *)(*(long *)((long)v64 + 0x2000) + (long)v64),v16,v17);
              *(long *)((long)v64 + 0x2000) = *(long *)((long)v64 + 0x2000) + *(long *)((long)v57 + 0x2000);
              *(long *)((long)v64 + 0x2008) = *(long *)((long)v64 + 0x2008) + *(long *)((long)v57 + 0x2008);
              v18 = v57;
            }
          }
          free(v57);
          if (v17 != 0xffffffffffffffff) { // branch-flip
            v46 = *(long *)((long)v64 + 0x2000);
            v70 = (unsigned int)(!v46 || v39 == 0.0);
            if (!v46 || v39 == 0.0) goto label_34b9;
            v45 = (int)dat_13114;
            if (*(char *)((long)v64 + v46 + -1) != dat_13114) {
              *(long *)((long)v64 + 0x2008) = *(long *)((long)v64 + 0x2008) + 1;
              v66 = (double)((long)v66 + 1);
            }
            v19 = (double)((long)v66 - *(long *)((long)v68 + 0x2008));
            v16 = v68;
            if ((unsigned long)v39 < (unsigned long)v19) {
              do {
                v66 = v19;
                v16 = *(void **)((long)v16 + 0x2010);
                v19 = (double)((long)v66 - *(long *)((long)v16 + 0x2008));
              } while ((unsigned long)v39 < (unsigned long)v19);
            }
            v46 = *(long *)((long)v16 + 0x2000);
            v18 = v16;
            if ((unsigned long)v39 < (unsigned long)v66) {
              v40 = (long)v66 - (long)v39;
              do {
                v18 = (void *)((long)rawmemchr(v18,v45) + 1);
                v40 -= 1;
              } while (v40);
            }
            v46 = (long)v16 + (v46 - (long)v18);
            if (v46)
              sub_5c60(v18,v46);
            for (v46 = *(long *)((long)v16 + 0x2010); v46; v46 = *(long *)(v46 + 0x2010)) {
              if (*(long *)&_DT_INIT[v46])
                sub_5c60(v46,*(long *)&_DT_INIT[v46]);
            }
            v70 = (unsigned int)v55;
          }
          else {
            v25 = sub_ae30(4,v63);
            v22 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v22,v25);
label_34b9:
            if (!v68) goto label_35da;
          }
          do {
            v16 = *(void **)((long)v68 + 0x2010);
            free(v68);
            v68 = v16;
          } while (v16);
        }
      }
      else {
        if (fstat(v10,&v49)) goto label_3590;
        v70 = (unsigned int)dat_13116;
        if (dat_13116) { // branch-flip
          if ((dat_1310d) || ((long)v39 <= -1)) {
label_3a3e:
            v41 = sub_5e30(v63,v10,v39,v48);
            v65 = -NAN;
            if (v41) goto label_3df1;
          }
          else {
            if ((v78 & 0xf000) != 0x8000) { // branch-flip
              if (lseek(v10,v39,1) == -1) goto label_3a3e;
            }
            else if ((long)lseek(v10,v39,1) <= -1)
              sub_5f40(v39,1,v63); // no-return
            v65 = -NAN;
            v48[0] = v39;
          }
label_3a6a:
          v70 = 1;
          v48[0] = (double)((long)v48[0] + sub_6020(0,v63,v10,v65));
        }
        else {
          if ((!dat_1310d) && (0 <= (long)v39)) {
            if ((v78 & 0xd000) != 0x8000) { // branch-flip
              v19 = (double)lseek(v10,-(long)v39,2);
              if (v19 == -NAN) goto label_3678;
              v46 = (long)v39 + (long)v19;
            }
            else {
              v19 = -NAN;
              v46 = v79;
            }
            v40 = v80;
            if (0x2000000000000000 <= (unsigned long)(v80 - 1U))
              v40 = 0x200;
            if (v40 < v46) {
              if ((v19 == -NAN) && (v19 = (double)lseek(v10,0,1), (long)v19 <= -1))
                sub_5f40(0,1,v63); // no-return
              if (((long)v19 < v46) && ((unsigned long)v39 < (unsigned long)(v46 - (long)v19))) {
                v19 = (double)(v46 - (long)v39);
                if ((long)lseek(v10,v19,0) <= -1)
                  sub_5f40(v19,0,v63); // no-return
              }
              v48[0] = v19;
              goto label_3a6a;
            }
          }
label_3678:
          v16 = (void *)sub_bd20(0x2010);
          *(unsigned long *)((long)v16 + 0x2000) = 0;
          *(unsigned long *)((long)v16 + 0x2008) = 0;
          v18 = (void *)sub_bd20(0x2010);
          v57 = v16;
          v19 = 0.0;
          while( true ) {
            v53 = v18;
            v18 = v16;
            v17 = sub_b540(v10,v53,_DT_INIT);
            if (v17 - 1 > 0xfffffffffffffffd) break;
            v48[0] = (double)((long)v48[0] + v17);
            v19 = (double)((long)v19 + v17);
            *(unsigned long *)((long)v53 + 0x2000) = v17;
            v46 = *(long *)((long)v57 + 0x2000);
            *(unsigned long *)((long)v53 + 0x2008) = 0;
            v16 = v18; // branch-flip
            if (_DT_INIT <= (void *)(v17 + v46)) {
              *(void **)((long)v57 + 0x2008) = v53;
              v20 = (double)((long)v19 - *(long *)((long)v18 + 0x2000));
              v57 = v53;
              if ((unsigned long)v20 <= (unsigned long)v39) // branch-flip
                v18 = (void *)sub_bd20(0x2010);
              else {
                v16 = *(void **)((long)v18 + 0x2008);
                v19 = v20;
              }
            }
            else {
              memcpy((void *)((long)v57 + v46),v53,v17);
              *(long *)((long)v57 + 0x2000) = *(long *)((long)v57 + 0x2000) + *(long *)((long)v53 + 0x2000);
              v18 = v53;
            }
          }
          free(v53);
          if (v17 != 0xffffffffffffffff) { // branch-flip
            v46 = *(long *)((long)v18 + 0x2000);
            v20 = (double)((long)v19 - v46);
            v16 = v18;
            while (v5 = v20, (unsigned long)v39 < (unsigned long)v5) {
              v16 = *(void **)((long)v16 + 0x2008);
              v46 = *(long *)((long)v16 + 0x2000);
              v20 = (double)((long)v5 - v46);
              v19 = v5;
            }
            v40 = 0;
            if ((unsigned long)v39 < (unsigned long)v19)
              v40 = (long)v19 - (long)v39;
            if (v46 - v40)
              sub_5c60((long)v16 + v40,v46 - v40);
            while (v16 = *(void **)((long)v16 + 0x2008), v16) {
              if (*(long *)((long)v16 + 0x2000))
                sub_5c60(v16,*(long *)((long)v16 + 0x2000));
            }
            v70 = 1;
          }
          else {
            v25 = sub_ae30(4,v63);
            v22 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v22,v25);
            if (!v18) goto label_35da;
          }
          do {
            v16 = *(void **)((long)v18 + 0x2008);
            free(v18);
            v18 = v16;
          } while (v16);
        }
      }
label_35da:
      if (dat_13118) { // branch-flip
        *(unsigned int *)((long)v61 + 0x3c) = v70 - 1;
        if (0 <= fstat(v10,&v49)) { // branch-flip
          if (((v78 & 0xf000) - 0x1000 & 0xffffe000) && ((v78 & 0xb000) != 0x8000)) {
            v22 = "";
            *(unsigned int *)((long)v61 + 0x3c) = 0xffffffff;
            *(char *)((long)v61 + 0x36) = 0;
            *(unsigned char *)((long)v61 + 0x34) = dat_1311a ^ 1;
            if (dat_1311a ^ 1)
              v22 = dcgettext(NULL,"; giving up on this name",5);
            v43 = (char *)*v61;
            if (!strcmp(v43,"-"))
              v43 = dcgettext(NULL,"standard input",5);
            v25 = sub_b1b0(0,3,v43);
            error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v25,v22);
          }
          else if ((unsigned char)v70) {
            *(int *)&v61[7] = v10;
            v22 = (char *)*v61;
            *(unsigned int *)&v61[6] = v78;
            v61[1] = (long)v48[0];
            *(unsigned int *)&v61[8] = -(unsigned int)(v9 == 0) | 1;
            v61[2] = v81;
            v61[0xb] = 0;
            v61[3] = v82;
            *(char *)((long)v61 + 0x34) = 0;
            v61[4] = CONCAT26(v76,CONCAT24(v75,v49));
            v61[5] = v77;
            if (!strcmp(v22,"-"))
              v22 = dcgettext(NULL,"standard input",5);
            *(char *)((long)v61 + 0x35) = sub_5460(v10,v22);
            goto label_3340;
          }
        }
        else {
          v27 = __errno_location();
          v22 = (char *)*v61;
          *(int *)((long)v61 + 0x3c) = *v27;
          if (!strcmp(v22,"-"))
            v22 = dcgettext(NULL,"standard input",5);
          v25 = sub_ae30(4,v22);
          v22 = dcgettext(NULL,"error reading %s",5);
          error(0,*v27,v22,v25);
        }
        v22 = (char *)*v61;
        *(unsigned char *)((long)v61 + 0x34) = dat_1311a ^ 1;
        if (!strcmp(v22,"-"))
          v22 = dcgettext(NULL,"standard input",5);
        sub_5bd0(v10,v22);
        v71 = 0;
        *(unsigned int *)&v61[7] = 0xffffffff;
      }
      else if ((v9) && (close(v10))) {
        v22 = (char *)*v61;
        if (!strcmp(v22,"-"))
          v22 = dcgettext(NULL,"standard input",5);
        v25 = sub_ae30(4,v22);
        v22 = dcgettext(NULL,"error reading %s",5);
        error(0,*__errno_location(),v22,v25);
        v71 = 0;
      }
      else {
        v71 &= (unsigned char)v70;
      }
    }
label_3340:
    v7 = dat_13118;
    v61 = &v61[0xc];
  } while (v33 != v61);
  if (dat_13118) {
    v46 = 0;
    v47 = v14;
    do {
      v22 = (char *)*v47;
      if ((((strcmp(v22,"-")) || (*(char *)((long)v47 + 0x34))) || ((int)v47[7] <= -1)) || ((*(unsigned int *)&v47[6] & 0xf000) != 0x1000))
        v46 += 1;
      else {
        *(unsigned int *)&v47[7] = 0xffffffff;
        *(char *)((long)v47 + 0x34) = 1;
      }
      v47 = &v47[0xc];
    } while (v33 != v47);
    if (v46) {
      if (fstat(1,v51) < 0) {
        v22 = dcgettext(NULL,"standard output",5);
        error(1,*__errno_location(),v22);
        return v28;
      }
      dat_13117 = (v74 & 0xf000) == 0x1000;
      if (!dat_1310c) {
        v47 = v14;
        do {
          if ((!*(char *)((long)v47 + 0x34)) && (v22 = (char *)*v47, !strcmp(v22,"-"))) goto label_3b98;
          v47 = &v47[0xc];
        } while (v33 != v47);
        v46 = 0;
        do {
          if ((0 <= *(int *)((long)v14 + v46 + 0x38)) && (*(char *)((long)v14 + v46 + 0x35))) goto label_3b98;
          v46 += 0x60;
        } while (v15 != v46);
        v46 = 0;
        do {
          if ((0 <= *(int *)((long)v14 + v46 + 0x38)) && (!*(char *)((long)v14 + v46 + 0x35))) {
            v44 = '\0';
            v47 = v14;
            goto label_484a;
          }
          v46 += 0x60;
        } while (v15 != v46);
      }
label_3b98:
      dat_1310c = '\x01';
      if (dat_13110) // branch-flip
        v41 = 0;
      else {
        v41 = 0;
        if (((dat_13020 == 2) && (v67 == 1)) && ((int)v14[7] != -1))
          v41 = (unsigned int)((*(unsigned int *)&v14[6] & 0xf000) != 0x8000);
      }
      v63 = (long)(v67 + -1);
      v44 = '\0';
label_3bf5:
      v58 = 0;
      v46 = 0;
      v33 = v14;
      do {
        while( true ) {
          if (*(char *)((long)v33 + 0x34)) goto label_3ca6;
          v9 = (int)v33[7];
          if (v9 <= -1) break;
          v22 = (char *)*v33;
          if (!strcmp(v22,"-"))
            v22 = dcgettext(NULL,"standard input",5);
          v2 = *(unsigned int *)&v33[6];
          v11 = *(unsigned int *)&v33[8];
          if (v41 != *(unsigned int *)&v33[8]) {
            v11 = sub_7b60(v9,3);
            if (v41) { // branch-flip
              if (0 <= (int)v11) {
label_3c76:
                *(unsigned int *)&v33[8] = v41;
                v11 = v41;
                goto label_3c7c;
              }
            }
            else if ((0 <= (int)v11) && ((v11 == (v11 | 0x800) || (sub_7b60(v9,4) != -1)))) goto label_3c76;
            v27 = __errno_location();
            if (((*(unsigned int *)&v33[6] & 0xf000) != 0x8000) || (*v27 != 1)) {
              v25 = sub_b1b0(0,3,v22);
              v22 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
              error(1,*v27,v22,v25);
              return v29;
            }
            v11 = *(unsigned int *)&v33[8];
          }
label_3c7c:
          v40 = -2;
          if (v11) { // branch-flip
label_3c8b:
            v40 = sub_6020(0,v22,v9,v40);
            v33[1] = v33[1] + v40;
            v58 = (bool)(v58 | v40 != 0);
          }
          else if (fstat(v9,&v49)) { // branch-flip
            *(unsigned int *)&v33[7] = 0xffffffff;
            v27 = __errno_location();
            *(int *)((long)v33 + 0x3c) = *v27;
            v25 = sub_b1b0(0,3,v22);
            error(0,*v27,"%s",v25);
            close(v9);
          }
          else {
            if ((*(unsigned int *)&v33[6] != v78) || ((((v78 & 0xf000) == 0x8000 && (v33[1] != v79)) || (((unsigned int)(v82 < v33[3]) - (unsigned int)(v33[3] < v82)) + ((unsigned int)(v81 < v33[2]) - (unsigned int)(v33[2] < v81)) * 2)))) {
              *(unsigned int *)&v33[6] = v78;
              v33[0xb] = 0;
              v33[2] = v81;
              v33[3] = v82;
              if (((v2 & 0xf000) == 0x8000) && (v79 < v33[1])) {
                v25 = sub_b1b0(0,3,v22);
                error(0,0,dcgettext(NULL,"%s: file truncated",5),v25);
                if ((long)lseek(v9,0,0) < 0)
                  sub_5f40(0,0,v22); // no-return
                v33[1] = 0;
              }
              if (((char *)v46 != v63) && (dat_13115)) {
                v43 = "\n";
                if (dat_13010)
                  v43 = "";
                __printf_chk(1,"%s==> %s <==\n",v43,v22);
                dat_13010 = '\0';
              }
              v63 = (char *)v46;
              if ((int)v33[8]) // branch-flip
                v40 = -2;
              else {
                v40 = -1;
                if (((v2 & 0xf000) == 0x8000) && (*(char *)((long)v33 + 0x35)))
                  v40 = v79 - v33[1];
              }
              goto label_3c8b;
            }
            v17 = v33[0xb];
            v33[0xb] = v17 + 1;
            if ((dat_13018 <= v17) && (dat_13020 == 1)) {
              sub_6660(v33,(int)v33[8] != 0);
              v33[0xb] = 0;
            }
          }
label_3ca6:
          v46 += 1;
          v33 = &v33[0xc];
          if (v67 == v46) goto label_3ce2;
        }
        v46 += 1;
        v47 = &v33[0xc];
        sub_6660(v33,v41);
        v33 = v47;
      } while (v67 != v46);
label_3ce2:
      if ((!dat_1311a) || (dat_13020 != 1)) {
        v22 = (char *)((long)v14 + 0x34);
        while ((*(int *)&v22[4] <= -1 && ((*v22 == '\x01' || (!dat_1311a))))) {
          v22 = &v22[0x60];
          if (v22 == (char *)((long)v14 + v15 + 0x34)) {
            error(0,0,dcgettext(NULL,"no files remaining",5));
            goto label_3369;
          }
        }
      }
      if ((!v58) || (v41)) {
        v9 = fflush_unlocked(stdout);
        if (v9) {
          v22 = dcgettext(NULL,"write error",5);
          error(1,*__errno_location(),v22);
          return v30;
        }
        if (dat_13117) {
          v48[0] = 4.94065645841247e-324;
          if (0 <= (int)poll(v48,1,0)) goto label_45f9;
        }
      }
      else {
        if (!dat_13117) goto label_3bf5;
        v48[0] = 4.94065645841247e-324;
        if ((int)poll(v48,1,0) < 0) goto label_3bf5;
label_45f9:
        if ((unsigned long)v48[0] & 0x18000000000000) {
          raise(0xd); // return-dupe
          exit(1); // return-dupe, no-return
        }
      }
      if (!v58) {
        if (v44) goto label_3369;
        if (((dat_13110) && (v9 = kill(dat_13110,0), v9)) && (*__errno_location() != 1))
          v44 = v7;
        else if (sub_c3a0(v72)) {
          v22 = dcgettext(NULL,"cannot read realtime clock",5);
          error(1,*__errno_location(),v22);
          return v26;
        }
      }
      goto label_3bf5;
    }
  }
label_3369:
  if ((dat_1310e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v23;
  }
  v17 = (unsigned long)(v71 ^ 1);
label_3381:
  if (v83 == *(long *)(v60 + 0x28))
    return v17;
  __stack_chk_fail(); // no-return
label_2bd5:
  dat_1310c = '\x01';
  goto label_2a90;
  while (v47 = &v47[0xc], v33 != v47) {
label_484a:
    v22 = (char *)*v47;
    if ((!lstat(v22,&v49)) && ((v78 & 0xf000) == 0xa000)) goto label_3b98;
  }
  v46 = 0;
  do {
    if (((0 <= *(int *)((long)v14 + v46 + 0x38)) && (v41 = *(unsigned int *)((long)v14 + v46 + 0x30) & 0xf000, v41 != 0x8000)) && (v41 != 0x1000)) goto label_3b98;
    v46 += 0x60;
  } while (v15 != v46);
  if ((v71) || (dat_13020 != 2)) {
    v9 = inotify_init();
    if (0 <= v9) {
      v10 = fflush_unlocked(stdout);
      if (v10) {
        v22 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v22);
        return v38;
      }
      v46 = sub_87c0(v67,0,sub_5430,sub_5450,0);
      if (!v46)
        sub_c220(); // no-return, return-dupe
      v12 = 0xc06;
      if (dat_13020 != 1)
        v12 = 2;
      v8 = '\0';
      v6 = '\0';
      v62 = 0;
      v47 = v14;
      do {
        v10 = dat_13020;
        if (!*(char *)((long)v47 + 0x34)) {
          v22 = (char *)*v47;
          v17 = strlen(v22);
          *(unsigned int *)((long)v47 + 0x44) = 0xffffffff;
          if (v17 <= v62)
            v17 = v62;
          if (v10 == 1) {
            v40 = sub_7a60(v22);
            v1 = v22[v40];
            v47[10] = sub_7840(v22) - (long)v22;
            v22[v40] = '\0';
            v22 = ".";
            if (v40)
              v22 = (char *)*v47;
            *(unsigned int *)&v47[9] = inotify_add_watch(v9,v22,0x784);
            *(char *)(*v47 + v40) = v1;
            if ((int)v47[9] < 0) {
              v27 = __errno_location();
              if (*v27 != 0x1c) { // branch-flip
                v25 = sub_ae30(4,*v47);
                v22 = dcgettext(NULL,"cannot watch parent directory of %s",5);
                error(0,*v27,v22,v25);
              }
              else {
label_4c07:
                v22 = "inotify resources exhausted";
label_4c13:
                error(0,0,dcgettext(NULL,v22,5));
              }
              goto label_4c28;
            }
          }
          v10 = inotify_add_watch(v9,*v47,v12);
          *(int *)((long)v47 + 0x44) = v10;
          v62 = v17;
          if (0 <= v10) { // branch-flip
            if (!sub_8e20(v46,v47)) {
              sub_c220();
            }
            v6 = v7;
          }
          else {
            if ((int)v47[7] != -1)
              v8 = v7;
            v36 = (unsigned int *)__errno_location();
            if ((*v36 & 0xffffffef) == 0xc) goto label_4c07;
            if (*v36 != *(unsigned int *)((long)v47 + 0x3c)) {
              v25 = sub_ae30(4,*v47);
              v22 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v36,v22,v25);
            }
          }
        }
        v47 = &v47[0xc];
      } while (v33 != v47);
      if (dat_13020 != 2) {
label_4c85:
        v52 = &v14[v67 * 0xc + -0xc];
        v47 = v14;
        do {
          if (!*(char *)((long)v47 + 0x34)) {
            if (dat_13020 != 1) { // branch-flip
              if ((((int)v47[7] != -1) && (v22 = (char *)*v47, !stat(v22,&v49))) && ((v47[4] != CONCAT26(v76,CONCAT24(v75,v49)) || (v47[5] != v77)))) {
                v40 = *v47;
                v25 = sub_ae30(4,sub_5fe0(v40));
                v22 = dcgettext(NULL,"%s was replaced",5);
                error(0,*__errno_location(),v22,v25);
                goto label_4c28;
              }
            }
            else {
              sub_6660(v47,0);
            }
            sub_6430(v47,&v52);
          }
          v47 = &v47[0xc];
        } while (v33 != v47);
        v17 = 0;
        v65 = (long)(v62 + 0x11);
        v66._0_4_ = 3;
        v62 = sub_bd20(v65);
        v63 = NULL;
label_4d4b:
        do {
          if (((dat_13020 == 1) && (!dat_1311a)) && (!sub_82f0(v46))) {
            error(1,0,dcgettext(NULL,"no files remaining",5));
            return v34;
          }
          if (v63 > v17) {
label_4d63:
            v27 = (int *)(v62 + v17);
            v41 = v27[1];
            v2 = v27[3];
            v10 = *v27;
            v17 = v17 + 0x10 + (unsigned long)v2;
            if (v41 & 0x400) { // branch-flip
              if (!v2) {
                v40 = 0;
                do {
                  if (v10 == (int)v14[v40 * 0xc + 9]) {
                    v22 = "directory containing watched file was removed";
                    goto label_4c13;
                  }
                  v40 += 1;
                } while (v67 != v40);
label_4e77:
                v73 = v10;
                v33 = (long *)sub_8530(v46,v48);
                if (v33) {
label_4e47:
                  if (v27[1] & 0xe04U) { // branch-flip
                    if (v27[1] & 0x400U) {
                      inotify_rm_watch(v9,*(unsigned int *)((long)v33 + 0x44));
                      sub_8e80(v46,v33);
                    }
                    sub_6660(v33,0);
                  }
                  else {
                    sub_6430(v33,&v52);
                  }
                }
                goto label_4d4b;
              }
            }
            else if (!v2) goto label_4e77;
            v40 = 0;
            v33 = v14;
            do {
              if ((v10 == (int)v33[9]) && (v3 = v33[10], v4 = *v33, !strcmp((char *)&v27[4],(char *)(v3 + v4)))) {
                if (v67 == v40) break;
                v33 = &v14[v40 * 0xc];
                if (!(v41 & 0x200)) {
                  v10 = inotify_add_watch(v9,*v33,v12);
                  if (v10 <= -1) {
                    v36 = (unsigned int *)__errno_location();
                    if ((*v36 & 0xffffffef) == 0xc) goto label_4c07;
                    v25 = sub_ae30(4,*v33);
                    v22 = dcgettext(NULL,"cannot watch %s",5);
                    error(0,*v36,v22,v25);
                  }
                  v45 = *(int *)((long)v33 + 0x44);
                  if (v10 != v45) { // branch-flip
                    if (0 <= v45) {
                      inotify_rm_watch(v9);
                      sub_8e80(v46,v33);
                    }
                  }
                  else if (0 <= v45) goto label_4e3a;
                  *(int *)((long)v33 + 0x44) = v10;
                  if (v10 == -1) break;
                  v69 = (long *)sub_8e80(v46,v33);
                  if ((v69) && (v33 != v69)) {
                    if (dat_13020 == 1)
                      sub_6660(v69,0);
                    *(unsigned int *)((long)v69 + 0x44) = 0xffffffff;
                    v25 = sub_5fe0(*v69);
                    sub_5bd0((int)v69[7],v25);
                  }
                  if (!sub_8e20(v46,v33)) {
                    sub_c220();
                  }
                }
label_4e3a:
                if (dat_13020 == 1)
                  sub_6660(v33,0);
                goto label_4e47;
              }
              v40 += 1;
              v33 = &v33[0xc];
            } while (v67 != v40);
            goto label_4d4b;
          }
          do {
            if (dat_13110) { // branch-flip
              if (v44)
                exit(0); // no-return
              v10 = kill(dat_13110,0);
              if ((v10) && (*__errno_location() != 1)) {
                v10 = 0;
                v44 = v7;
              }
              else {
                v10 = 0;
                if (0.0 < v72) {
                  if (dat_e4a8 <= v72) goto label_4fb1;
                  v10 = (int)(dat_e4b0 * v72);
                  v10 += (unsigned int)((double)v10 < dat_e4b0 * v72);
                }
              }
            }
            else {
label_4fb1:
              v10 = -1;
            }
            v75 = 1;
            v77 = 1;
            v49 = v9;
            v10 = poll(&v49,(unsigned long)dat_13117 + 1,v10);
          } while (!v10);
          if (v10 < 0) {
            v22 = dcgettext(NULL,"error waiting for inotify and output events",5);
            error(1,*__errno_location(),v22);
            return v37;
          }
          if (v77._6_2_) {
            raise(0xd);
            exit(1);
          }
          v63 = (unsigned long)sub_b540(v9,v62,v65);
          if (v63) {
            v17 = 0;
            if (v63 != (char *)0xffffffffffffffff) goto label_4d63;
            if (*__errno_location() != 0x16) {
label_500c:
              v22 = dcgettext(NULL,"error reading inotify event",5);
              error(1,*__errno_location(),v22);
              return v35;
            }
          }
          if (!v66._0_4_) goto label_500c;
          v17 = 0;
          v66._0_4_ -= 1;
          v65 = (long)((long)v65 * 2);
          v63 = NULL;
          v62 = sub_bd70(v62,v65);
        } while( true );
      }
      if (!v8) {
        if (!v6) {
          exit(1);
        }
        goto label_4c85;
      }
label_4c28:
      sub_8980(v46);
      close(v9);
      *__errno_location() = 0;
    }
    v22 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
    error(0,*__errno_location(),v22);
  }
  goto label_3b98;
  raise(0xd);
  exit(1);
}


// Function: sub_5430 @ 0x5430
unsigned long sub_5430(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}


// Function: sub_5450 @ 0x5450
unsigned int sub_5450(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}


// Function: sub_5460 @ 0x5460
unsigned long sub_5460(unsigned long a0,unsigned long a1) // ternary
{
  int *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  long v4 [15]; // stack - 0xa8
  unsigned long v5; // r12
  unsigned long v6; // r12
  undefined7 v7; // r12
  
  if (fstatfs(a0,v4)) {
    v1 = __errno_location();
    v6 = 1;
    if (*v1 != 0x26) {
      v2 = sub_ae30(4,a1);
      v3 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
      error(0,*v1,v3,v2);
    }
    return v6 & 0xffffffff; // return-dupe
  }
  if (v4[0] != 0x2bad1dea) {
    v7 = (undefined7)((unsigned long)v5 >> 8); // branch-flip
    if (0x2bad1deb <= v4[0]) {
      if (v4[0] != 0x62656570) {
        if (0x62656571 <= v4[0]) { // branch-flip
          if (v4[0] != 0x858458f6) {
            if (0x858458f7 <= v4[0]) { // branch-flip
              if (v4[0] != 0xcafe4a11) {
                if (0xcafe4a12 <= v4[0]) { // branch-flip
                  if (v4[0] != 0xf2f52010) {
                    if (0xf2f52011 <= v4[0]) { // branch-flip
                      if (v4[0] != 0xf97cff8c) {
                        v6 = CONCAT71(v7,v4[0] != 0xf995e849);
                        return v6 & 0xffffffff;
                      }
                    }
                    else if (v4[0] != 0xde5e81e4) {
                      v6 = CONCAT71(v7,v4[0] != 0xe0f5e1e2);
                      return v6 & 0xffffffff;
                    }
                  }
                }
                else if (v4[0] != 0xabba1974) {
                  if (0xabba1975 <= v4[0]) { // branch-flip
                    if (v4[0] != 0xc7571590) {
                      v6 = CONCAT71(v7,v4[0] != 0xc97e8168);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4[0] != 0x9123683e) {
                    v6 = CONCAT71(v7,v4[0] != 0x958458f6);
                    return v6 & 0xffffffff;
                  }
                }
              }
            }
            else if (v4[0] != 0x6c6f6f70) {
              if (0x6c6f6f71 <= v4[0]) { // branch-flip
                if (v4[0] != 0x73717368) {
                  if (0x73717369 <= v4[0]) { // branch-flip
                    if (v4[0] != 0x73727279) {
                      v6 = CONCAT71(v7,v4[0] != 0x74726163);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4[0] != 0x6e736673) {
                    v6 = CONCAT71(v7,v4[0] != 0x73636673);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4[0] != 0x64646178) {
                if (0x64646179 <= v4[0]) { // branch-flip
                  if (v4[0] != 0x67596969) {
                    v6 = CONCAT71(v7,v4[0] != 0x68191122);
                    return v6 & 0xffffffff;
                  }
                }
                else if ((v4[0] != 0x63677270) && (v4[0] != 0x64626720)) {
                  v6 = CONCAT71(v7,v4[0] != 0x62656572);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
        }
        else if (v4[0] != 0x53464846) {
          if (0x53464847 <= v4[0]) { // branch-flip
            if (v4[0] != 0x58465342) {
              if (0x58465343 <= v4[0]) { // branch-flip
                if (v4[0] != 0x5dca2df5) {
                  if (0x5dca2df6 <= v4[0]) { // branch-flip
                    if (v4[0] != 0x6165676c) {
                      v6 = CONCAT71(v7,v4[0] != 0x62646576);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4[0] != 0x5a3c69f0) {
                    v6 = CONCAT71(v7,v4[0] != 0x5a4f4653);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4[0] != 0x54190100) {
                if (0x54190101 <= v4[0]) { // branch-flip
                  if (v4[0] != 0x565a4653) {
                    v6 = CONCAT71(v7,v4[0] != 0x58295829);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4[0] != 0x5346544e) {
                  v6 = CONCAT71(v7,v4[0] != 0x534f434b);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
          else if (v4[0] != 0x453dcd28) {
            if (0x453dcd29 <= v4[0]) { // branch-flip
              if (v4[0] != 0x52654973) {
                if (0x52654974 <= v4[0]) { // branch-flip
                  if (v4[0] != 0x5345434d) {
                    v6 = CONCAT71(v7,v4[0] != 0x5346314d);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4[0] != 0x454d444d) {
                  v6 = CONCAT71(v7,v4[0] != 0x45584653);
                  return v6 & 0xffffffff;
                }
              }
            }
            else if (v4[0] != 0x42494e4d) {
              if (0x42494e4e <= v4[0]) { // branch-flip
                if (v4[0] != 0x43415d53) {
                  v6 = CONCAT71(v7,v4[0] != 0x444d4142);
                  return v6 & 0xffffffff;
                }
              }
              else if ((v4[0] != 0x3153464a) && (v4[0] != 0x42465331)) {
                v6 = CONCAT71(v7,v4[0] != 0x2fc12fc1);
                return v6 & 0xffffffff;
              }
            }
          }
        }
      }
    }
    else if (0x9fa3 <= v4[0]) { // branch-flip
      if (0x12ff7b8 <= v4[0]) { // branch-flip
        if (v4[0] != 0x15013346) {
          if (0x15013347 <= v4[0]) { // branch-flip
            if (v4[0] != 0x2011bab0) {
              if (0x2011bab1 <= v4[0]) { // branch-flip
                if (v4[0] != 0x24051905) {
                  v6 = CONCAT71(v7,v4[0] != 0x28cd3d45);
                  return v6 & 0xffffffff;
                }
              }
              else if (v4[0] != 0x19800202) {
                v6 = CONCAT71(v7,v4[0] != 0x1badface);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4[0] != 0xbad1dea) {
            if (0xbad1deb <= v4[0]) { // branch-flip
              if (v4[0] != 0x11307854) {
                v6 = CONCAT71(v7,v4[0] != 0x13661366);
                return v6 & 0xffffffff;
              }
            }
            else if (v4[0] != 0x7655821) {
              v6 = CONCAT71(v7,v4[0] != 0x9041934);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else if ((v4[0] <= 0x12ff7b3) && (v4[0] != 0x27e0eb)) {
        if (0x27e0ec <= v4[0]) { // branch-flip
          if (v4[0] != 0x1021994) {
            if (0x1021995 <= v4[0]) { // branch-flip
              if (v4[0] != 0x1021997) {
                v6 = CONCAT71(v7,v4[0] != 0x12fd16d);
                return v6 & 0xffffffff;
              }
            }
            else if (v4[0] != 0x414a53) {
              v6 = CONCAT71(v7,v4[0] != 0xc0ffee);
              return v6 & 0xffffffff;
            }
          }
        }
        else if (v4[0] != 0xef53) {
          if (0xef54 <= v4[0]) { // branch-flip
            if (v4[0] != 0xf15f) {
              v6 = CONCAT71(v7,v4[0] != 0x11954);
              return v6 & 0xffffffff;
            }
          }
          else if ((v4[0] != 0xadff) && (v4[0] != 0xef51)) {
            v6 = CONCAT71(v7,v4[0] != 0xadf5);
            return v6 & 0xffffffff;
          }
        }
      }
    }
    else if (v4[0] <= 0x9f9f) {
      if (0x4007 <= v4[0]) { // branch-flip
        if (v4[0] != 0x4d5a) {
          if (0x4d5b <= v4[0]) { // branch-flip
            if (v4[0] != 0x72b6) {
              if (0x72b7 <= v4[0]) {
                v6 = CONCAT71(v7,v4[0] != 0x9660);
                return v6 & 0xffffffff;
              }
              if (v4[0] != 0x5df5) {
                v6 = CONCAT71(v7,v4[0] != 0x7275);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4[0] != 0x4858) {
            if (0x4859 <= v4[0]) {
              v6 = CONCAT71(v7,v4[0] != 0x4d44);
              return v6 & 0xffffffff;
            }
            if (v4[0] != 0x4244) {
              v6 = CONCAT71(v7,v4[0] != 0x482b);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else {
        if (0x3fff < v4[0]) {
          v6 = (unsigned long)((unsigned int)(0x51L >> ((unsigned char)v4[0] & 0x3f)) & 1 ^ 1);
          return v6 & 0xffffffff;
        }
        if (0x1390 <= v4[0]) { // branch-flip
          if (v4[0] != 0x2478) {
            if (0x2479 <= v4[0]) {
              v6 = CONCAT71(v7,v4[0] != 0x3434);
              return v6 & 0xffffffff;
            }
            if (v4[0] != 0x1cd1) {
              v6 = CONCAT71(v7,v4[0] != 0x2468);
              return v6 & 0xffffffff;
            }
          }
        }
        else {
          if (0x1372 < v4[0]) {
            v6 = (unsigned long)((unsigned int)(0x10001401L >> ((unsigned char)v4[0] + 0x8d & 0x3f)) & 1 ^ 1);
            return v6 & 0xffffffff;
          }
          if (v4[0] != 0x187) {
            v6 = (0x188 <= v4[0]) ? CONCAT71(v7,v4[0] != 0x7c0) : CONCAT71(v7,(v4[0] - 0x2fU & 0xfffffffffffffffbU) != 0); // branch-flip
            return v6 & 0xffffffff;
          }
        }
      }
    }
  }
  v6 = 0;
  return v6 & 0xffffffff;
}


// Function: sub_5ce0 @ 0x5ce0
unsigned long sub_5ce0(unsigned long a0,unsigned int a1,long a2,long *a3) // return-dupe
{
  char *v1;
  long v2; // rax
  void *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char v6 [8200];
  char *v7;
  int v8; // r12d
  
  if (!a2)
    return 0;
  do {
    v2 = sub_b540(a1,v6,_DT_INIT);
    if (!v2)
      return 0xffffffff;
    if (v2 == -1) {
      v4 = sub_ae30(4,a0);
      v5 = dcgettext(NULL,"error reading %s",5);
      error(0,*__errno_location(),v5,v4);
      return 1;
    }
    *a3 = *a3 + v2;
    v8 = (int)dat_13114;
    v1 = &v6[v2];
    v7 = v6;
    while (v3 = memchr(v7,v8,(long)v1 - (long)v7), v3) {
      v7 = (char *)((long)v3 + 1);
      a2 -= 1;
      if (!a2) {
        if (v1 <= v7)
          return 0;
        if (!((long)v1 - (long)v7))
          return 0;
        sub_5c60(v7,(long)v1 - (long)v7);
        return 0;
      }
    }
  } while( true );
}


// Function: sub_5e30 @ 0x5e30
unsigned long sub_5e30(unsigned long a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  
  if (!a2)
    return 0;
  do {
    v1 = sub_b540(a1,v4,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == 0xffffffffffffffff) {
      v2 = sub_ae30(4,a0);
      v3 = dcgettext(NULL,"error reading %s",5);
      error(0,*__errno_location(),v3,v2);
      return 1;
    }
    *a3 = *a3 + v1;
    if (a2 < v1) {
      if (v1 - a2) {
        sub_5c60(&v4[a2],v1 - a2);
        return 0;
      }
      return 0;
    }
    a2 -= v1;
  } while (a2);
  return 0;
}


// Function: sub_6020 @ 0x6020
int * sub_6020(char a0,unsigned long a1,unsigned int a2,void *a3)
{
  int *v1;
  unsigned long v2;
  int *v3; // rax
  void *v4;
  char *v5; // rdx
  void *v6;
  char v7 [8200];
  long v8; // r12
  
  v8 = 0;
  v6 = a3;
  do {
    v4 = _DT_INIT;
    if (v6 <= (void *)0x2000)
      v4 = v6;
    v3 = (long)sub_b540(a2,v7,v4);
    if (v3 == (int *)0xffffffffffffffff) {
      v3 = __errno_location();
      v1 = v3;
      if (*v3 == 0xb) {
        v3 = (int *)v8;
        return v3;
      }
      v3 = (unsigned long)sub_ae30(4,a1);
      v2 = v3;
      v3 = (char *)dcgettext(NULL,"error reading %s",5);
      error(1,*v1,(char *)v3,v2);
      return v3;
    }
    if (!v3) {
      v3 = (int *)v8;
      return v3;
    }
    if (a0) {
      v5 = "\n";
      if (dat_13010)
        v5 = "";
      __printf_chk(1,"%s==> %s <==\n",v5,a1);
      dat_13010 = '\0';
    }
    v8 += (long)v3;
    sub_5c60(v7,v3);
    if (a3 != (void *)0xffffffffffffffff) {
      v6 = &v6[-(long)v3];
      if (!v6) {
        v3 = (int *)v8;
        return v3;
      }
      if (a3 == (void *)0xfffffffffffffffe) {
        v3 = (int *)v8;
        return v3;
      }
    }
    a0 = '\0';
  } while( true );
}


// Function: sub_6190 @ 0x6190
unsigned long sub_6190(unsigned long a0,unsigned int a1,long a2,long a3,long a4,long *a5) // return-dupe
{
  long v1;
  char v10 [8200];
  long v11; // rax
  long v12; // stack - 0x2050
  void *v2; // rax
  long v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  long v7;
  int v8; // ebx
  void *v9;
  
  if (!a2)
    return 1;
  v2 = (void *)((a4 - a3) % 0x2000);
  v9 = _DT_INIT;
  if (v2)
    v9 = v2;
  v11 = a4 - (long)v9;
  v12 = a4;
  if ((long)lseek(a1,v11,0) < 0)
    sub_5f40(v11,0,a0); // no-return
  v3 = sub_b540(a1,v10,v9);
  if (v3 != -1) {
    v8 = (int)dat_13114;
    *a5 = v11 + v3;
    if ((v3) && (v10[v3 + -1] != dat_13114))
      a2 -= 1;
    do {
      v7 = v3;
      while ((v7 && (v4 = memrchr(v10,v8), v4))) {
        v1 = a2 + -1;
        v7 = v4 - (long)v10;
        if (!a2) {
          if (v3 + -1 != v7)
            sub_5c60(v4 + 1);
          v11 = sub_6020(0,a0,a1,(v12 - v3) - v11);
          *a5 = *a5 + v11;
          return 1;
        }
        a2 = v1;
      }
      if (v11 == a3) {
        if (0 <= (long)lseek(a1,v11,0)) {
          *a5 = sub_6020(0,a0,a1,v12) + v11;
          return 1;
        }
        sub_5f40(v11,0,a0); // return-dupe, no-return
      }
      v11 -= 0x2000;
      if ((long)lseek(a1,v11,0) < 0) {
        sub_5f40(v11,0,a0);
      }
      v3 = sub_b540(a1,v10,_DT_INIT);
      if (v3 == -1) break;
      *a5 = v11 + v3;
      if (!v3)
        return 1;
      v8 = (int)dat_13114;
    } while( true );
  }
  v5 = sub_ae30(4,a0);
  v6 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v6,v5);
  return 0;
}


// Function: sub_6430 @ 0x6430
void sub_6430(unsigned long *a0,unsigned long *a1) // return-dupe x3
{
  unsigned int v1;
  long v10; // stack - 0x70
  long v11; // stack - 0x68
  int v2;
  long v3; // rax
  char *v4;
  int *v5; // rax
  unsigned long v6; // rax
  char v7 [48];
  bool v8;
  long v9; // stack - 0x98
  
  v2 = *(int *)&a0[7];
  if (v2 == -1)
    return;
  v4 = (char *)*a0;
  if (!strcmp(v4,"-")) {
    v4 = dcgettext(NULL,"standard input",5);
    v2 = *(int *)&a0[7];
  }
  if (fstat(v2,v7)) {
    v5 = __errno_location();
    *(int *)((long)a0 + 0x3c) = *v5;
    sub_5bd0(*(unsigned int *)&a0[7],v4);
    *(unsigned int *)&a0[7] = 0xffffffff;
    return;
  }
  if ((*(unsigned int *)&a0[6] & 0xf000) == 0x8000) {
    if ((long)a0[1] <= v9) { // branch-flip
      if ((v9 == a0[1]) && (!(((unsigned int)(v11 < (long)a0[3]) - (unsigned int)((long)a0[3] < v11)) + ((unsigned int)(v10 < (long)a0[2]) - (unsigned int)((long)a0[2] < v10)) * 2)))
        return;
    }
    else {
      v6 = sub_b1b0(0,3,v4);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v6);
      v1 = *(unsigned int *)&a0[7];
      if ((long)lseek(v1,0,0) < 0)
        sub_5f40(0,0,v4); // no-return
      a0[1] = 0;
    }
  }
  v8 = 0;
  if (dat_13115)
    v8 = (unsigned long *)*a1 != a0;
  v3 = sub_6020(v8,v4,*(unsigned int *)&a0[7],0xffffffffffffffff);
  a0[1] = a0[1] + v3;
  if (!v3)
    return;
  *a1 = a0;
  v2 = fflush_unlocked(stdout);
  if (!v2)
    return;
  v4 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v4);
}


// Function: sub_6660 @ 0x6660
void sub_6660(unsigned long *a0,unsigned int a1)
{
  char v1;
  char *v10;
  unsigned long v11; // rax
  char *v12;
  long v13; // stack - 0xd8
  long v14; // stack - 0xd0
  unsigned int v15; // stack - 0xc0
  unsigned long v16; // stack - 0x80
  unsigned long v17; // stack - 0x78
  unsigned int v2;
  int v3;
  char v4;
  int v5; // eax
  int v6; // eax
  unsigned int v7;
  int v8; // eax
  unsigned int *v9; // rax
  
  v10 = (char *)*a0;
  v5 = strcmp(v10,"-");
  v2 = *(unsigned int *)((long)a0 + 0x3c);
  v4 = *(char *)((long)a0 + 0x36);
  if (v5) { // branch-flip
    v6 = sub_7dd0(v10,((a1 ^ 1) & 0xff) << 0xb);
    v7 = *(unsigned int *)((long)a0 + 0x3c);
  }
  else {
    v6 = 0;
    v7 = v2;
  }
  v1 = dat_1311a;
  if ((*(int *)&a0[7] == -1) == (v7 == 0))
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1311a) { // branch-flip
    if (v6 != -1) { // branch-flip
      *(char *)((long)a0 + 0x36) = 1;
      if ((dat_1310c) || ((v10 = (char *)*a0, lstat(v10,&v13) || ((v15 & 0xf000) != 0xa000)))) goto label_67fb;
    }
    else {
      *(char *)((long)a0 + 0x36) = 0;
      v10 = (char *)*a0;
      if (dat_1310c) {
        *(int *)((long)a0 + 0x3c) = *__errno_location();
label_6991:
        v5 = strcmp(v10,"-");
        if (v4) {
          if (!v5)
            v10 = dcgettext(NULL,"standard input",5);
          v11 = sub_ae30(4,v10);
          v10 = dcgettext(NULL,"%s has become inaccessible",5);
          error(0,*(int *)((long)a0 + 0x3c),v10,v11);
          v10 = (char *)*a0;
          goto label_675a;
        }
        goto label_69a7;
      }
      if ((lstat(v10,&v13)) || ((v15 & 0xf000) != 0xa000)) goto label_6714;
    }
label_6cd0:
    v10 = (char *)*a0;
    *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
    *(char *)((long)a0 + 0x34) = 1;
    if (!strcmp(v10,"-"))
      v10 = dcgettext(NULL,"standard input",5);
    v11 = sub_ae30(4,v10);
    error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v11);
    v10 = (char *)*a0;
  }
  else {
    *(char *)((long)a0 + 0x36) = 1;
    if ((!dat_1310c) && ((v10 = (char *)*a0, !lstat(v10,&v13) && ((v15 & 0xf000) == 0xa000)))) goto label_6cd0;
    if (v6 != -1) {
label_67fb:
      if (fstat(v6,&v13) < 0) goto label_6714;
      if (((v15 & 0xf000) - 0x1000 & 0xffffe000) && ((v15 & 0xb000) != 0x8000)) {
        *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
        *(char *)((long)a0 + 0x36) = 0;
        if ((v1) && (dat_13020 == 1)) {
          *(char *)((long)a0 + 0x34) = 0;
          if (v4) // branch-flip
            v10 = "";
          else {
            v10 = "";
            if (v2 == 0xffffffff) goto label_69a7;
          }
        }
        else {
          *(char *)((long)a0 + 0x34) = 1;
          if ((!v4) && (v2 == 0xffffffff)) goto label_69a7;
          v10 = dcgettext(NULL,"; giving up on this name",5);
        }
        v12 = (char *)*a0;
        if (!strcmp(v12,"-"))
          v12 = dcgettext(NULL,"standard input",5);
        v11 = sub_ae30(4,v12);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v11,v10);
      }
      else {
        v10 = (char *)*a0;
        if (!strcmp(v10,"-"))
          v10 = dcgettext(NULL,"standard input",5);
        v4 = sub_5460(v6,v10);
        *(char *)((long)a0 + 0x35) = v4;
        if ((!v4) || (dat_1310c)) {
          *(unsigned int *)((long)a0 + 0x3c) = 0;
          v3 = *(int *)&a0[7];
          if (v2 & 0xfffffffd) { // branch-flip
            if (v3 != -1)
              __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
            v10 = (char *)*a0;
            if (!strcmp(v10,"-"))
              v10 = dcgettext(NULL,"standard input",5);
            v11 = sub_ae30(4,v10);
            v10 = "%s has become accessible";
          }
          else {
            v10 = (char *)*a0;
            v8 = strcmp(v10,"-");
            if (v3 != -1) {
              if ((a0[5] == v14) && (a0[4] == v13)) {
                if (!v8)
                  v10 = dcgettext(NULL,"standard input",5);
                sub_5bd0(v6,v10);
                return;
              }
              if (!v8)
                v10 = dcgettext(NULL,"standard input",5);
              v11 = sub_ae30(4,v10);
              error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v11);
              v10 = (char *)*a0;
              if (!strcmp(v10,"-"))
                v10 = dcgettext(NULL,"standard input",5);
              sub_5bd0(*(unsigned int *)&a0[7],v10);
              goto label_68e3;
            }
            if (!v8)
              v10 = dcgettext(NULL,"standard input",5);
            v11 = sub_ae30(4,v10);
            v10 = "%s has appeared;  following new file";
          }
          error(0,0,dcgettext(NULL,v10,5),v11);
label_68e3:
          a1 &= 0xff;
          *(int *)&a0[7] = v6;
          if (!v5)
            a1 = 0xffffffff;
          *(char *)((long)a0 + 0x34) = 0;
          a0[1] = 0;
          a0[2] = v16;
          *(unsigned int *)&a0[8] = a1;
          v10 = (char *)*a0;
          a0[3] = v17;
          a0[0xb] = 0;
          a0[4] = v13;
          a0[5] = v14;
          *(unsigned int *)&a0[6] = v15;
          if (!strcmp(v10,"-"))
            v10 = dcgettext(NULL,"standard input",5);
          if (0 <= (long)lseek(v6,0,0))
            return;
          sub_5f40(0,0,v10); // no-return
        }
        v10 = (char *)*a0;
        *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
        if (!strcmp(v10,"-"))
          v10 = dcgettext(NULL,"standard input",5);
        v11 = sub_ae30(4,v10);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v11);
        *(unsigned short *)((long)a0 + 0x34) = 0x101;
      }
label_69a7:
      v10 = (char *)*a0;
      v5 = strcmp(v10,"-");
      goto label_69b7;
    }
label_6714:
    v1 = *(char *)((long)a0 + 0x36);
    v9 = (unsigned int *)__errno_location();
    v10 = (char *)*a0;
    v7 = *v9;
    *(unsigned int *)((long)a0 + 0x3c) = v7;
    if (!v1) goto label_6991;
    v5 = strcmp(v10,"-");
    if (v7 != v2) {
      if (!v5)
        v10 = dcgettext(NULL,"standard input",5);
      v11 = sub_b1b0(0,3,v10);
      error(0,*v9,"%s",v11);
      v10 = (char *)*a0;
    }
  }
label_675a:
  v5 = strcmp(v10,"-");
label_69b7:
  if (!v5)
    v10 = dcgettext(NULL,"standard input",5);
  sub_5bd0(v6,v10);
  v10 = (char *)*a0;
  if (!strcmp(v10,"-"))
    v10 = dcgettext(NULL,"standard input",5);
  sub_5bd0(*(unsigned int *)&a0[7],v10);
  *(unsigned int *)&a0[7] = 0xffffffff;
}


// Function: sub_6f70 @ 0x6f70
void sub_6f70(int a0)
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
  
  v2 = dat_13138;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v2);
    __printf_chk(1,dcgettext(NULL,"Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n",5),10);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n",5),10,5);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -z, --zero-terminated    line delimiter is NUL, not newline\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n",5),v1);
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
      if (!strcmp("tail",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "tail";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "tail";
    if (!strcmp("tail","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "tail")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}


// Function: sub_7e50 @ 0x7e50
unsigned long sub_7e50(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}


// Function: sub_7e70 @ 0x7e70
bool sub_7e70(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_7e80 @ 0x7e80
unsigned long sub_7e80(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0xe590)
    return 1;
  v1 = v2[2];
  if ((((dat_e5a4 < v1) && (v1 < dat_e5a8)) && (dat_e5ac < v2[3])) && (dat_e5c0 <= *v2)) {
    v3 = *v2 + dat_e5a4;
    if (((v3 < v2[1]) && (v2[1] <= dat_e5b0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0xe590;
  return 0;
}


// Function: sub_7f10 @ 0x7f10
unsigned long sub_7f10(long *a0,long a1,unsigned long *a2,char a3) // early-return
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


// Function: sub_8160 @ 0x8160
unsigned long sub_8160(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_8196:
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
        goto label_8196;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}


// Function: sub_83d0 @ 0x83d0
void sub_83d0(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_843d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_843d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_8530 @ 0x8530
long sub_8530(long *a0,long a1) // return-dupe
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


// Function: sub_85f0 @ 0x85f0
long sub_85f0(long *a0,long a1)
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


// Function: sub_86d0 @ 0x86d0
long sub_86d0(unsigned long *a0,void *a1,unsigned long a2)
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


// Function: sub_87c0 @ 0x87c0
unsigned long * sub_87c0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_7e50;
  if (!a3)
    a3 = sub_7e70;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0xe590;
  v1[5] = a1;
  if (sub_7e80(v1)) {
    v2 = sub_8020(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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


// Function: sub_88d0 @ 0x88d0
void sub_88d0(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_896b;
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
label_896b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_8980 @ 0x8980
void sub_8980(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_8a2c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_89ef;
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
label_89ef:
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
label_8a2c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_8a70 @ 0x8a70
unsigned int sub_8a70(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_8020(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_8160(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_8160(a0,&v2,1)) && (sub_8160(a0,&v2,0))) {
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


// Function: sub_8bf0 @ 0x8bf0
long * sub_8bf0(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_7f10(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_8c7f;
label_8d06:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_8d06;
label_8c7f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_7e80(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_e5b4 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_e5b8 <= v5) ? (long)(v5 - dat_e5b8) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_8a70(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_7f10(a0,a1,&v3,0);
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


// Function: sub_8e20 @ 0x8e20
unsigned long sub_8e20(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_8bf0(a0,a1,&v3);
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


// Function: sub_8e80 @ 0x8e80
long sub_8e80(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_7f10(a0,a1,&v7,1);
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
      sub_7e80(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_e5b8 <= v8) ? (long)(v8 - dat_e5b8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_8a70(a0,v6)) {
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

