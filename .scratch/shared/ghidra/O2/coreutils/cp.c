// Function: main @ 0x4050

byte main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  long lVar2;
  long lVar3;
  byte bVar4;
  int iVar5;
  char *pcVar6;
  long lVar7;
  undefined8 uVar8;
  int *piVar9;
  undefined8 uVar10;
  long in_FS_OFFSET;
  long local_c8;
  long local_c0;
  long local_b8;
  long local_b0;
  undefined1 local_a2;
  undefined1 local_a1;
  undefined4 local_98;
  int local_94;
  int local_90;
  int iStack_8c;
  undefined8 local_88;
  undefined2 local_80;
  undefined4 local_7c;
  undefined1 local_78;
  long local_70;
  undefined1 local_68;
  undefined1 uStack_67;
  undefined1 uStack_66;
  char cStack_65;
  char cStack_64;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  char cStack_60;
  undefined1 uStack_5f;
  char cStack_5e;
  undefined1 uStack_5d;
  undefined1 local_5c;
  undefined1 uStack_5b;
  undefined1 local_5a;
  int local_54;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00112680(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00119d70(FUN_0010e2b0);
  iVar5 = is_selinux_enabled();
  DAT_0012315a = 0 < iVar5;
  FUN_00107670(&local_98);
  local_78 = 0;
  local_54 = 1;
  local_90 = 4;
  iStack_8c = 2;
  local_7c = 0;
  local_94 = 1;
  local_88 = 0x100000000;
  local_80 = 0;
  local_70 = 0;
  local_68 = 0;
  uStack_67 = 1;
  uStack_66 = 0;
  cStack_65 = '\0';
  cStack_64 = '\0';
  uStack_63 = 0;
  uStack_62 = 0;
  uStack_61 = 0;
  cStack_60 = '\0';
  uStack_5f = 0;
  cStack_5e = '\0';
  uStack_5d = 0;
  local_5c = 0;
  uStack_5b = 0;
  pcVar6 = getenv("POSIXLY_CORRECT");
  local_a2 = 0;
  local_50 = 0;
  local_5a = pcVar6 != (char *)0x0;
  local_48 = 0;
  local_b0 = 0;
  local_c8 = 0;
  local_a1 = 0;
  local_c0 = 0;
  local_b8 = 0;
  bVar1 = false;
  lVar2 = local_c0;
  lVar3 = local_b0;
LAB_001041b8:
  do {
    while (local_b0 = lVar3, local_c0 = lVar2,
          iVar5 = getopt_long(param_1,param_2,"abdfHilLnprst:uvxPRS:TZ",&PTR_s_archive_001224a0,0),
          lVar2 = local_c0, lVar3 = local_b0, iVar5 == -1) {
LAB_00104538:
      if ((local_88._7_1_ == '\0') || (cStack_5e == '\0')) {
        if (local_90 == 2) {
          uStack_5d = 0;
          if (bVar1) goto LAB_00104854;
LAB_001046a9:
          local_98 = 0;
          if ((local_54 != 2) || (iStack_8c == 2)) {
LAB_00104586:
            FUN_0010d0f0(local_b8);
            if (local_94 == 1) goto LAB_001046ff;
            goto LAB_0010459f;
          }
        }
        else {
          if (!bVar1) goto LAB_001046a9;
          if ((local_54 != 2) || (iStack_8c == 2)) {
            uVar8 = dcgettext(0,"backup type",5);
            local_98 = FUN_0010d8d0(uVar8,local_c0);
            goto LAB_00104586;
          }
        }
        pcVar6 = "--reflink can be used only with --sparse=auto";
      }
      else {
        pcVar6 = "cannot make both hard and symbolic links";
      }
LAB_00104784:
      uVar8 = dcgettext(0,pcVar6,5);
      error(0,0,uVar8);
switchD_001041fb_caseD_49:
      FUN_001054f0(1);
LAB_001047a3:
      local_70 = selabel_open(0,0,0);
      lVar3 = local_b0;
      if (local_70 == 0) {
        uVar8 = dcgettext(0,"warning: ignoring --context",5);
        piVar9 = __errno_location();
        error(0,*piVar9,uVar8);
      }
    }
    if (0x88 < iVar5) goto switchD_001041fb_caseD_49;
    if (iVar5 < 0x48) {
      if (iVar5 == -0x83) {
        FUN_00116a20(stdout,&DAT_0011a053,"GNU coreutils",PTR_DAT_00123020,"Torbjorn Granlund",
                     "David MacKenzie","Jim Meyering",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar5 == -0x82) {
        FUN_001054f0(0);
        goto LAB_00104538;
      }
      goto switchD_001041fb_caseD_49;
    }
    switch(iVar5) {
    case 0x48:
      local_94 = 3;
      break;
    default:
      goto switchD_001041fb_caseD_49;
    case 0x4c:
      local_94 = 4;
      break;
    case 0x50:
      local_94 = 2;
      break;
    case 0x52:
    case 0x72:
      cStack_60 = '\x01';
      break;
    case 0x53:
      local_b8 = optarg;
      bVar1 = true;
      break;
    case 0x54:
      local_a2 = 1;
      break;
    case 0x5a:
      if (DAT_0012315a == '\0') {
        if (optarg != 0) {
          uVar8 = dcgettext(0,"warning: ignoring --context; it requires an SELinux-enabled kernel",5
                           );
          error(0,0,uVar8);
        }
      }
      else {
        lVar3 = optarg;
        if (optarg == 0) goto LAB_001047a3;
      }
      break;
    case 0x61:
      local_68 = 1;
      local_94 = 2;
      local_7c._0_2_ = CONCAT11(1,(undefined1)local_7c);
      local_7c = CONCAT22(0x101,(undefined2)local_7c);
      uStack_66 = 1;
      if (DAT_0012315a != '\0') {
        cStack_65 = '\x01';
      }
      uStack_63 = 1;
      uStack_61 = 1;
      cStack_60 = '\x01';
      break;
    case 0x62:
      bVar1 = true;
      lVar2 = optarg;
      if (optarg == 0) {
        lVar2 = local_c0;
      }
      break;
    case 100:
      local_68 = 1;
      local_94 = 2;
      break;
    case 0x66:
      local_88._0_7_ = CONCAT16(1,(undefined6)local_88);
      break;
    case 0x69:
      local_90 = 3;
      break;
    case 0x6c:
      local_88 = CONCAT17(1,(undefined7)local_88);
      break;
    case 0x6e:
      local_90 = 2;
      break;
    case 0x73:
      cStack_5e = '\x01';
      break;
    case 0x74:
      if (local_c8 != 0) {
        uVar8 = dcgettext(0,"multiple target directories specified",5);
        error(1,0,uVar8);
LAB_00104854:
        pcVar6 = "options --backup and --no-clobber are mutually exclusive";
        goto LAB_00104784;
      }
      local_c8 = optarg;
      break;
    case 0x75:
      uStack_5d = 1;
      break;
    case 0x76:
      local_5c = 1;
      break;
    case 0x78:
      local_7c = CONCAT31(local_7c._1_3_,1);
      break;
    case 0x80:
      uStack_67 = 0;
      break;
    case 0x81:
      local_a1 = 1;
      break;
    case 0x82:
      FUN_001049a0(optarg,&local_98,0);
      break;
    case 0x83:
      DAT_00123159 = 1;
      break;
    case 0x84:
      if (optarg != 0) {
        FUN_001049a0(optarg,&local_98,1);
        uStack_66 = 1;
        break;
      }
    case 0x70:
      local_7c._0_2_ = CONCAT11(1,(undefined1)local_7c);
      local_7c = CONCAT22(0x101,(undefined2)local_7c);
      uStack_66 = 1;
      break;
    case 0x85:
      if (optarg == 0) {
        local_54 = 2;
      }
      else {
        lVar7 = FUN_0010cfc0("--reflink",optarg,&PTR_DAT_00122860,&DAT_0011a460,4,PTR_FUN_00123028,1
                            );
        local_54 = *(int *)(&DAT_0011a460 + lVar7 * 4);
      }
      break;
    case 0x86:
      lVar7 = FUN_0010cfc0("--sparse",optarg,&PTR_s_never_00122880,&DAT_0011a470,4,PTR_FUN_00123028,
                           1);
      iStack_8c = *(int *)(&DAT_0011a470 + lVar7 * 4);
      break;
    case 0x87:
      DAT_00123158 = 1;
      break;
    case 0x88:
      goto switchD_001041fb_caseD_88;
    }
  } while( true );
LAB_0010459f:
  if (cStack_60 != '\0') goto LAB_00104715;
  while (local_b0 == 0 && local_70 == 0) {
    if ((cStack_64 == '\0') || (DAT_0012315a != '\0')) goto LAB_001045e6;
LAB_001046db:
    uVar8 = dcgettext(0,"cannot preserve security context without an SELinux-enabled kernel",5);
    error(1,0,uVar8);
LAB_001046ff:
    if ((cStack_60 == '\0') || (local_88._7_1_ != '\0')) goto LAB_0010474e;
    local_94 = 2;
LAB_00104715:
    local_88._0_5_ = CONCAT14(local_a1,(undefined4)local_88);
  }
  if (cStack_64 == '\0') {
    cStack_65 = '\0';
LAB_001045cc:
    if ((local_b0 != 0) && (iVar5 = setfscreatecon(local_b0), iVar5 < 0)) {
      uVar8 = FUN_00114b30(local_b0);
      uVar10 = dcgettext(0,"failed to set default file creation context to %s",5);
      piVar9 = __errno_location();
      error(1,*piVar9,uVar10,uVar8);
      goto LAB_001048a1;
    }
LAB_001045e6:
    FUN_0010bdb0();
    bVar4 = FUN_00105a50(param_1 - (int)optind,param_2 + (int)optind,local_c8,local_a2,&local_98);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return bVar4 ^ 1;
    }
LAB_001048a1:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if (cStack_65 == '\0') {
    if (DAT_0012315a == '\0') goto LAB_001046db;
    goto LAB_001045cc;
  }
  uVar8 = dcgettext(0,"cannot set target context and preserve it",5);
  error(1,0,uVar8);
LAB_0010474e:
  local_94 = 4;
  goto LAB_0010459f;
switchD_001041fb_caseD_88:
  local_88._0_6_ = CONCAT15(1,(undefined5)local_88);
  goto LAB_001041b8;
}




// Function: decode_preserve_arg @ 0x49a0

void decode_preserve_arg(undefined8 param_1,undefined8 param_2,char param_3)

{
  char *__s;
  char *pcVar1;
  char *pcVar2;
  long lVar3;
  
  __s = (char *)FUN_00117130();
  pcVar1 = "--no-preserve";
  if (param_3 != '\0') {
    pcVar1 = "--preserve";
  }
  pcVar2 = strchr(__s,0x2c);
  if (pcVar2 != (char *)0x0) {
    *pcVar2 = '\0';
  }
  lVar3 = FUN_0010cfc0(pcVar1,__s,&PTR_DAT_00122460,&DAT_0011a440,4,PTR_FUN_00123028);
  if (*(uint *)(&DAT_0011a440 + lVar3 * 4) < 7) {
                    /* WARNING: Could not recover jumptable at 0x00104a4d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_0011a320 +
              *(int *)(&DAT_0011a320 + (ulong)*(uint *)(&DAT_0011a440 + lVar3 * 4) * 4)))();
    return;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: make_dir_parents_private @ 0x4b10

undefined8
make_dir_parents_private(char *param_1,ulong param_2,int param_3,long param_4,__dev_t *param_5,char *param_6,
            long param_7)

{
  __dev_t _Var1;
  long lVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  size_t __n;
  undefined8 uVar7;
  size_t sVar8;
  void *__dest;
  char *pcVar9;
  int *piVar10;
  __dev_t *p_Var11;
  undefined8 uVar12;
  uint uVar13;
  ulong uVar14;
  stat **ppsVar15;
  undefined1 *puVar16;
  stat **ppsVar17;
  undefined1 *puVar19;
  char *pcVar21;
  uint uVar22;
  long in_FS_OFFSET;
  stat *local_1a8;
  long local_1a0;
  __dev_t *local_198;
  __dev_t *local_190;
  stat *local_188;
  char *local_180;
  char *local_178;
  void *local_170;
  stat local_168;
  stat local_d8;
  long local_40;
  stat **ppsVar18;
  undefined1 *puVar20;
  
  ppsVar18 = &local_1a8;
  ppsVar17 = &local_1a8;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_1a0 = param_4;
  local_190 = param_5;
  local_180 = param_6;
  __n = FUN_0010e4a0();
  *local_190 = 0;
  ppsVar15 = &local_1a8;
  if (param_2 < __n) {
    sVar8 = strlen(param_1);
    ppsVar15 = &local_1a8;
    while (ppsVar18 != (stat **)((long)&local_1a8 - (sVar8 + 0x18 & 0xfffffffffffff000))) {
      ppsVar17 = (stat **)((long)ppsVar15 + -0x1000);
      *(undefined8 *)((long)ppsVar15 + -8) = *(undefined8 *)((long)ppsVar15 + -8);
      ppsVar18 = (stat **)((long)ppsVar15 + -0x1000);
      ppsVar15 = (stat **)((long)ppsVar15 + -0x1000);
    }
    uVar14 = (ulong)((uint)(sVar8 + 0x18) & 0xff0);
    lVar2 = -uVar14;
    puVar19 = (undefined1 *)((long)ppsVar17 + lVar2);
    puVar20 = (undefined1 *)((long)ppsVar17 + lVar2);
    if (uVar14 != 0) {
      *(undefined8 *)((long)ppsVar17 + -8) = *(undefined8 *)((long)ppsVar17 + -8);
    }
    *(undefined8 *)((long)ppsVar17 + lVar2 + -8) = 0x104c05;
    local_170 = memcpy((void *)((ulong)((long)ppsVar17 + lVar2 + 0xf) & 0xfffffffffffffff0),param_1,
                       sVar8 + 1);
    puVar16 = (undefined1 *)((long)ppsVar17 + lVar2);
    while (puVar19 != (undefined1 *)((long)ppsVar17 + (lVar2 - (__n + 0x18 & 0xfffffffffffff000))))
    {
      puVar20 = puVar16 + -0x1000;
      *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
      puVar19 = puVar16 + -0x1000;
      puVar16 = puVar16 + -0x1000;
    }
    uVar14 = (ulong)((uint)(__n + 0x18) & 0xff0);
    lVar2 = -uVar14;
    ppsVar15 = (stat **)(puVar20 + lVar2);
    puVar16 = puVar20 + lVar2;
    if (uVar14 != 0) {
      *(undefined8 *)(puVar20 + -8) = *(undefined8 *)(puVar20 + -8);
    }
    __dest = (void *)((ulong)(puVar20 + lVar2 + 0xf) & 0xfffffffffffffff0);
    *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104c6f;
    memcpy(__dest,local_170,__n);
    pcVar21 = (char *)((long)__dest + param_2);
    *(undefined1 *)((long)__dest + __n) = 0;
    cVar3 = *pcVar21;
    while (cVar3 == '/') {
      pcVar21 = pcVar21 + 1;
      cVar3 = *pcVar21;
    }
    local_188 = &local_168;
    *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ca4;
    iVar4 = fstatat(param_3,pcVar21,local_188,0);
    if (iVar4 == 0) {
      if ((local_168.st_mode & 0xf000) != 0x4000) {
LAB_0010512c:
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105136;
        uVar7 = FUN_00114440(4,__dest);
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10514c;
        uVar12 = dcgettext(0,"%s exists but is not a directory",5);
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10515d;
        error(0,0,uVar12,uVar7);
LAB_0010515d:
        uVar7 = 0;
        puVar16 = puVar20 + lVar2;
        goto LAB_00104b75;
      }
      *local_180 = '\0';
    }
    else {
      local_178 = (char *)(param_2 + (long)local_170);
      cVar3 = *local_178;
      pcVar21 = local_178;
      while (cVar3 == '/') {
        pcVar21 = pcVar21 + 1;
        cVar3 = *pcVar21;
      }
      local_1a8 = &local_d8;
      pcVar9 = pcVar21;
      while( true ) {
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104cfa;
        pcVar9 = strchr(pcVar9,0x2f);
        ppsVar15 = (stat **)(puVar20 + lVar2);
        if (pcVar9 == (char *)0x0) break;
        *pcVar9 = '\0';
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104d1d;
        iVar4 = fstatat(param_3,pcVar21,local_188,0);
        if ((iVar4 == 0) && ((*(uint *)(param_7 + 0x1c) & 0xffffff00) == 0)) {
LAB_00104d37:
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104d51;
          cVar3 = FUN_001072a0(local_178,local_170,0,0,param_7);
          if (cVar3 == '\0') goto LAB_0010515d;
          __dest = local_170;
          if ((local_168.st_mode & 0xf000) != 0x4000) goto LAB_0010512c;
          *local_180 = '\0';
LAB_00104d79:
          if ((*(long *)(param_7 + 0x28) != 0) || (*(char *)(param_7 + 0x33) != '\0')) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104d96;
            cVar3 = FUN_00107510(local_170,0,param_7);
            if ((cVar3 == '\0') && (*(char *)(param_7 + 0x34) != '\0')) goto LAB_0010515d;
          }
        }
        else {
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104df3;
          iVar5 = stat(local_178,local_1a8);
          if (iVar5 == 0) {
            if ((local_d8.st_mode & 0xf000) != 0x4000) {
              iVar5 = 0x14;
              goto LAB_0010503c;
            }
          }
          else {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104e00;
            piVar10 = __errno_location();
            iVar5 = *piVar10;
            if (iVar5 != 0) {
LAB_0010503c:
              local_170 = (void *)CONCAT44(local_170._4_4_,iVar5);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105054;
              uVar7 = FUN_00114440(4,local_178);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10506a;
              uVar12 = dcgettext(0,"failed to get attributes of %s",5);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10507f;
              error(0,(ulong)local_170 & 0xffffffff,uVar12,uVar7);
              uVar7 = 0;
              goto LAB_00104b75;
            }
          }
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104e16;
          p_Var11 = (__dev_t *)FUN_00116c50(0xa8);
          *p_Var11 = local_d8.st_dev;
          p_Var11[1] = local_d8.st_ino;
          p_Var11[2] = local_d8.st_nlink;
          p_Var11[3] = CONCAT44(local_d8.st_uid,local_d8.st_mode);
          p_Var11[4] = local_d8._32_8_;
          p_Var11[5] = local_d8.st_rdev;
          p_Var11[6] = local_d8.st_size;
          p_Var11[7] = local_d8.st_blksize;
          p_Var11[8] = local_d8.st_blocks;
          p_Var11[9] = local_d8.st_atim.tv_sec;
          p_Var11[10] = local_d8.st_atim.tv_nsec;
          p_Var11[0xb] = local_d8.st_mtim.tv_sec;
          p_Var11[0xc] = local_d8.st_mtim.tv_nsec;
          p_Var11[0xd] = local_d8.st_ctim.tv_sec;
          p_Var11[0xe] = local_d8.st_ctim.tv_nsec;
          p_Var11[0xf] = local_d8.__unused[0];
          p_Var11[0x10] = local_d8.__unused[1];
          p_Var11[0x11] = local_d8.__unused[2];
          p_Var11[0x13] = (long)pcVar9 - (long)local_170;
          _Var1 = *local_190;
          *(undefined1 *)(p_Var11 + 0x12) = 0;
          p_Var11[0x14] = _Var1;
          *local_190 = (__dev_t)p_Var11;
          if (iVar4 == 0) goto LAB_00104d37;
          _Var1 = p_Var11[3];
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ed7;
          local_198 = p_Var11;
          cVar3 = FUN_001072a0(local_178,local_170,(int)_Var1,1,param_7);
          if (cVar3 == '\0') goto LAB_0010515d;
          *local_180 = '\x01';
          uVar6 = (uint)local_198[3];
          if (*(char *)(param_7 + 0x1d) == '\0') {
            if (*(char *)(param_7 + 0x1e) == '\0') {
              uVar13 = 0xffffffff;
              uVar22 = 0;
            }
            else {
              uVar22 = uVar6 & 0x12;
              uVar13 = ~uVar22;
            }
          }
          else {
            uVar22 = uVar6 & 0x3f;
            uVar13 = ~uVar22;
          }
          if (*(char *)(param_7 + 0x20) != '\0') {
            uVar6 = 0x1ff;
          }
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104f34;
          iVar4 = mkdirat(param_3,pcVar21,uVar6 & uVar13 & 0xfff);
          if (iVar4 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1051d4;
            uVar7 = FUN_00114440(4,local_170);
            pcVar21 = "cannot make directory %s";
LAB_00105190:
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105197;
            uVar12 = dcgettext(0,pcVar21,5);
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10519f;
            piVar10 = __errno_location();
            iVar4 = *piVar10;
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1051b0;
            error(0,iVar4,uVar12,uVar7);
            uVar7 = 0;
            puVar16 = puVar20 + lVar2;
            goto LAB_00104b75;
          }
          if (local_1a0 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104f70;
            __printf_chk(1,local_1a0,local_178,local_170);
          }
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104f95;
          iVar4 = fstatat(param_3,pcVar21,local_188,0x100);
          if (iVar4 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105181;
            uVar7 = FUN_00114440(4,local_170);
            pcVar21 = "failed to get attributes of %s";
            goto LAB_00105190;
          }
          if (*(char *)(param_7 + 0x1e) == '\0') {
            if ((~local_168.st_mode & uVar22) == 0) {
LAB_00104fbf:
              if ((local_168.st_mode & 0x1c0) == 0x1c0) goto LAB_00104fde;
            }
            else {
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1050e1;
              uVar6 = FUN_0010bbd0();
              uVar22 = uVar22 & ~uVar6;
              if ((~local_168.st_mode & uVar22) == 0) goto LAB_00104fbf;
            }
            *(undefined1 *)(local_198 + 0x12) = 1;
            *(uint *)(local_198 + 3) = uVar22 | local_168.st_mode;
          }
LAB_00104fde:
          if ((local_168.st_mode | 0x1c0) != local_168.st_mode) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ffa;
            iVar4 = fchmodat(param_3,pcVar21,local_168.st_mode | 0x1c0,0x100);
            if (iVar4 != 0) {
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1051f6;
              uVar7 = FUN_00114440(4,local_170);
              pcVar21 = "setting permissions for %s";
              goto LAB_00105190;
            }
          }
          if (*local_180 == '\0') goto LAB_00104d79;
        }
        cVar3 = pcVar9[1];
        *pcVar9 = '/';
        pcVar9 = pcVar9 + 1;
        while (cVar3 == '/') {
          pcVar9 = pcVar9 + 1;
          cVar3 = *pcVar9;
        }
      }
    }
  }
  uVar7 = 1;
  puVar16 = (undefined1 *)ppsVar15;
LAB_00104b75:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined **)(puVar16 + -8) = &UNK_0010520c;
  __stack_chk_fail();
}




// Function: re_protect @ 0x5210

ulong re_protect(char *param_1,int param_2,ulong param_3,long param_4,long param_5)

{
  __gid_t _Var1;
  __uid_t __owner;
  undefined4 uVar2;
  __mode_t __mode;
  long lVar3;
  timespec **pptVar4;
  char cVar5;
  int iVar6;
  size_t sVar7;
  void *pvVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  int *piVar11;
  timespec **pptVar12;
  char *pcVar14;
  ulong uVar15;
  long in_FS_OFFSET;
  timespec *local_78;
  ulong local_70;
  timespec local_68;
  undefined8 local_58;
  undefined8 local_50;
  long local_40;
  timespec **pptVar13;
  
  pptVar12 = &local_78;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_70 = param_3;
  sVar7 = strlen(param_1);
  pptVar13 = &local_78;
  pptVar4 = &local_78;
  while (pptVar13 != (timespec **)((long)&local_78 - (sVar7 + 0x18 & 0xfffffffffffff000))) {
    pptVar12 = (timespec **)((long)pptVar4 + -0x1000);
    *(undefined8 *)((long)pptVar4 + -8) = *(undefined8 *)((long)pptVar4 + -8);
    pptVar13 = (timespec **)((long)pptVar4 + -0x1000);
    pptVar4 = (timespec **)((long)pptVar4 + -0x1000);
  }
  uVar15 = (ulong)((uint)(sVar7 + 0x18) & 0xff0);
  lVar3 = -uVar15;
  if (uVar15 != 0) {
    *(undefined8 *)((long)pptVar12 + -8) = *(undefined8 *)((long)pptVar12 + -8);
  }
  *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x1052a0;
  pvVar8 = memcpy((void *)((ulong)((long)pptVar12 + lVar3 + 0xf) & 0xfffffffffffffff0),param_1,
                  sVar7 + 1);
  pcVar14 = (char *)((long)pvVar8 + (local_70 - (long)param_1));
  if (param_4 != 0) {
    local_78 = &local_68;
    do {
      cVar5 = *(char *)(param_5 + 0x1f);
      *(undefined1 *)((long)pvVar8 + *(long *)(param_4 + 0x98)) = 0;
      if (cVar5 != '\0') {
        local_68.tv_sec = *(__time_t *)(param_4 + 0x48);
        local_68.tv_nsec = *(long *)(param_4 + 0x50);
        local_58 = *(undefined8 *)(param_4 + 0x58);
        local_50 = *(undefined8 *)(param_4 + 0x60);
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x10534e;
        iVar6 = utimensat(param_2,pcVar14,local_78,0);
        if (iVar6 != 0) {
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105363;
          uVar9 = FUN_00114440(4,pvVar8);
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105379;
          uVar10 = dcgettext(0,"failed to preserve times for %s",5);
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105381;
          piVar11 = __errno_location();
          iVar6 = *piVar11;
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105392;
          error(0,iVar6,uVar10,uVar9);
          uVar15 = 0;
          goto LAB_0010549e;
        }
      }
      if (*(char *)(param_5 + 0x1d) != '\0') {
        _Var1 = *(__gid_t *)(param_4 + 0x20);
        __owner = *(__uid_t *)(param_4 + 0x1c);
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x1053b7;
        iVar6 = fchownat(param_2,pcVar14,__owner,_Var1,0x100);
        if (iVar6 == 0) goto LAB_001052d3;
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x1053c7;
        cVar5 = FUN_001076c0(param_5);
        if (cVar5 != '\0') {
          _Var1 = *(__gid_t *)(param_4 + 0x20);
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x1053e8;
          fchownat(param_2,pcVar14,0xffffffff,_Var1,0x100);
          cVar5 = *(char *)(param_5 + 0x1e);
          goto joined_r0x001053f0;
        }
        local_70 = local_70 & 0xffffffffffffff00;
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x1054da;
        uVar9 = FUN_00114440(4,pvVar8);
        pcVar14 = "failed to preserve ownership for %s";
LAB_0010546b:
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105472;
        uVar10 = dcgettext(0,pcVar14,5);
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x10547a;
        piVar11 = __errno_location();
        iVar6 = *piVar11;
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x10548b;
        error(0,iVar6,uVar10,uVar9);
        uVar15 = local_70 & 0xff;
        goto LAB_0010549e;
      }
LAB_001052d3:
      cVar5 = *(char *)(param_5 + 0x1e);
joined_r0x001053f0:
      if (cVar5 == '\0') {
        if (*(char *)(param_4 + 0x90) != '\0') {
          __mode = *(__mode_t *)(param_4 + 0x18);
          local_70 = local_70 & 0xffffffffffffff00;
          *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105447;
          iVar6 = fchmodat(param_2,pcVar14,__mode,0x100);
          if (iVar6 != 0) {
            *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x10545c;
            uVar9 = FUN_00114440(4,pvVar8);
            pcVar14 = "failed to preserve permissions for %s";
            goto LAB_0010546b;
          }
        }
      }
      else {
        uVar2 = *(undefined4 *)(param_4 + 0x18);
        *(undefined8 *)((long)pptVar12 + lVar3 + -8) = 0x105419;
        iVar6 = FUN_0010c830(pcVar14,0xffffffff,pvVar8,0xffffffff,uVar2);
        if (iVar6 != 0) {
          uVar15 = 0;
          goto LAB_0010549e;
        }
      }
      *(undefined1 *)((long)pvVar8 + *(long *)(param_4 + 0x98)) = 0x2f;
      param_4 = *(long *)(param_4 + 0xa0);
    } while (param_4 != 0);
  }
  uVar15 = 1;
LAB_0010549e:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(code **)((long)pptVar12 + lVar3 + -8) = FUN_001054f0;
    __stack_chk_fail();
  }
  return uVar15;
}




// Function: usage @ 0x54f0

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
  
  uVar5 = DAT_001235a0;
  ppuVar6 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,
                      "Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n"
                      ,5);
    __printf_chk(1,uVar3,uVar5,uVar5,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don\'t copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --no-preserve=ATTR_LIST  don\'t preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe backup suffix is \'~\', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    local_b8 = &DAT_0011a051;
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
      iVar2 = strcmp("cp",pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar6 + 0x10);
      ppuVar6 = (undefined **)((long)ppuVar6 + 0x10);
    } while (pcVar4 != (char *)0x0);
    puVar8 = *(undefined **)((long)ppuVar6 + 8);
    if (puVar8 == (undefined *)0x0) {
      puVar8 = &DAT_0011a053;
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
    puVar7 = &DAT_0011a053;
    iVar2 = strcmp("cp","[");
    if (iVar2 == 0) {
      puVar7 = &DAT_0011a056;
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar4 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",puVar7);
    if (puVar8 != &DAT_0011a053) {
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




// Function: do_copy @ 0x5a50

uint do_copy(int param_1,undefined8 *param_2,long param_3,char param_4,int *param_5)

{
  char cVar1;
  void *pvVar2;
  long lVar3;
  void *__ptr;
  byte bVar4;
  char cVar5;
  int iVar6;
  uint uVar7;
  char *pcVar8;
  size_t sVar9;
  void *pvVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  int *piVar13;
  ulong uVar14;
  long lVar15;
  undefined1 *puVar16;
  byte **ppbVar17;
  byte **ppbVar18;
  byte **ppbVar19;
  char *pcVar20;
  int *piVar21;
  long lVar22;
  uint uVar23;
  long in_FS_OFFSET;
  byte bVar24;
  byte *local_128;
  char **local_120;
  undefined1 *local_118;
  long local_110;
  int *local_108;
  int local_100;
  int local_fc;
  byte local_ea;
  undefined1 local_e9;
  void *local_e8;
  char *local_e0;
  stat local_d8;
  long local_40;
  
  bVar24 = 0;
  ppbVar18 = &local_128;
  ppbVar19 = &local_128;
  ppbVar17 = &local_128;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_ea = 0;
  local_110 = param_3;
  local_108 = param_5;
  local_fc = param_1;
  if ((int)(uint)(param_3 == 0) < param_1) {
    local_d8.st_mode = 0;
    if (param_4 != '\0') {
      if (param_3 == 0) {
        if (param_1 < 3) goto LAB_00105c8d;
      }
      else {
        uVar11 = dcgettext(0,"cannot combine --target-directory (-t) and --no-target-directory (-T)"
                           ,5);
        param_2 = (undefined8 *)0x0;
        error(1,0,uVar11);
      }
      uVar11 = FUN_00114440(4,param_2[2]);
      pcVar20 = "extra operand %s";
LAB_00106026:
      uVar12 = dcgettext(0,pcVar20,5);
      error(0,0,uVar12,uVar11);
      goto LAB_0010603e;
    }
    if (param_3 == 0) {
      lVar15 = param_2[(long)param_1 + -1];
      local_110 = lVar15;
      local_100 = FUN_001156a0(lVar15,&local_d8);
      if (local_100 == -1) {
        piVar13 = __errno_location();
        iVar6 = *piVar13;
        if (iVar6 == 2) goto LAB_00105fc0;
        while (2 < local_fc) {
          param_2 = (undefined8 *)FUN_00114440(4,local_110);
          uVar11 = dcgettext(0,"target %s",5);
          error(1,iVar6,uVar11,param_2);
LAB_00105fc0:
          local_ea = 1;
        }
      }
      else {
        local_fc = param_1 + -1;
        if (lVar15 != 0) {
          if (1 < local_fc) goto LAB_00105eef;
          uVar23 = 1;
          if (local_fc == 1) goto LAB_00105aeb;
          goto LAB_00105c4d;
        }
      }
LAB_00105c8d:
      bVar4 = local_ea;
      pcVar20 = (char *)*param_2;
      pcVar8 = (char *)param_2[1];
      if (DAT_00123159 != '\0') {
        pcVar20 = "with --parents, the destination must be a directory";
        goto LAB_001060a1;
      }
      if (((((*(char *)((long)local_108 + 0x16) != '\0') && (*local_108 != 0)) &&
           (iVar6 = strcmp(pcVar20,pcVar8), iVar6 == 0)) &&
          ((bVar4 == 0 && ((local_d8.st_mode != 0 || (iVar6 = stat(pcVar8,&local_d8), iVar6 == 0))))
          )) && (piVar13 = local_108, (local_d8.st_mode & 0xf000) == 0x8000)) {
        pcVar8 = (char *)FUN_0010d850(0xffffff9c,pcVar8,*local_108);
        piVar21 = &DAT_00123100;
        for (lVar15 = 0x16; lVar15 != 0; lVar15 = lVar15 + -1) {
          *piVar21 = *piVar13;
          piVar13 = piVar13 + (ulong)bVar24 * -2 + 1;
          piVar21 = piVar21 + (ulong)bVar24 * -2 + 1;
        }
        DAT_00123100 = 0;
        local_108 = &DAT_00123100;
      }
      uVar23 = FUN_0010ba80(pcVar20,pcVar8,0xffffff9c,pcVar8,-(uint)bVar4,local_108,&local_e0,0);
      ppbVar18 = &local_128;
LAB_00105c4d:
      ppbVar19 = ppbVar18;
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar23;
      }
      goto LAB_0010608b;
    }
    local_100 = FUN_001156a0(param_3,&local_d8);
    if (local_100 != -1) {
      if (local_fc != 1) {
LAB_00105eef:
        piVar13 = local_108;
        FUN_001075f0(local_108);
        FUN_00107630(piVar13);
      }
LAB_00105aeb:
      local_120 = &local_e0;
      lVar15 = 0;
      uVar23 = 1;
      local_118 = &local_e9;
      local_128 = &local_ea;
      do {
        pcVar20 = (char *)param_2[lVar15];
        local_e0 = (char *)0x0;
        if (DAT_00123158 == '\0') {
          if (DAT_00123159 != '\0') goto LAB_00105d80;
LAB_00105b4d:
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105b52;
          pcVar8 = (char *)FUN_0010d970();
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105b5d;
          sVar9 = strlen(pcVar8);
          for (puVar16 = (undefined1 *)ppbVar17;
              puVar16 != (undefined1 *)((long)ppbVar17 + -(sVar9 + 0x18 & 0xfffffffffffff000));
              puVar16 = puVar16 + -0x1000) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          uVar14 = (ulong)((uint)(sVar9 + 0x18) & 0xff0);
          lVar3 = -uVar14;
          ppbVar17 = (byte **)(puVar16 + lVar3);
          if (uVar14 != 0) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105bba;
          pcVar8 = memcpy((void *)((ulong)(puVar16 + lVar3 + 0xf) & 0xfffffffffffffff0),pcVar8,
                          sVar9 + 1);
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105bc5;
          FUN_0010e5a0(pcVar8);
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105bd4;
          iVar6 = strcmp(pcVar8,"..");
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105bf1;
          pvVar10 = (void *)FUN_0010ea10(local_110,pcVar8 + (iVar6 == 0),local_120);
LAB_00105bf4:
          *(undefined8 *)((long)ppbVar17 + -8) = 0;
          *(undefined1 **)((long)ppbVar17 + -0x10) = local_118;
          *(undefined8 *)((long)ppbVar17 + -0x18) = 0x105c23;
          uVar7 = FUN_0010ba80(pcVar20,pvVar10,local_100,local_e0,local_ea,local_108);
          uVar23 = uVar23 & uVar7;
          if (DAT_00123159 != '\0') {
            *(undefined8 *)((long)ppbVar17 + -8) = 0x105ee3;
            uVar7 = FUN_00105210(pvVar10,local_100,local_e0,local_e8,local_108);
            uVar23 = uVar23 & uVar7;
            goto LAB_00105e83;
          }
        }
        else {
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105d70;
          FUN_0010e5a0(pcVar20);
          if (DAT_00123159 == '\0') goto LAB_00105b4d;
LAB_00105d80:
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105d85;
          sVar9 = strlen(pcVar20);
          for (puVar16 = (undefined1 *)ppbVar17;
              puVar16 != (undefined1 *)((long)ppbVar17 + -(sVar9 + 0x18 & 0xfffffffffffff000));
              puVar16 = puVar16 + -0x1000) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          uVar14 = (ulong)((uint)(sVar9 + 0x18) & 0xff0);
          lVar3 = -uVar14;
          ppbVar17 = (byte **)(puVar16 + lVar3);
          if (uVar14 != 0) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105de2;
          pvVar10 = memcpy((void *)((ulong)(puVar16 + lVar3 + 0xf) & 0xfffffffffffffff0),pcVar20,
                           sVar9 + 1);
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105ded;
          FUN_0010e5a0(pvVar10);
          *(undefined8 *)(puVar16 + lVar3 + -8) = 0x105e03;
          lVar22 = local_110;
          pvVar10 = (void *)FUN_0010ea10(local_110,pvVar10,local_120);
          pcVar8 = (char *)0x0;
          iVar6 = local_108[0xf];
          *(long *)(puVar16 + lVar3 + -8) = lVar22;
          if ((char)iVar6 != '\0') {
            pcVar8 = "%s -> %s\n";
          }
          *(int **)(puVar16 + lVar3 + -0x10) = local_108;
          *(undefined8 *)(puVar16 + lVar3 + -0x18) = 0x105e46;
          cVar5 = FUN_00104b10(pvVar10,(long)local_e0 - (long)pvVar10,local_100,pcVar8,&local_e8,
                               local_128);
          cVar1 = *local_e0;
          while (cVar1 == '/') {
            local_e0 = local_e0 + 1;
            cVar1 = *local_e0;
          }
          if (cVar5 != '\0') goto LAB_00105bf4;
          uVar23 = 0;
          ppbVar17 = (byte **)(puVar16 + lVar3);
LAB_00105e83:
          __ptr = local_e8;
          if (DAT_00123159 != '\0') {
            while (__ptr != (void *)0x0) {
              pvVar2 = *(void **)((long)__ptr + 0xa0);
              *(undefined8 *)((long)ppbVar17 + -8) = 0x105eb6;
              local_e8 = pvVar2;
              free(__ptr);
              __ptr = pvVar2;
            }
          }
        }
        lVar15 = lVar15 + 1;
        *(undefined8 *)((long)ppbVar17 + -8) = 0x105c41;
        free(pvVar10);
        ppbVar18 = ppbVar17;
      } while ((int)lVar15 < local_fc);
      goto LAB_00105c4d;
    }
  }
  else {
    if (param_1 == 1) {
      uVar11 = FUN_00114440(4,*param_2);
      pcVar20 = "missing destination file operand after %s";
      goto LAB_00106026;
    }
    pcVar20 = "missing file operand";
LAB_001060a1:
    uVar11 = dcgettext(0,pcVar20,5);
    error(0,0,uVar11);
LAB_0010603e:
    FUN_001054f0(1);
  }
  uVar11 = FUN_00114440(4,local_110);
  uVar12 = dcgettext(0,"target directory %s",5);
  piVar13 = __errno_location();
  error(1,*piVar13,uVar12,uVar11);
LAB_0010608b:
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)((long)ppbVar19 + -8) = 0x106090;
  __stack_chk_fail();
}




// Function: copy_attr_free @ 0x60f0

void copy_attr_free(void)

{
  return;
}




// Function: copy_attr_quote @ 0x6100

void copy_attr_quote(void)

{
  FUN_00114440(4);
  return;
}




// Function: subst_suffix @ 0x6110

void subst_suffix(void *param_1,long param_2,char *param_3)

{
  size_t sVar1;
  void *__dest;
  size_t __n;
  
  __n = param_2 - (long)param_1;
  sVar1 = strlen(param_3);
  __dest = (void *)FUN_00116c70(sVar1 + 1 + __n);
  memcpy((void *)((long)__dest + __n),param_3,sVar1 + 1);
  memcpy(__dest,param_1,__n);
  return;
}




// Function: write_zeros @ 0x6170

undefined8 write_zeros(undefined4 param_1,ulong param_2)

{
  ulong uVar1;
  ulong uVar2;
  
  if ((DAT_00123560 == (undefined *)0x0) &&
     (DAT_00123560 = calloc(DAT_00123018,1), DAT_00123560 == (undefined *)0x0)) {
    DAT_00123018 = 0x400;
    DAT_00123560 = &DAT_00123160;
  }
  while( true ) {
    if (param_2 == 0) {
      return 1;
    }
    uVar2 = DAT_00123018;
    if (param_2 <= DAT_00123018) {
      uVar2 = param_2;
    }
    uVar1 = FUN_001111c0(param_1,DAT_00123560,uVar2);
    if (uVar1 != uVar2) break;
    param_2 = param_2 - uVar1;
  }
  return 0;
}




// Function: copy_attr @ 0x6220

bool copy_attr(undefined8 param_1,undefined8 param_2,undefined8 param_3,uint param_4,long param_5
                 )

{
  char cVar1;
  int iVar2;
  code *pcVar3;
  code **ppcVar4;
  long in_FS_OFFSET;
  code *local_28;
  code *local_20;
  code *local_18;
  long local_10;
  
  ppcVar4 = &local_28;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = *(char *)(param_5 + 0x33);
  if ((*(char *)(param_5 + 0x31) == '\0') || (*(char *)(param_5 + 0x36) != '\0')) {
    pcVar3 = FUN_00106720;
    local_28 = FUN_00106440;
    if (cVar1 == '\0') {
      pcVar3 = FUN_00106720;
      if (*(long *)(param_5 + 0x28) == 0) {
        pcVar3 = (code *)0x0;
      }
      local_28 = FUN_00106440;
    }
  }
  else {
    if (*(char *)(param_5 + 0x37) != '\0') {
      pcVar3 = FUN_00106720;
      ppcVar4 = (code **)0x0;
      if (cVar1 == '\0') {
        pcVar3 = (code *)0x0;
        ppcVar4 = (code **)0x0;
        if (*(long *)(param_5 + 0x28) != 0) {
          pcVar3 = FUN_00106720;
          ppcVar4 = (code **)0x0;
        }
      }
      goto LAB_00106282;
    }
    pcVar3 = FUN_00106720;
    local_28 = FUN_00106360;
    if ((cVar1 == '\0') && (*(long *)(param_5 + 0x28) == 0)) {
      pcVar3 = (code *)0x0;
    }
  }
  local_20 = FUN_00106100;
  local_18 = FUN_001060f0;
LAB_00106282:
  if ((int)((uint)param_2 | param_4) < 0) {
    iVar2 = attr_copy_file(param_1,param_3,pcVar3,ppcVar4);
  }
  else {
    iVar2 = attr_copy_fd(param_1,param_2,param_3);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar2 == 0;
}




// Function: copy_attr_error @ 0x6360

void copy_attr_error(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  int iVar1;
  char in_AL;
  int *piVar2;
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
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  if ((iVar1 != 0x5f) && (iVar1 != 0x3d)) {
    local_d0 = &stack0x00000008;
    local_c8 = local_b8;
    local_d8 = 0x10;
    local_d4 = 0x30;
    FUN_001163b0(0,iVar1,param_10,&local_d8);
  }
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy_attr_allerror @ 0x6440

void copy_attr_allerror(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  int *piVar1;
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
  piVar1 = __errno_location();
  local_d0 = &stack0x00000008;
  local_d8 = 0x10;
  local_c8 = local_b8;
  local_d4 = 0x30;
  FUN_001163b0(0,*piVar1,param_10,&local_d8);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: emit_verbose @ 0x6510

void emit_verbose(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  uVar2 = FUN_00114310(1,4,param_2);
  uVar3 = FUN_00114310(0,4,param_1);
  __printf_chk(1,"%s -> %s",uVar3,uVar2);
  if (param_3 != 0) {
    uVar2 = FUN_00114440(4,param_3);
    uVar3 = dcgettext(0," (backup: %s)",5);
    __printf_chk(1,uVar3,uVar2);
  }
  pcVar1 = stdout->_IO_write_ptr;
  if (pcVar1 < stdout->_IO_write_end) {
    stdout->_IO_write_ptr = pcVar1 + 1;
    *pcVar1 = '\n';
    return;
  }
  __overflow(stdout,10);
  return;
}




// Function: create_hard_link @ 0x65d0

byte create_hard_link(void *param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,
                 undefined4 param_5,undefined8 param_6,undefined8 param_7,byte param_8,byte param_9)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  void *__ptr;
  undefined8 uStack_50;
  
  iVar1 = FUN_0010be90(param_2,param_3,param_5,param_6,(ulong)param_9 << 10);
  if (iVar1 < 1) {
    param_8 = param_8 & (byte)((uint)iVar1 >> 0x1f);
    if (param_8 == 0) {
      param_8 = 1;
    }
    else {
      uVar3 = FUN_00114440(4,param_4,0xffffffffffffffff);
      uVar4 = dcgettext(0,"removed %s\n",5);
      __printf_chk(1,uVar4,uVar3);
    }
  }
  else {
    __ptr = (void *)0x0;
    if (param_1 == (void *)0x0) {
      __ptr = (void *)FUN_00106110(param_4,param_6,param_3,uStack_50);
      param_1 = __ptr;
    }
    uVar3 = FUN_00114310(1,4,param_1);
    uVar4 = FUN_00114310(0,4,param_4);
    uVar2 = dcgettext(0,"cannot create hard link %s to %s",5);
    param_8 = 0;
    error(0,iVar1,uVar2,uVar4,uVar3);
    free(__ptr);
  }
  return param_8;
}




// Function: overwrite_ok @ 0x67a0

void overwrite_ok(long param_1,undefined8 param_2,int param_3,char *param_4,long param_5)

{
  uint uVar1;
  undefined8 uVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_3c;
  undefined1 local_3b [9];
  undefined1 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(uint *)(param_5 + 0x18) & 0xf000) != 0xa000) {
    cVar3 = FUN_00116bb0();
    if (cVar3 == '\0') {
      iVar4 = faccessat(param_3,param_4,2,0x200);
      if (iVar4 != 0) {
        FUN_0010e880(*(undefined4 *)(param_5 + 0x18),&local_3c);
        uVar1 = *(uint *)(param_5 + 0x18);
        local_32 = 0;
        uVar5 = FUN_00114440(4,param_2);
        uVar2 = DAT_001235a0;
        if ((*(char *)(param_1 + 0x18) == '\0') && ((*(uint *)(param_1 + 0x14) & 0xffff00) == 0)) {
          uVar6 = dcgettext(0,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
        }
        else {
          uVar6 = dcgettext(0,"%s: replace %s, overriding mode %04lo (%s)? ",5);
        }
        __fprintf_chk(stderr,1,uVar6,uVar2,uVar5,uVar1 & 0xfff,local_3b);
        goto LAB_00106813;
      }
    }
  }
  uVar5 = FUN_00114440(4,param_2);
  uVar2 = DAT_001235a0;
  uVar6 = dcgettext(0,"%s: overwrite %s? ",5);
  __fprintf_chk(stderr,1,uVar6,uVar2,uVar5);
LAB_00106813:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00117440();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: create_hole @ 0x6920

ulong create_hole(int param_1,undefined8 param_2,char param_3,long param_4)

{
  int iVar1;
  __off_t _Var2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulong uVar6;
  
  _Var2 = lseek(param_1,param_4,1);
  if (_Var2 < 0) {
    uVar4 = FUN_00114440(4,param_2);
    uVar5 = dcgettext(0,"cannot lseek %s",5);
    piVar3 = __errno_location();
    error(0,*piVar3,uVar5,uVar4);
    return 0;
  }
  if (param_3 != '\0') {
    iVar1 = fallocate(param_1,3,_Var2 - param_4,param_4);
    if (iVar1 < 0) {
      piVar3 = __errno_location();
      iVar1 = *piVar3;
      uVar6 = CONCAT71((int7)((ulong)param_4 >> 8),iVar1 == 0x5f || iVar1 == 0x26);
      if (iVar1 != 0x5f && iVar1 != 0x26) {
        uVar4 = FUN_00114440(4,param_2);
        uVar5 = dcgettext(0,"error deallocating %s",5);
        error(0,*piVar3,uVar5,uVar4);
        goto LAB_00106955;
      }
    }
  }
  uVar6 = 1;
LAB_00106955:
  return uVar6 & 0xffffffff;
}




// Function: sparse_copy @ 0x6a30

ulong sparse_copy(int param_1,undefined4 param_2,undefined8 *param_3,ulong param_4,ulong param_5,
                  undefined1 param_6,char param_7,undefined8 param_8,undefined8 param_9,
                  ulong param_10,long *param_11,byte *param_12)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  long lVar5;
  ulong uVar6;
  ulong uVar7;
  char *__buf;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  ulong uVar12;
  bool bVar13;
  bool bVar14;
  size_t __n;
  ulong uVar15;
  ulong uVar16;
  char *__s2;
  byte bVar17;
  char *pcVar18;
  bool bVar19;
  ulong uVar20;
  ulong local_98;
  
  *param_12 = 0;
  local_98 = param_10;
  *param_11 = 0;
  if ((param_5 == 0) && (param_7 != '\0')) {
    if (param_10 != 0) {
      do {
        while( true ) {
          uVar12 = 0x7fffffffc0000000;
          if (param_10 < 0x7fffffffc0000001) {
            uVar12 = param_10;
          }
          lVar5 = copy_file_range(param_1,0,param_2,0,uVar12);
          local_98 = param_10;
          if (lVar5 == 0) {
            if (*param_11 != 0) {
              return 1;
            }
            goto LAB_00106b28;
          }
          if (lVar5 < 0) break;
          *param_11 = *param_11 + lVar5;
          param_10 = param_10 - lVar5;
          if (param_10 == 0) {
            return 1;
          }
        }
        piVar8 = __errno_location();
        iVar3 = *piVar8;
        if (iVar3 == 0x26) goto LAB_00106b28;
        if (0x1a < iVar3) {
          if (iVar3 == 0x5f) goto LAB_00106b28;
          break;
        }
        if (0 < iVar3) {
          if ((1L << ((byte)iVar3 & 0x3f) & 0x4440200U) != 0) goto LAB_00106b28;
          if (iVar3 == 1) {
            if (*param_11 == 0) goto LAB_00106b28;
            break;
          }
        }
      } while (iVar3 == 4);
      uVar9 = FUN_00114310(1,4,param_9);
      uVar10 = FUN_00114310(0,4,param_8);
      uVar11 = dcgettext(0,"error copying %s to %s",5);
      error(0,*piVar8,uVar11,uVar10,uVar9);
      return 0;
    }
  }
  else if (param_10 != 0) {
LAB_00106b28:
    uVar12 = param_4;
    if (param_5 != 0) {
      uVar12 = param_5;
    }
    uVar20 = 0;
    bVar19 = false;
    do {
      __buf = (char *)*param_3;
      while( true ) {
        if (__buf == (char *)0x0) {
          iVar3 = getpagesize();
          __buf = (char *)FUN_00116bf0((long)iVar3,param_4);
          *param_3 = __buf;
        }
        uVar6 = param_4;
        if (local_98 <= param_4) {
          uVar6 = local_98;
        }
        uVar6 = read(param_1,__buf,uVar6);
        if ((long)uVar6 < 0) break;
        if (uVar6 == 0) {
LAB_00107023:
          if (bVar19 == false) {
            return 1;
          }
          uVar12 = FUN_00106920(param_2,param_9,param_6,uVar20);
          return uVar12;
        }
        *param_11 = *param_11 + uVar6;
        uVar15 = uVar12;
        uVar16 = uVar6;
        pcVar18 = __buf;
        bVar13 = bVar19;
LAB_00106c20:
        do {
          if (uVar16 < uVar15) {
            uVar15 = uVar16;
          }
          bVar19 = uVar15 != 0 && param_5 != 0;
          __n = uVar15;
          __s2 = __buf;
          if (uVar15 != 0 && param_5 != 0) {
            do {
              if (*__s2 != '\0') {
                bVar14 = bVar13;
                bVar2 = false;
                bVar1 = bVar19;
                goto LAB_00106c89;
              }
              __s2 = __s2 + 1;
              __n = __n - 1;
              if (__n == 0) {
                bVar17 = (bVar13 ^ 1U) & uVar20 != 0;
                goto LAB_00106d90;
              }
            } while ((__n & 0xf) != 0);
            iVar3 = memcmp(__buf,__s2,__n);
            bVar14 = (bool)(iVar3 == 0 ^ bVar13);
            bVar2 = iVar3 == 0;
            bVar1 = iVar3 != 0;
LAB_00106c89:
            bVar19 = bVar2;
            bVar17 = uVar20 != 0 & bVar14;
            if ((uVar15 == uVar16) && (bVar1)) {
              if (bVar17 == 0) {
                bVar19 = false;
                goto LAB_00106d32;
              }
              bVar1 = true;
              bVar19 = false;
            }
            else {
LAB_00106d90:
              if (bVar17 == 0) goto LAB_00106d99;
              bVar1 = false;
            }
joined_r0x00106d46:
            if (bVar13 == false) {
              uVar7 = FUN_001111c0(param_2,pcVar18,uVar20);
              if (uVar20 != uVar7) {
                uVar9 = FUN_00114440(4,param_9);
                uVar10 = dcgettext(0,"error writing %s",5);
                piVar8 = __errno_location();
                error(0,*piVar8,uVar10,uVar9);
                return 0;
              }
            }
            else {
              uVar4 = FUN_00106920(param_2,param_9,param_6,uVar20);
              if ((char)uVar4 == '\0') {
                return (ulong)uVar4;
              }
            }
            pcVar18 = __buf;
            uVar20 = uVar15;
            if (bVar1) {
              if (uVar15 == 0) {
                if (bVar17 != 0) {
                  uVar20 = 0;
                  break;
                }
                uVar16 = 0;
              }
              else if (bVar17 != 0) {
                uVar15 = 0;
                bVar13 = bVar19;
                goto LAB_00106c20;
              }
              uVar16 = uVar16 - uVar15;
              uVar20 = 0;
            }
            else {
              uVar16 = uVar16 - uVar15;
            }
          }
          else {
            if (((uVar15 == uVar16) && (bVar13 != true)) || (bVar19 = bVar13, uVar15 == 0)) {
LAB_00106d32:
              uVar20 = uVar20 + uVar15;
              bVar17 = 0;
              bVar1 = true;
              goto joined_r0x00106d46;
            }
LAB_00106d99:
            uVar20 = uVar20 + uVar15;
            if (uVar20 + 0x8000000000000000 < uVar15) {
              uVar9 = FUN_00114440(4,param_8);
              uVar10 = dcgettext(0,"overflow reading %s",5);
              error(0,0,uVar10,uVar9);
              return 0;
            }
            uVar16 = uVar16 - uVar15;
          }
          __buf = __buf + uVar15;
          bVar13 = bVar19;
        } while (uVar16 != 0);
        local_98 = local_98 - uVar6;
        *param_12 = bVar19;
        if (local_98 == 0) goto LAB_00107023;
        __buf = (char *)*param_3;
      }
      piVar8 = __errno_location();
      if (*piVar8 != 4) {
        uVar9 = FUN_00114440(4,param_8);
        uVar10 = dcgettext(0,"error reading %s",5);
        error(0,*piVar8,uVar10,uVar9);
        return 0;
      }
    } while( true );
  }
  return 1;
}




// Function: set_process_security_ctx @ 0x72a0

byte set_process_security_ctx(undefined8 param_1,undefined8 param_2,undefined8 param_3,byte param_4,long param_5
                 )

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte bVar5;
  long in_FS_OFFSET;
  undefined8 local_38;
  long local_30;
  
  bVar5 = *(byte *)(param_5 + 0x33);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (bVar5 == 0) {
    bVar5 = *(long *)(param_5 + 0x28) != 0 & param_4;
    if ((bVar5 != 0) && (iVar1 = FUN_0010c450(), iVar1 < 0)) {
      piVar2 = __errno_location();
      if ((*piVar2 != 0x5f) && (*piVar2 != 0x3d)) {
        uVar3 = FUN_00114440(4,param_2);
        uVar4 = dcgettext(0,"failed to set default file creation context for %s",5);
        error(0,*piVar2,uVar4,uVar3);
        goto LAB_0010735a;
      }
    }
    bVar5 = 1;
    goto LAB_0010735a;
  }
  if ((*(char *)(param_5 + 0x31) == '\0') || (*(char *)(param_5 + 0x34) != '\0')) {
    iVar1 = FUN_00115590(param_1,&local_38);
    if (-1 < iVar1) {
      iVar1 = setfscreatecon(local_38);
      if (iVar1 < 0) {
        piVar2 = __errno_location();
LAB_001074ba:
        uVar3 = FUN_00114b30(local_38);
        uVar4 = dcgettext(0,"failed to set default file creation context to %s",5);
        error(0,*piVar2,uVar4,uVar3);
        goto LAB_001073e7;
      }
      goto LAB_00107483;
    }
    piVar2 = __errno_location();
LAB_001072f8:
    uVar3 = FUN_00114440(4,param_1);
    uVar4 = dcgettext(0,"failed to get security context of %s",5);
    error(0,*piVar2,uVar4,uVar3);
LAB_0010732e:
    if (*(char *)(param_5 + 0x34) == '\0') goto LAB_0010735a;
  }
  else {
    if (*(char *)(param_5 + 0x37) != '\0') {
      iVar1 = FUN_00115590(param_1,&local_38);
      if (-1 < iVar1) {
        iVar1 = setfscreatecon(local_38);
        if (iVar1 < 0) goto LAB_001073e7;
        goto LAB_00107483;
      }
      goto LAB_0010732e;
    }
    iVar1 = FUN_00115590(param_1,&local_38);
    if (iVar1 < 0) {
      piVar2 = __errno_location();
      if ((*piVar2 != 0x5f) && (*piVar2 != 0x3d)) goto LAB_001072f8;
      goto LAB_0010732e;
    }
    iVar1 = setfscreatecon(local_38);
    if (-1 < iVar1) {
LAB_00107483:
      freecon(local_38);
      goto LAB_0010735a;
    }
    piVar2 = __errno_location();
    if ((*piVar2 != 0x5f) && (*piVar2 != 0x3d)) goto LAB_001074ba;
LAB_001073e7:
    if (*(char *)(param_5 + 0x34) == '\0') goto LAB_00107483;
    freecon(local_38);
  }
  bVar5 = 0;
LAB_0010735a:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar5;
}




// Function: set_file_security_ctx @ 0x7510

ulong set_file_security_ctx(undefined8 param_1,undefined1 param_2,long param_3)

{
  char cVar1;
  uint uVar2;
  ulong uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  
  uVar4 = *(undefined8 *)(param_3 + 0x28);
  if ((*(char *)(param_3 + 0x31) == '\0') || (*(char *)(param_3 + 0x34) != '\0')) {
    cVar1 = FUN_0010c6f0(uVar4,param_1,param_2);
    if (cVar1 == '\0') {
      piVar6 = __errno_location();
      goto LAB_00107596;
    }
  }
  else {
    if (*(char *)(param_3 + 0x37) != '\0') {
      uVar3 = FUN_0010c6f0(uVar4,param_1,param_2);
      return uVar3;
    }
    uVar2 = FUN_0010c6f0(uVar4,param_1,param_2);
    if ((char)uVar2 == '\0') {
      piVar6 = __errno_location();
      if (*piVar6 == 0x5f) {
        return (ulong)uVar2;
      }
      if (*piVar6 == 0x3d) {
        return (ulong)uVar2;
      }
LAB_00107596:
      uVar4 = FUN_00114310(0,4,param_1);
      uVar5 = dcgettext(0,"failed to set the security context of %s",5);
      error(0,*piVar6,uVar5,uVar4);
      return 0;
    }
  }
  return 1;
}




// Function: dest_info_init @ 0x75f0

void dest_info_init(long param_1)

{
  long lVar1;
  
  lVar1 = FUN_00111bb0(0x3d,0,FUN_00112460,FUN_00112430,FUN_001124d0);
  *(long *)(param_1 + 0x48) = lVar1;
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117150();
}




// Function: src_info_init @ 0x7630

void src_info_init(long param_1)

{
  long lVar1;
  
  lVar1 = FUN_00111bb0(0x3d,0,FUN_00112410,FUN_00112430,FUN_001124d0);
  *(long *)(param_1 + 0x50) = lVar1;
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117150();
}




// Function: cp_options_default @ 0x7670

void cp_options_default(undefined8 *param_1)

{
  __uid_t _Var1;
  ulong uVar2;
  undefined8 *puVar3;
  
  *param_1 = 0;
  param_1[10] = 0;
  puVar3 = (undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8);
  for (uVar2 = (ulong)(((int)param_1 -
                       (int)(undefined8 *)((ulong)(param_1 + 1) & 0xfffffffffffffff8)) + 0x58U >> 3)
      ; uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  _Var1 = geteuid();
  *(undefined4 *)(param_1 + 8) = 0xffffffff;
  *(bool *)((long)param_1 + 0x1b) = _Var1 == 0;
  *(bool *)((long)param_1 + 0x1a) = _Var1 == 0;
  return;
}




// Function: copy_internal @ 0x76f0

/* WARNING: Removing unreachable block (ram,0x00108718) */

ulong copy_internal(stat *param_1,stat *param_2,int param_3,stat *param_4,uint param_5,stat **param_6
                  ,undefined8 *param_7,uint *param_8,undefined4 param_9,stat **param_10,
                  stat *param_11,undefined1 *param_12)

{
  __mode_t *p_Var1;
  undefined *puVar2;
  __dev_t _Var3;
  stat **ppsVar4;
  char *__s2;
  __ino_t _Var5;
  __uid_t __owner;
  __gid_t __group;
  __mode_t _Var6;
  char cVar7;
  byte bVar8;
  char cVar9;
  int iVar10;
  int iVar11;
  uint uVar12;
  uint uVar13;
  undefined8 *puVar14;
  stat *psVar15;
  undefined8 uVar16;
  int *piVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  void *__ptr;
  uint *puVar20;
  char *pcVar21;
  size_t sVar22;
  void *pvVar23;
  ulong uVar24;
  ssize_t sVar25;
  stat **ppsVar26;
  __off_t _Var27;
  undefined8 *puVar28;
  stat *psVar29;
  long lVar30;
  undefined4 uVar31;
  ulong uVar32;
  undefined1 *puVar33;
  undefined1 *puVar34;
  undefined1 *puVar35;
  uint uVar36;
  char *pcVar37;
  undefined1 *puVar38;
  __blksize_t _Var39;
  undefined1 uVar40;
  stat *psVar41;
  ulong uVar42;
  undefined8 *__offset;
  ulong uVar43;
  size_t __n;
  long in_FS_OFFSET;
  bool bVar44;
  byte bVar45;
  undefined1 auStack_3e8 [8];
  undefined8 *local_3e0;
  uint local_3d8;
  uint local_3d4;
  stat **local_3d0;
  uint local_3c8;
  uint local_3c4;
  uint *local_3c0;
  stat **local_3b8;
  undefined8 local_3b0;
  undefined8 *local_3a8;
  uint *local_3a0;
  undefined8 *local_398;
  stat **local_390;
  undefined8 local_388;
  stat *local_380;
  stat **local_378;
  stat *local_370;
  ulong local_368;
  undefined8 *local_360;
  stat **local_358;
  stat *local_350;
  undefined8 local_348;
  stat *local_340;
  stat *local_338;
  int local_330;
  uint local_32c;
  stat *local_328;
  stat *local_320;
  undefined1 local_312 [2];
  stat *local_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  stat *local_2a8;
  __ino_t local_2a0;
  __nlink_t local_298;
  long local_290;
  stat local_288;
  stat local_1f8;
  stat local_168;
  stat local_d8;
  char local_41;
  long local_40;
  
  puVar35 = auStack_3e8;
  puVar33 = auStack_3e8;
  puVar38 = auStack_3e8;
  puVar34 = auStack_3e8;
  local_338 = param_2;
  local_340 = param_4;
  local_360 = param_7;
  local_330 = param_3;
  local_378 = param_6;
  local_328 = (stat *)CONCAT44(local_328._4_4_,param_9);
  local_358 = param_10;
  uVar12 = param_8[0x10];
  uVar43 = (ulong)uVar12;
  local_370 = param_11;
  local_348 = param_12;
  local_368 = CONCAT71(local_368._1_7_,(char)param_9);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  *(undefined1 *)&param_11->st_dev = 0;
  local_320 = (stat *)CONCAT71(local_320._1_7_,(char)param_8[6]);
  if ((char)param_8[6] != '\0') {
    if ((int)uVar12 < 0) {
      iVar10 = FUN_00114b50(0xffffff9c,param_1,param_3,param_4,1);
      if (iVar10 == 0) {
        param_5 = 1;
        *local_348 = 1;
        goto LAB_001079ae;
      }
      puVar20 = (uint *)__errno_location();
      uVar43 = (ulong)*puVar20;
    }
    bVar44 = (int)uVar43 == 0;
    local_320 = (stat *)CONCAT71(local_320._1_7_,bVar44);
    param_5 = (uint)bVar44;
    *local_348 = bVar44;
    if ((int)uVar43 != 0) goto LAB_0010779d;
LAB_001079ae:
    if (*(char *)((long)param_8 + 0x3f) != '\0') {
      uVar43 = 0;
      goto LAB_00107803;
    }
    uVar43 = 0;
    psVar29 = local_340;
    psVar15 = local_338;
    iVar10 = local_330;
    local_32c = param_5;
LAB_001077b2:
    iVar10 = fstatat(iVar10,(char *)psVar29,&local_288,(uint)(param_8[1] == 2) << 8);
    param_5 = local_32c;
    if (iVar10 != 0) goto LAB_00108528;
    local_32c = local_288.st_mode;
    if (((local_288.st_mode & 0xf000) != 0x4000) || ((char)param_8[0xe] != '\0')) goto LAB_00107803;
    uVar16 = FUN_00114440(4,param_1);
    uVar42 = 0;
    if (*(char *)((long)param_8 + 0x19) != '\0') {
      pcVar37 = "omitting directory %s";
      goto LAB_00109ad4;
    }
    uVar18 = dcgettext(0,"-r not specified; omitting directory %s",5);
LAB_001084c7:
    error(0,0,uVar18,uVar16);
    puVar35 = auStack_3e8;
    goto LAB_00107cf3;
  }
  local_320 = (stat *)CONCAT71(local_320._1_7_,0 < (int)param_5);
  if (uVar12 == 0) goto LAB_001079ae;
LAB_0010779d:
  if (((int)uVar43 != 0x11) || (param_8[2] != 2)) {
    psVar29 = param_1;
    psVar15 = param_1;
    iVar10 = -100;
    local_32c = param_5;
    goto LAB_001077b2;
  }
LAB_00107803:
  if (((char)local_328 != '\0') && (lVar30 = *(long *)(param_8 + 0x14), lVar30 != 0)) {
    psVar15 = &local_288;
    if (((local_32c & 0xf000) != 0x4000) && (*param_8 == 0)) {
      local_380 = (stat *)CONCAT44(local_380._4_4_,param_5);
      local_350 = &local_288;
      uVar12 = FUN_0010e820(lVar30,param_1,&local_288);
      uVar42 = (ulong)uVar12;
      if ((char)uVar12 != '\0') {
        uVar16 = FUN_00114440(4,param_1);
        pcVar37 = "warning: source file %s specified more than once";
LAB_00109ad4:
        uVar18 = dcgettext(0,pcVar37,5);
        goto LAB_001084c7;
      }
      lVar30 = *(long *)(param_8 + 0x14);
      psVar15 = local_350;
      param_5 = (uint)local_380;
    }
    local_350 = (stat *)CONCAT44(local_350._4_4_,param_5);
    FUN_0010e790(lVar30,param_1,psVar15);
    param_5 = (uint)local_350;
  }
  if (param_8[1] == 4) {
    local_380 = (stat *)CONCAT44(local_380._4_4_,1);
LAB_0010787c:
    local_350 = (stat *)0x0;
    psVar15 = (stat *)0x0;
    puVar35 = auStack_3e8;
    if (0 < (int)param_5) goto LAB_00107892;
  }
  else {
    if ((param_8[1] != 3) || ((char)local_328 == '\0')) {
      local_380 = (stat *)((ulong)local_380 & 0xffffffff00000000);
      goto LAB_0010787c;
    }
    local_380 = (stat *)CONCAT44(local_380._4_4_,1);
    if (0 < (int)param_5) {
      local_350 = (stat *)0x0;
      psVar15 = (stat *)0x0;
      puVar35 = auStack_3e8;
      goto LAB_00107ad0;
    }
  }
  if (((int)uVar43 == 0x11) && (param_8[2] == 2)) {
    psVar15 = (stat *)0x0;
    uVar43 = 0;
LAB_00108af3:
    cVar7 = (char)uVar43;
    if (*(char *)((long)param_8 + 0x3b) == '\0') goto LAB_00108c00;
    if ((local_32c & 0xf000) == 0x4000) {
      puVar35 = auStack_3e8;
      if ((char)param_8[6] == '\0') goto LAB_00108c33;
      goto LAB_0010910b;
    }
    bVar44 = false;
    if ((*(char *)((long)param_8 + 0x1f) != '\0') && (bVar44 = true, (char)param_8[6] != '\0')) {
      bVar44 = local_1f8.st_dev != local_288.st_dev;
    }
    iVar10 = FUN_00115b50(local_330,local_340,&local_1f8,&local_288,bVar44);
    if (iVar10 < 0) {
      puVar35 = auStack_3e8;
      puVar38 = auStack_3e8;
      if ((char)param_8[6] == '\0') goto LAB_00108c1d;
      goto LAB_0010910b;
    }
    if (local_348 != (undefined1 *)0x0) {
      *local_348 = 1;
    }
    lVar30 = FUN_0010bd30(local_340,local_288.st_ino,local_288.st_dev);
    puVar35 = auStack_3e8;
    if ((lVar30 == 0) ||
       (cVar7 = FUN_001065d0(0,local_330,lVar30,local_338,local_330,local_340,1,(char)param_8[0xf],
                             (ulong)local_380 & 0xffffffff), puVar35 = auStack_3e8, cVar7 != '\0'))
    {
LAB_00107d70:
      uVar42 = 1;
      goto LAB_00107cf3;
    }
    puVar35 = auStack_3e8;
    if ((*(char *)((long)param_8 + 0x33) == '\0') ||
       (iVar10 = setfscreatecon(0), puVar38 = auStack_3e8, puVar35 = auStack_3e8, iVar10 == 0))
    goto LAB_00107cf0;
LAB_00108bf7:
    cVar7 = (char)uVar43;
    *(undefined8 *)(puVar38 + -8) = 0x108bfc;
    FUN_00106770();
LAB_00108c00:
    puVar35 = puVar38;
    if ((char)param_8[6] == '\0') {
      puVar34 = puVar38;
      if ((local_32c & 0xf000) == 0x4000) goto LAB_00108c33;
LAB_00108c1d:
      puVar35 = puVar38;
      iVar10 = local_330;
      psVar41 = local_338;
      psVar29 = local_340;
      if (param_8[2] == 2) goto LAB_00107d70;
      puVar34 = puVar35;
      if (param_8[2] == 3) {
        *(undefined8 *)(puVar35 + -8) = 0x10a06b;
        cVar9 = FUN_001067a0(param_8,psVar41,iVar10,psVar29,&local_1f8);
        if (cVar9 == '\0') goto LAB_00107d70;
      }
LAB_00108c33:
      puVar35 = puVar34;
      if (cVar7 != '\0') goto LAB_00107d70;
      puVar33 = puVar34;
      if ((local_1f8.st_mode & 0xf000) != 0x4000) goto LAB_00108c56;
      if ((local_32c & 0xf000) == 0x4000) {
LAB_00108cc9:
        uVar42 = (ulong)*param_8;
        puVar33 = puVar34;
        goto LAB_00108ccc;
      }
LAB_00109818:
      psVar29 = local_338;
      if (((char)param_8[6] == '\0') || (uVar42 = (ulong)*param_8, puVar33 = puVar34, *param_8 == 0)
         ) {
        *(undefined8 *)(puVar34 + -8) = 0x10a283;
        uVar16 = FUN_00114440(4,psVar29);
        *(undefined8 *)(puVar34 + -8) = 0x10a299;
        uVar18 = dcgettext(0,"cannot overwrite directory %s with non-directory",5);
        uVar42 = 0;
        *(undefined8 *)(puVar34 + -8) = 0x10a2ad;
        error(0,0,uVar18,uVar16);
        puVar35 = puVar34;
        goto LAB_00107cf3;
      }
LAB_0010982f:
      puVar35 = puVar33;
      if ((local_288.st_mode & 0xf000) == 0x4000) {
LAB_0010a0b6:
        psVar29 = local_338;
        puVar35 = puVar33;
        if ((local_1f8.st_mode & 0xf000) != 0x4000) {
          if ((int)uVar42 != 0) goto LAB_00109847;
          *(undefined8 *)(puVar33 + -8) = 0x10a0e2;
          uVar16 = FUN_001147c0(0,3,psVar29);
          *(undefined8 *)(puVar33 + -8) = 0x10a0f4;
          uVar18 = FUN_001147c0(0,3,param_1);
          pcVar37 = "cannot move directory onto non-directory: %s -> %s";
          goto LAB_00107cd0;
        }
LAB_00109e4c:
        if ((int)uVar42 != 0) goto LAB_00109847;
LAB_00108d80:
        local_350 = (stat *)0x0;
        uVar43 = 0x11;
      }
      else {
LAB_00109847:
        iVar10 = (int)uVar42;
        *(undefined8 *)(puVar35 + -8) = 0x10984f;
        pcVar37 = (char *)FUN_0010d970();
        bVar45 = 1;
        if (*pcVar37 == '.') {
LAB_0010985e:
          iVar10 = (int)uVar42;
          if ((pcVar37[(ulong)(pcVar37[1] == '.') + 1] == '\0') ||
             (pcVar37[(ulong)(pcVar37[1] == '.') + 1] == '/')) goto LAB_00108ce2;
          if (bVar45 == 0) goto LAB_00109883;
        }
LAB_00109899:
        if (iVar10 != 3) {
          *(undefined8 *)(puVar35 + -8) = 0x1098ab;
          local_320 = (stat *)strlen(pcVar37);
          psVar29 = local_340;
          *(undefined8 *)(puVar35 + -8) = 0x1098be;
          pcVar21 = (char *)FUN_0010d970(psVar29);
          local_388 = pcVar21;
          *(undefined8 *)(puVar35 + -8) = 0x1098cd;
          local_350 = (stat *)strlen(pcVar21);
          pcVar21 = DAT_00123580;
          *(undefined8 *)(puVar35 + -8) = 0x1098e3;
          sVar22 = strlen(DAT_00123580);
          psVar29 = local_350;
          __s2 = local_388;
          if (local_320 == (stat *)((long)local_350->__unused + (sVar22 - 0x78))) {
            local_320 = local_350;
            *(undefined8 *)(puVar35 + -8) = 0x109910;
            iVar10 = memcmp(pcVar37,__s2,(size_t)psVar29);
            if (iVar10 == 0) {
              pcVar37 = pcVar37 + (long)local_320;
              *(undefined8 *)(puVar35 + -8) = 0x109928;
              iVar10 = strcmp(pcVar37,pcVar21);
              psVar29 = local_340;
              if (iVar10 == 0) {
                *(undefined8 *)(puVar35 + -8) = 0x10993b;
                sVar22 = strlen((char *)psVar29);
                *(undefined8 *)(puVar35 + -8) = 0x10994b;
                pcVar37 = (char *)FUN_00106110(psVar29,(long)psVar29->__unused + (sVar22 - 0x78),
                                               pcVar21);
                iVar10 = local_330;
                *(undefined8 *)(puVar35 + -8) = 0x109965;
                iVar10 = fstatat(iVar10,pcVar37,&local_d8,0);
                *(undefined8 *)(puVar35 + -8) = 0x109970;
                free(pcVar37);
                if (((iVar10 == 0) && (local_288.st_ino == local_d8.st_ino)) &&
                   (local_288.st_dev == local_d8.st_dev)) {
                  if ((char)param_8[6] == '\0') {
                    *(undefined8 *)(puVar35 + -8) = 0x10b770;
                    uVar16 = dcgettext(0,"backing up %s might destroy source;  %s not copied",5);
                  }
                  else {
                    *(undefined8 *)(puVar35 + -8) = 0x10b716;
                    uVar16 = dcgettext(0,"backing up %s might destroy source;  %s not moved",5);
                  }
                  *(undefined8 *)(puVar35 + -8) = 0x10b72b;
                  uVar18 = FUN_00114310(1,4,param_1);
                  psVar15 = local_338;
                  *(undefined8 *)(puVar35 + -8) = 0x10b741;
                  uVar19 = FUN_00114310(0,4,psVar15);
                  uVar42 = 0;
                  *(undefined8 *)(puVar35 + -8) = 0x10b758;
                  error(0,0,uVar16,uVar19,uVar18);
                  goto LAB_00107cf3;
                }
              }
            }
          }
        }
        iVar10 = local_330;
        psVar29 = local_340;
        uVar12 = *param_8;
        *(undefined8 *)(puVar35 + -8) = 0x1099a1;
        pcVar37 = (char *)FUN_0010d840(iVar10,psVar29,uVar12);
        if (pcVar37 == (char *)0x0) {
          *(undefined8 *)(puVar35 + -8) = 0x10b836;
          piVar17 = __errno_location();
          psVar29 = local_338;
          if (*piVar17 == 2) {
            local_320 = (stat *)CONCAT71(local_320._1_7_,1);
            uVar43 = 0x11;
            local_350 = (stat *)0x0;
            goto LAB_00107892;
          }
          *(undefined8 *)(puVar35 + -8) = 0x10b8ba;
          uVar16 = FUN_00114440(4,psVar29);
          *(undefined8 *)(puVar35 + -8) = 0x10b8d0;
          uVar18 = dcgettext(0,"cannot backup %s",5);
          iVar10 = *piVar17;
          uVar42 = 0;
          *(undefined8 *)(puVar35 + -8) = 0x10b8e6;
          error(0,iVar10,uVar18,uVar16);
          goto LAB_00107cf3;
        }
        __n = (long)psVar29 - (long)local_338;
        *(undefined8 *)(puVar35 + -8) = 0x1099bc;
        sVar22 = strlen(pcVar37);
        local_320 = (stat *)(sVar22 + 1);
        uVar43 = __n + 0x18 + sVar22;
        puVar38 = puVar35 + -(uVar43 & 0xfffffffffffff000);
        for (; puVar35 != puVar38; puVar35 = puVar35 + -0x1000) {
          *(undefined8 *)(puVar35 + -8) = *(undefined8 *)(puVar35 + -8);
        }
        uVar43 = (ulong)((uint)uVar43 & 0xff0);
        lVar30 = -uVar43;
        if (uVar43 != 0) {
          *(undefined8 *)(puVar35 + -8) = *(undefined8 *)(puVar35 + -8);
        }
        local_350 = (stat *)((ulong)(puVar35 + lVar30 + 0xf) & 0xfffffffffffffff0);
        uVar43 = 0x11;
        *(undefined8 *)(puVar35 + lVar30 + -8) = 0x109a35;
        pvVar23 = mempcpy(local_350,local_338,__n);
        *(undefined8 *)(puVar35 + lVar30 + -8) = 0x109a47;
        memcpy(pvVar23,pcVar37,(size_t)local_320);
        *(undefined8 *)(puVar35 + lVar30 + -8) = 0x109a4f;
        free(pcVar37);
        local_320 = (stat *)CONCAT71(local_320._1_7_,1);
        puVar35 = puVar35 + lVar30;
      }
    }
    else {
LAB_0010910b:
      uVar12 = param_8[2];
      if (uVar12 == 2) {
LAB_001091b0:
        if (local_348 != (undefined1 *)0x0) {
          *local_348 = 1;
        }
        goto LAB_00107d70;
      }
      puVar34 = puVar35;
      if (uVar12 == 3) {
LAB_0010917c:
        iVar10 = local_330;
        psVar41 = local_338;
        psVar29 = local_340;
        *(undefined8 *)(puVar35 + -8) = 0x10919f;
        cVar9 = FUN_001067a0(param_8,psVar41,iVar10,psVar29,&local_1f8);
        if (cVar9 != '\0') goto LAB_00108c33;
        goto LAB_001091b0;
      }
      if ((uVar12 != 4) || (*(char *)((long)param_8 + 0x3d) == '\0')) goto LAB_00108c33;
      if ((local_1f8.st_mode & 0xf000) != 0xa000) {
        *(undefined8 *)(puVar35 + -8) = 0x109150;
        cVar9 = FUN_00116bb0();
        iVar10 = local_330;
        psVar29 = local_340;
        if (cVar9 == '\0') {
          *(undefined8 *)(puVar35 + -8) = 0x109174;
          iVar10 = faccessat(iVar10,(char *)psVar29,2,0x200);
          if (iVar10 != 0) goto LAB_0010917c;
        }
        goto LAB_00108c33;
      }
      puVar33 = puVar35;
      if (cVar7 != '\0') goto LAB_00107d70;
LAB_00108c56:
      psVar29 = local_340;
      uVar12 = local_32c & 0xf000;
      if (uVar12 == 0x4000) {
        if (((char)param_8[6] != '\0') && (uVar42 = (ulong)*param_8, *param_8 != 0)) {
          if ((char)local_328 != '\0') goto LAB_00108c7d;
          goto LAB_0010982f;
        }
        *(undefined8 *)(puVar33 + -8) = 0x10a227;
        uVar16 = FUN_00114310(1,4,param_1);
        psVar15 = local_338;
        *(undefined8 *)(puVar33 + -8) = 0x10a23d;
        uVar18 = FUN_00114310(0,4,psVar15);
        pcVar37 = "cannot overwrite non-directory %s with directory %s";
        goto LAB_00107cd0;
      }
      uVar42 = (ulong)*param_8;
      if ((char)local_328 != '\0') {
LAB_00108c7d:
        if ((int)uVar42 == 3) {
          if ((char)param_8[6] != '\0') goto LAB_0010982f;
LAB_00109e1b:
          iVar10 = (int)uVar42;
          *(undefined8 *)(puVar33 + -8) = 0x109e23;
          pcVar37 = (char *)FUN_0010d970();
          puVar35 = puVar33;
          if (*pcVar37 == '.') {
            bVar45 = 0;
            goto LAB_0010985e;
          }
LAB_00109883:
          if ((local_1f8.st_mode & 0xf000) == 0x4000) goto LAB_00108d80;
          goto LAB_00109899;
        }
        uVar16 = *(undefined8 *)(param_8 + 0x12);
        *(undefined8 *)(puVar33 + -8) = 0x108c9e;
        cVar7 = FUN_0010e820(uVar16,psVar29,&local_1f8);
        if (cVar7 == '\0') {
          puVar34 = puVar33;
          if ((uVar12 != 0x4000) && ((local_1f8.st_mode & 0xf000) == 0x4000)) goto LAB_00109818;
          goto LAB_00108cc9;
        }
        *(undefined8 *)(puVar33 + -8) = 0x10a87a;
        uVar16 = FUN_00114310(1,4,param_1);
        psVar15 = local_338;
        *(undefined8 *)(puVar33 + -8) = 0x10a890;
        uVar18 = FUN_00114310(0,4,psVar15);
        pcVar37 = "will not overwrite just-created %s with %s";
        goto LAB_00107cd0;
      }
LAB_00108ccc:
      puVar35 = puVar33;
      if ((char)param_8[6] != '\0') {
        if ((local_288.st_mode & 0xf000) == 0x4000) goto LAB_0010a0b6;
        goto LAB_00109e4c;
      }
      bVar45 = 0;
      if ((int)uVar42 != 0) goto LAB_00109e1b;
LAB_00108ce2:
      iVar10 = local_330;
      psVar29 = local_340;
      uVar43 = 0x11;
      local_350 = (stat *)0x0;
      bVar45 = (local_1f8.st_mode & 0xf000) == 0x4000 | bVar45;
      uVar42 = CONCAT71((int7)(uVar42 >> 8),bVar45);
      if (bVar45 == 0) {
        if (*(char *)((long)param_8 + 0x15) == '\0') {
          if (*(char *)((long)param_8 + 0x31) == '\0') goto LAB_00107892;
          if (((char)param_8[0xc] == '\0') || (local_1f8.st_nlink < 2)) {
            uVar43 = 0x11;
            local_350 = (stat *)0x0;
            if ((param_8[1] != 2) || ((local_288.st_mode & 0xf000) == 0x8000)) goto LAB_00107892;
          }
        }
        local_350 = (stat *)0x0;
        *(undefined8 *)(puVar35 + -8) = 0x108d51;
        iVar10 = unlinkat(iVar10,(char *)psVar29,0);
        if (iVar10 != 0) {
          *(undefined8 *)(puVar35 + -8) = 0x108d5a;
          piVar17 = __errno_location();
          psVar29 = local_338;
          if (*piVar17 != 2) {
            *(undefined8 *)(puVar35 + -8) = 0x10acf5;
            uVar16 = FUN_00114440(4,psVar29);
            *(undefined8 *)(puVar35 + -8) = 0x10ad0b;
            uVar18 = dcgettext(0,"cannot remove %s",5);
            iVar10 = *piVar17;
            *(undefined8 *)(puVar35 + -8) = 0x10ad1e;
            error(0,iVar10,uVar18,uVar16);
            goto LAB_00107cf3;
          }
        }
        psVar29 = local_338;
        local_320 = (stat *)CONCAT71(local_320._1_7_,(char)param_8[0xf]);
        if ((char)param_8[0xf] == '\0') {
          local_320 = (stat *)CONCAT71(local_320._1_7_,1);
          goto LAB_00108d80;
        }
        uVar43 = 0x11;
        *(undefined8 *)(puVar35 + -8) = 0x10a9df;
        uVar16 = FUN_00114440(4,psVar29);
        *(undefined8 *)(puVar35 + -8) = 0x10a9f5;
        uVar18 = dcgettext(0,"removed %s\n",5);
        *(undefined8 *)(puVar35 + -8) = 0x10aa07;
        __printf_chk(1,uVar18,uVar16);
        local_350 = (stat *)0x0;
      }
    }
LAB_00107892:
    psVar29 = psVar15;
    if ((char)local_328 != '\0') {
LAB_00107ad0:
      iVar10 = local_330;
      psVar41 = local_340;
      psVar29 = psVar15;
      if ((*(long *)(param_8 + 0x12) == 0) || ((char)param_8[6] != '\0')) goto LAB_0010789f;
      if (*param_8 != 0) {
        if ((char)param_8[0xf] != '\0') goto LAB_00108258;
        if ((int)uVar43 == 0) goto LAB_00107d98;
LAB_001078ba:
        _Var5 = local_288.st_ino;
        _Var3 = local_288.st_dev;
        psVar29 = local_340;
        iVar10 = (int)uVar43;
        if (((char)param_8[0xe] == '\0') || ((local_32c & 0xf000) != 0x4000)) {
          cVar7 = (char)param_8[6];
          if (cVar7 == '\0') {
            if ((char)param_8[0xc] == '\0') goto LAB_00107d98;
            if (*(char *)((long)param_8 + 0x17) == '\0') goto LAB_0010820e;
            puVar14 = (undefined8 *)0x0;
LAB_00108246:
            uVar43 = (ulong)local_320 & 0xff;
            goto LAB_00107da3;
          }
          if (local_288.st_nlink == 1) {
            *(undefined8 *)(puVar35 + -8) = 0x109343;
            puVar14 = (undefined8 *)FUN_0010bcd0(_Var5,_Var3);
LAB_00108f3d:
            iVar11 = local_330;
            if (puVar14 == (undefined8 *)0x0) goto LAB_00107970;
            if ((local_32c & 0xf000) == 0x4000) goto LAB_0010790d;
            *(ulong *)(puVar35 + -0x10) = (ulong)local_380 & 0xffffffff;
            psVar29 = local_338;
            *(ulong *)(puVar35 + -0x18) = (ulong)(byte)param_8[0xf];
            *(undefined8 *)(puVar35 + -0x20) = 1;
            *(undefined8 *)(puVar35 + -0x28) = 0x108f90;
            cVar7 = FUN_001065d0(0,iVar11,puVar14,psVar29,iVar11);
            if (cVar7 == '\0') goto LAB_00108f9c;
            goto LAB_00107d70;
          }
          if (((char)param_8[0xc] != '\0') && (*(char *)((long)param_8 + 0x17) == '\0')) {
LAB_0010820e:
            if ((1 < local_288.st_nlink) ||
               ((((char)local_328 != '\0' && (param_8[1] == 3)) ||
                (puVar14 = (undefined8 *)0x0, param_8[1] == 4)))) {
              *(undefined8 *)(puVar35 + -8) = 0x108f3a;
              puVar14 = (undefined8 *)FUN_0010bd30(psVar29,_Var5,_Var3);
              goto LAB_00108f3d;
            }
            goto LAB_0010823e;
          }
          puVar14 = (undefined8 *)0x0;
LAB_00107a20:
          iVar11 = local_330;
          psVar15 = local_340;
          if (iVar10 != 0x11) {
LAB_00107a2a:
            iVar11 = local_330;
            psVar29 = local_338;
            psVar15 = local_340;
            if (iVar10 == 0x16) {
              *(undefined8 *)(puVar35 + -8) = 0x1093ae;
              uVar16 = FUN_00114310(1,4,DAT_00123568);
              *(undefined8 *)(puVar35 + -8) = 0x1093c4;
              uVar18 = FUN_00114310(0,4,DAT_00123570);
              *(undefined8 *)(puVar35 + -8) = 0x1093da;
              uVar19 = dcgettext(0,"cannot move %s to a subdirectory of itself, %s",5);
              uVar42 = 1;
              *(undefined8 *)(puVar35 + -8) = 0x1093f4;
              error(0,0,uVar19,uVar18,uVar16);
              *(undefined1 *)&local_370->st_dev = 1;
              goto LAB_00107cf3;
            }
            if (iVar10 == 0x12) {
              uVar12 = local_32c & 0xf000;
              *(undefined8 *)(puVar35 + -8) = 0x107a6a;
              iVar10 = unlinkat(iVar11,(char *)psVar15,(uint)(uVar12 == 0x4000) << 9);
              if (iVar10 != 0) {
                *(undefined8 *)(puVar35 + -8) = 0x107a73;
                piVar17 = __errno_location();
                psVar15 = local_338;
                if (*piVar17 != 2) {
                  *(undefined8 *)(puVar35 + -8) = 0x109246;
                  uVar16 = FUN_00114310(1,4,psVar15);
                  *(undefined8 *)(puVar35 + -8) = 0x109258;
                  uVar18 = FUN_00114310(0,4,param_1);
                  *(undefined8 *)(puVar35 + -8) = 0x10926e;
                  uVar19 = dcgettext(0,"inter-device move failed: %s to %s; unable to remove target"
                                     ,5);
                  iVar10 = *piVar17;
                  goto LAB_00108e3a;
                }
              }
              bVar45 = uVar12 != 0x4000 & (byte)param_8[0xf];
              local_320 = (stat *)CONCAT71(local_320._1_7_,bVar45);
              if (bVar45 == 0) {
                local_320 = (stat *)CONCAT71(local_320._1_7_,1);
                uVar43 = 1;
              }
              else {
                uVar43 = 1;
                *(undefined8 *)(puVar35 + -8) = 0x109369;
                uVar16 = dcgettext(0,"copied ",5);
                *(undefined8 *)(puVar35 + -8) = 0x109378;
                __printf_chk(1,uVar16);
                psVar29 = local_338;
                psVar15 = local_350;
                *(undefined8 *)(puVar35 + -8) = 0x10938e;
                FUN_00106510(param_1,psVar29,psVar15);
              }
              goto LAB_00107da3;
            }
            *(undefined8 *)(puVar35 + -8) = 0x108e06;
            uVar16 = FUN_00114310(1,4,psVar29);
            *(undefined8 *)(puVar35 + -8) = 0x108e18;
            uVar18 = FUN_00114310(0,4,param_1);
            *(undefined8 *)(puVar35 + -8) = 0x108e2e;
            uVar19 = dcgettext(0,"cannot move %s to %s",5);
LAB_00108e3a:
            *(undefined8 *)(puVar35 + -8) = 0x108e43;
            error(0,iVar10,uVar19,uVar18,uVar16);
            *(undefined8 *)(puVar35 + -8) = 0x108e56;
            FUN_0010bc60(local_288.st_ino,local_288.st_dev);
            goto LAB_00107cf0;
          }
          *(undefined8 *)(puVar35 + -8) = 0x108dba;
          iVar10 = renameat(-100,(char *)param_1,iVar11,(char *)psVar15);
          if (iVar10 != 0) {
            *(undefined8 *)(puVar35 + -8) = 0x108dc7;
            piVar17 = __errno_location();
            iVar10 = *piVar17;
            if (iVar10 != 0) goto LAB_00107a2a;
          }
LAB_00107d27:
          if ((char)param_8[0xf] != '\0') {
            *(undefined8 *)(puVar35 + -8) = 0x1092a3;
            uVar16 = dcgettext(0,"renamed ",5);
            *(undefined8 *)(puVar35 + -8) = 0x1092b2;
            __printf_chk(1,uVar16);
            psVar29 = local_338;
            psVar15 = local_350;
            *(undefined8 *)(puVar35 + -8) = 0x1092c8;
            FUN_00106510(param_1,psVar29,psVar15);
          }
          psVar15 = local_338;
          if (*(long *)(param_8 + 10) != 0) {
            *(undefined8 *)(puVar35 + -8) = 0x107d4d;
            FUN_00107510(psVar15,1,param_8);
          }
          psVar15 = local_340;
          if (local_348 != (undefined1 *)0x0) {
            *local_348 = 1;
          }
          if (((char)local_328 == '\0') || (*(char *)((long)param_8 + 0x3f) != '\0'))
          goto LAB_00107d70;
          uVar16 = *(undefined8 *)(param_8 + 0x12);
          *(undefined8 *)(puVar35 + -8) = 0x10921f;
          FUN_0010e790(uVar16,psVar15,&local_288);
          uVar42 = (ulong)local_328 & 0xff;
          goto LAB_00107cf3;
        }
        if ((char)local_328 == '\0') {
          *(undefined8 *)(puVar35 + -8) = 0x108f18;
          puVar14 = (undefined8 *)FUN_0010bcd0(_Var5,_Var3);
        }
        else {
          *(undefined8 *)(puVar35 + -8) = 0x107905;
          puVar14 = (undefined8 *)FUN_0010bd30(psVar29,_Var5);
        }
        if (puVar14 == (undefined8 *)0x0) {
LAB_00107970:
          cVar7 = (char)param_8[6];
LAB_0010823e:
          if (cVar7 == '\0') goto LAB_00108246;
          goto LAB_00107a20;
        }
LAB_0010790d:
        iVar11 = local_330;
        *(undefined8 *)(puVar35 + -8) = 0x107923;
        cVar7 = FUN_00114e20(0xffffff9c,param_1,iVar11,puVar14);
        iVar11 = local_330;
        psVar15 = local_340;
        if (cVar7 == '\0') {
          *(undefined8 *)(puVar35 + -8) = 0x107942;
          cVar7 = FUN_00114e20(iVar11,psVar15,iVar11,puVar14);
          psVar29 = local_338;
          psVar15 = local_340;
          if (cVar7 != '\0') {
            *(undefined8 *)(puVar35 + -8) = 0x10a3e9;
            uVar16 = FUN_00114440(4,DAT_00123570);
            *(undefined8 *)(puVar35 + -8) = 0x10a3ff;
            uVar18 = dcgettext(0,"warning: source directory %s specified more than once",5);
            *(undefined8 *)(puVar35 + -8) = 0x10a410;
            error(0,0,uVar18,uVar16);
            if ((char)param_8[6] != '\0') goto LAB_001091b0;
            goto LAB_00107d70;
          }
          if ((param_8[1] == 4) || ((param_8[1] == 3 && ((char)local_328 != '\0'))))
          goto LAB_00107970;
          *(undefined8 *)(puVar35 + -8) = 0x10a1b7;
          pvVar23 = (void *)FUN_00106110(psVar29,psVar15,puVar14);
          *(undefined8 *)(puVar35 + -8) = 0x10a1cc;
          psVar15 = (stat *)FUN_00114310(1,4,pvVar23);
          *(undefined8 *)(puVar35 + -8) = 0x10a1de;
          uVar43 = FUN_00114310(0,4,psVar29);
          *(undefined8 *)(puVar35 + -8) = 0x10a1f4;
          uVar16 = dcgettext(0,"will not create hard link %s to directory %s",5);
          *(undefined8 *)(puVar35 + -8) = 0x10a208;
          error(0,0,uVar16,uVar43,psVar15);
          *(undefined8 *)(puVar35 + -8) = 0x10a210;
          free(pvVar23);
          param_1 = psVar29;
        }
        else {
          *(undefined8 *)(puVar35 + -8) = 0x109dc1;
          psVar15 = (stat *)FUN_00114310(1,4,DAT_00123568);
          *(undefined8 *)(puVar35 + -8) = 0x109dd7;
          uVar16 = FUN_00114310(0,4,DAT_00123570);
          *(undefined8 *)(puVar35 + -8) = 0x109ded;
          uVar18 = dcgettext(0,"cannot copy a directory, %s, into itself, %s",5);
          *(undefined8 *)(puVar35 + -8) = 0x109e01;
          error(0,0,uVar18,uVar16,psVar15);
          *(undefined1 *)&local_370->st_dev = 1;
        }
LAB_00108f9c:
        if (*(char *)((long)param_8 + 0x33) == '\0') goto LAB_0010866c;
        *(undefined8 *)(puVar35 + -8) = 0x108fae;
        iVar10 = setfscreatecon(0);
        puVar38 = puVar35;
        if (iVar10 == 0) goto LAB_0010866c;
        goto LAB_00108bf7;
      }
      psVar29 = &local_1f8;
      if ((char)psVar15 == '\0') {
        psVar29 = &local_d8;
        *(undefined8 *)(puVar35 + -8) = 0x1091f1;
        iVar10 = fstatat(iVar10,(char *)psVar41,psVar29,0x100);
        psVar15 = psVar29;
        if (iVar10 != 0) goto LAB_0010789f;
      }
      psVar41 = local_340;
      p_Var1 = &psVar29->st_mode;
      psVar29 = psVar15;
      if ((*p_Var1 & 0xf000) != 0xa000) goto LAB_0010789f;
      uVar16 = *(undefined8 *)(param_8 + 0x12);
      *(undefined8 *)(puVar35 + -8) = 0x107b26;
      cVar7 = FUN_0010e820(uVar16,psVar41);
      psVar15 = local_338;
      if (cVar7 != '\0') {
        *(undefined8 *)(puVar35 + -8) = 0x107b44;
        uVar16 = FUN_00114310(1,4,psVar15);
        *(undefined8 *)(puVar35 + -8) = 0x107b56;
        uVar18 = FUN_00114310(0,4,param_1);
        *(undefined8 *)(puVar35 + -8) = 0x107b6c;
        uVar19 = dcgettext(0,"will not copy %s through just-created symlink %s",5);
        *(undefined8 *)(puVar35 + -8) = 0x107b80;
        error(0,0,uVar19,uVar18,uVar16);
        uVar42 = 0;
        goto LAB_00107cf3;
      }
    }
LAB_0010789f:
    psVar15 = psVar29;
    if (((char)param_8[0xf] != '\0') && ((char)param_8[6] == '\0')) {
LAB_00108258:
      psVar41 = local_338;
      psVar29 = local_350;
      if ((local_32c & 0xf000) != 0x4000) {
        *(undefined8 *)(puVar35 + -8) = 0x108284;
        FUN_00106510(param_1,psVar41,psVar29);
      }
    }
    if ((int)uVar43 != 0) goto LAB_001078ba;
    if ((char)param_8[6] != '\0') goto LAB_00107d27;
LAB_00107d98:
    uVar43 = (ulong)local_320 & 0xff;
    puVar14 = (undefined8 *)0x0;
LAB_00107da3:
    uVar13 = local_32c;
    psVar29 = local_338;
    uVar12 = local_32c;
    if (*(char *)((long)param_8 + 0x39) != '\0') {
      uVar12 = param_8[4];
    }
    psVar15 = (stat *)(ulong)(uVar12 & 0xfff);
    puVar38 = puVar35;
    if (*(char *)((long)param_8 + 0x1d) != '\0') {
      *(undefined8 *)(puVar35 + -8) = 0x107e4b;
      cVar7 = FUN_001072a0(param_1,psVar29,uVar13,uVar43,param_8);
      if (cVar7 == '\0') goto LAB_00107cf0;
      local_388 = (char *)(CONCAT44(local_388._4_4_,uVar12) & 0xffffffff0000003f);
      local_348 = (undefined1 *)(CONCAT44(local_348._4_4_,local_32c) & 0xffffffff0000f000);
      puVar28 = local_360;
      if ((local_32c & 0xf000) != 0x4000) goto LAB_00107e7a;
joined_r0x00108356:
      for (; _Var5 = local_288.st_ino, _Var3 = local_288.st_dev, puVar28 != (undefined8 *)0x0;
          puVar28 = (undefined8 *)*puVar28) {
        if ((puVar28[1] == local_288.st_ino) && ((stat *)puVar28[2] == (stat *)local_288.st_dev)) {
          *(undefined8 *)(puVar35 + -8) = 0x108385;
          uVar43 = FUN_00114440(4,param_1);
          pcVar37 = "cannot copy cyclic symbolic link %s";
          goto LAB_00108394;
        }
      }
      puVar34 = puVar35 + -0x20;
      puVar38 = puVar35 + -0x20;
      *(undefined8 *)(puVar35 + -8) = *(undefined8 *)(puVar35 + -8);
      iVar10 = local_330;
      psVar41 = local_338;
      psVar29 = local_340;
      local_3a8 = (undefined8 *)((ulong)(puVar35 + -0x11) & 0xfffffffffffffff0);
      *local_3a8 = local_360;
      local_3a8[1] = _Var5;
      local_3a8[2] = _Var3;
      if (((byte)local_320 != '\0') || ((local_1f8.st_mode & 0xf000) != 0x4000)) {
        uVar13 = ~(uint)local_388;
        *(undefined8 *)(puVar35 + -0x28) = 0x108794;
        iVar11 = mkdirat(iVar10,(char *)psVar29,uVar13 & uVar12 & 0xfff);
        iVar10 = local_330;
        psVar41 = local_338;
        psVar29 = local_340;
        if (iVar11 == 0) {
          *(undefined8 *)(puVar35 + -0x28) = 0x1087ba;
          iVar11 = fstatat(iVar10,(char *)psVar29,&local_1f8,0x100);
          _Var6 = local_1f8.st_mode;
          iVar10 = local_330;
          psVar41 = local_338;
          psVar29 = local_340;
          if (iVar11 == 0) {
            psVar15 = (stat *)(ulong)local_1f8.st_mode;
            local_3b8 = (stat **)((ulong)local_3b8 & 0xffffffffffffff00);
            if ((local_1f8.st_mode & 0x1c0) != 0x1c0) {
              uVar12 = local_1f8.st_mode | 0x1c0;
              *(undefined8 *)(puVar35 + -0x28) = 0x1087fc;
              iVar10 = fchmodat(iVar10,(char *)psVar29,uVar12,0x100);
              psVar29 = local_338;
              local_3c4 = _Var6;
              local_3b8 = (stat **)CONCAT71(local_3b8._1_7_,1);
              if (iVar10 != 0) {
                *(undefined8 *)(puVar35 + -0x28) = 0x10a4ec;
                param_1 = (stat *)FUN_00114440(4,psVar29);
                pcVar37 = "setting permissions for %s";
                puVar38 = puVar35 + -0x20;
                goto LAB_00108638;
              }
            }
            psVar15 = local_340;
            if (*(char *)local_358 == '\0') {
              *(undefined8 *)(puVar35 + -0x28) = 0x109f16;
              FUN_0010bd30(psVar15,local_1f8.st_ino,local_1f8.st_dev);
              *(undefined1 *)local_358 = 1;
            }
            psVar15 = local_338;
            if ((char)param_8[0xf] != '\0') {
              if ((char)param_8[6] == '\0') {
                *(undefined8 *)(puVar35 + -0x28) = 0x10a199;
                FUN_00106510(param_1,psVar15,0);
              }
              else {
                *(undefined8 *)(puVar35 + -0x28) = 0x108844;
                puVar14 = (undefined8 *)FUN_00114440(4,psVar15);
                *(undefined8 *)(puVar35 + -0x28) = 0x10885a;
                uVar16 = dcgettext(0,"created directory %s\n",5);
                *(undefined8 *)(puVar35 + -0x28) = 0x10886c;
                __printf_chk(1,uVar16,puVar14);
              }
            }
            goto LAB_00108870;
          }
          *(undefined8 *)(puVar35 + -0x28) = 0x10a034;
          param_1 = (stat *)FUN_00114440(4,psVar41);
          pcVar37 = "cannot stat %s";
          puVar38 = puVar35 + -0x20;
        }
        else {
          *(undefined8 *)(puVar35 + -0x28) = 0x1097e9;
          param_1 = (stat *)FUN_00114440(4,psVar41);
          pcVar37 = "cannot create directory %s";
        }
        goto LAB_00108638;
      }
      if ((*(long *)(param_8 + 10) != 0) || (*(char *)((long)param_8 + 0x33) != '\0')) {
        *(undefined8 *)(puVar35 + -0x28) = 0x109a8c;
        cVar7 = FUN_00107510(psVar41,0,param_8);
        if ((cVar7 != '\0') || (puVar38 = puVar35 + -0x20, (char)param_8[0xd] == '\0'))
        goto LAB_00109a9b;
        goto LAB_00108658;
      }
LAB_00109a9b:
      local_3b8 = (stat **)((ulong)local_3b8 & 0xffffffffffffff00);
      local_388 = (char *)((ulong)local_388 & 0xffffffff00000000);
LAB_00108870:
      bVar45 = local_378 != (stat **)0x0 & (byte)param_8[7];
      uVar42 = CONCAT71((int7)((ulong)puVar14 >> 8),bVar45);
      if ((bVar45 != 0) && (*local_378 != (stat *)local_288.st_dev)) goto LAB_00108aad;
      local_308 = *(__off_t *)param_8;
      uStack_300 = *(undefined8 *)(param_8 + 2);
      local_2f8 = *(undefined8 *)(param_8 + 4);
      uStack_2f0 = *(undefined8 *)(param_8 + 6);
      local_2e8 = *(undefined8 *)(param_8 + 8);
      uStack_2e0 = *(undefined8 *)(param_8 + 10);
      local_2d8 = *(undefined8 *)(param_8 + 0xc);
      uStack_2d0 = *(undefined8 *)(param_8 + 0xe);
      local_2c8 = *(undefined8 *)(param_8 + 0x10);
      uStack_2c0 = *(undefined8 *)(param_8 + 0x12);
      local_2b8 = *(undefined8 *)(param_8 + 0x14);
      *(undefined8 *)(puVar35 + -0x28) = 0x1088e0;
      pcVar37 = (char *)FUN_00115300(param_1,2);
      psVar15 = local_370;
      local_3b0 = pcVar37;
      if (pcVar37 == (char *)0x0) {
        *(undefined8 *)(puVar35 + -0x28) = 0x10b9a7;
        uVar16 = FUN_00114440(4,param_1);
        *(undefined8 *)(puVar35 + -0x28) = 0x10b9bd;
        uVar18 = dcgettext(0,"cannot access %s",5);
        *(undefined8 *)(puVar35 + -0x28) = 0x10b9c5;
        piVar17 = __errno_location();
        iVar10 = *piVar17;
        uVar42 = 0;
        *(undefined8 *)(puVar35 + -0x28) = 0x10b9d9;
        error(0,iVar10,uVar18,uVar16);
        goto LAB_00108aad;
      }
      if (param_8[1] == 3) {
        local_308 = CONCAT44(2,(undefined4)local_308);
      }
      if (*pcVar37 == '\0') {
        local_360 = (undefined8 *)((ulong)local_360._1_7_ << 8);
        uVar42 = 1;
        goto LAB_00108a91;
      }
      local_368 = (long)local_340 - (long)local_338;
      local_370 = &local_288;
      local_390 = &local_310;
      local_378 = (stat **)local_312;
      local_380 = (stat *)(local_312 + 1);
      local_398 = &local_308;
      local_360 = (undefined8 *)((ulong)local_360._1_7_ << 8);
      local_3a0 = (uint *)CONCAT44(local_3a0._4_4_,(int)uVar43);
      local_348 = (undefined1 *)CONCAT71(local_348._1_7_,1);
      local_3c0 = param_8;
      local_350 = param_1;
      goto LAB_001089cb;
    }
    local_348 = (undefined1 *)(CONCAT44(local_348._4_4_,local_32c) & 0xffffffff0000f000);
    if ((local_32c & 0xf000) == 0x4000) {
      *(undefined8 *)(puVar35 + -8) = 0x10832b;
      cVar7 = FUN_001072a0(param_1,psVar29,uVar13,uVar43,param_8);
      if (cVar7 != '\0') {
        local_388 = (char *)(CONCAT44(local_388._4_4_,uVar12) & 0xffffffff00000012);
        puVar28 = local_360;
        goto joined_r0x00108356;
      }
      goto LAB_00107cf0;
    }
    *(undefined8 *)(puVar35 + -8) = 0x107dfe;
    cVar7 = FUN_001072a0(param_1,psVar29,uVar13,uVar43,param_8);
    local_388 = (char *)((ulong)local_388 & 0xffffffff00000000);
    if (cVar7 == '\0') goto LAB_00107cf0;
LAB_00107e7a:
    _Var27 = local_288.st_size;
    _Var3 = local_288.st_rdev;
    iVar10 = local_330;
    psVar41 = local_338;
    psVar29 = local_340;
    uVar43 = (ulong)*(byte *)((long)param_8 + 0x3a);
    if (*(byte *)((long)param_8 + 0x3a) == 0) {
      if (*(char *)((long)param_8 + 0x17) == '\0') {
        if (((uint)local_348 != 0x8000) && (((uint)local_348 != 0xa000 & (byte)param_8[5]) == 0)) {
          local_358 = (stat **)((ulong)local_358._1_7_ << 8);
          if ((uint)local_348 == 0x1000) {
            uVar12 = ~(uint)local_388 & local_32c;
            psVar15 = (stat *)(ulong)uVar12;
            *(undefined8 *)(puVar35 + -8) = 0x10a2d6;
            iVar11 = mknodat(iVar10,(char *)psVar29,uVar12,0);
            iVar10 = local_330;
            psVar29 = local_340;
            cVar7 = (char)local_358;
            if (iVar11 != 0) {
              *(undefined8 *)(puVar35 + -8) = 0x10a2fd;
              iVar10 = mkfifoat(iVar10,(char *)psVar29,uVar12 & 0xffffefff);
              psVar29 = local_338;
              cVar7 = (char)local_358;
              if (iVar10 != 0) {
                *(undefined8 *)(puVar35 + -8) = 0x10a31e;
                param_1 = (stat *)FUN_00114440(4,psVar29);
                pcVar37 = "cannot create fifo %s";
                goto LAB_00108638;
              }
            }
            goto LAB_00107f17;
          }
          if (((uint)local_348 & 0xffffbfff) == 0x2000 || (uint)local_348 == 0xc000) {
            uVar12 = ~(uint)local_388 & local_32c;
            *(undefined8 *)(puVar35 + -8) = 0x108601;
            iVar10 = mknodat(iVar10,(char *)psVar29,uVar12,_Var3);
            psVar29 = local_338;
            cVar7 = (char)local_358;
            if (iVar10 == 0) goto LAB_00107f17;
            *(undefined8 *)(puVar35 + -8) = 0x108622;
            param_1 = (stat *)FUN_00114440(4,psVar29);
            pcVar37 = "cannot create special file %s";
LAB_00108638:
            *(undefined8 *)(puVar38 + -8) = 0x10863f;
            uVar43 = dcgettext(0,pcVar37,5);
            *(undefined8 *)(puVar38 + -8) = 0x108647;
            piVar17 = __errno_location();
            iVar10 = *piVar17;
            *(undefined8 *)(puVar38 + -8) = 0x108658;
            error(0,iVar10,uVar43);
            goto LAB_00108658;
          }
          if ((uint)local_348 == 0xa000) {
            *(undefined8 *)(puVar35 + -8) = 0x109f44;
            psVar15 = (stat *)FUN_0010c960(param_1,_Var27);
            iVar10 = local_330;
            psVar29 = local_340;
            if (psVar15 == (stat *)0x0) {
              *(undefined8 *)(puVar35 + -8) = 0x10ae5a;
              param_1 = (stat *)FUN_00114440(4,param_1);
              pcVar37 = "cannot read symbolic link %s";
              goto LAB_00108638;
            }
            uVar40 = *(undefined1 *)((long)param_8 + 0x16);
            *(undefined8 *)(puVar35 + -8) = 0x109f70;
            uVar12 = FUN_0010c000(psVar15,iVar10,psVar29,uVar40,0xffffffff);
            uVar43 = (ulong)uVar12;
            if (0 < (int)uVar12) {
              if (((*(char *)((long)param_8 + 0x3b) == '\x01') && ((byte)local_320 == '\0')) &&
                 ((local_1f8.st_mode & 0xf000) == 0xa000)) {
                local_358 = (stat **)local_1f8.st_size;
                *(undefined8 *)(puVar35 + -8) = 0x10afb5;
                ppsVar26 = (stat **)strlen((char *)psVar15);
                iVar10 = local_330;
                psVar29 = local_340;
                if (local_358 == ppsVar26) {
                  *(undefined8 *)(puVar35 + -8) = 0x10afd7;
                  ppsVar26 = (stat **)FUN_0010caf0(iVar10,psVar29);
                  if (ppsVar26 != (stat **)0x0) {
                    local_358 = ppsVar26;
                    *(undefined8 *)(puVar35 + -8) = 0x10aff2;
                    iVar10 = strcmp((char *)ppsVar26,(char *)psVar15);
                    ppsVar26 = local_358;
                    if (iVar10 == 0) {
                      *(undefined8 *)(puVar35 + -8) = 0x10b006;
                      free(ppsVar26);
                      *(undefined8 *)(puVar35 + -8) = 0x10b00e;
                      free(psVar15);
                      goto LAB_0010a90a;
                    }
                    *(undefined8 *)(puVar35 + -8) = 0x10ba71;
                    free(ppsVar26);
                  }
                }
              }
              *(undefined8 *)(puVar35 + -8) = 0x109f97;
              free(psVar15);
              psVar29 = local_338;
              *(undefined8 *)(puVar35 + -8) = 0x109fa8;
              param_1 = (stat *)FUN_00114440(4,psVar29);
              *(undefined8 *)(puVar35 + -8) = 0x109fbe;
              uVar16 = dcgettext(0,"cannot create symbolic link %s",5);
              *(undefined8 *)(puVar35 + -8) = 0x109fd0;
              error(0,uVar43,uVar16);
              goto LAB_00108658;
            }
            *(undefined8 *)(puVar35 + -8) = 0x10a90a;
            free(psVar15);
LAB_0010a90a:
            if (*(char *)((long)param_8 + 0x33) != '\0') {
              *(undefined8 *)(puVar35 + -8) = 0x10ae40;
              iVar10 = setfscreatecon();
              if (iVar10 != 0) goto LAB_00108bf7;
            }
            __group = local_288.st_gid;
            __owner = local_288.st_uid;
            iVar10 = local_330;
            psVar29 = local_340;
            uVar43 = 1;
            cVar7 = '\0';
            if (*(char *)((long)param_8 + 0x1d) != '\0') {
              local_358 = (stat **)CONCAT71(local_358._1_7_,*(char *)((long)param_8 + 0x1d));
              *(undefined8 *)(puVar35 + -8) = 0x10a954;
              iVar10 = fchownat(iVar10,(char *)psVar29,__owner,__group,0x100);
              if (iVar10 == 0) {
                uVar43 = (ulong)local_358 & 0xff;
                cVar7 = '\0';
              }
              else {
                *(undefined8 *)(puVar35 + -8) = 0x10a96c;
                uVar12 = FUN_001076c0();
                cVar7 = '\0';
                uVar43 = (ulong)uVar12;
                if ((char)uVar12 == '\0') {
                  *(undefined8 *)(puVar35 + -8) = 0x10a98d;
                  uVar43 = dcgettext(0,"failed to preserve ownership for %s",5);
                  *(undefined8 *)(puVar35 + -8) = 0x10a995;
                  piVar17 = __errno_location();
                  iVar10 = *piVar17;
                  *(undefined8 *)(puVar35 + -8) = 0x10a9aa;
                  error(0,iVar10,uVar43);
                  if (*(char *)((long)param_8 + 0x32) != '\0') goto LAB_00108658;
                  uVar43 = (ulong)local_358 & 0xff;
                  cVar7 = '\0';
                }
              }
            }
            goto LAB_00107f17;
          }
          *(undefined8 *)(puVar35 + -8) = 0x10a42d;
          uVar43 = FUN_00114440(4,param_1);
          pcVar37 = "%s has unknown file type";
LAB_00108394:
          *(undefined8 *)(puVar35 + -8) = 0x10839b;
          uVar16 = dcgettext(0,pcVar37,5);
          *(undefined8 *)(puVar35 + -8) = 0x1083ac;
          error(0,0,uVar16);
          goto LAB_00108658;
        }
        uVar13 = param_8[1];
        local_310 = (stat *)0x0;
        local_370 = (stat *)CONCAT44(local_370._4_4_,local_288.st_mode);
        local_358 = (stat **)CONCAT71(local_358._1_7_,*(undefined1 *)((long)param_8 + 0x31));
        local_360 = (undefined8 *)CONCAT71(local_360._1_7_,*(undefined1 *)((long)param_8 + 0x35));
        *(undefined8 *)(puVar35 + -8) = 0x10946d;
        uVar13 = FUN_0010e620(param_1,(ulong)(uVar13 == 2) << 0x11);
        uVar43 = (ulong)uVar13;
        if ((int)uVar13 < 0) {
          *(undefined8 *)(puVar35 + -8) = 0x10a25e;
          param_1 = (stat *)FUN_00114440(4,param_1);
          pcVar37 = "cannot open %s for reading";
          goto LAB_00108638;
        }
        *(undefined8 *)(puVar35 + -8) = 0x109486;
        iVar11 = fstat(uVar13,&local_d8);
        iVar10 = local_330;
        psVar15 = local_340;
        if (iVar11 == 0) {
          if ((local_288.st_ino != local_d8.st_ino) || (local_288.st_dev != local_d8.st_dev)) {
            *(undefined8 *)(puVar35 + -8) = 0x109fe2;
            FUN_00114440(4,param_1);
            *(undefined8 *)(puVar35 + -8) = 0x109ff8;
            uVar16 = dcgettext(0,"skipping file %s, as it was replaced while being copied",5);
            iVar10 = 0;
            goto LAB_0010a000;
          }
          local_3b0 = (char *)(CONCAT44(local_3b0._4_4_,uVar12) & 0xffffffff000001ff);
          if ((byte)local_320 != '\0') goto LAB_0010a500;
          bVar44 = (byte)local_358 == '\0';
          *(undefined8 *)(puVar35 + -8) = 0x1094f8;
          uVar12 = FUN_001125a0(iVar10,psVar15,(-(uint)bVar44 & 0xfffffe00) + 0x201);
          uVar42 = (ulong)uVar12;
          *(undefined8 *)(puVar35 + -8) = 0x1094ff;
          ppsVar26 = (stat **)__errno_location();
          iVar11 = local_330;
          psVar29 = local_338;
          psVar15 = local_340;
          iVar10 = *(int *)ppsVar26;
          if ((int)uVar12 < 0) {
            if (iVar10 != 2) {
              cVar7 = '\0';
              if (*(char *)((long)param_8 + 0x16) != '\0') {
                local_378 = ppsVar26;
                *(undefined8 *)(puVar35 + -8) = 0x10ad87;
                iVar10 = unlinkat(iVar11,(char *)psVar15,0);
                psVar15 = local_338;
                if (iVar10 == 0) {
                  if ((char)param_8[0xf] != '\0') {
                    *(undefined8 *)(puVar35 + -8) = 0x10b7dc;
                    uVar16 = FUN_00114440(4,psVar15);
                    *(undefined8 *)(puVar35 + -8) = 0x10b7f2;
                    uVar18 = dcgettext(0,"removed %s\n",5);
                    *(undefined8 *)(puVar35 + -8) = 0x10b804;
                    __printf_chk(1,uVar18,uVar16);
                  }
                }
                else if (*(int *)local_378 != 2) {
                  local_358 = local_378;
                  *(undefined8 *)(puVar35 + -8) = 0x10b6b1;
                  FUN_00114440(4,psVar15);
                  *(undefined8 *)(puVar35 + -8) = 0x10b6c7;
                  uVar16 = dcgettext(0,"cannot remove %s",5);
                  iVar10 = *(int *)local_358;
                  goto LAB_0010a000;
                }
                goto LAB_0010ada1;
              }
LAB_0010a58d:
              local_320 = (stat *)CONCAT71(local_320._1_7_,cVar7);
              goto LAB_0010a593;
            }
LAB_0010ada1:
            psVar15 = local_338;
            if (*(long *)(param_8 + 10) == 0) {
LAB_0010a500:
              iVar10 = local_330;
              psVar15 = local_340;
              uVar12 = ~(uint)local_388 & (uint)local_3b0;
              if (((byte)local_360 != '\0') && (*(char *)((long)param_8 + 0x1b) == '\0')) {
                uVar12 = uVar12 | 0x80;
              }
              local_380 = (stat *)CONCAT44(local_380._4_4_,uVar12);
              *(undefined8 *)(puVar35 + -8) = 0x10a548;
              uVar12 = FUN_001125a0(iVar10,psVar15,0xc1);
              uVar42 = (ulong)uVar12;
              *(undefined8 *)(puVar35 + -8) = 0x10a54f;
              local_378 = (stat **)__errno_location();
              iVar11 = local_330;
              psVar15 = local_340;
              bVar45 = *(int *)local_378 == 0x11 & (byte)(uVar12 >> 0x1f);
              local_320 = (stat *)CONCAT71(local_320._1_7_,bVar45);
              uVar12 = (uint)local_380;
              iVar10 = *(int *)local_378;
              if (bVar45 != 0) {
                cVar7 = (char)param_8[6];
                iVar10 = 0x11;
                if (cVar7 != '\0') goto LAB_0010a58d;
                local_390 = (stat **)CONCAT44(local_390._4_4_,0x11);
                *(undefined8 *)(puVar35 + -8) = 0x10aa39;
                sVar25 = readlinkat(iVar11,(char *)psVar15,&local_41,1);
                iVar11 = local_330;
                psVar29 = local_338;
                psVar15 = local_340;
                uVar12 = (uint)local_380;
                iVar10 = (int)local_390;
                if (-1 < sVar25) {
                  if (*(char *)((long)param_8 + 0x3e) == '\0') {
                    local_358 = (stat **)((ulong)local_358 & 0xffffffffffffff00);
                    *(undefined8 *)(puVar35 + -8) = 0x10b873;
                    FUN_00114440(4,psVar29);
                    *(undefined8 *)(puVar35 + -8) = 0x10b889;
                    uVar16 = dcgettext(0,"not writing through dangling symlink %s",5);
                    psVar15 = (stat *)0x0;
                    *(undefined8 *)(puVar35 + -8) = 0x10b89c;
                    error(0,0,uVar16);
                    uVar40 = SUB81(local_358,0);
                    goto LAB_00109710;
                  }
                  local_320 = (stat *)CONCAT44(local_320._4_4_,(uint)local_380);
                  *(undefined8 *)(puVar35 + -8) = 0x10aa7d;
                  uVar12 = FUN_001125a0(iVar11,psVar15,0x41);
                  uVar42 = (ulong)uVar12;
                  iVar10 = *(int *)local_378;
                  uVar12 = (uint)local_320;
                }
              }
              psVar15 = local_338;
              bVar45 = iVar10 == 0x15 & (byte)(uVar42 >> 0x1f);
              local_320 = (stat *)CONCAT71(local_320._1_7_,bVar45);
              if (bVar45 != 0) {
                iVar10 = 0x15;
                if ((char)local_338->st_dev != '\0') {
                  *(undefined8 *)(puVar35 + -8) = 0x10aac4;
                  sVar22 = strlen((char *)psVar15);
                  iVar10 = (*(char *)((long)psVar15->__unused + (sVar22 - 0x79)) != '/') + 0x14;
                }
LAB_0010a593:
                psVar15 = local_338;
                local_358 = (stat **)CONCAT44(local_358._4_4_,iVar10);
                *(undefined8 *)(puVar35 + -8) = 0x10a5ab;
                FUN_00114440(4,psVar15);
                *(undefined8 *)(puVar35 + -8) = 0x10a5c1;
                uVar16 = dcgettext(0,"cannot create regular file %s",5);
                iVar10 = (int)local_358;
                goto LAB_0010a000;
              }
              local_3c0 = (uint *)CONCAT44(local_3c0._4_4_,~(uint)local_3b0 & uVar12);
              if ((int)uVar42 < 0) {
                local_320 = (stat *)CONCAT71(local_320._1_7_,1);
                goto LAB_0010a593;
              }
              local_3d8 = (uint)local_388;
              local_320 = (stat *)CONCAT71(local_320._1_7_,1);
              goto LAB_0010954f;
            }
            uVar42 = (ulong)local_3b0 & 0xffffffff;
            *(undefined8 *)(puVar35 + -8) = 0x10adc9;
            cVar7 = FUN_001072a0(param_1,psVar15,uVar42,1,param_8);
            if (cVar7 != '\0') goto LAB_0010a500;
            local_320 = (stat *)((ulong)local_320 & 0xffffffffffffff00);
            psVar15 = (stat *)0x0;
            uVar40 = 0;
          }
          else {
            if ((*(long *)(param_8 + 10) == 0) && (*(char *)((long)param_8 + 0x33) == '\0')) {
LAB_0010953b:
              local_3d8 = 0;
              local_3c0 = (uint *)((ulong)local_3c0 & 0xffffffff00000000);
LAB_0010954f:
              uVar12 = (uint)uVar42;
              if ((byte)local_358 == '\0') {
LAB_0010a64a:
                uVar36 = *(byte *)((long)param_8 + 0x1d) | (uint)local_3c0;
                local_358 = (stat **)CONCAT44(local_358._4_4_,uVar36);
                if (uVar36 != 0) {
                  local_358 = (stat **)((ulong)local_358 & 0xffffffffffffff00);
                  goto LAB_00109568;
                }
                local_168._24_8_ = local_168._24_8_ & 0xffffffff00000000;
                if (*(char *)((long)param_8 + 0x1f) != '\0') {
LAB_0010a676:
                  iVar10 = local_330;
                  psVar15 = local_340;
                  local_2a8 = (stat *)local_288.st_atim.tv_sec;
                  local_2a0 = local_288.st_atim.tv_nsec;
                  local_298 = local_288.st_mtim.tv_sec;
                  local_290 = local_288.st_mtim.tv_nsec;
                  *(undefined8 *)(puVar35 + -8) = 0x10a6cc;
                  iVar10 = FUN_0010e6a0(uVar42,iVar10,psVar15,&local_2a8,0);
                  psVar15 = local_338;
                  if (iVar10 != 0) {
                    *(undefined8 *)(puVar35 + -8) = 0x10b175;
                    local_380 = (stat *)FUN_00114440(4,psVar15);
                    *(undefined8 *)(puVar35 + -8) = 0x10b18f;
                    local_378 = (stat **)dcgettext(0,"preserving times for %s",5);
                    *(undefined8 *)(puVar35 + -8) = 0x10b19b;
                    piVar17 = __errno_location();
                    ppsVar26 = local_378;
                    iVar10 = *piVar17;
                    *(undefined8 *)(puVar35 + -8) = 0x10b1b4;
                    error(0,iVar10,ppsVar26);
                    if (*(char *)((long)param_8 + 0x32) != '\0') goto LAB_001096e2;
                  }
                  goto LAB_0010a6d4;
                }
LAB_0010a74a:
                psVar15 = local_338;
                if ((byte)local_360 == '\0') {
LAB_0010a757:
                  bVar45 = 1;
                }
                else {
                  *(undefined8 *)(puVar35 + -8) = 0x10aec7;
                  cVar7 = FUN_00106220(param_1,uVar43,psVar15,uVar42,param_8);
                  if (cVar7 != '\0') goto LAB_0010a757;
                  bVar45 = *(byte *)((long)param_8 + 0x36) ^ 1;
                }
                psVar15 = local_338;
                if ((*(ulong *)(param_8 + 6) & 0xff0000000000ff) == 0) {
                  if (*(char *)((long)param_8 + 0x39) == '\0') {
                    bVar8 = (byte)local_320 & (byte)param_8[8];
                    local_360 = (undefined8 *)CONCAT71(local_360._1_7_,bVar8);
                    if (bVar8 == 0) {
                      if ((int)local_358 != 0 || local_3d8 != 0) {
                        if (DAT_00123010 == 0xffffffff) {
                          local_370 = (stat *)CONCAT71(local_370._1_7_,bVar45);
                          *(undefined8 *)(puVar35 + -8) = 0x10b9ec;
                          DAT_00123010 = umask(0);
                          local_360 = (undefined8 *)CONCAT44(local_360._4_4_,DAT_00123010);
                          *(undefined8 *)(puVar35 + -8) = 0x10b9ff;
                          umask(DAT_00123010);
                          bVar45 = (byte)local_370;
                          if ((~(uint)local_360 & local_3d8) != 0 || (int)local_358 != 0) {
                            bVar45 = (byte)local_370;
                            if ((uint)local_360 == 0xffffffff) {
                              local_358 = (stat **)CONCAT71(local_358._1_7_,(byte)local_370);
                              *(undefined8 *)(puVar35 + -8) = 0x10ba40;
                              DAT_00123010 = umask(0);
                              *(undefined8 *)(puVar35 + -8) = 0x10ba4d;
                              umask(DAT_00123010);
                              bVar45 = (byte)local_358;
                            }
                            goto LAB_0010a7ca;
                          }
                        }
                        else if ((~DAT_00123010 & local_3d8) != 0 || (int)local_358 != 0) {
LAB_0010a7ca:
                          local_358 = (stat **)CONCAT71(local_358._1_7_,bVar45);
                          uVar36 = (uint)local_3b0 & ~DAT_00123010;
                          *(undefined8 *)(puVar35 + -8) = 0x10a7e2;
                          iVar10 = fchmod(uVar12,uVar36);
                          psVar15 = local_338;
                          bVar45 = (byte)local_358;
                          if (iVar10 != 0) {
                            local_370 = (stat *)CONCAT71(local_370._1_7_,(byte)local_358);
                            *(undefined8 *)(puVar35 + -8) = 0x10a80a;
                            local_360 = (undefined8 *)FUN_00114440(4,psVar15);
                            *(undefined8 *)(puVar35 + -8) = 0x10a824;
                            local_358 = (stat **)dcgettext(0,"preserving permissions for %s",5);
                            *(undefined8 *)(puVar35 + -8) = 0x10a830;
                            piVar17 = __errno_location();
                            ppsVar26 = local_358;
                            iVar10 = *piVar17;
                            *(undefined8 *)(puVar35 + -8) = 0x10a849;
                            error(0,iVar10,ppsVar26);
                            bVar45 = (byte)local_370;
                            if (*(char *)((long)param_8 + 0x32) != '\0') {
                              bVar45 = 0;
                            }
                          }
                        }
                      }
                    }
                    else {
                      if (DAT_00123010 == 0xffffffff) {
                        local_320 = (stat *)CONCAT71(local_320._1_7_,bVar45);
                        *(undefined8 *)(puVar35 + -8) = 0x10b817;
                        DAT_00123010 = umask(0);
                        *(undefined8 *)(puVar35 + -8) = 0x10b824;
                        umask(DAT_00123010);
                        bVar45 = (byte)local_320;
                      }
                      psVar15 = local_338;
                      local_358 = (stat **)CONCAT71(local_358._1_7_,bVar45);
                      *(undefined8 *)(puVar35 + -8) = 0x10b7a5;
                      iVar10 = FUN_0010c8e0(psVar15,uVar42,~DAT_00123010 & 0x1b6);
                      local_320 = (stat *)CONCAT71(local_320._1_7_,(byte)local_360);
                      bVar45 = (byte)local_358;
                      if (iVar10 != 0) {
                        bVar45 = 0;
                      }
                    }
                  }
                  else {
                    uVar36 = param_8[4];
                    local_358 = (stat **)CONCAT71(local_358._1_7_,bVar45);
                    *(undefined8 *)(puVar35 + -8) = 0x10b041;
                    iVar10 = FUN_0010c8e0(psVar15,uVar42,uVar36);
                    bVar45 = (byte)local_358;
                    if (iVar10 != 0) {
                      bVar45 = 0;
                    }
                  }
                }
                else {
                  uVar24 = (ulong)local_370 & 0xffffffff;
                  local_358 = (stat **)CONCAT71(local_358._1_7_,bVar45);
                  *(undefined8 *)(puVar35 + -8) = 0x10ae90;
                  iVar10 = FUN_0010c830(param_1,uVar43,psVar15,uVar42,uVar24);
                  bVar45 = (byte)local_358;
                  if ((iVar10 != 0) && (*(char *)((long)param_8 + 0x32) != '\0')) {
                    bVar45 = 0;
                  }
                }
              }
              else {
                if (param_8[0x11] != 0) {
                  *(undefined8 *)(puVar35 + -8) = 0x10aeee;
                  iVar10 = ioctl(uVar12,0x40049409,uVar43);
                  if (iVar10 == 0) goto LAB_0010a64a;
                  if (param_8[0x11] == 2) {
                    *(undefined8 *)(puVar35 + -8) = 0x10af13;
                    local_370 = (stat *)FUN_00114310(1,4,param_1);
                    psVar15 = local_338;
                    *(undefined8 *)(puVar35 + -8) = 0x10af2d;
                    local_360 = (undefined8 *)FUN_00114310(0,4,psVar15);
                    *(undefined8 *)(puVar35 + -8) = 0x10af47;
                    local_358 = (stat **)dcgettext(0,"failed to clone %s from %s",5);
                    *(undefined8 *)(puVar35 + -8) = 0x10af53;
                    piVar17 = __errno_location();
                    ppsVar26 = local_358;
                    puVar28 = local_360;
                    psVar15 = local_370;
                    iVar10 = *piVar17;
                    *(undefined8 *)(puVar35 + -8) = 0x10af73;
                    error(0,iVar10,ppsVar26,puVar28,psVar15);
                    bVar45 = 0;
                    goto LAB_001096e5;
                  }
                }
LAB_00109568:
                *(undefined8 *)(puVar35 + -8) = 0x109576;
                iVar10 = fstat(uVar12,&local_168);
                psVar15 = local_338;
                if (iVar10 == 0) {
                  local_378 = (stat **)((ulong)local_378 & 0xffffffff00000000);
                  uVar36 = (uint)local_3c0 | local_168.st_mode;
                  if (local_168.st_mode != uVar36) {
                    *(undefined8 *)(puVar35 + -8) = 0x10959d;
                    iVar10 = fchmod(uVar12,uVar36);
                    uVar31 = (int)local_378;
                    if (iVar10 == 0) {
                      uVar31 = (uint)local_3c0;
                    }
                    local_3c0 = (uint *)CONCAT44(local_3c0._4_4_,uVar31);
                  }
                  if ((byte)local_358 == '\0') {
LAB_0010ac58:
                    local_358 = (stat **)CONCAT44(local_358._4_4_,(uint)local_3c0);
                    if (*(char *)((long)param_8 + 0x1f) != '\0') goto LAB_0010a676;
LAB_0010a6d4:
                    if (*(char *)((long)param_8 + 0x1d) != '\0') {
                      uVar24 = (ulong)local_288.st_uid;
                      uVar32 = (ulong)local_288.st_gid;
                      if ((local_288.st_uid != local_168.st_uid) ||
                         (local_288.st_gid != local_168.st_gid)) {
                        *(stat **)(puVar35 + -8) = &local_168;
                        psVar29 = local_338;
                        *(ulong *)(puVar35 + -0x10) = (ulong)local_320 & 0xff;
                        psVar15 = local_340;
                        *(ulong *)(puVar35 + -0x18) = uVar32;
                        iVar10 = local_330;
                        *(ulong *)(puVar35 + -0x20) = uVar24;
                        *(undefined8 *)(puVar35 + -0x28) = 0x10a72f;
                        iVar10 = FUN_00107080(param_8,psVar29,iVar10,psVar15,uVar42);
                        if (iVar10 == -1) goto LAB_001096e2;
                        if (iVar10 == 0) {
                          local_370 = (stat *)((ulong)local_370 & 0xfffffffffffff1ff);
                        }
                      }
                    }
                    goto LAB_0010a74a;
                  }
                  local_3d0 = (stat **)local_168.st_blksize;
                  local_390 = (stat **)local_168.st_blksize;
                  if ((stat **)0x1ffffffffffe0000 < (stat **)(local_168.st_blksize + -0x20000)) {
                    local_390 = (stat **)0x20000;
                    local_3d0 = (stat **)0x200;
                    if ((undefined *)(local_168.st_blksize + -1) < (undefined *)0x2000000000000000)
                    {
                      local_3d0 = (stat **)local_168.st_blksize;
                    }
                  }
                  if ((local_d8.st_mode & 0xf000) == 0x8000) {
                    iVar10 = 1;
                    if (local_d8.st_size / 0x200 <= local_d8.st_blocks) goto LAB_0010ab10;
                    *(undefined8 *)(puVar35 + -8) = 0x109662;
                    local_398 = (undefined8 *)lseek(uVar13,0,3);
                    if (-1 < (long)local_398) {
LAB_0010b4d9:
                      iVar10 = 3;
                      goto LAB_0010ab10;
                    }
                    *(undefined8 *)(puVar35 + -8) = 0x109677;
                    local_378 = (stat **)__errno_location();
                    iVar10 = *(int *)local_378;
                    if (iVar10 == 6) goto LAB_0010b4d9;
                    if ((iVar10 == 0x16) || (iVar10 == 0x5f)) {
                      iVar10 = 2;
                      goto LAB_0010ab10;
                    }
                    *(undefined8 *)(puVar35 + -8) = 0x1096ab;
                    local_358 = (stat **)FUN_00114440(4,param_1);
                    *(undefined8 *)(puVar35 + -8) = 0x1096c5;
                    uVar16 = dcgettext(0,"cannot lseek %s",5);
                    uVar31 = *(undefined4 *)local_378;
                    *(undefined8 *)(puVar35 + -8) = 0x1096e2;
                    error(0,uVar31,uVar16);
                  }
                  else {
                    iVar10 = 1;
LAB_0010ab10:
                    if (((local_168.st_mode & 0xf000) == 0x8000) &&
                       ((param_8[3] == 3 || ((iVar10 != 1 && (param_8[3] == 2)))))) {
                      local_378 = (stat **)CONCAT44(local_378._4_4_,iVar10);
                      *(undefined8 *)(puVar35 + -8) = 0x10b0b5;
                      posix_fadvise(uVar13,0,0,2);
                      if ((int)local_378 != 3) {
                        local_3d4 = (uint)(param_8[0x11] != 0);
                        goto LAB_0010abf8;
                      }
                      local_3d4 = (uint)(param_8[0x11] != 0);
                      local_3c8 = param_8[3];
LAB_0010b1f7:
                      local_312[0] = 0;
                      local_378 = (stat **)local_d8.st_size;
                      if ((long)local_398 < 0) {
                        local_3b8 = (stat **)0x0;
                        bVar44 = 0 < local_d8.st_size;
LAB_0010b5a2:
                        ppsVar26 = local_378;
                        if (local_3c8 == 1) {
                          lVar30 = (long)local_378 - (long)local_3b8;
                          *(undefined8 *)(puVar35 + -8) = 0x10b900;
                          cVar7 = FUN_00106170(uVar42,lVar30);
                          if (cVar7 != '\0') goto LAB_0010b423;
LAB_0010b908:
                          psVar15 = local_338;
                          *(undefined8 *)(puVar35 + -8) = 0x10b919;
                          local_378 = (stat **)FUN_00114440(4,psVar15);
                          pcVar37 = "failed to extend %s";
                          goto LAB_0010b541;
                        }
                        local_380 = (stat *)CONCAT71(local_380._1_7_,bVar44);
                        *(undefined8 *)(puVar35 + -8) = 0x10b5c3;
                        iVar10 = ftruncate(uVar12,(__off_t)ppsVar26);
                        ppsVar26 = local_3b8;
                        if (iVar10 != 0) goto LAB_0010b908;
                        if ((local_3c8 == 3) && ((char)local_380 != '\0')) {
                          lVar30 = (long)local_378 - (long)local_3b8;
                          *(undefined8 *)(puVar35 + -8) = 0x10b604;
                          iVar10 = fallocate(uVar12,3,(__off_t)ppsVar26,lVar30);
                          if (-1 < iVar10) goto LAB_0010b423;
                          *(undefined8 *)(puVar35 + -8) = 0x10b611;
                          psVar29 = (stat *)__errno_location();
                          psVar15 = local_338;
                          iVar10 = (int)psVar29->st_dev;
                          if (iVar10 == 0x5f || iVar10 == 0x26) goto LAB_0010b423;
                          local_378 = (stat **)CONCAT71(local_378._1_7_,
                                                        iVar10 == 0x5f || iVar10 == 0x26);
                          local_380 = psVar29;
                          *(undefined8 *)(puVar35 + -8) = 0x10b64b;
                          local_358 = (stat **)FUN_00114440(4,psVar15);
                          *(undefined8 *)(puVar35 + -8) = 0x10b665;
                          uVar16 = dcgettext(0,"error deallocating %s",5);
                          _Var3 = local_380->st_dev;
                          *(undefined8 *)(puVar35 + -8) = 0x10b682;
                          error(0,(int)_Var3,uVar16);
                          cVar7 = (char)local_378;
                        }
                        else {
LAB_0010b423:
                          cVar7 = (byte)local_358;
                        }
                      }
                      else {
                        local_3b8 = (stat **)0x0;
                        local_3e0 = (undefined8 *)0x0;
                        local_3a8 = (undefined8 *)CONCAT71(local_3a8._1_7_,(byte)local_358);
                        lVar30 = 0;
                        puVar28 = local_398;
                        __offset = local_3e0;
                        local_3a0 = param_8;
                        local_398 = puVar14;
                        local_380 = param_1;
                        do {
                          local_3e0 = __offset;
                          __offset = puVar28;
                          *(undefined8 *)(puVar35 + -8) = 0x10b266;
                          ppsVar26 = (stat **)lseek(uVar13,(__off_t)__offset,4);
                          if ((long)ppsVar26 < 0) {
                            *(undefined8 *)(puVar35 + -8) = 0x10b277;
                            piVar17 = __errno_location();
                            param_1 = local_380;
                            if (*piVar17 != 6) goto LAB_0010b521;
                            ppsVar26 = local_378;
                            if ((long)local_378 <= (long)__offset) {
                              *(undefined8 *)(puVar35 + -8) = 0x10b443;
                              ppsVar26 = (stat **)lseek(uVar13,0,2);
                              param_1 = local_380;
                              if ((long)ppsVar26 < 0) goto LAB_0010b521;
                              local_378 = ppsVar26;
                              if ((long)ppsVar26 <= (long)__offset) goto LAB_0010b3ff;
                            }
                          }
                          else if ((long)local_378 < (long)ppsVar26) {
                            local_378 = ppsVar26;
                          }
                          *(undefined8 *)(puVar35 + -8) = 0x10b2a0;
                          _Var27 = lseek(uVar13,(__off_t)__offset,0);
                          psVar15 = local_338;
                          param_1 = local_380;
                          if (_Var27 < 0) goto LAB_0010b521;
                          lVar30 = (long)__offset + (-lVar30 - (long)local_3e0);
                          if (lVar30 == 0) {
                            lVar30 = (long)ppsVar26 - (long)__offset;
                            local_3a8 = (undefined8 *)((ulong)local_3a8 & 0xffffffffffffff00);
                          }
                          else if (local_3c8 == 1) {
                            *(undefined8 *)(puVar35 + -8) = 0x10b46e;
                            cVar7 = FUN_00106170(uVar42,lVar30);
                            psVar15 = local_338;
                            param_1 = local_380;
                            puVar14 = local_398;
                            param_8 = local_3a0;
                            if (cVar7 == '\0') {
                              *(undefined8 *)(puVar35 + -8) = 0x10b982;
                              local_378 = (stat **)FUN_001147c0(0,3,psVar15);
                              pcVar37 = "%s: write failed";
                              goto LAB_0010b541;
                            }
                            lVar30 = (long)ppsVar26 - (long)__offset;
                            local_3a8 = (undefined8 *)((ulong)local_3a8 & 0xffffffffffffff00);
                          }
                          else {
                            bVar44 = local_3c8 == 3;
                            *(undefined8 *)(puVar35 + -8) = 0x10b2e1;
                            cVar7 = FUN_00106920(uVar42,psVar15,bVar44);
                            local_3a8 = (undefined8 *)CONCAT71(local_3a8._1_7_,cVar7);
                            puVar14 = local_398;
                            param_8 = local_3a0;
                            param_1 = local_380;
                            if (cVar7 == '\0') goto LAB_0010b56d;
                            lVar30 = (long)ppsVar26 - (long)__offset;
                          }
                          psVar15 = local_380;
                          *(undefined1 **)(puVar35 + -8) = local_312 + 1;
                          *(undefined8 **)(puVar35 + -0x10) = &local_308;
                          uVar24 = (ulong)local_3d4;
                          *(long *)(puVar35 + -0x18) = lVar30;
                          *(stat **)(puVar35 + -0x20) = local_338;
                          *(stat **)(puVar35 + -0x28) = psVar15;
                          *(ulong *)(puVar35 + -0x30) = uVar24;
                          *(undefined8 *)(puVar35 + -0x38) = 0x10b342;
                          cVar7 = FUN_00106a30(uVar43,uVar42,&local_310);
                          puVar14 = local_398;
                          param_8 = local_3a0;
                          param_1 = local_380;
                          if (cVar7 == '\0') goto LAB_0010b56d;
                          ppsVar26 = (stat **)(local_308 + (long)__offset);
                          bVar45 = (byte)local_3a8;
                          if (local_308 != 0) {
                            bVar45 = local_312[1];
                          }
                          local_3a8 = (undefined8 *)CONCAT71(local_3a8._1_7_,bVar45);
                          local_3b8 = ppsVar26;
                          if (local_308 < lVar30) goto LAB_0010b3ff;
                          *(undefined8 *)(puVar35 + -8) = 0x10b391;
                          puVar28 = (undefined8 *)lseek(uVar13,(__off_t)ppsVar26,3);
                        } while (-1 < (long)puVar28);
                        *(undefined8 *)(puVar35 + -8) = 0x10b3e8;
                        piVar17 = __errno_location();
                        param_1 = psVar15;
                        ppsVar26 = local_378;
                        if (*piVar17 == 6) {
LAB_0010b3ff:
                          local_378 = ppsVar26;
                          bVar44 = (long)local_3b8 < (long)local_378;
                          puVar14 = local_398;
                          param_8 = local_3a0;
                          if (((long)local_3b8 < (long)local_378) || ((byte)local_3a8 != '\0'))
                          goto LAB_0010b5a2;
                          goto LAB_0010b423;
                        }
LAB_0010b521:
                        puVar14 = local_398;
                        param_8 = local_3a0;
                        *(undefined8 *)(puVar35 + -8) = 0x10b52e;
                        local_378 = (stat **)FUN_00114440(4,param_1);
                        pcVar37 = "cannot lseek %s";
LAB_0010b541:
                        *(undefined8 *)(puVar35 + -8) = 0x10b548;
                        local_358 = (stat **)dcgettext(0,pcVar37,5);
                        *(undefined8 *)(puVar35 + -8) = 0x10b554;
                        piVar17 = __errno_location();
                        ppsVar26 = local_358;
                        iVar10 = *piVar17;
                        *(undefined8 *)(puVar35 + -8) = 0x10b56d;
                        error(0,iVar10,ppsVar26);
LAB_0010b56d:
                        cVar7 = '\0';
                      }
                    }
                    else {
                      local_378 = (stat **)CONCAT44(local_378._4_4_,iVar10);
                      *(undefined8 *)(puVar35 + -8) = 0x10ab3e;
                      posix_fadvise(uVar13,0,0,2);
                      ppsVar26 = local_390;
                      _Var39 = local_d8.st_blksize;
                      if (0x1ffffffffffe0000 < local_d8.st_blksize - 0x20000U) {
                        _Var39 = 0x20000;
                      }
                      *(undefined8 *)(puVar35 + -8) = 0x10ab78;
                      ppsVar26 = (stat **)FUN_0010da00(_Var39,ppsVar26,0x7fffffffffffffff);
                      if (((local_d8.st_mode & 0xf000) == 0x8000) &&
                         ((ulong)local_d8.st_size < local_390)) {
                        local_390 = (stat **)(local_d8.st_size + 1);
                      }
                      puVar2 = (undefined *)((long)ppsVar26 + -1) + (long)local_390;
                      local_390 = ppsVar26;
                      if (0 < (long)(puVar2 + -((ulong)puVar2 % (ulong)ppsVar26))) {
                        local_390 = (stat **)(puVar2 + -((ulong)puVar2 % (ulong)ppsVar26));
                      }
                      local_3d4 = (uint)(param_8[0x11] != 0);
                      if ((int)local_378 == 3) {
                        local_3c8 = 1;
                        goto LAB_0010b1f7;
                      }
                      local_3d0 = (stat **)0x0;
LAB_0010abf8:
                      ppsVar26 = local_3d0;
                      local_312[0] = 0;
                      *(undefined1 **)(puVar35 + -8) = local_312;
                      ppsVar4 = local_390;
                      *(undefined8 **)(puVar35 + -0x10) = &local_308;
                      uVar24 = (ulong)local_3d4;
                      *(undefined8 *)(puVar35 + -0x18) = 0xffffffffffffffff;
                      *(stat **)(puVar35 + -0x20) = local_338;
                      *(stat **)(puVar35 + -0x28) = param_1;
                      *(ulong *)(puVar35 + -0x30) = uVar24;
                      *(undefined8 *)(puVar35 + -0x38) = 0x10ac38;
                      cVar7 = FUN_00106a30(uVar43,uVar42,&local_310,ppsVar4,ppsVar26);
                    }
                    _Var27 = local_308;
                    if (cVar7 == '\x01') {
                      if (local_312[0] != 0) {
                        *(undefined8 *)(puVar35 + -8) = 0x10b107;
                        iVar10 = ftruncate(uVar12,_Var27);
                        psVar15 = local_338;
                        if (iVar10 < 0) {
                          *(undefined8 *)(puVar35 + -8) = 0x10b120;
                          local_360 = (undefined8 *)FUN_00114440(4,psVar15);
                          pcVar37 = "failed to extend %s";
                          goto LAB_0010b133;
                        }
                      }
                      goto LAB_0010ac58;
                    }
                  }
                }
                else {
                  *(undefined8 *)(puVar35 + -8) = 0x10b1d5;
                  local_360 = (undefined8 *)FUN_00114440(4,psVar15);
                  pcVar37 = "cannot fstat %s";
LAB_0010b133:
                  *(undefined8 *)(puVar35 + -8) = 0x10b13a;
                  local_358 = (stat **)dcgettext(0,pcVar37,5);
                  *(undefined8 *)(puVar35 + -8) = 0x10b146;
                  piVar17 = __errno_location();
                  ppsVar26 = local_358;
                  iVar10 = *piVar17;
                  *(undefined8 *)(puVar35 + -8) = 0x10b15f;
                  error(0,iVar10,ppsVar26);
                }
LAB_001096e2:
                bVar45 = 0;
              }
            }
            else {
              *(undefined8 *)(puVar35 + -8) = 0x109529;
              cVar7 = FUN_00107510(psVar29,0,param_8);
              if ((cVar7 != '\0') || ((char)param_8[0xd] == '\0')) goto LAB_0010953b;
              local_320 = (stat *)((ulong)local_320 & 0xffffffffffffff00);
              bVar45 = 0;
            }
LAB_001096e5:
            local_358 = (stat **)CONCAT71(local_358._1_7_,bVar45);
            *(undefined8 *)(puVar35 + -8) = 0x1096f3;
            iVar10 = close(uVar12);
            psVar15 = local_338;
            if (iVar10 < 0) {
              *(undefined8 *)(puVar35 + -8) = 0x10adf3;
              local_358 = (stat **)FUN_00114440(4,psVar15);
              *(undefined8 *)(puVar35 + -8) = 0x10ae0d;
              uVar16 = dcgettext(0,"failed to close %s",5);
              *(undefined8 *)(puVar35 + -8) = 0x10ae15;
              piVar17 = __errno_location();
              iVar10 = *piVar17;
              *(undefined8 *)(puVar35 + -8) = 0x10ae2a;
              error(0,iVar10,uVar16);
              uVar40 = 0;
              psVar15 = local_310;
            }
            else {
              uVar40 = SUB81(local_358,0);
              psVar15 = local_310;
            }
          }
        }
        else {
          *(undefined8 *)(puVar35 + -8) = 0x10a4a8;
          local_358 = (stat **)FUN_00114440(4,param_1);
          *(undefined8 *)(puVar35 + -8) = 0x10a4c2;
          uVar16 = dcgettext(0,"cannot fstat %s",5);
          *(undefined8 *)(puVar35 + -8) = 0x10a4ca;
          piVar17 = __errno_location();
          iVar10 = *piVar17;
LAB_0010a000:
          psVar15 = (stat *)0x0;
          *(undefined8 *)(puVar35 + -8) = 0x10a00b;
          error(0,iVar10,uVar16);
          uVar40 = 0;
        }
LAB_00109710:
        local_358 = (stat **)CONCAT71(local_358._1_7_,uVar40);
        *(undefined8 *)(puVar35 + -8) = 0x10971f;
        iVar10 = close(uVar13);
        if (iVar10 < 0) {
          *(undefined8 *)(puVar35 + -8) = 0x10a45f;
          param_1 = (stat *)FUN_00114440(4,param_1);
          *(undefined8 *)(puVar35 + -8) = 0x10a475;
          uVar43 = dcgettext(0,"failed to close %s",5);
          *(undefined8 *)(puVar35 + -8) = 0x10a47d;
          piVar17 = __errno_location();
          iVar10 = *piVar17;
          *(undefined8 *)(puVar35 + -8) = 0x10a48e;
          error(0,iVar10,uVar43);
          *(undefined8 *)(puVar35 + -8) = 0x10a496;
          free(psVar15);
          goto LAB_00108658;
        }
        uVar43 = 0;
        *(undefined8 *)(puVar35 + -8) = 0x109741;
        free(psVar15);
        cVar7 = (byte)local_358;
        cVar9 = (byte)local_358;
      }
      else {
        uVar42 = 1;
        if (*(char *)((long)param_8 + 0x16) == '\0') {
          uVar42 = (ulong)(param_8[2] == 3);
        }
        *(ulong *)(puVar35 + -0x10) = (ulong)local_380 & 0xffffffff;
        *(undefined8 *)(puVar35 + -0x18) = 0;
        *(ulong *)(puVar35 + -0x20) = uVar42;
        *(undefined8 *)(puVar35 + -0x28) = 0x108ea9;
        cVar9 = FUN_001065d0(param_1,0xffffff9c,param_1,psVar41,iVar10);
        cVar7 = '\0';
      }
      if (cVar9 != '\0') {
LAB_00107f17:
        psVar15 = local_338;
        uVar12 = (uint)uVar43;
        bVar45 = (uint)local_348 != 0x4000;
        if ((((byte)local_320 == '\0') && (bVar8 = ((byte)param_8[5] ^ 1) & bVar45, bVar8 != 0)) &&
           ((*(long *)(param_8 + 10) != 0 ||
            (bVar45 = bVar8, *(char *)((long)param_8 + 0x33) != '\0')))) {
          local_348 = (undefined1 *)CONCAT71(local_348._1_7_,cVar7);
          local_358 = (stat **)CONCAT71(local_358._1_7_,bVar8);
          *(undefined8 *)(puVar35 + -8) = 0x108fe9;
          uVar13 = FUN_00107510(psVar15,0,param_8);
          cVar7 = (char)local_348;
          psVar15 = (stat *)(ulong)uVar13;
          bVar45 = (byte)uVar13;
          if ((byte)uVar13 == 0) {
            local_320 = (stat *)CONCAT71(local_320._1_7_,(char)param_8[0xd]);
            if ((char)param_8[0xd] != '\0') goto LAB_00108658;
            bVar45 = (byte)local_358;
          }
        }
        uVar42 = 1;
        puVar34 = puVar35;
        if ((char)local_328 == '\0') goto LAB_00107fc0;
        local_368 = local_368 & 0xffffffffffffff00;
        uVar42 = (ulong)local_328 & 0xffffffff;
        goto LAB_00107f59;
      }
    }
    else {
      if ((char)param_1->st_dev != '/') {
        *(undefined8 *)(puVar35 + -8) = 0x107e9a;
        psVar15 = (stat *)FUN_0010e480(psVar29);
        if (local_330 == -100) {
          local_358 = (stat **)&DAT_0011a14b;
          *(undefined8 *)(puVar35 + -8) = 0x107ec3;
          iVar10 = strcmp(".",(char *)psVar15);
          if (iVar10 != 0) goto LAB_00107ece;
        }
        else {
LAB_00107ece:
          *(undefined8 *)(puVar35 + -8) = 0x107eda;
          iVar11 = stat(".",&local_168);
          iVar10 = local_330;
          if (iVar11 == 0) {
            *(undefined8 *)(puVar35 + -8) = 0x109aff;
            iVar10 = fstatat(iVar10,(char *)psVar15,&local_d8,0);
            if ((iVar10 == 0) &&
               ((local_168.st_ino != local_d8.st_ino || (local_168.st_dev != local_d8.st_dev)))) {
              *(undefined8 *)(puVar35 + -8) = 0x109b23;
              free(psVar15);
              psVar29 = local_338;
              *(undefined8 *)(puVar35 + -8) = 0x109b36;
              uVar43 = FUN_001147c0(0,3,psVar29);
              pcVar37 = "%s: can make relative symbolic links only in current directory";
              goto LAB_00108394;
            }
          }
        }
        *(undefined8 *)(puVar35 + -8) = 0x107eea;
        free(psVar15);
      }
      iVar10 = local_330;
      psVar15 = local_340;
      uVar40 = *(undefined1 *)((long)param_8 + 0x16);
      *(undefined8 *)(puVar35 + -8) = 0x107f0a;
      uVar12 = FUN_0010c000(param_1,iVar10,psVar15,uVar40,0xffffffff);
      cVar7 = '\0';
      psVar15 = (stat *)(ulong)uVar12;
      if ((int)uVar12 < 1) goto LAB_00107f17;
      *(undefined8 *)(puVar35 + -8) = 0x109772;
      param_1 = (stat *)FUN_00114310(1,4,param_1);
      psVar29 = local_338;
      *(undefined8 *)(puVar35 + -8) = 0x109788;
      uVar43 = FUN_00114310(0,4,psVar29);
      *(undefined8 *)(puVar35 + -8) = 0x10979e;
      uVar16 = dcgettext(0,"cannot create symbolic link %s to %s",5);
      *(undefined8 *)(puVar35 + -8) = 0x1097b2;
      error(0,uVar12,uVar16,uVar43,param_1);
    }
LAB_00108658:
    if (*(char *)((long)param_8 + 0x33) != '\0') {
      *(undefined8 *)(puVar38 + -8) = 0x1092d7;
      iVar10 = setfscreatecon();
      if (iVar10 != 0) goto LAB_00108bf7;
    }
    puVar35 = puVar38;
    if (puVar14 == (undefined8 *)0x0) {
      *(undefined8 *)(puVar38 + -8) = 0x109323;
      FUN_0010bc60(local_288.st_ino,local_288.st_dev);
    }
    goto LAB_0010866c;
  }
  uVar12 = local_32c & 0xf000;
  if ((((uVar12 == 0x8000) ||
       (bVar45 = (uVar12 == 0xa000 || uVar12 == 0x4000) | (byte)param_8[5] ^ 1,
       psVar15 = (stat *)(ulong)bVar45, bVar45 == 0)) &&
      (psVar15 = (stat *)(ulong)(byte)param_8[6], (byte)param_8[6] == 0)) &&
     ((psVar15 = (stat *)(ulong)*(byte *)((long)param_8 + 0x3a),
      *(byte *)((long)param_8 + 0x3a) == 0 &&
      (psVar15 = (stat *)(ulong)*(byte *)((long)param_8 + 0x17),
      *(byte *)((long)param_8 + 0x17) == 0)))) {
    if (*param_8 != 0) {
      psVar15 = (stat *)0x1;
      goto LAB_00107bca;
    }
    psVar15 = (stat *)(ulong)*(byte *)((long)param_8 + 0x15);
    if (*(byte *)((long)param_8 + 0x15) != 0) goto LAB_00107bca;
    iVar10 = 0;
    if (param_5 == 0) goto LAB_00107bd0;
LAB_00108460:
    local_320 = (stat *)CONCAT71(local_320._1_7_,1);
LAB_00108467:
    local_350 = (stat *)0x0;
    psVar15 = (stat *)0x0;
    puVar35 = auStack_3e8;
    if ((int)uVar43 != 0x11) goto LAB_00107892;
  }
  else {
LAB_00107bca:
    iVar10 = 0x100;
LAB_00107bd0:
    iVar10 = fstatat(local_330,(char *)local_340,&local_1f8,iVar10);
    if (iVar10 != 0) {
      piVar17 = __errno_location();
      if (*piVar17 == 0x28) {
        if (*(char *)((long)param_8 + 0x16) != '\0') goto LAB_00108467;
      }
      else if (*piVar17 == 2) goto LAB_00108460;
      uVar16 = FUN_00114440(4,local_338);
      uVar18 = dcgettext(0,"cannot stat %s",5);
      uVar42 = 0;
      error(0,*piVar17,uVar18,uVar16);
      puVar35 = auStack_3e8;
      goto LAB_00107cf3;
    }
  }
  if (param_8[2] == 2) goto LAB_00108af0;
  if ((local_288.st_ino == local_1f8.st_ino) && (local_288.st_dev == local_1f8.st_dev)) {
    uVar43 = (ulong)*(byte *)((long)param_8 + 0x17);
    if (*(byte *)((long)param_8 + 0x17) == 0) {
      if (param_8[1] == 2) {
        bVar44 = true;
        uVar31 = 1;
        goto LAB_00107c23;
      }
      psVar29 = &local_168;
      iVar10 = fstatat(local_330,(char *)local_340,psVar29,0x100);
      if (iVar10 == 0) {
        local_350 = &local_d8;
        iVar10 = lstat((char *)param_1,local_350);
        if (iVar10 == 0) {
          bVar44 = local_d8.st_ino == local_168.st_ino && local_d8.st_dev == local_168.st_dev;
          psVar41 = local_350;
          if ((((local_d8.st_mode & 0xf000) != 0xa000) || ((local_168.st_mode & 0xf000) != 0xa000))
             || (*(char *)((long)param_8 + 0x15) == '\0')) goto LAB_00107c47;
        }
      }
    }
    goto LAB_00108af3;
  }
  if (param_8[1] != 2) goto LAB_00108af0;
  bVar44 = false;
  uVar31 = 0;
LAB_00107c23:
  psVar29 = &local_1f8;
  psVar41 = &local_288;
  if (((local_288.st_mode & 0xf000) == 0xa000) && ((local_1f8.st_mode & 0xf000) == 0xa000)) {
    local_350 = (stat *)CONCAT44(local_350._4_4_,uVar31);
    uVar12 = FUN_00114e20(0xffffff9c,param_1,local_330);
    uVar43 = (ulong)uVar12;
    if ((char)uVar12 == '\0') {
      if ((*param_8 != 0) || ((uint)local_350 == 0)) goto LAB_00108af3;
      uVar43 = 1;
      bVar45 = (byte)param_8[6] ^ 1;
LAB_0010a173:
      if (bVar45 != 0) goto LAB_00108af3;
    }
  }
  else {
LAB_00107c47:
    if (*param_8 != 0) {
      if (bVar44) {
        uVar43 = 0;
        bVar45 = FUN_00114e20(0xffffff9c,param_1,local_330);
        bVar45 = bVar45 ^ 1;
        goto LAB_0010a173;
      }
      if ((((char)param_8[6] != '\0') || (param_8[1] == 2)) ||
         (((psVar41->st_mode & 0xf000) != 0xa000 || ((psVar29->st_mode & 0xf000) == 0xa000))))
      goto LAB_00108af0;
      goto LAB_00107c99;
    }
    cVar7 = (char)param_8[6];
    if ((cVar7 == '\0') && (*(char *)((long)param_8 + 0x15) == '\0')) {
      if ((psVar41->st_mode & 0xf000) != 0xa000) {
LAB_00109bb4:
        if ((psVar29->st_mode & 0xf000) != 0xa000) {
LAB_00109e99:
          if ((psVar41->st_ino != psVar29->st_ino) || (psVar41->st_dev != psVar29->st_dev))
          goto LAB_00108af0;
          uVar43 = (ulong)*(byte *)((long)param_8 + 0x17);
          if (*(byte *)((long)param_8 + 0x17) != 0) goto LAB_00108af3;
        }
LAB_00109bc9:
        cVar7 = (char)param_8[6];
        goto LAB_00109bce;
      }
    }
    else {
      if ((psVar29->st_mode & 0xf000) == 0xa000) goto LAB_00108af0;
      if ((bVar44) && (1 < psVar29->st_nlink)) {
        local_350 = psVar41;
        uVar12 = FUN_00114e20(0xffffff9c,param_1,local_330);
        uVar43 = (ulong)uVar12;
        if ((char)uVar12 != '\0') {
          psVar41 = local_350;
          if ((local_350->st_mode & 0xf000) != 0xa000) goto LAB_00109bb4;
          goto LAB_00109bc9;
        }
        bVar45 = (byte)param_8[6] ^ 1;
        goto LAB_0010a173;
      }
      if ((psVar41->st_mode & 0xf000) != 0xa000) goto LAB_00109e99;
LAB_00109bce:
      if ((((cVar7 != '\0') && ((local_288.st_mode & 0xf000) == 0xa000)) && (1 < psVar29->st_nlink))
         && (local_350 = psVar41, pcVar37 = canonicalize_file_name((char *)param_1),
            psVar41 = local_350, pcVar37 != (char *)0x0)) {
        bVar45 = FUN_00114e20(0xffffff9c,pcVar37,local_330);
        uVar43 = 0;
        bVar45 = bVar45 ^ 1;
        free(pcVar37);
        goto LAB_0010a173;
      }
    }
    if ((*(char *)((long)param_8 + 0x3a) != '\0') && ((psVar29->st_mode & 0xf000) == 0xa000)) {
LAB_00108af0:
      uVar43 = 0;
      goto LAB_00108af3;
    }
    if (param_8[1] == 2) {
      if ((psVar41->st_mode & 0xf000) == 0xa000) {
        iVar10 = stat((char *)param_1,&local_d8);
        if (iVar10 != 0) goto LAB_00108af0;
      }
      else {
        local_d8.st_dev = (__dev_t)psVar41->st_dev;
        local_d8.st_ino = psVar41->st_ino;
        local_d8.st_nlink = psVar41->st_nlink;
        local_d8.st_mode = psVar41->st_mode;
        local_d8.st_uid = psVar41->st_uid;
        local_d8.st_gid = psVar41->st_gid;
        local_d8.__pad0 = psVar41->__pad0;
        local_d8.st_rdev = psVar41->st_rdev;
        local_d8.st_size = (__off_t)psVar41->st_size;
        local_d8.st_blksize = psVar41->st_blksize;
        local_d8.st_blocks = psVar41->st_blocks;
        local_d8.st_atim.tv_sec = (psVar41->st_atim).tv_sec;
        local_d8.st_atim.tv_nsec = (psVar41->st_atim).tv_nsec;
        local_d8.st_mtim.tv_sec = (psVar41->st_mtim).tv_sec;
        local_d8.st_mtim.tv_nsec = (psVar41->st_mtim).tv_nsec;
        local_d8.st_ctim.tv_sec = (psVar41->st_ctim).tv_sec;
        local_d8.st_ctim.tv_nsec = (psVar41->st_ctim).tv_nsec;
        local_d8.__unused[0] = psVar41->__unused[0];
        local_d8.__unused[1] = psVar41->__unused[1];
        local_d8.__unused[2] = psVar41->__unused[2];
      }
      if ((psVar29->st_mode & 0xf000) == 0xa000) {
        iVar10 = fstatat(local_330,(char *)local_340,&local_168,0);
        if (iVar10 != 0) goto LAB_00108af0;
      }
      else {
        local_168.st_dev = (__dev_t)psVar29->st_dev;
        local_168.st_ino = psVar29->st_ino;
        local_168.st_nlink = psVar29->st_nlink;
        local_168.st_mode = psVar29->st_mode;
        local_168.st_uid = psVar29->st_uid;
        local_168.st_gid = psVar29->st_gid;
        local_168.__pad0 = psVar29->__pad0;
        local_168.st_rdev = psVar29->st_rdev;
        local_168.st_size = psVar29->st_size;
        local_168.st_blksize = (__blksize_t)psVar29->st_blksize;
        local_168.st_blocks = psVar29->st_blocks;
        local_168.st_atim.tv_sec = (psVar29->st_atim).tv_sec;
        local_168.st_atim.tv_nsec = (psVar29->st_atim).tv_nsec;
        local_168.st_mtim.tv_sec = (psVar29->st_mtim).tv_sec;
        local_168.st_mtim.tv_nsec = (psVar29->st_mtim).tv_nsec;
        local_168.st_ctim.tv_sec = (psVar29->st_ctim).tv_sec;
        local_168.st_ctim.tv_nsec = (psVar29->st_ctim).tv_nsec;
        local_168.__unused[0] = psVar29->__unused[0];
        local_168.__unused[1] = psVar29->__unused[1];
        local_168.__unused[2] = psVar29->__unused[2];
      }
      if ((local_d8.st_ino != local_168.st_ino) || (local_d8.st_dev != local_168.st_dev))
      goto LAB_00108af0;
      if (*(char *)((long)param_8 + 0x17) != '\0') {
        uVar43 = (ulong)((psVar29->st_mode & 0xf000) != 0xa000);
        goto LAB_00108af3;
      }
    }
  }
LAB_00107c99:
  uVar16 = FUN_00114310(1,4,local_338);
  uVar18 = FUN_00114310(0,4,param_1);
  pcVar37 = "%s and %s are the same file";
LAB_00107cd0:
  *(undefined8 *)(puVar33 + -8) = 0x107cd7;
  uVar19 = dcgettext(0,pcVar37,5);
  *(undefined8 *)(puVar33 + -8) = 0x107ceb;
  error(0,0,uVar19,uVar18,uVar16);
  puVar35 = puVar33;
  goto LAB_00107cf0;
LAB_0010866c:
  iVar10 = local_330;
  psVar15 = local_340;
  if (local_350 != (stat *)0x0) {
    pcVar37 = (char *)(((long)local_340 - (long)local_338) + (long)local_350);
    *(undefined8 *)(puVar35 + -8) = 0x10869d;
    iVar10 = renameat(iVar10,pcVar37,iVar10,(char *)psVar15);
    psVar15 = local_338;
    if (iVar10 == 0) {
      if ((char)param_8[0xf] != '\0') {
        *(undefined8 *)(puVar35 + -8) = 0x1086c6;
        uVar16 = FUN_00114310(1,4,psVar15);
        psVar15 = local_350;
        *(undefined8 *)(puVar35 + -8) = 0x1086dc;
        uVar18 = FUN_00114310(0,4,psVar15);
        *(undefined8 *)(puVar35 + -8) = 0x1086f2;
        uVar19 = dcgettext(0,"%s -> %s (unbackup)\n",5);
        *(undefined8 *)(puVar35 + -8) = 0x108707;
        __printf_chk(1,uVar19,uVar18,uVar16);
      }
    }
    else {
      *(undefined8 *)(puVar35 + -8) = 0x1092f9;
      uVar16 = FUN_00114440(4,psVar15);
      pcVar37 = "cannot un-backup %s";
LAB_00108541:
      *(undefined8 *)(puVar35 + -8) = 0x108548;
      uVar18 = dcgettext(0,pcVar37,5);
      *(undefined8 *)(puVar35 + -8) = 0x108550;
      piVar17 = __errno_location();
      iVar10 = *piVar17;
      *(undefined8 *)(puVar35 + -8) = 0x108561;
      error(0,iVar10,uVar18,uVar16);
    }
  }
  goto LAB_00107cf0;
  while( true ) {
    local_360 = (undefined8 *)CONCAT71(local_360._1_7_,(byte)local_360 | local_312[1]);
    *(undefined8 *)(puVar35 + -0x28) = 0x1089bd;
    sVar22 = strlen(pcVar37);
    pcVar37 = pcVar37 + sVar22 + 1;
    if (*pcVar37 == '\0') break;
LAB_001089cb:
    psVar29 = local_350;
    *(undefined8 *)(puVar35 + -0x28) = 0x1089dc;
    pvVar23 = (void *)FUN_0010ea10(psVar29,pcVar37,0);
    psVar29 = local_338;
    *(undefined8 *)(puVar35 + -0x28) = 0x1089f0;
    __ptr = (void *)FUN_0010ea10(psVar29,pcVar37,0);
    iVar10 = local_330;
    uVar42 = (ulong)local_3a0 & 0xffffffff;
    bVar45 = *(byte *)local_358;
    *(stat ***)(puVar35 + -0x28) = local_390;
    *(stat ***)(puVar35 + -0x30) = local_378;
    local_312[1] = bVar45;
    *(stat **)(puVar35 + -0x38) = local_380;
    uVar43 = local_368;
    *(undefined8 *)(puVar35 + -0x40) = 0;
    *(undefined8 **)(puVar35 + -0x48) = local_398;
    *(undefined8 **)(puVar35 + -0x50) = local_3a8;
    *(undefined8 *)(puVar35 + -0x58) = 0x108a4d;
    bVar8 = copy_internal(pvVar23,__ptr,iVar10,(long)__ptr + uVar43,uVar42);
    bVar45 = local_312[0];
    *(byte *)&psVar15->st_dev = (byte)psVar15->st_dev | local_312[0];
    local_348 = (undefined1 *)CONCAT71(local_348._1_7_,(byte)local_348 & bVar8);
    *(undefined8 *)(puVar35 + -0x28) = 0x108a6a;
    free(__ptr);
    *(undefined8 *)(puVar35 + -0x28) = 0x108a72;
    free(pvVar23);
    if (bVar45 != 0) break;
  }
  uVar42 = (ulong)local_348 & 0xff;
  param_8 = local_3c0;
  param_1 = local_350;
LAB_00108a91:
  pcVar37 = local_3b0;
  *(undefined8 *)(puVar35 + -0x28) = 0x108a9d;
  free(pcVar37);
  *(byte *)local_358 = (byte)local_360;
LAB_00108aad:
  if ((char)local_328 == '\0') {
    uVar12 = 0;
    local_368 = CONCAT71(local_368._1_7_,local_3b8._0_1_);
    puVar35 = puVar35 + -0x20;
  }
  else {
    cVar7 = '\0';
    uVar12 = 0;
    bVar45 = 0;
    local_368 = CONCAT71(local_368._1_7_,local_3b8._0_1_);
LAB_00107f59:
    iVar10 = local_330;
    psVar15 = local_340;
    if (*(long *)(param_8 + 0x12) != 0) {
      local_348 = (undefined1 *)CONCAT71(local_348._1_7_,cVar7);
      local_328 = &local_d8;
      *(undefined8 *)(puVar34 + -8) = 0x107f8c;
      iVar10 = fstatat(iVar10,(char *)psVar15,&local_d8,0x100);
      psVar29 = local_328;
      psVar15 = local_340;
      cVar7 = (byte)local_348;
      if (iVar10 == 0) {
        uVar16 = *(undefined8 *)(param_8 + 0x12);
        local_328 = (stat *)CONCAT71(local_328._1_7_,(byte)local_348);
        *(undefined8 *)(puVar34 + -8) = 0x107fb6;
        FUN_0010e790(uVar16,psVar15,psVar29);
        cVar7 = (char)local_328;
      }
    }
LAB_00107fc0:
    puVar35 = puVar34;
    if (((*(char *)((long)param_8 + 0x17) != '\0') && (bVar45 != 0)) || (cVar7 != '\0'))
    goto LAB_00107cf3;
  }
  iVar10 = local_330;
  psVar15 = local_340;
  if (*(char *)((long)param_8 + 0x1f) != '\0') {
    local_d8.st_dev = local_288.st_atim.tv_sec;
    local_d8.st_ino = local_288.st_atim.tv_nsec;
    local_d8.st_nlink = local_288.st_mtim.tv_sec;
    local_d8.st_mode = (undefined4)local_288.st_mtim.tv_nsec;
    local_d8.st_uid = local_288.st_mtim.tv_nsec._4_4_;
    *(undefined8 *)(puVar35 + -8) = 0x10803b;
    iVar10 = utimensat(iVar10,(char *)psVar15,(timespec *)&local_d8,(uVar12 & 0xff) << 8);
    psVar15 = local_338;
    if (iVar10 != 0) {
      *(undefined8 *)(puVar35 + -8) = 0x108050;
      local_328 = (stat *)FUN_00114440(4,psVar15);
      *(undefined8 *)(puVar35 + -8) = 0x10806a;
      uVar16 = dcgettext(0,"preserving times for %s",5);
      *(undefined8 *)(puVar35 + -8) = 0x108072;
      piVar17 = __errno_location();
      psVar15 = local_328;
      iVar10 = *piVar17;
      *(undefined8 *)(puVar35 + -8) = 0x108087;
      error(0,iVar10,uVar16,psVar15);
      if (*(char *)((long)param_8 + 0x32) != '\0') goto LAB_00107cf0;
    }
  }
  psVar15 = local_338;
  if ((char)uVar12 == '\0') {
    if (*(char *)((long)param_8 + 0x1d) != '\0') {
      if ((((byte)local_320 != '\0') || (local_1f8.st_uid != local_288.st_uid)) ||
         (local_1f8.st_gid != local_288.st_gid)) {
        *(stat **)(puVar35 + -8) = &local_1f8;
        *(ulong *)(puVar35 + -0x10) = (ulong)local_320 & 0xff;
        psVar29 = local_340;
        *(ulong *)(puVar35 + -0x18) = (ulong)local_288.st_gid;
        iVar10 = local_330;
        *(ulong *)(puVar35 + -0x20) = (ulong)local_288.st_uid;
        *(undefined8 *)(puVar35 + -0x28) = 0x108104;
        iVar10 = FUN_00107080(param_8,psVar15,iVar10,psVar29,0xffffffff,local_288.st_mode);
        if (iVar10 == -1) goto LAB_00107cf0;
        if (iVar10 == 0) {
          local_32c = local_32c & 0xfffff1ff;
        }
      }
    }
    psVar15 = local_338;
    if (*(char *)((long)param_8 + 0x35) != '\0') {
      *(undefined8 *)(puVar35 + -8) = 0x10a094;
      cVar7 = FUN_00106220(param_1,0xffffffff,psVar15,0xffffffff,param_8);
      if ((cVar7 == '\0') && (*(char *)((long)param_8 + 0x36) != '\0')) goto LAB_00107cf0;
    }
    uVar12 = local_32c;
    psVar15 = local_338;
    if ((*(ulong *)(param_8 + 6) & 0xff0000000000ff) == 0) {
      if (*(char *)((long)param_8 + 0x39) != '\0') {
        uVar12 = param_8[4];
LAB_00109ede:
        psVar15 = local_338;
        *(undefined8 *)(puVar35 + -8) = 0x109eef;
        iVar10 = FUN_0010c8e0(psVar15,0xffffffff,uVar12);
        if (iVar10 == 0) goto LAB_00107cf3;
        goto LAB_00107cf0;
      }
      if (((byte)local_320 & (byte)param_8[8]) != 0) {
        uVar12 = 0x1ff;
        if ((local_32c & 0x7000) != 0x4000) {
          uVar12 = 0x1b6;
        }
        uVar13 = DAT_00123010;
        if (DAT_00123010 == 0xffffffff) {
          *(undefined8 *)(puVar35 + -8) = 0x10ad2a;
          uVar13 = umask(0);
          *(undefined8 *)(puVar35 + -8) = 0x10ad39;
          DAT_00123010 = uVar13;
          umask(uVar13);
        }
        uVar12 = ~uVar13 & uVar12;
        goto LAB_00109ede;
      }
      if ((uint)local_388 == 0) {
LAB_00108169:
        if ((char)local_368 == '\0') goto LAB_00107cf3;
LAB_00108176:
        iVar10 = local_330;
        psVar15 = local_340;
        uVar12 = local_3c4 | (uint)local_388;
        *(undefined8 *)(puVar35 + -8) = 0x108199;
        iVar10 = fchmodat(iVar10,(char *)psVar15,uVar12,0x100);
        psVar15 = local_338;
        if (iVar10 == 0) goto LAB_00107cf3;
        *(undefined8 *)(puVar35 + -8) = 0x1081b2;
        uVar16 = FUN_00114440(4,psVar15);
        *(undefined8 *)(puVar35 + -8) = 0x1081c8;
        uVar18 = dcgettext(0,"preserving permissions for %s",5);
        *(undefined8 *)(puVar35 + -8) = 0x1081d0;
        piVar17 = __errno_location();
        iVar10 = *piVar17;
        *(undefined8 *)(puVar35 + -8) = 0x1081e1;
        error(0,iVar10,uVar18,uVar16);
        goto LAB_001081e1;
      }
      uVar12 = DAT_00123010;
      if (DAT_00123010 == 0xffffffff) {
        *(undefined8 *)(puVar35 + -8) = 0x10ad45;
        uVar12 = umask(0);
        *(undefined8 *)(puVar35 + -8) = 0x10ad54;
        DAT_00123010 = uVar12;
        umask(uVar12);
      }
      iVar10 = local_330;
      psVar15 = local_340;
      uVar12 = (uint)local_388 & ~uVar12;
      local_388 = (char *)CONCAT44(local_388._4_4_,uVar12);
      if (uVar12 == 0) goto LAB_00108169;
      if ((char)local_368 == '\x01') goto LAB_00108176;
      if ((byte)local_320 == '\0') {
LAB_0010a3a9:
        local_3c4 = local_1f8.st_mode;
        if (((uint)local_388 & ~local_1f8.st_mode) == 0) goto LAB_00107cf3;
        goto LAB_00108176;
      }
      *(undefined8 *)(puVar35 + -8) = 0x108513;
      iVar10 = fstatat(iVar10,(char *)psVar15,&local_1f8,0x100);
      psVar15 = local_338;
      if (iVar10 == 0) goto LAB_0010a3a9;
LAB_00108528:
      *(undefined8 *)(puVar35 + -8) = 0x108532;
      uVar16 = FUN_00114440(4,psVar15);
      pcVar37 = "cannot stat %s";
      goto LAB_00108541;
    }
    *(undefined8 *)(puVar35 + -8) = 0x109b70;
    iVar10 = FUN_0010c830(param_1,0xffffffff,psVar15,0xffffffff,uVar12);
    if (iVar10 == 0) goto LAB_00107cf3;
LAB_001081e1:
    cVar7 = *(char *)((long)param_8 + 0x32);
  }
  else {
    if (*(char *)((long)param_8 + 0x35) == '\0') goto LAB_00107cf3;
    *(undefined8 *)(puVar35 + -8) = 0x108ef7;
    cVar7 = FUN_00106220(param_1,0xffffffff,psVar15,0xffffffff,param_8);
    if (cVar7 != '\0') goto LAB_00107cf3;
    cVar7 = *(char *)((long)param_8 + 0x36);
  }
  if (cVar7 == '\0') goto LAB_00107cf3;
LAB_00107cf0:
  uVar42 = 0;
LAB_00107cf3:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar35 + -8) = 0x10b028;
    __stack_chk_fail();
  }
  return uVar42 & 0xffffffff;
}




// Function: copy @ 0xba80

void copy(undefined8 param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,
                 undefined8 param_5,uint *param_6,undefined8 param_7,undefined8 param_8)

{
  long in_FS_OFFSET;
  undefined1 local_11;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (3 < *param_6) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options");
  }
  if (2 < param_6[3] - 1) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options");
  }
  if (2 < param_6[0x11]) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options");
  }
  if ((*(char *)((long)param_6 + 0x17) != '\0') && (*(char *)((long)param_6 + 0x3a) != '\0')) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options");
  }
  if ((param_6[3] != 2) && (param_6[0x11] == 2)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
                  "src/copy.c",0xc13,"valid_options");
  }
  local_11 = 0;
  DAT_00123568 = param_2;
  DAT_00123570 = param_1;
  FUN_001076f0(param_1,param_2,param_3,param_4,param_5,0,0,param_6,1,&local_11,param_7,param_8);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: src_to_dest_hash @ 0xbc10

