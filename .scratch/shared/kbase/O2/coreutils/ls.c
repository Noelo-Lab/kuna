// Function: sub_4d10 @ 0x4d10
unsigned long sub_4d10(unsigned int a0,char **a1)
{
  char **v1;
  unsigned char v10;
  int v11;
  unsigned int v12;
  int v13; // eax
  unsigned long *v14; // rax
  void *v15;
  long v16; // rax
  unsigned long v17;
  unsigned long v18;
  unsigned long v19; // rax
  char v2;
  unsigned long v20; // rax
  void *v21; // stack - 0x58
  unsigned short v22; // stack - 0x43
  char *v23; // stack - 0x60
  char *v24;
  long v25;
  long *v26;
  char *v27;
  long v28;
  bool v29;
  char *v3;
  unsigned int v30; // stack - 0x98
  int v31; // stack - 0x90
  int v32; // stack - 0x8c
  unsigned long v33; // stack - 0x88
  char *v34; // stack - 0x80
  void *v35; // stack - 0x78
  int v36; // stack - 0x70
  unsigned long v37; // stack - 0x50
  char v38; // stack - 0x41
  long v39; // stack - 0x40
  char v4 [16];
  void *v5;
  void *v6;
  unsigned short v7;
  char v8;
  unsigned long v9;
  
  v39 = *(long *)(v28 + 0x28);
  sub_150f0(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  dat_251f8 = 2;
  sub_19d40(sub_f770);
  dat_262b8 = 1;
  dat_26210 = 0;
  dat_26380 = NULL;
  dat_26370 = 0x8000000000000000;
  dat_26378 = 0xffffffffffffffff;
  v33 = 0xffffffffffffffff;
  v35 = (void *)0xffffffffffffffff;
  v32 = -1;
  v31 = -1;
  v36 = -1;
  v30 = 0xffffffff;
  v29 = 0;
  v34 = NULL;
label_4e20:
  v21 = (void *)CONCAT44(v21._4_4_,0xffffffff);
  v17 = (unsigned long)a0;
  v11 = getopt_long(a0,a1,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(void *)0x24300,(int *)&v21);
  if (v11 != -1) {
    switch(v11) {
      case 0x31:
        v30 = (unsigned int)(v30 != 0);
        break;
      case 0x41:
        dat_262f0 = 1;
        break;
      case 0x42:
        sub_6be0("*~");
        sub_6be0(".*~");
        break;
      case 0x43:
        v30 = 2;
        break;
      case 0x44:
        dat_26318 = 1;
        break;
      case 0x46:
        if (optarg) {
          v25 = sub_ecf0("--classify",optarg,0x241e0,0x1a6e0,4,dat_251f0,1);
          v11 = *(int *)(v25 * 4 + 0x1a6e0);
          if ((v11 != 1) && ((v11 != 2 || (!sub_6d10())))) break;
        }
        dat_26314 = 3;
        break;
      case 0x47:
        dat_25028 = 0;
        break;
      case 0x48:
        dat_262f8 = 2;
        break;
      case 0x49:
        sub_6be0(optarg);
        break;
      case 0x4c:
        dat_262f8 = 4;
        break;
      case 0x4e:
        v31 = 0;
        break;
      case 0x51:
        v31 = 5;
        break;
      case 0x52:
        dat_262f6 = '\x01';
        break;
      case 0x53:
        v32 = 3;
        break;
      case 0x54:
        v35 = (void *)sub_18ad0(optarg,0,0,0x7fffffffffffffff,0x1bb19,dcgettext(NULL,"invalid tab size",5),2);
        break;
      case 0x55:
        v32 = 6;
        break;
      case 0x58:
        v32 = 1;
        break;
      case 0x5a:
        dat_26365 = '\x01';
        break;
      case 0x61:
        dat_262f0 = 2;
        break;
      case 0x62:
        v31 = 7;
        break;
      case 99:
        dat_26334 = 1;
        break;
      case 100:
        dat_262f5 = '\x01';
        break;
      case 0x66:
        dat_262f0 = 2;
        dat_26312 = 0;
        dat_26311 = 0;
        v12 = 0xffffffff;
        if (v30)
          v12 = v30;
        dat_2632c = '\0';
        v32 = 6;
        v30 = v12;
        break;
      case 0x67:
        dat_25029 = 0;
        v30 = 0;
        break;
      case 0x68:
        dat_26328 = 0xb0;
        dat_2631c = 0xb0;
        dat_26320 = 1;
        dat_25020 = 1;
        break;
      case 0x69:
        dat_262fc = 1;
        break;
      case 0x6b:
        v29 = 1;
        break;
      case 0x6c:
        v30 = 0;
        break;
      case 0x6d:
        v30 = 4;
        break;
      case 0x6e:
        dat_2632d = 1;
        v30 = 0;
        break;
      case 0x6f:
        dat_25028 = 0;
        v30 = 0;
        break;
      case 0x70:
        dat_26314 = 1;
        break;
      case 0x71:
        v36 = 1;
        break;
      case 0x72:
        dat_2632f = 1;
        break;
      case 0x73:
        dat_2632c = '\x01';
        break;
      case 0x74:
        v32 = 5;
        break;
      case 0x75:
        dat_26334 = 2;
        break;
      case 0x76:
label_5170:
        v32 = 4;
        break;
      case 0x77:
        v33 = sub_6c10(optarg);
        if (0 <= (long)v33) break;
        a1 = (char **)sub_174c0(optarg);
        error(2,0,"%s: %s",dcgettext(NULL,"invalid line width",5),a1);
        goto label_5170;
      case 0x78:
        v30 = 3;
        break;
      case 0x80:
        dat_2632e = 1;
        break;
      case 0x81:
        v11 = sub_12150(optarg,0x26328,0x26320);
        if (v11)
          sub_18d20(v11,(unsigned long)v21 & 0xffffffff,0,0x24300,optarg); // no-return
        dat_2631c = dat_26328;
        dat_25020 = dat_26320;
        break;
      case 0x82:
        if (optarg) { // branch-flip
          v25 = sub_ecf0("--color",optarg,0x241e0,0x1a6e0,4,dat_251f0);
          v11 = *(int *)(v25 * 4 + 0x1a6e0);
          if (v11 == 1) goto label_50a7;
          v10 = 0;
          if (v11 == 2)
            v10 = sub_6d10();
        }
        else {
label_50a7:
          v10 = 1;
        }
        dat_26312 = v10 & 1;
        break;
      case 0x83:
        dat_262f8 = 3;
        break;
      case 0x84:
        dat_26314 = 2;
        break;
      case 0x85:
        v25 = sub_ecf0("--format",optarg,0x242c0,0x1a750,4,dat_251f0);
        v30 = *(unsigned int *)(v25 * 4 + 0x1a750);
        break;
      case 0x86:
        v30 = 0;
        v34 = "full-iso";
        break;
      case 0x87:
        dat_262f4 = 1;
        break;
      case 0x88:
        v14 = (unsigned long *)sub_18590(0x10);
        *v14 = optarg;
        v14[1] = dat_262e0;
        dat_262e0 = v14;
        break;
      case 0x89:
        if (optarg) { // branch-flip
          v25 = sub_ecf0("--hyperlink",optarg,0x241e0,0x1a6e0,4,dat_251f0,1);
          v11 = *(int *)(v25 * 4 + 0x1a6e0);
          if (v11 == 1) goto label_4f7e;
          v10 = 0;
          if (v11 == 2)
            v10 = sub_6d10();
        }
        else {
label_4f7e:
          v10 = 1;
        }
        dat_26311 = v10 & 1;
        break;
      case 0x8a:
        v25 = sub_ecf0("--indicator-style",optarg,0x24980,0x1a770,4,dat_251f0,1);
        dat_26314 = *(unsigned int *)(v25 * 4 + 0x1a770);
        break;
      case 0x8b:
        v25 = sub_ecf0("--quoting-style",optarg,0x24a20,0x1e9c0,4,dat_251f0,1);
        v31 = *(int *)(v25 * 4 + 0x1e9c0);
        break;
      case 0x8c:
        goto label_4e85;
      case 0x8d:
        dat_26328 = 0x90;
        dat_2631c = 0x90;
        dat_26320 = 1;
        dat_25020 = 1;
        break;
      case 0x8e:
        v25 = sub_ecf0("--sort",optarg,0x24280,0x1a730,4,dat_251f0,1);
        v32 = *(int *)(v25 * 4 + 0x1a730);
        break;
      case 0x8f:
        v25 = sub_ecf0("--time",optarg,0x24240,0x1a710,4,dat_251f0,1);
        dat_26334 = *(int *)(v25 * 4 + 0x1a710);
        break;
      case 0x90:
        goto label_5463;
      case 0x91:
        dat_25019 = '\0';
        dat_26312 = 0;
        v31 = 0;
        v30 = (unsigned int)(v30 != 0);
label_4e85:
        v36 = 0;
        break;
      case 0xffffff7d:
        v24 = "ls";
        if ((dat_251e0 != 1) && (v24 = "dir", dat_251e0 != 2))
          v24 = "vdir";
        sub_183c0(stdout,v24,"GNU coreutils",dat_251e8,"Richard M. Stallman","David MacKenzie",0,v17);
        exit(0); // no-return
      case 0xffffff7e:
        sub_df00(0); // no-return
      default:
        goto label_65b3;
      
    }
    goto label_4e20;
  }
  if (!dat_26320) {
    v24 = getenv("LS_BLOCK_SIZE");
    sub_12150(v24,0x26328,0x26320);
    if ((v24) || (getenv("BLOCK_SIZE"))) {
      dat_2631c = dat_26328;
      dat_25020 = dat_26320;
    }
    if (v29) {
      dat_26328 = 0;
      dat_26320 = 0x400;
    }
  }
  if (0 <= (int)v30) { // branch-flip
    dat_26338 = v30;
    if (2 < v30 - 2) goto label_5af0;
label_556e:
    if (v33 == 0xffffffffffffffff) {
      if ((sub_6d10()) && (0 <= ioctl(1,0x5413,&v21))) {
        v33 = (unsigned long)v21._2_2_;
        if (v21._2_2_) goto label_557a;
      }
      v24 = getenv("COLUMNS");
      if ((v24) && (*v24)) {
        v33 = sub_6c10(v24);
        if (0 <= (long)v33) goto label_557a;
        v18 = sub_174c0(v24);
        error(0,0,dcgettext(NULL,"ignoring invalid width in environment variable COLUMNS: %s",5),v18);
      }
      v33 = 0x50; // crossjump-dupe
    }
  }
  else {
    if (dat_251e0 != 1) { // branch-flip
      if (dat_251e0 == 2) {
label_6386:
        dat_26338 = 2;
        goto label_556e;
      }
      dat_26338 = 0;
    }
    else {
      if (sub_6d10()) goto label_6386;
      dat_26338 = 1;
    }
label_5af0:
    if (dat_26312) goto label_556e;
    if (v33 != 0xffffffffffffffff) goto label_557a;
    v33 = 0x50;
  }
label_557a:
  dat_262b0 = v33;
  v4._8_8_ = 0;
  v4._0_8_ = v33;
  dat_26200 = (unsigned long)(v33 != (SUB168(v4._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v33 / 3) + v33 / 3;
  if (dat_26338 - 2 <= 2) {
    if (0 <= (long)v35) { // branch-flip
label_59f4:
      dat_262c0 = v35;
    }
    else {
      dat_262c0 = (void *)0x8;
      v24 = getenv("TABSIZE");
      if (v24) {
        if (!sub_18de0(v24,0,0,&v21,0x1bb19)) {
          v35 = v21;
          goto label_59f4;
        }
        v18 = sub_174c0(v24);
        error(0,0,dcgettext(NULL,"ignoring invalid tab size in environment variable TABSIZE: %s",5),v18);
      }
    }
  }
  v10 = (unsigned char)v36;
  if ((v36 == -1) && (v10 = 0, dat_251e0 == 1))
    v10 = sub_6d10();
  dat_262d8 = v10 & 1;
  if (0 <= v31) { // branch-flip
label_55eb:
    sub_16950(0,v31); // crossjump-dupe
  }
  else {
    v24 = getenv("QUOTING_STYLE");
    if (v24) {
      v11 = sub_e9c0(v24,0x24a20,0x1e9c0,4);
      if (0 <= v11) { // branch-flip
        v31 = *(int *)((long)v11 * 4 + 0x1e9c0);
        if (0 <= v31) goto label_55eb;
      }
      else {
        v18 = sub_174c0(v24);
        error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v18);
      }
    }
    v31 = 7;
    if (dat_251e0 != 1) goto label_55eb;
    if (sub_6d10()) {
      v31 = 3;
      sub_16950(0,v31);
    }
  }
  v11 = sub_16930(0);
  if (((dat_26338) && ((2 <= dat_26338 - 2 || (!dat_262b0)))) || ((v11 != 3 && ((v11 != 6 && (v11 != 1)))))) {
    dat_263a8 = 0;
    dat_262d0 = sub_168f0(0);
    if (v11 == 7)
      sub_16970(dat_262d0,0x20,1);
  }
  else {
    dat_263a8 = 1;
    dat_262d0 = sub_168f0(0);
  }
  if (2 <= dat_26314) {
    v24 = &"*=>@|"[dat_26314 - 2];
    v2 = "*=>@|"[dat_26314 - 2];
    while (v2) {
      v24 = &v24[1];
      sub_16970(dat_262d0,(int)v2,1);
      v2 = *v24;
    }
  }
  dat_262c8 = sub_168f0(0);
  sub_16970(dat_262c8,0x3a,1);
  dat_26318 = (dat_26311 ^ 1) & dat_26338 == 0 & dat_26318;
  if ((int)dat_25019 < (int)(unsigned int)dat_26318) {
    error(2,0,dcgettext(NULL,"--dired and --zero are incompatible",5));
    return v20;
  }
  if (0 <= v32) { // branch-flip
label_56f1:
    dat_26330 = v32;
    if (!dat_26338) {
label_5a12:
      if (!v34) {
        v34 = getenv("TIME_STYLE");
        if (!v34)
          v34 = "locale";
      }
      while (!strncmp(v34,"posix-",6)) {
        if (!sub_10260(2)) goto label_56ff;
        v34 = &v34[6];
      }
      if (*v34 != '+') { // branch-flip
        v26 = (long *)0x249c0;
        v25 = sub_e9c0(v34,0x249c0,0x1a780,4);
        if (v25 < 0) {
          sub_eb30("time style",v34,v25);
          v15 = stderr;
          fputs_unlocked(dcgettext(NULL,"Valid arguments are:\n",5),v15);
          v24 = "full-iso";
          do {
            v26 = &v26[1];
            __fprintf_chk(stderr,1,"  - [posix-]%s\n",v24);
            v15 = stderr;
            v24 = (char *)*v26;
          } while (v24);
          fputs_unlocked(dcgettext(NULL,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n",5),v15);
label_65b3:
          sub_df00(2); // no-return
        }
        switch(v25) { // branch-flip
          case 0:
            dat_25048 = "%Y-%m-%d %H:%M:%S.%N %z";
            dat_25040 = "%Y-%m-%d %H:%M:%S.%N %z";
            break;
          case 1:
            dat_25048 = "%Y-%m-%d %H:%M";
            dat_25040 = "%Y-%m-%d %H:%M";
            break;
          case 2:
            dat_25040 = "%Y-%m-%d ";
            dat_25048 = "%m-%d %H:%M";
            break;
          case 3:
            if (sub_10260(2)) {
              dat_25040 = dcgettext(NULL,dat_25040,2);
              dat_25048 = dcgettext(NULL,dat_25048,2);
            }
          
        }
      }
      else {
        v34 = &v34[1];
        v24 = strchr(v34,10);
        if (v24) { // branch-flip
          if (strchr(&v24[1],10)) {
            v18 = sub_174c0(v34);
            error(2,0,dcgettext(NULL,"invalid time style format %s",5),v18);
            return v19;
          }
          *v24 = '\0';
          dat_25048 = &v24[1];
        }
        else {
          dat_25048 = v34;
        }
        dat_25040 = v34;
      }
      sub_6d40();
    }
  }
  else {
    if (!dat_26338) {
      dat_26330 = 0;
      goto label_5a12;
    }
    if (3 <= (unsigned int)(dat_26334 - 1U)) {
      v32 = 0;
      goto label_56f1;
    }
    dat_26330 = 5;
  }
label_56ff:
  v11 = optind;
  if (dat_26312) { // branch-flip
    v23 = getenv("LS_COLORS");
    if ((v23) && (*v23)) {
      v22 = 0x3f3f;
      v38 = 0;
      dat_26300 = (void *)sub_18a70(v23);
      v21 = dat_26300;
      do {
        while( true ) {
          while (v2 = *v23, v2 == '*') {
            v24 = &v23[1];
            v15 = (void *)sub_18590(0x28);
            *(void **)((long)v15 + 0x20) = dat_26308;
            *(void **)((long)v15 + 8) = v21;
            dat_26308 = v15;
            if ((!sub_6960(&v21,&v23,1,v15)) || (v24 = &v23[1], *v23 != '=')) goto label_6174;
            *(void **)((long)v15 + 0x18) = v21;
            if (!sub_6960(&v21,&v23,0,(long)v15 + 0x10)) goto label_6174;
          }
          if (v2 != ':') break;
          v23 = &v23[1]; // branch-flip
        }
        if (!v2) goto label_61c9;
        v22 = CONCAT11(v22._1_1_,v2);
        v24 = &v23[1];
        if (!v23[1]) goto label_6174;
        v22 = CONCAT11(v23[1],v2);
        v24 = &v23[3];
        if (v23[2] != '=') goto label_6174;
        v25 = 0;
        v27 = "lc";
        while (strcmp((char *)&v22,v27)) {
          v25 += 1;
          v27 = *(char **)(v25 * 8 + 0x248a0);
          if (!v27) goto label_6141;
        }
        v25 = (long)(int)v25 * 0x10;
        *(void **)(v25 + 0x25068) = v21;
      } while (sub_6960(&v21,&v23,0,v25 + 0x25060));
label_6141:
      v18 = sub_174c0(&v22);
      error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),v18);
label_6174:
      error(0,0,dcgettext(NULL,"unparsable value for LS_COLORS environment variable",5));
      free(dat_26300);
      while (v5) {
        v15 = *(void **)((long)v5 + 0x20);
        free(v5);
      }
      dat_26312 = 0;
label_61c9:
      if ((dat_250d0 == 6) && (v13 = strncmp(dat_250d8,"target",6), !v13))
        dat_26390 = '\x01';
    }
    else {
      v24 = getenv("COLORTERM");
      if ((!v24) || (!*v24)) {
        v24 = getenv("TERM");
        if ((v24) && (*v24)) {
          v27 = "# Configuration file for dircolors, a utility to help you set the";
          do {
            if ((!strncmp(v27,"TERM ",5)) && (!fnmatch(&v27[5],v24,0))) goto label_61d7;
            v27 = &v27[strlen(v27) + 1];
          } while (&v27[-0x1a7a0] <= "loc");
        }
        dat_26312 = 0;
      }
    }
label_61d7:
    if (!dat_26312) goto label_5713;
    dat_262c0 = NULL;
    if ((((dat_262f4) || (sub_6c80(0xd))) || ((sub_6c80(0xe) && (dat_26390)))) || ((sub_6c80(0xc) && (!dat_26338)))) goto label_571c;
  }
  else {
label_5713:
    if (dat_262f4) {
label_571c:
      dat_262fd = 1;
    }
  }
  v25 = (long)v11;
  if (((!dat_262f8) && (dat_262f8 = 1, !dat_262f5)) && (dat_26314 != 3))
    dat_262f8 = (-(unsigned int)(dat_26338 == 0) & 0xfffffffe) + 3;
  if (dat_262f6) {
    dat_263c8 = sub_10c60(0x1e,0,sub_6910,sub_6920,sub_6bd0);
    if (!dat_263c8)
      sub_18a90(); // no-return
    _obstack_begin(0x260e0,0,0,dat_24fe8,dat_24fb0);
  }
  dat_262a8 = sub_17950(getenv("TZ"));
  v9 = dat_24fe8;
  v18 = dat_24fb0;
  if (((dat_26330 - 3U & 0xfffffffdU) && (dat_26338)) && ((!dat_26365 && (!dat_2632c)))) {
    dat_262a1 = 0;
    dat_262a0 = 1;
    if (((!dat_262f6) && (!dat_26312)) && (!dat_26314)) {
    }
  }
  else {
    dat_262a1 = 1;
    dat_262a0 = 0;
  }
  dat_262a0 &= 1;
  if (dat_26318) {
    _obstack_begin(0x261a0,0,0,dat_24fe8,dat_24fb0);
    _obstack_begin(0x26140,0,0,v9,v18);
  }
  if (dat_26311) {
    v17 = 0;
    do {
      while (v13 = (int)v17, v17 <= 0x5a) {
        v29 = 1;
        if ((v13 <= 0x40) && (10 <= (unsigned int)(v13 - 0x30U))) goto label_5d58;
label_5d70:
        *(unsigned char *)(v17 + 0x25fe0) = *(unsigned char *)(v17 + 0x25fe0) | v29;
        v17 += 1;
        if (v17 == 0x100) goto label_5db0;
      }
      v29 = 1;
      if (0x1a > (unsigned int)(v13 - 0x61U)) goto label_5d70;
label_5d58:
      if (((unsigned int)(v13 - 0x2dU) > 1) && (v13 != 0x7e)) {
        v29 = v13 == 0x5f;
        goto label_5d70;
      }
      *(unsigned char *)(v17 + 0x25fe0) = *(unsigned char *)(v17 + 0x25fe0) | 1;
      v17 += 1;
    } while (v17 != 0x100);
label_5db0:
    dat_26388 = sub_18c10();
    if (!dat_26388)
      dat_26388 = 0x1bb19;
  }
  dat_263b8 = 100;
  dat_263c0 = sub_186d0(100,0xd0);
  dat_263b0 = 0;
  sub_8730();
  if (1 <= (int)(a0 - v11)) { // branch-flip
    do {
      v1 = &a1[v25];
      v25 += 1;
      sub_96f0(*v1,0,1,0x1bb19);
    } while ((int)v25 < (int)a0);
    if (!dat_263b0) {
label_587e:
      v26 = dat_26380;
      if (1 < (int)(a0 - v11)) goto label_58c5;
      goto label_5f36;
    }
label_5b72:
    sub_8e30();
    if (!dat_262f5)
      sub_9510(0,1);
    if (!dat_263b0) goto label_587e;
    sub_d420();
    if (!dat_26380) goto label_5ba8;
    dat_261f8 += 1;
    v3 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v3) {
      __overflow(stdout,10);
      v26 = dat_26380;
      goto label_58c5;
    }
    *(char **)((long)stdout + 0x28) = &v3[1];
    *v3 = 10;
    v26 = dat_26380;
  }
  else {
    if (dat_262f5) // branch-flip
      sub_96f0(".",3,1,0x1bb19);
    else {
      sub_6fb0(".",0,1);
    }
    if (dat_263b0) goto label_5b72;
label_5f36:
    if (!dat_26380) goto label_5ba8;
    v26 = dat_26380;
    if (!dat_26380[3])
      dat_262b8 = 0;
  }
  do {
    dat_26380 = (long *)v26[3];
    if ((dat_263c8) && (!*v26)) {
      if ((unsigned long)(dat_260f8 - dat_260f0) <= 0xf)
        __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
      v25 = dat_260f8 + -0x10;
      v15 = *(void **)(dat_260f8 + -0x10);
      v18 = *(unsigned long *)(dat_260f8 + -8);
      dat_260f8 = v25;
      v15 = (void *)sub_11320(dat_263c8,&v21);
      if (!v15)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      free(v15);
      free((void *)*v26);
      free((void *)v26[1]);
      free(v26);
    }
    else {
      sub_d820(*v26,v26[1],(char)v26[2]);
      free((void *)*v26);
      free((void *)v26[1]);
      free(v26);
      dat_262b8 = 1;
    }
    v26 = dat_26380;
label_58c5:
  } while (v26);
label_5ba8:
  if ((dat_26312) && (dat_26310)) {
    if ((dat_25060 != 2) || (((v11 = memcmp(dat_25068,(void *)0x1bdf7,2), v11 || (dat_25070 != 1)) || (*dat_25078 != 'm')))) {
      sub_7880(0x25060);
      sub_7880(0x25070);
    }
    fflush_unlocked(stdout);
    sub_76b0(0);
    for (; v11; v11 = v11 + -1) {
      raise(0x13);
    }
    if (dat_26218)
      raise(dat_26218);
  }
  if (dat_26318) {
    sub_7450("//DIRED//",0x261a0);
    sub_7450("//SUBDIRED//",0x26140);
    v12 = sub_16930(dat_262d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",*(unsigned long *)((unsigned long)v12 * 8 + 0x24a20));
  }
  v25 = dat_263c8;
  if (dat_263c8) {
    v16 = sub_10790(dat_263c8);
    if (v16)
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_10e20(v25);
  }
  if (v39 != *(long *)(v28 + 0x28))
    __stack_chk_fail(); // no-return
  return (unsigned long)dat_26210;
label_5463:
  v34 = optarg;
  goto label_4e20;
}


// Function: sub_6910 @ 0x6910
unsigned long sub_6910(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_6920 @ 0x6920
unsigned long sub_6920(long *a0,long *a1)
{
  if (*a0 != *a1)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1[1] >> 8),a0[1] == a1[1]);
}


// Function: sub_6940 @ 0x6940
void sub_6940(unsigned int a0) // return-dupe
{
  if (dat_26218)
    return;
  dat_26218 = a0;
}


// Function: sub_6960 @ 0x6960
unsigned int sub_6960(long *a0,long *a1,unsigned int a2,long *a3)
{
  unsigned char v1;
  long v10;
  long v11;
  char *v12;
  bool v13;
  unsigned int v2; // eax
  char *v3;
  char *v4;
  char v5;
  char v6;
  long v7;
  long v8;
  long v9; // rdi
  
  v3 = (char *)*a1;
  v11 = *a0;
  v9 = 1;
  v5 = *v3;
  v10 = 0;
  v8 = v11 + 1;
  v6 = v5 + '\xa4';
  v13 = 0;
  v4 = v3;
  v7 = v8;
  if (v5 == '\\') goto label_69c6;
label_699a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_69fe:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_69fe;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_69b6;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_69b6;
    }
    v2 = 0;
    goto label_69fe;
  }
  v3 = &v3[1];
  v8 = v7;
