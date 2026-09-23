// Function: sub_7370 @ 0x7370
unsigned long sub_7370(int a0,long *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  
  v3 = (char *)*a1;
  if (!v3)
    v3 = "find";
  sub_22cb0(v3);
  sub_d970();
  dat_3e618 = 0;
  dat_3e604 = 0;
  dat_3e608 = 0;
  dat_3e5f8 = 0xffffff9c;
  if (sub_15dd0())
    sub_15c30();
  dat_3e610 = sub_e4d0("w");
  if (dat_3e610) {
    sub_de90(0x3e620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_2a3c0(sub_175a0)) {
      v1 = sub_dd00(a0,a1);
      if (dat_3e660 & 2)
        dat_3e670 = sub_db70;
      if (dat_3e660 & 0x80)
        __fprintf_chk(stderr,1,"cur_day_start = %s",ctime(0x3e648));
      v2 = sub_c6f0(a0,a1,v1);
      if (sub_8210(a0 - v1,&a1[v1])) {
        sub_9ee0(v2);
        sub_e360();
      }
      return (unsigned long)dat_3e604;
    }
    v3 = dcgettext(NULL,"The atexit library function failed",5);
    error(1,*__errno_location(),v3);
    return v4;
  }
  v3 = dcgettext(NULL,"Failed to initialize shared-file hash table",5);
  error(1,*__errno_location(),v3);
  return v5;
}


// Function: sub_7640 @ 0x7640
unsigned long sub_7640(unsigned long a0) // early-return
{
  int v1; // eax
  int *v2; // rax
  char v3 [152];
  
  v1 = (*dat_3e670)(a0,v3);
  if (!v1)
    return 0;
  v2 = __errno_location();
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),*v2 == 0x28);
}


// Function: sub_76a0 @ 0x76a0
void sub_76a0(int a0)
{
  if (!(dat_3e019 & 2))
    return;
  if ((a0 != -100) && (a0 < 0))
    __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir"); // no-return
  dat_3e5f8 = a0;
  if (0 <= dat_3e010)
    return;
  if (a0 == -100) {
    dat_3e010 = 0xffffff9c;
    return;
  }
  dat_3e010 = sub_17560();
}


// Function: sub_7720 @ 0x7720
void sub_7720(void)
{
  if ((dat_3e019 & 2) && (0 <= dat_3e010)) {
    close(dat_3e010);
    dat_3e010 = 0xffffffff;
    return;
  }
}


// Function: sub_7750 @ 0x7750
char * sub_7750(unsigned int a0)
{
  switch(a0) {
    default:
      __sprintf_chk((char *)0x3e598,1,0xe,"[%d]",(unsigned long)a0);
      return (char *)0x3e598;
    case 1:
      return "FTS_D";
    case 2:
      return "FTS_DC";
    case 3:
      return "FTS_DEFAULT";
    case 4:
      return "FTS_DNR";
    case 5:
      return "FTS_DOT";
    case 6:
      return "FTS_DP";
    case 7:
      return "FTS_ERR";
    case 8:
      return "FTS_F";
    case 9:
      return "FTS_INIT";
    case 10:
      return "FTS_NS";
    case 0xb:
      return "FTS_NSOK";
    case 0xc:
      return "FTS_SL";
    case 0xd:
      return "FTS_SLNONE";
    case 0xe:
      return "FTS_W";
    
  }
}


// Function: sub_7880 @ 0x7880
void sub_7880(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  dat_3e5f0 = *(unsigned long *)(a1 + 0x30);
  dat_3e5e4 = 2 <= (unsigned short)(*(short *)(a1 + 0x68) - 10U);
  dat_3e5f8 = *(unsigned int *)(a0 + 0x2c);
  v1 = sub_b5e0();
  sub_e2a0(*(unsigned long *)(a1 + 0x38),a2,v1);
  if (!dat_3e600)
    return;
  sub_1a190(a0,a1,4); // tail-call
}


// Function: sub_7950 @ 0x7950
void sub_7950(long *a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  if ((*(unsigned int *)&a0[0x11] & 0xf000) != 0xa000) {
    v1 = sub_7900(*(unsigned long *)(*a0 + 0x38),*(unsigned long *)(*a0 + 0x48),dat_3e68c);
    v2 = sub_dfb0(0,a0[7]);
    error(0,0,dcgettext(NULL,"File system loop detected; %s is part of the same file system loop as %s.",5),v2,v1); // tail-call
    return;
  }
  v1 = sub_dfb0(0,a0[7]);
  error(0,0,dcgettext(NULL,"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",5),v1); // tail-call
}


// Function: sub_7a00 @ 0x7a00
void sub_7a00(long a0,long a1)
{
  unsigned char v1;
  unsigned long v10; // stack - 0xd8
  unsigned int v11; // stack - 0xdc
  short v12;
  unsigned long v13;
  unsigned long v14; // r15
  unsigned long v15; // stack - 0xd0
  unsigned long v16; // stack - 0xc8
  unsigned long v17; // stack - 0xc0
  unsigned long v18; // stack - 0xb8
  unsigned long v19; // stack - 0xb0
  char v2; // al
  unsigned long v20; // stack - 0xa8
  unsigned long v21; // stack - 0xa0
  unsigned long v22; // stack - 0x98
  unsigned long v23; // stack - 0x90
  unsigned long v24; // stack - 0x88
  unsigned long v25; // stack - 0x80
  unsigned long v26; // stack - 0x78
  unsigned long v27; // stack - 0x70
  unsigned long v28; // stack - 0x68
  unsigned long v29; // stack - 0x60
  short v3;
  unsigned long v30; // stack - 0x58
  unsigned long v31; // stack - 0x50
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned int v8;
  unsigned long v9; // rbx
  
  if (dat_3e660 & 4) {
    v6 = sub_24e50(2,dat_3e68c,*(unsigned long *)(a1 + 0x30));
    v7 = sub_24e50(1,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    v13 = *(unsigned long *)(a1 + 0x58);
    v9 = (unsigned long)dat_3e014;
    v4 = sub_7750(*(unsigned short *)(a1 + 0x68));
    v5 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    __fprintf_chk(stderr,1,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v5,v4,v13,v9,v7,v6);
  }
  if (((*(short *)(a1 + 0x68) == 6) || ((long)(int)dat_3e014 < *(long *)(a1 + 0x58))) || (!*(long *)(a1 + 0x58)))
    sub_7720();
  sub_76a0(*(unsigned int *)(a0 + 0x2c));
  v15 = *(unsigned long *)(a1 + 0x78);
  v3 = *(short *)(a1 + 0x68);
  dat_3e014 = (unsigned int)*(long *)(a1 + 0x58);
  if (v3 == 7) {
    sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38)); // return-dupe
    return;
  }
  if (v3 != 4) { // branch-flip
    if (v3 == 2) {
      sub_7950(a1);
      dat_3e604 = 1;
      return;
    }
    if (v3 == 0xd) {
      v13 = *(unsigned long *)(a1 + 0x30);
      if (sub_7640(v13)) {
        sub_e060(0x28,*(unsigned long *)(a1 + 0x38));
        return;
      }
      goto label_7e31;
    }
    if (v3 == 10) {
      if (!*(long *)(a1 + 0x58)) {
        sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
        return;
      }
      v2 = sub_7640(*(unsigned long *)(a1 + 0x30));
      if (v2) {
        sub_e060(0x28,*(unsigned long *)(a1 + 0x38));
        return;
      }
      sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
      v3 = *(short *)(a1 + 0x68); // crossjump-dupe
    }
  }
  else {
    sub_e060(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
    if (dat_3e620)
      return;
label_7e31:
    v3 = *(short *)(a1 + 0x68);
  }
  if (2 <= (unsigned short)(v3 - 10U)) { // branch-flip
    v10 = *(unsigned long *)(a1 + 0x70);
    v15 = *(unsigned long *)(a1 + 0x78);
    v16 = *(unsigned long *)(a1 + 0x80);
    v17 = *(unsigned long *)(a1 + 0x88);
    dat_3e5e4 = 0x101;
    v18 = *(unsigned long *)(a1 + 0x90);
    v19 = *(unsigned long *)(a1 + 0x98);
    v20 = *(unsigned long *)(a1 + 0xa0);
    v21 = *(unsigned long *)(a1 + 0xa8);
    v22 = *(unsigned long *)(a1 + 0xb0);
    v23 = *(unsigned long *)(a1 + 0xb8);
    v24 = *(unsigned long *)(a1 + 0xc0);
    v25 = *(unsigned long *)(a1 + 200);
    v26 = *(unsigned long *)(a1 + 0xd0);
    v27 = *(unsigned long *)(a1 + 0xd8);
    v28 = *(unsigned long *)(a1 + 0xe0);
    v29 = *(unsigned long *)(a1 + 0xe8);
    v30 = *(unsigned long *)(a1 + 0xf0);
    v31 = *(unsigned long *)(a1 + 0xf8);
    v11 = (unsigned int)v17;
    dat_3e5e8 = (unsigned int)v17;
    if (!(unsigned int)v17) {
      v13 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
      error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v13);
      v8 = v11;
      goto label_7d35;
    }
    dat_3e5e0 = (unsigned int)*(unsigned long *)(a1 + 0x58);
label_7b66:
    v13 = *(unsigned long *)(a1 + 0x38);
    if (!sub_e150(&v11,v13,a1 + 0x100,&v10,0))
      return;
    v3 = *(short *)(a1 + 0x68);
    v12 = v3;
    if ((v11 & 0xf000) != 0x4000) goto label_7d4b;
label_7ba8:
    if (v3 == 0xb) {
      sub_1a190(a0,a1,1);
      return;
    }
    v13 = 1;
    v9 = 0;
    if (0 <= dat_3e624) {
label_7bc5:
      if ((long)dat_3e624 <= *(long *)(a1 + 0x58)) { // branch-flip
        sub_1a190(a0,a1,4);
        v12 = *(short *)(a1 + 0x68);
        v9 = (unsigned long)((long)dat_3e624 < *(long *)(a1 + 0x58));
      }
      else {
        v9 = 0;
        v12 = v3;
      }
    }
    v8 = (unsigned int)CONCAT71(dat_3e661,dat_3e660);
    v3 = v12;
    if (v12 != 1) { // branch-flip
      if ((v12 != 6) || (!dat_3e620)) goto label_7bf5;
      if (!(dat_3e660 & 4)) {
        dat_3e600 = 0;
        return;
      }
    }
    else {
      if (dat_3e620) goto label_7bf5;
      if (!(dat_3e660 & 4))
        return;
    }
label_7d91:
    v9 = 1;
label_7d96:
    v1 = dat_3e5e5;
    v14 = (unsigned long)dat_3e5e4;
    v6 = sub_7750(v12);
    v7 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + 0x38));
    __fprintf_chk(stderr,1,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v7,v6,v13,v9,v14,(unsigned long)v1);
  }
  else {
    if (dat_3e5e4)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((v3 != 0xb) && (dat_3e5e8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v11 = dat_3e5e8;
    v8 = dat_3e5e8;
label_7d35:
    dat_3e5e0 = (unsigned int)*(unsigned long *)(a1 + 0x58);
    if (v8) goto label_7b66;
    v3 = *(short *)(a1 + 0x68);
label_7d4b:
    v12 = v3;
    if (((unsigned short)(v3 - 1U) < 2) || (v3 == 6)) goto label_7ba8;
    v13 = 0;
    if (0 <= dat_3e624) goto label_7bc5;
    v8 = (unsigned int)CONCAT71(dat_3e661,dat_3e660);
    v9 = 0;
label_7bf5:
    v12 = v3;
    if (*(long *)(a1 + 0x58) < (long)dat_3e628) {
      if (!(v8 & 4)) goto label_7c26;
      goto label_7d91;
    }
    if (v8 & 4) goto label_7d96;
  }
  if (!(int)v9)
    sub_7880(a0,a1,&v10);
label_7c26:
  if (*(short *)(a1 + 0x68) == 6) {
    dat_3e600 = 0;
    return;
  }
}


// Function: sub_7fe0 @ 0x7fe0
unsigned long sub_7fe0(char *a0) // return-dupe
{
  int v1;
  unsigned long v10; // stack - 0x40
  long v2; // rax
  int *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x48
  unsigned int v8;
  int v9; // r13d
  
  dat_3e5fc = (unsigned int)strlen(a0);
  sub_76a0(0xffffff9c);
  v10 = 0;
  if (dat_3e668 != 1) { // branch-flip
    if (dat_3e668 != 2) { // branch-flip
      v8 = dat_3e018;
      if (!dat_3e668) {
        dat_3e018 |= 0x10;
        v8 = dat_3e018;
      }
    }
    else {
      dat_3e018 |= 0x11;
      v8 = dat_3e018;
    }
  }
  else {
    dat_3e018 |= 3;
    v8 = dat_3e018;
  }
  if (dat_3e62d) {
    dat_3e018 = v8 | 0x40;
    v8 = dat_3e018;
  }
  v9 = -0x80000000;
  v7 = a0;
  v2 = sub_195e0(&v7,v8,0);
  v3 = __errno_location();
  if (!v2) {
    v5 = sub_dfb0(0,a0);
    v6 = dcgettext(NULL,"cannot search %s",5);
    error(0,*v3,v6,v5);
    dat_3e604 = 1;
    return 1;
  }
  while( true ) {
    *v3 = 0;
    v4 = sub_19b40(v2);
    if (!v4) break;
    v1 = (int)*(unsigned long *)(v4 + 0x58);
    if ((dat_3e608) && (v9 != v1)) {
      sub_d940();
      v1 = *(int *)(v4 + 0x58);
    }
    v9 = v1;
    dat_3e5e8 = *(int *)(v4 + 0x88);
    dat_3e618 = 0;
    dat_3e5e4 = 0;
    dat_3e5e5 = dat_3e5e8 != 0;
    sub_7a00(v2);
  }
  if (*v3) { // branch-flip
    a0 = (char *)sub_dfb0(0,a0);
    v6 = "failed to read file names from file system at or below %s";
  }
  else {
    if (!sub_19950(v2))
      return 1;
    v6 = dcgettext(NULL,"failed to restore working directory after searching %s",5);
  }
  error(0,*v3,v6,a0);
  dat_3e604 = 1;
  return 0;
}


// Function: sub_8210 @ 0x8210
unsigned long sub_8210(int a0,unsigned long *a1)
{
  int v1; // eax
  unsigned long v10; // rax
  int v11; // stack - 0x16c
  long v12; // stack - 0x168
  unsigned short v13; // stack - 0x42
  long v14; // stack - 0xd8
  unsigned long v15; // r12
  char *v16; // stack - 0x180
  long v17; // stack - 0x160
  long v18; // stack - 0xd0
  unsigned long v2;
  long v3; // rax
  char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  if ((1 <= a0) && (v2 = *a1, !sub_dca0(v2,1))) {
    v16 = dat_3e690;
    if (dat_3e690) {
      v2 = sub_dfb0(0,*a1);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v2);
      error(1,0,"%s",dcgettext(NULL,"file operands cannot be combined with -files0-from",5));
      return v10;
    }
    v2 = 0;
    v3 = sub_171b0(a1);
  }
  else {
    v4 = dat_3e690;
    if (!dat_3e690) {
      v13 = 0x2e;
      return (unsigned long)(unsigned int)sub_7fe0(&v13);
    }
    v1 = strcmp(dat_3e690,"-");
    if (v1) { // branch-flip
      v2 = sub_dfb0(0,v4);
      v16 = fopen(dat_3e690,"r");
      if (!v16) {
        v4 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v4,v2);
        return v8;
      }
      v1 = fileno(v16);
      if (v1 < 0)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_3e698) && (!fstat(v1,&v12))) && (!fstat(0,&v14))) && ((v17 == v18 && (v12 == v14)))) {
        error(1,0,"%s: %s\n",dcgettext(NULL,"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",5),v2);
        return v6;
      }
      sub_174f0(v1,1);
    }
    else {
      if (dat_3e698) {
        error(1,0,"%s\n",dcgettext(NULL,"option -files0-from reading from standard input cannot be combined with -ok, -okdir",5));
        return v9;
      }
      v2 = sub_dfb0(0,dcgettext(NULL,"(standard input)",5));
      v16 = stdin;
    }
    v3 = sub_171e0(v16);
  }
  if (!v3)
    sub_26d50(); // no-return, return-dupe
  v15 = 1;
  do {
    while( true ) {
      v4 = (char *)sub_17220(v3,&v11);
      if (!v4) {
        if (v11 == 3) {
          sub_26d50();
        }
        if (v11 == 4) {
          v4 = dcgettext(NULL,"%s: read error",5);
          error(0,*__errno_location(),v4,v2);
          dat_3e604 = 1;
          sub_172e0(v3);
          return 0;
        }
        if (v11 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
        goto label_8316;
      }
      if (*v4) break;
      if (dat_3e690) { // branch-flip
        v5 = sub_172c0(v3);
        error(0,0,"%s:%lu: %s",v2,v5,dcgettext(NULL,"invalid zero-length file name",5));
      }
      else {
        error(0,2,"%s",sub_dfb0(0,v4));
      }
      dat_3e604 = 1;
      v15 = 0;
    }
    if ((!dat_3e690) && (sub_dca0(v4,1))) goto label_8316;
    dat_3e5fc = (unsigned int)strlen(v4);
  } while (sub_7fe0(v4));
  v15 = 0;
label_8316:
  sub_172e0(v3);
  if ((((char)v15) && (dat_3e690)) && ((ferror(v16) || (sub_17850(v16))))) {
    error(1,0,dcgettext(NULL,"error reading %s",5),v2);
    return v7;
  }
  return v15;
}


// Function: sub_86d0 @ 0x86d0
unsigned long sub_86d0(unsigned int *a0)
{
  *a0 = dat_3e018;
  return 1;
}


// Function: sub_86f0 @ 0x86f0
unsigned long sub_86f0(unsigned long a0,long a1)
{
  if (dat_3e620) {
    if (!dat_3e5e4)
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune"); // no-return
    if ((a1) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000)) {
      dat_3e600 = 1;
      return 1;
    }
  }
  return 1;
}


// Function: sub_8750 @ 0x8750
double sub_8750(unsigned long a0,long a1,unsigned long a2,long a3)
{
  double v1; // xmm0_qa
  
  difftime(a0,a2);
  return (double)(a1 - a3) * dat_2b960 + v1;
}


// Function: sub_8790 @ 0x8790
unsigned long sub_8790(long a0,long a1,long a2,long a3) // return-dupe
{
  if ((a0 == a2) && (a1 == a3))
    return 0;
  if ((double)sub_8750() < 0.0)
    return 0xffffffff;
  return 1;
}


// Function: sub_87d0 @ 0x87d0
unsigned long sub_87d0(unsigned long a0,unsigned long a1,long a2,int a3) // early-return
{
  int v1;
  unsigned long v2;
  unsigned long v3;
  double v4; // xmm0_qa
  
  v1 = *(int *)(a2 + 0x3c);
  if (v1 == 1) {
    v3 = *(unsigned long *)(a2 + 0x40);
    v2 = *(unsigned long *)(a2 + 0x48);
    return (unsigned long)((unsigned int)sub_8790(a0,a1,v3,v2) >> 0x1f);
  }
  if (v1 == 2) {
    v4 = (double)sub_8750(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    if (v4 <= dat_2b968)
      return 0;
    return (unsigned long)(v4 <= (double)a3);
  }
  if (!v1) {
    v3 = sub_8790(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),0 < (int)v3);
  }
  __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return
}


// Function: sub_8870 @ 0x8870
void sub_8870(unsigned long a0,unsigned long a1,long a2)
{
  long v1;
  
  v1 = *(long *)(a2 + 0x110);
  if (v1) {
    if (!sub_e2a0(a0,a1,v1))
      return;
  }
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}


// Function: sub_88c0 @ 0x88c0
void sub_88c0(unsigned long a0,unsigned long a1,long a2)
{
  long v1;
  
  v1 = *(long *)(a2 + 0x110);
  if (v1) {
    if (sub_e2a0(a0,a1,v1))
      return;
  }
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}


// Function: sub_8910 @ 0x8910
unsigned int sub_8910(int a0)
{
  int v1; // eax
  
  v1 = unlinkat(dat_3e5f8,dat_3e5f0,a0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_8940 @ 0x8940
int sub_8940(unsigned long a0,char *a1,int a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  int v4; // eax
  char *v5; // rax
  
  v5 = (char *)sub_176f0(a0);
  v3 = v5;
  sub_17810(v5);
  v4 = fnmatch(a1,v5,a2);
  v2 = v4;
  free(v3);
  v1 = (unsigned int)v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}


// Function: sub_8990 @ 0x8990
void sub_8990(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  char *v2; // rax
  
  sub_17bc0(stdout);
  v1 = __fprintf_chk(stderr,1,dcgettext(NULL,"< %s ... %s > ? ",5),a0,a1);
  if (0 <= v1) {
    sub_17bc0(stderr);
    sub_276e0(); // tail-call
    return;
  }
  v2 = dcgettext(NULL,"Failed to write prompt for -ok",5);
  error(1,*__errno_location(),v2);
}


// Function: sub_8a40 @ 0x8a40
char * sub_8a40(char *a0,char *a1)
{
  char *v1;
  int v2; // eax
  int v3; // ecx
  char *v4;
  
  if (!a0)
    return NULL;
  strcpy(a1,a0);
  v2 = (int)strlen(a1);
  v3 = v2 + -1;
  if (0 <= v3) { // branch-flip
    v1 = &a1[v3];
    do {
      v4 = v1;
      if ((*v4 != ' ') && (*v4 != '\t')) {
        v4 = &a1[v3 + 1];
        break;
      }
      v3 -= 1;
      v1 = &v4[-1];
    } while (v3 != -1);
  }
  else {
    v4 = &a1[v2];
  }
  *v4 = '\0';
  return a1;
}


// Function: sub_8b70 @ 0x8b70
void sub_8b70(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c40(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}


// Function: sub_8ba0 @ 0x8ba0
unsigned long sub_8ba0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c40(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
}


// Function: sub_8c00 @ 0x8c00
void sub_8c00(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c40(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}


// Function: sub_8c40 @ 0x8c40
void sub_8c40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c50(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}


// Function: sub_8c70 @ 0x8c70
unsigned long sub_8c70(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c50(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
}


// Function: sub_8cd0 @ 0x8cd0
void sub_8cd0(unsigned long a0,unsigned long a1,long a2)
{
  if (*(long *)(a2 + 0x110))
    sub_e2a0(a0,a1,*(long *)(a2 + 0x110));
  sub_e2a0(a0,a1,*(unsigned long *)(a2 + 0x118)); // tail-call
}


// Function: sub_8d10 @ 0x8d10
void sub_8d10(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c50(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}


// Function: sub_8d40 @ 0x8d40
unsigned int sub_8d40(unsigned long a0,long a1) // return-dupe
{
  int v1; // eax
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  
  v1 = strcmp(dat_3e5f0,".");
  if (!v1)
    return 1;
  v1 = 0;
  if (dat_3e5e4)
    v1 = (unsigned int)((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000) << 9;
  v2 = sub_8910(v1);
  if ((char)v2)
    return 1;
  v3 = __errno_location();
  if (*v3 != 2) { // branch-flip
    if (((*v3 == 0x15) && (!v1)) && (sub_8910(0x200)))
      return 1;
  }
  else if (dat_3e62e) {
    *v3 = 0;
    return (unsigned int)dat_3e62e;
  }
  v4 = sub_dfb0(0,a0);
  v5 = dcgettext(NULL,"cannot delete %s",5);
  error(0,*v3,v5,v4);
  dat_3e604 = 1;
  return v2;
}


// Function: sub_8e50 @ 0x8e50
bool sub_8e50(unsigned long a0,long a1)
{
  unsigned int v1; // eax
  int v2; // eax
  int *v3; // rax
  void *v4; // rax
  void *v5; // rax
  unsigned long v6; // rax
  bool v7; // r12b
  
  v1 = *(unsigned int *)(a1 + 0x18) & 0xf000;
  if (v1 != 0x4000) {
    if (v1 != 0x8000)
      return 0;
    return *(long *)(a1 + 0x30) == 0;
  }
  v3 = __errno_location();
  *v3 = 0;
  v2 = sub_1e690(dat_3e5f8,dat_3e5f0,0x90900);
  if (0 <= v2) {
    v4 = fdopendir(v2);
    if (!v4) {
      v6 = sub_dfb0(0,a0);
      error(0,*v3,"%s",v6);
      dat_3e604 = 1;
      close(v2);
      return 0;
    }
    *v3 = 0;
    do {
      v5 = readdir(v4);
      if (!v5) {
        v7 = 1;
        goto label_8f03;
      }
    } while ((*(char *)((long)v5 + 0x13) == '.') && ((!*(char *)((long)v5 + 0x14) || (*(short *)((long)v5 + 0x14) == 0x2e))));
    v7 = 0;
label_8f03:
    if (*v3) {
      v6 = sub_dfb0(0,a0);
      error(0,*v3,"%s",v6);
      dat_3e604 = 1;
      closedir(v4);
      return 0;
    }
    if (!closedir(v4))
      return v7;
  }
  v6 = sub_dfb0(0,a0);
  error(0,*v3,"%s",v6);
  dat_3e604 = 1;
  return 0;
}


// Function: sub_8ff0 @ 0x8ff0
void sub_8ff0(void)
{
  sub_a390(dat_3e5f0); // tail-call
}


// Function: sub_9000 @ 0x9000
unsigned long sub_9000(void)
{
  return 0;
}


// Function: sub_9010 @ 0x9010
unsigned long sub_9010(unsigned long a0,unsigned long a1,long a2)
{
  sub_16170(a0,dat_3e5f8,dat_3e5f0,a1,dat_3e638,dat_3e65c,*(char *)(a2 + 0x28),*(unsigned long *)(a2 + 0x40));
  return 1;
}


// Function: sub_9050 @ 0x9050
unsigned long sub_9050(unsigned long a0,unsigned long a1,long a2)
{
  sub_16960(*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x58),*(char *)(a2 + 0x50),"%s\n",a0);
  return 1;
}


// Function: sub_9090 @ 0x9090
unsigned long sub_9090(char *a0,unsigned long a1,long a2)
{
  void *v1;
  
  v1 = *(void **)(a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}


// Function: sub_90c0 @ 0x90c0
unsigned int sub_90c0(unsigned long a0,unsigned long a1,long a2)
{
  char *v1;
  int v2; // eax
  
  v1 = *(char **)(a2 + 0x38);
  v2 = strcmp((char *)sub_104c0(a1,a0),v1);
  return CONCAT31((undefined3)((unsigned int)v2 >> 8),v2 == 0);
}


// Function: sub_90f0 @ 0x90f0
unsigned long sub_90f0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x20) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x20));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x20) == *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9140 @ 0x9140
unsigned int sub_9140(unsigned long a0,long a1,long a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x20) >> 8),*(int *)(a2 + 0x38) == *(int *)(a1 + 0x20));
}


