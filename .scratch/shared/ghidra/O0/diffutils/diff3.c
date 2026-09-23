// Function: free_diff_block @ 0x2b49

void free_diff_block(void)

{
  return;
}




// Function: next_to_n2 @ 0x2b58

void next_to_n2(void)

{
  return;
}




// Function: main @ 0x2b67

undefined8 main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  byte local_135;
  int local_134;
  int local_130;
  uint local_12c;
  int local_128;
  undefined1 local_120 [8];
  void *local_118;
  void *local_110;
  undefined8 *local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  int local_e0 [4];
  int local_d0;
  int local_cc;
  undefined8 local_c8 [4];
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_12c = 0;
  local_128 = 0;
  DAT_00114030 = 2;
  FUN_0010637f(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_001062bb(0);
  FUN_00109533();
LAB_00102dbd:
  iVar3 = getopt_long(param_1,param_2,"aeimvx3AEL:TX",&PTR_s_diff_program_00113880,0);
  if (iVar3 != -1) {
    switch(iVar3) {
    case 0x33:
      DAT_0011410e = 1;
      local_12c = local_12c | 1;
      break;
    case 0x41:
      DAT_00114110 = '\x01';
      DAT_0011410c = '\x01';
      local_12c = local_12c | 2;
      break;
    case 0x45:
      DAT_0011410c = '\x01';
      local_12c = local_12c | 4;
      break;
    case 0x4c:
      if (local_128 < 3) {
        local_c8[local_128] = optarg;
        local_128 = local_128 + 1;
        break;
      }
      FUN_00103489("too many file label options",0);
    default:
      FUN_00103489(0,0);
      break;
    case 0x54:
      DAT_0011410d = 1;
      break;
    case 0x58:
      DAT_0011410f = 1;
      local_12c = local_12c | 8;
      break;
    case 0x61:
      DAT_00114109 = 1;
      break;
    case 0x65:
      local_12c = local_12c | 0x10;
      break;
    case 0x69:
      DAT_00114111 = 1;
      break;
    case 0x6d:
      DAT_00114112 = 1;
      break;
    case 0x76:
      uVar4 = FUN_00106940("Randy Smith");
      FUN_00108be7(stdout,"diff3","GNU diffutils",PTR_DAT_00114028,uVar4,0);
      FUN_001034ff();
      goto LAB_00103473;
    case 0x78:
      DAT_0011410f = 1;
      local_12c = local_12c | 0x20;
      break;
    case 0x80:
      PTR_DAT_00114020 = optarg;
      break;
    case 0x81:
      FUN_00103556();
      FUN_001034ff();
LAB_00103473:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return 0;
    case 0x82:
      goto switchD_00102c44_caseD_82;
    }
    goto LAB_00102dbd;
  }
  DAT_0011410b = ((DAT_00114112 ^ 1) & local_12c != 0) != 0;
  DAT_00114110 = (DAT_00114112 & local_12c == 0) != 0 || DAT_00114110 != '\0';
  bVar1 = (DAT_00114112 & local_12c == 0) != 0;
  bVar2 = DAT_0011410c != '\0';
  DAT_0011410c = bVar1 || bVar2;
  if ((((local_12c - 1 & local_12c) != 0) || ((DAT_00114112 & DAT_00114111) != 0)) ||
     ((local_128 != 0 && (!bVar1 && !bVar2)))) {
    FUN_00103489("incompatible options",0);
  }
  if (param_1 - optind != 3) {
    if (param_1 - optind < 3) {
      FUN_00103489("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
    }
    FUN_00103489("extra operand \'%s\'",param_2[(long)optind + 3]);
  }
  local_108 = param_2 + optind;
  for (local_134 = local_128; local_134 < 3; local_134 = local_134 + 1) {
    local_c8[local_134] = local_108[local_134];
  }
  local_130 = 2 - (uint)(DAT_00114112 | DAT_0011410b);
  iVar3 = strcmp((char *)local_108[local_130],"-");
  if (iVar3 == 0) {
    local_130 = 3 - local_130;
    iVar3 = strcmp((char *)*local_108,"-");
    if ((iVar3 == 0) || (iVar3 = strcmp((char *)local_108[local_130],"-"), iVar3 == 0)) {
      FUN_00105fcf("\'-\' specified for more than one input file");
    }
  }
  local_e0[0] = 0;
  local_e0[1] = 3 - local_130;
  local_e0[2] = local_130;
  for (local_134 = 0; local_134 < 3; local_134 = local_134 + 1) {
    local_e0[(long)local_e0[local_134] + 3] = local_134;
  }
  for (local_134 = 0; local_134 < 3; local_134 = local_134 + 1) {
    iVar3 = strcmp((char *)local_108[local_134],"-");
    if (iVar3 != 0) {
      iVar3 = stat((char *)local_108[local_134],&local_a8);
      if (iVar3 < 0) {
        FUN_0010600c(local_108[local_134]);
      }
      if ((local_a8.st_mode & 0xf000) == 0x4000) {
                    /* WARNING: Subroutine does not return */
        error(2,0x15,&DAT_0010f17b,local_108[local_134]);
      }
    }
  }
  signal(0x11,(__sighandler_t)0x0);
  local_100 = local_108[local_cc];
  local_f8 = FUN_001043e5(local_108[local_d0],local_100,local_120,&local_110);
  local_f0 = FUN_001043e5(local_108[local_e0[3]],local_100,local_120,&local_118);
  FUN_00102b58(local_f0);
  FUN_00102b58(local_f8);
  local_e8 = FUN_00103696(local_f0,local_f8);
  FUN_00102b49(local_f0);
  FUN_00102b49(local_f8);
  if (DAT_0011410b == 0) {
    if (DAT_00114112 != 0) {
      FUN_00109414(local_108[local_e0[3]],&DAT_0010f17e,stdin);
      local_135 = FUN_00105a29(stdin,stdout,local_e8,local_e0,local_e0 + 3,local_c8[0],local_c8[1],
                               local_c8[2]);
      iVar3 = ferror_unlocked(stdin);
      if (iVar3 == 0) goto LAB_00103442;
      FUN_00105fcf("read failed");
    }
    FUN_00105012(stdout,local_e8,local_e0,local_e0 + 3);
    local_135 = 0;
  }
  else {
    local_135 = FUN_001054de(stdout,local_e8,local_e0,local_e0 + 3,local_c8[0],local_c8[1],
                             local_c8[2]);
  }
LAB_00103442:
  free(local_118);
  free(local_110);
  FUN_001034ff();
                    /* WARNING: Subroutine does not return */
  exit((uint)local_135);
switchD_00102c44_caseD_82:
  DAT_0011410a = 1;
  goto LAB_00102dbd;
}




// Function: try_help @ 0x3489

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00124148;
  if (param_1 != 0) {
    uVar1 = gettext(param_1);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = gettext("Try \'%s --help\' for more information.");
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: check_stdout @ 0x34ff

void check_stdout(void)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 != 0) {
    FUN_00105fcf("write failed");
  }
  iVar1 = fclose(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("standard output");
    FUN_0010600c(uVar2);
  }
  return;
}