ulong src_to_dest_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: src_to_dest_compare @ 0xbc20

undefined8 src_to_dest_compare(long *param_1,long *param_2)

{
  if (*param_1 != *param_2) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_2[1] >> 8),param_1[1] == param_2[1]);
}




// Function: forget_created @ 0xbc60

void forget_created(undefined8 param_1,undefined8 param_2)

{
  void *__ptr;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = 0;
  local_28 = param_1;
  local_20 = param_2;
  __ptr = (void *)FUN_00112270(DAT_00123578,&local_28);
  if (__ptr != (void *)0x0) {
    free(*(void **)((long)__ptr + 0x10));
    free(__ptr);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: src_to_dest_lookup @ 0xbcd0

undefined8 src_to_dest_lookup(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = param_1;
  local_20 = param_2;
  lVar1 = FUN_00111920(DAT_00123578,&local_28);
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




// Function: remember_copied @ 0xbd30

undefined8 remember_copied(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *__ptr;
  undefined8 uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  
  __ptr = (undefined8 *)FUN_00116c50(0x18);
  uVar1 = FUN_00117130(param_1);
  uVar3 = DAT_00123578;
  *__ptr = param_2;
  __ptr[2] = uVar1;
  __ptr[1] = param_3;
  puVar2 = (undefined8 *)FUN_00112210(uVar3,__ptr);
  if (puVar2 != (undefined8 *)0x0) {
    uVar3 = 0;
    if (__ptr != puVar2) {
      free((void *)__ptr[2]);
      free(__ptr);
      uVar3 = puVar2[2];
    }
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117150();
}




// Function: hash_init @ 0xbdb0

void hash_init(void)

{
  DAT_00123578 = FUN_00111bb0(0x67,0,FUN_0010bc10,FUN_0010bc20,FUN_0010bc40);
  if (DAT_00123578 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117150();
}




// Function: try_link @ 0xbdf0

void try_link(char *param_1,int *param_2)

{
  linkat(*param_2,*(char **)(param_2 + 2),param_2[4],param_1,param_2[5]);
  return;
}




// Function: samedir_template @ 0xbe10

void * samedir_template(void *param_1,void *param_2)

{
  ulong __size;
  long lVar1;
  undefined8 *puVar2;
  
  lVar1 = FUN_0010d970();
  __size = (lVar1 - (long)param_1) + 9;
  if ((0x100 < __size) && (param_2 = malloc(__size), param_2 == (void *)0x0)) {
    return (void *)0x0;
  }
  puVar2 = mempcpy(param_2,param_1,lVar1 - (long)param_1);
  *puVar2 = DAT_0011c458;
  *(undefined1 *)(puVar2 + 1) = DAT_0011c460;
  return param_2;
}




// Function: try_symlink @ 0xbe70

void try_symlink(char *param_1,undefined8 *param_2)

{
  symlinkat((char *)*param_2,*(int *)(param_2 + 1),param_1);
  return;
}




// Function: force_linkat @ 0xbe90

int force_linkat(int param_1,char *param_2,int param_3,char *param_4,int param_5,char param_6,
                int param_7)

{
  int iVar1;
  char *__old;
  int *piVar2;
  long in_FS_OFFSET;
  int local_168 [2];
  char *local_160;
  int local_158;
  int local_154;
  char local_148 [264];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_7 < 0) {
    iVar1 = linkat(param_1,param_2,param_3,param_4,param_5);
    param_7 = 0;
    if (iVar1 == 0) goto LAB_0010bf63;
    piVar2 = __errno_location();
    param_7 = *piVar2;
  }
  if ((param_6 == '\x01') && (param_7 == 0x11)) {
    __old = (char *)FUN_0010be10(param_4,local_148);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      param_7 = *piVar2;
    }
    else {
      local_168[0] = param_1;
      local_160 = param_2;
      local_158 = param_3;
      local_154 = param_5;
      iVar1 = FUN_001157b0(__old,0,local_168,FUN_0010bdf0,6);
      if (iVar1 == 0) {
        iVar1 = renameat(param_3,__old,param_3,param_4);
        param_7 = -1;
        if (iVar1 != 0) {
          piVar2 = __errno_location();
          param_7 = *piVar2;
        }
        unlinkat(param_3,__old,0);
      }
      else {
        piVar2 = __errno_location();
        param_7 = *piVar2;
      }
      if (__old != local_148) {
        free(__old);
      }
    }
  }
LAB_0010bf63:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_7;
}




// Function: force_symlinkat @ 0xc000

int force_symlinkat(char *param_1,int param_2,char *param_3,char param_4,int param_5)

{
  int iVar1;
  char *__old;
  int *piVar2;
  long in_FS_OFFSET;
  char *local_158;
  int local_150;
  char local_148 [264];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_5 < 0) {
    iVar1 = symlinkat(param_1,param_2,param_3);
    param_5 = 0;
    if (iVar1 == 0) goto LAB_0010c0a3;
    piVar2 = __errno_location();
    param_5 = *piVar2;
  }
  if ((param_4 == '\x01') && (param_5 == 0x11)) {
    __old = (char *)FUN_0010be10(param_3,local_148);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      param_5 = *piVar2;
    }
    else {
      local_158 = param_1;
      local_150 = param_2;
      iVar1 = FUN_001157b0(__old,0,&local_158,FUN_0010be70,6);
      if (iVar1 == 0) {
        iVar1 = renameat(param_2,__old,param_2,param_3);
        param_5 = -1;
        if (iVar1 != 0) {
          piVar2 = __errno_location();
          param_5 = *piVar2;
          unlinkat(param_2,__old,0);
        }
      }
      else {
        piVar2 = __errno_location();
        param_5 = *piVar2;
      }
      if (__old != local_148) {
        free(__old);
      }
    }
  }
LAB_0010c0a3:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_5;
}




// Function: restorecon_private @ 0xc140

int restorecon_private(long param_1,char *param_2)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined8 local_e8;
  long local_e0;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e8 = 0;
  local_e0 = 0;
  if (param_1 == 0) {
    iVar3 = getfscreatecon(&local_e0);
    if (iVar3 < 0) {
      iVar3 = -1;
    }
    else {
      piVar4 = __errno_location();
      if (local_e0 == 0) {
        *piVar4 = 0x3d;
        iVar3 = -1;
      }
      else {
        iVar3 = lsetfilecon(param_2,local_e0);
        iVar1 = *piVar4;
        freecon(local_e0);
        *piVar4 = iVar1;
      }
    }
    goto LAB_0010c305;
  }
  iVar1 = open(param_2,0x20000);
  piVar4 = __errno_location();
  iVar3 = iVar1;
  if (iVar1 == -1) {
    iVar2 = *piVar4;
    if (iVar2 == 0x28) {
      iVar2 = lstat(param_2,&local_d8);
      if (-1 < iVar2) goto LAB_0010c1bd;
      iVar2 = *piVar4;
      lVar6 = 0;
      lVar5 = 0;
    }
    else {
      lVar6 = 0;
      lVar5 = 0;
    }
    goto LAB_0010c28e;
  }
  iVar2 = fstat(iVar1,&local_d8);
  if (iVar2 < 0) {
    lVar6 = 0;
    lVar5 = 0;
    iVar3 = -1;
    goto LAB_0010c3ab;
  }
LAB_0010c1bd:
  iVar2 = selabel_lookup(param_1,&local_e8,param_2,local_d8.st_mode);
  if (iVar2 < 0) {
    iVar2 = *piVar4;
    lVar6 = 0;
    lVar5 = 0;
    if (iVar2 == 2) {
      *piVar4 = 0x3d;
      iVar2 = 0x3d;
    }
LAB_0010c244:
    iVar3 = -1;
    if (iVar1 != -1) {
LAB_0010c24f:
      close(iVar1);
    }
  }
  else {
    lVar5 = context_new(local_e8);
    if (lVar5 == 0) {
      iVar2 = *piVar4;
      lVar6 = 0;
      goto LAB_0010c244;
    }
    if (iVar1 != -1) {
      iVar3 = FUN_00115610(iVar1);
      if (iVar3 < 0) {
        lVar6 = 0;
        iVar3 = -1;
LAB_0010c3ab:
        iVar2 = *piVar4;
      }
      else {
        lVar6 = context_new(local_e0);
        if (lVar6 != 0) {
LAB_0010c221:
          lVar7 = context_type_get(lVar5);
          if (((lVar7 == 0) || (iVar3 = context_type_set(lVar6,lVar7), iVar3 != 0)) ||
             (lVar7 = context_str(lVar6), lVar7 == 0)) {
            iVar2 = *piVar4;
            goto LAB_0010c244;
          }
          if (iVar1 == -1) {
            iVar3 = lsetfilecon(param_2,lVar7);
            iVar2 = *piVar4;
            goto LAB_0010c28e;
          }
          iVar3 = fsetfilecon(iVar1);
          goto LAB_0010c3ab;
        }
        iVar2 = *piVar4;
        iVar3 = -1;
      }
      goto LAB_0010c24f;
    }
    iVar2 = FUN_00115590(param_2,&local_e0);
    if (iVar2 < 0) {
      iVar2 = *piVar4;
      lVar6 = 0;
    }
    else {
      lVar6 = context_new(local_e0);
      if (lVar6 != 0) goto LAB_0010c221;
      iVar2 = *piVar4;
      iVar3 = -1;
    }
  }
LAB_0010c28e:
  context_free(lVar5);
  context_free(lVar6);
  freecon(local_e8);
  freecon(local_e0);
  *piVar4 = iVar2;
LAB_0010c305:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: defaultcon @ 0xc450

undefined4 defaultcon(undefined8 param_1,char *param_2,undefined4 param_3)

{
  short sVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  int *piVar5;
  void *__ptr;
  long lVar6;
  long lVar7;
  long lVar8;
  char *__ptr_00;
  long in_FS_OFFSET;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = 0;
  local_58 = 0;
  piVar5 = __errno_location();
  if (*param_2 == '/') {
    __ptr_00 = (char *)0x0;
  }
  else {
    param_2 = (char *)FUN_0010da80(param_2,2);
    __ptr_00 = param_2;
    if (param_2 == (char *)0x0) {
      iVar2 = *piVar5;
      __ptr_00 = (char *)0x0;
      lVar7 = 0;
      uVar4 = 0xffffffff;
      lVar6 = 0;
      goto LAB_0010c5df;
    }
  }
  iVar2 = selabel_lookup(param_1,&local_60,param_2,param_3);
  if (iVar2 < 0) {
    iVar2 = *piVar5;
    if (iVar2 == 2) {
      *piVar5 = 0x3d;
      iVar2 = 0x3d;
      lVar7 = 0;
      lVar6 = 0;
      uVar4 = 0xffffffff;
      goto LAB_0010c5df;
    }
  }
  else {
    local_50 = 0;
    local_48 = 0;
    __ptr = (void *)FUN_0010e480(param_2);
    iVar2 = getcon(&local_50);
    if (((iVar2 < 0) || (iVar2 = FUN_00115510(__ptr,&local_48), iVar2 < 0)) ||
       (sVar1 = mode_to_security_class(param_3), sVar1 == 0)) {
      iVar2 = *piVar5;
      lVar7 = 0;
      lVar6 = 0;
      free(__ptr);
      uVar4 = 0xffffffff;
      freecon(local_50);
      freecon(local_48);
      *piVar5 = iVar2;
      goto LAB_0010c5df;
    }
    iVar3 = security_compute_create(local_50,local_48,sVar1,&local_58);
    iVar2 = *piVar5;
    free(__ptr);
    freecon(local_50);
    freecon(local_48);
    *piVar5 = iVar2;
    if (-1 < iVar3) {
      lVar6 = context_new(local_60);
      if (lVar6 == 0) {
        iVar2 = *piVar5;
        lVar7 = 0;
        uVar4 = 0xffffffff;
      }
      else {
        lVar7 = context_new(local_58);
        if (((lVar7 == 0) || (lVar8 = context_type_get(lVar6), lVar8 == 0)) ||
           ((iVar2 = context_type_set(lVar7,lVar8), iVar2 != 0 ||
            (lVar8 = context_str(lVar7), lVar8 == 0)))) {
          iVar2 = *piVar5;
          uVar4 = 0xffffffff;
        }
        else {
          uVar4 = setfscreatecon(lVar8);
          iVar2 = *piVar5;
        }
      }
      goto LAB_0010c5df;
    }
  }
  lVar7 = 0;
  lVar6 = 0;
  uVar4 = 0xffffffff;
LAB_0010c5df:
  context_free(lVar6);
  context_free(lVar7);
  freecon(local_60);
  freecon(local_58);
  free(__ptr_00);
  *piVar5 = iVar2;
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: restorecon @ 0xc6f0

ulong restorecon(undefined8 param_1,char *param_2,char param_3)

{
  int iVar1;
  int *piVar2;
  long lVar3;
  long lVar4;
  char *__ptr;
  undefined7 uVar6;
  ulong uVar5;
  int iVar7;
  long in_FS_OFFSET;
  char *local_48;
  undefined8 local_40;
  long local_30;
  
  __ptr = (char *)0x0;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar6 = (undefined7)((ulong)param_1 >> 8);
  if (*param_2 == '/') {
    piVar2 = __errno_location();
  }
  else {
    __ptr = (char *)FUN_0010da80(param_2,2);
    if (__ptr == (char *)0x0) {
      uVar5 = 0;
      goto LAB_0010c7b0;
    }
    piVar2 = __errno_location();
    param_2 = __ptr;
  }
  if (param_3 != '\0') {
    local_40 = 0;
    iVar7 = 0;
    local_48 = param_2;
    lVar3 = FUN_00117190(&local_48,0x10,0);
    do {
      lVar4 = FUN_00110840(lVar3);
      while( true ) {
        if (lVar4 == 0) {
          if (*piVar2 != 0) {
            iVar7 = *piVar2;
          }
          iVar1 = FUN_00110650(lVar3);
          if (iVar1 != 0) {
            iVar7 = *piVar2;
          }
          free(__ptr);
          uVar5 = CONCAT71(uVar6,iVar7 == 0);
          goto LAB_0010c7b0;
        }
        iVar1 = FUN_0010c140(param_1,*(undefined8 *)(lVar3 + 0x20));
        if (-1 < iVar1) break;
        iVar7 = *piVar2;
        lVar4 = FUN_00110840(lVar3);
      }
    } while( true );
  }
  iVar1 = FUN_0010c140(param_1,param_2);
  iVar7 = *piVar2;
  uVar5 = CONCAT71(uVar6,iVar1 != -1);
  free(__ptr);
  *piVar2 = iVar7;
LAB_0010c7b0:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5 & 0xffffffff;
}




// Function: dir_name @ 0xe480

void dir_name(void)

{
  long lVar1;
  
  lVar1 = FUN_0010e4e0();
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117150();
}




// Function: raw_hasher @ 0x11240

ulong raw_hasher(ulong param_1,ulong param_2)

{
  return (param_1 >> 3 | param_1 << 0x3d) % param_2;
}




// Function: raw_comparator @ 0x11260

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0x11270

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0011c620) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0011c634 < fVar1) && (fVar1 < DAT_0011c638)) && (_DAT_0011c63c < pfVar2[3])) &&
     (_DAT_0011c650 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0011c634;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0011c640)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0011c620;
  return 0;
}




