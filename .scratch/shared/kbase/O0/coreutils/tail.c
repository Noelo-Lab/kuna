// Function: sub_4115 @ 0x4115
void sub_4115(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_18150;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n"),v2);
    printf(gettext("Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"),10);
    sub_2a69();
    sub_2a9a();
    v1 = stdout;
    fputs_unlocked(gettext("  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n"),v1);
    printf(gettext("  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"),10,5);
    v1 = stdout;
    fputs_unlocked(gettext("      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n"),v1);
    sub_2acb("tail");
  }
  exit(a0); // no-return
}


// Function: sub_431c @ 0x431c
void sub_431c(void)
{
  raise(0xd);
  exit(1); // no-return
}


// Function: sub_4338 @ 0x4338
void sub_4338(void)
{
  unsigned int v1; // stack - 0x18
  unsigned short v2; // stack - 0x14
  unsigned short v3; // stack - 0x12
  
  if (dat_1812f == '\x01') {
    v1 = 1;
    v3 = 0;
    v2 = 0;
    if ((0 <= (int)poll(&v1,1,0)) && (v3 & 0x18))
      sub_431c(); // no-return
  }
}


// Function: sub_43ba @ 0x43ba
bool sub_43ba(long a0)
{
  return (*(int *)(a0 + 0x3c) == 0) != (*(int *)(a0 + 0x38) == -1);
}


// Function: sub_43ee @ 0x43ee
char * sub_43ee(unsigned long *a0)
{
  char *v1;
  char *v2; // rax
  
  v1 = (char *)*a0;
  if (strcmp(v1,"-")) // branch-flip
    v2 = (char *)*a0;
  else {
    v2 = gettext("standard input");
  }
  return v2;
}


// Function: sub_4435 @ 0x4435
void sub_4435(long a0,unsigned int a1,unsigned long a2,unsigned long *a3,unsigned int a4)
{
  char v1 [16];
  
  *(unsigned int *)(a0 + 0x38) = a1;
  *(unsigned long *)(a0 + 8) = a2;
  v1 = sub_ee31(a3);
  *(long *)(a0 + 0x10) = SUB168(v1,0);
  *(long *)(a0 + 0x18) = SUB168(v1,8);
  *(unsigned long *)(a0 + 0x20) = *a3;
  *(unsigned long *)(a0 + 0x28) = a3[1];
  *(unsigned int *)(a0 + 0x30) = *(unsigned int *)&a3[3];
  *(unsigned int *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x58) = 0;
  *(char *)(a0 + 0x34) = 0;
}


// Function: sub_44d5 @ 0x44d5
void sub_44d5(unsigned int a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if ((a0 != 0xffffffff) && (a0)) {
    if (close(a0)) {
      v1 = sub_e8ca(4,a1);
      v2 = gettext("closing %s (fd=%d)");
      error(0,*__errno_location(),v2,v1,(unsigned long)a0);
    }
  }
}


// Function: sub_4559 @ 0x4559
void sub_4559(unsigned long a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_18020) ? "" : "\n"; // branch-flip
  printf("%s==> %s <==\n",v1,a0);
  dat_18020 = 0;
}


// Function: sub_45a9 @ 0x45a9
void sub_45a9(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (a1) {
    v1 = fwrite_unlocked(a0,1,a1,stdout);
    if (v1 < a1) {
      clearerr_unlocked(stdout);
      v2 = sub_e8ca(4,"standard output");
      v3 = gettext("error writing %s");
      error(1,*__errno_location(),v3,v2);
      return;
    }
  }
}


// Function: sub_4649 @ 0x4649
long sub_4649(char a0,unsigned long a1,unsigned int a2,void *a3)
{
  void *v1;
  long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  long v5; // rax
  char v6 [8200];
  char v7; // stack - 0x204c
  long v8; // stack - 0x2048
  void *v9; // stack - 0x2040
  
  v8 = 0;
  v7 = a0;
  v9 = a3;
  do {
    do {
      v1 = v9;
      if (_DT_INIT < v9)
        v1 = _DT_INIT;
      v2 = sub_ed34(a2,v6,v1);
      if (v2 == -1) {
        if (*__errno_location() == 0xb)
          return v8;
        v3 = sub_e8ca(4,a1);
        v4 = gettext("error reading %s");
        error(1,*__errno_location(),v4,v3);
        return v5;
      }
      if (!v2)
        return v8;
      if (v7) {
        sub_4559(a1);
        v7 = '\0';
      }
      sub_45a9(v6,v2);
      v8 += v2;
    } while (a3 == (void *)0xffffffffffffffff);
    v9 = &v9[-v2];
    if (!v9)
      return v8;
  } while (a3 != (void *)0xfffffffffffffffe);
  return v8;
}


// Function: sub_4804 @ 0x4804
long sub_4804(unsigned int a0,unsigned long a1,int a2,unsigned long a3)
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  char v5 [24];
  
  v1 = lseek(a0,a1,a2);
  if (0 <= v1)
    return v1;
  v2 = sub_cade(a1,v5);
  if (a2 != 2) { // branch-flip
    if (2 < a2)
      abort(); // no-return, return-dupe
    if (a2) { // branch-flip
      if (a2 != 1) {
        abort();
      }
      v3 = sub_ea5c(0,3,a3);
      v4 = gettext("%s: cannot seek to relative offset %s");
      error(0,*__errno_location(),v4,v3,v2);
    }
    else {
      v3 = sub_ea5c(0,3,a3);
      v4 = gettext("%s: cannot seek to offset %s");
      error(0,*__errno_location(),v4,v3,v2);
    }
  }
  else {
    v3 = sub_ea5c(0,3,a3);
    v4 = gettext("%s: cannot seek to end-relative offset %s");
    error(0,*__errno_location(),v4,v3,v2);
  }
  exit(1); // no-return
}


// Function: sub_49bd @ 0x49bd
unsigned long sub_49bd(unsigned long a0,unsigned int a1,long a2,long a3,long a4,long *a5) // return-dupe x2
{
  unsigned long v1; // rax
  long v10; // stack - 0x2030
  char *v2; // rax
  long v3;
  char v4 [8200];
  bool v5; // zf
  long v6; // stack - 0x2060
  void *v7; // stack - 0x2048
  long v8; // stack - 0x2040
  long v9; // stack - 0x2038
  
  if (!a2)
    return 1;
  v7 = (void *)((a4 - a3) % 0x2000);
  if (!v7)
    v7 = _DT_INIT;
  v8 = a4 - (long)v7;
  sub_4804(a1,v8,0,a0);
  v7 = (long)sub_ed34(a1,v4,v7);
  if (v7 == (void *)0xffffffffffffffff) {
    v1 = sub_e8ca(4,a0);
    v2 = gettext("error reading %s");
    error(0,*__errno_location(),v2,v1);
    return 0;
  }
  *a5 = v8 + (long)v7;
  v6 = a2;
  if ((v7) && (v4[(long)v7 + -1] != dat_18132))
    v6 = a2 + -1;
  do {
    v9 = (long)v7;
    while (v9) {
      v10 = memrchr(v4,(int)dat_18132,v9);
      if (!v10) break;
      v9 = v10 - (long)v4;
      v3 = v6 + -1;
      v5 = v6 == 0;
      v6 = v3;
      if (v5) {
        sub_45a9(v10 + 1,((long)v7 - v9) + -1);
        v3 = sub_4649(0,a0,a1,a4 - ((long)v7 + v8));
        *a5 = v3 + *a5;
        return 1;
      }
    }
    if (v8 == a3) {
      sub_4804(a1,a3,0,a0);
      *a5 = a3 + sub_4649(0,a0,a1,a4);
      return 1;
    }
    v8 -= 0x2000;
    sub_4804(a1,v8,0,a0);
    v7 = (long)sub_ed34(a1,v4,_DT_INIT);
    if (v7 == (void *)0xffffffffffffffff) {
      v1 = sub_e8ca(4,a0);
      v2 = gettext("error reading %s");
      error(0,*__errno_location(),v2,v1);
      return 0;
    }
    *a5 = v8 + (long)v7;
    if (!v7)
      return 1;
  } while( true );
}