label_69ab:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_69b6:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_699a;
label_69c6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_69fe;
    }
    v5 = v6 + '\xd0';
    switch(v5) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
      case 6:
      case 7:
        v6 = *v3;
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_69ab;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_69b6;
      default:
        goto label_6a40;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_6ad7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_6a40;
      case 0x32:
        v6 = '\b';
        goto label_6a40;
      case 0x35:
        v6 = '\x1b';
        break;
      case 0x36:
        v6 = '\f';
        break;
      case 0x3e:
        v6 = '\n';
        break;
      case 0x42:
        v6 = '\r';
        break;
      case 0x44:
        v6 = '\t';
        break;
      case 0x46:
        v6 = '\v';
label_6a40:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_6ad7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_6b22;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_69ab;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_6b22:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_69ab;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}


// Function: sub_6be0 @ 0x6be0
void sub_6be0(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_18590(0x10);
  *v1 = a0;
  v1[1] = dat_262e8;
  dat_262e8 = v1;
}


// Function: sub_6c10 @ 0x6c10
long sub_6c10(unsigned long a0) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_18de0(a0,0,0,&v2,0x1bb19);
  if (v1) // branch-flip
    v2 = -(unsigned long)(v1 != 1);
  else if (v2 <= -1)
    return 0;
  return v2;
}


