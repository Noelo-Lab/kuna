// Function: sub_73b9 @ 0x73b9
void sub_73b9(void)
{
  if ((dat_49010 & 0x200) && (0 <= dat_49018)) {
    close(dat_49018);
    dat_49018 = -1;
  }
}


// Function: sub_73f4 @ 0x73f4
void sub_73f4(int a0)
{
  if (dat_49010 & 0x200) {
    if ((a0 != -100) && (a0 <= -1))
      __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir"); // no-return
    dat_499d8 = a0;
    if (dat_49018 <= -1) {
      if (a0 != -100) { // branch-flip
        if (0 <= a0) // branch-flip
          dat_49018 = sub_1d639(a0);
        else if ((dat_49018 <= -1) && (a0 <= -1))
          __assert_fail("curr_fd >= 0 || dir_fd >= 0","ftsfind.c",0x7d,"inside_dir"); // no-return
      }
      else {
        dat_49018 = -100;
      }
    }
  }
}


// Function: sub_74c2 @ 0x74c2
char * sub_74c2(unsigned int a0) // return-dupe x14
{
  switch(a0) {
    default:
      sprintf((char *)0x49918,"[%d]",(unsigned long)a0);
      return (char *)0x49918;
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


// Function: sub_75b5 @ 0x75b5
void sub_75b5(long a0,long a1,unsigned long a2) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_75b5
{
  unsigned long v1; // rax
  
  dat_499c4 = ((*(short *)(a1 + 0x68) != 10) && (*(short *)(a1 + 0x68) != 0xb));
  dat_499d0 = *(unsigned long *)(a1 + 0x30);
  dat_499d8 = *(unsigned int *)(a0 + 0x2c);
  v1 = sub_d7ff();
  sub_10873(*(unsigned long *)(a1 + 0x38),a2,v1);
  if (!dat_499e0)
    return;
  sub_1fc0f(a0,a1,4);
}


// Function: sub_7664 @ 0x7664
char * sub_7664(unsigned int a0,long a1,long a2,unsigned int a3)
{
  char v1;
  char *v2; // rax
  
  if (a2) { // branch-flip
    v2 = (char *)(a2 + a1);
    v1 = *v2;
    v2 = (char *)(a2 + a1);
    *v2 = 0;
    v2 = (unsigned long)sub_2f861(a0,a3,a1);
    *(char *)(a1 + a2) = v1;
  }
  else {
    v2 = (unsigned long)sub_2f861(a0,a3,0x38110);
  }
  return v2;
}


// Function: sub_76ea @ 0x76ea
void sub_76ea(long *a0) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  if ((*(unsigned int *)&a0[0x11] & 0xf000) == 0xa000) {
    v1 = sub_1096a(0,a0[7]);
    error(0,0,gettext("Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."),v1);
    return;
  }
  v1 = sub_7664(1,*(unsigned long *)(*a0 + 0x38),*(unsigned long *)(*a0 + 0x48),dat_499ac);
  v2 = sub_1096a(0,a0[7]);
  error(0,0,gettext("File system loop detected; %s is part of the same file system loop as %s."),v2,v1);
}


// Function: sub_77d0 @ 0x77d0
unsigned int sub_77d0(unsigned long a0) // return-dupe
{
  int v1; // eax
  char v2 [152];
  
  v1 = (*dat_49990)(a0,v2);
  if ((v1) && (*__errno_location() == 0x28))
    return 1;
  return 0;
}


// Function: sub_7852 @ 0x7852
void sub_7852(long a0,long a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_7852
{
  unsigned int v1;
  unsigned int v10; // stack - 0xe0
  unsigned int v11; // stack - 0xdc
  unsigned long v12; // stack - 0xd0
  unsigned long v13; // stack - 0xc8
  unsigned long v14; // stack - 0xc0
  unsigned long v15; // stack - 0xb8
  unsigned long v16; // stack - 0xb0
  unsigned long v17; // stack - 0xa8
  unsigned long v18; // stack - 0xa0
  unsigned long v19; // stack - 0x98
  unsigned long v2;
  unsigned long v20; // stack - 0x90
  unsigned long v21; // stack - 0x88
  unsigned long v22; // stack - 0x80
  unsigned long v23; // stack - 0x78
  unsigned long v24; // stack - 0x70
  unsigned long v25; // stack - 0x68
  unsigned long v26; // stack - 0x60
  unsigned long v27; // stack - 0x58
  unsigned long v28; // stack - 0x50
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6;
  unsigned long v7; // stack - 0xd8
  unsigned int v8; // stack - 0xe4
  unsigned long v9; // r12
  
  if (dat_49980 & 4) {
    v2 = sub_2f861(2,dat_499ac,*(unsigned long *)(a1 + 0x30));
    v3 = sub_2f861(1,dat_499ac,*(unsigned long *)(a1 + 0x38));
    v1 = dat_49014;
    v6 = *(unsigned long *)(a1 + 0x58);
    v4 = sub_74c2(*(unsigned short *)(a1 + 0x68));
    v5 = sub_2f861(0,dat_499ac,*(unsigned long *)(a1 + 0x38));
    fprintf(stderr,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v5,v4,v6 & 0xffffffff,(unsigned long)v1,v3,v2);
  }
  if (*(short *)(a1 + 0x68) != 6) { // branch-flip
    if (((long)(int)dat_49014 < *(long *)(a1 + 0x58)) || (!*(long *)(a1 + 0x58)))
      sub_73b9();
  }
  else {
    sub_73b9();
  }
  sub_73f4(*(unsigned int *)(a0 + 0x2c));
  dat_49014 = (unsigned int)*(unsigned long *)(a1 + 0x58);
  v12 = *(unsigned long *)(a1 + 0x78);
  if (*(short *)(a1 + 0x68) == 7) {
    sub_10a13(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
    return;
  }
  if (*(short *)(a1 + 0x68) != 4) { // branch-flip
    if (*(short *)(a1 + 0x68) == 2) {
      sub_76ea(a1);
      dat_499e4 = 1;
      return;
    }
    if (*(short *)(a1 + 0x68) != 0xd) { // branch-flip
      if (*(short *)(a1 + 0x68) == 10) {
        if (!*(long *)(a1 + 0x58)) {
          sub_10a13(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
          return;
        }
        v2 = *(unsigned long *)(a1 + 0x30);
        if (sub_77d0(v2)) {
          sub_10a13(0x28,*(unsigned long *)(a1 + 0x38));
          return;
        }
        sub_10a13(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
      }
    }
    else {
      v2 = *(unsigned long *)(a1 + 0x30);
      if (sub_77d0(v2)) {
        sub_10a13(0x28,*(unsigned long *)(a1 + 0x38));
        return;
      }
    }
  }
  else {
    sub_10a13(*(unsigned int *)(a1 + 0x40),*(unsigned long *)(a1 + 0x38));
    if (dat_49940)
      return;
  }
  if ((*(short *)(a1 + 0x68) != 0xb) && (*(short *)(a1 + 0x68) != 10)) {
    dat_499c4 = 1;
    dat_499c5 = 1;
    v7 = *(unsigned long *)(a1 + 0x70);
    v12 = *(unsigned long *)(a1 + 0x78);
    v13 = *(unsigned long *)(a1 + 0x80);
    v14 = *(unsigned long *)(a1 + 0x88);
    v15 = *(unsigned long *)(a1 + 0x90);
    v16 = *(unsigned long *)(a1 + 0x98);
    v17 = *(unsigned long *)(a1 + 0xa0);
    v18 = *(unsigned long *)(a1 + 0xa8);
    v19 = *(unsigned long *)(a1 + 0xb0);
    v20 = *(unsigned long *)(a1 + 0xb8);
    v21 = *(unsigned long *)(a1 + 0xc0);
    v22 = *(unsigned long *)(a1 + 200);
    v23 = *(unsigned long *)(a1 + 0xd0);
    v24 = *(unsigned long *)(a1 + 0xd8);
    v25 = *(unsigned long *)(a1 + 0xe0);
    v26 = *(unsigned long *)(a1 + 0xe8);
    v28 = *(unsigned long *)(a1 + 0xf8);
    v27 = *(unsigned long *)(a1 + 0xf0);
    v8 = (unsigned int)v14;
    dat_499c8 = (unsigned int)v14;
    if (!(unsigned int)v14) {
      v2 = sub_2f861(0,dat_499ac,*(unsigned long *)(a1 + 0x38));
      error(0,0,gettext("WARNING: file %s appears to have mode 0000"),v2);
    }
  }
  else {
    if (dat_499c4 == 1)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((*(short *)(a1 + 0x68) != 0xb) && (dat_499c8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v8 = dat_499c8;
  }
  dat_499c0 = (unsigned int)*(unsigned long *)(a1 + 0x58);
  if ((v8) && (v2 = *(unsigned long *)(a1 + 0x38), sub_fe38(&v8,v2,a1 + 0x100,&v7,0) != '\x01'))
    return;
  v10 = 0;
  v11 = !(((v8 & 0xf000) != 0x4000) && (((*(short *)(a1 + 0x68) != 1 && (*(short *)(a1 + 0x68) != 6)) && (*(short *)(a1 + 0x68) != 2))));
  if ((v11) && (*(short *)(a1 + 0x68) == 0xb)) {
    sub_1fc0f(a0,a1,1);
    return;
  }
  if ((0 <= dat_49944) && (((long)dat_49944 <= *(long *)(a1 + 0x58) && (sub_1fc0f(a0,a1,4), (long)dat_49944 < *(long *)(a1 + 0x58)))))
    v10 = 1;
  if ((*(short *)(a1 + 0x68) != 1) || (dat_49940 == '\x01')) {
    if ((*(short *)(a1 + 0x68) != 6) || (!dat_49940)) {
      if (*(long *)(a1 + 0x58) < (long)dat_49948)
        v10 = 1;
    }
    else {
      v10 = 1;
    }
  }
  else {
    v10 = 1;
  }
  if (dat_49980 & 4) {
    v9 = (unsigned long)dat_499c5;
    v6 = (unsigned long)dat_499c4;
    v2 = sub_74c2(*(unsigned short *)(a1 + 0x68));
    v3 = sub_2f861(0,dat_499ac,*(unsigned long *)(a1 + 0x38));
    fprintf(stderr,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v3,v2,(unsigned long)v11,(unsigned long)v10,v6,v9);
  }
  if (!v10)
    sub_75b5(a0,a1,&v7);
  if (*(short *)(a1 + 0x68) != 6)
    return;
  dat_499e0 = 0;
}


// Function: sub_803a @ 0x803a
unsigned long sub_803a(char *a0) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  long v4; // rax
  char *v5; // stack - 0x38
  int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x30
  
  dat_499dc = (unsigned int)strlen(a0);
  sub_73f4(0xffffff9c);
  v7 = 0;
  if (dat_49988 != 2) { // branch-flip
    if (dat_49988 <= 2) {
      if (dat_49988) { // branch-flip
        if (dat_49988 == 1)
          dat_49010 |= 3;
      }
      else {
        dat_49010 |= 0x10;
      }
    }
  }
  else {
    dat_49010 |= 0x11;
  }
  if (dat_4994d)
    dat_49010 |= 0x40;
  v5 = a0;
  v1 = sub_1ec55(&v5,dat_49010,0);
  if (!v1) {
    v2 = sub_1096a(0,a0);
    v3 = gettext("cannot search %s");
    error(0,*__errno_location(),v3,v2);
    dat_499e4 = 1;
    return 1;
  }
  v6 = -0x80000000;
  while( true ) {
    *__errno_location() = 0;
    v4 = sub_1f599(v1);
    if (!v4) break;
    if ((dat_499e8) && (v6 != (int)*(unsigned long *)(v4 + 0x58)))
      sub_f6cb();
    v6 = (int)*(unsigned long *)(v4 + 0x58);
    dat_499f8 = 0;
    dat_499c4 = 0;
    dat_499c5 = *(int *)(v4 + 0x88) != 0;
    if ((bool)dat_499c5)
      dat_499c8 = *(unsigned int *)(v4 + 0x88);
    else {
      dat_499c8 = 0;
    }
    sub_7852(v1,v4);
  }
  if (!*__errno_location()) {
    if (!sub_1f0df(v1))
      return 1;
    v3 = gettext("failed to restore working directory after searching %s");
    error(0,*__errno_location(),v3,a0);
    dat_499e4 = 1;
    return 0;
  }
  v2 = sub_1096a(0,a0);
  error(0,*__errno_location(),"failed to read file names from file system at or below %s",v2);
  dat_499e4 = 1;
  return 0;
}


// Function: sub_82e7 @ 0x82e7
unsigned long sub_82e7(int a0,unsigned long *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_82e7
{
  char *v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  long v13; // stack - 0x148
  long v14; // stack - 0xb8
  unsigned short v15; // stack - 0x22
  unsigned int v16; // stack - 0x178
  unsigned char v17; // stack - 0x17a
  int v18; // stack - 0x174
  void *v19; // stack - 0x170
  bool v2; // al
  unsigned long v20; // stack - 0x168
  long v21; // stack - 0x160
  char *v22; // stack - 0x158
  unsigned long v23; // stack - 0x150
  long v24; // stack - 0x140
  long v25; // stack - 0xb0
  char v3; // al
  int v4; // eax
  unsigned long v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  char *v8; // rax
  unsigned long v9; // rax
  
  v2 = ((1 <= a0) && (v5 = *a1, sub_ff96(v5,1) != '\x01'));
  v19 = NULL;
  v20 = 0;
  if (dat_499b0) { // branch-flip
    if (v2) {
      v5 = sub_1096a(0,*a1);
      error(0,0,gettext("extra operand %s"),v5);
      error(1,0,"%s",gettext("file operands cannot be combined with -files0-from"));
      return v6;
    }
    v4 = strcmp(dat_499b0,"-");
    if (v4) { // branch-flip
      v20 = sub_1096a(0,dat_499b0);
      v19 = fopen(dat_499b0,"r");
      if (!v19) {
        v8 = gettext("cannot open %s for reading");
        error(1,*__errno_location(),v8,v20);
        return v9;
      }
      v18 = fileno(v19);
      if (v18 <= -1)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_499b8) && (v4 = fstat(v18,&v13), !v4)) && (!fstat(0,&v14))) && ((v24 == v25 && (v13 == v14)))) {
        error(1,0,"%s: %s\n",gettext("option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir"),v20);
        return v10;
      }
      sub_1d5ba(v18,1);
    }
    else {
      if (dat_499b8) {
        error(1,0,"%s\n",gettext("option -files0-from reading from standard input cannot be combined with -ok, -okdir"));
        return v7;
      }
      v8 = gettext("(standard input)");
      v20 = sub_1096a(0,v8);
      v19 = stdin;
    }
    v21 = sub_1d0ff(v19);
  }
  else {
    if (!v2) {
      v15 = 0x2e;
      return sub_803a(&v15);
    }
    v21 = sub_1d0aa(a1);
  }
  if (!v21)
    sub_321fa(); // no-return
  v17 = 1;
  do {
    while( true ) {
      v8 = (char *)sub_1d16c(v21,&v16);
      v22 = v8;
      if (!v8) {
        switch(v16) {
          default:
            __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
          case 2:
            goto label_8852;
          case 3:
            sub_321fa(); // no-return
          case 4:
            v8 = gettext("%s: read error");
            error(0,*__errno_location(),v8,v20);
            dat_499e4 = 1;
            v17 = 0;
          
        }
        goto label_8852;
      }
      if (*v8) break;
      if (dat_499b0) { // branch-flip
        v23 = sub_1d257(v21);
        error(0,0,"%s:%lu: %s",v20,v23,gettext("invalid zero-length file name"));
      }
      else {
        error(0,2,"%s",sub_1096a(0,v8));
      }
      dat_499e4 = 1;
      v17 = 0;
    }
    v1 = v8;
    if ((!dat_499b0) && (v1 = v22, sub_ff96(v8,1))) goto label_8852;
    v22 = v1;
    v11 = strlen(v22);
    dat_499dc = (unsigned int)v11;
    v3 = sub_803a(v22);
  } while (v3 == '\x01');
  v17 = 0;
label_8852:
  sub_1d295(v21);
  if (((v17) && (dat_499b0)) && ((v4 = ferror(v19), v4 || (v4 = sub_1dad5(v19), v4)))) {
    error(1,0,gettext("error reading %s"),v20);
    return v12;
  }
  return (unsigned long)v17;
}


// Function: sub_88ed @ 0x88ed
unsigned long sub_88ed(int a0,long *a1)
{
  int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  
  if (*a1) // branch-flip
    sub_2dd35(*a1);
  else {
    sub_2dd35("find");
  }
  sub_f78b();
  dat_499f8 = 0;
  dat_499e4 = 0;
  dat_499e8 = 0;
  dat_499d8 = 0xffffff9c;
  if (sub_1b5f4())
    sub_1b20a();
  dat_499f0 = sub_10be9("w");
  if (!dat_499f0) {
    v2 = gettext("Failed to initialize shared-file hash table");
    error(1,*__errno_location(),v2);
    return v3;
  }
  sub_106bd(0x49940);
  setlocale(6,"");
  bindtextdomain("findutils","/usr/local/share/locale");
  textdomain("findutils");
  if (sub_37150(sub_1d697)) {
    v2 = gettext("The atexit library function failed");
    error(1,*__errno_location(),v2);
    return v4;
  }
  v1 = sub_103e8(a0,a1);
  if (dat_49980 & 2)
    dat_49990 = sub_fd39;
  if (dat_49980 & 0x80)
    fprintf(stderr,"cur_day_start = %s",ctime(0x49968));
  v5 = sub_dd20(a0,a1,v1);
  if (sub_82e7(a0 - v1,&a1[v1])) {
    sub_ae53(v5);
    sub_fa38();
  }
  return (unsigned long)dat_499e4;
}


// Function: sub_8b12 @ 0x8b12
unsigned long sub_8b12(unsigned int *a0)
{
  *a0 = dat_49010;
  return 1;
}


// Function: sub_8b31 @ 0x8b31
double sub_8b31(unsigned long a0,long a1,unsigned long a2,long a3)
{
  double v1; // xmm0_qa
  
  difftime(a0,a2);
  return v1 + dat_389e8 * (double)(a1 - a3);
}


// Function: sub_8ba9 @ 0x8ba9
unsigned long sub_8ba9(long a0,long a1,long a2,long a3) // return-dupe x2
{
  double v1; // xmm0_qa
  
  if ((a0 == a2) && (a1 == a3))
    return 0;
  sub_8b31(a0,a1,a2,a3);
  if (v1 < 0.0)
    return 0xffffffff;
  return 1;
}


// Function: sub_8c32 @ 0x8c32
unsigned long sub_8c32(unsigned long a0,unsigned long a1,long a2,int a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_8c32
{
  unsigned long v1;
  unsigned int v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  v2 = *(unsigned int *)(a2 + 0x3c);
  if (v2 != 2) { // branch-flip
    if (2 < v2)
      __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return, return-dupe
    if (v2) { // branch-flip
      if (v2 != 1) {
        __assert_fail("0","pred.c",0x8a,"pred_timewindow");
      }
      v2 = sub_8ba9(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
      v2 >>= 0x1f;
      v3 = (unsigned long)v2;
    }
    else {
      v3 = sub_8ba9(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
      v1 = v3 >> 8;
      v3 = CONCAT71((undefined7)v1,0 < (int)v2);
    }
  }
  else {
    sub_8b31(a0,a1,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    v2 = !((v4 <= 0.0) || ((double)a3 < v4));
    v3 = (unsigned long)v2;
  }
  return v3;
}


// Function: sub_8d51 @ 0x8d51
void sub_8d51(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a4c(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}


// Function: sub_8d9c @ 0x8d9c
unsigned long sub_8d9c(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  if ((*(long *)(a2 + 0x110)) && (v1 = *(unsigned long *)(a2 + 0x110), !sub_10873(a0,a1,v1)))
    return 0;
  v1 = *(unsigned long *)(a2 + 0x118);
  return sub_10873(a0,a1,v1);
}


// Function: sub_8e0d @ 0x8e0d
unsigned long sub_8e0d(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
  v2 = sub_30a4c(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}


// Function: sub_8e96 @ 0x8e96
void sub_8e96(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a4c(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}


// Function: sub_8ee1 @ 0x8ee1
unsigned long sub_8ee1(void)
{
  return 1;
}


// Function: sub_8efc @ 0x8efc
void sub_8efc(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a66(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}


// Function: sub_8f47 @ 0x8f47
unsigned long sub_8f47(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
  v2 = sub_30a66(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}


// Function: sub_8fd0 @ 0x8fd0
void sub_8fd0(unsigned long a0,unsigned long a1,long a2)
{
  if (*(long *)(a2 + 0x110))
    sub_10873(a0,a1,*(unsigned long *)(a2 + 0x110));
  sub_10873(a0,a1,*(unsigned long *)(a2 + 0x118));
}


// Function: sub_9036 @ 0x9036
void sub_9036(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a66(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}


// Function: sub_9081 @ 0x9081
unsigned long sub_9081(int a0)
{
  int v1; // eax
  
  v1 = unlinkat(dat_499d8,dat_499d0,a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}


// Function: sub_90b1 @ 0x90b1
unsigned long sub_90b1(unsigned long a0,long a1) // return-dupe x3
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  int v4; // stack - 0x1c
  
  v1 = strcmp(dat_499d0,".");
  if (!v1)
    return 1;
  v4 = 0;
  if ((dat_499c4) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000))
    v4 = 0x200;
  if (sub_9081(v4))
    return 1;
  if ((*__errno_location() == 2) && (dat_4994e)) {
    *__errno_location() = 0;
    return 1;
  }
  if ((*__errno_location() == 0x15) && ((!v4 && (sub_9081(0x200)))))
    return 1;
  v2 = sub_1096a(0,a0);
  v3 = gettext("cannot delete %s");
  error(0,*__errno_location(),v3,v2);
  dat_499e4 = 1;
  return 0;
}


// Function: sub_91f4 @ 0x91f4
unsigned long sub_91f4(unsigned long a0,long a1) // early-return x5
{
  int v1; // eax
  unsigned long v2; // rax
  void *v3; // rax
  unsigned long v4; // rax
  unsigned char v5; // stack - 0x2d
  void *v6; // stack - 0x28
  
  if ((*(unsigned int *)(a1 + 0x18) & 0xf000) != 0x4000) { // branch-flip
    if ((*(unsigned int *)(a1 + 0x18) & 0xf000) != 0x8000)
      return 0;
    v4 = CONCAT71((undefined7)((unsigned long)*(long *)(a1 + 0x30) >> 8),*(long *)(a1 + 0x30) == 0);
  }
  else {
    v5 = 1;
    *__errno_location() = 0;
    v1 = sub_2704e(dat_499d8,dat_499d0,0x90900);
    if (v1 <= -1) {
      v2 = sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      return 0;
    }
    v3 = fdopendir(v1);
    if (!v3) {
      v2 = sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      close(v1);
      return 0;
    }
    *__errno_location() = 0;
    v6 = readdir(v3);
    while (v6) {
      if ((*(char *)((long)v6 + 0x13) != '.') || ((*(char *)((long)v6 + 0x14) && ((*(char *)((long)v6 + 0x14) != '.' || (*(char *)((long)v6 + 0x15))))))) {
        v5 = 0;
        break;
      }
      v6 = readdir(v3);
    }
    if (*__errno_location()) {
      v2 = sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      closedir(v3);
      return 0;
    }
    if (closedir(v3)) {
      v2 = sub_1096a(0,a0);
      error(0,*__errno_location(),"%s",v2);
      dat_499e4 = 1;
      return 0;
    }
    v4 = (unsigned long)v5;
  }
  return v4;
}


// Function: sub_9466 @ 0x9466
void sub_9466(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_b300(a0,a1,a2);
}


// Function: sub_9497 @ 0x9497
void sub_9497(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_b300(dat_499d0,a1,a2);
}


// Function: sub_94cb @ 0x94cb
unsigned long sub_94cb(void)
{
  return 0;
}


// Function: sub_94e6 @ 0x94e6
unsigned long sub_94e6(unsigned long a0,unsigned long a1,long a2)
{
  sub_1b7b3(a0,dat_499d8,dat_499d0,a1,dat_49958,dat_4997c,*(char *)(a2 + 0x28),*(unsigned long *)(a2 + 0x40));
  return 1;
}


// Function: sub_954f @ 0x954f
unsigned long sub_954f(unsigned long a0,unsigned long a1,long a2)
{
  sub_1c643(*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x58),*(char *)(a2 + 0x50),"%s\n",a0);
  return 1;
}


// Function: sub_959f @ 0x959f
unsigned long sub_959f(char *a0,unsigned long a1,long a2)
{
  void *v1;
  
  v1 = *(void **)(a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}


// Function: sub_95ee @ 0x95ee
bool sub_95ee(unsigned long a0,unsigned long a1,long a2)
{
  char *v1;
  char *v2; // rax
  
  v1 = *(char **)(a2 + 0x38);
  v2 = (char *)sub_13a18(a1,a0);
  return strcmp(v2,v1) == 0;
}


// Function: sub_9643 @ 0x9643
unsigned long sub_9643(unsigned long a0,long a1,long a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)(a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && ((unsigned long)*(unsigned int *)(a1 + 0x20) < *(unsigned long *)(a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x20))
        return 1;
    }
  }
  else if ((unsigned long)*(unsigned int *)(a1 + 0x20) == *(unsigned long *)(a2 + 0x40))
    return 1;
  return 0;
}


// Function: sub_96d8 @ 0x96d8
bool sub_96d8(unsigned long a0,long a1,long a2)
{
  return *(int *)(a2 + 0x38) == *(int *)(a1 + 0x20);
}


// Function: sub_970c @ 0x970c
void sub_970c(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_9975(a0,a1,a2,1);
}


// Function: sub_973f @ 0x973f
bool sub_973f(unsigned long a0,char *a1,int a2)
{
  int v1; // eax
  char *v2; // rax
  
  v2 = (char *)sub_1d85c(a0);
  sub_1da6f(v2);
  v1 = fnmatch(a1,v2,a2);
  free(v2);
  return v1 == 0;
}


// Function: sub_97a2 @ 0x97a2
void sub_97a2(unsigned long a0,unsigned long a1,long a2)
{
  sub_973f(a0,*(unsigned long *)(a2 + 0x38),0x10);
}


// Function: sub_97d8 @ 0x97d8
unsigned long sub_97d8(unsigned long a0,long a1,long a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)(a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (*(unsigned long *)(a1 + 8) < *(unsigned long *)(a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 8))
        return 1;
    }
  }
  else if (*(long *)(a1 + 8) == *(long *)(a2 + 0x40))
    return 1;
  return 0;
}


// Function: sub_986a @ 0x986a
bool sub_986a(char *a0,unsigned long a1,long a2)
{
  char *v1;
  
  v1 = *(char **)(a2 + 0x38);
  return fnmatch(v1,a0,0x10) == 0;
}


// Function: sub_98b0 @ 0x98b0
unsigned long sub_98b0(unsigned long a0,long a1,long a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)(a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (*(unsigned long *)(a1 + 0x10) < *(unsigned long *)(a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)(a2 + 0x40) < *(unsigned long *)(a1 + 0x10))
        return 1;
    }
  }
  else if (*(long *)(a1 + 0x10) == *(long *)(a2 + 0x40))
    return 1;
  return 0;
}


// Function: sub_9942 @ 0x9942
void sub_9942(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_9975(a0,a1,a2,0);
}


// Function: sub_9975 @ 0x9975
char sub_9975(unsigned long a0,long a1,long a2,char a3) // return-dupe, ternary
{
  char *v1;
  char *v2; // rax
  int v3; // edx
  char v4; // stack - 0x11
  
  v4 = 0;
  if ((*(unsigned int *)(a1 + 0x18) & 0xf000) != 0xa000)
    return 0;
  v2 = (char *)sub_1d06d(dat_499d8,dat_499d0);
  if (v2) { // branch-flip
    v3 = (a3) ? 0x10 : 0; // branch-flip
    v1 = *(char **)(a2 + 0x38);
    if (!fnmatch(v1,v2,v3))
      v4 = 1;
  }
  else {
    sub_10a13(*__errno_location(),a0);
    dat_499e4 = 1;
  }
  free(v2);
  return v4;
}


// Function: sub_9a33 @ 0x9a33
void sub_9a33(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_94e6(a0,a1,a2);
}


// Function: sub_9a64 @ 0x9a64
void sub_9a64(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a80(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x3c);
}


// Function: sub_9aaf @ 0x9aaf
void sub_9aaf(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  
  v1 = sub_30a80(a1);
  sub_8c32(SUB168(v1,0),SUB168(v1,8),a2,0x15180);
}


// Function: sub_9afa @ 0x9afa
void sub_9afa(unsigned long a0,unsigned long a1,long a2)
{
  sub_973f(a0,*(unsigned long *)(a2 + 0x38),0);
}


// Function: sub_9b30 @ 0x9b30
bool sub_9b30(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a2 + 0x118);
  return sub_10873(a0,a1,v1) == '\0';
}


// Function: sub_9b79 @ 0x9b79
unsigned long sub_9b79(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1; // rax
  char v2 [16];
  
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
  v2 = sub_30a80(a1);
  v1 = sub_8ba9(SUB168(v2,0),SUB168(v2,8),*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),0 < (int)v1);
}


// Function: sub_9c02 @ 0x9c02
unsigned long sub_9c02(unsigned long a0,unsigned long a1,long a2) // early-return x2
{
  bool v1;
  unsigned long v2; // rax
  char v3 [16];
  unsigned long v4; // stack - 0x28
  long v5; // stack - 0x20
  
  v1 = 0;
  if (*(int *)(a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)(a2 + 0x38)) {
    case 0:
      v3 = sub_30a4c(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 1:
      v3 = sub_30a9a(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      if (v5 <= -1) {
        v2 = sub_1096a(0,a0);
        error(0,0,gettext("WARNING: cannot determine birth time of file %s"),v2);
        return 0;
      }
      break;
    case 2:
      v3 = sub_30a66(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 3:
      v3 = sub_30a80(a1);
      v5 = SUB168(v3,8);
      v4 = SUB168(v3,0);
      v1 = 1;
      break;
    case 4:
      if (*(int *)(a2 + 0x38) != 4)
        return 0;
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    
  }
  if (v1) {
    v2 = sub_8ba9(v4,v5,*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x48));
    return CONCAT71((undefined7)((unsigned long)v2 >> 8),0 < (int)v2);
  }
  __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
}


// Function: sub_9dd5 @ 0x9dd5
unsigned long sub_9dd5(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)(a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_9e03 @ 0x9e03
unsigned long sub_9e03(unsigned long a0,long a1)
{
  void *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)(a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}


// Function: sub_9e31 @ 0x9e31
void sub_9e31(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  char *v2; // rax
  
  sub_1e1a8(stdout);
  v1 = fprintf(stderr,gettext("< %s ... %s > ? "),a0,a1);
  if (v1 <= -1) {
    v2 = gettext("Failed to write prompt for -ok");
    error(1,*__errno_location(),v2);
    return;
  }
  sub_1e1a8(stderr);
  sub_32d7c();
}


// Function: sub_9ece @ 0x9ece
unsigned long sub_9ece(unsigned long a0,unsigned long a1,long a2) // early-return
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (sub_9e31(v1,a0))
    return sub_b300(a0,a1,a2);
  return 0;
}


// Function: sub_9f27 @ 0x9f27
unsigned long sub_9f27(unsigned long a0,unsigned long a1,long a2) // early-return
{
  unsigned long v1;
  
  v1 = **(unsigned long **)(a2 + 0xe8);
  if (sub_9e31(v1,a0)) {
    v1 = sub_b300(dat_499d0,a1,a2);
    return v1;
  }
  return 0;
}


// Function: sub_9f83 @ 0x9f83
unsigned long sub_9f83(void)
{
  return 1;
}


// Function: sub_9f9e @ 0x9f9e
unsigned long sub_9f9e(unsigned long a0,unsigned long a1,long a2)
{
  unsigned long v1;
  
  if ((*(long *)(a2 + 0x110)) && (v1 = *(unsigned long *)(a2 + 0x110), sub_10873(a0,a1,v1) == '\x01'))
    return 1;
  v1 = *(unsigned long *)(a2 + 0x118);
  return sub_10873(a0,a1,v1);
}


// Function: sub_a012 @ 0xa012
bool sub_a012(char *a0,unsigned long a1,long a2)
{
  char *v1;
  
  v1 = *(char **)(a2 + 0x38);
  return fnmatch(v1,a0,0) == 0;
}


// Function: sub_a058 @ 0xa058
unsigned int sub_a058(unsigned long a0,long a1,long a2) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4; // eax
  
  v1 = *(unsigned int *)(a1 + 0x18);
  v4 = v1 & 0xf000;
  v2 = *(unsigned int *)(a2 + 0xc + ((long)(int)(unsigned int)(v4 == 0x4000) + 0xc) * 4);
  v3 = *(unsigned int *)(a2 + 0x38);
  if (v3 != 2) { // branch-flip
    if (2 < v3)
      abort(); // no-return, return-dupe
    if (v3) { // branch-flip
      if (v3 != 1) {
        abort();
      }
      if (!v2) {
        v4 = 1;
        return v4;
      }
      v4 = v1 & v2;
      v4 = CONCAT31((undefined3)(v4 >> 8),v4 != 0);
    }
    else {
      v4 = v1 & v2;
      v4 = CONCAT31((undefined3)(v4 >> 8),v2 == v4);
    }
  }
  else {
    v4 = v1 & 0xfff;
    v4 = CONCAT31((undefined3)(v4 >> 8),v2 == v4);
  }
  return v4;
}


// Function: sub_a0fa @ 0xa0fa
unsigned long sub_a0fa(void)
{
  unsigned long v1; // rax
  
  v1 = faccessat(dat_499d8,dat_499d0,1,0);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}


// Function: sub_a137 @ 0xa137
unsigned long sub_a137(void)
{
  unsigned long v1; // rax
  
  v1 = faccessat(dat_499d8,dat_499d0,4,0);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}


// Function: sub_a174 @ 0xa174
unsigned long sub_a174(void)
{
  unsigned long v1; // rax
  
  v1 = faccessat(dat_499d8,dat_499d0,2,0);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}


// Function: sub_a1b1 @ 0xa1b1
unsigned long sub_a1b1(unsigned long a0,unsigned long a1,long a2)
{
  sub_1c643(*(unsigned long *)(a2 + 0x40),*(unsigned long *)(a2 + 0x58),*(char *)(a2 + 0x50),"%s\n",a0);
  return 1;
}


// Function: sub_a201 @ 0xa201
void sub_a201(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_959f(a0,a1,a2);
}


// Function: sub_a232 @ 0xa232
unsigned long sub_a232(unsigned long a0,long a1)
{
  if (dat_49940) {
    if (!dat_499c4)
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune"); // no-return
    if ((a1) && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000))
      dat_499e0 = 1;
  }
  return 1;
}


// Function: sub_a2b0 @ 0xa2b0
void sub_a2b0(void)
{
  sub_fa38();
  exit(dat_499e4); // no-return
}


// Function: sub_a2da @ 0xa2da
bool sub_a2da(char *a0,unsigned long a1,long a2)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = strlen(a0);
  v1 = *(unsigned long *)(a2 + 0x38);
  return (int)v2 == re_match(v1,a0,v2 & 0xffffffff,0,0);
}


// Function: sub_a336 @ 0xa336
unsigned long sub_a336(unsigned long a0,long a1,long a2) // return-dupe x3
{
  unsigned int v1;
  unsigned long v2; // rax
  
  v2 = (unsigned long)(*(long *)(a1 + 0x30) % (long)*(int *)(a2 + 0x3c) != 0) + *(long *)(a1 + 0x30) / (long)*(int *)(a2 + 0x3c);
  v1 = *(unsigned int *)(a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && (v2 < *(unsigned long *)(a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)(a2 + 0x40) < v2)
        return 1;
    }
  }
  else if (v2 == *(unsigned long *)(a2 + 0x40))
    return 1;
  return 0;
}


// Function: sub_a3f7 @ 0xa3f7
unsigned int sub_a3f7(unsigned long a0,long *a1,long a2) // return-dupe x2, return-dupe
{
  int v1; // eax
  
  if ((a1[1]) && (a1[1] != *(long *)(a2 + 0x38)))
    return 0;
  v1 = sub_f27d(a0,dat_499d0,a1);
  if (v1)
    return 0;
  if ((a1[1] == *(long *)(a2 + 0x38)) && (*a1 == *(long *)(a2 + 0x40)))
    return 1;
  return 0;
}


// Function: sub_a497 @ 0xa497
unsigned long sub_a497(void)
{
  return 1;
}


// Function: sub_a4b2 @ 0xa4b2
unsigned long sub_a4b2(unsigned long a0,long a1,long a2) // return-dupe x2
{
  unsigned int v1; // stack - 0x10
  unsigned int v2; // stack - 0xc
  
  v2 = 7;
  if (!dat_499c5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  if (!dat_499c8)
    return 0;
  if (dat_499c4) // branch-flip
    v1 = *(unsigned int *)(a1 + 0x18);
  else {
    v1 = dat_499c8;
  }
  switch(v1 & 0xf000) { // branch-flip
    case 0x1000:
      v2 = 5;
      break;
    case 0x2000:
      v2 = 1;
      break;
    case 0x4000:
      v2 = 2;
      break;
    case 0x6000:
      v2 = 0;
      break;
    case 0x8000:
      v2 = 3;
      break;
    case 0xa000:
      v2 = 4;
      break;
    case 0xc000:
      v2 = 6;
    
  }
  if ((v2 != 7) && (*(char *)(a2 + 0x38 + (unsigned long)v2)))
    return 1;
  return 0;
}


// Function: sub_a600 @ 0xa600
unsigned long sub_a600(unsigned long a0,long a1,long a2) // return-dupe x3
{
  unsigned int v1;
  
  v1 = *(unsigned int *)(a2 + 0x38);
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if ((v1 == 1) && ((unsigned long)*(unsigned int *)(a1 + 0x1c) < *(unsigned long *)(a2 + 0x40)))
          return 1;
      }
      else if (*(unsigned long *)(a2 + 0x40) < (unsigned long)*(unsigned int *)(a1 + 0x1c))
        return 1;
    }
  }
  else if ((unsigned long)*(unsigned int *)(a1 + 0x1c) == *(unsigned long *)(a2 + 0x40))
    return 1;
  return 0;
}


