// Function: sub_2a20 @ 0x2a20
unsigned long sub_2a20(int a0,char **a1)
{
  long *v1;
  char *v10;
  unsigned char *v11;
  int *v12;
  unsigned int *v13;
  unsigned char v14;
  char v15;
  int v16; // eax
  int v17;
  unsigned int v18;
  int v19;
  long *v2;
  unsigned int v20;
  long v21;
  long v22;
  unsigned long v23;
  long v24;
  long v25; // rax
  unsigned long v26;
  unsigned long v27; // rax
  char *v28;
  char *v29;
  char *v3;
  long v30;
  long v31;
  long *v32;
  long v33 [6]; // stack - 0x148
  char v34 [24];
  int v35 [5]; // stack - 0x160
  char v36 [8];
  void *v37; // stack - 0x170
  void *v38; // stack - 0x178
  int *v39; // stack - 0x168
  char v4;
  char v40; // stack - 0x42
  unsigned char v41;
  unsigned int v42;
  long v43;
  long v44;
  unsigned long v45; // rdi
  long v46;
  long v47;
  char **v48;
  int *v49;
  unsigned long v5;
  int *v50;
  long v51;
  bool v52; // cf
  char *v53; // stack - 0x1f0
  long *v54; // stack - 0x1e8
  int **v55; // stack - 0x1d0
  long v56; // stack - 0x1b0
  unsigned char v57; // stack - 0x18d
  int v58; // stack - 0x14c
  unsigned int v59; // stack - 0xc0
  long v6;
  char v60; // stack - 0x41
  long v61 [8]; // stack - 0x118
  long v7;
  void *v8;
  void *v9;
  
  v42 = 0;
  dat_11020 = 2;
  sub_50e0(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_5020(0);
  sub_7940();
  v17 = 0;
  while( true ) {
    v16 = getopt_long(a0,a1,"aeimvx3AEL:TX",(void *)0x10940,NULL);
    v26 = dat_21140;
    if (v16 == -1) break;
    switch(v16) {
      case 0x33:
        dat_110ed = '\x01';
        v42 |= 1;
        break;
      default:
        sub_42a0(0,0); // no-return
      case 0x41:
        dat_110eb = 1;
        v42 |= 2;
        dat_110ef = 1;
        break;
      case 0x45:
        dat_110ef = 1;
        v42 |= 4;
        break;
      case 0x4c:
        if (2 < v17)
          sub_42a0("too many file label options",0); // no-return
        v33[(long)v17 + 10] = optarg;
        v17 += 1;
        break;
      case 0x54:
        dat_110ee = '\x01';
        break;
      case 0x58:
        dat_110ec = '\x01';
        v42 |= 8;
        break;
      case 0x61:
        dat_110f2 = 1;
        break;
      case 0x65:
        v42 |= 0x10;
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
        v42 |= 0x20;
        break;
      case 0x80:
        dat_11010 = optarg;
        break;
      case 0x81:
        v21 = 0x108a0;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5),v26);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare three files line by line.",5));
        v8 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v8);
        v28 = "-A, --show-all              output all changes, bracketing conflicts";
        do {
          if (*v28) { // branch-flip
            __printf_chk(1,"  %s\n",dcgettext(NULL,v28,5));
            v28 = *(char **)(v21 + 8);
            v8 = stdout;
          }
          else {
            v10 = *(char **)((long)stdout + 0x28);
            if (*(char **)((long)stdout + 0x30) <= v10) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)((long)stdout + 0x28) = &v10[1];
              *v10 = 10;
            }
            v28 = *(char **)(v21 + 8);
            v8 = stdout;
          }
          v21 += 8;
          stdout = v8;
        } while (v28);
        fputs_unlocked(dcgettext(NULL,"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n",5),v8);
        v28 = dcgettext(NULL,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
        __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v28);
        sub_7230();
        sub_4690();
        return 0;
      case 0x82:
        dat_110f1 = 1;
      
    }
  }
  dat_110f0 = (dat_110e9 ^ 1) & v42 != 0;
  v14 = v42 == 0 & dat_110e9;
  dat_110eb |= v14;
  dat_110ef = v14 | dat_110ef;
  if (((v42 - 1 & v42) || (dat_110e9 & dat_110ea)) || ((v17 && (!dat_110ef))))
    sub_42a0("incompatible options",0); // no-return
  if (a0 - optind != 3) {
    if (a0 - optind > 2)
      sub_42a0("extra operand \'%s\'",a1[(long)optind + 3]); // no-return
    sub_42a0("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
  }
  a1 = &a1[optind];
  for (v21 = (long)v17; (int)v21 <= 2; v21 = v21 + 1) {
    v33[v21 + 10] = (long)a1[v21];
  }
  v42 = (unsigned int)(v42 != 0 | dat_110e9);
  v17 = 2 - v42;
  v56 = (long)v17;
  v28 = a1[v56];
  if (strcmp(v28,"-")) {
label_2ed8:
    v35[0] = 0;
    v35[3] = 0;
    v16 = 3 - v17;
    v21 = (long)v16;
    v35[v21 + 3] = 1;
    v35[v56 + 3] = 2;
    v48 = a1;
    v35[1] = v16;
    v35[2] = v17;
    do {
      v28 = *v48;
      if (strcmp(v28,"-")) {
        if (stat(v28,v34) < 0)
          sub_4330(*v48); // no-return
        if ((v59 & 0xf000) == 0x4000) {
          error(2,0x15,"%s",*v48);
          return v27;
        }
      }
      v48 = &v48[1];
    } while (&a1[3] != v48);
    signal(0x11,0);
    v49 = (int *)0xcd20;
    v28 = a1[v58];
    v22 = sub_46e0(a1[v35[4]],v28,v36,&v37);
    v29 = *a1;
    v33[5] = v22;
    v33[4] = sub_46e0(v29,v28,v36,&v38);
    v55 = &v39;
    v39 = NULL;
label_2ff0:
    v22 = v61[3];
    v46 = v61[2];
    v44 = v61[1];
    v51 = v61[0];
    v50 = v39;
    v41 = dat_110f0;
    v14 = dat_110e9;
    v8 = stdout;
    if (v33[4]) {
      v33[3] = 0;
      v33[2] = 0;
      v33[1] = 0;
      v33[0] = 0;
      if (v33[5]) { // branch-flip
        v43 = *(long *)(v33[5] + 0x10);
        v47 = *(long *)(v33[4] + 0x10);
        v45 = (unsigned long)(v43 < v47);
        v42 = (unsigned int)(v43 < v47);
        v23 = (unsigned long)(v47 <= v43);
        v47 = v45 << 3;
        v43 = v33[v45 + 4];
      }
      else {
        v42 = 0;
        v47 = 0;
        v23 = 1;
        v45 = 0;
        v43 = v33[4];
      }
label_306f:
      v33[3] = 0;
      v33[2] = 0;
      v33[1] = 0;
      v33[0] = 0;
      v24 = *(long *)(v43 + 0x40);
      v30 = *(long *)(v43 + 0x18);
      v33[v45 + 2] = v43;
      v33[v45] = v43;
      v33[v45 + 4] = v24;
      v24 = v33[v23 + 4];
      *(unsigned long *)(v43 + 0x40) = 0;
      if (v24) {
        do {
          if (v30 + 1 < *(long *)(v24 + 0x10)) break;
          if (v33[v23]) // branch-flip
            *(long *)(v33[v23 + 2] + 0x40) = v24;
          else {
            v33[v23] = v24;
          }
          v33[v23 + 2] = v24;
          v43 = *(long *)(v33[v23 + 4] + 0x40);
          *(unsigned long *)(v24 + 0x40) = 0;
          v24 = *(long *)(v24 + 0x18);
          v33[v23 + 4] = v43;
          v20 = v42;
          v18 = v42 ^ 1;
          if (v30 < v24) {
            v30 = v24;
            v20 = v42 ^ 1;
            v18 = v42;
          }
          v42 = v20;
          v23 = (unsigned long)(int)v18;
          v24 = v33[v23 + 4];
        } while (v24);
        v45 = (unsigned long)(int)v42;
      }
      v43 = *(long *)(*(long *)((long)v33 + v47) + 0x10);
      v47 = *(long *)(v33[v45 + 2] + 0x18);
      v24 = 0;
      do {
        v32 = *(long **)((long)v33 + v24);
        if (v32) { // branch-flip
          *(long *)((long)v33 + v24 + 0x30) = (v43 - v32[2]) + *v32;
          v30 = *(long *)(v24 + (long)&v33[2]);
          v30 = (v47 - *(long *)(v30 + 0x18)) + *(long *)(v30 + 8);
        }
        else {
          *(long *)((long)v33 + v24 + 0x30) = (v43 - *(long *)&v49[0xc]) + *(long *)((long)v49 + v24 * 2 + 0x10);
          v30 = (v47 - *(long *)&v49[0xc]) + *(long *)((long)v49 + v24 * 2 + 0x10);
        }
        *(long *)((long)v33 + v24 + 0x40) = v30;
        v24 += 8;
      } while (v24 != 0x10);
      v49 = (int *)sub_7350(0x70);
      *v49 = 0;
      v49[0x1a] = 0;
      v49[0x1b] = 0;
      *(long *)&v49[2] = v51;
      *(long *)&v49[6] = v44;
      *(long *)&v49[4] = v46;
      *(long *)&v49[8] = v22;
      *(long *)&v49[10] = v43;
      *(long *)&v49[0xc] = v47;
      v47 = (v46 - v51) + 1;
      if (v47) { // branch-flip
        *(unsigned long *)&v49[0xe] = sub_76b0(v47,8);
        v26 = sub_76b0(v47,8);
        v22 = *(long *)&v49[8];
        v44 = *(long *)&v49[6];
        *(unsigned long *)&v49[0x14] = v26;
      }
      else {
        v49[0xe] = 0;
        v49[0xf] = 0;
        v49[0x14] = 0;
        v49[0x15] = 0;
      }
      v22 = (v22 - v44) + 1;
      if (v22) { // branch-flip
        *(unsigned long *)&v49[0x10] = sub_76b0(v22,8);
        v26 = sub_76b0(v22,8);
      }
      else {
        v49[0x10] = 0;
        v49[0x11] = 0;
        v26 = 0;
      }
      *(unsigned long *)&v49[0x16] = v26;
      v22 = (*(long *)&v49[0xc] - *(long *)&v49[10]) + 1;
      if (v22) { // branch-flip
        *(unsigned long *)&v49[0x12] = sub_76b0(v22,8);
        v22 = sub_76b0(v22,8);
      }
      else {
        v49[0x12] = 0;
        v49[0x13] = 0;
        v22 = 0;
      }
      *(long *)&v49[0x18] = v22;
      v54 = v33;
      do {
        v44 = *v54;
        if (v44) {
          v47 = *(long *)&v49[0x12];
          do {
            v24 = *(long *)(v44 + 0x10);
            v30 = *(long *)(v44 + 0x18);
            v26 = *(unsigned long *)(v44 + 0x38);
            v5 = *(unsigned long *)(v44 + 0x28);
            v31 = (v24 - v43) * 8;
            if (!sub_4460(v5,v26,v31 + v47,v22 + v31,(v30 - v24) + 1)) {
              sub_4300("internal error: screwup in format of diff blocks"); // return-dupe, no-return
            }
            v44 = *(long *)(v44 + 0x40);
          } while (v44);
        }
        v54 = &v54[1];
      } while (&v33[2] != v54);
      v44 = 0;
      do {
        v32 = *(long **)((long)v33 + v44);
        v24 = 0;
        v47 = v51;
        while( true ) {
          v30 = v46 + 1;
          if (v32)
            v30 = *v32;
          if (v30 <= v47) break;
          v47 += 1;
          *(unsigned long *)(*(long *)((long)v49 + v44 + 0x38) + v24) = *(unsigned long *)(*(long *)&v49[0x12] + v24);
          *(unsigned long *)(*(long *)((long)v49 + v44 + 0x50) + v24) = *(unsigned long *)(v22 + v24);
          v24 += 8;
        }
        if (v32) {
          v47 = *(long *)((long)v49 + v44 + 0x38);
          v24 = *(long *)((long)v49 + v44 + 0x50);
label_33b0:
          v30 = *v32;
          v31 = v32[1];
          v6 = v32[6];
          v7 = v32[4];
          v25 = (v30 - v51) * 8;
          if (sub_4460(v7,v6,v47 + v25,v24 + v25,(v31 - v30) + 1)) {
            v1 = &v32[1];
            v2 = &v32[3];
            v32 = (long *)v32[8];
            v30 = (*v2 - v43) * 8;
            v31 = (*v1 + 1) - v51;
            do {
              v30 += 8;
              if (v32) { // branch-flip
                if (*v32 - v51 <= v31) goto label_33b0;
              }
              else if ((v46 + 1) - v51 <= v31) goto label_3447;
              *(unsigned long *)(v47 + v31 * 8) = *(unsigned long *)(*(long *)&v49[0x12] + v30);
              *(unsigned long *)(v24 + v31 * 8) = *(unsigned long *)(v22 + v30);
              v31 += 1;
            } while( true );
          }
          sub_4300("internal error: screwup in format of diff blocks");
        }
label_3447:
        if (v44 + 8 == 0x10) goto label_3455;
        v51 = *(long *)((long)v33 + v44 + 0x38);
        v46 = *(long *)((long)v33 + v44 + 0x48);
        v44 += 8;
      } while( true );
    }
    if (v33[5]) {
      v33[3] = 0;
      v23 = 0;
      v47 = 8;
      v42 = 1;
      v33[2] = 0;
      v45 = 1;
      v33[1] = 0;
      v33[0] = 0;
      v43 = v33[5];
      goto label_306f;
    }
    if (dat_110f0) {
      v49 = NULL;
      while (v50) {
        v12 = *(int **)&v50[0x1a];
        *(int **)&v50[0x1a] = v49;
        v49 = v50;
        v50 = v12;
      }
      v57 = 0;
      do {
        if (!v49) {
          if (dat_110ea)
            fputs_unlocked("w\nq\n",v8);
          goto label_3918;
        }
        if (*v49 != 4) { // branch-flip
          v19 = v35[(unsigned long)(unsigned int)(*v49 - 5) + 3];
          if (v19 != 1) { // branch-flip
            if (v19 != 2) { // branch-flip
              if (v19 == -1) goto label_3da6;
            }
            else if (!dat_110ec) {
              v22 = *(long *)&v49[2];
              v44 = *(long *)&v49[4];
              goto label_3f3d;
            }
          }
          else if (dat_110eb) {
            v44 = *(long *)&v49[4];
            v22 = *(long *)&v49[2];
            __fprintf_chk(v8,1,"%lda\n",v44);
            __fprintf_chk(v8,1,">>>>>>> %s\n",v61[6]);
            sub_45f0(v8,0,v44 + 2,(*(long *)&v49[v21 * 4 + 4] - *(long *)&v49[v21 * 4 + 2]) + 3 + (*(long *)&v49[v56 * 4 + 4] - *(long *)&v49[v56 * 4 + 2]));
            __fprintf_chk(v8,1,"%lda\n<<<<<<< %s\n",v22 + -1,v61[5]);
            v15 = sub_4500(v8,v49,v16);
            fputs_unlocked("=======\n",v8);
            goto label_4072;
          }
        }
        else {
label_3da6:
          if (!dat_110ed) {
            v22 = *(long *)&v49[2];
            v44 = *(long *)&v49[4];
            if (dat_110ef) { // branch-flip
              __fprintf_chk(v8,1,"%lda\n",v44);
              v42 = (unsigned int)dat_110eb;
              if (dat_110eb) {
                __fprintf_chk(v8,1,"||||||| %s\n",v61[5]);
                v42 = sub_4500(v8,v49,v16);
              }
              fputs_unlocked("=======\n",v8);
              v20 = sub_4500(v8,v49,v17);
              __fprintf_chk(v8,1,">>>>>>> %s\n",v61[6]);
              sub_45f0(v8,(v42 | v20) & 0xff,v44 + 2,(*(long *)&v49[v21 * 4 + 4] - *(long *)&v49[v21 * 4 + 2]) + 3 + (*(long *)&v49[v56 * 4 + 4] - *(long *)&v49[v56 * 4 + 2]));
              __fprintf_chk(v8,1,"%lda\n<<<<<<< %s\n",v22 + -1,v61[4]);
              v15 = 0;
label_4072:
              sub_45f0(v8,v15,v22 + 1,(*(long *)&v49[v21 * 4 + 4] - *(long *)&v49[v21 * 4 + 2]) + 1);
              v57 = v41;
            }
            else {
label_3f3d:
              if (*(long *)&v49[v56 * 4 + 4] - *(long *)&v49[v56 * 4 + 2] != -1) { // branch-flip
                if (v44 - v22 != -1) { // branch-flip
                  if (v44 != v22) // branch-flip
                    __fprintf_chk(v8,1,"%ld,%ldc\n",v22,v44);
                  else {
                    __fprintf_chk(v8,1,"%ldc\n",v44);
                  }
                }
                else {
                  __fprintf_chk(v8,1,"%lda\n",v44);
                }
                v44 = *(long *)&v49[v56 * 4 + 4];
                v51 = *(long *)&v49[v56 * 4 + 2];
                sub_45f0(v8,sub_4500(v8,v49,v17),v22,(v44 - v51) + 1);
              }
              else if (v22 != v44) // branch-flip
                __fprintf_chk(v8,1,"%ld,%ldd\n",v22,v44);
              else {
                __fprintf_chk(v8,1,"%ldd\n",v22);
              }
            }
          }
        }
        v49 = *(int **)&v49[0x1a];
      } while( true );
    }
    if (!dat_110e9) {
      v28 = "  ";
      if (dat_110ee)
        v28 = "\t";
      for (v49 = v39; v49; v49 = *(int **)&v49[0x1a]) {
        if (*v49 != 4) { // branch-flip
          v42 = *v49 - 5;
          if (3 <= v42)
            sub_4300("internal error: invalid diff type passed to output"); // no-return
          v53._0_4_ = v35[(unsigned long)v42 + 3];
          v60 = 0;
          v40 = (char)(int)v53 + '1';
          v54._0_4_ = (unsigned int)((int)v53 == 0);
        }
        else {
          v40 = '\0';
          v54._0_4_ = 3;
          v53._0_4_ = 3;
        }
        __fprintf_chk(v8,1,"====%s\n",&v40);
        v42 = 0;
        do {
          v44 = (long)v35[(int)v42];
          v21 = *(long *)&v49[v44 * 4 + 2];
          v22 = *(long *)&v49[v44 * 4 + 4];
          v20 = v42 + 1;
          __fprintf_chk(v8,1,"%d:",(unsigned long)v20);
          if (v21 != v22) { // branch-flip
            if (v21 - v22 != 1) // branch-flip
              __fprintf_chk(v8,1,"%ld,%ldc\n",v21,v22);
            else {
              __fprintf_chk(v8,1,"%lda\n",v21 + -1);
            }
          }
          else {
            __fprintf_chk(v8,1,"%ldc\n",v21);
          }
          if ((v21 <= v22) && ((unsigned int)v54 != v42)) {
            v51 = 0;
            do {
              fputs_unlocked(v28,v8);
              v9 = *(void **)(*(long *)&v49[(v44 + 6) * 2 + 2] + v51 * 8);
              v23 = *(unsigned long *)(*(long *)&v49[v44 * 2 + 0x14] + v51 * 8);
              v51 += 1;
              fwrite_unlocked(v9,1,v23,v8);
            } while (v51 <= v22 - v21);
            if (*(char *)((long)v9 + (v23 - 1)) != '\n')
              __fprintf_chk(v8,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5));
          }
          if ((int)v53 == 1)
            v20 = *(unsigned int *)((long)(int)v42 * 4 + 0xcd00);
          v42 = v20;
        } while ((int)v20 <= 2);
      }
      v57 = 0;
      goto label_3918;
    }
    sub_7810(*a1,"r",stdin);
    v13 = stdin;
    v8 = stdout;
    v57 = 0;
    v22 = 0;
    goto label_36fa;
  }
  v28 = *a1;
  v17 = v42 + 1;
  if (strcmp(v28,"-")) {
    v56 = (long)v17;
    v28 = a1[v56];
    if (strcmp(v28,"-")) goto label_2ed8;
  }
  sub_4300("\'-\' specified for more than one input file"); // no-return
