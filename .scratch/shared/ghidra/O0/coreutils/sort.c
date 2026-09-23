// Function: async_safe_die @ 0x439b

void async_safe_die(int param_1,char *param_2)

{
  size_t sVar1;
  char *__s;
  long in_FS_OFFSET;
  undefined1 local_1c [12];
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  sVar1 = strlen(param_2);
  write(2,param_2,sVar1);
  if (param_1 != 0) {
    __s = (char *)FUN_001126fe(param_1,local_1c);
    write(2,": errno ",8);
    sVar1 = strlen(__s);
    write(2,__s,sVar1);
  }
  write(2,&DAT_0011b24c,1);
                    /* WARNING: Subroutine does not return */
  _exit(2);
}




// Function: sort_die @ 0x4464

void sort_die(undefined8 param_1,long param_2)

{
  undefined8 uVar1;
  int *piVar2;
  
  if (param_2 == 0) {
    param_2 = gettext("standard output");
  }
  uVar1 = FUN_001151eb(0,3,param_2);
  piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar2,"%s: %s",param_1,uVar1);
}




// Function: usage @ 0x44d3

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_00123868;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext(
                            "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"
                            );
    printf(pcVar3,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("Write sorted concatenation of all FILE(s) to standard output.\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_001040a2();
    FUN_001040d3();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("Ordering options:\n\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("Other options:\n\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pcVar3 = (char *)gettext(
                            "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"
                            );
    printf(pcVar3,&DAT_0011bd85);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -z, --zero-terminated     line delimiter is NUL, not newline\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00104104(&DAT_0011c185);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: cs_enter @ 0x47f1

void cs_enter(long param_1)

{
  int iVar1;
  
  iVar1 = pthread_sigmask(0,(__sigset_t *)&DAT_001236a0,(__sigset_t *)(param_1 + 8));
  *(bool *)param_1 = iVar1 == 0;
  return;
}




// Function: cs_leave @ 0x4833

void cs_leave(char *param_1)

{
  if (*param_1 != '\0') {
    pthread_sigmask(2,(__sigset_t *)(param_1 + 8),(__sigset_t *)0x0);
  }
  return;
}




// Function: proctab_hasher @ 0x486b

ulong proctab_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 8) % param_2;
}




// Function: proctab_comparator @ 0x489a

undefined4 proctab_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 8) >> 8),
                  *(int *)(param_1 + 8) == *(int *)(param_2 + 8));
}




// Function: reap @ 0x48cf

__pid_t reap(int param_1)

{
  char cVar1;
  __pid_t __pid;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  uint local_28;
  __pid_t local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __pid = param_1;
  if (param_1 == 0) {
    __pid = -1;
  }
  local_24 = waitpid(__pid,(int *)&local_28,(uint)(param_1 == 0));
  if (-1 < local_24) {
    if ((0 < local_24) && ((0 < param_1 || (cVar1 = FUN_00104a90(local_24), cVar1 != '\0')))) {
      if (((local_28 & 0x7f) != 0) || (((int)local_28 >> 8 & 0xffU) != 0)) {
        uVar2 = FUN_00115059(4,DAT_00123690);
        uVar3 = gettext("%s [-d] terminated abnormally");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar3,uVar2);
      }
      DAT_00123730 = DAT_00123730 + -1;
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_24;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar2 = FUN_00115059(4,DAT_00123690);
  uVar3 = gettext("waiting for %s [-d]");
  piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar4,uVar3,uVar2);
}




// Function: register_proc @ 0x4a08

void register_proc(long param_1)

{
  long lVar1;
  
  if (DAT_00123728 == 0) {
    DAT_00123728 = FUN_0011120d(0x2f,0,FUN_0010486b,FUN_0010489a,0);
    if (DAT_00123728 == 0) {
      FUN_0011865c();
    }
  }
  *(undefined1 *)(param_1 + 0xc) = 1;
  lVar1 = FUN_00111f4e(DAT_00123728,param_1);
  if (lVar1 == 0) {
    FUN_0011865c();
  }
  return;
}




// Function: delete_proc @ 0x4a90

bool delete_proc(undefined4 param_1)

{
  long lVar1;
  long in_FS_OFFSET;
  undefined1 local_28 [8];
  undefined4 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = param_1;
  lVar1 = FUN_00111fbe(DAT_00123728,local_28);
  if (lVar1 != 0) {
    *(undefined1 *)(lVar1 + 0xc) = 2;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar1 != 0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wait_proc @ 0x4aff

void wait_proc(undefined4 param_1)

{
  char cVar1;
  
  cVar1 = FUN_00104a90(param_1);
  if (cVar1 != '\0') {
    FUN_001048cf(param_1);
  }
  return;
}




// Function: reap_exited @ 0x4b29

void reap_exited(void)

{
  int iVar1;
  
  do {
    if (DAT_00123730 < 1) {
      return;
    }
    iVar1 = FUN_001048cf(0);
  } while (iVar1 != 0);
  return;
}




// Function: reap_some @ 0x4b4f

void reap_some(void)

{
  FUN_001048cf(0xffffffff);
  FUN_00104b29();
  return;
}




// Function: reap_all @ 0x4b69

void reap_all(void)

{
  while (0 < DAT_00123730) {
    FUN_001048cf(0xffffffff);
  }
  return;
}




// Function: cleanup @ 0x4b8b

void cleanup(void)

{
  undefined8 local_10;
  
  for (local_10 = DAT_00123720; local_10 != (undefined8 *)0x0; local_10 = (undefined8 *)*local_10) {
    unlink((char *)((long)local_10 + 0xd));
  }
  DAT_00123720 = (undefined8 *)0x0;
  return;
}




// Function: exit_cleanup @ 0x4bd4

void exit_cleanup(void)

{
  long in_FS_OFFSET;
  undefined1 local_98 [136];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00123720 != 0) {
    FUN_001047f1(local_98);
    FUN_00104b8b();
    FUN_00104833(local_98);
  }
  FUN_0010fa34();
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: create_temp_file @ 0x4c3d

undefined8 * create_temp_file(int *param_1,char param_2)

{
  int iVar1;
  char *__s;
  int iVar2;
  size_t __n;
  void *__dest;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined8 *local_c8;
  undefined1 local_a8 [136];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = *(char **)(DAT_001237a0 * 8 + DAT_00123668);
  __n = strlen(__s);
  local_c8 = (undefined8 *)FUN_00118024(__n + 0x20 & 0xfffffffffffffff8);
  __dest = (void *)((long)local_c8 + 0xd);
  memcpy(__dest,__s,__n);
  memcpy((void *)(__n + (long)__dest),"/sortXXXXXX",0xc);
  *local_c8 = 0;
  DAT_001237a0 = DAT_001237a0 + 1;
  if (DAT_001237a0 == DAT_00123670) {
    DAT_001237a0 = 0;
  }
  FUN_001047f1(local_a8);
  iVar2 = FUN_0011704d(__dest,0x80000);
  if (-1 < iVar2) {
    *(undefined8 **)PTR_DAT_00123100 = local_c8;
    PTR_DAT_00123100 = (undefined *)local_c8;
  }
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  FUN_00104833(local_a8);
  piVar3 = __errno_location();
  *piVar3 = iVar1;
  if (iVar2 < 0) {
    if (param_2 == '\x01') {
      piVar3 = __errno_location();
      if (*piVar3 == 0x18) {
        free(local_c8);
        local_c8 = (undefined8 *)0x0;
        goto LAB_00104e56;
      }
    }
    uVar4 = FUN_00115059(4,__s);
    uVar5 = gettext("cannot create temporary file in %s");
    piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar3,uVar5,uVar4);
  }
LAB_00104e56:
  *param_1 = iVar2;
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_c8;
}




// Function: get_outstatus @ 0x4e8c

undefined * get_outstatus(void)

{
  int iVar1;
  int *piVar2;
  undefined *puVar3;
  
  if (DAT_001237a8 == 0) {
    iVar1 = fstat(1,(stat *)&DAT_001237c0);
    if (iVar1 == 0) {
      DAT_001237a8 = -1;
    }
    else {
      piVar2 = __errno_location();
      DAT_001237a8 = *piVar2;
    }
  }
  if (DAT_001237a8 < 0) {
    puVar3 = &DAT_001237c0;
  }
  else {
    puVar3 = (undefined *)0x0;
  }
  return puVar3;
}




// Function: stream_open @ 0x4ee4

FILE * stream_open(char *param_1,char *param_2)

{
  int iVar1;
  int *piVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  FILE *local_28;
  
  if (*param_2 == 'r') {
    iVar1 = strcmp(param_1,"-");
    if (iVar1 == 0) {
      DAT_00123683 = 1;
      local_28 = stdin;
    }
    else {
      iVar1 = open(param_1,0x80000);
      if (iVar1 < 0) {
        local_28 = (FILE *)0x0;
      }
      else {
        local_28 = fdopen(iVar1,param_2);
      }
    }
    FUN_0010fed8(local_28,2);
  }
  else {
    if (*param_2 != 'w') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open");
    }
    if ((param_1 != (char *)0x0) && (iVar1 = ftruncate(1,0), iVar1 != 0)) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      lVar3 = FUN_00104e8c();
      if ((lVar3 == 0) || ((*(uint *)(lVar3 + 0x18) & 0xf000) == 0x8000)) {
        uVar4 = FUN_001151eb(0,3,param_1);
        uVar5 = gettext("%s: error truncating");
                    /* WARNING: Subroutine does not return */
        error(2,iVar1,uVar5,uVar4);
      }
    }
    local_28 = stdout;
  }
  return local_28;
}




// Function: xfopen @ 0x505e

long xfopen(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_00104ee4(param_1,param_2);
  if (lVar1 == 0) {
    uVar2 = gettext("open failed");
    FUN_00104464(uVar2,param_1);
  }
  return lVar1;
}




// Function: xfclose @ 0x50b7

void xfclose(FILE *param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = fileno(param_1);
  if (iVar1 == 0) {
    clearerr_unlocked(param_1);
  }
  else if (iVar1 == 1) {
    iVar1 = fflush_unlocked(param_1);
    if (iVar1 != 0) {
      uVar2 = gettext("fflush failed");
      FUN_00104464(uVar2,param_2);
    }
  }
  else {
    iVar1 = FUN_0010ff17(param_1);
    if (iVar1 != 0) {
      uVar2 = gettext("close failed");
      FUN_00104464(uVar2,param_2);
    }
  }
  return;
}




// Function: move_fd @ 0x515d

void move_fd(int param_1,int param_2)

{
  if (param_1 != param_2) {
    dup2(param_1,param_2);
    close(param_1);
  }
  return;
}




// Function: pipe_fork @ 0x5193

int pipe_fork(int *param_1,long param_2)

