// Function: main @ 0x3cd0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char *v10;
  unsigned long v11;
  unsigned long v12; // rax
  char *v13; // rax
  char *v14; // rax
  int v15; // stack - 0x64
  int v16; // stack - 0x60
  char *v17; // stack - 0x58
  char **v18;
  unsigned int v19; // r15d
  unsigned int v2;
  char *v20; // stack - 0x80
  unsigned int v21; // stack - 0x70
  unsigned int v22; // stack - 0x5c
  unsigned long v23; // stack - 0x50
  bool v3;
  int v4;
  unsigned int v5;
  int v6; // eax
  int v7; // eax
  struct_33 *v8; // rax
  char *v9;
  
  v19 = 1;
  v18 = argv;
  v17 = (char *)sub_5630();
  v23 = 0;
  sub_e310(*v18);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_137d0(sub_6020);
  dat_1d140 = sub_68c0();
  sub_d070(getenv("DU_BLOCK_SIZE"),(unsigned int *)0x1d170,(long *)0x1d148);
  v1 = 0;
  v2 = 0x10;
  v21 = 8;
  v20 = NULL;
  v3 = 0;
label_3dc0:
  do {
    v15 = -1;
    v18 = argv;
    v4 = getopt_long(argc,argv,"0abd:chHklmst:xB:DLPSX:",(option *)0x1c6e0,&v15);
    v9 = (char *)CONCAT44(dat_4,v4);
    if (v4 == -1) {
      if (!(bool)v19)
        sub_58b0(1); // return-dupe, no-return
      if (dat_1d186) { // branch-flip
        if (v1) {
          error(0,0,dcgettext(NULL,"cannot both summarize and show all entries",5));
          sub_58b0(1);
        }
      }
      else if (v3) {
        if (v1) {
          if (dat_1d010) {
label_43b7:
            v11 = dat_1d010;
            error(0,0,dcgettext(NULL,"warning: summarizing conflicts with --max-depth=%lu",5),v11);
            sub_58b0(1); // no-return
          }
          error(0,0,dcgettext(NULL,"warning: summarizing is the same as using --max-depth=0",5));
          v9 = v13;
          if (dat_1d010) goto label_43b7;
label_45ed:
          dat_1d010 = 0;
        }
      }
      else if (v1) goto label_45ed;
      if (dat_1d16d) {
        if (dat_1d185) {
          error(0,0,dcgettext(NULL,"warning: options --apparent-size and -b are ineffective with --inodes",5));
          v9 = v14;
        }
        dat_1d148 = 1;
      }
      if (!dat_1d16c) goto label_4226;
      if (dat_1d160) {
        v10 = dat_1d160;
        goto label_4210;
      }
      v10 = getenv("TIME_STYLE");
      v9 = NULL;
      if (v10) {
        dat_1d160 = v10;
        v4 = strcmp(v10,"locale");
        v9 = (char *)CONCAT44(dat_4,v4);
        if (v4) {
          if (*v10 != '+') goto label_477a;
          v9 = strchr(v10,10);
          if (!v9) goto label_421b;
          *v9 = '\0';
          goto label_4210;
        }
      }
      v10 = "long-iso";
      dat_1d160 = "long-iso";
      goto label_4697;
    }
    if (0x87 < v4) {
label_3e26:
      v19 = 0;
      goto label_3dc0;
    }
    if (v4 <= 0x2f) {
      if (v4 == -0x83) {
        sub_11510(stdout,"du","GNU coreutils",dat_1d018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0,v18);
        exit(0); // no-return
      }
      if (v4 == -0x82)
        sub_58b0(0); // no-return
      goto label_3e26;
    }
    switch(v4) {
      case 0x30:
        dat_1d182 = 1;
        goto label_3dc0;
      default:
        goto label_3e26;
      case 0x42:
label_3fb0:
        v4 = sub_d070(optarg,(unsigned int *)0x1d170,(long *)0x1d148);
        if (v4)
          sub_12020(v4,v15,0x42,0x1c6e0,optarg); // no-return
        break;
      case 0x44:
      case 0x48:
        v2 = 0x11;
        goto label_3dc0;
      case 0x4c:
        v2 = 2;
        goto label_3dc0;
      case 0x50:
        v2 = 0x10;
        goto label_3dc0;
      case 0x53:
        dat_1d180 = 1;
        goto label_3dc0;
      case 0x58:
        v4 = sub_6fe0(sub_6b50,dat_1d140,optarg,0x10000000,10);
        if (v4) {
          v12 = sub_10680(0,3,optarg);
          v19 = 0;
          error(0,*__errno_location(),"%s",v12);
        }
        goto label_3dc0;
      case 0x61:
        dat_1d186 = '\x01';
        goto label_3dc0;
      case 0x62:
        dat_1d185 = '\x01';
        dat_1d170 = 0;
        dat_1d148 = 1;
        goto label_3dc0;
      case 99:
        dat_1d181 = '\x01';
        goto label_3dc0;
      case 100:
        v4 = sub_120a0(optarg,NULL,0,&v16,"");
        if (v4) { // branch-flip
          v12 = sub_10840(optarg);
          v19 = 0;
          error(0,0,dcgettext(NULL,"invalid maximum depth %s",5),v12);
        }
        else {
          dat_1d010 = CONCAT44(v22,v16);
          v3 = 1;
        }
        goto label_3dc0;
      case 0x68:
        dat_1d170 = 0xb0;
        dat_1d148 = 1;
        goto label_3dc0;
      case 0x6b:
        dat_1d170 = 0;
        dat_1d148 = 0x400;
        goto label_3dc0;
      case 0x6c:
        dat_1d184 = '\x01';
        goto label_3dc0;
      case 0x6d:
        dat_1d170 = 0;
        dat_1d148 = 0x100000;
        goto label_3dc0;
      case 0x73:
        v1 = 1;
        goto label_3dc0;
      case 0x74:
        v4 = sub_11b80(optarg,NULL,0,(unsigned long *)0x1d178,"kKmMGTPEZY0");
        if (v4)
          sub_12020(v4,v15,0x74,0x1c6e0,optarg); // no-return
        if ((dat_1d178) || (*optarg != '-')) goto label_3dc0;
        error(1,0,dcgettext(NULL,"invalid --threshold argument \'-0\'",5));
        goto label_3fb0;
      case 0x78:
        v21 = 0x48;
        break;
      case 0x80:
        dat_1d185 = '\x01';
        break;
      case 0x81:
        sub_6b50(dat_1d140,optarg,0x10000000);
        break;
      case 0x82:
        v20 = optarg;
        break;
      case 0x83:
        dat_1d170 = 0x90;
        dat_1d148 = 1;
        break;
      case 0x85:
        dat_1d16c = '\x01';
        v5 = 0;
        if (optarg)
          v5 = *(unsigned int *)(sub_5dc0("--time",optarg,(long *)0x1c6a0,(void *)0x15700,4,dat_1d020,1) * 4 + 0x15700);
        dat_1d168 = v5;
        dat_1d150 = sub_109c0(getenv("TZ"));
        break;
      case 0x86:
        dat_1d160 = optarg;
        break;
      case 0x87:
        dat_1d16d = '\x01';
        goto label_3dc0;
      
    }
  } while( true );
