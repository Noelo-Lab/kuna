// Function: sub_29e9 @ 0x29e9
void sub_29e9(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,gettext(a0),a1);
  v1 = dat_20268;
  error(2,0,gettext("Try \'%s --help\' for more information."),v1);
}


// Function: sub_2a5f @ 0x2a5f
void sub_2a5f(int a0,long *a1,char a2) // return-dupe
{
  int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  v3 = *a1;
  v1 = sub_7e2e(v3,a1,0,&v2,"kKMGTPEZY0");
  if (((!v1) || ((v1 == 2 && (a2 == *(char *)*a1)))) && (0 <= v2)) {
    if (v2 <= *(long *)((long)a0 * 8 + 0x10200))
      return;
    *(long *)((long)a0 * 8 + 0x10200) = v2;
    return;
  }
  sub_29e9("invalid --ignore-initial value \'%s\'",v3); // no-return
}


// Function: sub_2b46 @ 0x2b46
void sub_2b46(int a0)
{
  if ((dat_10210) && (a0 != dat_10210))
    sub_29e9("options -l and -s are incompatible",0); // no-return
  dat_10210 = a0;
}


// Function: sub_2b8a @ 0x2b8a
void sub_2b8a(void)
{
  char *v1; // rax
  
  if (ferror_unlocked(stdout)) {
    error(2,0,"%s",gettext("write failed"));
    return;
  }
  if (fclose(stdout)) {
    v1 = gettext("standard output");
    error(2,*__errno_location(),"%s",v1);
    return;
  }
}