// Function: sub_9150 @ 0x9150
void sub_9150(unsigned long a0,long a1,unsigned long a2)
{
  sub_8ad0(a0,*(unsigned int *)(a1 + 0x18),a2,1); // tail-call
}


// Function: sub_9170 @ 0x9170
void sub_9170(unsigned long a0,unsigned long a1,long a2)
{
  sub_8940(a0,*(unsigned long *)(a2 + 0x38),0x10); // tail-call
}


// Function: sub_9190 @ 0x9190
unsigned long sub_9190(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)(a1 + 8) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 8));
  }
  return CONCAT71(v2,*(long *)(a1 + 8) == *(long *)(a2 + 0x40)) & 0xffffffff;
}


// Function: sub_91e0 @ 0x91e0
unsigned int sub_91e0(char *a0,unsigned long a1,long a2)
{
  int v1; // eax
  
  v1 = fnmatch(*(char **)(a2 + 0x38),a0,0x10);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_9210 @ 0x9210
unsigned long sub_9210(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)(a1 + 0x10) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 0x10));
  }
  return CONCAT71(v2,*(long *)(a1 + 0x10) == *(long *)(a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9260 @ 0x9260
void sub_9260(unsigned long a0,long a1,unsigned long a2)
{
  sub_8ad0(a0,*(unsigned int *)(a1 + 0x18),a2,0); // tail-call
}


// Function: sub_9280 @ 0x9280
void sub_9280(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c60(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x3c); // tail-call
}


// Function: sub_92b0 @ 0x92b0
void sub_92b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_25c60(a1);
  sub_87d0(SUB168(v1,0),SUB168(v1,8),a2,0x15180); // tail-call
}


// Function: sub_92e0 @ 0x92e0
void sub_92e0(unsigned long a0,unsigned long a1,long a2)
{
  sub_8940(a0,*(unsigned long *)(a2 + 0x38),0); // tail-call
}


// Function: sub_92f0 @ 0x92f0
unsigned int sub_92f0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a2 + 0x118);
  return sub_e2a0(a0,a1,v1) ^ 1;
}


// Function: sub_9310 @ 0x9310
unsigned long sub_9310(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (!*(int *)(a2 + 0x3c)) {
    v2 = sub_25c60(a1);
    v1 = sub_8790(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
  }
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
}


// Function: sub_9370 @ 0x9370
unsigned long sub_9370(unsigned long a0,unsigned long a1,long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // rdx
  char v3 [16];
  
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)(a2 + 0x38)) {
    case 0:
      v3 = sub_25c40(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 1:
      v3 = sub_25c70();
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      if (v2 <= -1) {
        v1 = sub_dfb0(0,a0);
        error(0,0,dcgettext(NULL,"WARNING: cannot determine birth time of file %s",5),v1);
        return 0;
      }
      break;
    case 2:
      v3 = sub_25c50(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 3:
      v3 = sub_25c60(a1);
      v2 = SUB168(v3,8);
      v1 = SUB168(v3,0);
      break;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    default:
      __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
    
  }
  v1 = sub_8790(v1,v2,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}


// Function: sub_94a0 @ 0x94a0
unsigned long sub_94a0(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)(a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_94c0 @ 0x94c0
unsigned long sub_94c0(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)(a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_94e0 @ 0x94e0
unsigned long sub_94e0(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (!sub_8990(v1,a0))
    return 0;
  return sub_a390(a0,a1,a2); // tail-call
}


// Function: sub_9530 @ 0x9530
unsigned long sub_9530(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (!sub_8990(v1,a0))
    return 0;
  v1 = sub_a390(dat_3e5f0,a1,a2); // tail-call
  return v1;
}


// Function: sub_95a0 @ 0x95a0
unsigned int sub_95a0(char *a0,unsigned long a1,long a2)
{
  int v1; // eax
  
  v1 = fnmatch(*(char **)(a2 + 0x38),a0,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_95c0 @ 0x95c0
unsigned long sub_95c0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned int v1;
  unsigned int v2;
  int v3;
  undefined7 v4; // r8
  
  v1 = *(unsigned int *)(a1 + 0x18);
  v2 = *(unsigned int *)(a2 + 0x3c + (unsigned long)((v1 & 0xf000) == 0x4000) * 4);
  v3 = *(int *)(a2 + 0x38);
  if (v3 == 1) {
    if (!v2)
      return 1;
    return (unsigned long)((v1 & v2) != 0);
  }
  v4 = (undefined7)((unsigned long)a4 >> 8);
  if (v3 != 2) {
    if (v3)
      abort(); // no-return
    return CONCAT71(v4,(v1 & v2) == v2) & 0xffffffff;
  }
  return CONCAT71(v4,(v1 & 0xfff) == v2) & 0xffffffff;
}


// Function: sub_9640 @ 0x9640
unsigned int sub_9640(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,1,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_9670 @ 0x9670
unsigned int sub_9670(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,4,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_96a0 @ 0x96a0
unsigned int sub_96a0(void)
{
  int v1; // eax
  
  v1 = faccessat(dat_3e5f8,dat_3e5f0,2,0);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}


// Function: sub_96f0 @ 0x96f0
unsigned long sub_96f0(char *a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  v1 = strlen(a0);
  v2 = re_match(*(unsigned long *)(a2 + 0x38),a0,v1 & 0xffffffff,0,0);
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 == (int)v1);
}


// Function: sub_9730 @ 0x9730
unsigned long sub_9730(unsigned long a0,long a1,long a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (*(long *)(a1 + 0x30) / (long)*(int *)(a2 + 0x3c) + 1) - (unsigned long)(*(long *)(a1 + 0x30) % (long)*(int *)(a2 + 0x3c) == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < v2);
  }
  return CONCAT71(v3,*(unsigned long *)(a2 + 0x40) == v2) & 0xffffffff;
}


// Function: sub_97a0 @ 0x97a0
unsigned long sub_97a0(unsigned long a0,long *a1,long a2)
{
  unsigned long v1; // rax
  
  if (((!a1[1]) || (a1[1] == *(long *)(a2 + 0x38))) && (v1 = sub_e080(a0,dat_3e5f0,a1), !(int)v1)) {
    if (a1[1] != *(long *)(a2 + 0x38))
      return v1;
    return CONCAT71((undefined7)((unsigned long)*(long *)(a2 + 0x40) >> 8),*a1 == *(long *)(a2 + 0x40));
  }
  return 0;
}


// Function: sub_9810 @ 0x9810
char sub_9810(unsigned long a0,long a1,long a2)
{
  unsigned int v1;
  long v2; // rcx
  char v3;
  
  if (!dat_3e5e5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  v3 = 0;
  if (dat_3e5e8) {
    v1 = dat_3e5e8;
    if (dat_3e5e4)
      v1 = *(unsigned int *)(a1 + 0x18);
    v1 &= 0xf000;
    if (v1 != 0x6000) { // branch-flip
      if (0x6001 <= v1) { // branch-flip
        if (v1 != 0xa000) { // branch-flip
          v2 = 6;
          if (v1 != 0xc000) {
            if (v1 != 0x8000)
              return 0;
            v2 = 3;
          }
        }
        else {
          v2 = 4;
        }
      }
      else if (v1 != 0x2000) { // branch-flip
        v2 = 2;
        if ((v1 != 0x4000) && (v2 = 5, v1 != 0x1000))
          return 0;
      }
      else {
        v2 = 1;
      }
    }
    else {
      v2 = 0;
    }
    v3 = *(char *)(a2 + 0x38 + v2);
  }
  return v3;
}


// Function: sub_98f0 @ 0x98f0
unsigned long sub_98f0(unsigned long a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)(a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x1c) < *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x1c));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)(a1 + 0x1c) == *(unsigned long *)(a2 + 0x40)) & 0xffffffff;
}


// Function: sub_9940 @ 0x9940
unsigned long sub_9940(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rbx
  long v2; // rdi
  char v3 [16];
  char v4 [16];
  
  v3 = sub_25c40(a1);
  v4 = sub_25c50(a1);
  if (0 <= (int)sub_8790(SUB168(v3,0),SUB168(v3,8),SUB168(v4,0),SUB168(v4,8))) {
    v2 = SUB168(v4,0) - SUB168(v3,0);
    v1 = SUB168(v4,8) - SUB168(v3,8);
    if (v1 < 0) {
      v1 += 1000000000;
      v2 -= 1;
    }
    return sub_87d0(v2,v1,a2,0x15180); // tail-call
  }
  return 0;
}


// Function: sub_99d0 @ 0x99d0
unsigned int sub_99d0(unsigned long a0,long a1,long a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x1c) >> 8),*(int *)(a2 + 0x38) == *(int *)(a1 + 0x1c));
}


// Function: sub_99e0 @ 0x99e0
int * sub_99e0(unsigned long a0,unsigned long a1,unsigned long a2) // early-return, ternary
{
  char v1;
  int *v2;
  char v3; // al
  int v4; // eax
  int *v5; // rax
  void *v6; // rbx
  char v7 [152];
  
  v3 = sub_dc30();
  v6 = (v3) ? sub_db20 : sub_daa0; // branch-flip
  sub_d860(v7);
  v4 = (*v6)(dat_3e5f0,v7);
  if (v4) { // branch-flip
    v3 = sub_dc30();
    v1 = v3;
    v5 = __errno_location();
    v2 = v5;
    if ((!v1) || (*v5 != 2)) {
      v5 = (unsigned long)sub_dfb0(0,a0);
      error(0,*v2,"%s",v5);
      v5 = NULL;
      dat_3e604 = 1;
      return v5;
    }
    v5 = (unsigned long)sub_9810(a0,a1,a2);
  }
  else {
    v5 = (unsigned long)sub_9810(a0,v7,a2);
  }
  return v5;
}


// Function: sub_9af0 @ 0x9af0
unsigned long sub_9af0(unsigned long a0,unsigned long a1,long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // stack - 0x38
  unsigned long v5; // r12
  unsigned long v6; // r12
  
  v1 = (*dat_3e680)(dat_3e5f8,dat_3e5f0,&v4);
  if (0 <= v1) { // branch-flip
    v1 = fnmatch(*(char **)(a2 + 0x38),v4,0);
    v6 = CONCAT71((undefined7)((unsigned long)v5 >> 8),v1 == 0);
    freecon(v4);
  }
  else {
    v2 = sub_dfb0(0,a0);
    v3 = dcgettext(NULL,"getfilecon failed: %s",5);
    v6 = 0;
    error(0,*__errno_location(),v3,v2);
  }
  return v6 & 0xffffffff;
}


// Function: sub_9bc0 @ 0x9bc0
void sub_9bc0(void *a0,long a1)
{
  unsigned long v1;
  char v2 [264];
  
  if (a1) {
    do {
      v1 = *(unsigned long *)(a1 + 8);
      __fprintf_chk(a0,1,"[%s] ",sub_8a40(v1,v2));
      a1 = *(long *)(a1 + 0x108);
    } while (a1);
  }
  __fprintf_chk(a0,1,"\n");
}


// Function: sub_9c70 @ 0x9c70
void sub_9c70(void *a0,long a1)
{
  char *v1; // rcx
  char *v2; // r8
  char *v3; // r9
  
  if (a1) {
    sub_9e10(a0,*(unsigned long *)(a1 + 0x110));
    v3 = "[need inum] ";
    if (!*(char *)(a1 + 0x1c))
      v3 = "";
    v2 = "[need type] ";
    if (!*(char *)(a1 + 0x1b))
      v2 = "";
    v1 = "[call stat] ";
    if (!*(char *)(a1 + 0x1a))
      v1 = "";
    __fprintf_chk(a0,1,"%s%s%s",v1,v2,v3);
    sub_b5a0(a0,a1);
    __fprintf_chk(a0,1," [est success rate %.4g] ");
    if (dat_3e660 & 0x40) {
      __fprintf_chk(a0,1,"[real success rate %lu/%lu",*(unsigned long *)(a1 + 0x128),*(unsigned long *)(a1 + 0x120));
      if (*(long *)(a1 + 0x120)) // branch-flip
        __fprintf_chk(a0,1,"=%.4g] ");
      else {
        __fprintf_chk(a0,1,"=_] ");
      }
    }
    sub_9e10(a0,*(unsigned long *)(a1 + 0x118)); // tail-call
    return;
  }
}


// Function: sub_9e10 @ 0x9e10
void sub_9e10(void *a0,long *a1)
{
  if (!a1)
    return;
  while( true ) {
    if (((void *)*a1 != sub_88c0) && ((void *)*a1 != sub_8870)) break;
    if (a1[0x22]) goto label_9e5e;
    a1 = (long *)a1[0x23];
    if (!a1)
      return;
  }
  if ((!a1[0x22]) && (!a1[0x23])) {
    sub_9c70(a0,a1); // tail-call
    return;
  }
label_9e5e:
  __fprintf_chk(a0,1,"%s"," ( ");
  sub_9c70(a0,a1);
  __fprintf_chk(a0,1,"%s"," ) "); // tail-call
  return;
}


// Function: sub_9ee0 @ 0x9ee0
void sub_9ee0(unsigned long a0)
{
  if (!(dat_3e660 & 0x40))
    return;
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  sub_9c70(stderr,a0);
  __fprintf_chk(stderr,1,"\n"); // tail-call
}


// Function: sub_9f40 @ 0x9f40
void sub_9f40(long *a0)
{
  void *v1;
  int *v2;
  
  if (!a0)
    return;
  do {
    v1 = (void *)*a0;
    if (!v1)
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check"); // no-return
    v2 = (int *)a0[0x26];
    if (!v2)
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check"); // no-return
    if ((v1 != *(void **)&v2[6]) && (*(void **)&v2[6]))
      __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check"); // no-return
    switch(*v2) {
      case 0:
      case 2:
        if (!*v2)
          __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check"); // no-return
        __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,"pred_sanity_check"); // no-return
      case 1:
      case 3:
      case 4:
      case 5:
        if (*(char *)((long)a0 + 0x19))
          __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check"); // no-return
        if ((char)a0[3])
          __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check"); // no-return
        break;
      case 6:
        if (!(char)a0[3])
          __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check"); // no-return
        if (((v1 != sub_86f0) && (v1 != sub_8a20)) && (!*(char *)((long)a0 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    a0 = (long *)a0[0x21];
    if (!a0)
      return;
  } while( true );
}


// Function: sub_a120 @ 0xa120
bool sub_a120(long a0,unsigned int a1,unsigned long a2)
{
  int v1;
  unsigned int *v2; // rax
  
  v2 = (unsigned int *)sub_26990(0x10);
  *(unsigned int **)(a0 + 0xc0) = v2;
  *(unsigned long *)&v2[2] = 0;
  *v2 = sub_1e690(a1,a2,0);
  v1 = **(int **)(a0 + 0xc0);
  if (0 <= v1)
    sub_174f0(v1,1);
  return 0 <= v1;
}


// Function: sub_a190 @ 0xa190
unsigned long sub_a190(long a0)
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  void *v4; // rax
  unsigned long v5; // rax
  
  v1 = dat_3e5f0;
  if (*(int *)(a0 + 0x98))
    return 1;
  v3 = strchr(dat_3e5f0,0x2f);
  if (v3) {
    v4 = (void *)sub_177a0(v1);
    v2 = sub_a120(a0,dat_3e5f8,v4);
    free(v4);
    return (unsigned long)v2;
  }
  v5 = sub_a120(a0,dat_3e5f8,"."); // tail-call
  return v5;
}


// Function: sub_a220 @ 0xa220
unsigned int sub_a220(unsigned int a0,int *a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x3a
  unsigned int v4;
  char *v5;
  char *v6;
  unsigned short v7; // stack - 0x32
  
  v4 = 1;
  if ((char)a0) {
    v3 = 0x6c756e2f7665642f;
    v7 = 0x6c;
    if (0 <= close(0)) { // branch-flip
      v4 = a0;
      if ((int)sub_17b20(&v3,0) < 0) {
        v2 = sub_dfb0(0,&v3);
        error(0,*__errno_location(),"%s",v2);
      }
    }
    else {
      v4 = 0;
      v5 = dcgettext(NULL,"Cannot close standard input",5);
      error(0,*__errno_location(),v5);
    }
  }
  if (sub_25250(a1)) {
    v5 = "";
    v6 = "";
    if (*a1 < 0) {
      v5 = *(char **)&a1[2];
      v6 = ": ";
      if (!v5) {
        v5 = "";
        v6 = v5;
      }
    }
    v1 = dcgettext(NULL,"Failed to change directory%s%s",5);
    v4 = 0;
    error(0,*__errno_location(),v1,v6,v5);
  }
  return v4;
}


// Function: sub_a390 @ 0xa390
unsigned long sub_a390(char *a0,unsigned long a1,unsigned long *a2)
{
  unsigned long *v1;
  char *v10; // stack - 0x50
  unsigned char v11; // stack - 0x41
  char *v12; // stack - 0x40
  unsigned long *v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  unsigned long v6; // rax
  long v7; // rbx
  char v8;
  bool v9; // zf
  
  v11 = sub_df90(*a2);
  if (v11) { // branch-flip
    if (!sub_a190(&a2[7])) {
      v4 = sub_dfb0(0,a0);
      v5 = dcgettext(NULL,"Failed to save working directory in order to run a command on %s",5);
      error(1,*__errno_location(),v5,v4);
      return v6;
    }
    a0 = (char *)sub_176f0(dat_3e5f0);
    v12 = a0;
    v8 = (*a0 != '/') * '\x02';
    v10 = NULL;
    if (*a0 != '/')
      v10 = "./";
  }
  else {
    if (a2[0x1f] != dat_3e5c0)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v10 = NULL;
    v8 = '\0';
    v12 = NULL;
  }
  v3 = (unsigned long)*(unsigned char *)&a2[7];
  v2 = &a2[0x13];
  v1 = &a2[8];
  if (*(unsigned char *)&a2[7]) { // branch-flip
    sub_15000(v1,v2,a0,strlen(a0) + 1,v10,v8,0);
    if (*(int *)&a2[0x1a])
      dat_3e608 = 1;
    if (!v12) {
      return v3 & 0xffffffff; // return-dupe
    }
    if (!v11)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // return-dupe, no-return
  }
  else {
    v7 = 0;
    if (1 <= *(int *)&a2[0x1e]) {
      do {
        v3 = strlen(a0);
        v5 = *(char **)(a2[0x1d] + v7 * 8);
        v7 += 1;
        sub_15290(v1,v2,v5,strlen(v5),v10,v8,a0,v3,0);
      } while ((int)v7 < *(int *)&a2[0x1e]);
    }
    sub_14e90(v1,v2);
    v9 = !(*(unsigned int *)&a2[0x20] >> 8 & 0xff) && !(*(unsigned int *)&a2[0x20] & 0x7f);
    v3 = CONCAT71((undefined7)((unsigned long)v7 >> 8),v9);
    if (!v11) {
      if (v12) {
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      return v3 & 0xffffffff;
    }
    sub_25280(a2[0x1f]);
    if (!v12) {
      return v3 & 0xffffffff;
    }
    v11 = v9;
  }
  free(v12);
  v3 = (unsigned long)v11;
  return v3 & 0xffffffff;
}


// Function: sub_a610 @ 0xa610
int * sub_a610(unsigned long a0,char *a1,unsigned long a2,unsigned long *a3) // return-dupe x2
{
  long v1;
  unsigned int v10;
  unsigned long v11; // rbx
  char v2;
  int v3;
  int *v4;
  unsigned long v5;
  char *v6;
  char v7; // al
  int v8; // eax
  int *v9; // rax
  
  if (dat_3e660 & 0x20) {
    v11 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",*(long *)&a1[0x60] + -1);
    if (*(long *)&a1[0x60] != 1) {
      do {
        v1 = v11 * 8;
        v11 += 1;
        v9 = (unsigned long)sub_dfb0(0,*(unsigned long *)(*(long *)&a1[0x68] + v1));
        __fprintf_chk(stderr,1," %s",v9);
        v9 = (unsigned long)(*(long *)&a1[0x60] - 1);
      } while (v11 < v9);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  sub_17bc0(stdout);
  sub_17bc0(stderr);
  if (dat_3e01c) {
    dat_3e01c = 0;
    signal(0x11,0);
  }
  v8 = fork();
  v3 = v8;
  if (v8 != -1) {
    if (!v8) {
      if (!*(long *)&a1[0xc0])
        __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
      v7 = sub_a220(a1[0xbc]);
      if (v7) {
        v7 = sub_15dd0();
        if (v7)
          sub_15df0();
        v7 = sub_15470(a3);
        v2 = v7;
        v9 = __errno_location();
        v4 = v9;
        if (v2) // branch-flip
          *v9 = 7;
        else {
          execvp(*a3,a3);
        }
        v9 = (unsigned long)sub_dfb0(0,*a3);
        error(0,*v4,"%s",v9);
      }
      _exit(1); // no-return
    }
    while (v8 = waitpid(v3,(int *)&a1[200],0), v8 == -1) {
      v9 = __errno_location();
      v4 = v9;
      if (*v9 != 4) {
        v9 = (unsigned long)sub_dfb0(0,*a3);
        v5 = v9;
        v9 = (char *)dcgettext(NULL,"error waiting for %s",5);
        error(0,*v4,(char *)v9,v5);
        v9 = NULL;
        dat_3e604 = 1;
        return v9;
      }
    }
    v10 = *(unsigned int *)&a1[200];
    v7 = ((unsigned char)v10 & 0x7f) + 1;
    if ('\x02' <= v7) { // branch-flip
      v9 = (unsigned long)sub_24e50(0,dat_3e68c,*a3);
      v5 = v9;
      v9 = (char *)dcgettext(NULL,"%s terminated by signal %d",5);
      error(0,0,(char *)v9,v5,(unsigned long)(v10 & 0x7f));
    }
    else {
      v10 = v10 >> 8 & 0xff;
      if (dat_3e660 & 0x20)
        __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v3,(unsigned long)v10);
      if (!v10) {
        v9 = (int *)0x1;
        return v9;
      }
    }
    if (!*a1) {
      v9 = (int *)0x1;
      return v9;
    }
    v9 = (int *)0x1;
    dat_3e604 = 1;
    return v9;
  }
  v9 = (char *)dcgettext(NULL,"cannot fork",5);
  v6 = (char *)v9;
  v9 = __errno_location();
  error(1,*v9,v6);
  return v9;
}


// Function: sub_a950 @ 0xa950
unsigned int sub_a950(long *a0,unsigned long a1)
{
  void *v1;
  unsigned char v2; // al
  unsigned int v3; // eax
  unsigned long v4; // rax
  undefined7 v5; // rsi
  
  v1 = (void *)*a0;
  v5 = (undefined7)((unsigned long)a1 >> 8);
  v3 = (unsigned int)CONCAT71((undefined7)((unsigned long)v4 >> 8),v1 == sub_92e0) | (unsigned int)CONCAT71(v5,v1 == sub_95a0) | (unsigned int)CONCAT71(v5,v1 == sub_9170);
  v2 = (unsigned char)v3 | v1 == sub_91e0;
  if (((!v2) && (((v1 != sub_8870 && v1 != sub_92f0) && dat_3e688) && v1 != sub_8cd0)) && (v1 != sub_88c0))
    return CONCAT31((undefined3)((unsigned int)(int)a0[4] >> 8),(int)a0[4] == 0);
  return CONCAT31((undefined3)(v3 >> 8),v2);
}


// Function: sub_a9e0 @ 0xa9e0
void sub_a9e0(unsigned long *a0)
{
  a0[1] = 0;
  *a0 = 0;
}


// Function: sub_a9f0 @ 0xa9f0
void sub_a9f0(long *a0,long a1,unsigned long *a2)
{
  long v1;
  
  *a2 = *(unsigned long *)(a1 + 0x110);
  v1 = a0[1];
  *(long *)(a1 + 0x110) = *a0;
  *a0 = a1;
  if (v1)
    return;
  a0[1] = a1;
}


// Function: sub_aa20 @ 0xaa20
unsigned int sub_aa20(long a0,long a1,char a2) // return-dupe, early-return
{
  float v1;
  float v2;
  
  if (*(unsigned int *)(a0 + 0x20) != *(unsigned int *)(a1 + 0x20))
    return -(unsigned int)(*(unsigned int *)(a0 + 0x20) < *(unsigned int *)(a1 + 0x20)) | 1;
  v1 = *(float *)(a0 + 0x24);
  v2 = *(float *)(a1 + 0x24);
  if (v1 == v2)
    return 0;
  if (!a2)
    return ((unsigned int)(v1 < v2) - 1) + (unsigned int)(v1 < v2);
  if (v1 < v2)
    return 0xffffffff;
  return 1;
}


// Function: sub_aa70 @ 0xaa70
unsigned long sub_aa70(long a0)
{
  unsigned long v1; // rax
  
  while( true ) {
    if (!a0)
      return 0;
    if (*(unsigned char *)(a0 + 0x18)) break;
    v1 = sub_aa70(*(unsigned long *)(a0 + 0x110));
    if ((char)v1)
      return v1;
    a0 = *(long *)(a0 + 0x118);
  }
  return (unsigned long)*(unsigned char *)(a0 + 0x18);
}


// Function: sub_aab0 @ 0xaab0
unsigned int sub_aab0(long a0)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = sub_aab0(*(unsigned long *)(a0 + 0x110));
    v1 = v2;
    v2 = sub_aab0(*(unsigned long *)(a0 + 0x118));
    if (v2 <= v1)
      v2 = v1;
    if (v2 < *(unsigned int *)(a0 + 0x20))
      v2 = *(unsigned int *)(a0 + 0x20);
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_ab00 @ 0xab00
void sub_ab00(long a0)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118);
  *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118) = *(unsigned long *)(a0 + 0x118);
  *(unsigned long *)(a0 + 0x118) = v1;
}


// Function: sub_ab30 @ 0xab30
void sub_ab30(float a0)
{
  if (a0 <= dat_2c588)
    return;
}


// Function: sub_ab60 @ 0xab60
void sub_ab60(unsigned long a0,long a1,unsigned long *a2)
{
  *(unsigned long *)(a1 + 0x110) = *a2;
  *a2 = a0;
}


// Function: sub_ab70 @ 0xab70
unsigned int sub_ab70(long a0)
{
  unsigned int v1;
  
  v1 = dat_2c588;
  if (a0)
    v1 = *(unsigned int *)(a0 + 0x24);
  return v1;
}


// Function: sub_ab90 @ 0xab90
char * sub_ab90(int a0)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  int *v3; // rdx
  
  v2 = 0;
  v3 = (int *)0x3e020;
  do {
    if (*v3 == a0)
      return *(char **)(v2 * 0x10 + 0x3e028);
    v1 = (int)v2 + 1;
    v2 = (unsigned long)v1;
    v3 = &v3[4];
  } while (v1 != 0xb);
  return "unknown";
}