// Function: sub_6c80 @ 0x6c80
unsigned long sub_6c80(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned long v3;
  
  v3 = 0;
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x25060);
  if (!v1)
    return 0;
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x25068);
  if (v1 != 1) {
    v3 = 0;
    if (v1 != 2)
      return 1;
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"00",2) != 0) & 0xffffffff;
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"0",1) != 0) & 0xffffffff;
}


// Function: sub_6d10 @ 0x6d10
unsigned int sub_6d10(void)
{
  unsigned int v1; // eax
  
  if ('\0' <= (char)dat_25018)
    return dat_25018 & 1;
  v1 = isatty(1);
  dat_25018 = (char)v1;
  return v1 & 1;
}


// Function: sub_6d40 @ 0x6d40
void sub_6d40(void) // return-dupe
{
  unsigned short v1;
  long v10 [2]; // stack - 0x658
  char v11 [1536];
  unsigned long v12; // stack - 0x660
  char v13 [8];
  unsigned long v14;
  long v15;
  int v16; // r12d
  unsigned long v17;
  long v18; // stack - 0x678
  long v2;
  char v3;
  unsigned int v4; // eax
  char *v5;
  unsigned char *v6; // rax
  long *v7; // rax
  char v8;
  char *v9;
  
  v15 = 0;
  do {
    v5 = *(char **)(v15 + 0x25040);
    v3 = *v5;
    while (v3) {
      v8 = v5[1];
      if (v3 == '%') {
        if (v8 != '%') { // branch-flip
          if (v8 == 'b') goto label_6dae;
        }
        else {
          v8 = v5[2];
          v5 = &v5[1];
        }
      }
      v5 = &v5[1];
      v3 = v8;
    }
    v5 = NULL;
label_6dae:
    *(char **)((long)v10 + v15) = v5;
    v15 += 8;
    if (v15 == 0x10) {
      if ((!v10[0]) && (!v10[1]))
        return;
      v17 = 0xc;
      while( true ) {
        v16 = 0x2000e;
        v14 = 0;
        v9 = v11;
        do {
          v12 = v17;
          v6 = (unsigned char *)nl_langinfo(v16);
          if (strchr((char *)v6,0x25))
            return;
          v7 = __ctype_b_loc();
          v1 = *(unsigned short *)(*v7 + (unsigned long)*v6 * 2);
          if (0x80 <= (unsigned long)sub_12810(v6,v9,0x80,&v12,v1 >> 0xb & 1,0))
            return;
          if (v14 < v12)
            v14 = v12;
          v16 += 1;
          v9 = &v9[0x80];
        } while (v9 != v13);
        if (v17 <= v14) break;
        v17 = v14;
      }
      v18 = 0;
      do {
        v15 = *(long *)(v18 * 8 + 0x25040);
        v2 = v10[v18];
        v5 = (char *)(v18 * 0x600 + 0x253e0);
        v9 = v11;
        do {
          if (v2) { // branch-flip
            if (0x80 < v2 - v15)
              return;
            v4 = __snprintf_chk(v5,0x80,1,0xffffffffffffffff,"%.*s%s%s",v2 - v15,v15,v9,v2 + 2);
          }
          else {
            v4 = snprintf(v5,0x80,"%s",v15);
          }
          if (0x80 <= v4)
            return;
          v9 = &v9[0x80];
          v5 = &v5[0x80];
        } while (v9 != v13);
        if (v18 == 1) {
          dat_253c8 = 1;
          return;
        }
        v18 = 1;
      } while( true );
    }
  } while( true );
}


// Function: sub_7050 @ 0x7050
void sub_7050(unsigned long a0,unsigned long a1,unsigned long *a2,unsigned int a3,unsigned int a4) // return-dupe x3
{
  long v1;
  unsigned long v10; // stack - 0xe8
  unsigned int v11; // stack - 0xe0
  unsigned long v12; // stack - 0xd8
  unsigned int v13; // stack - 0xd0
  unsigned long v14; // stack - 0xc8
  unsigned int v15; // stack - 0xc0
  unsigned long v16; // stack - 0xb8
  unsigned int v17; // stack - 0xb0
  unsigned int v18; // stack - 0xa8
  unsigned int v19; // stack - 0xa4
  unsigned char v2; // stack - 0x127
  unsigned int v20; // stack - 0xa0
  unsigned int v21; // stack - 0x9c
  unsigned int v3; // stack - 0x124
  unsigned int v4; // stack - 0x118
  unsigned long v5; // stack - 0x114
  unsigned short v6; // stack - 0x10c
  unsigned long v7; // stack - 0x108
  unsigned long v8; // stack - 0x100
  unsigned long v9; // stack - 0xf8
  
  if ((int)statx(a0,a1,a3 | 0x800,a4,&v1) <= -1)
    return;
  *a2 = ((unsigned long)v21 & 0xffffff00) << 0xc | ((unsigned long)v20 & 0xfffff000) << 0x20 | (unsigned long)((v20 & 0xfff) << 8) | (unsigned long)(unsigned char)v21;
  a2[2] = (unsigned long)v4;
  a2[1] = v7;
  *(unsigned int *)&a2[3] = (unsigned int)v6;
  *(unsigned long *)((long)a2 + 0x1c) = v5;
  a2[5] = ((unsigned long)v19 & 0xffffff00) << 0xc | ((unsigned long)v18 & 0xfffff000) << 0x20 | (unsigned long)((v18 & 0xfff) << 8) | (unsigned long)(unsigned char)v19;
  a2[7] = (unsigned long)v3;
  a2[6] = v8;
  a2[10] = (unsigned long)v11;
  a2[8] = v9;
  a2[0xc] = (unsigned long)v17;
  a2[9] = v10;
  a2[0xe] = (unsigned long)v15;
  a2[0xb] = v16;
  a2[0xd] = v14;
  if (!(a4 & 0x800))
    return;
  if (v2 & 8) {
    a2[0xb] = v12;
    a2[0xc] = (unsigned long)v13;
    return;
  }
  a2[0xc] = 0xffffffffffffffff;
  a2[0xb] = 0xffffffffffffffff;
}


// Function: sub_72a0 @ 0x72a0
unsigned char * sub_72a0(unsigned char *a0,char a1)
{
  unsigned char v1;
  unsigned char *v2; // rax
  unsigned char *v3;
  unsigned char *v4; // rbp
  
  v2 = (unsigned char *)sub_186d0(3,strlen((char *)a0) + 1);
  v1 = *a0;
  v3 = v2;
  do {
    while( true ) {
      if (!v1) {
        *v3 = 0;
        return v2;
      }
      a0 = &a0[1];
      if ((v1 != 0x2f) || (!a1)) break;
      *v3 = 0x2f;
label_72ef:
      v3 = &v3[1];
      v1 = *a0;
    }
    if (*(char *)((unsigned long)v1 + 0x25fe0)) {
      *v3 = v1;
      goto label_72ef;
    }
    v4 = &v3[3];
    __sprintf_chk((char *)v3,1,0xffffffffffffffff,"%%%02x",(unsigned long)v1);
    v1 = *a0;
    v3 = v4;
  } while( true );
}


// Function: sub_7360 @ 0x7360
char * sub_7360(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  int v2; // eax
  char *v3; // rax
  char *v4;
  char *v5;
  
  if (a1 <= a0)
    return v3;
  do {
    while( true ) {
      v1 = a0 + 1;
      v4 = *(char **)((long)stdout + 0x28);
      if ((!dat_262c0) || (a1 / dat_262c0 <= v1 / dat_262c0)) break;
      if (*(char **)((long)stdout + 0x30) <= v4) // branch-flip
        __overflow(stdout,9);
      else {
        *(char **)((long)stdout + 0x28) = &v4[1];
        *v4 = 9;
      }
      v4 = (char *)(a0 / dat_262c0);
      a0 = (dat_262c0 + a0) - a0 % dat_262c0;
      if (a1 <= a0)
        return v4;
    }
    if (*(char **)((long)stdout + 0x30) <= v4) { // branch-flip
      v2 = __overflow(stdout,0x20);
      v5 = (char *)CONCAT44(dat_4,v2);
    }
    else {
      v5 = &v4[1];
      *(char **)((long)stdout + 0x28) = v5;
      *v4 = 0x20;
    }
    a0 = v1;
  } while (v1 < a1);
  return v5;
}


// Function: sub_7430 @ 0x7430
void sub_7430(void *a0,unsigned long a1)
{
  dat_261f8 += a1;
  fwrite_unlocked(a0,1,a1,stdout); // tail-call
}


// Function: sub_7450 @ 0x7450
void sub_7450(char *a0,long a1) // return-dupe
{
  unsigned long v1;
  char *v2;
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  unsigned long *v6;
  
  v6 = *(unsigned long **)(a1 + 0x18);
  v5 = *(unsigned long **)(a1 + 0x10);
  if ((unsigned long)((long)v6 - (long)v5) <= 7)
    return;
  if (v6 == v5)
    *(unsigned char *)(a1 + 0x50) = *(unsigned char *)(a1 + 0x50) | 2;
  v3 = (long)v6 + *(unsigned long *)(a1 + 0x30) & ~*(unsigned long *)(a1 + 0x30);
  v4 = *(unsigned long *)(a1 + 0x20);
  if (v3 - *(long *)(a1 + 8) <= *(unsigned long *)(a1 + 0x20) - *(long *)(a1 + 8))
    v4 = v3;
  v6 = (unsigned long *)(((long)v6 - (long)v5 & 0xfffffffffffffff8U) + (long)v5);
  *(unsigned long *)(a1 + 0x18) = v4;
  *(unsigned long *)(a1 + 0x10) = v4;
  fputs_unlocked(a0,stdout);
  do {
    v1 = *v5;
    v5 = &v5[1];
    __printf_chk(1," %ld",v1);
  } while (v5 != v6);
  v2 = *(char **)((long)stdout + 0x28);
  if (v2 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}


// Function: sub_7540 @ 0x7540
void sub_7540(char a0,char *a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // rax
  
  v1 = sub_16dd0(4,a2);
  error(0,*__errno_location(),a1,v1);
  if (a0) {
    dat_26210 = 2;
    return;
  }
  if (dat_26210)
    return;
  dat_26210 = 1;
}


// Function: sub_7880 @ 0x7880
void sub_7880(unsigned long *a0)
{
  if (!dat_26310) {
    dat_26310 = '\x01';
    if (0 <= (int)tcgetpgrp(1))
      sub_76b0(1);
    if (dat_25088) // branch-flip
      sub_7880(0x25080);
    else {
      sub_7880(0x25060);
      sub_7880(0x25090);
      sub_7880(0x25070);
    }
  }
  fwrite_unlocked((void *)a0[1],*a0,1,stdout); // tail-call
}


// Function: sub_7d70 @ 0x7d70
long sub_7d70(char *a0,unsigned long a1,unsigned int a2)
{
  char v1 [8200];
  char *v2; // stack - 0x2038
  unsigned char v3; // stack - 0x2039
  long v4; // stack - 0x2030
  
  v2 = v1;
  sub_7920(&v2,a0,a1,a2,&v4,&v3);
  if ((v2 != v1) && (v2 != a0))
    free(v2);
  return (unsigned long)v3 + v4;
}


// Function: sub_7e10 @ 0x7e10
long sub_7e10(unsigned long *a0)
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4; // rax
  long v5;
  char *v6; // rax
  long v7;
  char v8 [664];
  
  if (dat_262fc) { // branch-flip
    if (dat_26338 == 4) {
      v6 = (char *)sub_12720(a0[4],v8);
      v7 = strlen(v6) + 1;
      goto label_7ee2;
    }
    v7 = (long)dat_26360 + 1;
    if (dat_2632c) {
label_7e61:
      v4 = (unsigned long)dat_2635c;
      goto label_7e6c;
    }
    if (dat_26365) goto label_7f29;
  }
  else {
    v7 = 0;
label_7ee2:
    if (dat_2632c) {
      if (dat_26338 != 4) goto label_7e61;
      v6 = "?";
      if (*(char *)&a0[0x17])
        v6 = (char *)sub_11550(a0[0xb],v8,dat_26328,0x200,dat_26320);
      v4 = strlen(v6);
label_7e6c:
      v7 += v4 + 1;
    }
    if (dat_26365) {
      if (dat_26338 != 4) { // branch-flip
label_7f29:
        v4 = (unsigned long)dat_26354;
      }
      else {
        v4 = strlen((char *)a0[0x16]);
      }
      v7 += v4 + 1;
      v5 = a0[0x19];
      goto label_7f41;
    }
  }
  v5 = a0[0x19];
label_7f41:
  if (!v5)
    v5 = sub_7d70(*a0,dat_262d0,*(unsigned int *)((long)a0 + 0xc4));
  v7 += v5;
  if (dat_26314) {
    v2 = *(unsigned int *)&a0[0x15];
    v3 = *(unsigned int *)&a0[6];
    v1 = *(char *)&a0[0x17];
    v7 = (v7 + 1) - (unsigned long)(sub_75b0(v1,v3,v2) == '\0');
  }
  return v7;
}