// Function: hash_find_entry @ 0x11300

long hash_find_entry(long *param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  ulong uVar4;
  long lVar5;
  long *plVar6;
  
  uVar4 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar4) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar6 = (long *)(uVar4 * 0x10 + *param_1);
  *param_3 = plVar6;
  lVar5 = *plVar6;
  if (lVar5 == 0) {
LAB_001113af:
    lVar5 = 0;
  }
  else {
    if (lVar5 != param_2) {
      cVar3 = (*(code *)param_1[7])(param_2);
      if (cVar3 == '\0') {
        for (; (long *)plVar6[1] != (long *)0x0; plVar6 = (long *)plVar6[1]) {
          if ((*(long *)plVar6[1] == param_2) ||
             (cVar3 = (*(code *)param_1[7])(param_2), cVar3 != '\0')) {
            plVar1 = (long *)plVar6[1];
            lVar5 = *plVar1;
            if (param_4 == '\0') {
              return lVar5;
            }
            plVar6[1] = plVar1[1];
            *plVar1 = 0;
            plVar1[1] = param_1[9];
            param_1[9] = (long)plVar1;
            return lVar5;
          }
        }
        goto LAB_001113af;
      }
      lVar5 = *plVar6;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar6[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar6 = *plVar1;
        plVar6[1] = lVar2;
        *plVar1 = 0;
        plVar1[1] = param_1[9];
        param_1[9] = (long)plVar1;
        return lVar5;
      }
      *plVar6 = 0;
    }
  }
  return lVar5;
}




