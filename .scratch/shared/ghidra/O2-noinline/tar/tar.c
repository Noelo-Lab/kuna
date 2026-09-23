// Function: main @ 0xbb10

int main(undefined4 param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  
  FUN_0010cd20();
  FUN_00148760(*param_2);
  setlocale(6,"");
  bindtextdomain("tar","/usr/local/share/locale");
  textdomain("tar");
  DAT_00183e08 = 2;
  DAT_00184d50 = FUN_00110820;
  DAT_00184d58 = 0;
  FUN_0014a590(0,7);
  uVar2 = dcgettext(0,"stdout",5);
  FUN_0013e420(uVar2);
  iVar1 = FUN_0014bd80();
  if (iVar1 == 0) {
    DAT_00184c18 = 10;
    DAT_00184838 = (void *)FUN_0014c5b0(0x50);
    DAT_00184830 = 0;
    signal(0x11,(__sighandler_t)0x0);
    FUN_0012e690(param_1,param_2);
    FUN_00126c90();
    if (DAT_00184a48 != 0) {
      FUN_0010d850();
    }
    switch(DAT_00184ba8) {
    case 0:
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,
                        "You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options"
                        ,5);
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    case 1:
    case 2:
    case 8:
      FUN_00130ef0();
      break;
    case 3:
      FUN_001143d0();
      break;
    case 4:
      FUN_00115dc0();
      break;
    case 5:
      FUN_00110af0();
      FUN_00122c60(FUN_00111730);
      break;
    case 6:
      FUN_00119b30();
      FUN_00122c60(FUN_00119c10);
      FUN_00118640();
      break;
    case 7:
      FUN_00122c60(FUN_00123260);
      break;
    case 9:
      FUN_001232e0();
    }
    FUN_00110910();
    if (DAT_00184a69 != '\0') {
      FUN_0010d150();
    }
    if (DAT_00184c20 != 0) {
      FUN_00114030();
    }
    if (DAT_00184a48 != 0) {
      FUN_0010d960();
    }
    free(DAT_00184838);
    FUN_00131b40();
    FUN_00126cc0();
    if (DAT_00184d58 == 2) {
      uVar2 = dcgettext(0,"Exiting with failure status due to previous errors",5);
      error(0,0,uVar2);
    }
    if (DAT_00183f28 == stdout) {
      FUN_0013e440();
    }
    else {
      iVar1 = ferror_unlocked(stderr);
      if ((iVar1 != 0) || (iVar1 = fclose(stderr), iVar1 != 0)) {
        FUN_0012f7d0(2);
      }
    }
    return DAT_00184d58;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = dcgettext(0,"failed to assert availability of the standard file descriptors",5);
  error(0,0,"%s",uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: bufmap_locate @ 0xbed0

long * bufmap_locate(ulong param_1)

{
  long *plVar1;
  long *plVar2;
  
  plVar1 = DAT_00184148;
  while( true ) {
    plVar2 = plVar1;
    if (plVar2 == (long *)0x0) {
      return (long *)0x0;
    }
    plVar1 = (long *)*plVar2;
    if (plVar1 == (long *)0x0) break;
    if (param_1 < (ulong)(plVar1[1] << 9)) {
      return plVar2;
    }
  }
  return plVar2;
}




// Function: find_zip_program @ 0xbf10

undefined * find_zip_program(int param_1,uint *param_2)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  int *piVar5;
  
  uVar3 = *param_2;
  uVar4 = (ulong)(int)uVar3;
  piVar5 = (int *)(&DAT_00181020 + uVar4 * 0x18);
  iVar1 = *piVar5;
  while( true ) {
    if (iVar1 == 0) {
      *param_2 = uVar3;
      return (undefined *)0x0;
    }
    iVar2 = (int)uVar4;
    uVar3 = iVar2 + 1;
    uVar4 = (ulong)uVar3;
    if (param_1 == iVar1) break;
    iVar1 = piVar5[6];
    piVar5 = piVar5 + 6;
  }
  *param_2 = uVar3;
  return &DAT_00181020 + (long)iVar2 * 0x18;
}




// Function: compress_option @ 0xbf70

undefined8 compress_option(undefined8 param_1)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_14 = 0;
  lVar1 = FUN_0010bf10(param_1,&local_14);
  uVar2 = 0;
  if (lVar1 != 0) {
    uVar2 = *(undefined8 *)(lVar1 + 0x10);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: bufmap_free @ 0xbfc0

void bufmap_free(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  
  __ptr = DAT_00184148;
  while ((__ptr != (undefined8 *)0x0 && (__ptr != param_1))) {
    puVar1 = (undefined8 *)*__ptr;
    free((void *)__ptr[2]);
    free(__ptr);
    __ptr = puVar1;
  }
  DAT_00184148 = __ptr;
  if (__ptr != (undefined8 *)0x0) {
    return;
  }
  DAT_00184140 = 0;
  return;
}




// Function: bufmap_reset @ 0xc030

void bufmap_reset(undefined8 *param_1,long param_2)

{
  FUN_0010bfc0();
  for (; param_1 != (undefined8 *)0x0; param_1 = (undefined8 *)*param_1) {
    param_1[1] = param_1[1] + param_2;
    param_1[5] = 0;
  }
  return;
}




// Function: set_volume_start_time @ 0xc070

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void set_volume_start_time(void)

{
  FUN_001411a0(&DAT_00184a10);
  _DAT_00184a00 = DAT_00184a10;
  _DAT_00184a08 = DAT_00184a18;
  return;
}




// Function: print_stats @ 0xc0a0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int print_stats(double param_1,undefined8 param_2,char *param_3)

{
  double dVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  ulong uVar5;
  undefined *puVar6;
  long in_FS_OFFSET;
  double dVar7;
  undefined1 local_548 [656];
  undefined1 local_2b8 [664];
  long local_20;
  
  iVar3 = 0;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_3 != (char *)0x0) && (*param_3 != '\0')) {
    uVar4 = dcgettext(0,param_3,5);
    iVar3 = __fprintf_chk(param_2,1,"%s: ",uVar4);
  }
  dVar1 = DAT_00160758;
  if ((DAT_00183f78 <= _DAT_00160748) || (dVar7 = param_1 / DAT_00183f78, DAT_00160750 <= dVar7)) {
    puVar6 = &DAT_00167d0f;
  }
  else {
    if (DAT_00160758 <= dVar7) {
      uVar5 = (long)(dVar7 - DAT_00160758) ^ 0x8000000000000000;
    }
    else {
      uVar5 = (ulong)dVar7;
    }
    puVar6 = (undefined *)FUN_001426b0(uVar5,local_2b8,0x1b0,1,1);
  }
  if (dVar1 <= param_1) {
    uVar5 = (long)(param_1 - dVar1) ^ 0x8000000000000000;
  }
  else {
    uVar5 = (ulong)param_1;
  }
  uVar4 = FUN_001426b0(uVar5,local_548,0x1b0,1,1);
  iVar2 = __fprintf_chk(param_1,param_2,1,"%.0f (%s, %s/s)",uVar4,puVar6);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2 + iVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: increase_volume_number @ 0xc240

void increase_volume_number(void)

{
  undefined8 uVar1;
  
  DAT_00181010 = DAT_00181010 + 1;
  if (-1 < DAT_00181010) {
    DAT_00181014 = DAT_00181014 + 1;
    return;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = dcgettext(0,"Volume number overflow",5);
  error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: backspace_output @ 0xc290

void backspace_output(void)

{
  char cVar1;
  int iVar2;
  char extraout_var;
  char extraout_var_00;
  int *piVar3;
  long lVar4;
  undefined8 uVar5;
  __off_t __offset;
  long in_FS_OFFSET;
  undefined2 local_28 [2];
  undefined4 local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_24 = 1;
  local_28[0] = 4;
  if (DAT_00184a34 < 0x40000000) {
    iVar2 = ioctl(DAT_00184a34,0x40086d01,local_28);
    cVar1 = (char)((uint)iVar2 >> 0x18);
  }
  else {
    FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_28);
    cVar1 = extraout_var;
  }
  if (cVar1 < '\0') {
    piVar3 = __errno_location();
    if (*piVar3 == 5) {
      if (DAT_00184a34 < 0x40000000) {
        iVar2 = ioctl(DAT_00184a34,0x40086d01,local_28);
        cVar1 = (char)((uint)iVar2 >> 0x18);
      }
      else {
        FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_28);
        cVar1 = extraout_var_00;
      }
      if (-1 < cVar1) goto LAB_0010c3fe;
    }
    if (DAT_00184a34 < 0x40000000) {
      lVar4 = lseek(DAT_00184a34,0,1);
    }
    else {
      lVar4 = FUN_00133f90(DAT_00184a34 + -0x40000000,0,1);
    }
    __offset = lVar4 - (DAT_00183f50 - (long)DAT_00183f58);
    if (__offset < 0) {
      __offset = 0;
    }
    if (DAT_00184a34 < 0x40000000) {
      lVar4 = lseek(DAT_00184a34,__offset,0);
    }
    else {
      lVar4 = FUN_00133f90(DAT_00184a34 + -0x40000000,__offset,0);
    }
    if (lVar4 != __offset) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar5 = dcgettext(0,"Cannot backspace archive file; it may be unreadable without -i",5);
      error(0,0,uVar5);
      if (DAT_00183f58 != DAT_00184cf0) {
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          memset(DAT_00183f58,0,(long)DAT_00184cf0 - (long)DAT_00183f58);
          return;
        }
        goto LAB_0010c485;
      }
    }
  }
LAB_0010c3fe:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010c485:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: _flush_write @ 0xc490

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long _flush_write(void)

{
  ulong uVar1;
  undefined8 *puVar2;
  int *piVar3;
  long lVar4;
  long lVar5;
  ulong uVar6;
  long lVar7;
  
  FUN_001108d0(1);
  if ((DAT_00184a70 != _DAT_00160748) && (DAT_00184a70 <= DAT_001841a0)) {
    piVar3 = __errno_location();
    *piVar3 = 0x1c;
    return 0;
  }
  lVar4 = DAT_00184b98;
  if (DAT_00184a30 == '\0') {
    lVar4 = FUN_0012af00();
  }
  if ((((lVar4 != 0) && (DAT_00184b10 != '\0')) && (DAT_00184138 == 0)) &&
     (puVar2 = (undefined8 *)FUN_0010bed0(lVar4), puVar2 != (undefined8 *)0x0)) {
    lVar7 = puVar2[1];
    uVar6 = lVar4 + lVar7 * -0x200;
    uVar1 = puVar2[4];
    lVar5 = (uVar6 >> 9) + puVar2[5];
    if (uVar1 <= uVar6) {
      uVar6 = uVar1;
    }
    puVar2[5] = lVar5;
    puVar2[4] = uVar1 - uVar6;
    if (uVar1 - uVar6 == 0) {
      puVar2 = (undefined8 *)*puVar2;
      lVar7 = lVar7 + lVar5;
    }
    FUN_0010c030(puVar2,-lVar7);
  }
  return lVar4;
}




// Function: change_tape_menu @ 0xc580

void change_tape_menu(FILE *param_1)

{
  int iVar1;
  byte *pbVar2;
  undefined8 *puVar3;
  byte bVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  __ssize_t _Var7;
  byte *pbVar8;
  char *pcVar9;
  byte *pbVar10;
  long in_FS_OFFSET;
  byte *local_50;
  size_t local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = (byte *)0x0;
  local_48 = 0;
LAB_0010c5c8:
  do {
    fputc_unlocked(7,stderr);
    uVar5 = FUN_0014ac50(*DAT_00184828);
    iVar1 = DAT_00181010 + 1;
    uVar6 = dcgettext(0,"Prepare volume #%d for %s and hit return: ",5);
    __fprintf_chk(stderr,1,uVar6,iVar1,uVar5);
    fflush_unlocked(stderr);
    _Var7 = getline((char **)&local_50,&local_48,param_1);
    if (_Var7 < 1) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar9 = "EOF where user reply was expected";
LAB_0010c772:
      uVar5 = dcgettext(0,pcVar9,5);
      error(0,0,uVar5);
      if (2 < DAT_00184ba8 - 5U) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar5 = dcgettext(0,"WARNING: Archive is incomplete",5);
        error(0,0,uVar5);
      }
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    bVar4 = *local_50;
    if (((bVar4 & 0xdf) == 0x59) || (bVar4 == 10)) {
LAB_0010c870:
      free(local_50);
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    if (bVar4 != 0x3f) {
      if ((char)bVar4 < '@') {
        if ((bVar4 != 0x21) || (DAT_00184a58 != '\0')) {
LAB_0010c680:
          pcVar9 = "Invalid input. Type ? for help.\n";
          goto LAB_0010c68c;
        }
        FUN_0012add0();
      }
      else {
        if (bVar4 != 0x6e) {
          if (bVar4 != 0x71) goto LAB_0010c680;
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          pcVar9 = "No new volume; exiting.\n";
          goto LAB_0010c772;
        }
        bVar4 = local_50[1];
        pbVar10 = local_50 + 1;
        if ((bVar4 == 9) || (bVar4 == 0x20)) {
          do {
            do {
              bVar4 = pbVar10[1];
              pbVar10 = pbVar10 + 1;
            } while (bVar4 == 0x20);
          } while (bVar4 == 9);
        }
        pbVar8 = pbVar10;
        if ((bVar4 != 10) && (bVar4 != 0)) {
          do {
            pbVar2 = pbVar8 + 1;
            pbVar8 = pbVar8 + 1;
            if (*pbVar2 == 0) break;
          } while (*pbVar2 != 10);
        }
        *pbVar8 = 0;
        puVar3 = DAT_00184828;
        if (*pbVar10 != 0) {
          uVar5 = FUN_0014c7a0();
          *puVar3 = uVar5;
          goto LAB_0010c870;
        }
        uVar5 = dcgettext(0,"File name not specified. Try again.\n",5);
        __fprintf_chk(stderr,1,"%s",uVar5);
      }
      goto LAB_0010c5c8;
    }
    uVar5 = dcgettext(0,
                      " n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n"
                      ,5);
    __fprintf_chk(stderr,1,uVar5);
    if (DAT_00184a58 == '\0') {
      uVar5 = dcgettext(0," !             Spawn a subshell\n",5);
      __fprintf_chk(stderr,1,uVar5);
    }
    pcVar9 = " ?             Print this list\n";
LAB_0010c68c:
    uVar5 = dcgettext(0,pcVar9,5);
    __fprintf_chk(stderr,1,uVar5);
  } while( true );
}




// Function: guess_seekable_archive @ 0xc8d0

void guess_seekable_archive(void)

{
  int iVar1;
  long in_FS_OFFSET;
  stat sStack_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184ba8 == 4) {
    DAT_001847f8 = 0;
  }
  if (DAT_001847fc == -1) {
    if ((DAT_00184b10 == '\0') && (DAT_00184b70 == 0)) {
      iVar1 = fstat(DAT_00184a34,&sStack_a8);
      if (iVar1 == 0) {
        DAT_001847f8 = (sStack_a8.st_mode & 0xf000) == 0x8000;
        goto LAB_0010c90a;
      }
    }
    DAT_001847f8 = false;
  }
  else {
    DAT_001847f8 = DAT_001847fc != 0;
  }
LAB_0010c90a:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: init_buffer @ 0xc980

void init_buffer(void)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  
  lVar3 = (long)DAT_00184178;
  lVar1 = *(long *)(&DAT_00184180 + lVar3 * 8);
  if (lVar1 == 0) {
    uVar2 = FUN_00125000(&DAT_00184190 + lVar3,DAT_00184b98);
    *(undefined8 *)(&DAT_00184180 + lVar3 * 8) = uVar2;
    lVar1 = *(long *)(&DAT_00184180 + (long)DAT_00184178 * 8);
  }
  DAT_00183f58 = lVar1;
  DAT_00183f48 = lVar1;
  DAT_00183f50 = lVar1 + (long)DAT_00184ba0 * 0x200;
  return;
}




// Function: archive_is_dev @ 0xca00

undefined4 archive_is_dev(void)

{
  int iVar1;
  undefined4 uVar2;
  long in_FS_OFFSET;
  stat sStack_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fstat(DAT_00184a34,&sStack_a8);
  if (iVar1 == 0) {
    uVar2 = CONCAT31((int3)((sStack_a8.st_mode & 0xb000) >> 8),
                     (sStack_a8.st_mode & 0xb000) == 0x2000);
  }
  else {
    FUN_00124ec0(*DAT_00184828);
    uVar2 = 0;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_tty @ 0xca80

void check_tty(int param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = strcmp((char *)*DAT_00184838,"-");
  if (iVar1 == 0) {
    iVar1 = isatty((uint)(param_1 != 0));
    if (iVar1 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      if (param_1 == 0) {
        uVar2 = dcgettext(0,"Refusing to read archive contents from terminal (missing -f option?)",5
                         );
      }
      else {
        uVar2 = dcgettext(0,"Refusing to write archive contents to terminal (missing -f option?)",5)
        ;
      }
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
  }
  return;
}




// Function: add_chunk_header @ 0xcb00

void add_chunk_header(long param_1)

{
  undefined8 uVar1;
  long lVar2;
  void **ppvVar3;
  long in_FS_OFFSET;
  void *local_1d8 [14];
  undefined4 local_168;
  __uid_t local_164;
  __gid_t local_160;
  undefined8 local_150;
  undefined8 local_c0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184ba4 == 4) {
    ppvVar3 = local_1d8;
    for (lVar2 = 0x37; lVar2 != 0; lVar2 = lVar2 + -1) {
      *ppvVar3 = (void *)0x0;
      ppvVar3 = ppvVar3 + 1;
    }
    local_1d8[0] = *(void **)(param_1 + 0x10);
    local_168 = 0x81a4;
    local_1d8[1] = local_1d8[0];
    local_164 = getuid();
    local_160 = getgid();
    local_1d8[0] = (void *)FUN_0011c080(local_1d8,"%d/GNUFileParts/%f.%n",(long)DAT_00181014);
    local_150 = *(undefined8 *)(param_1 + 0x20);
    local_1d8[1] = local_1d8[0];
    local_c0 = local_150;
    lVar2 = FUN_001132d0(local_1d8);
    if (lVar2 == 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    uVar1 = FUN_00112da0(0,local_1d8,lVar2);
    FUN_00112f00(uVar1);
    free(local_1d8[0]);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mv_begin_write @ 0xcc00

void mv_begin_write(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  long lVar3;
  undefined8 uVar4;
  
  if (DAT_00184b10 != '\0') {
    puVar2 = (undefined8 *)FUN_0014c5b0(0x30);
    puVar1 = puVar2;
    if (DAT_00184140 != (undefined8 *)0x0) {
      *DAT_00184140 = puVar2;
      puVar1 = DAT_00184148;
    }
    DAT_00184148 = puVar1;
    lVar3 = DAT_00183f48 - DAT_00183f58;
    DAT_00184140 = puVar2;
    *puVar2 = 0;
    puVar2[1] = lVar3 >> 9;
    uVar4 = FUN_0014c7a0(param_1);
    puVar2[3] = param_2;
    puVar2[4] = param_3;
    puVar2[2] = uVar4;
    puVar2[5] = 0;
    return;
  }
  return;
}




// Function: buffer_write_global_xheader @ 0xcca0

void buffer_write_global_xheader(void)

{
  FUN_0011ceb0(&DAT_001840e8);
  return;
}




// Function: mv_begin_read @ 0xccb0

void mv_begin_read(undefined8 *param_1)

{
  FUN_0010cc00(*param_1,param_1[0x11],param_1[0x11]);
  return;
}




// Function: mv_end @ 0xccd0

void mv_end(void)

{
  if (DAT_00184b10 == '\0') {
    return;
  }
  FUN_0010bfc0(0);
  return;
}




// Function: mv_size_left @ 0xccf0

void mv_size_left(undefined8 param_1)

{
  if (DAT_00184148 != 0) {
    *(undefined8 *)(DAT_00184148 + 0x20) = param_1;
  }
  return;
}




// Function: clear_read_error_count @ 0xcd10

void clear_read_error_count(void)

{
  DAT_00184164 = 0;
  return;
}




// Function: set_start_time @ 0xcd20

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void set_start_time(void)

{
  FUN_001411a0(&DAT_00184a20);
  DAT_00184a10 = DAT_00184a20;
  DAT_00184a18 = DAT_00184a28;
  _DAT_00184a00 = DAT_00184a20;
  _DAT_00184a08 = DAT_00184a28;
  return;
}




// Function: compute_duration @ 0xcd50

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

double compute_duration(void)

{
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001411a0(&local_28);
  DAT_00183f78 = (double)(local_20 - _DAT_00184a08) / _DAT_00160760 +
                 (double)(local_28 - _DAT_00184a00) + DAT_00183f78;
  FUN_001411a0(&DAT_00184a00);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00183f78;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: first_decompress_program @ 0xcdf0

long first_decompress_program(undefined4 *param_1)

{
  int iVar1;
  long lVar2;
  long lVar3;
  
  lVar3 = DAT_00184b70;
  iVar1 = DAT_00183f70;
  if ((DAT_00184b70 == 0) && (DAT_00183f70 != 0)) {
    *param_1 = 0;
    lVar2 = FUN_0010bf10(iVar1,param_1);
    if (lVar2 != 0) {
      return *(long *)(lVar2 + 8);
    }
  }
  return lVar3;
}




// Function: next_decompress_program @ 0xce40

undefined8 next_decompress_program(undefined8 param_1)

{
  long lVar1;
  
  if (DAT_00184b70 == 0) {
    lVar1 = FUN_0010bf10(DAT_00183f70,param_1);
    if (lVar1 != 0) {
      return *(undefined8 *)(lVar1 + 8);
    }
  }
  return 0;
}




// Function: format_total_stats @ 0xce80

int format_total_stats(FILE *param_1,undefined8 *param_2,int param_3,int param_4)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  ulong uVar5;
  long in_FS_OFFSET;
  double dVar6;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184ba8 < 8) {
    if (4 < DAT_00184ba8) {
      uVar5 = DAT_00183f38 * DAT_00184b98;
      uVar4 = dcgettext(0,*param_2,5);
      iVar2 = FUN_0010c0a0((double)uVar5,param_1,uVar4);
      goto joined_r0x0010d017;
    }
    if (DAT_00184ba8 == 4) {
      iVar2 = FUN_0010c0a0((double)(ulong)(DAT_00183f38 * DAT_00184b98),param_1,*param_2);
      fputc_unlocked(param_3,param_1);
      iVar3 = FUN_0010c0a0(DAT_001841a8 + DAT_001841a0,param_1,param_2[1]);
      fputc_unlocked(param_3,param_1);
      pcVar1 = (char *)param_2[2];
      iVar2 = iVar2 + iVar3 + 2;
      if ((pcVar1 != (char *)0x0) && (*pcVar1 != '\0')) {
        uVar4 = dcgettext(0,pcVar1,5);
        iVar3 = __fprintf_chk(param_1,1,"%s: ",uVar4);
        iVar2 = iVar2 + iVar3;
      }
      dVar6 = (double)(ulong)((DAT_00183f38 - DAT_001842d8) * DAT_00184b98) -
              (DAT_001841a8 + DAT_001841a0);
      if (DAT_00160758 <= dVar6) {
        uVar5 = (long)(dVar6 - DAT_00160758) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)dVar6;
      }
      uVar4 = FUN_00143090(uVar5,local_48);
      iVar3 = __fprintf_chk(param_1,1,"%s",uVar4);
      iVar2 = iVar2 + iVar3;
      goto joined_r0x0010d017;
    }
    if (DAT_00184ba8 == 0) goto LAB_0010d034;
  }
  else if (DAT_00184ba8 != 8) {
LAB_0010d034:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  iVar2 = FUN_0010c0a0(DAT_001841a8 + DAT_001841a0,param_1,param_2[1]);
joined_r0x0010d017:
  if (param_4 != 0) {
    iVar2 = iVar2 + 1;
    fputc_unlocked(param_4,param_1);
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar2;
}




// Function: print_total_stats @ 0xd150

void print_total_stats(void)

{
  FUN_0010ce80(stderr,&PTR_s_Total_bytes_read_0017f0e0,10,10);
  return;
}




// Function: current_block_ordinal @ 0xd180

long current_block_ordinal(void)

{
  return (DAT_00183f48 - DAT_00183f58 >> 9) + DAT_00184170;
}




// Function: reset_eof @ 0xd1a0

void reset_eof(void)

{
  if (DAT_00184161 != '\0') {
    DAT_00184161 = '\0';
    DAT_00183f40 = 1;
    DAT_00183f48 = DAT_00183f58;
    DAT_00183f50 = DAT_00183f58 + (long)DAT_00184ba0 * 0x200;
  }
  return;
}




// Function: set_next_block_after @ 0xd1f0

void set_next_block_after(ulong param_1)

{
  if (DAT_00183f48 <= param_1) {
    DAT_00183f48 = DAT_00183f48 + 0x200 + (param_1 - DAT_00183f48 & 0xfffffffffffffe00);
  }
  if (DAT_00183f48 <= DAT_00183f50) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: available_space_after @ 0xd2b0

long available_space_after(long param_1)

{
  return DAT_00183f50 - param_1;
}




// Function: xclose @ 0xd2c0

void xclose(int param_1)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = close(param_1);
  if (iVar2 == 0) {
    return;
  }
  uVar1 = dcgettext(0,"(pipe)",5);
  FUN_00132680(uVar1);
  return;
}




// Function: archive_read_error @ 0xd300

void archive_read_error(void)

{
  int iVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  FUN_00132820(*DAT_00184828);
  iVar1 = DAT_00184164;
  if (DAT_00184170 == 0) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar3 = "At beginning of tape, quitting now";
  }
  else {
    DAT_00184164 = DAT_00184164 + 1;
    if (iVar1 < 0xb) {
      return;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar3 = "Too many errors, quitting";
  }
  uVar2 = dcgettext(0,pcVar3,5);
  error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: short_read @ 0xd390

void short_read(ulong param_1)

{
  char cVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  
  lVar4 = DAT_00183f58 + param_1;
  uVar3 = DAT_00184b98 - param_1;
  if ((((uVar3 != 0) && ((DAT_00183b80._2_1_ & 0x40) != 0)) &&
      ((uVar3 & 0x1ff) == 0 && DAT_00184170 == 0)) &&
     ((param_1 != 0 && (cVar1 = FUN_0010ca00(), cVar1 != '\0')))) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = dcngettext(0,"Record size = %lu block","Record size = %lu blocks",param_1 >> 9,5);
    error(0,0,uVar2,param_1 >> 9);
  }
  while( true ) {
    if ((uVar3 & 0x1ff) == 0) {
      if (((uVar3 == 0) || (param_1 == 0)) || (DAT_00184160 == '\0')) break;
    }
    else if (param_1 == 0) break;
    while( true ) {
      if (DAT_00184a34 < 0x40000000) {
        param_1 = FUN_0014af10(DAT_00184a34,lVar4,uVar3);
      }
      else {
        param_1 = FUN_00133da0(DAT_00184a34 + -0x40000000,lVar4,uVar3);
      }
      if (param_1 != 0xffffffffffffffff) break;
      FUN_0010d300();
    }
    if (param_1 == 0) break;
    if (DAT_00184160 == '\0') {
      lVar4 = DAT_00184b98 - uVar3;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcngettext(0,"Unaligned block (%lu byte) in archive",
                         "Unaligned block (%lu bytes) in archive",lVar4,5);
      error(0,0,uVar2,lVar4);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    uVar3 = uVar3 - param_1;
    lVar4 = lVar4 + param_1;
  }
  DAT_00183f38 = DAT_00183f38 + 1;
  DAT_00183f50 = (DAT_00184b98 - uVar3 & 0xfffffffffffffe00) + DAT_00183f58;
  return;
}




// Function: seek_archive @ 0xd540

ulong seek_archive(long param_1)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  long lVar3;
  undefined8 uVar4;
  ulong uVar5;
  ulong uVar6;
  undefined1 auVar7 [16];
  
  lVar3 = (long)DAT_00184ba0 - (DAT_00183f48 - DAT_00183f58 >> 9);
  if (lVar3 * 0x200 < param_1) {
    uVar6 = param_1 + lVar3 * -0x200;
    if (DAT_00184b98 <= uVar6) {
      uVar5 = DAT_00184b98;
      auVar7 = FUN_0010d180(uVar6,DAT_00184b98,0);
      auVar1._8_8_ = 0;
      auVar1._0_8_ = uVar5;
      auVar2._8_8_ = auVar7._8_8_;
      auVar2._0_8_ = uVar6;
      lVar3 = uVar5 * SUB168(auVar2 / auVar1,0);
      if (DAT_00184a34 < 0x40000000) {
        uVar6 = lseek(DAT_00184a34,lVar3,1);
      }
      else {
        uVar6 = FUN_00133f90(DAT_00184a34 + -0x40000000,lVar3,1);
      }
      if (-1 < (long)uVar6) {
        if (uVar6 % DAT_00184b98 != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"rmtlseek not stopped at a record boundary",5);
          error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
          FUN_001163d0();
        }
        DAT_00184170 = ((long)uVar6 >> 9) - (long)DAT_00184ba0;
        uVar6 = ((long)uVar6 >> 9) - auVar7._0_8_;
        DAT_00183f38 = DAT_00183f38 + (long)uVar6 / (long)DAT_00184ba0;
        DAT_00183f48 = DAT_00183f50;
      }
      return uVar6;
    }
  }
  return 0;
}




// Function: write_fatal_details @ 0xd670

void write_fatal_details(void)

{
  int iVar1;
  
  FUN_00132e30();
  if (DAT_00184a34 < 0x40000000) {
    iVar1 = close(DAT_00184a34);
  }
  else {
    iVar1 = FUN_00133d60(DAT_00184a34 + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_00132680(*DAT_00184828);
  }
  FUN_0012aca0(DAT_00184168,0);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: archive_write_error @ 0xd6d0

void archive_write_error(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  if (DAT_00184a69 != '\0') goto LAB_0010d6fd;
  do {
    FUN_0010d670(*DAT_00184828,param_1,DAT_00184b98);
LAB_0010d6fd:
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    FUN_0010d150();
    *piVar2 = iVar1;
  } while( true );
}




// Function: simple_flush_write @ 0xd720

void simple_flush_write(void)

{
  long lVar1;
  
  lVar1 = FUN_0010c490();
  if (DAT_00184b98 == lVar1) {
    DAT_00183f30 = DAT_00183f30 + 1;
    DAT_001841a0 = (double)lVar1 + DAT_001841a0;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d6d0(lVar1);
}




// Function: simple_flush_read @ 0xd770

void simple_flush_read(void)

{
  long lVar1;
  
  FUN_001108d0();
  DAT_00184164 = 0;
  lVar1 = DAT_00184b98;
  if ((DAT_00183f20 != '\0') && (DAT_00184170 != 0)) {
    DAT_00184a34 = 1;
    lVar1 = FUN_0012af00();
    DAT_00184a34 = 0;
    if (DAT_00184b98 != lVar1) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d6d0(lVar1);
    }
  }
  while( true ) {
    if (DAT_00184a34 < 0x40000000) {
      lVar1 = FUN_0014af10(DAT_00184a34,DAT_00183f58,lVar1);
    }
    else {
      lVar1 = FUN_00133da0(DAT_00184a34 + -0x40000000);
    }
    if (DAT_00184b98 == lVar1) break;
    if (lVar1 != -1) {
      FUN_0010d390();
      return;
    }
    FUN_0010d300();
    lVar1 = DAT_00184b98;
  }
  DAT_00183f38 = DAT_00183f38 + 1;
  return;
}




// Function: init_volume_number @ 0xd850

void init_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  __stream = fopen(DAT_00184a48,"r");
  if (__stream == (FILE *)0x0) {
    piVar4 = __errno_location();
    if (*piVar4 != 2) {
      FUN_001327c0(DAT_00184a48);
      return;
    }
  }
  else {
    iVar1 = __isoc99_fscanf(__stream,"%d",&DAT_00181010);
    if ((iVar1 != 1) || (DAT_00181010 < 0)) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = FUN_0014aa70(DAT_00184a48);
      uVar3 = dcgettext(0,"%s: contains invalid volume number",5);
      error(0,0,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    iVar1 = ferror_unlocked(__stream);
    if (iVar1 == 0) {
      iVar1 = fclose(__stream);
    }
    else {
      FUN_00132820(DAT_00184a48);
      iVar1 = fclose(__stream);
    }
    if (iVar1 != 0) {
      FUN_00132680(DAT_00184a48);
      return;
    }
  }
  return;
}




// Function: closeout_volume_number @ 0xd960

void closeout_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  
  __stream = fopen(DAT_00184a48,"w");
  if (__stream == (FILE *)0x0) {
    FUN_001327c0(DAT_00184a48);
    return;
  }
  __fprintf_chk(__stream,1,&DAT_0016767a,DAT_00181010);
  iVar1 = ferror_unlocked(__stream);
  if (iVar1 == 0) {
    iVar1 = fclose(__stream);
  }
  else {
    FUN_00132e10(DAT_00184a48);
    iVar1 = fclose(__stream);
  }
  if (iVar1 == 0) {
    return;
  }
  FUN_00132680(DAT_00184a48);
  return;
}




// Function: new_volume @ 0xda00

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 new_volume(int param_1)

{
  int iVar1;
  void *pvVar2;
  FILE *pFVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  char *__s1;
  
  pFVar3 = DAT_00183f68;
  if (((DAT_00183f68 == (FILE *)0x0) && (DAT_00184b48 == 0)) && (pFVar3 = stdin, DAT_00184a34 == 0))
  {
    pFVar3 = fopen("/dev/tty","r");
  }
  DAT_00183f68 = pFVar3;
  if (DAT_00184280 != '\0') {
    return 0;
  }
  if (DAT_00184a50 != '\0') {
    FUN_001118d0();
  }
  FUN_00123be0(&DAT_00183f18,0);
  FUN_00123be0(&DAT_00183f10,0);
  DAT_00183f00 = 0;
  DAT_00183f08 = 0;
  DAT_00183f48 = DAT_00183f58;
  if (DAT_00184a34 < 0x40000000) {
    iVar1 = close(DAT_00184a34);
  }
  else {
    iVar1 = FUN_00133d60(DAT_00184a34 + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_00132680(*DAT_00184828);
  }
  DAT_00184828 = DAT_00184828 + 1;
  if (DAT_00184828 == DAT_00184838 + DAT_00184830) {
    DAT_00184828 = DAT_00184838;
    DAT_00183f60 = 1;
    goto LAB_0010db51;
  }
  if (DAT_00183f60 != 0) goto LAB_0010db51;
  do {
    puVar6 = DAT_00184828;
    __s1 = (char *)*DAT_00184828;
    iVar1 = strcmp(__s1,"-");
    if (iVar1 == 0) {
      DAT_00184160 = 1;
      DAT_00184a34 = 0;
      return 1;
    }
    if (DAT_00184a50 == '\0') {
      if (param_1 == 1) {
        if (DAT_00184b84 != '\0') {
          FUN_00124710(__s1,1);
          __s1 = (char *)*DAT_00184828;
          puVar6 = DAT_00184828;
        }
        if (DAT_00184d70 == '\0') {
          _DAT_00184d78 = strchr(__s1,0x3a);
          __s1 = (char *)*puVar6;
          if (((_DAT_00184d78 != (char *)0x0) && (__s1 < _DAT_00184d78)) &&
             (pvVar2 = memchr(__s1,0x2f,(long)_DAT_00184d78 - (long)__s1), pvVar2 == (void *)0x0)) {
            DAT_00184a34 = FUN_001338d0(__s1,0x41,0x40000000,DAT_00184ab0);
            goto LAB_0010dc82;
          }
        }
        DAT_00184a34 = creat(__s1,0x1b6);
      }
      else {
        if (param_1 == 2) goto LAB_0010dc38;
        if (DAT_00184d70 == '\0') {
          _DAT_00184d78 = strchr(__s1,0x3a);
          __s1 = (char *)*puVar6;
          if (((_DAT_00184d78 == (char *)0x0) || (_DAT_00184d78 <= __s1)) ||
             (pvVar2 = memchr(__s1,0x2f,(long)_DAT_00184d78 - (long)__s1), pvVar2 != (void *)0x0))
          goto LAB_0010dc0f;
          DAT_00184a34 = FUN_001338d0(__s1,0,0x40000000,DAT_00184ab0);
        }
        else {
LAB_0010dc0f:
          DAT_00184a34 = open(__s1,0,0x1b6);
        }
        FUN_0010c8d0();
      }
    }
    else {
LAB_0010dc38:
      if (DAT_00184d70 == '\0') {
        _DAT_00184d78 = strchr(__s1,0x3a);
        __s1 = (char *)*puVar6;
        if (((_DAT_00184d78 != (char *)0x0) && (__s1 < _DAT_00184d78)) &&
           (pvVar2 = memchr(__s1,0x2f,(long)_DAT_00184d78 - (long)__s1), pvVar2 == (void *)0x0)) {
          DAT_00184a34 = FUN_001338d0(__s1,0x42,0x40000000,DAT_00184ab0);
          goto LAB_0010dc82;
        }
      }
      DAT_00184a34 = open(__s1,0x42,0x1b6);
    }
LAB_0010dc82:
    if (-1 < DAT_00184a34) {
      return 1;
    }
    FUN_00132800(*DAT_00184828);
    if (((DAT_00184a50 != '\x01') && (param_1 == 1)) && (DAT_00184b84 != '\0')) {
      FUN_001245d0();
    }
LAB_0010db51:
    if (DAT_00184b48 == 0) {
      FUN_0010c580(DAT_00183f68);
    }
    else {
      if (DAT_00184a48 != 0) {
        FUN_0010d960();
      }
      iVar1 = FUN_0012b8b0(DAT_00184828,DAT_00181010 + 1);
      if (iVar1 != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = FUN_0014ac50(DAT_00184b48);
        uVar5 = dcgettext(0,"%s command failed",5);
        error(0,0,uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
    }
  } while( true );
}




// Function: drop_volume_label_suffix @ 0xde80

void * drop_volume_label_suffix(byte *param_1)

{
  int iVar1;
  size_t sVar2;
  ushort **ppuVar3;
  void *__dest;
  byte *pbVar4;
  
  sVar2 = strlen((char *)param_1);
  if ((sVar2 != 0) && (pbVar4 = param_1 + (sVar2 - 1), param_1 < pbVar4)) {
    ppuVar3 = __ctype_b_loc();
    do {
      if ((*(byte *)((long)*ppuVar3 + (ulong)*pbVar4 * 2 + 1) & 8) == 0) {
        if (pbVar4 <= param_1) {
          return (void *)0x0;
        }
        pbVar4 = pbVar4 + -7;
        if ((param_1 < pbVar4) && (iVar1 = memcmp(pbVar4," Volume ",8), iVar1 == 0)) {
          sVar2 = (long)pbVar4 - (long)param_1;
          __dest = (void *)FUN_0014c5b0(sVar2 + 1);
          memcpy(__dest,param_1,sVar2);
          *(undefined1 *)((long)__dest + sVar2) = 0;
          return __dest;
        }
        return (void *)0x0;
      }
      pbVar4 = pbVar4 + -1;
    } while (param_1 != pbVar4);
  }
  return (void *)0x0;
}




// Function: check_label_pattern @ 0xdf40

bool check_label_pattern(char *param_1)

{
  bool bVar1;
  int iVar2;
  char *__pattern;
  
  iVar2 = fnmatch(DAT_00184a40,param_1,0);
  bVar1 = true;
  if (iVar2 != 0) {
    if (DAT_00184b10 != '\0') {
      __pattern = (char *)FUN_0010de80(param_1);
      if (__pattern != (char *)0x0) {
        iVar2 = fnmatch(__pattern,DAT_00184a40,0);
        free(__pattern);
        return iVar2 == 0;
      }
    }
    bVar1 = false;
  }
  return bVar1;
}




// Function: flush_archive @ 0xdfd0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void flush_archive(void)

{
  long lVar1;
  long lVar2;
  long lVar3;
  
  if (DAT_00183f40 == 0) {
    if (DAT_00184cf8 == '\0') {
      DAT_00184170 = DAT_00184170 + ((long)DAT_00183f50 - DAT_00183f58 >> 9);
      DAT_00183f50 = (void *)(DAT_00183f58 + (long)DAT_00184ba0 * 0x200);
      goto LAB_0010e027;
    }
    DAT_00183f40 = 1;
    DAT_00184cf8 = '\0';
    FUN_0010c290();
    lVar3 = (long)DAT_00184ba0;
    lVar2 = (long)DAT_00183f50 - DAT_00183f58 >> 9;
    if (lVar2 < lVar3) {
      memset(DAT_00183f50,0,(lVar3 - lVar2) * 0x200);
      DAT_00183f50 = (void *)((long)DAT_00184ba0 * 0x200 + DAT_00183f58);
      return;
    }
  }
  else {
    lVar3 = (long)DAT_00184ba0;
    lVar2 = (long)DAT_00183f50 - DAT_00183f58 >> 9;
  }
  lVar1 = DAT_00183f48;
  DAT_00184170 = DAT_00184170 + lVar2;
  DAT_00183f50 = (void *)(lVar3 * 0x200 + DAT_00183f58);
  DAT_00183f48 = DAT_00183f58;
  if (DAT_00183f40 == 1) {
                    /* WARNING: Could not recover jumptable at 0x0010e07b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*_DAT_00184158)(lVar1 - DAT_00183f58);
    return;
  }
  if (DAT_00183f40 == 2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  if (DAT_00183f40 != 0) {
    DAT_00183f48 = DAT_00183f58;
    return;
  }
LAB_0010e027:
  DAT_00183f48 = DAT_00183f58;
  FUN_0010dfc0();
  return;
}




// Function: find_next_block @ 0xe130

long find_next_block(void)

{
  long lVar1;
  
  lVar1 = DAT_00183f48;
  if ((DAT_00183f48 == DAT_00183f50) && (lVar1 = 0, DAT_00184161 == '\0')) {
    FUN_0010dfd0();
    lVar1 = DAT_00183f48;
    if (DAT_00183f48 == DAT_00183f50) {
      DAT_00184161 = '\x01';
      lVar1 = 0;
    }
    return lVar1;
  }
  return lVar1;
}




// Function: _write_volume_label @ 0xe190

void _write_volume_label(undefined8 param_1)

{
  undefined8 *puVar1;
  ulong uVar2;
  undefined8 *puVar3;
  byte bVar4;
  
  bVar4 = 0;
  if (DAT_00184ba4 != 4) {
    puVar1 = (undefined8 *)FUN_0010e130();
    *puVar1 = 0;
    puVar1[0x3f] = 0;
    puVar3 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    for (uVar2 = (ulong)(((int)puVar1 -
                         (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8)) + 0x200U >>
                        3); uVar2 != 0; uVar2 = uVar2 - 1) {
      *puVar3 = 0;
      puVar3 = puVar3 + (ulong)bVar4 * -2 + 1;
    }
    __strcpy_chk(puVar1,param_1,100);
    FUN_00123be0(&DAT_00184848,puVar1);
    DAT_00184850 = FUN_0013e630(DAT_00184848);
    *(undefined1 *)((long)puVar1 + 0x9c) = 0x56;
    FUN_00112c10(DAT_00184a20,puVar1 + 0x11,0xc);
    FUN_00112f60(&DAT_00184840,puVar1,0xffffffffffffffff);
    FUN_0010d1f0(puVar1);
    return;
  }
  FUN_0011ca30("GNU.volume.label",&DAT_00183f80,param_1);
  return;
}




// Function: add_volume_label @ 0xe280

void add_volume_label(void)

{
  char *__s;
  size_t sVar1;
  size_t sVar2;
  void *__ptr;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_00143090((long)DAT_00181014,auStack_38);
  sVar1 = strlen(DAT_00184a40);
  sVar2 = strlen(__s);
  __ptr = (void *)FUN_0014c5b0(sVar1 + 9 + sVar2);
  __sprintf_chk(__ptr,1,0xffffffffffffffff,"%s %s %s",DAT_00184a40,"Volume",__s);
  FUN_0010e190(__ptr);
  free(__ptr);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_volume_label @ 0xe330

void write_volume_label(void)

{
  if (DAT_00184b10 != '\0') {
    FUN_0010e280();
    return;
  }
  FUN_0010e190(DAT_00184a40);
  return;
}




// Function: gnu_add_multi_volume_header @ 0xe350

void gnu_add_multi_volume_header(long param_1)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulong uVar6;
  long lVar7;
  uint uVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  char *__s;
  byte bVar11;
  
  bVar11 = 0;
  puVar2 = (undefined8 *)FUN_0010e130();
  __s = *(char **)(param_1 + 0x10);
  sVar3 = strlen(__s);
  if (100 < sVar3) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
      __s = *(char **)(param_1 + 0x10);
    }
    uVar4 = FUN_0014aa70(__s);
    uVar5 = dcgettext(0,"%s: file name too long to be stored in a GNU multivolume header, truncated"
                      ,5);
    error(0,0,uVar5,uVar4);
    sVar3 = 100;
  }
  *puVar2 = 0;
  puVar2[0x3f] = 0;
  puVar9 = (undefined8 *)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
  uVar6 = (ulong)(((int)puVar2 - (int)puVar9) + 0x200U >> 3);
  for (; uVar6 != 0; uVar6 = uVar6 - 1) {
    *puVar9 = 0;
    puVar9 = puVar9 + (ulong)bVar11 * -2 + 1;
  }
  uVar8 = (uint)sVar3;
  uVar6 = sVar3 & 0xffffffff;
  puVar9 = *(undefined8 **)(param_1 + 0x10);
  if (uVar8 < 8) {
    if ((sVar3 & 4) == 0) {
      if ((uVar8 != 0) && (*(undefined1 *)puVar2 = *(undefined1 *)puVar9, (sVar3 & 2) != 0)) {
        *(undefined2 *)((long)puVar2 + (uVar6 - 2)) = *(undefined2 *)((long)puVar9 + (uVar6 - 2));
      }
    }
    else {
      *(undefined4 *)puVar2 = *(undefined4 *)puVar9;
      *(undefined4 *)((long)puVar2 + (uVar6 - 4)) = *(undefined4 *)((long)puVar9 + (uVar6 - 4));
    }
  }
  else {
    puVar10 = (undefined8 *)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
    *puVar2 = *puVar9;
    *(undefined8 *)((long)puVar2 + ((sVar3 & 0xffffffff) - 8)) =
         *(undefined8 *)((long)puVar9 + ((sVar3 & 0xffffffff) - 8));
    lVar7 = (long)puVar2 - (long)puVar10;
    puVar9 = (undefined8 *)((long)puVar9 - lVar7);
    for (uVar6 = (ulong)((int)lVar7 + uVar8 >> 3); uVar6 != 0; uVar6 = uVar6 - 1) {
      *puVar10 = *puVar9;
      puVar9 = puVar9 + (ulong)bVar11 * -2 + 1;
      puVar10 = puVar10 + (ulong)bVar11 * -2 + 1;
    }
  }
  *(undefined1 *)((long)puVar2 + 0x9c) = 0x4d;
  FUN_00112be0(*(undefined8 *)(param_1 + 0x20),(long)puVar2 + 0x7c,0xc);
  FUN_00112be0(*(long *)(param_1 + 0x18) - *(long *)(param_1 + 0x20),(long)puVar2 + 0x171,0xc);
  uVar1 = DAT_00184a54;
  DAT_00184a54 = 0;
  FUN_00112f60(&DAT_00184840,puVar2,0xffffffffffffffff);
  DAT_00184a54 = uVar1;
  FUN_0010d1f0(puVar2);
  return;
}




// Function: add_multi_volume_header @ 0xe4d0

void add_multi_volume_header(long param_1)

{
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184ba4 == 4) {
    local_28 = *(long *)(param_1 + 0x18) - *(long *)(param_1 + 0x20);
    FUN_0011ca30("GNU.volume.filename",&DAT_00183f80,*(undefined8 *)(param_1 + 0x10));
    FUN_0011ca30("GNU.volume.size",&DAT_00183f80,param_1 + 0x20);
    FUN_0011ca30("GNU.volume.offset",&DAT_00183f80,&local_28);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_0010e350();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: _gnu_flush_write @ 0xe580

void _gnu_flush_write(long param_1)

{
  char cVar1;
  ulong uVar2;
  long lVar3;
  int *piVar4;
  undefined8 uVar5;
  void *__dest;
  size_t __n;
  void *__src;
  void *__src_00;
  
  uVar2 = FUN_0010c490();
  if (DAT_00184b98 == uVar2) {
    DAT_001841a0 = (double)(long)DAT_00184b98 + DAT_001841a0;
    if (DAT_00184b98 != 0) {
      DAT_00183f30 = DAT_00183f30 + 1;
      return;
    }
  }
  else {
    if (DAT_00184b10 == '\0') {
LAB_0010e602:
                    /* WARNING: Subroutine does not return */
      FUN_0010d6d0(uVar2);
    }
    DAT_001841a0 = (double)(long)uVar2 + DAT_001841a0;
    if (uVar2 != 0) {
      DAT_00183f30 = DAT_00183f30 + 1;
      lVar3 = FUN_0010bed0(uVar2);
      if ((uVar2 & 0x1ff) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar5 = dcgettext(0,"write did not end on a block boundary",5);
        error(0,0,uVar5);
        DAT_00184d58 = 2;
        goto LAB_0010e602;
      }
      if ((long)uVar2 < 0) {
        piVar4 = __errno_location();
        if ((1 < *piVar4 - 5U) && (*piVar4 != 0x1c)) goto LAB_0010e602;
      }
      goto LAB_0010e647;
    }
  }
  if (DAT_00184b98 == 0) {
    return;
  }
  uVar2 = 0;
  lVar3 = FUN_0010bed0(0);
LAB_0010e647:
  cVar1 = FUN_0010da00(1);
  if (cVar1 == '\0') {
    return;
  }
  __n = param_1 - uVar2;
  FUN_0012f6f0(&DAT_00183f80);
  FUN_0010c240();
  DAT_001841a8 = DAT_001841a8 + DAT_001841a0;
  DAT_001841a0 = 0.0;
  __src = (void *)(DAT_00183f58 + uVar2);
  DAT_00184178 = (uint)(DAT_00184178 == 0);
  FUN_0010c980();
  DAT_00184138 = 1;
  if (DAT_00184a40 != 0) {
    FUN_0010e280();
  }
  if (lVar3 == 0) {
    uVar5 = FUN_0010e130();
    FUN_00112da0(1,&DAT_00183f80,uVar5);
    FUN_0012f6f0(&DAT_00183f80);
  }
  else {
    FUN_0010e4d0(lVar3);
    uVar5 = FUN_0010e130();
    FUN_00112da0(1,&DAT_00183f80,uVar5);
    FUN_0012f6f0(&DAT_00183f80);
    FUN_0010cb00(lVar3);
  }
  __dest = (void *)FUN_0010e130();
  FUN_0010c030(lVar3,(long)__dest - DAT_00183f58 >> 9);
  uVar2 = FUN_0010d2b0(__dest);
  DAT_00184138 = 0;
  __src_00 = __src;
  if (uVar2 < __n) {
    do {
      __src = (void *)((long)__src_00 + uVar2);
      memcpy(__dest,__src_00,uVar2);
      __n = __n - uVar2;
      FUN_0010d1f0((long)__dest + (uVar2 - 1 & 0xfffffffffffffe00));
      __dest = (void *)FUN_0010e130();
      uVar2 = FUN_0010d2b0(__dest);
      __src_00 = __src;
    } while (uVar2 < __n);
  }
  memcpy(__dest,__src,__n);
  memset((void *)((long)__dest + __n),0,uVar2 - __n);
  FUN_0010d1f0((long)__dest + (__n - 1 & 0xfffffffffffffe00));
  FUN_0010e130();
  return;
}




// Function: gnu_flush_write @ 0xe850

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void gnu_flush_write(void)

{
  _DAT_00184158 = FUN_0010d720;
  FUN_0010e580();
  _DAT_00184158 = gnu_flush_write;
  return;
}




// Function: try_new_volume @ 0xe880

undefined4 try_new_volume(void)

{
  char *__s1;
  code *pcVar1;
  char cVar2;
  undefined4 uVar3;
  int iVar4;
  long lVar5;
  size_t sVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  char *__s2;
  long in_FS_OFFSET;
  long local_90;
  undefined1 local_88 [32];
  undefined1 local_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184ba8 < 3) {
    uVar3 = FUN_0010da00((DAT_00184ba8 != 0) * '\x02');
    cVar2 = (char)uVar3;
  }
  else {
    uVar3 = FUN_0010da00((DAT_00184ba8 == 8) * '\x02');
    cVar2 = (char)uVar3;
  }
  if (cVar2 == '\0') {
    uVar3 = 1;
    goto LAB_0010eae1;
  }
  while( true ) {
    if (DAT_00184a34 < 0x40000000) {
      lVar5 = FUN_0014af10();
    }
    else {
      lVar5 = FUN_00133da0(DAT_00184a34 + -0x40000000,DAT_00183f58,DAT_00184b98);
    }
    if (lVar5 != -1) break;
    FUN_0010d300();
  }
  if (DAT_00184b98 == lVar5) {
    local_90 = FUN_0010e130();
  }
  else {
    FUN_0010d390(lVar5);
    local_90 = FUN_0010e130();
  }
  if (local_90 == 0) goto LAB_0010eb23;
  cVar2 = *(char *)(local_90 + 0x9c);
  if (cVar2 == 'V') {
    cVar2 = FUN_0010d230();
    if (cVar2 != '\0') {
      FUN_0012f6f0(&DAT_00183f80);
      FUN_00123c20(&DAT_00183f18,DAT_00184408,100);
      FUN_0010d1f0(local_90);
      local_90 = FUN_0010e130();
      cVar2 = *(char *)(local_90 + 0x9c);
      goto joined_r0x0010ebac;
    }
  }
  else {
    if (cVar2 == 'g') {
      FUN_0012f630(&DAT_00183f80);
      iVar4 = FUN_001219e0(&local_90,&DAT_00183f80,2);
      if (iVar4 != 2) {
LAB_0010eb23:
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar3 = 0;
        uVar7 = dcgettext(0,"This does not look like a tar archive",5);
        error(0,0,uVar7);
        goto LAB_0010eae1;
      }
      FUN_0011c8c0(&DAT_00183f80);
      FUN_0012f6f0(&DAT_00183f80);
      iVar4 = FUN_001219e0(&local_90,&DAT_00183f80,0);
      if (iVar4 == 1) {
        FUN_0010d1f0(local_90);
      }
      else if (iVar4 != 5) goto LAB_0010eb23;
    }
    else {
joined_r0x0010ebac:
      if (cVar2 == 'M') {
        cVar2 = FUN_0010d230();
        if (cVar2 == '\0') goto LAB_0010ecd9;
        FUN_0012f6f0(&DAT_00183f80);
        FUN_00123c20(&DAT_00183f10,DAT_00184408,100);
        DAT_00183f08 = FUN_00122050(DAT_00184408 + 0x7c,0xc);
        DAT_00183f00 = FUN_00122050(DAT_00184408 + 0x171,0xc);
      }
    }
    lVar5 = DAT_00184148;
    __s1 = DAT_00183f10;
    if (DAT_00184148 == 0) {
LAB_0010ece8:
      FUN_0010c240();
      goto LAB_0010eae1;
    }
    if (DAT_00183f10 == (char *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      __s2 = *(char **)(DAT_00184148 + 0x10);
LAB_0010ecad:
      uVar7 = FUN_0014ac50(__s2);
      uVar8 = dcgettext(0,"%s is not continued on this volume",5);
      error(0,0,uVar8,uVar7);
    }
    else {
      __s2 = *(char **)(DAT_00184148 + 0x10);
      iVar4 = strcmp(DAT_00183f10,__s2);
      pcVar1 = DAT_00184d50;
      if (iVar4 != 0) {
        if ((((DAT_00184ba4 & 0xfffffffb) != 2) || (sVar6 = strlen(__s2), sVar6 < 100)) ||
           (iVar4 = strncmp(__s1,__s2,100), iVar4 != 0)) {
          if (pcVar1 != (code *)0x0) {
            (*pcVar1)();
            __s2 = *(char **)(DAT_00184148 + 0x10);
          }
          goto LAB_0010ecad;
        }
        if (pcVar1 != (code *)0x0) {
          (*pcVar1)();
          __s2 = *(char **)(DAT_00184148 + 0x10);
        }
        uVar7 = FUN_0014ac50(__s2);
        uVar8 = dcgettext(0,
                          "%s is possibly continued on this volume: header contains truncated name",
                          5);
        error(0,0,uVar8,uVar7);
        lVar5 = DAT_00184148;
      }
      if (!CARRY8(DAT_00183f00,DAT_00183f08) &&
          DAT_00183f00 + DAT_00183f08 == *(long *)(lVar5 + 0x18)) {
        if (*(long *)(lVar5 + 0x18) - *(long *)(lVar5 + 0x20) != DAT_00183f00) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar7 = FUN_00143090(DAT_00183f00,local_48);
          uVar8 = FUN_00143090(*(undefined8 *)(DAT_00184148 + 0x20),local_68);
          uVar9 = FUN_00143090(*(undefined8 *)(DAT_00184148 + 0x18),local_88);
          uVar10 = dcgettext(0,"This volume is out of sequence (%s - %s != %s)",5);
          error(0,0,uVar10,uVar9,uVar8,uVar7);
          uVar3 = 0;
          goto LAB_0010eae1;
        }
        goto LAB_0010ece8;
      }
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar7 = FUN_00143090(DAT_00183f00,local_48);
      uVar8 = FUN_00143090(DAT_00183f08,local_68);
      uVar9 = FUN_00143090(*(undefined8 *)(DAT_00184148 + 0x18),local_88);
      uVar10 = FUN_0014ac50(DAT_00183f10);
      uVar11 = dcgettext(0,"%s is the wrong size (%s != %s + %s)",5);
      error(0,0,uVar11,uVar10,uVar9,uVar8,uVar7);
    }
  }
LAB_0010ecd9:
  uVar3 = 0;
LAB_0010eae1:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_compressed_archive @ 0xedb0

ulong check_compressed_archive(undefined1 *param_1)

{
  char *__s1;
  void *pvVar1;
  undefined1 uVar2;
  int iVar3;
  long lVar4;
  ulong uVar5;
  size_t __n;
  uint *puVar6;
  void *__s1_00;
  undefined *__s2;
  long in_FS_OFFSET;
  undefined1 local_21;
  long local_20;
  
  uVar2 = DAT_00184160;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00184160 = 1;
  if (param_1 == (undefined1 *)0x0) {
    param_1 = &local_21;
  }
  DAT_00183f50 = DAT_00183f58;
  lVar4 = FUN_0010e130();
  pvVar1 = DAT_00183f58;
  *param_1 = lVar4 == 0;
  __s1_00 = DAT_00183f50;
  DAT_00184160 = uVar2;
  if (DAT_00183f58 != DAT_00183f50) {
    __s1 = (char *)((long)DAT_00183f58 + 0x101);
    iVar3 = strcmp(__s1,"ustar");
    if (((iVar3 == 0) || (iVar3 = strcmp(__s1,"ustar  "), __s1_00 = pvVar1, iVar3 == 0)) &&
       (uVar5 = FUN_001214d0(pvVar1,1), __s1_00 = DAT_00183f58, (int)uVar5 == 1)) goto LAB_0010ee87;
  }
  __s2 = &DAT_00160127;
  __n = 2;
  puVar6 = &DAT_0017f130;
  while (iVar3 = memcmp(__s1_00,__s2,__n), iVar3 != 0) {
    if (puVar6 + 6 == (uint *)&DAT_0017f1f0) {
      uVar5 = 0;
      goto LAB_0010ee87;
    }
    __n = *(size_t *)(puVar6 + 8);
    __s2 = *(undefined **)(puVar6 + 10);
    puVar6 = puVar6 + 6;
  }
  uVar5 = (ulong)*puVar6;
LAB_0010ee87:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: open_compressed_archive @ 0xeed0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int open_compressed_archive(void)

{
  undefined8 *puVar1;
  int iVar2;
  void *pvVar3;
  undefined8 uVar4;
  char *__s;
  long in_FS_OFFSET;
  char local_11;
  long local_10;
  
  puVar1 = DAT_00184838;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)*DAT_00184838;
  if (DAT_00184d70 == '\0') {
    _DAT_00184d78 = strchr(__s,0x3a);
    __s = (char *)*puVar1;
    if (((_DAT_00184d78 == (char *)0x0) || (_DAT_00184d78 <= __s)) ||
       (pvVar3 = memchr(__s,0x2f,(long)_DAT_00184d78 - (long)__s), pvVar3 != (void *)0x0))
    goto LAB_0010ef39;
    DAT_00184a34 = FUN_001338d0(__s,0,0x40000000,DAT_00184ab0);
  }
  else {
LAB_0010ef39:
    DAT_00184a34 = open(__s,0,0x1b6);
  }
  if (DAT_00184a34 == -1) goto LAB_0010efaf;
  if (DAT_00184b10 == '\0') {
    iVar2 = DAT_00183f70;
    if (DAT_00184b70 == 0) {
      iVar2 = FUN_0010edb0(&local_11);
      if (iVar2 == 0) {
        if (local_11 != '\0') {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"This does not look like a tar archive",5);
          error(0,0,uVar4);
          DAT_00184d58 = 2;
        }
        FUN_0012a260(*DAT_00184838,0);
        iVar2 = DAT_00183f70;
        if (DAT_00184b70 == 0) goto LAB_0010efaf;
      }
      else if (iVar2 == 1) {
        if (local_11 != '\0') {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"This does not look like a tar archive",5);
          error(0,0,uVar4);
          DAT_00184d58 = 2;
        }
        goto LAB_0010efaf;
      }
    }
    DAT_00183f70 = iVar2;
    if (DAT_00184a34 < 0x40000000) {
      close(DAT_00184a34);
    }
    else {
      FUN_00133d60(DAT_00184a34 + -0x40000000);
    }
    DAT_00184161 = 0;
    DAT_00184168 = FUN_0012b310();
    DAT_00184160 = 1;
  }
  DAT_00183f38 = 0;
  DAT_00183f50 = DAT_00183f58;
LAB_0010efaf:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00184a34;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: _open_archive @ 0xf0f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void _open_archive(int param_1)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  undefined8 uVar5;
  void *pvVar6;
  undefined8 uVar7;
  char *pcVar8;
  undefined8 *puVar9;
  long in_FS_OFFSET;
  bool bVar10;
  char local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184b98 == 0) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar8 = "Invalid value for record_size";
    goto LAB_0010f3ab;
  }
  if (DAT_00184830 == 0) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar8 = "No archive name given";
    goto LAB_0010f3ab;
  }
  FUN_0012f6f0(&DAT_00184840);
  DAT_00184178 = 0;
  FUN_0010c980();
  if (param_1 != 2) {
    DAT_00183f40 = param_1;
    FUN_0010ca80(param_1);
    puVar9 = DAT_00184838;
    DAT_00184160 = DAT_00184ab9;
    DAT_00183f38 = 0;
    if (DAT_00184b70 != 0) {
      if (param_1 == 1) goto LAB_0010f230;
      if (param_1 == 0) {
        DAT_00184168 = FUN_0012b310();
        DAT_00184160 = 1;
        DAT_00183f50 = DAT_00183f58;
      }
      goto LAB_0010f1aa;
    }
    pcVar8 = (char *)*DAT_00184838;
    iVar2 = strcmp(pcVar8,"-");
    if (iVar2 == 0) goto LAB_0010f2d9;
    if (param_1 != 1) {
      if (param_1 == 0) {
        DAT_00184a34 = FUN_0010eed0();
        if (DAT_00184a34 < 0) goto LAB_0010f210;
        FUN_0010c8d0();
      }
      goto LAB_0010f1aa;
    }
    bVar10 = DAT_00184b84 != '\0';
    if (bVar10) {
      FUN_00124710(pcVar8,1);
      pcVar8 = (char *)*DAT_00184838;
      puVar9 = DAT_00184838;
    }
    if (DAT_00184a50 == '\0') {
      if (DAT_00184d70 == '\0') {
        _DAT_00184d78 = strchr(pcVar8,0x3a);
        pcVar8 = (char *)*puVar9;
        if (((_DAT_00184d78 != (char *)0x0) && (pcVar8 < _DAT_00184d78)) &&
           (pvVar6 = memchr(pcVar8,0x2f,(long)_DAT_00184d78 - (long)pcVar8), pvVar6 == (void *)0x0))
        {
          DAT_00184a34 = FUN_001338d0(pcVar8,0x41,0x40000000,DAT_00184ab0);
          goto LAB_0010f60a;
        }
      }
      DAT_00184a34 = creat(pcVar8,0x1b6);
    }
    else {
      if (DAT_00184d70 == '\0') {
        _DAT_00184d78 = strchr(pcVar8,0x3a);
        pcVar8 = (char *)*puVar9;
        if (((_DAT_00184d78 != (char *)0x0) && (pcVar8 < _DAT_00184d78)) &&
           (pvVar6 = memchr(pcVar8,0x2f,(long)_DAT_00184d78 - (long)pcVar8), pvVar6 == (void *)0x0))
        {
          DAT_00184a34 = FUN_001338d0(pcVar8,0x42,0x40000000,DAT_00184ab0);
          goto LAB_0010f60a;
        }
      }
      DAT_00184a34 = open(pcVar8,0x42,0x1b6);
    }
LAB_0010f60a:
    if (DAT_00184a34 < 0) {
LAB_0010f628:
      piVar4 = __errno_location();
      iVar2 = *piVar4;
      if (!bVar10) goto LAB_0010f218;
      FUN_001245d0();
      goto LAB_0010f218;
    }
    if (DAT_00184a34 < 0x40000000) {
      cVar1 = FUN_0012ab10();
      if (cVar1 == '\0') goto LAB_0010f628;
      goto LAB_0010f4a8;
    }
    FUN_0012abd0();
    FUN_0012ab80();
    goto LAB_0010f1d5;
  }
  DAT_00183f40 = 0;
  FUN_0010ca80(0);
  puVar9 = DAT_00184838;
  DAT_00184160 = DAT_00184ab9;
  DAT_00183f38 = 0;
  if (DAT_00184b70 != 0) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  pcVar8 = (char *)*DAT_00184838;
  iVar2 = strcmp(pcVar8,"-");
  if (iVar2 == 0) {
LAB_0010f2d9:
    DAT_00184160 = 1;
    if (DAT_00184a50 != '\0') {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar8 = "Cannot verify stdin/stdout archive";
      goto LAB_0010f3ab;
    }
    if (param_1 == 1) {
      DAT_00184a34 = 1;
      if (DAT_00184820 == 0) goto LAB_0010f48d;
    }
    else {
      if (param_1 != 2) {
        if (param_1 == 0) {
          DAT_00184a34 = 0;
          uVar3 = FUN_0010edb0(&local_21);
          if (1 < uVar3) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            uVar5 = FUN_0010bf70(uVar3);
            uVar7 = dcgettext(0,"Archive is compressed. Use %s option",5);
            error(0,0,uVar7,uVar5);
                    /* WARNING: Subroutine does not return */
            FUN_001163d0();
          }
          if (local_21 != '\0') {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            uVar5 = dcgettext(0,"This does not look like a tar archive",5);
            error(0,0,uVar5);
            DAT_00184d58 = 2;
          }
        }
        goto LAB_0010f1aa;
      }
      DAT_00183f50 = DAT_00183f58;
      DAT_00184a34 = 0;
      DAT_00183f20 = 1;
      if (DAT_00184820 != 0) goto LAB_0010f200;
LAB_0010f48d:
      DAT_00183f28 = stderr;
    }
    cVar1 = FUN_0012ab10();
    if (cVar1 == '\0') goto LAB_0010f210;
LAB_0010f4a8:
    FUN_0012abd0();
    FUN_0012ab80();
    goto LAB_0010f1cd;
  }
  if (DAT_00184d70 == '\0') {
    _DAT_00184d78 = strchr(pcVar8,0x3a);
    pcVar8 = (char *)*puVar9;
    if (((_DAT_00184d78 == (char *)0x0) || (_DAT_00184d78 <= pcVar8)) ||
       (pvVar6 = memchr(pcVar8,0x2f,(long)_DAT_00184d78 - (long)pcVar8), pvVar6 != (void *)0x0))
    goto LAB_0010f40f;
    DAT_00184a34 = FUN_001338d0(pcVar8,0x42,0x40000000,DAT_00184ab0);
  }
  else {
LAB_0010f40f:
    DAT_00184a34 = open(pcVar8,0x42,0x1b6);
  }
  uVar3 = FUN_0010edb0(0);
  if (uVar3 < 2) {
LAB_0010f1aa:
    if (-1 < DAT_00184a34) {
      if (0x3fffffff < DAT_00184a34) goto LAB_0010f1bb;
LAB_0010f200:
      cVar1 = FUN_0012ab10();
      if (cVar1 != '\0') goto LAB_0010f1bb;
    }
LAB_0010f210:
    piVar4 = __errno_location();
    iVar2 = *piVar4;
LAB_0010f218:
    *piVar4 = iVar2;
    FUN_001327e0(*DAT_00184838);
LAB_0010f230:
    DAT_00184168 = FUN_0012af40();
    if ((DAT_00184820 == 0) && (iVar2 = strcmp((char *)*DAT_00184838,"-"), iVar2 == 0)) {
      DAT_00183f28 = stderr;
    }
    goto LAB_0010f1aa;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  pcVar8 = "Cannot update compressed archives";
LAB_0010f3ab:
  uVar5 = dcgettext(0,pcVar8,5);
  error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
LAB_0010f1bb:
  FUN_0012abd0();
  FUN_0012ab80();
  if (param_1 == 0) {
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010e130();
      return;
    }
    goto LAB_0010f4f4;
  }
LAB_0010f1cd:
  if (param_1 - 1U < 2) {
LAB_0010f1d5:
    DAT_00183f30 = 0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010f4f4:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: match_volume_label @ 0xf760

void match_volume_label(void)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 auStack_1d8 [360];
  undefined1 local_70 [80];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00183f18 != 0) {
LAB_0010f789:
    cVar1 = FUN_0010df40();
    if (cVar1 == '\0') {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = FUN_0014ac40(1,DAT_00184a40);
      uVar4 = FUN_0014ac40(0,DAT_00183f18);
      uVar5 = dcgettext(0,"Volume %s does not match %s",5);
      error(0,0,uVar5,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  lVar2 = FUN_0010e130();
  if (lVar2 != 0) {
    if (*(char *)(lVar2 + 0x9c) == 'V') {
      FUN_00123c20(&DAT_00183f18,lVar2,100);
    }
    else if (*(char *)(lVar2 + 0x9c) == 'g') {
      FUN_0012f630(auStack_1d8);
      uVar3 = FUN_001219b0(lVar2 + 0x7c,0xc);
      FUN_0011cac0(local_70,lVar2,uVar3);
      FUN_0011c8c0(auStack_1d8);
      FUN_0012f6f0(auStack_1d8);
    }
    if (DAT_00183f18 != 0) goto LAB_0010f789;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar3 = FUN_0014ac50(DAT_00184a40);
  uVar4 = dcgettext(0,"Archive not labeled to match %s",5);
  error(0,0,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: close_archive @ 0xf900

void close_archive(void)

{
  int iVar1;
  
  if ((DAT_00184cf8 != '\0') || (DAT_00183f40 == 1)) {
    do {
      FUN_0010dfd0();
    } while (DAT_00183f58 < DAT_00183f48);
  }
  FUN_0010cd50();
  if (DAT_00184a50 != '\0') {
    FUN_001118d0();
  }
  if (DAT_00184a34 < 0x40000000) {
    iVar1 = close(DAT_00184a34);
  }
  else {
    iVar1 = FUN_00133d60(DAT_00184a34 + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_00132680(*DAT_00184828);
  }
  FUN_0012aca0(DAT_00184168,DAT_00184161);
  FUN_0012f6f0(&DAT_00184840);
  free(DAT_00184190);
  free(DAT_00184198);
  FUN_0010bfc0(0);
  return;
}




// Function: _gnu_flush_read @ 0xf9e0

void _gnu_flush_read(void)

{
  char cVar1;
  long lVar2;
  int *piVar3;
  
  FUN_001108d0();
  DAT_00184164 = 0;
  lVar2 = DAT_00184b98;
  if ((DAT_00183f20 != '\0') && (DAT_00184170 != 0)) {
    DAT_00184a34 = 1;
    lVar2 = FUN_0012af00();
    DAT_00184a34 = 0;
    if (DAT_00184b98 != lVar2) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d6d0(lVar2);
    }
  }
  while( true ) {
    if (DAT_00184a34 < 0x40000000) {
      lVar2 = FUN_0014af10(DAT_00184a34,DAT_00183f58,lVar2);
    }
    else {
      lVar2 = FUN_00133da0(DAT_00184a34 + -0x40000000);
    }
    if (DAT_00184b98 == lVar2) {
      DAT_00183f38 = DAT_00183f38 + 1;
      return;
    }
    if (lVar2 == 0) break;
    if (lVar2 != -1) goto LAB_0010fa4e;
    piVar3 = __errno_location();
    if ((*piVar3 == 0x1c) && (DAT_00184b10 != '\0')) goto LAB_0010fa79;
    FUN_0010d300();
    lVar2 = DAT_00184b98;
  }
  if (DAT_00184b10 == '\0') {
LAB_0010fa4e:
    FUN_0010d390();
    return;
  }
LAB_0010fa79:
  do {
    cVar1 = FUN_0010e880();
  } while (cVar1 == '\0');
  if (DAT_00183f48 != DAT_00183f50) {
    return;
  }
  FUN_0010dfd0();
  return;
}




// Function: gnu_flush_read @ 0xfb20

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void gnu_flush_read(void)

{
  _DAT_00184150 = FUN_0010d770;
  FUN_0010f9e0();
  _DAT_00184150 = gnu_flush_read;
  return;
}




// Function: flush_write @ 0xfb50

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void flush_write(void)

{
                    /* WARNING: Could not recover jumptable at 0x0010fb5b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*_DAT_00184158)(DAT_00184b98);
  return;
}




// Function: open_archive @ 0xfb70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void open_archive(uint param_1)

{
  _DAT_00184150 = FUN_0010fb20;
  _DAT_00184158 = FUN_0010e850;
  FUN_0010f0f0();
  if (param_1 == 1) {
    DAT_00183f30 = 0;
    if (DAT_00184a40 != 0) {
      FUN_0010e330();
      FUN_0010c070();
      return;
    }
  }
  else if (((param_1 & 0xfffffffd) == 0) && (DAT_00184a40 != 0)) {
    FUN_0010f760();
    FUN_0010c070();
    return;
  }
  FUN_0010c070();
  return;
}




// Function: copy_string_unquote @ 0xfbf0

char * copy_string_unquote(void)

{
  char cVar1;
  char *__s;
  size_t sVar2;
  
  __s = (char *)FUN_0014c7a0();
  cVar1 = *__s;
  if ((cVar1 == '\"') || (cVar1 == '\'')) {
    sVar2 = strlen(__s);
    if (__s[sVar2 - 1] == cVar1) {
      memmove(__s,__s + 1,sVar2 - 2);
      __s[sVar2 - 2] = '\0';
      FUN_00123c70(__s);
      return __s;
    }
  }
  FUN_00123c70(__s);
  return __s;
}




// Function: alloc_action @ 0xfc60

void alloc_action(undefined4 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0014c760(0x18);
  if (DAT_00184268 != (long *)0x0) {
    *DAT_00184268 = lVar1;
    *(undefined4 *)(lVar1 + 8) = param_1;
    DAT_00184268 = (long *)lVar1;
    return;
  }
  *(undefined4 *)(lVar1 + 8) = param_1;
  DAT_00184270 = lVar1;
  DAT_00184268 = (long *)lVar1;
  return;
}




// Function: getwidth @ 0xfcb0

ulong getwidth(FILE *param_1)

{
  int iVar1;
  char *__nptr;
  ulong uVar2;
  long in_FS_OFFSET;
  undefined1 auStack_18 [2];
  ushort local_16;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fileno(param_1);
  iVar1 = ioctl(iVar1,0x5413,auStack_18);
  if ((iVar1 != 0) || (uVar2 = (ulong)local_16, local_16 == 0)) {
    __nptr = getenv("COLUMNS");
    if (__nptr != (char *)0x0) {
      uVar2 = strtol(__nptr,(char **)0x0,10);
      if (0 < (long)uVar2) goto LAB_0010fd0d;
    }
    uVar2 = 0x50;
  }
LAB_0010fd0d:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: format_checkpoint_string @ 0xfdf0

ulong format_checkpoint_string(FILE *param_1,ulong param_2,char *param_3,char param_4,undefined4 param_5)

{
  char cVar1;
  long lVar2;
  int iVar3;
  char *__s;
  char *pcVar4;
  size_t sVar5;
  char *__nptr;
  tm *ptVar6;
  long lVar7;
  long lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulong uVar11;
  undefined **ppuVar12;
  long in_FS_OFFSET;
  char *local_1a0;
  timeval local_178;
  undefined *local_168 [4];
  long local_148;
  long local_140;
  undefined *local_110;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 == '\0') {
    local_1a0 = (char *)dcgettext(0,"read",5);
    __s = (char *)FUN_00143090(param_5);
    if (param_3 == (char *)0x0) {
      param_3 = (char *)dcgettext(0,"Read checkpoint %u",5);
    }
  }
  else {
    local_1a0 = (char *)dcgettext(0,"write",5);
    __s = (char *)FUN_00143090(param_5);
    if (param_3 == (char *)0x0) {
      param_3 = (char *)dcgettext(0,"Write checkpoint %u",5);
    }
  }
  cVar1 = *param_3;
  do {
    if (cVar1 == '\0') {
      fflush_unlocked(param_1);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return param_2 & 0xffffffff;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (cVar1 == '%') {
      pcVar4 = param_3 + 1;
      cVar1 = param_3[1];
      if (cVar1 == '{') {
        __nptr = (char *)FUN_0010fd40();
        if (__nptr != (char *)0x0) {
          if (*pcVar4 == '*') {
            uVar11 = strtol(__nptr,(char **)0x0,10);
            goto LAB_001101de;
          }
          switch(*pcVar4) {
          case 'T':
            FUN_0010cd50();
            local_110 = &DAT_0016079c;
            lVar8 = FUN_001382c0(__nptr,&local_148,0x4644);
            lVar2 = local_140;
            lVar7 = local_148;
            if ((int)lVar8 == 0) {
              if (local_148 != 0) {
                lVar8 = 0;
                ppuVar12 = local_168;
                do {
                  ppuVar12[lVar8] = *(undefined **)(lVar2 + lVar8 * 8);
                  lVar8 = lVar8 + 1;
                } while (lVar8 != lVar7);
                if ((int)lVar8 == 3) goto LAB_001101ab;
              }
              ppuVar12 = local_168;
              lVar7 = (long)(int)lVar8;
              do {
                ppuVar12[lVar7] = (undefined *)0x0;
                lVar7 = lVar7 + 1;
              } while ((int)lVar7 != 3);
            }
            else {
              if (DAT_00184d50 != (code *)0x0) {
                (*DAT_00184d50)();
              }
              uVar9 = FUN_001354b0(&local_148);
              uVar10 = dcgettext(0,"cannot split string \'%s\': %s",5);
              error(0,0,uVar10,__nptr,uVar9);
              ppuVar12 = &PTR_DAT_00181140;
              DAT_00184d58 = 2;
            }
LAB_001101ab:
            iVar3 = FUN_0010ce80(param_1,ppuVar12,0x2c,0);
            param_2 = param_2 + (long)iVar3;
            FUN_00135420(&local_148);
            goto LAB_0010fee8;
          case 'c':
            goto switchD_0010feb8_caseD_63;
          case 'd':
            goto switchD_0010feb8_caseD_64;
          case 's':
            goto switchD_0010feb8_caseD_73;
          case 't':
            goto switchD_0011009a_caseD_74;
          case 'u':
            goto switchD_0010feb8_caseD_75;
          }
        }
switchD_0010feb8_caseD_55:
        param_2 = param_2 + 2;
        fputc_unlocked(0x25,param_1);
        fputc_unlocked((int)*pcVar4,param_1);
      }
      else if (cVar1 == '*') {
        uVar11 = FUN_0010fcb0(param_1);
LAB_001101de:
        if (param_2 < uVar11) {
          do {
            param_2 = param_2 + 1;
            fputc_unlocked(0x20,param_1);
          } while (uVar11 != param_2);
        }
      }
      else {
        switch(cVar1) {
        case 'T':
          FUN_0010cd50();
          iVar3 = FUN_0010ce80(param_1,&PTR_DAT_00181140,0x2c);
          param_2 = param_2 + (long)iVar3;
          break;
        default:
          goto switchD_0010feb8_caseD_55;
        case 'c':
switchD_0010feb8_caseD_63:
          iVar3 = format_checkpoint_string(param_1,param_2,"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r",
                               param_4,param_5);
          param_2 = param_2 + (long)iVar3;
          break;
        case 'd':
switchD_0010feb8_caseD_64:
          FUN_0010cd50();
          iVar3 = __fprintf_chk(param_1,1,&DAT_00160797);
          param_2 = param_2 + (long)iVar3;
          break;
        case 's':
switchD_0010feb8_caseD_73:
          fputs_unlocked(local_1a0,param_1);
          sVar5 = strlen(local_1a0);
          param_2 = param_2 + sVar5;
          break;
        case 't':
          __nptr = "%c";
switchD_0011009a_caseD_74:
          gettimeofday(&local_178,(__timezone_ptr_t)0x0);
          ptVar6 = localtime(&local_178.tv_sec);
          lVar7 = FUN_00141100(param_1,__nptr,ptVar6,0,(int)local_178.tv_usec * 1000);
          param_2 = param_2 + lVar7;
          break;
        case 'u':
switchD_0010feb8_caseD_75:
          fputs_unlocked(__s,param_1);
          sVar5 = strlen(__s);
          param_2 = param_2 + sVar5;
        }
      }
    }
    else {
      param_2 = param_2 + 1;
      fputc_unlocked((int)cVar1,param_1);
      pcVar4 = param_3;
      if (*param_3 == '\r') {
        DAT_001841d8 = 1;
        param_2 = 0;
      }
    }
LAB_0010fee8:
    param_3 = pcVar4 + 1;
    cVar1 = pcVar4[1];
  } while( true );
}




// Function: run_checkpoint_actions @ 0x10270

void run_checkpoint_actions(undefined1 param_1)

{
  undefined8 *puVar1;
  int iVar2;
  long in_FS_OFFSET;
  int local_44;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  for (puVar1 = DAT_00184270; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    switch(*(undefined4 *)(puVar1 + 1)) {
    case 0:
      fputc_unlocked(0x2e,DAT_00183f28);
      fflush_unlocked(DAT_00183f28);
      break;
    case 1:
      if ((DAT_001841d0 != (FILE *)0x0) ||
         (DAT_001841d0 = fopen("/dev/tty","w"), DAT_001841d0 != (FILE *)0x0)) {
        fputc_unlocked(7,DAT_001841d0);
        fflush_unlocked(DAT_001841d0);
      }
      break;
    case 2:
      iVar2 = __fprintf_chk(stderr,1,"%s: ",DAT_00184dd0);
      FUN_0010fdf0(stderr,(long)iVar2,puVar1[2],param_1,DAT_00184278);
      fputc_unlocked(10,stderr);
      break;
    case 3:
      if ((DAT_001841d0 != (FILE *)0x0) ||
         (DAT_001841d0 = fopen("/dev/tty","w"), DAT_001841d0 != (FILE *)0x0)) {
        FUN_0010fdf0(DAT_001841d0,0,puVar1[2],param_1,DAT_00184278);
      }
      break;
    case 4:
      sleep(*(uint *)(puVar1 + 2));
      break;
    case 5:
      FUN_0012bb40(puVar1[2],*DAT_00184828,DAT_00184278);
      break;
    case 6:
      FUN_0010cd50();
      FUN_0010d150();
      break;
    case 7:
      sigwait((sigset_t *)&DAT_001841e0,&local_44);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: checkpoint_compile_action @ 0x10490

void checkpoint_compile_action(char *param_1)

{
  int iVar1;
  long lVar2;
  ulong uVar3;
  char *pcVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184260 == 0) {
    sigemptyset((sigset_t *)&DAT_001841e0);
    DAT_00184260 = 1;
  }
  iVar1 = strcmp(param_1,".");
  if (iVar1 != 0) {
    iVar1 = strcmp(param_1,"dot");
    if (iVar1 != 0) {
      iVar1 = strcmp(param_1,"bell");
      if (iVar1 == 0) {
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_0010fc60(1);
          return;
        }
        goto LAB_00110741;
      }
      iVar1 = strcmp(param_1,"echo");
      if (iVar1 == 0) {
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_0010fc60(2);
          return;
        }
        goto LAB_00110741;
      }
      iVar1 = strncmp(param_1,"echo=",5);
      uVar5 = 2;
      if (iVar1 == 0) {
LAB_0011069d:
        lVar2 = FUN_0010fc60(uVar5);
        uVar5 = FUN_0010fbf0(param_1 + 5);
        *(undefined8 *)(lVar2 + 0x10) = uVar5;
      }
      else {
        iVar1 = strncmp(param_1,"exec=",5);
        if (iVar1 == 0) {
          uVar5 = 5;
          goto LAB_0011069d;
        }
        iVar1 = strncmp(param_1,"ttyout=",7);
        if (iVar1 == 0) {
          lVar2 = FUN_0010fc60(3);
          uVar5 = FUN_0010fbf0(param_1 + 7);
          *(undefined8 *)(lVar2 + 0x10) = uVar5;
        }
        else {
          iVar1 = strncmp(param_1,"sleep=",6);
          if (iVar1 != 0) {
            iVar1 = strcmp(param_1,"totals");
            if (iVar1 == 0) {
              if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_0010fc60(6);
                return;
              }
            }
            else {
              iVar1 = strncmp(param_1,"wait=",5);
              if (iVar1 != 0) {
                if (DAT_00184d50 != (code *)0x0) {
                  (*DAT_00184d50)();
                }
                pcVar4 = "%s: unknown checkpoint action";
                goto LAB_00110760;
              }
              lVar2 = FUN_0010fc60(7);
              iVar1 = FUN_0012c8a0(param_1 + 5);
              *(int *)(lVar2 + 0x10) = iVar1;
              if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
                sigaddset((sigset_t *)&DAT_001841e0,iVar1);
                return;
              }
            }
            goto LAB_00110741;
          }
          uVar3 = strtoul(param_1 + 6,&local_28,10);
          if (*local_28 != '\0') {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            pcVar4 = "%s: not a valid timeout";
LAB_00110760:
            uVar5 = dcgettext(0,pcVar4,5);
            error(0,0,uVar5,param_1);
                    /* WARNING: Subroutine does not return */
            FUN_001163d0();
          }
          lVar2 = FUN_0010fc60(4);
          *(ulong *)(lVar2 + 0x10) = uVar3;
        }
      }
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
      goto LAB_00110741;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_0010fc60(0);
    return;
  }
LAB_00110741:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: checkpoint_finish_compile @ 0x107a0

void checkpoint_finish_compile(void)

{
  if (DAT_00184260 == 0) {
    if (DAT_00184b78 == 0) {
      return;
    }
    if (DAT_00184270 != 0) {
      return;
    }
    FUN_00110490();
  }
  if (DAT_00184260 != 1) {
    return;
  }
  sigprocmask(0,(sigset_t *)&DAT_001841e0,(sigset_t *)0x0);
  if (DAT_00184b78 == 0) {
    DAT_00184b78 = 10;
  }
  DAT_00184260 = 2;
  return;
}




// Function: checkpoint_flush_actions @ 0x10820

void checkpoint_flush_actions(void)

{
  long lVar1;
  long *plVar2;
  FILE *__stream;
  
  plVar2 = DAT_00184270;
  __stream = DAT_001841d0;
  do {
    if (plVar2 == (long *)0x0) {
      DAT_001841d0 = __stream;
      return;
    }
    while ((((int)plVar2[1] != 3 || (__stream == (FILE *)0x0)) || (DAT_001841d8 == 0))) {
      plVar2 = (long *)*plVar2;
      if (plVar2 == (long *)0x0) {
        DAT_001841d0 = __stream;
        return;
      }
    }
    DAT_001841d0 = __stream;
    lVar1 = FUN_0010fcb0(__stream);
    if (lVar1 != 0) {
      for (lVar1 = lVar1 + -2; fputc_unlocked(0x20,__stream), __stream = DAT_001841d0, lVar1 != -1;
          lVar1 = lVar1 + -1) {
      }
    }
    fputc_unlocked(0xd,__stream);
    fflush_unlocked(DAT_001841d0);
    plVar2 = (long *)*plVar2;
    __stream = DAT_001841d0;
  } while( true );
}




// Function: checkpoint_run @ 0x108d0

void checkpoint_run(undefined1 param_1)

{
  if (DAT_00184b78 != 0) {
    DAT_00184278 = DAT_00184278 + 1;
    if (DAT_00184278 % DAT_00184b78 == 0) {
      FUN_00110270(param_1);
      return;
    }
  }
  return;
}




// Function: checkpoint_finish @ 0x10910

void checkpoint_finish(void)

{
  if (DAT_00184b78 == 0) {
    return;
  }
  FUN_00110820();
  if (DAT_001841d0 != (FILE *)0x0) {
    fclose(DAT_001841d0);
    return;
  }
  return;
}




// Function: process_noop @ 0x10950

undefined8 process_noop(void)

{
  return 1;
}




// Function: dumpdir_cmp @ 0x10960

int dumpdir_cmp(char *param_1,char *param_2)

{
  byte bVar1;
  int iVar2;
  ulong uVar3;
  size_t sVar4;
  char *pcVar5;
  
  do {
    bVar1 = *param_1 + 0xbc;
    uVar3 = 1L << (bVar1 & 0x3f);
    do {
      do {
        if (*param_1 == '\0') {
LAB_001109e0:
          return (int)*param_2;
        }
      } while (0x15 < bVar1);
      if ((uVar3 & 0x114000) != 0) goto LAB_001109e0;
      if ((uVar3 & 0x200400) != 0) {
        pcVar5 = strchr("YN",(int)*param_2);
        if (pcVar5 == (char *)0x0) {
          return 1;
        }
        iVar2 = strcmp(param_1 + 1,param_2 + 1);
        goto joined_r0x00110a10;
      }
    } while ((uVar3 & 1) == 0);
    iVar2 = strcmp(param_1,param_2);
joined_r0x00110a10:
    if (iVar2 != 0) {
      return 1;
    }
    sVar4 = strlen(param_1);
    param_1 = param_1 + sVar4 + 1;
    param_2 = param_2 + sVar4 + 1;
  } while( true );
}




// Function: diff_init @ 0x10af0

void diff_init(void)

{
  long in_FS_OFFSET;
  undefined1 auStack_18 [8];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00184288 = FUN_00125000(auStack_18,DAT_00184b98);
  if (DAT_00184b28 != 0) {
    FUN_0011fd80();
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: report_difference @ 0x10b50

void report_difference(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 long param_9,long param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined1 *local_d0;
  undefined1 *local_c8;
  long local_c0;
  undefined1 local_b8 [16];
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 local_58;
  undefined8 local_48;
  undefined8 local_38;
  undefined8 local_28;
  undefined8 local_18;
  
  if (in_AL != '\0') {
    local_88 = param_1;
    local_78 = param_2;
    local_68 = param_3;
    local_58 = param_4;
    local_48 = param_5;
    local_38 = param_6;
    local_28 = param_7;
    local_18 = param_8;
  }
  local_c0 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  if (param_10 != 0) {
    uVar1 = FUN_00123bb0(1,*(undefined8 *)(param_9 + 8));
    __fprintf_chk(DAT_00183f28,1,"%s: ",uVar1);
    local_d0 = &stack0x00000008;
    local_d8 = 0x10;
    local_c8 = local_b8;
    local_d4 = 0x30;
    __vfprintf_chk(DAT_00183f28,1,param_10,&local_d8);
    __fprintf_chk(DAT_00183f28,1,&DAT_00167707);
  }
  FUN_0012f7d0(1);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: process_rawdata @ 0x10c80

undefined8 process_rawdata(size_t param_1,void *param_2)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  
  sVar2 = FUN_00124960(DAT_00184290,DAT_00184288,param_1);
  if (param_1 == sVar2) {
    iVar1 = memcmp(param_2,DAT_00184288,param_1);
    uVar3 = 1;
    if (iVar1 != 0) {
      uVar3 = dcgettext(0,"Contents differ",5);
      FUN_00110b50(&DAT_00184840,uVar3);
      return 0;
    }
  }
  else {
    if (sVar2 == 0xffffffffffffffff) {
      FUN_00132820(DAT_00184848);
      FUN_00110b50(&DAT_00184840,0);
      return 0;
    }
    uVar3 = dcngettext(0,"Could only read %lu of %lu byte","Could only read %lu of %lu bytes",
                       param_1,5);
    FUN_00110b50(&DAT_00184840,uVar3,sVar2,param_1);
    uVar3 = 0;
  }
  return uVar3;
}




// Function: get_stat_data @ 0x10d70

undefined8 get_stat_data(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_001246f0();
  if (iVar1 == 0) {
    return 1;
  }
  piVar2 = __errno_location();
  if (*piVar2 == 2) {
    FUN_00132d50(param_1);
  }
  else {
    FUN_00132d30();
  }
  FUN_00110b50(&DAT_00184840,0);
  return 0;
}




// Function: diff_file @ 0x10dd0

void diff_file(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  undefined1 auVar6 [16];
  undefined1 auStack_c8 [24];
  uint local_b0;
  long local_98;
  long local_30;
  
  uVar3 = DAT_00184848;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00110d70(DAT_00184848,auStack_c8);
  if (iVar2 != 0) {
    if ((local_b0 & 0xf000) == 0x8000) {
      if (((local_b0 ^ DAT_001848b0) & 0xfff) != 0) {
        uVar4 = dcgettext(0,"Mode differs",5);
        FUN_00110b50(&DAT_00184840,uVar4);
      }
      cVar1 = FUN_0012ae90(auStack_c8,&DAT_00184898);
      if (cVar1 == '\0') {
        uVar4 = dcgettext(0,"Uid differs",5);
        FUN_00110b50(&DAT_00184840,uVar4);
      }
      cVar1 = FUN_0012aea0(auStack_c8,&DAT_00184898);
      if (cVar1 == '\0') {
        uVar4 = dcgettext(0,"Gid differs",5);
        FUN_00110b50(&DAT_00184840,uVar4);
      }
      auVar6 = FUN_0014bd20(auStack_c8);
      iVar2 = FUN_0012f7b0(auVar6._0_8_,auVar6._8_8_,DAT_00184938,DAT_00184940);
      if (iVar2 != 0) {
        uVar4 = dcgettext(0,"Mod time differs",5);
        FUN_00110b50(&DAT_00184840,uVar4);
      }
      if ((*(char *)(DAT_00184408 + 0x9c) == 'S') ||
         (pcVar5 = "Size differs", local_98 == DAT_001848c8)) {
        DAT_00184290 = __openat_2(DAT_00181234,uVar3,DAT_00184808);
        if (DAT_00184290 < 0) {
          FUN_001327c0(uVar3);
          FUN_00122bd0();
          FUN_00110b50(&DAT_00184840,0);
        }
        else {
          if (DAT_00184960 == '\0') {
            FUN_00110a20(FUN_00110c80);
          }
          else {
            FUN_0012a070(DAT_00184290,&DAT_00184840);
          }
          if ((DAT_00184b88 == 1) && (local_98 != 0)) {
            auVar6 = FUN_0014bd00(auStack_c8);
            iVar2 = FUN_00124a80(DAT_00184290,DAT_00181234,uVar3,auVar6._0_8_,auVar6._8_8_);
            if (iVar2 != 0) {
              FUN_00132dd0(uVar3);
            }
          }
          iVar2 = close(DAT_00184290);
          if (iVar2 != 0) {
            FUN_00132680(uVar3);
          }
        }
        goto LAB_00110f45;
      }
    }
    else {
      pcVar5 = "File type differs";
    }
    uVar3 = dcgettext(0,pcVar5,5);
    FUN_00110b50(&DAT_00184840,uVar3);
  }
  FUN_00122bd0();
LAB_00110f45:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_link @ 0x11090

void diff_link(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined1 auStack_148 [144];
  undefined1 local_b8 [152];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00110d70(DAT_00184848,auStack_148);
  if (iVar2 != 0) {
    iVar2 = FUN_00110d70(DAT_00184858,local_b8);
    if (iVar2 != 0) {
      cVar1 = FUN_0012aeb0(auStack_148,local_b8);
      if (cVar1 == '\0') {
        uVar3 = FUN_00123bb0(0,DAT_00184858);
        uVar4 = dcgettext(0,"Not linked to %s",5);
        FUN_00110b50(&DAT_00184840,uVar4,uVar3);
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_symlink @ 0x11160

void diff_symlink(void)

{
  int iVar1;
  size_t __n;
  size_t sVar2;
  undefined8 uVar3;
  int *piVar4;
  char *__buf;
  long in_FS_OFFSET;
  char acStack_438 [1032];
  long local_30;
  
  __buf = acStack_438;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __n = strlen(DAT_00184858);
  if (0x3ff < __n) {
    __buf = (char *)FUN_0014c5b0(__n + 1);
  }
  sVar2 = readlinkat(DAT_00181234,DAT_00184848,__buf,__n + 1);
  if ((long)sVar2 < 0) {
    piVar4 = __errno_location();
    if (*piVar4 == 2) {
      FUN_00132a80(DAT_00184848);
    }
    else {
      FUN_00132a60();
    }
    FUN_00110b50(&DAT_00184840,0);
  }
  else {
    if (__n == sVar2) {
      iVar1 = memcmp(DAT_00184858,__buf,__n);
      if (iVar1 == 0) goto LAB_00111201;
    }
    uVar3 = dcgettext(0,"Symlink differs",5);
    FUN_00110b50(&DAT_00184840,uVar3);
  }
LAB_00111201:
  if (__buf != acStack_438) {
    free(__buf);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_special @ 0x11280

void diff_special(void)

{
  int iVar1;
  undefined8 uVar2;
  uint uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  undefined1 auStack_a8 [24];
  uint local_90;
  long local_80;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00110d70(DAT_00184848,auStack_a8);
  if (iVar1 == 0) goto LAB_001112f6;
  uVar3 = local_90 & 0xf000;
  if (*(char *)(DAT_00184408 + 0x9c) == '3') {
    if (uVar3 == 0x2000) {
LAB_00111328:
      pcVar4 = "Device number differs";
      if (DAT_001848c0 == local_80) goto LAB_001112e9;
LAB_00111349:
      uVar2 = dcgettext(0,pcVar4,5);
      FUN_00110b50(&DAT_00184840,uVar2);
      goto LAB_001112f6;
    }
  }
  else if (*(char *)(DAT_00184408 + 0x9c) == '4') {
    if (uVar3 == 0x6000) goto LAB_00111328;
  }
  else if (uVar3 == 0x1000) {
LAB_001112e9:
    if (((local_90 ^ DAT_001848b0) & 0xfff) == 0) goto LAB_001112f6;
    pcVar4 = "Mode differs";
    goto LAB_00111349;
  }
  uVar2 = dcgettext(0,"File type differs",5);
  FUN_00110b50(&DAT_00184840,uVar2);
LAB_001112f6:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_dir @ 0x113b0

void diff_dir(void)

{
  int iVar1;
  undefined8 uVar2;
  char *pcVar3;
  long in_FS_OFFSET;
  undefined1 auStack_a8 [24];
  uint local_90;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00110d70(DAT_00184848,auStack_a8);
  if (iVar1 != 0) {
    if ((local_90 & 0xf000) == 0x4000) {
      if (((local_90 ^ DAT_001848b0) & 0xfff) == 0) goto LAB_00111406;
      pcVar3 = "Mode differs";
    }
    else {
      pcVar3 = "File type differs";
    }
    uVar2 = dcgettext(0,pcVar3,5);
    FUN_00110b50(&DAT_00184840,uVar2);
  }
LAB_00111406:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_multivol @ 0x11470

void diff_multivol(void)

{
  int iVar1;
  long __offset;
  __off_t _Var2;
  undefined8 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  undefined1 auStack_b8 [24];
  uint local_a0;
  long local_88;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184850 != '\0') {
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_001113b0();
      return;
    }
    goto LAB_00111640;
  }
  iVar1 = FUN_00110d70(DAT_00184848,auStack_b8);
  if (iVar1 != 0) {
    pcVar4 = "File type differs";
    if ((local_a0 & 0xf000) == 0x8000) {
      __offset = FUN_001219b0(DAT_00184408 + 0x171,0xc,5);
      if (((-1 < __offset) && (!SCARRY8(__offset,DAT_001848c8))) &&
         (local_88 == __offset + DAT_001848c8)) {
        iVar1 = __openat_2(DAT_00181234,DAT_00184848,DAT_00184808);
        if (iVar1 < 0) {
          FUN_001327c0(DAT_00184848);
          FUN_00110b50(&DAT_00184840,0);
          FUN_00122bd0();
        }
        else {
          _Var2 = lseek(iVar1,__offset,0);
          if (_Var2 < 0) {
            FUN_00132b20(DAT_00184848,__offset);
            FUN_00110b50(&DAT_00184840,0);
          }
          else {
            FUN_00110a20(FUN_00110c80);
          }
          iVar1 = close(iVar1);
          if (iVar1 != 0) {
            FUN_00132680(DAT_00184848);
          }
        }
        goto LAB_00111578;
      }
      pcVar4 = "Size differs";
    }
    uVar3 = dcgettext(0,pcVar4,5);
    FUN_00110b50(&DAT_00184840,uVar3);
    FUN_00122bd0();
  }
LAB_00111578:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00111640:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_archive @ 0x11730

void diff_archive(void)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  FUN_0010d1f0(DAT_00184408);
  if (DAT_00184a54 != 0) {
    if (DAT_00184280 != '\0') {
      uVar2 = dcgettext(0,"Verify ",5);
      __fprintf_chk(DAT_00183f28,1,uVar2);
    }
    FUN_00122990(&DAT_00184840,DAT_00184408,0xffffffffffffffff);
  }
  cVar1 = *(char *)(DAT_00184408 + 0x9c);
  switch(cVar1) {
  default:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
      cVar1 = *(char *)(DAT_00184408 + 0x9c);
    }
    uVar2 = FUN_0014aa70(DAT_00184848);
    uVar3 = dcgettext(0,"%s: Unknown file type \'%c\', diffed as normal file",5);
    error(0,0,uVar3,uVar2,(int)cVar1);
    DAT_00184d58 = 2;
  case '\0':
  case '0':
  case '7':
  case 'S':
    if (DAT_00184850 == '\0') {
      FUN_00110dd0();
      return;
    }
    break;
  case '1':
    FUN_00111090();
    return;
  case '2':
    FUN_00111160();
    return;
  case '3':
  case '4':
  case '6':
    FUN_00111280();
    return;
  case '5':
  case 'D':
    cVar1 = FUN_001201b0(&DAT_00184840);
    if (cVar1 != '\0') {
      FUN_00111650();
    }
    break;
  case 'M':
    FUN_00111470();
    return;
  case 'V':
    return;
  }
  FUN_001113b0();
  return;
}




// Function: verify_volume @ 0x118d0

void verify_volume(void)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int iVar7;
  long in_FS_OFFSET;
  undefined2 local_60 [2];
  undefined4 local_5c;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00132fd0();
  if (cVar1 == '\0') {
    cVar1 = FUN_00130840();
    if (cVar1 != '\0') {
LAB_00111a80:
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar5 = dcgettext(0,"Archive contains transformed file names.",5);
      error(0,0,uVar5);
      goto LAB_00111aaf;
    }
    FUN_0011f510();
  }
  else {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar5 = dcgettext(0,"Archive contains file names with leading prefixes removed.",5);
    error(0,0,uVar5);
    cVar1 = FUN_00130840();
    if (cVar1 != '\0') goto LAB_00111a80;
LAB_00111aaf:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar5 = dcgettext(0,"Verification may fail to locate original files.",5);
    error(0,0,uVar5);
    FUN_0011f510();
  }
  if (DAT_00184288 == 0) {
    FUN_00110af0();
  }
  fsync(DAT_00184a34);
  ioctl(DAT_00184a34,0x24b);
  local_5c = 1;
  local_60[0] = 2;
  if (DAT_00184a34 < 0x40000000) {
    iVar2 = ioctl(DAT_00184a34,0x40086d01,local_60);
  }
  else {
    iVar2 = FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_60);
  }
  if (iVar2 < 0) {
    piVar3 = __errno_location();
    if (*piVar3 == 5) {
      if (DAT_00184a34 < 0x40000000) {
        iVar2 = ioctl(DAT_00184a34,0x40086d01,local_60);
      }
      else {
        iVar2 = FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_60);
      }
      if (-1 < iVar2) goto LAB_001119b4;
      if (DAT_00184a34 < 0x40000000) goto LAB_00111c58;
LAB_0011199c:
      lVar4 = FUN_00133f90(DAT_00184a34 + -0x40000000,0,0);
    }
    else {
      if (0x3fffffff < DAT_00184a34) goto LAB_0011199c;
LAB_00111c58:
      lVar4 = lseek(DAT_00184a34,0,0);
    }
    if (lVar4 != 0) {
      FUN_00132bd0(*DAT_00184838);
      goto LAB_00111c19;
    }
  }
LAB_001119b4:
  DAT_00184280 = 1;
  DAT_00183f40 = 0;
  FUN_0010dfc0();
  while( true ) {
    iVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
    if (iVar2 == 5) {
      iVar7 = 0;
      do {
        iVar7 = iVar7 + 1;
        FUN_0010d1f0(DAT_00184408);
        iVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
      } while (iVar2 == 5);
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar5 = dcngettext(0,"VERIFY FAILURE: %d invalid header detected",
                         "VERIFY FAILURE: %d invalid headers detected",(long)iVar7,5);
      error(0,0,uVar5,iVar7);
      DAT_00184d58 = 2;
    }
    if (iVar2 == 4) break;
    if (iVar2 == 3) {
      FUN_0010d1f0(DAT_00184408);
      if (DAT_00184b51 == '\0') {
        iVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
        if (iVar2 == 3) break;
        if (((byte)DAT_00183b80 & 1) != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar5 = FUN_0010d180();
          uVar5 = FUN_00143090(uVar5,local_58);
          uVar6 = dcgettext(0,"A lone zero block at %s",5);
          error(0,0,uVar6,uVar5);
        }
      }
    }
    else {
      FUN_001215c0(DAT_00184408,&DAT_00184840,&DAT_00184400,1);
      FUN_00111730();
      FUN_0012f6f0(&DAT_00184840);
    }
  }
  DAT_00183f40 = 1;
  DAT_00184280 = 0;
LAB_00111c19:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: to_octal @ 0x11ce0

void to_octal(ulong param_1,long param_2,long param_3)

{
  do {
    param_3 = param_3 + -1;
    *(byte *)(param_2 + param_3) = ((byte)param_1 & 7) + 0x30;
    param_1 = param_1 >> 3;
  } while (param_3 != 0);
  return;
}




// Function: tar_copy_str @ 0x11d00

void tar_copy_str(long param_1,long param_2,long param_3)

{
  char cVar1;
  long lVar2;
  
  lVar2 = 0;
  do {
    cVar1 = *(char *)(param_2 + lVar2);
    *(char *)(param_1 + lVar2) = cVar1;
    if (cVar1 == '\0') {
      return;
    }
    lVar2 = lVar2 + 1;
  } while (param_3 != lVar2);
  return;
}




// Function: to_base256 @ 0x11d20

void to_base256(int param_1,ulong param_2,long param_3,long param_4)

{
  do {
    param_4 = param_4 + -1;
    *(char *)(param_3 + param_4) = (char)param_2;
    param_2 = param_2 >> 8 | (ulong)(uint)-param_1 << 0x38;
  } while (param_4 != 0);
  return;
}




// Function: file_dumpable_p @ 0x11d50

ulong file_dumpable_p(long param_1)

{
  ulong uVar1;
  long lVar2;
  uint uVar4;
  ulong uVar3;
  
  uVar1 = 1;
  uVar4 = *(uint *)(param_1 + 0x18) & 0xf000;
  if ((uVar4 != 0x4000) && (uVar1 = 0, uVar4 == 0x8000)) {
    if (DAT_00184a30 == '\0') {
      uVar1 = 1;
      if (*(long *)(param_1 + 0x30) == 0) {
        return (ulong)((*(uint *)(param_1 + 0x18) & 0x124) != 0x124);
      }
    }
    else {
      uVar1 = (ulong)DAT_00184a69;
      if ((DAT_00184a69 != 0) && (uVar1 = (ulong)DAT_00184a88, DAT_00184a88 != 0)) {
        uVar1 = *(ulong *)(param_1 + 0x30);
        uVar3 = uVar1;
        if ((long)uVar1 < 0) {
          uVar3 = uVar1 + 0x1ff;
        }
        lVar2 = (long)uVar3 >> 9;
        if (((uVar1 & 0x1ff) != 0) && (0x3fe < uVar1 + 0x1ff)) {
          lVar2 = lVar2 + 1;
        }
        return CONCAT71((int7)((ulong)lVar2 >> 8),*(long *)(param_1 + 0x40) < lVar2);
      }
    }
  }
  return uVar1;
}




// Function: split_long_name @ 0x11e00

void split_long_name(long param_1,ulong param_2)

{
  long lVar1;
  
  if (param_2 < 0x9d) {
    lVar1 = param_2 - 1;
    if (*(char *)(param_1 + -1 + param_2) == '/') {
      lVar1 = param_2 - 2;
    }
    if (lVar1 == 0) {
      return;
    }
  }
  else {
    lVar1 = 0x9b;
  }
  do {
    if (*(char *)(param_1 + lVar1) == '/') {
      return;
    }
    lVar1 = lVar1 + -1;
  } while (lVar1 != 0);
  return;
}




// Function: hash_link @ 0x11e50

ulong hash_link(ulong *param_1,ulong param_2)

{
  return (*param_1 ^ param_1[1]) % param_2;
}




// Function: compare_links @ 0x11e70

undefined8 compare_links(ulong *param_1,ulong *param_2)

{
  ulong uVar1;
  
  uVar1 = *param_1 ^ *param_2 | param_1[1] ^ param_2[1];
  return CONCAT71((int7)(uVar1 >> 8),uVar1 == 0);
}




// Function: to_chars_subst @ 0x11e90

undefined8
to_chars_subst(int param_1,int param_2,long param_3,undefined8 param_4,code *param_5,
            undefined8 param_6,long param_7,undefined8 param_8)

{
  undefined8 uVar1;
  ulong uVar2;
  long lVar3;
  undefined8 uVar4;
  ulong uVar5;
  ulong uVar6;
  long lVar7;
  char *pcVar8;
  long in_FS_OFFSET;
  uint local_bc;
  undefined1 local_b8 [32];
  undefined1 local_98;
  undefined1 local_97 [32];
  undefined1 local_77 [32];
  undefined1 local_57 [23];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    if (param_7 * 3 - 3U < 0x40) {
      pcVar8 = "0";
      uVar6 = (1L << ((char)param_7 * '\x03' - 3U & 0x3f)) - 1;
      uVar1 = FUN_00143090(uVar6,local_b8);
    }
    else {
      uVar6 = 0xffffffffffffffff;
      uVar1 = FUN_00143090(0xffffffffffffffff,local_b8);
      pcVar8 = "0";
    }
  }
  else {
    if ((ulong)((param_7 + -1) * 8) < 0x40) {
      lVar7 = 1L << ((byte)((int)(param_7 + -1) << 3) & 0x3f);
      uVar6 = lVar7 - 1;
      uVar1 = FUN_00143090(uVar6,local_b8);
    }
    else {
      uVar6 = 0xffffffffffffffff;
      lVar7 = -0x8000000000000000;
      uVar1 = FUN_00143090(0xffffffffffffffff,local_b8);
    }
    lVar7 = FUN_00143090(lVar7,local_77);
    *(undefined1 *)(lVar7 + -1) = 0x2d;
    pcVar8 = (char *)(lVar7 + -1);
  }
  if (param_1 == 0) {
    lVar7 = FUN_00143090(param_3,&local_98);
  }
  else {
    lVar7 = FUN_00143090(-param_3,local_97);
    *(undefined1 *)(lVar7 + -1) = 0x2d;
    lVar7 = lVar7 + -1;
  }
  if (param_5 == (code *)0x0) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar4 = dcgettext(0,"value %s out of %s range %s..%s",5);
    error(0,0,uVar4,lVar7,param_8,pcVar8,uVar1);
    uVar1 = 0;
    DAT_00184d58 = 2;
  }
  else {
    uVar2 = (*param_5)(&local_bc);
    local_bc = DAT_00184ba4 == 6 & local_bc;
    uVar5 = uVar6 & uVar2;
    if (local_bc != 0) {
      uVar5 = -(uVar6 & uVar2);
    }
    lVar3 = FUN_00143090(uVar5,local_57);
    if (local_bc != 0) {
      *(undefined1 *)(lVar3 + -1) = 0x2d;
      lVar3 = lVar3 + -1;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar4 = dcgettext(0,"value %s out of %s range %s..%s; substituting %s",5);
    error(0,0,uVar4,lVar7,param_8,pcVar8,uVar1,lVar3);
    uVar1 = FUN_00112150(local_bc,uVar5,param_4,0,param_6,param_7,param_8);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: to_chars @ 0x12150

undefined8
to_chars(undefined8 param_1,ulong param_2,long param_3,undefined8 param_4,undefined1 *param_5,
            long param_6,undefined8 param_7)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  undefined1 uVar4;
  uint uVar5;
  undefined8 uVar6;
  
  uVar5 = DAT_00184ba4 & 0xfffffffb;
  if ((int)param_1 == 0) {
    if ((0x3f < param_6 * 3 - 3U) || (param_2 <= (1L << ((char)param_6 * '\x03' - 3U & 0x3f)) - 1U))
    {
      param_5[param_6 + -1] = 0;
      FUN_00111ce0(param_2,param_5,param_6 + -1);
      return 1;
    }
    if (uVar5 == 2) {
      if ((0x3f < (ulong)((param_6 + -1) * 8)) ||
         (param_2 <= (1L << ((char)(param_6 + -1) * '\b' & 0x3fU)) - 1U)) {
        uVar4 = 0x80;
LAB_001121e5:
        *param_5 = uVar4;
        uVar6 = FUN_00111d20(param_1,param_2,param_5 + 1,param_6 + -1);
        return uVar6;
      }
      goto LAB_00112312;
    }
  }
  else if (uVar5 == 2) {
    lVar1 = param_6 + -1;
    if ((0x3f < (ulong)(lVar1 * 8)) || (~param_2 <= (1L << ((char)lVar1 * '\b' & 0x3fU)) - 1U)) {
      uVar4 = 0xff;
      goto LAB_001121e5;
    }
    uVar2 = param_3 * 8;
    uVar3 = param_6 * 3 - 3;
    if (uVar2 < uVar3 || uVar2 - uVar3 == 0) {
      if (DAT_001842b8 == 0) {
        DAT_001842b8 = 1;
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar6 = dcgettext(0,"Generating negative octal headers",5);
        error(0,0,uVar6);
      }
      param_5[param_6 + -1] = 0;
      if (uVar2 < 0x40) {
        param_2 = param_2 & ~(-1L << ((char)param_3 * '\b' & 0x3fU));
      }
      FUN_00111ce0(param_2,param_5,lVar1);
      return 1;
    }
    goto LAB_00112312;
  }
  param_4 = 0;
LAB_00112312:
  uVar6 = FUN_00111e90(param_1,uVar5 == 2,param_2,param_3,param_4,param_5,param_6,param_7);
  return uVar6;
}




// Function: uid_substitute @ 0x12380

int uid_substitute(undefined4 *param_1)

{
  int iVar1;
  
  iVar1 = DAT_001842b4;
  if (DAT_001842b4 != 0) {
    *param_1 = 0;
    return iVar1;
  }
  iVar1 = FUN_001269f0("nobody",&DAT_001842b4);
  if (iVar1 == 0) {
    DAT_001842b4 = 0xfffffffe;
    *param_1 = 0;
    return -2;
  }
  iVar1 = DAT_001842b4;
  *param_1 = 0;
  return iVar1;
}




// Function: gid_substitute @ 0x123e0

int gid_substitute(undefined4 *param_1)

{
  int iVar1;
  
  iVar1 = DAT_001842b0;
  if (DAT_001842b0 != 0) {
    *param_1 = 0;
    return iVar1;
  }
  iVar1 = FUN_00126aa0("nobody",&DAT_001842b0);
  if (iVar1 == 0) {
    DAT_001842b0 = 0xfffffffe;
    *param_1 = 0;
    return -2;
  }
  iVar1 = DAT_001842b0;
  *param_1 = 0;
  return iVar1;
}




// Function: write_ustar_long_name @ 0x12440

undefined8 * write_ustar_long_name(char *param_1)

{
  size_t sVar1;
  size_t __n;
  undefined8 *__dest;
  undefined8 uVar2;
  undefined8 uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  byte bVar6;
  
  bVar6 = 0;
  sVar1 = strlen(param_1);
  if (0x100 < sVar1) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = FUN_0014aa70(param_1);
    uVar3 = dcgettext(0,"%s: file name is too long (max %d); not dumped",5);
    error(0,0,uVar3,uVar2,0x100);
    DAT_00184d58 = 2;
    return (undefined8 *)0x0;
  }
  __n = FUN_00111e00(param_1,sVar1);
  if ((__n != 0) && (sVar1 = (sVar1 - 1) - __n, sVar1 - 1 < 100)) {
    __dest = (undefined8 *)FUN_0010e130();
    *__dest = 0;
    __dest[0x3f] = 0;
    puVar5 = (undefined8 *)((ulong)(__dest + 1) & 0xfffffffffffffff8);
    for (uVar4 = (ulong)(((int)__dest -
                         (int)(undefined8 *)((ulong)(__dest + 1) & 0xfffffffffffffff8)) + 0x200U >>
                        3); uVar4 != 0; uVar4 = uVar4 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + (ulong)bVar6 * -2 + 1;
    }
    memcpy((void *)((long)__dest + 0x159),param_1,__n);
    memcpy(__dest,param_1 + __n + 1,sVar1);
    return __dest;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = FUN_0014aa70(param_1);
  uVar3 = dcgettext(0,"%s: file name is too long (cannot be split); not dumped",5);
  error(0,0,uVar3,uVar2);
  DAT_00184d58 = 2;
  return (undefined8 *)0x0;
}




// Function: open_failure_recover @ 0x125b0

ulong open_failure_recover(long param_1)

{
  int *piVar1;
  long lVar2;
  long lVar3;
  ulong uVar4;
  
  piVar1 = __errno_location();
  uVar4 = CONCAT71((int7)((ulong)piVar1 >> 8),*piVar1 == 0x18 && param_1 != 0) & 0xffffffff;
  if (*piVar1 != 0x18 || param_1 == 0) {
    return uVar4;
  }
  if (*(long *)(param_1 + 0x198) != 0) {
    lVar2 = *(long *)(*(long *)(param_1 + 0x198) + 0x198);
    while (lVar2 != 0) {
      while (lVar3 = lVar2, lVar2 = *(long *)(lVar3 + 0x198), 0 < *(int *)(lVar3 + 0x1a8)) {
        if ((lVar2 == 0) || (*(int *)(lVar2 + 0x1a8) < 1)) {
          FUN_0012f670(lVar3);
          return uVar4;
        }
      }
    }
  }
  return 0;
}




// Function: unknown_file_error @ 0x12640

void unknown_file_error(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  if (((byte)DAT_00183b80 & 0x20) == 0) {
    if (DAT_00184b52 != '\0') {
      return;
    }
    FUN_0012f7d0(2);
    return;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = FUN_0014aa70(param_1);
  uVar2 = dcgettext(0,"%s: Unknown file type; file ignored",5);
  error(0,0,uVar2,uVar1);
  if (DAT_00184b52 != '\0') {
    return;
  }
  FUN_0012f7d0(2);
  return;
}




// Function: ensure_slash @ 0x126e0

void ensure_slash(long *param_1)

{
  char *__s;
  size_t sVar1;
  long lVar2;
  char *pcVar3;
  size_t sVar4;
  long lVar5;
  
  __s = (char *)*param_1;
  sVar1 = strlen(__s);
  do {
    sVar4 = sVar1;
    if (sVar4 == 0) {
      lVar5 = 1;
      pcVar3 = __s;
      goto LAB_0011271c;
    }
    sVar1 = sVar4 - 1;
  } while (__s[sVar4 - 1] == '/');
  pcVar3 = __s + sVar4;
  lVar5 = sVar4 + 1;
LAB_0011271c:
  if (*pcVar3 != '/') {
    lVar2 = FUN_0014c610(__s,sVar4 + 2);
    *param_1 = lVar2;
    pcVar3 = (char *)(lVar2 + sVar4);
  }
  *pcVar3 = '/';
  *(undefined1 *)(*param_1 + lVar5) = 0;
  return;
}




// Function: file_count_links @ 0x12760

void file_count_links(undefined8 *param_1)

{
  char *__ptr;
  undefined8 uVar1;
  size_t sVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  long in_FS_OFFSET;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00184b68 == '\0') && (DAT_001842c8 < (ulong)param_1[0xd])) {
    local_28 = (char *)0x0;
    uVar1 = FUN_00133020(*param_1,1,DAT_00184b92);
    FUN_00123be0(&local_28,uVar1);
    FUN_00130830(&local_28,2);
    sVar2 = strlen(local_28);
    puVar3 = (undefined8 *)FUN_0014c5b0(sVar2 + 0x19);
    __ptr = local_28;
    puVar3[1] = param_1[0xc];
    *puVar3 = param_1[0xb];
    puVar3[2] = param_1[0xd];
    strcpy((char *)(puVar3 + 3),local_28);
    free(__ptr);
    if (DAT_001842c0 == 0) {
      DAT_001842c0 = FUN_00141b30(0,0,FUN_00111e50,FUN_00111e70,0);
      if (DAT_001842c0 == 0) goto LAB_0011287f;
    }
    puVar4 = (undefined8 *)FUN_00142140(DAT_001842c0,puVar3);
    if (puVar4 == (undefined8 *)0x0) {
LAB_0011287f:
                    /* WARNING: Subroutine does not return */
      FUN_00116420();
    }
    if (puVar3 != puVar4) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    puVar3[2] = puVar3[2] + -1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_short_name @ 0x12990

undefined8 * write_short_name(long param_1)

{
  undefined8 *puVar1;
  ulong uVar2;
  undefined8 *puVar3;
  byte bVar4;
  
  bVar4 = 0;
  puVar1 = (undefined8 *)FUN_0010e130();
  *puVar1 = 0;
  puVar1[0x3f] = 0;
  puVar3 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
  for (uVar2 = (ulong)(((int)puVar1 - (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + (ulong)bVar4 * -2 + 1;
  }
  FUN_00112970(puVar1,*(undefined8 *)(param_1 + 8));
  return puVar1;
}




// Function: add_exclusion_tag @ 0x12a70

void add_exclusion_tag(char *param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  size_t sVar3;
  
  puVar2 = (undefined8 *)FUN_0014c5b0(0x28);
  uVar1 = DAT_001842d0;
  *puVar2 = param_1;
  *(undefined4 *)(puVar2 + 2) = param_2;
  puVar2[3] = param_3;
  puVar2[4] = uVar1;
  sVar3 = strlen(param_1);
  DAT_001842d0 = puVar2;
  puVar2[1] = sVar3;
  return;
}




// Function: exclusion_tag_warning @ 0x12ad0

void exclusion_tag_warning(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if ((DAT_00184a54 != 0) && (((byte)DAT_00183b80 & 4) != 0)) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar1 = FUN_0014a870(1,param_2);
    uVar2 = FUN_0014aa70(param_1);
    uVar3 = dcgettext(0,"%s: contains a cache directory tag %s; %s",5);
    error(0,0,uVar3,uVar2,uVar1,param_3);
    return;
  }
  return;
}




// Function: cachedir_file_p @ 0x12b70

undefined8 cachedir_file_p(int param_1)

{
  int iVar1;
  ssize_t sVar2;
  undefined4 extraout_var;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 auStack_48 [56];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sVar2 = read(param_1,auStack_48,0x2b);
  uVar3 = 0;
  if (sVar2 == 0x2b) {
    iVar1 = memcmp(auStack_48,"Signature: 8a477f597d28d172789f06886806bc55",0x2b);
    uVar3 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: off_to_chars @ 0x12be0

void off_to_chars(ulong param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00112150(param_1 >> 0x3f,param_1,8,0,param_2,param_3,"off_t");
  return;
}




// Function: time_to_chars @ 0x12c10

void time_to_chars(ulong param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00112150(param_1 >> 0x3f,param_1,8,0,param_2,param_3,"time_t");
  return;
}




// Function: write_eot @ 0x12c40

void write_eot(void)

{
  undefined8 *puVar1;
  void *__s;
  size_t __n;
  ulong uVar2;
  undefined8 *puVar3;
  byte bVar4;
  
  bVar4 = 0;
  puVar1 = (undefined8 *)FUN_0010e130();
  *puVar1 = 0;
  puVar1[0x3f] = 0;
  puVar3 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
  for (uVar2 = (ulong)(((int)puVar1 - (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + (ulong)bVar4 * -2 + 1;
  }
  FUN_0010d1f0(puVar1);
  __s = (void *)FUN_0010e130();
  __n = FUN_0010d2b0(__s);
  memset(__s,0,__n);
  FUN_0010d1f0(__s);
  return;
}




// Function: start_private_header @ 0x12cb0

undefined8 * start_private_header(undefined8 param_1,undefined8 param_2,ulong param_3)

{
  undefined8 *puVar1;
  ulong uVar2;
  undefined8 *puVar3;
  byte bVar4;
  
  bVar4 = 0;
  puVar1 = (undefined8 *)FUN_0010e130();
  *puVar1 = 0;
  puVar1[0x3f] = 0;
  puVar3 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
  for (uVar2 = (ulong)(((int)puVar1 - (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + (ulong)bVar4 * -2 + 1;
  }
  FUN_00112970(puVar1,param_1);
  FUN_00112be0(param_2,(long)puVar1 + 0x7c,0xc);
  uVar2 = 0x1ffffffff;
  if (param_3 < 0x200000000) {
    uVar2 = param_3;
  }
  if ((long)param_3 < 0) {
    uVar2 = 0;
  }
  FUN_00112c10(uVar2,puVar1 + 0x11,0xc);
  FUN_001129e0(0x81a4,(long)puVar1 + 100);
  FUN_001128b0(0,(long)puVar1 + 0x6c);
  FUN_00112940(0,(long)puVar1 + 0x74);
  *(undefined4 *)((long)puVar1 + 0x101) = 0x61747375;
  *(undefined2 *)((long)puVar1 + 0x105) = 0x72;
  *(undefined2 *)((long)puVar1 + 0x107) = 0x3030;
  return puVar1;
}




// Function: write_extended @ 0x12da0

undefined8 * write_extended(char param_1,long param_2,undefined8 *param_3)

{
  void *__ptr;
  long lVar1;
  ulong uVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  byte bVar7;
  undefined8 local_238 [63];
  undefined8 local_40;
  long local_30;
  
  bVar7 = 0;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(long *)(param_2 + 0x178) == 0) && (*(long *)(param_2 + 0x168) != 0)) {
    FUN_0011cc10(param_2 + 0x168);
    puVar4 = local_238;
    for (lVar1 = 0x40; lVar1 != 0; lVar1 = lVar1 + -1) {
      *puVar4 = *param_3;
      param_3 = param_3 + (ulong)bVar7 * -2 + 1;
      puVar4 = puVar4 + (ulong)bVar7 * -2 + 1;
    }
    if (param_1 == '\0') {
      __ptr = (void *)FUN_0011c460(param_2);
      if (DAT_00184af0 == 0) {
        uVar3 = *(undefined8 *)(param_2 + 0xb0);
        uVar5 = 0x78;
      }
      else {
        uVar5 = 0x78;
        uVar3 = DAT_00184ae0;
      }
    }
    else {
      __ptr = (void *)FUN_0011c4b0();
      uVar5 = 0x67;
      uVar3 = DAT_00184a20;
    }
    FUN_0011ccf0(uVar5,__ptr,uVar3,param_2 + 0x168);
    free(__ptr);
    param_3 = (undefined8 *)FUN_0010e130();
    *param_3 = local_238[0];
    lVar1 = (long)param_3 - (long)((ulong)(param_3 + 1) & 0xfffffffffffffff8);
    param_3[0x3f] = local_40;
    puVar4 = (undefined8 *)((long)local_238 - lVar1);
    puVar6 = (undefined8 *)((ulong)(param_3 + 1) & 0xfffffffffffffff8);
    for (uVar2 = (ulong)((int)lVar1 + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
      *puVar6 = *puVar4;
      puVar4 = puVar4 + (ulong)bVar7 * -2 + 1;
      puVar6 = puVar6 + (ulong)bVar7 * -2 + 1;
    }
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_3;
}




// Function: simple_finish_header @ 0x12f00

void simple_finish_header(byte *param_1)

{
  byte bVar1;
  byte *pbVar2;
  int iVar3;
  
  iVar3 = 0;
  param_1[0x94] = 0x20;
  param_1[0x95] = 0x20;
  param_1[0x96] = 0x20;
  param_1[0x97] = 0x20;
  param_1[0x98] = 0x20;
  param_1[0x99] = 0x20;
  param_1[0x9a] = 0x20;
  param_1[0x9b] = 0x20;
  pbVar2 = param_1;
  do {
    bVar1 = *pbVar2;
    pbVar2 = pbVar2 + 1;
    iVar3 = iVar3 + (uint)bVar1;
  } while (param_1 + 0x200 != pbVar2);
  FUN_00112a40((long)iVar3,param_1 + 0x94,iVar3);
  FUN_0010d1f0(param_1);
  return;
}




// Function: finish_header @ 0x12f60

void finish_header(undefined8 param_1,long param_2)

{
  uint uVar1;
  undefined8 uVar2;
  
  if (((DAT_00184a54 != 0) && (*(byte *)(param_2 + 0x9c) != 0x4b)) &&
     ((uVar1 = *(byte *)(param_2 + 0x9c) - 0x4c, 0x2c < (byte)uVar1 ||
      ((0x100008000001U >> ((ulong)uVar1 & 0x3f) & 1) == 0)))) {
    DAT_00184400 = DAT_00184ba4;
    FUN_00122990(param_1,param_2);
  }
  uVar2 = FUN_00112da0(0,param_1,param_2);
  FUN_00112f00(uVar2);
  return;
}




// Function: write_gnu_long_link @ 0x12fd0

void write_gnu_long_link(undefined8 param_1,char *param_2,undefined1 param_3)

{
  ulong uVar1;
  long lVar2;
  void *__dest;
  ulong __n;
  ulong __n_00;
  char *__src;
  
  uVar1 = strlen(param_2);
  __n_00 = uVar1 + 1;
  lVar2 = FUN_00112cb0("././@LongLink",__n_00,0);
  if (DAT_00184ad2 == '\0') {
    if (DAT_001842a8 == 0) {
      FUN_001268d0(0,&DAT_001842a8);
      FUN_00126960(0,&DAT_001842a0);
    }
    FUN_00112890(DAT_001842a8,lVar2 + 0x109);
    FUN_00112890(DAT_001842a0,lVar2 + 0x129);
  }
  *(undefined1 *)(lVar2 + 0x9c) = param_3;
  *(undefined8 *)(lVar2 + 0x101) = 0x20207261747375;
  FUN_00112f60(param_1,lVar2,0xffffffffffffffff);
  __dest = (void *)FUN_0010e130();
  __n = FUN_0010d2b0(__dest);
  __src = param_2;
  if (__n < __n_00) {
    do {
      __src = param_2 + __n;
      memcpy(__dest,param_2,__n);
      __n_00 = __n_00 - __n;
      FUN_0010d1f0((long)__dest + (__n - 1 & 0xfffffffffffffe00));
      __dest = (void *)FUN_0010e130();
      __n = FUN_0010d2b0(__dest);
      param_2 = __src;
    } while (__n < __n_00);
    uVar1 = __n_00 - 1;
  }
  memcpy(__dest,__src,__n_00);
  memset((void *)((long)__dest + __n_00),0,__n - __n_00);
  FUN_0010d1f0((long)__dest + (uVar1 & 0xfffffffffffffe00));
  return;
}




// Function: write_long_name @ 0x13140

undefined8 write_long_name(long param_1)

{
  undefined8 uVar1;
  size_t sVar2;
  undefined8 uVar3;
  char *__s;
  
  switch(DAT_00184ba4) {
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    __s = *(char **)(param_1 + 8);
    sVar2 = strlen(__s);
    if (99 < sVar2) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
        __s = *(char **)(param_1 + 8);
      }
      uVar1 = FUN_0014aa70(__s);
      uVar3 = dcgettext(0,"%s: file name is too long (max %d); not dumped",5);
      error(0,0,uVar3,uVar1,99);
      DAT_00184d58 = 2;
      return 0;
    }
    break;
  case 2:
  case 6:
    FUN_00112fd0(param_1,*(undefined8 *)(param_1 + 8),0x4c);
    break;
  case 3:
  case 5:
    uVar1 = FUN_00112440(*(undefined8 *)(param_1 + 8));
    return uVar1;
  case 4:
    FUN_0011ca30("path",param_1,0);
    uVar1 = FUN_00112990(param_1);
    return uVar1;
  }
  uVar1 = FUN_00112990(param_1);
  return uVar1;
}




// Function: write_header_name @ 0x13240

void write_header_name(long param_1)

{
  char cVar1;
  size_t sVar2;
  char *__s;
  bool bVar3;
  
  __s = *(char **)(param_1 + 8);
  if (DAT_00184ba4 == 4) {
    cVar1 = thunk_FUN_00131458();
    if (cVar1 == '\0') {
      FUN_0011ca30("path",param_1,0);
      FUN_00112990(param_1);
      return;
    }
    __s = *(char **)(param_1 + 8);
  }
  bVar3 = DAT_00184ba4 != 2;
  sVar2 = strlen(__s);
  if ((ulong)bVar3 + 99 < sVar2) {
    FUN_00113140();
    return;
  }
  FUN_00112990(param_1);
  return;
}




// Function: start_header @ 0x132d0

long start_header(long param_1)

{
  long lVar1;
  ulonglong __dev;
  char cVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  long lVar7;
  undefined8 uVar8;
  size_t sVar9;
  char *pcVar10;
  ulong uVar11;
  long in_FS_OFFSET;
  long local_68;
  long local_60;
  ulong local_58;
  long lStack_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_68 = 0;
  local_60 = 0;
  lVar7 = FUN_00113240();
  if (lVar7 != 0) {
    FUN_001239d0(*(undefined4 *)(param_1 + 0x74),param_1 + 0x74,&local_68);
    FUN_00123aa0(*(undefined4 *)(param_1 + 0x78),param_1 + 0x78,&local_60);
    if (DAT_00184b18 == 0) {
      uVar3 = *(uint *)(param_1 + 0x70);
    }
    else {
      uVar4 = *(uint *)(param_1 + 0x70);
      uVar3 = FUN_00143970(uVar4,(uVar4 & 0xf000) == 0x4000,DAT_00184b14,DAT_00184b18,0);
      uVar3 = uVar4 & 0xfffff000 | uVar3;
      *(uint *)(param_1 + 0x70) = uVar3;
    }
    if ((DAT_00184ba4 & 0xfffffffd) == 1) {
      FUN_001129e0(uVar3 & 0xfff,lVar7 + 100);
    }
    else {
      FUN_001129e0(uVar3,lVar7 + 100);
    }
    uVar3 = *(uint *)(param_1 + 0x74);
    if ((DAT_00184ba4 == 4) && (0x1fffff < uVar3)) {
      FUN_0011ca30("uid",param_1,0);
      uVar3 = 0;
    }
    cVar2 = FUN_001128b0(uVar3,lVar7 + 0x6c);
    if (cVar2 != '\0') {
      uVar3 = *(uint *)(param_1 + 0x78);
      if ((DAT_00184ba4 == 4) && (0x1fffff < uVar3)) {
        FUN_0011ca30(&DAT_00167634,param_1,0);
        uVar3 = 0;
      }
      cVar2 = FUN_00112940(uVar3,lVar7 + 0x74);
      if (cVar2 != '\0') {
        uVar11 = *(ulong *)(param_1 + 0x88);
        if ((DAT_00184ba4 == 4) && (uVar11 >> 0x21 != 0)) {
          FUN_0011ca30("size",param_1,0);
          uVar11 = 0;
        }
        cVar2 = FUN_00112be0(uVar11,lVar7 + 0x7c,0xc);
        lVar1 = DAT_00184ae8;
        uVar11 = DAT_00184ae0;
        if (cVar2 != '\0') {
          if (DAT_00184af0 == 1) {
            local_58 = DAT_00184ae0;
            lStack_50 = DAT_00184ae8;
          }
          else if (DAT_00184af0 == 2) {
            iVar6 = FUN_0014be80(*(undefined8 *)(param_1 + 0xf8),*(undefined8 *)(param_1 + 0x100),
                                 DAT_00184ae0,DAT_00184ae8);
            if (iVar6 < 1) {
              local_58 = *(ulong *)(param_1 + 0xf8);
              lStack_50 = *(long *)(param_1 + 0x100);
            }
            else {
              local_58 = uVar11;
              lStack_50 = lVar1;
            }
          }
          else if (DAT_00184af0 == 0) {
            local_58 = *(ulong *)(param_1 + 0xf8);
            lStack_50 = *(long *)(param_1 + 0x100);
          }
          if ((DAT_00184ba4 == 4) &&
             (((local_58 >> 0x21 != 0 || (lStack_50 != 0)) &&
              (FUN_0011ca30("mtime",param_1,&local_58), local_58 >> 0x21 != 0)))) {
            local_58 = 0;
          }
          cVar2 = FUN_00112c10(local_58,lVar7 + 0x88,0xc);
          uVar3 = DAT_00184ba4;
          if (cVar2 != '\0') {
            if ((*(uint *)(param_1 + 0x70) & 0xb000) == 0x2000) {
              __dev = *(ulonglong *)(param_1 + 0x80);
              uVar4 = gnu_dev_major(__dev);
              uVar5 = gnu_dev_minor(__dev);
              if ((0x1fffff < uVar4) && (uVar3 == 4)) {
                uVar4 = 0;
                FUN_0011ca30("devmajor",param_1,0);
              }
              cVar2 = FUN_00112910(uVar4,lVar7 + 0x149);
              if (cVar2 != '\0') {
                if ((DAT_00184ba4 == 4) && (0x1fffff < uVar5)) {
                  FUN_0011ca30("devminor",param_1,0);
                  uVar5 = 0;
                }
                cVar2 = FUN_001128e0(uVar5,lVar7 + 0x151);
joined_r0x0011370f:
                if (cVar2 != '\0') goto LAB_0011348d;
              }
            }
            else {
              if ((DAT_00184ba4 & 0xfffffffb) == 2) {
LAB_0011348d:
                if (DAT_00184ba4 == 4) {
                  FUN_0011ca30("atime",param_1,0);
                  FUN_0011ca30("ctime",param_1,0);
                }
                else if ((DAT_00184b50 != '\0') && ((DAT_00184ba4 & 0xfffffffb) == 2)) {
                  FUN_00112c10(*(undefined8 *)(param_1 + 0xe8),lVar7 + 0x159,0xc);
                  FUN_00112c10(*(undefined8 *)(param_1 + 0x108),lVar7 + 0x165,0xc);
                }
                uVar3 = DAT_00184ba4;
                if (DAT_00184ba4 == 1) {
                  *(undefined1 *)(lVar7 + 0x9c) = 0;
                  goto LAB_00113653;
                }
                *(undefined1 *)(lVar7 + 0x9c) = 0x30;
                if (uVar3 < 5) {
                  if (uVar3 < 3) {
                    if (uVar3 != 2) goto LAB_00113745;
                    goto LAB_00113722;
                  }
                  *(undefined4 *)(lVar7 + 0x101) = 0x61747375;
                  *(undefined2 *)(lVar7 + 0x105) = 0x72;
                  *(undefined2 *)(lVar7 + 0x107) = 0x3030;
                }
                else {
                  if (uVar3 != 6) {
LAB_00113745:
                    /* WARNING: Subroutine does not return */
                    abort();
                  }
LAB_00113722:
                  *(undefined8 *)(lVar7 + 0x101) = 0x20207261747375;
                }
                if (DAT_00184ad2 == '\0') {
                  if (local_68 == 0) {
                    FUN_001268d0(*(undefined4 *)(param_1 + 0x74),param_1 + 0x20);
                  }
                  else {
                    uVar8 = FUN_0014c7a0();
                    *(undefined8 *)(param_1 + 0x20) = uVar8;
                  }
                  if (local_60 == 0) {
                    FUN_00126960(*(undefined4 *)(param_1 + 0x78),param_1 + 0x28);
                  }
                  else {
                    uVar8 = FUN_0014c7a0();
                    *(undefined8 *)(param_1 + 0x28) = uVar8;
                  }
                  pcVar10 = *(char **)(param_1 + 0x20);
                  if (DAT_00184ba4 == 4) {
                    sVar9 = strlen(pcVar10);
                    if ((sVar9 < 0x21) && (cVar2 = thunk_FUN_00131458(pcVar10), cVar2 != '\0')) {
                      pcVar10 = *(char **)(param_1 + 0x20);
                    }
                    else {
                      FUN_0011ca30("uname",param_1,0);
                      pcVar10 = *(char **)(param_1 + 0x20);
                    }
                  }
                  FUN_00112890(pcVar10,lVar7 + 0x109);
                  if (DAT_00184ba4 == 4) {
                    pcVar10 = *(char **)(param_1 + 0x28);
                    sVar9 = strlen(pcVar10);
                    if ((0x20 < sVar9) || (cVar2 = thunk_FUN_00131458(pcVar10), cVar2 == '\0')) {
                      FUN_0011ca30("gname",param_1,0);
                    }
                  }
                  FUN_00112890(*(undefined8 *)(param_1 + 0x28),lVar7 + 0x129);
                  uVar3 = DAT_00184ba4;
                }
                if (uVar3 == 4) {
                  if (0 < DAT_00184a9c) {
                    if (*(long *)(param_1 + 0x38) != 0) {
                      FUN_0011ca30("SCHILY.acl.access",param_1,0);
                    }
                    if (*(long *)(param_1 + 0x48) != 0) {
                      FUN_0011ca30("SCHILY.acl.default",param_1,0);
                    }
                  }
                  if ((0 < DAT_00184aa0) && (*(long *)(param_1 + 0x30) != 0)) {
                    FUN_0011ca30("RHT.security.selinux",param_1,0);
                  }
                  if (0 < DAT_00184a98) {
                    local_58 = 0;
                    lVar1 = *(long *)(param_1 + 0x160);
                    if (*(long *)(param_1 + 0x158) != 0) {
                      do {
                        FUN_0011ca30(*(undefined8 *)(lVar1 + local_58 * 0x18),param_1,&local_58);
                        local_58 = local_58 + 1;
                      } while (local_58 < *(ulong *)(param_1 + 0x158));
                    }
                  }
                }
                goto LAB_00113653;
              }
              cVar2 = FUN_00112910(0,lVar7 + 0x149);
              if (cVar2 != '\0') {
                cVar2 = FUN_001128e0(0,lVar7 + 0x151);
                goto joined_r0x0011370f;
              }
            }
          }
        }
      }
    }
  }
  lVar7 = 0;
LAB_00113653:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_long_link @ 0x13a40

void write_long_link(long param_1)

{
  ulong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (DAT_00184ba4 < 7) {
    uVar1 = 1L << ((byte)DAT_00184ba4 & 0x3f);
    if ((uVar1 & 0x2a) != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = FUN_0014aa70(*(undefined8 *)(param_1 + 0x18));
      uVar3 = dcgettext(0,"%s: link name is too long; not dumped",5);
      error(0,0,uVar3,uVar2);
      DAT_00184d58 = 2;
      return;
    }
    if ((uVar1 & 0x44) != 0) {
      FUN_00112fd0(param_1,*(undefined8 *)(param_1 + 0x18),0x4b);
      return;
    }
    if (DAT_00184ba4 == 4) {
      FUN_0011ca30("linkpath",param_1,0);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: dump_hard_link @ 0x13b00

char dump_hard_link(undefined8 *param_1)

{
  char cVar1;
  long lVar2;
  char *__s;
  undefined8 uVar3;
  size_t sVar4;
  long in_FS_OFFSET;
  bool bVar5;
  undefined8 local_58;
  undefined8 local_50;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = '\0';
  if ((DAT_001842c0 != 0) &&
     ((DAT_001842c8 < (ulong)param_1[0xd] || (cVar1 = DAT_00184ab8, DAT_00184ab8 != '\0')))) {
    local_50 = param_1[0xc];
    local_58 = param_1[0xb];
    lVar2 = FUN_001418d0(DAT_001842c0,&local_58);
    cVar1 = '\0';
    if (lVar2 != 0) {
      __s = (char *)FUN_00133020(lVar2 + 0x18,1,DAT_00184b92);
      if (*(long *)(lVar2 + 0x10) != 0) {
        *(long *)(lVar2 + 0x10) = *(long *)(lVar2 + 0x10) + -1;
      }
      uVar3 = FUN_0010d180();
      FUN_00123be0(param_1 + 3,__s);
      bVar5 = DAT_00184ba4 != 2;
      sVar4 = strlen(__s);
      if ((ulong)bVar5 + 99 < sVar4) {
        FUN_00113a40(param_1);
      }
      param_1[0x11] = 0;
      lVar2 = FUN_001132d0(param_1);
      cVar1 = '\0';
      if (lVar2 != 0) {
        FUN_00111d00(lVar2 + 0x9d,__s,100);
        *(undefined1 *)(lVar2 + 0x9c) = 0x31;
        FUN_00112f60(param_1,lVar2,uVar3);
        cVar1 = DAT_00184ab8;
        if (DAT_00184ab8 == '\0') {
          cVar1 = '\x01';
        }
        else {
          FUN_00130b70(*param_1,0);
        }
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pad_archive @ 0x13c80

void pad_archive(long param_1)

{
  undefined8 *puVar1;
  ulong uVar2;
  long lVar3;
  undefined8 *puVar4;
  long lVar5;
  byte bVar6;
  
  bVar6 = 0;
  if (0 < param_1) {
    lVar5 = param_1 + -0x200;
    lVar3 = lVar5 - (param_1 - 1U & 0xfffffffffffffe00);
    while( true ) {
      puVar1 = (undefined8 *)FUN_0010e130();
      *puVar1 = 0;
      puVar1[0x3f] = 0;
      puVar4 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
      for (uVar2 = (ulong)(((int)puVar1 -
                           (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8)) + 0x200U
                          >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar4 = 0;
        puVar4 = puVar4 + (ulong)bVar6 * -2 + 1;
      }
      FUN_0010d1f0(puVar1);
      if (lVar5 == lVar3) break;
      lVar5 = lVar5 + -0x200;
    }
    return;
  }
  return;
}




// Function: dump_regular_file @ 0x13d10

undefined8 dump_regular_file(int param_1,undefined8 *param_2)

{
  uint uVar1;
  undefined8 uVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  ulong uVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  byte bVar11;
  undefined1 local_58 [24];
  long local_40;
  
  bVar11 = 0;
  uVar10 = param_2[0x11];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_0010d180();
  lVar3 = FUN_001132d0(param_2);
  if (lVar3 == 0) {
    uVar2 = 2;
  }
  else {
    FUN_00112f60(param_2,lVar3,uVar2);
    FUN_0010cc00(param_2[1],param_2[0x11],param_2[0x11]);
LAB_00113d78:
    do {
      uVar9 = uVar10;
      if ((long)uVar9 < 1) {
LAB_00113dbf:
        uVar2 = 0;
        goto LAB_00113dc1;
      }
      lVar3 = FUN_0010e130();
      uVar4 = FUN_0010d2b0(lVar3);
      if (uVar9 < uVar4) {
        if ((uVar9 & 0x1ff) != 0) {
          uVar1 = 0x200 - ((uint)uVar9 & 0x1ff);
          puVar8 = (undefined8 *)(lVar3 + uVar9);
          if (uVar1 < 8) {
            if ((uVar1 & 4) == 0) {
              if ((uVar1 != 0) && (*(undefined1 *)puVar8 = 0, (uVar1 & 2) != 0)) {
                *(undefined2 *)((long)puVar8 + ((ulong)uVar1 - 2)) = 0;
              }
            }
            else {
              *(undefined4 *)puVar8 = 0;
              *(undefined4 *)((long)puVar8 + ((ulong)uVar1 - 4)) = 0;
            }
          }
          else {
            *puVar8 = 0;
            *(undefined8 *)((long)puVar8 + ((ulong)uVar1 - 8)) = 0;
            uVar10 = (ulong)(uVar1 + ((int)puVar8 -
                                     (int)(undefined8 *)((ulong)(puVar8 + 1) & 0xfffffffffffffff8))
                            >> 3);
            puVar8 = (undefined8 *)((ulong)(puVar8 + 1) & 0xfffffffffffffff8);
            for (; uVar10 != 0; uVar10 = uVar10 - 1) {
              *puVar8 = 0;
              puVar8 = puVar8 + (ulong)bVar11 * -2 + 1;
            }
          }
        }
        uVar4 = uVar9;
        if (param_1 < 1) {
          FUN_0010d1f0(lVar3 + (uVar9 - 1 & 0xfffffffffffffe00));
          goto LAB_00113dbf;
        }
      }
      else if (param_1 < 1) {
        FUN_0010d1f0(lVar3 + (uVar4 - 1 & 0xfffffffffffffe00));
        uVar10 = uVar9 - uVar4;
        goto LAB_00113d78;
      }
      uVar5 = FUN_00124960(param_1,lVar3,uVar4);
      if (uVar5 == 0xffffffffffffffff) {
        FUN_00124e00(*param_2,param_2[0x11] - uVar9,uVar4);
        FUN_00113c80(uVar9);
        uVar2 = 1;
        goto LAB_00113dc1;
      }
      uVar10 = uVar9 - uVar5;
      FUN_0010d1f0((uVar4 - 1 & 0xfffffffffffffe00) + lVar3);
    } while (uVar5 == uVar4);
    memset((void *)(lVar3 + uVar5),0,uVar4 - uVar5);
    if (((byte)DAT_00183b80 & 0x80) != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = FUN_00143090(uVar10,local_58);
      uVar6 = FUN_0014aa70(*param_2);
      uVar7 = dcngettext(0,"%s: File shrank by %s byte; padding with zeros",
                         "%s: File shrank by %s bytes; padding with zeros",uVar10,5);
      error(0,0,uVar7,uVar6,uVar2);
    }
    if (DAT_00184b52 == '\0') {
      FUN_0012f7d0(1);
    }
    FUN_00113c80(uVar9 - uVar4);
    uVar2 = 1;
  }
LAB_00113dc1:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: get_directory_entries @ 0x13fe0

undefined8 get_directory_entries(long param_1)

{
  char cVar1;
  DIR *pDVar2;
  undefined8 uVar3;
  
  do {
    pDVar2 = fdopendir(*(int *)(param_1 + 0x1a8));
    *(DIR **)(param_1 + 0x1a0) = pDVar2;
    if (pDVar2 != (DIR *)0x0) {
      uVar3 = FUN_0014b0d0(pDVar2,DAT_001847e4);
      return uVar3;
    }
    cVar1 = FUN_001125b0(param_1);
  } while (cVar1 != '\0');
  return 0;
}




// Function: check_links @ 0x14030

void check_links(void)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (DAT_001842c0 != 0) {
    for (lVar1 = FUN_00141930(); lVar1 != 0; lVar1 = FUN_00141980(DAT_001842c0,lVar1)) {
      if (*(long *)(lVar1 + 0x10) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar2 = FUN_0014ac50(lVar1 + 0x18);
        uVar3 = dcgettext(0,"Missing links to %s.",5);
        error(0,0,uVar3,uVar2);
      }
    }
    return;
  }
  return;
}




// Function: subfile_open @ 0x140d0

int subfile_open(long param_1,undefined8 param_2,undefined4 param_3)

{
  undefined4 uVar1;
  char cVar2;
  int iVar3;
  
  if (DAT_00184298 == '\0') {
    DAT_00184298 = '\x01';
    strerror(2);
    dcgettext(0,&DAT_00167708,5);
  }
  while( true ) {
    uVar1 = DAT_00181234;
    if (param_1 != 0) {
      uVar1 = *(undefined4 *)(param_1 + 0x1a8);
    }
    iVar3 = __openat_2(uVar1,param_2,param_3);
    if (-1 < iVar3) break;
    cVar2 = FUN_001125b0(param_1);
    if (cVar2 == '\0') {
      return iVar3;
    }
  }
  return iVar3;
}




// Function: check_exclusion_tags @ 0x14160

undefined4 check_exclusion_tags(undefined8 param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  char cVar2;
  int __fd;
  
  puVar1 = DAT_001842d0;
  do {
    if (puVar1 == (undefined8 *)0x0) {
      return 0;
    }
    __fd = FUN_001140d0(param_1,*puVar1,DAT_00184808);
    if (-1 < __fd) {
      if (((code *)puVar1[3] == (code *)0x0) || (cVar2 = (*(code *)puVar1[3])(__fd), cVar2 != '\0'))
      {
        close(__fd);
        if (param_2 != (undefined8 *)0x0) {
          *param_2 = *puVar1;
        }
        return *(undefined4 *)(puVar1 + 2);
      }
      close(__fd);
    }
    puVar1 = (undefined8 *)puVar1[4];
  } while( true );
}




// Function: restore_parent_fd @ 0x14200

void restore_parent_fd(long param_1)

{
  undefined8 *puVar1;
  int iVar2;
  int __fd;
  int *piVar3;
  int iVar4;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  puVar1 = *(undefined8 **)(param_1 + 0x198);
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((puVar1 != (undefined8 *)0x0) && (*(int *)(puVar1 + 0x35) == 0)) {
    iVar2 = __openat_2(*(undefined4 *)(param_1 + 0x1a8),"..",DAT_00184804);
    if (iVar2 < 0) {
      piVar3 = __errno_location();
      iVar4 = -*piVar3;
      iVar2 = iVar4;
      if (((0 < *piVar3) && (__fd = __openat_2(DAT_00181234,*puVar1,DAT_00184804), -1 < __fd)) &&
         ((iVar2 = fstat(iVar4,&local_b8), iVar2 != 0 ||
          ((puVar1[0xc] != local_b8.st_ino || (iVar2 = __fd, puVar1[0xb] != local_b8.st_dev)))))) {
        close(__fd);
        iVar2 = iVar4;
      }
    }
    else {
      iVar4 = fstat(iVar2,&local_b8);
      if (((iVar4 != 0) || (puVar1[0xc] != local_b8.st_ino)) || (puVar1[0xb] != local_b8.st_dev)) {
        close(iVar2);
        *(undefined4 *)(puVar1 + 0x35) = 2;
        goto LAB_0011426b;
      }
    }
    *(int *)(puVar1 + 0x35) = iVar2;
  }
LAB_0011426b:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dump_file @ 0x14330

void dump_file(long param_1,undefined8 param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined1 auStack_1e8 [408];
  long local_50;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0012f630(auStack_1e8);
  local_50 = param_1;
  FUN_00114ca0(auStack_1e8,param_2,param_3);
  if ((param_1 != 0) && (DAT_00184b28 != 0)) {
    FUN_0011f580(param_1);
  }
  FUN_0012f6f0(auStack_1e8);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: create_archive @ 0x143d0

void create_archive(void)

{
  char cVar1;
  int iVar2;
  long lVar3;
  size_t sVar4;
  void *__dest;
  char *pcVar5;
  long in_FS_OFFSET;
  size_t local_218;
  ulong local_200;
  long local_1f8 [11];
  stat local_1a0 [2];
  int local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001842c8 = 0;
  if (DAT_00184630 != 2) {
    DAT_001842c8 = (ulong)(DAT_00184b69 ^ 1);
  }
  FUN_0010fb70(1);
  FUN_0010cca0();
  if (DAT_00184b50 == '\0') {
    while (lVar3 = FUN_00126cd0(1), lVar3 != 0) {
      cVar1 = FUN_00116b00(lVar3,0);
      if (cVar1 == '\0') {
        FUN_00114330(0,lVar3,lVar3);
      }
    }
  }
  else {
    local_200 = 0;
    FUN_001276a0();
    while (lVar3 = FUN_00127b00(), lVar3 != 0) {
      cVar1 = FUN_00116b00(*(undefined8 *)(lVar3 + 0x10),0);
      if (cVar1 == '\0') {
        FUN_00114330(0,*(undefined8 *)(lVar3 + 0x10),*(undefined8 *)(lVar3 + 0x10));
      }
    }
    FUN_00127b80();
    __dest = (void *)0x0;
    while (lVar3 = FUN_00127b00(), lVar3 != 0) {
      cVar1 = FUN_00116b00(*(undefined8 *)(lVar3 + 0x10),0);
      if (cVar1 == '\0') {
        pcVar5 = *(char **)(lVar3 + 0x10);
        sVar4 = strlen(pcVar5);
        if (local_200 <= sVar4) {
          do {
            __dest = (void *)FUN_0014c710(__dest,&local_200);
          } while (local_200 <= sVar4);
          pcVar5 = *(char **)(lVar3 + 0x10);
        }
        memcpy(__dest,pcVar5,sVar4);
        local_218 = sVar4;
        if (*(char *)((long)__dest + (sVar4 - 1)) != '/') {
          local_218 = sVar4 + 1;
          *(undefined1 *)((long)__dest + sVar4) = 0x2f;
        }
        FUN_0012f630(local_1f8);
        pcVar5 = (char *)FUN_0011fa60(*(undefined8 *)(lVar3 + 0x38));
        if (pcVar5 != (char *)0x0) {
          for (; cVar1 = *pcVar5, cVar1 != '\0'; pcVar5 = pcVar5 + sVar4 + 1) {
            sVar4 = strlen(pcVar5);
            if (cVar1 == 'Y') {
              if (local_1f8[0] == 0) {
                iVar2 = __openat_2(DAT_00181234,*(undefined8 *)(lVar3 + 0x10),DAT_00184804);
                if (iVar2 < 0) {
                  FUN_00124ef0(*(undefined8 *)(lVar3 + 0x10),*(long *)(lVar3 + 0x40) == 0,
                               FUN_00124dd0);
                  break;
                }
                local_50 = iVar2;
                iVar2 = fstat(iVar2,local_1a0);
                if (iVar2 != 0) {
                  FUN_00124ef0(*(undefined8 *)(lVar3 + 0x10),*(long *)(lVar3 + 0x40) == 0,
                               FUN_00124ec0);
                  break;
                }
                local_1f8[0] = FUN_0014c7a0(*(undefined8 *)(lVar3 + 0x10));
              }
              while (local_200 < local_218 + sVar4) {
                __dest = (void *)FUN_0014c710(__dest,&local_200);
              }
              strcpy((char *)((long)__dest + local_218),pcVar5 + 1);
              FUN_00114330(local_1f8,pcVar5 + 1,__dest);
            }
          }
        }
        FUN_0012f6f0(local_1f8);
      }
    }
    free(__dest);
  }
  FUN_00112c40();
  FUN_0010f900();
  FUN_00130b30();
  if (DAT_00184b28 == 0) {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00120010();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_dir0 @ 0x14710

void dump_dir0(undefined8 *param_1,char *param_2)

{
  long lVar1;
  long lVar2;
  char cVar3;
  int iVar4;
  uint uVar5;
  undefined8 uVar6;
  long lVar7;
  undefined8 uVar8;
  void *__src;
  ulong uVar9;
  void *__dest;
  ulong uVar10;
  char *__dest_00;
  size_t sVar11;
  size_t sVar12;
  undefined8 *puVar13;
  long in_FS_OFFSET;
  byte bVar14;
  char *local_48;
  long local_40;
  
  bVar14 = 0;
  lVar1 = param_1[0x33];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar6 = FUN_0010d180();
  param_1[0x11] = 0;
  lVar7 = FUN_001132d0(param_1);
  if (lVar7 != 0) {
    FUN_00116890(param_1);
    if (DAT_00184b50 == '\0') {
      *(undefined1 *)(lVar7 + 0x9c) = 0x35;
      FUN_00112f60(param_1,lVar7,uVar6);
    }
    else {
      lVar2 = *(long *)(DAT_00184620 + 0x38);
      if (DAT_00184ba4 == 4) {
        *(undefined1 *)(lVar7 + 0x9c) = 0x35;
        if (lVar2 != 0) {
          uVar8 = FUN_0011fa90();
          FUN_0011ca30("GNU.dumpdir",param_1,uVar8);
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
            FUN_00112f60(param_1,lVar7,uVar6);
            return;
          }
          goto LAB_00114c3a;
        }
      }
      else {
        *(undefined1 *)(lVar7 + 0x9c) = 0x44;
        if (lVar2 != 0) {
          uVar6 = FUN_0010d180();
          __src = (void *)FUN_0011fa90(*(undefined8 *)(DAT_00184620 + 0x38));
          uVar9 = FUN_0011f4c0(__src);
          FUN_00112be0(uVar9,lVar7 + 0x7c,0xc);
          FUN_00112f60(param_1,lVar7,uVar6);
          FUN_0010cc00(param_1[1],uVar9,uVar9);
          for (; 0 < (long)uVar9; uVar9 = uVar9 - uVar10) {
            __dest = (void *)FUN_0010e130();
            uVar10 = FUN_0010d2b0(__dest);
            if (uVar9 < uVar10) {
              if ((uVar9 & 0x1ff) != 0) {
                uVar5 = 0x200 - ((uint)uVar9 & 0x1ff);
                puVar13 = (undefined8 *)((long)__dest + uVar9);
                if (uVar5 < 8) {
                  if ((uVar5 & 4) == 0) {
                    if ((uVar5 != 0) && (*(undefined1 *)puVar13 = 0, (uVar5 & 2) != 0)) {
                      *(undefined2 *)((long)puVar13 + ((ulong)uVar5 - 2)) = 0;
                    }
                  }
                  else {
                    *(undefined4 *)puVar13 = 0;
                    *(undefined4 *)((long)puVar13 + ((ulong)uVar5 - 4)) = 0;
                  }
                }
                else {
                  *puVar13 = 0;
                  *(undefined8 *)((long)puVar13 + ((ulong)uVar5 - 8)) = 0;
                  uVar10 = (ulong)(uVar5 + ((int)puVar13 -
                                           (int)(undefined8 *)
                                                ((ulong)(puVar13 + 1) & 0xfffffffffffffff8)) >> 3);
                  puVar13 = (undefined8 *)((ulong)(puVar13 + 1) & 0xfffffffffffffff8);
                  for (; uVar10 != 0; uVar10 = uVar10 - 1) {
                    *puVar13 = 0;
                    puVar13 = puVar13 + (ulong)bVar14 * -2 + 1;
                  }
                }
              }
              memcpy(__dest,__src,uVar9);
              if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_0010d1f0((long)__dest + (uVar9 - 1 & 0xfffffffffffffe00));
                return;
              }
              goto LAB_00114c3a;
            }
            memcpy(__dest,__src,uVar10);
            __src = (void *)((long)__src + uVar10);
            FUN_0010d1f0((long)__dest + (uVar10 - 1 & 0xfffffffffffffe00));
          }
          goto LAB_00114880;
        }
      }
    }
    if (DAT_00184ad4 != 0) {
      if (((DAT_00184ad1 == '\0') || (lVar1 == 0)) ||
         (*(long *)(param_1[0x33] + 0x58) == param_1[0xb])) {
        iVar4 = FUN_00114160(param_1,&local_48);
        if (iVar4 == 1) {
          uVar6 = dcgettext(0,"contents not dumped",5);
          FUN_00112ad0(*param_1,local_48,uVar6);
          sVar11 = strlen((char *)*param_1);
          sVar12 = strlen(local_48);
          __dest_00 = (char *)FUN_0014c5b0(sVar11 + 1 + sVar12);
          strcpy(__dest_00,(char *)*param_1);
          strcat(__dest_00,local_48);
          FUN_00114330(param_1,local_48,__dest_00);
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_00114b87:
            free(__dest_00);
            return;
          }
          goto LAB_00114c3a;
        }
        if (iVar4 == 2) {
          uVar6 = dcgettext(0,"contents not dumped",5);
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
            FUN_00112ad0(*param_1,local_48,uVar6);
            return;
          }
          goto LAB_00114c3a;
        }
        if (iVar4 == 0) {
          __dest_00 = (char *)FUN_0014c7a0(*param_1);
          sVar11 = strlen(__dest_00);
          uVar9 = sVar11;
          while (sVar12 = strlen(param_2), sVar12 != 0) {
            uVar10 = sVar11 + sVar12;
            if (uVar9 < uVar10) {
              __dest_00 = (char *)FUN_0014c610(__dest_00,uVar10 + 1);
              uVar9 = uVar10;
            }
            strcpy(__dest_00 + sVar11,param_2);
            cVar3 = FUN_00116b00(__dest_00,param_1);
            if (cVar3 == '\0') {
              FUN_00114330(param_1,param_2,__dest_00);
            }
            param_2 = param_2 + sVar12 + 1;
          }
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) goto LAB_00114b87;
          goto LAB_00114c3a;
        }
      }
      else if ((DAT_00184a54 != 0) && ((DAT_00183b80._2_1_ & 4) != 0)) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar6 = FUN_0014aa70(*param_1);
        uVar8 = dcgettext(0,"%s: file is on a different filesystem; not dumped",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          error(0,0,uVar8,uVar6);
          return;
        }
        goto LAB_00114c3a;
      }
    }
  }
LAB_00114880:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00114c3a:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_dir @ 0x14c40

undefined8 dump_dir(undefined8 *param_1)

{
  void *__ptr;
  
  __ptr = (void *)FUN_00113fe0();
  if (__ptr != (void *)0x0) {
    FUN_00114710(param_1,__ptr);
    FUN_00114200(param_1);
    free(__ptr);
    return 1;
  }
  FUN_00124e60(*param_1);
  return 0;
}




// Function: dump_file0 @ 0x14ca0

void dump_file0(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  stat *__buf;
  long lVar1;
  ulong uVar2;
  char cVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  undefined8 uVar8;
  int *piVar9;
  size_t sVar10;
  ulong uVar11;
  long lVar12;
  undefined8 uVar13;
  code *pcVar14;
  char *pcVar15;
  long in_FS_OFFSET;
  bool bVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  char local_122;
  int local_100;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  iVar6 = DAT_00181234;
  lVar12 = param_1[0x33];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  bVar16 = lVar12 == 0;
  local_122 = DAT_00184b40;
  if (bVar16) {
    if (DAT_00184b40 == '\0') {
      FUN_00123be0(param_1,param_3);
      uVar8 = FUN_00133020(param_3,0,DAT_00184b92);
      FUN_00123be0(param_1 + 1,uVar8);
      FUN_00130830(param_1 + 1,1);
    }
    else {
      iVar4 = FUN_0012ca20(&DAT_00160e31,param_3);
      if (iVar4 == 0) goto LAB_00114db0;
      FUN_00123be0(param_1,param_3);
      uVar8 = FUN_00133020(param_3,0,DAT_00184b92);
      FUN_00123be0(param_1 + 1,uVar8);
      FUN_00130830(param_1 + 1,1);
      local_122 = '\0';
    }
LAB_00114e4d:
    __buf = (stat *)(param_1 + 0xb);
    iVar4 = fstatat(iVar6,param_2,__buf,DAT_00184800);
    if (iVar4 == 0) {
      cVar3 = FUN_00111d50(__buf);
      local_100 = 0;
      if (cVar3 != '\0') {
        local_100 = FUN_001140d0(lVar12,param_2,DAT_00184808);
        if (local_100 < 0) {
          pcVar14 = FUN_00124dd0;
          goto LAB_00114d67;
        }
        *(int *)(param_1 + 0x35) = local_100;
        iVar4 = fstat(local_100,__buf);
        if (iVar4 != 0) goto LAB_001150b0;
      }
      lVar1 = param_1[0x11];
      param_1[0x23] = lVar1;
      auVar17 = FUN_0014bd00(__buf);
      *(undefined1 (*) [16])(param_1 + 0x1d) = auVar17;
      auVar17 = FUN_0014bd20(__buf);
      *(undefined1 (*) [16])(param_1 + 0x1f) = auVar17;
      auVar17 = FUN_0014bd10(__buf);
      cVar3 = DAT_00184b50;
      *(undefined1 (*) [16])(param_1 + 0x21) = auVar17;
      uVar13 = DAT_00184b08;
      uVar8 = DAT_00184b00;
      if ((((cVar3 == '\x01') && (!bVar16)) || ((*(uint *)(param_1 + 0xe) & 0xf000) == 0x4000)) ||
         ((iVar4 = FUN_0014be80(param_1[0x1f],param_1[0x20],DAT_00184b00,DAT_00184b08), -1 < iVar4
          || ((DAT_00184b8c != 0 &&
              (iVar4 = FUN_0014be80(param_1[0x21],param_1[0x22],uVar8,uVar13), -1 < iVar4)))))) {
        cVar3 = FUN_0012ab40(param_1);
        if (cVar3 == '\0') {
          uVar5 = *(uint *)(param_1 + 0xe) & 0xf000;
          if (uVar5 == 0x4000) {
            FUN_001319f0(iVar6,param_2,param_1,0,0);
            FUN_00131de0(iVar6,param_2,param_1,local_100);
            FUN_00131b80(iVar6,param_2,param_1,local_100);
            FUN_001126e0(param_1);
            FUN_001126e0(param_1 + 1);
            iVar6 = FUN_00114160(param_1,&local_e0);
            if (iVar6 == 3) {
              uVar8 = dcgettext(0,"directory not dumped",5);
              if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_00112ad0(*param_1,local_e0,uVar8);
                return;
              }
              goto LAB_00115851;
            }
            cVar3 = FUN_00114c40(param_1);
            local_100 = *(int *)(param_1 + 0x35);
            iVar6 = DAT_00181234;
            if (lVar12 != 0) {
              iVar6 = *(int *)(lVar12 + 0x1a8);
            }
            if (cVar3 != '\0') {
              if (-1 < local_100) goto LAB_001153ca;
              piVar9 = __errno_location();
              *piVar9 = -local_100;
LAB_0011500a:
              FUN_00124ef0(param_3,bVar16,FUN_00124ec0);
            }
LAB_0011501e:
            FUN_0012f670(param_1);
          }
          else {
            cVar3 = FUN_00113b00(param_1);
            if (cVar3 == '\0') {
              uVar7 = *(uint *)(param_1 + 0xe) & 0xf000;
              if (uVar7 != 0x8000) {
                if (uVar7 == 0xa000) {
                  lVar12 = FUN_00139200(iVar6,param_2,param_1[0x11]);
                  param_1[3] = lVar12;
                  if (lVar12 == 0) {
                    piVar9 = __errno_location();
                    if (*piVar9 == 0xc) {
                    /* WARNING: Subroutine does not return */
                      FUN_00116420();
                    }
                    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_00115851;
                    pcVar14 = FUN_00124e30;
                    goto LAB_00114d83;
                  }
                  FUN_00130830(param_1 + 3,4);
                  bVar16 = DAT_00184ba4 != 2;
                  sVar10 = strlen((char *)param_1[3]);
                  if ((ulong)bVar16 + 99 < sVar10) {
                    FUN_00113a40(param_1);
                  }
                  FUN_00131de0(iVar6,param_2,param_1,0);
                  FUN_00131b80(iVar6,param_2,param_1,0);
                  uVar8 = FUN_0010d180();
                  param_1[0x11] = 0;
                  lVar12 = FUN_001132d0(param_1);
                  if (lVar12 != 0) {
                    FUN_00111d00(lVar12 + 0x9d,param_1[3],100);
                    *(undefined1 *)(lVar12 + 0x9c) = 0x32;
                    FUN_00112f60(param_1,lVar12,uVar8);
                    if (DAT_00184ab8 != '\0') {
                      FUN_00130b70(param_3,0);
                    }
                    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                      FUN_00112760(param_1);
                      return;
                    }
                    goto LAB_00115851;
                  }
                  goto LAB_00114db0;
                }
                if (uVar7 == 0x2000) {
                  FUN_001319f0(iVar6,param_2,param_1,0,1);
                  FUN_00131de0(iVar6,param_2,param_1,0);
                  cVar3 = '3';
                  FUN_00131b80(iVar6,param_2,param_1,0);
LAB_00115520:
                  if (DAT_00184ba4 != 1) {
                    uVar8 = FUN_0010d180();
                    param_1[0x11] = 0;
                    lVar12 = FUN_001132d0(param_1);
                    if (lVar12 != 0) {
                      *(char *)(lVar12 + 0x9c) = cVar3;
                      if (cVar3 != '6') {
                        uVar5 = gnu_dev_major(param_1[0x10]);
                        FUN_00112910(uVar5,lVar12 + 0x149);
                        uVar5 = gnu_dev_minor(param_1[0x10]);
                        FUN_001128e0(uVar5,lVar12 + 0x151);
                      }
                      FUN_00112f60(param_1,lVar12,uVar8);
                      if (DAT_00184ab8 != '\0') {
                        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                          FUN_00130b70(param_3,0);
                          return;
                        }
                        goto LAB_00115851;
                      }
                    }
                    goto LAB_00114db0;
                  }
                }
                else {
                  if (uVar7 == 0x6000) {
                    FUN_001319f0(iVar6,param_2,param_1,0,1);
                    FUN_00131de0(iVar6,param_2,param_1,0);
                    cVar3 = '4';
                    FUN_00131b80(iVar6,param_2,param_1,0);
                    goto LAB_00115520;
                  }
                  if (uVar7 == 0x1000) {
                    FUN_001319f0(iVar6,param_2,param_1,0,1);
                    FUN_00131de0(iVar6,param_2,param_1,0);
                    cVar3 = '6';
                    FUN_00131b80(iVar6,param_2,param_1,0);
                    goto LAB_00115520;
                  }
                  if (uVar7 == 0xc000) {
                    if (((byte)DAT_00183b80 & 0x20) != 0) {
                      if (DAT_00184d50 != (code *)0x0) {
                        (*DAT_00184d50)();
                      }
                      uVar8 = FUN_0014aa70(param_3);
                      pcVar15 = "%s: socket ignored";
                      goto LAB_00115150;
                    }
                    goto LAB_00114db0;
                  }
                }
                if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                  FUN_00112640(param_3);
                  return;
                }
                goto LAB_00115851;
              }
              FUN_001319f0(iVar6,param_2,param_1,0,1);
              FUN_00131de0(iVar6,param_2,param_1,local_100);
              FUN_00131b80(iVar6,param_2,param_1,local_100);
              if ((local_100 == 0) || (DAT_00184a88 == '\0')) {
LAB_001153a7:
                uVar7 = FUN_00113d10(local_100,param_1);
                if (1 < uVar7) {
                  if (uVar7 == 3) {
                    /* WARNING: Subroutine does not return */
                    abort();
                  }
                  goto LAB_0011501e;
                }
              }
              else {
                uVar2 = param_1[0x11];
                uVar11 = uVar2;
                if ((long)uVar2 < 0) {
                  uVar11 = uVar2 + 0x1ff;
                }
                lVar12 = (long)uVar11 >> 9;
                if (((uVar2 & 0x1ff) != 0) && (0x3fe < uVar2 + 0x1ff)) {
                  lVar12 = lVar12 + 1;
                }
                if ((lVar12 <= (long)param_1[0x13]) ||
                   (uVar7 = FUN_00129cd0(local_100,param_1), uVar7 == 3)) goto LAB_001153a7;
                if (1 < uVar7) goto LAB_0011501e;
              }
              FUN_00112760(param_1);
              if (uVar7 != 0) goto LAB_0011501e;
LAB_001153ca:
              if (local_100 == 0) {
                if ((iVar6 < 0) && (local_122 != '\0')) {
                  piVar9 = __errno_location();
                  *piVar9 = -iVar6;
                  goto LAB_0011500a;
                }
                iVar4 = fstatat(iVar6,param_2,&local_d8,DAT_00184800);
              }
              else {
                iVar4 = fstat(local_100,&local_d8);
              }
              if (iVar4 != 0) goto LAB_0011500a;
              auVar18 = FUN_0014bd10(&local_d8);
              iVar4 = FUN_0014be80(auVar18._0_8_,auVar18._8_8_,auVar17._0_8_,auVar17._8_8_);
              if (((iVar4 == 0) || ((DAT_00184ab8 == '\x01' && (uVar5 == 0x4000)))) &&
                 (local_d8.st_size <= lVar1)) {
                if ((((DAT_00184b88 == 1) && (local_100 != 0)) &&
                    ((lVar1 != 0 || (uVar5 == 0x4000)))) &&
                   (iVar6 = FUN_00124a80(local_100,iVar6,param_2,param_1[0x1d],param_1[0x1e]),
                   iVar6 != 0)) {
                  FUN_00132dd0(param_3);
                }
              }
              else {
                if (((byte)DAT_00183b80 & 0x10) != 0) {
                  if (DAT_00184d50 != (code *)0x0) {
                    (*DAT_00184d50)();
                  }
                  uVar8 = FUN_0014aa70(param_3);
                  uVar13 = dcgettext(0,"%s: file changed as we read it",5);
                  error(0,0,uVar13,uVar8);
                }
                FUN_0012f7d0(1);
              }
              cVar3 = FUN_0012f670(param_1);
              if ((cVar3 != '\0') && (DAT_00184ab8 != '\0')) {
                FUN_00130b70(param_3,uVar5 == 0x4000);
              }
            }
          }
LAB_00114db0:
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
            return;
          }
          goto LAB_00115851;
        }
        if ((DAT_00183b80._1_1_ & 4) == 0) goto LAB_00114db0;
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar8 = FUN_0014aa70(param_3);
        pcVar15 = "%s: file is the archive; not dumped";
      }
      else {
        if (((cVar3 != '\0') || (DAT_00184a54 == 0)) || ((DAT_00183b80._1_1_ & 1) == 0))
        goto LAB_00114db0;
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar8 = FUN_0014aa70(param_3);
        pcVar15 = "%s: file is unchanged; not dumped";
      }
LAB_00115150:
      uVar13 = dcgettext(0,pcVar15,5);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        error(0,0,uVar13,uVar8);
        return;
      }
      goto LAB_00115851;
    }
LAB_001150b0:
    pcVar14 = FUN_00124ec0;
  }
  else {
    iVar6 = *(int *)(lVar12 + 0x1a8);
    if ((DAT_00184b40 != '\0') && (iVar4 = FUN_0012ca20(&DAT_00160e31,param_3), iVar4 == 0))
    goto LAB_00114db0;
    FUN_00123be0(param_1,param_3);
    uVar8 = FUN_00133020(param_3,0,DAT_00184b92);
    FUN_00123be0(param_1 + 1,uVar8);
    FUN_00130830(param_1 + 1,1);
    if (-1 < iVar6) {
      local_122 = '\x01';
      goto LAB_00114e4d;
    }
    piVar9 = __errno_location();
    pcVar14 = FUN_00124dd0;
    *piVar9 = -iVar6;
  }
LAB_00114d67:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_00114d83:
    FUN_00124ef0(param_3,bVar16,pcVar14);
    return;
  }
LAB_00115851:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: flush_file @ 0x15860

void flush_file(void)

{
  long lVar1;
  long lVar2;
  
  FUN_0010d1f0(DAT_00184408);
  lVar2 = DAT_001848c8 + 0x3fe;
  if (-1 < DAT_001848c8 + 0x1ff) {
    lVar2 = DAT_001848c8 + 0x1ff;
  }
  lVar1 = DAT_00183f50 - DAT_00183f48;
  lVar2 = lVar2 >> 9;
  if (lVar1 >> 9 <= lVar2) {
    do {
      lVar2 = lVar2 - (lVar1 >> 9);
      FUN_0010dfd0();
      lVar1 = DAT_00183f50 - DAT_00183f48;
    } while (lVar1 >> 9 <= lVar2);
  }
  DAT_00183f48 = DAT_00183f48 + lVar2 * 0x200;
  return;
}




// Function: move_archive @ 0x158f0

void move_archive(long param_1)

{
  int iVar1;
  int *piVar2;
  long lVar3;
  long lVar4;
  long __offset;
  long in_FS_OFFSET;
  bool bVar5;
  undefined2 local_28 [2];
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
LAB_00115a30:
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    local_24 = (int)param_1;
    if (param_1 < 0) {
      local_24 = -local_24;
      local_28[0] = 4;
      bVar5 = local_24 + param_1 == 0;
    }
    else {
      local_28[0] = 3;
      bVar5 = local_24 == param_1;
    }
    if (bVar5) {
      if (DAT_00184a34 < 0x40000000) {
        iVar1 = ioctl(DAT_00184a34,0x40086d01,local_28);
      }
      else {
        iVar1 = FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_28);
      }
      if (-1 < iVar1) goto LAB_00115a30;
      piVar2 = __errno_location();
      if (*piVar2 == 5) {
        if (DAT_00184a34 < 0x40000000) {
          iVar1 = ioctl(DAT_00184a34,0x40086d01,local_28);
        }
        else {
          iVar1 = FUN_00134080(DAT_00184a34 + -0x40000000,0x40086d01,local_28);
        }
        if (-1 < iVar1) goto LAB_00115a30;
      }
    }
    if (DAT_00184a34 < 0x40000000) {
      lVar3 = lseek(DAT_00184a34,0,1);
    }
    else {
      lVar3 = FUN_00133f90(DAT_00184a34 + -0x40000000,0,1);
    }
    lVar4 = param_1 * DAT_00184b98;
    __offset = lVar3 + lVar4;
    if ((SEXT816(lVar4) == SEXT816(param_1) * SEXT816(DAT_00184b98)) &&
       (__offset < lVar3 == (bool)-(char)(lVar4 >> 0x3f))) {
      if (__offset < 0) {
        __offset = 0;
      }
      if (DAT_00184a34 < 0x40000000) {
        lVar3 = lseek(DAT_00184a34,__offset,0);
      }
      else {
        lVar3 = FUN_00133f90(DAT_00184a34 + -0x40000000);
      }
      if (lVar3 == __offset) goto LAB_00115a30;
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00132b20(*DAT_00184838);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_record @ 0x15b10

void write_record(int param_1)

{
  undefined8 uVar1;
  
  uVar1 = DAT_00183f58;
  DAT_00183f58 = DAT_001842e8;
  if (DAT_001842e0 == '\0') {
    FUN_001158f0((DAT_001842d8 + DAT_00183f30) - DAT_00183f38);
    FUN_0010fb50();
  }
  else {
    DAT_00184a34 = 1;
    FUN_0010fb50();
    DAT_00184a34 = 0;
  }
  DAT_00183f58 = uVar1;
  if ((param_1 != 0) && (DAT_001842e0 == '\0')) {
    FUN_001158f0(DAT_00183f38 - (DAT_001842d8 + DAT_00183f30));
  }
  DAT_001842e4 = 0;
  return;
}




// Function: write_recent_blocks @ 0x15bc0

void write_recent_blocks(long param_1,long param_2)

{
  int iVar1;
  undefined8 *puVar2;
  long lVar3;
  ulong uVar4;
  long lVar5;
  undefined8 *puVar6;
  bool bVar7;
  byte bVar8;
  
  bVar8 = 0;
  if (param_2 == 0) {
    return;
  }
  lVar5 = 0;
  do {
    while( true ) {
      puVar6 = (undefined8 *)(lVar5 * 0x200 + param_1);
      iVar1 = DAT_001842e4 + 1;
      puVar2 = (undefined8 *)((long)DAT_001842e4 * 0x200 + DAT_001842e8);
      DAT_001842e4 = iVar1;
      *puVar2 = *puVar6;
      puVar2[0x3f] = puVar6[0x3f];
      lVar3 = (long)puVar2 - (long)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
      bVar7 = iVar1 == DAT_00184ba0;
      puVar6 = (undefined8 *)((long)puVar6 - lVar3);
      puVar2 = (undefined8 *)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
      for (uVar4 = (ulong)((int)lVar3 + 0x200U >> 3); uVar4 != 0; uVar4 = uVar4 - 1) {
        *puVar2 = *puVar6;
        puVar6 = puVar6 + (ulong)bVar8 * -2 + 1;
        puVar2 = puVar2 + (ulong)bVar8 * -2 + 1;
      }
      if (bVar7) break;
      lVar5 = lVar5 + 1;
      if (param_2 == lVar5) {
        return;
      }
    }
    lVar5 = lVar5 + 1;
    FUN_00115b10(1);
  } while (param_2 != lVar5);
  return;
}




// Function: write_recent_bytes @ 0x15c70

void write_recent_bytes(long param_1,ulong param_2)

{
  undefined8 *puVar1;
  long lVar2;
  ulong uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  uint uVar7;
  byte bVar8;
  
  bVar8 = 0;
  uVar7 = (uint)param_2 & 0x1ff;
  uVar4 = (ulong)uVar7;
  FUN_00115bc0(param_1,param_2 >> 9);
  puVar5 = (undefined8 *)(param_1 + (param_2 & 0xfffffffffffffe00));
  puVar1 = (undefined8 *)((long)DAT_001842e4 * 0x200 + DAT_001842e8);
  if (uVar7 < 8) {
    if ((param_2 & 4) == 0) {
      if (((param_2 & 0x1ff) != 0) &&
         (*(undefined1 *)puVar1 = *(undefined1 *)puVar5, (param_2 & 2) != 0)) {
        *(undefined2 *)((long)puVar1 + (uVar4 - 2)) = *(undefined2 *)((uVar4 - 2) + (long)puVar5);
      }
    }
    else {
      *(undefined4 *)puVar1 = *(undefined4 *)puVar5;
      *(undefined4 *)((long)puVar1 + (uVar4 - 4)) = *(undefined4 *)((uVar4 - 4) + (long)puVar5);
    }
  }
  else {
    *puVar1 = *puVar5;
    *(undefined8 *)((long)puVar1 + (uVar4 - 8)) = *(undefined8 *)((uVar4 - 8) + (long)puVar5);
    lVar2 = (long)puVar1 - (long)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    puVar5 = (undefined8 *)((long)puVar5 - lVar2);
    puVar6 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    for (uVar3 = (ulong)((int)lVar2 + uVar7 >> 3); uVar3 != 0; uVar3 = uVar3 - 1) {
      *puVar6 = *puVar5;
      puVar5 = puVar5 + (ulong)bVar8 * -2 + 1;
      puVar6 = puVar6 + (ulong)bVar8 * -2 + 1;
    }
  }
  puVar1 = (undefined8 *)((long)puVar1 + uVar4);
  uVar4 = 0x200 - uVar4;
  uVar7 = (uint)uVar4;
  if (uVar7 < 8) {
    if ((uVar4 & 4) == 0) {
      if ((uVar7 != 0) && (*(undefined1 *)puVar1 = 0, (uVar4 & 2) != 0)) {
        *(undefined2 *)((long)puVar1 + ((uVar4 & 0xffffffff) - 2)) = 0;
      }
    }
    else {
      *(undefined4 *)puVar1 = 0;
      *(undefined4 *)((long)puVar1 + ((uVar4 & 0xffffffff) - 4)) = 0;
    }
  }
  else {
    *puVar1 = 0;
    *(undefined8 *)((long)puVar1 + ((uVar4 & 0xffffffff) - 8)) = 0;
    puVar5 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    for (uVar4 = (ulong)(uVar7 + ((int)puVar1 -
                                 (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8)) >> 3
                        ); uVar4 != 0; uVar4 = uVar4 - 1) {
      *puVar5 = 0;
      puVar5 = puVar5 + (ulong)bVar8 * -2 + 1;
    }
  }
  DAT_001842e4 = DAT_001842e4 + 1;
  if (DAT_001842e4 != DAT_00184ba0) {
    return;
  }
  FUN_00115b10(1);
  return;
}




// Function: fatal_exit @ 0x163d0

void fatal_exit(void)

{
  undefined8 uVar1;
  
  if (DAT_001842f0 != (code *)0x0) {
    (*DAT_001842f0)();
  }
  uVar1 = dcgettext(0,"Error is not recoverable: exiting now",5);
  error(2,0,uVar1);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: xalloc_die @ 0x16420

void xalloc_die(void)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"memory exhausted",5);
  error(0,0,"%s",uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: hg_initfn @ 0x16460

void hg_initfn(undefined4 *param_1)

{
  if (param_1 == (undefined4 *)0x0) {
    param_1 = &DAT_001842f8;
  }
  *param_1 = 0x8000000;
  return;
}




// Function: get_vcs_ignore_file @ 0x16480

undefined ** get_vcs_ignore_file(char *param_1)

{
  char *__s1;
  int iVar1;
  undefined **ppuVar2;
  
  ppuVar2 = &PTR_s__cvsignore_00181160;
  __s1 = PTR_s__cvsignore_00181160;
  while ((__s1 != (char *)0x0 && (iVar1 = strcmp(__s1,param_1), iVar1 != 0))) {
    __s1 = ppuVar2[5];
    ppuVar2 = ppuVar2 + 5;
  }
  return ppuVar2;
}




// Function: bzr_addfn @ 0x164d0

void bzr_addfn(undefined8 param_1,char *param_2,uint param_3)

{
  char cVar1;
  int iVar2;
  ushort **ppuVar3;
  
  ppuVar3 = __ctype_b_loc();
  while (cVar1 = *param_2, (*(byte *)((long)*ppuVar3 + (long)cVar1 * 2 + 1) & 0x20) != 0) {
    param_2 = param_2 + 1;
  }
  if ((cVar1 != '\0') && (cVar1 != '#')) {
    if (cVar1 == '!') {
      if (param_2[1] == '!') {
        param_2 = param_2 + 2;
      }
      else {
        param_2 = param_2 + 1;
        param_3 = param_3 | 0x20000000;
      }
    }
    iVar2 = strncmp(param_2,"RE:",3);
    if (iVar2 == 0) {
      param_2 = param_2 + 3;
      param_3 = param_3 & 0xefffffff | 0x8000000;
    }
    FUN_0013ee30(param_1,param_2,param_3);
    return;
  }
  return;
}




// Function: git_addfn @ 0x16580

void git_addfn(undefined8 param_1,char *param_2,undefined4 param_3)

{
  char cVar1;
  ushort **ppuVar2;
  
  ppuVar2 = __ctype_b_loc();
  while( true ) {
    cVar1 = *param_2;
    if ((*(byte *)((long)*ppuVar2 + (long)cVar1 * 2 + 1) & 0x20) == 0) break;
    param_2 = param_2 + 1;
  }
  if ((cVar1 != '\0') && (cVar1 != '#')) {
    if (cVar1 == '\\') {
      param_2 = param_2 + (param_2[1] == '#');
    }
    FUN_0013ee30(param_1,param_2,param_3);
    return;
  }
  return;
}




// Function: hg_addfn @ 0x16600

void hg_addfn(undefined8 param_1,char *param_2,uint param_3,uint *param_4)

{
  char *pcVar1;
  char cVar2;
  byte bVar3;
  ushort *puVar4;
  uint uVar5;
  int iVar6;
  ushort **ppuVar7;
  size_t sVar8;
  char *pcVar9;
  
  ppuVar7 = __ctype_b_loc();
  puVar4 = *ppuVar7;
  while (cVar2 = *param_2, (*(byte *)((long)puVar4 + (long)cVar2 * 2 + 1) & 0x20) != 0) {
    param_2 = param_2 + 1;
  }
  if ((cVar2 != '\0') && (cVar2 != '#')) {
    iVar6 = strncmp(param_2,"syntax:",7);
    if (iVar6 != 0) {
      sVar8 = strlen(param_2);
      pcVar9 = param_2;
      if (param_2[sVar8 - 1] == '/') {
        param_3 = param_3 | 0x4000008;
        pcVar9 = (char *)FUN_0014c5b0(sVar8);
        memcpy(pcVar9,param_2,sVar8 - 1);
        pcVar9[sVar8 - 1] = '\0';
        FUN_0013eaa0(param_1,pcVar9);
      }
      uVar5 = param_3 & 0xf7ffffff;
      if (*param_4 == 0x8000000) {
        uVar5 = param_3 & 0xefffffff;
      }
      FUN_0013ee30(param_1,pcVar9,*param_4 | uVar5);
      return;
    }
    pcVar9 = param_2 + 7;
    bVar3 = *(byte *)((long)puVar4 + (long)param_2[7] * 2 + 1);
    while ((bVar3 & 0x20) != 0) {
      pcVar1 = pcVar9 + 1;
      pcVar9 = pcVar9 + 1;
      bVar3 = *(byte *)((long)puVar4 + (long)*pcVar1 * 2 + 1);
    }
    iVar6 = strcmp(pcVar9,"regexp");
    if (iVar6 == 0) {
      *param_4 = 0x8000000;
    }
    else {
      iVar6 = strcmp(pcVar9,"glob");
      if (iVar6 == 0) {
        *param_4 = 0x10000000;
      }
    }
  }
  return;
}




// Function: cvs_addfn @ 0x16780

void cvs_addfn(undefined8 param_1,undefined8 param_2,undefined4 param_3)

{
  long lVar1;
  int iVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  ulong local_118;
  long local_110;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_001382c0(param_2,&local_118,0x844);
  if (iVar2 == 0) {
    if (local_118 != 0) {
      uVar3 = 0;
      do {
        lVar1 = uVar3 * 8;
        uVar3 = uVar3 + 1;
        FUN_0013ee30(param_1,*(undefined8 *)(local_110 + lVar1),param_3);
      } while (uVar3 < local_118);
    }
    FUN_00135420(&local_118);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: excfile_add @ 0x16820

void excfile_add(char *param_1,undefined4 param_2)

{
  size_t sVar1;
  undefined8 *puVar2;
  
  sVar1 = strlen(param_1);
  puVar2 = (undefined8 *)FUN_0014c5b0(sVar1 + 0x10);
  *puVar2 = 0;
  *(undefined4 *)(puVar2 + 1) = param_2;
  strcpy((char *)((long)puVar2 + 0xc),param_1);
  if (DAT_00184300 != (undefined8 *)0x0) {
    *DAT_00184300 = puVar2;
    DAT_00184300 = puVar2;
    return;
  }
  DAT_00184308 = puVar2;
  DAT_00184300 = puVar2;
  return;
}




// Function: info_attach_exclist @ 0x16890

void info_attach_exclist(long param_1)

{
  char *__file;
  long lVar1;
  undefined8 *puVar2;
  int iVar3;
  FILE *__stream;
  undefined8 uVar4;
  long lVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  int *piVar8;
  undefined4 uVar9;
  long *plVar10;
  undefined8 *puVar11;
  undefined8 *local_40;
  
  if (*(long *)(param_1 + 0x1b0) != 0) {
    return;
  }
  local_40 = (undefined8 *)0x0;
  if (DAT_00184308 != (long *)0x0) {
    plVar10 = DAT_00184308;
    puVar11 = (undefined8 *)0x0;
    do {
      while( true ) {
        __file = (char *)((long)plVar10 + 0xc);
        iVar3 = faccessat(*(int *)(param_1 + 0x1a8),__file,0,0);
        puVar7 = puVar11;
        puVar2 = local_40;
        if (iVar3 == 0) break;
LAB_001168e0:
        local_40 = puVar2;
        plVar10 = (long *)*plVar10;
        puVar11 = puVar7;
        if (plVar10 == (long *)0x0) goto LAB_001169d6;
      }
      iVar3 = FUN_001140d0(param_1,__file,0);
      if (iVar3 == -1) {
        FUN_001327c0(__file);
        goto LAB_001168e0;
      }
      __stream = fdopen(iVar3,"r");
      if (__stream == (FILE *)0x0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = dcgettext(0,"%s: fdopen failed",5);
        piVar8 = __errno_location();
        error(0,*piVar8,uVar4);
        DAT_00184d58 = 2;
        close(iVar3);
        goto LAB_001168e0;
      }
      uVar4 = FUN_0013eba0();
      lVar5 = FUN_00116480(__file);
      if (*(code **)(lVar5 + 0x18) == (code *)0x0) {
        uVar6 = *(undefined8 *)(lVar5 + 0x20);
      }
      else {
        uVar6 = (**(code **)(lVar5 + 0x18))(*(undefined8 *)(lVar5 + 0x20));
        *(undefined8 *)(lVar5 + 0x20) = uVar6;
      }
      iVar3 = FUN_0013f0c0(*(undefined8 *)(lVar5 + 0x10),uVar4,__stream,0x50000001,10,uVar6);
      if (iVar3 != 0) {
        piVar8 = __errno_location();
        iVar3 = *piVar8;
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = FUN_0014aa70(__file);
        error(0,iVar3,"%s",uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
      fclose(__stream);
      puVar7 = (undefined8 *)FUN_0014c5b0(0x20);
      lVar1 = plVar10[1];
      puVar7[3] = uVar4;
      uVar9 = 0;
      if ((int)lVar1 != 0) {
        uVar9 = *(undefined4 *)(lVar5 + 8);
      }
      *(undefined4 *)(puVar7 + 2) = uVar9;
      puVar7[1] = puVar11;
      *puVar7 = 0;
      puVar2 = puVar7;
      if (puVar11 == (undefined8 *)0x0) goto LAB_001168e0;
      plVar10 = (long *)*plVar10;
      *puVar11 = puVar7;
      puVar11 = puVar7;
    } while (plVar10 != (long *)0x0);
  }
LAB_001169d6:
  *(undefined8 **)(param_1 + 0x1b0) = local_40;
  return;
}




// Function: info_free_exclist @ 0x16ab0

void info_free_exclist(long param_1)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  
  __ptr = *(undefined8 **)(param_1 + 0x1b0);
  while (__ptr != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*__ptr;
    FUN_0013ebb0(__ptr[3]);
    free(__ptr);
    __ptr = puVar1;
  }
  *(undefined8 *)(param_1 + 0x1b0) = 0;
  return;
}




// Function: excluded_name @ 0x16b00

char excluded_name(char *param_1,long param_2)

{
  undefined8 *puVar1;
  char cVar2;
  void *__ptr;
  uint uVar3;
  char *pcVar4;
  char local_39;
  
  local_39 = FUN_0013ed70(DAT_00184b60,param_1);
  if ((local_39 == '\0') && (param_2 != 0)) {
    uVar3 = 0;
    __ptr = (void *)0x0;
    pcVar4 = (char *)0x0;
    do {
      for (puVar1 = *(undefined8 **)(param_2 + 0x1b0); puVar1 != (undefined8 *)0x0;
          puVar1 = (undefined8 *)*puVar1) {
        if ((uVar3 & *(uint *)(puVar1 + 2)) == 0) {
          cVar2 = FUN_0013ed70(puVar1[3],param_1);
          if (cVar2 == '\0') {
            if (pcVar4 == (char *)0x0) {
              cVar2 = *param_1;
              pcVar4 = param_1;
              while ((cVar2 == '.' && (pcVar4[1] == '/'))) {
                pcVar4 = pcVar4 + 2;
                cVar2 = *pcVar4;
              }
            }
            cVar2 = FUN_0013ed70(puVar1[3],pcVar4);
            if (cVar2 == '\0') {
              if (__ptr == (void *)0x0) {
                __ptr = (void *)FUN_0013e510(param_1);
                cVar2 = FUN_0013ed70(puVar1[3],__ptr);
              }
              else {
                cVar2 = FUN_0013ed70(puVar1[3],__ptr);
              }
              if (cVar2 == '\0') goto LAB_00116b70;
            }
          }
          local_39 = '\x01';
          goto LAB_00116bd5;
        }
LAB_00116b70:
      }
      param_2 = *(long *)(param_2 + 0x198);
      uVar3 = 2;
    } while (param_2 != 0);
LAB_00116bd5:
    free(__ptr);
  }
  return local_39;
}




// Function: exclude_vcs_ignores @ 0x16c40

void exclude_vcs_ignores(void)

{
  undefined **ppuVar1;
  undefined *puVar2;
  
  if (PTR_s__cvsignore_00181160 != (undefined *)0x0) {
    ppuVar1 = &PTR_s__cvsignore_00181160;
    puVar2 = PTR_s__cvsignore_00181160;
    do {
      ppuVar1 = ppuVar1 + 5;
      FUN_00116820(puVar2,0);
      puVar2 = *ppuVar1;
    } while (puVar2 != (undefined *)0x0);
    return;
  }
  return;
}




// Function: implemented @ 0x16c80

uint implemented(int param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 in_RAX;
  
  return (uint)CONCAT71((int7)((ulong)in_RAX >> 8),param_1 != 0x26) &
         CONCAT31((int3)((uint)param_3 >> 8),param_1 != 0x5f);
}




// Function: free_delayed_set_stat @ 0x16c90

void free_delayed_set_stat(void *param_1)

{
  free(*(void **)((long)param_1 + 0xa0));
  FUN_0011c600(*(undefined8 *)((long)param_1 + 0x90),*(undefined8 *)((long)param_1 + 0x88));
  free(*(void **)((long)param_1 + 0x60));
  free(*(void **)((long)param_1 + 0x68));
  free(*(void **)((long)param_1 + 0x78));
  free(param_1);
  return;
}




// Function: is_directory_link @ 0x16ce0

undefined4 is_directory_link(char *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  piVar4 = __errno_location();
  iVar1 = *piVar4;
  iVar2 = fstatat(DAT_00181234,param_1,&sStack_c8,0x100);
  if ((iVar2 == 0) && ((sStack_c8.st_mode & 0xf000) == 0xa000)) {
    iVar2 = fstatat(DAT_00181234,param_1,&sStack_c8,0);
    if (iVar2 == 0) {
      uVar3 = CONCAT31((int3)((sStack_c8.st_mode & 0xf000) >> 8),
                       (sStack_c8.st_mode & 0xf000) == 0x4000);
      goto LAB_00116d3d;
    }
  }
  uVar3 = 0;
LAB_00116d3d:
  *piVar4 = iVar1;
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_delayed_link_source @ 0x16da0

long * find_delayed_link_source(char *param_1)

{
  int iVar1;
  int *piVar2;
  long *plVar3;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  plVar3 = DAT_00184318;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184318 != (long *)0x0) {
    iVar1 = fstatat(DAT_00181234,param_1,&local_b8,0x100);
    if (iVar1 == 0) {
      do {
        if ((plVar3[1] == local_b8.st_dev) && (plVar3[2] == local_b8.st_ino)) goto LAB_00116e10;
        plVar3 = (long *)*plVar3;
      } while (plVar3 != (long *)0x0);
    }
    else {
      piVar2 = __errno_location();
      if (*piVar2 != 2) {
        plVar3 = (long *)0x0;
        FUN_00132d30(param_1);
        goto LAB_00116e10;
      }
    }
  }
  plVar3 = (long *)0x0;
LAB_00116e10:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return plVar3;
}




// Function: repair_delayed_set_stat @ 0x16e60

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void repair_delayed_set_stat(undefined8 param_1,__dev_t *param_2)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int iVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  puVar1 = DAT_00184320;
  do {
    if (puVar1 == (undefined8 *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar6 = FUN_0014aa70(param_1);
      uVar7 = dcgettext(0,"%s: Unexpected inconsistency when making directory",5);
      error(0,0,uVar7,uVar6);
      DAT_00184d58 = 2;
LAB_00116f87:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    iVar5 = fstatat(DAT_00181234,(char *)puVar1[0x14],&local_c8,*(int *)((long)puVar1 + 0x54));
    uVar4 = DAT_00184940;
    uVar3 = DAT_00184938;
    uVar7 = uRam0000000000184930;
    uVar6 = _DAT_00184928;
    if (iVar5 != 0) {
      FUN_00132d30(puVar1[0x14]);
      goto LAB_00116f87;
    }
    if ((local_c8.st_dev == *param_2) && (local_c8.st_ino == param_2[1])) {
      puVar1[1] = DAT_00184898;
      puVar1[2] = DAT_001848a0;
      *(undefined4 *)(puVar1 + 3) = DAT_001848b0;
      *(undefined4 *)((long)puVar1 + 0x1c) = DAT_001848b4;
      uVar2 = DAT_001848b8;
      *(undefined4 *)((long)puVar1 + 0x4c) = 0xffffffff;
      *(undefined4 *)(puVar1 + 4) = uVar2;
      *(undefined1 *)(puVar1 + 10) = 0;
      *(__mode_t *)(puVar1 + 9) = local_c8.st_mode;
      puVar1[5] = uVar6;
      puVar1[6] = uVar7;
      puVar1[7] = uVar3;
      puVar1[8] = uVar4;
      goto LAB_00116f87;
    }
    puVar1 = (undefined8 *)*puVar1;
  } while( true );
}




// Function: check_time @ 0x16fd0

void check_time(undefined8 param_1,long param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  long lVar6;
  long in_FS_OFFSET;
  long local_68;
  long local_60;
  undefined1 local_58 [40];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 < 0) {
    if ((DAT_00183b80._1_1_ & 0x80) != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = FUN_00122080(param_2,param_3,1);
      uVar3 = dcgettext(0,"%s: implausibly old time stamp %s",5);
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        error(0,0,uVar3,param_1,uVar2);
        return;
      }
      goto LAB_0011716d;
    }
  }
  else {
    iVar1 = FUN_0014be80(DAT_00184a10,DAT_00184a18,param_2,param_3);
    if (iVar1 < 0) {
      FUN_001411a0(&local_68);
      iVar1 = FUN_0014be80(local_68,local_60,param_2,param_3);
      if (iVar1 < 0) {
        lVar6 = param_2 - local_68;
        lVar5 = param_3 - local_60;
        if (lVar5 < 0) {
          lVar5 = lVar5 + 1000000000;
          lVar6 = lVar6 + -1;
        }
        if ((DAT_00183b80._1_1_ & 0x80) != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar2 = FUN_00124270(lVar6,lVar5,local_58);
          uVar3 = FUN_00122080(param_2,param_3,1);
          uVar4 = dcgettext(0,"%s: time stamp %s is %s s in the future",5);
          error(0,0,uVar4,param_1,uVar3,uVar2);
        }
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0011716d:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fd_chown @ 0x17180

int fd_chown(int param_1,char *param_2,__uid_t param_3,__gid_t param_4,int param_5)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (-1 < param_1) {
    iVar2 = fchown(param_1,param_3,param_4);
    if (iVar2 != 0) {
      piVar3 = __errno_location();
      cVar1 = FUN_00116c80(*piVar3);
      if (cVar1 == '\0') goto LAB_001171b9;
    }
    return iVar2;
  }
LAB_001171b9:
  iVar2 = fchownat(DAT_00181234,param_2,param_3,param_4,param_5);
  return iVar2;
}




// Function: fd_stat @ 0x171f0

void fd_stat(int param_1,char *param_2,stat *param_3,int param_4)

{
  if (-1 < param_1) {
    fstat(param_1,param_3);
    return;
  }
  fstatat(DAT_00181234,param_2,param_3,param_4);
  return;
}




// Function: fd_i_chmod @ 0x17210

int fd_i_chmod(int param_1,char *param_2,__mode_t param_3,int param_4)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (-1 < param_1) {
    iVar2 = fchmod(param_1,param_3);
    if (iVar2 != 0) {
      piVar3 = __errno_location();
      cVar1 = FUN_00116c80(*piVar3);
      if (cVar1 == '\0') goto LAB_00117246;
    }
    return iVar2;
  }
LAB_00117246:
  iVar2 = fchmodat(DAT_00181234,param_2,param_3,param_4);
  return iVar2;
}




// Function: fd_chmod @ 0x17280

undefined8 fd_chmod(undefined4 param_1,undefined8 param_2,uint param_3,int param_4,int param_5)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  bool bVar5;
  bool bVar6;
  
  iVar2 = FUN_00117210();
  bVar5 = param_5 != 0x32;
  bVar6 = param_4 != 0;
  if (iVar2 == 0) {
LAB_001172b5:
    if (!bVar5 || !bVar6) {
      return 0;
    }
    cVar1 = FUN_00116c80(0);
    if (cVar1 != '\0') {
      return 0;
    }
LAB_0011732b:
    iVar2 = FUN_00117210(param_1,param_2,param_3,0);
    if (iVar2 == 0) {
      return 0;
    }
    piVar4 = __errno_location();
    iVar2 = *piVar4;
LAB_001172e8:
    if (iVar2 == 0) {
      return 0;
    }
  }
  else {
    piVar4 = __errno_location();
    iVar2 = *piVar4;
    if (iVar2 != 1) {
LAB_001172e3:
      if (bVar5 && bVar6) {
        cVar1 = FUN_00116c80();
        if (cVar1 != '\0') {
          if (iVar2 == 0) {
            return 0;
          }
          goto LAB_00117310;
        }
        goto LAB_0011732b;
      }
      goto LAB_001172e8;
    }
    if (((param_3 & 0x800) != 0) && (iVar3 = thunk_FUN_00148740(), iVar3 == 0)) {
      iVar2 = FUN_00117210(param_1,param_2,param_3,param_4);
      if (iVar2 == 0) goto LAB_001172b5;
      iVar2 = *piVar4;
      goto LAB_001172e3;
    }
    if (bVar5 && bVar6) {
      cVar1 = FUN_00116c80(1);
      if (cVar1 != '\0') goto LAB_00117310;
      goto LAB_0011732b;
    }
  }
  if (param_5 == 0x32) {
    cVar1 = FUN_00116c80(iVar2);
    if (cVar1 == '\0') {
      return 0;
    }
    piVar4 = __errno_location();
  }
  else {
    piVar4 = __errno_location();
  }
LAB_00117310:
  *piVar4 = iVar2;
  return 0xffffffff;
}




// Function: set_mode @ 0x17410

void set_mode(undefined8 param_1,uint param_2,uint param_3,undefined4 param_4,uint param_5,
                 uint param_6,char param_7,undefined4 param_8)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined1 auStack_d8 [24];
  uint local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (((param_5 ^ param_2 | ~param_6) & param_3) != 0) {
    if ((~(param_6 | param_3) & 0xfff) != 0) {
      iVar1 = FUN_001171f0(param_4,param_1,auStack_d8,param_8);
      param_5 = local_c0;
      if (iVar1 != 0) {
        FUN_00132d30(param_1);
        goto LAB_00117479;
      }
    }
    param_3 = (param_2 ^ param_5 & 0xfff) & param_3;
    if (param_3 != 0) {
      param_3 = param_5 & 0xfff ^ param_3;
      iVar1 = FUN_00117280(param_4,param_1,param_3,param_8,(int)param_7);
      if (iVar1 != 0) {
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_00132550(param_1,param_3);
          return;
        }
        goto LAB_0011751c;
      }
    }
  }
LAB_00117479:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0011751c:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_stat @ 0x17530

void set_stat(undefined8 param_1,long param_2,undefined4 param_3,uint param_4,uint param_5,
                 char param_6,char param_7)

{
  undefined4 uVar1;
  undefined4 uVar2;
  char cVar3;
  int iVar4;
  int *piVar5;
  undefined8 uVar6;
  int iVar7;
  long in_FS_OFFSET;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  long local_40;
  
  iVar7 = (int)param_6;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184a68 == '\0') {
    uVar6 = 0x1ff;
    if (param_7 != '\0') goto LAB_00117588;
    if (DAT_00184b50 == '\0') {
      uStack_60 = 0x3ffffffe;
    }
    else {
      local_68 = *(undefined8 *)(param_2 + 0xe8);
      uStack_60 = *(undefined8 *)(param_2 + 0xf0);
    }
    local_58 = *(undefined8 *)(param_2 + 0xf8);
    uStack_50 = *(undefined8 *)(param_2 + 0x100);
    iVar4 = FUN_0013f360(param_3,DAT_00181234,param_1,&local_68);
    if (iVar4 == 0) {
      if (DAT_00184b50 != '\0') {
        FUN_00116fd0(param_1,local_68,uStack_60);
      }
      FUN_00116fd0(param_1,local_58,uStack_50);
    }
    else {
      if (param_6 == '2') {
        piVar5 = __errno_location();
        cVar3 = FUN_00116c80(*piVar5);
        if (cVar3 == '\0') goto LAB_0011769a;
      }
      FUN_00132dd0(param_1);
    }
LAB_0011769a:
    if (0 < DAT_00184aa8) {
LAB_001176aa:
      uVar1 = *(undefined4 *)(param_2 + 0x74);
      uVar2 = *(undefined4 *)(param_2 + 0x78);
      iVar4 = FUN_00117180(param_3,param_1,uVar1,uVar2);
      if (iVar4 == 0) {
        if (((~param_5 | param_4) & 0x49) != 0) {
          param_5 = param_5 & ~(param_4 & 0xc00);
        }
      }
      else {
        if (param_6 == '2') {
          piVar5 = __errno_location();
          cVar3 = FUN_00116c80(*piVar5);
          if (cVar3 == '\0') goto LAB_001176fb;
        }
        FUN_00132600(param_1,uVar1,uVar2);
      }
LAB_001176fb:
      uVar6 = 0xfff;
      if (DAT_00184aa4 < 1) {
        uVar6 = 0x1ff;
      }
      goto LAB_00117588;
    }
LAB_00117738:
    if ((0 < DAT_00184aa4) && (param_7 == '\0')) {
      uVar6 = 0xfff;
      goto LAB_00117588;
    }
  }
  else {
    if (DAT_00184aa8 < 1) goto LAB_00117738;
    if (param_7 == '\0') goto LAB_001176aa;
  }
  uVar6 = 0x1ff;
LAB_00117588:
  FUN_00117410(param_1,~DAT_00184328 & *(uint *)(param_2 + 0x70),uVar6,param_3,param_4,param_5);
  FUN_00131f20(param_2,param_1,iVar7,1);
  FUN_00131a70(param_2,param_1,iVar7);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  FUN_00131e70(param_2,param_1,iVar7);
  return;
}




// Function: apply_nonancestor_delayed_set_stat @ 0x17870

void apply_nonancestor_delayed_set_stat(char *param_1,byte param_2)

{
  ulong __n;
  undefined8 *puVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  byte bVar6;
  __mode_t _Var7;
  undefined4 uVar8;
  long in_FS_OFFSET;
  stat local_288;
  undefined1 local_1f8 [48];
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined4 local_188;
  undefined4 local_184;
  undefined4 local_180;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_a0;
  undefined8 local_98;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_1);
  if (DAT_00184320 != (undefined8 *)0x0) {
    bVar6 = 0;
    do {
      puVar1 = DAT_00184320;
      _Var7 = *(__mode_t *)(DAT_00184320 + 9);
      uVar8 = *(undefined4 *)((long)DAT_00184320 + 0x4c);
      bVar6 = bVar6 | *(byte *)(DAT_00184320 + 0xb);
      if ((param_2 < *(byte *)(DAT_00184320 + 0xb)) ||
         ((((__n = DAT_00184320[0x13], __n < sVar3 && (param_1[__n] != '\0')) &&
           ((param_1[__n] == '/' || (param_1[__n - 1] == '/')))) &&
          (iVar2 = memcmp(param_1,(void *)DAT_00184320[0x14],__n), iVar2 == 0)))) break;
      FUN_00124c80(*(undefined4 *)((long)puVar1 + 0x5c));
      if (bVar6 == 0) {
LAB_001178c0:
        local_188 = *(undefined4 *)(puVar1 + 3);
        local_110 = puVar1[5];
        uStack_108 = puVar1[6];
        local_100 = puVar1[7];
        uStack_f8 = puVar1[8];
        local_184 = *(undefined4 *)((long)puVar1 + 0x1c);
        local_180 = *(undefined4 *)(puVar1 + 4);
        local_1c8 = puVar1[0xc];
        local_1c0 = puVar1[0xd];
        local_1b8 = puVar1[0xe];
        local_1b0 = puVar1[0xf];
        local_1a8 = puVar1[0x10];
        local_98 = puVar1[0x12];
        local_a0 = puVar1[0x11];
        FUN_00117530(puVar1[0x14],local_1f8,0xffffffff,_Var7,uVar8,0x35,*(undefined1 *)(puVar1 + 10)
                    );
      }
      else {
        iVar2 = fstatat(DAT_00181234,(char *)puVar1[0x14],&local_288,*(int *)((long)puVar1 + 0x54));
        if (iVar2 == 0) {
          if ((local_288.st_dev == puVar1[1]) && (local_288.st_ino == puVar1[2])) {
            uVar8 = 0xffffffff;
            _Var7 = local_288.st_mode;
            goto LAB_001178c0;
          }
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = FUN_0014aa70(puVar1[0x14]);
          uVar5 = dcgettext(0,"%s: Directory renamed before its status could be extracted",5);
          error(0,0,uVar5,uVar4);
          DAT_00184d58 = 2;
        }
        else {
          FUN_00132d30(puVar1[0x14]);
        }
      }
      DAT_00184320 = (undefined8 *)*puVar1;
      FUN_00116c90(puVar1);
    } while (DAT_00184320 != (undefined8 *)0x0);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: mark_after_links @ 0x17b10

void mark_after_links(undefined8 *param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  long local_b8;
  undefined8 local_b0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    *(undefined1 *)(param_1 + 0xb) = 1;
    iVar1 = FUN_001246f0(param_1[0x14],&local_b8);
    if (iVar1 == 0) {
      param_1[1] = local_b8;
      param_1[2] = local_b0;
      param_1 = (undefined8 *)*param_1;
    }
    else {
      FUN_00132d30(param_1[0x14]);
      param_1 = (undefined8 *)*param_1;
    }
  } while ((param_1 != (undefined8 *)0x0) && (*(char *)(param_1 + 0xb) == '\0'));
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fixup_delayed_set_stat @ 0x17bb0

void fixup_delayed_set_stat(char *param_1,char *param_2)

{
  undefined8 *puVar1;
  char *__s1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  size_t sVar5;
  
  iVar2 = DAT_00184580;
  puVar1 = DAT_00184320;
  do {
    if (puVar1 == (undefined8 *)0x0) {
      return;
    }
    if (*(int *)((long)puVar1 + 0x5c) == iVar2) {
      __s1 = (char *)puVar1[0x14];
      iVar3 = strcmp(__s1,param_1);
      if (iVar3 == 0) {
        free(__s1);
        uVar4 = FUN_0014c7a0(param_2);
        puVar1[0x14] = uVar4;
        sVar5 = strlen(param_2);
        puVar1[0x13] = sVar5;
        return;
      }
    }
    puVar1 = (undefined8 *)*puVar1;
  } while( true );
}




// Function: delay_set_stat @ 0x17c30

void delay_set_stat(char *param_1,long param_2,undefined4 param_3,undefined4 param_4,
                 undefined4 param_5,undefined4 param_6)

{
  undefined4 uVar1;
  char *__s1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  char cVar7;
  int iVar8;
  size_t sVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  long lVar12;
  long in_FS_OFFSET;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar9 = strlen(param_1);
  for (puVar10 = DAT_00184320; puVar10 != (undefined8 *)0x0; puVar10 = (undefined8 *)*puVar10) {
    __s1 = (char *)puVar10[0x14];
    iVar8 = strcmp(__s1,param_1);
    if (iVar8 == 0) {
      if (*(char *)(puVar10 + 10) != '\0') {
        iVar8 = fstatat(DAT_00181234,__s1,&local_d8,*(int *)((long)puVar10 + 0x54));
        if (iVar8 == 0) {
          puVar10[1] = local_d8.st_dev;
          puVar10[2] = local_d8.st_ino;
        }
        else {
          FUN_00132d30(puVar10[0x14]);
        }
      }
      *(undefined4 *)(puVar10 + 3) = param_5;
      if (param_2 == 0) goto LAB_00117e08;
      goto LAB_00117cc6;
    }
  }
  puVar10 = (undefined8 *)FUN_0014c5b0(0xa8);
  puVar6 = puVar10;
  *puVar10 = DAT_00184320;
  DAT_00184320 = puVar6;
  puVar10[0x13] = sVar9;
  uVar11 = FUN_0014c7a0(param_1);
  *(undefined1 *)(puVar10 + 0xb) = 0;
  puVar10[0x14] = uVar11;
  if (param_2 == 0) {
    *(undefined4 *)(puVar10 + 3) = param_5;
LAB_00117e08:
    *(undefined4 *)(puVar10 + 9) = param_3;
    *(undefined1 *)(puVar10 + 10) = 1;
    *(undefined4 *)((long)puVar10 + 0x4c) = param_4;
    uVar1 = DAT_00184580;
    *(undefined4 *)((long)puVar10 + 0x54) = param_6;
    *(undefined4 *)((long)puVar10 + 0x5c) = uVar1;
    puVar10[0xc] = 0;
    puVar10[0xd] = 0;
    puVar10[0xe] = 0;
    puVar10[0xf] = 0;
    puVar10[0x10] = 0;
    puVar10[0x12] = 0;
    puVar10[0x11] = 0;
    cVar7 = FUN_001244e0(param_1);
    goto joined_r0x00117e71;
  }
  puVar10[1] = *(undefined8 *)(param_2 + 0x58);
  puVar10[2] = *(undefined8 *)(param_2 + 0x60);
  *(undefined4 *)(puVar10 + 3) = param_5;
LAB_00117cc6:
  uVar11 = *(undefined8 *)(param_2 + 0x30);
  uVar2 = *(undefined8 *)(param_2 + 0xe8);
  uVar3 = *(undefined8 *)(param_2 + 0xf0);
  uVar4 = *(undefined8 *)(param_2 + 0xf8);
  uVar5 = *(undefined8 *)(param_2 + 0x100);
  *(undefined4 *)((long)puVar10 + 0x1c) = *(undefined4 *)(param_2 + 0x74);
  uVar1 = *(undefined4 *)(param_2 + 0x78);
  *(undefined4 *)(puVar10 + 9) = param_3;
  *(undefined4 *)(puVar10 + 4) = uVar1;
  *(undefined1 *)(puVar10 + 10) = 0;
  *(undefined4 *)((long)puVar10 + 0x4c) = param_4;
  uVar1 = DAT_00184580;
  *(undefined4 *)((long)puVar10 + 0x54) = param_6;
  *(undefined4 *)((long)puVar10 + 0x5c) = uVar1;
  puVar10[0xc] = 0;
  puVar10[5] = uVar2;
  puVar10[6] = uVar3;
  puVar10[7] = uVar4;
  puVar10[8] = uVar5;
  FUN_00123be0(puVar10 + 0xc,uVar11);
  if (*(long *)(param_2 + 0x38) == 0) {
    lVar12 = *(long *)(param_2 + 0x48);
    puVar10[0xd] = 0;
    puVar10[0xe] = 0;
    if (lVar12 != 0) goto LAB_00117d61;
LAB_00117dee:
    puVar10[0xf] = 0;
    uVar11 = 0;
  }
  else {
    uVar11 = FUN_0014c770(*(long *)(param_2 + 0x38),*(long *)(param_2 + 0x40) + 1);
    lVar12 = *(long *)(param_2 + 0x48);
    puVar10[0xd] = uVar11;
    puVar10[0xe] = *(undefined8 *)(param_2 + 0x40);
    if (lVar12 == 0) goto LAB_00117dee;
LAB_00117d61:
    uVar11 = FUN_0014c770(lVar12,*(long *)(param_2 + 0x50) + 1);
    puVar10[0xf] = uVar11;
    uVar11 = *(undefined8 *)(param_2 + 0x50);
  }
  puVar10[0x10] = uVar11;
  FUN_0011c840(param_2,puVar10 + 0x12,puVar10 + 0x11);
  cVar7 = FUN_001244e0(param_1);
joined_r0x00117e71:
  if (cVar7 == '\0') {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00117b10(puVar10);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: make_directories @ 0x17f70

int make_directories(char *param_1,undefined1 *param_2)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  char *pcVar5;
  char *pcVar6;
  uint uVar7;
  uint uVar8;
  long in_FS_OFFSET;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar5 = param_1;
  if (*param_1 != '\0') {
LAB_00117fb0:
    cVar1 = pcVar5[1];
    pcVar6 = pcVar5;
    if (cVar1 != '\0') {
      while( true ) {
        uVar8 = DAT_0018432c;
        pcVar5 = pcVar6 + 1;
        if ((((param_1 == pcVar5) || (cVar1 != '/')) || (*pcVar6 == '/')) ||
           ((*pcVar6 == '.' &&
            (((pcVar5 == param_1 + 1 || (pcVar6[-1] == '/')) ||
             ((pcVar6[-1] == '.' && ((pcVar5 == param_1 + 2 || (pcVar6[-2] == '/'))))))))))
        goto LAB_00117fb0;
        *pcVar5 = '\0';
        uVar7 = ~uVar8 & 0x1ff;
        uVar8 = uVar7;
        if (DAT_00184330 == '\0') {
          uVar8 = uVar7 | 0xc0;
        }
        iVar2 = mkdirat(DAT_00181234,param_1,uVar8);
        if (iVar2 == 0) break;
        piVar4 = __errno_location();
        iVar2 = *piVar4;
        if ((iVar2 != 0x11) && (iVar3 = fstatat(DAT_00181234,param_1,&local_d8,0), iVar3 != 0)) {
          *piVar4 = iVar2;
          FUN_00132760(param_1);
          *pcVar5 = '/';
          goto LAB_00118063;
        }
        *pcVar5 = '/';
        cVar1 = pcVar6[2];
        pcVar6 = pcVar5;
        if (cVar1 == '\0') goto LAB_00118060;
      }
      FUN_00117c30(param_1,0,~DAT_00184328 & uVar8,0x1ff,uVar7,0x100);
      FUN_00122a00(param_1,(int)pcVar5 - (int)param_1,uVar7);
      *param_2 = 1;
      *pcVar5 = '/';
      goto LAB_00117fb0;
    }
  }
LAB_00118060:
  iVar3 = 0;
LAB_00118063:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: open_output_file @ 0x18140

int open_output_file(char *param_1,int param_2,uint param_3,int param_4,uint *param_5,
                undefined4 *param_6)

{
  int __fd;
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  uint __oflag;
  long in_FS_OFFSET;
  stat sStack_d8;
  long local_40;
  
  iVar1 = DAT_00184b34;
  __oflag = 0x809c1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184b34 == 2) {
    __oflag = (-(uint)(DAT_00184b69 == '\0') & 0x20000) + 0x80b41;
  }
  if (param_4 != 0) {
    __oflag = __oflag & 0xffffff7f;
  }
  if (((param_2 == 0x37) && (DAT_00184310 == 0)) &&
     (DAT_00184310 = 1, ((byte)DAT_00183b80 & 8) != 0)) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = dcgettext(0,"Extracting contiguous files as regular files",5);
    error(0,0,uVar2);
  }
  __fd = openat(DAT_00181234,param_1,__oflag,(ulong)param_3);
  if (-1 < __fd) {
    if (iVar1 == 2) {
      iVar1 = fstat(__fd,&sStack_d8);
      if (iVar1 == 0) {
        if ((sStack_d8.st_mode & 0xf000) == 0x8000) {
          *param_5 = sStack_d8.st_mode;
          *param_6 = 0xffffffff;
          goto LAB_001181e8;
        }
        close(__fd);
        piVar3 = __errno_location();
        *piVar3 = 0x11;
      }
      else {
        piVar3 = __errno_location();
        iVar1 = *piVar3;
        close(__fd);
        *piVar3 = iVar1;
      }
      __fd = -1;
    }
    else {
      *param_5 = ~DAT_00184328 & param_3;
      *param_6 = 0x1ff;
    }
  }
LAB_001181e8:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return __fd;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_direct_ancestor @ 0x182e0

undefined8 * find_direct_ancestor(char *param_1)

{
  undefined8 *puVar1;
  size_t __n;
  int iVar2;
  char *pcVar3;
  
  puVar1 = DAT_00184320;
  do {
    if (puVar1 == (undefined8 *)0x0) {
      return (undefined8 *)0x0;
    }
    if (*(char *)(puVar1 + 0xb) == '\0') {
      __n = puVar1[0x13];
      iVar2 = strncmp(param_1,(char *)puVar1[0x14],__n);
      if (((iVar2 == 0) && (param_1[__n] == '/')) &&
         (pcVar3 = (char *)FUN_0013e390(param_1), pcVar3 == param_1 + __n + 1)) {
        return puVar1;
      }
    }
    puVar1 = (undefined8 *)*puVar1;
  } while( true );
}




// Function: apply_delayed_links @ 0x18350

void apply_delayed_links(void)

{
  long *__file;
  char *__from;
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *__ptr;
  int iVar3;
  long *plVar4;
  long *__from_00;
  long in_FS_OFFSET;
  undefined1 auVar5 [16];
  stat local_288;
  undefined1 local_1f8 [48];
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined4 local_188;
  undefined4 local_184;
  undefined4 local_180;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_a0;
  undefined8 local_98;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = DAT_00184318;
  do {
    if (__ptr == (undefined8 *)0x0) {
      DAT_00184318 = (undefined8 *)0x0;
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    __from_00 = (long *)0x0;
    __from = (char *)(__ptr + 0x14);
    FUN_00124c80(*(undefined4 *)(__ptr + 0xb));
    plVar4 = (long *)__ptr[0xc];
    if (plVar4 != (long *)0x0) {
      do {
        while( true ) {
          __file = plVar4 + 1;
          iVar3 = fstatat(DAT_00181234,(char *)__file,&local_288,0x100);
          if (((iVar3 == 0) && (local_288.st_dev == __ptr[1])) && (local_288.st_ino == __ptr[2]))
          break;
LAB_001183b0:
          plVar4 = (long *)*plVar4;
          if (plVar4 == (long *)0x0) goto LAB_00118490;
        }
        auVar5 = FUN_0014bd30(&local_288);
        iVar3 = FUN_0014be80(auVar5._0_8_,auVar5._8_8_,__ptr[3],__ptr[4]);
        if (iVar3 != 0) goto LAB_001183b0;
        iVar3 = unlinkat(DAT_00181234,(char *)__file,0);
        if (iVar3 != 0) {
          FUN_00132db0(__file);
          goto LAB_001183b0;
        }
        if ((__from_00 != (long *)0x0) &&
           (iVar3 = linkat(DAT_00181234,(char *)__from_00,DAT_00181234,(char *)__file,0), iVar3 == 0
           )) goto LAB_001183b0;
        if (*(char *)(__ptr + 5) != '\0') {
          iVar3 = symlinkat(__from,DAT_00181234,(char *)__file);
          if (iVar3 == 0) {
            local_188 = *(undefined4 *)((long)__ptr + 0x2c);
            local_110 = __ptr[7];
            uStack_108 = __ptr[8];
            local_100 = __ptr[9];
            uStack_f8 = __ptr[10];
            local_184 = *(undefined4 *)(__ptr + 6);
            local_180 = *(undefined4 *)((long)__ptr + 0x34);
            local_1c8 = __ptr[0xd];
            local_1c0 = __ptr[0xe];
            local_1b8 = __ptr[0xf];
            local_1b0 = __ptr[0x10];
            local_1a8 = __ptr[0x11];
            local_98 = __ptr[0x13];
            local_a0 = __ptr[0x12];
            FUN_00117530(__file,local_1f8,0xffffffff,0,0,0x32,0,0x100);
            __from_00 = __file;
          }
          else {
            FUN_00132c90(__from,__file);
          }
          goto LAB_001183b0;
        }
        iVar3 = linkat(DAT_00181234,__from,DAT_00181234,(char *)__file,0);
        if (iVar3 == 0) goto LAB_001183b0;
        FUN_001326e0(__from,__file);
        plVar4 = (long *)*plVar4;
      } while (plVar4 != (long *)0x0);
LAB_00118490:
      puVar2 = (undefined8 *)__ptr[0xc];
      while (puVar2 != (undefined8 *)0x0) {
        puVar1 = (undefined8 *)*puVar2;
        free(puVar2);
        puVar2 = puVar1;
      }
    }
    FUN_0011c600(__ptr[0x13],__ptr[0x12]);
    free((void *)__ptr[0xd]);
    puVar2 = (undefined8 *)*__ptr;
    free(__ptr);
    __ptr = puVar2;
  } while( true );
}




// Function: extract_finish @ 0x18640

void extract_finish(void)

{
  FUN_00117870(&DAT_00167708,0);
  FUN_00118350();
  FUN_00117870(&DAT_00167708,1);
  return;
}




// Function: maybe_recoverable @ 0x18750

undefined8 maybe_recoverable(char *param_1,char param_2,char *param_3)

{
  uint uVar1;
  char cVar2;
  int iVar3;
  uint *puVar4;
  undefined8 uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined1 auStack_c8 [152];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar5 = 0;
  if (*param_3 != '\0') goto LAB_001187c6;
  puVar4 = (uint *)__errno_location();
  uVar1 = *puVar4;
  if (uVar1 == 0x11) {
LAB_00118812:
    if (DAT_00184b34 == 5) {
      uVar5 = 2;
      if ((DAT_00183b80._2_1_ & 0x10) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar5 = dcgettext(0,"%s: skipping existing file",5);
        error(0,0,uVar5,param_1);
        uVar5 = 2;
      }
      goto LAB_001187c6;
    }
    if (DAT_00184b34 < 6) {
      if (2 < DAT_00184b34) {
        uVar5 = 0;
        if (DAT_00184b34 == 4) goto LAB_001187c6;
        goto LAB_001187f5;
      }
LAB_00118850:
      iVar3 = FUN_00125490(param_1,0);
      if (iVar3 < 1) goto LAB_001187f5;
    }
    else {
      if ((DAT_00184b34 == 6) && (cVar2 = FUN_00118670(param_1), cVar2 == '\0')) goto LAB_00118850;
LAB_001187f5:
      iVar3 = FUN_00117f70(param_1,param_3);
      if ((iVar3 != 0) || (*param_3 == '\0')) goto LAB_001187c0;
    }
    uVar5 = 1;
  }
  else {
    if ((int)uVar1 < 0x12) {
      if (uVar1 == 2) goto LAB_001187f5;
    }
    else if ((((uVar1 == 0x28) || ((uVar1 & 0xffffffbf) == 0x1f)) && (param_2 != '\0')) &&
            ((DAT_00184b34 == 2 && (DAT_00184b69 == '\0')))) {
      pcVar6 = strchr(param_1,0x2f);
      if (pcVar6 == (char *)0x0) goto LAB_00118850;
      iVar3 = FUN_001246f0(param_1,auStack_c8);
      if (iVar3 == 0) goto LAB_00118812;
    }
LAB_001187c0:
    *puVar4 = uVar1;
    uVar5 = 0;
  }
LAB_001187c6:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: extract_fifo @ 0x18900

undefined4 extract_fifo(char *param_1,char param_2)

{
  int iVar1;
  undefined4 uVar2;
  uint uVar3;
  long in_FS_OFFSET;
  undefined1 local_31;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_31 = 0;
  uVar3 = DAT_001848b0;
  if (0 < DAT_00184aa8) {
    uVar3 = DAT_001848b0 & 0xffffffc0;
  }
  while( true ) {
    iVar1 = mkfifoat(DAT_00181234,param_1,uVar3 & 0x1ff);
    if (iVar1 == 0) {
      FUN_00117530(param_1,&DAT_00184840,0xffffffff,~DAT_00184328 & uVar3 & 0x1ff,0x1ff,(int)param_2
                   ,0,0x100);
      uVar2 = 0;
      goto LAB_00118972;
    }
    iVar1 = FUN_00118750(param_1,0,&local_31);
    if (iVar1 == 0) break;
    if (iVar1 == 2) {
      uVar2 = 0;
LAB_00118972:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  FUN_00132780(param_1);
  uVar2 = 0xffffffff;
  goto LAB_00118972;
}




// Function: extract_node @ 0x189e0

undefined4 extract_node(char *param_1,char param_2)

{
  int iVar1;
  undefined4 uVar2;
  uint uVar3;
  long in_FS_OFFSET;
  undefined1 local_31;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_31 = 0;
  uVar3 = DAT_001848b0;
  if (0 < DAT_00184aa8) {
    uVar3 = DAT_001848b0 & 0xffffffc0;
  }
  while( true ) {
    iVar1 = mknodat(DAT_00181234,param_1,uVar3 & 0x61ff,DAT_001848c0);
    if (iVar1 == 0) {
      FUN_00117530(param_1,&DAT_00184840,0xffffffff,~DAT_00184328 & uVar3 & 0x61ff,0x1ff,
                   (int)param_2,0,0x100);
      uVar2 = 0;
      goto LAB_00118a59;
    }
    iVar1 = FUN_00118750(param_1,0,&local_31);
    if (iVar1 == 0) break;
    if (iVar1 == 2) {
      uVar2 = 0;
LAB_00118a59:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  FUN_001327a0(param_1);
  uVar2 = 0xffffffff;
  goto LAB_00118a59;
}




// Function: create_placeholder_file @ 0x18ad0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 create_placeholder_file(char *param_1,char param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int iVar6;
  int iVar7;
  int *piVar8;
  long lVar9;
  size_t sVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  undefined4 uVar13;
  long in_FS_OFFSET;
  undefined1 auVar14 [16];
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    iVar6 = openat(DAT_00181234,param_1,0xc1,0);
    if (-1 < iVar6) {
      iVar7 = fstat(iVar6,&local_d8);
      if (iVar7 != 0) {
        FUN_00132d30(param_1);
        uVar13 = 0xffffffff;
        close(iVar6);
        goto LAB_00118b41;
      }
      iVar6 = close(iVar6);
      if (iVar6 != 0) {
        uVar13 = 0xffffffff;
        FUN_00132680(param_1);
        goto LAB_00118b41;
      }
      sVar10 = strlen(DAT_00184858);
      puVar11 = (undefined8 *)FUN_0014c5b0(sVar10 + 0xa1);
      if (param_4 == (undefined8 *)0x0) {
        puVar12 = puVar11;
        *puVar11 = DAT_00184318;
        DAT_00184318 = puVar12;
      }
      else {
        uVar1 = *param_4;
        *param_4 = puVar11;
        *puVar11 = uVar1;
      }
      puVar11[1] = local_d8.st_dev;
      puVar11[2] = local_d8.st_ino;
      auVar14 = FUN_0014bd30(&local_d8);
      *(char *)(puVar11 + 5) = param_2;
      *(undefined1 (*) [16])(puVar11 + 3) = auVar14;
      uVar5 = DAT_00184940;
      uVar4 = DAT_00184938;
      uVar3 = uRam0000000000184930;
      uVar1 = _DAT_00184928;
      if (param_2 != '\0') {
        *(undefined4 *)((long)puVar11 + 0x2c) = DAT_001848b0;
        uVar2 = _DAT_001848b4;
        puVar11[7] = uVar1;
        puVar11[8] = uVar3;
        puVar11[6] = uVar2;
        puVar11[9] = uVar4;
        puVar11[10] = uVar5;
      }
      *(undefined4 *)(puVar11 + 0xb) = DAT_00184580;
      sVar10 = strlen(param_1);
      puVar12 = (undefined8 *)FUN_0014c5b0(sVar10 + 9);
      puVar11[0xc] = puVar12;
      *puVar12 = 0;
      strcpy((char *)(puVar12 + 1),param_1);
      uVar1 = DAT_00184870;
      puVar11[0xd] = 0;
      FUN_00123be0(puVar11 + 0xd,uVar1);
      puVar11[0xe] = 0;
      puVar11[0xf] = 0;
      puVar11[0x10] = 0;
      puVar11[0x11] = 0;
      FUN_0011c840(&DAT_00184840,puVar11 + 0x13,puVar11 + 0x12);
      strcpy((char *)(puVar11 + 0x14),DAT_00184858);
      lVar9 = FUN_001182e0(param_1);
      if (lVar9 != 0) {
        FUN_00117b10(lVar9);
        uVar13 = 0;
        goto LAB_00118b41;
      }
      break;
    }
    piVar8 = __errno_location();
    if ((*piVar8 == 0x11) && (lVar9 = FUN_00116da0(param_1), lVar9 != 0)) break;
    iVar6 = FUN_00118750(param_1,0,param_3);
    if (iVar6 == 0) {
      uVar13 = 0xffffffff;
      FUN_001327c0(param_1);
      goto LAB_00118b41;
    }
  } while (iVar6 != 2);
  uVar13 = 0;
LAB_00118b41:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar13;
}




// Function: extract_link @ 0x18d40

undefined4 extract_link(char *param_1)

{
  char *__from;
  char cVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  long lVar5;
  int *piVar6;
  size_t sVar7;
  long *plVar8;
  long *plVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  undefined1 local_159;
  stat local_158;
  stat local_c8;
  long local_30;
  
  __from = DAT_00184858;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_159 = 0;
  if ((DAT_00184b92 == '\0') && (cVar1 = FUN_00127cc0(DAT_00184858), cVar1 != '\0')) {
    uVar4 = FUN_00118ad0(param_1,0,&local_159,0);
  }
  else {
    lVar5 = FUN_00116da0(__from);
    if (lVar5 == 0) {
      piVar6 = __errno_location();
      do {
        iVar3 = linkat(DAT_00181234,__from,DAT_00181234,param_1,0);
        plVar9 = DAT_00184318;
        iVar2 = *piVar6;
        if (iVar3 == 0) {
          if (DAT_00184318 == (long *)0x0) goto LAB_00118e68;
          iVar3 = fstatat(DAT_00181234,__from,&local_158,0x100);
          iVar2 = DAT_00184580;
          if (iVar3 == 0) goto LAB_00118ef4;
          goto LAB_00118e68;
        }
        if (((iVar2 == 0x11) && (iVar3 = strcmp(__from,param_1), iVar3 == 0)) ||
           (((iVar3 = fstatat(DAT_00181234,__from,&local_158,0x100), iVar3 == 0 &&
             ((iVar3 = fstatat(DAT_00181234,param_1,&local_c8,0x100), iVar3 == 0 &&
              (local_158.st_dev == local_c8.st_dev)))) && (local_158.st_ino == local_c8.st_ino))))
        goto LAB_00118e68;
        *piVar6 = iVar2;
        iVar2 = FUN_00118750(param_1,0,&local_159);
      } while (iVar2 == 1);
      uVar4 = 0;
      if ((iVar2 != 2) && ((DAT_00184b50 == '\0' || (*piVar6 != 0x11)))) {
        FUN_001326e0(__from,param_1);
        uVar4 = 1;
      }
    }
    else {
      uVar4 = FUN_00118ad0(param_1,0,&local_159,lVar5);
    }
  }
  goto LAB_00118e6b;
LAB_00118ef4:
  do {
    if ((((int)plVar9[0xb] == iVar2) && (plVar9[1] == local_158.st_dev)) &&
       (plVar9[2] == local_158.st_ino)) {
      auVar10 = FUN_0014bd30(&local_158);
      iVar3 = FUN_0014be80(plVar9[3],plVar9[4],auVar10._0_8_,auVar10._8_8_);
      if (iVar3 == 0) {
        sVar7 = strlen(param_1);
        plVar8 = (long *)FUN_0014c5b0(sVar7 + 9);
        strcpy((char *)(plVar8 + 1),param_1);
        *plVar8 = plVar9[0xc];
        plVar9[0xc] = (long)plVar8;
        break;
      }
    }
    plVar9 = (long *)*plVar9;
  } while (plVar9 != (long *)0x0);
LAB_00118e68:
  uVar4 = 0;
LAB_00118e6b:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: extract_symlink @ 0x18fc0

undefined8 extract_symlink(char *param_1)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_21 = 0;
  if ((DAT_00184b92 == '\0') && ((*DAT_00184858 == '/' || (cVar1 = FUN_00127cc0(), cVar1 != '\0'))))
  {
    uVar3 = FUN_00118ad0(param_1,1,&local_21,0);
  }
  else {
    do {
      iVar2 = symlinkat(DAT_00184858,DAT_00181234,param_1);
      if (iVar2 == 0) {
        FUN_00117530(param_1,&DAT_00184840,0xffffffff,0,0,0x32,0,0x100);
        uVar3 = 0;
        goto LAB_00119054;
      }
      iVar2 = FUN_00118750(param_1,0,&local_21);
      if (iVar2 == 0) {
        FUN_00132c90(DAT_00184858,param_1);
        uVar3 = 0xffffffff;
        goto LAB_00119054;
      }
    } while (iVar2 != 2);
    uVar3 = 0;
  }
LAB_00119054:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: extract_file @ 0x191d0

int extract_file(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  int iVar2;
  long lVar3;
  ulong uVar4;
  int *piVar5;
  ulong uVar6;
  undefined8 uVar7;
  uint uVar8;
  long in_FS_OFFSET;
  char local_6c;
  undefined1 local_51;
  undefined4 local_50;
  undefined4 local_4c;
  ulong local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_51 = 0;
  uVar8 = DAT_001848b0 & 0x1c0;
  if (DAT_00184aa8 < 1) {
    uVar8 = DAT_001848b0 & 0x1ff;
  }
  local_50 = 0;
  local_4c = 0;
  local_6c = (char)param_2;
  if (DAT_00184a6a == '\0') {
    if (DAT_00184a60 == 0) {
      local_48 = local_48 & 0xffffffff00000000;
      iVar1 = FUN_001190d0(param_1,0,(int)local_6c,&local_48);
      uVar4 = local_48;
      if (iVar1 == 0) {
        do {
          iVar1 = FUN_00118140(param_1,param_2,uVar8,uVar4 & 0xffffffff,&local_50,&local_4c);
          if (-1 < iVar1) goto LAB_0011926c;
          iVar1 = FUN_00118750(param_1,1,&local_51);
        } while (iVar1 == 1);
        FUN_00122bd0();
        iVar2 = 0;
        if (iVar1 != 2) {
          iVar2 = 1;
          FUN_001327c0(param_1);
        }
      }
      else {
        iVar2 = 1;
      }
      goto LAB_0011933f;
    }
    iVar1 = FUN_0012b640(param_1,0x66,&DAT_00184840);
    if (iVar1 < 0) {
      FUN_00122bd0();
      iVar2 = 0;
      goto LAB_0011933f;
    }
  }
  else {
    iVar1 = 1;
  }
LAB_0011926c:
  FUN_0010ccb0(&DAT_00184840);
  if (DAT_00184960 == '\0') {
    local_48 = DAT_001848c8;
    do {
      if ((long)local_48 < 1) goto LAB_001192fa;
      FUN_0010ccf0();
      lVar3 = FUN_0010e130();
      if (lVar3 == 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar7 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar7);
        DAT_00184d58 = 2;
        goto LAB_001192fa;
      }
      uVar4 = FUN_0010d2b0(lVar3);
      if (local_48 < uVar4) {
        uVar4 = local_48;
      }
      piVar5 = __errno_location();
      *piVar5 = 0;
      uVar6 = FUN_001249f0(iVar1,lVar3,uVar4);
      local_48 = local_48 - uVar4;
      FUN_0010d1f0(lVar3 + -1 + uVar4);
    } while (uVar4 == uVar6);
    if (DAT_00184a60 == 0) {
      FUN_00132e30(param_1,uVar6,uVar4);
    }
  }
  else {
    FUN_00129ed0(iVar1,&DAT_00184840,&local_48);
  }
LAB_001192fa:
  FUN_00122b10(local_48);
  iVar2 = 0;
  FUN_0010ccd0();
  if (DAT_00184a6a == '\0') {
    if (DAT_00184a60 == 0) {
      FUN_00117530(param_1,&DAT_00184840,iVar1,local_50,local_4c,(int)local_6c,0,
                   (ulong)(DAT_00184b34 != 2) << 8);
      iVar2 = close(iVar1);
    }
    else {
      iVar2 = close(iVar1);
    }
    if (iVar2 < 0) {
      FUN_00132680(param_1);
    }
    if (DAT_00184a60 != 0) {
      FUN_0012b6f0();
    }
  }
LAB_0011933f:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: prepare_to_extract @ 0x19540

undefined8 prepare_to_extract(undefined8 param_1,undefined4 param_2,undefined8 *param_3)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint *puVar5;
  code *pcVar6;
  
  switch(param_2) {
  case 0:
  case 0x30:
  case 0x37:
    if (DAT_00184850 == '\0') goto switchD_00119567_caseD_53;
    pcVar6 = FUN_00119850;
    break;
  default:
    if ((DAT_00183b80._2_1_ & 1) != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = FUN_0014aa70(param_1);
      uVar4 = dcgettext(0,"%s: Unknown file type \'%c\', extracted as normal file",5);
      error(0,0,uVar4,uVar3,param_2);
    }
  case 0x53:
switchD_00119567_caseD_53:
    if ((DAT_00184a6a == '\0') && (pcVar6 = FUN_001191d0, DAT_00184a60 == 0)) goto LAB_00119631;
    goto LAB_00119602;
  case 0x31:
    pcVar6 = FUN_00118d40;
    break;
  case 0x32:
    pcVar6 = FUN_00118fc0;
    break;
  case 0x33:
    DAT_001848b0 = DAT_001848b0 | 0x2000;
    pcVar6 = FUN_001189e0;
    break;
  case 0x34:
    DAT_001848b0 = DAT_001848b0 | 0x6000;
    pcVar6 = FUN_001189e0;
    break;
  case 0x35:
  case 0x44:
    pcVar6 = FUN_00119850;
    if (DAT_001849c8 != '\0') {
      DAT_001847e0 = 1;
    }
    break;
  case 0x36:
    pcVar6 = FUN_00118900;
    break;
  case 0x4b:
  case 0x4c:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar3 = dcgettext(0,"Unexpected long name header",5);
    error(0,0,uVar3);
    DAT_00184d58 = 2;
    return 0;
  case 0x4d:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar3 = FUN_0014aa70(DAT_00184848);
    uVar4 = dcgettext(0,"%s: Cannot extract -- file is continued from another volume",5);
    error(0,0,uVar4,uVar3);
    DAT_00184d58 = 2;
    return 0;
  case 0x56:
    goto switchD_00119567_caseD_56;
  }
  if (DAT_00184a6a != '\0') {
switchD_00119567_caseD_56:
    return 0;
  }
  if (DAT_00184a60 == 0) {
LAB_00119631:
    if (DAT_00184b34 == 3) {
      iVar2 = FUN_00125490(param_1,DAT_00184aba);
      if ((iVar2 == 0) && (puVar5 = (uint *)__errno_location(), (*puVar5 & 0xfffffffd) != 0)) {
        FUN_00132db0(param_1);
        return 0;
      }
    }
    else if ((DAT_00184b34 == 6) && (cVar1 = FUN_00118670(param_1,0), cVar1 != '\0')) {
      if ((DAT_00183b80._1_1_ & 8) == 0) {
        return 0;
      }
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = FUN_0014ac50(param_1);
      uVar4 = dcgettext(0,"Current %s is newer or same age",5);
      error(0,0,uVar4,uVar3);
      return 0;
    }
  }
  else {
    if (pcVar6 != FUN_001191d0) {
      return 0;
    }
LAB_00119602:
    pcVar6 = FUN_001191d0;
  }
  *param_3 = pcVar6;
  return 1;
}




// Function: extract_dir @ 0x19850

int extract_dir(char *param_1,int param_2)

{
  char cVar1;
  char cVar2;
  __mode_t __mode;
  int iVar3;
  int iVar4;
  int *piVar5;
  long lVar6;
  stat *psVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  uint uVar10;
  undefined8 uVar11;
  long in_FS_OFFSET;
  byte bVar12;
  char local_d9;
  stat local_d8;
  long local_40;
  
  bVar12 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_d9 = '\0';
  if ((DAT_00184ad1 != '\0') && (DAT_001847f0 == 0)) {
    iVar3 = fstatat(DAT_00181234,".",&local_d8,0);
    if (iVar3 == 0) {
      DAT_001847f0 = local_d8.st_dev;
    }
    else {
      FUN_00124ec0(".");
    }
  }
  if (DAT_00184b50 == '\0') {
    if (param_2 == 0x44) {
      FUN_00122bd0();
    }
  }
  else {
    FUN_001206e0(param_1);
  }
  uVar11 = 0;
  uVar10 = 0;
  __mode = FUN_00119820(DAT_001848b0);
LAB_001198cc:
  iVar3 = mkdirat(DAT_00181234,param_1,__mode);
  if (iVar3 == 0) goto LAB_00119ac0;
  piVar5 = __errno_location();
  cVar1 = local_d9;
  if (*piVar5 == 0x11) {
    if (local_d9 == '\0') {
      if (DAT_00184b30 != '\0') goto LAB_001199c4;
      if (DAT_00184b34 < 3) goto LAB_00119915;
      goto LAB_00119949;
    }
    if (DAT_00184b30 != '\0') {
LAB_001199c4:
      cVar2 = FUN_00116ce0(param_1);
      if (cVar2 == '\0') goto LAB_00119915;
LAB_001199d4:
      iVar3 = 0;
      goto LAB_00119993;
    }
LAB_00119915:
    iVar4 = FUN_001246f0(param_1,&local_d8);
    if (iVar4 != 0) {
LAB_00119943:
      *piVar5 = 0x11;
      goto LAB_00119949;
    }
    uVar10 = local_d8.st_mode;
    if ((local_d8.st_mode & 0xf000) != 0x4000) {
      uVar11 = 0xffffffff;
      goto LAB_00119943;
    }
    if (cVar1 != '\0') {
      FUN_00116e60(param_1,&local_d8);
      goto LAB_001199d4;
    }
    if (DAT_00184b34 == 1) {
      __mode = FUN_00119820(local_d8.st_mode,param_1);
      iVar3 = FUN_00117280(0xffffffff);
      if (iVar3 != 0) {
        FUN_00132550(param_1,__mode);
        goto LAB_00119af0;
      }
      psVar7 = &local_d8;
      puVar8 = &DAT_00184898;
      for (lVar6 = 0x24; lVar6 != 0; lVar6 = lVar6 + -1) {
        *(int *)puVar8 = (int)psVar7->st_dev;
        psVar7 = (stat *)((long)psVar7 + (ulong)bVar12 * -8 + 4);
        puVar8 = (undefined8 *)((long)puVar8 + ((ulong)bVar12 * -2 + 1) * 4);
      }
LAB_00119ac0:
      uVar9 = 0x100;
      uVar11 = 0x1ff;
      uVar10 = ~DAT_00184328 & __mode;
      goto LAB_00119977;
    }
LAB_00119af0:
    uVar11 = 0xffffffff;
  }
  else {
LAB_00119949:
    iVar4 = FUN_00118750(param_1,0,&local_d9);
    if (iVar4 != 0) goto code_r0x00119960;
    if (*piVar5 != 0x11) {
      iVar3 = 1;
      FUN_00132760(param_1);
      goto LAB_00119993;
    }
  }
  goto LAB_0011996f;
code_r0x00119960:
  if (iVar4 != 1) {
LAB_0011996f:
    if ((DAT_00184b34 & 0xfffffffd) == 0) {
      uVar9 = 0;
LAB_00119977:
      FUN_00117c30(param_1,&DAT_00184840,uVar10,uVar11,DAT_001848b0,uVar9);
    }
LAB_00119993:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return iVar3;
  }
  goto LAB_001198cc;
}




// Function: extr_init @ 0x19b30

void extr_init(void)

{
  __uid_t _Var1;
  __mode_t __mask;
  
  _Var1 = geteuid();
  DAT_00184330 = _Var1 == 0;
  DAT_00184aa4 = DAT_00184aa4 + (uint)(byte)DAT_00184330;
  DAT_00184aa8 = DAT_00184aa8 + (uint)(byte)DAT_00184330;
  __mask = umask(0);
  DAT_0018432c = __mask;
  if (0 < DAT_00184aa4) {
    DAT_00184328 = 0;
    return;
  }
  umask(__mask);
  DAT_00184328 = __mask;
  return;
}




// Function: remove_delayed_set_stat @ 0x19b90

void remove_delayed_set_stat(char *param_1)

{
  long *plVar1;
  int iVar2;
  int iVar3;
  long *plVar4;
  long *plVar5;
  
  iVar2 = DAT_00184580;
  if (DAT_00184320 != (long *)0x0) {
    plVar1 = DAT_00184320;
    plVar5 = (long *)0x0;
    do {
      plVar4 = plVar1;
      plVar1 = (long *)*plVar4;
      if ((*(int *)((long)plVar4 + 0x5c) == iVar2) &&
         (iVar3 = strcmp((char *)plVar4[0x14],param_1), iVar3 == 0)) {
        FUN_00116c90(plVar4);
        if (plVar5 == (long *)0x0) {
          DAT_00184320 = plVar1;
          return;
        }
        *plVar5 = (long)plVar1;
        return;
      }
      plVar5 = plVar4;
    } while (plVar1 != (long *)0x0);
  }
  return;
}




// Function: extract_archive @ 0x19c10

void extract_archive(void)

{
  undefined4 uVar1;
  char cVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  code *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001842f0 = FUN_00118640;
  FUN_0010d1f0(DAT_00184408);
  if ((DAT_00184b92 == '\0') && (cVar2 = FUN_00127cc0(DAT_00184840), cVar2 != '\0')) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar5 = FUN_0014aa70(DAT_00184840);
    uVar6 = dcgettext(0,"%s: Member name contains \'..\'",5);
    error(0,0,uVar6,uVar5);
    DAT_00184d58 = 2;
  }
  else if ((*DAT_00184848 != '\0') &&
          ((DAT_00184b40 == '\0' || (iVar3 = FUN_0012ca20("extract"), iVar3 != 0)))) {
    uVar1 = DAT_00184580;
    if (DAT_00184a54 != 0) {
      FUN_00122990(&DAT_00184840,DAT_00184408,0xffffffffffffffff);
      uVar1 = DAT_00184580;
    }
    DAT_00184580 = uVar1;
    if (DAT_001847e0 == '\0') {
      FUN_00117870(DAT_00184848,0);
      FUN_00124c80(uVar1);
    }
    if ((DAT_00184b84 == '\0') || (cVar2 = FUN_00124710(DAT_00184848,0), cVar2 != '\0')) {
      iVar3 = 0x53;
      cVar2 = FUN_00129e10(&DAT_00184840);
      if (cVar2 == '\0') {
        iVar3 = (int)*(char *)(DAT_00184408 + 0x9c);
      }
      cVar2 = FUN_00119540(DAT_00184848,iVar3,&local_28);
      if (cVar2 == '\0') {
        FUN_00122bd0();
LAB_00119cec:
        if (DAT_00184b84 != '\0') {
          if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
            FUN_001245d0();
            return;
          }
          goto LAB_00119eac;
        }
      }
      else {
        iVar3 = (*local_28)(DAT_00184848,iVar3);
        if (iVar3 != 0) goto LAB_00119cec;
      }
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
      goto LAB_00119eac;
    }
    piVar4 = __errno_location();
    iVar3 = *piVar4;
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar5 = FUN_0014aa70(DAT_00184848);
    uVar6 = dcgettext(0,"%s: Was unable to backup this file",5);
    error(0,iVar3,uVar6,uVar5);
    DAT_00184d58 = 2;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00122bd0();
    return;
  }
LAB_00119eac:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: rename_directory @ 0x19ec0

undefined8 rename_directory(char *param_1,char *param_2)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_31;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = renameat(DAT_00181234,param_1,DAT_00181234,param_2);
  if (iVar1 == 0) {
    FUN_00117bb0(param_1,param_2);
    uVar4 = 1;
  }
  else {
    piVar3 = __errno_location();
    iVar1 = *piVar3;
    if (iVar1 == 2) {
      iVar2 = FUN_00117f70(param_2,&local_31);
      if (iVar2 == 0) {
        iVar1 = renameat(DAT_00181234,param_1,DAT_00181234,param_2);
        uVar4 = 1;
        if (iVar1 == 0) goto LAB_00119f73;
        iVar1 = *piVar3;
      }
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar4 = FUN_0014ac40(1,param_2);
    uVar5 = FUN_0014ac40(0,param_1);
    uVar6 = dcgettext(0,"Cannot rename %s to %s",5);
    error(0,iVar1,uVar6,uVar5,uVar4);
    uVar4 = 0;
    DAT_00184d58 = 2;
  }
LAB_00119f73:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xattr_decode_keyword @ 0x19ff0

void xattr_decode_keyword(char *param_1)

{
  char cVar1;
  char *pcVar2;
  char *pcVar3;
  
  pcVar2 = param_1;
LAB_0011a00f:
  do {
    while( true ) {
      pcVar3 = pcVar2 + 1;
      cVar1 = *param_1;
      if (cVar1 == '%') break;
      *pcVar2 = cVar1;
      if (cVar1 == '\0') {
        return;
      }
LAB_0011a007:
      param_1 = param_1 + 1;
      pcVar2 = pcVar3;
    }
    if (param_1[1] != '3') {
      if ((param_1[1] != '2') || (param_1[2] != '5')) {
LAB_0011a028:
        *pcVar2 = '%';
        goto LAB_0011a007;
      }
      *pcVar2 = '%';
      param_1 = param_1 + 3;
      pcVar2 = pcVar3;
      goto LAB_0011a00f;
    }
    if (param_1[2] != 'D') goto LAB_0011a028;
    *pcVar2 = '=';
    param_1 = param_1 + 3;
    pcVar2 = pcVar3;
  } while( true );
}




// Function: dummy_coder @ 0x1a060

void dummy_coder(void)

{
  return;
}




// Function: dummy_decoder @ 0x1a070

void dummy_decoder(void)

{
  return;
}




// Function: xheader_protected_pattern_p @ 0x1a080

undefined8 xheader_protected_pattern_p(char *param_1)

{
  int iVar1;
  undefined **ppuVar2;
  char *__name;
  
  __name = "atime";
  ppuVar2 = &PTR_s_atime_0017f200;
  do {
    if ((*(char *)((long)ppuVar2 + 0x1c) == '\0') && (((ulong)ppuVar2[3] & 1) != 0)) {
      iVar1 = fnmatch(param_1,__name,0);
      if (iVar1 == 0) {
        return 1;
      }
    }
    __name = ppuVar2[4];
    ppuVar2 = ppuVar2 + 4;
  } while (__name != (char *)0x0);
  return 0;
}




// Function: xheader_set_single_keyword @ 0x1a0f0

void xheader_set_single_keyword(undefined8 param_1)

{
  undefined8 uVar1;
  
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = dcgettext(0,"Keyword %s is unknown or not yet implemented",5);
  error(0,0,uVar1,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: xheader_protected_keyword_p @ 0x1a140

undefined8 xheader_protected_keyword_p(char *param_1)

{
  int iVar1;
  undefined **ppuVar2;
  char *__s1;
  
  __s1 = "atime";
  ppuVar2 = &PTR_s_atime_0017f200;
  do {
    if ((*(char *)((long)ppuVar2 + 0x1c) == '\0') && (((ulong)ppuVar2[3] & 1) != 0)) {
      iVar1 = strcmp(__s1,param_1);
      if (iVar1 == 0) {
        return 1;
      }
    }
    __s1 = ppuVar2[4];
    ppuVar2 = ppuVar2 + 4;
  } while (__s1 != (char *)0x0);
  return 0;
}




// Function: xheader_keyword_override_p @ 0x1a1a0

undefined8 xheader_keyword_override_p(char *param_1)

{
  undefined8 *puVar1;
  int iVar2;
  
  puVar1 = DAT_00184380;
  while( true ) {
    if (puVar1 == (undefined8 *)0x0) {
      return 0;
    }
    iVar2 = strcmp((char *)puVar1[1],param_1);
    if (iVar2 == 0) break;
    puVar1 = (undefined8 *)*puVar1;
  }
  return 1;
}




// Function: assign_time_option @ 0x1a200

void assign_time_option(undefined8 param_1,undefined8 *param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 auVar3 [16];
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  auVar3 = FUN_001242f0(param_3,&local_38,0);
  cVar1 = FUN_00123ba0(auVar3._0_8_,auVar3._8_8_);
  if ((cVar1 == '\0') || (*local_38 != '\0')) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = dcgettext(0,"Time stamp is out of allowed range",5);
    error(0,0,uVar2);
    DAT_00184d58 = 2;
  }
  else {
    *param_2 = auVar3._0_8_;
    FUN_00123be0(param_1,param_3);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xheader_list_append @ 0x1a2c0

void xheader_list_append(undefined8 *param_1,undefined8 param_2,long param_3)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  
  puVar2 = (undefined8 *)FUN_0014c5b0(0x18);
  uVar3 = FUN_0014c7a0(param_2);
  puVar2[1] = uVar3;
  uVar3 = 0;
  if (param_3 != 0) {
    uVar3 = FUN_0014c7a0(param_3);
  }
  uVar1 = *param_1;
  puVar2[2] = uVar3;
  *param_1 = puVar2;
  *puVar2 = uVar1;
  return;
}




// Function: xheader_set_keyword_equal @ 0x1a320

void xheader_set_keyword_equal(byte *param_1,byte *param_2)

{
  byte bVar1;
  byte bVar2;
  char cVar3;
  int iVar4;
  ushort **ppuVar5;
  undefined8 uVar6;
  byte *pbVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  
  if (param_2 == param_1) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    param_1 = (byte *)FUN_0014ac50(param_1);
    uVar6 = dcgettext(0,"Malformed pax option: %s",5);
  }
  else {
    bVar1 = param_2[-1];
    pbVar7 = param_2;
    if (bVar1 == 0x3a) {
      pbVar7 = param_2 + -1;
    }
    if (param_1 < pbVar7) {
      ppuVar5 = __ctype_b_loc();
      do {
        if ((*(byte *)((long)*ppuVar5 + (ulong)*pbVar7 * 2 + 1) & 0x20) == 0) break;
        pbVar7 = pbVar7 + -1;
      } while (param_1 != pbVar7);
    }
    *pbVar7 = 0;
    bVar2 = param_2[1];
    param_2 = param_2 + 1;
    if (bVar2 != 0) {
      ppuVar5 = __ctype_b_loc();
      do {
        if ((*(byte *)((long)*ppuVar5 + (ulong)bVar2 * 2 + 1) & 0x20) == 0) break;
        bVar2 = param_2[1];
        param_2 = param_2 + 1;
      } while (bVar2 != 0);
    }
    iVar4 = strcmp((char *)param_1,"delete");
    if (iVar4 != 0) {
      iVar4 = strcmp((char *)param_1,"exthdr.name");
      puVar8 = &DAT_00184370;
      if (iVar4 != 0) {
        iVar4 = strcmp((char *)param_1,"globexthdr.name");
        if (iVar4 != 0) {
          iVar4 = strcmp((char *)param_1,"exthdr.mtime");
          puVar8 = &DAT_00184360;
          puVar9 = &DAT_00184368;
          if (iVar4 != 0) {
            iVar4 = strcmp((char *)param_1,"globexthdr.mtime");
            if (iVar4 != 0) {
              cVar3 = FUN_0011a140(param_1);
              if (cVar3 != '\0') {
                if (DAT_00184d50 != (code *)0x0) {
                  (*DAT_00184d50)();
                }
                uVar6 = dcgettext(0,"Keyword %s cannot be overridden",5);
                goto LAB_0011a4dd;
              }
              puVar8 = &DAT_00184388;
              pbVar7 = param_2;
              if (bVar1 == 0x3a) {
                puVar8 = &DAT_00184380;
              }
              goto LAB_0011a488;
            }
            puVar8 = &DAT_00184348;
            puVar9 = &DAT_00184350;
          }
          FUN_0011a200(puVar9,puVar8,param_2);
          return;
        }
        puVar8 = &DAT_00184358;
      }
      FUN_00123be0(puVar8,param_2);
      return;
    }
    cVar3 = FUN_0011a080(param_2);
    if (cVar3 == '\0') {
      pbVar7 = (byte *)0x0;
      puVar8 = &DAT_00184390;
      param_1 = param_2;
LAB_0011a488:
      FUN_0011a2c0(puVar8,param_1,pbVar7);
      return;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    param_1 = (byte *)FUN_0014ac50(param_2);
    uVar6 = dcgettext(0,"Pattern %s cannot be used",5);
  }
LAB_0011a4dd:
  error(0,0,uVar6,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: dumpdir_decoder @ 0x1a560

void dumpdir_decoder(long param_1,undefined8 param_2,void *param_3,size_t param_4)

{
  void *__dest;
  
  __dest = (void *)FUN_0014c5b0(param_4);
  *(void **)(param_1 + 400) = __dest;
  memcpy(__dest,param_3,param_4);
  return;
}




// Function: xheader_init @ 0x1a5a0

void xheader_init(long *param_1)

{
  undefined *puVar1;
  long lVar2;
  
  if (*param_1 != 0) {
    return;
  }
  lVar2 = FUN_0014c5b0(0x58);
  puVar1 = PTR_free_00180fb8;
  *param_1 = lVar2;
  _obstack_begin(lVar2,0,0,FUN_0014c5b0,puVar1);
  return;
}




// Function: decode_string @ 0x1a5e0

void decode_string(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  
  if ((void *)*param_1 != (void *)0x0) {
    free((void *)*param_1);
    *param_1 = 0;
  }
  cVar1 = FUN_00131340(0,param_2,param_1);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00123be0(param_1,param_2);
  return;
}




// Function: xattr_selinux_decoder @ 0x1a640

void xattr_selinux_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011a5e0(param_1 + 0x30,param_3);
  return;
}




// Function: xattr_encode_keyword @ 0x1a6b0

undefined1 * xattr_encode_keyword(char *param_1)

{
  char *pcVar1;
  undefined1 *puVar2;
  ulong uVar3;
  ulong uVar4;
  char cVar5;
  
  puVar2 = DAT_00184340;
  if (DAT_00184340 == (undefined1 *)0x0) {
    DAT_00184338 = 0x100;
    puVar2 = (undefined1 *)FUN_0014c5b0(0x100);
    DAT_00184340 = puVar2;
  }
  else {
    *DAT_00184340 = 0;
  }
  cVar5 = *param_1;
  uVar4 = 0;
  if (cVar5 == '\0') {
    *puVar2 = 0;
    return puVar2;
  }
  do {
    uVar3 = uVar4 + 2;
    if (uVar3 < DAT_00184338) {
      pcVar1 = puVar2 + uVar4;
      if (cVar5 != '%') goto LAB_0011a6fa;
LAB_0011a73f:
      builtin_strncpy(pcVar1,"%25",4);
    }
    else {
      puVar2 = (undefined1 *)FUN_0014c710(puVar2,&DAT_00184338);
      pcVar1 = puVar2 + uVar4;
      DAT_00184340 = puVar2;
      if (cVar5 == '%') goto LAB_0011a73f;
LAB_0011a6fa:
      if (cVar5 == '=') {
        builtin_strncpy(pcVar1,"%3D",4);
      }
      else {
        *pcVar1 = cVar5;
        uVar3 = uVar4;
      }
    }
    cVar5 = param_1[1];
    param_1 = param_1 + 1;
    uVar4 = uVar3 + 1;
    if (cVar5 == '\0') {
      puVar2[uVar4] = 0;
      return puVar2;
    }
  } while( true );
}




// Function: x_obstack_grow @ 0x1a7c0

void x_obstack_grow(long *param_1,void *param_2,ulong param_3)

{
  long lVar1;
  void *__dest;
  
  lVar1 = *param_1;
  __dest = *(void **)(lVar1 + 0x18);
  if ((ulong)(*(long *)(lVar1 + 0x20) - (long)__dest) < param_3) {
    _obstack_newchunk(lVar1,param_3);
    __dest = *(void **)(lVar1 + 0x18);
  }
  memcpy(__dest,param_2,param_3);
  *(long *)(lVar1 + 0x18) = *(long *)(lVar1 + 0x18) + param_3;
  param_1[1] = param_1[1] + param_3;
  return;
}




// Function: x_obstack_1grow @ 0x1a830

void x_obstack_1grow(long *param_1,undefined1 param_2)

{
  long lVar1;
  undefined1 *puVar2;
  
  lVar1 = *param_1;
  puVar2 = *(undefined1 **)(lVar1 + 0x18);
  if (*(undefined1 **)(lVar1 + 0x20) == puVar2) {
    _obstack_newchunk(lVar1,1);
    puVar2 = *(undefined1 **)(lVar1 + 0x18);
  }
  *(undefined1 **)(lVar1 + 0x18) = puVar2 + 1;
  *puVar2 = param_2;
  param_1[1] = param_1[1] + 1;
  return;
}




// Function: xheader_print_n @ 0x1a880

void xheader_print_n(undefined8 param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  char *__s;
  size_t sVar1;
  long lVar2;
  undefined1 *puVar3;
  undefined1 *puVar4;
  long in_FS_OFFSET;
  bool bVar5;
  undefined1 local_58 [20];
  undefined1 local_44 [4];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_0011a6b0(param_2);
  sVar1 = strlen(__s);
  puVar3 = (undefined1 *)0x0;
  do {
    lVar2 = FUN_00143090(puVar3 + sVar1 + 3 + param_4,local_58);
    puVar4 = local_44 + -lVar2;
    bVar5 = puVar3 != puVar4;
    puVar3 = puVar4;
  } while (bVar5);
  FUN_0011a7c0(param_1,lVar2,puVar4);
  FUN_0011a830(param_1,0x20);
  FUN_0011a7c0(param_1,__s,sVar1);
  FUN_0011a830(param_1,0x3d);
  FUN_0011a7c0(param_1,param_3,param_4);
  FUN_0011a830(param_1,10);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xheader_print @ 0x1a980

void xheader_print(undefined8 param_1,undefined8 param_2,char *param_3)

{
  size_t sVar1;
  
  sVar1 = strlen(param_3);
  FUN_0011a880(param_1,param_2,param_3,sVar1);
  return;
}




// Function: code_string @ 0x1a9b0

void code_string(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  void *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00131340(1,param_1,&local_28);
  if (cVar1 == '\0') {
    local_28 = (void *)FUN_0014c7a0(param_1);
  }
  FUN_0011a980(param_3,param_2,local_28);
  free(local_28);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xattr_selinux_coder @ 0x1aa40

void xattr_selinux_coder(long param_1)

{
  FUN_0011a9b0(*(undefined8 *)(param_1 + 0x30));
  return;
}




// Function: x_obstack_blank @ 0x1ab10

void x_obstack_blank(long *param_1,ulong param_2)

{
  long lVar1;
  long lVar2;
  
  lVar1 = *param_1;
  lVar2 = *(long *)(lVar1 + 0x18);
  if ((ulong)(*(long *)(lVar1 + 0x20) - lVar2) < param_2) {
    _obstack_newchunk(lVar1);
    lVar2 = *(long *)(lVar1 + 0x18);
  }
  *(ulong *)(lVar1 + 0x18) = lVar2 + param_2;
  param_1[1] = param_1[1] + param_2;
  return;
}




// Function: xheader_xattr__add @ 0x1ab50

void xheader_xattr__add(long *param_1,long *param_2,undefined8 param_3,undefined8 param_4,long param_5)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  long lVar4;
  
  lVar3 = *param_2;
  lVar1 = *param_1;
  *param_2 = lVar3 + 1;
  lVar3 = (lVar3 + 1) * 0x18;
  lVar4 = lVar3 + -0x18;
  lVar3 = FUN_0014c610(lVar1,lVar3);
  *param_1 = lVar3;
  uVar2 = FUN_0014c7a0(param_3);
  *(undefined8 *)(lVar3 + lVar4) = uVar2;
  lVar3 = *param_1;
  uVar2 = FUN_0014c770(param_4,param_5 + 1);
  *(undefined8 *)(lVar3 + lVar4 + 8) = uVar2;
  *(long *)(*param_1 + 0x10 + lVar4) = param_5;
  return;
}




// Function: xattr_acls_d_decoder @ 0x1abd0

void xattr_acls_d_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0014c770(param_3,param_4 + 1);
  *(long *)(param_1 + 0x50) = param_4;
  *(undefined8 *)(param_1 + 0x48) = uVar1;
  return;
}




// Function: xattr_acls_a_decoder @ 0x1ac00

void xattr_acls_a_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0014c770(param_3,param_4 + 1);
  *(long *)(param_1 + 0x40) = param_4;
  *(undefined8 *)(param_1 + 0x38) = uVar1;
  return;
}




// Function: locate_handler @ 0x1ac30

undefined ** locate_handler(char *param_1)

{
  int iVar1;
  size_t __n;
  char *__s;
  undefined **ppuVar2;
  
  ppuVar2 = &PTR_s_atime_0017f200;
  __s = "atime";
  while( true ) {
    if (*(char *)((long)ppuVar2 + 0x1c) == '\0') {
      iVar1 = strcmp(__s,param_1);
    }
    else {
      __n = strlen(__s);
      iVar1 = strncmp(__s,param_1,__n);
    }
    if (iVar1 == 0) break;
    __s = ppuVar2[4];
    ppuVar2 = ppuVar2 + 4;
    if (__s == (char *)0x0) {
      return (undefined **)0x0;
    }
  }
  return ppuVar2;
}




// Function: run_override_list @ 0x1acb0

void run_override_list(long *param_1,undefined8 param_2)

{
  char *__s;
  undefined8 *puVar1;
  size_t sVar2;
  
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    puVar1 = (undefined8 *)FUN_0011ac30(param_1[1]);
    if (puVar1 != (undefined8 *)0x0) {
      __s = (char *)param_1[2];
      sVar2 = strlen(__s);
      (*(code *)puVar1[2])(param_2,*puVar1,__s,sVar2);
    }
    param_1 = (long *)*param_1;
  } while (param_1 != (long *)0x0);
  return;
}




// Function: decg @ 0x1ad20

void decg(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  
  lVar1 = FUN_0011ac30(param_2);
  if ((lVar1 != 0) && ((*(byte *)(lVar1 + 0x18) & 2) != 0)) {
                    /* WARNING: Could not recover jumptable at 0x0011ad69. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (**(code **)(lVar1 + 0x10))(param_1,param_2,param_3,param_4);
    return;
  }
  FUN_0011a2c0(param_1,param_2,param_3);
  return;
}




// Function: out_of_range_header @ 0x1ad90

void out_of_range_header(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 auStack_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00142f50(param_3,auStack_68);
  uVar2 = FUN_00143090(param_4,local_48);
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar3 = dcgettext(0,"Extended header %s=%s is out of range %s..%s",5);
  error(0,0,uVar3,param_1,param_2,uVar1,uVar2);
  DAT_00184d58 = 2;
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: decode_time @ 0x1ae40

undefined4 decode_time(undefined1 (*param_1) [16],char *param_2,undefined8 param_3)

{
  undefined4 uVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 auVar3 [16];
  char *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  auVar3 = FUN_001242f0(param_2,&local_48,1);
  uVar1 = FUN_00123ba0(auVar3._0_8_,auVar3._8_8_);
  if ((char)uVar1 == '\0') {
    if ((param_2 < local_48) && (*local_48 == '\0')) {
      FUN_0011ad90(param_3,param_2,0x8000000000000000,0x7fffffffffffffff);
    }
    else {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,"Malformed extended header: invalid %s=%s",5);
      error(0,0,uVar2,param_3,param_2);
      DAT_00184d58 = 2;
    }
  }
  else {
    *param_1 = auVar3;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mtime_decoder @ 0x1af40

void mtime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ae40(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0xf8) = local_28;
    *(undefined8 *)(param_1 + 0x100) = uStack_20;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ctime_decoder @ 0x1afa0

void ctime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ae40(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x108) = local_28;
    *(undefined8 *)(param_1 + 0x110) = uStack_20;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: atime_decoder @ 0x1b000

void atime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ae40(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0xe8) = local_28;
    *(undefined8 *)(param_1 + 0xf0) = uStack_20;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dumpdir_coder @ 0x1b060

void dumpdir_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0011f4c0(param_4);
  FUN_0011a880(param_3,param_2,param_4,uVar1);
  return;
}




// Function: sparse_map_decoder @ 0x1b090

void sparse_map_decoder(long param_1,undefined8 param_2,char *param_3)

{
  ulong uVar1;
  bool bVar2;
  int *piVar3;
  intmax_t iVar4;
  undefined8 uVar5;
  intmax_t *piVar6;
  char *pcVar7;
  char cVar8;
  intmax_t unaff_R12;
  long in_FS_OFFSET;
  char *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  *(undefined8 *)(param_1 + 0x130) = 0;
  if ((int)*param_3 - 0x30U < 10) {
    piVar3 = __errno_location();
    bVar2 = true;
    do {
      *piVar3 = 0;
      iVar4 = strtoimax(param_3,&local_48,10);
      if (bVar2) {
        if (*piVar3 == 0x22) goto LAB_0011b1f8;
      }
      else {
        if (*piVar3 == 0x22) {
LAB_0011b1f8:
          FUN_0011ad90(param_2,param_3,0,0x7fffffffffffffff);
          goto LAB_0011b1d0;
        }
        uVar1 = *(ulong *)(param_1 + 0x130);
        if (*(ulong *)(param_1 + 0x138) <= uVar1) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          pcVar7 = "Malformed extended header: excess %s=%s";
          goto LAB_0011b1aa;
        }
        piVar6 = (intmax_t *)(uVar1 * 0x10 + *(long *)(param_1 + 0x140));
        *(ulong *)(param_1 + 0x130) = uVar1 + 1;
        *piVar6 = unaff_R12;
        piVar6[1] = iVar4;
        iVar4 = unaff_R12;
      }
      cVar8 = *local_48;
      if (cVar8 == '\0') {
        if (bVar2) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar5 = dcgettext(0,"Malformed extended header: invalid %s: odd number of values",5);
          error(0,0,uVar5,param_2);
          DAT_00184d58 = 2;
        }
        goto LAB_0011b1d0;
      }
      if (cVar8 != ',') {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
          cVar8 = *local_48;
        }
        uVar5 = dcgettext(0,"Malformed extended header: invalid %s: unexpected delimiter %c",5);
        error(0,0,uVar5,param_2,(int)cVar8);
        DAT_00184d58 = 2;
        goto LAB_0011b1d0;
      }
      param_3 = local_48 + 1;
      unaff_R12 = iVar4;
      bVar2 = (bool)(bVar2 ^ 1);
    } while ((int)local_48[1] - 0x30U < 10);
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  pcVar7 = "Malformed extended header: invalid %s=%s";
LAB_0011b1aa:
  uVar5 = dcgettext(0,pcVar7,5);
  error(0,0,uVar5,param_2,param_3);
  DAT_00184d58 = 2;
LAB_0011b1d0:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: raw_path_decoder @ 0x1b2e0

void raw_path_decoder(long param_1,undefined8 param_2)

{
  undefined1 uVar1;
  
  FUN_0011a5e0();
  FUN_0011a5e0(param_1 + 8,param_2);
  uVar1 = FUN_0013e630(*(undefined8 *)(param_1 + 8));
  *(undefined1 *)(param_1 + 0x10) = uVar1;
  return;
}




// Function: sparse_path_decoder @ 0x1b310

void sparse_path_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  *(undefined1 *)(param_1 + 0x151) = 1;
  FUN_0011b2e0(param_1,param_3);
  return;
}




// Function: code_time @ 0x1b340

void code_time(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 auStack_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00124270(param_1,param_2,auStack_48);
  FUN_0011a980(param_4,param_3,uVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mtime_coder @ 0x1b390

void mtime_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  if (param_4 != (undefined8 *)0x0) {
    FUN_0011b340(*param_4,param_4[1],param_2,param_3);
    return;
  }
  FUN_0011b340(*(undefined8 *)(param_1 + 0xf8),*(undefined8 *)(param_1 + 0x100),param_2,param_3);
  return;
}




// Function: gid_decoder @ 0x1b530

void gid_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b440(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x78) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: uid_decoder @ 0x1b590

void uid_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b440(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x74) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_offset_decoder @ 0x1b640

void volume_offset_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    DAT_00183f00 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_size_decoder @ 0x1b6a0

void volume_size_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    DAT_00183f08 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_numbytes_decoder @ 0x1b700

void sparse_numbytes_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  ulong uVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_0011b5f0(&local_28,param_3,0x7fffffffffffffff,param_2);
  if (cVar2 != '\0') {
    uVar1 = *(ulong *)(param_1 + 0x130);
    if (uVar1 < *(ulong *)(param_1 + 0x138)) {
      *(ulong *)(param_1 + 0x130) = uVar1 + 1;
      *(undefined8 *)(uVar1 * 0x10 + *(long *)(param_1 + 0x140) + 8) = local_28;
    }
    else {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Malformed extended header: excess %s=%s",5);
      error(0,0,uVar3,param_2,param_3);
      DAT_00184d58 = 2;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_offset_decoder @ 0x1b7e0

void sparse_offset_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_28,param_3,0x7fffffffffffffff,param_2);
  if (cVar1 != '\0') {
    if (*(ulong *)(param_1 + 0x130) < *(ulong *)(param_1 + 0x138)) {
      *(undefined8 *)(*(ulong *)(param_1 + 0x130) * 0x10 + *(long *)(param_1 + 0x140)) = local_28;
    }
    else {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,"Malformed extended header: excess %s=%s",5);
      error(0,0,uVar2,"GNU.sparse.offset",param_3);
      DAT_00184d58 = 2;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_size_decoder @ 0x1b8b0

void sparse_size_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_18,param_3,0x7fffffffffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined1 *)(param_1 + 0x150) = 1;
    *(undefined8 *)(param_1 + 0x148) = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_minor_decoder @ 0x1b920

void sparse_minor_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x128) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_major_decoder @ 0x1b980

void sparse_major_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x124) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: size_decoder @ 0x1b9e0

void size_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_18,param_3,0x7fffffffffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x88) = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_numblocks_decoder @ 0x1ba40

void sparse_numblocks_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b5f0(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x138) = local_18;
    uVar2 = FUN_0014c720(local_18,0x10);
    *(undefined8 *)(param_1 + 0x130) = 0;
    *(undefined8 *)(param_1 + 0x140) = uVar2;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: gid_coder @ 0x1bb20

void gid_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011bac0(*(undefined4 *)(param_1 + 0x78),param_2,0xffffffff,param_3);
  return;
}




// Function: uid_coder @ 0x1bb40

void uid_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011bac0(*(undefined4 *)(param_1 + 0x74),param_2,0xffffffff,param_3);
  return;
}




// Function: xheader_keyword_deleted_p @ 0x1bef0

undefined8 xheader_keyword_deleted_p(char *param_1)

{
  undefined8 *puVar1;
  int iVar2;
  
  puVar1 = DAT_00184390;
  while( true ) {
    if (puVar1 == (undefined8 *)0x0) {
      return 0;
    }
    iVar2 = fnmatch((char *)puVar1[1],param_1,0);
    if (iVar2 == 0) break;
    puVar1 = (undefined8 *)*puVar1;
  }
  return 1;
}




// Function: decx @ 0x1bf50

void decx(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  
  cVar1 = FUN_0011bef0(param_2);
  if (cVar1 == '\0') {
    cVar1 = FUN_0011a1a0(param_2);
    if (cVar1 == '\0') {
      lVar2 = FUN_0011ac30(param_2);
      if (lVar2 != 0) {
                    /* WARNING: Could not recover jumptable at 0x0011bfbc. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (**(code **)(lVar2 + 0x10))(param_1,param_2,param_3,param_4);
        return;
      }
      if ((DAT_00183b80._2_1_ & 2) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar3 = dcgettext(0,"Ignoring unknown extended header keyword \'%s\'",5);
        error(0,0,uVar3,param_2);
        return;
      }
    }
  }
  return;
}




// Function: xheader_set_option @ 0x1c010

char * xheader_set_option(char *param_1)

{
  char cVar1;
  char cVar2;
  __pid_t _Var3;
  char *pcVar4;
  char *pcVar5;
  size_t sVar6;
  char *pcVar7;
  char *pcVar8;
  undefined1 *puVar9;
  char *__s;
  void *__ptr;
  undefined8 extraout_RDX;
  char *pcVar10;
  undefined1 *puVar11;
  long in_FS_OFFSET;
  char *pcStack_c0;
  char *pcStack_b8;
  undefined1 auStack_90 [20];
  undefined1 auStack_7c [12];
  undefined1 auStack_70 [20];
  undefined1 auStack_5c [4];
  long lStack_58;
  undefined *puStack_48;
  char *pcStack_40;
  
  pcVar4 = strtok(param_1,",");
  while( true ) {
    if (pcVar4 == (char *)0x0) {
      return (char *)0x0;
    }
    pcVar5 = strchr(pcVar4,0x3d);
    if (pcVar5 == (char *)0x0) break;
    FUN_0011a320();
    pcVar4 = strtok((char *)0x0,",");
  }
  pcVar7 = pcVar4;
  FUN_0011a0f0();
  puStack_48 = &DAT_0016079c;
  lStack_58 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = *pcVar5;
  pcStack_40 = pcVar4;
  if (cVar2 == '\0') {
    pcStack_c0 = (char *)0x0;
    __s = (char *)0x0;
    pcVar4 = (char *)0x0;
    __ptr = (void *)0x0;
    pcStack_b8 = (char *)0x0;
    puVar11 = (undefined1 *)0x1;
  }
  else {
    __s = (char *)0x0;
    pcStack_c0 = (char *)0x0;
    pcVar4 = (char *)0x0;
    __ptr = (void *)0x0;
    puVar11 = (undefined1 *)0x0;
    pcStack_b8 = (char *)0x0;
    pcVar10 = pcVar5;
    do {
      while( true ) {
        cVar1 = pcVar10[1];
        if ((cVar2 == '%') && (cVar1 != '\0')) break;
        puVar11 = puVar11 + 1;
        pcVar10 = pcVar10 + 1;
        cVar2 = cVar1;
LAB_0011c0ee:
        if (cVar2 == '\0') goto LAB_0011c188;
      }
      if (cVar1 == 'f') {
        if (pcVar7 == (char *)0x0) {
LAB_0011c3e8:
          pcVar8 = pcVar10 + 2;
          pcVar10 = pcVar10 + 2;
          cVar2 = *pcVar8;
        }
        else {
          __s = (char *)FUN_0013e390(*(undefined8 *)pcVar7);
          sVar6 = strlen(__s);
          puVar11 = puVar11 + sVar6;
          pcVar8 = pcVar10 + 2;
          pcVar10 = pcVar10 + 2;
          cVar2 = *pcVar8;
        }
        goto LAB_0011c0ee;
      }
      if ('f' < cVar1) {
        if (cVar1 == 'n') {
          pcVar8 = (char *)FUN_00143090(extraout_RDX,auStack_70);
          puVar9 = auStack_5c;
          pcStack_b8 = pcVar8;
        }
        else {
          if (cVar1 != 'p') goto LAB_0011c33a;
          _Var3 = getpid();
          pcVar8 = (char *)FUN_00143090((long)_Var3,auStack_90);
          puVar9 = auStack_7c;
          pcStack_c0 = pcVar8;
        }
        puVar11 = puVar9 + ((long)puVar11 - (long)pcVar8);
        pcVar8 = pcVar10 + 2;
        pcVar10 = pcVar10 + 2;
        cVar2 = *pcVar8;
        goto LAB_0011c0ee;
      }
      if (cVar1 == '%') {
        pcVar8 = pcVar10 + 2;
        puVar11 = puVar11 + 1;
        pcVar10 = pcVar10 + 2;
        cVar2 = *pcVar8;
        goto LAB_0011c0ee;
      }
      if (cVar1 != 'd') {
LAB_0011c33a:
        pcVar8 = pcVar10 + 2;
        puVar11 = puVar11 + 2;
        pcVar10 = pcVar10 + 2;
        cVar2 = *pcVar8;
        goto LAB_0011c0ee;
      }
      if (pcVar7 == (char *)0x0) goto LAB_0011c3e8;
      if (__ptr == (void *)0x0) {
        __ptr = (void *)FUN_0013e4f0(*(undefined8 *)pcVar7);
      }
      pcVar4 = (char *)FUN_00133020(__ptr,0,DAT_00184b92);
      sVar6 = strlen(pcVar4);
      puVar11 = puVar11 + sVar6;
      cVar2 = pcVar10[2];
      pcVar10 = pcVar10 + 2;
    } while (cVar2 != '\0');
LAB_0011c188:
    puVar11 = puVar11 + 1;
  }
  pcVar10 = (char *)FUN_0014c5b0(puVar11);
  cVar2 = *pcVar5;
  pcVar7 = pcVar10;
  if (cVar2 != '\0') {
LAB_0011c1c0:
    cVar1 = pcVar5[1];
    if (cVar2 == '%') {
      if (cVar1 == 'f') {
        if (__s != (char *)0x0) {
          pcVar7 = stpcpy(pcVar7,__s);
          goto LAB_0011c2cd;
        }
LAB_0011c1fd:
        cVar2 = pcVar5[2];
      }
      else {
        if (cVar1 < 'g') {
          if (cVar1 == '%') {
            *pcVar7 = '%';
            pcVar8 = pcVar5 + 2;
            pcVar7 = pcVar7 + 1;
            pcVar5 = pcVar5 + 2;
            cVar2 = *pcVar8;
            goto joined_r0x0011c1be;
          }
          if (cVar1 == 'd') {
            if (pcVar4 != (char *)0x0) {
              pcVar7 = stpcpy(pcVar7,pcVar4);
            }
            goto LAB_0011c1fd;
          }
          *pcVar7 = '%';
          if (cVar1 == '\0') {
            pcVar7 = pcVar7 + 1;
            goto LAB_0011c210;
          }
LAB_0011c350:
          pcVar7[1] = cVar1;
          pcVar8 = pcVar5 + 2;
          pcVar7 = pcVar7 + 2;
          pcVar5 = pcVar5 + 2;
          cVar2 = *pcVar8;
          goto joined_r0x0011c1be;
        }
        if (cVar1 == 'n') {
          pcVar7 = stpcpy(pcVar7,pcStack_b8);
        }
        else {
          if (cVar1 != 'p') {
            *pcVar7 = '%';
            goto LAB_0011c350;
          }
          pcVar7 = stpcpy(pcVar7,pcStack_c0);
        }
LAB_0011c2cd:
        cVar2 = pcVar5[2];
      }
      pcVar5 = pcVar5 + 2;
    }
    else {
      *pcVar7 = cVar2;
      pcVar5 = pcVar5 + 1;
      pcVar7 = pcVar7 + 1;
      cVar2 = cVar1;
    }
joined_r0x0011c1be:
    if (cVar2 == '\0') goto LAB_0011c210;
    goto LAB_0011c1c0;
  }
  free(__ptr);
  goto LAB_0011c22f;
LAB_0011c210:
  free(__ptr);
  if (pcVar10 < pcVar7) {
    do {
      if (pcVar7[-1] != '/') break;
      pcVar7 = pcVar7 + -1;
    } while (pcVar10 != pcVar7);
  }
LAB_0011c22f:
  *pcVar7 = '\0';
  if (lStack_58 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return pcVar10;
}




// Function: xheader_format_name @ 0x1c080

char * xheader_format_name(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  char *pcVar1;
  char cVar2;
  char cVar3;
  __pid_t _Var4;
  char *__s;
  size_t sVar5;
  char *pcVar6;
  undefined1 *puVar7;
  char *__s_00;
  void *__ptr;
  char *pcVar8;
  undefined1 *puVar9;
  long in_FS_OFFSET;
  char *local_a8;
  char *local_a0;
  undefined1 local_78 [20];
  undefined1 local_64 [12];
  undefined1 local_58 [20];
  undefined1 local_44 [4];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar3 = *param_2;
  if (cVar3 == '\0') {
    local_a8 = (char *)0x0;
    __s_00 = (char *)0x0;
    __s = (char *)0x0;
    __ptr = (void *)0x0;
    local_a0 = (char *)0x0;
    puVar9 = (undefined1 *)0x1;
  }
  else {
    __s_00 = (char *)0x0;
    local_a8 = (char *)0x0;
    __s = (char *)0x0;
    __ptr = (void *)0x0;
    puVar9 = (undefined1 *)0x0;
    local_a0 = (char *)0x0;
    pcVar8 = param_2;
    do {
      while( true ) {
        cVar2 = pcVar8[1];
        if ((cVar3 == '%') && (cVar2 != '\0')) break;
        puVar9 = puVar9 + 1;
        pcVar8 = pcVar8 + 1;
        cVar3 = cVar2;
LAB_0011c0ee:
        if (cVar3 == '\0') goto LAB_0011c188;
      }
      if (cVar2 == 'f') {
        if (param_1 == (undefined8 *)0x0) {
LAB_0011c3e8:
          pcVar6 = pcVar8 + 2;
          pcVar8 = pcVar8 + 2;
          cVar3 = *pcVar6;
        }
        else {
          __s_00 = (char *)FUN_0013e390(*param_1);
          sVar5 = strlen(__s_00);
          puVar9 = puVar9 + sVar5;
          pcVar6 = pcVar8 + 2;
          pcVar8 = pcVar8 + 2;
          cVar3 = *pcVar6;
        }
        goto LAB_0011c0ee;
      }
      if ('f' < cVar2) {
        if (cVar2 == 'n') {
          pcVar6 = (char *)FUN_00143090(param_3,local_58);
          puVar7 = local_44;
          local_a0 = pcVar6;
        }
        else {
          if (cVar2 != 'p') goto LAB_0011c33a;
          _Var4 = getpid();
          pcVar6 = (char *)FUN_00143090((long)_Var4,local_78);
          puVar7 = local_64;
          local_a8 = pcVar6;
        }
        puVar9 = puVar7 + ((long)puVar9 - (long)pcVar6);
        pcVar6 = pcVar8 + 2;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011c0ee;
      }
      if (cVar2 == '%') {
        pcVar6 = pcVar8 + 2;
        puVar9 = puVar9 + 1;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011c0ee;
      }
      if (cVar2 != 'd') {
LAB_0011c33a:
        pcVar6 = pcVar8 + 2;
        puVar9 = puVar9 + 2;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011c0ee;
      }
      if (param_1 == (undefined8 *)0x0) goto LAB_0011c3e8;
      if (__ptr == (void *)0x0) {
        __ptr = (void *)FUN_0013e4f0(*param_1);
      }
      __s = (char *)FUN_00133020(__ptr,0,DAT_00184b92);
      sVar5 = strlen(__s);
      puVar9 = puVar9 + sVar5;
      cVar3 = pcVar8[2];
      pcVar8 = pcVar8 + 2;
    } while (cVar3 != '\0');
LAB_0011c188:
    puVar9 = puVar9 + 1;
  }
  pcVar6 = (char *)FUN_0014c5b0(puVar9);
  cVar3 = *param_2;
  pcVar8 = pcVar6;
  if (cVar3 != '\0') {
LAB_0011c1c0:
    cVar2 = param_2[1];
    if (cVar3 == '%') {
      if (cVar2 == 'f') {
        if (__s_00 != (char *)0x0) {
          pcVar8 = stpcpy(pcVar8,__s_00);
          goto LAB_0011c2cd;
        }
LAB_0011c1fd:
        cVar3 = param_2[2];
      }
      else {
        if (cVar2 < 'g') {
          if (cVar2 == '%') {
            *pcVar8 = '%';
            pcVar1 = param_2 + 2;
            pcVar8 = pcVar8 + 1;
            param_2 = param_2 + 2;
            cVar3 = *pcVar1;
            goto joined_r0x0011c208;
          }
          if (cVar2 == 'd') {
            if (__s != (char *)0x0) {
              pcVar8 = stpcpy(pcVar8,__s);
            }
            goto LAB_0011c1fd;
          }
          *pcVar8 = '%';
          if (cVar2 == '\0') {
            pcVar8 = pcVar8 + 1;
            goto LAB_0011c210;
          }
LAB_0011c350:
          pcVar8[1] = cVar2;
          pcVar1 = param_2 + 2;
          pcVar8 = pcVar8 + 2;
          param_2 = param_2 + 2;
          cVar3 = *pcVar1;
          goto joined_r0x0011c208;
        }
        if (cVar2 == 'n') {
          pcVar8 = stpcpy(pcVar8,local_a0);
        }
        else {
          if (cVar2 != 'p') {
            *pcVar8 = '%';
            goto LAB_0011c350;
          }
          pcVar8 = stpcpy(pcVar8,local_a8);
        }
LAB_0011c2cd:
        cVar3 = param_2[2];
      }
      param_2 = param_2 + 2;
    }
    else {
      *pcVar8 = cVar3;
      param_2 = param_2 + 1;
      pcVar8 = pcVar8 + 1;
      cVar3 = cVar2;
    }
joined_r0x0011c208:
    if (cVar3 == '\0') goto LAB_0011c210;
    goto LAB_0011c1c0;
  }
  free(__ptr);
  goto LAB_0011c22f;
LAB_0011c210:
  free(__ptr);
  if (pcVar6 < pcVar8) {
    do {
      if (pcVar8[-1] != '/') break;
      pcVar8 = pcVar8 + -1;
    } while (pcVar6 != pcVar8);
  }
LAB_0011c22f:
  *pcVar8 = '\0';
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return pcVar6;
}




// Function: xheader_xhdr_name @ 0x1c460

void xheader_xhdr_name(undefined8 param_1)

{
  if (DAT_00184370 != 0) {
    FUN_0011c080(param_1,DAT_00184370,0);
    return;
  }
  FUN_00123be0(&DAT_00184370,(&PTR_s__d_PaxHeaders__f_0017f5e0)[DAT_00184a38]);
  FUN_0011c080(param_1,DAT_00184370,0);
  return;
}




// Function: xheader_ghdr_name @ 0x1c4b0

void xheader_ghdr_name(void)

{
  char *pcVar1;
  size_t sVar2;
  size_t sVar3;
  char *pcVar4;
  
  pcVar4 = DAT_00184358;
  if (DAT_00184358 == (char *)0x0) {
    pcVar4 = (&PTR_s__GlobalHead__n_0017f5f0)[DAT_00184a38];
    pcVar1 = getenv("TMPDIR");
    if (pcVar1 == (char *)0x0) {
      pcVar1 = "/tmp";
    }
    sVar2 = strlen(pcVar1);
    sVar3 = strlen(pcVar4);
    DAT_00184358 = (char *)FUN_0014c5b0(sVar2 + 1 + sVar3);
    pcVar1 = strcpy(DAT_00184358,pcVar1);
    pcVar4 = strcat(pcVar1,pcVar4);
  }
  FUN_0011c080(0,pcVar4,DAT_00184398 + 1);
  return;
}




// Function: xheader_forbid_global @ 0x1c560

void xheader_forbid_global(void)

{
  undefined8 uVar1;
  
  if (DAT_00184388 == 0) {
    return;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = dcgettext(0,"can\'t update global extended header record",5);
  error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: xheader_xattr_init @ 0x1c5b0

void xheader_xattr_init(long param_1)

{
  *(undefined8 *)(param_1 + 0x160) = 0;
  *(undefined8 *)(param_1 + 0x158) = 0;
  *(undefined8 *)(param_1 + 0x38) = 0;
  *(undefined8 *)(param_1 + 0x40) = 0;
  *(undefined8 *)(param_1 + 0x48) = 0;
  *(undefined8 *)(param_1 + 0x50) = 0;
  *(undefined8 *)(param_1 + 0x30) = 0;
  return;
}




// Function: xheader_xattr_free @ 0x1c600

void xheader_xattr_free(undefined8 *param_1,long param_2)

{
  undefined8 *puVar1;
  long lVar2;
  
  if (param_2 != 0) {
    lVar2 = 0;
    puVar1 = param_1;
    do {
      lVar2 = lVar2 + 1;
      free((void *)*puVar1);
      free((void *)puVar1[1]);
      puVar1 = puVar1 + 3;
    } while (param_2 != lVar2);
  }
  free(param_1);
  return;
}




// Function: xheader_xattr_add @ 0x1c650

void xheader_xattr_add(long param_1,char *param_2,undefined8 param_3,undefined8 param_4)

{
  size_t sVar1;
  size_t sVar2;
  undefined8 *__ptr;
  
  sVar1 = strlen(param_2);
  sVar2 = strlen("SCHILY.xattr.");
  __ptr = (undefined8 *)FUN_0014c5b0(sVar1 + 1 + sVar2);
  *__ptr = 0x782e594c49484353;
  *(undefined4 *)(__ptr + 1) = 0x72747461;
  *(undefined1 *)((long)__ptr + 0xc) = 0x2e;
  strcpy((char *)((long)__ptr + 0xd),param_2);
  FUN_0011ab50(param_1 + 0x160,param_1 + 0x158,__ptr,param_3,param_4);
  free(__ptr);
  return;
}




// Function: xattr_decoder @ 0x1c6e0

void xattr_decoder(undefined8 param_1,char *param_2,void *param_3,long param_4)

{
  long lVar1;
  undefined1 *puVar2;
  size_t sVar3;
  ulong uVar4;
  void *__dest;
  undefined1 *puVar5;
  undefined1 *puVar7;
  long in_FS_OFFSET;
  undefined1 auStack_48 [8];
  long local_40;
  undefined1 *puVar6;
  undefined1 *puVar8;
  
  puVar5 = auStack_48;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_2);
  puVar6 = auStack_48;
  puVar8 = auStack_48;
  while (puVar6 != auStack_48 + -(sVar3 + 0x18 & 0xfffffffffffff000)) {
    puVar5 = puVar8 + -0x1000;
    *(undefined8 *)(puVar8 + -8) = *(undefined8 *)(puVar8 + -8);
    puVar6 = puVar8 + -0x1000;
    puVar8 = puVar8 + -0x1000;
  }
  uVar4 = (ulong)((uint)(sVar3 + 0x18) & 0xff0);
  lVar1 = -uVar4;
  puVar7 = puVar5 + lVar1;
  puVar8 = puVar5 + lVar1;
  if (uVar4 != 0) {
    *(undefined8 *)(puVar5 + -8) = *(undefined8 *)(puVar5 + -8);
  }
  __dest = (void *)((ulong)(puVar5 + lVar1 + 0xf) & 0xfffffffffffffff0);
  *(undefined8 *)(puVar5 + lVar1 + -8) = 0x11c776;
  memcpy(__dest,param_2,sVar3 + 1);
  puVar2 = puVar5 + lVar1;
  while (puVar7 != puVar5 + (lVar1 - (param_4 + 0x18U & 0xfffffffffffff000))) {
    puVar8 = puVar2 + -0x1000;
    *(undefined8 *)(puVar2 + -8) = *(undefined8 *)(puVar2 + -8);
    puVar7 = puVar2 + -0x1000;
    puVar2 = puVar2 + -0x1000;
  }
  uVar4 = (ulong)((uint)(param_4 + 0x18U) & 0xff0);
  lVar1 = -uVar4;
  if (uVar4 != 0) {
    *(undefined8 *)(puVar8 + -8) = *(undefined8 *)(puVar8 + -8);
  }
  *(undefined8 *)(puVar8 + lVar1 + -8) = 0x11c7d2;
  memcpy((void *)((ulong)(puVar8 + lVar1 + 0xf) & 0xfffffffffffffff0),param_3,param_4 + 1);
  *(undefined8 *)(puVar8 + lVar1 + -8) = 0x11c7da;
  FUN_00119ff0(__dest);
  *(undefined8 *)(puVar8 + lVar1 + -8) = 0x11c7e6;
  sVar3 = strlen("SCHILY.xattr.");
  *(undefined8 *)(puVar8 + lVar1 + -8) = 0x11c7f8;
  FUN_0011c650(param_1,(long)__dest + sVar3,
               (void *)((ulong)(puVar8 + lVar1 + 0xf) & 0xfffffffffffffff0),param_4);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)(puVar8 + lVar1 + -8) = &UNK_0011c83d;
    __stack_chk_fail();
  }
  return;
}




// Function: xheader_xattr_copy @ 0x1c840

void xheader_xattr_copy(long param_1,undefined8 *param_2,undefined8 *param_3)

{
  undefined8 *puVar1;
  ulong uVar2;
  ulong uVar3;
  
  *param_2 = 0;
  *param_3 = 0;
  if (*(long *)(param_1 + 0x158) != 0) {
    uVar2 = 0;
    do {
      uVar3 = uVar2 + 1;
      puVar1 = (undefined8 *)(*(long *)(param_1 + 0x160) + uVar2 * 0x18);
      FUN_0011ab50(param_2,param_3,*puVar1,puVar1[1],puVar1[2]);
      uVar2 = uVar3;
    } while (uVar3 < *(ulong *)(param_1 + 0x158));
    return;
  }
  return;
}




// Function: xheader_decode @ 0x1c8c0

void xheader_decode(long param_1)

{
  char cVar1;
  long lVar2;
  long lVar3;
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0011acb0(DAT_00184388,param_1);
  FUN_0011acb0(DAT_00184378,param_1);
  lVar3 = *(long *)(param_1 + 0x170);
  if (lVar3 != 0) {
    lVar2 = *(long *)(param_1 + 0x178);
    local_28 = lVar2 + 0x200;
    while( true ) {
      cVar1 = FUN_0011bc70(lVar3,lVar2,&local_28,FUN_0011bf50,param_1);
      if (cVar1 == '\0') break;
      lVar3 = *(long *)(param_1 + 0x170);
      lVar2 = *(long *)(param_1 + 0x178);
    }
  }
  FUN_0011acb0(DAT_00184380,param_1);
  *(undefined8 *)(param_1 + 0x118) = *(undefined8 *)(param_1 + 0x88);
  if (*(char *)(param_1 + 0x150) != '\0') {
    *(undefined8 *)(param_1 + 0x88) = *(undefined8 *)(param_1 + 0x148);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xheader_decode_global @ 0x1c9a0

void xheader_decode_global(long param_1)

{
  char cVar1;
  long in_FS_OFFSET;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(long *)(param_1 + 8) != 0) {
    local_38 = *(long *)(param_1 + 0x10) + 0x200;
    FUN_0011bc20();
    do {
      cVar1 = FUN_0011bc70(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x10),&local_38,
                           FUN_0011ad20,&DAT_00184378);
    } while (cVar1 != '\0');
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: xheader_store @ 0x1ca30

void xheader_store(undefined8 param_1,long param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  
  if (*(long *)(param_2 + 0x178) != 0) {
    return;
  }
  lVar2 = FUN_0011ac30();
  if ((lVar2 != 0) && (*(long *)(lVar2 + 8) != 0)) {
    cVar1 = FUN_0011bef0(param_1);
    if (cVar1 == '\0') {
      FUN_0011a5a0(param_2 + 0x168);
      cVar1 = FUN_0011a1a0(param_1);
      if (cVar1 == '\0') {
                    /* WARNING: Could not recover jumptable at 0x0011caa4. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (**(code **)(lVar2 + 8))(param_2,param_1,param_2 + 0x168,param_3);
        return;
      }
    }
  }
  return;
}




// Function: xheader_read @ 0x1cac0

void xheader_read(long param_1,undefined8 *param_2,long param_3)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  long lVar4;
  ulong uVar5;
  uint uVar6;
  ulong uVar7;
  undefined8 *puVar8;
  ulong uVar9;
  long lVar10;
  byte bVar11;
  
  bVar11 = 0;
  lVar4 = 0;
  if (-1 < param_3) {
    lVar4 = param_3;
  }
  lVar10 = 0;
  uVar9 = lVar4 + 0x200;
  *(ulong *)(param_1 + 8) = uVar9;
  lVar1 = FUN_0014c5b0(lVar4 + 0x201);
  *(long *)(param_1 + 0x10) = lVar1;
  *(undefined1 *)(lVar1 + 0x200 + lVar4) = 0;
  do {
    uVar7 = 0x200;
    if (uVar9 < 0x201) {
      uVar7 = uVar9;
    }
    if (param_2 == (undefined8 *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,"Unexpected EOF in archive",5);
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    puVar3 = (undefined8 *)(*(long *)(param_1 + 0x10) + lVar10);
    uVar6 = (uint)uVar7;
    if (uVar6 < 8) {
      if ((uVar7 & 4) == 0) {
        if ((uVar6 != 0) && (*(undefined1 *)puVar3 = *(undefined1 *)param_2, (uVar7 & 2) != 0)) {
          *(undefined2 *)((long)puVar3 + ((uVar7 & 0xffffffff) - 2)) =
               *(undefined2 *)((long)param_2 + ((uVar7 & 0xffffffff) - 2));
        }
      }
      else {
        *(undefined4 *)puVar3 = *(undefined4 *)param_2;
        *(undefined4 *)((long)puVar3 + ((uVar7 & 0xffffffff) - 4)) =
             *(undefined4 *)((long)param_2 + ((uVar7 & 0xffffffff) - 4));
      }
    }
    else {
      *puVar3 = *param_2;
      *(undefined8 *)((long)puVar3 + ((uVar7 & 0xffffffff) - 8)) =
           *(undefined8 *)((long)param_2 + ((uVar7 & 0xffffffff) - 8));
      lVar4 = (long)puVar3 - (long)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
      puVar8 = (undefined8 *)((long)param_2 - lVar4);
      puVar3 = (undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
      for (uVar5 = (ulong)((int)lVar4 + uVar6 >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar3 = *puVar8;
        puVar8 = puVar8 + (ulong)bVar11 * -2 + 1;
        puVar3 = puVar3 + (ulong)bVar11 * -2 + 1;
      }
    }
    uVar9 = uVar9 - uVar7;
    lVar10 = lVar10 + uVar7;
    FUN_0010d1f0();
    param_2 = (undefined8 *)FUN_0010e130();
  } while (0 < (long)uVar9);
  return;
}




// Function: xheader_finish @ 0x1cc10

void xheader_finish(long *param_1)

{
  undefined8 *puVar1;
  long lVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  
  for (puVar1 = DAT_00184380; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    FUN_0011a9b0(puVar1[2],puVar1[1],param_1);
  }
  lVar2 = *param_1;
  lVar3 = *(long *)(lVar2 + 0x10);
  if (*(long *)(lVar2 + 0x18) == lVar3) {
    *(byte *)(lVar2 + 0x50) = *(byte *)(lVar2 + 0x50) | 2;
  }
  uVar5 = *(long *)(lVar2 + 0x18) + *(ulong *)(lVar2 + 0x30) & ~*(ulong *)(lVar2 + 0x30);
  uVar4 = *(ulong *)(lVar2 + 0x20);
  if (uVar5 - *(long *)(lVar2 + 8) <= *(ulong *)(lVar2 + 0x20) - *(long *)(lVar2 + 8)) {
    uVar4 = uVar5;
  }
  *(ulong *)(lVar2 + 0x18) = uVar4;
  *(ulong *)(lVar2 + 0x10) = uVar4;
  param_1[2] = lVar3;
  return;
}




// Function: xheader_destroy @ 0x1cca0

void xheader_destroy(long *param_1)

{
  if (*param_1 == 0) {
    free((void *)param_1[2]);
  }
  else {
    _obstack_free(*param_1,0);
    free((void *)*param_1);
    *param_1 = 0;
  }
  param_1[2] = 0;
  param_1[1] = 0;
  return;
}




// Function: xheader_write @ 0x1ccf0

void xheader_write(char param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  uint uVar1;
  long lVar2;
  undefined8 *puVar3;
  void *pvVar4;
  ulong uVar5;
  long lVar6;
  undefined8 *puVar7;
  size_t __n;
  undefined8 *puVar8;
  byte bVar9;
  
  bVar9 = 0;
  __n = *(size_t *)(param_4 + 8);
  if (param_1 == 'g') {
    if (DAT_00184350 != 0) {
      param_3 = DAT_00184348;
    }
  }
  else if ((param_1 == 'x') && (DAT_00184368 != 0)) {
    param_3 = DAT_00184360;
  }
  lVar2 = FUN_00112cb0(param_2,__n,param_3);
  uVar1 = (uint)__n;
  *(char *)(lVar2 + 0x9c) = param_1;
  FUN_00112f00(lVar2);
  lVar2 = *(long *)(param_4 + 0x10) + __n;
  while( true ) {
    puVar3 = (undefined8 *)FUN_0010e130();
    puVar8 = (undefined8 *)(lVar2 - __n);
    if (__n == (uVar1 & 0x1ff)) break;
    __n = __n - 0x200;
    lVar6 = (long)puVar3 - (long)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
    *puVar3 = *puVar8;
    puVar3[0x3f] = puVar8[0x3f];
    puVar8 = (undefined8 *)((long)puVar8 - lVar6);
    puVar7 = (undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
    for (uVar5 = (ulong)((int)lVar6 + 0x200U >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
      *puVar7 = *puVar8;
      puVar8 = puVar8 + (ulong)bVar9 * -2 + 1;
      puVar7 = puVar7 + (ulong)bVar9 * -2 + 1;
    }
    FUN_0010d1f0(puVar3);
    if (__n == 0) goto LAB_0011cdb8;
  }
  pvVar4 = memcpy(puVar3,puVar8,__n);
  uVar5 = 0x200 - __n;
  puVar3 = (undefined8 *)(__n + (long)pvVar4);
  uVar1 = (uint)uVar5;
  if (uVar1 < 8) {
    if ((uVar5 & 4) == 0) {
      if ((uVar1 != 0) && (*(undefined1 *)puVar3 = 0, (uVar5 & 2) != 0)) {
        *(undefined2 *)((long)puVar3 + ((uVar5 & 0xffffffff) - 2)) = 0;
      }
    }
    else {
      *(undefined4 *)puVar3 = 0;
      *(undefined4 *)((long)puVar3 + ((uVar5 & 0xffffffff) - 4)) = 0;
    }
  }
  else {
    *puVar3 = 0;
    *(undefined8 *)((long)puVar3 + ((uVar5 & 0xffffffff) - 8)) = 0;
    uVar5 = (ulong)(uVar1 + ((int)puVar3 -
                            (int)(undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8)) >> 3);
    puVar3 = (undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
    for (; uVar5 != 0; uVar5 = uVar5 - 1) {
      *puVar3 = 0;
      puVar3 = puVar3 + (ulong)bVar9 * -2 + 1;
    }
  }
  FUN_0010d1f0(pvVar4);
LAB_0011cdb8:
  FUN_0011cca0(param_4);
  if (param_1 == 'g') {
    DAT_00184398 = DAT_00184398 + 1;
  }
  return;
}




// Function: xheader_write_global @ 0x1ceb0

void xheader_write_global(long *param_1)

{
  undefined8 *puVar1;
  void *__ptr;
  
  if (DAT_00184388 != (undefined8 *)0x0) {
    FUN_0011a5a0();
    for (puVar1 = DAT_00184388; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
      FUN_0011a9b0(puVar1[2],puVar1[1],param_1);
    }
  }
  if (*param_1 != 0) {
    FUN_0011cc10(param_1);
    __ptr = (void *)FUN_0011c4b0();
    FUN_0011ccf0(0x67,__ptr,DAT_00184a20,param_1);
    free(__ptr);
    return;
  }
  return;
}




// Function: xheader_string_begin @ 0x1cf40

void xheader_string_begin(long param_1)

{
  *(undefined8 *)(param_1 + 0x18) = 0;
  return;
}




// Function: xheader_string_add @ 0x1cf50

void xheader_string_add(long param_1,char *param_2)

{
  size_t sVar1;
  
  if (*(long *)(param_1 + 0x10) != 0) {
    return;
  }
  FUN_0011a5a0();
  sVar1 = strlen(param_2);
  *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + sVar1;
  sVar1 = strlen(param_2);
  FUN_0011a7c0(param_1,param_2,sVar1);
  return;
}




// Function: xheader_string_end @ 0x1cfa0

undefined8 xheader_string_end(long *param_1,char *param_2)

{
  long lVar1;
  size_t sVar2;
  char *pcVar3;
  ulong uVar4;
  undefined8 uVar5;
  undefined1 *puVar6;
  char *__dest;
  long in_FS_OFFSET;
  bool bVar7;
  undefined1 local_58 [20];
  undefined1 local_44 [4];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar5 = 0;
  if (param_1[2] == 0) {
    FUN_0011a5a0();
    sVar2 = strlen(param_2);
    lVar1 = param_1[3];
    puVar6 = (undefined1 *)0x0;
    do {
      pcVar3 = (char *)FUN_00143090(puVar6 + lVar1 + 3 + sVar2,local_58);
      bVar7 = puVar6 != local_44 + -(long)pcVar3;
      puVar6 = local_44 + -(long)pcVar3;
    } while (bVar7);
    sVar2 = strlen(param_2);
    FUN_0011ab10(param_1,local_44 + sVar2 + -(long)pcVar3 + 2);
    FUN_0011a830(param_1,10);
    uVar4 = ~param_1[3];
    __dest = (char *)((uVar4 - (long)(local_44 + sVar2 + -(long)pcVar3 + 2)) +
                     *(long *)(*param_1 + 0x18));
    memmove((void *)(*(long *)(*param_1 + 0x18) + uVar4),__dest,param_1[3]);
    pcVar3 = stpcpy(__dest,pcVar3);
    *pcVar3 = ' ';
    pcVar3 = stpcpy(pcVar3 + 1,param_2);
    *pcVar3 = '=';
    uVar5 = 1;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_directory_meta @ 0x1d0c0

ulong hash_directory_meta(long param_1,ulong param_2)

{
  return (ulong)(*(long *)(param_1 + 0x18) + *(long *)(param_1 + 0x20)) % param_2;
}




// Function: compare_directory_meta @ 0x1d0e0

undefined8 compare_directory_meta(long param_1,long param_2)

{
  if (*(long *)(param_1 + 0x18) != *(long *)(param_2 + 0x18)) {
    return 0;
  }
  return CONCAT71((int7)((ulong)*(long *)(param_2 + 0x20) >> 8),
                  *(long *)(param_1 + 0x20) == *(long *)(param_2 + 0x20));
}




// Function: dumpdir_next @ 0x1d110

char * dumpdir_next(long *param_1)

{
  ulong uVar1;
  long *plVar2;
  long lVar3;
  size_t sVar4;
  char *__s;
  
  uVar1 = param_1[2];
  plVar2 = (long *)*param_1;
  if ((int)param_1[1] == 0) {
    __s = (char *)0x0;
    if (uVar1 < (ulong)plVar2[2]) {
      lVar3 = *(long *)(plVar2[3] + uVar1 * 8);
      param_1[2] = uVar1 + 1;
      return (char *)(lVar3 + -1);
    }
  }
  else {
    __s = (char *)(*plVar2 + uVar1);
    if (*__s == '\0') {
      __s = (char *)0x0;
    }
    else {
      sVar4 = strlen(__s);
      param_1[2] = uVar1 + 1 + sVar4;
    }
  }
  return __s;
}




// Function: free_directory @ 0x1d180

void free_directory(void *param_1)

{
  free(*(void **)((long)param_1 + 0x50));
  free(*(void **)((long)param_1 + 0x58));
  free(param_1);
  return;
}




// Function: dumpdir_free @ 0x1d1a0

void dumpdir_free(void *param_1)

{
  free(*(void **)((long)param_1 + 0x18));
  free(param_1);
  return;
}




// Function: dirlist_replace_prefix @ 0x1d1c0

void dirlist_replace_prefix(char *param_1,char *param_2)

{
  undefined8 *puVar1;
  size_t sVar2;
  size_t sVar3;
  
  sVar2 = strlen(param_1);
  sVar3 = strlen(param_2);
  for (puVar1 = DAT_001843c0; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    FUN_00123fc0(puVar1 + 0xb,param_1,sVar2,param_2,sVar3);
  }
  return;
}




// Function: dumpdir_first @ 0x1d220

void dumpdir_first(undefined8 param_1,undefined4 param_2,long *param_3)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0014c5b0(0x18);
  *puVar1 = param_1;
  *(undefined4 *)(puVar1 + 1) = param_2;
  puVar1[2] = 0;
  *param_3 = (long)puVar1;
  FUN_0011d110(puVar1);
  return;
}




// Function: make_directory @ 0x1d260

undefined8 * make_directory(char *param_1,undefined8 param_2)

{
  size_t sVar1;
  undefined8 *puVar2;
  void *pvVar3;
  size_t __n;
  
  sVar1 = strlen(param_1);
  puVar2 = (undefined8 *)FUN_0014c5b0(0x60);
  *puVar2 = 0;
  puVar2[6] = 0;
  puVar2[5] = 0;
  puVar2[8] = 0;
  *(undefined4 *)((long)puVar2 + 0x3c) = 0;
  if ((sVar1 < 2) || (__n = sVar1 - 1, param_1[sVar1 - 1] != '/')) {
    __n = sVar1;
    sVar1 = sVar1 + 1;
  }
  pvVar3 = (void *)FUN_0014c5b0(sVar1);
  puVar2[0xb] = pvVar3;
  pvVar3 = memcpy(pvVar3,param_1,__n);
  puVar2[10] = param_2;
  *(undefined1 *)((long)pvVar3 + __n) = 0;
  puVar2[9] = 0;
  return puVar2;
}




// Function: find_directory @ 0x1d310

undefined8 find_directory(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = 0;
  if (DAT_001843b0 != 0) {
    uVar1 = FUN_00125340(DAT_00184580,param_1);
    uVar2 = FUN_0011d260(param_1,uVar1);
    uVar1 = FUN_001418d0(DAT_001843b0,uVar2);
    FUN_0011d180(uVar2);
  }
  return uVar1;
}




// Function: find_directory_meta @ 0x1d370

undefined8 find_directory_meta(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  long lVar2;
  undefined8 uVar3;
  
  uVar3 = 0;
  if (DAT_001843a8 != 0) {
    lVar2 = FUN_0011d260(&DAT_00167708,0);
    lVar1 = DAT_001843a8;
    *(undefined8 *)(lVar2 + 0x18) = param_1;
    *(undefined8 *)(lVar2 + 0x20) = param_2;
    uVar3 = FUN_001418d0(lVar1,lVar2);
    FUN_0011d180(lVar2);
  }
  return uVar3;
}




// Function: attach_directory @ 0x1d3d0

void attach_directory(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00125340(DAT_00184580,param_1);
  uVar1 = FUN_0011d260(param_1,uVar1);
  if (DAT_001843b8 != (undefined8 *)0x0) {
    *DAT_001843b8 = uVar1;
    DAT_001843b8 = (undefined8 *)uVar1;
    return;
  }
  DAT_001843c0 = uVar1;
  DAT_001843b8 = (undefined8 *)uVar1;
  return;
}




// Function: compare_dirnames @ 0x1d420

void compare_dirnames(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: compare_directory_canonical_names @ 0x1d430

bool compare_directory_canonical_names(long param_1,long param_2)

{
  int iVar1;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x50),*(char **)(param_2 + 0x50));
  return iVar1 == 0;
}




// Function: dumpdir_locate @ 0x1d450

long dumpdir_locate(long param_1,undefined8 param_2)

{
  long *plVar1;
  undefined8 local_10 [2];
  
  if (param_1 != 0) {
    local_10[0] = param_2;
    plVar1 = bsearch(local_10,*(void **)(param_1 + 0x18),*(size_t *)(param_1 + 0x10),8,FUN_0011d420)
    ;
    if (plVar1 != (long *)0x0) {
      return *plVar1 + -1;
    }
  }
  return 0;
}




// Function: dumpdir_create0 @ 0x1d4a0

undefined8 * dumpdir_create0(char *param_1,char *param_2)

{
  size_t sVar1;
  char *pcVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  long lVar5;
  long lVar6;
  size_t sVar7;
  long lVar8;
  char cVar9;
  char *pcVar10;
  long local_50;
  
  cVar9 = *param_1;
  if (cVar9 == '\0') {
    local_50 = 0;
    lVar8 = 1;
    lVar6 = 0x21;
    lVar5 = 0;
    sVar7 = 1;
  }
  else {
    local_50 = 0;
    sVar7 = 1;
    lVar5 = 0;
    pcVar10 = param_1;
    do {
      sVar1 = strlen(pcVar10);
      sVar7 = sVar7 + sVar1 + 1;
      if (param_2 == (char *)0x0) {
LAB_0011d505:
        local_50 = local_50 + 1;
      }
      else {
        pcVar2 = strchr(param_2,(int)cVar9);
        if (pcVar2 != (char *)0x0) goto LAB_0011d505;
      }
      pcVar10 = pcVar10 + sVar1 + 1;
      lVar5 = lVar5 + 1;
      cVar9 = *pcVar10;
    } while (cVar9 != '\0');
    lVar6 = sVar7 + 0x20;
    lVar8 = local_50 + 1;
  }
  puVar3 = (undefined8 *)FUN_0014c5b0(lVar6);
  *puVar3 = puVar3 + 4;
  memcpy(puVar3 + 4,param_1,sVar7);
  puVar3[1] = lVar5;
  puVar3[2] = local_50;
  puVar4 = (undefined8 *)FUN_0014c720(lVar8,8);
  pcVar10 = (char *)*puVar3;
  puVar3[3] = puVar4;
  cVar9 = *pcVar10;
  if (cVar9 != '\0') {
    lVar8 = 0;
    do {
      if (param_2 == (char *)0x0) {
LAB_0011d592:
        puVar4[lVar8] = pcVar10 + 1;
        lVar8 = lVar8 + 1;
      }
      else {
        pcVar2 = strchr(param_2,(int)cVar9);
        if (pcVar2 != (char *)0x0) goto LAB_0011d592;
      }
      sVar7 = strlen(pcVar10);
      pcVar10 = pcVar10 + sVar7 + 1;
      cVar9 = *pcVar10;
    } while (cVar9 != '\0');
    puVar4 = puVar4 + lVar8;
  }
  *puVar4 = 0;
  return puVar3;
}




// Function: makedumpdir @ 0x1d600

void makedumpdir(long param_1,char *param_2)

{
  char cVar1;
  byte bVar2;
  size_t sVar3;
  undefined8 *puVar4;
  long lVar5;
  char *pcVar6;
  undefined8 uVar7;
  undefined8 *__base;
  char *pcVar8;
  char *pcVar9;
  size_t __nmemb;
  long lVar10;
  char *local_40;
  
  lVar10 = 0;
  if (*(int *)(param_1 + 0x38) != 2) {
    lVar5 = *(long *)(param_1 + 0x40);
    if (lVar5 == 0) {
      cVar1 = *param_2;
      lVar10 = *(long *)(param_1 + 0x28);
      goto joined_r0x0011d63a;
    }
    lVar10 = *(long *)(lVar5 + 0x30);
    if (lVar10 == 0) {
      lVar10 = *(long *)(lVar5 + 0x28);
    }
  }
  cVar1 = *param_2;
joined_r0x0011d63a:
  if (cVar1 == '\0') {
    __nmemb = 0;
    __base = (undefined8 *)FUN_0014c720(0);
    local_40 = (char *)0x1;
    puVar4 = __base;
    if (*param_2 != '\0') goto LAB_0011d690;
    qsort(__base,0,8,FUN_0011d420);
    pcVar9 = (char *)FUN_0014c5b0(1);
    local_40 = pcVar9;
  }
  else {
    lVar5 = 0;
    __nmemb = 0;
    pcVar9 = param_2;
    do {
      __nmemb = __nmemb + 1;
      sVar3 = strlen(pcVar9);
      pcVar9 = pcVar9 + sVar3 + 1;
      lVar5 = sVar3 + 2 + lVar5;
    } while (*pcVar9 != '\0');
    local_40 = (char *)(lVar5 + 1);
    puVar4 = (undefined8 *)FUN_0014c720(__nmemb,8);
    cVar1 = *param_2;
    __base = puVar4;
    while (cVar1 != '\0') {
LAB_0011d690:
      *puVar4 = param_2;
      puVar4 = puVar4 + 1;
      sVar3 = strlen(param_2);
      param_2 = param_2 + sVar3 + 1;
      cVar1 = *param_2;
    }
    sVar3 = 0;
    qsort(__base,__nmemb,8,FUN_0011d420);
    local_40 = (char *)FUN_0014c5b0(local_40);
    pcVar9 = local_40;
    if (__nmemb != 0) {
      do {
        lVar5 = FUN_0011d450(lVar10,__base[sVar3]);
        if (lVar5 == 0) {
          bVar2 = -(*(long *)(param_1 + 0x48) == 0) & 0x10;
        }
        else {
          bVar2 = -(*(long *)(param_1 + 0x48) == 0) & 0xd7;
        }
        *pcVar9 = bVar2 + 0x49;
        pcVar6 = (char *)__base[sVar3];
        pcVar8 = pcVar9 + 1;
        do {
          cVar1 = *pcVar6;
          pcVar6 = pcVar6 + 1;
          pcVar9 = pcVar8 + 1;
          *pcVar8 = cVar1;
          pcVar8 = pcVar9;
        } while (cVar1 != '\0');
        sVar3 = sVar3 + 1;
      } while (__nmemb != sVar3);
    }
  }
  uVar7 = *(undefined8 *)(param_1 + 0x28);
  *pcVar9 = '\0';
  *(undefined8 *)(param_1 + 0x30) = uVar7;
  uVar7 = FUN_0011d4a0(local_40,0);
  *(undefined8 *)(param_1 + 0x28) = uVar7;
  free(local_40);
  free(__base);
  return;
}




// Function: dumpdir_create @ 0x1d7f0

void dumpdir_create(undefined8 param_1)

{
  FUN_0011d4a0(param_1,&DAT_001619db);
  return;
}




// Function: maketagdumpdir @ 0x1d800

void maketagdumpdir(long param_1)

{
  void *__src;
  size_t sVar1;
  undefined1 *__ptr;
  undefined8 uVar2;
  
  sVar1 = strlen(*(char **)(param_1 + 0x48));
  __ptr = (undefined1 *)FUN_0014c5b0(sVar1 + 3);
  __src = *(void **)(param_1 + 0x48);
  *__ptr = 0x59;
  memcpy(__ptr + 1,__src,sVar1 + 1);
  uVar2 = *(undefined8 *)(param_1 + 0x28);
  __ptr[sVar1 + 2] = 0;
  *(undefined8 *)(param_1 + 0x30) = uVar2;
  uVar2 = FUN_0011d4a0(__ptr,0);
  *(undefined8 *)(param_1 + 0x28) = uVar2;
  free(__ptr);
  return;
}




// Function: dumpdir_ok @ 0x1d860

undefined8 dumpdir_ok(char *param_1)

{
  bool bVar1;
  size_t sVar2;
  undefined8 uVar3;
  char cVar4;
  char *pcVar5;
  int iVar6;
  
  cVar4 = *param_1;
  if (cVar4 != '\0') {
    iVar6 = 0;
    bVar1 = false;
    do {
      if (iVar6 == 0) {
        if (cVar4 == 'T') {
LAB_0011da57:
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          pcVar5 = "Malformed dumpdir: \'T\' not preceded by \'R\'";
          goto LAB_0011da2b;
        }
LAB_0011d8d3:
        if (cVar4 == 'X') {
          if (bVar1) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            pcVar5 = "Malformed dumpdir: \'X\' duplicated";
LAB_0011da2b:
            uVar3 = dcgettext(0,pcVar5,5);
            error(0,0,uVar3);
            DAT_00184d58 = 2;
            return 0;
          }
          bVar1 = true;
        }
        else if (cVar4 == 'R') {
          if (param_1[1] == '\0') {
            if (!bVar1) {
              if (DAT_00184d50 != (code *)0x0) {
                (*DAT_00184d50)();
              }
              pcVar5 = "Malformed dumpdir: empty name in \'R\'";
              goto LAB_0011da2b;
            }
            iVar6 = 0x54;
            bVar1 = false;
          }
          else {
            iVar6 = 0x54;
          }
        }
      }
      else {
        if (cVar4 != iVar6) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar3 = dcgettext(0,"Malformed dumpdir: expected \'%c\' but found %#3o",5);
          error(0,0,uVar3,iVar6,cVar4);
          DAT_00184d58 = 2;
          return 0;
        }
        if (cVar4 != 'T') goto LAB_0011d8d3;
        if (iVar6 != 0x54) goto LAB_0011da57;
        if ((param_1[1] == '\0') && (!bVar1)) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          pcVar5 = "Malformed dumpdir: empty name in \'T\'";
          goto LAB_0011da2b;
        }
        iVar6 = 0;
      }
      sVar2 = strlen(param_1);
      param_1 = param_1 + sVar2 + 1;
      cVar4 = *param_1;
    } while (cVar4 != '\0');
    if (iVar6 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Malformed dumpdir: expected \'%c\' but found end of data",5);
      error(0,0,uVar3,iVar6);
      DAT_00184d58 = 2;
      return 0;
    }
    if ((bVar1) && (((byte)DAT_00183b80 & 2) != 0)) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Malformed dumpdir: \'X\' never used",5);
      error(0,0,uVar3);
    }
  }
  return 1;
}




// Function: note_directory @ 0x1dac0

long note_directory(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,char param_6,char param_7,long param_8)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  
  lVar1 = FUN_0011d3d0();
  *(undefined8 *)(lVar1 + 0x10) = param_3;
  *(undefined8 *)(lVar1 + 8) = param_2;
  *(undefined8 *)(lVar1 + 0x18) = param_4;
  *(undefined8 *)(lVar1 + 0x20) = param_5;
  *(undefined4 *)(lVar1 + 0x38) = 1;
  if (param_6 != '\0') {
    *(uint *)(lVar1 + 0x3c) = *(uint *)(lVar1 + 0x3c) | 2;
  }
  if (param_7 != '\0') {
    *(uint *)(lVar1 + 0x3c) = *(uint *)(lVar1 + 0x3c) | 4;
  }
  uVar2 = 0;
  if (param_8 != 0) {
    uVar2 = FUN_0011d7f0(param_8);
  }
  lVar3 = DAT_001843b0;
  *(undefined8 *)(lVar1 + 0x28) = uVar2;
  if ((((lVar3 != 0) ||
       (lVar3 = FUN_00141b30(0,0,FUN_0011e1a0,FUN_0011d430,0), DAT_001843b0 = lVar3, lVar3 != 0)) &&
      (lVar3 = FUN_00142140(lVar3,lVar1), lVar3 != 0)) &&
     (((DAT_001843a8 != 0 ||
       (DAT_001843a8 = FUN_00141b30(0,0,FUN_0011d0c0,FUN_0011d0e0,0), DAT_001843a8 != 0)) &&
      (lVar3 = FUN_00142140(DAT_001843a8,lVar1), lVar3 != 0)))) {
    return lVar1;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00116420();
}




// Function: procdir @ 0x1dbe0

long procdir(char *param_1,long param_2,uint param_3,undefined1 *param_4)

{
  ulong uVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  long lVar6;
  long lVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  char *pcVar11;
  long in_FS_OFFSET;
  undefined1 auVar12 [16];
  undefined8 local_48;
  long local_40;
  
  uVar1 = *(ulong *)(param_2 + 0x58);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar6 = FUN_0011d310();
  if (lVar6 == 0) {
    lVar7 = FUN_0011d370(*(undefined8 *)(param_2 + 0x58),*(undefined8 *)(param_2 + 0x60));
    uVar8 = *(undefined8 *)(param_2 + 0x60);
    uVar9 = *(undefined8 *)(param_2 + 0x58);
    auVar12 = FUN_0014bd20(param_2 + 0x58);
    lVar6 = FUN_0011dac0(param_1,auVar12._0_8_,auVar12._8_8_,uVar9,uVar8,uVar1 >> 0x3f,1);
    if (lVar7 == 0) {
      *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 8;
      if ((DAT_00183b80._1_1_ & 0x10) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar8 = FUN_0014aa70(param_1);
        uVar9 = dcgettext(0,"%s: Directory is new",5);
        error(0,0,uVar9,uVar8);
      }
      if (DAT_00184b28 == 0) {
        auVar12 = FUN_0014bd20(param_2 + 0x58);
        uVar9 = DAT_00184b08;
        uVar8 = DAT_00184b00;
        iVar3 = FUN_0014be80(auVar12._0_8_,auVar12._8_8_,DAT_00184b00,DAT_00184b08);
        if (iVar3 < 0) goto LAB_0011e09c;
        if (DAT_00184b8c != 0) {
          auVar12 = FUN_0014bd10(param_2 + 0x58);
          iVar3 = FUN_0014be80(auVar12._0_8_,auVar12._8_8_,uVar8,uVar9);
          if (iVar3 < 0) goto LAB_0011e09c;
        }
        uVar5 = 1;
      }
      else {
LAB_0011e09c:
        uVar5 = 2;
      }
      *(undefined4 *)(lVar6 + 0x38) = uVar5;
      bVar2 = false;
    }
    else {
      pcVar11 = *(char **)(lVar7 + 0x58);
      iVar3 = strcmp(pcVar11,param_1);
      if (iVar3 != 0) {
        if ((DAT_00183b80._1_1_ & 0x20) != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
            pcVar11 = *(char **)(lVar7 + 0x58);
          }
          uVar8 = FUN_0014ac40(1,pcVar11);
          uVar9 = FUN_0014aa70(param_1);
          uVar10 = dcgettext(0,"%s: Directory has been renamed from %s",5);
          error(0,0,uVar10,uVar9,uVar8);
          pcVar11 = *(char **)(lVar7 + 0x58);
        }
        *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 0x10;
        *(long *)(lVar6 + 0x40) = lVar7;
        *(uint *)(lVar7 + 0x3c) = *(uint *)(lVar7 + 0x3c) & 0xffffffef;
        FUN_0011d1c0(pcVar11,param_1);
      }
      *(undefined4 *)(lVar6 + 0x38) = 1;
      bVar2 = false;
    }
LAB_0011dd95:
    if (((DAT_00184ad1 == '\0') || (*(long *)(param_2 + 0x198) == 0)) ||
       (*(long *)(param_2 + 0x58) == *(long *)(*(long *)(param_2 + 0x198) + 0x58))) {
      if (((param_3 & 0x10) != 0) && (*(uint *)(lVar6 + 0x38) = param_3 & 3, (param_3 & 3) == 0)) {
        *param_4 = 0x4e;
      }
      if ((bVar2) && ((DAT_00183b80._1_1_ & 0x20) != 0)) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar8 = FUN_0014aa70(param_1);
        uVar9 = dcgettext(0,"%s: Directory has been renamed",5);
        error(0,0,uVar9,uVar8);
      }
      iVar3 = *(int *)(lVar6 + 0x38);
    }
    else {
      if ((DAT_00183b80._2_1_ & 4) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar8 = FUN_0014aa70(*(undefined8 *)(lVar6 + 0x58));
        uVar9 = dcgettext(0,"%s: directory is on a different filesystem; not dumped",5);
        error(0,0,uVar9,uVar8);
      }
      *(undefined4 *)(lVar6 + 0x38) = 0;
      if (*(long *)(lVar6 + 0x28) == 0) {
        *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 1;
        goto LAB_0011dd58;
      }
      FUN_0011d1a0();
      *(undefined8 *)(lVar6 + 0x28) = 0;
      iVar3 = *(int *)(lVar6 + 0x38);
    }
    *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 1;
    if (iVar3 != 0) {
      iVar3 = FUN_00114160(param_2,&local_48);
      if (iVar3 == 2) {
        uVar8 = dcgettext(0,"contents not dumped",5);
        FUN_00112ad0(param_1,local_48,uVar8);
        *(undefined8 *)(lVar6 + 0x48) = local_48;
      }
      else if (iVar3 == 3) {
        uVar8 = dcgettext(0,"directory not dumped",5);
        FUN_00112ad0(param_1,local_48,uVar8);
        *param_4 = 0x4e;
        *(undefined4 *)(lVar6 + 0x38) = 0;
      }
      else if (iVar3 == 1) {
        uVar8 = dcgettext(0,"contents not dumped",5);
        FUN_00112ad0(param_1,local_48,uVar8);
        *(undefined4 *)(lVar6 + 0x38) = 0;
        *(undefined8 *)(lVar6 + 0x48) = local_48;
      }
    }
  }
  else {
    if ((*(byte *)(lVar6 + 0x3c) & 1) == 0) {
LAB_0011dc3b:
      iVar3 = strcmp(*(char **)(lVar6 + 0x58),param_1);
      if (iVar3 == 0) {
        if ((DAT_00184b20 == '\0') ||
           ((((*(byte *)(lVar6 + 0x3c) & 2) != 0 && ((long)uVar1 < 0)) ||
            (lVar7 = *(long *)(param_2 + 0x58), *(long *)(lVar6 + 0x18) == lVar7)))) {
          if (*(long *)(lVar6 + 0x20) != *(long *)(param_2 + 0x60)) {
            lVar7 = *(long *)(param_2 + 0x58);
            goto LAB_0011dc86;
          }
          *(undefined4 *)(lVar6 + 0x38) = 1;
          bVar2 = false;
          uVar4 = *(uint *)(lVar6 + 0x3c);
        }
        else {
LAB_0011dc86:
          lVar7 = FUN_0011d370(lVar7);
          if (lVar7 == 0) {
            uVar8 = *(undefined8 *)(param_2 + 0x58);
            *(undefined4 *)(lVar6 + 0x38) = 2;
            bVar2 = true;
            *(undefined8 *)(lVar6 + 0x18) = uVar8;
            *(undefined8 *)(lVar6 + 0x20) = *(undefined8 *)(param_2 + 0x60);
          }
          else {
            pcVar11 = *(char **)(lVar7 + 0x58);
            iVar3 = strcmp(pcVar11,param_1);
            if (iVar3 != 0) {
              if ((DAT_00183b80._1_1_ & 0x20) != 0) {
                if (DAT_00184d50 != (code *)0x0) {
                  (*DAT_00184d50)();
                  pcVar11 = *(char **)(lVar7 + 0x58);
                }
                uVar8 = FUN_0014ac40(1,pcVar11);
                uVar9 = FUN_0014aa70(param_1);
                uVar10 = dcgettext(0,"%s: Directory has been renamed from %s",5);
                error(0,0,uVar10,uVar9,uVar8);
                pcVar11 = *(char **)(lVar7 + 0x58);
              }
              *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 0x10;
              *(long *)(lVar6 + 0x40) = lVar7;
              *(uint *)(lVar7 + 0x3c) = *(uint *)(lVar7 + 0x3c) & 0xffffffef;
              FUN_0011d1c0(pcVar11,param_1);
            }
            *(undefined4 *)(lVar6 + 0x38) = 1;
            bVar2 = false;
          }
          uVar4 = *(uint *)(lVar6 + 0x3c);
          if ((long)uVar1 < 0) {
            uVar4 = *(uint *)(lVar6 + 0x3c) | 2;
          }
        }
        *(uint *)(lVar6 + 0x3c) = uVar4 | 4;
        goto LAB_0011dd95;
      }
    }
    else if ((param_3 & 0x20) != 0) {
      FUN_00123be0(lVar6 + 0x58,param_1);
      goto LAB_0011dc3b;
    }
    *param_4 = 0x4e;
  }
LAB_0011dd58:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar6;
}




// Function: hash_directory_canonical_name @ 0x1e1a0

void hash_directory_canonical_name(long param_1)

{
  FUN_00141ac0(*(undefined8 *)(param_1 + 0x50));
  return;
}




// Function: obstack_code_rename @ 0x1e1b0

void obstack_code_rename(long param_1,char *param_2,char *param_3)

{
  ulong uVar1;
  size_t sVar2;
  undefined1 *puVar3;
  void *pvVar4;
  
  if (*param_2 == '\0') {
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) == puVar3) {
LAB_0011e29a:
      _obstack_newchunk(param_1,1);
      puVar3 = *(undefined1 **)(param_1 + 0x18);
    }
  }
  else {
    param_2 = (char *)FUN_00133020(param_2,0,DAT_00184b92);
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) == puVar3) goto LAB_0011e29a;
  }
  *(undefined1 **)(param_1 + 0x18) = puVar3 + 1;
  *puVar3 = 0x52;
  sVar2 = strlen(param_2);
  pvVar4 = *(void **)(param_1 + 0x18);
  uVar1 = sVar2 + 1;
  if ((ulong)(*(long *)(param_1 + 0x20) - (long)pvVar4) < uVar1) {
    _obstack_newchunk(param_1,uVar1);
    pvVar4 = *(void **)(param_1 + 0x18);
  }
  memcpy(pvVar4,param_2,uVar1);
  puVar3 = (undefined1 *)(*(long *)(param_1 + 0x18) + uVar1);
  *(undefined1 **)(param_1 + 0x18) = puVar3;
  if (*param_3 == '\0') {
    if (*(undefined1 **)(param_1 + 0x20) != puVar3) goto LAB_0011e230;
  }
  else {
    param_3 = (char *)FUN_00133020(param_3,0,DAT_00184b92);
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) != puVar3) goto LAB_0011e230;
  }
  _obstack_newchunk(param_1,1);
  puVar3 = *(undefined1 **)(param_1 + 0x18);
LAB_0011e230:
  *(undefined1 **)(param_1 + 0x18) = puVar3 + 1;
  *puVar3 = 0x54;
  sVar2 = strlen(param_3);
  pvVar4 = *(void **)(param_1 + 0x18);
  uVar1 = sVar2 + 1;
  if ((ulong)(*(long *)(param_1 + 0x20) - (long)pvVar4) < uVar1) {
    _obstack_newchunk(param_1,uVar1);
    pvVar4 = *(void **)(param_1 + 0x18);
  }
  memcpy(pvVar4,param_3,uVar1);
  *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + uVar1;
  return;
}




// Function: store_rename @ 0x1e320

void store_rename(long param_1,long param_2)

{
  ulong __n;
  long lVar1;
  char *__s;
  undefined1 *puVar2;
  size_t sVar3;
  long lVar4;
  void *__dest;
  
  lVar1 = param_1;
  do {
    lVar4 = lVar1;
    if (lVar4 == 0) {
      for (; (param_1 != 0 && (*(long *)(param_1 + 0x40) != 0)); param_1 = *(long *)(param_1 + 0x40)
          ) {
        FUN_0011e1b0(param_2,*(undefined8 *)(*(long *)(param_1 + 0x40) + 0x58),
                     *(undefined8 *)(param_1 + 0x58));
      }
      return;
    }
    lVar1 = *(long *)(lVar4 + 0x40);
  } while (*(long *)(lVar4 + 0x40) != param_1);
  __s = (char *)FUN_0013e4f0(*(undefined8 *)(param_1 + 0x58));
  puVar2 = *(undefined1 **)(param_2 + 0x18);
  if (*(undefined1 **)(param_2 + 0x20) == puVar2) {
    _obstack_newchunk(param_2,1);
    puVar2 = *(undefined1 **)(param_2 + 0x18);
  }
  *(undefined1 **)(param_2 + 0x18) = puVar2 + 1;
  *puVar2 = 0x58;
  sVar3 = strlen(__s);
  __dest = *(void **)(param_2 + 0x18);
  __n = sVar3 + 1;
  if ((ulong)(*(long *)(param_2 + 0x20) - (long)__dest) < __n) {
    _obstack_newchunk(param_2,__n);
    __dest = *(void **)(param_2 + 0x18);
  }
  memcpy(__dest,__s,__n);
  *(long *)(param_2 + 0x18) = *(long *)(param_2 + 0x18) + __n;
  FUN_0011e1b0(param_2,*(undefined8 *)(param_1 + 0x58),&DAT_00167708);
  for (; lVar4 != param_1; param_1 = *(long *)(param_1 + 0x40)) {
    FUN_0011e1b0(param_2,*(undefined8 *)(*(long *)(param_1 + 0x40) + 0x58),
                 *(undefined8 *)(param_1 + 0x58));
  }
  FUN_0011e1b0(param_2,&DAT_00167708,*(undefined8 *)(lVar4 + 0x58));
  free(__s);
  return;
}




// Function: read_incr_db_01 @ 0x1e470

void read_incr_db_01(int param_1,char *param_2)

{
  char cVar1;
  char cVar2;
  char *pcVar3;
  __ssize_t _Var4;
  uintmax_t uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined8 uVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  char **local_70;
  long local_68;
  char *local_58;
  size_t local_50;
  char *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = (char *)0x0;
  local_50 = 0;
  if (param_1 == 1) {
    _Var4 = getline(&local_58,&local_50,DAT_001843a0);
    if (_Var4 < 1) {
      FUN_00132820(DAT_00184b28);
      free(local_58);
      goto LAB_0011e6fa;
    }
    auVar10 = FUN_001242f0(local_58,&local_48,0);
    DAT_00184b08 = auVar10._8_8_;
    DAT_00184b00 = auVar10._0_8_;
    cVar1 = FUN_00123ba0(DAT_00184b00);
    if (cVar1 == '\0') {
      uVar9 = 2;
      goto LAB_0011e88b;
    }
    local_68 = 2;
    if (*local_48 != '\0') {
      pcVar3 = local_48 + 1;
      piVar8 = __errno_location();
      *piVar8 = 0;
      uVar5 = strtoumax(pcVar3,&local_48,10);
      if (*piVar8 == 0) {
        if (uVar5 < 1000000000) {
          if (local_48 != pcVar3) {
            local_68 = 2;
            DAT_00184b08 = uVar5;
            goto LAB_0011e5ad;
          }
        }
        else {
          *piVar8 = 0x22;
        }
      }
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar9 = dcgettext(0,"Invalid time stamp",5);
      uVar6 = FUN_0014aa70(DAT_00184b28);
      error(0,*piVar8,"%s:%ld: %s",uVar6,2,uVar9);
      DAT_00184d58 = 2;
      DAT_00184b00 = 0x8000000000000000;
      DAT_00184b08 = 0xffffffffffffffff;
      local_68 = 2;
    }
  }
  else {
    pcVar3 = strdup(param_2);
    local_58 = pcVar3;
    local_50 = strlen(pcVar3);
    local_50 = local_50 + 1;
    auVar10 = FUN_001242f0(pcVar3,&local_48,0);
    DAT_00184b08 = auVar10._8_8_;
    DAT_00184b00 = auVar10._0_8_;
    cVar1 = FUN_00123ba0(DAT_00184b00);
    local_68 = 1;
    if (cVar1 == '\0') {
      uVar9 = 1;
LAB_0011e88b:
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar6 = dcgettext(0,"Invalid time stamp",5);
      uVar7 = FUN_0014aa70(DAT_00184b28);
      piVar8 = __errno_location();
      error(0,*piVar8,"%s:%ld: %s",uVar7,uVar9,uVar6);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
  }
LAB_0011e5ad:
  while( true ) {
    local_70 = &local_58;
    _Var4 = getline(local_70,&local_50,DAT_001843a0);
    if ((int)_Var4 < 1) break;
    cVar1 = *local_58;
    local_68 = local_68 + 1;
    pcVar3 = local_58 + (cVar1 == '+');
    if (local_58[(long)(int)_Var4 + -1] == '\n') {
      local_58[(long)(int)_Var4 + -1] = '\0';
    }
    piVar8 = __errno_location();
    uVar9 = 0;
    uVar5 = 0;
    if (param_1 == 1) {
      auVar10 = FUN_001242f0(pcVar3,&local_48,0);
      pcVar3 = local_48;
      uVar9 = auVar10._0_8_;
      cVar2 = FUN_00123ba0(uVar9,auVar10._8_8_);
      if ((cVar2 == '\0') || (*pcVar3 != ' ')) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar3 = "Invalid modification time";
        goto LAB_0011e697;
      }
      *piVar8 = 0;
      uVar5 = strtoumax(pcVar3,&local_48,10);
      if (uVar5 < 1000000000) {
        if (((*piVar8 == 0) && (local_48 != pcVar3)) && (pcVar3 = local_48, *local_48 == ' '))
        goto LAB_0011e520;
      }
      else if (*piVar8 == 0) {
        *piVar8 = 0x22;
      }
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar3 = "Invalid modification time (nanoseconds)";
LAB_0011e697:
      uVar9 = dcgettext(0,pcVar3,5);
      uVar6 = FUN_0014aa70(DAT_00184b28);
      error(0,*piVar8,"%s:%ld: %s",uVar6,local_68,uVar9);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
LAB_0011e520:
    uVar6 = FUN_001240a0(pcVar3,&local_48,0,0xffffffffffffffff);
    if ((*piVar8 != 0) || (*local_48 != ' ')) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar3 = "Invalid device number";
      goto LAB_0011e697;
    }
    uVar7 = FUN_001240a0(local_48,&local_48,0,0xffffffffffffffff);
    if ((*piVar8 != 0) || (*local_48 != ' ')) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar3 = "Invalid inode number";
      goto LAB_0011e697;
    }
    pcVar3 = local_48 + 1;
    FUN_00123c70(pcVar3);
    FUN_0011dac0(pcVar3,uVar9,uVar5,uVar6,uVar7,cVar1 == '+',0,0);
  }
  free(local_58);
LAB_0011e6fa:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_obstack @ 0x1e960

int read_obstack(FILE *param_1,long param_2,long *param_3)

{
  int iVar1;
  undefined1 *puVar2;
  long lVar3;
  
  lVar3 = 0;
  iVar1 = getc_unlocked(param_1);
  while (1 < iVar1 + 1U) {
    puVar2 = *(undefined1 **)(param_2 + 0x18);
    if (*(undefined1 **)(param_2 + 0x20) == puVar2) {
      _obstack_newchunk(param_2,1);
      puVar2 = *(undefined1 **)(param_2 + 0x18);
    }
    lVar3 = lVar3 + 1;
    *(undefined1 **)(param_2 + 0x18) = puVar2 + 1;
    *puVar2 = (char)iVar1;
    iVar1 = getc_unlocked(param_1);
  }
  puVar2 = *(undefined1 **)(param_2 + 0x18);
  if (*(undefined1 **)(param_2 + 0x20) == puVar2) {
    _obstack_newchunk(param_2,1);
    puVar2 = *(undefined1 **)(param_2 + 0x18);
  }
  *(undefined1 **)(param_2 + 0x18) = puVar2 + 1;
  *puVar2 = 0;
  *param_3 = lVar3;
  return iVar1;
}




// Function: read_num @ 0x1ea10

bool read_num(FILE *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 *param_5)

{
  undefined1 uVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  __off_t _Var5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  int *piVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  int iVar13;
  long lVar14;
  long in_FS_OFFSET;
  bool bVar15;
  undefined1 *puVar16;
  undefined1 *puVar17;
  undefined1 local_b8 [32];
  undefined1 local_98 [32];
  undefined1 local_78 [32];
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = getc_unlocked(param_1);
  bVar15 = uVar3 == 0x2d;
  lVar14 = 0;
  do {
    iVar13 = (int)lVar14;
    bVar2 = iVar13 == 0 && bVar15;
    uVar1 = (char)uVar3;
    if (iVar13 != 0 || !bVar15) {
      if (9 < uVar3 - 0x30) {
        local_b8[iVar13] = 0;
        if ((int)uVar3 < 0) {
          iVar4 = ferror_unlocked(param_1);
          if (iVar4 != 0) {
            FUN_001329b0(DAT_00184b28);
            goto LAB_0011ed60;
          }
          if (iVar13 != 0) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            uVar6 = dcgettext(0,"Unexpected EOF in snapshot file",5);
            uVar7 = FUN_0014aa70(DAT_00184b28);
            error(0,0,"%s: %s",uVar7,uVar6);
                    /* WARNING: Subroutine does not return */
            FUN_001163d0();
          }
        }
        else {
          if (uVar3 != 0) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            _Var5 = ftello(param_1);
            uVar6 = FUN_00142ff0(_Var5,local_98);
            uVar7 = FUN_0014aa70(DAT_00184b28);
            uVar8 = dcgettext(0,"%s: byte %s: %s %s followed by invalid byte 0x%02x",5);
            puVar17 = (undefined1 *)(ulong)uVar3;
            puVar16 = local_b8;
            goto LAB_0011eaef;
          }
          uVar6 = param_4;
          uVar7 = FUN_001240a0(local_b8,0,param_3);
          *param_5 = uVar7;
          piVar9 = __errno_location();
          iVar13 = *piVar9;
          if (iVar13 != 0) {
            if (iVar13 == 0x22) {
              if (DAT_00184d50 != (code *)0x0) {
                (*DAT_00184d50)();
              }
              uVar7 = FUN_00143090(param_4,local_58);
              uVar8 = FUN_00142f50(param_3,local_78);
              _Var5 = ftello(param_1);
              uVar10 = FUN_00142ff0(_Var5,local_98);
              uVar11 = FUN_0014aa70(DAT_00184b28);
              uVar12 = dcgettext(0,"%s: byte %s: (valid range %s..%s)\n\t%s %s",5);
              error(0,0x22,uVar12,uVar11,uVar10,uVar8,uVar7,param_2,local_b8,uVar6);
                    /* WARNING: Subroutine does not return */
              FUN_001163d0();
            }
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            _Var5 = ftello(param_1);
            uVar6 = FUN_00142ff0(_Var5,local_98);
            uVar7 = FUN_0014aa70(DAT_00184b28);
            uVar8 = dcgettext(0,"%s: byte %s: %s %s",5);
            error(0,iVar13,uVar8,uVar7,uVar6,param_2,local_b8,uVar8);
                    /* WARNING: Subroutine does not return */
            FUN_001163d0();
          }
          bVar2 = true;
        }
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return bVar2;
        }
LAB_0011ed60:
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      local_b8[lVar14] = (char)uVar3;
      puVar17 = local_b8;
      uVar1 = local_b8[0];
      if (lVar14 == 0x14) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        _Var5 = ftello(param_1);
        uVar6 = FUN_00142ff0(_Var5,local_98);
        uVar7 = FUN_0014aa70(DAT_00184b28);
        uVar8 = dcgettext(0,"%s: byte %s: %s %.*s... too long",5);
        puVar16 = (undefined1 *)0x15;
LAB_0011eaef:
        error(0,0,uVar8,uVar7,uVar6,param_2,puVar16,puVar17);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
    }
    local_b8[0] = uVar1;
    lVar14 = lVar14 + 1;
    uVar3 = getc_unlocked(param_1);
  } while( true );
}




// Function: read_timespec @ 0x1ed70

void read_timespec(undefined8 param_1,undefined8 *param_2)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ea10(param_1,"sec",0x8000000000000000,0x7fffffffffffffff,&local_30);
  if (cVar1 != '\0') {
    cVar1 = FUN_0011ea10(param_1,"nsec",0,999999999,&local_28);
    if (cVar1 != '\0') {
      *param_2 = local_30;
      param_2[1] = local_28;
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = dcgettext(0,"Unexpected EOF in snapshot file",5);
  uVar3 = FUN_0014aa70(DAT_00184b28);
  error(0,0,"%s: %s",uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: read_incr_db_2 @ 0x1ee50

void read_incr_db_2(void)

{
  long lVar1;
  long lVar2;
  ulong uVar3;
  char cVar4;
  int iVar5;
  ulong uVar6;
  ulong uVar7;
  undefined8 uVar8;
  __off_t _Var9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  long in_FS_OFFSET;
  bool bVar13;
  long local_d8;
  ulong local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined1 local_b8 [8];
  ulong local_b0;
  ulong local_a8;
  ulong local_a0;
  ulong local_98;
  ulong local_88;
  byte local_68;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  _obstack_begin(local_b8,0,0,FUN_0014c5b0,PTR_free_00180fb8);
  FUN_0011ed70(DAT_001843a0,&DAT_00184b00);
  uVar6 = local_a8;
  uVar3 = local_a0;
  do {
    local_a0 = uVar3;
    local_a8 = uVar6;
    cVar4 = FUN_0011ea10(DAT_001843a0,&DAT_00161a6a,0,1,&local_d8);
    if (cVar4 == '\0') {
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    bVar13 = local_d8 != 0;
    FUN_0011ed70(DAT_001843a0,&local_c8);
    cVar4 = FUN_0011ea10(DAT_001843a0,&DAT_00167193,0,0xffffffffffffffff,&local_d8);
    lVar1 = local_d8;
    if (((cVar4 == '\0') ||
        (cVar4 = FUN_0011ea10(DAT_001843a0,&DAT_00161a6e,0,0xffffffffffffffff,&local_d8),
        lVar2 = local_d8, cVar4 == '\0')) ||
       (iVar5 = FUN_0011e960(DAT_001843a0,local_b8,&local_d0), uVar3 = local_a8, iVar5 != 0)) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar8 = dcgettext(0,"Unexpected EOF in snapshot file",5);
      uVar10 = FUN_0014aa70(DAT_00184b28);
      error(0,0,"%s: %s",uVar10,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    if (local_a0 == local_a8) {
      local_68 = local_68 | 2;
    }
    uVar6 = local_a0 + local_88 & ~local_88;
    local_a8 = local_98;
    local_a0 = local_98;
    if (uVar6 - local_b0 <= local_98 - local_b0) {
      local_a8 = uVar6;
      local_a0 = uVar6;
    }
    do {
      iVar5 = FUN_0011e960(DAT_001843a0,local_b8,&local_d0);
      if (iVar5 != 0) break;
    } while (1 < local_d0);
    iVar5 = getc_unlocked(DAT_001843a0);
    uVar6 = local_a8;
    if (iVar5 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar8 = dcgettext(0,"Missing record terminator",5);
      _Var9 = ftello(DAT_001843a0);
      uVar10 = FUN_00142ff0(_Var9,local_58);
      uVar11 = FUN_0014aa70(DAT_00184b28);
      uVar12 = dcgettext(0,"%s: byte %s: %s",5);
      error(0,0,uVar12,uVar11,uVar10,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    if (local_a0 == local_a8) {
      local_68 = local_68 | 2;
    }
    uVar7 = local_a0 + local_88 & ~local_88;
    local_a8 = local_98;
    if (uVar7 - local_b0 <= local_98 - local_b0) {
      local_a8 = uVar7;
    }
    local_a0 = local_a8;
    FUN_0011dac0(uVar3,local_c8,local_c0,lVar1,lVar2,bVar13,0,uVar6);
    if ((uVar6 <= local_b0) || (uVar3 = uVar6, local_98 <= uVar6)) {
      _obstack_free(local_b8,uVar6);
      uVar6 = local_a8;
      uVar3 = local_a0;
    }
  } while( true );
}




// Function: write_directory_file_entry @ 0x1f1c0

undefined8 write_directory_file_entry(long param_1,FILE *param_2)

{
  void *__ptr;
  int iVar1;
  size_t sVar2;
  undefined4 extraout_var;
  char *pcVar3;
  long in_FS_OFFSET;
  void *local_50;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(uint *)(param_1 + 0x3c) & 4) != 0) {
    pcVar3 = "1";
    if ((*(uint *)(param_1 + 0x3c) & 2) == 0) {
      pcVar3 = "0";
    }
    fwrite_unlocked(pcVar3,2,1,param_2);
    pcVar3 = (char *)FUN_00124080(*(undefined8 *)(param_1 + 8),0x8000000000000000,0x7fffffffffffffff
                                  ,local_48);
    sVar2 = strlen(pcVar3);
    fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
    pcVar3 = (char *)FUN_00142f50(*(undefined8 *)(param_1 + 0x10),local_48);
    sVar2 = strlen(pcVar3);
    fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
    pcVar3 = (char *)FUN_00124080(*(undefined8 *)(param_1 + 0x18),0,0xffffffffffffffff,local_48);
    sVar2 = strlen(pcVar3);
    fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
    pcVar3 = (char *)FUN_00124080(*(undefined8 *)(param_1 + 0x20),0,0xffffffffffffffff,local_48);
    sVar2 = strlen(pcVar3);
    fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
    pcVar3 = *(char **)(param_1 + 0x58);
    sVar2 = strlen(pcVar3);
    fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
    if (*(long *)(param_1 + 0x28) != 0) {
      pcVar3 = (char *)FUN_0011d220(*(long *)(param_1 + 0x28),0,&local_50);
      __ptr = local_50;
      while (pcVar3 != (char *)0x0) {
        sVar2 = strlen(pcVar3);
        fwrite_unlocked(pcVar3,sVar2 + 1,1,param_2);
        __ptr = local_50;
        pcVar3 = (char *)FUN_0011d110(local_50);
      }
      free(__ptr);
    }
    fwrite_unlocked(&DAT_00161eec,2,1,param_2);
  }
  iVar1 = ferror_unlocked(param_2);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_gnu_dumpdir @ 0x1f3c0

void get_gnu_dumpdir(long param_1)

{
  void *pvVar1;
  ulong __n;
  void *__src;
  undefined8 uVar2;
  ulong uVar3;
  void *__dest;
  
  uVar3 = *(ulong *)(param_1 + 0x88);
  pvVar1 = (void *)FUN_0014c5b0(uVar3);
  FUN_0010d1f0(DAT_00184408);
  FUN_0010ccb0(param_1);
  __dest = pvVar1;
  for (; uVar3 != 0; uVar3 = uVar3 - __n) {
    FUN_0010ccf0(uVar3);
    __src = (void *)FUN_0010e130();
    if (__src == (void *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,"Unexpected EOF in archive",5);
      error(1,0,uVar2);
      DAT_00184d58 = 2;
    }
    __n = FUN_0010d2b0(__src);
    if (uVar3 < __n) {
      __n = uVar3;
    }
    memcpy(__dest,__src,__n);
    __dest = (void *)((long)__dest + __n);
    FUN_0010d1f0((long)__src + (__n - 1));
  }
  FUN_0010ccd0();
  *(undefined1 *)(param_1 + 0x189) = 1;
  *(void **)(param_1 + 400) = pvVar1;
  return;
}




// Function: dumpdir_size @ 0x1f4c0

long dumpdir_size(char *param_1)

{
  size_t sVar1;
  long lVar2;
  
  if (*param_1 != '\0') {
    lVar2 = 0;
    do {
      sVar1 = strlen(param_1);
      param_1 = param_1 + sVar1 + 1;
      lVar2 = lVar2 + sVar1 + 1;
    } while (*param_1 != '\0');
    return lVar2 + 1;
  }
  return 1;
}




// Function: clear_directory_table @ 0x1f510

void clear_directory_table(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  
  if (DAT_001843b0 != 0) {
    FUN_00141c30();
  }
  puVar2 = DAT_001843c0;
  if (DAT_001843a8 != 0) {
    FUN_00141c30();
    puVar2 = DAT_001843c0;
  }
  while (puVar2 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*puVar2;
    FUN_0011d180(puVar2);
    puVar2 = puVar1;
  }
  DAT_001843b8 = 0;
  DAT_001843c0 = (undefined8 *)0x0;
  return;
}




// Function: rebase_directory @ 0x1f570

void rebase_directory(long param_1)

{
  FUN_00123fc0(param_1 + 0x58);
  return;
}




// Function: update_parent_directory @ 0x1f580

void update_parent_directory(undefined8 *param_1)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined1 auVar3 [16];
  stat sStack_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_0011d310(*param_1);
  if (lVar2 != 0) {
    iVar1 = fstat(*(int *)(param_1 + 0x35),&sStack_b8);
    if (iVar1 == 0) {
      auVar3 = FUN_0014bd20(&sStack_b8);
      *(undefined1 (*) [16])(lVar2 + 8) = auVar3;
    }
    else {
      FUN_00124ec0(*(undefined8 *)(lVar2 + 0x58));
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: scan_directory @ 0x1f610

long scan_directory(undefined8 *param_1)

{
  char *__file;
  __dev_t _Var1;
  char cVar2;
  int iVar3;
  void *__ptr;
  void *pvVar4;
  long lVar5;
  undefined8 uVar6;
  char *pcVar7;
  undefined8 uVar8;
  int *piVar9;
  code *pcVar10;
  undefined8 uVar11;
  long in_FS_OFFSET;
  undefined1 auVar12 [16];
  undefined1 local_201;
  void *local_200;
  undefined1 local_1f8 [88];
  stat local_1a0 [2];
  undefined8 *local_60;
  int local_50;
  long local_40;
  
  uVar6 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = (void *)FUN_00113fe0();
  lVar5 = param_1[0x33];
  _Var1 = param_1[0xb];
  if (__ptr == (void *)0x0) {
    FUN_00132ac0(uVar6);
    FUN_00116890(param_1);
    pvVar4 = (void *)FUN_0014c7a0(uVar6);
    FUN_00123e60(pvVar4);
    uVar8 = 0;
    if (lVar5 == 0) {
      uVar8 = 0x20;
    }
    lVar5 = FUN_0011dbe0(pvVar4,param_1,uVar8,&local_201);
    free(pvVar4);
    uVar6 = FUN_00125040(uVar6);
  }
  else {
    FUN_00116890(param_1);
    pvVar4 = (void *)FUN_0014c7a0(uVar6);
    FUN_00123e60(pvVar4);
    uVar8 = 0;
    if (lVar5 == 0) {
      uVar8 = 0x20;
    }
    lVar5 = FUN_0011dbe0(pvVar4,param_1,uVar8,&local_201);
    free(pvVar4);
    uVar6 = FUN_00125040(uVar6);
    if (*(int *)(lVar5 + 0x38) == 0) {
      if (*(long *)(lVar5 + 0x48) != 0) {
        FUN_0011d800(lVar5);
      }
    }
    else {
      FUN_0011d600(lVar5,__ptr);
      pcVar7 = (char *)FUN_0011d220(*(undefined8 *)(lVar5 + 0x28),1,&local_200);
      pvVar4 = local_200;
      if (pcVar7 != (char *)0x0) {
        do {
          __file = pcVar7 + 1;
          uVar8 = FUN_001250d0(uVar6,__file);
          if ((*pcVar7 == 'I') || (cVar2 = FUN_00116b00(uVar8,param_1), cVar2 != '\0')) {
            *pcVar7 = 'N';
          }
          else {
            iVar3 = *(int *)(param_1 + 0x35);
            FUN_0012f630(local_1f8);
            if (iVar3 < 0) {
              piVar9 = __errno_location();
              pcVar10 = FUN_00124dd0;
              *piVar9 = -iVar3;
LAB_0011f7d3:
              FUN_00124ef0(uVar8,0,pcVar10);
              *pcVar7 = 'N';
            }
            else {
              iVar3 = fstatat(iVar3,__file,local_1a0,DAT_00184800);
              if (iVar3 != 0) {
LAB_0011f84a:
                pcVar10 = FUN_00124ec0;
                goto LAB_0011f7d3;
              }
              if ((local_1a0[0].st_mode & 0xf000) == 0x4000) {
                iVar3 = FUN_001140d0(param_1,__file,DAT_00184808);
                if (iVar3 < 0) {
                  pcVar10 = FUN_00124dd0;
                  goto LAB_0011f7d3;
                }
                local_50 = iVar3;
                iVar3 = fstat(iVar3,local_1a0);
                if (iVar3 == 0) {
                  if ((local_1a0[0].st_mode & 0xf000) != 0x4000) goto LAB_0011f8cb;
                  if (DAT_00184ad4 == 0) {
                    uVar11 = 0x10;
                  }
                  else {
                    uVar11 = 0;
                    if (*(int *)(lVar5 + 0x38) == 2) {
                      uVar11 = 0x12;
                    }
                  }
                  *pcVar7 = 'D';
                  local_60 = param_1;
                  FUN_0011dbe0(uVar8,local_1f8,uVar11,pcVar7);
                  FUN_00114200(local_1f8);
                  goto LAB_0011f7e0;
                }
                goto LAB_0011f84a;
              }
LAB_0011f8cb:
              if ((DAT_00184ad1 != '\0') && (local_1a0[0].st_dev != _Var1)) goto LAB_0011f8e3;
              if (*pcVar7 != 'Y') {
                auVar12 = FUN_0014bd20(local_1a0);
                uVar11 = DAT_00184b08;
                uVar8 = DAT_00184b00;
                iVar3 = FUN_0014be80(auVar12._0_8_,auVar12._8_8_,DAT_00184b00,DAT_00184b08);
                if (iVar3 < 0) {
                  if (DAT_00184b8c != 0) {
                    auVar12 = FUN_0014bd10(local_1a0);
                    iVar3 = FUN_0014be80(auVar12._0_8_,auVar12._8_8_,uVar8,uVar11);
                    if (-1 < iVar3) goto LAB_0011f972;
                  }
LAB_0011f8e3:
                  *pcVar7 = 'N';
                }
                else {
LAB_0011f972:
                  *pcVar7 = 'Y';
                }
              }
            }
LAB_0011f7e0:
            FUN_0012f6f0(local_1f8);
          }
          pvVar4 = local_200;
          pcVar7 = (char *)FUN_0011d110(local_200);
        } while (pcVar7 != (char *)0x0);
      }
      free(pvVar4);
    }
  }
  FUN_001250b0(uVar6);
  free(__ptr);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar5;
}




// Function: directory_contents @ 0x1fa60

undefined8 directory_contents(long param_1)

{
  if ((param_1 != 0) && (*(undefined8 **)(param_1 + 0x28) != (undefined8 *)0x0)) {
    return **(undefined8 **)(param_1 + 0x28);
  }
  return 0;
}




// Function: safe_directory_contents @ 0x1fa90

undefined * safe_directory_contents(void)

{
  undefined *puVar1;
  
  puVar1 = (undefined *)FUN_0011fa60();
  if (puVar1 == (undefined *)0x0) {
    puVar1 = &DAT_00161eef;
  }
  return puVar1;
}




// Function: append_incremental_renames @ 0x1fab0

void append_incremental_renames(long param_1)

{
  void *__src;
  long lVar1;
  undefined8 uVar2;
  long *plVar3;
  ulong __n;
  long in_FS_OFFSET;
  undefined1 auStack_88 [8];
  long local_80;
  undefined1 *local_78;
  undefined1 *local_70;
  undefined1 *local_68;
  ulong local_58;
  byte local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001843c0 != (long *)0x0) {
    __n = 0;
    _obstack_begin(auStack_88,0,0,FUN_0014c5b0,PTR_free_00180fb8);
    __src = (void *)FUN_0011fa60(param_1);
    plVar3 = DAT_001843c0;
    if (__src != (void *)0x0) {
      lVar1 = FUN_0011f4c0(__src);
      __n = lVar1 - 1;
      if ((ulong)((long)local_68 - (long)local_70) < __n) {
        _obstack_newchunk(auStack_88,__n);
      }
      memcpy(local_70,__src,__n);
      local_70 = local_70 + __n;
      plVar3 = DAT_001843c0;
    }
    for (; plVar3 != (long *)0x0; plVar3 = (long *)*plVar3) {
      while ((*(byte *)((long)plVar3 + 0x3c) & 0x10) == 0) {
        plVar3 = (long *)*plVar3;
        if (plVar3 == (long *)0x0) goto LAB_0011fb79;
      }
      FUN_0011e320(plVar3,auStack_88);
    }
LAB_0011fb79:
    if ((param_1 != 0) && (__n != (long)local_70 - (long)local_78)) {
      if (local_70 == local_68) {
        _obstack_newchunk(auStack_88,1);
      }
      *local_70 = 0;
      local_70 = local_70 + 1;
      FUN_0011d1a0(*(undefined8 *)(param_1 + 0x28));
      if (local_70 == local_78) {
        local_38 = local_38 | 2;
      }
      local_78 = local_68;
      if ((ulong)((long)((ulong)(local_70 + local_58) & ~local_58) - local_80) <=
          (ulong)((long)local_68 - local_80)) {
        local_78 = (undefined1 *)((ulong)(local_70 + local_58) & ~local_58);
      }
      local_70 = local_78;
      uVar2 = FUN_0011d7f0();
      *(undefined8 *)(param_1 + 0x28) = uVar2;
    }
    _obstack_free(auStack_88,0);
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: show_snapshot_field_ranges @ 0x1fc70

void show_snapshot_field_ranges(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined **ppuVar3;
  undefined *puVar4;
  undefined *puVar5;
  long in_FS_OFFSET;
  undefined1 auStack_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __printf_chk(1,"This tar\'s snapshot file field ranges are\n");
  __printf_chk(1,"   (%-15s => [ %s, %s ]):\n\n","field name",&DAT_00161aa0,&DAT_00161a9c);
  puVar4 = (undefined *)0x0;
  puVar5 = (undefined *)0x1;
  ppuVar3 = &PTR_DAT_0017f600;
  while( true ) {
    uVar1 = FUN_00124080(puVar5,puVar4,puVar5,local_48);
    uVar2 = FUN_00124080(ppuVar3[1],ppuVar3[1],ppuVar3[2],auStack_68);
    __printf_chk(1,"    %-15s => [ %s, %s ],\n",*ppuVar3,uVar2,uVar1);
    if (ppuVar3[3] == (undefined *)0x0) break;
    puVar5 = ppuVar3[5];
    puVar4 = ppuVar3[4];
    ppuVar3 = ppuVar3 + 3;
  }
  __printf_chk(1,&DAT_00167707);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_directory_file @ 0x1fd80

void read_directory_file(void)

{
  char cVar1;
  int iVar2;
  __ssize_t _Var3;
  undefined8 uVar4;
  uintmax_t uVar5;
  char *pcVar6;
  char *pcVar7;
  ulong uVar8;
  long in_FS_OFFSET;
  char *local_30;
  size_t local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = (char *)0x0;
  local_28 = 0;
  iVar2 = open(DAT_00184b28,(-(uint)(DAT_00184b24 == 0) & 0x200) + 0x42,0x1b6);
  if (iVar2 < 0) {
    FUN_001327c0(DAT_00184b28);
    goto LAB_0011ff2a;
  }
  DAT_001843a0 = fdopen(iVar2,"r+");
  if (DAT_001843a0 == (FILE *)0x0) {
    FUN_001327c0(DAT_00184b28);
    close(iVar2);
    goto LAB_0011ff2a;
  }
  FUN_00127b00();
  FUN_00127b80();
  _Var3 = getline(&local_30,&local_28,DAT_001843a0);
  pcVar7 = local_30;
  if (_Var3 < 1) {
LAB_0011ff10:
    iVar2 = ferror_unlocked(DAT_001843a0);
  }
  else {
    iVar2 = strncmp(local_30,"GNU tar",7);
    if (iVar2 == 0) {
      pcVar6 = pcVar7 + 8;
      if (pcVar7[7] != '-') {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = dcgettext(0,"Bad incremental file format",5);
        error(1,0,uVar4);
        DAT_00184d58 = 2;
      }
      cVar1 = pcVar7[8];
      while (cVar1 != '-') {
        if (cVar1 == '\0') {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"Bad incremental file format",5);
          error(1,0,uVar4);
          DAT_00184d58 = 2;
        }
        pcVar7 = pcVar6 + 1;
        pcVar6 = pcVar6 + 1;
        cVar1 = *pcVar7;
      }
      uVar5 = strtoumax(pcVar6 + 1,(char **)0x0,10);
      if (1 < uVar5) {
        if (uVar5 == 2) {
          FUN_0011ee50();
        }
        else {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"Unsupported incremental format version: %lu",5);
          error(1,0,uVar4,uVar5);
          DAT_00184d58 = 2;
        }
        goto LAB_0011ff10;
      }
      uVar8 = uVar5 & 0xffffffff;
      pcVar7 = local_30;
    }
    else {
      uVar8 = 0;
    }
    FUN_0011e470(uVar8,pcVar7);
    iVar2 = ferror_unlocked(DAT_001843a0);
  }
  if (iVar2 != 0) {
    FUN_00132820(DAT_00184b28);
  }
  free(local_30);
LAB_0011ff2a:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_directory_file @ 0x20010

void write_directory_file(void)

{
  FILE *__stream;
  int iVar1;
  char *pcVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  __stream = DAT_001843a0;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001843a0 != (FILE *)0x0) {
    iVar1 = fseeko(DAT_001843a0,0,0);
    if (iVar1 != 0) {
      FUN_00132b00(DAT_00184b28);
    }
    iVar1 = fileno(__stream);
    iVar1 = FUN_0012aed0(iVar1);
    if (iVar1 != 0) {
      FUN_00132d70(DAT_00184b28);
    }
    __fprintf_chk(__stream,1,"%s-%s-%d\n","GNU tar",&DAT_00161b0c,2);
    pcVar2 = (char *)FUN_00142f50(DAT_00184a20,auStack_38);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    pcVar2 = (char *)FUN_00143090(DAT_00184a28,auStack_38);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    iVar1 = ferror_unlocked(__stream);
    if ((iVar1 == 0) && (DAT_001843b0 != 0)) {
      FUN_00141a40(DAT_001843b0,FUN_0011f1c0,__stream);
    }
    iVar1 = ferror_unlocked(__stream);
    if (iVar1 == 0) {
      iVar1 = fclose(__stream);
    }
    else {
      FUN_00132e10(DAT_00184b28);
      iVar1 = fclose(__stream);
    }
    if (iVar1 != 0) {
      FUN_00132680(DAT_00184b28);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: is_dumpdir @ 0x201b0

char is_dumpdir(long param_1)

{
  if ((*(char *)(param_1 + 0x188) != '\0') && (*(long *)(param_1 + 400) == 0)) {
    FUN_0011f3c0();
    return *(char *)(param_1 + 0x188);
  }
  return *(char *)(param_1 + 0x188);
}




// Function: try_purge_directory @ 0x201f0

uint try_purge_directory(undefined8 param_1)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  char *__ptr;
  size_t sVar4;
  char *pcVar5;
  char *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  void *__ptr_00;
  char *pcVar9;
  undefined8 uVar10;
  int *piVar11;
  undefined8 uVar12;
  char *pcVar13;
  long in_FS_OFFSET;
  long local_d8 [3];
  uint local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001201b0(&DAT_00184840);
  if (((cVar1 == '\0') || (__ptr = (char *)FUN_001253c0(param_1,0), __ptr == (char *)0x0)) ||
     (uVar2 = FUN_0011d860(DAT_001849d0), (char)uVar2 == '\0')) {
LAB_00120227:
    uVar2 = 0;
  }
  else {
    pcVar9 = (char *)0x0;
    cVar1 = *DAT_001849d0;
    pcVar5 = DAT_001849d0;
    while (cVar1 != '\0') {
      pcVar13 = pcVar5;
      if (cVar1 == 'X') {
        sVar4 = strlen(pcVar5 + 1);
        pcVar9 = (char *)FUN_0014c610(pcVar9,sVar4 + 0xc);
        memcpy(pcVar9,pcVar5 + 1,sVar4);
        builtin_strncpy(pcVar9 + sVar4,"/tar.XXXXXX",0xc);
        pcVar5 = mkdtemp(pcVar9);
        if (pcVar5 == (char *)0x0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar7 = FUN_0014ac50(pcVar9);
          uVar10 = dcgettext(0,"Cannot create temporary directory using template %s",5);
          piVar11 = __errno_location();
          error(0,*piVar11,uVar10,uVar7);
          DAT_00184d58 = 2;
LAB_001203d9:
          free(pcVar9);
          free(__ptr);
          goto LAB_00120227;
        }
      }
      else {
        sVar4 = strlen(pcVar5);
        if (cVar1 == 'R') {
          pcVar13 = pcVar5 + sVar4 + 1;
          pcVar6 = pcVar5 + 1;
          if (pcVar5[1] != '\0') {
            pcVar6 = (char *)FUN_00133020(pcVar6,0,DAT_00184b92);
          }
          if (pcVar13[1] != '\0') {
            FUN_00133020(pcVar13 + 1,0,DAT_00184b92);
          }
          pcVar5 = pcVar9;
          if (*pcVar6 != '\0') {
            pcVar5 = pcVar6;
          }
          cVar1 = FUN_00119ec0(pcVar5);
          if (cVar1 == '\0') goto LAB_001203d9;
        }
      }
      sVar4 = strlen(pcVar13);
      pcVar5 = pcVar13 + sVar4 + 1;
      cVar1 = *pcVar5;
    }
    __ptr_00 = (void *)0x0;
    free(pcVar9);
    uVar7 = FUN_0011d7f0(DAT_001849d0);
    if (*__ptr != '\0') {
      pcVar5 = __ptr;
      do {
        free(__ptr_00);
        __ptr_00 = (void *)FUN_00127bc0(param_1,pcVar5);
        iVar3 = FUN_001246f0(__ptr_00,local_d8);
        if (iVar3 == 0) {
          pcVar9 = (char *)FUN_0011d450(uVar7,pcVar5);
          if (pcVar9 == (char *)0x0) {
LAB_00120540:
            if ((DAT_00184ad1 != '\0') && (local_d8[0] != DAT_001847f0)) {
              if (DAT_00184d50 != (code *)0x0) {
                (*DAT_00184d50)();
              }
              uVar10 = FUN_0014aa70(__ptr_00);
              pcVar9 = "%s: directory is on a different device: not purging";
              goto LAB_001204b7;
            }
            if ((DAT_00184b40 == '\0') || (iVar3 = FUN_0012ca20("delete",__ptr_00), iVar3 != 0)) {
              if (DAT_00184a54 != 0) {
                uVar8 = FUN_0014ac50(__ptr_00);
                uVar10 = DAT_00184dd0;
                uVar12 = dcgettext(0,"%s: Deleting %s\n",5);
                __fprintf_chk(DAT_00183f28,1,uVar12,uVar10,uVar8);
              }
              iVar3 = FUN_00125490(__ptr_00,1);
              if (iVar3 == 0) {
                piVar11 = __errno_location();
                iVar3 = *piVar11;
                if (DAT_00184d50 != (code *)0x0) {
                  (*DAT_00184d50)();
                }
                uVar10 = FUN_0014aa70(__ptr_00);
                uVar8 = dcgettext(0,"%s: Cannot remove",5);
                error(0,iVar3,uVar8,uVar10);
                DAT_00184d58 = 2;
              }
            }
          }
          else if (*pcVar9 == 'D') {
            if ((local_c0 & 0xf000) != 0x4000) goto LAB_00120540;
          }
          else if ((*pcVar9 == 'Y') && ((local_c0 & 0xf000) == 0x4000)) goto LAB_00120540;
        }
        else {
          piVar11 = __errno_location();
          if (*piVar11 == 2) goto LAB_001204cf;
          FUN_00124ec0(__ptr_00);
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar10 = FUN_0014aa70(__ptr_00);
          pcVar9 = "%s: Not purging directory: unable to stat";
LAB_001204b7:
          uVar8 = dcgettext(0,pcVar9,5);
          error(0,0,uVar8,uVar10);
        }
LAB_001204cf:
        sVar4 = strlen(pcVar5);
        pcVar5 = pcVar5 + sVar4 + 1;
      } while (*pcVar5 != '\0');
      uVar2 = uVar2 & 0xff;
    }
    free(__ptr_00);
    FUN_0011d1a0(uVar7);
    free(__ptr);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: purge_directory @ 0x206e0

void purge_directory(void)

{
  char cVar1;
  
  cVar1 = FUN_001201f0();
  if (cVar1 != '\0') {
    return;
  }
  FUN_00122bd0();
  return;
}




// Function: list_dumpdir @ 0x20710

void list_dumpdir(char *param_1,long param_2)

{
  char *pcVar1;
  char cVar2;
  bool bVar3;
  uint uVar4;
  
  pcVar1 = param_1 + param_2;
  if (param_2 != 0) {
    bVar3 = false;
    do {
      while( true ) {
        cVar2 = *param_1;
        param_1 = param_1 + 1;
        if (cVar2 != '\0') break;
        bVar3 = false;
        fputc_unlocked(10,DAT_00183f28);
        if (param_1 == pcVar1) {
          return;
        }
      }
      uVar4 = (int)cVar2 - 0x44;
      if (((byte)uVar4 < 0x16) && ((0x314401UL >> ((ulong)uVar4 & 0x3f) & 1) != 0)) {
        __fprintf_chk(DAT_00183f28,1,"%c");
        if (!bVar3) {
          __fprintf_chk(DAT_00183f28,1,&DAT_00167a2f);
        }
        bVar3 = true;
      }
      else {
        fputc_unlocked((int)cVar2,DAT_00183f28);
      }
    } while (param_1 != pcVar1);
  }
  return;
}




// Function: decode_xform @ 0x207f0

char * decode_xform(char *param_1,int *param_2)

{
  int iVar1;
  size_t sVar2;
  
  iVar1 = *param_2;
  if (iVar1 == 2) {
    param_1 = (char *)FUN_00133020(param_1,1,DAT_00184b92);
  }
  else {
    if (iVar1 == 4) {
      return param_1;
    }
    if (iVar1 == 1) {
      param_1 = (char *)FUN_00133020(param_1,0,DAT_00184b92);
    }
  }
  if (DAT_00184a90 == 0) {
    return param_1;
  }
  sVar2 = FUN_00127c50(param_1);
  if (sVar2 == 0xffffffffffffffff) {
    sVar2 = strlen(param_1);
  }
  return param_1 + sVar2;
}




// Function: from_header @ 0x20890

undefined8
from_header(ushort **param_1,size_t param_2,long param_3,long param_4,ushort **param_5,
            undefined4 param_6,undefined4 param_7)

{
  byte bVar1;
  byte bVar2;
  ushort **ppuVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  ulong uVar8;
  ushort **ppuVar9;
  uint uVar10;
  ushort **ppuVar11;
  undefined1 *puVar12;
  undefined1 *puVar13;
  undefined1 *puVar14;
  char *pcVar16;
  ushort **ppuVar17;
  ushort **ppuVar18;
  ushort **ppuVar19;
  void *__dest;
  ushort **ppuVar20;
  long in_FS_OFFSET;
  bool bVar21;
  bool bVar22;
  undefined1 auStack_498 [8];
  ushort **local_490;
  ushort **local_488;
  ushort **local_480;
  undefined4 local_474;
  ushort **local_470;
  undefined1 local_468 [33];
  undefined1 local_447 [31];
  undefined1 local_428;
  undefined1 local_427 [999];
  long local_40;
  undefined1 *puVar15;
  
  ppuVar11 = (ushort **)((long)param_1 + param_2);
  puVar15 = auStack_498;
  puVar14 = auStack_498;
  puVar12 = auStack_498;
  puVar13 = auStack_498;
  local_488 = (ushort **)CONCAT44(local_488._4_4_,param_6);
  local_474 = param_7;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  ppuVar9 = (ushort **)((long)param_1 + (ulong)(*(byte *)param_1 == 0));
  local_480 = param_5;
  if (ppuVar11 != ppuVar9) {
    local_470 = ppuVar9;
    ppuVar3 = __ctype_b_loc();
    ppuVar9 = local_470;
    do {
      bVar2 = *(byte *)ppuVar9;
      if ((*(byte *)((long)*ppuVar3 + (ulong)bVar2 * 2 + 1) & 0x20) == 0) {
        local_470 = (ushort **)-param_4;
        if ((int)(char)bVar2 - 0x30U < 8) {
          ppuVar17 = (ushort **)((long)ppuVar9 + 1);
          ppuVar19 = (ushort **)(long)(int)((int)(char)bVar2 - 0x30U);
          if (ppuVar11 != ppuVar17) {
            bVar22 = false;
            ppuVar20 = ppuVar17;
            goto LAB_00120994;
          }
          bVar22 = false;
          ppuVar20 = ppuVar11;
          goto LAB_00120cc0;
        }
        if ((char)local_488 == '\0') {
          if ((bVar2 - 0x2b & 0xfd) == 0) {
            if (((char)local_474 == '\0') && (DAT_00184450 == '\0')) {
              DAT_00184450 = '\x01';
              local_490 = ppuVar9;
              local_488 = ppuVar3;
              if (DAT_00184d50 != (code *)0x0) {
                (*DAT_00184d50)();
              }
              uVar4 = dcgettext(0,"Archive contains obsolescent base-64 headers",5);
              error(0,0,uVar4);
              bVar2 = *(byte *)local_490;
              ppuVar9 = local_490;
              ppuVar3 = local_488;
            }
            ppuVar9 = (ushort **)((long)ppuVar9 + 1);
            bVar22 = bVar2 == 0x2d;
            ppuVar19 = (ushort **)0x0;
            if (ppuVar11 == ppuVar9) goto LAB_00120ca8;
            goto LAB_00120baf;
          }
          if ((bVar2 == 0x80) || (bVar2 == 0xff)) {
            ppuVar9 = (ushort **)((long)ppuVar9 + 1);
            ppuVar19 = (ushort **)(long)(int)((bVar2 & 0x3f) - (bVar2 & 0x40));
            goto LAB_00120eb2;
          }
        }
        if (bVar2 != 0) goto LAB_00120a91;
        ppuVar19 = (ushort **)0x0;
        goto LAB_00120ca8;
      }
      ppuVar9 = (ushort **)((long)ppuVar9 + 1);
    } while (ppuVar11 != ppuVar9);
  }
  puVar12 = auStack_498;
  if ((param_3 != 0) && (puVar12 = auStack_498, (char)local_474 != '\x01')) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar16 = "Blanks in header where numeric %s value expected";
    goto LAB_00120a58;
  }
LAB_00120a00:
  uVar4 = 0xffffffffffffffff;
  puVar13 = puVar12;
LAB_00120a07:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined **)(puVar13 + -8) = &UNK_00121159;
  __stack_chk_fail();
  while( true ) {
    bVar21 = (ushort **)((ulong)ppuVar19 & 0x1fffffffffffffff) != ppuVar19;
    ppuVar19 = (ushort **)((long)(int)uVar10 + (long)ppuVar19 * 8);
    ppuVar20 = (ushort **)((long)ppuVar20 + 1);
    bVar22 = (bool)(bVar22 | bVar21);
    if (ppuVar11 == ppuVar20) break;
LAB_00120994:
    bVar1 = *(byte *)ppuVar20;
    uVar8 = (ulong)(uint)(int)(char)bVar1;
    uVar10 = (int)(char)bVar1 - 0x30;
    if (7 < uVar10) {
      if ((local_480 < ppuVar19) || (bVar22)) goto LAB_00120cce;
      if ((ppuVar11 == ppuVar20) || (bVar1 == 0)) goto LAB_00120ca8;
      goto LAB_001209ca;
    }
  }
LAB_00120cc0:
  if ((ppuVar19 <= local_480) && (!bVar22)) goto LAB_00120ca8;
LAB_00120cce:
  if ((char)bVar2 < '2') {
LAB_00120e3b:
    if (bVar22) {
LAB_001210d1:
      puVar12 = auStack_498;
      if ((param_3 != 0) && (puVar12 = auStack_498, (char)local_474 != '\x01')) {
        local_470 = ppuVar9;
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = dcgettext(0,"Archive octal value %.*s is out of %s range",5);
        error(0,0,uVar4,(int)ppuVar20 - (int)local_470,local_470,param_3);
        uVar4 = 0xffffffffffffffff;
        DAT_00184d58 = 2;
        goto LAB_00120a07;
      }
      goto LAB_00120a00;
    }
LAB_00120e44:
    if ((ppuVar11 != ppuVar20) && (uVar8 = (ulong)*(byte *)ppuVar20, *(byte *)ppuVar20 != 0)) {
LAB_001209ca:
      if ((*(byte *)((long)*ppuVar3 + (uVar8 & 0xff) * 2 + 1) & 0x20) == 0) {
LAB_00120a91:
        puVar12 = auStack_498;
        if (param_3 != 0) {
LAB_00120a9a:
          if (DAT_00184448 == 0) {
            DAT_00184448 = FUN_0014a530(0);
            FUN_0014a590(DAT_00184448,8);
          }
          for (; (param_1 != ppuVar11 && (*(byte *)((long)ppuVar11 + -1) == 0));
              ppuVar11 = (ushort **)((long)ppuVar11 + -1)) {
          }
          FUN_0014a650(&local_428,1000,param_1,(long)ppuVar11 - (long)param_1,DAT_00184448);
          puVar12 = auStack_498;
          if ((char)local_474 == '\0') {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            uVar4 = dcgettext(0,"Archive contains %.*s where numeric %s value expected",5);
            error(0,0,uVar4,1000,&local_428,param_3);
            DAT_00184d58 = 2;
            puVar12 = auStack_498;
          }
        }
        goto LAB_00120a00;
      }
    }
LAB_001209e0:
    if (local_480 < ppuVar19) {
      puVar12 = auStack_498;
      if ((param_3 != 0) && (puVar12 = auStack_498, (char)local_474 != '\x01')) {
        lVar6 = FUN_00143090(local_470,local_447);
        lVar7 = FUN_00143090(ppuVar19,local_427);
LAB_00120c2e:
        if (local_470 != (ushort **)0x0) {
          *(undefined1 *)(lVar6 + -1) = 0x2d;
          lVar6 = lVar6 + -1;
        }
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = FUN_00143090(local_480,local_468);
        uVar5 = dcgettext(0,"Archive value %s is out of %s range %s..%s",5);
        error(0,0,uVar5,lVar7,param_3,lVar6,uVar4);
        DAT_00184d58 = 2;
        puVar12 = auStack_498;
      }
      goto LAB_00120a00;
    }
  }
  else {
    if (param_3 == 0) {
      if (bVar22) goto LAB_00120a00;
      goto LAB_00120e44;
    }
    ppuVar18 = (ushort **)(long)(7 - (char)(bVar2 - 0x30 | 4));
    if (ppuVar11 == ppuVar17) {
      bVar2 = 0;
      ppuVar20 = ppuVar11;
    }
    else {
      bVar2 = 0;
      do {
        bVar1 = *(byte *)ppuVar17;
        ppuVar20 = ppuVar17;
        if (7 < (int)(char)bVar1 - 0x30U) break;
        ppuVar17 = (ushort **)((long)ppuVar17 + 1);
        bVar2 = bVar2 | (ushort **)((ulong)ppuVar18 & 0x1fffffffffffffff) != ppuVar18;
        ppuVar18 = (ushort **)((long)(int)(7 - ((int)(char)bVar1 - 0x30U)) + (long)ppuVar18 * 8);
        ppuVar20 = ppuVar11;
      } while (ppuVar11 != ppuVar17);
    }
    ppuVar19 = (ushort **)((long)ppuVar18 + 1);
    if ((bool)(ppuVar19 == (ushort **)0x0 | bVar2)) goto LAB_001210d1;
    bVar22 = false;
    if (local_470 < ppuVar19) goto LAB_00120e3b;
    if ((char)local_474 == '\0') {
      local_488 = ppuVar9;
      local_480 = ppuVar18;
      local_470 = ppuVar3;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar4 = dcgettext(0,"Archive octal value %.*s is out of %s range; assuming two\'s complement",
                        5);
      error(0,0,uVar4,(int)ppuVar20 - (int)local_488,local_488,param_3);
      ppuVar18 = local_480;
      ppuVar3 = local_470;
    }
    if ((ppuVar11 != ppuVar20) && (*(byte *)ppuVar20 != 0)) {
      if ((*(byte *)((long)*ppuVar3 + (ulong)*(byte *)ppuVar20 * 2 + 1) & 0x20) != 0) {
LAB_00120e33:
        ppuVar19 = (ushort **)-(long)ppuVar19;
        goto LAB_00120ca8;
      }
      goto LAB_00120a9a;
    }
    ppuVar19 = (ushort **)~(ulong)ppuVar18;
  }
LAB_00120ca8:
  uVar4 = FUN_00123b90(ppuVar19);
  puVar13 = auStack_498;
  goto LAB_00120a07;
LAB_00120baf:
  bVar2 = *(byte *)ppuVar9;
  if ((char)(&DAT_00184460)[bVar2] < '@') {
    if ((ushort **)((ulong)ppuVar19 & 0x3ffffffffffffff) == ppuVar19) goto code_r0x00120b9f;
    puVar13 = auStack_498;
    while (puVar15 != auStack_498 + -(param_2 + 0x18 & 0xfffffffffffff000)) {
      puVar14 = puVar13 + -0x1000;
      *(undefined8 *)(puVar13 + -8) = *(undefined8 *)(puVar13 + -8);
      puVar15 = puVar13 + -0x1000;
      puVar13 = puVar13 + -0x1000;
    }
    uVar8 = (ulong)((uint)(param_2 + 0x18) & 0xff0);
    lVar7 = -uVar8;
    puVar13 = puVar14 + lVar7;
    if (uVar8 != 0) {
      *(undefined8 *)(puVar14 + -8) = *(undefined8 *)(puVar14 + -8);
    }
    __dest = (void *)((ulong)(puVar14 + lVar7 + 0xf) & 0xfffffffffffffff0);
    *(undefined8 *)(puVar14 + lVar7 + -8) = 0x121016;
    memcpy(__dest,param_1,param_2);
    *(undefined1 *)((long)__dest + param_2) = 0;
    puVar12 = puVar14 + lVar7;
    if (((char)local_474 != '\x01') && (puVar12 = puVar14 + lVar7, param_3 != 0)) {
      if (DAT_00184d50 != (code *)0x0) {
        *(undefined8 *)(puVar14 + lVar7 + -8) = 0x12103f;
        (*DAT_00184d50)();
      }
      *(undefined8 *)(puVar14 + lVar7 + -8) = 0x121047;
      uVar4 = FUN_0014ac50(__dest);
      *(undefined8 *)(puVar14 + lVar7 + -8) = 0x12105d;
      uVar5 = dcgettext(0,"Archive signed base-64 string %s is out of %s range",5);
      *(undefined8 *)(puVar14 + lVar7 + -8) = 0x121071;
      error(0,0,uVar5,uVar4,param_3);
      uVar4 = 0xffffffffffffffff;
      DAT_00184d58 = 2;
      goto LAB_00120a07;
    }
    goto LAB_00120a00;
  }
  if (((ppuVar9 != ppuVar11) && (bVar2 != 0)) &&
     ((*(byte *)((long)*ppuVar3 + (ulong)bVar2 * 2 + 1) & 0x20) == 0)) goto LAB_00120a91;
  goto LAB_00120bd5;
code_r0x00120b9f:
  ppuVar9 = (ushort **)((long)ppuVar9 + 1);
  ppuVar19 = (ushort **)((long)ppuVar19 << 6 | (long)(char)(&DAT_00184460)[bVar2]);
  if (ppuVar11 == ppuVar9) goto LAB_00120bd5;
  goto LAB_00120baf;
LAB_00120eb2:
  bVar1 = *(byte *)ppuVar9;
  ppuVar9 = (ushort **)((long)ppuVar9 + 1);
  ppuVar19 = (ushort **)((long)ppuVar19 * 0x100 + (ulong)bVar1);
  if (ppuVar11 != ppuVar9) goto LAB_00120ea0;
  bVar22 = (bVar2 & 0x40) != 0;
  if ((bVar2 & 0x40) != 0) {
    ppuVar19 = (ushort **)-(long)ppuVar19;
  }
LAB_00120bd5:
  if (!bVar22) goto LAB_001209e0;
  if (ppuVar19 <= local_470) goto LAB_00120e33;
  puVar12 = auStack_498;
  if (((char)local_474 != '\x01') && (puVar12 = auStack_498, param_3 != 0)) {
    lVar6 = FUN_00143090(local_470,local_447);
    lVar7 = FUN_00143090(ppuVar19,local_427);
    *(undefined1 *)(lVar7 + -1) = 0x2d;
    lVar7 = lVar7 + -1;
    goto LAB_00120c2e;
  }
  goto LAB_00120a00;
LAB_00120ea0:
  if ((ushort **)((ulong)ppuVar19 & 0xffffffffffffff | (ulong)-(bVar2 & 0x40) << 0x32) != ppuVar19)
  goto LAB_00121092;
  goto LAB_00120eb2;
LAB_00121092:
  puVar12 = auStack_498;
  if ((param_3 != 0) && (puVar12 = auStack_498, (char)local_474 != '\x01')) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar16 = "Archive base-256 value is out of %s range";
LAB_00120a58:
    uVar4 = dcgettext(0,pcVar16,5);
    error(0,0,uVar4,param_3);
    DAT_00184d58 = 2;
    puVar12 = auStack_498;
  }
  goto LAB_00120a00;
}




// Function: base64_init @ 0x21160

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void base64_init(void)

{
  long lVar1;
  long lVar2;
  
  lVar2 = 0x41;
  _DAT_00184460 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184468 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184470 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184478 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184480 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184488 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184490 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184498 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844a0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844a8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844b0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844b8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844c0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844c8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844d0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844d8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844e0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844e8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_001844f0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam00000000001844f8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184500 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184508 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184510 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184518 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184520 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184528 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184530 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184538 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184540 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184548 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  _DAT_00184550 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._64_8_;
  uRam0000000000184558 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00162520._72_8_;
  lVar1 = 0;
  while( true ) {
    (&DAT_00184460)[lVar2] = (char)lVar1;
    if (lVar1 + 1 == 0x40) break;
    lVar2 = (long)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"
                  [lVar1 + 1];
    lVar1 = lVar1 + 1;
  }
  return;
}




// Function: transform_stat_info @ 0x21440

void transform_stat_info(int param_1,long param_2)

{
  if (param_1 == 0x56) {
    return;
  }
  FUN_00121420(param_2 + 8,1);
  if (param_1 == 0x31) {
    FUN_00121420(param_2 + 0x18,2);
  }
  else if (param_1 == 0x32) {
    FUN_00121420(param_2 + 0x18,4);
  }
  if (DAT_00184ad0 != '\0') {
    FUN_00121360();
    return;
  }
  return;
}




// Function: tar_checksum @ 0x214d0

undefined1  [16] tar_checksum(byte *param_1,byte param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  byte *pbVar4;
  int iVar5;
  undefined8 uVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  
  iVar5 = 0;
  iVar3 = 0;
  pbVar4 = param_1;
  do {
    auVar8._8_8_ = pbVar4 + 1;
    iVar3 = iVar3 + (uint)*pbVar4;
    iVar5 = iVar5 + (char)*pbVar4;
    pbVar4 = auVar8._8_8_;
  } while (param_1 + 0x200 != auVar8._8_8_);
  if (iVar3 == 0) {
    auVar8._0_8_ = 3;
    return auVar8;
  }
  pbVar4 = param_1 + 0x9b;
  do {
    bVar1 = *pbVar4;
    pbVar4 = pbVar4 + -1;
    iVar3 = iVar3 - (uint)bVar1;
    iVar5 = iVar5 - (char)bVar1;
  } while (param_1 + 0x93 != pbVar4);
  auVar9._9_7_ = 0;
  auVar9[8] = param_2;
  iVar2 = FUN_00120890(param_1 + 0x94,8,0,0,0x7fffffff,1);
  if (-1 < iVar2) {
    if ((iVar3 + 0x100 == iVar2) || (uVar6 = 5, iVar5 + 0x100 == iVar2)) {
      uVar6 = 1;
    }
    auVar7[8] = param_2;
    auVar7._0_8_ = uVar6;
    auVar7._9_7_ = 0;
    return auVar7;
  }
  auVar9._0_8_ = 5;
  return auVar9;
}




// Function: decode_header @ 0x215c0

void decode_header(long param_1,long param_2,int *param_3,int param_4)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 uVar3;
  int iVar4;
  uint uVar5;
  uint __major;
  undefined8 uVar6;
  ulonglong uVar7;
  long lVar8;
  int iVar9;
  long in_FS_OFFSET;
  char local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_00121210(param_1 + 100,&local_41);
  iVar4 = strcmp((char *)(param_1 + 0x101),"ustar");
  if (iVar4 == 0) {
    if ((((*(char *)(param_1 + 0x1db) != '\0') || (7 < (byte)(*(char *)(param_1 + 0x1dc) - 0x30U)))
        || (*(char *)(param_1 + 0x1e7) != ' ')) ||
       ((7 < (byte)(*(char *)(param_1 + 0x1e8) - 0x30U) ||
        (iVar9 = 5, *(char *)(param_1 + 499) != ' ')))) {
      iVar9 = 4 - (uint)(*(long *)(param_2 + 0x170) == 0);
    }
  }
  else {
    iVar9 = 1;
    iVar4 = strcmp((char *)(param_1 + 0x101),"ustar  ");
    if (iVar4 == 0) {
      iVar9 = (-(uint)(local_41 == '\0') & 4) + 2;
    }
  }
  *param_3 = iVar9;
  *(undefined4 *)(param_2 + 0x70) = uVar3;
  uVar6 = FUN_00121320(param_1 + 0x88);
  lVar8 = 0;
  cVar2 = *(char *)(param_1 + 0x109);
  *(undefined8 *)(param_2 + 0x100) = 0;
  *(undefined8 *)(param_2 + 0xf8) = uVar6;
  if (cVar2 != '\0') {
    lVar8 = param_1 + 0x109;
  }
  FUN_00123c20(param_2 + 0x20,lVar8,0x20);
  lVar8 = 0;
  if (*(char *)(param_1 + 0x129) != '\0') {
    lVar8 = param_1 + 0x129;
  }
  FUN_00123c20(param_2 + 0x28,lVar8,0x20);
  FUN_0011c5b0(param_2);
  uVar1 = DAT_00184a28;
  uVar6 = DAT_00184a20;
  if (iVar9 == 2) {
    if (DAT_00184b50 == '\0') {
      *(undefined8 *)(param_2 + 0x108) = DAT_00184a20;
      *(undefined8 *)(param_2 + 0x110) = uVar1;
      *(undefined8 *)(param_2 + 0xe8) = uVar6;
      *(undefined8 *)(param_2 + 0xf0) = uVar1;
    }
    else {
      uVar6 = FUN_00121320(param_1 + 0x159);
      *(undefined8 *)(param_2 + 0xe8) = uVar6;
      uVar6 = FUN_00121320(param_1 + 0x165);
      *(undefined8 *)(param_2 + 0x110) = 0;
      *(undefined8 *)(param_2 + 0x108) = uVar6;
      *(undefined8 *)(param_2 + 0xf0) = 0;
    }
  }
  else if (iVar9 == 5) {
    uVar6 = FUN_00121320(param_1 + 0x1dc);
    *(undefined8 *)(param_2 + 0xe8) = uVar6;
    uVar6 = FUN_00121320(param_1 + 0x1e8);
    *(undefined8 *)(param_2 + 0x110) = 0;
    *(undefined8 *)(param_2 + 0x108) = uVar6;
    *(undefined8 *)(param_2 + 0xf0) = 0;
  }
  else {
    *(undefined8 *)(param_2 + 0x108) = DAT_00184a20;
    *(undefined8 *)(param_2 + 0x110) = uVar1;
    *(undefined8 *)(param_2 + 0xe8) = uVar6;
    *(undefined8 *)(param_2 + 0xf0) = uVar1;
    if (iVar9 == 1) {
      uVar3 = FUN_00121260(param_1 + 0x6c);
      *(undefined4 *)(param_2 + 0x74) = uVar3;
      uVar3 = FUN_00121290(param_1 + 0x74);
      *(undefined4 *)(param_2 + 0x78) = uVar3;
      uVar7 = 0;
      goto LAB_0012173b;
    }
  }
  if (param_4 != 0) {
    if (((DAT_00184ad2 != '\0') || (*(char *)(param_1 + 0x109) == '\0')) ||
       (iVar4 = FUN_001269f0(param_1 + 0x109,param_2 + 0x74), iVar4 == 0)) {
      uVar3 = FUN_00121260(param_1 + 0x6c);
      *(undefined4 *)(param_2 + 0x74) = uVar3;
    }
    if (((DAT_00184ad2 != '\0') || (*(char *)(param_1 + 0x129) == '\0')) ||
       (iVar4 = FUN_00126aa0(param_1 + 0x129,param_2 + 0x78), iVar4 == 0)) {
      uVar3 = FUN_00121290(param_1 + 0x74);
      *(undefined4 *)(param_2 + 0x78) = uVar3;
    }
  }
  uVar7 = 0;
  if ((int)*(char *)(param_1 + 0x9c) - 0x33U < 2) {
    uVar5 = FUN_001212c0(param_1 + 0x151);
    __major = FUN_001212f0(param_1 + 0x149);
    uVar7 = gnu_dev_makedev(__major,uVar5);
  }
LAB_0012173b:
  *(ulonglong *)(param_2 + 0x80) = uVar7;
  FUN_0011c8c0(param_2);
  cVar2 = FUN_00129e10(param_2);
  uVar5 = DAT_00184400;
  if (cVar2 == '\0') {
    *(undefined1 *)(param_2 + 0x120) = 0;
    if ((((uVar5 & 0xfffffffb) == 2) && (*(char *)(DAT_00184408 + 0x9c) == 'D')) ||
       (*(long *)(param_2 + 400) != 0)) {
      *(undefined1 *)(param_2 + 0x188) = 1;
    }
  }
  else {
    FUN_00129e70(param_2);
    *(undefined1 *)(param_2 + 0x120) = 1;
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: off_from_header @ 0x219b0

void off_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00120890(param_1,param_2,"off_t",0,0x7fffffffffffffff,0,0);
  return;
}




// Function: read_header @ 0x219e0

int read_header(undefined8 *param_1,long param_2,int param_3)

{
  char cVar1;
  undefined4 uVar2;
  undefined1 uVar3;
  int iVar4;
  int iVar5;
  undefined8 *puVar6;
  ulong uVar7;
  undefined8 uVar8;
  long lVar9;
  undefined8 *puVar10;
  void *__src;
  char *__s;
  size_t sVar11;
  long lVar12;
  ulong uVar13;
  undefined8 *puVar14;
  long in_FS_OFFSET;
  byte bVar15;
  undefined8 *local_188;
  undefined8 *local_180;
  ulong local_178;
  ulong local_170;
  undefined1 local_168 [16];
  undefined1 local_158 [16];
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  ulong local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined2 local_b0;
  undefined1 local_ae;
  undefined1 local_ad;
  long local_40;
  
  bVar15 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_178 = 0;
  local_170 = 0;
  local_188 = (undefined8 *)0x0;
  local_180 = (undefined8 *)0x0;
LAB_00121a40:
  puVar6 = (undefined8 *)FUN_0010e130();
  *param_1 = puVar6;
joined_r0x00121a4e:
  if (puVar6 == (undefined8 *)0x0) {
    iVar4 = 4;
    goto LAB_00121b40;
  }
  iVar4 = FUN_001214d0(puVar6,0);
  if (iVar4 != 1) goto LAB_00121b40;
  if (*(char *)((long)puVar6 + 0x9c) == '1') {
    *(undefined8 *)(param_2 + 0x88) = 0;
LAB_00121a83:
    free(DAT_001843f8);
    puVar10 = local_180 + 0x40;
    if (local_180 == (undefined8 *)0x0) {
      if (*(char *)((long)puVar6 + 0x159) == '\0') {
        __s = (char *)&local_148;
      }
      else {
        iVar5 = strcmp((char *)((long)puVar6 + 0x101),"ustar");
        __s = (char *)&local_148;
        if (iVar5 == 0) {
          local_148 = *(undefined8 *)((long)puVar6 + 0x159);
          uStack_140 = *(undefined8 *)((long)puVar6 + 0x161);
          local_b8 = *(undefined8 *)((long)puVar6 + 0x1e9);
          local_ad = 0;
          local_138 = *(undefined8 *)((long)puVar6 + 0x169);
          uStack_130 = *(undefined8 *)((long)puVar6 + 0x171);
          local_128 = *(undefined8 *)((long)puVar6 + 0x179);
          uStack_120 = *(undefined8 *)((long)puVar6 + 0x181);
          local_118 = *(undefined8 *)((long)puVar6 + 0x189);
          uStack_110 = *(undefined8 *)((long)puVar6 + 0x191);
          local_108 = *(undefined8 *)((long)puVar6 + 0x199);
          uStack_100 = *(undefined8 *)((long)puVar6 + 0x1a1);
          local_f8 = *(undefined8 *)((long)puVar6 + 0x1a9);
          uStack_f0 = *(undefined8 *)((long)puVar6 + 0x1b1);
          local_ae = *(undefined1 *)((long)puVar6 + 499);
          local_e8 = *(ulong *)((long)puVar6 + 0x1b9);
          uStack_e0 = *(undefined8 *)((long)puVar6 + 0x1c1);
          local_b0 = *(undefined2 *)((long)puVar6 + 0x1f1);
          local_d8 = *(undefined8 *)((long)puVar6 + 0x1c9);
          uStack_d0 = *(undefined8 *)((long)puVar6 + 0x1d1);
          local_c8 = *(undefined8 *)((long)puVar6 + 0x1d9);
          uStack_c0 = *(undefined8 *)((long)puVar6 + 0x1e1);
          sVar11 = strlen(__s);
          __s[sVar11] = '/';
          __s = (char *)((long)&local_148 + sVar11 + 1);
        }
      }
      puVar10 = &local_148;
      local_170 = 0;
      uVar8 = puVar6[1];
      *(undefined8 *)__s = *puVar6;
      *(undefined8 *)(__s + 8) = uVar8;
      uVar8 = puVar6[3];
      *(undefined8 *)(__s + 0x10) = puVar6[2];
      *(undefined8 *)(__s + 0x18) = uVar8;
      uVar8 = puVar6[5];
      *(undefined8 *)(__s + 0x20) = puVar6[4];
      *(undefined8 *)(__s + 0x28) = uVar8;
      uVar8 = puVar6[7];
      *(undefined8 *)(__s + 0x30) = puVar6[6];
      *(undefined8 *)(__s + 0x38) = uVar8;
      uVar8 = puVar6[9];
      *(undefined8 *)(__s + 0x40) = puVar6[8];
      *(undefined8 *)(__s + 0x48) = uVar8;
      uVar8 = puVar6[0xb];
      *(undefined8 *)(__s + 0x50) = puVar6[10];
      *(undefined8 *)(__s + 0x58) = uVar8;
      uVar2 = *(undefined4 *)(puVar6 + 0xc);
      __s[100] = '\0';
      *(undefined4 *)(__s + 0x60) = uVar2;
    }
    DAT_001843f8 = local_180;
    DAT_001843e8 = local_170;
    FUN_00123be0(param_2,puVar10);
    FUN_00123be0(param_2 + 8,puVar10);
    uVar3 = FUN_0013e630(*(undefined8 *)(param_2 + 8));
    puVar10 = DAT_001843f0;
    *(undefined1 *)(param_2 + 0x10) = uVar3;
    free(puVar10);
    if (local_188 == (undefined8 *)0x0) {
      local_148 = *(undefined8 *)((long)puVar6 + 0x9d);
      uStack_140 = *(undefined8 *)((long)puVar6 + 0xa5);
      puVar10 = &local_148;
      local_138 = *(undefined8 *)((long)puVar6 + 0xad);
      uStack_130 = *(undefined8 *)((long)puVar6 + 0xb5);
      local_128 = *(undefined8 *)((long)puVar6 + 0xbd);
      uStack_120 = *(undefined8 *)((long)puVar6 + 0xc5);
      local_178 = 0;
      local_118 = *(undefined8 *)((long)puVar6 + 0xcd);
      uStack_110 = *(undefined8 *)((long)puVar6 + 0xd5);
      local_e8 = CONCAT44((int)(local_e8 >> 0x20),*(undefined4 *)((long)puVar6 + 0xfd)) &
                 0xffffff00ffffffff;
      local_108 = *(undefined8 *)((long)puVar6 + 0xdd);
      uStack_100 = *(undefined8 *)((long)puVar6 + 0xe5);
      local_f8 = *(undefined8 *)((long)puVar6 + 0xed);
      uStack_f0 = *(undefined8 *)((long)puVar6 + 0xf5);
    }
    else {
      puVar10 = local_188 + 0x40;
    }
    DAT_001843f0 = local_188;
    DAT_001843e0 = local_178;
    FUN_00123be0(param_2 + 0x18,puVar10);
    local_188 = (undefined8 *)0x0;
    local_180 = (undefined8 *)0x0;
    goto LAB_00121b40;
  }
  lVar12 = (long)puVar6 + 0x7c;
  uVar7 = FUN_001219b0(lVar12,0xc);
  *(ulong *)(param_2 + 0x88) = uVar7;
  if ((long)uVar7 < 0) {
    iVar4 = 5;
    goto LAB_00121b40;
  }
  cVar1 = *(char *)((long)puVar6 + 0x9c);
  if (((byte)(cVar1 + 0xb5U) < 2) || (cVar1 == 'x')) {
    if (param_3 == 1) goto LAB_00121cc0;
    if ((byte)(cVar1 + 0xb5U) < 2) goto LAB_00121cd0;
  }
  else {
    if ((cVar1 != 'g') && (cVar1 != 'X')) goto LAB_00121a83;
    if (param_3 == 1) goto LAB_00121cc0;
    if (cVar1 != 'X') goto code_r0x00121c2e;
  }
  uVar8 = FUN_001219b0(lVar12,0xc);
  FUN_0011cac0(param_2 + 0x168,puVar6,uVar8);
  puVar6 = (undefined8 *)FUN_0010e130();
  *param_1 = puVar6;
  goto joined_r0x00121a4e;
LAB_00121cd0:
  uVar13 = uVar7 + 0x200;
  if (((uVar7 & 0x1ff) != 0) &&
     (uVar13 = (uVar7 + 0x400) - (ulong)((uint)uVar7 & 0x1ff), uVar13 < uVar7)) {
                    /* WARNING: Subroutine does not return */
    FUN_00116420();
  }
  puVar10 = (undefined8 *)FUN_0014c5b0(uVar13 + 1);
  if (*(char *)((long)puVar6 + 0x9c) == 'L') {
    free(local_180);
    local_180 = puVar10;
    local_170 = uVar13 >> 9;
  }
  else {
    free(local_188);
    local_188 = puVar10;
    local_178 = uVar13 >> 9;
  }
  FUN_0010d1f0(puVar6);
  puVar14 = puVar10 + 0x40;
  lVar12 = (long)puVar10 - (long)((ulong)(puVar10 + 1) & 0xfffffffffffffff8);
  *puVar10 = *puVar6;
  puVar10[0x3f] = puVar6[0x3f];
  puVar6 = (undefined8 *)((long)puVar6 - lVar12);
  puVar10 = (undefined8 *)((ulong)(puVar10 + 1) & 0xfffffffffffffff8);
  for (uVar7 = (ulong)((int)lVar12 + 0x200U >> 3); uVar7 != 0; uVar7 = uVar7 - 1) {
    *puVar10 = *puVar6;
    puVar6 = puVar6 + (ulong)bVar15 * -2 + 1;
    puVar10 = puVar10 + (ulong)bVar15 * -2 + 1;
  }
  for (uVar13 = uVar13 - 0x200; uVar13 != 0; uVar13 = uVar13 - uVar7) {
    __src = (void *)FUN_0010e130();
    if (__src == (void *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar8 = dcgettext(0,"Unexpected EOF in archive",5);
      error(0,0,uVar8);
      DAT_00184d58 = 2;
      break;
    }
    uVar7 = FUN_0010d2b0(__src);
    if (uVar13 <= uVar7) {
      uVar7 = uVar13;
    }
    memcpy(puVar14,__src,uVar7);
    puVar14 = (undefined8 *)((long)puVar14 + uVar7);
    FUN_0010d1f0((long)__src + (uVar7 - 1));
  }
  *(undefined1 *)puVar14 = 0;
  goto LAB_00121a40;
code_r0x00121c2e:
  if (DAT_00184560 == (undefined8 *)0x0) {
    DAT_00184560 = (undefined8 *)FUN_0014c5b0(0x200);
  }
  puVar14 = DAT_00184560;
  puVar10 = DAT_00184560 + 1;
  *DAT_00184560 = *puVar6;
  puVar14[0x3f] = puVar6[0x3f];
  lVar9 = (long)puVar14 - (long)((ulong)puVar10 & 0xfffffffffffffff8);
  puVar14 = (undefined8 *)((long)puVar6 - lVar9);
  puVar10 = (undefined8 *)((ulong)puVar10 & 0xfffffffffffffff8);
  for (uVar7 = (ulong)((int)lVar9 + 0x200U >> 3); uVar7 != 0; uVar7 = uVar7 - 1) {
    *puVar10 = *puVar14;
    puVar14 = puVar14 + (ulong)bVar15 * -2 + 1;
    puVar10 = puVar10 + (ulong)bVar15 * -2 + 1;
  }
  local_168 = (undefined1  [16])0x0;
  local_158 = (undefined1  [16])0x0;
  uVar8 = FUN_001219b0(lVar12,0xc);
  FUN_0011cac0(local_168,puVar6,uVar8);
  FUN_0011c9a0(local_168);
  FUN_0011cca0(local_168);
  if (param_3 == 2) {
LAB_00121cc0:
    iVar4 = 2;
LAB_00121b40:
    free(local_180);
    free(local_188);
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return iVar4;
  }
  goto LAB_00121a40;
}




// Function: uintmax_from_header @ 0x22050

void uintmax_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00120890(param_1,param_2,"uintmax_t",0,0xffffffffffffffff,0,0);
  return;
}




// Function: tartime @ 0x22080

undefined * tartime(long param_1,int param_2,char param_3)

{
  tm *__tp;
  undefined *puVar1;
  size_t sVar2;
  undefined *puVar3;
  undefined *puVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = param_1;
  if ((param_2 != 0) && (param_1 < 0)) {
    local_28 = param_1 + 1;
    param_2 = 1000000000 - param_2;
  }
  if (DAT_00184b91 == '\0') {
    __tp = localtime(&local_28);
  }
  else {
    __tp = gmtime(&local_28);
  }
  if (__tp == (tm *)0x0) {
    if (param_1 < 0) {
      lVar5 = FUN_00143090(-local_28,&DAT_00184426);
      puVar1 = (undefined *)(lVar5 + -1);
      *(undefined1 *)(lVar5 + -1) = 0x2d;
    }
    else {
      puVar1 = (undefined *)FUN_00143090(local_28,&DAT_00184426);
    }
    puVar4 = puVar1;
    puVar3 = &DAT_00184441;
    if (param_3 == '\0') {
      puVar3 = &DAT_00184434;
    }
    for (; puVar3 < puVar4; puVar4 = puVar4 + -1) {
      puVar4[-1] = 0x20;
    }
    lVar5 = (long)puVar3 - (long)puVar1;
    if (puVar1 < puVar3) {
      lVar5 = 0;
    }
    puVar1 = puVar1 + lVar5;
    if (param_3 != '\0') {
      FUN_00124170(param_2,&DAT_0018443a);
    }
  }
  else if (param_3 == '\0') {
    strftime(&DAT_00184420,0x25,"%Y-%m-%d %H:%M",__tp);
    puVar1 = &DAT_00184420;
  }
  else {
    puVar1 = &DAT_00184420;
    strftime(&DAT_00184420,0x25,"%Y-%m-%d %H:%M:%S",__tp);
    sVar2 = strlen(&DAT_00184420);
    FUN_00124170(param_2,&DAT_00184420 + sVar2);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: simple_print_header @ 0x22220

void simple_print_header(long *param_1,long param_2,long param_3)

{
  FILE *__stream;
  int iVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
  size_t sVar6;
  char *__s;
  char *__s_00;
  size_t sVar7;
  int iVar8;
  long lVar9;
  long in_FS_OFFSET;
  undefined1 *local_f0;
  undefined1 local_e6;
  undefined1 local_e5;
  char local_e4;
  undefined1 local_e3 [9];
  undefined1 local_da [2];
  undefined1 local_d8 [32];
  undefined1 local_b8 [32];
  undefined1 local_98 [32];
  char local_78 [56];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001847e1 == '\0') {
    lVar9 = *param_1;
    if (lVar9 == 0) {
      lVar9 = param_1[1];
    }
  }
  else {
    lVar9 = param_1[1];
    if (lVar9 == 0) {
      lVar9 = *param_1;
    }
  }
  if (DAT_00184b7c != '\0') {
    if (param_3 < 0) {
      param_3 = FUN_0010d180();
    }
    uVar3 = FUN_00143090((param_3 - DAT_001843e0) - DAT_001843e8,local_78);
    uVar4 = dcgettext(0,"block %s: ",5);
    __fprintf_chk(DAT_00183f28,1,uVar4,uVar3);
  }
  __stream = DAT_00183f28;
  if (DAT_00184a54 < 2) {
    pcVar5 = (char *)FUN_0014a8a0(lVar9);
    fputs_unlocked(pcVar5,__stream);
    if ((DAT_001847e1 != '\0') && ((char)param_1[2] != '\0')) {
      fputc_unlocked(0x2f,DAT_00183f28);
    }
    fputc_unlocked(10,DAT_00183f28);
    goto LAB_001225f0;
  }
  local_e4 = '?';
  switch(*(undefined1 *)(param_2 + 0x9c)) {
  case 0:
  case 0x30:
  case 0x53:
    local_e4 = (-((char)param_1[2] == '\0') & 0xc9U) + 100;
    break;
  case 0x31:
    local_e4 = 'h';
    break;
  case 0x32:
    local_e4 = 'l';
    break;
  case 0x33:
    local_e4 = 'c';
    break;
  case 0x34:
    local_e4 = 'b';
    break;
  case 0x35:
  case 0x44:
    local_e4 = 'd';
    break;
  case 0x36:
    local_e4 = 'p';
    break;
  case 0x37:
    local_e4 = 'C';
    break;
  case 0x4b:
  case 0x4c:
    local_e4 = 'L';
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar3 = dcgettext(0,"Unexpected long name header",5);
    error(0,0,uVar3);
    DAT_00184d58 = 2;
    break;
  case 0x4d:
    local_e4 = 'M';
    break;
  case 0x56:
    DAT_00184451 = 1;
    local_e4 = 'V';
  }
  FUN_00132310((int)param_1[0xe],local_e3);
  FUN_00132010(param_1,local_da);
  pcVar5 = (char *)FUN_00122080(param_1[0x1f],param_1[0x20],DAT_00184b90);
  sVar6 = strlen(pcVar5);
  if (DAT_00181228 < (int)sVar6) {
    DAT_00181228 = (int)sVar6;
  }
  __s = (char *)param_1[4];
  if ((((__s == (char *)0x0) || (*__s == '\0')) || (DAT_00184400 == 1)) || (DAT_00184ad2 != '\0')) {
    __s = (char *)FUN_00143090(*(undefined4 *)((long)param_1 + 0x74),local_d8);
  }
  __s_00 = (char *)param_1[5];
  if (((__s_00 == (char *)0x0) || (*__s_00 == '\0')) ||
     ((DAT_00184400 == 1 || (DAT_00184ad2 != '\0')))) {
    __s_00 = (char *)FUN_00143090((int)param_1[0xf],local_b8);
  }
  if ((int)*(char *)(param_2 + 0x9c) - 0x33U < 2) {
    uVar2 = gnu_dev_major(param_1[0x10]);
    uVar3 = FUN_00143090(uVar2,local_98);
    __strcpy_chk(local_78,uVar3,0x2a);
    __strcat_chk(local_78,&DAT_0016079c,0x2a);
    uVar2 = gnu_dev_minor(param_1[0x10]);
    uVar3 = FUN_00143090(uVar2,local_98);
    __strcat_chk(local_78,uVar3,0x2a);
  }
  else {
    uVar3 = FUN_00143090(param_1[0x11],local_98);
    __strcpy_chk(local_78,uVar3,0x2a);
  }
  local_f0 = local_98;
  sVar6 = strlen(local_78);
  iVar8 = (int)sVar6;
  sVar6 = strlen(__s);
  sVar7 = strlen(__s_00);
  iVar1 = (int)sVar6 + 2 + (int)sVar7 + iVar8;
  if (iVar1 <= DAT_0018122c) {
    iVar8 = (DAT_0018122c - iVar1) + iVar8;
    iVar1 = DAT_0018122c;
  }
  DAT_0018122c = iVar1;
  __fprintf_chk(DAT_00183f28,1,"%s %s/%s %*s %-*s",&local_e4,__s,__s_00,iVar8,local_78,DAT_00181228,
                pcVar5);
  uVar3 = FUN_0014a8a0(lVar9);
  __fprintf_chk(DAT_00183f28,1," %s",uVar3);
  if ((DAT_001847e1 != '\0') && ((char)param_1[2] != '\0')) {
    fputc_unlocked(0x2f,DAT_00183f28);
  }
  switch(*(undefined1 *)(param_2 + 0x9c)) {
  case 0:
  case 0x30:
  case 0x33:
  case 0x34:
  case 0x35:
  case 0x36:
  case 0x37:
  case 0x44:
  case 0x53:
    putc_unlocked(10,DAT_00183f28);
    break;
  default:
    local_e5 = 0;
    local_e6 = *(undefined1 *)(param_2 + 0x9c);
    uVar3 = FUN_0014ac50(&local_e6);
    pcVar5 = " unknown file type %s\n";
    goto LAB_00122652;
  case 0x31:
    uVar3 = FUN_0014a8a0(param_1[3]);
    pcVar5 = " link to %s\n";
LAB_00122652:
    uVar4 = dcgettext(0,pcVar5,5);
    __fprintf_chk(DAT_00183f28,1,uVar4,uVar3);
    break;
  case 0x32:
    uVar3 = FUN_0014a8a0(param_1[3]);
    __fprintf_chk(DAT_00183f28,1," -> %s\n",uVar3);
    break;
  case 0x4b:
    pcVar5 = "--Long Link--\n";
    goto LAB_00122764;
  case 0x4c:
    pcVar5 = "--Long Name--\n";
    goto LAB_00122764;
  case 0x4d:
    uVar3 = FUN_00122050(param_2 + 0x171,0xc);
    uVar3 = FUN_00143090(uVar3,local_f0);
    __strcpy_chk(local_78,uVar3,0x2a);
    uVar3 = dcgettext(0,"--Continued at byte %s--\n",5);
    __fprintf_chk(DAT_00183f28,1,uVar3,local_78);
    break;
  case 0x56:
    pcVar5 = "--Volume Header--\n";
LAB_00122764:
    uVar3 = dcgettext(0,pcVar5,5);
    __fprintf_chk(DAT_00183f28,1,uVar3);
  }
LAB_001225f0:
  fflush_unlocked(DAT_00183f28);
  FUN_00132150(param_1);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_volume_label @ 0x228b0

void print_volume_label(void)

{
  long lVar1;
  undefined8 *puVar2;
  long in_FS_OFFSET;
  undefined1 local_3ec [4];
  undefined1 local_3e8 [8];
  undefined1 local_3e0 [440];
  undefined8 local_228 [17];
  undefined8 local_1a0;
  undefined4 local_198;
  undefined1 local_18c;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  puVar2 = local_228;
  for (lVar1 = 0x40; lVar1 != 0; lVar1 = lVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  local_18c = 0x56;
  if (DAT_00184560 != 0) {
    local_1a0 = *(undefined8 *)(DAT_00184560 + 0x88);
    local_198 = *(undefined4 *)(DAT_00184560 + 0x90);
  }
  FUN_0012f630(local_3e8);
  FUN_00123be0(local_3e0,".");
  FUN_001215c0(local_228,local_3e8,local_3ec,0);
  FUN_00123be0(local_3e0,DAT_00183f18);
  FUN_00122220(local_3e8,local_228,0);
  FUN_0012f6f0(local_3e8);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_header @ 0x22990

void print_header(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  if (((DAT_00184400 == 4) && (DAT_00184451 == '\0')) && (DAT_00183f18 != 0)) {
    FUN_001228b0();
    DAT_00184451 = 1;
    FUN_00122220(param_1,param_2,param_3);
    return;
  }
  FUN_00122220();
  return;
}




// Function: print_for_mkdir @ 0x22a00

void print_for_mkdir(undefined8 param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_43;
  undefined1 local_42 [10];
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (1 < DAT_00184a54) {
    local_43 = 100;
    FUN_00132310(param_3,local_42);
    if (DAT_00184b7c != '\0') {
      uVar1 = FUN_0010d180();
      uVar1 = FUN_00143090(uVar1,local_38);
      uVar2 = dcgettext(0,"block %s: ",5);
      __fprintf_chk(DAT_00183f28,1,uVar2,uVar1);
    }
    uVar1 = FUN_0014a8a0(param_1);
    uVar2 = dcgettext(0,"Creating directory:",5);
    __fprintf_chk(DAT_00183f28,1,"%s %*s %s\n",&local_43,DAT_0018122c + 1 + DAT_00181228,uVar2,uVar1
                 );
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: skip_file @ 0x22b10

void skip_file(long param_1)

{
  ulong uVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  
  if (DAT_001847f8 != '\0') {
    lVar3 = FUN_0010d540();
    if (lVar3 < 0) {
      DAT_001847f8 = '\0';
    }
    else {
      param_1 = param_1 + lVar3 * -0x200;
    }
  }
  FUN_0010ccf0(param_1);
  if (0 < param_1) {
    uVar1 = param_1 - 1;
    lVar3 = param_1 + -0x200;
    do {
      lVar2 = FUN_0010e130();
      if (lVar2 == 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
      FUN_0010d1f0();
      param_1 = param_1 + -0x200;
      FUN_0010ccf0(param_1);
    } while (param_1 != lVar3 - (uVar1 & 0xfffffffffffffe00));
  }
  return;
}




// Function: skip_member @ 0x22bd0

void skip_member(void)

{
  char cVar1;
  
  if (DAT_001849c9 != '\0') {
    return;
  }
  cVar1 = *(char *)(DAT_00184408 + 0x9c);
  FUN_0010d1f0();
  FUN_0010ccb0(&DAT_00184840);
  if (DAT_00184960 == '\0') {
    if (cVar1 == '5') {
      FUN_0010ccd0();
      return;
    }
    FUN_00122b10(DAT_001848c8);
    FUN_0010ccd0();
    return;
  }
  FUN_00129fe0(&DAT_00184840);
  FUN_0010ccd0();
  return;
}




// Function: read_and @ 0x22c60

void read_and(code *param_1)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_58 [24];
  long local_40;
  
  iVar3 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00121160();
  FUN_00126d90();
  FUN_0010fb70(0);
LAB_00122cbd:
  FUN_0012f6f0(&DAT_00184840);
  uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
  switch(uVar2) {
  case 0:
  case 2:
switchD_00122ce0_caseD_0:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
switchD_00122ce0_caseD_1:
switchD_00122e5b_caseD_1:
    FUN_001215c0(DAT_00184408,&DAT_00184840,&DAT_00184400,1);
    cVar1 = FUN_00127250(DAT_00184848);
    if (cVar1 != '\0') {
      if (-1 < DAT_00184b08) {
        DAT_00184938 = FUN_00121320(DAT_00184408 + 0x88);
        DAT_00184940 = 0;
        iVar3 = FUN_0014be80(DAT_00184938,0,DAT_00184b00,DAT_00184b08);
        if (iVar3 < 0) goto LAB_00122df2;
      }
      cVar1 = FUN_00116b00(DAT_00184848,DAT_001849d8);
      if (cVar1 != '\0') goto LAB_00122df2;
      iVar3 = (int)*(char *)(DAT_00184408 + 0x9c);
LAB_00123000:
      FUN_00121440(iVar3,&DAT_00184840);
      (*param_1)();
      cVar1 = FUN_001273a0(&DAT_00184840);
      if (cVar1 == '\0') goto code_r0x0012301e;
      break;
    }
LAB_00122df2:
    cVar1 = *(char *)(DAT_00184408 + 0x9c);
    iVar3 = (int)cVar1;
    if ((cVar1 == 'M') || (cVar1 == 'V')) goto LAB_00123000;
    if ((cVar1 == '5') && (DAT_00184a89 != '\0')) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar4 = FUN_0014aa70(DAT_00184848);
      uVar6 = dcgettext(0,"%s: Omitting",5);
      error(0,0,uVar6,uVar4);
    }
    FUN_00122bd0();
    cVar1 = FUN_001273a0(&DAT_00184840);
    if (cVar1 != '\0') break;
    FUN_0012f6f0(&DAT_00184840);
    uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
    switch(uVar2) {
    case 0:
    case 2:
      goto switchD_00122ce0_caseD_0;
    case 1:
      goto switchD_00122e5b_caseD_1;
    case 3:
      goto switchD_00122e5b_caseD_3;
    case 4:
      goto switchD_00122ce0_caseD_4;
    case 5:
      goto switchD_00122e5b_caseD_5;
    default:
      break;
    }
  case 3:
    goto switchD_00122ce0_caseD_3;
  case 4:
switchD_00122ce0_caseD_4:
    if (DAT_00184b7c != '\0') {
      uVar4 = FUN_0010d180();
      uVar4 = FUN_00143090(uVar4,local_58);
      uVar6 = dcgettext(0,"block %s: ** End of File **\n",5);
      __fprintf_chk(DAT_00183f28,1,uVar6,uVar4);
    }
    break;
  case 5:
    FUN_0010d1f0(DAT_00184408);
    if (iVar3 != 0) {
      if (iVar3 == 1) goto LAB_00122eae;
      goto LAB_00122f49;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar4 = dcgettext(0,"This does not look like a tar archive",5);
    error(0,0,uVar4);
    DAT_00184d58 = 2;
    goto LAB_00122eae;
  }
  goto switchD_00122ce0_default;
code_r0x0012301e:
  FUN_0012f6f0(&DAT_00184840);
  uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
  switch(uVar2) {
  case 0:
  case 2:
    goto switchD_00122ce0_caseD_0;
  case 1:
    goto switchD_00122e5b_caseD_1;
  case 3:
switchD_00122e5b_caseD_3:
    iVar3 = 1;
    goto switchD_00122ce0_caseD_3;
  case 4:
    goto switchD_00122ce0_caseD_4;
  case 5:
    break;
  default:
    goto switchD_00122ce0_default;
  }
switchD_00122e5b_caseD_5:
  FUN_0010d1f0(DAT_00184408);
LAB_00122eae:
  if (DAT_00184b7c != '\0') {
    lVar5 = FUN_0010d180();
    uVar4 = FUN_00143090((lVar5 - DAT_001843e0) - DAT_001843e8,local_58);
    uVar6 = dcgettext(0,"block %s: ",5);
    __fprintf_chk(DAT_00183f28,1,uVar6,uVar4);
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar4 = dcgettext(0,"Skipping to next header",5);
  error(0,0,uVar4);
  DAT_00184d58 = 2;
  cVar1 = FUN_001273a0(&DAT_00184840);
  if (cVar1 != '\0') goto switchD_00122ce0_default;
  FUN_0012f6f0(&DAT_00184840);
  uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
  switch(uVar2) {
  case 0:
  case 2:
    goto switchD_00122ce0_caseD_0;
  case 1:
    goto switchD_00122ce0_caseD_1;
  case 3:
    break;
  case 4:
    goto switchD_00122ce0_caseD_4;
  case 5:
    FUN_0010d1f0(DAT_00184408);
LAB_00122f49:
    do {
      cVar1 = FUN_001273a0(&DAT_00184840);
      if (cVar1 != '\0') goto switchD_00122ce0_default;
      FUN_0012f6f0(&DAT_00184840);
      uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
      switch(uVar2) {
      case 0:
      case 2:
        goto switchD_00122ce0_caseD_0;
      case 1:
        goto switchD_00122ce0_caseD_1;
      case 3:
        goto switchD_00122f32_caseD_3;
      case 4:
        goto switchD_00122ce0_caseD_4;
      case 5:
        FUN_0010d1f0(DAT_00184408);
        cVar1 = FUN_001273a0(&DAT_00184840);
        if (cVar1 != '\0') goto switchD_00122ce0_default;
        FUN_0012f6f0(&DAT_00184840);
        uVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
        switch(uVar2) {
        case 0:
        case 2:
          goto switchD_00122ce0_caseD_0;
        case 1:
          goto switchD_00122ce0_caseD_1;
        case 3:
          goto switchD_00122f32_caseD_3;
        case 4:
          goto switchD_00122ce0_caseD_4;
        case 5:
          FUN_0010d1f0(DAT_00184408);
          break;
        default:
          goto switchD_00122ce0_default;
        }
        break;
      default:
        goto switchD_00122ce0_default;
      }
    } while( true );
  default:
    goto switchD_00122ce0_default;
  }
switchD_00122f32_caseD_3:
  iVar3 = 5;
switchD_00122ce0_caseD_3:
  if (DAT_00184b7c != '\0') {
    uVar4 = FUN_0010d180();
    uVar4 = FUN_00143090(uVar4,local_58);
    uVar6 = dcgettext(0,"block %s: ** Block of NULs **\n",5);
    __fprintf_chk(DAT_00183f28,1,uVar6,uVar4);
  }
  FUN_0010d1f0(DAT_00184408);
  if (DAT_00184b51 == '\0') {
    iVar3 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
    if ((iVar3 != 3) && (((byte)DAT_00183b80 & 1) != 0)) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar4 = FUN_0010d180();
      uVar4 = FUN_00143090(uVar4,local_58);
      uVar6 = dcgettext(0,"A lone zero block at %s",5);
      error(0,0,uVar6,uVar4);
    }
    goto switchD_00122ce0_default;
  }
  cVar1 = FUN_001273a0(&DAT_00184840);
  if (cVar1 != '\0') {
switchD_00122ce0_default:
    FUN_0010f900();
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    FUN_00127430();
    return;
  }
  goto LAB_00122cbd;
}




// Function: list_archive @ 0x23260

void list_archive(void)

{
  undefined8 uVar1;
  char cVar2;
  
  uVar1 = FUN_0010d180();
  if (DAT_00184a54 != 0) {
    FUN_00122990(&DAT_00184840,DAT_00184408,uVar1);
    if ((DAT_00184b50 != '\0') && (2 < DAT_00184a54)) {
      cVar2 = FUN_001201b0(&DAT_00184840);
      if (cVar2 != '\0') {
        uVar1 = FUN_0011f4c0(DAT_001849d0);
        FUN_00120710(DAT_001849d0,uVar1);
      }
    }
  }
  FUN_00122bd0();
  return;
}




// Function: test_archive_label @ 0x232e0

void test_archive_label(void)

{
  char cVar1;
  int iVar2;
  void *__ptr;
  
  FUN_00121160();
  FUN_00126d90();
  FUN_0010fb70(0);
  iVar2 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
  if (iVar2 == 1) {
    FUN_001215c0(DAT_00184408,&DAT_00184840,&DAT_00184400,0);
    if (*(char *)(DAT_00184408 + 0x9c) == 'V') {
      FUN_00123c20(&DAT_00183f18,DAT_00184408,100);
    }
    if (DAT_00183f18 != 0) {
      if (DAT_00184a54 != 0) {
        FUN_001228b0();
      }
      cVar1 = FUN_00127250(DAT_00183f18);
      if ((cVar1 == '\0') && (DAT_00184b10 != '\0')) {
        __ptr = (void *)FUN_0010de80(DAT_00183f18);
        FUN_00127250(__ptr);
        free(__ptr);
      }
    }
  }
  FUN_0010f900();
  FUN_001275b0();
  return;
}




// Function: map_hash @ 0x233c0

ulong map_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: map_compare @ 0x233d0

undefined8 map_compare(long *param_1,long *param_2)

{
  return CONCAT71((int7)((ulong)*param_2 >> 8),*param_1 == *param_2);
}




// Function: name_to_uid @ 0x233e0

ulong name_to_uid(char *param_1)

{
  passwd *ppVar1;
  
  ppVar1 = getpwnam(param_1);
  if (ppVar1 != (passwd *)0x0) {
    return (ulong)ppVar1->pw_uid;
  }
  return 0xffffffffffffffff;
}




// Function: name_to_gid @ 0x23410

ulong name_to_gid(char *param_1)

{
  group *pgVar1;
  
  pgVar1 = getgrnam(param_1);
  if (pgVar1 != (group *)0x0) {
    return (ulong)pgVar1->gr_gid;
  }
  return 0xffffffffffffffff;
}




// Function: owner_map_read @ 0x239a0

void owner_map_read(undefined8 param_1)

{
  FUN_00123520(&DAT_00184570,param_1,FUN_001233e0,"UID");
  return;
}




// Function: owner_map_translate @ 0x239d0

bool owner_map_translate(ulong param_1,int *param_2,long *param_3)

{
  long lVar1;
  long lVar2;
  long in_FS_OFFSET;
  bool bVar3;
  ulong local_38 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184570 != 0) {
    local_38[0] = param_1 & 0xffffffff;
    lVar2 = FUN_001418d0(DAT_00184570,local_38);
    if (lVar2 != 0) {
      lVar1 = *(long *)(lVar2 + 0x10);
      *param_2 = (int)*(undefined8 *)(lVar2 + 8);
      *param_3 = lVar1;
      bVar3 = false;
      goto LAB_00123a22;
    }
  }
  bVar3 = DAT_00184abc == -1;
  if (!bVar3) {
    *param_2 = DAT_00184abc;
  }
  if (DAT_00184ac0 != 0) {
    *param_3 = DAT_00184ac0;
    bVar3 = false;
  }
LAB_00123a22:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: group_map_read @ 0x23a70

void group_map_read(undefined8 param_1)

{
  FUN_00123520(&DAT_00184568,param_1,FUN_00123410,"GID");
  return;
}




// Function: group_map_translate @ 0x23aa0

bool group_map_translate(ulong param_1,int *param_2,long *param_3)

{
  long lVar1;
  long lVar2;
  long in_FS_OFFSET;
  bool bVar3;
  ulong local_38 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184568 != 0) {
    local_38[0] = param_1 & 0xffffffff;
    lVar2 = FUN_001418d0(DAT_00184568,local_38);
    if (lVar2 != 0) {
      lVar1 = *(long *)(lVar2 + 0x10);
      *param_2 = (int)*(undefined8 *)(lVar2 + 8);
      *param_3 = lVar1;
      bVar3 = false;
      goto LAB_00123af2;
    }
  }
  bVar3 = DAT_00184b54 == -1;
  if (!bVar3) {
    *param_2 = DAT_00184b54;
  }
  if (DAT_00184b58 != 0) {
    *param_3 = DAT_00184b58;
    bVar3 = false;
  }
LAB_00123af2:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ptr_align @ 0x23b40

long ptr_align(long param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = (param_2 - 1) + param_1;
  return uVar1 - uVar1 % param_2;
}




// Function: namebuf_finish @ 0x23b60

long namebuf_finish(long *param_1)

{
  long lVar1;
  
  lVar1 = *param_1;
  if (*(char *)(lVar1 + -1 + param_1[2]) == '/') {
    *(undefined1 *)(lVar1 + param_1[2]) = 0;
  }
  free(param_1);
  return lVar1;
}




// Function: quote_n_colon @ 0x23bb0

void quote_n_colon(undefined4 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_0014a570(0);
  FUN_0014aa90(param_1,uVar1,param_2);
  return;
}




// Function: assign_string @ 0x23be0

void assign_string(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  
  free((void *)*param_1);
  uVar1 = 0;
  if (param_2 != 0) {
    uVar1 = FUN_0014c7a0(param_2);
  }
  *param_1 = uVar1;
  return;
}




// Function: assign_string_n @ 0x23c20

void assign_string_n(undefined8 *param_1,char *param_2,size_t param_3)

{
  size_t __n;
  void *pvVar1;
  
  free((void *)*param_1);
  if (param_2 != (char *)0x0) {
    __n = strnlen(param_2,param_3);
    pvVar1 = (void *)FUN_0014c5b0(__n + 1);
    pvVar1 = memcpy(pvVar1,param_2,__n);
    *(undefined1 *)((long)pvVar1 + __n) = 0;
    *param_1 = pvVar1;
  }
  return;
}




// Function: unquote_string @ 0x23c70

undefined8 unquote_string(char *param_1)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  undefined8 uVar5;
  
  cVar2 = *param_1;
  if (cVar2 == '\0') {
    return 1;
  }
  uVar5 = 1;
  pcVar3 = param_1;
  do {
    while( true ) {
      pcVar4 = pcVar3 + 1;
      pcVar1 = param_1 + 1;
      if (cVar2 != '\\') break;
      switch(pcVar3[1]) {
      case '0':
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
        cVar2 = pcVar3[1] + -0x30;
        if ((byte)(pcVar3[2] - 0x30U) < 8) {
          cVar2 = pcVar3[2] + -0x30 + cVar2 * '\b';
          if ((byte)(pcVar3[3] - 0x30U) < 8) {
            pcVar4 = pcVar3 + 4;
            *param_1 = pcVar3[3] + -0x30 + cVar2 * '\b';
            cVar2 = *pcVar4;
          }
          else {
            *param_1 = cVar2;
            pcVar4 = pcVar3 + 3;
            cVar2 = *pcVar4;
          }
        }
        else {
          *param_1 = cVar2;
          pcVar4 = pcVar3 + 2;
          cVar2 = *pcVar4;
        }
        break;
      default:
        *param_1 = '\\';
        if (pcVar3[1] == '\0') {
          uVar5 = 0;
          goto LAB_00123cf0;
        }
        param_1[1] = pcVar3[1];
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        pcVar1 = param_1 + 2;
        uVar5 = 0;
        break;
      case '?':
        *param_1 = '\x7f';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case '\\':
        *param_1 = '\\';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'a':
        *param_1 = '\a';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'b':
        *param_1 = '\b';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'f':
        *param_1 = '\f';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'n':
        *param_1 = '\n';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'r':
        *param_1 = '\r';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 't':
        *param_1 = '\t';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
        break;
      case 'v':
        *param_1 = '\v';
        pcVar4 = pcVar3 + 2;
        cVar2 = *pcVar4;
      }
LAB_00123ca9:
      param_1 = pcVar1;
      pcVar3 = pcVar4;
      if (cVar2 == '\0') goto LAB_00123cf0;
    }
    if (param_1 != pcVar3) {
      *param_1 = cVar2;
      cVar2 = pcVar3[1];
      goto LAB_00123ca9;
    }
    cVar2 = param_1[1];
    param_1 = pcVar1;
    pcVar3 = pcVar4;
  } while (cVar2 != '\0');
LAB_00123cf0:
  if (pcVar4 != pcVar1) {
    *pcVar1 = '\0';
  }
  return uVar5;
}




// Function: zap_slashes @ 0x23e60

char * zap_slashes(char *param_1)

{
  size_t sVar1;
  char *pcVar2;
  char *pcVar3;
  
  if ((param_1 != (char *)0x0) && (*param_1 != '\0')) {
    sVar1 = strlen(param_1);
    pcVar3 = param_1 + (sVar1 - 1);
    if (param_1 < param_1 + (sVar1 - 1)) {
      do {
        if (*pcVar3 != '/') {
          return param_1;
        }
        pcVar2 = pcVar3 + -1;
        *pcVar3 = '\0';
        pcVar3 = pcVar2;
      } while (param_1 != pcVar2);
    }
  }
  return param_1;
}




// Function: normalize_filename_x @ 0x23ea0

void normalize_filename_x(char *param_1)

{
  char *pcVar1;
  char *pcVar2;
  char cVar3;
  char cVar4;
  char *pcVar5;
  
  cVar3 = *param_1;
  pcVar2 = param_1;
  pcVar1 = param_1;
  pcVar5 = param_1;
  if (cVar3 == '.') {
    do {
      pcVar1 = pcVar2;
      if (pcVar2[1] != '/') break;
      cVar3 = pcVar2[2];
      pcVar1 = pcVar2 + 2;
      if (cVar3 == '/') {
        pcVar1 = pcVar2 + 3;
        cVar3 = pcVar2[3];
        while (cVar3 == '/') {
          pcVar2 = pcVar1 + 1;
          pcVar1 = pcVar1 + 1;
          cVar3 = *pcVar2;
        }
      }
      pcVar5 = pcVar5 + (cVar3 == '\0');
      *pcVar5 = cVar3;
      pcVar2 = pcVar1;
    } while (cVar3 == '.');
    cVar3 = *pcVar1;
  }
  *pcVar5 = cVar3;
  while (pcVar2 = pcVar5 + 1, pcVar5 = pcVar2, cVar4 = cVar3, cVar3 != '\0') {
    while( true ) {
      pcVar1 = pcVar1 + 1;
      cVar3 = *pcVar1;
      if ((cVar4 == '/') && (pcVar1[cVar3 == '.'] == '/')) break;
      pcVar2 = pcVar5 + 1;
      *pcVar5 = cVar3;
      pcVar5 = pcVar2;
      cVar4 = cVar3;
      if (cVar3 == '\0') goto LAB_00123f80;
    }
    do {
      pcVar1 = pcVar1 + (ulong)(cVar3 == '.') + 1;
      cVar3 = *pcVar1;
    } while (pcVar1[cVar3 == '.'] == '/');
    *pcVar5 = cVar3;
  }
LAB_00123f80:
  if ((long)pcVar2 - (long)param_1 < 3) {
    return;
  }
  if (pcVar2[-2] == '.') {
    if ((pcVar2[-3] != '/') || (pcVar2 = pcVar2 + -1, (long)pcVar2 - (long)param_1 < 3))
    goto LAB_00123fa0;
  }
  else if (pcVar2[-2] != '/') goto LAB_00123fa0;
  pcVar2 = pcVar2 + -1;
LAB_00123fa0:
  pcVar2[-1] = '\0';
  return;
}




// Function: replace_prefix @ 0x23fc0

void replace_prefix(undefined8 *param_1,void *param_2,ulong param_3,void *param_4,ulong param_5)

{
  int iVar1;
  size_t sVar2;
  char *__s;
  char *__src;
  
  __s = (char *)*param_1;
  sVar2 = strlen(__s);
  if (param_3 < sVar2) {
    iVar1 = memcmp(__s,param_2,param_3);
    if ((iVar1 == 0) && (__src = __s + param_3, *__src == '/')) {
      if (param_3 < param_5) {
        __s = (char *)FUN_0014c610(__s,(sVar2 - param_3) + 1 + param_5);
        *param_1 = __s;
        __src = __s + param_3;
      }
      memmove(__s + param_5,__src,(sVar2 - param_3) + 1);
      memcpy(__s,param_4,param_5);
      return;
    }
  }
  return;
}




// Function: sysinttostr @ 0x24080

void sysinttostr(ulong param_1,undefined8 param_2,ulong param_3,undefined8 param_4)

{
  if (param_1 <= param_3) {
    FUN_00143090();
    return;
  }
  FUN_00142f50(param_1,param_4);
  return;
}




// Function: strtosysint @ 0x240a0

ulong strtosysint(char *param_1,char **param_2,ulong param_3,ulong param_4)

{
  int *piVar1;
  uintmax_t uVar2;
  ulong uVar3;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  if ((long)param_4 < 0) {
    if ((int)*param_1 - 0x30U < 10) {
      uVar2 = strtoumax(param_1,param_2,10);
      if (param_4 < uVar2) {
        *piVar1 = 0x22;
        return param_4;
      }
      uVar3 = FUN_00123b90(uVar2);
      return uVar3;
    }
  }
  else if ((int)param_1[*param_1 == '-'] - 0x30U < 10) {
    uVar3 = strtoimax(param_1,param_2,10);
    if (((long)param_3 <= (long)uVar3) && ((long)uVar3 <= (long)param_4)) {
      return uVar3;
    }
    *piVar1 = 0x22;
    if ((long)param_3 <= (long)uVar3) {
      return param_4;
    }
    return param_3;
  }
  *piVar1 = 0x16;
  return 0;
}




// Function: code_ns_fraction @ 0x24170

void code_ns_fraction(int param_1,undefined1 *param_2)

{
  long lVar1;
  long lVar2;
  int iVar3;
  int iVar4;
  
  if (param_1 == 0) {
    *param_2 = 0;
    return;
  }
  lVar1 = (long)param_1;
  *param_2 = 0x2e;
  if (param_1 % 10 == 0) {
    iVar3 = 9;
    do {
      iVar4 = iVar3;
      iVar3 = iVar4 + -1;
      param_1 = (int)(lVar1 * 0x66666667 >> 0x22) - (param_1 >> 0x1f);
      lVar1 = (long)param_1;
    } while (param_1 % 10 == 0);
    iVar4 = iVar4 + -2;
    lVar2 = (long)iVar4;
    param_2[(long)iVar3 + 1] = 0;
    param_2[lVar2 + 1] = (char)(param_1 % 10) + '0';
    if (iVar4 == 0) {
      return;
    }
  }
  else {
    param_2[10] = 0;
    lVar2 = 8;
    param_2[9] = (char)(param_1 % 10) + '0';
  }
  do {
    param_1 = (int)(lVar1 * 0x66666667 >> 0x22) - (param_1 >> 0x1f);
    lVar1 = (long)param_1;
    param_2[lVar2] = (char)param_1 + (char)(param_1 / 10) * -10 + '0';
    lVar2 = lVar2 + -1;
  } while ((int)lVar2 != 0);
  return;
}




// Function: code_timespec @ 0x24270

long code_timespec(long param_1,uint param_2,long param_3)

{
  long lVar1;
  
  if (param_2 < 1000000000) {
    if ((param_2 == 0) || (-1 < param_1)) goto LAB_001242d2;
    param_1 = param_1 + 1;
    param_2 = 1000000000 - param_2;
  }
  else {
    param_2 = 0;
LAB_001242d2:
    if (-1 < param_1) {
      lVar1 = FUN_00143090(param_1,param_3 + 1);
      goto LAB_001242b8;
    }
  }
  lVar1 = FUN_00143090(-param_1,param_3 + 1);
  *(undefined1 *)(lVar1 + -1) = 0x2d;
  lVar1 = lVar1 + -1;
LAB_001242b8:
  FUN_00124170(param_2,param_3 + 0x15);
  return lVar1;
}




// Function: decode_timespec @ 0x242f0

undefined1  [16] decode_timespec(char *param_1,char **param_2,char param_3)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  uintmax_t uVar5;
  long lVar6;
  char *pcVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  
  cVar1 = *param_1;
  piVar4 = __errno_location();
  if (9 < (int)param_1[cVar1 == '-'] - 0x30U) {
    *piVar4 = 0x16;
    *param_2 = param_1;
    auVar11._8_8_ = 0xffffffffffffffff;
    auVar11._0_8_ = 0x8000000000000000;
    return auVar11;
  }
  *piVar4 = 0;
  if (cVar1 == '-') {
    uVar5 = strtoimax(param_1,param_2,10);
    pcVar7 = *param_2;
    iVar10 = *piVar4;
joined_r0x00124474:
    iVar3 = 0;
    if ((param_3 == '\0') || (iVar3 = 0, *pcVar7 != '.')) goto LAB_00124403;
  }
  else {
    uVar5 = strtoumax(param_1,param_2,10);
    if (-1 < (long)uVar5) {
      pcVar7 = *param_2;
      iVar10 = *piVar4;
      goto joined_r0x00124474;
    }
    *piVar4 = 0x22;
    pcVar7 = *param_2;
    if ((param_3 == '\0') || (*pcVar7 != '.')) {
      *param_2 = pcVar7;
      auVar13._8_8_ = 0xffffffffffffffff;
      auVar13._0_8_ = 0x8000000000000000;
      return auVar13;
    }
    iVar10 = 0x22;
    uVar5 = 0x8000000000000000;
  }
  iVar8 = (int)pcVar7[1];
  pcVar7 = pcVar7 + 1;
  if (iVar8 - 0x30U < 10) {
    uVar9 = 0;
    iVar2 = 0;
    iVar3 = 0;
    do {
      if (iVar2 < 9) {
        iVar2 = iVar2 + 1;
        iVar3 = iVar8 + -0x30 + iVar3 * 10;
      }
      else {
        uVar9 = uVar9 | (char)iVar8 != '0';
      }
      iVar8 = (int)pcVar7[1];
      pcVar7 = pcVar7 + 1;
    } while (iVar8 - 0x30U < 10);
    if (iVar2 < 9) goto LAB_001243f0;
  }
  else {
    iVar3 = 0;
    uVar9 = 0;
    iVar2 = 0;
LAB_001243f0:
    do {
      iVar2 = iVar2 + 1;
      iVar3 = iVar3 * 10;
    } while (iVar2 != 9);
  }
  if (cVar1 == '-') {
    iVar8 = iVar3 + uVar9;
    iVar3 = 0;
    if (iVar8 != 0) {
      if (uVar5 == 0x8000000000000000) {
        iVar3 = -1;
      }
      else {
        uVar5 = uVar5 - 1;
        iVar3 = 1000000000 - iVar8;
      }
    }
  }
LAB_00124403:
  lVar6 = (long)iVar3;
  *param_2 = pcVar7;
  if (iVar10 == 0x22) {
    lVar6 = -1;
  }
  auVar12._8_8_ = lVar6;
  auVar12._0_8_ = uVar5;
  return auVar12;
}




// Function: must_be_dot_or_slash @ 0x244e0

bool must_be_dot_or_slash(char *param_1)

{
  char cVar1;
  
  cVar1 = *param_1;
  if (cVar1 == '/') {
    while( true ) {
      for (; cVar1 = param_1[1], cVar1 == '/'; param_1 = param_1 + 1) {
      }
      if (cVar1 != '.') break;
      param_1 = param_1 + (ulong)(param_1[2] == '.') + 2;
      if (*param_1 != '/') {
        return false;
      }
    }
  }
  else {
    while (cVar1 == '.') {
      while( true ) {
        cVar1 = param_1[1];
        if (cVar1 != '/') goto LAB_00124520;
        cVar1 = param_1[2];
        param_1 = param_1 + 2;
        if (cVar1 != '/') break;
        do {
          cVar1 = param_1[1];
          param_1 = param_1 + 1;
        } while (cVar1 == '/');
        if (cVar1 != '.') goto LAB_00124520;
      }
    }
  }
LAB_00124520:
  return cVar1 == '\0';
}




// Function: safer_rmdir @ 0x24560

undefined4 safer_rmdir(char *param_1)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined4 uVar4;
  
  cVar1 = FUN_001244e0();
  if (cVar1 == '\0') {
    iVar2 = unlinkat(DAT_00181234,param_1,0x200);
    if (iVar2 == 0) {
      FUN_00119b90(param_1);
      uVar4 = 0;
    }
    else {
      uVar4 = 0xffffffff;
    }
  }
  else {
    piVar3 = __errno_location();
    uVar4 = 0xffffffff;
    *piVar3 = 0;
  }
  return uVar4;
}




// Function: undo_last_backup @ 0x245d0

void undo_last_backup(void)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  if (DAT_001845f8 != (char *)0x0) {
    iVar1 = renameat(DAT_00181234,DAT_001845f8,DAT_00181234,DAT_00184600);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = FUN_0014ac40(1,DAT_00184600);
      uVar4 = FUN_0014aa70(DAT_001845f8);
      uVar5 = dcgettext(0,"%s: Cannot rename to %s",5);
      error(0,iVar1,uVar5,uVar4,uVar3);
      DAT_00184d58 = 2;
    }
    if (DAT_00184a54 != 0) {
      uVar3 = FUN_0014ac40(1,DAT_00184600);
      uVar4 = FUN_0014ac40(0,DAT_001845f8);
      uVar5 = dcgettext(0,"Renaming %s back to %s\n",5);
      __fprintf_chk(DAT_00183f28,1,uVar5,uVar4,uVar3);
    }
    FUN_00123be0(&DAT_001845f8,0);
    return;
  }
  return;
}




// Function: deref_stat @ 0x246f0

void deref_stat(char *param_1,stat *param_2)

{
  fstatat(DAT_00181234,param_1,param_2,DAT_00184800);
  return;
}




// Function: maybe_backup_file @ 0x24710

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 maybe_backup_file(char *param_1,char param_2)

{
  int iVar1;
  void *pvVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 auStack_c8 [24];
  uint local_b0;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00123be0(&DAT_00184600,param_1);
  FUN_00123be0(&DAT_001845f8,0);
  if ((((param_2 == '\0') || (DAT_00184d70 != '\0')) ||
      (_DAT_00184d78 = strchr(param_1,0x3a), _DAT_00184d78 == (char *)0x0)) ||
     ((_DAT_00184d78 <= param_1 ||
      (pvVar2 = memchr(param_1,0x2f,(long)_DAT_00184d78 - (long)param_1), pvVar2 != (void *)0x0))))
  {
    iVar1 = FUN_001246f0(param_1,auStack_c8);
    if (iVar1 == 0) {
      if (((local_b0 & 0xf000) != 0x4000) && ((param_2 == '\0' || ((local_b0 & 0xb000) != 0x2000))))
      {
        DAT_001845f8 = (char *)FUN_0013e2f0(DAT_00181234,param_1,DAT_00184b80);
        if (DAT_001845f8 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_00116420();
        }
        iVar1 = renameat(DAT_00181234,DAT_00184600,DAT_00181234,DAT_001845f8);
        if (iVar1 != 0) {
          piVar3 = __errno_location();
          iVar1 = *piVar3;
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar6 = FUN_0014ac40(1,DAT_001845f8);
          uVar4 = FUN_0014aa70(DAT_00184600);
          uVar5 = dcgettext(0,"%s: Cannot rename to %s",5);
          error(0,iVar1,uVar5,uVar4,uVar6);
          DAT_00184d58 = 2;
          FUN_00123be0(&DAT_001845f8,0);
          uVar6 = 0;
          goto LAB_001247c7;
        }
        if (DAT_00184a54 != 0) {
          uVar6 = FUN_0014ac40(1,DAT_001845f8);
          uVar4 = FUN_0014ac40(0,DAT_00184600);
          uVar5 = dcgettext(0,"Renaming %s to %s\n",5);
          __fprintf_chk(DAT_00183f28,1,uVar5,uVar4,uVar6);
        }
      }
    }
    else {
      piVar3 = __errno_location();
      if (*piVar3 != 2) {
        FUN_00132d30(param_1);
        uVar6 = 0;
        goto LAB_001247c7;
      }
    }
  }
  uVar6 = 1;
LAB_001247c7:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: blocking_read @ 0x24960

long blocking_read(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  uint uVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  
  lVar3 = FUN_0014af10();
  if (lVar3 == -1) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_0014d180(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_0014d180(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          lVar3 = FUN_0014af10(param_1,param_2,param_3);
          return lVar3;
        }
      }
    }
  }
  return lVar3;
}




// Function: blocking_write @ 0x249f0

ulong blocking_write(undefined4 param_1,long param_2,ulong param_3)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  int *piVar4;
  long lVar5;
  
  uVar3 = FUN_00141120();
  if (uVar3 < param_3) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_0014d180(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_0014d180(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          lVar5 = FUN_00141120(param_1,param_2 + uVar3,param_3 - uVar3);
          uVar3 = uVar3 + lVar5;
        }
      }
    }
  }
  return uVar3;
}




// Function: set_file_atime @ 0x24a80

void set_file_atime(void)

{
  long lVar1;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0013f360();
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: chdir_count @ 0x24ae0

int chdir_count(void)

{
  int iVar1;
  
  iVar1 = (int)DAT_001845e8 + -1;
  if (DAT_001845e8 == 0) {
    iVar1 = 0;
  }
  return iVar1;
}




// Function: chdir_arg @ 0x24b00

ulong chdir_arg(char *param_1)

{
  char *pcVar1;
  long *plVar2;
  char cVar3;
  undefined8 *puVar4;
  ulong uVar5;
  
  uVar5 = DAT_001845e8;
  if (DAT_001845e8 == DAT_001845e0) {
    if (DAT_001845e8 == 0) {
      DAT_001845e0 = 2;
    }
    puVar4 = (undefined8 *)FUN_0014c680(DAT_001845f0,&DAT_001845e0,0x18);
    uVar5 = DAT_001845e8;
    DAT_001845f0 = puVar4;
    if (DAT_001845e8 == 0) {
      puVar4[1] = 0;
      *puVar4 = ".";
      uVar5 = 1;
      *(undefined4 *)(puVar4 + 2) = 0xffffff9c;
    }
  }
  if (*param_1 == '.') {
    do {
      while( true ) {
        if (param_1[1] != '/') {
          cVar3 = param_1[1];
          goto joined_r0x00124c19;
        }
        pcVar1 = param_1 + 2;
        param_1 = param_1 + 2;
        if (*pcVar1 == '/') break;
        if (*pcVar1 != '.') goto LAB_00124b87;
      }
      do {
        pcVar1 = param_1 + 1;
        param_1 = param_1 + 1;
      } while (*pcVar1 == '/');
    } while (*pcVar1 == '.');
LAB_00124b87:
    cVar3 = *param_1;
joined_r0x00124c19:
    if (cVar3 == '\0') {
      DAT_001845e8 = uVar5;
      return (ulong)((int)uVar5 - 1);
    }
  }
  plVar2 = DAT_001845f0 + uVar5 * 3;
  *plVar2 = (long)param_1;
  plVar2[1] = 0;
  *(undefined4 *)(plVar2 + 2) = 0;
  DAT_001845e8 = uVar5 + 1;
  return uVar5;
}




// Function: tar_dirname @ 0x24c30

undefined8 tar_dirname(void)

{
  return *(undefined8 *)(DAT_001845f0 + (long)DAT_00184580 * 0x18);
}




// Function: close_diag @ 0x24c50

void close_diag(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132680();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_001326a0();
  return;
}




// Function: chdir_do @ 0x24c80

void chdir_do(int param_1)

{
  undefined8 *puVar1;
  int iVar2;
  ulong uVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  char *pcVar7;
  
  if (DAT_00184580 == param_1) {
    return;
  }
  puVar1 = (undefined8 *)(DAT_001845f0 + (long)param_1 * 0x18);
  iVar4 = *(int *)(puVar1 + 2);
  iVar5 = DAT_001845a0;
  if (iVar4 == 0) {
    pcVar7 = (char *)*puVar1;
    if (*pcVar7 != '/') {
      chdir_do(param_1 + -1);
      pcVar7 = (char *)*puVar1;
    }
    iVar4 = __openat_2(DAT_00181234,pcVar7,DAT_00184804 & 0xfffdffff);
    uVar3 = DAT_00184590;
    if (iVar4 < 0) {
      FUN_001327e0(*puVar1);
      if (DAT_00184b52 == '\0') {
        FUN_001327c0();
        return;
      }
      if ((DAT_00183b80._2_1_ & 0x80) != 0) {
        FUN_00132800();
        return;
      }
      return;
    }
    *(int *)(puVar1 + 2) = iVar4;
    if (uVar3 < 0x10) {
      DAT_00184590 = uVar3 + 1;
      (&DAT_001845a0)[uVar3] = param_1;
      iVar5 = DAT_001845a0;
    }
    else {
      puVar1 = (undefined8 *)(DAT_001845f0 + (long)DAT_001845dc * 0x18);
      iVar5 = close(*(int *)(puVar1 + 2));
      if (iVar5 != 0) {
        FUN_00124c50(*puVar1);
      }
      *(undefined4 *)(puVar1 + 2) = 0;
      iVar5 = DAT_001845a0;
      DAT_001845dc = param_1;
    }
  }
  DAT_001845a0 = iVar5;
  if ((0 < iVar4) && (DAT_001845a0 = param_1, param_1 != iVar5)) {
    piVar6 = &DAT_001845a4;
    do {
      iVar2 = *piVar6;
      *piVar6 = iVar5;
      piVar6 = piVar6 + 1;
      iVar5 = iVar2;
    } while (param_1 != iVar2);
  }
  DAT_00184580 = param_1;
  DAT_00181234 = iVar4;
  return;
}




// Function: open_diag @ 0x24dd0

void open_diag(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_001327c0();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132800();
  return;
}




// Function: read_diag_details @ 0x24e00

void read_diag_details(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132840();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132900();
  return;
}




// Function: readlink_diag @ 0x24e30

void readlink_diag(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132a60();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132a80();
  return;
}




// Function: savedir_diag @ 0x24e60

void savedir_diag(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132ac0();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132ae0();
  return;
}




// Function: seek_diag_details @ 0x24e90

void seek_diag_details(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132b20();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132bf0();
  return;
}




// Function: stat_diag @ 0x24ec0

void stat_diag(void)

{
  if (DAT_00184b52 == '\0') {
    FUN_00132d30();
    return;
  }
  if ((DAT_00183b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132d50();
  return;
}




// Function: file_removed_diag @ 0x24ef0

void file_removed_diag(undefined8 param_1,char param_2,code *UNRECOVERED_JUMPTABLE)

{
  int *piVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (param_2 == '\0') {
    piVar1 = __errno_location();
    if (*piVar1 == 2) {
      if (((byte)DAT_00183b80 & 0x40) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar2 = FUN_0014aa70(param_1);
        uVar3 = dcgettext(0,"%s: File removed before we read it",5);
        error(0,0,uVar3,uVar2);
      }
      FUN_0012f7d0(1);
      return;
    }
  }
                    /* WARNING: Could not recover jumptable at 0x00124f17. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*UNRECOVERED_JUMPTABLE)(param_1);
  return;
}




// Function: xfork @ 0x24f80

void xfork(void)

{
  undefined8 uVar1;
  __pid_t _Var2;
  
  _Var2 = fork();
  if (_Var2 != -1) {
    return;
  }
  uVar1 = dcgettext(0,"child process",5);
                    /* WARNING: Subroutine does not return */
  FUN_00132490(&DAT_00162779,uVar1);
}




// Function: xpipe @ 0x24fc0

void xpipe(int *param_1)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = pipe(param_1);
  if (-1 < iVar2) {
    return;
  }
  uVar1 = dcgettext(0,"interprocess channel",5);
                    /* WARNING: Subroutine does not return */
  FUN_00132490(&DAT_00162793,uVar1);
}




// Function: page_aligned_alloc @ 0x25000

void page_aligned_alloc(undefined8 *param_1,ulong param_2)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  
  iVar1 = getpagesize();
  uVar3 = (ulong)iVar1;
  if (!CARRY8(param_2,uVar3)) {
    uVar2 = FUN_0014c5b0(param_2 + uVar3);
    *param_1 = uVar2;
    FUN_00123b40(uVar2,uVar3);
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00116420();
}




// Function: namebuf_create @ 0x25040

undefined8 * namebuf_create(char *param_1)

{
  char cVar1;
  undefined8 *puVar2;
  size_t sVar3;
  char *__dest;
  
  puVar2 = (undefined8 *)FUN_0014c5b0(0x18);
  sVar3 = strlen(param_1);
  puVar2[1] = sVar3 + 2;
  __dest = (char *)FUN_0014c5b0();
  *puVar2 = __dest;
  strcpy(__dest,param_1);
  sVar3 = strlen(__dest);
  cVar1 = __dest[sVar3 - 1];
  puVar2[2] = sVar3;
  if (cVar1 != '/') {
    __dest[sVar3] = '/';
    puVar2[2] = sVar3 + 1;
  }
  return puVar2;
}




// Function: namebuf_free @ 0x250b0

void namebuf_free(undefined8 *param_1)

{
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: namebuf_name @ 0x250d0

long namebuf_name(long *param_1,char *param_2)

{
  size_t sVar1;
  long lVar2;
  long lVar3;
  
  sVar1 = strlen(param_2);
  lVar3 = param_1[2];
  if (lVar3 + 1 + sVar1 < (ulong)param_1[1]) {
    lVar2 = *param_1;
  }
  else {
    lVar2 = *param_1;
    do {
      lVar2 = FUN_0014c710(lVar2,param_1 + 1);
      *param_1 = lVar2;
      lVar3 = param_1[2];
    } while ((ulong)param_1[1] <= lVar3 + 1 + sVar1);
  }
  strcpy((char *)(lVar2 + lVar3),param_2);
  return *param_1;
}




// Function: namebuf_add_dir @ 0x25150

void namebuf_add_dir(long *param_1,char *param_2)

{
  size_t sVar1;
  
  if (*(char *)(*param_1 + -1 + param_1[2]) != '/') {
    FUN_001250d0(param_1,&DAT_00181230);
    param_1[2] = param_1[2] + 1;
  }
  FUN_001250d0(param_1,param_2);
  sVar1 = strlen(param_2);
  param_1[2] = param_1[2] + sVar1;
  return;
}




// Function: tar_getcdpath @ 0x251a0

long tar_getcdpath(int param_1)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  int iVar3;
  long *plVar4;
  long lVar5;
  undefined8 uVar6;
  int iVar7;
  long lVar8;
  long lVar9;
  
  uVar2 = DAT_00184580;
  if (DAT_001845f0 == 0) {
    lVar8 = DAT_00184588;
    if ((DAT_00184588 == 0) && (lVar8 = FUN_0014c7c0(), DAT_00184588 = lVar8, lVar8 == 0)) {
LAB_00125275:
                    /* WARNING: Subroutine does not return */
      FUN_00132490("getcwd",".");
    }
  }
  else {
    lVar9 = (long)param_1 * 0x18;
    lVar8 = *(long *)(DAT_001845f0 + 8 + lVar9);
    if (lVar8 == 0) {
      if (param_1 < 0) {
        iVar7 = param_1 + 1;
      }
      else {
        plVar4 = (long *)(DAT_001845f0 + -0x10 + lVar9);
        iVar3 = param_1;
        do {
          iVar7 = iVar3;
          if (iVar7 == 0) goto LAB_001252cc;
          lVar8 = *plVar4;
          plVar4 = plVar4 + -3;
          iVar3 = iVar7 + -1;
        } while (lVar8 == 0);
      }
      while (iVar7 <= param_1) {
LAB_001252cc:
        FUN_00124c80(iVar7);
        lVar8 = DAT_001845f0;
        if (iVar7 == 0) {
          lVar5 = FUN_0014c7c0();
          *(long *)(lVar8 + 8) = lVar5;
          if (lVar5 == 0) goto LAB_00125275;
          if (param_1 < 1) break;
          iVar7 = 1;
          FUN_00124c80(1);
        }
        lVar5 = (long)iVar7;
        iVar7 = iVar7 + 1;
        lVar8 = lVar5 * 0x18;
        puVar1 = (undefined8 *)(DAT_001845f0 + lVar8);
        if (*(char *)*puVar1 == '/') {
          uVar6 = FUN_0014c7a0();
          puVar1[1] = uVar6;
        }
        else {
          uVar6 = FUN_00125040(*(undefined8 *)(DAT_001845f0 + -0x10 + lVar8));
          FUN_00125150(uVar6,*(undefined8 *)(DAT_001845f0 + lVar5 * 0x18));
          lVar8 = lVar8 + DAT_001845f0;
          uVar6 = FUN_00123b60(uVar6);
          *(undefined8 *)(lVar8 + 8) = uVar6;
        }
      }
      FUN_00124c80(uVar2);
      return *(long *)(DAT_001845f0 + 8 + lVar9);
    }
  }
  return lVar8;
}




// Function: normalize_filename @ 0x25340

char * normalize_filename(undefined8 param_1,char *param_2)

{
  char *__s;
  size_t sVar1;
  size_t sVar2;
  char *__dest;
  
  if (*param_2 == '/') {
    __dest = (char *)FUN_0014c7a0(param_2);
  }
  else {
    __s = (char *)FUN_001251a0();
    sVar1 = strlen(__s);
    sVar2 = strlen(param_2);
    __dest = (char *)FUN_0014c5b0(sVar1 + 2 + sVar2);
    strcpy(__dest,__s);
    __dest[sVar1] = '/';
    strcpy(__dest + sVar1 + 1,param_2);
  }
  FUN_00123ea0(__dest);
  return __dest;
}




// Function: tar_savedir @ 0x253c0

long tar_savedir(undefined8 param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  DIR *__dirp;
  long lVar3;
  
  iVar1 = __openat_2(DAT_00181234,param_1,DAT_00184808 | 0x10000);
  if (iVar1 < 0) {
    if ((param_2 != 0) || (piVar2 = __errno_location(), *piVar2 != 2)) {
      FUN_001327c0(param_1);
    }
  }
  else {
    __dirp = fdopendir(iVar1);
    if (__dirp != (DIR *)0x0) {
      lVar3 = FUN_0014b0d0(__dirp,DAT_001847e4);
      if (lVar3 == 0) {
        FUN_00132ac0(param_1);
      }
      iVar1 = closedir(__dirp);
      if (iVar1 == 0) {
        return lVar3;
      }
      FUN_00132ac0(param_1);
      return lVar3;
    }
    FUN_00132ac0(param_1);
    iVar1 = close(iVar1);
    if (iVar1 != 0) {
      FUN_00132ac0(param_1);
      return 0;
    }
  }
  return 0;
}




// Function: remove_any_file @ 0x25490

uint remove_any_file(char *param_1,int param_2)

{
  char *__s;
  uint uVar1;
  int iVar2;
  uint *puVar3;
  char *__s_00;
  size_t sVar4;
  void *__ptr;
  
  iVar2 = unlinkat(DAT_00181234,param_1,0);
  if (iVar2 == 0) {
    return 1;
  }
  puVar3 = (uint *)__errno_location();
  if ((*puVar3 == 1) || (*puVar3 == 0x15)) {
    iVar2 = FUN_00124560(param_1);
    if (iVar2 == 0) {
      return 1;
    }
    if (0x27 < *puVar3) {
      return 0;
    }
    if ((0x8000020001U >> ((ulong)*puVar3 & 0x3f) & 1) == 0) {
      return 0;
    }
    if (param_2 != 1) {
      return -(uint)(param_2 == 2);
    }
    __s_00 = (char *)FUN_001253c0(param_1,0);
    if (__s_00 != (char *)0x0) {
      sVar4 = strlen(__s_00);
      __s = __s_00;
      while( true ) {
        if (sVar4 == 0) {
          free(__s_00);
          iVar2 = FUN_00124560(param_1);
          return (uint)(iVar2 == 0);
        }
        __ptr = (void *)FUN_00127bc0(param_1,__s);
        iVar2 = remove_any_file(__ptr,1);
        uVar1 = *puVar3;
        free(__ptr);
        if (iVar2 == 0) break;
        __s = __s + sVar4 + 1;
        sVar4 = strlen(__s);
      }
      free(__s_00);
      *puVar3 = uVar1;
      return 0;
    }
  }
  return 0;
}




// Function: file_selection_option @ 0x255e0

long * file_selection_option(int param_1)

{
  long *plVar1;
  
  for (plVar1 = &DAT_00181280; ((*plVar1 != 0 || ((int)plVar1[1] != 0)) || (plVar1[4] != 0));
      plVar1 = plVar1 + 6) {
    if (param_1 == (int)plVar1[1]) {
      return plVar1;
    }
  }
  return (long *)0x0;
}




// Function: file_selection_option_name @ 0x25610

undefined8 file_selection_option_name(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = (undefined8 *)FUN_001255e0();
  uVar2 = 0;
  if (puVar1 != (undefined8 *)0x0) {
    uVar2 = *puVar1;
  }
  return uVar2;
}




// Function: is_file_selection_option @ 0x25620

bool is_file_selection_option(void)

{
  long lVar1;
  
  lVar1 = FUN_001255e0();
  return lVar1 != 0;
}




// Function: name_list_adjust @ 0x25630

void name_list_adjust(void)

{
  long lVar1;
  long lVar2;
  
  if (DAT_001846b0 != 0) {
    lVar2 = *(long *)(DAT_001846b0 + 8);
    while (lVar1 = lVar2, lVar1 != 0) {
      DAT_001846b0 = lVar1;
      lVar2 = *(long *)(lVar1 + 8);
    }
  }
  return;
}




// Function: unconsumed_option_push @ 0x25660

void unconsumed_option_push(undefined8 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_00184628;
  if (DAT_00184628 != (undefined8 *)0x0) {
    *DAT_00184628 = param_1;
  }
  DAT_00184628 = param_1;
  return;
}




// Function: merge_sort_sll @ 0x25690

undefined8 * merge_sort_sll(undefined8 *param_1,int param_2,code *param_3)

{
  undefined8 uVar1;
  undefined8 **ppuVar2;
  undefined8 **ppuVar3;
  int iVar4;
  undefined8 **ppuVar5;
  undefined8 **ppuVar6;
  undefined8 **ppuVar7;
  undefined8 *puVar8;
  int iVar9;
  long in_FS_OFFSET;
  undefined8 *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  puVar8 = param_1;
  if (param_2 != 1) {
    if (param_2 == 2) {
      iVar4 = (*param_3)(param_1,*param_1);
      if (0 < iVar4) {
        puVar8 = (undefined8 *)*param_1;
        *puVar8 = param_1;
        *param_1 = 0;
      }
    }
    else {
      iVar9 = (param_2 + 1) / 2;
      iVar4 = iVar9;
      while (iVar4 = iVar4 + -1, iVar4 != 0) {
        puVar8 = (undefined8 *)*puVar8;
      }
      uVar1 = *puVar8;
      *puVar8 = 0;
      ppuVar5 = (undefined8 **)merge_sort_sll(param_1,iVar9,param_3);
      ppuVar6 = (undefined8 **)merge_sort_sll(uVar1,param_2 / 2,param_3);
      if ((ppuVar5 == (undefined8 **)0x0) || (ppuVar6 == (undefined8 **)0x0)) {
        ppuVar7 = &local_38;
      }
      else {
        ppuVar7 = &local_38;
        do {
          iVar4 = (*param_3)(ppuVar5,ppuVar6);
          if (iVar4 < 0) {
            ppuVar3 = (undefined8 **)*ppuVar5;
            *ppuVar7 = ppuVar5;
            ppuVar2 = ppuVar6;
            ppuVar7 = ppuVar5;
            ppuVar5 = ppuVar3;
          }
          else {
            ppuVar2 = (undefined8 **)*ppuVar6;
            *ppuVar7 = ppuVar6;
            ppuVar7 = ppuVar6;
          }
          ppuVar6 = ppuVar2;
        } while ((ppuVar5 != (undefined8 **)0x0) && (ppuVar6 != (undefined8 **)0x0));
      }
      if (ppuVar5 == (undefined8 **)0x0) {
        ppuVar5 = ppuVar6;
      }
      *ppuVar7 = ppuVar5;
      puVar8 = local_38;
    }
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return puVar8;
}




// Function: merge_sort @ 0x25800

void merge_sort(void)

{
  long *plVar1;
  long *plVar2;
  long *plVar3;
  
  plVar1 = (long *)FUN_00125690();
  if (plVar1 != (long *)0x0) {
    plVar3 = (long *)0x0;
    do {
      plVar2 = plVar1;
      plVar2[1] = (long)plVar3;
      plVar1 = (long *)*plVar2;
      plVar3 = plVar2;
    } while ((long *)*plVar2 != (long *)0x0);
  }
  return;
}




// Function: file_list_name @ 0x25830

undefined8 file_list_name(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = DAT_001846b0;
  while( true ) {
    if (puVar1 == (undefined8 *)0x0) {
      uVar2 = dcgettext(0,"command line",5);
      return uVar2;
    }
    if ((*(int *)(puVar1 + 2) == 2) && (puVar1[6] != 0)) break;
    puVar1 = (undefined8 *)*puVar1;
  }
  return puVar1[3];
}




// Function: name_elt_alloc @ 0x25880

void name_elt_alloc(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_0014c5b0(0x38);
  if (DAT_001846b0 == (undefined8 *)0x0) {
    *puVar3 = 0;
    puVar3[1] = 0;
    *(undefined4 *)(puVar3 + 2) = 3;
    DAT_001846b0 = puVar3;
    puVar3 = (undefined8 *)FUN_0014c5b0(0x38);
  }
  puVar2 = DAT_001846b0;
  puVar1 = (undefined8 *)DAT_001846b0[1];
  puVar3[1] = puVar1;
  if (puVar1 != (undefined8 *)0x0) {
    *puVar1 = puVar3;
  }
  *puVar3 = puVar2;
  puVar2[1] = puVar3;
  return;
}




// Function: name_add_dir @ 0x258f0

void name_add_dir(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_00125880();
  *(undefined4 *)(lVar1 + 0x10) = 1;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  return;
}




// Function: name_add_file @ 0x25910

void name_add_file(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_00125880();
  DAT_00184630 = 2;
  *(undefined4 *)(lVar1 + 0x10) = 2;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  *(undefined8 *)(lVar1 + 0x20) = 0;
  *(undefined8 *)(lVar1 + 0x30) = 0;
  return;
}




// Function: name_add_option @ 0x25940

void name_add_option(undefined4 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_00125880();
  *(undefined4 *)(lVar1 + 0x10) = 4;
  *(undefined4 *)(lVar1 + 0x18) = param_1;
  *(undefined8 *)(lVar1 + 0x20) = param_2;
  return;
}




// Function: compare_names_found @ 0x25970

void compare_names_found(long param_1,long param_2)

{
  if (DAT_00184b38 == 0) {
    if ((*(ulong *)(param_2 + 0x30) != 0) != (*(ulong *)(param_1 + 0x30) != 0)) {
      return;
    }
  }
  else if (DAT_00184b38 <= *(ulong *)(param_2 + 0x30) != DAT_00184b38 <= *(ulong *)(param_1 + 0x30))
  {
    return;
  }
  strcmp(*(char **)(param_1 + 0x10),*(char **)(param_2 + 0x10));
  return;
}




// Function: name_compare @ 0x259d0

bool name_compare(long param_1,long param_2)

{
  int iVar1;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x58),*(char **)(param_2 + 0x58));
  return iVar1 == 0;
}




// Function: compare_names @ 0x259f0

void compare_names(long param_1,long param_2)

{
  strcmp(*(char **)(param_1 + 0x10),*(char **)(param_2 + 0x10));
  return;
}




// Function: unconsumed_option_free @ 0x25a10

void unconsumed_option_free(void)

{
  void *__ptr;
  
  if (DAT_00184628 == (void *)0x0) {
    return;
  }
  do {
    __ptr = DAT_00184628;
    DAT_00184628 = *(void **)((long)DAT_00184628 + 8);
    free(__ptr);
  } while (DAT_00184628 != (void *)0x0);
  return;
}




// Function: name_list_advance @ 0x25a50

void name_list_advance(void)

{
  int iVar1;
  long *__ptr;
  
  __ptr = DAT_001846b0;
  DAT_001846b0 = (long *)*DAT_001846b0;
  if (DAT_001846b0 != (long *)0x0) {
    *(undefined8 *)((long)DAT_001846b0 + 8) = 0;
  }
  iVar1 = (int)__ptr[2];
  if ((iVar1 != 4) && (iVar1 != 1)) {
    if (iVar1 == 3) {
      free(__ptr);
      return;
    }
    FUN_00125a10();
    free(__ptr);
    return;
  }
  if ((DAT_00184ba8 != 3) && (DAT_00184ba8 != 8)) {
    return;
  }
  FUN_00125660(__ptr);
  return;
}




// Function: free_name @ 0x25ad0

void free_name(void *param_1)

{
  if (param_1 != (void *)0x0) {
    free(*(void **)((long)param_1 + 0x10));
    free(*(void **)((long)param_1 + 0x58));
    free(param_1);
    return;
  }
  return;
}




// Function: add_file_id @ 0x25b00

char * add_file_id(char *param_1)

{
  undefined8 uVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  size_t sVar8;
  char *pcVar9;
  char *pcVar10;
  uint uVar11;
  long in_FS_OFFSET;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = stat(param_1,&local_c8);
  if (iVar2 != 0) {
    FUN_00132d10();
    sVar8 = strlen(param_1);
    pcVar9 = param_1 + (sVar8 - 1);
    pcVar10 = pcVar9;
    if (param_1 < pcVar9) {
      do {
        if (*pcVar10 != '/') {
          return pcVar10;
        }
        pcVar9 = pcVar10 + -1;
        *pcVar10 = '\0';
        pcVar10 = pcVar9;
      } while (param_1 != pcVar9);
    }
    return pcVar9;
  }
  uVar4 = FUN_00125830();
  puVar7 = DAT_00184698;
  do {
    if (puVar7 == (undefined8 *)0x0) {
      puVar7 = (undefined8 *)FUN_0014c5b0(0x20);
      puVar7[3] = uVar4;
      *puVar7 = DAT_00184698;
      DAT_00184698 = puVar7;
      puVar7[1] = local_c8.st_ino;
      puVar7[2] = local_c8.st_dev;
      uVar11 = 0;
LAB_00125c30:
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return (char *)(ulong)uVar11;
    }
    if ((puVar7[1] == local_c8.st_ino) && (puVar7[2] == local_c8.st_dev)) {
      uVar3 = FUN_0014a5b0(0,0x3a,1);
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar1 = puVar7[3];
      uVar5 = FUN_0014a870(0,param_1);
      uVar6 = dcgettext(0,"%s: file list requested from %s already read from %s",5);
      error(0,0,uVar6,uVar5,uVar4,uVar1);
      DAT_00184d58 = 2;
      uVar11 = 1;
      FUN_0014a5b0(0,0x3a,uVar3);
      goto LAB_00125c30;
    }
    puVar7 = (undefined8 *)*puVar7;
  } while( true );
}




// Function: chopslash @ 0x25c70

void chopslash(char *param_1)

{
  size_t sVar1;
  char *pcVar2;
  char *pcVar3;
  
  sVar1 = strlen(param_1);
  pcVar3 = param_1 + (sVar1 - 1);
  if (param_1 < param_1 + (sVar1 - 1)) {
    do {
      if (*pcVar3 != '/') {
        return;
      }
      pcVar2 = pcVar3 + -1;
      *pcVar3 = '\0';
      pcVar3 = pcVar2;
    } while (param_1 != pcVar2);
  }
  return;
}




// Function: read_name_from_file @ 0x25ca0

byte read_name_from_file(long param_1)

{
  int iVar1;
  FILE *__stream;
  int iVar2;
  byte bVar3;
  long lVar4;
  bool bVar5;
  bool bVar6;
  
  *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
  __stream = *(FILE **)(param_1 + 0x30);
  iVar1 = *(int *)(param_1 + 0x28);
  iVar2 = getc_unlocked(__stream);
  bVar5 = iVar2 != -1;
  if ((iVar1 == iVar2) || (!bVar5)) {
    bVar6 = true;
    lVar4 = 0;
  }
  else {
    lVar4 = 0;
    do {
      if (DAT_001846a0 == lVar4) {
        DAT_001846a8 = FUN_0014c710(DAT_001846a8,&DAT_001846a0);
        *(char *)(DAT_001846a8 + lVar4) = (char)iVar2;
      }
      else {
        *(char *)(DAT_001846a8 + lVar4) = (char)iVar2;
      }
      if (iVar2 == 0) {
        return 2;
      }
      lVar4 = lVar4 + 1;
      iVar2 = getc_unlocked(__stream);
    } while ((iVar1 != iVar2) && (iVar2 != -1));
    bVar6 = lVar4 == 0;
    bVar5 = iVar2 != -1 && bVar6;
  }
  bVar3 = 3;
  if (!bVar5) {
    if (DAT_001846a0 == lVar4) {
      DAT_001846a8 = FUN_0014c710(DAT_001846a8,&DAT_001846a0);
    }
    *(undefined1 *)(DAT_001846a8 + lVar4) = 0;
    FUN_00125c70();
    bVar3 = iVar2 == -1 & bVar6;
  }
  return bVar3;
}




// Function: handle_option @ 0x25dc0

undefined4 handle_option(char *param_1,long param_2)

{
  undefined8 *puVar1;
  int iVar2;
  ushort **ppuVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char cVar7;
  undefined4 uVar8;
  long in_FS_OFFSET;
  undefined4 local_138 [2];
  undefined8 local_130;
  undefined8 local_128;
  int local_118;
  undefined4 uStack_114;
  undefined8 *local_110;
  long local_108;
  long local_30;
  
  cVar7 = *param_1;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (cVar7 != '\0') {
    ppuVar3 = __ctype_b_loc();
    do {
      if ((*(byte *)((long)*ppuVar3 + (long)cVar7 * 2 + 1) & 0x20) == 0) {
        if (cVar7 == '-') {
          local_108 = 1;
          iVar2 = FUN_001382c0(param_1,&local_118,0x2000e46);
          if (iVar2 != 0) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
            }
            uVar5 = FUN_001354b0(&local_118);
            uVar6 = dcgettext(0,"cannot split string \'%s\': %s",5);
            error(0,0,uVar6,param_1,uVar5);
                    /* WARNING: Subroutine does not return */
            FUN_001163d0();
          }
          *local_110 = DAT_00184dd0;
          local_130 = *(undefined8 *)(param_2 + 0x18);
          local_138[0] = 2;
          local_128 = *(undefined8 *)(param_2 + 0x20);
          FUN_0012f5b0((int)local_108 + local_118,local_110,local_138);
          lVar4 = CONCAT44(uStack_114,local_118) + local_108;
          if (lVar4 != 0) {
            puVar1 = local_110 + lVar4;
            do {
              *local_110 = 0;
              local_110 = local_110 + 1;
            } while (puVar1 != local_110);
          }
          FUN_00135420(&local_118);
          uVar8 = 0;
          goto LAB_00125ec6;
        }
        break;
      }
      cVar7 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar7 != '\0');
  }
  uVar8 = 1;
LAB_00125ec6:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: add_exclude_array @ 0x25f40

void add_exclude_array(long *param_1,undefined4 param_2)

{
  long lVar1;
  
  lVar1 = *param_1;
  while (lVar1 != 0) {
    param_1 = param_1 + 1;
    FUN_0013ee30(DAT_00184b60,lVar1,param_2);
    lVar1 = *param_1;
  }
  return;
}




// Function: handle_file_selection_option @ 0x25f80

void handle_file_selection_option(undefined4 param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  switch(param_1) {
  case 0x58:
    iVar1 = FUN_0013f2a0(FUN_0013ee30,DAT_00184b60,param_2,
                         DAT_001846f8 | DAT_00184ad4 | (uint)(DAT_001846fc != 1) << 0x1c,10);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = FUN_0014aa70(param_2);
      error(0,iVar1,"%s",uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    break;
  default:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    error(0,0,"unhandled positional option %d",param_1);
                    /* WARNING: Subroutine does not return */
    FUN_001163d0();
  case 0x101:
    FUN_00125f40(&PTR_DAT_0017f6a0,0x10000000);
    return;
  case 0x102:
    uVar3 = 1;
    goto LAB_00126147;
  case 0x103:
    uVar3 = 2;
    goto LAB_00126147;
  case 0x104:
    uVar3 = 3;
LAB_00126147:
    FUN_00112a70("CACHEDIR.TAG",uVar3,FUN_00112b70);
    return;
  case 0x105:
    FUN_0013ee30(DAT_00184b60,param_2,
                 DAT_001846f8 | DAT_00184ad4 | (uint)(DAT_001846fc != 1) << 0x1c);
    return;
  case 0x106:
    uVar3 = 2;
    goto LAB_001260f5;
  case 0x107:
    uVar3 = 1;
LAB_001260f5:
    FUN_00116820(param_2,uVar3);
    return;
  case 0x108:
    uVar3 = 1;
    goto LAB_001260cf;
  case 0x109:
    uVar3 = 2;
    goto LAB_001260cf;
  case 0x10a:
    uVar3 = 3;
LAB_001260cf:
    FUN_00112a70(param_2,uVar3,0);
    return;
  case 0x10b:
    FUN_00125f40(&PTR_DAT_0017f6c0,0);
    return;
  case 0x10c:
    FUN_00116c40();
    return;
  case 0x10d:
    DAT_001846f8 = DAT_001846f8 | 0x10;
    break;
  case 0x10e:
    DAT_001846f8 = DAT_001846f8 & 0xffffffef;
    break;
  case 0x10f:
    DAT_001846f8 = DAT_001846f8 | 0x40000000;
    break;
  case 0x110:
    DAT_001846f8 = DAT_001846f8 & 0xbfffffff;
    DAT_00181238 = 0;
    break;
  case 0x111:
    DAT_00184ad4 = 8;
    break;
  case 0x112:
    DAT_00184ad4 = 0;
    break;
  case 0x113:
    DAT_001847e8 = 1;
    break;
  case 0x114:
    DAT_001847e8 = 0;
    break;
  case 0x115:
    DAT_00184700 = 0;
    break;
  case 0x116:
    DAT_001846f8 = DAT_001846f8 | 1;
    break;
  case 0x117:
    DAT_001846fc = 1;
    break;
  case 0x118:
    DAT_0018123c = 0;
    DAT_00184700 = 1;
    break;
  case 0x119:
    DAT_0018123c = 10;
    DAT_00184700 = 0;
    break;
  case 0x11a:
    DAT_00184700 = 1;
    break;
  case 0x11b:
    DAT_001846f8 = DAT_001846f8 & 0xfffffffe;
    break;
  case 0x11c:
    DAT_001846fc = 2;
  }
  return;
}




// Function: unconsumed_option_report @ 0x26210

void unconsumed_option_report(void)

{
  long *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long *plVar5;
  long lVar6;
  
  if (DAT_00184628 == (long *)0x0) {
    return;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = dcgettext(0,
                    "The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly."
                    ,5);
  error(0,0,uVar2);
  plVar1 = DAT_00184628;
  do {
    plVar5 = plVar1;
    plVar1 = (long *)plVar5[1];
  } while ((long *)plVar5[1] != (long *)0x0);
  do {
    while( true ) {
      DAT_00184d58 = 2;
      if ((int)plVar5[2] == 1) break;
      if ((int)plVar5[2] == 4) {
        lVar6 = plVar5[4];
        if (lVar6 == 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar2 = FUN_00125610((int)plVar5[3],"--%s has no effect");
          goto LAB_0012632e;
        }
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
          lVar6 = plVar5[4];
        }
        uVar2 = FUN_0014ac50(lVar6);
        uVar4 = FUN_00125610((int)plVar5[3],"--%s %s has no effect");
        uVar3 = dcgettext(0);
        error(0,0,uVar3,uVar4,uVar2);
      }
      plVar5 = (long *)*plVar5;
      if (plVar5 == (long *)0x0) goto LAB_00126358;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = FUN_0014ac50(plVar5[3]);
LAB_0012632e:
    uVar4 = dcgettext(0);
    error(0,0,uVar4,uVar2);
    plVar5 = (long *)*plVar5;
  } while (plVar5 != (long *)0x0);
LAB_00126358:
  DAT_00184d58 = 2;
  FUN_00125a10();
  return;
}




// Function: make_name @ 0x263a0

long make_name(undefined *param_1)

{
  long lVar1;
  char *__s;
  size_t sVar2;
  
  lVar1 = FUN_0014c760(0x60);
  if (param_1 == (undefined *)0x0) {
    param_1 = &DAT_00167708;
  }
  __s = (char *)FUN_0014c7a0(param_1);
  *(char **)(lVar1 + 0x10) = __s;
  sVar2 = strlen(__s);
  *(size_t *)(lVar1 + 0x18) = sVar2;
  return lVar1;
}




// Function: regex_usage_warning @ 0x263f0

undefined4 regex_usage_warning(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  if (DAT_001846fc == 0) {
    cVar1 = FUN_0013ead0(param_1,0);
    if (cVar1 != '\0') {
      DAT_00184634 = 1;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,"Pattern matching characters used in file names",5);
      error(0,0,uVar2);
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar2 = dcgettext(0,
                        "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning"
                        ,5);
      error(0,0,uVar2);
    }
    return DAT_00184634;
  }
  return DAT_00184634;
}




// Function: name_hash @ 0x26490

void name_hash(long param_1)

{
  FUN_00141ac0(*(undefined8 *)(param_1 + 0x58));
  return;
}




// Function: name_next_elt @ 0x267d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined * name_next_elt(int param_1)

{
  long lVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  
  do {
    lVar1 = DAT_001846b0;
    if (DAT_001846b0 == 0) {
      FUN_00126210();
      return (undefined *)0x0;
    }
    switch(*(undefined4 *)(DAT_001846b0 + 0x10)) {
    case 0:
switchD_00126801_caseD_0:
      FUN_00126770(*(undefined8 *)(DAT_001846b0 + 0x18));
      if (DAT_001847e8 != '\0') {
        FUN_00123c70(DAT_001846a8);
      }
      _DAT_00184670 = *(undefined4 *)(lVar1 + 0x10);
      _DAT_00184678 = DAT_001846a8;
      FUN_00125a50();
      return &DAT_00184660;
    case 1:
      if (param_1 == 0) goto switchD_00126801_caseD_0;
      uVar4 = FUN_0014c7a0(*(undefined8 *)(DAT_001846b0 + 0x18));
      uVar3 = FUN_00124b00(uVar4);
      FUN_00124c80(uVar3);
      FUN_00125a50();
      break;
    case 2:
      iVar2 = FUN_00126500(DAT_001846b0);
      if (iVar2 == 0) {
        return &DAT_00184660;
      }
      break;
    case 4:
      FUN_00125f80(*(undefined4 *)(DAT_001846b0 + 0x18),*(undefined8 *)(DAT_001846b0 + 0x20));
    case 3:
      FUN_00125a50();
    }
  } while( true );
}




// Function: uid_to_uname @ 0x268d0

void uid_to_uname(__uid_t param_1,undefined8 *param_2)

{
  __uid_t _Var1;
  passwd *ppVar2;
  undefined8 uVar3;
  
  if ((param_1 != 0) && (_Var1 = DAT_001846cc, DAT_001846cc == param_1)) {
LAB_00126946:
    DAT_001846cc = _Var1;
    uVar3 = FUN_0014c7a0(&DAT_00167708);
    *param_2 = uVar3;
    return;
  }
  if ((DAT_001846f0 == 0) || (DAT_001846e4 != param_1)) {
    ppVar2 = getpwuid(param_1);
    _Var1 = param_1;
    if (ppVar2 == (passwd *)0x0) goto LAB_00126946;
    DAT_001846e4 = param_1;
    FUN_00123be0(&DAT_001846f0,ppVar2->pw_name);
  }
  uVar3 = FUN_0014c7a0(DAT_001846f0);
  *param_2 = uVar3;
  return;
}




// Function: gid_to_gname @ 0x26960

void gid_to_gname(__gid_t param_1,undefined8 *param_2)

{
  __gid_t _Var1;
  group *pgVar2;
  undefined8 uVar3;
  
  if ((param_1 != 0) && (_Var1 = DAT_001846c8, DAT_001846c8 == param_1)) {
LAB_001269d6:
    DAT_001846c8 = _Var1;
    uVar3 = FUN_0014c7a0(&DAT_00167708);
    *param_2 = uVar3;
    return;
  }
  if ((DAT_001846e8 == 0) || (DAT_001846e0 != param_1)) {
    pgVar2 = getgrgid(param_1);
    _Var1 = param_1;
    if (pgVar2 == (group *)0x0) goto LAB_001269d6;
    DAT_001846e0 = param_1;
    FUN_00123be0(&DAT_001846e8,pgVar2->gr_name);
  }
  uVar3 = FUN_0014c7a0(DAT_001846e8);
  *param_2 = uVar3;
  return;
}




// Function: uname_to_uid @ 0x269f0

undefined4 uname_to_uid(char *param_1,__uid_t *param_2)

{
  int iVar1;
  passwd *ppVar2;
  
  if ((DAT_001846d8 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_001846d8), iVar1 == 0)) {
    return 0;
  }
  if ((DAT_001846f0 == (char *)0x0) || (*param_1 != *DAT_001846f0)) {
    ppVar2 = getpwnam(param_1);
  }
  else {
    iVar1 = strcmp(param_1,DAT_001846f0);
    if (iVar1 == 0) goto LAB_00126a4e;
    ppVar2 = getpwnam(param_1);
  }
  if (ppVar2 == (passwd *)0x0) {
    FUN_00123be0(&DAT_001846d8,param_1);
    return 0;
  }
  DAT_001846e4 = ppVar2->pw_uid;
  FUN_00123be0(&DAT_001846f0,ppVar2->pw_name);
LAB_00126a4e:
  *param_2 = DAT_001846e4;
  return 1;
}




// Function: gname_to_gid @ 0x26aa0

undefined4 gname_to_gid(char *param_1,__gid_t *param_2)

{
  int iVar1;
  group *pgVar2;
  
  if ((DAT_001846d0 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_001846d0), iVar1 == 0)) {
    return 0;
  }
  if ((DAT_001846e8 == (char *)0x0) || (*param_1 != *DAT_001846e8)) {
    pgVar2 = getgrnam(param_1);
  }
  else {
    iVar1 = strcmp(param_1,DAT_001846e8);
    if (iVar1 == 0) goto LAB_00126afe;
    pgVar2 = getgrnam(param_1);
  }
  if (pgVar2 == (group *)0x0) {
    FUN_00123be0(&DAT_001846d0,param_1);
    return 0;
  }
  DAT_001846e0 = pgVar2->gr_gid;
  FUN_00123be0(&DAT_001846e8,param_1);
LAB_00126afe:
  *param_2 = DAT_001846e0;
  return 1;
}




// Function: name_add_name @ 0x26b50

void name_add_name(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_00125880();
  *(undefined4 *)(lVar1 + 0x10) = 0;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  if (DAT_00184630 != 0) {
    if (DAT_00184630 == 1) {
      DAT_00184630 = 2;
    }
    return;
  }
  DAT_00184630 = 1;
  return;
}




// Function: names_parse_opt @ 0x26ba0

undefined8 names_parse_opt(int param_1,undefined8 param_2,long param_3)

{
  int *piVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char cVar4;
  undefined8 uVar5;
  
  if (param_1 == 0x100) {
    FUN_00126b50(param_2);
    return 0;
  }
  if (param_1 < 0x101) {
    if (param_1 == 0x43) {
      FUN_001258f0(param_2);
      return 0;
    }
    if (param_1 == 0x54) {
      FUN_00125910(param_2);
      return 0;
    }
  }
  else if (param_1 == 0x1000005) {
    piVar1 = (int *)**(undefined8 **)(param_3 + 0x28);
    if (*piVar1 != 2) {
      return 7;
    }
    uVar2 = *(undefined8 *)(piVar1 + 4);
    uVar3 = *(undefined8 *)(piVar1 + 2);
    uVar5 = dcgettext(0,"%s:%lu: unrecognized option",5);
    error(0,0,uVar5,uVar3,uVar2);
    FUN_0012f7d0(2);
    return 7;
  }
  cVar4 = FUN_00125620();
  if (cVar4 == '\0') {
    return 7;
  }
  FUN_00125940();
  return 0;
}




// Function: name_init @ 0x26c90

void name_init(void)

{
  DAT_001846a8 = FUN_0014c5b0(0x66);
  DAT_001846a0 = 100;
  FUN_00125630();
  return;
}




// Function: name_term @ 0x26cc0

void name_term(void)

{
  free(DAT_001846a8);
  return;
}




// Function: name_next @ 0x26cd0

undefined8 name_next(void)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_001267d0();
  uVar2 = 0;
  if (lVar1 != 0) {
    uVar2 = *(undefined8 *)(lVar1 + 0x18);
  }
  return uVar2;
}




// Function: addname @ 0x26cf0

void addname(undefined8 param_1,undefined4 param_2,undefined1 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  uint uVar2;
  bool bVar3;
  
  puVar1 = (undefined8 *)FUN_001263a0();
  uVar2 = DAT_00181238 | DAT_001846f8 | DAT_00184ad4;
  bVar3 = DAT_001846fc == 2;
  *puVar1 = 0;
  *(undefined4 *)(puVar1 + 5) = param_2;
  puVar1[6] = 0;
  puVar1[1] = DAT_001846b8;
  *(uint *)(puVar1 + 4) = uVar2 | (uint)bVar3 << 0x1c;
  puVar1[7] = 0;
  puVar1[8] = param_4;
  *(undefined1 *)((long)puVar1 + 0x24) = param_3;
  if (DAT_001846b8 != (undefined8 *)0x0) {
    *DAT_001846b8 = puVar1;
    DAT_001846b8 = puVar1;
    return;
  }
  DAT_001846c0 = puVar1;
  DAT_001846b8 = puVar1;
  return;
}




// Function: name_gather @ 0x26d90

void name_gather(void)

{
  int iVar1;
  long lVar2;
  uint uVar3;
  int iVar4;
  bool bVar5;
  
  iVar1 = 0;
  iVar4 = iVar1;
  if (DAT_00184aac == '\0') {
    while (lVar2 = FUN_001267d0(0), lVar2 != 0) {
      if (*(int *)(lVar2 + 0x10) == 1) {
        FUN_0014c7a0();
        iVar1 = FUN_00124b00();
      }
      else {
        FUN_00126cf0(*(undefined8 *)(lVar2 + 0x18),iVar1,1,0);
        iVar4 = iVar1;
      }
    }
    if (iVar4 == iVar1) {
      return;
    }
  }
  else {
    while (lVar2 = FUN_001267d0(0), lVar2 != 0) {
      if (*(int *)(lVar2 + 0x10) != 1) {
        FUN_00125ad0(DAT_00184638);
        DAT_001846b8 = (undefined8 *)FUN_001263a0(*(undefined8 *)(lVar2 + 0x18));
        iVar1 = DAT_00184640;
        *DAT_001846b8 = 0;
        *(int *)(DAT_001846b8 + 5) = iVar1;
        uVar3 = DAT_00181238 | DAT_001846f8 | DAT_00184ad4;
        DAT_001846b8[6] = 0;
        bVar5 = DAT_001846fc == 2;
        DAT_001846b8[7] = 0;
        DAT_001846b8[8] = 0;
        *(undefined1 *)((long)DAT_001846b8 + 0x24) = 1;
        DAT_00184638 = DAT_001846b8;
        *(uint *)(DAT_001846b8 + 4) = uVar3 | (uint)bVar5 << 0x1c;
        DAT_001846c0 = DAT_001846b8;
        return;
      }
      FUN_0014c7a0(*(undefined8 *)(lVar2 + 0x18));
      DAT_00184640 = FUN_00124b00();
    }
    iVar1 = DAT_00184640;
    if (DAT_00184640 == 0) {
      return;
    }
  }
  FUN_00126cf0(0,iVar1,0,0);
  return;
}




// Function: add_hierarchy_to_namelist @ 0x26ee0

void add_hierarchy_to_namelist(long param_1,long param_2)

{
  undefined4 uVar1;
  ulong uVar2;
  long lVar3;
  int iVar4;
  undefined8 uVar5;
  char *__s;
  char *__dest;
  size_t sVar6;
  long lVar7;
  int *piVar8;
  char cVar9;
  long in_FS_OFFSET;
  ulong local_238;
  long local_230;
  long local_218;
  ulong local_200;
  undefined8 local_1f8 [11];
  stat local_1a0 [2];
  long local_60;
  int local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar5 = FUN_0011f610();
  *(undefined8 *)(param_2 + 0x38) = uVar5;
  __s = (char *)FUN_0011fa60(uVar5);
  if (__s != (char *)0x0) {
    uVar2 = *(ulong *)(param_2 + 0x18);
    local_200 = uVar2 + 0x66;
    if (uVar2 < 100) {
      local_200 = 0x66;
    }
    __dest = (char *)FUN_0014c5b0();
    uVar1 = *(undefined4 *)(param_2 + 0x28);
    strcpy(__dest,*(char **)(param_2 + 0x10));
    local_238 = uVar2;
    if (__dest[uVar2 - 1] != '/') {
      local_238 = uVar2 + 1;
      __dest[uVar2] = '/';
      __dest[uVar2 + 1] = '\0';
    }
    local_230 = 0;
    cVar9 = *__s;
    if (cVar9 != '\0') {
      local_218 = 0;
      do {
        sVar6 = strlen(__s);
        if (cVar9 == 'D') {
          if (local_200 < local_238 + sVar6) {
            do {
              __dest = (char *)FUN_0014c710(__dest,&local_200);
            } while (local_200 < local_238 + sVar6);
          }
          strcpy(__dest + local_238,__s + 1);
          lVar7 = FUN_00126cf0(__dest,uVar1,0,param_2);
          lVar3 = lVar7;
          if (local_230 != 0) {
            *(long *)(local_218 + 0x50) = lVar7;
            lVar3 = local_230;
          }
          local_230 = lVar3;
          FUN_0012f630(local_1f8);
          iVar4 = *(int *)(param_1 + 0x1a8);
          local_60 = param_1;
          if (iVar4 < 0) {
            piVar8 = __errno_location();
            *piVar8 = -iVar4;
LAB_001270a5:
            FUN_00124dd0(__dest);
          }
          else {
            iVar4 = FUN_001140d0(param_1,__s + 1,DAT_00184808 | 0x10000);
            if (iVar4 < 0) goto LAB_001270a5;
            local_50 = iVar4;
            iVar4 = fstat(iVar4,local_1a0);
            if (iVar4 == 0) {
              local_1f8[0] = FUN_0014c7a0(__dest);
              add_hierarchy_to_namelist(local_1f8,lVar7);
              FUN_00114200(local_1f8);
            }
            else {
              FUN_00124ec0();
            }
          }
          FUN_0012f6f0(local_1f8);
          local_218 = lVar7;
        }
        __s = __s + sVar6 + 1;
        cVar9 = *__s;
      } while (cVar9 != '\0');
    }
    free(__dest);
    *(long *)(param_2 + 0x48) = local_230;
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: remname @ 0x27150

void remname(long *param_1)

{
  long *plVar1;
  long lVar2;
  long lVar3;
  
  plVar1 = (long *)param_1[1];
  lVar2 = *param_1;
  lVar3 = lVar2;
  if (plVar1 != (long *)0x0) {
    *plVar1 = lVar2;
    lVar3 = DAT_001846c0;
  }
  DAT_001846c0 = lVar3;
  if (lVar2 == 0) {
    DAT_001846b8 = plVar1;
    return;
  }
  *(long **)(lVar2 + 8) = plVar1;
  return;
}




// Function: add_starting_file @ 0x27190

void add_starting_file(void)

{
  undefined8 *puVar1;
  uint uVar2;
  undefined8 *puVar3;
  bool bVar4;
  
  puVar3 = (undefined8 *)FUN_001263a0();
  if (DAT_00184a78 != '\0') {
    FUN_00127150(DAT_001846c0);
    FUN_00125ad0();
  }
  bVar4 = DAT_001846b8 == (undefined8 *)0x0;
  puVar3[1] = 0;
  puVar1 = puVar3;
  *puVar3 = DAT_001846c0;
  DAT_001846c0 = puVar1;
  if (bVar4) {
    DAT_001846b8 = puVar3;
  }
  uVar2 = DAT_00181238 | DAT_001846f8;
  puVar3[6] = 0;
  uVar2 = uVar2 | DAT_00184ad4;
  bVar4 = DAT_001846fc == 2;
  *(undefined4 *)(puVar3 + 5) = 0;
  puVar3[7] = 0;
  puVar3[8] = 0;
  *(undefined1 *)((long)puVar3 + 0x24) = 1;
  *(uint *)(puVar3 + 4) = uVar2 | (uint)bVar4 << 0x1c;
  DAT_00184a78 = 1;
  return;
}




// Function: name_match @ 0x27250

bool name_match(long param_1)

{
  long lVar1;
  
  do {
    if (DAT_001846c0 == 0) {
      return true;
    }
    if (**(char **)(DAT_001846c0 + 0x10) == '\0') {
      FUN_00124c80(*(undefined4 *)(DAT_001846c0 + 0x28));
      DAT_001846c0 = 0;
      DAT_001846b8 = 0;
      return true;
    }
    lVar1 = FUN_001264a0(param_1);
    if (DAT_00184a78 == '\0') {
joined_r0x0012732a:
      if (lVar1 != 0) {
        if (((*(char *)(param_1 + *(long *)(lVar1 + 0x18)) != '/') || (DAT_00184ad4 == 0)) ||
           (*(long *)(lVar1 + 0x30) == 0)) {
          *(long *)(lVar1 + 0x30) = *(long *)(lVar1 + 0x30) + 1;
        }
        FUN_00124c80(*(undefined4 *)(lVar1 + 0x28));
        if (DAT_00184b38 == 0) {
          return *(long *)(lVar1 + 0x30) != 0;
        }
        return *(long *)(lVar1 + 0x30) == DAT_00184b38;
      }
    }
    else if (DAT_001846c0 == lVar1) {
      DAT_00184a78 = '\0';
      goto joined_r0x0012732a;
    }
    if (((DAT_00184aac == '\0') || (*(long *)(DAT_001846c0 + 0x30) == 0)) ||
       (FUN_00126d90(), *(long *)(DAT_001846c0 + 0x30) != 0)) {
      return false;
    }
  } while( true );
}




// Function: all_names_found @ 0x273a0

undefined8 all_names_found(long param_1)

{
  char *__s;
  undefined8 *puVar1;
  ulong uVar2;
  size_t sVar3;
  undefined8 uVar4;
  
  uVar2 = DAT_00184b38;
  __s = *(char **)(param_1 + 8);
  if (__s == (char *)0x0) {
    return 0;
  }
  uVar4 = 0;
  if (DAT_00184b38 != 0) {
    if (*(char *)(param_1 + 0x10) != '\0') {
      return 0;
    }
    sVar3 = strlen(__s);
    for (puVar1 = DAT_001846c0; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
      if ((*(char *)puVar1[2] != '\0') && ((ulong)puVar1[6] < uVar2)) {
        return 0;
      }
      if (((ulong)puVar1[3] <= sVar3) && (__s[puVar1[3]] == '/')) {
        return 0;
      }
    }
    uVar4 = 1;
  }
  return uVar4;
}




// Function: names_notfound @ 0x27430

void names_notfound(void)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  
  puVar2 = DAT_001846c0;
joined_r0x00127456:
  do {
    if (puVar2 == (undefined8 *)0x0) {
      DAT_001846c0 = (undefined8 *)0x0;
      DAT_001846b8 = 0;
      if (DAT_00184aac == '\0') {
        return;
      }
      while (lVar5 = FUN_00126cd0(1), lVar5 != 0) {
        FUN_001263f0(lVar5);
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar3 = FUN_0014aa70(lVar5);
        uVar4 = dcgettext(0,"%s: Not found in archive",5);
        error(0,0,uVar4,uVar3);
        DAT_00184d58 = 2;
      }
      return;
    }
    if (DAT_00184b38 == 0) {
      if (puVar2[6] == 0) {
        cVar1 = *(char *)puVar2[2];
joined_r0x00127492:
        if (cVar1 != '\0') {
          FUN_001263f0();
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar3 = FUN_0014aa70(puVar2[2]);
          if (puVar2[6] == 0) {
            uVar4 = dcgettext(0,"%s: Not found in archive",5);
          }
          else {
            uVar4 = dcgettext(0,"%s: Required occurrence not found in archive",5);
          }
          error(0,0,uVar4,uVar3);
          DAT_00184d58 = 2;
          puVar2 = (undefined8 *)*puVar2;
          goto joined_r0x00127456;
        }
      }
    }
    else if ((ulong)puVar2[6] < DAT_00184b38) {
      cVar1 = *(char *)puVar2[2];
      goto joined_r0x00127492;
    }
    puVar2 = (undefined8 *)*puVar2;
  } while( true );
}




// Function: label_notfound @ 0x275b0

void label_notfound(void)

{
  int iVar1;
  long *plVar2;
  long lVar3;
  undefined8 uVar4;
  
  plVar2 = DAT_001846c0;
  if (DAT_001846c0 == (long *)0x0) {
    return;
  }
  do {
    if (DAT_00184b38 == 0) {
      if (plVar2[6] != 0) {
        return;
      }
    }
    else if (DAT_00184b38 <= (ulong)plVar2[6]) {
      return;
    }
    plVar2 = (long *)*plVar2;
  } while (plVar2 != (long *)0x0);
  if (DAT_00184a54 != 0) {
    uVar4 = dcgettext(0,"Archive label mismatch",5);
    error(0,0,uVar4);
  }
  FUN_0012f7d0(1);
  plVar2 = DAT_001846c0;
  while ((plVar2 != (long *)0x0 && (iVar1 = FUN_001263f0(plVar2[2]), iVar1 == 0))) {
    plVar2 = (long *)*plVar2;
  }
  DAT_001846c0 = (long *)0x0;
  DAT_001846b8 = 0;
  if (DAT_00184aac != '\0') {
    do {
      lVar3 = FUN_00126cd0(1);
      if (lVar3 == 0) {
        return;
      }
      iVar1 = FUN_001263f0(lVar3);
    } while (iVar1 == 0);
  }
  return;
}




// Function: collect_and_sort_names @ 0x276a0

void collect_and_sort_names(void)

{
  long *plVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  undefined8 *puVar6;
  long *plVar7;
  long *plVar8;
  char *pcVar9;
  long in_FS_OFFSET;
  undefined8 local_1f8 [11];
  stat local_1a0 [2];
  int local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00126d90();
  if (DAT_001846c0 == (long *)0x0) {
    FUN_00126cf0(".",0,0,0);
    if (DAT_00184b28 != 0) goto LAB_001276e7;
LAB_00127710:
    if (DAT_001846c0 == (long *)0x0) {
      iVar2 = 0;
      goto LAB_001277b7;
    }
  }
  else if (DAT_00184b28 != 0) {
LAB_001276e7:
    iVar2 = FUN_00124ae0();
    if (iVar2 != 0) {
      if (iVar2 != 1) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar9 = "Only one -C option is allowed with --listed-incremental";
LAB_00127a54:
        uVar4 = dcgettext(0,pcVar9,5);
        error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_0012c970(2);
      }
      if ((int)DAT_001846c0[5] == 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar9 = "Using -C option inside file list is not allowed with --listed-incremental";
        goto LAB_00127a54;
      }
    }
    FUN_0011fd80();
    goto LAB_00127710;
  }
  plVar8 = DAT_001846c0;
  iVar3 = 0;
  do {
    while ((((iVar2 = iVar3, plVar8[6] != 0 || (plVar8[7] != 0)) ||
            ((*(byte *)((long)plVar8 + 0x23) & 0x10) != 0)) ||
           (FUN_00124c80((int)plVar8[5]), *(char *)plVar8[2] == '\0'))) {
LAB_00127730:
      plVar8 = (long *)*plVar8;
      iVar3 = iVar2 + 1;
      if (plVar8 == (long *)0x0) goto LAB_001277b0;
    }
    FUN_0012f630(local_1f8);
    iVar3 = FUN_001246f0(plVar8[2],local_1a0);
    if (iVar3 != 0) {
      FUN_00124ec0(plVar8[2]);
      goto LAB_00127730;
    }
    if ((local_1a0[0].st_mode & 0xf000) == 0x4000) {
      iVar3 = __openat_2(DAT_00181234,plVar8[2],DAT_00184808 | 0x10000);
      if (iVar3 < 0) {
        FUN_00124dd0(plVar8[2]);
      }
      else {
        local_50 = iVar3;
        iVar3 = fstat(iVar3,local_1a0);
        if (iVar3 == 0) {
          local_1f8[0] = FUN_0014c7a0(plVar8[2]);
          plVar8[6] = plVar8[6] + 1;
          FUN_00126ee0(local_1f8,plVar8);
        }
        else {
          FUN_00124ec0();
        }
      }
    }
    FUN_0012f6f0(local_1f8);
    plVar8 = (long *)*plVar8;
    iVar3 = iVar2 + 1;
  } while (plVar8 != (long *)0x0);
LAB_001277b0:
  iVar2 = iVar2 + 1;
LAB_001277b7:
  DAT_001846c0 = (long *)FUN_00125800(DAT_001846c0,iVar2,FUN_001259f0);
  uVar4 = FUN_00141b30(0,0,FUN_00126490,FUN_001259d0,0);
  if (DAT_001846c0 == (long *)0x0) {
    plVar8 = (long *)0x0;
    iVar2 = 0;
  }
  else {
    iVar2 = 0;
    plVar1 = DAT_001846c0;
    plVar8 = (long *)0x0;
    do {
      while( true ) {
        plVar7 = plVar1;
        plVar1 = (long *)*plVar7;
        lVar5 = FUN_00125340((int)plVar7[5],plVar7[2]);
        plVar7[0xb] = lVar5;
        if ((plVar8 != (long *)0x0) && (lVar5 = FUN_001418d0(uVar4,plVar7), lVar5 != 0)) break;
LAB_00127869:
        plVar7[6] = 0;
        lVar5 = FUN_00142140(uVar4,plVar7);
        if (lVar5 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_00116420();
        }
        iVar2 = iVar2 + 1;
        plVar8 = plVar7;
        if (plVar1 == (long *)0x0) goto LAB_00127891;
      }
      if (plVar7[8] == 0) {
        if (*(long *)(lVar5 + 0x48) != 0) {
          FUN_001266c0(*(long *)(lVar5 + 0x48),plVar7[2],plVar7[3]);
        }
        iVar2 = iVar2 + -1;
        FUN_001421a0(uVar4,plVar7);
        FUN_00127150(lVar5);
        FUN_00125ad0();
        goto LAB_00127869;
      }
      if (plVar7[9] != 0) {
        FUN_001266c0(plVar7[9],*(undefined8 *)(lVar5 + 0x10),*(undefined8 *)(lVar5 + 0x18));
      }
      FUN_00127150(plVar7);
      FUN_00125ad0();
    } while (plVar1 != (long *)0x0);
  }
LAB_00127891:
  DAT_001846b8 = plVar8;
  FUN_00141ce0(uVar4);
  DAT_001846c0 = (long *)FUN_00125800(DAT_001846c0,iVar2,FUN_00125970);
  puVar6 = DAT_001846c0;
  if (DAT_00184b28 != 0) {
    for (; puVar6 != (undefined8 *)0x0; puVar6 = (undefined8 *)*puVar6) {
      if (*(char *)puVar6[2] != '\0') {
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_0011fab0(puVar6[7]);
          return;
        }
        goto LAB_00127a8f;
      }
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00127a8f:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: name_scan @ 0x27aa0

long name_scan(undefined8 param_1)

{
  long lVar1;
  
  while( true ) {
    lVar1 = FUN_001264a0(param_1);
    if (lVar1 != 0) {
      return lVar1;
    }
    if (DAT_00184aac == '\0') {
      return 0;
    }
    if (DAT_001846c0 == 0) {
      return 0;
    }
    if (*(long *)(DAT_001846c0 + 0x30) == 0) break;
    FUN_00126d90();
    if (*(long *)(DAT_001846c0 + 0x30) != 0) {
      return 0;
    }
  }
  return 0;
}




// Function: name_from_list @ 0x27b00

undefined8 * name_from_list(void)

{
  undefined8 *puVar1;
  
  puVar1 = DAT_001846c0;
  if (DAT_00184620 != (undefined8 *)0x0) goto LAB_00127b10;
  while( true ) {
    DAT_00184620 = puVar1;
    if (DAT_00184620 == (undefined8 *)0x0) {
      return (undefined8 *)0x0;
    }
LAB_00127b10:
    puVar1 = DAT_00184620;
    if ((DAT_00184620[6] == 0) && (*(char *)DAT_00184620[2] != '\0')) break;
    puVar1 = (undefined8 *)*DAT_00184620;
  }
  DAT_00184620[6] = 1;
  FUN_00124c80(*(undefined4 *)(puVar1 + 5));
  return DAT_00184620;
}




// Function: blank_name_list @ 0x27b80

void blank_name_list(void)

{
  undefined8 *puVar1;
  
  DAT_00184620 = 0;
  for (puVar1 = DAT_001846c0; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    puVar1[6] = 0;
  }
  return;
}




// Function: make_file_name @ 0x27bc0

void * make_file_name(char *param_1,char *param_2)

{
  size_t __n;
  size_t sVar1;
  void *__dest;
  size_t sVar2;
  
  __n = strlen(param_1);
  sVar1 = strlen(param_2);
  sVar2 = 0;
  if ((__n != 0) && (sVar2 = __n, param_1[__n - 1] != '/')) {
    sVar2 = __n + 1;
  }
  __dest = (void *)FUN_0014c5b0(sVar1 + 1 + sVar2);
  memcpy(__dest,param_1,__n);
  *(undefined1 *)((long)__dest + __n) = 0x2f;
  memcpy((void *)((long)__dest + sVar2),param_2,sVar1 + 1);
  return __dest;
}




// Function: stripped_prefix_len @ 0x27c50

long stripped_prefix_len(char *param_1,long param_2)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  
  cVar2 = *param_1;
  pcVar3 = param_1;
  while (cVar2 == '/') {
    pcVar1 = pcVar3 + 1;
    pcVar3 = pcVar3 + 1;
    cVar2 = *pcVar1;
  }
  while( true ) {
    while( true ) {
      if (cVar2 == '\0') {
        return -1;
      }
      pcVar3 = pcVar3 + 1;
      if (cVar2 == '/') break;
      cVar2 = *pcVar3;
    }
    param_2 = param_2 + -1;
    if (param_2 == 0) break;
    for (; cVar2 = *pcVar3, cVar2 == '/'; pcVar3 = pcVar3 + 1) {
    }
  }
  return (long)pcVar3 - (long)param_1;
}




// Function: contains_dot_dot @ 0x27cc0

bool contains_dot_dot(char *param_1)

{
  char cVar1;
  char cVar2;
  
  while (((cVar2 = *param_1, cVar2 != '.' || (param_1[1] != '.')) ||
         (cVar1 = param_1[2], cVar1 != '/' && cVar1 != '\0'))) {
    while (param_1 = param_1 + 1, cVar2 != '/') {
      if (cVar2 == '\0') {
        return false;
      }
      cVar2 = *param_1;
    }
  }
  return cVar1 == '/' || cVar1 == '\0';
}




// Function: tar_sparse_member_p @ 0x27d10

undefined8 tar_sparse_member_p(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x10);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127d1d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 0;
}




// Function: tar_sparse_done @ 0x27d30

undefined8 tar_sparse_done(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 8);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127d3d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 1;
}




// Function: tar_sparse_scan @ 0x27d50

undefined8 tar_sparse_scan(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x30);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127d5d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 1;
}




// Function: tar_sparse_dump_region @ 0x27d70

undefined8 tar_sparse_dump_region(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x38);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127d7d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 0;
}




// Function: tar_sparse_extract_region @ 0x27d90

undefined8 tar_sparse_extract_region(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x40);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127d9d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 0;
}




// Function: tar_sparse_decode_header @ 0x27db0

undefined8 tar_sparse_decode_header(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x28);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127dbd. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 1;
}




// Function: tar_sparse_fixup_header @ 0x27dd0

undefined8 tar_sparse_fixup_header(long param_1)

{
  code *UNRECOVERED_JUMPTABLE;
  undefined8 uVar1;
  
  UNRECOVERED_JUMPTABLE = *(code **)(*(long *)(param_1 + 0x20) + 0x20);
  if (UNRECOVERED_JUMPTABLE != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x00127ddd. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*UNRECOVERED_JUMPTABLE)();
    return uVar1;
  }
  return 1;
}




// Function: zero_block_p @ 0x27df0

undefined8 zero_block_p(char *param_1,long param_2)

{
  char cVar1;
  char *pcVar2;
  
  pcVar2 = param_1;
  do {
    if (pcVar2 == param_1 + param_2) {
      return 1;
    }
    cVar1 = *pcVar2;
    pcVar2 = pcVar2 + 1;
  } while (cVar1 == '\0');
  return 0;
}




// Function: sparse_add_map @ 0x27e20

void sparse_add_map(long param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  long lVar2;
  undefined8 uVar3;
  long lVar4;
  
  lVar4 = *(long *)(param_1 + 0x140);
  lVar2 = *(long *)(param_1 + 0x130);
  if (*(long *)(param_1 + 0x138) == lVar2) {
    lVar4 = FUN_0014c680(lVar4,param_1 + 0x138,0x10);
    *(long *)(param_1 + 0x140) = lVar4;
  }
  uVar3 = param_2[1];
  puVar1 = (undefined8 *)(lVar4 + lVar2 * 0x10);
  *puVar1 = *param_2;
  puVar1[1] = uVar3;
  *(long *)(param_1 + 0x130) = lVar2 + 1;
  return;
}




// Function: sparse_select_optab @ 0x27e90

undefined8 sparse_select_optab(long param_1)

{
  uint uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00184400;
  if (DAT_00184400 == 0) {
    uVar1 = DAT_00184ba4;
  }
  if (uVar1 == 5) {
    *(undefined **)(param_1 + 0x20) = &DAT_0017f7e0;
    return 1;
  }
  if (uVar1 < 6) {
    if (uVar1 == 2) {
LAB_00127ed0:
      *(undefined **)(param_1 + 0x20) = &DAT_0017f840;
      return 1;
    }
    if (uVar1 == 4) {
      uVar2 = 1;
      *(undefined **)(param_1 + 0x20) = &DAT_0017f780;
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 0;
    if (uVar1 == 6) goto LAB_00127ed0;
  }
  return uVar2;
}




// Function: tar_sparse_init @ 0x27f70

void tar_sparse_init(undefined1 (*param_1) [16])

{
  char cVar1;
  
  *param_1 = (undefined1  [16])0x0;
  param_1[1] = (undefined1  [16])0x0;
  param_1[2] = (undefined1  [16])0x0;
  cVar1 = FUN_00127e90();
  if ((cVar1 != '\0') && ((code *)**(undefined8 **)param_1[2] != (code *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x00127f9c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)**(undefined8 **)param_1[2])();
    return;
  }
  return;
}




// Function: dump_zeros @ 0x27fb0

undefined8 dump_zeros(int *param_1,long param_2)

{
  long lVar1;
  ssize_t sVar2;
  int *piVar3;
  size_t __n;
  
  lVar1 = *(long *)(param_1 + 2);
  if (param_2 < lVar1) {
    piVar3 = __errno_location();
    *piVar3 = 0x16;
    return 0;
  }
  while( true ) {
    if (param_2 <= lVar1) {
      return 1;
    }
    __n = param_2 - lVar1;
    if (0x200 < (long)__n) {
      __n = 0x200;
    }
    sVar2 = write(*param_1,&DAT_00163760,__n);
    if (sVar2 < 1) break;
    lVar1 = sVar2 + *(long *)(param_1 + 2);
    *(long *)(param_1 + 2) = lVar1;
  }
  if (sVar2 == 0) {
    piVar3 = __errno_location();
    *piVar3 = 0x16;
  }
  return 0;
}




// Function: lseek_or_error @ 0x28050

undefined8 lseek_or_error(int *param_1,__off_t param_2)

{
  char cVar1;
  __off_t _Var2;
  
  if ((char)param_1[1] == '\0') {
    cVar1 = FUN_00127fb0();
    if (cVar1 == '\0') goto LAB_00128089;
  }
  else {
    _Var2 = lseek(*param_1,param_2,0);
    if (_Var2 < 0) {
LAB_00128089:
      FUN_00124e90(**(undefined8 **)(param_1 + 6),param_2);
      return 0;
    }
  }
  return 1;
}




// Function: sparse_extract_region @ 0x280b0

uint sparse_extract_region(undefined4 *param_1,long param_2)

{
  byte bVar1;
  uint uVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  
  uVar2 = FUN_00128050(param_1,*(undefined8 *)
                                (*(long *)(*(long *)(param_1 + 6) + 0x140) + param_2 * 0x10));
  if ((char)uVar2 != '\0') {
    lVar7 = *(long *)(*(long *)(*(long *)(param_1 + 6) + 0x140) + 8 + param_2 * 0x10);
    if (lVar7 == 0) {
      bVar1 = *(byte *)(param_1 + 1);
      if ((bVar1 != 0) && (iVar3 = FUN_0012aed0(*param_1), iVar3 != 0)) {
        uVar2 = (uint)bVar1;
        FUN_00132d90(**(undefined8 **)(param_1 + 6));
      }
    }
    else {
      while (0 < lVar7) {
        lVar6 = 0x200;
        if (lVar7 < 0x201) {
          lVar6 = lVar7;
        }
        lVar4 = FUN_0010e130();
        if (lVar4 == 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar5 = dcgettext(0,"Unexpected EOF in archive",5);
          error(0,0,uVar5);
          DAT_00184d58 = 2;
          return 0;
        }
        FUN_0010d1f0(lVar4);
        *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
        lVar4 = FUN_001249f0(*param_1,lVar4,lVar6);
        lVar7 = lVar7 - lVar4;
        FUN_0010ccf0();
        *(long *)(param_1 + 2) = *(long *)(param_1 + 2) + lVar4;
        if (lVar6 != lVar4) {
          FUN_00132e30(**(undefined8 **)(param_1 + 6),lVar4,lVar6);
          return 0;
        }
      }
    }
  }
  return uVar2;
}




// Function: star_fixup_header @ 0x28220

ulong star_fixup_header(long param_1)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  
  uVar2 = FUN_001219b0(DAT_00184408 + 0x1c4,0xc);
  lVar1 = *(long *)(param_1 + 0x18);
  *(undefined8 *)(lVar1 + 0x118) = *(undefined8 *)(lVar1 + 0x88);
  uVar3 = 0;
  if (-1 < (long)uVar2) {
    uVar3 = uVar2;
  }
  *(ulong *)(lVar1 + 0x88) = uVar3;
  return ~uVar2 >> 0x3f;
}




// Function: oldgnu_fixup_header @ 0x28270

ulong oldgnu_fixup_header(long param_1)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  
  uVar2 = FUN_001219b0(DAT_00184408 + 0x1e3,0xc);
  lVar1 = *(long *)(param_1 + 0x18);
  *(undefined8 *)(lVar1 + 0x118) = *(undefined8 *)(lVar1 + 0x88);
  uVar3 = 0;
  if (-1 < (long)uVar2) {
    uVar3 = uVar2;
  }
  *(ulong *)(lVar1 + 0x88) = uVar3;
  return ~uVar2 >> 0x3f;
}




// Function: oldgnu_add_sparse @ 0x282c0

undefined8 oldgnu_add_sparse(long param_1,long param_2)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  ulong local_38;
  ulong local_30;
  long local_20;
  
  uVar2 = 1;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_2 + 0xc) != '\0') {
    local_38 = FUN_001219b0(param_2,0xc);
    local_30 = FUN_001219b0(param_2 + 0xc,0xc);
    uVar2 = 2;
    if ((((-1 < (long)(local_30 | local_38)) && (!SCARRY8(local_38,local_30))) &&
        (lVar1 = *(long *)(param_1 + 0x18), (long)(local_38 + local_30) <= *(long *)(lVar1 + 0x88)))
       && (-1 < *(long *)(lVar1 + 0x118))) {
      FUN_00127e20(lVar1,&local_38);
      uVar2 = 0;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: star_get_sparse_info @ 0x28370

undefined8 star_get_sparse_info(long param_1)

{
  char cVar1;
  int iVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulong uVar6;
  long lVar7;
  
  lVar3 = DAT_00184408;
  cVar1 = *(char *)(DAT_00184408 + 0x159);
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  if ((cVar1 == '\0') && (*(char *)(lVar3 + 0x16e) != '\0')) {
    lVar7 = lVar3 + 0x164;
    do {
      iVar2 = FUN_001282c0(param_1,lVar7);
      if (iVar2 != 0) goto LAB_00128409;
      lVar7 = lVar7 + 0x18;
    } while (lVar3 + 0x1c4 != lVar7);
    if (*(char *)(lVar3 + 0x163) != '\0') {
      lVar3 = FUN_0010e130();
      if (lVar3 != 0) goto LAB_001283c1;
LAB_00128462:
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar4 = dcgettext(0,"Unexpected EOF in archive",5);
      error(0,0,uVar4);
      DAT_00184d58 = 2;
      return 0;
    }
  }
  else {
    do {
      lVar3 = FUN_0010e130();
      if (lVar3 == 0) goto LAB_00128462;
LAB_001283c1:
      uVar6 = 0;
      FUN_0010d1f0(lVar3);
      lVar7 = lVar3;
      do {
        uVar6 = uVar6 + 1;
        iVar2 = FUN_001282c0(param_1,lVar7);
        lVar7 = lVar7 + 0x18;
        if (0x14 < uVar6) break;
      } while (iVar2 == 0);
      *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + 0x200;
    } while ((*(char *)(lVar3 + 0x1f8) != '\0') && (iVar2 == 0));
LAB_00128409:
    if (iVar2 == 2) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar4 = **(undefined8 **)(param_1 + 0x18);
      uVar5 = dcgettext(0,"%s: invalid sparse archive member",5);
      error(0,0,uVar5,uVar4);
      DAT_00184d58 = 2;
      return 0;
    }
  }
  return 1;
}




// Function: oldgnu_get_sparse_info @ 0x28500

undefined8 oldgnu_get_sparse_info(long param_1)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  ulong uVar6;
  long lVar7;
  
  lVar1 = DAT_00184408;
  lVar5 = DAT_00184408 + 0x182;
  lVar7 = DAT_00184408 + 0x1e2;
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  do {
    iVar2 = FUN_001282c0(param_1,lVar5);
    if (iVar2 != 0) goto LAB_001285b0;
    lVar5 = lVar5 + 0x18;
  } while (lVar5 != lVar7);
  if (*(char *)(lVar1 + 0x1e2) != '\0') {
    do {
      lVar5 = FUN_0010e130();
      if (lVar5 == 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar3 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar3);
        DAT_00184d58 = 2;
        return 0;
      }
      uVar6 = 0;
      FUN_0010d1f0(lVar5);
      lVar7 = lVar5;
      do {
        uVar6 = uVar6 + 1;
        iVar2 = FUN_001282c0(param_1,lVar7);
        lVar7 = lVar7 + 0x18;
        if (0x14 < uVar6) break;
      } while (iVar2 == 0);
    } while ((*(char *)(lVar5 + 0x1f8) != '\0') && (iVar2 == 0));
LAB_001285b0:
    if (iVar2 == 2) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = **(undefined8 **)(param_1 + 0x18);
      uVar4 = dcgettext(0,"%s: invalid sparse archive member",5);
      error(0,0,uVar4,uVar3);
      DAT_00184d58 = 2;
      return 0;
    }
  }
  return 1;
}




// Function: sparse_dump_region @ 0x28660

char sparse_dump_region(int *param_1,long param_2)

{
  int iVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  undefined8 *puVar8;
  long lVar9;
  long *plVar10;
  long in_FS_OFFSET;
  char local_f1;
  stat local_e8;
  undefined1 local_58 [24];
  long local_40;
  
  param_2 = param_2 * 0x10;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar8 = (undefined8 *)(param_2 + *(long *)(*(long *)(param_1 + 6) + 0x140));
  lVar7 = puVar8[1];
  local_f1 = FUN_00128050(param_1,*puVar8);
  if (local_f1 != '\0') {
    for (; 0 < lVar7; lVar7 = lVar7 - lVar3) {
      lVar9 = 0x200;
      if (lVar7 < 0x201) {
        lVar9 = lVar7;
      }
      lVar2 = FUN_0010e130();
      lVar3 = FUN_0014af10(*param_1,lVar2,lVar9);
      if (lVar3 == -1) {
        plVar10 = (long *)(param_2 + (*(undefined8 **)(param_1 + 6))[0x28]);
        FUN_00124e00(**(undefined8 **)(param_1 + 6),(plVar10[1] + *plVar10) - lVar7,lVar9);
        local_f1 = '\0';
        break;
      }
      if (lVar3 == 0) {
        iVar1 = fstat(*param_1,&local_e8);
        if (iVar1 == 0) {
          lVar7 = *(long *)(*(long *)(param_1 + 6) + 0x88) - local_e8.st_size;
        }
        else {
          plVar10 = (long *)(param_2 + *(long *)(*(long *)(param_1 + 6) + 0x140));
          lVar7 = (lVar7 + *(long *)(*(long *)(param_1 + 6) + 0x88)) - (plVar10[1] + *plVar10);
        }
        if (((byte)DAT_00183b80 & 0x80) != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = FUN_00143090(lVar7,local_58);
          uVar5 = FUN_0014aa70(**(undefined8 **)(param_1 + 6));
          uVar6 = dcngettext(0,"%s: File shrank by %s byte; padding with zeros",
                             "%s: File shrank by %s bytes; padding with zeros",lVar7,5);
          error(0,0,uVar6,uVar5,uVar4);
        }
        if (DAT_00184b52 == '\0') {
          FUN_0012f7d0(1);
        }
        local_f1 = '\0';
        break;
      }
      memset((void *)(lVar2 + lVar3),0,0x200 - lVar3);
      *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + lVar3;
      FUN_0010d1f0(lVar2);
    }
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_f1;
}




// Function: decode_num @ 0x28870

undefined8 decode_num(uintmax_t *param_1,char *param_2,ulong param_3)

{
  int *piVar1;
  uintmax_t uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((int)*param_2 - 0x30U < 10) {
    piVar1 = __errno_location();
    *piVar1 = 0;
    uVar2 = strtoumax(param_2,&local_38,10);
    if (((uVar2 <= param_3) && (*piVar1 != 0x22)) && (*local_38 == '\0')) {
      *param_1 = uVar2;
      uVar3 = 1;
      goto LAB_00128897;
    }
  }
  uVar3 = 0;
LAB_00128897:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pax_decode_header @ 0x28910

undefined4 pax_decode_header(long param_1)

{
  char cVar1;
  undefined4 uVar2;
  long lVar3;
  char *pcVar4;
  char *pcVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  char *pcVar8;
  long lVar9;
  char *pcVar10;
  char *pcVar11;
  char *pcVar12;
  long in_FS_OFFSET;
  ulong local_88;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  char local_58 [20];
  char local_44 [4];
  long local_40;
  
  uVar2 = 1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(int *)(*(long *)(param_1 + 0x18) + 0x124) == 0) goto LAB_00128944;
  lVar3 = FUN_0010d180();
  FUN_0010d1f0(DAT_00184408);
  pcVar4 = (char *)FUN_0010e130();
  if (pcVar4 == (char *)0x0) {
LAB_00128ca8:
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar7 = dcgettext(0,"Unexpected EOF in archive",5);
    error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
    FUN_001163d0();
  }
  pcVar10 = pcVar4 + 0x200;
  pcVar5 = pcVar4;
  pcVar11 = local_58;
  do {
    if (pcVar5 == pcVar10) {
      FUN_0010d1f0(pcVar4);
      pcVar5 = (char *)FUN_0010e130();
      if (pcVar5 == (char *)0x0) goto LAB_00128ca8;
      pcVar10 = pcVar5 + 0x200;
      pcVar4 = pcVar5;
    }
    pcVar8 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar12 = pcVar11 + 1;
    *pcVar11 = cVar1;
    if (cVar1 == '\n') {
      *pcVar11 = '\0';
      uVar2 = FUN_00128870(&local_70,local_58,0xffffffffffffffff);
      if ((char)uVar2 == '\0') goto LAB_00128a87;
      lVar9 = *(long *)(param_1 + 0x18);
      *(undefined8 *)(lVar9 + 0x138) = local_70;
      uVar7 = FUN_0014c720(local_70,0x10);
      *(undefined8 *)(lVar9 + 0x140) = uVar7;
      lVar9 = *(long *)(param_1 + 0x18);
      *(undefined8 *)(lVar9 + 0x130) = 0;
      if (*(long *)(lVar9 + 0x138) == 0) goto LAB_00128c84;
      local_88 = 0;
      pcVar5 = local_58;
      goto LAB_00128b3b;
    }
    pcVar5 = pcVar8;
    pcVar11 = pcVar12;
  } while (pcVar12 != local_44);
LAB_00128a00:
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  pcVar4 = "%s: numeric overflow in sparse archive member";
  uVar7 = **(undefined8 **)(param_1 + 0x18);
  goto LAB_00128a21;
  while( true ) {
    *pcVar5 = '\0';
    cVar1 = FUN_00128870(&local_70,local_58,0x7fffffffffffffff);
    if (cVar1 == '\0') goto LAB_00128a87;
    local_68 = local_70;
    pcVar5 = local_58;
    pcVar11 = pcVar8 + 1;
    while( true ) {
      if (pcVar11 == pcVar10) {
        FUN_0010d1f0(pcVar4);
        pcVar11 = (char *)FUN_0010e130();
        if (pcVar11 == (char *)0x0) goto LAB_00128ca8;
        pcVar10 = pcVar11 + 0x200;
        pcVar4 = pcVar11;
      }
      pcVar8 = pcVar11 + 1;
      cVar1 = *pcVar11;
      pcVar12 = pcVar5 + 1;
      *pcVar5 = cVar1;
      if (cVar1 == '\n') break;
      pcVar5 = pcVar12;
      pcVar11 = pcVar8;
      if (pcVar12 == local_44) goto LAB_00128a00;
    }
    *pcVar5 = '\0';
    cVar1 = FUN_00128870(&local_70,local_58,0x7fffffffffffffff);
    if (cVar1 == '\0') goto LAB_00128a87;
    local_60 = local_70;
    FUN_00127e20(*(undefined8 *)(param_1 + 0x18),&local_68);
    local_88 = local_88 + 1;
    pcVar5 = local_58;
    if (*(ulong *)(*(long *)(param_1 + 0x18) + 0x138) <= local_88) break;
LAB_00128b3b:
    if (pcVar8 == pcVar10) {
      FUN_0010d1f0(pcVar4);
      pcVar8 = (char *)FUN_0010e130();
      if (pcVar8 == (char *)0x0) goto LAB_00128ca8;
      pcVar10 = pcVar8 + 0x200;
      pcVar4 = pcVar8;
    }
    cVar1 = *pcVar8;
    pcVar11 = pcVar5 + 1;
    *pcVar5 = cVar1;
    if (cVar1 != '\n') {
      pcVar5 = pcVar11;
      pcVar8 = pcVar8 + 1;
      if (pcVar11 == local_44) goto LAB_00128a00;
      goto LAB_00128b3b;
    }
  }
LAB_00128c84:
  FUN_0010d1f0(pcVar4);
  lVar9 = FUN_0010d180();
  *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + (lVar9 - lVar3) * 0x200;
  goto LAB_00128944;
LAB_00128a87:
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  pcVar4 = "%s: malformed sparse archive member";
  uVar7 = **(undefined8 **)(param_1 + 0x18);
LAB_00128a21:
  uVar2 = 0;
  uVar6 = dcgettext(0,pcVar4,5);
  error(0,0,uVar6,uVar7);
  DAT_00184d58 = 2;
LAB_00128944:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: pax_start_header @ 0x28ce0

void pax_start_header(long param_1)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x88);
  *(undefined8 *)(param_1 + 0x88) = *(undefined8 *)(param_1 + 0x118);
  FUN_001132d0();
  *(undefined8 *)(param_1 + 0x88) = uVar1;
  return;
}




// Function: pax_dump_header_0 @ 0x28d20

undefined4 pax_dump_header_0(long param_1)

{
  long lVar1;
  char cVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  long in_FS_OFFSET;
  ulong local_60;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = FUN_0010d180();
  lVar6 = *(long *)(*(long *)(param_1 + 0x18) + 0x140);
  FUN_0011ca30("GNU.sparse.size",*(long *)(param_1 + 0x18),0);
  FUN_0011ca30("GNU.sparse.numblocks",*(undefined8 *)(param_1 + 0x18),0);
  cVar2 = FUN_0011bef0("GNU.sparse.map");
  if ((cVar2 == '\0') && (DAT_00184a80 != 0)) {
    FUN_0011ca30("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
    lVar7 = *(long *)(param_1 + 0x18);
    lVar1 = *(long *)(lVar7 + 8);
    uVar5 = FUN_0011c080(lVar7,"%d/GNUSparseFile.%p/%f",0);
    *(undefined8 *)(lVar7 + 8) = uVar5;
    FUN_0011cf40(*(long *)(param_1 + 0x18) + 0x168);
    lVar7 = *(long *)(param_1 + 0x18);
    local_60 = 0;
    if (*(long *)(lVar7 + 0x130) != 0) {
      while( true ) {
        uVar5 = FUN_00143090(*(undefined8 *)(lVar6 + local_60 * 0x10),local_58);
        FUN_0011cf50(*(long *)(param_1 + 0x18) + 0x168,uVar5);
        FUN_0011cf50(*(long *)(param_1 + 0x18) + 0x168,&DAT_0016079c);
        uVar5 = FUN_00143090(*(undefined8 *)(lVar6 + 8 + local_60 * 0x10),local_58);
        FUN_0011cf50(*(long *)(param_1 + 0x18) + 0x168,uVar5);
        lVar7 = *(long *)(param_1 + 0x18);
        local_60 = local_60 + 1;
        if (*(ulong *)(lVar7 + 0x130) <= local_60) break;
        if (local_60 != 0) {
          FUN_0011cf50(lVar7 + 0x168,&DAT_0016079c);
        }
      }
    }
    uVar3 = FUN_0011cfa0(lVar7 + 0x168,"GNU.sparse.map");
    if ((char)uVar3 == '\0') {
      free(*(void **)(*(long *)(param_1 + 0x18) + 8));
      *(long *)(*(long *)(param_1 + 0x18) + 8) = lVar1;
    }
    else {
      uVar5 = FUN_00128ce0(*(undefined8 *)(param_1 + 0x18));
      FUN_00112f60(*(undefined8 *)(param_1 + 0x18),uVar5,uVar4);
      if (lVar1 != 0) {
        free(*(void **)(*(long *)(param_1 + 0x18) + 8));
        *(long *)(*(long *)(param_1 + 0x18) + 8) = lVar1;
      }
    }
  }
  else {
    local_60 = 0;
    lVar6 = *(long *)(param_1 + 0x18);
    if (*(long *)(lVar6 + 0x130) != 0) {
      do {
        FUN_0011ca30("GNU.sparse.offset",lVar6,&local_60);
        FUN_0011ca30("GNU.sparse.numbytes",*(undefined8 *)(param_1 + 0x18),&local_60);
        lVar6 = *(long *)(param_1 + 0x18);
        local_60 = local_60 + 1;
      } while (local_60 < *(ulong *)(lVar6 + 0x130));
    }
    uVar3 = 1;
    uVar5 = FUN_00128ce0(lVar6);
    FUN_00112f60(*(undefined8 *)(param_1 + 0x18),uVar5,uVar4);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pax_dump_header_1 @ 0x28fd0

undefined8 pax_dump_header_1(long param_1)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  size_t sVar5;
  size_t sVar6;
  undefined8 uVar7;
  char *pcVar8;
  char *pcVar9;
  char *pcVar10;
  ulong uVar11;
  long lVar12;
  long lVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  long in_FS_OFFSET;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010d180();
  lVar12 = *(long *)(param_1 + 0x18);
  uVar2 = *(undefined8 *)(lVar12 + 8);
  puVar14 = *(undefined8 **)(lVar12 + 0x140);
  pcVar4 = (char *)FUN_00143090(*(undefined8 *)(lVar12 + 0x130));
  sVar5 = strlen(pcVar4);
  lVar13 = *(long *)(param_1 + 0x18);
  lVar12 = sVar5 + 1;
  if (*(long *)(lVar13 + 0x130) != 0) {
    uVar11 = 0;
    puVar15 = puVar14;
    do {
      uVar11 = uVar11 + 1;
      pcVar4 = (char *)FUN_00143090(*puVar15,local_58);
      sVar5 = strlen(pcVar4);
      pcVar4 = (char *)FUN_00143090(puVar15[1],local_58);
      sVar6 = strlen(pcVar4);
      lVar13 = *(long *)(param_1 + 0x18);
      lVar12 = sVar5 + 2 + lVar12 + sVar6;
      puVar15 = puVar15 + 2;
    } while (uVar11 < *(ulong *)(lVar13 + 0x130));
  }
  uVar11 = lVar12 + 0x3fe;
  if (-1 < (long)(lVar12 + 0x1ffU)) {
    uVar11 = lVar12 + 0x1ffU;
  }
  *(long *)(lVar13 + 0x118) = *(long *)(lVar13 + 0x118) + (uVar11 & 0xfffffffffffffe00);
  *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + (uVar11 & 0xfffffffffffffe00);
  FUN_0011ca30("GNU.sparse.major",lVar13,0);
  FUN_0011ca30("GNU.sparse.minor",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011ca30("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011ca30("GNU.sparse.realsize",*(undefined8 *)(param_1 + 0x18),0);
  lVar12 = *(long *)(param_1 + 0x18);
  uVar7 = FUN_0011c080(lVar12,"%d/GNUSparseFile.%p/%f",0);
  *(undefined8 *)(lVar12 + 8) = uVar7;
  lVar12 = *(long *)(param_1 + 0x18);
  pcVar4 = *(char **)(lVar12 + 8);
  sVar5 = strlen(pcVar4);
  if (100 < sVar5) {
    pcVar4[100] = '\0';
    lVar12 = *(long *)(param_1 + 0x18);
  }
  uVar7 = FUN_00128ce0(lVar12);
  FUN_00112f60(*(undefined8 *)(param_1 + 0x18),uVar7,uVar3);
  free(*(void **)(*(long *)(param_1 + 0x18) + 8));
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 8) = uVar2;
  pcVar8 = (char *)FUN_0010e130();
  pcVar9 = (char *)FUN_00143090(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130));
  pcVar10 = pcVar8 + 0x200;
  cVar1 = *pcVar9;
  pcVar4 = pcVar8;
  while (cVar1 != '\0') {
    if (pcVar4 == pcVar10) {
      FUN_0010d1f0(pcVar8);
      pcVar4 = (char *)FUN_0010e130();
      pcVar10 = pcVar4 + 0x200;
      pcVar8 = pcVar4;
    }
    cVar1 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    *pcVar4 = cVar1;
    pcVar4 = pcVar4 + 1;
    cVar1 = *pcVar9;
  }
  if (pcVar10 == pcVar4) {
    FUN_0010d1f0(pcVar8);
    pcVar8 = (char *)FUN_0010e130();
    pcVar4 = pcVar8;
  }
  *pcVar4 = '\n';
  pcVar4 = pcVar4 + 1;
  if (*(long *)(*(long *)(param_1 + 0x18) + 0x130) != 0) {
    uVar11 = 0;
    do {
      pcVar9 = (char *)FUN_00143090(*puVar14,local_58);
      pcVar10 = pcVar8 + 0x200;
      cVar1 = *pcVar9;
      while (cVar1 != '\0') {
        if (pcVar4 == pcVar10) {
          FUN_0010d1f0(pcVar8);
          pcVar4 = (char *)FUN_0010e130();
          pcVar10 = pcVar4 + 0x200;
          pcVar8 = pcVar4;
        }
        cVar1 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        *pcVar4 = cVar1;
        pcVar4 = pcVar4 + 1;
        cVar1 = *pcVar9;
      }
      if (pcVar10 == pcVar4) {
        FUN_0010d1f0(pcVar8);
        pcVar4 = (char *)FUN_0010e130();
        pcVar10 = pcVar4 + 0x200;
        pcVar8 = pcVar4;
      }
      *pcVar4 = '\n';
      pcVar9 = (char *)FUN_00143090(puVar14[1]);
      cVar1 = *pcVar9;
      while (pcVar4 = pcVar4 + 1, cVar1 != '\0') {
        if (pcVar4 == pcVar10) {
          FUN_0010d1f0(pcVar8);
          pcVar4 = (char *)FUN_0010e130();
          pcVar10 = pcVar4 + 0x200;
          pcVar8 = pcVar4;
        }
        cVar1 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        *pcVar4 = cVar1;
        cVar1 = *pcVar9;
      }
      if (pcVar10 == pcVar4) {
        FUN_0010d1f0(pcVar8);
        pcVar8 = (char *)FUN_0010e130();
        pcVar4 = pcVar8;
      }
      *pcVar4 = '\n';
      pcVar4 = pcVar4 + 1;
      uVar11 = uVar11 + 1;
      puVar14 = puVar14 + 2;
    } while (uVar11 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130));
  }
  memset(pcVar4,0,(size_t)(pcVar8 + (0x200 - (long)pcVar4)));
  FUN_0010d1f0(pcVar8);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return 1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pax_dump_header @ 0x29390

void pax_dump_header(long param_1)

{
  long lVar1;
  undefined4 uVar2;
  int iVar3;
  
  iVar3 = DAT_00184a84;
  uVar2 = DAT_00184a80;
  lVar1 = *(long *)(param_1 + 0x18);
  *(int *)(lVar1 + 0x124) = DAT_00184a84;
  *(undefined4 *)(lVar1 + 0x128) = uVar2;
  if (iVar3 == 0) {
    FUN_00128d20();
    return;
  }
  FUN_00128fd0();
  return;
}




// Function: oldgnu_store_sparse_info @ 0x293d0

void oldgnu_store_sparse_info(long param_1,ulong *param_2,long param_3,long param_4)

{
  ulong uVar1;
  long lVar2;
  
  lVar2 = *(long *)(param_1 + 0x18);
  uVar1 = *param_2;
  if (uVar1 < *(ulong *)(lVar2 + 0x130)) {
    do {
      param_4 = param_4 + -1;
      FUN_00112be0(*(undefined8 *)(uVar1 * 0x10 + *(long *)(lVar2 + 0x140)),param_3,0xc);
      lVar2 = param_3 + 0xc;
      param_3 = param_3 + 0x18;
      FUN_00112be0(*(undefined8 *)
                    (*param_2 * 0x10 + *(long *)(*(long *)(param_1 + 0x18) + 0x140) + 8),lVar2,0xc);
      lVar2 = *(long *)(param_1 + 0x18);
      uVar1 = *param_2 + 1;
      *param_2 = uVar1;
      if (*(ulong *)(lVar2 + 0x130) <= uVar1) {
        return;
      }
    } while (param_4 != 0);
  }
  return;
}




// Function: oldgnu_dump_header @ 0x29480

undefined8 oldgnu_dump_header(long param_1)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  undefined8 *puVar4;
  ulong uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  byte bVar7;
  ulong local_38;
  long local_30;
  
  bVar7 = 0;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_0010d180();
  lVar3 = FUN_001132d0(*(undefined8 *)(param_1 + 0x18));
  *(undefined1 *)(lVar3 + 0x9c) = 0x53;
  lVar1 = *(long *)(param_1 + 0x18);
  if (4 < *(ulong *)(lVar1 + 0x130)) {
    *(undefined1 *)(lVar3 + 0x1e2) = 1;
  }
  FUN_00112be0(*(undefined8 *)(lVar1 + 0x88),lVar3 + 0x1e3,0xc);
  FUN_00112be0(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x118),lVar3 + 0x7c,0xc);
  local_38 = 0;
  FUN_001293d0(param_1,&local_38,lVar3 + 0x182,4);
  uVar5 = local_38;
  lVar1 = *(long *)(param_1 + 0x18);
  *(bool *)(lVar3 + 0x1e2) = local_38 < *(ulong *)(lVar1 + 0x130);
  FUN_00112f60(lVar1,lVar3,uVar2);
  if (uVar5 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
    do {
      puVar4 = (undefined8 *)FUN_0010e130();
      *puVar4 = 0;
      puVar4[0x3f] = 0;
      puVar6 = (undefined8 *)((ulong)(puVar4 + 1) & 0xfffffffffffffff8);
      for (uVar5 = (ulong)(((int)puVar4 -
                           (int)(undefined8 *)((ulong)(puVar4 + 1) & 0xfffffffffffffff8)) + 0x200U
                          >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar6 = 0;
        puVar6 = puVar6 + (ulong)bVar7 * -2 + 1;
      }
      FUN_001293d0(param_1,&local_38,puVar4,0x15);
      uVar5 = local_38;
      if (local_38 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
        *(undefined1 *)(puVar4 + 0x3f) = 1;
      }
      FUN_0010d1f0(puVar4);
    } while (uVar5 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130));
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 1;
}




// Function: sparse_scan_file_raw @ 0x29600

undefined8 sparse_scan_file_raw(undefined4 *param_1)

{
  undefined4 uVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  undefined8 uVar5;
  long extraout_RDX;
  undefined1 *puVar6;
  long lVar7;
  long lVar8;
  long in_FS_OFFSET;
  undefined1 auVar9 [16];
  long local_258;
  long local_250;
  undefined1 local_248 [520];
  long local_40;
  
  lVar8 = 0;
  lVar2 = *(long *)(param_1 + 6);
  uVar1 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_258 = 0;
  *(undefined8 *)(lVar2 + 0x118) = 0;
  local_250 = 0;
  cVar3 = FUN_00127d50(param_1,0,0);
  if (cVar3 == '\0') {
LAB_001296d8:
    uVar5 = 0;
  }
  else {
    while( true ) {
      auVar9 = FUN_00124960(uVar1,local_248,0x200);
      lVar4 = auVar9._0_8_;
      lVar7 = auVar9._8_8_;
      if ((lVar4 == 0) || (lVar7 = local_250, lVar4 == -1)) break;
      cVar3 = FUN_00127df0(local_248,lVar4);
      if (cVar3 == '\0') {
        if (extraout_RDX == 0) {
          local_258 = lVar8;
        }
        *(long *)(lVar2 + 0x118) = *(long *)(lVar2 + 0x118) + lVar4;
        local_250 = extraout_RDX + lVar4;
        puVar6 = local_248;
LAB_001296c7:
        cVar3 = FUN_00127d50(param_1,1,puVar6);
        if (cVar3 == '\0') goto LAB_001296d8;
      }
      else if (extraout_RDX != 0) {
        FUN_00127e20(lVar2,&local_258);
        puVar6 = (undefined1 *)0x0;
        local_250 = 0;
        goto LAB_001296c7;
      }
      lVar8 = lVar8 + lVar4;
    }
    if (local_250 == 0) {
      local_258 = lVar8;
    }
    FUN_00127e20(lVar2,&local_258,lVar7);
    *(long *)(lVar2 + 0x118) = *(long *)(lVar2 + 0x118) + lVar4;
    uVar5 = FUN_00127d50(param_1,2,0);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: check_sparse_region @ 0x29770

undefined4 check_sparse_region(undefined4 *param_1,long param_2,long param_3)

{
  char cVar1;
  undefined4 uVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long extraout_RDX;
  long lVar6;
  long in_FS_OFFSET;
  undefined1 auStack_268 [32];
  undefined1 local_248 [520];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00128050();
  if ((char)uVar2 != '\0') {
    for (; param_2 < param_3; param_2 = param_2 + extraout_RDX) {
      lVar6 = param_3 - param_2;
      if (0x200 < lVar6) {
        lVar6 = 0x200;
      }
      lVar3 = FUN_0014af10(*param_1,local_248,lVar6);
      if (lVar3 == -1) {
        FUN_00124e00(**(undefined8 **)(param_1 + 6),param_2,lVar6);
LAB_00129819:
        uVar2 = 0;
        break;
      }
      if (lVar3 == 0) {
        uVar4 = dcgettext(0,"Size differs",5);
        FUN_00110b50(*(undefined8 *)(param_1 + 6),uVar4);
        goto LAB_00129819;
      }
      cVar1 = FUN_00127df0(local_248,lVar3,lVar3);
      if (cVar1 == '\0') {
        uVar4 = FUN_00142ff0(param_2,auStack_268);
        uVar5 = dcgettext(0,"File fragment at %s is not a hole",5);
        FUN_00110b50(*(undefined8 *)(param_1 + 6),uVar5,uVar4);
        goto LAB_00129819;
      }
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_data_region @ 0x298b0

char check_data_region(undefined4 *param_1,long param_2)

{
  int iVar1;
  size_t __n;
  void *__s1;
  undefined8 uVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  long in_FS_OFFSET;
  char local_251;
  undefined1 local_248 [520];
  long local_40;
  
  param_2 = param_2 * 0x10;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_251 = FUN_00128050(param_1,*(undefined8 *)
                                    (*(long *)(*(long *)(param_1 + 6) + 0x140) + param_2));
  if (local_251 != '\0') {
    lVar3 = *(long *)(*(long *)(*(long *)(param_1 + 6) + 0x140) + 8 + param_2);
    FUN_0010ccf0(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
    do {
      if (lVar3 < 1) goto LAB_001298ff;
      lVar4 = 0x200;
      if (lVar3 < 0x201) {
        lVar4 = lVar3;
      }
      __s1 = (void *)FUN_0010e130();
      if (__s1 == (void *)0x0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar2 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar2);
        DAT_00184d58 = 2;
        goto LAB_00129a20;
      }
      FUN_0010d1f0(__s1);
      *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
      __n = FUN_0014af10(*param_1,local_248,lVar4);
      if (__n == 0xffffffffffffffff) {
        plVar5 = (long *)(param_2 + (*(undefined8 **)(param_1 + 6))[0x28]);
        FUN_00124e00(**(undefined8 **)(param_1 + 6),(plVar5[1] + *plVar5) - lVar3,lVar4);
        goto LAB_00129a20;
      }
      if (__n == 0) {
        uVar2 = dcgettext(0,"Size differs",5);
        FUN_00110b50(&DAT_00184840,uVar2);
        goto LAB_00129a20;
      }
      lVar3 = lVar3 - __n;
      FUN_0010ccf0(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
      iVar1 = memcmp(__s1,local_248,__n);
    } while (iVar1 == 0);
    uVar2 = dcgettext(0,"Contents differ",5);
    FUN_00110b50(*(undefined8 *)(param_1 + 6),uVar2);
LAB_00129a20:
    local_251 = '\0';
  }
LAB_001298ff:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_251;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_dump_file @ 0x29cd0

byte sparse_dump_file(int param_1,long param_2)

{
  char cVar1;
  byte bVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  int local_58;
  undefined1 local_54;
  long local_48;
  long local_40;
  long local_38;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00127f70(&local_58);
  bVar2 = 3;
  if (cVar1 != '\0') {
    local_54 = 1;
    local_58 = param_1;
    local_40 = param_2;
    cVar1 = FUN_00129c60(&local_58);
    if (cVar1 != '\0') {
      if ((*(long *)(local_38 + 0x38) == 0) || (FUN_00129cb0(&local_58), param_1 < 0)) {
LAB_00129d4a:
        FUN_00113c80(*(long *)(local_40 + 0x118) - local_48);
        bVar2 = FUN_00127d30(&local_58);
        bVar2 = bVar2 ^ 1;
        goto LAB_00129d69;
      }
      FUN_0010cc00(*(undefined8 *)(local_40 + 8),*(undefined8 *)(local_40 + 0x88),
                   *(long *)(local_40 + 0x118) - local_48);
      uVar3 = 0;
      do {
        if (*(ulong *)(local_40 + 0x130) <= uVar3) goto LAB_00129d4a;
        cVar1 = FUN_00127d70(&local_58,uVar3);
        uVar3 = uVar3 + 1;
      } while (cVar1 != '\0');
    }
    FUN_00113c80(*(long *)(local_40 + 0x118) - local_48);
    FUN_00127d30(&local_58);
    bVar2 = 1;
  }
LAB_00129d69:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar2;
}




// Function: sparse_member_p @ 0x29e10

void sparse_member_p(undefined8 param_1)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined1 auStack_58 [24];
  undefined8 local_40;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00127f70(auStack_58);
  if (cVar1 != '\0') {
    local_40 = param_1;
    FUN_00127d10(auStack_58);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_fixup_header @ 0x29e70

void sparse_fixup_header(undefined8 param_1)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined1 auStack_58 [24];
  undefined8 local_40;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00127f70(auStack_58);
  if (cVar1 != '\0') {
    local_40 = param_1;
    FUN_00127dd0(auStack_58);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_extract_file @ 0x29ed0

byte sparse_extract_file(int param_1,long param_2,long *param_3)

{
  ulong uVar1;
  char cVar2;
  byte bVar3;
  __off_t _Var4;
  long in_FS_OFFSET;
  int local_68;
  undefined1 local_64;
  undefined8 local_60;
  long local_58;
  long local_50;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_00127f70(&local_68);
  bVar3 = 3;
  if (cVar2 != '\0') {
    local_68 = param_1;
    local_50 = param_2;
    _Var4 = lseek(param_1,0,0);
    local_60 = 0;
    local_64 = _Var4 == 0;
    cVar2 = FUN_00127db0(&local_68);
    uVar1 = 0;
    while (cVar2 != '\0') {
      if (*(ulong *)(local_50 + 0x130) <= uVar1) {
        *param_3 = *(long *)(local_50 + 0x118) - local_58;
        bVar3 = FUN_00127d30(&local_68);
        bVar3 = bVar3 ^ 1;
        goto LAB_00129f87;
      }
      cVar2 = FUN_00127d90(&local_68,uVar1);
      uVar1 = uVar1 + 1;
    }
    *param_3 = *(long *)(local_50 + 0x118) - local_58;
    FUN_00127d30(&local_68);
    bVar3 = 1;
  }
LAB_00129f87:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_skip_file @ 0x29fe0

byte sparse_skip_file(long param_1)

{
  char cVar1;
  byte bVar2;
  byte bVar3;
  long in_FS_OFFSET;
  undefined4 local_58 [4];
  long local_48;
  long local_40;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00127f70(local_58);
  bVar2 = 3;
  if (cVar1 != '\0') {
    local_58[0] = 0xffffffff;
    local_40 = param_1;
    bVar2 = FUN_00127db0(local_58);
    FUN_00122b10(*(long *)(local_40 + 0x118) - local_48);
    bVar3 = FUN_00127d30(local_58);
    bVar2 = bVar3 & bVar2 ^ 1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_diff_file @ 0x2a070

undefined4 sparse_diff_file(undefined4 param_1,long param_2)

{
  char cVar1;
  undefined4 uVar2;
  long *plVar3;
  ulong uVar4;
  long lVar5;
  long in_FS_OFFSET;
  undefined4 local_68;
  undefined1 local_64;
  long local_58;
  long local_50;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00127f70(&local_68);
  if ((char)uVar2 != '\0') {
    local_64 = 1;
    local_68 = param_1;
    local_50 = param_2;
    uVar2 = FUN_00127db0(&local_68);
    FUN_0010ccb0(param_2);
    if ((char)uVar2 != '\0') {
      lVar5 = 0;
      uVar4 = 0;
      do {
        if (*(ulong *)(local_50 + 0x130) <= uVar4) goto LAB_0012a140;
        cVar1 = FUN_00129770(&local_68,lVar5,
                             *(undefined8 *)(*(long *)(local_50 + 0x140) + uVar4 * 0x10));
        if (cVar1 == '\0') break;
        cVar1 = FUN_001298b0(&local_68,uVar4);
        plVar3 = (long *)(uVar4 * 0x10 + *(long *)(local_50 + 0x140));
        lVar5 = plVar3[1] + *plVar3;
        uVar4 = uVar4 + 1;
      } while (cVar1 != '\0');
    }
    uVar2 = 0;
    FUN_00122b10(*(long *)(local_50 + 0x118) - local_58);
LAB_0012a140:
    FUN_0010ccd0();
    FUN_00127d30(&local_68);
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: find_compression_suffix @ 0x2a190

undefined ** find_compression_suffix(char *param_1,long *param_2)

{
  int iVar1;
  char *pcVar2;
  undefined *__n;
  size_t sVar3;
  undefined **ppuVar4;
  undefined *__s1;
  
  pcVar2 = strrchr(param_1,0x2e);
  if (pcVar2 != (char *)0x0) {
    __n = (undefined *)strlen(pcVar2 + 1);
    if (PTR_s_tar_00161ae8_4_00181920 != (undefined *)0x0) {
      ppuVar4 = &PTR_s_tar_00161ae8_4_00181920;
      __s1 = PTR_s_tar_00161ae8_4_00181920;
      do {
        if ((ppuVar4[1] == __n) && (iVar1 = memcmp(__s1,pcVar2 + 1,(size_t)__n), iVar1 == 0)) {
          if (param_2 == (long *)0x0) {
            return ppuVar4;
          }
          sVar3 = strlen(param_1);
          *param_2 = (sVar3 - (long)__n) + -1;
          return ppuVar4;
        }
        __s1 = ppuVar4[3];
        ppuVar4 = ppuVar4 + 3;
      } while (__s1 != (undefined *)0x0);
    }
  }
  return (undefined **)0x0;
}




// Function: find_compression_program @ 0x2a240

undefined8 find_compression_program(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_0012a190(param_1,0);
  if (lVar1 != 0) {
    param_2 = *(undefined8 *)(lVar1 + 0x10);
  }
  return param_2;
}




// Function: set_compression_program_by_suffix @ 0x2a260

void set_compression_program_by_suffix(void)

{
  long lVar1;
  
  lVar1 = FUN_0012a240();
  if (lVar1 != 0) {
    DAT_00184b70 = lVar1;
  }
  return;
}




// Function: strip_compression_suffix @ 0x2a280

void * strip_compression_suffix(void *param_1)

{
  int iVar1;
  undefined8 *puVar2;
  void *__dest;
  long in_FS_OFFSET;
  ulong local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  puVar2 = (undefined8 *)FUN_0012a190(param_1,&local_38);
  __dest = (void *)0x0;
  if (puVar2 != (undefined8 *)0x0) {
    if (local_38 < 5) {
      __dest = (void *)0x0;
      if (local_38 == 0) goto LAB_0012a303;
    }
    else {
      iVar1 = strncmp((char *)((long)param_1 + (local_38 - 4)),".tar",4);
      if ((iVar1 == 0) && (*(char *)*puVar2 != 't')) {
        local_38 = local_38 - 4;
      }
    }
    __dest = (void *)FUN_0014c5b0(local_38 + 1);
    memcpy(__dest,param_1,local_38);
    *(undefined1 *)((long)__dest + local_38) = 0;
  }
LAB_0012a303:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return __dest;
}




// Function: is_regular_file @ 0x2a340

bool is_regular_file(char *param_1)

{
  int iVar1;
  int *piVar2;
  long in_FS_OFFSET;
  bool bVar3;
  stat sStack_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat(param_1,&sStack_a8);
  if (iVar1 == 0) {
    bVar3 = (sStack_a8.st_mode & 0xf000) == 0x8000;
  }
  else {
    piVar2 = __errno_location();
    bVar3 = *piVar2 == 2;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xexec @ 0x2a3b0

void xexec(ulong param_1)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  int iVar4;
  long in_FS_OFFSET;
  char *local_38;
  undefined *local_30;
  ulong local_28;
  undefined8 local_20;
  undefined8 local_10;
  
  iVar4 = (int)&local_38;
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_38 = "/bin/sh";
  local_30 = &DAT_00163997;
  local_20 = 0;
  local_28 = param_1;
  execv("/bin/sh",&local_38);
  FUN_001326c0();
  if ((int)param_1 == iVar4) {
    return;
  }
  iVar1 = close(iVar4);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    if (iVar1 != 9) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Cannot close",5);
      error(0,iVar1,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
  }
  iVar1 = dup((int)param_1);
  if (iVar4 != iVar1) {
    if (iVar1 < 0) {
      piVar2 = __errno_location();
      iVar4 = *piVar2;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Cannot dup",5);
      error(0,iVar4,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
                    /* WARNING: Subroutine does not return */
    abort();
  }
  FUN_0010d2c0(param_1 & 0xffffffff);
  return;
}




// Function: xdup2 @ 0x2a400

void xdup2(int param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  if (param_1 == param_2) {
    return;
  }
  iVar1 = close(param_2);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    if (iVar1 != 9) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Cannot close",5);
      error(0,iVar1,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
  }
  iVar1 = dup(param_1);
  if (param_2 != iVar1) {
    if (iVar1 < 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Cannot dup",5);
      error(0,iVar1,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
                    /* WARNING: Subroutine does not return */
    abort();
  }
  FUN_0010d2c0(param_1);
  return;
}




// Function: wait_for_grandchild @ 0x2a4c0

void wait_for_grandchild(__pid_t param_1)

{
  __pid_t _Var1;
  int *piVar2;
  uint __status;
  long in_FS_OFFSET;
  uint local_24;
  undefined8 local_20;
  
  local_20 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  do {
    _Var1 = waitpid(param_1,(int *)&local_24,0);
    if (_Var1 != -1) goto LAB_0012a4fb;
    piVar2 = __errno_location();
  } while (*piVar2 == 4);
  FUN_00132df0(DAT_00184b70);
LAB_0012a4fb:
  __status = local_24 >> 8 & 0xff;
  if ('\x01' < (char)(((byte)local_24 & 0x7f) + 1)) {
    raise(local_24 & 0x7f);
    __status = 0;
  }
                    /* WARNING: Subroutine does not return */
  exit(__status);
}




// Function: run_decompress_program @ 0x2a530

void run_decompress_program(void)

{
  int iVar1;
  long lVar2;
  long lVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_12c [4];
  long local_128;
  char **local_120;
  long local_118;
  undefined8 local_b8;
  undefined8 local_40;
  
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_118 = 1;
  local_b8 = environ;
  lVar2 = FUN_0010cdf0(local_12c);
  if (lVar2 != 0) {
    uVar6 = 0x2080e06;
    while( true ) {
      iVar1 = FUN_001382c0(lVar2,&local_128,uVar6);
      if (iVar1 != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar6 = FUN_001354b0(&local_128);
        uVar5 = dcgettext(0,"cannot split string \'%s\': %s",5);
        error(0,0,uVar5,lVar2,uVar6);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
      memmove(local_120,local_120 + local_118,local_128 * 8);
      local_120[local_128] = "-d";
      execvp(*local_120,local_120);
      local_120[local_128] = (char *)0x0;
      lVar3 = FUN_0010ce40(local_12c);
      if (lVar3 == 0) break;
      if ((DAT_00183b80._2_1_ & 8) != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar6 = dcgettext(0,"cannot run %s",5);
        piVar4 = __errno_location();
        error(0,*piVar4,uVar6,lVar2);
        if ((DAT_00183b80._2_1_ & 8) != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar6 = dcgettext(0,"trying %s",5);
          error(0,0,uVar6,lVar3);
        }
      }
      uVar6 = 0x2080e0e;
      lVar2 = lVar3;
    }
    FUN_001326c0(lVar2);
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar6 = dcgettext(0,"unable to run decompression program",5);
  error(0,0,uVar6);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: str_to_env @ 0x2a720

void str_to_env(char *param_1,char *param_2)

{
  int iVar1;
  
  if (param_2 == (char *)0x0) {
    unsetenv(param_1);
    return;
  }
  iVar1 = setenv(param_1,param_2,1);
  if (iVar1 == 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00116420();
}




// Function: time_to_env @ 0x2a750

void time_to_env(char *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  char *__value;
  long in_FS_OFFSET;
  undefined1 auStack_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __value = (char *)FUN_00124270(param_2,param_3,auStack_38);
  iVar1 = setenv(param_1,__value,1);
  if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_00116420();
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dec_to_env @ 0x2a7b0

void dec_to_env(char *param_1,undefined8 param_2)

{
  int iVar1;
  char *__value;
  long in_FS_OFFSET;
  undefined1 auStack_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __value = (char *)FUN_00143090(param_2,auStack_28);
  iVar1 = setenv(param_1,__value,1);
  if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_00116420();
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: stat_to_env @ 0x2a8f0

void stat_to_env(undefined8 param_1,char param_2,long param_3)

{
  int iVar1;
  uint uVar2;
  undefined8 uVar3;
  
  FUN_0012a720("TAR_VERSION",&DAT_00161b0c);
  FUN_0012a720("TAR_ARCHIVE",*DAT_00184828);
  FUN_0012a7b0("TAR_VOLUME",((long)DAT_00184828 - DAT_00184838 >> 3) + 1);
  FUN_0012a7b0("TAR_BLOCKING_FACTOR",(long)DAT_00184ba0);
  iVar1 = DAT_00184400;
  if (DAT_00184400 == 0) {
    iVar1 = DAT_00184ba4;
  }
  uVar3 = FUN_0012c230(iVar1);
  FUN_0012a720("TAR_FORMAT",uVar3);
  FUN_0012a810((int)param_2);
  FUN_0012a870(*(undefined4 *)(param_3 + 0x70));
  FUN_0012a720("TAR_FILENAME",param_1);
  FUN_0012a720("TAR_REALNAME",*(undefined8 *)(param_3 + 8));
  FUN_0012a720("TAR_UNAME",*(undefined8 *)(param_3 + 0x20));
  FUN_0012a720("TAR_GNAME",*(undefined8 *)(param_3 + 0x28));
  FUN_0012a750("TAR_ATIME",*(undefined8 *)(param_3 + 0xe8),*(undefined8 *)(param_3 + 0xf0));
  FUN_0012a750("TAR_MTIME",*(undefined8 *)(param_3 + 0xf8),*(undefined8 *)(param_3 + 0x100));
  FUN_0012a750("TAR_CTIME",*(undefined8 *)(param_3 + 0x108),*(undefined8 *)(param_3 + 0x110));
  FUN_0012a7b0("TAR_SIZE",*(undefined8 *)(param_3 + 0x88));
  FUN_0012a7b0("TAR_UID",*(undefined4 *)(param_3 + 0x74));
  FUN_0012a7b0("TAR_GID",*(undefined4 *)(param_3 + 0x78));
  if (param_2 != 'h') {
    if (param_2 < 'i') {
      if ((byte)(param_2 + 0x9eU) < 2) {
        uVar2 = gnu_dev_minor(*(ulonglong *)(param_3 + 0x80));
        FUN_0012a7b0("TAR_MINOR",uVar2);
        uVar2 = gnu_dev_major(*(ulonglong *)(param_3 + 0x80));
        FUN_0012a7b0("TAR_MAJOR",uVar2);
        unsetenv("TAR_LINKNAME");
        return;
      }
    }
    else if (param_2 == 'l') goto LAB_0012aae0;
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    unsetenv("TAR_LINKNAME");
    return;
  }
LAB_0012aae0:
  unsetenv("TAR_MINOR");
  unsetenv("TAR_MAJOR");
  FUN_0012a720("TAR_LINKNAME",*(undefined8 *)(param_3 + 0x18));
  return;
}




// Function: sys_get_archive_stat @ 0x2ab10

bool sys_get_archive_stat(void)

{
  int iVar1;
  
  iVar1 = fstat(DAT_00184a34,(stat *)&DAT_00184740);
  return iVar1 == 0;
}




// Function: sys_file_is_archive @ 0x2ab40

bool sys_file_is_archive(long param_1)

{
  if ((DAT_00184818 != 0) && (DAT_00184818 == *(long *)(param_1 + 0x58))) {
    return *(long *)(param_1 + 0x60) == DAT_00184810;
  }
  return false;
}




// Function: sys_save_archive_dev_ino @ 0x2ab80

void sys_save_archive_dev_ino(void)

{
  if ((DAT_00184a34 < 0x40000000) && ((DAT_00184758 & 0xf000) == 0x8000)) {
    DAT_00184818 = DAT_00184740;
    DAT_00184810 = DAT_00184748;
    return;
  }
  DAT_00184818 = 0;
  return;
}




// Function: sys_detect_dev_null_output @ 0x2abd0

void sys_detect_dev_null_output(void)

{
  int iVar1;
  long in_FS_OFFSET;
  bool bVar2;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = strcmp((char *)*DAT_00184838,"/dev/null");
  bVar2 = true;
  if (iVar1 != 0) {
    if ((DAT_00184a34 < 0x40000000) && ((DAT_00184758 & 0xf000) == 0x2000)) {
      iVar1 = stat("/dev/null",&local_a8);
      if ((iVar1 == 0) && (DAT_00184740 == local_a8.st_dev)) {
        bVar2 = DAT_00184748 == local_a8.st_ino;
        goto LAB_0012ac35;
      }
    }
    bVar2 = false;
  }
LAB_0012ac35:
  DAT_00184a30 = bVar2;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_wait_for_child @ 0x2aca0

void sys_wait_for_child(int param_1,char param_2)

{
  __pid_t _Var1;
  int *piVar2;
  undefined8 uVar3;
  uint uVar4;
  long in_FS_OFFSET;
  uint local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != 0) {
    do {
      _Var1 = waitpid(param_1,(int *)&local_24,0);
      if (_Var1 != -1) {
        if ((char)(((byte)local_24 & 0x7f) + 1) < '\x02') goto LAB_0012ad28;
        goto LAB_0012acff;
      }
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    FUN_00132df0(DAT_00184b70);
    if ((char)(((byte)local_24 & 0x7f) + 1) < '\x02') {
LAB_0012ad28:
      if ((char)(local_24 >> 8) != '\0') {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar4 = local_24 >> 8 & 0xff;
        uVar3 = dcgettext(0,"Child returned status %d",5);
LAB_0012adb7:
        error(0,0,uVar3,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
    }
    else {
LAB_0012acff:
      uVar4 = local_24 & 0x7f;
      if ((uVar4 != 0xd) || (param_2 != '\0')) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar3 = dcgettext(0,"Child died with signal %d",5);
        goto LAB_0012adb7;
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_spawn_shell @ 0x2add0

void sys_spawn_shell(void)

{
  int __pid;
  __pid_t _Var1;
  char *__file;
  int *piVar2;
  long in_FS_OFFSET;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __file = getenv("SHELL");
  if (__file == (char *)0x0) {
    __file = "/bin/sh";
  }
  __pid = FUN_00124f80();
  if (__pid == 0) {
    execlp(__file,"-sh",&DAT_00163ad0,0);
    FUN_001326c0(__file);
  }
  else {
    do {
      _Var1 = waitpid(__pid,&local_24,0);
      if (_Var1 != -1) goto LAB_0012ae3b;
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    FUN_00132df0(__file);
LAB_0012ae3b:
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_compare_uid @ 0x2ae90

undefined4 sys_compare_uid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x1c) >> 8),
                  *(int *)(param_1 + 0x1c) == *(int *)(param_2 + 0x1c));
}




// Function: sys_compare_gid @ 0x2aea0

undefined4 sys_compare_gid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x20) >> 8),
                  *(int *)(param_1 + 0x20) == *(int *)(param_2 + 0x20));
}




// Function: sys_compare_links @ 0x2aeb0

undefined8 sys_compare_links(long *param_1,long *param_2)

{
  if (*param_2 != *param_1) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_1[1] >> 8),param_2[1] == param_1[1]);
}




// Function: sys_truncate @ 0x2aed0

int sys_truncate(int param_1)

{
  int iVar1;
  __off_t __length;
  
  __length = lseek(param_1,0,1);
  if (-1 < __length) {
    iVar1 = ftruncate(param_1,__length);
    return iVar1;
  }
  return -1;
}




// Function: sys_write_archive_buffer @ 0x2af00

void sys_write_archive_buffer(void)

{
  if (0x3fffffff < DAT_00184a34) {
    FUN_00133ea0(DAT_00184a34 + -0x40000000);
    return;
  }
  FUN_00141120(DAT_00184a34,DAT_00183f58,DAT_00184b98);
  return;
}




// Function: sys_child_open_for_compress @ 0x2af40

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int sys_child_open_for_compress(void)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  long lVar5;
  int *piVar6;
  void *pvVar7;
  char *pcVar8;
  long lVar9;
  long in_FS_OFFSET;
  undefined4 local_30;
  int local_2c;
  undefined4 local_28;
  undefined4 local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  signal(0xd,(__sighandler_t)0x1);
  FUN_00124fc0(&local_30);
  iVar1 = FUN_00124f80();
  if (0 < iVar1) goto LAB_0012b076;
  uVar3 = dcgettext(0,"tar (child)",5);
  FUN_00148760(uVar3);
  signal(0xd,(__sighandler_t)0x0);
  FUN_0012a400(local_30,0);
  FUN_0010d2c0(local_2c);
  piVar4 = DAT_00184838;
  if (DAT_00184d70 == '\0') {
    _DAT_00184d78 = strchr(*(char **)DAT_00184838,0x3a);
    pcVar8 = *(char **)piVar4;
    if (((_DAT_00184d78 == (char *)0x0) || (_DAT_00184d78 <= pcVar8)) ||
       (pvVar7 = memchr(pcVar8,0x2f,(long)_DAT_00184d78 - (long)pcVar8), pvVar7 != (void *)0x0))
    goto LAB_0012aff6;
  }
  else {
    pcVar8 = *(char **)DAT_00184838;
LAB_0012aff6:
    iVar1 = FUN_0012a340(pcVar8);
    if (iVar1 != 0) {
      if (DAT_00184b84 != '\0') goto LAB_0012b246;
      goto LAB_0012b010;
    }
  }
  FUN_00124fc0(&local_28);
  iVar1 = FUN_00124f80();
  if (iVar1 == 0) {
    uVar3 = dcgettext(0,"tar (grandchild)",5);
    FUN_00148760(uVar3);
    FUN_0012a400(local_24,1);
    FUN_0010d2c0(local_28);
    while( true ) {
      do {
        FUN_0012a3b0(DAT_00184b70);
LAB_0012b246:
        FUN_00124710(*(undefined8 *)DAT_00184838,1);
LAB_0012b010:
        pcVar8 = *(char **)DAT_00184838;
        iVar1 = strcmp(pcVar8,"-");
      } while (iVar1 == 0);
      DAT_00184a34 = creat(pcVar8,0x1b6);
      if (DAT_00184a34 < 0) break;
      FUN_0012a400(DAT_00184a34,1);
    }
    piVar4 = __errno_location();
    iVar1 = *piVar4;
    if (DAT_00184b84 != '\0') goto LAB_0012b2b7;
  }
  else {
    FUN_0012a400(local_28,0);
    FUN_0010d2c0(local_24);
    piVar4 = DAT_00184838;
    pcVar8 = *(char **)DAT_00184838;
    iVar2 = strcmp(pcVar8,"-");
    if (iVar2 == 0) {
      DAT_00184a34 = 1;
      piVar6 = DAT_00184b98;
    }
    else {
      if (DAT_00184d70 == '\0') {
        _DAT_00184d78 = strchr(pcVar8,0x3a);
        pcVar8 = *(char **)piVar4;
        if (((_DAT_00184d78 == (char *)0x0) || (_DAT_00184d78 <= pcVar8)) ||
           (pvVar7 = memchr(pcVar8,0x2f,(long)_DAT_00184d78 - (long)pcVar8), pvVar7 != (void *)0x0))
        goto LAB_0012b25f;
        DAT_00184a34 = FUN_001338d0(pcVar8,0x41,0x40000000,DAT_00184ab0);
      }
      else {
LAB_0012b25f:
        DAT_00184a34 = creat(pcVar8,0x1b6);
      }
      piVar6 = DAT_00184b98;
      if (DAT_00184a34 < 0) {
        FUN_001327e0(*(undefined8 *)DAT_00184838);
        goto LAB_0012b290;
      }
    }
    while (piVar6 != (int *)0x0) {
      piVar4 = (int *)0x0;
      lVar9 = DAT_00183f58;
      do {
        lVar5 = FUN_0014af10(0,lVar9,(long)piVar6 - (long)piVar4);
        if (lVar5 == -1) {
          FUN_001329b0(DAT_00184b70);
          goto LAB_0012b2cd;
        }
        if (lVar5 == 0) {
          if (piVar4 == (int *)0x0) goto LAB_0012b2b0;
          memset((void *)(DAT_00183f58 + (long)piVar4),0,(long)DAT_00184b98 - (long)piVar4);
          piVar6 = (int *)FUN_0012af00();
          if (DAT_00184b98 == piVar6) goto LAB_0012b2b0;
          goto LAB_0012b1b3;
        }
        piVar4 = (int *)((long)piVar4 + lVar5);
        lVar9 = lVar9 + lVar5;
        piVar6 = DAT_00184b98;
      } while (piVar4 < DAT_00184b98);
LAB_0012b290:
      piVar6 = (int *)FUN_0012af00();
      if (DAT_00184b98 != piVar6) {
LAB_0012b1b3:
                    /* WARNING: Subroutine does not return */
        FUN_0010d6d0(piVar6);
      }
    }
LAB_0012b2b0:
    FUN_0012a4c0(iVar1);
LAB_0012b2b7:
    FUN_001245d0();
  }
  piVar6 = DAT_00184838;
  *piVar4 = iVar1;
  iVar1 = FUN_001327e0(*(undefined8 *)piVar6);
LAB_0012b076:
  DAT_00184a34 = local_2c;
  FUN_0010d2c0(local_30);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
LAB_0012b2cd:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_child_open_for_uncompress @ 0x2b310

/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int sys_child_open_for_uncompress(void)

{
  int iVar1;
  uint uVar2;
  undefined8 uVar3;
  void *pvVar4;
  char *pcVar5;
  char *pcVar6;
  undefined8 extraout_RDX;
  undefined8 *puVar7;
  char *__s1;
  char cVar8;
  ulong uVar9;
  ulong unaff_R12;
  char *pcVar10;
  long in_FS_OFFSET;
  undefined4 uStack_70;
  int iStack_6c;
  long lStack_68;
  undefined8 *puStack_60;
  char *pcStack_58;
  ulong uStack_50;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  uint local_34;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uStack_50 = 0x12b338;
  FUN_00124fc0(&local_40);
  uStack_50 = 0x12b33d;
  iVar1 = FUN_00124f80();
  if (0 < iVar1) {
    DAT_00184a34 = local_40;
    uStack_50 = 0x12b50e;
    FUN_0010d2c0(local_3c);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar1;
    }
                    /* WARNING: Subroutine does not return */
    uStack_50 = 0x12b5ff;
    __stack_chk_fail();
  }
  uStack_50 = 0x12b358;
  uVar3 = dcgettext(0,"tar (child)",5);
  uStack_50 = 0x12b367;
  FUN_00148760(uVar3);
  uStack_50 = 0x12b373;
  signal(0xd,(__sighandler_t)0x0);
  uStack_50 = 0x12b381;
  FUN_0012a400(local_3c,1);
  uStack_50 = 0x12b38a;
  FUN_0010d2c0(local_40);
  puVar7 = DAT_00184838;
  __s1 = (char *)*DAT_00184838;
  uStack_50 = 0x12b39f;
  iVar1 = strcmp(__s1,"-");
  if (iVar1 != 0) {
    if (DAT_00184d70 == '\0') {
      uStack_50 = 0x12b3b9;
      _DAT_00184d78 = strchr(__s1,0x3a);
      __s1 = (char *)*puVar7;
      if ((_DAT_00184d78 != (char *)0x0) && (__s1 < _DAT_00184d78)) {
        uStack_50 = 0x12b3e0;
        pvVar4 = memchr(__s1,0x2f,(long)_DAT_00184d78 - (long)__s1);
        if (pvVar4 == (void *)0x0) goto LAB_0012b3f8;
      }
    }
    uStack_50 = 0x12b3f0;
    iVar1 = FUN_0012a340(__s1);
    if (iVar1 != 0) goto LAB_0012b5b2;
  }
LAB_0012b3f8:
  uStack_50 = 0x12b402;
  FUN_00124fc0(&local_38);
  uStack_50 = 0x12b407;
  uVar2 = FUN_00124f80();
  unaff_R12 = (ulong)uVar2;
  if (uVar2 != 0) {
    uStack_50 = 0x12b420;
    FUN_0012a400(local_34,1);
    uStack_50 = 0x12b429;
    FUN_0010d2c0(local_38);
    puVar7 = DAT_00184838;
    __s1 = (char *)*DAT_00184838;
    uStack_50 = 0x12b43e;
    iVar1 = strcmp(__s1,"-");
    if (iVar1 != 0) goto LAB_0012b542;
    DAT_00184a34 = 0;
    do {
      while( true ) {
        uStack_50 = 0x12b455;
        FUN_0010cd10();
        while( true ) {
          if (DAT_00184a34 < 0x40000000) {
            uStack_50 = 0x12b49a;
            pcVar5 = (char *)FUN_0014af10();
            pcVar10 = DAT_00183f58;
          }
          else {
            uStack_50 = 0x12b46b;
            pcVar5 = (char *)FUN_00133da0(DAT_00184a34 + -0x40000000,DAT_00183f58,DAT_00184b98);
            pcVar10 = DAT_00183f58;
          }
          DAT_00183f58 = pcVar10;
          if (pcVar5 != (char *)0xffffffffffffffff) break;
          uStack_50 = 0x12b479;
          FUN_0010d300();
        }
        if (pcVar5 == (char *)0x0) break;
        do {
          __s1 = (char *)0x200;
          if (pcVar5 < (char *)0x201) {
            __s1 = pcVar5;
          }
          uStack_50 = 0x12b4d4;
          pcVar6 = (char *)FUN_00141120(1,pcVar10,__s1);
          if (pcVar6 != __s1) {
            uStack_50 = 0x12b4e5;
            FUN_00132e10(DAT_00184b70);
          }
          pcVar10 = pcVar10 + (long)__s1;
          pcVar5 = pcVar5 + -(long)__s1;
        } while (pcVar5 != (char *)0x0);
      }
      uStack_50 = 0x12b53a;
      FUN_0010d2c0(1);
      uStack_50 = 0x12b542;
      FUN_0012a4c0(unaff_R12 & 0xffffffff);
      puVar7 = (undefined8 *)0x0;
LAB_0012b542:
      if (DAT_00184d70 == '\0') {
        uStack_50 = 0x12b558;
        _DAT_00184d78 = strchr(__s1,0x3a);
        __s1 = (char *)*puVar7;
        if ((_DAT_00184d78 != (char *)0x0) && (__s1 < _DAT_00184d78)) {
          uStack_50 = 0x12b57f;
          pvVar4 = memchr(__s1,0x2f,(long)_DAT_00184d78 - (long)__s1);
          if (pvVar4 == (void *)0x0) goto LAB_0012b5e2;
        }
      }
      uStack_50 = 0x12b595;
      DAT_00184a34 = open(__s1,0,0x1b6);
      while (DAT_00184a34 < 0) {
        do {
          uStack_50 = 0x12b5b2;
          FUN_001327e0(*DAT_00184838);
LAB_0012b5b2:
          uStack_50 = 0x12b5ca;
          DAT_00184a34 = open((char *)*DAT_00184838,0,0x1b6);
        } while (DAT_00184a34 < 0);
        uStack_50 = 0x12b5dd;
        FUN_0012a400(DAT_00184a34);
        uStack_50 = 0x12b5e2;
        FUN_0012a530();
LAB_0012b5e2:
        uStack_50 = 0x12b5f8;
        DAT_00184a34 = FUN_001338d0(__s1,0,0x40000000,DAT_00184ab0);
      }
    } while( true );
  }
  uStack_50 = 0x12b612;
  uVar3 = dcgettext(0,"tar (grandchild)",5);
  uStack_50 = 0x12b61a;
  FUN_00148760(uVar3);
  cVar8 = '\0';
  uStack_50 = 0x12b625;
  FUN_0012a400(local_38,0);
  uVar9 = (ulong)local_34;
  uStack_50 = 0x12b62e;
  FUN_0010d2c0(uVar9);
  uStack_50 = 0x12b633;
  FUN_0012a530();
  puStack_60 = puVar7;
  lStack_68 = *(long *)(in_FS_OFFSET + 0x28);
  pcStack_58 = __s1;
  uStack_50 = unaff_R12;
  FUN_00124fc0(&uStack_70);
  DAT_00184728 = signal(0xd,(__sighandler_t)0x1);
  DAT_00184730 = FUN_00124f80();
  if (DAT_00184730 == 0) {
    FUN_0012a400(uStack_70,0);
    FUN_0010d2c0(iStack_6c);
    FUN_0012a8f0(uVar9,(int)cVar8,extraout_RDX);
    FUN_0012a3b0(DAT_00184a60);
  }
  else {
    FUN_0010d2c0(uStack_70);
    if (lStack_68 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iStack_6c;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_exec_command @ 0x2b640

undefined4 sys_exec_command(undefined8 param_1,char param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined4 local_28;
  undefined4 local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00124fc0(&local_28);
  DAT_00184728 = signal(0xd,(__sighandler_t)0x1);
  DAT_00184730 = FUN_00124f80();
  if (DAT_00184730 == 0) {
    FUN_0012a400(local_28,0);
    FUN_0010d2c0(local_24);
    FUN_0012a8f0(param_1,(int)param_2,param_3);
    FUN_0012a3b0(DAT_00184a60);
  }
  else {
    FUN_0010d2c0(local_28);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_24;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_wait_command @ 0x2b6f0

void sys_wait_command(void)

{
  __pid_t _Var1;
  int *piVar2;
  undefined8 uVar3;
  uint uVar4;
  int __pid;
  long lVar5;
  long in_FS_OFFSET;
  uint local_24;
  long local_20;
  
  __pid = DAT_00184730;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (-1 < DAT_00184730) {
    signal(0xd,DAT_00184728);
    do {
      _Var1 = waitpid(__pid,(int *)&local_24,0);
      if (_Var1 != -1) {
        uVar4 = local_24 & 0x7f;
        if (uVar4 == 0) {
          if ((DAT_00184a59 == '\0') && (uVar4 = local_24 >> 8, (char)(local_24 >> 8) != '\0')) {
            if (DAT_00184d50 != (code *)0x0) {
              (*DAT_00184d50)();
              uVar4 = local_24 >> 8;
            }
            lVar5 = (long)DAT_00184730;
            uVar3 = dcgettext(0,"%lu: Child returned status %d",5);
            error(0,0,uVar3,lVar5,uVar4 & 0xff);
            DAT_00184d58 = 2;
          }
        }
        else if ((char)((char)uVar4 + '\x01') < '\x02') {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          lVar5 = (long)DAT_00184730;
          uVar3 = dcgettext(0,"%lu: Child terminated on unknown reason",5);
          error(0,0,uVar3,lVar5);
          DAT_00184d58 = 2;
        }
        else {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
            uVar4 = local_24 & 0x7f;
          }
          lVar5 = (long)DAT_00184730;
          uVar3 = dcgettext(0,"%lu: Child terminated on signal %d",5);
          error(0,0,uVar3,lVar5,uVar4);
        }
        DAT_00184730 = -1;
        goto LAB_0012b7c5;
      }
      piVar2 = __errno_location();
      __pid = DAT_00184730;
    } while (*piVar2 == 4);
    DAT_00184730 = -1;
    FUN_00132df0(DAT_00184a60);
  }
LAB_0012b7c5:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_exec_info_script @ 0x2b8b0

uint sys_exec_info_script(undefined8 *param_1,int param_2)

{
  int iVar1;
  __pid_t _Var2;
  FILE *__stream;
  __ssize_t _Var3;
  int *piVar4;
  char *pcVar5;
  uint uVar6;
  int iVar7;
  long in_FS_OFFSET;
  uint local_64;
  char *local_60;
  size_t local_58;
  int local_50;
  int local_4c;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00124fc0(&local_50);
  DAT_00184720 = signal(0xd,(__sighandler_t)0x1);
  iVar1 = FUN_00124f80();
  if (iVar1 == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*param_1,1);
    pcVar5 = (char *)FUN_00143090((long)param_2,local_48);
    setenv("TAR_VOLUME",pcVar5,1);
    pcVar5 = (char *)FUN_00143090((long)DAT_00184ba0,local_48);
    setenv("TAR_BLOCKING_FACTOR",pcVar5,1);
    pcVar5 = (char *)FUN_0012c7e0(DAT_00184ba8);
    setenv("TAR_SUBCOMMAND",pcVar5,1);
    iVar1 = DAT_00184400;
    if (DAT_00184400 == 0) {
      iVar1 = DAT_00184ba4;
    }
    pcVar5 = (char *)FUN_0012c230(iVar1);
    setenv("TAR_FORMAT",pcVar5,1);
    pcVar5 = (char *)FUN_00143090((long)local_4c,local_48);
    setenv("TAR_FD",pcVar5,1);
    FUN_0010d2c0(local_50);
    FUN_0012a3b0(DAT_00184b48);
  }
  else {
    local_60 = (char *)0x0;
    local_58 = 0;
    FUN_0010d2c0(local_4c);
    __stream = fdopen(local_50,"r");
    _Var3 = getline(&local_60,&local_58,__stream);
    iVar7 = (int)_Var3;
    fclose(__stream);
    if ((0 < iVar7) && (local_60[(long)iVar7 + -1] == '\n')) {
      local_60[(long)iVar7 + -1] = '\0';
      iVar7 = iVar7 + -1;
    }
    do {
      _Var2 = waitpid(iVar1,(int *)&local_64,0);
      if (_Var2 != -1) {
        signal(0xd,DAT_00184720);
        if ((local_64 & 0x7f) == 0) {
          if (((char)(local_64 >> 8) == '\0') && (0 < iVar7)) {
            *param_1 = local_60;
            uVar6 = 0;
          }
          else {
            free(local_60);
            uVar6 = local_64 >> 8 & 0xff;
          }
        }
        else {
          free(local_60);
          uVar6 = 0xffffffff;
        }
        goto LAB_0012b9cb;
      }
      piVar4 = __errno_location();
    } while (*piVar4 == 4);
    signal(0xd,DAT_00184720);
    FUN_00132df0(DAT_00184b48);
    uVar6 = 0xffffffff;
LAB_0012b9cb:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return uVar6;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_exec_checkpoint_script @ 0x2bb40

void sys_exec_checkpoint_script(undefined8 param_1,char *param_2,int param_3)

{
  int iVar1;
  __pid_t _Var2;
  int *piVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  int local_4c;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00124f80();
  if (iVar1 == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",param_2,1);
    pcVar4 = (char *)FUN_00143090((long)param_3,local_48);
    setenv("TAR_CHECKPOINT",pcVar4,1);
    pcVar4 = (char *)FUN_00143090((long)DAT_00184ba0,local_48);
    setenv("TAR_BLOCKING_FACTOR",pcVar4,1);
    pcVar4 = (char *)FUN_0012c7e0(DAT_00184ba8);
    setenv("TAR_SUBCOMMAND",pcVar4,1);
    iVar1 = DAT_00184400;
    if (DAT_00184400 == 0) {
      iVar1 = DAT_00184ba4;
    }
    pcVar4 = (char *)FUN_0012c230(iVar1);
    setenv("TAR_FORMAT",pcVar4,1);
    FUN_0012a3b0(param_1);
  }
  else {
    do {
      _Var2 = waitpid(iVar1,&local_4c,0);
      if (_Var2 != -1) goto LAB_0012bb9b;
      piVar3 = __errno_location();
    } while (*piVar3 == 4);
    FUN_00132df0(param_1);
LAB_0012bb9b:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: optloc_lookup @ 0x2bcb0

undefined8 optloc_lookup(int param_1)

{
  return *(undefined8 *)(&DAT_00184bc0 + (long)param_1 * 8);
}




// Function: option_set_in_cl @ 0x2bcc0

bool option_set_in_cl(void)

{
  int *piVar1;
  
  piVar1 = (int *)FUN_0012bcb0();
  if (piVar1 != (int *)0x0) {
    return *piVar1 == 1;
  }
  return false;
}




// Function: find_argp_option_key @ 0x2bce0

long * find_argp_option_key(long *param_1,int param_2)

{
  for (; (((*param_1 != 0 || ((int)param_1[1] != 0)) || (param_1[2] != 0)) ||
         (((int)param_1[3] != 0 || (param_1[4] != 0)))); param_1 = param_1 + 6) {
    if (param_2 == (int)param_1[1]) {
      return param_1;
    }
  }
  return (long *)0x0;
}




// Function: optloc_eq @ 0x2bd20

bool optloc_eq(int *param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  
  bVar1 = false;
  if ((*param_1 == *param_2) && (bVar1 = true, *param_1 != 1)) {
    iVar2 = strcmp(*(char **)(param_1 + 2),*(char **)(param_2 + 2));
    return iVar2 == 0;
  }
  return bVar1;
}




// Function: optloc_save @ 0x2bd60

undefined8 optloc_save(uint param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  size_t sVar3;
  undefined4 *puVar4;
  char *pcVar5;
  long lVar6;
  
  lVar6 = 0x20;
  if (*(char **)(param_2 + 2) != (char *)0x0) {
    sVar3 = strlen(*(char **)(param_2 + 2));
    lVar6 = sVar3 + 0x21;
  }
  puVar4 = (undefined4 *)FUN_0014c5b0(lVar6);
  pcVar5 = (char *)0x0;
  if (*(char **)(param_2 + 2) != (char *)0x0) {
    pcVar5 = strcpy((char *)(puVar4 + 8),*(char **)(param_2 + 2));
  }
  uVar1 = *param_2;
  *(char **)(puVar4 + 2) = pcVar5;
  *puVar4 = uVar1;
  *(undefined8 *)(puVar4 + 4) = *(undefined8 *)(param_2 + 4);
  uVar2 = *(undefined8 *)(&DAT_00184bc0 + (ulong)param_1 * 8);
  *(undefined4 **)(&DAT_00184bc0 + (ulong)param_1 * 8) = puVar4;
  *(undefined8 *)(puVar4 + 6) = uVar2;
  return uVar2;
}




// Function: tar_set_quoting_style @ 0x2bde0

void tar_set_quoting_style(char *param_1)

{
  undefined8 uVar1;
  int iVar2;
  undefined8 uVar3;
  ulong uVar4;
  char *__s2;
  
  if (PTR_s_literal_00180680 != (undefined *)0x0) {
    uVar4 = 0;
    __s2 = PTR_s_literal_00180680;
    do {
      iVar2 = strcmp(param_1,__s2);
      if (iVar2 == 0) {
        FUN_0014a590(0,uVar4 & 0xffffffff);
        return;
      }
      uVar4 = uVar4 + 1;
      __s2 = (&PTR_s_literal_00180680)[uVar4];
    } while (__s2 != (char *)0x0);
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = DAT_00184dd0;
  uVar3 = dcgettext(0,"Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list."
                    ,5);
  error(0,0,uVar3,param_1,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: sigstat @ 0x2be90

void sigstat(int param_1)

{
  FUN_0010cd50();
  FUN_0010d150();
  signal(param_1,sigstat);
  return;
}




// Function: archive_format_string @ 0x2c230

char * archive_format_string(int param_1)

{
  undefined **ppuVar1;
  int iVar2;
  char *pcVar3;
  
  iVar2 = 1;
  pcVar3 = "v7";
  ppuVar1 = &PTR_DAT_0017f9a0;
  while( true ) {
    if (param_1 == iVar2) {
      return pcVar3;
    }
    pcVar3 = ppuVar1[2];
    if (pcVar3 == (char *)0x0) break;
    iVar2 = *(int *)(ppuVar1 + 3);
    ppuVar1 = ppuVar1 + 2;
  }
  return "unknown?";
}




// Function: format_default_settings @ 0x2c280

void format_default_settings(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0012c230(6,param_2,param_3,0x14);
  FUN_0014cc50("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",uVar1,
               &DAT_0016863b);
  return;
}




// Function: tar_help_filter @ 0x2c2d0

undefined8 tar_help_filter(int param_1,undefined8 param_2)

{
  char *pcVar1;
  size_t sVar2;
  undefined2 *puVar3;
  undefined1 *puVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 auStack_78 [8];
  long local_70;
  undefined1 *local_68;
  undefined2 *local_60;
  undefined1 *local_58;
  ulong local_48;
  byte local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0x94) {
    uVar5 = dcgettext(0,"filter the archive through %s",5);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      pcVar1 = "lzip";
      goto LAB_0012c519;
    }
    goto LAB_0012c735;
  }
  if (param_1 < 0x95) {
    if (param_1 == 0x6a) {
      uVar5 = dcgettext(0,"filter the archive through %s",5);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "bzip2";
        goto LAB_0012c519;
      }
      goto LAB_0012c735;
    }
    if (param_1 < 0x6b) {
      if (param_1 == 0x4a) {
        uVar5 = dcgettext(0,"filter the archive through %s",5);
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "xz";
          goto LAB_0012c519;
        }
        goto LAB_0012c735;
      }
      if (param_1 == 0x5a) {
        uVar5 = dcgettext(0,"filter the archive through %s",5);
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "compress";
          goto LAB_0012c519;
        }
        goto LAB_0012c735;
      }
    }
    else if (param_1 == 0x7a) {
      uVar5 = dcgettext(0,"filter the archive through %s",5);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "gzip";
        goto LAB_0012c519;
      }
      goto LAB_0012c735;
    }
  }
  else {
    if (param_1 == 0xcf) {
      uVar5 = dcgettext(0,"filter the archive through %s",5);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "zstd";
        goto LAB_0012c519;
      }
      goto LAB_0012c735;
    }
    if (param_1 < 0xd0) {
      if (param_1 == 0x95) {
        uVar5 = dcgettext(0,"filter the archive through %s",5);
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "lzma";
LAB_0012c519:
          uVar5 = FUN_0014cc50(uVar5,pcVar1);
          return uVar5;
        }
        goto LAB_0012c735;
      }
      if (param_1 == 0x96) {
        uVar5 = dcgettext(0,"filter the archive through %s",5);
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "lzop";
          goto LAB_0012c519;
        }
        goto LAB_0012c735;
      }
    }
    else if (param_1 == 0x2000004) {
      _obstack_begin(auStack_78,0,0,FUN_0014c5b0,PTR_free_00180fb8);
      pcVar1 = (char *)dcgettext(0,"Valid arguments for the --quoting-style option are:",5);
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_58 - (long)local_60) < sVar2) {
        _obstack_newchunk(auStack_78,sVar2);
      }
      memcpy(local_60,pcVar1,sVar2);
      local_60 = (undefined2 *)((long)local_60 + sVar2);
      if ((ulong)((long)local_58 - (long)local_60) < 2) {
        _obstack_newchunk(auStack_78,2);
      }
      *local_60 = 0xa0a;
      local_60 = local_60 + 1;
      FUN_0012c050(auStack_78);
      pcVar1 = (char *)dcgettext(0,"\n*This* tar defaults to:\n",5);
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_58 - (long)local_60) < sVar2) {
        _obstack_newchunk(auStack_78,sVar2);
      }
      memcpy(local_60,pcVar1,sVar2);
      local_60 = (undefined2 *)((long)local_60 + sVar2);
      pcVar1 = (char *)FUN_0012c280();
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_58 - (long)local_60) < sVar2) {
        _obstack_newchunk(auStack_78,sVar2);
      }
      memcpy(local_60,pcVar1,sVar2);
      local_60 = (undefined2 *)((long)local_60 + sVar2);
      puVar3 = local_60;
      if (local_60 == (undefined2 *)local_58) {
        _obstack_newchunk(auStack_78,1);
        puVar3 = local_60;
      }
      local_60 = (undefined2 *)((long)puVar3 + 1);
      *(undefined1 *)puVar3 = 10;
      if ((undefined2 *)local_58 == local_60) {
        _obstack_newchunk(auStack_78,1);
      }
      *(undefined1 *)local_60 = 0;
      if ((undefined1 *)((long)local_60 + 1) == local_68) {
        local_28 = local_28 | 2;
      }
      puVar4 = (undefined1 *)((ulong)((undefined1 *)((long)local_60 + 1) + local_48) & ~local_48);
      local_68 = local_58;
      if ((ulong)((long)puVar4 - local_70) <= (ulong)((long)local_58 - local_70)) {
        local_68 = puVar4;
      }
      local_60 = (undefined2 *)local_68;
      param_2 = FUN_0014c7a0();
      _obstack_free(auStack_78,0);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2;
  }
LAB_0012c735:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: subcommand_string @ 0x2c7e0

char * subcommand_string(undefined4 param_1)

{
  switch(param_1) {
  case 0:
    return "unknown?";
  case 1:
    return "-r";
  case 2:
    return "-A";
  case 3:
    return "-c";
  case 4:
    return "-D";
  case 5:
    return "-d";
  case 6:
    return "-x";
  case 7:
    return "-t";
  case 8:
    return "-u";
  case 9:
    return "--test-label";
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  }
}




// Function: decode_signal @ 0x2c8a0

undefined4 decode_signal(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined **ppuVar3;
  char *__s2;
  
  ppuVar3 = &PTR_DAT_0017f8e0;
  iVar1 = strncmp(param_1,"SIG",3);
  __s2 = param_1 + 3;
  if (iVar1 != 0) {
    __s2 = param_1;
  }
  iVar1 = strcmp("USR1",__s2);
  while( true ) {
    if (iVar1 == 0) {
      return *(undefined4 *)(ppuVar3 + 1);
    }
    ppuVar3 = ppuVar3 + 2;
    if (ppuVar3 == &PTR_DAT_0017f930) break;
    iVar1 = strcmp(*ppuVar3,__s2);
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = dcgettext(0,"Unknown signal name: %s",5);
  error(0,0,uVar2,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_001163d0();
}




// Function: set_stat_signal @ 0x2c950

void set_stat_signal(void)

{
  undefined4 uVar1;
  
  uVar1 = FUN_0012c8a0();
  FUN_0012be80(uVar1);
  return;
}




// Function: usage @ 0x2c970

void usage(int param_1)

{
  argp_help(&PTR_DAT_00181b20,stderr,4,DAT_00184dd0);
  FUN_0013e440();
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: request_stdin @ 0x2c9b0

void request_stdin(undefined8 param_1)

{
  long lVar1;
  undefined8 uVar2;
  
  if (DAT_00184c10 == 0) {
    DAT_00184c10 = param_1;
    return;
  }
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  lVar1 = DAT_00184c10;
  uVar2 = dcgettext(0,"Options \'%s\' and \'%s\' both want standard input",5);
  error(0,0,uVar2,lVar1,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: confirm @ 0x2ca20

ulong confirm(undefined8 param_1,ulong param_2)

{
  uint uVar1;
  int iVar2;
  undefined8 uVar3;
  __ssize_t _Var4;
  long in_FS_OFFSET;
  char *local_30;
  size_t local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00184bb0 == (FILE *)0x0) {
    if ((DAT_00184a34 != 0) && (DAT_00184c10 == 0)) {
      FUN_0012c9b0(&DAT_00165bc7);
      DAT_00184bb0 = stdin;
      goto LAB_0012ca51;
    }
    DAT_00184bb0 = fopen("/dev/tty","r");
    if (DAT_00184bb0 != (FILE *)0x0) goto LAB_0012ca51;
    FUN_001327e0("/dev/tty");
LAB_0012cb30:
    local_30 = (char *)0x0;
    local_28 = 0;
    _Var4 = getline(&local_30,&local_28,DAT_00184bb0);
    if (_Var4 < 0) {
      DAT_00184bac = 1;
      free(local_30);
    }
    else {
      iVar2 = rpmatch(local_30);
      uVar1 = DAT_00184bac;
      param_2 = (ulong)(0 < iVar2);
      free(local_30);
      if (uVar1 == 0) goto LAB_0012cab5;
    }
  }
  else {
LAB_0012ca51:
    uVar3 = FUN_0014ac50(param_2);
    __fprintf_chk(DAT_00183f28,1,"%s %s?",param_1,uVar3);
    fflush_unlocked(DAT_00183f28);
    param_2 = (ulong)DAT_00184bac;
    if (DAT_00184bac == 0) goto LAB_0012cb30;
    param_2 = 0;
  }
  fputc_unlocked(10,DAT_00183f28);
  fflush_unlocked(DAT_00183f28);
LAB_0012cab5:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_2 & 0xffffffff;
}




// Function: get_date_or_file @ 0x2cbe0

undefined4 get_date_or_file(long param_1,undefined8 param_2,char *param_3,undefined1 (*param_4) [16])

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  undefined4 uVar7;
  long in_FS_OFFSET;
  undefined1 auVar8 [16];
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((byte)(*param_3 - 0x2eU) < 2) {
    iVar2 = stat(param_3,&sStack_c8);
    if (iVar2 != 0) {
      FUN_00132d30(param_3);
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar6 = dcgettext(0,"Date sample file not found",5);
      error(0,0,uVar6);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    }
    auVar8 = FUN_0014bd20(&sStack_c8);
    *param_4 = auVar8;
    uVar7 = 0;
  }
  else {
    cVar1 = FUN_001486d0(param_4,param_3,0);
    if (cVar1 == '\0') {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar6 = FUN_0014ac50(param_3);
      uVar3 = FUN_00122080(*(undefined8 *)*param_4,*(undefined8 *)(*param_4 + 8),0);
      uVar4 = dcgettext(0,"Substituting %s for unknown date format %s",5);
      uVar7 = 1;
      error(0,0,uVar4,uVar3,uVar6);
      *(undefined8 *)(*param_4 + 8) = 0;
    }
    else {
      puVar5 = (undefined8 *)FUN_0014c5b0(0x28);
      uVar6 = *(undefined8 *)*param_4;
      uVar3 = *(undefined8 *)(*param_4 + 8);
      puVar5[3] = param_2;
      uVar7 = 0;
      puVar5[1] = uVar6;
      puVar5[2] = uVar3;
      uVar6 = FUN_0014c7a0(param_3);
      puVar5[4] = uVar6;
      uVar6 = *(undefined8 *)(param_1 + 8);
      *(undefined8 **)(param_1 + 8) = puVar5;
      *puVar5 = uVar6;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: expand_pax_option @ 0x2cd70

undefined8 expand_pax_option(undefined8 param_1,char *param_2)

{
  ulong __n;
  byte bVar1;
  char cVar2;
  int iVar3;
  size_t __n_00;
  void *pvVar4;
  char *pcVar5;
  undefined8 uVar6;
  ushort **ppuVar7;
  size_t __n_01;
  ulong uVar8;
  void *__src;
  long in_FS_OFFSET;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined1 local_c8 [8];
  long local_c0;
  char *local_b8;
  char *local_b0;
  char *local_a8;
  ulong local_98;
  byte local_78;
  undefined1 local_68 [40];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  _obstack_begin(local_c8,0,0,FUN_0014c5b0,PTR_free_00180fb8);
  for (; pcVar5 = local_b0, *param_2 != '\0'; param_2 = param_2 + __n_00 + 1) {
    uVar8 = (long)local_a8 - (long)local_b0;
    __n_00 = strcspn(param_2,",");
    pvVar4 = memchr(param_2,0x3d,__n_00);
    if (pvVar4 == (void *)0x0) {
      if (uVar8 < __n_00) {
        _obstack_newchunk(local_c8,__n_00);
        pcVar5 = local_b0;
      }
      memcpy(pcVar5,param_2,__n_00);
      local_b0 = local_b0 + __n_00;
LAB_0012ce3f:
      cVar2 = param_2[__n_00];
    }
    else {
      __n = (long)pvVar4 + (1 - (long)param_2);
      if (uVar8 < __n) {
        _obstack_newchunk(local_c8,__n);
        pcVar5 = local_b0;
      }
      memcpy(pcVar5,param_2,__n);
      uVar8 = __n_00 - __n;
      pcVar5 = local_b0 + __n;
      __src = (void *)((long)pvVar4 + 1);
      bVar1 = *(byte *)((long)pvVar4 + 1);
      local_b0 = pcVar5;
      if (bVar1 != 0) {
        ppuVar7 = __ctype_b_loc();
        do {
          if ((*(byte *)((long)*ppuVar7 + (ulong)bVar1 * 2 + 1) & 0x20) == 0) {
            if ((bVar1 == 0x7b) && (*(char *)((long)__src + (uVar8 - 1)) == '}')) {
              pvVar4 = (void *)FUN_0014c5b0(uVar8);
              memcpy(pvVar4,(void *)((long)__src + 1),uVar8 - 2);
              *(undefined1 *)((long)pvVar4 + (uVar8 - 2)) = 0;
              iVar3 = FUN_0012cbe0(param_1,"--pax-option",pvVar4,&local_d8);
              if (iVar3 == 0) {
                pcVar5 = (char *)FUN_00124270(local_d8,local_d0,local_68);
                __n_01 = strlen(pcVar5);
                if ((ulong)((long)local_a8 - (long)local_b0) < __n_01) {
                  _obstack_newchunk(local_c8,__n_01);
                }
                memcpy(local_b0,pcVar5,__n_01);
                local_b0 = local_b0 + __n_01;
              }
              else {
                if ((ulong)((long)local_a8 - (long)local_b0) < uVar8) {
                  _obstack_newchunk(local_c8,uVar8);
                }
                memcpy(local_b0,__src,uVar8);
                local_b0 = local_b0 + uVar8;
              }
              free(pvVar4);
              goto LAB_0012ce3f;
            }
            break;
          }
          bVar1 = *(byte *)((long)__src + 1);
          __src = (void *)((long)__src + 1);
          uVar8 = uVar8 - 1;
        } while (bVar1 != 0);
      }
      if ((ulong)((long)local_a8 - (long)pcVar5) < uVar8) {
        _obstack_newchunk(local_c8,uVar8);
        pcVar5 = local_b0;
      }
      memcpy(pcVar5,__src,uVar8);
      cVar2 = param_2[__n_00];
      local_b0 = local_b0 + uVar8;
    }
    if (cVar2 == '\0') break;
    if (local_a8 == local_b0) {
      _obstack_newchunk(local_c8,1);
      cVar2 = param_2[__n_00];
    }
    *local_b0 = cVar2;
    local_b0 = local_b0 + 1;
  }
  if (local_b0 == local_a8) {
    _obstack_newchunk(local_c8,1);
  }
  *local_b0 = '\0';
  if (local_b0 + 1 == local_b8) {
    local_78 = local_78 | 2;
  }
  pcVar5 = (char *)((ulong)(local_b0 + 1 + local_98) & ~local_98);
  local_b8 = local_a8;
  if ((ulong)((long)pcVar5 - local_c0) <= (ulong)((long)local_a8 - local_c0)) {
    local_b8 = pcVar5;
  }
  local_b0 = local_b8;
  uVar6 = FUN_0014c7a0();
  _obstack_free(local_c8,0);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_subcommand_option @ 0x2d160

void set_subcommand_option(int param_1)

{
  undefined8 uVar1;
  
  if ((DAT_00184ba8 != 0) && (DAT_00184ba8 != param_1)) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar1 = dcgettext(0,
                      "You may not specify more than one \'-Acdtrux\', \'--delete\' or  \'--test-label\' option"
                      ,5);
    error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_0012c970(2);
  }
  DAT_00184ba8 = param_1;
  return;
}




// Function: option_conflict_error @ 0x2d1b0

void option_conflict_error(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar1 = dcgettext(0,"\'%s\' cannot be used with \'%s\'",5);
  error(0,0,uVar1,param_1,param_2);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: set_old_files_option @ 0x2d200

void set_old_files_option(uint param_1,undefined8 param_2)

{
  int iVar1;
  long lVar2;
  int *piVar3;
  undefined8 uVar4;
  char *__s2;
  
  lVar2 = FUN_0012bd60(9);
  if (lVar2 != 0) {
    iVar1 = FUN_0012bd20(param_2,lVar2);
    if ((iVar1 != 0) && (DAT_00184b34 != param_1)) {
      __s2 = (&PTR_s___overwrite_dir_0017f8a0)[(int)param_1];
      FUN_0012d1b0(__s2,(&PTR_s___overwrite_dir_0017f8a0)[DAT_00184b34]);
      piVar3 = (int *)FUN_0012bd60(0);
      if (DAT_00184b70 != (char *)0x0) {
        iVar1 = strcmp(DAT_00184b70,__s2);
        if ((iVar1 != 0) && (*piVar3 == 1)) {
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar4 = dcgettext(0,"Conflicting compression options",5);
          error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
          FUN_0012c970(2);
        }
      }
      DAT_00184b70 = __s2;
      return;
    }
  }
  DAT_00184b34 = param_1;
  return;
}




// Function: set_use_compress_program_option @ 0x2d260

void set_use_compress_program_option(char *param_1)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  piVar2 = (int *)FUN_0012bd60(0);
  if (DAT_00184b70 != (char *)0x0) {
    iVar1 = strcmp(DAT_00184b70,param_1);
    if ((iVar1 != 0) && (*piVar2 == 1)) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar3 = dcgettext(0,"Conflicting compression options",5);
      error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    }
  }
  DAT_00184b70 = param_1;
  return;
}




// Function: set_archive_format @ 0x2d2e0

void set_archive_format(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined **ppuVar4;
  char *__s1;
  
  __s1 = "v7";
  ppuVar4 = &PTR_DAT_0017f9a0;
  do {
    iVar1 = strcmp(__s1,param_1);
    if (iVar1 == 0) {
      DAT_00184ba4 = *(undefined4 *)(ppuVar4 + 1);
      return;
    }
    __s1 = ppuVar4[2];
    ppuVar4 = ppuVar4 + 2;
  } while (__s1 != (char *)0x0);
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  uVar2 = FUN_0014aa70(param_1);
  uVar3 = dcgettext(0,"%s: Invalid archive format",5);
  error(0,0,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
}




// Function: set_xattr_option @ 0x2d370

void set_xattr_option(int param_1)

{
  if (param_1 != 1) {
    DAT_00184a98 = param_1;
    return;
  }
  FUN_0012d2e0("posix");
  DAT_00184a98 = param_1;
  return;
}




// Function: parse_opt @ 0x2d3a0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int parse_opt(int param_1,char *param_2,long *param_3)

{
  long *plVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  long lVar5;
  ulong uVar6;
  void *pvVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  long lVar11;
  char *pcVar12;
  undefined8 *puVar13;
  long in_FS_OFFSET;
  ulong local_40;
  char *local_38;
  long local_30;
  
  puVar13 = (undefined8 *)param_3[5];
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (0xcf < param_1) {
    if (param_1 != 0x1000003) {
      if (param_1 == 0x1000005) {
        piVar2 = (int *)*puVar13;
        if (*piVar2 == 2) {
          uVar10 = *(undefined8 *)(piVar2 + 4);
          uVar8 = *(undefined8 *)(piVar2 + 2);
          uVar9 = dcgettext(0,"%s:%lu: location of the error",5);
          error(0,0,uVar9,uVar8,uVar10);
        }
        else if (*piVar2 == 0) {
          uVar10 = *(undefined8 *)(piVar2 + 2);
          uVar8 = dcgettext(0,"error parsing %s",5);
          error(0,0,uVar8,uVar10);
        }
                    /* WARNING: Subroutine does not return */
        exit(0x40);
      }
      param_1 = 7;
      goto LAB_0012d469;
    }
    if ((*(long **)(*param_3 + 0x20) != (long *)0x0) && (**(long **)(*param_3 + 0x20) != 0)) {
      lVar11 = 0x20;
      lVar5 = 0;
      while( true ) {
        *(undefined8 **)(param_3[6] + lVar5) = puVar13;
        lVar5 = lVar5 + 8;
        plVar1 = (long *)(*(long *)(*param_3 + 0x20) + lVar11);
        lVar11 = lVar11 + 0x20;
        if (*plVar1 == 0) break;
        puVar13 = (undefined8 *)param_3[5];
      }
    }
    goto LAB_0012d44d;
  }
  if (param_1 < 0) {
    param_1 = 7;
    goto LAB_0012d469;
  }
  switch(param_1) {
  case 0:
    FUN_00126b50(param_2);
    break;
  default:
    param_1 = 7;
    break;
  case 0x30:
  case 0x31:
  case 0x32:
  case 0x33:
  case 0x34:
  case 0x35:
  case 0x36:
  case 0x37:
    uVar10 = dcgettext(0,"Options \'-[0-7][lmh]\' not supported by *this* tar",5);
    argp_error(param_3,uVar10);
                    /* WARNING: Subroutine does not return */
    exit(0x40);
  case 0x41:
    FUN_0012d160(2);
    param_1 = 0;
    break;
  case 0x42:
    DAT_00184ab9 = 1;
    param_1 = 0;
    break;
  case 0x46:
    param_1 = 0;
    DAT_00184b10 = 1;
    DAT_00184b48 = param_2;
    break;
  case 0x47:
    goto switchD_0012d3f6_caseD_47;
  case 0x48:
    FUN_0012d2e0(param_2);
    param_1 = 0;
    break;
  case 0x49:
    FUN_0012d260(param_2,*puVar13);
    param_1 = 0;
    break;
  case 0x4a:
    FUN_0012d260(&DAT_00163987,*puVar13);
    param_1 = 0;
    break;
  case 0x4b:
    FUN_0012bd60(5,*puVar13);
    FUN_00127190(param_2);
    param_1 = 0;
    break;
  case 0x4c:
    iVar3 = FUN_0014c870(param_2,&local_38,10,&local_40,"bBcGgkKMmPTtw");
    if (iVar3 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Invalid tape length";
LAB_0012e2b3:
      uVar10 = dcgettext(0,pcVar12,5);
      uVar8 = FUN_0014aa70(param_2);
      error(0,0,"%s: %s",uVar8,uVar10);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    }
    if ((param_2 < local_38) &&
       (pcVar12 = strchr("bBcGgkKMmPTtw",(int)local_38[-1]), pcVar12 == (char *)0x0)) {
      DAT_00184a70 = (double)local_40 * _DAT_00166d60;
    }
    else if ((long)local_40 < 0) {
      DAT_00184a70 = (double)local_40;
    }
    else {
      DAT_00184a70 = (double)(long)local_40;
    }
    DAT_00184b10 = 1;
    param_1 = 0;
    break;
  case 0x4d:
    DAT_00184b10 = 1;
    param_1 = 0;
    break;
  case 0x4e:
    DAT_00184b8c = 1;
    if (-1 < DAT_00184b08) {
LAB_0012e380:
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "More than one threshold date";
LAB_0012e39a:
      uVar10 = dcgettext(0,pcVar12,5);
      error(0,0,uVar10);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    }
    pcVar12 = "--after-date";
    goto LAB_0012d8ca;
  case 0x4f:
    DAT_00184a6a = 1;
    param_1 = 0;
    break;
  case 0x50:
    FUN_0012bd60(8,*puVar13);
    DAT_00184b92 = 1;
    param_1 = 0;
    break;
  case 0x52:
    DAT_00184b7c = 1;
    param_1 = 0;
    break;
  case 0x53:
    DAT_00184a88 = 1;
    param_1 = 0;
    break;
  case 0x55:
    FUN_0012d200(3,*puVar13);
    param_1 = 0;
    break;
  case 0x56:
    param_1 = 0;
    DAT_00184a40 = param_2;
    break;
  case 0x57:
    FUN_0012bd60(4,*puVar13);
    DAT_00184a50 = 1;
    param_1 = 0;
    break;
  case 0x5a:
    FUN_0012d260("compress",*puVar13);
    param_1 = 0;
    break;
  case 0x61:
    *(undefined1 *)((long)puVar13 + 0x12) = 1;
    param_1 = 0;
    break;
  case 0x62:
    iVar3 = FUN_0014c870(param_2,0,10,&local_38,&DAT_00167708);
    if ((((iVar3 != 0) || (DAT_00184ba0 = (int)local_38, (char *)(long)DAT_00184ba0 != local_38)) ||
        (DAT_00184ba0 < 1)) ||
       (DAT_00184b98 = (char *)((long)local_38 << 9),
       local_38 != (char *)((ulong)local_38 & 0x7fffffffffffff))) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Invalid blocking factor";
      goto LAB_0012e2b3;
    }
    param_1 = 0;
    break;
  case 99:
    FUN_0012d160(3);
    param_1 = 0;
    break;
  case 100:
    FUN_0012d160(5);
    param_1 = 0;
    break;
  case 0x66:
    if (DAT_00184830 == DAT_00184c18) {
      DAT_00184838 = FUN_0014c680(DAT_00184838,&DAT_00184c18,8);
    }
    lVar5 = DAT_00184830 * 8;
    DAT_00184830 = DAT_00184830 + 1;
    *(char **)(DAT_00184838 + lVar5) = param_2;
    param_1 = 0;
    break;
  case 0x67:
    FUN_0012bd60(2,*puVar13);
    DAT_00184b8c = 1;
    DAT_00184b28 = param_2;
    goto switchD_0012d3f6_caseD_47;
  case 0x68:
    DAT_00184b69 = 1;
    param_1 = 0;
    break;
  case 0x69:
    DAT_00184b51 = 1;
    param_1 = 0;
    break;
  case 0x6a:
    FUN_0012d260("bzip2",*puVar13);
    param_1 = 0;
    break;
  case 0x6b:
    FUN_0012d200(4,*puVar13);
    param_1 = 0;
    break;
  case 0x6c:
    DAT_00184c20 = 1;
    param_1 = 0;
    break;
  case 0x6d:
    DAT_00184a68 = 1;
    param_1 = 0;
    break;
  case 0x6e:
    DAT_001847fc = 1;
    param_1 = 0;
    break;
  case 0x6f:
    *(undefined1 *)(puVar13 + 2) = 1;
    param_1 = 0;
    break;
  case 0x70:
    DAT_00184aa4 = 1;
    param_1 = 0;
    break;
  case 0x72:
    FUN_0012d160(1);
    param_1 = 0;
    break;
  case 0x73:
    FUN_0012bd60(6,*puVar13);
    DAT_00184aac = 1;
    param_1 = 0;
    break;
  case 0x74:
    FUN_0012d160(7);
    DAT_00184a54 = DAT_00184a54 + 1;
    param_1 = 0;
    break;
  case 0x75:
    FUN_0012d160(8);
    param_1 = 0;
    break;
  case 0x76:
    DAT_00184a54 = DAT_00184a54 + 1;
    param_1 = 0;
    DAT_00183b80 = DAT_00183b80 | 0x583000;
    break;
  case 0x77:
    DAT_00184b40 = 1;
    param_1 = 0;
    break;
  case 0x78:
    FUN_0012d160(6);
    param_1 = 0;
    break;
  case 0x7a:
    FUN_0012d260(&DAT_0016645a,*puVar13);
    param_1 = 0;
    break;
  case 0x80:
    FUN_0012d2e0("posix");
    param_1 = 0;
    DAT_00184a9c = 1;
    break;
  case 0x81:
    uVar4 = 1;
    if (param_2 != (char *)0x0) {
      lVar5 = FUN_00139670("--atime-preserve",param_2,&PTR_s_replace_0017f980,&DAT_00166a90,4,
                           PTR_FUN_00183dd8);
      uVar4 = *(undefined4 *)(&DAT_00166a90 + lVar5 * 4);
    }
    param_1 = 0;
    DAT_00184b88 = uVar4;
    break;
  case 0x82:
    DAT_00184b84 = 1;
    if (param_2 != (char *)0x0) {
      puVar13[4] = param_2;
      param_1 = 0;
      break;
    }
    goto LAB_0012d44d;
  case 0x83:
    DAT_00184b20 = 1;
    param_1 = 0;
    break;
  case 0x84:
    if (param_2 == (char *)0x0) {
      DAT_00184b78 = 10;
      param_1 = 0;
    }
    else {
      if (*param_2 == '.') {
        param_2 = param_2 + 1;
        FUN_00110490(".");
      }
      uVar6 = strtoul(param_2,&local_38,0);
      DAT_00184b78 = (undefined4)uVar6;
      if (*local_38 != '\0') {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar12 = "--checkpoint value is not an integer";
        goto LAB_0012e366;
      }
      param_1 = 0;
    }
    break;
  case 0x85:
    FUN_00110490(param_2);
    param_1 = 0;
    break;
  case 0x86:
    DAT_00184af0 = 2;
    param_1 = 0;
    break;
  case 0x87:
    DAT_001847e0 = 1;
    param_1 = 0;
    break;
  case 0x88:
    DAT_00184b68 = 1;
    param_1 = 0;
    break;
  case 0x89:
    FUN_0012d160(4);
    param_1 = 0;
    break;
  case 0x8a:
    DAT_00184d70 = 1;
    param_1 = 0;
    break;
  case 0x8b:
    DAT_00184b90 = 1;
    param_1 = 0;
    break;
  case 0x8c:
    lVar5 = FUN_0012bf00(param_2,&DAT_00184b58);
    if (lVar5 != -1) {
      DAT_00184b54 = (undefined4)lVar5;
      param_1 = 0;
      break;
    }
    DAT_00184b54 = 0xffffffff;
    if (DAT_00184b58 != 0) {
      FUN_00126aa0(DAT_00184b58,&DAT_00184b54);
      param_1 = 0;
      break;
    }
    goto LAB_0012d44d;
  case 0x8d:
    FUN_00123a70(param_2);
    param_1 = 0;
    break;
  case 0x8e:
    DAT_00184a59 = 1;
    param_1 = 0;
    break;
  case 0x8f:
    DAT_00184b52 = 1;
    param_1 = 0;
    break;
  case 0x90:
    param_1 = 0;
    DAT_00184820 = param_2;
    break;
  case 0x91:
    DAT_00184b30 = 1;
    param_1 = 0;
    break;
  case 0x92:
    FUN_0012d200(6,*puVar13);
    param_1 = 0;
    break;
  case 0x93:
    uVar6 = strtoul(param_2,&local_38,10);
    DAT_00184b24 = (undefined4)uVar6;
    if (*local_38 == '\0') {
      param_1 = 0;
      break;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar12 = "Invalid incremental level value";
    goto LAB_0012e39a;
  case 0x94:
    FUN_0012d260(&DAT_001601cd,*puVar13);
    param_1 = 0;
    break;
  case 0x95:
    FUN_0012d260(&DAT_001601d4,*puVar13);
    param_1 = 0;
    break;
  case 0x96:
    FUN_0012d260(&DAT_001601de,*puVar13);
    param_1 = 0;
    break;
  case 0x97:
    DAT_00184b18 = FUN_001435a0();
    if (DAT_00184b18 == 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Invalid mode given on option";
LAB_0012e366:
      uVar10 = dcgettext(0,pcVar12,5);
      error(0,0,uVar10);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    DAT_00184b14 = umask(0);
    umask(DAT_00184b14);
    param_1 = 0;
    break;
  case 0x98:
    FUN_0012cbe0(puVar13,"--mtime",param_2,&DAT_00184ae0);
    if (DAT_00184af0 == 0) {
      DAT_00184af0 = 1;
      param_1 = 0;
      break;
    }
    goto LAB_0012d44d;
  case 0x99:
    if (-1 < DAT_00184b08) goto LAB_0012e380;
    pcVar12 = "--newer-mtime";
LAB_0012d8ca:
    FUN_0012cbe0(puVar13,pcVar12,param_2,&DAT_00184b00);
    FUN_0012bd60(3,*puVar13);
    param_1 = 0;
    break;
  case 0x9a:
    DAT_00184a9c = 0xffffffff;
    param_1 = 0;
    break;
  case 0x9b:
    *(undefined1 *)((long)puVar13 + 0x12) = 0;
    param_1 = 0;
    break;
  case 0x9c:
    DAT_00184b20 = 0;
    param_1 = 0;
    break;
  case 0x9d:
    DAT_001847e0 = 0;
    param_1 = 0;
    break;
  case 0x9e:
    DAT_00184a59 = 0;
    param_1 = 0;
    break;
  case 0x9f:
    FUN_0012d200(1,*puVar13);
    param_1 = 0;
    break;
  case 0xa0:
    for (; *param_2 != '\0'; param_2 = param_2 + 1) {
      FUN_0014a5b0(0,(int)*param_2,0);
    }
    param_1 = 0;
    break;
  case 0xa1:
    DAT_00184aa8 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa2:
    DAT_00184aa4 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa3:
    DAT_001847fc = 0;
    param_1 = 0;
    break;
  case 0xa4:
    DAT_00184aa0 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa5:
    FUN_0012d370(0xffffffff);
    param_1 = 0;
    break;
  case 0xa6:
    DAT_00184ad2 = 1;
    param_1 = 0;
    break;
  case 0xa7:
    FUN_0012bd60(1,*puVar13);
    if (param_2 == (char *)0x0) {
      DAT_00184b38 = (char *)0x1;
      param_1 = 0;
    }
    else {
      iVar3 = FUN_0014c870(param_2,0,10,&local_38,&DAT_00167708);
      if (iVar3 != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar10 = dcgettext(0,"Invalid number",5);
        uVar8 = FUN_0014aa70(param_2);
        error(0,0,"%s: %s",uVar8,uVar10);
                    /* WARNING: Subroutine does not return */
        FUN_001163d0();
      }
      DAT_00184b38 = local_38;
      param_1 = 0;
    }
    break;
  case 0xa8:
    FUN_0012d2e0(&DAT_0016633a);
    param_1 = 0;
    break;
  case 0xa9:
    DAT_00184ad1 = 1;
    param_1 = 0;
    break;
  case 0xaa:
    FUN_0012bd60(7,*puVar13);
    DAT_00184ad0 = 1;
    param_1 = 0;
    DAT_00184ac8 = param_2;
    break;
  case 0xab:
    FUN_0012d200(0,*puVar13);
    param_1 = 0;
    break;
  case 0xac:
    FUN_0012d200(2,*puVar13);
    param_1 = 0;
    break;
  case 0xad:
    lVar5 = FUN_0012bf00(param_2,&DAT_00184ac0);
    if (lVar5 != -1) {
      DAT_00184abc = (undefined4)lVar5;
      param_1 = 0;
      break;
    }
    DAT_00184abc = 0xffffffff;
    if (DAT_00184ac0 != 0) {
      FUN_001269f0(DAT_00184ac0,&DAT_00184abc);
      param_1 = 0;
      break;
    }
LAB_0012d44d:
    param_1 = 0;
    break;
  case 0xae:
    FUN_001239a0(param_2);
    param_1 = 0;
    break;
  case 0xaf:
    pvVar7 = (void *)FUN_0012cd70(puVar13);
    *(undefined1 *)((long)puVar13 + 0x11) = 1;
    FUN_0011c010(pvVar7);
    free(pvVar7);
    param_1 = 0;
    break;
  case 0xb0:
    FUN_0012d2e0("posix");
    param_1 = 0;
    break;
  case 0xb1:
    for (; *param_2 != '\0'; param_2 = param_2 + 1) {
      FUN_0014a5b0(0,(int)*param_2,1);
    }
    param_1 = 0;
    break;
  case 0xb2:
    FUN_0012bde0(param_2);
    param_1 = 0;
    break;
  case 0xb3:
    iVar3 = FUN_0014c870(param_2,0,10,&local_38,"bBcGgkKMmPTtw");
    if (iVar3 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Invalid record size";
      goto LAB_0012e2b3;
    }
    DAT_00184b98 = local_38;
    if (((ulong)local_38 & 0x1ff) != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar10 = dcgettext(0,"Record size must be a multiple of %d.",5);
      error(0,0,uVar10,0x200);
                    /* WARNING: Subroutine does not return */
      FUN_0012c970(2);
    }
    DAT_00184ba0 = (int)((ulong)local_38 >> 9);
    param_1 = 0;
    break;
  case 0xb4:
    DAT_00184aba = 1;
    param_1 = 0;
    break;
  case 0xb5:
    DAT_00184ab8 = 1;
    param_1 = 0;
    break;
  case 0xb6:
    DAT_00184a58 = 1;
    param_1 = 0;
    break;
  case 0xb7:
    param_1 = 0;
    PTR_s__usr_local_libexec_rmt_00183c60 = param_2;
    break;
  case 0xb8:
    param_1 = 0;
    DAT_00184ab0 = param_2;
    break;
  case 0xb9:
    DAT_00184aa8 = 1;
    param_1 = 0;
    break;
  case 0xba:
    FUN_0012d2e0("posix");
    param_1 = 0;
    DAT_00184aa0 = 1;
    break;
  case 0xbb:
    pvVar7 = (void *)FUN_0012c280();
    __printf_chk(1,&DAT_00167682,pvVar7);
    FUN_0013e440();
    free(pvVar7);
                    /* WARNING: Subroutine does not return */
    exit(0);
  case 0xbc:
    DAT_00184a89 = 1;
    param_1 = 0;
    break;
  case 0xbd:
    FUN_0011fc70();
    FUN_0013e440();
                    /* WARNING: Subroutine does not return */
    exit(0);
  case 0xbe:
    DAT_001847e1 = 1;
    param_1 = 0;
    break;
  case 0xbf:
    FUN_0012d200(5,*puVar13);
    param_1 = 0;
    break;
  case 0xc0:
    lVar5 = FUN_00139670("--sort",param_2,&DAT_0017f960,&DAT_00181ae8,4,PTR_FUN_00183dd8);
    DAT_001847e4 = *(undefined4 *)(&DAT_00181ae8 + lVar5 * 4);
    param_1 = 0;
    break;
  case 0xc1:
    lVar5 = FUN_00139670("--hole-detection",param_2,&PTR_DAT_0017f930,&DAT_00166a88,4,
                         PTR_FUN_00183dd8);
    DAT_00184a88 = 1;
    DAT_00184a7c = *(undefined4 *)(&DAT_00166a88 + lVar5 * 4);
    param_1 = 0;
    break;
  case 0xc2:
    DAT_00184a88 = 1;
    uVar6 = strtoul(param_2,&local_38,10);
    DAT_00184a84 = (undefined4)uVar6;
    if (*local_38 == '\0') {
LAB_0012d67c:
      param_1 = 0;
      break;
    }
    if (*local_38 == '.') {
      uVar6 = strtoul(local_38 + 1,&local_38,10);
      DAT_00184a80 = (undefined4)uVar6;
      if (*local_38 == '\0') goto LAB_0012d67c;
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    pcVar12 = "Invalid sparse version value";
    goto LAB_0012e39a;
  case 0xc3:
    iVar3 = FUN_0014c870(param_2,0,10,&local_38,&DAT_00167708);
    if (iVar3 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Invalid number of elements";
      goto LAB_0012e2b3;
    }
    DAT_00184a90 = local_38;
    param_1 = 0;
    break;
  case 0xc4:
    DAT_00184b84 = 1;
    param_1 = 0;
    puVar13[3] = param_2;
    break;
  case 0xc5:
    FUN_0012d160(9);
    param_1 = 0;
    break;
  case 0xc6:
    if (param_2 == (char *)0x0) {
      DAT_00184a69 = 1;
      param_1 = 0;
    }
    else {
      FUN_0012c950(param_2);
      param_1 = 0;
    }
    break;
  case 199:
    if (DAT_00184a60 != (char *)0x0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar12 = "Only one --to-command option allowed";
      goto LAB_0012e39a;
    }
    param_1 = 0;
    DAT_00184a60 = param_2;
    break;
  case 200:
    FUN_00130710(param_2);
    param_1 = 0;
    break;
  case 0xc9:
    DAT_00184b91 = 1;
    param_1 = 0;
    break;
  case 0xca:
    param_1 = 0;
    DAT_00184a48 = param_2;
    break;
  case 0xcb:
    FUN_00131470(param_2);
    param_1 = 0;
    break;
  case 0xcc:
    FUN_0012d370(1);
    param_1 = 0;
    break;
  case 0xcd:
  case 0xce:
    FUN_0012d370(1);
    FUN_00131af0(param_2,param_1 == 0xce);
    param_1 = 0;
    break;
  case 0xcf:
    FUN_0012d260(&DAT_001601e5,*puVar13);
    param_1 = 0;
  }
LAB_0012d469:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_0012d3f6_caseD_47:
  DAT_00184b50 = 1;
  param_1 = 0;
  goto LAB_0012d469;
}




// Function: parse_default_options @ 0x2e4b0

void parse_default_options(long *param_1)

{
  long lVar1;
  char cVar2;
  int iVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_12c [4];
  undefined4 local_128 [2];
  char *local_120;
  undefined8 local_118;
  undefined8 local_110;
  long local_108;
  undefined8 *local_100;
  undefined8 local_f8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar4 = getenv("TAR_OPTIONS");
  local_128[0] = 0;
  local_120 = "TAR_OPTIONS";
  local_118 = 0;
  local_110 = 0;
  if (pcVar4 != (char *)0x0) {
    local_f8 = 1;
    iVar3 = FUN_001382c0(pcVar4,&local_108,0x2000e46);
    if (iVar3 != 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar6 = FUN_001354b0(&local_108);
      uVar5 = dcgettext(0,"cannot split TAR_OPTIONS: %s",5);
      error(0,0,uVar5,uVar6);
                    /* WARNING: Subroutine does not return */
      FUN_001163d0();
    }
    if (local_108 != 0) {
      lVar1 = *param_1;
      *local_100 = DAT_00184dd0;
      *param_1 = (long)local_128;
      iVar3 = argp_parse(&PTR_DAT_00181b20,(int)local_108 + (int)local_f8,local_100,0x28,local_12c,
                         param_1);
      if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      *param_1 = lVar1;
      cVar2 = FUN_0012bca0();
      if (cVar2 != '\0') {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar4 = local_120;
        uVar6 = dcgettext(0,"non-option arguments in %s",5);
        error(0,0,uVar6,pcVar4);
                    /* WARNING: Subroutine does not return */
        FUN_0012c970(2);
      }
      local_108 = 0;
    }
    FUN_00135420(&local_108);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: decode_options @ 0x2e690

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void decode_options(int param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  undefined2 uVar2;
  uint uVar3;
  char cVar4;
  int iVar5;
  char *pcVar6;
  int *piVar7;
  undefined8 uVar8;
  size_t sVar9;
  int *piVar10;
  long lVar11;
  void *__ptr;
  undefined8 uVar12;
  ulong extraout_RDX;
  ulong extraout_RDX_00;
  ulong extraout_RDX_01;
  ulong extraout_RDX_02;
  undefined8 extraout_RDX_03;
  uint uVar13;
  long *plVar14;
  long *plVar15;
  undefined8 *puVar16;
  undefined8 *puVar18;
  ulong uVar19;
  undefined8 *puVar20;
  undefined8 *puVar21;
  long in_FS_OFFSET;
  byte bVar22;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined2 uStack_e0;
  undefined1 uStack_de;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  long lStack_c8;
  undefined8 uStack_c0;
  int local_b4;
  undefined8 *local_b0;
  int local_9c;
  undefined4 local_98 [2];
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined4 *local_78;
  undefined8 local_70;
  undefined2 local_68;
  char local_66;
  char *local_60;
  undefined8 local_58;
  undefined1 local_43;
  char local_42;
  undefined1 local_41;
  long local_40;
  undefined8 *puVar17;
  
  bVar22 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = local_98;
  local_68 = 0;
  local_98[0] = 1;
  local_90 = 0;
  local_88 = 0;
  local_80 = 0;
  local_70 = 0;
  local_66 = '\0';
  local_60 = (char *)0x0;
  local_58 = 0;
  uStack_c0 = 0x12e71e;
  FUN_0013da30("tar",&PTR_s_John_Gilmore_00181b00);
  uStack_c0 = 0x12e72a;
  local_60 = getenv("SIMPLE_BACKUP_SUFFIX");
  uStack_c0 = 0x12e73b;
  pcVar6 = getenv("POSIXLY_CORRECT");
  DAT_00184ba8 = 0;
  DAT_00184ba4 = 0;
  DAT_00184a38 = pcVar6 != (char *)0x0;
  DAT_00184ba0 = 0x14;
  DAT_00184b98 = 0x2800;
  uStack_c0 = 0x12e773;
  DAT_00184b60 = FUN_0013eba0();
  DAT_001847e8 = 1;
  DAT_00184a7c = 0;
  DAT_00184b00 = 0x8000000000000000;
  DAT_00184b08 = -1;
  DAT_00184ae0 = 0x8000000000000000;
  DAT_00184ae8 = -1;
  DAT_00184ad4 = 8;
  DAT_00184a84 = 1;
  DAT_00184a80 = 0;
  DAT_001847e4 = 0;
  DAT_00184abc = 0xffffffff;
  DAT_00184ac0 = 0;
  DAT_00184b54 = 0xffffffff;
  DAT_00184b58 = 0;
  DAT_00184b20 = 1;
  DAT_00184b24 = -1;
  DAT_001847fc = 0xffffffff;
  if ((1 < param_1) && (*(char *)param_2[1] != '-')) {
    local_43 = 0x2d;
    puVar20 = param_2 + 2;
    local_41 = 0;
    uStack_c0 = 0x12ed43;
    sVar9 = strlen((char *)param_2[1]);
    iVar5 = (int)sVar9 + param_1;
    local_b4 = iVar5 + -1;
    uStack_c0 = 0x12ed5b;
    local_b0 = (undefined8 *)FUN_0014c5b0((long)iVar5 * 8);
    pcVar6 = (char *)param_2[1];
    puVar21 = local_b0 + 1;
    puVar1 = param_2 + param_1;
    *local_b0 = *param_2;
    cVar4 = *pcVar6;
    if (cVar4 != '\0') {
      puVar16 = puVar21;
      do {
        while( true ) {
          puVar21 = puVar16 + 1;
          uStack_c0 = 0x12edc3;
          local_42 = cVar4;
          uVar8 = FUN_0014c7a0(&local_43);
          *puVar16 = uVar8;
          uStack_c0 = 0x12edce;
          lVar11 = FUN_0012beb0((int)*pcVar6);
          if ((lVar11 != 0) && (*(long *)(lVar11 + 0x10) != 0)) break;
          cVar4 = pcVar6[1];
          pcVar6 = pcVar6 + 1;
          puVar16 = puVar21;
          if (cVar4 == '\0') goto LAB_0012ede2;
        }
        if (puVar1 <= puVar20) {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f33e;
            (*DAT_00184d50)();
          }
          cVar4 = *pcVar6;
          uStack_c0 = 0x12f355;
          uVar8 = dcgettext(0,"Old option \'%c\' requires an argument.",5);
          uStack_c0 = 0x12f366;
          error(0,0,uVar8,(int)cVar4);
                    /* WARNING: Subroutine does not return */
          uStack_c0 = 0x12f370;
          FUN_0012c970(2);
        }
        uVar8 = *puVar20;
        pcVar6 = pcVar6 + 1;
        puVar21 = puVar16 + 2;
        puVar20 = puVar20 + 1;
        puVar16[1] = uVar8;
        cVar4 = *pcVar6;
        puVar16 = puVar21;
      } while (cVar4 != '\0');
    }
LAB_0012ede2:
    puVar16 = puVar20;
    puVar18 = puVar21;
    if (puVar20 < puVar1) {
      do {
        puVar17 = puVar16 + (ulong)bVar22 * -2 + 1;
        *puVar18 = *puVar16;
        puVar16 = puVar17;
        puVar18 = puVar18 + (ulong)bVar22 * -2 + 1;
      } while (puVar17 < puVar1);
      puVar21 = (undefined8 *)
                ((long)puVar21 + ((long)puVar1 + (-1 - (long)puVar20) & 0xfffffffffffffff8U) + 8);
    }
    *puVar21 = 0;
    param_2 = local_b0;
    param_1 = local_b4;
  }
  uStack_c0 = 0x12e846;
  FUN_0012e4b0(&local_78);
  piVar10 = &local_9c;
  uStack_c0 = 0x12e864;
  iVar5 = argp_parse(&PTR_DAT_00181b20,param_1,param_2,8,piVar10,&local_78);
  if (iVar5 != 0) {
                    /* WARNING: Subroutine does not return */
    uStack_c0 = 0x12f42d;
    exit(2);
  }
  uVar2 = local_68;
  if ((char)local_68 != '\0') {
    if (DAT_00184ba8 == 3) {
      uStack_c0 = 0x12f2fd;
      FUN_0012d2e0(&DAT_0016633a);
      uVar2 = local_68;
    }
    else {
      DAT_00184aa8 = 0xffffffff;
    }
  }
  for (; local_68 = uVar2, local_9c < param_1; local_9c = local_9c + 1) {
    uStack_c0 = 0x12e8a3;
    FUN_00126b50(param_2[local_9c]);
    uVar2 = local_68;
  }
  if (DAT_00184ba4 == 0) {
    local_68._1_1_ = (char)((ushort)uVar2 >> 8);
    if (local_68._1_1_ == '\0') {
      DAT_00184ba4 = 6;
    }
    else {
      DAT_00184ba4 = 4;
    }
  }
  if ((((DAT_00184a40 != (char *)0x0) && (DAT_00184ba8 == 3)) || (DAT_00184b50 != '\0')) ||
     ((DAT_00184b10 != '\0' || (DAT_00184a88 != '\0')))) {
    uStack_c0 = 0x12e8f6;
    FUN_0012e640();
  }
  if (DAT_00184b38 == 0) goto LAB_0012e904;
  uStack_c0 = 0x12eccf;
  cVar4 = FUN_0012bca0();
  if (cVar4 == '\0') {
LAB_0012f3a5:
    if (DAT_00184d50 != (code *)0x0) {
      uStack_c0 = 0x12f3b3;
      (*DAT_00184d50)();
    }
    pcVar6 = "--occurrence is meaningless without a file list";
    goto LAB_0012f200;
  }
  if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 0x10) == 0) {
    uStack_c0 = 0x12ecf8;
    iVar5 = FUN_0012bcc0(1);
    if (iVar5 == 0) {
      DAT_00184b38 = 0;
      goto LAB_0012e904;
    }
    uStack_c0 = 0x12f3cb;
    uVar8 = FUN_0012c7e0(extraout_RDX_01 & 0xffffffff);
    uStack_c0 = 0x12f3da;
    FUN_0012d1b0("--occurrence",uVar8);
LAB_0012f3da:
    uStack_c0 = 0x12f3ed;
    FUN_0012d1b0("--listed-incremental","--newer");
LAB_0012f3ed:
    uStack_c0 = 0x12f3f5;
    uVar8 = FUN_0012c7e0((ulong)piVar10 & 0xffffffff);
    uStack_c0 = 0x12f404;
    FUN_0012d1b0("--same-order",uVar8);
  }
  else {
LAB_0012e904:
    plVar14 = DAT_00184838;
    if (DAT_00184830 == 0) {
      DAT_00184830 = 1;
      uStack_c0 = 0x12f12c;
      pcVar6 = getenv("TAPE");
      *plVar14 = (long)pcVar6;
      if (*DAT_00184838 == 0) {
        *DAT_00184838 = (long)&DAT_0016863b;
      }
    }
    if ((DAT_00184830 < 2) || (DAT_00184b10 != '\0')) {
      if (DAT_00184b28 == 0) {
LAB_0012e97a:
        if (DAT_00184b24 != -1) {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f267;
            (*DAT_00184d50)();
          }
          uStack_c0 = 0x12f27a;
          uVar8 = dcgettext(0,"--level is meaningless without --listed-incremental",5);
          uStack_c0 = 0x12f288;
          error(0,0,uVar8);
        }
      }
      else if (-1 < DAT_00184b08) {
        uStack_c0 = 0x12e945;
        piVar7 = (int *)FUN_0012bcb0(2);
        uStack_c0 = 0x12e952;
        uVar8 = FUN_0012bcb0(3);
        uStack_c0 = 0x12e95d;
        iVar5 = FUN_0012bd20(piVar7,uVar8);
        if (iVar5 != 0) goto LAB_0012f3da;
        if (*piVar7 == 1) {
          DAT_00184b28 = 0;
          goto LAB_0012e97a;
        }
        DAT_00184b00 = 0;
        DAT_00184b08 = 0;
      }
      pcVar6 = DAT_00184a40;
      if ((DAT_00184a40 != (char *)0x0) && ((DAT_00184ba4 & 0xfffffffb) == 2)) {
        uVar19 = (ulong)(-(uint)(DAT_00184b10 == '\0') & 0x12) + 0x51;
        uStack_c0 = 0x12e9bb;
        sVar9 = strlen(DAT_00184a40);
        if (uVar19 < sVar9) {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f48c;
            (*DAT_00184d50)();
            pcVar6 = DAT_00184a40;
          }
          uStack_c0 = 0x12f49b;
          uVar8 = FUN_0014aa70(pcVar6);
          uStack_c0 = 0x12f4bc;
          uVar12 = dcngettext(0,"%s: Volume label is too long (limit is %lu byte)",
                              "%s: Volume label is too long (limit is %lu bytes)",uVar19,5);
          uStack_c0 = 0x12f4d0;
          error(0,0,uVar12,uVar8,uVar19);
                    /* WARNING: Subroutine does not return */
          uStack_c0 = 0x12f4da;
          FUN_0012c970(2);
        }
      }
      if (DAT_00184a50 == '\0') {
        if (DAT_00184b70 != 0) {
          if (DAT_00184b10 != '\0') {
            if (DAT_00184d50 != (code *)0x0) {
              uStack_c0 = 0x12f545;
              (*DAT_00184d50)();
            }
            pcVar6 = "Cannot use multi-volume compressed archives";
            goto LAB_0012f200;
          }
          if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 4) != 0) {
            if (DAT_00184d50 != (code *)0x0) {
              uStack_c0 = 0x12f526;
              (*DAT_00184d50)();
            }
            pcVar6 = "Cannot update compressed archives";
            goto LAB_0012f200;
          }
          if (DAT_00184ba8 == 2) {
            if (DAT_00184d50 != (code *)0x0) {
              uStack_c0 = 0x12f507;
              (*DAT_00184d50)();
            }
            pcVar6 = "Cannot concatenate compressed archives";
            goto LAB_0012f200;
          }
        }
      }
      else {
        if (DAT_00184b10 != '\0') {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f564;
            (*DAT_00184d50)();
          }
          pcVar6 = "Cannot verify multi-volume archives";
          goto LAB_0012f200;
        }
        if (DAT_00184b70 != 0) {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f37e;
            (*DAT_00184d50)();
          }
          pcVar6 = "Cannot verify compressed archives";
          goto LAB_0012f200;
        }
        if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 2) == 0) {
          uStack_c0 = 0x12ecb6;
          iVar5 = FUN_0012bcc0(4);
          if (iVar5 != 0) {
            uStack_c0 = 0x12f396;
            uVar8 = FUN_0012c7e0(extraout_RDX_00 & 0xffffffff);
            uStack_c0 = 0x12f3a5;
            FUN_0012d1b0("--verify",uVar8);
            goto LAB_0012f3a5;
          }
          DAT_00184a50 = '\0';
        }
      }
      if ((DAT_00184af0 == 2) && (DAT_00184ae8 < 0)) {
        if (DAT_00184d50 != (code *)0x0) {
          uStack_c0 = 0x12f0fd;
          (*DAT_00184d50)();
        }
        pcVar6 = "--clamp-mtime needs a date specified using --mtime";
        goto LAB_0012f200;
      }
      if ((local_68._1_1_ == '\0') || (DAT_00184ba4 == 4)) {
        if (DAT_00184a9c < 1) {
          if (DAT_00184aa0 < 1) {
LAB_0012ee71:
            if ((0 < DAT_00184a98) && (DAT_00184ba4 != 4)) {
              if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 1) == 0) {
                if (DAT_00184d50 != (code *)0x0) {
                  uStack_c0 = 0x12eeb0;
                  (*DAT_00184d50)();
                }
                pcVar6 = "--xattrs can be used only on POSIX archives";
                goto LAB_0012f200;
              }
              goto LAB_0012ea5e;
            }
          }
          else if (DAT_00184ba4 != 4) goto LAB_0012ee3f;
        }
        else {
          if (DAT_00184ba4 != 4) {
            if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 1) == 0) {
              if (DAT_00184d50 != (code *)0x0) {
                uStack_c0 = 0x12f2b2;
                (*DAT_00184d50)();
              }
              pcVar6 = "--acls can be used only on POSIX archives";
              goto LAB_0012f200;
            }
            goto LAB_0012ea51;
          }
          if (DAT_00184aa0 < 1) goto LAB_0012ee71;
        }
        if (DAT_00184a78 == '\0') goto LAB_0012ea67;
        piVar10 = (int *)(ulong)DAT_00184ba8;
        if (((&DAT_00166a60)[(long)piVar10 * 4] & 1) == 0) {
          uStack_c0 = 0x12ec59;
          iVar5 = FUN_0012bcc0(5);
          uVar19 = extraout_RDX;
          if (iVar5 != 0) goto LAB_0012f599;
          DAT_00184a78 = '\0';
          if (DAT_00184aac == '\0') goto LAB_0012ea9f;
          goto LAB_0012ea86;
        }
      }
      else {
        if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 1) == 0) {
          if (DAT_00184d50 != (code *)0x0) {
            uStack_c0 = 0x12f4e8;
            (*DAT_00184d50)();
          }
          pcVar6 = "--pax-option can be used only on POSIX archives";
          goto LAB_0012f200;
        }
        if (DAT_00184a9c < 1) {
          if (DAT_00184aa0 < 1) goto LAB_0012ee71;
LAB_0012ee3f:
          if (((&DAT_00166a60)[(ulong)DAT_00184ba8 * 4] & 1) == 0) {
            if (DAT_00184d50 != (code *)0x0) {
              uStack_c0 = 0x12ee57;
              (*DAT_00184d50)();
            }
            pcVar6 = "--selinux can be used only on POSIX archives";
            goto LAB_0012f200;
          }
        }
        else {
LAB_0012ea51:
          if (DAT_00184aa0 < 1) goto LAB_0012ee71;
        }
LAB_0012ea5e:
        if (DAT_00184a78 != '\0') goto LAB_0012ea9f;
LAB_0012ea67:
        if ((DAT_00184aac != '\0') &&
           (piVar10 = (int *)(ulong)DAT_00184ba8, ((&DAT_00166a60)[(long)piVar10 * 4] & 1) == 0)) {
LAB_0012ea86:
          uStack_c0 = 0x12ea90;
          iVar5 = FUN_0012bcc0(6);
          if (iVar5 != 0) goto LAB_0012f3ed;
          DAT_00184aac = '\0';
        }
      }
LAB_0012ea9f:
      if (DAT_00184ad0 != '\0') {
        if (DAT_00184b92 != '\0') {
          uStack_c0 = 0x12eabf;
          piVar10 = (int *)FUN_0012bcb0(7);
          uStack_c0 = 0x12eacc;
          uVar8 = FUN_0012bcb0(8);
          uStack_c0 = 0x12ead7;
          iVar5 = FUN_0012bd20(piVar10,uVar8);
          if (iVar5 != 0) {
            uStack_c0 = 0x12f599;
            FUN_0012d1b0("--one-top-level","--absolute-names");
            uVar19 = extraout_RDX_02;
LAB_0012f599:
            uStack_c0 = 0x12f5a0;
            uVar8 = FUN_0012c7e0(uVar19 & 0xffffffff);
            pcVar6 = "--starting-file";
            uStack_c0 = 0x12f5af;
            FUN_0012d1b0("--starting-file",uVar8);
            lStack_c8 = *(long *)(in_FS_OFFSET + 0x28);
            uStack_e0 = 0;
            uStack_e8 = 0;
            uStack_de = 0;
            uStack_d8 = 0;
            uStack_d0 = 0;
            uStack_f0 = extraout_RDX_03;
            argp_parse(&PTR_DAT_00181240,(ulong)pcVar6 & 0xffffffff,uVar8,0x2a,0,&uStack_f0);
            if (lStack_c8 == *(long *)(in_FS_OFFSET + 0x28)) {
              return;
            }
                    /* WARNING: Subroutine does not return */
            __stack_chk_fail();
          }
          if (*piVar10 != 1) {
            DAT_00184ad0 = '\0';
            goto LAB_0012eaf0;
          }
          DAT_00184b92 = '\0';
        }
        if (DAT_00184ac8 == 0) {
          uStack_c0 = 0x12f1be;
          __ptr = (void *)FUN_0013e510(*DAT_00184838);
          uStack_c0 = 0x12f1c9;
          DAT_00184ac8 = FUN_0012a280(__ptr);
          uStack_c0 = 0x12f1d8;
          free(__ptr);
          if (DAT_00184ac8 == 0) {
            if (DAT_00184d50 != (code *)0x0) {
              uStack_c0 = 0x12f1f4;
              (*DAT_00184d50)();
            }
            pcVar6 = 
            "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR"
            ;
            goto LAB_0012f200;
          }
        }
      }
LAB_0012eaf0:
      uVar3 = DAT_00184ba8;
      if (DAT_00184aba != '\0') {
        DAT_00184b34 = 3;
      }
      if (DAT_00184b69 == 0) {
        if (DAT_00184b88 != 2) {
          DAT_00184808 = 0xa0900;
          uVar13 = 0;
          goto LAB_0012f08e;
        }
        DAT_00184808 = 0xe0900;
        DAT_00184804 = 0xf0900;
        DAT_00184800 = 0x100;
      }
      else {
        DAT_00184808 = 0x80900;
        if (DAT_00184b88 == 2) {
          uVar13 = 0x40000;
LAB_0012f08e:
          DAT_00184808 = DAT_00184808 | uVar13;
          DAT_00184800 = (DAT_00184b69 ^ 1) << 8;
          DAT_00184804 = DAT_00184808 | 0x10000;
        }
        else {
          DAT_00184804 = 0x90900;
          DAT_00184800 = 0;
        }
      }
      if (DAT_00184ba8 == 9) {
        uStack_c0 = 0x12f0cd;
        cVar4 = FUN_0012bca0();
        if (cVar4 == '\0') {
          DAT_00184a54 = DAT_00184a54 + 1;
        }
      }
      else if (DAT_00184b91 != '\0') {
        DAT_00184a54 = 2;
      }
      if ((DAT_00184a70 != _DAT_00160748) && (DAT_00184a70 < (double)DAT_00184b98)) {
        if (DAT_00184d50 != (code *)0x0) {
          uStack_c0 = 0x12f43b;
          (*DAT_00184d50)();
        }
        pcVar6 = "Volume length cannot be less than record size";
        goto LAB_0012f200;
      }
      if ((DAT_00184aac == '\0') || (DAT_00184b28 == 0)) {
LAB_0012ebe4:
        switch(uVar3) {
        case 1:
        case 2:
        case 8:
          plVar14 = DAT_00184838 + DAT_00184830;
          for (plVar15 = DAT_00184838; DAT_00184828 = plVar15, plVar15 < plVar14;
              plVar15 = plVar15 + 1) {
            uStack_c0 = 0x12ef7b;
            iVar5 = strcmp((char *)*plVar15,"-");
            if (iVar5 == 0) {
              if (DAT_00184d50 != (code *)0x0) {
                uStack_c0 = 0x12f31f;
                (*DAT_00184d50)();
              }
              pcVar6 = "Options \'-Aru\' are incompatible with \'-f -\'";
              goto LAB_0012f200;
            }
          }
          break;
        case 3:
          uStack_c0 = 0x12f02a;
          cVar4 = FUN_0012bca0();
          if (cVar4 == '\0') goto LAB_0012f45f;
          if ((local_66 != '\0') && (DAT_00184830 != 0)) {
            pcVar6 = (char *)*DAT_00184838;
            uStack_c0 = 0x12f064;
            iVar5 = strcmp(pcVar6,"-");
            if (iVar5 != 0) {
              uStack_c0 = 0x12f07b;
              FUN_0012a260(pcVar6,DAT_00184b70);
            }
          }
          break;
        case 5:
        case 6:
        case 7:
        case 9:
          DAT_00184828 = DAT_00184838;
          plVar14 = DAT_00184838;
          uVar19 = DAT_00184830;
          if (DAT_00184838 < DAT_00184838 + DAT_00184830) {
            do {
              uStack_c0 = 0x12ef26;
              iVar5 = strcmp((char *)*DAT_00184828,"-");
              if (iVar5 == 0) {
                uStack_c0 = 0x12ef32;
                FUN_0012c9b0(&DAT_00165e6c);
                plVar14 = DAT_00184838;
                uVar19 = DAT_00184830;
              }
              DAT_00184828 = DAT_00184828 + 1;
            } while (DAT_00184828 < plVar14 + uVar19);
          }
        }
        if (DAT_00184820 == (char *)0x0) {
          DAT_00183f28 = stdout;
          if (DAT_00184a6a != '\0') {
            DAT_00183f28 = stderr;
          }
        }
        else {
          uStack_c0 = 0x12efaf;
          DAT_00183f28 = fopen(DAT_00184820,"w");
          if (DAT_00183f28 == (FILE *)0x0) {
            uStack_c0 = 0x12f581;
            FUN_001327e0(DAT_00184820);
            goto LAB_0012f581;
          }
        }
        DAT_00184828 = DAT_00184838;
        if (local_60 != (char *)0x0) {
          uStack_c0 = 0x12efdc;
          DAT_00184db8 = FUN_0014c7a0();
        }
        if (DAT_00184b84 != '\0') {
          uStack_c0 = 0x12f15e;
          DAT_00184b80 = FUN_0013e360("--backup",local_58);
          if (((DAT_00184b80 == 0) || (DAT_00184a6a != '\0')) || (DAT_00184a60 != 0)) {
            DAT_00184b84 = '\0';
          }
        }
        uStack_c0 = 0x12eff5;
        FUN_001107a0();
        uStack_c0 = 0x12efff;
        FUN_0012c160(local_70);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return;
        }
LAB_0012f581:
                    /* WARNING: Subroutine does not return */
        uStack_c0 = 0x12f586;
        __stack_chk_fail();
      }
      uStack_c0 = 0x12ebb3;
      piVar10 = (int *)FUN_0012bcb0(6);
      uStack_c0 = 0x12ebc0;
      uVar8 = FUN_0012bcb0(2);
      uStack_c0 = 0x12ebcb;
      iVar5 = FUN_0012bd20(piVar10,uVar8);
      if (iVar5 == 0) {
        if (*piVar10 == 1) {
          DAT_00184b28 = 0;
        }
        else {
          DAT_00184aac = '\0';
        }
        goto LAB_0012ebe4;
      }
      uStack_c0 = 0x12f45f;
      FUN_0012d1b0("--preserve-order","--listed-incremental");
LAB_0012f45f:
      if (DAT_00184d50 != (code *)0x0) {
        uStack_c0 = 0x12f46d;
        (*DAT_00184d50)();
      }
      pcVar6 = "Cowardly refusing to create an empty archive";
      goto LAB_0012f200;
    }
  }
  if (DAT_00184d50 != (code *)0x0) {
    uStack_c0 = 0x12f412;
    (*DAT_00184d50)();
  }
  pcVar6 = "Multiple archive files require \'-M\' option";
LAB_0012f200:
  uStack_c0 = 0x12f207;
  uVar8 = dcgettext(0,pcVar6,5);
  uStack_c0 = 0x12f215;
  error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
  uStack_c0 = 0x12f21f;
  FUN_0012c970(2);
}




// Function: more_options @ 0x2f5b0

void more_options(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined8 local_38;
  undefined8 local_30;
  undefined2 local_28;
  undefined1 local_26;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = 0;
  local_30 = 0;
  local_26 = 0;
  local_20 = 0;
  local_18 = 0;
  local_38 = param_3;
  argp_parse(&PTR_DAT_00181240,param_1,param_2,0x2a,0,&local_38);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: tar_stat_init @ 0x2f630

void tar_stat_init(undefined8 *param_1)

{
  ulong uVar1;
  undefined8 *puVar2;
  
  *param_1 = 0;
  param_1[0x36] = 0;
  puVar2 = (undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8);
  for (uVar1 = (ulong)(((int)param_1 -
                       (int)(undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8)) + 0x1b8U >> 3
                      ); uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  return;
}




// Function: tar_stat_close @ 0x2f670

bool tar_stat_close(undefined8 *param_1)

{
  int iVar1;
  
  if ((DIR *)param_1[0x34] == (DIR *)0x0) {
    if (*(int *)(param_1 + 0x35) < 1) {
      *(undefined4 *)(param_1 + 0x35) = 0;
      return true;
    }
    iVar1 = close(*(int *)(param_1 + 0x35));
  }
  else {
    iVar1 = closedir((DIR *)param_1[0x34]);
  }
  *(undefined4 *)(param_1 + 0x35) = 0;
  param_1[0x34] = 0;
  if (iVar1 != 0) {
    FUN_00124c50(*param_1);
  }
  return iVar1 == 0;
}




// Function: tar_stat_destroy @ 0x2f6f0

void tar_stat_destroy(undefined8 *param_1)

{
  ulong uVar1;
  undefined8 *puVar2;
  byte bVar3;
  
  bVar3 = 0;
  FUN_0012f670();
  FUN_0011c600(param_1[0x2c],param_1[0x2b]);
  free((void *)*param_1);
  free((void *)param_1[1]);
  free((void *)param_1[3]);
  free((void *)param_1[4]);
  free((void *)param_1[5]);
  free((void *)param_1[6]);
  free((void *)param_1[7]);
  free((void *)param_1[9]);
  free((void *)param_1[0x28]);
  free((void *)param_1[0x32]);
  FUN_0011cca0(param_1 + 0x2d);
  FUN_00116ab0(param_1);
  *param_1 = 0;
  param_1[0x36] = 0;
  puVar2 = (undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8);
  for (uVar1 = (ulong)(((int)param_1 -
                       (int)(undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8)) + 0x1b8U >> 3
                      ); uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + (ulong)bVar3 * -2 + 1;
  }
  return;
}




// Function: tar_timespec_cmp @ 0x2f7b0

void tar_timespec_cmp(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  if (DAT_00184400 != 4) {
    param_4 = 0;
    param_2 = 0;
  }
  FUN_0014be80(param_1,param_2,param_3,param_4);
  return;
}




// Function: set_exit_status @ 0x2f7d0

void set_exit_status(int param_1)

{
  if (DAT_00184d58 < param_1) {
    DAT_00184d58 = param_1;
  }
  return;
}




// Function: parse_xform_flags @ 0x2f7f0

undefined8 parse_xform_flags(uint *param_1,undefined4 param_2)

{
  switch(param_2) {
  case 0x48:
    *param_1 = *param_1 & 0xfffffffd;
    return 0;
  default:
    return 1;
  case 0x52:
    *param_1 = *param_1 & 0xfffffffe;
    return 0;
  case 0x53:
    *param_1 = *param_1 & 0xfffffffb;
    return 0;
  case 0x68:
    *param_1 = *param_1 | 2;
    return 0;
  case 0x72:
    *param_1 = *param_1 | 1;
    return 0;
  case 0x73:
    *param_1 = *param_1 | 4;
    return 0;
  }
}




// Function: add_segment @ 0x2f880

void add_segment(long param_1)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  
  puVar2 = (undefined8 *)FUN_0014c5b0(0x20);
  puVar1 = *(undefined8 **)(param_1 + 0x60);
  *puVar2 = 0;
  if (puVar1 != (undefined8 *)0x0) {
    *puVar1 = puVar2;
    *(long *)(param_1 + 0x68) = *(long *)(param_1 + 0x68) + 1;
    *(undefined8 **)(param_1 + 0x60) = puVar2;
    return;
  }
  *(long *)(param_1 + 0x68) = *(long *)(param_1 + 0x68) + 1;
  *(undefined8 **)(param_1 + 0x58) = puVar2;
  *(undefined8 **)(param_1 + 0x60) = puVar2;
  return;
}




// Function: add_case_ctl_segment @ 0x2f8c0

void add_case_ctl_segment(undefined8 param_1,undefined4 param_2)

{
  long lVar1;
  
  lVar1 = FUN_0012f880();
  *(undefined4 *)(lVar1 + 8) = 2;
  *(undefined4 *)(lVar1 + 0x10) = param_2;
  return;
}




// Function: add_backref_segment @ 0x2f8e0

void add_backref_segment(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_0012f880();
  *(undefined4 *)(lVar1 + 8) = 1;
  *(undefined8 *)(lVar1 + 0x10) = param_2;
  return;
}




// Function: add_char_segment @ 0x2f900

void add_char_segment(undefined8 param_1,undefined1 param_2)

{
  long lVar1;
  undefined1 *puVar2;
  
  lVar1 = FUN_0012f880();
  *(undefined4 *)(lVar1 + 8) = 0;
  puVar2 = (undefined1 *)FUN_0014c5b0(2);
  *(undefined1 **)(lVar1 + 0x10) = puVar2;
  *puVar2 = param_2;
  *(undefined1 *)(*(long *)(lVar1 + 0x10) + 1) = 0;
  *(undefined8 *)(lVar1 + 0x18) = 1;
  return;
}




// Function: new_transform @ 0x2f940

void new_transform(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0014c760(0x70);
  if (DAT_00184cc0 != (undefined8 *)0x0) {
    *DAT_00184cc0 = uVar1;
    DAT_00184cc0 = (undefined8 *)uVar1;
    return;
  }
  DAT_00184cc8 = uVar1;
  DAT_00184cc0 = (undefined8 *)uVar1;
  return;
}




// Function: add_literal_segment @ 0x2f990

void add_literal_segment(undefined8 param_1,void *param_2,long param_3)

{
  long lVar1;
  void *__dest;
  size_t __n;
  
  __n = param_3 - (long)param_2;
  if (__n == 0) {
    return;
  }
  lVar1 = FUN_0012f880();
  *(undefined4 *)(lVar1 + 8) = 0;
  __dest = (void *)FUN_0014c5b0(__n + 1);
  *(void **)(lVar1 + 0x10) = __dest;
  memcpy(__dest,param_2,__n);
  *(undefined1 *)(*(long *)(lVar1 + 0x10) + __n) = 0;
  *(size_t *)(lVar1 + 0x18) = __n;
  return;
}




// Function: parse_transform_expr @ 0x2f9f0

char * parse_transform_expr(char *param_1)

{
  char cVar1;
  uint uVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  long lVar7;
  char *pcVar8;
  ulong uVar9;
  undefined8 uVar10;
  undefined4 *puVar11;
  long lVar12;
  char cVar13;
  char cVar14;
  char *pcVar15;
  ulong uVar16;
  ulong uVar17;
  int iVar18;
  int iVar19;
  long in_FS_OFFSET;
  char *local_258;
  char *local_250;
  undefined1 local_248;
  char local_247;
  undefined1 local_246 [518];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar7 = FUN_0012f940();
  uVar3 = DAT_00183b60;
  if (*param_1 == 's') {
    cVar1 = param_1[1];
    if (cVar1 != '\0') {
      uVar17 = 2;
      cVar4 = param_1[2];
      while (cVar4 != '\0') {
        iVar5 = (int)uVar17;
        iVar18 = iVar5 + 1;
        if (cVar1 == cVar4) {
          lVar12 = (long)iVar18;
          pcVar15 = param_1 + lVar12;
          cVar13 = *pcVar15;
          iVar19 = iVar18;
          if (cVar13 != '\0') goto LAB_0012fb74;
          break;
        }
        if (cVar4 == '\\') {
          iVar18 = (iVar5 + 2) - (uint)(param_1[uVar17 + 1] == '\0');
        }
        uVar17 = (ulong)iVar18;
        cVar4 = param_1[uVar17];
      }
    }
    goto LAB_0012ffb5;
  }
  iVar5 = strncmp(param_1,"flags=",6);
  if (iVar5 != 0) goto LAB_0012ffb5;
  cVar1 = param_1[6];
  param_1 = param_1 + 6;
  DAT_00183b60 = 0;
  if (cVar1 != '\0') {
    puVar11 = &DAT_00183b60;
    do {
      if (cVar1 == ';') {
        param_1 = param_1 + 1;
        break;
      }
      iVar5 = FUN_0012f7f0(puVar11);
      if (iVar5 != 0) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        cVar1 = *param_1;
        pcVar15 = "Unknown transform flag: %c";
LAB_0012ff6e:
        uVar10 = dcgettext(0,pcVar15,5);
        error(0,0,uVar10,(int)cVar1);
                    /* WARNING: Subroutine does not return */
        FUN_0012c970(2);
      }
      cVar1 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar1 != '\0');
  }
  goto LAB_0012fa91;
LAB_0012fc30:
  if (cVar14 == ';') goto LAB_0012ff30;
  goto LAB_0012fc3a;
  while( true ) {
    if (cVar13 == '\\') {
      iVar19 = (iVar19 + 1) - (uint)(param_1[lVar12 + 1] == '\0');
    }
    iVar19 = iVar19 + 1;
    lVar12 = (long)iVar19;
    cVar13 = param_1[lVar12];
    if (cVar13 == '\0') break;
LAB_0012fb74:
    if (cVar13 == cVar4) {
      local_250 = param_1 + lVar12 + 1;
      *(undefined4 *)(lVar7 + 8) = 0;
      *(undefined4 *)(lVar7 + 0xc) = uVar3;
      cVar4 = *local_250;
      uVar2 = 0;
      cVar14 = cVar4;
      if ((cVar4 == '\0') || (cVar4 == ';')) goto LAB_0012fc30;
      uVar16 = uVar17;
      goto LAB_0012fbf3;
    }
  }
LAB_0012ffb5:
  if (DAT_00184d50 != (code *)0x0) {
    (*DAT_00184d50)();
  }
  pcVar15 = "Invalid transform expression";
LAB_0012ffcf:
  uVar10 = dcgettext(0,pcVar15,5);
  error(0,0,uVar10);
                    /* WARNING: Subroutine does not return */
  FUN_0012c970(2);
  while( true ) {
    cVar14 = '\0';
    if (cVar4 == '\0') goto LAB_0012fc30;
    iVar5 = (int)uVar16;
    if (cVar4 == ';') break;
LAB_0012fbf3:
    iVar5 = (int)uVar16;
    pcVar8 = local_250;
    if (cVar4 == 'i') {
      uVar2 = uVar2 | 2;
      local_250 = local_250 + 1;
LAB_0012fbdb:
      iVar5 = (int)uVar16;
      cVar4 = pcVar8[1];
    }
    else {
      if ('i' < cVar4) {
        if (cVar4 != 'x') goto LAB_0012fbc2;
        uVar2 = uVar2 | 1;
        local_250 = local_250 + 1;
        goto LAB_0012fbdb;
      }
      if (cVar4 < ':') {
        if ('/' < cVar4) {
          uVar9 = strtoul(local_250,&local_250,0);
          uVar16 = uVar16 & 0xffffffff;
          *(int *)(lVar7 + 0x10) = (int)uVar9;
          pcVar8 = local_250 + -1;
          goto LAB_0012fbdb;
        }
LAB_0012fbc2:
        iVar5 = FUN_0012f7f0(lVar7 + 0xc);
        if (iVar5 == 0) {
          pcVar8 = local_250;
          local_250 = local_250 + 1;
          goto LAB_0012fbdb;
        }
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        pcVar15 = "Unknown flag in transform expression: %c";
        cVar1 = *local_250;
        goto LAB_0012ff6e;
      }
      if (cVar4 != 'g') goto LAB_0012fbc2;
      *(undefined4 *)(lVar7 + 8) = 1;
      cVar4 = local_250[1];
      local_250 = local_250 + 1;
    }
  }
LAB_0012ff30:
  local_250 = local_250 + 1;
LAB_0012fc3a:
  pcVar8 = (char *)FUN_0014c5b0((long)(iVar5 + -1));
  memcpy(pcVar8,param_1 + 2,(long)(iVar5 + -2));
  pcVar8[uVar17 - 2] = '\0';
  iVar6 = FUN_0015ae70(lVar7 + 0x18,pcVar8,uVar2);
  if (iVar6 != 0) {
    FUN_0015af90(iVar6,lVar7 + 0x18,&local_248,0x200);
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar10 = dcgettext(0,"Invalid transform expression: %s",5);
    error(0,0,uVar10,&local_248);
                    /* WARNING: Subroutine does not return */
    FUN_0012c970(2);
  }
  if ((*pcVar8 == '^') || ((iVar5 != 2 && (pcVar8[uVar17 - 3] == '$')))) {
    *(undefined4 *)(lVar7 + 8) = 0;
  }
  iVar19 = iVar19 - iVar18;
  free(pcVar8);
  pcVar8 = (char *)FUN_0014c5b0((long)(iVar19 + 1));
  memcpy(pcVar8,pcVar15,(long)iVar19);
  pcVar8[iVar19] = '\0';
  cVar4 = *pcVar8;
  pcVar15 = pcVar8;
  while (local_258 = pcVar15, cVar4 != '\0') {
    while (cVar4 != '\\') {
      if (cVar4 == '&') {
        FUN_0012f990(lVar7,pcVar15);
        FUN_0012f8e0(lVar7,0);
        local_258 = local_258 + 1;
        pcVar15 = local_258;
      }
      else {
        local_258 = local_258 + 1;
      }
      cVar4 = *local_258;
      if (cVar4 == '\0') goto LAB_0012fe40;
    }
    FUN_0012f990(lVar7);
    pcVar15 = local_258 + 1;
    cVar4 = local_258[1];
    local_258 = pcVar15;
    switch(cVar4) {
    case '&':
      uVar10 = 0x26;
      break;
    default:
      if (cVar4 == cVar13) {
        FUN_0012f900(lVar7,(int)cVar1);
      }
      else {
        local_248 = 0x5c;
        local_247 = cVar4;
        FUN_0012f990(lVar7,&local_248,local_246);
      }
      local_258 = local_258 + 1;
      goto LAB_0012fe28;
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
      uVar17 = strtoul(pcVar15,&local_258,10);
      if (uVar17 <= *(ulong *)(lVar7 + 0x48)) {
        FUN_0012f8e0(lVar7);
        goto LAB_0012fe28;
      }
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      pcVar15 = "Invalid transform replacement: back reference out of range";
      goto LAB_0012ffcf;
    case 'E':
      uVar10 = 0;
      goto LAB_0012fe5f;
    case 'L':
      uVar10 = 4;
      goto LAB_0012fe5f;
    case 'U':
      uVar10 = 3;
      goto LAB_0012fe5f;
    case '\\':
      uVar10 = 0x5c;
      break;
    case 'a':
      uVar10 = 7;
      break;
    case 'b':
      uVar10 = 8;
      break;
    case 'f':
      uVar10 = 0xc;
      break;
    case 'l':
      uVar10 = 2;
      goto LAB_0012fe5f;
    case 'n':
      uVar10 = 10;
      break;
    case 'r':
      uVar10 = 0xd;
      break;
    case 't':
      uVar10 = 9;
      break;
    case 'u':
      uVar10 = 1;
LAB_0012fe5f:
      FUN_0012f8c0(lVar7,uVar10);
      local_258 = local_258 + 1;
      goto LAB_0012fe28;
    case 'v':
      uVar10 = 0xb;
    }
    FUN_0012f900(lVar7,uVar10);
    local_258 = local_258 + 1;
LAB_0012fe28:
    pcVar15 = local_258;
    cVar4 = *local_258;
  }
LAB_0012fe40:
  FUN_0012f990(lVar7);
  free(pcVar8);
  param_1 = local_250;
LAB_0012fa91:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_1;
}




// Function: run_case_conv @ 0x30070

byte * run_case_conv(uint param_1,void *param_2,ulong param_3)

{
  byte *pbVar1;
  __int32_t **pp_Var2;
  byte *pbVar3;
  byte *pbVar4;
  
  if (DAT_00184c40 < param_3) {
    DAT_00184c40 = param_3;
    DAT_00184cb8 = (byte *)FUN_0014c610(DAT_00184cb8,param_3);
  }
  pbVar1 = DAT_00184cb8;
  memcpy(DAT_00184cb8,param_2,param_3);
  if (param_1 == 3) {
    if (pbVar1 < pbVar1 + param_3) {
      pp_Var2 = __ctype_toupper_loc();
      pbVar3 = pbVar1;
      do {
        pbVar4 = pbVar3 + 1;
        *pbVar3 = (byte)(*pp_Var2)[*pbVar3];
        pbVar3 = pbVar4;
      } while (pbVar4 != pbVar1 + param_3);
      return pbVar1;
    }
  }
  else if (param_1 < 4) {
    if (param_1 == 1) {
      pp_Var2 = __ctype_toupper_loc();
      *pbVar1 = (byte)(*pp_Var2)[*pbVar1];
      return pbVar1;
    }
    if (param_1 == 2) {
      pp_Var2 = __ctype_tolower_loc();
      *pbVar1 = (byte)(*pp_Var2)[*pbVar1];
    }
  }
  else if ((param_1 == 4) && (pbVar1 < pbVar1 + param_3)) {
    pp_Var2 = __ctype_tolower_loc();
    pbVar3 = pbVar1;
    do {
      pbVar4 = pbVar3 + 1;
      *pbVar3 = (byte)(*pp_Var2)[*pbVar3];
      pbVar3 = pbVar4;
    } while (pbVar4 != pbVar1 + param_3);
    return pbVar1;
  }
  return pbVar1;
}




// Function: _single_transform_name_to_obstack @ 0x301b0

void _single_transform_name_to_obstack(long param_1,char *param_2)

{
  uint uVar1;
  ulong uVar2;
  undefined8 *puVar3;
  uint uVar4;
  undefined1 *puVar5;
  int iVar6;
  ulong *__ptr;
  ulong __n;
  size_t __n_00;
  ulong uVar7;
  char *pcVar8;
  uint uVar9;
  uint uVar10;
  ulong local_68;
  uint local_44;
  
  __ptr = (ulong *)FUN_0014c5b0((*(long *)(param_1 + 0x48) + 1) * 0x10);
  if (*param_2 != '\0') {
    local_44 = 0;
    uVar9 = 0;
    local_68 = 0;
    do {
      iVar6 = FUN_0015b070(param_1 + 0x18,param_2,*(long *)(param_1 + 0x48) + 1,__ptr,0);
      if (iVar6 == 0) {
        uVar2 = *__ptr;
        __n = __ptr[1];
        if (uVar2 != 0) {
          if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < uVar2) {
            _obstack_newchunk(&DAT_00184c60,uVar2);
          }
          memcpy(DAT_00184c78,param_2,uVar2);
          DAT_00184c78 = DAT_00184c78 + uVar2;
        }
        local_68 = local_68 + 1;
        if ((*(uint *)(param_1 + 0x10) == 0) || (*(uint *)(param_1 + 0x10) <= local_68)) {
          uVar4 = local_44;
          for (puVar3 = *(undefined8 **)(param_1 + 0x58); local_44 = uVar4,
              puVar3 != (undefined8 *)0x0; puVar3 = (undefined8 *)*puVar3) {
            iVar6 = *(int *)(puVar3 + 1);
            uVar10 = uVar9;
            if (iVar6 == 1) {
              uVar2 = __ptr[puVar3[2] * 2];
              if ((uVar2 != 0xffffffffffffffff) &&
                 (uVar7 = (__ptr + puVar3[2] * 2)[1], uVar7 != 0xffffffffffffffff)) {
                uVar7 = uVar7 - uVar2;
                pcVar8 = param_2 + uVar2;
                if ((uVar9 != 0) && (pcVar8 = (char *)FUN_00130070(uVar9,pcVar8), uVar9 - 1 < 2)) {
                  local_44 = 0;
                  uVar9 = uVar4;
                }
                if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < uVar7) {
LAB_0013044f:
                  _obstack_newchunk(&DAT_00184c60,uVar7);
                }
LAB_00130405:
                memcpy(DAT_00184c78,pcVar8,uVar7);
                DAT_00184c78 = DAT_00184c78 + uVar7;
                uVar10 = uVar9;
              }
            }
            else if (iVar6 == 2) {
              uVar1 = *(uint *)(puVar3 + 2);
              uVar10 = uVar1;
              if (uVar1 < 3) {
                if (uVar1 == 0) {
                  uVar10 = 0;
                }
                else {
                  local_44 = uVar9;
                  if ((uVar4 != 0) && (1 < uVar4 - 3)) {
                    local_44 = uVar4;
                  }
                }
              }
              else {
                uVar10 = uVar9;
                if (uVar1 - 3 < 2) {
                  uVar10 = uVar1;
                }
              }
            }
            else if (iVar6 == 0) {
              if (uVar9 == 0) {
                pcVar8 = (char *)puVar3[2];
              }
              else {
                pcVar8 = (char *)FUN_00130070(uVar9,puVar3[2],puVar3[3]);
                if (uVar9 - 1 < 2) {
                  local_44 = 0;
                  uVar9 = uVar4;
                }
              }
              uVar7 = puVar3[3];
              if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < uVar7) goto LAB_0013044f;
              goto LAB_00130405;
            }
            uVar9 = uVar10;
            uVar4 = local_44;
          }
          goto LAB_001302f9;
        }
        if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < __n) {
          _obstack_newchunk(&DAT_00184c60,__n);
        }
        pcVar8 = param_2 + __n;
        memcpy(DAT_00184c78,param_2,__n);
        DAT_00184c78 = DAT_00184c78 + __n;
      }
      else {
        __n = strlen(param_2);
        if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < __n) {
          _obstack_newchunk(&DAT_00184c60,__n);
        }
        memcpy(DAT_00184c78,param_2,__n);
        DAT_00184c78 = DAT_00184c78 + __n;
LAB_001302f9:
        pcVar8 = param_2 + __n;
        if (*(int *)(param_1 + 8) == 0) {
          __n_00 = strlen(pcVar8);
          if ((ulong)((long)DAT_00184c80 - (long)DAT_00184c78) < __n_00) {
            _obstack_newchunk(&DAT_00184c60,__n_00);
          }
          memcpy(DAT_00184c78,pcVar8,__n_00);
          DAT_00184c78 = DAT_00184c78 + __n_00;
          if (DAT_00184c80 == DAT_00184c78) goto LAB_00130352;
          goto LAB_001305a4;
        }
      }
      param_2 = pcVar8;
    } while (*pcVar8 != '\0');
  }
  if (DAT_00184c80 == DAT_00184c78) {
LAB_00130352:
    _obstack_newchunk(&DAT_00184c60,1);
  }
LAB_001305a4:
  puVar5 = DAT_00184c78 + 1;
  *DAT_00184c78 = 0;
  DAT_00184c78 = puVar5;
  free(__ptr);
  return;
}




// Function: _transform_name_to_obstack @ 0x30600

undefined8 _transform_name_to_obstack(uint param_1,ulong param_2,ulong *param_3)

{
  ulong uVar1;
  long *plVar2;
  undefined8 uVar3;
  
  if (DAT_00184c48 == '\0') {
    _obstack_begin(&DAT_00184c60,0,0,FUN_0014c5b0,PTR_free_00180fb8);
    DAT_00184c48 = '\x01';
  }
  uVar3 = 0;
  for (plVar2 = DAT_00184cc8; plVar2 != (long *)0x0; plVar2 = (long *)*plVar2) {
    while ((param_1 & *(uint *)((long)plVar2 + 0xc)) == 0) {
      plVar2 = (long *)*plVar2;
      if (plVar2 == (long *)0x0) goto LAB_001306b4;
    }
    FUN_001301b0(plVar2,param_2);
    if (DAT_00184c78 == DAT_00184c70) {
      DAT_00184cb0 = DAT_00184cb0 | 2;
    }
    uVar3 = 1;
    uVar1 = DAT_00184c78 + DAT_00184c90 & ~DAT_00184c90;
    DAT_00184c78 = DAT_00184c80;
    if (uVar1 - DAT_00184c68 <= DAT_00184c80 - DAT_00184c68) {
      DAT_00184c78 = uVar1;
    }
    param_2 = DAT_00184c70;
    DAT_00184c70 = DAT_00184c78;
  }
LAB_001306b4:
  *param_3 = param_2;
  return uVar3;
}




// Function: set_transform_expr @ 0x30710

void set_transform_expr(char *param_1)

{
  if (*param_1 == '\0') {
    return;
  }
  do {
    param_1 = (char *)FUN_0012f9f0(param_1);
  } while (*param_1 != '\0');
  return;
}




// Function: transform_name_fp @ 0x30740

undefined4 transform_name_fp(undefined8 *param_1,undefined4 param_2,code *param_3,undefined8 param_4)

{
  undefined4 uVar1;
  void *pvVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  void *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00130600(param_2,*param_1,&local_38);
  if ((char)uVar1 == '\0') {
    if (param_3 != (code *)0x0) {
      *param_1 = 0;
      uVar3 = (*param_3)(local_38,param_4);
      FUN_00123be0(param_1,uVar3);
      uVar1 = 1;
      free(local_38);
    }
  }
  else {
    pvVar2 = local_38;
    if (param_3 != (code *)0x0) {
      pvVar2 = (void *)(*param_3)(local_38,param_4);
    }
    FUN_00123be0(param_1,pvVar2);
    if ((DAT_00184c68 < local_38) && (local_38 < DAT_00184c80)) {
      DAT_00184c70 = local_38;
      DAT_00184c78 = local_38;
    }
    else {
      _obstack_free(&DAT_00184c60,local_38);
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: transform_name @ 0x30830

void transform_name(undefined8 param_1,undefined8 param_2)

{
  FUN_00130740(param_1,param_2,0,0);
  return;
}




// Function: transform_program_p @ 0x30840

bool transform_program_p(void)

{
  return DAT_00184cc8 != 0;
}




// Function: dunlink_insert @ 0x30850

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void dunlink_insert(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  
  puVar1 = DAT_00184ce8;
  if (param_1 == (undefined8 *)0x0) {
    puVar2 = param_2;
    *param_2 = DAT_00184ce8;
    DAT_00184ce8 = puVar2;
  }
  else {
    *param_2 = *param_1;
    *param_1 = param_2;
    puVar1 = (undefined8 *)*param_2;
  }
  if (puVar1 != (undefined8 *)0x0) {
    _DAT_00184cd8 = _DAT_00184cd8 + 1;
    return;
  }
  _DAT_00184cd8 = _DAT_00184cd8 + 1;
  DAT_00184ce0 = param_2;
  return;
}




// Function: dunlink_reclaim @ 0x308a0

void dunlink_reclaim(undefined8 *param_1)

{
  undefined8 *puVar1;
  
  free((void *)param_1[2]);
  puVar1 = param_1;
  *param_1 = DAT_00184cd0;
  DAT_00184cd0 = puVar1;
  return;
}




// Function: flush_deferred_unlinks @ 0x308c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void flush_deferred_unlinks(char param_1)

{
  long *plVar1;
  undefined4 uVar2;
  long *plVar3;
  int iVar4;
  int *piVar5;
  char *pcVar6;
  long *plVar7;
  long *plVar8;
  
  uVar2 = DAT_00184580;
  plVar7 = (long *)0x0;
  plVar8 = DAT_00184ce8;
  if (DAT_00184ce8 != (long *)0x0) {
    do {
      while ((plVar1 = (long *)*plVar8, param_1 == '\0' &&
             (plVar3 = DAT_00184ce8, DAT_00183f30 <= (ulong)plVar8[4]))) {
LAB_00130903:
        DAT_00184ce8 = plVar3;
        plVar7 = plVar8;
        plVar8 = plVar1;
        if (plVar1 == (long *)0x0) goto LAB_001309b5;
      }
      FUN_00124c80((int)plVar8[1]);
      if ((char)plVar8[3] == '\0') {
        iVar4 = unlinkat(DAT_00181234,(char *)plVar8[2],0);
        if ((iVar4 != 0) && (piVar5 = __errno_location(), *piVar5 != 2)) {
          FUN_00132db0(plVar8[2]);
        }
      }
      else {
        pcVar6 = (char *)plVar8[2];
        if (((int)plVar8[1] != 0) &&
           ((plVar3 = DAT_00184ce8, *pcVar6 == '\0' ||
            (iVar4 = strcmp(pcVar6,"."), plVar3 = DAT_00184ce8, iVar4 == 0)))) goto LAB_00130903;
        iVar4 = unlinkat(DAT_00181234,pcVar6,0x200);
        if (iVar4 != 0) {
          piVar5 = __errno_location();
          iVar4 = *piVar5;
          plVar3 = DAT_00184ce8;
          if ((iVar4 == 0x11) || (iVar4 == 0x27)) goto LAB_00130903;
          if (iVar4 != 2) {
            FUN_00132aa0(pcVar6);
          }
        }
      }
      FUN_001308a0(plVar8);
      _DAT_00184cd8 = _DAT_00184cd8 + -1;
      plVar8 = plVar7;
      plVar3 = plVar1;
      if (plVar7 == (long *)0x0) goto LAB_00130903;
      *plVar7 = (long)plVar1;
      plVar8 = plVar1;
    } while (plVar1 != (long *)0x0);
LAB_001309b5:
    if (DAT_00184ce8 != (long *)0x0) {
      plVar8 = DAT_00184ce8;
      if (param_1 != '\0') {
        do {
          plVar7 = (long *)*plVar8;
          FUN_00124c80((int)plVar8[1]);
          pcVar6 = (char *)plVar8[2];
          if ((((int)plVar8[1] != 0) && ((char)plVar8[3] != '\0')) &&
             ((*pcVar6 == '\0' || (iVar4 = strcmp(pcVar6,"."), iVar4 == 0)))) {
            pcVar6 = (char *)FUN_00124c30();
            FUN_00124c80((int)plVar8[1] + -1);
          }
          iVar4 = unlinkat(DAT_00181234,pcVar6,0x200);
          if ((iVar4 != 0) && (piVar5 = __errno_location(), *piVar5 != 2)) {
            FUN_00132aa0(pcVar6);
          }
          FUN_001308a0(plVar8);
          _DAT_00184cd8 = _DAT_00184cd8 + -1;
          plVar8 = plVar7;
        } while (plVar7 != (long *)0x0);
        DAT_00184ce0 = 0;
        DAT_00184ce8 = (long *)0x0;
      }
      goto LAB_00130a88;
    }
  }
  DAT_00184ce0 = 0;
LAB_00130a88:
  FUN_00124c80(uVar2);
  return;
}




// Function: dunlink_alloc @ 0x30b00

void dunlink_alloc(void)

{
  undefined8 uVar1;
  
  if (DAT_00184cd0 != (undefined8 *)0x0) {
    uVar1 = *DAT_00184cd0;
    *DAT_00184cd0 = 0;
    DAT_00184cd0 = (undefined8 *)uVar1;
    return;
  }
  FUN_0014c5b0(0x28);
  return;
}




// Function: finish_deferred_unlinks @ 0x30b30

void finish_deferred_unlinks(void)

{
  undefined8 *puVar1;
  
  FUN_001308c0(1);
  while (DAT_00184cd0 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*DAT_00184cd0;
    free(DAT_00184cd0);
    DAT_00184cd0 = puVar1;
  }
  return;
}




// Function: queue_deferred_unlink @ 0x30b70

void queue_deferred_unlink(undefined8 param_1,char param_2)

{
  ulong uVar1;
  long *plVar2;
  int iVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  long *plVar6;
  long *plVar7;
  
  if ((DAT_00184ce8 != (long *)0x0) && ((ulong)DAT_00184ce8[4] < DAT_00183f30)) {
    FUN_001308c0(0);
  }
  puVar4 = (undefined8 *)FUN_00130b00();
  *puVar4 = 0;
  *(undefined4 *)(puVar4 + 1) = DAT_00184580;
  uVar5 = FUN_0014c7a0(param_1);
  puVar4[2] = uVar5;
  FUN_00123ea0(uVar5);
  uVar1 = DAT_00183f30;
  *(char *)(puVar4 + 3) = param_2;
  puVar4[4] = uVar1;
  if ((param_2 != '\0') &&
     (((*(char *)puVar4[2] == '\0' || (iVar3 = strcmp((char *)puVar4[2],"."), iVar3 == 0)) &&
      (DAT_00184ce8 != (long *)0x0)))) {
    plVar2 = DAT_00184ce8;
    plVar7 = (long *)0x0;
    do {
      plVar6 = plVar2;
      if ((((char)plVar6[3] != '\0') &&
          ((*(char *)plVar6[2] == '\0' || (iVar3 = strcmp((char *)plVar6[2],"."), iVar3 == 0)))) &&
         ((int)plVar6[1] < *(int *)(puVar4 + 1))) {
        FUN_00130850(plVar7,puVar4);
        return;
      }
      plVar2 = (long *)*plVar6;
      plVar7 = plVar6;
    } while ((long *)*plVar6 != (long *)0x0);
  }
  FUN_00130850(DAT_00184ce0,puVar4);
  return;
}




// Function: append_file @ 0x30ca0

void append_file(char *param_1)

{
  ulong uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  long lVar5;
  long lVar6;
  ulong uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulong uVar11;
  undefined8 *puVar12;
  long in_FS_OFFSET;
  byte bVar13;
  stat sStack_e8;
  undefined1 local_58 [24];
  long local_40;
  
  bVar13 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = openat(DAT_00181234,param_1,0);
  if (iVar2 < 0) {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_001327c0(param_1);
      return;
    }
  }
  else {
    iVar3 = fstat(iVar2,&sStack_e8);
    uVar1 = sStack_e8.st_size;
    if (iVar3 == 0) {
      for (; 0 < (long)uVar1; uVar1 = uVar1 - lVar5) {
        lVar6 = FUN_0010e130();
        uVar7 = FUN_0010d2b0(lVar6);
        if ((uVar1 < uVar7) && (uVar7 = uVar1, (uVar1 & 0x1ff) != 0)) {
          uVar4 = 0x200 - ((uint)uVar1 & 0x1ff);
          puVar12 = (undefined8 *)(lVar6 + uVar1);
          if (uVar4 < 8) {
            if ((uVar4 & 4) == 0) {
              if ((uVar4 != 0) && (*(undefined1 *)puVar12 = 0, (uVar4 & 2) != 0)) {
                *(undefined2 *)((long)puVar12 + ((ulong)uVar4 - 2)) = 0;
              }
            }
            else {
              *(undefined4 *)puVar12 = 0;
              *(undefined4 *)((long)puVar12 + ((ulong)uVar4 - 4)) = 0;
            }
          }
          else {
            *puVar12 = 0;
            *(undefined8 *)((long)puVar12 + ((ulong)uVar4 - 8)) = 0;
            uVar11 = (ulong)(uVar4 + ((int)puVar12 -
                                     (int)(undefined8 *)((ulong)(puVar12 + 1) & 0xfffffffffffffff8))
                            >> 3);
            puVar12 = (undefined8 *)((ulong)(puVar12 + 1) & 0xfffffffffffffff8);
            for (; uVar11 != 0; uVar11 = uVar11 - 1) {
              *puVar12 = 0;
              puVar12 = puVar12 + (ulong)bVar13 * -2 + 1;
            }
          }
        }
        lVar5 = FUN_0014af10(iVar2,lVar6,uVar7);
        if (lVar5 == -1) {
          FUN_001329d0(param_1,sStack_e8.st_size - uVar1,uVar7);
LAB_00130e83:
          if (DAT_00184d50 != (code *)0x0) {
            (*DAT_00184d50)();
          }
          uVar8 = FUN_00143090(uVar1,local_58);
          uVar9 = FUN_0014aa70(param_1);
          uVar10 = dcngettext(0,"%s: File shrank by %s byte","%s: File shrank by %s bytes",uVar1,5);
          error(0,0,uVar10,uVar9,uVar8);
                    /* WARNING: Subroutine does not return */
          FUN_001163d0();
        }
        if (lVar5 == 0) goto LAB_00130e83;
        FUN_0010d1f0(lVar6 + (lVar5 - 1U & 0xfffffffffffffe00));
      }
    }
    else {
      FUN_00132d30(param_1);
    }
    iVar2 = close(iVar2);
    if (iVar2 != 0) {
      FUN_00132680(param_1);
    }
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: update_archive @ 0x30ef0

void update_archive(void)

{
  char *__s;
  undefined4 uVar1;
  char cVar2;
  uint uVar3;
  int iVar4;
  long lVar5;
  undefined8 uVar6;
  char *__ptr;
  undefined8 uVar7;
  size_t sVar8;
  uint uVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  undefined1 local_d8 [24];
  uint local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00126d90();
  FUN_0010fb70(2);
  FUN_0011c560();
  uVar9 = 0;
LAB_00130f58:
  uVar3 = FUN_001219e0(&DAT_00184408,&DAT_00184840,0);
  switch(uVar3) {
  case 0:
  case 2:
switchD_00130f7a_caseD_0:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    FUN_001215c0(DAT_00184408,&DAT_00184840,&DAT_00184400,0);
    FUN_00121440((int)*(char *)(DAT_00184408 + 0x9c),&DAT_00184840);
    DAT_00184ba4 = DAT_00184400;
    if ((DAT_00184ba8 == 8) && (lVar5 = FUN_00127aa0(DAT_00184848), lVar5 != 0)) {
      FUN_00124c80(*(undefined4 *)(lVar5 + 0x28));
      iVar4 = FUN_001246f0(DAT_00184848,local_d8);
      if (iVar4 == 0) {
        if ((local_c0 & 0xf000) == 0x4000) {
          __ptr = (char *)FUN_001253c0(*(undefined8 *)(lVar5 + 0x10),1);
          if (__ptr != (char *)0x0) {
            uVar6 = FUN_00125040(*(undefined8 *)(lVar5 + 0x10));
            cVar2 = *__ptr;
            __s = __ptr;
            while (cVar2 != '\0') {
              uVar1 = *(undefined4 *)(lVar5 + 0x28);
              uVar7 = FUN_001250d0(uVar6,__s);
              FUN_00126cf0(uVar7,uVar1,0);
              sVar8 = strlen(__s);
              __s = __s + sVar8 + 1;
              cVar2 = *__s;
            }
            FUN_001250b0(uVar6);
            free(__ptr);
            FUN_00127150(lVar5);
          }
        }
        else {
          auVar10 = FUN_0014bd20(local_d8);
          iVar4 = FUN_0012f7b0(auVar10._0_8_,auVar10._8_8_,DAT_00184938);
          if (iVar4 < 1) {
            FUN_00127150(lVar5);
          }
        }
      }
    }
    FUN_00122bd0();
    FUN_0012f6f0(&DAT_00184840);
    uVar9 = uVar3;
    goto LAB_00130f58;
  case 3:
    DAT_00183f48 = DAT_00184408;
    FUN_0012f6f0(&DAT_00184840);
    break;
  case 4:
    FUN_0012f6f0(&DAT_00184840);
    break;
  case 5:
    FUN_0010d1f0(DAT_00184408);
    if (uVar9 != 1) {
      if (uVar9 < 2) {
        if (DAT_00184d50 != (code *)0x0) {
          (*DAT_00184d50)();
        }
        uVar6 = dcgettext(0,"This does not look like a tar archive",5);
        error(0,0,uVar6);
      }
      else if (uVar9 != 3) {
        if (uVar9 != 4) goto switchD_00130f7a_default;
        goto switchD_00130f7a_caseD_0;
      }
    }
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar6 = dcgettext(0,"Skipping to next header",5);
    error(0,0,uVar6);
    DAT_00184d58 = 2;
    FUN_0012f6f0(&DAT_00184840);
    uVar9 = uVar3;
    goto LAB_00130f58;
  default:
    goto switchD_00130f7a_default;
  }
  FUN_0010d1a0();
  DAT_00184cf8 = 1;
  DAT_00184cf0 = DAT_00183f48;
LAB_00130fb0:
  do {
    lVar5 = FUN_00127b00();
    while( true ) {
      if (lVar5 == 0) {
        FUN_00112c40();
        FUN_0010f900();
        FUN_00130b30();
        if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        FUN_00127430();
        return;
      }
      uVar6 = *(undefined8 *)(lVar5 + 0x10);
      cVar2 = FUN_00116b00(uVar6,0);
      if ((cVar2 != '\0') ||
         ((DAT_00184b40 != '\0' && (iVar4 = FUN_0012ca20(&DAT_00160e31), iVar4 == 0))))
      goto LAB_00130fb0;
      if (DAT_00184ba8 == 2) break;
      FUN_00114330(0,uVar6,uVar6);
      lVar5 = FUN_00127b00();
    }
    FUN_00130ca0(uVar6);
  } while( true );
switchD_00130f7a_default:
  FUN_0012f6f0(&DAT_00184840);
  uVar9 = uVar3;
  goto LAB_00130f58;
}




// Function: utf8_init @ 0x312d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void utf8_init(byte param_1)

{
  char *pcVar1;
  
  if (*(long *)(&DAT_00183b70 + (ulong)param_1 * 8) != -1) {
    return;
  }
  if (param_1 == 0) {
    pcVar1 = (char *)FUN_001430f0();
    _DAT_00183b70 = iconv_open(pcVar1,"UTF-8");
    return;
  }
  pcVar1 = (char *)FUN_001430f0();
  _DAT_00183b78 = iconv_open("UTF-8",pcVar1);
  return;
}




// Function: utf8_convert @ 0x31340

undefined8 utf8_convert(undefined1 param_1,char *param_2,undefined8 *param_3)

{
  iconv_t __cd;
  char *__ptr;
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  char *local_50;
  char *local_48;
  size_t local_40;
  size_t local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __cd = (iconv_t)FUN_001312d0(param_1);
  if (__cd == (iconv_t)0x0) {
    uVar3 = 1;
    uVar2 = FUN_0014c7a0(param_2);
    *param_3 = uVar2;
  }
  else {
    uVar3 = 0;
    if (__cd != (iconv_t)0xffffffffffffffff) {
      local_40 = strlen(param_2);
      local_40 = local_40 + 1;
      local_38 = local_40 * 0x10 + 1;
      __ptr = (char *)FUN_0014c5b0();
      local_50 = param_2;
      local_48 = __ptr;
      sVar1 = iconv(__cd,&local_50,&local_40,&local_48,&local_38);
      if (sVar1 == 0) {
        uVar3 = 1;
        *local_48 = '\0';
        *param_3 = __ptr;
      }
      else {
        free(__ptr);
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_warning_option @ 0x31470

void set_warning_option(char *param_1)

{
  undefined *puVar1;
  int iVar2;
  size_t sVar3;
  long lVar4;
  
  iVar2 = strcmp(param_1,"none");
  if (iVar2 == 0) {
    DAT_00183b80 = 0;
    return;
  }
  sVar3 = strlen(param_1);
  puVar1 = PTR_FUN_00183dd8;
  if ((2 < sVar3) && (iVar2 = memcmp(param_1,&DAT_0016708a,3), iVar2 == 0)) {
    lVar4 = FUN_00139670("--warning",param_1 + 3,&PTR_s_all_001629f1_0xc_0017fa20,&DAT_00183ba0,4,
                         puVar1);
    DAT_00183b80 = ~*(uint *)(&DAT_00183ba0 + lVar4 * 4) & DAT_00183b80;
    return;
  }
  lVar4 = FUN_00139670("--warning",param_1,&PTR_s_all_001629f1_0xc_0017fa20,&DAT_00183ba0,4,puVar1);
  DAT_00183b80 = DAT_00183b80 | *(uint *)(&DAT_00183ba0 + lVar4 * 4);
  return;
}




// Function: mask_map_realloc @ 0x31550

void mask_map_realloc(undefined8 *param_1)

{
  undefined8 uVar1;
  
  if (param_1[2] != param_1[1]) {
    return;
  }
  if (param_1[1] == 0) {
    param_1[1] = 4;
  }
  uVar1 = FUN_0014c680(*param_1,param_1 + 1,8);
  *param_1 = uVar1;
  return;
}




// Function: xattrs_kw_included @ 0x31600

ulong xattrs_kw_included(char *param_1,ulong param_2)

{
  int iVar1;
  undefined4 extraout_var;
  ulong uVar2;
  
  if (DAT_00184d28 != 0) {
    uVar2 = FUN_00131590(param_1,&DAT_00184d20);
    return uVar2;
  }
  if ((char)param_2 != '\0') {
    return param_2 & 0xffffffff;
  }
  iVar1 = strncmp(param_1,"user.",5);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: xattrs_masked_out @ 0x31730

undefined8 xattrs_masked_out(undefined8 param_1,undefined1 param_2)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00131600(param_1,param_2);
  if (cVar1 == '\0') {
    return 1;
  }
  uVar2 = FUN_00131710(param_1);
  return uVar2;
}




// Function: xattrs_acls_get @ 0x319f0

void xattrs_acls_get(void)

{
  undefined8 uVar1;
  
  if (DAT_00184a9c < 1) {
    return;
  }
  if (DAT_00184d14 == 0) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar1 = dcgettext(0,"POSIX ACL support is not available",5);
    error(0,0,uVar1);
    DAT_00184d14 = 1;
    return;
  }
  DAT_00184d14 = 1;
  return;
}




// Function: xattrs_acls_set @ 0x31a70

void xattrs_acls_set(undefined8 param_1,undefined8 param_2,char param_3)

{
  undefined8 uVar1;
  
  if ((0 < DAT_00184a9c) && (param_3 != '2')) {
    if (DAT_00184d10 == 0) {
      if (DAT_00184d50 != (code *)0x0) {
        (*DAT_00184d50)();
      }
      uVar1 = dcgettext(0,"POSIX ACL support is not available",5);
      error(0,0,uVar1);
      DAT_00184d10 = 1;
      return;
    }
    DAT_00184d10 = 1;
    return;
  }
  return;
}




// Function: xattrs_mask_add @ 0x31af0

void xattrs_mask_add(undefined8 param_1,char param_2)

{
  long lVar1;
  long *plVar2;
  
  plVar2 = &DAT_00184d20;
  if (param_2 == '\0') {
    plVar2 = &DAT_00184d38;
  }
  FUN_00131550(plVar2);
  lVar1 = plVar2[2];
  plVar2[2] = lVar1 + 1;
  *(undefined8 *)(*plVar2 + lVar1 * 8) = param_1;
  return;
}




// Function: xattrs_clear_setup @ 0x31b40

void xattrs_clear_setup(void)

{
  FUN_001319d0(DAT_00184d20,DAT_00184d28);
  FUN_001319d0(DAT_00184d38,DAT_00184d40);
  return;
}




// Function: xattrs_xattrs_get @ 0x31b80

void xattrs_xattrs_get(undefined4 param_1,undefined8 param_2,undefined8 param_3,int param_4)

{
  char cVar1;
  long lVar2;
  char *pcVar3;
  size_t sVar4;
  int *piVar5;
  long lVar6;
  char *pcVar7;
  
  if (0 < DAT_00184a98) {
    if (DAT_00184d08 == (char *)0x0) {
      DAT_00184d08 = (char *)FUN_0014c680(0,&DAT_00183c10,1);
    }
    while( true ) {
      pcVar7 = DAT_00184d08;
      if (param_4 == 0) {
        lVar2 = FUN_00138f90(param_1,param_2,DAT_00184d08,DAT_00183c10);
        pcVar7 = DAT_00184d08;
      }
      else {
        lVar2 = flistxattr(param_4,DAT_00184d08,DAT_00183c10);
      }
      if (lVar2 != -1) break;
      piVar5 = __errno_location();
      if (*piVar5 != 0x22) {
        pcVar7 = "llistxattrat";
        if (param_4 != 0) {
          pcVar7 = "flistxattr";
        }
        FUN_001324f0(pcVar7,param_2);
        return;
      }
      DAT_00184d08 = (char *)FUN_0014c680(DAT_00184d08,&DAT_00183c10,1);
    }
    if (DAT_00184d00 == (void *)0x0) {
      DAT_00184d00 = (void *)FUN_0014c680(0,&DAT_00183c08,1);
    }
    if (0 < lVar2) {
      pcVar3 = "fgetxattr";
      if (param_4 == 0) {
        pcVar3 = "lgetxattrat";
      }
      do {
        sVar4 = strlen(pcVar7);
        while( true ) {
          if (param_4 == 0) {
            lVar6 = FUN_00138a90(param_1,param_2,pcVar7,DAT_00184d00);
          }
          else {
            lVar6 = fgetxattr(param_4,pcVar7,DAT_00184d00,DAT_00183c08);
          }
          if (lVar6 != -1) {
            cVar1 = FUN_00131730(pcVar7,1);
            if (cVar1 == '\0') {
              FUN_0011c650(param_3,pcVar7,DAT_00184d00,lVar6);
            }
            goto LAB_00131d0c;
          }
          piVar5 = __errno_location();
          if (*piVar5 != 0x22) break;
          DAT_00184d00 = (void *)FUN_0014c680(DAT_00184d00,&DAT_00183c08,1);
        }
        if (*piVar5 != 0x3d) {
          FUN_001324f0(pcVar3,param_2);
        }
LAB_00131d0c:
        pcVar7 = pcVar7 + sVar4 + 1;
        lVar2 = (lVar2 - sVar4) + -1;
      } while (0 < lVar2);
    }
  }
  return;
}




// Function: xattrs_selinux_get @ 0x31de0

void xattrs_selinux_get(undefined8 param_1,undefined8 param_2,long param_3,int param_4)

{
  int iVar1;
  int *piVar2;
  char *pcVar3;
  
  if (DAT_00184aa0 < 1) {
    return;
  }
  if (param_4 == 0) {
    iVar1 = FUN_0014b660();
  }
  else {
    iVar1 = FUN_0014d390(param_4,param_3 + 0x30);
  }
  if (iVar1 == -1) {
    piVar2 = __errno_location();
    if ((*piVar2 != 0x5f) && (*piVar2 != 0x3d)) {
      pcVar3 = "fgetfilecon";
      if (param_4 == 0) {
        pcVar3 = "lgetfileconat";
      }
      FUN_001324f0(pcVar3,param_2);
      return;
    }
  }
  return;
}




// Function: xattrs_selinux_set @ 0x31e70

void xattrs_selinux_set(long param_1,undefined8 param_2,char param_3)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  
  if ((DAT_00184aa0 < 1) || (*(long *)(param_1 + 0x30) == 0)) {
    return;
  }
  if (param_3 == '2') {
    iVar1 = FUN_0014baa0(DAT_00181234,param_2,*(long *)(param_1 + 0x30));
    pcVar4 = "lsetfileconat";
  }
  else {
    iVar1 = FUN_0014b880();
    pcVar4 = "setfileconat";
  }
  if ((iVar1 == -1) && ((DAT_00183b80._2_1_ & 0x20) != 0)) {
    if (DAT_00184d50 != (code *)0x0) {
      (*DAT_00184d50)();
    }
    uVar2 = dcgettext(0,"%s: Cannot set SELinux context for file \'%s\'",5);
    piVar3 = __errno_location();
    error(0,*piVar3,uVar2,pcVar4,param_2);
    return;
  }
  return;
}




// Function: xattrs_xattrs_set @ 0x31f20

void xattrs_xattrs_set(long param_1,undefined8 param_2,char param_3,uint param_4)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  ulong uVar4;
  long lVar5;
  char *__s1;
  
  if ((0 < DAT_00184a98) && (*(long *)(param_1 + 0x158) != 0)) {
    uVar4 = 0;
    do {
      lVar5 = *(long *)(*(long *)(param_1 + 0x160) + uVar4 * 0x18);
      sVar3 = strlen("SCHILY.xattr.");
      __s1 = (char *)(lVar5 + sVar3);
      if (param_3 == '0') {
        iVar2 = strcmp(__s1,"security.capability");
        if ((iVar2 != 0) != param_4) goto LAB_00131fb5;
      }
      else {
LAB_00131fb5:
        cVar1 = FUN_00131730(__s1,0);
        if (cVar1 == '\0') {
          lVar5 = uVar4 * 0x18 + *(long *)(param_1 + 0x160);
          FUN_00131650(param_2,(int)param_3,__s1,*(undefined8 *)(lVar5 + 8),
                       *(undefined8 *)(lVar5 + 0x10));
        }
      }
      uVar4 = uVar4 + 1;
    } while (uVar4 < *(ulong *)(param_1 + 0x158));
  }
  return;
}




// Function: xattrs_print_char @ 0x32010

void xattrs_print_char(long param_1,undefined2 *param_2)

{
  char cVar1;
  size_t sVar2;
  ulong uVar3;
  
  if (DAT_00184a54 < 2) {
    *(undefined1 *)param_2 = 0;
    return;
  }
  if (((0 < DAT_00184a98) || (0 < DAT_00184aa0)) || (0 < DAT_00184a9c)) {
    *param_2 = 0x20;
    if ((0 < DAT_00184a98) && (*(long *)(param_1 + 0x158) != 0)) {
      uVar3 = 0;
      do {
        sVar2 = strlen("SCHILY.xattr.");
        cVar1 = FUN_00131730(sVar2 + *(long *)(*(long *)(param_1 + 0x160) + uVar3 * 0x18),0);
        if (cVar1 == '\0') {
          *(undefined1 *)param_2 = 0x2a;
          if (DAT_00184aa0 < 1) goto joined_r0x0013210a;
          goto LAB_001320c0;
        }
        uVar3 = uVar3 + 1;
      } while (uVar3 < *(ulong *)(param_1 + 0x158));
    }
    if (0 < DAT_00184aa0) {
LAB_001320c0:
      if (*(long *)(param_1 + 0x30) != 0) {
        *(undefined1 *)param_2 = 0x2e;
      }
    }
  }
joined_r0x0013210a:
  if ((0 < DAT_00184a9c) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0))))
  {
    *(undefined1 *)param_2 = 0x2b;
  }
  return;
}




// Function: xattrs_print @ 0x32150

void xattrs_print(long param_1)

{
  char cVar1;
  size_t sVar2;
  long lVar3;
  long lVar4;
  ulong uVar5;
  
  if (DAT_00184a54 < 3) {
    return;
  }
  if ((0 < DAT_00184aa0) && (*(long *)(param_1 + 0x30) != 0)) {
    __fprintf_chk(DAT_00183f28,1,"  s: %s\n");
  }
  if ((0 < DAT_00184a9c) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0))))
  {
    __fprintf_chk(DAT_00183f28,1,"  a: ");
    FUN_00131760(&DAT_00167708,*(undefined8 *)(param_1 + 0x38),*(undefined8 *)(param_1 + 0x40));
    lVar4 = *(long *)(param_1 + 0x50);
    if ((*(long *)(param_1 + 0x40) != 0) && (lVar4 != 0)) {
      __fprintf_chk(DAT_00183f28,1,&DAT_0016079c);
      lVar4 = *(long *)(param_1 + 0x50);
    }
    FUN_00131760("default:",*(undefined8 *)(param_1 + 0x48),lVar4);
    __fprintf_chk(DAT_00183f28,1,&DAT_00167707);
  }
  if ((0 < DAT_00184a98) && (*(long *)(param_1 + 0x158) != 0)) {
    uVar5 = 0;
    do {
      while( true ) {
        sVar2 = strlen("SCHILY.xattr.");
        lVar4 = uVar5 * 0x18;
        lVar3 = sVar2 + *(long *)(*(long *)(param_1 + 0x160) + uVar5 * 0x18);
        cVar1 = FUN_00131730(lVar3,0);
        if (cVar1 != '\0') break;
        uVar5 = uVar5 + 1;
        __fprintf_chk(DAT_00183f28,1,"  x: %lu %s\n",
                      *(undefined8 *)(*(long *)(param_1 + 0x160) + 0x10 + lVar4),lVar3);
        if (*(ulong *)(param_1 + 0x158) <= uVar5) {
          return;
        }
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < *(ulong *)(param_1 + 0x158));
  }
  return;
}



