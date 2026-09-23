// Function: sub_2940 @ 0x2940
unsigned long sub_2940(int a0,char **a1)
{
  unsigned long v1;
  long v10; // rax
  long v11; // rax
  long v12;
  long v13;
  unsigned long v14; // rax
  char *v15; // stack - 0x170
  long v16; // stack - 0x168
  long v17; // stack - 0xd8
  long v18; // stack - 0x160
  unsigned int v19; // stack - 0x150
  char *v2;
  long v20; // stack - 0x140
  long v21; // stack - 0xd0
  unsigned int v22; // stack - 0xc0
  long v23; // stack - 0xb0
  int v3;
  int v4; // eax
  int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  long v8;
  unsigned long v9; // rax
  
  dat_d020 = 2;
  sub_4180(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_3f90(0);
  sub_64f0();
  while (v3 = getopt_long(a0,a1,"bci:ln:sv",(void *)0xca40,NULL), v3 != -1) {
    switch(v3) {
      case 0x62:
      case 99:
        dat_d0b2 = 1;
        break;
      default:
        sub_3240(0,0); // no-return
      case 0x69:
        sub_32d0(0,&optarg,0x3a);
        v2 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v2;
          if (dat_d0c8 < dat_d0c0)
            dat_d0c8 = dat_d0c0;
        }
        else {
          optarg = v2;
          sub_32d0(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_32a0(1);
        break;
      case 0x6e:
        v3 = sub_6640(optarg,0,0,&v15,"kKMGTPEZY0");
        if ((v3) || ((long)v15 < 0))
          sub_3240("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_d010 < 0) || ((long)v15 <= (long)dat_d010))
          dat_d010 = v15;
        break;
      case 0x73:
        sub_32a0(3);
        break;
      case 0x76:
        v6 = sub_4490("David MacKenzie");
        v7 = sub_4520("Torbjorn Granlund",0x9682);
        sub_5f90(stdout,"cmp","GNU diffutils",dat_d018,v7,v6,0,v7);
        sub_3d80();
        return 0;
      case 0x80:
        sub_3370();
        sub_3d80();
        return 0;
      
    }
  }
  if (optind == a0)
    sub_3240("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
  dat_d230 = a1[optind];
  dat_d238 = "-";
  v3 = optind + 1;
  if (optind + 1 < a0) {
    dat_d238 = a1[(long)optind + 1];
    v3 = optind + 2;
  }
  optind = v3;
  v8 = (long)optind;
  if (optind < a0) {
    optind += 1;
    v15 = a1[v8];
    sub_32d0(0,&v15,0);
    v8 = (long)optind;
    if (optind < a0) {
      optind += 1;
      v15 = a1[v8];
      sub_32d0(1,&v15,0);
      if (optind < a0)
        sub_3240("extra operand \'%s\'",a1[optind]); // no-return
    }
  }
  v8 = 0;
  do {
    if (((v8) && (dat_d0c0 == dat_d0c8)) && (v3 = strcmp(dat_d230,dat_d238), !v3))
      return 0;
    v2 = *(char **)(v8 * 8 + 0xd230);
    if (strcmp(v2,"-")) { // branch-flip
      v3 = open(v2,0,0);
      *(int *)(v8 * 4 + 0xd220) = v3;
      if (v3 <= -1) {
        if (dat_d0b4 == 3)
          exit(2); // no-return
        error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230)); // return-dupe
        return v9;
      }
    }
    else {
      *(unsigned int *)(v8 * 4 + 0xd220) = 0;
      v3 = 0;
    }
    if (fstat(v3,(void *)(v8 * 0x90 + 0xd100))) {
      error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230));
      return v9;
    }
    if (v8 == 1) goto label_2ca0;
    v8 = 1;
  } while( true );
label_2ca0:
  if ((dat_d108 != dat_d198) || (dat_d100 != dat_d190)) {
    if ((dat_d118 & 0xf000) != 0x6000) {
      if ((dat_d118 & 0xf000) == 0x2000) {
        if ((dat_d1a8 & 0xf000) == 0x2000) goto label_2f2c;
        goto label_2e90;
      }
      goto label_2cd7;
    }
    if ((dat_d1a8 & 0xf000) == 0x6000) {
label_2f2c:
      if (dat_d128 == dat_d1b8) goto label_2dd8;
      goto label_2cd7;
    }
label_2e90:
    if (dat_d0b4 == 3) goto label_2d0c;
label_2e9d:
    if ((fstat(1,&v16)) || (stat("/dev/null",&v17))) goto label_2eb4;
    if ((v18 != v21) || (v16 != v17)) {
      if ((v19 & 0xf000) != 0x6000) { // branch-flip
        if (((v19 & 0xf000) == 0x2000) && ((v22 & 0xf000) == 0x2000)) {
label_2faa:
          if (v20 == v23) goto label_2fbd;
        }
      }
      else if ((v22 & 0xf000) == 0x6000) goto label_2faa;
    }
    else {
label_2fbd:
      dat_d0b4 = 2;
    }
label_2eb4:
    if (dat_d0b4 != 3) goto label_2d0c;
  }
  else {
label_2dd8:
    if (((dat_d118 == dat_d1a8) && (dat_d110 == dat_d1a0)) && ((dat_d11c == dat_d1ac && ((((dat_d120 == dat_d1b0 && (dat_d130 == dat_d1c0)) && (dat_d158 == dat_d1e8)) && (dat_d168 == dat_d1f8)))))) {
      v8 = sub_34c0(0);
      if (v8 == sub_34c0(1))
        return 0;
    }
label_2cd7:
    if (dat_d0b4 != 3) goto label_2e9d;
  }
  v8 = dat_d130;
  if (((dat_d118 & 0xf000) == 0x8000) && ((dat_d1a8 & 0xf000) == 0x8000)) {
    v10 = sub_34c0(0);
    v13 = dat_d1c0;
    v11 = sub_34c0(1);
    v12 = 0;
    if (0 <= v8 - v10)
      v12 = v8 - v10;
    v13 -= v11;
    if (v13 <= -1)
      v13 = 0;
    if (v13 != v12) {
      if (0 <= (long)dat_d010) {
        if (v12 < v13)
          v13 = v12;
        if ((long)dat_d010 <= v13) goto label_2d0c;
      }
      exit(1); // no-return
    }
  }
label_2d0c:
  dat_d0d0 = sub_6ac0(dat_d138,dat_d1c8,0x7ffffffffffffff7);
  v1 = dat_d0d0 + 0xf;
  dat_d0e0 = sub_6100((v1 >> 3) << 4);
  dat_d0e8 = dat_d0e0 + (v1 & 0xfffffffffffffff8);
  v4 = sub_35e0();
  v5 = close(dat_d220);
  v3 = 0;
  if (!v5) {
    v3 = close(dat_d224);
    if (!v3) {
      if ((v4) && (dat_d0b4 < 2))
        sub_3d80();
      exit(v4); // no-return
    }
    v3 = 1;
  }
  error(2,*__errno_location(),"%s",*(unsigned long *)((long)v3 * 8 + 0xd230));
  return v14;
}