// Function: sub_a695 @ 0xa695
unsigned long sub_a695(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  char v1 [16];
  char v2 [16];
  long v3; // stack - 0x38
  long v4; // stack - 0x30
  
  v1 = sub_30a4c(a1);
  v2 = sub_30a66(a1);
  if ((int)sub_8ba9(SUB168(v1,0),SUB168(v1,8),SUB168(v2,0),SUB168(v2,8)) <= -1)
    return 0;
  v3 = SUB168(v2,0) - SUB168(v1,0);
  v4 = SUB168(v2,8) - SUB168(v1,8);
  if (v4 <= -1) {
    v4 += 1000000000;
    v3 -= 1;
  }
  return sub_8c32(v3,v4,a2,0x15180);
}


// Function: sub_a75a @ 0xa75a
bool sub_a75a(unsigned long a0,long a1,long a2)
{
  return *(int *)(a2 + 0x38) == *(int *)(a1 + 0x1c);
}


// Function: sub_a78e @ 0xa78e
int * sub_a78e(unsigned long a0,unsigned long a1,unsigned long a2) // early-return, ternary
{
  unsigned long v1;
  char v2; // al
  int v3; // eax
  int *v4; // rax
  char v5 [152];
  void *v6; // stack - 0xc0
  
  v2 = sub_fe03();
  v6 = (v2) ? sub_fcc0 : sub_fc21; // branch-flip
  sub_f26e(v5);
  v3 = (*v6)(dat_499d0,v5);
  if (v3) { // branch-flip
    v2 = sub_fe03();
    if ((!v2) || (v4 = __errno_location(), *v4 != 2)) {
      v4 = (unsigned long)sub_1096a(0,a0);
      v1 = v4;
      v4 = __errno_location();
      error(0,*v4,"%s",v1);
      v4 = NULL;
      dat_499e4 = 1;
      return v4;
    }
    v4 = (unsigned long)sub_a4b2(a0,a1,a2);
  }
  else {
    v4 = (unsigned long)sub_a4b2(a0,v5,a2);
  }
  return v4;
}