label_477a:
  while( true ) {
    v10 = dat_1d160;
    v4 = strncmp(dat_1d160,"posix-",6);
    v9 = (char *)CONCAT44(dat_4,v4);
    if (v4) break;
    dat_1d160 = &v10[6];
  }
label_4210:
  if (*v10 != '+') { // branch-flip
label_4697:
    v4 = *(int *)(sub_5dc0("time style",v10,(long *)0x1c680,(void *)0x156f0,4,dat_1d020,1,v9) * 4 + 0x156f0);
    if (v4 != 1) { // branch-flip
      if (v4 != 2) { // branch-flip
        if (!v4)
          dat_1d158 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      else {
        dat_1d158 = "%Y-%m-%d";
      }
    }
    else {
      dat_1d158 = "%Y-%m-%d %H:%M";
    }
  }
  else {
label_421b:
    dat_1d158 = &v10[1];
  }
label_4226:
  if (v20) { // branch-flip
    if (optind < argc) {
      v9 = (char *)sub_10840(argv[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v9);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_58b0(1); // no-return
    }
    if ((strcmp(v20,"-")) && (!sub_8c10(v20,"r",stdin))) {
      v12 = sub_10580(4,v20);
      v9 = dcgettext(NULL,"cannot open %s for reading",5);
      error(1,*__errno_location(),v9,v12);
      return v6;
    }
    v8 = (struct_33 *)sub_5ee0(stdin);
    dat_1d183 = (bool)v19;
  }
  else {
    v18 = &v17;
    if (optind < argc)
      v18 = &argv[optind];
    v8 = (struct_33 *)sub_5eb0(v18);
    dat_1d183 = optind + 1 < argc || v2 == 2;
  }
  if ((v8) && (dat_1d198 = sub_6240(), dat_1d198)) {
    if ((dat_1d184) || (!dat_1d183))
      v21 |= 0x100;
    while (v9 = (char *)sub_5f20(v8,&v16), v9) {
      if (v20) { // branch-flip
        if ((strcmp(v20,"-")) || (strcmp(v9,"-"))) {
          if (*v9) goto label_4341;
label_4428:
          v11 = sub_5fc0(v8);
          v9 = dcgettext(NULL,"invalid zero-length file name",5);
          error(0,0,"%s:%lu: %s",(char *)sub_10680(0,3,v20),v11,v9);
        }
        else {
          v10 = (char *)sub_10580(4,v9);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v10);
          if (!*v9) goto label_4428;
        }
        v19 = 0;
      }
      else if (*v9) { // branch-flip
label_4341:
        dat_1d110 = v9;
        v19 &= sub_5540(v21 | v2);
      }
      else {
        v19 = 0;
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
      }
    }
    if (v16 != 3) {
      if (v16 != 4) { // branch-flip
        if (v16 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
      }
      else {
        v19 = 0;
        v12 = sub_10680(0,3,v20);
        v9 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v9,v12);
      }
      sub_5fe0(v8);
      sub_62b0(dat_1d198);
      if (dat_1d190)
        sub_62b0(dat_1d190);
      if ((v20) && (((ferror_unlocked(stdin) || (v4 = sub_70c0(stdin), v4)) && ((char)v19)))) {
        v9 = (char *)sub_10580(4,v20);
        error(1,0,dcgettext(NULL,"error reading %s",5),v9);
        return v7;
      }
      if (dat_1d181)
        sub_4e30((unsigned long *)0x1d120,dcgettext(NULL,"total",5));
      return v19 ^ 1;
    }
  }
  sub_11a40(); // no-return
}