{
  undefined8 uVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  long local_c8;
  __pid_t local_b0;
  double local_a8;
  undefined1 local_98 [136];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = DAT_0011d138;
  iVar2 = FUN_0011311d(param_1,0x80000);
  if (iVar2 < 0) {
    local_b0 = -1;
  }
  else {
    local_c8 = param_2;
    if (DAT_00123024 + 1U < DAT_00123730) {
      FUN_00104b4f();
      local_c8 = param_2;
    }
    while (local_c8 != 0) {
      FUN_001047f1(local_98);
      uVar1 = DAT_00123720;
      DAT_00123720 = 0;
      local_b0 = fork();
      piVar3 = __errno_location();
      iVar2 = *piVar3;
      if (local_b0 != 0) {
        DAT_00123720 = uVar1;
      }
      FUN_00104833(local_98);
      piVar3 = __errno_location();
      *piVar3 = iVar2;
      if ((-1 < local_b0) || (piVar3 = __errno_location(), *piVar3 != 0xb)) break;
      FUN_00118863(local_a8);
      local_a8 = local_a8 + local_a8;
      FUN_00104b29();
      local_c8 = local_c8 + -1;
    }
    if (local_b0 < 0) {
      piVar3 = __errno_location();
      iVar2 = *piVar3;
      close(*param_1);
      close(param_1[1]);
      piVar3 = __errno_location();
      *piVar3 = iVar2;
    }
    else if (local_b0 == 0) {
      close(0);
      close(1);
    }
    else {
      DAT_00123730 = DAT_00123730 + 1;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_b0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: maybe_create_temp @ 0x5372

long maybe_create_temp(long *param_1,undefined1 param_2)

{
  undefined4 uVar1;
  int *piVar2;
  FILE *pFVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  int local_34;
  long local_30;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = FUN_00104c3d(&local_34,param_2);
  if (local_30 == 0) {
    lVar5 = 0;
  }
  else {
    *(undefined1 *)(local_30 + 0xc) = 0;
    if (DAT_00123690 != (char *)0x0) {
      uVar1 = FUN_00105193(&local_28,4);
      *(undefined4 *)(local_30 + 8) = uVar1;
      if (*(int *)(local_30 + 8) < 1) {
        if (*(int *)(local_30 + 8) == 0) {
          close(local_24);
          FUN_0010515d(local_34,1);
          FUN_0010515d(local_28,0);
          execlp(DAT_00123690,DAT_00123690,0);
          piVar2 = __errno_location();
          FUN_0010439b(*piVar2,"couldn\'t execute compress program");
        }
      }
      else {
        close(local_34);
        close(local_28);
        local_34 = local_24;
        FUN_00104a08(local_30);
      }
    }
    pFVar3 = fdopen(local_34,"w");
    *param_1 = (long)pFVar3;
    lVar5 = local_30;
    if (*param_1 == 0) {
      lVar5 = local_30 + 0xd;
      uVar4 = gettext("couldn\'t create temporary file");
      FUN_00104464(uVar4,lVar5);
      lVar5 = local_30;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: create_temp @ 0x54f9

void create_temp(undefined8 param_1)

{
  FUN_00105372(param_1,0);
  return;
}




// Function: open_temp @ 0x551c

FILE * open_temp(long param_1)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  long in_FS_OFFSET;
  FILE *local_30;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = (FILE *)0x0;
  if (*(char *)(param_1 + 0xc) == '\x01') {
    FUN_00104aff(*(undefined4 *)(param_1 + 8));
  }
  iVar1 = open((char *)(param_1 + 0xd),0);
  if (iVar1 < 0) {
    local_30 = (FILE *)0x0;
  }
  else {
    iVar2 = FUN_00105193(&local_28,9);
    if (iVar2 == -1) {
      piVar5 = __errno_location();
      if (*piVar5 != 0x18) {
        uVar3 = FUN_00115059(4,DAT_00123690);
        uVar4 = gettext("couldn\'t create process for %s -d");
        piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(2,*piVar5,uVar4,uVar3);
      }
      close(iVar1);
      piVar5 = __errno_location();
      *piVar5 = 0x18;
    }
    else {
      if (iVar2 == 0) {
        close(local_28);
        FUN_0010515d(iVar1,0);
        FUN_0010515d(local_24,1);
        execlp(DAT_00123690,DAT_00123690,&DAT_0011c49a,0);
        piVar5 = __errno_location();
        FUN_0010439b(*piVar5,"couldn\'t execute compress program (with -d)");
      }
      *(int *)(param_1 + 8) = iVar2;
      FUN_00104a08(param_1);
      close(iVar1);
      close(local_24);
      local_30 = fdopen(local_28,"r");
      if (local_30 == (FILE *)0x0) {
        piVar5 = __errno_location();
        iVar1 = *piVar5;
        close(local_28);
        piVar5 = __errno_location();
        *piVar5 = iVar1;
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_30;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: add_temp_dir @ 0x570f

void add_temp_dir(undefined8 param_1)

{
  long lVar1;
  
  if (DAT_00123670 == DAT_00123678) {
    DAT_00123668 = FUN_00118221(DAT_00123668,&DAT_00123678,8);
  }
  lVar1 = DAT_00123670 * 8;
  DAT_00123670 = DAT_00123670 + 1;
  *(undefined8 *)(DAT_00123668 + lVar1) = param_1;
  return;
}




// Function: zaptemp @ 0x5782

void zaptemp(char *param_1)

{
  int iVar1;
  long *__ptr;
  long lVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  long *local_c0;
  undefined1 local_a8 [136];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_c0 = &DAT_00123720;
  __ptr = local_c0;
  do {
    local_c0 = __ptr;
    __ptr = (long *)*local_c0;
  } while (param_1 != (char *)((long)__ptr + 0xd));
  if (*(char *)((long)__ptr + 0xc) == '\x01') {
    FUN_00104aff((int)__ptr[1]);
  }
  lVar2 = *__ptr;
  FUN_001047f1(local_a8);
  iVar3 = unlink(param_1);
  piVar4 = __errno_location();
  iVar1 = *piVar4;
  *local_c0 = lVar2;
  FUN_00104833(local_a8);
  if (iVar3 == 0) {
    if (lVar2 == 0) {
      PTR_DAT_00123100 = (undefined *)local_c0;
    }
    free(__ptr);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar5 = FUN_001151eb(0,3,param_1);
  uVar6 = gettext("warning: cannot remove: %s");
                    /* WARNING: Subroutine does not return */
  error(0,iVar1,uVar6,uVar5);
}




// Function: struct_month_cmp @ 0x590b

void struct_month_cmp(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: inittables @ 0x594a

void inittables(void)

{
  ushort *puVar1;
  undefined1 uVar2;
  char cVar3;
  byte bVar4;
  int iVar5;
  char *__s;
  size_t sVar6;
  undefined *puVar7;
  ushort **ppuVar8;
  ulong local_48;
  ulong local_40;
  long local_38;
  
  local_48 = 0;
  do {
    if (0xff < local_48) {
      if (DAT_0012322a != '\0') {
        for (local_48 = 0; local_48 < 0xc; local_48 = local_48 + 1) {
          __s = nl_langinfo((int)local_48 + 0x2000e);
          sVar6 = strlen(__s);
          puVar7 = (undefined *)FUN_00118024(sVar6 + 1);
          (&PTR_DAT_00123040)[local_48 * 2] = puVar7;
          *(int *)(&DAT_00123048 + local_48 * 0x10) = (int)local_48 + 1;
          local_38 = 0;
          for (local_40 = 0; local_40 < sVar6; local_40 = local_40 + 1) {
            ppuVar8 = __ctype_b_loc();
            puVar1 = *ppuVar8;
            bVar4 = FUN_00104042((int)__s[local_40]);
            if ((puVar1[bVar4] & 1) == 0) {
              bVar4 = FUN_00104042((int)__s[local_40]);
              puVar7[local_38] = (&DAT_00123560)[(int)(uint)bVar4];
              local_38 = local_38 + 1;
            }
          }
          puVar7[local_38] = 0;
        }
        qsort(&PTR_DAT_00123040,0xc,0x10,FUN_0010590b);
      }
      return;
    }
    uVar2 = FUN_00104051(local_48 & 0xff);
    (&DAT_00123260)[local_48] = uVar2;
    ppuVar8 = __ctype_b_loc();
    iVar5 = (int)local_48;
    (&DAT_00123360)[local_48] = ((*ppuVar8)[iVar5] & 0x4000) == 0;
    ppuVar8 = __ctype_b_loc();
    if (((*ppuVar8)[iVar5] & 8) == 0) {
      cVar3 = FUN_00104051(local_48 & 0xff);
      if (cVar3 == '\x01') goto LAB_001059f3;
      uVar2 = 1;
    }
    else {
LAB_001059f3:
      uVar2 = 0;
    }
    (&DAT_00123460)[local_48] = uVar2;
    iVar5 = toupper(iVar5);
    (&DAT_00123560)[local_48] = (char)iVar5;
    local_48 = local_48 + 1;
  } while( true );
}




// Function: specify_nmerge @ 0x5ba8

/* WARNING: Enum "__rlimit_resource": Some values do not have unique names */

void specify_nmerge(int param_1,char param_2,undefined8 param_3)

{
  undefined *puVar1;
  int iVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  int local_58;
  ulong local_50;
  rlimit local_48 [2];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = FUN_00118b54(param_3,0,10,&local_50,&DAT_0011b1db);
  iVar2 = getrlimit(RLIMIT_NOFILE,local_48);
  if (iVar2 == 0) {
    uVar3 = (int)local_48[0].rlim_cur - 3;
  }
  else {
    uVar3 = 0x11;
  }
  if (local_58 == 0) {
    DAT_00123024 = (uint)local_50;
    if ((local_50 & 0xffffffff) == local_50) {
      if (DAT_00123024 < 2) {
        uVar4 = FUN_001154a0(param_3);
        puVar1 = (&PTR_s_ignore_leading_blanks_00122560)[(long)param_1 * 4];
        uVar5 = gettext("invalid --%s argument %s");
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar5,puVar1,uVar4);
      }
      if (DAT_00123024 <= uVar3) goto code_r0x00105def;
      local_58 = 1;
    }
    else {
      local_58 = 1;
    }
  }
  if (local_58 == 1) {
    uVar4 = FUN_001154a0(param_3);
    puVar1 = (&PTR_s_ignore_leading_blanks_00122560)[(long)param_1 * 4];
    uVar5 = gettext("--%s argument %s too large");
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar5,puVar1,uVar4);
  }
  FUN_00118a33(local_58,param_1,(int)param_2,&PTR_s_ignore_leading_blanks_00122560,param_3);
code_r0x00105def:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: specify_sort_size @ 0x5e0c

/* WARNING: Removing unreachable block (ram,0x00106005) */

void specify_sort_size(undefined4 param_1,char param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  double dVar1;
  int local_2c;
  ulong local_28;
  char *local_20;
  double local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_2c = FUN_00118b54(param_3,&local_20,10,&local_28,"EgGkKmMPtTYZ");
  if ((local_2c == 0) && ((int)local_20[-1] - 0x30U < 10)) {
    if (local_28 < 0x40000000000000) {
      local_28 = local_28 << 10;
    }
    else {
      local_2c = 1;
    }
  }
  if (((local_2c == 2) && ((int)local_20[-1] - 0x30U < 10)) && (local_20[1] == '\0')) {
    if (*local_20 == '%') {
      dVar1 = (double)FUN_00112e84();
      local_18 = ((double)local_28 * dVar1) / DAT_0011d140;
      if (DAT_0011d148 <= local_18) {
        local_2c = 1;
      }
      else {
        if (DAT_0011d150 <= local_18) {
          local_28 = (long)(local_18 - DAT_0011d150) ^ 0x8000000000000000;
        }
        else {
          local_28 = (ulong)local_18;
        }
        local_2c = 0;
      }
    }
    else if (*local_20 == 'b') {
      local_2c = 0;
    }
  }
  if (local_2c == 0) {
    if ((DAT_00123660 <= local_28) &&
       (DAT_00123660 = local_28, local_28 <= (ulong)DAT_00123024 * 0x22)) {
      DAT_00123660 = (ulong)DAT_00123024 * 0x22;
    }
  }
  else {
    FUN_00118a33(local_2c,param_1,(int)param_2,&PTR_s_ignore_leading_blanks_00122560,param_3);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: specify_nthreads @ 0x6042

long specify_nthreads(undefined4 param_1,char param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00118b54(param_3,0,10,&local_18,&DAT_0011b1db);
  if (iVar1 == 1) {
    local_18 = -1;
  }
  else {
    if (iVar1 != 0) {
      FUN_00118a33(iVar1,param_1,(int)param_2,&PTR_s_ignore_leading_blanks_00122560,param_3);
    }
    if (local_18 == 0) {
      uVar2 = gettext("number in parallel must be nonzero");
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar2);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_18;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: default_sort_size @ 0x610d

/* WARNING: Enum "__rlimit_resource": Some values do not have unique names */

ulong default_sort_size(void)

{
  int iVar1;
  long in_FS_OFFSET;
  double dVar2;
  double dVar3;
  ulong local_48;
  rlimit local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0xffffffffffffffff;
  iVar1 = getrlimit(RLIMIT_DATA,&local_28);
  if ((iVar1 == 0) && (local_28.rlim_cur != 0xffffffffffffffff)) {
    local_48 = local_28.rlim_cur;
  }
  iVar1 = getrlimit(RLIMIT_AS,&local_28);
  if ((iVar1 == 0) && (local_28.rlim_cur < local_48)) {
    local_48 = local_28.rlim_cur;
  }
  local_48 = local_48 >> 1;
  iVar1 = getrlimit(__RLIMIT_RSS,&local_28);
  if ((iVar1 == 0) && ((local_28.rlim_cur >> 4) * 0xf < local_48)) {
    local_48 = (local_28.rlim_cur >> 4) * 0xf;
  }
  dVar2 = (double)FUN_00112fab();
  dVar3 = (double)FUN_00112e84();
  if (dVar2 <= dVar3 / DAT_0011d158) {
    dVar2 = dVar3 / DAT_0011d158;
  }
  if (dVar3 * DAT_0011d160 < (double)local_48) {
    dVar3 = DAT_0011d160 * dVar3;
    if (DAT_0011d150 <= dVar3) {
      local_48 = (long)(dVar3 - DAT_0011d150) ^ 0x8000000000000000;
    }
    else {
      local_48 = (ulong)dVar3;
    }
  }
  if (dVar2 < (double)local_48) {
    if (DAT_0011d150 <= dVar2) {
      local_48 = (long)(dVar2 - DAT_0011d150) ^ 0x8000000000000000;
    }
    else {
      local_48 = (ulong)dVar2;
    }
  }
  if (local_48 <= (ulong)DAT_00123024 * 0x22) {
    local_48 = (ulong)DAT_00123024 * 0x22;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_48;
}




// Function: sort_buffer_size @ 0x637c

long sort_buffer_size(long param_1,ulong param_2,long param_3,ulong param_4,long param_5)

{
  undefined8 uVar1;
  int iVar2;
  undefined8 uVar3;
  ulong uVar4;
  long lVar5;
  long in_FS_OFFSET;
  bool bVar6;
  long local_e0;
  ulong local_d8;
  ulong local_d0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_e0 = param_5 + 2;
  for (local_d8 = 0; lVar5 = local_e0, local_d8 < param_4; local_d8 = local_d8 + 1) {
    if (local_d8 < param_2) {
      iVar2 = fileno(*(FILE **)(param_1 + local_d8 * 8));
      iVar2 = fstat(iVar2,&local_b8);
      bVar6 = iVar2 != 0;
    }
    else {
      iVar2 = strcmp(*(char **)(param_3 + local_d8 * 8),"-");
      if (iVar2 == 0) {
        iVar2 = fstat(0,&local_b8);
        bVar6 = iVar2 != 0;
      }
      else {
        iVar2 = stat(*(char **)(param_3 + local_d8 * 8),&local_b8);
        bVar6 = iVar2 != 0;
      }
    }
    if (bVar6) {
      uVar1 = *(undefined8 *)(param_3 + local_d8 * 8);
      uVar3 = gettext("stat failed");
      FUN_00104464(uVar3,uVar1);
    }
    if ((local_b8.st_mode & 0xf000) == 0x8000) {
      local_d0 = local_b8.st_size;
    }
    else {
      lVar5 = DAT_00123660;
      if (DAT_00123660 != 0) break;
      local_d0 = 0x20000;
    }
    if ((DAT_00123850 == 0) && (DAT_00123850 = DAT_00123660, DAT_00123660 == 0)) {
      DAT_00123850 = FUN_0010610d();
    }
    uVar4 = local_d0 * (param_5 + 1U) + 1;
    lVar5 = DAT_00123850;
    if ((uVar4 / (param_5 + 1U) != local_d0) || ((ulong)(DAT_00123850 - local_e0) <= uVar4)) break;
    local_e0 = local_e0 + uVar4;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: initbuf @ 0x6618

void initbuf(long *param_1,long param_2,ulong param_3)

{
  void *pvVar1;
  ulong local_20;
  
  local_20 = param_3;
  do {
    local_20 = (local_20 & 0xffffffffffffffe0) + 0x20;
    pvVar1 = malloc(local_20);
    *param_1 = (long)pvVar1;
    if (*param_1 != 0) goto code_r0x00106681;
    local_20 = local_20 >> 1;
  } while (param_2 + 1U < local_20);
  FUN_0011865c();
code_r0x00106681:
  param_1[5] = param_2;
  param_1[3] = local_20;
  param_1[2] = 0;
  param_1[4] = param_1[2];
  param_1[1] = param_1[4];
  *(undefined1 *)(param_1 + 6) = 0;
  return;
}




// Function: buffer_linelim @ 0x66d0

long buffer_linelim(long *param_1)

{
  return param_1[3] + *param_1;
}




// Function: begfield @ 0x66f4

char * begfield(undefined8 *param_1,long *param_2)

{
  long lVar1;
  long lVar2;
  byte bVar3;
  char *pcVar4;
  char *local_28;
  long local_20;
  
  local_28 = (char *)*param_1;
  pcVar4 = local_28 + param_1[1] + -1;
  local_20 = *param_2;
  lVar2 = param_2[1];
  if (DAT_00123020 == 0x80) {
    while ((local_28 < pcVar4 && (lVar1 = local_20 + -1, local_20 != 0))) {
      while ((local_28 < pcVar4 &&
             (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\0'))) {
        local_28 = local_28 + 1;
      }
      while ((local_20 = lVar1, local_28 < pcVar4 &&
             (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\x01'))) {
        local_28 = local_28 + 1;
      }
    }
  }
  else {
    while ((local_28 < pcVar4 && (lVar1 = local_20 + -1, local_20 != 0))) {
      for (; (local_28 < pcVar4 && (*local_28 != DAT_00123020)); local_28 = local_28 + 1) {
      }
      local_20 = lVar1;
      if (local_28 < pcVar4) {
        local_28 = local_28 + 1;
      }
    }
  }
  if ((char)param_2[6] != '\0') {
    while ((local_28 < pcVar4 &&
           (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\0'))) {
      local_28 = local_28 + 1;
    }
  }
  if (local_28 + lVar2 < pcVar4) {
    pcVar4 = local_28 + lVar2;
  }
  return pcVar4;
}




// Function: limfield @ 0x6898

char * limfield(undefined8 *param_1,long param_2)

{
  long lVar1;
  long lVar2;
  byte bVar3;
  char *pcVar4;
  char *pcVar5;
  char *local_28;
  long local_20;
  
  local_28 = (char *)*param_1;
  pcVar4 = local_28 + param_1[1] + -1;
  local_20 = *(long *)(param_2 + 0x10);
  lVar2 = *(long *)(param_2 + 0x18);
  if (lVar2 == 0) {
    local_20 = local_20 + 1;
  }
  if (DAT_00123020 == 0x80) {
    while ((local_28 < pcVar4 && (lVar1 = local_20 + -1, local_20 != 0))) {
      while ((local_28 < pcVar4 &&
             (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\0'))) {
        local_28 = local_28 + 1;
      }
      while ((local_20 = lVar1, local_28 < pcVar4 &&
             (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\x01'))) {
        local_28 = local_28 + 1;
      }
    }
  }
  else {
    while ((local_28 < pcVar4 && (lVar1 = local_20 + -1, local_20 != 0))) {
      for (; (local_28 < pcVar4 && (*local_28 != DAT_00123020)); local_28 = local_28 + 1) {
      }
      local_20 = lVar1;
      if ((local_28 < pcVar4) && ((lVar1 != 0 || (lVar2 != 0)))) {
        local_28 = local_28 + 1;
      }
    }
  }
  if (lVar2 != 0) {
    if (*(char *)(param_2 + 0x31) != '\0') {
      while ((local_28 < pcVar4 &&
             (bVar3 = FUN_00104042((int)*local_28), (&DAT_00123260)[(int)(uint)bVar3] != '\0'))) {
        local_28 = local_28 + 1;
      }
    }
    pcVar5 = local_28 + lVar2;
    local_28 = pcVar4;
    if (pcVar5 < pcVar4) {
      local_28 = pcVar5;
    }
  }
  return local_28;
}




// Function: fillbuf @ 0x6a5e

undefined8 fillbuf(long *param_1,FILE *param_2,undefined8 param_3)

{
  long *plVar1;
  char cVar2;
  byte bVar3;
  int iVar4;
  undefined8 uVar5;
  ulong uVar6;
  undefined1 *puVar7;
  long lVar8;
  long in_FS_OFFSET;
  ulong local_78;
  ulong local_70;
  char *local_68;
  long *local_60;
  ulong local_58;
  char *local_50;
  char *local_48;
  long *local_40;
  long local_38;
  long local_30;
  ulong local_28;
  size_t local_20;
  undefined1 *local_18;
  long local_10;
  
  cVar2 = DAT_00123010;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = DAT_00123688;
  local_38 = param_1[5];
  local_70 = DAT_00123018 - 0x22;
  if ((char)param_1[6] == '\0') {
    if (param_1[1] != param_1[4]) {
      memmove((void *)*param_1,(void *)((param_1[1] - param_1[4]) + *param_1),param_1[4]);
      param_1[1] = param_1[4];
      param_1[2] = 0;
    }
    do {
      local_68 = (char *)(param_1[1] + *param_1);
      local_30 = FUN_001066d0(param_1);
      local_60 = (long *)(local_30 + param_1[2] * -0x20);
      local_58 = (local_30 - param_1[2] * local_38) - (long)local_68;
      if (param_1[2] == 0) {
        local_50 = (char *)*param_1;
      }
      else {
        local_50 = (char *)(local_60[1] + *local_60);
      }
      do {
        if (local_58 <= local_38 + 1U) break;
        local_28 = (local_58 - 1) / (local_38 + 1U);
        local_20 = fread_unlocked(local_68,1,local_28,param_2);
        local_48 = local_68 + local_20;
        local_58 = local_58 - local_20;
        if (local_20 != local_28) {
          iVar4 = ferror_unlocked(param_2);
          if (iVar4 != 0) {
            uVar5 = gettext("read failed");
            FUN_00104464(uVar5,param_3);
          }
          iVar4 = feof_unlocked(param_2);
          if (iVar4 != 0) {
            *(undefined1 *)(param_1 + 6) = 1;
            if (local_48 == (char *)*param_1) {
              uVar5 = 0;
              goto LAB_00106f4e;
            }
            if ((local_50 != local_48) && (cVar2 != local_48[-1])) {
              *local_48 = cVar2;
              local_48 = local_48 + 1;
            }
          }
        }
        while (puVar7 = memchr(local_68,(int)cVar2,(long)local_48 - (long)local_68),
              local_18 = puVar7, puVar7 != (undefined1 *)0x0) {
          *puVar7 = 0;
          local_68 = puVar7 + 1;
          plVar1 = local_60 + -4;
          *plVar1 = (long)local_50;
          local_60[-3] = (long)local_68 - (long)local_50;
          uVar6 = local_60[-3];
          if ((ulong)local_60[-3] <= local_70) {
            uVar6 = local_70;
          }
          local_58 = local_58 - local_38;
          local_70 = uVar6;
          local_60 = plVar1;
          if (local_40 != (long *)0x0) {
            if (local_40[2] != -1) {
              puVar7 = (undefined1 *)FUN_00106898(plVar1,local_40);
            }
            local_60[3] = (long)puVar7;
            if (*local_40 == -1) {
              if ((char)local_40[6] != '\0') {
                while (bVar3 = FUN_00104042((int)*local_50),
                      (&DAT_00123260)[(int)(uint)bVar3] != '\0') {
                  local_50 = local_50 + 1;
                }
              }
              local_60[2] = (long)local_50;
            }
            else {
              lVar8 = FUN_001066f4(local_60,local_40);
              local_60[2] = lVar8;
            }
          }
          local_50 = local_68;
        }
        local_68 = local_48;
      } while ((char)param_1[6] == '\0');
      param_1[1] = (long)local_68 - *param_1;
      lVar8 = FUN_001066d0(param_1);
      param_1[2] = lVar8 - (long)local_60 >> 5;
      if (param_1[2] != 0) goto code_r0x00106ecc;
      local_78 = (ulong)param_1[3] >> 5;
      lVar8 = FUN_00118221(*param_1,&local_78,0x20);
      *param_1 = lVar8;
      param_1[3] = local_78 << 5;
    } while( true );
  }
  uVar5 = 0;
LAB_00106f4e:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
code_r0x00106ecc:
  param_1[4] = (long)local_68 - (long)local_50;
  DAT_00123018 = local_70 + 0x22;
  uVar5 = 1;
  goto LAB_00106f4e;
}




// Function: traverse_raw_number @ 0x6f64

char traverse_raw_number(long *param_1)

{
  char *pcVar1;
  char cVar2;
  bool bVar3;
  char local_13;
  char *local_10;
  
  local_13 = '\0';
  bVar3 = false;
  pcVar1 = (char *)*param_1;
  while( true ) {
    local_10 = pcVar1;
    pcVar1 = local_10 + 1;
    cVar2 = *local_10;
    if (9 < (int)cVar2 - 0x30U) break;
    if (local_13 < cVar2) {
      local_13 = cVar2;
    }
    bVar3 = *pcVar1 == DAT_00123224;
    if (bVar3) {
      pcVar1 = local_10 + 2;
    }
  }
  if (bVar3) {
    *param_1 = (long)(local_10 + -1);
  }
  else {
    local_10 = pcVar1;
    if (cVar2 == DAT_00123220) {
      while( true ) {
        pcVar1 = local_10 + 1;
        cVar2 = *local_10;
        local_10 = pcVar1;
        if (9 < (int)cVar2 - 0x30U) break;
        if (local_13 < cVar2) {
          local_13 = cVar2;
        }
      }
    }
    *param_1 = (long)(local_10 + -1);
  }
  return local_13;
}




// Function: find_unit_order @ 0x7042

int find_unit_order(char *param_1)

{
  char cVar1;
  char cVar2;
  int iVar3;
  long in_FS_OFFSET;
  byte *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = *param_1;
  local_18 = (byte *)(param_1 + (cVar1 == '-'));
  cVar2 = FUN_00106f64(&local_18);
  if (cVar2 < '1') {
    iVar3 = 0;
  }
  else {
    iVar3 = (int)(char)(&DAT_0011c5c0)[(int)(uint)*local_18];
    if (cVar1 == '-') {
      iVar3 = -iVar3;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: human_numcompare @ 0x70e2

ulong human_numcompare(char *param_1,char *param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  ulong uVar4;
  char *local_38;
  char *local_30;
  
  local_30 = param_1;
  while (bVar1 = FUN_00104042((int)*local_30), local_38 = param_2,
        (&DAT_00123260)[(int)(uint)bVar1] != '\0') {
    local_30 = local_30 + 1;
  }
  while (bVar1 = FUN_00104042((int)*local_38), (&DAT_00123260)[(int)(uint)bVar1] != '\0') {
    local_38 = local_38 + 1;
  }
  iVar2 = FUN_00107042(local_30);
  iVar3 = FUN_00107042(local_38);
  if (iVar2 - iVar3 == 0) {
    uVar4 = FUN_0011764c(local_30,local_38,(int)DAT_00123220,DAT_00123224);
  }
  else {
    uVar4 = (ulong)(uint)(iVar2 - iVar3);
  }
  return uVar4;
}




// Function: numcompare @ 0x71a3

void numcompare(char *param_1,char *param_2)

{
  byte bVar1;
  char *local_18;
  char *local_10;
  
  local_10 = param_1;
  while (bVar1 = FUN_00104042((int)*local_10), local_18 = param_2,
        (&DAT_00123260)[(int)(uint)bVar1] != '\0') {
    local_10 = local_10 + 1;
  }
  while (bVar1 = FUN_00104042((int)*local_18), (&DAT_00123260)[(int)(uint)bVar1] != '\0') {
    local_18 = local_18 + 1;
  }
  FUN_0011764c(local_10,local_18,(int)DAT_00123220,DAT_00123224);
  return;
}




// Function: nan_compare @ 0x7231

void nan_compare(char *param_1,char *param_2)

{
  long in_FS_OFFSET;
  undefined1 local_38 [16];
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  memset(local_38,0,0x10);
  strtold(param_1,(char **)0x0);
  memset(local_28,0,0x10);
  strtold(param_2,(char **)0x0);
  memcmp(local_38,local_28,0x10);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: general_numcompare @ 0x72d6

undefined8 general_numcompare(char *param_1,char *param_2)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  longdouble in_ST0;
  longdouble in_ST1;
  char *local_48;
  char *local_40;
  longdouble local_38;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  strtold(param_1,&local_48);
  local_38 = in_ST0;
  strtold(param_2,&local_40);
  if (param_1 == local_48) {
    if (param_2 == local_40) {
      uVar1 = 0;
    }
    else {
      uVar1 = 0xffffffff;
    }
  }
  else if (param_2 == local_40) {
    uVar1 = 1;
  }
  else if (in_ST1 <= local_38) {
    if (local_38 <= in_ST1) {
      if (in_ST1 == local_38) {
        uVar1 = 0;
      }
      else if (NAN(in_ST1) || NAN(in_ST1)) {
        if (NAN(local_38) || NAN(local_38)) {
          uVar1 = FUN_00107231(param_1,param_2);
        }
        else {
          uVar1 = 1;
        }
      }
      else {
        uVar1 = 0xffffffff;
      }
    }
    else {
      uVar1 = 1;
    }
  }
  else {
    uVar1 = 0xffffffff;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: getmonth @ 0x73f3

undefined4 getmonth(char *param_1,undefined8 *param_2)

{
  byte bVar1;
  byte bVar2;
  ulong uVar3;
  char *local_48;
  ulong local_38;
  ulong local_30;
  char *local_28;
  char *local_20;
  
  local_38 = 0;
  local_30 = 0xc;
  local_48 = param_1;
  while (bVar1 = FUN_00104042((int)*local_48), (&DAT_00123260)[(int)(uint)bVar1] != '\0') {
    local_48 = local_48 + 1;
  }
  do {
    uVar3 = local_30 + local_38 >> 1;
    local_28 = local_48;
    local_20 = (&PTR_DAT_00123040)[uVar3 * 2];
    while( true ) {
      if (*local_20 == '\0') {
        if (param_2 != (undefined8 *)0x0) {
          *param_2 = local_28;
        }
        return *(undefined4 *)(&DAT_00123048 + uVar3 * 0x10);
      }
      bVar1 = FUN_00104042((int)*local_28);
      bVar1 = FUN_00104042((int)(char)(&DAT_00123560)[(int)(uint)bVar1]);
      bVar2 = FUN_00104042((int)*local_20);
      if (bVar1 < bVar2) goto LAB_0010755a;
      bVar1 = FUN_00104042((int)*local_28);
      bVar1 = FUN_00104042((int)(char)(&DAT_00123560)[(int)(uint)bVar1]);
      bVar2 = FUN_00104042((int)*local_20);
      if (bVar2 < bVar1) break;
      local_28 = local_28 + 1;
      local_20 = local_20 + 1;
    }
    local_38 = uVar3 + 1;
    uVar3 = local_30;
LAB_0010755a:
    local_30 = uVar3;
    if (local_30 <= local_38) {
      return 0;
    }
  } while( true );
}




// Function: random_md5_state_init @ 0x7573

void random_md5_state_init(char *param_1)

{
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_00115668(param_1,0x10);
  if (lVar2 == 0) {
    pcVar4 = param_1;
    if (param_1 == (char *)0x0) {
      pcVar4 = "getrandom";
    }
    uVar3 = gettext("open failed");
    FUN_00104464(uVar3,pcVar4);
  }
  FUN_001159b4(lVar2,local_38,0x10);
  iVar1 = FUN_00115a0f(lVar2);
  if (iVar1 != 0) {
    uVar3 = gettext("close failed");
    FUN_00104464(uVar3,param_1);
  }
  FUN_0010fb2c(&DAT_00123740);
  FUN_0010fb4b(local_38,0x10,&DAT_00123740);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xstrxfrm @ 0x7668

size_t xstrxfrm(char *param_1,char *param_2,size_t param_3)

{
  int *piVar1;
  size_t sVar2;
  undefined8 uVar3;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  sVar2 = strxfrm(param_1,param_2,param_3);
  piVar1 = __errno_location();
  if (*piVar1 != 0) {
    uVar3 = gettext("string transformation failed");
    piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(0,*piVar1,uVar3);
  }
  return sVar2;
}




// Function: compare_random @ 0x7755

int compare_random(char *param_1,ulong param_2,char *param_3,ulong param_4)

{
  ulong uVar1;
  ulong uVar2;
  long lVar3;
  size_t sVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  ulong local_1108;
  char *local_1100;
  ulong local_10f8;
  char *local_10f0;
  int local_10e0;
  int local_10dc;
  char *local_10d8;
  ulong local_10d0;
  char *local_10c8;
  undefined1 local_1098 [16];
  undefined1 local_1088 [16];
  undefined8 local_1078;
  undefined8 local_1070;
  undefined8 local_1068;
  undefined8 local_1060;
  undefined8 local_1058;
  undefined8 local_1050;
  undefined8 local_1048;
  undefined8 local_1040;
  undefined8 local_1038;
  undefined8 local_1030;
  undefined8 local_1028;
  undefined4 local_1020;
  undefined8 local_101c;
  undefined8 local_1014;
  undefined4 uStack_100c;
  undefined4 local_1008;
  undefined4 uStack_1004;
  undefined4 uStack_1000;
  undefined8 local_ffc;
  undefined8 local_ff4;
  undefined8 local_fec;
  undefined8 local_fe4;
  undefined8 local_fdc;
  undefined8 local_fd4;
  undefined8 local_fcc;
  undefined4 local_fc4;
  char local_fb8 [4008];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_10e0 = 0;
  local_10d8 = local_fb8;
  local_10d0 = 4000;
  local_10c8 = (char *)0x0;
  local_101c = DAT_00123740;
  local_1014 = DAT_00123748;
  uStack_100c = (undefined4)DAT_00123750;
  local_1008 = (undefined4)((ulong)DAT_00123750 >> 0x20);
  uStack_1004 = (undefined4)DAT_00123758;
  uStack_1000 = (undefined4)((ulong)DAT_00123758 >> 0x20);
  local_ffc = DAT_00123760;
  local_ff4 = DAT_00123768;
  local_fec = DAT_00123770;
  local_fe4 = DAT_00123778;
  local_fdc = DAT_00123780;
  local_fd4 = DAT_00123788;
  local_fcc = DAT_00123790;
  local_fc4 = DAT_00123798;
  local_1078 = DAT_00123740;
  local_1070 = DAT_00123748;
  local_1068 = DAT_00123750;
  local_1060 = DAT_00123758;
  local_1058 = DAT_00123760;
  local_1050 = DAT_00123768;
  local_1048 = DAT_00123770;
  local_1040 = DAT_00123778;
  local_1038 = DAT_00123780;
  local_1030 = DAT_00123788;
  local_1028 = DAT_00123790;
  local_1020 = DAT_00123798;
  local_1108 = param_4;
  local_1100 = param_3;
  local_10f8 = param_2;
  local_10f0 = param_1;
  if (DAT_00123229 != '\0') {
    param_1 = param_1 + param_2;
    param_3 = param_3 + param_4;
    while( true ) {
      uVar1 = (param_2 + param_4) * 3 + 2;
      if (local_10d0 < uVar1) {
        uVar2 = local_10d0 * 3 >> 1;
        local_10d0 = uVar1;
        if (uVar1 < uVar2) {
          local_10d0 = uVar2;
        }
        free(local_10c8);
        local_10c8 = malloc(local_10d0);
        local_10d8 = local_10c8;
        if (local_10c8 == (char *)0x0) {
          local_10d8 = local_fb8;
          local_10d0 = 4000;
        }
      }
      if (local_10f0 < param_1) {
        lVar3 = FUN_00107668(local_10d8,local_10f0,local_10d0);
        local_10f8 = lVar3 + 1;
      }
      else {
        local_10f8 = 0;
      }
      if (local_1100 < param_3) {
        if (local_10d0 < local_10f8) {
          lVar3 = 0;
          pcVar5 = (char *)0x0;
        }
        else {
          lVar3 = local_10d0 - local_10f8;
          pcVar5 = local_10d8 + local_10f8;
        }
        lVar3 = FUN_00107668(pcVar5,local_1100,lVar3);
        local_1108 = lVar3 + 1;
      }
      else {
        local_1108 = 0;
      }
      if ((local_10d0 < local_10f8) || (local_10d0 < local_1108 + local_10f8)) {
        local_10d0 = local_1108 + local_10f8;
        if (local_10d0 < 0x5555555555555555) {
          local_10d0 = local_10d0 * 3 >> 1;
        }
        free(local_10c8);
        local_10d8 = (char *)FUN_00118024(local_10d0);
        if (local_10f0 < param_1) {
          strxfrm(local_10d8,local_10f0,local_10f8);
        }
        local_10c8 = local_10d8;
        if (local_1100 < param_3) {
          strxfrm(local_10d8 + local_10f8,local_1100,local_1108);
        }
      }
      if (local_10f0 < param_1) {
        sVar4 = strlen(local_10f0);
        local_10f0 = local_10f0 + sVar4 + 1;
      }
      if (local_1100 < param_3) {
        sVar4 = strlen(local_1100);
        local_1100 = local_1100 + sVar4 + 1;
      }
      if ((param_1 <= local_10f0) && (param_3 <= local_1100)) break;
      FUN_0010fb4b(local_10d8,local_10f8,&local_1078);
      FUN_0010fb4b(local_10d8 + local_10f8,local_1108,&local_101c);
      if (local_10e0 == 0) {
        uVar1 = local_1108;
        if (local_10f8 < local_1108) {
          uVar1 = local_10f8;
        }
        local_10e0 = memcmp(local_10d8,local_10d8 + local_10f8,uVar1);
        if (local_10e0 == 0) {
          local_10e0 = (uint)(local_1108 < local_10f8) - (uint)(local_10f8 < local_1108);
        }
      }
    }
    local_10f0 = local_10d8;
    local_1100 = local_10d8 + local_10f8;
  }
  FUN_0010fb4b(local_10f0,local_10f8,&local_1078);
  FUN_0010fbaf(&local_1078,local_1098);
  FUN_0010fb4b(local_1100,local_1108,&local_101c);
  FUN_0010fbaf(&local_101c,local_1088);
  local_10dc = memcmp(local_1098,local_1088,0x10);
  if (local_10dc == 0) {
    if (local_10e0 == 0) {
      uVar1 = local_1108;
      if (local_10f8 < local_1108) {
        uVar1 = local_10f8;
      }
      local_10e0 = memcmp(local_10f0,local_1100,uVar1);
      if (local_10e0 == 0) {
        local_10e0 = (uint)(local_1108 < local_10f8) - (uint)(local_10f8 < local_1108);
      }
    }
    local_10dc = local_10e0;
  }
  free(local_10c8);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_10dc;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: debug_width @ 0x7ee4

long debug_width(char *param_1,char *param_2)

{
  int iVar1;
  char *local_20;
  long local_10;
  
  iVar1 = FUN_0011293d(param_1,(long)param_2 - (long)param_1,0);
  local_10 = (long)iVar1;
  for (local_20 = param_1; local_20 < param_2; local_20 = local_20 + 1) {
    local_10 = local_10 + (int)(uint)(*local_20 == '\t');
  }
  return local_10;
}




// Function: mark_key @ 0x7f4c

void mark_key(long param_1,long param_2)

{
  char *__format;
  long local_18;
  long local_10;
  
  local_10 = param_1;
  while( true ) {
    if (local_10 == 0) break;
    putchar_unlocked(0x20);
    local_10 = local_10 + -1;
  }
  local_18 = param_2;
  if (param_2 == 0) {
    __format = (char *)gettext("^ no match for key\n");
    printf(__format);
  }
  else {
    do {
      putchar_unlocked(0x5f);
      local_18 = local_18 + -1;
    } while (local_18 != 0);
    putchar_unlocked(10);
  }
  return;
}




// Function: key_numeric @ 0x7fc5

undefined4 key_numeric(long param_1)

{
  undefined4 uVar1;
  
  if (((*(char *)(param_1 + 0x32) == '\0') && (*(char *)(param_1 + 0x34) == '\0')) &&
     (*(char *)(param_1 + 0x35) == '\0')) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: debug_key @ 0x8002

void debug_key(undefined8 *param_1,long *param_2)

{
  byte bVar1;
  char cVar2;
  byte bVar3;
  int iVar4;
  long in_FS_OFFSET;
  byte *local_60;
  byte *local_58;
  byte *local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined8 local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = (byte *)*param_1;
  local_48 = local_50 + param_1[1] + -1;
  local_40 = local_50;
  if (param_2 != (long *)0x0) {
    if (*param_2 != -1) {
      local_50 = (byte *)FUN_001066f4(param_1,param_2);
    }
    if (param_2[2] != -1) {
      local_48 = (byte *)FUN_00106898(param_1,param_2);
    }
    if ((((char)param_2[6] == '\0') || (*param_2 != -1)) &&
       (*(char *)((long)param_2 + 0x36) == '\0')) {
      cVar2 = FUN_00107fc5(param_2);
      if (cVar2 == '\0') goto LAB_00108246;
    }
    bVar1 = *local_48;
    *local_48 = 0;
    while( true ) {
      bVar3 = FUN_00104042((int)(char)*local_50);
      if ((&DAT_00123260)[(int)(uint)bVar3] == '\0') break;
      local_50 = local_50 + 1;
    }
    local_60 = local_50;
    if (local_48 < local_50) {
      local_60 = local_48;
    }
    else if (*(char *)((long)param_2 + 0x36) == '\0') {
      if (*(char *)((long)param_2 + 0x34) == '\0') {
        if ((*(char *)((long)param_2 + 0x32) == '\0') && (*(char *)((long)param_2 + 0x35) == '\0'))
        {
          local_60 = local_48;
        }
        else {
          if ((local_50 < local_48) && (*local_50 == 0x2d)) {
            iVar4 = 1;
          }
          else {
            iVar4 = 0;
          }
          local_58 = local_50 + iVar4;
          cVar2 = FUN_00106f64(&local_58);
          if ('/' < cVar2) {
            if ((*(char *)((long)param_2 + 0x35) == '\0') ||
               ((&DAT_0011c5c0)[(int)(uint)*local_58] == '\0')) {
              iVar4 = 0;
            }
            else {
              iVar4 = 1;
            }
            local_60 = local_58 + iVar4;
          }
        }
      }
      else {
        strtold((char *)local_50,(char **)&local_60);
      }
    }
    else {
      FUN_001073f3(local_50,&local_60);
    }
    *local_48 = bVar1;
    local_48 = local_60;
  }
LAB_00108246:
  local_38 = FUN_00107ee4(local_40,local_50);
  local_30 = FUN_00107ee4(local_50,local_48);
  FUN_00107f4c(local_38,local_30);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: debug_line @ 0x829e

void debug_line(undefined8 param_1)

{
  long local_10;
  
  local_10 = DAT_00123688;
  while( true ) {
    do {
      FUN_00108002(param_1,local_10);
      if (local_10 == 0) {
        return;
      }
      local_10 = *(long *)(local_10 + 0x40);
    } while (local_10 != 0);
    if (DAT_00123682 == '\x01') break;
    if (DAT_00123681 == '\x01') {
      return;
    }
  }
  return;
}




// Function: default_key_compare @ 0x8305

undefined4 default_key_compare(long param_1)

{
  char cVar1;
  
  if (((((*(long *)(param_1 + 0x20) == 0) && (*(long *)(param_1 + 0x28) == 0)) &&
       (*(char *)(param_1 + 0x30) != '\x01')) &&
      ((*(char *)(param_1 + 0x31) != '\x01' && (cVar1 = FUN_00107fc5(param_1), cVar1 != '\x01'))))
     && ((*(char *)(param_1 + 0x36) != '\x01' &&
         ((*(char *)(param_1 + 0x38) != '\x01' && (*(char *)(param_1 + 0x33) != '\x01')))))) {
    return 1;
  }
  return 0;
}




// Function: key_to_opts @ 0x839e

void key_to_opts(long param_1,undefined1 *param_2)

{
  undefined1 *local_18;
  
  if ((*(char *)(param_1 + 0x30) != '\0') || (local_18 = param_2, *(char *)(param_1 + 0x31) != '\0')
     ) {
    local_18 = param_2 + 1;
    *param_2 = 0x62;
  }
  if (*(undefined **)(param_1 + 0x20) == &DAT_00123460) {
    *local_18 = 100;
    local_18 = local_18 + 1;
  }
  if (*(long *)(param_1 + 0x28) != 0) {
    *local_18 = 0x66;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x34) != '\0') {
    *local_18 = 0x67;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x35) != '\0') {
    *local_18 = 0x68;
    local_18 = local_18 + 1;
  }
  if (*(undefined **)(param_1 + 0x20) == &DAT_00123360) {
    *local_18 = 0x69;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x36) != '\0') {
    *local_18 = 0x4d;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x32) != '\0') {
    *local_18 = 0x6e;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x33) != '\0') {
    *local_18 = 0x52;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x37) != '\0') {
    *local_18 = 0x72;
    local_18 = local_18 + 1;
  }
  if (*(char *)(param_1 + 0x38) != '\0') {
    *local_18 = 0x56;
    local_18 = local_18 + 1;
  }
  *local_18 = 0;
  return;
}




// Function: key_warnings @ 0x84fe

/* WARNING: Removing unreachable block (ram,0x00108c96) */
/* WARNING: Removing unreachable block (ram,0x00108cef) */
/* WARNING: Removing unreachable block (ram,0x00108ca3) */
/* WARNING: Removing unreachable block (ram,0x00108cb0) */
/* WARNING: Removing unreachable block (ram,0x00108ce5) */
/* WARNING: Removing unreachable block (ram,0x00108cde) */
/* WARNING: Removing unreachable block (ram,0x00108cea) */
/* WARNING: Removing unreachable block (ram,0x00108d00) */
/* WARNING: Removing unreachable block (ram,0x00108d04) */
/* WARNING: Removing unreachable block (ram,0x00108d66) */
/* WARNING: Removing unreachable block (ram,0x00108dc0) */
/* WARNING: Removing unreachable block (ram,0x00108d73) */
/* WARNING: Removing unreachable block (ram,0x00108d80) */
/* WARNING: Removing unreachable block (ram,0x00108db6) */
/* WARNING: Removing unreachable block (ram,0x00108daf) */
/* WARNING: Removing unreachable block (ram,0x00108dbb) */
/* WARNING: Removing unreachable block (ram,0x00108dd5) */
/* WARNING: Removing unreachable block (ram,0x00108dd9) */
/* WARNING: Removing unreachable block (ram,0x00108e2b) */
/* WARNING: Removing unreachable block (ram,0x00108e36) */
/* WARNING: Removing unreachable block (ram,0x00108e7d) */
/* WARNING: Removing unreachable block (ram,0x00108e86) */
/* WARNING: Removing unreachable block (ram,0x00108e91) */
/* WARNING: Removing unreachable block (ram,0x00108f7b) */
/* WARNING: Removing unreachable block (ram,0x00108f86) */

void key_warnings(undefined8 *param_1,char param_2)

{
  ulong uVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  bool bVar6;
  bool bVar7;
  bool bVar8;
  bool bVar9;
  char cVar10;
  char *pcVar11;
  char *pcVar12;
  char *pcVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined *puVar16;
  undefined8 uVar17;
  size_t sVar18;
  long in_FS_OFFSET;
  ulong *local_138;
  long local_130;
  ulong local_128;
  ulong local_120;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  ulong local_d8;
  ulong local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined1 local_a8 [32];
  char local_88 [48];
  char local_58 [56];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f8 = *param_1;
  local_f0 = param_1[1];
  local_e8 = param_1[2];
  local_e0 = param_1[3];
  local_d8 = param_1[4];
  local_d0 = param_1[5];
  local_c8 = param_1[6];
  local_c0 = param_1[7];
  local_b8 = param_1[8];
  local_130 = 1;
  bVar6 = false;
  bVar7 = false;
  local_138 = DAT_00123688;
  while( true ) {
    if (local_138 == (ulong *)0x0) {
      if ((bVar6) || (bVar7)) {
        local_58[0] = DAT_00123220;
        local_58[1] = 0;
        uVar15 = FUN_001154a0(local_58);
        if (DAT_00123220 == DAT_00123020) {
          puVar16 = &DAT_0011b1db;
        }
        else {
          puVar16 = (undefined *)gettext("note ");
        }
        uVar17 = gettext("%snumbers use %s as a decimal point in this locale");
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar17,puVar16,uVar15);
      }
      cVar10 = FUN_00108305(&local_f8);
      if ((cVar10 == '\x01') &&
         ((local_c8._7_1_ == '\0' ||
          (((DAT_00123681 == '\0' && (DAT_00123682 == '\0')) || (DAT_00123688 == (ulong *)0x0))))))
      {
        if (((local_c8._7_1_ != '\0') && (DAT_00123681 != '\x01')) &&
           ((DAT_00123682 != '\x01' && (DAT_00123688 != (ulong *)0x0)))) {
          uVar15 = gettext("option \'-r\' only applies to last-resort comparison");
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar15);
        }
        if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return;
      }
      if ((DAT_00123681 != '\x01') && (DAT_00123682 != '\x01')) {
        local_c8 = local_c8 & 0xffffffffffffff;
      }
      FUN_0010839e(&local_f8,local_58);
      sVar18 = strlen(local_58);
      uVar15 = FUN_00104094(sVar18);
      uVar15 = ngettext("option \'-%s\' is ignored","options \'-%s\' are ignored",uVar15);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar15,local_58);
    }
    cVar10 = FUN_00107fc5(local_138);
    if (cVar10 != '\0') {
      if (*(char *)((long)local_138 + 0x34) == '\0') {
        bVar6 = true;
      }
      else {
        bVar7 = true;
      }
    }
    if (*(char *)((long)local_138 + 0x39) != '\0') {
      local_128 = *local_138;
      uVar1 = local_138[2];
      if (local_128 == 0xffffffffffffffff) {
        local_128 = 0;
      }
      pcVar11 = (char *)FUN_0011287e(local_128,local_a8);
      pcVar12 = stpcpy(local_88,"+");
      pcVar11 = stpcpy(pcVar12,pcVar11);
      pcVar12 = (char *)FUN_0011287e(local_128 + 1,local_a8);
      pcVar13 = stpcpy(local_58,"-k ");
      pcVar12 = stpcpy(pcVar13,pcVar12);
      if (local_138[2] != 0xffffffffffffffff) {
        pcVar13 = (char *)FUN_0011287e(uVar1 + 1,local_a8);
        pcVar11 = stpcpy(pcVar11," -");
        stpcpy(pcVar11,pcVar13);
        pcVar11 = (char *)FUN_0011287e(uVar1 + (local_138[3] == 0xffffffffffffffff) + 1,local_a8);
        pcVar12 = stpcpy(pcVar12,",");
        stpcpy(pcVar12,pcVar11);
      }
      uVar15 = FUN_00115473(1,local_58);
      uVar17 = FUN_00115473(0,local_88);
      uVar14 = gettext("obsolescent key %s used; consider %s instead");
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar14,uVar17,uVar15);
    }
    if ((*local_138 == 0xffffffffffffffff) || (*local_138 <= local_138[2])) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    if (bVar8) {
      uVar15 = gettext("key %lu has zero width and will be ignored");
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar15,local_130);
    }
    cVar10 = FUN_00107fc5(local_138);
    if ((cVar10 == '\0') && (*(char *)((long)local_138 + 0x36) == '\0')) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    if ((local_138[2] == 0) && (local_138[3] != 0)) {
      bVar9 = true;
    }
    else {
      bVar9 = false;
    }
    if ((((param_2 != '\x01') && (DAT_00123020 == 0x80)) && (!bVar9)) &&
       (((((char)local_138[6] != '\x01' && (!bVar8)) ||
         (((char)local_138[6] != '\x01' && (local_138[1] != 0)))) ||
        ((*(char *)((long)local_138 + 0x31) != '\x01' && (local_138[3] != 0)))))) break;
    if ((param_2 != '\x01') && (cVar10 = FUN_00107fc5(local_138), cVar10 != '\0')) {
      local_120 = *local_138 + 1;
      if (local_120 == 0) {
        local_120 = *local_138 + 2;
      }
      if ((local_138[2] == 0xffffffffffffffff) || (local_120 < local_138[2] + 1)) {
        uVar15 = gettext("key %lu is numeric and spans multiple fields");
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar15,local_130);
      }
    }
    uVar1 = local_c8;
    if ((local_d8 != 0) && (local_d8 == local_138[4])) {
      local_d8 = 0;
    }
    if ((local_d0 != 0) && (local_d0 == local_138[5])) {
      local_d0 = 0;
    }
    bVar2 = ((byte)local_138[6] ^ 1) & (byte)local_c8;
    bVar3 = (*(byte *)((long)local_138 + 0x31) ^ 1) & local_c8._1_1_;
    local_c8._6_1_ = SUB81(uVar1,6);
    local_c8._7_1_ = SUB81(uVar1,7);
    local_c8._2_1_ = SUB81(uVar1,2);
    bVar4 = (*(byte *)((long)local_138 + 0x32) ^ 1) & local_c8._2_1_;
    local_c8._4_1_ = SUB81(uVar1,4);
    local_c8._5_1_ = SUB81(uVar1,5);
    local_c8._3_1_ = SUB81(uVar1,3);
    bVar5 = (*(byte *)((long)local_138 + 0x33) ^ 1) & local_c8._3_1_;
    local_c8 = CONCAT44(CONCAT22(CONCAT11(local_c8._7_1_,
                                          ((*(byte *)((long)local_138 + 0x36) ^ 1) & local_c8._6_1_)
                                          != 0),
                                 CONCAT11(((*(byte *)((long)local_138 + 0x35) ^ 1) & local_c8._5_1_)
                                          != 0,((*(byte *)((long)local_138 + 0x34) ^ 1) &
                                               local_c8._4_1_) != 0)),
                        CONCAT13(bVar5 != 0,CONCAT12(bVar4 != 0,CONCAT11(bVar3 != 0,bVar2 != 0))));
    local_c0 = CONCAT71(local_c0._1_7_,(((byte)local_138[7] ^ 1) & (byte)local_c0) != 0);
    local_c8 = CONCAT17(((*(byte *)((long)local_138 + 0x37) ^ 1) & local_c8._7_1_) != 0,
                        (undefined7)local_c8);
    local_138 = (ulong *)local_138[8];
    local_130 = local_130 + 1;
  }
  uVar15 = gettext("leading blanks are significant in key %lu; consider also specifying \'b\'");
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar15,local_130);
}




// Function: keycompare @ 0x9116

uint keycompare(long *param_1,long *param_2)

{
  long lVar1;
  long lVar2;
  char cVar3;
  byte bVar4;
  byte bVar5;
  int iVar6;
  int iVar7;
  char *pcVar8;
  char *pcVar9;
  ulong uVar10;
  ulong uVar11;
  ulong uVar12;
  long in_FS_OFFSET;
  char local_104e;
  char local_104d;
  uint local_104c;
  long *local_1048;
  char *local_1040;
  char *local_1038;
  char *local_1030;
  char *local_1028;
  char *local_1020;
  char *local_1018;
  ulong local_1010;
  ulong local_1008;
  char *local_1000;
  ulong local_ff8;
  char local_fc8 [4008];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_1048 = DAT_00123688;
  local_1040 = (char *)param_1[2];
  local_1038 = (char *)param_2[2];
  local_1030 = (char *)param_1[3];
  local_1028 = (char *)param_2[3];
  do {
    lVar1 = local_1048[5];
    lVar2 = local_1048[4];
    pcVar8 = local_1040;
    if (local_1040 <= local_1030) {
      pcVar8 = local_1030;
    }
    pcVar9 = local_1038;
    if (local_1038 <= local_1028) {
      pcVar9 = local_1028;
    }
    uVar10 = (long)pcVar8 - (long)local_1040;
    uVar11 = (long)pcVar9 - (long)local_1038;
    if ((((DAT_00123229 == '\0') && (cVar3 = FUN_00107fc5(local_1048), cVar3 == '\0')) &&
        (*(char *)((long)local_1048 + 0x36) == '\0')) &&
       ((*(char *)((long)local_1048 + 0x33) == '\0' && ((char)local_1048[7] == '\0')))) {
      if (lVar2 != 0) {
        if (lVar1 == 0) {
          while( true ) {
            while ((local_1040 < pcVar8 &&
                   (bVar5 = FUN_00104042((int)*local_1040), *(char *)(lVar2 + (ulong)bVar5) != '\0')
                   )) {
              local_1040 = local_1040 + 1;
            }
            while ((local_1038 < pcVar9 &&
                   (bVar5 = FUN_00104042((int)*local_1038), *(char *)(lVar2 + (ulong)bVar5) != '\0')
                   )) {
              local_1038 = local_1038 + 1;
            }
            if ((pcVar8 <= local_1040) || (pcVar9 <= local_1038)) break;
            bVar5 = FUN_00104042((int)*local_1040);
            bVar4 = FUN_00104042((int)*local_1038);
            local_104c = (uint)bVar5 - (uint)bVar4;
            if (local_104c != 0) goto LAB_00109df1;
            local_1040 = local_1040 + 1;
            local_1038 = local_1038 + 1;
          }
          local_104c = (uint)(local_1040 < pcVar8) - (uint)(local_1038 < pcVar9);
        }
        else {
          while( true ) {
            while ((local_1040 < pcVar8 &&
                   (bVar5 = FUN_00104042((int)*local_1040), *(char *)(lVar2 + (ulong)bVar5) != '\0')
                   )) {
              local_1040 = local_1040 + 1;
            }
            while ((local_1038 < pcVar9 &&
                   (bVar5 = FUN_00104042((int)*local_1038), *(char *)(lVar2 + (ulong)bVar5) != '\0')
                   )) {
              local_1038 = local_1038 + 1;
            }
            if ((pcVar8 <= local_1040) || (pcVar9 <= local_1038)) break;
            bVar5 = FUN_00104042((int)*local_1040);
            bVar5 = FUN_00104042((int)*(char *)(lVar1 + (ulong)bVar5));
            bVar4 = FUN_00104042((int)*local_1038);
            bVar4 = FUN_00104042((int)*(char *)(lVar1 + (ulong)bVar4));
            local_104c = (uint)bVar5 - (uint)bVar4;
            if (local_104c != 0) goto LAB_00109df1;
            local_1040 = local_1040 + 1;
            local_1038 = local_1038 + 1;
          }
          local_104c = (uint)(local_1040 < pcVar8) - (uint)(local_1038 < pcVar9);
        }
        goto LAB_00109bf2;
      }
      if (uVar10 == 0) {
        local_104c = -(uint)(uVar11 != 0);
        goto LAB_00109bf2;
      }
      if (uVar11 != 0) {
        if (lVar1 == 0) {
          uVar12 = uVar11;
          if (uVar10 < uVar11) {
            uVar12 = uVar10;
          }
          local_104c = memcmp(local_1040,local_1038,uVar12);
          if (local_104c != 0) goto LAB_00109df1;
        }
        else {
          while ((local_1040 < pcVar8 && (local_1038 < pcVar9))) {
            bVar5 = FUN_00104042((int)*local_1040);
            bVar5 = FUN_00104042((int)*(char *)(lVar1 + (ulong)bVar5));
            bVar4 = FUN_00104042((int)*local_1038);
            bVar4 = FUN_00104042((int)*(char *)(lVar1 + (ulong)bVar4));
            local_104c = (uint)bVar5 - (uint)bVar4;
            local_1040 = local_1040 + 1;
            local_1038 = local_1038 + 1;
            if (local_104c != 0) goto LAB_00109df1;
          }
        }
        if (uVar10 < uVar11) {
          local_104c = 0xffffffff;
        }
        else {
          local_104c = (uint)(uVar10 != uVar11);
        }
        goto LAB_00109bf2;
      }
      local_104c = 1;
LAB_00109df1:
      if (*(char *)((long)local_1048 + 0x37) != '\0') {
        local_104c = -local_104c;
      }
      goto LAB_00109e10;
    }
    if ((lVar2 == 0) && (lVar1 == 0)) {
      local_1020 = local_1040;
      local_104e = local_1040[uVar10];
      local_1040[uVar10] = '\0';
      local_1018 = local_1038;
      local_104d = local_1038[uVar11];
      local_1038[uVar11] = '\0';
      local_1010 = uVar10;
      local_1008 = uVar11;
    }
    else {
      uVar12 = uVar11 + uVar10 + 2;
      if (uVar12 < 0xfa1) {
        local_1020 = local_fc8;
        local_1000 = (char *)0x0;
      }
      else {
        local_1020 = (char *)FUN_00118024(uVar12);
        local_1000 = local_1020;
      }
      local_1018 = local_1020 + uVar10 + 1;
      local_1010 = 0;
      for (local_ff8 = 0; local_ff8 < uVar10; local_ff8 = local_ff8 + 1) {
        if ((lVar2 == 0) ||
           (bVar5 = FUN_00104042((int)local_1040[local_ff8]),
           *(char *)(lVar2 + (ulong)bVar5) != '\x01')) {
          if (lVar1 == 0) {
            cVar3 = local_1040[local_ff8];
          }
          else {
            bVar5 = FUN_00104042((int)local_1040[local_ff8]);
            cVar3 = *(char *)(lVar1 + (ulong)bVar5);
          }
          local_1020[local_1010] = cVar3;
          local_1010 = local_1010 + 1;
        }
      }
      local_1020[local_1010] = '\0';
      local_1008 = 0;
      for (local_ff8 = 0; local_ff8 < uVar11; local_ff8 = local_ff8 + 1) {
        if ((lVar2 == 0) ||
           (bVar5 = FUN_00104042((int)local_1038[local_ff8]),
           *(char *)(lVar2 + (ulong)bVar5) != '\x01')) {
          if (lVar1 == 0) {
            cVar3 = local_1038[local_ff8];
          }
          else {
            bVar5 = FUN_00104042((int)local_1038[local_ff8]);
            cVar3 = *(char *)(lVar1 + (ulong)bVar5);
          }
          local_1018[local_1008] = cVar3;
          local_1008 = local_1008 + 1;
        }
      }
      local_1018[local_1008] = '\0';
    }
    if (*(char *)((long)local_1048 + 0x32) == '\0') {
      if (*(char *)((long)local_1048 + 0x34) == '\0') {
        if (*(char *)((long)local_1048 + 0x35) == '\0') {
          if (*(char *)((long)local_1048 + 0x36) == '\0') {
            if (*(char *)((long)local_1048 + 0x33) == '\0') {
              if ((char)local_1048[7] == '\0') {
                if (local_1010 == 0) {
                  local_104c = -(uint)(local_1008 != 0);
                }
                else if (local_1008 == 0) {
                  local_104c = 1;
                }
                else {
                  local_104c = FUN_001187ec(local_1020,local_1010 + 1,local_1018,local_1008 + 1);
                }
              }
              else {
                local_104c = FUN_00110461(local_1020,local_1010,local_1018,local_1008);
              }
            }
            else {
              local_104c = FUN_00107755(local_1020,local_1010,local_1018,local_1008);
            }
          }
          else {
            iVar6 = FUN_001073f3(local_1020,0);
            iVar7 = FUN_001073f3(local_1018,0);
            local_104c = iVar6 - iVar7;
          }
        }
        else {
          local_104c = FUN_001070e2(local_1020,local_1018);
        }
      }
      else {
        local_104c = FUN_001072d6(local_1020,local_1018);
      }
    }
    else {
      local_104c = FUN_001071a3(local_1020,local_1018);
    }
    if ((lVar2 == 0) && (lVar1 == 0)) {
      local_1020[local_1010] = local_104e;
      local_1018[local_1008] = local_104d;
    }
    else {
      free(local_1000);
    }
LAB_00109bf2:
    if (local_104c != 0) goto LAB_00109df1;
    local_1048 = (long *)local_1048[8];
    if (local_1048 == (long *)0x0) {
      local_104c = 0;
LAB_00109e10:
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return local_104c;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (local_1048[2] == -1) {
      local_1030 = (char *)(param_1[1] + -1 + *param_1);
      local_1028 = (char *)(param_2[1] + -1 + *param_2);
    }
    else {
      local_1030 = (char *)FUN_00106898(param_1,local_1048);
      local_1028 = (char *)FUN_00106898(param_2,local_1048);
    }
    if (*local_1048 == -1) {
      local_1040 = (char *)*param_1;
      local_1038 = (char *)*param_2;
      if ((char)local_1048[6] != '\0') {
        while ((local_1040 < local_1030 &&
               (bVar5 = FUN_00104042((int)*local_1040), (&DAT_00123260)[(int)(uint)bVar5] != '\0')))
        {
          local_1040 = local_1040 + 1;
        }
        while ((local_1038 < local_1028 &&
               (bVar5 = FUN_00104042((int)*local_1038), (&DAT_00123260)[(int)(uint)bVar5] != '\0')))
        {
          local_1038 = local_1038 + 1;
        }
      }
    }
    else {
      local_1040 = (char *)FUN_001066f4(param_1,local_1048);
      local_1038 = (char *)FUN_001066f4(param_2,local_1048);
    }
  } while( true );
}




// Function: compare @ 0x9e2a

uint compare(undefined8 *param_1,undefined8 *param_2)

{
  ulong uVar1;
  ulong uVar2;
  ulong __n;
  uint local_1c;
  
  if ((DAT_00123688 == 0) ||
     (((local_1c = FUN_00109116(param_1,param_2), local_1c == 0 && (DAT_00123682 == '\0')) &&
      (DAT_00123681 == '\0')))) {
    uVar1 = param_1[1] - 1;
    uVar2 = param_2[1] - 1;
    if (uVar1 == 0) {
      local_1c = -(uint)(uVar2 != 0);
    }
    else if (uVar2 == 0) {
      local_1c = 1;
    }
    else if (DAT_00123229 == '\0') {
      __n = uVar2;
      if (uVar1 < uVar2) {
        __n = uVar1;
      }
      local_1c = memcmp((void *)*param_1,(void *)*param_2,__n);
      if (local_1c == 0) {
        if (uVar1 < uVar2) {
          local_1c = 0xffffffff;
        }
        else {
          local_1c = (uint)(uVar1 != uVar2);
        }
      }
    }
    else {
      local_1c = FUN_001187ec(*param_1,param_1[1],*param_2,param_2[1]);
    }
    if (DAT_00123680 != '\0') {
      local_1c = -local_1c;
    }
  }
  return local_1c;
}




// Function: write_line @ 0x9f73

void write_line(undefined8 *param_1,FILE *param_2,long param_3)

{
  char *pcVar1;
  size_t __n;
  int iVar2;
  char *pcVar3;
  undefined8 uVar4;
  size_t sVar5;
  char local_29;
  char *local_28;
  
  local_28 = (char *)*param_1;
  __n = param_1[1];
  pcVar3 = local_28 + __n;
  if ((param_3 == 0) && (DAT_00123698 != '\0')) {
    while (local_28 < pcVar3) {
      pcVar1 = local_28 + 1;
      local_29 = *local_28;
      if (local_29 == '\t') {
        local_29 = '>';
      }
      else if (pcVar1 == pcVar3) {
        local_29 = '\n';
      }
      iVar2 = fputc_unlocked((int)local_29,param_2);
      local_28 = pcVar1;
      if (iVar2 == -1) {
        uVar4 = gettext("write failed");
        FUN_00104464(uVar4,0);
      }
    }
    FUN_0010829e(param_1);
  }
  else {
    pcVar3[-1] = DAT_00123010;
    sVar5 = fwrite_unlocked(local_28,1,__n,param_2);
    if (__n != sVar5) {
      uVar4 = gettext("write failed");
      FUN_00104464(uVar4,param_3);
    }
    pcVar3[-1] = '\0';
  }
  return;
}




// Function: check @ 0xa0b1

undefined1 check(undefined8 param_1,char param_2)

{
  long *plVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  long *plVar5;
  long lVar6;
  undefined8 uVar7;
  char *__format;
  undefined8 uVar8;
  ulong uVar9;
  long in_FS_OFFSET;
  bool bVar10;
  undefined1 local_da;
  ulong local_d8;
  long local_d0;
  long *local_c8;
  void *local_98;
  long local_90;
  long local_88;
  long local_80;
  void *local_78 [2];
  long local_68;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = FUN_0010505e(param_1,&DAT_0011c4cc);
  lVar6 = DAT_00123688;
  local_d8 = 0;
  local_d0 = 0;
  bVar10 = DAT_00123682 == '\0';
  local_da = 1;
  uVar9 = DAT_00123660;
  if (DAT_00123660 < DAT_00123018) {
    uVar9 = DAT_00123018;
  }
  FUN_00106618(local_78,0x20,uVar9);
  local_98 = (void *)0x0;
  do {
    cVar2 = FUN_00106a5e(local_78,uVar4,param_1);
    if (cVar2 == '\0') {
LAB_0010a446:
      FUN_001050b7(uVar4,param_1);
      free(local_78[0]);
      free(local_98);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return local_da;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_c8 = (long *)FUN_001066d0(local_78);
    plVar5 = local_c8 + local_68 * -4;
    if ((local_d8 != 0) &&
       (iVar3 = FUN_00109e2a(&local_98,local_c8 + -4), (int)(uint)bVar10 <= iVar3)) {
LAB_0010a1f4:
      if (param_2 == 'c') {
        lVar6 = FUN_001066d0(local_78);
        uVar7 = FUN_0011287e(local_d0 + (lVar6 - (long)(local_c8 + -4) >> 5),local_38);
        uVar8 = DAT_00123868;
        __format = (char *)gettext("%s: %s:%s: disorder: ");
        fprintf(stderr,__format,uVar8,param_1,uVar7);
        uVar8 = gettext("standard error");
        FUN_00109f73(local_c8 + -4,stderr,uVar8);
      }
      local_da = 0;
      goto LAB_0010a446;
    }
    while (plVar1 = local_c8 + -4, plVar5 < plVar1) {
      iVar3 = FUN_00109e2a(plVar1,local_c8 + -8);
      local_c8 = plVar1;
      if ((int)(uint)bVar10 <= iVar3) goto LAB_0010a1f4;
    }
    local_d0 = local_d0 + local_68;
    if (local_d8 < (ulong)local_c8[-3]) {
      do {
        local_d8 = local_d8 << 1;
        if (local_d8 == 0) {
          local_d8 = local_c8[-3];
          break;
        }
      } while (local_d8 < (ulong)local_c8[-3]);
      free(local_98);
      local_98 = (void *)FUN_00118024(local_d8);
    }
    memcpy(local_98,(void *)*plVar1,local_c8[-3]);
    local_90 = local_c8[-3];
    if (lVar6 != 0) {
      local_88 = (local_c8[-2] - *plVar1) + (long)local_98;
      local_80 = (local_c8[-1] - *plVar1) + (long)local_98;
    }
  } while( true );
}




// Function: open_input_files @ 0xa4a1

long open_input_files(long param_1,ulong param_2,long *param_3)

{
  long lVar1;
  undefined8 uVar2;
  int local_14;
  
  lVar1 = FUN_0011819d(param_2,8);
  *param_3 = lVar1;
  lVar1 = *param_3;
  for (local_14 = 0; (ulong)(long)local_14 < param_2; local_14 = local_14 + 1) {
    if ((*(long *)(param_1 + (long)local_14 * 0x10 + 8) == 0) ||
       (*(char *)(*(long *)(param_1 + (long)local_14 * 0x10 + 8) + 0xc) == '\0')) {
      uVar2 = FUN_00104ee4(*(undefined8 *)(param_1 + (long)local_14 * 0x10),&DAT_0011c4cc);
    }
    else {
      uVar2 = FUN_0010551c(*(undefined8 *)(param_1 + (long)local_14 * 0x10 + 8));
    }
    *(undefined8 *)(lVar1 + (long)local_14 * 8) = uVar2;
    if (*(long *)(lVar1 + (long)local_14 * 8) == 0) break;
  }
  return (long)local_14;
}




// Function: mergefps @ 0xa5c0

void mergefps(long param_1,ulong param_2,ulong param_3,undefined8 param_4,undefined8 param_5,
                 void *param_6)

{
  long *plVar1;
  undefined8 uVar2;
  long lVar3;
  char cVar4;
  int iVar5;
  void *__ptr;
  void *__ptr_00;
  void *__ptr_01;
  ulong *__ptr_02;
  long lVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  ulong uVar9;
  long in_FS_OFFSET;
  ulong local_100;
  ulong local_f8;
  void **local_d8;
  ulong local_d0;
  ulong local_c8;
  ulong local_c0;
  ulong local_b8;
  ulong local_b0;
  ulong local_a8;
  void *local_48;
  size_t local_40;
  long local_38;
  long local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = (void *)FUN_0011819d(param_3,0x38);
  local_d8 = (void **)0x0;
  local_d0 = 0;
  __ptr_00 = (void *)FUN_0011819d(param_3,8);
  __ptr_01 = (void *)FUN_0011819d(param_3,8);
  __ptr_02 = (ulong *)FUN_0011819d(param_3,8);
  lVar3 = DAT_00123688;
  local_48 = (void *)0x0;
  local_c8 = 0;
  local_100 = param_3;
  local_f8 = param_2;
  while (local_c8 < local_100) {
    uVar9 = DAT_00123660 / local_100;
    if (DAT_00123660 / local_100 < DAT_00123018) {
      uVar9 = DAT_00123018;
    }
    FUN_00106618((void *)((long)__ptr + local_c8 * 0x38),0x20,uVar9);
    cVar4 = FUN_00106a5e((void *)((long)__ptr + local_c8 * 0x38),
                         *(undefined8 *)((long)param_6 + local_c8 * 8),
                         *(undefined8 *)(param_1 + local_c8 * 0x10));
    if (cVar4 == '\0') {
      FUN_001050b7(*(undefined8 *)((long)param_6 + local_c8 * 8),
                   *(undefined8 *)(param_1 + local_c8 * 0x10));
      if (local_c8 < local_f8) {
        local_f8 = local_f8 - 1;
        FUN_00105782(*(undefined8 *)(param_1 + local_c8 * 0x10));
      }
      free(*(void **)((long)__ptr + local_c8 * 0x38));
      local_100 = local_100 - 1;
      for (local_c0 = local_c8; local_c0 < local_100; local_c0 = local_c0 + 1) {
        puVar7 = (undefined8 *)(param_1 + (local_c0 + 1) * 0x10);
        puVar8 = (undefined8 *)(local_c0 * 0x10 + param_1);
        uVar2 = puVar7[1];
        *puVar8 = *puVar7;
        puVar8[1] = uVar2;
        *(undefined8 *)((long)param_6 + local_c0 * 8) =
             *(undefined8 *)((long)param_6 + (local_c0 + 1) * 8);
      }
    }
    else {
      lVar6 = FUN_001066d0((void *)((long)__ptr + local_c8 * 0x38));
      *(long *)((long)__ptr_00 + local_c8 * 8) = lVar6 + -0x20;
      *(long *)((long)__ptr_01 + local_c8 * 8) =
           lVar6 + *(long *)((long)__ptr + local_c8 * 0x38 + 0x10) * -0x20;
      local_c8 = local_c8 + 1;
    }
  }
  for (local_c8 = 0; local_c8 < local_100; local_c8 = local_c8 + 1) {
    __ptr_02[local_c8] = local_c8;
  }
  for (local_c8 = 1; local_c8 < local_100; local_c8 = local_c8 + 1) {
    iVar5 = FUN_00109e2a(*(undefined8 *)((long)__ptr_00 + __ptr_02[local_c8 - 1] * 8),
                         *(undefined8 *)((long)__ptr_00 + __ptr_02[local_c8] * 8));
    if (0 < iVar5) {
      uVar9 = __ptr_02[local_c8 - 1];
      __ptr_02[local_c8 - 1] = __ptr_02[local_c8];
      __ptr_02[local_c8] = uVar9;
      local_c8 = 0;
    }
  }
LAB_0010b27c:
  do {
    if (local_100 == 0) {
      if ((DAT_00123682 != '\0') && (local_d8 != (void **)0x0)) {
        FUN_00109f73(&local_48,param_4,param_5);
        free(local_48);
      }
      FUN_001050b7(param_4,param_5);
      free(param_6);
      free(__ptr);
      free(__ptr_02);
      free(__ptr_01);
      free(__ptr_00);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    plVar1 = *(long **)((long)__ptr_00 + *__ptr_02 * 8);
    if (DAT_00123682 == '\0') {
      FUN_00109f73(plVar1,param_4,param_5);
    }
    else {
      if ((local_d8 != (void **)0x0) && (iVar5 = FUN_00109e2a(local_d8,plVar1), iVar5 != 0)) {
        local_d8 = (void **)0x0;
        FUN_00109f73(&local_48,param_4,param_5);
      }
      if (local_d8 == (void **)0x0) {
        local_d8 = &local_48;
        if (local_d0 < (ulong)plVar1[1]) {
          do {
            if (local_d0 == 0) {
              local_d0 = plVar1[1];
              break;
            }
            local_d0 = local_d0 << 1;
          } while (local_d0 < (ulong)plVar1[1]);
          free(local_48);
          local_48 = (void *)FUN_00118024(local_d0);
        }
        local_40 = plVar1[1];
        memcpy(local_48,(void *)*plVar1,local_40);
        if (lVar3 != 0) {
          local_38 = (plVar1[2] - *plVar1) + (long)local_48;
          local_30 = (plVar1[3] - *plVar1) + (long)local_48;
        }
      }
    }
    if (*(long **)((long)__ptr_01 + *__ptr_02 * 8) < plVar1) {
      *(long **)((long)__ptr_00 + *__ptr_02 * 8) = plVar1 + -4;
    }
    else {
      cVar4 = FUN_00106a5e((void *)((long)__ptr + *__ptr_02 * 0x38),
                           *(undefined8 *)((long)param_6 + *__ptr_02 * 8),
                           *(undefined8 *)(param_1 + *__ptr_02 * 0x10));
      if (cVar4 == '\0') {
        for (local_c8 = 1; local_c8 < local_100; local_c8 = local_c8 + 1) {
          if (*__ptr_02 < __ptr_02[local_c8]) {
            __ptr_02[local_c8] = __ptr_02[local_c8] - 1;
          }
        }
        local_100 = local_100 - 1;
        FUN_001050b7(*(undefined8 *)((long)param_6 + *__ptr_02 * 8),
                     *(undefined8 *)(param_1 + *__ptr_02 * 0x10));
        if (*__ptr_02 < local_f8) {
          local_f8 = local_f8 - 1;
          FUN_00105782(*(undefined8 *)(param_1 + *__ptr_02 * 0x10));
        }
        free(*(void **)((long)__ptr + *__ptr_02 * 0x38));
        for (local_c8 = *__ptr_02; local_c8 < local_100; local_c8 = local_c8 + 1) {
          *(undefined8 *)((long)param_6 + local_c8 * 8) =
               *(undefined8 *)((long)param_6 + (local_c8 + 1) * 8);
          puVar7 = (undefined8 *)(param_1 + (local_c8 + 1) * 0x10);
          puVar8 = (undefined8 *)(local_c8 * 0x10 + param_1);
          uVar2 = puVar7[1];
          *puVar8 = *puVar7;
          puVar8[1] = uVar2;
          puVar8 = (undefined8 *)((local_c8 + 1) * 0x38 + (long)__ptr);
          puVar7 = (undefined8 *)((long)__ptr + local_c8 * 0x38);
          uVar2 = puVar8[1];
          *puVar7 = *puVar8;
          puVar7[1] = uVar2;
          uVar2 = puVar8[3];
          puVar7[2] = puVar8[2];
          puVar7[3] = uVar2;
          uVar2 = puVar8[5];
          puVar7[4] = puVar8[4];
          puVar7[5] = uVar2;
          puVar7[6] = puVar8[6];
          *(undefined8 *)((long)__ptr_00 + local_c8 * 8) =
               *(undefined8 *)((long)__ptr_00 + (local_c8 + 1) * 8);
          *(undefined8 *)((long)__ptr_01 + local_c8 * 8) =
               *(undefined8 *)((long)__ptr_01 + (local_c8 + 1) * 8);
        }
        for (local_c8 = 0; local_c8 < local_100; local_c8 = local_c8 + 1) {
          __ptr_02[local_c8] = __ptr_02[local_c8 + 1];
        }
        goto LAB_0010b27c;
      }
      lVar6 = FUN_001066d0((void *)((long)__ptr + *__ptr_02 * 0x38));
      *(long *)((long)__ptr_00 + *__ptr_02 * 8) = lVar6 + -0x20;
      *(long *)((long)__ptr_01 + *__ptr_02 * 8) =
           lVar6 + *(long *)((long)__ptr + *__ptr_02 * 0x38 + 0x10) * -0x20;
    }
    local_b8 = 1;
    local_b0 = local_100;
    local_a8 = 1;
    uVar9 = *__ptr_02;
    while (local_b8 < local_b0) {
      iVar5 = FUN_00109e2a(*(undefined8 *)((long)__ptr_00 + uVar9 * 8),
                           *(undefined8 *)((long)__ptr_00 + __ptr_02[local_a8] * 8));
      if ((iVar5 < 0) || ((iVar5 == 0 && (uVar9 < __ptr_02[local_a8])))) {
        local_b0 = local_a8;
      }
      else {
        local_b8 = local_a8 + 1;
      }
      local_a8 = local_b0 + local_b8 >> 1;
    }
    for (local_c0 = 0; local_c0 < local_b8 - 1; local_c0 = local_c0 + 1) {
      __ptr_02[local_c0] = __ptr_02[local_c0 + 1];
    }
    __ptr_02[local_b8 - 1] = uVar9;
  } while( true );
}




// Function: mergefiles @ 0xb344

ulong mergefiles(long param_1,undefined8 param_2,ulong param_3,undefined8 param_4,
                  undefined8 param_5)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_30;
  ulong local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = FUN_0010a4a1(param_1,param_3,&local_30);
  if ((local_28 < param_3) && (local_28 < 2)) {
    uVar1 = *(undefined8 *)(param_1 + local_28 * 0x10);
    uVar2 = gettext("open failed");
    FUN_00104464(uVar2,uVar1);
  }
  FUN_0010a5c0(param_1,param_2,local_28,param_4,param_5,local_30);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_28;
}




// Function: mergelines @ 0xb413

void mergelines(undefined8 *param_1,ulong param_2,undefined8 *param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  int iVar3;
  undefined8 *local_40;
  undefined8 *local_30;
  ulong local_20;
  long local_18;
  undefined8 *local_10;
  
  local_20 = param_2 >> 1;
  local_18 = param_2 - local_20;
  local_40 = param_3;
  local_30 = param_1;
  local_10 = param_1 + local_20 * -4;
  do {
    while (iVar3 = FUN_00109e2a(local_40 + -4,local_10 + -4), 0 < iVar3) {
      puVar1 = local_30 + -4;
      uVar2 = local_10[-3];
      *puVar1 = local_10[-4];
      local_30[-3] = uVar2;
      uVar2 = local_10[-1];
      local_30[-2] = local_10[-2];
      local_30[-1] = uVar2;
      local_18 = local_18 + -1;
      local_30 = puVar1;
      local_10 = local_10 + -4;
      if (local_18 == 0) {
        do {
          uVar2 = local_40[-3];
          local_30[-4] = local_40[-4];
          local_30[-3] = uVar2;
          uVar2 = local_40[-1];
          local_30[-2] = local_40[-2];
          local_30[-1] = uVar2;
          local_20 = local_20 - 1;
          local_40 = local_40 + -4;
          local_30 = local_30 + -4;
        } while (local_20 != 0);
        return;
      }
    }
    uVar2 = local_40[-3];
    local_30[-4] = local_40[-4];
    local_30[-3] = uVar2;
    uVar2 = local_40[-1];
    local_30[-2] = local_40[-2];
    local_30[-1] = uVar2;
    local_20 = local_20 - 1;
    local_40 = local_40 + -4;
    local_30 = local_30 + -4;
  } while (local_20 != 0);
  return;
}




// Function: sequential_sort @ 0xb536

void sequential_sort(long param_1,ulong param_2,long param_3,char param_4)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  long lVar6;
  long local_38;
  long local_30;
  
  if (param_2 == 2) {
    iVar3 = FUN_00109e2a(param_1 + -0x20,param_1 + -0x40);
    uVar4 = (uint)(0 < iVar3);
    if (param_4 == '\0') {
      if (uVar4 != 0) {
        uVar2 = *(undefined8 *)(param_1 + -0x18);
        *(undefined8 *)(param_3 + -0x20) = *(undefined8 *)(param_1 + -0x20);
        *(undefined8 *)(param_3 + -0x18) = uVar2;
        uVar2 = *(undefined8 *)(param_1 + -8);
        *(undefined8 *)(param_3 + -0x10) = *(undefined8 *)(param_1 + -0x10);
        *(undefined8 *)(param_3 + -8) = uVar2;
        *(undefined8 *)(param_1 + -0x20) = *(undefined8 *)(param_1 + -0x40);
        *(undefined8 *)(param_1 + -0x18) = *(undefined8 *)(param_1 + -0x38);
        *(undefined8 *)(param_1 + -0x10) = *(undefined8 *)(param_1 + -0x30);
        *(undefined8 *)(param_1 + -8) = *(undefined8 *)(param_1 + -0x28);
        uVar2 = *(undefined8 *)(param_3 + -0x18);
        *(undefined8 *)(param_1 + -0x40) = *(undefined8 *)(param_3 + -0x20);
        *(undefined8 *)(param_1 + -0x38) = uVar2;
        uVar2 = *(undefined8 *)(param_3 + -8);
        *(undefined8 *)(param_1 + -0x30) = *(undefined8 *)(param_3 + -0x10);
        *(undefined8 *)(param_1 + -0x28) = uVar2;
      }
    }
    else {
      puVar1 = (undefined8 *)((long)(int)~uVar4 * 0x20 + param_1);
      uVar2 = puVar1[1];
      *(undefined8 *)(param_3 + -0x20) = *puVar1;
      *(undefined8 *)(param_3 + -0x18) = uVar2;
      uVar2 = puVar1[3];
      *(undefined8 *)(param_3 + -0x10) = puVar1[2];
      *(undefined8 *)(param_3 + -8) = uVar2;
      puVar1 = (undefined8 *)((long)(int)(uVar4 - 2) * 0x20 + param_1);
      uVar2 = puVar1[1];
      *(undefined8 *)(param_3 + -0x40) = *puVar1;
      *(undefined8 *)(param_3 + -0x38) = uVar2;
      uVar2 = puVar1[3];
      *(undefined8 *)(param_3 + -0x30) = puVar1[2];
      *(undefined8 *)(param_3 + -0x28) = uVar2;
    }
  }
  else {
    uVar5 = param_2 >> 1;
    if (param_4 == '\0') {
      lVar6 = 0;
    }
    else {
      lVar6 = uVar5 * -0x20;
    }
    sequential_sort(param_1 + uVar5 * -0x20,param_2 - uVar5,lVar6 + param_3,param_4);
    if (uVar5 < 2) {
      if (param_4 != '\x01') {
        uVar2 = *(undefined8 *)(param_1 + -0x18);
        *(undefined8 *)(param_3 + -0x20) = *(undefined8 *)(param_1 + -0x20);
        *(undefined8 *)(param_3 + -0x18) = uVar2;
        uVar2 = *(undefined8 *)(param_1 + -8);
        *(undefined8 *)(param_3 + -0x10) = *(undefined8 *)(param_1 + -0x10);
        *(undefined8 *)(param_3 + -8) = uVar2;
      }
    }
    else {
      sequential_sort(param_1,uVar5,param_3,param_4 == '\0');
    }
    local_38 = param_1;
    local_30 = param_3;
    if (param_4 != '\0') {
      local_38 = param_3;
      local_30 = param_1;
    }
    FUN_0010b413(local_38,param_2,local_30);
  }
  return;
}




// Function: merge_tree_init @ 0xb7b6

undefined8 * merge_tree_init(long param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00118024(param_1 << 8);
  puVar1[3] = 0;
  puVar1[2] = puVar1[3];
  puVar1[1] = puVar1[2];
  *puVar1 = puVar1[1];
  puVar1[4] = 0;
  puVar1[6] = param_2;
  puVar1[5] = puVar1[6];
  puVar1[7] = 0;
  *(undefined4 *)(puVar1 + 10) = 0;
  *(undefined1 *)((long)puVar1 + 0x54) = 0;
  pthread_mutex_init((pthread_mutex_t *)(puVar1 + 0xb),(pthread_mutexattr_t *)0x0);
  FUN_0010b911(puVar1,puVar1 + 0x10,param_3,param_1,param_2,0);
  return puVar1;
}




// Function: merge_tree_destroy @ 0xb8b3

void merge_tree_destroy(long param_1,void *param_2)

{
  undefined8 local_18;
  undefined8 local_10;
  
  local_18 = param_1 * 2;
  local_10 = param_2;
  while (local_18 != 0) {
    pthread_mutex_destroy((pthread_mutex_t *)((long)local_10 + 0x58));
    local_10 = (void *)((long)local_10 + 0x80);
    local_18 = local_18 + -1;
  }
  free(param_2);
  return;
}




// Function: init_node @ 0xb911

undefined8 *
init_node(long param_1,undefined8 *param_2,long param_3,ulong param_4,long param_5,char param_6)

{
  ulong uVar1;
  ulong uVar2;
  long lVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 *local_68;
  
  if (param_6 == '\0') {
    uVar1 = *(ulong *)(param_1 + 0x30);
  }
  else {
    uVar1 = *(ulong *)(param_1 + 0x28);
  }
  uVar2 = uVar1 >> 1;
  param_3 = param_3 + param_5 * -0x20;
  lVar3 = param_3 + uVar2 * -0x20;
  if (param_6 == '\0') {
    lVar4 = param_1 + 0x18;
  }
  else {
    lVar4 = param_1 + 0x10;
  }
  local_68 = param_2 + 0x10;
  param_2[2] = param_3;
  *param_2 = param_2[2];
  param_2[3] = lVar3;
  param_2[1] = param_2[3];
  param_2[4] = lVar4;
  param_2[5] = uVar2;
  param_2[6] = uVar1 - uVar2;
  param_2[7] = param_1;
  *(int *)(param_2 + 10) = *(int *)(param_1 + 0x50) + 1;
  *(undefined1 *)((long)param_2 + 0x54) = 0;
  pthread_mutex_init((pthread_mutex_t *)(param_2 + 0xb),(pthread_mutexattr_t *)0x0);
  if (param_4 < 2) {
    param_2[8] = 0;
    param_2[9] = 0;
  }
  else {
    param_2[8] = local_68;
    uVar5 = init_node(param_2,local_68,param_3,param_4 >> 1,param_5,1);
    param_2[9] = uVar5;
    local_68 = (undefined8 *)init_node(param_2,uVar5,lVar3,param_4 - (param_4 >> 1),param_5,0);
  }
  return local_68;
}




// Function: compare_nodes @ 0xbb0c

bool compare_nodes(long param_1,long param_2)

{
  bool bVar1;
  
  if (*(int *)(param_1 + 0x50) == *(int *)(param_2 + 0x50)) {
    bVar1 = (ulong)(*(long *)(param_1 + 0x28) + *(long *)(param_1 + 0x30)) <
            (ulong)(*(long *)(param_2 + 0x30) + *(long *)(param_2 + 0x28));
  }
  else {
    bVar1 = *(uint *)(param_1 + 0x50) < *(uint *)(param_2 + 0x50);
  }
  return bVar1;
}




// Function: lock_node @ 0xbb87

void lock_node(long param_1)

{
  pthread_mutex_lock((pthread_mutex_t *)(param_1 + 0x58));
  return;
}




// Function: unlock_node @ 0xbba6

void unlock_node(long param_1)

{
  pthread_mutex_unlock((pthread_mutex_t *)(param_1 + 0x58));
  return;
}




// Function: queue_destroy @ 0xbbc5

void queue_destroy(undefined8 *param_1)

{
  FUN_00112398(*param_1);
  pthread_cond_destroy((pthread_cond_t *)(param_1 + 6));
  pthread_mutex_destroy((pthread_mutex_t *)(param_1 + 1));
  return;
}




// Function: queue_init @ 0xbc07

void queue_init(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  
  uVar1 = FUN_001122ed(FUN_0010bb0c,param_2 * 2);
  *param_1 = uVar1;
  pthread_mutex_init((pthread_mutex_t *)(param_1 + 1),(pthread_mutexattr_t *)0x0);
  pthread_cond_init((pthread_cond_t *)(param_1 + 6),(pthread_condattr_t *)0x0);
  return;
}




// Function: queue_insert @ 0xbc6b

void queue_insert(undefined8 *param_1,long param_2)

{
  pthread_mutex_lock((pthread_mutex_t *)(param_1 + 1));
  FUN_001123c6(*param_1,param_2);
  *(undefined1 *)(param_2 + 0x54) = 1;
  pthread_cond_signal((pthread_cond_t *)(param_1 + 6));
  pthread_mutex_unlock((pthread_mutex_t *)(param_1 + 1));
  return;
}




// Function: queue_pop @ 0xbcd0

long queue_pop(undefined8 *param_1)

{
  long lVar1;
  
  pthread_mutex_lock((pthread_mutex_t *)(param_1 + 1));
  while( true ) {
    lVar1 = FUN_00112474(*param_1);
    if (lVar1 != 0) break;
    pthread_cond_wait((pthread_cond_t *)(param_1 + 6),(pthread_mutex_t *)(param_1 + 1));
  }
  pthread_mutex_unlock((pthread_mutex_t *)(param_1 + 1));
  FUN_0010bb87(lVar1);
  *(undefined1 *)(lVar1 + 0x54) = 0;
  return lVar1;
}




// Function: write_unique @ 0xbd51

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void write_unique(long *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  if (DAT_00123682 != '\0') {
    if ((DAT_00123240 != 0) && (iVar1 = FUN_00109e2a(param_1,&DAT_00123240), iVar1 == 0)) {
      return;
    }
    DAT_00123240 = *param_1;
    _DAT_00123248 = param_1[1];
    _DAT_00123250 = param_1[2];
    _DAT_00123258 = param_1[3];
  }
  FUN_00109f73(param_1,param_2,param_3);
  return;
}




// Function: mergelines_node @ 0xbde5

void mergelines_node(long *param_1,ulong param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  long lVar2;
  long lVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  int iVar6;
  bool bVar7;
  long local_38;
  undefined8 *local_30;
  
  lVar2 = *param_1;
  lVar3 = param_1[1];
  local_38 = (param_2 >> (((char)(int)param_1[10] + '\x01') * '\x02' & 0x3fU)) + 1;
  if (*(uint *)(param_1 + 10) < 2) {
    while (((*param_1 != param_1[2] && (param_1[1] != param_1[3])) &&
           (lVar1 = local_38 + -1, bVar7 = local_38 != 0, local_38 = lVar1, bVar7))) {
      iVar6 = FUN_00109e2a(*param_1 + -0x20,param_1[1] + -0x20);
      if (iVar6 < 1) {
        *param_1 = *param_1 + -0x20;
        FUN_0010bd51(*param_1,param_3,param_4);
      }
      else {
        param_1[1] = param_1[1] + -0x20;
        FUN_0010bd51(param_1[1],param_3,param_4);
      }
    }
    if (lVar3 - param_1[1] >> 5 == param_1[6]) {
      while ((*param_1 != param_1[2] && (local_38 != 0))) {
        *param_1 = *param_1 + -0x20;
        FUN_0010bd51(*param_1,param_3,param_4);
        local_38 = local_38 + -1;
      }
    }
    else if (lVar2 - *param_1 >> 5 == param_1[5]) {
      while ((param_1[1] != param_1[3] && (local_38 != 0))) {
        param_1[1] = param_1[1] + -0x20;
        FUN_0010bd51(param_1[1],param_3,param_4);
        local_38 = local_38 + -1;
      }
    }
  }
  else {
    local_30 = *(undefined8 **)param_1[4];
    while (((*param_1 != param_1[2] && (param_1[1] != param_1[3])) &&
           (lVar1 = local_38 + -1, bVar7 = local_38 != 0, local_38 = lVar1, bVar7))) {
      iVar6 = FUN_00109e2a(*param_1 + -0x20,param_1[1] + -0x20);
      if (iVar6 < 1) {
        *param_1 = *param_1 + -0x20;
        puVar4 = (undefined8 *)*param_1;
        uVar5 = puVar4[1];
        local_30[-4] = *puVar4;
        local_30[-3] = uVar5;
        uVar5 = puVar4[3];
        local_30[-2] = puVar4[2];
        local_30[-1] = uVar5;
        local_30 = local_30 + -4;
      }
      else {
        param_1[1] = param_1[1] + -0x20;
        puVar4 = (undefined8 *)param_1[1];
        uVar5 = puVar4[1];
        local_30[-4] = *puVar4;
        local_30[-3] = uVar5;
        uVar5 = puVar4[3];
        local_30[-2] = puVar4[2];
        local_30[-1] = uVar5;
        local_30 = local_30 + -4;
      }
    }
    if (lVar3 - param_1[1] >> 5 == param_1[6]) {
      while ((*param_1 != param_1[2] && (local_38 != 0))) {
        *param_1 = *param_1 + -0x20;
        puVar4 = (undefined8 *)*param_1;
        uVar5 = puVar4[1];
        local_30[-4] = *puVar4;
        local_30[-3] = uVar5;
        uVar5 = puVar4[3];
        local_30[-2] = puVar4[2];
        local_30[-1] = uVar5;
        local_38 = local_38 + -1;
        local_30 = local_30 + -4;
      }
    }
    else if (lVar2 - *param_1 >> 5 == param_1[5]) {
      while ((param_1[1] != param_1[3] && (local_38 != 0))) {
        param_1[1] = param_1[1] + -0x20;
        puVar4 = (undefined8 *)param_1[1];
        uVar5 = puVar4[1];
        local_30[-4] = *puVar4;
        local_30[-3] = uVar5;
        uVar5 = puVar4[3];
        local_30[-2] = puVar4[2];
        local_30[-1] = uVar5;
        local_38 = local_38 + -1;
        local_30 = local_30 + -4;
      }
    }
    *(undefined8 **)param_1[4] = local_30;
  }
  param_1[5] = param_1[5] - (lVar2 - *param_1 >> 5);
  param_1[6] = param_1[6] - (lVar3 - param_1[1] >> 5);
  return;
}




// Function: queue_check_insert @ 0xc28e

void queue_check_insert(undefined8 param_1,long *param_2)

{
  bool bVar1;
  
  if (*(char *)((long)param_2 + 0x54) != '\x01') {
    if (*param_2 == param_2[2]) {
      if ((param_2[1] != param_2[3]) && (param_2[5] == 0)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
    }
    else if ((param_2[1] != param_2[3]) || (param_2[6] == 0)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    if (bVar1) {
      FUN_0010bc6b(param_1,param_2);
    }
  }
  return;
}




// Function: queue_check_insert_parent @ 0xc34c

void queue_check_insert_parent(undefined8 param_1,long param_2)

{
  if (*(uint *)(param_2 + 0x50) < 2) {
    if (*(long *)(param_2 + 0x30) + *(long *)(param_2 + 0x28) == 0) {
      FUN_0010bc6b(param_1,*(undefined8 *)(param_2 + 0x38));
    }
  }
  else {
    FUN_0010bb87(*(undefined8 *)(param_2 + 0x38));
    FUN_0010c28e(param_1,*(undefined8 *)(param_2 + 0x38));
    FUN_0010bba6(*(undefined8 *)(param_2 + 0x38));
  }
  return;
}




// Function: merge_loop @ 0xc3d7

void merge_loop(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  
  while( true ) {
    lVar1 = FUN_0010bcd0(param_1);
    if (*(int *)(lVar1 + 0x50) == 0) break;
    FUN_0010bde5(lVar1,param_2,param_3,param_4);
    FUN_0010c28e(param_1,lVar1);
    FUN_0010c34c(param_1,lVar1);
    FUN_0010bba6(lVar1);
  }
  FUN_0010bba6(lVar1);
  FUN_0010bc6b(param_1,lVar1);
  return;
}




// Function: sortlines_thread @ 0xc481

undefined8 sortlines_thread(undefined8 *param_1)

{
  FUN_0010c4e8(*param_1,param_1[1],param_1[2],param_1[3],param_1[4],param_1[5],param_1[6]);
  return 0;
}




// Function: sortlines @ 0xc4e8

void sortlines(long param_1,ulong param_2,long param_3,long *param_4,undefined8 param_5,
                 undefined8 param_6,undefined8 param_7)

{
  int iVar1;
  long in_FS_OFFSET;
  pthread_t local_80;
  ulong local_78;
  ulong local_70;
  long local_68;
  ulong local_60;
  ulong local_58;
  long local_50;
  long local_48;
  ulong local_40;
  long local_38;
  long local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = param_4[6] + param_4[5];
  local_70 = param_2 >> 1;
  local_68 = param_2 - local_70;
  local_30 = param_4[8];
  local_18 = param_7;
  local_48 = param_1;
  local_40 = local_70;
  local_38 = param_3;
  local_28 = param_5;
  local_20 = param_6;
  if ((1 < param_2) && (0x1ffff < local_78)) {
    iVar1 = pthread_create(&local_80,(pthread_attr_t *)0x0,FUN_0010c481,&local_48);
    if (iVar1 == 0) {
      sortlines(param_4[5] * -0x20 + param_1,local_68,param_3,param_4[9],param_5,param_6,param_7)
      ;
      pthread_join(local_80,(void **)0x0);
      goto code_r0x0010c7e4;
    }
  }
  local_60 = param_4[5];
  local_58 = param_4[6];
  local_50 = param_1 + param_3 * -0x20;
  if (1 < local_58) {
    FUN_0010b536(local_60 * -0x20 + param_1,local_58,(local_60 >> 1) * -0x20 + local_50,0);
  }
  if (1 < local_60) {
    FUN_0010b536(param_1,local_60,local_50,0);
  }
  *param_4 = param_1;
  param_4[1] = local_60 * -0x20 + param_1;
  param_4[2] = local_60 * -0x20 + param_1;
  param_4[3] = local_60 * -0x20 + local_58 * -0x20 + param_1;
  FUN_0010bc6b(param_5,param_4);
  FUN_0010c3d7(param_5,param_3,param_6,param_7);
code_r0x0010c7e4:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: avoid_trashing_input @ 0xc7fa

void avoid_trashing_input(long param_1,ulong param_2,ulong param_3,char *param_4)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  long in_FS_OFFSET;
  undefined8 local_c8;
  long local_c0;
  ulong local_b8;
  __dev_t *local_b0;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_c0 = 0;
  for (local_b8 = param_2; local_b8 < param_3; local_b8 = local_b8 + 1) {
    iVar2 = strcmp(*(char **)(param_1 + local_b8 * 0x10),"-");
    if (((param_4 == (char *)0x0) ||
        (iVar3 = strcmp(param_4,*(char **)(param_1 + local_b8 * 0x10)), iVar3 != 0)) || (iVar2 == 0)
       ) {
      local_b0 = (__dev_t *)FUN_00104e8c();
      if (local_b0 == (__dev_t *)0x0) break;
      if (iVar2 == 0) {
        iVar2 = fstat(0,&local_a8);
      }
      else {
        iVar2 = stat(*(char **)(param_1 + local_b8 * 0x10),&local_a8);
      }
      if (((iVar2 == 0) && (local_a8.st_ino == local_b0[1])) && (local_a8.st_dev == *local_b0)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
    }
    else {
      bVar1 = true;
    }
    if (bVar1) {
      if (local_c0 == 0) {
        local_c0 = FUN_001054f9(&local_c8);
        FUN_0010b344(local_b8 * 0x10 + param_1,0,1,local_c8,local_c0 + 0xd);
      }
      *(long *)(param_1 + local_b8 * 0x10) = local_c0 + 0xd;
      *(long *)(local_b8 * 0x10 + param_1 + 8) = local_c0;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: check_inputs @ 0xca81

void check_inputs(long param_1,ulong param_2)

{
  undefined8 uVar1;
  int iVar2;
  undefined8 uVar3;
  ulong local_20;
  
  for (local_20 = 0; local_20 < param_2; local_20 = local_20 + 1) {
    iVar2 = strcmp(*(char **)(param_1 + local_20 * 8),"-");
    if (iVar2 != 0) {
      iVar2 = euidaccess(*(char **)(param_1 + local_20 * 8),4);
      if (iVar2 != 0) {
        uVar1 = *(undefined8 *)(param_1 + local_20 * 8);
        uVar3 = gettext("cannot read");
        FUN_00104464(uVar3,uVar1);
      }
    }
  }
  return;
}




// Function: check_output @ 0xcb44

void check_output(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  if (param_1 != (char *)0x0) {
    iVar1 = open(param_1,0x80041,0x1b6);
    if (iVar1 < 0) {
      uVar2 = gettext("open failed");
      FUN_00104464(uVar2,param_1);
    }
    FUN_0010515d(iVar1,1);
  }
  return;
}




// Function: merge @ 0xcbb9

void merge(long *param_1,ulong param_2,ulong param_3,undefined8 param_4)

{
  long lVar1;
  int *piVar2;
  undefined8 uVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  ulong local_b0;
  ulong local_a8;
  long local_90;
  undefined8 local_88;
  long local_80;
  ulong local_78;
  ulong local_70;
  long local_68;
  long local_60;
  ulong local_58;
  ulong local_50;
  ulong local_48;
  long local_40;
  ulong local_38;
  long local_30;
  ulong local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = param_2;
  for (local_b0 = param_3; DAT_00123024 < local_b0; local_b0 = local_b0 + (local_78 - local_80)) {
    local_78 = 0;
    for (local_80 = 0; (ulong)DAT_00123024 <= local_b0 - local_80; local_80 = local_80 + local_28) {
      local_30 = FUN_001054f9(&local_88);
      uVar4 = local_a8;
      if (DAT_00123024 <= local_a8) {
        uVar4 = (ulong)DAT_00123024;
      }
      local_28 = FUN_0010b344(param_1 + local_80 * 2,uVar4,DAT_00123024,local_88,local_30 + 0xd);
      uVar4 = local_a8;
      if (local_28 <= local_a8) {
        uVar4 = local_28;
      }
      local_a8 = local_a8 - uVar4;
      param_1[local_78 * 2] = local_30 + 0xd;
      param_1[local_78 * 2 + 1] = local_30;
      local_78 = local_78 + 1;
    }
    local_58 = local_b0 - local_80;
    local_50 = (ulong)DAT_00123024 - local_78 % (ulong)DAT_00123024;
    if (local_50 < local_58) {
      local_48 = (local_58 - local_50) + 1;
      local_40 = FUN_001054f9(&local_88);
      uVar4 = local_a8;
      if (local_48 <= local_a8) {
        uVar4 = local_48;
      }
      local_38 = FUN_0010b344(param_1 + local_80 * 2,uVar4,local_48,local_88,local_40 + 0xd);
      uVar4 = local_a8;
      if (local_38 <= local_a8) {
        uVar4 = local_38;
      }
      local_a8 = local_a8 - uVar4;
      param_1[local_78 * 2] = local_40 + 0xd;
      param_1[local_78 * 2 + 1] = local_40;
      local_80 = local_80 + local_38;
      local_78 = local_78 + 1;
    }
    memmove(param_1 + local_78 * 2,param_1 + local_80 * 2,(local_b0 - local_80) * 0x10);
    local_a8 = local_a8 + local_78;
  }
  FUN_0010c7fa(param_1,local_a8,local_b0,param_4);
  do {
    local_70 = FUN_0010a4a1(param_1,local_b0,&local_90);
    if (local_70 == local_b0) {
      local_68 = FUN_00104ee4(param_4,&DAT_0011c452);
      if (local_68 != 0) {
        FUN_0010a5c0(param_1,local_a8,local_b0,local_68,param_4,local_90);
        if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return;
      }
      piVar2 = __errno_location();
      if ((*piVar2 != 0x18) || (local_70 < 3)) {
        uVar3 = gettext("open failed");
        FUN_00104464(uVar3,param_4);
      }
    }
    else if (local_70 < 3) {
      lVar1 = param_1[local_70 * 2];
      uVar3 = gettext("open failed");
      FUN_00104464(uVar3,lVar1);
    }
    do {
      local_70 = local_70 - 1;
      FUN_001050b7(*(undefined8 *)(local_70 * 8 + local_90),param_1[local_70 * 2]);
      local_60 = FUN_00105372(&local_88,2 < local_70);
    } while (local_60 == 0);
    uVar4 = local_a8;
    if (local_70 <= local_a8) {
      uVar4 = local_70;
    }
    FUN_0010a5c0(param_1,uVar4,local_70,local_88,local_60 + 0xd,local_90);
    uVar4 = local_a8;
    if (local_70 <= local_a8) {
      uVar4 = local_70;
    }
    *param_1 = local_60 + 0xd;
    param_1[1] = local_60;
    memmove(param_1 + 2,param_1 + local_70 * 2,(local_b0 - local_70) * 0x10);
    local_a8 = (local_a8 - uVar4) + 1;
    local_b0 = (local_b0 - local_70) + 1;
  } while( true );
}




// Function: sort @ 0xd0f9

void sort(undefined8 *param_1,long param_2,long param_3,ulong param_4)

{
  bool bVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  long local_138;
  undefined8 *local_130;
  undefined8 local_120;
  undefined8 local_118;
  long local_110;
  long local_108;
  long local_100;
  ulong local_f8;
  long local_f0;
  undefined8 *local_e8;
  long local_e0;
  undefined8 local_d8;
  long local_d0;
  long local_c8;
  void *local_c0;
  void *local_b8;
  long local_b0;
  ulong local_a8;
  long local_a0;
  long local_98;
  char local_88;
  undefined1 local_78 [104];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_110 = 0;
  bVar1 = false;
  local_a0 = 0;
  local_138 = param_2;
  local_130 = param_1;
  do {
    if (local_138 == 0) {
LAB_0010d4c6:
      free(local_b8);
      if (!bVar1) {
        local_e8 = DAT_00123720;
        local_c0 = (void *)FUN_0011819d(local_110,0x10);
        local_e0 = 0;
        for (; local_e8 != (undefined8 *)0x0; local_e8 = (undefined8 *)*local_e8) {
          *(long *)((long)local_c0 + local_e0 * 0x10) = (long)local_e8 + 0xd;
          *(undefined8 **)((long)local_c0 + local_e0 * 0x10 + 8) = local_e8;
          local_e0 = local_e0 + 1;
        }
        FUN_0010cbb9(local_c0,local_110,local_110,param_3);
        free(local_c0);
      }
      FUN_00104b69();
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_d8 = *local_130;
    local_120 = FUN_0010505e(local_d8,&DAT_0011c4cc);
    if (param_4 < 2) {
      local_100 = 0x30;
    }
    else {
      local_f0 = 1;
      for (local_f8 = 1; local_f8 < param_4; local_f8 = local_f8 << 1) {
        local_f0 = local_f0 + 1;
      }
      local_100 = local_f0 << 5;
    }
    if (local_a0 == 0) {
      uVar3 = FUN_0010637c(&local_120,1,local_130,local_138,local_100);
      FUN_00106618(&local_b8,local_100,uVar3);
    }
    local_88 = '\0';
    local_130 = local_130 + 1;
    local_138 = local_138 + -1;
    while (cVar2 = FUN_00106a5e(&local_b8,local_120,local_d8), cVar2 != '\0') {
      if (((local_88 != '\0') && (local_138 != 0)) &&
         (local_100 + 1U < (local_a0 - local_b0) - local_a8 * local_100)) {
        local_98 = local_b0;
        break;
      }
      DAT_00123240 = 0;
      local_d0 = FUN_001066d0(&local_b8);
      if (((local_88 == '\0') || (local_138 != 0)) || ((local_110 != 0 || (local_98 != 0)))) {
        local_110 = local_110 + 1;
        local_108 = FUN_001054f9(&local_118);
        local_108 = local_108 + 0xd;
      }
      else {
        FUN_001050b7(local_120,local_d8);
        local_118 = FUN_0010505e(param_3,&DAT_0011c452);
        bVar1 = true;
        local_108 = param_3;
      }
      if (local_a8 < 2) {
        FUN_0010bd51(local_d0 + -0x20,local_118,local_108);
      }
      else {
        FUN_0010bc07(local_78,param_4);
        local_c8 = FUN_0010b7b6(param_4,local_a8,local_d0);
        FUN_0010c4e8(local_d0,param_4,local_a8,local_c8 + 0x80,local_78,local_118,local_108);
        FUN_0010b8b3(param_4,local_c8);
        FUN_0010bbc5(local_78);
      }
      FUN_001050b7(local_118,local_108);
      if (bVar1) goto LAB_0010d4c6;
    }
    FUN_001050b7(local_120,local_d8);
  } while( true );
}




// Function: insertkey @ 0xd5d8

void insertkey(undefined8 param_1)

{
  long lVar1;
  long *local_18;
  
  lVar1 = FUN_00118564(param_1,0x48);
  for (local_18 = &DAT_00123688; *local_18 != 0; local_18 = (long *)(*local_18 + 0x40)) {
  }
  *local_18 = lVar1;
  *(undefined8 *)(lVar1 + 0x40) = 0;
  return;
}




// Function: badfieldspec @ 0xd640

void badfieldspec(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  uVar1 = FUN_001154a0(param_1);
  uVar2 = gettext(param_2);
  uVar3 = gettext("%s: invalid field specification %s");
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3,uVar2,uVar1);
}




// Function: incompatible_options @ 0xd6a1

void incompatible_options(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = gettext("options \'-%s\' are incompatible");
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar1,param_1);
}




// Function: check_ordering_compatibility @ 0xd6de

void check_ordering_compatibility(void)

{
  long in_FS_OFFSET;
  long local_40;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_40 = DAT_00123688; local_40 != 0; local_40 = *(long *)(local_40 + 0x40)) {
    if (1 < (uint)(byte)(*(long *)(local_40 + 0x20) != 0 |
                        *(byte *)(local_40 + 0x33) | *(byte *)(local_40 + 0x38)) +
            (uint)*(byte *)(local_40 + 0x32) + (uint)*(byte *)(local_40 + 0x34) +
            (uint)*(byte *)(local_40 + 0x35) + (uint)*(byte *)(local_40 + 0x36)) {
      *(undefined1 *)(local_40 + 0x37) = 0;
      *(undefined1 *)(local_40 + 0x31) = *(undefined1 *)(local_40 + 0x37);
      *(undefined1 *)(local_40 + 0x30) = *(undefined1 *)(local_40 + 0x31);
      FUN_0010839e(local_40,local_38);
      FUN_0010d6a1(local_38);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: parse_field_count @ 0xd7de

undefined8 parse_field_count(undefined8 param_1,long *param_2,long param_3)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00118b54(param_1,&local_30,10,&local_28,&DAT_0011b1db);
  switch(uVar1) {
  case 0:
  case 2:
    *param_2 = local_28;
    if (*param_2 == local_28) break;
  case 1:
  case 3:
    *param_2 = -1;
    break;
  case 4:
    if (param_3 != 0) {
      uVar2 = FUN_001154a0(param_1);
      uVar3 = gettext(param_3);
      uVar4 = gettext("%s: invalid count at start of %s");
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar4,uVar3,uVar2);
    }
    local_30 = 0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_30;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sighandler @ 0xd8f9

void sighandler(int param_1)

{
  FUN_00104b8b();
  signal(param_1,(__sighandler_t)0x0);
  raise(param_1);
  return;
}




// Function: set_ordering @ 0xd929

char * set_ordering(char *param_1,long param_2,int param_3)

{
  char *local_10;
  
  for (local_10 = param_1; *local_10 != '\0'; local_10 = local_10 + 1) {
    switch(*local_10) {
    case 'M':
      *(undefined1 *)(param_2 + 0x36) = 1;
      break;
    default:
      return local_10;
    case 'R':
      *(undefined1 *)(param_2 + 0x33) = 1;
      break;
    case 'V':
      *(undefined1 *)(param_2 + 0x38) = 1;
      break;
    case 'b':
      if ((param_3 == 0) || (param_3 == 2)) {
        *(undefined1 *)(param_2 + 0x30) = 1;
      }
      if ((param_3 == 1) || (param_3 == 2)) {
        *(undefined1 *)(param_2 + 0x31) = 1;
      }
      break;
    case 'd':
      *(undefined **)(param_2 + 0x20) = &DAT_00123460;
      break;
    case 'f':
      *(undefined **)(param_2 + 0x28) = &DAT_00123560;
      break;
    case 'g':
      *(undefined1 *)(param_2 + 0x34) = 1;
      break;
    case 'h':
      *(undefined1 *)(param_2 + 0x35) = 1;
      break;
    case 'i':
      if (*(long *)(param_2 + 0x20) == 0) {
        *(undefined **)(param_2 + 0x20) = &DAT_00123360;
      }
      break;
    case 'n':
      *(undefined1 *)(param_2 + 0x32) = 1;
      break;
    case 'r':
      *(undefined1 *)(param_2 + 0x37) = 1;
    }
  }
  return local_10;
}




// Function: key_init @ 0xda55

void * key_init(void *param_1)

{
  memset(param_1,0,0x48);
  *(undefined8 *)((long)param_1 + 0x10) = 0xffffffffffffffff;
  return param_1;
}




// Function: main @ 0xda8d

undefined8 main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  char cVar5;
  int iVar6;
  uint uVar7;
  char *pcVar8;
  char *pcVar9;
  lconv *plVar10;
  char *pcVar11;
  long *plVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  long lVar15;
  ulong uVar16;
  long in_FS_OFFSET;
  bool bVar17;
  bool bVar18;
  char local_338;
  char local_333;
  int local_330;
  long *local_328;
  char *local_320;
  char *local_318;
  ulong local_310;
  ulong local_308;
  undefined8 *local_300;
  char *local_2f8;
  char *local_2f0;
  ulong local_2e8;
  char *local_2e0;
  ulong local_2d8;
  ulong local_2d0;
  undefined1 local_288 [80];
  undefined8 local_238 [4];
  undefined8 local_218;
  undefined8 local_210;
  undefined1 local_208;
  undefined1 local_207;
  undefined1 local_206;
  char local_205;
  undefined1 local_204;
  undefined1 local_203;
  undefined1 local_202;
  undefined1 local_201;
  undefined1 local_200;
  undefined4 local_1e8;
  undefined4 uStack_1e4;
  undefined8 *local_1e0;
  _union_1457 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined4 local_40;
  undefined1 local_22;
  undefined1 local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_330 = 0;
  local_338 = '\0';
  bVar3 = false;
  local_318 = (char *)0x0;
  bVar18 = false;
  local_310 = 0;
  local_308 = 0;
  pcVar8 = getenv("POSIXLY_CORRECT");
  iVar6 = FUN_001133ab();
  if ((iVar6 < 0x30db0) || (0x31068 < iVar6)) {
    bVar2 = true;
  }
  else {
    bVar2 = false;
  }
  local_2f8 = (char *)0x0;
  local_2f0 = (char *)0x0;
  FUN_0011345f(*param_2);
  pcVar9 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00104029(2);
  DAT_00123229 = FUN_001107aa(3);
  DAT_0012322a = FUN_001107aa(2);
  plVar10 = localeconv();
  DAT_00123220 = *plVar10->decimal_point;
  if ((DAT_00123220 == '\0') || (plVar10->decimal_point[1] != '\0')) {
    DAT_00123220 = '.';
  }
  DAT_00123224 = (int)*plVar10->thousands_sep;
  if ((DAT_00123224 != 0) && (plVar10->thousands_sep[1] != '\0')) {
    DAT_00123228 = 1;
  }
  if ((DAT_00123224 == 0) || (plVar10->thousands_sep[1] != '\0')) {
    DAT_00123224 = 0x80;
  }
  DAT_00123683 = '\0';
  FUN_0010594a();
  sigemptyset((sigset_t *)&DAT_001236a0);
  for (local_2e8 = 0; local_2e8 < 0xb; local_2e8 = local_2e8 + 1) {
    sigaction(*(int *)(&DAT_0011d100 + local_2e8 * 4),(sigaction *)0x0,(sigaction *)&local_c8);
    if (local_c8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_001236a0,*(int *)(&DAT_0011d100 + local_2e8 * 4));
    }
  }
  local_c8.sa_handler = FUN_0010d8f9;
  local_c0 = DAT_001236a0;
  local_b8 = DAT_001236a8;
  local_b0 = DAT_001236b0;
  local_a8 = DAT_001236b8;
  local_a0 = DAT_001236c0;
  local_98 = DAT_001236c8;
  local_90 = DAT_001236d0;
  local_88 = DAT_001236d8;
  local_80 = DAT_001236e0;
  local_78 = DAT_001236e8;
  local_70 = DAT_001236f0;
  local_68 = DAT_001236f8;
  local_60 = DAT_00123700;
  local_58 = DAT_00123708;
  local_50 = DAT_00123710;
  local_48 = DAT_00123718;
  local_40 = 0;
  for (local_2e8 = 0; local_2e8 < 0xb; local_2e8 = local_2e8 + 1) {
    iVar6 = sigismember((sigset_t *)&DAT_001236a0,*(int *)(&DAT_0011d100 + local_2e8 * 4));
    if (iVar6 != 0) {
      sigaction(*(int *)(&DAT_0011d100 + local_2e8 * 4),(sigaction *)&local_c8,(sigaction *)0x0);
    }
  }
  signal(0x11,(__sighandler_t)0x0);
  FUN_0011a960(FUN_00104bd4);
  FUN_0010da55(local_238);
  local_238[0] = 0xffffffffffffffff;
  local_300 = (undefined8 *)FUN_0011819d((long)param_1,8);
  do {
    while( true ) {
      while ((local_1e8 = 0xffffffff, local_330 == -1 ||
             ((((pcVar8 != (char *)0x0 && (local_308 != 0)) &&
               (((!bVar2 ||
                 ((((local_338 != '\0' || (param_1 == optind)) || (*(char *)param_2[optind] != '-'))
                  || (*(char *)(param_2[optind] + 1) != 'o')))) ||
                ((*(char *)(param_2[optind] + 2) == '\0' && (param_1 == optind + 1)))))) ||
              (local_330 = getopt_long(param_1,param_2,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",
                                       &PTR_s_ignore_leading_blanks_00122560,&local_1e8),
              local_330 == -1))))) {
        if (param_1 <= optind) {
          if (local_2f8 != (char *)0x0) {
            if (local_308 != 0) {
              uVar13 = FUN_00115059(4,*local_300);
              uVar14 = gettext("extra operand %s");
                    /* WARNING: Subroutine does not return */
              error(0,0,uVar14,uVar13);
            }
            uVar13 = FUN_0010505e(local_2f8,&DAT_0011c4cc);
            FUN_0011676c(&local_1e8);
            cVar5 = FUN_00116b8b(uVar13,&local_1e8);
            if (cVar5 != '\x01') {
              uVar13 = FUN_00115059(4,local_2f8);
              uVar14 = gettext("cannot read file names from %s");
                    /* WARNING: Subroutine does not return */
              error(2,0,uVar14,uVar13);
            }
            FUN_001050b7(uVar13,local_2f8);
            if (CONCAT44(uStack_1e4,local_1e8) == 0) {
              uVar13 = FUN_00115059(4,local_2f8);
              uVar14 = gettext("no input from %s");
                    /* WARNING: Subroutine does not return */
              error(2,0,uVar14,uVar13);
            }
            free(local_300);
            local_300 = local_1e0;
            local_308 = CONCAT44(uStack_1e4,local_1e8);
            for (local_2d8 = 0; local_2d8 < local_308; local_2d8 = local_2d8 + 1) {
              iVar6 = strcmp((char *)local_1e0[local_2d8],"-");
              if (iVar6 == 0) {
                uVar13 = FUN_00115059(4,local_1e0[local_2d8]);
                uVar14 = gettext("when reading file names from stdin, no file name of %s allowed");
                    /* WARNING: Subroutine does not return */
                error(2,0,uVar14,uVar13);
              }
              if (*(char *)local_1e0[local_2d8] == '\0') {
                uVar13 = FUN_001151eb(0,3,local_2f8);
                uVar14 = gettext("%s:%lu: invalid zero-length file name");
                    /* WARNING: Subroutine does not return */
                error(2,0,uVar14,uVar13,local_2d8 + 1);
              }
            }
          }
          for (local_328 = (long *)DAT_00123688; local_328 != (long *)0x0;
              local_328 = *(long **)((long)local_328 + 0x40)) {
            cVar5 = FUN_00108305(local_328);
            if ((cVar5 != '\0') && (*(char *)((long)local_328 + 0x37) != '\x01')) {
              *(undefined8 *)((long)local_328 + 0x20) = local_218;
              *(undefined8 *)((long)local_328 + 0x28) = local_210;
              *(undefined1 *)((long)local_328 + 0x30) = local_208;
              *(undefined1 *)((long)local_328 + 0x31) = local_207;
              *(undefined1 *)((long)local_328 + 0x36) = local_202;
              *(undefined1 *)((long)local_328 + 0x32) = local_206;
              *(undefined1 *)((long)local_328 + 0x34) = local_204;
              *(undefined1 *)((long)local_328 + 0x35) = local_203;
              *(undefined1 *)((long)local_328 + 0x38) = local_200;
              *(char *)((long)local_328 + 0x33) = local_205;
              *(undefined1 *)((long)local_328 + 0x37) = local_201;
            }
            bVar18 = *(char *)((long)local_328 + 0x33) != '\0' || bVar18;
          }
          if ((DAT_00123688 == 0) && (cVar5 = FUN_00108305(local_238), cVar5 != '\x01')) {
            FUN_0010d5d8(local_238);
            bVar18 = local_205 != '\0' || bVar18;
          }
          FUN_0010d6de();
          if (DAT_00123698 != '\0') {
            if ((local_338 != '\0') || (local_2f0 != (char *)0x0)) {
              if (local_338 == '\0') {
                local_338 = 'o';
              }
              s_X___debug_00123028[0] = local_338;
              FUN_0010d6a1(s_X___debug_00123028);
            }
            bVar18 = false;
            if (pcVar9 != (char *)0x0) {
              pcVar8 = setlocale(3,"");
              bVar18 = pcVar8 != (char *)0x0;
            }
            if (!bVar18) {
              uVar13 = gettext("failed to set locale");
                    /* WARNING: Subroutine does not return */
              error(0,0,&DAT_0011ce1c,uVar13);
            }
            if (DAT_00123229 != '\0') {
              pcVar8 = setlocale(3,(char *)0x0);
              uVar13 = FUN_001154a0(pcVar8);
              uVar14 = gettext("text ordering performed using %s sorting rules");
                    /* WARNING: Subroutine does not return */
              error(0,0,uVar14,uVar13);
            }
            uVar13 = gettext("text ordering performed using simple byte comparison");
                    /* WARNING: Subroutine does not return */
            error(0,0,&DAT_0011ce1c,uVar13);
          }
          DAT_00123680 = local_201;
          if (bVar18) {
            FUN_00107573(local_318);
          }
          if (DAT_00123670 == 0) {
            pcVar8 = getenv("TMPDIR");
            if (pcVar8 == (char *)0x0) {
              pcVar8 = "/tmp";
            }
            FUN_0010570f(pcVar8);
          }
          if (local_308 == 0) {
            local_308 = 1;
            free(local_300);
            local_300 = (undefined8 *)FUN_00118024(8);
            *local_300 = &DAT_0011c3bb;
          }
          if ((DAT_00123660 != 0) && (DAT_00123660 <= (ulong)DAT_00123024 * 0x22)) {
            DAT_00123660 = (ulong)DAT_00123024 * 0x22;
          }
          if (local_338 == '\0') {
            FUN_0010ca81(local_300,local_308);
            FUN_0010cb44(local_2f0);
            if (bVar3) {
              lVar15 = FUN_00118502(local_308,0x10);
              for (local_2d0 = 0; local_2d0 < local_308; local_2d0 = local_2d0 + 1) {
                *(undefined8 *)(lVar15 + local_2d0 * 0x10) = local_300[local_2d0];
              }
              FUN_0010cbb9(lVar15,0,local_308,local_2f0);
            }
            else {
              if ((local_310 == 0) && (local_310 = FUN_00112de4(2), 8 < local_310)) {
                local_310 = 8;
              }
              uVar16 = 0xffffffffffffff;
              if (local_310 < 0x100000000000000) {
                uVar16 = local_310;
              }
              FUN_0010d0f9(local_300,local_308,local_2f0,uVar16);
            }
            if ((DAT_00123683 != '\0') && (iVar6 = FUN_0010ff17(stdin), iVar6 == -1)) {
              uVar13 = gettext("close failed");
              FUN_00104464(uVar13,&DAT_0011c3bb);
            }
            if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
              __stack_chk_fail();
            }
            return 0;
          }
          if (1 < local_308) {
            uVar13 = FUN_00115059(4,local_300[1]);
            uVar14 = gettext("extra operand %s not allowed with -%c");
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar14,uVar13,(int)local_338);
          }
          if (local_2f0 != (char *)0x0) {
            DAT_00123032 = local_338;
            FUN_0010d6a1(&DAT_00123032);
          }
          uVar7 = FUN_0010a0b1(*local_300,(int)local_338);
                    /* WARNING: Subroutine does not return */
          exit((uVar7 ^ 1) & 0xff);
        }
        lVar15 = (long)optind;
        optind = optind + 1;
        local_300[local_308] = param_2[lVar15];
        local_308 = local_308 + 1;
      }
      if (local_330 < 0x88) break;
switchD_0010e0f8_caseD_2:
      FUN_001044d3(2);
    }
    if (local_330 < 1) {
      if (local_330 == -0x83) {
LAB_0010eb8e:
        FUN_00117e44(stdout,&DAT_0011c185,"GNU coreutils",PTR_DAT_00123108,"Mike Haertel",
                     "Paul Eggert",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (local_330 == -0x82) {
        FUN_001044d3(0);
        goto LAB_0010eb8e;
      }
      goto switchD_0010e0f8_caseD_2;
    }
    cVar5 = (char)local_330;
    switch(local_330) {
    case 1:
      local_328 = (long *)0x0;
      if (*optarg == '+') {
        if (((param_1 == optind) || (*(char *)param_2[optind] != '-')) ||
           (9 < (int)*(char *)(param_2[optind] + 1) - 0x30U)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        if ((!bVar4) || (pcVar8 != (char *)0x0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        bVar17 = bVar1 || bVar2;
        bVar1 = bVar1 || bVar2;
        bVar2 = bVar17;
        if (bVar1) {
          local_328 = (long *)FUN_0010da55(local_288);
          local_320 = (char *)FUN_0010d7de(optarg + 1,local_328,0);
          if ((local_320 != (char *)0x0) && (*local_320 == '.')) {
            local_320 = (char *)FUN_0010d7de(local_320 + 1,local_328 + 1,0);
          }
          if ((*local_328 == 0) && (local_328[1] == 0)) {
            *local_328 = -1;
          }
          if ((local_320 == (char *)0x0) ||
             (pcVar11 = (char *)FUN_0010d929(local_320,local_328,0), *pcVar11 != '\0')) {
            local_328 = (long *)0x0;
          }
          else {
            if (bVar4) {
              lVar15 = param_2[optind];
              optind = optind + 1;
              local_320 = (char *)FUN_0010d7de(lVar15 + 1,local_328 + 2,"invalid number after \'-\'"
                                              );
              if (local_320 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                __assert_fail("s","src/sort.c",0x1161,"main");
              }
              if (*local_320 == '.') {
                local_320 = (char *)FUN_0010d7de(local_320 + 1,local_328 + 3,
                                                 "invalid number after \'.\'");
              }
              if ((local_328[3] == 0) && (local_328[2] != 0)) {
                local_328[2] = local_328[2] + -1;
              }
              pcVar11 = (char *)FUN_0010d929(local_320,local_328,1);
              if (*pcVar11 != '\0') {
                FUN_0010d640(lVar15,"stray character in field spec");
              }
            }
            *(undefined1 *)((long)local_328 + 0x39) = 1;
            FUN_0010d5d8(local_328);
          }
        }
      }
      if (local_328 == (long *)0x0) {
        local_300[local_308] = optarg;
        local_308 = local_308 + 1;
      }
      break;
    default:
      goto switchD_0010e0f8_caseD_2;
    case 0x53:
      FUN_00105e0c(local_1e8,(int)cVar5,optarg);
      break;
    case 0x54:
      FUN_0010570f(optarg);
      break;
    case 0x6b:
      plVar12 = (long *)FUN_0010da55(local_288);
      local_320 = (char *)FUN_0010d7de(optarg,plVar12,"invalid number at field start");
      lVar15 = *plVar12;
      *plVar12 = lVar15 + -1;
      if (lVar15 == 0) {
        FUN_0010d640(optarg,"field number is zero");
      }
      if (*local_320 == '.') {
        local_320 = (char *)FUN_0010d7de(local_320 + 1,plVar12 + 1,"invalid number after \'.\'");
        lVar15 = plVar12[1];
        plVar12[1] = lVar15 + -1;
        if (lVar15 == 0) {
          FUN_0010d640(optarg,"character offset is zero");
        }
      }
      if ((*plVar12 == 0) && (plVar12[1] == 0)) {
        *plVar12 = -1;
      }
      local_320 = (char *)FUN_0010d929(local_320,plVar12,0);
      if (*local_320 == ',') {
        local_320 = (char *)FUN_0010d7de(local_320 + 1,plVar12 + 2,"invalid number after \',\'");
        lVar15 = plVar12[2];
        plVar12[2] = lVar15 + -1;
        if (lVar15 == 0) {
          FUN_0010d640(optarg,"field number is zero");
        }
        if (*local_320 == '.') {
          local_320 = (char *)FUN_0010d7de(local_320 + 1,plVar12 + 3,"invalid number after \'.\'");
        }
        local_320 = (char *)FUN_0010d929(local_320,plVar12,1);
      }
      else {
        plVar12[2] = -1;
        plVar12[3] = 0;
      }
      if (*local_320 != '\0') {
        FUN_0010d640(optarg,"stray character in field spec");
      }
      FUN_0010d5d8(plVar12);
      break;
    case 0x6d:
      bVar3 = true;
      break;
    case 0x6f:
      if ((local_2f0 != (char *)0x0) && (iVar6 = strcmp(local_2f0,optarg), iVar6 != 0)) {
        uVar13 = gettext("multiple output files specified");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar13);
      }
      local_2f0 = optarg;
      break;
    case 0x73:
      DAT_00123681 = 1;
      break;
    case 0x74:
      local_333 = *optarg;
      if (local_333 == '\0') {
        uVar13 = gettext("empty tab");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar13);
      }
      if (optarg[1] != '\0') {
        iVar6 = strcmp(optarg,"\\0");
        if (iVar6 != 0) {
          uVar13 = FUN_001154a0(optarg);
          uVar14 = gettext("multi-character tab %s");
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar14,uVar13);
        }
        local_333 = '\0';
      }
      if ((DAT_00123020 != 0x80) && (local_333 != DAT_00123020)) {
        uVar13 = gettext("incompatible tabs");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar13);
      }
      DAT_00123020 = (int)local_333;
      break;
    case 0x75:
      DAT_00123682 = 1;
      break;
    case 0x79:
      if ((char *)param_2[(long)optind + -1] == optarg) {
        for (local_2e0 = optarg; (int)*local_2e0 - 0x30U < 10; local_2e0 = local_2e0 + 1) {
        }
        optind = optind - (uint)(*local_2e0 != '\0');
      }
      break;
    case 0x7a:
      DAT_00123010 = 0;
      break;
    case 0x80:
      if (optarg == (char *)0x0) {
        local_330 = 99;
      }
      else {
        lVar15 = FUN_0010f8c1("--check",optarg,&PTR_s_quiet_00122940,"CCcgeneral-numeric",1,
                              PTR_FUN_00123110,1);
        local_330 = (int)"CCcgeneral-numeric"[lVar15];
      }
    case 0x43:
    case 99:
      if ((local_338 != '\0') && (local_330 != local_338)) {
        FUN_0010d6a1(&DAT_0011cbf9);
      }
      local_338 = (char)local_330;
      break;
    case 0x81:
      if ((DAT_00123690 != (char *)0x0) && (iVar6 = strcmp(DAT_00123690,optarg), iVar6 != 0)) {
        uVar13 = gettext("multiple compress programs specified");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar13);
      }
      DAT_00123690 = optarg;
      break;
    case 0x82:
      DAT_00123698 = '\x01';
      break;
    case 0x83:
      local_2f8 = optarg;
      break;
    case 0x84:
      FUN_00105ba8(local_1e8,(int)cVar5,optarg);
      break;
    case 0x85:
      if ((local_318 != (char *)0x0) && (iVar6 = strcmp(local_318,optarg), iVar6 != 0)) {
        uVar13 = gettext("multiple random sources specified");
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar13);
      }
      local_318 = optarg;
      break;
    case 0x86:
      lVar15 = FUN_0010f8c1("--sort",optarg,&DAT_00122960,"ghMnRVwaiting for %s [-d]",1,
                            PTR_FUN_00123110,1);
      local_330 = (int)"ghMnRVwaiting for %s [-d]"[lVar15];
    case 0x4d:
    case 0x52:
    case 0x56:
    case 0x62:
    case 100:
    case 0x66:
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6e:
    case 0x72:
      local_22 = (undefined1)local_330;
      local_21 = 0;
      FUN_0010d929(&local_22,local_238,2);
      break;
    case 0x87:
      local_310 = FUN_00106042(local_1e8,(int)cVar5,optarg);
    }
  } while( true );
}




// Function: hash_get_n_buckets @ 0x10854

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0x1086a

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0x10880

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0x10896

ulong hash_get_max_bucket_length(undefined8 *param_1)

{
  long *local_28;
  ulong local_20;
  long *local_18;
  ulong local_10;
  
  local_20 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_18 = local_28;
      local_10 = 1;
      while (local_18 = (long *)local_18[1], local_18 != (long *)0x0) {
        local_10 = local_10 + 1;
      }
      if (local_20 < local_10) {
        local_20 = local_10;
      }
    }
  }
  return local_20;
}




// Function: hash_table_ok @ 0x10918

undefined8 hash_table_ok(undefined8 *param_1)

{
  undefined8 uVar1;
  long *local_28;
  long local_20;
  long local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_10 = local_28;
      local_20 = local_20 + 1;
      do {
        local_18 = local_18 + 1;
        local_10 = (long *)local_10[1];
      } while (local_10 != (long *)0x0);
    }
  }
  if ((local_20 == param_1[3]) && (local_18 == param_1[4])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_print_statistics @ 0x109b6

void hash_print_statistics(undefined8 param_1,FILE *param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_00110880(param_1);
  uVar2 = FUN_00110854(param_1);
  uVar3 = FUN_0011086a(param_1);
  uVar4 = FUN_00110896(param_1);
  fprintf(param_2,"# entries:         %lu\n",uVar1);
  fprintf(param_2,"# buckets:         %lu\n",uVar2);
  fprintf(param_2,"# buckets used:    %lu (%.2f%%)\n",(DAT_0011d278 * (double)uVar3) / (double)uVar2
          ,uVar3);
  fprintf(param_2,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: safe_hasher @ 0x10afd

long safe_hasher(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  
  uVar1 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar1) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return uVar1 * 0x10 + *param_1;
}




// Function: hash_lookup @ 0x10b58

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *local_18;
  
  local_18 = (long *)FUN_00110afd(param_1,param_2);
  if (*local_18 != 0) {
    for (; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
      if ((param_2 == *local_18) ||
         (cVar1 = (**(code **)(param_1 + 0x38))(param_2,*local_18), cVar1 != '\0')) {
        return *local_18;
      }
    }
  }
  return 0;
}




// Function: hash_get_first @ 0x10bef

long hash_get_first(undefined8 *param_1)

{
  long lVar1;
  long *local_10;
  
  if (param_1[4] == 0) {
    lVar1 = 0;
  }
  else {
    local_10 = (long *)*param_1;
    while( true ) {
      if ((long *)param_1[1] <= local_10) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (*local_10 != 0) break;
      local_10 = local_10 + 2;
    }
    lVar1 = *local_10;
  }
  return lVar1;
}




// Function: hash_get_next @ 0x10c4f

long hash_get_next(long param_1,long param_2)

{
  long *local_18;
  long *local_10;
  
  local_18 = (long *)FUN_00110afd(param_1,param_2);
  local_10 = local_18;
  while ((param_2 != *local_10 || (local_10[1] == 0))) {
    local_10 = (long *)local_10[1];
    if (local_10 == (long *)0x0) {
      do {
        local_18 = local_18 + 2;
        if (*(long **)(param_1 + 8) <= local_18) {
          return 0;
        }
      } while (*local_18 == 0);
      return *local_18;
    }
  }
  return *(long *)local_10[1];
}




// Function: hash_get_entries @ 0x10ced

ulong hash_get_entries(undefined8 *param_1,long param_2,ulong param_3)

{
  ulong local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        if (param_3 <= local_20) {
          return local_20;
        }
        *(long *)(local_20 * 8 + param_2) = *local_10;
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_do_for_each @ 0x10d8d

long hash_do_for_each(undefined8 *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        cVar1 = (*param_2)(*local_10,param_3);
        if (cVar1 != '\x01') {
          return local_20;
        }
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_string @ 0x10e25

ulong hash_string(byte *param_1,ulong param_2)

{
  byte *local_20;
  ulong local_10;
  
  local_10 = 0;
  for (local_20 = param_1; *local_20 != 0; local_20 = local_20 + 1) {
    local_10 = ((ulong)*local_20 + local_10 * 0x1f) % param_2;
  }
  return local_10;
}




// Function: is_prime @ 0x10e7f

undefined8 is_prime(ulong param_1)

{
  undefined8 local_18;
  undefined8 local_10;
  
  local_10 = 9;
  for (local_18 = 3; (local_10 < param_1 && (param_1 % local_18 != 0)); local_18 = local_18 + 2) {
    local_10 = local_10 + (local_18 + 1) * 4;
  }
  return CONCAT71((int7)(param_1 % local_18 >> 8),param_1 % local_18 != 0);
}




// Function: next_prime @ 0x10eee

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong local_10;
  
  local_10 = param_1;
  if (param_1 < 10) {
    local_10 = 10;
  }
  local_10 = local_10 | 1;
  while( true ) {
    if (local_10 == 0xffffffffffffffff) {
      return 0xffffffffffffffff;
    }
    cVar1 = FUN_00110e7f(local_10);
    if (cVar1 == '\x01') break;
    local_10 = local_10 + 2;
  }
  return local_10;
}




// Function: hash_reset_tuning @ 0x10f39

void hash_reset_tuning(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_0011d280;
  param_1[2] = DAT_0011d284;
  param_1[3] = DAT_0011d288;
  *(undefined1 *)(param_1 + 4) = 0;
  return;
}




// Function: raw_hasher @ 0x10f8f

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = FUN_00119063(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0x10fca

bool raw_comparator(long param_1,long param_2)

{
  return param_1 == param_2;
}




// Function: check_tuning @ 0x10fe7

undefined8 check_tuning(long param_1)

{
  float *pfVar1;
  undefined8 uVar2;
  
  pfVar1 = *(float **)(param_1 + 0x28);
  if (pfVar1 == (float *)&DAT_0011d1f0) {
    uVar2 = 1;
  }
  else if ((((pfVar1[2] <= DAT_0011d28c) || (DAT_0011d280 - DAT_0011d28c <= pfVar1[2])) ||
           (pfVar1[3] <= DAT_0011d28c + DAT_0011d280)) ||
          (((*pfVar1 < 0.0 || (pfVar1[1] <= *pfVar1 + DAT_0011d28c)) ||
           ((DAT_0011d280 < pfVar1[1] || (pfVar1[2] <= *pfVar1 + DAT_0011d28c)))))) {
    *(undefined **)(param_1 + 0x28) = &DAT_0011d1f0;
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: compute_bucket_size @ 0x110f4

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong compute_bucket_size(ulong param_1,long param_2)

{
  ulong uVar1;
  float fVar2;
  ulong local_20;
  
  local_20 = param_1;
  if (*(char *)(param_2 + 0x10) != '\x01') {
    fVar2 = (float)param_1 / *(float *)(param_2 + 8);
    if (_DAT_0011d290 <= fVar2) {
      return 0;
    }
    if (_DAT_0011d294 <= fVar2) {
      local_20 = (long)(fVar2 - _DAT_0011d294) ^ 0x8000000000000000;
    }
    else {
      local_20 = (ulong)fVar2;
    }
  }
  uVar1 = FUN_00110eee(local_20);
  if ((long)(uVar1 << 3) < 0 || uVar1 >> 0x3d != 0) {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_initialize @ 0x1120d

long * hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,long param_5)

{
  char cVar1;
  long *__ptr;
  long lVar2;
  void *pvVar3;
  code *local_38;
  code *local_30;
  undefined *local_28;
  
  local_30 = param_3;
  if (param_3 == (code *)0x0) {
    local_30 = FUN_00110f8f;
  }
  local_38 = param_4;
  if (param_4 == (code *)0x0) {
    local_38 = FUN_00110fca;
  }
  __ptr = malloc(0x50);
  if (__ptr != (long *)0x0) {
    local_28 = param_2;
    if (param_2 == (undefined *)0x0) {
      local_28 = &DAT_0011d1f0;
    }
    __ptr[5] = (long)local_28;
    cVar1 = FUN_00110fe7(__ptr);
    if (cVar1 == '\x01') {
      lVar2 = FUN_001110f4(param_1,local_28);
      __ptr[2] = lVar2;
      if (__ptr[2] != 0) {
        pvVar3 = calloc(__ptr[2],0x10);
        *__ptr = (long)pvVar3;
        if (*__ptr != 0) {
          __ptr[1] = *__ptr + __ptr[2] * 0x10;
          __ptr[3] = 0;
          __ptr[4] = 0;
          __ptr[6] = (long)local_30;
          __ptr[7] = (long)local_38;
          __ptr[8] = param_5;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (long *)0x0;
}




// Function: hash_clear @ 0x11382

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  long *local_20;
  undefined8 *local_18;
  
  for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
    if (*local_20 != 0) {
      local_18 = (undefined8 *)local_20[1];
      while (local_18 != (undefined8 *)0x0) {
        if (param_1[8] != 0) {
          (*(code *)param_1[8])(*local_18);
        }
        *local_18 = 0;
        puVar1 = (undefined8 *)local_18[1];
        local_18[1] = param_1[9];
        param_1[9] = local_18;
        local_18 = puVar1;
      }
      if (param_1[8] != 0) {
        (*(code *)param_1[8])(*local_20);
      }
      *local_20 = 0;
      local_20[1] = 0;
    }
  }
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x1148d

void hash_free(ulong *param_1)

{
  void *pvVar1;
  long *local_20;
  long *local_18;
  
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
      if (*local_20 != 0) {
        for (local_18 = local_20; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
          (*(code *)param_1[8])(*local_18);
        }
      }
    }
  }
  for (local_20 = (long *)*param_1; local_20 < param_1[1];
      local_20 = (long *)((long)local_20 + 0x10)) {
    local_18 = *(void **)((long)local_20 + 8);
    while (local_18 != (void *)0x0) {
      pvVar1 = *(void **)((long)local_18 + 8);
      free(local_18);
      local_18 = pvVar1;
    }
  }
  local_18 = (void *)param_1[9];
  while (local_18 != (void *)0x0) {
    pvVar1 = *(void **)((long)local_18 + 8);
    free(local_18);
    local_18 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: allocate_entry @ 0x115bc

void * allocate_entry(long param_1)

{
  undefined8 local_10;
  
  if (*(long *)(param_1 + 0x48) == 0) {
    local_10 = malloc(0x10);
  }
  else {
    local_10 = *(void **)(param_1 + 0x48);
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)((long)local_10 + 8);
  }
  return local_10;
}




// Function: free_entry @ 0x1160b

void free_entry(long param_1,undefined8 *param_2)

{
  *param_2 = 0;
  param_2[1] = *(undefined8 *)(param_1 + 0x48);
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: hash_find_entry @ 0x11645

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  long *local_38;
  
  local_38 = (long *)FUN_00110afd(param_1,param_2);
  *param_3 = local_38;
  if (*local_38 == 0) {
    lVar4 = 0;
  }
  else if ((param_2 == *local_38) ||
          (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*local_38), cVar3 != '\0')) {
    lVar4 = *local_38;
    if (param_4 != '\0') {
      if (local_38[1] == 0) {
        *local_38 = 0;
      }
      else {
        plVar1 = (long *)local_38[1];
        lVar2 = plVar1[1];
        *local_38 = *plVar1;
        local_38[1] = lVar2;
        FUN_0011160b(param_1,plVar1);
      }
    }
  }
  else {
    for (; local_38[1] != 0; local_38 = (long *)local_38[1]) {
      if ((param_2 == *(long *)local_38[1]) ||
         (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*(undefined8 *)local_38[1]), cVar3 != '\0'))
      {
        lVar4 = *(long *)local_38[1];
        if (param_4 == '\0') {
          return lVar4;
        }
        lVar2 = local_38[1];
        local_38[1] = *(long *)(lVar2 + 8);
        FUN_0011160b(param_1,lVar2);
        return lVar4;
      }
    }
    lVar4 = 0;
  }
  return lVar4;
}




// Function: transfer_entries @ 0x117de

undefined8 transfer_entries(long param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  long *local_38;
  long *local_30;
  
  local_38 = (long *)*param_2;
  do {
    if ((long *)param_2[1] <= local_38) {
      return 1;
    }
    if (*local_38 != 0) {
      local_30 = (long *)local_38[1];
      while (local_30 != (long *)0x0) {
        lVar1 = *local_30;
        plVar3 = (long *)FUN_00110afd(param_1,lVar1);
        plVar2 = (long *)local_30[1];
        if (*plVar3 == 0) {
          *plVar3 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0011160b(param_1,local_30);
          local_30 = plVar2;
        }
        else {
          local_30[1] = plVar3[1];
          plVar3[1] = (long)local_30;
          local_30 = plVar2;
        }
      }
      lVar1 = *local_38;
      local_38[1] = 0;
      if (param_3 == '\0') {
        plVar2 = (long *)FUN_00110afd(param_1,lVar1);
        if (*plVar2 == 0) {
          *plVar2 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
        }
        else {
          plVar3 = (long *)FUN_001115bc(param_1);
          if (plVar3 == (long *)0x0) {
            return 0;
          }
          *plVar3 = lVar1;
          plVar3[1] = plVar2[1];
          plVar2[1] = (long)plVar3;
        }
        *local_38 = 0;
        param_2[3] = param_2[3] + -1;
      }
    }
    local_38 = local_38 + 2;
  } while( true );
}




// Function: hash_rehash @ 0x119b2

undefined8 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  size_t __nmemb;
  undefined8 uVar2;
  long in_FS_OFFSET;
  void *local_68;
  void *local_60;
  size_t local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_001110f4(param_2,param_1[5]);
  if (__nmemb == 0) {
    uVar2 = 0;
  }
  else if (__nmemb == param_1[2]) {
    uVar2 = 1;
  }
  else {
    local_68 = calloc(__nmemb,0x10);
    if (local_68 == (void *)0x0) {
      uVar2 = 0;
    }
    else {
      local_60 = (void *)((long)local_68 + __nmemb * 0x10);
      local_50 = 0;
      local_48 = 0;
      local_40 = param_1[5];
      local_38 = param_1[6];
      local_30 = param_1[7];
      local_28 = param_1[8];
      local_20 = param_1[9];
      local_58 = __nmemb;
      cVar1 = FUN_001117de(&local_68,param_1,0);
      if (cVar1 == '\0') {
        param_1[9] = local_20;
        cVar1 = FUN_001117de(param_1,&local_68,1);
        if (cVar1 == '\x01') {
          cVar1 = FUN_001117de(param_1,&local_68,0);
          if (cVar1 == '\x01') {
            free(local_68);
            uVar2 = 0;
            goto LAB_00111bc7;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_68;
      param_1[1] = local_60;
      param_1[2] = local_58;
      param_1[3] = local_50;
      param_1[9] = local_20;
      uVar2 = 1;
    }
  }
LAB_00111bc7:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert_if_absent @ 0x11bdd

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  long in_FS_OFFSET;
  float fVar5;
  long *local_30;
  long local_28;
  long local_20;
  long *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  local_28 = FUN_00111645(param_1,param_2,&local_30,0);
  if (local_28 != 0) {
    if (param_3 != (long *)0x0) {
      *param_3 = local_28;
    }
    uVar2 = 0;
    goto LAB_00111f38;
  }
  if (((float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
       (float)*(ulong *)(param_1 + 0x18)) &&
     (FUN_00110fe7(param_1),
     (float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
     (float)*(ulong *)(param_1 + 0x18))) {
    local_20 = *(long *)(param_1 + 0x28);
    if (*(char *)(local_20 + 0x10) == '\0') {
      fVar5 = *(float *)(local_20 + 8) *
              *(float *)(local_20 + 0xc) * (float)*(ulong *)(param_1 + 0x10);
    }
    else {
      fVar5 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_20 + 0xc);
    }
    if (_DAT_0011d290 <= fVar5) {
      uVar2 = 0xffffffff;
      goto LAB_00111f38;
    }
    if (_DAT_0011d294 <= fVar5) {
      uVar3 = (long)(fVar5 - _DAT_0011d294) ^ 0x8000000000000000;
    }
    else {
      uVar3 = (ulong)fVar5;
    }
    cVar1 = FUN_001119b2(param_1,uVar3);
    if (cVar1 != '\x01') {
      uVar2 = 0xffffffff;
      goto LAB_00111f38;
    }
    lVar4 = FUN_00111645(param_1,param_2,&local_30,0);
    if (lVar4 != 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
  }
  if (*local_30 == 0) {
    *local_30 = param_2;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
    *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    uVar2 = 1;
  }
  else {
    local_18 = (long *)FUN_001115bc(param_1);
    if (local_18 == (long *)0x0) {
      uVar2 = 0xffffffff;
    }
    else {
      *local_18 = param_2;
      local_18[1] = local_30[1];
      local_30[1] = (long)local_18;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      uVar2 = 1;
    }
  }
LAB_00111f38:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert @ 0x11f4e

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00111bdd(param_1,param_2,&local_18);
  if (iVar1 == -1) {
    param_2 = 0;
  }
  else if (iVar1 == 0) {
    param_2 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_remove @ 0x11fbe

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long hash_remove(long param_1,undefined8 param_2)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  float fVar3;
  long *local_40;
  void *local_38;
  long local_30;
  long local_28;
  ulong local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = FUN_00111645(param_1,param_2,&local_40,1);
  if (local_30 == 0) {
    lVar2 = 0;
  }
  else {
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1;
    lVar2 = local_30;
    if (((*local_40 == 0) &&
        (*(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + -1,
        (float)*(ulong *)(param_1 + 0x18) <
        (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) &&
       (FUN_00110fe7(param_1), lVar2 = local_30,
       (float)*(ulong *)(param_1 + 0x18) <
       (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) {
      local_28 = *(long *)(param_1 + 0x28);
      if (*(char *)(local_28 + 0x10) == '\0') {
        fVar3 = *(float *)(local_28 + 8) *
                *(float *)(local_28 + 4) * (float)*(ulong *)(param_1 + 0x10);
        if (_DAT_0011d294 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_0011d294) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      else {
        fVar3 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_28 + 4);
        if (_DAT_0011d294 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_0011d294) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      cVar1 = FUN_001119b2(param_1,local_20);
      lVar2 = local_30;
      if (cVar1 != '\x01') {
        local_38 = *(void **)(param_1 + 0x48);
        while (local_38 != (void *)0x0) {
          local_18 = *(void **)((long)local_38 + 8);
          free(local_38);
          local_38 = local_18;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
        lVar2 = local_30;
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar2;
}




// Function: hash_delete @ 0x122c4

void hash_delete(undefined8 param_1,undefined8 param_2)

{
  FUN_00111fbe(param_1,param_2);
  return;
}




// Function: num_processors_via_affinity_mask @ 0x12bc5

long num_processors_via_affinity_mask(void)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  cpu_set_t local_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = sched_getaffinity(0,0x80,&local_98);
  if (iVar1 == 0) {
    iVar1 = __sched_cpucount(0x80,&local_98);
    lVar2 = (long)iVar1;
    if (lVar2 != 0) goto LAB_00112c35;
  }
  lVar2 = 0;
LAB_00112c35:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: num_processors_ignoring_omp @ 0x12c4b

ulong num_processors_ignoring_omp(int param_1)

{
  ulong uVar1;
  ulong uVar2;
  
  if (param_1 == 1) {
    uVar1 = FUN_00112bc5();
    if (uVar1 != 0) {
      return uVar1;
    }
    uVar1 = sysconf(0x54);
  }
  else {
    uVar1 = sysconf(0x53);
    if (((uVar1 == 1) || (uVar1 == 2)) && (uVar2 = FUN_00112bc5(), uVar1 < uVar2)) {
      uVar1 = uVar2;
    }
  }
  if ((long)uVar1 < 1) {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: parse_omp_threads @ 0x12cdc

ulong parse_omp_threads(char *param_1)

{
  char cVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  char *local_30;
  char *local_28;
  ulong local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = 0;
  local_30 = param_1;
  if (param_1 == (char *)0x0) {
    uVar2 = 0;
  }
  else {
    while ((*local_30 != '\0' && (cVar1 = FUN_00119332((int)*local_30), cVar1 != '\0'))) {
      local_30 = local_30 + 1;
    }
    cVar1 = FUN_00119258((int)*local_30);
    if (cVar1 != '\0') {
      local_28 = (char *)0x0;
      local_18 = strtoul(local_30,&local_28,10);
      if (local_28 != (char *)0x0) {
        while ((*local_28 != '\0' && (cVar1 = FUN_00119332((int)*local_28), cVar1 != '\0'))) {
          local_28 = local_28 + 1;
        }
        uVar2 = local_18;
        if ((*local_28 == '\0') || (*local_28 == ',')) goto LAB_00112dce;
      }
    }
    uVar2 = local_20;
  }
LAB_00112dce:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: num_processors @ 0x12de4

ulong num_processors(int param_1)

{
  char *pcVar1;
  ulong uVar2;
  int local_2c;
  ulong local_20;
  
  local_20 = 0xffffffffffffffff;
  local_2c = param_1;
  if (param_1 == 2) {
    pcVar1 = getenv("OMP_NUM_THREADS");
    uVar2 = FUN_00112cdc(pcVar1);
    pcVar1 = getenv("OMP_THREAD_LIMIT");
    local_20 = FUN_00112cdc(pcVar1);
    if (local_20 == 0) {
      local_20 = 0xffffffffffffffff;
    }
    if (uVar2 != 0) {
      if (uVar2 < local_20) {
        return uVar2;
      }
      return local_20;
    }
    local_2c = 1;
  }
  uVar2 = FUN_00112c4b(local_2c);
  if (local_20 <= uVar2) {
    uVar2 = local_20;
  }
  return uVar2;
}