// Function: sub_a8d8 @ 0xa8d8
unsigned long sub_a8d8(unsigned long a0,unsigned long a1,long a2) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // stack - 0x28
  
  v1 = (*dat_499a0)(dat_499d8,dat_499d0,&v4);
  if (v1 <= -1) {
    v2 = sub_1096a(0,a0);
    v3 = gettext("getfilecon failed: %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  v1 = fnmatch(*(char **)(a2 + 0x38),v4,0);
  return CONCAT71((undefined7)((unsigned long)freecon(v4) >> 8),v1 == 0);
}


// Function: sub_a9cd @ 0xa9cd
char * sub_a9cd(char *a0,char *a1) // early-return
{
  int v1;
  int v2; // stack - 0xc
  
  if (!a0)
    return NULL;
  strcpy(a1,a0);
  v2 = (int)strlen(a1);
  do {
    v1 = v2;
    v2 = v1 + -1;
    if (v2 <= -1) break;
  } while ((a1[v2] == ' ') || (a1[v2] == '\t'));
  a1[v1] = '\0';
  return a1;
}


// Function: sub_aa62 @ 0xaa62
void sub_aa62(void *a0,long a1)
{
  unsigned long v1;
  char v2 [264];
  long v3; // stack - 0x120
  
  for (v3 = a1; v3; v3 = *(long *)(v3 + 0x108)) {
    v1 = *(unsigned long *)(v3 + 8);
    fprintf(a0,"[%s] ",sub_a9cd(v1,v2));
  }
  fprintf(a0,"\n");
}


// Function: sub_ab30 @ 0xab30
void sub_ab30(void *a0,long *a1) // return-dupe x3
{
  bool v1;
  
  v1 = 0;
  if (!a1)
    return;
  if ((((void *)*a1 == sub_9f9e) || ((void *)*a1 == sub_8d9c)) && (!a1[0x22])) {
    sub_ab30(a0,a1[0x23]);
    return;
  }
  if ((a1[0x22]) || (a1[0x23]))
    v1 = 1;
  if (v1)
    fprintf(a0,"%s"," ( ");
  sub_ac38(a0,a1);
  if (!v1)
    return;
  fprintf(a0,"%s"," ) ");
}


// Function: sub_ac38 @ 0xac38
void sub_ac38(void *a0,long a1) // return-dupe, ternary x3
{
  char *v1; // rax
  char *v2; // rcx
  char *v3; // rdx
  
  if (!a1)
    return;
  sub_ab30(a0,*(unsigned long *)(a1 + 0x110));
  v2 = (*(char *)(a1 + 0x1c)) ? "[need inum] " : ""; // branch-flip
  v3 = (*(char *)(a1 + 0x1b)) ? "[need type] " : ""; // branch-flip
  v1 = (*(char *)(a1 + 0x1a)) ? "[call stat] " : ""; // branch-flip
  fprintf(a0,"%s%s%s",v1,v3,v2);
  sub_c2dc(a0,a1);
  fprintf(a0," [est success rate %.4g] ");
  if (dat_49980 & 0x40) {
    fprintf(a0,"[real success rate %lu/%lu",*(unsigned long *)(a1 + 0x128),*(unsigned long *)(a1 + 0x120));
    if (*(long *)(a1 + 0x120)) // branch-flip
      fprintf(a0,"=%.4g] ");
    else {
      fprintf(a0,"=_] ");
    }
  }
  sub_ab30(a0,*(unsigned long *)(a1 + 0x118));
}


// Function: sub_ae53 @ 0xae53
void sub_ae53(unsigned long a0) // return-dupe
{
  if (!(dat_49980 & 0x40))
    return;
  fprintf(stderr,"Predicate success rates after completion:\n");
  sub_ac38(stderr,a0);
  fprintf(stderr,"\n");
}


// Function: sub_aec7 @ 0xaec7
void sub_aec7(long *a0)
{
  long *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (!v1)
      return;
    if (!*v1)
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check"); // no-return
    if (!v1[0x26])
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check"); // no-return
    if ((*(long *)(v1[0x26] + 0x18)) && (*(long *)(v1[0x26] + 0x18) != *v1)) break;
    switch(*(unsigned int *)v1[0x26]) {
      case 0:
      case 2:
        if (!*(int *)v1[0x26])
          __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check"); // no-return
        if (*(int *)v1[0x26] == 2)
          __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,"pred_sanity_check"); // no-return
        break;
      case 1:
      case 3:
      case 4:
      case 5:
        if (*(char *)((long)v1 + 0x19) == '\x01')
          __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check"); // no-return
        if ((char)v1[3] == '\x01')
          __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check"); // no-return
        break;
      case 6:
        if (!(char)v1[3])
          __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check"); // no-return
        if ((((void *)*v1 != sub_a232) && ((void *)*v1 != sub_a2b0)) && (!*(char *)((long)v1 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    v1 = (long *)v1[0x21];
  }
  __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check"); // no-return
}


// Function: sub_b184 @ 0xb184
bool sub_b184(long a0,unsigned int a1,unsigned long a2)
{
  unsigned int *v1;
  bool v2; // sf
  
  *(unsigned long *)(a0 + 0xc0) = sub_31bc2(0x10);
  *(unsigned long *)(*(long *)(a0 + 0xc0) + 8) = 0;
  v1 = *(unsigned int **)(a0 + 0xc0);
  *v1 = sub_2704e(a1,a2,0);
  v2 = 0 <= **(int **)(a0 + 0xc0);
  if (v2)
    sub_1d5ba(**(unsigned int **)(a0 + 0xc0),1);
  return v2;
}


// Function: sub_b22b @ 0xb22b
char * sub_b22b(long a0) // early-return
{
  void *v1;
  unsigned char v2; // al
  char *v3; // rax
  
  if (*(int *)(a0 + 0x98)) {
    v3 = (char *)0x1;
    return v3;
  }
  if (!*(int *)(a0 + 0x98)) {
    v3 = strchr(dat_499d0,0x2f);
    if (v3) { // branch-flip
      v3 = (void *)sub_1d9d4(dat_499d0);
      v1 = v3;
      v2 = sub_b184(a0,dat_499d8,v3);
      free(v1);
      v3 = (unsigned long)(unsigned long)v2;
    }
    else {
      v3 = (unsigned long)sub_b184(a0,dat_499d8,".");
    }
    return v3;
  }
  __assert_fail("!execp->state.todo","exec.c",0x47,"record_exec_dir"); // no-return
}


// Function: sub_b300 @ 0xb300
unsigned long sub_b300(char *a0,unsigned long a1,unsigned long *a2) // ternary
{
  char v1; // al
  char *v10; // stack - 0x38
  char *v11; // stack - 0x30
  unsigned long v12; // stack - 0x28
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned char v7; // stack - 0x46
  int v8; // stack - 0x44
  char *v9; // stack - 0x40
  
  v9 = NULL;
  v1 = sub_10933(*a2);
  if (v1) { // branch-flip
    if (sub_b22b(&a2[7]) != '\x01') {
      v2 = sub_1096a(0,a0);
      v3 = gettext("Failed to save working directory in order to run a command on %s");
      error(1,*__errno_location(),v3,v2);
      return v4;
    }
    v9 = (char *)sub_1d85c(dat_499d0);
    v10 = v9;
    if (*v9 != '/') { // branch-flip
      v11 = "./";
      v12 = 2;
    }
    else {
      v11 = NULL;
      v12 = 0;
    }
  }
  else {
    if (a2[0x1f] != dat_49a00)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v11 = NULL;
    v12 = 0;
    v10 = a0;
  }
  if (*(char *)&a2[7]) { // branch-flip
    sub_1a1b8(&a2[8],&a2[0x13],v10,strlen(v10) + 1,v11,v12,0);
    if (*(int *)&a2[0x1a])
      dat_499e8 = 1;
    v7 = 1;
  }
  else {
    for (v8 = 0; v8 < *(int *)&a2[0x1e]; v8 = v8 + 1) {
      v5 = strlen(v10);
      v6 = strlen(*(char **)((long)v8 * 8 + a2[0x1d]));
      sub_19af7(&a2[8],&a2[0x13],*(unsigned long *)((long)v8 * 8 + a2[0x1d]),v6,v11,v12,v10,v5,0);
    }
    sub_19f5a(&a2[8],&a2[0x13]);
    if (*(unsigned int *)&a2[0x20] & 0x7f) // branch-flip
      v7 = 0;
    else {
      v7 = (*(int *)&a2[0x20] >> 8 & 0xffU) ? 0 : 1; // branch-flip
    }
    if (v1)
      sub_2fe52(a2[0x1f]);
  }
  if (v9) {
    if (!v1)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // no-return
    free(v9);
  }
  return (unsigned long)v7;
}


// Function: sub_b615 @ 0xb615
char sub_b615(char a0,int *a1) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  unsigned long v3; // stack - 0x3a
  unsigned long v4;
  char v5; // stack - 0x3b
  unsigned short v6; // stack - 0x32
  
  v5 = 1;
  if (a0) {
    v3 = 0x6c756e2f7665642f;
    v6 = 0x6c;
    if (0 <= close(0)) { // branch-flip
      if ((int)sub_1e034(&v3,0) <= -1) {
        v4 = sub_1096a(0,&v3);
        error(0,*__errno_location(),"%s",v4);
      }
    }
    else {
      v1 = gettext("Cannot close standard input");
      error(0,*__errno_location(),v1);
      v5 = 0;
    }
  }
  if (!sub_2fe17(a1))
    return v5;
  if ((0 <= *a1) || (!*(long *)&a1[2]))
    v4 = 0x38a9e;
  else {
    v4 = *(unsigned long *)&a1[2];
  }
  if ((0 <= *a1) || (!*(long *)&a1[2]))
    v1 = "";
  else {
    v1 = ": ";
  }
  v2 = gettext("Failed to change directory%s%s");
  error(0,*__errno_location(),v2,v1,v4);
  return 0;
}


// Function: sub_b7ae @ 0xb7ae
unsigned long sub_b7ae(unsigned long a0,char *a1,unsigned long a2,unsigned long *a3) // return-dupe x2
{
  char v1;
  int v2; // eax
  unsigned int v3;
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long v6;
  int v7; // stack - 0x2c
  
  if (dat_49980 & 0x20) {
    fprintf(stderr,"DebugExec: launching process (argc=%lu):",*(long *)&a1[0x60] + -1);
    for (v7 = 0; (unsigned long)(long)v7 < (unsigned long)(*(long *)&a1[0x60] - 1U); v7 = v7 + 1) {
      v6 = *(unsigned long *)((long)v7 * 8 + *(long *)&a1[0x68]);
      fprintf(stderr," %s",sub_1096a(0,v6));
    }
    fprintf(stderr,"\n");
  }
  sub_1e1a8(stdout);
  sub_1e1a8(stderr);
  if (dat_4901c) {
    dat_4901c = 0;
    signal(0x11,0);
  }
  v2 = fork();
  if (v2 == -1) {
    v4 = gettext("cannot fork");
    error(1,*__errno_location(),v4);
    return v5;
  }
  if (!v2) {
    if (!*(long *)&a1[0xc0])
      __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
    v6 = *(unsigned long *)&a1[0xc0];
    v1 = a1[0xbc];
    if (sub_b615(v1,v6) != '\x01')
      _exit(1); // no-return
    if (sub_1b5f4())
      sub_1b61e();
    if (sub_1a94d(a3)) // branch-flip
      *__errno_location() = 7;
    else {
      execvp(*a3,a3);
    }
    v6 = sub_1096a(0,*a3);
    error(0,*__errno_location(),"%s",v6);
    _exit(1); // no-return
  }
  while (waitpid(v2,(int *)&a1[200],0) == -1) {
    if (*__errno_location() != 4) {
      v6 = sub_1096a(0,*a3);
      v4 = gettext("error waiting for %s");
      error(0,*__errno_location(),v4,v6);
      dat_499e4 = 1;
      return 0;
    }
  }
  if ((char)(((unsigned char)*(unsigned int *)&a1[200] & 0x7f) + 1) >> 1 <= '\0') {
    v3 = *(int *)&a1[200] >> 8 & 0xff;
    if (dat_49980 & 0x20)
      fprintf(stderr,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v2,(unsigned long)v3);
    if (v3) {
      if (!*a1)
        return 1;
      dat_499e4 = 1;
      return 1;
    }
    return 1;
  }
  v3 = *(unsigned int *)&a1[200];
  v6 = sub_2f861(0,dat_499ac,*a3);
  error(0,0,gettext("%s terminated by signal %d"),v6,(unsigned long)(v3 & 0x7f));
  if (!*a1)
    return 1;
  dat_499e4 = 1;
  return 1;
}


// Function: sub_bb9d @ 0xbb9d
unsigned long sub_bb9d(char *a0,char a1) // early-return
{
  char *v1;
  int v2; // eax
  int v3; // stack - 0x14
  unsigned long v4; // stack - 0x10
  
  v3 = 0;
  if (a1)
    v3 = 0x10;
  if (dat_49a28) {
    v4 = 0;
    while( true ) {
      if (dat_49a28 <= v4)
        return 0;
      v1 = *(char **)(v4 * 8 + dat_49a20);
      if (!fnmatch(a0,v1,v3)) break;
      v4 += 1;
    }
    return 1;
  }
  v2 = fnmatch(a0,".",v3);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
}


// Function: sub_bc44 @ 0xbc44
char * sub_bc44(long *a0,short a1,long a2)
{
  long v1;
  unsigned long v2;
  char *v3; // rax
  long v4; // stack - 0x28
  
  v1 = *a0;
  if (!*a0) {
    v3 = gettext("invalid expression");
    error(1,0,v3);
    return v3;
  }
  switch(*(unsigned int *)(*a0 + 0x10)) {
    case 0:
      v3 = gettext("invalid expression");
      error(1,0,v3);
      goto label_bd00;
    case 2:
      v4 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      v3 = (unsigned long)sub_bc44(a0,4,v4);
      *(char **)(v4 + 0x118) = v3;
      break;
    case 3:
label_bd00:
      v2 = *(unsigned long *)(v1 + 8);
      v3 = gettext("invalid expression; you have used a binary operator \'%s\' with nothing before it.");
      error(1,0,v3,v2);
      goto label_bd31;
    case 4:
      if ((!*(long *)(*a0 + 0x108)) || (*(char *)(*(long *)(*a0 + 0x108) + 0x29))) {
        v2 = *(unsigned long *)(v1 + 8);
        v3 = gettext("invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'");
        error(1,0,v3,v2);
      }
      v1 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      if (*(int *)(*a0 + 0x10) == 5) {
        if (*(char *)(v1 + 0x29)) {
          v2 = *(unsigned long *)(*a0 + 8);
          v3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'.");
          error(1,0,v3,v2);
        }
        v3 = gettext("invalid expression; empty parentheses are not allowed.");
        error(1,0,v3);
      }
      v3 = (long)sub_bc44(a0,0,v1);
      v4 = (long)v3;
      if ((!*a0) || (*(int *)(*a0 + 0x10) != 5)) {
        v3 = gettext("invalid expression; I was expecting to find a \')\' somewhere but did not see one.");
        error(1,0,v3);
      }
      *a0 = *(long *)(*a0 + 0x108);
      break;
    case 5:
label_bd31:
      if (!a2) {
        v2 = *(unsigned long *)(v1 + 8);
        v3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'.");
        error(1,0,v3,v2);
      }
      if (((*(int *)(a2 + 0x10) == 2) || (*(int *)(a2 + 0x10) == 3)) && (*(char *)(v1 + 0x29) != '\x01')) {
        v2 = *(unsigned long *)(a2 + 8);
        v3 = gettext("expected an expression between \'%s\' and \')\'");
        error(1,0,v3,v2);
      }
      if (*(char *)(*a0 + 0x29)) {
        v2 = *(unsigned long *)(a2 + 8);
        v3 = gettext("expected an expression after \'%s\'");
        error(1,0,v3,v2);
      }
      v3 = gettext("invalid expression; you have too many \')\'");
      error(1,0,v3);
      goto label_be27;
    case 1:
      v4 = *a0;
      *a0 = *(long *)(*a0 + 0x108);
      break;
    default:
      v3 = gettext("oops -- invalid expression type!");
      error(1,0,v3);
      return v3;
    
  }
  if (*a0) { // branch-flip
    if ((int)a1 < *(int *)(*a0 + 0x14)) {
      v3 = (long)sub_c08f(a0,v4,(int)a1);
      v4 = (long)v3;
      if (!v3) {
        v3 = gettext("invalid expression");
        error(1,0,v3);
        return v3;
      }
    }
    v3 = (char *)v4;
  }
  else {
    v3 = (char *)v4;
  }
  return v3;
}


// Function: sub_c08f @ 0xc08f
long sub_c08f(long *a0,long a1,short a2) // early-return
{
  unsigned int v1;
  unsigned int v2;
  long v3;
  long v4; // rax
  long v5; // rax
  long v6; // stack - 0x28
  
  if ((!*a0) || (*(int *)(*a0 + 0x10) == 5))
    return 0;
  v6 = a1;
  while( true ) {
    if (((!*a0) || (*(int *)(*a0 + 0x14) <= (int)a2)) || (v1 = *(unsigned int *)(*a0 + 0x10), v1 == 5))
      return v6;
    if (5 < v1) break;
    if (v1 == 4) {
      error(1,0,gettext("invalid expression")); // return-dupe
      return v4;
    }
    if (4 < v1) break;
    if (v1 <= 2) {
      error(1,0,gettext("invalid expression"));
      return v4;
    }
    if (v1 != 3) break;
    v3 = *a0;
    *(long *)(*a0 + 0x110) = v6;
    v6 = *a0;
    *a0 = *(long *)(*a0 + 0x108);
    v2 = *(unsigned int *)(v6 + 0x14);
    *(unsigned long *)(v6 + 0x118) = sub_bc44(a0,(int)(short)v2,v3);
  }
  v1 = *(unsigned int *)(*a0 + 0x10);
  error(1,0,gettext("oops -- invalid expression type (%d)!"),(unsigned long)v1);
  return v5;
}


// Function: sub_c209 @ 0xc209
unsigned int sub_c209(long *a0) // early-return x3
{
  if (((((void *)*a0 == sub_9afa) || ((void *)*a0 == sub_a012)) || ((void *)*a0 == sub_97a2)) || ((void *)*a0 == sub_986a))
    return 1;
  if (!dat_499a8)
    return 0;
  if ((((void *)*a0 != sub_8d9c) && ((void *)*a0 != sub_9b30)) && (((void *)*a0 != sub_8fd0 && ((void *)*a0 != sub_9f9e))))
    return CONCAT31((undefined3)((unsigned int)(int)a0[4] >> 8),(int)a0[4] == 0);
  return 0;
}


// Function: sub_c2dc @ 0xc2dc
void sub_c2dc(void *a0,long a1) // return-dupe
{
  if (*(long *)(a1 + 0x30)) {
    fprintf(a0,"%s %s",*(unsigned long *)(a1 + 8),*(unsigned long *)(a1 + 0x30));
    return;
  }
  fprintf(a0,"%s",*(unsigned long *)(a1 + 8));
}


// Function: sub_c34d @ 0xc34d
void sub_c34d(unsigned long *a0)
{
  a0[1] = 0;
  *a0 = a0[1];
}


// Function: sub_c377 @ 0xc377
void sub_c377(long *a0,long a1,unsigned long *a2) // return-dupe
{
  *a2 = *(unsigned long *)(a1 + 0x110);
  *(long *)(a1 + 0x110) = *a0;
  *a0 = a1;
  if (a0[1])
    return;
  a0[1] = *a0;
}


// Function: sub_c3e1 @ 0xc3e1
unsigned long sub_c3e1(long a0,long a1,char a2) // return-dupe x6
{
  if (*(int *)(a0 + 0x20) != *(int *)(a1 + 0x20)) {
    if (*(unsigned int *)(a0 + 0x20) < *(unsigned int *)(a1 + 0x20))
      return 0xffffffff;
    return 1;
  }
  if (*(float *)(a0 + 0x24) == *(float *)(a1 + 0x24))
    return 0;
  if (!a2) {
    if (*(float *)(a0 + 0x24) < *(float *)(a1 + 0x24))
      return 1;
    return 0xffffffff;
  }
  if (*(float *)(a0 + 0x24) < *(float *)(a1 + 0x24))
    return 0xffffffff;
  return 1;
}


// Function: sub_c49b @ 0xc49b
void sub_c49b(long *a0,unsigned long a1) // return-dupe
{
  int v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  long v5; // stack - 0x28
  long v6; // stack - 0x38
  long v7; // stack - 0x20
  
  if (!*a0)
    return;
  if (dat_49980 & 8) {
    fprintf(stderr,"%s:\n","predlist before merge sort");
    sub_ea9f(stderr,*a0,2);
  }
  sub_d838(*a0);
  sub_c34d(&v5);
  while (*a0) {
    v2 = *a0;
    *a0 = *(long *)(*a0 + 0x110);
    *(unsigned long *)(v2 + 0x110) = 0;
    v6 = v5;
    while ((v6 && (v1 = *(int *)(v6 + 0x14), v3 = *(unsigned long *)(v2 + 0x118), v4 = *(unsigned long *)(v6 + 0x118), (int)sub_c3e1(v4,v3,v1 != 2) <= -1))) {
      v6 = *(long *)(v6 + 0x110);
    }
    if (v6) { // branch-flip
      *(unsigned long *)(v2 + 0x110) = *(unsigned long *)(v6 + 0x110);
      if (!*(long *)(v2 + 0x110))
        v7 = v2;
      *(long *)(v6 + 0x110) = v2;
    }
    else {
      *(long *)(v2 + 0x110) = v5;
      v5 = v2;
      if (!v7)
        v7 = v2;
    }
  }
  if (dat_49980 & 8) {
    fprintf(stderr,"%s:\n","predlist after merge sort");
    sub_ea9f(stderr,v5,2);
  }
  sub_d838(v5);
  sub_d3fd(v5,v7,a1);
  sub_c34d(a0);
}


// Function: sub_c6e0 @ 0xc6e0
void sub_c6e0(long a0,int a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  int v1; // stack - 0xc
  
  dat_49a38 = sub_c49b;
  sub_c49b(a2,a4);
  (*dat_49a38)(a3,a4);
  for (v1 = 0; v1 < a1; v1 = v1 + 1) {
    (*dat_49a38)((long)v1 * 0x10 + a0,a4);
  }
}


// Function: sub_c77a @ 0xc77a
unsigned int sub_c77a(long a0) // return-dupe
{
  unsigned long v1;
  
  if (!a0)
    return 0;
  if (((!*(char *)(a0 + 0x18)) && (v1 = *(unsigned long *)(a0 + 0x110), !sub_c77a(v1))) && (v1 = *(unsigned long *)(a0 + 0x118), !sub_c77a(v1)))
    return 0;
  return 1;
}


// Function: sub_c7e3 @ 0xc7e3
unsigned int sub_c7e3(long a0) // early-return
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3; // stack - 0x14
  
  if (a0) {
    v2 = sub_c7e3(*(unsigned long *)(a0 + 0x110));
    v1 = v2;
    v2 = sub_c7e3(*(unsigned long *)(a0 + 0x118));
    if (v1 <= v2)
      v1 = v2;
    v2 = v1;
    v3 = v2;
    if (v2 < *(unsigned int *)(a0 + 0x20))
      v3 = *(unsigned int *)(a0 + 0x20);
    v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_c856 @ 0xc856
void sub_c856(long a0)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118);
  *(unsigned long *)(*(long *)(a0 + 0x110) + 0x118) = *(unsigned long *)(a0 + 0x118);
  *(unsigned long *)(a0 + 0x118) = v1;
}


// Function: sub_c8a7 @ 0xc8a7
unsigned long sub_c8a7(unsigned long a0,long *a1) // return-dupe
{
  float v1;
  float v2;
  long v3;
  bool v4;
  long *v5; // rax
  long *v6; // rax
  int v7; // stack - 0x30
  int v8; // stack - 0x2c
  char *v9; // stack - 0x20
  
  v9 = NULL;
  if ((int)a1[2] != 3)
    v9 = "Not a binary operation";
  if ((!v9) && ((!a1[0x22] || (!a1[0x23]))))
    v9 = "Doesn\'t have two arms";
  if ((!v9) && (!*(long *)(a1[0x22] + 0x118)))
    v9 = "Left arm has no child on RHS";
  v5 = &a1[0x23];
  v6 = (long *)(a1[0x22] + 0x118);
  if ((!v9) && (v3 = *v6, sub_c77a(v3)))
    v9 = "Left subtree has side-effects";
  if ((!v9) && (v3 = *v5, sub_c77a(v3)))
    v9 = "Right subtree has side-effects";
  if (!v9) {
    v7 = sub_c7e3(*v6);
    v8 = sub_c7e3(*v5);
    if (v7 < v8)
      v9 = "efficient as-is";
  }
  if (!v9) {
    if (v7 != v8) // branch-flip
      v4 = 1;
    else {
      v1 = *(float *)(*v6 + 0x24);
      v2 = *(float *)(*v5 + 0x24);
      if (dat_49980 & 8)
        fprintf(stderr,"Success rates: l=%f, r=%f\n",a0,(double)v2);
      if ((void *)*a1 != sub_9f9e) { // branch-flip
        if ((void *)*a1 != sub_8d9c) { // branch-flip
          v4 = 0;
          v9 = "Not \'AND\' or \'OR\'";
        }
        else {
          v4 = v1 < v2;
          if (v2 <= v1)
            v9 = "Operation is AND; right success rate <= left";
        }
      }
      else {
        v4 = v2 < v1;
        if (v1 <= v2)
          v9 = "Operation is OR; right success rate >= left";
      }
    }
    if (v4) {
      if (dat_49980 & 8) {
        fprintf(stderr,"Performing arm swap on:\n");
        sub_ea9f(stderr,a1,0);
      }
      sub_c856(a1);
      return 1;
    }
  }
  if (!(dat_49980 & 8))
    return 0;
  fprintf(stderr,"Not an arm swap candidate (%s):\n",v9);
  sub_ea9f(stderr,a1,0);
  return 0;
}


// Function: sub_cb9f @ 0xcb9f
char sub_cb9f(long a0) // return-dupe
{
  unsigned long v1;
  bool v2;
  
  if (a0) {
    do {
      v2 = 0;
      if (((sub_c8a7(a0)) || (v1 = *(unsigned long *)(a0 + 0x110), sub_cb9f(v1))) || (v1 = *(unsigned long *)(a0 + 0x118), sub_cb9f(v1)))
        v2 = 1;
    } while (v2);
    return 0;
  }
  return 0;
}


// Function: sub_cc0f @ 0xcc0f
unsigned long sub_cc0f(long *a0) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_cc0f
{
  unsigned int v1;
  unsigned long v10; // stack - 0xe8
  unsigned long v11; // stack - 0xf8
  unsigned char v12; // stack - 0x12a
  int v13; // stack - 0x128
  unsigned int v14; // stack - 0x124
  int v15; // stack - 0x120
  long v16; // stack - 0x118
  long *v17; // stack - 0x110
  unsigned long v18; // stack - 0xf0
  unsigned long v19; // stack - 0xe0
  unsigned int v2;
  void *v3;
  unsigned long v4;
  long v5;
  long v6;
  bool v7; // al
  unsigned long v8; // rax
  char v9 [184];
  
  v11 = 0;
  v18 = 0;
  v10 = 0;
  v19 = 0;
  v12 = 0;
  if ((!a0) || (!*a0))
    return 0;
  for (v13 = 0; v13 <= 10; v13 = v13 + 1) {
    sub_c34d(&v9[(long)v13 * 0x10]);
  }
  v14 = 3;
  v17 = a0;
  for (v16 = *a0; *(long *)(v16 + 0x110); v16 = *(long *)(v16 + 0x110)) {
    v17 = (long *)(v16 + 0x110);
    v14 = *(unsigned int *)(v16 + 0x14);
  }
  if (*(int *)(v16 + 0x10) != 3)
    sub_d2d0(v16,v14,v17);
  if (dat_49980 & 9) {
    fprintf(stderr,"Normalized Eval Tree:\n");
    sub_ea9f(stderr,*a0,0);
  }
  v15 = 0;
  v17 = a0;
  if ((*a0) && (*(int *)(*a0 + 0x10) == 3))
    v15 = *(int *)(*a0 + 0x14);
label_d216:
  do {
    v16 = *v17;
    if (!v16) {
      sub_c6e0(v9,0xb,&v10,&v11,v17);
      return (unsigned long)v12;
    }
    if ((*(int *)(v16 + 0x10) == 3) && (v15 != *(int *)(v16 + 0x14)))
      v16 = sub_d2d0(v16,v15,v17);
    v1 = *(unsigned int *)(*(long *)(v16 + 0x118) + 0x10);
    v3 = (void *)**(long **)(v16 + 0x118);
    if (v1 == 3) {
      v5 = *(long *)(v16 + 0x118);
      *(char *)(v5 + 0x18) = sub_cc0f(v16 + 0x118);
      goto label_d1ab;
    }
    if (4 <= v1) {
      error(1,0,gettext("oops -- invalid expression type!")); // return-dupe
      return v8;
    }
    if (2 <= v1) {
      if (v1 != 2) {
        error(1,0,gettext("oops -- invalid expression type!"));
        return v8;
      }
      v5 = *(long *)(v16 + 0x118);
      v6 = *(long *)(v16 + 0x118);
      *(char *)(v6 + 0x18) = sub_cc0f(v5 + 0x118);
label_d1ab:
      if (*(char *)(*(long *)(v16 + 0x118) + 0x18)) {
        sub_c6e0(v9,0xb,&v10,&v11,v17);
        v12 = 1;
      }
      v17 = (long *)(v16 + 0x110);
      goto label_d216;
    }
    if ((v15 == 1) || (*(char *)(*(long *)(v16 + 0x118) + 0x18) == '\x01')) goto label_d1ab;
    v4 = *(unsigned long *)(v16 + 0x118);
    if (sub_c209(v4)) { // branch-flip
      if (dat_49980 & 8) {
        fprintf(stderr,"-O%d: promoting cheap predicate ",(unsigned long)dat_499a8);
        sub_c2dc(stderr,*(unsigned long *)(v16 + 0x118));
        fprintf(stderr," into name_list\n");
      }
      sub_c377(&v10,v16,v17);
    }
    else if (v3 != sub_a2da) { // branch-flip
      v7 = (((2 <= dat_499a8) && (((*(int *)(*(long *)(v16 + 0x118) + 0x20) == 2 || (*(int *)(*(long *)(v16 + 0x118) + 0x20) == 1)) && (*(char *)(*(long *)(v16 + 0x118) + 0x1a) != '\x01')))) || (3 <= dat_499a8));
      if (!v7) goto label_d1ab;
      if (dat_49980 & 8) {
        fprintf(stderr,"-O%d: categorising predicate ",(unsigned long)dat_499a8);
        sub_c2dc(stderr,*(unsigned long *)(v16 + 0x118));
        v2 = *(unsigned int *)(*(long *)(v16 + 0x118) + 0x20);
        fprintf(stderr," by cost (%s)\n",sub_e95d(v2));
      }
      sub_c377(&v9[(unsigned long)*(unsigned int *)(*(long *)(v16 + 0x118) + 0x20) * 0x10],v16,v17);
    }
    else {
      sub_c377(&v11,v16,v17);
    }
  } while( true );
}


// Function: sub_d290 @ 0xd290
void sub_d290(void)
{
  return;
}


// Function: sub_d2d0 @ 0xd2d0
unsigned long * sub_d2d0(unsigned long a0,unsigned int a1,unsigned long *a2)
{
  unsigned long *v1; // rax
  unsigned int v2; // xmm0_da
  unsigned int v3; // xmm0_da
  
  v1 = (unsigned long *)sub_3205a(0x138);
  *(unsigned int *)&v1[2] = 3;
  *(unsigned int *)((long)v1 + 0x14) = a1;
  *(unsigned int *)&v1[4] = 0;
  if (a1 != 3) { // branch-flip
    if (4 <= a1) goto label_d3dd;
    if (a1 != 1) { // branch-flip
      if (a1 == 2) {
        *v1 = sub_9f9e;
        v1[1] = "-o";
        sub_d290();
        *(unsigned int *)((long)v1 + 0x24) = v2;
      }
    }
    else {
      *v1 = sub_8fd0;
      v1[1] = ",";
      *(unsigned int *)((long)v1 + 0x24) = dat_39684;
    }
  }
  else {
    *v1 = sub_8d9c;
    v1[1] = "-a";
    sub_d290();
    *(unsigned int *)((long)v1 + 0x24) = v3;
  }
label_d3dd:
  v1[0x23] = a0;
  *a2 = v1;
  return v1;
}


// Function: sub_d3fd @ 0xd3fd
void sub_d3fd(unsigned long a0,long a1,unsigned long *a2)
{
  *(unsigned long *)(a1 + 0x110) = *a2;
  *a2 = a0;
}


// Function: sub_d431 @ 0xd431
unsigned long sub_d431(long a0,unsigned long a1,long a2,void *a3)
{
  unsigned long v1; // stack - 0x18
  
  v1 = 1;
  while( true ) {
    if (a1 <= v1)
      return 1;
    if ((int)(*a3)(v1 * a2 + a0,(v1 - 1) * a2 + a0) <= -1) break;
    if (1 <= (int)(*a3)((v1 - 1) * a2 + a0,v1 * a2 + a0))
      __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted"); // no-return
    v1 += 1;
  }
  return 0;
}


// Function: sub_d52e @ 0xd52e
void sub_d52e(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x20
  
  v2 = *a0;
  v1 = *a1;
  memcmp(&v2,&v1,8);
}


// Function: sub_d5a5 @ 0xd5a5
char * sub_d5a5(long *a0) // ternary x2
{
  long v1;
  char *v2; // rax
  long v3 [3]; // stack - 0x38
  unsigned int v4; // stack - 0x48
  unsigned int v5; // stack - 0x44
  
  if (*(char *)((long)a0 + 0x1a)) // branch-flip
    v4 = 3;
  else if (*(char *)((long)a0 + 0x1c)) // branch-flip
    v4 = 1;
  else {
    v4 = (*(char *)((long)a0 + 0x1b)) ? 2 : 0; // branch-flip
  }
  if (((void *)*a0 != sub_9466) && ((void *)*a0 != sub_9497)) {
    if ((void *)*a0 != sub_1376d) { // branch-flip
      if (!dat_49a30) {
        qsort(0x49020,0x3b,0x10,sub_d52e);
        if (sub_d431(0x49020,0x3b,0x10,sub_d52e) != '\x01') {
          error(1,0,"failed to sort the costlookup array");
          return v2;
        }
        dat_49a30 = 1;
      }
      v3[0] = *a0;
      v2 = (long)bsearch(v3,0x49020,0x3b,0x10,sub_d52e);
      if (v2) // branch-flip
        v5 = *(unsigned int *)((long)v2 + 8);
      else {
        v1 = a0[1];
        v2 = gettext("warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug");
        error(0,0,v2,v1);
        v5 = 10;
      }
    }
    else {
      v5 = *(unsigned int *)&a0[4];
    }
  }
  else {
    v5 = ((char)a0[7]) ? 7 : 8; // branch-flip
  }
  if (v4 >= v5)
    v5 = v4;
  v2 = (unsigned long)(unsigned long)v5;
  return v2;
}


// Function: sub_d7ac @ 0xd7ac
void sub_d7ac(long a0) // return-dupe
{
  if (!a0)
    return;
  sub_d7ac(*(unsigned long *)(a0 + 0x118));
  sub_d7ac(*(unsigned long *)(a0 + 0x110));
  *(unsigned int *)(a0 + 0x20) = sub_d5a5(a0);
}


// Function: sub_d7ff @ 0xd7ff
unsigned long sub_d7ff(void)
{
  return dat_49a10;
}


// Function: sub_d810 @ 0xd810
unsigned int sub_d810(long a0)
{
  unsigned int v1;
  
  if (a0) // branch-flip
    v1 = *(unsigned int *)(a0 + 0x24);
  else {
    v1 = dat_39684;
  }
  return v1;
}


// Function: sub_d838 @ 0xd838
void sub_d838(long *a0) // return-dupe x4
{
  unsigned int v1; // xmm0_da
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  if (a0[0x23])
    sub_d838(a0[0x23]);
  if (a0[0x22])
    sub_d838(a0[0x22]);
  if ((int)a0[2] == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if ((int)a0[2] == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  switch((int)a0[2]) {
    case 0:
      goto label_d958;
    case 1:
      if (a0[0x23])
        __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates"); // no-return
      if (!a0[0x22])
        return;
      __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates"); // no-return
    case 2:
      if ((void *)*a0 != sub_9b30)
        __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
      if (!a0[0x22]) {
        *(float *)((long)a0 + 0x24) = dat_39684 - *(float *)(a0[0x23] + 0x24);
        return;
      }
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
    case 3:
      if ((void *)*a0 != sub_8d9c) { // branch-flip
        if ((void *)*a0 != sub_8fd0) {
          if ((void *)*a0 != sub_9f9e)
            __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
          sub_d810(a0[0x23]);
          sub_d810(a0[0x22]);
        }
      }
      else {
        sub_d810(a0[0x23]);
        sub_d810(a0[0x22]);
      }
      sub_d290();
      *(unsigned int *)((long)a0 + 0x24) = v1;
      return;
    case 4:
    case 5:
      *(float *)((long)a0 + 0x24) = dat_39684;
      return;
    default:
      __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
    
  }
label_d958:
  if (a0[0x23])
    __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates"); // no-return
  if (!a0[0x22])
    return;
  __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates"); // no-return
}


// Function: sub_dc43 @ 0xdc43
void sub_dc43(long a0,char a1) // return-dupe
{
  if ((a1) && (*(int *)(a0 + 0x10) != 3))
    __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
  if (*(long *)(a0 + 0x110)) {
    if (*(int *)(*(long *)(a0 + 0x110) + 0x10) != 3)
      __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
    sub_dc43(*(unsigned long *)(a0 + 0x110),0);
  }
  if (!*(long *)(a0 + 0x118))
    return;
  sub_dc43(*(unsigned long *)(a0 + 0x118),0);
}


// Function: sub_dd20 @ 0xdd20
long * sub_dd20(int a0,long a1,int a2)
{
  unsigned long v1;
  long *v10; // stack - 0x50
  int v11; // stack - 0x54
  long v12; // stack - 0x48
  long v13; // stack - 0x40
  long v14; // stack - 0x38
  long v15; // stack - 0x30
  int *v16; // stack - 0x28
  long v2;
  long *v3;
  int *v4;
  char v5; // al
  int v6; // eax
  long *v7; // rax
  int v8; // stack - 0x5c
  int v9; // stack - 0x58
  
  dat_49a08 = NULL;
  v7 = (long)(long)a2;
  v7 = (long)(a1 + (long)v7 * 8);
  dat_49a20 = (long)v7;
  v8 = a0;
  v9 = a2;
  while ((v9 < v8 && (v7 = (long)(long)v9, v7 = (unsigned long *)(a1 + (long)v7 * 8), v5 = sub_ff96(*v7,1), v5 != '\x01'))) {
    v7 = (long)(dat_49a28 + 1);
    dat_49a28 = (long)v7;
    v6 = v9 + 1;
    v9 = v6;
  }
  v7 = (long)sub_1437a("(");
  v12 = (long)v7;
  v7 = (long)sub_1437a(")");
  v13 = (long)v7;
  v7 = (long)sub_1437a("print");
  v14 = (long)v7;
  if (!v12)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v13)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v7)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_168b2(v12,a1,&v8);
  dat_49a18[1] = (long)"(";
  *(char *)((long)dat_49a08 + 0x29) = 1;
  sub_14262(a1,v8,dat_49a18,dat_49a08);
  sub_aec7(dat_49a18);
  while( true ) {
    if (v8 <= v9) {
      sub_14287(a1,v8,dat_49a18,dat_49a08);
      v3 = dat_49a08;
      if (dat_49a08[0x21]) { // branch-flip
        v5 = sub_ff58(dat_49a08[0x21]);
        if (v5 != '\x01') { // branch-flip
          v10 = dat_49a08;
          dat_49a08 = (long *)dat_49a08[0x21];
          sub_aec7(dat_49a08);
          free(v10);
        }
        else {
          sub_14833(v13,a1,&v8);
          dat_49a18[1] = (long)")";
          *(char *)((long)dat_49a18 + 0x29) = 1;
          sub_aec7(dat_49a18);
          sub_16eb3(v14,a1,&v8);
          dat_49a18[1] = (long)"-print";
          *(char *)((long)dat_49a18 + 0x29) = 1;
          sub_aec7(dat_49a18);
          sub_aec7(dat_49a08);
        }
      }
      else {
        v10 = dat_49a08;
        dat_49a08 = (long *)dat_49a08[0x21];
        dat_49a18 = dat_49a08;
        free(v3);
        sub_16eb3(v14,a1,&v8);
        dat_49a18[1] = (long)"-print";
        sub_aec7(dat_49a18);
        sub_aec7(dat_49a08);
      }
      if (dat_49980 & 9) {
        fprintf(stderr,"Predicate List:\n");
        sub_aa62(stderr,dat_49a08);
      }
      sub_13f89(dat_49a08);
      sub_aec7(dat_49a08);
      v10 = dat_49a08;
      v7 = (unsigned long)sub_bc44(&v10,0,0);
      dat_49a10 = v7;
      sub_d838(v7);
      if (!v10) {
        if (dat_49980 & 9) {
          fprintf(stderr,"Eval Tree:\n");
          sub_ea9f(stderr,dat_49a10,0);
        }
        sub_d7ac(dat_49a10);
        sub_cc0f(0x49a10);
        sub_dc43(dat_49a10,1);
        sub_cb9f(dat_49a10);
        sub_dc43(dat_49a10,1);
        if (dat_49980 & 9) {
          fprintf(stderr,"Optimized Eval Tree:\n");
          sub_ea9f(stderr,dat_49a10,0);
          fprintf(stderr,"Optimized command line:\n");
          sub_ac38(stderr,dat_49a10);
          fprintf(stderr,"\n");
        }
        v7 = (long *)dat_49a10;
        return v7;
      }
      if ((void *)*v10 == sub_8ee1) {
        v7 = (char *)gettext("you have too many \')\'");
        error(1,0,(char *)v7);
        return v7;
      }
      if (!v10[1]) {
        v7 = (char *)gettext("unexpected extra predicate");
        error(1,0,(char *)v7);
        return v7;
      }
      v2 = v10[1];
      v7 = (char *)gettext("unexpected extra predicate \'%s\'");
      error(1,0,(char *)v7,v2);
      return v7;
    }
    dat_499f8 = 0;
    v7 = (long)(long)v9;
    v7 = (unsigned long *)(a1 + (long)v7 * 8);
    v5 = sub_ff96(*v7,0);
    if (v5 != '\x01') {
      v7 = (long)(long)v9;
      v7 = (unsigned long *)(a1 + (long)v7 * 8);
      v1 = *v7;
      v7 = (char *)gettext("paths must precede expression: `%s\'");
      error(0,0,(char *)v7,v1);
      v7 = (long)(long)v9;
      v7 = (unsigned long *)(a1 + (long)v7 * 8);
      v6 = access((char *)*v7,0);
      if (!v6) {
        v2 = dat_49a18[1];
        v7 = (char *)gettext("possible unquoted pattern after predicate `%s\'?");
        error(0,0,(char *)v7,v2);
      }
      exit(1); // no-return
    }
    v7 = (long)(long)v9;
    v7 = (long *)(a1 + (long)v7 * 8);
    v15 = *v7;
    v7 = (int *)sub_1437a(v15);
    v4 = (int *)v7;
    v16 = (int *)v7;
    if (!v7) {
      v7 = (char *)gettext("unknown predicate `%s\'");
      error(1,0,(char *)v7,v15);
      return v7;
    }
    if (*(int *)v7 != 4) {
      v6 = v9 + 1;
      v9 = v6;
    }
    v11 = v9;
    v5 = (**(void **)&v4[4])(v4,a1,&v9);
    if (v5 != '\x01') break;
    dat_49a18[1] = v15;
    if (v11 != v9) { // branch-flip
      v7 = (long)(long)v11;
      dat_49a18[6] = *(long *)((long)v7 * 8 + a1);
    }
    else {
      dat_49a18[6] = 0;
    }
    sub_aec7(dat_49a18);
    sub_aec7(dat_49a08);
  }
  v7 = (long)(long)v9;
  v7 = (long *)(a1 + (long)v7 * 8);
  if (!*v7) {
    v7 = (char *)gettext("missing argument to `%s\'");
    error(1,0,(char *)v7,v15);
    return v7;
  }
  if ((*v16 == 4) && (v11 == v9)) {
    v7 = (char *)gettext("invalid predicate `%s\'");
    error(1,0,(char *)v7,v15);
    return v7;
  }
  v7 = (long)(long)v9;
  v7 = (unsigned long *)(a1 + (long)v7 * 8);
  v1 = *v7;
  v7 = (char *)gettext("invalid argument `%s\' to `%s\'");
  error(1,0,(char *)v7,v1,v15);
  return v7;
}


// Function: sub_e60c @ 0xe60c
void sub_e60c(long a0)
{
  *(unsigned long *)(a0 + 0x128) = 0;
  *(unsigned long *)(a0 + 0x120) = *(unsigned long *)(a0 + 0x128);
}


// Function: sub_e644 @ 0xe644
long sub_e644(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_e67d(a0);
  if (v1)
    *(unsigned long *)(v1 + 0x30) = 0;
  return v1;
}


// Function: sub_e67d @ 0xe67d
long sub_e67d(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 == 2)
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
  v1 = sub_3205a(0x138);
  if (dat_49a08) // branch-flip
    *(long *)(dat_49a18 + 0x108) = v1;
  else {
    dat_49a08 = v1;
  }
  *(int **)(v1 + 0x130) = a0;
  *(unsigned int *)(v1 + 0x10) = 0;
  *(unsigned int *)(v1 + 0x14) = 0;
  *(char *)(v1 + 0x1a) = 1;
  *(char *)(v1 + 0x1b) = 1;
  *(unsigned int *)(v1 + 0x20) = 10;
  *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
  *(char *)(v1 + 0x28) = dat_4994f;
  *(unsigned int *)(v1 + 0x24) = dat_39684;
  dat_49a18 = v1;
  sub_e60c(v1);
  return dat_49a18;
}


