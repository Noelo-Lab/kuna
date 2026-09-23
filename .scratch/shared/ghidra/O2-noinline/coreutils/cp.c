// Function: main @ 0x40b0

byte main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  long lVar2;
  long lVar3;
  byte bVar4;
  int iVar5;
  long lVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined8 uVar9;
  char *pcVar10;
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
  int local_8c;
  undefined1 local_84;
  undefined1 local_83;
  undefined1 local_82;
  char local_81;
  undefined1 local_7c;
  undefined1 local_7b;
  undefined2 local_7a;
  long local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  char local_65;
  char local_64;
  undefined1 local_63;
  undefined2 local_61;
  char local_5e;
  undefined1 local_5d;
  undefined1 local_5c;
  int local_54;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001127f0(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_0011a150(FUN_0010e560);
  iVar5 = is_selinux_enabled();
  DAT_0012515a = 0 < iVar5;
  FUN_001049f0(&local_98);
  local_a2 = 0;
  local_b0 = 0;
  local_c8 = 0;
  local_a1 = 0;
  local_c0 = 0;
  local_b8 = 0;
  bVar1 = false;
  lVar2 = local_c0;
  lVar3 = local_b0;
LAB_00104188:
  do {
    while (local_b0 = lVar3, local_c0 = lVar2,
          iVar5 = getopt_long(param_1,param_2,"abdfHilLnprst:uvxPRS:TZ",&PTR_s_archive_001244a0,0),
          lVar2 = local_c0, lVar3 = local_b0, iVar5 == -1) {
LAB_00104508:
      if ((local_81 == '\0') || (local_5e == '\0')) {
        if (local_90 == 2) {
          local_5d = 0;
          if (bVar1) goto LAB_00104824;
LAB_00104679:
          local_98 = 0;
          if ((local_54 != 2) || (local_8c == 2)) {
LAB_00104556:
            FUN_0010d750(local_b8);
            if (local_94 == 1) goto LAB_001046cf;
            goto LAB_0010456f;
          }
        }
        else {
          if (!bVar1) goto LAB_00104679;
          if ((local_54 != 2) || (local_8c == 2)) {
            uVar7 = dcgettext(0,"backup type",5);
            local_98 = FUN_0010db20(uVar7,local_c0);
            goto LAB_00104556;
          }
        }
        pcVar10 = "--reflink can be used only with --sparse=auto";
      }
      else {
        pcVar10 = "cannot make both hard and symbolic links";
      }
LAB_00104754:
      uVar7 = dcgettext(0,pcVar10,5);
      error(0,0,uVar7);
switchD_001041cb_caseD_49:
      FUN_001057e0(1);
LAB_00104773:
      local_70 = selabel_open(0,0,0);
      lVar3 = local_b0;
      if (local_70 == 0) {
        uVar7 = dcgettext(0,"warning: ignoring --context",5);
        piVar8 = __errno_location();
        error(0,*piVar8,uVar7);
      }
    }
    if (0x88 < iVar5) goto switchD_001041cb_caseD_49;
    if (iVar5 < 0x48) {
      if (iVar5 == -0x83) {
        FUN_00116cc0(stdout,&DAT_0011c6d9,"GNU coreutils",PTR_DAT_00125020,"Torbjorn Granlund",
                     "David MacKenzie","Jim Meyering",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar5 == -0x82) {
        FUN_001057e0(0);
        goto LAB_00104508;
      }
      goto switchD_001041cb_caseD_49;
    }
    switch(iVar5) {
    case 0x48:
      local_94 = 3;
      break;
    default:
      goto switchD_001041cb_caseD_49;
    case 0x4c:
      local_94 = 4;
      break;
    case 0x50:
      local_94 = 2;
      break;
    case 0x52:
    case 0x72:
      local_61 = CONCAT11(1,(undefined1)local_61);
      break;
    case 0x53:
      local_b8 = optarg;
      bVar1 = true;
      break;
    case 0x54:
      local_a2 = 1;
      break;
    case 0x5a:
      if (DAT_0012515a == '\0') {
        if (optarg != 0) {
          uVar7 = dcgettext(0,"warning: ignoring --context; it requires an SELinux-enabled kernel",5
                           );
          error(0,0,uVar7);
        }
      }
      else {
        lVar3 = optarg;
        if (optarg == 0) goto LAB_00104773;
      }
      break;
    case 0x61:
      local_68 = 1;
      local_94 = 2;
      local_7b = 1;
      local_7a = 0x101;
      local_66 = 1;
      if (DAT_0012515a != '\0') {
        local_65 = '\x01';
      }
      local_63 = 1;
      local_61 = 0x101;
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
      local_82 = 1;
      break;
    case 0x69:
      local_90 = 3;
      break;
    case 0x6c:
      local_81 = '\x01';
      break;
    case 0x6e:
      local_90 = 2;
      break;
    case 0x73:
      local_5e = '\x01';
      break;
    case 0x74:
      if (local_c8 != 0) {
        uVar7 = dcgettext(0,"multiple target directories specified",5);
        error(1,0,uVar7);
LAB_00104824:
        pcVar10 = "options --backup and --no-clobber are mutually exclusive";
        goto LAB_00104754;
      }
      local_c8 = optarg;
      break;
    case 0x75:
      local_5d = 1;
      break;
    case 0x76:
      local_5c = 1;
      break;
    case 0x78:
      local_7c = 1;
      break;
    case 0x80:
      local_67 = 0;
      break;
    case 0x81:
      local_a1 = 1;
      break;
    case 0x82:
      FUN_00104a80(optarg,&local_98,0);
      break;
    case 0x83:
      DAT_00125159 = 1;
      break;
    case 0x84:
      if (optarg != 0) {
        FUN_00104a80(optarg,&local_98,1);
        local_66 = 1;
        break;
      }
    case 0x70:
      local_7b = 1;
      local_7a = 0x101;
      local_66 = 1;
      break;
    case 0x85:
      if (optarg == 0) {
        local_54 = 2;
      }
      else {
        lVar6 = FUN_0010d220("--reflink",optarg,&PTR_DAT_00124860,&DAT_0011cad0,4,PTR_FUN_00125028,1
                            );
        local_54 = *(int *)(&DAT_0011cad0 + lVar6 * 4);
      }
      break;
    case 0x86:
      lVar6 = FUN_0010d220("--sparse",optarg,&PTR_s_never_00124880,&DAT_0011cae0,4,PTR_FUN_00125028,
                           1);
      local_8c = *(int *)(&DAT_0011cae0 + lVar6 * 4);
      break;
    case 0x87:
      DAT_00125158 = 1;
      break;
    case 0x88:
      goto switchD_001041cb_caseD_88;
    }
  } while( true );
LAB_0010456f:
  if (local_61._1_1_ != '\0') goto LAB_001046e5;
  while (local_b0 == 0 && local_70 == 0) {
    if ((local_64 == '\0') || (DAT_0012515a != '\0')) goto LAB_001045b6;
LAB_001046ab:
    uVar7 = dcgettext(0,"cannot preserve security context without an SELinux-enabled kernel",5);
    error(1,0,uVar7);
LAB_001046cf:
    if ((local_61._1_1_ == '\0') || (local_81 != '\0')) goto LAB_0010471e;
    local_94 = 2;
LAB_001046e5:
    local_84 = local_a1;
  }
  if (local_64 == '\0') {
    local_65 = '\0';
LAB_0010459c:
    if ((local_b0 != 0) && (iVar5 = setfscreatecon(local_b0), iVar5 < 0)) {
      uVar7 = FUN_00114e00(local_b0);
      uVar9 = dcgettext(0,"failed to set default file creation context to %s",5);
      piVar8 = __errno_location();
      error(1,*piVar8,uVar9,uVar7);
      goto LAB_00104871;
    }
LAB_001045b6:
    FUN_0010bfe0();
    bVar4 = FUN_00105ae0(param_1 - optind,param_2 + optind,local_c8,local_a2,&local_98);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return bVar4 ^ 1;
    }
LAB_00104871:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if (local_65 == '\0') {
    if (DAT_0012515a == '\0') goto LAB_001046ab;
    goto LAB_0010459c;
  }
  uVar7 = dcgettext(0,"cannot set target context and preserve it",5);
  error(1,0,uVar7);
LAB_0010471e:
  local_94 = 4;
  goto LAB_0010456f;
switchD_001041cb_caseD_88:
  local_83 = 1;
  goto LAB_00104188;
}




// Function: cp_option_init @ 0x49f0

void cp_option_init(long param_1)

{
  char *pcVar1;
  
  FUN_001083b0();
  *(undefined4 *)(param_1 + 0x44) = 1;
  *(undefined8 *)(param_1 + 8) = 0x200000004;
  *(undefined8 *)(param_1 + 0x10) = 0x100000000;
  *(undefined4 *)(param_1 + 0x1c) = 0;
  *(undefined1 *)(param_1 + 0x20) = 0;
  *(undefined4 *)(param_1 + 4) = 1;
  *(undefined2 *)(param_1 + 0x18) = 0;
  *(undefined8 *)(param_1 + 0x28) = 0;
  *(undefined8 *)(param_1 + 0x30) = 0x100;
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined2 *)(param_1 + 0x3c) = 0;
  pcVar1 = getenv("POSIXLY_CORRECT");
  *(undefined8 *)(param_1 + 0x48) = 0;
  *(undefined8 *)(param_1 + 0x50) = 0;
  *(bool *)(param_1 + 0x3e) = pcVar1 != (char *)0x0;
  return;
}




// Function: decode_preserve_arg @ 0x4a80

void decode_preserve_arg(undefined8 param_1,undefined8 param_2,char param_3)

