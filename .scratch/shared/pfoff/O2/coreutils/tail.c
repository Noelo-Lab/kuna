// Function: main @ 0x2960
int main(int argc,char **argv,char **envp)
{
  unsigned char *v1;
  char v10;
  int v11;
  int v12;
  int v13;
  int v14; // eax
  int v15; // eax
  int v16; // eax
  unsigned int v17; // eax
  int v18; // eax
  int v19; // eax
  char **v2;
  int v20; // eax
  int v21; // eax
  int v22; // eax
  int v23; // eax
  int v24; // eax
  int v25; // eax
  int v26; // eax
  long *v27; // rax
  long v28; // rax
  void *v29;
  unsigned char v3;
  unsigned long v30;
  void *v31;
  double v32;
  double v33;
  unsigned char *v34;
  char *v35;
  int *v36;
  long v37;
  long *v38;
  unsigned int *v39; // rax
  char v4;
  unsigned long v40; // rax
  double v41;
  unsigned long v42;
  unsigned int v43; // edx
  unsigned int v44;
  char **v45;
  unsigned char *v46;
  char *v47;
  char v48;
  int v49;
  unsigned int v5;
  long v50;
  long *v51;
  double v52 [8]; // stack - 0x1c8
  stat v53; // stack - 0xd8
  double v54; // stack - 0x1d8
  stat v55; // stack - 0x168
  long *v56; // stack - 0x1d0
  void *v57; // rbp
  unsigned long v58; // r10
  unsigned char v59;
  long v6;
  char **v60;
  void *v61;
  bool v62;
  unsigned char v63;
  long v64;
  long *v65; // stack - 0x260
  void *v66; // stack - 0x258
  char *v67; // stack - 0x250
  void *v68; // stack - 0x248
  double v69; // stack - 0x240
  double v7;
  double v70; // stack - 0x238
  unsigned long v71; // stack - 0x230
  void *v72; // stack - 0x228
  long *v73; // stack - 0x220
  unsigned int v74; // stack - 0x210
  unsigned char v75; // stack - 0x209
  double v76; // stack - 0x208
  int v77; // stack - 0x184
  long v78; // stack - 0x40
  char v8;
  char v9;
  
  v78 = *(long *)(v64 + 0x28);
  v54 = 4.94065645841247e-323;
  sub_9150(*argv);
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
  if (argc != 2) { // branch-flip
    if (argc == 3) {
      v35 = argv[2];
      if ((*v35 == '-') && (v35[1])) goto label_2a20;
      goto label_2a30;
    }
    if (argc == 4) {
      v35 = argv[2];
label_2a20:
      if (!strcmp(v35,"--")) goto label_2a30;
    }
label_2a52:
    v28 = 0;
  }
  else {
label_2a30:
    v11 = sub_90c0();
    v35 = argv[1];
    v1 = (unsigned char *)&v35[1];
    if (*v35 == '+') {
      v59 = 1;
      if (0x2b8 < (unsigned int)(v11 - 0x30db0U)) goto label_38a5;
      goto label_2a52;
    }
    if ((*v35 != '-') || ((0x30db0 <= v11 && (!v35[(unsigned long)(v35[1] == 'c') + 1])))) goto label_2a52;
    v59 = 0;
label_38a5:
    v63 = v35[1];
    v46 = v1;
    while ((unsigned int)((int)(char)v63 - 0x30U) <= 9) {
      v34 = &v46[1];
      v46 = &v46[1];
      v63 = *v34;
    }
    if (v63 != 99) { // branch-flip
      if (v63 == 0x6c) {
        v63 = 1;
        v41 = 4.94065645841247e-323;
        goto label_41b9;
      }
      if (v63 == 0x62) {
        v63 = 0;
        v41 = 2.52961610670718e-320;
        goto label_41b9;
      }
      v63 = 1;
      v41 = 4.94065645841247e-323;
      v34 = v46;
    }
    else {
      v63 = 0;
      v41 = 4.94065645841247e-323;
label_41b9:
      v34 = &v46[1];
    }
    v3 = *v34;
    if (v3 == 0x66)
      v34 = &v34[1];
    if (*v34) goto label_2a52;
    if (v1 != v46) { // branch-flip
      if (sub_c520(v1,NULL,10,&v54,"b") & 0xfffffffd) {
        v35 = (char *)sub_b520(argv[1]);
        v47 = dcgettext(NULL,"invalid number",5);
        error(1,*__errno_location(),"%s: %s",v47,v35);
        return v22;
      }
    }
    else {
      v54 = v41;
    }
    v28 = 1;
    dat_13116 = v59;
    dat_13118 = v3 == 0x66;
    dat_13119 = v63;
  }
  v11 = 0;
  argv = &argv[v28];
  argc -= (int)v28;
  v76 = dat_e4a0;
label_2a90:
  v12 = getopt_long(argc,argv,"c:n:fFqs:vz0123456789",(option *)0x12900,NULL);
  if (v12 != -1) {
    if (0x85 < v12) {
label_42ba:
      sub_6f70(1); // no-return, return-dupe
    }
    if (v12 <= 0x2f) {
      if (v12 == -0x83) {
        sub_bb50(stdout,"tail","GNU coreutils",dat_13030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0,v58);
        exit(0); // no-return
      }
      if (v12 == -0x82)
        sub_6f70(0); // no-return
      sub_6f70(1);
    }
    switch(v12) {
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
        error(1,0,dcgettext(NULL,"option used in invalid context -- %c",5),(char)v12);
        return v15;
      default:
        goto label_42ba;
      case 0x46:
        dat_13118 = '\x01';
        dat_13020 = 1;
        dat_1311a = 1;
        break;
      case 99:
      case 0x6e:
        dat_13119 = v12 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_13116 = 1;
        }
        if (v12 != 0x6e) // branch-flip
          v35 = dcgettext(NULL,"invalid number of bytes",5);
        else {
          v35 = dcgettext(NULL,"invalid number of lines",5);
        }
        v54 = (double)sub_c370(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v35,0);
        break;
      case 0x66:
      case 0x84:
        dat_13118 = '\x01';
        if (optarg) // branch-flip
          dat_13020 = *(int *)(sub_7710("--follow",optarg,0x12b00,0xe498,4,dat_13038,1) * 4 + 0xe498);
        else {
          dat_13020 = 2;
        }
        break;
      case 0x71:
label_2cf0:
        v11 = 2;
        break;
      case 0x73:
        v9 = sub_c450(optarg,0,v52,sub_78d0);
        if (v9) {
          v76 = v52[0];
          if (0.0 <= v52[0]) break;
        }
        argv = (char **)sub_b520(optarg);
        error(1,0,dcgettext(NULL,"invalid number of seconds: %s",5),argv);
        goto label_2cf0;
      case 0x76:
        v11 = 1;
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
      v35 = "warning: --retry ignored; --retry is useful only when following";
label_3152:
      error(0,0,dcgettext(NULL,v35,5));
      goto label_3167;
    }
    if (dat_13020 == 2) {
      v35 = "warning: --retry only effective for the initial open";
      goto label_3152;
    }
    if (dat_13110) goto label_2da1;
  }
  else {
label_3167:
    if (dat_13110) {
      if (dat_13118) { // branch-flip
label_2da1:
        if ((kill(dat_13110,0)) && (*__errno_location() == 0x26)) {
          error(0,0,dcgettext(NULL,"warning: --pid=PID is not supported on this system",5));
          dat_13110 = 0;
        }
      }
      else {
        error(0,0,dcgettext(NULL,"warning: PID ignored; --pid=PID is useful only when following",5));
      }
    }
  }
  if ((dat_13116) && (v54 != 0.0))
    v54 = (double)((long)v54 - 1);
  v45 = (char **)0x13028;
  v71 = 1;
  if (optind < argc) {
    v45 = &argv[optind];
    v71 = (unsigned long)(argc - optind);
  }
  v62 = 0;
  v2 = &v45[v71];
  v60 = v45;
  do {
    v35 = *v60;
    if (!strcmp(v35,"-"))
      v62 = 1;
    v60 = &v60[1];
  } while (v60 != v2);
  if (v62) {
    if (dat_13020 == 1) {
      v35 = (char *)sub_ae30(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v35);
      return v21;
    }
    if (dat_13118) {
      if (((((dat_13110) || (v71 != 1)) || (dat_13020 != 2)) || ((fstat(0,&v53) || ((v53._24_4_ & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2e85;
    }
    if (v54 == 0.0) goto label_3d8d;
  }
  else {
label_2e85:
    if ((v54 == 0.0) && (!dat_13118)) {
label_3d8d:
      v44 = 0;
      if (!dat_13116) goto label_3381;
    }
  }
  v27 = (long *)sub_be60(v71,0x60);
  v38 = v27;
  do {
    v35 = *v45;
    v45 = &v45[1];
    v51 = &v38[0xc];
    *v38 = (long)v35;
    v38 = v51;
  } while (v45 != v2);
  if ((v11 == 1) || ((!v11 && (v71 != 1))))
    dat_13115 = '\x01';
  v75 = 1;
  v65 = v27;
  v28 = v71 * 0x60;
  v38 = &v27[v71 * 0xc];
  do {
    v41 = v54;
    v69 = v54;
    v67 = (char *)*v65;
    v11 = strcmp(v67,"-");
    if (v11) { // branch-flip
      v12 = sub_7dd0(v67,0);
      v59 = dat_1311a;
      if (dat_1311a) { // branch-flip
        v67 = (char *)*v65;
        if (v12 != -1) goto label_2f60;
        *(char *)((long)v65 + 0x36) = 0;
      }
      else {
        v67 = (char *)*v65;
        *(char *)((long)v65 + 0x36) = 1;
        if (v12 != -1) goto label_2f69;
      }
      v36 = __errno_location();
      if (dat_13118) {
        v11 = *v36;
        *(unsigned int *)&v65[7] = 0xffffffff;
        *(int *)((long)v65 + 0x3c) = v11;
        *(unsigned char *)((long)v65 + 0x34) = v59 ^ 1;
        v65[5] = 0;
        v65[4] = 0;
      }
      if (!strcmp(v67,"-"))
        v67 = dcgettext(NULL,"standard input",5);
      v35 = (char *)sub_ae30(4,v67);
      v47 = dcgettext(NULL,"cannot open %s for reading",5);
      error(0,*v36,v47,v35);
      v75 = 0;
    }
    else {
      dat_1310e = '\x01';
      v12 = 0;
label_2f60:
      *(char *)((long)v65 + 0x36) = 1;
label_2f69:
      if (dat_13115) {
        if (!strcmp(v67,"-"))
          v67 = dcgettext(NULL,"standard input",5);
        v35 = "\n";
        if (dat_13010)
          v35 = "";
        __printf_chk(1,"%s==> %s <==\n",v35,v67);
        dat_13010 = '\0';
        v67 = (char *)*v65;
      }
      if (!strcmp(v67,"-"))
        v67 = dcgettext(NULL,"standard input",5);
      v59 = dat_13119;
      v52[0] = 0.0;
      if (dat_13119) { // branch-flip
        if (fstat(v12,&v53)) { // branch-flip
label_3590:
          v40 = sub_ae30(4,v67);
          v35 = dcgettext(NULL,"cannot fstat %s",5);
          error(0,*__errno_location(),v35,v40);
          v74 = 0;
        }
        else {
          v74 = (unsigned int)dat_13116;
          if (dat_13116) {
            v44 = sub_5ce0(v67,v12,v41,v52);
            if (v44) { // branch-flip
label_3df1:
              v74 = v44 >> 0x1f;
            }
            else {
              v52[0] = (double)((long)v52[0] + sub_6020(0,v67,v12,0xffffffffffffffff));
            }
            goto label_35da;
          }
          if (((!dat_1310d) && ((v53._24_4_ & 0xf000) == 0x8000)) && (v30 = lseek(v12,0,1), v30 != 0xffffffffffffffff)) {
            v32 = (double)lseek(v12,0,2);
            if ((long)v30 < (long)v32) {
              v52[0] = v32;
              v74 = (unsigned int)v59;
              if (v32 != 0.0)
                v74 = (unsigned int)(unsigned char)sub_6190(v67,v12,v41,v30,v32,v52);
              goto label_35da;
            }
            if ((long)lseek(v12,v30,0) <= -1)
              sub_5f40(v30,0,v67); // no-return
          }
          v68 = (void *)sub_bd20(0x2018);
          *(unsigned long *)((long)v68 + 0x2008) = 0;
          *(unsigned long *)((long)v68 + 0x2000) = 0;
          *(unsigned long *)((long)v68 + 0x2010) = 0;
          v29 = (void *)sub_bd20(0x2018);
          v72 = v68;
          v70 = 0.0;
          v31 = v29;
          while( true ) {
            v61 = v31;
            v30 = sub_b540(v12,v29,0x2000);
            if (v30 - 1 > 0xfffffffffffffffd) break;
            *(unsigned long *)((long)v61 + 0x2000) = v30;
            v49 = (int)dat_13114;
            v52[0] = (double)((long)v52[0] + v30);
            *(unsigned long *)((long)v61 + 0x2008) = 0;
            *(unsigned long *)((long)v61 + 0x2010) = 0;
            v31 = v29;
            while (v31 = memchr(v31,v49,(long)v29 + (v30 - (long)v31)), v31) {
              *(long *)((long)v61 + 0x2008) = *(long *)((long)v61 + 0x2008) + 1;
              v31 = (void *)((long)v31 + 1);
            }
            v70 = (double)((long)v70 + *(long *)((long)v61 + 0x2008));
            if (0x2000 <= v30 + *(long *)((long)v68 + 0x2000)) { // branch-flip
              *(void **)((long)v68 + 0x2010) = v61;
              v32 = (double)((long)v70 - *(long *)((long)v72 + 0x2008));
              v68 = v61;
              if ((unsigned long)v32 <= (unsigned long)v41) { // branch-flip
                v29 = (void *)sub_bd20(0x2018);
                v31 = v29;
              }
              else {
                v29 = v72;
                v31 = v72;
                v70 = v32;
                v72 = *(void **)((long)v72 + 0x2010);
              }
            }
            else {
              memcpy((void *)(*(long *)((long)v68 + 0x2000) + (long)v68),v29,v30);
              *(long *)((long)v68 + 0x2000) = *(long *)((long)v68 + 0x2000) + *(long *)((long)v61 + 0x2000);
              *(long *)((long)v68 + 0x2008) = *(long *)((long)v68 + 0x2008) + *(long *)((long)v61 + 0x2008);
              v31 = v61;
            }
          }
          free(v61);
          if (v30 != 0xffffffffffffffff) { // branch-flip
            v50 = *(long *)((long)v68 + 0x2000);
            v74 = (unsigned int)(!v50 || v41 == 0.0);
            if (!v50 || v41 == 0.0) goto label_34b9;
            v49 = (int)dat_13114;
            if (*(char *)((long)v68 + v50 + -1) != dat_13114) {
              *(long *)((long)v68 + 0x2008) = *(long *)((long)v68 + 0x2008) + 1;
              v70 = (double)((long)v70 + 1);
            }
            v32 = (double)((long)v70 - *(long *)((long)v72 + 0x2008));
            v29 = v72;
            if ((unsigned long)v41 < (unsigned long)v32) {
              do {
                v70 = v32;
                v29 = *(void **)((long)v29 + 0x2010);
                v32 = (double)((long)v70 - *(long *)((long)v29 + 0x2008));
              } while ((unsigned long)v41 < (unsigned long)v32);
            }
            v50 = *(long *)((long)v29 + 0x2000);
            v31 = v29;
            if ((unsigned long)v41 < (unsigned long)v70) {
              v37 = (long)v70 - (long)v41;
              do {
                v31 = (void *)((long)rawmemchr(v31,v49) + 1);
                v37 -= 1;
              } while (v37);
            }
            v30 = (long)v29 + (v50 - (long)v31);
            if (v30)
              sub_5c60(v31,v30);
            for (v50 = *(long *)((long)v29 + 0x2010); v50; v50 = *(long *)(v50 + 0x2010)) {
              if (*(unsigned long *)(v50 + 0x2000))
                sub_5c60(v50,*(unsigned long *)(v50 + 0x2000));
            }
            v74 = (unsigned int)v59;
          }
          else {
            v40 = sub_ae30(4,v67);
            v35 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v35,v40);
label_34b9:
            if (!v72) goto label_35da;
          }
          do {
            v29 = *(void **)((long)v72 + 0x2010);
            free(v72);
            v72 = v29;
          } while (v29);
        }
      }
      else {
        if (fstat(v12,&v53)) goto label_3590;
        v74 = (unsigned int)dat_13116;
        if (dat_13116) { // branch-flip
          if ((dat_1310d) || ((long)v41 <= -1)) {
label_3a3e:
            v44 = sub_5e30(v67,v12,v41,v52);
            v69 = -NAN;
            if (v44) goto label_3df1;
          }
          else {
            if ((v53._24_4_ & 0xf000) != 0x8000) { // branch-flip
              if (lseek(v12,v41,1) == -1) goto label_3a3e;
            }
            else if ((long)lseek(v12,v41,1) <= -1)
              sub_5f40(v41,1,v67); // no-return
            v69 = -NAN;
            v52[0] = v41;
          }
label_3a6a:
          v50 = sub_6020(0,v67,v12,v69);
          v74 = 1;
          v52[0] = (double)((long)v52[0] + v50);
        }
        else {
          if ((!dat_1310d) && (0 <= (long)v41)) {
            if ((v53._24_4_ & 0xd000) != 0x8000) { // branch-flip
              v32 = (double)lseek(v12,-(long)v41,2);
              if (v32 == -NAN) goto label_3678;
              v50 = (long)v41 + (long)v32;
            }
            else {
              v32 = -NAN;
              v50 = v53._48_8_;
            }
            v37 = v53._56_8_;
            if (0x2000000000000000 <= (unsigned long)(v53._56_8_ - 1U))
              v37 = 0x200;
            if (v37 < v50) {
              if ((v32 == -NAN) && (v32 = (double)lseek(v12,0,1), (long)v32 <= -1))
                sub_5f40(0,1,v67); // no-return
              if (((long)v32 < v50) && ((unsigned long)v41 < (unsigned long)(v50 - (long)v32))) {
                v32 = (double)(v50 - (long)v41);
                if ((long)lseek(v12,v32,0) <= -1)
                  sub_5f40(v32,0,v67); // no-return
              }
              v52[0] = v32;
              goto label_3a6a;
            }
          }
label_3678:
          v29 = (void *)sub_bd20(0x2010);
          *(unsigned long *)((long)v29 + 0x2000) = 0;
          *(unsigned long *)((long)v29 + 0x2008) = 0;
          v31 = (void *)sub_bd20(0x2010);
          v61 = v29;
          v32 = 0.0;
          while( true ) {
            v57 = v31;
            v31 = v29;
            v30 = sub_b540(v12,v57,0x2000);
            if (v30 - 1 > 0xfffffffffffffffd) break;
            v52[0] = (double)((long)v52[0] + v30);
            v32 = (double)((long)v32 + v30);
            *(unsigned long *)((long)v57 + 0x2000) = v30;
            v50 = *(long *)((long)v61 + 0x2000);
            *(unsigned long *)((long)v57 + 0x2008) = 0;
            v29 = v31; // branch-flip
            if (0x2000 <= v30 + v50) {
              *(void **)((long)v61 + 0x2008) = v57;
              v33 = (double)((long)v32 - *(long *)((long)v31 + 0x2000));
              v61 = v57;
              if ((unsigned long)v33 <= (unsigned long)v41) // branch-flip
                v31 = (void *)sub_bd20(0x2010);
              else {
                v29 = *(void **)((long)v31 + 0x2008);
                v32 = v33;
              }
            }
            else {
              memcpy((void *)((long)v61 + v50),v57,v30);
              *(long *)((long)v61 + 0x2000) = *(long *)((long)v61 + 0x2000) + *(long *)((long)v57 + 0x2000);
              v31 = v57;
            }
          }
          free(v57);
          if (v30 != 0xffffffffffffffff) { // branch-flip
            v50 = *(long *)((long)v31 + 0x2000);
            v33 = (double)((long)v32 - v50);
            v29 = v31;
            while (v7 = v33, (unsigned long)v41 < (unsigned long)v7) {
              v29 = *(void **)((long)v29 + 0x2008);
              v50 = *(long *)((long)v29 + 0x2000);
              v33 = (double)((long)v7 - v50);
              v32 = v7;
            }
            v37 = 0;
            if ((unsigned long)v41 < (unsigned long)v32)
              v37 = (long)v32 - (long)v41;
            if (v50 - v37)
              sub_5c60((void *)((long)v29 + v37),v50 - v37);
            while (v29 = *(void **)((long)v29 + 0x2008), v29) {
              if (*(unsigned long *)((long)v29 + 0x2000))
                sub_5c60(v29,*(unsigned long *)((long)v29 + 0x2000));
            }
            v74 = 1;
          }
          else {
            v40 = sub_ae30(4,v67);
            v35 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v35,v40);
            if (!v31) goto label_35da;
          }
          do {
            v29 = *(void **)((long)v31 + 0x2008);
            free(v31);
            v31 = v29;
          } while (v29);
        }
      }
label_35da:
      if (dat_13118) { // branch-flip
        *(unsigned int *)((long)v65 + 0x3c) = v74 - 1;
        if (0 <= fstat(v12,&v53)) { // branch-flip
          if (((v53._24_4_ & 0xf000) - 0x1000 & 0xffffe000) && ((v53._24_4_ & 0xb000) != 0x8000)) {
            v35 = "";
            *(unsigned int *)((long)v65 + 0x3c) = 0xffffffff;
            *(char *)((long)v65 + 0x36) = 0;
            *(unsigned char *)((long)v65 + 0x34) = dat_1311a ^ 1;
            if (dat_1311a ^ 1)
              v35 = dcgettext(NULL,"; giving up on this name",5);
            v47 = (char *)*v65;
            if (!strcmp(v47,"-"))
              v47 = dcgettext(NULL,"standard input",5);
            v47 = (char *)sub_b1b0(0,3,v47);
            error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v47,v35);
          }
          else if ((unsigned char)v74) {
            *(int *)&v65[7] = v12;
            v35 = (char *)*v65;
            *(unsigned int *)&v65[6] = v53._24_4_;
            v65[1] = (long)v52[0];
            *(unsigned int *)&v65[8] = -(unsigned int)(v11 == 0) | 1;
            v65[2] = v53._88_8_;
            v65[0xb] = 0;
            v65[3] = v53._96_8_;
            *(char *)((long)v65 + 0x34) = 0;
            v65[4] = CONCAT26(v53._6_2_,CONCAT24(v53._4_2_,v53._0_4_));
            v65[5] = v53._8_8_;
            if (!strcmp(v35,"-"))
              v35 = dcgettext(NULL,"standard input",5);
            *(char *)((long)v65 + 0x35) = sub_5460(v12,v35);
            goto label_3340;
          }
        }
        else {
          v36 = __errno_location();
          v35 = (char *)*v65;
          *(int *)((long)v65 + 0x3c) = *v36;
          if (!strcmp(v35,"-"))
            v35 = dcgettext(NULL,"standard input",5);
          v35 = (char *)sub_ae30(4,v35);
          v47 = dcgettext(NULL,"error reading %s",5);
          error(0,*v36,v47,v35);
        }
        v35 = (char *)*v65;
        *(unsigned char *)((long)v65 + 0x34) = dat_1311a ^ 1;
        if (!strcmp(v35,"-"))
          v35 = dcgettext(NULL,"standard input",5);
        sub_5bd0(v12,v35);
        v75 = 0;
        *(unsigned int *)&v65[7] = 0xffffffff;
      }
      else if ((v11) && (close(v12))) {
        v35 = (char *)*v65;
        if (!strcmp(v35,"-"))
          v35 = dcgettext(NULL,"standard input",5);
        v40 = sub_ae30(4,v35);
        v35 = dcgettext(NULL,"error reading %s",5);
        error(0,*__errno_location(),v35,v40);
        v75 = 0;
      }
      else {
        v75 &= (unsigned char)v74;
      }
    }
label_3340:
    v9 = dat_13118;
    v65 = &v65[0xc];
  } while (v38 != v65);
  if (dat_13118) {
    v50 = 0;
    v51 = v27;
    do {
      v35 = (char *)*v51;
      if ((((strcmp(v35,"-")) || (*(char *)((long)v51 + 0x34))) || ((int)v51[7] <= -1)) || ((*(unsigned int *)&v51[6] & 0xf000) != 0x1000))
        v50 += 1;
      else {
        *(unsigned int *)&v51[7] = 0xffffffff;
        *(char *)((long)v51 + 0x34) = 1;
      }
      v51 = &v51[0xc];
    } while (v38 != v51);
    if (v50) {
      if (fstat(1,&v55) < 0) {
        v35 = dcgettext(NULL,"standard output",5);
        error(1,*__errno_location(),v35);
        return v18;
      }
      dat_13117 = (v55._24_4_ & 0xf000) == 0x1000;
      if (!dat_1310c) {
        v51 = v27;
        do {
          if ((!*(char *)((long)v51 + 0x34)) && (v35 = (char *)*v51, !strcmp(v35,"-"))) goto label_3b98;
          v51 = &v51[0xc];
        } while (v38 != v51);
        v50 = 0;
        do {
          if ((0 <= *(int *)((long)v27 + v50 + 0x38)) && (*(char *)((long)v27 + v50 + 0x35))) goto label_3b98;
          v50 += 0x60;
        } while (v28 != v50);
        v50 = 0;
        do {
          if ((0 <= *(int *)((long)v27 + v50 + 0x38)) && (!*(char *)((long)v27 + v50 + 0x35))) {
            v48 = '\0';
            v51 = v27;
            goto label_484a;
          }
          v50 += 0x60;
        } while (v28 != v50);
      }
label_3b98:
      dat_1310c = '\x01';
      if (dat_13110) // branch-flip
        v44 = 0;
      else {
        v44 = 0;
        if (((dat_13020 == 2) && (v71 == 1)) && ((int)v27[7] != -1))
          v44 = (unsigned int)((*(unsigned int *)&v27[6] & 0xf000) != 0x8000);
      }
      v67 = (unsigned long)(v71 - 1);
      v48 = '\0';
label_3bf5:
      v62 = 0;
      v30 = 0;
      v38 = v27;
      do {
        while( true ) {
          if (*(char *)((long)v38 + 0x34)) goto label_3ca6;
          v11 = (int)v38[7];
          if (v11 <= -1) break;
          v35 = (char *)*v38;
          if (!strcmp(v35,"-"))
            v35 = dcgettext(NULL,"standard input",5);
          v5 = *(unsigned int *)&v38[6];
          v13 = *(unsigned int *)&v38[8];
          if (v44 != *(unsigned int *)&v38[8]) {
            v13 = sub_7b60(v11,3);
            if (v44) { // branch-flip
              if (0 <= v13) {
label_3c76:
                *(unsigned int *)&v38[8] = v44;
                v13 = v44;
                goto label_3c7c;
              }
            }
            else if ((0 <= v13) && ((v43 = v13 | 0x800, v13 == v43 || (sub_7b60(v11,4) != -1)))) goto label_3c76;
            v36 = __errno_location();
            if (((*(unsigned int *)&v38[6] & 0xf000) != 0x8000) || (*v36 != 1)) {
              v35 = (char *)sub_b1b0(0,3,v35);
              v47 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
              error(1,*v36,v47,v35);
              return v19;
            }
            v13 = *(unsigned int *)&v38[8];
          }
label_3c7c:
          v42 = 0xfffffffffffffffe;
          if (v13) { // branch-flip
label_3c8b:
            v50 = sub_6020(0,v35,v11,v42);
            v38[1] = v38[1] + v50;
            v62 = (bool)(v62 | v50 != 0);
          }
          else if (fstat(v11,&v53)) { // branch-flip
            *(unsigned int *)&v38[7] = 0xffffffff;
            v36 = __errno_location();
            *(int *)((long)v38 + 0x3c) = *v36;
            v35 = (char *)sub_b1b0(0,3,v35);
            error(0,*v36,"%s",v35);
            close(v11);
          }
          else {
            if ((*(unsigned int *)&v38[6] != v53._24_4_) || ((((v53._24_4_ & 0xf000) == 0x8000 && (v38[1] != v53._48_8_)) || (((unsigned int)(v53._96_8_ < v38[3]) - (unsigned int)(v38[3] < v53._96_8_)) + ((unsigned int)(v53._88_8_ < v38[2]) - (unsigned int)(v38[2] < v53._88_8_)) * 2)))) {
              *(unsigned int *)&v38[6] = v53._24_4_;
              v38[0xb] = 0;
              v38[2] = v53._88_8_;
              v38[3] = v53._96_8_;
              if (((v5 & 0xf000) == 0x8000) && (v53._48_8_ < v38[1])) {
                v47 = (char *)sub_b1b0(0,3,v35);
                error(0,0,dcgettext(NULL,"%s: file truncated",5),v47);
                if ((long)lseek(v11,0,0) < 0)
                  sub_5f40(0,0,v35); // no-return
                v38[1] = 0;
              }
              if (((char *)v30 != v67) && (dat_13115)) {
                v47 = "\n";
                if (dat_13010)
                  v47 = "";
                __printf_chk(1,"%s==> %s <==\n",v47,v35);
                dat_13010 = '\0';
              }
              v67 = (char *)v30;
              if ((int)v38[8]) // branch-flip
                v42 = 0xfffffffffffffffe;
              else {
                v42 = 0xffffffffffffffff;
                if (((v5 & 0xf000) == 0x8000) && (*(char *)((long)v38 + 0x35)))
                  v42 = v53._48_8_ - v38[1];
              }
              goto label_3c8b;
            }
            v42 = v38[0xb];
            v38[0xb] = v42 + 1;
            if ((dat_13018 <= v42) && (dat_13020 == 1)) {
              sub_6660(v38,(int)v38[8] != 0);
              v38[0xb] = 0;
            }
          }
label_3ca6:
          v30 += 1;
          v38 = &v38[0xc];
          if (v71 == v30) goto label_3ce2;
        }
        v30 += 1;
        v51 = &v38[0xc];
        sub_6660(v38,v44);
        v38 = v51;
      } while (v71 != v30);
label_3ce2:
      if ((!dat_1311a) || (dat_13020 != 1)) {
        v35 = (char *)((long)v27 + 0x34);
        while ((*(int *)&v35[4] <= -1 && ((*v35 == '\x01' || (!dat_1311a))))) {
          v35 = &v35[0x60];
          if (v35 == (char *)((long)v27 + v28 + 0x34)) {
            error(0,0,dcgettext(NULL,"no files remaining",5));
            goto label_3369;
          }
        }
      }
      if ((!v62) || (v44)) {
        if (fflush_unlocked(stdout)) {
          v35 = dcgettext(NULL,"write error",5);
          error(1,*__errno_location(),v35);
          return v20;
        }
        if (dat_13117) {
          v52[0] = 4.94065645841247e-324;
          if (0 <= (int)poll(v52,1,0)) goto label_45f9;
        }
      }
      else {
        if (!dat_13117) goto label_3bf5;
        v52[0] = 4.94065645841247e-324;
        if ((int)poll(v52,1,0) < 0) goto label_3bf5;
label_45f9:
        if ((unsigned long)v52[0] & 0x18000000000000) {
          raise(0xd); // return-dupe
          exit(1); // return-dupe, no-return
        }
      }
      if (!v62) {
        if (v48) goto label_3369;
        if (((dat_13110) && (kill(dat_13110,0))) && (*__errno_location() != 1))
          v48 = v9;
        else if (sub_c3a0(v76)) {
          v35 = dcgettext(NULL,"cannot read realtime clock",5);
          error(1,*__errno_location(),v35);
          return v16;
        }
      }
      goto label_3bf5;
    }
  }
label_3369:
  if ((dat_1310e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v14;
  }
  v44 = (unsigned int)(v75 ^ 1);
label_3381:
  if (v78 == *(long *)(v64 + 0x28))
    return v44;
  __stack_chk_fail(); // no-return
label_2bd5:
  dat_1310c = '\x01';
  goto label_2a90;
  while (v51 = &v51[0xc], v38 != v51) {
label_484a:
    v35 = (char *)*v51;
    if ((!lstat(v35,&v53)) && ((v53._24_4_ & 0xf000) == 0xa000)) goto label_3b98;
  }
  v50 = 0;
  do {
    if (((0 <= *(int *)((long)v27 + v50 + 0x38)) && (v44 = *(unsigned int *)((long)v27 + v50 + 0x30) & 0xf000, v44 != 0x8000)) && (v44 != 0x1000)) goto label_3b98;
    v50 += 0x60;
  } while (v28 != v50);
  if ((v75) || (dat_13020 != 2)) {
    v11 = inotify_init();
    if (0 <= v11) {
      if (fflush_unlocked(stdout)) {
        v35 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v35);
        return v26;
      }
      v50 = sub_87c0(v71,0,sub_5430,sub_5450,0);
      if (!v50)
        sub_c220(); // no-return, return-dupe
      v17 = 0xc06;
      if (dat_13020 != 1)
        v17 = 2;
      v10 = '\0';
      v8 = '\0';
      v66 = NULL;
      v51 = v27;
      do {
        v12 = dat_13020;
        if (!*(char *)((long)v51 + 0x34)) {
          v35 = (char *)*v51;
          v29 = (unsigned long)strlen(v35);
          *(unsigned int *)((long)v51 + 0x44) = 0xffffffff;
          if (v29 <= v66)
            v29 = v66;
          if (v12 == 1) {
            v37 = sub_7a60(v35);
            v4 = v35[v37];
            v51[10] = sub_7840(v35) - (long)v35;
            v35[v37] = '\0';
            v35 = ".";
            if (v37)
              v35 = (char *)*v51;
            *(unsigned int *)&v51[9] = inotify_add_watch(v11,v35,0x784);
            *(char *)(*v51 + v37) = v4;
            if ((int)v51[9] < 0) {
              v36 = __errno_location();
              if (*v36 != 0x1c) { // branch-flip
                v35 = (char *)sub_ae30(4,(char *)*v51);
                v47 = dcgettext(NULL,"cannot watch parent directory of %s",5);
                error(0,*v36,v47,v35);
              }
              else {
label_4c07:
                v35 = "inotify resources exhausted";
label_4c13:
                error(0,0,dcgettext(NULL,v35,5));
              }
              goto label_4c28;
            }
          }
          v12 = inotify_add_watch(v11,*v51,v17);
          *(int *)((long)v51 + 0x44) = v12;
          v66 = v29;
          if (0 <= v12) { // branch-flip
            if (!sub_8e20(v50,v51)) {
              sub_c220();
            }
            v8 = v9;
          }
          else {
            if ((int)v51[7] != -1)
              v10 = v9;
            v39 = (unsigned int *)__errno_location();
            if ((*v39 & 0xffffffef) == 0xc) goto label_4c07;
            if (*v39 != *(unsigned int *)((long)v51 + 0x3c)) {
              v35 = (char *)sub_ae30(4,(char *)*v51);
              v47 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v39,v47,v35);
            }
          }
        }
        v51 = &v51[0xc];
      } while (v38 != v51);
      if (dat_13020 != 2) {
label_4c85:
        v56 = &v27[v71 * 0xc + -0xc];
        v51 = v27;
        do {
          if (!*(char *)((long)v51 + 0x34)) {
            if (dat_13020 != 1) { // branch-flip
              if ((((int)v51[7] != -1) && (v35 = (char *)*v51, !stat(v35,&v53))) && ((v51[4] != CONCAT26(v53._6_2_,CONCAT24(v53._4_2_,v53._0_4_)) || (v51[5] != v53._8_8_)))) {
                v35 = (char *)*v51;
                v40 = sub_ae30(4,(char *)sub_5fe0(v35));
                v35 = dcgettext(NULL,"%s was replaced",5);
                error(0,*__errno_location(),v35,v40);
                goto label_4c28;
              }
            }
            else {
              sub_6660(v51,0);
            }
            sub_6430(v51,&v56);
          }
          v51 = &v51[0xc];
        } while (v38 != v51);
        v30 = 0;
        v69 = (unsigned long)((long)v66 + 0x11);
        v66 = (void *)sub_bd20(v69);
        v70._0_4_ = 3;
        v67 = NULL;
label_4d4b:
        do {
          if (((dat_13020 == 1) && (!dat_1311a)) && (!sub_82f0(v50))) {
            error(1,0,dcgettext(NULL,"no files remaining",5));
            return v23;
          }
          if (v67 > v30) {
label_4d63:
            v36 = (int *)((long)v66 + v30);
            v44 = v36[1];
            v5 = v36[3];
            v12 = *v36;
            v30 = v30 + 0x10 + (unsigned long)v5;
            if (v44 & 0x400) { // branch-flip
              if (!v5) {
                v42 = 0;
                do {
                  if (v12 == (int)v27[v42 * 0xc + 9]) {
                    v35 = "directory containing watched file was removed";
                    goto label_4c13;
                  }
                  v42 += 1;
                } while (v71 != v42);
label_4e77:
                v77 = v12;
                v38 = (long *)sub_8530(v50,v52);
                if (v38) {
label_4e47:
                  if (v36[1] & 0xe04U) { // branch-flip
                    if (v36[1] & 0x400U) {
                      inotify_rm_watch(v11,*(unsigned int *)((long)v38 + 0x44));
                      sub_8e80(v50,v38);
                    }
                    sub_6660(v38,0);
                  }
                  else {
                    sub_6430(v38,&v56);
                  }
                }
                goto label_4d4b;
              }
            }
            else if (!v5) goto label_4e77;
            v42 = 0;
            v38 = v27;
            do {
              if ((v12 == (int)v38[9]) && (v37 = v38[10], v6 = *v38, !strcmp((char *)&v36[4],(char *)(v37 + v6)))) {
                if (v71 == v42) break;
                v38 = &v27[v42 * 0xc];
                if (!(v44 & 0x200)) {
                  v12 = inotify_add_watch(v11,*v38,v17);
                  if (v12 <= -1) {
                    v39 = (unsigned int *)__errno_location();
                    if ((*v39 & 0xffffffef) == 0xc) goto label_4c07;
                    v35 = (char *)sub_ae30(4,(char *)*v38);
                    v47 = dcgettext(NULL,"cannot watch %s",5);
                    error(0,*v39,v47,v35);
                  }
                  v49 = *(int *)((long)v38 + 0x44);
                  if (v12 != v49) { // branch-flip
                    if (0 <= v49) {
                      inotify_rm_watch(v11);
                      sub_8e80(v50,v38);
                    }
                  }
                  else if (0 <= v49) goto label_4e3a;
                  *(int *)((long)v38 + 0x44) = v12;
                  if (v12 == -1) break;
                  v73 = (long *)sub_8e80(v50,v38);
                  if ((v73) && (v38 != v73)) {
                    if (dat_13020 == 1)
                      sub_6660(v73,0);
                    *(unsigned int *)((long)v73 + 0x44) = 0xffffffff;
                    v35 = (char *)sub_5fe0((char *)*v73);
                    sub_5bd0(*(unsigned int *)&v73[7],v35);
                  }
                  if (!sub_8e20(v50,v38)) {
                    sub_c220();
                  }
                }
label_4e3a:
                if (dat_13020 == 1)
                  sub_6660(v38,0);
                goto label_4e47;
              }
              v42 += 1;
              v38 = &v38[0xc];
            } while (v71 != v42);
            goto label_4d4b;
          }
          do {
            if (dat_13110) { // branch-flip
              if (v48)
                exit(0); // no-return
              if ((kill(dat_13110,0)) && (*__errno_location() != 1)) {
                v12 = 0;
                v48 = v9;
              }
              else {
                v12 = 0;
                if (0.0 < v76) {
                  if (dat_e4a8 <= v76) goto label_4fb1;
                  v12 = (int)(dat_e4b0 * v76);
                  v12 += (unsigned int)((double)v12 < dat_e4b0 * v76);
                }
              }
            }
            else {
label_4fb1:
              v12 = -1;
            }
            v53._4_2_ = 1;
            v53._8_8_ = 1;
            v53._0_4_ = v11;
            v12 = poll(&v53,(unsigned long)dat_13117 + 1,v12);
          } while (!v12);
          if (v12 < 0) {
            v35 = dcgettext(NULL,"error waiting for inotify and output events",5);
            error(1,*__errno_location(),v35);
            return v25;
          }
          if (v53._14_2_) {
            raise(0xd);
            exit(1);
          }
          v67 = (unsigned long)sub_b540(v11,v66,v69);
          if (v67) {
            v30 = 0;
            if (v67 != (char *)0xffffffffffffffff) goto label_4d63;
            if (*__errno_location() != 0x16) {
label_500c:
              v35 = dcgettext(NULL,"error reading inotify event",5);
              error(1,*__errno_location(),v35);
              return v24;
            }
          }
          if (!v70._0_4_) goto label_500c;
          v30 = 0;
          v70._0_4_ -= 1;
          v69 = (unsigned long)((long)v69 * 2);
          v66 = (void *)sub_bd70(v66,v69);
          v67 = NULL;
        } while( true );
      }
      if (!v10) {
        if (!v8) {
          exit(1);
        }
        goto label_4c85;
      }
label_4c28:
      sub_8980(v50);
      close(v11);
      *__errno_location() = 0;
    }
    v35 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
    error(0,*__errno_location(),v35);
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
unsigned long sub_5460(int a0,char *a1) // ternary
{
  int *v1; // rax
  char *v2; // rax
  char *v3; // rax
  statfs v4; // stack - 0xa8
  unsigned long v5; // r12
  unsigned long v6; // r12
  undefined7 v7; // r12
  
  if (fstatfs(a0,&v4)) {
    v1 = __errno_location();
    v6 = 1;
    if (*v1 != 0x26) {
      v2 = (char *)sub_ae30(4,a1);
      v3 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
      error(0,*v1,v3,v2);
    }
    return v6 & 0xffffffff; // return-dupe
  }
  if (v4._0_8_ != 0x2bad1dea) {
    v7 = (undefined7)((unsigned long)v5 >> 8); // branch-flip
    if (0x2bad1deb <= (long)v4._0_8_) {
      if (v4._0_8_ != 0x62656570) {
        if (0x62656571 <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x858458f6) {
            if (0x858458f7 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0xcafe4a11) {
                if (0xcafe4a12 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0xf2f52010) {
                    if (0xf2f52011 <= (long)v4._0_8_) { // branch-flip
                      if (v4._0_8_ != 0xf97cff8c) {
                        v6 = CONCAT71(v7,v4._0_8_ != 0xf995e849);
                        return v6 & 0xffffffff;
                      }
                    }
                    else if (v4._0_8_ != 0xde5e81e4) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0xe0f5e1e2);
                      return v6 & 0xffffffff;
                    }
                  }
                }
                else if (v4._0_8_ != 0xabba1974) {
                  if (0xabba1975 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0xc7571590) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0xc97e8168);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x9123683e) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x958458f6);
                    return v6 & 0xffffffff;
                  }
                }
              }
            }
            else if (v4._0_8_ != 0x6c6f6f70) {
              if (0x6c6f6f71 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x73717368) {
                  if (0x73717369 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0x73727279) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0x74726163);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x6e736673) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x73636673);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4._0_8_ != 0x64646178) {
                if (0x64646179 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x67596969) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x68191122);
                    return v6 & 0xffffffff;
                  }
                }
                else if ((v4._0_8_ != 0x63677270) && (v4._0_8_ != 0x64626720)) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x62656572);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
        }
        else if (v4._0_8_ != 0x53464846) {
          if (0x53464847 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x58465342) {
              if (0x58465343 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x5dca2df5) {
                  if (0x5dca2df6 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0x6165676c) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0x62646576);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x5a3c69f0) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x5a4f4653);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4._0_8_ != 0x54190100) {
                if (0x54190101 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x565a4653) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x58295829);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4._0_8_ != 0x5346544e) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x534f434b);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
          else if (v4._0_8_ != 0x453dcd28) {
            if (0x453dcd29 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x52654973) {
                if (0x52654974 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x5345434d) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x5346314d);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4._0_8_ != 0x454d444d) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x45584653);
                  return v6 & 0xffffffff;
                }
              }
            }
            else if (v4._0_8_ != 0x42494e4d) {
              if (0x42494e4e <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x43415d53) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x444d4142);
                  return v6 & 0xffffffff;
                }
              }
              else if ((v4._0_8_ != 0x3153464a) && (v4._0_8_ != 0x42465331)) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x2fc12fc1);
                return v6 & 0xffffffff;
              }
            }
          }
        }
      }
    }
    else if (0x9fa3 <= (long)v4._0_8_) { // branch-flip
      if (0x12ff7b8 <= (long)v4._0_8_) { // branch-flip
        if (v4._0_8_ != 0x15013346) {
          if (0x15013347 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x2011bab0) {
              if (0x2011bab1 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x24051905) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x28cd3d45);
                  return v6 & 0xffffffff;
                }
              }
              else if (v4._0_8_ != 0x19800202) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x1badface);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4._0_8_ != 0xbad1dea) {
            if (0xbad1deb <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x11307854) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x13661366);
                return v6 & 0xffffffff;
              }
            }
            else if (v4._0_8_ != 0x7655821) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x9041934);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else if (((long)v4._0_8_ <= 0x12ff7b3) && (v4._0_8_ != 0x27e0eb)) {
        if (0x27e0ec <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x1021994) {
            if (0x1021995 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x1021997) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x12fd16d);
                return v6 & 0xffffffff;
              }
            }
            else if (v4._0_8_ != 0x414a53) {
              v6 = CONCAT71(v7,v4._0_8_ != 0xc0ffee);
              return v6 & 0xffffffff;
            }
          }
        }
        else if (v4._0_8_ != 0xef53) {
          if (0xef54 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0xf15f) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x11954);
              return v6 & 0xffffffff;
            }
          }
          else if ((v4._0_8_ != 0xadff) && (v4._0_8_ != 0xef51)) {
            v6 = CONCAT71(v7,v4._0_8_ != 0xadf5);
            return v6 & 0xffffffff;
          }
        }
      }
    }
    else if ((long)v4._0_8_ <= 0x9f9f) {
      if (0x4007 <= (long)v4._0_8_) { // branch-flip
        if (v4._0_8_ != 0x4d5a) {
          if (0x4d5b <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x72b6) {
              if (0x72b7 <= (long)v4._0_8_) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x9660);
                return v6 & 0xffffffff;
              }
              if (v4._0_8_ != 0x5df5) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x7275);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4._0_8_ != 0x4858) {
            if (0x4859 <= (long)v4._0_8_) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x4d44);
              return v6 & 0xffffffff;
            }
            if (v4._0_8_ != 0x4244) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x482b);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else {
        if (0x3fff < (long)v4._0_8_) {
          v6 = (unsigned long)((unsigned int)(0x51L >> (v4._0_1_ & 0x3f)) & 1 ^ 1);
          return v6 & 0xffffffff;
        }
        if (0x1390 <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x2478) {
            if (0x2479 <= (long)v4._0_8_) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x3434);
              return v6 & 0xffffffff;
            }
            if (v4._0_8_ != 0x1cd1) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x2468);
              return v6 & 0xffffffff;
            }
          }
        }
        else {
          if (0x1372 < (long)v4._0_8_) {
            v6 = (unsigned long)((unsigned int)(0x10001401L >> (v4._0_1_ + 0x8d & 0x3f)) & 1 ^ 1);
            return v6 & 0xffffffff;
          }
          if (v4._0_8_ != 0x187) {
            v6 = (0x188 <= (long)v4._0_8_) ? CONCAT71(v7,v4._0_8_ != 0x7c0) : CONCAT71(v7,(v4._0_8_ - 0x2f & 0xfffffffffffffffb) != 0); // branch-flip
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
unsigned long sub_5ce0(char *a0,unsigned int a1,long a2,long *a3) // return-dupe
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
unsigned long sub_5e30(char *a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
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
int * sub_6020(char a0,char *a1,unsigned int a2,unsigned long a3)
{
  int *v1;
  int *v2; // rax
  unsigned long v3;
  char *v4;
  unsigned long v5;
  char v6 [8200];
  long v7; // r12
  
  v7 = 0;
  v5 = a3;
  do {
    v3 = 0x2000;
    if (v5 <= 0x2000)
      v3 = v5;
    v2 = (unsigned long)sub_b540(a2,v6,v3);
    if (v2 == (int *)0xffffffffffffffff) {
      v2 = __errno_location();
      v1 = v2;
      if (*v2 == 0xb) {
        v2 = (int *)v7;
        return v2;
      }
      v2 = (char *)sub_ae30(4,a1);
      v4 = (char *)v2;
      v2 = (char *)dcgettext(NULL,"error reading %s",5);
      error(1,*v1,(char *)v2,v4);
      return v2;
    }
    if (!v2) {
      v2 = (int *)v7;
      return v2;
    }
    if (a0) {
      v4 = "\n";
      if (dat_13010)
        v4 = "";
      __printf_chk(1,"%s==> %s <==\n",v4,a1);
      dat_13010 = '\0';
    }
    v7 += (long)v2;
    sub_5c60(v6,v2);
    if (a3 != 0xffffffffffffffff) {
      v5 -= (long)v2;
      if (!v5) {
        v2 = (int *)v7;
        return v2;
      }
      if (a3 == 0xfffffffffffffffe) {
        v2 = (int *)v7;
        return v2;
      }
    }
    a0 = 0;
  } while( true );
}


// Function: sub_6190 @ 0x6190
unsigned long sub_6190(char *a0,unsigned int a1,long a2,unsigned long a3,unsigned long a4,long *a5) // return-dupe
{
  long v1; // rax
  unsigned long v10; // stack - 0x2050
  unsigned long v2;
  void *v3; // rax
  long v4;
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7;
  int v8; // ebx
  char v9 [8200];
  
  if (!a2)
    return 1;
  v1 = (long)(a4 - a3) % 0x2000;
  v2 = 0x2000;
  if (v1)
    v2 = v1;
  v1 = a4 - v2;
  v10 = a4;
  if ((long)lseek(a1,v1,0) < 0)
    sub_5f40(v1,0,a0); // no-return
  v2 = sub_b540(a1,v9,v2);
  if (v2 != 0xffffffffffffffff) {
    v8 = (int)dat_13114;
    *a5 = v1 + v2;
    if ((v2) && (v9[v2 - 1] != dat_13114))
      a2 -= 1;
    do {
      v7 = v2;
      while ((v7 && (v3 = memrchr(v9,v8,v7), v3))) {
        v4 = a2 + -1;
        v7 = (long)v3 - (long)v9;
        if (!a2) {
          if (v2 - 1 != v7)
            sub_5c60((void *)((long)v3 + 1));
          v4 = sub_6020(0,a0,a1,(v10 - v2) - v1);
          *a5 = *a5 + v4;
          return 1;
        }
        a2 = v4;
      }
      if (v1 == a3) {
        if (0 <= (long)lseek(a1,v1,0)) {
          *a5 = sub_6020(0,a0,a1,v10) + v1;
          return 1;
        }
        sub_5f40(v1,0,a0); // return-dupe, no-return
      }
      v1 -= 0x2000;
      if ((long)lseek(a1,v1,0) < 0) {
        sub_5f40(v1,0,a0);
      }
      v2 = sub_b540(a1,v9,0x2000);
      if (v2 == 0xffffffffffffffff) break;
      *a5 = v1 + v2;
      if (!v2)
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
void sub_6430(struct_1 *a0,long *a1) // return-dupe x3
{
  unsigned int v1;
  long v2; // rax
  char *v3;
  int *v4; // rax
  char *v5; // rax
  stat v6; // stack - 0xc8
  int v7;
  bool v8;
  
  v7 = a0->field_0x38;
  if (v7 == -1)
    return;
  v3 = a0->field_0x0;
  if (!strcmp(v3,"-")) {
    v3 = dcgettext(NULL,"standard input",5);
    v7 = a0->field_0x38;
  }
  if (fstat(v7,&v6)) {
    v4 = __errno_location();
    a0->field_0x3c = *v4;
    sub_5bd0(a0->field_0x38,v3);
    a0->field_0x38 = 0xffffffff;
    return;
  }
  if ((a0->field_0x30 & 0xf000) == 0x8000) {
    if ((long)a0->field_0x8 <= (long)v6._48_8_) { // branch-flip
      if ((v6._48_8_ == a0->field_0x8) && (!(((unsigned int)((long)v6._96_8_ < a0->field_0x18) - (unsigned int)(a0->field_0x18 < (long)v6._96_8_)) + ((unsigned int)((long)v6._88_8_ < a0->field_0x10) - (unsigned int)(a0->field_0x10 < (long)v6._88_8_)) * 2)))
        return;
    }
    else {
      v5 = (char *)sub_b1b0(0,3,v3);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v5);
      v1 = a0->field_0x38;
      if ((long)lseek(v1,0,0) < 0)
        sub_5f40(0,0,v3); // no-return
      a0->field_0x8 = 0;
    }
  }
  v8 = 0;
  if (dat_13115)
    v8 = (struct_1 *)*a1 != a0;
  v2 = sub_6020(v8,v3,a0->field_0x38,0xffffffffffffffff);
  a0->field_0x8 = a0->field_0x8 + v2;
  if (!v2)
    return;
  *a1 = (long)a0;
  if (!fflush_unlocked(stdout))
    return;
  v3 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v3);
}


// Function: sub_6660 @ 0x6660
void sub_6660(struct_2 *a0,unsigned int a1)
{
  char v1;
  unsigned long v10; // rax
  char *v11;
  char *v12;
  stat v13; // stack - 0xd8
  unsigned int v2;
  int v3;
  char v4;
  int v5; // eax
  int v6; // eax
  unsigned int v7;
  int v8; // eax
  unsigned int *v9; // rax
  
  v11 = a0->field_0x0;
  v5 = strcmp(v11,"-");
  v2 = a0->field_0x3c;
  v4 = a0->field_0x36;
  if (v5) { // branch-flip
    v6 = sub_7dd0(v11,((a1 ^ 1) & 0xff) << 0xb);
    v7 = a0->field_0x3c;
  }
  else {
    v6 = 0;
    v7 = v2;
  }
  v1 = dat_1311a;
  if ((a0->field_0x38 == -1) == (v7 == 0))
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1311a) { // branch-flip
    if (v6 != -1) { // branch-flip
      a0->field_0x36 = 1;
      if ((dat_1310c) || ((v11 = a0->field_0x0, lstat(v11,&v13) || ((v13._24_4_ & 0xf000) != 0xa000)))) goto label_67fb;
    }
    else {
      a0->field_0x36 = 0;
      v11 = a0->field_0x0;
      if (dat_1310c) {
        a0->field_0x3c = *__errno_location();
label_6991:
        v5 = strcmp(v11,"-");
        if (v4) {
          if (!v5)
            v11 = dcgettext(NULL,"standard input",5);
          v11 = (char *)sub_ae30(4,v11);
          v12 = dcgettext(NULL,"%s has become inaccessible",5);
          error(0,a0->field_0x3c,v12,v11);
          v11 = a0->field_0x0;
          goto label_675a;
        }
        goto label_69a7;
      }
      if ((lstat(v11,&v13)) || ((v13._24_4_ & 0xf000) != 0xa000)) goto label_6714;
    }
label_6cd0:
    v11 = a0->field_0x0;
    a0->field_0x3c = 0xffffffff;
    *(char *)&a0->field_0x34 = 1;
    if (!strcmp(v11,"-"))
      v11 = dcgettext(NULL,"standard input",5);
    v11 = (char *)sub_ae30(4,v11);
    error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v11);
    v11 = a0->field_0x0;
  }
  else {
    a0->field_0x36 = 1;
    if ((!dat_1310c) && ((v11 = a0->field_0x0, !lstat(v11,&v13) && ((v13._24_4_ & 0xf000) == 0xa000)))) goto label_6cd0;
    if (v6 != -1) {
label_67fb:
      if (fstat(v6,&v13) < 0) goto label_6714;
      if (((v13._24_4_ & 0xf000) - 0x1000 & 0xffffe000) && ((v13._24_4_ & 0xb000) != 0x8000)) {
        a0->field_0x3c = 0xffffffff;
        a0->field_0x36 = 0;
        if ((v1) && (dat_13020 == 1)) {
          *(char *)&a0->field_0x34 = 0;
          if (v4) // branch-flip
            v11 = "";
          else {
            v11 = "";
            if (v2 == 0xffffffff) goto label_69a7;
          }
        }
        else {
          *(char *)&a0->field_0x34 = 1;
          if ((!v4) && (v2 == 0xffffffff)) goto label_69a7;
          v11 = dcgettext(NULL,"; giving up on this name",5);
        }
        v12 = a0->field_0x0;
        if (!strcmp(v12,"-"))
          v12 = dcgettext(NULL,"standard input",5);
        v12 = (char *)sub_ae30(4,v12);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v12,v11);
      }
      else {
        v11 = a0->field_0x0;
        if (!strcmp(v11,"-"))
          v11 = dcgettext(NULL,"standard input",5);
        v4 = sub_5460(v6,v11);
        *(char *)((long)&a0->field_0x34 + 1) = v4;
        if ((!v4) || (dat_1310c)) {
          a0->field_0x3c = 0;
          v3 = a0->field_0x38;
          if (v2 & 0xfffffffd) { // branch-flip
            if (v3 != -1)
              __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
            v11 = a0->field_0x0;
            if (!strcmp(v11,"-"))
              v11 = dcgettext(NULL,"standard input",5);
            v10 = sub_ae30(4,v11);
            v11 = "%s has become accessible";
          }
          else {
            v11 = a0->field_0x0;
            v8 = strcmp(v11,"-");
            if (v3 != -1) {
              if ((a0->field_0x28 == v13._8_8_) && (a0->field_0x20 == v13._0_8_)) {
                if (!v8)
                  v11 = dcgettext(NULL,"standard input",5);
                sub_5bd0(v6,v11);
                return;
              }
              if (!v8)
                v11 = dcgettext(NULL,"standard input",5);
              v11 = (char *)sub_ae30(4,v11);
              error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v11);
              v11 = a0->field_0x0;
              if (!strcmp(v11,"-"))
                v11 = dcgettext(NULL,"standard input",5);
              sub_5bd0(a0->field_0x38,v11);
              goto label_68e3;
            }
            if (!v8)
              v11 = dcgettext(NULL,"standard input",5);
            v10 = sub_ae30(4,v11);
            v11 = "%s has appeared;  following new file";
          }
          error(0,0,dcgettext(NULL,v11,5),v10);
label_68e3:
          a1 &= 0xff;
          a0->field_0x38 = v6;
          if (!v5)
            a1 = 0xffffffff;
          *(char *)&a0->field_0x34 = 0;
          a0->field_0x8 = 0;
          a0->field_0x10 = v13._88_8_;
          a0->field_0x40 = a1;
          v11 = a0->field_0x0;
          a0->field_0x18 = v13._96_8_;
          a0->field_0x58 = 0;
          a0->field_0x20 = v13._0_8_;
          a0->field_0x28 = v13._8_8_;
          a0->field_0x30 = v13._24_4_;
          if (!strcmp(v11,"-"))
            v11 = dcgettext(NULL,"standard input",5);
          if (0 <= (long)lseek(v6,0,0))
            return;
          sub_5f40(0,0,v11); // no-return
        }
        v11 = a0->field_0x0;
        a0->field_0x3c = 0xffffffff;
        if (!strcmp(v11,"-"))
          v11 = dcgettext(NULL,"standard input",5);
        v11 = (char *)sub_ae30(4,v11);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v11);
        a0->field_0x34 = 0x101;
      }
label_69a7:
      v11 = a0->field_0x0;
      v5 = strcmp(v11,"-");
      goto label_69b7;
    }
label_6714:
    v1 = a0->field_0x36;
    v9 = (unsigned int *)__errno_location();
    v11 = a0->field_0x0;
    v7 = *v9;
    a0->field_0x3c = v7;
    if (!v1) goto label_6991;
    v5 = strcmp(v11,"-");
    if (v7 != v2) {
      if (!v5)
        v11 = dcgettext(NULL,"standard input",5);
      v11 = (char *)sub_b1b0(0,3,v11);
      error(0,*v9,"%s",v11);
      v11 = a0->field_0x0;
    }
  }