// Function: sub_e7dd @ 0xe7dd
long sub_e7dd(unsigned long a0,unsigned long a1)
{
  unsigned int v1;
  long v2; // rax
  unsigned long *v3; // rax
  long v4; // rax
  
  if (!dat_49a40)
    dat_49a40 = sub_1437a("and");
  if (!dat_49a40)
    __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  if (!dat_49a18) goto label_e92c;
  v1 = *(unsigned int *)(dat_49a18 + 0x10);
  if (v1 != 5) {
    if (5 < v1) goto label_e92c;
    if (!v1) {
      error(1,0,gettext("oops -- invalid default insertion of and!"));
      return v2;
    }
    if (v1 != 1) goto label_e92c;
  }
  v3 = (unsigned long *)sub_e644(dat_49a40);
  *v3 = sub_8d9c;
  v3[1] = "-a";
  *(unsigned int *)&v3[2] = 3;
  *(unsigned int *)((long)v3 + 0x14) = 3;
  *(char *)((long)v3 + 0x1a) = 0;
  *(char *)((long)v3 + 0x1b) = 0;
  *(char *)((long)v3 + 0x1c) = 0;
  v3[6] = 0;
  v3[7] = 0;
  *(char *)&v3[3] = 0;
  *(char *)((long)v3 + 0x19) = 0;
label_e92c:
  v4 = sub_e67d(a0);
  *(unsigned long *)(v4 + 0x30) = a1;
  *(unsigned long *)(v4 + 0x130) = a0;
  return v4;
}


// Function: sub_e95d @ 0xe95d
char * sub_e95d(int a0)
{
  unsigned int v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (0xb <= v1)
      return "unknown";
    if (a0 == *(int *)((unsigned long)v1 * 0x10 + 300000)) break;
    v1 += 1;
  }
  return *(char **)((unsigned long)v1 * 0x10 + 0x493e8);
}


// Function: sub_e9bd @ 0xe9bd
unsigned long sub_e9bd(short a0)
{
  int v1; // stack - 0xc
  
  v1 = 0;
  while ((*(short *)((long)v1 * 0x10 + 0x49520) != -1 && (a0 != *(short *)((long)v1 * 0x10 + 0x49520)))) {
    v1 += 1;
  }
  return *(unsigned long *)((long)v1 * 0x10 + 0x49528);
}


// Function: sub_ea2e @ 0xea2e
unsigned long sub_ea2e(short a0)
{
  int v1; // stack - 0xc
  
  v1 = 0;
  while ((*(short *)((long)v1 * 0x10 + 0x494a0) != -1 && (a0 != *(short *)((long)v1 * 0x10 + 0x494a0)))) {
    v1 += 1;
  }
  return *(unsigned long *)((long)v1 * 0x10 + 0x494a8);
}


// Function: sub_ea9f @ 0xea9f
void sub_ea9f(void *a0,long a1,int a2) // return-dupe x3, ternary x3
{
  unsigned int v1;
  unsigned long v2; // rax
  char *v3; // rax
  bool v4;
  int v5; // stack - 0x20
  
  if (!a1)
    return;
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  fprintf(a0,"pred=[");
  sub_c2dc(a0,a1);
  v2 = sub_ea2e((int)(short)*(unsigned int *)(a1 + 0x14));
  v1 = *(unsigned int *)(a1 + 0x10);
  fprintf(a0,"] type=%s prec=%s",sub_e9bd((int)(short)v1),v2);
  v3 = (*(char *)(a1 + 0x18)) ? "" : "no "; // branch-flip
  v1 = *(unsigned int *)(a1 + 0x20);
  fprintf(a0," cost=%s est_success_rate=%#.4g %sside effects ",sub_e95d(v1),v3);
  if (((*(char *)(a1 + 0x1a)) || (*(char *)(a1 + 0x1b))) || (*(char *)(a1 + 0x1c))) {
    fprintf(a0,"Needs ");
    v4 = *(char *)(a1 + 0x1a) != '\0';
    if (v4)
      fprintf(a0,"stat");
    if (*(char *)(a1 + 0x1c)) {
      v3 = (v4) ? "," : "";
      fprintf(a0,"%sinode",v3);
      v4 = 1;
    }
    if (*(char *)(a1 + 0x1b)) {
      v3 = (v4) ? "," : "";
      fprintf(a0,"%stype",v3);
    }
  }
  fprintf(a0,"\n");
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  if ((!*(long *)(a1 + 0x110)) && (!*(long *)(a1 + 0x118))) {
    fprintf(a0,"no children.\n");
    return;
  }
  if (*(long *)(a1 + 0x110)) { // branch-flip
    fprintf(a0,"left:\n");
    sub_ea9f(a0,*(unsigned long *)(a1 + 0x110),a2 + 1);
  }
  else {
    fprintf(a0,"no left.\n");
  }
  for (v5 = 0; v5 < a2; v5 = v5 + 1) {
    fprintf(a0,"    ");
  }
  if (*(long *)(a1 + 0x118)) {
    fprintf(a0,"right:\n");
    sub_ea9f(a0,*(unsigned long *)(a1 + 0x118),a2 + 1);
    return;
  }
  fprintf(a0,"no right.\n");
}


// Function: sub_ee5a @ 0xee5a
unsigned long * sub_ee5a(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e7dd(a0,a2);
  *v1 = a1;
  v1[1] = *(unsigned long *)(a0 + 8);
  v1[7] = 0;
  *(unsigned int *)&v1[2] = 1;
  *(unsigned int *)((long)v1 + 0x14) = 0;
  return v1;
}


// Function: sub_eecc @ 0xeecc
void sub_eecc(long a0,unsigned long a1)
{
  if (!*(long *)(a0 + 0x18))
    __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary"); // no-return
  sub_ee5a(a0,*(unsigned long *)(a0 + 0x18),a1);
}


// Function: sub_ef32 @ 0xef32
void sub_ef32(unsigned long a0)
{
  sub_eecc(a0,0);
}


// Function: sub_ef55 @ 0xef55
void sub_ef55(int a0) // return-dupe, ternary
{
  void *v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x20
  
  v1 = stdout;
  fputs(gettext("Valid arguments for -D:\n"),v1);
  if (a0) {
    for (v3 = 0; v3 <= 8; v3 = v3 + 1) {
      fprintf(stdout,"%-10s %s\n",*(unsigned long *)(v3 * 0x18 + 0x495a0),*(unsigned long *)(v3 * 0x18 + 0x495b0));
    }
    return;
  }
  for (v3 = 0; v3 <= 8; v3 = v3 + 1) {
    v2 = (v3) ? ", " : ""; // branch-flip
    fprintf(stdout,"%s%s",v2,*(unsigned long *)(v3 * 0x18 + 0x495a0));
  }
}


// Function: sub_f071 @ 0xf071
void sub_f071(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_4a230;
  if (a0) {
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
    exit(a0); // no-return
  }
  fprintf(stdout,gettext("Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n"),v2);
  v1 = stdout;
  fputs(gettext("\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n"),v1);
  v1 = stdout;
  fputs(gettext("\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n"),v1);
  v1 = stdout;
  fputs(gettext("\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n"),v1);
  v1 = stdout;
  fputs(gettext("\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n"),v1);
  v1 = stdout;
  fputs(gettext("\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n"),v1);
  v1 = stdout;
  fputs(gettext("\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n"),v1);
  v1 = stdout;
  fputs(gettext("\nOther common options:\n"),v1);
  v1 = stdout;
  fputs(gettext("      --help                   display this help and exit\n"),v1);
  v1 = stdout;
  fputs(gettext("      --version                output version information and exit\n\n"),v1);
  sub_ef55(0);
  v1 = stdout;
  fputs(gettext("\nUse \'-D help\' for a description of the options, or see find(1)\n\n"),v1);
  sub_1d008(stdout,dat_4a230);
  exit(0); // no-return
}


// Function: sub_f26e @ 0xf26e
void sub_f26e(void)
{
  return;
}


// Function: sub_f27d @ 0xf27d
unsigned long sub_f27d(unsigned long a0,unsigned long a1,long a2) // return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_499c4 != '\x01') {
    sub_f26e(a2);
    v1 = (*dat_49990)(a1,a2);
    if (v1) {
      if ((dat_4994e == '\x01') && (*__errno_location() == 2))
        return 0xffffffff;
      sub_10a13(*__errno_location(),a0);
      return 0xffffffff;
    }
    if (!*(int *)(a2 + 0x18)) {
      v2 = sub_2f861(0,dat_499ac,a1);
      error(0,0,gettext("WARNING: file %s appears to have mode 0000"),v2);
      dat_499e4 = 1;
    }
  }
  dat_499c8 = *(unsigned int *)(a2 + 0x18);
  dat_499c5 = 1;
  dat_499c4 = 1;
  return 0;
}


// Function: sub_f388 @ 0xf388
unsigned long sub_f388(unsigned long a0,long a1,long a2) // return-dupe
{
  bool v1;
  int v2; // eax
  
  v1 = 0;
  if ((*(char *)(a2 + 0x1a)) && (dat_499c4 != '\x01'))
    v1 = 1;
  else if ((*(char *)(a2 + 0x1b)) && (dat_499c5 != '\x01'))
    v1 = 1;
  else if (*(char *)(a2 + 0x1c)) {
    if (*(long *)(a1 + 8)) { // branch-flip
      if ((dat_499c5 != '\x01') || ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x4000))
        v1 = 1;
    }
    else {
      v1 = 1;
    }
  }
  if ((v1) && (v2 = sub_f27d(a0,dat_499d0,a1), v2))
    return 0xffffffff;
  return 0;
}


// Function: sub_f45a @ 0xf45a
unsigned long sub_f45a(void) // return-dupe
{
  int v1; // eax
  char v2 [130];
  char v3 [262];
  float v4; // xmm0_da
  double v5; // xmm0_qa
  
  if (uname(v2))
    return 1;
  v5 = (double)(*dat_48fb8)(v3);
  v4 = (float)v5;
  v1 = strcmp("Linux",v2);
  if (v1) {
    v1 = strcmp("FreeBSD",v2);
    if (v1)
      return 1;
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),(float)s_3a453._1_4_ <= v4);
  }
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),dat_3a450 <= v4);
}


// Function: sub_f540 @ 0xf540
unsigned long sub_f540(long a0)
{
  sub_19f5a(a0 + 8,a0 + 0x60);
  return 0;
}


// Function: sub_f57a @ 0xf57a
void sub_f57a(long a0) // return-dupe
{
  sub_1a9dd(*(unsigned long *)(a0 + 0xc0),sub_f540,a0);
  if (*(long *)(a0 + 0xc0) == dat_49a00)
    return;
  sub_2fe52(*(unsigned long *)(a0 + 0xc0));
  free(*(void **)(a0 + 0xc0));
  *(unsigned long *)(a0 + 0xc0) = 0;
}


// Function: sub_f5fa @ 0xf5fa
void sub_f5fa(long *a0) // return-dupe
{
  if (!a0)
    return;
  if (dat_499e8) {
    sub_f5fa(a0[0x22]);
    if (((((void *)*a0 == sub_9497) || ((void *)*a0 == sub_9f27)) && ((char)a0[7])) && ((int)a0[0x1a]))
      sub_f57a(&a0[7]);
    sub_f5fa(a0[0x23]);
    return;
  }
  __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
}


// Function: sub_f6cb @ 0xf6cb
void sub_f6cb(void) // return-dupe
{
  if (!dat_499e8)
    return;
  sub_f5fa(sub_d7ff());
  dat_499e8 = 0;
}


// Function: sub_f6f5 @ 0xf6f5
void sub_f6f5(long *a0) // return-dupe
{
  if (!a0)
    return;
  sub_f6f5(a0[0x22]);
  if ((((void *)*a0 == sub_9466) && ((char)a0[7])) && ((int)a0[0x1a]))
    sub_19f5a(&a0[8],&a0[0x13]);
  sub_f6f5(a0[0x23]);
}


// Function: sub_f78b @ 0xf78b
void sub_f78b(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rbx
  unsigned long v4;
  
  dat_49a00 = (int *)sub_31bc2(0x10);
  v1 = sub_2fd99(dat_49a00);
  if (v1) {
    if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
      v4 = 0x398cd;
    else {
      v4 = *(unsigned long *)&dat_49a00[2];
    }
    if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
      v3 = "";
    else {
      v3 = ": ";
    }
    v2 = gettext("Failed to save initial working directory%s%s");
    error(1,*__errno_location(),v2,v3,v4);
    return;
  }
}


// Function: sub_f861 @ 0xf861
void sub_f861(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rbx
  unsigned long v4;
  
  v1 = sub_2fe17(dat_49a00);
  if (!v1) {
    sub_2fe52(dat_49a00);
    free(dat_49a00);
    dat_49a00 = NULL;
    return;
  }
  if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
    v4 = 0x398cd;
  else {
    v4 = *(unsigned long *)&dat_49a00[2];
  }
  if ((0 <= *dat_49a00) || (!*(long *)&dat_49a00[2]))
    v3 = "";
  else {
    v3 = ": ";
  }
  v2 = gettext("Failed to restore initial working directory%s%s");
  error(0,*__errno_location(),v2,v3,v4);
  _exit(1); // no-return
}


// Function: sub_f959 @ 0xf959
void sub_f959(long a0,void *a1) // return-dupe
{
  if (*(long *)(a0 + 0x110))
    sub_f959(*(unsigned long *)(a0 + 0x110),a1);
  (*a1)(a0);
  if (!*(long *)(a0 + 0x118))
    return;
  sub_f959(*(unsigned long *)(a0 + 0x118),a1);
}


// Function: sub_f9d1 @ 0xf9d1
void sub_f9d1(long *a0) // return-dupe
{
  if (((((void *)*a0 != sub_954f) && ((void *)*a0 != sub_1376d)) && ((void *)*a0 != sub_94e6)) && ((void *)*a0 != sub_959f))
    return;
  a0[8] = 0;
}


// Function: sub_fa38 @ 0xfa38
void sub_fa38(void) // return-dupe
{
  int v1; // eax
  long v2; // rax
  
  v2 = sub_d7ff();
  if (v2) {
    sub_f959(v2,sub_f6f5);
    sub_f6cb();
  }
  sub_10cac(dat_499f0);
  if (v2)
    sub_f959(v2,sub_f9d1);
  sub_f861();
  if (sub_1b5f4()) {
    sub_1b61e();
    sub_1b5c4();
  }
  v1 = sub_1e1a8(stdout);
  if (v1 != -1)
    return;
  sub_10a77(*__errno_location(),"standard output");
}


// Function: sub_fae2 @ 0xfae2
int * sub_fae2(char *a0,void *a1,unsigned int a2)
{
  int v1; // eax
  int *v2; // rax
  
  v2 = __errno_location();
  if ((*v2 != 2) && (*v2 != 0x14))
    v2 = (unsigned long)(unsigned long)a2;
  else {
    if (dat_49980 & 2)
      fprintf(stderr,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
    v1 = fstatat(dat_499d8,a0,a1,0x100);
  }
  return v2;
}


// Function: sub_fb5c @ 0xfb5c
unsigned long sub_fb5c(char *a0,void *a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_499d8 != -100) && (dat_499d8 <= -1))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  sub_f26e(a1);
  if (dat_499c0) // branch-flip
    v1 = fstatat(dat_499d8,a0,a1,0x100);
  else {
    v1 = fstatat(dat_499d8,a0,a1,0);
    if (!v1) {
      v2 = 0;
      return v2;
    }
    v2 = sub_fae2(a0,a1,v1);
  }
  return v2;
}


// Function: sub_fc21 @ 0xfc21
unsigned long sub_fc21(char *a0,void *a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if ((dat_499d8 != -100) && (dat_499d8 <= -1))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat"); // no-return
  sub_f26e(a1);
  v1 = fstatat(dat_499d8,a0,a1,0);
  if (!v1)
    return 0;
  v2 = sub_fae2(a0,a1,v1);
  return v2;
}