// Function: usage @ 0x3556

void usage(void)

{
  FILE *pFVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined **local_20;
  
  uVar3 = DAT_00124148;
  pcVar2 = (char *)gettext("Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n");
  printf(pcVar2,uVar3);
  uVar3 = gettext("Compare three files line by line.");
  printf("%s\n\n",uVar3);
  pFVar1 = stdout;
  pcVar2 = (char *)gettext(
                          "Mandatory arguments to long options are mandatory for short options too.\n"
                          );
  fputs_unlocked(pcVar2,pFVar1);
  for (local_20 = &PTR_s__A____show_all_output_all_change_00113a40; pFVar1 = stdout,
      *local_20 != (undefined *)0x0; local_20 = local_20 + 1) {
    if (**local_20 == '\0') {
      putchar_unlocked(10);
    }
    else {
      uVar3 = gettext(*local_20);
      printf("  %s\n",uVar3);
    }
  }
  pcVar2 = (char *)gettext(
                          "\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n"
                          );
  fputs_unlocked(pcVar2,pFVar1);
  uVar3 = gettext("Exit status is 0 if successful, 1 if conflicts, 2 if trouble.");
  uVar4 = gettext("If a FILE is \'-\', read standard input.");
  printf("\n%s\n%s\n",uVar4,uVar3);
  FUN_00108ccb();
  return;
}




// Function: make_3way_diff @ 0x3696

long make_3way_diff(long param_1,long param_2)

