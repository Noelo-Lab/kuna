// Function: main @ 0x3cd0

byte main(uint param_1,undefined8 *param_2)

{
  bool bVar1;
  uint uVar2;
  bool bVar3;
  byte bVar4;
  undefined4 uVar5;
  int iVar6;
  uint uVar7;
  char *pcVar8;
  int *piVar9;
  long lVar10;
  char *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  int *piVar15;
  undefined8 *puVar16;
  undefined **__s1;
  char *pcVar17;
  bool bVar18;
  long in_FS_OFFSET;
  undefined **local_80;
  uint local_70;
  undefined4 local_64;
  int local_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  undefined8 local_50;
  long local_40;
  
  bVar18 = true;
  pcVar17 = "0abd:chHklmst:xB:DLPSX:";
  piVar15 = (int *)(ulong)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar11 = (char *)&local_64;
  puVar16 = param_2;
  local_58 = FUN_00105630();
  local_50 = 0;
  FUN_0010e310(*puVar16);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  __s1 = &PTR_DAT_0011c6e0;
  textdomain("coreutils");
  FUN_001137d0(FUN_00106020);
  DAT_0011d140 = FUN_001068c0();
  pcVar8 = getenv("DU_BLOCK_SIZE");
  FUN_0010d070(pcVar8,&DAT_0011d170,&DAT_0011d148);
  bVar1 = false;
  uVar2 = 0x10;
  local_70 = 8;
  local_80 = (undefined **)0x0;
  bVar3 = false;
LAB_00103dc0:
  do {
    local_64 = 0xffffffff;
    puVar16 = param_2;
    pcVar8 = (char *)getopt_long(param_1,param_2,"0abd:chHklmst:xB:DLPSX:",&PTR_DAT_0011c6e0,pcVar11
                                );
    iVar6 = (int)pcVar8;
    if (iVar6 == -1) {
      if (bVar18 == false) goto LAB_00104893;
      if (DAT_0011d186 == '\0') {
        if (bVar3) {
          if (bVar1) {
            if (DAT_0011d010 != (char *)0x0) {
LAB_001043b7:
              pcVar8 = DAT_0011d010;
              uVar12 = dcgettext(0,"warning: summarizing conflicts with --max-depth=%lu",5);
              error(0,0,uVar12,pcVar8);
              FUN_001058b0(1);
              goto LAB_001043e8;
            }
            uVar12 = dcgettext(0,"warning: summarizing is the same as using --max-depth=0",5);
            pcVar8 = (char *)error(0,0,uVar12);
            if (DAT_0011d010 != (char *)0x0) goto LAB_001043b7;
LAB_001045ed:
            DAT_0011d010 = (char *)0x0;
          }
        }
        else if (bVar1) goto LAB_001045ed;
      }
      else if (bVar1) goto LAB_00104872;
      if (DAT_0011d16d != '\0') {
        if (DAT_0011d185 != '\0') {
          uVar12 = dcgettext(0,
                             "warning: options --apparent-size and -b are ineffective with --inodes"
                             ,5);
          pcVar8 = (char *)error(0,0,uVar12);
        }
        DAT_0011d148 = 1;
      }
      if (DAT_0011d16c == '\0') goto LAB_00104226;
      pcVar11 = DAT_0011d160;
      if ((undefined **)DAT_0011d160 != (undefined **)0x0) goto LAB_00104210;
      __s1 = (undefined **)getenv("TIME_STYLE");
      pcVar8 = (char *)0x0;
      if (__s1 != (undefined **)0x0) {
        DAT_0011d160 = (char *)__s1;
        uVar7 = strcmp((char *)__s1,"locale");
        pcVar8 = (char *)(ulong)uVar7;
        if (uVar7 != 0) {
LAB_0010475f:
          if (*(char *)__s1 != '+') break;
          pcVar8 = strchr((char *)__s1,10);
          if (pcVar8 == (char *)0x0) goto LAB_0010421b;
          *pcVar8 = '\0';
          pcVar11 = (char *)__s1;
          goto LAB_00104210;
        }
      }
      pcVar11 = "long-iso";
      DAT_0011d160 = "long-iso";
      goto LAB_00104697;
    }
    if (0x87 < iVar6) {
switchD_00103e06_caseD_31:
      bVar18 = false;
      goto LAB_00103dc0;
    }
    if (iVar6 < 0x30) {
      if (iVar6 == -0x83) {
        FUN_00111510(stdout,&DAT_00115241,"GNU coreutils",PTR_DAT_0011d018,"Torbjorn Granlund",
                     "David MacKenzie","Paul Eggert","Jim Meyering",0,puVar16);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar6 == -0x82) {
        FUN_001058b0(0);
        goto LAB_0010475f;
      }
      goto switchD_00103e06_caseD_31;
    }
    switch(iVar6) {
    case 0x30:
      DAT_0011d182 = 1;
      break;
    default:
      goto switchD_00103e06_caseD_31;
    case 0x44:
    case 0x48:
      uVar2 = 0x11;
      break;
    case 0x4c:
      uVar2 = 2;
      break;
    case 0x50:
      uVar2 = 0x10;
      break;
    case 0x53:
      DAT_0011d180 = 1;
      break;
    case 0x58:
      iVar6 = FUN_00106fe0(FUN_00106b50,DAT_0011d140,optarg,0x10000000,10);
      if (iVar6 != 0) {
        uVar12 = FUN_00110680(0,3,optarg);
        piVar9 = __errno_location();
        bVar18 = false;
        error(0,*piVar9,&DAT_0011576f,uVar12);
      }
      break;
    case 0x61:
      DAT_0011d186 = '\x01';
      break;
    case 0x62:
      DAT_0011d185 = '\x01';
      DAT_0011d170 = 0;
      DAT_0011d148 = 1;
      break;
    case 99:
      DAT_0011d181 = '\x01';
      break;
    case 100:
      iVar6 = FUN_001120a0(optarg,0,0,&local_60,"");
      if (iVar6 == 0) {
        DAT_0011d010 = (char *)CONCAT44(uStack_5c,local_60);
        bVar3 = true;
      }
      else {
        uVar12 = FUN_00110840(optarg);
        uVar13 = dcgettext(0,"invalid maximum depth %s",5);
        bVar18 = false;
        error(0,0,uVar13,uVar12);
      }
      break;
    case 0x68:
      DAT_0011d170 = 0xb0;
      DAT_0011d148 = 1;
      break;
    case 0x6b:
      DAT_0011d170 = 0;
      DAT_0011d148 = 0x400;
      break;
    case 0x6c:
      DAT_0011d184 = '\x01';
      break;
    case 0x6d:
      DAT_0011d170 = 0;
      DAT_0011d148 = 0x100000;
      break;
    case 0x73:
      bVar1 = true;
      break;
    case 0x74:
      iVar6 = FUN_00111b80(optarg,0,0,&DAT_0011d178,"kKmMGTPEZY0");
      if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00112020(iVar6,local_64,0x74,&PTR_DAT_0011c6e0,optarg);
      }
      if ((DAT_0011d178 != 0) || (*(char *)optarg != '-')) break;
      uVar12 = dcgettext(0,"invalid --threshold argument \'-0\'",5);
      error(1,0,uVar12);
    case 0x42:
      iVar6 = FUN_0010d070(optarg,&DAT_0011d170,&DAT_0011d148);
      if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00112020(iVar6,local_64,0x42,&PTR_DAT_0011c6e0,optarg);
      }
      break;
    case 0x78:
      local_70 = 0x48;
      break;
    case 0x80:
      DAT_0011d185 = '\x01';
      break;
    case 0x81:
      FUN_00106b50(DAT_0011d140,optarg,0x10000000);
      break;
    case 0x82:
      local_80 = optarg;
      break;
    case 0x83:
      DAT_0011d170 = 0x90;
      DAT_0011d148 = 1;
      break;
    case 0x85:
      DAT_0011d16c = '\x01';
      uVar5 = 0;
      if (optarg != (undefined **)0x0) {
        lVar10 = FUN_00105dc0("--time",optarg,&PTR_s_atime_0011c6a0,&DAT_00115700,4,PTR_FUN_0011d020
                              ,1);
        uVar5 = *(undefined4 *)(&DAT_00115700 + lVar10 * 4);
      }
      DAT_0011d168 = uVar5;
      pcVar8 = getenv("TZ");
      DAT_0011d150 = FUN_001109c0(pcVar8);
      break;
    case 0x86:
      DAT_0011d160 = (char *)optarg;
      break;
    case 0x87:
      DAT_0011d16d = '\x01';
    }
  } while( true );
  while( true ) {
    pcVar11 = DAT_0011d160;
    uVar7 = strncmp(DAT_0011d160,"posix-",6);
    pcVar8 = (char *)(ulong)uVar7;
    if (uVar7 != 0) break;
    DAT_0011d160 = (char *)((long)pcVar11 + 6);
  }
