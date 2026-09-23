// Function: sub_3c50 @ 0x3c50
unsigned long sub_3c50(unsigned int a0,char **a1)
{
  long v1;
  long v10; // rax
  long *v11;
  long v12;
  unsigned long *v13; // rax
  int *v14; // rax
  unsigned long v15;
  char *v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  long v19; // rcx
  unsigned long v2;
  unsigned long v20; // rdx
  unsigned long v21;
  unsigned long v22;
  long v23;
  unsigned long v24; // rdx
  unsigned long v25; // rdx
  unsigned long v26; // rdx
  unsigned long v27; // rdx
  unsigned long v28; // rdx
  unsigned long v29; // rdx
  unsigned long v3;
  bool v30;
  int v31; // stack - 0x10c
  unsigned long v32; // stack - 0x108
  unsigned long v33; // stack - 0xd8
  char *v34; // stack - 0xe8
  unsigned short v35;
  unsigned long v36;
  char **v37;
  unsigned long v38;
  unsigned long v39;
  long *v4;
  bool v40;
  bool v41;
  bool v42; // stack - 0x160
  char *v43; // stack - 0x158
  unsigned int v44; // stack - 0x148
  long v45; // stack - 0x138
  bool v46; // stack - 0x125
  unsigned long v47; // stack - 0x100
  unsigned long v48; // stack - 0xf8
  unsigned long v49; // stack - 0xf0
  unsigned int v5;
  unsigned long v50; // stack - 0xe0
  unsigned long v51; // stack - 0xd0
  char v6; // al
  int v7;
  long v8; // rax
  char *v9;
  
  v50 = 0;
  v34 = ".";
  sub_eb20(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_13be0(sub_5f80);
  dat_1b140 = sub_6800();
  sub_d520(getenv("DU_BLOCK_SIZE"),0x1b170,0x1b148);
  v30 = 0;
  v5 = 0x10;
  v44 = 8;
  v43 = NULL;
  v42 = 1;
  v40 = 0;
label_3d50:
  do {
    v22 = (unsigned long)a0;
    v31 = -1;
    v7 = getopt_long(a0,a1,"0abd:chHklmst:xB:DLPSX:",(void *)0x1a700,&v31);
    if (v7 == -1) {
      if (!v42)
        sub_5540(1); // return-dupe, no-return
      v15 = v20;
      if (dat_1b186) { // branch-flip
        if (v30) {
          error(0,0,dcgettext(NULL,"cannot both summarize and show all entries",5));
          sub_5540(1);
        }
      }
      else if (v40) {
        if (v30) {
          if (dat_1b010) {
label_445d:
            v22 = dat_1b010;
            error(0,0,dcgettext(NULL,"warning: summarizing conflicts with --max-depth=%lu",5),v22);
            sub_5540(1); // no-return
          }
          error(0,0,dcgettext(NULL,"warning: summarizing is the same as using --max-depth=0",5));
          v15 = v24;
          if (dat_1b010) goto label_445d;
          dat_1b010 = 0; // crossjump-dupe
        }
      }
      else if (v30)
        dat_1b010 = 0;
      if (dat_1b16d) {
        if (dat_1b185) {
          error(0,0,dcgettext(NULL,"warning: options --apparent-size and -b are ineffective with --inodes",5));
          v15 = v27;
        }
        dat_1b148 = 1;
      }
      if (!dat_1b16c) goto label_41b8;
      if (dat_1b160) {
        v9 = dat_1b160;
        goto label_41a2;
      }
      v9 = getenv("TIME_STYLE");
      v15 = v25;
      if (v9) {
        dat_1b160 = v9;
        v15 = v26;
        if (strcmp(v9,"locale")) {
          if (*v9 != '+') goto label_4fc8;
          v16 = strchr(v9,10);
          if (!v16) goto label_41ad;
          *v16 = '\0';
          v15 = v29;
          goto label_41a2;
        }
      }
      v9 = "long-iso";
      dat_1b160 = "long-iso";
      goto label_4d36;
    }
    if (0x87 < v7) {
label_3dae:
      v42 = 0;
      goto label_3d50;
    }
    if (v7 <= 0x2f) {
      if (v7 == -0x83) {
        sub_11ab0(stdout,"du","GNU coreutils",dat_1b018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0,v22);
        exit(0); // no-return
      }
      if (v7 == -0x82)
        sub_5540(0); // no-return
      goto label_3dae;
    }
    switch(v7) {
      case 0x30:
        dat_1b182 = 1;
        goto label_3d50;
      default:
        goto label_3dae;
      case 0x42:
label_3f40:
        v7 = sub_d520(optarg,0x1b170,0x1b148);
        if (v7)
          sub_12850(v7,v31,0x42,0x1a700,optarg); // no-return
        break;
      case 0x44:
      case 0x48:
        v5 = 0x11;
        goto label_3d50;
      case 0x4c:
        v5 = 2;
        goto label_3d50;
      case 0x50:
        v5 = 0x10;
        goto label_3d50;
      case 0x53:
        dat_1b180 = '\x01';
        goto label_3d50;
      case 0x58:
        v7 = sub_70c0(sub_6b70,dat_1b140,optarg,0x10000000,10);
        if (v7) {
          v15 = sub_10b80(0,3,optarg);
          error(0,*__errno_location(),"%s",v15);
          v42 = 0;
        }
        goto label_3d50;
      case 0x61:
        dat_1b186 = '\x01';
        goto label_3d50;
      case 0x62:
        dat_1b185 = '\x01';
        dat_1b170 = 0;
        dat_1b148 = 1;
        goto label_3d50;
      case 99:
        dat_1b181 = '\x01';
        goto label_3d50;
      case 100:
        v7 = sub_12910(optarg,0,0,&v32,"");
        if (v7) { // branch-flip
          v15 = sub_10ef0(optarg);
          error(0,0,dcgettext(NULL,"invalid maximum depth %s",5),v15);
          v42 = 0;
        }
        else {
          dat_1b010 = v32;
          v40 = 1;
        }
        goto label_3d50;
      case 0x68:
        dat_1b170 = 0xb0;
        dat_1b148 = 1;
        goto label_3d50;
      case 0x6b:
        dat_1b170 = 0;
        dat_1b148 = 0x400;
        goto label_3d50;
      case 0x6c:
        dat_1b184 = '\x01';
        goto label_3d50;
      case 0x6d:
        dat_1b170 = 0;
        dat_1b148 = 0x100000;
        goto label_3d50;
      case 0x73:
        v30 = 1;
        goto label_3d50;
      case 0x74:
        v7 = sub_12250(optarg,0,0,0x1b178,"kKmMGTPEZY0");
        if (v7)
          sub_12850(v7,v31,0x74,0x1a700,optarg); // no-return
        if ((dat_1b178) || (*optarg != '-')) goto label_3d50;
        error(1,0,dcgettext(NULL,"invalid --threshold argument \'-0\'",5));
        goto label_3f40;
      case 0x78:
        v44 = 0x48;
        break;
      case 0x80:
        dat_1b185 = '\x01';
        break;
      case 0x81:
        sub_6b70(dat_1b140,optarg,0x10000000);
        break;
      case 0x82:
        v43 = optarg;
        break;
      case 0x83:
        dat_1b170 = 0x90;
        dat_1b148 = 1;
        break;
      case 0x85:
        dat_1b16c = '\x01';
        v7 = 0;
        if (optarg) {
          v8 = sub_5ce0("--time",optarg,0x1a6c0,0x156f0,4,dat_1b020,1);
          v7 = *(int *)(v8 * 4 + 0x156f0);
        }
        dat_1b168 = v7;
        dat_1b150 = sub_10fe0(getenv("TZ"));
        break;
      case 0x86:
        dat_1b160 = optarg;
        break;
      case 0x87:
        dat_1b16d = '\x01';
        goto label_3d50;
      
    }
  } while( true );
label_4b60:
  v14 = __errno_location();
  if (*v14) {
    v40 = 0;
    v15 = sub_10b80(0,3,*(unsigned long *)(v10 + 0x20));
    v9 = dcgettext(NULL,"fts_read failed: %s",5);
    error(0,*v14,v9,v15);
  }
  dat_1b188 = 0;
  if (sub_abe0(v10)) {
    v40 = 0;
    v9 = dcgettext(NULL,"fts_close failed",5);
    error(0,*v14,v9);
  }
  v46 = (bool)(v46 & v40);
  goto label_4290;
label_4fc8:
  while( true ) {
    v9 = dat_1b160;
    v7 = strncmp(dat_1b160,"posix-",6);
    v15 = v28;
    if (v7) break;
    dat_1b160 = &v9[6];
  }
label_41a2:
  if (*v9 != '+') { // branch-flip
label_4d36:
    v8 = sub_5ce0("time style",v9,0x1a6a0,0x156e0,4,dat_1b020,1,v15);
    v7 = *(int *)(v8 * 4 + 0x156e0);
    if (v7 != 1) { // branch-flip
      if (v7 != 2) { // branch-flip
        if (!v7)
          dat_1b158 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      else {
        dat_1b158 = "%Y-%m-%d";
      }
    }
    else {
      dat_1b158 = "%Y-%m-%d %H:%M";
    }
  }
  else {
label_41ad:
    dat_1b158 = &v9[1];
  }
label_41b8:
  if (v43) { // branch-flip
    if (optind < (int)a0) {
      v15 = sub_10ef0(a1[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v15);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_5540(1); // no-return
    }
    if ((strcmp(v43,"-")) && (v8 = sub_8e20(v43,"r",stdin), !v8)) {
      v15 = sub_10800(4,v43);
      v9 = dcgettext(NULL,"cannot open %s for reading",5);
      error(1,*__errno_location(),v9,v15);
      return v18;
    }
    v8 = sub_5e40(stdin);
    dat_1b183 = v42;
  }
  else {
    v37 = &v34;
    if (optind < (int)a0)
      v37 = &a1[optind];
    v8 = sub_5e10(v37);
    dat_1b183 = optind + 1 < (int)a0 || v5 == 2;
  }
  if ((v8) && (dat_1b198 = sub_6130(), dat_1b198)) {
    if ((dat_1b184) || (!dat_1b183))
      v44 |= 0x100;
    v46 = v42;
label_4290:
    while (v9 = (char *)sub_5e80(v8,&v31), v9) {
      if (v43) { // branch-flip
        if ((strcmp(v43,"-")) || (strcmp(v9,"-"))) {
          if (*v9) goto label_42e9;
label_4788:
          v15 = sub_5f20(v8);
          v9 = dcgettext(NULL,"invalid zero-length file name",5);
          error(0,0,"%s:%lu: %s",sub_10b80(0,3,v43),v15,v9);
        }
        else {
          v15 = sub_10800(4,v9);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v15);
          if (!*v9) goto label_4788;
        }
        v46 = 0;
      }
      else {
        if (*v9) {
label_42e9:
          dat_1b110 = v9;
          v10 = sub_121c0(0x1b110,v44 | v5,0);
          v40 = v42;
label_430c:
          v11 = (long *)sub_add0(v10);
          if (!v11) goto label_4b60;
          v1 = v11[7];
          v35 = *(unsigned short *)&v11[0xd];
          if (v35 != 4) { // branch-flip
            v30 = v42;
            if (v35 == 6) goto label_4510;
            v6 = sub_69c0(dat_1b140,v1);
            if (v6) {
label_43a8:
              if (v35 == 1) {
                sub_b520(v10,v11,4);
                if (v11 != (long *)sub_add0(v10))
                  __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
              }
              goto label_430c;
            }
            if (v35 == 0xb) {
              sub_b520(v10,v11,1);
              if (v11 != (long *)sub_add0(v10))
                __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
              v35 = *(unsigned short *)&v11[0xd];
            }
            if (v35 == 10 || v35 == 0xd) {
              v40 = 0;
              v15 = sub_10800(4,v1);
              v9 = dcgettext(NULL,"cannot access %s",5);
              error(0,(int)v11[8],v9,v15);
              goto label_430c;
            }
            if (((*(unsigned char *)(v10 + 0x48) & 0x40) && (1 <= v11[0xb])) && (*(long *)(v10 + 0x18) != v11[0xe])) goto label_43a8;
            if ((!dat_1b184) && ((dat_1b183 || (((*(unsigned int *)&v11[0x11] & 0xf000) != 0x4000 && (2 <= (unsigned long)v11[0x10])))))) {
              v7 = sub_61d0(dat_1b198,v11[0xe],v11[0xf]);
              if (v7 < 0) {
                sub_12180(); // no-return, return-dupe
              }
              if (!v7) goto label_43a8;
            }
            if (v35 == 2) {
              if (sub_12210(v10,v11)) {
                v4 = (long *)*v11;
                if (!dat_1b190) {
                  dat_1b190 = sub_6130();
                  if (!dat_1b190) {
                    sub_12180();
                  }
                  v23 = sub_df90(0);
                  while (v23) {
                    if (((!(*(unsigned char *)(v23 + 0x28) & 3)) && (v9 = *(char **)(v23 + 8), !stat(v9,&v33))) && (v7 = sub_61d0(dat_1b190,v33,v51), v7 <= -1)) {
                      sub_12180();
                    }
                    v12 = *(long *)(v23 + 0x30);
                    sub_e9f0(v23);
                    v23 = v12;
                  }
                }
                do {
                  if (v11 == v4) break;
                  v7 = sub_6260(dat_1b190,v11[0xe],v11[0xf]);
                  if (0 < v7) goto label_430c;
                  v11 = (long *)v11[1];
                } while (v11);
                v40 = 0;
                v15 = sub_10b80(0,3,v1);
                error(0,0,dcgettext(NULL,"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n",5),v15);
              }
              goto label_430c;
            }
            if (v35 != 7) {
              if (v35 != 1) goto label_4510;
              goto label_430c;
            }
            v15 = sub_10b80(0,3,v1);
            error(0,(int)v11[8],"%s",v15);
            v30 = 0;
          }
          else {
            v30 = 0;
            v15 = sub_10800(4,v1);
            v9 = dcgettext(NULL,"cannot read directory %s",5);
            error(0,(int)v11[8],v9,v15);
          }
label_4510:
          if (dat_1b168) { // branch-flip
            if (dat_1b168 != 2) { // branch-flip
              v22 = v11[0x1b];
              v39 = v11[0x1c];
            }
            else {
              v22 = v11[0x17];
              v39 = v11[0x18];
            }
          }
          else {
            v22 = v11[0x19];
            v39 = v11[0x1a];
          }
          if (dat_1b185) { // branch-flip
            v38 = 0;
            if (0 <= v11[0x14])
              v38 = v11[0x14];
          }
          else {
            v38 = v11[0x16] << 9;
          }
          v2 = v11[0xb];
          v47 = 1;
          v32 = v38;
          v48 = v22;
          v49 = v39;
          if (dat_1b108) { // branch-flip
            if (v2 != dat_1b188) {
              if (v2 <= dat_1b188) { // branch-flip
                if (v2 != dat_1b188 - 1)
                  __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
                v13 = (unsigned long *)(dat_1b188 * 0x40 + dat_1b100);
                v32 = v38 + *v13;
                if (CARRY8(v38,*v13))
                  v32 = 0xffffffffffffffff;
                v47 = v13[1] + 1;
                if ((int)(((unsigned int)((long)v13[3] < (long)v39) - (unsigned int)((long)v39 < (long)v13[3])) + ((unsigned int)((long)v13[2] < (long)v22) - (unsigned int)((long)v22 < (long)v13[2])) * 2) <= -1) {
                  v48 = v13[2];
                  v49 = v13[3];
                }
                if (!dat_1b180) {
                  v41 = CARRY8(v32,v13[4]);
                  v32 += v13[4];
                  if (v41)
                    v32 = 0xffffffffffffffff;
                  v47 += v13[5];
                  if ((int)(((unsigned int)((long)v13[7] < (long)v49) - (unsigned int)((long)v49 < (long)v13[7])) + ((unsigned int)((long)v13[6] < (long)v48) - (unsigned int)((long)v48 < (long)v13[6])) * 2) <= -1) {
                    v48 = v13[6];
                    v49 = v13[7];
                  }
                }
                v23 = v2 * 0x40 + dat_1b100;
                v21 = *(unsigned long *)(v23 + 0x30);
                v36 = 0xffffffffffffffff;
                if (!CARRY8(*v13,*(unsigned long *)(v23 + 0x20)))
                  v36 = *v13 + *(unsigned long *)(v23 + 0x20);
                v45 = *(long *)(v23 + 0x38);
                *(unsigned long *)(v23 + 0x20) = v36;
                v12 = v13[1] + *(long *)(v23 + 0x28);
                v3 = v13[2];
                *(long *)(v23 + 0x28) = v12;
                if ((int)(((unsigned int)((long)v13[3] < v45) - (unsigned int)(v45 < (long)v13[3])) + ((unsigned int)((long)v3 < (long)v21) - (unsigned int)((long)v21 < (long)v3)) * 2) <= -1) {
                  v3 = v13[3];
                  v21 = v13[2];
                  *(unsigned long *)(v23 + 0x30) = v13[2];
                  *(unsigned long *)(v23 + 0x38) = v3;
                  v45 = *(long *)(v23 + 0x38);
                }
                v19 = v36 + v13[4];
                if (CARRY8(v36,v13[4]))
                  v19 = -1;
                *(unsigned long *)(v23 + 0x28) = v12 + v13[5];
                v36 = v13[6];
                *(long *)(v23 + 0x20) = v19;
                if ((int)(((unsigned int)((long)v13[7] < v45) - (unsigned int)(v45 < (long)v13[7])) + ((unsigned int)((long)v36 < (long)v21) - (unsigned int)((long)v21 < (long)v36)) * 2) <= -1) {
                  v21 = v13[7];
                  *(unsigned long *)(v23 + 0x30) = v13[6];
                  *(unsigned long *)(v23 + 0x38) = v21;
                }
              }
              else {
                if (dat_1b108 <= v2) {
                  dat_1b100 = sub_11d40(dat_1b100,v2,0x80);
                  dat_1b108 = v2 * 2;
                }
                v21 = dat_1b188 + 1;
                if (v21 <= v2) {
                  v23 = dat_1b188 * 0x40 + dat_1b100;
                  do {
                    v21 += 1;
                    *(unsigned long *)(v23 + 0x40) = 0;
                    v12 = v23 + 0x40;
                    *(unsigned long *)(v23 + 0x48) = 0;
                    *(unsigned long *)(v23 + 0x50) = 0x8000000000000000;
                    *(unsigned long *)(v23 + 0x58) = 0xffffffffffffffff;
                    *(unsigned long *)(v23 + 0x60) = 0;
                    *(unsigned long *)(v23 + 0x68) = 0;
                    *(unsigned long *)(v23 + 0x70) = 0x8000000000000000;
                    *(unsigned long *)(v23 + 0x78) = 0xffffffffffffffff;
                    v23 = v12;
                  } while (v21 <= v2);
                }
              }
            }
          }
          else {
            dat_1b108 = v2 + 10;
            dat_1b100 = sub_12060(dat_1b108,0x40);
          }
          if ((!dat_1b180) || ((v35 & 0xfffd) != 4)) {
            v13 = (unsigned long *)(v2 * 0x40 + dat_1b100);
            v21 = v38 + *v13;
            if (CARRY8(v38,*v13))
              v21 = 0xffffffffffffffff;
            v13[1] = v13[1] + 1;
            *v13 = v21;
            if ((int)(((unsigned int)((long)v39 < (long)v13[3]) - (unsigned int)((long)v13[3] < (long)v39)) + ((unsigned int)((long)v22 < (long)v13[2]) - (unsigned int)((long)v13[2] < (long)v22)) * 2) <= -1) {
              v13[2] = v22;
              v13[3] = v39;
            }
          }
          v41 = CARRY8(v38,dat_1b120);
          dat_1b120 = v38 + dat_1b120;
          if (v41)
            dat_1b120 = 0xffffffffffffffff;
          dat_1b128 += 1;
          if ((int)(((unsigned int)((long)v39 < (long)dat_1b138) - (unsigned int)((long)dat_1b138 < (long)v39)) + ((unsigned int)((long)v22 < (long)dat_1b130) - (unsigned int)((long)dat_1b130 < (long)v22)) * 2) <= -1) {
            dat_1b130 = v22;
            dat_1b138 = v39;
          }
          dat_1b188 = v2;
          if (((((v35 & 0xfffd) != 4) && (!dat_1b186)) || (dat_1b010 < v2)) && (v2))
            v40 = (bool)(v40 & v30);
          else {
            v22 = v47;
            if (!dat_1b16d)
              v22 = v32;
            v41 = dat_1b178 <= v22;
            if ((long)dat_1b178 <= -1)
              v41 = v22 <= -dat_1b178;
            v40 = (bool)(v40 & v30);
            if (v41)
              sub_53d0(&v32,v1);
          }
          goto label_430c;
        }
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
        v46 = 0;
      }
    }
    if (v31 != 3) {
      if (v31 != 4) { // branch-flip
        if (v31 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
      }
      else {
        v15 = sub_10b80(0,3,v43);
        v9 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v9,v15);
        v46 = 0;
      }
      sub_5f40(v8);
      sub_61a0(dat_1b198);
      if (dat_1b190)
        sub_61a0(dat_1b190);
      if (((v43) && ((*stdin & 0x20 || (v7 = sub_71a0(stdin), v7)))) && (v46)) {
        v15 = sub_10800(4,v43);
        error(1,0,dcgettext(NULL,"error reading %s",5),v15);
        return v17;
      }
      if (dat_1b181)
        sub_53d0(0x1b120,dcgettext(NULL,"total",5));
      return (unsigned long)(v46 ^ 1);
    }
  }
  sub_12180();
}


// Function: sub_5340 @ 0x5340
void sub_5340(long a0)
{
  void *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != -1) // branch-flip
    v2 = (char *)sub_c920(a0,v3,dat_1b170,1,dat_1b148);
  else {
    v2 = dcgettext(NULL,"Infinity",5);
  }
  fputs_unlocked(v2,v1);
}


// Function: sub_53d0 @ 0x53d0
void sub_53d0(unsigned long *a0,unsigned long a1)
{
  char *v1;
  unsigned long v2;
  long v3; // rax
  char *v4; // rax
  unsigned long v5;
  unsigned long v6; // stack - 0x98
  char v7 [64];
  char v8 [24];
  unsigned long v9; // stack - 0x90
  
  v5 = a0[1];
  if (!dat_1b16d)
    v5 = *a0;
  sub_5340(v5);
  if (dat_1b16c) {
    v1 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v1) // branch-flip
      __overflow(stdout,9);
    else {
      *(char **)((long)stdout + 0x28) = &v1[1];
      *v1 = 9;
    }
    v2 = dat_1b158;
    v5 = dat_1b150;
    v6 = a0[2];
    v9 = a0[3];
    v3 = sub_112f0(dat_1b150,&v6,v7);
    if (v3) // branch-flip
      sub_8e00(stdout,v2,v7,v5,v9 & 0xffffffff);
    else {
      v4 = (char *)sub_d920(v6,v8);
      v5 = sub_10ef0(v4);
      error(0,0,dcgettext(NULL,"time %s is out of range",5),v5);
      fputs_unlocked(v4,stdout);
    }
  }
  __printf_chk(1,"\t%s%c",a1,(unsigned long)(-(unsigned int)(dat_1b182 == '\0') & 10));
  fflush_unlocked(stdout); // tail-call
}