{
  long in_FS_OFFSET;
  uint local_8c;
  uint local_88;
  uint local_84;
  long local_80;
  long local_78;
  long local_70;
  long *local_68;
  undefined *local_60;
  long local_58;
  undefined *local_50;
  long local_48 [5];
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_80 = 0;
  local_68 = &local_80;
  local_48[4] = param_1;
  local_20 = param_2;
  local_60 = &DAT_0010fd80;
  while ((local_48[4] != 0 || (local_20 != 0))) {
    local_48[3] = 0;
    local_48[2] = 0;
    local_48[1] = 0;
    local_48[0] = 0;
    if (local_48[4] == 0) {
      local_88 = 1;
    }
    else if (local_20 == 0) {
      local_88 = 0;
    }
    else {
      local_88 = (uint)(*(long *)(local_20 + 0x10) < *(long *)(local_48[4] + 0x10));
    }
    local_8c = local_88;
    local_58 = local_48[(long)(int)local_88 + 4];
    local_78 = *(long *)(local_58 + 0x18);
    local_48[(long)(int)local_88 + 2] = local_58;
    local_48[(int)local_88] = local_48[(long)(int)local_88 + 2];
    local_48[(long)(int)local_88 + 4] = *(long *)(local_58 + 0x40);
    *(undefined8 *)(local_48[(long)(int)local_88 + 2] + 0x40) = 0;
    local_84 = local_88 ^ 1;
    local_70 = local_48[(long)(int)local_84 + 4];
    while ((local_70 != 0 && (*(long *)(local_70 + 0x10) <= local_78 + 1))) {
      if (local_48[(int)local_84] == 0) {
        local_48[(int)local_84] = local_70;
      }
      else {
        *(long *)(local_48[(long)(int)local_84 + 2] + 0x40) = local_70;
      }
      local_48[(long)(int)local_84 + 2] = local_70;
      local_48[(long)(int)local_84 + 4] = *(long *)(local_48[(long)(int)local_84 + 4] + 0x40);
      *(undefined8 *)(local_70 + 0x40) = 0;
      if (local_78 < *(long *)(local_70 + 0x18)) {
        local_8c = local_8c ^ 1;
        local_78 = *(long *)(local_70 + 0x18);
      }
      local_84 = local_8c ^ 1;
      local_70 = local_48[(long)(int)local_84 + 4];
    }
    local_50 = (undefined *)FUN_0010393d(local_48,local_48 + 2,local_88,local_8c,local_60);
    if (local_50 == (undefined *)0x0) {
      FUN_00105fcf("internal error: screwup in format of diff blocks");
    }
    *local_68 = (long)local_50;
    local_68 = (long *)(local_50 + 0x68);
    local_60 = local_50;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_80;
}




// Function: using_to_diff3_block @ 0x393d

undefined4 * using_to_diff3_block(long *param_1,long param_2,int param_3,int param_4,long param_5)

{
  long lVar1;
  char cVar2;
  undefined4 *puVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  long in_FS_OFFSET;
  int local_a4;
  long *local_a0;
  long local_98;
  long local_90;
  long local_38 [5];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  lVar6 = *(long *)(param_1[param_3] + 0x10);
  lVar4 = *(long *)(*(long *)(param_2 + (long)param_4 * 8) + 0x18);
  for (local_a4 = 0; local_a4 < 2; local_a4 = local_a4 + 1) {
    if (param_1[local_a4] == 0) {
      local_38[local_a4] =
           (lVar6 - *(long *)(param_5 + 0x30)) + *(long *)((long)local_a4 * 0x10 + param_5 + 0x10);
      local_38[(long)local_a4 + 2] =
           (lVar4 - *(long *)(param_5 + 0x30)) + *(long *)((long)local_a4 * 0x10 + param_5 + 0x10);
    }
    else {
      local_38[local_a4] =
           (lVar6 - *(long *)(param_1[local_a4] + 0x10)) + *(long *)param_1[local_a4];
      local_38[(long)local_a4 + 2] =
           (lVar4 - *(long *)(*(long *)(param_2 + (long)local_a4 * 8) + 0x18)) +
           *(long *)(*(long *)(param_2 + (long)local_a4 * 8) + 8);
    }
  }
  puVar3 = (undefined4 *)FUN_00104116(local_38[0],local_38[2],local_38[1],local_38[3],lVar6,lVar4);
  for (local_a4 = 0; local_a4 < 2; local_a4 = local_a4 + 1) {
    for (local_a0 = (long *)param_1[local_a4]; local_a0 != (long *)0x0;
        local_a0 = *(long **)((long)local_a0 + 0x40)) {
      lVar4 = *(long *)((long)local_a0 + 0x10) - lVar6;
      cVar2 = FUN_00104067(*(undefined8 *)((long)local_a0 + 0x28),
                           *(undefined8 *)((long)local_a0 + 0x38),
                           *(long *)(puVar3 + 0x12) + lVar4 * 8,*(long *)(puVar3 + 0x18) + lVar4 * 8
                           ,(*(long *)((long)local_a0 + 0x18) - *(long *)((long)local_a0 + 0x10)) +
                            1);
      if (cVar2 != '\x01') {
        puVar3 = (undefined4 *)0x0;
        goto LAB_00104051;
      }
    }
  }
  local_a4 = 0;
  do {
    if (1 < local_a4) {
      if (*param_1 == 0) {
        *puVar3 = 6;
      }
      else if (param_1[1] == 0) {
        *puVar3 = 5;
      }
      else {
        lVar6 = (*(long *)(puVar3 + 4) - *(long *)(puVar3 + 2)) + 1;
        if ((lVar6 == (*(long *)(puVar3 + 8) - *(long *)(puVar3 + 6)) + 1) &&
           (cVar2 = FUN_00104311(*(undefined8 *)(puVar3 + 0xe),*(undefined8 *)(puVar3 + 0x14),
                                 *(undefined8 *)(puVar3 + 0x10),*(undefined8 *)(puVar3 + 0x16),lVar6
                                ), cVar2 == '\x01')) {
          *puVar3 = 7;
        }
        else {
          *puVar3 = 4;
        }
      }
LAB_00104051:
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return puVar3;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_a0 = (long *)param_1[local_a4];
    lVar4 = local_38[local_a4];
    lVar1 = local_38[(long)local_a4 + 2];
    local_98 = 0;
    while( true ) {
      if (local_a0 == (long *)0x0) {
        lVar5 = lVar1 + 1;
      }
      else {
        lVar5 = *local_a0;
      }
      if (lVar5 <= local_98 + lVar4) break;
      *(undefined8 *)(*(long *)(puVar3 + ((long)local_a4 + 6) * 2 + 2) + local_98 * 8) =
           *(undefined8 *)(*(long *)(puVar3 + 0x12) + local_98 * 8);
      *(undefined8 *)(*(long *)(puVar3 + ((long)local_a4 + 10) * 2) + local_98 * 8) =
           *(undefined8 *)(*(long *)(puVar3 + 0x18) + local_98 * 8);
      local_98 = local_98 + 1;
    }
    for (; local_a0 != (long *)0x0; local_a0 = (long *)local_a0[8]) {
      cVar2 = FUN_00104067(local_a0[4],local_a0[6],
                           *(long *)(puVar3 + ((long)local_a4 + 6) * 2 + 2) +
                           (*local_a0 - lVar4) * 8,
                           *(long *)(puVar3 + ((long)local_a4 + 10) * 2) + (*local_a0 - lVar4) * 8,
                           (local_a0[1] - *local_a0) + 1);
      if (cVar2 != '\x01') {
        puVar3 = (undefined4 *)0x0;
        goto LAB_00104051;
      }
      local_90 = (local_a0[3] + 1) - lVar6;
      local_98 = (local_a0[1] + 1) - lVar4;
      while( true ) {
        if (local_a0[8] == 0) {
          lVar5 = lVar1 + 1;
        }
        else {
          lVar5 = *(long *)local_a0[8];
        }
        if (lVar5 - lVar4 <= local_98) break;
        *(undefined8 *)(*(long *)(puVar3 + ((long)local_a4 + 6) * 2 + 2) + local_98 * 8) =
             *(undefined8 *)(*(long *)(puVar3 + 0x12) + local_90 * 8);
        *(undefined8 *)(*(long *)(puVar3 + ((long)local_a4 + 10) * 2) + local_98 * 8) =
             *(undefined8 *)(*(long *)(puVar3 + 0x18) + local_90 * 8);
        local_90 = local_90 + 1;
        local_98 = local_98 + 1;
      }
    }
    local_a4 = local_a4 + 1;
  } while( true );
}




// Function: copy_stringlist @ 0x4067

undefined8 copy_stringlist(long *param_1,size_t *param_2,long *param_3,size_t *param_4,long param_5)

{
  int iVar1;
  long local_50;
  
  local_50 = param_5;
  do {
    if (local_50 == 0) {
      return 1;
    }
    if (*param_3 == 0) {
      *param_3 = *param_1;
      *param_4 = *param_2;
    }
    else if ((*param_2 != *param_4) ||
            (iVar1 = memcmp((void *)*param_1,(void *)*param_3,*param_2), iVar1 != 0)) {
      return 0;
    }
    param_3 = param_3 + 1;
    param_1 = param_1 + 1;
    param_4 = param_4 + 1;
    param_2 = param_2 + 1;
    local_50 = local_50 + -1;
  } while( true );
}




// Function: create_diff3_block @ 0x4116

undefined4 *
create_diff3_block(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 param_5,undefined8 param_6)

{
  undefined4 *puVar1;
  long lVar2;
  undefined8 uVar3;
  
  puVar1 = (undefined4 *)FUN_00108e10(0x70);
  *puVar1 = 0;
  *(undefined8 *)(puVar1 + 0x1a) = 0;
  *(undefined8 *)(puVar1 + 2) = param_1;
  *(undefined8 *)(puVar1 + 4) = param_2;
  *(undefined8 *)(puVar1 + 6) = param_3;
  *(undefined8 *)(puVar1 + 8) = param_4;
  *(undefined8 *)(puVar1 + 10) = param_5;
  *(undefined8 *)(puVar1 + 0xc) = param_6;
  lVar2 = (*(long *)(puVar1 + 4) - *(long *)(puVar1 + 2)) + 1;
  if (lVar2 == 0) {
    *(undefined8 *)(puVar1 + 0xe) = 0;
    *(undefined8 *)(puVar1 + 0x14) = 0;
  }
  else {
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0xe) = uVar3;
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0x14) = uVar3;
  }
  lVar2 = (*(long *)(puVar1 + 8) - *(long *)(puVar1 + 6)) + 1;
  if (lVar2 == 0) {
    *(undefined8 *)(puVar1 + 0x10) = 0;
    *(undefined8 *)(puVar1 + 0x16) = 0;
  }
  else {
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0x10) = uVar3;
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0x16) = uVar3;
  }
  lVar2 = (*(long *)(puVar1 + 0xc) - *(long *)(puVar1 + 10)) + 1;
  if (lVar2 == 0) {
    *(undefined8 *)(puVar1 + 0x12) = 0;
    *(undefined8 *)(puVar1 + 0x18) = 0;
  }
  else {
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0x12) = uVar3;
    uVar3 = FUN_0010927a(lVar2,8);
    *(undefined8 *)(puVar1 + 0x18) = uVar3;
  }
  return puVar1;
}