// Function: sub_abd0 @ 0xabd0
char * sub_abd0(short a0)
{
  long v1;
  long v2; // rax
  int v3; // ecx
  short v4;
  
  if (a0) {
    v4 = 1;
    v3 = 0;
    v1 = 0x3c040;
    while( true ) {
      v3 += 1;
      v2 = v1 + 0x10;
      if ((a0 == v4) || (v4 == -1)) break;
      v4 = *(short *)(v1 + 0x20);
      v1 = v2;
    }
    return *(char **)((long)v3 * 0x10 + 0x3c048);
  }
  return "no";
}


// Function: sub_ac30 @ 0xac30
char * sub_ac30(short a0)
{
  long v1;
  long v2; // rax
  int v3; // ecx
  short v4;
  
  if (a0) {
    v4 = 1;
    v3 = 0;
    v1 = 0x3c0c0;
    while( true ) {
      v3 += 1;
      v2 = v1 + 0x10;
      if ((a0 == v4) || (v4 == -1)) break;
      v4 = *(short *)(v1 + 0x20);
      v1 = v2;
    }
    return *(char **)((long)v3 * 0x10 + 0x3c0c8);
  }
  return "no";
}


// Function: sub_ac90 @ 0xac90
void sub_ac90(long a0,char a1)
{
  long v1;
  
  while( true ) {
    if ((a1) && (*(int *)(a0 + 0x10) != 3))
      __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
    v1 = *(long *)(a0 + 0x110);
    if (v1) {
      if (*(int *)(v1 + 0x10) != 3)
        __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
      sub_ac90(v1,0);
    }
    a0 = *(long *)(a0 + 0x118);
    if (!a0) break;
    a1 = '\0';
  }
}


// Function: sub_ad10 @ 0xad10
char * sub_ad10(long *a0,long a1,short a2)
{
  short v1;
  unsigned int v2;
  char *v3; // rax
  long v4;
  
  v4 = *a0;
  if ((!v4) || (*(int *)(v4 + 0x10) == 5)) {
    v3 = NULL;
    return v3;
  }
  while( true ) {
    if (*(int *)(v4 + 0x14) <= (int)a2) {
      v3 = (char *)a1;
      return v3;
    }
    v2 = *(unsigned int *)(v4 + 0x10);
    if (v2 == 4) break;
    if (4 < v2) {
      if (v2 == 5) {
        v3 = (char *)a1;
        return v3;
      }
      v3 = dcgettext(NULL,"oops -- invalid expression type (%d)!",5);
      error(1,0,v3,(unsigned long)v2);
      return v3;
    }
    if (v2 != 3) break;
    *(long *)(v4 + 0x110) = a1;
    a1 = *a0;
    v1 = *(short *)(a1 + 0x14);
    *a0 = *(long *)(a1 + 0x108);
    v3 = (unsigned long)sub_ae10(a0,(int)v1);
    *(char **)(a1 + 0x118) = v3;
    v4 = *a0;
    if (!v4) {
      v3 = (char *)a1;
      return v3;
    }
  }
  v3 = dcgettext(NULL,"invalid expression",5);
  error(1,0,v3);
  return v3;
}


// Function: sub_ae10 @ 0xae10
char * sub_ae10(long *a0,short a1,long a2)
{
  int v1;
  long v2;
  unsigned long v3;
  char *v4; // rax
  long *v5;
  long v6;
  
  v6 = *a0;
  if (!v6) {
    v4 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v4);
    return v4;
  }
  v5 = a0;
  switch(*(unsigned int *)(v6 + 0x10)) {
    case 0:
label_b048:
      v4 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v4);
label_b06c:
      v4 = dcgettext(NULL,"oops -- invalid expression type!",5);
      error(1,0,v4);
      return v4;
    case 1:
      v2 = *(long *)(v6 + 0x108);
      *a0 = v2;
      break;
    case 2:
label_aeb0:
      *v5 = *(long *)(v6 + 0x108);
      v4 = (unsigned long)sub_ae10();
      *(char **)(v6 + 0x118) = v4;
      v2 = *a0;
      break;
    case 3:
      goto label_b01c;
    case 4:
      v2 = *(long *)(v6 + 0x108);
      if ((!v2) || (*(char *)(v2 + 0x29))) {
        v3 = *(unsigned long *)(v6 + 8);
        v4 = dcgettext(NULL,"invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'",5);
        error(1,0,v4,v3);
        return v4;
      }
      v1 = *(int *)(v2 + 0x10);
      *a0 = v2;
      if (v1 != 5) {
label_af48:
        v4 = (long)sub_ae10();
        v6 = *a0;
        if ((!v6) || (*(int *)(v6 + 0x10) != 5)) {
          v4 = dcgettext(NULL,"invalid expression; I was expecting to find a \')\' somewhere but did not see one.",5);
          error(1,0,v4);
          return v4;
        }
        v2 = *(long *)(v6 + 0x108);
        *a0 = v2;
        v6 = (long)v4;
        break;
      }
      if (*(char *)(v6 + 0x29)) {
        v6 = *(long *)(v2 + 8);
        v4 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        v5 = (long *)0x1;
        error(1,0,v4,v6);
        goto label_aeb0;
      }
      goto label_aff8;
    case 5:
      if (!a2) {
        v3 = *(unsigned long *)(v6 + 8);
        v4 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        error(1,0,v4,v3);
        return v4;
      }
      if (2 <= (unsigned int)(*(int *)(a2 + 0x10) - 2U)) { // branch-flip
        if (!*(char *)(v6 + 0x29)) {
          v4 = dcgettext(NULL,"invalid expression; you have too many \')\'",5);
          error(1,0,v4);
          goto label_af48;
        }
      }
      else if (!*(char *)(v6 + 0x29)) {
        v3 = *(unsigned long *)(a2 + 8);
        v4 = dcgettext(NULL,"expected an expression between \'%s\' and \')\'",5);
        error(1,0,v4,v3);
        return v4;
      }
      v6 = *(long *)(a2 + 8);
      v4 = dcgettext(NULL,"expected an expression after \'%s\'",5);
      error(1,0,v4,v6);
      goto label_afb8;
    default:
      goto label_b06c;
    
  }
  v4 = (char *)v2;
  if ((v4) && ((int)a1 < *(int *)((long)v4 + 0x14))) {
label_afb8:
    v4 = (long)sub_ad10(a0,v6);
    v6 = (long)v4;
    if (!v4) {
      v4 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v4);
      v6 = 0;
label_aff8:
      v4 = dcgettext(NULL,"invalid expression; empty parentheses are not allowed.",5);
      error(1,0,v4);
label_b01c:
      v3 = *(unsigned long *)(v6 + 8);
      v4 = dcgettext(NULL,"invalid expression; you have used a binary operator \'%s\' with nothing before it.",5);
      error(1,0,v4,v3);
      goto label_b048;
    }
  }
  v4 = (char *)v6;
  return v4;
}


// Function: sub_b160 @ 0xb160
void sub_b160(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  v2 = *a1;
  memcmp(&v1,&v2,8);
}


// Function: sub_b1c0 @ 0xb1c0
void sub_b1c0(long a0,int a1,long *a2)
{
  unsigned int v1;
  unsigned long *v2; // rax
  long v3; // rax
  unsigned int v4; // xmm0_da
  unsigned int v5; // xmm0_da
  
  v2 = (unsigned long *)sub_26c50(0x138);
  *(unsigned int *)&v2[2] = 3;
  *(int *)((long)v2 + 0x14) = a1;
  *(unsigned int *)&v2[4] = 0;
  if (a1 == 2) {
    v1 = *(unsigned int *)(a0 + 0x24);
    v2[1] = "-o";
    *v2 = sub_88c0;
    v3 = sub_ab30(v1);
    *(long *)(v3 + 0x118) = a0;
    *(unsigned int *)(v3 + 0x24) = v4;
    *a2 = v3;
    return;
  }
  if (a1 != 3) {
    if (a1 != 1) {
      v2[0x23] = a0;
      *a2 = (long)v2;
      return;
    }
    *a2 = (long)v2;
    v2[1] = ",";
    *v2 = sub_8cd0;
    *(unsigned int *)((long)v2 + 0x24) = 0x3f800000;
    v2[0x23] = a0;
    return;
  }
  v1 = *(unsigned int *)(a0 + 0x24);
  v2[1] = "-a";
  *v2 = sub_8870;
  v3 = sub_ab30(v1);
  *(long *)(v3 + 0x118) = a0;
  *(unsigned int *)(v3 + 0x24) = v5;
  *a2 = v3;
}


// Function: sub_b350 @ 0xb350
void * sub_b350(long *a0)
{
  long v1;
  unsigned int v2;
  void *v3; // rax
  void *v4 [3]; // stack - 0x38
  unsigned int v5;
  
  v5 = 3;
  if ((!*(char *)((long)a0 + 0x1a)) && (v5 = 1, !*(char *)((long)a0 + 0x1c)))
    v5 = (unsigned int)*(unsigned char *)((long)a0 + 0x1b) * 2;
  v3 = (void *)*a0;
  if ((v3 != sub_8fe0) && (v3 != sub_8ff0)) {
    if (v3 != sub_fad0) { // branch-flip
      if (!dat_3e6b0) {
        qsort(0x3e0e0,0x3b,0x10,sub_b160);
        if (!sub_b2d0()) {
          error(1,0,"failed to sort the costlookup array");
          return v3;
        }
        dat_3e6b0 = 1;
        v3 = (void *)*a0;
      }
      v4[0] = v3;
      v3 = (long)bsearch(v4,0x3e0e0,0x3b,0x10,sub_b160);
      if (v3) // branch-flip
        v2 = *(unsigned int *)((long)v3 + 8);
      else {
        v1 = a0[1];
        v3 = (char *)dcgettext(NULL,"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",5);
        error(0,0,(char *)v3,v1);
        v2 = 10;
      }
    }
    else {
      v2 = *(unsigned int *)&a0[4];
    }
    if (v2 <= v5)
      v2 = v5;
  }
  else {
    v2 = 8 - (unsigned int)*(unsigned char *)&a0[7];
  }
  v3 = (unsigned long)(unsigned long)v2;
  return v3;
}


// Function: sub_b4d0 @ 0xb4d0
void sub_b4d0(long a0)
{
  if (a0) {
    sub_b4d0(*(unsigned long *)(a0 + 0x118));
    sub_b4d0(*(unsigned long *)(a0 + 0x110));
    *(unsigned int *)(a0 + 0x20) = sub_b350(a0);
    return;
  }
}


// Function: sub_b510 @ 0xb510
unsigned long sub_b510(char *a0,unsigned char a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3; // rbx
  
  if (!dat_3e6b8) {
    v2 = fnmatch(a0,".",(unsigned int)a1 << 4);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
  }
  v3 = 0;
  do {
    v1 = *(char **)(dat_3e6c0 + v3 * 8);
    if (!fnmatch(a0,v1,(unsigned int)a1 << 4))
      return 1;
    v3 += 1;
  } while (v3 < dat_3e6b8);
  return 0;
}


// Function: sub_b5a0 @ 0xb5a0
void sub_b5a0(void *a0,long a1)
{
  if (*(long *)(a1 + 0x30)) {
    __fprintf_chk(a0,1,"%s %s",*(unsigned long *)(a1 + 8)); // tail-call
    return;
  }
  __fprintf_chk(a0,1,"%s",*(unsigned long *)(a1 + 8)); // tail-call
}


// Function: sub_b5e0 @ 0xb5e0
unsigned long sub_b5e0(void)
{
  return dat_3e6d0;
}