// Function: sub_7ff0 @ 0x7ff0
unsigned long sub_7ff0(char a0)
{
  unsigned long *v1;
  unsigned long v10;
  unsigned long v11;
  char *v12;
  unsigned long v13;
  unsigned long v14;
  bool v15; // zf
  unsigned long *v2;
  char v3 [16];
  char v4 [16];
  long v5; // rax
  unsigned long *v6;
  unsigned long *v7; // rax
  unsigned long v8; // rcx
  long v9;
  
  v11 = dat_263b0;
  v14 = dat_263b0;
  if (dat_26200) { // branch-flip
    if (dat_263b0 <= dat_26200) { // branch-flip
      if (dat_263b0 <= dat_252e0) goto label_827c;
      if (dat_263b0 < dat_26200 >> 1) goto label_8035;
    }
    else {
      v14 = dat_26200;
      if (dat_26200 <= dat_252e0) goto label_80ec;
      v14 = dat_26200;
      if (dat_26200 < dat_26200 >> 1) goto label_8035;
    }
    dat_26208 = (char *)sub_18650(dat_26208,dat_26200,0x18);
    v13 = dat_26200;
label_8054:
    v3._8_8_ = 0;
    v3._0_8_ = v13 - dat_252e0;
    v4._8_8_ = 0;
    v4._0_8_ = dat_252e0 + 1 + v13;
    if ((CARRY8(dat_252e0 + 1,v13)) || (SUB168(v3._0_16_ * v4._0_16_,8)))
      sub_18a90(); // no-return
    v5 = sub_186d0(SUB168(v3._0_16_ * v4._0_16_,0) >> 1,8);
    if (dat_252e0 < v13) {
      v9 = dat_252e0 * 8 + 8;
      do {
        *(long *)&dat_26208[v9 * 3 + -8] = v5;
        v5 += v9;
        v9 += 8;
      } while (v13 * 8 + 8 != v9);
    }
    v11 = dat_263b0;
    dat_252e0 = v13;
    if (!v14) goto label_8288;
label_80ec:
    v5 = 3;
    v13 = 0;
    do {
      v2 = *(unsigned long **)&dat_26208[v5 * 8 + -8];
      dat_26208[v5 * 8 + -0x18] = '\x01';
      *(long *)&dat_26208[v5 * 8 + -0x10] = v5;
      v6 = v2;
      do {
        *v6 = 3;
        v7 = &v6[1];
        v15 = &v2[v13] != v6;
        v6 = v7;
      } while (v15);
      v13 += 1;
      v5 += 3;
    } while (v13 < v14);
    if (!v11) goto label_823d;
  }
  else {
    if (dat_252e0 < dat_263b0) {
label_8035:
      dat_26208 = (char *)sub_18650(dat_26208,v14,0x30);
      v13 = v14 * 2;
      goto label_8054;
    }
label_827c:
    if (dat_263b0) goto label_80ec;
label_8288:
    if (!dat_263b0)
      return v14;
  }
  v11 = 0;
  do {
    v5 = sub_7e10(*(unsigned long *)(dat_263a0 + v11 * 8));
    if (v14) {
      v13 = 0;
      v12 = dat_26208;
      do {
        v8 = v13 + 1;
        if (*v12) {
          if (a0) // branch-flip
            v10 = v11 / (((dat_263b0 - 1) + v8) / v8);
          else {
            v10 = v11 % v8;
          }
          v1 = (unsigned long *)(*(long *)&v12[0x10] + v10 * 8);
          v13 = v5 + (unsigned long)(v10 != v13) * 2;
          v10 = *v1;
          if (v10 < v13) {
            *(unsigned long *)&v12[8] = *(long *)&v12[8] + (v13 - v10);
            *v1 = v13;
            *v12 = *(unsigned long *)&v12[8] < dat_262b0;
          }
        }
        v12 = &v12[0x18];
        v13 = v8;
      } while (v14 != v8);
    }
    v11 += 1;
  } while (v11 < dat_263b0);
label_823d:
  if (2 <= v14) {
    v12 = &dat_26208[v14 * 0x18 + -0x18];
    v11 = v14;
    do {
      if (*v12)
        return v11;
      v11 -= 1;
      v12 = &v12[-0x18];
      v14 = 1;
    } while (v11 != 1);
  }
  return v14;
}


// Function: sub_84f0 @ 0x84f0
void sub_84f0(void)
{
  char v1 [136];
  int v2;
  
  while ((dat_26218 || (dat_26214))) {
    if (dat_26310) {
      sub_7880(0x25060);
      sub_7880(0x25070);
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(void *)0x26220,v1);
    v2 = dat_26218;
    if (dat_26214) { // branch-flip
      dat_26214 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_26218,0);
    }
    raise(v2);
    sigprocmask(2,v1,NULL);
  }
}


// Function: sub_85f0 @ 0x85f0
int sub_85f0(unsigned long a0) // early-return
{
  unsigned int v1; // eax
  long v2; // rax
  
  if ((!dat_2632d) && (v2 = sub_12340(a0), v2)) {
    v1 = sub_12ec0(v2,0);
    if (0 <= (int)v1)
      return (unsigned long)v1;
    return 0;
  }
  return __snprintf_chk(NULL,0,1,0xffffffffffffffff,"%lu",a0 & 0xffffffff); // tail-call
}


// Function: sub_8640 @ 0x8640
unsigned int sub_8640(void)
{
  unsigned int v1; // eax
  unsigned int v2;
  
  v1 = (-(unsigned int)(dat_262fc == '\0') & 0xffffff00) + 0x102;
  if (dat_2632c)
    v1 |= 0x400;
  if (dat_26338) { // branch-flip
    switch(dat_26330) {
      case 0:
      case 1:
      case 2:
      case 4:
      case 6:
label_86f0:
        return v1;
      case 3:
label_86f8:
        return v1 | 0x200;
      case 5:
        if (dat_26334 <= 3) {
          v2 = *(unsigned int *)&"@"[(unsigned long)dat_26334 * 4];
          return v1 | v2; // return-dupe
        }
      
    }
  }
  else if (dat_26334 <= 3) {
    v2 = *(unsigned int *)&"@"[(unsigned long)dat_26334 * 4];
    if ((dat_25029) || (dat_2632e))
      v1 = v1 | v2 | 0x20c;
    else {
      v1 = v1 | v2 | 0x204;
    }
    if (dat_25028)
      v1 |= 0x10;
    switch(dat_26330) {
      case 0:
      case 1:
      case 2:
      case 4:
      case 6:
        goto label_86f0;
      case 3:
        goto label_86f8;
      case 5:
        return v1 | v2;
      
    }
  }
  abort(); // no-return
}


// Function: sub_8730 @ 0x8730
void sub_8730(void)
{
  unsigned long *v1;
  unsigned long v2; // rbp
  
  if (dat_263b0) {
    v2 = 0;
    do {
      v1 = *(unsigned long **)(dat_263a0 + v2 * 8);
      free((void *)*v1);
      free((void *)v1[1]);
      free((void *)v1[2]);
      if (v1[0x16] != 0x2502a)
        freecon();
      v2 += 1;
    } while (v2 < dat_263b0);
  }
  dat_263a9 = 0;
  dat_263b0 = 0;
  dat_26364 = 0;
  dat_26360 = 0;
  dat_2635c = 0;
  dat_26358 = 0;
  dat_26350 = 0;
  dat_2634c = 0;
  dat_26348 = 0;
  dat_26354 = 0;
  dat_26344 = 0;
  dat_26340 = 0;
  dat_2633c = 0;
}


// Function: sub_8980 @ 0x8980
void sub_8980(unsigned long *a0,unsigned long *a1)
{
  long v1;
  long v2;
  
  v2 = a0[0x19];
  if (!v2)
    v2 = sub_7d70(*a0,dat_262d0,*(unsigned int *)((long)a0 + 0xc4));
  v1 = a1[0x19];
  if (!v1)
    v1 = sub_7d70(*a1,dat_262d0,*(unsigned int *)((long)a1 + 0xc4));
  if ((int)v2 == (int)v1) {
    strcmp((char *)*a0,(char *)*a1); // tail-call
    return;
  }
}


// Function: sub_8a00 @ 0x8a00
void sub_8a00(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a0[0xd] < (long)a1[0xd]) - (unsigned int)((long)a1[0xd] < (long)a0[0xd])) + ((unsigned int)((long)a0[0xc] < (long)a1[0xc]) - (unsigned int)((long)a1[0xc] < (long)a0[0xc])) * 2)) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8ae0 @ 0x8ae0
void sub_8ae0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a1[0x11] < (long)a0[0x11]) - (unsigned int)((long)a0[0x11] < (long)a1[0x11])) + ((unsigned int)((long)a1[0x10] < (long)a0[0x10]) - (unsigned int)((long)a0[0x10] < (long)a1[0x10])) * 2)) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8b60 @ 0x8b60
unsigned long sub_8b60(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8ae0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8ae0(a0,a1);
}