// Function: compare_line_list @ 0x4311

undefined8 compare_line_list(long *param_1,size_t *param_2,long *param_3,size_t *param_4,long param_5)

{
  int iVar1;
  long local_50;
  long *local_28;
  long *local_20;
  size_t *local_18;
  size_t *local_10;
  
  local_50 = param_5;
  local_28 = param_1;
  local_20 = param_3;
  local_18 = param_2;
  local_10 = param_4;
  while( true ) {
    if (local_50 == 0) {
      return 1;
    }
    if (*local_28 == 0) {
      return 0;
    }
    if (*local_20 == 0) {
      return 0;
    }
    if (*local_18 != *local_10) break;
    iVar1 = memcmp((void *)*local_28,(void *)*local_20,*local_18);
    local_50 = local_50 + -1;
    local_28 = local_28 + 1;
    local_20 = local_20 + 1;
    local_18 = local_18 + 1;
    local_10 = local_10 + 1;
    if (iVar1 != 0) {
      return 0;
    }
  }
  return 0;
}




// Function: process_diff @ 0x43e5

long process_diff(undefined8 param_1,undefined8 param_2,undefined8 *param_3,undefined8 *param_4)

{
  char cVar1;
  undefined8 uVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  long lVar6;
  long in_FS_OFFSET;
  char *local_70;
  char *local_68;
  long local_60;
  long local_58;
  long *local_50;
  long *local_48;
  ulong local_40;
  char *local_38;
  ulong local_30;
  ulong local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = &local_60;
  local_40 = 0xfffffffffffffff;
  local_38 = (char *)FUN_00104a3a(param_1,param_2,&local_70);
  *param_4 = local_70;
  local_68 = local_70;
  do {
    if (local_38 <= local_68) {
      *local_50 = 0;
      *param_3 = local_48;
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return local_60;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_48 = (long *)FUN_00108e10(0x48);
    local_48[5] = 0;
    local_48[4] = local_48[5];
    local_48[7] = 0;
    local_48[6] = local_48[7];
    uVar3 = FUN_00104887(&local_68,local_48);
    uVar2 = DAT_00124148;
    if ((uVar3 == 0) || (*local_68 != '\n')) {
      pcVar5 = (char *)gettext("%s: diff failed: ");
      fprintf(stderr,pcVar5,uVar2);
      do {
        putc_unlocked((int)*local_68,stderr);
        pcVar5 = local_68 + 1;
        cVar1 = *local_68;
        local_68 = pcVar5;
      } while (cVar1 != '\n');
                    /* WARNING: Subroutine does not return */
      exit(2);
    }
    local_68 = local_68 + 1;
    if (uVar3 == 3) {
      local_48[2] = local_48[2] + 1;
    }
    else if (uVar3 < 4) {
      if (uVar3 == 1) {
        *local_48 = *local_48 + 1;
      }
      else if (uVar3 != 2) goto LAB_0010458e;
    }
    else {
LAB_0010458e:
      FUN_00105fcf("internal error: invalid diff type in process_diff");
    }
    if (uVar3 != 1) {
      local_30 = (local_48[1] - *local_48) + 1;
      if (local_40 <= local_30) {
        FUN_001093d4();
      }
      lVar6 = FUN_00108e10(local_30 << 3);
      local_48[4] = lVar6;
      lVar6 = FUN_00108e10(local_30 << 3);
      local_48[6] = lVar6;
      for (local_58 = 0; local_58 < (long)local_30; local_58 = local_58 + 1) {
        local_68 = (char *)FUN_00104edc(local_68,local_48[4] + local_58 * 8,
                                        local_48[6] + local_58 * 8,local_38,0x3c);
      }
    }
    if (uVar3 == 2) {
      iVar4 = strncmp(local_68,"---\n",4);
      if (iVar4 != 0) {
        FUN_00105fcf("invalid diff format; invalid change separator");
      }
      local_68 = local_68 + 4;
    }
    if (uVar3 != 3) {
      local_28 = (local_48[3] - local_48[2]) + 1;
      if (local_40 <= local_28) {
        FUN_001093d4();
      }
      lVar6 = FUN_00108e10(local_28 << 3);
      local_48[5] = lVar6;
      lVar6 = FUN_00108e10(local_28 << 3);
      local_48[7] = lVar6;
      for (local_58 = 0; local_58 < (long)local_28; local_58 = local_58 + 1) {
        local_68 = (char *)FUN_00104edc(local_68,local_48[5] + local_58 * 8,
                                        local_48[7] + local_58 * 8,local_38,0x3e);
      }
    }
    *local_50 = (long)local_48;
    local_50 = local_48 + 8;
  } while( true );
}




// Function: skipwhite @ 0x47d5

char * skipwhite(char *param_1)

{
  char *local_10;
  
  for (local_10 = param_1; (*local_10 == ' ' || (*local_10 == '\t')); local_10 = local_10 + 1) {
  }
  return local_10;
}




// Function: readnum @ 0x4804

byte * readnum(byte *param_1,long *param_2)

{
  byte *local_20;
  byte local_11;
  long local_10;
  
  local_11 = *param_1;
  local_10 = 0;
  local_20 = param_1;
  if (local_11 - 0x30 < 10) {
    do {
      local_10 = local_10 * 10 + (long)(int)(local_11 - 0x30);
      local_20 = local_20 + 1;
      local_11 = *local_20;
    } while (local_11 - 0x30 < 10);
    *param_2 = local_10;
  }
  else {
    local_20 = (byte *)0x0;
  }
  return local_20;
}




// Function: process_diff_control @ 0x4887

undefined4 process_diff_control(undefined8 *param_1,undefined8 *param_2)

{
  char cVar1;
  undefined8 uVar2;
  long lVar3;
  char *pcVar4;
  undefined4 local_1c;
  char *local_18;
  
  uVar2 = FUN_001047d5(*param_1);
  lVar3 = FUN_00104804(uVar2,param_2);
  if (lVar3 == 0) {
    return 0;
  }
  local_18 = (char *)FUN_001047d5(lVar3);
  if (*local_18 == ',') {
    local_18 = (char *)FUN_00104804(local_18 + 1,param_2 + 1);
    if (local_18 == (char *)0x0) {
      return 0;
    }
  }
  else {
    param_2[1] = *param_2;
  }
  pcVar4 = (char *)FUN_001047d5(local_18);
  cVar1 = *pcVar4;
  if (cVar1 == 'd') {
    local_1c = 3;
  }
  else {
    if ('d' < cVar1) {
      return 0;
    }
    if (cVar1 == 'a') {
      local_1c = 1;
    }
    else {
      if (cVar1 != 'c') {
        return 0;
      }
      local_1c = 2;
    }
  }
  uVar2 = FUN_001047d5(pcVar4 + 1);
  lVar3 = FUN_00104804(uVar2,param_2 + 2);
  if (lVar3 == 0) {
    local_1c = 0;
  }
  else {
    local_18 = (char *)FUN_001047d5(lVar3);
    if (*local_18 == ',') {
      lVar3 = FUN_00104804(local_18 + 1,param_2 + 3);
      if (lVar3 == 0) {
        return 0;
      }
      local_18 = (char *)FUN_001047d5(lVar3);
    }
    else {
      param_2[3] = param_2[2];
    }
    *param_1 = local_18;
  }
  return local_1c;
}




// Function: read_diff @ 0x4a3a

long read_diff(undefined *param_1,undefined *param_2,long *param_3)

{
  undefined **ppuVar1;
  undefined *puVar2;
  int iVar3;
  __pid_t _Var4;
  int *piVar5;
  undefined8 uVar6;
  char *pcVar7;
  long in_FS_OFFSET;
  uint local_154;
  int local_150;
  __pid_t local_14c;
  int local_148;
  uint local_144;
  long local_140;
  ulong local_138;
  long local_130;
  undefined **local_128;
  long local_120;
  long local_118;
  int local_110;
  int local_10c;
  char *local_108;
  undefined *local_100 [9];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_150 = 0;
  local_128 = local_100;
  local_108 = PTR_DAT_00114020;
  if (DAT_00114109 != '\0') {
    local_128 = local_100 + 1;
    local_100[0] = &DAT_0010fb16;
  }
  if (DAT_0011410a != '\0') {
    *local_128 = "--strip-trailing-cr";
    local_128 = local_128 + 1;
  }
  *local_128 = "--horizon-lines=100";
  local_128[1] = &DAT_0010fb41;
  ppuVar1 = local_128 + 3;
  local_128[2] = param_1;
  local_128 = local_128 + 4;
  *ppuVar1 = param_2;
  *local_128 = (undefined *)0x0;
  iVar3 = pipe(&local_110);
  if (iVar3 != 0) {
    FUN_0010600c(&DAT_0010fb44);
  }
  local_14c = fork();
  if (local_14c == 0) {
    close(local_110);
    if (local_10c != 1) {
      dup2(local_10c,1);
      close(local_10c);
    }
    execvp(PTR_DAT_00114020,&local_108);
    piVar5 = __errno_location();
    if (*piVar5 == 2) {
      iVar3 = 0x7f;
    }
    else {
      iVar3 = 0x7e;
    }
                    /* WARNING: Subroutine does not return */
    _exit(iVar3);
  }
  if (local_14c == -1) {
    FUN_0010600c(&DAT_0010fb49);
  }
  close(local_10c);
  local_148 = local_110;
  iVar3 = fstat(local_110,&local_b8);
  if (iVar3 != 0) {
    FUN_0010600c("fstat");
  }
  if (local_b8.st_blksize < 1) {
    local_b8.st_blksize = 1;
  }
  local_138 = local_b8.st_blksize;
  local_140 = FUN_00108e10(local_b8.st_blksize);
  local_130 = 0;
  do {
    local_120 = local_138 - local_130;
    local_118 = FUN_00109671(local_148,local_140 + local_130,local_120);
    local_130 = local_130 + local_118;
    if (local_118 != local_120) {
      if (local_118 != -1) {
        if ((local_130 != 0) && (*(char *)(local_140 + local_130 + -1) != '\n')) {
          FUN_00105fcf("invalid diff format; incomplete last line");
        }
        *param_3 = local_140;
        iVar3 = close(local_148);
        if (iVar3 != 0) {
          FUN_0010600c("close");
        }
        _Var4 = waitpid(local_14c,(int *)&local_154,0);
        if (_Var4 < 0) {
          FUN_0010600c("waitpid");
        }
        puVar2 = PTR_DAT_00114020;
        if ((local_150 == 0) && ((local_154 & 0x7f) == 0)) {
          local_144 = (int)local_154 >> 8 & 0xff;
        }
        else {
          local_144 = 0x7fffffff;
        }
        if (1 < local_144) {
          if (local_144 == 0x7e) {
            pcVar7 = "subsidiary program \'%s\' could not be invoked";
          }
          else if (local_144 == 0x7f) {
            pcVar7 = "subsidiary program \'%s\' not found";
          }
          else if (local_144 == 0x7fffffff) {
            pcVar7 = "subsidiary program \'%s\' failed";
          }
          else {
            pcVar7 = "subsidiary program \'%s\' failed (exit status %d)";
          }
          uVar6 = gettext(pcVar7);
                    /* WARNING: Subroutine does not return */
          error(2,local_150,uVar6,puVar2,local_144);
        }
        if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return local_130 + local_140;
      }
      uVar6 = gettext("read failed");
      FUN_0010600c(uVar6);
    }
    if (0x3ffffffffffffffe < local_138) {
      FUN_001093d4();
    }
    local_138 = local_138 << 1;
    local_140 = FUN_00108e5c(local_140,local_138);
  } while( true );
}




// Function: scan_diff_line @ 0x4edc

char * scan_diff_line(char *param_1,ulong *param_2,long *param_3,char *param_4,char param_5)

{
  char *pcVar1;
  char cVar2;
  char *local_10;
  
  if ((param_5 != *param_1) || (param_1[1] != ' ')) {
    FUN_00105fcf("invalid diff format; incorrect leading line chars");
  }
  *param_2 = (ulong)(param_1 + 2);
  pcVar1 = param_1 + 2;
  do {
    local_10 = pcVar1;
    pcVar1 = local_10 + 1;
  } while (*local_10 != '\n');
  *param_3 = (long)pcVar1 - *param_2;
  if ((pcVar1 < param_4) && (*pcVar1 == '\\')) {
    if (DAT_0011410b == '\0') {
      *param_3 = *param_3 + -1;
    }
    else {
      fprintf(stderr,"%s:",DAT_00124148);
    }
    local_10 = local_10 + 2;
    do {
      if (DAT_0011410b != '\0') {
        putc_unlocked((int)*local_10,stderr);
      }
      cVar2 = *local_10;
      pcVar1 = local_10 + 1;
      local_10 = local_10 + 1;
    } while (cVar2 != '\n');
  }
  local_10 = pcVar1;
  return local_10;
}




// Function: output_diff3 @ 0x5012

void output_diff3(FILE *param_1,uint *param_2,long param_3,long param_4)

{
  uint uVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  void *__ptr;
  size_t __n;
  char *__s;
  undefined8 uVar5;
  long in_FS_OFFSET;
  uint local_70;
  int local_6c;
  uint local_68;
  uint *local_60;
  long local_58;
  char local_12 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = param_2;
  if (DAT_0011410d == '\0') {
    __s = "  ";
  }
  else {
    __s = "\t";
  }
  for (; local_60 != (uint *)0x0; local_60 = *(uint **)(local_60 + 0x1a)) {
    uVar1 = *local_60;
    if (uVar1 == 4) {
      local_12[0] = '\0';
      local_68 = 3;
      local_6c = 3;
    }
    else if ((uVar1 < 4) || (2 < uVar1 - 5)) {
      FUN_00105fcf("internal error: invalid diff type passed to output");
    }
    else {
      local_6c = *(int *)(param_4 + (ulong)(*local_60 - 5) * 4);
      local_12[0] = (char)local_6c + '1';
      local_12[1] = 0;
      local_68 = (uint)(local_6c == 0);
    }
    fprintf(param_1,"====%s\n",local_12);
    local_70 = 0;
    while ((int)local_70 < 3) {
      iVar2 = *(int *)(param_3 + (long)(int)local_70 * 4);
      lVar3 = *(long *)(local_60 + (long)iVar2 * 4 + 2);
      lVar4 = *(long *)(local_60 + (long)iVar2 * 4 + 4);
      fprintf(param_1,"%d:",(ulong)(local_70 + 1));
      if (lVar3 == lVar4) {
        fprintf(param_1,"%ldc\n",lVar3);
      }
      else if (lVar3 - lVar4 == 1) {
        fprintf(param_1,"%lda\n",lVar3 + -1);
      }
      else {
        fprintf(param_1,"%ld,%ldc\n",lVar3,lVar4);
      }
      if ((local_70 != local_68) && (lVar3 <= lVar4)) {
        local_58 = 0;
        do {
          fputs_unlocked(__s,param_1);
          __ptr = *(void **)(local_58 * 8 + *(long *)(local_60 + ((long)iVar2 + 6) * 2 + 2));
          __n = *(size_t *)(local_58 * 8 + *(long *)(local_60 + ((long)iVar2 + 10) * 2));
          fwrite_unlocked(__ptr,1,__n,param_1);
          local_58 = local_58 + 1;
        } while (local_58 <= lVar4 - lVar3);
        if (*(char *)((long)__ptr + (__n - 1)) != '\n') {
          uVar5 = gettext("No newline at end of file");
          fprintf(param_1,"\n\\ %s\n",uVar5);
        }
      }
      if (local_6c == 1) {
        local_70 = *(uint *)(&DAT_00114010 + (long)(int)local_70 * 4);
      }
      else {
        local_70 = local_70 + 1;
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dotlines @ 0x5369

undefined1 dotlines(FILE *param_1,long param_2,int param_3)

{
  char *__ptr;
  undefined1 local_19;
  undefined8 local_18;
  
  local_19 = 0;
  for (local_18 = 0;
      local_18 <=
      *(long *)((long)param_3 * 0x10 + param_2 + 0x10) -
      *(long *)((long)param_3 * 0x10 + param_2 + 8); local_18 = local_18 + 1) {
    __ptr = *(char **)(local_18 * 8 + *(long *)(param_2 + 8 + ((long)param_3 + 6) * 8));
    if (*__ptr == '.') {
      local_19 = 1;
      fputc_unlocked(0x2e,param_1);
    }
    fwrite_unlocked(__ptr,1,*(size_t *)
                             (local_18 * 8 + *(long *)(param_2 + ((long)param_3 + 10) * 8)),param_1)
    ;
  }
  return local_19;
}




// Function: undotlines @ 0x544f

void undotlines(FILE *param_1,char param_2,long param_3,long param_4)

{
  fputs_unlocked(".\n",param_1);
  if (param_2 != '\0') {
    if (param_4 == 1) {
      fprintf(param_1,"%lds/^\\.//\n",param_3);
    }
    else {
      fprintf(param_1,"%ld,%lds/^\\.//\n",param_3,param_4 + param_3 + -1);
    }
  }
  return;
}




// Function: output_diff3_edscript @ 0x54de

undefined1
output_diff3_edscript(FILE *param_1,undefined8 param_2,int *param_3,long param_4,undefined8 param_5,
            undefined8 param_6,undefined8 param_7)

{
  long lVar1;
  long lVar2;
  long lVar3;
  char cVar4;
  undefined1 uVar5;
  uint uVar6;
  undefined8 uVar7;
  bool local_37;
  undefined1 local_36;
  char local_35;
  int *local_30;
  
  local_36 = 0;
  local_30 = (int *)FUN_00105f95(param_2);
  do {
    if (local_30 == (int *)0x0) {
      if (DAT_00114111 != '\0') {
        fputs_unlocked("w\nq\n",param_1);
      }
      return local_36;
    }
    if (*local_30 == 4) {
      uVar6 = 4;
    }
    else {
      uVar6 = *(int *)(param_4 + (ulong)(*local_30 - 5) * 4) + 5;
    }
    if (uVar6 == 7) {
      if (DAT_0011410f == '\0') {
        local_35 = '\0';
        goto LAB_001055b7;
      }
    }
    else if (uVar6 < 8) {
      if (uVar6 == 4) {
        if (DAT_0011410e == '\0') {
          local_35 = DAT_0011410c;
          goto LAB_001055b7;
        }
      }
      else if ((uVar6 == 6) && (DAT_00114110 == '\x01')) {
        local_35 = '\x01';
LAB_001055b7:
        lVar1 = *(long *)(local_30 + (long)*param_3 * 4 + 2);
        lVar2 = *(long *)(local_30 + (long)*param_3 * 4 + 4);
        if (local_35 == '\0') {
          if (*(long *)(local_30 + (long)param_3[2] * 4 + 4) -
              *(long *)(local_30 + (long)param_3[2] * 4 + 2) == -1) {
            if (lVar1 == lVar2) {
              fprintf(param_1,"%ldd\n",lVar1);
            }
            else {
              fprintf(param_1,"%ld,%ldd\n",lVar1,lVar2);
            }
          }
          else {
            if (lVar2 - lVar1 == -1) {
              fprintf(param_1,"%lda\n",lVar2);
            }
            else if (lVar2 == lVar1) {
              fprintf(param_1,"%ldc\n",lVar2);
            }
            else {
              fprintf(param_1,"%ld,%ldc\n",lVar1,lVar2);
            }
            lVar2 = *(long *)(local_30 + (long)param_3[2] * 4 + 4);
            lVar3 = *(long *)(local_30 + (long)param_3[2] * 4 + 2);
            uVar5 = FUN_00105369(param_1,local_30,param_3[2]);
            FUN_0010544f(param_1,uVar5,lVar1,(lVar2 - lVar3) + 1);
          }
        }
        else {
          local_36 = 1;
          fprintf(param_1,"%lda\n",lVar2);
          local_37 = false;
          if (uVar6 == 4) {
            if (DAT_00114110 != '\0') {
              fprintf(param_1,"||||||| %s\n",param_6);
              local_37 = (bool)FUN_00105369(param_1,local_30,param_3[1]);
            }
            fputs_unlocked("=======\n",param_1);
            cVar4 = FUN_00105369(param_1,local_30,param_3[2]);
            local_37 = cVar4 != '\0' || local_37 != false;
          }
          fprintf(param_1,">>>>>>> %s\n",param_7);
          FUN_0010544f(param_1,local_37,lVar2 + 2,
                       (*(long *)(local_30 + (long)param_3[2] * 4 + 4) -
                       *(long *)(local_30 + (long)param_3[2] * 4 + 2)) +
                       (*(long *)(local_30 + (long)param_3[1] * 4 + 4) -
                       *(long *)(local_30 + (long)param_3[1] * 4 + 2)) + 3);
          uVar7 = param_6;
          if (uVar6 == 4) {
            uVar7 = param_5;
          }
          fprintf(param_1,"%lda\n<<<<<<< %s\n",lVar1 + -1,uVar7);
          local_37 = false;
          if (uVar6 == 6) {
            local_37 = (bool)FUN_00105369(param_1,local_30,param_3[1]);
            fputs_unlocked("=======\n",param_1);
          }
          FUN_0010544f(param_1,local_37,lVar1 + 1,
                       (*(long *)(local_30 + (long)param_3[1] * 4 + 4) -
                       *(long *)(local_30 + (long)param_3[1] * 4 + 2)) + 1);
        }
      }
    }
    local_30 = *(int **)(local_30 + 0x1a);
  } while( true );
}




// Function: output_diff3_merge @ 0x5a29

undefined1
output_diff3_merge(FILE *param_1,FILE *param_2,int *param_3,int *param_4,long param_5,undefined8 param_6,
            undefined8 param_7,undefined8 param_8)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  undefined1 local_42;
  char local_41;
  long local_38;
  int *local_30;
  long local_28;
  char *local_20;
  
  local_42 = 0;
  local_28 = 0;
  local_30 = param_3;
  do {
    if (local_30 == (int *)0x0) {
      do {
        iVar2 = getc_unlocked(param_1);
        if (iVar2 == -1) {
          iVar3 = ferror_unlocked(param_1);
          iVar4 = feof_unlocked(param_1);
          if (iVar4 != 0 || iVar3 != 0) {
            return local_42;
          }
        }
        putc_unlocked(iVar2,param_2);
      } while( true );
    }
    if (*local_30 == 4) {
      uVar1 = 4;
    }
    else {
      uVar1 = *(int *)(param_5 + (ulong)(*local_30 - 5) * 4) + 5;
    }
    local_20 = "<<<<<<< %s\n";
    if (uVar1 == 7) {
      if (DAT_0011410f == '\0') {
        local_41 = '\0';
        goto LAB_00105b18;
      }
    }
    else if (uVar1 < 8) {
      if (uVar1 == 4) {
        if (DAT_0011410e == '\0') {
          local_41 = DAT_0011410c;
          local_20 = "||||||| %s\n";
          goto LAB_00105b18;
        }
      }
      else if ((uVar1 == 6) && (DAT_00114110 == '\x01')) {
        local_41 = '\x01';
LAB_00105b18:
        local_38 = (*(long *)(local_30 + 2) - local_28) + -1;
        local_28 = local_28 + local_38;
        while (local_38 = local_38 + -1, -1 < local_38) {
          do {
            iVar2 = getc_unlocked(param_1);
            if (iVar2 == -1) {
              iVar3 = ferror_unlocked(param_1);
              if (iVar3 != 0) {
                uVar5 = gettext("read failed");
                FUN_0010600c(uVar5);
              }
              iVar3 = feof_unlocked(param_1);
              if (iVar3 != 0) {
                FUN_00105fcf("input file shrank");
              }
            }
            putc_unlocked(iVar2,param_2);
          } while (iVar2 != 10);
        }
        if (local_41 != '\0') {
          local_42 = 1;
          if (uVar1 == 4) {
            fprintf(param_2,"<<<<<<< %s\n",param_6);
            for (local_38 = 0;
                local_38 <=
                *(long *)(local_30 + (long)*param_4 * 4 + 4) -
                *(long *)(local_30 + (long)*param_4 * 4 + 2); local_38 = local_38 + 1) {
              fwrite_unlocked(*(void **)(local_38 * 8 +
                                        *(long *)(local_30 + ((long)*param_4 + 6) * 2 + 2)),1,
                              *(size_t *)
                               (local_38 * 8 + *(long *)(local_30 + ((long)*param_4 + 10) * 2)),
                              param_2);
            }
          }
          if (DAT_00114110 != '\0') {
            fprintf(param_2,local_20,param_7);
            for (local_38 = 0;
                local_38 <=
                *(long *)(local_30 + (long)param_4[1] * 4 + 4) -
                *(long *)(local_30 + (long)param_4[1] * 4 + 2); local_38 = local_38 + 1) {
              fwrite_unlocked(*(void **)(local_38 * 8 +
                                        *(long *)(local_30 + ((long)param_4[1] + 6) * 2 + 2)),1,
                              *(size_t *)
                               (local_38 * 8 + *(long *)(local_30 + ((long)param_4[1] + 10) * 2)),
                              param_2);
            }
          }
          fputs_unlocked("=======\n",param_2);
        }
        for (local_38 = 0;
            local_38 <=
            *(long *)(local_30 + (long)param_4[2] * 4 + 4) -
            *(long *)(local_30 + (long)param_4[2] * 4 + 2); local_38 = local_38 + 1) {
          fwrite_unlocked(*(void **)(local_38 * 8 +
                                    *(long *)(local_30 + ((long)param_4[2] + 6) * 2 + 2)),1,
                          *(size_t *)
                           (local_38 * 8 + *(long *)(local_30 + ((long)param_4[2] + 10) * 2)),
                          param_2);
        }
        if (local_41 != '\0') {
          fprintf(param_2,">>>>>>> %s\n",param_8);
        }
        local_38 = (*(long *)(local_30 + 4) - *(long *)(local_30 + 2)) + 1;
        local_28 = local_28 + local_38;
        while (local_38 = local_38 + -1, -1 < local_38) {
          while (iVar2 = getc_unlocked(param_1), iVar2 != 10) {
            if (iVar2 == -1) {
              iVar2 = ferror_unlocked(param_1);
              if (iVar2 != 0) {
                uVar5 = gettext("read failed");
                FUN_0010600c(uVar5);
              }
              iVar2 = feof_unlocked(param_1);
              if (iVar2 != 0) {
                if ((local_38 == 0) && (*(long *)(local_30 + 0x1a) == 0)) {
                  return local_42;
                }
                FUN_00105fcf("input file shrank");
                return local_42;
              }
            }
          }
        }
      }
    }
    local_30 = *(int **)(local_30 + 0x1a);
  } while( true );
}




// Function: reverse_diff3_blocklist @ 0x5f95

long reverse_diff3_blocklist(long param_1)

{
  long lVar1;
  long lVar2;
  
  lVar2 = 0;
  while (lVar1 = param_1, lVar1 != 0) {
    param_1 = *(long *)(lVar1 + 0x68);
    *(long *)(lVar1 + 0x68) = lVar2;
    lVar2 = lVar1;
  }
  return lVar2;
}




// Function: fatal @ 0x5fcf

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = gettext(param_1);
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_0010f17b,uVar1);
}




// Function: perror_with_exit @ 0x600c

void perror_with_exit(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar1,&DAT_0010f17b,param_1);
}