label_3455:
  v19 = 6;
  if (v33[0]) {
    if (v33[1]) { // branch-flip
      v22 = *(long *)&v49[4];
      v19 = 4;
      v44 = *(long *)&v49[2];
      if (v22 - v44 == *(long *)&v49[8] - *(long *)&v49[6]) {
        v24 = 0;
        v51 = *(long *)&v49[0x16];
        v46 = *(long *)&v49[0x10];
        v43 = *(long *)&v49[0x14];
        v47 = *(long *)&v49[0xe];
        do {
          if ((v22 - v44) + 1 == v24) {
            v19 = 7;
            goto label_3492;
          }
          v8 = *(void **)(v47 + v24 * 8);
          if (((!v8) || (v9 = *(void **)(v46 + v24 * 8), !v9)) || (v23 = *(unsigned long *)(v43 + v24 * 8), v23 != *(unsigned long *)(v51 + v24 * 8))) break;
          v24 += 1;
        } while (!memcmp(v8,v9,v23));
        v19 = 4;
      }
    }
    else {
      v19 = 5;
    }
  }
label_3492:
  *v49 = v19;
  *v55 = v49;
  v55 = (int **)&v49[0x1a];
  goto label_2ff0;
label_36fa:
  if (!v50) {
    do {
      v11 = *(unsigned char **)&v13[2];
      if (*(unsigned char **)&v13[4] <= v11) { // branch-flip
        v42 = __uflow(v13);
        if ((v42 == 0xffffffff) && (*v13 & 0x30)) {
label_3908:
          if (*stdin & 0x20)
            sub_4300("read failed"); // no-return
label_3918:
          free(v38);
          free(v37);
          sub_4690();
          exit((unsigned int)v57); // no-return
        }
      }
      else {
        *(unsigned char **)&v13[2] = &v11[1];
        v42 = (unsigned int)*v11;
      }
      v10 = *(char **)((long)v8 + 0x28);
      if (*(char **)((long)v8 + 0x30) <= v10) // branch-flip
        __overflow(v8,v42 & 0xff);
      else {
        *(char **)((long)v8 + 0x28) = &v10[1];
        *v10 = (char)v42;
      }
    } while( true );
  }
  if (*v50 != 4) { // branch-flip
    v17 = v35[(unsigned long)(unsigned int)(*v50 - 5) + 3];
    if (v17 != 1) { // branch-flip
      if (v17 != 2) { // branch-flip
        if (v17 == -1) goto label_3736;
      }
      else if (!dat_110ec) {
        v17 = 7;
        v54 = (long)(*(long *)&v50[2] + -1);
        v53 = "<<<<<<< %s\n";
        v22 = (*(long *)&v50[2] - v22) + -2;
        v41 = 0;
        if (0 <= v22) goto label_37a8;
        goto label_382f;
      }
    }
    else if (dat_110eb) {
      v17 = 6;
      v54 = (long)(*(long *)&v50[2] + -1);
      v22 = (*(long *)&v50[2] - v22) + -2;
      v53 = "<<<<<<< %s\n";
      v41 = dat_110eb;
      if (0 <= v22) goto label_37a8;
label_39e2:
      __fprintf_chk(v8,1,v53,v61[5]);
      for (v22 = 0; v22 <= *(long *)&v50[v21 * 4 + 4] - *(long *)&v50[v21 * 4 + 2]; v22 = v22 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v50[v21 * 2 + 0xe] + v22 * 8),1,*(unsigned long *)(*(long *)&v50[v21 * 2 + 0x14] + v22 * 8),v8);
      }
      fputs_unlocked("=======\n",v8); // crossjump-dupe
      v57 = v14;
      v41 = v14;
      goto label_382f;
    }
  }
  else {
label_3736:
    if (!dat_110ed) {
      v54 = (long)(*(long *)&v50[2] + -1);
      v22 = (*(long *)&v50[2] - v22) + -2;
      if (0 <= v22) { // branch-flip
        v17 = 4;
        v53 = "||||||| %s\n";
        v41 = dat_110ef;
label_37a8:
        do {
          v11 = *(unsigned char **)&v13[2];
          if (*(unsigned char **)&v13[4] <= v11) { // branch-flip
            v42 = __uflow(v13);
            if (v42 == 0xffffffff) {
              if (*v13 & 0x20)
                sub_4330(dcgettext(NULL,"read failed",5)); // return-dupe, no-return
              if (*v13 & 0x10)
                sub_4300("input file shrank"); // return-dupe, no-return
            }
          }
          else {
            *(unsigned char **)&v13[2] = &v11[1];
            v42 = (unsigned int)*v11;
          }
          v10 = *(char **)((long)v8 + 0x28);
          if (*(char **)((long)v8 + 0x30) <= v10) // branch-flip
            __overflow(v8,v42 & 0xff);
          else {
            *(char **)((long)v8 + 0x28) = &v10[1];
            *v10 = (char)v42;
          }
        } while ((v42 != 10) || (v52 = v22 != 0, v22 = v22 + -1, v52));
        v52 = v41 != 0;
        v41 = 0;
        if (v52) {
          if (v17 == 4) goto label_3a91;
label_3808:
          if (dat_110eb) goto label_39e2;
          fputs_unlocked("=======\n",v8);
          v57 = v14;
          v41 = v14;
        }
      }
      else {
        v41 = 0;
        if (dat_110ef) {
          v53 = "||||||| %s\n";
label_3a91:
          __fprintf_chk(v8,1,"<<<<<<< %s\n",v61[4]);
          for (v22 = 0; v22 <= *(long *)&v50[4] - *(long *)&v50[2]; v22 = v22 + 1) {
            fwrite_unlocked(*(void **)(*(long *)&v50[0xe] + v22 * 8),1,*(unsigned long *)(*(long *)&v50[0x14] + v22 * 8),v8);
          }
          goto label_3808;
        }
      }
label_382f:
      for (v22 = 0; v22 <= *(long *)&v50[v56 * 4 + 4] - *(long *)&v50[v56 * 4 + 2]; v22 = v22 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v50[v56 * 2 + 0xe] + v22 * 8),1,*(unsigned long *)(*(long *)&v50[v56 * 2 + 0x14] + v22 * 8),v8);
      }
      if (v41)
        __fprintf_chk(v8,1,">>>>>>> %s\n",v61[6]);
      v22 = (*(long *)&v50[4] - *(long *)&v50[2]) + 1;
      v44 = v22;