// Function: sub_8bf0 @ 0x8bf0
void sub_8bf0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a1[0xf] < (long)a0[0xf]) - (unsigned int)((long)a0[0xf] < (long)a1[0xf])) + ((unsigned int)((long)a1[0xe] < (long)a0[0xe]) - (unsigned int)((long)a0[0xe] < (long)a1[0xe])) * 2)) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8cd0 @ 0x8cd0
void sub_8cd0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a0[0xf] < (long)a1[0xf]) - (unsigned int)((long)a1[0xf] < (long)a0[0xf])) + ((unsigned int)((long)a0[0xe] < (long)a1[0xe]) - (unsigned int)((long)a1[0xe] < (long)a0[0xe])) * 2)) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8d40 @ 0x8d40
void sub_8d40(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a0[0x11] < (long)a1[0x11]) - (unsigned int)((long)a1[0x11] < (long)a0[0x11])) + ((unsigned int)((long)a0[0x10] < (long)a1[0x10]) - (unsigned int)((long)a1[0x10] < (long)a0[0x10])) * 2)) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8dc0 @ 0x8dc0
void sub_8dc0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)((long)a1[0xd] < (long)a0[0xd]) - (unsigned int)((long)a0[0xd] < (long)a1[0xd])) + ((unsigned int)((long)a1[0xc] < (long)a0[0xc]) - (unsigned int)((long)a0[0xc] < (long)a1[0xc])) * 2)) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_8e30 @ 0x8e30
void sub_8e30(void) // return-dupe
{
  unsigned long *v1;
  unsigned int v2;
  int v3;
  long *v4;
  long v5;
  unsigned long v6;
  
  v6 = dat_263b0;
  if (dat_26398 < (dat_263b0 >> 1) + dat_263b0) {
    free(dat_263a0);
    dat_263a0 = (long *)sub_186d0(v6,0x18);
    dat_26398 = dat_263b0 * 3;
  }
  if (dat_263b0) { // branch-flip
    v4 = dat_263a0;
    v5 = dat_263c0;
    do {
      *v4 = v5;
      v4 = &v4[1];
      v5 += 0xd0;
    } while (v4 != &dat_263a0[dat_263b0]);
    if (dat_26330 != 2) goto label_8ec2;
label_8fbb:
    v6 = 0;
    do {
      v1 = (unsigned long *)dat_263a0[v6];
      v5 = v1[0x19];
      if (!v5)
        v5 = sub_7d70(*v1,dat_262d0,*(unsigned int *)((long)v1 + 0xc4));
      v6 += 1;
      v1[0x19] = v5;
    } while (v6 < dat_263b0);
    v2 = dat_26330;
  }
  else {
    if (dat_26330 == 2) goto label_8ed9;
label_8ec2:
    v2 = dat_26330;
    if (((dat_262b0) && ((unsigned int)(dat_26338 - 2U) <= 1)) && (dat_263b0)) goto label_8fbb;
  }
  if (v2 == 6)
    return;
label_8ed9:
  if (_setjmp(0x25300)) { // branch-flip
    if (dat_26330 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    if (dat_263b0) {
      v4 = dat_263a0;
      v5 = dat_263c0;
      do {
        *v4 = v5;
        v4 = &v4[1];
        v5 += 0xd0;
      } while (&dat_263a0[dat_263b0] != v4);
    }
    v3 = 1;
  }
  else {
    v3 = 0;
  }
  v2 = dat_26330;
  if (dat_26330 == 5)
    v2 = dat_26334 + 5;
  sub_131b0(dat_263a0,dat_263b0,*(unsigned long *)(((unsigned long)dat_262f4 + ((unsigned long)dat_2632f + ((long)v3 + (unsigned long)v2 * 2) * 2) * 2) * 8 + 0x23fa0));
}


// Function: sub_91a0 @ 0x91a0
void sub_91a0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  int *v3; // rax
  long v4;
  long v5;
  
  v5 = a1[0x19];
  if (!v5)
    v5 = sub_7d70(*a1,dat_262d0,*(unsigned int *)((long)a1 + 0xc4));
  v4 = a0[0x19];
  if (!v4)
    v4 = sub_7d70(*a0,dat_262d0,*(unsigned int *)((long)a0 + 0xc4));
  if ((int)v5 == (int)v4) {
    v3 = __errno_location();
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    *v3 = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_9230 @ 0x9230
void sub_9230(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  int *v3; // rax
  long v4;
  long v5;
  
  v5 = a0[0x19];
  if (!v5)
    v5 = sub_7d70(*a0,dat_262d0,*(unsigned int *)((long)a0 + 0xc4));
  v4 = a1[0x19];
  if (!v4)
    v4 = sub_7d70(*a1,dat_262d0,*(unsigned int *)((long)a1 + 0xc4));
  if ((int)v5 == (int)v4) {
    v3 = __errno_location();
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    *v3 = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_9370 @ 0x9370
void sub_9370(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4; // rax
  
  v1 = (char *)*a0;
  v3 = strrchr(v1,0x2e);
  v2 = (char *)*a1;
  v4 = strrchr(v2,0x2e);
  if (!v4)
    v4 = "";
  if (!v3)
    v3 = "";
  *__errno_location() = 0;
  if (!strcoll(v3,v4)) {
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_9400 @ 0x9400
void sub_9400(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4; // rax
  
  v1 = (char *)*a1;
  v3 = strrchr(v1,0x2e);
  v2 = (char *)*a0;
  v4 = strrchr(v2,0x2e);
  if (!v4)
    v4 = "";
  if (!v3)
    v3 = "";
  *__errno_location() = 0;
  if (!strcoll(v3,v4)) {
    strcoll(v1,v2); // tail-call
    return;
  }
}


// Function: sub_9490 @ 0x9490
void sub_9490(unsigned long *a0,unsigned long *a1)
{
  long v1;
  long v2;
  
  v2 = a1[0x19];
  if (!v2)
    v2 = sub_7d70(*a1,dat_262d0,*(unsigned int *)((long)a1 + 0xc4));
  v1 = a0[0x19];
  if (!v1)
    v1 = sub_7d70(*a0,dat_262d0,*(unsigned int *)((long)a0 + 0xc4));
  if ((int)v2 == (int)v1) {
    strcmp((char *)*a1,(char *)*a0); // tail-call
    return;
  }
}


// Function: sub_9510 @ 0x9510
void sub_9510(long a0,char a1) // return-dupe
{
  int v1;
  char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  char *v5; // rax
  void *v6; // rax
  long *v7;
  long v8;
  bool v9; // cf
  
  if ((a0) && (dat_263c8)) {
    v3 = (unsigned long *)sub_18590(0x20);
    v4 = sub_18a70(a0);
    *v3 = 0;
    v3[1] = v4;
    *(char *)&v3[2] = 0;
    v3[3] = dat_26380;
    dat_26380 = v3;
  }
  v8 = dat_263b0 + -1;
  if (!dat_263b0) {
    dat_263b0 = 0;
    return;
  }
  do {
    while( true ) {
      v3 = (unsigned long *)dat_263a0[v8];
      if ((*(int *)&v3[0x15] == 3) || (*(int *)&v3[0x15] == 9)) break;
label_9588:
      v9 = v8 == 0;
      v8 -= 1;
      if (v9) goto label_9640;
    }
    v2 = (char *)*v3;
    if (a0) { // branch-flip
      v5 = (char *)sub_ee20(v2);
      if ((*v5 == '.') && ((!v5[(unsigned long)(v5[1] == '.') + 1] || (v5[(unsigned long)(v5[1] == '.') + 1] == '/')))) goto label_9588;
      if (*v2 == '/') goto label_96a0;
      v6 = (void *)sub_fbb0(a0,v2,0);
      sub_6fb0(v6,v3[1],a1);
      free(v6);
    }
    else {
label_96a0:
      sub_6fb0(v2,v3[1],a1);
    }
    if (*(int *)&v3[0x15] != 9) goto label_9588;
    free((void *)*v3);
    free((void *)v3[1]);
    free((void *)v3[2]);
    if (v3[0x16] == 0x2502a) goto label_9588;
    freecon();
    v9 = v8 != 0;
    v8 -= 1;
  } while (v9);
label_9640:
  if (!dat_263b0) {
    dat_263b0 = 0;
    return;
  }
  v8 = 0;
  v7 = dat_263a0;
  do {
    v1 = *(int *)(*v7 + 0xa8);
    dat_263a0[v8] = *v7;
    v7 = &v7[1];
    v8 += (unsigned long)(v1 != 9);
  } while (&dat_263a0[dat_263b0] != v7);
  dat_263b0 = v8;
}


// Function: sub_a660 @ 0xa660
unsigned long sub_a660(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_a6e0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_a6e0:
  if ((long)a0[9] < (long)a1[9])
    return 0xffffffff;
  if (a1[9] != a0[9])
    return 1;
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  return strcmp(v2,v1); // tail-call
}


// Function: sub_a710 @ 0xa710
unsigned long sub_a710(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_a771:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_a771;
  }
  v3 = ((unsigned int)((long)a1[0x11] < (long)a0[0x11]) - (unsigned int)((long)a0[0x11] < (long)a1[0x11])) + ((unsigned int)((long)a1[0x10] < (long)a0[0x10]) - (unsigned int)((long)a0[0x10] < (long)a1[0x10])) * 2;
  if (!v3) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_a7f0 @ 0xa7f0
unsigned long sub_a7f0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_a849:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_a849;
  }
  v3 = ((unsigned int)((long)a1[0xd] < (long)a0[0xd]) - (unsigned int)((long)a0[0xd] < (long)a1[0xd])) + ((unsigned int)((long)a1[0xc] < (long)a0[0xc]) - (unsigned int)((long)a0[0xc] < (long)a1[0xc])) * 2;
  if (!v3) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_a8c0 @ 0xa8c0
unsigned long sub_a8c0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) {
      v1 = (char *)*a1; // return-dupe
      v2 = (char *)*a0;
      return strcmp(v2,v1); // tail-call
    }
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  return strcmp(v2,v1);
}


// Function: sub_a960 @ 0xa960
unsigned long sub_a960(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_a9e0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_a9e0:
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  *__errno_location() = 0;
  return strcoll(v2,v1); // tail-call
}


// Function: sub_aa20 @ 0xaa20
unsigned long sub_aa20(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  unsigned long v5; // rax
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_aaa0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_aaa0:
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  v5 = sub_101e0(v1,v2);
  if (!(int)v5)
    return strcmp(v1,v2); // tail-call
  return v5;
}


// Function: sub_aaf0 @ 0xaaf0
unsigned long sub_aaf0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  unsigned long v5; // rax
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_ab70;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_ab70:
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  v5 = sub_101e0(v1,v2);
  if (!(int)v5)
    return strcmp(v1,v2); // tail-call
  return v5;
}


// Function: sub_abc0 @ 0xabc0
unsigned long sub_abc0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_ac40;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_ac40:
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  *__errno_location() = 0;
  return strcoll(v2,v1); // tail-call
}


// Function: sub_ac80 @ 0xac80
unsigned long sub_ac80(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8cd0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8cd0(a0,a1);
}


// Function: sub_ad10 @ 0xad10
unsigned long sub_ad10(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8d40(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8d40(a0,a1);
}


// Function: sub_ada0 @ 0xada0
unsigned long sub_ada0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9490(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9490(a0,a1);
}


// Function: sub_ae30 @ 0xae30
unsigned long sub_ae30(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_ae89:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_ae89;
  }
  v3 = ((unsigned int)((long)a1[0xf] < (long)a0[0xf]) - (unsigned int)((long)a0[0xf] < (long)a1[0xf])) + ((unsigned int)((long)a1[0xe] < (long)a0[0xe]) - (unsigned int)((long)a0[0xe] < (long)a1[0xe])) * 2;
  if (!v3) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_af00 @ 0xaf00
unsigned long sub_af00(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_af59:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_af59;
  }
  v3 = ((unsigned int)((long)a0[0xf] < (long)a1[0xf]) - (unsigned int)((long)a1[0xf] < (long)a0[0xf])) + ((unsigned int)((long)a0[0xe] < (long)a1[0xe]) - (unsigned int)((long)a1[0xe] < (long)a0[0xe])) * 2;
  if (!v3) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_afd0 @ 0xafd0
unsigned long sub_afd0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8bf0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8bf0(a0,a1);
}


// Function: sub_b060 @ 0xb060
unsigned long sub_b060(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9230(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9230(a0,a1);
}


// Function: sub_b0f0 @ 0xb0f0
unsigned long sub_b0f0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  unsigned long v3; // rax
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      v3 = sub_71f0(a0,a1,dat_24fd8); // tail-call, return-dupe
      return v3;
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  v3 = sub_71f0(a0,a1,dat_24fd8);
  return v3;
}


// Function: sub_b190 @ 0xb190
unsigned long sub_b190(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9400(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9400(a0,a1);
}


// Function: sub_b220 @ 0xb220
unsigned long sub_b220(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_b2a0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b2a0:
  if ((long)a1[9] < (long)a0[9])
    return 0xffffffff;
  if (a0[9] != a1[9])
    return 1;
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  return strcmp(v2,v1); // tail-call
}


// Function: sub_b2d0 @ 0xb2d0
unsigned long sub_b2d0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_b350;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b350:
  if ((long)a0[9] < (long)a1[9])
    return 0xffffffff;
  if (a1[9] != a0[9])
    return 1;
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  *__errno_location() = 0;
  return strcoll(v1,v2); // tail-call
}


// Function: sub_b3a0 @ 0xb3a0
unsigned long sub_b3a0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8980(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8980(a0,a1);
}


// Function: sub_b430 @ 0xb430
unsigned long sub_b430(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_91a0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_91a0(a0,a1);
}


// Function: sub_b4c0 @ 0xb4c0
unsigned long sub_b4c0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9370(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9370(a0,a1);
}


// Function: sub_b550 @ 0xb550
unsigned long sub_b550(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) {
      v1 = (char *)*a0; // return-dupe
      v2 = (char *)*a1;
      return strcmp(v2,v1); // tail-call
    }
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  return strcmp(v2,v1);
}


// Function: sub_b5f0 @ 0xb5f0
unsigned long sub_b5f0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_b670;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b670:
  if ((long)a1[9] < (long)a0[9])
    return 0xffffffff;
  if (a0[9] != a1[9])
    return 1;
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  *__errno_location() = 0;
  return strcoll(v1,v2); // tail-call
}


// Function: sub_b6c0 @ 0xb6c0
unsigned long sub_b6c0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b719:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b719;
  }
  v3 = ((unsigned int)((long)a0[0xd] < (long)a1[0xd]) - (unsigned int)((long)a1[0xd] < (long)a0[0xd])) + ((unsigned int)((long)a0[0xc] < (long)a1[0xc]) - (unsigned int)((long)a1[0xc] < (long)a0[0xc])) * 2;
  if (!v3) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_b790 @ 0xb790
unsigned long sub_b790(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8dc0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8dc0(a0,a1);
}


// Function: sub_b820 @ 0xb820
unsigned long sub_b820(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  unsigned long v3; // rax
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      v3 = sub_71f0(a1,a0,dat_24fd8); // tail-call, return-dupe
      return v3;
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  v3 = sub_71f0(a1,a0,dat_24fd8);
  return v3;
}