// Function: transfer_entries @ 0x11550

undefined8 transfer_entries(long *param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  ulong uVar4;
  long *plVar5;
  long *plVar6;
  long *plVar7;
  long lVar8;
  
  plVar6 = (long *)*param_2;
  if (plVar6 < (long *)param_2[1]) {
    do {
      while (lVar8 = *plVar6, lVar8 != 0) {
        if ((long *)plVar6[1] != (long *)0x0) {
          uVar4 = param_1[2];
          plVar7 = (long *)plVar6[1];
          do {
            lVar8 = *plVar7;
            uVar2 = (*(code *)param_1[6])(lVar8,uVar4);
            uVar4 = param_1[2];
            if (uVar4 <= uVar2) goto LAB_001116bb;
            plVar3 = (long *)(uVar2 * 0x10 + *param_1);
            plVar5 = (long *)plVar7[1];
            if (*plVar3 == 0) {
              *plVar3 = lVar8;
              lVar8 = param_1[9];
              param_1[3] = param_1[3] + 1;
              *plVar7 = 0;
              plVar7[1] = lVar8;
              param_1[9] = (long)plVar7;
            }
            else {
              plVar7[1] = plVar3[1];
              plVar3[1] = (long)plVar7;
            }
            plVar7 = plVar5;
          } while (plVar5 != (long *)0x0);
          lVar8 = *plVar6;
        }
        plVar6[1] = 0;
        if (param_3 != '\0') break;
        uVar4 = (*(code *)param_1[6])(lVar8,param_1[2]);
        if ((ulong)param_1[2] <= uVar4) {
LAB_001116bb:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        plVar7 = (long *)(uVar4 * 0x10 + *param_1);
        if (*plVar7 == 0) {
          *plVar7 = lVar8;
          param_1[3] = param_1[3] + 1;
        }
        else {
          plVar5 = (long *)param_1[9];
          if (plVar5 == (long *)0x0) {
            plVar5 = malloc(0x10);
            if (plVar5 == (long *)0x0) {
              return 0;
            }
          }
          else {
            param_1[9] = plVar5[1];
          }
          lVar1 = plVar7[1];
          *plVar5 = lVar8;
          plVar5[1] = lVar1;
          plVar7[1] = (long)plVar5;
        }
        *plVar6 = 0;
        plVar6 = plVar6 + 2;
        param_2[3] = param_2[3] + -1;
        if ((long *)param_2[1] <= plVar6) {
          return 1;
        }
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_2[1]);
  }
  return 1;
}




// Function: hash_print_statistics @ 0x117c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 *param_1,undefined8 param_2)

{
  ulong uVar1;
  ulong uVar2;
  long lVar3;
  long *plVar4;
  ulong uVar5;
  ulong uVar6;
  
  uVar6 = 0;
  uVar1 = param_1[2];
  uVar2 = param_1[3];
  for (plVar4 = (long *)*param_1; plVar4 < (long *)param_1[1]; plVar4 = plVar4 + 2) {
    while (*plVar4 == 0) {
      plVar4 = plVar4 + 2;
      if ((long *)param_1[1] <= plVar4) goto LAB_0011182d;
    }
    uVar5 = 1;
    for (lVar3 = plVar4[1]; lVar3 != 0; lVar3 = *(long *)(lVar3 + 8)) {
      uVar5 = uVar5 + 1;
    }
    if (uVar6 < uVar5) {
      uVar6 = uVar5;
    }
  }
LAB_0011182d:
  __fprintf_chk(param_2,1,"# entries:         %lu\n",param_1[4]);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar1);
  __fprintf_chk(((double)uVar2 * _DAT_0011c650) / (double)uVar1,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar2);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar6);
  return;
}