label_675a:
  v5 = strcmp(v11,"-");
label_69b7:
  if (!v5)
    v11 = dcgettext(NULL,"standard input",5);
  sub_5bd0(v6,v11);
  v11 = a0->field_0x0;
  if (!strcmp(v11,"-"))
    v11 = dcgettext(NULL,"standard input",5);
  sub_5bd0(a0->field_0x38,v11);
  a0->field_0x38 = 0xffffffff;
}


// Function: sub_6f70 @ 0x6f70
void sub_6f70(int a0)
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
  
  v6 = dat_13138;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v6);
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
      if (!strcmp("tail",v6)) break;
      v6 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v6);
    v6 = *(char **)((long)v4 + 8);
    if (!v6)
      v6 = "tail";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "tail";
    if (!strcmp("tail","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "tail")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
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
unsigned long sub_7f10(struct_5 *a0,long a1,unsigned long *a2,bool a3) // early-return
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


// Function: sub_8160 @ 0x8160
unsigned long sub_8160(struct_6 *a0,struct_10 *a1,bool a2) // return-dupe x2
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
label_8196:
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
        goto label_8196;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}


// Function: sub_83d0 @ 0x83d0
void sub_83d0(unsigned long *a0,FILE *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_e5c0) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_8530 @ 0x8530
long sub_8530(struct_3 *a0,long a1) // return-dupe
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


// Function: sub_85f0 @ 0x85f0
long sub_85f0(struct_11 *a0,long a1)
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


// Function: sub_86d0 @ 0x86d0
long sub_86d0(struct_10 *a0,void *a1,unsigned long a2)
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
    v2 = sub_8020(*(float *)(a1 + 8),a0,(unsigned char)*(char *)(a1 + 0x10));
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
void sub_88d0(struct_13 *a0)
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
        if (a0->field_0x8 <= v5) goto label_896b;
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
label_896b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_8980 @ 0x8980
void sub_8980(struct_4 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_8a2c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_89ef;
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
label_89ef:
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
label_8a2c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_8a70 @ 0x8a70
unsigned int sub_8a70(struct_8 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_8020(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_8160(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_8160(a0,&v2,1)) && (sub_8160(a0,&v2,0))) {
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
unsigned long sub_8e20(long a0,unsigned long a1) // early-return
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
long sub_8e80(long a0,long a1) // ternary x2
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

