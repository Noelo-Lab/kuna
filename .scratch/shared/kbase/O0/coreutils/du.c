// Function: sub_40da @ 0x40da
void sub_40da(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0x8000000000000000;
  a0[3] = 0xffffffffffffffff;
}


// Function: sub_411a @ 0x411a
void sub_411a(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  *a0 = a1;
  a0[1] = 1;
  a0[2] = a2;
  a0[3] = a3;
}


// Function: sub_415c @ 0x415c
void sub_415c(unsigned long *a0,long *a1) // return-dupe
{
  long v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  
  v4 = *a1 + *a0;
  if (v4 < *a0)
    v4 = 0xffffffffffffffff;
  *a0 = v4;
  a0[1] = a0[1] + a1[1];
  v1 = a1[3];
  v2 = a1[2];
  v4 = a0[2];
  v3 = a0[3];
  if (0 > (int)sub_1489d(v4,v3,v2,v1)) {
    v4 = a1[3];
    a0[2] = a1[2];
    a0[3] = v4;
    return;
  }
}


// Function: sub_41ff @ 0x41ff
void sub_41ff(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_211d0;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Summarize device usage of the set of FILEs, recursively for directories.\n"),v1);
    sub_3d79();
    v1 = stdout;
    fputs_unlocked(gettext("  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_3ddb("DU");
    sub_3daa();
    sub_3e10("du");
  }
  exit(a0); // no-return
}


// Function: sub_43fb @ 0x43fb
unsigned long sub_43fb(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = sub_6bcf(a0,a2,a1);
  if ((int)v1 <= -1)
    sub_15992(); // no-return
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 != 0);
}


// Function: sub_4441 @ 0x4441
void sub_4441(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3) // return-dupe
{
  unsigned long v1; // rax
  char v2 [64];
  unsigned long v3; // stack - 0xa8
  char v4 [24];
  unsigned long v5; // stack - 0xa0
  unsigned long v6; // stack - 0x98
  unsigned long v7; // stack - 0x90
  char *v8; // stack - 0x80
  
  v3 = a1;
  v5 = a2;
  v6 = a3;
  v7 = a0;
  if (sub_14623(a3,&v3,v2)) {
    sub_80da(stdout,v7,v2,v6,v5 & 0xffffffff);
    return;
  }
  v8 = (char *)sub_40a7(v3,v4);
  v1 = sub_1401b(v8);
  error(0,0,gettext("time %s is out of range"),v1);
  fputs_unlocked(v8,stdout);
}


// Function: sub_4559 @ 0x4559
void sub_4559(long a0)
{
  void *v1;
  char *v2; // rax
  char v3 [664];
  
  v1 = stdout;
  if (a0 != -1) // branch-flip
    v2 = (char *)sub_f661(a0,v3,dat_21148,1,dat_21170);
  else {
    v2 = gettext("Infinity");
  }
  fputs_unlocked(v2,v1);
}


// Function: sub_45f2 @ 0x45f2
void sub_45f2(unsigned long *a0,unsigned long a1) // ternary x2
{
  unsigned long v1;
  
  v1 = (dat_2114c) ? a0[1] : *a0; // branch-flip
  sub_4559(v1);
  if (dat_2114d) {
    putchar_unlocked(9);
    sub_4441(dat_21160,a0[2],a0[3],dat_21168);
  }
  v1 = (dat_2113c) ? 0 : 10; // branch-flip
  printf("\t%s%c",a1,v1);
  fflush_unlocked(stdout);
}


// Function: sub_46a8 @ 0x46a8
void sub_46a8(void)
{
  char *v1;
  long v2;
  unsigned long v3; // stack - 0xa8
  long v4; // stack - 0xb8
  unsigned long v5; // stack - 0xa0
  
  v4 = sub_11107(0);
  while (v4) {
    if (((!(*(unsigned char *)(v4 + 0x28) & 2)) && (!(*(unsigned char *)(v4 + 0x28) & 1))) && (v1 = *(char **)(v4 + 8), !stat(v1,&v3)))
      sub_43fb(dat_21128,v5,v3);
    v2 = *(long *)(v4 + 0x30);
    sub_11d9d(v4);
    v4 = v2;
  }
}


