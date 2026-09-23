// Function: main @ 0x2900
int main(int argc,char **argv,char **envp)
{
  char *v1;
  int v10; // eax
  int v11; // eax
  long v12;
  long *v13; // rax
  char *v14;
  char *v15;
  char *v16;
  char *v17; // rax
  long v18; // rax
  long v19; // rax
  unsigned char v2;
  long v20;
  long v21;
  unsigned long v22;
  char *v23;
  unsigned long v24; // rcx
  unsigned long v25; // rcx
  long *v26;
  long *v27;
  char *v28; // stack - 0x1b0
  char v29 [24];
  unsigned char v3;
  char v30 [5];
  char v31 [32];
  stat v32; // stack - 0x1a8
  char v33 [5];
  stat v34; // stack - 0x118
  char *v35;
  long *v36;
  unsigned int v37; // ebp
  char *v38;
  char *v39; // stack - 0x210
  char v4;
  char *v40; // stack - 0x208
  int v41; // stack - 0x200
  int v42; // stack - 0x1ec
  unsigned long v43; // stack - 0x1e8
  bool v44; // stack - 0x1bd
  int v45; // stack - 0x1bc
  FILE *v5;
  int v6;
  int v7; // eax
  int v8; // eax
  int v9; // eax
  
  dat_d020 = 2;
  sub_4080(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_3e90(NULL);
  sub_6870();
  while( true ) {
    v24 = 0xca60;
    v6 = getopt_long(argc,argv,"bci:ln:sv",(option *)0xca60,NULL);
    v22 = dat_1d280;
    if (v6 == -1) break;
    switch(v6) {
      case 0x62:
      case 99:
        dat_d0b2 = '\x01';
        break;
      default:
        sub_3a80(NULL,0); // no-return
      case 0x69:
        sub_3ae0(0,&optarg,0x3a);
        v15 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v15;
          if (dat_d0c8 < dat_d0c0)
            dat_d0c8 = dat_d0c0;
        }
        else {
          optarg = v15;
          sub_3ae0(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_3c60(1);
        break;
      case 0x6e:
        v6 = sub_6940(optarg,NULL,0,&v28,"kKMGTPEZY0");
        if ((v6) || ((long)v28 < 0))
          sub_3a80("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_d010 < 0) || ((long)v28 <= (long)dat_d010))
          dat_d010 = v28;
        break;
      case 0x73:
        sub_3c60(3);
        break;
      case 0x76:
        v22 = sub_4520("David MacKenzie");
        sub_61b0(stdout,"cmp","GNU diffutils",dat_d018,sub_45b0("Torbjorn Granlund",0x962a),v22,0,v24);
        sub_3c90();
        return 0;
      case 0x80:
        v27 = (long *)0xca00;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5),v22);
        __printf_chk(1,"%s\n",dcgettext(NULL,"Compare two files byte by byte.",5));
        __printf_chk(1,"\n%s\n\n",dcgettext(NULL,"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).",5));
        v5 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v5);
        v15 = "-b, --print-bytes          print differing bytes";
        do {
          v27 = &v27[1];
          __printf_chk(1,"  %s\n",dcgettext(NULL,v15,5));
          v15 = (char *)*v27;
        } while (v15);
        v15 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
        v14 = dcgettext(NULL,"If a FILE is \'-\' or missing, read standard input.",5);
        __printf_chk(1,"\n%s\n\n%s\n%s\n",dcgettext(NULL,"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",5),v14,v15);
        sub_6290();
        sub_3c90();
        return 0;
      
    }
  }
  if (optind == argc)
    sub_3a80("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  dat_d230 = argv[optind];
  dat_d238 = "-";
  v6 = optind + 1;
  if (optind + 1 < argc) {
    dat_d238 = argv[(long)optind + 1];
    v6 = optind + 2;
  }
  optind = v6;
  v12 = (long)optind;
  if (optind < argc) {
    optind += 1;
    v28 = argv[v12];
    sub_3ae0(0,&v28,0);
    v12 = (long)optind;
    if (optind < argc) {
      optind += 1;
      v28 = argv[v12];
      sub_3ae0(1,&v28,0);
      if (optind < argc)
        sub_3a80("extra operand \'%s\'",argv[optind]); // no-return
    }
  }
  v12 = 0;
  do {
    if (((v12) && (dat_d0c0 == dat_d0c8)) && (!strcmp(dat_d230,dat_d238)))
      return 0;
    v15 = *(char **)(v12 * 8 + 0xd230);
    if (strcmp(v15,"-")) { // branch-flip
      v6 = open(v15,0,0);
      *(int *)(v12 * 4 + 0xd220) = v6;
      if (v6 <= -1) {
        if (dat_d0b4 == 3)
          exit(2); // no-return
        error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230)); // return-dupe
        return v8;
      }
    }
    else {
      *(unsigned int *)(v12 * 4 + 0xd220) = 0;
      v6 = 0;
    }
    v45 = fstat(v6,(stat *)(v12 * 0x90 + 0xd100));
    if (v45) {
      error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230));
      return v8;
    }
    if (v12 == 1) goto label_2da0;
    v12 = 1;
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
    if ((fstat(1,&v32)) || (stat("/dev/null",&v34))) goto label_325e;
    if ((v32._8_8_ != v34._8_8_) || (v32._0_8_ != v34._0_8_)) {
      if ((v32._24_4_ & 0xf000) != 0x6000) { // branch-flip
        if (((v32._24_4_ & 0xf000) == 0x2000) && ((v34._24_4_ & 0xf000) == 0x2000)) {
label_3649:
          if (v32._40_8_ == v34._40_8_) goto label_365f;
        }
      }
      else if ((v34._24_4_ & 0xf000) == 0x6000) goto label_3649;
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
      v12 = sub_3b80(0);
      if (v12 == sub_3b80(1))
        return 0;
    }