// Function: sub_fcc0 @ 0xfcc0
void sub_fcc0(char *a0,void *a1)
{
  if ((dat_499d8 <= -1) && (dat_499d8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  sub_f26e(a1);
  fstatat(dat_499d8,a0,a1,0x100);
}


// Function: sub_fd39 @ 0xfd39
void sub_fd39(unsigned long a0,unsigned long a1) // return-dupe x2
{
  dat_49a48 += 1;
  fprintf(stderr,"debug_stat (%s)\n",a0);
  if (dat_49988 == 2) {
    sub_fb5c(a0,a1);
    return;
  }
  if (dat_49988 <= 2) {
    if (!dat_49988) {
      sub_fcc0(a0,a1);
      return;
    }
    if (dat_49988 == 1) {
      sub_fc21(a0,a1);
      return;
    }
  }
  __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
}


// Function: sub_fe03 @ 0xfe03
unsigned int sub_fe03(void) // early-return x2
{
  if (dat_49988 == 1)
    return 1;
  if (dat_49988 != 2)
    return 0;
  return CONCAT31((undefined3)((unsigned int)dat_499c0 >> 8),dat_499c0 == 0);
}


// Function: sub_fe38 @ 0xfe38
unsigned long sub_fe38(unsigned int *a0,unsigned long a1,unsigned long a2,long a3,char a4) // return-dupe x3
{
  if (*a0) {
    if (((*a0 & 0xf000) == 0xa000) && (sub_fe03())) {
      if (!sub_f27d(a1,a2,a3)) {
        dat_499c8 = *(unsigned int *)(a3 + 0x18);
        *a0 = dat_499c8;
        dat_499c5 = 1;
        return 1;
      }
      return 0;
    }
    dat_499c8 = *a0;
    *(unsigned int *)(a3 + 0x18) = dat_499c8;
    dat_499c5 = 1;
    return 1;
  }
  if (a4) {
    dat_499c4 = 0;
    dat_499c5 = 0;
    dat_499c8 = 0;
    return 1;
  }
  if (!sub_f27d(a1,a2,a3)) {
    *a0 = *(unsigned int *)(a3 + 0x18);
    dat_499c5 = 1;
    dat_499c8 = *a0;
    return 1;
  }
  return 0;
}


// Function: sub_ff58 @ 0xff58
unsigned long sub_ff58(long a0)
{
  long v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (!v1)
      return 1;
    if (*(char *)(v1 + 0x19)) break;
    v1 = *(long *)(v1 + 0x108);
  }
  return 0;
}


// Function: sub_ff96 @ 0xff96
bool sub_ff96(char *a0,char a1) // early-return x6
{
  switch(*a0) {
    case 0x21:
    case 0x28:
      goto label_10027;
    default:
      return 0;
    case 0x29:
    case 0x2c:
      if (!a0[1])
        return a1 == '\0';
      return 0;
    case 0x2d:
      if (!a0[1])
        return 0;
      return 1;
    
  }
label_10027:
  if (!a0[1])
    return 1;
  return 0;
}


// Function: sub_1004b @ 0x1004b
void sub_1004b(char *a0)
{
  bool v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // stack - 0x40
  char v5 [2]; // stack - 0x22
  char *v6; // stack - 0x38
  unsigned long v7; // stack - 0x30
  
  v4 = NULL;
  v5[0] = ',';
  v5[1] = '\0';
  v1 = 1;
  v6 = strtok_r(a0,v5,&v4);
  do {
    if (!v6) {
      if (v1) {
        error(0,0,gettext("Empty argument to the -D option."));
        sub_f071(1); // no-return
      }
      if (!(dat_49980 & 0x10))
        return;
      sub_ef55(1);
      exit(0); // no-return
    }
    v1 = 0;
    for (v7 = 0; v7 <= 8; v7 = v7 + 1) {
      v2 = strcmp(*(char **)(v7 * 0x18 + 0x495a0),v6);
      if (!v2) {
        dat_49980 = (long)*(int *)(v7 * 0x18 + 0x495a8) | dat_49980;
        break;
      }
    }
    if (9 <= v7) {
      v3 = sub_2f861(0,dat_499ac,a0);
      error(0,0,gettext("Ignoring unrecognised debug flag %s"),v3);
    }
    v6 = strtok_r(NULL,v5,&v4);
  } while( true );
}


// Function: sub_101fd @ 0x101fd
void sub_101fd(unsigned char *a0)
{
  int v1;
  char *v2; // rax
  unsigned char *v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  
  if (!*a0) {
    error(1,0,gettext("The -O option must be immediately followed by a decimal integer"));
    return;
  }
  if (!(*(unsigned short *)((unsigned long)*a0 * 2 + *(long *)__ctype_b_loc()) & 0x800)) {
    error(1,0,gettext("Please specify a decimal number immediately after -O"));
    return;
  }
  v1 = *__errno_location();
  *__errno_location() = 0;
  v4 = strtoul((char *)a0,(char **)&v3,10);
  if ((!v4) && (a0 == v3)) {
    error(1,0,gettext("Please specify a decimal number immediately after -O"));
    return;
  }
  if (*v3) {
    error(1,0,gettext("Invalid optimisation level %s"),a0);
    return;
  }
  if ((v4 == 0xffffffffffffffff) && (*__errno_location())) {
    v2 = gettext("Invalid optimisation level %s");
    error(1,*__errno_location(),v2,a0);
    return;
  }
  if (v4 <= 0xffff) {
    dat_499a8 = (unsigned short)v4;
    *__errno_location() = v1;
    return;
  }
  error(1,0,gettext("Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."),v4);
}


// Function: sub_103e8 @ 0x103e8
int sub_103e8(int a0,long a1)
{
  char *v1;
  int v2; // stack - 0x10
  
  v2 = 1;
  do {
    if (a0 <= v2)
      return v2;
    v1 = *(char **)(a1 + (long)v2 * 8);
    if (strcmp("-H",v1)) { // branch-flip
      v1 = *(char **)(a1 + (long)v2 * 8);
      if (strcmp("-L",v1)) { // branch-flip
        v1 = *(char **)(a1 + (long)v2 * 8);
        if (strcmp("-P",v1)) { // branch-flip
          v1 = *(char **)(a1 + (long)v2 * 8);
          if (!strcmp("--",v1))
            return v2 + 1;
          v1 = *(char **)(a1 + (long)v2 * 8);
          if (strcmp("-D",v1)) { // branch-flip
            v1 = *(char **)(a1 + (long)v2 * 8);
            if (strncmp("-O",v1,2))
              return v2;
            sub_101fd(*(long *)(a1 + (long)v2 * 8) + 2);
          }
          else {
            if (a0 <= v2 + 1) {
              error(0,0,gettext("Missing argument after the -D option."));
              sub_f071(1); // no-return
            }
            sub_1004b(*(unsigned long *)(a1 + ((long)v2 + 1) * 8));
            v2 += 1;
          }
        }
        else {
          sub_141b0(0);
        }
      }
      else {
        sub_141b0(1);
      }
    }
    else {
      sub_141b0(2);
    }
    v2 += 1;
  } while( true );
}


// Function: sub_10601 @ 0x10601
undefined16 sub_10601(void)
{
  long v1;
  char v2 [16];
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  
  if (gettimeofday(&v3,NULL)) { // branch-flip
    v3 = time(0);
    if (v3 == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v1 = 0;
  }
  else {
    v1 = v4 * 1000;
  }
  v2._8_8_ = v1;
  v2._0_8_ = v3;
  return v2._0_16_;
}


// Function: sub_106bd @ 0x106bd
void sub_106bd(char *a0)
{
  char v1 [16];
  
  if (getenv("POSIXLY_CORRECT")) // branch-flip
    a0[0x11] = 1;
  else {
    a0[0x11] = 0;
  }
  a0[0x58] = sub_f45a();
  *(unsigned int *)&a0[0x5c] = 0;
  if (isatty(0)) { // branch-flip
    a0[0x10] = 1;
    a0[0xf] = 0;
  }
  else {
    a0[0x10] = 0;
    a0[0xf] = 0;
  }
  if (a0[0x11])
    a0[0x10] = 0;
  *a0 = 1;
  a0[1] = 0;
  *(unsigned int *)&a0[8] = 0xffffffff;
  *(unsigned int *)&a0[4] = *(unsigned int *)&a0[8];
  v1 = sub_10601();
  *(long *)&a0[0x18] = SUB168(v1,0);
  *(long *)&a0[0x20] = SUB168(v1,8);
  *(long *)&a0[0x28] = *(long *)&a0[0x18] + -0x15180;
  *(unsigned long *)&a0[0x30] = *(unsigned long *)&a0[0x20];
  a0[0x38] = 0;
  a0[0xd] = 0;
  a0[0xe] = 0;
  if (a0[0x11]) // branch-flip
    *(unsigned int *)&a0[0x3c] = 0x200;
  else {
    *(unsigned int *)&a0[0x3c] = 0x400;
  }
  *(unsigned long *)&a0[0x40] = 0;
  *(unsigned short *)&a0[0x68] = 2;
  if (getenv("FIND_BLOCK_SIZE")) {
    error(1,0,gettext("The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"));
    return;
  }
  a0[0xc] = 0;
  sub_141b0(0);
  *(unsigned int *)&a0[0x6c] = 8;
  *(unsigned long *)&a0[0x70] = 0;
  a0[0x78] = 0;
}


// Function: sub_10873 @ 0x10873
unsigned long sub_10873(unsigned long a0,unsigned long a1,unsigned long *a2) // return-dupe x2
{
  void *v1;
  
  a2[0x24] = a2[0x24] + 1;
  if ((((*(char *)((long)a2 + 0x1a)) || (*(char *)((long)a2 + 0x1b))) || (*(char *)((long)a2 + 0x1c))) && (sub_f388(a0,a1,a2)))
    return 0;
  v1 = (void *)*a2;
  if ((*v1)(a0,a1,a2)) {
    a2[0x25] = a2[0x25] + 1;
    return 1;
  }
  return 0;
}


// Function: sub_10933 @ 0x10933
unsigned int sub_10933(void *a0) // return-dupe
{
  if ((a0 != sub_9497) && (a0 != sub_9f27))
    return 0;
  return 1;
}


// Function: sub_1096a @ 0x1096a
void sub_1096a(unsigned int a0,unsigned long a1)
{
  sub_2f861(a0,dat_499ac,a1);
}


// Function: sub_10995 @ 0x10995
void sub_10995(int a0,int a1,char a2,unsigned long a3) // return-dupe
{
  if ((a2 != '\x01') || (dat_499f8 != '\x01')) {
    error(a0,a1,"%s",sub_1096a(0,a3));
    dat_499e4 = 1;
  }
  if (!a2)
    return;
  dat_499f8 = 1;
}


// Function: sub_10a13 @ 0x10a13
void sub_10a13(unsigned int a0,unsigned long a1)
{
  sub_10995(0,a0,1,a1);
}


// Function: sub_10a44 @ 0x10a44
void sub_10a44(unsigned int a0,unsigned long a1)
{
  sub_10995(1,a0,1,a1);
  abort(); // no-return
}


// Function: sub_10a77 @ 0x10a77
void sub_10a77(unsigned int a0,unsigned long a1)
{
  sub_10995(0,a0,0,a1);
}


// Function: sub_10aa8 @ 0x10aa8
void sub_10aa8(unsigned int a0,unsigned long a1)
{
  dat_499f8 = 0;
  sub_10995(1,a0,0,a1);
  abort(); // no-return
}


// Function: sub_10ae2 @ 0x10ae2
unsigned int sub_10ae2(long *a0,long *a1) // return-dupe
{
  if ((a0[1] == a1[1]) && (*a0 == *a1))
    return 1;
  return 0;
}


// Function: sub_10b3b @ 0x10b3b
void sub_10b3b(void *a0)
{
  unsigned long v1;
  
  if (*(long *)((long)a0 + 0x18)) {
    v1 = *(unsigned long *)((long)a0 + 0x18);
    if (sub_1dad5(v1)) {
      v1 = *(unsigned long *)((long)a0 + 0x10);
      sub_10aa8(*__errno_location(),v1); // no-return
    }
  }
  free(*(void **)((long)a0 + 0x10));
  free(a0);
}


// Function: sub_10bb1 @ 0x10bb1
unsigned long sub_10bb1(unsigned long *a0,unsigned long a1)
{
  return (a0[1] ^ *a0) % a1;
}


// Function: sub_10be9 @ 0x10be9
long * sub_10be9(char *a0) // return-dupe x2
{
  long *v1; // rax
  
  v1 = malloc(0x10);
  if (!v1)
    return NULL;
  *v1 = (long)strdup(a0);
  if (*v1) {
    v1[1] = sub_21bb1(0xb,0,sub_10bb1,sub_10ae2,sub_10b3b);
    if (!v1[1]) {
      free((void *)*v1);
      free(v1);
      return NULL;
    }
    return v1;
  }
  free(v1);
  return NULL;
}


// Function: sub_10cac @ 0x10cac
void sub_10cac(unsigned long *a0)
{
  free((void *)*a0);
  sub_21e31(a0[1]);
}


// Function: sub_10ce6 @ 0x10ce6
unsigned long sub_10ce6(unsigned long *a0,char *a1) // early-return x5
{
  int v1;
  unsigned long *v2; // rax
  long v3; // rax
  unsigned long v4;
  unsigned long v5; // stack - 0xa8
  unsigned long v6; // stack - 0xa0
  
  v2 = malloc(0x20);
  if (!v2)
    return 0;
  v2[2] = strdup(a1);
  if (!v2[2]) {
    free(v2);
    return 0;
  }
  v4 = *a0;
  v2[3] = sub_1e49e(a1,v4);
  if (!v2[3]) {
    sub_10b3b(v2);
    return 0;
  }
  v1 = fileno((void *)v2[3]);
  if (0 <= v1) {
    sub_1d5ba(v1,1);
    if (fstat(v1,&v5) <= -1) {
      sub_10b3b(v2);
      return 0;
    }
    *v2 = v5;
    v2[1] = v6;
    v3 = sub_214fc(a0[1],v2);
    if (v3) { // branch-flip
      sub_10b3b(v2);
      v4 = *(unsigned long *)(v3 + 0x18);
    }
    else {
      v4 = a0[1];
      if (!sub_228f2(v4,v2)) {
        v1 = *__errno_location();
        sub_10b3b(v2);
        *__errno_location() = v1;
        return 0;
      }
      v4 = v2[3];
    }
    return v4;
  }
  __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
}


// Function: sub_10f4b @ 0x10f4b
long sub_10f4b(long *a0,char *a1,int a2,int a3,char a4,char a5,long a6)
{
  long v1;
  char *v2;
  unsigned int v3; // stack - 0x24
  char *v4; // stack - 0x20
  
  v3 = 0;
  if (a4 == '{')
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment"); // no-return
  if (a4 == '[')
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment"); // no-return
  if (a4 == '(')
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment"); // no-return
  *a0 = sub_31bc2(0x20);
  *(int *)*a0 = a3;
  *(char *)(*a0 + 4) = a4;
  *(char *)(*a0 + 5) = a5;
  *(unsigned long *)(*a0 + 0x18) = 0;
  *(int *)(*a0 + 0x10) = a2;
  v1 = *a0;
  *(unsigned long *)(v1 + 8) = sub_31bc2((long)a2 + 2);
  v2 = *(char **)(v1 + 8);
  strncpy(v2,a1,(long)a2);
  v2 = &v2[a2];
  if ((a3) && (a3 != 1)) {
    if (a3 != 2)
      __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment"); // no-return
    v4 = v2;
    switch(a4) {
      case 0x25:
        v4 = &v2[1];
        *v2 = '%';
        break;
      case 0x41:
      case 0x42:
      case 0x43:
      case 0x46:
      case 0x4d:
      case 0x54:
      case 0x61:
      case 99:
      case 0x67:
      case 0x73:
      case 0x74:
      case 0x75:
        *(char *)(a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x44:
      case 0x47:
      case 0x55:
      case 0x62:
      case 0x6b:
      case 0x6e:
        *(char *)(a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x48:
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x50:
      case 0x66:
      case 0x68:
      case 0x70:
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x53:
        *(char *)(a6 + 0x1a) = 1;
        v3 = 3;
        v4 = &v2[1];
        *v2 = 'g';
        break;
      case 0x59:
        *(char *)(a6 + 0x1a) = 1;
        v3 = 2;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x5a:
        v3 = 5;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 100:
        v4 = &v2[1];
        *v2 = 'd';
        break;
      case 0x69:
        *(char *)(a6 + 0x1c) = 1;
        v3 = 1;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x6c:
        *(char *)(a6 + 0x1a) = 1;
        v3 = 4;
        v4 = &v2[1];
        *v2 = 's';
        break;
      case 0x6d:
        v4 = &v2[1];
        *v2 = 'o';
        *(char *)(a6 + 0x1a) = 1;
        v3 = 3;
        break;
      case 0x79:
        *(char *)(a6 + 0x1b) = 1;
        v3 = 2;
        v4 = &v2[1];
        *v2 = 's';
      
    }
    *v4 = '\0';
    if (*(unsigned int *)(a6 + 0x20) < v3)
      *(unsigned int *)(a6 + 0x20) = v3;
    v1 = *a0;
  }
  else {
    if (a4)
      __assert_fail("0 == format_char","print.c",0x61,"make_segment"); // no-return
    if (a5)
      __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment"); // no-return
    *v2 = '\0';
    v1 = *a0;
  }
  return v1 + 0x18;
}


// Function: sub_1133c @ 0x1133c
unsigned int sub_1133c(char a0) // return-dupe
{
  if (('0' <= a0) && (a0 <= '7'))
    return 1;
  return 0;
}


// Function: sub_11366 @ 0x11366
int sub_11366(long a0,long *a1)
{
  char v1;
  int v2;
  int v3; // r12d
  long v4; // stack - 0x20
  
  v4 = 0;
  v2 = 0;
  for (v3 = 0; v3 <= 2; v3 = v3 + 1) {
    v1 = *(char *)(v4 + a0);
    if (!sub_1133c((int)v1)) break;
    v2 = (int)*(char *)(v4 + a0) + v2 * 8 + -0x30;
    v4 += 1;
  }
  *a1 = v4 + -1;
  return v2;
}


// Function: sub_113f2 @ 0x113f2
char sub_113f2(char a0) // return-dupe x8
{
  switch(a0) {
    case 0x5c:
      return 0x5c;
    default:
      return 0;
    case 0x61:
      return 7;
    case 0x62:
      return 8;
    case 0x66:
      return 0xc;
    case 0x6e:
      return 10;
    case 0x72:
      return 0xd;
    case 0x74:
      return 9;
    case 0x76:
      return 0xb;
    
  }
}


// Function: sub_11467 @ 0x11467
long sub_11467(long a0)
{
  char v1;
  long v2; // stack - 0x10
  
  v2 = 0;
  do {
    v2 += 1;
    if (!*(char *)(v2 + a0)) break;
    v1 = *(char *)(v2 + a0);
  } while (strchr("-+ #",(int)v1));
  while (*(unsigned short *)((unsigned long)*(unsigned char *)(v2 + a0) * 2 + *(long *)__ctype_b_loc()) & 0x800) {
    v2 += 1;
  }
  if (*(char *)(v2 + a0) == '.') {
    do {
      v2 += 1;
    } while (*(unsigned short *)((unsigned long)*(unsigned char *)(v2 + a0) * 2 + *(long *)__ctype_b_loc()) & 0x800);
  }
  return v2;
}


// Function: sub_11545 @ 0x11545
unsigned long sub_11545(char a0) // return-dupe x2
{
  if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)a0))
    return 1;
  if (strchr("ABCT",(int)a0))
    return 2;
  return 0;
}


// Function: sub_1159f @ 0x1159f
unsigned long sub_1159f(unsigned long *a0,unsigned long a1,char *a2) // return-dupe
{
  unsigned long v1;
  long v10; // stack - 0x38
  long v11; // stack - 0x30
  long v12; // stack - 0x28
  char v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rdx
  long v6; // stack - 0x58
  char *v7; // stack - 0x50
  char *v8; // stack - 0x48
  unsigned long *v9; // stack - 0x40
  
  v7 = a2;
  v12 = sub_ee5a(a1,sub_1376d,a2);
  *(char *)(v12 + 0x19) = 1;
  *(char *)(v12 + 0x18) = *(char *)(v12 + 0x19);
  v1 = a0[1];
  *(unsigned long *)(v12 + 0x38) = *a0;
  *(unsigned long *)(v12 + 0x40) = v1;
  v1 = a0[3];
  *(unsigned long *)(v12 + 0x48) = a0[2];
  *(unsigned long *)(v12 + 0x50) = v1;
  *(unsigned long *)(v12 + 0x58) = a0[4];
  *(char *)(v12 + 0x1b) = 0;
  *(char *)(v12 + 0x1a) = 0;
  *(unsigned int *)(v12 + 0x20) = 0;
  v9 = (unsigned long *)(v12 + 0x38);
  *v9 = 0;
  v8 = v7;
  do {
    if (!*v8) {
      if (v8 <= v7)
        return 1;
      sub_10f4b(v9,v7,(int)v8 - (int)v7,0,0,0,v12);
      return 1;
    }
    if ((*v8 == '\\') && (v8[1] == 'c')) {
      sub_10f4b(v9,v7,(int)v8 - (int)v7,1,0,0,v12);
      if (!*(char *)(v12 + 0x1a))
        return 1;
      if (3 <= *(unsigned int *)(v12 + 0x20))
        return 1;
      *(unsigned int *)(v12 + 0x20) = 3;
      return 1;
    }
    if (*v8 != '\\') { // branch-flip
      if (*v8 == '%') {
        if (!v8[1]) {
          error(1,0,gettext("error: %s at end of format string"),v8);
          return v3;
        }
        if (v8[1] != '%') // branch-flip
          v11 = sub_11467(v8);
        else {
          v11 = 1;
        }
        v8 = &v8[v11];
        v11 = sub_11545((int)*v8);
        if ((v11) && (v8[v11 + -1])) {
          if (v11 != 2) // branch-flip
            v2 = '\0';
          else {
            v2 = v8[1];
          }
          v9 = (unsigned long *)sub_10f4b(v9,v7,(int)v8 - (int)v7,2,(int)*v8,(int)v2,v12);
          v8 = &v8[v11 + -1];
        }
        else {
          v2 = *v8;
          if (strchr("{[(",(int)v2)) {
            v2 = *v8;
            error(1,0,gettext("error: the format directive `%%%c\' is reserved for future use"),(unsigned long)(unsigned int)(int)v2);
            return v4;
          }
          if ((v11 != 2) || (v8[1])) {
            v2 = *v8;
            error(0,0,gettext("warning: unrecognized format directive `%%%c\'"),(unsigned long)(unsigned int)(int)v2);
          }
          else {
            v2 = *v8;
            error(0,0,gettext("warning: format directive `%%%c\' should be followed by another character"),(unsigned long)(unsigned int)(int)v2);
          }
          v9 = (unsigned long *)sub_10f4b(v9,v7,((int)v8 + 1) - (int)v7,0,0,0,v12);
        }
        v7 = &v8[1];
      }
    }
    else {
      v10 = 1;
      if (v8[1]) { // branch-flip
        v2 = v8[1];
        if (sub_1133c((int)v2)) { // branch-flip
          v6 = 0;
          v5 = &v8[v10];
          *v8 = sub_11366(v5,&v6);
          v10 += v6;
        }
        else {
          v2 = sub_113f2((int)v8[v10]);
          if (!v2) {
            v2 = v8[v10];
            error(0,0,gettext("warning: unrecognized escape `\\%c\'"),(unsigned long)(unsigned int)(int)v2);
            v8 = &v8[v10];
            goto label_11aa1;
          }
          *v8 = v2;
        }
      }
      else {
        error(0,0,gettext("warning: escape `\\\' followed by nothing at all"));
        v10 -= 1;
      }
      v9 = (unsigned long *)sub_10f4b(v9,v7,((int)v8 - (int)v7) + 1,0,0,0,v12);
      v7 = &v8[v10 + 1];
      v8 = &v8[v10];
    }
label_11aa1:
    v8 = &v8[1];
  } while( true );
}


// Function: sub_11b14 @ 0x11b14
unsigned long sub_11b14(long a0,long a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  long v2; // stack - 0x10
  
  v1 = 0;
  for (v2 = 0; (*(char *)(v2 + a0) && (*(char *)(v2 + a1))); v2 = v2 + 1) {
    if (*(char *)(v2 + a0) != *(char *)(v2 + a1)) {
      if ((!(*(unsigned short *)((unsigned long)*(unsigned char *)(v2 + a0) * 2 + *(long *)__ctype_b_loc()) & 0x800)) || (!(*(unsigned short *)((unsigned long)*(unsigned char *)(v2 + a1) * 2 + *(long *)__ctype_b_loc()) & 0x800)))
        return 0;
      if (v1) {
        if (v2 - *a2 != *a3)
          return 0;
        *a3 = *a3 + 1;
      }
      else {
        *a2 = v2;
        *a3 = 1;
        v1 = 1;
      }
    }
  }
  if ((!*(char *)(v2 + a0)) && (!*(char *)(v2 + a1)))
    return 1;
  return 0;
}


// Function: sub_11c89 @ 0x11c89
long sub_11c89(char *a0,unsigned long *a1,void *a2,unsigned long a3) // ternary
{
  char v1; // al
  unsigned long v10; // stack - 0x60
  unsigned int v11; // stack - 0x54
  unsigned long v12; // stack - 0x50
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  unsigned long v15; // stack - 0x38
  unsigned long v16; // stack - 0x30
  unsigned long v17; // stack - 0x28
  unsigned long v2; // stack - 0x58
  long v3; // stack - 0x90
  long v4; // stack - 0x98
  char *v5; // stack - 0x88
  unsigned long v6; // stack - 0x80
  unsigned long v7; // stack - 0x78
  char *v8; // stack - 0x70
  long v9; // stack - 0x68
  
  v5 = NULL;
  v5 = (char *)sub_31bc2(strlen(a0) + 2);
  *v5 = '_';
  memcpy(&v5[1],a0,strlen(a0) + 1);
  v12 = a1[1];
  v13 = a1[2];
  v14 = a1[3];
  v15 = a1[4];
  v16 = a1[5];
  v17 = a1[6];
  v2 = (int)*a1;
  v11 = (unsigned int)((unsigned long)*a1 >> 0x20); // branch-flip
  v2 = (0xb <= v2) ? v2 + -0xb : v2 + 0xb;
  if (!dat_49a60) {
    dat_49a68 = 1;
    dat_49a60 = (char *)sub_31bc2(1);
  }
  while( true ) {
    v6 = strftime(dat_49a60,dat_49a68,v5,a1);
    if ((v6) && (v6 < dat_49a68)) break;
    dat_49a60 = (char *)sub_31dbf(dat_49a60,0x49a68,1);
  }
  v4 = 0;
  v3 = 0;
  v7 = a3 + v6 + 1;
  dat_49a60 = (long)sub_31c2c(dat_49a60,v7);
  dat_49a68 = v7;
  v8 = (char *)sub_31bc2(v7);
  strftime(v8,dat_49a68,v5,&v2);
  v1 = sub_11b14(dat_49a60,v8,&v4,&v3);
  if ((v1) && ((v3 == 2 && (!(*(unsigned short *)((unsigned long)*(unsigned char *)((long)dat_49a60 + v3 + v4) * 2 + *(long *)__ctype_b_loc()) & 0x800))))) {
    v9 = v3 + v4;
    v10 = (v6 - v9) + 1;
    if (v7 != v10 + v9 + a3)
      __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
    memmove((void *)((long)dat_49a60 + a3 + v9),(void *)((long)dat_49a60 + v9),v10);
    memcpy((void *)((long)dat_49a60 + v3 + v4),a2,a3);
  }
  free(v5);
  free(v8);
  return (long)dat_49a60 + 1;
}


// Function: sub_1201f @ 0x1201f
char * sub_1201f(long a0,unsigned long a1,int a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1201f
{
  long v1;
  char *v10; // stack - 0x80
  long v11; // stack - 0x78
  unsigned long v12; // stack - 0x70
  char *v13; // stack - 0x68
  unsigned long v14; // stack - 0x60
  char v15; // stack - 0x53
  char v16; // stack - 0x52
  char v2; // stack - 0x54
  char v3 [40]; // stack - 0x48
  long v4; // stack - 0xa8
  unsigned long v5; // stack - 0xa0
  int v6; // stack - 0x98
  int v7; // stack - 0x94
  char *v8; // stack - 0x90
  void *v9; // stack - 0x88
  
  v6 = 0;
  v7 = 0;
  v4 = a0;
  v5 = a1;
  if (a2 != 0x2b) { // branch-flip
    v2 = '%';
    v15 = (char)a2;
    v16 = 0;
    v7 = !((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned char)(a2 - 0x40U) & 0x3f) & 1)));
  }
  else {
    strcpy(&v2,"%Y-%m-%d+%T");
    v7 = 1;
  }
  if (v7) { // branch-flip
    v6 = snprintf(v3,0x20,".%09ld0",v5);
    if (0x20 <= v6)
      __assert_fail("charsprinted < NS_BUF_LEN","print.c",0x299,"format_date"); // no-return
  }
  else {
    v6 = 0;
    v3[0] = '\0';
  }
  if (a2 != 0x40) {
    v9 = localtime(&v4);
    if ((v9) && (v10 = (char *)sub_11c89(&v2,v9,v3,(long)v6), v10))
      return v10;
  }
  v11 = v4;
  if (0 <= v4) // branch-flip
    v1 = v4;
  else {
    v1 = -v4;
  }
  v8 = (char *)sub_22eb1(v1,0x49a81,0,1,1);
  if ((char *)0x49a81 <= v8) {
    if ((char *)0x49d97 <= v8)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v4 <= -1) {
      v8 = &v8[-1];
      *v8 = '-';
    }
    if (v7) {
      v12 = strlen(v8);
      v13 = &v8[v12 - 0x49a80];
      if ((char *)0x317 <= v13)
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v14 = 0x316 - (long)v13;
      if (v14 <= strlen(v3))
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)v6,v14,v3);
      if (v14 <= strlen(v3))
        __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      strcat(v8,v3);
    }
    return v8;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}


// Function: sub_123c8 @ 0x123c8
unsigned long sub_123c8(unsigned long a0,unsigned long a1) // early-return
{
  unsigned long v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  int v3; // stack - 0x14
  unsigned int *v4; // stack - 0x10
  
  v1 = a0;
  v2 = a1;
  v4 = localtime(&v1);
  if (!v4)
    return sub_1201f(v1,v2,0x40);
  if ((int)v4[6] <= -1)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (7 <= (int)v4[6])
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  if ((int)v4[4] <= -1)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xc <= (int)v4[4])
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  if ((int)v4[2] <= -1)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x18 <= (int)v4[2])
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  if ((int)v4[1] <= 0x3b) {
    if (0x3e <= (int)*v4)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v3 = snprintf((char *)0x49da0,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",*(unsigned long *)((long)(int)v4[6] * 8 + 0x49680),*(unsigned long *)((long)(int)v4[4] * 8 + 0x496c0),(unsigned long)v4[3],(unsigned long)v4[2],(unsigned long)v4[1],(unsigned long)*v4,v2,(unsigned long)(v4[5] + 0x76c));
    if (v3 <= 0x3ff)
      return 0x49da0;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}


// Function: sub_12693 @ 0x12693
double sub_12693(long a0)
{
  double v1;
  
  if (*(long *)(a0 + 0x30)) // branch-flip
    v1 = (dat_3acc0 * (double)*(long *)(a0 + 0x40)) / (double)*(long *)(a0 + 0x30);
  else if (*(long *)(a0 + 0x40)) { // branch-flip
    if (0 <= *(long *)(a0 + 0x40)) // branch-flip
      v1 = dat_3acb8;
    else {
      v1 = dat_3acb0;
    }
  }
  else {
    v1 = dat_3aca8;
  }
  return v1;
}


// Function: sub_1272c @ 0x1272c
void sub_1272c(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  unsigned long v12; // stack - 0x98
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  char v2; // al
  char v3 [16];
  unsigned int v4; // stack - 0xe8
  unsigned int v5; // stack - 0xe4
  char *v6; // stack - 0xe0
  char *v7; // stack - 0xd8
  unsigned long v8; // stack - 0xb8
  unsigned long v9; // stack - 0xb0
  
  if (v2) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v4 = 0x10;
  v5 = 0x30;
  v6 = &Stack0000000000000008;
  v7 = v3;
  v1 = *(unsigned long *)(a8 + 8);
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  if (0 <= (int)vfprintf(v1,a9,&v4))
    return;
  v1 = *(unsigned long *)(a8 + 0x10);
  sub_10a77(*__errno_location(),v1);
}


// Function: sub_12839 @ 0x12839
void sub_12839(long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  
  v1 = *(char *)(a0 + 0x18);
  v2 = *(unsigned long *)(a0 + 0x20);
  v3 = *(unsigned long *)(a0 + 8);
  if (0 <= (int)sub_1c643(v3,v2,v1,a1,a2))
    return;
  v2 = *(unsigned long *)(a0 + 0x10);
  sub_10a77(*__errno_location(),v2);
}


// Function: sub_128a9 @ 0x128a9
void sub_128a9(void *a0,unsigned long a1,unsigned long a2,long a3) // return-dupe
{
  void *v1;
  unsigned long v2;
  
  v1 = *(void **)(a3 + 8);
  if (a2 <= fwrite(a0,a1,a2,v1))
    return;
  v2 = *(unsigned long *)(a3 + 0x10);
  sub_10a77(*__errno_location(),v2);
}


// Function: sub_12910 @ 0x12910
void sub_12910(long a0) // return-dupe
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 8);
  if (!sub_1e1a8(v1))
    return;
  v1 = *(unsigned long *)(a0 + 0x10);
  sub_10a77(*__errno_location(),v1);
}


// Function: sub_12955 @ 0x12955
char * sub_12955(int a0) // return-dupe x7
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
  if (a0 == 0x1000)
    return "p";
  return "U";
}


