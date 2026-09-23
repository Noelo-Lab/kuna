// Function: sub_2900 @ 0x2900
unsigned long sub_2900(int a0,char **a1)
{
  char *v1;
  char *v10;
  char *v11;
  unsigned long v12;
  char *v13; // rax
  char *v14;
  unsigned long v15; // rax
  unsigned long v16; // rax
  long v17; // rax
  long v18; // rax
  long v19;
  char *v2;
  long v20;
  unsigned long v21; // rax
  unsigned long v22; // rax
  unsigned long v23; // rax
  unsigned long v24; // rax
  unsigned long v25; // rax
  unsigned long v26; // rcx
  long *v27;
  long *v28;
  char *v29; // stack - 0x1b0
  unsigned char v3;
  char v30 [24];
  char v31 [5];
  char v32 [32];
  long v33; // stack - 0x1a8
  char v34 [5];
  long v35; // stack - 0x118
  unsigned int v36; // ebp
  char *v37;
  long *v38;
  char *v39;
  unsigned char v4;
  char *v40; // stack - 0x210
  char *v41; // stack - 0x208
  int v42; // stack - 0x200
  unsigned int v43; // stack - 0x1ec
  long v44; // stack - 0x1e8
  bool v45; // stack - 0x1bd
  int v46; // stack - 0x1bc
  long v47; // stack - 0x1a0
  unsigned int v48; // stack - 0x190
  long v49; // stack - 0x180
  char v5;
  long v50; // stack - 0x110
  unsigned int v51; // stack - 0x100
  long v52; // stack - 0xf0
  void *v6;
  int v7;
  long v8;
  long *v9; // rax
  
  dat_d020 = 2;
  sub_4080(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_3e90(0);
  sub_6870();
  while( true ) {
    v25 = 0xca60;
    v7 = getopt_long(a0,a1,"bci:ln:sv",(void *)0xca60,NULL);
    v12 = dat_1d280;
    if (v7 == -1) break;
    switch(v7) {
      case 0x62:
      case 99:
        dat_d0b2 = '\x01';
        break;
      default:
        sub_3a80(0,0); // no-return
      case 0x69:
        sub_3ae0(0,&optarg,0x3a);
        v11 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v11;
          if (dat_d0c8 < dat_d0c0)
            dat_d0c8 = dat_d0c0;
        }
        else {
          optarg = v11;
          sub_3ae0(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_3c60(1);
        break;
      case 0x6e:
        v7 = sub_6940(optarg,0,0,&v29,"kKMGTPEZY0");
        if ((v7) || ((long)v29 < 0))
          sub_3a80("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_d010 < 0) || ((long)v29 <= (long)dat_d010))
          dat_d010 = v29;
        break;
      case 0x73:
        sub_3c60(3);
        break;
      case 0x76:
        v12 = sub_4520("David MacKenzie");
        sub_61b0(stdout,"cmp","GNU diffutils",dat_d018,sub_45b0("Torbjorn Granlund",0x962a),v12,0,v25);
        sub_3c90();
        return 0;
      case 0x80:
        v28 = (long *)0xca00;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5),v12);
        __printf_chk(1,"%s\n",dcgettext(NULL,"Compare two files byte by byte.",5));
        __printf_chk(1,"\n%s\n\n",dcgettext(NULL,"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).",5));
        v6 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v6);
        v11 = "-b, --print-bytes          print differing bytes";
        do {
          v28 = &v28[1];
          __printf_chk(1,"  %s\n",dcgettext(NULL,v11,5));
          v11 = (char *)*v28;
        } while (v11);
        v11 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
        v10 = dcgettext(NULL,"If a FILE is \'-\' or missing, read standard input.",5);
        __printf_chk(1,"\n%s\n\n%s\n%s\n",dcgettext(NULL,"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",5),v10,v11);
        sub_6290();
        sub_3c90();
        return 0;
      
    }
  }
  if (optind == a0)
    sub_3a80("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
  dat_d230 = a1[optind];
  dat_d238 = "-";
  v7 = optind + 1;
  if (optind + 1 < a0) {
    dat_d238 = a1[(long)optind + 1];
    v7 = optind + 2;
  }
  optind = v7;
  v8 = (long)optind;
  if (optind < a0) {
    optind += 1;
    v29 = a1[v8];
    sub_3ae0(0,&v29,0);
    v8 = (long)optind;
    if (optind < a0) {
      optind += 1;
      v29 = a1[v8];
      sub_3ae0(1,&v29,0);
      if (optind < a0)
        sub_3a80("extra operand \'%s\'",a1[optind]); // no-return
    }
  }
  v8 = 0;
  do {
    if (((v8) && (dat_d0c0 == dat_d0c8)) && (v7 = strcmp(dat_d230,dat_d238), !v7))
      return 0;
    v11 = *(char **)(v8 * 8 + 0xd230);
    if (strcmp(v11,"-")) { // branch-flip
      v7 = open(v11,0,0);
      *(int *)(v8 * 4 + 0xd220) = v7;
      if (v7 <= -1) {
        if (dat_d0b4 == 3)
          exit(2); // no-return
        error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230)); // return-dupe
        return v16;
      }
    }
    else {
      *(unsigned int *)(v8 * 4 + 0xd220) = 0;
      v7 = 0;
    }
    v46 = fstat(v7,(void *)(v8 * 0x90 + 0xd100));
    if (v46) {
      error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230));
      return v16;
    }
    if (v8 == 1) goto label_2da0;
    v8 = 1;
  } while( true );