LAB_00104210:
  __s1 = (undefined **)pcVar11;
  if (*pcVar11 == '+') {
LAB_0010421b:
    DAT_0011d158 = (char *)((long)__s1 + 1);
  }
  else {
LAB_00104697:
    lVar10 = FUN_00105dc0("time style",pcVar11,&PTR_s_full_iso_0011c680,&DAT_001156f0,4,
                          PTR_FUN_0011d020,1,pcVar8);
    iVar6 = *(int *)(&DAT_001156f0 + lVar10 * 4);
    if (iVar6 == 1) {
      DAT_0011d158 = "%Y-%m-%d %H:%M";
    }
    else if (iVar6 == 2) {
      DAT_0011d158 = "%Y-%m-%d";
    }
    else if (iVar6 == 0) {
      DAT_0011d158 = "%Y-%m-%d %H:%M:%S.%N %z";
    }
  }
LAB_00104226:
  if (local_80 == (undefined **)0x0) {
    puVar16 = &local_58;
    if (optind < (int)param_1) {
      puVar16 = param_2 + optind;
    }
    param_2 = (undefined8 *)FUN_00105eb0(puVar16);
    DAT_0011d183 = optind + 1 < (int)param_1 || uVar2 == 2;
LAB_0010428d:
    if ((param_2 != (undefined8 *)0x0) && (DAT_0011d198 = FUN_00106240(), DAT_0011d198 != 0)) {
      if ((DAT_0011d184 != '\0') || (DAT_0011d183 == '\0')) {
        local_70 = local_70 | 0x100;
      }
      local_70 = local_70 | uVar2;
      piVar15 = &local_60;
      pcVar8 = "-";
      pcVar17 = "invalid zero-length file name";
      while (pcVar11 = (char *)FUN_00105f20(param_2,piVar15), pcVar11 != (char *)0x0) {
        if (local_80 == (undefined **)0x0) {
          if (*pcVar11 == '\0') {
            bVar18 = false;
            uVar12 = dcgettext(0,pcVar17,5);
            error(0,0,&DAT_0011576f,uVar12);
          }
          else {
LAB_00104341:
            DAT_0011d110 = pcVar11;
            bVar4 = FUN_00105540(local_70);
            bVar18 = (bool)(bVar18 & bVar4);
          }
        }
        else {
          iVar6 = strcmp((char *)local_80,pcVar8);
          if ((iVar6 == 0) && (iVar6 = strcmp(pcVar11,pcVar8), iVar6 == 0)) {
LAB_001043e8:
            uVar12 = FUN_00110580(4,pcVar11);
            uVar13 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",5)
            ;
            error(0,0,uVar13,uVar12);
            if (*pcVar11 == '\0') goto LAB_00104428;
          }
          else {
            if (*pcVar11 != '\0') goto LAB_00104341;
LAB_00104428:
            uVar12 = FUN_00105fc0(param_2);
            uVar13 = dcgettext(0,pcVar17,5);
            uVar14 = FUN_00110680(0,3,local_80);
            error(0,0,"%s:%lu: %s",uVar14,uVar12,uVar13);
          }
          bVar18 = false;
        }
      }
      if (local_60 != 3) {
        if (local_60 == 4) {
          bVar18 = false;
          uVar12 = FUN_00110680(0,3,local_80);
          uVar13 = dcgettext(0,"%s: read error",5);
          piVar15 = __errno_location();
          error(0,*piVar15,uVar13,uVar12);
        }
        else if (local_60 != 2) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main");
        }
        FUN_00105fe0(param_2);
        FUN_001062b0(DAT_0011d198);
        if (DAT_0011d190 != 0) {
          FUN_001062b0();
        }
        if ((local_80 == (undefined **)0x0) ||
           (((iVar6 = ferror_unlocked(stdin), iVar6 == 0 &&
             (iVar6 = FUN_001070c0(stdin), iVar6 == 0)) || (bVar18 == false)))) {
          if (DAT_0011d181 != '\0') {
            uVar12 = dcgettext(0,"total",5);
            FUN_00104e30(&DAT_0011d120,uVar12);
          }
          if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
            return bVar18 ^ 1;
          }
        }
        else {
          uVar12 = FUN_00110580(4,local_80);
          uVar13 = dcgettext(0,"error reading %s",5);
          error(1,0,uVar13,uVar12);
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    }
  }
  else {
    if (optind < (int)param_1) {
      uVar12 = FUN_00110840(param_2[optind]);
      uVar13 = dcgettext(0,"extra operand %s",5);
      error(0,0,uVar13,uVar12);
      uVar12 = dcgettext(0,"file operands cannot be combined with --files0-from",5);
      __fprintf_chk(stderr,1,"%s\n",uVar12);
      FUN_001058b0(1);
LAB_00104872:
      uVar12 = dcgettext(0,"cannot both summarize and show all entries",5);
      error(0,0,uVar12);
LAB_00104893:
      FUN_001058b0(1);
    }
    else {
      iVar6 = strcmp((char *)local_80,"-");
      if ((iVar6 == 0) || (lVar10 = FUN_00108c10(local_80,"r",stdin), lVar10 != 0)) {
        param_2 = (undefined8 *)FUN_00105ee0(stdin);
        DAT_0011d183 = bVar18;
        goto LAB_0010428d;
      }
    }
    uVar12 = FUN_00110580(4,local_80);
    uVar13 = dcgettext(0,"cannot open %s for reading",5);
    piVar15 = __errno_location();
    error(1,*piVar15,uVar13,uVar12);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00111a40();
}




