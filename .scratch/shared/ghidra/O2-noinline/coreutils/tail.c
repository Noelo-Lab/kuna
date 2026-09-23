// Function: main @ 0x29a0

byte main(int param_1,undefined8 *param_2)

{
  undefined **ppuVar1;
  undefined *puVar2;
  bool bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  int iVar7;
  int iVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  long lVar11;
  undefined8 uVar12;
  int *piVar13;
  undefined8 uVar14;
  undefined **ppuVar15;
  undefined **ppuVar16;
  undefined8 *puVar17;
  long in_FS_OFFSET;
  long local_100;
  int local_f4;
  long local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_f4 = 0;
  local_f0 = 10;
  local_e8 = DAT_0010f4f0;
  FUN_001091a0(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_0010d460(FUN_00107b00);
  DAT_0011510e = '\0';
  DAT_00115119 = 1;
  DAT_00115115 = 0;
  DAT_00115116 = '\0';
  DAT_00115118 = '\0';
  DAT_00115114 = 10;
  bVar4 = FUN_00103bc0(param_1,param_2,&local_f0);
  param_1 = param_1 - (uint)bVar4;
  FUN_00107080(param_1,param_2 + bVar4,&local_f0,&local_f4,&local_e8);
  if ((DAT_00115116 != '\0') && (local_f0 != 0)) {
    local_f0 = local_f0 + -1;
  }
  ppuVar15 = &PTR_DAT_00115028;
  local_100 = 1;
  if (optind < param_1) {
    ppuVar15 = (undefined **)(param_2 + bVar4 + optind);
    local_100 = (long)(param_1 - optind);
  }
  bVar3 = false;
  ppuVar1 = ppuVar15 + local_100;
  ppuVar16 = ppuVar15;
  do {
    iVar7 = strcmp(*ppuVar16,"-");
    if (iVar7 == 0) {
      bVar3 = true;
    }
    ppuVar16 = ppuVar16 + 1;
  } while (ppuVar1 != ppuVar16);
  if (bVar3) {
    if (DAT_00115020 == 1) {
      uVar12 = FUN_0010b410(4,&DAT_0010ef7f);
      uVar14 = dcgettext(0,"cannot follow %s by name",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar14,uVar12);
    }
    if (DAT_00115118 != '\0') {
      if (((((DAT_00115110 != 0) || (DAT_00115020 != 2)) || (local_100 != 1)) ||
          ((iVar7 = fstat(0,&local_d8), iVar7 != 0 || ((local_d8.st_mode & 0xf000) == 0x8000)))) &&
         (iVar7 = isatty(0), iVar7 != 0)) {
        uVar12 = dcgettext(0,"warning: following standard input indefinitely is ineffective",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12);
      }
      goto LAB_00102b55;
    }
    if (local_f0 == 0) goto LAB_00102c53;
  }
  else {
LAB_00102b55:
    if ((local_f0 == 0) && (DAT_00115118 == '\0')) {
LAB_00102c53:
      bVar4 = 0;
      if (DAT_00115116 == '\0') goto LAB_00102c04;
    }
  }
  puVar9 = (undefined8 *)FUN_0010c0c0(local_100,0x60);
  puVar10 = puVar9;
  do {
    puVar2 = *ppuVar15;
    ppuVar15 = ppuVar15 + 1;
    *puVar10 = puVar2;
    puVar10 = puVar10 + 0xc;
  } while (ppuVar15 != ppuVar1);
  if ((local_f4 == 1) || ((local_100 != 1 && (local_f4 == 0)))) {
    DAT_00115115 = 1;
  }
  bVar4 = 1;
  puVar10 = puVar9;
  do {
    puVar17 = puVar10 + 0xc;
    bVar5 = FUN_00106ab0(puVar10,local_f0);
    bVar4 = bVar4 & bVar5;
    puVar10 = puVar17;
  } while (puVar9 + local_100 * 0xc != puVar17);
  if ((DAT_00115118 != '\0') && (lVar11 = FUN_00103a50(puVar9,local_100), lVar11 != 0)) {
    iVar7 = fstat(1,&local_d8);
    if (iVar7 < 0) {
      uVar12 = dcgettext(0,"standard output",5);
      piVar13 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(1,*piVar13,uVar12);
    }
    DAT_00115117 = (local_d8.st_mode & 0xf000) == 0x1000;
    if ((((DAT_0011510c == '\0') && (cVar6 = FUN_00103ae0(puVar9,local_100), cVar6 == '\0')) &&
        (cVar6 = FUN_001038d0(puVar9,local_100), cVar6 == '\0')) &&
       (((cVar6 = FUN_00103900(puVar9), cVar6 != '\0' &&
         (cVar6 = FUN_00104b40(puVar9), cVar6 == '\0')) &&
        ((cVar6 = FUN_00103930(puVar9,local_100), cVar6 == '\0' &&
         ((bVar4 != 0 || (DAT_00115020 != 2)))))))) {
      iVar7 = inotify_init();
      piVar13 = __errno_location();
      if (-1 < iVar7) {
        iVar8 = fflush_unlocked(stdout);
        if (iVar8 != 0) {
          uVar12 = dcgettext(0,"write error",5);
                    /* WARNING: Subroutine does not return */
          error(1,*piVar13,uVar12);
        }
        FUN_00105880(local_e8,iVar7,puVar9,local_100,&local_e0);
        FUN_00108a10(local_e0);
        close(iVar7);
        *piVar13 = 0;
      }
      uVar12 = dcgettext(0,"inotify cannot be used, reverting to polling",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar13,uVar12);
    }
    DAT_0011510c = '\x01';
    FUN_00106220(local_e8,puVar9,local_100);
  }
  if ((DAT_0011510e != '\0') && (iVar7 = close(0), iVar7 < 0)) {
    piVar13 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(1,*piVar13,&DAT_0010ef7f);
  }
  bVar4 = bVar4 ^ 1;
LAB_00102c04:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar4;
}




// Function: record_open_fd @ 0x3830

void record_open_fd(long param_1,undefined4 param_2,undefined8 param_3,undefined8 *param_4,
                 undefined4 param_5)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  
  *(undefined4 *)(param_1 + 0x38) = param_2;
  *(undefined8 *)(param_1 + 8) = param_3;
  auVar3 = FUN_0010b7b0(param_4);
  uVar2 = *param_4;
  *(undefined1 (*) [16])(param_1 + 0x10) = auVar3;
  *(undefined8 *)(param_1 + 0x20) = uVar2;
  *(undefined8 *)(param_1 + 0x28) = param_4[1];
  uVar1 = *(undefined4 *)(param_4 + 3);
  *(undefined4 *)(param_1 + 0x40) = param_5;
  *(undefined4 *)(param_1 + 0x30) = uVar1;
  *(undefined8 *)(param_1 + 0x58) = 0;
  *(undefined1 *)(param_1 + 0x34) = 0;
  return;
}




// Function: any_live_files @ 0x3880

byte any_live_files(long param_1,long param_2)

{
  byte bVar1;
  byte *pbVar2;
  byte *pbVar3;
  
  if ((DAT_0011511a == 0) || (bVar1 = DAT_0011511a, DAT_00115020 != 1)) {
    pbVar3 = (byte *)(param_1 + 0x34);
    pbVar2 = pbVar3 + param_2 * 0x60;
    while (*(int *)(pbVar3 + 4) < 0) {
      bVar1 = (*pbVar3 ^ 1) & DAT_0011511a;
      if (bVar1 != 0) {
        return bVar1;
      }
      pbVar3 = pbVar3 + 0x60;
      if (pbVar3 == pbVar2) {
        return 0;
      }
    }
    bVar1 = 1;
  }
  return bVar1;
}




// Function: any_remote_file @ 0x38d0

char any_remote_file(long param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  
  pcVar2 = (char *)(param_1 + 0x35);
  pcVar1 = pcVar2 + param_2 * 0x60;
  while ((*(int *)(pcVar2 + 3) < 0 || (*pcVar2 == '\0'))) {
    pcVar2 = pcVar2 + 0x60;
    if (pcVar2 == pcVar1) {
      return '\0';
    }
  }
  return *pcVar2;
}




// Function: any_non_remote_file @ 0x3900

undefined8 any_non_remote_file(long param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  
  pcVar2 = (char *)(param_1 + 0x35);
  pcVar1 = pcVar2 + param_2 * 0x60;
  while ((*(int *)(pcVar2 + 3) < 0 || (*pcVar2 != '\0'))) {
    pcVar2 = pcVar2 + 0x60;
    if (pcVar2 == pcVar1) {
      return 0;
    }
  }
  return 1;
}




// Function: any_non_regular_fifo @ 0x3930

undefined4 any_non_regular_fifo(long param_1,long param_2)

{
  uint *puVar1;
  uint uVar2;
  uint *puVar3;
  
  puVar3 = (uint *)(param_1 + 0x30);
  puVar1 = puVar3 + param_2 * 0x18;
  while (((int)puVar3[2] < 0 || (uVar2 = *puVar3 & 0xf000, uVar2 == 0x1000 || uVar2 == 0x8000))) {
    puVar3 = puVar3 + 0x18;
    if (puVar1 == puVar3) {
      return 0;
    }
  }
  return CONCAT31((int3)(puVar3[2] >> 8),uVar2 != 0x1000 && uVar2 != 0x8000);
}




// Function: wd_hasher @ 0x3980

ulong wd_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 0x44) % param_2;
}




// Function: wd_comparator @ 0x39a0

undefined4 wd_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x44) >> 8),
                  *(int *)(param_1 + 0x44) == *(int *)(param_2 + 0x44));
}




// Function: write_header @ 0x3a10

void write_header(undefined8 param_1)

{
  char *pcVar1;
  
  pcVar1 = "";
  if (DAT_00115010 == '\0') {
    pcVar1 = "\n";
  }
  __printf_chk(1,"%s==> %s <==\n",pcVar1,param_1);
  DAT_00115010 = 0;
  return;
}




// Function: ignore_fifo_and_pipe @ 0x3a50

long ignore_fifo_and_pipe(undefined8 *param_1,long param_2)