// Function: sub_b5f0 @ 0xb5f0
unsigned long sub_b5f0(long *a0)
{
  unsigned int v1;
  long v2;
  long v3;
  void *v4;
  float v5;
  unsigned long v6; // xmm0_qa
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  v2 = a0[0x23];
  if (v2)
    sub_b5f0(v2);
  v3 = a0[0x22];
  if (v3)
    sub_b5f0(v3);
  v1 = *(unsigned int *)&a0[2];
  if (v1 == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if (v1 == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  if (v1 != 2) {
    if (2 < v1) {
      if (v1 != 3)
        __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
      v4 = (void *)*a0;
      if (v4 != sub_8870) { // branch-flip
        if (v4 != sub_8cd0) { // branch-flip
          if (v4 != sub_88c0)
            __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
          v5 = (float)sub_ab70(v2);
          v5 = (float)sub_ab70(v3) + v5;
        }
        else {
          v5 = dat_2c588;
        }
      }
      else {
        v5 = (float)sub_ab70(v2);
        v5 = (float)sub_ab70(v3) * v5;
      }
      v6 = sub_ab30(v5);
      *(int *)((long)a0 + 0x24) = (int)v6;
      return v6;
    }
    if (v1) { // branch-flip
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates"); // no-return
    }
    else {
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates"); // no-return
    }
    return (unsigned long)*(unsigned int *)((long)a0 + 0x24);
  }
  if ((void *)*a0 != sub_92f0)
    __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
  if (!v3) {
    v5 = dat_2c588 - *(float *)(v2 + 0x24);
    *(float *)((long)a0 + 0x24) = v5;
    return (unsigned long)(unsigned int)v5;
  }
  __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
}


// Function: sub_b8a0 @ 0xb8a0
void sub_b8a0(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 != 2) {
    v1 = sub_26c50(0x138);
    if (dat_3e6d8) // branch-flip
      *(long *)(dat_3e6c8 + 0x108) = v1;
    else {
      dat_3e6d8 = v1;
    }
    *(unsigned short *)(v1 + 0x1a) = 0x101;
    *(int **)(v1 + 0x130) = a0;
    *(unsigned long *)(v1 + 0x10) = 0;
    *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
    *(char *)(v1 + 0x28) = dat_3e62f;
    *(unsigned long *)(v1 + 0x20) = 0x3f8000000000000a;
    dat_3e6c8 = v1;
    sub_b2b0(v1);
    return;
  }
  __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
}


// Function: sub_b970 @ 0xb970
void sub_b970(void) // return-dupe
{
  long v1; // rax
  
  v1 = sub_b8a0();
  if (!v1)
    return;
  *(unsigned long *)(v1 + 0x30) = 0;
}


// Function: sub_b990 @ 0xb990
void sub_b990(unsigned long a0,unsigned long a1)
{
  long v1;
  unsigned long *v2; // rax
  
  v1 = dat_3e6a0;
  if (!dat_3e6a0) {
    v1 = sub_147f0("and");
    dat_3e6a0 = v1;
    if (!v1)
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  }
  if (dat_3e6c8) {
    if ((*(unsigned int *)(dat_3e6c8 + 0x10) & 0xfffffffb) != 1) { // branch-flip
      if (!*(unsigned int *)(dat_3e6c8 + 0x10)) {
        error(1,0,dcgettext(NULL,"oops -- invalid default insertion of and!",5));
        return;
      }
    }
    else {
      v2 = (unsigned long *)sub_b970(v1);
      v2[1] = "-a";
      *v2 = sub_8870;
      v2[6] = 0;
      v2[7] = 0;
      v2[2] = 0x300000003;
      *(unsigned int *)&v2[3] = 0;
      *(char *)((long)v2 + 0x1c) = 0;
    }
  }
  v1 = sub_b8a0(a0);
  *(unsigned long *)(v1 + 0x30) = a1;
  *(unsigned long *)(v1 + 0x130) = a0;
}


// Function: sub_baa0 @ 0xbaa0
void sub_baa0(void *a0,long a1,int a2)
{
  char v1;
  short v2;
  unsigned int v3;
  char *v4; // rcx
  int v5; // esi
  
  if (!a1)
    return;
  do {
    if (1 <= a2) {
      v5 = 0;
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    __fprintf_chk(a0,1,"pred=[");
    sub_b5a0(a0,a1);
    sub_ac30((int)*(short *)(a1 + 0x14));
    v2 = *(short *)(a1 + 0x10);
    __fprintf_chk(a0,1,"] type=%s prec=%s",sub_abd0((int)v2));
    v3 = *(unsigned int *)(a1 + 0x20);
    v5 = 1;
    __fprintf_chk(a0,v5," cost=%s est_success_rate=%#.4g %sside effects ",sub_ab90(v3));
    if (*(unsigned long *)(a1 + 0x18) & 0xffffff0000) {
      __fprintf_chk(a0,1,"Needs ");
      if (*(char *)(a1 + 0x1a)) { // branch-flip
        __fprintf_chk(a0,1,"stat");
        if (*(char *)(a1 + 0x1c)) { // branch-flip
          v4 = ",";
label_bcc1:
          __fprintf_chk(a0,1,"%sinode",v4);
          v1 = *(char *)(a1 + 0x1b);
        }
        else {
          v1 = *(char *)(a1 + 0x1b);
        }
        if (!v1) goto label_bba8;
        v4 = ",";
      }
      else {
        if (*(char *)(a1 + 0x1c)) {
          v4 = "";
          goto label_bcc1;
        }
        if (!*(char *)(a1 + 0x1b)) goto label_bba8;
        v4 = "";
      }
      __fprintf_chk(a0,1,"%stype",v4);
    }
label_bba8:
    __fprintf_chk(a0,1,"\n");
    if (1 <= a2) {
      v5 = 0;
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    if (*(long *)(a1 + 0x110)) { // branch-flip
      __fprintf_chk(a0,1,"left:\n");
      sub_baa0(a0,*(unsigned long *)(a1 + 0x110),a2 + 1);
    }
    else {
      if (!*(long *)(a1 + 0x118)) {
        v4 = "no children.\n";
        __fprintf_chk(a0,1,v4); // return-dupe, tail-call
        return;
      }
      __fprintf_chk(a0,1,"no left.\n");
    }
    v5 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v5 += 1;
      } while (a2 != v5);
    }
    if (!*(long *)(a1 + 0x118)) {
      v4 = "no right.\n";
      __fprintf_chk(a0,1,v4);
      return;
    }
    a2 += 1;
    __fprintf_chk(a0,1,"right:\n");
    a1 = *(long *)(a1 + 0x118);
    if (!a1)
      return;
  } while( true );
}


// Function: sub_bdc0 @ 0xbdc0
void sub_bdc0(long *a0,unsigned long a1)
{
  long v1;
  unsigned long v2;
  long v3;
  long v4;
  long v5; // stack - 0x48
  long v6;
  long v7;
  long v8; // stack - 0x40
  
  v4 = *a0;
  if (!v4)
    return;
  if (dat_3e660 & 8) { // branch-flip
    __fprintf_chk(stderr,1,"%s:\n","predlist before merge sort");
    sub_baa0(stderr,*a0,2);
    v4 = *a0;
    sub_b5f0(v4);
    sub_a9e0(&v5);
    v7 = v5;
    if (!v4) goto label_be98;
  }
  else {
    sub_b5f0(v4);
    sub_a9e0(&v5);
  }
  v6 = v5;
  do {
    v1 = *(long *)(v4 + 0x110);
    *a0 = v1;
    *(unsigned long *)(v4 + 0x110) = 0;
    v7 = v4;
    if (v6) {
      v3 = v6;
      do {
        v2 = *(unsigned long *)(v3 + 0x118);
        if (0 <= (int)sub_aa20(v2)) {
          v4 = *(long *)(v3 + 0x110);
          *(long *)(v7 + 0x110) = v4;
          if (!v4)
            v8 = v7;
          *(long *)(v3 + 0x110) = v7;
          v7 = v6;
          goto label_be93;
        }
        v3 = *(long *)(v3 + 0x110);
      } while (v3);
    }
    *(long *)(v7 + 0x110) = v6;
    if (!v8)
      v8 = v7;
    v5 = v7;
label_be93:
    v4 = v1;
    v6 = v7;
  } while (v1);
label_be98:
  if (dat_3e660 & 8) {
    __fprintf_chk(stderr,1,"%s:\n","predlist after merge sort");
    sub_baa0(stderr,v7,2);
  }
  sub_b5f0(v7);
  sub_ab60(v7,v8,a1);
  sub_a9e0(a0); // tail-call
}


// Function: sub_c030 @ 0xc030
unsigned long sub_c030(long *a0)
{
  unsigned int v1;
  char v10 [176];
  char v11 [8];
  unsigned long v12; // stack - 0x108
  int v13;
  unsigned int v14;
  int v15;
  long *v16;
  void *v17;
  unsigned long v18;
  unsigned long *v19;
  long v2;
  unsigned long v20; // stack - 0x110
  unsigned long v21; // stack - 0x100
  char *v3;
  char *v4; // rax
  long v5;
  unsigned long v6; // rax
  char *v7;
  long v8;
  unsigned long v9; // stack - 0x118
  
  v5 = *a0;
  v9 = 0;
  v20 = 0;
  v12 = 0;
  v21 = 0;
  if (!v5)
    return 0;
  v7 = v11;
  v3 = v10;
  do {
    v4 = &v3[0x10];
    sub_a9e0(v3);
    v3 = v4;
  } while (v7 != v4);
  v8 = v5;
  v2 = *(long *)(v5 + 0x110);
  if (*(long *)(v5 + 0x110)) { // branch-flip
    while (v5 = v2, *(long *)(v5 + 0x110)) {
      v8 = v5;
      v2 = *(long *)(v5 + 0x110);
    }
    v14 = *(unsigned int *)(v8 + 0x14);
    v16 = (long *)(v8 + 0x110);
  }
  else {
    v14 = 3;
    v16 = a0;
  }
  if (*(int *)(v5 + 0x10) != 3) { // branch-flip
    sub_b1c0(v5,v14,v16);
    if (dat_3e660 & 9) goto label_c2e0;
  }
  else {
    if (!(dat_3e660 & 9)) {
      v5 = *a0;
      goto label_c0fe;
    }
label_c2e0:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    sub_baa0(stderr,*a0,0);
  }
  v5 = *a0;
  if (!v5) {
    v18 = 0;
    sub_bfd0(v10,0xb,&v12,&v9,a0); // return-dupe
    return v18;
  }
label_c0fe:
  v15 = *(int *)(v5 + 0x10);
  v13 = 0;
  if (v15 == 3)
    v13 = *(int *)(v5 + 0x14);
  v18 = 0;
  do {
    if ((v15 == 3) && (*(int *)(v5 + 0x14) != v13))
      v5 = sub_b1c0(v5,v13,a0);
    v19 = *(unsigned long **)(v5 + 0x118);
    v1 = *(unsigned int *)&v19[2];
    v17 = (void *)*v19;
    if (v1 != 2) { // branch-flip
      if (2 < v1) {
        if (v1 != 3) {
          error(1,0,dcgettext(NULL,"oops -- invalid expression type!",5));
          return v6;
        }
        *(char *)&v19[3] = sub_c030(v5 + 0x118);
        v19 = *(unsigned long **)(v5 + 0x118);
        goto label_c250;
      }
      if (v13 == 1) goto label_c250;
      if (*(char *)&v19[3]) goto label_c2a0;
      if (!sub_a950(v19)) {
        if (v17 == sub_96f0) {
          sub_a9f0(&v9,v5);
          goto label_c25e;
        }
        if ((2 <= dat_3e688) && (((v1 = *(unsigned int *)&v19[4], v1 - 1 < 2 && (!*(char *)((long)v19 + 0x1a))) || (dat_3e688 != 2)))) {
          if (dat_3e660 & 8) {
            __fprintf_chk(stderr,1,"-O%d: categorising predicate ");
            sub_b5a0(stderr,*(unsigned long *)(v5 + 0x118));
            v15 = 1;
            v14 = *(unsigned int *)(*(long *)(v5 + 0x118) + 0x20);
            __fprintf_chk(stderr,v15," by cost (%s)\n",sub_ab90(v14));
            v1 = *(unsigned int *)(*(long *)(v5 + 0x118) + 0x20);
          }
          sub_a9f0(&v10[(unsigned long)v1 * 0x10],v5);
          goto label_c25e;
        }
        goto label_c257;
      }
      if (dat_3e660 & 8) {
        __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",(unsigned long)dat_3e688);
        sub_b5a0(stderr,*(unsigned long *)(v5 + 0x118));
        __fprintf_chk(stderr,1," into name_list\n");
      }
      sub_a9f0(&v12,v5);
    }
    else {
      *(char *)&v19[3] = sub_c030(&v19[0x23]);
      v19 = *(unsigned long **)(v5 + 0x118);
label_c250:
      if (*(char *)&v19[3]) {
label_c2a0:
        v18 = 1;
        sub_bfd0(v10,0xb,&v12,&v9,a0);
      }
label_c257:
      a0 = (long *)(v5 + 0x110);
    }
label_c25e:
    v5 = *a0;
    if (!v5) {
      sub_bfd0(v10,0xb,&v12,&v9,a0);
      return v18;
    }
    v15 = *(int *)(v5 + 0x10);
  } while( true );
}


// Function: sub_c450 @ 0xc450
unsigned int sub_c450(long *a0)
{
  float v1;
  long v10;
  float v2;
  char v3; // al
  unsigned int v4; // eax
  int v5; // eax
  int v6; // eax
  unsigned long v7; // rcx
  long v8;
  char *v9; // r8
  
  v9 = "Not a binary operation";
  v7 = (unsigned long)((unsigned int)dat_3e660 & 8);
  if ((int)a0[2] != 3) goto label_c471;
  if ((!a0[0x22]) || (v10 = a0[0x23], !v10)) {
    v9 = "Doesn\'t have two arms";
    goto label_c471;
  }
  v8 = *(long *)(a0[0x22] + 0x118);
  if (!v8) {
    v9 = "Left arm has no child on RHS";
    goto label_c471;
  }
  v3 = sub_aa70(v8);
  if (v3) {
    v9 = "Left subtree has side-effects";
    goto label_c471;
  }
  v4 = sub_aa70(v10);
  if ((char)v4) {
    v9 = "Right subtree has side-effects";
    goto label_c471;
  }
  v5 = sub_aab0(v8);
  v6 = sub_aab0(v10);
  if (v5 < v6) {
    v9 = "efficient as-is";
    goto label_c471;
  }
  if (v5 == v6) {
    v1 = *(float *)(v8 + 0x24);
    v2 = *(float *)(v10 + 0x24);
    if (v7) { // branch-flip
      __fprintf_chk(stderr,1,"Success rates: l=%f, r=%f\n");
      v7 = (unsigned long)((unsigned int)dat_3e660 & 8);
      if ((void *)*a0 == sub_88c0) {
        v9 = "Operation is OR; right success rate >= left";
        if (v1 <= v2) goto label_c471;
        goto label_c5c0;
      }
      v9 = "Not \'AND\' or \'OR\'";
      if ((void *)*a0 != sub_8870) goto label_c471;
    }
    else {
      if ((void *)*a0 == sub_88c0) {
        if (v1 <= v2)
          return v4;
        sub_ab00(a0); // return-dupe
        return 1;
      }
      if ((void *)*a0 != sub_8870)
        return v4;
    }
    if (v2 <= v1) {
      v9 = "Operation is AND; right success rate <= left";
label_c471:
      if (!v7)
        return 0;
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",v9);
      sub_baa0(stderr,a0,0);
      return 0;
    }
  }
label_c5c0:
  if (v7) {
    __fprintf_chk(stderr,1,"Performing arm swap on:\n");
    sub_baa0(stderr,a0,0);
  }
  sub_ab00(a0);
  return 1;
}


// Function: sub_c6b0 @ 0xc6b0
unsigned long sub_c6b0(long a0) // return-dupe
{
  unsigned long v1;
  
  if (!a0)
    return 0;
  do {
    do {
    } while (sub_c450(a0));
    v1 = *(unsigned long *)(a0 + 0x110);
  } while ((sub_c6b0(v1)) || (v1 = *(unsigned long *)(a0 + 0x118), sub_c6b0(v1)));
  return 0;
}


// Function: sub_c6f0 @ 0xc6f0
int * sub_c6f0(int a0,long a1,int a2)
{
  long *v1;
  long *v10; // stack - 0x48
  unsigned long v11;
  int v12;
  bool v13; // zf
  long v14; // stack - 0x70
  long v2;
  char *v3;
  long v4;
  int *v5;
  char v6; // al
  int *v7; // rax
  int v8 [4]; // stack - 0x5c
  int v9; // stack - 0x4c
  
  v7 = (long)(long)a2;
  dat_3e6d8 = NULL;
  dat_3e6c0 = a1 + (long)v7 * 8;
  v8[0] = a0;
  v9 = a2;
  if (a2 < a0) {
    while (v6 = sub_dca0(*(unsigned long *)(a1 + (long)v7 * 8),1), !v6) {
      dat_3e6b8 += 1;
      v9 += 1;
      if (v8[0] <= v9) break;
      v7 = (long)(long)v9;
    }
  }
  v7 = (long)sub_147f0("(");
  v2 = (long)v7;
  v7 = (long)sub_147f0(")");
  v14 = (long)v7;
  v7 = (long)sub_147f0("print");
  v4 = (long)v7;
  if (!v2)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v14)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v7)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_13ec0(v2,a1,v8);
  *(char **)(dat_3e6c8 + 8) = "(";
  *(char *)((long)dat_3e6d8 + 0x29) = 1;
  sub_147d0(a1,v8[0]);
  sub_9f40(dat_3e6c8);
  if (v9 < v8[0]) {
    do {
      v7 = (long)(long)v9;
      dat_3e618 = 0;
      v6 = sub_dca0(*(unsigned long *)(a1 + (long)v7 * 8),0);
      v13 = v6 == '\0';
      v7 = (long)(long)v9;
      v11 = *(unsigned long *)(a1 + (long)v7 * 8);
      if (v13) {
        v7 = (char *)dcgettext(NULL,"paths must precede expression: `%s\'",5);
        error(0,0,(char *)v7,v11);
        v7 = (long)(long)v9;
        v3 = *(char **)(a1 + (long)v7 * 8);
        if (!access(v3,0)) {
          v11 = *(unsigned long *)(dat_3e6c8 + 8);
          v7 = (char *)dcgettext(NULL,"possible unquoted pattern after predicate `%s\'?",5);
          error(0,0,(char *)v7,v11);
        }
        exit(1); // no-return
      }
      v7 = (int *)sub_147f0(v11);
      v5 = v7;
      if (!v7) {
        v7 = (char *)dcgettext(NULL,"unknown predicate `%s\'",5);
        error(1,0,(char *)v7,v11);
        return v7;
      }
      v12 = v9;
      if (*v7 != 4) {
        v12 = v9 + 1;
        v9 = v12;
      }
      v6 = (**(void **)&v7[4])(v7,a1,&v9);
      if (!v6) {
        v2 = *(long *)(a1 + (long)v9 * 8);
        if (!v2) {
          v7 = (char *)dcgettext(NULL,"missing argument to `%s\'",5);
          error(1,0,(char *)v7,v11);
          return v7;
        }
        if ((*v5 == 4) && (v9 == v12)) {
          v7 = (char *)dcgettext(NULL,"invalid predicate `%s\'",5);
          error(1,0,(char *)v7,v11);
          return v7;
        }
        v7 = (char *)dcgettext(NULL,"invalid argument `%s\' to `%s\'",5);
        error(1,0,(char *)v7,v2,v11);
        return v7;
      }
      v7 = NULL;
      *(unsigned long *)(dat_3e6c8 + 8) = v11;
      if (v9 != v12)
        v7 = *(int **)(a1 + (long)v12 * 8);
      *(int **)(dat_3e6c8 + 0x30) = v7;
      sub_9f40(dat_3e6c8);
      sub_9f40(dat_3e6d8);
    } while (v9 < v8[0]);
  }
  sub_147e0(a1,v8[0],dat_3e6c8,dat_3e6d8);
  v1 = dat_3e6d8;
  if (dat_3e6d8[0x21]) { // branch-flip
    v6 = sub_dc60(dat_3e6d8[0x21]);
    if (v6) { // branch-flip
      sub_13690(v14,a1,v8);
      *(char **)(dat_3e6c8 + 8) = ")";
      *(char *)(dat_3e6c8 + 0x29) = 1;
      sub_9f40(dat_3e6c8);
      sub_12910(v4,a1,v8);
      *(char **)(dat_3e6c8 + 8) = "-print";
      *(char *)(dat_3e6c8 + 0x29) = 1;
      sub_9f40(dat_3e6c8);
      sub_9f40(dat_3e6d8);
    }
    else {
      v1 = &dat_3e6d8[0x21];
      v10 = dat_3e6d8;
      dat_3e6d8 = (long *)*v1;
      sub_9f40((long *)*v1);
      free(v10);
    }
  }
  else {
    v10 = dat_3e6d8;
    dat_3e6c8 = 0;
    dat_3e6d8 = NULL;
    free(v1);
    sub_12910(v4,a1,v8);
    *(char **)(dat_3e6c8 + 8) = "-print";
    sub_9f40(dat_3e6c8);
    sub_9f40(dat_3e6d8);
  }
  if (dat_3e660 & 9) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    sub_9bc0(stderr,dat_3e6d8);
  }
  sub_14670(dat_3e6d8);
  sub_9f40(dat_3e6d8);
  v10 = dat_3e6d8;
  v7 = (unsigned long)sub_ae10(&v10,0,0);
  dat_3e6d0 = v7;
  sub_b5f0(v7);
  if (!v10) {
    if (dat_3e660 & 9) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      sub_baa0(stderr,dat_3e6d0,0);
      v7 = (int *)dat_3e6d0;
    }
    sub_b4d0(v7);
    sub_c030(0x3e6d0);
    v11 = dat_3e6d0;
    sub_ac90(dat_3e6d0,1);
    sub_c6b0(v11);
    v11 = dat_3e6d0;
    sub_ac90(dat_3e6d0,1);
    if (dat_3e660 & 9) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      sub_baa0(stderr,dat_3e6d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      sub_9c70(stderr,dat_3e6d0);
      __fprintf_chk(stderr,1,"\n");
      v11 = dat_3e6d0;
    }
    v7 = (int *)v11;
    return v7;
  }
  if ((void *)*v10 == sub_8c30) {
    v7 = (char *)dcgettext(NULL,"you have too many \')\'",5);
    error(1,0,(char *)v7);
    return v7;
  }
  v2 = v10[1];
  if (v2) {
    v7 = (char *)dcgettext(NULL,"unexpected extra predicate \'%s\'",5);
    error(1,0,(char *)v7,v2);
    return v7;
  }
  v7 = (char *)dcgettext(NULL,"unexpected extra predicate",5);
  error(1,0,(char *)v7);
  return v7;
}


// Function: sub_cdf0 @ 0xcdf0
void sub_cdf0(long a0,void *a1)
{
  do {
    if (*(long *)(a0 + 0x110))
      sub_cdf0(*(long *)(a0 + 0x110),a1);
    (*a1)(a0);
    a0 = *(long *)(a0 + 0x118);
  } while (a0);
}


// Function: sub_ce30 @ 0xce30
void sub_ce30(long *a0) // return-dupe
{
  void *v1;
  
  v1 = (void *)*a0;
  if (((v1 != sub_9050 && v1 != sub_fad0) && v1 != sub_9010) && (v1 != sub_9090))
    return;
  a0[8] = 0;
}


// Function: sub_ce70 @ 0xce70
void sub_ce70(int a0)
{
  void *v1;
  long v2;
  char *v3;
  unsigned long *v4; // rbx
  char *v5;
  
  v1 = stdout;
  fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v1);
  v4 = (unsigned long *)0x3c158;
  if (a0) {
    v3 = "exec";
    v5 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
    for (; __fprintf_chk(stdout,1,"%-10s %s\n",v3,v5), v4 != (unsigned long *)0x3c218; v4 = &v4[3]) {
      v5 = (char *)v4[2];
      v3 = (char *)*v4;
    }
    return;
  }
  v5 = "exec";
  v2 = 0;
  do {
    if (v2) { // branch-flip
      __fprintf_chk(stdout,1,"%s%s",", ",v5);
      if (v2 + 1 == 9)
        return;
    }
    else {
      __fprintf_chk(stdout,1,"%s%s","",v5);
    }
    v5 = (char *)*v4;
    v4 = &v4[3];
    v2 += 1;
  } while( true );
}


// Function: sub_cf70 @ 0xcf70
void sub_cf70(long a0) // return-dupe
{
  sub_154f0(*(unsigned long *)(a0 + 0xc0),sub_d060,a0);
  if (*(long *)(a0 + 0xc0) == dat_3e5c0)
    return;
  sub_25280(*(long *)(a0 + 0xc0));
  free(*(void **)(a0 + 0xc0));
  *(unsigned long *)(a0 + 0xc0) = 0;
}


// Function: sub_cfc0 @ 0xcfc0
void sub_cfc0(long *a0) // return-dupe
{
  if (!a0)
    return;
  do {
    while( true ) {
      if (!dat_3e608)
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
      sub_cfc0(a0[0x22]);
      if (((((void *)*a0 == sub_9530) || ((void *)*a0 == sub_8ff0)) && ((char)a0[7])) && ((int)a0[0x1a])) break;
      a0 = (long *)a0[0x23];
      if (!a0)
        return;
    }
    sub_cf70(&a0[7]);
    a0 = (long *)a0[0x23];
  } while (a0);
}


// Function: sub_d060 @ 0xd060
unsigned long sub_d060(long a0)
{
  sub_14e90(a0 + 8,a0 + 0x60);
  return 0;
}


// Function: sub_d080 @ 0xd080
void sub_d080(long *a0)
{
  if (!a0)
    return;
  do {
    sub_d080(a0[0x22]);
    if ((((void *)*a0 == sub_8fe0) && ((char)a0[7])) && ((int)a0[0x1a]))
      sub_14e90(&a0[8],&a0[0x13]);
    a0 = (long *)a0[0x23];
  } while (a0);
}


// Function: sub_d0f0 @ 0xd0f0
void sub_d0f0(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  v1 = sub_25250(dat_3e5c0);
  if (!v1) {
    sub_25280(dat_3e5c0);
    free(dat_3e5c0);
    dat_3e5c0 = NULL;
    return;
  }
  v3 = "";
  v4 = "";
  if (*dat_3e5c0 < 0) {
    v3 = *(char **)&dat_3e5c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to restore initial working directory%s%s",5);
  error(0,*__errno_location(),v2,v4,v3);
  _exit(1); // no-return
}


// Function: sub_d1a0 @ 0xd1a0
unsigned long sub_d1a0(char *a0,void *a1,unsigned int a2)
{
  int v1;
  
  v1 = *__errno_location();
  if ((v1 != 2) && (v1 != 0x14))
    return (unsigned long)a2;
  if (dat_3e660 & 2)
    __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
  v1 = fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
  return v1;
}


// Function: sub_d220 @ 0xd220
void sub_d220(unsigned char *a0)
{
  unsigned char v1;
  int v2;
  int *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned char *v6; // stack - 0x38
  
  v1 = *a0;
  if (!v1) {
    error(1,0,dcgettext(NULL,"The -O option must be immediately followed by a decimal integer",5));
    return;
  }
  if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 8)) {
    error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
    return;
  }
  v3 = __errno_location();
  v2 = *v3;
  *v3 = 0;
  v4 = strtoul((char *)a0,(char **)&v6,10);
  if (v4) { // branch-flip
    if (!*v6) {
      if (v4 != 0xffffffffffffffff) { // branch-flip
        if (v4 <= 0xffff) goto label_d295;
      }
      else if (*v3) {
        v5 = dcgettext(NULL,"Invalid optimisation level %s",5);
        error(1,*v3,v5,a0);
        return;
      }
      error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v4);
      return;
    }
  }
  else {
    if (v6 == a0) {
      error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
      return;
    }
    if (!*v6) {
label_d295:
      dat_3e688 = (short)v4;
      *v3 = v2;
      return;
    }
  }
  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),a0);
}


// Function: sub_d3d0 @ 0xd3d0
undefined16 sub_d3d0(void)
{
  char v1 [16];
  long v2; // rdx
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  
  if (gettimeofday(&v3,NULL)) { // branch-flip
    v3 = time(0);
    if (v3 == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v2 = 0;
  }
  else {
    v2 = v4 * 1000;
  }
  v1._8_8_ = v2;
  v1._0_8_ = v3;
  return v1._0_16_;
}


// Function: sub_d450 @ 0xd450
void sub_d450(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_b990(a0,a2);
  v1 = *(unsigned long *)(a0 + 8);
  *v2 = a1;
  v2[1] = v1;
  v2[7] = 0;
  v2[2] = 1;
}


// Function: sub_d490 @ 0xd490
void sub_d490(long a0,unsigned long a1)
{
  if (*(long *)(a0 + 0x18)) {
    sub_d450(a0,*(long *)(a0 + 0x18),a1); // tail-call
    return;
  }
  __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary"); // no-return
}


// Function: sub_d4d0 @ 0xd4d0
void sub_d4d0(unsigned long a0)
{
  sub_d490(a0,0); // tail-call
}


// Function: sub_d4e0 @ 0xd4e0
void sub_d4e0(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_3eed0;
  if (a0) {
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
    exit(a0); // no-return
  }
  __fprintf_chk(stdout,1,dcgettext(NULL,"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n",5),v2);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOther common options:\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --help                   display this help and exit\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --version                output version information and exit\n\n",5),v1);
  sub_ce70(0);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nUse \'-D help\' for a description of the options, or see find(1)\n\n",5),v1);
  sub_17130(stdout,dat_3eed0);
  exit(0); // no-return
}


// Function: sub_d6f0 @ 0xd6f0
void sub_d6f0(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // stack - 0x50
  char v4 [2]; // stack - 0x42
  char *v5;
  unsigned long *v6; // r13
  long v7; // r15
  
  v3 = NULL;
  v4[0] = ',';
  v4[1] = '\0';
  v1 = strtok_r(a0,v4,&v3);
  if (!v1) {
    error(0,0,dcgettext(NULL,"Empty argument to the -D option.",5));
    sub_d4e0(1); // no-return
  }
  do {
    v6 = (unsigned long *)0x3c158;
    v5 = "exec";
    v7 = 0;
    while (strcmp(v5,v1)) {
      v7 += 1;
      if (v7 == 9) {
        v2 = sub_24e50(0,dat_3e68c,a0);
        error(0,0,dcgettext(NULL,"Ignoring unrecognised debug flag %s",5),v2);
        goto label_d7bb;
      }
      v5 = (char *)*v6;
      v6 = &v6[3];
    }
    dat_3e660 |= (long)*(int *)(v7 * 0x18 + 0x3c148);
label_d7bb:
    v1 = strtok_r(NULL,v4,&v3);
    if (!v1) {
      if (dat_3e660 & 0x10) {
        sub_ce70(1);
        exit(0); // no-return
      }
      return;
    }
  } while( true );
}


// Function: sub_d860 @ 0xd860
void sub_d860(void)
{
  return;
}


// Function: sub_d870 @ 0xd870
bool sub_d870(void)
{
  char v1 [130];
  char v2 [262];
  bool v3;
  float v4; // xmm1_da
  
  v3 = 1;
  if (!uname(v1)) {
    v4 = (float)(double)atof(v2);
    if (strcmp("Linux",v1)) { // branch-flip
      if (!strcmp("FreeBSD",v1))
        v3 = (float)s_2d2e3._1_4_ <= v4;
    }
    else {
      v3 = dat_2d2e0 <= v4;
    }
  }
  return v3;
}


// Function: sub_d940 @ 0xd940
void sub_d940(void)
{
  if (!dat_3e608)
    return;
  sub_cfc0(sub_b5e0());
  dat_3e608 = 0;
}


// Function: sub_d970 @ 0xd970
void sub_d970(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  dat_3e5c0 = (int *)sub_26990(0x10);
  v1 = sub_251f0(dat_3e5c0);
  if (!v1)
    return;
  v3 = "";
  v4 = "";
  if (*dat_3e5c0 < 0) {
    v3 = *(char **)&dat_3e5c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to save initial working directory%s%s",5);
  error(1,*__errno_location(),v2,v4,v3);
}


// Function: sub_da00 @ 0xda00
unsigned long sub_da00(char *a0,void *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_3e5f8 != -100) && (dat_3e5f8 < 0))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  if (dat_3e5e0) {
    v1 = fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
    return v1;
  }
  v1 = fstatat(dat_3e5f8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_d1a0(a0,a1,v1); // tail-call
  return v2;
}


// Function: sub_daa0 @ 0xdaa0
unsigned long sub_daa0(char *a0,void *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_3e5f8 != -100) && (dat_3e5f8 < 0))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat"); // no-return
  v1 = fstatat(dat_3e5f8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_d1a0(a0,a1,v1); // tail-call
  return v2;
}


// Function: sub_db20 @ 0xdb20
void sub_db20(char *a0,void *a1)
{
  if ((dat_3e5f8 <= -1) && (dat_3e5f8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  fstatat(dat_3e5f8,a0,a1,0x100); // tail-call
}


// Function: sub_db70 @ 0xdb70
void sub_db70(unsigned long a0,unsigned long a1)
{
  dat_3e6e0 += 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",a0);
  if (dat_3e668 == 1) {
    sub_daa0(a0,a1); // tail-call
    return;
  }
  if (dat_3e668 != 2) {
    if (dat_3e668)
      __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
    sub_db20(a0,a1); // tail-call
    return;
  }
  sub_da00(a0,a1); // tail-call
}


// Function: sub_dc30 @ 0xdc30
unsigned int sub_dc30(void) // early-return x2
{
  if (dat_3e668 == 1)
    return 1;
  if (dat_3e668 != 2)
    return 0;
  return CONCAT31((undefined3)((unsigned int)dat_3e5e0 >> 8),dat_3e5e0 == 0);
}


// Function: sub_dc60 @ 0xdc60
unsigned long sub_dc60(long a0)
{
  while( true ) {
    if (!a0)
      return 1;
    if (*(char *)(a0 + 0x19)) break;
    a0 = *(long *)(a0 + 0x108);
  }
  return 0;
}


// Function: sub_dca0 @ 0xdca0
unsigned long sub_dca0(char *a0,unsigned int a1) // return-dupe
{
  undefined7 v1; // rax
  
  v1 = (undefined7)((unsigned long)((long)*(int *)((unsigned long)(unsigned char)(*a0 - 0x21) * 4 + 0x2d230) + 0x2d230) >> 8);
  switch(*a0) {
    case 0x21:
    case 0x28:
      return CONCAT71(v1,a0[1] == '\0');
    default:
      return 0;
    case 0x29:
    case 0x2c:
      goto label_dcd0;
    case 0x2d:
      return CONCAT71(v1,a0[1] != '\0');
    
  }
label_dcd0:
  if (!a0[1])
    return (unsigned long)(a1 ^ 1);
  return 0;
}


// Function: sub_dd00 @ 0xdd00
int sub_dd00(int a0,long a1)
{
  char *v1;
  long v2; // rax
  int v3; // r12d
  
  if (2 <= a0) { // branch-flip
    v3 = 1;
    do {
      while( true ) {
        v2 = (long)v3;
        v1 = *(char **)(a1 + v2 * 8);
        if (strcmp("-H",v1)) break;
        v3 += 1;
        sub_146f0(2);
        if (a0 <= v3)
          return v3;
      }
      if (strcmp("-L",v1)) { // branch-flip
        if (strcmp("-P",v1)) { // branch-flip
          if (!strcmp("--",v1))
            return v3 + 1;
          if (strcmp("-D",v1)) { // branch-flip
            if (strncmp("-O",v1,2))
              return v3;
            sub_d220(&v1[2]);
          }
          else {
            v3 += 1;
            if (a0 <= v3) {
              error(0,0,dcgettext(NULL,"Missing argument after the -D option.",5));
              sub_d4e0(1); // no-return
            }
            sub_d6f0(*(unsigned long *)(a1 + 8 + v2 * 8));
          }
        }
        else {
          sub_146f0(0);
        }
      }
      else {
        sub_146f0(1);
      }
      v3 += 1;
    } while (v3 < a0);
  }
  else {
    v3 = 1;
  }
  return v3;
}


// Function: sub_de90 @ 0xde90
void sub_de90(unsigned short *a0)
{
  char v1; // al
  int v2; // eax
  char v3 [16];
  
  *(bool *)((long)a0 + 0x11) = getenv("POSIXLY_CORRECT") != NULL;
  v1 = sub_d870();
  *(unsigned int *)&a0[0x2e] = 0;
  *(char *)&a0[0x2c] = v1;
  v2 = isatty(0);
  *(char *)((long)a0 + 0xf) = 0;
  *(bool *)&a0[8] = v2 != 0;
  if (*(char *)((long)a0 + 0x11))
    *(char *)&a0[8] = 0;
  *(unsigned long *)&a0[2] = 0xffffffffffffffff;
  *a0 = 1;
  v3 = sub_d3d0();
  *(char *)&a0[0x1c] = 0;
  *(long *)&a0[0xc] = SUB168(v3,0);
  *(long *)&a0[0x10] = SUB168(v3,8);
  *(long *)&a0[0x18] = SUB168(v3,8);
  *(long *)&a0[0x14] = SUB168(v3,0) + -0x15180;
  *(unsigned short *)((long)a0 + 0xd) = 0;
  *(unsigned long *)&a0[0x20] = 0;
  *(unsigned int *)&a0[0x1e] = (-(unsigned int)(*(char *)((long)a0 + 0x11) == '\0') & 0x200) + 0x200;
  a0[0x34] = 2;
  if (!getenv("FIND_BLOCK_SIZE")) {
    *(char *)&a0[6] = 0;
    sub_146f0(0);
    *(unsigned int *)&a0[0x36] = 8;
    *(unsigned long *)&a0[0x38] = 0;
    *(char *)&a0[0x3c] = 0;
    return;
  }
  error(1,0,dcgettext(NULL,"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable",5));
}


// Function: sub_df90 @ 0xdf90
unsigned int sub_df90(void *a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == sub_9530) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == sub_8ff0);
}


