// Function: sub_3cd0 @ 0x3cd0
unsigned long sub_3cd0(int a0,char **a1)
{
  bool v1;
  char *v10;
  char *v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  int v14; // stack - 0x64
  int v15; // stack - 0x60
  char *v16; // stack - 0x58
  char **v17;
  unsigned int v18; // r15d
  char *v19; // stack - 0x80
  unsigned int v2;
  unsigned int v20; // stack - 0x70
  unsigned int v21; // stack - 0x5c
  unsigned long v22; // stack - 0x50
  bool v3;
  int v4;
  unsigned int v5;
  long v6;
  char *v7;
  unsigned long v8; // rax
  char *v9; // rax
  
  v18 = 1;
  v17 = a1;
  v16 = (char *)sub_5630();
  v22 = 0;
  sub_e310(*v17);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_137d0(sub_6020);
  dat_1d140 = sub_68c0();
  sub_d070(getenv("DU_BLOCK_SIZE"),0x1d170,0x1d148);
  v1 = 0;
  v2 = 0x10;
  v20 = 8;
  v19 = NULL;
  v3 = 0;
label_3dc0:
  do {
    v14 = -1;
    v17 = a1;
    v4 = getopt_long(a0,a1,"0abd:chHklmst:xB:DLPSX:",(void *)0x1c6e0,&v14);
    v7 = (char *)CONCAT44(dat_4,v4);
    if (v4 == -1) {
      if (!SUB41(v18,0))
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
            v6 = dat_1d010;
            error(0,0,dcgettext(NULL,"warning: summarizing conflicts with --max-depth=%lu",5),v6);
            sub_58b0(1); // no-return
          }
          error(0,0,dcgettext(NULL,"warning: summarizing is the same as using --max-depth=0",5));
          v7 = v9;
          if (dat_1d010) goto label_43b7;
label_45ed:
          dat_1d010 = 0;
        }
      }
      else if (v1) goto label_45ed;
      if (dat_1d16d) {
        if (dat_1d185) {
          error(0,0,dcgettext(NULL,"warning: options --apparent-size and -b are ineffective with --inodes",5));
          v7 = v11;
        }
        dat_1d148 = 1;
      }
      if (!dat_1d16c) goto label_4226;
      if (dat_1d160) {
        v10 = dat_1d160;
        goto label_4210;
      }
      v10 = getenv("TIME_STYLE");
      v7 = NULL;
      if (v10) {
        dat_1d160 = v10;
        v4 = strcmp(v10,"locale");
        v7 = (char *)CONCAT44(dat_4,v4);
        if (v4) {
          if (*v10 != '+') goto label_477a;
          v7 = strchr(v10,10);
          if (!v7) goto label_421b;
          *v7 = '\0';
          goto label_4210;
        }
      }
      v10 = "long-iso";
      dat_1d160 = "long-iso";
      goto label_4697;
    }
    if (0x87 < v4) {
label_3e26:
      v18 = 0;
      goto label_3dc0;
    }
    if (v4 <= 0x2f) {
      if (v4 == -0x83) {
        sub_11510(stdout,"du","GNU coreutils",dat_1d018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0,v17);
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
        v4 = sub_d070(optarg,0x1d170,0x1d148);
        if (v4)
          sub_12020(v4,v14,0x42,0x1c6e0,optarg); // no-return
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
          v8 = sub_10680(0,3,optarg);
          v18 = 0;
          error(0,*__errno_location(),"%s",v8);
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
        v4 = sub_120a0(optarg,0,0,&v15,"");
        if (v4) { // branch-flip
          v8 = sub_10840(optarg);
          v18 = 0;
          error(0,0,dcgettext(NULL,"invalid maximum depth %s",5),v8);
        }
        else {
          dat_1d010 = CONCAT44(v21,v15);
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
        v4 = sub_11b80(optarg,0,0,0x1d178,"kKmMGTPEZY0");
        if (v4)
          sub_12020(v4,v14,0x74,0x1c6e0,optarg); // no-return
        if ((dat_1d178) || (*optarg != '-')) goto label_3dc0;
        error(1,0,dcgettext(NULL,"invalid --threshold argument \'-0\'",5));
        goto label_3fb0;
      case 0x78:
        v20 = 0x48;
        break;
      case 0x80:
        dat_1d185 = '\x01';
        break;
      case 0x81:
        sub_6b50(dat_1d140,optarg,0x10000000);
        break;
      case 0x82:
        v19 = optarg;
        break;
      case 0x83:
        dat_1d170 = 0x90;
        dat_1d148 = 1;
        break;
      case 0x85:
        dat_1d16c = '\x01';
        v5 = 0;
        if (optarg) {
          v6 = sub_5dc0("--time",optarg,0x1c6a0,0x15700,4,dat_1d020,1);
          v5 = *(unsigned int *)(v6 * 4 + 0x15700);
        }
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
    v7 = (char *)CONCAT44(dat_4,v4);
    if (v4) break;
    dat_1d160 = &v10[6];
  }
label_4210:
  if (*v10 != '+') { // branch-flip
label_4697:
    v6 = sub_5dc0("time style",v10,0x1c680,0x156f0,4,dat_1d020,1,v7);
    v4 = *(int *)(v6 * 4 + 0x156f0);
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
  if (v19) { // branch-flip
    if (optind < a0) {
      v8 = sub_10840(a1[optind]);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v8);
      __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
      sub_58b0(1); // no-return
    }
    if ((strcmp(v19,"-")) && (v6 = sub_8c10(v19,"r",stdin), !v6)) {
      v8 = sub_10580(4,v19);
      v7 = dcgettext(NULL,"cannot open %s for reading",5);
      error(1,*__errno_location(),v7,v8);
      return v12;
    }
    v6 = sub_5ee0(stdin);
    dat_1d183 = SUB41(v18,0);
  }
  else {
    v17 = &v16;
    if (optind < a0)
      v17 = &a1[optind];
    v6 = sub_5eb0(v17);
    dat_1d183 = optind + 1 < a0 || v2 == 2;
  }
  if ((v6) && (dat_1d198 = sub_6240(), dat_1d198)) {
    if ((dat_1d184) || (!dat_1d183))
      v20 |= 0x100;
    while (v7 = (char *)sub_5f20(v6,&v15), v7) {
      if (v19) { // branch-flip
        if ((strcmp(v19,"-")) || (strcmp(v7,"-"))) {
          if (*v7) goto label_4341;
label_4428:
          v8 = sub_5fc0(v6);
          v7 = dcgettext(NULL,"invalid zero-length file name",5);
          error(0,0,"%s:%lu: %s",sub_10680(0,3,v19),v8,v7);
        }
        else {
          v8 = sub_10580(4,v7);
          error(0,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v8);
          if (!*v7) goto label_4428;
        }
        v18 = 0;
      }
      else if (*v7) { // branch-flip
label_4341:
        dat_1d110 = v7;
        v18 &= sub_5540(v20 | v2);
      }
      else {
        v18 = 0;
        error(0,0,"%s",dcgettext(NULL,"invalid zero-length file name",5));
      }
    }
    if (v15 != 3) {
      if (v15 != 4) { // branch-flip
        if (v15 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
      }
      else {
        v18 = 0;
        v8 = sub_10680(0,3,v19);
        v7 = dcgettext(NULL,"%s: read error",5);
        error(0,*__errno_location(),v7,v8);
      }
      sub_5fe0(v6);
      sub_62b0(dat_1d198);
      if (dat_1d190)
        sub_62b0(dat_1d190);
      if ((v19) && (((v4 = ferror_unlocked(stdin), v4 || (v4 = sub_70c0(stdin), v4)) && ((char)v18)))) {
        v8 = sub_10580(4,v19);
        error(1,0,dcgettext(NULL,"error reading %s",5),v8);
        return v13;
      }
      if (dat_1d181)
        sub_4e30(0x1d120,dcgettext(NULL,"total",5));
      return (unsigned long)(v18 ^ 1);
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
unsigned int sub_4b50(unsigned long a0,unsigned long a1,unsigned long a2)
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
  unsigned long v4; // stack - 0xb8
  unsigned long v5; // stack - 0xb0
  
  v3 = sub_d8b0(0);
  if (!v3)
    return;
  while( true ) {
    if ((!(*(unsigned char *)(v3 + 0x28) & 3)) && (v2 = *(char **)(v3 + 8), !stat(v2,&v4)))
      sub_4b50(dat_1d190,v5,v4);
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
void sub_4cc0(long a0)
{
  void *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != -1) // branch-flip
    v2 = (char *)sub_c800(a0,v3,dat_1d170,1,dat_1d148);
  else {
    v2 = dcgettext(NULL,"Infinity",5);
  }
  fputs_unlocked(v2,v1);
}


// Function: sub_4d60 @ 0x4d60
void sub_4d60(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3) // return-dupe
{
  char *v1; // rax
  unsigned long v2; // rax
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
  v2 = sub_10840(v1);
  error(0,0,dcgettext(NULL,"time %s is out of range",5),v2);
  fputs_unlocked(v1,stdout);
}


// Function: sub_4e30 @ 0x4e30
void sub_4e30(unsigned long *a0,unsigned long a1)
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
  __printf_chk(1,"\t%s%c",a1,(unsigned long)(-(unsigned int)(dat_1d182 == '\0') & 10));
  fflush_unlocked(stdout); // tail-call
}


// Function: sub_4ec0 @ 0x4ec0
unsigned int sub_4ec0(long a0,long a1) // return-dupe
{
  long v1;
  unsigned long v10; // rdx
  unsigned short v11;
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x68
  char v14 [16];
  unsigned long v15; // stack - 0x80
  unsigned long v16; // stack - 0x78
  unsigned long v17; // stack - 0x70
  unsigned long v18; // stack - 0x60
  unsigned long v19; // stack - 0x58
  char v2;
  unsigned long v20; // stack - 0x50
  unsigned int v3; // eax
  unsigned long v4;
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7;
  long v8;
  unsigned long v9; // rdx
  
  v11 = *(unsigned short *)(a1 + 0x68);
  v7 = *(unsigned long *)(a1 + 0x38);
  if (v11 != 4) { // branch-flip
    v3 = 1;
    if (v11 != 6) {
      v3 = sub_6a90(dat_1d140,v7);
      if ((char)v3) {
label_50e0:
        if (v11 != 1)
          return 1;
        sub_b190(a0,a1,4);
        if (a1 == sub_ab50(a0))
          return 1;
        __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
      }
      if (v11 == 0xb) {
        sub_b190(a0,a1,1);
        if (a1 != sub_ab50(a0))
          __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
        v11 = *(unsigned short *)(a1 + 0x68);
      }
      if (v11 == 10 || v11 == 0xd) {
        v7 = sub_10580(4,v7);
        v5 = dcgettext(NULL,"cannot access %s",5);
        error(0,*(int *)(a1 + 0x40),v5,v7);
        return v3;
      }
      if ((((*(unsigned char *)(a0 + 0x48) & 0x40) && (1 <= *(long *)(a1 + 0x58))) && (*(long *)(a0 + 0x18) != *(long *)(a1 + 0x70))) || ((!dat_1d184 && (((dat_1d183 || (((*(unsigned int *)(a1 + 0x88) & 0xf000) != 0x4000 && (2 <= *(unsigned long *)(a1 + 0x80))))) && (v2 = sub_4b50(dat_1d198,*(unsigned long *)(a1 + 0x78),*(unsigned long *)(a1 + 0x70)), !v2)))))) goto label_50e0;
      if (v11 == 2) {
        if (!sub_11ad0(a0,a1))
          return 1;
        v3 = sub_4c30(a1);
        if ((char)v3)
          return 1;
        v7 = sub_10680(0,3,v7);
        error(0,0,dcgettext(NULL,"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n",5),v7);
        return v3;
      }
      if (v11 != 7) { // branch-flip
        v3 = 1;
        if (v11 == 1)
          return 1;
      }
      else {
        v3 = 0;
        v6 = sub_10680(0,3,v7);
        error(0,*(int *)(a1 + 0x40),"%s",v6);
      }
    }
    if (dat_1d168) goto label_4f13;
label_5118:
    v14 = sub_108b0(a1 + 0x70);
    v9 = SUB168(v14,8);
    v6 = SUB168(v14,0);
    if (dat_1d185) goto label_4f2e;
label_512a:
    v8 = *(long *)(a1 + 0xb0) << 9;
  }
  else {
    v6 = sub_10580(4,v7);
    v5 = dcgettext(NULL,"cannot read directory %s",5);
    v3 = 0;
    error(0,*(int *)(a1 + 0x40),v5,v6);
    if (!dat_1d168) goto label_5118;
label_4f13:
    if (dat_1d168 != 2) { // branch-flip
      v14 = sub_108a0(a1 + 0x70);
      v9 = SUB168(v14,8);
      v6 = SUB168(v14,0);
    }
    else {
      v14 = sub_10890(a1 + 0x70);
      v9 = SUB168(v14,8);
      v6 = SUB168(v14,0);
    }
    if (!dat_1d185) goto label_512a;
label_4f2e:
    v8 = *(long *)(a1 + 0xa0);
    if (*(long *)(a1 + 0xa0) <= -1)
      v8 = 0;
  }
  sub_4a70(&v12,v8,v6,v9);
  v8 = dat_1d100;
  v4 = *(unsigned long *)(a1 + 0x58);
  v13 = v12;
  v18 = v15;
  v19 = v16;
  v20 = v17;
  if (dat_1d108) { // branch-flip
    v2 = dat_1d180;
    if (dat_1d188 != v4) {
      if (v4 <= dat_1d188) { // branch-flip
        if (dat_1d188 - 1 != v4)
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
        v1 = dat_1d100 + dat_1d188 * 0x40;
        sub_4a90(&v13,v1);
        v2 = dat_1d180;
        if (!dat_1d180)
          sub_4a90(&v13,v1 + 0x20);
        v8 = v8 + 0x20 + v4 * 0x40;
        sub_4a90(v8,v1);
        sub_4a90(v8,v1 + 0x20);
      }
      else {
        if (dat_1d108 <= v4) {
          dat_1d100 = sub_11770(dat_1d100,v4,0x80);
          dat_1d108 = v4 * 2;
        }
        v2 = dat_1d180;
        v10 = dat_1d188 + 1;
        if (v10 <= v4) {
          v8 = v10 * 0x40 + dat_1d100;
          do {
            v10 += 1;
            sub_4a40(v8);
            v1 = v8 + 0x20;
            v8 += 0x40;
            sub_4a40(v1);
          } while (v10 <= v4);
        }
      }
    }
  }
  else {
    dat_1d108 = v4 + 10;
    dat_1d100 = sub_11920(dat_1d108,0x40);
    v2 = dat_1d180;
  }
  dat_1d188 = v4;
  if (v2) { // branch-flip
    if ((v11 & 0xfffd) == 4) {
      sub_4a90(0x1d120,&v12);
      goto label_502a;
    }
    sub_4a90(v4 * 0x40 + dat_1d100,&v12);
    sub_4a90(0x1d120,&v12);
label_5021:
    if (dat_1d186) goto label_502a;
  }
  else {
    sub_4a90(v4 * 0x40 + dat_1d100,&v12);
    sub_4a90(0x1d120,&v12);
    if ((v11 & 0xfffd) != 4) goto label_5021;
label_502a:
    if (v4 <= dat_1d010) goto label_503c;
  }
  if (v4)
    return v3;
label_503c:
  v4 = v18;
  if (!dat_1d16d)
    v4 = v13;
  if (0 <= (long)dat_1d178) { // branch-flip
    if (v4 < dat_1d178)
      return v3;
  }
  else if (-dat_1d178 < v4)
    return v3;
  sub_4e30(&v13,v7);
  return v3;
}


// Function: sub_58b0 @ 0x58b0
void sub_58b0(int a0)
{
  void *v1;
  unsigned long v2;
  
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
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5; // rsi
  long v6; // r8
  
  v1._8_8_ = 0;
  v1._0_8_ = a0;
  v2 = SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v4 = a0 % 3;
  if ((10 <= a0) && (v4)) {
    v6 = 0x10;
    v5 = 9;
    v3 = 3;
    do {
      v3 += 2;
      v5 += v6;
      v2 = a0 / v3;
      v4 = a0 % v3;
      if (a0 <= v5) break;
      v6 += 8;
    } while (v4);
  }
  v3 = v2 >> 8;
  v2 = CONCAT71((undefined7)v3,v4 != 0);
  return v2;
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
unsigned long sub_b420(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x15a40)
    return 1;
  v1 = v2[2];
  if ((((dat_15a54 < v1) && (v1 < dat_15a58)) && (dat_15a5c < v2[3])) && (dat_15a70 <= *v2)) {
    v3 = *v2 + dat_15a54;
    if (((v3 < v2[1]) && (v2[1] <= dat_15a60)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x15a40;
  return 0;
}


// Function: sub_b4b0 @ 0xb4b0
void sub_b4b0(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_b4d0 @ 0xb4d0
long sub_b4d0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}


// Function: sub_b500 @ 0xb500
long sub_b500(long a0,long a1,unsigned long *a2,char a3) // early-return
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
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
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
void sub_b600(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_b630 @ 0xb630
unsigned long sub_b630(long a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  long *v2; // rax
  long *v3;
  long *v4;
  long *v5;
  long v6;
  
  v5 = (long *)*a1;
  if ((long *)a1[1] <= v5)
    return 1;
  do {
    while (v6 = *v5, !v6) {
label_b658:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_b4d0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_b4b0(a0);
          v3 = v4;
          if (!v4) goto label_b6c8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_b6c8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_b658;
    v3 = (long *)sub_b4d0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_b600(a0);
      if (!v4)
        return 0;
      v1 = v3[1];
      *v4 = v6;
      v4[1] = v1;
      v3[1] = (long)v4;
    }
    else {
      *v3 = v6;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1[3] = a1[3] + -1;
    if ((long *)a1[1] <= v5)
      return 1;
  } while( true );
}


// Function: sub_b7f0 @ 0xb7f0
unsigned long sub_b7f0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_b800 @ 0xb800
unsigned long sub_b800(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_b810 @ 0xb810
unsigned long sub_b810(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_b820 @ 0xb820
unsigned long sub_b820(unsigned long *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = (long *)*a0;
  v4 = 0;
  do {
    if ((long *)a0[1] <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if ((long *)a0[1] <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_b8f0 @ 0xb8f0
void sub_b8f0(unsigned long a0,void *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_ba10 @ 0xba10
long sub_ba10(long a0,long a1)
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
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_bac0 @ 0xbac0
long sub_bac0(long a0,long a1)
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
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_bb80 @ 0xbb80
long sub_bb80(unsigned long *a0,void *a1,unsigned long a2)
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
unsigned long * sub_bc70(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = 0x15a40;
  v1[5] = a1;
  if (sub_b420(v1)) {
    v2 = sub_b760(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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
void sub_bd80(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_be1b;
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
label_be1b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_be30 @ 0xbe30
void sub_be30(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_bedc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_be9f;
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
label_be9f:
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
label_bedc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_bf20 @ 0xbf20
unsigned int sub_bf20(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_b760(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_b630(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_b630(a0,&v2,1)) && (sub_b630(a0,&v2,0))) {
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


// Function: sub_c0a0 @ 0xc0a0
unsigned long sub_c0a0(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
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
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_b420(a0);
      v1 = *(long *)(a0 + 0x28);
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
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_b600(a0);
    if (!v2)
      return 0xffffffff;
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_c290 @ 0xc290
unsigned long sub_c290(unsigned long a0,unsigned long a1) // early-return
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
long sub_c2f0(long a0,unsigned long a1) // ternary x2
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_b420(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_15a68 <= v8) ? (long)(v8 - dat_15a68) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_bf20(a0,v6)) {
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