// Function: duinfo_init @ 0x4a40

void duinfo_init(undefined8 *param_1)

{
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0x8000000000000000;
  param_1[3] = 0xffffffffffffffff;
  return;
}




// Function: duinfo_set @ 0x4a70

void duinfo_set(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  *param_1 = param_2;
  param_1[1] = 1;
  param_1[2] = param_3;
  param_1[3] = param_4;
  return;
}




// Function: duinfo_add @ 0x4a90

void duinfo_add(ulong *param_1,ulong *param_2)

{
  ulong uVar1;
  ulong uVar2;
  int iVar3;
  ulong uVar4;
  
  uVar4 = *param_2 + *param_1;
  uVar1 = param_2[2];
  if (CARRY8(*param_2,*param_1)) {
    uVar4 = 0xffffffffffffffff;
  }
  uVar2 = param_2[3];
  *param_1 = uVar4;
  param_1[1] = param_1[1] + param_2[1];
  iVar3 = FUN_00110ee0(param_1[2],param_1[3],uVar1,uVar2);
  if (iVar3 < 0) {
    param_1[2] = uVar1;
    param_1[3] = uVar2;
  }
  return;
}




// Function: hash_ins @ 0x4b50

bool hash_ins(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  iVar1 = FUN_001062e0(param_1,param_3,param_2);
  if (-1 < iVar1) {
    return iVar1 != 0;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00111a40();
}




// Function: fill_mount_table @ 0x4b80

void fill_mount_table(void)

{
  long lVar1;
  int iVar2;
  long lVar3;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar3 = FUN_0010d8b0(0);
  while (lVar3 != 0) {
    if (((*(byte *)(lVar3 + 0x28) & 3) == 0) &&
       (iVar2 = stat(*(char **)(lVar3 + 8),&local_b8), iVar2 == 0)) {
      FUN_00104b50(DAT_0011d190,local_b8.st_ino,local_b8.st_dev);
    }
    lVar1 = *(long *)(lVar3 + 0x30);
    FUN_0010d860(lVar3);
    lVar3 = lVar1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mount_point_in_fts_cycle @ 0x4c30

undefined8 mount_point_in_fts_cycle(long *param_1)

{
  long *plVar1;
  int iVar2;
  
  plVar1 = (long *)*param_1;
  if (DAT_0011d190 == 0) {
    DAT_0011d190 = FUN_00106240();
    if (DAT_0011d190 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_00111a40();
    }
    FUN_00104b80();
  }
  while( true ) {
    if (param_1 == plVar1) {
      return 0;
    }
    iVar2 = FUN_00106330(DAT_0011d190,param_1[0xe],param_1[0xf]);
    if (0 < iVar2) break;
    param_1 = (long *)param_1[1];
    if (param_1 == (long *)0x0) {
      return 0;
    }
  }
  return 1;
}




// Function: print_only_size @ 0x4cc0

void print_only_size(long param_1)

{
  FILE *__stream;
  char *__s;
  long in_FS_OFFSET;
  undefined1 auStack_2a8 [664];
  long local_10;
  
  __stream = stdout;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == -1) {
    __s = (char *)dcgettext(0,"Infinity",5);
  }
  else {
    __s = (char *)FUN_0010c800(param_1,auStack_2a8,DAT_0011d170,1,DAT_0011d148);
  }
  fputs_unlocked(__s,__stream);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: show_date @ 0x4d60

void show_date(undefined8 param_1,undefined8 param_2,ulong param_3,undefined8 param_4)

{
  long lVar1;
  char *__s;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_88;
  ulong local_80;
  undefined1 local_78 [64];
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_88 = param_2;
  local_80 = param_3;
  lVar1 = FUN_00110ce0(param_4,&local_88,local_78);
  if (lVar1 == 0) {
    __s = (char *)thunk_FUN_0010d2d0(local_88,local_38);
    uVar2 = FUN_00110840(__s);
    uVar3 = dcgettext(0,"time %s is out of range",5);
    error(0,0,uVar3,uVar2);
    fputs_unlocked(__s,stdout);
  }
  else {
    FUN_00108b90(stdout,param_1,local_78,param_4,local_80 & 0xffffffff);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_size @ 0x4e30

void print_size(undefined8 *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  uVar1 = param_1[1];
  if (DAT_0011d16d == '\0') {
    uVar1 = *param_1;
  }
  FUN_00104cc0(uVar1);
  if (DAT_0011d16c != '\0') {
    putchar_unlocked(9);
    FUN_00104d60(DAT_0011d158,param_1[2],param_1[3],DAT_0011d150);
  }
  __printf_chk(1,"\t%s%c",param_2,-(DAT_0011d182 == '\0') & 10);
  fflush_unlocked(stdout);
  return;
}




// Function: process_file @ 0x4ec0

undefined4 process_file(long param_1,long param_2)

{
  long lVar1;
  char cVar2;
  undefined4 uVar3;
  ulong uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long lVar8;
  ulong extraout_RDX;
  ushort uVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  ulong local_88;
  ulong uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  ulong local_68;
  ulong uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  long local_40;
  
  uVar9 = *(ushort *)(param_2 + 0x68);
  uVar7 = *(undefined8 *)(param_2 + 0x38);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (uVar9 == 4) {
    uVar6 = FUN_00110580(4,uVar7);
    uVar5 = dcgettext(0,"cannot read directory %s",5);
    uVar3 = 0;
    error(0,*(undefined4 *)(param_2 + 0x40),uVar5,uVar6);
    if (DAT_0011d168 != 0) goto LAB_00104f13;
LAB_00105118:
    auVar10 = FUN_001108b0(param_2 + 0x70);
    if (DAT_0011d185 != '\0') goto LAB_00104f2e;
LAB_0010512a:
    lVar8 = *(long *)(param_2 + 0xb0) << 9;
  }
  else {
    uVar3 = 1;
    if (uVar9 != 6) {
      uVar3 = FUN_00106a90(DAT_0011d140,uVar7);
      if ((char)uVar3 == '\0') {
        if (uVar9 == 0xb) {
          FUN_0010b190(param_1,param_2,1);
          lVar8 = FUN_0010ab50(param_1);
          if (param_2 != lVar8) {
                    /* WARNING: Subroutine does not return */
            __assert_fail("e == ent","src/du.c",0x20f,"process_file");
          }
          uVar9 = *(ushort *)(param_2 + 0x68);
        }
        if (uVar9 == 10 || uVar9 == 0xd) {
          uVar7 = FUN_00110580(4,uVar7);
          uVar6 = dcgettext(0,"cannot access %s",5);
          error(0,*(undefined4 *)(param_2 + 0x40),uVar6,uVar7);
          goto LAB_001050ef;
        }
        if (((((*(byte *)(param_1 + 0x48) & 0x40) != 0) && (0 < *(long *)(param_2 + 0x58))) &&
            (*(long *)(param_1 + 0x18) != *(long *)(param_2 + 0x70))) ||
           ((DAT_0011d184 == '\0' &&
            (((DAT_0011d183 != '\0' ||
              (((*(uint *)(param_2 + 0x88) & 0xf000) != 0x4000 && (1 < *(ulong *)(param_2 + 0x80))))
              ) && (cVar2 = FUN_00104b50(DAT_0011d198,*(undefined8 *)(param_2 + 0x78),
                                         *(undefined8 *)(param_2 + 0x70)), cVar2 == '\0'))))))
        goto LAB_001050e0;
        if (uVar9 != 2) {
          if (uVar9 == 7) {
            uVar3 = 0;
            uVar6 = FUN_00110680(0,3,uVar7);
            error(0,*(undefined4 *)(param_2 + 0x40),&DAT_0011576f,uVar6);
          }
          else {
            uVar3 = 1;
            if (uVar9 == 1) goto LAB_001050ef;
          }
          goto LAB_00104f01;
        }
        cVar2 = FUN_00111ad0(param_1,param_2);
        if ((cVar2 != '\0') && (uVar3 = FUN_00104c30(param_2), (char)uVar3 == '\0')) {
          uVar7 = FUN_00110680(0,3,uVar7);
          uVar6 = dcgettext(0,
                            "WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"
                            ,5);
          error(0,0,uVar6,uVar7);
          goto LAB_001050ef;
        }
      }
      else {
LAB_001050e0:
        if (uVar9 == 1) {
          FUN_0010b190(param_1,param_2,4);
          lVar8 = FUN_0010ab50(param_1);
          if (param_2 != lVar8) {
                    /* WARNING: Subroutine does not return */
            __assert_fail("e == ent","src/du.c",0x230,"process_file");
          }
        }
      }
      uVar3 = 1;
      goto LAB_001050ef;
    }
LAB_00104f01:
    if (DAT_0011d168 == 0) goto LAB_00105118;
LAB_00104f13:
    if (DAT_0011d168 == 2) {
      auVar10 = FUN_00110890();
    }
    else {
      auVar10 = FUN_001108a0();
    }
    if (DAT_0011d185 == '\0') goto LAB_0010512a;
LAB_00104f2e:
    lVar8 = *(long *)(param_2 + 0xa0);
    if (*(long *)(param_2 + 0xa0) < 0) {
      lVar8 = 0;
    }
  }
  FUN_00104a70(&local_88,lVar8,auVar10._0_8_,auVar10._8_8_);
  lVar8 = DAT_0011d100;
  uVar4 = *(ulong *)(param_2 + 0x58);
  local_68 = local_88;
  uStack_60 = uStack_80;
  local_58 = local_78;
  uStack_50 = uStack_70;
  if (DAT_0011d108 == 0) {
    DAT_0011d108 = uVar4 + 10;
    DAT_0011d100 = FUN_00111920(DAT_0011d108,0x40);
    cVar2 = DAT_0011d180;
  }
  else {
    cVar2 = DAT_0011d180;
    if (DAT_0011d188 != uVar4) {
      if (DAT_0011d188 < uVar4) {
        if (DAT_0011d108 <= uVar4) {
          DAT_0011d100 = thunk_FUN_00111730(DAT_0011d100,uVar4,0x80);
          DAT_0011d108 = uVar4 * 2;
        }
        cVar2 = DAT_0011d180;
        if (DAT_0011d188 + 1 <= uVar4) {
          lVar8 = (DAT_0011d188 + 1) * 0x40 + DAT_0011d100;
          do {
            FUN_00104a40(lVar8);
            lVar1 = lVar8 + 0x20;
            lVar8 = lVar8 + 0x40;
            FUN_00104a40(lVar1);
          } while (extraout_RDX <= uVar4);
        }
      }
      else {
        if (DAT_0011d188 - 1 != uVar4) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file");
        }
        lVar1 = DAT_0011d100 + DAT_0011d188 * 0x40;
        FUN_00104a90(&local_68,lVar1);
        cVar2 = DAT_0011d180;
        if (DAT_0011d180 == '\0') {
          FUN_00104a90(&local_68,lVar1 + 0x20);
        }
        lVar8 = lVar8 + 0x20 + uVar4 * 0x40;
        FUN_00104a90(lVar8,lVar1);
        FUN_00104a90(lVar8,lVar1 + 0x20);
      }
    }
  }
  DAT_0011d188 = uVar4;
  if (cVar2 == '\0') {
    FUN_00104a90(uVar4 * 0x40 + DAT_0011d100,&local_88);
    FUN_00104a90(&DAT_0011d120,&local_88);
    if ((uVar9 & 0xfffd) != 4) goto LAB_00105021;
LAB_0010502a:
    if (DAT_0011d010 < uVar4) goto LAB_00105033;
  }
  else {
    if ((uVar9 & 0xfffd) == 4) {
      FUN_00104a90(&DAT_0011d120,&local_88);
      goto LAB_0010502a;
    }
    FUN_00104a90(uVar4 * 0x40 + DAT_0011d100,&local_88);
    FUN_00104a90(&DAT_0011d120,&local_88);
LAB_00105021:
    if (DAT_0011d186 != '\0') goto LAB_0010502a;
LAB_00105033:
    if (uVar4 != 0) goto LAB_001050ef;
  }
  uVar4 = uStack_60;
  if (DAT_0011d16d == '\0') {
    uVar4 = local_68;
  }
  if ((long)DAT_0011d178 < 0) {
    if (-DAT_0011d178 < uVar4) goto LAB_001050ef;
  }
  else if (uVar4 < DAT_0011d178) goto LAB_001050ef;
  FUN_00104e30(&local_68,uVar7);
LAB_001050ef:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: usage @ 0x58b0

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_0011d1b0;
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar3,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Summarize device usage of the set of FILEs, recursively for directories.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00104af0();
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00105870();
    FUN_00104b20();
    FUN_00105640();
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: is_prime @ 0xb340

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




// Function: next_prime @ 0xb3b0

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong uVar2;
  
  uVar2 = 10;
  if (9 < param_1) {
    uVar2 = param_1;
  }
  uVar2 = uVar2 | 1;
  while ((uVar2 != 0xffffffffffffffff && (cVar1 = FUN_0010b340(uVar2), cVar1 == '\0'))) {
    uVar2 = uVar2 + 2;
  }
  return uVar2;
}




// Function: raw_hasher @ 0xb3f0

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = thunk_FUN_00112d80(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0xb410

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0xb420

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_00115a40) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_00115a54 < fVar1) && (fVar1 < DAT_00115a58)) && (_DAT_00115a5c < pfVar2[3])) &&
     (_DAT_00115a70 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_00115a54;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_00115a60)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_00115a40;
  return 0;
}