{
  int iVar1;
  undefined8 *puVar2;
  long lVar3;
  
  lVar3 = 0;
  puVar2 = param_1 + param_2 * 0xc;
  do {
    iVar1 = strcmp((char *)*param_1,"-");
    if ((((iVar1 == 0) && (*(char *)((long)param_1 + 0x34) == '\0')) && (-1 < *(int *)(param_1 + 7))
        ) && ((*(uint *)(param_1 + 6) & 0xf000) == 0x1000)) {
      *(undefined4 *)(param_1 + 7) = 0xffffffff;
      *(undefined1 *)((long)param_1 + 0x34) = 1;
    }
    else {
      lVar3 = lVar3 + 1;
    }
    param_1 = param_1 + 0xc;
  } while (puVar2 != param_1);
  return lVar3;
}




// Function: tailable_stdin @ 0x3ae0

undefined8 tailable_stdin(undefined8 *param_1,long param_2)

{
  int iVar1;
  undefined8 *puVar2;
  
  puVar2 = param_1 + param_2 * 0xc;
  while ((*(char *)((long)param_1 + 0x34) != '\0' ||
         (iVar1 = strcmp((char *)*param_1,"-"), iVar1 != 0))) {
    param_1 = param_1 + 0xc;
    if (param_1 == puVar2) {
      return 0;
    }
  }
  return 1;
}




// Function: close_fd @ 0x3b30

void close_fd(int param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (param_1 + 1U < 2) {
    return;
  }
  iVar1 = close(param_1);
  if (iVar1 == 0) {
    return;
  }
  uVar2 = FUN_0010b410(4,param_2);
  uVar3 = dcgettext(0,"closing %s (fd=%d)",5);
  piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar4,uVar3,uVar2,param_1);
}




// Function: parse_obsolete_option @ 0x3bc0

undefined8 parse_obsolete_option(int param_1,long param_2,undefined8 *param_3)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  char *pcVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined8 uVar7;
  char *pcVar8;
  undefined1 uVar9;
  char *pcVar10;
  undefined1 uVar11;
  
  if (param_1 != 2) {
    if (param_1 == 3) {
      pcVar10 = *(char **)(param_2 + 0x10);
      if ((*pcVar10 != '-') || (pcVar10[1] == '\0')) goto LAB_00103bf5;
    }
    else {
      if (param_1 != 4) {
        return 0;
      }
      pcVar10 = *(char **)(param_2 + 0x10);
    }
    iVar2 = strcmp(pcVar10,"--");
    if (iVar2 != 0) {
      return 0;
    }
  }
LAB_00103bf5:
  iVar2 = FUN_00109110();
  pcVar8 = *(char **)(param_2 + 8);
  pcVar10 = pcVar8 + 1;
  if (*pcVar8 == '+') {
    uVar9 = 1;
    if (iVar2 - 0x30db0U < 0x2b9) {
      return 0;
    }
  }
  else {
    if (*pcVar8 != '-') {
      return 0;
    }
    if ((0x30daf < iVar2) && (pcVar8[(ulong)(pcVar8[1] == 'c') + 1] == '\0')) {
      return 0;
    }
    uVar9 = 0;
  }
  cVar1 = pcVar8[1];
  pcVar8 = pcVar10;
  while ((int)cVar1 - 0x30U < 10) {
    pcVar4 = pcVar8 + 1;
    pcVar8 = pcVar8 + 1;
    cVar1 = *pcVar4;
  }
  if (cVar1 == 'c') {
    uVar11 = 0;
    uVar7 = 10;
  }
  else if (cVar1 == 'l') {
    uVar11 = 1;
    uVar7 = 10;
  }
  else {
    if (cVar1 != 'b') {
      uVar11 = 1;
      uVar7 = 10;
      pcVar4 = pcVar8;
      goto LAB_00103c8b;
    }
    uVar11 = 0;
    uVar7 = 0x1400;
  }
  pcVar4 = pcVar8 + 1;
LAB_00103c8b:
  cVar1 = *pcVar4;
  if (cVar1 == 'f') {
    pcVar4 = pcVar4 + 1;
  }
  if (*pcVar4 != '\0') {
    return 0;
  }
  if (pcVar10 == pcVar8) {
    *param_3 = uVar7;
  }
  else {
    uVar3 = FUN_0010c6c0(pcVar10,0,10,param_3,&DAT_0010ef81);
    if ((uVar3 & 0xfffffffd) != 0) {
      uVar7 = FUN_0010b6d0(*(undefined8 *)(param_2 + 8));
      uVar5 = dcgettext(0,"invalid number",5);
      piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(1,*piVar6,"%s: %s",uVar5,uVar7);
    }
  }
  DAT_00115116 = uVar9;
  DAT_00115118 = cVar1 == 'f';
  DAT_00115119 = uVar11;
  return 1;
}




// Function: xwrite_stdout @ 0x3da0

void xwrite_stdout(void *param_1,size_t param_2)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (param_2 == 0) {
    return;
  }
  sVar1 = fwrite_unlocked(param_1,1,param_2,stdout);
  if (param_2 <= sVar1) {
    return;
  }
  clearerr_unlocked(stdout);
  uVar2 = FUN_0010b410(4,"standard output");
  uVar3 = dcgettext(0,"error writing %s",5);
  piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(1,*piVar4,uVar3,uVar2);
}




// Function: start_lines @ 0x3e30

undefined8 start_lines(undefined8 param_1,undefined4 param_2,long param_3,long *param_4)