// Function: sub_4a40 @ 0x4a40
void sub_4a40(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0x8000000000000000;
  a0[3] = 0xffffffffffffffff;
}


// Function: sub_4a70 @ 0x4a70
void sub_4a70(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  *a0 = a1;
  a0[1] = 1;
  a0[2] = a2;
  a0[3] = a3;
}


// Function: sub_4a90 @ 0x4a90
void sub_4a90(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  
  v4 = *a1 + *a0;
  v1 = a1[2];
  if (CARRY8(*a1,*a0))
    v4 = 0xffffffffffffffff;
  v2 = a1[3];
  *a0 = v4;
  a0[1] = a0[1] + a1[1];
  v4 = a0[3];
  v3 = a0[2];
  if (0 <= (int)sub_10ee0(v3,v4,v1,v2))
    return;
  a0[2] = v1;
  a0[3] = v2;
}


// Function: sub_4b50 @ 0x4b50
unsigned int sub_4b50(struct_12 *a0,long a1,long a2)
{
  int v1; // eax
  
  v1 = sub_62e0(a0,a2,a1);
  if (0 <= v1)
    return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 != 0);
  sub_11a40(); // no-return
}


// Function: sub_4b80 @ 0x4b80
void sub_4b80(void) // return-dupe
{
  long v1;
  char *v2;
  long v3;
  stat v4; // stack - 0xb8
  
  v3 = sub_d8b0(0);
  if (!v3)
    return;
  while( true ) {
    if ((!(*(unsigned char *)(v3 + 0x28) & 3)) && (v2 = *(char **)(v3 + 8), !stat(v2,&v4)))
      sub_4b50(dat_1d190,v4._8_8_,v4._0_8_);
    v1 = *(long *)(v3 + 0x30);
    sub_d860(v3);
    if (!v1) break;
    v3 = v1;
  }
}


