// Function: main @ 0x2a20
int main(int argc,char **argv,char **envp)
{
  long *v1;
  int *v10;
  FILE *v11;
  FILE *v12;
  unsigned char v13;
  char v14;
  int v15; // eax
  int v16;
  int v17;
  int v18;
  int v19; // eax
  long *v2;
  unsigned int v20;
  int v21;
  long v22;
  long v23;
  long v24;
  long v25; // rax
  long v26;
  unsigned long v27;
  char *v28;
  char *v29;
  char *v3;
  long v30;
  long v31;
  long *v32;
  char *v33 [4]; // stack - 0xf8
  stat v34; // stack - 0xd8
  int v35 [5]; // stack - 0x160
  char v36 [8];
  void *v37; // stack - 0x170
  void *v38; // stack - 0x178
  int *v39; // stack - 0x168
  char v4;
  long v40 [6]; // stack - 0x148
  char v41; // stack - 0x42
  unsigned char v42;
  long v43;
  long v44;
  unsigned long v45; // rdi
  unsigned long v46;
  long v47;
  long v48;
  char **v49;
  long v5;
  int *v50;
  int *v51;
  long v52;
  bool v53; // cf
  char *v54; // stack - 0x1f0
  long *v55; // stack - 0x1e8
  int **v56; // stack - 0x1d0
  long v57; // stack - 0x1b0
  unsigned char v58; // stack - 0x18d
  int v59; // stack - 0x14c
  void *v6;
  char v60; // stack - 0x41
  long v61 [4]; // stack - 0x118
  void *v7;
  char *v8;
  unsigned char *v9;
  
  v20 = 0;
  dat_11020 = 2;
  sub_50e0(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_5020(NULL);
  sub_7940();
  v16 = 0;
  while( true ) {
    v15 = getopt_long(argc,argv,"aeimvx3AEL:TX",(option *)0x10940,NULL);
    v27 = dat_21140;
    if (v15 == -1) break;
    switch(v15) {
      case 0x33:
        dat_110ed = '\x01';
        v20 |= 1;
        break;
      default:
        sub_42a0(NULL,0); // no-return
      case 0x41:
        dat_110eb = 1;
        v20 |= 2;
        dat_110ef = 1;
        break;
      case 0x45:
        dat_110ef = 1;
        v20 |= 4;
        break;
      case 0x4c:
        if (2 < v16)
          sub_42a0("too many file label options",0); // no-return
        v33[v16] = optarg;
        v16 += 1;
        break;
      case 0x54:
        dat_110ee = '\x01';
        break;
      case 0x58:
        dat_110ec = '\x01';
        v20 |= 8;
        break;
      case 0x61:
        dat_110f2 = 1;
        break;
      case 0x65:
        v20 |= 0x10;
        break;
      case 0x69:
        dat_110ea = 1;
        break;
      case 0x6d:
        dat_110e9 = 1;
        break;
      case 0x76:
        sub_7150(stdout,"diff3","GNU diffutils",dat_11018,sub_5580("Randy Smith"),0);
        sub_4690();
        return 0;
      case 0x78:
        dat_110ec = '\x01';
        v20 |= 0x20;
        break;
      case 0x80:
        dat_11010 = optarg;
        break;
      case 0x81:
        v22 = 0x108a0;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5),v27);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare three files line by line.",5));
        v11 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v11);
        v28 = "-A, --show-all              output all changes, bracketing conflicts";
        do {
          if (*v28) { // branch-flip
            __printf_chk(1,"  %s\n",dcgettext(NULL,v28,5));
            v28 = *(char **)(v22 + 8);
            v11 = stdout;
          }
          else {
            v8 = *(char **)&stdout->field_0x28;
            if (*(char **)&stdout->field_0x30 <= v8) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)&stdout->field_0x28 = &v8[1];
              *v8 = 10;
            }
            v28 = *(char **)(v22 + 8);
            v11 = stdout;
          }
          v22 += 8;
          stdout = v11;
        } while (v28);
        fputs_unlocked(dcgettext(NULL,"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n",5),v11);
        v28 = dcgettext(NULL,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
        __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v28);
        sub_7230();
        sub_4690();
        return 0;
      case 0x82:
        dat_110f1 = 1;
      
    }
  }
  dat_110f0 = (dat_110e9 ^ 1) & v20 != 0;
  v13 = v20 == 0 & dat_110e9;
  dat_110eb |= v13;
  dat_110ef = v13 | dat_110ef;
  if (((v20 - 1 & v20) || (dat_110e9 & dat_110ea)) || ((v16 && (!dat_110ef))))
    sub_42a0("incompatible options",0); // no-return
  if (argc - optind != 3) {
    if (argc - optind > 2)
      sub_42a0("extra operand \'%s\'",argv[(long)optind + 3]); // no-return
    sub_42a0("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  argv = &argv[optind];
  for (v22 = (long)v16; (int)v22 <= 2; v22 = v22 + 1) {
    v33[v22] = argv[v22];
  }
  v13 = v20 != 0 | dat_110e9;
  v16 = 2 - (unsigned int)v13;
  v57 = (long)v16;
  v28 = argv[v57];
  if (strcmp(v28,"-")) {
label_2ed8:
    v35[0] = 0;
    v35[3] = 0;
    v15 = 3 - v16;
    v22 = (long)v15;
    v35[v22 + 3] = 1;
    v35[v57 + 3] = 2;
    v49 = argv;
    v35[1] = v15;
    v35[2] = v16;
    do {
      v28 = *v49;
      if (strcmp(v28,"-")) {
        if (stat(v28,&v34) < 0)
          sub_4330(*v49); // no-return
        if ((v34._24_4_ & 0xf000) == 0x4000) {
          error(2,0x15,"%s",*v49);
          return v19;
        }
      }
      v49 = &v49[1];
    } while (&argv[3] != v49);
    signal(0x11,0);
    v50 = (int *)0xcd20;
    v28 = argv[v59];
    v23 = sub_46e0(argv[v35[4]],v28,v36,&v37);
    v40[4] = sub_46e0(*argv,v28,v36,&v38);
    v40[5] = v23;
    v56 = &v39;
    v39 = NULL;
label_2ff0:
    v23 = v61[3];
    v47 = v61[2];
    v44 = v61[1];
    v52 = v61[0];
    v51 = v39;
    v42 = dat_110f0;
    v13 = dat_110e9;
    v11 = stdout;
    if (v40[4]) {
      v40[3] = 0;
      v40[2] = 0;
      v40[1] = 0;
      v40[0] = 0;
      if (v40[5]) { // branch-flip
        v43 = *(long *)(v40[5] + 0x10);
        v48 = *(long *)(v40[4] + 0x10);
        v45 = (unsigned long)(v43 < v48);
        v20 = (unsigned int)(v43 < v48);
        v46 = (unsigned long)(v48 <= v43);
        v48 = v45 << 3;
        v43 = v40[v45 + 4];
      }
      else {
        v20 = 0;
        v48 = 0;
        v46 = 1;
        v45 = 0;
        v43 = v40[4];
      }
label_306f:
      v40[3] = 0;
      v40[2] = 0;
      v40[1] = 0;
      v40[0] = 0;
      v24 = *(long *)(v43 + 0x40);
      v30 = *(long *)(v43 + 0x18);
      v40[v45 + 2] = v43;
      v40[v45] = v43;
      v40[v45 + 4] = v24;
      v24 = v40[v46 + 4];
      *(unsigned long *)(v43 + 0x40) = 0;
      if (v24) {
        do {
          if (v30 + 1 < *(long *)(v24 + 0x10)) break;
          if (v40[v46]) // branch-flip
            *(long *)(v40[v46 + 2] + 0x40) = v24;
          else {
            v40[v46] = v24;
          }
          v40[v46 + 2] = v24;
          v43 = *(long *)(v40[v46 + 4] + 0x40);
          *(unsigned long *)(v24 + 0x40) = 0;
          v24 = *(long *)(v24 + 0x18);
          v40[v46 + 4] = v43;
          v21 = v20;
          v17 = v20 ^ 1;
          if (v30 < v24) {
            v30 = v24;
            v21 = v20 ^ 1;
            v17 = v20;
          }
          v20 = v21;
          v46 = (unsigned long)v17;
          v24 = v40[v46 + 4];
        } while (v24);
        v45 = (unsigned long)(int)v20;
      }
      v43 = *(long *)(*(long *)((long)v40 + v48) + 0x10);
      v48 = *(long *)(v40[v45 + 2] + 0x18);
      v24 = 0;
      do {
        v32 = *(long **)((long)v40 + v24);
        if (v32) { // branch-flip
          *(long *)((long)v40 + v24 + 0x30) = (v43 - v32[2]) + *v32;
          v30 = *(long *)(v24 + (long)&v40[2]);
          v30 = (v48 - *(long *)(v30 + 0x18)) + *(long *)(v30 + 8);
        }
        else {
          *(long *)((long)v40 + v24 + 0x30) = (v43 - *(long *)&v50[0xc]) + *(long *)((long)v50 + v24 * 2 + 0x10);
          v30 = (v48 - *(long *)&v50[0xc]) + *(long *)((long)v50 + v24 * 2 + 0x10);
        }
        *(long *)((long)v40 + v24 + 0x40) = v30;
        v24 += 8;
      } while (v24 != 0x10);
      v50 = (int *)sub_7350(0x70);
      *v50 = 0;
      v50[0x1a] = 0;
      v50[0x1b] = 0;
      *(long *)&v50[2] = v52;
      *(long *)&v50[6] = v44;
      *(long *)&v50[4] = v47;
      *(long *)&v50[8] = v23;
      *(long *)&v50[10] = v43;
      *(long *)&v50[0xc] = v48;
      v46 = (v47 - v52) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0xe] = sub_76b0(v46,8);
        v27 = sub_76b0(v46,8);
        v23 = *(long *)&v50[8];
        v44 = *(long *)&v50[6];
        *(unsigned long *)&v50[0x14] = v27;
      }
      else {
        v50[0xe] = 0;
        v50[0xf] = 0;
        v50[0x14] = 0;
        v50[0x15] = 0;
      }
      v46 = (v23 - v44) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0x10] = sub_76b0(v46,8);
        v27 = sub_76b0(v46,8);
      }
      else {
        v50[0x10] = 0;
        v50[0x11] = 0;
        v27 = 0;
      }
      *(unsigned long *)&v50[0x16] = v27;
      v46 = (*(long *)&v50[0xc] - *(long *)&v50[10]) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0x12] = sub_76b0(v46,8);
        v23 = sub_76b0(v46,8);
      }
      else {
        v50[0x12] = 0;
        v50[0x13] = 0;
        v23 = 0;
      }
      *(long *)&v50[0x18] = v23;
      v55 = v40;
      do {
        v44 = *v55;
        if (v44) {
          v48 = *(long *)&v50[0x12];
          do {
            v24 = *(long *)(v44 + 0x10);
            v30 = *(long *)(v44 + 0x18);
            v26 = *(long *)(v44 + 0x38);
            v5 = *(long *)(v44 + 0x28);
            v31 = (v24 - v43) * 8;
            if (!sub_4460(v5,v26,v31 + v48,v23 + v31,(v30 - v24) + 1)) {
              sub_4300("internal error: screwup in format of diff blocks"); // return-dupe, no-return
            }
            v44 = *(long *)(v44 + 0x40);
          } while (v44);
        }
        v55 = &v55[1];
      } while (&v40[2] != v55);
      v44 = 0;
      do {
        v32 = *(long **)((long)v40 + v44);
        v24 = 0;
        v48 = v52;
        while( true ) {
          v30 = v47 + 1;
          if (v32)
            v30 = *v32;
          if (v30 <= v48) break;
          v48 += 1;
          *(unsigned long *)(*(long *)((long)v50 + v44 + 0x38) + v24) = *(unsigned long *)(*(long *)&v50[0x12] + v24);
          *(unsigned long *)(*(long *)((long)v50 + v44 + 0x50) + v24) = *(unsigned long *)(v23 + v24);
          v24 += 8;
        }
        if (v32) {
          v48 = *(long *)((long)v50 + v44 + 0x38);
          v24 = *(long *)((long)v50 + v44 + 0x50);
label_33b0:
          v30 = *v32;
          v26 = v32[1];
          v5 = v32[6];
          v31 = v32[4];
          v25 = (v30 - v52) * 8;
          if (sub_4460(v31,v5,v48 + v25,v24 + v25,(v26 - v30) + 1)) {
            v1 = &v32[1];
            v2 = &v32[3];
            v32 = (long *)v32[8];
            v30 = (*v2 - v43) * 8;
            v26 = (*v1 + 1) - v52;
            do {
              v30 += 8;
              if (v32) { // branch-flip
                if (*v32 - v52 <= v26) goto label_33b0;
              }
              else if ((v47 + 1) - v52 <= v26) goto label_3447;
              *(unsigned long *)(v48 + v26 * 8) = *(unsigned long *)(*(long *)&v50[0x12] + v30);
              *(unsigned long *)(v24 + v26 * 8) = *(unsigned long *)(v23 + v30);
              v26 += 1;
            } while( true );
          }
          sub_4300("internal error: screwup in format of diff blocks");
        }
label_3447:
        if (v44 + 8 == 0x10) goto label_3455;
        v52 = *(long *)((long)v40 + v44 + 0x38);
        v47 = *(long *)((long)v40 + v44 + 0x48);
        v44 += 8;
      } while( true );
    }
    if (v40[5]) {
      v40[3] = 0;
      v46 = 0;
      v48 = 8;
      v20 = 1;
      v40[2] = 0;
      v45 = 1;
      v40[1] = 0;
      v40[0] = 0;
      v43 = v40[5];
      goto label_306f;
    }
    if (dat_110f0) {
      v50 = NULL;
      while (v51) {
        v10 = *(int **)&v51[0x1a];
        *(int **)&v51[0x1a] = v50;
        v50 = v51;
        v51 = v10;
      }
      v58 = 0;
      do {
        if (!v50) {
          if (dat_110ea)
            fputs_unlocked("w\nq\n",v11);
          goto label_3918;
        }
        if (*v50 != 4) { // branch-flip
          v18 = v35[(unsigned long)(unsigned int)(*v50 - 5) + 3];
          if (v18 != 1) { // branch-flip
            if (v18 != 2) { // branch-flip
              if (v18 == -1) goto label_3da6;
            }
            else if (!dat_110ec) {
              v23 = *(long *)&v50[2];
              v44 = *(long *)&v50[4];
              goto label_3f3d;
            }
          }
          else if (dat_110eb) {
            v44 = *(long *)&v50[4];
            v23 = *(long *)&v50[2];
            __fprintf_chk(v11,1,"%lda\n",v44);
            __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
            sub_45f0(v11,0,v44 + 2,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 3 + (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2]));
            __fprintf_chk(v11,1,"%lda\n<<<<<<< %s\n",v23 + -1,v33[1]);
            v14 = sub_4500(v11,v50,v15);
            fputs_unlocked("=======\n",v11);
            goto label_4072;
          }
        }
        else {
label_3da6:
          if (!dat_110ed) {
            v23 = *(long *)&v50[2];
            v44 = *(long *)&v50[4];
            if (dat_110ef) { // branch-flip
              __fprintf_chk(v11,1,"%lda\n",v44);
              v20 = (unsigned int)dat_110eb;
              if (dat_110eb) {
                __fprintf_chk(v11,1,"||||||| %s\n",v33[1]);
                v20 = sub_4500(v11,v50,v15);
              }
              fputs_unlocked("=======\n",v11);
              v21 = sub_4500(v11,v50,v16);
              __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
              sub_45f0(v11,(v20 | v21) & 0xff,v44 + 2,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 3 + (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2]));
              __fprintf_chk(v11,1,"%lda\n<<<<<<< %s\n",v23 + -1,v33[0]);
              v14 = 0;
label_4072:
              sub_45f0(v11,(unsigned char)v14,v23 + 1,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 1);
              v58 = v42;
            }
            else {
label_3f3d:
              if (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2] != -1) { // branch-flip
                if (v44 - v23 != -1) { // branch-flip
                  if (v44 != v23) // branch-flip
                    __fprintf_chk(v11,1,"%ld,%ldc\n",v23,v44);
                  else {
                    __fprintf_chk(v11,1,"%ldc\n",v44);
                  }
                }
                else {
                  __fprintf_chk(v11,1,"%lda\n",v44);
                }
                v44 = *(long *)&v50[v57 * 4 + 4];
                v52 = *(long *)&v50[v57 * 4 + 2];
                sub_45f0(v11,(unsigned char)sub_4500(v11,v50,v16),v23,(v44 - v52) + 1);
              }
              else if (v23 != v44) // branch-flip
                __fprintf_chk(v11,1,"%ld,%ldd\n",v23,v44);
              else {
                __fprintf_chk(v11,1,"%ldd\n",v23);
              }
            }
          }
        }
        v50 = *(int **)&v50[0x1a];
      } while( true );
    }
    if (!dat_110e9) {
      v28 = "  ";
      if (dat_110ee)
        v28 = "\t";
      for (v50 = v39; v50; v50 = *(int **)&v50[0x1a]) {
        if (*v50 != 4) { // branch-flip
          v20 = *v50 - 5;
          if (3 <= v20)
            sub_4300("internal error: invalid diff type passed to output"); // no-return
          v54._0_4_ = v35[(unsigned long)v20 + 3];
          v60 = 0;
          v41 = (char)(int)v54 + '1';
          v55._0_4_ = (unsigned int)((int)v54 == 0);
        }
        else {
          v41 = '\0';
          v55._0_4_ = 3;
          v54._0_4_ = 3;
        }
        __fprintf_chk(v11,1,"====%s\n",&v41);
        v20 = 0;
        do {
          v44 = (long)v35[(int)v20];
          v22 = *(long *)&v50[v44 * 4 + 2];
          v23 = *(long *)&v50[v44 * 4 + 4];
          v21 = v20 + 1;
          __fprintf_chk(v11,1,"%d:",v21);
          if (v22 != v23) { // branch-flip
            if (v22 - v23 != 1) // branch-flip
              __fprintf_chk(v11,1,"%ld,%ldc\n",v22,v23);
            else {
              __fprintf_chk(v11,1,"%lda\n",v22 + -1);
            }
          }
          else {
            __fprintf_chk(v11,1,"%ldc\n",v22);
          }
          if ((v22 <= v23) && ((unsigned int)v55 != v20)) {
            v52 = 0;
            do {
              fputs_unlocked(v28,v11);
              v6 = *(void **)(*(long *)&v50[(v44 + 6) * 2 + 2] + v52 * 8);
              v46 = *(unsigned long *)(*(long *)&v50[v44 * 2 + 0x14] + v52 * 8);
              v52 += 1;
              fwrite_unlocked(v6,1,v46,v11);
            } while (v52 <= v23 - v22);
            if (*(char *)((long)v6 + (v46 - 1)) != '\n')
              __fprintf_chk(v11,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5));
          }
          if ((int)v54 == 1)
            v21 = *(unsigned int *)((long)(int)v20 * 4 + 0xcd00);
          v20 = v21;
        } while (v21 <= 2);
      }
      v58 = 0;
      goto label_3918;
    }
    sub_7810(*argv,"r",stdin);
    v12 = stdin;
    v11 = stdout;
    v58 = 0;
    v23 = 0;
    goto label_36fa;
  }
  v28 = *argv;
  v16 = v13 + 1;
  if (strcmp(v28,"-")) {
    v57 = (long)v16;
    v28 = argv[v57];
    if (strcmp(v28,"-")) goto label_2ed8;
  }
  sub_4300("\'-\' specified for more than one input file"); // no-return