label_2da0:
  if ((dat_d108 != dat_d198) || (dat_d100 != dat_d190)) {
    if ((dat_d118 & 0xf000) != 0x6000) {
      if ((dat_d118 & 0xf000) == 0x2000) {
        if ((dat_d1a8 & 0xf000) == 0x2000) goto label_3531;
        goto label_323a;
      }
      goto label_2ddb;
    }
    if ((dat_d1a8 & 0xf000) == 0x6000) {
label_3531:
      if (dat_d128 == dat_d1b8) goto label_3120;
      goto label_2ddb;
    }
label_323a:
    if (dat_d0b4 == 3) goto label_2e10;
label_3247:
    if ((fstat(1,&v33)) || (stat("/dev/null",&v35))) goto label_325e;
    if ((v47 != v50) || (v33 != v35)) {
      if ((v48 & 0xf000) != 0x6000) { // branch-flip
        if (((v48 & 0xf000) == 0x2000) && ((v51 & 0xf000) == 0x2000)) {
label_3649:
          if (v49 == v52) goto label_365f;
        }
      }
      else if ((v51 & 0xf000) == 0x6000) goto label_3649;
    }
    else {
label_365f:
      dat_d0b4 = 2;
    }
label_325e:
    if (dat_d0b4 != 3) goto label_2e10;
  }
  else {
label_3120:
    if (((dat_d118 == dat_d1a8) && (dat_d110 == dat_d1a0)) && ((dat_d11c == dat_d1ac && ((((dat_d120 == dat_d1b0 && (dat_d130 == dat_d1c0)) && (dat_d158 == dat_d1e8)) && (dat_d168 == dat_d1f8)))))) {
      v8 = sub_3b80(0);
      if (v8 == sub_3b80(1))
        return 0;
    }
label_2ddb:
    if (dat_d0b4 != 3) goto label_3247;
  }
  v8 = dat_d130;
  if (((dat_d118 & 0xf000) == 0x8000) && ((dat_d1a8 & 0xf000) == 0x8000)) {
    v17 = sub_3b80(0);
    v20 = dat_d1c0;
    v18 = sub_3b80(1);
    v19 = 0;
    if (0 <= v8 - v17)
      v19 = v8 - v17;
    v20 -= v18;
    if (v20 <= -1)
      v20 = 0;
    if (v20 != v19) {
      if (0 <= (long)dat_d010) {
        if (v19 < v20)
          v20 = v19;
        if ((long)dat_d010 <= v20) goto label_2e10;
      }
      exit(1); // no-return
    }
  }
label_2e10:
  dat_d0d0 = (char *)sub_7070(dat_d138,dat_d1c8,0x7ffffffffffffff7);
  v11 = &dat_d0d0[0xf];
  v9 = (long *)sub_63b0(((unsigned long)v11 >> 3) << 4);
  v8 = dat_d130;
  v28 = (long *)((long)v9 + ((unsigned long)v11 & 0xfffffffffffffff8));
  v41 = dat_d010;
  dat_d0e0 = v9;
  dat_d0e8 = v28;
  if (dat_d0b4 == 1) {
    v11 = (char *)0x7fffffffffffffff;
    if (0 <= (long)dat_d010)
      v11 = dat_d010;
    if ((dat_d118 & 0xf000) == 0x8000) {
      v10 = (char *)(v8 - sub_3b80(0));
      if ((long)v10 < (long)v11)
        v11 = v10;
    }
    v8 = dat_d1c0;
    if ((dat_d1a8 & 0xf000) == 0x8000) {
      v10 = (char *)(v8 - sub_3b80(1));
      if ((long)v10 < (long)v11)
        v11 = v10;
    }
    v43 = 1;
    while (v11 = (char *)((long)v11 / 10), v11) {
      v43 += 1;
    }
  }
  v8 = 0;
  do {
    v11 = *(char **)(v8 * 8 + 0xd0c0);
    if ((v11) && (sub_3b80(v8) == -1)) {
      do {
        v10 = dat_d0d0;
        if (v11 <= dat_d0d0)
          v10 = v11;
        v14 = (char *)sub_6fc0(*(unsigned int *)(v8 * 4 + 0xd220),v9,v10);
        if (v10 != v14) {
          if (v14 == (char *)0xffffffffffffffff) {
            error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230));
            return v15;
          }
          break;
        }
        v11 = &v11[-(long)v10];
      } while (v11);
    }
    if (v8 == 1) goto label_2e9d;
    v8 = 1;
  } while( true );