// Function: sub_4c30 @ 0x4c30
unsigned long sub_4c30(long *a0) // return-dupe
{
  long *v1;
  int v2; // eax
  
  v1 = (long *)*a0;
  if (!dat_1d190) {
    dat_1d190 = sub_6240();
    if (!dat_1d190)
      sub_11a40(); // no-return
    sub_4b80();
  }
  while( true ) {
    if (a0 == v1)
      return 0;
    v2 = sub_6330(dat_1d190,a0[0xe],a0[0xf]);
    if (1 <= v2) break;
    a0 = (long *)a0[1];
    if (!a0)
      return 0;
  }
  return 1;
}


// Function: sub_4cc0 @ 0x4cc0
void sub_4cc0(unsigned long a0)
{
  FILE *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != 0xffffffffffffffff) // branch-flip
    v2 = (char *)sub_c800(a0,v3,dat_1d170,1,dat_1d148);
  else {
    v2 = dcgettext(NULL,"Infinity",5);
  }
  fputs_unlocked(v2,v1);
}


// Function: sub_4d60 @ 0x4d60
void sub_4d60(unsigned long a0,unsigned long a1,unsigned long a2,long *a3) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  unsigned long v3; // stack - 0x88
  char v4 [64];
  char v5 [24];
  unsigned long v6; // stack - 0x80
  
  v3 = a1;
  v6 = a2;
  if (sub_10ce0(a3,&v3,v4)) {
    sub_8b90(stdout,a0,v4,a3,v6 & 0xffffffff);
    return;
  }
  v1 = (char *)sub_4d50(v3,v5);
  v2 = (char *)sub_10840(v1);
  error(0,0,dcgettext(NULL,"time %s is out of range",5),v2);
  fputs_unlocked(v1,stdout);
}


// Function: sub_4e30 @ 0x4e30
void sub_4e30(unsigned long *a0,char *a1)
{
  unsigned long v1;
  
  v1 = a0[1];
  if (!dat_1d16d)
    v1 = *a0;
  sub_4cc0(v1);
  if (dat_1d16c) {
    putchar_unlocked(9);
    sub_4d60(dat_1d158,a0[2],a0[3],dat_1d150);
  }
  __printf_chk(1,"\t%s%c",a1,-(dat_1d182 == '\0') & 10);
  fflush_unlocked(stdout); // tail-call
}