// Function: sub_129e7 @ 0x129e7
void sub_129e7(unsigned long a0,unsigned int *a1,void *a2,unsigned long *a3)
{
  unsigned int v1;
  char *v10; // stack - 0x380
  void *v11; // stack - 0x378
  unsigned long *v12; // stack - 0x370
  char *v13; // stack - 0x368
  unsigned long *v14; // stack - 0x360
  void *v15; // stack - 0x358
  void *v16; // stack - 0x350
  unsigned int v17; // stack - 0x330
  char v18; // stack - 0x2ae
  int v2; // eax
  char *v3; // rax
  unsigned long v4; // rax
  char v5 [10];
  char v6 [24];
  unsigned long v7; // stack - 0x390
  char v8 [16];
  char *v9; // stack - 0x388
  
  v1 = *a1;
  if (v1 != 2) { // branch-flip
    if (v1 <= 2) {
      if (v1) { // branch-flip
        if (v1 == 1) {
          sub_128a9(*(unsigned long *)&a1[2],1,(long)(int)a1[4],a0);
          sub_12910(a0);
        }
      }
      else {
        sub_128a9(*(unsigned long *)&a1[2],1,(long)(int)a1[4],a0);
      }
    }
  }
  else {
    switch((char)a1[1]) {
      case 0:
        error(1,0,gettext("error: %s at end of format string"),"%");
        break;
      case 0x25:
        sub_128a9(*(unsigned long *)&a1[2],1,(long)(int)a1[4],a0);
        break;
      case 0x44:
        v4 = sub_22eb1(*a3,v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x46:
        v4 = sub_13a18(a3,a2);
        sub_12839(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x48:
        v16 = (void *)sub_31bc2((long)(dat_499dc + 1));
        memcpy(v16,a2,(long)dat_499dc);
        *(char *)((long)v16 + (long)dat_499dc) = 0;
        sub_1272c(a0,*(unsigned long *)&a1[2],v16);
        free(v16);
        break;
      case 0x4d:
        sub_1e472(a3,v5);
        v18 = 0;
        sub_1272c(a0,*(unsigned long *)&a1[2],v5);
        break;
      case 0x50:
        if (1 <= dat_499c0) { // branch-flip
          v9 = (char *)((long)a2 + (long)dat_499dc);
          if (*v9 == '/')
            v9 = &v9[1];
        }
        else {
          v9 = "";
        }
        sub_12839(a0,*(unsigned long *)&a1[2],v9);
        break;
      case 0x53:
        sub_12693(a3);
        sub_1272c(a0,*(unsigned long *)&a1[2]);
        break;
      case 0x59:
        if ((*(unsigned int *)&a3[3] & 0xf000) != 0xa000) { // branch-flip
          v4 = sub_12955(*(unsigned int *)&a3[3] & 0xf000);
          sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        }
        else {
          v2 = fstatat(dat_499d8,dat_499d0,v6,0);
          if (v2) { // branch-flip
            if ((*__errno_location() != 2) && (*__errno_location() != 0x14)) {
              if (*__errno_location() != 0x28) { // branch-flip
                sub_1272c(a0,*(unsigned long *)&a1[2],"?");
                v4 = sub_1096a(0,a2);
                error(0,*__errno_location(),"%s",v4);
              }
              else {
                sub_1272c(a0,*(unsigned long *)&a1[2],"L");
              }
            }
            else {
              sub_1272c(a0,*(unsigned long *)&a1[2],"N");
            }
          }
          else {
            v4 = sub_12955(v17 & 0xf000);
            sub_1272c(a0,*(unsigned long *)&a1[2],v4);
          }
        }
        break;
      case 0x5a:
        v2 = (*dat_499a0)(dat_499d8,dat_499d0,&v7);
        if (0 <= v2) { // branch-flip
          sub_1272c(a0,*(unsigned long *)&a1[2],v7);
          freecon(v7);
        }
        else {
          sub_1272c(a0,*(unsigned long *)&a1[2],0x3aa45);
          v4 = sub_1096a(0,a2);
          v3 = gettext("getfilecon failed: %s");
          error(0,*__errno_location(),v3,v4);
          dat_499e4 = 1;
        }
        break;
      case 0x61:
        v8 = sub_30a4c(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x62:
        v4 = sub_22eb1(a3[8],v5,0,0x200,0x200);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 99:
        v8 = sub_30a66(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 100:
        sub_1272c(a0,*(unsigned long *)&a1[2],dat_499c0);
        break;
      case 0x66:
        v15 = (void *)sub_1d85c(a2);
        sub_12839(a0,*(unsigned long *)&a1[2],v15);
        free(v15);
        break;
      case 0x67:
        v14 = getgrgid(*(unsigned int *)&a3[4]);
        if (v14) {
          *(char *)((long)(int)a1[4] + *(long *)&a1[2]) = 0x73;
          sub_1272c(a0,*(unsigned long *)&a1[2],*v14);
          return;
        }
        v4 = sub_22eb1(*(unsigned int *)&a3[4],v5,0,1,1); // return-dupe
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        return;
      case 0x47:
        v4 = sub_22eb1(*(unsigned int *)&a3[4],v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x68:
        v3 = (char *)sub_321c9(a2);
        v13 = v3;
        for (v10 = &v13[strlen(v3) - 1]; (v13 <= v10 && (*v10 == '/')); v10 = &v10[-1]) {
        }
        if ((v13 < v10) && (v10[1] == '/'))
          v10[1] = '\0';
        v10 = strrchr(v13,0x2f);
        if (v10) { // branch-flip
          *v10 = '\0';
          sub_12839(a0,*(unsigned long *)&a1[2],v13);
        }
        else {
          sub_12839(a0,*(unsigned long *)&a1[2],".");
        }
        free(v13);
        break;
      case 0x69:
        v4 = sub_22eb1(a3[1],v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x6b:
        v4 = sub_22eb1(a3[8],v5,0,0x200,0x400);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x6c:
        v11 = NULL;
        if ((*(unsigned int *)&a3[3] & 0xf000) == 0xa000) {
          v11 = (void *)sub_1d06d(dat_499d8,dat_499d0);
          if (!v11) {
            sub_10a13(*__errno_location(),a2);
            dat_499e4 = 1;
          }
        }
        if (v11) // branch-flip
          sub_12839(a0,*(unsigned long *)&a1[2],v11);
        else {
          sub_12839(a0,*(unsigned long *)&a1[2],0x3aa45);
        }
        free(v11);
        break;
      case 0x6d:
        sub_1272c(a0,*(unsigned long *)&a1[2],*(unsigned int *)&a3[3] & 0xfff);
        break;
      case 0x6e:
        v4 = sub_22eb1(a3[2],v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x70:
        sub_12839(a0,*(unsigned long *)&a1[2],a2);
        break;
      case 0x73:
        v4 = sub_22eb1(a3[6],v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x74:
        v8 = sub_30a80(a3);
        v4 = sub_123c8(SUB168(v8,0),SUB168(v8,8));
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x75:
        v12 = getpwuid(*(unsigned int *)((long)a3 + 0x1c));
        if (v12) {
          *(char *)((long)(int)a1[4] + *(long *)&a1[2]) = 0x73;
          sub_1272c(a0,*(unsigned long *)&a1[2],*v12);
          return;
        }
        v4 = sub_22eb1(*(unsigned int *)((long)a3 + 0x1c),v5,0,1,1); // return-dupe
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        return;
      case 0x55:
        v4 = sub_22eb1(*(unsigned int *)((long)a3 + 0x1c),v5,0,1,1);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
        break;
      case 0x79:
        v4 = sub_12955(*(unsigned int *)&a3[3] & 0xf000);
        sub_1272c(a0,*(unsigned long *)&a1[2],v4);
      
    }
  }
  return;
}


// Function: sub_1376d @ 0x1376d
unsigned long sub_1376d(unsigned long a0,unsigned long a1,long a2)
{
  char v1;
  unsigned char v2;
  unsigned long *v3; // rax
  unsigned long v4; // rax
  char v5 [16];
  int *v6; // stack - 0x28
  unsigned long v7; // stack - 0x18
  unsigned long v8; // stack - 0x10
  
  v3 = (unsigned long *)(a2 + 0x38);
  v6 = (int *)*v3;
  do {
    if (!v6)
      return 1;
    if ((*v6 != 2) || (!*(char *)((long)v6 + 5)))
      sub_129e7(v3,v6,a0,a1);
    else {
      v1 = (char)v6[1];
      if (v1 != 'T') { // branch-flip
        if ('T' < v1)
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        if (v1 != 'C') { // branch-flip
          if ('C' < v1) {
            __assert_fail("0","print.c",0x518,"pred_fprintf");
          }
          if (v1 != 'A') { // branch-flip
            if (v1 != 'B') {
              __assert_fail("0","print.c",0x518,"pred_fprintf");
            }
            v5 = sub_30a9a(a1);
            v8 = SUB168(v5,8);
            v7 = SUB168(v5,0);
            if (*(char *)((long)v6 + 5) != '@') // branch-flip
              v2 = (unsigned char)~SUB161(v5,0xf) >> 7;
            else {
              v2 = 1;
            }
          }
          else {
            v5 = sub_30a4c(a1);
            v8 = SUB168(v5,8);
            v7 = SUB168(v5,0);
            v2 = 1;
          }
        }
        else {
          v5 = sub_30a66(a1);
          v8 = SUB168(v5,8);
          v7 = SUB168(v5,0);
          v2 = 1;
        }
      }
      else {
        v5 = sub_30a80(a1);
        v8 = SUB168(v5,8);
        v7 = SUB168(v5,0);
        v2 = 1;
      }
      if (v2) { // branch-flip
        v4 = sub_1201f(v7,v8,(int)*(char *)((long)v6 + 5));
        sub_1272c(v3,*(unsigned long *)&v6[2],v4);
      }
      else {
        sub_1272c(v3,*(unsigned long *)&v6[2],0x3aa45);
      }
    }
    v6 = *(int **)&v6[6];
  } while( true );
}


// Function: sub_1395f @ 0x1395f
void sub_1395f(long a0)
{
  long v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  while (v2) {
    v1 = *(long *)(v2 + 0x30);
    sub_26f67(v2);
    v2 = v1;
  }
}


// Function: sub_1399c @ 0x1399c
long sub_1399c(char a0)
{
  if (((dat_4a1a0) && (dat_4a1a8 != '\x01')) && (a0)) {
    sub_1395f(dat_4a1a0);
    dat_4a1a0 = 0;
  }
  if (!dat_4a1a0) {
    dat_4a1a0 = sub_262d1(a0);
    dat_4a1a8 = a0;
  }
  return dat_4a1a0;
}


// Function: sub_13a18 @ 0x13a18
void * sub_13a18(long *a0,unsigned long a1)
{
  if (dat_4a1b0) {
    if ((dat_4a1b8) && (*a0 == dat_4a1c0))
      return dat_4a1b0;
    free(dat_4a1b0);
  }
  dat_4a1c0 = *a0;
  dat_4a1b0 = (void *)sub_13be0(a0,a1,0x4a1b8);
  return dat_4a1b0;
}


// Function: sub_13aa6 @ 0x13aa6
unsigned long sub_13aa6(char *a0) // return-dupe x2
{
  char *v1;
  long v2; // stack - 0x18
  
  if (!strcmp("afs",a0))
    return 1;
  v2 = sub_1399c(0);
  if (v2) {
    while( true ) {
      if (!v2)
        return 0;
      v1 = *(char **)(v2 + 0x18);
      if (!strcmp(a0,v1)) break;
      v2 = *(long *)(v2 + 0x30);
    }
    return 1;
  }
  return 1;
}


// Function: sub_13b3b @ 0x13b3b
unsigned long sub_13b3b(long a0) // return-dupe x2
{
  int v1; // eax
  unsigned long v2 [19]; // stack - 0xa8
  
  if (*(long *)(a0 + 0x20) != -1)
    return 0;
  sub_f26e(v2);
  v1 = (*dat_49990)(*(unsigned long *)(a0 + 8),v2);
  if (!v1) {
    *(unsigned long *)(a0 + 0x20) = v2[0];
    return 0;
  }
  return 0xffffffff;
}


// Function: sub_13be0 @ 0x13be0
long sub_13be0(long *a0,unsigned long a1,unsigned long a2)
{
  char *v1;
  unsigned long v2;
  long v3; // rax
  long v4; // stack - 0x28
  long v5; // stack - 0x20
  long v6; // stack - 0x18
  
  v5 = 0;
  v4 = sub_1399c(1);
  if (v4) {
    v6 = 0;
    for (; v4; v4 = *(long *)(v4 + 0x30)) {
      v1 = *(char **)(v4 + 0x18);
      if (((strcmp(v1,"ignore")) && (!sub_13b3b(v4))) && (*(long *)(v4 + 0x20) == *a0))
        v5 = v4;
    }
    if (v5) {
      v2 = *(unsigned long *)(v5 + 0x18);
      v6 = sub_321c9(v2);
    }
    *(bool *)a2 = v6 != 0;
    if (!v6)
      v6 = sub_321c9(gettext("unknown"));
    return v6;
  }
  error(1,0,gettext("Cannot read mounted file system list"));
  return v3;
}


// Function: sub_13cfc @ 0x13cfc
void * sub_13cfc(long *a0)
{
  int v1; // eax
  unsigned long v2; // stack - 0x40
  long v3; // stack - 0x38
  long v4; // stack - 0x30
  void *v5; // stack - 0x28
  long v6; // stack - 0x20
  void *v7; // stack - 0x18
  
  v2 = 0;
  v3 = 0;
  v5 = NULL;
  v6 = sub_262d1(0);
  for (v4 = v6; v4; v4 = *(long *)(v4 + 0x30)) {
    v7 = (void *)sub_1ac29(v5,(v3 + 1) * 8,&v2);
    if (v7) { // branch-flip
      v5 = v7;
      v1 = sub_13b3b(v4);
      if (!v1) {
        *(unsigned long *)(v3 * 8 + (long)v5) = *(unsigned long *)(v4 + 0x20);
        v3 += 1;
      }
    }
    else {
      free(v5);
      v5 = NULL;
    }
  }
  sub_1395f(v6);
  if (v5)
    *a0 = v3;
  return v5;
}


// Function: sub_13e1c @ 0x13e1c
int * sub_13e1c(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  int *v1; // rax
  
  v1 = __errno_location();
  if ((*v1 != 2) && (*v1 != 0x14))
    v1 = (unsigned long)(unsigned long)a3;
  else {
    if (dat_49980 & 2)
      fprintf(stderr,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",a1);
    v1 = (unsigned long)sub_3011a(a0,a1,a2);
  }
  return v1;
}


// Function: sub_13e94 @ 0x13e94
unsigned long sub_13e94(unsigned int a0,unsigned long a1,unsigned long a2) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_499c0) // branch-flip
    v2 = sub_3011a(a0,a1,a2);
  else {
    v1 = sub_2fe8c(a0,a1,a2);
    if (!v1) {
      v2 = 0;
      return v2;
    }
    v2 = sub_13e1c(a0,a1,a2,v1);
  }
  return v2;
}


// Function: sub_13f08 @ 0x13f08
unsigned long sub_13f08(unsigned int a0,unsigned long a1,unsigned long a2) // early-return
{
  int v1; // eax
  
  v1 = sub_2fe8c(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_13e1c(a0,a1,a2,v1);
}


// Function: sub_13f5b @ 0x13f5b
void sub_13f5b(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_3011a(a0,a1,a2);
}


// Function: sub_13f89 @ 0x13f89
void sub_13f89(long *a0)
{
  bool v1;
  bool v2;
  long *v3; // stack - 0x20
  
  v2 = 0;
  v1 = 0;
  for (v3 = a0; v3; v3 = (long *)v3[0x21]) {
    if ((void *)*v3 != sub_90b1) { // branch-flip
      if ((void *)*v3 == sub_a232)
        v1 = 1;
    }
    else {
      v2 = 1;
    }
  }
  if (((v1) && (v2)) && (dat_49941 != '\x01')) {
    error(1,0,gettext("The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."));
    return;
  }
}


// Function: sub_14033 @ 0x14033
long sub_14033(void)
{
  int v1; // stack - 0xc
  
  if (!dat_4a1d0) {
    v1 = 0;
    while (*(long *)((long)v1 * 0x20 + 0x47de8)) {
      if (*(int *)((long)v1 * 0x20 + 0x47de0) == 1) {
        dat_4a1d0 = (long)v1 * 0x20 + 0x47de0;
        return dat_4a1d0;
      }
      v1 += 1;
    }
  }
  return dat_4a1d0;
}


// Function: sub_140b3 @ 0x140b3
unsigned long sub_140b3(unsigned long a0,char a1,unsigned long *a2) // early-return x3
{
  char v1 [16];
  
  if (a1 == 'm') {
    v1 = sub_30a80(a0);
    *a2 = SUB168(v1,0);
    a2[1] = SUB168(v1,8);
    return 1;
  }
  if (a1 <= 'm') {
    if (a1 == 'c') {
      v1 = sub_30a66(a0);
      *a2 = SUB168(v1,0);
      a2[1] = SUB168(v1,8);
      return 1;
    }
    if (a1 <= 'c') {
      if (a1 == 'B') {
        v1 = sub_30a9a(a0);
        *a2 = SUB168(v1,0);
        a2[1] = SUB168(v1,8);
        return (unsigned long)~a2[1] >> 0x3f;
      }
      if (a1 == 'a') {
        v1 = sub_30a4c(a0);
        *a2 = SUB168(v1,0);
        a2[1] = SUB168(v1,8);
        return 1;
      }
    }
  }
  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return
}


// Function: sub_141b0 @ 0x141b0
void sub_141b0(unsigned int a0) // return-dupe
{
  switch(a0) { // branch-flip
    case 0:
      dat_49990 = sub_fcc0;
      dat_499a0 = sub_13f5b;
      break;
    case 1:
      dat_49990 = sub_fc21;
      dat_499a0 = sub_13f08;
      dat_4994c = 1;
      break;
    case 2:
      dat_49990 = sub_fb5c;
      dat_499a0 = sub_13e94;
      dat_4994c = 1;
    
  }
  dat_49988 = a0;
  if (!(dat_49980 & 2))
    return;
  dat_49990 = sub_fd39;
}


// Function: sub_14262 @ 0x14262
void sub_14262(void)
{
  dat_4a1c8 = 0;
}


// Function: sub_14287 @ 0x14287
void sub_14287(void)
{
  return;
}


// Function: sub_142a1 @ 0x142a1
char sub_142a1(void) // early-return
{
  if (dat_49951)
    return 0;
  return dat_49950;
}


// Function: sub_142c4 @ 0x142c4
int * sub_142c4(long a0,int *a1)
{
  long v1;
  char v2; // al
  
  if (*a1 != 2) {
    if (*a1 == 1)
      return NULL;
    if (*a1) { // branch-flip
      if (!dat_4a1c8)
        dat_4a1c8 = a0;
    }
    else if ((dat_4a1c8) && (v2 = sub_142a1(), v1 = dat_4a1c8, v2))
      error(0,0,gettext("warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments."),a0,v1,a0);
  }
  return a1;
}


// Function: sub_1437a @ 0x1437a
unsigned long sub_1437a(char *a0)
{
  char *v1;
  char *v2; // stack - 0x20
  int v3; // stack - 0x14
  
  if ((!strncmp("-newer",a0,6)) && (strlen(a0) == 8))
    return sub_142c4(a0,0x47dc0);
  v2 = a0;
  if (*a0 == '-')
    v2 = &a0[1];
  v3 = 0;
  while( true ) {
    if (!*(long *)((long)v3 * 0x20 + 0x47de8))
      return 0;
    v1 = *(char **)((long)v3 * 0x20 + 0x47de8);
    if (!strcmp(v1,v2)) break;
    v3 += 1;
  }
  return sub_142c4(a0,(long)v3 * 0x20 + 0x47de0);
}


// Function: sub_1446f @ 0x1446f
unsigned int sub_1446f(float a0)
{
  unsigned int v1;
  
  if (dat_3c028 <= a0) {
    v1 = dat_3c034;
    if ((dat_3c030 > a0) || (a0 > dat_3c038)) {
      return v1; // return-dupe
    }
    v1 = dat_3c03c;
  }
  else {
    v1 = dat_3c02c;
  }
  return v1;
}


// Function: sub_144ce @ 0x144ce
float sub_144ce(long a0)
{
  a0 = dat_49968 - a0;
  sub_1446f();
  return (float)(int)(a0 / 0x15180);
}


// Function: sub_14528 @ 0x14528
unsigned long sub_14528(long a0,int *a1,unsigned long *a2) // return-dupe
{
  if ((a0) && (*(long *)(a0 + (long)*a1 * 8))) {
    *a2 = *(unsigned long *)(a0 + (long)*a1 * 8);
    *a1 = *a1 + 1;
    return 1;
  }
  *a2 = 0;
  return 0;
}


// Function: sub_145ab @ 0x145ab
char sub_145ab(unsigned long a0,unsigned long a1,unsigned long *a2)
{
  char v1; // al
  unsigned long v2; // stack - 0x18
  
  v1 = sub_14528(a0,a1,&v2);
  *a2 = v2;
  return v1;
}


// Function: sub_14611 @ 0x14611
unsigned long sub_14611(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  int v1; // eax
  unsigned long v2; // stack - 0x28
  
  if (!sub_145ab(a0,a1,&v2)) {
    *a3 = 0;
    return 0;
  }
  *a3 = v2;
  v1 = (*dat_49990)(v2,a2);
  if (!v1)
    return 1;
  sub_10a44(*__errno_location(),v2); // no-return
}


// Function: sub_146c4 @ 0x146c4
unsigned long sub_146c4(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e644(a0);
  *v1 = sub_8d9c;
  *(unsigned int *)&v1[2] = 3;
  *(unsigned int *)((long)v1 + 0x14) = 3;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  return 1;
}


// Function: sub_1472e @ 0x1472e
bool sub_1472e(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  unsigned long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    *(unsigned int *)(v1 + 0x38) = 0;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}


// Function: sub_14833 @ 0x14833
unsigned long sub_14833(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e644(a0);
  *v1 = sub_8ee1;
  *(unsigned int *)&v1[2] = 5;
  *(unsigned int *)((long)v1 + 0x14) = 0;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  return 1;
}


// Function: sub_1489d @ 0x1489d
bool sub_1489d(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  unsigned long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    *(unsigned int *)(v1 + 0x38) = 2;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}


// Function: sub_149a2 @ 0x149a2
unsigned long sub_149a2(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e644(a0);
  *v1 = sub_8fd0;
  *(unsigned int *)&v1[2] = 3;
  *(unsigned int *)((long)v1 + 0x14) = 1;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  *(unsigned int *)((long)v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_14a1d @ 0x14a1d
int * sub_14a1d(void) // return-dupe
{
  int *v1;
  int v2; // eax
  int *v3; // rax
  int v4;
  
  if (dat_49978 == '\x01') {
    v3 = (int *)0x1;
    return v3;
  }
  v3 = (long)(dat_49968 + 0x15180);
  dat_49970 = 0;
  dat_49968 = (long)v3;
  v3 = localtime((void *)0x49968);
  v1 = v3;
  if (v3) { // branch-flip
    v4 = *v3;
    v2 = v3[1] * 0x3c;
    v4 += v2;
    v2 = v1[2] * 0xe10;
    v2 += v4;
    v3 = (long)(long)v2;
  }
  else {
    v3 = (long)(dat_49968 % 0x15180);
  }
  dat_49968 -= (long)v3;
  v3 = (int *)0x1;
  dat_49978 = 1;
  return v3;
}


// Function: sub_14afc @ 0x14afc
unsigned long sub_14afc(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  dat_49940 = 0;
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_14b71 @ 0x14b71
void sub_14b71(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49940 = 0;
  dat_49941 = 1;
  sub_17bae(a0,a1,a2);
}


// Function: sub_14bb0 @ 0x14bb0
void sub_14bb0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if (sub_142a1())
    error(0,0,gettext("warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature."));
  sub_14b71(a0,a1,a2);
}


// Function: sub_14c10 @ 0x14c10
unsigned long sub_14c10(unsigned long a0)
{
  *(unsigned int *)(sub_ef32(a0) + 0x24) = dat_3c02c;
  return 1;
}


// Function: sub_14c50 @ 0x14c50
void sub_14c50(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1893a("-exec",a0,a1,a2);
}


// Function: sub_14c88 @ 0x14c88
void sub_14c88(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1893a("-execdir",a0,a1,a2);
}


// Function: sub_14cc0 @ 0x14cc0
unsigned long sub_14cc0(void)
{
  long v1; // rax
  
  v1 = sub_ef32(sub_1437a("false"));
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(char *)(v1 + 0x19) = 0;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}


// Function: sub_14d31 @ 0x14d31
void sub_14d31(void)
{
  sub_14cc0();
}


// Function: sub_14d50 @ 0x14d50
bool sub_14d50(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x18
  bool v2; // zf
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2)
    dat_499b0 = v1;
  return v2;
}


// Function: sub_14dbf @ 0x14dbf
unsigned long sub_14dbf(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  if (a1) // branch-flip
    sub_19964(a1,v1 + 0x38);
  else {
    sub_19a85(v1 + 0x38);
  }
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_14e42 @ 0x14e42
unsigned long sub_14e42(unsigned long a0,unsigned long a1,int *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v1))
    return 0;
  if (!sub_14dbf(a0,v1)) {
    *a2 = *a2 + -1;
    return 0;
  }
  return 1;
}


// Function: sub_14ecc @ 0x14ecc
void sub_14ecc(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_141b0(1);
  sub_17bae(a0,a1,a2);
}


// Function: sub_14f07 @ 0x14f07
bool sub_14f07(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    sub_19964(v1,v3 + 0x38);
    *(char *)(v3 + 0x19) = 1;
    *(char *)(v3 + 0x18) = *(char *)(v3 + 0x19);
    *(char *)(v3 + 0x1b) = 0;
    *(char *)(v3 + 0x1a) = *(char *)(v3 + 0x1b);
    *(unsigned int *)(v3 + 0x24) = dat_3c030;
  }
  return v2;
}


// Function: sub_14fd8 @ 0x14fd8
unsigned long sub_14fd8(unsigned long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_eecc(a0,a1);
  if (a1) // branch-flip
    sub_19964(a1,v1 + 0x38);
  else {
    sub_19a85(v1 + 0x38);
  }
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_15079 @ 0x15079
unsigned long sub_15079(unsigned long a0,unsigned long a1,int *a2) // return-dupe
{
  unsigned long v1; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v1))
    return 0;
  if (!sub_14fd8(a0,v1)) {
    *a2 = *a2 + -1;
    return 0;
  }
  return 1;
}


// Function: sub_15103 @ 0x15103
unsigned int sub_15103(char *a0)
{
  char *v1; // rax
  char v2 [152];
  unsigned int v3;
  
  if (stat("/",v2)) // branch-flip
    v3 = dat_3c030;
  else {
    v1 = (char *)sub_13a18(v2,"/");
    if (strcmp(a0,v1)) // branch-flip
      v3 = dat_3c034;
    else {
      v3 = dat_3c040;
    }
  }
  return v3;
}


// Function: sub_151c2 @ 0x151c2
unsigned long sub_151c2(unsigned long a0,unsigned long a1,unsigned long a2) // early-return x2
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x20
  unsigned int v4; // xmm0_da
  long v5; // stack - 0x18
  
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  if ((2 <= dat_499a8) && (v1 = sub_13aa6(v3), !v1)) {
    if (dat_49980 & 8)
      fprintf(stderr,"-fstype %s can never succeed, substituting -false\n",v3);
    v2 = sub_14cc0();
    return v2;
  }
  v5 = sub_eecc(a0,v3);
  *(unsigned long *)(v5 + 0x38) = v3;
  sub_15103(v3);
  *(unsigned int *)(v5 + 0x24) = v4;
  return 1;
}


// Function: sub_152b9 @ 0x152b9
unsigned long sub_152b9(unsigned long a0,unsigned long a1,int *a2) // return-dupe
{
  long v1; // rax
  unsigned int v2;
  
  v1 = sub_197bf(a1,a2,a0);
  if (v1) {
    if (100 <= *(unsigned long *)(v1 + 0x40)) // branch-flip
      v2 = dat_3c048;
    else {
      v2 = dat_3c044;
    }
    *(unsigned int *)(v1 + 0x24) = v2;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}


// Function: sub_15339 @ 0x15339
unsigned long sub_15339(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  int v1; // eax
  unsigned int v10;
  unsigned int v11; // stack - 0x44
  void *v12; // stack - 0x30
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  long v8; // rax
  char *v9; // stack - 0x38
  
  if (!sub_145ab(a1,a2,&v9))
    return 0;
  v12 = getgrnam(v9);
  endgrent();
  if (v12) // branch-flip
    v11 = *(unsigned int *)((long)v12 + 0x10);
  else {
    v2 = strspn(v9,"0123456789");
    v1 = (int)v2;
    if (!v1) {
      if (*v9) {
        v3 = sub_2f861(0,dat_499ac,v9);
        error(1,0,gettext("%s is not the name of an existing group"),v3);
        return v6;
      }
      error(1,0,gettext("argument to -group is empty, but should be a group name"));
      return v7;
    }
    if (v9[v1]) {
      v3 = sub_2f861(1,dat_499ac,&v9[v1]);
      v4 = sub_2f861(0,dat_499ac,v9);
      error(1,0,gettext("%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"),v4,v3);
      return v5;
    }
    v11 = sub_1cd12(v9,dat_499ac);
  }
  v8 = sub_eecc(a0,v9);
  *(unsigned int *)(v8 + 0x38) = v11;
  if (100 <= *(unsigned long *)(v8 + 0x40)) // branch-flip
    v10 = dat_3c048;
  else {
    v10 = dat_3c044;
  }
  *(unsigned int *)(v8 + 0x24) = v10;
  return 1;
}


// Function: sub_15554 @ 0x15554
void sub_15554(void)
{
  sub_f071(0); // no-return
}


// Function: sub_15576 @ 0x15576
unsigned int sub_15576(char *a0,int a1)
{
  unsigned int v1;
  
  if (strpbrk(a0,"*?[")) // branch-flip
    v1 = dat_3c04c; // return-dupe
  else {
    if (a1) {
      if (strpbrk(a0,".")) {
        v1 = dat_3c04c;
        return v1;
      }
    }
    v1 = dat_3c028;
  }
  return v1;
}


// Function: sub_155d9 @ 0x155d9
bool sub_155d9(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    *(unsigned long *)(v3 + 0x38) = v1;
    *(float *)(v3 + 0x24) = (float)sub_15576(v1,0) * dat_3c028;
  }
  return v2;
}


// Function: sub_15686 @ 0x15686
unsigned long sub_15686(void)
{
  unsigned long v1; // rax
  
  if (dat_4a1d8 == 1)
    return (unsigned long)dat_4a1d8; // return-dupe
  if (!fnmatch("foo","foo",0)) {
    if (fnmatch("Foo","foo",0)) {
      if (!fnmatch("Foo","foo",0x10)) {
        dat_4a1d8 = 1;
        return (unsigned long)dat_4a1d8;
      }
    }
  }
  error(1,0,gettext("sanity check of the fnmatch() library function failed."));
  return v1;
}


// Function: sub_1573c @ 0x1573c
unsigned long sub_1573c(unsigned long a0,unsigned long a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  if (sub_142a1()) {
    if (strchr(a2,0x2f)) {
      v1 = sub_1096a(2,a1);
      v2 = sub_1096a(1,"/");
      v3 = sub_1096a(0,a0);
      error(0,0,gettext("warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?"),v3,v2,v1);
    }
  }
  return 1;
}


// Function: sub_157fa @ 0x157fa
unsigned long sub_157fa(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char v1; // al
  unsigned long v2; // stack - 0x20
  unsigned int v3; // xmm0_da
  long v4; // stack - 0x18
  
  sub_15686();
  if ((sub_145ab(a1,a2,&v2)) && (v1 = sub_1573c("-iname","-iwholename",v2), v1)) {
    v4 = sub_eecc(a0,v2);
    *(char *)(v4 + 0x1b) = 0;
    *(char *)(v4 + 0x1a) = *(char *)(v4 + 0x1b);
    *(unsigned long *)(v4 + 0x38) = v2;
    sub_15576(v2,0);
    *(unsigned int *)(v4 + 0x24) = v3;
    return 1;
  }
  return 0;
}


// Function: sub_158e1 @ 0x158e1
bool sub_158e1(unsigned long a0,unsigned long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_197bf(a1,a2,a0);
  if (!v1)
    *a2 = *a2 + -1;
  else {
    *(unsigned int *)(v1 + 0x24) = dat_3c050;
    *(char *)(v1 + 0x1c) = 1;
    *(char *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1b) = 0;
  }
  return v1 != 0;
}


// Function: sub_15961 @ 0x15961
void sub_15961(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1723e(a1,a2,a0,dat_4999c | 0x400000);
}


// Function: sub_1599c @ 0x1599c
unsigned long sub_1599c(unsigned long a0,unsigned long a1,int *a2) // return-dupe, return-dupe x2
{
  long v1; // rax
  
  v1 = sub_197bf(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(long *)(v1 + 0x40) == 1) {
    *(unsigned int *)(v1 + 0x24) = dat_3c044;
    return 1;
  }
  if (*(long *)(v1 + 0x40) == 2) {
    *(unsigned int *)(v1 + 0x24) = dat_3c02c;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = dat_3c054;
  return 1;
}


// Function: sub_15a46 @ 0x15a46
bool sub_15a46(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  bool v2; // zf
  long v3; // stack - 0x18
  
  sub_15686();
  v2 = sub_145ab(a1,a2,&v1) != '\0';
  if (v2) {
    v3 = sub_eecc(a0,v1);
    *(unsigned long *)(v3 + 0x38) = v1;
    *(float *)(v3 + 0x24) = (float)sub_15576(v1,0) * dat_3c028;
  }
  return v2;
}


// Function: sub_15af8 @ 0x15af8
void sub_15af8(unsigned long a0)
{
  sub_14dbf(a0,0);
}


// Function: sub_15b23 @ 0x15b23
unsigned long sub_15b23(unsigned long a0,long a1,int *a2,int *a3)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  
  v6 = *(unsigned long *)(a1 + (long)*a2 * 8 + -8);
  if (!sub_145ab(a1,a2,&v5))
    return 0;
  v2 = strspn(v5,"0123456789");
  v1 = (int)v2;
  if ((1 <= v1) && (!v5[v1])) {
    v1 = sub_1cd12(v5,dat_499ac);
    *a3 = v1;
    if (0 <= *a3)
      return sub_17bae(a0,a1,a2);
  }
  v3 = sub_2f861(0,dat_499ac,v5);
  error(1,0,gettext("Expected a positive decimal integer argument to %s, but got %s"),v6,v3);
  return v4;
}


// Function: sub_15c63 @ 0x15c63
void sub_15c63(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15b23(a0,a1,a2,0x49944);
}


// Function: sub_15c98 @ 0x15c98
void sub_15c98(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15b23(a0,a1,a2,0x49948);
}


// Function: sub_15ccd @ 0x15ccd
unsigned long sub_15ccd(unsigned long a0,unsigned long a1,unsigned int *a2,unsigned int a3) // return-dupe
{
  unsigned int v1;
  unsigned long v10; // stack - 0x20
  unsigned long v11; // stack - 0x18
  char v2; // al
  unsigned long v3; // stack - 0x48
  unsigned int v4; // stack - 0x28
  unsigned int v5; // xmm0_da
  long v6; // stack - 0x40
  long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned int v9; // stack - 0x24
  
  v1 = *a2;
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  v8 = dat_49970;
  v7 = dat_49968 + 0x15180;
  v4 = a3;
  v2 = sub_1916a(v3,&v4,v7,dat_49970,"arithmetic overflow while converting %s minutes to a number of seconds");
  if (!v2) {
    *a2 = v1;
    return 0;
  }
  v6 = sub_eecc(a0,v3);
  *(unsigned long *)(v6 + 0x38) = CONCAT44(v9,v4);
  *(unsigned long *)(v6 + 0x40) = v10;
  *(unsigned long *)(v6 + 0x48) = v11;
  sub_144ce(v10);
  *(unsigned int *)(v6 + 0x24) = v5;
  return 1;
}


// Function: sub_15df0 @ 0x15df0
void sub_15df0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15ccd(a0,a1,a2,0);
}


// Function: sub_15e23 @ 0x15e23
void sub_15e23(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15ccd(a0,a1,a2,2);
}


// Function: sub_15e56 @ 0x15e56
void sub_15e56(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15ccd(a0,a1,a2,3);
}


// Function: sub_15e89 @ 0x15e89
unsigned long sub_15e89(unsigned long a0,unsigned long a1,unsigned int *a2) // return-dupe
{
  unsigned int v1;
  char v2; // al
  unsigned long v3; // stack - 0x20
  unsigned int v4; // xmm0_da
  long v5; // stack - 0x18
  
  v1 = *a2;
  if (!sub_145ab(a1,a2,&v3))
    return 0;
  sub_15686();
  v2 = sub_1573c("-name","-wholename",v3);
  if (!v2) {
    *a2 = v1;
    return 0;
  }
  v5 = sub_eecc(a0,v3);
  *(char *)(v5 + 0x1b) = 0;
  *(char *)(v5 + 0x1a) = *(char *)(v5 + 0x1b);
  *(unsigned long *)(v5 + 0x38) = v3;
  sub_15576(v3,0);
  *(unsigned int *)(v5 + 0x24) = v4;
  return 1;
}


// Function: sub_15f82 @ 0x15f82
unsigned long sub_15f82(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e7dd(a0,0);
  *v1 = sub_9b30;
  *(unsigned int *)&v1[2] = 2;
  *(unsigned int *)((long)v1 + 0x14) = 4;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  return 1;
}


// Function: sub_15ff1 @ 0x15ff1
bool sub_15ff1(unsigned long a0,unsigned long a1,unsigned long a2)
{
  long v1; // rax
  char v2 [88];
  unsigned long v3; // stack - 0xc8
  bool v4; // zf
  unsigned int v5; // xmm0_da
  char v6 [16];
  long v7; // stack - 0xc0
  unsigned long v8; // stack - 0x60
  
  sub_f26e(v2);
  v4 = sub_14611(a1,a2,v2,&v3) != '\0';
  if (v4) {
    v1 = sub_eecc(a0,v3);
    v7 = v1;
    v6 = sub_30a80(v2);
    *(long *)(v1 + 0x40) = SUB168(v6,0);
    *(long *)(v1 + 0x48) = SUB168(v6,8);
    *(unsigned int *)(v7 + 0x38) = 3;
    *(unsigned int *)(v7 + 0x3c) = 0;
    sub_144ce(v8);
    *(unsigned int *)(v7 + 0x24) = v5;
  }
  return v4;
}


// Function: sub_160f6 @ 0x160f6
long * sub_160f6(unsigned long a0,long a1,int *a2) // early-return x4
{
  char v1;
  char v2;
  unsigned long v3;
  long *v4;
  int v5; // eax
  long *v6; // rax
  char v7 [6]; // stack - 0x26
  char v8 [146];
  unsigned int v9; // xmm0_da
  
  if ((!a1) || (v6 = (long)(long)*a2, v6 = (long *)(a1 + (long)v6 * 8), !*v6)) {
    v6 = NULL;
    return v6;
  }
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v6 = (unsigned long)strlen((char *)*v6);
  if (v6 != (long *)0x8) {
    v6 = NULL;
    return v6;
  }
  builtin_strncpy(v7,"aBcmt",6);
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v5 = strncmp("-newer",(char *)*v6,6);
  if (v5)
    __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
  v6 = (long)(long)*a2;
  v6 = (long *)(a1 + (long)v6 * 8);
  v1 = *(char *)(*v6 + 6);
  v6 = (long)(long)*a2;
  v6 = (long *)(a1 + (long)v6 * 8);
  v2 = *(char *)(*v6 + 7);
  if ((v1 == 'B') || (v2 == 'B')) {
    v6 = (char *)gettext("This system does not provide a way to find the birth time of a file.");
    error(0,0,(char *)v6);
    v6 = NULL;
    return v6;
  }
  if (((v1 == 't') || (v6 = (char *)strchr(v7,(int)v1), !v6)) || (v6 = (char *)strchr(v7,(int)v2), !v6)) {
    v6 = NULL;
    return v6;
  }
  v5 = *a2 + 1;
  v6 = (long)(long)v5;
  v6 = (long *)(a1 + (long)v6 * 8);
  if (!*v6) {
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v6 = (unsigned long)sub_2f861(0,dat_499ac,*v6);
    v3 = v6;
    v6 = (char *)gettext("The %s test needs an argument");
    error(1,0,(char *)v6,v3);
    return v6;
  }
  *a2 = *a2 + 1;
  v6 = (long)(long)*a2;
  v6 = (unsigned long *)(a1 + (long)v6 * 8);
  v6 = (long *)sub_eecc(a0,*v6);
  v4 = v6;
  if (v1 != 'm') { // branch-flip
    if ('m' < v1) {
label_16427:
      v6 = (char *)strchr(v7,(int)v1);
      if (!v6)
        __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
      __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
    }
    if (v1 != 'c') { // branch-flip
      if ('c' < v1) goto label_16427;
      if (v1 != 'B') { // branch-flip
        if (v1 != 'a') goto label_16427;
        *(unsigned int *)&v6[7] = 0;
      }
      else {
        *(unsigned int *)&v6[7] = 1;
      }
    }
    else {
      *(unsigned int *)&v6[7] = 2;
    }
  }
  else {
    *(unsigned int *)&v6[7] = 3;
  }
  if (v2 != 't') { // branch-flip
    sub_f26e(v8);
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v5 = (*dat_49990)(*v6,v8);
    if (v5) {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v3 = *v6;
      v6 = (int *)__errno_location();
      sub_10a44(*(int *)v6,v3); // no-return
    }
    v5 = sub_140b3(v8,(int)v2,&v4[8]);
    if (!v5) {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v6 = (unsigned long)sub_1096a(0,*v6);
      v3 = v6;
      v6 = (char *)gettext("Cannot obtain birth time of file %s");
      error(1,0,(char *)v6,v3);
      return v6;
    }
  }
  else {
    v6 = (long)(long)*a2;
    v6 = (unsigned long *)(a1 + (long)v6 * 8);
    v3 = *v6;
    if (sub_2dcb0(&v4[8],v3,0x49958) != '\x01') {
      v6 = (long)(long)*a2;
      v6 = (unsigned long *)(a1 + (long)v6 * 8);
      v6 = (unsigned long)sub_2f861(0,dat_499ac,*v6);
      v3 = v6;
      v6 = (char *)gettext("I cannot figure out how to interpret %s as a date or time");
      error(1,0,(char *)v6,v3);
      return v6;
    }
  }
  *(unsigned int *)((long)v4 + 0x3c) = 0;
  sub_144ce(v4[8]);
  *(unsigned int *)((long)v4 + 0x24) = v9;
  *a2 = *a2 + 1;
  if (!*v4)
    __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
  if ((void *)*v4 == sub_9c02) {
    if (*(char *)((long)v4 + 0x1a)) {
      v6 = (long *)0x1;
      return v6;
    }
    __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
  }
  __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
}


// Function: sub_1674d @ 0x1674d
void sub_1674d(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994c = 1;
  sub_17bae(a0,a1,a2);
}


// Function: sub_16785 @ 0x16785
unsigned long sub_16785(unsigned long a0)
{
  *(unsigned int *)(sub_eecc(a0,0) + 0x24) = dat_3c060;
  return 1;
}


// Function: sub_167ca @ 0x167ca
unsigned long sub_167ca(unsigned long a0)
{
  *(unsigned int *)(sub_ef32(a0) + 0x24) = dat_3c054;
  return 1;
}


// Function: sub_1680a @ 0x1680a
void sub_1680a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49950 = 0;
  sub_17bae(a0,a1,a2);
}


// Function: sub_16842 @ 0x16842
void sub_16842(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1893a("-ok",a0,a1,a2);
}


// Function: sub_1687a @ 0x1687a
void sub_1687a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1893a("-okdir",a0,a1,a2);
}


// Function: sub_168b2 @ 0x168b2
unsigned long sub_168b2(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e7dd(a0,0);
  *v1 = sub_9f83;
  *(unsigned int *)&v1[2] = 4;
  *(unsigned int *)((long)v1 + 0x14) = 0;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  return 1;
}


// Function: sub_16921 @ 0x16921
unsigned long sub_16921(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_e644(a0);
  *v1 = sub_9f9e;
  *(unsigned int *)&v1[2] = 3;
  *(unsigned int *)((long)v1 + 0x14) = 2;
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  return 1;
}


// Function: sub_1698b @ 0x1698b
unsigned long sub_1698b(char *a0,char a1) // return-dupe x2
{
  char *v1; // rax
  
  v1 = strrchr(a0,0x2f);
  if ((v1) && (!v1[1])) {
    if (sub_bb9d(a0,a1))
      return 1;
    return 0;
  }
  return 1;
}


// Function: sub_169f6 @ 0x169f6
unsigned long sub_169f6(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,void *a4) // return-dupe
{
  char v1; // al
  unsigned long v2; // stack - 0x30
  unsigned int v3; // xmm0_da
  long v4; // stack - 0x28
  
  sub_15686();
  if (sub_145ab(a1,a2,&v2)) {
    v4 = sub_ee5a(a0,a4,v2);
    *(char *)(v4 + 0x1b) = 0;
    *(char *)(v4 + 0x1a) = *(char *)(v4 + 0x1b);
    *(unsigned long *)(v4 + 0x38) = v2;
    v3 = sub_15576(v2,0);
    *(unsigned int *)(v4 + 0x24) = v3;
    if ((dat_49951 != '\x01') && (v1 = sub_1698b(v2,a4 == sub_986a), v1 != '\x01')) {
      error(0,0,gettext("warning: -%s %s will not match anything because it ends with /."),a3,v2);
      *(unsigned int *)(v4 + 0x24) = dat_3c064;
    }
    return 1;
  }
  return 0;
}


// Function: sub_16b4b @ 0x16b4b
void sub_16b4b(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_169f6(a0,a1,a2,"path",sub_a012);
}


// Function: sub_16b8a @ 0x16b8a
void sub_16b8a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_169f6(a0,a1,a2,"wholename",sub_a012);
}


// Function: sub_16bc9 @ 0x16bc9
void sub_16bc9(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_169f6(a0,a1,a2,"ipath",sub_986a);
}


// Function: sub_16c08 @ 0x16c08
void sub_16c08(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_169f6(a0,a1,a2,"iwholename",sub_986a);
}


// Function: sub_16c47 @ 0x16c47
unsigned long sub_16c47(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char *v1;
  long v10; // stack - 0x30
  int v11; // stack - 0x24
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // stack - 0x40
  int v5; // stack - 0x28
  unsigned int v6; // stack - 0x4c
  int v7; // stack - 0x48
  unsigned int v8; // stack - 0x44
  void *v9; // stack - 0x38
  
  if (sub_145ab(a1,a2,&v4) != '\x01')
    return 0;
  if (*v4 != '-') { // branch-flip
    if (*v4 != '/') { // branch-flip
      v7 = 0;
      v8 = 2;
      v6 = dat_3c02c;
    }
    else {
      v7 = 1;
      v8 = 1;
      v6 = dat_3c034;
    }
  }
  else {
    v7 = 1;
    v8 = 0;
    v6 = dat_3c048;
  }
  v9 = (void *)sub_25a35(&v4[v7]);
  if ((v9) && (((*v4 != '+' || (v4[1] <= '/')) || ('8' <= v4[1])))) {
    v5 = sub_25f2e(0,0,0,v9,0);
    v11 = sub_25f2e(0,1,0,v9,0);
    free(v9);
    v1 = v4;
    if (((*v4 == '/') && (!v5)) && (!v11)) {
      error(0,0,gettext("warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."),v1);
      v8 = 0;
      v6 = dat_3c068;
    }
    v10 = sub_eecc(a0,v4);
    *(unsigned int *)(v10 + 0x24) = v6;
    *(unsigned int *)(v10 + 0x38) = v8;
    memcpy((void *)(v10 + 0x3c),&v5,8);
    return 1;
  }
  v2 = sub_2f861(0,dat_499ac,v4);
  error(1,0,gettext("invalid mode %s"),v2);
  return v3;
}


// Function: sub_16eb3 @ 0x16eb3
unsigned long sub_16eb3(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x19) = 1;
  *(char *)(v1 + 0x18) = *(char *)(v1 + 0x19);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  sub_19a85(v1 + 0x38);
  return 1;
}