label_3455:
  v18 = 6;
  if (v40[0]) {
    if (v40[1]) { // branch-flip
      v23 = *(long *)&v50[4];
      v18 = 4;
      v44 = *(long *)&v50[2];
      if (v23 - v44 == *(long *)&v50[8] - *(long *)&v50[6]) {
        v24 = 0;
        v52 = *(long *)&v50[0x16];
        v47 = *(long *)&v50[0x10];
        v43 = *(long *)&v50[0x14];
        v48 = *(long *)&v50[0xe];
        do {
          if ((v23 - v44) + 1 == v24) {
            v18 = 7;
            goto label_3492;
          }
          v6 = *(void **)(v48 + v24 * 8);
          if (((!v6) || (v7 = *(void **)(v47 + v24 * 8), !v7)) || (v46 = *(unsigned long *)(v43 + v24 * 8), v46 != *(unsigned long *)(v52 + v24 * 8))) break;
          v24 += 1;
        } while (!memcmp(v6,v7,v46));
        v18 = 4;
      }
    }
    else {
      v18 = 5;
    }
  }
label_3492:
  *v50 = v18;
  *v56 = v50;
  v56 = (int **)&v50[0x1a];
  goto label_2ff0;
label_36fa:
  if (!v51) {
    do {
      v9 = *(unsigned char **)&v12->field_0x8;
      if (*(unsigned char **)&v12->field_0x10 <= v9) { // branch-flip
        v20 = __uflow(v12);
        if ((v20 == 0xffffffff) && (*(unsigned char *)v12 & 0x30)) {
label_3908:
          if (*(unsigned char *)stdin & 0x20)
            sub_4300("read failed"); // no-return
label_3918:
          free(v38);
          free(v37);
          sub_4690();
          exit((unsigned int)v58); // no-return
        }
      }
      else {
        *(unsigned char **)&v12->field_0x8 = &v9[1];
        v20 = (unsigned int)*v9;
      }
      v8 = *(char **)&v11->field_0x28;
      if (*(char **)&v11->field_0x30 <= v8) // branch-flip
        __overflow(v11,v20 & 0xff);
      else {
        *(char **)&v11->field_0x28 = &v8[1];
        *v8 = (char)v20;
      }
    } while( true );
  }
  if (*v51 != 4) { // branch-flip
    v16 = v35[(unsigned long)(unsigned int)(*v51 - 5) + 3];
    if (v16 != 1) { // branch-flip
      if (v16 != 2) { // branch-flip
        if (v16 == -1) goto label_3736;
      }
      else if (!dat_110ec) {
        v16 = 7;
        v55 = (long)(*(long *)&v51[2] + -1);
        v54 = "<<<<<<< %s\n";
        v23 = (*(long *)&v51[2] - v23) + -2;
        v42 = 0;
        if (0 <= v23) goto label_37a8;
        goto label_382f;
      }
    }
    else if (dat_110eb) {
      v16 = 6;
      v55 = (long)(*(long *)&v51[2] + -1);
      v23 = (*(long *)&v51[2] - v23) + -2;
      v54 = "<<<<<<< %s\n";
      v42 = dat_110eb;
      if (0 <= v23) goto label_37a8;
label_39e2:
      __fprintf_chk(v11,1,v54,v33[1]);
      for (v23 = 0; v23 <= *(long *)&v51[v22 * 4 + 4] - *(long *)&v51[v22 * 4 + 2]; v23 = v23 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v51[v22 * 2 + 0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[v22 * 2 + 0x14] + v23 * 8),v11);
      }
      fputs_unlocked("=======\n",v11); // crossjump-dupe
      v58 = v13;
      v42 = v13;
      goto label_382f;
    }
  }
  else {
label_3736:
    if (!dat_110ed) {
      v55 = (long)(*(long *)&v51[2] + -1);
      v23 = (*(long *)&v51[2] - v23) + -2;
      if (0 <= v23) { // branch-flip
        v16 = 4;
        v54 = "||||||| %s\n";
        v42 = dat_110ef;
label_37a8:
        do {
          v9 = *(unsigned char **)&v12->field_0x8;
          if (*(unsigned char **)&v12->field_0x10 <= v9) { // branch-flip
            v20 = __uflow(v12);
            if (v20 == 0xffffffff) {
              if (*(unsigned int *)v12 & 0x20)
                sub_4330(dcgettext(NULL,"read failed",5)); // return-dupe, no-return
              if (*(unsigned int *)v12 & 0x10)
                sub_4300("input file shrank"); // return-dupe, no-return
            }
          }
          else {
            *(unsigned char **)&v12->field_0x8 = &v9[1];
            v20 = (unsigned int)*v9;
          }
          v8 = *(char **)&v11->field_0x28;
          if (*(char **)&v11->field_0x30 <= v8) // branch-flip
            __overflow(v11,v20 & 0xff);
          else {
            *(char **)&v11->field_0x28 = &v8[1];
            *v8 = (char)v20;
          }
        } while ((v20 != 10) || (v53 = v23 != 0, v23 = v23 + -1, v53));
        v53 = v42 != 0;
        v42 = 0;
        if (v53) {
          if (v16 == 4) goto label_3a91;
label_3808:
          if (dat_110eb) goto label_39e2;
          fputs_unlocked("=======\n",v11);
          v58 = v13;
          v42 = v13;
        }
      }
      else {
        v42 = 0;
        if (dat_110ef) {
          v54 = "||||||| %s\n";
label_3a91:
          __fprintf_chk(v11,1,"<<<<<<< %s\n",v33[0]);
          for (v23 = 0; v23 <= *(long *)&v51[4] - *(long *)&v51[2]; v23 = v23 + 1) {
            fwrite_unlocked(*(void **)(*(long *)&v51[0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[0x14] + v23 * 8),v11);
          }
          goto label_3808;
        }
      }
label_382f:
      for (v23 = 0; v23 <= *(long *)&v51[v57 * 4 + 4] - *(long *)&v51[v57 * 4 + 2]; v23 = v23 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v51[v57 * 2 + 0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[v57 * 2 + 0x14] + v23 * 8),v11);
      }
      if (v42)
        __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
      v23 = (*(long *)&v51[4] - *(long *)&v51[2]) + 1;
      v44 = v23;
label_389a:
      v44 -= 1;
      if (0 <= v44) {
        v28 = *(char **)&v12->field_0x8;
        v29 = *(char **)&v12->field_0x10;
        do {
          while (v28 >= v29) {
            v16 = __uflow(v12);
            if (v16 == 10) goto label_389a;
            if (v16 == -1) {
              if (*(unsigned int *)v12 & 0x20) {
                sub_4330(dcgettext(NULL,"read failed",5));
              }
              if (*(unsigned int *)v12 & 0x10) {
                if ((v44) || (*(long *)&v51[0x1a])) {
                  sub_4300("input file shrank");
                }
                goto label_3908;
              }
            }
            v29 = *(char **)&v12->field_0x10;
            v28 = *(char **)&v12->field_0x8;
          }
          v3 = &v28[1];
          *(char **)&v12->field_0x8 = v3;
          v4 = *v28;
          v28 = v3;
        } while (v4 != '\n');
        goto label_389a;
      }
      v23 = (long)v55 + v23;
    }
  }
  v51 = *(int **)&v51[0x1a];
  goto label_36fa;
}


