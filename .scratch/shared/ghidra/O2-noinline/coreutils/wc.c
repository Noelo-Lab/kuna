// Function: main @ 0x28a0

byte main(uint param_1,undefined8 *param_2)

{
  byte bVar1;
  byte bVar2;
  int iVar3;
  char *pcVar4;
  undefined8 uVar5;
  char *__s1;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  ulong uVar9;
  long lVar10;
  FILE *__stream;
  uint *puVar11;
  uint *puVar12;
  char *pcVar13;
  undefined **__s2;
  long in_FS_OFFSET;
  double dVar14;
  double dVar15;
  double local_228;
  uint *local_218;
  char local_209;
  int local_1fc;
  double local_1f8;
  undefined8 *local_1f0;
  stat local_d8;
  long local_40;
  
  __s2 = &PTR_s_bytes_0010da20;
  pcVar13 = "clLmw";
  __stream = (FILE *)(ulong)param_1;
  puVar11 = &switchD_00102a03::switchdataD_001098d8;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00104ea0(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils");
  textdomain("coreutils");
  FUN_00108cd0(FUN_00104870);
  iVar3 = getpagesize();
  DAT_0010e120 = (long)iVar3;
  setvbuf(stdout,(char *)0x0,1,0);
  pcVar4 = getenv("POSIXLY_CORRECT");
  DAT_0010e131 = '\0';
  DAT_0010e132 = '\0';
  DAT_0010e118 = pcVar4 != (char *)0x0;
  DAT_0010e133 = '\0';
  DAT_0010e134 = '\0';
  DAT_0010e130 = '\0';
  DAT_0010e138 = 0;
  DAT_0010e140 = 0;
  DAT_0010e148 = 0;
  DAT_0010e150 = 0;
  DAT_0010e158 = 0;
  pcVar4 = (char *)0x0;
  while( true ) {
    uVar5 = getopt_long(param_1,param_2,"clLmw",&PTR_s_bytes_0010da20,0);
    iVar3 = (int)uVar5;
    if (iVar3 == -1) break;
    if (0x81 < iVar3) goto switchD_00102a03_caseD_4d;
    if (iVar3 < 0x4c) {
      if (iVar3 == -0x83) {
        FUN_00107d20(stdout,&DAT_001095fb,"GNU coreutils",PTR_DAT_0010e018,"Paul Rubin",
                     "David MacKenzie",0,uVar5);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar3 != -0x82) goto switchD_00102a03_caseD_4d;
      FUN_001045c0(0);
      break;
    }
    switch(iVar3) {
    case 0x4c:
      DAT_0010e130 = '\x01';
      break;
    default:
      goto switchD_00102a03_caseD_4d;
    case 99:
      DAT_0010e131 = '\x01';
      break;
    case 0x6c:
      DAT_0010e134 = '\x01';
      break;
    case 0x6d:
      DAT_0010e132 = '\x01';
      break;
    case 0x77:
      DAT_0010e133 = '\x01';
      break;
    case 0x80:
      DAT_0010e160 = 1;
      break;
    case 0x81:
      pcVar4 = optarg;
    }
  }
  if ((((DAT_0010e134 == '\0') && (DAT_0010e133 == '\0')) && (DAT_0010e132 == '\0')) &&
     ((DAT_0010e131 == '\0' && (DAT_0010e130 == '\0')))) {
    DAT_0010e131 = '\x01';
    DAT_0010e133 = '\x01';
    DAT_0010e134 = '\x01';
  }
  iVar3 = optind;
  if (pcVar4 == (char *)0x0) goto LAB_00102f18;
  if (optind < (int)param_1) {
    pcVar4 = (char *)FUN_00107110(4,param_2[optind]);
    uVar5 = dcgettext(0,"extra operand %s",5);
    error(0,0,uVar5,pcVar4);
    uVar5 = dcgettext(0,"file operands cannot be combined with --files0-from",5);
    __fprintf_chk(stderr,1,"%s\n",uVar5);
switchD_00102a03_caseD_4d:
    FUN_001045c0(1);
LAB_00103052:
    dVar15 = (double)FUN_00104d60();
    dVar15 = DAT_001099c8 * dVar15;
LAB_00102f85:
    if (local_228 <= dVar15) {
      FUN_001074d0(&local_1f8);
      local_209 = FUN_00107580(__stream,&local_1f8);
      if ((local_209 == '\0') || (iVar3 = FUN_00104960(__stream), iVar3 != 0)) {
        uVar5 = FUN_00107110(4,pcVar4);
        uVar6 = dcgettext(0,"cannot read file names from %s",5);
        error(1,0,uVar6,uVar5);
LAB_001030da:
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      local_228 = local_1f8;
      __stream = (FILE *)FUN_00104700(local_1f0);
      param_2 = local_1f0;
      goto LAB_00102b30;
    }
  }
  else {
    iVar3 = strcmp(pcVar4,"-");
    __stream = stdin;
    if ((iVar3 != 0) && (__stream = fopen(pcVar4,"r"), __stream == (FILE *)0x0)) {
      __s1 = (char *)FUN_00107110(4,pcVar4);
      pcVar4 = (char *)dcgettext(0,"cannot open %s for reading",5);
      piVar8 = __errno_location();
      error(1,*piVar8,pcVar4,__s1);
      __stream = (FILE *)0x0;
      goto LAB_00102cd0;
    }
    iVar3 = fileno(__stream);
    iVar3 = fstat(iVar3,&local_d8);
    if ((iVar3 == 0) && ((local_d8.st_mode & 0xf000) == 0x8000)) {
      local_228 = (double)local_d8.st_size;
      dVar14 = (double)FUN_00104d60();
      dVar15 = DAT_001099c0;
      if (dVar14 * DAT_001099c8 <= DAT_001099c0) goto LAB_00103052;
      goto LAB_00102f85;
    }
  }
  param_2 = (undefined8 *)0x0;
  __stream = (FILE *)FUN_00104730(__stream);
  local_209 = '\0';
  local_228 = 0.0;
LAB_00102b30:
  while (__stream != (FILE *)0x0) {
    pcVar13 = (char *)&local_1fc;
    puVar11 = (uint *)FUN_001032f0(local_228,param_2);
    __s2 = (undefined **)&DAT_00109a72;
    DAT_0010e12c = FUN_00103230(local_228,puVar11);
    bVar2 = 1;
    local_218 = puVar11;
    while (__s1 = (char *)FUN_00104770(__stream,pcVar13), __s1 != (char *)0x0) {
      if (pcVar4 == (char *)0x0) {
        if (*__s1 == '\0') {
          uVar5 = dcgettext(0,"invalid zero-length file name",5);
          error(0,0,"%s",uVar5);
LAB_00102c21:
          bVar2 = 0;
        }
        else {
LAB_00102ba5:
          puVar12 = local_218;
          if (local_228 != 0.0) {
            puVar12 = puVar11;
          }
          bVar1 = FUN_00104500(__s1,puVar12);
          bVar2 = bVar2 & bVar1;
        }
      }
      else {
        iVar3 = strcmp(pcVar4,(char *)__s2);
        if ((iVar3 == 0) && (iVar3 = strcmp(__s1,(char *)__s2), iVar3 == 0)) {
LAB_00102cd0:
          uVar5 = FUN_00107110(4,__s1);
          uVar6 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",5);
          error(0,0,uVar6,uVar5);
          if (*__s1 != '\0') goto LAB_00102c21;
        }
        else if (*__s1 != '\0') goto LAB_00102ba5;
        uVar5 = FUN_00104810(__stream);
        uVar6 = dcgettext(0,"invalid zero-length file name",5);
        uVar7 = FUN_00107210(0,3,pcVar4);
        error(0,0,"%s:%lu: %s",uVar7,uVar5,uVar6);
        bVar2 = 0;
      }
      if (local_228 == 0.0) {
        *local_218 = 1;
      }
      puVar11 = puVar11 + 0x26;
    }
    if (local_1fc == 3) break;
    if (local_1fc == 4) {
      param_2 = (undefined8 *)FUN_00107210(0,3,pcVar4);
      pcVar4 = (char *)dcgettext(0,"%s: read error",5);
      piVar8 = __errno_location();
      error(0,*piVar8,pcVar4,param_2);
      bVar2 = 0;
    }
    else {
      if (local_1fc != 2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main");
      }
      bVar1 = pcVar4 == (char *)0x0 & bVar2;
      param_2 = (undefined8 *)0x0;
      if ((bVar1 != 0) && (lVar10 = FUN_00104810(__stream), bVar2 = bVar1, lVar10 == 0)) {
        bVar2 = FUN_00104500(0,local_218);
      }
    }
    if (local_209 != '\0') {
      FUN_00107550(&local_1f8);
    }
    uVar9 = FUN_00104810(__stream);
    if (1 < uVar9) {
      uVar5 = dcgettext(0,"total",5);
      FUN_001035d0(DAT_0010e158,DAT_0010e150,DAT_0010e148,DAT_0010e140,DAT_0010e138,uVar5);
    }
    FUN_00104830(__stream);
    free(local_218);
    if ((DAT_0010e128 == '\0') || (iVar3 = close(0), iVar3 == 0)) {
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return bVar2 ^ 1;
      }
      goto LAB_001030da;
    }
    piVar8 = __errno_location();
    iVar3 = error(1,*piVar8,&DAT_00109a72);
LAB_00102f18:
    if (iVar3 < (int)__stream) {
      local_228 = (double)(long)((int)__stream - iVar3);
      param_2 = param_2 + iVar3;
    }
    else {
      local_228 = 4.94065645841247e-324;
      param_2 = (undefined8 *)&DAT_0010e110;
    }
    __stream = (FILE *)FUN_00104700(param_2);
    local_209 = '\0';
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108250();
}




// Function: iswnbspace @ 0x31f0

bool iswnbspace(int param_1)

{
  bool bVar1;
  
  bVar1 = false;
  if (DAT_0010e118 == '\0') {
    if ((param_1 != 0xa0) && (param_1 != 0x2007)) {
      return param_1 == 0x202f || param_1 == 0x2060;
    }
    bVar1 = true;
  }
  return bVar1;
}




// Function: compute_number_width @ 0x3230

int compute_number_width(long param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  ulong uVar3;
  int *piVar4;
  int iVar5;
  
  if ((param_1 != 0) && (*param_2 < 1)) {
    uVar3 = 0;
    iVar5 = 1;
    piVar4 = param_2 + param_1 * 0x26;
    do {
      if (*param_2 == 0) {
        if ((param_2[8] & 0xf000U) == 0x8000) {
          uVar3 = uVar3 + *(long *)(param_2 + 0xe);
        }
        else {
          iVar5 = 7;
        }
      }
      param_2 = param_2 + 0x26;
    } while (param_2 != piVar4);
    iVar2 = 1;
    if (9 < uVar3) {
      do {
        iVar2 = iVar2 + 1;
        bVar1 = 99 < uVar3;
        uVar3 = uVar3 / 10;
      } while (bVar1);
    }
    if (iVar5 <= iVar2) {
      iVar5 = iVar2;
    }
    return iVar5;
  }
  return 1;
}




// Function: get_input_fstatus @ 0x32f0

undefined4 * get_input_fstatus(long param_1,long param_2)

{
  char *__s1;
  int iVar1;
  undefined4 *puVar2;
  long lVar3;
  stat *__buf;
  
  lVar3 = 1;
  if (param_1 != 0) {
    lVar3 = param_1;
  }
  puVar2 = (undefined4 *)FUN_00107fb0(lVar3,0x98);
  if ((param_1 == 0) ||
     ((param_1 == 1 &&
      ((uint)DAT_0010e134 + (uint)DAT_0010e133 + (uint)DAT_0010e132 + (uint)DAT_0010e131 +
       (uint)DAT_0010e130 == 1)))) {
    *puVar2 = 1;
  }
  else {
    lVar3 = 0;
    __buf = (stat *)(puVar2 + 2);
    do {
      __s1 = *(char **)(param_2 + lVar3 * 8);
      if (__s1 == (char *)0x0) {
LAB_00103348:
        iVar1 = fstat(0,__buf);
      }
      else {
        iVar1 = strcmp(__s1,"-");
        if (iVar1 == 0) goto LAB_00103348;
        iVar1 = stat(__s1,__buf);
      }
      lVar3 = lVar3 + 1;
      *(int *)(__buf[-1].__unused + 2) = iVar1;
      __buf = (stat *)&__buf[1].st_ino;
    } while (param_1 != lVar3);
  }
  return puVar2;
}




// Function: wc_lines @ 0x33e0

undefined1 wc_lines(undefined8 param_1,undefined4 param_2,long param_3,long param_4)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  undefined1 *puVar4;
  ulong uVar5;
  char *pcVar6;
  char *pcVar7;
  undefined8 uVar8;
  int *piVar9;
  bool bVar10;
  long lVar11;
  undefined1 *puVar12;
  long lVar13;
  long in_FS_OFFSET;
  undefined1 local_4030 [16384];
  
  puVar4 = &stack0xffffffffffffffd0;
  do {
    puVar12 = puVar4;
    *(undefined8 *)(puVar12 + -0x1000) = *(undefined8 *)(puVar12 + -0x1000);
    puVar4 = puVar12 + -0x1000;
  } while (puVar12 + -0x1000 != local_4030);
  *(long *)(puVar12 + -0x1030) = param_3;
  *(undefined8 *)(puVar12 + -0x1020) = param_1;
  *(long *)(puVar12 + -0x1028) = param_4;
  *(undefined8 *)(puVar12 + 0x2ff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
  bVar10 = param_3 == 0 || param_4 == 0;
  puVar12[-0x1031] = bVar10;
  if (bVar10) {
    puVar12[-0x1031] = 0;
  }
  else {
    bVar10 = false;
    pcVar2 = puVar12 + -0x1018;
    *(undefined8 *)(puVar12 + -0x1040) = 0;
    lVar13 = 0;
    while( true ) {
      *(undefined8 *)(puVar12 + -0x1050) = 0x103470;
      uVar5 = FUN_00107770(param_2,pcVar2,0x4000);
      if (uVar5 == 0) break;
      if (uVar5 == 0xffffffffffffffff) {
        *(undefined8 *)(puVar12 + -0x1050) = 0x103581;
        uVar8 = FUN_00107210(0,3,*(undefined8 *)(puVar12 + -0x1020));
        *(undefined8 *)(puVar12 + -0x1050) = 0x103589;
        piVar9 = __errno_location();
        iVar3 = *piVar9;
        *(undefined8 *)(puVar12 + -0x1050) = 0x10359e;
        error(0,iVar3,"%s",uVar8);
        goto LAB_00103525;
      }
      *(ulong *)(puVar12 + -0x1040) = *(long *)(puVar12 + -0x1040) + uVar5;
      pcVar1 = pcVar2 + uVar5;
      *(ulong *)(puVar12 + -0x1048) = uVar5 / 0xf;
      if (bVar10) {
        *pcVar1 = '\n';
        lVar11 = lVar13;
        pcVar7 = pcVar2;
        while( true ) {
          *(undefined8 *)(puVar12 + -0x1050) = 0x103502;
          pcVar7 = rawmemchr(pcVar7,10);
          if (pcVar1 <= pcVar7) break;
          pcVar7 = pcVar7 + 1;
          lVar11 = lVar11 + 1;
        }
        bVar10 = (ulong)(lVar11 - lVar13) <= *(ulong *)(puVar12 + -0x1048);
        lVar13 = lVar11;
      }
      else {
        pcVar7 = pcVar2;
        lVar11 = lVar13;
        if (pcVar1 == pcVar2) {
          bVar10 = true;
        }
        else {
          do {
            pcVar6 = pcVar7 + 1;
            lVar11 = lVar11 + (ulong)(*pcVar7 == '\n');
            pcVar7 = pcVar6;
          } while (pcVar1 != pcVar6);
          bVar10 = (ulong)(lVar11 - lVar13) <= *(ulong *)(puVar12 + -0x1048);
          lVar13 = lVar11;
        }
      }
    }
    puVar12[-0x1031] = 1;
    **(undefined8 **)(puVar12 + -0x1028) = *(undefined8 *)(puVar12 + -0x1040);
    **(long **)(puVar12 + -0x1030) = lVar13;
  }
LAB_00103525:
  if (*(long *)(puVar12 + 0x2ff0) != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(code **)(puVar12 + -0x1050) = FUN_001035b0;
    __stack_chk_fail();
  }
  return puVar12[-0x1031];
}




// Function: isnbspace @ 0x35b0

void isnbspace(int param_1)

{
  wint_t wVar1;
  
  wVar1 = btowc(param_1);
  FUN_001031f0(wVar1);
  return;
}




// Function: write_counts @ 0x35d0

void write_counts(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,char *param_6)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined *puVar3;
  undefined *puVar4;
  long in_FS_OFFSET;
  undefined1 auStack_58 [24];
  long local_40;
  
  puVar3 = &DAT_001099b1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0010e134 != '\0') {
    uVar2 = FUN_00104ad0(param_1,auStack_58);
    puVar3 = &DAT_001099b0;
    __printf_chk(1,&DAT_001099b1,DAT_0010e12c,uVar2);
  }
  puVar4 = puVar3;
  if (DAT_0010e133 != '\0') {
    uVar2 = FUN_00104ad0(param_2,auStack_58);
    puVar4 = &DAT_001099b0;
    __printf_chk(1,puVar3,DAT_0010e12c,uVar2);
  }
  puVar3 = puVar4;
  if (DAT_0010e132 != '\0') {
    uVar2 = FUN_00104ad0(param_3,auStack_58);
    puVar3 = &DAT_001099b0;
    __printf_chk(1,puVar4,DAT_0010e12c,uVar2);
  }
  puVar4 = puVar3;
  if (DAT_0010e131 != '\0') {
    uVar2 = FUN_00104ad0(param_4,auStack_58);
    puVar4 = &DAT_001099b0;
    __printf_chk(1,puVar3,DAT_0010e12c,uVar2);
  }
  if (DAT_0010e130 != '\0') {
    uVar2 = FUN_00104ad0(param_5,auStack_58);
    __printf_chk(1,puVar4,DAT_0010e12c,uVar2);
  }
  if (param_6 != (char *)0x0) {
    pcVar1 = strchr(param_6,10);
    if (pcVar1 != (char *)0x0) {
      param_6 = (char *)FUN_00107210(0,3,param_6);
    }
    __printf_chk(1," %s",param_6);
  }
  putchar_unlocked(10);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: avx2_supported @ 0x3830

/* WARNING: Type propagation algorithm not settling */

char avx2_supported(void)

{
  int iVar1;
  undefined8 uVar2;
  char *pcVar3;
  undefined4 *puVar4;
  char cVar5;
  long in_FS_OFFSET;
  undefined4 local_40;
  uint local_3c;
  uint local_38 [2];
  long local_30;
  
  puVar4 = &local_40;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = 0;
  local_3c = 0;
  local_38[0] = 0;
  local_38[1] = 0;
  iVar1 = FUN_001037b0(puVar4,&local_3c,local_38,local_38 + 1);
  cVar5 = DAT_0010e160;
  if (iVar1 == 0) {
LAB_001038d8:
    if (cVar5 != '\0') {
      pcVar3 = "failed to get cpuid";
      goto LAB_001038e9;
    }
  }
  else {
    if ((local_38[0] & 0x8000000) != 0) {
      local_38[1] = 0;
      local_38[0] = 0;
      local_3c = 0;
      local_40 = 0;
      iVar1 = FUN_001037f0(puVar4,&local_3c,local_38,local_38 + 1);
      if (iVar1 == 0) goto LAB_001038d8;
      if ((local_3c & 0x20) != 0) {
        if (cVar5 == '\0') {
          cVar5 = '\x01';
        }
        else {
          uVar2 = dcgettext(0,"using avx2 hardware support",5);
          error(0,0,"%s",uVar2);
        }
        goto LAB_001038af;
      }
    }
    if (cVar5 != '\0') {
      pcVar3 = "avx2 support not detected";
LAB_001038e9:
      uVar2 = dcgettext(0,pcVar3,5);
      error(0,0,"%s",uVar2);
    }
  }
  cVar5 = '\0';
LAB_001038af:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wc @ 0x3bf0

undefined4 wc(undefined4 param_1,long param_2,int *param_3,long param_4)

{
  ushort uVar1;
  undefined1 *puVar2;
  byte bVar3;
  char cVar4;
  undefined4 uVar5;
  int iVar6;
  size_t sVar7;
  __off_t _Var8;
  long lVar9;
  ushort **ppuVar10;
  undefined8 uVar11;
  int *piVar12;
  ulong uVar13;
  ulong uVar14;
  ulong uVar15;
  undefined1 *puVar16;
  long lVar17;
  wint_t __wc;
  char *pcVar18;
  char *pcVar19;
  ulong uVar20;
  long lVar21;
  long in_FS_OFFSET;
  bool bVar22;
  undefined1 local_4030 [16384];
  
  puVar2 = &stack0xffffffffffffffd0;
  do {
    puVar16 = puVar2;
    *(undefined8 *)(puVar16 + -0x1000) = *(undefined8 *)(puVar16 + -0x1000);
    puVar2 = puVar16 + -0x1000;
  } while (puVar16 + -0x1000 != local_4030);
  *(undefined4 *)(puVar16 + -0x1050) = param_1;
  *(long *)(puVar16 + -0x1048) = param_2;
  *(undefined8 *)(puVar16 + 0x2ff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
  *(long *)(puVar16 + -0x1040) = param_2;
  if (param_2 == 0) {
    *(undefined8 *)(puVar16 + -0x1080) = 0x1042d0;
    uVar11 = dcgettext(0,"standard input",5);
    *(undefined8 *)(puVar16 + -0x1040) = uVar11;
  }
  *(undefined8 *)(puVar16 + -0x1028) = 0;
  *(undefined8 *)(puVar16 + -0x1030) = 0;
  *(undefined8 *)(puVar16 + -0x1080) = 0x103c5e;
  sVar7 = __ctype_get_mb_cur_max();
  bVar3 = DAT_0010e131;
  if (sVar7 < 2) {
    puVar16[-0x1049] = 0;
    if (DAT_0010e131 == 0) {
      bVar3 = DAT_0010e132;
    }
  }
  else {
    puVar16[-0x1049] = DAT_0010e132;
  }
  if ((DAT_0010e133 == '\0') && (DAT_0010e130 == '\0')) {
    if ((bVar3 == 1) && (puVar16[-0x1049] == '\0')) {
      if (DAT_0010e134 != '\0') {
        *(undefined8 *)(puVar16 + -0x1080) = 0x10446e;
        posix_fadvise(*(int *)(puVar16 + -0x1050),0,0,2);
LAB_0010446e:
        if (DAT_0010e134 != '\0') {
LAB_00104480:
          *(undefined8 *)(puVar16 + -0x1080) = 0x104485;
          cVar4 = FUN_00103830();
          if (cVar4 != '\0') {
            PTR_FUN_0010e010 = FUN_00108a90;
          }
          uVar15 = 0;
          uVar20 = 0;
          lVar21 = 0;
          *(undefined8 *)(puVar16 + -0x1080) = 0x1044ac;
          uVar5 = (*(code *)PTR_FUN_0010e010)
                            (*(undefined8 *)(puVar16 + -0x1040),*(undefined4 *)(puVar16 + -0x1050),
                             puVar16 + -0x1030,puVar16 + -0x1028);
          uVar14 = *(ulong *)(puVar16 + -0x1028);
          goto LAB_00103db8;
        }
      }
      iVar6 = *param_3;
      if (0 < iVar6) {
        *(undefined8 *)(puVar16 + -0x1080) = 0x1043ec;
        iVar6 = fstat(*(int *)(puVar16 + -0x1050),(stat *)(param_3 + 2));
        *param_3 = iVar6;
      }
      if (iVar6 == 0) {
        iVar6 = param_3[8];
        *(undefined8 *)(puVar16 + -0x1080) = 0x103cd5;
        uVar5 = FUN_00103be0(iVar6);
        uVar20 = DAT_0010e120;
        bVar3 = (byte)uVar5;
        if ((bVar3 != 0) && (uVar14 = *(ulong *)(param_3 + 0xe), -1 < (long)uVar14)) {
          if (param_4 == -1) {
            *(ulong *)(puVar16 + -0x1078) = uVar14;
            *(undefined8 *)(puVar16 + -0x1080) = 0x104414;
            uVar13 = lseek(*(int *)(puVar16 + -0x1050),0,1);
            uVar15 = *(ulong *)(puVar16 + -0x1078);
            if (uVar15 % uVar20 != 0) {
              if (uVar15 < uVar13) {
                uVar14 = 0;
              }
              else {
                uVar14 = uVar15 - uVar13;
              }
              goto LAB_0010442f;
            }
            bVar3 = (byte)~(byte)(uVar13 >> 0x38) >> 7;
          }
          else {
            uVar13 = 0;
            if (uVar14 % DAT_0010e120 != 0) {
LAB_0010442f:
              *(ulong *)(puVar16 + -0x1028) = uVar14;
              uVar15 = 0;
              uVar20 = 0;
              lVar21 = 0;
              goto LAB_00103db8;
            }
          }
          uVar20 = 0x201;
          if (*(long *)(param_3 + 0x10) - 1U < 0x2000000000000000) {
            uVar20 = *(long *)(param_3 + 0x10) + 1;
          }
          lVar21 = uVar14 - uVar14 % uVar20;
          if (((long)uVar13 < lVar21) && (bVar3 != 0)) {
            *(undefined8 *)(puVar16 + -0x1080) = 0x103d58;
            _Var8 = lseek(*(int *)(puVar16 + -0x1050),lVar21,1);
            if (-1 < _Var8) {
              *(ulong *)(puVar16 + -0x1028) = lVar21 - uVar13;
            }
          }
        }
      }
      iVar6 = *(int *)(puVar16 + -0x1050);
      *(undefined8 *)(puVar16 + -0x1080) = 0x103d7e;
      posix_fadvise(iVar6,0,0,2);
      while( true ) {
        *(undefined8 *)(puVar16 + -0x1080) = 0x103d9e;
        lVar21 = FUN_00107770(iVar6,puVar16 + -0x1018,0x4000);
        if (lVar21 == 0) break;
        if (lVar21 == -1) {
          uVar15 = 0;
          lVar21 = 0;
          *(undefined8 *)(puVar16 + -0x1080) = 0x1043b5;
          uVar11 = FUN_00107210(0,3,*(undefined8 *)(puVar16 + -0x1040));
          *(undefined8 *)(puVar16 + -0x1080) = 0x1043bd;
          piVar12 = __errno_location();
          iVar6 = *piVar12;
          uVar20 = 0;
          *(undefined8 *)(puVar16 + -0x1080) = 0x1043d5;
          error(0,iVar6,"%s",uVar11);
          uVar14 = *(ulong *)(puVar16 + -0x1028);
          uVar5 = 0;
          goto LAB_00103db8;
        }
        *(long *)(puVar16 + -0x1028) = *(long *)(puVar16 + -0x1028) + lVar21;
      }
      uVar14 = *(ulong *)(puVar16 + -0x1028);
      uVar15 = 0;
      lVar21 = 0;
      uVar5 = 1;
      uVar20 = 0;
      goto LAB_00103db8;
    }
    *(undefined8 *)(puVar16 + -0x1080) = 0x1044db;
    posix_fadvise(*(int *)(puVar16 + -0x1050),0,0,2);
    if (puVar16[-0x1049] != '\x01') {
      if (bVar3 != 0) goto LAB_0010446e;
      if (puVar16[-0x1049] == '\0') goto LAB_00104480;
    }
  }
  else {
    *(undefined8 *)(puVar16 + -0x1080) = 0x103e5a;
    posix_fadvise(*(int *)(puVar16 + -0x1050),0,0,2);
  }
  *(undefined8 *)(puVar16 + -0x1080) = 0x103e5f;
  sVar7 = __ctype_get_mb_cur_max();
  if (sVar7 < 2) {
    uVar20 = 0;
    uVar15 = 0;
    *(undefined1 **)(puVar16 + -0x1058) = puVar16 + -0x1018;
    lVar21 = 0;
    uVar13 = uVar15;
    while( true ) {
      *(ulong *)(puVar16 + -0x1078) = uVar20;
      *(undefined8 *)(puVar16 + -0x1080) = 0x1041a7;
      lVar9 = FUN_00107770(*(undefined4 *)(puVar16 + -0x1050),*(undefined8 *)(puVar16 + -0x1058),
                           0x4000);
      uVar20 = *(ulong *)(puVar16 + -0x1078);
      if (lVar9 == 0) break;
      if (lVar9 == -1) {
        *(undefined8 *)(puVar16 + -0x1080) = 0x10437e;
        uVar11 = FUN_00107210(0,3,*(undefined8 *)(puVar16 + -0x1040));
        *(undefined8 *)(puVar16 + -0x1080) = 0x104386;
        piVar12 = __errno_location();
        iVar6 = *piVar12;
        uVar5 = 0;
        *(undefined8 *)(puVar16 + -0x1080) = 0x10439e;
        error(0,iVar6,"%s",uVar11);
        goto LAB_00104347;
      }
      *(long *)(puVar16 + -0x1028) = *(long *)(puVar16 + -0x1028) + lVar9;
      pcVar19 = *(char **)(puVar16 + -0x1058);
      pcVar18 = pcVar19 + lVar9;
      do {
        cVar4 = *pcVar19;
        pcVar19 = pcVar19 + 1;
        switch(cVar4) {
        case '\t':
          uVar20 = (uVar20 & 0xfffffffffffffff8) + 8;
          break;
        case '\n':
          *(long *)(puVar16 + -0x1030) = *(long *)(puVar16 + -0x1030) + 1;
        case '\f':
        case '\r':
          if (uVar15 < uVar20) {
            uVar15 = uVar20;
          }
          uVar20 = 0;
          break;
        case '\v':
          break;
        default:
          *(ulong *)(puVar16 + -0x1070) = uVar20;
          *(char **)(puVar16 + -0x1078) = pcVar18;
          *(undefined8 *)(puVar16 + -0x1080) = 0x1041fe;
          ppuVar10 = __ctype_b_loc();
          *(undefined8 *)(puVar16 + -0x1080) = 0x10420a;
          bVar3 = FUN_001031e0((int)cVar4);
          pcVar18 = *(char **)(puVar16 + -0x1078);
          uVar20 = *(ulong *)(puVar16 + -0x1070);
          if (((*ppuVar10)[bVar3] & 0x4000) != 0) {
            uVar20 = uVar20 + 1;
            if (((*ppuVar10)[bVar3] & 0x2000) == 0) {
              *(ulong *)(puVar16 + -0x1070) = uVar20;
              *(undefined8 *)(puVar16 + -0x1080) = 0x10423c;
              iVar6 = FUN_001035b0(bVar3);
              pcVar18 = *(char **)(puVar16 + -0x1078);
              uVar20 = *(ulong *)(puVar16 + -0x1070);
              if (iVar6 == 0) {
                lVar21 = 1;
                goto LAB_00104250;
              }
            }
            break;
          }
          goto LAB_00104250;
        case ' ':
          uVar20 = uVar20 + 1;
        }
        uVar13 = uVar13 + lVar21;
        lVar21 = 0;
LAB_00104250:
      } while (pcVar18 != pcVar19);
    }
    uVar5 = 1;
LAB_00104347:
    uVar14 = *(ulong *)(puVar16 + -0x1028);
    if (uVar15 < uVar20) {
      uVar15 = uVar20;
    }
    lVar21 = lVar21 + uVar13;
    uVar20 = 0;
  }
  else {
    lVar21 = 0;
    bVar22 = false;
    uVar13 = 0;
    *(undefined8 *)(puVar16 + -0x1020) = 0;
    uVar20 = 0;
    *(undefined8 *)(puVar16 + -0x1078) = 0;
    *(undefined8 *)(puVar16 + -0x1060) = 0;
    *(undefined8 *)(puVar16 + -0x1068) = 0;
    *(undefined1 **)(puVar16 + -0x1058) = puVar16 + -0x1018;
LAB_00103ea8:
    *(undefined8 *)(puVar16 + -0x1080) = 0x103ec2;
    lVar9 = FUN_00107770(*(undefined4 *)(puVar16 + -0x1050),*(long *)(puVar16 + -0x1058) + lVar21,
                         0x4000 - lVar21);
    if (lVar9 == 0) {
      uVar5 = 1;
    }
    else {
      if (lVar9 != -1) {
        *(long *)(puVar16 + -0x1028) = *(long *)(puVar16 + -0x1028) + lVar9;
        pcVar18 = *(char **)(puVar16 + -0x1058);
        lVar21 = lVar21 + lVar9;
        if (bVar22) goto LAB_00103f9c;
LAB_00103ef0:
        cVar4 = *pcVar18;
        puVar16[-0x104a] = cVar4;
        *(int *)(puVar16 + -0x1070) = (int)cVar4;
        *(undefined8 *)(puVar16 + -0x1080) = 0x103f03;
        cVar4 = FUN_00104bc0();
        if (cVar4 == '\0') goto LAB_00103f9c;
        *(undefined4 *)(puVar16 + -0x1034) = *(undefined4 *)(puVar16 + -0x1070);
        switch(puVar16[-0x104a]) {
        case 9:
          lVar9 = 1;
switchD_001040ee_caseD_9:
          *(ulong *)(puVar16 + -0x1078) = (*(ulong *)(puVar16 + -0x1078) & 0xfffffffffffffff8) + 8;
switchD_001040ee_caseD_b:
          do {
            *(ulong *)(puVar16 + -0x1068) = *(long *)(puVar16 + -0x1068) + uVar13;
            uVar13 = 0;
LAB_00103f80:
            while( true ) {
              do {
                pcVar18 = pcVar18 + lVar9;
                lVar21 = lVar21 - lVar9;
                uVar20 = uVar20 + 1;
                while( true ) {
                  if (lVar21 == 0) goto LAB_00103ea8;
                  if (!bVar22) goto LAB_00103ef0;
LAB_00103f9c:
                  uVar11 = *(undefined8 *)(puVar16 + -0x1020);
                  *(undefined8 *)(puVar16 + -0x1080) = 0x103fb4;
                  lVar9 = FUN_00104be0(puVar16 + -0x1034,pcVar18,lVar21,
                                       (mbstate_t *)(puVar16 + -0x1020));
                  if (lVar9 == -2) {
                    *(undefined8 *)(puVar16 + -0x1020) = uVar11;
                    if (lVar21 != 0) {
                      if (lVar21 == 0x4000) {
                        pcVar18 = pcVar18 + 1;
                        lVar21 = 0x3fff;
                      }
                      *(undefined8 *)(puVar16 + -0x1080) = 0x104142;
                      __memmove_chk(*(undefined8 *)(puVar16 + -0x1058),pcVar18,lVar21,0x4001);
                    }
                    bVar22 = true;
                    goto LAB_00103ea8;
                  }
                  if (lVar9 != -1) break;
                  pcVar18 = pcVar18 + 1;
                  lVar21 = lVar21 + -1;
                  bVar22 = true;
                }
                *(long *)(puVar16 + -0x1070) = lVar9;
                *(undefined8 *)(puVar16 + -0x1080) = 0x103fd5;
                iVar6 = mbsinit((mbstate_t *)(puVar16 + -0x1020));
                lVar9 = *(long *)(puVar16 + -0x1070);
                bVar22 = iVar6 == 0;
                if (lVar9 == 0) {
                  *(undefined4 *)(puVar16 + -0x1034) = 0;
                  __wc = 0;
                  lVar9 = 1;
                }
                else {
                  __wc = *(wint_t *)(puVar16 + -0x1034);
                  switch(__wc) {
                  case 9:
                    goto switchD_001040ee_caseD_9;
                  case 10:
                    goto switchD_001040ee_caseD_a;
                  case 0xb:
                    goto switchD_001040ee_caseD_b;
                  case 0xc:
                  case 0xd:
                    goto switchD_001040ee_caseD_c;
                  default:
                    break;
                  case 0x20:
                    goto switchD_001040ee_caseD_20;
                  }
                }
                *(long *)(puVar16 + -0x1070) = lVar9;
                *(undefined8 *)(puVar16 + -0x1080) = 0x104002;
                iVar6 = iswprint(__wc);
                lVar9 = *(long *)(puVar16 + -0x1070);
              } while (iVar6 == 0);
              if (DAT_0010e130 != '\0') {
                *(undefined8 *)(puVar16 + -0x1080) = 0x10429f;
                iVar6 = wcwidth(*(wchar_t *)(puVar16 + -0x1034));
                lVar9 = *(long *)(puVar16 + -0x1070);
                lVar17 = (long)iVar6 + *(long *)(puVar16 + -0x1078);
                if (iVar6 < 1) {
                  lVar17 = *(long *)(puVar16 + -0x1078);
                }
                *(long *)(puVar16 + -0x1078) = lVar17;
              }
              *(long *)(puVar16 + -0x1070) = lVar9;
              *(undefined8 *)(puVar16 + -0x1080) = 0x10402a;
              iVar6 = iswspace(*(wint_t *)(puVar16 + -0x1034));
              lVar9 = *(long *)(puVar16 + -0x1070);
              if (iVar6 != 0) break;
              *(undefined8 *)(puVar16 + -0x1080) = 0x10403c;
              iVar6 = FUN_001031f0(*(undefined4 *)(puVar16 + -0x1034));
              lVar9 = *(long *)(puVar16 + -0x1070);
              if (iVar6 != 0) break;
              uVar13 = 1;
            }
          } while( true );
        case 10:
          lVar9 = 1;
          goto switchD_001040ee_caseD_a;
        case 0xb:
switchD_00103f32_caseD_b:
          lVar9 = 1;
          goto switchD_001040ee_caseD_b;
        case 0xc:
        case 0xd:
          lVar9 = 1;
          goto switchD_001040ee_caseD_c;
        case 0x20:
          lVar9 = 1;
          goto switchD_001040ee_caseD_20;
        }
        puVar16[-0x104a] = cVar4;
        *(undefined4 *)(puVar16 + -0x1070) = *(undefined4 *)(puVar16 + -0x1070);
        *(undefined8 *)(puVar16 + -0x1080) = 0x103f45;
        ppuVar10 = __ctype_b_loc();
        *(undefined8 *)(puVar16 + -0x1080) = 0x103f51;
        bVar3 = FUN_001031e0(*(undefined4 *)(puVar16 + -0x1070));
        uVar1 = (*ppuVar10)[bVar3];
        if ((uVar1 & 0x4000) != 0) {
          *(long *)(puVar16 + -0x1078) = *(long *)(puVar16 + -0x1078) + 1;
          if ((uVar1 & 0x2000) != 0) goto switchD_00103f32_caseD_b;
          uVar13 = (ulong)(byte)puVar16[-0x104a];
        }
        lVar9 = 1;
        goto LAB_00103f80;
      }
      *(undefined8 *)(puVar16 + -0x1080) = 0x104310;
      uVar11 = FUN_00107210(0,3,*(undefined8 *)(puVar16 + -0x1040));
      *(undefined8 *)(puVar16 + -0x1080) = 0x104318;
      piVar12 = __errno_location();
      iVar6 = *piVar12;
      uVar5 = 0;
      *(undefined8 *)(puVar16 + -0x1080) = 0x104330;
      error(0,iVar6,"%s",uVar11);
    }
    uVar14 = *(ulong *)(puVar16 + -0x1028);
    uVar15 = *(ulong *)(puVar16 + -0x1078);
    if (*(ulong *)(puVar16 + -0x1078) <= *(ulong *)(puVar16 + -0x1060)) {
      uVar15 = *(ulong *)(puVar16 + -0x1060);
    }
    lVar21 = uVar13 + *(long *)(puVar16 + -0x1068);
  }
LAB_00103db8:
  if ((byte)puVar16[-0x1049] < DAT_0010e132) {
    uVar20 = uVar14;
  }
  *(undefined8 *)(puVar16 + -0x1080) = 0x103ddf;
  FUN_001035d0(*(undefined8 *)(puVar16 + -0x1030),lVar21,uVar20,uVar14,uVar15,
               *(undefined8 *)(puVar16 + -0x1048));
  DAT_0010e150 = DAT_0010e150 + lVar21;
  DAT_0010e158 = DAT_0010e158 + *(long *)(puVar16 + -0x1030);
  DAT_0010e148 = DAT_0010e148 + uVar20;
  DAT_0010e140 = DAT_0010e140 + *(long *)(puVar16 + -0x1028);
  if (DAT_0010e138 < uVar15) {
    DAT_0010e138 = uVar15;
  }
  if (*(long *)(puVar16 + 0x2ff0) == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar16 + -0x1080) = 0x104448;
  __stack_chk_fail();
switchD_001040ee_caseD_20:
  *(long *)(puVar16 + -0x1078) = *(long *)(puVar16 + -0x1078) + 1;
  goto switchD_001040ee_caseD_b;
switchD_001040ee_caseD_a:
  *(long *)(puVar16 + -0x1030) = *(long *)(puVar16 + -0x1030) + 1;
switchD_001040ee_caseD_c:
  uVar14 = *(ulong *)(puVar16 + -0x1078);
  *(undefined8 *)(puVar16 + -0x1078) = 0;
  if (uVar14 <= *(ulong *)(puVar16 + -0x1060)) {
    uVar14 = *(ulong *)(puVar16 + -0x1060);
  }
  *(ulong *)(puVar16 + -0x1060) = uVar14;
  goto switchD_001040ee_caseD_b;
}




// Function: wc_file @ 0x4500

ulong wc_file(char *param_1,undefined8 param_2)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  undefined8 uVar4;
  int *piVar5;
  
  if (param_1 != (char *)0x0) {
    iVar1 = strcmp(param_1,"-");
    if (iVar1 != 0) {
      iVar1 = open(param_1,0);
      if (iVar1 != -1) {
        uVar2 = FUN_00103bf0(iVar1,param_1,param_2,0);
        iVar1 = close(iVar1);
        if (iVar1 == 0) {
          return (ulong)uVar2;
        }
      }
      uVar4 = FUN_00107210(0,3,param_1);
      piVar5 = __errno_location();
      error(0,*piVar5,"%s",uVar4);
      return 0;
    }
  }
  DAT_0010e128 = 1;
  uVar3 = FUN_00103bf0(0,param_1,param_2,0xffffffffffffffff);
  return uVar3;
}




// Function: usage @ 0x45c0

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_0010e178;
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar3,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_001032c0();
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_001039b0();
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: wc_lines_avx2 @ 0x8a90

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong wc_lines_avx2(undefined8 param_1,undefined4 param_2,long *param_3,long *param_4)

{
  undefined1 auVar1 [32];
  ulong uVar2;
  undefined1 (*pauVar3) [32];
  undefined8 uVar4;
  int *piVar5;
  int iVar6;
  undefined1 (*pauVar7) [32];
  long lVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  bool bVar16;
  undefined8 unaff_R13;
  ulong uVar17;
  long lVar18;
  long in_FS_OFFSET;
  undefined1 auVar19 [32];
  undefined1 auVar20 [32];
  undefined1 auVar21 [64];
  undefined1 local_4020 [64];
  undefined1 auStack_3fe0 [16280];
  long local_48;
  
  local_48 = *(long *)(in_FS_OFFSET + 0x28);
  bVar16 = param_3 == (long *)0x0 || param_4 == (long *)0x0;
  uVar17 = CONCAT71((int7)((ulong)unaff_R13 >> 8),bVar16);
  if (bVar16) {
    uVar17 = 0;
  }
  else {
    lVar8 = 0;
    lVar18 = 0;
    while (uVar2 = FUN_00107770(param_2,(undefined1 (*) [32])local_4020,0x3fc0), uVar2 != 0) {
      if (uVar2 == 0xffffffffffffffff) {
        uVar4 = FUN_00107210(0,3,param_1);
        piVar5 = __errno_location();
        error(0,*piVar5,"%s",uVar4);
        goto LAB_00108c4b;
      }
      lVar8 = lVar8 + uVar2;
      if (uVar2 < 0x40) {
        auVar20 = SUB6432(ZEXT1664((undefined1  [16])0x0),0);
        pauVar3 = (undefined1 (*) [32])local_4020;
        auVar19 = auVar20;
      }
      else {
        auVar21 = ZEXT1664((undefined1  [16])0x0);
        auVar20 = auVar21._0_32_;
        pauVar7 = (undefined1 (*) [32])local_4020;
        do {
          auVar19 = vpcmpeqb_avx2(_DAT_0010a200,*pauVar7);
          auVar1 = vpcmpeqb_avx2(_DAT_0010a200,pauVar7[1]);
          pauVar7 = pauVar7 + 2;
          auVar20 = vpsubb_avx2(auVar20,auVar19);
          auVar19 = vpsubb_avx2(auVar21._0_32_,auVar1);
          auVar21 = ZEXT3264(auVar19);
          pauVar3 = (undefined1 (*) [32])(auStack_3fe0 + (uVar2 - 0x40 & 0xffffffffffffffc0));
        } while (pauVar7 !=
                 (undefined1 (*) [32])(auStack_3fe0 + (uVar2 - 0x40 & 0xffffffffffffffc0)));
      }
      auVar20 = vpsadbw_avx2(auVar20,SUB6432(ZEXT1664((undefined1  [16])0x0),0));
      auVar19 = vpsadbw_avx2(auVar19,SUB6432(ZEXT1664((undefined1  [16])0x0),0));
      iVar9 = vpextrw_avx(auVar20._0_16_,0);
      iVar6 = vpextrw_avx(auVar20._0_16_,4);
      iVar10 = vpextrw_avx(auVar20._16_16_,0);
      iVar13 = vpextrw_avx(auVar19._0_16_,0);
      iVar11 = vpextrw_avx(auVar20._16_16_,4);
      iVar12 = vpextrw_avx(auVar19._0_16_,4);
      iVar14 = vpextrw_avx(auVar19._16_16_,0);
      iVar15 = vpextrw_avx(auVar19._16_16_,4);
      lVar18 = lVar18 + (long)(iVar6 + iVar9 + iVar10 + iVar11) +
                        (long)(iVar12 + iVar13 + iVar14 + iVar15);
      for (; (undefined1 (*) [32])(*(undefined1 (*) [32])local_4020 + uVar2) != pauVar3;
          pauVar3 = (undefined1 (*) [32])(*pauVar3 + 1)) {
        lVar18 = lVar18 + (ulong)((*pauVar3)[0] == '\n');
      }
    }
    uVar17 = 1;
    *param_3 = lVar18;
    *param_4 = lVar8;
  }
LAB_00108c4b:
  if (local_48 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar17 & 0xffffffff;
}