{
  char *__s;
  char *pcVar1;
  char *pcVar2;
  long lVar3;
  
  __s = (char *)FUN_00117230();
  pcVar1 = "--no-preserve";
  if (param_3 != '\0') {
    pcVar1 = "--preserve";
  }
  pcVar2 = strchr(__s,0x2c);
  if (pcVar2 != (char *)0x0) {
    *pcVar2 = '\0';
  }
  lVar3 = FUN_0010d220(pcVar1,__s,&DAT_00124460,&DAT_0011cab0,4,PTR_FUN_00125028);
  if (*(uint *)(&DAT_0011cab0 + lVar3 * 4) < 7) {
                    /* WARNING: Could not recover jumptable at 0x00104b2d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_0011c990 +
              *(int *)(&DAT_0011c990 + (ulong)*(uint *)(&DAT_0011cab0 + lVar3 * 4) * 4)))();
    return;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: make_dir_parents_private @ 0x4bf0

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
  __n = FUN_0010e750();
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
    *(undefined8 *)((long)ppsVar17 + lVar2 + -8) = 0x104ce5;
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
    *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104d4f;
    memcpy(__dest,local_170,__n);
    pcVar21 = (char *)((long)__dest + param_2);
    *(undefined1 *)((long)__dest + __n) = 0;
    cVar3 = *pcVar21;
    while (cVar3 == '/') {
      pcVar21 = pcVar21 + 1;
      cVar3 = *pcVar21;
    }
    local_188 = &local_168;
    *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104d84;
    iVar4 = fstatat(param_3,pcVar21,local_188,0);
    if (iVar4 == 0) {
      if ((local_168.st_mode & 0xf000) != 0x4000) {
LAB_001051fc:
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105206;
        uVar7 = FUN_00114b40(4,__dest);
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10521c;
        uVar12 = dcgettext(0,"%s exists but is not a directory",5);
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10522d;
        error(0,0,uVar12,uVar7);
LAB_0010522d:
        uVar7 = 0;
        puVar16 = puVar20 + lVar2;
        goto LAB_00104c55;
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
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104dda;
        pcVar9 = strchr(pcVar9,0x2f);
        ppsVar15 = (stat **)(puVar20 + lVar2);
        if (pcVar9 == (char *)0x0) break;
        *pcVar9 = '\0';
        *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104dfd;
        iVar4 = fstatat(param_3,pcVar21,local_188,0);
        if ((iVar4 == 0) && ((*(uint *)(param_7 + 0x1c) & 0xffffff00) == 0)) {
LAB_00104e17:
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104e31;
          cVar3 = FUN_00107ff0(local_178,local_170,0,0,param_7);
          if (cVar3 == '\0') goto LAB_0010522d;
          __dest = local_170;
          if ((local_168.st_mode & 0xf000) != 0x4000) goto LAB_001051fc;
          *local_180 = '\0';
LAB_00104e59:
          if ((*(long *)(param_7 + 0x28) != 0) || (*(char *)(param_7 + 0x33) != '\0')) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104e76;
            cVar3 = FUN_00108250(local_170,0,param_7);
            if ((cVar3 == '\0') && (*(char *)(param_7 + 0x34) != '\0')) goto LAB_0010522d;
          }
        }
        else {
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ed3;
          iVar5 = stat(local_178,local_1a8);
          if (iVar5 == 0) {
            if ((local_d8.st_mode & 0xf000) != 0x4000) {
              iVar5 = 0x14;
              goto LAB_00105114;
            }
          }
          else {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ee0;
            piVar10 = __errno_location();
            iVar5 = *piVar10;
            if (iVar5 != 0) {
LAB_00105114:
              local_170 = (void *)CONCAT44(local_170._4_4_,iVar5);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10512c;
              uVar7 = FUN_00114b40(4,local_178);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105142;
              uVar12 = dcgettext(0,"failed to get attributes of %s",5);
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105157;
              error(0,(ulong)local_170 & 0xffffffff,uVar12,uVar7);
              uVar7 = 0;
              goto LAB_00104c55;
            }
          }
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104ef6;
          p_Var11 = (__dev_t *)FUN_00116e90(0xa8);
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
          if (iVar4 == 0) goto LAB_00104e17;
          _Var1 = p_Var11[3];
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x104fb7;
          local_198 = p_Var11;
          cVar3 = FUN_00107ff0(local_178,local_170,(int)_Var1,1,param_7);
          if (cVar3 == '\0') goto LAB_0010522d;
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
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105014;
          iVar4 = mkdirat(param_3,pcVar21,uVar6 & uVar13 & 0xfff);
          if (iVar4 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1052a4;
            uVar7 = FUN_00114b40(4,local_170);
            pcVar21 = "cannot make directory %s";
LAB_00105260:
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105267;
            uVar12 = dcgettext(0,pcVar21,5);
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x10526f;
            piVar10 = __errno_location();
            iVar4 = *piVar10;
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105280;
            error(0,iVar4,uVar12,uVar7);
            uVar7 = 0;
            puVar16 = puVar20 + lVar2;
            goto LAB_00104c55;
          }
          if (local_1a0 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105050;
            __printf_chk(1,local_1a0,local_178,local_170);
          }
          *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105075;
          iVar4 = fstatat(param_3,pcVar21,local_188,0x100);
          if (iVar4 != 0) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x105251;
            uVar7 = FUN_00114b40(4,local_170);
            pcVar21 = "failed to get attributes of %s";
            goto LAB_00105260;
          }
          if (*(char *)(param_7 + 0x1e) == '\0') {
            if ((~local_168.st_mode & uVar22) == 0) {
LAB_0010509f:
              if ((local_168.st_mode & 0x1c0) == 0x1c0) goto LAB_001050be;
            }
            else {
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1051b1;
              uVar6 = FUN_00108640();
              uVar22 = uVar22 & ~uVar6;
              if ((~local_168.st_mode & uVar22) == 0) goto LAB_0010509f;
            }
            *(undefined1 *)(local_198 + 0x12) = 1;
            *(uint *)(local_198 + 3) = uVar22 | local_168.st_mode;
          }
LAB_001050be:
          if ((local_168.st_mode | 0x1c0) != local_168.st_mode) {
            *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1050d5;
            iVar4 = FUN_0010e520(param_3,pcVar21);
            if (iVar4 != 0) {
              *(undefined8 *)(puVar20 + lVar2 + -8) = 0x1052c6;
              uVar7 = FUN_00114b40(4,local_170);
              pcVar21 = "setting permissions for %s";
              goto LAB_00105260;
            }
          }
          if (*local_180 == '\0') goto LAB_00104e59;
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
LAB_00104c55:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined **)(puVar16 + -8) = &UNK_001052dc;
  __stack_chk_fail();
}




// Function: re_protect @ 0x52e0

ulong re_protect(char *param_1,int param_2,ulong param_3,long param_4,long param_5)

{
  undefined4 uVar1;
  long lVar2;
  timespec **pptVar3;
  char cVar4;
  int iVar5;
  size_t sVar6;
  void *pvVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  int *piVar10;
  timespec **pptVar11;
  char *pcVar13;
  ulong uVar14;
  long in_FS_OFFSET;
  timespec tVar15;
  undefined1 auVar16 [16];
  timespec *local_78;
  ulong local_70;
  timespec local_68;
  undefined1 local_58 [16];
  long local_40;
  timespec **pptVar12;
  
  pptVar11 = &local_78;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_70 = param_3;
  sVar6 = strlen(param_1);
  pptVar12 = &local_78;
  pptVar3 = &local_78;
  while (pptVar12 != (timespec **)((long)&local_78 - (sVar6 + 0x18 & 0xfffffffffffff000))) {
    pptVar11 = (timespec **)((long)pptVar3 + -0x1000);
    *(undefined8 *)((long)pptVar3 + -8) = *(undefined8 *)((long)pptVar3 + -8);
    pptVar12 = (timespec **)((long)pptVar3 + -0x1000);
    pptVar3 = (timespec **)((long)pptVar3 + -0x1000);
  }
  uVar14 = (ulong)((uint)(sVar6 + 0x18) & 0xff0);
  lVar2 = -uVar14;
  if (uVar14 != 0) {
    *(undefined8 *)((long)pptVar11 + -8) = *(undefined8 *)((long)pptVar11 + -8);
  }
  *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105370;
  pvVar7 = memcpy((void *)((ulong)((long)pptVar11 + lVar2 + 0xf) & 0xfffffffffffffff0),param_1,
                  sVar6 + 1);
  pcVar13 = (char *)((long)pvVar7 + (local_70 - (long)param_1));
  if (param_4 != 0) {
    local_78 = &local_68;
    do {
      cVar4 = *(char *)(param_5 + 0x1f);
      *(undefined1 *)((long)pvVar7 + *(long *)(param_4 + 0x98)) = 0;
      if (cVar4 != '\0') {
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x1053f5;
        tVar15 = (timespec)FUN_00115960(param_4);
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105405;
        local_68 = tVar15;
        auVar16 = FUN_00115980(param_4);
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x10541e;
        local_58 = auVar16;
        iVar5 = utimensat(param_2,pcVar13,local_78,0);
        if (iVar5 != 0) {
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105433;
          uVar8 = FUN_00114b40(4,pvVar7);
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105449;
          uVar9 = dcgettext(0,"failed to preserve times for %s",5);
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105451;
          piVar10 = __errno_location();
          iVar5 = *piVar10;
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105462;
          error(0,iVar5,uVar9,uVar8);
          uVar14 = 0;
          goto LAB_00105556;
        }
      }
      if (*(char *)(param_5 + 0x1d) != '\0') {
        uVar1 = *(undefined4 *)(param_4 + 0x1c);
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105481;
        iVar5 = FUN_0010e540(param_2,pcVar13,uVar1);
        if (iVar5 == 0) goto LAB_001053a3;
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105491;
        cVar4 = FUN_00108400(param_5);
        if (cVar4 != '\0') {
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x1054ac;
          FUN_0010e540(param_2,pcVar13,0xffffffff);
          cVar4 = *(char *)(param_5 + 0x1e);
          goto joined_r0x001054b4;
        }
        local_70 = local_70 & 0xffffffffffffff00;
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105592;
        uVar8 = FUN_00114b40(4,pvVar7);
        pcVar13 = "failed to preserve ownership for %s";
LAB_00105526:
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x10552d;
        uVar9 = dcgettext(0,pcVar13,5);
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105535;
        piVar10 = __errno_location();
        iVar5 = *piVar10;
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105546;
        error(0,iVar5,uVar9,uVar8);
        uVar14 = local_70 & 0xff;
        goto LAB_00105556;
      }
LAB_001053a3:
      cVar4 = *(char *)(param_5 + 0x1e);
joined_r0x001054b4:
      if (cVar4 == '\0') {
        if (*(char *)(param_4 + 0x90) != '\0') {
          uVar1 = *(undefined4 *)(param_4 + 0x18);
          local_70 = local_70 & 0xffffffffffffff00;
          *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105502;
          iVar5 = FUN_0010e520(param_2,pcVar13,uVar1);
          if (iVar5 != 0) {
            *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x105517;
            uVar8 = FUN_00114b40(4,pvVar7);
            pcVar13 = "failed to preserve permissions for %s";
            goto LAB_00105526;
          }
        }
      }
      else {
        uVar1 = *(undefined4 *)(param_4 + 0x18);
        *(undefined8 *)((long)pptVar11 + lVar2 + -8) = 0x1054d9;
        iVar5 = FUN_0010caa0(pcVar13,0xffffffff,pvVar7,0xffffffff,uVar1);
        if (iVar5 != 0) {
          uVar14 = 0;
          goto LAB_00105556;
        }
      }
      *(undefined1 *)((long)pvVar7 + *(long *)(param_4 + 0x98)) = 0x2f;
      param_4 = *(long *)(param_4 + 0xa0);
    } while (param_4 != 0);
  }
  uVar14 = 1;
LAB_00105556:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)((long)pptVar11 + lVar2 + -8) = &UNK_001055a8;
    __stack_chk_fail();
  }
  return uVar14;
}




// Function: usage @ 0x57e0

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_001255a0;
  if (param_1 == 0) {
    uVar3 = dcgettext(0,
                      "Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n"
                      ,5);
    __printf_chk(1,uVar3,uVar2,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00104970();
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
    FUN_001049a0();
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_001055b0();
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: do_copy @ 0x5ae0

uint do_copy(int param_1,undefined8 *param_2,long param_3,char param_4,int *param_5)

{
  void *pvVar1;
  long lVar2;
  void *__ptr;
  byte bVar3;
  char cVar4;
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
  undefined4 local_100;
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
        if (param_1 < 3) goto LAB_00105d25;
      }
      else {
        uVar11 = dcgettext(0,"cannot combine --target-directory (-t) and --no-target-directory (-T)"
                           ,5);
        param_2 = (undefined8 *)0x0;
        error(1,0,uVar11);
      }
      uVar11 = FUN_00114b40(4,param_2[2]);
      pcVar20 = "extra operand %s";
LAB_001060c1:
      uVar12 = dcgettext(0,pcVar20,5);
      error(0,0,uVar12,uVar11);
      goto LAB_001060d9;
    }
    if (param_3 == 0) {
      lVar15 = param_2[(long)param_1 + -1];
      local_110 = lVar15;
      local_100 = FUN_00115a20(lVar15,&local_d8);
      uVar23 = FUN_00115a10(local_100);
      if ((char)uVar23 == '\0') {
        piVar13 = __errno_location();
        iVar6 = *piVar13;
        if (iVar6 == 2) goto LAB_0010605b;
        while (2 < local_fc) {
          param_2 = (undefined8 *)FUN_00114b40(4,local_110);
          uVar11 = dcgettext(0,"target %s",5);
          error(1,iVar6,uVar11,param_2);
LAB_0010605b:
          local_ea = 1;
        }
      }
      else {
        local_fc = param_1 + -1;
        if (lVar15 != 0) {
          if (1 < local_fc) goto LAB_00105f87;
          if (local_fc == 1) goto LAB_00105b81;
          goto LAB_00105ce5;
        }
      }
LAB_00105d25:
      bVar3 = local_ea;
      pcVar20 = (char *)*param_2;
      pcVar8 = (char *)param_2[1];
      if (DAT_00125159 != '\0') {
        pcVar20 = "with --parents, the destination must be a directory";
        goto LAB_0010613c;
      }
      if (((((*(char *)((long)local_108 + 0x16) != '\0') && (*local_108 != 0)) &&
           (iVar6 = strcmp(pcVar20,pcVar8), iVar6 == 0)) &&
          ((bVar3 == 0 && ((local_d8.st_mode != 0 || (iVar6 = stat(pcVar8,&local_d8), iVar6 == 0))))
          )) && (piVar13 = local_108, (local_d8.st_mode & 0xf000) == 0x8000)) {
        pcVar8 = (char *)FUN_0010daa0(0xffffff9c,pcVar8,*local_108);
        piVar21 = &DAT_00125100;
        for (lVar15 = 0x16; lVar15 != 0; lVar15 = lVar15 + -1) {
          *piVar21 = *piVar13;
          piVar13 = piVar13 + (ulong)bVar24 * -2 + 1;
          piVar21 = piVar21 + (ulong)bVar24 * -2 + 1;
        }
        DAT_00125100 = 0;
        local_108 = &DAT_00125100;
      }
      uVar23 = FUN_0010bae0(pcVar20,pcVar8,0xffffff9c,pcVar8,-(uint)bVar3,local_108,&local_e0,0);
      ppbVar18 = &local_128;
LAB_00105ce5:
      ppbVar19 = ppbVar18;
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar23;
      }
      goto LAB_00106126;
    }
    local_100 = FUN_00115a20(param_3,&local_d8);
    cVar4 = FUN_00115a10(local_100);
    if (cVar4 != '\0') {
      if (local_fc != 1) {
LAB_00105f87:
        piVar13 = local_108;
        FUN_00108330(local_108);
        FUN_00108370(piVar13);
      }
LAB_00105b81:
      local_120 = &local_e0;
      lVar15 = 0;
      uVar23 = 1;
      local_118 = &local_e9;
      local_128 = &local_ea;
      do {
        pcVar20 = (char *)param_2[lVar15];
        local_e0 = (char *)0x0;
        if (DAT_00125158 == '\0') {
          if (DAT_00125159 != '\0') goto LAB_00105e18;
LAB_00105be5:
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105bea;
          pcVar8 = (char *)FUN_0010db60();
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105bf5;
          sVar9 = strlen(pcVar8);
          for (puVar16 = (undefined1 *)ppbVar17;
              puVar16 != (undefined1 *)((long)ppbVar17 + -(sVar9 + 0x18 & 0xfffffffffffff000));
              puVar16 = puVar16 + -0x1000) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          uVar14 = (ulong)((uint)(sVar9 + 0x18) & 0xff0);
          lVar2 = -uVar14;
          ppbVar17 = (byte **)(puVar16 + lVar2);
          if (uVar14 != 0) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105c52;
          pcVar8 = memcpy((void *)((ulong)(puVar16 + lVar2 + 0xf) & 0xfffffffffffffff0),pcVar8,
                          sVar9 + 1);
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105c5d;
          FUN_0010e800(pcVar8);
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105c6c;
          iVar6 = strcmp(pcVar8,"..");
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105c89;
          pvVar10 = (void *)FUN_0010ec90(local_110,pcVar8 + (iVar6 == 0),local_120);
LAB_00105c8c:
          *(undefined8 *)((long)ppbVar17 + -8) = 0;
          *(undefined1 **)((long)ppbVar17 + -0x10) = local_118;
          *(undefined8 *)((long)ppbVar17 + -0x18) = 0x105cbb;
          uVar7 = FUN_0010bae0(pcVar20,pvVar10,local_100,local_e0,local_ea,local_108);
          uVar23 = uVar23 & uVar7;
          if (DAT_00125159 != '\0') {
            *(undefined8 *)((long)ppbVar17 + -8) = 0x105f7b;
            uVar7 = FUN_001052e0(pvVar10,local_100,local_e0,local_e8,local_108);
            uVar23 = uVar23 & uVar7;
            goto LAB_00105f1b;
          }
        }
        else {
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105e08;
          FUN_0010e800(pcVar20);
          if (DAT_00125159 == '\0') goto LAB_00105be5;
LAB_00105e18:
          *(undefined8 *)((long)ppbVar17 + -8) = 0x105e1d;
          sVar9 = strlen(pcVar20);
          for (puVar16 = (undefined1 *)ppbVar17;
              puVar16 != (undefined1 *)((long)ppbVar17 + -(sVar9 + 0x18 & 0xfffffffffffff000));
              puVar16 = puVar16 + -0x1000) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          uVar14 = (ulong)((uint)(sVar9 + 0x18) & 0xff0);
          lVar2 = -uVar14;
          ppbVar17 = (byte **)(puVar16 + lVar2);
          if (uVar14 != 0) {
            *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
          }
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105e7a;
          pvVar10 = memcpy((void *)((ulong)(puVar16 + lVar2 + 0xf) & 0xfffffffffffffff0),pcVar20,
                           sVar9 + 1);
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105e85;
          FUN_0010e800(pvVar10);
          *(undefined8 *)(puVar16 + lVar2 + -8) = 0x105e9b;
          lVar22 = local_110;
          pvVar10 = (void *)FUN_0010ec90(local_110,pvVar10,local_120);
          pcVar8 = (char *)0x0;
          iVar6 = local_108[0xf];
          *(long *)(puVar16 + lVar2 + -8) = lVar22;
          if ((char)iVar6 != '\0') {
            pcVar8 = "%s -> %s\n";
          }
          *(int **)(puVar16 + lVar2 + -0x10) = local_108;
          *(undefined8 *)(puVar16 + lVar2 + -0x18) = 0x105ede;
          cVar5 = FUN_00104bf0(pvVar10,(long)local_e0 - (long)pvVar10,local_100,pcVar8,&local_e8,
                               local_128);
          cVar4 = *local_e0;
          while (cVar4 == '/') {
            local_e0 = local_e0 + 1;
            cVar4 = *local_e0;
          }
          if (cVar5 != '\0') goto LAB_00105c8c;
          uVar23 = 0;
          ppbVar17 = (byte **)(puVar16 + lVar2);
LAB_00105f1b:
          __ptr = local_e8;
          if (DAT_00125159 != '\0') {
            while (__ptr != (void *)0x0) {
              pvVar1 = *(void **)((long)__ptr + 0xa0);
              *(undefined8 *)((long)ppbVar17 + -8) = 0x105f4e;
              local_e8 = pvVar1;
              free(__ptr);
              __ptr = pvVar1;
            }
          }
        }
        lVar15 = lVar15 + 1;
        *(undefined8 *)((long)ppbVar17 + -8) = 0x105cd9;
        free(pvVar10);
        ppbVar18 = ppbVar17;
      } while ((int)lVar15 < local_fc);
      goto LAB_00105ce5;
    }
  }
  else {
    if (param_1 == 1) {
      uVar11 = FUN_00114b40(4,*param_2);
      pcVar20 = "missing destination file operand after %s";
      goto LAB_001060c1;
    }
    pcVar20 = "missing file operand";
LAB_0010613c:
    uVar11 = dcgettext(0,pcVar20,5);
    error(0,0,uVar11);
LAB_001060d9:
    FUN_001057e0(1);
  }
  uVar11 = FUN_00114b40(4,local_110);
  uVar12 = dcgettext(0,"target directory %s",5);
  piVar13 = __errno_location();
  error(1,*piVar13,uVar12,uVar11);
LAB_00106126:
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)((long)ppbVar19 + -8) = 0x10612b;
  __stack_chk_fail();
}




// Function: is_ancestor @ 0x61e0

undefined8 is_ancestor(long *param_1,long *param_2)

{
  if (param_2 != (long *)0x0) {
    do {
      if ((param_2[1] == param_1[1]) && (param_2[2] == *param_1)) {
        return 1;
      }
      param_2 = (long *)*param_2;
    } while (param_2 != (long *)0x0);
  }
  return 0;
}




// Function: copy_attr_free @ 0x6220

void copy_attr_free(void)

{
  return;
}




// Function: owner_failure_ok @ 0x6230

byte owner_failure_ok(long param_1)

{
  int iVar1;
  byte bVar2;
  int *piVar3;
  
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  bVar2 = iVar1 == 1 || iVar1 == 0x16;
  if (iVar1 == 1 || iVar1 == 0x16) {
    bVar2 = *(byte *)(param_1 + 0x1b) ^ 1;
  }
  return bVar2;
}




// Function: copy_attr_quote @ 0x6260

void copy_attr_quote(void)

{
  FUN_00114b40(4);
  return;
}




// Function: valid_options @ 0x6270

undefined8 valid_options(uint *param_1)

{
  if (3 < *param_1) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options");
  }
  if (2 < param_1[3] - 1) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options");
  }
  if (2 < param_1[0x11]) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options");
  }
  if ((*(char *)((long)param_1 + 0x17) != '\0') && (*(char *)((long)param_1 + 0x3a) != '\0')) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options");
  }
  if ((param_1[0x11] == 2) && (param_1[3] != 2)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
                  "src/copy.c",0xc13,"valid_options");
  }
  return 1;
}




// Function: writable_destination @ 0x6370

undefined8 writable_destination(int param_1,char *param_2,uint param_3)

{
  char cVar1;
  int iVar2;
  undefined4 extraout_var;
  
  if ((param_3 & 0xf000) == 0xa000) {
    return 1;
  }
  cVar1 = FUN_00116e20();
  if (cVar1 != '\0') {
    return 1;
  }
  iVar2 = faccessat(param_1,param_2,2,0x200);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar2) >> 8),iVar2 == 0);
}




// Function: overwrite_ok @ 0x63e0

void overwrite_ok(long param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,long param_5
                 )

{
  uint uVar1;
  undefined8 uVar2;
  char cVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 local_3c;
  undefined1 local_3b [9];
  undefined1 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar3 = FUN_00106370(param_3,param_4,*(undefined4 *)(param_5 + 0x18));
  if (cVar3 == '\0') {
    FUN_0010eb60(*(undefined4 *)(param_5 + 0x18),&local_3c);
    uVar1 = *(uint *)(param_5 + 0x18);
    local_32 = 0;
    uVar4 = FUN_00114b40(4,param_2);
    uVar2 = DAT_001255a0;
    if ((*(char *)(param_1 + 0x18) == '\0') && ((*(uint *)(param_1 + 0x14) & 0xffff00) == 0)) {
      uVar5 = dcgettext(0,"%s: unwritable %s (mode %04lo, %s); try anyway? ",5);
    }
    else {
      uVar5 = dcgettext(0,"%s: replace %s, overriding mode %04lo (%s)? ",5);
    }
    __fprintf_chk(stderr,1,uVar5,uVar2,uVar4,uVar1 & 0xfff,local_3b);
  }
  else {
    uVar4 = FUN_00114b40(4,param_2);
    uVar2 = DAT_001255a0;
    uVar5 = dcgettext(0,"%s: overwrite %s? ",5);
    __fprintf_chk(stderr,1,uVar5,uVar2,uVar4);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00117540();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: abandon_move @ 0x6530

uint abandon_move(long param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,long param_5
                 )

{
  int iVar1;
  char cVar2;
  uint uVar3;
  
  uVar3 = (uint)*(byte *)(param_1 + 0x18);
  if (*(byte *)(param_1 + 0x18) == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move");
  }
  iVar1 = *(int *)(param_1 + 8);
  if (iVar1 != 2) {
    if ((iVar1 == 3) ||
       (((iVar1 == 4 && (*(char *)(param_1 + 0x3d) != '\0')) &&
        (cVar2 = FUN_00106370(param_3,param_4,*(undefined4 *)(param_5 + 0x18)), cVar2 == '\0')))) {
      uVar3 = FUN_001063e0(param_1,param_2,param_3,param_4,param_5);
      return uVar3 ^ 1;
    }
    uVar3 = 0;
  }
  return uVar3;
}




// Function: subst_suffix @ 0x65e0

void subst_suffix(void *param_1,long param_2,char *param_3)

{
  size_t sVar1;
  void *__dest;
  size_t __n;
  
  __n = param_2 - (long)param_1;
  sVar1 = strlen(param_3);
  __dest = (void *)FUN_00116eb0(sVar1 + 1 + __n);
  memcpy((void *)((long)__dest + __n),param_3,sVar1 + 1);
  memcpy(__dest,param_1,__n);
  return;
}




// Function: source_is_dst_backup @ 0x6640

bool source_is_dst_backup(char *param_1,__dev_t *param_2,int param_3,char *param_4)

{
  int iVar1;
  size_t sVar2;
  char *__s;
  size_t __n;
  size_t sVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  bool bVar5;
  stat local_d8;
  long local_40;
  
  bVar5 = false;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar2 = strlen(param_1);
  __s = (char *)FUN_0010db60(param_4);
  __n = strlen(__s);
  pcVar4 = DAT_00125580;
  sVar3 = strlen(DAT_00125580);
  if (sVar3 + __n == sVar2) {
    iVar1 = memcmp(param_1,__s,__n);
    if (iVar1 == 0) {
      iVar1 = strcmp(param_1 + __n,pcVar4);
      if (iVar1 == 0) {
        sVar2 = strlen(param_4);
        pcVar4 = (char *)FUN_001065e0(param_4,param_4 + sVar2,pcVar4);
        iVar1 = fstatat(param_3,pcVar4,&local_d8,0);
        free(pcVar4);
        bVar5 = false;
        if ((iVar1 == 0) && (param_2[1] == local_d8.st_ino)) {
          bVar5 = *param_2 == local_d8.st_dev;
        }
      }
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: clone_file @ 0x6770

void clone_file(int param_1,ulong param_2)

{
  ioctl(param_1,0x40049409,param_2 & 0xffffffff);
  return;
}




// Function: infer_scantype @ 0x6780

int infer_scantype(int param_1,long param_2,__off_t *param_3)

{
  long lVar1;
  byte bVar2;
  long lVar3;
  __off_t _Var4;
  int *piVar5;
  int iVar6;
  
  iVar6 = 1;
  if ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x8000) {
    lVar1 = *(long *)(param_2 + 0x30);
    lVar3 = lVar1 + 0x1ff;
    if (-1 < lVar1) {
      lVar3 = lVar1;
    }
    if (*(long *)(param_2 + 0x40) < lVar3 >> 9) {
      iVar6 = 3;
      _Var4 = lseek(param_1,0,3);
      *param_3 = _Var4;
      if (_Var4 < 0) {
        piVar5 = __errno_location();
        if ((*piVar5 != 6) && (iVar6 = 2, *piVar5 != 0x16)) {
          bVar2 = FUN_001061c0();
          return (uint)bVar2 * 2;
        }
      }
    }
  }
  return iVar6;
}




// Function: write_zeros @ 0x6820

undefined8 write_zeros(undefined4 param_1,ulong param_2)

{
  ulong uVar1;
  ulong uVar2;
  
  if ((DAT_00125560 == (undefined *)0x0) &&
     (DAT_00125560 = calloc(DAT_00125018,1), DAT_00125560 == (undefined *)0x0)) {
    DAT_00125018 = 0x400;
    DAT_00125560 = &DAT_00125160;
  }
  while( true ) {
    if (param_2 == 0) {
      return 1;
    }
    uVar2 = DAT_00125018;
    if (param_2 <= DAT_00125018) {
      uVar2 = param_2;
    }
    uVar1 = FUN_00111320(param_1,DAT_00125560,uVar2);
    if (uVar1 != uVar2) break;
    param_2 = param_2 - uVar1;
  }
  return 0;
}




// Function: punch_hole @ 0x68d0

int punch_hole(int param_1,__off_t param_2,__off_t param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  int extraout_EDX;
  
  iVar2 = fallocate(param_1,3,param_2,param_3);
  if (iVar2 < 0) {
    piVar3 = __errno_location();
    cVar1 = FUN_001061c0(*piVar3);
    if ((extraout_EDX == 0x26) || (cVar1 != '\0')) {
      return 0;
    }
  }
  return iVar2;
}




// Function: create_hole @ 0x6910

undefined8 create_hole(int param_1,undefined8 param_2,char param_3,long param_4)

{
  int iVar1;
  __off_t _Var2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  char *pcVar6;
  
  _Var2 = lseek(param_1,param_4,1);
  if (_Var2 < 0) {
    uVar3 = FUN_00114b40(4,param_2);
    pcVar6 = "cannot lseek %s";
LAB_001069a4:
    uVar4 = dcgettext(0,pcVar6,5);
    piVar5 = __errno_location();
    error(0,*piVar5,uVar4,uVar3);
    return 0;
  }
  if (param_3 != '\0') {
    iVar1 = FUN_001068d0(param_1,_Var2 - param_4,param_4);
    if (iVar1 < 0) {
      uVar3 = FUN_00114b40(4,param_2);
      pcVar6 = "error deallocating %s";
      goto LAB_001069a4;
    }
  }
  return 1;
}




// Function: sparse_copy @ 0x6a40

undefined4 *
sparse_copy(int param_1,ulong param_2,long *param_3,ulong param_4,ulong param_5,undefined1 param_6,
            char param_7,undefined8 param_8,undefined8 param_9,ulong param_10,long *param_11,
            undefined1 *param_12)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  long lVar5;
  ulong uVar6;
  int *piVar7;
  ulong uVar8;
  uint *puVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  void *__buf;
  undefined4 *puVar13;
  ulong uVar14;
  undefined4 *extraout_RDX;
  ulong uVar15;
  undefined4 *unaff_RBP;
  undefined4 uVar16;
  byte bVar17;
  void *pvVar18;
  byte bVar19;
  ulong uVar20;
  ulong local_b0;
  ulong local_98;
  
  uVar16 = (undefined4)param_2;
  *param_12 = 0;
  local_98 = param_10;
  *param_11 = 0;
  if ((param_5 == 0) && (param_7 != '\0')) {
    if (param_10 != 0) {
      unaff_RBP = (undefined4 *)(param_2 & 0xffffffff);
      do {
        while( true ) {
          uVar14 = 0x7fffffffc0000000;
          if (param_10 < 0x7fffffffc0000001) {
            uVar14 = param_10;
          }
          lVar5 = copy_file_range(param_1,0,unaff_RBP,0,uVar14);
          local_98 = param_10;
          if (lVar5 == 0) {
            if (*param_11 != 0) {
              return (undefined4 *)0x1;
            }
            goto LAB_00106b30;
          }
          if (lVar5 < 0) break;
          *param_11 = *param_11 + lVar5;
          param_10 = param_10 - lVar5;
          if (param_10 == 0) {
            return (undefined4 *)0x1;
          }
        }
        puVar9 = (uint *)__errno_location();
        uVar2 = *puVar9;
        if ((uVar2 == 0x26) || (uVar3 = FUN_001061c0(uVar2), (char)uVar3 != '\0'))
        goto LAB_00106b30;
        if (uVar2 < 0x1b) {
          if ((1L << ((byte)uVar2 & 0x3f) & 0x4440200U) != 0) goto LAB_00106b30;
          if (uVar2 == 1) {
            unaff_RBP = extraout_RDX;
            if (*param_11 == 0) goto LAB_00106b30;
            goto code_r0x00106e4a;
          }
        }
        if (uVar2 != 4) {
code_r0x00106e4a:
          uVar10 = FUN_00114a70(1,4,param_9);
          uVar11 = FUN_00114a70(0,4,param_8);
          uVar12 = dcgettext(0,"error copying %s to %s",5);
          error(0,*extraout_RDX,uVar12,uVar11,uVar10);
          return (undefined4 *)(ulong)uVar3;
        }
      } while( true );
    }
  }
  else if (param_10 != 0) {
LAB_00106b30:
    uVar14 = param_4;
    if (param_5 != 0) {
      uVar14 = param_5;
    }
    uVar20 = 0;
    puVar13 = (undefined4 *)0x0;
    while( true ) {
      while( true ) {
        __buf = (void *)*param_3;
        if (__buf == (void *)0x0) {
          iVar4 = getpagesize();
          __buf = (void *)FUN_00116e60((long)iVar4,param_4);
          *param_3 = (long)__buf;
        }
        uVar6 = local_98;
        if (param_4 <= local_98) {
          uVar6 = param_4;
        }
        uVar6 = read(param_1,__buf,uVar6);
        if (-1 < (long)uVar6) break;
        piVar7 = __errno_location();
        if (*piVar7 != 4) {
          uVar10 = FUN_00114b40(4,param_8);
          uVar11 = dcgettext(0,"error reading %s",5);
          error(0,*piVar7,uVar11,uVar10);
          return (undefined4 *)0x0;
        }
      }
      if (uVar6 == 0) break;
      *param_11 = *param_11 + uVar6;
      uVar15 = uVar14;
      pvVar18 = __buf;
      local_b0 = uVar6;
LAB_00106c20:
      do {
        if (local_b0 < uVar15) {
          uVar15 = local_b0;
        }
        bVar19 = (byte)puVar13;
        if (uVar15 != 0 && param_5 != 0) {
          uVar8 = FUN_001069e0(__buf,uVar15);
          unaff_RBP = (undefined4 *)(uVar8 & 0xffffffff);
          bVar17 = ((byte)uVar8 ^ bVar19) & uVar20 != 0;
          if (((char)unaff_RBP == '\x01') || (uVar15 != local_b0)) {
            if (bVar17 == 0) goto LAB_00106d71;
            bVar1 = false;
          }
          else {
            if (bVar17 == 0) {
              unaff_RBP = (undefined4 *)0x0;
              goto LAB_00106d13;
            }
            bVar1 = true;
            unaff_RBP = (undefined4 *)0x0;
          }
joined_r0x00106d2c:
          if (bVar19 == 0) {
            uVar8 = FUN_00111320(uVar16,pvVar18,uVar20);
            if (uVar20 != uVar8) {
              uVar10 = FUN_00114b40(4,param_9);
              uVar11 = dcgettext(0,"error writing %s",5);
              piVar7 = __errno_location();
              error(0,*piVar7,uVar11,uVar10);
              return puVar13;
            }
          }
          else {
            uVar2 = FUN_00106910(uVar16,param_9,param_6,uVar20);
            if ((char)uVar2 == '\0') {
              return (undefined4 *)(ulong)uVar2;
            }
          }
          pvVar18 = __buf;
          uVar20 = uVar15;
          if (bVar1) {
            if (uVar15 == 0) {
              if (bVar17 != 0) {
                uVar20 = 0;
                break;
              }
              local_b0 = 0;
            }
            else if (bVar17 != 0) {
              puVar13 = (undefined4 *)((ulong)unaff_RBP & 0xffffffff);
              uVar15 = 0;
              goto LAB_00106c20;
            }
            local_b0 = local_b0 - uVar15;
            puVar13 = (undefined4 *)((ulong)unaff_RBP & 0xffffffff);
            uVar20 = 0;
          }
          else {
            local_b0 = local_b0 - uVar15;
            puVar13 = (undefined4 *)((ulong)unaff_RBP & 0xffffffff);
          }
        }
        else {
          if (((bVar19 != 1) &&
              (unaff_RBP = (undefined4 *)
                           CONCAT71((int7)((ulong)unaff_RBP >> 8),uVar15 != 0 && param_5 != 0),
              uVar15 == local_b0)) || (unaff_RBP = puVar13, uVar15 == 0)) {
LAB_00106d13:
            uVar20 = uVar20 + uVar15;
            bVar1 = true;
            bVar17 = 0;
            goto joined_r0x00106d2c;
          }
LAB_00106d71:
          uVar20 = uVar20 + uVar15;
          if (uVar20 + 0x8000000000000000 < uVar15) {
            uVar10 = FUN_00114b40(4,param_8);
            uVar11 = dcgettext(0,"overflow reading %s",5);
            error(0,0,uVar11,uVar10);
            return (undefined4 *)0x0;
          }
          local_b0 = local_b0 - uVar15;
          puVar13 = unaff_RBP;
        }
        __buf = (void *)((long)__buf + uVar15);
      } while (local_b0 != 0);
      local_98 = local_98 - uVar6;
      *param_12 = (char)unaff_RBP;
      puVar13 = unaff_RBP;
      if (local_98 == 0) break;
      puVar13 = (undefined4 *)((ulong)unaff_RBP & 0xffffffff);
    }
    if ((char)puVar13 != '\0') {
      puVar13 = (undefined4 *)FUN_00106910(uVar16,param_9,param_6,uVar20);
      return puVar13;
    }
  }
  return (undefined4 *)0x1;
}




// Function: fchmod_or_lchmod @ 0x7020

void fchmod_or_lchmod(int param_1,undefined4 param_2,undefined8 param_3,__mode_t param_4)

{
  if (param_1 < 0) {
    FUN_0010e520(param_2,param_3,param_4);
    return;
  }
  fchmod(param_1,param_4);
  return;
}




// Function: copy_attr @ 0x7040

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
    pcVar3 = FUN_00107330;
    local_28 = FUN_00107260;
    if (cVar1 == '\0') {
      pcVar3 = FUN_00107330;
      if (*(long *)(param_5 + 0x28) == 0) {
        pcVar3 = (code *)0x0;
      }
      local_28 = FUN_00107260;
    }
  }
  else {
    if (*(char *)(param_5 + 0x37) != '\0') {
      pcVar3 = FUN_00107330;
      ppcVar4 = (code **)0x0;
      if (cVar1 == '\0') {
        pcVar3 = (code *)0x0;
        ppcVar4 = (code **)0x0;
        if (*(long *)(param_5 + 0x28) != 0) {
          pcVar3 = FUN_00107330;
          ppcVar4 = (code **)0x0;
        }
      }
      goto LAB_001070a2;
    }
    pcVar3 = FUN_00107330;
    local_28 = FUN_00107180;
    if ((cVar1 == '\0') && (*(long *)(param_5 + 0x28) == 0)) {
      pcVar3 = (code *)0x0;
    }
  }
  local_20 = FUN_00106260;
  local_18 = FUN_00106220;
LAB_001070a2:
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




// Function: copy_attr_error @ 0x7180

void copy_attr_error(void)

{
  long lVar1;
  char cVar2;
  int *piVar3;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  piVar3 = __errno_location();
  cVar2 = FUN_001061d0(*piVar3);
  if (cVar2 == '\0') {
    FUN_00116760(0);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy_attr_allerror @ 0x7260

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
  FUN_00116760(0,*piVar1,param_10,&local_d8);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_selinux_attr @ 0x7330

bool check_selinux_attr(char *param_1,undefined8 param_2)

{
  int iVar1;
  
  iVar1 = strncmp(param_1,"security.selinux",0x10);
  if (iVar1 == 0) {
    return false;
  }
  iVar1 = attr_copy_check_permissions(param_1,param_2);
  return iVar1 != 0;
}




// Function: restore_default_fscreatecon_or_die @ 0x7380

void restore_default_fscreatecon_or_die(void)

{
  int iVar1;
  undefined8 uVar2;
  uint *puVar3;
  long extraout_RDX;
  ulong uVar4;
  undefined8 uVar5;
  
  iVar1 = setfscreatecon(0);
  if (iVar1 == 0) {
    return;
  }
  uVar2 = dcgettext(0,"failed to restore the default file creation context",5);
  puVar3 = (uint *)__errno_location();
  uVar5 = 1;
  uVar4 = (ulong)*puVar3;
  error(1,uVar4,uVar2);
  uVar2 = FUN_00114a70(1,4,uVar4);
  uVar5 = FUN_00114a70(0,4,uVar5);
  __printf_chk(1,"%s -> %s",uVar5,uVar2);
  if (extraout_RDX != 0) {
    uVar2 = FUN_00114b40(4,extraout_RDX);
    uVar5 = dcgettext(0," (backup: %s)",5);
    __printf_chk(1,uVar5,uVar2);
  }
  putchar_unlocked(10);
  return;
}




// Function: emit_verbose @ 0x73c0

void emit_verbose(undefined8 param_1,undefined8 param_2,long param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_00114a70(1,4,param_2);
  uVar2 = FUN_00114a70(0,4,param_1);
  __printf_chk(1,"%s -> %s",uVar2,uVar1);
  if (param_3 != 0) {
    uVar1 = FUN_00114b40(4,param_3);
    uVar2 = dcgettext(0," (backup: %s)",5);
    __printf_chk(1,uVar2,uVar1);
  }
  putchar_unlocked(10);
  return;
}




// Function: create_hard_link @ 0x7460

byte create_hard_link(void *param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,
                 undefined4 param_5,undefined8 param_6,undefined8 param_7,byte param_8,byte param_9)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  void *__ptr;
  undefined8 uStack_50;
  
  iVar1 = FUN_0010c0c0(param_2,param_3,param_5,param_6,(ulong)param_9 << 10);
  if (iVar1 < 1) {
    param_8 = param_8 & (byte)((uint)iVar1 >> 0x1f);
    if (param_8 == 0) {
      param_8 = 1;
    }
    else {
      uVar3 = FUN_00114b40(4,param_4,0xffffffffffffffff);
      uVar4 = dcgettext(0,"removed %s\n",5);
      __printf_chk(1,uVar4,uVar3);
    }
  }
  else {
    __ptr = (void *)0x0;
    if (param_1 == (void *)0x0) {
      __ptr = (void *)FUN_001065e0(param_4,param_6,param_3,uStack_50);
      param_1 = __ptr;
    }
    uVar3 = FUN_00114a70(1,4,param_1);
    uVar4 = FUN_00114a70(0,4,param_4);
    uVar2 = dcgettext(0,"cannot create hard link %s to %s",5);
    param_8 = 0;
    error(0,iVar1,uVar2,uVar4,uVar3);
    free(__ptr);
  }
  return param_8;
}




// Function: set_process_security_ctx @ 0x7ff0

byte set_process_security_ctx(undefined8 param_1,undefined8 param_2,undefined8 param_3,byte param_4,long param_5
                 )

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  byte bVar6;
  long in_FS_OFFSET;
  undefined8 local_38;
  long local_30;
  
  bVar6 = *(byte *)(param_5 + 0x33);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (bVar6 == 0) {
    bVar6 = *(long *)(param_5 + 0x28) != 0 & param_4;
    if ((bVar6 != 0) && (iVar2 = FUN_0010c760(), iVar2 < 0)) {
      piVar3 = __errno_location();
      cVar1 = FUN_001061d0(*piVar3);
      if (cVar1 == '\0') {
        uVar4 = FUN_00114b40(4,param_2);
        uVar5 = dcgettext(0,"failed to set default file creation context for %s",5);
        error(0,*piVar3,uVar5,uVar4);
        goto LAB_001080aa;
      }
    }
    bVar6 = 1;
    goto LAB_001080aa;
  }
  if ((*(char *)(param_5 + 0x31) == '\0') || (*(char *)(param_5 + 0x34) != '\0')) {
    iVar2 = FUN_001158e0(param_1,&local_38);
    if (-1 < iVar2) {
      iVar2 = setfscreatecon(local_38);
      if (iVar2 < 0) {
        piVar3 = __errno_location();
LAB_001081fd:
        uVar4 = FUN_00114e00(local_38);
        uVar5 = dcgettext(0,"failed to set default file creation context to %s",5);
        error(0,*piVar3,uVar5,uVar4);
        goto LAB_0010812f;
      }
      goto LAB_001081cb;
    }
    piVar3 = __errno_location();
LAB_00108048:
    uVar4 = FUN_00114b40(4,param_1);
    uVar5 = dcgettext(0,"failed to get security context of %s",5);
    error(0,*piVar3,uVar5,uVar4);
LAB_0010807e:
    if (*(char *)(param_5 + 0x34) == '\0') goto LAB_001080aa;
  }
  else {
    if (*(char *)(param_5 + 0x37) != '\0') {
      iVar2 = FUN_001158e0(param_1,&local_38);
      if (-1 < iVar2) {
        iVar2 = setfscreatecon(local_38);
        if (iVar2 < 0) goto LAB_0010812f;
        goto LAB_001081cb;
      }
      goto LAB_0010807e;
    }
    iVar2 = FUN_001158e0(param_1,&local_38);
    if (iVar2 < 0) {
      piVar3 = __errno_location();
      cVar1 = FUN_001061d0(*piVar3);
      if (cVar1 == '\0') goto LAB_00108048;
      goto LAB_0010807e;
    }
    iVar2 = setfscreatecon(local_38);
    if (-1 < iVar2) {
LAB_001081cb:
      freecon(local_38);
      goto LAB_001080aa;
    }
    piVar3 = __errno_location();
    cVar1 = FUN_001061d0(*piVar3);
    if (cVar1 == '\0') goto LAB_001081fd;
LAB_0010812f:
    if (*(char *)(param_5 + 0x34) == '\0') goto LAB_001081cb;
    freecon(local_38);
  }
  bVar6 = 0;
LAB_001080aa:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar6;
}




// Function: set_file_security_ctx @ 0x8250

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
    cVar1 = FUN_0010c960(uVar4,param_1,param_2);
    if (cVar1 == '\0') {
      piVar6 = __errno_location();
      goto LAB_001082d5;
    }
  }
  else {
    if (*(char *)(param_3 + 0x37) != '\0') {
      uVar3 = FUN_0010c960(uVar4,param_1,param_2);
      return uVar3;
    }
    uVar2 = FUN_0010c960(uVar4,param_1,param_2);
    if ((char)uVar2 == '\0') {
      piVar6 = __errno_location();
      cVar1 = FUN_001061d0(*piVar6);
      if (cVar1 != '\0') {
        return (ulong)uVar2;
      }
LAB_001082d5:
      uVar4 = FUN_00114a70(0,4,param_1);
      uVar5 = dcgettext(0,"failed to set the security context of %s",5);
      error(0,*piVar6,uVar5,uVar4);
      return 0;
    }
  }
  return 1;
}




// Function: dest_info_init @ 0x8330

void dest_info_init(long param_1)

{
  long lVar1;
  
  lVar1 = FUN_00111cd0(0x3d,0,FUN_00112540,FUN_00112510,FUN_001125b0);
  *(long *)(param_1 + 0x48) = lVar1;
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117250();
}




// Function: src_info_init @ 0x8370

void src_info_init(long param_1)

{
  long lVar1;
  
  lVar1 = FUN_00111cd0(0x3d,0,FUN_001124f0,FUN_00112510,FUN_001125b0);
  *(long *)(param_1 + 0x50) = lVar1;
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117250();
}




// Function: cp_options_default @ 0x83b0

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




// Function: chown_failure_ok @ 0x8400

byte chown_failure_ok(long param_1)

{
  int iVar1;
  byte bVar2;
  int *piVar3;
  
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  bVar2 = iVar1 == 1 || iVar1 == 0x16;
  if (iVar1 == 1 || iVar1 == 0x16) {
    bVar2 = *(byte *)(param_1 + 0x1a) ^ 1;
  }
  return bVar2;
}




// Function: cached_umask @ 0x8640

__mode_t cached_umask(void)

{
  __mode_t __mask;
  
  if (DAT_00125010 != 0xffffffff) {
    return DAT_00125010;
  }
  __mask = umask(0);
  DAT_00125010 = __mask;
  umask(__mask);
  return __mask;
}




// Function: copy_reg @ 0x8680

uint copy_reg(undefined8 param_1,char *param_2,int param_3,char *param_4,long param_5,
                 uint param_6,uint param_7,char *param_8,__dev_t *param_9)

{
  char cVar1;
  char cVar2;
  char cVar3;
  byte bVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  ulong uVar12;
  ulong uVar13;
  int *piVar14;
  size_t sVar15;
  undefined8 uVar16;
  ssize_t sVar17;
  undefined4 uVar18;
  char *pcVar19;
  uint uVar20;
  void *__ptr;
  long in_FS_OFFSET;
  bool bVar21;
  bool bVar22;
  long local_1d8;
  uint local_1c4;
  int local_1b8;
  char local_1a1;
  void *local_1a0;
  undefined8 local_198;
  __off_t local_190;
  undefined1 local_188 [16];
  undefined1 local_178 [16];
  stat local_168;
  stat local_d8;
  char local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar9 = (uint)param_9[3];
  local_1a0 = (void *)0x0;
  cVar3 = *(char *)(param_5 + 0x31);
  cVar1 = *(char *)(param_5 + 0x35);
  iVar5 = FUN_0010e880(param_1,(ulong)(*(int *)(param_5 + 4) == 2) << 0x11);
  if (iVar5 < 0) {
    uVar7 = 0;
    uVar10 = FUN_00114b40(4,param_1);
    uVar11 = dcgettext(0,"cannot open %s for reading",5);
    piVar14 = __errno_location();
    error(0,*piVar14,uVar11,uVar10);
    goto LAB_0010879f;
  }
  iVar6 = fstat(iVar5,&local_d8);
  if (iVar6 == 0) {
    if ((param_9[1] != local_d8.st_ino) || (*param_9 != local_d8.st_dev)) {
      uVar10 = FUN_00114b40(4,param_1);
      uVar11 = dcgettext(0,"skipping file %s, as it was replaced while being copied",5);
      local_1b8 = 0;
      goto LAB_00108779;
    }
    cVar2 = *param_8;
    piVar14 = __errno_location();
    if (cVar2 != '\0') {
LAB_00108cdb:
      uVar20 = ~param_7 & param_6;
      if ((cVar1 != '\0') && (*(char *)(param_5 + 0x1b) == '\0')) {
        uVar20 = uVar20 | 0x80;
      }
      iVar6 = FUN_00112710(param_3,param_4,0xc1,uVar20);
      local_1b8 = *piVar14;
      bVar4 = -(char)(iVar6 >> 0x1f);
      if ((local_1b8 == 0x11) && (bVar4 != 0)) {
        local_1b8 = 0x11;
        if (*(char *)(param_5 + 0x18) == '\0') {
          sVar17 = readlinkat(param_3,param_4,&local_41,1);
          local_1b8 = 0x11;
          if (-1 < sVar17) {
            if (*(char *)(param_5 + 0x3e) == '\0') {
              uVar10 = FUN_00114b40(4,param_2);
              uVar11 = dcgettext(0,"not writing through dangling symlink %s",5);
              error(0,0,uVar11,uVar10);
              uVar7 = 0;
LAB_001092ad:
              __ptr = (void *)0x0;
              goto LAB_00108788;
            }
            iVar6 = FUN_00112710(param_3,param_4,0x41,uVar20);
            local_1b8 = *piVar14;
            bVar4 = local_1b8 == 0x15 & (byte)((uint)iVar6 >> 0x1f);
            goto LAB_00108da9;
          }
        }
      }
      else {
        bVar4 = bVar4 & local_1b8 == 0x15;
LAB_00108da9:
        if (bVar4 == 0) {
          uVar20 = ~param_6 & uVar20;
          if (-1 < iVar6) goto LAB_001088bc;
        }
        else {
          local_1b8 = 0x15;
          if (*param_2 != '\0') {
            sVar15 = strlen(param_2);
            local_1b8 = (param_2[sVar15 - 1] != '/') + 0x14;
          }
        }
      }
LAB_00108d49:
      uVar10 = FUN_00114b40(4,param_2);
      uVar11 = dcgettext(0,"cannot create regular file %s",5);
      goto LAB_00108779;
    }
    iVar6 = FUN_00112710(param_3,param_4,(-(uint)(cVar3 == '\0') & 0xfffffe00) + 0x201);
    local_1b8 = *piVar14;
    if (iVar6 < 0) {
      if (local_1b8 == 2) goto LAB_00108ca8;
      if (*(char *)(param_5 + 0x16) == '\0') {
        if (*param_8 != '\0') goto LAB_00108cdb;
        goto LAB_00108d49;
      }
      iVar6 = unlinkat(param_3,param_4,0);
      if (iVar6 == 0) {
        if (*(char *)(param_5 + 0x3c) != '\0') {
          uVar10 = FUN_00114b40(4,param_2);
          uVar11 = dcgettext(0,"removed %s\n",5);
          __printf_chk(1,uVar11,uVar10);
        }
LAB_00108ca8:
        if ((*(long *)(param_5 + 0x28) == 0) ||
           (uVar7 = FUN_00107ff0(param_1,param_2,param_6,1,param_5), (char)uVar7 != '\0')) {
          *param_8 = '\x01';
          goto LAB_00108cdb;
        }
        goto LAB_001092ad;
      }
      if (*piVar14 == 2) goto LAB_00108ca8;
      uVar10 = FUN_00114b40(4,param_2);
      uVar11 = dcgettext(0,"cannot remove %s",5);
      __ptr = (void *)0x0;
      error(0,*piVar14,uVar11,uVar10);
      uVar7 = 0;
      goto LAB_00108788;
    }
    if ((*(long *)(param_5 + 0x28) == 0) && (*(char *)(param_5 + 0x33) == '\0')) {
LAB_001088a0:
      if (*param_8 != '\0') goto LAB_00108cdb;
      param_7 = 0;
      uVar20 = 0;
LAB_001088bc:
      local_1c4 = uVar9;
      if (cVar3 == '\0') {
LAB_00108de0:
        if (*(char *)(param_5 + 0x1d) != '\0' || uVar20 != 0) {
          cVar3 = '\0';
          goto LAB_001088d2;
        }
        local_168.st_mode = 0;
        uVar20 = 0;
        if (*(char *)(param_5 + 0x1f) != '\0') {
LAB_00108e06:
          local_188 = FUN_00115960(param_9);
          local_178 = FUN_00115980(param_9);
          iVar8 = FUN_0010e900(iVar6,param_3,param_4,local_188,0);
          if (iVar8 != 0) {
            uVar10 = FUN_00114b40(4,param_2);
            uVar11 = dcgettext(0,"preserving times for %s",5);
            error(0,*piVar14,uVar11,uVar10);
            if (*(char *)(param_5 + 0x32) != '\0') goto LAB_00108eb0;
          }
          goto LAB_00108ad0;
        }
      }
      else {
        if (*(int *)(param_5 + 0x44) != 0) {
          iVar8 = FUN_00106770(iVar6,iVar5);
          if (iVar8 == 0) goto LAB_00108de0;
          if (*(int *)(param_5 + 0x44) == 2) {
            uVar7 = 0;
            uVar10 = FUN_00114a70(1,4,param_1);
            uVar11 = FUN_00114a70(0,4,param_2);
            uVar16 = dcgettext(0,"failed to clone %s from %s",5);
            error(0,*piVar14,uVar16,uVar11,uVar10);
            goto LAB_00108ba8;
          }
        }
LAB_001088d2:
        iVar8 = fstat(iVar6,&local_168);
        if (iVar8 != 0) {
          uVar7 = 0;
          uVar10 = FUN_00114b40(4,param_2);
          uVar11 = dcgettext(0,"cannot fstat %s",5);
          error(0,*piVar14,uVar11,uVar10);
          goto LAB_00108ba8;
        }
        if ((local_168.st_mode != (local_168.st_mode | uVar20)) &&
           (iVar8 = FUN_00107020(iVar6,param_3,param_4), iVar8 != 0)) {
          uVar20 = 0;
        }
        if (cVar3 != '\0') {
          uVar12 = FUN_00107fd0(local_168.st_blksize);
          local_1d8 = 0x200;
          if (local_168.st_blksize - 1U < 0x2000000000000000) {
            local_1d8 = local_168.st_blksize;
          }
          iVar8 = FUN_00106780(iVar5);
          if (iVar8 == 0) {
            uVar10 = FUN_00114b40(4,param_1);
            pcVar19 = "cannot lseek %s";
          }
          else {
            if (((local_168.st_mode & 0xf000) == 0x8000) &&
               ((*(int *)(param_5 + 0xc) == 3 || ((*(int *)(param_5 + 0xc) == 2 && (iVar8 != 1))))))
            {
              posix_fadvise(iVar5,0,0,2);
              if (iVar8 == 3) {
                bVar21 = *(int *)(param_5 + 0x44) != 0;
                uVar18 = *(undefined4 *)(param_5 + 0xc);
                goto LAB_001091e4;
              }
              bVar21 = *(int *)(param_5 + 0x44) != 0;
              bVar22 = *(int *)(param_5 + 0xc) == 3;
LAB_00108a67:
              local_1a1 = '\0';
              cVar3 = FUN_00106a40(iVar5,iVar6,&local_1a0,uVar12,local_1d8,bVar22,bVar21,param_1,
                                   param_2,0xffffffffffffffff,&local_190,&local_1a1);
            }
            else {
              posix_fadvise(iVar5,0,0,2);
              uVar10 = FUN_00107fd0(local_d8.st_blksize);
              uVar13 = FUN_0010dbf0(uVar10,uVar12,0x7fffffffffffffff);
              if (((local_d8.st_mode & 0xf000) == 0x8000) && ((ulong)local_d8.st_size < uVar12)) {
                uVar12 = local_d8.st_size + 1;
              }
              uVar12 = (uVar12 - 1) + uVar13;
              uVar12 = uVar12 - uVar12 % uVar13;
              bVar21 = *(int *)(param_5 + 0x44) != 0;
              if (0 < (long)uVar12) {
                uVar18 = 1;
                if (iVar8 == 3) goto LAB_001091e4;
                local_1d8 = 0;
                bVar22 = *(int *)(param_5 + 0xc) == 3;
                goto LAB_00108a67;
              }
              uVar12 = uVar13;
              if (iVar8 != 3) {
                local_1d8 = 0;
                bVar22 = *(int *)(param_5 + 0xc) == 3;
                goto LAB_00108a67;
              }
              uVar18 = 1;
LAB_001091e4:
              local_1a1 = '\0';
              cVar3 = FUN_00107b50(iVar5,iVar6,&local_1a0,uVar12,local_1d8,local_198,
                                   local_d8.st_size,uVar18,bVar21,param_1,param_2);
            }
            if (cVar3 != '\x01') goto LAB_00108eb0;
            if ((local_1a1 == '\0') || (iVar8 = ftruncate(iVar6,local_190), -1 < iVar8))
            goto LAB_00108ab7;
            uVar10 = FUN_00114b40(4,param_2);
            pcVar19 = "failed to extend %s";
          }
          uVar11 = dcgettext(0,pcVar19,5);
          error(0,*piVar14,uVar11,uVar10);
LAB_00108eb0:
          uVar7 = 0;
          goto LAB_00108ba8;
        }
LAB_00108ab7:
        if (*(char *)(param_5 + 0x1f) != '\0') goto LAB_00108e06;
LAB_00108ad0:
        if (*(char *)(param_5 + 0x1d) != '\0') {
          if ((*(__uid_t *)((long)param_9 + 0x1c) != local_168.st_uid) ||
             (local_168.st_gid != (__gid_t)param_9[4])) {
            uVar7 = 0;
            iVar8 = FUN_00108430(param_5,param_2,param_3,param_4,iVar6,(int)param_9[3],
                                 *(__uid_t *)((long)param_9 + 0x1c),(__gid_t)param_9[4],*param_8,
                                 &local_168);
            if (iVar8 == -1) goto LAB_00108ba8;
            local_1c4 = uVar9 & 0xfffff1ff;
            if (iVar8 != 0) {
              local_1c4 = uVar9;
            }
          }
        }
      }
      if (cVar1 == '\0') {
LAB_00108b54:
        uVar7 = 1;
      }
      else {
        cVar3 = FUN_00107040(param_1,iVar5,param_2,iVar6,param_5);
        if (cVar3 != '\0') goto LAB_00108b54;
        uVar7 = *(byte *)(param_5 + 0x36) ^ 1;
      }
      if ((*(ulong *)(param_5 + 0x18) & 0xff0000000000ff) == 0) {
        if (*(char *)(param_5 + 0x39) == '\0') {
          if ((*(char *)(param_5 + 0x20) == '\0') || (*param_8 == '\0')) {
            if (uVar20 != 0 || param_7 != 0) {
              uVar9 = FUN_00108640();
              if ((~uVar9 & param_7) != 0 || uVar20 != 0) {
                uVar9 = FUN_00108640();
                iVar8 = FUN_00107020(iVar6,param_3,param_4,param_6 & ~uVar9);
                if (iVar8 != 0) {
                  uVar10 = FUN_00114b40(4,param_2);
                  uVar11 = dcgettext(0,"preserving permissions for %s",5);
                  error(0,*piVar14,uVar11,uVar10);
                  if (*(char *)(param_5 + 0x32) != '\0') {
                    uVar7 = 0;
                  }
                }
              }
            }
          }
          else {
            uVar9 = FUN_00108640();
            iVar8 = FUN_0010cb50(param_2,iVar6,~uVar9 & 0x1b6);
            if (iVar8 != 0) {
              uVar7 = 0;
            }
          }
        }
        else {
          iVar8 = FUN_0010cb50(param_2,iVar6,*(undefined4 *)(param_5 + 0x10));
          if (iVar8 != 0) {
            uVar7 = 0;
          }
        }
      }
      else {
        iVar8 = FUN_0010caa0(param_1,iVar5,param_2,iVar6,local_1c4);
        if ((iVar8 != 0) && (*(char *)(param_5 + 0x32) != '\0')) {
          uVar7 = 0;
        }
      }
    }
    else {
      uVar7 = FUN_00108250(param_2,0,param_5);
      if (((char)uVar7 != '\0') || (*(char *)(param_5 + 0x34) == '\0')) goto LAB_001088a0;
    }
LAB_00108ba8:
    iVar6 = close(iVar6);
    __ptr = local_1a0;
    if (iVar6 < 0) {
      uVar7 = 0;
      uVar10 = FUN_00114b40(4,param_2);
      uVar11 = dcgettext(0,"failed to close %s",5);
      error(0,*piVar14,uVar11,uVar10);
      __ptr = local_1a0;
      goto LAB_00108788;
    }
    iVar5 = close(iVar5);
  }
  else {
    uVar10 = FUN_00114b40(4,param_1);
    uVar11 = dcgettext(0,"cannot fstat %s",5);
    piVar14 = __errno_location();
    local_1b8 = *piVar14;
LAB_00108779:
    uVar7 = 0;
    error(0,local_1b8,uVar11,uVar10);
    __ptr = (void *)0x0;
LAB_00108788:
    iVar5 = close(iVar5);
  }
  if (iVar5 < 0) {
    uVar10 = FUN_00114b40(4,param_1);
    uVar11 = dcgettext(0,"failed to close %s",5);
    piVar14 = __errno_location();
    uVar7 = 0;
    error(0,*piVar14,uVar11,uVar10);
  }
  free(__ptr);
LAB_0010879f:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy_internal @ 0x9470

/* WARNING: Removing unreachable block (ram,0x0010a098) */

ulong copy_internal(stat *param_1,stat *param_2,int param_3,stat *param_4,uint param_5,
                  __dev_t *param_6,stat *param_7,int *param_8,undefined4 param_9,char *param_10,
                  undefined1 *param_11,stat *param_12)

{
  undefined1 auVar1 [16];
  timespec tVar2;
  stat *__path;
  __dev_t _Var3;
  __ino_t _Var4;
  __uid_t _Var5;
  __gid_t _Var6;
  char cVar7;
  byte bVar8;
  char cVar9;
  int iVar10;
  uint uVar11;
  long lVar12;
  char *pcVar13;
  undefined8 uVar14;
  int *piVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  ulong uVar18;
  size_t sVar19;
  void *pvVar20;
  stat *psVar21;
  stat *psVar22;
  uint uVar23;
  undefined1 *puVar24;
  undefined1 *puVar25;
  undefined1 *puVar26;
  undefined1 *puVar27;
  undefined1 *puVar28;
  undefined1 *puVar29;
  undefined1 uVar30;
  __mode_t _Var31;
  int iVar32;
  size_t __n;
  byte bVar33;
  stat *unaff_R15;
  long in_FS_OFFSET;
  bool bVar34;
  byte bVar35;
  undefined1 auStack_308 [4];
  uint local_304;
  long local_300;
  uint local_2f8;
  byte local_2f1;
  __dev_t *local_2f0;
  undefined1 *local_2e8;
  char *local_2e0;
  undefined8 *local_2d8;
  stat *local_2d0;
  undefined8 local_2c8;
  undefined8 local_2c0;
  ulong local_2b8;
  stat *local_2b0;
  stat *local_2a8;
  int local_2a0;
  uint local_29c;
  bool local_28a;
  char local_289;
  stat local_288;
  stat local_1f8;
  stat local_168;
  undefined1 local_d8 [152];
  long local_40;
  
  puVar25 = auStack_308;
  puVar24 = auStack_308;
  puVar29 = auStack_308;
  puVar26 = auStack_308;
  puVar27 = auStack_308;
  puVar28 = auStack_308;
  local_2d0 = param_7;
  local_2e0 = param_10;
  local_2b8 = CONCAT44(local_2b8._4_4_,param_9);
  local_2e8 = param_11;
  iVar32 = param_8[0x10];
  local_2c0 = param_12;
  local_2d8 = (undefined8 *)CONCAT71(local_2d8._1_7_,(char)param_9);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  *param_11 = 0;
  local_28a = 0 < (int)param_5;
  local_2f0 = param_6;
  local_2b0 = param_2;
  local_2a8 = param_4;
  local_2a0 = param_3;
  if ((char)param_8[6] == '\0') {
    if (iVar32 != 0) goto LAB_00109560;
LAB_00109531:
    if (*(char *)((long)param_8 + 0x3f) != '\0') {
      iVar32 = 0;
LAB_001095b7:
      if ((byte)local_2b8 != '\0') goto LAB_001095c4;
LAB_00109767:
      local_2f1 = FUN_00107fb0(param_8[1],0);
      if ((int)param_5 < 1) goto LAB_00109858;
      local_2c8 = (stat *)0x0;
      goto LAB_00109668;
    }
    iVar32 = 0;
    psVar22 = local_2a8;
    unaff_R15 = local_2b0;
    iVar10 = local_2a0;
LAB_00109575:
    iVar10 = fstatat(iVar10,(char *)psVar22,&local_288,(uint)(param_8[1] == 2) << 8);
    psVar22 = unaff_R15;
    if (iVar10 == 0) {
      local_29c = local_288.st_mode;
      if (((local_288.st_mode & 0xf000) != 0x4000) ||
         (unaff_R15 = (stat *)0x0, (char)param_8[0xe] != '\0')) goto LAB_001095b7;
      uVar14 = FUN_00114b40(4,param_1);
      unaff_R15 = (stat *)0x0;
      if (*(char *)((long)param_8 + 0x19) != '\0') {
        pcVar13 = "omitting directory %s";
        goto LAB_0010b09c;
      }
      uVar16 = dcgettext(0,"-r not specified; omitting directory %s",5);
      goto LAB_0010a3de;
    }
LAB_0010a440:
    *(undefined8 *)(puVar25 + -8) = 0x10a44a;
    uVar14 = FUN_00114b40(4,psVar22);
    pcVar13 = "cannot stat %s";
LAB_0010a459:
    unaff_R15 = (stat *)0x0;
    *(undefined8 *)(puVar25 + -8) = 0x10a463;
    uVar16 = dcgettext(0,pcVar13,5);
    *(undefined8 *)(puVar25 + -8) = 0x10a46b;
    piVar15 = __errno_location();
    iVar32 = *piVar15;
    *(undefined8 *)(puVar25 + -8) = 0x10a47c;
    error(0,iVar32,uVar16,uVar14);
    goto LAB_00109a3b;
  }
  if (iVar32 < 0) {
    iVar32 = FUN_00114e40(0xffffff9c,param_1,param_3,param_4,1);
    if (iVar32 == 0) {
      local_28a = true;
      param_5 = 1;
      *(char *)&local_2c0->st_dev = '\x01';
      goto LAB_00109531;
    }
    piVar15 = __errno_location();
    iVar32 = *piVar15;
  }
  local_28a = iVar32 == 0;
  *(bool *)&local_2c0->st_dev = local_28a;
  param_5 = (uint)local_28a;
  if (iVar32 == 0) goto LAB_00109531;
LAB_00109560:
  if ((iVar32 != 0x11) || (param_8[2] != 2)) {
    iVar10 = -100;
    psVar22 = param_1;
    unaff_R15 = param_1;
    goto LAB_00109575;
  }
  if ((byte)local_2b8 == '\0') goto LAB_00109767;
LAB_001095c4:
  lVar12 = *(long *)(param_8 + 0x14);
  if (lVar12 != 0) {
    psVar22 = &local_288;
    if (((local_29c & 0xf000) == 0x4000) || (*param_8 != 0)) {
LAB_001095f2:
      FUN_0010ea00(lVar12,param_1,psVar22);
      goto LAB_001095fd;
    }
    local_2c8 = &local_288;
    uVar23 = FUN_0010ea90(lVar12,param_1,&local_288);
    unaff_R15 = (stat *)(ulong)uVar23;
    if ((char)uVar23 == '\0') {
      lVar12 = *(long *)(param_8 + 0x14);
      psVar22 = local_2c8;
      goto LAB_001095f2;
    }
    uVar14 = FUN_00114b40(4,param_1);
    pcVar13 = "warning: source file %s specified more than once";
LAB_0010b09c:
    uVar16 = dcgettext(0,pcVar13,5);
LAB_0010a3de:
    error(0,0,uVar16,uVar14);
    puVar25 = auStack_308;
    goto LAB_00109a3b;
  }
LAB_001095fd:
  local_2f1 = FUN_00107fb0(param_8[1],1);
  if ((int)param_5 < 1) {
LAB_00109858:
    if ((iVar32 == 0x11) && (param_8[2] == 2)) {
      local_289 = '\0';
      cVar7 = '\0';
      goto LAB_00109935;
    }
    uVar23 = local_29c & 0xf000;
    if ((((uVar23 == 0x8000) ||
         (bVar33 = (uVar23 == 0xa000 || uVar23 == 0x4000) | *(byte *)(param_8 + 5) ^ 1, bVar33 == 0)
         ) && (bVar33 = *(byte *)(param_8 + 6), bVar33 == 0)) &&
       ((bVar33 = *(byte *)((long)param_8 + 0x3a), bVar33 == 0 &&
        (bVar33 = *(byte *)((long)param_8 + 0x17), bVar33 == 0)))) {
      if (*param_8 != 0) {
        bVar33 = 1;
        iVar10 = 0x100;
        goto LAB_001098a2;
      }
      bVar33 = *(byte *)((long)param_8 + 0x15);
      if (bVar33 != 0) goto LAB_0010989c;
      iVar10 = 0;
      if (param_5 == 0) goto LAB_001098a2;
LAB_0010a36b:
      local_28a = true;
LAB_0010a372:
      local_2c8 = (stat *)0x0;
      cVar7 = '\0';
      local_2c8._0_1_ = '\0';
      puVar24 = auStack_308;
      if (iVar32 == 0x11) {
        iVar32 = param_8[2];
        goto LAB_001098db;
      }
LAB_0010a38a:
      if ((byte)local_2b8 == '\0') goto LAB_00109668;
      goto LAB_00109627;
    }
LAB_0010989c:
    iVar10 = 0x100;
LAB_001098a2:
    unaff_R15 = &local_1f8;
    local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,bVar33);
    iVar10 = fstatat(local_2a0,(char *)local_2a8,unaff_R15,iVar10);
    if (iVar10 != 0) {
      piVar15 = __errno_location();
      if (*piVar15 == 0x28) {
        if (*(char *)((long)param_8 + 0x16) != '\0') goto LAB_0010a372;
      }
      else if (*piVar15 == 2) goto LAB_0010a36b;
      unaff_R15 = (stat *)0x0;
      uVar14 = FUN_00114b40(4,local_2b0);
      uVar16 = dcgettext(0,"cannot stat %s",5);
      error(0,*piVar15,uVar16,uVar14);
      puVar25 = auStack_308;
      goto LAB_00109a3b;
    }
    iVar32 = param_8[2];
LAB_001098db:
    local_289 = '\0';
    cVar7 = (char)local_2c8;
    if (iVar32 != 2) {
      pcVar13 = &local_289;
      uVar23 = FUN_001075b0(param_1,&local_288,local_2a0,local_2a8,&local_1f8,param_8);
      unaff_R15 = (stat *)(ulong)uVar23;
      cVar7 = (char)local_2c8;
      if ((char)uVar23 != '\0') goto LAB_00109935;
      uVar14 = FUN_00114a70(1,4,local_2b0,pcVar13);
      uVar16 = FUN_00114a70(0,4,param_1);
      pcVar13 = "%s and %s are the same file";
LAB_0010abe0:
      *(undefined8 *)(puVar28 + -8) = 0x10abe7;
      uVar17 = dcgettext(0,pcVar13,5);
      *(undefined8 *)(puVar28 + -8) = 0x10abfb;
      error(0,0,uVar17,uVar16,uVar14);
      puVar25 = puVar28;
      goto LAB_00109a3b;
    }
LAB_00109935:
    if (*(char *)((long)param_8 + 0x3b) == '\0') {
      if ((char)param_8[6] != '\0') goto LAB_0010aa71;
      if ((local_29c & 0xf000) != 0x4000) {
LAB_0010a62c:
        puVar28 = auStack_308;
        if (param_8[2] == 2) goto LAB_00109ab0;
        if (param_8[2] == 3) {
          local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
          cVar9 = FUN_001063e0(param_8,local_2b0,local_2a0,local_2a8,&local_1f8);
          puVar28 = auStack_308;
          cVar7 = (char)local_2c8;
          if (cVar9 == '\0') goto LAB_00109ab0;
        }
      }
LAB_0010a641:
      puVar28 = auStack_308;
      if (local_289 == '\0') {
        unaff_R15 = (stat *)0x0;
        if ((local_1f8.st_mode & 0xf000) == 0x4000) {
          if ((local_29c & 0xf000) == 0x4000) {
LAB_0010a6f4:
            iVar32 = *param_8;
LAB_0010a6f6:
            if ((char)param_8[6] == '\0') {
              _Var31 = local_1f8.st_mode;
              iVar10 = iVar32;
              if (iVar32 != 0) goto LAB_0010b25f;
              goto LAB_0010a70f;
            }
LAB_0010b0d1:
            if ((local_288.st_mode & 0xf000) == 0x4000) {
LAB_0010b487:
              if ((local_1f8.st_mode & 0xf000) != 0x4000) {
                if (iVar32 == 0) {
                  uVar14 = FUN_00114c40(0,3,local_2b0);
                  uVar16 = FUN_00114c40(0,3,param_1);
                  pcVar13 = "cannot move directory onto non-directory: %s -> %s";
                  puVar28 = auStack_308;
                  goto LAB_0010abe0;
                }
                goto LAB_0010aec6;
              }
            }
            if (iVar32 != 0) goto LAB_0010aec6;
          }
          else {
LAB_0010ae9c:
            if (((char)param_8[6] == '\0') || (iVar32 = *param_8, iVar32 == 0)) {
              uVar14 = FUN_00114b40(4,local_2b0);
              pcVar13 = "cannot overwrite directory %s with non-directory";
              goto LAB_0010b09c;
            }
            if ((local_288.st_mode & 0xf000) == 0x4000) goto LAB_0010b487;
LAB_0010aec6:
            local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
            local_300 = CONCAT44(local_300._4_4_,iVar32);
            uVar14 = FUN_0010db60(param_1);
            cVar9 = FUN_00106190(uVar14);
            cVar7 = (char)local_2c8;
            if (cVar9 == '\0') {
              iVar32 = (int)local_300;
LAB_0010aef9:
              if (iVar32 != 3) {
                local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
                cVar9 = FUN_00106640();
                cVar7 = (char)local_2c8;
                if (cVar9 != '\0') {
                  if ((char)param_8[6] == '\0') {
                    uVar14 = dcgettext(0,"backing up %s might destroy source;  %s not copied",5);
                  }
                  else {
                    uVar14 = dcgettext(0,"backing up %s might destroy source;  %s not moved",5);
                  }
                  uVar16 = FUN_00114a70(1,4,param_1);
                  uVar17 = FUN_00114a70(0,4,local_2b0);
                  error(0,0,uVar14,uVar17,uVar16);
                  puVar25 = auStack_308;
                  goto LAB_00109a3b;
                }
              }
              psVar22 = local_2a8;
              local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
              pcVar13 = (char *)FUN_0010da90(local_2a0,local_2a8,*param_8);
              if (pcVar13 == (char *)0x0) {
                local_300 = CONCAT71(local_300._1_7_,(char)local_2c8);
                piVar15 = __errno_location();
                local_2c8 = (stat *)0x0;
                puVar27 = auStack_308;
                cVar7 = (char)local_300;
                if (*piVar15 != 2) {
                  uVar14 = FUN_00114b40(4,local_2b0);
                  pcVar13 = "cannot backup %s";
LAB_0010b95a:
                  uVar16 = dcgettext(0,pcVar13,5);
                  error(0,*piVar15,uVar16,uVar14);
                  puVar25 = auStack_308;
                  goto LAB_00109a3b;
                }
              }
              else {
                __n = (long)psVar22 - (long)local_2b0;
                sVar19 = strlen(pcVar13);
                unaff_R15 = (stat *)(sVar19 + 1);
                uVar18 = __n + 0x18 + sVar19;
                puVar25 = auStack_308;
                while (puVar29 != auStack_308 + -(uVar18 & 0xfffffffffffff000)) {
                  puVar26 = puVar25 + -0x1000;
                  *(undefined8 *)(puVar25 + -8) = *(undefined8 *)(puVar25 + -8);
                  puVar29 = puVar25 + -0x1000;
                  puVar25 = puVar25 + -0x1000;
                }
                uVar18 = (ulong)((uint)uVar18 & 0xff0);
                lVar12 = -uVar18;
                if (uVar18 != 0) {
                  *(undefined8 *)(puVar26 + -8) = *(undefined8 *)(puVar26 + -8);
                }
                local_300 = CONCAT71(local_300._1_7_,(char)local_2c8);
                local_2c8 = (stat *)((ulong)(puVar26 + lVar12 + 0xf) & 0xfffffffffffffff0);
                *(undefined8 *)(puVar26 + lVar12 + -8) = 0x10afea;
                pvVar20 = mempcpy(local_2c8,local_2b0,__n);
                *(undefined8 *)(puVar26 + lVar12 + -8) = 0x10aff8;
                memcpy(pvVar20,pcVar13,(size_t)unaff_R15);
                *(undefined8 *)(puVar26 + lVar12 + -8) = 0x10b000;
                free(pcVar13);
                puVar27 = puVar26 + lVar12;
                cVar7 = (char)local_300;
              }
              local_28a = true;
              goto LAB_0010a7a3;
            }
          }
LAB_0010a798:
          local_2c8 = (stat *)0x0;
          puVar27 = auStack_308;
        }
        else {
          if ((local_29c & 0xf000) == 0x4000) {
            if (((char)param_8[6] == '\0') || (iVar32 = *param_8, iVar32 == 0)) {
              uVar14 = FUN_00114a70(1,4,param_1);
              uVar16 = FUN_00114a70(0,4,local_2b0);
              pcVar13 = "cannot overwrite non-directory %s with directory %s";
              puVar28 = auStack_308;
              goto LAB_0010abe0;
            }
            if ((byte)local_2b8 != '\0') goto LAB_0010a68f;
            goto LAB_0010b0d1;
          }
          iVar32 = *param_8;
          if ((byte)local_2b8 == '\0') goto LAB_0010a6f6;
LAB_0010a68f:
          local_2c8 = (stat *)(CONCAT44(local_2c8._4_4_,local_29c) & 0xffffffff0000f000);
          if (iVar32 != 3) {
            local_300 = CONCAT71(local_300._1_7_,cVar7);
            cVar7 = FUN_0010ea90(*(undefined8 *)(param_8 + 0x12),local_2a8,&local_1f8);
            if (cVar7 != '\0') {
              uVar14 = FUN_00114a70(1,4,param_1);
              uVar16 = FUN_00114a70(0,4,local_2b0);
              pcVar13 = "will not overwrite just-created %s with %s";
              puVar28 = auStack_308;
              goto LAB_0010abe0;
            }
            cVar7 = (char)local_300;
            if (((int)local_2c8 == 0x4000) || ((local_1f8.st_mode & 0xf000) != 0x4000))
            goto LAB_0010a6f4;
            goto LAB_0010ae9c;
          }
          iVar32 = 3;
          iVar10 = 3;
          if ((char)param_8[6] != '\0') goto LAB_0010b0d1;
LAB_0010b25f:
          _Var31 = local_1f8.st_mode;
          local_2c8 = (stat *)CONCAT44(local_2c8._4_4_,iVar10);
          local_300 = CONCAT71(local_300._1_7_,cVar7);
          uVar14 = FUN_0010db60(param_1);
          cVar9 = FUN_00106190(uVar14);
          cVar7 = (char)local_300;
          if (cVar9 == '\0') {
            iVar32 = (int)local_2c8;
            if ((_Var31 & 0xf000) == 0x4000) goto LAB_0010a798;
            goto LAB_0010aef9;
          }
LAB_0010a70f:
          if (((_Var31 & 0xf000) == 0x4000) ||
             ((*(char *)((long)param_8 + 0x15) == '\0' &&
              ((*(char *)((long)param_8 + 0x31) == '\0' ||
               ((((char)param_8[0xc] == '\0' || (local_1f8.st_nlink < 2)) &&
                ((param_8[1] != 2 || ((local_288.st_mode & 0xf000) == 0x8000))))))))))
          goto LAB_0010a798;
          local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
          iVar32 = unlinkat(local_2a0,(char *)local_2a8,0);
          if (iVar32 != 0) {
            piVar15 = __errno_location();
            if (*piVar15 == 2) goto LAB_0010a783;
            uVar14 = FUN_00114b40(4,local_2b0);
            pcVar13 = "cannot remove %s";
            goto LAB_0010b95a;
          }
LAB_0010a783:
          local_28a = true;
          cVar7 = (char)local_2c8;
          if ((char)param_8[0xf] == '\0') goto LAB_0010a798;
          local_300 = CONCAT71(local_300._1_7_,(char)local_2c8);
          uVar14 = FUN_00114b40(4,local_2b0);
          uVar16 = dcgettext(0,"removed %s\n",5);
          __printf_chk(1,uVar16,uVar14);
          local_2c8 = (stat *)0x0;
          cVar7 = (char)local_300;
        }
LAB_0010a7a3:
        iVar32 = 0x11;
        puVar24 = puVar27;
        goto LAB_0010a38a;
      }
      goto LAB_00109ab0;
    }
    if ((local_29c & 0xf000) == 0x4000) {
      if ((char)param_8[6] == '\0') goto LAB_0010a641;
LAB_0010aa71:
      local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
      uVar23 = FUN_00106530(param_8,local_2b0,local_2a0,local_2a8,&local_1f8);
      unaff_R15 = (stat *)(ulong)uVar23;
      cVar7 = (char)local_2c8;
      if ((char)uVar23 == '\0') goto LAB_0010a641;
      puVar28 = auStack_308;
      puVar25 = auStack_308;
      if (local_2c0 == (stat *)0x0) goto LAB_00109ab0;
LAB_0010aaba:
      *(char *)&local_2c0->st_dev = '\x01';
      goto LAB_00109a3b;
    }
    bVar34 = false;
    if ((*(char *)((long)param_8 + 0x1f) != '\0') && (bVar34 = true, (char)param_8[6] != '\0')) {
      bVar34 = local_1f8.st_dev != local_288.st_dev;
    }
    local_2c8 = (stat *)CONCAT71(local_2c8._1_7_,cVar7);
    iVar32 = FUN_00115e70(local_2a0,local_2a8,&local_1f8,&local_288,bVar34);
    psVar22 = local_2a8;
    if (iVar32 < 0) {
      cVar7 = (char)local_2c8;
      if ((char)param_8[6] == '\0') goto LAB_0010a62c;
      goto LAB_0010aa71;
    }
    if (local_2c0 != (stat *)0x0) {
      *(char *)&local_2c0->st_dev = '\x01';
    }
    lVar12 = FUN_0010bf60(local_2a8,local_288.st_ino,local_288.st_dev);
    puVar28 = auStack_308;
    if (lVar12 == 0) goto LAB_00109ab0;
    cVar7 = FUN_00107460(0,local_2a0,lVar12,local_2b0,local_2a0,psVar22,1,(char)param_8[0xf],
                         local_2f1);
    puVar28 = auStack_308;
    if (cVar7 != '\0') goto LAB_00109ab0;
    puVar25 = auStack_308;
    if (*(char *)((long)param_8 + 0x33) != '\0') {
      FUN_00107380();
      puVar25 = auStack_308;
    }
    goto LAB_00109a38;
  }
  local_2c8 = (stat *)0x0;
  cVar7 = '\0';
  puVar24 = auStack_308;
LAB_00109627:
  iVar10 = local_2a0;
  psVar22 = local_2a8;
  if ((*(long *)(param_8 + 0x12) == 0) || (unaff_R15 = (stat *)0x0, (char)param_8[6] != '\0')) {
LAB_00109668:
    puVar25 = puVar24;
    if (((char)param_8[0xf] != '\0') && ((char)param_8[6] == '\0')) {
LAB_00109f88:
      psVar21 = local_2b0;
      psVar22 = local_2c8;
      if ((local_29c & 0xf000) != 0x4000) {
        *(undefined8 *)(puVar25 + -8) = 0x109fb4;
        FUN_001073c0(param_1,psVar21,psVar22);
      }
    }
    if (iVar32 != 0) goto LAB_00109681;
    if ((char)param_8[6] != '\0') goto LAB_00109a66;
LAB_00109ad8:
    local_300 = 0;
LAB_00109aee:
    bVar33 = local_28a;
    uVar11 = local_29c;
    psVar22 = local_2b0;
    uVar23 = local_29c;
    if (*(char *)((long)param_8 + 0x39) != '\0') {
      uVar23 = param_8[4];
    }
    local_2c0 = (stat *)(CONCAT44(local_2c0._4_4_,uVar23) & 0xffffffff00000fff);
    puVar26 = puVar25;
    if (*(char *)((long)param_8 + 0x1d) == '\0') {
      if ((local_29c & 0xf000) == 0x4000) {
        *(undefined8 *)(puVar25 + -8) = 0x10a053;
        cVar7 = FUN_00107ff0(param_1,psVar22,uVar11,bVar33);
        if (cVar7 != '\0') {
          local_2f8 = (uint)local_2c0 & 0x12;
LAB_0010a06a:
          psVar21 = local_2d0;
          uVar18 = (ulong)(uVar11 & 0xf000);
          psVar22 = &local_288;
          *(undefined8 *)(puVar25 + -8) = 0x10a080;
          local_2f1 = FUN_001061e0(psVar22,psVar21);
          if (local_2f1 != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x10ae5d;
            uVar14 = FUN_00114b40(4,param_1);
            pcVar13 = "cannot copy cyclic symbolic link %s";
LAB_0010ae6c:
            *(undefined8 *)(puVar25 + -8) = 0x10ae73;
            uVar16 = dcgettext(0,pcVar13,5);
            *(undefined8 *)(puVar25 + -8) = 0x10ae84;
            error(0,0,uVar16,uVar14);
            goto LAB_0010a558;
          }
          puVar27 = puVar25 + -0x20;
          puVar26 = puVar25 + -0x20;
          *(undefined8 *)(puVar25 + -8) = *(undefined8 *)(puVar25 + -8);
          iVar32 = local_2a0;
          __path = local_2a8;
          psVar21 = local_2b0;
          local_2d8 = (undefined8 *)((ulong)(puVar25 + -0x11) & 0xfffffffffffffff0);
          *local_2d8 = local_2d0;
          local_2d8[1] = local_288.st_ino;
          local_2d8[2] = local_288.st_dev;
          if (((bool)bVar33 != false) || ((local_1f8.st_mode & 0xf000) != 0x4000)) {
            uVar23 = ~local_2f8 & (uint)local_2c0;
            local_2d0 = psVar22;
            *(undefined8 *)(puVar25 + -0x28) = 0x10a12c;
            iVar10 = mkdirat(iVar32,(char *)__path,uVar23);
            iVar32 = local_2a0;
            psVar21 = local_2a8;
            psVar22 = local_2b0;
            if (iVar10 == 0) {
              local_2c0 = local_2d0;
              *(undefined8 *)(puVar25 + -0x28) = 0x10a160;
              iVar10 = fstatat(iVar32,(char *)psVar21,&local_1f8,0x100);
              _Var31 = local_1f8.st_mode;
              iVar32 = local_2a0;
              psVar21 = local_2a8;
              psVar22 = local_2b0;
              if (iVar10 == 0) {
                uVar18 = (ulong)local_1f8.st_mode;
                uVar30 = 0;
                if ((local_1f8.st_mode & 0x1c0) != 0x1c0) {
                  uVar23 = local_1f8.st_mode | 0x1c0;
                  *(undefined8 *)(puVar25 + -0x28) = 0x10a1aa;
                  iVar32 = FUN_0010e520(iVar32,psVar21,uVar23);
                  psVar22 = local_2b0;
                  local_304 = _Var31;
                  uVar30 = 1;
                  if (iVar32 != 0) {
                    *(undefined8 *)(puVar25 + -0x28) = 0x10b76a;
                    uVar14 = FUN_00114b40(4,psVar22);
                    pcVar13 = "setting permissions for %s";
                    puVar26 = puVar25 + -0x20;
                    goto LAB_0010a538;
                  }
                }
                psVar22 = local_2a8;
                psVar21 = local_2c0;
                if (*local_2e0 == '\0') {
                  local_2c8 = local_2c0;
                  local_2c0 = (stat *)CONCAT71(local_2c0._1_7_,uVar30);
                  *(undefined8 *)(puVar25 + -0x28) = 0x10b305;
                  FUN_0010bf60(psVar22,local_1f8.st_ino,local_1f8.st_dev);
                  *local_2e0 = '\x01';
                  psVar21 = local_2c8;
                  uVar30 = (undefined1)local_2c0;
                }
                psVar22 = local_2b0;
                if ((char)param_8[0xf] != '\0') {
                  local_2c0 = (stat *)CONCAT71(local_2c0._1_7_,uVar30);
                  local_2c8 = psVar21;
                  if ((char)param_8[6] == '\0') {
                    *(undefined8 *)(puVar25 + -0x28) = 0x10b4ed;
                    FUN_001073c0(param_1,psVar22,0);
                    uVar30 = (undefined1)local_2c0;
                  }
                  else {
                    *(undefined8 *)(puVar25 + -0x28) = 0x10a205;
                    uVar18 = FUN_00114b40(4,psVar22);
                    *(undefined8 *)(puVar25 + -0x28) = 0x10a21b;
                    uVar14 = dcgettext(0,"created directory %s\n",5);
                    *(undefined8 *)(puVar25 + -0x28) = 0x10a22d;
                    __printf_chk(1,uVar14,uVar18);
                    uVar30 = (undefined1)local_2c0;
                  }
                }
                goto LAB_0010a240;
              }
              *(undefined8 *)(puVar25 + -0x28) = 0x10b3f0;
              uVar14 = FUN_00114b40(4,psVar22);
              pcVar13 = "cannot stat %s";
              puVar26 = puVar25 + -0x20;
            }
            else {
              *(undefined8 *)(puVar25 + -0x28) = 0x10ae39;
              uVar14 = FUN_00114b40(4,psVar22);
              pcVar13 = "cannot create directory %s";
            }
LAB_0010a538:
            *(undefined8 *)(puVar26 + -8) = 0x10a53f;
            uVar16 = dcgettext(0,pcVar13,5);
            *(undefined8 *)(puVar26 + -8) = 0x10a547;
            piVar15 = __errno_location();
            iVar32 = *piVar15;
            *(undefined8 *)(puVar26 + -8) = 0x10a558;
            error(0,iVar32,uVar16,uVar14);
            goto LAB_0010a558;
          }
          if ((*(long *)(param_8 + 10) != 0) || (*(char *)((long)param_8 + 0x33) != '\0')) {
            local_2c0 = psVar22;
            *(undefined8 *)(puVar25 + -0x28) = 0x10b053;
            cVar7 = FUN_00108250(psVar21,0,param_8);
            if ((cVar7 == '\0') && (puVar26 = puVar25 + -0x20, (char)param_8[0xd] != '\0'))
            goto LAB_0010a558;
          }
          local_2f8 = 0;
          uVar30 = 0;
LAB_0010a240:
          bVar35 = local_2f0 != (__dev_t *)0x0 & *(byte *)(param_8 + 7);
          unaff_R15 = (stat *)CONCAT71((int7)(uVar18 >> 8),bVar35);
          if ((bVar35 == 0) || (*local_2f0 == local_288.st_dev)) {
            *(undefined1 **)(puVar25 + -0x28) = local_2e8;
            iVar32 = local_2a0;
            psVar21 = local_2a8;
            *(char **)(puVar25 + -0x30) = local_2e0;
            psVar22 = local_2b0;
            *(int **)(puVar25 + -0x38) = param_8;
            *(undefined8 **)(puVar25 + -0x40) = local_2d8;
            local_2c0 = (stat *)CONCAT71(local_2c0._1_7_,uVar30);
            *(undefined8 *)(puVar25 + -0x48) = 0x10a291;
            uVar23 = FUN_0010bbc0(param_1,psVar22,iVar32,psVar21);
            unaff_R15 = (stat *)(ulong)uVar23;
            uVar30 = (undefined1)local_2c0;
          }
          if ((byte)local_2b8 != '\0') {
            local_2d0 = (stat *)((ulong)local_2d0._1_7_ << 8);
            bVar35 = 0;
            local_2d8 = (undefined8 *)CONCAT71(local_2d8._1_7_,uVar30);
            goto LAB_00109c9d;
          }
          local_2d0 = (stat *)((ulong)local_2d0._1_7_ << 8);
          local_2d8 = (undefined8 *)CONCAT71(local_2d8._1_7_,uVar30);
          puVar25 = puVar25 + -0x20;
          goto LAB_00109d14;
        }
      }
      else {
        *(undefined8 *)(puVar25 + -8) = 0x109b4f;
        cVar7 = FUN_00107ff0(param_1,psVar22,uVar11,bVar33,param_8);
        local_2f8 = 0;
        if (cVar7 != '\0') goto LAB_00109bba;
      }
    }
    else {
      *(undefined8 *)(puVar25 + -8) = 0x109b8f;
      cVar7 = FUN_00107ff0(param_1,psVar22,uVar11);
      if (cVar7 == '\0') goto LAB_00109a38;
      local_2f8 = (uint)local_2c0 & 0x3f;
      if ((uVar11 & 0xf000) == 0x4000) goto LAB_0010a06a;
LAB_00109bba:
      iVar32 = local_2a0;
      psVar21 = local_2a8;
      psVar22 = local_2b0;
      uVar11 = uVar11 & 0xf000;
      local_2d0 = (stat *)CONCAT71(local_2d0._1_7_,*(char *)((long)param_8 + 0x3a));
      if (*(char *)((long)param_8 + 0x3a) != '\0') {
        if ((char)param_1->st_dev != '/') {
          *(undefined8 *)(puVar25 + -8) = 0x109bdf;
          pcVar13 = (char *)FUN_0010e730(psVar21);
          if (local_2a0 == -100) {
            local_2c0 = (stat *)&DAT_0011c7d1;
            *(undefined8 *)(puVar25 + -8) = 0x109c08;
            iVar32 = strcmp(".",pcVar13);
            if (iVar32 != 0) goto LAB_00109c13;
          }
          else {
LAB_00109c13:
            *(undefined8 *)(puVar25 + -8) = 0x109c1f;
            iVar10 = stat(".",&local_168);
            iVar32 = local_2a0;
            if (iVar10 == 0) {
              *(undefined8 *)(puVar25 + -8) = 0x10b10f;
              iVar32 = fstatat(iVar32,pcVar13,(stat *)local_d8,0);
              if ((iVar32 == 0) &&
                 ((local_168.st_ino != local_d8._8_8_ || (local_168.st_dev != local_d8._0_8_)))) {
                *(undefined8 *)(puVar25 + -8) = 0x10b133;
                free(pcVar13);
                psVar22 = local_2b0;
                *(undefined8 *)(puVar25 + -8) = 0x10b146;
                uVar14 = FUN_00114c40(0,3,psVar22);
                pcVar13 = "%s: can make relative symbolic links only in current directory";
                goto LAB_0010ae6c;
              }
            }
          }
          *(undefined8 *)(puVar25 + -8) = 0x109c2f;
          free(pcVar13);
        }
        iVar32 = local_2a0;
        psVar22 = local_2a8;
        uVar30 = *(undefined1 *)((long)param_8 + 0x16);
        *(undefined8 *)(puVar25 + -8) = 0x109c4e;
        iVar32 = FUN_0010c230(param_1,iVar32,psVar22,uVar30,0xffffffff);
        local_2f1 = 0;
        if (iVar32 < 1) goto LAB_00109c60;
        *(undefined8 *)(puVar25 + -8) = 0x10ada2;
        uVar14 = FUN_00114a70(1,4,param_1);
        psVar22 = local_2b0;
        *(undefined8 *)(puVar25 + -8) = 0x10adb8;
        uVar16 = FUN_00114a70(0,4,psVar22);
        *(undefined8 *)(puVar25 + -8) = 0x10adce;
        uVar17 = dcgettext(0,"cannot create symbolic link %s to %s",5);
        *(undefined8 *)(puVar25 + -8) = 0x10ade3;
        error(0,iVar32,uVar17,uVar16,uVar14);
LAB_0010a558:
        puVar25 = puVar26;
        if (*(char *)((long)param_8 + 0x33) != '\0') goto LAB_0010a9d3;
        goto LAB_0010a562;
      }
      if (*(char *)((long)param_8 + 0x17) == '\0') {
        if ((uVar11 == 0x8000) ||
           (local_2f1 = uVar11 != 0xa000 & *(byte *)(param_8 + 5), local_2f1 != 0)) {
          uVar23 = (uint)local_2c0;
          *(stat **)(puVar25 + -0x10) = &local_288;
          psVar22 = local_2b0;
          *(bool **)(puVar25 + -0x18) = &local_28a;
          *(ulong *)(puVar25 + -0x20) = (ulong)local_2f8;
          *(undefined8 *)(puVar25 + -0x28) = 0x10ad60;
          local_2f1 = FUN_00108680(param_1,psVar22,iVar32,psVar21,param_8,uVar23 & 0x1ff);
          if (local_2f1 == 0) goto LAB_0010a558;
          local_2d0 = (stat *)((ulong)local_2d0 & 0xffffffffffffff00);
          bVar33 = local_28a;
        }
        else if (uVar11 == 0x1000) {
          uVar23 = ~local_2f8 & local_29c;
          *(undefined8 *)(puVar25 + -8) = 0x10b605;
          iVar10 = mknodat(iVar32,(char *)psVar21,uVar23,0);
          iVar32 = local_2a0;
          psVar22 = local_2a8;
          if (iVar10 != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x10b625;
            iVar32 = mkfifoat(iVar32,(char *)psVar22,uVar23 & 0xffffefff);
            psVar22 = local_2b0;
            if (iVar32 != 0) {
              *(undefined8 *)(puVar25 + -8) = 0x10b63e;
              uVar14 = FUN_00114b40(4,psVar22);
              pcVar13 = "cannot create fifo %s";
              goto LAB_0010a538;
            }
          }
        }
        else if ((uVar11 >> 8 & 0xbf) == 0x20 || uVar11 == 0xc000) {
          uVar23 = ~local_2f8 & local_29c;
          *(undefined8 *)(puVar25 + -8) = 0x10a50c;
          iVar32 = mknodat(iVar32,(char *)psVar21,uVar23,local_288.st_rdev);
          psVar22 = local_2b0;
          if (iVar32 != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x10a525;
            uVar14 = FUN_00114b40(4,psVar22);
            pcVar13 = "cannot create special file %s";
            goto LAB_0010a538;
          }
        }
        else {
          if (uVar11 != 0xa000) {
            *(undefined8 *)(puVar25 + -8) = 0x10b734;
            uVar14 = FUN_00114b40(4,param_1);
            pcVar13 = "%s has unknown file type";
            goto LAB_0010ae6c;
          }
          *(undefined8 *)(puVar25 + -8) = 0x10b33f;
          pcVar13 = (char *)FUN_0010cbf0(param_1,local_288.st_size);
          iVar32 = local_2a0;
          psVar22 = local_2a8;
          if (pcVar13 == (char *)0x0) {
            *(undefined8 *)(puVar25 + -8) = 0x10b990;
            uVar14 = FUN_00114b40(4,param_1);
            pcVar13 = "cannot read symbolic link %s";
            goto LAB_0010a538;
          }
          uVar30 = *(undefined1 *)((long)param_8 + 0x16);
          *(undefined8 *)(puVar25 + -8) = 0x10b36a;
          iVar32 = FUN_0010c230(pcVar13,iVar32,psVar22,uVar30,0xffffffff);
          local_2c0 = (stat *)CONCAT44(local_2c0._4_4_,iVar32);
          if (0 < iVar32) {
            if (((*(char *)((long)param_8 + 0x3b) == '\x01') && ((bool)bVar33 == false)) &&
               ((local_1f8.st_mode & 0xf000) == 0xa000)) {
              local_2d0 = (stat *)local_1f8.st_size;
              *(undefined8 *)(puVar25 + -8) = 0x10b9d0;
              psVar21 = (stat *)strlen(pcVar13);
              iVar32 = local_2a0;
              psVar22 = local_2a8;
              if (local_2d0 == psVar21) {
                *(undefined8 *)(puVar25 + -8) = 0x10b9f2;
                psVar22 = (stat *)FUN_0010cd80(iVar32,psVar22);
                if (psVar22 != (stat *)0x0) {
                  local_2d0 = psVar22;
                  *(undefined8 *)(puVar25 + -8) = 0x10ba0d;
                  iVar32 = strcmp((char *)psVar22,pcVar13);
                  psVar22 = local_2d0;
                  if (iVar32 == 0) {
                    *(undefined8 *)(puVar25 + -8) = 0x10ba1d;
                    free(psVar22);
                    *(undefined8 *)(puVar25 + -8) = 0x10ba25;
                    free(pcVar13);
                    goto LAB_0010b8b9;
                  }
                  *(undefined8 *)(puVar25 + -8) = 0x10ba87;
                  free(psVar22);
                }
              }
            }
            *(undefined8 *)(puVar25 + -8) = 0x10b38f;
            free(pcVar13);
            psVar22 = local_2b0;
            *(undefined8 *)(puVar25 + -8) = 0x10b3a0;
            uVar14 = FUN_00114b40(4,psVar22);
            *(undefined8 *)(puVar25 + -8) = 0x10b3b6;
            uVar16 = dcgettext(0,"cannot create symbolic link %s",5);
            uVar18 = (ulong)local_2c0 & 0xffffffff;
            *(undefined8 *)(puVar25 + -8) = 0x10b3cb;
            error(0,uVar18,uVar16,uVar14);
            goto LAB_0010a558;
          }
          *(undefined8 *)(puVar25 + -8) = 0x10b8b9;
          free(pcVar13);
LAB_0010b8b9:
          if (*(char *)((long)param_8 + 0x33) != '\0') {
            *(undefined8 *)(puVar25 + -8) = 0x10b97e;
            FUN_00107380();
          }
          _Var6 = local_288.st_gid;
          _Var5 = local_288.st_uid;
          iVar32 = local_2a0;
          psVar22 = local_2a8;
          local_2f1 = *(byte *)((long)param_8 + 0x1d);
          local_2d0 = (stat *)CONCAT71(local_2d0._1_7_,1);
          if (local_2f1 != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x10b8fa;
            iVar32 = FUN_0010e540(iVar32,psVar22,_Var5,_Var6);
            if (iVar32 != 0) {
              *(undefined8 *)(puVar25 + -8) = 0x10b90a;
              cVar7 = FUN_00108400(param_8);
              local_2d0 = (stat *)CONCAT71(local_2d0._1_7_,cVar7);
              if (cVar7 != '\0') {
                local_2f1 = 0;
                goto LAB_00109c60;
              }
              *(undefined8 *)(puVar25 + -8) = 0x10ba3d;
              uVar14 = dcgettext(0,"failed to preserve ownership for %s",5);
              *(undefined8 *)(puVar25 + -8) = 0x10ba45;
              piVar15 = __errno_location();
              psVar22 = local_2b0;
              iVar32 = *piVar15;
              *(undefined8 *)(puVar25 + -8) = 0x10ba5a;
              error(0,iVar32,uVar14,psVar22);
              if (*(char *)((long)param_8 + 0x32) != '\0') goto LAB_0010a558;
            }
            bVar35 = local_2f1;
            local_2f1 = 0;
            local_2d0 = (stat *)CONCAT71(local_2d0._1_7_,bVar35);
          }
        }
      }
      else {
        uVar18 = 1;
        if (*(char *)((long)param_8 + 0x16) == '\0') {
          uVar18 = (ulong)(param_8[2] == 3);
        }
        *(ulong *)(puVar25 + -0x10) = (ulong)local_2f1;
        *(undefined8 *)(puVar25 + -0x18) = 0;
        *(ulong *)(puVar25 + -0x20) = uVar18;
        *(undefined8 *)(puVar25 + -0x28) = 0x10a8b8;
        cVar7 = FUN_00107460(param_1,0xffffff9c,param_1,psVar22,iVar32,psVar21);
        local_2f1 = 0;
        if (cVar7 == '\0') goto LAB_0010a558;
      }
LAB_00109c60:
      psVar22 = local_2b0;
      bVar35 = uVar11 != 0x4000;
      if (((bVar33 == 0) && (bVar8 = (*(byte *)(param_8 + 5) ^ 1) & bVar35, bVar8 != 0)) &&
         ((*(long *)(param_8 + 10) != 0 || (bVar35 = bVar8, *(char *)((long)param_8 + 0x33) != '\0')
          ))) {
        *(undefined8 *)(puVar25 + -8) = 0x10a9fc;
        bVar35 = FUN_00108250(psVar22,0,param_8);
        if ((bVar35 == 0) && (bVar33 = 0, bVar35 = bVar8, (char)param_8[0xd] != '\0'))
        goto LAB_0010a558;
      }
      unaff_R15 = (stat *)0x1;
      puVar27 = puVar25;
      if ((char)local_2b8 != '\0') {
        local_2d8 = (undefined8 *)((ulong)local_2d8 & 0xffffffffffffff00);
        unaff_R15 = (stat *)(local_2b8 & 0xffffffff);
LAB_00109c9d:
        iVar32 = local_2a0;
        psVar22 = local_2a8;
        if (*(long *)(param_8 + 0x12) != 0) {
          local_2b8 = CONCAT71(local_2b8._1_7_,bVar35);
          *(undefined8 *)(puVar27 + -8) = 0x109ccc;
          iVar32 = fstatat(iVar32,(char *)psVar22,(stat *)local_d8,0x100);
          psVar22 = local_2a8;
          bVar35 = (byte)local_2b8;
          if (iVar32 == 0) {
            uVar14 = *(undefined8 *)(param_8 + 0x12);
            *(undefined8 *)(puVar27 + -8) = 0x109ceb;
            FUN_0010ea00(uVar14,psVar22,(stat *)local_d8);
            bVar35 = (byte)local_2b8;
          }
        }
      }
      tVar2.tv_nsec = local_d8._8_8_;
      tVar2.tv_sec = local_d8._0_8_;
      auVar1._8_8_ = local_d8._24_8_;
      auVar1._0_8_ = local_d8._16_8_;
      puVar25 = puVar27;
      if (((*(char *)((long)param_8 + 0x17) != '\0') && (bVar35 != 0)) ||
         (local_d8._16_16_ = auVar1, local_d8._0_16_ = (undefined1  [16])tVar2, local_2f1 != 0))
      goto LAB_00109a3b;
LAB_00109d14:
      if (*(char *)((long)param_8 + 0x1f) != '\0') {
        *(undefined8 *)(puVar25 + -8) = 0x109d2d;
        local_d8._0_16_ = FUN_00115960(&local_288);
        *(undefined8 *)(puVar25 + -8) = 0x109d43;
        local_d8._16_16_ = FUN_00115980(&local_288);
        iVar32 = local_2a0;
        psVar22 = local_2a8;
        uVar23 = (uint)(byte)local_2d0;
        *(undefined8 *)(puVar25 + -8) = 0x109d74;
        iVar32 = utimensat(iVar32,(char *)psVar22,(timespec *)local_d8,uVar23 << 8);
        psVar22 = local_2b0;
        if (iVar32 != 0) {
          *(undefined8 *)(puVar25 + -8) = 0x109d89;
          local_2b8 = FUN_00114b40(4,psVar22);
          *(undefined8 *)(puVar25 + -8) = 0x109da3;
          uVar14 = dcgettext(0,"preserving times for %s",5);
          *(undefined8 *)(puVar25 + -8) = 0x109dab;
          piVar15 = __errno_location();
          uVar18 = local_2b8;
          iVar32 = *piVar15;
          *(undefined8 *)(puVar25 + -8) = 0x109dc0;
          error(0,iVar32,uVar14,uVar18);
          if (*(char *)((long)param_8 + 0x32) != '\0') goto LAB_00109a38;
        }
      }
      psVar22 = local_2b0;
      if ((byte)local_2d0 == '\0') {
        if (*(char *)((long)param_8 + 0x1d) != '\0') {
          if (((bVar33 != 0) || (local_1f8.st_uid != local_288.st_uid)) ||
             (local_1f8.st_gid != local_288.st_gid)) {
            *(stat **)(puVar25 + -8) = &local_1f8;
            *(ulong *)(puVar25 + -0x10) = (ulong)bVar33;
            psVar21 = local_2a8;
            *(ulong *)(puVar25 + -0x18) = (ulong)local_288.st_gid;
            iVar32 = local_2a0;
            *(ulong *)(puVar25 + -0x20) = (ulong)local_288.st_uid;
            *(undefined8 *)(puVar25 + -0x28) = 0x109e38;
            iVar32 = FUN_00108430(param_8,psVar22,iVar32,psVar21,0xffffffff,local_288.st_mode);
            if (iVar32 == -1) goto LAB_00109a38;
            if (iVar32 == 0) {
              local_29c = local_29c & 0xfffff1ff;
            }
          }
        }
        psVar22 = local_2b0;
        if (*(char *)((long)param_8 + 0x35) != '\0') {
          *(undefined8 *)(puVar25 + -8) = 0x10b45f;
          cVar7 = FUN_00107040(param_1,0xffffffff,psVar22,0xffffffff,param_8);
          if ((cVar7 == '\0') && (*(char *)((long)param_8 + 0x36) != '\0')) goto LAB_00109a38;
        }
        uVar23 = local_29c;
        psVar22 = local_2b0;
        if ((*(ulong *)(param_8 + 6) & 0xff0000000000ff) == 0) {
          if (*(char *)((long)param_8 + 0x39) == '\0') {
            if ((bVar33 & *(byte *)(param_8 + 8)) == 0) {
              if (local_2f8 == 0) {
LAB_00109e9a:
                if ((char)local_2d8 == '\0') goto LAB_00109a3b;
              }
              else {
                *(undefined8 *)(puVar25 + -8) = 0x10b65d;
                uVar23 = FUN_00108640();
                iVar32 = local_2a0;
                psVar22 = local_2a8;
                local_2f8 = local_2f8 & ~uVar23;
                if (local_2f8 == 0) goto LAB_00109e9a;
                if ((char)local_2d8 != '\x01') {
                  if (bVar33 != 0) {
                    *(undefined8 *)(puVar25 + -8) = 0x10a42f;
                    iVar32 = fstatat(iVar32,(char *)psVar22,&local_1f8,0x100);
                    psVar22 = local_2b0;
                    if (iVar32 != 0) goto LAB_0010a440;
                  }
                  local_304 = local_1f8.st_mode;
                  if ((local_2f8 & ~local_1f8.st_mode) == 0) goto LAB_00109a3b;
                }
              }
              iVar32 = local_2a0;
              psVar22 = local_2a8;
              uVar23 = local_304 | local_2f8;
              *(undefined8 *)(puVar25 + -8) = 0x109ec5;
              iVar32 = FUN_0010e520(iVar32,psVar22,uVar23);
              psVar22 = local_2b0;
              if (iVar32 == 0) goto LAB_00109a3b;
              *(undefined8 *)(puVar25 + -8) = 0x109ede;
              uVar14 = FUN_00114b40(4,psVar22);
              *(undefined8 *)(puVar25 + -8) = 0x109ef4;
              uVar16 = dcgettext(0,"preserving permissions for %s",5);
              *(undefined8 *)(puVar25 + -8) = 0x109efc;
              piVar15 = __errno_location();
              iVar32 = *piVar15;
              *(undefined8 *)(puVar25 + -8) = 0x109f0d;
              error(0,iVar32,uVar16,uVar14);
              goto LAB_00109f0d;
            }
            uVar23 = 0x1ff;
            if ((local_29c & 0x7000) != 0x4000) {
              uVar23 = 0x1b6;
            }
            *(undefined8 *)(puVar25 + -8) = 0x10b6c2;
            uVar11 = FUN_00108640();
            uVar23 = ~uVar11 & uVar23;
          }
          else {
            uVar23 = param_8[4];
          }
          psVar22 = local_2b0;
          *(undefined8 *)(puVar25 + -8) = 0x10b2d0;
          iVar32 = FUN_0010cb50(psVar22,0xffffffff,uVar23);
          if (iVar32 == 0) goto LAB_00109a3b;
          goto LAB_00109a38;
        }
        *(undefined8 *)(puVar25 + -8) = 0x10b1c0;
        iVar32 = FUN_0010caa0(param_1,0xffffffff,psVar22,0xffffffff,uVar23);
        if (iVar32 == 0) goto LAB_00109a3b;
LAB_00109f0d:
        cVar7 = *(char *)((long)param_8 + 0x32);
      }
      else {
        if (*(char *)((long)param_8 + 0x35) == '\0') goto LAB_00109a3b;
        *(undefined8 *)(puVar25 + -8) = 0x10a906;
        cVar7 = FUN_00107040(param_1,0xffffffff,psVar22,0xffffffff,param_8);
        if (cVar7 != '\0') goto LAB_00109a3b;
        cVar7 = *(char *)((long)param_8 + 0x36);
      }
      if (cVar7 == '\0') goto LAB_00109a3b;
    }
  }
  else {
    if (*param_8 == 0) {
      psVar21 = &local_1f8;
      if (cVar7 == '\0') {
        psVar21 = (stat *)local_d8;
        *(undefined8 *)(puVar24 + -8) = 0x10aae9;
        iVar10 = fstatat(iVar10,(char *)psVar22,psVar21,0x100);
        if (iVar10 != 0) goto LAB_00109668;
      }
      psVar22 = local_2a8;
      if ((psVar21->st_mode & 0xf000) == 0xa000) {
        uVar14 = *(undefined8 *)(param_8 + 0x12);
        *(undefined8 *)(puVar24 + -8) = 0x10aba0;
        cVar7 = FUN_0010ea90(uVar14,psVar22);
        psVar22 = local_2b0;
        if (cVar7 != '\0') {
          *(undefined8 *)(puVar24 + -8) = 0x10abbe;
          uVar14 = FUN_00114a70(1,4,psVar22);
          *(undefined8 *)(puVar24 + -8) = 0x10abd0;
          uVar16 = FUN_00114a70(0,4,param_1);
          pcVar13 = "will not copy %s through just-created symlink %s";
          unaff_R15 = (stat *)0x0;
          puVar28 = puVar24;
          goto LAB_0010abe0;
        }
      }
      goto LAB_00109668;
    }
    puVar25 = puVar24;
    if ((char)param_8[0xf] != '\0') goto LAB_00109f88;
    if (iVar32 == 0) goto LAB_00109ad8;
LAB_00109681:
    _Var4 = local_288.st_ino;
    _Var3 = local_288.st_dev;
    psVar22 = local_2a8;
    puVar28 = puVar25;
    if (((char)param_8[0xe] == '\0') || ((local_29c & 0xf000) != 0x4000)) {
      cVar7 = (char)param_8[6];
      if (cVar7 != '\0') {
        if (local_288.st_nlink == 1) {
          *(undefined8 *)(puVar25 + -8) = 0x10aca3;
          local_300 = FUN_0010bf00(_Var4,_Var3);
LAB_0010a961:
          iVar10 = local_2a0;
          psVar21 = local_2a8;
          psVar22 = local_2b0;
          if (local_300 != 0) {
            if ((local_29c & 0xf000) == 0x4000) goto LAB_001096d5;
            *(ulong *)(puVar25 + -0x10) = (ulong)local_2f1;
            lVar12 = local_300;
            *(ulong *)(puVar25 + -0x18) = (ulong)*(byte *)(param_8 + 0xf);
            *(undefined8 *)(puVar25 + -0x20) = 1;
            *(undefined8 *)(puVar25 + -0x28) = 0x10a9bd;
            cVar7 = FUN_00107460(0,iVar10,lVar12,psVar22,iVar10,psVar21);
            if (cVar7 != '\0') goto LAB_00109ab0;
            goto LAB_0010a9c9;
          }
          goto LAB_00109740;
        }
        if (((char)param_8[0xc] != '\0') && (*(char *)((long)param_8 + 0x17) == '\0'))
        goto LAB_00109f34;
        local_300 = 0;
LAB_001097c5:
        iVar10 = local_2a0;
        psVar22 = local_2a8;
        if (iVar32 == 0x11) {
          *(undefined8 *)(puVar25 + -8) = 0x10a7ca;
          iVar32 = renameat(-100,(char *)param_1,iVar10,(char *)psVar22);
          if (iVar32 != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x10a7d7;
            piVar15 = __errno_location();
            iVar32 = *piVar15;
            if (iVar32 != 0) goto LAB_001097cf;
          }
LAB_00109a66:
          if ((char)param_8[0xf] != '\0') {
            *(undefined8 *)(puVar25 + -8) = 0x10ac23;
            uVar14 = dcgettext(0,"renamed ",5);
            *(undefined8 *)(puVar25 + -8) = 0x10ac32;
            __printf_chk(1,uVar14);
            psVar21 = local_2b0;
            psVar22 = local_2c8;
            *(undefined8 *)(puVar25 + -8) = 0x10ac48;
            FUN_001073c0(param_1,psVar21,psVar22);
          }
          psVar22 = local_2b0;
          if (*(long *)(param_8 + 10) != 0) {
            *(undefined8 *)(puVar25 + -8) = 0x109a8b;
            FUN_00108250(psVar22,1,param_8);
          }
          psVar22 = local_2a8;
          if (local_2c0 != (stat *)0x0) {
            *(char *)&local_2c0->st_dev = '\x01';
          }
          puVar28 = puVar25;
          if (((byte)local_2b8 != '\0') && (*(char *)((long)param_8 + 0x3f) == '\0')) {
            uVar14 = *(undefined8 *)(param_8 + 0x12);
            *(undefined8 *)(puVar25 + -8) = 0x10ab17;
            FUN_0010ea00(uVar14,psVar22,&local_288);
            unaff_R15 = (stat *)(local_2b8 & 0xff);
            goto LAB_00109a3b;
          }
        }
        else {
LAB_001097cf:
          iVar10 = local_2a0;
          psVar21 = local_2a8;
          psVar22 = local_2b0;
          if (iVar32 != 0x16) {
            if (iVar32 != 0x12) {
              *(undefined8 *)(puVar25 + -8) = 0x10a816;
              uVar14 = FUN_00114a70(1,4,psVar22);
              *(undefined8 *)(puVar25 + -8) = 0x10a828;
              uVar16 = FUN_00114a70(0,4,param_1);
              *(undefined8 *)(puVar25 + -8) = 0x10a83e;
              uVar17 = dcgettext(0,"cannot move %s to %s",5);
LAB_0010a84a:
              *(undefined8 *)(puVar25 + -8) = 0x10a853;
              error(0,iVar32,uVar17,uVar16,uVar14);
              *(undefined8 *)(puVar25 + -8) = 0x10a866;
              FUN_0010bea0(local_288.st_ino,local_288.st_dev);
              goto LAB_00109a38;
            }
            uVar23 = local_29c & 0xf000;
            *(undefined8 *)(puVar25 + -8) = 0x109812;
            iVar32 = unlinkat(iVar10,(char *)psVar21,(uint)(uVar23 == 0x4000) << 9);
            if (iVar32 != 0) {
              *(undefined8 *)(puVar25 + -8) = 0x10981b;
              piVar15 = __errno_location();
              psVar22 = local_2b0;
              if (*piVar15 != 2) {
                *(undefined8 *)(puVar25 + -8) = 0x10ab3e;
                uVar14 = FUN_00114a70(1,4,psVar22);
                *(undefined8 *)(puVar25 + -8) = 0x10ab50;
                uVar16 = FUN_00114a70(0,4,param_1);
                *(undefined8 *)(puVar25 + -8) = 0x10ab66;
                uVar17 = dcgettext(0,"inter-device move failed: %s to %s; unable to remove target",5
                                  );
                iVar32 = *piVar15;
                goto LAB_0010a84a;
              }
            }
            if (((char)param_8[0xf] != '\0') && (uVar23 != 0x4000)) {
              *(undefined8 *)(puVar25 + -8) = 0x10aa33;
              uVar14 = dcgettext(0,"copied ",5);
              *(undefined8 *)(puVar25 + -8) = 0x10aa42;
              __printf_chk(1,uVar14);
              psVar21 = local_2b0;
              psVar22 = local_2c8;
              *(undefined8 *)(puVar25 + -8) = 0x10aa58;
              FUN_001073c0(param_1,psVar21,psVar22);
            }
            local_28a = true;
            goto LAB_00109aee;
          }
          *(undefined8 *)(puVar25 + -8) = 0x10acc6;
          uVar14 = FUN_00114a70(1,4,DAT_00125568);
          *(undefined8 *)(puVar25 + -8) = 0x10acdc;
          uVar16 = FUN_00114a70(0,4,DAT_00125570);
          *(undefined8 *)(puVar25 + -8) = 0x10acf2;
          uVar17 = dcgettext(0,"cannot move %s to a subdirectory of itself, %s",5);
          *(undefined8 *)(puVar25 + -8) = 0x10ad06;
          error(0,0,uVar17,uVar16,uVar14);
          *local_2e8 = 1;
        }
LAB_00109ab0:
        unaff_R15 = (stat *)0x1;
        puVar25 = puVar28;
        goto LAB_00109a3b;
      }
      if ((char)param_8[0xc] == '\0') goto LAB_00109ad8;
      if (*(char *)((long)param_8 + 0x17) == '\0') {
LAB_00109f34:
        if ((1 < local_288.st_nlink) ||
           ((((byte)local_2b8 != '\0' && (param_8[1] == 3)) || (local_300 = 0, param_8[1] == 4)))) {
          *(undefined8 *)(puVar25 + -8) = 0x10a95a;
          local_300 = FUN_0010bf60(psVar22,_Var4,_Var3);
          goto LAB_0010a961;
        }
        goto LAB_00109f6b;
      }
      local_300 = 0;
LAB_00109f73:
      goto LAB_00109aee;
    }
    if ((byte)local_2b8 == '\0') {
      *(undefined8 *)(puVar25 + -8) = 0x1096c4;
      local_300 = FUN_0010bf00(_Var4,_Var3);
    }
    else {
      *(undefined8 *)(puVar25 + -8) = 0x10a92f;
      local_300 = FUN_0010bf60(psVar22,_Var4);
    }
    if (local_300 == 0) {
LAB_00109740:
      cVar7 = (char)param_8[6];
LAB_00109f6b:
      if (cVar7 != '\0') goto LAB_001097c5;
      goto LAB_00109f73;
    }
LAB_001096d5:
    iVar10 = local_2a0;
    lVar12 = local_300;
    *(undefined8 *)(puVar25 + -8) = 0x1096ef;
    cVar7 = FUN_00115130(0xffffff9c,param_1,iVar10,lVar12);
    iVar10 = local_2a0;
    psVar22 = local_2a8;
    lVar12 = local_300;
    if (cVar7 == '\0') {
      *(undefined8 *)(puVar25 + -8) = 0x109712;
      cVar7 = FUN_00115130(iVar10,psVar22,iVar10,lVar12);
      psVar21 = local_2a8;
      psVar22 = local_2b0;
      lVar12 = local_300;
      if (cVar7 != '\0') {
        *(undefined8 *)(puVar25 + -8) = 0x10b6de;
        uVar14 = FUN_00114b40(4,DAT_00125570);
        *(undefined8 *)(puVar25 + -8) = 0x10b6f4;
        uVar16 = dcgettext(0,"warning: source directory %s specified more than once",5);
        *(undefined8 *)(puVar25 + -8) = 0x10b705;
        error(0,0,uVar16,uVar14);
        bVar33 = local_2c0 != (stat *)0x0 & *(byte *)(param_8 + 6);
        unaff_R15 = (stat *)CONCAT71((int7)((ulong)unaff_R15 >> 8),bVar33);
        if (bVar33 != 0) goto LAB_0010aaba;
        goto LAB_00109ab0;
      }
      if ((param_8[1] == 4) || ((param_8[1] == 3 && ((byte)local_2b8 != '\0')))) goto LAB_00109740;
      *(undefined8 *)(puVar25 + -8) = 0x10b51e;
      pvVar20 = (void *)FUN_001065e0(psVar22,psVar21,lVar12);
      *(undefined8 *)(puVar25 + -8) = 0x10b533;
      uVar14 = FUN_00114a70(1,4,pvVar20);
      *(undefined8 *)(puVar25 + -8) = 0x10b545;
      uVar16 = FUN_00114a70(0,4,psVar22);
      *(undefined8 *)(puVar25 + -8) = 0x10b55b;
      uVar17 = dcgettext(0,"will not create hard link %s to directory %s",5);
      *(undefined8 *)(puVar25 + -8) = 0x10b56f;
      error(0,0,uVar17,uVar16,uVar14);
      *(undefined8 *)(puVar25 + -8) = 0x10b577;
      free(pvVar20);
    }
    else {
      *(undefined8 *)(puVar25 + -8) = 0x10b206;
      uVar14 = FUN_00114a70(1,4,DAT_00125568);
      *(undefined8 *)(puVar25 + -8) = 0x10b21c;
      uVar16 = FUN_00114a70(0,4,DAT_00125570);
      *(undefined8 *)(puVar25 + -8) = 0x10b232;
      uVar17 = dcgettext(0,"cannot copy a directory, %s, into itself, %s",5);
      *(undefined8 *)(puVar25 + -8) = 0x10b246;
      error(0,0,uVar17,uVar16,uVar14);
      *local_2e8 = 1;
    }
LAB_0010a9c9:
    if (*(char *)((long)param_8 + 0x33) != '\0') {
LAB_0010a9d3:
      *(undefined8 *)(puVar25 + -8) = 0x10a9d8;
      FUN_00107380();
LAB_0010a562:
      if (local_300 == 0) {
        *(undefined8 *)(puVar25 + -8) = 0x10ac8b;
        FUN_0010bea0(local_288.st_ino,local_288.st_dev);
      }
    }
    iVar32 = local_2a0;
    psVar22 = local_2a8;
    if (local_2c8 != (stat *)0x0) {
      pcVar13 = (char *)((long)local_2c8 + ((long)local_2a8 - (long)local_2b0));
      *(undefined8 *)(puVar25 + -8) = 0x10a5a1;
      iVar32 = renameat(iVar32,pcVar13,iVar32,(char *)psVar22);
      psVar22 = local_2b0;
      if (iVar32 != 0) {
        *(undefined8 *)(puVar25 + -8) = 0x10ac61;
        uVar14 = FUN_00114b40(4,psVar22);
        pcVar13 = "cannot un-backup %s";
        goto LAB_0010a459;
      }
      if ((char)param_8[0xf] != '\0') {
        *(undefined8 *)(puVar25 + -8) = 0x10a5c9;
        uVar14 = FUN_00114a70(1,4,psVar22);
        psVar22 = local_2c8;
        *(undefined8 *)(puVar25 + -8) = 0x10a5df;
        uVar16 = FUN_00114a70(0,4,psVar22);
        *(undefined8 *)(puVar25 + -8) = 0x10a5f5;
        uVar17 = dcgettext(0,"%s -> %s (unbackup)\n",5);
        *(undefined8 *)(puVar25 + -8) = 0x10a60a;
        __printf_chk(1,uVar17,uVar16,uVar14);
      }
    }
  }
LAB_00109a38:
  unaff_R15 = (stat *)0x0;
LAB_00109a3b:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return (ulong)unaff_R15 & 0xffffffff;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar25 + -8) = 0x10ba82;
  __stack_chk_fail();
}




