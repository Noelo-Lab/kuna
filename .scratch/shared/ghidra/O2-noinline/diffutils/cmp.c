// Function: main @ 0x2940

undefined8 main(int param_1,undefined8 *param_2)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  int *piVar12;
  undefined8 *puVar13;
  long in_FS_OFFSET;
  undefined1 auVar14 [16];
  undefined8 uStack_188;
  long *local_180 [2];
  long local_170;
  stat local_168;
  stat local_d8;
  long local_40;
  
  puVar13 = (undefined8 *)(long)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0010d020 = 2;
  FUN_00104180(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00103f90(0);
  FUN_001064f0();
  local_180[0] = &local_170;
  while (iVar2 = getopt_long(param_1,param_2,"bci:ln:sv",&PTR_s_print_bytes_0010ca40,0), iVar2 != -1
        ) {
    switch(iVar2) {
    case 0x62:
    case 99:
      DAT_0010d0b2 = 1;
      break;
    default:
      lVar7 = 0;
      optind = FUN_00103240(0);
      goto LAB_00102db4;
    case 0x69:
      FUN_001032d0(0,&optarg,0x3a);
      pcVar1 = optarg + 1;
      if (*optarg == ':') {
        optarg = pcVar1;
        FUN_001032d0(1,&optarg,0);
      }
      else {
        optarg = pcVar1;
        if (DAT_0010d0c8 < DAT_0010d0c0) {
          DAT_0010d0c8 = DAT_0010d0c0;
        }
      }
      break;
    case 0x6c:
      FUN_001032a0(1);
      break;
    case 0x6e:
      iVar2 = FUN_00106640(optarg,0,0,local_180[0]);
      if ((iVar2 != 0) || (local_170 < 0)) {
        FUN_00103240("invalid --bytes value \'%s\'",optarg);
        goto LAB_001030b0;
      }
      if ((DAT_0010d010 < 0) || (local_170 <= DAT_0010d010)) {
        DAT_0010d010 = local_170;
      }
      break;
    case 0x73:
      FUN_001032a0(3);
      break;
    case 0x76:
      uVar5 = FUN_00104490("David MacKenzie");
      uVar6 = FUN_00104520("Torbjorn Granlund",&DAT_00109682);
      FUN_00105f90(stdout,&DAT_001096b5,"GNU diffutils",PTR_DAT_0010d018,uVar6,uVar5,0,uVar6);
      FUN_00103d80();
      goto LAB_00102acb;
    case 0x80:
      FUN_00103370();
      FUN_00103d80();
      goto LAB_00102acb;
    }
  }
  if (optind != param_1) {
    lVar7 = (long)optind * 8;
    DAT_0010d230 = (char *)param_2[optind];
    DAT_0010d238 = "-";
    iVar2 = optind + 1;
    if (optind + 1 < param_1) {
LAB_00102db4:
      DAT_0010d238 = *(char **)((long)param_2 + lVar7 + 8);
      iVar2 = optind + 2;
    }
    optind = iVar2;
    lVar7 = (long)optind;
    if (param_1 <= optind) {
LAB_00102c1f:
      lVar7 = 0;
      puVar13 = &DAT_0010d230;
      do {
        local_180[0] = (long *)CONCAT44(local_180[0]._4_4_,(int)lVar7);
        if (((lVar7 != 0) && (DAT_0010d0c0 == DAT_0010d0c8)) &&
           (iVar2 = strcmp(DAT_0010d230,DAT_0010d238), iVar2 == 0)) goto LAB_00102acb;
        pcVar1 = (&DAT_0010d230)[lVar7];
        iVar2 = strcmp(pcVar1,"-");
        if (iVar2 != 0) {
          iVar2 = open(pcVar1,0,0);
          (&DAT_0010d220)[lVar7] = iVar2;
          if (-1 < iVar2) goto LAB_00102c7c;
          if (DAT_0010d0b4 == 3) {
                    /* WARNING: Subroutine does not return */
            exit(2);
          }
LAB_00102fd6:
          piVar12 = __errno_location();
          iVar2 = (int)local_180[0];
          error(2,*piVar12,&DAT_001095df,(&DAT_0010d230)[(int)local_180[0]]);
          goto LAB_00102ffa;
        }
        (&DAT_0010d220)[lVar7] = 0;
        iVar2 = 0;
LAB_00102c7c:
        iVar2 = fstat(iVar2,(stat *)(&DAT_0010d100 + lVar7 * 0x12));
        if (iVar2 != 0) goto LAB_00102fd6;
        if (lVar7 == 1) goto code_r0x00102ca0;
        lVar7 = 1;
      } while( true );
    }
    optind = optind + 1;
    local_170 = param_2[lVar7];
    FUN_001032d0(0,&local_170,0);
    lVar7 = (long)optind;
    if (param_1 <= optind) goto LAB_00102c1f;
    optind = optind + 1;
    local_170 = param_2[lVar7];
    FUN_001032d0(1,&local_170,0);
    lVar7 = (long)optind;
    if (param_1 <= optind) goto LAB_00102c1f;
    goto LAB_001030ee;
  }
LAB_001030dd:
  lVar7 = FUN_00103240("missing operand after \'%s\'",param_2[(long)puVar13 + -1]);
LAB_001030ee:
  auVar14 = FUN_00103240("extra operand \'%s\'",param_2[lVar7]);
  uVar5 = uStack_188;
  uStack_188 = auVar14._0_8_;
  (*(code *)PTR___libc_start_main_0010cfc0)
            (main,uVar5,local_180,0,0,auVar14._8_8_,&uStack_188);
  do {
                    /* WARNING: Do nothing block with infinite loop */
  } while( true );
code_r0x00102ca0:
  iVar2 = 0;
  if ((DAT_0010d108 == DAT_0010d198) && (DAT_0010d100 == DAT_0010d190)) {
LAB_00102dd8:
    if (((DAT_0010d118 == DAT_0010d1a8) && (DAT_0010d110 == DAT_0010d1a0)) &&
       ((DAT_0010d11c == DAT_0010d1ac &&
        ((((DAT_0010d120 == DAT_0010d1b0 && (DAT_0010d130 == DAT_0010d1c0)) &&
          (DAT_0010d158 == DAT_0010d1e8)) && (DAT_0010d168 == DAT_0010d1f8)))))) {
      lVar7 = FUN_001034c0(0);
      lVar11 = FUN_001034c0(1);
      if (lVar7 == lVar11) {
LAB_00102acb:
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return 0;
        }
LAB_001030b0:
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    }
LAB_00102cd7:
    if (DAT_0010d0b4 != 3) {
LAB_00102e9d:
      iVar3 = fstat(1,&local_168);
      if ((iVar3 != 0) || (iVar3 = stat("/dev/null",&local_d8), iVar3 != 0)) goto LAB_00102eb4;
      if ((local_168.st_ino == local_d8.st_ino) && (local_168.st_dev == local_d8.st_dev)) {
LAB_00102fbd:
        DAT_0010d0b4 = 2;
      }
      else if ((local_168.st_mode & 0xf000) == 0x6000) {
        if ((local_d8.st_mode & 0xf000) == 0x6000) goto LAB_00102faa;
      }
      else if (((local_168.st_mode & 0xf000) == 0x2000) && ((local_d8.st_mode & 0xf000) == 0x2000))
      {
LAB_00102faa:
        if (local_168.st_rdev == local_d8.st_rdev) goto LAB_00102fbd;
      }
LAB_00102eb4:
      if (DAT_0010d0b4 != 3) goto LAB_00102d0c;
    }
    if (((DAT_0010d118 & 0xf000) == 0x8000) && ((DAT_0010d1a8 & 0xf000) == 0x8000)) {
LAB_00102ffa:
      lVar7 = DAT_0010d130;
      lVar8 = FUN_001034c0(0);
      lVar11 = DAT_0010d1c0;
      lVar9 = FUN_001034c0(1);
      lVar10 = 0;
      if (-1 < lVar7 - lVar8) {
        lVar10 = lVar7 - lVar8;
      }
      lVar11 = lVar11 - lVar9;
      if (lVar11 < 0) {
        lVar11 = 0;
      }
      if (lVar11 != lVar10) {
        if (DAT_0010d010 < 0) {
LAB_0010305f:
                    /* WARNING: Subroutine does not return */
          exit(1);
        }
        if (lVar10 < lVar11) {
          lVar11 = lVar10;
        }
        if (lVar11 < DAT_0010d010) goto LAB_0010305f;
      }
    }
  }
  else {
    if ((DAT_0010d118 & 0xf000) != 0x6000) {
      if ((DAT_0010d118 & 0xf000) == 0x2000) {
        if ((DAT_0010d1a8 & 0xf000) == 0x2000) goto LAB_00102f2c;
        goto LAB_00102e90;
      }
      goto LAB_00102cd7;
    }
    if ((DAT_0010d1a8 & 0xf000) == 0x6000) {
LAB_00102f2c:
      if (DAT_0010d128 == DAT_0010d1b8) goto LAB_00102dd8;
      goto LAB_00102cd7;
    }
LAB_00102e90:
    if (DAT_0010d0b4 != 3) goto LAB_00102e9d;
  }
LAB_00102d0c:
  DAT_0010d0d0 = FUN_00106ac0(DAT_0010d138,DAT_0010d1c8,0x7ffffffffffffff7);
  param_2 = (undefined8 *)(DAT_0010d0d0 + 0xfU & 0xfffffffffffffff8);
  DAT_0010d0e0 = FUN_00106100((DAT_0010d0d0 + 0xfU >> 3) << 4);
  DAT_0010d0e8 = DAT_0010d0e0 + (long)param_2;
  iVar3 = FUN_001035e0();
  iVar4 = close(DAT_0010d220);
  if (iVar4 == 0) {
    iVar2 = close(DAT_0010d224);
    if (iVar2 == 0) {
      if ((iVar3 != 0) && (DAT_0010d0b4 < 2)) {
        FUN_00103d80();
      }
                    /* WARNING: Subroutine does not return */
      exit(iVar3);
    }
    iVar2 = 1;
  }
  piVar12 = __errno_location();
  error(2,*piVar12,&DAT_001095df,(&DAT_0010d230)[iVar2]);
  goto LAB_001030dd;
}