// Function: sub_dfb0 @ 0xdfb0
void sub_dfb0(unsigned long a0,unsigned long a1)
{
  sub_24e50(a0,dat_3e68c,a1); // tail-call
}


// Function: sub_dfd0 @ 0xdfd0
void sub_dfd0(int a0,int a1,char a2,unsigned long a3) // return-dupe
{
  if (!a2) {
    error(a0,a1,"%s",sub_dfb0(0,a3));
    dat_3e604 = 1;
    return;
  }
  if (dat_3e618) {
    dat_3e618 = 1;
    return;
  }
  error(a0,a1,"%s",sub_dfb0(0,a3));
  dat_3e604 = 1;
  dat_3e618 = 1;
}


// Function: sub_e060 @ 0xe060
void sub_e060(unsigned int a0,unsigned long a1)
{
  sub_dfd0(0,a0,1,a1); // tail-call
}


// Function: sub_e080 @ 0xe080
unsigned long sub_e080(unsigned long a0,unsigned long a1,long a2) // return-dupe x2
{
  int v1; // eax
  unsigned long v2; // rax
  int *v3; // rax
  
  if (dat_3e5e4) {
    dat_3e5e4 = 0x101;
    dat_3e5e8 = *(unsigned int *)(a2 + 0x18);
    return 0;
  }
  v1 = (*dat_3e670)(a1,a2);
  if (!v1) {
    if (*(int *)(a2 + 0x18)) {
      dat_3e5e8 = *(int *)(a2 + 0x18);
      dat_3e5e4 = 0x101;
      return 0;
    }
    v2 = sub_24e50(0,dat_3e68c,a1);
    error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v2);
    dat_3e5e4 = 0x101;
    dat_3e5e8 = *(unsigned int *)(a2 + 0x18);
    dat_3e604 = 1;
    return 0;
  }
  v3 = __errno_location();
  if ((dat_3e62e) && (*v3 == 2))
    return 0xffffffff;
  sub_e060(*v3,a0);
  return 0xffffffff;
}


// Function: sub_e150 @ 0xe150
unsigned int sub_e150(unsigned int *a0,unsigned long a1,unsigned long a2,long a3,unsigned int a4) // return-dupe
{
  unsigned int v1; // eax
  unsigned int v2;
  
  v2 = *a0;
  if (v2) {
    if (((v2 & 0xf000) == 0xa000) && (v1 = sub_dc30(), (char)v1)) {
      if (sub_e080())
        return 0;
      dat_3e5e8 = *(unsigned int *)(a3 + 0x18);
      *a0 = dat_3e5e8;
      dat_3e5e4 = CONCAT11(1,dat_3e5e4);
      return v1;
    }
    *(unsigned int *)(a3 + 0x18) = v2;
    dat_3e5e4 = CONCAT11(1,dat_3e5e4);
    dat_3e5e8 = v2;
    return 1;
  }
  if ((char)a4) {
    dat_3e5e4 = 0;
    dat_3e5e8 = 0;
    return a4;
  }
  if (sub_e080(a1,a2,a3))
    return 0;
  dat_3e5e8 = *(unsigned int *)(a3 + 0x18);
  *a0 = dat_3e5e8;
  dat_3e5e4 = CONCAT11(1,dat_3e5e4);
  return 1;
}


// Function: sub_e230 @ 0xe230
int sub_e230(unsigned long a0,long a1,long a2)
{
  int v1; // eax
  
  if ((((!*(char *)(a2 + 0x1a)) || (dat_3e5e4)) && ((!*(char *)(a2 + 0x1b) || (dat_3e5e5)))) && ((!*(char *)(a2 + 0x1c) || (((*(long *)(a1 + 8) && (dat_3e5e5)) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) != 0x4000))))))
    return 0;
  v1 = sub_e080(a0,dat_3e5f0,a1);
  return -(unsigned int)(v1 != 0);
}


// Function: sub_e2a0 @ 0xe2a0
unsigned long sub_e2a0(unsigned long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long v1; // rax
  
  a2[0x24] = a2[0x24] + 1;
  if (((!(a2[3] & 0xffffff0000)) || (!sub_e230())) && (v1 = (*(void *)*a2)(a0,a1,a2), (char)v1)) {
    a2[0x25] = a2[0x25] + 1;
    return v1;
  }
  return 0;
}


// Function: sub_e310 @ 0xe310
void sub_e310(unsigned int a0,unsigned long a1)
{
  sub_dfd0(1,a0,1,a1);
  abort(); // no-return
}


// Function: sub_e340 @ 0xe340
void sub_e340(unsigned int a0,unsigned long a1)
{
  sub_dfd0(0,a0,0,a1); // tail-call
}


// Function: sub_e360 @ 0xe360
void sub_e360(void)
{
  char v1; // al
  int v2; // eax
  long v3; // rax
  
  v3 = sub_b5e0();
  if (v3) { // branch-flip
    sub_cdf0(v3,sub_d080);
    sub_d940();
    sub_e560(dat_3e610);
    sub_cdf0(v3,sub_ce30);
    sub_d0f0();
    v1 = sub_15dd0();
  }
  else {
    sub_e560(dat_3e610);
    sub_d0f0();
    v1 = sub_15dd0();
  }
  if (v1) {
    sub_15df0();
    sub_15da0();
  }
  v2 = sub_17bc0(stdout);
  if (v2 == -1) {
    sub_e340(*__errno_location(),"standard output"); // tail-call
    return;
  }
}


// Function: sub_e410 @ 0xe410
void sub_e410(unsigned int a0,unsigned long a1)
{
  dat_3e618 = 0;
  sub_dfd0(1,a0,0,a1);
  abort(); // no-return
}


// Function: sub_e440 @ 0xe440
unsigned long sub_e440(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}


// Function: sub_e470 @ 0xe470
unsigned long sub_e470(unsigned long *a0,unsigned long a1)
{
  return (*a0 ^ a0[1]) % a1;
}


// Function: sub_e490 @ 0xe490
void sub_e490(void *a0)
{
  long v1;
  int *v2; // rax
  
  v1 = *(long *)((long)a0 + 0x18);
  if (v1) {
    if (sub_17850(v1)) {
      v2 = __errno_location();
      sub_e410(*v2,*(unsigned long *)((long)a0 + 0x10)); // no-return
    }
  }
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}


// Function: sub_e4d0 @ 0xe4d0
unsigned long * sub_e4d0(char *a0)
{
  unsigned long *v1; // rax
  char *v2; // rax
  long v3; // rax
  unsigned long *v4;
  
  v1 = malloc(0x10);
  v4 = v1;
  if (v1) {
    v2 = strdup(a0);
    *v1 = v2;
    if (v2) { // branch-flip
      v3 = sub_1ac70(0xb,0,sub_e470,sub_e440,sub_e490);
      v1[1] = v3;
      if (!v3) {
        free((void *)*v1);
        v4 = NULL;
        free(v1);
      }
    }
    else {
      v4 = NULL;
      free(v1);
    }
  }
  return v4;
}


// Function: sub_e560 @ 0xe560
void sub_e560(unsigned long *a0)
{
  free((void *)*a0);
  sub_1ae30(a0[1]); // tail-call
}


// Function: sub_e580 @ 0xe580
unsigned long sub_e580(unsigned long *a0,char *a1)
{
  int v1;
  unsigned long *v2; // rax
  char *v3; // rax
  void *v4; // rax
  long v5; // rax
  int *v6; // rax
  unsigned long v7; // stack - 0xc8
  unsigned long v8;
  unsigned long v9; // stack - 0xc0
  
  v2 = malloc(0x20);
  if (v2) { // branch-flip
    v3 = strdup(a1);
    v2[2] = v3;
    if (v3) { // branch-flip
      v4 = (void *)sub_17da0(a1,*a0);
      v2[3] = v4;
      if (v4) { // branch-flip
        v1 = fileno(v4);
        if (v1 < 0)
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
        sub_174f0(v1,1);
        if (0 <= fstat(v1,&v7)) { // branch-flip
          v8 = a0[1];
          *v2 = v7;
          v2[1] = v9;
          v5 = sub_1aa10(v8,v2);
          if (v5) { // branch-flip
            sub_e490(v2);
            v8 = *(unsigned long *)(v5 + 0x18);
          }
          else {
            v8 = a0[1];
            if (sub_1b290(v8,v2)) // branch-flip
              v8 = v2[3];
            else {
              v6 = __errno_location();
              v1 = *v6;
              sub_e490(v2);
              *v6 = v1;
              v8 = 0;
            }
          }
        }
        else {
          v8 = 0;
          sub_e490(v2);
        }
      }
      else {
        sub_e490(v2);
        v8 = 0;
      }
    }
    else {
      free(v2);
      v8 = 0;
    }
  }
  else {
    v8 = 0;
  }
  return v8;
}


// Function: sub_e720 @ 0xe720
bool sub_e720(char a0)
{
  return (unsigned char)(a0 - 0x30U) <= 7;
}


// Function: sub_e730 @ 0xe730
int sub_e730(char a0) // early-return
{
  if (0x1b <= (unsigned char)(a0 + 0xa4U))
    return 0;
  return (int)"\\"[(unsigned char)(a0 + 0xa4U)];
}


// Function: sub_e750 @ 0xe750
char * sub_e750(int a0) // early-return x7
{
  if (a0 == 0x8000)
    return "f";
  if (a0 == 0x4000)
    return "d";
  if (a0 == 0xa000)
    return "l";
  if (a0 == 0xc000)
    return "s";
  if (a0 == 0x6000)
    return "b";
  if (a0 == 0x2000)
    return "c";
  if (a0 != 0x1000)
    return "U";
  return "p";
}


// Function: sub_e7d0 @ 0xe7d0
char sub_e7d0(char a0)
{
  if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)a0))
    return '\x01';
  return (strchr("ABCT",(int)a0) != NULL) * '\x02';
}


// Function: sub_e820 @ 0xe820
long sub_e820(long a0)
{
  long v1;
  unsigned char v2;
  long v3; // r12
  
  v3 = 0;
  while( true ) {
    v3 += 1;
    v2 = *(unsigned char *)(a0 + v3);
    if (!v2) break;
    if (!strchr("-+ #",(int)(char)v2)) break;
  }
  v1 = *(long *)__ctype_b_loc();
  while (*(unsigned char *)(v1 + 1 + (unsigned long)v2 * 2) & 8) {
    v3 += 1;
    v2 = *(unsigned char *)(a0 + v3);
  }
  if (v2 == 0x2e) {
    do {
      v3 += 1;
    } while (*(unsigned char *)(v1 + 1 + (unsigned long)*(unsigned char *)(a0 + v3) * 2) & 8);
  }
  return v3;
}


// Function: sub_e8c0 @ 0xe8c0
void sub_e8c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,long a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  void *v1;
  unsigned long v10; // stack - 0xa0
  unsigned long v11; // stack - 0x98
  unsigned long v12; // stack - 0x90
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  char v2; // al
  unsigned long v20; // stack - 0x18
  int *v3; // rax
  unsigned int v4; // stack - 0xd8
  char v5 [16];
  unsigned int v6; // stack - 0xd4
  char *v7; // stack - 0xd0
  char *v8; // stack - 0xc8
  unsigned long v9; // stack - 0xa8
  
  if (v2) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v1 = *(void **)(a8 + 8);
  v7 = &Stack0000000000000008;
  v4 = 0x10;
  v8 = v5;
  v6 = 0x30;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  if (0 <= __vfprintf_chk(v1,1,a9,&v4))
    return;
  v3 = __errno_location();
  sub_e340(*v3,*(unsigned long *)(a8 + 0x10));
}


// Function: sub_e9a0 @ 0xe9a0
void sub_e9a0(long a0)
{
  unsigned long v1;
  int *v2; // rax
  
  v1 = *(unsigned long *)(a0 + 8);
  if (!sub_17bc0(v1))
    return;
  v2 = __errno_location();
  sub_e340(*v2,*(unsigned long *)(a0 + 0x10)); // tail-call
}


// Function: sub_e9d0 @ 0xe9d0
void sub_e9d0(long a0,unsigned long a1,unsigned long a2)
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  int *v4; // rax
  
  v2 = *(unsigned long *)(a0 + 0x20);
  v1 = *(char *)(a0 + 0x18);
  v3 = *(unsigned long *)(a0 + 8);
  if (0 <= (int)sub_16960(v3,v2,v1,a1,a2))
    return;
  v4 = __errno_location();
  sub_e340(*v4,*(unsigned long *)(a0 + 0x10)); // tail-call
}


// Function: sub_eb50 @ 0xeb50
long sub_eb50(char *a0,int *a1,void *a2,unsigned long a3)
{
  unsigned char *v1;
  long v10; // rax
  unsigned long v11; // stack - 0x78
  long v12; // stack - 0x80
  long v13; // stack - 0x88
  unsigned int v14; // stack - 0x74
  unsigned long v15; // stack - 0x70
  unsigned long v16; // stack - 0x68
  unsigned long v17; // stack - 0x60
  unsigned long v18; // stack - 0x58
  unsigned long v19; // stack - 0x50
  unsigned long v2;
  unsigned long v20; // stack - 0x48
  int v3;
  long v4;
  char v5; // al
  int v6;
  char *v7; // rax
  unsigned long v8;
  char *v9;
  
  v7 = (char *)sub_26990(strlen(a0) + 2);
  *v7 = '_';
  memcpy(&v7[1],a0,strlen(a0) + 1);
  v3 = *a1;
  v20 = *(unsigned long *)&a1[0xc];
  v15 = *(unsigned long *)&a1[2];
  v16 = *(unsigned long *)&a1[4];
  v17 = *(unsigned long *)&a1[6];
  v18 = *(unsigned long *)&a1[8];
  v19 = *(unsigned long *)&a1[10];
  v6 = v3 + 0xb;
  if (0xb <= v3)
    v6 = v3 + -0xb;
  v14 = (unsigned int)((unsigned long)*(unsigned long *)a1 >> 0x20);
  v11 = CONCAT44(v14,v6);
  v9 = dat_3eb08;
  if (!dat_3eb08) {
    dat_3eb00 = 1;
    v9 = (char *)sub_26990(1);
    dat_3eb08 = v9;
  }
  while ((v8 = strftime(v9,dat_3eb00,v7,a1), !v8 || (dat_3eb00 <= v8))) {
    v9 = (char *)sub_26ad0(v9,0x3eb00,1);
    dat_3eb08 = v9;
  }
  v2 = v8 + 1 + a3;
  v13 = 0;
  v12 = 0;
  dat_3eb08 = (long)sub_269e0(v9,v2);
  dat_3eb00 = v2;
  v9 = (char *)sub_26990(v2);
  strftime(v9,dat_3eb00,v7,&v11);
  v4 = (long)dat_3eb08;
  v5 = sub_ea60(dat_3eb08,v9,&v13,&v12);
  if ((v5) && (v12 == 2)) {
    v10 = v13 + 2;
    v1 = (unsigned char *)(v4 + v10);
    if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)*v1 * 2) & 8)) {
      v8 = (v8 - v10) + 1;
      if (v10 + a3 + v8 != v2)
        __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
      memmove((void *)(v4 + v10 + a3),v1,v8);
      memcpy(v1,a2,a3);
    }
  }
  free(v7);
  free(v9);
  return v4 + 1;
}


// Function: sub_eda0 @ 0xeda0
char * sub_eda0(long a0,unsigned long a1,int a2)
{
  bool v1;
  unsigned long v10; // r13
  unsigned long v11; // stack - 0x70
  unsigned int v12; // stack - 0x5c
  bool v2;
  int v3;
  char *v4; // rax
  void *v5; // rax
  long v6; // stack - 0x78
  char v7 [40]; // stack - 0x58
  unsigned long v8; // stack - 0x64
  long v9;
  
  v6 = a0;
  v11 = a1;
  if (a2 != 0x2b) { // branch-flip
    v8._0_2_ = CONCAT11((char)a2,0x25);
    v8._0_3_ = (undefined3)(unsigned short)v8;
    if ((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned long)(unsigned int)(a2 - 0x40U) & 0x3f) & 1))) {
      v7[0] = '\0';
      v2 = 0;
      v1 = 0;
      v3 = 0;
    }
    else {
      v2 = 1;
      v1 = 1;
      v3 = __snprintf_chk(v7,0x20,1,0x20,".%09ld0",a1);
    }
    if (a2 == 0x40) goto label_ee0a;
  }
  else {
    v8 = 0x64252d6d252d5925;
    v12 = 0x54252b;
    v3 = __snprintf_chk(v7,0x20,1,0x20,".%09ld0",a1);
    v2 = 1;
  }
  v1 = v2;
  v5 = localtime(&v6);
  if ((v5) && (v4 = (char *)sub_eb50(&v8,v5,v7,(long)v3), v4))
    return v4;
label_ee0a:
  v9 = -v6;
  if (-v6 <= -1)
    v9 = v6;
  v4 = (char *)sub_1b800(v9,0x3eb21,0,1,1);
  if ((char *)0x3eb21 <= v4) {
    if ((char *)0x3ee37 <= v4)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v6 <= -1) {
      v4[-1] = '-';
      v4 = &v4[-1];
    }
    if (v1) {
      if ((char *)0x317 <= &v4[strlen(v4) - 0x3eb20])
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v10 = 0x316 - (long)&v4[strlen(v4) - 0x3eb20];
      if (v10 <= strlen(v7)) {
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)v3,v10,v7);
        if (v10 <= strlen(v7))
          __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      }
      strcat(v4,v7);
    }
    return v4;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}