// Function: sub_42a0 @ 0x42a0
void sub_42a0(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_21140;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_4300 @ 0x4300
void sub_4300(char *a0)
{
  error(2,0,"%s",dcgettext(NULL,a0,5));
}


// Function: sub_4330 @ 0x4330
void sub_4330(char *a0)
{
  error(2,*__errno_location(),"%s",a0);
}


// Function: sub_4360 @ 0x4360
char * sub_4360(char *a0,unsigned long *a1,long *a2,char *a3,char a4)
{
  char *v1;
  char v2;
  char *v3;
  
  if ((*a0 != a4) || (a0[1] != ' '))
    sub_4300("invalid diff format; incorrect leading line chars"); // no-return
  a0 = &a0[2];
  *a1 = (unsigned long)a0;
  v3 = a0;
  while (v1 = &v3[1], *v3 != '\n') {
    v3 = v1;
  }
  *a2 = (long)v1 - (long)a0;
  if ((v1 < a3) && (*v1 == '\\')) {
    if (dat_110f0) // branch-flip
      __fprintf_chk(stderr,1,"%s:",dat_21140);
    else {
      *a2 = ((long)v1 - (long)a0) + -1;
    }
    v3 = &v3[2];
    v2 = dat_110f0;
    while( true ) {
      if (v2) {
        v2 = *v3;
        v1 = *(char **)&stderr->field_0x28;
        if (*(char **)&stderr->field_0x30 <= v1) // branch-flip
          __overflow(stderr,(unsigned int)(unsigned char)v2);
        else {
          *(char **)&stderr->field_0x28 = &v1[1];
          *v1 = v2;
        }
      }
      if (*v3 == '\n') break;
      v3 = &v3[1];
      v2 = dat_110f0;
    }
    return &v3[1];
  }
  return v1;
}


// Function: sub_4460 @ 0x4460
unsigned long sub_4460(long a0,long a1,long a2,long a3,long a4)
{
  void *v1;
  void *v2;
  unsigned long v3;
  long v4; // rbx
  
  if (!a4)
    return 1;
  v4 = 0;
  while( true ) {
    while( true ) {
      v2 = *(void **)(a2 + v4 * 8);
      v3 = *(unsigned long *)(a1 + v4 * 8);
      if (v2) break;
      *(unsigned long *)(a2 + v4 * 8) = *(unsigned long *)(a0 + v4 * 8);
      *(unsigned long *)(a3 + v4 * 8) = v3;
      v4 += 1;
      if (v4 == a4)
        return 1;
    }
    if (*(unsigned long *)(a3 + v4 * 8) != v3)
      return 0;
    v1 = *(void **)(a0 + v4 * 8);
    if (memcmp(v1,v2,v3)) break;
    v4 += 1;
    if (v4 == a4)
      return 1;
  }
  return 0;
}


// Function: sub_4500 @ 0x4500
char sub_4500(FILE *a0,long a1,int a2)
{
  long v1;
  char *v2;
  char *v3;
  long v4; // rbx
  long v5; // r12
  long v6; // r15
  char v7; // stack - 0x41
  
  v6 = (long)a2;
  v5 = v6 * 0x10 + a1;
  if (*(long *)(v5 + 8) <= *(long *)(v5 + 0x10)) { // branch-flip
    v7 = 0;
    v4 = 0;
    do {
      v1 = v4 * 8;
      v2 = *(char **)(*(long *)(a1 + 0x38 + v6 * 8) + v4 * 8);
      if (*v2 == '.') {
        v3 = *(char **)&a0->field_0x28;
        if (*(char **)&a0->field_0x30 <= v3) { // branch-flip
          __overflow(a0,0x2e);
          v7 = 1;
        }
        else {
          v7 = 1;
          *(char **)&a0->field_0x28 = &v3[1];
          *v3 = 0x2e;
        }
      }
      v4 += 1;
      fwrite_unlocked(v2,1,*(unsigned long *)(*(long *)(a1 + 0x50 + v6 * 8) + v1),a0);
    } while (v4 <= *(long *)(v5 + 0x10) - *(long *)(v5 + 8));
  }
  else {
    v7 = 0;
  }
  return v7;
}


// Function: sub_46e0 @ 0x46e0
unsigned long sub_46e0(char *a0,char *a1,unsigned long *a2,unsigned long *a3)
{
  long v1;
  unsigned char *v10;
  unsigned long v11; // rax
  unsigned char v12;
  unsigned int v13;
  unsigned char *v14;
  long v15;
  unsigned char *v16;
  char *v17 [9]; // stack - 0x120
  int v18; // stack - 0x130
  stat v19; // stack - 0xd8
  unsigned int *v2;
  unsigned int v20; // stack - 0x138
  char *v21; // stack - 0x128
  char *v22; // rsi
  long v23;
  long v24;
  unsigned long v25;
  long *v26;
  unsigned int v27; // stack - 0x134
  int v28; // stack - 0x12c
  unsigned int v3;
  char *v4;
  int v5; // eax
  char **v6;
  char **v7;
  unsigned char *v8;
  long v9;
  
  v21 = dat_11010;
  v6 = v17;
  if (dat_110f2) {
    v17[0] = "-a";
    v6 = &v17[1];
  }
  v7 = v6;
  if (dat_110f1) {
    v7 = &v6[1];
    *v6 = "--strip-trailing-cr";
  }
  v7[2] = a0;
  *v7 = "--horizon-lines=100";
  v7[1] = "--";
  v7[3] = a1;
  v7[4] = NULL;
  if (pipe(&v18))
    sub_4330("pipe"); // no-return
  v5 = fork();
  if (!v5) {
    close(v18);
    if (v28 != 1) {
      dup2(v28,1);
      close(v28);
    }
    execvp(dat_11010,&v21);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  if (v5 == -1)
    sub_4330("fork"); // no-return
  close(v28);
  if (fstat(v18,&v19))
    sub_4330("fstat"); // no-return
  v25 = v19._56_8_;
  if ((long)v19._56_8_ <= 0)
    v25 = 1;
  v15 = 0;
  v8 = (unsigned char *)sub_7350(v25);
  while( true ) {
    v23 = v25 - v15;
    v9 = sub_7a10(v18,&v8[v15],v23);
    v15 += v9;
    if (v23 != v9) break;
    if (0x3ffffffffffffffe < v25) {
      sub_77d0(); // no-return, return-dupe
    }
    v25 *= 2;
    v8 = (unsigned char *)sub_73a0(v8,v25);
  }
  if (v9 == -1)
    sub_4330(dcgettext(NULL,"read failed",5)); // no-return
  if ((v15) && (v8[v15 + -1] != 10))
    sub_4300("invalid diff format; incomplete last line"); // no-return
  if (close(v18))
    sub_4330("close"); // no-return
  v2 = &v20;
  v5 = waitpid(v5,(int *)v2,0);
  v4 = dat_11010;
  if (v5 < 0)
    sub_4330("waitpid"); // no-return
  if (v20 & 0x7f) { // branch-flip
    v13 = 0x7fffffff;
    v22 = "subsidiary program \'%s\' failed";
  }
  else {
    if (!((int)v20 >> 8 & 0xfeU)) {
      v16 = &v8[v15];
      *a3 = v8;
      v13 = v20;
      v3 = v27;
      do {
        if (v16 <= v8) {
          v2[0] = 0;
          v2[1] = 0;
          *a2 = v26;
          return CONCAT44(v3,v13);
        }
        v20 = v13;
        v27 = v3;
        v26 = (long *)sub_7350(0x48);
        v4 = dat_21140;
        v26[5] = 0;
        v26[4] = 0;
        v26[7] = 0;
        v26[6] = 0;
        v12 = *v8;
        v10 = v8;
        if (v12 != 0x20) goto label_48f5;
        do {
          do {
            v12 = v10[1];
            v10 = &v10[1];
          } while (v12 == 0x20);
label_48f5:
          v13 = (unsigned int)v12;
        } while (v12 == 9);
        if (9 < v13 - 0x30) {
label_4999:
          __fprintf_chk(stderr,1,dcgettext(NULL,"%s: diff failed: ",5),v4);
          do {
            v12 = *v8;
            v16 = *(unsigned char **)&stderr->field_0x28;
            if (*(unsigned char **)&stderr->field_0x30 <= v16) // branch-flip
              __overflow(stderr,(unsigned int)v12);
            else {
              *(unsigned char **)&stderr->field_0x28 = &v16[1];
              *v16 = v12;
            }
            v16 = &v8[1];
            v12 = *v8;
            v8 = v16;
          } while (v12 != 10);
          exit(2); // no-return
        }
        v15 = 0;
        do {
          v10 = &v10[1];
          v15 = (long)(int)(v13 - 0x30) + v15 * 10;
          v13 = (unsigned int)*v10;
        } while (v13 - 0x30 <= 9);
        *v26 = v15;
        v12 = *v10;
        if (v12 != 0x20) goto label_494d;
        do {
          do {
            v12 = v10[1];
            v10 = &v10[1];
          } while (v12 == 0x20);
label_494d:
        } while (v12 == 9);
        if (v12 != 0x2c) // branch-flip
          v26[1] = v15;
        else {
          v14 = &v10[1];
          v13 = (unsigned int)v10[1];
          if (9 < (unsigned int)v10[1] - 0x30) goto label_4999;
          v9 = 0;
          v10 = v14;
          do {
            v10 = &v10[1];
            v9 = (long)(int)(v13 - 0x30) + v9 * 10;
            v13 = (unsigned int)*v10;
          } while (v13 - 0x30 <= 9);
          v26[1] = v9;
        }
        v12 = *v10;
        if ((v12 == 9) || (v12 == 0x20)) {
          do {
            do {
              v12 = v10[1];
              v10 = &v10[1];
            } while (v12 == 0x20);
          } while (v12 == 9);
        }
        if (v12 != 99) { // branch-flip
          if (v12 != 100) { // branch-flip
            if (v12 != 0x61) goto label_4999;
            v5 = 1;
          }
          else {
            v5 = 3;
          }
        }
        else {
          v5 = 2;
        }
        v14 = &v10[1];
        v12 = v10[1];
        if (v12 != 0x20) goto label_4a2c;
        do {
          do {
            v12 = v14[1];
            v14 = &v14[1];
          } while (v12 == 0x20);
label_4a2c:
          v13 = (unsigned int)v12;
        } while (v12 == 9);
        if (9 < v13 - 0x30) goto label_4999;
        v9 = 0;
        do {
          v14 = &v14[1];
          v9 = (long)(int)(v13 - 0x30) + v9 * 10;
          v13 = (unsigned int)*v14;
        } while (v13 - 0x30 <= 9);
        v26[2] = v9;
        v12 = *v14;
        if (v12 != 0x20) goto label_4a7c;
        do {
          do {
            v12 = v14[1];
            v14 = &v14[1];
          } while (v12 == 0x20);
label_4a7c:
        } while (v12 == 9);
        if (v12 != 0x2c) { // branch-flip
          v26[3] = v9;
          v12 = *v14;
        }
        else {
          v13 = (unsigned int)v14[1];
          if (10 <= (unsigned int)v14[1] - 0x30) goto label_4999;
          v14 = &v14[1];
          v23 = 0;
          do {
            v14 = &v14[1];
            v23 = (long)(int)(v13 - 0x30) + v23 * 10;
            v13 = (unsigned int)*v14;
          } while (v13 - 0x30 <= 9);
          v26[3] = v23;
          v12 = *v14;
          if ((v12 == 9) || (v12 == 0x20)) {
            do {
              do {
                v12 = v14[1];
                v14 = &v14[1];
              } while (v12 == 0x20);
            } while (v12 == 9);
          }
        }
        v8 = &v14[1];
        if (v12 != 10) {
          v8 = v14;
          goto label_4999;
        }
        if (v5 != 2) { // branch-flip
          if (v5 == 3) {
            v26[2] = v9 + 1;
            goto label_4ab7;
          }
          *v26 = v15 + 1;
          if (v5 != 1) goto label_4c78;
label_4b6d:
          v15 = v26[3];
          v9 = v26[2];
          v25 = (v15 - v9) + 1;
          if (0xffffffffffffffe < v25) {
            sub_77d0();
          }
          v26[5] = sub_7350(v25 * 8);
          v23 = sub_7350(v25 * 8);
          v26[7] = v23;
          if (v25) {
            v24 = 0;
            while( true ) {
              v8 = (unsigned char *)sub_4360(v8,(unsigned long *)(v24 * 8 + v26[5]),(long *)(v23 + v24 * 8),v16,0x3e);
              v1 = v24 + 1;
              if (v15 - v9 == v24) break;
              v23 = v26[7];
              v24 = v1;
            }
          }
        }
        else {
label_4c78:
          v15 = *v26;
label_4ab7:
          v9 = v26[1];
          v25 = (v9 - v15) + 1;
          if (0xffffffffffffffe < v25)
            sub_77d0();
          v26[4] = sub_7350(v25 * 8);
          v23 = sub_7350(v25 * 8);
          v26[6] = v23;
          if (v25) {
            v24 = 0;
            while( true ) {
              v8 = (unsigned char *)sub_4360(v8,(unsigned long *)(v24 * 8 + v26[4]),(long *)(v23 + v24 * 8),v16,0x3c);
              v1 = v24 + 1;
              if (v9 - v15 == v24) break;
              v23 = v26[6];
              v24 = v1;
            }
          }
          if (v5 == 2) {
            if (strncmp((char *)v8,"---\n",4))
              sub_4300("invalid diff format; invalid change separator"); // no-return
            v8 = &v8[4];
            goto label_4b6d;
          }
          if (v5 != 3) goto label_4b6d;
        }
        *(long **)v2 = v26;
        v2 = (unsigned int *)&v26[8];
        v13 = v20;
        v3 = v27;
      } while( true );
    }
    v13 = (int)v20 >> 8 & 0xff;
    if ((char)(v20 >> 8) != '~') { // branch-flip
      v22 = "subsidiary program \'%s\' failed (exit status %d)";
      if (v13 == 0x7f)
        v22 = "subsidiary program \'%s\' not found";
    }
    else {
      v22 = "subsidiary program \'%s\' could not be invoked";
    }
  }
  error(2,0,dcgettext(NULL,v22,5),v4,(unsigned long)v13);
  return v11;
}