label_2ddb:
    if (dat_d0b4 != 3) goto label_3247;
  }
  v12 = dat_d130;
  if (((dat_d118 & 0xf000) == 0x8000) && ((dat_d1a8 & 0xf000) == 0x8000)) {
    v18 = sub_3b80(0);
    v21 = dat_d1c0;
    v19 = sub_3b80(1);
    v20 = 0;
    if (0 <= v12 - v18)
      v20 = v12 - v18;
    v21 -= v19;
    if (v21 <= -1)
      v21 = 0;
    if (v21 != v20) {
      if (0 <= (long)dat_d010) {
        if (v20 < v21)
          v21 = v20;
        if ((long)dat_d010 <= v21) goto label_2e10;
      }
      exit(1); // no-return
    }
  }
label_2e10:
  dat_d0d0 = (char *)sub_7070(dat_d138,dat_d1c8,(void *)0x7ffffffffffffff7);
  v15 = &dat_d0d0[0xf];
  v13 = (long *)sub_63b0(((unsigned long)v15 >> 3) << 4);
  v12 = dat_d130;
  v27 = (long *)((long)v13 + ((unsigned long)v15 & 0xfffffffffffffff8));
  v40 = dat_d010;
  dat_d0e0 = v13;
  dat_d0e8 = v27;
  if (dat_d0b4 == 1) {
    v15 = (char *)0x7fffffffffffffff;
    if (0 <= (long)dat_d010)
      v15 = dat_d010;
    if ((dat_d118 & 0xf000) == 0x8000) {
      v14 = (char *)(v12 - sub_3b80(0));
      if ((long)v14 < (long)v15)
        v15 = v14;
    }
    v12 = dat_d1c0;
    if ((dat_d1a8 & 0xf000) == 0x8000) {
      v14 = (char *)(v12 - sub_3b80(1));
      if ((long)v14 < (long)v15)
        v15 = v14;
    }
    v42 = 1;
    while (v15 = (char *)((long)v15 / 10), v15) {
      v42 += 1;
    }
  }
  v12 = 0;
  do {
    v15 = *(char **)(v12 * 8 + 0xd0c0);
    if ((v15) && (sub_3b80(v12) == -1)) {
      do {
        v14 = dat_d0d0;
        if (v15 <= dat_d0d0)
          v14 = v15;
        v23 = (char *)sub_6fc0(*(unsigned int *)(v12 * 4 + 0xd220),v13,v14);
        if (v14 != v23) {
          if (v23 == (char *)0xffffffffffffffff) {
            error(2,*__errno_location(),"%s",*(char **)((long)(int)v12 * 8 + 0xd230));
            return v7;
          }
          break;
        }
        v15 = &v15[-(long)v14];
      } while (v15);
    }
    if (v12 == 1) goto label_2e9d;
    v12 = 1;
  } while( true );