// Function: sub_f050 @ 0xf050
unsigned long sub_f050(unsigned long a0,unsigned long a1) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  int v4; // eax
  unsigned int *v5; // rax
  unsigned long v6; // stack - 0x18
  unsigned long v7; // stack - 0x10
  
  v6 = a0;
  v7 = a1;
  v5 = localtime(&v6);
  if (!v5)
    return sub_eda0(v6,v7,0x40);
  v1 = v5[6];
  if ((int)v1 < 0)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (6 < (int)v1)
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  v2 = v5[4];
  if ((int)v2 < 0)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xb < (int)v2)
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  v3 = v5[2];
  if ((int)v3 < 0)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x17 < (int)v3)
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  if ((int)v5[1] <= 0x3b) {
    if (0x3d < (int)*v5)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v4 = __snprintf_chk((char *)0x3e700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",*(unsigned long *)((long)(int)v1 * 8 + 0x3c280),*(unsigned long *)((long)(int)v2 * 8 + 0x3c220),(unsigned long)v5[3],(unsigned long)v3,(unsigned long)v5[1],(unsigned long)*v5,v7,(unsigned long)(v5[5] + 0x76c));
    if (v4 <= 0x3ff)
      return 0x3e700;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}


// Function: sub_f310 @ 0xf310
void sub_f310(unsigned long a0,int *a1,char *a2,unsigned long *a3) // return-dupe
{
  int v1;
  char v10 [24];
  unsigned int v11;
  unsigned long v12; // r14
  char v13 [16];
  unsigned int v14; // stack - 0x340
  char v15; // stack - 0x2be
  unsigned long v2;
  unsigned long v3;
  unsigned long *v4; // rax
  char *v5;
  int *v6; // rax
  char *v7;
  char v8 [10];
  unsigned long v9; // stack - 0x360
  
  v1 = *a1;
  if (v1 == 1) {
    sub_ea10(*(unsigned long *)&a1[2],(long)a1[4],a0);
    sub_e9a0(a0); // tail-call
    return;
  }
  if (v1 != 2) {
    if (v1)
      return;
label_f378:
    sub_ea10(*(unsigned long *)&a1[2],(long)a1[4],a0); // tail-call
    return;
  }
  switch((char)a1[1]) {
    case 0:
      error(1,0,dcgettext(NULL,"error: %s at end of format string",5),"%");
      goto label_f34a;
    default:
      return;
    case 0x25:
      goto label_f378;
    case 0x44:
      v2 = sub_1b800(*a3,v8,0,1,1);
      break;
    case 0x46:
      v7 = (char *)sub_104c0(a3,a2);
      goto label_f4a3;
    case 0x47:
      goto label_f45f;
    case 0x48:
      v7 = (char *)sub_26990((long)(dat_3e5fc + 1));
      v12 = (unsigned long)dat_3e5fc;
      memcpy(v7,a2,v12);
      v7[v12] = '\0';
      sub_e8c0(a0,*(unsigned long *)&a1[2],v7);
      free(v7); // return-dupe, tail-call
      return;
    case 0x4d:
      sub_17d90(a3,v8);
      v15 = 0;
      sub_e8c0(a0,*(unsigned long *)&a1[2],v8);
      return;
    case 0x50:
      v7 = "";
      if (1 <= (int)dat_3e5e0)
        v7 = &(&a2[dat_3e5fc])[a2[dat_3e5fc] == '/'];
label_f4a3:
      a2 = v7;
      v2 = *(unsigned long *)&a1[2];
      sub_e9d0(a0,v2,a2); // return-dupe, tail-call
      return;
    case 0x53:
      sub_f2c0(a3[6],a3[8]);
      sub_e8c0(a0,*(unsigned long *)&a1[2]); // tail-call
      return;
    case 0x55:
      goto label_f415;
    case 0x59:
      v11 = *(unsigned int *)&a3[3] & 0xf000;
      if (v11 == 0xa000) {
        v1 = fstatat(dat_3e5f8,dat_3e5f0,v10,0);
        if (v1) {
          v6 = __errno_location();
          v2 = *(unsigned long *)&a1[2];
          v1 = *v6;
          if ((v1 != 2) && (v1 != 0x14)) {
            if (v1 != 0x28) {
              sub_e8c0(a0,v2,"?");
              v2 = sub_dfb0(0,a2);
              error(0,*v6,"%s",v2);
              return;
            }
            sub_e8c0(a0,v2,"L");
            return;
          }
          sub_e8c0(a0,v2,"N");
          return;
        }
        v2 = sub_e750(v14 & 0xf000);
        break;
      }
      v3 = sub_e750(v11); // return-dupe
      v2 = *(unsigned long *)&a1[2];
      sub_e8c0(a0,v2,v3); // return-dupe, tail-call
      return;
    case 0x5a:
      v1 = (*dat_3e680)(dat_3e5f8,dat_3e5f0,&v9);
      if (0 <= v1) {
        sub_e8c0(a0,*(unsigned long *)&a1[2],v9);
        freecon(v9);
        return;
      }
label_f34a:
      sub_e8c0(a0);
      v2 = sub_dfb0(0,a2);
      v7 = dcgettext(NULL,"getfilecon failed: %s",5);
      error(0,*__errno_location(),v7,v2);
      dat_3e604 = 1;
      return;
    case 0x61:
      v13 = sub_25c40(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
      goto label_f690;
    case 0x62:
      v2 = sub_1b800(a3[8],v8,0,0x200,0x200);
      break;
    case 99:
      v13 = sub_25c50(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
      goto label_f690;
    case 100:
      v2 = *(unsigned long *)&a1[2];
      v11 = dat_3e5e0;
      sub_e8c0(a0,v2,v11); // return-dupe, tail-call
      return;
    case 0x66:
      v7 = (char *)sub_176f0(a2);
      goto label_f84e;
    case 0x67:
      v4 = getgrgid(*(unsigned int *)&a3[4]);
      if (v4) {
label_f659:
        *(char *)(*(long *)&a1[2] + (long)a1[4]) = 0x73;
        v2 = *(unsigned long *)&a1[2];
        v3 = *v4;
        sub_e8c0(a0,v2,v3);
        return;
      }
      v2 = sub_1b800(*(unsigned int *)&a3[4],v8,0,1,1); // return-dupe
      sub_e8c0(a0,*(unsigned long *)&a1[2],v2); // return-dupe
      return;
    case 0x68:
      v7 = (char *)sub_26d30(a2);
      for (v5 = &v7[strlen(v7) - 1]; v7 <= v5; v5 = &v5[-1]) {
        if (*v5 != '/') {
          if ((v7 < v5) && (v5[1] == '/'))
            v5[1] = '\0';
          break;
        }
      }
      v5 = strrchr(v7,0x2f);
      if (!v5) {
        sub_e9d0(a0,*(unsigned long *)&a1[2],".");
        free(v7);
        return;
      }
      *v5 = '\0';
      sub_e9d0(a0,*(unsigned long *)&a1[2],v7); // return-dupe
      free(v7);
      return;
    case 0x69:
      v2 = sub_1b800(a3[1],v8,0,1,1);
      break;
    case 0x6b:
      v2 = sub_1b800(a3[8],v8,0,0x200,0x400);
      break;
    case 0x6c:
      if ((*(unsigned int *)&a3[3] & 0xf000) == 0xa000) {
        v7 = (char *)sub_17190(dat_3e5f8,dat_3e5f0);
        if (v7) {
          sub_e9d0(a0,*(unsigned long *)&a1[2],v7);
          free(v7);
          return;
        }
        sub_e060(*__errno_location(),a2);
        dat_3e604 = 1;
      }
      v7 = NULL;
      sub_e9d0(a0,*(unsigned long *)&a1[2],"");
      free(v7);
      return;
    case 0x6d:
      v2 = *(unsigned long *)&a1[2];
      v11 = *(unsigned int *)&a3[3] & 0xfff;
      sub_e8c0(a0,v2,v11);
      return;
    case 0x6e:
      v2 = sub_1b800(a3[2],v8,0,1,1);
      break;
    case 0x70:
      v2 = *(unsigned long *)&a1[2];
      sub_e9d0(a0,v2,a2);
      return;
    case 0x73:
      v2 = sub_1b800(a3[6],v8,0,1,1);
      break;
    case 0x74:
      v13 = sub_25c60(a3);
      v3 = SUB168(v13,8);
      v2 = SUB168(v13,0);
label_f690:
      v3 = sub_f050(v2,v3);
label_f58c:
      v2 = *(unsigned long *)&a1[2];
      sub_e8c0(a0,v2,v3);
      return;
    case 0x75:
      v4 = getpwuid(*(unsigned int *)((long)a3 + 0x1c));
      if (v4) goto label_f659;
      v2 = sub_1b800(*(unsigned int *)((long)a3 + 0x1c),v8,0,1,1); // return-dupe
      sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
      return;
    case 0x79:
      v11 = *(unsigned int *)&a3[3] & 0xf000;
      v3 = sub_e750(v11);
      goto label_f58c;
    
  }
label_f34a:
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
  v2 = sub_1b800(*(unsigned int *)((long)a3 + 0x1c),v8,0,1,1);
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
  v2 = sub_1b800(*(unsigned int *)&a3[4],v8,0,1,1);
  sub_e8c0(a0,*(unsigned long *)&a1[2],v2);
  return;
}


// Function: sub_fad0 @ 0xfad0
unsigned long sub_fad0(unsigned long a0,unsigned long a1,long a2) // return-dupe
{
  char v1;
  char v2;
  unsigned long v3; // rax
  int *v4;
  long v5; // rdx
  char v6 [16];
  
  v4 = *(int **)(a2 + 0x38);
  if (!v4)
    return 1;
  a2 += 0x38;
  do {
    while ((*v4 != 2 || (v1 = *(char *)((long)v4 + 5), !v1))) {
      sub_f310(a2,v4,a0,a1);
      v4 = *(int **)&v4[6];
      if (!v4)
        return 1;
    }
    v2 = (char)v4[1];
    if (v2 != 'C') { // branch-flip
      if ('C' < v2) {
        if (v2 != 'T')
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        v6 = sub_25c60(a1);
        v5 = SUB168(v6,8);
        v3 = SUB168(v6,0);
        goto label_fb35;
      }
      if (v2 == 'A') {
        v6 = sub_25c40(a1);
        v5 = SUB168(v6,8);
        v3 = SUB168(v6,0);
        goto label_fb35;
      }
      if (v2 != 'B') {
        __assert_fail("0","print.c",0x518,"pred_fprintf");
      }
      v6 = sub_25c70(a1);
      v5 = SUB168(v6,8);
      v3 = SUB168(v6,0);
      if ((v1 == '@') || (0 <= v5)) goto label_fb35;
      sub_e8c0(a2,*(unsigned long *)&v4[2],"");
    }
    else {
      v6 = sub_25c50(a1);
      v5 = SUB168(v6,8);
      v3 = SUB168(v6,0);
label_fb35:
      v3 = sub_eda0(v3,v5,(int)v1);
      sub_e8c0(a2,*(unsigned long *)&v4[2],v3);
    }
    v4 = *(int **)&v4[6];
    if (!v4)
      return 1;
  } while( true );
}


// Function: sub_fc20 @ 0xfc20
long sub_fc20(long *a0,char *a1,unsigned int a2,unsigned int a3,char a4,char a5,long a6)
{
  long v1;
  unsigned int v2; // eax
  unsigned int *v3; // rax
  char *v4; // rax
  unsigned long v5; // r14
  char v6; // stack - 0x39
  
  if (a4 == '{')
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment"); // no-return
  if (a4 == '[')
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment"); // no-return
  if (a4 == '(')
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment"); // no-return
  v5 = (unsigned long)(int)a2;
  v3 = (unsigned int *)sub_26990(0x20);
  *a0 = (long)v3;
  *v3 = a3;
  *(char *)&v3[1] = a4;
  *(char *)((long)v3 + 5) = a5;
  v3[6] = 0;
  v3[7] = 0;
  v3[4] = a2;
  v4 = (char *)sub_26990(v5 + 2);
  *(char **)&v3[2] = v4;
  v4 = &strncpy(v4,a1,v5)[v5];
  if (a3 < 2) {
    if (a4)
      __assert_fail("0 == format_char","print.c",0x61,"make_segment"); // no-return
    if (!a5) {
      v1 = *a0;
      *v4 = '\0';
      return v1 + 0x18;
    }
    __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment"); // no-return
  }
  if (a3 != 2)
    __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment"); // no-return
  switch(a4) {
    case 0x25:
    case 100:
      v6 = a4;
      goto label_fd69;
    default:
      v2 = 0;
      break;
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x46:
    case 0x47:
    case 0x4d:
    case 0x54:
    case 0x55:
    case 0x61:
    case 0x62:
    case 99:
    case 0x67:
    case 0x6b:
    case 0x6e:
    case 0x73:
    case 0x74:
    case 0x75:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x48:
    case 0x50:
    case 0x66:
    case 0x68:
    case 0x70:
      v6 = 's';
label_fd69:
      *v4 = v6;
      v4[1] = '\0';
      return *a0 + 0x18; // return-dupe
    case 0x53:
      *v4 = 'g';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x59:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 2;
      break;
    case 0x5a:
      *v4 = 's';
      v2 = 5;
      v4 = &v4[1];
      break;
    case 0x69:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1c) = 1;
      v2 = 1;
      break;
    case 0x6c:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 4;
      break;
    case 0x6d:
      *v4 = 'o';
      v4 = &v4[1];
      *(char *)(a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x79:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)(a6 + 0x1b) = 1;
      v2 = 2;
    
  }
  *v4 = '\0';
  if (*(unsigned int *)(a6 + 0x20) < v2)
    *(unsigned int *)(a6 + 0x20) = v2;
  return *a0 + 0x18;
}


// Function: sub_fef0 @ 0xfef0
unsigned long sub_fef0(unsigned long *a0,unsigned long a1,char *a2) // return-dupe x2
{
  unsigned long v1;
  char *v10;
  int v11; // r9d
  int v12; // r12d
  char *v13;
  unsigned int v14; // r14d
  long v15;
  char *v16;
  char v2;
  long v3; // rax
  long v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char v7;
  long v8; // stack - 0x48
  char *v9;
  
  v3 = sub_d450(a1,sub_fad0);
  v1 = a0[1];
  *(unsigned long *)(v3 + 0x38) = *a0;
  *(unsigned long *)(v3 + 0x40) = v1;
  v1 = a0[3];
  *(unsigned long *)(v3 + 0x48) = a0[2];
  *(unsigned long *)(v3 + 0x50) = v1;
  v1 = a0[4];
  *(unsigned int *)(v3 + 0x18) = 0x101;
  *(unsigned long *)(v3 + 0x58) = v1;
  *(unsigned int *)(v3 + 0x20) = 0;
  *(unsigned long *)(v3 + 0x38) = 0;
  v2 = *a2;
  if (!v2)
    return 1;
  v4 = v3 + 0x38;
  v10 = a2;
  do {
    while( true ) {
      v7 = v10[1];
      v12 = (int)a2;
      v13 = a2;
      if (v2 != '\\') break;
      if (v7 == 'c') {
        sub_fc20(v4,a2,(int)v10 - v12,1,0,0,v3);
        if (!*(char *)(v3 + 0x1a))
          return 1;
        if (3 <= *(unsigned int *)(v3 + 0x20))
          return 1;
        *(unsigned int *)(v3 + 0x20) = 3;
        return 1;
      }
      if (v7) { // branch-flip
        v14 = (unsigned int)v7;
        v9 = &v10[1];
        if (sub_e720(v14)) { // branch-flip
          v8 = 0;
          *v10 = sub_f270(v9,&v8);
          v15 = v8 + 2;
          v9 = &v10[v8 + 1];
        }
        else {
          v2 = sub_e730(v14);
          if (!v2) {
            v16 = &v10[2];
            error(0,0,dcgettext(NULL,"warning: unrecognized escape `\\%c\'",5),(unsigned long)v14);
            v10 = v9;
            goto label_ff7f;
          }
          *v10 = v2;
          v15 = 2;
        }
      }
      else {
        v15 = 1;
        error(0,0,dcgettext(NULL,"warning: escape `\\\' followed by nothing at all",5));
        v9 = v10;
      }
      v13 = &v10[v15];
      v4 = sub_fc20(v4,a2,((int)v10 - v12) + 1,0,0,0,v3);
      v2 = v9[1];
      a2 = v13;
      v10 = v13;
      v16 = v13;
      if (!v2) goto label_10011;
    }
    if (v2 != '%') // branch-flip
      v16 = &v10[1];
    else {
      if (!v7) {
        error(1,0,dcgettext(NULL,"error: %s at end of format string",5),v10);
        return v6;
      }
      v13 = &v10[1];
      if (v7 != '%') {
        v7 = v10[sub_e820(v10)];
        v13 = &v10[sub_e820(v10)];
      }
      v10 = v13;
      v14 = (unsigned int)v7;
      v15 = sub_e7d0(v14);
      if (v15) { // branch-flip
        if (!v10[v15 + -1]) {
          if (strchr("{[(",v14)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(unsigned long)v14); // return-dupe
            return v5;
          }
          if ((v15 != 2) || (v16 = "warning: format directive `%%%c\' should be followed by another character", v10[1])) goto label_100f0;
          goto label_100fc;
        }
        v11 = 0;
        if (v15 == 2)
          v11 = (int)v10[1];
        v13 = &v10[v15];
        v4 = sub_fc20(v4,a2,(int)v10 - v12,2,v14,v11,v3);
        v10 = &v10[v15 + -1];
      }
      else {
        if (strchr("{[(",v14)) {
          error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(unsigned long)v14);
          return v5;
        }
label_100f0:
        v16 = "warning: unrecognized format directive `%%%c\'";
label_100fc:
        v13 = &v10[1];
        error(0,0,dcgettext(NULL,v16,5),(unsigned long)v14);
        v4 = sub_fc20(v4,a2,(int)v13 - v12,0,0,0,v3);
      }
      v16 = v13;
    }
label_ff7f:
    v2 = v10[1];
    a2 = v13;
    v10 = v16;
  } while (v2);
label_10011:
  if (v16 <= v13)
    return 1;
  sub_fc20(v4,v13,(int)v16 - (int)v13,0,0,0,v3);
  return 1;
}


// Function: sub_102e0 @ 0x102e0
void sub_102e0(long a0)
{
  long v1;
  
  if (a0) {
    do {
      v1 = *(long *)(a0 + 0x30);
      sub_1dc40(a0);
      a0 = v1;
    } while (v1);
    return;
  }
}


// Function: sub_10310 @ 0x10310
void sub_10310(char a0)
{
  if (dat_3ee40) {
    if ((dat_3ee38 == '\x01') || (!a0))
      return;
    sub_102e0(dat_3ee40);
    dat_3ee40 = 0;
  }
  dat_3ee38 = a0;
  dat_3ee40 = sub_1dc90(a0);
}


// Function: sub_10360 @ 0x10360
unsigned long sub_10360(long a0) // early-return x2
{
  unsigned long v1; // rax
  unsigned long v2 [19]; // stack - 0xb8
  
  if (*(long *)(a0 + 0x20) != -1)
    return 0;
  sub_d860(v2);
  v1 = (*dat_3e670)(*(unsigned long *)(a0 + 8),v2);
  if (!(int)v1) {
    *(unsigned long *)(a0 + 0x20) = v2[0];
    return v1;
  }
  return 0xffffffff;
}


// Function: sub_104c0 @ 0x104c0
void sub_104c0(long *a0)
{
  if (dat_3ee58) {
    if ((dat_3ee50) && (*a0 == dat_3ee48))
      return;
    free(dat_3ee58);
  }
  dat_3ee48 = *a0;
  dat_3ee58 = (void *)sub_103e0(a0);
}


// Function: sub_10510 @ 0x10510
unsigned long sub_10510(char *a0)
{
  char *v1;
  long v2;
  
  if (strcmp("afs",a0)) {
    v2 = sub_10310(0);
    if (v2) {
      while( true ) {
        v1 = *(char **)(v2 + 0x18);
        if (!strcmp(a0,v1)) break;
        v2 = *(long *)(v2 + 0x30);
        if (!v2)
          return 0;
      }
    }
  }
  return 1;
}


// Function: sub_10580 @ 0x10580
void * sub_10580(long *a0)
{
  long v1;
  long v2;
  long v3; // rax
  void *v4; // rax
  unsigned long v5; // stack - 0x48
  long v6;
  long v7;
  void *v8;
  
  v5 = 0;
  v3 = sub_1dc90(0);
  if (v3) { // branch-flip
    v6 = 0;
    v7 = v3;
    v8 = NULL;
    do {
      while( true ) {
        v2 = v6 + 1;
        v1 = v2 * 8;
        v4 = (void *)sub_15710(v8,v1,&v5);
        if (v4) break;
        v4 = NULL;
        free(v8);
        v7 = *(long *)(v7 + 0x30);
        v8 = v4;
        if (!v7) goto label_1063a;
      }
      if (!sub_10360(v7)) {
        *(unsigned long *)((long)v4 + v1 + -8) = *(unsigned long *)(v7 + 0x20);
        v6 = v2;
      }
      v7 = *(long *)(v7 + 0x30);
      v8 = v4;
    } while (v7);
label_1063a:
    sub_102e0(v3);
    if (v4)
      *a0 = v6;
  }
  else {
    v4 = NULL;
    sub_102e0(0);
  }
  return v4;
}


// Function: sub_10690 @ 0x10690
long sub_10690(void)
{
  int v1;
  int *v2;
  int *v3; // rax
  int v4; // edx
  long v5;
  
  v4 = 0;
  v2 = (int *)0x3cd40;
  v5 = dat_3ee68;
  if (!dat_3ee68) {
    while( true ) {
      v5 = *(long *)&v2[2];
      v4 += 1;
      if (!v5) break;
      v3 = &v2[8];
      v1 = *v2;
      v2 = v3;
      if (v1 == 1) {
        dat_3ee68 = (long)v4 * 0x20 + 0x3cd20;
        return dat_3ee68;
      }
    }
  }
  return v5;
}


// Function: sub_106f0 @ 0x106f0
char sub_106f0(void) // early-return
{
  if (dat_3e631)
    return 0;
  return dat_3e630;
}


// Function: sub_10710 @ 0x10710
unsigned int sub_10710(float a0)
{
  unsigned int v1;
  
  if (dat_2ec74 <= a0) {
    v1 = dat_2ec6c;
    if ((dat_2c588 <= a0) && (a0 <= dat_2ec78))
      v1 = dat_2ec70;
    return v1;
  }
  return dat_2ec68;
}


// Function: sub_10760 @ 0x10760
void sub_10760(long a0,unsigned long a1)
{
  sub_10710(a0,a1,(dat_3e648 - a0) / 0x15180); // tail-call
}


// Function: sub_10790 @ 0x10790
unsigned long sub_10790(long a0,int *a1,long *a2)
{
  int v1;
  long v2;
  
  if (a0) {
    v1 = *a1;
    v2 = *(long *)(a0 + (long)v1 * 8);
    if (v2) {
      *a2 = v2;
      *a1 = v1 + 1;
      return 1;
    }
  }
  *a2 = 0;
  return 0;
}


// Function: sub_107d0 @ 0x107d0
void sub_107d0(unsigned long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long v1; // stack - 0x18
  
  sub_10790(a0,a1,&v1);
  *a2 = v1;
}


// Function: sub_10810 @ 0x10810
void sub_10810(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_107d0(a1,a2,&v1))
    return;
  dat_3e690 = v1;
}


// Function: sub_10860 @ 0x10860
unsigned long sub_10860(unsigned long *a0,unsigned int *a1)
{
  char *v1;
  
  v1 = (char *)*a0;
  if (*v1 == '+') {
    *a1 = 0;
    *a0 = &v1[1];
    return 1;
  }
  if (*v1 != '-') {
    *a1 = 2;
    return 1;
  }
  *a1 = 1;
  *a0 = &v1[1];
  return 1;
}


// Function: sub_108b0 @ 0x108b0
int * sub_108b0(long a0,int *a1)
{
  int v1;
  long v2;
  
  v2 = dat_3ee70;
  v1 = *a1;
  if (v1 != 2) {
    if (v1 != 1) { // branch-flip
      if (v1) { // branch-flip
        if (!dat_3ee70)
          dat_3ee70 = a0;
      }
      else if (dat_3ee70) {
        if (sub_106f0())
          error(0,0,dcgettext(NULL,"warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments.",5),a0,v2,a0);
      }
    }
    else {
      a1 = NULL;
    }
  }
  return a1;
}


// Function: sub_10950 @ 0x10950
unsigned long sub_10950(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if ((v1 != 2) && (v1 != 0x14))
    return (unsigned long)a3;
  if (dat_3e660 & 2)
    __fprintf_chk(stderr,1,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",a1);
  v2 = sub_254c0(a0,a1,a2); // tail-call
  return v2;
}


// Function: sub_109e0 @ 0x109e0
unsigned long sub_109e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_3e5e0) {
    v2 = sub_254c0(a0,a1,a2); // tail-call
    return v2;
  }
  v1 = sub_252a0(a0,a1,a2);
  if (!v1)
    return 0;
  v2 = sub_10950(a0 & 0xffffffff,a1,a2,v1); // tail-call
  return v2;
}


// Function: sub_10a40 @ 0x10a40
unsigned long sub_10a40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = sub_252a0(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_10950(a0 & 0xffffffff,a1,a2,v1); // tail-call
}


// Function: sub_10a80 @ 0x10a80
void sub_10a80(void)
{
  char *v1; // rdx
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_15e60("find");
  __printf_chk(1,dcgettext(NULL,"Features enabled: ",5));
  __printf_chk(1,"D_TYPE ");
  v1 = "enabled";
  if (!dat_3e678)
    v1 = "disabled";
  __printf_chk(1,"O_NOFOLLOW(%s) ",v1);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  if (1 <= (int)is_selinux_enabled())
    __printf_chk(1,"SELINUX ");
  v2 = 0;
  if (sub_86d0(&v2)) {
    __printf_chk(1,"FTS(");
    if (v2 & 0x200)
      __printf_chk(1,"FTS_CWDFD");
    __printf_chk(1,") ");
  }
  __printf_chk(1,"CBO(level=%d) ",(unsigned long)dat_3e688);
  __printf_chk(1,"\n");
  exit(0); // no-return
}


// Function: sub_10bf0 @ 0x10bf0
unsigned long sub_10bf0(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_10c20 @ 0x10c20
unsigned long sub_10c20(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  *(unsigned int *)(v1 + 0x18) = 1;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_10c50 @ 0x10c50
unsigned long sub_10c50(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  if (!dat_3e620)
    *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned short *)(v1 + 0x18) = 1;
  return 1;
}


// Function: sub_10c80 @ 0x10c80
unsigned long sub_10c80(void)
{
  *(unsigned int *)(sub_d4d0() + 0x24) = 0x3a83126f;
  return 1;
}


// Function: sub_10ca0 @ 0x10ca0
unsigned long sub_10ca0(void)
{
  long *v1; // rax
  unsigned int v2;
  
  v1 = (long *)sub_d4d0();
  *(unsigned int *)&v1[3] = 0;
  v2 = dat_2ec7c;
  if ((void *)*v1 != sub_9640)
    v2 = dat_2ec80;
  *(unsigned int *)((long)v1 + 0x24) = v2;
  return 1;
}


// Function: sub_10ce0 @ 0x10ce0
unsigned long sub_10ce0(void)
{
  *(unsigned int *)(sub_d4d0() + 0x24) = 0x3c23d70a;
  return 1;
}


// Function: sub_10d00 @ 0x10d00
unsigned long sub_10d00(void)
{
  long v1; // rax
  
  v1 = sub_d4d0();
  dat_3e620 = 0;
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_10d30 @ 0x10d30
char * sub_10d30(unsigned long a0,unsigned long a1,unsigned long a2,void *a3)
{
  char v1;
  unsigned long v10; // r13
  float v11;
  unsigned int v2;
  unsigned int v3;
  long v4;
  char v5; // al
  unsigned int v6;
  char *v7; // rax
  char *v8; // stack - 0x48
  char *v9; // r12
  
  v9 = "-xtype";
  if (a3 != sub_99e0)
    v9 = "-type";
  v6 = sub_107d0(a0,a1,&v8);
  v3 = v6;
  v10 = (unsigned long)v6;
  if (v5) {
    if (!*v8) {
      v7 = dcgettext(NULL,"Arguments to %s should contain at least one letter",5);
      error(1,0,v7,v9);
      return v7;
    }
    v7 = (long)sub_d450(a2,a3,v8);
    v4 = (long)v7;
    v6 = 0;
    if (a3 == sub_99e0)
      v6 = v3;
    *(unsigned int *)(v4 + 0x24) = 0;
    v2 = 0;
    if (a3 != sub_99e0)
      v2 = v3;
    *(char *)(v4 + 0x1a) = v5;
    *(char *)(v4 + 0x1b) = (char)v2;
    v1 = *v8;
    while (v1) {
      switch(v1) {
        case 0x44:
          v7 = dcgettext(NULL,"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",5);
          error(1,0,v7,v9,0x44);
          return v7;
        default:
          v7 = dcgettext(NULL,"Unknown argument to %s: %c",5);
          error(1,0,v7,v9,(unsigned long)(unsigned int)(int)v1);
          return v7;
        case 0x62:
          v7 = NULL;
          v11 = dat_2ec84;
          break;
        case 99:
          v7 = (char *)0x1;
          v11 = dat_2ec88;
          break;
        case 100:
          v7 = (char *)0x2;
          v11 = dat_2ec8c;
          break;
        case 0x66:
          v7 = (char *)0x3;
          v11 = dat_2ec90;
          break;
        case 0x6c:
          v7 = (char *)0x4;
          v11 = dat_2ec94;
          break;
        case 0x70:
          v7 = (char *)0x5;
          v11 = dat_2ec98;
          break;
        case 0x73:
          v7 = (char *)0x6;
          v11 = dat_2ec9c;
        
      }
      if (*(char *)(v4 + 0x38 + (long)v7)) {
        v7 = dcgettext(NULL,"Duplicate file type \'%c\' in the argument list to %s.",5);
        error(1,0,v7,(unsigned long)(unsigned int)(int)v1,v9);
        return v7;
      }
      *(float *)(v4 + 0x24) = v11 + *(float *)(v4 + 0x24);
      *(char *)(v4 + 0x38 + (long)v7) = 1;
      if (!v8[1]) {
        v7 = (char *)v10;
        return v7;
      }
      if (v8[1] != ',') {
        v7 = dcgettext(NULL,"Must separate multiple arguments to %s using: \',\'",5);
        error(1,0,v7,v9);
        return v7;
      }
      v1 = v8[2];
      v8 = &v8[2];
      if (!v1) {
        v7 = dcgettext(NULL,"Last file type in list argument to %s is missing, i.e., list is ending on: \',\'",5);
        error(1,0,v7,v9);
        return v7;
      }
    }
  }
  v7 = (char *)v10;
  return v7;
}


// Function: sub_10fe0 @ 0x10fe0
void sub_10fe0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_10d30(a1,a2,a0,sub_99e0); // tail-call
}


// Function: sub_11020 @ 0x11020
char * sub_11020(void)
{
  int v1; // eax
  char *v2; // rax
  
  if (dat_3ee60) {
    v2 = (char *)0x1;
    return v2;
  }
  v1 = fnmatch("foo","foo",0);
  if (!v1) {
    v1 = fnmatch("Foo","foo",0);
    if (v1) {
      v1 = fnmatch("Foo","foo",0x10);
      if (!v1) {
        dat_3ee60 = 1;
        v2 = (char *)0x1;
        return v2;
      }
    }
  }
  v2 = dcgettext(NULL,"sanity check of the fnmatch() library function failed.",5);
  error(1,0,v2);
  return v2;
}


// Function: sub_110c0 @ 0x110c0
unsigned int sub_110c0(char *a0,int a1)
{
  unsigned int v1;
  char *v2; // rax
  unsigned int v3;
  
  v2 = strpbrk(a0,"*?[");
  v1 = dat_2ec74;
  v3 = dat_2eca0;
  if ((!v2) && (v3 = v1, a1)) {
    if (strpbrk(a0,"."))
      return dat_2eca0;
  }
  return v3;
}


// Function: sub_11130 @ 0x11130
unsigned long sub_11130(char *a0,char a1)
{
  char *v1; // rax
  
  v1 = strrchr(a0,0x2f);
  if ((v1) && (!v1[1]))
    return sub_b510(a0,a1); // tail-call
  return 1;
}


// Function: sub_11180 @ 0x11180
unsigned int sub_11180(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,void *a4)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x48
  unsigned int v4; // xmm0_da
  
  sub_11020();
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d450(a0,a4,v3);
    *(unsigned short *)(v2 + 0x1a) = 0;
    *(unsigned long *)(v2 + 0x38) = v3;
    v4 = sub_110c0(v3,0);
    *(unsigned int *)(v2 + 0x24) = v4;
    if (dat_3e631)
      return v1;
    if (sub_11130(v3,a4 == sub_91e0))
      return v1;
    error(0,0,dcgettext(NULL,"warning: -%s %s will not match anything because it ends with /.",5),a3,v3);
    *(unsigned int *)(v2 + 0x24) = 0x322bcc77;
  }
  return v1;
}