// Function: copy @ 0xbae0

void copy(undefined8 param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,
                 undefined4 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined1 local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00106270(param_6);
  if (cVar1 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy");
  }
  local_41 = 0;
  DAT_00125568 = param_2;
  DAT_00125570 = param_1;
  FUN_00109470(param_1,param_2,param_3,param_4,param_5,0,0,param_6,1,&local_41,param_7,param_8);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy_dir @ 0xbbc0

byte copy_dir(undefined8 param_1,long param_2,undefined4 param_3,long param_4,undefined1 param_5
                 ,undefined8 param_6,undefined8 param_7,undefined8 *param_8,byte *param_9,
                 byte *param_10)

{
  byte bVar1;
  byte bVar2;
  char *__ptr;
  size_t sVar3;
  void *__ptr_00;
  void *__ptr_01;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  char *__s;
  long in_FS_OFFSET;
  byte local_fa;
  byte local_f9;
  byte local_9b;
  byte local_9a;
  undefined1 local_99;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  long local_40;
  
  local_98 = *param_8;
  uStack_90 = param_8[1];
  local_88 = param_8[2];
  uStack_80 = param_8[3];
  local_78 = param_8[4];
  uStack_70 = param_8[5];
  local_68 = param_8[6];
  uStack_60 = param_8[7];
  local_58 = param_8[8];
  uStack_50 = param_8[9];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = param_8[10];
  __ptr = (char *)FUN_00115610(param_1,2);
  if (__ptr == (char *)0x0) {
    uVar4 = FUN_00114b40(4,param_1);
    uVar5 = dcgettext(0,"cannot access %s",5);
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    local_fa = 0;
  }
  else {
    if (*(int *)((long)param_8 + 4) == 3) {
      local_98 = CONCAT44(2,(undefined4)local_98);
    }
    if (*__ptr == '\0') {
      local_f9 = 0;
      local_fa = 1;
    }
    else {
      local_f9 = 0;
      local_fa = 1;
      __s = __ptr;
      do {
        __ptr_00 = (void *)FUN_0010ec90(param_1,__s,0);
        __ptr_01 = (void *)FUN_0010ec90(param_2,__s,0);
        local_9a = *param_9;
        bVar2 = FUN_00109470(__ptr_00,__ptr_01,param_3,(long)__ptr_01 + (param_4 - param_2),param_5,
                             param_6,param_7,&local_98,0,&local_9a,&local_9b,&local_99);
        bVar1 = local_9b;
        local_fa = local_fa & bVar2;
        *param_10 = *param_10 | local_9b;
        free(__ptr_01);
        free(__ptr_00);
        if (bVar1 != 0) break;
        local_f9 = local_f9 | local_9a;
        sVar3 = strlen(__s);
        __s = __s + sVar3 + 1;
      } while (*__s != '\0');
    }
    free(__ptr);
    *param_9 = local_f9;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_fa;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: src_to_dest_hash @ 0xbe50

ulong src_to_dest_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: src_to_dest_compare @ 0xbe60

undefined8 src_to_dest_compare(long *param_1,long *param_2)

{
  if (*param_1 != *param_2) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_2[1] >> 8),param_1[1] == param_2[1]);
}




// Function: src_to_dest_free @ 0xbe80

void src_to_dest_free(void *param_1)

{
  free(*(void **)((long)param_1 + 0x10));
  free(param_1);
  return;
}




// Function: forget_created @ 0xbea0

void forget_created(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = 0;
  local_28 = param_1;
  local_20 = param_2;
  lVar1 = FUN_00112350(DAT_00125578,&local_28);
  if (lVar1 != 0) {
    FUN_0010be80(lVar1);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: src_to_dest_lookup @ 0xbf00

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
  lVar1 = FUN_00111a70(DAT_00125578,&local_28);
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




// Function: remember_copied @ 0xbf60

undefined8 remember_copied(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  
  puVar1 = (undefined8 *)FUN_00116e90(0x18);
  uVar2 = FUN_00117230(param_1);
  uVar4 = DAT_00125578;
  *puVar1 = param_2;
  puVar1[2] = uVar2;
  puVar1[1] = param_3;
  puVar3 = (undefined8 *)FUN_001122f0(uVar4,puVar1);
  if (puVar3 != (undefined8 *)0x0) {
    uVar4 = 0;
    if (puVar1 != puVar3) {
      FUN_0010be80(puVar1);
      uVar4 = puVar3[2];
    }
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117250();
}




// Function: hash_init @ 0xbfe0

void hash_init(void)

{
  DAT_00125578 = FUN_00111cd0(0x67,0,FUN_0010be50,FUN_0010be60,FUN_0010be80);
  if (DAT_00125578 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117250();
}




// Function: try_link @ 0xc020

void try_link(char *param_1,int *param_2)

{
  linkat(*param_2,*(char **)(param_2 + 2),param_2[4],param_1,param_2[5]);
  return;
}




// Function: samedir_template @ 0xc040

void * samedir_template(void *param_1,void *param_2)

{
  ulong __size;
  long lVar1;
  undefined8 *puVar2;
  
  lVar1 = FUN_0010db60();
  __size = (lVar1 - (long)param_1) + 9;
  if ((0x100 < __size) && (param_2 = malloc(__size), param_2 == (void *)0x0)) {
    return (void *)0x0;
  }
  puVar2 = mempcpy(param_2,param_1,lVar1 - (long)param_1);
  *puVar2 = DAT_0011d490;
  *(undefined1 *)(puVar2 + 1) = DAT_0011d498;
  return param_2;
}




// Function: try_symlink @ 0xc0a0

void try_symlink(char *param_1,undefined8 *param_2)

{
  symlinkat((char *)*param_2,*(int *)(param_2 + 1),param_1);
  return;
}




// Function: force_linkat @ 0xc0c0

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
    if (iVar1 == 0) goto LAB_0010c193;
    piVar2 = __errno_location();
    param_7 = *piVar2;
  }
  if ((param_6 == '\x01') && (param_7 == 0x11)) {
    __old = (char *)FUN_0010c040(param_4,local_148);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      param_7 = *piVar2;
    }
    else {
      local_168[0] = param_1;
      local_160 = param_2;
      local_158 = param_3;
      local_154 = param_5;
      iVar1 = FUN_00115ba0(__old,0,local_168,FUN_0010c020,6);
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
LAB_0010c193:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_7;
}




// Function: force_symlinkat @ 0xc230

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
    if (iVar1 == 0) goto LAB_0010c2d3;
    piVar2 = __errno_location();
    param_5 = *piVar2;
  }
  if ((param_4 == '\x01') && (param_5 == 0x11)) {
    __old = (char *)FUN_0010c040(param_3,local_148);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      param_5 = *piVar2;
    }
    else {
      local_158 = param_1;
      local_150 = param_2;
      iVar1 = FUN_00115ba0(__old,0,&local_158,FUN_0010c0a0,6);
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
LAB_0010c2d3:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_5;
}




// Function: computecon @ 0xc370

undefined4 computecon(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  short sVar1;
  int iVar2;
  undefined4 uVar3;
  void *__ptr;
  int *piVar4;
  long in_FS_OFFSET;
  undefined8 local_40;
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = 0;
  local_38 = 0;
  __ptr = (void *)FUN_0010e730();
  iVar2 = getcon(&local_40);
  if (-1 < iVar2) {
    iVar2 = FUN_001158c0(__ptr,&local_38);
    if (-1 < iVar2) {
      sVar1 = mode_to_security_class(param_2);
      if (sVar1 != 0) {
        uVar3 = security_compute_create(local_40,local_38,sVar1,param_3);
        goto LAB_0010c3f2;
      }
    }
  }
  uVar3 = 0xffffffff;
LAB_0010c3f2:
  piVar4 = __errno_location();
  iVar2 = *piVar4;
  free(__ptr);
  freecon(local_40);
  freecon(local_38);
  *piVar4 = iVar2;
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: restorecon_private @ 0xc450

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
    goto LAB_0010c615;
  }
  iVar1 = open(param_2,0x20000);
  piVar4 = __errno_location();
  iVar3 = iVar1;
  if (iVar1 == -1) {
    iVar2 = *piVar4;
    if (iVar2 == 0x28) {
      iVar2 = lstat(param_2,&local_d8);
      if (-1 < iVar2) goto LAB_0010c4cd;
      iVar2 = *piVar4;
      lVar6 = 0;
      lVar5 = 0;
    }
    else {
      lVar6 = 0;
      lVar5 = 0;
    }
    goto LAB_0010c59e;
  }
  iVar2 = fstat(iVar1,&local_d8);
  if (iVar2 < 0) {
    lVar6 = 0;
    lVar5 = 0;
    iVar3 = -1;
    goto LAB_0010c6bb;
  }
LAB_0010c4cd:
  iVar2 = selabel_lookup(param_1,&local_e8,param_2,local_d8.st_mode);
  if (iVar2 < 0) {
    iVar2 = *piVar4;
    lVar6 = 0;
    lVar5 = 0;
    if (iVar2 == 2) {
      *piVar4 = 0x3d;
      iVar2 = 0x3d;
    }
LAB_0010c554:
    iVar3 = -1;
    if (iVar1 != -1) {
LAB_0010c55f:
      close(iVar1);
    }
  }
  else {
    lVar5 = context_new(local_e8);
    if (lVar5 == 0) {
      iVar2 = *piVar4;
      lVar6 = 0;
      goto LAB_0010c554;
    }
    if (iVar1 != -1) {
      iVar3 = FUN_00115900(iVar1);
      if (iVar3 < 0) {
        lVar6 = 0;
        iVar3 = -1;
LAB_0010c6bb:
        iVar2 = *piVar4;
      }
      else {
        lVar6 = context_new(local_e0);
        if (lVar6 != 0) {
LAB_0010c531:
          lVar7 = context_type_get(lVar5);
          if (((lVar7 == 0) || (iVar3 = context_type_set(lVar6,lVar7), iVar3 != 0)) ||
             (lVar7 = context_str(lVar6), lVar7 == 0)) {
            iVar2 = *piVar4;
            goto LAB_0010c554;
          }
          if (iVar1 == -1) {
            iVar3 = lsetfilecon(param_2,lVar7);
            iVar2 = *piVar4;
            goto LAB_0010c59e;
          }
          iVar3 = fsetfilecon(iVar1);
          goto LAB_0010c6bb;
        }
        iVar2 = *piVar4;
        iVar3 = -1;
      }
      goto LAB_0010c55f;
    }
    iVar2 = FUN_001158e0(param_2,&local_e0);
    if (iVar2 < 0) {
      iVar2 = *piVar4;
      lVar6 = 0;
    }
    else {
      lVar6 = context_new(local_e0);
      if (lVar6 != 0) goto LAB_0010c531;
      iVar2 = *piVar4;
      iVar3 = -1;
    }
  }
LAB_0010c59e:
  context_free(lVar5);
  context_free(lVar6);
  freecon(local_e8);
  freecon(local_e0);
  *piVar4 = iVar2;
LAB_0010c615:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: defaultcon @ 0xc760

undefined4 defaultcon(undefined8 param_1,char *param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  char *__ptr;
  long in_FS_OFFSET;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = 0;
  local_48 = 0;
  piVar3 = __errno_location();
  if (*param_2 == '/') {
    __ptr = (char *)0x0;
  }
  else {
    param_2 = (char *)FUN_0010e4c0(param_2,2);
    __ptr = param_2;
    if (param_2 == (char *)0x0) {
      iVar1 = *piVar3;
      __ptr = (char *)0x0;
      lVar5 = 0;
      uVar2 = 0xffffffff;
      lVar4 = 0;
      goto LAB_0010c869;
    }
  }
  iVar1 = selabel_lookup(param_1,&local_50,param_2,param_3);
  if (iVar1 < 0) {
    iVar1 = *piVar3;
    if (iVar1 == 2) {
      *piVar3 = 0x3d;
      iVar1 = 0x3d;
      lVar5 = 0;
      lVar4 = 0;
      uVar2 = 0xffffffff;
    }
    else {
      lVar5 = 0;
      lVar4 = 0;
      uVar2 = 0xffffffff;
    }
  }
  else {
    iVar1 = FUN_0010c370(param_2,param_3,&local_48);
    if (iVar1 < 0) {
      iVar1 = *piVar3;
      lVar5 = 0;
      lVar4 = 0;
      uVar2 = 0xffffffff;
    }
    else {
      lVar4 = context_new(local_50);
      if (lVar4 == 0) {
        iVar1 = *piVar3;
        lVar5 = 0;
        uVar2 = 0xffffffff;
      }
      else {
        lVar5 = context_new(local_48);
        if ((((lVar5 == 0) || (lVar6 = context_type_get(lVar4), lVar6 == 0)) ||
            (iVar1 = context_type_set(lVar5,lVar6), iVar1 != 0)) ||
           (lVar6 = context_str(lVar5), lVar6 == 0)) {
          iVar1 = *piVar3;
          uVar2 = 0xffffffff;
        }
        else {
          uVar2 = setfscreatecon(lVar6);
          iVar1 = *piVar3;
        }
      }
    }
  }
LAB_0010c869:
  context_free(lVar4);
  context_free(lVar5);
  freecon(local_50);
  freecon(local_48);
  free(__ptr);
  *piVar3 = iVar1;
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: restorecon @ 0xc960

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
    __ptr = (char *)FUN_0010e4c0(param_2,2);
    if (__ptr == (char *)0x0) {
      uVar5 = 0;
      goto LAB_0010ca20;
    }
    piVar2 = __errno_location();
    param_2 = __ptr;
  }
  if (param_3 != '\0') {
    local_40 = 0;
    iVar7 = 0;
    local_48 = param_2;
    lVar3 = FUN_00117290(&local_48,0x10,0);
    do {
      lVar4 = FUN_00110b30(lVar3);
      while( true ) {
        if (lVar4 == 0) {
          if (*piVar2 != 0) {
            iVar7 = *piVar2;
          }
          iVar1 = FUN_00110940(lVar3);
          if (iVar1 != 0) {
            iVar7 = *piVar2;
          }
          free(__ptr);
          uVar5 = CONCAT71(uVar6,iVar7 == 0);
          goto LAB_0010ca20;
        }
        iVar1 = FUN_0010c450(param_1,*(undefined8 *)(lVar3 + 0x20));
        if (-1 < iVar1) break;
        iVar7 = *piVar2;
        lVar4 = FUN_00110b30(lVar3);
      }
    } while( true );
  }
  iVar1 = FUN_0010c450(param_1,param_2);
  iVar7 = *piVar2;
  uVar5 = CONCAT71(uVar6,iVar1 != -1);
  free(__ptr);
  *piVar2 = iVar7;
LAB_0010ca20:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5 & 0xffffffff;
}




// Function: dir_name @ 0xe730

void dir_name(void)

{
  long lVar1;
  
  lVar1 = FUN_0010e790();
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117250();
}




// Function: is_prime @ 0x113a0

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




// Function: next_prime @ 0x11410

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong uVar2;
  
  uVar2 = 10;
  if (9 < param_1) {
    uVar2 = param_1;
  }
  uVar2 = uVar2 | 1;
  while ((uVar2 != 0xffffffffffffffff && (cVar1 = FUN_001113a0(uVar2), cVar1 == '\0'))) {
    uVar2 = uVar2 + 2;
  }
  return uVar2;
}




// Function: raw_hasher @ 0x11450

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = thunk_FUN_001176e0(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0x11470

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0x11480

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0011d660) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0011d674 < fVar1) && (fVar1 < DAT_0011d678)) && (_DAT_0011d67c < pfVar2[3])) &&
     (_DAT_0011d690 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0011d674;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0011d680)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0011d660;
  return 0;
}