// Function: sub_4ec0 @ 0x4ec0
unsigned int sub_4ec0(void *a0,long *a1) // return-dupe
{
  unsigned long *v1;
  char *v10;
  unsigned long *v11; // rcx
  unsigned long v12; // rdx
  unsigned long v13; // rdx
  unsigned short v14;
  unsigned long v15; // stack - 0x88
  unsigned long v16; // stack - 0x68
  long v17;
  char v18 [16];
  unsigned long v19; // stack - 0x80
  unsigned long *v2;
  unsigned long v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned long v22; // stack - 0x60
  unsigned long v23; // stack - 0x58
  unsigned long v24; // stack - 0x50
  unsigned long *v3;
  char v4;
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned long v7;
  char *v8; // rax
  char *v9; // rax
  
  v14 = *(unsigned short *)&a1[0xd];
  v10 = (char *)a1[7];
  if (v14 != 4) { // branch-flip
    v5 = 1;
    if (v14 != 6) {
      v5 = sub_6a90(dat_1d140,v10);
      if ((char)v5) {
label_50e0:
        if (v14 != 1)
          return 1;
        sub_b190(a0,a1,4);
        if (a1 == (long *)sub_ab50(a0))
          return 1;
        __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
      }
      if (v14 == 0xb) {
        sub_b190(a0,a1,1);
        if (a1 != (long *)sub_ab50(a0))
          __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
        v14 = *(unsigned short *)&a1[0xd];
      }
      if (v14 == 10 || v14 == 0xd) {
        v10 = (char *)sub_10580(4,v10);
        v9 = dcgettext(NULL,"cannot access %s",5);
        error(0,(int)a1[8],v9,v10);
        return v5;
      }
      if ((((*(unsigned char *)((long)a0 + 0x48) & 0x40) && (1 <= a1[0xb])) && (*(long *)((long)a0 + 0x18) != a1[0xe])) || ((!dat_1d184 && (((dat_1d183 || (((*(unsigned int *)&a1[0x11] & 0xf000) != 0x4000 && (2 <= (unsigned long)a1[0x10])))) && (v4 = sub_4b50(dat_1d198,a1[0xf],a1[0xe]), !v4)))))) goto label_50e0;
      if (v14 == 2) {
        if (!sub_11ad0(a0,a1))
          return 1;
        v5 = sub_4c30(a1);
        if ((char)v5)
          return 1;
        v10 = (char *)sub_10680(0,3,v10);
        error(0,0,dcgettext(NULL,"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n",5),v10);
        return v5;
      }
      if (v14 != 7) { // branch-flip
        v5 = 1;
        if (v14 == 1)
          return 1;
      }
      else {
        v5 = 0;
        v9 = (char *)sub_10680(0,3,v10);
        error(0,(int)a1[8],"%s",v9);
      }
    }
    if (dat_1d168) goto label_4f13;
label_5118:
    v18 = sub_108b0(&a1[0xe]);
    v12 = SUB168(v18,8);
    v6 = SUB168(v18,0);
    if (dat_1d185) goto label_4f2e;
label_512a:
    v17 = a1[0x16] << 9;
  }
  else {
    v9 = (char *)sub_10580(4,v10);
    v8 = dcgettext(NULL,"cannot read directory %s",5);
    v5 = 0;
    error(0,(int)a1[8],v8,v9);
    if (!dat_1d168) goto label_5118;
label_4f13:
    if (dat_1d168 != 2) { // branch-flip
      v18 = sub_108a0(&a1[0xe]);
      v12 = SUB168(v18,8);
      v6 = SUB168(v18,0);
    }
    else {
      v18 = sub_10890(&a1[0xe]);
      v12 = SUB168(v18,8);
      v6 = SUB168(v18,0);
    }
    if (!dat_1d185) goto label_512a;
label_4f2e:
    v17 = a1[0x14];
    if (a1[0x14] <= -1)
      v17 = 0;
  }
  sub_4a70(&v15,v17,v6,v12);
  v17 = dat_1d100;
  v7 = a1[0xb];
  v16 = v15;
  v22 = v19;
  v23 = v20;
  v24 = v21;
  if (dat_1d108) { // branch-flip
    v4 = dat_1d180;
    if (dat_1d188 != v7) {
      if (v7 <= dat_1d188) { // branch-flip
        if (dat_1d188 - 1 != v7)
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
        v2 = (unsigned long *)(dat_1d100 + dat_1d188 * 0x40);
        sub_4a90(&v16,v2);
        v4 = dat_1d180;
        if (!dat_1d180)
          sub_4a90(&v16,&v2[4]);
        v3 = (unsigned long *)(v17 + 0x20 + v7 * 0x40);
        sub_4a90(v3,v2);
        sub_4a90(v3,&v2[4]);
      }
      else {
        if (dat_1d108 <= v7) {
          dat_1d100 = sub_11770(dat_1d100,v7,0x80);
          dat_1d108 = v7 * 2;
        }
        v4 = dat_1d180;
        v13 = dat_1d188 + 1;
        if (v13 <= v7) {
          v11 = (unsigned long *)(v13 * 0x40 + dat_1d100);
          do {
            v13 += 1;
            sub_4a40(v11);
            v1 = &v11[4];
            v11 = &v11[8];
            sub_4a40(v1);
          } while (v13 <= v7);
        }
      }
    }
  }
  else {
    dat_1d108 = v7 + 10;
    dat_1d100 = sub_11920(dat_1d108,0x40);
    v4 = dat_1d180;
  }
  dat_1d188 = v7;
  if (v4) { // branch-flip
    if ((v14 & 0xfffd) == 4) {
      sub_4a90((unsigned long *)0x1d120,&v15);
      goto label_502a;
    }
    sub_4a90((unsigned long *)(v7 * 0x40 + dat_1d100),&v15);
    sub_4a90((unsigned long *)0x1d120,&v15);
label_5021:
    if (dat_1d186) goto label_502a;
  }
  else {
    sub_4a90((unsigned long *)(v7 * 0x40 + dat_1d100),&v15);
    sub_4a90((unsigned long *)0x1d120,&v15);
    if ((v14 & 0xfffd) != 4) goto label_5021;
label_502a:
    if (v7 <= dat_1d010) goto label_503c;
  }
  if (v7)
    return v5;
label_503c:
  v7 = v22;
  if (!dat_1d16d)
    v7 = v16;
  if (0 <= (long)dat_1d178) { // branch-flip
    if (v7 < dat_1d178)
      return v5;
  }
  else if (-dat_1d178 < v7)
    return v5;
  sub_4e30(&v16,v10);
  return v5;
}