// Function: sub_11280 @ 0x11280
void sub_11280(void)
{
  sub_11180(); // tail-call
}


// Function: sub_11300 @ 0x11300
char * sub_11300(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned int v2; // eax
  char *v3; // rax
  unsigned int v4;
  char *v5; // stack - 0x38
  unsigned long v6; // r12
  unsigned int v7;
  
  v2 = sub_107d0(a1,a2,&v5);
  v6 = (unsigned long)v2;
  if ((char)v2) {
    v3 = getpwnam(v5);
    endpwent();
    if (v3) // branch-flip
      v4 = *(unsigned int *)((long)v3 + 0x10);
    else {
      v3 = (unsigned long)strspn(v5,"0123456789");
      if ((!v3) || (v5[(long)v3])) {
        if (!*v5) {
          v3 = dcgettext(NULL,"The argument to -user should not be empty",5);
          error(1,0,v3);
          return v3;
        }
        v3 = (unsigned long)sub_24e50(0,dat_3e68c,v5);
        v1 = v3;
        v3 = dcgettext(NULL,"%s is not the name of a known user",5);
        error(1,0,v3,v1);
        return v3;
      }
      v2 = sub_16f40(v5,dat_3e68c);
      v4 = v2;
    }
    v3 = (long)sub_d490(a0,v5);
    *(unsigned int *)((long)v3 + 0x38) = v4;
    v7 = dat_2eca4;
    if (100 <= v4)
      v7 = dat_2ec7c;
    *(unsigned int *)((long)v3 + 0x24) = v7;
  }
  v3 = (char *)v6;
  return v3;
}


// Function: sub_11460 @ 0x11460
unsigned long sub_11460(unsigned long a0)
{
  *(unsigned int *)(sub_d490(a0,0) + 0x24) = 0x38d1b717;
  return 1;
}


// Function: sub_11480 @ 0x11480
unsigned int sub_11480(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x38
  
  sub_11020();
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    *(unsigned long *)(v2 + 0x38) = v3;
    *(float *)(v2 + 0x24) = (float)sub_110c0(v3,0) * dat_2ec74;
  }
  return v1;
}


// Function: sub_11520 @ 0x11520
unsigned int sub_11520(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    *(unsigned long *)(v2 + 0x38) = v3;
    *(float *)(v2 + 0x24) = (float)sub_110c0(v3,0) * dat_2ec74;
  }
  return v1;
}


// Function: sub_115b0 @ 0x115b0
char * sub_115b0(unsigned long a0,long a1,int *a2)
{
  int v1;
  unsigned long v2;
  char *v3; // rax
  
  if (!a1) {
    v3 = NULL;
    return v3;
  }
  v3 = (long)(long)*a2;
  if (*(long *)(a1 + (long)v3 * 8)) {
    if (1 <= (int)is_selinux_enabled()) {
      v3 = (long)sub_d490(a0,0);
      v1 = *a2;
      *(unsigned int *)((long)v3 + 0x24) = 0x3c23d70a;
      v2 = *(unsigned long *)(a1 + (long)v1 * 8);
      *(char *)((long)v3 + 0x1a) = 0;
      *(unsigned long *)((long)v3 + 0x38) = v2;
      v3 = (char *)0x1;
      *a2 = *a2 + 1;
      return v3;
    }
    v3 = dcgettext(NULL,"invalid predicate -context: SELinux is not enabled.",5);
    error(1,0,v3);
    return v3;
  }
  v3 = NULL;
  return v3;
}


// Function: sub_11650 @ 0x11650
unsigned long sub_11650(double a0,unsigned long a1,long a2,long a3,long a4,char *a5) // return-dupe
{
  unsigned long v1;
  double v10; // xmm0_qa
  char v2; // al
  unsigned int v3; // eax
  long v4; // rax
  unsigned long v5; // rax
  long v6; // rdx
  unsigned long v7 [2]; // stack - 0x60
  double v8; // stack - 0x50
  double v9; // stack - 0x48
  
  v7[0] = a1;
  v2 = sub_10860(v7,a2 + 4);
  v1 = v7[0];
  if (!v2)
    return 0;
  if (*(int *)(a2 + 4)) { // branch-flip
    if (*(int *)(a2 + 4) == 1)
      *(unsigned int *)(a2 + 4) = 0;
  }
  else {
    *(unsigned int *)(a2 + 4) = 1;
  }
  v3 = sub_26d90(v7[0],0,&v8,dat_3dff0);
  if (!(char)v3)
    return 0;
  v10 = (double)modf(a0 * v8,&v9) * dat_2ecb0;
  if (dat_2ecb0 <= v10)
    __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
  v4 = (long)((double)a3 - v9);
  *(long *)(a2 + 8) = v4;
  if (a3 < v4 != v9 < 0.0) {
    error(1,0,a5,v1);
    return v5;
  }
  v6 = (long)((double)a4 - v10);
  if (v10 <= (double)a4) {
    *(long *)(a2 + 0x10) = v6;
    return (unsigned long)v3;
  }
  *(long *)(a2 + 0x10) = v6 + 1000000000;
  *(long *)(a2 + 8) = v4 + -1;
  return (unsigned long)v3;
}


// Function: sub_11800 @ 0x11800
char * sub_11800(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1;
  unsigned long v10; // stack - 0x28
  char v2; // al
  unsigned int v3;
  char *v4; // rax
  unsigned long v5; // stack - 0x40
  unsigned long v6; // stack - 0x38
  bool v7; // zf
  unsigned int v8; // xmm0_da
  unsigned long v9; // stack - 0x30
  
  v3 = sub_107d0(a1,a2,&v5);
  v7 = v2 != '\0';
  if (v7) {
    v3 = sub_11650(v5,&v6,0,0,"arithmetic overflow while converting %s days to a number of seconds");
    v1 = v3;
    if (!v2) {
      v4 = dcgettext(NULL,"Invalid argument %s to -used",5);
      error(1,0,v4,v5);
      return v4;
    }
    v4 = (long)sub_d490(a0,v5);
    *(unsigned long *)((long)v4 + 0x38) = v6;
    *(unsigned long *)((long)v4 + 0x40) = v9;
    *(unsigned long *)((long)v4 + 0x48) = v10;
    sub_10710();
    *(unsigned int *)((long)v4 + 0x24) = v8;
    v3 = v1;
  }
  v4 = (char *)(unsigned long)v3;
  return v4;
}


// Function: sub_11920 @ 0x11920
unsigned int sub_11920(unsigned long a0,unsigned long a1,unsigned int *a2,unsigned int a3)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x50
  unsigned int v4; // stack - 0x48
  unsigned int v5;
  unsigned int v6; // stack - 0x44
  unsigned long v7; // stack - 0x40
  unsigned long v8; // stack - 0x38
  
  v5 = *a2;
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v4 = a3;
    v1 = sub_11650(dat_2ecc0,v3,&v4,dat_3e648 + 0x15180,dat_3e650,"arithmetic overflow while converting %s minutes to a number of seconds");
    if ((char)v1) { // branch-flip
      v2 = sub_d490(a0,v3);
      *(unsigned long *)(v2 + 0x38) = CONCAT44(v6,v4);
      *(unsigned long *)(v2 + 0x40) = v7;
      *(unsigned long *)(v2 + 0x48) = v8;
      v5 = sub_10760(v7);
      *(unsigned int *)(v2 + 0x24) = v5;
    }
    else {
      *a2 = v5;
    }
  }
  return v1;
}


// Function: sub_11a10 @ 0x11a10
void sub_11a10(void)
{
  sub_11920(); // tail-call
}


// Function: sub_11a40 @ 0x11a40
unsigned long sub_11a40(unsigned long a0,char a1,unsigned long *a2)
{
  char v1 [16];
  
  if (a1 == 'c') {
    v1 = sub_25c50(a0);
    *a2 = SUB168(v1,0);
    a2[1] = SUB168(v1,8);
    return 1;
  }
  if ('d' <= a1) { // branch-flip
    if (a1 == 'm') {
      v1 = sub_25c60(a0);
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return 1;
    }
  }
  else {
    if (a1 == 'B') {
      v1 = sub_25c70();
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return (unsigned long)~a2[1] >> 0x3f;
    }
    if (a1 == 'a') {
      v1 = sub_25c40(a0);
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return 1;
    }
  }
  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return
}


// Function: sub_11ae0 @ 0x11ae0
unsigned long sub_11ae0(long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x20
  char v3 [8];
  
  if ((a0) && (v2 = a0, sub_10860(&v2,a2,0,a1))) {
    v1 = sub_27300(v2,v3,10);
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
  }
  return 0;
}


// Function: sub_11b60 @ 0x11b60
long sub_11b60(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rax
  char *v2; // rcx
  unsigned long v3; // stack - 0x30
  int v4; // stack - 0x34
  unsigned long v5; // stack - 0x28
  char *v6; // r8
  
  v1 = 0;
  if ((sub_107d0(a0,a1,&v3)) && (sub_11ae0(v3,&v5,&v4))) {
    v1 = sub_d490(a2,v3);
    *(int *)(v1 + 0x38) = v4;
    *(unsigned long *)(v1 + 0x40) = v5;
    if (dat_3e660 & 1) {
      __fprintf_chk(stderr,1,"inserting %s\n",*(unsigned long *)(v1 + 8));
      v6 = " >";
      v2 = "gt";
      if (v4) {
        v6 = " <";
        v2 = "lt";
        if (v4 != 1) {
          v6 = " ?";
          if (v4 == 2)
            v6 = " =";
          v2 = "?";
          if (v4 == 2)
            v2 = "eq";
        }
      }
      __fprintf_chk(stderr,1,"    type: %s    %s  ",v2,v6);
      __fprintf_chk(stderr,1,"%lu\n",*(unsigned long *)(v1 + 0x40));
    }
  }
  return v1;
}


// Function: sub_11cb0 @ 0x11cb0
unsigned long sub_11cb0(unsigned long a0,unsigned long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_11b60(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(long *)(v1 + 0x40) != 1) {
    if (*(long *)(v1 + 0x40) != 2) {
      *(unsigned int *)(v1 + 0x24) = 0x3a83126f;
      return 1;
    }
    *(unsigned int *)(v1 + 0x24) = 0x3c23d70a;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = 0x3f7d70a4;
  return 1;
}


// Function: sub_11d20 @ 0x11d20
unsigned long sub_11d20(unsigned long a0,unsigned long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_11b60(a1,a2,a0);
  if (v1) {
    *(unsigned int *)(v1 + 0x24) = 0x358637bd;
    *(unsigned short *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1c) = 1;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}


// Function: sub_11de0 @ 0x11de0
char * sub_11de0(unsigned long a0,long a1,int *a2)
{
  char *v1;
  unsigned long v10; // r15
  unsigned int v11;
  char *v2;
  unsigned int v3; // eax
  char *v4; // rax
  char v5;
  int v6; // stack - 0x54
  long v7; // stack - 0x50
  char v8 [2]; // stack - 0x42
  long v9; // r13
  
  if ((a1) && (v4 = (long)(long)*a2, v2 = *(char **)(a1 + (long)v4 * 8), v2)) {
    v4 = (unsigned long)strlen(v2);
    if (!v3) {
      v4 = dcgettext(NULL,"invalid null argument to -size",5);
      error(1,0,v4);
      return v4;
    }
    v4 = (long)(long)(int)v3;
    v1 = &v2[(long)v4 + -1];
    v5 = *v1;
    if (v5 != 'b') { // branch-flip
      if ('c' <= v5) { // branch-flip
        if (v5 != 'k') { // branch-flip
          if (v5 != 'w') { // branch-flip
            if (v5 != 'c') {
              v4 = dcgettext(NULL,"invalid -size type `%c\'",5); // return-dupe
              error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
              return v4;
            }
            *v1 = '\0';
            v9 = 1;
          }
          else {
            *v1 = '\0';
            v9 = 2;
          }
        }
        else {
          *v1 = '\0';
          v9 = 0x400;
        }
      }
      else if (v5 != 'G') { // branch-flip
        if ('H' <= v5) { // branch-flip
          if (v5 != 'M') {
            v4 = dcgettext(NULL,"invalid -size type `%c\'",5);
            error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
            return v4;
          }
          *v1 = '\0';
          v9 = 0x100000;
        }
        else {
          if (9 < (unsigned char)(v5 - 0x30U)) {
            v4 = dcgettext(NULL,"invalid -size type `%c\'",5);
            error(1,0,v4,(unsigned long)(unsigned int)(int)v5);
            return v4;
          }
          v9 = 0x200;
          v5 = '\0';
        }
      }
      else {
        *v1 = '\0';
        v9 = 0x40000000;
      }
    }
    else {
      *v1 = '\0';
      v9 = 0x200;
    }
    v3 = sub_11ae0(v2,&v7,&v6);
    v10 = (unsigned long)v3;
    if (!(char)v3) {
      v8[1] = 0;
      v8[0] = v5;
      v4 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
      error(1,0,v4,v2,v8);
      return v4;
    }
    v4 = (long)sub_d490(a0,v2);
    *(int *)((long)v4 + 0x3c) = (int)v9;
    *(int *)((long)v4 + 0x38) = v6;
    if (v6) { // branch-flip
      v11 = dat_2ec68;
      if ((v6 == 1) && (v11 = dat_2ec80, (unsigned long)(v9 * v7) <= 0x5000))
        v11 = dat_2ec74;
    }
    else {
      v11 = dat_2ec74;
      if ((unsigned long)(v9 * v7) <= 0x5000)
        v11 = dat_2ec80;
    }
    *(long *)((long)v4 + 0x40) = v7;
    *(unsigned short *)((long)v4 + 0x1a) = 1;
    *(unsigned int *)((long)v4 + 0x24) = v11;
    *a2 = *a2 + 1;
  }
  else {
    v10 = 0;
  }
  v4 = (char *)v10;
  return v4;
}


// Function: sub_12080 @ 0x12080
unsigned int sub_12080(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned int v1; // eax
  int v2; // eax
  unsigned long v3; // stack - 0x38
  
  v1 = sub_107d0(a0,a1,&v3);
  if ((char)v1) { // branch-flip
    *a3 = v3;
    v2 = (*dat_3e670)(v3,a2);
    if (v2)
      sub_e310(*__errno_location(),v3); // no-return
  }
  else {
    *a3 = 0;
  }
  return v1;
}


// Function: sub_12110 @ 0x12110
unsigned int sub_12110(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    v5 = sub_25c60(v3);
    *(unsigned long *)(v2 + 0x38) = 3;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}


// Function: sub_121d0 @ 0x121d0
unsigned int sub_121d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    *(unsigned int *)(v2 + 0x38) = 2;
    v5 = sub_25c60(v3);
    *(unsigned int *)(v2 + 0x3c) = 0;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}


// Function: sub_122a0 @ 0x122a0
unsigned int sub_122a0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [88];
  unsigned long v4; // stack - 0xd0
  char v5 [16];
  unsigned long v6; // stack - 0x70
  
  sub_d860(v3);
  v1 = sub_12080(a1,a2,v3,&v4);
  if ((char)v1) {
    v2 = sub_d490(a0,v4);
    *(unsigned int *)(v2 + 0x38) = 0;
    v5 = sub_25c60(v3);
    *(unsigned int *)(v2 + 0x3c) = 0;
    *(long *)(v2 + 0x40) = SUB168(v5,0);
    *(long *)(v2 + 0x48) = SUB168(v5,8);
    *(unsigned int *)(v2 + 0x24) = sub_10760(v6);
  }
  return v1;
}


// Function: sub_12370 @ 0x12370
unsigned int sub_12370(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v10; // stack - 0xd0
  int v2;
  int v3; // eax
  long v4; // rax
  long v5; // stack - 0x168
  unsigned long v6; // stack - 0x170
  long v7; // stack - 0xd8
  long v8; // stack - 0x160
  unsigned int v9; // stack - 0x150
  
  sub_d860(&v5);
  v1 = sub_12080(a1,a2,&v5,&v6);
  if (!(char)v1)
    return v1;
  sub_d860(&v7);
  if (dat_3e668) { // branch-flip
label_12440:
    v2 = sub_15cc0(v6,0);
  }
  else {
    if (!dat_3e678) {
      if ((v9 & 0xf000) == 0xa000) {
        v2 = -2;
        goto label_123c8;
      }
      goto label_12440;
    }
    v2 = sub_15cc0(v6,0x20000);
  }
  if (0 <= v2) {
    if ((fstat(v2,&v7)) || (v3 = (*dat_3e670)(v6,&v5), v3))
      sub_e310(*__errno_location(),v6); // no-return
    if ((dat_3e668) || (dat_3e678)) {
      v5 = v7;
      v8 = v10;
    }
    else if ((((v9 & 0xf000) == 0xa000) || (v5 != v7)) || (v8 != v10)) {
      close(v2);
      v2 = -1;
    }
  }
label_123c8:
  v4 = sub_d490(a0,v6);
  *(int *)(v4 + 0x48) = v2;
  *(long *)(v4 + 0x38) = v8;
  *(unsigned int *)(v4 + 0x24) = 0x3c23d70a;
  *(long *)(v4 + 0x40) = v5;
  *(unsigned short *)(v4 + 0x1a) = 1;
  return v1;
}


// Function: sub_125d0 @ 0x125d0
unsigned long sub_125d0(unsigned long a0,unsigned long a1,unsigned long a2,int a3)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long *v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  char *v7; // stack - 0x38
  
  v1 = sub_107d0(a0,a1,&v7);
  if ((char)v1) {
    v2 = sub_d450(a2,sub_96f0,v7);
    *(unsigned short *)(v2 + 0x1a) = 0;
    v3 = (unsigned long *)sub_26990(0x40);
    *(unsigned long **)(v2 + 0x38) = v3;
    v3[1] = 100;
    v4 = sub_26990(100);
    v3[4] = 0;
    *v3 = v4;
    re_set_syntax((long)a3);
    v3[3] = (long)a3;
    v3[5] = 0;
    v5 = re_compile_pattern(v7,strlen(v7),v3);
    if (v5) {
      error(1,0,dcgettext(NULL,"failed to compile regular expression \'%s\': %s",5),v7,v5);
      return v6;
    }
    *(unsigned int *)(v2 + 0x24) = sub_110c0(v7,1);
  }
  return (unsigned long)v1;
}


// Function: sub_12710 @ 0x12710
void sub_12710(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_125d0(a1,a2,a0,dat_3e67c | 0x400000); // tail-call
}


// Function: sub_12740 @ 0x12740
unsigned long sub_12740(void *a0) // early-return
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (v1 == -1)
    return 0;
  v1 = isatty(v1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 != 0);
}


// Function: sub_12770 @ 0x12770
void sub_12770(char *a0,unsigned long *a1)
{
  unsigned long v1;
  long v2; // rax
  char *v3; // rax
  
  *a1 = 0;
  a1[4] = sub_24ac0(0);
  if (!strcmp(a0,"/dev/stderr")) {
    a1[1] = stderr;
    v3 = dcgettext(NULL,"standard error",5);
    v1 = a1[1];
    a1[2] = v3;
    *(char *)&a1[3] = sub_12740(v1);
    return;
  }
  if (strcmp(a0,"/dev/stdout")) {
    v2 = sub_e580(dat_3e610,a0);
    a1[2] = a0;
    a1[1] = v2;
    if (v2) {
      *(char *)&a1[3] = sub_12740(v2);
      return;
    }
    sub_e410(*__errno_location(),a0); // no-return
  }
  a1[1] = stdout;
  v3 = dcgettext(NULL,"standard output",5);
  v1 = a1[1];
  a1[2] = v3;
  *(char *)&a1[3] = sub_12740(v1);
}


// Function: sub_12870 @ 0x12870
void sub_12870(unsigned long a0)
{
  sub_12770("/dev/stdout",a0); // tail-call
}


// Function: sub_12880 @ 0x12880
void sub_12880(unsigned long a0,unsigned long a1,unsigned int *a2) // return-dupe x2
{
  unsigned int v1;
  unsigned long v2; // stack - 0x60
  char v3 [40];
  
  v1 = *a2;
  if (!sub_10790(a1,a2,&v2))
    return;
  sub_12870(v3);
  if (sub_fef0(v3,a0,v2))
    return;
  *a2 = v1;
}


// Function: sub_12910 @ 0x12910
unsigned long sub_12910(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0);
  *(unsigned int *)(v1 + 0x18) = 0x101;
  sub_12870(v1 + 0x38);
  return 1;
}


// Function: sub_12940 @ 0x12940
unsigned long sub_12940(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_d490(a0,a1);
  if (a1) // branch-flip
    sub_12770(a1,v1 + 0x38);
  else {
    sub_12870(v1 + 0x38);
  }
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_12990 @ 0x12990
void sub_12990(unsigned long a0)
{
  sub_12940(a0,0); // tail-call
}


// Function: sub_129a0 @ 0x129a0
void sub_129a0(unsigned long a0,unsigned long a1,int *a2)
{
  unsigned long v1; // stack - 0x28
  
  if ((sub_107d0(a1,a2,&v1)) && (!sub_12940(a0,v1)))
    *a2 = *a2 + -1;
}


// Function: sub_12a10 @ 0x12a10
unsigned long sub_12a10(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0);
  if (a1) // branch-flip
    sub_12770(a1,v1 + 0x38);
  else {
    sub_12870(v1 + 0x38);
  }
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  *(unsigned short *)(v1 + 0x18) = 0x101;
  return 1;
}


// Function: sub_12a60 @ 0x12a60
void sub_12a60(unsigned long a0)
{
  sub_12a10(a0,0); // tail-call
}


// Function: sub_12a70 @ 0x12a70
void sub_12a70(unsigned long a0,unsigned long a1,int *a2)
{
  unsigned long v1; // stack - 0x28
  
  if ((sub_107d0(a1,a2,&v1)) && (!sub_12a10(a0,v1)))
    *a2 = *a2 + -1;
}


// Function: sub_12ae0 @ 0x12ae0
unsigned long sub_12ae0(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x68
  unsigned long v3; // stack - 0x60
  char v4 [40];
  unsigned int v5;
  
  v5 = *a2;
  if ((sub_107d0(a1,a2,&v2)) && (sub_10790(a1,a2,&v3))) {
    sub_12770(v2,v4);
    v5 = *a2;
    v1 = sub_fef0(v4,a0,v3);
    if ((char)v1)
      return v1;
  }
  *a2 = v5;
  return 0;
}


// Function: sub_12b90 @ 0x12b90
unsigned int sub_12b90(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v3);
  if ((char)v1) {
    v2 = sub_d490(a0,v3);
    sub_12770(v3,v2 + 0x38);
    *(unsigned int *)(v2 + 0x18) = 0x101;
    *(unsigned int *)(v2 + 0x24) = 0x3f800000;
  }
  return v1;
}