// Function: sub_4df7 @ 0x4df7
char sub_4df7(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  long *v1;
  void *v10; // stack - 0x58
  unsigned long v11; // stack - 0x50
  void *v12; // stack - 0x48
  void *v13; // stack - 0x40
  long v14; // stack - 0x38
  void *v2;
  long v3;
  unsigned long v4; // rax
  char *v5; // rax
  int v6; // edx
  char v7; // stack - 0x69
  void *v8; // stack - 0x68
  void *v9; // stack - 0x60
  
  v11 = 0;
  v7 = 1;
  v9 = (void *)sub_f96c(0x2018);
  v8 = v9;
  *(unsigned long *)((long)v9 + 0x2008) = 0;
  *(unsigned long *)((long)v9 + 0x2000) = *(unsigned long *)((long)v9 + 0x2008);
  *(unsigned long *)((long)v9 + 0x2010) = 0;
  v10 = (void *)sub_f96c(0x2018);
  while ((v3 = sub_ed34(a1,v10,_DT_INIT), v3 && (v3 != -1))) {
    *(long *)((long)v10 + 0x2000) = v3;
    *a3 = *a3 + v3;
    *(unsigned long *)((long)v10 + 0x2008) = 0;
    *(unsigned long *)((long)v10 + 0x2010) = 0;
    v12 = v10;
    while (v12 = memchr(v12,(int)dat_18132,(long)v10 + (v3 - (long)v12)), v12) {
      v12 = (void *)((long)v12 + 1);
      *(long *)((long)v10 + 0x2008) = *(long *)((long)v10 + 0x2008) + 1;
    }
    v11 += *(long *)((long)v10 + 0x2008);
    if (0x2000 <= (unsigned long)(*(long *)((long)v9 + 0x2000) + *(long *)((long)v10 + 0x2000))) { // branch-flip
      *(void **)((long)v9 + 0x2010) = v10;
      v9 = *(void **)((long)v9 + 0x2010);
      if (v11 - *(long *)((long)v8 + 0x2008) <= a2) // branch-flip
        v10 = (void *)sub_f96c(0x2018);
      else {
        v10 = v8;
        v11 -= *(long *)((long)v8 + 0x2008);
        v8 = *(void **)((long)v8 + 0x2010);
      }
    }
    else {
      memcpy((void *)((long)v9 + *(long *)((long)v9 + 0x2000)),v10,*(unsigned long *)((long)v10 + 0x2000));
      *(long *)((long)v9 + 0x2000) = *(long *)((long)v9 + 0x2000) + *(long *)((long)v10 + 0x2000);
      *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + *(long *)((long)v10 + 0x2008);
    }
  }
  free(v10);
  if (v3 != -1) { // branch-flip
    if ((!*(long *)((long)v9 + 0x2000)) || (!a2)) goto label_5272;
    if (*(char *)((long)v9 + *(long *)((long)v9 + 0x2000) + -1) != dat_18132) {
      *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + 1;
      v11 += 1;
    }
    v10 = v8;
    for (; a2 < v11 - *(long *)((long)v10 + 0x2008); v11 = v11 - *v1) {
      v1 = (long *)((long)v10 + 0x2008);
      v10 = *(void **)((long)v10 + 0x2010);
    }
    v13 = v10;
    v3 = *(long *)((long)v10 + 0x2000);
    if (a2 < v11) {
      for (v14 = v11 - a2; v14; v14 = v14 + -1) {
        v6 = (int)dat_18132;
        v13 = (void *)((long)rawmemchr(v13,v6) + 1);
      }
    }
    sub_45a9(v13,(long)v10 + (v3 - (long)v13));
    for (v10 = *(void **)((long)v10 + 0x2010); v10; v10 = *(void **)((long)v10 + 0x2010)) {
      sub_45a9(v10,*(unsigned long *)&_DT_INIT[(long)v10]);
    }
  }
  else {
    v4 = sub_e8ca(4,a0);
    v5 = gettext("error reading %s");
    error(0,*__errno_location(),v5,v4);
    v7 = 0;
  }
label_5272:
  while (v8) {
    v2 = *(void **)((long)v8 + 0x2010);
    free(v8);
    v8 = v2;
  }
  return v7;
}


// Function: sub_5286 @ 0x5286
char sub_5286(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  long *v1;
  long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  void *v2;
  long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char v6; // stack - 0x49
  void *v7; // stack - 0x48
  void *v8; // stack - 0x40
  void *v9; // stack - 0x38
  
  v11 = 0;
  v6 = 1;
  v8 = (void *)sub_f96c(0x2010);
  v7 = v8;
  *(unsigned long *)((long)v8 + 0x2000) = 0;
  *(unsigned long *)((long)v8 + 0x2008) = 0;
  v9 = (void *)sub_f96c(0x2010);
  while ((v3 = sub_ed34(a1,v9,_DT_INIT), v3 && (v3 != -1))) {
    *a3 = *a3 + v3;
    *(long *)((long)v9 + 0x2000) = v3;
    *(unsigned long *)((long)v9 + 0x2008) = 0;
    v11 += *(long *)((long)v9 + 0x2000);
    if (0x2000 <= (unsigned long)(*(long *)((long)v8 + 0x2000) + *(long *)((long)v9 + 0x2000))) { // branch-flip
      *(void **)((long)v8 + 0x2008) = v9;
      v8 = *(void **)((long)v8 + 0x2008);
      if (v11 - *(long *)((long)v7 + 0x2000) <= a2) // branch-flip
        v9 = (void *)sub_f96c(0x2010);
      else {
        v9 = v7;
        v11 -= *(long *)((long)v7 + 0x2000);
        v7 = *(void **)((long)v7 + 0x2008);
      }
    }
    else {
      memcpy((void *)((long)v8 + *(long *)((long)v8 + 0x2000)),v9,*(unsigned long *)((long)v9 + 0x2000));
      *(long *)((long)v8 + 0x2000) = *(long *)((long)v8 + 0x2000) + *(long *)((long)v9 + 0x2000);
    }
  }
  free(v9);
  if (v3 != -1) { // branch-flip
    v9 = v7;
    for (; a2 < v11 - *(long *)((long)v9 + 0x2000); v11 = v11 - *v1) {
      v1 = (long *)((long)v9 + 0x2000);
      v9 = *(void **)((long)v9 + 0x2008);
    }
    if (v11 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = v11 - a2;
    }
    sub_45a9(v10 + (long)v9,*(long *)((long)v9 + 0x2000) - v10);
    for (v9 = *(void **)((long)v9 + 0x2008); v9; v9 = *(void **)((long)v9 + 0x2008)) {
      sub_45a9(v9,*(unsigned long *)&_DT_INIT[(long)v9]);
    }
  }
  else {
    v4 = sub_e8ca(4,a0);
    v5 = gettext("error reading %s");
    error(0,*__errno_location(),v5,v4);
    v6 = 0;
  }
  while (v7) {
    v2 = *(void **)((long)v7 + 0x2008);
    free(v7);
    v7 = v2;
  }
  return v6;
}


// Function: sub_55ae @ 0x55ae
unsigned long sub_55ae(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  unsigned long v5; // stack - 0x2050
  
  v5 = a2;
  while( true ) {
    if (!v5)
      return 0;
    v1 = sub_ed34(a1,v4,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == 0xffffffffffffffff) {
      v2 = sub_e8ca(4,a0);
      v3 = gettext("error reading %s");
      error(0,*__errno_location(),v3,v2);
      return 1;
    }
    *a3 = *a3 + v1;
    if (v5 < v1) break;
    v5 -= v1;
  }
  sub_45a9(&v4[v5],v1 - v5);
  return 0;
}


// Function: sub_5741 @ 0x5741
unsigned long sub_5741(unsigned long a0,unsigned int a1,long a2,long *a3) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  void *v5; // rax
  char v6 [8200];
  long v7; // stack - 0x2060
  char *v8; // stack - 0x2040
  
  v7 = a2;
  if (!a2)
    return 0;
  while( true ) {
    v1 = sub_ed34(a1,v6,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == -1) break;
    v4 = &v6[v1];
    *a3 = *a3 + v1;
    v8 = v6;
    while (v5 = memchr(v8,(int)dat_18132,(long)v4 - (long)v8), v5) {
      v8 = (char *)((long)v5 + 1);
      v7 -= 1;
      if (!v7) {
        if (v4 <= v8)
          return 0;
        sub_45a9(v8,(long)v4 - (long)v8);
        return 0;
      }
    }
  }
  v2 = sub_e8ca(4,a0);
  v3 = gettext("error reading %s");
  error(0,*__errno_location(),v3,v2);
  return 1;
}


// Function: sub_5938 @ 0x5938
char sub_5938(unsigned int a0,unsigned long a1)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  unsigned long v4 [15]; // stack - 0x98
  char v5; // stack - 0x9d
  
  v5 = 1;
  if (fstatfs(a0,v4)) { // branch-flip
    if (*__errno_location() != 0x26) {
      v2 = sub_e8ca(4,a1);
      v3 = gettext("cannot determine location of %s. reverting to polling");
      error(0,*__errno_location(),v3,v2);
    }
  }
  else {
    v1 = sub_2da1(v4[0]);
    if (v1 != 1) { // branch-flip
      if ((1 < v1) || ((v1 != -1 && (v1))))
        __assert_fail("!\"unexpected return value from is_local_fs_type\"","src/tail.c",0x3cc,"fremote"); // no-return
    }
    else {
      v5 = 0;
    }
  }
  return v5;
}