// Function: sub_b8c0 @ 0xb8c0
unsigned long sub_b8c0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b919:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b919;
  }
  v3 = ((unsigned int)((long)a1[0xf] < (long)a0[0xf]) - (unsigned int)((long)a0[0xf] < (long)a1[0xf])) + ((unsigned int)((long)a1[0xe] < (long)a0[0xe]) - (unsigned int)((long)a0[0xe] < (long)a1[0xe])) * 2;
  if (!v3) {
    v1 = (char *)*a0;
    v2 = (char *)*a1;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_b990 @ 0xb990
unsigned long sub_b990(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b9e9:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b9e9;
  }
  v3 = ((unsigned int)((long)a0[0xf] < (long)a1[0xf]) - (unsigned int)((long)a1[0xf] < (long)a0[0xf])) + ((unsigned int)((long)a0[0xe] < (long)a1[0xe]) - (unsigned int)((long)a1[0xe] < (long)a0[0xe])) * 2;
  if (!v3) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_baf0 @ 0xbaf0
unsigned long sub_baf0(long a0,long a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)(a0 + 0xa8);
  if (((*(int *)(a1 + 0xa8) != 3) && (*(int *)(a1 + 0xa8) != 9)) && ((*(unsigned int *)(a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8a00(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)(a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8a00(a0,a1);
}


// Function: sub_bc10 @ 0xbc10
unsigned long sub_bc10(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_bc71:
    v3 = v4 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_bc71;
  }
  v3 = ((unsigned int)((long)a0[0x11] < (long)a1[0x11]) - (unsigned int)((long)a1[0x11] < (long)a0[0x11])) + ((unsigned int)((long)a0[0x10] < (long)a1[0x10]) - (unsigned int)((long)a1[0x10] < (long)a0[0x10])) * 2;
  if (!v3) {
    v1 = (char *)*a1;
    v2 = (char *)*a0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}


// Function: sub_bcf0 @ 0xbcf0
long sub_bcf0(unsigned char *a0,unsigned long a1,unsigned int a2,long a3,char a4,long a5,long a6)
{
  unsigned char v1;
  unsigned char v10 [8200];
  unsigned char *v11; // stack - 0x2050
  unsigned char v12; // stack - 0x2051
  long v13;
  unsigned long v14;
  char v15;
  char v16;
  unsigned char *v2;
  char *v3;
  long v4;
  unsigned long v5; // rax
  void *v6; // rax
  char *v7; // rax
  char *v8; // rcx
  long *v9;
  
  v4 = a6;
  v11 = v10;
  v5 = sub_7920(&v11,a0,a1,a2,0,&v12);
  if ((v12) && (a4)) {
    dat_261f8 += 1;
    v3 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)((long)stdout + 0x28) = &v3[1];
      *v3 = 0x20;
    }
  }
  if (a3) {
    if (sub_6c80(4)) {
      sub_7880(0x25060);
      sub_7880(0x25070);
    }
    sub_7880(0x25060);
    sub_7880(a3);
    sub_7880(0x25070);
  }
  v16 = dat_263a9;
  v14 = v5;
  if (v4) { // branch-flip
    v15 = '\0';
    if ((dat_263a8) && (v15 = dat_263a9, dat_263a9)) {
      if (v12) { // branch-flip
        v13 = 0;
        v16 = '\0';
      }
      else {
        v1 = *v11;
        v2 = *(unsigned char **)((long)stdout + 0x28);
        v14 = v5 - 2;
        if (*(unsigned char **)((long)stdout + 0x30) <= v2) { // branch-flip
          __overflow(stdout,(unsigned int)v1);
          v13 = 1;
        }
        else {
          v13 = 1;
          *(unsigned char **)((long)stdout + 0x28) = &v2[1];
          *v2 = v1;
        }
      }
    }
    else {
      v13 = 0;
      v16 = v15;
    }
    v6 = (void *)sub_72a0(dat_26388,0);
    v7 = (char *)sub_72a0(v4,1);
    v8 = "";
    if (*v7 != '/')
      v8 = "/";
    __printf_chk(1,"\x1b]8;;file://%s%s%s\a",v6,v8);
    free(v6);
    free(v7);
    v15 = v16;
  }
  else {
    v13 = 0;
    v16 = '\0';
    v15 = '\0';
  }
  if (a5) { // branch-flip
    if (dat_26318) {
      v9 = *(long **)(a5 + 0x18);
      if ((unsigned long)(*(long *)(a5 + 0x20) - (long)v9) <= 7) {
        _obstack_newchunk(a5,8);
        v9 = *(long **)(a5 + 0x18);
      }
      *v9 = dat_261f8;
      *(long *)(a5 + 0x18) = *(long *)(a5 + 0x18) + 8;
    }
    fwrite_unlocked(&v11[v13],1,v14,stdout);
    dat_261f8 += v5;
    v16 = v15;
    if (dat_26318) {
      v9 = *(long **)(a5 + 0x18);
      if ((unsigned long)(*(long *)(a5 + 0x20) - (long)v9) <= 7) {
        _obstack_newchunk(a5,8);
        v9 = *(long **)(a5 + 0x18);
      }
      *v9 = dat_261f8;
      *(long *)(a5 + 0x18) = *(long *)(a5 + 0x18) + 8;
    }
  }
  else {
    fwrite_unlocked(&v11[v13],1,v14,stdout);
    dat_261f8 += v5;
  }
  if ((v4) && (fputs_unlocked("\x1b]8;;\a",stdout), v16)) {
    v1 = v11[v5 - 1];
    v2 = *(unsigned char **)((long)stdout + 0x28);
    if (*(unsigned char **)((long)stdout + 0x30) <= v2) // branch-flip
      __overflow(stdout,(unsigned int)v1);
    else {
      *(unsigned char **)((long)stdout + 0x28) = &v2[1];
      *v2 = v1;
    }
  }
  if ((v11 != a0) && (v11 != v10))
    free(v11);
  return v12 + v5;
}


// Function: sub_c0e0 @ 0xc0e0
long sub_c0e0(unsigned long *a0,unsigned char a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1;
  char v10;
  unsigned long v2;
  unsigned int v3; // eax
  long v4; // rax
  unsigned long v5;
  unsigned long *v6;
  unsigned int v7; // edx
  unsigned int v8;
  char *v9;
  
  if (a1) { // branch-flip
    v9 = (char *)a0[1];
    if (!dat_26312) {
      v4 = sub_bcf0(v9,dat_262d0,*(unsigned int *)((long)a0 + 0xc4),0,a1 ^ 1,a2,a0[2]); // return-dupe
      sub_84f0();
      return v4;
    }
    v8 = *(unsigned int *)((long)a0 + 0xac);
    if (!*(char *)((long)a0 + 0xb9)) {
      if (!sub_6c80(0xc)) {
        v3 = 0xffffffff;
        goto label_c1b4;
      }
      v4 = 0xc0;
      goto label_c21e;
    }
    v10 = *(char *)&a0[0x17];
    v3 = 0;
    if (v10) goto label_c303;
label_c1c5:
    v8 = *(unsigned int *)&"\r"[(unsigned long)*(unsigned int *)&a0[0x15] * 4];
    v5 = (unsigned long)v8;
    v10 = v8 == 7;
    if (v8 != 5) { // branch-flip
label_c1e6:
      if ((v3) || (!v10))
        v4 = v5 << 4;
      else {
        v4 = 0xd0;
        if (!dat_26390)
          v4 = (-(unsigned long)(sub_6c80(0xd) == '\0') & 0xffffffffffffffa0) + 0xd0;
      }
    }
    else {
label_c3b0:
      v5 = strlen(v9);
      for (v6 = dat_26308; v6; v6 = (unsigned long *)v6[4]) {
        v1 = *v6;
        if ((v1 <= v5) && (v2 = v6[1], !sub_eeb0(&v9[v5 - v1],v2))) {
          v6 = &v6[2];
          goto label_c228;
        }
      }
      v4 = 0x50;
    }
  }
  else {
    v9 = (char *)*a0;
    if (!dat_26312) {
      v4 = sub_bcf0(v9,dat_262d0,*(unsigned int *)((long)a0 + 0xc4),0,a1 ^ 1,a2,a0[2]);
      sub_84f0();
      return v4;
    }
    v3 = (unsigned int)*(unsigned char *)((long)a0 + 0xb9);
    if ((dat_26390) && (*(unsigned char *)((long)a0 + 0xb9)))
      v8 = *(unsigned int *)((long)a0 + 0xac);
    else {
      v8 = *(unsigned int *)&a0[6];
    }
label_c1b4:
    v10 = *(char *)&a0[0x17];
    if (!v10) goto label_c1c5;
label_c303:
    v7 = v8 & 0xf000;
    if (v7 == 0x8000) {
      if ((v8 & 0x800) && (sub_6c80(0x10))) {
        v4 = 0x100;
        goto label_c21e;
      }
      if ((v8 & 0x400) && (sub_6c80(0x11))) {
        v4 = 0x110;
        goto label_c21e;
      }
      if ((sub_6c80(0x15)) && (*(char *)&a0[0x18])) {
        v4 = 0x150;
        goto label_c21e;
      }
      if ((v8 & 0x49) && (sub_6c80(0xe))) {
        v4 = 0xe0;
        goto label_c21e;
      }
      if ((2 <= (unsigned long)a0[5]) && (sub_6c80(0x16))) {
        v4 = 0x160;
        goto label_c21e;
      }
      goto label_c3b0;
    }
    if (v7 != 0x4000) {
      if (v7 != 0xa000) {
        v4 = 0x80;
        if ((((v7 != 0x1000) && (v4 = 0x90, v7 != 0xc000)) && (v4 = 0xa0, v7 != 0x6000)) && (v4 = 0xb0, v7 != 0x2000))
          v4 = 0xd0;
        goto label_c21e;
      }
      v5 = 7;
      goto label_c1e6;
    }
    if ((v8 & 0x202) == 0x202) {
      v4 = 0x140;
      if (sub_6c80(0x14)) goto label_c21e;
    }
    if (v8 & 2) {
      v4 = 0x130;
      if (sub_6c80(0x13)) goto label_c21e;
    }
    v4 = 0x60;
    if (v8 & 0x200)
      v4 = (-(unsigned long)(sub_6c80(0x12) == '\0') & 0xffffffffffffff40) + 0x120;
  }
label_c21e:
  v6 = (unsigned long *)(v4 + 0x25060);
label_c228:
  if (!v6[1]) {
    if (!sub_6c80(4)) {
      v4 = sub_bcf0(v9,dat_262d0,*(unsigned int *)((long)a0 + 0xc4),0,a1 ^ 1,a2,a0[2]);
      sub_84f0();
      return v4;
    }
    v6 = NULL;
  }
  v4 = sub_bcf0(v9,dat_262d0,*(unsigned int *)((long)a0 + 0xc4),v6,a1 ^ 1,a2,a0[2]);
  sub_84f0();
  if (dat_25088) // branch-flip
    sub_7880(0x25080);
  else {
    sub_7880(0x25060);
    sub_7880(0x25090);
    sub_7880(0x25070);
  }
  if (!dat_262b0)
    return v4;
  v5 = v4 + -1 + a3;
  if (a3 / dat_262b0 == v5 / dat_262b0)
    return v4;
  sub_7880(0x251d0,a3 / dat_262b0,v5 % dat_262b0);
  return v4;
}


// Function: sub_c7c0 @ 0xc7c0
void sub_c7c0(char a0)
{
  unsigned long v1;
  unsigned long v2;
  char *v3;
  unsigned char *v4;
  unsigned long v5;
  unsigned int v6; // ecx
  unsigned char v7;
  unsigned long v8; // rbx
  unsigned long v9;
  
  if (dat_263b0) {
    v8 = 0;
    v9 = 0;
    do {
      v2 = *(unsigned long *)(dat_263a0 + v8 * 8);
      if (dat_262b0) { // branch-flip
        v5 = sub_7e10(v2);
        if (v8) { // branch-flip
          v1 = v5 + v9 + 2;
          if ((!dat_262b0) || ((v1 < dat_262b0 && (v9 <= -v5 - 3)))) {
            v5 = v1;
            goto label_c870;
          }
          v6 = (unsigned int)(char)dat_25019;
          v9 = 0;
          v7 = dat_25019;
label_c87a:
          v3 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
            __overflow(stdout,(int)a0);
          else {
            *(char **)((long)stdout + 0x28) = &v3[1];
            *v3 = a0;
          }
          v4 = *(unsigned char **)((long)stdout + 0x28);
          if (*(unsigned char **)((long)stdout + 0x30) <= v4) // branch-flip
            __overflow(stdout,v6 & 0xff);
          else {
            *(unsigned char **)((long)stdout + 0x28) = &v4[1];
            *v4 = v7;
          }
        }
        else {
          v5 = v9 + v5;
        }
      }
      else {
        v5 = v9;
        if (v8) {
          v5 = v9 + 2;
label_c870:
          v9 += 2;
          v6 = 0x20;
          v7 = 0x20;
          goto label_c87a;
        }
      }
      v8 += 1;
      sub_c590(v2,v9);
      v9 = v5;
    } while (v8 < dat_263b0);
  }
  v4 = *(unsigned char **)((long)stdout + 0x28);
  if (v4 < *(unsigned char **)((long)stdout + 0x30)) {
    *(unsigned char **)((long)stdout + 0x28) = &v4[1];
    *v4 = dat_25019;
    return;
  }
  __overflow(stdout,(unsigned int)dat_25019); // tail-call
}


// Function: sub_c980 @ 0xc980
void sub_c980(long a0) // return-dupe x5
{
  unsigned int v1;
  unsigned long v10; // rax
  char *v11; // rbx
  char *v12;
  char *v13;
  long v14; // rbx
  char v15; // stack - 0x12a4
  char v16 [3656];
  char v17 [1008];
  char v18 [16];
  unsigned long v19; // stack - 0x1338
  unsigned int v2;
  char v20 [32];
  char v21 [16];
  unsigned long v22; // stack - 0x1340
  int v23; // ebp
  unsigned int v24; // r8d
  unsigned long v25; // r8
  char *v26; // rax
  bool v27;
  unsigned long v28; // stack - 0x1330
  int v29; // stack - 0x1318
  char *v3;
  int v30; // stack - 0x12d8
  unsigned long v31; // stack - 0x12a3
  unsigned short v32; // stack - 0x129b
  char v33; // stack - 0x1299
  char v4;
  int v5;
  unsigned int v6;
  char *v7;
  long v8;
  unsigned long v9; // rax
  
  if (*(char *)(a0 + 0xb8)) // branch-flip
    sub_fba0(a0 + 0x18,&v15);
  else {
    v33 = 0;
    v32 = 0x3f3f;
    v15 = "?pcdb-lswd"[*(unsigned int *)(a0 + 0xa8)];
    v31 = 0x3f3f3f3f3f3f3f3f;
  }
  if (dat_26364) { // branch-flip
    if (*(int *)(a0 + 0xbc) == 1) {
      v32 = CONCAT11(0x2e,(char)v32);
      goto label_c9e6;
    }
    if (*(int *)(a0 + 0xbc) != 2) goto label_c9e6;
    v32 = CONCAT11(0x2b,(char)v32);
    if (dat_26334 != 2) goto label_c9f5;
label_ccf8:
    v28 = *(unsigned long *)(a0 + 0x68);
    v19 = *(unsigned long *)(a0 + 0x60);
    v27 = 1;
  }
  else {
    v32 &= 0xff;
label_c9e6:
    if (dat_26334 == 2) goto label_ccf8;
label_c9f5:
    if (3 <= dat_26334) { // branch-flip
      if (dat_26334 != 3)
        abort(); // no-return
      v19 = *(unsigned long *)(a0 + 0x70);
      v28 = *(unsigned long *)(a0 + 0x78);
      v27 = (v19 & v28) != 0xffffffffffffffff;
    }
    else if (dat_26334) { // branch-flip
      v28 = *(unsigned long *)(a0 + 0x88);
      v19 = *(unsigned long *)(a0 + 0x80);
      v27 = 1;
    }
    else {
      v28 = *(unsigned long *)(a0 + 0x78);
      v19 = *(unsigned long *)(a0 + 0x70);
      v27 = 1;
    }
  }
  v12 = v16;
  if (dat_262fc) {
    v12 = "?";
    if ((*(char *)(a0 + 0xb8)) && (*(long *)(a0 + 0x20)))
      v12 = (char *)sub_12720(*(long *)(a0 + 0x20),v17);
    v25 = (unsigned long)dat_26360;
    v12 = &v16[__sprintf_chk(v16,1,0xe3b,"%*s ",v25,v12)];
  }
  if (dat_2632c) {
    v26 = "?";
    if (*(char *)(a0 + 0xb8))
      v26 = (char *)sub_11550(*(unsigned long *)(a0 + 0x58),v17,dat_26328,0x200,dat_26320);
    v5 = dat_2635c;
    v5 -= sub_12ec0(v26,0);
    if (v5 <= 0) goto label_cfd0;
    v7 = &v12[v5];
    do {
      v11 = &v12[1];
      *v12 = ' ';
      v12 = v11;
      v13 = v26;
    } while (v11 != v7);
    while( true ) {
      v26 = &v13[1];
      v12 = &v7[1];
      v4 = *v13;
      *v7 = v4;
      if (!v4) break;
label_cfd0:
      v7 = v12;
      v13 = v26;
    }
    *v7 = ' ';
  }
  v26 = "?";
  if (*(char *)(a0 + 0xb8))
    v26 = (char *)sub_12720(*(unsigned long *)(a0 + 0x28),v17);
  v25 = (unsigned long)dat_26358;
  v12 = &v12[__sprintf_chk(v12,1,0xffffffffffffffff,"%s %*s ",&v15,v25,v26)];
  if (dat_26318)
    sub_7430("  ",strlen("  "));
  if ((((dat_25029) || (dat_25028)) || (dat_2632e)) || (dat_26365)) {
    sub_7430(v16,(long)v12 - (long)v16);
    v1 = dat_26350;
    if (dat_25029) {
      v12 = "?";
      v2 = *(unsigned int *)(a0 + 0x34);
      if ((*(char *)(a0 + 0xb8)) && (v12 = NULL, !dat_2632d))
        v12 = (char *)sub_12340(v2);
      sub_92c0(v12,v2,v1);
    }
    v2 = dat_2634c;
    v1 = dat_26348;
    if (dat_25028) {
      v12 = "?";
      v1 = *(unsigned int *)(a0 + 0x38);
      if ((*(char *)(a0 + 0xb8)) && (v12 = NULL, !dat_2632d))
        v12 = (char *)sub_124e0(v1);
      sub_92c0(v12,v1,v2);
      v1 = dat_26348;
    }
    dat_26348 = v1;
    if (dat_2632e) {
      v12 = "?";
      v2 = *(unsigned int *)(a0 + 0x34);
      if ((*(char *)(a0 + 0xb8)) && (v12 = NULL, !dat_2632d))
        v12 = (char *)sub_12340(v2);
      sub_92c0(v12,v2,v1);
    }
    v12 = v16;
    if (dat_26365)
      sub_92c0(*(unsigned long *)(a0 + 0xb0),0,dat_26354);
  }
  if (*(char *)(a0 + 0xb8)) { // branch-flip
    if ((*(unsigned int *)(a0 + 0x30) & 0xb000) != 0x2000) {
      v26 = (char *)sub_11550(*(unsigned long *)(a0 + 0x48),v17,dat_2631c,1,dat_25020);
      goto label_cb76;
    }
    v23 = dat_2633c - (dat_26344 + 2 + dat_26340);
    v9 = sub_12720((unsigned int)((*(unsigned long *)(a0 + 0x40) >> 0x14) << 8) | (unsigned int)*(unsigned long *)(a0 + 0x40) & 0xff,v17);
    v6 = dat_26340;
    v10 = sub_12720((unsigned int)((unsigned long)*(unsigned long *)(a0 + 0x40) >> 0x20) & 0xfffff000 | (unsigned int)((unsigned long)*(unsigned long *)(a0 + 0x40) >> 8) & 0xfff,v20);
    v5 = 0;
    if (0 <= v23)
      v5 = v23;
    v24 = v5 + dat_26344;
    v7 = &v12[__sprintf_chk(v12,1,0xffffffffffffffff,"%*s, %*s ",(unsigned long)v24,v10,(unsigned long)v6,v9)];
  }
  else {
    v26 = "?";
label_cb76:
    v5 = dat_2633c;
    v5 -= sub_12ec0(v26,0);
    if (1 <= v5) { // branch-flip
      v13 = v12;
      do {
        v11 = &v13[1];
        *v13 = ' ';
        v13 = v11;
        v7 = &v12[v5];
      } while (v11 != &v12[v5]);
    }
    else {
      v7 = v12;
    }
    do {
      v12 = v7;
      v4 = *v26;
      v26 = &v26[1];
      v7 = &v12[1];
      *v12 = v4;
    } while (v4);
    *v12 = ' ';
  }
  *v7 = '\x01';
  if ((*(char *)(a0 + 0xb8)) && (v27)) {
    v8 = sub_17c60(dat_262a8,&v19,v18);
    if (v8) { // branch-flip
      v27 = (long)v28 < dat_26378;
      v6 = (unsigned int)(dat_26378 < (long)v28);
      v5 = (unsigned int)v27 - (unsigned int)(dat_26378 < (long)v28);
      if ((int)(v5 + ((unsigned int)((long)v19 < dat_26370) - (unsigned int)(dat_26370 < (long)v19)) * 2) < 0) {
        sub_10200(0x26370);
        v27 = (long)v28 < dat_26378;
        v6 = (unsigned int)(dat_26378 < (long)v28);
        v5 = v27 - v6;
      }
      v6 = (v6 - v27) + ((unsigned int)(dat_26370 < (long)v19) - (unsigned int)((long)v19 < dat_26370)) * 2 & v5 + ((unsigned int)((long)v19 < dat_26370 + -0xf0c2ac) - (unsigned int)(dat_26370 + -0xf0c2ac < (long)v19)) * 2;
      if (dat_253c8) // branch-flip
        v8 = ((unsigned long)((int)v6 >> 0x1f & 0xc) + (long)v29) * 0x80 + 0x253e0;
      else {
        v8 = *(long *)((unsigned long)(v6 >> 0x1f) * 8 + 0x25040);
      }
      v8 = sub_14d80(v7,0x3e9,v8,v18,dat_262a8,v28);
      if (!v8) goto label_d1c8;
      v7 = &v7[v8];
    }
    else {
label_d1c8:
      if (*v7) {
        if (!*(char *)(a0 + 0xb8)) goto label_cbdc;
        v12 = (char *)sub_12680(v19,v20);
        goto label_cbe3;
      }
    }
    *v7 = ' ';
    v7 = &v7[1];
    goto label_cc19;
  }
label_cbdc:
  v12 = "?";
label_cbe3:
  v25 = (unsigned long)dat_25014;
  if ((int)dat_25014 < 0) {
    v22 = 0;
    v8 = sub_17c60(dat_262a8,&v22,v21);
    if (v8) { // branch-flip
      v8 = dat_25040;
      if (dat_253c8)
        v8 = (long)v30 * 0x80 + 0x253e0;
      v8 = sub_14d80(v17,0x3e9,v8,v21,dat_262a8,0);
      if (!v8) goto label_d364;
      v6 = sub_12ca0(v17,v8,0);
      dat_25014 = v6;
    }
    else {
label_d364:
      v6 = dat_25014;
    }
    v25 = (unsigned long)v6;
    if ((int)v6 <= -1) {
      dat_25014 = 0;
      v25 = 0;
    }
  }
  v7 = &v7[__sprintf_chk(v7,1,0xffffffffffffffff,"%*s ",v25,v12)];
label_cc19:
  v14 = (long)v7 - (long)v16;
  sub_7430(v16,v14);
  v8 = sub_c0e0(a0,0,0x261a0,v14);
  if (*(int *)(a0 + 0xa8) != 6) { // branch-flip
    if (!dat_26314)
      return;
    v6 = sub_75b0(*(char *)(a0 + 0xb8),*(unsigned int *)(a0 + 0x30),*(int *)(a0 + 0xa8));
    v4 = (char)v6;
  }
  else {
    if (!*(long *)(a0 + 8))
      return;
    sub_7430(" -> ",strlen(" -> "));
    sub_c0e0(a0,1,0,v14 + 4 + v8);
    if (!dat_26314)
      return;
    v6 = sub_75b0(1,*(unsigned int *)(a0 + 0xac),0);
    v4 = (char)v6;
  }
  if (!v4)
    return;
  dat_261f8 += 1;
  v3 = *(char **)((long)stdout + 0x28);
  if (v3 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v3[1];
    *v3 = (char)v6;
    return;
  }
  __overflow(stdout,v6 & 0xff);
}


// Function: sub_d420 @ 0xd420
void sub_d420(void) // return-dupe x6
{
  long v1;
  long v10; // rax
  unsigned long v11; // stack - 0x40
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rdx
  unsigned long v6;
  long v7;
  unsigned long v8;
  long v9;
  
  switch(dat_26338) {
    case 0:
      v3 = 0;
      if (!dat_263b0)
        return;
      do {
        if ((dat_26312) && (sub_6c80(4))) {
          sub_7880(0x25060);
          sub_7880(0x250a0);
          sub_7880(0x25070);
        }
        sub_c980(dat_263a0[v3]);
        dat_261f8 += 1;
        v2 = *(unsigned char **)((long)stdout + 0x28);
        if (*(unsigned char **)((long)stdout + 0x30) <= v2) // branch-flip
          __overflow(stdout,(unsigned int)dat_25019);
        else {
          *(unsigned char **)((long)stdout + 0x28) = &v2[1];
          *v2 = dat_25019;
        }
        v3 += 1;
      } while (v3 < dat_263b0);
      return;
    case 1:
      v3 = 0;
      if (!dat_263b0)
        return;
      do {
        sub_c590(dat_263a0[v3],0);
        v2 = *(unsigned char **)((long)stdout + 0x28);
        if (*(unsigned char **)((long)stdout + 0x30) <= v2) // branch-flip
          __overflow(stdout,(unsigned int)dat_25019);
        else {
          *(unsigned char **)((long)stdout + 0x28) = &v2[1];
          *v2 = dat_25019;
        }
        v3 += 1;
      } while (v3 < dat_263b0);
      return;
    case 2:
      if (dat_262b0) {
        v3 = sub_7ff0(1);
        v11 = 0;
        v1 = dat_26208 + v3 * 0x18;
        v3 = (dat_263b0 / v3 + 1) - (unsigned long)(dat_263b0 % v3 == 0);
        if (!v3)
          return;
        do {
          v4 = 0;
          v6 = v11;
          v9 = 0;
          while( true ) {
            v8 = dat_263a0[v6];
            v10 = sub_7e10(v8);
            v7 = *(long *)(*(long *)(v1 + -8) + v4);
            v4 += 8;
            sub_c590(v8,v9);
            v6 += v3;
            if (dat_263b0 <= v6) break;
            v7 += v9;
            sub_7360(v10 + v9,v7);
            v9 = v7;
          }
          v2 = *(unsigned char **)((long)stdout + 0x28);
          if (*(unsigned char **)((long)stdout + 0x30) <= v2) // branch-flip
            __overflow(stdout,(unsigned int)dat_25019);
          else {
            *(unsigned char **)((long)stdout + 0x28) = &v2[1];
            *v2 = dat_25019;
          }
          v11 += 1;
        } while (v3 != v11);
        return;
      }
      break;
    case 3:
      if (dat_262b0) {
        v3 = sub_7ff0(0);
        v1 = dat_26208 + -0x18 + v3 * 0x18;
        v8 = *dat_263a0;
        v4 = sub_7e10(v8);
        v9 = **(long **)(v1 + 0x10);
        sub_c590(v8,0);
        if (2 <= dat_263b0) {
          v6 = 1;
          v7 = 0;
          do {
            v5 = v6 % v3;
            if (v5) { // branch-flip
              v10 = v9 + v7;
              sub_7360(v7 + v4,v10);
            }
            else {
              v2 = *(unsigned char **)((long)stdout + 0x28);
              if (*(unsigned char **)((long)stdout + 0x30) <= v2) { // branch-flip
                v10 = 0;
                __overflow(stdout,(unsigned int)dat_25019);
              }
              else {
                v10 = 0;
                *(unsigned char **)((long)stdout + 0x28) = &v2[1];
                *v2 = dat_25019;
              }
            }
            v8 = dat_263a0[v6];
            v6 += 1;
            sub_c590(v8,v10);
            v4 = sub_7e10(v8);
            v9 = *(long *)(*(long *)(v1 + 0x10) + v5 * 8);
            v7 = v10;
          } while (v6 < dat_263b0);
        }
        v2 = *(unsigned char **)((long)stdout + 0x28);
        if (v2 < *(unsigned char **)((long)stdout + 0x30)) {
          *(unsigned char **)((long)stdout + 0x28) = &v2[1];
          *v2 = dat_25019;
          return;
        }
        __overflow(stdout,(unsigned int)dat_25019); // tail-call
        return;
      }
      break;
    case 4:
      v8 = 0x2c;
      sub_c7c0(v8); // return-dupe, tail-call
      return;
    default:
      return;
    
  }
  v8 = 0x20;
  sub_c7c0(v8);
  return;
}


// Function: sub_d820 @ 0xd820
void sub_d820(char *a0,char *a1,char a2)
{
  char *v1;
  void *v10;
  char *v11;
  unsigned long v12; // rax
  char *v13;
  unsigned long v14; // stack - 0x368
  char v15 [663];
  unsigned int v16;
  long v17; // stack - 0x378
  unsigned long v18; // stack - 0x360
  unsigned long v2;
  unsigned char v3; // al
  int v4; // eax
  int *v5; // rax
  void *v6; // rax
  unsigned long *v7;
  unsigned long *v8;
  unsigned long v9;
  
  v5 = __errno_location();
  *v5 = 0;
  v6 = opendir(a0);
  if (!v6) {
    sub_7540(a2,dcgettext(NULL,"cannot open directory %s",5),a0); // tail-call
    return;
  }
  if (dat_263c8) {
    v4 = dirfd(v6);
    if (0 <= v4) { // branch-flip
      v4 = sub_7050(v4,0x1bb19,&v14,0x1000,0x100);
      v9 = v14;
      v2 = v18;
    }
    else {
      v4 = sub_7050(0xffffff9c,a0,&v14,0,0x100);
      v9 = v14;
      v2 = v18;
    }
    if (v4 <= -1) {
      sub_7540(a2,dcgettext(NULL,"cannot determine device and inode of %s",5),a0);
      closedir(v6); // tail-call
      return;
    }
    v7 = (unsigned long *)sub_18590(0x10);
    v7[1] = v9;
    *v7 = v2;
    v8 = (unsigned long *)sub_112c0(dat_263c8,v7);
    if (!v8)
      sub_18a90(); // no-return
    if (v7 != v8) {
      free(v7);
      v9 = sub_17150(0,3,a0);
      error(0,0,dcgettext(NULL,"%s: not listing already-listed directory",5),v9);
      closedir(v6);
      dat_26210 = 2;
      return;
    }
    if ((unsigned long)(dat_26100 - (long)dat_260f8) <= 0xf)
      _obstack_newchunk(0x260e0,0x10);
    v7 = &dat_260f8[2];
    dat_260f8[1] = v9;
    *dat_260f8 = v2;
    dat_260f8 = v7;
  }
  sub_8730();
  if (dat_262f6) { // branch-flip
    if (dat_25010) goto label_d9cf;
label_db04:
    dat_261f8 += 1;
    v1 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v1) {
      __overflow(stdout,10);
      goto label_d9cf;
    }
    *(char **)((long)stdout + 0x28) = &v1[1];
    *v1 = 10;
    if (dat_26318) goto label_db40;
label_d9e3:
    dat_25010 = '\0';
  }
  else {
    if (!dat_262b8) goto label_da47;
    if (!dat_25010) goto label_db04;
label_d9cf:
    if (!dat_26318) goto label_d9e3;
label_db40:
    dat_25010 = '\0';
    sub_7430("  ",strlen("  "));
  }
  v10 = NULL;
  if ((dat_26311) && (v10 = (void *)sub_ef30(a0,2), !v10))
    sub_7540(a2,dcgettext(NULL,"error canonicalizing %s",5),a0);
  if (!a1)
    a1 = a0;
  sub_bcf0(a1,dat_262c8,0xffffffff,0,1,0x26140,v10);
  free(v10);
  sub_7430(":\n",strlen(":\n"));
label_da47:
  v17 = 0;
  do {
    *v5 = 0;
    v10 = readdir(v6);
    if (v10) { // branch-flip
      v11 = (char *)((long)v10 + 0x13);
      v7 = dat_262e8;
      if (dat_262f0 != 2) {
        if (*(char *)((long)v10 + 0x13) != '.') { // branch-flip
          v8 = dat_262e0;
          if (!dat_262f0) {
            for (; v7 = dat_262e8, v8; v8 = (unsigned long *)v8[1]) {
              v13 = (char *)*v8;
              if (!fnmatch(v13,v11,4)) goto label_dae0;
            }
          }
        }
        else if ((!dat_262f0) || (!*(char *)((long)v10 + (unsigned long)(*(char *)((long)v10 + 0x14) == '.') + 0x14))) goto label_dae0;
      }
      for (; v7; v7 = (unsigned long *)v7[1]) {
        v13 = (char *)*v7;
        if (!fnmatch(v13,v11,4)) goto label_dae0;
      }
      v16 = 0;
      v3 = *(char *)((long)v10 + 0x12) - 1;
      if (v3 <= 0xd)
        v16 = *(unsigned int *)((unsigned long)v3 * 4 + 0x1a600);
      v17 += sub_96f0(v11,v16,0,a0);
      if ((((dat_26338 == 1) && (dat_26330 == 6)) && (!dat_2632c)) && (!dat_262f6)) {
        sub_8e30();
        sub_d420();
        sub_8730();
      }
    }
    else {
      if (!*v5) {
label_dc53:
        if (closedir(v6))
          sub_7540(a2,dcgettext(NULL,"closing directory %s",5),a0);
        sub_8e30();
        if (dat_262f6)
          sub_9510(a0,0);
        if ((!dat_26338) || (dat_2632c)) {
          v11 = (char *)sub_11550(v17,v15,dat_26328,0x200,dat_26320);
          v12 = strlen(v11);
          v11[-1] = ' ';
          v11[v12] = dat_25019;
          if (dat_26318)
            sub_7430("  ",strlen("  "));
          v13 = dcgettext(NULL,"total",5);
          sub_7430(v13,strlen(v13));
          sub_7430(&v11[-1],&(&v11[v12])[1 - (long)&v11[-1]]);
        }
        if (!dat_263b0)
          return;
        sub_d420(); // tail-call
        return;
      }
      sub_7540(a2,dcgettext(NULL,"reading directory %s",5),a0);
      if (*v5 != 0x4b) goto label_dc53;
    }
label_dae0:
    sub_84f0();
  } while( true );
}


// Function: sub_df00 @ 0xdf00
void sub_df00(int a0)
{
  void *v1;
  long v10; // fs_offset
  char *v11; // stack - 0xb0
  char *v12; // stack - 0x88
  char *v13; // stack - 0x80
  char *v14; // stack - 0x78
  char *v15; // stack - 0x70
  char *v16; // stack - 0x68
  char *v17; // stack - 0x60
  unsigned long v18; // stack - 0x58
  unsigned long v19; // stack - 0x50
  unsigned long v2;
  unsigned long v20; // stack - 0x40
  char *v21 [4]; // stack - 0xa8
  int v3; // eax
  char *v4;
  char **v5; // rbx
  char *v6; // stack - 0xb8
  char *v7; // rbp
  char *v8;
  char *v9; // r13
  
  v2 = dat_26400;
  v5 = &v6;
  v20 = *(unsigned long *)(v10 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -B, --ignore-backups       do not list implied entries ending with ~\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --full-time            like -l --time-style=full-iso\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -g                         like -l, but do not list owner\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -G, --no-group             in a long listing, don\'t print group names\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hyperlink[=WHEN]     hyperlink file names WHEN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -l                         use a long listing format\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -m                         fill width with a comma separated list of entries\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -q, --hide-control-chars   print ? instead of nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -Q, --quote-name           enclose entry names in double quotes\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -S                         sort by file size, largest first\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -U                         do not sort; list entries in directory order\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -v                         natural sort of (version) numbers within text\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n",5),v1);
    v1 = stdout;
    v7 = "ls";
    fputs_unlocked(dcgettext(NULL,"\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n",5),v1);
    if ((dat_251e0 != 1) && (v7 = "dir", dat_251e0 != 2))
      v7 = "vdir";
    v18 = 0;
    v11 = "test invocation";
    v21[0] = "coreutils";
    v8 = "[";
    v21[1] = "Multi-call invocation";
    v12 = "sha256sum";
    v21[2] = "sha224sum";
    v14 = "sha384sum";
    v6 = "[";
    v21[3] = "sha2 utilities";
    v13 = "sha2 utilities";
    v15 = "sha2 utilities";
    v16 = "sha512sum";
    v17 = "sha2 utilities";
    v19 = 0;
    do {
      if (!strcmp(v7,v8)) break;
      v8 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v8);
    v8 = *(char **)((long)v5 + 8);
    if (!*(char **)((long)v5 + 8))
      v8 = v7;
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "test";
    if (strcmp(v7,"["))
      v4 = v7;
    v9 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != v8)
      v9 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v8,v9);
  }
  exit(a0); // no-return
}