// Function: sub_31f0 @ 0x31f0
long sub_31f0(long *a0,long *a1)
{
  long *v1;
  
  v1 = a0;
  if (*a0 != *a1) { // branch-flip
    for (; (char)*v1 == (char)*a1; v1 = (long *)((long)v1 + 1)) {
label_3230:
      a1 = (long *)((long)a1 + 1);
    }
  }
  else {
    do {
      v1 = &v1[1];
      a1 = &a1[1];
    } while (*v1 == *a1);
    if ((char)*v1 == (char)*a1) goto label_3230;
  }
  return (long)v1 - (long)a0;
}


// Function: sub_3240 @ 0x3240
void sub_3240(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1d280;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_32a0 @ 0x32a0
void sub_32a0(int a0)
{
  if ((dat_d0b4) && (dat_d0b4 != a0))
    sub_3240("options -l and -s are incompatible",0); // no-return
  dat_d0b4 = a0;
}


// Function: sub_32d0 @ 0x32d0
void sub_32d0(int a0,unsigned long *a1,char a2) // return-dupe
{
  unsigned long v1;
  int v2; // eax
  long v3; // stack - 0x38
  
  v1 = *a1;
  v2 = sub_6640(v1,a1,0,&v3,"kKMGTPEZY0");
  if (((!v2) || ((v2 == 2 && (*(char *)*a1 == a2)))) && (0 <= v3)) {
    if (v3 <= *(long *)((long)a0 * 8 + 0xd0c0))
      return;
    *(long *)((long)a0 * 8 + 0xd0c0) = v3;
    return;
  }
  sub_3240("invalid --ignore-initial value \'%s\'",v1); // no-return
}


// Function: sub_3370 @ 0x3370
void sub_3370(void)
{
  void *v1;
  unsigned long v2;
  char *v3; // rax
  long *v4; // rbx
  char *v5;
  
  v2 = dat_1d280;
  v4 = (long *)0xc9e0;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5),v2);
  __printf_chk(1,"%s\n",dcgettext(NULL,"Compare two files byte by byte.",5));
  __printf_chk(1,"\n%s\n\n",dcgettext(NULL,"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v5 = "-b, --print-bytes          print differing bytes";
  do {
    v4 = &v4[1];
    __printf_chk(1,"  %s\n",dcgettext(NULL,v5,5));
    v5 = (char *)*v4;
  } while (v5);
  v5 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
  v3 = dcgettext(NULL,"If a FILE is \'-\' or missing, read standard input.",5);
  __printf_chk(1,"\n%s\n\n%s\n%s\n",dcgettext(NULL,"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",5),v3,v5);
  sub_6040(); // tail-call
}


// Function: sub_34c0 @ 0x34c0
unsigned long sub_34c0(int a0)
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


// Function: sub_3510 @ 0x3510
long sub_3510(char *a0,long a1)
{
  char *v1;
  char v2;
  long v3; // r12
  
  v3 = 0;
  v1 = &a0[a1];
  v2 = *v1;
  *v1 = 10;
  while( true ) {
    a0 = rawmemchr(a0,10);
    if (v1 == a0) break;
    v3 += 1;
    a0 = &a0[1];
  }
  *v1 = v2;
  return v3;
}


// Function: sub_3550 @ 0x3550
void sub_3550(unsigned char *a0,unsigned char a1)
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


// Function: sub_35e0 @ 0x35e0
unsigned long sub_35e0(void) // return-dupe x2
{
  unsigned char v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long v12;
  int *v13; // rax
  unsigned long v14; // rax
  unsigned long v15; // rax
  unsigned long v16;
  char *v17; // rax
  unsigned long v18; // rax
  unsigned long v19; // rax
  void *v2;
  char v20 [24];
  char v21 [5];
  char v22 [32];
  char v23 [5];
  long v24;
  long v25; // r13
  long v26;
  unsigned long v27; // stack - 0xe0
  int v28; // stack - 0xd8
  unsigned int v29; // stack - 0xcc
  void *v3;
  long v30; // stack - 0xc8
  bool v31; // stack - 0xa1
  unsigned long v4;
  unsigned char v5;
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned long v8;
  unsigned long v9;
  
  v26 = dat_d130;
  v3 = dat_d0e8;
  v2 = dat_d0e0;
  v27 = dat_d010;
  if (dat_d0b4 == 1) {
    v8 = 0x7fffffffffffffff;
    if (0 <= (long)dat_d010)
      v8 = dat_d010;
    if ((dat_d118 & 0xf000) == 0x8000) {
      v9 = v26 - sub_34c0(0);
      if ((long)v9 < (long)v8)
        v8 = v9;
    }
    v26 = dat_d1c0;
    if ((dat_d1a8 & 0xf000) == 0x8000) {
      v9 = v26 - sub_34c0(1);
      if ((long)v9 < (long)v8)
        v8 = v9;
    }
    v29 = 1;
    while (v8 = (long)v8 / 10, v8) {
      v29 += 1;
    }
  }
  v26 = 0;
  do {
    v8 = *(unsigned long *)(v26 * 8 + 0xd0c0);
    if ((v8) && (sub_34c0(v26) == -1)) {
      do {
        v9 = dat_d0d0;
        if (v8 <= dat_d0d0)
          v9 = v8;
        v12 = sub_6a10(*(unsigned int *)(v26 * 4 + 0xd220),v2,v9);
        if (v9 != v12) {
          if (v12 == 0xffffffffffffffff) {
            v13 = __errno_location();
            error(2,*v13,"%s",*(unsigned long *)((long)(int)v26 * 8 + 0xd230));
            return v14;
          }
          break;
        }
        v8 -= v9;
      } while (v8);
    }
    if (v26 == 1) goto label_3658;
    v26 = 1;
  } while( true );
label_3658:
  v31 = 1;
  v28 = 0;
  v30 = 1;
  v26 = 1;
  v8 = dat_d0d0;
  do {
    v9 = v8;
    if (0 <= (long)v27) {
      if (v27 <= v8)
        v9 = v27;
      v27 -= v9;
    }
    v8 = sub_6a10(dat_d220,v2,v9);
    if (v8 == 0xffffffffffffffff) {
      error(2,*__errno_location(),"%s",dat_d230);
      return v19;
    }
    v9 = sub_6a10(dat_d224,v3,v9);
    if (v9 == 0xffffffffffffffff) {
      error(2,*__errno_location(),"%s",dat_d238);
      return v18;
    }
    v12 = v9;
    if (v8 <= v9)
      v12 = v8;
    if (memcmp(v2,v3,v12)) { // branch-flip
      if ((v9 <= v8) && (*(unsigned char *)((long)v3 + v8) = 0x55, v8 != v9))
        v5 = 0xaa;
      else {
        *(unsigned char *)((long)v2 + v9) = 0x79;
        v5 = ~*(unsigned char *)((long)v3 + v8);
      }
      *(unsigned char *)((long)v2 + v8) = v5;
      *(unsigned char *)((long)v3 + v9) = ~*(unsigned char *)((long)v2 + v9);
      v6 = ~(unsigned int)v8 & 7;
      if (v6) {
        v7 = 0;
        do {
          v10 = (unsigned long)v7;
          v7 += 1;
          *(char *)((long)v2 + v10 + v8 + 1) = 0;
        } while (v7 < v6);
      }
      v6 = ~(unsigned int)v9 & 7;
      if (v6) {
        v7 = 0;
        do {
          v10 = (unsigned long)v7;
          v7 += 1;
          *(char *)((long)v3 + v10 + v9 + 1) = 0;
        } while (v7 < v6);
      }
      v10 = sub_31f0(v2,v3);
      v24 = v26 + v10;
      if ((v10) && (!dat_d0b4)) {
label_3905:
        v30 += sub_3510(v2,v10);
        if (v10 < v12) {
label_3a01:
          v11 = sub_40e0(v24,v22);
          v15 = sub_40e0(v30,v20);
          if (!dat_d0b2) {
            v17 = dcgettext(NULL,"%s %s differ: byte %s, line %s\n",5);
            if ((v17 == "%s %s differ: byte %s, line %s\n") && (!sub_4050(5)))
              v17 = "%s %s differ: char %s, line %s\n";
            __printf_chk(1,v17,dat_d230,dat_d238,v11,v15);
            return 1;
          }
          v5 = *(unsigned char *)((long)v2 + v10);
          v1 = *(unsigned char *)((long)v3 + v10);
          sub_3550(v23,(unsigned long)v5);
          sub_3550(v21,v1);
          v4 = dat_d238;
          v16 = dat_d230;
          __printf_chk(1,dcgettext(NULL,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5),v16,v4,v11,v15,(unsigned long)v5,v23,(unsigned long)v1,v21);
          return 1;
        }
        v31 = *(char *)((long)v2 + (v10 - 1)) == '\n';
      }
      else if (v10 < v12) {
        if (dat_d0b4 == 2) {
          v28 = 1;
          if (v8 != v9)
            return 1;
          break;
        }
        if (3 <= dat_d0b4) { // branch-flip
          if (dat_d0b4 == 3)
            return 1;
        }
        else {
          if (!dat_d0b4) goto label_3a01;
          do {
            while( true ) {
              v25 = v24;
              v5 = *(unsigned char *)((long)v2 + (v25 - v26));
              v1 = *(unsigned char *)((long)v3 + (v25 - v26));
              if (v5 != v1) break;
label_37fa:
              v24 = v25 + 1;
              if (v26 + v12 == v25 + 1) goto label_38b0;
            }
            v11 = sub_40e0(v25,v20);
            if (!dat_d0b2) {
              __printf_chk(1,"%*s %3o %3o\n",(unsigned long)v29,v11,(unsigned long)v5,(unsigned long)v1);
              goto label_37fa;
            }
            sub_3550(v21,(unsigned long)v5);
            sub_3550(v22,v1);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",(unsigned long)v29,v11,(unsigned long)v5,v21,(unsigned long)v1,v22);
            v24 = v25 + 1;
          } while (v26 + v12 != v25 + 1);
label_38b0:
          v24 = v25 + 1;
          v28 = -1;
        }
      }
    }
    else {
      v24 = v26 + v12;
      if ((!dat_d0b4) && (v12)) {
        v10 = v12;
        goto label_3905;
      }
    }
    if (v8 != v9) {
      if (dat_d0b4 == 3)
        return 1;
      v11 = *(unsigned long *)((unsigned long)(v9 < v8) * 8 + 0xd230);
      if (v24 != 1) {
        v15 = sub_40e0(v24 + -1,v22);
        if (!dat_d0b4) {
          v16 = sub_40e0(v30 - (unsigned long)v31,v20);
          if (v31) // branch-flip
            v17 = dcgettext(NULL,"cmp: EOF on %s after byte %s, line %s\n",5);
          else {
            v17 = dcgettext(NULL,"cmp: EOF on %s after byte %s, in line %s\n",5);
          }
          __fprintf_chk(stderr,1,v17,v11,v15,v16);
          return 1;
        }
        __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s after byte %s\n",5),v11,v15);
        return 1;
      }
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s which is empty\n",5),v11);
      return 1;
    }
    v26 = v24;
  } while (dat_d0d0 == v8);
  return (unsigned long)(v28 != 0);
}


// Function: sub_3d80 @ 0x3d80
void sub_3d80(void)
{
  int v1; // eax
  char *v2; // rax
  
  v1 = ferror_unlocked(stdout);
  if (v1) {
    error(2,0,"%s",dcgettext(NULL,"write failed",5));
    return;
  }
  v1 = fclose(stdout);
  if (!v1)
    return;
  v2 = dcgettext(NULL,"standard output",5);
  error(2,*__errno_location(),"%s",v2);
}