{
  undefined1 *puVar1;
  long lVar2;
  void *pvVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined1 *__s;
  int __c;
  long in_FS_OFFSET;
  undefined1 local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = 0;
  if (param_3 != 0) {
    while( true ) {
      lVar2 = FUN_0010b6e0(param_2,local_2048,0x2000);
      if (lVar2 == 0) break;
      if (lVar2 == -1) {
        uVar4 = FUN_0010b410(4,param_1);
        uVar5 = dcgettext(0,"error reading %s",5);
        piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(0,*piVar6,uVar5,uVar4);
      }
      *param_4 = *param_4 + lVar2;
      __c = (int)DAT_00115114;
      puVar1 = local_2048 + lVar2;
      __s = local_2048;
      while( true ) {
        pvVar3 = memchr(__s,__c,(long)puVar1 - (long)__s);
        if (pvVar3 == (void *)0x0) break;
        __s = (undefined1 *)((long)pvVar3 + 1);
        param_3 = param_3 + -1;
        if (param_3 == 0) {
          uVar4 = 0;
          if (__s < puVar1) {
            FUN_00103da0(__s,(long)puVar1 - (long)__s);
            uVar4 = 0;
          }
          goto LAB_00103ee4;
        }
      }
    }
    uVar4 = 0xffffffff;
  }
LAB_00103ee4:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_remainder @ 0x3f80

long dump_remainder(char param_1,undefined8 param_2,undefined4 param_3,ulong param_4)

{
  long lVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  ulong uVar5;
  ulong uVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined1 local_2048 [8200];
  long local_40;
  
  lVar7 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar6 = param_4;
  while( true ) {
    uVar5 = 0x2000;
    if (uVar6 < 0x2001) {
      uVar5 = uVar6;
    }
    lVar1 = FUN_0010b6e0(param_3,local_2048,uVar5);
    if (lVar1 == -1) break;
    if (lVar1 == 0) goto LAB_0010403d;
    if (param_1 != '\0') {
      FUN_00103a10(param_2);
    }
    lVar7 = lVar7 + lVar1;
    FUN_00103da0(local_2048,lVar1);
    if ((param_4 != 0xffffffffffffffff) &&
       ((uVar6 = uVar6 - lVar1, uVar6 == 0 || (param_4 == 0xfffffffffffffffe)))) goto LAB_0010403d;
    param_1 = '\0';
  }
  piVar2 = __errno_location();
  if (*piVar2 != 0xb) {
    uVar3 = FUN_0010b410(4,param_2);
    uVar4 = dcgettext(0,"error reading %s",5);
                    /* WARNING: Subroutine does not return */
    error(1,*piVar2,uVar4,uVar3);
  }
LAB_0010403d:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: start_bytes @ 0x40c0

undefined8 start_bytes(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  ulong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  undefined1 auStack_2038 [8200];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    if (param_3 == 0) {
LAB_0010414f:
      uVar2 = 0;
LAB_00104151:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    uVar1 = FUN_0010b6e0(param_2,auStack_2038,0x2000);
    if (uVar1 == 0) {
      uVar2 = 0xffffffff;
      goto LAB_00104151;
    }
    if (uVar1 == 0xffffffffffffffff) {
      uVar2 = FUN_0010b410(4,param_1);
      uVar3 = dcgettext(0,"error reading %s",5);
      piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar4,uVar3,uVar2);
    }
    *param_4 = *param_4 + uVar1;
    if (param_3 < uVar1) {
      FUN_00103da0(auStack_2038 + param_3,uVar1 - param_3);
      goto LAB_0010414f;
    }
    param_3 = param_3 - uVar1;
  } while( true );
}




// Function: xlseek @ 0x41c0

void xlseek(int param_1,__off_t param_2,int param_3,undefined8 param_4)

{
  __off_t _Var1;
  undefined8 uVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined1 auStack_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  _Var1 = lseek(param_1,param_2,param_3);
  if (_Var1 < 0) {
    uVar2 = FUN_00109070(param_2,auStack_48);
    piVar3 = __errno_location();
    if (param_3 == 1) {
      uVar4 = FUN_0010b510(0,3,param_4);
      pcVar6 = "%s: cannot seek to relative offset %s";
    }
    else {
      uVar4 = FUN_0010b510(0,3,param_4);
      pcVar6 = "%s: cannot seek to offset %s";
    }
    uVar5 = dcgettext(0,pcVar6,5);
                    /* WARNING: Subroutine does not return */
    error(0,*piVar3,uVar5,uVar4,uVar2);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_lines @ 0x4290

undefined8
file_lines(undefined8 param_1,undefined4 param_2,long param_3,long param_4,long param_5,
            long *param_6)

{
  char cVar1;
  size_t __n;
  void *pvVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  int __c;
  long lVar6;
  size_t sVar7;
  long in_FS_OFFSET;
  bool bVar8;
  undefined8 local_2050;
  undefined1 local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_2050 = param_5;
  if (param_3 != 0) {
    lVar6 = (param_5 - param_4) % 0x2000;
    if (lVar6 == 0) {
      lVar6 = 0x2000;
    }
    param_5 = param_5 - lVar6;
    FUN_001041c0(param_2,param_5,0,param_1);
    __n = FUN_0010b6e0(param_2,local_2048,lVar6);
    cVar1 = DAT_00115114;
    if (__n == 0xffffffffffffffff) {
LAB_00104487:
      uVar3 = FUN_0010b410(4,param_1);
      uVar4 = dcgettext(0,"error reading %s",5);
      piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar5,uVar4,uVar3);
    }
    __c = (int)DAT_00115114;
    *param_6 = param_5 + __n;
    sVar7 = __n;
    if ((__n != 0) && (local_2048[__n - 1] != cVar1)) {
      param_3 = param_3 + -1;
    }
    do {
      while ((__n == 0 || (pvVar2 = memrchr(local_2048,__c,__n), pvVar2 == (void *)0x0))) {
        if (param_5 == param_4) {
          FUN_001041c0(param_2,param_5,0,param_1);
          lVar6 = FUN_00103f80(0,param_1,param_2,local_2050);
          *param_6 = param_5 + lVar6;
          goto LAB_0010444e;
        }
        param_5 = param_5 + -0x2000;
        FUN_001041c0(param_2,param_5,0,param_1);
        __n = FUN_0010b6e0(param_2,local_2048,0x2000);
        if (__n == 0xffffffffffffffff) goto LAB_00104487;
        *param_6 = param_5 + __n;
        if (__n == 0) goto LAB_0010444e;
        __c = (int)DAT_00115114;
        sVar7 = __n;
      }
      __n = (long)pvVar2 - (long)local_2048;
      bVar8 = param_3 != 0;
      param_3 = param_3 + -1;
    } while (bVar8);
    FUN_00103da0((long)pvVar2 + 1,(sVar7 - 1) - __n);
    lVar6 = FUN_00103f80(0,param_1,param_2,(local_2050 - sVar7) - param_5);
    *param_6 = *param_6 + lVar6;
  }
LAB_0010444e:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 1;
}




// Function: pipe_bytes @ 0x4510

undefined8 pipe_bytes(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  ulong uVar1;
  void *pvVar2;
  void *pvVar3;
  size_t __n;
  ulong uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  void *__ptr;
  long lVar8;
  void *__ptr_00;
  void *pvVar9;
  ulong uVar10;
  
  pvVar2 = (void *)FUN_0010bfa0(0x2010);
  *(undefined8 *)((long)pvVar2 + 0x2000) = 0;
  *(undefined8 *)((long)pvVar2 + 0x2008) = 0;
  pvVar3 = (void *)FUN_0010bfa0(0x2010);
  __ptr = pvVar3;
  pvVar9 = pvVar2;
  uVar10 = 0;
  while (__ptr_00 = __ptr, __ptr = pvVar2, __n = FUN_0010b6e0(param_2,pvVar3,0x2000),
        __n - 1 < 0xfffffffffffffffe) {
    uVar10 = uVar10 + __n;
    *param_4 = *param_4 + __n;
    *(size_t *)((long)__ptr_00 + 0x2000) = __n;
    lVar8 = *(long *)((long)pvVar9 + 0x2000);
    *(undefined8 *)((long)__ptr_00 + 0x2008) = 0;
    pvVar2 = __ptr;
    if (lVar8 + __n < 0x2000) {
      memcpy((void *)(lVar8 + (long)pvVar9),pvVar3,__n);
      *(long *)((long)pvVar9 + 0x2000) =
           *(long *)((long)pvVar9 + 0x2000) + *(long *)((long)__ptr_00 + 0x2000);
      __ptr = __ptr_00;
    }
    else {
      *(void **)((long)pvVar9 + 0x2008) = __ptr_00;
      uVar4 = uVar10 - *(long *)((long)__ptr + 0x2000);
      pvVar9 = __ptr_00;
      if (param_3 < uVar4) {
        pvVar2 = *(void **)((long)__ptr + 0x2008);
        pvVar3 = __ptr;
        uVar10 = uVar4;
      }
      else {
        pvVar3 = (void *)FUN_0010bfa0(0x2010);
        __ptr = pvVar3;
      }
    }
  }
  free(__ptr_00);
  if (__n != 0xffffffffffffffff) {
    lVar8 = *(long *)((long)__ptr + 0x2000);
    uVar4 = uVar10 - lVar8;
    pvVar2 = __ptr;
    while (uVar1 = uVar4, param_3 < uVar1) {
      pvVar2 = *(void **)((long)pvVar2 + 0x2008);
      lVar8 = *(long *)((long)pvVar2 + 0x2000);
      uVar10 = uVar1;
      uVar4 = uVar1 - lVar8;
    }
    pvVar3 = pvVar2;
    if (param_3 < uVar10) {
      lVar8 = lVar8 + (param_3 - uVar10);
      pvVar3 = (void *)((long)pvVar2 + (uVar10 - param_3));
    }
    FUN_00103da0(pvVar3,lVar8);
    for (lVar8 = *(long *)((long)pvVar2 + 0x2008); lVar8 != 0; lVar8 = *(long *)(lVar8 + 0x2008)) {
      FUN_00103da0(lVar8,*(undefined8 *)(lVar8 + 0x2000));
    }
    do {
      pvVar2 = *(void **)((long)__ptr + 0x2008);
      free(__ptr);
      __ptr = pvVar2;
    } while (pvVar2 != (void *)0x0);
    return 1;
  }
  uVar5 = FUN_0010b410(4,param_1);
  uVar6 = dcgettext(0,"error reading %s",5);
  piVar7 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar7,uVar6,uVar5);
}




// Function: pipe_lines @ 0x4760

ulong pipe_lines(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  ulong uVar1;
  char cVar2;
  void *pvVar3;
  void *pvVar4;
  long lVar5;
  ulong uVar6;
  void *pvVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  int *piVar10;
  undefined8 extraout_RDX;
  int __c;
  void *__ptr;
  ulong uVar11;
  long lVar12;
  
  pvVar3 = (void *)FUN_0010bfa0(0x2018);
  *(undefined8 *)((long)pvVar3 + 0x2008) = 0;
  *(undefined8 *)((long)pvVar3 + 0x2000) = 0;
  *(undefined8 *)((long)pvVar3 + 0x2010) = 0;
  pvVar4 = (void *)FUN_0010bfa0(0x2018);
  pvVar7 = pvVar3;
  uVar11 = 0;
  while (__ptr = pvVar3, lVar5 = FUN_0010b6e0(param_2,pvVar4,0x2000),
        lVar5 - 1U < 0xfffffffffffffffe) {
    *(long *)((long)pvVar4 + 0x2000) = lVar5;
    __c = (int)DAT_00115114;
    *param_4 = *param_4 + lVar5;
    *(undefined8 *)((long)pvVar4 + 0x2008) = 0;
    *(undefined8 *)((long)pvVar4 + 0x2010) = 0;
    pvVar3 = pvVar4;
    while (pvVar3 = memchr(pvVar3,__c,(long)pvVar4 + (lVar5 - (long)pvVar3)), pvVar3 != (void *)0x0)
    {
      *(long *)((long)pvVar4 + 0x2008) = *(long *)((long)pvVar4 + 0x2008) + 1;
      pvVar3 = (void *)((long)pvVar3 + 1);
    }
    uVar11 = uVar11 + *(long *)((long)pvVar4 + 0x2008);
    pvVar3 = __ptr;
    if (*(size_t *)((long)pvVar4 + 0x2000) + *(long *)((long)pvVar7 + 0x2000) < 0x2000) {
      memcpy((void *)(*(long *)((long)pvVar7 + 0x2000) + (long)pvVar7),pvVar4,
             *(size_t *)((long)pvVar4 + 0x2000));
      *(long *)((long)pvVar7 + 0x2000) =
           *(long *)((long)pvVar7 + 0x2000) + *(long *)((long)pvVar4 + 0x2000);
      *(long *)((long)pvVar7 + 0x2008) =
           *(long *)((long)pvVar7 + 0x2008) + *(long *)((long)pvVar4 + 0x2008);
    }
    else {
      *(void **)((long)pvVar7 + 0x2010) = pvVar4;
      uVar6 = uVar11 - *(long *)((long)__ptr + 0x2008);
      pvVar7 = pvVar4;
      if (param_3 < uVar6) {
        pvVar3 = *(void **)((long)__ptr + 0x2010);
        uVar11 = uVar6;
        pvVar4 = __ptr;
      }
      else {
        pvVar4 = (void *)FUN_0010bfa0(0x2018);
      }
    }
  }
  free(pvVar4);
  cVar2 = DAT_00115114;
  if (lVar5 != -1) {
    lVar5 = *(long *)((long)pvVar7 + 0x2000);
    uVar6 = CONCAT71((int7)((ulong)extraout_RDX >> 8),lVar5 == 0 || param_3 == 0) & 0xffffffff;
    if (lVar5 == 0 || param_3 == 0) {
      if (__ptr == (void *)0x0) {
        return uVar6;
      }
    }
    else {
      if (*(char *)((long)pvVar7 + lVar5 + -1) != DAT_00115114) {
        *(long *)((long)pvVar7 + 0x2008) = *(long *)((long)pvVar7 + 0x2008) + 1;
        uVar11 = uVar11 + 1;
      }
      uVar6 = uVar11 - *(long *)((long)__ptr + 0x2008);
      pvVar3 = __ptr;
      while (uVar1 = uVar6, param_3 < uVar1) {
        pvVar3 = *(void **)((long)pvVar3 + 0x2010);
        uVar11 = uVar1;
        uVar6 = uVar1 - *(long *)((long)pvVar3 + 0x2008);
      }
      lVar5 = *(long *)((long)pvVar3 + 0x2000);
      pvVar7 = pvVar3;
      if (param_3 < uVar11) {
        lVar12 = uVar11 - param_3;
        do {
          pvVar7 = rawmemchr(pvVar7,(int)cVar2);
          pvVar7 = (void *)((long)pvVar7 + 1);
          lVar12 = lVar12 + -1;
        } while (lVar12 != 0);
      }
      FUN_00103da0(pvVar7,(long)pvVar3 + (lVar5 - (long)pvVar7));
      for (lVar5 = *(long *)((long)pvVar3 + 0x2010); lVar5 != 0; lVar5 = *(long *)(lVar5 + 0x2010))
      {
        FUN_00103da0(lVar5,*(undefined8 *)(lVar5 + 0x2000));
      }
      uVar6 = 1;
    }
    do {
      pvVar3 = *(void **)((long)__ptr + 0x2010);
      free(__ptr);
      __ptr = pvVar3;
    } while (pvVar3 != (void *)0x0);
    return uVar6;
  }
  uVar8 = FUN_0010b410(4,param_1);
  uVar9 = dcgettext(0,"error reading %s",5);
  piVar10 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar10,uVar9,uVar8);
}