// Function: hash_lookup @ 0x11920

long hash_lookup(long *param_1,long param_2)

{
  char cVar1;
  ulong uVar2;
  long *plVar3;
  long lVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  lVar4 = *plVar3;
  if (lVar4 != 0) {
    while( true ) {
      if (lVar4 == param_2) {
        return param_2;
      }
      cVar1 = (*(code *)param_1[7])(param_2);
      if (cVar1 != '\0') break;
      plVar3 = (long *)plVar3[1];
      if (plVar3 == (long *)0x0) {
        return 0;
      }
      lVar4 = *plVar3;
    }
    return *plVar3;
  }
  return 0;
}




// Function: hash_get_next @ 0x119e0

long hash_get_next(long *param_1,long param_2)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  long *plVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  plVar4 = plVar3;
  do {
    lVar1 = *plVar4;
    plVar4 = (long *)plVar4[1];
    if (lVar1 == param_2) {
      if (plVar4 != (long *)0x0) {
        return *plVar4;
      }
      break;
    }
  } while (plVar4 != (long *)0x0);
  do {
    plVar3 = plVar3 + 2;
    if ((long *)param_1[1] <= plVar3) {
      return 0;
    }
  } while (*plVar3 == 0);
  return *plVar3;
}




// Function: hash_do_for_each @ 0x11ac0

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