// Function: sub_58b0 @ 0x58b0
void sub_58b0(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_1d1b0;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Summarize device usage of the set of FILEs, recursively for directories.\n",5),v1);
    sub_4af0();
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
    sub_5870();
    sub_4b20();
    sub_5640();
  }
  exit(a0); // no-return
}


// Function: sub_b340 @ 0xb340
unsigned long sub_b340(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long v3; // rdx
  unsigned long v4; // rsi
  long v5; // r8
  
  v1 = SUB168(ZEXT816(a0) * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v3 = a0 % 3;
  if ((10 <= a0) && (v3)) {
    v5 = 0x10;
    v4 = 9;
    v2 = 3;
    do {
      v2 += 2;
      v4 += v5;
      v1 = a0 / v2;
      v3 = a0 % v2;
      if (a0 <= v4) break;
      v5 += 8;
    } while (v3);
  }
  v2 = v1 >> 8;
  v1 = CONCAT71((undefined7)v2,v3 != 0);
  return v1;
}


// Function: sub_b3b0 @ 0xb3b0
unsigned long sub_b3b0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_b340(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}


// Function: sub_b3f0 @ 0xb3f0
unsigned long sub_b3f0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_12dc0(a0,3) % a1;
}


// Function: sub_b410 @ 0xb410
bool sub_b410(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_b420 @ 0xb420
unsigned long sub_b420(struct_3 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x15a40)
    return 1;
  v1 = v2[2];
  if ((((dat_15a54 < v1) && (v1 < dat_15a58)) && (dat_15a5c < v2[3])) && (dat_15a70 <= *v2)) {
    v3 = *v2 + dat_15a54;
    if (((v3 < v2[1]) && (v2[1] <= dat_15a60)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x15a40;
  return 0;
}


// Function: sub_b4b0 @ 0xb4b0
void sub_b4b0(struct_5 *a0,struct_4 *a1)
{
  unsigned long v1;
  
  v1 = a0->field_0x48;
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  a0->field_0x48 = a1;
}


// Function: sub_b4d0 @ 0xb4d0
long sub_b4d0(struct_6 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}


// Function: sub_b500 @ 0xb500
long sub_b500(struct_7 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_b4d0(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = a0->field_0x38;
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = a0->field_0x38, (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_b4b0(a0);
      return v5;
    }
    v5 = *v4;
  }
  if (a3) {
    v2 = (long *)v4[1];
    if (v2) {
      v3 = v2[1];
      *v4 = *v2;
      v4[1] = v3;
      sub_b4b0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}


// Function: sub_b600 @ 0xb600
void sub_b600(struct_5 *a0)
{
  if (a0->field_0x48) {
    a0->field_0x48 = *(unsigned long *)(a0->field_0x48 + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_b630 @ 0xb630
unsigned long sub_b630(struct_10 *a0,struct_43 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_9 *v3; // rax
  struct_9 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_b658:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_9 *)sub_b4d0(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_b4b0(a0);
          v6 = v1;
          if (!v1) goto label_b6c8;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_b6c8:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_b658;
    v3 = (struct_9 *)sub_b4d0(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_9 *)sub_b600(a0);
      if (!v4)
        return 0;
      v2 = v3->field_0x8;
      v4->field_0x0 = v7;
      v4->field_0x8 = v2;
      v3->field_0x8 = (long)v4;
    }
    else {
      v3->field_0x0 = v7;
      a0->field_0x18 = a0->field_0x18 + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1->field_0x18 = a1->field_0x18 + -1;
    if (a1->field_0x8 <= v5)
      return 1;
  } while( true );
}


// Function: sub_b7f0 @ 0xb7f0
unsigned long sub_b7f0(struct_22 *a0)
{
  return a0->field_0x10;
}


// Function: sub_b800 @ 0xb800
unsigned long sub_b800(struct_42 *a0)
{
  return a0->field_0x18;
}


// Function: sub_b810 @ 0xb810
unsigned long sub_b810(struct_24 *a0)
{
  return a0->field_0x20;
}


// Function: sub_b820 @ 0xb820
unsigned long sub_b820(struct_43 *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = a0->field_0x0;
  v4 = 0;
  do {
    if (a0->field_0x8 <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if (a0->field_0x8 <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_b8f0 @ 0xb8f0
void sub_b8f0(struct_24 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_b810(a0);
  v2 = sub_b7f0();
  v3 = sub_b800();
  v4 = sub_b820();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_15a70) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_ba10 @ 0xba10
long sub_ba10(struct_7 *a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_b4d0(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = a0->field_0x38;
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_bac0 @ 0xbac0
long sub_bac0(void *a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_b4d0(a0,a1);
  v3 = v2;
  do {
    v1 = *v3;
    v3 = (long *)v3[1];
    if (v1 == a1) {
      if (v3)
        return *v3;
      break;
    }
  } while (v3);
  do {
    v2 = &v2[2];
    if (*(long **)((long)a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_bb80 @ 0xbb80
long sub_bb80(struct_43 *a0,void *a1,unsigned long a2)
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


// Function: sub_bc00 @ 0xbc00
unsigned long sub_bc00(unsigned char *a0,unsigned long a1)
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


// Function: sub_bc70 @ 0xbc70
unsigned long * sub_bc70(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_b3f0;
  if (!a3)
    a3 = sub_b410;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x15a40;
  v1[5] = a1;
  if (sub_b420(v1)) {
    v2 = sub_b760(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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


// Function: sub_bd80 @ 0xbd80
void sub_bd80(struct_45 *a0)
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
        if (a0->field_0x8 <= v5) goto label_be1b;
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
label_be1b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_be30 @ 0xbe30
void sub_be30(struct_16 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_bedc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_be9f;
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
label_be9f:
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
label_bedc:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_bf20 @ 0xbf20
unsigned int sub_bf20(struct_11 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_b760(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_b630(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_b630(a0,&v2,1)) && (sub_b630(a0,&v2,0))) {
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


// Function: sub_c0a0 @ 0xc0a0
unsigned long sub_c0a0(void *a0,long a1,long *a2) // ternary
{
  long v1;
  struct_9 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_b500(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)((long)a0 + 0x18);
    v5 = (float)*(unsigned long *)((long)a0 + 0x10);
    if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
      sub_b420(a0);
      v1 = *(long *)((long)a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_15a64 <= v5)
          return 0xffffffff;
        v4 = (dat_15a68 <= v5) ? (long)(v5 - dat_15a68) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_bf20(a0,v4))
          return 0xffffffff;
        if (sub_b500(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
      return 1;
    }
    v2 = (struct_9 *)sub_b600(a0);
    if (!v2)
      return 0xffffffff;
    v2->field_0x0 = a1;
    v2->field_0x8 = v3[1];
    v3[1] = (long)v2;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_c290 @ 0xc290
unsigned long sub_c290(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_c0a0(a0,a1,&v3);
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


// Function: sub_c2f0 @ 0xc2f0
long sub_c2f0(void *a0,long a1) // ternary x2
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
  
  v5 = sub_b500(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)((long)a0 + 0x18) - 1;
    *(unsigned long *)((long)a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)((long)a0 + 0x10) : *(unsigned long *)((long)a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)((long)a0 + 0x28) * v8) {
      sub_b420(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_15a68 <= v8) ? (long)(v8 - dat_15a68) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_bf20(a0,v6)) {
          v4 = *(void **)((long)a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)((long)a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