label_389a:
      v44 -= 1;
      if (0 <= v44) {
        v28 = *(char **)&v13[2];
        v29 = *(char **)&v13[4];
        do {
          while (v28 >= v29) {
            v17 = __uflow(v13);
            if (v17 == 10) goto label_389a;
            if (v17 == -1) {
              if (*v13 & 0x20) {
                sub_4330(dcgettext(NULL,"read failed",5));
              }
              if (*v13 & 0x10) {
                if ((v44) || (*(long *)&v50[0x1a])) {
                  sub_4300("input file shrank");
                }
                goto label_3908;
              }
            }
            v29 = *(char **)&v13[4];
            v28 = *(char **)&v13[2];
          }
          v3 = &v28[1];
          *(char **)&v13[2] = v3;
          v4 = *v28;
          v28 = v3;
        } while (v4 != '\n');
        goto label_389a;
      }
      v22 = (long)v54 + v22;
    }
  }
  v50 = *(int **)&v50[0x1a];
  goto label_36fa;
}


// Function: sub_42a0 @ 0x42a0
void sub_42a0(char *a0,unsigned long a1)
{
  unsigned long v1;
  
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
void sub_4330(unsigned long a0)
{
  error(2,*__errno_location(),"%s",a0);
}


// Function: sub_4360 @ 0x4360
unsigned char * sub_4360(char *a0,unsigned long *a1,long *a2,unsigned char *a3,char a4)
{
  unsigned char *v1;
  unsigned char v2;
  char v3;
  unsigned char *v4;
  unsigned char *v5;
  
  if ((*a0 != a4) || (a0[1] != ' '))
    sub_4300("invalid diff format; incorrect leading line chars"); // no-return
  v5 = (unsigned char *)&a0[2];
  *a1 = (unsigned long)v5;
  v4 = v5;
  while (v1 = &v4[1], *v4 != 10) {
    v4 = v1;
  }
  *a2 = (long)v1 - (long)v5;
  if ((v1 < a3) && (*v1 == 0x5c)) {
    if (dat_110f0) // branch-flip
      __fprintf_chk(stderr,1,"%s:",dat_21140);
    else {
      *a2 = ((long)v1 - (long)v5) + -1;
    }
    v4 = &v4[2];
    v3 = dat_110f0;
    while( true ) {
      if (v3) {
        v2 = *v4;
        v5 = *(unsigned char **)((long)stderr + 0x28);
        if (*(unsigned char **)((long)stderr + 0x30) <= v5) // branch-flip
          __overflow(stderr,(unsigned int)v2);
        else {
          *(unsigned char **)((long)stderr + 0x28) = &v5[1];
          *v5 = v2;
        }
      }
      if (*v4 == 10) break;
      v4 = &v4[1];
      v3 = dat_110f0;
    }
    return &v4[1];
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
char sub_4500(void *a0,long a1,int a2)
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
        v3 = *(char **)((long)a0 + 0x28);
        if (*(char **)((long)a0 + 0x30) <= v3) { // branch-flip
          __overflow(a0,0x2e);
          v7 = 1;
        }
        else {
          v7 = 1;
          *(char **)((long)a0 + 0x28) = &v3[1];
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
  char v19 [56];
  unsigned int *v2;
  unsigned int v20; // stack - 0x138
  unsigned long v21; // stack - 0x128
  char *v22; // rsi
  long v23;
  long v24;
  unsigned long v25;
  long *v26;
  unsigned int v27; // stack - 0x134
  int v28; // stack - 0x12c
  unsigned long v29; // stack - 0xa0
  unsigned int v3;
  unsigned long v4;
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
  if (fstat(v18,v19))
    sub_4330("fstat"); // no-return
  v25 = v29;
  if ((long)v29 <= 0)
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
            v16 = *(unsigned char **)((long)stderr + 0x28);
            if (*(unsigned char **)((long)stderr + 0x30) <= v16) // branch-flip
              __overflow(stderr,(unsigned int)v12);
            else {
              *(unsigned char **)((long)stderr + 0x28) = &v16[1];
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
              v8 = (unsigned char *)sub_4360(v8,v24 * 8 + v26[5],v23 + v24 * 8,v16,0x3e);
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
              v8 = (unsigned char *)sub_4360(v8,v24 * 8 + v26[4],v23 + v24 * 8,v16,0x3c);
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