// Function: free_entry @ 0xb4b0

void free_entry(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x48);
  *param_2 = 0;
  param_2[1] = uVar1;
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: safe_hasher @ 0xb4d0

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




// Function: hash_find_entry @ 0xb500

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  long lVar5;
  
  plVar4 = (long *)FUN_0010b4d0();
  *param_3 = plVar4;
  lVar5 = *plVar4;
  if (lVar5 == 0) {
LAB_0010b5a1:
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
            FUN_0010b4b0(param_1);
            return lVar5;
          }
        }
        goto LAB_0010b5a1;
      }
      lVar5 = *plVar4;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar4[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar4 = *plVar1;
        plVar4[1] = lVar2;
        FUN_0010b4b0(param_1);
        return lVar5;
      }
      *plVar4 = 0;
    }
  }
  return lVar5;
}




// Function: allocate_entry @ 0xb600

void allocate_entry(long param_1)

{
  if (*(long *)(param_1 + 0x48) != 0) {
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)(*(long *)(param_1 + 0x48) + 8);
    return;
  }
  malloc(0x10);
  return;
}




// Function: transfer_entries @ 0xb630

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
LAB_0010b658:
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
          plVar2 = (long *)FUN_0010b4d0(param_1,lVar6);
          plVar4 = (long *)plVar3[1];
          if (*plVar2 != 0) break;
          *plVar2 = lVar6;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0010b4b0(param_1);
          plVar3 = plVar4;
          if (plVar4 == (long *)0x0) goto LAB_0010b6c8;
        }
        plVar3[1] = plVar2[1];
        plVar2[1] = (long)plVar3;
        plVar3 = plVar4;
      } while (plVar4 != (long *)0x0);
