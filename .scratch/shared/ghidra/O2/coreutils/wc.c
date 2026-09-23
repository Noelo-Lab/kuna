// Function: main @ 0x2860

byte main(uint param_1,undefined8 *param_2)

{
  bool bVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  char *pcVar5;
  char *__s1;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  ulong uVar9;
  long lVar10;
  int *piVar11;
  long lVar12;
  ulong uVar13;
  uint *__stream;
  ulong uVar14;
  char *pcVar15;
  stat *__buf;
  undefined **ppuVar16;
  long in_FS_OFFSET;
  double dVar17;
  double dVar18;
  double local_230;
  int *local_220;
  char local_20d;
  int local_1fc;
  ulong local_1f8;
  undefined8 *local_1f0;
  stat local_d8;
  long local_40;
  
  ppuVar16 = &PTR_s_bytes_0010ca20;
  pcVar15 = "clLmw";
  uVar9 = (ulong)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00104d10(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils");
  __stream = &switchD_001029c4::switchdataD_0010932c;
  textdomain("coreutils");
  FUN_001088e0(FUN_001046f0);
  iVar4 = getpagesize();
  DAT_0010d120 = (long)iVar4;
  setvbuf(stdout,(char *)0x0,1,0);
  pcVar5 = getenv("POSIXLY_CORRECT");
  DAT_0010d131 = 0;
  DAT_0010d132 = 0;
  DAT_0010d118._0_1_ = pcVar5 != (char *)0x0;
  DAT_0010d133 = 0;
  DAT_0010d134 = 0;
  DAT_0010d130 = 0;
  DAT_0010d138 = 0;
  DAT_0010d140 = 0;
  DAT_0010d148 = 0;
  DAT_0010d150 = 0;
  DAT_0010d158 = 0;
  pcVar5 = (char *)0x0;
  while( true ) {
    uVar6 = 0;
    iVar4 = getopt_long(param_1,param_2,"clLmw",&PTR_s_bytes_0010ca20);
    if (iVar4 == -1) break;
    if (0x81 < iVar4) goto switchD_001029c4_caseD_4d;
    if (iVar4 < 0x4c) {
      if (iVar4 == -0x83) {
        FUN_00107a40(stdout,&DAT_0010905f,"GNU coreutils",PTR_DAT_0010d018,"Paul Rubin",
                     "David MacKenzie",0,uVar6);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar4 != -0x82) goto switchD_001029c4_caseD_4d;
      FUN_00104230(0);
      break;
    }
    switch(iVar4) {
    case 0x4c:
      DAT_0010d130 = 1;
      break;
    default:
      goto switchD_001029c4_caseD_4d;
    case 99:
      DAT_0010d131 = 1;
      break;
    case 0x6c:
      DAT_0010d134 = 1;
      break;
    case 0x6d:
      DAT_0010d132 = 1;
      break;
    case 0x77:
      DAT_0010d133 = 1;
      break;
    case 0x80:
      DAT_0010d160 = 1;
      break;
    case 0x81:
      pcVar5 = optarg;
    }
  }
  if ((((DAT_0010d134 == 0) && (DAT_0010d133 == 0)) && (DAT_0010d132 == 0)) &&
     ((DAT_0010d131 == 0 && (DAT_0010d130 == 0)))) {
    DAT_0010d131 = 1;
    DAT_0010d133 = 1;
    DAT_0010d134 = 1;
  }
  iVar4 = optind;
  if (pcVar5 == (char *)0x0) goto LAB_00102fe2;
  if (optind < (int)param_1) {
    pcVar5 = (char *)FUN_001069f0(4,param_2[optind]);
    uVar6 = dcgettext(0,"extra operand %s",5);
    error(0,0,uVar6,pcVar5);
    uVar6 = dcgettext(0,"file operands cannot be combined with --files0-from",5);
    __fprintf_chk(stderr,1,"%s\n",uVar6);
switchD_001029c4_caseD_4d:
    FUN_00104230(1);
LAB_001032a3:
    dVar18 = (double)FUN_00104bd0();
    dVar18 = DAT_001099c0 * dVar18;
LAB_001031a9:
    if (dVar18 < local_230) goto LAB_00102ad5;
    FUN_001071f0(&local_1f8);
    local_20d = FUN_001072a0(__stream,&local_1f8);
    if ((local_20d == '\0') || (iVar4 = FUN_001047e0(__stream), iVar4 != 0)) {
      uVar6 = FUN_001069f0(4,pcVar5);
      uVar7 = dcgettext(0,"cannot read file names from %s",5);
      error(1,0,uVar7,uVar6);
      goto LAB_0010332c;
    }
    lVar12 = FUN_00104580(local_1f0);
    if (lVar12 == 0) goto LAB_0010332c;
    uVar9 = 1;
    if (local_1f8 != 0) {
      uVar9 = local_1f8;
    }
    local_220 = (int *)FUN_00107d50(uVar9,0x98);
    uVar9 = local_1f8;
    param_2 = local_1f0;
    if (local_1f8 != 0) goto LAB_00103024;
  }
  else {
    iVar4 = strcmp(pcVar5,"-");
    __stream = (uint *)stdin;
    if ((iVar4 != 0) && (__stream = (uint *)fopen(pcVar5,"r"), (FILE *)__stream == (FILE *)0x0)) {
      __s1 = (char *)FUN_001069f0(4,pcVar5);
      pcVar5 = (char *)dcgettext(0,"cannot open %s for reading",5);
      piVar11 = __errno_location();
      error(1,*piVar11,pcVar5,__s1);
      lVar12 = 0;
      goto LAB_00102d08;
    }
    iVar4 = fileno((FILE *)__stream);
    iVar4 = fstat(iVar4,&local_d8);
    if ((iVar4 == 0) && ((local_d8.st_mode & 0xf000) == 0x8000)) {
      local_230 = (double)local_d8.st_size;
      dVar17 = (double)FUN_00104bd0();
      dVar18 = DAT_001099b8;
      if (dVar17 * DAT_001099c0 <= DAT_001099b8) goto LAB_001032a3;
      goto LAB_001031a9;
    }
LAB_00102ad5:
    lVar12 = FUN_001045b0(__stream);
    if (lVar12 == 0) goto LAB_0010332c;
    local_220 = (int *)FUN_00107d50(1,0x98);
    local_20d = '\0';
  }
  uVar9 = 0;
  *local_220 = 1;
  DAT_0010d12c = 1;
LAB_00102b14:
  ppuVar16 = (undefined **)&local_1fc;
  pcVar15 = "-";
  local_230 = 0.0;
  bVar3 = 1;
  while (__s1 = (char *)FUN_001045f0(lVar12,ppuVar16), __s1 != (char *)0x0) {
    if (pcVar5 == (char *)0x0) {
      if (*__s1 == '\0') {
        dcgettext(0,"invalid zero-length file name",5);
        error(0,0,"%s");
LAB_00102de1:
        bVar3 = 0;
      }
      else {
LAB_00102c10:
        dVar18 = 0.0;
        if (uVar9 != 0) {
          dVar18 = local_230;
        }
LAB_00102c1b:
        iVar4 = strcmp(__s1,pcVar15);
        piVar11 = (int *)((long)dVar18 + (long)local_220);
        if (iVar4 == 0) {
          DAT_0010d128 = '\x01';
          bVar2 = FUN_00103820(0,__s1,(int *)((long)dVar18 + (long)local_220),0xffffffffffffffff);
        }
        else {
LAB_00102b6c:
          iVar4 = open(__s1,0);
          if (iVar4 != -1) {
            bVar2 = FUN_00103820(iVar4,__s1,piVar11,0);
            iVar4 = close(iVar4);
            if (iVar4 == 0) goto LAB_00102bb5;
          }
          FUN_00106d70(0,3,__s1);
          piVar11 = __errno_location();
          error(0,*piVar11,"%s");
          bVar2 = 0;
        }
LAB_00102bb5:
        bVar3 = bVar3 & bVar2;
      }
    }
    else {
      iVar4 = strcmp(pcVar5,pcVar15);
      if (iVar4 == 0) {
        iVar4 = strcmp(__s1,pcVar15);
        if (iVar4 == 0) {
LAB_00102d08:
          FUN_001069f0(4,__s1);
          uVar6 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",5);
          error(0,0,uVar6);
          if (*__s1 != '\0') goto LAB_00102de1;
        }
        else if (*__s1 != '\0') {
          dVar18 = local_230;
          piVar11 = local_220;
          if (uVar9 != 0) goto LAB_00102c1b;
          goto LAB_00102b6c;
        }
      }
      else if (*__s1 != '\0') goto LAB_00102c10;
      uVar6 = FUN_00104690(lVar12);
      uVar7 = dcgettext(0,"invalid zero-length file name",5);
      uVar8 = FUN_00106d70(0,3,pcVar5);
      error(0,0,"%s:%lu: %s",uVar8,uVar6,uVar7);
      bVar3 = 0;
    }
    if (uVar9 == 0) {
      *local_220 = 1;
    }
    local_230 = (double)((long)local_230 + 0x98);
  }
  if (local_1fc != 3) {
    if (local_1fc == 4) {
      param_2 = (undefined8 *)FUN_00106d70(0,3,pcVar5);
      pcVar5 = (char *)dcgettext(0,"%s: read error",5);
      piVar11 = __errno_location();
      error(0,*piVar11,pcVar5,param_2);
      bVar3 = 0;
    }
    else {
      if (local_1fc != 2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main");
      }
      bVar2 = pcVar5 == (char *)0x0 & bVar3;
      uVar9 = (ulong)bVar2;
      param_2 = (undefined8 *)0x0;
      if ((bVar2 != 0) && (lVar10 = FUN_00104690(lVar12), bVar3 = bVar2, lVar10 == 0)) {
        DAT_0010d128 = '\x01';
        bVar3 = FUN_00103820(0,0,local_220,0xffffffffffffffff);
      }
    }
    param_1 = (uint)uVar9;
    if (local_20d != '\0') {
      FUN_00107270(&local_1f8);
    }
    uVar9 = FUN_00104690(lVar12);
    if (1 < uVar9) {
      uVar6 = dcgettext(0,"total",5);
      FUN_00103430(DAT_0010d158,DAT_0010d150,DAT_0010d148,DAT_0010d140,DAT_0010d138,uVar6);
    }
    FUN_001046b0(lVar12);
    free(local_220);
    if ((DAT_0010d128 == '\0') || (iVar4 = close(0), iVar4 == 0)) {
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return bVar3 ^ 1;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    piVar11 = __errno_location();
    iVar4 = error(1,*piVar11,&DAT_00109a52);
LAB_00102fe2:
    if (iVar4 < (int)param_1) {
      param_2 = param_2 + iVar4;
      uVar9 = (ulong)(int)(param_1 - iVar4);
    }
    else {
      uVar9 = 1;
      param_2 = &DAT_0010d110;
    }
    lVar12 = FUN_00104580(param_2);
    if (lVar12 == 0) goto LAB_0010332c;
    local_220 = (int *)FUN_00107d50(uVar9,0x98);
    local_20d = '\0';
LAB_00103024:
    if ((uVar9 == 1) &&
       ((uint)DAT_0010d134 + (uint)DAT_0010d133 + (uint)DAT_0010d132 + (uint)DAT_0010d131 +
        (uint)DAT_0010d130 == 1)) {
      *local_220 = 1;
      DAT_0010d12c = 1;
    }
    else {
      uVar14 = 0;
      __buf = (stat *)(local_220 + 2);
      do {
        pcVar15 = (char *)param_2[uVar14];
        if ((pcVar15 == (char *)0x0) || (iVar4 = strcmp(pcVar15,"-"), iVar4 == 0)) {
          iVar4 = fstat(0,__buf);
        }
        else {
          iVar4 = stat(pcVar15,__buf);
        }
        uVar14 = uVar14 + 1;
        *(int *)(__buf[-1].__unused + 2) = iVar4;
        __buf = (stat *)&__buf[1].st_ino;
      } while (uVar14 < uVar9);
      DAT_0010d12c = 1;
      if (*local_220 < 1) {
        iVar4 = 1;
        uVar14 = 0;
        uVar13 = 0;
        piVar11 = local_220;
        do {
          if (*piVar11 == 0) {
            if ((piVar11[8] & 0xf000U) == 0x8000) {
              uVar14 = uVar14 + *(long *)(piVar11 + 0xe);
            }
            else {
              iVar4 = 7;
            }
          }
          uVar13 = uVar13 + 1;
          piVar11 = piVar11 + 0x26;
        } while (uVar13 < uVar9);
        DAT_0010d12c = 1;
        if (9 < uVar14) {
          do {
            DAT_0010d12c = DAT_0010d12c + 1;
            bVar1 = 99 < uVar14;
            uVar14 = uVar14 / 10;
          } while (bVar1);
        }
        if (DAT_0010d12c <= iVar4) {
          DAT_0010d12c = iVar4;
        }
      }
    }
    goto LAB_00102b14;
  }
LAB_0010332c:
                    /* WARNING: Subroutine does not return */
  FUN_00108110();
}




// Function: write_counts @ 0x3430

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
  
  puVar3 = &DAT_00109405;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0010d134 != '\0') {
    uVar2 = FUN_00104940(param_1,auStack_58);
    puVar3 = &DAT_00109404;
    __printf_chk(1,&DAT_00109405,DAT_0010d12c,uVar2);
  }
  puVar4 = puVar3;
  if (DAT_0010d133 != '\0') {
    uVar2 = FUN_00104940(param_2,auStack_58);
    puVar4 = &DAT_00109404;
    __printf_chk(1,puVar3,DAT_0010d12c,uVar2);
  }
  puVar3 = puVar4;
  if (DAT_0010d132 != '\0') {
    uVar2 = FUN_00104940(param_3,auStack_58);
    puVar3 = &DAT_00109404;
    __printf_chk(1,puVar4,DAT_0010d12c,uVar2);
  }
  puVar4 = puVar3;
  if (DAT_0010d131 != '\0') {
    uVar2 = FUN_00104940(param_4,auStack_58);
    puVar4 = &DAT_00109404;
    __printf_chk(1,puVar3,DAT_0010d12c,uVar2);
  }
  if (DAT_0010d130 != '\0') {
    uVar2 = FUN_00104940(param_5,auStack_58);
    __printf_chk(1,puVar4,DAT_0010d12c,uVar2);
  }
  if (param_6 != (char *)0x0) {
    pcVar1 = strchr(param_6,10);
    if (pcVar1 != (char *)0x0) {
      param_6 = (char *)FUN_00106d70(0,3,param_6);
    }
    __printf_chk(1," %s",param_6);
  }
  pcVar1 = stdout->_IO_write_ptr;
  if (pcVar1 < stdout->_IO_write_end) {
    stdout->_IO_write_ptr = pcVar1 + 1;
    *pcVar1 = '\n';
  }
  else {
    __overflow(stdout,10);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wc @ 0x3820

/* WARNING: Removing unreachable block (ram,0x00104119) */
/* WARNING: Removing unreachable block (ram,0x0010410b) */
/* WARNING: Removing unreachable block (ram,0x001040ff) */
/* WARNING: Removing unreachable block (ram,0x001040f4) */

undefined4 wc(undefined4 param_1,long param_2,int *param_3,long param_4)

{
  undefined1 uVar1;
  uint *puVar2;
  wint_t wVar3;
  int iVar4;
  undefined4 uVar5;
  size_t sVar6;
  __off_t _Var7;
  long lVar8;
  ushort **ppuVar9;
  undefined8 uVar10;
  int *piVar11;
  ulong uVar12;
  ulong uVar13;
  undefined1 *puVar14;
  undefined1 *puVar15;
  ulong uVar16;
  char *pcVar17;
  byte bVar18;
  long lVar19;
  ulong uVar20;
  byte *pbVar21;
  ulong uVar22;
  uint uVar23;
  undefined1 *puVar24;
  long in_FS_OFFSET;
  char cVar25;
  undefined1 local_4030 [16384];
  
  puVar14 = &stack0xffffffffffffffd0;
  do {
    puVar15 = puVar14;
    *(undefined8 *)(puVar15 + -0x1000) = *(undefined8 *)(puVar15 + -0x1000);
    puVar14 = puVar15 + -0x1000;
  } while (puVar15 + -0x1000 != local_4030);
  *(undefined4 *)(puVar15 + -0x1050) = param_1;
  *(long *)(puVar15 + -0x1048) = param_2;
  *(undefined8 *)(puVar15 + 0x2ff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
  *(long *)(puVar15 + -0x1040) = param_2;
  if (param_2 == 0) {
    *(undefined8 *)(puVar15 + -0x1080) = 0x103f58;
    uVar10 = dcgettext(0,"standard input",5);
    *(undefined8 *)(puVar15 + -0x1040) = uVar10;
  }
  *(undefined8 *)(puVar15 + -0x1028) = 0;
  *(undefined8 *)(puVar15 + -0x1030) = 0;
  *(undefined8 *)(puVar15 + -0x1080) = 0x10388e;
  sVar6 = __ctype_get_mb_cur_max();
  bVar18 = DAT_0010d131;
  if (sVar6 < 2) {
    puVar15[-0x1049] = 0;
    if (DAT_0010d131 == 0) {
      bVar18 = DAT_0010d132;
    }
  }
  else {
    puVar15[-0x1049] = DAT_0010d132;
  }
  if ((DAT_0010d133 == '\0') && (DAT_0010d130 == '\0')) {
    if ((bVar18 == 1) && (puVar15[-0x1049] == '\0')) {
      if (DAT_0010d134 != '\0') {
        *(undefined8 *)(puVar15 + -0x1080) = 0x1040e0;
        posix_fadvise(*(int *)(puVar15 + -0x1050),0,0,2);
LAB_001040e0:
        if (DAT_0010d134 != '\0') {
LAB_001040f0:
          piVar11 = (int *)cpuid_basic_info(0);
          uVar23 = piVar11[3];
          if (*piVar11 == 0) {
LAB_00104165:
            if (DAT_0010d160 != '\0') {
              pcVar17 = "failed to get cpuid";
LAB_0010417a:
              *(undefined8 *)(puVar15 + -0x1080) = 0x104181;
              uVar10 = dcgettext(0,pcVar17,5,uVar23);
              *(undefined8 *)(puVar15 + -0x1080) = 0x104196;
              error(0,0,"%s",uVar10);
            }
          }
          else {
            lVar19 = cpuid_Version_info(1);
            uVar23 = 0;
            if ((*(uint *)(lVar19 + 0xc) & 0x8000000) == 0) {
LAB_00104120:
              pcVar17 = "avx2 support not detected";
              if (DAT_0010d160 != '\0') goto LAB_0010417a;
            }
            else {
              puVar2 = (uint *)cpuid_basic_info(0);
              uVar23 = puVar2[3];
              if (*puVar2 < 7) goto LAB_00104165;
              lVar19 = cpuid_Extended_Feature_Enumeration_info(7);
              uVar23 = *(uint *)(lVar19 + 0xc);
              if ((*(uint *)(lVar19 + 4) & 0x20) == 0) goto LAB_00104120;
              if (DAT_0010d160 != '\0') {
                *(undefined8 *)(puVar15 + -0x1080) = 0x1041c4;
                uVar10 = dcgettext(0,"using avx2 hardware support",5,uVar23);
                *(undefined8 *)(puVar15 + -0x1080) = 0x1041d9;
                error(0,0,"%s",uVar10);
              }
              PTR_FUN_0010d010 = FUN_001086a0;
            }
          }
          uVar16 = 0;
          uVar20 = 0;
          lVar19 = 0;
          *(undefined8 *)(puVar15 + -0x1080) = 0x104158;
          uVar5 = (*(code *)PTR_FUN_0010d010)
                            (*(undefined8 *)(puVar15 + -0x1040),*(undefined4 *)(puVar15 + -0x1050),
                             puVar15 + -0x1030,puVar15 + -0x1028);
          uVar13 = *(ulong *)(puVar15 + -0x1028);
          goto LAB_001039f8;
        }
      }
      iVar4 = *param_3;
      if (0 < iVar4) {
        *(undefined8 *)(puVar15 + -0x1080) = 0x10406a;
        iVar4 = fstat(*(int *)(puVar15 + -0x1050),(stat *)(param_3 + 2));
        *param_3 = iVar4;
      }
      uVar20 = DAT_0010d120;
      if (((iVar4 == 0) && ((param_3[8] & 0xd000U) == 0x8000)) &&
         (uVar13 = *(ulong *)(param_3 + 0xe), -1 < (long)uVar13)) {
        if (param_4 == -1) {
          *(ulong *)(puVar15 + -0x1078) = uVar13;
          *(undefined8 *)(puVar15 + -0x1080) = 0x104094;
          uVar12 = lseek(*(int *)(puVar15 + -0x1050),0,1);
          uVar16 = *(ulong *)(puVar15 + -0x1078);
          if (uVar16 % uVar20 != 0) {
            if (uVar16 < uVar12) {
              uVar13 = 0;
            }
            else {
              uVar13 = uVar16 - uVar12;
            }
            goto LAB_001040b7;
          }
          bVar18 = (byte)~(byte)(uVar12 >> 0x38) >> 7;
        }
        else {
          if (uVar13 % DAT_0010d120 != 0) {
LAB_001040b7:
            *(ulong *)(puVar15 + -0x1028) = uVar13;
            uVar16 = 0;
            uVar20 = 0;
            lVar19 = 0;
            uVar5 = 1;
            goto LAB_001039f8;
          }
          bVar18 = 1;
          uVar12 = 0;
        }
        uVar20 = 0x201;
        if (*(long *)(param_3 + 0x10) - 1U < 0x2000000000000000) {
          uVar20 = *(long *)(param_3 + 0x10) + 1;
        }
        lVar19 = uVar13 - uVar13 % uVar20;
        if (((long)uVar12 < lVar19) && (bVar18 != 0)) {
          *(undefined8 *)(puVar15 + -0x1080) = 0x103994;
          _Var7 = lseek(*(int *)(puVar15 + -0x1050),lVar19,1);
          if (-1 < _Var7) {
            *(ulong *)(puVar15 + -0x1028) = lVar19 - uVar12;
          }
        }
      }
      iVar4 = *(int *)(puVar15 + -0x1050);
      *(undefined8 *)(puVar15 + -0x1080) = 0x1039ba;
      posix_fadvise(iVar4,0,0,2);
      while( true ) {
        *(undefined8 *)(puVar15 + -0x1080) = 0x1039de;
        lVar19 = FUN_00107490(iVar4,puVar15 + -0x1018,0x4000);
        if (lVar19 == 0) break;
        if (lVar19 == -1) {
          uVar16 = 0;
          lVar19 = 0;
          *(undefined8 *)(puVar15 + -0x1080) = 0x104033;
          uVar10 = FUN_00106d70(0,3,*(undefined8 *)(puVar15 + -0x1040));
          *(undefined8 *)(puVar15 + -0x1080) = 0x10403b;
          piVar11 = __errno_location();
          iVar4 = *piVar11;
          uVar20 = 0;
          *(undefined8 *)(puVar15 + -0x1080) = 0x104053;
          error(0,iVar4,"%s",uVar10);
          uVar13 = *(ulong *)(puVar15 + -0x1028);
          uVar5 = 0;
          goto LAB_001039f8;
        }
        *(long *)(puVar15 + -0x1028) = *(long *)(puVar15 + -0x1028) + lVar19;
      }
      uVar13 = *(ulong *)(puVar15 + -0x1028);
      uVar16 = 0;
      lVar19 = 0;
      uVar5 = 1;
      uVar20 = 0;
      goto LAB_001039f8;
    }
    *(undefined8 *)(puVar15 + -0x1080) = 0x104204;
    posix_fadvise(*(int *)(puVar15 + -0x1050),0,0,2);
    if (puVar15[-0x1049] != '\x01') {
      if (bVar18 != 0) goto LAB_001040e0;
      if (puVar15[-0x1049] == '\0') goto LAB_001040f0;
    }
  }
  else {
    *(undefined8 *)(puVar15 + -0x1080) = 0x103a9a;
    posix_fadvise(*(int *)(puVar15 + -0x1050),0,0,2);
  }
  *(undefined8 *)(puVar15 + -0x1080) = 0x103a9f;
  sVar6 = __ctype_get_mb_cur_max();
  if (sVar6 < 2) {
    uVar16 = 0;
    uVar22 = 0;
    *(undefined1 **)(puVar15 + -0x1058) = puVar15 + -0x1018;
    uVar12 = 0;
    uVar20 = uVar16;
    while( true ) {
      *(undefined8 *)(puVar15 + -0x1080) = 0x103e1b;
      lVar19 = FUN_00107490(*(undefined4 *)(puVar15 + -0x1050),*(undefined8 *)(puVar15 + -0x1058),
                            0x4000);
      if (lVar19 == 0) break;
      if (lVar19 == -1) {
        uVar5 = 0;
        *(undefined8 *)(puVar15 + -0x1080) = 0x103fff;
        uVar10 = FUN_00106d70(0,3,*(undefined8 *)(puVar15 + -0x1040));
        *(undefined8 *)(puVar15 + -0x1080) = 0x104007;
        piVar11 = __errno_location();
        iVar4 = *piVar11;
        *(undefined8 *)(puVar15 + -0x1080) = 0x10401c;
        error(0,iVar4,"%s",uVar10);
        goto LAB_00103fc8;
      }
      *(long *)(puVar15 + -0x1028) = *(long *)(puVar15 + -0x1028) + lVar19;
      puVar24 = *(undefined1 **)(puVar15 + -0x1058);
      puVar14 = puVar24 + lVar19;
      do {
        uVar1 = *puVar24;
        puVar24 = puVar24 + 1;
        switch(uVar1) {
        case 9:
          uVar22 = (uVar22 & 0xfffffffffffffff8) + 8;
          break;
        case 10:
          *(long *)(puVar15 + -0x1030) = *(long *)(puVar15 + -0x1030) + 1;
        case 0xc:
        case 0xd:
          if (uVar16 < uVar22) {
            uVar16 = uVar22;
          }
          uVar22 = 0;
          break;
        case 0xb:
          break;
        default:
          *(undefined1 **)(puVar15 + -0x1070) = puVar14;
          puVar15[-0x1078] = uVar1;
          *(undefined8 *)(puVar15 + -0x1080) = 0x103e6e;
          ppuVar9 = __ctype_b_loc();
          puVar14 = *(undefined1 **)(puVar15 + -0x1070);
          bVar18 = puVar15[-0x1078];
          if (((*ppuVar9)[bVar18] & 0x4000) != 0) {
            uVar22 = uVar22 + 1;
            if (((*ppuVar9)[bVar18] & 0x2000) != 0) break;
            *(undefined1 **)(puVar15 + -0x1078) = puVar14;
            *(undefined8 *)(puVar15 + -0x1080) = 0x103e9b;
            wVar3 = btowc((uint)bVar18);
            puVar14 = *(undefined1 **)(puVar15 + -0x1078);
            if ((byte)DAT_0010d118 == 0) {
              if (((wVar3 == 0xa0) || (wVar3 == 0x2007)) || ((wVar3 == 0x202f || (wVar3 == 0x2060)))
                 ) break;
              uVar12 = 1;
            }
            else {
              uVar12 = (ulong)(byte)DAT_0010d118;
            }
          }
          goto LAB_00103ed0;
        case 0x20:
          uVar22 = uVar22 + 1;
        }
        uVar20 = uVar20 + uVar12;
        uVar12 = 0;
LAB_00103ed0:
      } while (puVar14 != puVar24);
    }
    uVar5 = 1;
LAB_00103fc8:
    uVar13 = *(ulong *)(puVar15 + -0x1028);
    if (uVar16 < uVar22) {
      uVar16 = uVar22;
    }
    lVar19 = uVar12 + uVar20;
    uVar20 = 0;
  }
  else {
    lVar19 = 0;
    cVar25 = '\0';
    uVar16 = 0;
    *(undefined8 *)(puVar15 + -0x1020) = 0;
    uVar12 = 0;
    uVar20 = 0;
    *(undefined8 *)(puVar15 + -0x1060) = 0;
    *(undefined8 *)(puVar15 + -0x1070) = 0;
    *(undefined1 **)(puVar15 + -0x1058) = puVar15 + -0x1018;
LAB_00103ae0:
    *(undefined8 *)(puVar15 + -0x1080) = 0x103afa;
    lVar8 = FUN_00107490(*(undefined4 *)(puVar15 + -0x1050),*(long *)(puVar15 + -0x1058) + lVar19,
                         0x4000 - lVar19);
    if (lVar8 != 0) {
      if (lVar8 == -1) {
        *(undefined8 *)(puVar15 + -0x1080) = 0x103f94;
        uVar10 = FUN_00106d70(0,3,*(undefined8 *)(puVar15 + -0x1040));
        *(undefined8 *)(puVar15 + -0x1080) = 0x103f9c;
        piVar11 = __errno_location();
        iVar4 = *piVar11;
        uVar5 = 0;
        *(undefined8 *)(puVar15 + -0x1080) = 0x103fb4;
        error(0,iVar4,"%s",uVar10);
        goto LAB_00103f68;
      }
      *(long *)(puVar15 + -0x1028) = *(long *)(puVar15 + -0x1028) + lVar8;
      lVar19 = lVar19 + lVar8;
      pbVar21 = *(byte **)(puVar15 + -0x1058);
      do {
        if (cVar25 != '\0') {
LAB_00103c30:
          *(undefined1 **)(puVar15 + -0x1078) = puVar15 + -0x1020;
          *(undefined8 *)(puVar15 + -0x1068) = *(undefined8 *)(puVar15 + -0x1020);
          *(undefined8 *)(puVar15 + -0x1080) = 0x103c56;
          lVar8 = FUN_00104a50(puVar15 + -0x1034,pbVar21,lVar19,puVar15 + -0x1020);
          if (lVar8 != -2) {
            if (lVar8 != -1) {
              *(undefined8 *)(puVar15 + -0x1080) = 0x103c79;
              iVar4 = mbsinit(*(mbstate_t **)(puVar15 + -0x1078));
              cVar25 = iVar4 == 0;
              if (lVar8 == 0) {
                *(undefined4 *)(puVar15 + -0x1034) = 0;
                wVar3 = 0;
                lVar8 = 1;
              }
              else {
                wVar3 = *(wint_t *)(puVar15 + -0x1034);
                switch(wVar3) {
                case 9:
                  goto switchD_00103d3e_caseD_9;
                case 10:
                  goto switchD_00103d3e_caseD_a;
                case 0xb:
                  goto switchD_00103d3e_caseD_b;
                case 0xc:
                case 0xd:
                  goto switchD_00103d3e_caseD_c;
                default:
                  break;
                case 0x20:
                  goto switchD_00103d3e_caseD_20;
                }
              }
              puVar15[-0x1078] = cVar25;
              *(undefined8 *)(puVar15 + -0x1080) = 0x103ca1;
              iVar4 = iswprint(wVar3);
              cVar25 = puVar15[-0x1078];
              if (iVar4 != 0) {
                if (DAT_0010d130 != '\0') {
                  *(undefined8 *)(puVar15 + -0x1080) = 0x103f2f;
                  iVar4 = wcwidth(*(wchar_t *)(puVar15 + -0x1034));
                  cVar25 = puVar15[-0x1078];
                  if (0 < iVar4) {
                    uVar16 = (long)iVar4 + uVar16;
                  }
                }
                puVar15[-0x1078] = cVar25;
                *(undefined8 *)(puVar15 + -0x1080) = 0x103cc8;
                iVar4 = iswspace(*(wint_t *)(puVar15 + -0x1034));
                cVar25 = puVar15[-0x1078];
                if (iVar4 != 0) goto switchD_00103d3e_caseD_b;
                if ((byte)DAT_0010d118 == 0) {
                  iVar4 = *(int *)(puVar15 + -0x1034);
                  if ((((iVar4 == 0xa0) || (iVar4 == 0x2007)) || (iVar4 == 0x202f)) ||
                     (iVar4 == 0x2060)) goto switchD_00103d3e_caseD_b;
                  uVar12 = 1;
                }
                else {
                  uVar12 = (ulong)(byte)DAT_0010d118;
                }
              }
              goto LAB_00103ba8;
            }
            pbVar21 = pbVar21 + 1;
            lVar8 = -1;
            cVar25 = '\x01';
            goto LAB_00103bb2;
          }
          *(undefined8 *)(puVar15 + -0x1020) = *(undefined8 *)(puVar15 + -0x1068);
          if (lVar19 != 0) {
            if (lVar19 == 0x4000) {
              pbVar21 = pbVar21 + 1;
              lVar19 = 0x3fff;
            }
            *(undefined8 *)(puVar15 + -0x1080) = 0x103d9a;
            __memmove_chk(*(undefined8 *)(puVar15 + -0x1058),pbVar21,lVar19,0x4001);
          }
          cVar25 = '\x01';
          break;
        }
        bVar18 = *pbVar21;
        uVar23 = *(uint *)(&DAT_001099e0 + (ulong)(bVar18 >> 5) * 4) >> (bVar18 & 0x1f) & 1;
        if (uVar23 == 0) goto LAB_00103c30;
        *(int *)(puVar15 + -0x1034) = (int)(char)bVar18;
        switch(bVar18) {
        case 9:
          lVar8 = 1;
switchD_00103d3e_caseD_9:
          uVar16 = (uVar16 & 0xfffffffffffffff8) + 8;
          break;
        case 10:
          lVar8 = 1;
switchD_00103d3e_caseD_a:
          *(long *)(puVar15 + -0x1030) = *(long *)(puVar15 + -0x1030) + 1;
          goto switchD_00103d3e_caseD_c;
        case 0xb:
switchD_00103b6e_caseD_b:
          lVar8 = 1;
          break;
        case 0xc:
        case 0xd:
          lVar8 = 1;
switchD_00103d3e_caseD_c:
          if (uVar16 <= *(ulong *)(puVar15 + -0x1060)) {
            uVar16 = *(ulong *)(puVar15 + -0x1060);
          }
          *(ulong *)(puVar15 + -0x1060) = uVar16;
          uVar16 = 0;
          break;
        default:
          puVar15[-0x1068] = 0;
          puVar15[-0x1078] = bVar18;
          *(undefined8 *)(puVar15 + -0x1080) = 0x103b85;
          ppuVar9 = __ctype_b_loc();
          cVar25 = puVar15[-0x1068];
          if (((*ppuVar9)[(byte)puVar15[-0x1078]] & 0x4000) == 0) {
            lVar8 = 1;
          }
          else {
            uVar16 = uVar16 + 1;
            if (((*ppuVar9)[(byte)puVar15[-0x1078]] & 0x2000) != 0) goto switchD_00103b6e_caseD_b;
            uVar12 = (ulong)uVar23;
            lVar8 = 1;
          }
          goto LAB_00103ba8;
        case 0x20:
          lVar8 = 1;
switchD_00103d3e_caseD_20:
          uVar16 = uVar16 + 1;
        }
switchD_00103d3e_caseD_b:
        *(ulong *)(puVar15 + -0x1070) = *(long *)(puVar15 + -0x1070) + uVar12;
        uVar12 = 0;
LAB_00103ba8:
        pbVar21 = pbVar21 + lVar8;
        lVar8 = -lVar8;
        uVar20 = uVar20 + 1;
LAB_00103bb2:
        lVar19 = lVar19 + lVar8;
      } while (lVar19 != 0);
      goto LAB_00103ae0;
    }
    uVar5 = 1;
LAB_00103f68:
    uVar13 = *(ulong *)(puVar15 + -0x1028);
    if (uVar16 <= *(ulong *)(puVar15 + -0x1060)) {
      uVar16 = *(ulong *)(puVar15 + -0x1060);
    }
    lVar19 = uVar12 + *(long *)(puVar15 + -0x1070);
  }
LAB_001039f8:
  if ((byte)puVar15[-0x1049] < DAT_0010d132) {
    uVar20 = uVar13;
  }
  *(undefined8 *)(puVar15 + -0x1080) = 0x103a1f;
  FUN_00103430(*(undefined8 *)(puVar15 + -0x1030),lVar19,uVar20,uVar13,uVar16,
               *(undefined8 *)(puVar15 + -0x1048));
  DAT_0010d150 = DAT_0010d150 + lVar19;
  DAT_0010d158 = DAT_0010d158 + *(long *)(puVar15 + -0x1030);
  DAT_0010d148 = DAT_0010d148 + uVar20;
  DAT_0010d140 = DAT_0010d140 + *(long *)(puVar15 + -0x1028);
  if (DAT_0010d138 < uVar16) {
    DAT_0010d138 = uVar16;
  }
  if (*(long *)(puVar15 + 0x2ff0) != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)(puVar15 + -0x1080) = &UNK_0010422d;
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: usage @ 0x4230

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined **ppuVar6;
  undefined *puVar7;
  undefined *puVar8;
  long in_FS_OFFSET;
  undefined *local_b8;
  char *local_b0;
  char *local_a8 [4];
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_40;
  
  uVar5 = DAT_0010d178;
  ppuVar6 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar3,uVar5,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"\nWith no FILE, or when FILE is -, read standard input.\n",5);
    fputs_unlocked(pcVar4,pFVar1);
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
    local_b8 = &DAT_0010905d;
    pcVar4 = "[";
    local_b0 = "test invocation";
    local_a8[0] = "coreutils";
    local_a8[1] = "Multi-call invocation";
    local_88 = "sha256sum";
    local_a8[2] = "sha224sum";
    local_78 = "sha384sum";
    local_a8[3] = "sha2 utilities";
    local_80 = "sha2 utilities";
    local_70 = "sha2 utilities";
    local_68 = "sha512sum";
    local_60 = "sha2 utilities";
    local_58 = 0;
    local_50 = 0;
    do {
      iVar2 = strcmp("wc",pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar6 + 0x10);
      ppuVar6 = (undefined **)((long)ppuVar6 + 0x10);
    } while (pcVar4 != (char *)0x0);
    puVar8 = *(undefined **)((long)ppuVar6 + 8);
    if (puVar8 == (undefined *)0x0) {
      puVar8 = &DAT_0010905f;
    }
    uVar5 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar5,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar4 = setlocale(5,(char *)0x0);
    if (pcVar4 != (char *)0x0) {
      iVar2 = strncmp(pcVar4,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar4 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar4,pFVar1);
      }
    }
    puVar7 = &DAT_0010905f;
    iVar2 = strcmp("wc","[");
    if (iVar2 == 0) {
      puVar7 = &DAT_00109062;
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar4 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",puVar7);
    if (puVar8 != &DAT_0010905f) {
      pcVar4 = "";
    }
    uVar5 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar5,puVar8,pcVar4);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar5);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: wc_lines_avx2 @ 0x86a0

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
    while (uVar2 = FUN_00107490(param_2,(undefined1 (*) [32])local_4020,0x3fc0), uVar2 != 0) {
      if (uVar2 == 0xffffffffffffffff) {
        uVar4 = FUN_00106d70(0,3,param_1);
        piVar5 = __errno_location();
        error(0,*piVar5,"%s",uVar4);
        goto LAB_0010885b;
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
          auVar19 = vpcmpeqb_avx2(_DAT_0010a1c0,*pauVar7);
          auVar1 = vpcmpeqb_avx2(_DAT_0010a1c0,pauVar7[1]);
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
LAB_0010885b:
  if (local_48 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar17 & 0xffffffff;
}