// Function: sub_12c20 @ 0x12c20
unsigned long sub_12c20(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned int v10; // r14d
  unsigned int v11; // stack - 0x54
  unsigned int v2; // eax
  void *v3; // rax
  long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  char *v8; // stack - 0x48
  char *v9;
  
  v1 = sub_107d0(a1,a2,&v8);
  if ((char)v1) {
    if (*v8 != '-') { // branch-flip
      if (*v8 != '/') { // branch-flip
        v10 = 2;
        v11 = dat_2ec68;
        v9 = v8;
      }
      else {
        v11 = dat_2ec6c;
        v9 = &v8[1];
        v10 = 1;
      }
    }
    else {
      v9 = &v8[1];
      v10 = 0;
      v11 = dat_2ec7c;
    }
    v3 = (void *)sub_1d600(v9);
    if ((!v3) || ((*v8 == '+' && ((unsigned char)(v8[1] - 0x30U) <= 7)))) {
      v6 = sub_24e50(0,dat_3e68c,v8);
      error(1,0,dcgettext(NULL,"invalid mode %s",5),v6);
      return v7;
    }
    v2 = sub_1d9d0(0,0,0,v3,0);
    v4 = sub_1d9d0(0,1,0,v3,0);
    free(v3);
    if ((*v8 == '/') && (!(int)v4 && !v2)) {
      v10 = 0;
      error(0,0,dcgettext(NULL,"warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",5),v8);
      v11 = dat_2eca8;
    }
    v5 = sub_d490(a0,v8);
    *(unsigned int *)(v5 + 0x38) = v10;
    *(unsigned long *)(v5 + 0x3c) = (unsigned long)v2 | v4 << 0x20;
    *(unsigned int *)(v5 + 0x24) = v11;
  }
  return (unsigned long)v1;
}


// Function: sub_12e20 @ 0x12e20
void sub_12e20(unsigned long a0) // return-dupe
{
  char v1;
  char *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x40
  
  v2 = getenv("PATH");
  if (!v2)
    return;
  sub_170d0(v2,":",1,&v6,&v5);
  while (v5) {
    v1 = v2[v6];
    if ((v5 == 1) && (v1 == '.')) break;
    if (v1 != '/') {
      v3 = strndup(&v2[v6],v5);
      if (!v3)
        v3 = &v2[v6];
      v4 = sub_dfb0(0,v3);
      error(1,0,dcgettext(NULL,"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH",5),v4,a0);
      return;
    }
    if (!sub_170d0(v2,":",0,&v6,&v5))
      return;
  }
  error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0);
}


// Function: sub_12f60 @ 0x12f60
unsigned long sub_12f60(unsigned long a0,long a1,long a2,int *a3)
{
  int v1;
  unsigned long v10; // rax
  long v11;
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14; // rax
  long *v15;
  char v16 [24];
  char *v17;
  int v18;
  int v19; // ebp
  void *v2;
  unsigned long *v20;
  long *v21;
  char *v22; // stack - 0xa8
  bool v3;
  void *v4;
  void *v5; // rax
  long v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v2 = *(void **)(a1 + 0x18);
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v5 = (void *)sub_d450(a1,v2,"(some -exec* arguments)");
  *(unsigned int *)((long)v5 + 0x18) = 0x101;
  *(unsigned long *)((long)v5 + 0xf8) = 0;
  if (v2 != sub_94e0 && v2 != sub_9530) {
    *(char *)((long)v5 + 0xf4) = 0;
    v4 = sub_8ff0;
label_13056:
    if (v2 != v4) {
      if (!dat_3e5c0)
        __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
      *(long *)((long)v5 + 0xf8) = dat_3e5c0;
      goto label_13074;
    }
  }
  else {
    dat_3e698 = 1;
    *(char *)((long)v5 + 0xf4) = 1;
    if (v2 != sub_8ff0) {
      v4 = sub_9530;
      goto label_13056;
    }
  }
  dat_3e62e = 0;
  sub_12e20(a0);
label_13074:
  *(char *)((long)v5 + 0x38) = 0;
  v1 = *a3;
  v6 = (long)v1 * 8;
  v21 = (long *)(a2 + v6);
  v17 = (char *)*v21;
  v18 = v1;
  if (v17) {
    v20 = (unsigned long *)(a2 + 8 + v6);
    v22 = NULL;
    v19 = 0;
    v3 = 0;
    v15 = v21;
    do {
      if (*v17 != ';') { // branch-flip
        if ((((*v17 == '+') && (v2 != sub_94e0 && v2 != sub_9530)) && (!v17[1])) && (v3)) {
          *(char *)((long)v5 + 0x38) = 1;
label_1315e:
          if ((v1 != v18) && (*v15)) {
            if (*(char *)((long)v5 + 0x38)) {
              v17 = "dir";
              if (v2 != sub_8ff0)
                v17 = "";
              if (1 < v19) {
                error(1,0,dcgettext(NULL,"Only one instance of {} is supported with -exec%s ... +",5),v17);
                return v14;
              }
              if (strlen(v22) != 2) {
                __snprintf_chk(v16,0x13,1,0x13,"-exec%s ... {} +",v17);
                v7 = sub_24e50(2,dat_3e68c,v22);
                v8 = sub_24e50(1,dat_3e68c,"{}");
                v9 = sub_24e50(0,dat_3e68c,v16);
                error(1,0,dcgettext(NULL,"In %s the %s must appear by itself, but you specified %s",5),v9,v8,v7);
                return v10;
              }
            }
            v6 = (long)v5 + 0x40;
            if ((unsigned int)(sub_14ca0(v6,0x800) - 1) < 2) {
              error(1,0,dcgettext(NULL,"The environment is too large for exec().",5));
              return v13;
            }
            sub_14d80(v6);
            v19 = v18 - v1;
            v11 = (long)v5 + 0x98;
            *(void **)((long)v5 + 0x80) = sub_a610;
            if (*(char *)((long)v5 + 0x38)) { // branch-flip
              *(unsigned long *)((long)v5 + 0xe8) = 0;
              *(unsigned long *)((long)v5 + 0x70) = 0;
              *(long *)((long)v5 + 0x78) = (long)(v19 + -1);
              *(unsigned long *)((long)v5 + 0x68) = 0;
              *(unsigned long *)((long)v5 + 0x88) = 0;
              *(unsigned long *)((long)v5 + 0x90) = 0;
              sub_14db0(v6,v11,(long)v5 + 0x38);
              if (v1 < v18 + -1) {
                do {
                  v17 = (char *)*v21;
                  v21 = &v21[1];
                  sub_15000(v6,v11,v17,strlen(v17) + 1,0,0,1);
                } while (v21 != (long *)(a2 + 8 + ((unsigned long)(unsigned int)((v18 - v1) - 2) + (long)v1) * 8));
              }
            }
            else {
              *(int *)((long)v5 + 0xf0) = v19;
              *(char **)((long)v5 + 0x70) = "{}";
              *(unsigned long *)((long)v5 + 0x68) = strlen("{}");
              *(unsigned long *)((long)v5 + 0x88) = 0;
              *(unsigned long *)((long)v5 + 0x90) = 0;
              *(unsigned long *)((long)v5 + 0xe8) = sub_26990((long)v19 << 3);
              sub_14db0(v6,v11,(long)v5 + 0x38);
              v1 = *(int *)((long)v5 + 0xf0);
              if (1 <= v1) {
                v6 = *(long *)((long)v5 + 0xe8);
                v11 = 0;
                do {
                  *(long *)(v6 + v11 * 8) = v21[v11];
                  v11 += 1;
                } while (v11 != v1);
              }
            }
            if (*v15) {
              *a3 = v18 + 1;
              return 1;
            }
            *a3 = v18;
            return 1;
          }
          goto label_1328a;
        }
      }
      else if (!v17[1]) goto label_1315e;
      if (sub_1caf0(v17,"{}")) { // branch-flip
        v19 += 1;
        if ((!v18) && ((v2 == sub_8ff0 || (v2 == sub_9530)))) {
          error(1,0,dcgettext(NULL,"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",5));
          return v12;
        }
        v3 = 1;
        v22 = v17;
      }
      else {
        v3 = 0;
      }
      v17 = (char *)*v20;
      v18 += 1;
      v20 = &v20[1];
      v15 = &v15[1];
    } while (v17);
  }
label_1328a:
  *a3 = v18;
  free(v5);
  return 0;
}


// Function: sub_13550 @ 0x13550
void sub_13550(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_12f60("-okdir",a0,a1,a2); // tail-call
}


// Function: sub_135d0 @ 0x135d0
unsigned long sub_135d0(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x200000003;
  *v1 = sub_88c0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}


// Function: sub_13610 @ 0x13610
unsigned long sub_13610(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x300000003;
  *v1 = sub_8870;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}


// Function: sub_13650 @ 0x13650
unsigned long sub_13650(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 0x100000003;
  *v1 = sub_8cd0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  *(unsigned int *)((long)v1 + 0x24) = 0x3f800000;
  return 1;
}


// Function: sub_13690 @ 0x13690
unsigned long sub_13690(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b970();
  v1[2] = 5;
  *v1 = sub_8c30;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}


// Function: sub_136c0 @ 0x136c0
unsigned long sub_136c0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rdx
  
  if (sub_106f0()) {
    if (strchr(v4,0x2f)) {
      v1 = sub_dfb0(2,a1);
      v2 = sub_dfb0(1,"/");
      v3 = sub_dfb0(0,a0);
      error(0,0,dcgettext(NULL,"warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?",5),v3,v2,v1);
    }
  }
  return 1;
}


// Function: sub_13750 @ 0x13750
unsigned int sub_13750(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned int v1;
  unsigned int v2; // eax
  long v3; // rax
  unsigned long v4; // stack - 0x38
  
  v1 = *a2;
  v2 = sub_107d0(a1,a2,&v4);
  if ((char)v2) {
    sub_11020();
    v2 = sub_136c0("-name","-wholename",v4);
    if ((char)v2) { // branch-flip
      v3 = sub_d490(a0,v4);
      *(unsigned short *)(v3 + 0x1a) = 0;
      *(unsigned long *)(v3 + 0x38) = v4;
      *(unsigned int *)(v3 + 0x24) = sub_110c0(v4,0);
    }
    else {
      *a2 = v1;
    }
  }
  return v2;
}


// Function: sub_13810 @ 0x13810
unsigned int sub_13810(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  sub_11020();
  if ((sub_107d0(a1,a2,&v3)) && (v1 = sub_136c0("-iname","-iwholename",v3), (char)v1)) {
    v2 = sub_d490(a0,v3);
    *(unsigned short *)(v2 + 0x1a) = 0;
    *(unsigned long *)(v2 + 0x38) = v3;
    *(unsigned int *)(v2 + 0x24) = sub_110c0(v3,0);
  }
  else {
    v1 = 0;
  }
  return v1;
}


// Function: sub_138d0 @ 0x138d0
unsigned long sub_138d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v10; // rax
  char *v11; // stack - 0x38
  int v2; // eax
  unsigned int v3;
  void *v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v1 = sub_107d0(a1,a2,&v11);
  if ((char)v1) {
    v4 = getgrnam(v11);
    endgrent();
    if (v4) // branch-flip
      v3 = *(unsigned int *)((long)v4 + 0x10);
    else {
      v2 = (int)strspn(v11,"0123456789");
      if (!v2) {
        if (!*v11) {
          error(1,0,dcgettext(NULL,"argument to -group is empty, but should be a group name",5));
          return v7;
        }
        v8 = sub_24e50(0,dat_3e68c,v11);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group",5),v8);
        return v6;
      }
      if (v11[v2]) {
        v8 = sub_24e50(1,dat_3e68c,&v11[v2]);
        v9 = sub_24e50(0,dat_3e68c,v11);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s",5),v9,v8);
        return v10;
      }
      v3 = sub_16f40(v11);
    }
    v5 = sub_d490(a0,v11);
    *(unsigned int *)(v5 + 0x38) = v3;
    v3 = dat_2eca4;
    if (100 <= *(unsigned long *)(v5 + 0x40))
      v3 = dat_2ec7c;
    *(unsigned int *)(v5 + 0x24) = v3;
  }
  return (unsigned long)v1;
}


// Function: sub_13a70 @ 0x13a70
char sub_13a70(char *a0)
{
  char *v1; // rax
  char v2 [152];
  char v3; // xmm0_ba
  undefined4 v4;
  
  v4 = dat_2c588;
  if (!stat("/",v2)) {
    v1 = (char *)sub_104c0(dat_2c588,v2,"/");
    v4 = s_2ecab._1_4_;
    if (strcmp(a0,v1))
      v4 = dat_2ec6c;
  }
  v3 = (char)v4;
  return v3;
}


// Function: sub_13b10 @ 0x13b10
unsigned long sub_13b10(void) // ternary
{
  int *v1; // rax
  long v2; // rdx
  
  if (dat_3e658)
    return 1;
  dat_3e648 += 0x15180;
  dat_3e650 = 0;
  v1 = localtime((void *)0x3e648);
  v2 = (v1) ? (long)(v1[1] * 0x3c + *v1 + v1[2] * 0xe10) : dat_3e648 % 0x15180; // branch-flip
  dat_3e658 = 1;
  dat_3e648 -= v2;
  return 1;
}


// Function: sub_13bc0 @ 0x13bc0
unsigned int sub_13bc0(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned int v1;
  char *v10; // r8
  int v11; // stack - 0x54
  unsigned long v12; // stack - 0x50
  unsigned long v13; // stack - 0x48
  unsigned int v2; // eax
  long v3;
  unsigned long v4; // rax
  char *v5; // rcx
  unsigned long v6; // stack - 0x68
  int v7; // stack - 0x6c
  unsigned int v8; // stack - 0x58
  long v9; // stack - 0x60
  
  dcgettext(NULL,"arithmetic overflow while converting %s days to a number of seconds",5);
  v1 = *a2;
  v2 = sub_107d0(a1,a2,&v6);
  if ((char)v2) {
    v3 = dat_3e648;
    if ((sub_10860(&v6,&v7)) && (v7 == 1))
      v3 += 0x1517f;
    v2 = sub_11650(dat_2ecb8,v6,&v8,v3);
    if ((char)v2) { // branch-flip
      v3 = sub_d490(a0,v6);
      *(unsigned long *)(v3 + 0x38) = CONCAT44(v11,v8);
      *(unsigned long *)(v3 + 0x40) = v12;
      *(unsigned long *)(v3 + 0x48) = v13;
      *(unsigned int *)(v3 + 0x24) = sub_10760(v12);
      if (dat_3e660 & 1) {
        __fprintf_chk(stderr,1,"inserting %s\n",*(unsigned long *)(v3 + 8));
        if (v11) { // branch-flip
          if (v11 != 1) { // branch-flip
            v10 = " ?";
            v5 = "?";
            if (v11 == 2) {
              __fprintf_chk(stderr,1,"    type: %s    %s  ","eq",">=");
              v9 = *(long *)(v3 + 0x40);
              v4 = ctime(&v9);
              __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v3 + 0x40),v4);
              v9 = *(long *)(v3 + 0x40) + 0x15180;
              __fprintf_chk(stderr,1,"                 <  %lu %s",v9,ctime(&v9));
              return v2;
            }
          }
          else {
            v10 = " <";
            v5 = "lt";
          }
        }
        else {
          v10 = " >";
          v5 = "gt";
        }
        __fprintf_chk(stderr,1,"    type: %s    %s  ",v5,v10);
        v9 = *(long *)(v3 + 0x40);
        v4 = ctime(&v9);
        __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v3 + 0x40),v4);
      }
    }
    else {
      *a2 = v1;
    }
  }
  return v2;
}


// Function: sub_13e80 @ 0x13e80
unsigned long sub_13e80(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b990(a0,0);
  v1[2] = 0x400000002;
  *v1 = sub_92f0;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}


// Function: sub_13ec0 @ 0x13ec0
unsigned long sub_13ec0(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_b990(a0,0);
  v1[2] = 4;
  *v1 = sub_9590;
  *(unsigned short *)((long)v1 + 0x1a) = 0;
  return 1;
}


// Function: sub_13f00 @ 0x13f00
unsigned long sub_13f00(unsigned long a0,long a1,int *a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  char v1;
  unsigned long v10; // rax
  int *v11; // rax
  char v12 [6]; // stack - 0x46
  char v13 [146];
  bool v14; // r9b
  unsigned long v15; // r9
  unsigned int v16; // eax
  char v2;
  char *v3;
  void *v4;
  int v5;
  long *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  if (a1) {
    v5 = *a2;
    v3 = *(char **)(a1 + (long)v5 * 8);
    if ((v3) && (strlen(v3) == 8)) {
      builtin_strncpy(v12,"aBcmt",6);
      if (strncmp("-newer",v3,6))
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
      v1 = v3[6];
      v2 = v3[7];
      v14 = v1 == 'B' || v2 == 'B';
      v15 = CONCAT71((undefined7)((unsigned long)a5 >> 8),v14);
      if (v14) {
        error(0,0,dcgettext(NULL,"This system does not provide a way to find the birth time of a file.",5));
        v15 = 0;
      }
      else if (v1 != 't') {
        v15 = 0;
        if (strchr(v12,(int)v1)) {
          v15 = 0;
          if (strchr(v12,(int)v2)) {
            if (!*(long *)(a1 + 8 + (long)v5 * 8)) {
              v9 = sub_24e50(0,dat_3e68c,v3);
              error(1,0,dcgettext(NULL,"The %s test needs an argument",5),v9);
              return v8;
            }
            *a2 = v5 + 1;
            v6 = (long *)sub_d490(a0);
            if (v1 != 'c') { // branch-flip
              if (v1 != 'm') { // branch-flip
                if (v1 != 'a') {
                  if (!strchr(v12,(int)v1))
                    __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
                  __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
                }
                v16 = 0;
              }
              else {
                v16 = 3;
              }
            }
            else {
              v16 = 2;
            }
            *(unsigned int *)&v6[7] = v16;
            if (v2 != 't') { // branch-flip
              sub_d860(v13);
              v5 = (*dat_3e670)(*(unsigned long *)(a1 + (long)*a2 * 8),v13);
              if (v5) {
                v11 = __errno_location();
                sub_e310(*v11,*(unsigned long *)(a1 + (long)*a2 * 8)); // no-return
              }
              if (!sub_11a40(v13,(int)v2,&v6[8])) {
                v9 = sub_dfb0(0,*(unsigned long *)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"Cannot obtain birth time of file %s",5),v9);
                return v10;
              }
            }
            else {
              v9 = *(unsigned long *)(a1 + (long)*a2 * 8);
              if (!sub_22c40(&v6[8],v9,0x3e638)) {
                v9 = sub_24e50(0,dat_3e68c,*(unsigned long *)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"I cannot figure out how to interpret %s as a date or time",5),v9);
                return v7;
              }
            }
            *(unsigned int *)((long)v6 + 0x3c) = 0;
            v16 = sub_10760(v6[8]);
            v4 = (void *)*v6;
            *(unsigned int *)((long)v6 + 0x24) = v16;
            *a2 = *a2 + 1;
            if (!v4)
              __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
            if (v4 != sub_9370)
              __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
            v15 = (unsigned long)*(unsigned char *)((long)v6 + 0x1a);
            if (!*(unsigned char *)((long)v6 + 0x1a))
              __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
          }
        }
      }
      return v15 & 0xffffffff; // return-dupe
    }
  }
  v15 = 0;
  return v15 & 0xffffffff;
}


// Function: sub_14370 @ 0x14370
unsigned long sub_14370(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x28
  
  if (!sub_107d0(a1,a2,&v2))
    return 0;
  dat_3e67c = sub_16d30(v2);
  v1 = sub_14350(a1,a2); // tail-call
  return v1;
}


// Function: sub_14400 @ 0x14400
char * sub_14400(unsigned long a0,long a1,int *a2,int *a3)
{
  unsigned long v1;
  unsigned long v2;
  int v3; // eax
  char *v4; // rax
  char *v5; // stack - 0x38
  unsigned int v6;
  
  v4 = (long)(long)*a2;
  v1 = *(unsigned long *)(a1 + -8 + (long)v4 * 8);
  if (!sub_107d0(a1,a2,&v5)) {
    v4 = NULL;
    return v4;
  }
  v4 = (unsigned long)strspn(v5,"0123456789");
  v6 = dat_3e68c;
  if ((1 <= v3) && (v4 = (long)(long)v3, !v5[(long)v4])) {
    v3 = sub_16f40(v5,dat_3e68c);
    *a3 = v3;
    if (0 <= v3) {
      v4 = (unsigned long)sub_14350(a1,a2); // tail-call
      return v4;
    }
    v6 = dat_3e68c;
  }
  v4 = (unsigned long)sub_24e50(0,v6,v5);
  v2 = v4;
  v4 = dcgettext(NULL,"Expected a positive decimal integer argument to %s, but got %s",5);
  error(1,0,v4,v1,v2);
  return v4;
}


// Function: sub_14510 @ 0x14510
void sub_14510(void)
{
  sub_14400(); // tail-call
}


// Function: sub_14530 @ 0x14530
void sub_14530(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e620 = 0x100;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_14550 @ 0x14550
void sub_14550(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if (sub_106f0())
    error(0,0,dcgettext(NULL,"warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.",5));
  sub_14530(a0,a1,a2); // tail-call
}


// Function: sub_145b0 @ 0x145b0
void sub_145b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62e = 1;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_145d0 @ 0x145d0
void sub_145d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62d = 1;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_145f0 @ 0x145f0
void sub_145f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62c = 1;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_14610 @ 0x14610
void sub_14610(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e62e = 0;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_14630 @ 0x14630
void sub_14630(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e630 = 0;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_14650 @ 0x14650
void sub_14650(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_3e630 = 1;
  sub_14350(a1,a2); // tail-call
}


// Function: sub_14670 @ 0x14670
void sub_14670(long *a0)
{
  void *v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = 0;
    do {
      while( true ) {
        v1 = (void *)*a0;
        if (v1 != sub_8d40) break;
        a0 = (long *)a0[0x21];
        v2 |= 1;
        if (!a0) goto label_146ba;
      }
      a0 = (long *)a0[0x21];
      if (v1 == sub_86f0)
        v2 |= 2;
    } while (a0);
label_146ba:
    if ((v2 == 3) && (!dat_3e621)) {
      error(1,0,dcgettext(NULL,"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.",5));
      return;
    }
  }
}


// Function: sub_146f0 @ 0x146f0
void sub_146f0(int a0) // return-dupe
{
  if (a0 != 1) { // branch-flip
    if (a0 != 2) { // branch-flip
      if (!a0) {
        dat_3e670 = sub_db20;
        dat_3e680 = sub_10940;
      }
    }
    else {
      dat_3e62c = 1;
      dat_3e670 = sub_da00;
      dat_3e680 = sub_109e0;
    }
  }
  else {
    dat_3e62c = 1;
    dat_3e670 = sub_daa0;
    dat_3e680 = sub_10a40;
  }
  dat_3e668 = a0;
  if (!(dat_3e660 & 2))
    return;
  dat_3e670 = sub_db70;
}


// Function: sub_147a0 @ 0x147a0
void sub_147a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_146f0(1,a2,a2,a3,a1);
  sub_14350(a1); // tail-call
}


// Function: sub_147d0 @ 0x147d0
void sub_147d0(void)
{
  dat_3ee70 = 0;
}


// Function: sub_147e0 @ 0x147e0
void sub_147e0(void)
{
  return;
}


// Function: sub_147f0 @ 0x147f0
unsigned long sub_147f0(char *a0)
{
  char v1;
  long *v2; // rbx
  int v3; // ebp
  long v4;
  char *v5;
  
  if (!strncmp("-newer",a0,6)) {
    v4 = 0x3d800;
    if (strlen(a0) == 8) {
      return sub_108b0(a0,v4); // return-dupe, tail-call
    }
  }
  v1 = *a0;
  v2 = (long *)0x3cd48;
  v5 = "!";
  v3 = 0;
  while( true ) {
    if (!strcmp(v5,&a0[v1 == '-'])) break;
    v5 = (char *)*v2;
    v2 = &v2[4];
    v3 += 1;
    if (!v5)
      return 0;
  }
  v4 = (long)v3 * 0x20 + 0x3cd20;
  return sub_108b0(a0,v4);
}


// Function: sub_148a0 @ 0x148a0
unsigned long sub_148a0(void)
{
  long v1; // rax
  
  v1 = sub_d4d0(sub_147f0("false"));
  *(unsigned int *)(v1 + 0x18) = 0;
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}


// Function: sub_148d0 @ 0x148d0
void sub_148d0(void)
{
  sub_148a0(); // tail-call
}


// Function: sub_148e0 @ 0x148e0
unsigned long sub_148e0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x28
  
  v1 = sub_107d0(a1,a2,&v4);
  if ((char)v1) {
    if ((2 <= dat_3e688) && (!sub_10510(v4))) {
      if (dat_3e660 & 8)
        __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",v4);
      v2 = sub_148a0(); // tail-call
      return v2;
    }
    v3 = sub_d490(a0,v4);
    *(unsigned long *)(v3 + 0x38) = v4;
    *(unsigned int *)(v3 + 0x24) = sub_13a70(v4);
  }
  return (unsigned long)v1;
}