label_2e9d:
  v41 = 0;
  v39 = (char *)0x1;
  v43 = 1;
  v44 = 1;
label_2ebc:
  v15 = dat_d0d0;
  if (0 <= (long)v40) {
    if (v40 < dat_d0d0)
      v15 = v40;
    v40 = &v40[-(long)v15];
  }
  v14 = (char *)sub_6fc0(dat_d220,v13,v15);
  if (v14 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d230);
    return v10;
  }
  v15 = (char *)sub_6fc0(dat_d224,v27,v15);
  if (v15 == (char *)0xffffffffffffffff) {
    error(2,*__errno_location(),"%s",dat_d238);
    return v11;
  }
  v23 = v15;
  if (v14 <= v15)
    v23 = v14;
  v6 = memcmp(v13,v27,(unsigned long)v23);
  if (v6) { // branch-flip
    if ((v14 < v15) || (*(unsigned char *)((long)v27 + (long)v14) = 0x55, v14 == v15))
      *(char *)((long)v13 + (long)v15) = 'y';
    *(unsigned char *)((long)v13 + (long)v14) = ~*(unsigned char *)((long)v27 + (long)v14);
    *(unsigned char *)((long)v27 + (long)v15) = ~*(unsigned char *)((long)v13 + (long)v15);
    v25 = (unsigned long)(~(unsigned int)v14 & 7);
    v35 = &((char *)((long)v13 + 1))[(long)v14];
    while (v25) {
      v1 = &v35[1];
      *v35 = 0;
      v25 -= 1;
      v35 = v1;
    }
    v25 = (unsigned long)(~(unsigned int)v15 & 7);
    v35 = &((char *)((long)v27 + 1))[(long)v15];
    while (v25) {
      v1 = &v35[1];
      *v35 = 0;
      v25 -= 1;
      v35 = v1;
    }
    v36 = v27;
    for (v26 = v13; *v26 == *v36; v26 = &v26[1]) {
      v36 = &v36[1];
    }
    v12 = 0;
    do {
      v21 = v12;
      v12 = v21 + 1;
    } while (*(char *)((long)v26 + v21) == *(char *)((long)v36 + v21));
    v38 = (char *)((long)v26 + (v21 - (long)v13));
    v16 = &v39[(long)v38];
    if ((v38) && (!dat_d0b4)) goto label_32fc;
    if (v23 <= v38) goto label_328b;
    if (dat_d0b4 != 2) {
      if (3 <= dat_d0b4) {
        if (dat_d0b4 != 3) goto label_328b;
        goto label_3380;
      }
      if (!dat_d0b4) goto label_3496;
      do {
        v2 = *(unsigned char *)((long)v13 + (long)v38);
        v3 = *(unsigned char *)((long)v27 + (long)v38);
        if (v2 != v3) {
          v16 = (char *)sub_3fe0(&v38[(long)v39],v29);
          v37 = (unsigned int)v2;
          if (dat_d0b2) { // branch-flip
            sub_3bd0(v30,v37);
            sub_3bd0(v31,v3);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",v42,v16,v37,v30,(unsigned int)v3,v31);
          }
          else {
            __printf_chk(1,"%*s %3o %3o\n",v42,v16,v37,(unsigned int)v3);
          }
        }
        v38 = &v38[1];
      } while (v23 != v38);
      v41 = -1;
      v16 = &v23[(long)v39];
      goto label_328b;
    }
    if (v14 == v15) {
      v41 = 1;
      goto label_32a1;
    }
  }
  else {
    v16 = &v39[(long)v23];
    if ((!v23) || (dat_d0b4)) goto label_328b;
    v38 = v23;
label_32fc:
    v39 = v16;
    v16 = (char *)((long)v13 + (long)v38);
    v4 = *v16;
    v12 = 0;
    *v16 = '\n';
    v26 = v13;
    while (v17 = rawmemchr(v26,10), v16 != v17) {
      v12 += 1;
      v26 = (long *)&v17[1];
    }
    v43 += v12;
    *v16 = v4;
    v16 = v39;
    if (v38 < v23) {
label_3496:
      v39 = v16;
      v15 = (char *)sub_3fe0(v39,v31);
      v14 = (char *)sub_3fe0(v43,v29);
      if (dat_d0b2) { // branch-flip
        v2 = *(unsigned char *)((long)v13 + (long)v38);
        v3 = *(unsigned char *)((long)v27 + (long)v38);
        sub_3bd0(v33,v2);
        sub_3bd0(v30,(unsigned int)v3);
        v16 = dat_d238;
        v23 = dat_d230;
        __printf_chk(1,dcgettext(NULL,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5),v23,v16,v15,v14,(unsigned int)v2,v33,(unsigned int)v3,v30);
      }
      else {
        v23 = dcgettext(NULL,"%s %s differ: byte %s, line %s\n",5);
        if ((v23 == "%s %s differ: byte %s, line %s\n") && (!sub_3f50(5)))
          v23 = "%s %s differ: char %s, line %s\n";
        __printf_chk(1,v23,dat_d230,dat_d238,v15,v14);
      }
      goto label_3380;
    }
    v44 = ((char *)((long)v13 + -1))[(long)v38] == '\n';
label_328b:
    v39 = v16;
    if (v14 == v15) {
      if (v14 != dat_d0d0) {
label_32a1:
        v37 = (unsigned int)(v41 != 0);
label_32ac:
        if (!close(dat_d220)) {
          if (!close(dat_d224)) {
            if ((v37) && (dat_d0b4 < 2))
              sub_3c90();
            exit(v37); // no-return
          }
          v45 = 1;
        }
        error(2,*__errno_location(),"%s",*(char **)((long)v45 * 8 + 0xd230));
        return v9;
      }
      goto label_2ebc;
    }
    if (dat_d0b4 == 3) goto label_3380;
    v15 = *(char **)((unsigned long)(v15 < v14) * 8 + 0xd230);
    if (v39 == (char *)0x1) {
      v37 = 1;
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s which is empty\n",5),v15);
      goto label_32ac;
    }
    v14 = (char *)sub_3fe0(&v39[-1],v31);
    if (dat_d0b4) // branch-flip
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s after byte %s\n",5),v15,v14);
    else {
      v22 = sub_3fe0(v43 - v44,v29);
      if (v44) // branch-flip
        v23 = dcgettext(NULL,"cmp: EOF on %s after byte %s, line %s\n",5);
      else {
        v23 = dcgettext(NULL,"cmp: EOF on %s after byte %s, in line %s\n",5);
      }
      __fprintf_chk(stderr,1,v23,v15,v14,v22);
    }
  }
label_3380:
  v37 = 1;
  goto label_32ac;
}


// Function: sub_3a80 @ 0x3a80
void sub_3a80(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1d280;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_3ae0 @ 0x3ae0
void sub_3ae0(int a0,unsigned long *a1,char a2) // return-dupe
{
  unsigned char *v1;
  int v2; // eax
  long v3; // stack - 0x38
  
  v1 = (unsigned char *)*a1;
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