LAB_0010b6c8:
      lVar6 = *plVar5;
    }
    plVar5[1] = 0;
    if (param_3 != '\0') goto LAB_0010b658;
    plVar3 = (long *)FUN_0010b4d0(param_1,lVar6);
    if (*plVar3 == 0) {
      *plVar3 = lVar6;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    }
    else {
      plVar4 = (long *)FUN_0010b600(param_1);
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




// Function: hash_get_n_buckets @ 0xb7f0

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0xb800

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0xb810

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0xb820

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




// Function: hash_print_statistics @ 0xb8f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_0010b810();
  uVar2 = FUN_0010b7f0();
  uVar3 = FUN_0010b800();
  uVar4 = FUN_0010b820();
  __fprintf_chk(param_2,1,"# entries:         %lu\n",uVar1);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar2);
  __fprintf_chk(((double)uVar3 * _DAT_00115a70) / (double)uVar2,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar3);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: hash_lookup @ 0xba10

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = (long *)FUN_0010b4d0();
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




// Function: hash_get_next @ 0xbac0

long hash_get_next(long param_1,long param_2)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  
  plVar2 = (long *)FUN_0010b4d0();
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




// Function: hash_do_for_each @ 0xbb80

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




// Function: hash_string @ 0xbc00

ulong hash_string(byte *param_1,ulong param_2)