// Function: sub_5540 @ 0x5540
void sub_5540(int a0)
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
  
  v2 = dat_1b1b0;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Summarize device usage of the set of FILEs, recursively for directories.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n",5),"DU");
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1);
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
      if (!strcmp("du",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "du";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "du";
    if (!strcmp("du","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "du")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}


// Function: sub_b750 @ 0xb750
unsigned long sub_b750(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}


// Function: sub_b770 @ 0xb770
bool sub_b770(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_b780 @ 0xb780
unsigned long sub_b780(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x15aa0)
    return 1;
  v1 = v2[2];
  if ((((dat_15ab4 < v1) && (v1 < dat_15ab8)) && (dat_15abc < v2[3])) && (dat_15ad0 <= *v2)) {
    v3 = *v2 + dat_15ab4;
    if (((v3 < v2[1]) && (v2[1] <= dat_15ac0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x15aa0;
  return 0;
}


// Function: sub_b810 @ 0xb810
unsigned long sub_b810(long *a0,long a1,unsigned long *a2,char a3) // early-return
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


// Function: sub_ba60 @ 0xba60
unsigned long sub_ba60(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_ba96:
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
        goto label_ba96;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}


// Function: sub_bcd0 @ 0xbcd0
void sub_bcd0(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_bd3d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_bd3d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}


// Function: sub_be30 @ 0xbe30
long sub_be30(long *a0,long a1) // return-dupe
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


// Function: sub_bef0 @ 0xbef0
long sub_bef0(long *a0,long a1)
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


// Function: sub_bfd0 @ 0xbfd0
long sub_bfd0(unsigned long *a0,void *a1,unsigned long a2)
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


// Function: sub_c050 @ 0xc050
unsigned long sub_c050(unsigned char *a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  
  v2 = 0;
  v1 = *a0;
  while (v1) {
    a0 = &a0[1];
    v2 = (v2 * 0x1f + (unsigned long)v1) % a1;
    v1 = *a0;
  }
  return v2;
}


// Function: sub_c0c0 @ 0xc0c0
unsigned long * sub_c0c0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_b750;
  if (!a3)
    a3 = sub_b770;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x15aa0;
  v1[5] = a1;
  if (sub_b780(v1)) {
    v2 = sub_b920(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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


// Function: sub_c1d0 @ 0xc1d0
void sub_c1d0(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_c26b;
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
label_c26b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_c280 @ 0xc280
void sub_c280(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_c32c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_c2ef;
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
label_c2ef:
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
label_c32c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_c370 @ 0xc370
unsigned int sub_c370(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_b920(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_ba60(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_ba60(a0,&v2,1)) && (sub_ba60(a0,&v2,0))) {
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


// Function: sub_c4f0 @ 0xc4f0
long * sub_c4f0(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_b810(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_c57f;
label_c606:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_c606;
label_c57f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_b780(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_15ac4 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_15ac8 <= v5) ? (long)(v5 - dat_15ac8) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_c370(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_b810(a0,a1,&v3,0);
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


// Function: sub_c720 @ 0xc720
unsigned long sub_c720(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_c4f0(a0,a1,&v3);
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


// Function: sub_c780 @ 0xc780
long sub_c780(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_b810(a0,a1,&v7,1);
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
      sub_b780(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_15ac8 <= v8) ? (long)(v8 - dat_15ac8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_c370(a0,v6)) {
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