// Function: sub_102f0 @ 0x102f0
unsigned long sub_102f0(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}


// Function: sub_10310 @ 0x10310
bool sub_10310(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_10320 @ 0x10320
unsigned long sub_10320(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1e310)
    return 1;
  v1 = v2[2];
  if ((((dat_1e324 < v1) && (v1 < dat_1e328)) && (dat_1e32c < v2[3])) && (dat_1e340 <= *v2)) {
    v3 = *v2 + dat_1e324;
    if (((v3 < v2[1]) && (v2[1] <= dat_1e330)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1e310;
  return 0;
}


// Function: sub_103b0 @ 0x103b0
unsigned long sub_103b0(long *a0,long a1,unsigned long *a2,char a3) // early-return
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


// Function: sub_10600 @ 0x10600
unsigned long sub_10600(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_10636:
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
        goto label_10636;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}


// Function: sub_10870 @ 0x10870
void sub_10870(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_108dd;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_108dd:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_109d0 @ 0x109d0
long sub_109d0(long *a0,long a1) // return-dupe
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


// Function: sub_10a90 @ 0x10a90
long sub_10a90(long *a0,long a1)
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


// Function: sub_10b70 @ 0x10b70
long sub_10b70(unsigned long *a0,void *a1,unsigned long a2)
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


// Function: sub_10c60 @ 0x10c60
unsigned long * sub_10c60(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_102f0;
  if (!a3)
    a3 = sub_10310;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x1e310;
  v1[5] = a1;
  if (sub_10320(v1)) {
    v2 = sub_104c0(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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


// Function: sub_10d70 @ 0x10d70
void sub_10d70(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_10e0b;
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
label_10e0b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_10e20 @ 0x10e20
void sub_10e20(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_10ecc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_10e8f;
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
label_10e8f:
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
label_10ecc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_10f10 @ 0x10f10
unsigned int sub_10f10(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_104c0(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_10600(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_10600(a0,&v2,1)) && (sub_10600(a0,&v2,0))) {
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


// Function: sub_11090 @ 0x11090
long * sub_11090(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_103b0(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_1111f;
label_111a6:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_111a6;
label_1111f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_10320(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_1e334 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_1e338 <= v5) ? (long)(v5 - dat_1e338) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_10f10(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_103b0(a0,a1,&v3,0);
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


// Function: sub_112c0 @ 0x112c0
unsigned long sub_112c0(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_11090(a0,a1,&v3);
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


// Function: sub_11320 @ 0x11320
long sub_11320(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_103b0(a0,a1,&v7,1);
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
      sub_10320(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1e338 <= v8) ? (long)(v8 - dat_1e338) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_10f10(a0,v6)) {
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