// Function: free_entry @ 0x11510

void free_entry(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x48);
  *param_2 = 0;
  param_2[1] = uVar1;
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: safe_hasher @ 0x11530

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




// Function: hash_find_entry @ 0x11560

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  long lVar5;
  
  plVar4 = (long *)FUN_00111530();
  *param_3 = plVar4;
  lVar5 = *plVar4;
  if (lVar5 == 0) {
LAB_00111601:
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
            FUN_00111510(param_1);
            return lVar5;
          }
        }
        goto LAB_00111601;
      }
      lVar5 = *plVar4;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar4[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar4 = *plVar1;
        plVar4[1] = lVar2;
        FUN_00111510(param_1);
        return lVar5;
      }
      *plVar4 = 0;
    }
  }
  return lVar5;
}




// Function: allocate_entry @ 0x11660

void allocate_entry(long param_1)

{
  if (*(long *)(param_1 + 0x48) != 0) {
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)(*(long *)(param_1 + 0x48) + 8);
    return;
  }
  malloc(0x10);
  return;
}




// Function: transfer_entries @ 0x11690

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
LAB_001116b8:
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
          plVar2 = (long *)FUN_00111530(param_1,lVar6);
          plVar4 = (long *)plVar3[1];
          if (*plVar2 != 0) break;
          *plVar2 = lVar6;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_00111510(param_1);
          plVar3 = plVar4;
          if (plVar4 == (long *)0x0) goto LAB_00111728;
        }
        plVar3[1] = plVar2[1];
        plVar2[1] = (long)plVar3;
        plVar3 = plVar4;
      } while (plVar4 != (long *)0x0);