label_2e9d:
  v42 = 0;
  v40 = (char *)0x1;
  v44 = 1;
  v45 = 1;
label_2ebc:
  v11 = dat_d0d0;
  if (0 <= (long)v41) {
    if (v41 < dat_d0d0)
      v11 = v41;
    v41 = &v41[-(long)v11];
  }
  v10 = (char *)sub_6fc0(dat_d220,v9,v11);
  if (v10 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d230);
    return v23;
  }
  v11 = (char *)sub_6fc0(dat_d224,v28,v11);
  if (v11 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d238);
    return v24;
  }
  v14 = v11;
  if (v10 <= v11)
    v14 = v10;
  v7 = memcmp(v9,v28,(unsigned long)v14);
  if (v7) { // branch-flip
    if ((v10 < v11) || (*(unsigned char *)((long)v28 + (long)v10) = 0x55, v10 == v11))
      *(char *)((long)v9 + (long)v11) = 'y';
    *(unsigned char *)((long)v9 + (long)v10) = ~*(unsigned char *)((long)v28 + (long)v10);
    *(unsigned char *)((long)v28 + (long)v11) = ~*(unsigned char *)((long)v9 + (long)v11);
    v26 = (unsigned long)(~(unsigned int)v10 & 7);
    v37 = &((char *)((long)v9 + 1))[(long)v10];
    while (v26) {
      v2 = &v37[1];
      *v37 = 0;
      v26 -= 1;
      v37 = v2;
    }
    v26 = (unsigned long)(~(unsigned int)v11 & 7);
    v37 = &((char *)((long)v28 + 1))[(long)v11];
    while (v26) {
      v2 = &v37[1];
      *v37 = 0;
      v26 -= 1;
      v37 = v2;
    }
    v38 = v28;
    for (v27 = v9; *v27 == *v38; v27 = &v27[1]) {
      v38 = &v38[1];
    }
    v8 = 0;
    do {
      v20 = v8;
      v8 = v20 + 1;
    } while (*(char *)((long)v27 + v20) == *(char *)((long)v38 + v20));
    v39 = (char *)((long)v27 + (v20 - (long)v9));
    v1 = &v40[(long)v39];
    if ((v39) && (!dat_d0b4)) goto label_32fc;
    if (v14 <= v39) goto label_328b;
    if (dat_d0b4 != 2) {
      if (3 <= dat_d0b4) {
        if (dat_d0b4 != 3) goto label_328b;
        goto label_3380;
      }
      if (!dat_d0b4) goto label_3496;
      do {
        v3 = *(unsigned char *)((long)v9 + (long)v39);
        v4 = *(unsigned char *)((long)v28 + (long)v39);
        if (v3 != v4) {
          v12 = sub_3fe0(&v39[(long)v40],v30);
          if (dat_d0b2) { // branch-flip
            sub_3bd0(v31,v3);
            sub_3bd0(v32,v4);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",(unsigned long)v43,v12,(unsigned long)v3,v31,(unsigned long)v4,v32);
          }
          else {
            __printf_chk(1,"%*s %3o %3o\n",(unsigned long)v43,v12,v3,(unsigned long)v4);
          }
        }
        v39 = &v39[1];
      } while (v14 != v39);
      v42 = -1;
      v1 = &v14[(long)v40];
      goto label_328b;
    }
    if (v10 == v11) {
      v42 = 1;
      goto label_32a1;
    }
  }
  else {
    v1 = &v40[(long)v14];
    if ((!v14) || (dat_d0b4)) goto label_328b;
    v39 = v14;
label_32fc:
    v40 = v1;
    v1 = (char *)((long)v9 + (long)v39);
    v5 = *v1;
    v8 = 0;
    *v1 = '\n';
    v27 = v9;
    while (v13 = rawmemchr(v27,10), v1 != v13) {
      v8 += 1;
      v27 = (long *)&v13[1];
    }
    v44 += v8;
    *v1 = v5;
    v1 = v40;
    if (v39 < v14) {
label_3496:
      v40 = v1;
      v12 = sub_3fe0(v40,v32);
      v25 = sub_3fe0(v44,v30);
      if (dat_d0b2) { // branch-flip
        v3 = *(unsigned char *)((long)v9 + (long)v39);
        v4 = *(unsigned char *)((long)v28 + (long)v39);
        sub_3bd0(v34,v3);
        sub_3bd0(v31,(unsigned int)v4);
        v10 = dat_d238;
        v11 = dat_d230;
        __printf_chk(1,dcgettext(NULL,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5),v11,v10,v12,v25,(unsigned long)v3,v34,(unsigned int)v4,v31);
      }
      else {
        v11 = dcgettext(NULL,"%s %s differ: byte %s, line %s\n",5);
        if ((v11 == "%s %s differ: byte %s, line %s\n") && (!sub_3f50(5)))
          v11 = "%s %s differ: char %s, line %s\n";
        __printf_chk(1,v11,dat_d230,dat_d238,v12,v25);
      }
      goto label_3380;
    }
    v45 = ((char *)((long)v9 + -1))[(long)v39] == '\n';
label_328b:
    v40 = v1;
    if (v10 == v11) {
      if (v10 != dat_d0d0) {
label_32a1:
        v36 = (unsigned int)(v42 != 0);
label_32ac:
        v7 = close(dat_d220);
        if (!v7) {
          v7 = close(dat_d224);
          if (!v7) {
            if ((v36) && (dat_d0b4 < 2))
              sub_3c90();
            exit(v36); // no-return
          }
          v46 = 1;
        }
        error(2,*__errno_location(),"%s",*(unsigned long *)((long)v46 * 8 + 0xd230));
        return v21;
      }
      goto label_2ebc;
    }
    if (dat_d0b4 == 3) goto label_3380;
    v12 = *(unsigned long *)((unsigned long)(v11 < v10) * 8 + 0xd230);
    if (v40 == (char *)0x1) {
      v36 = 1;
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s which is empty\n",5),v12);
      goto label_32ac;
    }
    v25 = sub_3fe0(&v40[-1],v32);
    if (dat_d0b4) // branch-flip
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s after byte %s\n",5),v12,v25);
    else {
      v22 = sub_3fe0(v44 - (unsigned long)v45,v30);
      if (v45) // branch-flip
        v11 = dcgettext(NULL,"cmp: EOF on %s after byte %s, line %s\n",5);
      else {
        v11 = dcgettext(NULL,"cmp: EOF on %s after byte %s, in line %s\n",5);
      }
      __fprintf_chk(stderr,1,v11,v12,v25,v22);
    }
  }