{
  byte bVar1;
  ulong uVar2;
  
  uVar2 = 0;
  bVar1 = *param_1;
  while (bVar1 != 0) {
    param_1 = param_1 + 1;
    uVar2 = (uVar2 * 0x1f + (ulong)bVar1) % param_2;
    bVar1 = *param_1;
  }
  return uVar2;
}




// Function: hash_initialize @ 0xbc70

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_0010b3f0;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_0010b410;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_00115a40;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_0010b420(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_0010b760(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0xbd80

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
        if ((long *)param_1[1] <= plVar7) goto LAB_0010be1b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010be1b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0xbe30

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
    if (plVar3 <= plVar5) goto LAB_0010bedc;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_0010be9f;
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
LAB_0010be9f:
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
LAB_0010bedc:
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




// Function: hash_rehash @ 0xbf20

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
  __nmemb = FUN_0010b760(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_0010c026;
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
      uVar2 = FUN_0010b630(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_0010b630(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_0010b630(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_0010c026;
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
      goto LAB_0010c026;
    }
  }
  uVar2 = 0;
LAB_0010c026:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0xc0a0

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
LAB_0010c267:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  lVar2 = FUN_0010b500(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar5 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0010c0ee;
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
    FUN_0010b420(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar8._0_4_ <= auVar7._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0010c14e;
    fVar6 = auVar7._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar6 = fVar6 * *(float *)(lVar2 + 8);
    }
    if (fVar6 < _DAT_00115a64) {
      if (_DAT_00115a68 <= fVar6) {
        uVar4 = (long)(fVar6 - _DAT_00115a68) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)fVar6;
      }
      cVar1 = FUN_0010bf20(param_1,uVar4);
      if (cVar1 != '\0') {
        lVar2 = FUN_0010b500(param_1,param_2,&local_38,0);
        if (lVar2 != 0) goto LAB_0010c267;
        goto LAB_0010c14e;
      }
    }
  }
  else {
LAB_0010c14e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar5 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0010c0ee;
    }
    plVar3 = (long *)FUN_0010b600(param_1);
    if (plVar3 != (long *)0x0) {
      *plVar3 = param_2;
      uVar5 = 1;
      plVar3[1] = local_38[1];
      local_38[1] = (long)plVar3;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      goto LAB_0010c0ee;
    }
  }
  uVar5 = 0xffffffff;
LAB_0010c0ee:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert @ 0xc290

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010c0a0(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xc2f0

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
  lVar4 = FUN_0010b500(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_0010c333;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_0010c380;
LAB_0010c436:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_0010c436;
LAB_0010c380:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_0010b420(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_00115a68 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00115a68) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_0010bf20(param_1,uVar5);
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
LAB_0010c333:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}



