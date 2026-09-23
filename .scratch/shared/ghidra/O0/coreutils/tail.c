// Function: usage @ 0x4115

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_00118150;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext("Usage: %s [OPTION]... [FILE]...\n");
    printf(pcVar3,uVar2);
    pcVar3 = (char *)gettext(
                            "Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"
                            );
    printf(pcVar3,10);
    FUN_00102a69();
    FUN_00102a9a();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pcVar3 = (char *)gettext(
                            "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"
                            );
    printf(pcVar3,10,5);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00102acb(&DAT_00112ca5);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: die_pipe @ 0x431c

void die_pipe(void)

{
  raise(0xd);
                    /* WARNING: Subroutine does not return */
  exit(1);
}




// Function: check_output_alive @ 0x4338

void check_output_alive(void)

{
  int iVar1;
  long in_FS_OFFSET;
  pollfd local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0011812f == '\x01') {
    local_18.fd = 1;
    local_18.revents = 0;
    local_18.events = 0;
    iVar1 = poll(&local_18,1,0);
    if ((-1 < iVar1) && ((local_18.revents & 0x18U) != 0)) {
      FUN_0010431c();
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: valid_file_spec @ 0x43ba

bool valid_file_spec(long param_1)

{
  return (*(int *)(param_1 + 0x3c) == 0) != (*(int *)(param_1 + 0x38) == -1);
}




// Function: pretty_name @ 0x43ee

undefined8 pretty_name(undefined8 *param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = strcmp((char *)*param_1,"-");
  if (iVar1 == 0) {
    uVar2 = gettext("standard input");
  }
  else {
    uVar2 = *param_1;
  }
  return uVar2;
}




// Function: record_open_fd @ 0x4435

void record_open_fd(long param_1,undefined4 param_2,undefined8 param_3,undefined8 *param_4,
                 undefined4 param_5)

{
  undefined1 auVar1 [16];
  
  *(undefined4 *)(param_1 + 0x38) = param_2;
  *(undefined8 *)(param_1 + 8) = param_3;
  auVar1 = FUN_0010ee31(param_4);
  *(undefined1 (*) [16])(param_1 + 0x10) = auVar1;
  *(undefined8 *)(param_1 + 0x20) = *param_4;
  *(undefined8 *)(param_1 + 0x28) = param_4[1];
  *(undefined4 *)(param_1 + 0x30) = *(undefined4 *)(param_4 + 3);
  *(undefined4 *)(param_1 + 0x40) = param_5;
  *(undefined8 *)(param_1 + 0x58) = 0;
  *(undefined1 *)(param_1 + 0x34) = 0;
  return;
}




// Function: close_fd @ 0x44d5

void close_fd(int param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if ((param_1 != -1) && (param_1 != 0)) {
    iVar1 = close(param_1);
    if (iVar1 != 0) {
      uVar2 = FUN_0010e8ca(4,param_2);
      uVar3 = gettext("closing %s (fd=%d)");
      piVar4 = __errno_location();
      error(0,*piVar4,uVar3,uVar2,param_1);
    }
  }
  return;
}




// Function: write_header @ 0x4559

void write_header(undefined8 param_1)

{
  undefined *puVar1;
  
  if (DAT_00118020 == '\0') {
    puVar1 = &DAT_00112cce;
  }
  else {
    puVar1 = &DAT_001121c3;
  }
  printf("%s==> %s <==\n",puVar1,param_1);
  DAT_00118020 = 0;
  return;
}




// Function: xwrite_stdout @ 0x45a9

void xwrite_stdout(void *param_1,ulong param_2)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (param_2 != 0) {
    sVar1 = fwrite_unlocked(param_1,1,param_2,stdout);
    if (sVar1 < param_2) {
      clearerr_unlocked(stdout);
      uVar2 = FUN_0010e8ca(4,"standard output");
      uVar3 = gettext("error writing %s");
      piVar4 = __errno_location();
      error(1,*piVar4,uVar3,uVar2);
    }
  }
  return;
}




// Function: dump_remainder @ 0x4649

long dump_remainder(char param_1,undefined8 param_2,undefined4 param_3,ulong param_4)

{
  ulong uVar1;
  long lVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  char local_204c;
  long local_2048;
  ulong local_2040;
  undefined1 local_2028 [8200];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2048 = 0;
  local_204c = param_1;
  local_2040 = param_4;
  do {
    uVar1 = local_2040;
    if (0x2000 < local_2040) {
      uVar1 = 0x2000;
    }
    lVar2 = FUN_0010ed34(param_3,local_2028,uVar1);
    if (lVar2 == -1) {
      piVar3 = __errno_location();
      if (*piVar3 == 0xb) {
LAB_001047dd:
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          return local_2048;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      uVar4 = FUN_0010e8ca(4,param_2);
      uVar5 = gettext("error reading %s");
      piVar3 = __errno_location();
      error(1,*piVar3,uVar5,uVar4);
    }
    if (lVar2 == 0) goto LAB_001047dd;
    if (local_204c != '\0') {
      FUN_00104559(param_2);
      local_204c = '\0';
    }
    FUN_001045a9(local_2028,lVar2);
    local_2048 = local_2048 + lVar2;
    if ((param_4 != 0xffffffffffffffff) &&
       ((local_2040 = local_2040 - lVar2, local_2040 == 0 || (param_4 == 0xfffffffffffffffe))))
    goto LAB_001047dd;
  } while( true );
}




// Function: xlseek @ 0x4804

__off_t xlseek(int param_1,__off_t param_2,int param_3,undefined8 param_4)

{
  __off_t _Var1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  long in_FS_OFFSET;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  _Var1 = lseek(param_1,param_2,param_3);
  if (-1 < _Var1) {
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return _Var1;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar2 = FUN_0010cade(param_2,local_38);
  if (param_3 == 2) {
    uVar3 = FUN_0010ea5c(0,3,param_4);
    uVar4 = gettext("%s: cannot seek to end-relative offset %s");
    piVar5 = __errno_location();
    error(0,*piVar5,uVar4,uVar3,uVar2);
  }
  else {
    if (2 < param_3) {
LAB_001049a0:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if (param_3 == 0) {
      uVar3 = FUN_0010ea5c(0,3,param_4);
      uVar4 = gettext("%s: cannot seek to offset %s");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar4,uVar3,uVar2);
    }
    else {
      if (param_3 != 1) goto LAB_001049a0;
      uVar3 = FUN_0010ea5c(0,3,param_4);
      uVar4 = gettext("%s: cannot seek to relative offset %s");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar4,uVar3,uVar2);
    }
  }
                    /* WARNING: Subroutine does not return */
  exit(1);
}




// Function: file_lines @ 0x49bd

undefined8
file_lines(undefined8 param_1,undefined4 param_2,long param_3,long param_4,long param_5,
            long *param_6)

{
  undefined8 uVar1;
  undefined8 uVar2;
  int *piVar3;
  long lVar4;
  long in_FS_OFFSET;
  bool bVar5;
  long local_2060;
  size_t local_2048;
  long local_2040;
  size_t local_2038;
  undefined8 local_2030;
  undefined1 local_2028 [8200];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_3 == 0) {
    uVar1 = 1;
  }
  else {
    local_2048 = (param_5 - param_4) % 0x2000;
    if (local_2048 == 0) {
      local_2048 = 0x2000;
    }
    local_2040 = param_5 - local_2048;
    FUN_00104804(param_2,local_2040,0,param_1);
    local_2048 = FUN_0010ed34(param_2,local_2028,local_2048);
    if (local_2048 == 0xffffffffffffffff) {
      uVar1 = FUN_0010e8ca(4,param_1);
      uVar2 = gettext("error reading %s");
      piVar3 = __errno_location();
      error(0,*piVar3,uVar2,uVar1);
      uVar1 = 0;
    }
    else {
      *param_6 = local_2040 + local_2048;
      local_2060 = param_3;
      if ((local_2048 != 0) && (local_2028[local_2048 - 1] != DAT_00118132)) {
        local_2060 = param_3 + -1;
      }
      do {
        local_2038 = local_2048;
        while ((local_2038 != 0 &&
               (local_2030 = memrchr(local_2028,(int)DAT_00118132,local_2038),
               local_2030 != (void *)0x0))) {
          local_2038 = (long)local_2030 - (long)local_2028;
          bVar5 = local_2060 == 0;
          local_2060 = local_2060 + -1;
          if (bVar5) {
            FUN_001045a9((long)local_2030 + 1,(local_2048 - local_2038) + -1);
            lVar4 = FUN_00104649(0,param_1,param_2,param_5 - (local_2048 + local_2040));
            *param_6 = lVar4 + *param_6;
            uVar1 = 1;
            goto LAB_00104dd7;
          }
        }
        if (local_2040 == param_4) {
          FUN_00104804(param_2,param_4,0,param_1);
          lVar4 = FUN_00104649(0,param_1,param_2,param_5);
          *param_6 = param_4 + lVar4;
          uVar1 = 1;
          goto LAB_00104dd7;
        }
        local_2040 = local_2040 + -0x2000;
        FUN_00104804(param_2,local_2040,0,param_1);
        local_2048 = FUN_0010ed34(param_2,local_2028,0x2000);
        if (local_2048 == 0xffffffffffffffff) {
          uVar1 = FUN_0010e8ca(4,param_1);
          uVar2 = gettext("error reading %s");
          piVar3 = __errno_location();
          error(0,*piVar3,uVar2,uVar1);
          uVar1 = 0;
          goto LAB_00104dd7;
        }
        *param_6 = local_2040 + local_2048;
      } while (local_2048 != 0);
      uVar1 = 1;
    }
  }
LAB_00104dd7:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pipe_lines @ 0x4df7

undefined1 pipe_lines(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  long *plVar1;
  void *pvVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined1 local_69;
  void *local_68;
  void *local_60;
  void *local_58;
  ulong local_50;
  void *local_48;
  void *local_40;
  long local_38;
  
  local_50 = 0;
  local_69 = 1;
  local_68 = (void *)FUN_0010f96c(0x2018);
  *(undefined8 *)((long)local_68 + 0x2008) = 0;
  *(undefined8 *)((long)local_68 + 0x2000) = *(undefined8 *)((long)local_68 + 0x2008);
  *(undefined8 *)((long)local_68 + 0x2010) = 0;
  local_58 = (void *)FUN_0010f96c(0x2018);
  local_60 = local_68;
  while ((lVar3 = FUN_0010ed34(param_2,local_58,0x2000), lVar3 != 0 && (lVar3 != -1))) {
    *(long *)((long)local_58 + 0x2000) = lVar3;
    *param_4 = *param_4 + lVar3;
    *(undefined8 *)((long)local_58 + 0x2008) = 0;
    *(undefined8 *)((long)local_58 + 0x2010) = 0;
    local_48 = local_58;
    while (local_48 = memchr(local_48,(int)DAT_00118132,(long)local_58 + (lVar3 - (long)local_48)),
          local_48 != (void *)0x0) {
      local_48 = (void *)((long)local_48 + 1);
      *(long *)((long)local_58 + 0x2008) = *(long *)((long)local_58 + 0x2008) + 1;
    }
    local_50 = local_50 + *(long *)((long)local_58 + 0x2008);
    if ((ulong)(*(long *)((long)local_60 + 0x2000) + *(long *)((long)local_58 + 0x2000)) < 0x2000) {
      memcpy((void *)((long)local_60 + *(long *)((long)local_60 + 0x2000)),local_58,
             *(size_t *)((long)local_58 + 0x2000));
      *(long *)((long)local_60 + 0x2000) =
           *(long *)((long)local_60 + 0x2000) + *(long *)((long)local_58 + 0x2000);
      *(long *)((long)local_60 + 0x2008) =
           *(long *)((long)local_60 + 0x2008) + *(long *)((long)local_58 + 0x2008);
    }
    else {
      *(void **)((long)local_60 + 0x2010) = local_58;
      local_60 = *(void **)((long)local_60 + 0x2010);
      if (param_3 < local_50 - *(long *)((long)local_68 + 0x2008)) {
        local_58 = local_68;
        local_50 = local_50 - *(long *)((long)local_68 + 0x2008);
        local_68 = *(void **)((long)local_68 + 0x2010);
      }
      else {
        local_58 = (void *)FUN_0010f96c(0x2018);
      }
    }
  }
  free(local_58);
  if (lVar3 == -1) {
    uVar4 = FUN_0010e8ca(4,param_1);
    uVar5 = gettext("error reading %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    local_69 = 0;
  }
  else if ((*(long *)((long)local_60 + 0x2000) != 0) && (param_3 != 0)) {
    if (*(char *)((long)local_60 + *(long *)((long)local_60 + 0x2000) + -1) != DAT_00118132) {
      *(long *)((long)local_60 + 0x2008) = *(long *)((long)local_60 + 0x2008) + 1;
      local_50 = local_50 + 1;
    }
    local_58 = local_68;
    for (; param_3 < local_50 - *(long *)((long)local_58 + 0x2008); local_50 = local_50 - *plVar1) {
      plVar1 = (long *)((long)local_58 + 0x2008);
      local_58 = *(void **)((long)local_58 + 0x2010);
    }
    local_40 = local_58;
    lVar3 = *(long *)((long)local_58 + 0x2000);
    if (param_3 < local_50) {
      for (local_38 = local_50 - param_3; local_38 != 0; local_38 = local_38 + -1) {
        local_40 = rawmemchr(local_40,(int)DAT_00118132);
        local_40 = (void *)((long)local_40 + 1);
      }
    }
    FUN_001045a9(local_40,(long)local_58 + (lVar3 - (long)local_40));
    for (local_58 = *(void **)((long)local_58 + 0x2010); local_58 != (void *)0x0;
        local_58 = *(void **)((long)local_58 + 0x2010)) {
      FUN_001045a9(local_58,*(undefined8 *)((long)local_58 + 0x2000));
    }
  }
  while (local_68 != (void *)0x0) {
    pvVar2 = *(void **)((long)local_68 + 0x2010);
    free(local_68);
    local_68 = pvVar2;
  }
  return local_69;
}




// Function: pipe_bytes @ 0x5286

undefined1 pipe_bytes(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  long *plVar1;
  void *pvVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined1 local_49;
  void *local_48;
  void *local_40;
  void *local_38;
  long local_30;
  ulong local_28;
  
  local_28 = 0;
  local_49 = 1;
  local_48 = (void *)FUN_0010f96c(0x2010);
  *(undefined8 *)((long)local_48 + 0x2000) = 0;
  *(undefined8 *)((long)local_48 + 0x2008) = 0;
  local_38 = (void *)FUN_0010f96c(0x2010);
  local_40 = local_48;
  while ((lVar3 = FUN_0010ed34(param_2,local_38,0x2000), lVar3 != 0 && (lVar3 != -1))) {
    *param_4 = *param_4 + lVar3;
    *(long *)((long)local_38 + 0x2000) = lVar3;
    *(undefined8 *)((long)local_38 + 0x2008) = 0;
    local_28 = local_28 + *(long *)((long)local_38 + 0x2000);
    if ((ulong)(*(long *)((long)local_40 + 0x2000) + *(long *)((long)local_38 + 0x2000)) < 0x2000) {
      memcpy((void *)((long)local_40 + *(long *)((long)local_40 + 0x2000)),local_38,
             *(size_t *)((long)local_38 + 0x2000));
      *(long *)((long)local_40 + 0x2000) =
           *(long *)((long)local_40 + 0x2000) + *(long *)((long)local_38 + 0x2000);
    }
    else {
      *(void **)((long)local_40 + 0x2008) = local_38;
      local_40 = *(void **)((long)local_40 + 0x2008);
      if (param_3 < local_28 - *(long *)((long)local_48 + 0x2000)) {
        local_38 = local_48;
        local_28 = local_28 - *(long *)((long)local_48 + 0x2000);
        local_48 = *(void **)((long)local_48 + 0x2008);
      }
      else {
        local_38 = (void *)FUN_0010f96c(0x2010);
      }
    }
  }
  free(local_38);
  if (lVar3 == -1) {
    uVar4 = FUN_0010e8ca(4,param_1);
    uVar5 = gettext("error reading %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    local_49 = 0;
  }
  else {
    local_38 = local_48;
    for (; param_3 < local_28 - *(long *)((long)local_38 + 0x2000); local_28 = local_28 - *plVar1) {
      plVar1 = (long *)((long)local_38 + 0x2000);
      local_38 = *(void **)((long)local_38 + 0x2008);
    }
    if (param_3 < local_28) {
      local_30 = local_28 - param_3;
    }
    else {
      local_30 = 0;
    }
    FUN_001045a9(local_30 + (long)local_38,*(long *)((long)local_38 + 0x2000) - local_30);
    for (local_38 = *(void **)((long)local_38 + 0x2008); local_38 != (void *)0x0;
        local_38 = *(void **)((long)local_38 + 0x2008)) {
      FUN_001045a9(local_38,*(undefined8 *)((long)local_38 + 0x2000));
    }
  }
  while (local_48 != (void *)0x0) {
    pvVar2 = *(void **)((long)local_48 + 0x2008);
    free(local_48);
    local_48 = pvVar2;
  }
  return local_49;
}




// Function: start_bytes @ 0x55ae

undefined8 start_bytes(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  ulong uVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  ulong local_2050;
  undefined1 local_2028 [8200];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2050 = param_3;
  do {
    if (local_2050 == 0) {
LAB_0010571c:
      uVar4 = 0;
LAB_00105721:
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar4;
    }
    uVar1 = FUN_0010ed34(param_2,local_2028,0x2000);
    if (uVar1 == 0) {
      uVar4 = 0xffffffff;
      goto LAB_00105721;
    }
    if (uVar1 == 0xffffffffffffffff) {
      uVar4 = FUN_0010e8ca(4,param_1);
      uVar2 = gettext("error reading %s");
      piVar3 = __errno_location();
      error(0,*piVar3,uVar2,uVar4);
      uVar4 = 1;
      goto LAB_00105721;
    }
    *param_4 = *param_4 + uVar1;
    if (local_2050 < uVar1) {
      FUN_001045a9(local_2028 + local_2050,uVar1 - local_2050);
      goto LAB_0010571c;
    }
    local_2050 = local_2050 - uVar1;
  } while( true );
}




// Function: start_lines @ 0x5741

undefined8 start_lines(undefined8 param_1,undefined4 param_2,long param_3,long *param_4)

{
  undefined8 uVar1;
  long lVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined1 *puVar5;
  void *pvVar6;
  long in_FS_OFFSET;
  long local_2060;
  undefined1 *local_2040;
  undefined1 local_2028 [8200];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2060 = param_3;
  if (param_3 == 0) {
    uVar1 = 0;
  }
  else {
    while (lVar2 = FUN_0010ed34(param_2,local_2028,0x2000), lVar2 != 0) {
      if (lVar2 == -1) {
        uVar1 = FUN_0010e8ca(4,param_1);
        uVar3 = gettext("error reading %s");
        piVar4 = __errno_location();
        error(0,*piVar4,uVar3,uVar1);
        uVar1 = 1;
        goto LAB_00105918;
      }
      puVar5 = local_2028 + lVar2;
      *param_4 = *param_4 + lVar2;
      local_2040 = local_2028;
      while (pvVar6 = memchr(local_2040,(int)DAT_00118132,(long)puVar5 - (long)local_2040),
            pvVar6 != (void *)0x0) {
        local_2040 = (undefined1 *)((long)pvVar6 + 1);
        local_2060 = local_2060 + -1;
        if (local_2060 == 0) {
          if (local_2040 < puVar5) {
            FUN_001045a9(local_2040,(long)puVar5 - (long)local_2040);
          }
          uVar1 = 0;
          goto LAB_00105918;
        }
      }
    }
    uVar1 = 0xffffffff;
  }
LAB_00105918:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fremote @ 0x5938

undefined1 fremote(int param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined1 local_9d;
  statfs local_98;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_9d = 1;
  iVar1 = fstatfs(param_1,&local_98);
  if (iVar1 == 0) {
    iVar1 = FUN_00102da1(local_98.f_type);
    if (iVar1 == 1) {
      local_9d = 0;
    }
    else if ((1 < iVar1) || ((iVar1 != -1 && (iVar1 != 0)))) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("!\"unexpected return value from is_local_fs_type\"","src/tail.c",0x3cc,
                    "fremote");
    }
  }
  else {
    piVar2 = __errno_location();
    if (*piVar2 != 0x26) {
      uVar3 = FUN_0010e8ca(4,param_2);
      uVar4 = gettext("cannot determine location of %s. reverting to polling");
      piVar2 = __errno_location();
      error(0,*piVar2,uVar4,uVar3);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_9d;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: recheck @ 0x5a69

void recheck(undefined8 *param_1,byte param_2)

{
  char cVar1;
  int iVar2;
  bool bVar3;
  bool bVar4;
  char cVar5;
  undefined1 uVar6;
  int iVar7;
  int __fd;
  int iVar8;
  undefined8 uVar9;
  int *piVar10;
  undefined *puVar11;
  uint uVar12;
  undefined8 uVar13;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar3 = true;
  iVar7 = strcmp((char *)*param_1,"-");
  cVar1 = *(char *)((long)param_1 + 0x36);
  iVar2 = *(int *)((long)param_1 + 0x3c);
  if (iVar7 == 0) {
    __fd = 0;
  }
  else {
    if (param_2 == 0) {
      uVar13 = 0x800;
    }
    else {
      uVar13 = 0;
    }
    __fd = FUN_0010af09(*param_1,uVar13);
  }
  cVar5 = FUN_001043ba(param_1);
  if (cVar5 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck");
  }
  if ((DAT_0011812c == '\x01') && (__fd == -1)) {
    uVar6 = 0;
  }
  else {
    uVar6 = 1;
  }
  *(undefined1 *)((long)param_1 + 0x36) = uVar6;
  if (((DAT_0011813a == '\x01') || (iVar8 = lstat((char *)*param_1,&local_b8), iVar8 != 0)) ||
     ((local_b8.st_mode & 0xf000) != 0xa000)) {
    if ((__fd == -1) || (iVar8 = fstat(__fd,&local_b8), iVar8 < 0)) {
      bVar3 = false;
      piVar10 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar10;
      if (*(char *)((long)param_1 + 0x36) == '\x01') {
        piVar10 = __errno_location();
        if (iVar2 != *piVar10) {
          uVar13 = FUN_001043ee(param_1);
          uVar13 = FUN_0010ea5c(0,3,uVar13);
          piVar10 = __errno_location();
          error(0,*piVar10,&DAT_00112e69,uVar13);
        }
      }
      else if (cVar1 != '\0') {
        uVar13 = FUN_001043ee(param_1);
        uVar13 = FUN_0010e8ca(4,uVar13);
        uVar9 = gettext("%s has become inaccessible");
        error(0,*(undefined4 *)((long)param_1 + 0x3c),uVar9,uVar13);
      }
    }
    else if ((((local_b8.st_mode & 0xf000) == 0x8000) ||
             (((local_b8.st_mode & 0xf000) == 0x1000 || ((local_b8.st_mode & 0xf000) == 0xc000))))
            || ((local_b8.st_mode & 0xf000) == 0x2000)) {
      uVar13 = FUN_001043ee(param_1);
      uVar6 = FUN_00105938(__fd,uVar13);
      *(undefined1 *)((long)param_1 + 0x35) = uVar6;
      if ((*(char *)((long)param_1 + 0x35) == '\0') || (DAT_0011813a == '\x01')) {
        *(undefined4 *)((long)param_1 + 0x3c) = 0;
      }
      else {
        bVar3 = false;
        *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
        uVar13 = FUN_001043ee(param_1);
        uVar13 = FUN_0010e8ca(4,uVar13);
        uVar9 = gettext("%s has been replaced with an untailable remote file");
        error(0,0,uVar9,uVar13);
        *(undefined1 *)((long)param_1 + 0x34) = 1;
        *(undefined1 *)((long)param_1 + 0x35) = 1;
      }
    }
    else {
      bVar3 = false;
      *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
      *(undefined1 *)((long)param_1 + 0x36) = 0;
      if ((DAT_0011812c == '\x01') && (DAT_00118010 == 1)) {
        uVar6 = 0;
      }
      else {
        uVar6 = 1;
      }
      *(undefined1 *)((long)param_1 + 0x34) = uVar6;
      if ((cVar1 != '\0') || (iVar2 != *(int *)((long)param_1 + 0x3c))) {
        if (*(char *)((long)param_1 + 0x34) == '\0') {
          puVar11 = &DAT_001121c3;
        }
        else {
          puVar11 = (undefined *)gettext("; giving up on this name");
        }
        uVar13 = FUN_001043ee(param_1);
        uVar13 = FUN_0010e8ca(4,uVar13);
        uVar9 = gettext("%s has been replaced with an untailable file%s");
        error(0,0,uVar9,uVar13,puVar11);
      }
    }
  }
  else {
    bVar3 = false;
    *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
    *(undefined1 *)((long)param_1 + 0x34) = 1;
    uVar13 = FUN_001043ee(param_1);
    uVar13 = FUN_0010e8ca(4,uVar13);
    uVar9 = gettext("%s has been replaced with an untailable symbolic link");
    error(0,0,uVar9,uVar13);
  }
  bVar4 = false;
  if (bVar3) {
    if ((iVar2 == 0) || (iVar2 == 2)) {
      if (*(int *)(param_1 + 7) == -1) {
        bVar4 = true;
        uVar13 = FUN_001043ee(param_1);
        uVar13 = FUN_0010e8ca(4,uVar13);
        uVar9 = gettext("%s has appeared;  following new file");
        error(0,0,uVar9,uVar13);
      }
      else if ((param_1[5] == local_b8.st_ino) && (param_1[4] == local_b8.st_dev)) {
        uVar13 = FUN_001043ee(param_1);
        FUN_001044d5(__fd,uVar13);
      }
      else {
        bVar4 = true;
        uVar13 = FUN_001043ee(param_1);
        uVar13 = FUN_0010e8ca(4,uVar13);
        uVar9 = gettext("%s has been replaced;  following new file");
        error(0,0,uVar9,uVar13);
        uVar13 = FUN_001043ee(param_1);
        FUN_001044d5(*(undefined4 *)(param_1 + 7),uVar13);
      }
    }
    else {
      bVar4 = true;
      if (*(int *)(param_1 + 7) != -1) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck");
      }
      uVar13 = FUN_001043ee(param_1);
      uVar13 = FUN_0010e8ca(4,uVar13);
      uVar9 = gettext("%s has become accessible");
      error(0,0,uVar9,uVar13);
    }
  }
  else {
    uVar13 = FUN_001043ee(param_1);
    FUN_001044d5(__fd,uVar13);
    uVar13 = FUN_001043ee(param_1);
    FUN_001044d5(*(undefined4 *)(param_1 + 7),uVar13);
    *(undefined4 *)(param_1 + 7) = 0xffffffff;
  }
  if (bVar4) {
    if (iVar7 == 0) {
      uVar12 = 0xffffffff;
    }
    else {
      uVar12 = (uint)param_2;
    }
    FUN_00104435(param_1,__fd,0,&local_b8,uVar12);
    uVar13 = FUN_001043ee(param_1);
    FUN_00104804(__fd,0,0,uVar13);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: any_live_files @ 0x6245

undefined8 any_live_files(long param_1,ulong param_2)

{
  undefined8 uVar1;
  ulong local_10;
  
  if ((DAT_0011812c == '\0') || (DAT_00118010 != 1)) {
    for (local_10 = 0; local_10 < param_2; local_10 = local_10 + 1) {
      if (-1 < *(int *)(param_1 + local_10 * 0x60 + 0x38)) {
        return 1;
      }
      if ((*(char *)(param_1 + local_10 * 0x60 + 0x34) != '\x01') && (DAT_0011812c != '\0')) {
        return 1;
      }
    }
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: tail_forever @ 0x62f3

void tail_forever(undefined8 param_1,long param_2,ulong param_3)

{
  uint uVar1;
  ulong uVar2;
  bool bVar3;
  byte bVar4;
  char cVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  long lVar12;
  undefined8 uVar13;
  int *piVar14;
  long in_FS_OFFSET;
  bool bVar15;
  undefined1 auVar16 [16];
  ulong local_e0;
  ulong local_d8;
  long local_d0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((((DAT_00118134 == 0) && (DAT_00118010 == 2)) && (param_3 == 1)) &&
     ((*(int *)(param_2 + 0x38) != -1 && ((*(uint *)(param_2 + 0x30) & 0xf000) != 0x8000)))) {
    bVar4 = 1;
  }
  else {
    bVar4 = 0;
  }
  bVar3 = false;
  local_e0 = param_3 - 1;
  do {
    do {
      bVar15 = false;
      for (local_d8 = 0; local_d8 < param_3; local_d8 = local_d8 + 1) {
        if (*(char *)(param_2 + local_d8 * 0x60 + 0x34) == '\0') {
          if (*(int *)(param_2 + local_d8 * 0x60 + 0x38) < 0) {
            FUN_00105a69(param_2 + local_d8 * 0x60,bVar4);
          }
          else {
            iVar9 = *(int *)(param_2 + local_d8 * 0x60 + 0x38);
            uVar13 = FUN_001043ee(param_2 + local_d8 * 0x60);
            uVar1 = *(uint *)(param_2 + local_d8 * 0x60 + 0x30);
            if (*(uint *)(param_2 + local_d8 * 0x60 + 0x40) != (uint)bVar4) {
              uVar6 = FUN_0010aa63(iVar9,3);
              if (bVar4 == 0) {
                uVar7 = 0x800;
              }
              else {
                uVar7 = 0;
              }
              if (((int)uVar6 < 0) ||
                 (((uVar7 | uVar6) != uVar6 &&
                  (iVar8 = FUN_0010aa63(iVar9,4,uVar7 | uVar6), iVar8 == -1)))) {
                if (((*(uint *)(param_2 + local_d8 * 0x60 + 0x30) & 0xf000) != 0x8000) ||
                   (piVar14 = __errno_location(), *piVar14 != 1)) {
                  uVar10 = FUN_0010ea5c(0,3,uVar13);
                  uVar11 = gettext("%s: cannot change nonblocking mode");
                  piVar14 = __errno_location();
                  error(1,*piVar14,uVar11,uVar10);
                }
              }
              else {
                *(uint *)(local_d8 * 0x60 + param_2 + 0x40) = (uint)bVar4;
              }
            }
            if (*(int *)(param_2 + local_d8 * 0x60 + 0x40) == 0) {
              iVar8 = fstat(iVar9,&local_b8);
              if (iVar8 != 0) {
                *(undefined4 *)(param_2 + local_d8 * 0x60 + 0x38) = 0xffffffff;
                piVar14 = __errno_location();
                *(int *)(local_d8 * 0x60 + param_2 + 0x3c) = *piVar14;
                uVar13 = FUN_0010ea5c(0,3,uVar13);
                piVar14 = __errno_location();
                error(0,*piVar14,&DAT_00112e69,uVar13);
                close(iVar9);
                goto LAB_00106bb6;
              }
              if ((*(uint *)(param_2 + local_d8 * 0x60 + 0x30) == local_b8.st_mode) &&
                 (((local_b8.st_mode & 0xf000) != 0x8000 ||
                  (*(long *)(param_2 + local_d8 * 0x60 + 8) == local_b8.st_size)))) {
                auVar16 = FUN_0010ee31(&local_b8);
                lVar12 = param_2 + local_d8 * 0x60;
                iVar8 = FUN_0010eeaf(*(undefined8 *)(lVar12 + 0x10),*(undefined8 *)(lVar12 + 0x18),
                                     auVar16._0_8_,auVar16._8_8_);
                if (iVar8 == 0) {
                  lVar12 = param_2 + local_d8 * 0x60;
                  uVar2 = *(ulong *)(lVar12 + 0x58);
                  *(ulong *)(lVar12 + 0x58) = uVar2 + 1;
                  if ((DAT_00118018 <= uVar2) && (DAT_00118010 == 1)) {
                    FUN_00105a69(param_2 + local_d8 * 0x60,
                                 *(int *)(param_2 + local_d8 * 0x60 + 0x40) != 0);
                    *(undefined8 *)(param_2 + local_d8 * 0x60 + 0x58) = 0;
                  }
                  goto LAB_00106bb6;
                }
              }
              auVar16 = FUN_0010ee31(&local_b8);
              *(undefined1 (*) [16])(local_d8 * 0x60 + param_2 + 0x10) = auVar16;
              *(__mode_t *)(local_d8 * 0x60 + param_2 + 0x30) = local_b8.st_mode;
              *(undefined8 *)(param_2 + local_d8 * 0x60 + 0x58) = 0;
              if (((uVar1 & 0xf000) == 0x8000) &&
                 (local_b8.st_size < *(long *)(param_2 + local_d8 * 0x60 + 8))) {
                uVar10 = FUN_0010ea5c(0,3,uVar13);
                uVar11 = gettext("%s: file truncated");
                error(0,0,uVar11,uVar10);
                FUN_00104804(iVar9,0,0,uVar13);
                *(undefined8 *)(param_2 + local_d8 * 0x60 + 8) = 0;
              }
              if (local_d8 != local_e0) {
                if (DAT_00118131 != '\0') {
                  FUN_00104559(uVar13);
                }
                local_e0 = local_d8;
              }
            }
            if (*(int *)(param_2 + local_d8 * 0x60 + 0x40) == 0) {
              if (((uVar1 & 0xf000) == 0x8000) &&
                 (*(char *)(param_2 + local_d8 * 0x60 + 0x35) != '\0')) {
                local_d0 = local_b8.st_size - *(long *)(param_2 + local_d8 * 0x60 + 8);
              }
              else {
                local_d0 = -1;
              }
            }
            else {
              local_d0 = -2;
            }
            lVar12 = FUN_00104649(0,uVar13,iVar9,local_d0);
            bVar15 = lVar12 != 0 || bVar15;
            *(long *)(param_2 + local_d8 * 0x60 + 8) =
                 *(long *)(param_2 + local_d8 * 0x60 + 8) + lVar12;
          }
        }
LAB_00106bb6:
      }
      cVar5 = FUN_00106245(param_2,param_3);
      if (cVar5 != '\x01') {
        uVar13 = gettext("no files remaining");
        error(0,0,uVar13);
        goto code_r0x00106d37;
      }
      if (((!bVar15) || (bVar4 != 0)) && (iVar9 = fflush_unlocked(stdout), iVar9 != 0)) {
        uVar13 = gettext("write error");
        piVar14 = __errno_location();
        error(1,*piVar14,uVar13);
      }
      FUN_00104338();
    } while (bVar15);
    if (bVar3) goto code_r0x00106d37;
    if (((DAT_00118134 == 0) || (iVar9 = kill(DAT_00118134,0), iVar9 == 0)) ||
       (piVar14 = __errno_location(), *piVar14 == 1)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  } while ((bVar3) || (iVar9 = FUN_001101af(param_1), iVar9 == 0));
  uVar13 = gettext("cannot read realtime clock");
  piVar14 = __errno_location();
  error(1,*piVar14,uVar13);
code_r0x00106d37:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: any_remote_file @ 0x6d57

undefined8 any_remote_file(long param_1,ulong param_2)

{
  ulong local_10;
  
  local_10 = 0;
  while( true ) {
    if (param_2 <= local_10) {
      return 0;
    }
    if ((-1 < *(int *)(param_1 + local_10 * 0x60 + 0x38)) &&
       (*(char *)(param_1 + local_10 * 0x60 + 0x35) != '\0')) break;
    local_10 = local_10 + 1;
  }
  return 1;
}




// Function: any_non_remote_file @ 0x6dd3

undefined8 any_non_remote_file(long param_1,ulong param_2)

{
  ulong local_10;
  
  local_10 = 0;
  while( true ) {
    if (param_2 <= local_10) {
      return 0;
    }
    if ((-1 < *(int *)(param_1 + local_10 * 0x60 + 0x38)) &&
       (*(char *)(param_1 + local_10 * 0x60 + 0x35) != '\x01')) break;
    local_10 = local_10 + 1;
  }
  return 1;
}




// Function: any_symlinks @ 0x6e52

undefined8 any_symlinks(long param_1,ulong param_2)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  ulong local_b0;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_b0 = 0;
  do {
    if (param_2 <= local_b0) {
      uVar2 = 0;
LAB_00106efb:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar2;
    }
    iVar1 = lstat(*(char **)(param_1 + local_b0 * 0x60),&local_a8);
    if ((iVar1 == 0) && ((local_a8.st_mode & 0xf000) == 0xa000)) {
      uVar2 = 1;
      goto LAB_00106efb;
    }
    local_b0 = local_b0 + 1;
  } while( true );
}




// Function: any_non_regular_fifo @ 0x6f11

undefined8 any_non_regular_fifo(long param_1,ulong param_2)

{
  ulong local_10;
  
  local_10 = 0;
  while( true ) {
    if (param_2 <= local_10) {
      return 0;
    }
    if (((-1 < *(int *)(param_1 + local_10 * 0x60 + 0x38)) &&
        ((*(uint *)(param_1 + local_10 * 0x60 + 0x30) & 0xf000) != 0x8000)) &&
       ((*(uint *)(param_1 + local_10 * 0x60 + 0x30) & 0xf000) != 0x1000)) break;
    local_10 = local_10 + 1;
  }
  return 1;
}




// Function: tailable_stdin @ 0x6fc5

undefined8 tailable_stdin(long param_1,ulong param_2)

{
  int iVar1;
  ulong local_10;
  
  local_10 = 0;
  while( true ) {
    if (param_2 <= local_10) {
      return 0;
    }
    if ((*(char *)(param_1 + local_10 * 0x60 + 0x34) != '\x01') &&
       (iVar1 = strcmp(*(char **)(param_1 + local_10 * 0x60),"-"), iVar1 == 0)) break;
    local_10 = local_10 + 1;
  }
  return 1;
}




// Function: wd_hasher @ 0x705a

ulong wd_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 0x44) % param_2;
}




// Function: wd_comparator @ 0x7089

undefined4 wd_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x44) >> 8),
                  *(int *)(param_1 + 0x44) == *(int *)(param_2 + 0x44));
}




// Function: check_fspec @ 0x70be

void check_fspec(long param_1,long *param_2)

{
  undefined1 uVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined1 auVar8 [16];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(int *)(param_1 + 0x38) != -1) {
    uVar3 = FUN_001043ee(param_1);
    iVar2 = fstat(*(int *)(param_1 + 0x38),&local_b8);
    if (iVar2 == 0) {
      if (((*(uint *)(param_1 + 0x30) & 0xf000) == 0x8000) &&
         (local_b8.st_size < *(long *)(param_1 + 8))) {
        uVar5 = FUN_0010ea5c(0,3,uVar3);
        uVar6 = gettext("%s: file truncated");
        error(0,0,uVar6,uVar5);
        FUN_00104804(*(undefined4 *)(param_1 + 0x38),0,0,uVar3);
        *(undefined8 *)(param_1 + 8) = 0;
      }
      else if (((*(uint *)(param_1 + 0x30) & 0xf000) == 0x8000) &&
              (local_b8.st_size == *(long *)(param_1 + 8))) {
        auVar8 = FUN_0010ee31(&local_b8);
        iVar2 = FUN_0010eeaf(*(undefined8 *)(param_1 + 0x10),*(undefined8 *)(param_1 + 0x18),
                             auVar8._0_8_,auVar8._8_8_);
        if (iVar2 == 0) goto LAB_0010735f;
      }
      if ((DAT_00118131 == '\0') || (param_1 == *param_2)) {
        uVar1 = 0;
      }
      else {
        uVar1 = 1;
      }
      lVar7 = FUN_00104649(uVar1,uVar3,*(undefined4 *)(param_1 + 0x38),0xffffffffffffffff);
      *(long *)(param_1 + 8) = lVar7 + *(long *)(param_1 + 8);
      if (lVar7 != 0) {
        *param_2 = param_1;
        iVar2 = fflush_unlocked(stdout);
        if (iVar2 != 0) {
          uVar3 = gettext("write error");
          piVar4 = __errno_location();
          error(1,*piVar4,uVar3);
        }
      }
    }
    else {
      piVar4 = __errno_location();
      *(int *)(param_1 + 0x3c) = *piVar4;
      FUN_001044d5(*(undefined4 *)(param_1 + 0x38),uVar3);
      *(undefined4 *)(param_1 + 0x38) = 0xffffffff;
    }
  }
LAB_0010735f:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: tail_forever_inotify @ 0x7379

void tail_forever_inotify(double param_1,int param_2,long param_3,ulong param_4,long *param_5)

{
  undefined1 uVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  int iVar6;
  long lVar7;
  undefined8 uVar8;
  int *piVar9;
  undefined8 uVar10;
  char *__name;
  long in_FS_OFFSET;
  bool bVar11;
  int local_1a4;
  uint32_t local_1a0;
  int local_19c;
  int local_198;
  long local_190;
  size_t local_188;
  long local_180;
  ulong local_178;
  ulong local_170;
  ulong local_168;
  long local_160;
  ulong local_158;
  long local_150;
  size_t local_148;
  long local_140;
  double local_138;
  int *local_130;
  int *local_128;
  long local_120;
  undefined1 local_118 [68];
  int local_d4;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_1a4 = 3;
  bVar4 = false;
  bVar3 = false;
  bVar2 = false;
  bVar11 = false;
  bVar5 = false;
  local_188 = 0;
  local_178 = 0;
  local_170 = 0;
  local_150 = FUN_0010b9fe(param_4,0,FUN_0010705a,FUN_00107089,0);
  if (local_150 == 0) {
    FUN_0010ffa4();
  }
  *param_5 = local_150;
  local_1a0 = 2;
  if (DAT_00118010 == 1) {
    local_1a0 = 0xc06;
  }
  for (local_168 = 0; local_168 < param_4; local_168 = local_168 + 1) {
    if (*(char *)(param_3 + local_168 * 0x60 + 0x34) != '\x01') {
      local_148 = strlen(*(char **)(param_3 + local_168 * 0x60));
      if (local_188 < local_148) {
        local_188 = local_148;
      }
      *(undefined4 *)(param_3 + local_168 * 0x60 + 0x44) = 0xffffffff;
      if (DAT_00118010 == 1) {
        local_140 = FUN_0010a943(*(undefined8 *)(param_3 + local_168 * 0x60));
        uVar1 = *(undefined1 *)(local_140 + *(long *)(param_3 + local_168 * 0x60));
        lVar7 = FUN_0010a685(*(undefined8 *)(param_3 + local_168 * 0x60));
        *(long *)(param_3 + local_168 * 0x60 + 0x50) = lVar7 - *(long *)(param_3 + local_168 * 0x60)
        ;
        *(undefined1 *)(local_140 + *(long *)(param_3 + local_168 * 0x60)) = 0;
        if (local_140 == 0) {
          __name = ".";
        }
        else {
          __name = *(char **)(param_3 + local_168 * 0x60);
        }
        lVar7 = local_168 * 0x60;
        iVar6 = inotify_add_watch(param_2,__name,0x784);
        *(int *)(lVar7 + param_3 + 0x48) = iVar6;
        *(undefined1 *)(*(long *)(param_3 + local_168 * 0x60) + local_140) = uVar1;
        if (*(int *)(param_3 + local_168 * 0x60 + 0x48) < 0) {
          piVar9 = __errno_location();
          if (*piVar9 == 0x1c) {
            uVar8 = gettext("inotify resources exhausted");
            error(0,0,uVar8);
          }
          else {
            uVar8 = FUN_0010e8ca(4,*(undefined8 *)(param_3 + local_168 * 0x60));
            uVar10 = gettext("cannot watch parent directory of %s");
            piVar9 = __errno_location();
            error(0,*piVar9,uVar10,uVar8);
          }
          bVar2 = true;
          break;
        }
      }
      lVar7 = local_168 * 0x60;
      iVar6 = inotify_add_watch(param_2,*(char **)(param_3 + local_168 * 0x60),local_1a0);
      *(int *)(lVar7 + param_3 + 0x44) = iVar6;
      if (*(int *)(param_3 + local_168 * 0x60 + 0x44) < 0) {
        if (*(int *)(param_3 + local_168 * 0x60 + 0x38) != -1) {
          bVar3 = true;
        }
        piVar9 = __errno_location();
        if ((*piVar9 == 0x1c) || (piVar9 = __errno_location(), *piVar9 == 0xc)) {
          bVar11 = true;
          uVar8 = gettext("inotify resources exhausted");
          error(0,0,uVar8);
          break;
        }
        piVar9 = __errno_location();
        if (*piVar9 != *(int *)(param_3 + local_168 * 0x60 + 0x3c)) {
          uVar8 = FUN_0010e8ca(4,*(undefined8 *)(param_3 + local_168 * 0x60));
          uVar10 = gettext("cannot watch %s");
          piVar9 = __errno_location();
          error(0,*piVar9,uVar10,uVar8);
        }
      }
      else {
        lVar7 = FUN_0010c73f(local_150,local_168 * 0x60 + param_3);
        if (lVar7 == 0) {
          FUN_0010ffa4();
        }
        bVar4 = true;
      }
    }
  }
  if (((bVar11) || (bVar2)) || ((DAT_00118010 == 2 && (bVar3)))) {
LAB_00108500:
    if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if ((DAT_00118010 == 2) && (!bVar4)) {
                    /* WARNING: Subroutine does not return */
    exit(1);
  }
  local_190 = param_3 + param_4 * 0x60 + -0x60;
  for (local_168 = 0; local_168 < param_4; local_168 = local_168 + 1) {
    if (*(char *)(param_3 + local_168 * 0x60 + 0x34) != '\x01') {
      if (DAT_00118010 == 1) {
        FUN_00105a69(param_3 + local_168 * 0x60,0);
      }
      else if (((*(int *)(param_3 + local_168 * 0x60 + 0x38) != -1) &&
               (iVar6 = stat(*(char **)(param_3 + local_168 * 0x60),&local_b8), iVar6 == 0)) &&
              ((*(long *)(param_3 + local_168 * 0x60 + 0x20) !=
                CONCAT26(local_b8.st_dev._6_2_,CONCAT24(local_b8.st_dev._4_2_,(int)local_b8.st_dev))
               || (*(long *)(param_3 + local_168 * 0x60 + 0x28) !=
                   CONCAT26(local_b8.st_ino._6_2_,
                            CONCAT24(local_b8.st_ino._4_2_,(undefined4)local_b8.st_ino)))))) {
        uVar8 = FUN_001043ee(param_3 + local_168 * 0x60);
        uVar8 = FUN_0010e8ca(4,uVar8);
        uVar10 = gettext("%s was replaced");
        piVar9 = __errno_location();
        error(0,*piVar9,uVar10,uVar8);
        goto LAB_00108500;
      }
      FUN_001070be(local_168 * 0x60 + param_3,&local_190);
    }
  }
  local_188 = local_188 + 0x11;
  local_180 = FUN_0010f96c(local_188);
LAB_00107c7d:
  if (((DAT_00118010 == 1) && (DAT_0011812c != '\x01')) &&
     (lVar7 = FUN_0010b071(local_150), lVar7 == 0)) {
    uVar8 = gettext("no files remaining");
    error(1,0,uVar8);
  }
  if (local_170 <= local_178) {
    do {
      local_19c = -1;
      if (DAT_00118134 != 0) {
        if (bVar5) {
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        iVar6 = kill(DAT_00118134,0);
        if ((iVar6 == 0) || (piVar9 = __errno_location(), *piVar9 == 1)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
        if ((bVar5) || (param_1 <= 0.0)) {
          local_19c = 0;
        }
        else if (param_1 < DAT_00113558) {
          local_138 = DAT_00113560 * param_1;
          local_19c = (int)local_138 + (uint)((double)(int)local_138 < local_138);
        }
      }
      local_b8.st_dev._4_2_ = 1;
      local_b8.st_ino._0_4_ = 1;
      local_b8.st_ino._6_2_ = 0;
      local_b8.st_ino._4_2_ = 0;
      local_b8.st_dev._0_4_ = param_2;
      iVar6 = poll((pollfd *)&local_b8,(long)(int)(DAT_0011812f + 1),local_19c);
    } while (iVar6 == 0);
    if (iVar6 < 0) {
      uVar8 = gettext("error waiting for inotify and output events");
      piVar9 = __errno_location();
      error(1,*piVar9,uVar8);
    }
    if (local_b8.st_ino._6_2_ != 0) {
      FUN_0010431c();
    }
    local_170 = FUN_0010ed34(param_2,local_180,local_188);
    local_178 = 0;
    if (((local_170 == 0) ||
        ((local_170 == 0xffffffffffffffff && (piVar9 = __errno_location(), *piVar9 == 0x16)))) &&
       (iVar6 = local_1a4 + -1, bVar11 = local_1a4 != 0, local_1a4 = iVar6, bVar11)) {
      local_170 = 0;
      local_188 = local_188 << 1;
      local_180 = FUN_0010f9d6(local_180,local_188);
      goto LAB_00107c7d;
    }
    if ((local_170 == 0) || (local_170 == 0xffffffffffffffff)) {
      uVar8 = gettext("error reading inotify event");
      piVar9 = __errno_location();
      error(1,*piVar9,uVar8);
    }
  }
  local_130 = (int *)(local_178 + local_180);
  local_178 = local_178 + (uint)local_130[3] + 0x10;
  local_128 = local_130;
  if (((local_130[1] & 0x400U) != 0) && (local_130[3] == 0)) {
    for (local_168 = 0; local_168 < param_4; local_168 = local_168 + 1) {
      if (*local_130 == *(int *)(param_3 + local_168 * 0x60 + 0x48)) {
        uVar8 = gettext("directory containing watched file was removed");
        error(0,0,uVar8);
        goto LAB_00108500;
      }
    }
  }
  if (local_130[3] != 0) goto code_r0x00108067;
  local_d4 = *local_130;
  local_160 = FUN_0010b349(local_150,local_118);
  goto LAB_00108453;
code_r0x00108067:
  local_158 = 0;
  while ((local_158 < param_4 &&
         ((*(int *)(param_3 + local_158 * 0x60 + 0x48) != *local_128 ||
          (iVar6 = strcmp((char *)(local_128 + 4),
                          (char *)(*(long *)(param_3 + local_158 * 0x60) +
                                  *(long *)(param_3 + local_158 * 0x60 + 0x50))), iVar6 != 0))))) {
    local_158 = local_158 + 1;
  }
  if (local_158 != param_4) {
    local_160 = param_3 + local_158 * 0x60;
    local_198 = -1;
    bVar11 = (local_128[1] & 0x200U) != 0;
    if ((!bVar11) &&
       (local_198 = inotify_add_watch(param_2,*(char **)(param_3 + local_158 * 0x60),local_1a0),
       local_198 < 0)) {
      piVar9 = __errno_location();
      if ((*piVar9 == 0x1c) || (piVar9 = __errno_location(), *piVar9 == 0xc)) {
        uVar8 = gettext("inotify resources exhausted");
        error(0,0,uVar8);
        goto LAB_00108500;
      }
      uVar8 = FUN_0010e8ca(4,*(undefined8 *)(param_3 + local_158 * 0x60));
      uVar10 = gettext("cannot watch %s");
      piVar9 = __errno_location();
      error(0,*piVar9,uVar10,uVar8);
    }
    if ((bVar11) || ((-1 < *(int *)(local_160 + 0x44) && (local_198 == *(int *)(local_160 + 0x44))))
       ) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
    if (bVar11) {
      if (-1 < *(int *)(local_160 + 0x44)) {
        inotify_rm_watch(param_2,*(int *)(local_160 + 0x44));
        FUN_0010c7af(local_150,local_160);
      }
      *(int *)(local_160 + 0x44) = local_198;
      if (local_198 == -1) goto LAB_00107c7d;
      local_120 = FUN_0010c7af(local_150,local_160);
      if ((local_120 != 0) && (local_120 != local_160)) {
        if (DAT_00118010 == 1) {
          FUN_00105a69(local_120,0);
        }
        *(undefined4 *)(local_120 + 0x44) = 0xffffffff;
        uVar8 = FUN_001043ee(local_120);
        FUN_001044d5(*(undefined4 *)(local_120 + 0x38),uVar8);
      }
      lVar7 = FUN_0010c73f(local_150,local_160);
      if (lVar7 == 0) {
        FUN_0010ffa4();
      }
    }
    if (DAT_00118010 == 1) {
      FUN_00105a69(local_160,0);
    }
LAB_00108453:
    if (local_160 != 0) {
      if ((local_128[1] & 0xe04U) == 0) {
        FUN_001070be(local_160,&local_190);
      }
      else {
        if ((local_128[1] & 0x400U) != 0) {
          inotify_rm_watch(param_2,*(int *)(local_160 + 0x44));
          FUN_0010c7af(local_150,local_160);
        }
        FUN_00105a69(local_160,0);
      }
    }
  }
  goto LAB_00107c7d;
}




// Function: tail_bytes @ 0x8520

ulong tail_bytes(undefined8 param_1,int param_2,ulong param_3,long *param_4)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  ulong uVar7;
  long lVar8;
  __off_t _Var9;
  long in_FS_OFFSET;
  ulong local_f0;
  long local_d0;
  long local_c8;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = fstat(param_2,&local_b8);
  if (iVar2 == 0) {
    if (DAT_00118130 == '\0') {
      local_d0 = -1;
      local_c8 = -1;
      if ((DAT_00118139 != '\x01') && (-1 < (long)param_3)) {
        cVar1 = FUN_00102d62(&local_b8);
        if (cVar1 == '\0') {
          local_c8 = lseek(param_2,-param_3,2);
          if (local_c8 != -1) {
            local_d0 = param_3 + local_c8;
          }
        }
        else {
          local_d0 = local_b8.st_size;
        }
      }
      if ((local_b8.st_blksize < 1) || (0x2000000000000000 < (ulong)local_b8.st_blksize)) {
        local_b8.st_blksize = 0x200;
      }
      if (local_d0 <= local_b8.st_blksize) {
        uVar7 = FUN_00105286(param_1,param_2,param_3,param_4);
        goto LAB_001088a7;
      }
      if (local_c8 == -1) {
        local_c8 = FUN_00104804(param_2,0,1,param_1);
      }
      if ((local_c8 < local_d0) && (param_3 < (ulong)(local_d0 - local_c8))) {
        local_c8 = local_d0 - param_3;
        FUN_00104804(param_2,local_c8,0,param_1);
      }
      *param_4 = local_c8;
      local_f0 = param_3;
    }
    else {
      if (((DAT_00118139 == '\x01') || ((long)param_3 < 0)) ||
         ((((local_b8.st_mode & 0xf000) != 0x8000 ||
           (lVar8 = FUN_00104804(param_2,param_3,1,param_1), lVar8 < 0)) &&
          (_Var9 = lseek(param_2,param_3,1), _Var9 == -1)))) {
        uVar3 = FUN_001055ae(param_1,param_2,param_3,param_4);
        if (uVar3 != 0) {
          uVar7 = (ulong)(uVar3 >> 0x1f);
          goto LAB_001088a7;
        }
      }
      else {
        *param_4 = *param_4 + param_3;
      }
      local_f0 = 0xffffffffffffffff;
    }
    lVar8 = FUN_00104649(0,param_1,param_2,local_f0);
    *param_4 = lVar8 + *param_4;
    uVar7 = 1;
  }
  else {
    uVar4 = FUN_0010e8ca(4,param_1);
    uVar5 = gettext("cannot fstat %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    uVar7 = 0;
  }
LAB_001088a7:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: tail_lines @ 0x88c7

ulong tail_lines(undefined8 param_1,int param_2,undefined8 param_3,long *param_4)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  ulong uVar7;
  long lVar8;
  __off_t _Var9;
  long in_FS_OFFSET;
  long local_c8;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = fstat(param_2,&local_b8);
  if (iVar2 != 0) {
    uVar4 = FUN_0010e8ca(4,param_1);
    uVar5 = gettext("cannot fstat %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    uVar7 = 0;
    goto LAB_00108b47;
  }
  if (DAT_00118130 == '\0') {
    local_c8 = -1;
    if ((DAT_00118139 != '\x01') && ((local_b8.st_mode & 0xf000) == 0x8000)) {
      local_c8 = lseek(param_2,0,1);
      if (local_c8 != -1) {
        _Var9 = lseek(param_2,0,2);
        if (local_c8 < _Var9) {
          *param_4 = _Var9;
          if (_Var9 != 0) {
            cVar1 = FUN_001049bd(param_1,param_2,param_3,local_c8,_Var9,param_4);
            if (cVar1 != '\x01') {
              uVar7 = 0;
              goto LAB_00108b47;
            }
          }
          goto LAB_00108b42;
        }
      }
    }
    if (local_c8 != -1) {
      FUN_00104804(param_2,local_c8,0,param_1);
    }
    uVar7 = FUN_00104df7(param_1,param_2,param_3,param_4);
  }
  else {
    uVar3 = FUN_00105741(param_1,param_2,param_3,param_4);
    if (uVar3 != 0) {
      uVar7 = (ulong)(uVar3 >> 0x1f);
      goto LAB_00108b47;
    }
    lVar8 = FUN_00104649(0,param_1,param_2,0xffffffffffffffff);
    *param_4 = lVar8 + *param_4;
LAB_00108b42:
    uVar7 = 1;
  }
LAB_00108b47:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: tail @ 0x8b67

void tail(undefined8 param_1,undefined4 param_2,undefined8 param_3,undefined8 *param_4)

{
  *param_4 = 0;
  if (DAT_0011812d == '\0') {
    FUN_00108520(param_1,param_2,param_3,param_4);
  }
  else {
    FUN_001088c7(param_1,param_2,param_3,param_4);
  }
  return;
}




// Function: tail_file @ 0x8bca

byte tail_file(undefined8 *param_1,undefined8 param_2)

{
  undefined1 uVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined *puVar6;
  undefined4 uVar7;
  long in_FS_OFFSET;
  bool bVar8;
  byte local_c6;
  int local_c4;
  undefined8 local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = strcmp((char *)*param_1,"-");
  bVar8 = iVar2 != 0;
  if (bVar8) {
    local_c4 = FUN_0010af09(*param_1,0);
  }
  else {
    DAT_00118138 = 1;
    local_c4 = 0;
    FUN_0010ffef(0,0);
  }
  if ((DAT_0011812c == '\x01') && (local_c4 == -1)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  *(undefined1 *)((long)param_1 + 0x36) = uVar1;
  if (local_c4 == -1) {
    if (DAT_0011812e != '\0') {
      *(undefined4 *)(param_1 + 7) = 0xffffffff;
      piVar3 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar3;
      *(bool *)((long)param_1 + 0x34) = DAT_0011812c == '\0';
      param_1[5] = 0;
      param_1[4] = 0;
    }
    uVar4 = FUN_001043ee(param_1);
    uVar4 = FUN_0010e8ca(4,uVar4);
    uVar5 = gettext("cannot open %s for reading");
    piVar3 = __errno_location();
    error(0,*piVar3,uVar5,uVar4);
    local_c6 = 0;
  }
  else {
    if (DAT_00118131 != '\0') {
      uVar4 = FUN_001043ee(param_1);
      FUN_00104559(uVar4);
    }
    uVar4 = FUN_001043ee(param_1);
    local_c6 = FUN_00108b67(uVar4,local_c4,param_2,&local_c0);
    if (DAT_0011812e == '\0') {
      if (bVar8) {
        iVar2 = close(local_c4);
        if (iVar2 != 0) {
          uVar4 = FUN_001043ee(param_1);
          uVar4 = FUN_0010e8ca(4,uVar4);
          uVar5 = gettext("error reading %s");
          piVar3 = __errno_location();
          error(0,*piVar3,uVar5,uVar4);
          local_c6 = 0;
        }
      }
    }
    else {
      *(uint *)((long)param_1 + 0x3c) = local_c6 - 1;
      iVar2 = fstat(local_c4,&local_b8);
      if (iVar2 < 0) {
        local_c6 = 0;
        piVar3 = __errno_location();
        *(int *)((long)param_1 + 0x3c) = *piVar3;
        uVar4 = FUN_001043ee(param_1);
        uVar4 = FUN_0010e8ca(4,uVar4);
        uVar5 = gettext("error reading %s");
        piVar3 = __errno_location();
        error(0,*piVar3,uVar5,uVar4);
      }
      else if (((((local_b8.st_mode & 0xf000) != 0x8000) && ((local_b8.st_mode & 0xf000) != 0x1000))
               && ((local_b8.st_mode & 0xf000) != 0xc000)) &&
              ((local_b8.st_mode & 0xf000) != 0x2000)) {
        local_c6 = 0;
        *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
        *(undefined1 *)((long)param_1 + 0x36) = 0;
        *(bool *)((long)param_1 + 0x34) = DAT_0011812c == '\0';
        if (*(char *)((long)param_1 + 0x34) == '\0') {
          puVar6 = &DAT_001121c3;
        }
        else {
          puVar6 = (undefined *)gettext("; giving up on this name");
        }
        uVar4 = FUN_001043ee(param_1);
        uVar4 = FUN_0010ea5c(0,3,uVar4);
        uVar5 = gettext("%s: cannot follow end of this type of file%s");
        error(0,0,uVar5,uVar4,puVar6);
      }
      if (local_c6 == 1) {
        if (bVar8) {
          uVar7 = 1;
        }
        else {
          uVar7 = 0xffffffff;
        }
        FUN_00104435(param_1,local_c4,local_c0,&local_b8,uVar7);
        uVar4 = FUN_001043ee(param_1);
        uVar1 = FUN_00105938(local_c4,uVar4);
        *(undefined1 *)((long)param_1 + 0x35) = uVar1;
      }
      else {
        *(bool *)((long)param_1 + 0x34) = DAT_0011812c == '\0';
        uVar4 = FUN_001043ee(param_1);
        FUN_001044d5(local_c4,uVar4);
        *(undefined4 *)(param_1 + 7) = 0xffffffff;
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_c6;
}




// Function: parse_obsolete_option @ 0x90f5

undefined8 parse_obsolete_option(int param_1,long param_2,long *param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  bool bVar4;
  int iVar5;
  uint uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  int *piVar9;
  undefined1 local_3d;
  undefined1 local_3c;
  int local_38;
  char *local_30;
  
  local_38 = 10;
  local_3c = 1;
  if (((param_1 != 2) &&
      ((param_1 != 3 ||
       ((**(char **)(param_2 + 0x10) == '-' && (*(char *)(*(long *)(param_2 + 0x10) + 1) != '\0'))))
      )) && ((param_1 < 3 ||
             ((4 < param_1 || (iVar5 = strcmp(*(char **)(param_2 + 0x10),"--"), iVar5 != 0)))))) {
    return 0;
  }
  iVar5 = FUN_0010cc1c();
  if ((iVar5 < 0x30db0) || (0x31068 < iVar5)) {
    bVar4 = true;
  }
  else {
    bVar4 = false;
  }
  pcVar1 = *(char **)(param_2 + 8) + 1;
  cVar2 = **(char **)(param_2 + 8);
  local_30 = pcVar1;
  if (cVar2 == '+') {
    if (!bVar4) {
      return 0;
    }
    local_3d = 1;
  }
  else {
    if (cVar2 != '-') {
      return 0;
    }
    if ((iVar5 >= 0x30db0) && (pcVar1[*pcVar1 == 'c'] == '\0')) {
      return 0;
    }
    local_3d = 0;
  }
  for (; pcVar3 = local_30, (int)*local_30 - 0x30U < 10; local_30 = local_30 + 1) {
  }
  cVar2 = *local_30;
  if (cVar2 != 'l') {
    if ('l' < cVar2) goto LAB_0010929b;
    if (cVar2 == 'b') {
      local_38 = 0x1400;
    }
    else if (cVar2 != 'c') goto LAB_0010929b;
    local_3c = 0;
  }
  local_30 = local_30 + 1;
LAB_0010929b:
  cVar2 = *local_30;
  if (cVar2 == 'f') {
    local_30 = local_30 + 1;
  }
  if (*local_30 == '\0') {
    if (pcVar1 == pcVar3) {
      *param_3 = (long)local_38;
    }
    else {
      uVar6 = FUN_00110410(pcVar1,0,10,param_3,&DAT_00113120);
      if ((uVar6 & 0xfffffffd) != 0) {
        uVar7 = FUN_0010ed11(*(undefined8 *)(param_2 + 8));
        uVar8 = gettext("invalid number");
        piVar9 = __errno_location();
        error(1,*piVar9,"%s: %s",uVar8,uVar7);
      }
    }
    DAT_00118130 = local_3d;
    DAT_0011812d = local_3c;
    uVar7 = 1;
    DAT_0011812e = cVar2 == 'f';
  }
  else {
    uVar7 = 0;
  }
  return uVar7;
}




// Function: parse_options @ 0x9381

void parse_options(undefined4 param_1,undefined8 param_2,undefined8 *param_3,undefined4 *param_4,
                 double *param_5)

{
  char cVar1;
  int iVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  double local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    iVar2 = getopt_long(param_1,param_2,"c:n:fFqs:vz0123456789",&PTR_s_bytes_00117920,0);
    if (iVar2 == -1) {
      if (DAT_0011812c != '\0') {
        if (DAT_0011812e == '\x01') {
          if (DAT_00118010 == 2) {
            uVar5 = gettext("warning: --retry only effective for the initial open");
            error(0,0,uVar5);
          }
        }
        else {
          DAT_0011812c = '\0';
          uVar5 = gettext("warning: --retry ignored; --retry is useful only when following");
          error(0,0,uVar5);
        }
      }
      if ((DAT_00118134 == 0) || (DAT_0011812e == '\x01')) {
        if ((DAT_00118134 != 0) &&
           ((iVar2 = kill(DAT_00118134,0), iVar2 != 0 &&
            (piVar6 = __errno_location(), *piVar6 == 0x26)))) {
          uVar5 = gettext("warning: --pid=PID is not supported on this system");
          error(0,0,uVar5);
          DAT_00118134 = 0;
        }
      }
      else {
        uVar5 = gettext("warning: PID ignored; --pid=PID is useful only when following");
        error(0,0,uVar5);
      }
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    if (0x85 < iVar2) goto switchD_00109416_caseD_3a;
    if (iVar2 < 0x30) {
      if (iVar2 == -0x83) {
LAB_001096d6:
        FUN_0010f78c(stdout,&DAT_00112ca5,"GNU coreutils",PTR_DAT_00118030,"Paul Rubin",
                     "David MacKenzie","Ian Lance Taylor","Jim Meyering",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar2 == -0x82) {
        FUN_00104115(0);
        goto LAB_001096d6;
      }
      goto switchD_00109416_caseD_3a;
    }
    switch(iVar2) {
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
      uVar5 = gettext("option used in invalid context -- %c");
      error(1,0,uVar5,iVar2);
    default:
switchD_00109416_caseD_3a:
      FUN_00104115(1);
      break;
    case 0x46:
      DAT_0011812e = '\x01';
      DAT_00118010 = 1;
      DAT_0011812c = '\x01';
      break;
    case 99:
    case 0x6e:
      DAT_0011812d = iVar2 == 0x6e;
      if (*optarg == '+') {
        DAT_00118130 = 1;
      }
      else if (*optarg == '-') {
        optarg = optarg + 1;
      }
      if ((bool)DAT_0011812d) {
        uVar5 = gettext("invalid number of lines");
      }
      else {
        uVar5 = gettext("invalid number of bytes");
      }
      uVar5 = FUN_00110156(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",uVar5,0);
      *param_3 = uVar5;
      break;
    case 0x66:
    case 0x84:
      DAT_0011812e = '\x01';
      if (optarg == (char *)0x0) {
        DAT_00118010 = 2;
      }
      else {
        lVar3 = FUN_0010a546("--follow",optarg,&PTR_s_descriptor_00117900,&DAT_00112210,4,
                             PTR_FUN_00118038,1);
        DAT_00118010 = *(int *)(&DAT_00112210 + lVar3 * 4);
      }
      break;
    case 0x71:
      *param_4 = 2;
      break;
    case 0x73:
      cVar1 = FUN_00110261(optarg,0,&local_28,FUN_0010a745);
      if ((cVar1 != '\x01') || (local_28 < 0.0)) {
        uVar5 = FUN_0010ed11(optarg);
        uVar4 = gettext("invalid number of seconds: %s");
        error(1,0,uVar4,uVar5);
      }
      *param_5 = local_28;
      break;
    case 0x76:
      *param_4 = 1;
      break;
    case 0x7a:
      DAT_00118132 = 0;
      break;
    case 0x80:
      DAT_0011812c = '\x01';
      break;
    case 0x81:
      uVar5 = gettext("invalid maximum number of unchanged stats between opens");
      DAT_00118018 = FUN_00110156(optarg,0,0xffffffffffffffff,&DAT_001121c3,uVar5,0);
      break;
    case 0x82:
      uVar5 = gettext("invalid PID");
      DAT_00118134 = FUN_00110156(optarg,0,0x7fffffff,&DAT_001121c3,uVar5,0);
      break;
    case 0x83:
      DAT_00118139 = 1;
      break;
    case 0x85:
      DAT_0011813a = 1;
    }
  } while( true );
}




// Function: ignore_fifo_and_pipe @ 0x98cb

long ignore_fifo_and_pipe(long param_1,ulong param_2)

{
  bool bVar1;
  int iVar2;
  long local_18;
  ulong local_10;
  
  local_18 = 0;
  for (local_10 = 0; local_10 < param_2; local_10 = local_10 + 1) {
    iVar2 = strcmp(*(char **)(param_1 + local_10 * 0x60),"-");
    if ((((iVar2 == 0) && (*(char *)(param_1 + local_10 * 0x60 + 0x34) != '\x01')) &&
        (-1 < *(int *)(param_1 + local_10 * 0x60 + 0x38))) &&
       ((*(uint *)(param_1 + local_10 * 0x60 + 0x30) & 0xf000) == 0x1000)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    if (bVar1) {
      *(undefined4 *)(param_1 + local_10 * 0x60 + 0x38) = 0xffffffff;
      *(undefined1 *)(param_1 + local_10 * 0x60 + 0x34) = 1;
    }
    else {
      local_18 = local_18 + 1;
    }
  }
  return local_18;
}




// Function: main @ 0x9a14

byte main(int param_1,undefined8 *param_2)

{
  byte bVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  int *piVar8;
  long in_FS_OFFSET;
  bool local_fc;
  int local_f8;
  int local_f4;
  long local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  ulong local_d8;
  undefined **local_d0;
  ulong local_c8;
  long local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f8 = 0;
  local_fc = true;
  local_f0 = 10;
  local_e8 = DAT_00113568;
  FUN_0010ccd0(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00111700(FUN_0010a84b);
  DAT_00118138 = '\0';
  DAT_0011812d = 1;
  DAT_00118131 = 0;
  DAT_00118130 = '\0';
  DAT_0011812e = '\0';
  DAT_00118132 = 10;
  bVar1 = FUN_001090f5(param_1,param_2,&local_f0);
  param_1 = param_1 - (uint)bVar1;
  FUN_00109381(param_1,param_2 + bVar1,&local_f0,&local_f8,&local_e8);
  if ((DAT_00118130 != '\0') && (local_f0 != 0)) {
    local_f0 = local_f0 + -1;
  }
  if (optind < param_1) {
    local_d8 = (ulong)(param_1 - optind);
    local_d0 = (undefined **)(param_2 + bVar1 + optind);
  }
  else {
    local_d8 = 1;
    local_d0 = &PTR_DAT_00118028;
  }
  bVar2 = false;
  for (local_c8 = 0; local_c8 < local_d8; local_c8 = local_c8 + 1) {
    iVar4 = strcmp(local_d0[local_c8],"-");
    if (iVar4 == 0) {
      bVar2 = true;
    }
  }
  if ((bVar2) && (DAT_00118010 == 1)) {
    uVar5 = FUN_0010e8ca(4,&DAT_00112caa);
    uVar6 = gettext("cannot follow %s by name");
    error(1,0,uVar6,uVar5);
  }
  if ((DAT_0011812e != '\0') && (bVar2)) {
    if ((DAT_00118134 == 0) && ((DAT_00118010 == 2 && (local_d8 == 1)))) {
      iVar4 = fstat(0,&local_b8);
      if ((iVar4 != 0) || ((local_b8.st_mode & 0xf000) == 0x8000)) goto LAB_00109d1d;
      bVar2 = true;
    }
    else {
LAB_00109d1d:
      bVar2 = false;
    }
    if (!bVar2) {
      iVar4 = isatty(0);
      if (iVar4 != 0) {
        uVar5 = gettext("warning: following standard input indefinitely is ineffective");
        error(0,0,uVar5);
      }
    }
  }
  if (((local_f0 == 0) && (DAT_0011812e != '\x01')) && (DAT_00118130 != '\x01')) {
    bVar1 = 0;
    goto LAB_0010a17e;
  }
  local_c0 = FUN_0010fae5(local_d8,0x60);
  for (local_c8 = 0; local_c8 < local_d8; local_c8 = local_c8 + 1) {
    *(undefined **)(local_c8 * 0x60 + local_c0) = local_d0[local_c8];
  }
  if ((local_f8 == 1) || ((local_f8 == 0 && (1 < local_d8)))) {
    DAT_00118131 = 1;
  }
  FUN_0010ffef(1,0);
  for (local_c8 = 0; local_c8 < local_d8; local_c8 = local_c8 + 1) {
    bVar1 = FUN_00108bca(local_c0 + local_c8 * 0x60,local_f0);
    local_fc = (bVar1 & local_fc) != 0;
  }
  if (DAT_0011812e != '\0') {
    lVar7 = FUN_001098cb(local_c0,local_d8);
    if (lVar7 != 0) {
      iVar4 = fstat(1,&local_b8);
      if (iVar4 < 0) {
        uVar5 = gettext("standard output");
        piVar8 = __errno_location();
        error(1,*piVar8,uVar5);
      }
      DAT_0011812f = (local_b8.st_mode & 0xf000) == 0x1000;
      if (DAT_0011813a != '\x01') {
        cVar3 = FUN_00106fc5(local_c0,local_d8);
        if (cVar3 == '\0') {
          cVar3 = FUN_00106d57(local_c0,local_d8);
          if (cVar3 == '\0') {
            cVar3 = FUN_00106dd3(local_c0,local_d8);
            if (cVar3 == '\x01') {
              cVar3 = FUN_00106e52(local_c0,local_d8);
              if (cVar3 == '\0') {
                cVar3 = FUN_00106f11(local_c0,local_d8);
                if ((cVar3 == '\0') && ((local_fc == true || (DAT_00118010 != 2))))
                goto LAB_0010a021;
              }
            }
          }
        }
        DAT_0011813a = '\x01';
      }
LAB_0010a021:
      if (DAT_0011813a != '\x01') {
        local_f4 = inotify_init();
        if (-1 < local_f4) {
          iVar4 = fflush_unlocked(stdout);
          if (iVar4 != 0) {
            uVar5 = gettext("write error");
            piVar8 = __errno_location();
            error(1,*piVar8,uVar5);
          }
          FUN_00107379(local_e8,local_f4,local_c0,local_d8,&local_e0);
          FUN_0010bc7e(local_e0);
          close(local_f4);
          piVar8 = __errno_location();
          *piVar8 = 0;
        }
        uVar5 = gettext("inotify cannot be used, reverting to polling");
        piVar8 = __errno_location();
        error(0,*piVar8,uVar5);
      }
      DAT_0011813a = '\x01';
      FUN_001062f3(local_e8,local_c0,local_d8);
    }
  }
  if (DAT_00118138 != '\0') {
    iVar4 = close(0);
    if (iVar4 < 0) {
      piVar8 = __errno_location();
      error(1,*piVar8,&DAT_00112caa);
    }
  }
  bVar1 = local_fc ^ 1;
LAB_0010a17e:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar1;
}




// Function: hash_get_n_buckets @ 0xb045

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0xb05b

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0xb071

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0xb087

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




// Function: hash_table_ok @ 0xb109

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




// Function: hash_print_statistics @ 0xb1a7

void hash_print_statistics(undefined8 param_1,FILE *param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_0010b071(param_1);
  uVar2 = FUN_0010b045(param_1);
  uVar3 = FUN_0010b05b(param_1);
  uVar4 = FUN_0010b087(param_1);
  fprintf(param_2,"# entries:         %lu\n",uVar1);
  fprintf(param_2,"# buckets:         %lu\n",uVar2);
  fprintf(param_2,"# buckets used:    %lu (%.2f%%)\n",(DAT_00113678 * (double)uVar3) / (double)uVar2
          ,uVar3);
  fprintf(param_2,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: safe_hasher @ 0xb2ee

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




// Function: hash_lookup @ 0xb349

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *local_18;
  
  local_18 = (long *)FUN_0010b2ee(param_1,param_2);
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




// Function: hash_get_first @ 0xb3e0

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




// Function: hash_get_next @ 0xb440

long hash_get_next(long param_1,long param_2)

{
  long *local_18;
  long *local_10;
  
  local_18 = (long *)FUN_0010b2ee(param_1,param_2);
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




// Function: hash_get_entries @ 0xb4de

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




// Function: hash_do_for_each @ 0xb57e

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




// Function: hash_string @ 0xb616

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




// Function: is_prime @ 0xb670

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




// Function: next_prime @ 0xb6df

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
    cVar1 = FUN_0010b670(local_10);
    if (cVar1 == '\x01') break;
    local_10 = local_10 + 2;
  }
  return local_10;
}




// Function: hash_reset_tuning @ 0xb72a

void hash_reset_tuning(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_00113680;
  param_1[2] = DAT_00113684;
  param_1[3] = DAT_00113688;
  *(undefined1 *)(param_1 + 4) = 0;
  return;
}




// Function: raw_hasher @ 0xb780

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = FUN_00110957(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0xb7bb

bool raw_comparator(long param_1,long param_2)

{
  return param_1 == param_2;
}




// Function: check_tuning @ 0xb7d8

undefined8 check_tuning(long param_1)

{
  float *pfVar1;
  undefined8 uVar2;
  
  pfVar1 = *(float **)(param_1 + 0x28);
  if (pfVar1 == (float *)&DAT_001135f0) {
    uVar2 = 1;
  }
  else if ((((pfVar1[2] <= DAT_0011368c) || (DAT_00113680 - DAT_0011368c <= pfVar1[2])) ||
           (pfVar1[3] <= DAT_0011368c + DAT_00113680)) ||
          (((*pfVar1 < 0.0 || (pfVar1[1] <= *pfVar1 + DAT_0011368c)) ||
           ((DAT_00113680 < pfVar1[1] || (pfVar1[2] <= *pfVar1 + DAT_0011368c)))))) {
    *(undefined **)(param_1 + 0x28) = &DAT_001135f0;
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: compute_bucket_size @ 0xb8e5

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong compute_bucket_size(ulong param_1,long param_2)

{
  ulong uVar1;
  float fVar2;
  ulong local_20;
  
  local_20 = param_1;
  if (*(char *)(param_2 + 0x10) != '\x01') {
    fVar2 = (float)param_1 / *(float *)(param_2 + 8);
    if (_DAT_00113690 <= fVar2) {
      return 0;
    }
    if (_DAT_00113694 <= fVar2) {
      local_20 = (long)(fVar2 - _DAT_00113694) ^ 0x8000000000000000;
    }
    else {
      local_20 = (ulong)fVar2;
    }
  }
  uVar1 = FUN_0010b6df(local_20);
  if ((long)(uVar1 << 3) < 0 || uVar1 >> 0x3d != 0) {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_initialize @ 0xb9fe

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
    local_30 = FUN_0010b780;
  }
  local_38 = param_4;
  if (param_4 == (code *)0x0) {
    local_38 = FUN_0010b7bb;
  }
  __ptr = malloc(0x50);
  if (__ptr != (long *)0x0) {
    local_28 = param_2;
    if (param_2 == (undefined *)0x0) {
      local_28 = &DAT_001135f0;
    }
    __ptr[5] = (long)local_28;
    cVar1 = FUN_0010b7d8(__ptr);
    if (cVar1 == '\x01') {
      lVar2 = FUN_0010b8e5(param_1,local_28);
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




// Function: hash_clear @ 0xbb73

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




// Function: hash_free @ 0xbc7e

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




// Function: allocate_entry @ 0xbdad

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




// Function: free_entry @ 0xbdfc

void free_entry(long param_1,undefined8 *param_2)

{
  *param_2 = 0;
  param_2[1] = *(undefined8 *)(param_1 + 0x48);
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: hash_find_entry @ 0xbe36

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  long *local_38;
  
  local_38 = (long *)FUN_0010b2ee(param_1,param_2);
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
        FUN_0010bdfc(param_1,plVar1);
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
        FUN_0010bdfc(param_1,lVar2);
        return lVar4;
      }
    }
    lVar4 = 0;
  }
  return lVar4;
}




// Function: transfer_entries @ 0xbfcf

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
        plVar3 = (long *)FUN_0010b2ee(param_1,lVar1);
        plVar2 = (long *)local_30[1];
        if (*plVar3 == 0) {
          *plVar3 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0010bdfc(param_1,local_30);
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
        plVar2 = (long *)FUN_0010b2ee(param_1,lVar1);
        if (*plVar2 == 0) {
          *plVar2 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
        }
        else {
          plVar3 = (long *)FUN_0010bdad(param_1);
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




// Function: hash_rehash @ 0xc1a3

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
  __nmemb = FUN_0010b8e5(param_2,param_1[5]);
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
      cVar1 = FUN_0010bfcf(&local_68,param_1,0);
      if (cVar1 == '\0') {
        param_1[9] = local_20;
        cVar1 = FUN_0010bfcf(param_1,&local_68,1);
        if (cVar1 == '\x01') {
          cVar1 = FUN_0010bfcf(param_1,&local_68,0);
          if (cVar1 == '\x01') {
            free(local_68);
            uVar2 = 0;
            goto LAB_0010c3b8;
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
LAB_0010c3b8:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert_if_absent @ 0xc3ce

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
  local_28 = FUN_0010be36(param_1,param_2,&local_30,0);
  if (local_28 != 0) {
    if (param_3 != (long *)0x0) {
      *param_3 = local_28;
    }
    uVar2 = 0;
    goto LAB_0010c729;
  }
  if (((float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
       (float)*(ulong *)(param_1 + 0x18)) &&
     (FUN_0010b7d8(param_1),
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
    if (_DAT_00113690 <= fVar5) {
      uVar2 = 0xffffffff;
      goto LAB_0010c729;
    }
    if (_DAT_00113694 <= fVar5) {
      uVar3 = (long)(fVar5 - _DAT_00113694) ^ 0x8000000000000000;
    }
    else {
      uVar3 = (ulong)fVar5;
    }
    cVar1 = FUN_0010c1a3(param_1,uVar3);
    if (cVar1 != '\x01') {
      uVar2 = 0xffffffff;
      goto LAB_0010c729;
    }
    lVar4 = FUN_0010be36(param_1,param_2,&local_30,0);
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
    local_18 = (long *)FUN_0010bdad(param_1);
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
LAB_0010c729:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert @ 0xc73f

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010c3ce(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xc7af

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
  local_30 = FUN_0010be36(param_1,param_2,&local_40,1);
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
       (FUN_0010b7d8(param_1), lVar2 = local_30,
       (float)*(ulong *)(param_1 + 0x18) <
       (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) {
      local_28 = *(long *)(param_1 + 0x28);
      if (*(char *)(local_28 + 0x10) == '\0') {
        fVar3 = *(float *)(local_28 + 8) *
                *(float *)(local_28 + 4) * (float)*(ulong *)(param_1 + 0x10);
        if (_DAT_00113694 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_00113694) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      else {
        fVar3 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_28 + 4);
        if (_DAT_00113694 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_00113694) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      cVar1 = FUN_0010c1a3(param_1,local_20);
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




// Function: hash_delete @ 0xcab5

void hash_delete(undefined8 param_1,undefined8 param_2)

{
  FUN_0010c7af(param_1,param_2);
  return;
}