// Function: sub_16f20 @ 0x16f20
void sub_16f20(unsigned long a0)
{
  sub_14fd8(a0,0);
}


// Function: sub_16f4b @ 0x16f4b
unsigned long sub_16f4b(unsigned long a0,unsigned long a1,unsigned int *a2) // return-dupe x2
{
  unsigned int v1;
  unsigned long v2; // stack - 0x40
  char v3 [40];
  
  v1 = *a2;
  if (!sub_14528(a1,a2,&v2))
    return 0;
  sub_19a85(v3);
  if (sub_1159f(v3,a0,v2))
    return 1;
  *a2 = v1;
  return 0;
}


// Function: sub_16fef @ 0x16fef
unsigned long sub_16fef(unsigned long a0,unsigned long a1,unsigned int *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  char v3 [40];
  unsigned int v4; // stack - 0x4c
  
  v4 = *a2;
  if ((sub_145ab(a1,a2,&v1)) && (sub_14528(a1,a2,&v2))) {
    sub_19964(v1,v3);
    v4 = *a2;
    if (sub_1159f(v3,a0,v2))
      return 1;
  }
  *a2 = v4;
  return 0;
}


// Function: sub_170b7 @ 0x170b7
unsigned long sub_170b7(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  if (dat_49940 != '\x01') {
    *(char *)(v1 + 0x1b) = 0;
    *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  }
  *(char *)(v1 + 0x18) = 1;
  *(char *)(v1 + 0x19) = 0;
  return 1;
}


// Function: sub_1711b @ 0x1711b
unsigned long sub_1711b(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(char *)(v1 + 0x18) = 1;
  *(char *)(v1 + 0x19) = 0;
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_17182 @ 0x17182
unsigned long sub_17182(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (sub_145ab(a1,a2,&v2)) {
    dat_4999c = sub_1ca8f(v2);
    v1 = sub_17bae(a0,a1,a2);
    return v1;
  }
  return 0;
}


// Function: sub_1720a @ 0x1720a
void sub_1720a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1723e(a1,a2,a0,dat_4999c);
}


// Function: sub_1723e @ 0x1723e
unsigned long sub_1723e(unsigned long a0,unsigned long a1,unsigned long a2,int a3) // return-dupe
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // stack - 0x40
  unsigned int v5; // xmm0_da
  long v6; // stack - 0x38
  unsigned long *v7; // stack - 0x30
  long v8; // stack - 0x28
  
  if (!sub_145ab(a0,a1,&v4))
    return 0;
  v6 = sub_ee5a(a2,sub_a2da,v4);
  *(char *)(v6 + 0x1b) = 0;
  *(char *)(v6 + 0x1a) = *(char *)(v6 + 0x1b);
  v7 = (unsigned long *)sub_31bc2(0x40);
  *(unsigned long **)(v6 + 0x38) = v7;
  v7[1] = 100;
  v1 = v7[1];
  *v7 = sub_31bc2(v1);
  v7[4] = 0;
  re_set_syntax((long)a3);
  v7[3] = (long)a3;
  v7[5] = 0;
  v2 = strlen(v4);
  v8 = re_compile_pattern(v4,v2,v7);
  if (!v8) {
    sub_15576(v4,1);
    *(unsigned int *)(v6 + 0x24) = v5;
    return 1;
  }
  error(1,0,gettext("failed to compile regular expression \'%s\': %s"),v4,v8);
  return v3;
}


// Function: sub_173d3 @ 0x173d3
unsigned long sub_173d3(unsigned long a0,long a1,int *a2) // return-dupe
{
  char v1;
  undefined4 v10;
  char v11; // stack - 0x4d
  int v12; // stack - 0x48
  int v13; // stack - 0x44
  char *v14; // stack - 0x38
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  long v6; // rax
  int v7; // stack - 0x4c
  long v8; // stack - 0x40
  char v9 [2]; // stack - 0x22
  
  v12 = 0x200;
  if ((!a1) || (!*(long *)(a1 + (long)*a2 * 8)))
    return 0;
  v14 = *(char **)(a1 + (long)*a2 * 8);
  v2 = strlen(v14);
  v13 = (int)v2;
  if (!v13) {
    error(1,0,gettext("invalid null argument to -size"));
    return v3;
  }
  v11 = v14[(long)v13 + -1];
  if (v11 != 'w') { // branch-flip
    if ('w' < v11) {
label_175d2:
      v1 = *(char *)((long)v13 + -1 + *(long *)(a1 + (long)*a2 * 8));
      error(1,0,gettext("invalid -size type `%c\'"),(unsigned long)(unsigned int)(int)v1);
      return v4;
    }
    if (v11 != 'k') { // branch-flip
      if ('k' < v11) goto label_175d2;
      if (v11 != 'c') { // branch-flip
        if ('c' < v11) goto label_175d2;
        if (v11 != 'b') { // branch-flip
          if ('b' < v11) goto label_175d2;
          if (v11 != 'M') { // branch-flip
            if ('M' < v11) goto label_175d2;
            if (':' <= v11) { // branch-flip
              if (v11 != 'G') goto label_175d2;
              v12 = 0x40000000;
              v14[(long)v13 + -1] = '\0';
            }
            else {
              if (v11 <= '/') goto label_175d2;
              v11 = '\0';
            }
          }
          else {
            v12 = 0x100000;
            v14[(long)v13 + -1] = '\0';
          }
        }
        else {
          v12 = 0x200;
          v14[(long)v13 + -1] = '\0';
        }
      }
      else {
        v12 = 1;
        v14[(long)v13 + -1] = '\0';
      }
    }
    else {
      v12 = 0x400;
      v14[(long)v13 + -1] = '\0';
    }
  }
  else {
    v12 = 2;
    v14[(long)v13 + -1] = '\0';
  }
  v1 = sub_19722(v14,&v8,&v7);
  if (v1 == '\x01') {
    v6 = sub_eecc(a0,v14);
    *(int *)(v6 + 0x38) = v7;
    *(int *)(v6 + 0x3c) = v12;
    *(long *)(v6 + 0x40) = v8;
    *(char *)(v6 + 0x1a) = 1;
    *(char *)(v6 + 0x1b) = 0;
    if (v7) { // branch-flip
      if (v7 != 1) // branch-flip
        *(unsigned int *)(v6 + 0x24) = dat_3c02c;
      else {
        if (0x5001 <= (unsigned long)(v8 * v12)) // branch-flip
          v10 = s_3c06b._1_4_;
        else {
          v10 = dat_3c028;
        }
        *(unsigned int *)(v6 + 0x24) = v10;
      }
    }
    else {
      if (0x5001 <= (unsigned long)(v8 * v12)) // branch-flip
        v10 = dat_3c028;
      else {
        v10 = s_3c06b._1_4_;
      }
      *(unsigned int *)(v6 + 0x24) = v10;
    }
    *a2 = *a2 + 1;
    return 1;
  }
  v9[0] = v11;
  v9[1] = 0;
  error(1,0,gettext("Invalid argument `%s%s\' to -size"),v14,v9);
  return v5;
}