label_3380:
  v36 = 1;
  goto label_32ac;
}


// Function: sub_3a80 @ 0x3a80
void sub_3a80(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1d280;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_3ae0 @ 0x3ae0
void sub_3ae0(int a0,unsigned long *a1,char a2) // return-dupe
{
  unsigned long v1;
  int v2; // eax
  long v3; // stack - 0x38
  
  v1 = *a1;
  v2 = sub_6940(v1,a1,0,&v3,"kKMGTPEZY0");
  if (((!v2) || ((v2 == 2 && (*(char *)*a1 == a2)))) && (0 <= v3)) {
    if (v3 <= *(long *)((long)a0 * 8 + 0xd0c0))
      return;
    *(long *)((long)a0 * 8 + 0xd0c0) = v3;
    return;
  }
  sub_3a80("invalid --ignore-initial value \'%s\'",v1); // no-return
}


// Function: sub_3b80 @ 0x3b80
unsigned long sub_3b80(int a0)
{
  unsigned long v1; // rax
  long v2; // rbx
  
  v2 = (long)a0;
  if (*(char *)(v2 + 0xd0b0))
    return *(unsigned long *)(v2 * 8 + 0xd0a0);
  *(char *)(v2 + 0xd0b0) = 1;
  v1 = lseek(*(unsigned int *)(v2 * 4 + 0xd220),*(unsigned long *)(v2 * 8 + 0xd0c0),1);
  *(unsigned long *)(v2 * 8 + 0xd0a0) = v1;
  return v1;
}


// Function: sub_3bd0 @ 0x3bd0
void sub_3bd0(unsigned char *a0,unsigned char a1)
{
  unsigned char *v1; // rbp
  
  if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)a1 * 2) & 0x40)) {
    if ((char)a1 < '\0') {
      a1 += 0x80;
      v1 = &a0[2];
      a0[0] = 0x4d;
      a0[1] = 0x2d;
      a0 = v1;
    }
    if (0x20 <= a1) { // branch-flip
      if (a1 == 0x7f) {
        *a0 = 0x5e;
        a1 = 0x3f;
        a0 = &a0[1];
      }
    }
    else {
      *a0 = 0x5e;
      a1 += 0x40;
      a0 = &a0[1];
    }
  }
  *a0 = a1;
  a0[1] = 0;
}