// Function: sub_4795 @ 0x4795
unsigned long sub_4795(long *a0)
{
  long *v1;
  int v2; // eax
  long *v3; // stack - 0x20
  
  v1 = (long *)*a0;
  v3 = a0;
  if (!dat_21128) {
    dat_21128 = sub_6936();
    if (!dat_21128)
      sub_15992(); // no-return
    sub_46a8();
  }
  for (; (v3 && (v3 != v1)); v3 = (long *)v3[1]) {
    v2 = sub_6c4b(dat_21128,v3[0xe],v3[0xf]);
    if (1 <= v2)
      return 1;
  }
  return 0;
}


// Function: sub_4831 @ 0x4831
char sub_4831(long a0,long a1)
{
  unsigned short v1;
  unsigned long v10; // stack - 0x68
  unsigned long v11; // stack - 0x48
  bool v12;
  char v13 [16];
  char v14; // stack - 0xa6
  char v15; // stack - 0xa5
  unsigned long v16; // stack - 0xa0
  unsigned long v17; // stack - 0x60
  unsigned long v18; // stack - 0x58
  unsigned long v19; // stack - 0x50
  char v2; // al
  unsigned long v20; // stack - 0x40
  unsigned long v21; // stack - 0x38
  unsigned long v22; // stack - 0x30
  long *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6;
  unsigned long v7;
  long v8;
  unsigned long v9; // rdx
  
  v14 = 1;
  v6 = *(unsigned long *)(a1 + 0x38);
  v3 = (long *)(a1 + 0x70);
  v1 = *(unsigned short *)(a1 + 0x68);
  if (v1 != 4) { // branch-flip
    if (v1 != 6) {
      v15 = sub_76f0(dat_21178,v6);
      if (v15 != '\x01') {
        if (v1 == 0xb) {
          sub_c3cb(a0,a1,1);
          if (sub_bd55(a0) != a1)
            __assert_fail("e == ent","src/du.c",0x20f,"process_file"); // no-return
          v1 = *(unsigned short *)(a1 + 0x68);
        }
        if ((v1 == 10) || (v1 == 0xd)) {
          v6 = sub_13bd4(4,v6);
          v5 = gettext("cannot access %s");
          error(0,*(int *)(a1 + 0x40),v5,v6);
          return 0;
        }
        if (((*(unsigned int *)(a0 + 0x48) & 0x40) && (1 <= *(long *)(a1 + 0x58))) && (*(long *)(a0 + 0x18) != *v3))
          v15 = '\x01';
      }
      if ((v15) || (((dat_2113a != '\x01' && ((dat_2113b || (((*(unsigned int *)(a1 + 0x88) & 0xf000) != 0x4000 && (2 <= *(unsigned long *)(a1 + 0x80))))))) && (v2 = sub_43fb(dat_21120,*(unsigned long *)(a1 + 0x78),*v3), v2 != '\x01')))) {
        if (v1 == 1) {
          sub_c3cb(a0,a1,4);
          if (sub_bd55(a0) != a1)
            __assert_fail("e == ent","src/du.c",0x230,"process_file"); // no-return
        }
        return 1;
      }
      if (v1 != 7) { // branch-flip
        if (v1 <= 7) {
          if (v1 == 1)
            return 1;
          if (v1 == 2) {
            if ((sub_15a4d(a0,a1)) && (sub_4795(a1) != '\x01')) {
              v6 = sub_13d66(0,3,v6);
              error(0,0,gettext("WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"),v6);
              return 0;
            }
            return 1;
          }
        }
      }
      else {
        v4 = sub_13d66(0,3,v6);
        error(0,*(int *)(a1 + 0x40),"%s",v4);
        v14 = 0;
      }
    }
  }
  else {
    v4 = sub_13bd4(4,v6);
    v5 = gettext("cannot read directory %s");
    error(0,*(int *)(a1 + 0x40),v5,v4);
    v14 = 0;
  }
  if (dat_21150) { // branch-flip
    if (dat_21150 != 2) { // branch-flip
      v13 = sub_140ad(v3);
      v9 = SUB168(v13,8);
      v4 = SUB168(v13,0);
    }
    else {
      v13 = sub_14093(v3);
      v9 = SUB168(v13,8);
      v4 = SUB168(v13,0);
    }
  }
  else {
    v13 = sub_140c7(v3);
    v9 = SUB168(v13,8);
    v4 = SUB168(v13,0);
  }
  if (dat_21139) { // branch-flip
    v8 = *(long *)(a1 + 0xa0);
    if (v8 <= -1)
      v8 = 0;
  }
  else {
    v8 = *(long *)(a1 + 0xb0) << 9;
  }
  sub_411a(&v10,v8,v4,v9);
  v7 = *(unsigned long *)(a1 + 0x58);
  v11 = v10;
  v20 = v17;
  v21 = v18;
  v22 = v19;
  if (dat_211a0) { // branch-flip
    if (v7 != dat_21130) {
      if (v7 <= dat_21130) { // branch-flip
        if (v7 != dat_21130 - 1)
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file"); // no-return
        sub_415c(&v11,dat_21130 * 0x40 + dat_211a8);
        if (dat_2113e != '\x01')
          sub_415c(&v11,dat_21130 * 0x40 + dat_211a8 + 0x20);
        sub_415c(v7 * 0x40 + dat_211a8 + 0x20,dat_21130 * 0x40 + dat_211a8);
        sub_415c(v7 * 0x40 + dat_211a8 + 0x20,dat_21130 * 0x40 + dat_211a8 + 0x20);
      }
      else {
        v16 = dat_21130;
        if (dat_211a0 <= v7) {
          dat_211a8 = sub_15307(dat_211a8,v7,0x80);
          dat_211a0 = v7 * 2;
          v16 = dat_21130;
        }
        while (v16 = v16 + 1, v16 <= v7) {
          sub_40da(v16 * 0x40 + dat_211a8);
          sub_40da(v16 * 0x40 + dat_211a8 + 0x20);
        }
      }
    }
  }
  else {
    dat_211a0 = v7 + 10;
    dat_211a8 = sub_15838(dat_211a0,0x40);
  }
  dat_21130 = v7;
  if ((dat_2113e != '\x01') || ((v1 != 6 && (v1 != 4))))
    sub_415c(v7 * 0x40 + dat_211a8,&v10);
  sub_415c(0x21180,&v10);
  if (((((v1 == 6) || (v1 == 4)) && (v7 <= dat_21010)) || ((dat_21138 && (v7 <= dat_21010)))) || (!v7)) {
    if (dat_2114c) // branch-flip
      v7 = v20;
    else {
      v7 = v11;
    }
    if (0 <= (long)dat_21140) // branch-flip
      v12 = dat_21140 <= v7;
    else {
      v12 = v7 <= -dat_21140;
    }
    if (v12)
      sub_45f2(&v11,v6);
  }
  return v14;
}