// Function: sub_1777e @ 0x1777e
unsigned long sub_1777e(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe, ternary
{
  int v1; // eax
  unsigned long v10; // stack - 0x130
  unsigned long v11; // stack - 0x128
  unsigned long v12; // stack - 0x120
  unsigned long v13; // stack - 0x118
  unsigned long v14; // stack - 0x110
  unsigned long v15; // stack - 0x108
  unsigned long v16; // stack - 0x100
  unsigned long v17; // stack - 0xf8
  unsigned long v18; // stack - 0xf0
  unsigned long v19; // stack - 0xe8
  long v2; // rax
  unsigned long v20; // stack - 0xe0
  unsigned long v21; // stack - 0xd8
  unsigned long v22; // stack - 0xd0
  unsigned long v23; // stack - 0xc8
  unsigned long v24; // stack - 0xc0
  long v25; // stack - 0xb0
  unsigned long v26; // stack - 0xa8
  unsigned long v27; // stack - 0xa0
  unsigned long v28; // stack - 0x98
  unsigned long v29; // stack - 0x90
  long v3; // stack - 0x148
  unsigned long v30; // stack - 0x88
  unsigned long v31; // stack - 0x80
  unsigned long v32; // stack - 0x78
  unsigned long v33; // stack - 0x70
  unsigned long v34; // stack - 0x68
  unsigned long v35; // stack - 0x60
  unsigned long v36; // stack - 0x58
  unsigned long v37; // stack - 0x50
  unsigned long v38; // stack - 0x48
  unsigned long v39; // stack - 0x40
  unsigned long v4 [2]; // stack - 0x158
  unsigned long v40; // stack - 0x38
  unsigned long v41; // stack - 0x30
  long v5; // stack - 0xb8
  int v6; // stack - 0x160
  unsigned int v7; // stack - 0x15c
  long v8; // stack - 0x140
  unsigned long v9; // stack - 0x138
  
  sub_f26e(&v3);
  if (sub_14611(a1,a2,&v3,v4) != '\x01')
    return 0;
  sub_f26e(&v5);
  v7 = 0;
  if (dat_49988) // branch-flip
    v6 = -1;
  else if (dat_49998) { // branch-flip
    v7 = 0x20000;
    v6 = -1;
  }
  else {
    v6 = (((unsigned int)v10 & 0xf000) != 0xa000) ? -1 : -2; // branch-flip
  }
  if (v6 == -3)
    __assert_fail("fd != -3","parser.c",0x8bf,"parse_samefile"); // no-return
  if (v6 == -1) {
    v6 = sub_1b433(v4[0],v7);
    if (0 <= v6) {
      if (fstat(v6,&v5))
        sub_10a44(*__errno_location(),v4[0]); // no-return
      v1 = (*dat_49990)(v4[0],&v3);
      if (v1)
        sub_10a44(*__errno_location(),v4[0]); // no-return
      if ((dat_49988) || (dat_49998 == '\x01')) {
        v3 = v5;
        v8 = v25;
        v9 = v26;
        v10 = v27;
        v11 = v28;
        v12 = v29;
        v13 = v30;
        v14 = v31;
        v15 = v32;
        v16 = v33;
        v17 = v34;
        v18 = v35;
        v19 = v36;
        v20 = v37;
        v21 = v38;
        v22 = v39;
        v23 = v40;
        v24 = v41;
      }
      else if (((unsigned int)v10 & 0xf000) != 0xa000) { // branch-flip
        if ((v3 != v5) || (v8 != v25)) {
          close(v6);
          v6 = -1;
        }
      }
      else {
        close(v6);
        v6 = -1;
      }
    }
  }
  v2 = sub_eecc(a0,v4[0]);
  *(long *)(v2 + 0x38) = v8;
  *(long *)(v2 + 0x40) = v3;
  *(int *)(v2 + 0x48) = v6;
  *(char *)(v2 + 0x1b) = 0;
  *(char *)(v2 + 0x1a) = 1;
  *(unsigned int *)(v2 + 0x24) = dat_3c02c;
  return 1;
}


// Function: sub_17b57 @ 0x17b57
unsigned long sub_17b57(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_ef32(a0);
  *(char *)(v1 + 0x1b) = 0;
  *(char *)(v1 + 0x1a) = *(char *)(v1 + 0x1b);
  *(unsigned int *)(v1 + 0x24) = dat_3c030;
  return 1;
}


// Function: sub_17bae @ 0x17bae
void sub_17bae(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_17b57(sub_14033(),a1,a2);
}


// Function: sub_17be3 @ 0x17be3
unsigned long sub_17be3(unsigned long a0) // return-dupe
{
  long *v1; // rax
  
  v1 = (long *)sub_ef32(a0);
  *(char *)((long)v1 + 0x1b) = 0;
  *(char *)((long)v1 + 0x1a) = *(char *)((long)v1 + 0x1b);
  *(char *)((long)v1 + 0x19) = 0;
  *(char *)&v1[3] = *(char *)((long)v1 + 0x19);
  if ((void *)*v1 == sub_a0fa) {
    *(unsigned int *)((long)v1 + 0x24) = dat_3c048;
    return 1;
  }
  *(unsigned int *)((long)v1 + 0x24) = s_3c06b._1_4_;
  return 1;
}


// Function: sub_17c77 @ 0x17c77
void sub_17c77(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_183bf(a1,a2,a0,sub_a4b2);
}


// Function: sub_17cac @ 0x17cac
unsigned long sub_17cac(unsigned long a0,unsigned long a1,int *a2) // return-dupe
{
  long v1; // rax
  unsigned int v2;
  
  v1 = sub_197bf(a1,a2,a0);
  if (v1) {
    if (100 <= *(unsigned long *)(v1 + 0x40)) // branch-flip
      v2 = dat_3c048;
    else {
      v2 = dat_3c044;
    }
    *(unsigned int *)(v1 + 0x24) = v2;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}


// Function: sub_17d2c @ 0x17d2c
char * sub_17d2c(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char v1; // al
  unsigned long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  char *v2; // rax
  unsigned long v3; // stack - 0x60
  unsigned long v4; // stack - 0x38
  unsigned int v5; // xmm0_da
  char *v6; // stack - 0x58
  long v7; // stack - 0x50
  unsigned long v8; // stack - 0x48
  unsigned long v9; // stack - 0x40
  
  v6 = "arithmetic overflow while converting %s days to a number of seconds";
  v1 = sub_145ab(a1,a2,&v3);
  if (!v1) {
    v2 = NULL;
    return v2;
  }
  v8 = 0;
  v9 = 0;
  v1 = sub_1916a(v3,&v4,0,0,v6);
  if (v1) {
    v2 = (long)sub_eecc(a0,v3);
    v7 = (long)v2;
    *(unsigned long *)((long)v2 + 0x38) = v4;
    *(unsigned long *)((long)v2 + 0x40) = v10;
    *(unsigned long *)((long)v2 + 0x48) = v11;
    sub_1446f();
    *(unsigned int *)(v7 + 0x24) = v5;
    v2 = (char *)0x1;
    return v2;
  }
  v2 = gettext("Invalid argument %s to -used");
  error(1,0,v2,v3);
  return v2;
}


// Function: sub_17e8d @ 0x17e8d
unsigned long sub_17e8d(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rax
  char *v5; // stack - 0x40
  unsigned int v6;
  unsigned int v7; // stack - 0x44
  void *v8; // stack - 0x38
  unsigned long v9; // stack - 0x30
  
  if (!sub_145ab(a1,a2,&v5))
    return 0;
  v8 = getpwnam(v5);
  endpwent();
  if (v8) // branch-flip
    v7 = *(unsigned int *)((long)v8 + 0x10);
  else {
    v9 = strspn(v5,"0123456789");
    if ((!v9) || (v5[v9])) {
      if (*v5) {
        v1 = sub_2f861(0,dat_499ac,v5);
        error(1,0,gettext("%s is not the name of a known user"),v1);
        return v2;
      }
      error(1,0,gettext("The argument to -user should not be empty"));
      return v3;
    }
    v7 = sub_1cd12(v5,dat_499ac);
  }
  v4 = sub_eecc(a0,v5);
  *(unsigned int *)(v4 + 0x38) = v7;
  if (100 <= *(unsigned int *)(v4 + 0x38)) // branch-flip
    v6 = dat_3c048;
  else {
    v6 = dat_3c044;
  }
  *(unsigned int *)(v4 + 0x24) = v6;
  return 1;
}


// Function: sub_1802c @ 0x1802c
void sub_1802c(void) // ternary
{
  char *v1; // rax
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_1b6b6("find");
  printf(gettext("Features enabled: "));
  printf("D_TYPE ");
  v1 = (dat_49998) ? "enabled" : "disabled"; // branch-flip
  printf("O_NOFOLLOW(%s) ",v1);
  printf("LEAF_OPTIMISATION ");
  if (1 <= (int)is_selinux_enabled())
    printf("SELINUX ");
  v2 = 0;
  if (sub_8b12(&v2)) {
    printf("FTS(");
    if (v2 & 0x200)
      printf("FTS_CWDFD");
    printf(") ");
  }
  printf("CBO(level=%d) ",(unsigned long)dat_499a8);
  printf("\n");
  exit(0); // no-return
}


// Function: sub_181cd @ 0x181cd
long * sub_181cd(unsigned long a0,long a1,int *a2) // return-dupe
{
  long v1;
  long *v2; // rax
  
  if ((a1) && (v2 = (long)(long)*a2, v2 = (long *)(a1 + (long)v2 * 8), *v2)) {
    if (1 <= (int)is_selinux_enabled()) {
      v2 = (long)sub_eecc(a0,0);
      v1 = (long)v2;
      *(unsigned int *)((long)v2 + 0x24) = dat_3c02c;
      *(char *)((long)v2 + 0x1a) = 0;
      v2 = (long)(long)*a2;
      v2 = (unsigned long *)(a1 + (long)v2 * 8);
      *(long *)(v1 + 0x38) = *v2;
      *a2 = *a2 + 1;
      v2 = (long *)0x1;
      return v2;
    }
    v2 = (char *)gettext("invalid predicate -context: SELinux is not enabled.");
    error(1,0,(char *)v2);
    return v2;
  }
  v2 = NULL;
  return v2;
}


// Function: sub_182aa @ 0x182aa
void sub_182aa(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994d = 1;
  sub_17bae(a0,a1,a2);
}


// Function: sub_182e2 @ 0x182e2
void sub_182e2(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994e = 1;
  sub_17bae(a0,a1,a2);
}


// Function: sub_1831a @ 0x1831a
void sub_1831a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_4994e = 0;
  sub_17bae(a0,a1,a2);
}


// Function: sub_18352 @ 0x18352
void sub_18352(unsigned long a0,unsigned long a1,unsigned long a2)
{
  dat_49950 = 1;
  sub_17bae(a0,a1,a2);
}


// Function: sub_1838a @ 0x1838a
void sub_1838a(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_183bf(a1,a2,a0,sub_a78e);
}


// Function: sub_183bf @ 0x183bf
char * sub_183bf(unsigned long a0,unsigned long a1,unsigned long a2,void *a3) // return-dupe, ternary
{
  char v1;
  char *v2;
  char *v3; // rax
  char *v4; // stack - 0x38
  unsigned int v5; // stack - 0x40
  float v6; // stack - 0x3c
  char *v7; // stack - 0x30
  long v8; // stack - 0x28
  
  v3 = (a3 != sub_a78e) ? "-type" : "-xtype"; // branch-flip
  v7 = v3;
  if (sub_145ab(a0,a1,&v4) != '\x01') {
    v3 = NULL;
    return v3;
  }
  if (!*v4) {
    v3 = gettext("Arguments to %s should contain at least one letter");
    error(1,0,v3,v7);
    return v3;
  }
  v3 = (long)sub_ee5a(a2,a3,v4);
  v8 = (long)v3;
  *(unsigned int *)((long)v3 + 0x24) = 0;
  if (a3 != sub_a78e) { // branch-flip
    *(char *)((long)v3 + 0x1a) = 0;
    *(char *)((long)v3 + 0x1b) = 1;
  }
  else {
    *(char *)((long)v3 + 0x1a) = 1;
    *(char *)((long)v3 + 0x1b) = 0;
  }
  do {
    do {
      v2 = v4;
      if (!*v4) {
        v3 = (char *)0x1;
        return v3;
      }
      switch(*v4) {
        case 0x44:
          v1 = *v4;
          v3 = gettext("%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.");
          error(1,0,v3,v7,(unsigned long)(unsigned int)(int)v1);
          v1 = *v4; // return-dupe
          v3 = gettext("Unknown argument to %s: %c");
          error(1,0,v3,v7,(unsigned long)(unsigned int)(int)v1);
          return v3;
        default:
          v1 = *v4;
          v3 = gettext("Unknown argument to %s: %c");
          error(1,0,v3,v7,(unsigned long)(unsigned int)(int)v1);
          return v3;
        case 0x62:
          v5 = 0;
          v6 = dat_3c078;
          break;
        case 99:
          v5 = 1;
          v6 = dat_3c07c;
          break;
        case 100:
          v5 = 2;
          v6 = dat_3c080;
          break;
        case 0x66:
          v5 = 3;
          v6 = dat_3c084;
          break;
        case 0x6c:
          v5 = 4;
          v6 = dat_3c088;
          break;
        case 0x70:
          v5 = 5;
          v6 = dat_3c08c;
          break;
        case 0x73:
          v5 = 6;
          v6 = dat_3c090;
        
      }
      v3 = (unsigned long)(unsigned long)v5;
      if (*(char *)(v8 + 0x38 + (long)v3)) {
        v1 = *v4;
        v3 = gettext("Duplicate file type \'%c\' in the argument list to %s.");
        error(1,0,v3,(unsigned long)(unsigned int)(int)v1,v7);
        return v3;
      }
      *(float *)(v8 + 0x24) = *(float *)(v8 + 0x24) + v6;
      v3 = (unsigned long)(unsigned long)v5;
      *(char *)(v8 + 0x38 + (long)v3) = 1;
      v3 = &v4[1];
      v4 = v3;
    } while (!*v3);
    if (*v3 != ',') {
      v3 = gettext("Must separate multiple arguments to %s using: \',\'");
      error(1,0,v3,v7);
      return v3;
    }
    v3 = &v2[2];
    v4 = v3;
  } while (*v3);
  v3 = gettext("Last file type in list argument to %s is missing, i.e., list is ending on: \',\'");
  error(1,0,v3,v7);
  return v3;
}


// Function: sub_1876f @ 0x1876f
unsigned long sub_1876f(void *a0) // early-return
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (v1 == -1)
    return 0;
  v1 = isatty(v1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 != 0);
}


// Function: sub_187ac @ 0x187ac
void sub_187ac(unsigned long a0) // return-dupe
{
  char v1; // al
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  long v5; // stack - 0x48
  char *v6; // stack - 0x38
  char *v7; // stack - 0x30
  char *v8; // stack - 0x28
  
  v6 = getenv("PATH");
  v7 = ":";
  if (!v6)
    return;
  sub_1cf56(v6,":",1,&v5,&v4);
  do {
    if ((!v4) || ((v4 == 1 && (v6[v5] == '.')))) {
      error(1,0,gettext("The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"),a0);
      return;
    }
    if (v6[v5] != '/') {
      v2 = strndup(&v6[v5],v4);
      v8 = v2;
      if (!v2)
        v2 = &v6[v5];
      v3 = sub_1096a(0,v2);
      error(1,0,gettext("The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"),v3,a0);
      return;
    }
    v1 = sub_1cf56(v6,v7,0,&v5,&v4);
  } while (v1);
}


// Function: sub_1893a @ 0x1893a
unsigned long sub_1893a(unsigned long a0,long a1,long a2,int *a3) // early-return x2, ternary
{
  int v1;
  unsigned long v10;
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14; // rax
  char v15 [24];
  int v16; // stack - 0x90
  int v17; // stack - 0x8c
  int v18; // stack - 0x84
  char *v19; // stack - 0x78
  void *v2;
  char *v20; // stack - 0x70
  char *v3;
  bool v4;
  bool v5;
  int v6;
  void *v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v2 = *(void **)(a1 + 0x18);
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v7 = (void *)sub_ee5a(a1,v2,"(some -exec* arguments)");
  *(char *)((long)v7 + 0x19) = 1;
  *(char *)((long)v7 + 0x18) = *(char *)((long)v7 + 0x19);
  *(char *)((long)v7 + 0x1a) = 0;
  *(char *)((long)v7 + 0x1b) = *(char *)((long)v7 + 0x1a);
  *(unsigned long *)((long)v7 + 0xf8) = 0;
  if ((v2 != sub_9f27) && (v2 != sub_9ece)) {
    v4 = 1;
    *(char *)((long)v7 + 0xf4) = 0;
  }
  else {
    v4 = 0;
    dat_499b8 = 1;
    *(char *)((long)v7 + 0xf4) = 1;
  }
  if ((v2 != sub_9497) && (v2 != sub_9f27)) {
    if (!dat_49a00)
      __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
    *(long *)((long)v7 + 0xf8) = dat_49a00;
  }
  else {
    *(unsigned long *)((long)v7 + 0xf8) = 0;
    dat_4994e = 0;
    sub_187ac(a0);
  }
  *(char *)((long)v7 + 0x38) = 0;
  v1 = *a3;
  v5 = 0;
  v18 = 0;
  v19 = NULL;
  v16 = v1;
  while( true ) {
    if ((!*(long *)(a2 + (long)v16 * 8)) || ((**(char **)(a2 + (long)v16 * 8) == ';' && (!*(char *)(*(long *)(a2 + (long)v16 * 8) + 1))))) goto label_18cc6;
    if ((v4) && (((**(char **)(a2 + (long)v16 * 8) == '+' && (!*(char *)(*(long *)(a2 + (long)v16 * 8) + 1))) && (v5)))) break;
    v5 = 0;
    v10 = *(unsigned long *)(a2 + (long)v16 * 8);
    if (sub_24cc3(v10,"{}")) {
      v5 = 1;
      v19 = *(char **)(a2 + (long)v16 * 8);
      v18 += 1;
      if ((!v16) && ((v2 == sub_9497 || (v2 == sub_9f27)))) {
        error(1,0,gettext("You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."));
        return v8;
      }
    }
    v16 += 1;
  }
  *(char *)((long)v7 + 0x38) = 1;
label_18cc6:
  if ((v16 == v1) || (!*(long *)(a2 + (long)v16 * 8))) {
    *a3 = v16;
    free(v7);
    return 0;
  }
  if (*(char *)((long)v7 + 0x38)) {
    v20 = (v2 != sub_9497) ? "" : "dir"; // branch-flip
    if (2 <= v18) {
      error(1,0,gettext("Only one instance of {} is supported with -exec%s ... +"),v20);
      return v9;
    }
    if (strlen(v19) != 2) {
      if (0x14 <= (unsigned int)snprintf(v15,0x13,"-exec%s ... {} +",v20))
        __assert_fail("needed <= MsgBufSize","parser.c",0xbc6,"insert_exec_ok"); // no-return
      v10 = sub_2f861(2,dat_499ac,v19);
      v11 = sub_2f861(1,dat_499ac,"{}");
      v12 = sub_2f861(0,dat_499ac,v15);
      error(1,0,gettext("In %s the %s must appear by itself, but you specified %s"),v12,v11,v10);
      return v13;
    }
  }
  v6 = sub_1a58c((long)v7 + 0x40,0x800);
  if ((v6) && ((unsigned int)(v6 - 1U) <= 1)) {
    error(1,0,gettext("The environment is too large for exec()."));
    return v14;
  }
  sub_1a6e3((long)v7 + 0x40);
  *(void **)((long)v7 + 0x80) = sub_b7ae;
  if (*(char *)((long)v7 + 0x38)) { // branch-flip
    *(unsigned long *)((long)v7 + 0xe8) = 0;
    *(unsigned long *)((long)v7 + 0x70) = 0;
    *(unsigned long *)((long)v7 + 0x68) = 0;
    *(unsigned long *)((long)v7 + 0x88) = 0;
    *(unsigned long *)((long)v7 + 0x90) = 0;
    *(long *)((long)v7 + 0x78) = (long)((v16 - v1) + -1);
    sub_1a742((long)v7 + 0x40,(long)v7 + 0x98,(long)v7 + 0x38);
    for (v17 = v1; v17 < v16 + -1; v17 = v17 + 1) {
      v3 = *(char **)(a2 + (long)v17 * 8);
      sub_1a1b8((long)v7 + 0x40,(long)v7 + 0x98,*(unsigned long *)(a2 + (long)v17 * 8),strlen(v3) + 1,0,0,1);
    }
  }
  else {
    *(int *)((long)v7 + 0xf0) = v16 - v1;
    *(char **)((long)v7 + 0x70) = "{}";
    v3 = *(char **)((long)v7 + 0x70);
    *(unsigned long *)((long)v7 + 0x68) = strlen(v3);
    *(unsigned long *)((long)v7 + 0x88) = 0;
    *(unsigned long *)((long)v7 + 0x90) = 0;
    v6 = *(int *)((long)v7 + 0xf0);
    *(unsigned long *)((long)v7 + 0xe8) = sub_31bc2((long)v6 << 3);
    sub_1a742((long)v7 + 0x40,(long)v7 + 0x98,(long)v7 + 0x38);
    for (v17 = 0; v17 < *(int *)((long)v7 + 0xf0); v17 = v17 + 1) {
      *(unsigned long *)(*(long *)((long)v7 + 0xe8) + (long)v17 * 8) = *(unsigned long *)((long)(v1 + v17) * 8 + a2);
    }
  }
  if (*(long *)(a2 + (long)v16 * 8)) // branch-flip
    *a3 = v16 + 1;
  else {
    *a3 = v16;
  }
  return 1;
}


// Function: sub_1916a @ 0x1916a
unsigned long sub_1916a(unsigned long a0,long a1,long a2,long a3,char *a4) // return-dupe x2, return-dupe
{
  unsigned int v1;
  char v2; // al
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x30
  char v5 [8];
  double v6; // stack - 0x20
  double v7; // xmm0_qa
  double v8; // stack - 0x18
  
  v4 = a0;
  v2 = sub_196ab(&v4,a1 + 4);
  if (!v2) {
    v3 = 0;
    return v3;
  }
  v1 = *(unsigned int *)(a1 + 4);
  if ((v1 != 2) && (v1 <= 2)) {
    if (v1) { // branch-flip
      if (v1 == 1)
        *(unsigned int *)(a1 + 4) = 0;
    }
    else {
      *(unsigned int *)(a1 + 4) = 1;
    }
  }
  v2 = sub_3223a(v4,0,v5,dat_48fe8);
  if (!v2) {
    v3 = 0;
    return v3;
  }
  modf(&v6);
  v8 = dat_3c098 * v7;
  if ((double)dat_3c000 <= v8)
    __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
  *(long *)(a1 + 8) = (long)((double)a2 - v6);
  v2 = v6 < 0.0 != a2 < *(long *)(a1 + 8);
  if ((bool)v2) {
    error(1,0,a4,v4);
    return v3;
  }
  *(long *)(a1 + 0x10) = (long)((double)a3 - v8);
  if (v8 <= (double)a3) {
    v3 = 1;
    return v3;
  }
  *(long *)(a1 + 0x10) = *(long *)(a1 + 0x10) + dat_3c000;
  *(long *)(a1 + 8) = *(long *)(a1 + 8) + -1;
  v3 = 1;
  return v3;
}


// Function: sub_1938f @ 0x1938f
char * sub_1938f(unsigned long a0,unsigned long a1,unsigned int *a2) // return-dupe x2, ternary x2
{
  char v1; // al
  char *v10; // stack - 0x58
  unsigned long v11; // stack - 0x50
  long v12; // stack - 0x48
  long v13; // stack - 0x40
  long v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  int v16; // stack - 0x24
  unsigned long v17; // stack - 0x20
  unsigned long v18; // stack - 0x18
  char *v2; // rax
  char *v3; // rdx
  unsigned long v4; // stack - 0x68
  int v5; // stack - 0x70
  unsigned int v6; // stack - 0x28
  long v7; // stack - 0x60
  unsigned int v8; // xmm0_da
  unsigned int v9; // stack - 0x6c
  
  v2 = gettext("arithmetic overflow while converting %s days to a number of seconds");
  v10 = v2;
  v9 = *a2;
  v1 = sub_145ab(a1,a2,&v4);
  if (v1 != '\x01') {
    v2 = NULL;
    return v2;
  }
  v11 = v4;
  v14 = dat_49968;
  v15 = dat_49970;
  v1 = sub_196ab(&v4,&v5);
  if ((v1) && (v5 == 1)) {
    v2 = (long)(v14 + 0x1517f);
    v12 = (long)v2;
    v2 = (long)(v14 + 0x1517f);
    v14 = (long)v2;
  }
  v4 = v11;
  v1 = sub_1916a(v11,&v6,v14,v15,v10);
  if (v1 != '\x01') {
    *a2 = v9;
    v2 = NULL;
    return v2;
  }
  v2 = (long)sub_eecc(a0,v11);
  v13 = (long)v2;
  *(unsigned long *)((long)v2 + 0x38) = CONCAT44(v16,v6);
  *(unsigned long *)((long)v2 + 0x40) = v17;
  *(unsigned long *)((long)v2 + 0x48) = v18;
  sub_144ce(v17);
  *(unsigned int *)(v13 + 0x24) = v8;
  if (!(dat_49980 & 1)) {
    v2 = (char *)0x1;
    return v2;
  }
  fprintf(stderr,"inserting %s\n",*(unsigned long *)(v13 + 8));
  if (v16) { // branch-flip
    if (v16 != 1) { // branch-flip
      v3 = (v16 != 2) ? " ?" : ">="; // branch-flip
    }
    else {
      v3 = " <";
    }
  }
  else {
    v3 = " >";
  }
  if (v16) { // branch-flip
    if (v16 != 1) { // branch-flip
      v2 = (v16 != 2) ? "?" : "eq"; // branch-flip
    }
    else {
      v2 = "lt";
    }
  }
  else {
    v2 = "gt";
  }
  fprintf(stderr,"    type: %s    %s  ",v2,v3);
  v7 = *(long *)(v13 + 0x40);
  v2 = (unsigned long)ctime(&v7);
  fprintf(stderr,"%lu %s",*(unsigned long *)(v13 + 0x40),v2);
  if (v16 != 2) {
    v2 = (char *)0x1;
    return v2;
  }
  v2 = (long)(*(long *)(v13 + 0x40) + 0x15180);
  v7 = (long)v2;
  v2 = (unsigned long)ctime(&v7);
  fprintf(stderr,"                 <  %lu %s",v7,v2);
  v2 = (char *)0x1;
  return v2;
}


// Function: sub_196ab @ 0x196ab
unsigned long sub_196ab(long *a0,unsigned int *a1) // return-dupe x2
{
  if (*(char *)*a0 == '+') {
    *a1 = 0;
    *a0 = *a0 + 1;
    return 1;
  }
  if (*(char *)*a0 != '-') {
    *a1 = 2;
    return 1;
  }
  *a1 = 1;
  *a0 = *a0 + 1;
  return 1;
}


// Function: sub_19722 @ 0x19722
unsigned long sub_19722(long a0,unsigned long a1,long a2) // early-return x2
{
  unsigned long v1; // rax
  long v2; // stack - 0x20
  char v3 [8];
  
  if (!a0)
    return 0;
  v2 = a0;
  if ((a2) && (sub_196ab(&v2,a2) != '\x01'))
    return 0;
  v1 = sub_3291e(v2,v3,10,a1,0x3bdf6);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}


// Function: sub_197bf @ 0x197bf
char * sub_197bf(unsigned long a0,unsigned long a1,unsigned long a2) // early-return, ternary x2
{
  char v1; // al
  char *v2; // rax
  char *v3; // rdx
  unsigned long v4; // stack - 0x28
  int v5; // stack - 0x2c
  unsigned long v6; // stack - 0x20
  long v7; // stack - 0x18
  
  v1 = sub_145ab(a0,a1,&v4);
  if ((v1) && (v1 = sub_19722(v4,&v6,&v5), v1)) {
    v2 = (long)sub_eecc(a2,v4);
    v7 = (long)v2;
    *(int *)((long)v2 + 0x38) = v5;
    *(unsigned long *)((long)v2 + 0x40) = v6;
    if (dat_49980 & 1) {
      fprintf(stderr,"inserting %s\n",*(unsigned long *)((long)v2 + 8));
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          v3 = (v5 != 2) ? " ?" : " ="; // branch-flip
        }
        else {
          v3 = " <";
        }
      }
      else {
        v3 = " >";
      }
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          v2 = (v5 != 2) ? "?" : "eq"; // branch-flip
        }
        else {
          v2 = "lt";
        }
      }
      else {
        v2 = "gt";
      }
      fprintf(stderr,"    type: %s    %s  ",v2,v3);
      fprintf(stderr,"%lu\n",*(unsigned long *)(v7 + 0x40));
    }
    v2 = (char *)v7;
    return v2;
  }
  v2 = NULL;
  return v2;
}


// Function: sub_19964 @ 0x19964
void sub_19964(char *a0,unsigned long *a1)
{
  unsigned long v1;
  
  *a1 = 0;
  a1[4] = sub_2de20(0);
  if (strcmp(a0,"/dev/stderr")) { // branch-flip
    if (strcmp(a0,"/dev/stdout")) { // branch-flip
      v1 = sub_10ce6(dat_499f0,a0);
      a1[1] = v1;
      a1[2] = a0;
      if (!a1[1])
        sub_10aa8(*__errno_location(),a0); // no-return
    }
    else {
      a1[1] = stdout;
      a1[2] = gettext("standard output");
    }
  }
  else {
    a1[1] = stderr;
    a1[2] = gettext("standard error");
  }
  v1 = a1[1];
  *(char *)&a1[3] = sub_1876f(v1);
}


// Function: sub_19a85 @ 0x19a85
void sub_19a85(unsigned long a0)
{
  sub_19964("/dev/stdout",a0);
}