LAB_00111728:
      lVar6 = *plVar5;
    }
    plVar5[1] = 0;
    if (param_3 != '\0') goto LAB_001116b8;
    plVar3 = (long *)FUN_00111530(param_1,lVar6);
    if (*plVar3 == 0) {
      *plVar3 = lVar6;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    }
    else {
      plVar4 = (long *)FUN_00111660(param_1);
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




// Function: hash_get_n_buckets @ 0x11850

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0x11860

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0x11870

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0x11880

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




// Function: hash_print_statistics @ 0x11950

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_00111870();
  uVar2 = FUN_00111850();
  uVar3 = FUN_00111860();
  uVar4 = FUN_00111880();
  __fprintf_chk(param_2,1,"# entries:         %lu\n",uVar1);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar2);
  __fprintf_chk(((double)uVar3 * _DAT_0011d690) / (double)uVar2,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar3);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: hash_lookup @ 0x11a70

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = (long *)FUN_00111530();
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




// Function: hash_get_next @ 0x11b20

long hash_get_next(long param_1,long param_2)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  
  plVar2 = (long *)FUN_00111530();
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




// Function: hash_do_for_each @ 0x11be0

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




// Function: hash_initialize @ 0x11cd0

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_00111450;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_00111470;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0011d660;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_00111480(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_001117c0(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0x11de0

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
        if ((long *)param_1[1] <= plVar7) goto LAB_00111e7b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_00111e7b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x11e90

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
    if (plVar3 <= plVar5) goto LAB_00111f3c;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_00111eff;
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
LAB_00111eff:
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
LAB_00111f3c:
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




// Function: hash_rehash @ 0x11f80

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
  __nmemb = FUN_001117c0(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_00112086;
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
      uVar2 = FUN_00111690(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_00111690(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_00111690(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_00112086;
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
      goto LAB_00112086;
    }
  }
  uVar2 = 0;
LAB_00112086:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0x12100

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
LAB_001122c7:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  lVar2 = FUN_00111560(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar5 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0011214e;
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
    FUN_00111480(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar8._0_4_ <= auVar7._0_4_ * *(float *)(lVar2 + 8)) goto LAB_001121ae;
    fVar6 = auVar7._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar6 = fVar6 * *(float *)(lVar2 + 8);
    }
    if (fVar6 < _DAT_0011d684) {
      if (_DAT_0011d688 <= fVar6) {
        uVar4 = (long)(fVar6 - _DAT_0011d688) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)fVar6;
      }
      cVar1 = FUN_00111f80(param_1,uVar4);
      if (cVar1 != '\0') {
        lVar2 = FUN_00111560(param_1,param_2,&local_38,0);
        if (lVar2 != 0) goto LAB_001122c7;
        goto LAB_001121ae;
      }
    }
  }
  else {
LAB_001121ae:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar5 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0011214e;
    }
    plVar3 = (long *)FUN_00111660(param_1);
    if (plVar3 != (long *)0x0) {
      *plVar3 = param_2;
      uVar5 = 1;
      plVar3[1] = local_38[1];
      local_38[1] = (long)plVar3;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      goto LAB_0011214e;
    }
  }
  uVar5 = 0xffffffff;
LAB_0011214e:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert @ 0x122f0

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00112100(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0x12350

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
  lVar4 = FUN_00111560(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_00112393;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_001123e0;
LAB_00112496:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_00112496;
LAB_001123e0:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_00111480(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0011d688 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011d688) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_00111f80(param_1,uVar5);
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
LAB_00112393:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}