// Function: sub_2c28 @ 0x2c28
void sub_2c28(void)
{
  FILE *v1;
  char *v2;
  char *v3; // rax
  long *v4; // stack - 0x20
  
  v2 = dat_20268;
  printf(gettext("Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n"),v2);
  printf("%s\n",gettext("Compare two files byte by byte."));
  printf("\n%s\n\n",gettext("The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  for (v4 = (long *)0xfb40; *v4; v4 = &v4[1]) {
    v2 = (char *)*v4;
    printf("  %s\n",gettext(v2));
  }
  v2 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  v3 = gettext("If a FILE is \'-\' or missing, read standard input.");
  printf("\n%s\n\n%s\n%s\n",gettext("SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."),v3,v2);
  sub_74c0();
}


// Function: main @ 0x2d6d
int main(int argc,char **argv,char **envp)
{
  int v1;
  long v10; // rax
  char *v11; // stack - 0x168
  stat v12; // stack - 0x148
  stat v13; // stack - 0xb8
  int v14; // stack - 0x17c
  int v15; // stack - 0x178
  int v16; // stack - 0x174
  long v17; // stack - 0x160
  long v18; // stack - 0x158
  unsigned long v19; // stack - 0x150
  char *v2;
  bool v3;
  long v4;
  int v5;
  int v6; // eax
  int v7; // eax
  unsigned long v8; // rax
  long v9;
  
  dat_10020 = 2;
  sub_4a4b(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_479f(NULL);
  sub_7c09();
  while (v5 = getopt_long(argc,argv,"bci:ln:sv",(option *)0xfa00,NULL), v5 != -1) {
    switch(v5) {
      case 0x62:
      case 99:
        dat_10214 = 1;
        break;
      default:
        sub_29e9(NULL,0); // no-return
      case 0x69:
        sub_2a5f(0,&optarg,0x3a);
        v2 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v2;
          if (dat_10208 < dat_10200)
            dat_10208 = dat_10200;
        }
        else {
          optarg = v2;
          sub_2a5f(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_2b46(1);
        break;
      case 0x6e:
        v5 = sub_7e2e(optarg,NULL,0,&v11,"kKMGTPEZY0");
        if ((v5) || ((long)v11 <= -1))
          sub_29e9("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_10010 < 0) || ((long)v11 <= (long)dat_10010))
          dat_10010 = v11;
        break;
      case 0x73:
        sub_2b46(3);
        break;
      case 0x76:
        v8 = sub_500c("David MacKenzie");
        sub_73dc(stdout,"cmp","GNU diffutils",dat_10018,sub_50b2("Torbjorn Granlund",0xc60c),v8,0);
        sub_2b8a();
        return 0;
      case 0x80:
        sub_2c28();
        sub_2b8a();
        return 0;
      
    }
  }
  if (argc == optind)
    sub_29e9("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  v5 = optind + 1;
  dat_100a0 = argv[optind];
  if (argc <= v5) { // branch-flip
    dat_100a8 = "-";
    optind = v5;
  }
  else {
    optind += 2;
    dat_100a8 = argv[v5];
  }
  v14 = 0;
  while ((v14 <= 1 && (optind < argc))) {
    v5 = optind + 1;
    v11 = argv[optind];
    optind = v5;
    sub_2a5f(v14,&v11,0);
    v14 += 1;
  }
  if (optind < argc)
    sub_29e9("extra operand \'%s\'",argv[optind]); // no-return
  for (v15 = 0; v15 <= 1; v15 = v15 + 1) {
    if (((v15) && (dat_10200 == dat_10208)) && (!strcmp(dat_100a0,dat_100a8)))
      return 0;
    v2 = *(char **)((long)v15 * 8 + 0x100a0);
    if (strcmp(v2,"-")) { // branch-flip
      v2 = *(char **)((long)v15 * 8 + 0x100a0);
      *(int *)((long)v15 * 4 + 0x100b0) = open(v2,0,0);
    }
    else {
      *(unsigned int *)((long)v15 * 4 + 0x100b0) = 0;
    }
    if ((*(int *)((long)v15 * 4 + 0x100b0) < 0) || (v5 = *(int *)((long)v15 * 4 + 0x100b0), fstat(v5,(stat *)((long)v15 * 0x90 + 0x100c0)))) {
      if ((*(int *)((long)v15 * 4 + 0x100b0) <= -1) && (dat_10210 == 3))
        exit(2); // no-return
      v2 = *(char **)((long)v15 * 8 + 0x100a0);
      error(2,*__errno_location(),"%s",v2);
      return v6;
    }
  }
  if (((dat_100c8 != dat_10158) || (dat_100c0 != dat_10150)) && (((((dat_100d8 & 0xf000) != 0x6000 || ((dat_10168 & 0xf000) != 0x6000)) && (((dat_100d8 & 0xf000) != 0x2000 || ((dat_10168 & 0xf000) != 0x2000)))) || (dat_100e8 != dat_10178))))
    v3 = 0;
  else {
    v3 = 1;
  }
  if ((((((v3) && (dat_100d8 == dat_10168)) && (dat_100d0 == dat_10160)) && ((dat_100dc == dat_1016c && (dat_100e0 == dat_10170)))) && (dat_100f0 == dat_10180)) && ((dat_10118 == dat_101a8 && (dat_10128 == dat_101b8)))) {
    v9 = sub_447f(0);
    if (v9 == sub_447f(1))
      return 0;
  }
  if (((dat_10210 != 3) && (!fstat(1,&v12))) && (!stat("/dev/null",&v13))) {
    if (((v12._8_8_ != v13._8_8_) || (v12._0_8_ != v13._0_8_)) && (((((v12._24_4_ & 0xf000) != 0x6000 || ((v13._24_4_ & 0xf000) != 0x6000)) && (((v12._24_4_ & 0xf000) != 0x2000 || ((v13._24_4_ & 0xf000) != 0x2000)))) || (v12._40_8_ != v13._40_8_))))
      v3 = 0;
    else {
      v3 = 1;
    }
    if (v3)
      dat_10210 = 2;
  }
  v9 = dat_100f0;
  if (((dat_10210 == 3) && ((dat_100d8 & 0xf000) == 0x8000)) && ((dat_10168 & 0xf000) == 0x8000)) {
    v10 = sub_447f(0);
    v4 = dat_10180;
    v17 = v9 - v10;
    v18 = v4 - sub_447f(1);
    if (v17 <= -1)
      v17 = 0;
    if (v18 <= -1)
      v18 = 0;
    if (v17 != v18) {
      if (0 <= (long)dat_10010) {
        v9 = v17;
        if (v18 < v17)
          v9 = v18;
        if ((long)dat_10010 <= v9) goto label_368d;
      }
      exit(1); // no-return
    }
  }
label_368d:
  dat_101f0 = sub_8307(dat_100f8,dat_10188,(void *)0x7ffffffffffffff7);
  v19 = dat_101f0 + 0xfU >> 3;
  dat_101e0 = sub_7605(v19 << 4);
  dat_101e8 = v19 * 8 + dat_101e0;
  v5 = sub_37d2();
  v16 = 0;
  while( true ) {
    if (2 <= v16) {
      if ((v5) && (dat_10210 <= 1))
        sub_2b8a();
      exit(v5); // no-return
    }
    v1 = *(int *)((long)v16 * 4 + 0x100b0);
    if (close(v1)) break;
    v16 += 1;
  }
  v2 = *(char **)((long)v16 * 8 + 0x100a0);
  error(2,*__errno_location(),"%s",v2);
  return v7;
}


// Function: sub_37d2 @ 0x37d2
unsigned long sub_37d2(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_37d2
{
  unsigned char v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13;
  char *v14; // rax
  unsigned long v15; // rax
  char *v16;
  char v17 [32];
  char v18 [24];
  char v19 [5];
  unsigned char v2;
  char v20 [5];
  bool v21; // stack - 0x14a
  int v22; // stack - 0x144
  int v23; // stack - 0x140
  int v24; // stack - 0x13c
  long v25; // stack - 0x138
  long v26; // stack - 0x130
  unsigned long v27; // stack - 0x128
  unsigned long v28; // stack - 0x120
  unsigned long v29; // stack - 0x118
  long v3;
  unsigned long v30; // stack - 0x110
  unsigned long v31; // stack - 0x108
  unsigned long v32; // stack - 0xd8
  unsigned long v33; // stack - 0xd0
  char *v4;
  void *v5;
  void *v6;
  bool v7; // al
  unsigned long v8;
  unsigned long v9; // rax
  
  v6 = dat_101e8;
  v5 = dat_101e0;
  v21 = 1;
  v25 = 1;
  v26 = 1;
  v27 = dat_10010;
  v22 = 0;
  if (dat_10210 == 1) {
    if (0 <= (long)dat_10010) // branch-flip
      v29 = dat_10010;
    else {
      v29 = 0x7fffffffffffffff;
    }
    for (v23 = 0; v23 <= 1; v23 = v23 + 1) {
      if ((*(unsigned int *)((long)v23 * 0x90 + 0x100d8) & 0xf000) == 0x8000) {
        v3 = *(long *)((long)v23 * 0x90 + 0x100f0);
        v8 = v3 - sub_447f(v23);
        if ((long)v8 < (long)v29)
          v29 = v8;
      }
    }
    v24 = 1;
    while (v29 = (long)v29 / 10, v29) {
      v24 += 1;
    }
  }
  v23 = 0;
  do {
    if (2 <= v23) {
      while( true ) {
        v31 = dat_101f0;
        if (0 <= (long)v27) {
          if (v27 < dat_101f0)
            v31 = v27;
          v27 -= v31;
        }
        v32 = sub_8238(dat_100b0,v5,v31);
        v13 = dat_100a0;
        if (v32 == 0xffffffffffffffff) {
          error(2,*__errno_location(),"%s",v13);
          return v11;
        }
        v33 = sub_8238(dat_100b4,v6,v31);
        v13 = dat_100a8;
        if (v33 == 0xffffffffffffffff) {
          error(2,*__errno_location(),"%s",v13);
          return v12;
        }
        v8 = v33;
        if (v32 <= v33)
          v8 = v32;
        if (memcmp(v5,v6,v8)) { // branch-flip
          if (v33 <= v32)
            *(char *)(v32 + (long)v6) = 0x55;
          if (v32 <= v33)
            *(char *)(v33 + (long)v5) = 0x79;
          *(unsigned char *)(v32 + (long)v5) = ~*(unsigned char *)(v32 + (long)v6);
          *(unsigned char *)(v33 + (long)v6) = ~*(unsigned char *)(v33 + (long)v5);
          memset((void *)((long)v5 + v32 + 1),0,(unsigned long)(~(unsigned int)v32 & 7));
          memset((void *)((long)v6 + v33 + 1),0,(unsigned long)(~(unsigned int)v33 & 7));
          v28 = sub_42d7(v5,v6);
        }
        else {
          v28 = v8;
        }
        v26 = v28 + v26;
        if ((!dat_10210) && (v28)) {
          v25 = sub_4350(v5,v28) + v25;
          v21 = *(char *)((long)v5 + (v28 - 1)) == '\n';
        }
        if (v28 < v8) {
          switch(dat_10210) {
            case 0:
              v13 = (char *)sub_490d(v26,v17);
              v14 = (char *)sub_490d(v25,v18);
              if (dat_10214 == '\x01') {
                v1 = *(unsigned char *)(v28 + (long)v5);
                v2 = *(unsigned char *)(v28 + (long)v6);
                sub_43cf(v19,v1);
                sub_43cf(v20,v2);
                v4 = dat_100a8;
                v16 = dat_100a0;
                printf(gettext("%s %s differ: byte %s, line %s is %3o %s %3o %s\n"),v16,v4,v13,v14,(unsigned int)v1,v19,(unsigned int)v2,v20);
                return 1;
              }
              v16 = gettext("%s %s differ: byte %s, line %s\n");
              v7 = ((v16 != "%s %s differ: byte %s, line %s\n") || (sub_4863(5)));
              if (!v7)
                v16 = "%s %s differ: char %s, line %s\n";
              printf(v16,dat_100a0,dat_100a8,v13,v14);
              return 1;
            case 1:
              do {
                v1 = *(unsigned char *)(v28 + (long)v5);
                v2 = *(unsigned char *)(v28 + (long)v6);
                if (v1 != v2) {
                  v13 = (char *)sub_490d(v26,v18);
                  if (dat_10214 != '\x01') // branch-flip
                    printf("%*s %3o %3o\n",v24,v13,(unsigned int)v1,(unsigned int)v2);
                  else {
                    sub_43cf(v20,v1);
                    sub_43cf(v17,v2);
                    printf("%*s %3o %-4s %3o %s\n",v24,v13,(unsigned int)v1,v20,(unsigned int)v2,v17);
                  }
                }
                v26 += 1;
                v28 += 1;
              } while (v28 < v8);
              v22 = -1;
              break;
            case 2:
              v22 = 1;
              break;
            case 3:
              return 1;
            
          }
        }
        if (v32 != v33) break;
        if ((1 <= v22) || (v32 != dat_101f0))
          return (unsigned long)(v22 != 0);
      }
      if ((v22 <= 0) && (dat_10210 != 3)) {
        v13 = *(char **)((long)(int)(unsigned int)(v33 < v32) * 8 + 0x100a0);
        if (v26 != 1) { // branch-flip
          v14 = (char *)sub_490d(v26 + -1,v17);
          if (dat_10210) // branch-flip
            fprintf(stderr,gettext("cmp: EOF on %s after byte %s\n"),v13,v14);
          else {
            v15 = sub_490d(v25 - (unsigned long)v21,v18);
            if (v21) // branch-flip
              v16 = gettext("cmp: EOF on %s after byte %s, line %s\n");
            else {
              v16 = gettext("cmp: EOF on %s after byte %s, in line %s\n");
            }
            fprintf(stderr,v16,v13,v14,v15);
          }
        }
        else {
          fprintf(stderr,gettext("cmp: EOF on %s which is empty\n"),v13);
        }
      }
      return 1;
    }
    v30 = *(unsigned long *)((long)v23 * 8 + 0x10200);
    if ((v30) && (sub_447f(v23) == -1)) {
      do {
        v8 = dat_101f0;
        if (v30 <= dat_101f0)
          v8 = v30;
        v9 = sub_8238(*(unsigned int *)((long)v23 * 4 + 0x100b0),v5,v8);
        if (v9 != v8) {
          if (v9 == 0xffffffffffffffff) {
            v13 = *(char **)((long)v23 * 8 + 0x100a0);
            error(2,*__errno_location(),"%s",v13);
            return v10;
          }
          break;
        }
        v30 -= v9;
      } while (v30);
    }
    v23 += 1;
  } while( true );
}


// Function: sub_42d7 @ 0x42d7
long sub_42d7(long *a0,long *a1)
{
  long *v1; // stack - 0x28
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = a1;
  for (v1 = a0; *v1 == *v2; v1 = &v1[1]) {
    v2 = &v2[1];
  }
  v4 = v2;
  for (v3 = v1; (char)*v3 == (char)*v4; v3 = (long *)((long)v3 + 1)) {
    v4 = (long *)((long)v4 + 1);
  }
  return (long)v3 - (long)a0;
}


// Function: sub_4350 @ 0x4350
long sub_4350(char *a0,long a1)
{
  char v1;
  char *v2; // rax
  long v3; // stack - 0x20
  char *v4; // stack - 0x18
  
  v3 = 0;
  v2 = &a0[a1];
  v1 = *v2;
  *v2 = 10;
  v4 = a0;
  while( true ) {
    v4 = rawmemchr(v4,10);
    if (v4 == v2) break;
    v3 += 1;
    v4 = &v4[1];
  }
  *v2 = v1;
  return v3;
}


// Function: sub_43cf @ 0x43cf
void sub_43cf(unsigned char *a0,unsigned char a1)
{
  unsigned char *v1;
  unsigned char v2; // stack - 0x14
  unsigned char *v3; // stack - 0x10
  
  v2 = a1;
  v3 = a0;
  if (!(*(unsigned short *)((unsigned long)a1 * 2 + *(long *)__ctype_b_loc()) & 0x4000)) {
    v3 = a0;
    if ((char)a1 <= '\xff') {
      *a0 = 0x4d;
      v3 = &a0[2];
      a0[1] = 0x2d;
      v2 = a1 + 0x80;
    }
    if (0x20 <= v2) { // branch-flip
      if (v2 == 0x7f) {
        v1 = &v3[1];
        *v3 = 0x5e;
        v2 = 0x3f;
        v3 = v1;
      }
    }
    else {
      v1 = &v3[1];
      *v3 = 0x5e;
      v2 += 0x40;
      v3 = v1;
    }
  }
  *v3 = v2;
  v3[1] = 0;
}


// Function: sub_447f @ 0x447f
unsigned long sub_447f(int a0)
{
  unsigned int v1;
  unsigned long v2;
  
  if (*(char *)((long)a0 + 0x10215) != '\x01') {
    *(char *)((long)a0 + 0x10215) = 1;
    v2 = *(unsigned long *)((long)a0 * 8 + 0x10200);
    v1 = *(unsigned int *)((long)a0 * 4 + 0x100b0);
    *(unsigned long *)((long)a0 * 8 + 0x10220) = lseek(v1,v2,1);
  }
  return *(unsigned long *)((long)a0 * 8 + 0x10220);
}