// Function: sub_5a69 @ 0x5a69
void sub_5a69(unsigned long *a0,unsigned char a1) // return-dupe, ternary x2, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_5a69
{
  char v1;
  long v10; // stack - 0xb8
  bool v11; // zf
  long v12; // stack - 0xb0
  unsigned int v13; // stack - 0xa0
  int v2;
  bool v3;
  bool v4;
  char v5; // al
  int v6; // eax
  char *v7;
  unsigned int v8; // ecx
  unsigned long v9; // rax
  
  v3 = 1;
  v7 = (char *)*a0;
  v11 = strcmp(v7,"-") == 0;
  v1 = *(char *)((long)a0 + 0x36);
  v2 = *(int *)((long)a0 + 0x3c);
  if (v11)
    v6 = 0;
  else {
    v9 = (a1) ? 0 : 0x800; // branch-flip
    v6 = sub_af09(*a0,v9);
  }
  if (!sub_43ba(a0))
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  v5 = ((dat_1812c != '\x01') || (v6 != -1));
  *(char *)((long)a0 + 0x36) = v5;
  if (((dat_1813a != '\x01') && (v7 = (char *)*a0, !lstat(v7,&v10))) && ((v13 & 0xf000) == 0xa000)) {
    v3 = 0;
    *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
    *(char *)((long)a0 + 0x34) = 1;
    v9 = sub_e8ca(4,sub_43ee(a0));
    error(0,0,gettext("%s has been replaced with an untailable symbolic link"),v9);
  }
  else if ((v6 != -1) && (0 <= fstat(v6,&v10))) {
    if ((((v13 & 0xf000) != 0x8000) && ((v13 & 0xf000) != 0x1000)) && (((v13 & 0xf000) != 0xc000 && ((v13 & 0xf000) != 0x2000)))) {
      v3 = 0;
      *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
      *(char *)((long)a0 + 0x36) = 0;
      v5 = ((dat_1812c != '\x01') || (dat_18010 != 1));
      *(char *)((long)a0 + 0x34) = v5;
      if ((v1) || (v2 != *(int *)((long)a0 + 0x3c))) {
        if (*(char *)((long)a0 + 0x34)) // branch-flip
          v7 = gettext("; giving up on this name");
        else {
          v7 = "";
        }
        v9 = sub_e8ca(4,sub_43ee(a0));
        error(0,0,gettext("%s has been replaced with an untailable file%s"),v9,v7);
      }
    }
    else {
      v9 = sub_43ee(a0);
      *(char *)((long)a0 + 0x35) = sub_5938(v6,v9);
      if ((*(char *)((long)a0 + 0x35)) && (dat_1813a != '\x01')) {
        v3 = 0;
        *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
        v9 = sub_e8ca(4,sub_43ee(a0));
        error(0,0,gettext("%s has been replaced with an untailable remote file"),v9);
        *(char *)((long)a0 + 0x34) = 1;
        *(char *)((long)a0 + 0x35) = 1;
      }
      else {
        *(unsigned int *)((long)a0 + 0x3c) = 0;
      }
    }
  }
  else {
    v3 = 0;
    *(int *)((long)a0 + 0x3c) = *__errno_location();
    if (*(char *)((long)a0 + 0x36) != '\x01') { // branch-flip
      if (v1) {
        v9 = sub_e8ca(4,sub_43ee(a0));
        v7 = gettext("%s has become inaccessible");
        error(0,*(int *)((long)a0 + 0x3c),v7,v9);
      }
    }
    else if (v2 != *__errno_location()) {
      v9 = sub_ea5c(0,3,sub_43ee(a0));
      error(0,*__errno_location(),"%s",v9);
    }
  }
  v4 = 0;
  if (v3) {
    if ((v2) && (v2 != 2)) {
      v4 = 1;
      if (*(int *)&a0[7] != -1)
        __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
      v9 = sub_e8ca(4,sub_43ee(a0));
      error(0,0,gettext("%s has become accessible"),v9);
    }
    else if (*(int *)&a0[7] != -1) { // branch-flip
      if ((a0[5] != v12) || (a0[4] != v10)) {
        v4 = 1;
        v9 = sub_e8ca(4,sub_43ee(a0));
        error(0,0,gettext("%s has been replaced;  following new file"),v9);
        v9 = sub_43ee(a0);
        sub_44d5(*(unsigned int *)&a0[7],v9);
      }
      else {
        sub_44d5(v6,sub_43ee(a0));
      }
    }
    else {
      v4 = 1;
      v9 = sub_e8ca(4,sub_43ee(a0));
      error(0,0,gettext("%s has appeared;  following new file"),v9);
    }
  }
  else {
    sub_44d5(v6,sub_43ee(a0));
    v9 = sub_43ee(a0);
    sub_44d5(*(unsigned int *)&a0[7],v9);
    *(unsigned int *)&a0[7] = 0xffffffff;
  }
  if (!v4)
    return;
  v8 = (v11) ? 0xffffffff : (unsigned int)a1;
  sub_4435(a0,v6,0,&v10,v8);
  sub_4804(v6,0,0,sub_43ee(a0));
}


// Function: sub_6245 @ 0x6245
unsigned long sub_6245(long a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // stack - 0x10
  
  if ((dat_1812c) && (dat_18010 == 1))
    return 1;
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if (0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) break;
    if ((*(char *)(a0 + v1 * 0x60 + 0x34) != '\x01') && (dat_1812c))
      return 1;
    v1 += 1;
  }
  return 1;
}


// Function: sub_62f3 @ 0x62f3
void sub_62f3(long a0,unsigned long a1) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_62f3
{
  unsigned int v1;
  long v10;
  unsigned long v11;
  char *v12; // rax
  char v13 [24];
  bool v14;
  char v15 [16];
  unsigned long v16; // stack - 0xe0
  unsigned long v17; // stack - 0xd8
  long v18; // stack - 0xd0
  unsigned int v19; // stack - 0xa0
  unsigned long v2;
  long v20; // stack - 0x88
  unsigned long v3;
  bool v4;
  unsigned char v5; // al
  unsigned int v6; // eax
  unsigned int v7; // eax
  int v8;
  unsigned long v9; // rax
  
  v5 = !((((dat_18134) || (dat_18010 != 2)) || (a1 != 1)) || ((*(int *)(a0 + 0x38) == -1 || ((*(unsigned int *)(a0 + 0x30) & 0xf000) == 0x8000))));
  v4 = 0;
  v16 = a1 - 1;
  do {
    v14 = 0;
    for (v17 = 0; v17 < a1; v17 = v17 + 1) {
      if (!*(char *)(a0 + v17 * 0x60 + 0x34)) {
        if (0 <= *(int *)(a0 + v17 * 0x60 + 0x38)) { // branch-flip
          v8 = *(int *)(a0 + v17 * 0x60 + 0x38);
          v9 = sub_43ee(a0 + v17 * 0x60);
          v1 = *(unsigned int *)(a0 + v17 * 0x60 + 0x30);
          if (*(unsigned int *)(a0 + v17 * 0x60 + 0x40) != (unsigned int)v5) {
            v6 = sub_aa63(v8,3);
            v7 = (v5) ? 0 : 0x800; // branch-flip
            if ((0 <= (int)v6) && (((v7 | v6) == v6 || (sub_aa63(v8,4,v7 | v6) != -1))))
              *(unsigned int *)(v17 * 0x60 + a0 + 0x40) = (unsigned int)v5;
            else if (((*(unsigned int *)(a0 + v17 * 0x60 + 0x30) & 0xf000) != 0x8000) || (*__errno_location() != 1)) {
              v9 = sub_ea5c(0,3,v9);
              v12 = gettext("%s: cannot change nonblocking mode");
              error(1,*__errno_location(),v12,v9);
              return;
            }
          }
          if (!*(int *)(a0 + v17 * 0x60 + 0x40)) {
            if (fstat(v8,v13)) {
              *(unsigned int *)(a0 + v17 * 0x60 + 0x38) = 0xffffffff;
              *(int *)(v17 * 0x60 + a0 + 0x3c) = *__errno_location();
              v9 = sub_ea5c(0,3,v9);
              error(0,*__errno_location(),"%s",v9);
              close(v8);
              goto label_6bb6;
            }
            if ((*(unsigned int *)(a0 + v17 * 0x60 + 0x30) == v19) && (((v19 & 0xf000) != 0x8000 || (*(long *)(a0 + v17 * 0x60 + 8) == v20)))) {
              v15 = sub_ee31(v13);
              v10 = a0 + v17 * 0x60;
              v11 = *(unsigned long *)(v10 + 0x10);
              v2 = *(unsigned long *)(v10 + 0x18);
              if (!sub_eeaf(v11,v2,SUB168(v15,0),SUB168(v15,8))) {
                v10 = a0 + v17 * 0x60;
                v3 = *(unsigned long *)(v10 + 0x58);
                *(unsigned long *)(v10 + 0x58) = v3 + 1;
                if ((dat_18018 <= v3) && (dat_18010 == 1)) {
                  sub_5a69(a0 + v17 * 0x60,*(int *)(a0 + v17 * 0x60 + 0x40) != 0);
                  *(unsigned long *)(a0 + v17 * 0x60 + 0x58) = 0;
                }
                goto label_6bb6;
              }
            }
            v10 = v17 * 0x60 + a0;
            v15 = sub_ee31(v13);
            *(long *)(v10 + 0x10) = SUB168(v15,0);
            *(long *)(v10 + 0x18) = SUB168(v15,8);
            *(unsigned int *)(v17 * 0x60 + a0 + 0x30) = v19;
            *(unsigned long *)(a0 + v17 * 0x60 + 0x58) = 0;
            if (((v1 & 0xf000) == 0x8000) && (v20 < *(long *)(a0 + v17 * 0x60 + 8))) {
              v11 = sub_ea5c(0,3,v9);
              error(0,0,gettext("%s: file truncated"),v11);
              sub_4804(v8,0,0,v9);
              *(unsigned long *)(a0 + v17 * 0x60 + 8) = 0;
            }
            if (v17 != v16) {
              if (dat_18131)
                sub_4559(v9);
              v16 = v17;
            }
          }
          if (*(int *)(a0 + v17 * 0x60 + 0x40)) // branch-flip
            v18 = -2;
          else if (((v1 & 0xf000) != 0x8000) || (!*(char *)(a0 + v17 * 0x60 + 0x35)))
            v18 = -1;
          else {
            v18 = v20 - *(long *)(a0 + v17 * 0x60 + 8);
          }
          v10 = sub_4649(0,v9,v8,v18);
          v14 = v10 || v14;
          *(long *)(a0 + v17 * 0x60 + 8) = *(long *)(a0 + v17 * 0x60 + 8) + v10;
        }
        else {
          sub_5a69(a0 + v17 * 0x60,v5);
        }
      }
label_6bb6:
    }
    if (sub_6245(a0,a1) != '\x01') {
      error(0,0,gettext("no files remaining"));
      return;
    }
    if (((!v14) || (v5)) && (v8 = fflush_unlocked(stdout), v8)) {
      v12 = gettext("write error");
      error(1,*__errno_location(),v12);
      return;
    }
    sub_4338();
    if (!v14) {
      if (v4)
        return;
      if (((dat_18134) && (v8 = kill(dat_18134,0), v8)) && (*__errno_location() != 1))
        v4 = 1;
      else {
        v4 = 0;
      }
      if ((!v4) && (sub_101af())) {
        v12 = gettext("cannot read realtime clock");
        error(1,*__errno_location(),v12);
        return;
      }
    }
  } while( true );
}