// Function: fremote @ 0x4a40

undefined8 fremote(int param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  statfs local_a8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fstatfs(param_1,&local_a8);
  if (iVar1 == 0) {
    iVar1 = FUN_00102fb0(local_a8.f_type);
    if (iVar1 < 1) {
      if (iVar1 < -1) {
LAB_00104ac2:
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected return value from is_local_fs_type\"","src/tail.c",0x3cc,
                      "fremote");
      }
      uVar4 = 1;
    }
    else {
      if (iVar1 != 1) goto LAB_00104ac2;
      uVar4 = 0;
    }
  }
  else {
    piVar2 = __errno_location();
    uVar4 = 1;
    if (*piVar2 != 0x26) {
      uVar4 = FUN_0010b410(4,param_2);
      uVar3 = dcgettext(0,"cannot determine location of %s. reverting to polling",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar2,uVar3,uVar4);
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: any_symlinks @ 0x4b40

undefined8 any_symlinks(undefined8 *param_1,long param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  long in_FS_OFFSET;
  stat sStack_b8;
  long local_20;
  
  puVar3 = param_1 + param_2 * 0xc;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  while( true ) {
    iVar1 = lstat((char *)*param_1,&sStack_b8);
    if ((iVar1 == 0) && ((sStack_b8.st_mode & 0xf000) == 0xa000)) break;
    param_1 = param_1 + 0xc;
    if (param_1 == puVar3) {
      uVar2 = 0;
LAB_00104b9a:
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  uVar2 = 1;
  goto LAB_00104b9a;
}




// Function: die_pipe @ 0x4bd0

void die_pipe(void)

{
  raise(0xd);
                    /* WARNING: Subroutine does not return */
  exit(1);
}




// Function: check_output_alive @ 0x4bf0

void check_output_alive(void)

{
  int iVar1;
  long in_FS_OFFSET;
  pollfd local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00115117 != '\0') {
    local_18.fd = 1;
    local_18.events = 0;
    local_18.revents = 0;
    iVar1 = poll(&local_18,1,0);
    if ((-1 < iVar1) && (((ulong)local_18 & 0x18000000000000) != 0)) {
      FUN_00104bd0();
      goto LAB_00104c4f;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00104c4f:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_fspec @ 0x5090

void check_fspec(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  int *piVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  bool bVar6;
  undefined1 auVar7 [16];
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(int *)(param_1 + 7) != -1) {
    uVar2 = FUN_00105050(*param_1);
    iVar1 = fstat(*(int *)(param_1 + 7),&sStack_c8);
    if (iVar1 == 0) {
      if ((*(uint *)(param_1 + 6) & 0xf000) == 0x8000) {
        if (sStack_c8.st_size < (long)param_1[1]) {
          uVar2 = FUN_0010b510(0,3,uVar2);
          uVar5 = dcgettext(0,"%s: file truncated",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar5,uVar2);
        }
        if (sStack_c8.st_size == param_1[1]) {
          auVar7 = FUN_0010b7b0(&sStack_c8);
          iVar1 = FUN_0010b800(param_1[2],param_1[3],auVar7._0_8_,auVar7._8_8_);
          if (iVar1 == 0) goto LAB_00105120;
        }
      }
      bVar6 = false;
      if (DAT_00115115 != '\0') {
        bVar6 = (undefined8 *)*param_2 != param_1;
      }
      lVar3 = FUN_00103f80(bVar6,uVar2,*(undefined4 *)(param_1 + 7),0xffffffffffffffff);
      param_1[1] = param_1[1] + lVar3;
      if (lVar3 != 0) {
        *param_2 = param_1;
        iVar1 = fflush_unlocked(stdout);
        if (iVar1 != 0) {
          uVar2 = dcgettext(0,"write error",5);
          piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
          error(1,*piVar4,uVar2);
        }
      }
    }
    else {
      piVar4 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar4;
      FUN_00103b30(*(undefined4 *)(param_1 + 7),uVar2);
      *(undefined4 *)(param_1 + 7) = 0xffffffff;
    }
  }
LAB_00105120:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: recheck @ 0x5270

void recheck(undefined8 *param_1,uint param_2)

{
  uint uVar1;
  uint uVar2;
  char cVar3;
  char cVar4;
  char cVar5;
  char cVar6;
  int iVar7;
  int iVar8;
  uint *puVar9;
  undefined8 uVar10;
  int *piVar11;
  undefined8 uVar12;
  char *pcVar13;
  long in_FS_OFFSET;
  stat local_d8;
  long local_40;
  
  pcVar13 = (char *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar7 = strcmp(pcVar13,"-");
  uVar1 = *(uint *)((long)param_1 + 0x3c);
  cVar6 = *(char *)((long)param_1 + 0x36);
  if (iVar7 == 0) {
    iVar7 = 0;
    uVar2 = uVar1;
  }
  else {
    iVar7 = FUN_00107ea0(pcVar13,((param_2 ^ 1) & 0xff) << 0xb);
    uVar2 = *(uint *)((long)param_1 + 0x3c);
  }
  cVar5 = FUN_00105260(*(undefined4 *)(param_1 + 7),uVar2);
  cVar4 = DAT_0011511a;
  cVar3 = DAT_0011510c;
  if (cVar5 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck");
  }
  if (DAT_0011511a == '\0') {
    *(undefined1 *)((long)param_1 + 0x36) = 1;
    if (((cVar3 == '\0') && (iVar8 = lstat((char *)*param_1,&local_d8), iVar8 == 0)) &&
       ((local_d8.st_mode & 0xf000) == 0xa000)) {
LAB_00105750:
      *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
      *(undefined1 *)((long)param_1 + 0x34) = 1;
      uVar10 = FUN_00105050(*param_1);
      uVar10 = FUN_0010b410(4,uVar10);
      uVar12 = dcgettext(0,"%s has been replaced with an untailable symbolic link",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar12,uVar10);
    }
    if (iVar7 != -1) goto LAB_001053ca;
LAB_00105321:
    puVar9 = (uint *)__errno_location();
    pcVar13 = (char *)*param_1;
    uVar2 = *puVar9;
    *(uint *)((long)param_1 + 0x3c) = uVar2;
    if (*(char *)((long)param_1 + 0x36) == '\0') {
LAB_001054d9:
      if (cVar6 != '\0') {
        uVar10 = FUN_00105050(pcVar13);
        uVar10 = FUN_0010b410(4,uVar10);
        uVar12 = dcgettext(0,"%s has become inaccessible",5);
                    /* WARNING: Subroutine does not return */
        error(0,*(undefined4 *)((long)param_1 + 0x3c),uVar12,uVar10);
      }
    }
    else if (uVar2 != uVar1) {
      uVar10 = FUN_00105050(pcVar13);
      uVar10 = FUN_0010b510(0,3,uVar10);
                    /* WARNING: Subroutine does not return */
      error(0,*puVar9,&DAT_0010f553,uVar10);
    }
  }
  else {
    if (iVar7 == -1) {
      *(undefined1 *)((long)param_1 + 0x36) = 0;
      pcVar13 = (char *)*param_1;
      if (cVar3 == '\0') {
        iVar8 = lstat(pcVar13,&local_d8);
        if ((iVar8 == 0) && ((local_d8.st_mode & 0xf000) == 0xa000)) goto LAB_00105750;
        goto LAB_00105321;
      }
      piVar11 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar11;
      goto LAB_001054d9;
    }
    *(undefined1 *)((long)param_1 + 0x36) = 1;
    if (((cVar3 == '\0') && (iVar8 = lstat((char *)*param_1,&local_d8), iVar8 == 0)) &&
       ((local_d8.st_mode & 0xf000) == 0xa000)) goto LAB_00105750;
LAB_001053ca:
    iVar8 = fstat(iVar7,&local_d8);
    if (iVar8 < 0) goto LAB_00105321;
    if ((((local_d8.st_mode & 0xf000) - 0x1000 & 0xffffe000) == 0) ||
       ((local_d8.st_mode & 0xb000) == 0x8000)) {
      uVar10 = FUN_00105050(*param_1);
      cVar6 = FUN_00104a40(iVar7,uVar10);
      *(char *)((long)param_1 + 0x35) = cVar6;
      if ((cVar6 != '\0') && (DAT_0011510c == '\0')) {
        *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
        uVar10 = FUN_00105050(*param_1);
        uVar10 = FUN_0010b410(4,uVar10);
        uVar12 = dcgettext(0,"%s has been replaced with an untailable remote file",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12,uVar10);
      }
      *(undefined4 *)((long)param_1 + 0x3c) = 0;
      if ((uVar1 & 0xfffffffd) != 0) {
        if (*(int *)(param_1 + 7) != -1) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck");
        }
        uVar10 = FUN_00105050(*param_1);
        uVar10 = FUN_0010b410(4,uVar10);
        pcVar13 = "%s has become accessible";
LAB_0010546c:
        uVar12 = dcgettext(0,pcVar13,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12,uVar10);
      }
      uVar10 = *param_1;
      if (*(int *)(param_1 + 7) == -1) {
        uVar10 = FUN_00105050(uVar10);
        uVar10 = FUN_0010b410(4,uVar10);
        pcVar13 = "%s has appeared;  following new file";
        goto LAB_0010546c;
      }
      if ((param_1[5] != local_d8.st_ino) || (param_1[4] != local_d8.st_dev)) {
        uVar10 = FUN_00105050(uVar10);
        uVar10 = FUN_0010b410(4,uVar10);
        uVar12 = dcgettext(0,"%s has been replaced;  following new file",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12,uVar10);
      }
      uVar10 = FUN_00105050(uVar10);
      FUN_00103b30(iVar7,uVar10);
      goto LAB_00105374;
    }
    *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
    *(undefined1 *)((long)param_1 + 0x36) = 0;
    if ((cVar4 == '\0') || (DAT_00115020 != 1)) {
      *(undefined1 *)((long)param_1 + 0x34) = 1;
      if ((cVar6 != '\0') || (uVar1 != 0xffffffff)) {
        pcVar13 = (char *)dcgettext(0,"; giving up on this name",5);
        goto LAB_001056db;
      }
    }
    else {
      *(undefined1 *)((long)param_1 + 0x34) = 0;
      if (cVar6 != '\0') {
        pcVar13 = "";
LAB_001056db:
        uVar10 = FUN_00105050(*param_1);
        uVar10 = FUN_0010b410(4,uVar10);
        uVar12 = dcgettext(0,"%s has been replaced with an untailable file%s",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12,uVar10,pcVar13);
      }
      pcVar13 = "";
      if (uVar1 != 0xffffffff) goto LAB_001056db;
    }
  }
  uVar10 = FUN_00105050(*param_1);
  FUN_00103b30(iVar7,uVar10);
  uVar10 = FUN_00105050(*param_1);
  FUN_00103b30(*(undefined4 *)(param_1 + 7),uVar10);
  *(undefined4 *)(param_1 + 7) = 0xffffffff;
LAB_00105374:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: tail_forever_inotify @ 0x5880

void tail_forever_inotify(double param_1,int param_2,long *param_3,long param_4,long *param_5)

{
  char cVar1;
  uint uVar2;
  uint uVar3;
  bool bVar4;
  bool bVar5;
  uint32_t __mask;
  int iVar6;
  int iVar7;
  long lVar8;
  size_t sVar9;
  long lVar10;
  long lVar11;
  uint *puVar12;
  undefined8 uVar13;
  long *plVar14;
  int *piVar15;
  undefined8 uVar16;
  long *plVar17;
  char *__name;
  char *pcVar18;
  ulong uVar19;
  long in_FS_OFFSET;
  ulong local_1a8;
  ulong local_198;
  int *local_180;
  long local_168;
  int local_15c;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar8 = FUN_00108850(param_4,0,FUN_00103980,FUN_001039a0,0);
  if (lVar8 == 0) {
LAB_00105fea:
                    /* WARNING: Subroutine does not return */
    FUN_0010c360();
  }
  bVar5 = false;
  bVar4 = false;
  *param_5 = lVar8;
  local_1a8._0_4_ = DAT_00115020;
  __mask = 2;
  if (DAT_00115020 == 1) {
    __mask = 0xc06;
  }
  uVar19 = 0;
  plVar17 = param_3 + param_4 * 0xc;
  plVar14 = param_3;
LAB_00105938:
  do {
    if (*(char *)((long)plVar14 + 0x34) == '\0') {
      pcVar18 = (char *)*plVar14;
      sVar9 = strlen(pcVar18);
      *(undefined4 *)((long)plVar14 + 0x44) = 0xffffffff;
      if (uVar19 < sVar9) {
        uVar19 = sVar9;
      }
      if ((int)local_1a8 == 1) {
        lVar10 = FUN_00107bb0(pcVar18);
        cVar1 = pcVar18[lVar10];
        lVar11 = FUN_00107990(pcVar18);
        __name = ".";
        plVar14[10] = lVar11 - (long)pcVar18;
        pcVar18[lVar10] = '\0';
        if (lVar10 != 0) {
          __name = (char *)*plVar14;
        }
        iVar6 = inotify_add_watch(param_2,__name,0x784);
        *(int *)(plVar14 + 9) = iVar6;
        *(char *)(*plVar14 + lVar10) = cVar1;
        if ((int)plVar14[9] < 0) {
          piVar15 = __errno_location();
          if (*piVar15 == 0x1c) {
LAB_00105b10:
            pcVar18 = "inotify resources exhausted";
LAB_00105b1c:
            dcgettext(0,pcVar18,5);
            if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
              error(0,0);
            }
          }
          else {
            uVar13 = FUN_0010b410(4,*plVar14);
            uVar16 = dcgettext(0,"cannot watch parent directory of %s",5);
            if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
              error(0,*piVar15,uVar16,uVar13);
            }
          }
          goto LAB_001061db;
        }
        pcVar18 = (char *)*plVar14;
      }
      iVar6 = inotify_add_watch(param_2,pcVar18,__mask);
      *(int *)((long)plVar14 + 0x44) = iVar6;
      if (-1 < iVar6) {
        lVar10 = FUN_00108e70(lVar8);
        if (lVar10 == 0) goto LAB_00105fea;
        plVar14 = plVar14 + 0xc;
        bVar4 = true;
        local_1a8._0_4_ = DAT_00115020;
        if (plVar14 == plVar17) break;
        goto LAB_00105938;
      }
      if ((int)plVar14[7] != -1) {
        bVar5 = true;
      }
      puVar12 = (uint *)__errno_location();
      if ((*puVar12 & 0xffffffef) == 0xc) goto LAB_00105b10;
      if (*(uint *)((long)plVar14 + 0x3c) != *puVar12) {
        uVar13 = FUN_0010b410(4,*plVar14);
        uVar16 = dcgettext(0,"cannot watch %s",5);
                    /* WARNING: Subroutine does not return */
        error(0,*puVar12,uVar16,uVar13);
      }
    }
    plVar14 = plVar14 + 0xc;
  } while (plVar14 != plVar17);
  plVar14 = param_3;
  if ((int)local_1a8 == 2) {
    if (bVar5) {
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
LAB_001061db:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (!bVar4) {
                    /* WARNING: Subroutine does not return */
      exit(1);
    }
  }
  do {
    if (*(char *)((long)plVar14 + 0x34) == '\0') {
      if (DAT_00115020 == 1) {
        FUN_00105270(plVar14,0);
      }
      else if ((((int)plVar14[7] != -1) && (iVar6 = stat((char *)*plVar14,&local_d8), iVar6 == 0))
              && ((plVar14[4] !=
                   CONCAT26(local_d8.st_dev._6_2_,
                            CONCAT24(local_d8.st_dev._4_2_,(int)local_d8.st_dev)) ||
                  (plVar14[5] != local_d8.st_ino)))) {
        uVar13 = FUN_00105050(*plVar14);
        uVar13 = FUN_0010b410(4,uVar13);
        uVar16 = dcgettext(0,"%s was replaced",5);
        piVar15 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(0,*piVar15,uVar16,uVar13);
      }
      FUN_00105090(plVar14);
    }
    plVar14 = plVar14 + 0xc;
  } while (plVar14 != plVar17);
  local_168 = uVar19 + 0x11;
  bVar4 = false;
  local_180 = (int *)FUN_0010bfa0(local_168);
  local_198 = 0;
  local_1a8 = 0;
  local_15c = 3;
joined_r0x00105bf7:
  do {
    if (((DAT_00115020 == 1) && (DAT_0011511a == '\0')) && (lVar10 = FUN_001083f0(), lVar10 == 0)) {
      uVar13 = dcgettext(0,"no files remaining",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar13);
    }
    if (local_1a8 < local_198) {
      lVar10 = local_1a8 + 0x10;
      piVar15 = (int *)((long)local_180 + local_1a8);
LAB_00105c1f:
      uVar2 = piVar15[3];
      iVar6 = *piVar15;
      local_1a8 = lVar10 + (ulong)uVar2;
      uVar3 = piVar15[1];
      if ((uVar3 & 0x400) == 0) {
        if (uVar2 != 0) goto LAB_00105c55;
      }
      else {
        plVar14 = param_3 + 9;
        if (uVar2 != 0) {
LAB_00105c55:
          lVar10 = 0;
          plVar14 = param_3;
          do {
            if (((int)plVar14[9] == iVar6) &&
               (iVar7 = strcmp((char *)(piVar15 + 4),(char *)(plVar14[10] + *plVar14)), iVar7 == 0))
            {
              if (param_4 == lVar10) break;
              plVar14 = param_3 + lVar10 * 0xc;
              if ((uVar3 & 0x200) == 0) {
                iVar6 = inotify_add_watch(param_2,(char *)*plVar14,__mask);
                if (iVar6 < 0) {
                  puVar12 = (uint *)__errno_location();
                  if ((*puVar12 & 0xffffffef) != 0xc) {
                    uVar13 = FUN_0010b410(4,*plVar14);
                    uVar16 = dcgettext(0,"cannot watch %s",5);
                    /* WARNING: Subroutine does not return */
                    error(0,*puVar12,uVar16,uVar13);
                  }
                  goto LAB_00105b10;
                }
                iVar7 = *(int *)((long)plVar14 + 0x44);
                if (iVar7 == iVar6) {
                  if (-1 < iVar7) goto LAB_00105cc7;
                }
                else if (-1 < iVar7) {
                  inotify_rm_watch(param_2,iVar7);
                  FUN_00108ed0(lVar8);
                }
                *(int *)((long)plVar14 + 0x44) = iVar6;
                if (iVar6 == -1) break;
                plVar17 = (long *)FUN_00108ed0(lVar8,plVar14);
                if ((plVar17 != (long *)0x0) && (plVar14 != plVar17)) {
                  if (DAT_00115020 == 1) {
                    FUN_00105270(plVar17,0);
                  }
                  *(undefined4 *)((long)plVar17 + 0x44) = 0xffffffff;
                  uVar13 = FUN_00105050(*plVar17);
                  FUN_00103b30((int)plVar17[7],uVar13);
                }
                lVar10 = FUN_00108e70(lVar8);
                if (lVar10 == 0) goto LAB_00105fea;
              }
LAB_00105cc7:
              if (DAT_00115020 == 1) {
                FUN_00105270(plVar14);
              }
              uVar2 = piVar15[1];
              goto joined_r0x00105cde;
            }
            lVar10 = lVar10 + 1;
            plVar14 = plVar14 + 0xc;
          } while (param_4 != lVar10);
          goto joined_r0x00105bf7;
        }
        do {
          if ((int)*plVar14 == iVar6) {
            pcVar18 = "directory containing watched file was removed";
            goto LAB_00105b1c;
          }
          plVar14 = plVar14 + 0xc;
        } while (param_3 + 9 + param_4 * 0xc != plVar14);
      }
      plVar14 = (long *)FUN_001085f0(lVar8);
      if (plVar14 != (long *)0x0) {
        uVar2 = piVar15[1];
joined_r0x00105cde:
        if ((uVar2 & 0xe04) == 0) {
          FUN_00105090();
        }
        else {
          if ((uVar2 & 0x400) != 0) {
            inotify_rm_watch(param_2,*(int *)((long)plVar14 + 0x44));
            FUN_00108ed0(lVar8);
          }
          FUN_00105270();
        }
      }
      goto joined_r0x00105bf7;
    }
    do {
      if (DAT_00115110 == 0) {
LAB_00105e10:
        iVar6 = -1;
      }
      else {
        if (bVar4) {
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        iVar6 = kill(DAT_00115110,0);
        if ((iVar6 == 0) || (piVar15 = __errno_location(), *piVar15 == 1)) {
          iVar6 = 0;
          if (0.0 < param_1) {
            if (DAT_0010f4e0 <= param_1) goto LAB_00105e10;
            iVar6 = (int)(DAT_0010f4e8 * param_1);
            iVar6 = (uint)((double)iVar6 < DAT_0010f4e8 * param_1) + iVar6;
          }
        }
        else {
          iVar6 = 0;
          bVar4 = true;
        }
      }
      local_d8.st_dev._4_2_ = 1;
      local_d8.st_ino = 1;
      local_d8.st_dev._0_4_ = param_2;
      iVar6 = poll((pollfd *)&local_d8,(ulong)DAT_00115117 + 1,iVar6);
    } while (iVar6 == 0);
    if (iVar6 < 0) {
LAB_001061e5:
      uVar13 = dcgettext(0,"error waiting for inotify and output events",5);
      piVar15 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(1,*piVar15,uVar13);
    }
    if (local_d8.st_ino._6_2_ != 0) {
      FUN_00104bd0();
      goto LAB_001061e5;
    }
    local_198 = FUN_0010b6e0(param_2,local_180,local_168);
    if (local_198 != 0) {
      lVar10 = 0x10;
      piVar15 = local_180;
      if (local_198 != 0xffffffffffffffff) goto LAB_00105c1f;
      piVar15 = __errno_location();
      if (*piVar15 != 0x16) {
LAB_00105ed4:
        uVar13 = dcgettext(0,"error reading inotify event",5);
        piVar15 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(1,*piVar15,uVar13);
      }
    }
    if (local_15c == 0) goto LAB_00105ed4;
    local_15c = local_15c + -1;
    local_168 = local_168 * 2;
    local_180 = (int *)FUN_0010bff0(local_180);
    local_198 = 0;
    local_1a8 = 0;
  } while( true );
}




// Function: tail_forever @ 0x6220

void tail_forever(undefined8 param_1,undefined8 *param_2,long param_3)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  __mode_t _Var5;
  char cVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  undefined8 uVar11;
  int *piVar12;
  undefined8 uVar13;
  long lVar14;
  undefined8 *puVar15;
  long lVar16;
  uint uVar17;
  long in_FS_OFFSET;
  bool bVar18;
  undefined1 auVar19 [16];
  long local_100;
  bool local_e2;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00115110 == 0) {
    local_e2 = DAT_00115020 == 2 && param_3 == 1;
    uVar17 = 0;
    if (local_e2) {
      if (*(int *)(param_2 + 7) == -1) {
        local_e2 = false;
      }
      else {
        local_e2 = (*(uint *)(param_2 + 6) & 0xf000) != 0x8000;
        uVar17 = (uint)local_e2;
      }
    }
  }
  else {
    local_e2 = false;
    uVar17 = 0;
  }
  local_100 = param_3 + -1;
  bVar18 = false;
  do {
    bVar3 = bVar18;
    bVar4 = false;
    lVar16 = 0;
    puVar15 = param_2;
LAB_00106349:
    do {
      if (*(char *)((long)puVar15 + 0x34) == '\0') {
        iVar10 = *(int *)(puVar15 + 7);
        if (iVar10 < 0) {
          lVar16 = lVar16 + 1;
          FUN_00105270(puVar15,uVar17);
          puVar15 = puVar15 + 0xc;
          if (param_3 == lVar16) break;
          goto LAB_00106349;
        }
        uVar11 = FUN_00105050(*puVar15);
        uVar1 = *(uint *)(puVar15 + 6);
        uVar9 = uVar17;
        if (*(uint *)(puVar15 + 8) != uVar17) {
          uVar7 = FUN_00107d50(iVar10,3);
          if (uVar17 == 0) {
            if ((-1 < (int)uVar7) &&
               ((uVar7 == (uVar7 | 0x800) || (iVar8 = FUN_00107d50(iVar10,4), iVar8 != -1)))) {
LAB_00106309:
              *(uint *)(puVar15 + 8) = uVar17;
              goto LAB_00106310;
            }
          }
          else if (-1 < (int)uVar7) goto LAB_00106309;
          piVar12 = __errno_location();
          if (((*(uint *)(puVar15 + 6) & 0xf000) != 0x8000) || (*piVar12 != 1)) {
            uVar11 = FUN_0010b510(0,3,uVar11);
            uVar13 = dcgettext(0,"%s: cannot change nonblocking mode",5);
                    /* WARNING: Subroutine does not return */
            error(1,*piVar12,uVar13,uVar11);
          }
          uVar9 = *(uint *)(puVar15 + 8);
        }
LAB_00106310:
        lVar14 = -2;
        if (uVar9 == 0) {
          iVar8 = fstat(iVar10,&local_d8);
          _Var5 = local_d8.st_mode;
          if (iVar8 != 0) {
            *(undefined4 *)(puVar15 + 7) = 0xffffffff;
            piVar12 = __errno_location();
            *(int *)((long)puVar15 + 0x3c) = *piVar12;
            uVar11 = FUN_0010b510(0,3,uVar11);
                    /* WARNING: Subroutine does not return */
            error(0,*piVar12,&DAT_0010f553,uVar11);
          }
          if ((*(uint *)(puVar15 + 6) == local_d8.st_mode) &&
             (((local_d8.st_mode & 0xf000) != 0x8000 || (puVar15[1] == local_d8.st_size)))) {
            auVar19 = FUN_0010b7b0(&local_d8);
            iVar8 = FUN_0010b800(puVar15[2],puVar15[3],auVar19._0_8_,auVar19._8_8_);
            if (iVar8 == 0) {
              bVar18 = DAT_00115018 <= (ulong)puVar15[0xb];
              puVar15[0xb] = puVar15[0xb] + 1;
              if ((bVar18) && (DAT_00115020 == 1)) {
                FUN_00105270(puVar15,*(int *)(puVar15 + 8) != 0);
                puVar15[0xb] = 0;
              }
              goto LAB_0010633a;
            }
          }
          auVar19 = FUN_0010b7b0(&local_d8);
          puVar15[0xb] = 0;
          *(undefined1 (*) [16])(puVar15 + 2) = auVar19;
          *(__mode_t *)(puVar15 + 6) = _Var5;
          if (((uVar1 & 0xf000) == 0x8000) && (local_d8.st_size < (long)puVar15[1])) {
            uVar11 = FUN_0010b510(0,3,uVar11);
            uVar13 = dcgettext(0,"%s: file truncated",5);
                    /* WARNING: Subroutine does not return */
            error(0,0,uVar13,uVar11);
          }
          if ((local_100 != lVar16) && (DAT_00115115 != '\0')) {
            FUN_00103a10(uVar11);
          }
          local_100 = lVar16;
          if (*(int *)(puVar15 + 8) == 0) {
            lVar14 = -1;
            if (((uVar1 & 0xf000) == 0x8000) && (*(char *)((long)puVar15 + 0x35) != '\0')) {
              lVar14 = local_d8.st_size - puVar15[1];
            }
          }
          else {
            lVar14 = -2;
          }
        }
        lVar14 = FUN_00103f80(0,uVar11,iVar10,lVar14);
        puVar15[1] = puVar15[1] + lVar14;
        bVar4 = (bool)(bVar4 | lVar14 != 0);
      }
LAB_0010633a:
      lVar16 = lVar16 + 1;
      puVar15 = puVar15 + 0xc;
    } while (param_3 != lVar16);
    cVar6 = FUN_00103880(param_2);
    if (cVar6 == '\0') {
      uVar11 = dcgettext(0,"no files remaining",5);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar11);
      }
LAB_001067ce:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    bVar2 = (bool)(bVar4 ^ 1U | local_e2);
    bVar18 = bVar3;
    if (bVar2) {
      iVar10 = fflush_unlocked(stdout);
      if (iVar10 != 0) {
        uVar11 = dcgettext(0,"write error",5);
        piVar12 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(1,*piVar12,uVar11);
      }
      FUN_00104bf0();
      if (!bVar4) {
        if (bVar3) {
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
            return;
          }
          goto LAB_001067ce;
        }
        if ((((DAT_00115110 == 0) || (iVar10 = kill(DAT_00115110,0), iVar10 == 0)) ||
            (piVar12 = __errno_location(), bVar18 = bVar2, *piVar12 == 1)) &&
           (iVar10 = FUN_0010c4e0(param_1), bVar18 = bVar3, iVar10 != 0)) {
          uVar11 = dcgettext(0,"cannot read realtime clock",5);
          piVar12 = __errno_location();
                    /* WARNING: Subroutine does not return */
          error(1,*piVar12,uVar11);
        }
      }
    }
    else {
      FUN_00104bf0();
    }
  } while( true );
}




// Function: tail @ 0x6a90

void tail(void)

{
  undefined8 *in_RCX;
  bool bVar1;
  
  bVar1 = DAT_00115119 != '\0';
  *in_RCX = 0;
  if (bVar1) {
    FUN_00104c60();
    return;
  }
  FUN_00106820();
  return;
}




// Function: tail_file @ 0x6ab0

uint tail_file(undefined8 *param_1,undefined8 param_2)

{
  byte bVar1;
  undefined1 uVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  int __fd;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  char *pcVar9;
  long in_FS_OFFSET;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  pcVar9 = (char *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar3 = strcmp(pcVar9,"-");
  if (iVar3 == 0) {
    DAT_0011510e = 1;
    __fd = 0;
  }
  else {
    __fd = FUN_00107ea0(pcVar9,0);
    bVar1 = DAT_0011511a;
    if (DAT_0011511a == 0) {
      *(undefined1 *)((long)param_1 + 0x36) = 1;
      pcVar9 = (char *)*param_1;
      if (__fd == -1) {
LAB_00106c50:
        piVar8 = __errno_location();
        if (DAT_00115118 != '\0') {
          *(undefined4 *)(param_1 + 7) = 0xffffffff;
          iVar3 = *piVar8;
          *(byte *)((long)param_1 + 0x34) = bVar1 ^ 1;
          *(int *)((long)param_1 + 0x3c) = iVar3;
          param_1[5] = 0;
          param_1[4] = 0;
        }
        uVar6 = FUN_00105050(*param_1);
        uVar6 = FUN_0010b410(4,uVar6);
        uVar7 = dcgettext(0,"cannot open %s for reading",5);
                    /* WARNING: Subroutine does not return */
        error(0,*piVar8,uVar7,uVar6);
      }
      goto LAB_00106b04;
    }
    pcVar9 = (char *)*param_1;
    if (__fd == -1) {
      *(undefined1 *)((long)param_1 + 0x36) = 0;
      goto LAB_00106c50;
    }
  }
  *(undefined1 *)((long)param_1 + 0x36) = 1;
LAB_00106b04:
  if (DAT_00115115 != '\0') {
    uVar6 = FUN_00105050(pcVar9);
    FUN_00103a10(uVar6);
    pcVar9 = (char *)*param_1;
  }
  uVar6 = FUN_00105050(pcVar9);
  uVar4 = FUN_00106a90(uVar6,__fd,param_2,&local_e0);
  if (DAT_00115118 == '\0') {
    if (iVar3 != 0) {
      iVar3 = close(__fd);
      if (iVar3 != 0) {
        uVar6 = FUN_00105050(*param_1);
        uVar6 = FUN_0010b410(4,uVar6);
        uVar7 = dcgettext(0,"error reading %s",5);
        piVar8 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(0,*piVar8,uVar7,uVar6);
      }
    }
  }
  else {
    *(uint *)((long)param_1 + 0x3c) = (uVar4 & 0xff) - 1;
    iVar5 = fstat(__fd,&local_d8);
    bVar1 = DAT_0011511a;
    if (iVar5 < 0) {
      piVar8 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar8;
      uVar6 = FUN_00105050(*param_1);
      uVar6 = FUN_0010b410(4,uVar6);
      uVar7 = dcgettext(0,"error reading %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar8,uVar7,uVar6);
    }
    if ((((local_d8.st_mode & 0xf000) - 0x1000 & 0xffffe000) != 0) &&
       ((local_d8.st_mode & 0xb000) != 0x8000)) {
      *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
      pcVar9 = "";
      *(undefined1 *)((long)param_1 + 0x36) = 0;
      *(byte *)((long)param_1 + 0x34) = bVar1 ^ 1;
      if ((bVar1 ^ 1) != 0) {
        pcVar9 = (char *)dcgettext(0,"; giving up on this name",5);
      }
      uVar6 = FUN_00105050(*param_1);
      uVar6 = FUN_0010b510(0,3,uVar6);
      uVar7 = dcgettext(0,"%s: cannot follow end of this type of file%s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar7,uVar6,pcVar9);
    }
    if ((char)uVar4 == '\0') {
      uVar4 = 0;
      *(byte *)((long)param_1 + 0x34) = DAT_0011511a ^ 1;
      uVar6 = FUN_00105050(*param_1);
      FUN_00103b30(__fd,uVar6);
      *(undefined4 *)(param_1 + 7) = 0xffffffff;
    }
    else {
      FUN_00103830(param_1,__fd,local_e0,&local_d8,-(uint)(iVar3 == 0) | 1);
      uVar6 = FUN_00105050(*param_1);
      uVar2 = FUN_00104a40(__fd,uVar6);
      *(undefined1 *)((long)param_1 + 0x35) = uVar2;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0x6e60

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar3 = DAT_00115138;
  if (param_1 == 0) {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar2,uVar3);
    uVar3 = dcgettext(0,
                      "Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"
                      ,5);
    __printf_chk(1,uVar3,10);
    FUN_001039b0();
    FUN_001039e0();
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    uVar3 = dcgettext(0,
                      "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"
                      ,5);
    __printf_chk(1,uVar3,10,5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"  -z, --zero-terminated    line delimiter is NUL, not newline\n",5
                              );
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00104e20();
  }
  else {
    uVar2 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar3);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: is_prime @ 0x7f20

undefined8 is_prime(ulong param_1)

{
  undefined1 auVar1 [16];
  ulong uVar2;
  ulong uVar3;
  ulong uVar4;
  ulong uVar5;
  long lVar6;
  
  auVar1._8_8_ = 0;
  auVar1._0_8_ = param_1;
  uVar2 = SUB168(auVar1 * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  uVar4 = param_1 - ((SUB168(auVar1 * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) +
                    param_1 / 3);
  if ((9 < param_1) && (uVar4 != 0)) {
    lVar6 = 0x10;
    uVar5 = 9;
    uVar3 = 3;
    do {
      uVar3 = uVar3 + 2;
      uVar5 = uVar5 + lVar6;
      uVar2 = param_1 / uVar3;
      uVar4 = param_1 % uVar3;
      if (param_1 <= uVar5) break;
      lVar6 = lVar6 + 8;
    } while (uVar4 != 0);
  }
  return CONCAT71((int7)(uVar2 >> 8),uVar4 != 0);
}




// Function: next_prime @ 0x7f90

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong uVar2;
  
  uVar2 = 10;
  if (9 < param_1) {
    uVar2 = param_1;
  }
  uVar2 = uVar2 | 1;
  while ((uVar2 != 0xffffffffffffffff && (cVar1 = FUN_00107f20(uVar2), cVar1 == '\0'))) {
    uVar2 = uVar2 + 2;
  }
  return uVar2;
}




// Function: raw_hasher @ 0x7fd0

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = thunk_FUN_0010cab0(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0x7ff0

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0x8000

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0010f5d0) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0010f5e4 < fVar1) && (fVar1 < DAT_0010f5e8)) && (_DAT_0010f5ec < pfVar2[3])) &&
     (_DAT_0010f600 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0010f5e4;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0010f5f0)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0010f5d0;
  return 0;
}




// Function: free_entry @ 0x8090

void free_entry(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x48);
  *param_2 = 0;
  param_2[1] = uVar1;
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: safe_hasher @ 0x80b0

long safe_hasher(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  
  uVar1 = (*(code *)param_1[6])(param_2,param_1[2]);
  if (uVar1 < (ulong)param_1[2]) {
    return uVar1 * 0x10 + *param_1;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: hash_find_entry @ 0x80e0

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  long lVar5;
  
  plVar4 = (long *)FUN_001080b0();
  *param_3 = plVar4;
  lVar5 = *plVar4;
  if (lVar5 == 0) {
LAB_00108181:
    lVar5 = 0;
  }
  else {
    if (lVar5 != param_2) {
      cVar3 = (**(code **)(param_1 + 0x38))(param_2);
      if (cVar3 == '\0') {
        for (; (long *)plVar4[1] != (long *)0x0; plVar4 = (long *)plVar4[1]) {
          if ((*(long *)plVar4[1] == param_2) ||
             (cVar3 = (**(code **)(param_1 + 0x38))(param_2), cVar3 != '\0')) {
            lVar5 = *(long *)plVar4[1];
            if (param_4 == '\0') {
              return lVar5;
            }
            plVar4[1] = ((long *)plVar4[1])[1];
            FUN_00108090(param_1);
            return lVar5;
          }
        }
        goto LAB_00108181;
      }
      lVar5 = *plVar4;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar4[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar4 = *plVar1;
        plVar4[1] = lVar2;
        FUN_00108090(param_1);
        return lVar5;
      }
      *plVar4 = 0;
    }
  }
  return lVar5;
}




// Function: allocate_entry @ 0x81e0

void allocate_entry(long param_1)

{
  if (*(long *)(param_1 + 0x48) != 0) {
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)(*(long *)(param_1 + 0x48) + 8);
    return;
  }
  malloc(0x10);
  return;
}




// Function: transfer_entries @ 0x8210

undefined8 transfer_entries(long param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  
  plVar5 = (long *)*param_2;
  if ((long *)param_2[1] <= plVar5) {
    return 1;
  }
  do {
    while (lVar6 = *plVar5, lVar6 == 0) {
LAB_00108238:
      plVar5 = plVar5 + 2;
      if ((long *)param_2[1] <= plVar5) {
        return 1;
      }
    }
    plVar3 = (long *)plVar5[1];
    if ((long *)plVar5[1] != (long *)0x0) {
      do {
        while( true ) {
          lVar6 = *plVar3;
          plVar2 = (long *)FUN_001080b0(param_1,lVar6);
          plVar4 = (long *)plVar3[1];
          if (*plVar2 != 0) break;
          *plVar2 = lVar6;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_00108090(param_1);
          plVar3 = plVar4;
          if (plVar4 == (long *)0x0) goto LAB_001082a8;
        }
        plVar3[1] = plVar2[1];
        plVar2[1] = (long)plVar3;
        plVar3 = plVar4;
      } while (plVar4 != (long *)0x0);
LAB_001082a8:
      lVar6 = *plVar5;
    }
    plVar5[1] = 0;
    if (param_3 != '\0') goto LAB_00108238;
    plVar3 = (long *)FUN_001080b0(param_1,lVar6);
    if (*plVar3 == 0) {
      *plVar3 = lVar6;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    }
    else {
      plVar4 = (long *)FUN_001081e0(param_1);
      if (plVar4 == (long *)0x0) {
        return 0;
      }
      lVar1 = plVar3[1];
      *plVar4 = lVar6;
      plVar4[1] = lVar1;
      plVar3[1] = (long)plVar4;
    }
    *plVar5 = 0;
    plVar5 = plVar5 + 2;
    param_2[3] = param_2[3] + -1;
    if ((long *)param_2[1] <= plVar5) {
      return 1;
    }
  } while( true );
}




// Function: hash_get_n_buckets @ 0x83d0

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0x83e0

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0x83f0

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0x8400

ulong hash_get_max_bucket_length(undefined8 *param_1)

{
  long lVar1;
  long *plVar2;
  ulong uVar3;
  ulong uVar4;
  
  plVar2 = (long *)*param_1;
  uVar4 = 0;
  do {
    if ((long *)param_1[1] <= plVar2) {
      return uVar4;
    }
    while (*plVar2 != 0) {
      uVar3 = 1;
      for (lVar1 = plVar2[1]; lVar1 != 0; lVar1 = *(long *)(lVar1 + 8)) {
        uVar3 = uVar3 + 1;
      }
      if (uVar4 < uVar3) {
        uVar4 = uVar3;
      }
      plVar2 = plVar2 + 2;
      if ((long *)param_1[1] <= plVar2) {
        return uVar4;
      }
    }
    plVar2 = plVar2 + 2;
  } while( true );
}




// Function: hash_print_statistics @ 0x84d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_001083f0();
  uVar2 = FUN_001083d0();
  uVar3 = FUN_001083e0();
  uVar4 = FUN_00108400();
  __fprintf_chk(param_2,1,"# entries:         %lu\n",uVar1);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar2);
  __fprintf_chk(((double)uVar3 * _DAT_0010f600) / (double)uVar2,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar3);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: hash_lookup @ 0x85f0

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = (long *)FUN_001080b0();
  lVar3 = *plVar2;
  if (lVar3 == 0) {
    return 0;
  }
  while( true ) {
    if (lVar3 == param_2) {
      return param_2;
    }
    cVar1 = (**(code **)(param_1 + 0x38))(param_2);
    if (cVar1 != '\0') break;
    plVar2 = (long *)plVar2[1];
    if (plVar2 == (long *)0x0) {
      return 0;
    }
    lVar3 = *plVar2;
  }
  return *plVar2;
}




// Function: hash_get_next @ 0x86a0

long hash_get_next(long param_1,long param_2)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  
  plVar2 = (long *)FUN_001080b0();
  plVar3 = plVar2;
  do {
    lVar1 = *plVar3;
    plVar3 = (long *)plVar3[1];
    if (lVar1 == param_2) {
      if (plVar3 != (long *)0x0) {
        return *plVar3;
      }
      break;
    }
  } while (plVar3 != (long *)0x0);
  do {
    plVar2 = plVar2 + 2;
    if (*(long **)(param_1 + 8) <= plVar2) {
      return 0;
    }
  } while (*plVar2 == 0);
  return *plVar2;
}




// Function: hash_do_for_each @ 0x8760

long hash_do_for_each(ulong *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  
  plVar5 = (long *)*param_1;
  if (plVar5 < (long *)param_1[1]) {
    lVar4 = 0;
    do {
      lVar3 = *plVar5;
      plVar2 = plVar5;
      if (lVar3 != 0) {
        while( true ) {
          cVar1 = (*param_2)(lVar3,param_3);
          if (cVar1 == '\0') {
            return lVar4;
          }
          plVar2 = (long *)plVar2[1];
          lVar4 = lVar4 + 1;
          if (plVar2 == (long *)0x0) break;
          lVar3 = *plVar2;
        }
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
  else {
    lVar4 = 0;
  }
  return lVar4;
}




// Function: hash_initialize @ 0x8850

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_00107fd0;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_00107ff0;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0010f5d0;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_00108000(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_00108340(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
      __ptr[2] = __nmemb;
      if (__nmemb != 0) {
        pvVar2 = calloc(__nmemb,0x10);
        *__ptr = pvVar2;
        if (pvVar2 != (void *)0x0) {
          __ptr[6] = param_3;
          __ptr[3] = 0;
          __ptr[7] = param_4;
          __ptr[8] = param_5;
          __ptr[1] = (void *)((long)pvVar2 + __ptr[2] * 0x10);
          __ptr[4] = 0;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (undefined8 *)0x0;
}




// Function: hash_clear @ 0x8960

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  code *pcVar4;
  code *pcVar5;
  long *plVar6;
  long *plVar7;
  
  plVar6 = (long *)*param_1;
  if ((long *)*param_1 < (long *)param_1[1]) {
    do {
      while (*plVar6 != 0) {
        puVar3 = (undefined8 *)plVar6[1];
        pcVar4 = (code *)param_1[8];
        while (puVar3 != (undefined8 *)0x0) {
          pcVar5 = (code *)0x0;
          if (pcVar4 != (code *)0x0) {
            (*pcVar4)(*puVar3);
            pcVar5 = (code *)param_1[8];
          }
          puVar1 = (undefined8 *)puVar3[1];
          uVar2 = param_1[9];
          *puVar3 = 0;
          puVar3[1] = uVar2;
          param_1[9] = puVar3;
          puVar3 = puVar1;
          pcVar4 = pcVar5;
        }
        if (pcVar4 != (code *)0x0) {
          (*pcVar4)(*plVar6);
        }
        *plVar6 = 0;
        plVar7 = plVar6 + 2;
        plVar6[1] = 0;
        plVar6 = plVar7;
        if ((long *)param_1[1] <= plVar7) goto LAB_001089fb;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_001089fb:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x8a10

void hash_free(ulong *param_1)

{
  void *pvVar1;
  void *pvVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  
  plVar5 = (long *)*param_1;
  plVar3 = (long *)param_1[1];
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    if (plVar3 <= plVar5) goto LAB_00108abc;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_00108a7f;
      }
      while( true ) {
        (*(code *)param_1[8])(lVar6);
        plVar4 = (long *)plVar4[1];
        if (plVar4 == (long *)0x0) break;
        lVar6 = *plVar4;
      }
      plVar3 = (long *)param_1[1];
      plVar5 = plVar5 + 2;
    } while (plVar5 < plVar3);
LAB_00108a7f:
    plVar5 = (long *)*param_1;
  }
  if (plVar5 < plVar3) {
    do {
      pvVar2 = (void *)plVar5[1];
      while (pvVar2 != (void *)0x0) {
        pvVar1 = *(void **)((long)pvVar2 + 8);
        free(pvVar2);
        pvVar2 = pvVar1;
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
LAB_00108abc:
  pvVar2 = (void *)param_1[9];
  while (pvVar2 != (void *)0x0) {
    pvVar1 = *(void **)((long)pvVar2 + 8);
    free(pvVar2);
    pvVar2 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: hash_rehash @ 0x8b00

undefined4 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 uVar2;
  size_t __nmemb;
  long in_FS_OFFSET;
  void *local_88;
  void *local_80;
  size_t local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_00108340(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_00108c06;
    }
    local_88 = calloc(__nmemb,0x10);
    if (local_88 != (void *)0x0) {
      local_80 = (void *)((long)local_88 + __nmemb * 0x10);
      local_70 = 0;
      local_60 = param_1[5];
      local_68 = 0;
      local_58 = param_1[6];
      local_50 = param_1[7];
      local_48 = param_1[8];
      local_40 = param_1[9];
      local_78 = __nmemb;
      uVar2 = FUN_00108210(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_00108210(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_00108210(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_00108c06;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_88;
      param_1[1] = local_80;
      param_1[2] = local_78;
      param_1[3] = local_70;
      param_1[9] = local_40;
      goto LAB_00108c06;
    }
  }
  uVar2 = 0;
LAB_00108c06:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0x8c80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  long *plVar3;
  ulong uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  float fVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  long *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
LAB_00108e47:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  lVar2 = FUN_001080e0(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar5 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_00108cce;
  }
  uVar4 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar4 < 0) {
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar4;
  }
  else {
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar4;
  }
  uVar4 = *(ulong *)(param_1 + 0x10);
  if ((long)uVar4 < 0) {
    auVar7._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar7._0_4_ = (float)uVar4;
  }
  else {
    auVar7._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar7._0_4_ = (float)(long)uVar4;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar7._0_4_ < auVar8._0_4_) {
    FUN_00108000(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar8._0_4_ <= auVar7._0_4_ * *(float *)(lVar2 + 8)) goto LAB_00108d2e;
    fVar6 = auVar7._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar6 = fVar6 * *(float *)(lVar2 + 8);
    }
    if (fVar6 < _DAT_0010f5f4) {
      if (_DAT_0010f5f8 <= fVar6) {
        uVar4 = (long)(fVar6 - _DAT_0010f5f8) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)fVar6;
      }
      cVar1 = FUN_00108b00(param_1,uVar4);
      if (cVar1 != '\0') {
        lVar2 = FUN_001080e0(param_1,param_2,&local_38,0);
        if (lVar2 != 0) goto LAB_00108e47;
        goto LAB_00108d2e;
      }
    }
  }
  else {
LAB_00108d2e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar5 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_00108cce;
    }
    plVar3 = (long *)FUN_001081e0(param_1);
    if (plVar3 != (long *)0x0) {
      *plVar3 = param_2;
      uVar5 = 1;
      plVar3[1] = local_38[1];
      local_38[1] = (long)plVar3;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      goto LAB_00108cce;
    }
  }
  uVar5 = 0xffffffff;
LAB_00108cce:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert @ 0x8e70

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00108c80(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0x8ed0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long hash_remove(long param_1,undefined8 param_2)

{
  float *pfVar1;
  void *pvVar2;
  void *__ptr;
  char cVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar4 = FUN_001080e0(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_00108f13;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_00108f60;
LAB_00109016:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_00109016;
LAB_00108f60:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_00108000(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0010f5f8 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0010f5f8) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_00108b00(param_1,uVar5);
      if (cVar3 == '\0') {
        __ptr = *(void **)(param_1 + 0x48);
        while (__ptr != (void *)0x0) {
          pvVar2 = *(void **)((long)__ptr + 8);
          free(__ptr);
          __ptr = pvVar2;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
      }
    }
  }
LAB_00108f13:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}