// Function: block_compare @ 0x31f0

long block_compare(long *param_1,long *param_2)

{
  long *plVar1;
  
  plVar1 = param_1;
  if (*param_1 == *param_2) {
    do {
      plVar1 = plVar1 + 1;
      param_2 = param_2 + 1;
    } while (*plVar1 == *param_2);
    if ((char)*plVar1 == (char)*param_2) goto LAB_00103230;
  }
  else {
    for (; (char)*plVar1 == (char)*param_2; plVar1 = (long *)((long)plVar1 + 1)) {
LAB_00103230:
      param_2 = (long *)((long)param_2 + 1);
    }
  }
  return (long)plVar1 - (long)param_1;
}




// Function: try_help @ 0x3240

void try_help(long param_1,undefined8 param_2)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char extraout_DL;
  undefined8 *puVar5;
  int iVar6;
  long in_FS_OFFSET;
  long lStack_48;
  
  if (param_1 != 0) {
    uVar3 = dcgettext(0,param_1,5);
    error(0,0,uVar3,param_2);
  }
  uVar3 = DAT_0011d280;
  uVar4 = dcgettext(0,"Try \'%s --help\' for more information.",5);
  iVar2 = 2;
  error(2,0,uVar4,uVar3);
  if ((DAT_0010d0b4 == 0) || (DAT_0010d0b4 == iVar2)) {
    DAT_0010d0b4 = iVar2;
    return;
  }
  puVar5 = (undefined8 *)0x0;
  iVar6 = 0x109030;
  try_help();
  uVar3 = *puVar5;
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00106640(uVar3);
  if (((iVar2 == 0) || ((iVar2 == 2 && (*(char *)*puVar5 == extraout_DL)))) && (-1 < lStack_48)) {
    if ((long)(&DAT_0010d0c0)[iVar6] < lStack_48) {
      (&DAT_0010d0c0)[iVar6] = lStack_48;
    }
    if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    try_help("invalid --ignore-initial value \'%s\'",uVar3);
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: specify_comparison_type @ 0x32a0

void specify_comparison_type(int param_1)

{
  undefined8 uVar1;
  long lVar2;
  int iVar3;
  char extraout_DL;
  undefined8 *puVar4;
  int iVar5;
  long in_FS_OFFSET;
  long lStack_40;
  
  if ((DAT_0010d0b4 == 0) || (DAT_0010d0b4 == param_1)) {
    DAT_0010d0b4 = param_1;
    return;
  }
  puVar4 = (undefined8 *)0x0;
  iVar5 = 0x109030;
  FUN_00103240();
  uVar1 = *puVar4;
  lVar2 = *(long *)(in_FS_OFFSET + 0x28);
  iVar3 = FUN_00106640(uVar1);
  if (((iVar3 == 0) || ((iVar3 == 2 && (*(char *)*puVar4 == extraout_DL)))) && (-1 < lStack_40)) {
    if ((long)(&DAT_0010d0c0)[iVar5] < lStack_40) {
      (&DAT_0010d0c0)[iVar5] = lStack_40;
    }
    if (lVar2 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    FUN_00103240("invalid --ignore-initial value \'%s\'",uVar1);
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: specify_ignore_initial @ 0x32d0

void specify_ignore_initial(int param_1,undefined8 *param_2,char param_3)

{
  undefined8 uVar1;
  int iVar2;
  long in_FS_OFFSET;
  long local_38;
  long local_30;
  
  uVar1 = *param_2;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00106640(uVar1,param_2,0,&local_38,"kKMGTPEZY0");
  if (((iVar2 == 0) || ((iVar2 == 2 && (*(char *)*param_2 == param_3)))) && (-1 < local_38)) {
    if ((long)(&DAT_0010d0c0)[param_1] < local_38) {
      (&DAT_0010d0c0)[param_1] = local_38;
    }
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    FUN_00103240("invalid --ignore-initial value \'%s\'",uVar1);
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0x3370

void usage(void)

{
  FILE *__stream;
  undefined8 uVar1;
  undefined8 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  undefined **ppuVar5;
  
  uVar2 = DAT_0011d280;
  ppuVar5 = &PTR_s__b____print_bytes_print_differin_0010c9e0;
  uVar1 = dcgettext(0,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5);
  __printf_chk(1,uVar1,uVar2);
  uVar2 = dcgettext(0,"Compare two files byte by byte.",5);
  __printf_chk(1,&DAT_001095db,uVar2);
  uVar2 = dcgettext(0,
                    "The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."
                    ,5);
  __printf_chk(1,&DAT_001095c7,uVar2);
  __stream = stdout;
  pcVar3 = (char *)dcgettext(0,
                             "Mandatory arguments to long options are mandatory for short options too.\n"
                             ,5);
  fputs_unlocked(pcVar3,__stream);
  pcVar3 = "-b, --print-bytes          print differing bytes";
  do {
    ppuVar5 = ppuVar5 + 1;
    uVar2 = dcgettext(0,pcVar3,5);
    __printf_chk(1,"  %s\n",uVar2);
    pcVar3 = *ppuVar5;
  } while (pcVar3 != (char *)0x0);
  uVar2 = dcgettext(0,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
  uVar1 = dcgettext(0,"If a FILE is \'-\' or missing, read standard input.",5);
  uVar4 = dcgettext(0,
                    "SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."
                    ,5);
  __printf_chk(1,&DAT_001095d3,uVar4,uVar1,uVar2);
  FUN_00106040();
  return;
}




// Function: file_position @ 0x34c0

__off_t file_position(int param_1)

{
  __off_t _Var1;
  long lVar2;
  
  lVar2 = (long)param_1;
  if ((&DAT_0010d0b0)[lVar2] != '\0') {
    return *(__off_t *)(&DAT_0010d0a0 + lVar2 * 8);
  }
  (&DAT_0010d0b0)[lVar2] = 1;
  _Var1 = lseek((&DAT_0010d220)[lVar2],(&DAT_0010d0c0)[lVar2],1);
  *(__off_t *)(&DAT_0010d0a0 + lVar2 * 8) = _Var1;
  return _Var1;
}




// Function: count_newlines @ 0x3510

long count_newlines(undefined1 *param_1,long param_2)

{
  undefined1 *puVar1;
  undefined1 uVar2;
  long lVar3;
  
  lVar3 = 0;
  puVar1 = param_1 + param_2;
  uVar2 = *puVar1;
  *puVar1 = 10;
  while( true ) {
    param_1 = rawmemchr(param_1,10);
    if (puVar1 == param_1) break;
    lVar3 = lVar3 + 1;
    param_1 = param_1 + 1;
  }
  *puVar1 = uVar2;
  return lVar3;
}




// Function: sprintc @ 0x3550

void sprintc(byte *param_1,byte param_2)

{
  ushort **ppuVar1;
  
  ppuVar1 = __ctype_b_loc();
  if ((*(byte *)((long)*ppuVar1 + (ulong)param_2 * 2 + 1) & 0x40) == 0) {
    if ((char)param_2 < '\0') {
      param_2 = param_2 + 0x80;
      param_1[0] = 0x4d;
      param_1[1] = 0x2d;
      param_1 = param_1 + 2;
    }
    if (param_2 < 0x20) {
      *param_1 = 0x5e;
      param_2 = param_2 + 0x40;
      param_1 = param_1 + 1;
    }
    else if (param_2 == 0x7f) {
      *param_1 = 0x5e;
      param_2 = 0x3f;
      param_1 = param_1 + 1;
    }
  }
  *param_1 = param_2;
  param_1[1] = 0;
  return;
}




// Function: cmp @ 0x35e0

bool cmp(void)

{
  byte bVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  void *__s1;
  void *__s2;
  byte bVar4;
  char cVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  ulong uVar9;
  ulong uVar10;
  ulong uVar11;
  undefined8 uVar12;
  long lVar13;
  ulong uVar14;
  char *pcVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  int *piVar20;
  ulong extraout_RDX;
  ulong uVar21;
  ulong uVar22;
  ulong __n;
  long lVar23;
  long in_FS_OFFSET;
  bool bVar24;
  ulong local_e0;
  int local_d8;
  int local_cc;
  long local_c8;
  bool local_a1;
  undefined1 local_82 [5];
  undefined1 local_7d [5];
  undefined1 local_78 [32];
  undefined1 local_58 [24];
  long local_40;
  
  lVar23 = DAT_0010d130;
  __s2 = DAT_0010d0e8;
  __s1 = DAT_0010d0e0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e0 = DAT_0010d010;
  if (DAT_0010d0b4 == 1) {
    uVar21 = 0x7fffffffffffffff;
    if (-1 < (long)DAT_0010d010) {
      uVar21 = DAT_0010d010;
    }
    if ((DAT_0010d118 & 0xf000) == 0x8000) {
      lVar13 = FUN_001034c0(0);
      if (lVar23 - lVar13 < (long)uVar21) {
        uVar21 = lVar23 - lVar13;
      }
    }
    lVar23 = DAT_0010d1c0;
    if ((DAT_0010d1a8 & 0xf000) == 0x8000) {
      lVar13 = FUN_001034c0(1);
      if (lVar23 - lVar13 < (long)uVar21) {
        uVar21 = lVar23 - lVar13;
      }
    }
    local_cc = 1;
    while (uVar21 = (long)uVar21 / 10, uVar21 != 0) {
      local_cc = local_cc + 1;
    }
  }
  lVar23 = 0;
  do {
    uVar21 = (&DAT_0010d0c0)[lVar23];
    if ((uVar21 != 0) && (lVar13 = FUN_001034c0(lVar23), lVar13 == -1)) {
      do {
        uVar11 = DAT_0010d0d0;
        if (uVar21 <= DAT_0010d0d0) {
          uVar11 = uVar21;
        }
        uVar14 = FUN_00106a10((&DAT_0010d220)[lVar23],__s1,uVar11);
        if (uVar11 != uVar14) {
          if (uVar14 == 0xffffffffffffffff) {
            piVar20 = __errno_location();
            uVar11 = (&DAT_0010d230)[(int)lVar23];
            error(2,*piVar20,&DAT_001095df);
            goto LAB_00103a01;
          }
          break;
        }
        uVar21 = uVar21 - uVar11;
      } while (uVar21 != 0);
    }
    if (lVar23 == 1) goto LAB_00103658;
    lVar23 = 1;
  } while( true );
LAB_00103a01:
  uVar12 = FUN_001040e0(uVar21,local_78);
  uVar17 = FUN_001040e0(local_c8,local_58);
  if (DAT_0010d0b2 == '\0') {
    pcVar15 = (char *)dcgettext(0,"%s %s differ: byte %s, line %s\n",5);
    if ((pcVar15 == "%s %s differ: byte %s, line %s\n") &&
       (cVar5 = FUN_00104050(5,"%s %s differ: byte %s, line %s\n"), cVar5 == '\0')) {
      pcVar15 = "%s %s differ: char %s, line %s\n";
    }
    __printf_chk(1,pcVar15,DAT_0010d230,DAT_0010d238,uVar12,uVar17);
  }
  else {
    uVar2 = *(undefined1 *)((long)__s1 + uVar11);
    uVar3 = *(undefined1 *)((long)__s2 + uVar11);
    FUN_00103550(local_82,uVar2);
    FUN_00103550(local_7d,uVar3);
    uVar19 = DAT_0010d238;
    uVar18 = DAT_0010d230;
    uVar16 = dcgettext(0,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5);
    __printf_chk(1,uVar16,uVar18,uVar19,uVar12,uVar17,uVar2,local_82,uVar3,local_7d);
  }
LAB_00103945:
  bVar24 = true;
  goto LAB_0010394a;
LAB_00103658:
  local_a1 = true;
  local_d8 = 0;
  local_c8 = 1;
  uVar14 = 1;
  uVar9 = DAT_0010d0d0;
  do {
    uVar21 = uVar9;
    if (-1 < (long)local_e0) {
      if (local_e0 <= uVar9) {
        uVar21 = local_e0;
      }
      local_e0 = local_e0 - uVar21;
    }
    uVar9 = FUN_00106a10(DAT_0010d220,__s1,uVar21);
    if (uVar9 == 0xffffffffffffffff) {
LAB_00103d53:
      piVar20 = __errno_location();
      error(2,*piVar20,&DAT_001095df,DAT_0010d230);
      goto LAB_00103d74;
    }
    uVar10 = FUN_00106a10(DAT_0010d224,__s2,uVar21);
    if (uVar10 == 0xffffffffffffffff) {
      piVar20 = __errno_location();
      error(2,*piVar20,&DAT_001095df,DAT_0010d238);
      goto LAB_00103d53;
    }
    __n = uVar10;
    if (uVar9 <= uVar10) {
      __n = uVar9;
    }
    iVar6 = memcmp(__s1,__s2,__n);
    if (iVar6 == 0) {
      uVar21 = uVar14 + __n;
      if ((DAT_0010d0b4 == 0) && (uVar11 = __n, __n != 0)) goto LAB_00103905;
    }
    else {
      if ((uVar9 < uVar10) || (*(byte *)((long)__s2 + uVar9) = 0x55, uVar9 == uVar10)) {
        *(byte *)((long)__s1 + uVar10) = 0x79;
        bVar4 = ~*(byte *)((long)__s2 + uVar9);
      }
      else {
        bVar4 = 0xaa;
      }
      *(byte *)((long)__s1 + uVar9) = bVar4;
      *(byte *)((long)__s2 + uVar10) = ~*(byte *)((long)__s1 + uVar10);
      uVar7 = ~(uint)uVar9 & 7;
      if (uVar7 != 0) {
        uVar8 = 0;
        do {
          uVar21 = (ulong)uVar8;
          uVar8 = uVar8 + 1;
          *(undefined1 *)((long)__s1 + uVar21 + uVar9 + 1) = 0;
        } while (uVar8 < uVar7);
      }
      uVar7 = ~(uint)uVar10 & 7;
      if (uVar7 != 0) {
        uVar8 = 0;
        do {
          uVar21 = (ulong)uVar8;
          uVar8 = uVar8 + 1;
          *(undefined1 *)((long)__s2 + uVar21 + uVar10 + 1) = 0;
        } while (uVar8 < uVar7);
      }
      uVar11 = FUN_001031f0(__s1,__s2);
      uVar21 = uVar14 + uVar11;
      if ((uVar11 == 0) || (DAT_0010d0b4 != 0)) {
        if (uVar11 < __n) {
          if (DAT_0010d0b4 == 2) {
            local_d8 = 1;
            if (uVar9 != uVar10) goto LAB_00103945;
            break;
          }
          if (DAT_0010d0b4 < 3) {
            if (DAT_0010d0b4 == 0) goto LAB_00103a01;
            uVar11 = uVar14;
            do {
              while( true ) {
                uVar22 = uVar21;
                bVar4 = *(byte *)((long)__s1 + (uVar22 - uVar14));
                bVar1 = *(byte *)((long)__s2 + (uVar22 - uVar14));
                uVar21 = (ulong)bVar1;
                if (bVar4 != bVar1) break;
LAB_001037fa:
                uVar21 = uVar22 + 1;
                if (uVar14 + __n == uVar22 + 1) goto LAB_001038b0;
              }
              uVar12 = FUN_001040e0(uVar22,local_58,uVar11);
              if (DAT_0010d0b2 == '\0') {
                __printf_chk(1,"%*s %3o %3o\n",local_cc,uVar12,bVar4,uVar21);
                uVar11 = extraout_RDX;
                goto LAB_001037fa;
              }
              FUN_00103550(local_7d,bVar4);
              FUN_00103550(local_78,bVar1);
              uVar11 = uVar21;
              __printf_chk(1,"%*s %3o %-4s %3o %s\n",local_cc,uVar12,bVar4,local_7d,uVar21,local_78)
              ;
              uVar21 = uVar22 + 1;
            } while (uVar14 + __n != uVar22 + 1);
LAB_001038b0:
            uVar21 = uVar22 + 1;
            local_d8 = -1;
          }
          else if (DAT_0010d0b4 == 3) goto LAB_00103945;
        }
      }
      else {
LAB_00103905:
        lVar23 = FUN_00103510(__s1,uVar11);
        local_c8 = local_c8 + lVar23;
        if (uVar11 < __n) goto LAB_00103a01;
        local_a1 = *(char *)((long)__s1 + (uVar11 - 1)) == '\n';
      }
    }
    if (uVar9 != uVar10) {
      if (DAT_0010d0b4 == 3) goto LAB_00103945;
      uVar12 = (&DAT_0010d230)[uVar10 < uVar9];
      if (uVar21 == 1) {
        uVar17 = dcgettext(0,"cmp: EOF on %s which is empty\n",5);
        __fprintf_chk(stderr,1,uVar17,uVar12);
        bVar24 = true;
        goto LAB_0010394a;
      }
      uVar17 = FUN_001040e0(uVar21 - 1,local_78);
      if (DAT_0010d0b4 == 0) {
        uVar18 = FUN_001040e0(local_c8 - (ulong)local_a1,local_58);
        if (local_a1 == false) {
          uVar19 = dcgettext(0,"cmp: EOF on %s after byte %s, in line %s\n",5);
        }
        else {
          uVar19 = dcgettext(0,"cmp: EOF on %s after byte %s, line %s\n",5);
        }
        __fprintf_chk(stderr,1,uVar19,uVar12,uVar17,uVar18);
      }
      else {
        uVar18 = dcgettext(0,"cmp: EOF on %s after byte %s\n",5);
        __fprintf_chk(stderr,1,uVar18,uVar12,uVar17);
      }
      goto LAB_00103945;
    }
    uVar14 = uVar21;
  } while (DAT_0010d0d0 == uVar9);
  bVar24 = local_d8 != 0;
LAB_0010394a:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar24;
  }
LAB_00103d74:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_stdout @ 0x3d80

void check_stdout(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 == 0) {
    iVar1 = fclose(stdout);
    if (iVar1 == 0) {
      return;
    }
  }
  else {
    uVar2 = dcgettext(0,"write failed",5);
    error(2,0,&DAT_001095df,uVar2);
  }
  uVar2 = dcgettext(0,"standard output",5);
  piVar3 = __errno_location();
  error(2,*piVar3,&DAT_001095df,uVar2);
  return;
}