// Function: sub_6d57 @ 0x6d57
unsigned long sub_6d57(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if ((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && (*(char *)(a0 + v1 * 0x60 + 0x35))) break;
    v1 += 1;
  }
  return 1;
}


// Function: sub_6dd3 @ 0x6dd3
unsigned long sub_6dd3(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if ((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && (*(char *)(a0 + v1 * 0x60 + 0x35) != '\x01')) break;
    v1 += 1;
  }
  return 1;
}


// Function: sub_6e52 @ 0x6e52
unsigned long sub_6e52(long a0,unsigned long a1)
{
  char *v1;
  char v2 [24];
  unsigned long v3; // stack - 0xb0
  unsigned int v4; // stack - 0x90
  
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 0;
    v1 = *(char **)(a0 + v3 * 0x60);
    if ((!lstat(v1,v2)) && ((v4 & 0xf000) == 0xa000)) break;
    v3 += 1;
  }
  return 1;
}


// Function: sub_6f11 @ 0x6f11
unsigned long sub_6f11(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if (((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && ((*(unsigned int *)(a0 + v1 * 0x60 + 0x30) & 0xf000) != 0x8000)) && ((*(unsigned int *)(a0 + v1 * 0x60 + 0x30) & 0xf000) != 0x1000)) break;
    v1 += 1;
  }
  return 1;
}


// Function: sub_6fc5 @ 0x6fc5
unsigned long sub_6fc5(long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  while( true ) {
    if (a1 <= v2)
      return 0;
    if ((*(char *)(a0 + v2 * 0x60 + 0x34) != '\x01') && (v1 = *(char **)(a0 + v2 * 0x60), !strcmp(v1,"-"))) break;
    v2 += 1;
  }
  return 1;
}


// Function: sub_705a @ 0x705a
unsigned long sub_705a(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}


// Function: sub_7089 @ 0x7089
unsigned int sub_7089(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}