// Function: sub_505b @ 0x505b
bool sub_505b(long *a0,unsigned int a1)
{
  long v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  bool v5; // stack - 0x29
  
  v5 = 1;
  if (*a0) {
    v1 = sub_159d2(a0,a1,0);
    while( true ) {
      v2 = sub_bd55(v1);
      if (!v2) break;
      v5 = (sub_4831(v1,v2) & v5) != 0;
    }
    if (*__errno_location()) {
      v3 = sub_13d66(0,3,*(unsigned long *)(v1 + 0x20));
      v4 = gettext("fts_read failed: %s");
      error(0,*__errno_location(),v4,v3);
      v5 = 0;
    }
    dat_21130 = 0;
    if (sub_b89b(v1)) {
      v4 = gettext("fts_close failed");
      error(0,*__errno_location(),v4);
      v5 = 0;
    }
  }
  return v5;
}


// Function: sub_5197 @ 0x5197
unsigned long sub_5197(int a0,char **a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_5197
{
  unsigned int v1;
  unsigned long v10; // rax
  int v11; // stack - 0x90
  unsigned int v12; // stack - 0x78
  char *v13; // stack - 0x38
  bool v14; // stack - 0x93
  unsigned int v15; // stack - 0x8c
  unsigned int v16; // stack - 0x88
  int v17; // stack - 0x84
  int v18; // stack - 0x80
  int v19; // stack - 0x7c
  bool v2;
  unsigned int v20; // stack - 0x74
  char *v21; // stack - 0x70
  long v22; // stack - 0x68
  long v23; // stack - 0x60
  char *v24; // stack - 0x58
  char **v25; // stack - 0x50
  char *v26; // stack - 0x48
  unsigned long v27; // stack - 0x40
  unsigned long v28; // stack - 0x30
  bool v3;
  unsigned char v4; // al
  int v5; // eax
  unsigned long v6; // rax
  long v7; // rax
  char *v8; // rax
  unsigned long v9; // rax
  
  v3 = 0;
  v14 = 1;
  v21 = NULL;
  v15 = 8;
  v16 = 0x10;
  v2 = 0;
  v13 = (char *)sub_40cc(".");
  v28 = 0;
  sub_11fda(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_186c0(sub_6762);
  dat_21178 = sub_6ebe();
  sub_102cd(getenv("DU_BLOCK_SIZE"),0x21148,0x21170);
label_52a5:
  v11 = -1;
  v17 = getopt_long(a0,a1,"0abd:chHklmst:xB:DLPSX:",(void *)0x20680,&v11);
  if (v17 != -1) {
    if (0x87 < v17) {
label_57be:
      v14 = 0;
      goto label_52a5;
    }
    if (v17 <= 0x2f) {
      if (v17 == -0x83) {
        sub_1517a(stdout,"du","GNU coreutils",dat_21018,"Torbjorn Granlund","David MacKenzie","Paul Eggert","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v17 == -0x82)
        sub_41ff(0); // no-return
      goto label_57be;
    }
    switch(v17) {
      case 0x30:
        dat_2113c = 1;
        break;
      default:
        goto label_57be;
      case 0x42:
        v19 = sub_102cd(optarg,0x21148,0x21170);
        if (v19)
          sub_160c2(v19,v11,(int)(char)v17,0x20680,optarg); // no-return
        v19 = 0;
        break;
      case 0x44:
      case 0x48:
        v16 = 0x11;
        break;
      case 0x4c:
        v16 = 2;
        break;
      case 0x50:
        v16 = 0x10;
        break;
      case 0x53:
        dat_2113e = 1;
        break;
      case 0x58:
        v5 = sub_7dac(sub_77e3,dat_21178,optarg,0x10000000,10);
        if (v5) {
          v6 = sub_13d66(0,3,optarg);
          error(0,*__errno_location(),"%s",v6);
          v14 = 0;
        }
        break;
      case 0x61:
        dat_21138 = '\x01';
        break;
      case 0x62:
        dat_21139 = '\x01';
        dat_21148 = 0;
        dat_21170 = 1;
        break;
      case 99:
        dat_2113d = '\x01';
        break;
      case 100:
        v5 = sub_161e3(optarg,0,0,&v12,0x1934b);
        if (v5) { // branch-flip
          v6 = sub_1401b(optarg);
          error(0,0,gettext("invalid maximum depth %s"),v6);
          v14 = 0;
        }
        else {
          v3 = 1;
          dat_21010 = CONCAT44(v20,v12);
        }
        break;
      case 0x68:
        dat_21148 = 0xb0;
        dat_21170 = 1;
        break;
      case 0x6b:
        dat_21148 = 0;
        dat_21170 = 0x400;
        break;
      case 0x6c:
        dat_2113a = '\x01';
        break;
      case 0x6d:
        dat_21148 = 0;
        dat_21170 = 0x100000;
        break;
      case 0x73:
        v2 = 1;
        break;
      case 0x74:
        v18 = sub_15b9a(optarg,0,0,0x21140,"kKmMGTPEZY0");
        if (v18)
          sub_160c2(v18,v11,(int)(char)v17,0x20680,optarg); // no-return
        if ((dat_21140) || (*optarg != '-')) break;
        error(1,0,gettext("invalid --threshold argument \'-0\'"));
        goto label_555e;
      case 0x78:
        v15 |= 0x40;
        break;
      case 0x80:
        dat_21139 = '\x01';
        break;
      case 0x81:
        sub_77e3(dat_21178,optarg,0x10000000);
        break;
      case 0x82:
        v21 = optarg;
        break;
      case 0x83:
        dat_21148 = 0x90;
        dat_21170 = 1;
        break;
      case 0x85:
        dat_2114d = '\x01';
        if (optarg) { // branch-flip
          v7 = sub_63c9("--time",optarg,0x209c0,0x194a0,4,dat_21020,1);
          dat_21150 = *(unsigned int *)(v7 * 4 + 0x194a0);
        }
        else {
          dat_21150 = 0;
        }
        dat_21168 = sub_141a3(getenv("TZ"));
        break;
      case 0x86:
        dat_21158 = optarg;
        break;
      case 0x87:
        goto label_569e;
      
    }
    goto label_52a5;
  }
  if (v14 != 1)
    sub_41ff(1); // no-return
  if ((dat_21138) && (v2)) {
    error(0,0,gettext("cannot both summarize and show all entries"));
    sub_41ff(1); // no-return
  }
  if (((v2) && (v3)) && (!dat_21010))
    error(0,0,gettext("warning: summarizing is the same as using --max-depth=0"));
  if (((v2) && (v3)) && (dat_21010)) {
    v23 = dat_21010;
    error(0,0,gettext("warning: summarizing conflicts with --max-depth=%lu"),v23);
    sub_41ff(1); // no-return
  }
  if (v2)
    dat_21010 = 0;
  if (dat_2114c) {
    if (dat_21139)
      error(0,0,gettext("warning: options --apparent-size and -b are ineffective with --inodes"));
    dat_21170 = 1;
  }
  if (dat_2114d) {
    if (!dat_21158) {
      dat_21158 = getenv("TIME_STYLE");
      if ((dat_21158) && (v5 = strcmp(dat_21158,"locale"), v5)) {
        if (*dat_21158 != '+') { // branch-flip
          while (v5 = strncmp(dat_21158,"posix-",dat_1a750), !v5) {
            dat_21158 = &dat_21158[dat_1a750];
          }
        }
        else {
          v24 = strchr(dat_21158,10);
          if (v24)
            *v24 = '\0';
        }
      }
      else {
        dat_21158 = "long-iso";
      }
    }
    if (*dat_21158 != '+') { // branch-flip
      v7 = sub_63c9("time style",dat_21158,0x20a00,0x194d0,4,dat_21020,1);
      v1 = *(unsigned int *)(v7 * 4 + 0x194d0);
      if (v1 != 2) { // branch-flip
        if (v1 <= 2) {
          if (v1) { // branch-flip
            if (v1 == 1)
              dat_21160 = "%Y-%m-%d %H:%M";
          }
          else {
            dat_21160 = "%Y-%m-%d %H:%M:%S.%N %z";
          }
        }
      }
      else {
        dat_21160 = "%Y-%m-%d";
      }
    }
    else {
      dat_21160 = &dat_21158[1];
    }
  }
  if (v21) { // branch-flip
    if (optind < a0) {
      v6 = sub_1401b(a1[optind]);
      error(0,0,gettext("extra operand %s"),v6);
      fprintf(stderr,"%s\n",gettext("file operands cannot be combined with --files0-from"));
      sub_41ff(1); // no-return
    }
    v5 = strcmp(v21,"-");
    if ((v5) && (v7 = sub_abea(v21,"r",stdin), !v7)) {
      v6 = sub_13bd4(4,v21);
      v8 = gettext("cannot open %s for reading");
      error(1,*__errno_location(),v8,v6);
      return v9;
    }
    v22 = sub_655d(stdin);
    dat_2113b = '\x01';
  }
  else {
    if (a0 <= optind) // branch-flip
      v25 = &v13;
    else {
      v25 = &a1[optind];
    }
    v22 = sub_6508(v25);
    dat_2113b = !((a0 <= optind + 1) && (v16 != 2));
  }
  if (!v22)
    sub_15992(); // no-return
  dat_21120 = sub_6936();
  if (!dat_21120)
    sub_15992(); // no-return
  if ((dat_2113a) || (dat_2113b != '\x01'))
    v15 |= 0x100;
  v15 |= v16;
  while( true ) {
    v2 = 0;
    v26 = (char *)sub_65ca(v22,&v12);
    if (!v26) break;
    if (((v21) && (v5 = strcmp(v21,"-"), !v5)) && (v5 = strcmp(v26,"-"), !v5)) {
      v6 = sub_13bd4(4,v26);
      error(0,0,gettext("when reading file names from stdin, no file name of %s allowed"),v6);
      v2 = 1;
    }
    if (!*v26) {
      if (v21) { // branch-flip
        v27 = sub_66b5(v22);
        v8 = gettext("invalid zero-length file name");
        v6 = sub_13d66(0,3,v21);
        error(0,0,"%s:%lu: %s",v6,v27,v8);
      }
      else {
        error(0,0,"%s",gettext("invalid zero-length file name"));
      }
      v2 = 1;
    }
    if (v2)
      v14 = 0;
    else {
      dat_211b0 = v26;
      v4 = sub_505b(0x211b0,v15);
      v14 = (v4 & v14) != 0;
    }
  }
  if (v12 != 4) {
    if (v12 <= 4) {
      if (v12 == 2) goto label_5f1c;
      if (v12 == 3)
        sub_15992(); // no-return
    }
    __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main"); // no-return
  }
  v6 = sub_13d66(0,3,v21);
  v8 = gettext("%s: read error");
  error(0,*__errno_location(),v8,v6);
  v14 = 0;
label_5f1c:
  sub_66f3(v22);
  sub_69c8(dat_21120);
  if (dat_21128)
    sub_69c8(dat_21128);
  if ((v21) && (((v5 = ferror_unlocked(stdin), v5 || (v5 = sub_7ea0(stdin), v5)) && (v14)))) {
    v6 = sub_13bd4(4,v21);
    error(1,0,gettext("error reading %s"),v6);
    return v10;
  }
  if (dat_2113d)
    sub_45f2(0x21180,gettext("total"));
  return (unsigned long)(v14 ^ 1);
label_569e:
  dat_2114c = '\x01';
  goto label_52a5;
}


// Function: sub_d9a8 @ 0xd9a8
unsigned long sub_d9a8(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_d9be @ 0xd9be
unsigned long sub_d9be(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_d9d4 @ 0xd9d4
unsigned long sub_d9d4(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_d9ea @ 0xd9ea
unsigned long sub_d9ea(unsigned long *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}


// Function: sub_da6c @ 0xda6c
unsigned long sub_da6c(unsigned long *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0[3]) && (v3 == a0[4]))
    return 1;
  return 0;
}


// Function: sub_db0a @ 0xdb0a
void sub_db0a(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_d9d4(a0);
  v2 = sub_d9a8(a0);
  v3 = sub_d9be(a0);
  v4 = sub_d9ea(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",(double)v2,(dat_1aac8 * (double)v3) / (double)v2,v3);
  fprintf(a1,"max bucket length: %lu\n",v4);
}


// Function: sub_dc51 @ 0xdc51
long sub_dc51(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v1)
    abort(); // no-return
  return v1 * 0x10 + *a0;
}


// Function: sub_dcac @ 0xdcac
long sub_dcac(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_dc51(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}


// Function: sub_dd43 @ 0xdd43
long sub_dd43(unsigned long *a0) // early-return
{
  long *v1; // stack - 0x10
  
  if (!a0[4])
    return 0;
  v1 = (long *)*a0;
  while( true ) {
    if ((long *)a0[1] <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}


// Function: sub_dda3 @ 0xdda3
long sub_dda3(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_dc51(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}


// Function: sub_de41 @ 0xde41
unsigned long sub_de41(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_dee1 @ 0xdee1
long sub_dee1(unsigned long *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_df79 @ 0xdf79
unsigned long sub_df79(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}


// Function: sub_dfd3 @ 0xdfd3
unsigned long sub_dfd3(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}


// Function: sub_e042 @ 0xe042
unsigned long sub_e042(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_dfd3(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}


// Function: sub_e08d @ 0xe08d
void sub_e08d(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = dat_1aad0;
  a0[2] = dat_1aad4;
  a0[3] = dat_1aad8;
  *(char *)&a0[4] = 0;
}


// Function: sub_e0e3 @ 0xe0e3
unsigned long sub_e0e3(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_175f6(a0,3) % a1;
}


// Function: sub_e11e @ 0xe11e
unsigned long sub_e11e(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}


// Function: sub_e13b @ 0xe13b
unsigned long sub_e13b(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x1aa40)
    return 1;
  if ((((dat_1aadc < v1[2]) && (v1[2] < dat_1aad0 - dat_1aadc)) && (dat_1aadc + dat_1aad0 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1aadc < v1[1])) && ((v1[1] <= dat_1aad0 && (*v1 + dat_1aadc < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x1aa40;
  return 0;
}


// Function: sub_e248 @ 0xe248
unsigned long sub_e248(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_1aae0 <= v2)
      return 0;
    if (dat_1aae4 <= v2)
      v3 = (long)(v2 - dat_1aae4) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_e042(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}


// Function: sub_e361 @ 0xe361
long * sub_e361(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_e0e3;
  v3 = a3;
  if (!a3)
    v3 = sub_e11e;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x1aa40;
  v2[5] = v5;
  if (sub_e13b(v2) == '\x01') {
    v2[2] = sub_e248(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}


// Function: sub_e4d6 @ 0xe4d6
void sub_e4d6(unsigned long *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0[8])
          (*(void *)a0[8])(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0[9];
        a0[9] = v3;
        v3 = v1;
      }
      if (a0[8])
        (*(void *)a0[8])(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0[3] = 0;
  a0[4] = 0;
}


// Function: sub_e5e1 @ 0xe5e1
void sub_e5e1(unsigned long *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0[8]) && (a0[4])) {
    for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0[8])(*v3);
        }
      }
    }
  }
  for (v2 = (unsigned long)*a0; v2 < a0[1]; v2 = (unsigned long)((long)v2 + 0x10)) {
    v3 = *(void **)((long)v2 + 8);
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = (void *)a0[9];
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free((void *)*a0);
  free(a0);
}


// Function: sub_e710 @ 0xe710
void * sub_e710(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}


// Function: sub_e75f @ 0xe75f
void sub_e75f(long a0,unsigned long *a1)
{
  *a1 = 0;
  a1[1] = *(unsigned long *)(a0 + 0x48);
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_e799 @ 0xe799
long sub_e799(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long v2;
  long *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_dc51(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v4 = v6[1];
      v6[1] = *(long *)(v4 + 8);
      sub_e75f(a0,v4);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (long *)v6[1];
      v4 = v3[1];
      *v6 = *v3;
      v6[1] = v4;
      sub_e75f(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}


// Function: sub_e932 @ 0xe932
unsigned long sub_e932(long a0,unsigned long *a1,char a2)
{
  long v1;
  long *v2;
  long *v3; // rax
  long *v4; // stack - 0x38
  long *v5; // stack - 0x30
  
  v4 = (long *)*a1;
  do {
    if ((long *)a1[1] <= v4)
      return 1;
    if (*v4) {
      v5 = (long *)v4[1];
      while (v5) {
        v1 = *v5;
        v3 = (long *)sub_dc51(a0,v1);
        v2 = (long *)v5[1];
        if (*v3) { // branch-flip
          v5[1] = v3[1];
          v3[1] = (long)v5;
        }
        else {
          *v3 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_e75f(a0,v5);
        }
        v5 = v2;
      }
      v1 = *v4;
      v4[1] = 0;
      if (!a2) {
        v2 = (long *)sub_dc51(a0,v1);
        if (*v2) { // branch-flip
          v3 = (long *)sub_e710(a0);
          if (!v3)
            return 0;
          *v3 = v1;
          v3[1] = v2[1];
          v2[1] = (long)v3;
        }
        else {
          *v2 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v4 = 0;
        a1[3] = a1[3] + -1;
      }
    }
    v4 = &v4[2];
  } while( true );
}


// Function: sub_eb06 @ 0xeb06
unsigned long sub_eb06(unsigned long *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_e248(a1,a0[5]);
  if (!v3)
    return 0;
  if (v3 == a0[2])
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  if (sub_e932(&v1,a0,0)) {
    free((void *)*a0);
    *a0 = v1;
    a0[1] = v2;
    a0[2] = v3;
    a0[3] = v4;
    a0[9] = v10;
    return 1;
  }
  a0[9] = v10;
  if ((sub_e932(a0,&v1,1) == '\x01') && (sub_e932(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}


// Function: sub_ed31 @ 0xed31
unsigned long sub_ed31(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_e799(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_e13b(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_1aae0 <= v4)
      return 0xffffffff;
    v1 = (dat_1aae4 <= v4) ? (long)(v4 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_eb06(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_e799(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_e710(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}


// Function: sub_f0a2 @ 0xf0a2
unsigned long sub_f0a2(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_ed31(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_f112 @ 0xf112
long sub_f112(long a0,unsigned long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_e799(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_e13b(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_1aae4 <= v3) ? (long)(v3 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_1aae4 <= v3) ? (long)(v3 - dat_1aae4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_eb06(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}


// Function: sub_f418 @ 0xf418
void sub_f418(unsigned long a0,unsigned long a1)
{
  sub_f112(a0,a1);
}