// Function: hash_initialize @ 0x11bb0

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_00111240;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_00111260;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0011c620;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_00111270(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_00111410(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0x11cc0

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
        if ((long *)param_1[1] <= plVar7) goto LAB_00111d5b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_00111d5b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x11d70

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
    if (plVar3 <= plVar5) goto LAB_00111e1c;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_00111ddf;
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
LAB_00111ddf:
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
LAB_00111e1c:
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




// Function: hash_rehash @ 0x11e60

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
  __nmemb = FUN_00111410(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_00111f66;
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
      uVar2 = FUN_00111550(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_00111550(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_00111550(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_00111f66;
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
      goto LAB_00111f66;
    }
  }
  uVar2 = 0;
LAB_00111f66:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0x11fe0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  ulong uVar3;
  long *plVar4;
  ulong uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) goto LAB_001121e3;
  lVar2 = FUN_00111300(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar6 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0011202e;
  }
  uVar5 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar5 < 0) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar3) goto LAB_0011206f;
LAB_001120f6:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar3;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar3 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar3 < 0) goto LAB_001120f6;
LAB_0011206f:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar3;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar8._0_4_ < auVar9._0_4_) {
    FUN_00111270(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar9._0_4_ <= auVar8._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0011208e;
    fVar7 = auVar8._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar7 = fVar7 * *(float *)(lVar2 + 8);
    }
    if (fVar7 < _DAT_0011c644) {
      if (_DAT_0011c648 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011c648) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar1 = FUN_00111e60(param_1,uVar5);
      if (cVar1 != '\0') {
        lVar2 = FUN_00111300(param_1,param_2,&local_38,0);
        if (lVar2 != 0) {
LAB_001121e3:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        goto LAB_0011208e;
      }
    }
LAB_0011214d:
    uVar6 = 0xffffffff;
  }
  else {
LAB_0011208e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar6 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0011202e;
    }
    plVar4 = *(long **)(param_1 + 0x48);
    if (plVar4 == (long *)0x0) {
      plVar4 = malloc(0x10);
      if (plVar4 == (long *)0x0) goto LAB_0011214d;
    }
    else {
      *(long *)(param_1 + 0x48) = plVar4[1];
    }
    lVar2 = local_38[1];
    *plVar4 = param_2;
    uVar6 = 1;
    plVar4[1] = lVar2;
    local_38[1] = (long)plVar4;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
  }
LAB_0011202e:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: hash_insert @ 0x12210

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00111fe0(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0x12270

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
  lVar4 = FUN_00111300(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_001122b3;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_00112300;
LAB_001123b6:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_001123b6;
LAB_00112300:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_00111270(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0011c648 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011c648) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_00111e60(param_1,uVar5);
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
LAB_001122b3:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}