// Function: sub_70be @ 0x70be
void sub_70be(long a0,long *a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_70be
{
  unsigned long v1;
  long v10; // stack - 0x88
  char v2; // al
  int v3;
  unsigned long v4; // rax
  unsigned long v5;
  long v6; // rax
  char *v7; // rax
  char v8 [48];
  char v9 [16];
  
  if (*(int *)(a0 + 0x38) == -1)
    return;
  v4 = sub_43ee(a0);
  v3 = *(int *)(a0 + 0x38);
  if (fstat(v3,v8)) {
    *(int *)(a0 + 0x3c) = *__errno_location();
    sub_44d5(*(unsigned int *)(a0 + 0x38),v4);
    *(unsigned int *)(a0 + 0x38) = 0xffffffff;
    return;
  }
  if (((*(unsigned int *)(a0 + 0x30) & 0xf000) != 0x8000) || (*(long *)(a0 + 8) <= v10)) {
    if (((*(unsigned int *)(a0 + 0x30) & 0xf000) == 0x8000) && (v10 == *(long *)(a0 + 8))) {
      v9 = sub_ee31(v8);
      v5 = *(unsigned long *)(a0 + 0x10);
      v1 = *(unsigned long *)(a0 + 0x18);
      if (!sub_eeaf(v5,v1,SUB168(v9,0),SUB168(v9,8)))
        return;
    }
  }
  else {
    v5 = sub_ea5c(0,3,v4);
    error(0,0,gettext("%s: file truncated"),v5);
    sub_4804(*(unsigned int *)(a0 + 0x38),0,0,v4);
    *(unsigned long *)(a0 + 8) = 0;
  }
  v2 = ((dat_18131) && (a0 != *a1));
  v6 = sub_4649(v2,v4,*(unsigned int *)(a0 + 0x38),0xffffffffffffffff);
  *(long *)(a0 + 8) = v6 + *(long *)(a0 + 8);
  if (!v6)
    return;
  *a1 = a0;
  v3 = fflush_unlocked(stdout);
  if (!v3)
    return;
  v7 = gettext("write error");
  error(1,*__errno_location(),v7);
}


// Function: sub_7379 @ 0x7379
void sub_7379(double a0,unsigned int a1,long a2,unsigned long a3,long *a4) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_7379
{
  char v1;
  int *v10; // rax
  unsigned long v11;
  unsigned int v12; // stack - 0xb8
  long v13; // stack - 0x190
  char v14 [68];
  bool v15;
  int v16; // stack - 0x1a4
  unsigned int v17; // stack - 0x1a0
  int v18; // stack - 0x19c
  int v19; // stack - 0x198
  long v2;
  unsigned long v20; // stack - 0x188
  long v21; // stack - 0x180
  unsigned long v22; // stack - 0x178
  unsigned long v23; // stack - 0x170
  unsigned long v24; // stack - 0x168
  long v25; // stack - 0x160
  unsigned long v26; // stack - 0x158
  long v27; // stack - 0x150
  unsigned long v28; // stack - 0x148
  long v29; // stack - 0x140
  bool v3;
  double v30; // stack - 0x138
  int *v31; // stack - 0x130
  int *v32; // stack - 0x128
  long v33; // stack - 0x120
  int v34; // stack - 0xd4
  unsigned short v35; // stack - 0xb4
  unsigned short v36; // stack - 0xb2
  unsigned int v37; // stack - 0xb0
  unsigned short v38; // stack - 0xac
  short v39; // stack - 0xaa
  bool v4;
  bool v5;
  bool v6;
  int v7;
  long v8;
  char *v9;
  
  v16 = 3;
  v5 = 0;
  v4 = 0;
  v3 = 0;
  v15 = 0;
  v6 = 0;
  v20 = 0;
  v22 = 0;
  v23 = 0;
  v27 = sub_b9fe(a3,0,sub_705a,sub_7089,0);
  if (!v27)
    sub_ffa4(); // no-return
  *a4 = v27;
  v17 = 2;
  if (dat_18010 == 1)
    v17 = 0xc06;
  for (v24 = 0; v24 < a3; v24 = v24 + 1) {
    if (*(char *)(a2 + v24 * 0x60 + 0x34) != '\x01') {
      v28 = strlen(*(char **)(a2 + v24 * 0x60));
      if (v20 < v28)
        v20 = v28;
      *(unsigned int *)(a2 + v24 * 0x60 + 0x44) = 0xffffffff;
      if (dat_18010 == 1) {
        v29 = sub_a943(*(unsigned long *)(a2 + v24 * 0x60));
        v1 = *(char *)(v29 + *(long *)(a2 + v24 * 0x60));
        v8 = sub_a685(*(unsigned long *)(a2 + v24 * 0x60));
        *(long *)(a2 + v24 * 0x60 + 0x50) = v8 - *(long *)(a2 + v24 * 0x60);
        *(char *)(v29 + *(long *)(a2 + v24 * 0x60)) = 0;
        if (v29) // branch-flip
          v9 = *(char **)(a2 + v24 * 0x60);
        else {
          v9 = ".";
        }
        v8 = v24 * 0x60;
        *(unsigned int *)(v8 + a2 + 0x48) = inotify_add_watch(a1,v9,0x784);
        *(char *)(*(long *)(a2 + v24 * 0x60) + v29) = v1;
        if (*(int *)(a2 + v24 * 0x60 + 0x48) <= -1) {
          if (*__errno_location() != 0x1c) { // branch-flip
            v11 = sub_e8ca(4,*(unsigned long *)(a2 + v24 * 0x60));
            v9 = gettext("cannot watch parent directory of %s");
            error(0,*__errno_location(),v9,v11);
          }
          else {
            error(0,0,gettext("inotify resources exhausted"));
          }
          v3 = 1;
          break;
        }
      }
      v11 = *(unsigned long *)(a2 + v24 * 0x60);
      v8 = v24 * 0x60;
      *(unsigned int *)(v8 + a2 + 0x44) = inotify_add_watch(a1,v11,v17);
      if (0 <= *(int *)(a2 + v24 * 0x60 + 0x44)) { // branch-flip
        v8 = sub_c73f(v27,v24 * 0x60 + a2);
        if (!v8)
          sub_ffa4(); // no-return
        v5 = 1;
      }
      else {
        if (*(int *)(a2 + v24 * 0x60 + 0x38) != -1)
          v4 = 1;
        if ((*__errno_location() == 0x1c) || (*__errno_location() == 0xc)) {
          v15 = 1;
          error(0,0,gettext("inotify resources exhausted"));
          break;
        }
        if (*__errno_location() != *(int *)(a2 + v24 * 0x60 + 0x3c)) {
          v11 = sub_e8ca(4,*(unsigned long *)(a2 + v24 * 0x60));
          v9 = gettext("cannot watch %s");
          error(0,*__errno_location(),v9,v11);
        }
      }
    }
  }
  if (((v15) || (v3)) || ((dat_18010 == 2 && (v4))))
    return;
  if ((dat_18010 == 2) && (!v5))
    exit(1); // no-return
  v13 = a2 + a3 * 0x60 + -0x60;
  for (v24 = 0; v24 < a3; v24 = v24 + 1) {
    if (*(char *)(a2 + v24 * 0x60 + 0x34) != '\x01') {
      if (dat_18010 != 1) { // branch-flip
        if (((*(int *)(a2 + v24 * 0x60 + 0x38) != -1) && (v9 = *(char **)(a2 + v24 * 0x60), !stat(v9,&v12))) && ((*(long *)(a2 + v24 * 0x60 + 0x20) != CONCAT26(v36,CONCAT24(v35,v12)) || (*(long *)(a2 + v24 * 0x60 + 0x28) != CONCAT26(v39,CONCAT24(v38,v37)))))) {
          v8 = v24 * 0x60;
          v11 = sub_e8ca(4,sub_43ee(a2 + v8));
          v9 = gettext("%s was replaced");
          error(0,*__errno_location(),v9,v11);
          return;
        }
      }
      else {
        sub_5a69(a2 + v24 * 0x60,0);
      }
      sub_70be(v24 * 0x60 + a2,&v13);
    }
  }
  v20 += 0x11;
  v21 = sub_f96c(v20);
label_7c7d:
  while( true ) {
    if (((dat_18010 == 1) && (dat_1812c != '\x01')) && (v8 = sub_b071(v27), !v8)) {
      error(1,0,gettext("no files remaining"));
      return;
    }
    if (v22 < v23) break;
    do {
      v18 = -1;
      if (dat_18134) {
        if (v6)
          exit(0); // no-return
        v7 = kill(dat_18134,0);
        if ((v7) && (*__errno_location() != 1))
          v6 = 1;
        else {
          v6 = 0;
        }
        if ((v6) || (a0 <= 0.0))
          v18 = 0;
        else if (a0 < dat_13558) {
          v30 = dat_13560 * a0;
          v18 = (int)v30 + (unsigned int)((double)(int)v30 < v30);
        }
      }
      v35 = 1;
      v37 = 1;
      v39 = 0;
      v38 = 0;
      v12 = a1;
      v7 = poll(&v12,dat_1812f + 1,v18);
    } while (!v7);
    if (v7 <= -1) {
      v9 = gettext("error waiting for inotify and output events");
      error(1,*__errno_location(),v9);
      return;
    }
    if (v39)
      sub_431c(); // no-return
    v23 = sub_ed34(a1,v21,v20);
    v22 = 0;
    if (((v23) && ((v23 != 0xffffffffffffffff || (*__errno_location() != 0x16)))) || (v7 = v16 + -1, v15 = v16 == 0, v16 = v7, v15)) {
      if ((!v23) || (v23 == 0xffffffffffffffff)) {
        v9 = gettext("error reading inotify event");
        error(1,*__errno_location(),v9);
        return;
      }
      break;
    }
    v23 = 0;
    v20 <<= 1;
    v21 = sub_f9d6(v21,v20);
  }
  v31 = (int *)(v22 + v21);
  v22 = v22 + (unsigned int)v31[3] + 0x10;
  v32 = v31;
  if ((v31[1] & 0x400U) && (!v31[3])) {
    for (v24 = 0; v24 < a3; v24 = v24 + 1) {
      if (*v31 == *(int *)(a2 + v24 * 0x60 + 0x48)) {
        error(0,0,gettext("directory containing watched file was removed"));
        return;
      }
    }
  }
  if (v31[3]) { // branch-flip
    v26 = 0;
    while ((v26 < a3 && ((*(int *)(a2 + v26 * 0x60 + 0x48) != *v32 || (v8 = *(long *)(a2 + v26 * 0x60), v2 = *(long *)(a2 + v26 * 0x60 + 0x50), v10 = &v32[4], strcmp((char *)v10,(char *)(v8 + v2))))))) {
      v26 += 1;
    }
    if (v26 == a3) goto label_7c7d;
    v25 = a2 + v26 * 0x60;
    v19 = -1;
    v15 = (v32[1] & 0x200U) != 0;
    if (!v15) {
      v19 = inotify_add_watch(a1,*(unsigned long *)(a2 + v26 * 0x60),v17);
      if (v19 <= -1) {
        if ((*__errno_location() == 0x1c) || (*__errno_location() == 0xc)) {
          error(0,0,gettext("inotify resources exhausted"));
          return;
        }
        v11 = sub_e8ca(4,*(unsigned long *)(a2 + v26 * 0x60));
        v9 = gettext("cannot watch %s");
        error(0,*__errno_location(),v9,v11);
      }
    }
    v15 = !((v15) || ((0 <= *(int *)(v25 + 0x44) && (v19 == *(int *)(v25 + 0x44)))));
    if (v15) {
      if (0 <= *(int *)(v25 + 0x44)) {
        inotify_rm_watch(a1,*(unsigned int *)(v25 + 0x44));
        sub_c7af(v27,v25);
      }
      *(int *)(v25 + 0x44) = v19;
      if (v19 == -1) goto label_7c7d;
      v33 = sub_c7af(v27,v25);
      if ((v33) && (v33 != v25)) {
        if (dat_18010 == 1)
          sub_5a69(v33,0);
        *(unsigned int *)(v33 + 0x44) = 0xffffffff;
        v11 = sub_43ee(v33);
        sub_44d5(*(unsigned int *)(v33 + 0x38),v11);
      }
      v8 = sub_c73f(v27,v25);
      if (!v8)
        sub_ffa4(); // no-return
    }
    if (dat_18010 == 1)
      sub_5a69(v25,0);
  }
  else {
    v34 = *v31;
    v25 = sub_b349(v27,v14);
  }
  if (v25) {
    if (v32[1] & 0xe04U) { // branch-flip
      if (v32[1] & 0x400U) {
        inotify_rm_watch(a1,*(unsigned int *)(v25 + 0x44));
        sub_c7af(v27,v25);
      }
      sub_5a69(v25,0);
    }
    else {
      sub_70be(v25,&v13);
    }
  }
  goto label_7c7d;
}


// Function: sub_8520 @ 0x8520
unsigned long sub_8520(unsigned long a0,int a1,unsigned long a2,long *a3) // return-dupe
{
  unsigned int v1; // eax
  long v10; // stack - 0x88
  unsigned long v11; // stack - 0x80
  unsigned long v2; // rax
  char *v3; // rax
  long v4; // rax
  char v5 [24];
  unsigned long v6; // stack - 0xf0
  long v7; // stack - 0xd0
  long v8; // stack - 0xc8
  unsigned int v9; // stack - 0xa0
  
  if (fstat(a1,v5)) {
    v2 = sub_e8ca(4,a0);
    v3 = gettext("cannot fstat %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  if (dat_18130) { // branch-flip
    if (((dat_18139 != '\x01') && (0 <= (long)a2)) && ((((v9 & 0xf000) == 0x8000 && (0 <= (long)sub_4804(a1,a2,1,a0))) || (lseek(a1,a2,1) != -1))))
      *a3 = *a3 + a2;
    else {
      v1 = sub_55ae(a0,a1,a2,a3);
      if (v1)
        return (unsigned long)(v1 >> 0x1f);
    }
    v6 = 0xffffffffffffffff;
  }
  else {
    v7 = -1;
    v8 = -1;
    if ((dat_18139 != '\x01') && (0 <= (long)a2)) {
      if (sub_2d62(v5)) // branch-flip
        v7 = v10;
      else {
        v8 = lseek(a1,-a2,2);
        if (v8 != -1)
          v7 = a2 + v8;
      }
    }
    if (((long)v11 <= 0) || (0x2000000000000000 < v11))
      v11 = 0x200;
    if (v7 <= (long)v11)
      return sub_5286(a0,a1,a2,a3);
    if (v8 == -1)
      v8 = sub_4804(a1,0,1,a0);
    if ((v8 < v7) && (a2 < (unsigned long)(v7 - v8))) {
      v8 = v7 - a2;
      sub_4804(a1,v8,0,a0);
    }
    *a3 = v8;
    v6 = a2;
  }
  v4 = sub_4649(0,a0,a1,v6);
  *a3 = v4 + *a3;
  return 1;
}


// Function: sub_88c7 @ 0x88c7
int * sub_88c7(unsigned long a0,int a1,unsigned long a2,long *a3) // early-return x3
{
  unsigned long v1;
  char *v2;
  long v3;
  char v4; // al
  unsigned int v5; // eax
  int *v6; // rax
  char v7 [24];
  long v8; // stack - 0xc8
  unsigned int v9; // stack - 0xa0
  
  v5 = fstat(a1,v7);
  if (v5) {
    v6 = (unsigned long)sub_e8ca(4,a0);
    v1 = v6;
    v6 = (char *)gettext("cannot fstat %s");
    v2 = (char *)v6;
    v6 = __errno_location();
    error(0,*v6,v2,v1);
    v6 = NULL;
    return v6;
  }
  if (dat_18130) { // branch-flip
    v5 = sub_5741(a0,a1,a2,a3);
    if (!v5) {
      v6 = (long)sub_4649(0,a0,a1,0xffffffffffffffff);
      *a3 = (long)v6 + *a3;
      v6 = (int *)0x1;
      return v6;
    }
    v5 >>= 0x1f;
    v6 = (unsigned long)(unsigned long)v5;
  }
  else {
    v8 = -1;
    if ((dat_18139 != '\x01') && (v5 = v9 & 0xf000, v5 == 0x8000)) {
      v6 = (long)lseek(a1,0,1);
      v3 = (long)v6;
      v8 = (long)v6;
      if ((v6 != (int *)0xffffffffffffffff) && (v6 = (long)lseek(a1,0,2), v3 < (long)v6)) {
        *a3 = (long)v6;
        if (!v6) {
          v6 = (int *)0x1;
          return v6;
        }
        v4 = sub_49bd(a0,a1,a2,v3,v6,a3);
        if (v4 == '\x01') {
          v6 = (int *)0x1;
          return v6;
        }
        v6 = NULL;
        return v6;
      }
    }
    if (v8 != -1)
      sub_4804(a1,v8,0,a0);
    v6 = (unsigned long)sub_4df7(a0,a1,a2,a3);
  }
  return v6;
}


// Function: sub_8b67 @ 0x8b67
void sub_8b67(unsigned long a0,unsigned int a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  *a3 = 0;
  if (dat_1812d) {
    sub_88c7(a0,a1,a2,a3);
    return;
  }
  sub_8520(a0,a1,a2,a3);
}


// Function: sub_8bca @ 0x8bca
unsigned char sub_8bca(unsigned long *a0,unsigned long a1) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_8bca
{
  char v1; // al
  unsigned int v10; // stack - 0xa0
  unsigned long v2;
  char *v3;
  unsigned int v4; // ecx
  unsigned long v5; // stack - 0xc0
  char v6 [24];
  bool v7; // zf
  unsigned char v8; // stack - 0xc6
  int v9; // stack - 0xc4
  
  v3 = (char *)*a0;
  v7 = strcmp(v3,"-") != 0;
  if (v7) {
    v2 = *a0;
    v9 = sub_af09(v2,0);
  }
  else {
    dat_18138 = 1;
    v9 = 0;
    sub_ffef(0,0);
  }
  v1 = ((dat_1812c != '\x01') || (v9 != -1));
  *(char *)((long)a0 + 0x36) = v1;
  if (v9 != -1) { // branch-flip
    if (dat_18131)
      sub_4559(sub_43ee(a0));
    v8 = sub_8b67(sub_43ee(a0),v9,a1,&v5);
    if (dat_1812e) { // branch-flip
      *(unsigned int *)((long)a0 + 0x3c) = (unsigned int)v8 - 1;
      if (0 <= fstat(v9,v6)) { // branch-flip
        if (((((v10 & 0xf000) != 0x8000) && ((v10 & 0xf000) != 0x1000)) && ((v10 & 0xf000) != 0xc000)) && ((v10 & 0xf000) != 0x2000)) {
          v8 = 0;
          *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
          *(char *)((long)a0 + 0x36) = 0;
          *(bool *)((long)a0 + 0x34) = dat_1812c == '\0';
          if (*(char *)((long)a0 + 0x34)) // branch-flip
            v3 = gettext("; giving up on this name");
          else {
            v3 = "";
          }
          v2 = sub_ea5c(0,3,sub_43ee(a0));
          error(0,0,gettext("%s: cannot follow end of this type of file%s"),v2,v3);
        }
      }
      else {
        v8 = 0;
        *(int *)((long)a0 + 0x3c) = *__errno_location();
        v2 = sub_e8ca(4,sub_43ee(a0));
        v3 = gettext("error reading %s");
        error(0,*__errno_location(),v3,v2);
      }
      if (v8 != 1) { // branch-flip
        *(bool *)((long)a0 + 0x34) = dat_1812c == '\0';
        sub_44d5(v9,sub_43ee(a0));
        *(unsigned int *)&a0[7] = 0xffffffff;
      }
      else {
        v4 = (v7) ? 1 : 0xffffffff;
        sub_4435(a0,v9,v5,v6,v4);
        v2 = sub_43ee(a0);
        *(char *)((long)a0 + 0x35) = sub_5938(v9,v2);
      }
    }
    else {
      if (!v7)
        return v8;
      if (!close(v9))
        return v8;
      v2 = sub_e8ca(4,sub_43ee(a0));
      v3 = gettext("error reading %s");
      error(0,*__errno_location(),v3,v2);
      v8 = 0;
    }
  }
  else {
    if (dat_1812e) {
      *(unsigned int *)&a0[7] = 0xffffffff;
      *(int *)((long)a0 + 0x3c) = *__errno_location();
      *(bool *)((long)a0 + 0x34) = dat_1812c == '\0';
      a0[5] = 0;
      a0[4] = 0;
    }
    v2 = sub_e8ca(4,sub_43ee(a0));
    v3 = gettext("cannot open %s for reading");
    error(0,*__errno_location(),v3,v2);
    v8 = 0;
  }
  return v8;
}


// Function: sub_90f5 @ 0x90f5
long * sub_90f5(int a0,long a1,long *a2) // return-dupe x5, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_90f5
{
  char *v1;
  char v10; // stack - 0x3c
  int v11; // stack - 0x38
  char *v12; // stack - 0x30
  char v2;
  bool v3;
  unsigned long v4;
  char *v5;
  bool v6; // al
  unsigned int v7; // eax
  long *v8; // rax
  char v9; // stack - 0x3d
  
  v11 = 10;
  v10 = 1;
  if (((a0 != 2) && ((a0 != 3 || ((v8 = (unsigned long *)(a1 + 0x10), *(char *)*v8 == '-' && (v8 = (long *)(a1 + 0x10), v8 = (char *)(*v8 + 1), *(char *)v8)))))) && ((a0 <= 2 || ((5 <= a0 || (v8 = (unsigned long *)(a1 + 0x10), v7 = strcmp((char *)*v8,"--"), v7)))))) {
    v8 = NULL;
    return v8;
  }
  v7 = sub_cc1c();
  v3 = (int)v7 <= 0x30daf;
  v6 = ((v3) || (0x31069 <= (int)v7));
  v1 = &(*(char **)(a1 + 8))[1];
  v2 = **(char **)(a1 + 8);
  v12 = v1;
  if (v2 != '+') { // branch-flip
    if (v2 != '-') {
      v8 = NULL;
      return v8;
    }
    if ((!v3) && (v8 = (char *)&v1[*v1 == 'c'], !*(char *)v8)) {
      v8 = NULL;
      return v8;
    }
    v9 = 0;
  }
  else {
    if (!v6) {
      v8 = NULL;
      return v8;
    }
    v9 = 1;
  }
  for (; v5 = v12, v7 = (int)*v12, v7 = v7 - 0x30, v7 <= 9; v12 = &v12[1]) {
  }
  v2 = *v12;
  if (v2 != 'l') {
    if ('m' <= v2) goto label_929b;
    if (v2 != 'b') { // branch-flip
      if (v2 != 'c') goto label_929b;
    }
    else {
      v11 = 0x1400;
    }
    v10 = 0;
  }
  v12 = &v12[1];
label_929b:
  v2 = *v12;
  if (v2 == 'f')
    v12 = &v12[1];
  if (*v12) {
    v8 = NULL;
    return v8;
  }
  if (v1 != v5) { // branch-flip
    v7 = sub_10410(v1,0,10,a2,"b");
    v7 &= 0xfffffffd;
    if (v7) {
      v8 = (unsigned long *)(a1 + 8);
      v8 = (unsigned long)sub_ed11(*v8);
      v4 = v8;
      v8 = (char *)gettext("invalid number");
      v1 = (char *)v8;
      v8 = (int *)__errno_location();
      error(1,*(int *)v8,"%s: %s",v1,v4);
      return v8;
    }
  }
  else {
    *a2 = (long)v11;
  }
  v8 = (long *)0x1;
  dat_1812d = v10;
  dat_1812e = v2 == 'f';
  dat_18130 = v9;
  return v8;
}


// Function: sub_9381 @ 0x9381
void sub_9381(int a0,char **a1,unsigned long *a2,unsigned int *a3,double *a4)
{
  char v1; // al
  unsigned int v2; // eax
  int v3; // eax
  char *v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  double v7; // stack - 0x28
  
  while( true ) {
    v2 = getopt_long(a0,a1,"c:n:fFqs:vz0123456789",(void *)0x17920,NULL);
    if (v2 == 0xffffffff) {
      if (dat_1812c) {
        if (dat_1812e != '\x01') { // branch-flip
          dat_1812c = '\0';
          error(0,0,gettext("warning: --retry ignored; --retry is useful only when following"));
        }
        else if (dat_18010 == 2)
          error(0,0,gettext("warning: --retry only effective for the initial open"));
      }
      if ((dat_18134) && (dat_1812e != '\x01'))
        error(0,0,gettext("warning: PID ignored; --pid=PID is useful only when following"));
      else if ((dat_18134) && ((v3 = kill(dat_18134,0), v3 && (*__errno_location() == 0x26)))) {
        error(0,0,gettext("warning: --pid=PID is not supported on this system"));
        dat_18134 = 0;
      }
      return;
    }
    if (0x85 < (int)v2) break;
    if ((int)v2 <= 0x2f) {
      if (v2 == 0xffffff7d) {
        sub_f78c(stdout,"tail","GNU coreutils",dat_18030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == 0xffffff7e)
        sub_4115(0); // no-return
      break;
    }
    switch(v2) {
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
        error(1,0,gettext("option used in invalid context -- %c"),(unsigned long)v2);
        return;
      default:
        goto label_975f;
      case 0x46:
        dat_1812e = '\x01';
        dat_18010 = 1;
        dat_1812c = '\x01';
        break;
      case 99:
      case 0x6e:
        dat_1812d = v2 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_18130 = 1;
        }
        if ((bool)dat_1812d)
          v4 = gettext("invalid number of lines");
        else {
          v4 = gettext("invalid number of bytes");
        }
        v6 = sub_10156(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v4,0);
        *a2 = v6;
        break;
      case 0x66:
      case 0x84:
        dat_1812e = '\x01';
        if (optarg) { // branch-flip
          v5 = sub_a546("--follow",optarg,0x17900,0x12210,4,dat_18038,1);
          dat_18010 = *(int *)(v5 * 4 + 0x12210);
        }
        else {
          dat_18010 = 2;
        }
        break;
      case 0x71:
        *a3 = 2;
        break;
      case 0x73:
        v1 = sub_10261(optarg,0,&v7,sub_a745);
        if ((v1 != '\x01') || (v7 < 0.0)) {
          v6 = sub_ed11(optarg);
          error(1,0,gettext("invalid number of seconds: %s"),v6);
        }
        *a4 = v7;
        break;
      case 0x76:
        *a3 = 1;
        break;
      case 0x7a:
        dat_18132 = 0;
        break;
      case 0x80:
        dat_1812c = '\x01';
        break;
      case 0x81:
        dat_18018 = sub_10156(optarg,0,0xffffffffffffffff,0x121c3,gettext("invalid maximum number of unchanged stats between opens"),0);
        break;
      case 0x82:
        dat_18134 = sub_10156(optarg,0,0x7fffffff,0x121c3,gettext("invalid PID"),0);
        break;
      case 0x83:
        dat_18139 = 1;
        break;
      case 0x85:
        dat_1813a = 1;
      
    }
  }
label_975f:
  sub_4115(1); // no-return
}


// Function: sub_98cb @ 0x98cb
long sub_98cb(long a0,unsigned long a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_98cb
{
  char *v1;
  bool v2; // al
  long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v3 = 0;
  for (v4 = 0; v4 < a1; v4 = v4 + 1) {
    v1 = *(char **)(a0 + v4 * 0x60);
    v2 = !(((strcmp(v1,"-")) || (*(char *)(a0 + v4 * 0x60 + 0x34) == '\x01')) || ((*(int *)(a0 + v4 * 0x60 + 0x38) <= -1 || ((*(unsigned int *)(a0 + v4 * 0x60 + 0x30) & 0xf000) != 0x1000))));
    if (v2) {
      *(unsigned int *)(a0 + v4 * 0x60 + 0x38) = 0xffffffff;
      *(char *)(a0 + v4 * 0x60 + 0x34) = 1;
    }
    else {
      v3 += 1;
    }
  }
  return v3;
}


// Function: sub_9a14 @ 0x9a14
unsigned long sub_9a14(int a0,unsigned long *a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_9a14
{
  unsigned char v1; // al
  unsigned long v10; // rax
  unsigned long v11; // rax
  long v12; // stack - 0xf0
  unsigned long v13; // stack - 0xe8
  int v14; // stack - 0xf8
  char v15 [24];
  unsigned long v16; // stack - 0xe0
  bool v17; // stack - 0xfc
  int v18; // stack - 0xf4
  unsigned long v19; // stack - 0xd8
  bool v2;
  unsigned long *v20; // stack - 0xd0
  unsigned long v21; // stack - 0xc8
  long v22; // stack - 0xc0
  unsigned int v23; // stack - 0xa0
  char v3; // al
  int v4; // eax
  unsigned long v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  char *v8;
  unsigned long v9; // rax
  
  v14 = 0;
  v17 = 1;
  v12 = 10;
  v13 = dat_13568;
  sub_ccd0(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_11700(sub_a84b);
  dat_18138 = '\0';
  dat_1812d = 1;
  dat_18131 = 0;
  dat_18130 = '\0';
  dat_1812e = '\0';
  dat_18132 = 10;
  v1 = sub_90f5(a0,a1,&v12);
  a0 -= (unsigned int)v1;
  sub_9381(a0,&a1[v1],&v12,&v14,&v13);
  if ((dat_18130) && (v12))
    v12 -= 1;
  if (a0 <= optind) { // branch-flip
    v19 = 1;
    v20 = (unsigned long *)0x18028;
  }
  else {
    v19 = (unsigned long)(a0 - optind);
    v20 = &(&a1[v1])[optind];
  }
  v2 = 0;
  for (v21 = 0; v21 < v19; v21 = v21 + 1) {
    v8 = (char *)v20[v21];
    if (!strcmp(v8,"-"))
      v2 = 1;
  }
  if ((v2) && (dat_18010 == 1)) {
    v5 = sub_e8ca(4,"-");
    error(1,0,gettext("cannot follow %s by name"),v5);
    return v6;
  }
  if ((dat_1812e) && (v2)) {
    v2 = !((dat_18134) || ((((dat_18010 != 2 || (v19 != 1)) || (fstat(0,v15))) || ((v23 & 0xf000) == 0x8000))));
    if ((!v2) && (isatty(0)))
      error(0,0,gettext("warning: following standard input indefinitely is ineffective"));
  }
  if (((!v12) && (dat_1812e != '\x01')) && (dat_18130 != '\x01'))
    return 0;
  v22 = sub_fae5(v19,0x60);
  for (v21 = 0; v21 < v19; v21 = v21 + 1) {
    *(unsigned long *)(v21 * 0x60 + v22) = v20[v21];
  }
  if ((v14 == 1) || ((!v14 && (2 <= v19))))
    dat_18131 = 1;
  sub_ffef(1,0);
  for (v21 = 0; v21 < v19; v21 = v21 + 1) {
    v1 = sub_8bca(v22 + v21 * 0x60,v12);
    v17 = (v1 & v17) != 0;
  }
  if ((dat_1812e) && (v7 = sub_98cb(v22,v19), v7)) {
    if (fstat(1,v15) <= -1) {
      v8 = gettext("standard output");
      error(1,*__errno_location(),v8);
      return v9;
    }
    dat_1812f = (v23 & 0xf000) == 0x1000;
    if ((dat_1813a != '\x01') && (((((v3 = sub_6fc5(v22,v19), v3 || (v3 = sub_6d57(v22,v19), v3)) || (v3 = sub_6dd3(v22,v19), v3 != '\x01')) || ((v3 = sub_6e52(v22,v19), v3 || (v3 = sub_6f11(v22,v19), v3)))) || ((v17 != 1 && (dat_18010 == 2))))))
      dat_1813a = '\x01';
    if (dat_1813a != '\x01') {
      v18 = inotify_init();
      if (0 <= v18) {
        v4 = fflush_unlocked(stdout);
        if (v4) {
          v8 = gettext("write error");
          error(1,*__errno_location(),v8);
          return v10;
        }
        sub_7379(v18,v22,v19,&v16);
        sub_bc7e(v16);
        close(v18);
        *__errno_location() = 0;
      }
      v8 = gettext("inotify cannot be used, reverting to polling");
      error(0,*__errno_location(),v8);
    }
    dat_1813a = '\x01';
    sub_62f3(v22,v19,v19,v13);
  }
  if ((dat_18138) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v11;
  }
  return (unsigned long)(v17 ^ 1);
}


// Function: sub_b045 @ 0xb045
unsigned long sub_b045(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_b05b @ 0xb05b
unsigned long sub_b05b(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_b071 @ 0xb071
unsigned long sub_b071(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_b087 @ 0xb087
unsigned long sub_b087(unsigned long *a0)
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


// Function: sub_b109 @ 0xb109
unsigned long sub_b109(unsigned long *a0) // return-dupe
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


// Function: sub_b1a7 @ 0xb1a7
void sub_b1a7(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_b071(a0);
  v2 = sub_b045(a0);
  v3 = sub_b05b(a0);
  v4 = sub_b087(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",(double)v2,(dat_13678 * (double)v3) / (double)v2,v3);
  fprintf(a1,"max bucket length: %lu\n",v4);
}


// Function: sub_b2ee @ 0xb2ee
long sub_b2ee(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v1)
    abort(); // no-return
  return v1 * 0x10 + *a0;
}


// Function: sub_b349 @ 0xb349
long sub_b349(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_b2ee(a0,a1);
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


// Function: sub_b3e0 @ 0xb3e0
long sub_b3e0(unsigned long *a0) // early-return
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


// Function: sub_b440 @ 0xb440
long sub_b440(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_b2ee(a0,a1);
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


// Function: sub_b4de @ 0xb4de
unsigned long sub_b4de(unsigned long *a0,long a1,unsigned long a2)
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


// Function: sub_b57e @ 0xb57e
long sub_b57e(unsigned long *a0,void *a1,unsigned long a2)
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


// Function: sub_b616 @ 0xb616
unsigned long sub_b616(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}


// Function: sub_b670 @ 0xb670
unsigned long sub_b670(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}


// Function: sub_b6df @ 0xb6df
unsigned long sub_b6df(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_b670(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}


// Function: sub_b72a @ 0xb72a
void sub_b72a(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = dat_13680;
  a0[2] = dat_13684;
  a0[3] = dat_13688;
  *(char *)&a0[4] = 0;
}


// Function: sub_b780 @ 0xb780
unsigned long sub_b780(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_10957(a0,3) % a1;
}


// Function: sub_b7bb @ 0xb7bb
unsigned long sub_b7bb(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}


// Function: sub_b7d8 @ 0xb7d8
unsigned long sub_b7d8(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x135f0)
    return 1;
  if ((((dat_1368c < v1[2]) && (v1[2] < dat_13680 - dat_1368c)) && (dat_1368c + dat_13680 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1368c < v1[1])) && ((v1[1] <= dat_13680 && (*v1 + dat_1368c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x135f0;
  return 0;
}


// Function: sub_b8e5 @ 0xb8e5
unsigned long sub_b8e5(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_13690 <= v2)
      return 0;
    if (dat_13694 <= v2)
      v3 = (long)(v2 - dat_13694) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_b6df(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}


// Function: sub_b9fe @ 0xb9fe
long * sub_b9fe(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_b780;
  v3 = a3;
  if (!a3)
    v3 = sub_b7bb;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x135f0;
  v2[5] = v5;
  if (sub_b7d8(v2) == '\x01') {
    v2[2] = sub_b8e5(a0,v5);
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


// Function: sub_bb73 @ 0xbb73
void sub_bb73(unsigned long *a0)
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


// Function: sub_bc7e @ 0xbc7e
void sub_bc7e(unsigned long *a0)
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


// Function: sub_bdad @ 0xbdad
void * sub_bdad(long a0)
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


// Function: sub_bdfc @ 0xbdfc
void sub_bdfc(long a0,unsigned long *a1)
{
  *a1 = 0;
  a1[1] = *(unsigned long *)(a0 + 0x48);
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_be36 @ 0xbe36
long sub_be36(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long v2;
  long *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_b2ee(a0,a1);
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
      sub_bdfc(a0,v4);
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
      sub_bdfc(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}


// Function: sub_bfcf @ 0xbfcf
unsigned long sub_bfcf(long a0,unsigned long *a1,char a2)
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
        v3 = (long *)sub_b2ee(a0,v1);
        v2 = (long *)v5[1];
        if (*v3) { // branch-flip
          v5[1] = v3[1];
          v3[1] = (long)v5;
        }
        else {
          *v3 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_bdfc(a0,v5);
        }
        v5 = v2;
      }
      v1 = *v4;
      v4[1] = 0;
      if (!a2) {
        v2 = (long *)sub_b2ee(a0,v1);
        if (*v2) { // branch-flip
          v3 = (long *)sub_bdad(a0);
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


// Function: sub_c1a3 @ 0xc1a3
unsigned long sub_c1a3(unsigned long *a0,unsigned long a1) // return-dupe x4
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
  
  v3 = sub_b8e5(a1,a0[5]);
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
  if (sub_bfcf(&v1,a0,0)) {
    free((void *)*a0);
    *a0 = v1;
    a0[1] = v2;
    a0[2] = v3;
    a0[3] = v4;
    a0[9] = v10;
    return 1;
  }
  a0[9] = v10;
  if ((sub_bfcf(a0,&v1,1) == '\x01') && (sub_bfcf(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}


// Function: sub_c3ce @ 0xc3ce
unsigned long sub_c3ce(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_be36(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_b7d8(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_13690 <= v4)
      return 0xffffffff;
    v1 = (dat_13694 <= v4) ? (long)(v4 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_c1a3(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_be36(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_bdad(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}


// Function: sub_c73f @ 0xc73f
unsigned long sub_c73f(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_c3ce(a0,a1,&v3);
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


// Function: sub_c7af @ 0xc7af
long sub_c7af(long a0,unsigned long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_be36(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_b7d8(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_13694 <= v3) ? (long)(v3 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_13694 <= v3) ? (long)(v3 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_c1a3(a0,v7);
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


// Function: sub_cab5 @ 0xcab5
void sub_cab5(unsigned long a0,unsigned long a1)
{
  sub_c7af(a0,a1);
}

