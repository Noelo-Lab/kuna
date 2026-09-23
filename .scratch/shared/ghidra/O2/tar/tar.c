// Function: main @ 0xa9f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int main(int param_1,undefined8 *param_2)

{
  long *plVar1;
  undefined8 *puVar2;
  undefined2 uVar3;
  uint uVar4;
  char cVar5;
  int iVar6;
  undefined8 uVar7;
  char *pcVar8;
  size_t sVar9;
  long lVar10;
  char *__s2;
  undefined8 uVar11;
  void *__ptr;
  uint uVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  long *plVar15;
  int *piVar16;
  undefined *puVar18;
  int *piVar19;
  ulong uVar20;
  long in_FS_OFFSET;
  byte bVar21;
  undefined1 auVar22 [16];
  undefined8 uStack_1d8;
  int *local_1d0;
  uint local_1c4;
  undefined8 *local_1c0;
  undefined1 local_1ac [4];
  int local_1a8 [2];
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  int local_188 [2];
  char *local_180;
  undefined8 local_178;
  undefined8 local_170;
  int *local_168;
  undefined8 *local_160;
  undefined2 local_158;
  char local_156;
  char *local_150;
  undefined8 local_148;
  long local_138;
  undefined8 *local_130;
  undefined8 local_128;
  undefined1 local_43;
  char local_42;
  undefined1 local_41;
  long local_40;
  int *piVar17;
  
  bVar21 = 0;
  uVar20 = (ulong)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010c740();
  FUN_0014af40(*param_2);
  setlocale(6,"");
  bindtextdomain("tar","/usr/local/share/locale");
  textdomain("tar");
  DAT_00181e08 = 2;
  DAT_00182d50 = FUN_00110c20;
  DAT_00182d58 = 0;
  FUN_0014c760(0,7);
  uVar7 = dcgettext(0,"stdout",5);
  FUN_00140160(uVar7);
  iVar6 = FUN_0014e3a0();
  if (iVar6 != 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar7 = dcgettext(0,"failed to assert availability of the standard file descriptors",5);
    pcVar8 = "%s";
LAB_0010bc57:
    error(0,0,pcVar8,uVar7);
                    /* WARNING: Subroutine does not return */
    FUN_00116fd0();
  }
  DAT_00182c18 = 10;
  DAT_00182838 = (long *)FUN_0014ecc0(0x50);
  DAT_00182830 = 0;
  signal(0x11,(__sighandler_t)0x0);
  local_168 = local_1a8;
  local_158 = 0;
  local_1a8[0] = 1;
  local_1a0 = 0;
  local_198 = 0;
  local_190 = 0;
  local_160 = (undefined8 *)0x0;
  local_156 = '\0';
  local_150 = (char *)0x0;
  local_148 = 0;
  FUN_0013f7c0("tar",&PTR_s_John_Gilmore_0017fb00);
  local_150 = getenv("SIMPLE_BACKUP_SUFFIX");
  pcVar8 = getenv("POSIXLY_CORRECT");
  DAT_00182ba8 = 0;
  DAT_00182ba4 = 0;
  DAT_00182a38 = pcVar8 != (char *)0x0;
  DAT_00182ba0 = 0x14;
  DAT_00182b98 = 0x2800;
  DAT_00182b60 = FUN_00140920();
  DAT_001827e8 = 1;
  DAT_00182a7c = 0;
  DAT_00182b00 = 0x8000000000000000;
  DAT_00182b08 = -1;
  DAT_00182ae0 = 0x8000000000000000;
  DAT_00182ae8 = -1;
  DAT_00182ad4 = 8;
  DAT_00182a84 = 1;
  DAT_00182a80 = 0;
  DAT_001827e4 = 0;
  DAT_00182abc = 0xffffffff;
  DAT_00182ac0 = 0;
  DAT_00182b54 = 0xffffffff;
  DAT_00182b58 = 0;
  DAT_00182b20 = 1;
  DAT_00182b24 = -1;
  DAT_001827fc = 0xffffffff;
  if ((1 < param_1) && (*(char *)param_2[1] != '-')) {
    local_43 = 0x2d;
    piVar19 = (int *)(param_2 + 2);
    local_41 = 0;
    sVar9 = strlen((char *)param_2[1]);
    param_1 = (int)sVar9 + param_1;
    local_1c4 = param_1 - 1;
    local_1c0 = (undefined8 *)FUN_0014ecc0((long)param_1 * 8);
    pcVar8 = (char *)param_2[1];
    puVar14 = local_1c0 + 1;
    *local_1c0 = *param_2;
    cVar5 = *pcVar8;
    local_1d0 = (int *)(param_2 + uVar20);
    if (cVar5 != '\0') {
      puVar13 = puVar14;
      do {
        while( true ) {
          puVar14 = puVar13 + 1;
          local_42 = cVar5;
          uVar7 = FUN_0014ee30(&local_43);
          puVar18 = PTR_DAT_0017fb20;
          *puVar13 = uVar7;
          lVar10 = FUN_0012d3c0(puVar18,(int)*pcVar8);
          if (lVar10 != 0) break;
          if (PTR_PTR_0017fb40 != (undefined *)0x0) {
            puVar18 = PTR_PTR_0017fb40;
            for (puVar2 = *(undefined8 **)PTR_PTR_0017fb40; puVar2 != (undefined8 *)0x0;
                puVar2 = (undefined8 *)*puVar2) {
              lVar10 = FUN_0012d3c0(*puVar2);
              if (lVar10 != 0) goto LAB_0010b2c8;
              puVar2 = (undefined8 *)(puVar18 + 0x20);
              puVar18 = puVar18 + 0x20;
            }
          }
LAB_0010b35e:
          cVar5 = pcVar8[1];
          pcVar8 = pcVar8 + 1;
          puVar13 = puVar14;
          if (cVar5 == '\0') goto LAB_0010b36d;
        }
LAB_0010b2c8:
        if (*(long *)(lVar10 + 0x10) == 0) goto LAB_0010b35e;
        if (local_1d0 <= piVar19) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          cVar5 = *pcVar8;
          uVar7 = dcgettext(0,"Old option \'%c\' requires an argument.",5);
          error(0,0,uVar7,(int)cVar5);
                    /* WARNING: Subroutine does not return */
          FUN_0012dea0(2);
        }
        uVar7 = *(undefined8 *)piVar19;
        pcVar8 = pcVar8 + 1;
        puVar14 = puVar13 + 2;
        piVar19 = piVar19 + 2;
        puVar13[1] = uVar7;
        cVar5 = *pcVar8;
        puVar13 = puVar14;
      } while (cVar5 != '\0');
    }
LAB_0010b36d:
    piVar16 = piVar19;
    puVar13 = puVar14;
    if (piVar19 < local_1d0) {
      do {
        piVar17 = piVar16 + ((ulong)bVar21 * -2 + 1) * 2;
        *puVar13 = *(undefined8 *)piVar16;
        piVar16 = piVar17;
        puVar13 = puVar13 + (ulong)bVar21 * -2 + 1;
      } while (piVar17 < local_1d0);
      puVar14 = (undefined8 *)
                ((long)puVar14 + ((long)local_1d0 + (-1 - (long)piVar19) & 0xfffffffffffffff8U) + 8)
      ;
    }
    *puVar14 = 0;
    uVar20 = (ulong)local_1c4;
    param_2 = local_1c0;
  }
  pcVar8 = getenv("TAR_OPTIONS");
  local_180 = "TAR_OPTIONS";
  local_188[0] = 0;
  local_178 = 0;
  local_170 = 0;
  if (pcVar8 != (char *)0x0) {
    local_128 = 1;
    iVar6 = FUN_001375a0(pcVar8,&local_138,0x2000e46);
    if (iVar6 != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar7 = FUN_001358f0(&local_138);
      pcVar8 = (char *)dcgettext(0,"cannot split TAR_OPTIONS: %s",5);
      goto LAB_0010bc57;
    }
    if (local_138 != 0) {
      local_1d0 = local_168;
      *local_130 = DAT_00182dd0;
      local_168 = local_188;
      iVar6 = argp_parse(&PTR_DAT_0017fb20,(int)local_138 + (int)local_128,local_130,0x28,local_1ac,
                         &local_168);
      if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      local_168 = local_1d0;
      if (DAT_00182630 != 0) goto LAB_0010bd39;
      local_138 = 0;
    }
    FUN_00135800(&local_138);
  }
  iVar6 = argp_parse(&PTR_DAT_0017fb20,uVar20 & 0xffffffff,param_2,8,local_188,&local_168);
  if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
    exit(2);
  }
  plVar1 = DAT_00182838;
  uVar3 = local_158;
  if ((char)local_158 != '\0') {
    if (DAT_00182ba8 == 3) {
      FUN_0012e430(&DAT_001673ec);
      plVar1 = DAT_00182838;
      uVar3 = local_158;
    }
    else {
      DAT_00182aa8 = 0xffffffff;
    }
  }
  for (; DAT_00182838 = plVar1, local_158 = uVar3, local_188[0] < (int)uVar20;
      local_188[0] = local_188[0] + 1) {
    FUN_00128130(param_2[local_188[0]]);
    plVar1 = DAT_00182838;
    uVar3 = local_158;
  }
  if (DAT_00182ba4 == 0) {
    local_158._1_1_ = (char)((ushort)uVar3 >> 8);
    if (local_158._1_1_ == '\0') {
      DAT_00182ba4 = 6;
    }
    else {
      DAT_00182ba4 = 4;
    }
  }
  if (((((DAT_00182a40 != (char *)0x0) && (DAT_00182ba8 == 3)) || (DAT_00182b50 != '\0')) ||
      ((DAT_00182b10 != '\0' || (DAT_00182a88 != '\0')))) &&
     ((1 << ((byte)DAT_00182ba4 & 0x1f) & 0x54U) == 0)) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "GNU features wanted on incompatible archive format";
    goto LAB_0010bae2;
  }
  if (DAT_00182b38 != 0) {
    if (DAT_00182630 == 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar8 = "--occurrence is meaningless without a file list";
      goto LAB_0010bae2;
    }
    if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 0x10) == 0) {
      if ((DAT_00182bc8 != (int *)0x0) && (*DAT_00182bc8 == 1)) {
        uVar7 = FUN_0012dd30();
        auVar22 = FUN_0012e2f0("--occurrence",uVar7);
        uVar7 = uStack_1d8;
        uStack_1d8 = auVar22._0_8_;
        (*(code *)PTR___libc_start_main_0017efd0)
                  (main,uVar7,&local_1d0,0,0,auVar22._8_8_,&uStack_1d8);
        do {
                    /* WARNING: Do nothing block with infinite loop */
        } while( true );
      }
      DAT_00182b38 = 0;
    }
  }
  if (DAT_00182830 == 0) {
    DAT_00182830 = 1;
    pcVar8 = getenv("TAPE");
    *plVar1 = (long)pcVar8;
    if (*DAT_00182838 == 0) {
      *DAT_00182838 = (long)&DAT_0016b716;
    }
  }
  piVar19 = DAT_00182bd0;
  if ((1 < DAT_00182830) && (DAT_00182b10 == '\0')) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "Multiple archive files require \'-M\' option";
    goto LAB_0010bae2;
  }
  if (DAT_00182b28 != 0) {
    if (DAT_00182b08 < 0) goto LAB_0010aefe;
    iVar6 = FUN_0012dca0(DAT_00182bd0,DAT_00182bd8);
    if (iVar6 == 0) {
      if (*piVar19 == 1) {
        DAT_00182b28 = 0;
        goto LAB_0010aef1;
      }
      DAT_00182b00 = 0;
      DAT_00182b08 = 0;
      goto LAB_0010aefe;
    }
    FUN_0012e2f0("--listed-incremental","--newer");
LAB_0010bdd8:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "Cannot concatenate compressed archives";
    goto LAB_0010bae2;
  }
LAB_0010aef1:
  if (DAT_00182b24 != -1) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar7 = dcgettext(0,"--level is meaningless without --listed-incremental",5);
    error(0,0,uVar7);
  }
LAB_0010aefe:
  pcVar8 = DAT_00182a40;
  if ((DAT_00182a40 != (char *)0x0) && ((DAT_00182ba4 & 0xfffffffb) == 2)) {
    uVar20 = (ulong)(-(uint)(DAT_00182b10 == '\0') & 0x12) + 0x51;
    sVar9 = strlen(DAT_00182a40);
    if (uVar20 < sVar9) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
        pcVar8 = DAT_00182a40;
      }
      uVar7 = FUN_0014ce40(pcVar8);
      uVar11 = dcngettext(0,"%s: Volume label is too long (limit is %lu byte)",
                          "%s: Volume label is too long (limit is %lu bytes)",uVar20,5);
      error(0,0,uVar11,uVar7,uVar20);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
  }
  piVar19 = DAT_00182bf8;
  if (DAT_00182a50 != '\0') {
    if (DAT_00182b10 != '\0') {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar8 = "Cannot verify multi-volume archives";
      goto LAB_0010bae2;
    }
    if (DAT_00182b70 != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar8 = "Cannot verify compressed archives";
      goto LAB_0010bae2;
    }
    if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 2) != 0) goto LAB_0010afcf;
    if ((DAT_00182be0 == (int *)0x0) || (*DAT_00182be0 != 1)) {
      DAT_00182a50 = '\0';
      goto LAB_0010afcf;
    }
    uVar7 = FUN_0012dd30();
    FUN_0012e2f0("--verify",uVar7);
LAB_0010bb91:
    FUN_00132ec0(DAT_00182820);
LAB_0010bb9d:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "Cowardly refusing to create an empty archive";
LAB_0010bae2:
    uVar7 = dcgettext(0,pcVar8,5);
    error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
    FUN_0012dea0(2);
  }
  if (DAT_00182b70 != 0) {
    if (DAT_00182b10 == '\0') {
      if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 4) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar8 = "Cannot update compressed archives";
        goto LAB_0010bae2;
      }
      if (DAT_00182ba8 == 2) goto LAB_0010bdd8;
      goto LAB_0010afcf;
    }
LAB_0010bbee:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "Cannot use multi-volume compressed archives";
    goto LAB_0010bae2;
  }
LAB_0010afcf:
  if ((DAT_00182af0 == 2) && (DAT_00182ae8 < 0)) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "--clamp-mtime needs a date specified using --mtime";
    goto LAB_0010bae2;
  }
  if ((local_158._1_1_ == '\0') || (DAT_00182ba4 == 4)) {
    if (DAT_00182a9c < 1) {
      if (DAT_00182aa0 < 1) {
LAB_0010b41f:
        if ((0 < DAT_00182a98) && (DAT_00182ba4 != 4)) {
          if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) == 0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            pcVar8 = "--xattrs can be used only on POSIX archives";
            goto LAB_0010bae2;
          }
          goto LAB_0010b028;
        }
      }
      else if (DAT_00182ba4 != 4) goto LAB_0010b3ed;
    }
    else {
      if (DAT_00182ba4 != 4) {
        if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) == 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          pcVar8 = "--acls can be used only on POSIX archives";
          goto LAB_0010bae2;
        }
        goto LAB_0010b01b;
      }
      if (DAT_00182aa0 < 1) goto LAB_0010b41f;
    }
    if (DAT_00182a78 == '\0') goto LAB_0010b031;
    if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) != 0) goto LAB_0010b06c;
    if ((DAT_00182be8 == (int *)0x0) || (*DAT_00182be8 != 1)) {
      DAT_00182a78 = '\0';
      if (DAT_00182aac == '\0') goto LAB_0010b06c;
      goto LAB_0010b050;
    }
LAB_0010bdac:
    uVar7 = FUN_0012dd30();
    FUN_0012e2f0("--starting-file",uVar7);
    goto LAB_0010bdc0;
  }
  if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) == 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "--pax-option can be used only on POSIX archives";
    goto LAB_0010bae2;
  }
  if (DAT_00182a9c < 1) {
    if (DAT_00182aa0 < 1) goto LAB_0010b41f;
LAB_0010b3ed:
    if (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) == 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar8 = "--selinux can be used only on POSIX archives";
      goto LAB_0010bae2;
    }
  }
  else {
LAB_0010b01b:
    if (DAT_00182aa0 < 1) goto LAB_0010b41f;
  }
LAB_0010b028:
  if (DAT_00182a78 != '\0') goto LAB_0010b06c;
LAB_0010b031:
  if ((DAT_00182aac != '\0') && (((&DAT_00169a60)[(ulong)DAT_00182ba8 * 4] & 1) == 0)) {
LAB_0010b050:
    if ((DAT_00182bf0 != (int *)0x0) && (*DAT_00182bf0 == 1)) {
      uVar7 = FUN_0012dd30();
      FUN_0012e2f0("--same-order",uVar7);
LAB_0010bd39:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar8 = local_180;
      uVar7 = dcgettext(0,"non-option arguments in %s",5);
      error(0,0,uVar7,pcVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
    DAT_00182aac = '\0';
  }
LAB_0010b06c:
  if (DAT_00182ad0 != '\0') {
    if (DAT_00182b92 != '\0') {
      iVar6 = FUN_0012dca0(DAT_00182bf8,DAT_00182c00);
      if (iVar6 != 0) {
        FUN_0012e2f0("--one-top-level","--absolute-names");
        goto LAB_0010bbee;
      }
      if (*piVar19 != 1) {
        DAT_00182ad0 = '\0';
        goto LAB_0010b0b8;
      }
      DAT_00182b92 = '\0';
    }
    if (DAT_00182ac8 == 0) {
      __ptr = (void *)FUN_00140250(*DAT_00182838);
      DAT_00182ac8 = FUN_0012b8b0(__ptr);
      free(__ptr);
      if (DAT_00182ac8 == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar8 = 
        "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
        goto LAB_0010bae2;
      }
    }
  }
LAB_0010b0b8:
  piVar19 = DAT_00182bf0;
  uVar4 = DAT_00182ba8;
  if (DAT_00182aba != '\0') {
    DAT_00182b34 = 3;
  }
  if (DAT_00182b69 == 0) {
    if (DAT_00182b88 != 2) {
      DAT_00182808 = 0xa0900;
      uVar12 = 0;
      goto LAB_0010b7a6;
    }
    DAT_00182808 = 0xe0900;
    DAT_00182804 = 0xf0900;
    DAT_00182800 = 0x100;
  }
  else {
    DAT_00182808 = 0x80900;
    if (DAT_00182b88 == 2) {
      uVar12 = 0x40000;
LAB_0010b7a6:
      DAT_00182808 = DAT_00182808 | uVar12;
      DAT_00182800 = (DAT_00182b69 ^ 1) << 8;
      DAT_00182804 = DAT_00182808 | 0x10000;
    }
    else {
      DAT_00182804 = 0x90900;
      DAT_00182800 = 0;
    }
  }
  if (DAT_00182ba8 == 9) {
    if (DAT_00182630 == 0) {
      DAT_00182a54 = DAT_00182a54 + 1;
    }
  }
  else if (DAT_00182b91 != '\0') {
    DAT_00182a54 = 2;
  }
  if ((DAT_00182a70 != _DAT_00163748) && (DAT_00182a70 < (double)DAT_00182b98)) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "Volume length cannot be less than record size";
    goto LAB_0010bae2;
  }
  if ((DAT_00182aac != '\0') && (DAT_00182b28 != 0)) {
    iVar6 = FUN_0012dca0(DAT_00182bf0,DAT_00182bd0);
    if (iVar6 != 0) {
      FUN_0012e2f0("--preserve-order","--listed-incremental");
      goto LAB_0010bdac;
    }
    if (*piVar19 == 1) {
      DAT_00182b28 = 0;
    }
    else {
      DAT_00182aac = '\0';
    }
  }
  switch(uVar4) {
  case 1:
  case 2:
  case 8:
    plVar1 = DAT_00182838 + DAT_00182830;
    for (plVar15 = DAT_00182838; DAT_00182828 = plVar15, plVar15 < plVar1; plVar15 = plVar15 + 1) {
      iVar6 = strcmp((char *)*plVar15,"-");
      if (iVar6 == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar8 = "Options \'-Aru\' are incompatible with \'-f -\'";
        goto LAB_0010bae2;
      }
    }
    break;
  case 3:
    if (DAT_00182630 != 0) {
      if ((local_156 != '\0') && (DAT_00182830 != 0)) {
        pcVar8 = (char *)*DAT_00182838;
        iVar6 = strcmp(pcVar8,"-");
        if (iVar6 != 0) {
          FUN_0012b880(pcVar8,DAT_00182b70);
        }
      }
      break;
    }
    goto LAB_0010bb9d;
  case 5:
  case 6:
  case 7:
  case 9:
    plVar1 = DAT_00182838 + DAT_00182830;
    for (plVar15 = DAT_00182838; DAT_00182828 = plVar15, plVar15 < plVar1; plVar15 = plVar15 + 1) {
      iVar6 = strcmp((char *)*plVar15,"-");
      if (iVar6 == 0) {
        FUN_0012df30(&DAT_00166f36);
      }
    }
  }
  if (DAT_00182820 == (char *)0x0) {
    DAT_00181f28 = stdout;
    if (DAT_00182a6a != '\0') {
      DAT_00181f28 = stderr;
    }
  }
  else {
    DAT_00181f28 = fopen(DAT_00182820,"w");
    if (DAT_00181f28 == (FILE *)0x0) goto LAB_0010bb91;
  }
  DAT_00182828 = DAT_00182838;
  if (local_150 != (char *)0x0) {
    DAT_00182db8 = FUN_0014ee30();
  }
  if ((DAT_00182b84 != '\0') &&
     (((DAT_00182b80 = FUN_00140040("--backup",local_148), DAT_00182b80 == 0 ||
       (DAT_00182a6a != '\0')) || (DAT_00182a60 != 0)))) {
    DAT_00182b84 = '\0';
  }
  FUN_00110ba0();
  puVar14 = local_160;
  while (puVar14 != (undefined8 *)0x0) {
    puVar13 = (undefined8 *)*puVar14;
    if (DAT_00182a54 == 0) {
LAB_0010b5a0:
      pcVar8 = (char *)puVar14[4];
    }
    else {
      __s2 = (char *)FUN_00123240(puVar14[1],puVar14[2],1);
      pcVar8 = (char *)puVar14[4];
      iVar6 = strcmp(pcVar8,__s2);
      if (iVar6 != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
          pcVar8 = (char *)puVar14[4];
        }
        uVar7 = puVar14[3];
        uVar11 = dcgettext(0,"Option %s: Treating date \'%s\' as %s",5);
        error(0,0,uVar11,uVar7,pcVar8,__s2);
        goto LAB_0010b5a0;
      }
    }
    free(pcVar8);
    free(puVar14);
    puVar14 = puVar13;
  }
  FUN_00128180();
  if (DAT_00182a48 != 0) {
    FUN_0010dbf0();
  }
  switch(DAT_00182ba8) {
  case 0:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar8 = "You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options";
    goto LAB_0010bae2;
  case 1:
  case 2:
  case 8:
    FUN_00131350();
    break;
  case 3:
    FUN_00116020();
    break;
  case 4:
    FUN_00116780();
    break;
  case 5:
    FUN_001110c0();
    FUN_00123e80(FUN_00111680);
    break;
  case 6:
    FUN_0011a0b0();
    FUN_00123e80(FUN_0011a1e0);
    FUN_0011a080();
    break;
  case 7:
    FUN_00123e80(FUN_00124580);
    break;
  case 9:
    FUN_00124680();
  }
  FUN_00110fa0();
  if (DAT_00182a69 != '\0') {
    FUN_0010cc30();
  }
  if (DAT_00182c20 != 0) {
    FUN_001148b0();
  }
  if (DAT_00182a48 != 0) {
    FUN_0010dcf0();
  }
  free(DAT_00182838);
  FUN_001320a0();
  FUN_001281e0();
  if (DAT_00182d58 == 2) {
    uVar7 = dcgettext(0,"Exiting with failure status due to previous errors",5);
    error(0,0,uVar7);
  }
  if (DAT_00181f28 == stdout) {
    FUN_00140180();
  }
  else if ((((stderr->_flags & 0x20) != 0) || (iVar6 = fclose(stderr), iVar6 != 0)) &&
          (DAT_00182d58 < 2)) {
    DAT_00182d58 = 2;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00182d58;
  }
LAB_0010bdc0:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_stats @ 0xbf60

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
  dVar1 = DAT_00163758;
  if ((DAT_00181f78 <= _DAT_00163748) || (dVar7 = param_1 / DAT_00181f78, DAT_00163750 <= dVar7)) {
    puVar6 = &DAT_0016acf7;
  }
  else {
    if (DAT_00163758 <= dVar7) {
      uVar5 = (long)(dVar7 - DAT_00163758) ^ 0x8000000000000000;
    }
    else {
      uVar5 = (ulong)dVar7;
    }
    puVar6 = (undefined *)FUN_00144440(uVar5,local_2b8,0x1b0,1,1);
  }
  if (dVar1 <= param_1) {
    uVar5 = (long)(param_1 - dVar1) ^ 0x8000000000000000;
  }
  else {
    uVar5 = (ulong)param_1;
  }
  uVar4 = FUN_00144440(uVar5,local_548,0x1b0,1,1);
  iVar2 = __fprintf_chk(param_1,param_2,1,"%.0f (%s, %s/s)",uVar4,puVar6);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2 + iVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: init_buffer @ 0xc100

void init_buffer(void)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  
  lVar3 = (long)DAT_00182178;
  lVar1 = *(long *)(&DAT_00182180 + lVar3 * 8);
  if (lVar1 == 0) {
    uVar2 = FUN_001265a0(&DAT_00182190 + lVar3,DAT_00182b98);
    *(undefined8 *)(&DAT_00182180 + lVar3 * 8) = uVar2;
    lVar1 = *(long *)(&DAT_00182180 + (long)DAT_00182178 * 8);
  }
  DAT_00181f58 = lVar1;
  DAT_00181f48 = lVar1;
  DAT_00181f50 = lVar1 + (long)DAT_00182ba0 * 0x200;
  return;
}




// Function: _flush_write @ 0xc180

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong _flush_write(void)

{
  long *plVar1;
  long lVar2;
  ulong uVar3;
  int *piVar4;
  ulong uVar5;
  ulong uVar6;
  long lVar7;
  long *plVar8;
  long *plVar9;
  
  FUN_00110cf0(1);
  if ((DAT_00182a70 != _DAT_00163748) && (DAT_00182a70 <= DAT_001821a0)) {
    piVar4 = __errno_location();
    *piVar4 = 0x1c;
    return 0;
  }
  uVar5 = DAT_00182b98;
  if (DAT_00182a30 == '\0') {
    uVar5 = FUN_0012c140();
  }
  plVar9 = DAT_00182148;
  if (((uVar5 != 0) && (DAT_00182b10 != '\0')) && (plVar1 = DAT_00182148, DAT_00182138 == 0)) {
    do {
      plVar8 = plVar1;
      if (plVar8 == (long *)0x0) {
        return uVar5;
      }
      plVar1 = (long *)*plVar8;
    } while ((plVar1 != (long *)0x0) && ((ulong)(plVar1[1] << 9) <= uVar5));
    lVar2 = plVar8[1];
    uVar6 = uVar5 + lVar2 * -0x200;
    uVar3 = plVar8[4];
    lVar7 = (uVar6 >> 9) + plVar8[5];
    if (uVar3 <= uVar6) {
      uVar6 = uVar3;
    }
    plVar8[5] = lVar7;
    plVar8[4] = uVar3 - uVar6;
    lVar7 = lVar7 + lVar2;
    if (uVar3 - uVar6 != 0) {
      lVar7 = lVar2;
      plVar1 = plVar8;
    }
    if ((plVar1 != plVar9) && (plVar8 = plVar9, plVar9 != (long *)0x0)) {
      do {
        plVar9 = (long *)*plVar8;
        free((void *)plVar8[2]);
        free(plVar8);
        if (plVar9 == (long *)0x0) break;
        plVar8 = plVar9;
      } while (plVar1 != plVar9);
    }
    DAT_00182148 = plVar9;
    if (plVar9 == (long *)0x0) {
      DAT_00182140 = 0;
    }
    for (; plVar1 != (long *)0x0; plVar1 = (long *)*plVar1) {
      plVar1[1] = plVar1[1] - lVar7;
      plVar1[5] = 0;
    }
  }
  return uVar5;
}




// Function: add_chunk_header @ 0xc400

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
  if (DAT_00182ba4 == 4) {
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
    local_1d8[0] = (void *)FUN_0011cdf0(local_1d8,"%d/GNUFileParts/%f.%n",(long)DAT_0017f014);
    local_150 = *(undefined8 *)(param_1 + 0x20);
    local_1d8[1] = local_1d8[0];
    local_c0 = local_150;
    lVar2 = FUN_00113570(local_1d8);
    if (lVar2 == 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    uVar1 = FUN_00113200(0,local_1d8,lVar2);
    FUN_00113230(uVar1);
    free(local_1d8[0]);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: buffer_write_global_xheader @ 0xc660

void buffer_write_global_xheader(void)

{
  FUN_0011dd60(&DAT_001820e8);
  return;
}




// Function: mv_begin_read @ 0xc670

void mv_begin_read(undefined8 *param_1)

{
  if (DAT_00182b10 == '\0') {
    return;
  }
  FUN_0010c500(*param_1,param_1[0x11],param_1[0x11]);
  return;
}




// Function: mv_end @ 0xc6a0

void mv_end(void)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  
  __ptr = DAT_00182148;
  if (DAT_00182b10 != '\0') {
    while (__ptr != (undefined8 *)0x0) {
      puVar1 = (undefined8 *)*__ptr;
      free((void *)__ptr[2]);
      free(__ptr);
      __ptr = puVar1;
    }
    DAT_00182148 = (undefined8 *)0x0;
    DAT_00182140 = 0;
    return;
  }
  return;
}




// Function: mv_size_left @ 0xc710

void mv_size_left(undefined8 param_1)

{
  if (DAT_00182148 != 0) {
    *(undefined8 *)(DAT_00182148 + 0x20) = param_1;
  }
  return;
}




// Function: clear_read_error_count @ 0xc730

void clear_read_error_count(void)

{
  DAT_00182164 = 0;
  return;
}




// Function: set_start_time @ 0xc740

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void set_start_time(void)

{
  FUN_00143220(&DAT_00182a20);
  DAT_00182a10 = DAT_00182a20;
  DAT_00182a18 = DAT_00182a28;
  _DAT_00182a00 = DAT_00182a20;
  _DAT_00182a08 = DAT_00182a28;
  return;
}




// Function: compute_duration @ 0xc770

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

double compute_duration(void)

{
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00143220(&local_28);
  DAT_00181f78 = (double)(local_20 - _DAT_00182a08) / _DAT_00163760 +
                 (double)(local_28 - _DAT_00182a00) + DAT_00181f78;
  FUN_00143220(&DAT_00182a00);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00181f78;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: first_decompress_program @ 0xc810

undefined * first_decompress_program(int *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  long lVar5;
  
  iVar1 = DAT_00181f70;
  if ((DAT_00182b70 == (undefined *)0x0) && (DAT_00181f70 != 0)) {
    *param_1 = 0;
    if (DAT_0017f020 == 0) {
      iVar2 = 0;
    }
    else {
      piVar3 = &DAT_0017f038;
      iVar2 = 0;
      iVar4 = DAT_0017f020;
      do {
        lVar5 = (long)iVar2;
        iVar2 = iVar2 + 1;
        if (iVar1 == iVar4) {
          *param_1 = iVar2;
          return (&PTR_s_compress_001674d8_8_0017f028)[lVar5 * 3];
        }
        iVar4 = *piVar3;
        piVar3 = piVar3 + 6;
      } while (iVar4 != 0);
    }
    *param_1 = iVar2;
    return (undefined *)0x0;
  }
  return DAT_00182b70;
}




// Function: next_decompress_program @ 0xc890

undefined * next_decompress_program(uint *param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  int *piVar5;
  
  if (DAT_00182b70 != 0) {
    return (undefined *)0x0;
  }
  uVar3 = *param_1;
  uVar4 = (ulong)(int)uVar3;
  piVar5 = &DAT_0017f020 + uVar4 * 6;
  iVar1 = *piVar5;
  while( true ) {
    if (iVar1 == 0) {
      *param_1 = uVar3;
      return (undefined *)0x0;
    }
    iVar2 = (int)uVar4;
    uVar3 = iVar2 + 1;
    uVar4 = (ulong)uVar3;
    if (DAT_00181f70 == iVar1) break;
    iVar1 = piVar5[6];
    piVar5 = piVar5 + 6;
  }
  *param_1 = uVar3;
  return (&PTR_s_compress_001674d8_8_0017f028)[(long)iVar2 * 3];
}




// Function: format_total_stats @ 0xc900

int format_total_stats(_IO_FILE *param_1,undefined8 *param_2,byte param_3,uint param_4)

{
  byte *pbVar1;
  char *pcVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  double dVar7;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182ba8 < 8) {
    if (4 < DAT_00182ba8) {
      uVar6 = DAT_00181f38 * DAT_00182b98;
      uVar5 = dcgettext(0,*param_2,5);
      iVar3 = FUN_0010bf60((double)uVar6,param_1,uVar5);
      goto joined_r0x0010cac6;
    }
    if (DAT_00182ba8 == 4) {
      iVar3 = FUN_0010bf60((double)(ulong)(DAT_00181f38 * DAT_00182b98),param_1,*param_2);
      pbVar1 = (byte *)param_1->_IO_write_ptr;
      if (pbVar1 < param_1->_IO_write_end) {
        param_1->_IO_write_ptr = (char *)(pbVar1 + 1);
        *pbVar1 = param_3;
      }
      else {
        __overflow(param_1,(uint)param_3);
      }
      iVar4 = FUN_0010bf60(DAT_001821a8 + DAT_001821a0,param_1,param_2[1]);
      pbVar1 = (byte *)param_1->_IO_write_ptr;
      if (pbVar1 < param_1->_IO_write_end) {
        param_1->_IO_write_ptr = (char *)(pbVar1 + 1);
        *pbVar1 = param_3;
      }
      else {
        __overflow(param_1,(uint)param_3);
      }
      pcVar2 = (char *)param_2[2];
      iVar3 = iVar3 + iVar4 + 2;
      if ((pcVar2 != (char *)0x0) && (*pcVar2 != '\0')) {
        uVar5 = dcgettext(0,pcVar2,5);
        iVar4 = __fprintf_chk(param_1,1,"%s: ",uVar5);
        iVar3 = iVar3 + iVar4;
      }
      dVar7 = (double)(ulong)((DAT_00181f38 - DAT_001822d8) * DAT_00182b98) -
              (DAT_001821a8 + DAT_001821a0);
      if (DAT_00163758 <= dVar7) {
        uVar6 = (long)(dVar7 - DAT_00163758) ^ 0x8000000000000000;
      }
      else {
        uVar6 = (ulong)dVar7;
      }
      uVar5 = FUN_00145370(uVar6,local_48);
      iVar4 = __fprintf_chk(param_1,1,"%s",uVar5);
      iVar3 = iVar3 + iVar4;
      goto joined_r0x0010cac6;
    }
    if (DAT_00182ba8 == 0) goto LAB_0010cad4;
  }
  else if (DAT_00182ba8 != 8) {
LAB_0010cad4:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  iVar3 = FUN_0010bf60(DAT_001821a8 + DAT_001821a0,param_1,param_2[1]);
joined_r0x0010cac6:
  if (param_4 != 0) {
    pcVar2 = param_1->_IO_write_ptr;
    if (pcVar2 < param_1->_IO_write_end) {
      param_1->_IO_write_ptr = pcVar2 + 1;
      *pcVar2 = (char)param_4;
    }
    else {
      __overflow(param_1,param_4 & 0xff);
    }
    iVar3 = iVar3 + 1;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_total_stats @ 0xcc30

void print_total_stats(void)

{
  FUN_0010c900(stderr,&PTR_s_Total_bytes_read_0017d140,10,10);
  return;
}




// Function: reset_eof @ 0xcc80

void reset_eof(void)

{
  if (DAT_00182161 != '\0') {
    DAT_00182161 = '\0';
    DAT_00181f40 = 1;
    DAT_00181f48 = DAT_00181f58;
    DAT_00181f50 = DAT_00181f58 + (long)DAT_00182ba0 * 0x200;
  }
  return;
}




// Function: xclose @ 0xcd20

void xclose(int param_1)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = close(param_1);
  if (iVar2 == 0) {
    return;
  }
  uVar1 = dcgettext(0,"(pipe)",5);
  FUN_00132d60(uVar1);
  return;
}




// Function: archive_read_error @ 0xcd60

void archive_read_error(void)

{
  int iVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  FUN_00132f00(*DAT_00182828);
  iVar1 = DAT_00182164;
  if (DAT_00182170 == 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar3 = "At beginning of tape, quitting now";
  }
  else {
    DAT_00182164 = DAT_00182164 + 1;
    if (iVar1 < 0xb) {
      return;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar3 = "Too many errors, quitting";
  }
  uVar2 = dcgettext(0,pcVar3,5);
  error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: short_read @ 0xcdf0

void short_read(ulong param_1)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  long in_FS_OFFSET;
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  lVar4 = DAT_00181f58 + param_1;
  uVar3 = DAT_00182b98 - param_1;
  if ((((uVar3 != 0) && ((DAT_00181b80._2_1_ & 0x40) != 0)) &&
      ((uVar3 & 0x1ff) == 0 && DAT_00182170 == 0)) && (param_1 != 0)) {
    iVar1 = fstat(DAT_00182a34,&sStack_c8);
    if (iVar1 == 0) {
      if ((sStack_c8.st_mode & 0xb000) == 0x2000) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar2 = dcngettext(0,"Record size = %lu block","Record size = %lu blocks",param_1 >> 9,5);
        error(0,0,uVar2,param_1 >> 9);
      }
    }
    else {
      FUN_00126460(*DAT_00182828);
    }
  }
  while( true ) {
    if ((uVar3 & 0x1ff) == 0) {
      if (((uVar3 == 0) || (param_1 == 0)) || (DAT_00182160 == '\0')) break;
    }
    else if (param_1 == 0) break;
    while( true ) {
      if (DAT_00182a34 < 0x40000000) {
        param_1 = FUN_0014d570(DAT_00182a34,lVar4,uVar3);
      }
      else {
        param_1 = FUN_00134460(DAT_00182a34 + -0x40000000,lVar4,uVar3);
      }
      if (param_1 != 0xffffffffffffffff) break;
      FUN_0010cd60();
    }
    if (param_1 == 0) break;
    if (DAT_00182160 == '\0') {
      lVar4 = DAT_00182b98 - uVar3;
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = dcngettext(0,"Unaligned block (%lu byte) in archive",
                         "Unaligned block (%lu bytes) in archive",lVar4,5);
      error(0,0,uVar2,lVar4);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
    }
    uVar3 = uVar3 - param_1;
    lVar4 = lVar4 + param_1;
  }
  DAT_00181f38 = DAT_00181f38 + 1;
  DAT_00181f50 = (DAT_00182b98 - uVar3 & 0xfffffffffffffe00) + DAT_00181f58;
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: flush_archive @ 0xd000

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void flush_archive(void)

{
  char cVar1;
  void *pvVar2;
  char extraout_var_00;
  int iVar3;
  char extraout_var;
  int *piVar4;
  long lVar5;
  undefined8 uVar6;
  long lVar7;
  __off_t __offset;
  long in_FS_OFFSET;
  undefined2 local_18 [2];
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00181f40 == 0) {
    if (DAT_00182cf8 != '\0') {
      DAT_00181f40 = 1;
      DAT_00182cf8 = '\0';
      local_18[0] = 4;
      local_14 = 1;
      if (DAT_00182a34 < 0x40000000) {
        iVar3 = ioctl(DAT_00182a34,0x40086d01,local_18);
        cVar1 = (char)((uint)iVar3 >> 0x18);
      }
      else {
        FUN_001348b0(DAT_00182a34 + -0x40000000,0x40086d01,local_18);
        cVar1 = extraout_var;
      }
      if (cVar1 < '\0') {
        piVar4 = __errno_location();
        if (*piVar4 == 5) {
          if (DAT_00182a34 < 0x40000000) {
            iVar3 = ioctl(DAT_00182a34,0x40086d01,local_18);
            cVar1 = (char)((uint)iVar3 >> 0x18);
          }
          else {
            FUN_001348b0(DAT_00182a34 + -0x40000000,0x40086d01,local_18);
            cVar1 = extraout_var_00;
          }
          if (-1 < cVar1) goto LAB_0010d24d;
        }
        if (DAT_00182a34 < 0x40000000) {
          lVar5 = lseek(DAT_00182a34,0,1);
        }
        else {
          lVar5 = FUN_00134720(DAT_00182a34 + -0x40000000,0,1);
        }
        __offset = lVar5 - ((long)DAT_00181f50 - (long)DAT_00181f58);
        if (__offset < 0) {
          __offset = 0;
        }
        if (DAT_00182a34 < 0x40000000) {
          lVar5 = lseek(DAT_00182a34,__offset,0);
        }
        else {
          lVar5 = FUN_00134720(DAT_00182a34 + -0x40000000,__offset,0);
        }
        if (__offset != lVar5) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar6 = dcgettext(0,"Cannot backspace archive file; it may be unreadable without -i",5);
          error(0,0,uVar6);
          if (DAT_00181f58 != DAT_00182cf0) {
            memset(DAT_00181f58,0,(long)DAT_00182cf0 - (long)DAT_00181f58);
          }
        }
      }
LAB_0010d24d:
      lVar7 = (long)DAT_00182ba0;
      lVar5 = (long)DAT_00181f50 - (long)DAT_00181f58 >> 9;
      if (lVar7 <= lVar5) goto LAB_0010d0ac;
      memset(DAT_00181f50,0,(lVar7 - lVar5) * 0x200);
      DAT_00181f50 = (void *)((long)DAT_00182ba0 * 0x200 + (long)DAT_00181f58);
LAB_0010d111:
      pvVar2 = DAT_00181f48;
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
      goto LAB_0010d353;
    }
    DAT_00182170 = DAT_00182170 + ((long)DAT_00181f50 - (long)DAT_00181f58 >> 9);
    DAT_00181f50 = (void *)((long)DAT_00181f58 + (long)DAT_00182ba0 * 0x200);
  }
  else {
    lVar7 = (long)DAT_00182ba0;
    lVar5 = (long)DAT_00181f50 - (long)DAT_00181f58 >> 9;
LAB_0010d0ac:
    DAT_00182170 = DAT_00182170 + lVar5;
    DAT_00181f50 = (void *)(lVar7 * 0x200 + (long)DAT_00181f58);
    if (DAT_00181f40 == 1) {
      pvVar2 = DAT_00181f58;
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        lVar5 = (long)DAT_00181f48 - (long)DAT_00181f58;
        DAT_00181f48 = DAT_00181f58;
                    /* WARNING: Could not recover jumptable at 0x0010d0f1. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (*_DAT_00182158)(lVar5);
        return;
      }
      goto LAB_0010d353;
    }
    if (DAT_00181f40 == 2) {
      DAT_00181f48 = DAT_00181f58;
                    /* WARNING: Subroutine does not return */
      abort();
    }
    DAT_00181f48 = DAT_00181f58;
    if (DAT_00181f40 != 0) goto LAB_0010d111;
  }
  pvVar2 = DAT_00181f58;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    DAT_00181f48 = DAT_00181f58;
                    /* WARNING: Could not recover jumptable at 0x0010d081. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*_DAT_00182150)();
    return;
  }
LAB_0010d353:
  DAT_00181f48 = pvVar2;
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: add_volume_label @ 0xd530

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
  __s = (char *)FUN_00145370((long)DAT_0017f014,auStack_38);
  sVar1 = strlen(DAT_00182a40);
  sVar2 = strlen(__s);
  __ptr = (void *)FUN_0014ecc0(sVar1 + 9 + sVar2);
  __sprintf_chk(__ptr,1,0xffffffffffffffff,"%s %s %s",DAT_00182a40,"Volume",__s);
  if (DAT_00182ba4 == 4) {
    FUN_0011da70("GNU.volume.label",&DAT_00181f80,__ptr);
  }
  else {
    FUN_0010d3c0(__ptr);
  }
  free(__ptr);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_compressed_archive @ 0xd600

ulong check_compressed_archive(undefined1 *param_1)

{
  char *__s1;
  undefined1 uVar1;
  int iVar2;
  ulong uVar3;
  size_t __n;
  uint *puVar4;
  void *__s1_00;
  undefined *__s2;
  long in_FS_OFFSET;
  bool bVar5;
  undefined1 local_21;
  long local_20;
  
  uVar1 = DAT_00182160;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00182160 = 1;
  if (param_1 == (undefined1 *)0x0) {
    param_1 = &local_21;
  }
  DAT_00181f50 = DAT_00181f58;
  if (DAT_00181f58 == DAT_00181f48) {
    if (DAT_00182161 == '\0') {
      FUN_0010d000();
      if (DAT_00181f48 != DAT_00181f50) goto LAB_0010d657;
      DAT_00182161 = '\x01';
      bVar5 = true;
      goto LAB_0010d65d;
    }
    *param_1 = 1;
    __s1_00 = DAT_00181f58;
    DAT_00182160 = uVar1;
  }
  else {
LAB_0010d657:
    bVar5 = DAT_00181f48 == (void *)0x0;
LAB_0010d65d:
    __s1_00 = DAT_00181f58;
    *param_1 = bVar5;
    DAT_00182160 = uVar1;
    if (DAT_00181f58 != DAT_00181f50) {
      __s1 = (char *)((long)DAT_00181f58 + 0x101);
      iVar2 = strcmp(__s1,"ustar");
      if (((iVar2 == 0) || (iVar2 = strcmp(__s1,"ustar  "), iVar2 == 0)) &&
         (uVar3 = FUN_00122480(__s1_00,1), __s1_00 = DAT_00181f58, (int)uVar3 == 1))
      goto LAB_0010d6ef;
    }
  }
  __s2 = &DAT_001630b4;
  __n = 2;
  puVar4 = &DAT_0017d190;
  while (iVar2 = memcmp(__s1_00,__s2,__n), iVar2 != 0) {
    if (puVar4 + 6 == (uint *)&DAT_0017d250) {
      uVar3 = 0;
      goto LAB_0010d6ef;
    }
    __n = *(size_t *)(puVar4 + 8);
    __s2 = *(undefined **)(puVar4 + 10);
    puVar4 = puVar4 + 6;
  }
  uVar3 = (ulong)*puVar4;
LAB_0010d6ef:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: seek_archive @ 0xd780

ulong seek_archive(long param_1)

{
  long lVar1;
  ulong uVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  ulong uVar6;
  
  lVar1 = DAT_00182170;
  lVar4 = DAT_00181f48 - DAT_00181f58 >> 9;
  lVar5 = DAT_00182ba0 - lVar4;
  if ((param_1 <= lVar5 * 0x200) || (uVar2 = param_1 + lVar5 * -0x200, uVar2 < DAT_00182b98)) {
    return 0;
  }
  lVar5 = (uVar2 / DAT_00182b98) * DAT_00182b98;
  if (DAT_00182a34 < 0x40000000) {
    uVar2 = lseek(DAT_00182a34,lVar5,1);
  }
  else {
    uVar2 = FUN_00134720(DAT_00182a34 + -0x40000000,lVar5,1);
  }
  if ((long)uVar2 < 0) {
    return uVar2;
  }
  if (uVar2 % DAT_00182b98 != 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar3 = dcgettext(0,"rmtlseek not stopped at a record boundary",5);
    error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
    FUN_00116fd0();
  }
  uVar6 = ((long)uVar2 >> 9) - (lVar4 + lVar1);
  DAT_00182170 = ((long)uVar2 >> 9) - (long)DAT_00182ba0;
  DAT_00181f38 = DAT_00181f38 + (long)uVar6 / (long)DAT_00182ba0;
  DAT_00181f48 = DAT_00181f50;
  return uVar6;
}




// Function: close_archive @ 0xd8d0

void close_archive(void)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  int iVar2;
  
  if ((DAT_00182cf8 != '\0') || (DAT_00181f40 == 1)) {
    do {
      FUN_0010d000();
    } while (DAT_00181f58 < DAT_00181f48);
  }
  FUN_0010c770();
  if (DAT_00182a50 != '\0') {
    FUN_00111f60();
  }
  if (DAT_00182a34 < 0x40000000) {
    iVar2 = close(DAT_00182a34);
  }
  else {
    iVar2 = FUN_00134350(DAT_00182a34 + -0x40000000);
  }
  if (iVar2 != 0) {
    FUN_00132d60(*DAT_00182828);
  }
  FUN_0012bee0(DAT_00182168,DAT_00182161);
  FUN_0012fc00(&DAT_00182840);
  free(DAT_00182190);
  free(DAT_00182198);
  __ptr = DAT_00182148;
  while (__ptr != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*__ptr;
    free((void *)__ptr[2]);
    free(__ptr);
    __ptr = puVar1;
  }
  DAT_00182148 = (undefined8 *)0x0;
  DAT_00182140 = 0;
  return;
}




// Function: write_fatal_details @ 0xda10

void write_fatal_details(void)

{
  int iVar1;
  
  FUN_00133510();
  if (DAT_00182a34 < 0x40000000) {
    iVar1 = close(DAT_00182a34);
  }
  else {
    iVar1 = FUN_00134350(DAT_00182a34 + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_00132d60(*DAT_00182828);
  }
  FUN_0012bee0(DAT_00182168,0);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: archive_write_error @ 0xda70

void archive_write_error(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  if (DAT_00182a69 != '\0') goto LAB_0010da9d;
  do {
    FUN_0010da10(*DAT_00182828,param_1,DAT_00182b98);
LAB_0010da9d:
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    FUN_0010cc30();
    *piVar2 = iVar1;
  } while( true );
}




// Function: simple_flush_write @ 0xdac0

void simple_flush_write(void)

{
  long lVar1;
  
  lVar1 = FUN_0010c180();
  if (DAT_00182b98 == lVar1) {
    DAT_00181f30 = DAT_00181f30 + 1;
    DAT_001821a0 = (double)lVar1 + DAT_001821a0;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010da70(lVar1);
}




// Function: init_volume_number @ 0xdbf0

void init_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  __stream = fopen(DAT_00182a48,"r");
  if (__stream == (FILE *)0x0) {
    piVar4 = __errno_location();
    if (*piVar4 != 2) {
      FUN_00132ea0(DAT_00182a48);
      return;
    }
  }
  else {
    iVar1 = __isoc99_fscanf(__stream,"%d",&DAT_0017f010);
    if ((iVar1 != 1) || (DAT_0017f010 < 0)) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = FUN_0014ce40(DAT_00182a48);
      uVar3 = dcgettext(0,"%s: contains invalid volume number",5);
      error(0,0,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
    }
    if ((__stream->_flags & 0x20) == 0) {
      iVar1 = fclose(__stream);
    }
    else {
      FUN_00132f00(DAT_00182a48);
      iVar1 = fclose(__stream);
    }
    if (iVar1 != 0) {
      FUN_00132d60(DAT_00182a48);
      return;
    }
  }
  return;
}




// Function: closeout_volume_number @ 0xdcf0

void closeout_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  
  __stream = fopen(DAT_00182a48,"w");
  if (__stream == (FILE *)0x0) {
    FUN_00132ea0(DAT_00182a48);
    return;
  }
  __fprintf_chk(__stream,1,&DAT_0016a67a,DAT_0017f010);
  if ((__stream->_flags & 0x20) == 0) {
    iVar1 = fclose(__stream);
  }
  else {
    FUN_001334f0(DAT_00182a48);
    iVar1 = fclose(__stream);
  }
  if (iVar1 == 0) {
    return;
  }
  FUN_00132d60(DAT_00182a48);
  return;
}




// Function: gnu_flush_write @ 0xe570

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void gnu_flush_write(long param_1)

{
  long *plVar1;
  long *plVar2;
  undefined4 uVar3;
  char cVar4;
  ulong uVar5;
  int *piVar6;
  size_t sVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  long lVar10;
  long lVar11;
  uint uVar12;
  long *plVar13;
  size_t __n;
  undefined8 *puVar14;
  undefined8 *puVar15;
  void *__src;
  void *__src_00;
  undefined8 *puVar16;
  char *__s;
  long in_FS_OFFSET;
  byte bVar17;
  long local_48;
  long local_40;
  
  bVar17 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  _DAT_00182158 = FUN_0010dac0;
  uVar5 = FUN_0010c180();
  if ((uVar5 != DAT_00182b98) && (DAT_00182b10 == '\0')) {
LAB_0010ec6d:
                    /* WARNING: Subroutine does not return */
    FUN_0010da70(uVar5);
  }
  if (uVar5 != 0) {
    DAT_00181f30 = DAT_00181f30 + 1;
  }
  DAT_001821a0 = (double)(long)uVar5 + DAT_001821a0;
  plVar1 = DAT_00182148;
  if (uVar5 != DAT_00182b98) {
    do {
      plVar13 = plVar1;
      if ((plVar13 == (long *)0x0) || (plVar1 = (long *)*plVar13, plVar1 == (long *)0x0)) break;
    } while ((ulong)(plVar1[1] << 9) <= uVar5);
    if ((uVar5 & 0x1ff) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar8 = dcgettext(0,"write did not end on a block boundary",5);
      error(0,0,uVar8);
      DAT_00182d58 = 2;
      goto LAB_0010ec6d;
    }
    if ((long)uVar5 < 0) {
      piVar6 = __errno_location();
      if ((1 < *piVar6 - 5U) && (*piVar6 != 0x1c)) goto LAB_0010ec6d;
    }
    cVar4 = FUN_0010dd80(1);
    if (cVar4 != '\0') {
      FUN_0012fc00(&DAT_00181f80);
      DAT_0017f010 = DAT_0017f010 + 1;
      if (DAT_0017f010 < 0) {
        FUN_0010c310();
        goto LAB_0010ec7a;
      }
      __n = param_1 - uVar5;
      DAT_001821a8 = DAT_001821a8 + DAT_001821a0;
      DAT_001821a0 = 0.0;
      DAT_0017f014 = DAT_0017f014 + 1;
      __src = (void *)(DAT_00181f58 + uVar5);
      DAT_00182178 = (uint)(DAT_00182178 == 0);
      FUN_0010c100();
      DAT_00182138 = 1;
      if (DAT_00182a40 != 0) {
        FUN_0010d530();
      }
      if (plVar13 != (long *)0x0) {
        if (DAT_00182ba4 == 4) {
          local_48 = plVar13[3] - plVar13[4];
          FUN_0011da70("GNU.volume.filename",&DAT_00181f80,plVar13[2]);
          FUN_0011da70("GNU.volume.size",&DAT_00181f80,plVar13 + 4);
          FUN_0011da70("GNU.volume.offset",&DAT_00181f80,&local_48);
        }
        else {
          puVar16 = DAT_00181f48;
          if (((DAT_00181f48 == DAT_00181f50) && (puVar16 = (undefined8 *)0x0, DAT_00182161 == '\0')
              ) && (FUN_0010d000(), puVar16 = DAT_00181f48, DAT_00181f48 == DAT_00181f50)) {
            DAT_00182161 = '\x01';
            puVar16 = (undefined8 *)0x0;
          }
          __s = (char *)plVar13[2];
          sVar7 = strlen(__s);
          if (100 < sVar7) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
              __s = (char *)plVar13[2];
            }
            uVar8 = FUN_0014ce40(__s);
            uVar9 = dcgettext(0,
                              "%s: file name too long to be stored in a GNU multivolume header, truncated"
                              ,5);
            error(0,0,uVar9,uVar8);
            sVar7 = 100;
          }
          *puVar16 = 0;
          puVar16[0x3f] = 0;
          puVar14 = (undefined8 *)((ulong)(puVar16 + 1) & 0xfffffffffffffff8);
          uVar5 = (ulong)(((int)puVar16 - (int)puVar14) + 0x200U >> 3);
          for (; uVar5 != 0; uVar5 = uVar5 - 1) {
            *puVar14 = 0;
            puVar14 = puVar14 + (ulong)bVar17 * -2 + 1;
          }
          uVar12 = (uint)sVar7;
          uVar5 = sVar7 & 0xffffffff;
          puVar14 = (undefined8 *)plVar13[2];
          if (uVar12 < 8) {
            if ((sVar7 & 4) == 0) {
              if ((uVar12 != 0) &&
                 (*(undefined1 *)puVar16 = *(undefined1 *)puVar14, (sVar7 & 2) != 0)) {
                *(undefined2 *)((long)puVar16 + (uVar5 - 2)) =
                     *(undefined2 *)((long)puVar14 + (uVar5 - 2));
              }
            }
            else {
              *(undefined4 *)puVar16 = *(undefined4 *)puVar14;
              *(undefined4 *)((long)puVar16 + (uVar5 - 4)) =
                   *(undefined4 *)((long)puVar14 + (uVar5 - 4));
            }
          }
          else {
            puVar15 = (undefined8 *)((ulong)(puVar16 + 1) & 0xfffffffffffffff8);
            *puVar16 = *puVar14;
            *(undefined8 *)((long)puVar16 + ((sVar7 & 0xffffffff) - 8)) =
                 *(undefined8 *)((long)puVar14 + ((sVar7 & 0xffffffff) - 8));
            lVar11 = (long)puVar16 - (long)puVar15;
            puVar14 = (undefined8 *)((long)puVar14 - lVar11);
            for (uVar5 = (ulong)((int)lVar11 + uVar12 >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
              *puVar15 = *puVar14;
              puVar14 = puVar14 + (ulong)bVar17 * -2 + 1;
              puVar15 = puVar15 + (ulong)bVar17 * -2 + 1;
            }
          }
          *(undefined1 *)((long)puVar16 + 0x9c) = 0x4d;
          FUN_00112f80(plVar13[4],(long)puVar16 + 0x7c,0xc);
          FUN_00112f80(plVar13[3] - plVar13[4],(long)puVar16 + 0x171,0xc);
          uVar3 = DAT_00182a54;
          DAT_00182a54 = 0;
          FUN_00113320(&DAT_00182840,puVar16,0xffffffffffffffff);
          if (DAT_00181f48 <= puVar16) {
            DAT_00181f48 = (undefined8 *)
                           ((long)DAT_00181f48 +
                           ((long)puVar16 - (long)DAT_00181f48 & 0xfffffffffffffe00U) + 0x200);
          }
          DAT_00182a54 = uVar3;
          if (DAT_00181f50 < DAT_00181f48) goto LAB_0010e87d;
        }
      }
      puVar16 = DAT_00181f48;
      if (((DAT_00181f48 == DAT_00181f50) && (puVar16 = (undefined8 *)0x0, DAT_00182161 == '\0')) &&
         (FUN_0010d000(), puVar16 = DAT_00181f48, DAT_00181f48 == DAT_00181f50)) {
        DAT_00182161 = '\x01';
        puVar16 = (undefined8 *)0x0;
      }
      FUN_00113200(1,&DAT_00181f80,puVar16);
      FUN_0012fc00(&DAT_00181f80);
      if (plVar13 != (long *)0x0) {
        FUN_0010c400(plVar13);
      }
      lVar11 = DAT_00181f58;
      plVar1 = DAT_00182148;
      puVar16 = DAT_00181f48;
      if (((DAT_00181f48 == DAT_00181f50) && (puVar16 = (undefined8 *)0x0, DAT_00182161 == '\0')) &&
         (FUN_0010d000(), lVar11 = DAT_00181f58, plVar1 = DAT_00182148, puVar16 = DAT_00181f48,
         DAT_00181f48 == DAT_00181f50)) {
        DAT_00182161 = '\x01';
        puVar16 = (undefined8 *)0x0;
      }
      while ((lVar10 = DAT_00181f58, plVar1 != (long *)0x0 && (plVar13 != plVar1))) {
        plVar2 = (long *)*plVar1;
        DAT_00181f58 = lVar11;
        free((void *)plVar1[2]);
        free(plVar1);
        lVar11 = DAT_00181f58;
        plVar1 = plVar2;
        DAT_00181f58 = lVar10;
      }
      if (plVar1 == (long *)0x0) {
        DAT_00182140 = 0;
      }
      lVar10 = (long)puVar16 - DAT_00181f58;
      DAT_00182148 = plVar1;
      DAT_00181f58 = lVar11;
      for (; plVar13 != (long *)0x0; plVar13 = (long *)*plVar13) {
        plVar13[1] = plVar13[1] + (lVar10 >> 9);
        plVar13[5] = 0;
      }
      DAT_00182138 = 0;
      uVar5 = (long)DAT_00181f50 - (long)puVar16;
      __src_00 = __src;
      if (uVar5 < __n) {
        do {
          __src = (void *)((long)__src_00 + uVar5);
          memcpy(puVar16,__src_00,uVar5);
          __n = __n - uVar5;
          puVar16 = (undefined8 *)((uVar5 - 1 & 0xfffffffffffffe00) + (long)puVar16);
          if (DAT_00181f48 <= puVar16) {
            DAT_00181f48 = (undefined8 *)
                           ((long)DAT_00181f48 +
                           ((long)puVar16 - (long)DAT_00181f48 & 0xfffffffffffffe00U) + 0x200);
          }
          if (DAT_00181f50 < DAT_00181f48) goto LAB_0010e87d;
          puVar16 = DAT_00181f48;
          if (((DAT_00181f50 == DAT_00181f48) && (puVar16 = (undefined8 *)0x0, DAT_00182161 == '\0')
              ) && (FUN_0010d000(), puVar16 = DAT_00181f48, DAT_00181f48 == DAT_00181f50)) {
            DAT_00182161 = '\x01';
            puVar16 = (undefined8 *)0x0;
          }
          uVar5 = (long)DAT_00181f50 - (long)puVar16;
          __src_00 = __src;
        } while (uVar5 < __n);
      }
      memcpy(puVar16,__src,__n);
      memset((void *)((long)puVar16 + __n),0,uVar5 - __n);
      puVar16 = (undefined8 *)((long)puVar16 + (__n - 1 & 0xfffffffffffffe00));
      if (DAT_00181f48 <= puVar16) {
        DAT_00181f48 = (undefined8 *)
                       ((long)DAT_00181f48 +
                       ((long)puVar16 - (long)DAT_00181f48 & 0xfffffffffffffe00U) + 0x200);
      }
      if (DAT_00181f50 < DAT_00181f48) {
LAB_0010e87d:
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (((DAT_00181f50 == DAT_00181f48) && (DAT_00182161 == '\0')) &&
         (FUN_0010d000(), DAT_00181f48 == DAT_00181f50)) {
        DAT_00182161 = '\x01';
      }
    }
  }
  _DAT_00182158 = gnu_flush_write;
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010ec7a:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: drop_volume_label_suffix @ 0xf3f0

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
          __dest = (void *)FUN_0014ecc0(sVar2 + 1);
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




// Function: match_volume_label @ 0xf4b0

void match_volume_label(void)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined1 auStack_1d8 [360];
  undefined1 local_70 [80];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00181f18 == (char *)0x0) {
    if (DAT_00181f48 == DAT_00181f50) {
      if (DAT_00182161 != '\0') goto LAB_0010f625;
      FUN_0010d000();
      if (DAT_00181f48 == DAT_00181f50) {
        DAT_00182161 = '\x01';
        goto LAB_0010f625;
      }
    }
    lVar1 = DAT_00181f48;
    if (DAT_00181f48 == 0) {
LAB_0010f625:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar3 = FUN_0014d2d0(DAT_00182a40);
      uVar4 = dcgettext(0,"Archive not labeled to match %s",5);
      error(0,0,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
    }
    if (*(char *)(DAT_00181f48 + 0x9c) == 'V') {
      FUN_00125160(&DAT_00181f18,DAT_00181f48,100);
    }
    else if (*(char *)(DAT_00181f48 + 0x9c) == 'g') {
      FUN_0012fb40(auStack_1d8);
      uVar3 = FUN_001231e0(lVar1 + 0x7c,0xc);
      FUN_0011db80(local_70,lVar1,uVar3);
      FUN_0011d8a0(auStack_1d8);
      FUN_0012fc00(auStack_1d8);
    }
    if (DAT_00181f18 == (char *)0x0) goto LAB_0010f625;
  }
  pcVar6 = DAT_00181f18;
  iVar2 = fnmatch(DAT_00182a40,DAT_00181f18,0);
  if (iVar2 == 0) {
LAB_0010f593:
    if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if ((DAT_00182b10 != '\0') && (pcVar6 = (char *)FUN_0010f3f0(pcVar6), pcVar6 != (char *)0x0)) {
    iVar2 = fnmatch(pcVar6,DAT_00182a40,0);
    free(pcVar6);
    if (iVar2 == 0) goto LAB_0010f593;
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar3 = FUN_0014d2b0(1,DAT_00182a40);
  uVar4 = FUN_0014d2b0(0,DAT_00181f18);
  uVar5 = dcgettext(0,"Volume %s does not match %s",5);
  error(0,0,uVar5,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: flush_write @ 0xf6f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void flush_write(void)

{
                    /* WARNING: Could not recover jumptable at 0x0010f6fb. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*_DAT_00182158)(DAT_00182b98);
  return;
}




// Function: open_archive @ 0xf710

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void open_archive(int param_1)

{
  uint uVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  undefined8 uVar6;
  void *pvVar7;
  int *piVar8;
  uint *puVar9;
  char *pcVar10;
  undefined8 *puVar11;
  undefined *puVar12;
  long in_FS_OFFSET;
  bool bVar13;
  char local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  _DAT_00182150 = FUN_0010ec80;
  _DAT_00182158 = FUN_0010e570;
  if (DAT_00182b98 == 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar10 = "Invalid value for record_size";
  }
  else if (DAT_00182830 == 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar10 = "No archive name given";
  }
  else {
    FUN_0012fc00(&DAT_00182840);
    DAT_00182178 = 0;
    FUN_0010c100();
    iVar4 = 0;
    if (param_1 != 2) {
      iVar4 = param_1;
    }
    DAT_00181f40 = iVar4;
    iVar3 = strcmp((char *)*DAT_00182838,"-");
    if ((iVar3 != 0) || (iVar3 = isatty((uint)(iVar4 != 0)), iVar3 == 0)) {
      puVar11 = DAT_00182838;
      DAT_00182160 = DAT_00182ab9;
      DAT_00181f38 = 0;
      if (DAT_00182b70 == 0) {
        pcVar10 = (char *)*DAT_00182838;
        iVar4 = strcmp(pcVar10,"-");
        if (iVar4 == 0) {
          DAT_00182160 = 1;
          if (DAT_00182a50 != '\0') {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            pcVar10 = "Cannot verify stdin/stdout archive";
            goto LAB_0010fd8b;
          }
          if (param_1 == 1) {
            DAT_00182a34 = 1;
          }
          else {
            if (param_1 != 2) {
              if (param_1 == 0) {
                DAT_00182a34 = 0;
                uVar5 = FUN_0010d600(&local_41);
                if (1 < uVar5) {
                  if (DAT_00182d50 != (code *)0x0) {
                    (*DAT_00182d50)();
                  }
                  puVar9 = &DAT_0017f020;
                  iVar4 = 0;
                  do {
                    uVar1 = *puVar9;
                    puVar12 = (undefined *)0x0;
                    if (uVar1 == 0) {
LAB_00110069:
                      uVar6 = dcgettext(0,"Archive is compressed. Use %s option",5);
                      error(0,0,uVar6,puVar12);
                    /* WARNING: Subroutine does not return */
                      FUN_00116fd0();
                    }
                    puVar9 = puVar9 + 6;
                    if (uVar5 == uVar1) {
                      puVar12 = (&PTR_DAT_0017f030)[(long)iVar4 * 3];
                      goto LAB_00110069;
                    }
                    iVar4 = iVar4 + 1;
                  } while( true );
                }
                if (local_41 != '\0') {
                  if (DAT_00182d50 != (code *)0x0) {
                    (*DAT_00182d50)();
                  }
                  uVar6 = dcgettext(0,"This does not look like a tar archive",5);
                  error(0,0,uVar6);
                  DAT_00182d58 = 2;
                }
              }
              goto LAB_0010f7f6;
            }
            DAT_00182a34 = 0;
            DAT_00181f20 = 1;
            DAT_00181f50 = DAT_00181f58;
          }
          if (DAT_00182820 == 0) {
            DAT_00181f28 = stderr;
          }
          cVar2 = FUN_0012bd50();
          if (cVar2 == '\0') goto LAB_0010fb60;
LAB_0010fc06:
          FUN_0012be10();
          FUN_0012bdc0();
LAB_0010f821:
          if (1 < param_1 - 1U) goto LAB_0010f860;
          DAT_00181f30 = 0;
          if (param_1 != 1) goto LAB_0010f8d3;
LAB_0010f83d:
          DAT_00181f30 = 0;
          if (DAT_00182a40 != 0) {
            if (DAT_00182b10 == '\0') {
              if (DAT_00182ba4 == 4) {
                FUN_0011da70("GNU.volume.label",&DAT_00181f80,DAT_00182a40);
              }
              else {
                FUN_0010d3c0();
              }
            }
            else {
              FUN_0010d530();
            }
          }
          goto LAB_0010f860;
        }
        if (param_1 != 1) {
          if (param_1 != 2) {
            if (param_1 != 0) goto LAB_0010f7f6;
            if (DAT_00182d70 == '\0') {
              _DAT_00182d78 = strchr(pcVar10,0x3a);
              pcVar10 = (char *)*puVar11;
              if (((_DAT_00182d78 == (char *)0x0) || (_DAT_00182d78 <= pcVar10)) ||
                 (pvVar7 = memchr(pcVar10,0x2f,(long)_DAT_00182d78 - (long)pcVar10),
                 pvVar7 != (void *)0x0)) goto LAB_0010fad1;
              DAT_00182a34 = FUN_00133a80(pcVar10,0,0x40000000,DAT_00182ab0);
            }
            else {
LAB_0010fad1:
              DAT_00182a34 = open(pcVar10,0,0x1b6);
            }
            if (DAT_00182a34 == -1) goto LAB_0010fb60;
            if (DAT_00182b10 == '\0') {
              iVar4 = DAT_00181f70;
              if (DAT_00182b70 != 0) {
LAB_0010fb04:
                DAT_00181f70 = iVar4;
                if (DAT_00182a34 < 0x40000000) {
                  close(DAT_00182a34);
                }
                else {
                  FUN_00134350(DAT_00182a34 + -0x40000000);
                }
                DAT_00182161 = '\0';
                DAT_00182168 = FUN_0012c570();
                DAT_00182160 = 1;
                goto LAB_0010fb39;
              }
              iVar4 = FUN_0010d600(&local_41);
              if (iVar4 == 0) {
                if (local_41 != '\0') {
                  if (DAT_00182d50 != (code *)0x0) {
                    (*DAT_00182d50)();
                  }
                  uVar6 = dcgettext(0,"This does not look like a tar archive",5);
                  error(0,0,uVar6);
                  DAT_00182d58 = 2;
                }
                FUN_0012b880(*DAT_00182838,0);
                iVar4 = DAT_00181f70;
                if (DAT_00182b70 != 0) goto LAB_0010fb04;
              }
              else {
                if (iVar4 != 1) goto LAB_0010fb04;
                if (local_41 != '\0') {
                  if (DAT_00182d50 != (code *)0x0) {
                    (*DAT_00182d50)();
                  }
                  uVar6 = dcgettext(0,"This does not look like a tar archive",5);
                  error(0,0,uVar6);
                  DAT_00182d58 = 2;
                }
              }
            }
            else {
LAB_0010fb39:
              DAT_00181f38 = 0;
              DAT_00181f50 = DAT_00181f58;
            }
            if (DAT_00182a34 < 0) goto LAB_0010fb60;
            FUN_0010c350();
            goto LAB_0010f7f6;
          }
          if (DAT_00182d70 == '\0') {
            _DAT_00182d78 = strchr(pcVar10,0x3a);
            pcVar10 = (char *)*puVar11;
            if (((_DAT_00182d78 == (char *)0x0) || (_DAT_00182d78 <= pcVar10)) ||
               (pvVar7 = memchr(pcVar10,0x2f,(long)_DAT_00182d78 - (long)pcVar10),
               pvVar7 != (void *)0x0)) goto LAB_0010fd47;
            DAT_00182a34 = FUN_00133a80(pcVar10,0x42,0x40000000,DAT_00182ab0);
          }
          else {
LAB_0010fd47:
            DAT_00182a34 = open(pcVar10,0x42,0x1b6);
          }
          uVar5 = FUN_0010d600(0);
          if (1 < uVar5) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            pcVar10 = "Cannot update compressed archives";
            goto LAB_0010fd8b;
          }
          goto LAB_0010f7f6;
        }
        bVar13 = DAT_00182b84 != '\0';
        if (bVar13) {
          FUN_00125c10(pcVar10,1);
          pcVar10 = (char *)*DAT_00182838;
          puVar11 = DAT_00182838;
        }
        if (DAT_00182a50 == '\0') {
          if (DAT_00182d70 == '\0') {
            _DAT_00182d78 = strchr(pcVar10,0x3a);
            pcVar10 = (char *)*puVar11;
            if (((_DAT_00182d78 != (char *)0x0) && (pcVar10 < _DAT_00182d78)) &&
               (pvVar7 = memchr(pcVar10,0x2f,(long)_DAT_00182d78 - (long)pcVar10),
               pvVar7 == (void *)0x0)) {
              DAT_00182a34 = FUN_00133a80(pcVar10,0x41,0x40000000,DAT_00182ab0);
              goto LAB_0010fcb9;
            }
          }
          DAT_00182a34 = creat(pcVar10,0x1b6);
        }
        else {
          if (DAT_00182d70 == '\0') {
            _DAT_00182d78 = strchr(pcVar10,0x3a);
            pcVar10 = (char *)*puVar11;
            if (((_DAT_00182d78 != (char *)0x0) && (pcVar10 < _DAT_00182d78)) &&
               (pvVar7 = memchr(pcVar10,0x2f,(long)_DAT_00182d78 - (long)pcVar10),
               pvVar7 == (void *)0x0)) {
              DAT_00182a34 = FUN_00133a80(pcVar10,0x42,0x40000000,DAT_00182ab0);
              goto LAB_0010fcb9;
            }
          }
          DAT_00182a34 = open(pcVar10,0x42,0x1b6);
        }
LAB_0010fcb9:
        if (-1 < DAT_00182a34) {
          if (0x3fffffff < DAT_00182a34) {
            FUN_0012be10();
            FUN_0012bdc0();
            goto LAB_0010f83d;
          }
          cVar2 = FUN_0012bd50();
          if (cVar2 != '\0') goto LAB_0010fc06;
        }
        piVar8 = __errno_location();
        iVar4 = *piVar8;
        if (bVar13) {
          FUN_00125ea0();
        }
LAB_0010fb6b:
        puVar11 = DAT_00182838;
        *piVar8 = iVar4;
        FUN_00132ec0(*puVar11);
LAB_0010fb80:
        if ((DAT_00182161 == '\0') && (FUN_0010d000(), DAT_00181f48 == DAT_00181f50)) {
          DAT_00182161 = '\x01';
        }
      }
      else {
        if (param_1 == 1) {
          DAT_00182168 = FUN_0012c180();
          if ((DAT_00182820 == 0) && (iVar4 = strcmp((char *)*DAT_00182838,"-"), iVar4 == 0)) {
            DAT_00181f28 = stderr;
          }
        }
        else {
          if (param_1 == 2) {
                    /* WARNING: Subroutine does not return */
            abort();
          }
          if (param_1 == 0) {
            DAT_00182168 = FUN_0012c570();
            DAT_00182160 = 1;
            DAT_00181f50 = DAT_00181f58;
          }
        }
LAB_0010f7f6:
        if (DAT_00182a34 < 0) {
LAB_0010fb60:
          piVar8 = __errno_location();
          iVar4 = *piVar8;
          goto LAB_0010fb6b;
        }
        if (DAT_00182a34 < 0x40000000) {
          cVar2 = FUN_0012bd50();
          if (cVar2 == '\0') goto LAB_0010fb60;
          FUN_0012be10();
          FUN_0012bdc0();
        }
        else {
          FUN_0012be10();
          FUN_0012bdc0();
        }
        if (param_1 != 0) goto LAB_0010f821;
        if (DAT_00181f48 == DAT_00181f50) goto LAB_0010fb80;
      }
LAB_0010f8d3:
      if (DAT_00182a40 != 0) {
        FUN_0010f4b0();
      }
LAB_0010f860:
      FUN_00143220(&DAT_00182a10);
      _DAT_00182a00 = DAT_00182a10;
      _DAT_00182a08 = DAT_00182a18;
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    if (iVar4 == 0) {
      uVar6 = dcgettext(0,"Refusing to read archive contents from terminal (missing -f option?)",5);
      goto LAB_0010f9a1;
    }
    pcVar10 = "Refusing to write archive contents to terminal (missing -f option?)";
  }
LAB_0010fd8b:
  uVar6 = dcgettext(0,pcVar10,5);
LAB_0010f9a1:
  error(0,0,uVar6);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: copy_string_unquote @ 0x100a0

char * copy_string_unquote(void)

{
  char cVar1;
  char *__s;
  size_t sVar2;
  
  __s = (char *)FUN_0014ee30();
  cVar1 = *__s;
  if ((cVar1 == '\"') || (cVar1 == '\'')) {
    sVar2 = strlen(__s);
    if (__s[sVar2 - 1] == cVar1) {
      memmove(__s,__s + 1,sVar2 - 2);
      __s[sVar2 - 2] = '\0';
      FUN_001251b0(__s);
      return __s;
    }
  }
  FUN_001251b0(__s);
  return __s;
}




// Function: getwidth @ 0x10110

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
      if (0 < (long)uVar2) goto LAB_0011016d;
    }
    uVar2 = 0x50;
  }
LAB_0011016d:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: format_checkpoint_string @ 0x101a0

ulong format_checkpoint_string(FILE *param_1,ulong param_2,byte *param_3,char param_4,undefined4 param_5)

{
  byte bVar1;
  byte *pbVar2;
  long lVar3;
  int iVar4;
  size_t sVar5;
  char *pcVar6;
  tm *ptVar7;
  long lVar8;
  long lVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  ulong uVar12;
  undefined **ppuVar13;
  char *pcVar14;
  byte *pbVar15;
  long in_FS_OFFSET;
  char *local_1a0;
  char *local_198;
  timeval local_178;
  undefined *local_168 [4];
  long local_148;
  long local_140;
  undefined *local_110;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 == '\0') {
    local_1a0 = (char *)dcgettext(0,"read",5);
    local_198 = (char *)FUN_00145370(param_5);
    if (param_3 == (byte *)0x0) {
      param_3 = (byte *)dcgettext(0,"Read checkpoint %u",5);
    }
  }
  else {
    local_1a0 = (char *)dcgettext(0,"write",5);
    local_198 = (char *)FUN_00145370(param_5);
    if (param_3 == (byte *)0x0) {
      param_3 = (byte *)dcgettext(0,"Write checkpoint %u",5);
    }
  }
  bVar1 = *param_3;
  do {
    if (bVar1 == 0) {
      fflush_unlocked(param_1);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return param_2 & 0xffffffff;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (bVar1 != 0x25) {
      pbVar15 = (byte *)param_1->_IO_write_ptr;
      if (pbVar15 < param_1->_IO_write_end) {
        param_1->_IO_write_ptr = (char *)(pbVar15 + 1);
        *pbVar15 = bVar1;
      }
      else {
        __overflow(param_1,(uint)bVar1);
      }
      param_2 = param_2 + 1;
      pbVar15 = param_3;
      if (*param_3 == 0xd) {
        DAT_001821d8 = 1;
        param_2 = 0;
      }
      goto LAB_0011025c;
    }
    bVar1 = param_3[1];
    pbVar15 = param_3 + 1;
    if (bVar1 == 0x7b) {
      pcVar6 = strchr((char *)(param_3 + 2),0x7d);
      if (pcVar6 != (char *)0x0) {
        uVar12 = (long)pcVar6 - (long)pbVar15;
        if (DAT_001821c0 < uVar12) {
          DAT_001821c0 = uVar12;
          DAT_001821c8 = (char *)FUN_0014ece0(DAT_001821c8,uVar12);
        }
        memcpy(DAT_001821c8,param_3 + 2,uVar12 - 1);
        pbVar15 = (byte *)(pcVar6 + 1);
        DAT_001821c8[uVar12 - 1] = '\0';
        pcVar14 = DAT_001821c8;
        if (DAT_001821c8 != (char *)0x0) {
          if (pcVar6[1] == '*') {
            uVar12 = strtol(DAT_001821c8,(char **)0x0,10);
            goto LAB_0011060c;
          }
          switch(pcVar6[1]) {
          case 'T':
            FUN_0010c770();
            local_110 = &DAT_0016379c;
            lVar9 = FUN_001375a0(pcVar14,&local_148,0x4644);
            lVar3 = local_140;
            lVar8 = local_148;
            if ((int)lVar9 == 0) {
              if (local_148 != 0) {
                lVar9 = 0;
                ppuVar13 = local_168;
                do {
                  ppuVar13[lVar9] = *(undefined **)(lVar3 + lVar9 * 8);
                  lVar9 = lVar9 + 1;
                } while (lVar9 != lVar8);
                if ((int)lVar9 == 3) goto LAB_001105d9;
              }
              ppuVar13 = local_168;
              lVar8 = (long)(int)lVar9;
              do {
                ppuVar13[lVar8] = (undefined *)0x0;
                lVar8 = lVar8 + 1;
              } while ((int)lVar8 != 3);
            }
            else {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar10 = FUN_001358f0(&local_148);
              uVar11 = dcgettext(0,"cannot split string \'%s\': %s",5);
              error(0,0,uVar11,pcVar14,uVar10);
              ppuVar13 = &PTR_DAT_0017f140;
              DAT_00182d58 = 2;
            }
LAB_001105d9:
            iVar4 = FUN_0010c900(param_1,ppuVar13,0x2c,0);
            param_2 = param_2 + (long)iVar4;
            FUN_00135800(&local_148);
            goto LAB_0011025c;
          case 'c':
            goto switchD_001102ca_caseD_63;
          case 'd':
            goto switchD_001102ca_caseD_64;
          case 's':
            goto switchD_001102ca_caseD_73;
          case 't':
            goto switchD_001104e8_caseD_74;
          case 'u':
            goto switchD_001102ca_caseD_75;
          }
        }
      }
switchD_001102ca_caseD_55:
      pcVar6 = param_1->_IO_write_ptr;
      if (pcVar6 < param_1->_IO_write_end) {
        param_1->_IO_write_ptr = pcVar6 + 1;
        *pcVar6 = '%';
      }
      else {
        __overflow(param_1,0x25);
      }
      bVar1 = *pbVar15;
      pbVar2 = (byte *)param_1->_IO_write_ptr;
      if (pbVar2 < param_1->_IO_write_end) {
        param_1->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar1;
      }
      else {
        __overflow(param_1,(uint)bVar1);
      }
      param_2 = param_2 + 2;
      goto LAB_0011025c;
    }
    if (bVar1 == 0x2a) {
      uVar12 = FUN_00110110(param_1);
LAB_0011060c:
      if (param_2 < uVar12) {
        do {
          pcVar6 = param_1->_IO_write_ptr;
          if (pcVar6 < param_1->_IO_write_end) {
            param_1->_IO_write_ptr = pcVar6 + 1;
            *pcVar6 = ' ';
          }
          else {
            __overflow(param_1,0x20);
          }
          param_2 = param_2 + 1;
        } while (param_2 != uVar12);
      }
      goto LAB_0011025c;
    }
    switch(bVar1) {
    case 0x54:
      FUN_0010c770();
      iVar4 = FUN_0010c900(param_1,&PTR_DAT_0017f140,0x2c);
      param_2 = param_2 + (long)iVar4;
      break;
    default:
      goto switchD_001102ca_caseD_55;
    case 99:
switchD_001102ca_caseD_63:
      iVar4 = format_checkpoint_string(param_1,param_2,"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r",param_4,
                           param_5);
      param_2 = param_2 + (long)iVar4;
      break;
    case 100:
switchD_001102ca_caseD_64:
      FUN_0010c770();
      iVar4 = __fprintf_chk(param_1,1,&DAT_00163797);
      param_2 = param_2 + (long)iVar4;
      break;
    case 0x73:
switchD_001102ca_caseD_73:
      pcVar6 = local_1a0;
      goto LAB_001103a0;
    case 0x74:
      pcVar14 = "%c";
switchD_001104e8_caseD_74:
      gettimeofday(&local_178,(__timezone_ptr_t)0x0);
      ptVar7 = localtime(&local_178.tv_sec);
      lVar8 = FUN_00143180(param_1,pcVar14,ptVar7,0,(int)local_178.tv_usec * 1000);
      param_2 = param_2 + lVar8;
      break;
    case 0x75:
switchD_001102ca_caseD_75:
      pcVar6 = local_198;
LAB_001103a0:
      fputs_unlocked(pcVar6,param_1);
      sVar5 = strlen(pcVar6);
      param_2 = param_2 + sVar5;
    }
LAB_0011025c:
    bVar1 = pbVar15[1];
    param_3 = pbVar15 + 1;
  } while( true );
}




// Function: checkpoint_finish_compile @ 0x10ba0

void checkpoint_finish_compile(void)

{
  if (DAT_00182260 == 0) {
    if (DAT_00182b78 == 0) {
      return;
    }
    if (DAT_00182270 != 0) {
      return;
    }
    FUN_00110710();
  }
  if (DAT_00182260 != 1) {
    return;
  }
  sigprocmask(0,(sigset_t *)&DAT_001821e0,(sigset_t *)0x0);
  if (DAT_00182b78 == 0) {
    DAT_00182b78 = 10;
  }
  DAT_00182260 = 2;
  return;
}




// Function: checkpoint_flush_actions @ 0x10c20

void checkpoint_flush_actions(void)

{
  char *pcVar1;
  long lVar2;
  undefined8 *puVar3;
  _IO_FILE *__stream;
  
  puVar3 = DAT_00182270;
  do {
    if (puVar3 == (undefined8 *)0x0) {
      return;
    }
    while (((__stream = DAT_001821d0, *(int *)(puVar3 + 1) == 3 && (DAT_001821d0 != (_IO_FILE *)0x0)
            ) && (DAT_001821d8 != 0))) {
      for (lVar2 = FUN_00110110(DAT_001821d0); lVar2 != 0; lVar2 = lVar2 + -1) {
        pcVar1 = __stream->_IO_write_ptr;
        if (pcVar1 < __stream->_IO_write_end) {
          __stream->_IO_write_ptr = pcVar1 + 1;
          *pcVar1 = ' ';
        }
        else {
          __overflow(__stream,0x20);
          __stream = DAT_001821d0;
        }
      }
      pcVar1 = __stream->_IO_write_ptr;
      if (pcVar1 < __stream->_IO_write_end) {
        __stream->_IO_write_ptr = pcVar1 + 1;
        *pcVar1 = '\r';
      }
      else {
        __overflow(__stream,0xd);
        __stream = DAT_001821d0;
      }
      fflush_unlocked(__stream);
      puVar3 = (undefined8 *)*puVar3;
      if (puVar3 == (undefined8 *)0x0) {
        return;
      }
    }
    puVar3 = (undefined8 *)*puVar3;
  } while( true );
}




// Function: checkpoint_run @ 0x10cf0

void checkpoint_run(undefined1 param_1)

{
  char *pcVar1;
  undefined8 *puVar2;
  int iVar3;
  long in_FS_OFFSET;
  int local_44;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00182b78 != 0) &&
     (DAT_00182278 = DAT_00182278 + 1, puVar2 = DAT_00182270, DAT_00182278 % DAT_00182b78 == 0)) {
    for (; puVar2 != (undefined8 *)0x0; puVar2 = (undefined8 *)*puVar2) {
      switch(*(undefined4 *)(puVar2 + 1)) {
      case 0:
        pcVar1 = DAT_00181f28->_IO_write_ptr;
        if (pcVar1 < DAT_00181f28->_IO_write_end) {
          DAT_00181f28->_IO_write_ptr = pcVar1 + 1;
          *pcVar1 = '.';
        }
        else {
          __overflow(DAT_00181f28,0x2e);
        }
        fflush_unlocked(DAT_00181f28);
        break;
      case 1:
        if ((DAT_001821d0 != (_IO_FILE *)0x0) ||
           (DAT_001821d0 = fopen("/dev/tty","w"), DAT_001821d0 != (FILE *)0x0)) {
          pcVar1 = DAT_001821d0->_IO_write_ptr;
          if (pcVar1 < DAT_001821d0->_IO_write_end) {
            DAT_001821d0->_IO_write_ptr = pcVar1 + 1;
            *pcVar1 = '\a';
          }
          else {
            __overflow(DAT_001821d0,7);
          }
          fflush_unlocked(DAT_001821d0);
        }
        break;
      case 2:
        iVar3 = __fprintf_chk(stderr,1,"%s: ",DAT_00182dd0);
        FUN_001101a0(stderr,(long)iVar3,puVar2[2],param_1,DAT_00182278);
        pcVar1 = stderr->_IO_write_ptr;
        if (pcVar1 < stderr->_IO_write_end) {
          stderr->_IO_write_ptr = pcVar1 + 1;
          *pcVar1 = '\n';
        }
        else {
          __overflow(stderr,10);
        }
        break;
      case 3:
        if ((DAT_001821d0 != (_IO_FILE *)0x0) ||
           (DAT_001821d0 = fopen("/dev/tty","w"), DAT_001821d0 != (FILE *)0x0)) {
          FUN_001101a0(DAT_001821d0,0,puVar2[2],param_1,DAT_00182278);
        }
        break;
      case 4:
        sleep(*(uint *)(puVar2 + 2));
        break;
      case 5:
        FUN_0012d260(puVar2[2],*DAT_00182828,DAT_00182278);
        break;
      case 6:
        FUN_0010c770();
        FUN_0010cc30();
        break;
      case 7:
        sigwait((sigset_t *)&DAT_001821e0,&local_44);
      }
    }
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: checkpoint_finish @ 0x10fa0

void checkpoint_finish(void)

{
  if (DAT_00182b78 == 0) {
    return;
  }
  FUN_00110c20();
  if (DAT_001821d0 != (FILE *)0x0) {
    fclose(DAT_001821d0);
    return;
  }
  return;
}




// Function: process_noop @ 0x10fe0

undefined8 process_noop(void)

{
  return 1;
}




// Function: report_difference @ 0x11120

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
    uVar1 = FUN_001250f0(1,*(undefined8 *)(param_9 + 8));
    __fprintf_chk(DAT_00181f28,1,"%s: ",uVar1);
    local_d0 = &stack0x00000008;
    local_d8 = 0x10;
    local_c8 = local_b8;
    local_d4 = 0x30;
    __vfprintf_chk(DAT_00181f28,1,param_10,&local_d8);
    __fprintf_chk(DAT_00181f28,1,&DAT_0016a7d2);
  }
  FUN_0012fd00(1);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: process_rawdata @ 0x11250

undefined8 process_rawdata(size_t param_1,void *param_2)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  
  sVar2 = FUN_00125fe0(DAT_00182290,DAT_00182288,param_1);
  if (param_1 == sVar2) {
    iVar1 = memcmp(param_2,DAT_00182288,param_1);
    uVar3 = 1;
    if (iVar1 != 0) {
      uVar3 = dcgettext(0,"Contents differ",5);
      FUN_00111120(&DAT_00182840,uVar3);
      return 0;
    }
  }
  else {
    if (sVar2 == 0xffffffffffffffff) {
      FUN_00132f00(DAT_00182848);
      FUN_00111120(&DAT_00182840,0);
      return 0;
    }
    uVar3 = dcngettext(0,"Could only read %lu of %lu byte","Could only read %lu of %lu bytes",
                       param_1,5);
    FUN_00111120(&DAT_00182840,uVar3,sVar2,param_1);
    uVar3 = 0;
  }
  return uVar3;
}




// Function: diff_symlink @ 0x11340

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
  __n = strlen(DAT_00182858);
  if (0x3ff < __n) {
    __buf = (char *)FUN_0014ecc0(__n + 1);
  }
  sVar2 = readlinkat(DAT_0017f234,DAT_00182848,__buf,__n + 1);
  if ((long)sVar2 < 0) {
    piVar4 = __errno_location();
    if (*piVar4 == 2) {
      FUN_00133160(DAT_00182848);
    }
    else {
      FUN_00133140();
    }
    FUN_00111120(&DAT_00182840,0);
  }
  else {
    if (__n == sVar2) {
      iVar1 = memcmp(DAT_00182858,__buf,__n);
      if (iVar1 == 0) goto LAB_001113e1;
    }
    uVar3 = dcgettext(0,"Symlink differs",5);
    FUN_00111120(&DAT_00182840,uVar3);
  }
LAB_001113e1:
  if (__buf != acStack_438) {
    free(__buf);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_dir @ 0x11460

void diff_dir(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  undefined1 auStack_b8 [24];
  uint local_a0;
  long local_20;
  
  uVar2 = DAT_00182848;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00125fc0(DAT_00182848,auStack_b8);
  if (iVar1 == 0) {
    if ((local_a0 & 0xf000) == 0x4000) {
      if (((local_a0 ^ DAT_001828b0) & 0xfff) == 0) goto LAB_001114bf;
      pcVar4 = "Mode differs";
    }
    else {
      pcVar4 = "File type differs";
    }
    uVar2 = dcgettext(0,pcVar4,5);
    FUN_00111120(&DAT_00182840,uVar2);
  }
  else {
    piVar3 = __errno_location();
    if (*piVar3 == 2) {
      FUN_00133430(uVar2);
    }
    else {
      FUN_00133410();
    }
    FUN_00111120(&DAT_00182840,0);
  }
LAB_001114bf:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_link @ 0x11560

void diff_link(void)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 auStack_158 [144];
  undefined1 local_c8 [152];
  long local_30;
  
  uVar4 = DAT_00182848;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00125fc0(DAT_00182848,auStack_158);
  uVar5 = DAT_00182858;
  if (iVar2 == 0) {
    iVar2 = FUN_00125fc0(DAT_00182858,local_c8);
    if (iVar2 == 0) {
      cVar1 = FUN_0012c0f0(auStack_158,local_c8);
      if (cVar1 == '\0') {
        uVar4 = FUN_001250f0(0,DAT_00182858);
        uVar5 = dcgettext(0,"Not linked to %s",5);
        FUN_00111120(&DAT_00182840,uVar5,uVar4);
      }
      goto LAB_001115ce;
    }
    piVar3 = __errno_location();
    uVar4 = uVar5;
    if (*piVar3 != 2) goto LAB_00111605;
LAB_0011162d:
    FUN_00133430();
  }
  else {
    piVar3 = __errno_location();
    if (*piVar3 == 2) goto LAB_0011162d;
LAB_00111605:
    FUN_00133410(uVar4);
  }
  FUN_00111120(&DAT_00182840,0);
LAB_001115ce:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diff_archive @ 0x11680

void diff_archive(void)

{
  char cVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *__s2;
  ulong uVar6;
  size_t sVar7;
  long __offset;
  __off_t _Var8;
  int *piVar9;
  char *pcVar10;
  byte bVar11;
  uint uVar12;
  code *pcVar13;
  char *pcVar14;
  long in_FS_OFFSET;
  undefined1 auStack_c8 [24];
  uint local_b0;
  long local_a0;
  long local_98;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010ccd0(DAT_00182408);
  if (DAT_00182a54 != 0) {
    if (DAT_00182280 != '\0') {
      uVar4 = dcgettext(0,"Verify ",5);
      __fprintf_chk(DAT_00181f28,1,uVar4);
    }
    FUN_00123bc0(&DAT_00182840,DAT_00182408,0xffffffffffffffff);
  }
  uVar4 = DAT_00182848;
  cVar1 = *(char *)(DAT_00182408 + 0x9c);
  switch(cVar1) {
  case '\0':
  case '0':
  case '7':
  case 'S':
    goto switchD_0011170a_caseD_0;
  default:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
      cVar1 = *(char *)(DAT_00182408 + 0x9c);
    }
    uVar4 = FUN_0014ce40(DAT_00182848);
    uVar5 = dcgettext(0,"%s: Unknown file type \'%c\', diffed as normal file",5);
    error(0,0,uVar5,uVar4,(int)cVar1);
    DAT_00182d58 = 2;
switchD_0011170a_caseD_0:
    uVar4 = DAT_00182848;
    if (DAT_00182850 != '\0') {
LAB_0011177d:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        FUN_00111460();
        return;
      }
      goto LAB_00111f55;
    }
    iVar2 = FUN_00125fc0(DAT_00182848,auStack_c8);
    if (iVar2 == 0) {
      if ((local_b0 & 0xf000) == 0x8000) {
        if (((local_b0 ^ DAT_001828b0) & 0xfff) != 0) {
          uVar5 = dcgettext(0,"Mode differs",5);
          FUN_00111120(&DAT_00182840,uVar5);
        }
        cVar1 = FUN_0012c0d0(auStack_c8,&DAT_00182898);
        if (cVar1 == '\0') {
          uVar5 = dcgettext(0,"Uid differs",5);
          FUN_00111120(&DAT_00182840,uVar5);
        }
        cVar1 = FUN_0012c0e0(auStack_c8,&DAT_00182898);
        if (cVar1 == '\0') {
          uVar5 = dcgettext(0,"Gid differs",5);
          FUN_00111120(&DAT_00182840,uVar5);
        }
        iVar2 = FUN_0012fcc0(local_70,local_68,DAT_00182938,DAT_00182940);
        if (iVar2 != 0) {
          uVar5 = dcgettext(0,"Mod time differs",5);
          FUN_00111120(&DAT_00182840,uVar5);
        }
        if ((*(char *)(DAT_00182408 + 0x9c) == 'S') || (local_98 == DAT_001828c8)) {
          DAT_00182290 = __openat_2(DAT_0017f234,uVar4,DAT_00182808);
          if (DAT_00182290 < 0) {
            FUN_00132ea0(uVar4);
            FUN_00124660();
            FUN_00111120(&DAT_00182840,0);
          }
          else {
            if (DAT_00182960 == '\0') {
              FUN_00110ff0(FUN_00111250);
            }
            else {
              FUN_0012b690(DAT_00182290,&DAT_00182840);
            }
            if (((DAT_00182b88 == 1) && (local_98 != 0)) &&
               (iVar2 = FUN_00126100(DAT_00182290,DAT_0017f234,uVar4,local_80,local_78), iVar2 != 0)
               ) {
              FUN_001334b0(uVar4);
            }
            iVar2 = close(DAT_00182290);
            if (iVar2 != 0) {
              FUN_00132d60(uVar4);
            }
          }
          break;
        }
LAB_00111cd0:
        pcVar14 = "Size differs";
      }
      else {
LAB_001117e5:
        pcVar14 = "File type differs";
      }
      uVar4 = dcgettext(0,pcVar14,5);
      FUN_00111120(&DAT_00182840,uVar4);
      FUN_00124660();
    }
    else {
      piVar9 = __errno_location();
      if (*piVar9 == 2) {
        FUN_00133430(uVar4);
      }
      else {
        FUN_00133410();
      }
LAB_00111cb6:
      FUN_00111120(&DAT_00182840,0);
      FUN_00124660();
    }
    break;
  case '1':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00111560();
      return;
    }
    goto LAB_00111f55;
  case '2':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00111340();
      return;
    }
    goto LAB_00111f55;
  case '3':
  case '4':
  case '6':
    iVar2 = FUN_00125fc0(DAT_00182848,auStack_c8);
    if (iVar2 == 0) {
      uVar12 = local_b0 & 0xf000;
      if (*(char *)(DAT_00182408 + 0x9c) == '3') {
        if (uVar12 == 0x2000) {
LAB_00111cf4:
          pcVar14 = "Device number differs";
          if (DAT_001828c0 == local_a0) goto LAB_001118c9;
LAB_001118e6:
          uVar4 = dcgettext(0,pcVar14,5);
          FUN_00111120(&DAT_00182840,uVar4);
          break;
        }
      }
      else if (*(char *)(DAT_00182408 + 0x9c) == '4') {
        if (uVar12 == 0x6000) goto LAB_00111cf4;
      }
      else if (uVar12 == 0x1000) {
LAB_001118c9:
        if (((local_b0 ^ DAT_001828b0) & 0xfff) != 0) {
          pcVar14 = "Mode differs";
          goto LAB_001118e6;
        }
        break;
      }
      uVar4 = dcgettext(0,"File type differs",5);
      FUN_00111120(&DAT_00182840,uVar4);
    }
    else {
      piVar9 = __errno_location();
      if (*piVar9 == 2) {
        FUN_00133430(uVar4);
      }
      else {
        FUN_00133410();
      }
      FUN_00111120(&DAT_00182840,0);
    }
    break;
  case '5':
  case 'D':
    cVar1 = FUN_00120fc0(&DAT_00182840);
    if (cVar1 != '\0') {
      iVar2 = DAT_001829e8;
      if (DAT_001829e8 == 0) {
        iVar2 = FUN_00114950(DAT_001829d8,DAT_00182840,DAT_00182808);
        if (iVar2 < 0) {
          pcVar13 = FUN_00126370;
        }
        else {
          iVar3 = fstat(iVar2,(stat *)&DAT_00182898);
          if (iVar3 == 0) goto LAB_0011195f;
          close(iVar2);
          pcVar13 = FUN_00126460;
        }
        FUN_00126490(DAT_00182840,0,pcVar13);
      }
      else {
LAB_0011195f:
        DAT_001829e8 = iVar2;
        uVar4 = FUN_0011f810(&DAT_00182840);
        __s2 = (char *)FUN_0011fed0(uVar4);
        pcVar14 = DAT_001829d0;
        if (__s2 != (char *)0x0) {
          do {
            bVar11 = *pcVar14 + 0xbc;
            uVar6 = 1L << (bVar11 & 0x3f);
            do {
              do {
                if (*pcVar14 == '\0') {
LAB_00111d50:
                  if (*__s2 == '\0') goto LAB_0011177d;
                  goto LAB_00111d5b;
                }
              } while (0x15 < bVar11);
              if ((uVar6 & 0x114000) != 0) goto LAB_00111d50;
              if ((uVar6 & 0x200400) != 0) {
                pcVar10 = strchr("YN",(int)*__s2);
                if (pcVar10 == (char *)0x0) goto LAB_00111d5b;
                iVar2 = strcmp(pcVar14 + 1,__s2 + 1);
                goto joined_r0x00111d9f;
              }
            } while ((uVar6 & 1) == 0);
            iVar2 = strcmp(pcVar14,__s2);
joined_r0x00111d9f:
            if (iVar2 != 0) goto LAB_00111d5b;
            sVar7 = strlen(pcVar14);
            __s2 = __s2 + sVar7 + 1;
            pcVar14 = pcVar14 + sVar7 + 1;
          } while( true );
        }
        FUN_00110ff0(FUN_00110fe0);
      }
    }
    goto LAB_0011177d;
  case 'M':
    if (DAT_00182850 != '\0') goto LAB_0011177d;
    iVar2 = FUN_00125fc0(DAT_00182848,auStack_c8);
    if (iVar2 != 0) {
      piVar9 = __errno_location();
      if (*piVar9 == 2) {
        FUN_00133430(uVar4);
      }
      else {
        FUN_00133410();
      }
      FUN_00111120(&DAT_00182840,0);
      break;
    }
    if ((local_b0 & 0xf000) == 0x8000) {
      __offset = FUN_001231e0(DAT_00182408 + 0x171,0xc);
      if (((-1 < __offset) && (!SCARRY8(__offset,DAT_001828c8))) &&
         (local_98 == __offset + DAT_001828c8)) {
        iVar2 = __openat_2(DAT_0017f234,DAT_00182848,DAT_00182808);
        if (-1 < iVar2) {
          _Var8 = lseek(iVar2,__offset,0);
          if (_Var8 < 0) {
            FUN_00133200(DAT_00182848,__offset);
            FUN_00111120(&DAT_00182840,0);
          }
          else {
            FUN_00110ff0(FUN_00111250);
          }
          iVar2 = close(iVar2);
          if (iVar2 != 0) {
            FUN_00132d60(DAT_00182848);
          }
          break;
        }
        FUN_00132ea0(DAT_00182848);
        goto LAB_00111cb6;
      }
      goto LAB_00111cd0;
    }
    goto LAB_001117e5;
  case 'V':
    break;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00111f55:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
LAB_00111d5b:
  uVar4 = dcgettext(0,"Contents differ",5);
  FUN_00111120(&DAT_00182840,uVar4);
  goto LAB_0011177d;
}




// Function: verify_volume @ 0x11f60

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
  cVar1 = FUN_001335f0();
  if (cVar1 == '\0') {
    cVar1 = FUN_00130e80();
    if (cVar1 != '\0') {
LAB_00112110:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcgettext(0,"Archive contains transformed file names.",5);
      error(0,0,uVar5);
      goto LAB_0011213f;
    }
  }
  else {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar5 = dcgettext(0,"Archive contains file names with leading prefixes removed.",5);
    error(0,0,uVar5);
    cVar1 = FUN_00130e80();
    if (cVar1 != '\0') goto LAB_00112110;
LAB_0011213f:
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar5 = dcgettext(0,"Verification may fail to locate original files.",5);
    error(0,0,uVar5);
  }
  FUN_0011f6e0();
  if ((DAT_00182288 == 0) && (DAT_00182288 = FUN_001265a0(local_60,DAT_00182b98), DAT_00182b28 != 0)
     ) {
    FUN_001203b0();
  }
  fsync(DAT_00182a34);
  ioctl(DAT_00182a34,0x24b);
  local_5c = 1;
  local_60[0] = 2;
  if (DAT_00182a34 < 0x40000000) {
    iVar2 = ioctl(DAT_00182a34,0x40086d01,local_60);
  }
  else {
    iVar2 = FUN_001348b0(DAT_00182a34 + -0x40000000,0x40086d01,local_60);
  }
  if (iVar2 < 0) {
    piVar3 = __errno_location();
    if (*piVar3 == 5) {
      if (DAT_00182a34 < 0x40000000) {
        iVar2 = ioctl(DAT_00182a34,0x40086d01,local_60);
      }
      else {
        iVar2 = FUN_001348b0(DAT_00182a34 + -0x40000000,0x40086d01,local_60);
      }
      if (-1 < iVar2) goto LAB_00112044;
      if (DAT_00182a34 < 0x40000000) goto LAB_001122d0;
LAB_0011202c:
      lVar4 = FUN_00134720(DAT_00182a34 + -0x40000000,0,0);
    }
    else {
      if (0x3fffffff < DAT_00182a34) goto LAB_0011202c;
LAB_001122d0:
      lVar4 = lseek(DAT_00182a34,0,0);
    }
    if (lVar4 != 0) {
      FUN_001332b0(*DAT_00182838);
      goto LAB_00112291;
    }
  }
LAB_00112044:
  DAT_00182280 = 1;
  DAT_00181f40 = 0;
  FUN_0010f6e0();
  while( true ) {
    iVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
    if (iVar2 == 5) {
      iVar7 = 0;
      do {
        iVar7 = iVar7 + 1;
        FUN_0010ccd0(DAT_00182408);
        iVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
      } while (iVar2 == 5);
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcngettext(0,"VERIFY FAILURE: %d invalid header detected",
                         "VERIFY FAILURE: %d invalid headers detected",(long)iVar7,5);
      error(0,0,uVar5,iVar7);
      DAT_00182d58 = 2;
    }
    if (iVar2 == 4) break;
    if (iVar2 == 3) {
      FUN_0010ccd0(DAT_00182408);
      if (DAT_00182b51 == '\0') {
        iVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
        if (iVar2 == 3) break;
        if (((byte)DAT_00181b80 & 1) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar5 = FUN_0010cc60();
          uVar5 = FUN_00145370(uVar5,local_58);
          uVar6 = dcgettext(0,"A lone zero block at %s",5);
          error(0,0,uVar6,uVar5);
        }
      }
    }
    else {
      FUN_00122c60(DAT_00182408,&DAT_00182840,&DAT_00182400,1);
      FUN_00111680();
      FUN_0012fc00(&DAT_00182840);
    }
  }
  DAT_00181f40 = 1;
  DAT_00182280 = 0;
LAB_00112291:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_link @ 0x12380

ulong hash_link(ulong *param_1,ulong param_2)

{
  return (*param_1 ^ param_1[1]) % param_2;
}




// Function: compare_links @ 0x123a0

undefined8 compare_links(ulong *param_1,ulong *param_2)

{
  ulong uVar1;
  
  uVar1 = *param_1 ^ *param_2 | param_1[1] ^ param_2[1];
  return CONCAT71((int7)(uVar1 >> 8),uVar1 == 0);
}




// Function: to_chars_subst @ 0x123c0

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
      uVar1 = FUN_00145370(uVar6,local_b8);
    }
    else {
      uVar6 = 0xffffffffffffffff;
      uVar1 = FUN_00145370(0xffffffffffffffff,local_b8);
      pcVar8 = "0";
    }
  }
  else {
    if ((ulong)((param_7 + -1) * 8) < 0x40) {
      lVar7 = 1L << ((byte)((int)(param_7 + -1) << 3) & 0x3f);
      uVar6 = lVar7 - 1;
      uVar1 = FUN_00145370(uVar6,local_b8);
    }
    else {
      uVar6 = 0xffffffffffffffff;
      lVar7 = -0x8000000000000000;
      uVar1 = FUN_00145370(0xffffffffffffffff,local_b8);
    }
    lVar7 = FUN_00145370(lVar7,local_77);
    *(undefined1 *)(lVar7 + -1) = 0x2d;
    pcVar8 = (char *)(lVar7 + -1);
  }
  if (param_1 == 0) {
    lVar7 = FUN_00145370(param_3,&local_98);
  }
  else {
    lVar7 = FUN_00145370(-param_3,local_97);
    *(undefined1 *)(lVar7 + -1) = 0x2d;
    lVar7 = lVar7 + -1;
  }
  if (param_5 == (code *)0x0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar4 = dcgettext(0,"value %s out of %s range %s..%s",5);
    error(0,0,uVar4,lVar7,param_8,pcVar8,uVar1);
    uVar1 = 0;
    DAT_00182d58 = 2;
  }
  else {
    uVar2 = (*param_5)(&local_bc);
    local_bc = DAT_00182ba4 == 6 & local_bc;
    uVar5 = uVar6 & uVar2;
    if (local_bc != 0) {
      uVar5 = -(uVar6 & uVar2);
    }
    lVar3 = FUN_00145370(uVar5,local_57);
    if (local_bc != 0) {
      *(undefined1 *)(lVar3 + -1) = 0x2d;
      lVar3 = lVar3 + -1;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar4 = dcgettext(0,"value %s out of %s range %s..%s; substituting %s",5);
    error(0,0,uVar4,lVar7,param_8,pcVar8,uVar1,lVar3);
    uVar1 = FUN_00112680(local_bc,uVar5,param_4,0,param_6,param_7,param_8);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: open_failure_recover @ 0x12990

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
          FUN_0012fb80(lVar3);
          return uVar4;
        }
      }
    }
  }
  return 0;
}




// Function: unknown_file_error @ 0x12a20

void unknown_file_error(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  if (((byte)DAT_00181b80 & 0x20) == 0) {
    if (DAT_00182b52 != '\0') {
      return;
    }
    FUN_0012fd00(2);
    return;
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar1 = FUN_0014ce40(param_1);
  uVar2 = dcgettext(0,"%s: Unknown file type; file ignored",5);
  error(0,0,uVar2,uVar1);
  if (DAT_00182b52 != '\0') {
    return;
  }
  FUN_0012fd00(2);
  return;
}




// Function: ensure_slash @ 0x12ac0

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
      goto LAB_00112afc;
    }
    sVar1 = sVar4 - 1;
  } while (__s[sVar4 - 1] == '/');
  pcVar3 = __s + sVar4;
  lVar5 = sVar4 + 1;
LAB_00112afc:
  if (*pcVar3 != '/') {
    lVar2 = FUN_0014ece0(__s,sVar4 + 2);
    *param_1 = lVar2;
    pcVar3 = (char *)(lVar2 + sVar4);
  }
  *pcVar3 = '/';
  *(undefined1 *)(*param_1 + lVar5) = 0;
  return;
}




// Function: write_short_name @ 0x12da0

undefined8 * write_short_name(long param_1)

{
  char cVar1;
  long lVar2;
  undefined8 *puVar3;
  long lVar4;
  ulong uVar5;
  undefined8 *puVar6;
  byte bVar7;
  
  bVar7 = 0;
  puVar3 = (undefined8 *)FUN_0010d360();
  *puVar3 = 0;
  puVar3[0x3f] = 0;
  lVar4 = 0;
  puVar6 = (undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
  for (uVar5 = (ulong)(((int)puVar3 - (int)(undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
    *puVar6 = 0;
    puVar6 = puVar6 + (ulong)bVar7 * -2 + 1;
  }
  lVar2 = *(long *)(param_1 + 8);
  do {
    cVar1 = *(char *)(lVar2 + lVar4);
    *(char *)((long)puVar3 + lVar4) = cVar1;
    if (cVar1 == '\0') break;
    lVar4 = lVar4 + 1;
  } while (lVar4 != 100);
  if (DAT_00182ba4 == 2) {
    *(undefined1 *)((long)puVar3 + 99) = 0;
  }
  return puVar3;
}




// Function: add_exclusion_tag @ 0x12e10

void add_exclusion_tag(char *param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  size_t sVar3;
  
  puVar2 = (undefined8 *)FUN_0014ecc0(0x28);
  uVar1 = DAT_001822d0;
  *puVar2 = param_1;
  *(undefined4 *)(puVar2 + 2) = param_2;
  puVar2[3] = param_3;
  puVar2[4] = uVar1;
  sVar3 = strlen(param_1);
  DAT_001822d0 = puVar2;
  puVar2[1] = sVar3;
  return;
}




// Function: cachedir_file_p @ 0x12f10

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




// Function: write_eot @ 0x12fe0

void write_eot(void)

{
  undefined8 *puVar1;
  void *__s;
  size_t __n;
  ulong uVar2;
  undefined8 *puVar3;
  byte bVar4;
  
  bVar4 = 0;
  puVar1 = (undefined8 *)FUN_0010d360();
  *puVar1 = 0;
  puVar1[0x3f] = 0;
  puVar3 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
  for (uVar2 = (ulong)(((int)puVar1 - (int)(undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + (ulong)bVar4 * -2 + 1;
  }
  FUN_0010ccd0(puVar1);
  __s = (void *)FUN_0010d360();
  __n = FUN_0010cd10(__s);
  memset(__s,0,__n);
  FUN_0010ccd0(__s);
  return;
}




// Function: start_private_header @ 0x13050

undefined8 * start_private_header(long param_1,ulong param_2,ulong param_3)

{
  char cVar1;
  undefined8 *puVar2;
  long lVar3;
  char *pcVar4;
  char *pcVar5;
  undefined1 *puVar6;
  ulong uVar7;
  undefined8 *puVar8;
  byte bVar9;
  
  bVar9 = 0;
  puVar2 = (undefined8 *)FUN_0010d360();
  *puVar2 = 0;
  puVar2[0x3f] = 0;
  lVar3 = 0;
  puVar8 = (undefined8 *)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
  for (uVar7 = (ulong)(((int)puVar2 - (int)(undefined8 *)((ulong)(puVar2 + 1) & 0xfffffffffffffff8))
                       + 0x200U >> 3); uVar7 != 0; uVar7 = uVar7 - 1) {
    *puVar8 = 0;
    puVar8 = puVar8 + (ulong)bVar9 * -2 + 1;
  }
  do {
    cVar1 = *(char *)(param_1 + lVar3);
    *(char *)((long)puVar2 + lVar3) = cVar1;
    if (cVar1 == '\0') break;
    lVar3 = lVar3 + 1;
  } while (lVar3 != 100);
  if (DAT_00182ba4 == 2) {
    *(undefined1 *)((long)puVar2 + 99) = 0;
  }
  FUN_00112680(param_2 >> 0x3f,param_2,8,0,(long)puVar2 + 0x7c,0xc,"off_t");
  uVar7 = 0x1ffffffff;
  if (param_3 < 0x200000000) {
    uVar7 = param_3;
  }
  if ((long)param_3 < 0) {
    uVar7 = 0;
  }
  FUN_00112680(0,uVar7,8,0,puVar2 + 0x11,0xc,"time_t");
  if ((DAT_00182ba4 - 3U < 2) || (uVar7 = 0x81a4, DAT_00182ba4 == 6)) {
    uVar7 = 0x1a4;
  }
  *(undefined1 *)((long)puVar2 + 0x6b) = 0;
  pcVar4 = (char *)((long)puVar2 + 0x6a);
  do {
    pcVar5 = pcVar4 + -1;
    *pcVar4 = ((byte)uVar7 & 7) + 0x30;
    pcVar4 = pcVar5;
    uVar7 = uVar7 >> 3;
  } while ((char *)((long)puVar2 + 99) != pcVar5);
  *(undefined1 *)((long)puVar2 + 0x73) = 0;
  puVar6 = (undefined1 *)((long)puVar2 + 0x72);
  do {
    *puVar6 = 0x30;
    puVar6 = puVar6 + -1;
  } while (puVar6 != (undefined1 *)((long)puVar2 + 0x6b));
  *(undefined1 *)((long)puVar2 + 0x7b) = 0;
  puVar6 = (undefined1 *)((long)puVar2 + 0x7a);
  do {
    *puVar6 = 0x30;
    puVar6 = puVar6 + -1;
  } while (puVar6 != (undefined1 *)((long)puVar2 + 0x73));
  *(undefined4 *)((long)puVar2 + 0x101) = 0x61747375;
  *(undefined2 *)((long)puVar2 + 0x105) = 0x72;
  *(undefined2 *)((long)puVar2 + 0x107) = 0x3030;
  return puVar2;
}




// Function: simple_finish_header @ 0x13230

void simple_finish_header(byte *param_1)

{
  byte bVar1;
  byte *pbVar2;
  byte *pbVar3;
  int iVar4;
  ulong uVar5;
  
  iVar4 = 0;
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
    iVar4 = iVar4 + (uint)bVar1;
  } while (pbVar2 != param_1 + 0x200);
  uVar5 = (ulong)iVar4;
  if (0x3ffff < iVar4) {
    if ((DAT_00182ba4 & 0xfffffffb) == 2) {
      param_1[0x94] = 0x80;
      pbVar2 = param_1 + 0x9a;
      do {
        *pbVar2 = (byte)uVar5;
        pbVar2 = pbVar2 + -1;
        uVar5 = uVar5 >> 8;
      } while (pbVar2 != param_1 + 0x94);
      FUN_0010ccd0(param_1);
      return;
    }
    FUN_001123c0(0,0,uVar5,8,0,param_1 + 0x94,7);
    FUN_0010ccd0(param_1);
    return;
  }
  param_1[0x9a] = 0;
  pbVar2 = param_1 + 0x99;
  do {
    pbVar3 = pbVar2 + -1;
    *pbVar2 = ((byte)uVar5 & 7) + 0x30;
    pbVar2 = pbVar3;
    uVar5 = uVar5 >> 3;
  } while (param_1 + 0x93 != pbVar3);
  FUN_0010ccd0(param_1);
  return;
}




// Function: finish_header @ 0x13320

void finish_header(long param_1,long param_2)

{
  uint uVar1;
  undefined8 uVar2;
  
  if (((DAT_00182a54 != 0) && (*(byte *)(param_2 + 0x9c) != 0x4b)) &&
     ((uVar1 = *(byte *)(param_2 + 0x9c) - 0x4c, 0x2c < (byte)uVar1 ||
      ((0x100008000001U >> ((ulong)uVar1 & 0x3f) & 1) == 0)))) {
    DAT_00182400 = DAT_00182ba4;
    FUN_00123bc0(param_1,param_2);
  }
  if ((*(long *)(param_1 + 0x178) == 0) && (*(long *)(param_1 + 0x168) != 0)) {
    uVar2 = FUN_00112c70(0,param_1,param_2);
    FUN_00113230(uVar2);
    return;
  }
  FUN_00113230(param_2);
  return;
}




// Function: write_gnu_long_link @ 0x133d0

void write_gnu_long_link(undefined8 param_1,char *param_2,undefined1 param_3)

{
  char cVar1;
  long lVar2;
  ulong uVar3;
  long lVar4;
  long lVar5;
  void *__dest;
  ulong __n;
  ulong __n_00;
  char *__src;
  
  uVar3 = strlen(param_2);
  __n_00 = uVar3 + 1;
  lVar4 = FUN_00113050("././@LongLink",__n_00,0);
  if (DAT_00182ad2 == '\0') {
    if (DAT_001822a8 == 0) {
      FUN_00127eb0(0,&DAT_001822a8);
      FUN_00127f40(0,&DAT_001822a0);
    }
    lVar2 = DAT_001822a8;
    lVar5 = 0;
    do {
      cVar1 = *(char *)(lVar2 + lVar5);
      *(char *)(lVar4 + 0x109 + lVar5) = cVar1;
      if (cVar1 == '\0') break;
      lVar5 = lVar5 + 1;
    } while (lVar5 != 0x20);
    *(undefined1 *)(lVar4 + 0x128) = 0;
    lVar2 = DAT_001822a0;
    lVar5 = 0;
    do {
      cVar1 = *(char *)(lVar2 + lVar5);
      *(char *)(lVar4 + 0x129 + lVar5) = cVar1;
      if (cVar1 == '\0') break;
      lVar5 = lVar5 + 1;
    } while (lVar5 != 0x20);
    *(undefined1 *)(lVar4 + 0x148) = 0;
  }
  *(undefined1 *)(lVar4 + 0x9c) = param_3;
  *(undefined8 *)(lVar4 + 0x101) = 0x20207261747375;
  FUN_00113320(param_1,lVar4,0xffffffffffffffff);
  __dest = (void *)FUN_0010d360();
  __n = FUN_0010cd10(__dest);
  __src = param_2;
  if (__n < __n_00) {
    do {
      param_2 = __src + __n;
      memcpy(__dest,__src,__n);
      __n_00 = __n_00 - __n;
      FUN_0010ccd0((long)__dest + (__n - 1 & 0xfffffffffffffe00));
      __dest = (void *)FUN_0010d360();
      __n = FUN_0010cd10(__dest);
      __src = param_2;
    } while (__n < __n_00);
    uVar3 = __n_00 - 1;
  }
  memcpy(__dest,param_2,__n_00);
  memset((void *)((long)__dest + __n_00),0,__n - __n_00);
  FUN_0010ccd0((long)__dest + (uVar3 & 0xfffffffffffffe00));
  return;
}




// Function: start_header @ 0x13570

char * start_header(long param_1)

{
  char cVar1;
  uint uVar2;
  size_t sVar3;
  char *pcVar4;
  char *pcVar5;
  undefined8 uVar6;
  long lVar7;
  long lVar8;
  undefined8 uVar9;
  ulong uVar10;
  char *pcVar11;
  ulong uVar12;
  ulong uVar13;
  uint uVar14;
  uint uVar15;
  undefined8 *puVar16;
  undefined8 uVar17;
  uint uVar18;
  long in_FS_OFFSET;
  byte bVar19;
  char *pcStack_60;
  long local_58;
  long local_50;
  ulong local_48;
  long lStack_40;
  long local_30;
  
  bVar19 = 0;
  pcVar5 = *(char **)(param_1 + 8);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = 0;
  local_50 = 0;
  if (DAT_00182ba4 == 4) {
    cVar1 = thunk_FUN_00131a98(pcVar5);
    if (cVar1 != '\0') {
      pcVar5 = *(char **)(param_1 + 8);
      goto LAB_001135b7;
    }
    FUN_0011da70("path",param_1,0);
    pcVar4 = (char *)FUN_00112da0(param_1);
LAB_00113605:
    if (pcVar4 != (char *)0x0) {
      FUN_00124e10(*(undefined4 *)(param_1 + 0x74),param_1 + 0x74,&local_58);
      pcStack_60 = (char *)0x11362e;
      FUN_00124ee0(*(undefined4 *)(param_1 + 0x78),param_1 + 0x78,&local_50);
      if (DAT_00182b18 == 0) {
        uVar2 = *(uint *)(param_1 + 0x70);
      }
      else {
        uVar14 = *(uint *)(param_1 + 0x70);
        pcStack_60 = (char *)0x113663;
        uVar2 = FUN_00145ce0(uVar14,(uVar14 & 0xf000) == 0x4000,DAT_00182b14,DAT_00182b18,0);
        uVar2 = uVar14 & 0xfffff000 | uVar2;
        *(uint *)(param_1 + 0x70) = uVar2;
      }
      if ((DAT_00182ba4 & 0xfffffffd) == 1) {
        pcVar4[0x6b] = '\0';
        uVar12 = (ulong)(uVar2 & 0xfff);
        pcVar5 = pcVar4 + 0x6a;
        do {
          pcVar11 = pcVar5 + -1;
          *pcVar5 = ((byte)uVar12 & 7) + 0x30;
          uVar12 = uVar12 >> 3;
          pcVar5 = pcVar11;
        } while (pcVar4 + 99 != pcVar11);
      }
      else {
        if ((DAT_00182ba4 - 3 < 2) || (DAT_00182ba4 == 6)) {
          uVar12 = (ulong)(uVar2 & 0xfff);
        }
        else {
          uVar12 = (ulong)uVar2;
          if (0x1fffff < uVar2) {
            if (DAT_00182ba4 == 2) {
              pcVar4[100] = -0x80;
              pcVar5 = pcVar4 + 0x6b;
              do {
                *pcVar5 = (char)uVar12;
                pcVar5 = pcVar5 + -1;
                uVar12 = uVar12 >> 8;
              } while (pcVar4 + 100 != pcVar5);
            }
            else {
              pcStack_60 = "mode_t";
              FUN_001123c0(0,0,uVar12,4,0,pcVar4 + 100,8);
            }
            goto LAB_001136b9;
          }
        }
        pcVar4[0x6b] = '\0';
        pcVar5 = pcVar4 + 0x6a;
        do {
          pcVar11 = pcVar5 + -1;
          *pcVar5 = ((byte)uVar12 & 7) + 0x30;
          pcVar5 = pcVar11;
          uVar12 = uVar12 >> 3;
        } while (pcVar4 + 99 != pcVar11);
      }
LAB_001136b9:
      uVar2 = *(uint *)(param_1 + 0x74);
      uVar12 = (ulong)uVar2;
      if ((DAT_00182ba4 == 4) && (0x1fffff < uVar2)) {
        pcStack_60 = (char *)0x113b81;
        FUN_0011da70("uid",param_1,0);
        uVar12 = 0;
LAB_00113b83:
        pcVar4[0x73] = '\0';
        pcVar5 = pcVar4 + 0x72;
        do {
          pcVar11 = pcVar5 + -1;
          *pcVar5 = ((byte)uVar12 & 7) + 0x30;
          pcVar5 = pcVar11;
          uVar12 = uVar12 >> 3;
        } while (pcVar4 + 0x6b != pcVar11);
      }
      else {
        if (uVar2 < 0x200000) goto LAB_00113b83;
        if ((DAT_00182ba4 & 0xfffffffb) == 2) {
          pcVar4[0x6c] = -0x80;
          pcVar5 = pcVar4 + 0x73;
          do {
            *pcVar5 = (char)uVar12;
            pcVar5 = pcVar5 + -1;
            uVar12 = uVar12 >> 8;
          } while (pcVar4 + 0x6c != pcVar5);
        }
        else {
          pcStack_60 = "uid_t";
          cVar1 = FUN_001123c0(0,0,uVar12,4,0,pcVar4 + 0x6c,8);
          if (cVar1 == '\0') goto LAB_00113a56;
        }
      }
      uVar2 = *(uint *)(param_1 + 0x78);
      if ((uVar2 < 0x200000) || (DAT_00182ba4 != 4)) {
        uVar12 = (ulong)uVar2;
        if (uVar2 < 0x200000) goto LAB_00113bcb;
        if ((DAT_00182ba4 & 0xfffffffb) == 2) {
          pcVar4[0x74] = -0x80;
          pcVar5 = pcVar4 + 0x7b;
          do {
            *pcVar5 = (char)uVar12;
            pcVar5 = pcVar5 + -1;
            uVar12 = uVar12 >> 8;
          } while (pcVar4 + 0x74 != pcVar5);
          goto LAB_00113761;
        }
        pcStack_60 = "gid_t";
        cVar1 = FUN_001123c0(0,0,uVar12,4,0,pcVar4 + 0x74,8);
        if (cVar1 == '\0') goto LAB_00113a56;
        uVar12 = *(ulong *)(param_1 + 0x88);
        if (DAT_00182ba4 == 4) goto LAB_00113b42;
LAB_00113775:
        uVar2 = (uint)(uVar12 >> 0x3f);
      }
      else {
        pcStack_60 = (char *)0x113bc9;
        FUN_0011da70(&DAT_0016a634,param_1,0);
        uVar12 = 0;
LAB_00113bcb:
        pcVar4[0x7b] = '\0';
        pcVar5 = pcVar4 + 0x7a;
        do {
          pcVar11 = pcVar5 + -1;
          *pcVar5 = ((byte)uVar12 & 7) + 0x30;
          pcVar5 = pcVar11;
          uVar12 = uVar12 >> 3;
        } while (pcVar4 + 0x73 != pcVar11);
LAB_00113761:
        uVar12 = *(ulong *)(param_1 + 0x88);
        if (DAT_00182ba4 != 4) goto LAB_00113775;
LAB_00113b42:
        uVar2 = 0;
        if (uVar12 >> 0x21 != 0) {
          pcStack_60 = (char *)0x113b62;
          FUN_0011da70("size",param_1,0);
          uVar12 = 0;
        }
      }
      cVar1 = FUN_00112680(uVar2,uVar12,8,0,pcVar4 + 0x7c,0xc,"off_t");
      if (cVar1 != '\0') {
        if (DAT_00182af0 == 1) {
          local_48 = DAT_00182ae0;
          lStack_40 = DAT_00182ae8;
        }
        else if (DAT_00182af0 == 2) {
          if ((int)(((uint)(DAT_00182ae8 < *(long *)(param_1 + 0x100)) -
                    (uint)(*(long *)(param_1 + 0x100) < DAT_00182ae8)) +
                   ((uint)((long)DAT_00182ae0 < *(long *)(param_1 + 0xf8)) -
                   (uint)(*(long *)(param_1 + 0xf8) < (long)DAT_00182ae0)) * 2) < 1) {
            local_48 = *(ulong *)(param_1 + 0xf8);
            lStack_40 = *(long *)(param_1 + 0x100);
          }
          else {
            local_48 = DAT_00182ae0;
            lStack_40 = DAT_00182ae8;
          }
        }
        else if (DAT_00182af0 == 0) {
          local_48 = *(ulong *)(param_1 + 0xf8);
          lStack_40 = *(long *)(param_1 + 0x100);
        }
        if (DAT_00182ba4 == 4) {
          if ((local_48 >> 0x21 != 0) || (uVar12 = 0, lStack_40 != 0)) {
            pcStack_60 = (char *)0x113e5b;
            FUN_0011da70("mtime",param_1,&local_48);
            uVar12 = 0;
            if (local_48 >> 0x21 != 0) {
              local_48 = 0;
            }
          }
        }
        else {
          uVar12 = local_48 >> 0x3f;
        }
        pcVar5 = "time_t";
        cVar1 = FUN_00112680(uVar12,local_48,8,0,pcVar4 + 0x88,0xc);
        if (cVar1 != '\0') {
          if ((*(uint *)(param_1 + 0x70) & 0xb000) == 0x2000) {
            uVar12 = *(ulong *)(param_1 + 0x80);
            uVar14 = (uint)(uVar12 >> 0x20);
            uVar2 = uVar14 & 0xfffff000 | (uint)(uVar12 >> 8) & 0xfff;
            uVar15 = (uint)((uVar12 >> 0x14) << 8);
            uVar18 = uVar15 | (uint)(uVar12 & 0xff);
            if ((uVar2 < 0x200000) || (DAT_00182ba4 != 4)) {
              lVar7 = (long)(int)uVar2;
              uVar14 = uVar14 >> 0x1f;
            }
            else {
              FUN_0011da70("devmajor",param_1,0,uVar12 & 0xff,pcVar5,pcStack_60);
              uVar14 = 0;
              lVar7 = 0;
            }
            cVar1 = FUN_00112680(uVar14,lVar7,4,0,pcVar4 + 0x149,8,"major_t");
            if (cVar1 != '\0') {
              if ((DAT_00182ba4 == 4) && (0x1fffff < uVar18)) {
                FUN_0011da70("devminor",param_1,0);
                uVar15 = 0;
                lVar7 = 0;
              }
              else {
                lVar7 = (long)(int)uVar18;
                uVar15 = uVar15 >> 0x1f;
              }
              cVar1 = FUN_00112680(uVar15,lVar7,4,0,pcVar4 + 0x151,8);
joined_r0x00113d23:
              if (cVar1 != '\0') goto LAB_00113845;
            }
          }
          else {
            if ((DAT_00182ba4 & 0xfffffffb) == 2) {
LAB_00113845:
              if (DAT_00182ba4 == 4) {
                FUN_0011da70("atime",param_1,0);
                FUN_0011da70("ctime",param_1,0);
              }
              else if ((DAT_00182b50 != '\0') && ((DAT_00182ba4 & 0xfffffffb) == 2)) {
                FUN_00112680(*(ulong *)(param_1 + 0xe8) >> 0x3f,*(ulong *)(param_1 + 0xe8),8,0,
                             pcVar4 + 0x159,0xc,"time_t");
                FUN_00112680(*(ulong *)(param_1 + 0x108) >> 0x3f,*(ulong *)(param_1 + 0x108),8,0,
                             pcVar4 + 0x165,0xc,"time_t");
              }
              uVar2 = DAT_00182ba4;
              if (DAT_00182ba4 == 1) {
                pcVar4[0x9c] = '\0';
                goto LAB_00113a59;
              }
              pcVar4[0x9c] = '0';
              if (uVar2 < 5) {
                if (uVar2 < 3) {
                  if (uVar2 != 2) goto switchD_001135e4_caseD_0;
                  goto LAB_00113e02;
                }
                builtin_strncpy(pcVar4 + 0x101,"ustar",6);
                pcVar4[0x107] = '0';
                pcVar4[0x108] = '0';
              }
              else {
                if (uVar2 != 6) {
switchD_001135e4_caseD_0:
                    /* WARNING: Subroutine does not return */
                  abort();
                }
LAB_00113e02:
                builtin_strncpy(pcVar4 + 0x101,"ustar  ",8);
              }
              if (DAT_00182ad2 == '\0') {
                if (local_58 == 0) {
                  FUN_00127eb0(*(undefined4 *)(param_1 + 0x74),param_1 + 0x20);
                }
                else {
                  uVar6 = FUN_0014ee30();
                  *(undefined8 *)(param_1 + 0x20) = uVar6;
                }
                if (local_50 == 0) {
                  FUN_00127f40(*(undefined4 *)(param_1 + 0x78),param_1 + 0x28);
                }
                else {
                  uVar6 = FUN_0014ee30();
                  *(undefined8 *)(param_1 + 0x28) = uVar6;
                }
                pcVar5 = *(char **)(param_1 + 0x20);
                if (DAT_00182ba4 == 4) {
                  sVar3 = strlen(pcVar5);
                  if ((sVar3 < 0x21) && (cVar1 = thunk_FUN_00131a98(pcVar5), cVar1 != '\0')) {
                    pcVar5 = *(char **)(param_1 + 0x20);
                  }
                  else {
                    FUN_0011da70("uname",param_1,0);
                    pcVar5 = *(char **)(param_1 + 0x20);
                  }
                }
                lVar7 = 0;
                do {
                  cVar1 = pcVar5[lVar7];
                  pcVar4[lVar7 + 0x109] = cVar1;
                  if (cVar1 == '\0') break;
                  lVar7 = lVar7 + 1;
                } while (lVar7 != 0x20);
                pcVar4[0x128] = '\0';
                if (DAT_00182ba4 == 4) {
                  pcVar5 = *(char **)(param_1 + 0x28);
                  sVar3 = strlen(pcVar5);
                  if ((0x20 < sVar3) || (cVar1 = thunk_FUN_00131a98(pcVar5), cVar1 == '\0')) {
                    FUN_0011da70("gname",param_1,0);
                  }
                }
                lVar7 = *(long *)(param_1 + 0x28);
                lVar8 = 0;
                do {
                  cVar1 = *(char *)(lVar7 + lVar8);
                  pcVar4[lVar8 + 0x129] = cVar1;
                  if (cVar1 == '\0') break;
                  lVar8 = lVar8 + 1;
                } while (lVar8 != 0x20);
                pcVar4[0x148] = '\0';
                uVar2 = DAT_00182ba4;
              }
              if (uVar2 == 4) {
                if (0 < DAT_00182a9c) {
                  if (*(long *)(param_1 + 0x38) != 0) {
                    FUN_0011da70("SCHILY.acl.access",param_1,0);
                  }
                  if (*(long *)(param_1 + 0x48) != 0) {
                    FUN_0011da70("SCHILY.acl.default",param_1,0);
                  }
                }
                if ((0 < DAT_00182aa0) && (*(long *)(param_1 + 0x30) != 0)) {
                  FUN_0011da70("RHT.security.selinux",param_1,0);
                }
                if (0 < DAT_00182a98) {
                  local_48 = 0;
                  lVar7 = *(long *)(param_1 + 0x160);
                  if (*(long *)(param_1 + 0x158) != 0) {
                    do {
                      FUN_0011da70(*(undefined8 *)(lVar7 + local_48 * 0x18),param_1,&local_48);
                      local_48 = local_48 + 1;
                    } while (local_48 < *(ulong *)(param_1 + 0x158));
                  }
                }
              }
              goto LAB_00113a59;
            }
            cVar1 = FUN_00112680(0,0,4,0,pcVar4 + 0x149,8,"major_t");
            if (cVar1 != '\0') {
              cVar1 = FUN_00112680(0,0,4,0,pcVar4 + 0x151,8,"minor_t");
              goto joined_r0x00113d23;
            }
          }
        }
      }
    }
  }
  else {
LAB_001135b7:
    uVar2 = DAT_00182ba4;
    sVar3 = strlen(pcVar5);
    if (uVar2 == 2) {
      if (99 < sVar3) {
switchD_001135e4_caseD_6:
        FUN_001133d0(param_1,pcVar5,0x4c);
      }
LAB_001135fa:
      pcVar4 = (char *)FUN_00112da0(param_1);
      goto LAB_00113605;
    }
    if (sVar3 < 0x65) goto LAB_001135fa;
    switch(uVar2) {
    default:
      goto switchD_001135e4_caseD_0;
    case 1:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
        pcVar5 = *(char **)(param_1 + 8);
      }
      uVar6 = FUN_0014ce40(pcVar5);
      uVar9 = dcgettext(0,"%s: file name is too long (max %d); not dumped",5);
      uVar17 = 99;
      break;
    case 3:
    case 5:
      if (sVar3 < 0x101) {
        if (sVar3 < 0x9d) {
          uVar12 = sVar3 - 1;
          if (pcVar5[sVar3 - 1] != '/') goto LAB_00113d8f;
          cVar1 = pcVar5[sVar3 - 2];
          uVar12 = sVar3 - 2;
        }
        else {
          cVar1 = pcVar5[0x9b];
          uVar12 = 0x9b;
        }
        while (cVar1 != '/') {
LAB_00113d8f:
          uVar12 = uVar12 - 1;
          if (uVar12 == 0) goto LAB_00113d95;
          cVar1 = pcVar5[uVar12];
        }
        uVar13 = (sVar3 - 1) - uVar12;
        if (99 < uVar13 - 1) {
LAB_00113d95:
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar6 = FUN_0014ce40(pcVar5);
          uVar9 = dcgettext(0,"%s: file name is too long (cannot be split); not dumped",5);
          pcVar4 = (char *)0x0;
          error(0,0,uVar9,uVar6);
          DAT_00182d58 = 2;
          goto LAB_00113a59;
        }
        pcVar4 = (char *)FUN_0010d360();
        pcVar4[0] = '\0';
        pcVar4[1] = '\0';
        pcVar4[2] = '\0';
        pcVar4[3] = '\0';
        pcVar4[4] = '\0';
        pcVar4[5] = '\0';
        pcVar4[6] = '\0';
        pcVar4[7] = '\0';
        pcVar4[0x1f8] = '\0';
        pcVar4[0x1f9] = '\0';
        pcVar4[0x1fa] = '\0';
        pcVar4[0x1fb] = '\0';
        pcVar4[0x1fc] = '\0';
        pcVar4[0x1fd] = '\0';
        pcVar4[0x1fe] = '\0';
        pcVar4[0x1ff] = '\0';
        puVar16 = (undefined8 *)((ulong)(pcVar4 + 8) & 0xfffffffffffffff8);
        uVar10 = (ulong)(((int)pcVar4 - (int)puVar16) + 0x200U >> 3);
        for (; uVar10 != 0; uVar10 = uVar10 - 1) {
          *puVar16 = 0;
          puVar16 = puVar16 + (ulong)bVar19 * -2 + 1;
        }
        pcVar11 = pcVar4 + 0x159;
        uVar2 = (uint)uVar12;
        uVar10 = uVar12 & 0xffffffff;
        if (uVar2 < 8) {
          if ((uVar12 & 4) == 0) {
            if ((uVar2 != 0) && (*pcVar11 = *pcVar5, (uVar12 & 2) != 0)) {
              *(undefined2 *)(pcVar4 + uVar10 + 0x157) = *(undefined2 *)(pcVar5 + (uVar10 - 2));
            }
          }
          else {
            *(undefined4 *)pcVar11 = *(undefined4 *)pcVar5;
            *(undefined4 *)(pcVar4 + uVar10 + 0x155) = *(undefined4 *)(pcVar5 + (uVar10 - 4));
          }
        }
        else {
          *(undefined8 *)(pcVar4 + 0x159) = *(undefined8 *)pcVar5;
          *(undefined8 *)(pcVar4 + (uVar12 & 0xffffffff) + 0x151) =
               *(undefined8 *)(pcVar5 + ((uVar12 & 0xffffffff) - 8));
          lVar7 = (long)pcVar11 - (long)((ulong)(pcVar4 + 0x161) & 0xfffffffffffffff8);
          pcVar11 = pcVar5 + -lVar7;
          puVar16 = (undefined8 *)((ulong)(pcVar4 + 0x161) & 0xfffffffffffffff8);
          for (uVar10 = (ulong)(uVar2 + (int)lVar7 >> 3); uVar10 != 0; uVar10 = uVar10 - 1) {
            *puVar16 = *(undefined8 *)pcVar11;
            pcVar11 = pcVar11 + ((ulong)bVar19 * -2 + 1) * 8;
            puVar16 = puVar16 + (ulong)bVar19 * -2 + 1;
          }
        }
        pcVar5 = pcVar5 + uVar12 + 1;
        if (uVar13 < 8) {
          if ((uVar13 & 4) == 0) {
            if ((uVar13 != 0) && (*pcVar4 = *pcVar5, (uVar13 & 2) != 0)) {
              *(undefined2 *)(pcVar4 + (uVar13 - 2)) = *(undefined2 *)(pcVar5 + (uVar13 - 2));
            }
          }
          else {
            *(undefined4 *)pcVar4 = *(undefined4 *)pcVar5;
            *(undefined4 *)(pcVar4 + (uVar13 - 4)) = *(undefined4 *)(pcVar5 + (uVar13 - 4));
          }
        }
        else {
          puVar16 = (undefined8 *)((ulong)(pcVar4 + 8) & 0xfffffffffffffff8);
          *(undefined8 *)pcVar4 = *(undefined8 *)pcVar5;
          *(undefined8 *)(pcVar4 + (uVar13 - 8)) = *(undefined8 *)(pcVar5 + (uVar13 - 8));
          lVar7 = (long)pcVar4 - (long)puVar16;
          pcVar5 = pcVar5 + -lVar7;
          for (uVar12 = uVar13 + lVar7 >> 3; uVar12 != 0; uVar12 = uVar12 - 1) {
            *puVar16 = *(undefined8 *)pcVar5;
            pcVar5 = pcVar5 + ((ulong)bVar19 * -2 + 1) * 8;
            puVar16 = puVar16 + (ulong)bVar19 * -2 + 1;
          }
        }
        goto LAB_00113605;
      }
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar6 = FUN_0014ce40(pcVar5);
      uVar9 = dcgettext(0,"%s: file name is too long (max %d); not dumped",5);
      uVar17 = 0x100;
      break;
    case 4:
      FUN_0011da70("path",param_1,0);
      goto LAB_001135fa;
    case 6:
      goto switchD_001135e4_caseD_6;
    }
    error(0,0,uVar9,uVar6,uVar17);
    DAT_00182d58 = 2;
  }
LAB_00113a56:
  pcVar4 = (char *)0x0;
LAB_00113a59:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pcVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_long_link @ 0x14390

void write_long_link(long param_1)

{
  ulong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (DAT_00182ba4 < 7) {
    uVar1 = 1L << ((byte)DAT_00182ba4 & 0x3f);
    if ((uVar1 & 0x2a) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = FUN_0014ce40(*(undefined8 *)(param_1 + 0x18));
      uVar3 = dcgettext(0,"%s: link name is too long; not dumped",5);
      error(0,0,uVar3,uVar2);
      DAT_00182d58 = 2;
      return;
    }
    if ((uVar1 & 0x44) != 0) {
      FUN_001133d0(param_1,*(undefined8 *)(param_1 + 0x18),0x4b);
      return;
    }
    if (DAT_00182ba4 == 4) {
      FUN_0011da70("linkpath",param_1,0);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: dump_regular_file @ 0x14450

undefined8 dump_regular_file(int param_1,undefined8 *param_2)

{
  long lVar1;
  uint uVar2;
  undefined8 uVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  ulong uVar10;
  ulong uVar11;
  undefined8 *puVar12;
  long in_FS_OFFSET;
  byte bVar13;
  undefined1 local_58 [24];
  long local_40;
  
  bVar13 = 0;
  uVar11 = param_2[0x11];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010cc60();
  lVar4 = FUN_00113570(param_2);
  if (lVar4 == 0) {
    uVar3 = 2;
  }
  else {
    FUN_00113320(param_2,lVar4,uVar3);
    FUN_0010c640(param_2[1],param_2[0x11],param_2[0x11]);
LAB_001144b8:
    do {
      uVar10 = uVar11;
      if ((long)uVar10 < 1) {
LAB_00114500:
        uVar3 = 0;
        goto LAB_00114502;
      }
      lVar4 = FUN_0010d360();
      uVar5 = FUN_0010cd10(lVar4);
      if (uVar10 < uVar5) {
        if ((uVar10 & 0x1ff) != 0) {
          uVar2 = 0x200 - ((uint)uVar10 & 0x1ff);
          puVar12 = (undefined8 *)(lVar4 + uVar10);
          if (uVar2 < 8) {
            if ((uVar2 & 4) == 0) {
              if ((uVar2 != 0) && (*(undefined1 *)puVar12 = 0, (uVar2 & 2) != 0)) {
                *(undefined2 *)((long)puVar12 + ((ulong)uVar2 - 2)) = 0;
              }
            }
            else {
              *(undefined4 *)puVar12 = 0;
              *(undefined4 *)((long)puVar12 + ((ulong)uVar2 - 4)) = 0;
            }
          }
          else {
            *puVar12 = 0;
            *(undefined8 *)((long)puVar12 + ((ulong)uVar2 - 8)) = 0;
            uVar11 = (ulong)(uVar2 + ((int)puVar12 -
                                     (int)(undefined8 *)((ulong)(puVar12 + 1) & 0xfffffffffffffff8))
                            >> 3);
            puVar12 = (undefined8 *)((ulong)(puVar12 + 1) & 0xfffffffffffffff8);
            for (; uVar11 != 0; uVar11 = uVar11 - 1) {
              *puVar12 = 0;
              puVar12 = puVar12 + (ulong)bVar13 * -2 + 1;
            }
          }
        }
        uVar5 = uVar10;
        if (param_1 < 1) {
          FUN_0010ccd0(lVar4 + (uVar10 - 1 & 0xfffffffffffffe00));
          goto LAB_00114500;
        }
      }
      else if (param_1 < 1) {
        FUN_0010ccd0(lVar4 + (uVar5 - 1 & 0xfffffffffffffe00));
        uVar11 = uVar10 - uVar5;
        goto LAB_001144b8;
      }
      uVar6 = FUN_00125fe0(param_1,lVar4,uVar5);
      if (uVar6 == 0xffffffffffffffff) {
        FUN_001263a0(*param_2,param_2[0x11] - uVar10,uVar5);
        do {
          puVar9 = (undefined8 *)FUN_0010d360();
          uVar10 = uVar10 - 0x200;
          *puVar9 = 0;
          puVar9[0x3f] = 0;
          puVar12 = (undefined8 *)((ulong)(puVar9 + 1) & 0xfffffffffffffff8);
          for (uVar11 = (ulong)(((int)puVar9 -
                                (int)(undefined8 *)((ulong)(puVar9 + 1) & 0xfffffffffffffff8)) +
                                0x200U >> 3); uVar11 != 0; uVar11 = uVar11 - 1) {
            *puVar12 = 0;
            puVar12 = puVar12 + (ulong)bVar13 * -2 + 1;
          }
          FUN_0010ccd0(puVar9);
        } while (0 < (long)uVar10);
        uVar3 = 1;
        goto LAB_00114502;
      }
      uVar11 = uVar10 - uVar6;
      FUN_0010ccd0((uVar5 - 1 & 0xfffffffffffffe00) + lVar4);
    } while (uVar6 == uVar5);
    memset((void *)(lVar4 + uVar6),0,uVar5 - uVar6);
    if (((byte)DAT_00181b80 & 0x80) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar3 = FUN_00145370(uVar11,local_58);
      uVar7 = FUN_0014ce40(*param_2);
      uVar8 = dcngettext(0,"%s: File shrank by %s byte; padding with zeros",
                         "%s: File shrank by %s bytes; padding with zeros",uVar11,5);
      error(0,0,uVar8,uVar7,uVar3);
    }
    if (DAT_00182b52 == '\0') {
      FUN_0012fd00(1);
    }
    lVar4 = uVar10 - uVar5;
    if (0 < lVar4) {
      uVar11 = lVar4 - 1;
      lVar1 = lVar4 + -0x200;
      do {
        puVar9 = (undefined8 *)FUN_0010d360();
        lVar4 = lVar4 + -0x200;
        *puVar9 = 0;
        puVar9[0x3f] = 0;
        puVar12 = (undefined8 *)((ulong)(puVar9 + 1) & 0xfffffffffffffff8);
        for (uVar10 = (ulong)(((int)puVar9 -
                              (int)(undefined8 *)((ulong)(puVar9 + 1) & 0xfffffffffffffff8)) +
                              0x200U >> 3); uVar10 != 0; uVar10 = uVar10 - 1) {
          *puVar12 = 0;
          puVar12 = puVar12 + (ulong)bVar13 * -2 + 1;
        }
        FUN_0010ccd0(puVar9);
      } while (lVar4 != lVar1 - (uVar11 & 0xfffffffffffffe00));
    }
    uVar3 = 1;
  }
LAB_00114502:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: check_links @ 0x148b0

void check_links(void)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (DAT_001822c0 != 0) {
    for (lVar1 = FUN_001439d0(); lVar1 != 0; lVar1 = FUN_00143a20(DAT_001822c0,lVar1)) {
      if (*(long *)(lVar1 + 0x10) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar2 = FUN_0014d2d0(lVar1 + 0x18);
        uVar3 = dcgettext(0,"Missing links to %s.",5);
        error(0,0,uVar3,uVar2);
      }
    }
    return;
  }
  return;
}




// Function: subfile_open @ 0x14950

int subfile_open(long param_1,undefined8 param_2,undefined4 param_3)

{
  undefined4 uVar1;
  char cVar2;
  int iVar3;
  
  if (DAT_00182298 == '\0') {
    DAT_00182298 = '\x01';
    strerror(2);
    dcgettext(0,&DAT_0016a7d3,5);
  }
  while( true ) {
    uVar1 = DAT_0017f234;
    if (param_1 != 0) {
      uVar1 = *(undefined4 *)(param_1 + 0x1a8);
    }
    iVar3 = __openat_2(uVar1,param_2,param_3);
    if (-1 < iVar3) break;
    cVar2 = FUN_00112990(param_1);
    if (cVar2 == '\0') {
      return iVar3;
    }
  }
  return iVar3;
}




// Function: check_exclusion_tags @ 0x149e0

undefined4 check_exclusion_tags(undefined8 param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  char cVar2;
  int __fd;
  
  puVar1 = DAT_001822d0;
  do {
    if (puVar1 == (undefined8 *)0x0) {
      return 0;
    }
    __fd = FUN_00114950(param_1,*puVar1,DAT_00182808);
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




// Function: restore_parent_fd @ 0x14a80

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
    iVar2 = __openat_2(*(undefined4 *)(param_1 + 0x1a8),"..",DAT_00182804);
    if (iVar2 < 0) {
      piVar3 = __errno_location();
      iVar4 = -*piVar3;
      iVar2 = iVar4;
      if (((0 < *piVar3) && (__fd = __openat_2(DAT_0017f234,*puVar1,DAT_00182804), -1 < __fd)) &&
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
        goto LAB_00114aeb;
      }
    }
    *(int *)(puVar1 + 0x35) = iVar2;
  }
LAB_00114aeb:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dump_file0 @ 0x14c50

void dump_file0(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  long lVar3;
  bool bVar4;
  char cVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  undefined8 uVar10;
  int *piVar11;
  DIR *pDVar12;
  char *pcVar13;
  long lVar14;
  undefined8 uVar15;
  long lVar16;
  void *__src;
  ulong uVar17;
  void *__dest;
  ulong uVar18;
  char *__s;
  size_t sVar19;
  size_t sVar20;
  char *pcVar21;
  code *pcVar22;
  long lVar23;
  long lVar24;
  undefined8 *puVar25;
  long in_FS_OFFSET;
  bool bVar26;
  bool bVar27;
  byte bVar28;
  uint local_128;
  ulong local_f8;
  undefined8 local_e8;
  char *local_e0;
  stat local_d8;
  long local_40;
  
  iVar9 = DAT_0017f234;
  bVar28 = 0;
  lVar16 = param_1[0x33];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  bVar26 = lVar16 == 0;
  if (bVar26) {
    if (DAT_00182b40 == '\0') {
      FUN_00125120(param_1,param_3);
      uVar10 = FUN_00133640(param_3,0,DAT_00182b92);
      FUN_00125120(param_1 + 1,uVar10);
      FUN_00130e70(param_1 + 1,1);
      bVar4 = false;
    }
    else {
      iVar6 = FUN_0012df50(&DAT_00163e11,param_3);
      if (iVar6 == 0) goto LAB_00114d60;
      FUN_00125120(param_1,param_3);
      uVar10 = FUN_00133640(param_3,0,DAT_00182b92);
      FUN_00125120(param_1 + 1,uVar10);
      FUN_00130e70(param_1 + 1,1);
      bVar4 = false;
    }
LAB_00114dfd:
    iVar6 = fstatat(iVar9,param_2,(stat *)(param_1 + 0xb),DAT_00182800);
    if (iVar6 == 0) {
      uVar7 = *(uint *)(param_1 + 0xe) & 0xf000;
      if (uVar7 == 0x4000) {
LAB_00114e5e:
        iVar6 = FUN_00114950(lVar16,param_2,DAT_00182808);
        if (iVar6 < 0) {
          pcVar22 = FUN_00126370;
          goto LAB_00114d15;
        }
        *(int *)(param_1 + 0x35) = iVar6;
        iVar8 = fstat(iVar6,(stat *)(param_1 + 0xb));
        if (iVar8 != 0) goto LAB_00115210;
      }
      else {
        iVar6 = 0;
        if (uVar7 == 0x8000) {
          if (DAT_00182a30 == '\0') {
            if ((param_1[0x11] != 0) || ((*(uint *)(param_1 + 0xe) & 0x124) != 0x124))
            goto LAB_00114e5e;
          }
          else if ((DAT_00182a69 != '\0') && (DAT_00182a88 != '\0')) {
            uVar18 = param_1[0x11];
            uVar17 = uVar18;
            if ((long)uVar18 < 0) {
              uVar17 = uVar18 + 0x1ff;
            }
            lVar23 = (long)uVar17 >> 9;
            if (((uVar18 & 0x1ff) != 0) && (0x3fe < uVar18 + 0x1ff)) {
              lVar23 = lVar23 + 1;
            }
            if ((long)param_1[0x13] < lVar23) goto LAB_00114e5e;
          }
        }
      }
      cVar5 = DAT_00182b50;
      lVar23 = param_1[0x11];
      lVar24 = param_1[0x19];
      lVar1 = param_1[0x16];
      param_1[0x23] = lVar23;
      param_1[0x1d] = param_1[0x14];
      lVar14 = param_1[0x17];
      param_1[0x1e] = param_1[0x15];
      lVar2 = param_1[0x18];
      param_1[0x1f] = lVar1;
      param_1[0x20] = lVar14;
      param_1[0x21] = lVar2;
      param_1[0x22] = lVar24;
      if ((((cVar5 == '\x01') && (!bVar26)) || ((*(uint *)(param_1 + 0xe) & 0xf000) == 0x4000)) ||
         ((-1 < (int)(((uint)(DAT_00182b08 < lVar14) - (uint)(lVar14 < DAT_00182b08)) +
                     ((uint)(DAT_00182b00 < lVar1) - (uint)(lVar1 < DAT_00182b00)) * 2) ||
          ((DAT_00182b8c != 0 &&
           (-1 < (int)(((uint)(DAT_00182b08 < lVar24) - (uint)(lVar24 < DAT_00182b08)) +
                      ((uint)(DAT_00182b00 < lVar2) - (uint)(lVar2 < DAT_00182b00)) * 2))))))) {
        cVar5 = FUN_0012bd80(param_1);
        if (cVar5 == '\0') {
          uVar7 = *(uint *)(param_1 + 0xe) & 0xf000;
          local_128 = *(uint *)(param_1 + 0xe) & 0xf000;
          if (uVar7 == 0x4000) {
            FUN_00131ed0(iVar9,param_2,param_1,0,0);
            FUN_00132430(iVar9,param_2,param_1,iVar6);
            FUN_00132100(iVar9,param_2,param_1,iVar6);
            FUN_00112ac0(param_1);
            FUN_00112ac0(param_1 + 1);
            iVar9 = FUN_001149e0(param_1,&local_e8);
            if (iVar9 == 3) {
              uVar10 = dcgettext(0,"directory not dumped",5);
              if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_00112e70(*param_1,local_e8,uVar10);
                return;
              }
              goto LAB_00115f46;
            }
            do {
              pDVar12 = fdopendir(*(int *)(param_1 + 0x35));
              param_1[0x34] = pDVar12;
              if (pDVar12 != (DIR *)0x0) {
                uVar18 = (ulong)DAT_001827e4;
                pcVar13 = (char *)FUN_0014d730(pDVar12);
                if (pcVar13 != (char *)0x0) {
                  lVar1 = param_1[0x33];
                  uVar10 = FUN_0010cc60();
                  param_1[0x11] = 0;
                  lVar14 = FUN_00113570(param_1);
                  if (lVar14 == 0) goto LAB_00115153;
                  FUN_00117440(param_1);
                  if (DAT_00182b50 == '\0') {
                    *(undefined1 *)(lVar14 + 0x9c) = 0x35;
                    FUN_00113320(param_1,lVar14,uVar10);
                  }
                  else {
                    lVar3 = *(long *)(DAT_00182620 + 0x38);
                    if (DAT_00182ba4 == 4) {
                      *(undefined1 *)(lVar14 + 0x9c) = 0x35;
                      if (lVar3 != 0) {
                        uVar15 = FUN_0011ff00();
                        FUN_0011da70("GNU.dumpdir",param_1,uVar15);
                        FUN_00113320(param_1,lVar14,uVar10);
                        goto LAB_00115153;
                      }
                    }
                    else {
                      *(undefined1 *)(lVar14 + 0x9c) = 0x44;
                      if (lVar3 != 0) {
                        uVar10 = FUN_0010cc60();
                        __src = (void *)FUN_0011ff00(*(undefined8 *)(DAT_00182620 + 0x38));
                        uVar17 = FUN_0011f690(__src);
                        FUN_00112680(uVar17 >> 0x3f,uVar17,8,0,lVar14 + 0x7c,0xc,"off_t",uVar18);
                        FUN_00113320(param_1,lVar14,uVar10);
                        FUN_0010c640(param_1[1],uVar17,uVar17);
                        goto joined_r0x00115cdf;
                      }
                    }
                  }
                  if (DAT_00182ad4 != 0) {
                    if (((DAT_00182ad1 == '\0') || (lVar1 == 0)) ||
                       (*(long *)(param_1[0x33] + 0x58) == param_1[0xb])) {
                      iVar9 = FUN_001149e0(param_1,&local_e0);
                      if (iVar9 == 1) {
                        uVar10 = dcgettext(0,"contents not dumped",5);
                        FUN_00112e70(*param_1,local_e0,uVar10);
                        sVar19 = strlen((char *)*param_1);
                        sVar20 = strlen(local_e0);
                        pcVar21 = (char *)FUN_0014ecc0(sVar19 + 1 + sVar20);
                        strcpy(pcVar21,(char *)*param_1);
                        strcat(pcVar21,local_e0);
                        FUN_00114bb0(param_1,local_e0,pcVar21);
                        free(pcVar21);
                      }
                      else if (iVar9 == 2) {
                        uVar10 = dcgettext(0,"contents not dumped",5);
                        FUN_00112e70(*param_1,local_e0,uVar10);
                      }
                      else if (iVar9 == 0) {
                        __s = (char *)FUN_0014ee30(*param_1);
                        sVar19 = strlen(__s);
                        pcVar21 = pcVar13;
                        local_f8 = sVar19;
                        while (sVar20 = strlen(pcVar21), sVar20 != 0) {
                          uVar18 = sVar19 + sVar20;
                          if (local_f8 < uVar18) {
                            __s = (char *)FUN_0014ece0(__s,uVar18 + 1);
                            local_f8 = uVar18;
                          }
                          strcpy(__s + sVar19,pcVar21);
                          cVar5 = FUN_001176d0(__s,param_1);
                          if (cVar5 == '\0') {
                            FUN_00114bb0(param_1,pcVar21,__s);
                          }
                          pcVar21 = pcVar21 + sVar20 + 1;
                        }
                        free(__s);
                      }
                    }
                    else if ((DAT_00182a54 != 0) && ((DAT_00181b80._2_1_ & 4) != 0)) {
                      if (DAT_00182d50 != (code *)0x0) {
                        (*DAT_00182d50)();
                      }
                      uVar10 = FUN_0014ce40(*param_1);
                      uVar15 = dcgettext(0,"%s: file is on a different filesystem; not dumped",5);
                      error(0,0,uVar15,uVar10);
                    }
                  }
                  goto LAB_00115153;
                }
                break;
              }
              cVar5 = FUN_00112990(param_1);
            } while (cVar5 != '\0');
            FUN_00126400(*param_1);
            bVar27 = false;
LAB_0011516b:
            iVar6 = *(int *)(param_1 + 0x35);
            iVar9 = DAT_0017f234;
            if (lVar16 != 0) {
              iVar9 = *(int *)(lVar16 + 0x1a8);
            }
            if (bVar27) {
              if (-1 < iVar6) goto LAB_0011558b;
              piVar11 = __errno_location();
              *piVar11 = -iVar6;
              goto LAB_001151a8;
            }
          }
          else {
            if ((DAT_001822c0 != 0) &&
               ((DAT_001822c8 < (ulong)param_1[0xd] || (DAT_00182ab8 != '\0')))) {
              local_d8.st_ino = param_1[0xc];
              local_d8.st_dev = param_1[0xb];
              lVar16 = FUN_00143960(DAT_001822c0,&local_d8);
              if (lVar16 != 0) {
                pcVar13 = (char *)FUN_00133640(lVar16 + 0x18,1,DAT_00182b92);
                if (*(long *)(lVar16 + 0x10) != 0) {
                  *(long *)(lVar16 + 0x10) = *(long *)(lVar16 + 0x10) + -1;
                }
                uVar10 = FUN_0010cc60();
                FUN_00125120(param_1 + 3,pcVar13);
                bVar27 = DAT_00182ba4 != 2;
                sVar19 = strlen(pcVar13);
                if ((ulong)bVar27 + 99 < sVar19) {
                  FUN_00114390(param_1);
                }
                param_1[0x11] = 0;
                lVar16 = FUN_00113570(param_1);
                if (lVar16 != 0) {
                  lVar23 = 0;
                  do {
                    cVar5 = pcVar13[lVar23];
                    *(char *)(lVar16 + 0x9d + lVar23) = cVar5;
                    if (cVar5 == '\0') break;
                    lVar23 = lVar23 + 1;
                  } while (lVar23 != 100);
                  *(undefined1 *)(lVar16 + 0x9c) = 0x31;
                  FUN_00113320(param_1,lVar16,uVar10);
                  if (DAT_00182ab8 != '\0') {
                    FUN_00131150(*param_1,0);
                  }
                  goto LAB_00114d60;
                }
              }
              uVar7 = *(uint *)(param_1 + 0xe) & 0xf000;
            }
            if (uVar7 != 0x8000) {
              if (uVar7 == 0xa000) {
                lVar16 = FUN_0013a040(iVar9,param_2,param_1[0x11]);
                param_1[3] = lVar16;
                if (lVar16 == 0) {
                  piVar11 = __errno_location();
                  if (*piVar11 == 0xc) {
                    /* WARNING: Subroutine does not return */
                    FUN_00117020();
                  }
                  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_00115f46;
                  pcVar22 = FUN_001263d0;
                  goto LAB_00114d31;
                }
                FUN_00130e70(param_1 + 3,4);
                bVar26 = DAT_00182ba4 != 2;
                sVar19 = strlen((char *)param_1[3]);
                if ((ulong)bVar26 + 99 < sVar19) {
                  FUN_00114390(param_1);
                }
                FUN_00132430(iVar9,param_2,param_1,0);
                FUN_00132100(iVar9,param_2,param_1,0);
                uVar10 = FUN_0010cc60();
                param_1[0x11] = 0;
                lVar16 = FUN_00113570(param_1);
                if (lVar16 != 0) {
                  lVar23 = param_1[3];
                  lVar24 = 0;
                  do {
                    cVar5 = *(char *)(lVar23 + lVar24);
                    *(char *)(lVar16 + 0x9d + lVar24) = cVar5;
                    if (cVar5 == '\0') break;
                    lVar24 = lVar24 + 1;
                  } while (lVar24 != 100);
                  *(undefined1 *)(lVar16 + 0x9c) = 0x32;
                  FUN_00113320(param_1,lVar16,uVar10);
                  if (DAT_00182ab8 != '\0') {
                    FUN_00131150(param_3,0);
                  }
                  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    FUN_00112b40(param_1);
                    return;
                  }
                  goto LAB_00115f46;
                }
                goto LAB_00114d60;
              }
              if (uVar7 == 0x2000) {
                cVar5 = '3';
                FUN_00131ed0(iVar9,param_2,param_1,0,1);
                FUN_00132430(iVar9,param_2,param_1,0);
                FUN_00132100(iVar9,param_2,param_1,0);
LAB_00115863:
                if (DAT_00182ba4 != 1) {
                  uVar10 = FUN_0010cc60();
                  param_1[0x11] = 0;
                  lVar16 = FUN_00113570(param_1);
                  if (lVar16 != 0) {
                    *(char *)(lVar16 + 0x9c) = cVar5;
                    if (cVar5 != '6') {
                      uVar7 = (uint)((ulong)param_1[0x10] >> 0x20);
                      FUN_00112680(uVar7 >> 0x1f,
                                   (long)(int)(uVar7 & 0xfffff000 |
                                              (uint)((ulong)param_1[0x10] >> 8) & 0xfff),4,0,
                                   lVar16 + 0x149,8,"major_t");
                      uVar7 = (uint)(((ulong)param_1[0x10] >> 0x14) << 8);
                      FUN_00112680(uVar7 >> 0x1f,(long)(int)(uVar7 | (uint)param_1[0x10] & 0xff),4,0
                                   ,lVar16 + 0x151,8,"minor_t");
                    }
                    FUN_00113320(param_1,lVar16,uVar10);
                    if (DAT_00182ab8 != '\0') {
                      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                        FUN_00131150(param_3,0);
                        return;
                      }
                      goto LAB_00115f46;
                    }
                  }
                  goto LAB_00114d60;
                }
              }
              else {
                if (uVar7 == 0x6000) {
                  cVar5 = '4';
                  FUN_00131ed0(iVar9,param_2,param_1,0,1);
                  FUN_00132430(iVar9,param_2,param_1,0);
                  FUN_00132100(iVar9,param_2,param_1,0);
                  goto LAB_00115863;
                }
                if (uVar7 == 0x1000) {
                  cVar5 = '6';
                  FUN_00131ed0(iVar9,param_2,param_1,0,1);
                  FUN_00132430(iVar9,param_2,param_1,0);
                  FUN_00132100(iVar9,param_2,param_1,0);
                  goto LAB_00115863;
                }
                if (uVar7 == 0xc000) {
                  if (((byte)DAT_00181b80 & 0x20) != 0) {
                    if (DAT_00182d50 != (code *)0x0) {
                      (*DAT_00182d50)();
                    }
                    uVar10 = FUN_0014ce40(param_3);
                    pcVar13 = "%s: socket ignored";
                    goto LAB_001153b5;
                  }
                  goto LAB_00114d60;
                }
              }
              if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_00112a20(param_3);
                return;
              }
              goto LAB_00115f46;
            }
            FUN_00131ed0(iVar9,param_2,param_1,0,1);
            FUN_00132430(iVar9,param_2,param_1,iVar6);
            FUN_00132100(iVar9,param_2,param_1,iVar6);
            if ((iVar6 == 0) || (DAT_00182a88 == '\0')) {
LAB_00115555:
              uVar7 = FUN_00114450(iVar6,param_1);
LAB_0011556c:
              if (1 < uVar7) {
                if (uVar7 == 3) {
                    /* WARNING: Subroutine does not return */
                  abort();
                }
                goto LAB_001151bc;
              }
            }
            else {
              uVar18 = param_1[0x11];
              uVar17 = uVar18;
              if ((long)uVar18 < 0) {
                uVar17 = uVar18 + 0x1ff;
              }
              lVar16 = (long)uVar17 >> 9;
              if (((uVar18 & 0x1ff) != 0) && (0x3fe < uVar18 + 0x1ff)) {
                lVar16 = lVar16 + 1;
              }
              if (lVar16 <= (long)param_1[0x13]) goto LAB_00115555;
              uVar7 = FUN_0012af80(iVar6,param_1);
              if (uVar7 == 3) {
                uVar7 = FUN_00114450(iVar6,param_1);
                goto LAB_0011556c;
              }
              if (1 < uVar7) goto LAB_001151bc;
            }
            FUN_00112b40(param_1);
            if (uVar7 == 0) {
LAB_0011558b:
              if (iVar6 == 0) {
                if ((iVar9 < 0) && (bVar4)) {
                  piVar11 = __errno_location();
                  *piVar11 = -iVar9;
                  goto LAB_001151a8;
                }
                iVar8 = fstatat(iVar9,param_2,&local_d8,DAT_00182800);
              }
              else {
                iVar8 = fstat(iVar6,&local_d8);
              }
              if (iVar8 == 0) {
                if (((((uint)(lVar24 < local_d8.st_ctim.tv_nsec) -
                      (uint)(local_d8.st_ctim.tv_nsec < lVar24)) +
                      ((uint)(lVar2 < local_d8.st_ctim.tv_sec) -
                      (uint)(local_d8.st_ctim.tv_sec < lVar2)) * 2 == 0) ||
                    ((DAT_00182ab8 == '\x01' && (local_128 == 0x4000)))) &&
                   (local_d8.st_size <= lVar23)) {
                  if ((((DAT_00182b88 == 1) && (iVar6 != 0)) &&
                      ((lVar23 != 0 || (local_128 == 0x4000)))) &&
                     (iVar9 = FUN_00126100(iVar6,iVar9,param_2,param_1[0x1d],param_1[0x1e]),
                     iVar9 != 0)) {
                    FUN_001334b0(param_3);
                  }
                }
                else {
                  if (((byte)DAT_00181b80 & 0x10) != 0) {
                    if (DAT_00182d50 != (code *)0x0) {
                      (*DAT_00182d50)();
                    }
                    uVar10 = FUN_0014ce40(param_3);
                    uVar15 = dcgettext(0,"%s: file changed as we read it",5);
                    error(0,0,uVar15,uVar10);
                  }
                  FUN_0012fd00(1);
                }
                cVar5 = FUN_0012fb80(param_1);
                if ((cVar5 != '\0') && (DAT_00182ab8 != '\0')) {
                  FUN_00131150(param_3,local_128 == 0x4000);
                }
                goto LAB_00114d60;
              }
LAB_001151a8:
              FUN_00126490(param_3,bVar26,FUN_00126460);
            }
          }
LAB_001151bc:
          FUN_0012fb80(param_1);
        }
        else if ((DAT_00181b80._1_1_ & 4) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar10 = FUN_0014ce40(param_3);
          pcVar13 = "%s: file is the archive; not dumped";
          goto LAB_001153b5;
        }
      }
      else if (((cVar5 == '\0') && (DAT_00182a54 != 0)) && ((DAT_00181b80._1_1_ & 1) != 0)) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar10 = FUN_0014ce40(param_3);
        pcVar13 = "%s: file is unchanged; not dumped";
LAB_001153b5:
        uVar15 = dcgettext(0,pcVar13,5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          error(0,0,uVar15,uVar10);
          return;
        }
        goto LAB_00115f46;
      }
LAB_00114d60:
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
      goto LAB_00115f46;
    }
LAB_00115210:
    pcVar22 = FUN_00126460;
  }
  else {
    iVar9 = *(int *)(lVar16 + 0x1a8);
    if ((DAT_00182b40 != '\0') && (iVar6 = FUN_0012df50(&DAT_00163e11,param_3), iVar6 == 0))
    goto LAB_00114d60;
    FUN_00125120(param_1,param_3);
    uVar10 = FUN_00133640(param_3,0,DAT_00182b92);
    FUN_00125120(param_1 + 1,uVar10);
    FUN_00130e70(param_1 + 1,1);
    if (-1 < iVar9) {
      bVar4 = true;
      goto LAB_00114dfd;
    }
    piVar11 = __errno_location();
    pcVar22 = FUN_00126370;
    *piVar11 = -iVar9;
  }
LAB_00114d15:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_00114d31:
    FUN_00126490(param_3,bVar26,pcVar22);
    return;
  }
LAB_00115f46:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
joined_r0x00115cdf:
  if (0 < (long)uVar17) {
    __dest = (void *)FUN_0010d360();
    uVar18 = FUN_0010cd10(__dest);
    if (uVar18 <= uVar17) goto LAB_00115d5e;
    if ((uVar17 & 0x1ff) != 0) {
      uVar7 = 0x200 - ((uint)uVar17 & 0x1ff);
      puVar25 = (undefined8 *)((long)__dest + uVar17);
      if (uVar7 < 8) {
        if ((uVar7 & 4) == 0) {
          if ((uVar7 != 0) && (*(undefined1 *)puVar25 = 0, (uVar7 & 2) != 0)) {
            *(undefined2 *)((long)puVar25 + ((ulong)uVar7 - 2)) = 0;
          }
        }
        else {
          *(undefined4 *)puVar25 = 0;
          *(undefined4 *)((long)puVar25 + ((ulong)uVar7 - 4)) = 0;
        }
      }
      else {
        *puVar25 = 0;
        *(undefined8 *)((long)puVar25 + ((ulong)uVar7 - 8)) = 0;
        uVar18 = (ulong)(uVar7 + ((int)puVar25 -
                                 (int)(undefined8 *)((ulong)(puVar25 + 1) & 0xfffffffffffffff8)) >>
                        3);
        puVar25 = (undefined8 *)((ulong)(puVar25 + 1) & 0xfffffffffffffff8);
        for (; uVar18 != 0; uVar18 = uVar18 - 1) {
          *puVar25 = 0;
          puVar25 = puVar25 + (ulong)bVar28 * -2 + 1;
        }
      }
    }
    memcpy(__dest,__src,uVar17);
    FUN_0010ccd0((long)__dest + (uVar17 - 1 & 0xfffffffffffffe00));
  }
  goto LAB_00115153;
LAB_00115d5e:
  uVar17 = uVar17 - uVar18;
  memcpy(__dest,__src,uVar18);
  __src = (void *)((long)__src + uVar18);
  FUN_0010ccd0((long)__dest + (uVar18 - 1 & 0xfffffffffffffe00));
  goto joined_r0x00115cdf;
LAB_00115153:
  FUN_00114a80(param_1);
  free(pcVar13);
  bVar27 = true;
  goto LAB_0011516b;
}




// Function: create_archive @ 0x16020

void create_archive(void)

{
  undefined8 uVar1;
  char cVar2;
  int iVar3;
  long lVar4;
  size_t sVar5;
  void *__dest;
  char *pcVar6;
  long in_FS_OFFSET;
  size_t local_3e0;
  ulong local_3c0;
  long local_3b8 [11];
  stat local_360 [2];
  int local_210;
  undefined1 local_1f8 [408];
  long *local_60;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001822c8 = 0;
  if (DAT_00182630 != 2) {
    DAT_001822c8 = (ulong)(DAT_00182b69 ^ 1);
  }
  FUN_0010f710(1);
  FUN_0010c660();
  if (DAT_00182b50 == '\0') {
    while (lVar4 = FUN_001281f0(1), lVar4 != 0) {
      cVar2 = FUN_001176d0(lVar4,0);
      if (cVar2 == '\0') {
        FUN_0012fb40(local_1f8);
        local_60 = (long *)0x0;
        FUN_00114c50(local_1f8,lVar4,lVar4);
        FUN_0012fc00(local_1f8);
      }
    }
  }
  else {
    local_3c0 = 0;
    FUN_00128cf0();
    while (lVar4 = FUN_00129260(), lVar4 != 0) {
      cVar2 = FUN_001176d0(*(undefined8 *)(lVar4 + 0x10),0);
      if (cVar2 == '\0') {
        uVar1 = *(undefined8 *)(lVar4 + 0x10);
        FUN_0012fb40(local_1f8);
        local_60 = (long *)0x0;
        FUN_00114c50(local_1f8,uVar1,uVar1);
        FUN_0012fc00(local_1f8);
      }
    }
    FUN_001292e0();
    __dest = (void *)0x0;
    while (lVar4 = FUN_00129260(), lVar4 != 0) {
      cVar2 = FUN_001176d0(*(undefined8 *)(lVar4 + 0x10),0);
      if (cVar2 == '\0') {
        pcVar6 = *(char **)(lVar4 + 0x10);
        sVar5 = strlen(pcVar6);
        if (local_3c0 <= sVar5) {
          do {
            __dest = (void *)FUN_0014ed20(__dest,&local_3c0);
          } while (local_3c0 <= sVar5);
          pcVar6 = *(char **)(lVar4 + 0x10);
        }
        memcpy(__dest,pcVar6,sVar5);
        local_3e0 = sVar5;
        if (*(char *)((long)__dest + (sVar5 - 1)) != '/') {
          local_3e0 = sVar5 + 1;
          *(undefined1 *)((long)__dest + sVar5) = 0x2f;
        }
        FUN_0012fb40(local_3b8);
        pcVar6 = (char *)FUN_0011fed0(*(undefined8 *)(lVar4 + 0x38));
        if (pcVar6 != (char *)0x0) {
          for (; cVar2 = *pcVar6, cVar2 != '\0'; pcVar6 = pcVar6 + sVar5 + 1) {
            sVar5 = strlen(pcVar6);
            if (cVar2 == 'Y') {
              if (local_3b8[0] == 0) {
                iVar3 = __openat_2(DAT_0017f234,*(undefined8 *)(lVar4 + 0x10),DAT_00182804);
                if (iVar3 < 0) {
                  FUN_00126490(*(undefined8 *)(lVar4 + 0x10),*(long *)(lVar4 + 0x40) == 0,
                               FUN_00126370);
                  break;
                }
                local_210 = iVar3;
                iVar3 = fstat(iVar3,local_360);
                if (iVar3 != 0) {
                  FUN_00126490(*(undefined8 *)(lVar4 + 0x10),*(long *)(lVar4 + 0x40) == 0,
                               FUN_00126460);
                  break;
                }
                local_3b8[0] = FUN_0014ee30(*(undefined8 *)(lVar4 + 0x10));
              }
              while (local_3c0 < local_3e0 + sVar5) {
                __dest = (void *)FUN_0014ed20(__dest,&local_3c0);
              }
              strcpy((char *)((long)__dest + local_3e0),pcVar6 + 1);
              FUN_0012fb40(local_1f8);
              local_60 = local_3b8;
              FUN_00114c50(local_1f8,pcVar6 + 1,__dest);
              if (DAT_00182b28 != 0) {
                FUN_0011f780(local_3b8);
              }
              FUN_0012fc00(local_1f8);
            }
          }
        }
        FUN_0012fc00(local_3b8);
      }
    }
    free(__dest);
  }
  FUN_00112fe0();
  FUN_0010d8d0();
  FUN_00131110();
  if (DAT_00182b28 == 0) {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00120e40();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_record @ 0x16620

void write_record(int param_1)

{
  undefined8 uVar1;
  
  uVar1 = DAT_00181f58;
  DAT_00181f58 = DAT_001822e8;
  if (DAT_001822e0 == '\0') {
    if (DAT_001822d8 + DAT_00181f30 != DAT_00181f38) {
      FUN_00116410();
    }
    FUN_0010f6f0();
  }
  else {
    DAT_00182a34 = 1;
    FUN_0010f6f0();
    DAT_00182a34 = 0;
  }
  DAT_00181f58 = uVar1;
  if (((param_1 != 0) && (DAT_001822e0 == '\0')) && (DAT_00181f38 != DAT_001822d8 + DAT_00181f30)) {
    FUN_00116410();
  }
  DAT_001822e4 = 0;
  return;
}




// Function: write_recent_blocks @ 0x166d0

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
      iVar1 = DAT_001822e4 + 1;
      puVar2 = (undefined8 *)((long)DAT_001822e4 * 0x200 + DAT_001822e8);
      DAT_001822e4 = iVar1;
      *puVar2 = *puVar6;
      puVar2[0x3f] = puVar6[0x3f];
      lVar3 = (long)puVar2 - (long)((ulong)(puVar2 + 1) & 0xfffffffffffffff8);
      bVar7 = iVar1 == DAT_00182ba0;
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
    FUN_00116620(1);
  } while (param_2 != lVar5);
  return;
}




// Function: fatal_exit @ 0x16fd0

void fatal_exit(void)

{
  undefined8 uVar1;
  
  if (DAT_001822f0 != (code *)0x0) {
    (*DAT_001822f0)();
  }
  uVar1 = dcgettext(0,"Error is not recoverable: exiting now",5);
  error(2,0,uVar1);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: xalloc_die @ 0x17020

void xalloc_die(void)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"memory exhausted",5);
  error(0,0,"%s",uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: hg_initfn @ 0x17060

void hg_initfn(undefined4 *param_1)

{
  if (param_1 == (undefined4 *)0x0) {
    param_1 = &DAT_001822f8;
  }
  *param_1 = 0x8000000;
  return;
}




// Function: bzr_addfn @ 0x17080

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
    FUN_00140c90(param_1,param_2,param_3);
    return;
  }
  return;
}




// Function: git_addfn @ 0x17130

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
    FUN_00140c90(param_1,param_2,param_3);
    return;
  }
  return;
}




// Function: hg_addfn @ 0x171b0

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
        pcVar9 = (char *)FUN_0014ecc0(sVar8);
        memcpy(pcVar9,param_2,sVar8 - 1);
        pcVar9[sVar8 - 1] = '\0';
        FUN_00140820(param_1,pcVar9);
      }
      uVar5 = param_3 & 0xf7ffffff;
      if (*param_4 == 0x8000000) {
        uVar5 = param_3 & 0xefffffff;
      }
      FUN_00140c90(param_1,pcVar9,*param_4 | uVar5);
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




// Function: cvs_addfn @ 0x17330

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
  iVar2 = FUN_001375a0(param_2,&local_118,0x844);
  if (iVar2 == 0) {
    if (local_118 != 0) {
      uVar3 = 0;
      do {
        lVar1 = uVar3 * 8;
        uVar3 = uVar3 + 1;
        FUN_00140c90(param_1,*(undefined8 *)(local_110 + lVar1),param_3);
      } while (uVar3 < local_118);
    }
    FUN_00135800(&local_118);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: excfile_add @ 0x173d0

void excfile_add(char *param_1,undefined4 param_2)

{
  size_t sVar1;
  undefined8 *puVar2;
  
  sVar1 = strlen(param_1);
  puVar2 = (undefined8 *)FUN_0014ecc0(sVar1 + 0x10);
  *puVar2 = 0;
  *(undefined4 *)(puVar2 + 1) = param_2;
  strcpy((char *)((long)puVar2 + 0xc),param_1);
  if (DAT_00182300 != (undefined8 *)0x0) {
    *DAT_00182300 = puVar2;
    DAT_00182300 = puVar2;
    return;
  }
  DAT_00182308 = puVar2;
  DAT_00182300 = puVar2;
  return;
}




// Function: info_attach_exclist @ 0x17440

void info_attach_exclist(long param_1)

{
  char *__file;
  char *__s1;
  long lVar1;
  undefined8 *puVar2;
  int iVar3;
  FILE *__stream;
  undefined8 uVar4;
  undefined *puVar5;
  undefined8 *puVar6;
  int *piVar7;
  undefined4 uVar8;
  long *plVar9;
  undefined **ppuVar10;
  undefined8 *puVar11;
  undefined8 *local_40;
  
  if (*(long *)(param_1 + 0x1b0) != 0) {
    return;
  }
  local_40 = (undefined8 *)0x0;
  if (DAT_00182308 != (long *)0x0) {
    plVar9 = DAT_00182308;
    puVar11 = (undefined8 *)0x0;
    do {
      while( true ) {
        __file = (char *)((long)plVar9 + 0xc);
        iVar3 = faccessat(*(int *)(param_1 + 0x1a8),__file,0,0);
        puVar6 = puVar11;
        puVar2 = local_40;
        if (iVar3 == 0) break;
LAB_00117490:
        local_40 = puVar2;
        plVar9 = (long *)*plVar9;
        puVar11 = puVar6;
        if (plVar9 == (long *)0x0) goto LAB_001175af;
      }
      iVar3 = FUN_00114950(param_1,__file,0);
      if (iVar3 == -1) {
        FUN_00132ea0(__file);
        goto LAB_00117490;
      }
      __stream = fdopen(iVar3,"r");
      if (__stream == (FILE *)0x0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar4 = dcgettext(0,"%s: fdopen failed",5);
        piVar7 = __errno_location();
        error(0,*piVar7,uVar4);
        DAT_00182d58 = 2;
        close(iVar3);
        goto LAB_00117490;
      }
      uVar4 = FUN_00140920();
      ppuVar10 = &PTR_s__cvsignore_0017f160;
      __s1 = PTR_s__cvsignore_0017f160;
      while ((__s1 != (char *)0x0 && (iVar3 = strcmp(__s1,__file), iVar3 != 0))) {
        __s1 = ppuVar10[5];
        ppuVar10 = ppuVar10 + 5;
      }
      puVar5 = ppuVar10[4];
      if ((code *)ppuVar10[3] != (code *)0x0) {
        puVar5 = (undefined *)(*(code *)ppuVar10[3])(puVar5);
        ppuVar10[4] = puVar5;
      }
      iVar3 = FUN_00141000(ppuVar10[2],uVar4,__stream,0x50000001,10,puVar5);
      if (iVar3 != 0) {
        piVar7 = __errno_location();
        iVar3 = *piVar7;
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar4 = FUN_0014ce40(__file);
        error(0,iVar3,"%s",uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      fclose(__stream);
      puVar6 = (undefined8 *)FUN_0014ecc0(0x20);
      lVar1 = plVar9[1];
      puVar6[3] = uVar4;
      uVar8 = 0;
      if ((int)lVar1 != 0) {
        uVar8 = *(undefined4 *)(ppuVar10 + 1);
      }
      *(undefined4 *)(puVar6 + 2) = uVar8;
      puVar6[1] = puVar11;
      *puVar6 = 0;
      puVar2 = puVar6;
      if (puVar11 == (undefined8 *)0x0) goto LAB_00117490;
      plVar9 = (long *)*plVar9;
      *puVar11 = puVar6;
      puVar11 = puVar6;
    } while (plVar9 != (long *)0x0);
  }
LAB_001175af:
  *(undefined8 **)(param_1 + 0x1b0) = local_40;
  return;
}




// Function: info_free_exclist @ 0x17680

void info_free_exclist(long param_1)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  
  __ptr = *(undefined8 **)(param_1 + 0x1b0);
  while (__ptr != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*__ptr;
    FUN_00140930(__ptr[3]);
    free(__ptr);
    __ptr = puVar1;
  }
  *(undefined8 *)(param_1 + 0x1b0) = 0;
  return;
}




// Function: excluded_name @ 0x176d0

char excluded_name(char *param_1,long param_2)

{
  undefined8 *puVar1;
  char cVar2;
  void *__ptr;
  uint uVar3;
  char *pcVar4;
  char local_39;
  
  local_39 = FUN_00140ae0(DAT_00182b60,param_1);
  if ((local_39 == '\0') && (param_2 != 0)) {
    uVar3 = 0;
    __ptr = (void *)0x0;
    pcVar4 = (char *)0x0;
    do {
      for (puVar1 = *(undefined8 **)(param_2 + 0x1b0); puVar1 != (undefined8 *)0x0;
          puVar1 = (undefined8 *)*puVar1) {
        if ((uVar3 & *(uint *)(puVar1 + 2)) == 0) {
          cVar2 = FUN_00140ae0(puVar1[3],param_1);
          if (cVar2 == '\0') {
            if (pcVar4 == (char *)0x0) {
              cVar2 = *param_1;
              pcVar4 = param_1;
              while ((cVar2 == '.' && (pcVar4[1] == '/'))) {
                pcVar4 = pcVar4 + 2;
                cVar2 = *pcVar4;
              }
            }
            cVar2 = FUN_00140ae0(puVar1[3],pcVar4);
            if (cVar2 == '\0') {
              if (__ptr == (void *)0x0) {
                __ptr = (void *)FUN_00140250(param_1);
                cVar2 = FUN_00140ae0(puVar1[3],__ptr);
              }
              else {
                cVar2 = FUN_00140ae0(puVar1[3],__ptr);
              }
              if (cVar2 == '\0') goto LAB_00117740;
            }
          }
          local_39 = '\x01';
          goto LAB_001177a5;
        }
LAB_00117740:
      }
      param_2 = *(long *)(param_2 + 0x198);
      uVar3 = 2;
    } while (param_2 != 0);
LAB_001177a5:
    free(__ptr);
  }
  return local_39;
}




// Function: exclude_vcs_ignores @ 0x17810

void exclude_vcs_ignores(void)

{
  undefined **ppuVar1;
  undefined *puVar2;
  
  if (PTR_s__cvsignore_0017f160 != (undefined *)0x0) {
    ppuVar1 = &PTR_s__cvsignore_0017f160;
    puVar2 = PTR_s__cvsignore_0017f160;
    do {
      ppuVar1 = ppuVar1 + 5;
      FUN_001173d0(puVar2,0);
      puVar2 = *ppuVar1;
    } while (puVar2 != (undefined *)0x0);
    return;
  }
  return;
}




// Function: find_delayed_link_source @ 0x17850

long * find_delayed_link_source(char *param_1)

{
  int iVar1;
  int *piVar2;
  long *plVar3;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  plVar3 = DAT_00182318;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182318 != (long *)0x0) {
    iVar1 = fstatat(DAT_0017f234,param_1,&local_b8,0x100);
    if (iVar1 == 0) {
      do {
        if ((plVar3[1] == local_b8.st_dev) && (plVar3[2] == local_b8.st_ino)) goto LAB_001178c0;
        plVar3 = (long *)*plVar3;
      } while (plVar3 != (long *)0x0);
    }
    else {
      piVar2 = __errno_location();
      if (*piVar2 != 2) {
        plVar3 = (long *)0x0;
        FUN_00133410(param_1);
        goto LAB_001178c0;
      }
    }
  }
  plVar3 = (long *)0x0;
LAB_001178c0:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return plVar3;
}




// Function: check_time @ 0x17910

void check_time(undefined8 param_1,long param_2,long param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_68;
  long local_60;
  undefined1 local_58 [40];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 < 0) {
    if ((DAT_00181b80._1_1_ & 0x80) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar1 = FUN_00123240(param_2,param_3,1);
      uVar2 = dcgettext(0,"%s: implausibly old time stamp %s",5);
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        error(0,0,uVar2,param_1,uVar1);
        return;
      }
      goto LAB_00117ad8;
    }
  }
  else if ((int)(((uint)(param_3 < DAT_00182a18) - (uint)(DAT_00182a18 < param_3)) +
                ((uint)(param_2 < DAT_00182a10) - (uint)(DAT_00182a10 < param_2)) * 2) < 0) {
    FUN_00143220(&local_68);
    if ((int)(((uint)(param_3 < local_60) - (uint)(local_60 < param_3)) +
             ((uint)(param_2 < local_68) - (uint)(local_68 < param_2)) * 2) < 0) {
      lVar5 = param_2 - local_68;
      lVar4 = param_3 - local_60;
      if (lVar4 < 0) {
        lVar4 = lVar4 + 1000000000;
        lVar5 = lVar5 + -1;
      }
      if ((DAT_00181b80._1_1_ & 0x80) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar1 = FUN_001257a0(lVar5,lVar4,local_58);
        uVar2 = FUN_00123240(param_2,param_3,1);
        uVar3 = dcgettext(0,"%s: time stamp %s is %s s in the future",5);
        error(0,0,uVar3,param_1,uVar2,uVar1);
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_00117ad8:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mark_after_links @ 0x17ae0

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
    iVar1 = FUN_00125fc0(param_1[0x14],&local_b8);
    if (iVar1 == 0) {
      param_1[1] = local_b8;
      param_1[2] = local_b0;
      param_1 = (undefined8 *)*param_1;
    }
    else {
      FUN_00133410(param_1[0x14]);
      param_1 = (undefined8 *)*param_1;
    }
  } while ((param_1 != (undefined8 *)0x0) && (*(char *)(param_1 + 0xb) == '\0'));
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: delay_set_stat @ 0x17b80

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
  for (puVar10 = DAT_00182320; puVar10 != (undefined8 *)0x0; puVar10 = (undefined8 *)*puVar10) {
    __s1 = (char *)puVar10[0x14];
    iVar8 = strcmp(__s1,param_1);
    if (iVar8 == 0) {
      if (*(char *)(puVar10 + 10) != '\0') {
        iVar8 = fstatat(DAT_0017f234,__s1,&local_d8,*(int *)((long)puVar10 + 0x54));
        if (iVar8 == 0) {
          puVar10[1] = local_d8.st_dev;
          puVar10[2] = local_d8.st_ino;
        }
        else {
          FUN_00133410(puVar10[0x14]);
        }
      }
      *(undefined4 *)(puVar10 + 3) = param_5;
      if (param_2 == 0) goto LAB_00117d58;
      goto LAB_00117c16;
    }
  }
  puVar10 = (undefined8 *)FUN_0014ecc0(0xa8);
  puVar6 = puVar10;
  *puVar10 = DAT_00182320;
  DAT_00182320 = puVar6;
  puVar10[0x13] = sVar9;
  uVar11 = FUN_0014ee30(param_1);
  *(undefined1 *)(puVar10 + 0xb) = 0;
  puVar10[0x14] = uVar11;
  if (param_2 == 0) {
    *(undefined4 *)(puVar10 + 3) = param_5;
LAB_00117d58:
    *(undefined4 *)(puVar10 + 9) = param_3;
    *(undefined1 *)(puVar10 + 10) = 1;
    *(undefined4 *)((long)puVar10 + 0x4c) = param_4;
    uVar1 = DAT_00182580;
    *(undefined4 *)((long)puVar10 + 0x54) = param_6;
    *(undefined4 *)((long)puVar10 + 0x5c) = uVar1;
    puVar10[0xc] = 0;
    puVar10[0xd] = 0;
    puVar10[0xe] = 0;
    puVar10[0xf] = 0;
    puVar10[0x10] = 0;
    puVar10[0x12] = 0;
    puVar10[0x11] = 0;
    cVar7 = FUN_00125b20(param_1);
    goto joined_r0x00117dc1;
  }
  puVar10[1] = *(undefined8 *)(param_2 + 0x58);
  puVar10[2] = *(undefined8 *)(param_2 + 0x60);
  *(undefined4 *)(puVar10 + 3) = param_5;
LAB_00117c16:
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
  uVar1 = DAT_00182580;
  *(undefined4 *)((long)puVar10 + 0x54) = param_6;
  *(undefined4 *)((long)puVar10 + 0x5c) = uVar1;
  puVar10[0xc] = 0;
  puVar10[5] = uVar2;
  puVar10[6] = uVar3;
  puVar10[7] = uVar4;
  puVar10[8] = uVar5;
  FUN_00125120(puVar10 + 0xc,uVar11);
  if (*(long *)(param_2 + 0x38) == 0) {
    lVar12 = *(long *)(param_2 + 0x48);
    puVar10[0xd] = 0;
    puVar10[0xe] = 0;
    if (lVar12 != 0) goto LAB_00117cb1;
LAB_00117d3e:
    puVar10[0xf] = 0;
    uVar11 = 0;
  }
  else {
    uVar11 = FUN_0014edf0(*(long *)(param_2 + 0x38),*(long *)(param_2 + 0x40) + 1);
    lVar12 = *(long *)(param_2 + 0x48);
    puVar10[0xd] = uVar11;
    puVar10[0xe] = *(undefined8 *)(param_2 + 0x40);
    if (lVar12 == 0) goto LAB_00117d3e;
LAB_00117cb1:
    uVar11 = FUN_0014edf0(lVar12,*(long *)(param_2 + 0x50) + 1);
    puVar10[0xf] = uVar11;
    uVar11 = *(undefined8 *)(param_2 + 0x50);
  }
  puVar10[0x10] = uVar11;
  FUN_0011d820(param_2,puVar10 + 0x12,puVar10 + 0x11);
  cVar7 = FUN_00125b20(param_1);
joined_r0x00117dc1:
  if (cVar7 == '\0') {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00117ae0(puVar10);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: make_directories @ 0x17ec0

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
LAB_00117f00:
    cVar1 = pcVar5[1];
    pcVar6 = pcVar5;
    if (cVar1 != '\0') {
      while( true ) {
        uVar8 = DAT_0018232c;
        pcVar5 = pcVar6 + 1;
        if ((((param_1 == pcVar5) || (cVar1 != '/')) || (*pcVar6 == '/')) ||
           ((*pcVar6 == '.' &&
            (((pcVar5 == param_1 + 1 || (pcVar6[-1] == '/')) ||
             ((pcVar6[-1] == '.' && ((pcVar5 == param_1 + 2 || (pcVar6[-2] == '/'))))))))))
        goto LAB_00117f00;
        *pcVar5 = '\0';
        uVar7 = ~uVar8 & 0x1ff;
        uVar8 = uVar7;
        if (DAT_00182330 == '\0') {
          uVar8 = uVar7 | 0xc0;
        }
        iVar2 = mkdirat(DAT_0017f234,param_1,uVar8);
        if (iVar2 == 0) break;
        piVar4 = __errno_location();
        iVar2 = *piVar4;
        if ((iVar2 != 0x11) && (iVar3 = fstatat(DAT_0017f234,param_1,&local_d8,0), iVar3 != 0)) {
          *piVar4 = iVar2;
          FUN_00132e40(param_1);
          *pcVar5 = '/';
          goto LAB_00117fb3;
        }
        *pcVar5 = '/';
        cVar1 = pcVar6[2];
        pcVar6 = pcVar5;
        if (cVar1 == '\0') goto LAB_00117fb0;
      }
      FUN_00117b80(param_1,0,~DAT_00182328 & uVar8,0x1ff,uVar7,0x100);
      FUN_00123c30(param_1,(int)pcVar5 - (int)param_1,uVar7);
      *param_2 = 1;
      *pcVar5 = '/';
      goto LAB_00117f00;
    }
  }
LAB_00117fb0:
  iVar3 = 0;
LAB_00117fb3:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: fd_chmod @ 0x18100

undefined8
fd_chmod(undefined4 param_1,undefined8 param_2,undefined4 param_3,int param_4,int param_5)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  iVar1 = FUN_00118090();
  if (iVar1 == 0) goto LAB_00118168;
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  if (iVar1 == 1) {
LAB_00118198:
    *piVar2 = iVar1;
    uVar3 = 0xffffffff;
  }
  else {
    if ((param_4 == 0) || (param_5 == 0x32)) {
      if ((iVar1 != 0) && ((param_5 != 0x32 || ((iVar1 != 0x26 && (iVar1 != 0x5f))))))
      goto LAB_00118198;
    }
    else {
      if ((iVar1 == 0x5f) || (iVar1 == 0x26)) {
        iVar1 = FUN_00118090(param_1,param_2,param_3,0);
        if (iVar1 == 0) goto LAB_00118168;
        iVar1 = *piVar2;
      }
      if (iVar1 != 0) goto LAB_00118198;
    }
LAB_00118168:
    uVar3 = 0;
  }
  return uVar3;
}




// Function: set_stat @ 0x181b0

void set_stat(char *param_1,long param_2,int param_3,__mode_t param_4,uint param_5,char param_6,
                 char param_7,int param_8)

{
  __uid_t __owner;
  __gid_t __group;
  int iVar1;
  int *piVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  long in_FS_OFFSET;
  uint local_ec;
  stat local_d8;
  long local_40;
  
  iVar5 = (int)param_6;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_ec = ~param_5;
  if (DAT_00182a68 == '\0') {
    uVar4 = 0x1ff;
    if (param_7 == '\0') {
      if (DAT_00182b50 == '\0') {
        local_d8.st_ino = 0x3ffffffe;
      }
      else {
        local_d8.st_dev = *(__dev_t *)(param_2 + 0xe8);
        local_d8.st_ino = *(__ino_t *)(param_2 + 0xf0);
      }
      local_d8.st_nlink = *(__nlink_t *)(param_2 + 0xf8);
      local_d8._24_8_ = *(undefined8 *)(param_2 + 0x100);
      iVar1 = FUN_001412d0(param_3,DAT_0017f234,param_1,&local_d8,param_8);
      if (iVar1 == 0) {
        if (DAT_00182b50 != '\0') {
          FUN_00117910(param_1,local_d8.st_dev,local_d8.st_ino);
        }
        FUN_00117910(param_1,local_d8.st_nlink,local_d8._24_8_);
      }
      else {
        if (param_6 == '2') {
          piVar2 = __errno_location();
          if ((*piVar2 == 0x5f) || (*piVar2 == 0x26)) goto LAB_0011836d;
        }
        FUN_001334b0(param_1);
      }
LAB_0011836d:
      if (DAT_00182aa8 < 1) goto LAB_00118420;
LAB_0011837b:
      __owner = *(__uid_t *)(param_2 + 0x74);
      __group = *(__gid_t *)(param_2 + 0x78);
      if (param_3 < 0) {
LAB_001184d8:
        iVar1 = fchownat(DAT_0017f234,param_1,__owner,__group,param_8);
        if (iVar1 != 0) goto LAB_001183d7;
LAB_00118505:
        if (((local_ec | param_4) & 0x49) != 0) {
          param_5 = param_5 & ~(param_4 & 0xc00);
          local_ec = ~param_5;
        }
      }
      else {
        iVar1 = fchown(param_3,__owner,__group);
        if (iVar1 == 0) goto LAB_00118505;
        piVar2 = __errno_location();
        if ((*piVar2 == 0x26) || (*piVar2 == 0x5f)) goto LAB_001184d8;
LAB_001183d7:
        if (param_6 == '2') {
          piVar2 = __errno_location();
          if ((*piVar2 == 0x26) || (*piVar2 == 0x5f)) goto LAB_001183fe;
        }
        FUN_00132ce0(param_1,__owner,__group);
      }
LAB_001183fe:
      uVar4 = 0xfff;
      if (DAT_00182aa4 < 1) {
        uVar4 = 0x1ff;
      }
    }
  }
  else {
    if (0 < DAT_00182aa8) {
      uVar4 = 0x1ff;
      if (param_7 != '\0') goto LAB_0011821d;
      goto LAB_0011837b;
    }
LAB_00118420:
    if ((DAT_00182aa4 < 1) || (uVar4 = 0xfff, param_7 != '\0')) {
      uVar4 = 0x1ff;
    }
  }
LAB_0011821d:
  uVar3 = ~DAT_00182328 & *(uint *)(param_2 + 0x70);
  if (((uVar3 ^ param_4 | local_ec) & uVar4) != 0) {
    if ((~(param_5 | uVar4) & 0xfff) != 0) {
      if (param_3 < 0) {
        iVar1 = fstatat(DAT_0017f234,param_1,&local_d8,param_8);
      }
      else {
        iVar1 = fstat(param_3,&local_d8);
      }
      param_4 = local_d8.st_mode;
      if (iVar1 != 0) {
        FUN_00133410(param_1);
        goto LAB_0011825e;
      }
    }
    uVar4 = (uVar3 ^ param_4 & 0xfff) & uVar4;
    if (uVar4 != 0) {
      uVar4 = param_4 & 0xfff ^ uVar4;
      iVar1 = FUN_00118100(param_3,param_1,uVar4,param_8,iVar5);
      if (iVar1 != 0) {
        FUN_00132c30(param_1,uVar4);
      }
    }
  }
LAB_0011825e:
  FUN_00132570(param_2,param_1,iVar5,1);
  FUN_00131f50(param_2,param_1,iVar5);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  FUN_001324c0(param_2,param_1,iVar5);
  return;
}




// Function: apply_delayed_links @ 0x185e0

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
  __ptr = DAT_00182318;
  do {
    if (__ptr == (undefined8 *)0x0) {
      DAT_00182318 = (undefined8 *)0x0;
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    __from_00 = (long *)0x0;
    __from = (char *)(__ptr + 0x14);
    FUN_00126300(*(undefined4 *)(__ptr + 0xb));
    plVar4 = (long *)__ptr[0xc];
    if (plVar4 != (long *)0x0) {
      do {
        while( true ) {
          __file = plVar4 + 1;
          iVar3 = fstatat(DAT_0017f234,(char *)__file,&local_288,0x100);
          if ((((iVar3 == 0) && (local_288.st_dev == __ptr[1])) && (local_288.st_ino == __ptr[2]))
             && (iVar3 = (int)~(uint)((ulong)__ptr[3] >> 0x20) >> 0x1f,
                CONCAT31((int3)((uint)-iVar3 >> 8),(long)__ptr[4] < -1) +
                ((int)~(uint)((ulong)__ptr[4] >> 0x20) >> 0x1f) +
                ((uint)((long)__ptr[3] < -1) + iVar3) * 2 == 0)) break;
LAB_00118640:
          plVar4 = (long *)*plVar4;
          if (plVar4 == (long *)0x0) goto LAB_00118730;
        }
        iVar3 = unlinkat(DAT_0017f234,(char *)__file,0);
        if (iVar3 != 0) {
          FUN_00133490(__file);
          goto LAB_00118640;
        }
        if ((__from_00 != (long *)0x0) &&
           (iVar3 = linkat(DAT_0017f234,(char *)__from_00,DAT_0017f234,(char *)__file,0), iVar3 == 0
           )) goto LAB_00118640;
        if (*(char *)(__ptr + 5) != '\0') {
          iVar3 = symlinkat(__from,DAT_0017f234,(char *)__file);
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
            FUN_001181b0(__file,local_1f8,0xffffffff,0,0,0x32,0,0x100);
            __from_00 = __file;
          }
          else {
            FUN_00133370(__from,__file);
          }
          goto LAB_00118640;
        }
        iVar3 = linkat(DAT_0017f234,__from,DAT_0017f234,(char *)__file,0);
        if (iVar3 == 0) goto LAB_00118640;
        FUN_00132dc0(__from,__file);
        plVar4 = (long *)*plVar4;
      } while (plVar4 != (long *)0x0);
LAB_00118730:
      puVar2 = (undefined8 *)__ptr[0xc];
      while (puVar2 != (undefined8 *)0x0) {
        puVar1 = (undefined8 *)*puVar2;
        free(puVar2);
        puVar2 = puVar1;
      }
    }
    FUN_0011d580(__ptr[0x13],__ptr[0x12]);
    free((void *)__ptr[0xd]);
    puVar2 = (undefined8 *)*__ptr;
    free(__ptr);
    __ptr = puVar2;
  } while( true );
}




// Function: maybe_recoverable @ 0x18980

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
  if (*param_3 != '\0') goto LAB_001189f6;
  puVar4 = (uint *)__errno_location();
  uVar1 = *puVar4;
  if (uVar1 == 0x11) {
LAB_00118a42:
    if (DAT_00182b34 == 5) {
      uVar5 = 2;
      if ((DAT_00181b80._2_1_ & 0x10) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar5 = dcgettext(0,"%s: skipping existing file",5);
        error(0,0,uVar5,param_1);
        uVar5 = 2;
      }
      goto LAB_001189f6;
    }
    if (DAT_00182b34 < 6) {
      if (2 < DAT_00182b34) {
        uVar5 = 0;
        if (DAT_00182b34 == 4) goto LAB_001189f6;
        goto LAB_00118a25;
      }
LAB_00118a80:
      iVar3 = FUN_00126a40(param_1,0);
      if (iVar3 < 1) goto LAB_00118a25;
    }
    else {
      if ((DAT_00182b34 == 6) && (cVar2 = FUN_001188c0(param_1), cVar2 == '\0')) goto LAB_00118a80;
LAB_00118a25:
      iVar3 = FUN_00117ec0(param_1,param_3);
      if ((iVar3 != 0) || (*param_3 == '\0')) goto LAB_001189f0;
    }
    uVar5 = 1;
  }
  else {
    if ((int)uVar1 < 0x12) {
      if (uVar1 == 2) goto LAB_00118a25;
    }
    else if ((((uVar1 == 0x28) || ((uVar1 & 0xffffffbf) == 0x1f)) && (param_2 != '\0')) &&
            ((DAT_00182b34 == 2 && (DAT_00182b69 == '\0')))) {
      pcVar6 = strchr(param_1,0x2f);
      if (pcVar6 == (char *)0x0) goto LAB_00118a80;
      iVar3 = FUN_00125fc0(param_1,auStack_c8);
      if (iVar3 == 0) goto LAB_00118a42;
    }
LAB_001189f0:
    *puVar4 = uVar1;
    uVar5 = 0;
  }
LAB_001189f6:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: extract_fifo @ 0x18b30

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
  uVar3 = DAT_001828b0;
  if (0 < DAT_00182aa8) {
    uVar3 = DAT_001828b0 & 0xffffffc0;
  }
  while( true ) {
    iVar1 = mkfifoat(DAT_0017f234,param_1,uVar3 & 0x1ff);
    if (iVar1 == 0) {
      FUN_001181b0(param_1,&DAT_00182840,0xffffffff,~DAT_00182328 & uVar3 & 0x1ff,0x1ff,(int)param_2
                   ,0,0x100);
      uVar2 = 0;
      goto LAB_00118ba2;
    }
    iVar1 = FUN_00118980(param_1,0,&local_31);
    if (iVar1 == 0) break;
    if (iVar1 == 2) {
      uVar2 = 0;
LAB_00118ba2:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  FUN_00132e60(param_1);
  uVar2 = 0xffffffff;
  goto LAB_00118ba2;
}




// Function: extract_node @ 0x18c10

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
  uVar3 = DAT_001828b0;
  if (0 < DAT_00182aa8) {
    uVar3 = DAT_001828b0 & 0xffffffc0;
  }
  while( true ) {
    iVar1 = mknodat(DAT_0017f234,param_1,uVar3 & 0x61ff,DAT_001828c0);
    if (iVar1 == 0) {
      FUN_001181b0(param_1,&DAT_00182840,0xffffffff,~DAT_00182328 & uVar3 & 0x61ff,0x1ff,
                   (int)param_2,0,0x100);
      uVar2 = 0;
      goto LAB_00118c89;
    }
    iVar1 = FUN_00118980(param_1,0,&local_31);
    if (iVar1 == 0) break;
    if (iVar1 == 2) {
      uVar2 = 0;
LAB_00118c89:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  FUN_00132e80(param_1);
  uVar2 = 0xffffffff;
  goto LAB_00118c89;
}




// Function: create_placeholder_file @ 0x18d00

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
  char *pcVar13;
  undefined4 uVar14;
  long in_FS_OFFSET;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    iVar6 = openat(DAT_0017f234,param_1,0xc1,0);
    if (-1 < iVar6) {
      iVar7 = fstat(iVar6,&local_c8);
      if (iVar7 != 0) {
        uVar14 = 0xffffffff;
        FUN_00133410(param_1);
        close(iVar6);
        goto LAB_00118d6f;
      }
      iVar6 = close(iVar6);
      if (iVar6 != 0) {
        uVar14 = 0xffffffff;
        FUN_00132d60(param_1);
        goto LAB_00118d6f;
      }
      sVar10 = strlen(DAT_00182858);
      puVar11 = (undefined8 *)FUN_0014ecc0(sVar10 + 0xa1);
      if (param_4 == (undefined8 *)0x0) {
        puVar12 = puVar11;
        *puVar11 = DAT_00182318;
        DAT_00182318 = puVar12;
      }
      else {
        uVar1 = *param_4;
        *param_4 = puVar11;
        *puVar11 = uVar1;
      }
      puVar11[3] = 0xffffffffffffffff;
      puVar11[4] = 0xffffffffffffffff;
      puVar11[1] = local_c8.st_dev;
      *(char *)(puVar11 + 5) = param_2;
      puVar11[2] = local_c8.st_ino;
      uVar5 = DAT_00182940;
      uVar4 = DAT_00182938;
      uVar3 = uRam0000000000182930;
      uVar1 = _DAT_00182928;
      if (param_2 != '\0') {
        *(undefined4 *)((long)puVar11 + 0x2c) = DAT_001828b0;
        uVar2 = DAT_001828b4;
        puVar11[7] = uVar1;
        puVar11[8] = uVar3;
        puVar11[6] = uVar2;
        puVar11[9] = uVar4;
        puVar11[10] = uVar5;
      }
      *(undefined4 *)(puVar11 + 0xb) = DAT_00182580;
      sVar10 = strlen(param_1);
      puVar12 = (undefined8 *)FUN_0014ecc0(sVar10 + 9);
      puVar11[0xc] = puVar12;
      *puVar12 = 0;
      strcpy((char *)(puVar12 + 1),param_1);
      uVar1 = DAT_00182870;
      puVar11[0xd] = 0;
      FUN_00125120(puVar11 + 0xd,uVar1);
      puVar11[0xe] = 0;
      puVar11[0xf] = 0;
      puVar11[0x10] = 0;
      puVar11[0x11] = 0;
      FUN_0011d820(&DAT_00182840,puVar11 + 0x13,puVar11 + 0x12);
      strcpy((char *)(puVar11 + 0x14),DAT_00182858);
      puVar11 = DAT_00182320;
      goto joined_r0x00118f0d;
    }
    piVar8 = __errno_location();
    if ((*piVar8 == 0x11) && (lVar9 = FUN_00117850(param_1), lVar9 != 0)) break;
    iVar6 = FUN_00118980(param_1,0,param_3);
    if (iVar6 == 0) {
      uVar14 = 0xffffffff;
      FUN_00132ea0(param_1);
      goto LAB_00118d6f;
    }
  } while (iVar6 != 2);
LAB_00118d6c:
  uVar14 = 0;
LAB_00118d6f:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar14;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
joined_r0x00118f0d:
  if (puVar11 == (undefined8 *)0x0) goto LAB_00118d6c;
  if (*(char *)(puVar11 + 0xb) == '\0') {
    sVar10 = puVar11[0x13];
    iVar6 = strncmp(param_1,(char *)puVar11[0x14],sVar10);
    if (((iVar6 == 0) && (param_1[sVar10] == '/')) &&
       (pcVar13 = (char *)FUN_001400d0(param_1), pcVar13 == param_1 + sVar10 + 1)) {
      FUN_00117ae0(puVar11);
      uVar14 = 0;
      goto LAB_00118d6f;
    }
  }
  puVar11 = (undefined8 *)*puVar11;
  goto joined_r0x00118f0d;
}




// Function: extract_link @ 0x18fd0

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
  undefined1 local_159;
  stat local_158;
  stat local_c8;
  long local_30;
  
  __from = DAT_00182858;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_159 = 0;
  if ((DAT_00182b92 == '\0') && (cVar1 = FUN_00129420(DAT_00182858), cVar1 != '\0')) {
    uVar4 = FUN_00118d00(param_1,0,&local_159,0);
  }
  else {
    lVar5 = FUN_00117850(__from);
    if (lVar5 == 0) {
      piVar6 = __errno_location();
      do {
        iVar3 = linkat(DAT_0017f234,__from,DAT_0017f234,param_1,0);
        plVar9 = DAT_00182318;
        iVar2 = *piVar6;
        if (iVar3 == 0) {
          if ((DAT_00182318 == (long *)0x0) ||
             (iVar2 = fstatat(DAT_0017f234,__from,&local_158,0x100), iVar2 != 0)) goto LAB_001190f8;
          goto LAB_0011918c;
        }
        if (((iVar2 == 0x11) && (iVar3 = strcmp(__from,param_1), iVar3 == 0)) ||
           (((iVar3 = fstatat(DAT_0017f234,__from,&local_158,0x100), iVar3 == 0 &&
             ((iVar3 = fstatat(DAT_0017f234,param_1,&local_c8,0x100), iVar3 == 0 &&
              (local_158.st_dev == local_c8.st_dev)))) && (local_158.st_ino == local_c8.st_ino))))
        goto LAB_001190f8;
        *piVar6 = iVar2;
        iVar2 = FUN_00118980(param_1,0,&local_159);
      } while (iVar2 == 1);
      uVar4 = 0;
      if ((iVar2 != 2) && ((DAT_00182b50 == '\0' || (*piVar6 != 0x11)))) {
        FUN_00132dc0(__from,param_1);
        uVar4 = 1;
      }
    }
    else {
      uVar4 = FUN_00118d00(param_1,0,&local_159,lVar5);
    }
  }
  goto LAB_001190fb;
  while (plVar9 = (long *)*plVar9, plVar9 != (long *)0x0) {
LAB_0011918c:
    if (((int)plVar9[0xb] == DAT_00182580) &&
       (((plVar9[1] == local_158.st_dev && (plVar9[2] == local_158.st_ino)) &&
        ((-(uint)(plVar9[4] < -1) - ((int)~(uint)((ulong)plVar9[4] >> 0x20) >> 0x1f)) +
         (-(uint)(plVar9[3] < -1) - ((int)~(uint)((ulong)plVar9[3] >> 0x20) >> 0x1f)) * 2 == 0)))) {
      sVar7 = strlen(param_1);
      plVar8 = (long *)FUN_0014ecc0(sVar7 + 9);
      strcpy((char *)(plVar8 + 1),param_1);
      *plVar8 = plVar9[0xc];
      plVar9[0xc] = (long)plVar8;
      break;
    }
  }
LAB_001190f8:
  uVar4 = 0;
LAB_001190fb:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: extract_symlink @ 0x19270

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
  if ((DAT_00182b92 == '\0') && ((*DAT_00182858 == '/' || (cVar1 = FUN_00129420(), cVar1 != '\0'))))
  {
    uVar3 = FUN_00118d00(param_1,1,&local_21,0);
  }
  else {
    do {
      iVar2 = symlinkat(DAT_00182858,DAT_0017f234,param_1);
      if (iVar2 == 0) {
        FUN_001181b0(param_1,&DAT_00182840,0xffffffff,0,0,0x32,0,0x100);
        uVar3 = 0;
        goto LAB_00119304;
      }
      iVar2 = FUN_00118980(param_1,0,&local_21);
      if (iVar2 == 0) {
        FUN_00133370(DAT_00182858,param_1);
        uVar3 = 0xffffffff;
        goto LAB_00119304;
      }
    } while (iVar2 != 2);
    uVar3 = 0;
  }
LAB_00119304:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: extract_file @ 0x19380

int extract_file(char *param_1,int param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  long lVar4;
  ulong uVar5;
  int *piVar6;
  ulong uVar7;
  undefined8 uVar8;
  uint uVar9;
  uint uVar10;
  long in_FS_OFFSET;
  char local_f4;
  uint local_f0;
  undefined4 local_ec;
  undefined1 local_e1;
  ulong local_e0;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e1 = 0;
  uVar9 = DAT_001828b0 & 0x1c0;
  if (DAT_00182aa8 < 1) {
    uVar9 = DAT_001828b0 & 0x1ff;
  }
  local_f4 = (char)param_2;
  if (DAT_00182a6a == '\0') {
    if (DAT_00182a60 != 0) {
      iVar2 = FUN_0012c8b0(param_1,0x66,&DAT_00182840);
      local_ec = 0;
      local_f0 = 0;
      if (iVar2 < 0) {
        FUN_00124660();
        iVar2 = 0;
        goto LAB_001194fa;
      }
      goto LAB_00119421;
    }
    local_e0 = local_e0 & 0xffffffffffffff00;
    if ((0 < DAT_00182a98) && (DAT_00182998 != 0)) {
      uVar10 = ~DAT_00182328 & DAT_001828b0;
      do {
        iVar2 = mknodat(DAT_0017f234,param_1,uVar10 & 0x1ff,0);
        if (iVar2 == 0) {
          bVar1 = true;
          FUN_00132570(&DAT_00182840,param_1,(int)local_f4,0);
          goto LAB_001195a8;
        }
        iVar2 = FUN_00118980(param_1,0,&local_e0);
        if (iVar2 == 0) {
          FUN_00124660();
          iVar2 = 1;
          FUN_00132ea0(param_1);
          goto LAB_001194fa;
        }
      } while (iVar2 != 2);
    }
    bVar1 = false;
LAB_001195a8:
    do {
      iVar3 = DAT_00182b34;
      uVar10 = 0x809c1;
      if (DAT_00182b34 == 2) {
        uVar10 = (-(uint)(DAT_00182b69 == '\0') & 0x20000) + 0x80b41;
      }
      if (bVar1) {
        uVar10 = uVar10 & 0xffffff7f;
      }
      if (((param_2 == 0x37) && (DAT_00182310 == 0)) &&
         (DAT_00182310 = 1, ((byte)DAT_00181b80 & 8) != 0)) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar8 = dcgettext(0,"Extracting contiguous files as regular files",5);
        error(0,0,uVar8);
      }
      iVar2 = openat(DAT_0017f234,param_1,uVar10,(ulong)uVar9);
      if (-1 < iVar2) {
        if (iVar3 != 2) {
          local_ec = 0x1ff;
          local_f0 = ~DAT_00182328 & uVar9;
          goto LAB_00119421;
        }
        iVar3 = fstat(iVar2,&local_d8);
        if (iVar3 == 0) {
          local_f0 = local_d8.st_mode;
          if ((local_d8.st_mode & 0xf000) == 0x8000) {
            local_ec = 0xffffffff;
            goto LAB_00119421;
          }
          close(iVar2);
          piVar6 = __errno_location();
          *piVar6 = 0x11;
        }
        else {
          piVar6 = __errno_location();
          iVar3 = *piVar6;
          close(iVar2);
          *piVar6 = iVar3;
        }
      }
      iVar2 = FUN_00118980(param_1,1,&local_e1);
    } while (iVar2 == 1);
    FUN_00124660();
    if (iVar2 != 2) {
      iVar2 = 1;
      FUN_00132ea0(param_1);
      goto LAB_001194fa;
    }
  }
  else {
    local_ec = 0;
    iVar2 = 1;
    local_f0 = 0;
LAB_00119421:
    FUN_0010c670(&DAT_00182840);
    if (DAT_00182960 == '\0') {
      local_e0 = DAT_001828c8;
      do {
        if ((long)local_e0 < 1) goto LAB_001194b3;
        FUN_0010c710();
        lVar4 = FUN_0010d360();
        if (lVar4 == 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar8 = dcgettext(0,"Unexpected EOF in archive",5);
          error(0,0,uVar8);
          DAT_00182d58 = 2;
          goto LAB_001194b3;
        }
        uVar5 = FUN_0010cd10(lVar4);
        if (local_e0 < uVar5) {
          uVar5 = local_e0;
        }
        piVar6 = __errno_location();
        *piVar6 = 0;
        uVar7 = FUN_00126070(iVar2,lVar4,uVar5);
        local_e0 = local_e0 - uVar5;
        FUN_0010ccd0(lVar4 + -1 + uVar5);
      } while (uVar5 == uVar7);
      if (DAT_00182a60 == 0) {
        FUN_00133510(param_1,uVar7,uVar5);
      }
    }
    else {
      FUN_0012b460(iVar2,&DAT_00182840,&local_e0);
    }
LAB_001194b3:
    FUN_00123d40(local_e0);
    FUN_0010c6a0();
    if (DAT_00182a6a == '\0') {
      if (DAT_00182a60 == 0) {
        FUN_001181b0(param_1,&DAT_00182840,iVar2,local_f0,local_ec,(int)local_f4,0,
                     (ulong)(DAT_00182b34 != 2) << 8);
        iVar2 = close(iVar2);
      }
      else {
        iVar2 = close(iVar2);
      }
      if (iVar2 < 0) {
        FUN_00132d60(param_1);
      }
      if (DAT_00182a60 != 0) {
        FUN_0012ce00();
      }
      goto LAB_001194fa;
    }
  }
  iVar2 = 0;
LAB_001194fa:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: extract_dir @ 0x198d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int extract_dir(char *param_1,int param_2)

{
  undefined4 uVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  int *piVar8;
  long lVar9;
  long *plVar10;
  __dev_t *p_Var11;
  undefined8 *puVar12;
  undefined8 uVar13;
  uint uVar14;
  uint uVar15;
  undefined8 uVar16;
  long in_FS_OFFSET;
  byte bVar17;
  char local_169;
  __dev_t local_168;
  __ino_t local_160;
  uint local_150;
  stat local_d8;
  long local_40;
  
  bVar17 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_169 = '\0';
  if ((DAT_00182ad1 != '\0') && (DAT_001827f0 == 0)) {
    iVar6 = fstatat(DAT_0017f234,".",&local_d8,0);
    if (iVar6 == 0) {
      DAT_001827f0 = local_d8.st_dev;
    }
    else {
      FUN_00126460(".");
    }
  }
  if (DAT_00182b50 == '\0') {
    if (param_2 == 0x44) {
      FUN_00124660();
    }
  }
  else {
    FUN_00121100(param_1);
  }
  uVar14 = 0x1c0;
  if ((DAT_00182aa8 < 1) && (uVar14 = 0x1c0, DAT_00182aa4 < 1)) {
    uVar14 = 0x1ff;
  }
  uVar14 = DAT_001828b0 & uVar14;
  if (DAT_00182330 == '\0') {
    uVar14 = uVar14 | 0xc0;
  }
  uVar16 = 0;
  uVar15 = 0;
LAB_00119975:
  iVar6 = mkdirat(DAT_0017f234,param_1,uVar14);
  if (iVar6 == 0) goto LAB_00119c10;
  piVar8 = __errno_location();
  cVar5 = local_169;
  if (*piVar8 == 0x11) {
    if (local_169 == '\0') {
      if (DAT_00182b30 != '\0') goto LAB_00119a74;
      if (DAT_00182b34 < 3) goto LAB_001199be;
      goto LAB_001199f3;
    }
    if (DAT_00182b30 != '\0') {
LAB_00119a74:
      iVar7 = fstatat(DAT_0017f234,param_1,&local_d8,0x100);
      if ((((iVar7 == 0) && ((local_d8.st_mode & 0xf000) == 0xa000)) &&
          (iVar7 = fstatat(DAT_0017f234,param_1,&local_d8,0), iVar7 == 0)) &&
         ((local_d8.st_mode & 0xf000) == 0x4000)) {
        *piVar8 = 0x11;
        goto LAB_00119b95;
      }
      *piVar8 = 0x11;
    }
LAB_001199be:
    iVar7 = FUN_00125fc0(param_1,&local_168);
    if (iVar7 != 0) {
LAB_001199ed:
      *piVar8 = 0x11;
      goto LAB_001199f3;
    }
    uVar15 = local_150;
    if ((local_150 & 0xf000) != 0x4000) {
      uVar16 = 0xffffffff;
      goto LAB_001199ed;
    }
    if (cVar5 != '\0') {
      if (DAT_00182320 == (long *)0x0) goto LAB_00119d24;
      plVar10 = DAT_00182320;
      goto LAB_00119c7c;
    }
    if (DAT_00182b34 == 1) {
      uVar14 = 0x1c0;
      if ((DAT_00182aa8 < 1) && (uVar14 = 0x1c0, DAT_00182aa4 < 1)) {
        uVar14 = 0x1ff;
      }
      uVar14 = uVar14 & local_150 | -(uint)(DAT_00182330 == '\0') & 0xc0;
      iVar6 = FUN_00118100(0xffffffff,param_1,uVar14,0x100,0x35);
      if (iVar6 != 0) {
        FUN_00132c30(param_1,uVar14);
        goto LAB_00119c44;
      }
      p_Var11 = &local_168;
      puVar12 = &DAT_00182898;
      for (lVar9 = 0x24; lVar9 != 0; lVar9 = lVar9 + -1) {
        *(int *)puVar12 = (int)*p_Var11;
        p_Var11 = (__dev_t *)((long)p_Var11 + (ulong)bVar17 * -8 + 4);
        puVar12 = (undefined8 *)((long)puVar12 + ((ulong)bVar17 * -2 + 1) * 4);
      }
LAB_00119c10:
      uVar13 = 0x100;
      uVar16 = 0x1ff;
      uVar15 = ~DAT_00182328 & uVar14;
      goto LAB_00119a21;
    }
LAB_00119c44:
    uVar16 = 0xffffffff;
  }
  else {
LAB_001199f3:
    iVar7 = FUN_00118980(param_1,0,&local_169);
    if (iVar7 != 0) goto code_r0x00119a0a;
    if (*piVar8 != 0x11) {
      iVar6 = 1;
      FUN_00132e40(param_1);
      goto LAB_00119a3d;
    }
  }
  goto LAB_00119a19;
code_r0x00119a0a:
  if (iVar7 != 1) goto LAB_00119a19;
  goto LAB_00119975;
LAB_00119a19:
  if ((DAT_00182b34 & 0xfffffffd) == 0) {
    uVar13 = 0;
LAB_00119a21:
    FUN_00117b80(param_1,&DAT_00182840,uVar15,uVar16,DAT_001828b0,uVar13);
  }
  goto LAB_00119a3d;
  while (plVar10 = (long *)*plVar10, plVar10 != (long *)0x0) {
LAB_00119c7c:
    iVar6 = fstatat(DAT_0017f234,(char *)plVar10[0x14],&local_d8,*(int *)((long)plVar10 + 0x54));
    lVar4 = DAT_00182940;
    lVar3 = DAT_00182938;
    lVar2 = lRam0000000000182930;
    lVar9 = _DAT_00182928;
    if (iVar6 != 0) {
      FUN_00133410(plVar10[0x14]);
      goto LAB_00119b95;
    }
    if ((local_d8.st_dev == local_168) && (local_d8.st_ino == local_160)) {
      plVar10[1] = DAT_00182898;
      plVar10[2] = DAT_001828a0;
      *(uint *)(plVar10 + 3) = DAT_001828b0;
      *(undefined4 *)((long)plVar10 + 0x1c) = (undefined4)DAT_001828b4;
      uVar1 = DAT_001828b4._4_4_;
      plVar10[5] = lVar9;
      plVar10[6] = lVar2;
      *(undefined4 *)(plVar10 + 4) = uVar1;
      plVar10[7] = lVar3;
      plVar10[8] = lVar4;
      *(__mode_t *)(plVar10 + 9) = local_d8.st_mode;
      *(undefined4 *)((long)plVar10 + 0x4c) = 0xffffffff;
      *(undefined1 *)(plVar10 + 10) = 0;
      goto LAB_00119b95;
    }
  }
LAB_00119d24:
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar16 = FUN_0014ce40(param_1);
  uVar13 = dcgettext(0,"%s: Unexpected inconsistency when making directory",5);
  error(0,0,uVar13,uVar16);
  DAT_00182d58 = 2;
LAB_00119b95:
  iVar6 = 0;
LAB_00119a3d:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar6;
}




// Function: apply_nonancestor_delayed_set_stat @ 0x19da0

void apply_nonancestor_delayed_set_stat(char *param_1,byte param_2)

{
  ulong __n;
  undefined8 *__ptr;
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte bVar5;
  __mode_t _Var6;
  undefined4 uVar7;
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
  sVar2 = strlen(param_1);
  if (DAT_00182320 != (undefined8 *)0x0) {
    bVar5 = 0;
    do {
      __ptr = DAT_00182320;
      _Var6 = *(__mode_t *)(DAT_00182320 + 9);
      uVar7 = *(undefined4 *)((long)DAT_00182320 + 0x4c);
      bVar5 = bVar5 | *(byte *)(DAT_00182320 + 0xb);
      if ((param_2 < *(byte *)(DAT_00182320 + 0xb)) ||
         ((((__n = DAT_00182320[0x13], __n < sVar2 && (param_1[__n] != '\0')) &&
           ((param_1[__n] == '/' || (param_1[__n - 1] == '/')))) &&
          (iVar1 = memcmp(param_1,(void *)DAT_00182320[0x14],__n), iVar1 == 0)))) break;
      FUN_00126300(*(undefined4 *)((long)__ptr + 0x5c));
      if (bVar5 == 0) {
LAB_00119df0:
        local_188 = *(undefined4 *)(__ptr + 3);
        local_110 = __ptr[5];
        uStack_108 = __ptr[6];
        local_100 = __ptr[7];
        uStack_f8 = __ptr[8];
        local_184 = *(undefined4 *)((long)__ptr + 0x1c);
        local_180 = *(undefined4 *)(__ptr + 4);
        local_1c8 = __ptr[0xc];
        local_1c0 = __ptr[0xd];
        local_1b8 = __ptr[0xe];
        local_1b0 = __ptr[0xf];
        local_1a8 = __ptr[0x10];
        local_98 = __ptr[0x12];
        local_a0 = __ptr[0x11];
        FUN_001181b0(__ptr[0x14],local_1f8,0xffffffff,_Var6,uVar7,0x35,*(undefined1 *)(__ptr + 10),
                     *(undefined4 *)((long)__ptr + 0x54));
      }
      else {
        iVar1 = fstatat(DAT_0017f234,(char *)__ptr[0x14],&local_288,*(int *)((long)__ptr + 0x54));
        if (iVar1 == 0) {
          if ((local_288.st_dev == __ptr[1]) && (local_288.st_ino == __ptr[2])) {
            uVar7 = 0xffffffff;
            _Var6 = local_288.st_mode;
            goto LAB_00119df0;
          }
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar3 = FUN_0014ce40(__ptr[0x14]);
          uVar4 = dcgettext(0,"%s: Directory renamed before its status could be extracted",5);
          error(0,0,uVar4,uVar3);
          DAT_00182d58 = 2;
        }
        else {
          FUN_00133410(__ptr[0x14]);
        }
      }
      DAT_00182320 = (undefined8 *)*__ptr;
      free((void *)__ptr[0x14]);
      FUN_0011d580(__ptr[0x12],__ptr[0x11]);
      free((void *)__ptr[0xc]);
      free((void *)__ptr[0xd]);
      free((void *)__ptr[0xf]);
      free(__ptr);
    } while (DAT_00182320 != (undefined8 *)0x0);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: extract_finish @ 0x1a080

void extract_finish(void)

{
  FUN_00119da0(&DAT_0016a7d3,0);
  FUN_001185e0();
  FUN_00119da0(&DAT_0016a7d3,1);
  return;
}




// Function: extr_init @ 0x1a0b0

void extr_init(void)

{
  __uid_t _Var1;
  __mode_t __mask;
  
  _Var1 = geteuid();
  DAT_00182330 = _Var1 == 0;
  DAT_00182aa4 = DAT_00182aa4 + (uint)(byte)DAT_00182330;
  DAT_00182aa8 = DAT_00182aa8 + (uint)(byte)DAT_00182330;
  __mask = umask(0);
  DAT_0018232c = __mask;
  if (0 < DAT_00182aa4) {
    DAT_00182328 = 0;
    return;
  }
  umask(__mask);
  DAT_00182328 = __mask;
  return;
}




// Function: remove_delayed_set_stat @ 0x1a110

void remove_delayed_set_stat(char *param_1)

{
  long *plVar1;
  char *__s1;
  int iVar2;
  int iVar3;
  long *__ptr;
  long *plVar4;
  
  iVar2 = DAT_00182580;
  if (DAT_00182320 != (long *)0x0) {
    plVar1 = DAT_00182320;
    plVar4 = (long *)0x0;
    do {
      __ptr = plVar1;
      plVar1 = (long *)*__ptr;
      if (*(int *)((long)__ptr + 0x5c) == iVar2) {
        __s1 = (char *)__ptr[0x14];
        iVar3 = strcmp(__s1,param_1);
        if (iVar3 == 0) {
          free(__s1);
          FUN_0011d580(__ptr[0x12],__ptr[0x11]);
          free((void *)__ptr[0xc]);
          free((void *)__ptr[0xd]);
          free((void *)__ptr[0xf]);
          free(__ptr);
          if (plVar4 == (long *)0x0) {
            DAT_00182320 = plVar1;
            return;
          }
          *plVar4 = (long)plVar1;
          return;
        }
      }
      plVar4 = __ptr;
    } while (plVar1 != (long *)0x0);
  }
  return;
}




// Function: extract_archive @ 0x1a1e0

void extract_archive(void)

{
  undefined4 uVar1;
  char *pcVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  uint *puVar9;
  code *pcVar10;
  
  DAT_001822f0 = FUN_0011a080;
  FUN_0010ccd0(DAT_00182408);
  if ((DAT_00182b92 == '\0') && (cVar3 = FUN_00129420(DAT_00182840), cVar3 != '\0')) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar6 = FUN_0014ce40(DAT_00182840);
    uVar7 = dcgettext(0,"%s: Member name contains \'..\'",5);
    error(0,0,uVar7,uVar6);
    DAT_00182d58 = 2;
LAB_0011a39f:
    FUN_00124660();
    return;
  }
  if ((*DAT_00182848 == '\0') ||
     ((DAT_00182b40 != '\0' && (iVar4 = FUN_0012df50("extract"), iVar4 == 0)))) goto LAB_0011a39f;
  uVar1 = DAT_00182580;
  if (DAT_00182a54 != 0) {
    FUN_00123bc0(&DAT_00182840,DAT_00182408,0xffffffffffffffff);
    uVar1 = DAT_00182580;
  }
  DAT_00182580 = uVar1;
  if (DAT_001827e0 == '\0') {
    FUN_00119da0(DAT_00182848,0);
    FUN_00126300(uVar1);
  }
  if ((DAT_00182b84 != '\0') && (cVar3 = FUN_00125c10(DAT_00182848,0), cVar3 == '\0')) {
    piVar8 = __errno_location();
    iVar4 = *piVar8;
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar6 = FUN_0014ce40(DAT_00182848);
    uVar7 = dcgettext(0,"%s: Was unable to backup this file",5);
    error(0,iVar4,uVar7,uVar6);
    DAT_00182d58 = 2;
    goto LAB_0011a39f;
  }
  cVar3 = FUN_0012b3a0(&DAT_00182840);
  pcVar2 = DAT_00182848;
  if (cVar3 != '\0') {
    iVar4 = 0x53;
    goto LAB_0011a28c;
  }
  iVar4 = (int)*(char *)(DAT_00182408 + 0x9c);
  switch(*(char *)(DAT_00182408 + 0x9c)) {
  case '\0':
  case '0':
  case '7':
    if (DAT_00182850 != '\0') {
      pcVar10 = FUN_001198d0;
      break;
    }
    goto LAB_0011a28c;
  default:
    if ((DAT_00181b80._2_1_ & 1) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar6 = FUN_0014ce40(pcVar2);
      uVar7 = dcgettext(0,"%s: Unknown file type \'%c\', extracted as normal file",5);
      error(0,0,uVar7,uVar6,iVar4);
    }
    goto LAB_0011a28c;
  case '1':
    iVar4 = 0x31;
    pcVar10 = FUN_00118fd0;
    break;
  case '2':
    iVar4 = 0x32;
    pcVar10 = FUN_00119270;
    break;
  case '3':
    DAT_001828b0 = DAT_001828b0 | 0x2000;
    iVar4 = 0x33;
    pcVar10 = FUN_00118c10;
    break;
  case '4':
    DAT_001828b0 = DAT_001828b0 | 0x6000;
    iVar4 = 0x34;
    pcVar10 = FUN_00118c10;
    break;
  case '5':
  case 'D':
    pcVar10 = FUN_001198d0;
    if (DAT_001829c8 != '\0') {
      DAT_001827e0 = '\x01';
    }
    break;
  case '6':
    iVar4 = 0x36;
    pcVar10 = FUN_00118b30;
    break;
  case 'K':
  case 'L':
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar6 = dcgettext(0,"Unexpected long name header",5);
    error(0,0,uVar6);
    DAT_00182d58 = 2;
    goto switchD_0011a3e4_caseD_56;
  case 'M':
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar6 = FUN_0014ce40(DAT_00182848);
    uVar7 = dcgettext(0,"%s: Cannot extract -- file is continued from another volume",5);
    error(0,0,uVar7,uVar6);
    DAT_00182d58 = 2;
    goto switchD_0011a3e4_caseD_56;
  case 'S':
    iVar4 = 0x53;
LAB_0011a28c:
    if ((DAT_00182a6a == '\0') && (DAT_00182a60 == 0)) {
      pcVar10 = FUN_00119380;
      goto LAB_0011a2ae;
    }
    pcVar10 = FUN_00119380;
    goto LAB_0011a4be;
  case 'V':
    goto switchD_0011a3e4_caseD_56;
  }
  if (DAT_00182a6a == '\0') {
    if (DAT_00182a60 == 0) {
LAB_0011a2ae:
      if (DAT_00182b34 == 3) {
        iVar5 = FUN_00126a40(pcVar2,DAT_00182aba);
        if ((iVar5 == 0) && (puVar9 = (uint *)__errno_location(), (*puVar9 & 0xfffffffd) != 0)) {
          FUN_00133490(pcVar2);
          goto switchD_0011a3e4_caseD_56;
        }
      }
      else if ((DAT_00182b34 == 6) && (cVar3 = FUN_001188c0(pcVar2,0), cVar3 != '\0')) {
        if ((DAT_00181b80._1_1_ & 8) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar6 = FUN_0014d2d0(pcVar2);
          uVar7 = dcgettext(0,"Current %s is newer or same age",5);
          error(0,0,uVar7,uVar6);
        }
        goto switchD_0011a3e4_caseD_56;
      }
LAB_0011a4be:
      iVar4 = (*pcVar10)(DAT_00182848,iVar4);
      if (iVar4 == 0) {
        return;
      }
      goto LAB_0011a325;
    }
    if (pcVar10 == FUN_00119380) goto LAB_0011a4be;
  }
switchD_0011a3e4_caseD_56:
  FUN_00124660();
LAB_0011a325:
  if (DAT_00182b84 == '\0') {
    return;
  }
  FUN_00125ea0();
  return;
}




// Function: rename_directory @ 0x1a700

undefined8 rename_directory(char *param_1,char *param_2)

{
  char *__s1;
  undefined8 *puVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  size_t sVar8;
  long in_FS_OFFSET;
  undefined1 local_31;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = renameat(DAT_0017f234,param_1,DAT_0017f234,param_2);
  iVar3 = DAT_00182580;
  puVar1 = DAT_00182320;
  if (iVar2 == 0) {
    for (; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
      if (*(int *)((long)puVar1 + 0x5c) == iVar3) {
        __s1 = (char *)puVar1[0x14];
        iVar2 = strcmp(__s1,param_1);
        if (iVar2 == 0) {
          free(__s1);
          uVar5 = FUN_0014ee30(param_2);
          puVar1[0x14] = uVar5;
          sVar8 = strlen(param_2);
          puVar1[0x13] = sVar8;
          uVar5 = 1;
          goto LAB_0011a7b5;
        }
      }
    }
    uVar5 = 1;
  }
  else {
    piVar4 = __errno_location();
    iVar3 = *piVar4;
    if ((iVar3 == 2) && (iVar2 = FUN_00117ec0(param_2,&local_31), iVar2 == 0)) {
      iVar3 = renameat(DAT_0017f234,param_1,DAT_0017f234,param_2);
      uVar5 = 1;
      if (iVar3 == 0) goto LAB_0011a7b5;
      iVar3 = *piVar4;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar5 = FUN_0014d2b0(1,param_2);
    uVar6 = FUN_0014d2b0(0,param_1);
    uVar7 = dcgettext(0,"Cannot rename %s to %s",5);
    error(0,iVar3,uVar7,uVar6,uVar5);
    uVar5 = 0;
    DAT_00182d58 = 2;
  }
LAB_0011a7b5:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: dummy_coder @ 0x1a8a0

void dummy_coder(void)

{
  return;
}




// Function: dummy_decoder @ 0x1a8b0

void dummy_decoder(void)

{
  return;
}




// Function: assign_time_option @ 0x1a8c0

void assign_time_option(undefined8 param_1,undefined8 *param_2,undefined8 param_3)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 auVar2 [16];
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  auVar2 = FUN_00125930(param_3,&local_28,0);
  if ((auVar2._8_8_ < 0) || (*local_28 != '\0')) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar1 = dcgettext(0,"Time stamp is out of allowed range",5);
    error(0,0,uVar1);
    DAT_00182d58 = 2;
  }
  else {
    *param_2 = auVar2._0_8_;
    FUN_00125120(param_1,param_3);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dumpdir_decoder @ 0x1a970

void dumpdir_decoder(long param_1,undefined8 param_2,void *param_3,size_t param_4)

{
  void *__dest;
  
  __dest = (void *)FUN_0014ecc0(param_4);
  *(void **)(param_1 + 400) = __dest;
  memcpy(__dest,param_3,param_4);
  return;
}




// Function: x_obstack_grow @ 0x1a9b0

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




// Function: xheader_xattr__add @ 0x1aa20

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
  lVar3 = FUN_0014ece0(lVar1,lVar3);
  *param_1 = lVar3;
  uVar2 = FUN_0014ee30(param_3);
  *(undefined8 *)(lVar3 + lVar4) = uVar2;
  lVar3 = *param_1;
  uVar2 = FUN_0014edf0(param_4,param_5 + 1);
  *(undefined8 *)(lVar3 + lVar4 + 8) = uVar2;
  *(long *)(*param_1 + 0x10 + lVar4) = param_5;
  return;
}




// Function: xattr_acls_d_decoder @ 0x1aaa0

void xattr_acls_d_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0014edf0(param_3,param_4 + 1);
  *(long *)(param_1 + 0x50) = param_4;
  *(undefined8 *)(param_1 + 0x48) = uVar1;
  return;
}




// Function: xattr_acls_a_decoder @ 0x1aad0

void xattr_acls_a_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0014edf0(param_3,param_4 + 1);
  *(long *)(param_1 + 0x40) = param_4;
  *(undefined8 *)(param_1 + 0x38) = uVar1;
  return;
}




// Function: locate_handler @ 0x1ab00

undefined ** locate_handler(char *param_1)

{
  int iVar1;
  size_t __n;
  char *__s;
  undefined **ppuVar2;
  
  ppuVar2 = &PTR_s_atime_0017d260;
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




// Function: run_override_list @ 0x1ab80

void run_override_list(long *param_1,undefined8 param_2)

{
  char *__s;
  undefined8 *puVar1;
  size_t sVar2;
  
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    puVar1 = (undefined8 *)FUN_0011ab00(param_1[1]);
    if (puVar1 != (undefined8 *)0x0) {
      __s = (char *)param_1[2];
      sVar2 = strlen(__s);
      (*(code *)puVar1[2])(param_2,*puVar1,__s,sVar2);
    }
    param_1 = (long *)*param_1;
  } while (param_1 != (long *)0x0);
  return;
}




// Function: out_of_range_header @ 0x1abf0

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
  uVar1 = FUN_00145230(param_3,auStack_68);
  uVar2 = FUN_00145370(param_4,local_48);
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar3 = dcgettext(0,"Extended header %s=%s is out of range %s..%s",5);
  error(0,0,uVar3,param_1,param_2,uVar1,uVar2);
  DAT_00182d58 = 2;
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: decode_time @ 0x1aca0

undefined8 decode_time(undefined1 (*param_1) [16],char *param_2,undefined8 param_3)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 auVar2 [16];
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  auVar2 = FUN_00125930(param_2,&local_28,1);
  if (auVar2._8_8_ < 0) {
    if ((param_2 < local_28) && (*local_28 == '\0')) {
      FUN_0011abf0(param_3,param_2,0x8000000000000000,0x7fffffffffffffff);
      uVar1 = 0;
    }
    else {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar1 = dcgettext(0,"Malformed extended header: invalid %s=%s",5);
      error(0,0,uVar1,param_3,param_2);
      uVar1 = 0;
      DAT_00182d58 = 2;
    }
  }
  else {
    *param_1 = auVar2;
    uVar1 = 1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mtime_decoder @ 0x1ad90

void mtime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011aca0(&local_28,param_3,param_2);
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




// Function: ctime_decoder @ 0x1adf0

void ctime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011aca0(&local_28,param_3,param_2);
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




// Function: atime_decoder @ 0x1ae50

void atime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 uStack_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011aca0(&local_28,param_3,param_2);
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




// Function: sparse_map_decoder @ 0x1aeb0

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
        if (*piVar3 == 0x22) goto LAB_0011b018;
      }
      else {
        if (*piVar3 == 0x22) {
LAB_0011b018:
          FUN_0011abf0(param_2,param_3,0,0x7fffffffffffffff);
          goto LAB_0011aff0;
        }
        uVar1 = *(ulong *)(param_1 + 0x130);
        if (*(ulong *)(param_1 + 0x138) <= uVar1) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          pcVar7 = "Malformed extended header: excess %s=%s";
          goto LAB_0011afca;
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
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar5 = dcgettext(0,"Malformed extended header: invalid %s: odd number of values",5);
          error(0,0,uVar5,param_2);
          DAT_00182d58 = 2;
        }
        goto LAB_0011aff0;
      }
      if (cVar8 != ',') {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
          cVar8 = *local_48;
        }
        uVar5 = dcgettext(0,"Malformed extended header: invalid %s: unexpected delimiter %c",5);
        error(0,0,uVar5,param_2,(int)cVar8);
        DAT_00182d58 = 2;
        goto LAB_0011aff0;
      }
      param_3 = local_48 + 1;
      unaff_R12 = iVar4;
      bVar2 = (bool)(bVar2 ^ 1);
    } while ((int)local_48[1] - 0x30U < 10);
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  pcVar7 = "Malformed extended header: invalid %s=%s";
LAB_0011afca:
  uVar5 = dcgettext(0,pcVar7,5);
  error(0,0,uVar5,param_2,param_3);
  DAT_00182d58 = 2;
LAB_0011aff0:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: gid_decoder @ 0x1b1f0

void gid_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x78) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: uid_decoder @ 0x1b250

void uid_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x74) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: size_decoder @ 0x1b610

void size_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(&local_18,param_3,0x7fffffffffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x88) = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_size_decoder @ 0x1b670

void volume_size_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    DAT_00181f08 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_minor_decoder @ 0x1b6d0

void sparse_minor_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x128) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_major_decoder @ 0x1b730

void sparse_major_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(local_18,param_3,0xffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x124) = local_18[0];
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_offset_decoder @ 0x1b790

void volume_offset_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    DAT_00181f00 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_size_decoder @ 0x1b7f0

void sparse_size_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(&local_18,param_3,0x7fffffffffffffff,param_2);
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




// Function: sparse_numblocks_decoder @ 0x1b860

void sparse_numblocks_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011b100(&local_18,param_3,0xffffffffffffffff,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x138) = local_18;
    uVar2 = FUN_0014edb0(local_18,0x10);
    *(undefined8 *)(param_1 + 0x130) = 0;
    *(undefined8 *)(param_1 + 0x140) = uVar2;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: decg @ 0x1bab0

void decg(undefined8 *param_1,undefined8 param_2,long param_3,undefined8 param_4)

{
  undefined8 uVar1;
  long lVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  
  lVar2 = FUN_0011ab00(param_2);
  if ((lVar2 != 0) && ((*(byte *)(lVar2 + 0x18) & 2) != 0)) {
                    /* WARNING: Could not recover jumptable at 0x0011baf3. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (**(code **)(lVar2 + 0x10))(param_1,param_2,param_3,param_4);
    return;
  }
  puVar3 = (undefined8 *)FUN_0014ecc0(0x18);
  uVar4 = FUN_0014ee30(param_2);
  puVar3[1] = uVar4;
  uVar4 = 0;
  if (param_3 != 0) {
    uVar4 = FUN_0014ee30(param_3);
  }
  uVar1 = *param_1;
  puVar3[2] = uVar4;
  *param_1 = puVar3;
  *puVar3 = uVar1;
  return;
}




// Function: xheader_print_n @ 0x1bb40

void xheader_print_n(long *param_1,char *param_2,undefined8 param_3,long param_4)

{
  char *__s;
  size_t sVar1;
  long lVar2;
  undefined1 *puVar3;
  ulong uVar4;
  ulong uVar5;
  char *pcVar6;
  char cVar7;
  undefined1 *puVar8;
  long in_FS_OFFSET;
  bool bVar9;
  undefined1 local_58 [20];
  undefined1 local_44 [4];
  long local_40;
  
  pcVar6 = DAT_00182340;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182340 == (char *)0x0) {
    DAT_00182338 = 0x100;
    DAT_00182340 = (char *)FUN_0014ecc0(0x100);
    cVar7 = *param_2;
    pcVar6 = DAT_00182340;
  }
  else {
    *DAT_00182340 = '\0';
    cVar7 = *param_2;
  }
  __s = pcVar6;
  if (cVar7 != '\0') {
    uVar5 = 0;
    do {
      uVar4 = uVar5 + 2;
      if (uVar4 < DAT_00182338) {
        pcVar6 = __s + uVar5;
        if (cVar7 != '%') goto LAB_0011bba2;
LAB_0011bbed:
        builtin_strncpy(pcVar6,"%25",4);
      }
      else {
        __s = (char *)FUN_0014ed20(__s,&DAT_00182338);
        pcVar6 = __s + uVar5;
        DAT_00182340 = __s;
        if (cVar7 == '%') goto LAB_0011bbed;
LAB_0011bba2:
        if (cVar7 == '=') {
          builtin_strncpy(pcVar6,"%3D",4);
        }
        else {
          *pcVar6 = cVar7;
          uVar4 = uVar5;
        }
      }
      cVar7 = param_2[1];
      param_2 = param_2 + 1;
      uVar5 = uVar4 + 1;
    } while (cVar7 != '\0');
    pcVar6 = __s + uVar5;
  }
  *pcVar6 = '\0';
  sVar1 = strlen(__s);
  puVar3 = (undefined1 *)0x0;
  do {
    lVar2 = FUN_00145370(puVar3 + sVar1 + 3 + param_4,local_58);
    puVar8 = local_44 + -lVar2;
    bVar9 = puVar3 != puVar8;
    puVar3 = puVar8;
  } while (bVar9);
  FUN_0011a9b0(param_1,lVar2,puVar8);
  lVar2 = *param_1;
  puVar3 = *(undefined1 **)(lVar2 + 0x18);
  if (*(undefined1 **)(lVar2 + 0x20) == puVar3) {
    _obstack_newchunk(lVar2,1);
    puVar3 = *(undefined1 **)(lVar2 + 0x18);
  }
  *(undefined1 **)(lVar2 + 0x18) = puVar3 + 1;
  *puVar3 = 0x20;
  param_1[1] = param_1[1] + 1;
  FUN_0011a9b0(param_1,__s,sVar1);
  lVar2 = *param_1;
  puVar3 = *(undefined1 **)(lVar2 + 0x18);
  if (*(undefined1 **)(lVar2 + 0x20) == puVar3) {
    _obstack_newchunk(lVar2,1);
    puVar3 = *(undefined1 **)(lVar2 + 0x18);
  }
  *(undefined1 **)(lVar2 + 0x18) = puVar3 + 1;
  *puVar3 = 0x3d;
  param_1[1] = param_1[1] + 1;
  FUN_0011a9b0(param_1,param_3,param_4);
  lVar2 = *param_1;
  puVar3 = *(undefined1 **)(lVar2 + 0x18);
  if (*(undefined1 **)(lVar2 + 0x20) == puVar3) {
    _obstack_newchunk(lVar2,1);
    puVar3 = *(undefined1 **)(lVar2 + 0x18);
  }
  *(undefined1 **)(lVar2 + 0x18) = puVar3 + 1;
  *puVar3 = 10;
  param_1[1] = param_1[1] + 1;
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: code_string @ 0x1bdb0

void code_string(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char *pcVar1;
  char cVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_00131920(1,param_1,&local_38);
  if (cVar2 == '\0') {
    local_38 = (char *)FUN_0014ee30(param_1);
  }
  pcVar1 = local_38;
  sVar3 = strlen(local_38);
  FUN_0011bb40(param_3,param_2,pcVar1,sVar3);
  free(local_38);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xattr_selinux_coder @ 0x1be50

void xattr_selinux_coder(long param_1)

{
  FUN_0011bdb0(*(undefined8 *)(param_1 + 0x30));
  return;
}




// Function: dumpdir_coder @ 0x1bf20

void dumpdir_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0011f690(param_4);
  FUN_0011bb40(param_3,param_2,param_4,uVar1);
  return;
}




// Function: mtime_coder @ 0x1bf50

void mtime_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  char *__s;
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 auStack_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 == (undefined8 *)0x0) {
    uVar3 = *(undefined8 *)(param_1 + 0xf8);
    uVar2 = *(undefined8 *)(param_1 + 0x100);
  }
  else {
    uVar3 = *param_4;
    uVar2 = param_4[1];
  }
  __s = (char *)FUN_001257a0(uVar3,uVar2,auStack_48);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ctime_coder @ 0x1bfe0

void ctime_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001257a0(*(undefined8 *)(param_1 + 0x108),*(undefined8 *)(param_1 + 0x110),
                             auStack_48);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: atime_coder @ 0x1c060

void atime_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001257a0(*(undefined8 *)(param_1 + 0xe8),*(undefined8 *)(param_1 + 0xf0),
                             auStack_48);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_offset_coder @ 0x1c0e0

void sparse_offset_coder(long param_1,undefined8 param_2,undefined8 param_3,long *param_4)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined8 *)(*param_4 * 0x10 + *(long *)(param_1 + 0x140)),0,
                             0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_major_coder @ 0x1c170

void sparse_major_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined4 *)(param_1 + 0x124),0,0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_minor_coder @ 0x1c1f0

void sparse_minor_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined4 *)(param_1 + 0x128),0,0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_size_coder @ 0x1c370

void volume_size_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*param_4,0,0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: uid_coder @ 0x1c3f0

void uid_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined4 *)(param_1 + 0x74),0,0xffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_numblocks_coder @ 0x1c4e0

void sparse_numblocks_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined8 *)(param_1 + 0x130),0,0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: gid_coder @ 0x1c560

void gid_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined4 *)(param_1 + 0x78),0,0xffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_numbytes_coder @ 0x1c5d0

void sparse_numbytes_coder(long param_1,undefined8 param_2,undefined8 param_3,long *param_4)

{
  char *__s;
  size_t sVar1;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_001255c0(*(undefined8 *)(*param_4 * 0x10 + *(long *)(param_1 + 0x140) + 8),0,
                             0xffffffffffffffff,auStack_38);
  sVar1 = strlen(__s);
  FUN_0011bb40(param_3,param_2,__s,sVar1);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volume_filename_decoder @ 0x1c660

void volume_filename_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (DAT_00181f10 != (void *)0x0) {
    free(DAT_00181f10);
    DAT_00181f10 = (void *)0x0;
  }
  cVar1 = FUN_00131920(0,param_3,&DAT_00181f10);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(&DAT_00181f10,param_3);
  return;
}




// Function: volume_label_decoder @ 0x1c6d0

void volume_label_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (DAT_00181f18 != (void *)0x0) {
    free(DAT_00181f18);
    DAT_00181f18 = (void *)0x0;
  }
  cVar1 = FUN_00131920(0,param_3,&DAT_00181f18);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(&DAT_00181f18,param_3);
  return;
}




// Function: linkpath_decoder @ 0x1c740

void linkpath_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (*(void **)(param_1 + 0x18) != (void *)0x0) {
    free(*(void **)(param_1 + 0x18));
    *(undefined8 *)(param_1 + 0x18) = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 0x18);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(param_1 + 0x18,param_3);
  return;
}




// Function: xattr_selinux_decoder @ 0x1c790

void xattr_selinux_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (*(void **)(param_1 + 0x30) != (void *)0x0) {
    free(*(void **)(param_1 + 0x30));
    *(undefined8 *)(param_1 + 0x30) = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 0x30);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(param_1 + 0x30,param_3);
  return;
}




// Function: uname_decoder @ 0x1c7e0

void uname_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (*(void **)(param_1 + 0x20) != (void *)0x0) {
    free(*(void **)(param_1 + 0x20));
    *(undefined8 *)(param_1 + 0x20) = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 0x20);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(param_1 + 0x20,param_3);
  return;
}




// Function: gname_decoder @ 0x1c830

void gname_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  
  if (*(void **)(param_1 + 0x28) != (void *)0x0) {
    free(*(void **)(param_1 + 0x28));
    *(undefined8 *)(param_1 + 0x28) = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 0x28);
  if (cVar1 != '\0') {
    return;
  }
  FUN_00125120(param_1 + 0x28,param_3);
  return;
}




// Function: sparse_path_decoder @ 0x1c880

void sparse_path_decoder(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined1 uVar2;
  
  *(undefined1 *)((long)param_1 + 0x151) = 1;
  if ((void *)*param_1 != (void *)0x0) {
    free((void *)*param_1);
    *param_1 = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1);
  if (cVar1 == '\0') {
    FUN_00125120(param_1,param_3);
  }
  if ((void *)param_1[1] != (void *)0x0) {
    free((void *)param_1[1]);
    param_1[1] = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 1);
  if (cVar1 == '\0') {
    FUN_00125120(param_1 + 1,param_3);
  }
  uVar2 = FUN_001403c0(param_1[1]);
  *(undefined1 *)(param_1 + 2) = uVar2;
  return;
}




// Function: path_decoder @ 0x1c920

void path_decoder(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined1 uVar2;
  
  if (*(char *)((long)param_1 + 0x151) != '\0') {
    return;
  }
  if ((void *)*param_1 != (void *)0x0) {
    free((void *)*param_1);
    *param_1 = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1);
  if (cVar1 == '\0') {
    FUN_00125120(param_1,param_3);
  }
  if ((void *)param_1[1] != (void *)0x0) {
    free((void *)param_1[1]);
    param_1[1] = 0;
  }
  cVar1 = FUN_00131920(0,param_3,param_1 + 1);
  if (cVar1 == '\0') {
    FUN_00125120(param_1 + 1,param_3);
  }
  uVar2 = FUN_001403c0(param_1[1]);
  *(undefined1 *)(param_1 + 2) = uVar2;
  return;
}




// Function: xheader_set_option @ 0x1ca20

void xheader_set_option(char *param_1)

{
  byte bVar1;
  int iVar2;
  byte *__s;
  byte *pbVar3;
  ushort **ppuVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  undefined **ppuVar7;
  char *pcVar8;
  byte bVar9;
  byte *pbVar10;
  
  __s = (byte *)strtok(param_1,",");
  while( true ) {
    if (__s == (byte *)0x0) {
      return;
    }
    pbVar3 = (byte *)strchr((char *)__s,0x3d);
    if (pbVar3 == (byte *)0x0) break;
    if (pbVar3 == __s) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      __s = (byte *)FUN_0014d2d0(__s);
      pcVar8 = "Malformed pax option: %s";
LAB_0011cbfc:
      uVar6 = dcgettext(0,pcVar8,5);
      goto LAB_0011cc09;
    }
    bVar1 = pbVar3[-1];
    pbVar10 = pbVar3;
    if (bVar1 == 0x3a) {
      pbVar10 = pbVar3 + -1;
    }
    if (__s < pbVar10) {
      ppuVar4 = __ctype_b_loc();
      do {
        if ((*(byte *)((long)*ppuVar4 + (ulong)*pbVar10 * 2 + 1) & 0x20) == 0) break;
        pbVar10 = pbVar10 + -1;
      } while (pbVar10 != __s);
    }
    *pbVar10 = 0;
    pbVar10 = pbVar3 + 1;
    bVar9 = pbVar3[1];
    if (bVar9 != 0) {
      ppuVar4 = __ctype_b_loc();
      do {
        if ((*(byte *)((long)*ppuVar4 + (ulong)bVar9 * 2 + 1) & 0x20) == 0) break;
        bVar9 = pbVar10[1];
        pbVar10 = pbVar10 + 1;
      } while (bVar9 != 0);
    }
    iVar2 = strcmp((char *)__s,"delete");
    if (iVar2 == 0) {
      pcVar8 = "atime";
      ppuVar7 = &PTR_s_atime_0017d260;
      do {
        if (((*(char *)((long)ppuVar7 + 0x1c) == '\0') && (((ulong)ppuVar7[3] & 1) != 0)) &&
           (iVar2 = fnmatch((char *)pbVar10,pcVar8,0), iVar2 == 0)) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          __s = (byte *)FUN_0014d2d0(pbVar10);
          pcVar8 = "Pattern %s cannot be used";
          goto LAB_0011cbfc;
        }
        pcVar8 = ppuVar7[4];
        ppuVar7 = ppuVar7 + 4;
      } while (pcVar8 != (char *)0x0);
      puVar5 = (undefined8 *)FUN_0014ecc0(0x18);
      uVar6 = FUN_0014ee30();
      puVar5[2] = 0;
      puVar5[1] = uVar6;
      *puVar5 = DAT_00182390;
      DAT_00182390 = puVar5;
    }
    else {
      iVar2 = strcmp((char *)__s,"exthdr.name");
      if (iVar2 == 0) {
        FUN_00125120(&DAT_00182370,pbVar10);
      }
      else {
        iVar2 = strcmp((char *)__s,"globexthdr.name");
        if (iVar2 == 0) {
          FUN_00125120(&DAT_00182358,pbVar10);
        }
        else {
          iVar2 = strcmp((char *)__s,"exthdr.mtime");
          if (iVar2 == 0) {
            FUN_0011a8c0(&DAT_00182368,&DAT_00182360,pbVar10);
          }
          else {
            iVar2 = strcmp((char *)__s,"globexthdr.mtime");
            if (iVar2 == 0) {
              FUN_0011a8c0(&DAT_00182350,&DAT_00182348,pbVar10);
            }
            else {
              pcVar8 = "atime";
              ppuVar7 = &PTR_s_atime_0017d260;
              do {
                if (((*(char *)((long)ppuVar7 + 0x1c) == '\0') && (((ulong)ppuVar7[3] & 1) != 0)) &&
                   (iVar2 = strcmp(pcVar8,(char *)__s), iVar2 == 0)) {
                  if (DAT_00182d50 != (code *)0x0) {
                    (*DAT_00182d50)();
                  }
                  uVar6 = dcgettext(0,"Keyword %s cannot be overridden",5);
                  goto LAB_0011cc09;
                }
                pcVar8 = ppuVar7[4];
                ppuVar7 = ppuVar7 + 4;
              } while (pcVar8 != (char *)0x0);
              if (bVar1 == 0x3a) {
                puVar5 = (undefined8 *)FUN_0014ecc0(0x18);
                uVar6 = FUN_0014ee30(__s);
                puVar5[1] = uVar6;
                uVar6 = FUN_0014ee30();
                puVar5[2] = uVar6;
                *puVar5 = DAT_00182380;
                DAT_00182380 = puVar5;
              }
              else {
                puVar5 = (undefined8 *)FUN_0014ecc0(0x18);
                uVar6 = FUN_0014ee30(__s);
                puVar5[1] = uVar6;
                uVar6 = FUN_0014ee30();
                puVar5[2] = uVar6;
                *puVar5 = DAT_00182388;
                DAT_00182388 = puVar5;
              }
            }
          }
        }
      }
    }
    __s = (byte *)strtok((char *)0x0,",");
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar6 = dcgettext(0,"Keyword %s is unknown or not yet implemented",5);
LAB_0011cc09:
  error(0,0,uVar6,__s);
                    /* WARNING: Subroutine does not return */
  FUN_0012dea0(2);
}




// Function: xheader_format_name @ 0x1cdf0

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
LAB_0011ce5e:
        if (cVar3 == '\0') goto LAB_0011cef8;
      }
      if (cVar2 == 'f') {
        if (param_1 == (undefined8 *)0x0) {
LAB_0011d158:
          pcVar6 = pcVar8 + 2;
          pcVar8 = pcVar8 + 2;
          cVar3 = *pcVar6;
        }
        else {
          __s_00 = (char *)FUN_001400d0(*param_1);
          sVar5 = strlen(__s_00);
          puVar9 = puVar9 + sVar5;
          pcVar6 = pcVar8 + 2;
          pcVar8 = pcVar8 + 2;
          cVar3 = *pcVar6;
        }
        goto LAB_0011ce5e;
      }
      if ('f' < cVar2) {
        if (cVar2 == 'n') {
          pcVar6 = (char *)FUN_00145370(param_3,local_58);
          puVar7 = local_44;
          local_a0 = pcVar6;
        }
        else {
          if (cVar2 != 'p') goto LAB_0011d0aa;
          _Var4 = getpid();
          pcVar6 = (char *)FUN_00145370((long)_Var4,local_78);
          puVar7 = local_64;
          local_a8 = pcVar6;
        }
        puVar9 = puVar7 + ((long)puVar9 - (long)pcVar6);
        pcVar6 = pcVar8 + 2;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011ce5e;
      }
      if (cVar2 == '%') {
        pcVar6 = pcVar8 + 2;
        puVar9 = puVar9 + 1;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011ce5e;
      }
      if (cVar2 != 'd') {
LAB_0011d0aa:
        pcVar6 = pcVar8 + 2;
        puVar9 = puVar9 + 2;
        pcVar8 = pcVar8 + 2;
        cVar3 = *pcVar6;
        goto LAB_0011ce5e;
      }
      if (param_1 == (undefined8 *)0x0) goto LAB_0011d158;
      if (__ptr == (void *)0x0) {
        __ptr = (void *)FUN_00140230(*param_1);
      }
      __s = (char *)FUN_00133640(__ptr,0,DAT_00182b92);
      sVar5 = strlen(__s);
      puVar9 = puVar9 + sVar5;
      cVar3 = pcVar8[2];
      pcVar8 = pcVar8 + 2;
    } while (cVar3 != '\0');
LAB_0011cef8:
    puVar9 = puVar9 + 1;
  }
  pcVar6 = (char *)FUN_0014ecc0(puVar9);
  cVar3 = *param_2;
  pcVar8 = pcVar6;
  if (cVar3 != '\0') {
LAB_0011cf30:
    cVar2 = param_2[1];
    if (cVar3 == '%') {
      if (cVar2 == 'f') {
        if (__s_00 != (char *)0x0) {
          pcVar8 = stpcpy(pcVar8,__s_00);
          goto LAB_0011d03d;
        }
LAB_0011cf6d:
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
            goto joined_r0x0011cf78;
          }
          if (cVar2 == 'd') {
            if (__s != (char *)0x0) {
              pcVar8 = stpcpy(pcVar8,__s);
            }
            goto LAB_0011cf6d;
          }
          *pcVar8 = '%';
          if (cVar2 == '\0') {
            pcVar8 = pcVar8 + 1;
            goto LAB_0011cf80;
          }
LAB_0011d0c0:
          pcVar8[1] = cVar2;
          pcVar1 = param_2 + 2;
          pcVar8 = pcVar8 + 2;
          param_2 = param_2 + 2;
          cVar3 = *pcVar1;
          goto joined_r0x0011cf78;
        }
        if (cVar2 == 'n') {
          pcVar8 = stpcpy(pcVar8,local_a0);
        }
        else {
          if (cVar2 != 'p') {
            *pcVar8 = '%';
            goto LAB_0011d0c0;
          }
          pcVar8 = stpcpy(pcVar8,local_a8);
        }
LAB_0011d03d:
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
joined_r0x0011cf78:
    if (cVar3 == '\0') goto LAB_0011cf80;
    goto LAB_0011cf30;
  }
  free(__ptr);
  goto LAB_0011cf9f;
LAB_0011cf80:
  free(__ptr);
  if (pcVar6 < pcVar8) {
    do {
      if (pcVar8[-1] != '/') break;
      pcVar8 = pcVar8 + -1;
    } while (pcVar6 != pcVar8);
  }
LAB_0011cf9f:
  *pcVar8 = '\0';
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return pcVar6;
}




// Function: xheader_xhdr_name @ 0x1d1d0

void xheader_xhdr_name(undefined8 param_1)

{
  if (DAT_00182370 != 0) {
    FUN_0011cdf0(param_1,DAT_00182370,0);
    return;
  }
  FUN_00125120(&DAT_00182370,(&PTR_s__d_PaxHeaders__f_0017d640)[DAT_00182a38]);
  FUN_0011cdf0(param_1,DAT_00182370,0);
  return;
}




// Function: xheader_ghdr_name @ 0x1d220

void xheader_ghdr_name(void)

{
  char *pcVar1;
  size_t sVar2;
  size_t sVar3;
  char *pcVar4;
  
  pcVar4 = DAT_00182358;
  if (DAT_00182358 == (char *)0x0) {
    pcVar4 = (&PTR_s__GlobalHead__n_0017d650)[DAT_00182a38];
    pcVar1 = getenv("TMPDIR");
    if (pcVar1 == (char *)0x0) {
      pcVar1 = "/tmp";
    }
    sVar2 = strlen(pcVar1);
    sVar3 = strlen(pcVar4);
    DAT_00182358 = (char *)FUN_0014ecc0(sVar2 + 1 + sVar3);
    pcVar1 = strcpy(DAT_00182358,pcVar1);
    pcVar4 = strcat(pcVar1,pcVar4);
  }
  FUN_0011cdf0(0,pcVar4,DAT_00182398 + 1);
  return;
}




// Function: xheader_write @ 0x1d2d0

void xheader_write(char param_1,undefined8 param_2,undefined8 param_3,long *param_4)

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
  __n = param_4[1];
  if (param_1 == 'g') {
    if (DAT_00182350 != 0) {
      param_3 = DAT_00182348;
    }
  }
  else if ((param_1 == 'x') && (DAT_00182368 != 0)) {
    param_3 = DAT_00182360;
  }
  lVar2 = FUN_00113050(param_2,__n,param_3);
  uVar1 = (uint)__n;
  *(char *)(lVar2 + 0x9c) = param_1;
  FUN_00113230(lVar2);
  lVar2 = param_4[2] + __n;
  while( true ) {
    puVar3 = (undefined8 *)FUN_0010d360();
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
    FUN_0010ccd0(puVar3);
    if (__n == 0) goto LAB_0011d39c;
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
  FUN_0010ccd0(pvVar4);
LAB_0011d39c:
  if (*param_4 == 0) {
    free((void *)param_4[2]);
  }
  else {
    _obstack_free(*param_4,0);
    free((void *)*param_4);
    *param_4 = 0;
  }
  param_4[2] = 0;
  param_4[1] = 0;
  if (param_1 == 'g') {
    DAT_00182398 = DAT_00182398 + 1;
  }
  return;
}




// Function: xheader_forbid_global @ 0x1d4e0

void xheader_forbid_global(void)

{
  undefined8 uVar1;
  
  if (DAT_00182388 == 0) {
    return;
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar1 = dcgettext(0,"can\'t update global extended header record",5);
  error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_0012dea0(2);
}




// Function: xheader_xattr_init @ 0x1d530

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




// Function: xheader_xattr_free @ 0x1d580

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




// Function: xheader_xattr_add @ 0x1d5d0

void xheader_xattr_add(long param_1,char *param_2,undefined8 param_3,undefined8 param_4)

{
  size_t sVar1;
  size_t sVar2;
  undefined8 *__ptr;
  
  sVar1 = strlen(param_2);
  sVar2 = strlen("SCHILY.xattr.");
  __ptr = (undefined8 *)FUN_0014ecc0(sVar1 + 1 + sVar2);
  *__ptr = 0x782e594c49484353;
  *(undefined4 *)(__ptr + 1) = 0x72747461;
  *(undefined1 *)((long)__ptr + 0xc) = 0x2e;
  strcpy((char *)((long)__ptr + 0xd),param_2);
  FUN_0011aa20(param_1 + 0x160,param_1 + 0x158,__ptr,param_3,param_4);
  free(__ptr);
  return;
}




// Function: xattr_decoder @ 0x1d660

void xattr_decoder(undefined8 param_1,char *param_2,void *param_3,long param_4)

{
  char cVar1;
  long lVar2;
  char *pcVar3;
  undefined1 *puVar4;
  size_t sVar5;
  char *pcVar6;
  char *pcVar7;
  ulong uVar8;
  char *__dest;
  undefined1 *puVar9;
  undefined1 *puVar11;
  long in_FS_OFFSET;
  undefined1 auStack_48 [8];
  long local_40;
  undefined1 *puVar10;
  undefined1 *puVar12;
  
  puVar9 = auStack_48;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar5 = strlen(param_2);
  puVar10 = auStack_48;
  puVar12 = auStack_48;
  while (puVar10 != auStack_48 + -(sVar5 + 0x18 & 0xfffffffffffff000)) {
    puVar9 = puVar12 + -0x1000;
    *(undefined8 *)(puVar12 + -8) = *(undefined8 *)(puVar12 + -8);
    puVar10 = puVar12 + -0x1000;
    puVar12 = puVar12 + -0x1000;
  }
  uVar8 = (ulong)((uint)(sVar5 + 0x18) & 0xff0);
  lVar2 = -uVar8;
  puVar11 = puVar9 + lVar2;
  puVar12 = puVar9 + lVar2;
  if (uVar8 != 0) {
    *(undefined8 *)(puVar9 + -8) = *(undefined8 *)(puVar9 + -8);
  }
  __dest = (char *)((ulong)(puVar9 + lVar2 + 0xf) & 0xfffffffffffffff0);
  *(undefined8 *)(puVar9 + lVar2 + -8) = 0x11d6f6;
  memcpy(__dest,param_2,sVar5 + 1);
  puVar4 = puVar9 + lVar2;
  while (puVar11 != puVar9 + (lVar2 - (param_4 + 0x18U & 0xfffffffffffff000))) {
    puVar12 = puVar4 + -0x1000;
    *(undefined8 *)(puVar4 + -8) = *(undefined8 *)(puVar4 + -8);
    puVar11 = puVar4 + -0x1000;
    puVar4 = puVar4 + -0x1000;
  }
  uVar8 = (ulong)((uint)(param_4 + 0x18U) & 0xff0);
  lVar2 = -uVar8;
  if (uVar8 != 0) {
    *(undefined8 *)(puVar12 + -8) = *(undefined8 *)(puVar12 + -8);
  }
  *(undefined8 *)(puVar12 + lVar2 + -8) = 0x11d756;
  memcpy((void *)((ulong)(puVar12 + lVar2 + 0xf) & 0xfffffffffffffff0),param_3,param_4 + 1);
  pcVar6 = __dest;
  pcVar3 = __dest;
LAB_0011d76f:
  do {
    while( true ) {
      pcVar7 = pcVar3 + 1;
      cVar1 = *pcVar6;
      if (cVar1 == '%') break;
      *pcVar3 = cVar1;
      if (cVar1 == '\0') {
        *(undefined8 *)(puVar12 + lVar2 + -8) = 0x11d7cc;
        sVar5 = strlen("SCHILY.xattr.");
        *(undefined8 *)(puVar12 + lVar2 + -8) = 0x11d7de;
        FUN_0011d5d0(param_1,__dest + sVar5,
                     (void *)((ulong)(puVar12 + lVar2 + 0xf) & 0xfffffffffffffff0),param_4);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return;
        }
                    /* WARNING: Subroutine does not return */
        *(undefined **)(puVar12 + lVar2 + -8) = &UNK_0011d81d;
        __stack_chk_fail();
      }
LAB_0011d767:
      pcVar6 = pcVar6 + 1;
      pcVar3 = pcVar7;
    }
    if (pcVar6[1] != '3') {
      if ((pcVar6[1] != '2') || (pcVar6[2] != '5')) {
LAB_0011d78b:
        *pcVar3 = '%';
        goto LAB_0011d767;
      }
      *pcVar3 = '%';
      pcVar6 = pcVar6 + 3;
      pcVar3 = pcVar7;
      goto LAB_0011d76f;
    }
    if (pcVar6[2] != 'D') goto LAB_0011d78b;
    *pcVar3 = '=';
    pcVar6 = pcVar6 + 3;
    pcVar3 = pcVar7;
  } while( true );
}




// Function: xheader_xattr_copy @ 0x1d820

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
      FUN_0011aa20(param_2,param_3,*puVar1,puVar1[1],puVar1[2]);
      uVar2 = uVar3;
    } while (uVar3 < *(ulong *)(param_1 + 0x158));
    return;
  }
  return;
}




// Function: xheader_decode @ 0x1d8a0

void xheader_decode(long param_1)

{
  char cVar1;
  long lVar2;
  long lVar3;
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0011ab80(DAT_00182388,param_1);
  FUN_0011ab80(DAT_00182378,param_1);
  lVar3 = *(long *)(param_1 + 0x170);
  if (lVar3 != 0) {
    lVar2 = *(long *)(param_1 + 0x178);
    local_28 = lVar2 + 0x200;
    while( true ) {
      cVar1 = FUN_0011b390(lVar3,lVar2,&local_28,FUN_0011b9b0,param_1);
      if (cVar1 == '\0') break;
      lVar3 = *(long *)(param_1 + 0x170);
      lVar2 = *(long *)(param_1 + 0x178);
    }
  }
  FUN_0011ab80(DAT_00182380,param_1);
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




// Function: xheader_decode_global @ 0x1d980

void xheader_decode_global(long param_1)

{
  long *plVar1;
  char cVar2;
  long *__ptr;
  long lVar3;
  long lVar4;
  long in_FS_OFFSET;
  long local_38;
  long local_30;
  
  lVar4 = *(long *)(param_1 + 8);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (lVar4 != 0) {
    lVar3 = *(long *)(param_1 + 0x10);
    local_38 = lVar3 + 0x200;
    __ptr = DAT_00182378;
    if (DAT_00182378 != (long *)0x0) {
      do {
        plVar1 = (long *)*__ptr;
        free((void *)__ptr[1]);
        free((void *)__ptr[2]);
        free(__ptr);
        __ptr = plVar1;
      } while (plVar1 != (long *)0x0);
      lVar4 = *(long *)(param_1 + 8);
      lVar3 = *(long *)(param_1 + 0x10);
    }
    DAT_00182378 = (long *)0x0;
    while( true ) {
      cVar2 = FUN_0011b390(lVar4,lVar3,&local_38,FUN_0011bab0,&DAT_00182378);
      if (cVar2 == '\0') break;
      lVar4 = *(long *)(param_1 + 8);
      lVar3 = *(long *)(param_1 + 0x10);
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xheader_store @ 0x1da70

void xheader_store(char *param_1,long param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined *puVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  
  if (*(long *)(param_2 + 0x178) != 0) {
    return;
  }
  lVar4 = FUN_0011ab00();
  if ((lVar4 != 0) && (puVar1 = DAT_00182390, *(long *)(lVar4 + 8) != 0)) {
    for (; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
      iVar3 = fnmatch((char *)puVar1[1],param_1,0);
      if (iVar3 == 0) {
        return;
      }
    }
    puVar1 = DAT_00182380;
    if (*(long *)(param_2 + 0x168) == 0) {
      uVar5 = FUN_0014ecc0(0x58);
      puVar2 = PTR_free_0017efc8;
      *(undefined8 *)(param_2 + 0x168) = uVar5;
      _obstack_begin(uVar5,0,0,FUN_0014ecc0,puVar2);
      puVar1 = DAT_00182380;
    }
    while( true ) {
      if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x0011db3d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (**(code **)(lVar4 + 8))(param_2,param_1,param_2 + 0x168,param_3);
        return;
      }
      iVar3 = strcmp((char *)puVar1[1],param_1);
      if (iVar3 == 0) break;
      puVar1 = (undefined8 *)*puVar1;
    }
  }
  return;
}




// Function: xheader_read @ 0x1db80

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
  lVar1 = FUN_0014ecc0(lVar4 + 0x201);
  *(long *)(param_1 + 0x10) = lVar1;
  *(undefined1 *)(lVar1 + 0x200 + lVar4) = 0;
  do {
    uVar7 = 0x200;
    if (uVar9 < 0x201) {
      uVar7 = uVar9;
    }
    if (param_2 == (undefined8 *)0x0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = dcgettext(0,"Unexpected EOF in archive",5);
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
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
    FUN_0010ccd0();
    param_2 = (undefined8 *)FUN_0010d360();
  } while (0 < (long)uVar9);
  return;
}




// Function: xheader_finish @ 0x1dcd0

void xheader_finish(long *param_1)

{
  undefined8 *puVar1;
  long lVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  
  for (puVar1 = DAT_00182380; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    FUN_0011bdb0(puVar1[2],puVar1[1],param_1);
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




// Function: xheader_write_global @ 0x1dd60

void xheader_write_global(long *param_1)

{
  undefined *puVar1;
  void *__ptr;
  long lVar2;
  long *plVar3;
  
  if (DAT_00182388 != (long *)0x0) {
    plVar3 = DAT_00182388;
    if (*param_1 == 0) {
      lVar2 = FUN_0014ecc0(0x58);
      puVar1 = PTR_free_0017efc8;
      *param_1 = lVar2;
      _obstack_begin(lVar2,0,0,FUN_0014ecc0,puVar1);
      plVar3 = DAT_00182388;
      if (DAT_00182388 == (long *)0x0) goto LAB_0011dd98;
    }
    do {
      FUN_0011bdb0(plVar3[2],plVar3[1],param_1);
      plVar3 = (long *)*plVar3;
    } while (plVar3 != (long *)0x0);
  }
LAB_0011dd98:
  if (*param_1 == 0) {
    return;
  }
  FUN_0011dcd0(param_1);
  __ptr = (void *)FUN_0011d220();
  FUN_0011d2d0(0x67,__ptr,DAT_00182a20,param_1);
  free(__ptr);
  return;
}




// Function: xheader_string_begin @ 0x1de70

void xheader_string_begin(long param_1)

{
  *(undefined8 *)(param_1 + 0x18) = 0;
  return;
}




// Function: xheader_string_add @ 0x1de80

void xheader_string_add(long *param_1,char *param_2)

{
  undefined *puVar1;
  size_t sVar2;
  long lVar3;
  
  if (param_1[2] != 0) {
    return;
  }
  if (*param_1 == 0) {
    lVar3 = FUN_0014ecc0(0x58);
    puVar1 = PTR_free_0017efc8;
    *param_1 = lVar3;
    _obstack_begin(lVar3,0,0,FUN_0014ecc0,puVar1);
  }
  sVar2 = strlen(param_2);
  param_1[3] = param_1[3] + sVar2;
  sVar2 = strlen(param_2);
  FUN_0011a9b0(param_1,param_2,sVar2);
  return;
}




// Function: xheader_string_end @ 0x1df00

undefined8 xheader_string_end(long *param_1,char *param_2)

{
  undefined *puVar1;
  size_t sVar2;
  char *pcVar3;
  long lVar4;
  undefined1 *puVar5;
  ulong uVar6;
  undefined8 uVar7;
  long lVar8;
  undefined1 *puVar9;
  undefined1 *puVar10;
  char *__dest;
  long lVar11;
  long in_FS_OFFSET;
  bool bVar12;
  undefined1 local_58 [20];
  undefined1 local_44 [4];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar7 = 0;
  if (param_1[2] == 0) {
    if (*param_1 == 0) {
      lVar8 = FUN_0014ecc0(0x58);
      puVar1 = PTR_free_0017efc8;
      *param_1 = lVar8;
      _obstack_begin(lVar8,0,0,FUN_0014ecc0,puVar1);
    }
    sVar2 = strlen(param_2);
    lVar8 = param_1[3];
    puVar10 = (undefined1 *)0x0;
    do {
      pcVar3 = (char *)FUN_00145370(puVar10 + lVar8 + 3 + sVar2,local_58);
      bVar12 = puVar10 != local_44 + -(long)pcVar3;
      puVar10 = local_44 + -(long)pcVar3;
    } while (bVar12);
    sVar2 = strlen(param_2);
    puVar10 = local_44 + sVar2 + -(long)pcVar3 + 2;
    lVar8 = *param_1;
    puVar9 = *(undefined1 **)(lVar8 + 0x20);
    lVar4 = *(long *)(lVar8 + 0x18);
    lVar11 = lVar8;
    if (puVar9 + -lVar4 < puVar10) {
      _obstack_newchunk(lVar8,puVar10);
      lVar11 = *param_1;
      lVar4 = *(long *)(lVar8 + 0x18);
      puVar9 = *(undefined1 **)(lVar11 + 0x20);
    }
    *(undefined1 **)(lVar8 + 0x18) = puVar10 + lVar4;
    puVar5 = *(undefined1 **)(lVar11 + 0x18);
    param_1[1] = (long)(puVar10 + param_1[1]);
    if (puVar5 == puVar9) {
      _obstack_newchunk(lVar11,1);
      puVar5 = *(undefined1 **)(lVar11 + 0x18);
    }
    *(undefined1 **)(lVar11 + 0x18) = puVar5 + 1;
    *puVar5 = 10;
    param_1[1] = param_1[1] + 1;
    uVar6 = ~param_1[3];
    __dest = (char *)((uVar6 - (long)puVar10) + *(long *)(*param_1 + 0x18));
    memmove((void *)(*(long *)(*param_1 + 0x18) + uVar6),__dest,param_1[3]);
    pcVar3 = stpcpy(__dest,pcVar3);
    *pcVar3 = ' ';
    pcVar3 = stpcpy(pcVar3 + 1,param_2);
    *pcVar3 = '=';
    uVar7 = 1;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_directory_meta @ 0x1e0d0

ulong hash_directory_meta(long param_1,ulong param_2)

{
  return (ulong)(*(long *)(param_1 + 0x18) + *(long *)(param_1 + 0x20)) % param_2;
}




// Function: compare_directory_meta @ 0x1e0f0

undefined8 compare_directory_meta(long param_1,long param_2)

{
  if (*(long *)(param_1 + 0x18) != *(long *)(param_2 + 0x18)) {
    return 0;
  }
  return CONCAT71((int7)((ulong)*(long *)(param_2 + 0x20) >> 8),
                  *(long *)(param_1 + 0x20) == *(long *)(param_2 + 0x20));
}




// Function: make_directory @ 0x1e190

undefined8 * make_directory(char *param_1,undefined8 param_2)

{
  size_t sVar1;
  undefined8 *puVar2;
  void *pvVar3;
  size_t __n;
  
  sVar1 = strlen(param_1);
  puVar2 = (undefined8 *)FUN_0014ecc0(0x60);
  *puVar2 = 0;
  puVar2[6] = 0;
  puVar2[5] = 0;
  puVar2[8] = 0;
  *(undefined4 *)((long)puVar2 + 0x3c) = 0;
  if ((sVar1 < 2) || (__n = sVar1 - 1, param_1[sVar1 - 1] != '/')) {
    __n = sVar1;
    sVar1 = sVar1 + 1;
  }
  pvVar3 = (void *)FUN_0014ecc0(sVar1);
  puVar2[0xb] = pvVar3;
  pvVar3 = memcpy(pvVar3,param_1,__n);
  puVar2[10] = param_2;
  *(undefined1 *)((long)pvVar3 + __n) = 0;
  puVar2[9] = 0;
  return puVar2;
}




// Function: find_directory @ 0x1e240

undefined8 find_directory(undefined8 param_1)

{
  undefined8 uVar1;
  void *__ptr;
  
  uVar1 = 0;
  if (DAT_001823b0 != 0) {
    uVar1 = FUN_00126700(DAT_00182580,param_1);
    __ptr = (void *)FUN_0011e190(param_1,uVar1);
    uVar1 = FUN_00143960(DAT_001823b0,__ptr);
    free(*(void **)((long)__ptr + 0x50));
    free(*(void **)((long)__ptr + 0x58));
    free(__ptr);
  }
  return uVar1;
}




// Function: compare_directory_canonical_names @ 0x1e2c0

bool compare_directory_canonical_names(long param_1,long param_2)

{
  int iVar1;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x50),*(char **)(param_2 + 0x50));
  return iVar1 == 0;
}




// Function: dumpdir_create0 @ 0x1e2e0

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
LAB_0011e345:
        local_50 = local_50 + 1;
      }
      else {
        pcVar2 = strchr(param_2,(int)cVar9);
        if (pcVar2 != (char *)0x0) goto LAB_0011e345;
      }
      pcVar10 = pcVar10 + sVar1 + 1;
      lVar5 = lVar5 + 1;
      cVar9 = *pcVar10;
    } while (cVar9 != '\0');
    lVar6 = sVar7 + 0x20;
    lVar8 = local_50 + 1;
  }
  puVar3 = (undefined8 *)FUN_0014ecc0(lVar6);
  *puVar3 = puVar3 + 4;
  memcpy(puVar3 + 4,param_1,sVar7);
  puVar3[1] = lVar5;
  puVar3[2] = local_50;
  puVar4 = (undefined8 *)FUN_0014edb0(lVar8,8);
  pcVar10 = (char *)*puVar3;
  puVar3[3] = puVar4;
  cVar9 = *pcVar10;
  if (cVar9 != '\0') {
    lVar8 = 0;
    do {
      if (param_2 == (char *)0x0) {
LAB_0011e3d2:
        puVar4[lVar8] = pcVar10 + 1;
        lVar8 = lVar8 + 1;
      }
      else {
        pcVar2 = strchr(param_2,(int)cVar9);
        if (pcVar2 != (char *)0x0) goto LAB_0011e3d2;
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




// Function: note_directory @ 0x1e440

long note_directory(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,char param_6,char param_7,long param_8)

{
  undefined8 uVar1;
  long lVar2;
  long lVar3;
  
  uVar1 = FUN_00126700(DAT_00182580,param_1);
  lVar2 = FUN_0011e190(param_1,uVar1);
  lVar3 = lVar2;
  if (DAT_001823b8 != (long *)0x0) {
    *DAT_001823b8 = lVar2;
    lVar3 = DAT_001823c0;
  }
  DAT_001823c0 = lVar3;
  DAT_001823b8 = (long *)lVar2;
  *(undefined8 *)(lVar2 + 8) = param_2;
  *(undefined8 *)(lVar2 + 0x10) = param_3;
  *(undefined8 *)(lVar2 + 0x18) = param_4;
  *(undefined8 *)(lVar2 + 0x20) = param_5;
  *(undefined4 *)(lVar2 + 0x38) = 1;
  if (param_6 != '\0') {
    *(uint *)(lVar2 + 0x3c) = *(uint *)(lVar2 + 0x3c) | 2;
  }
  if (param_7 != '\0') {
    *(uint *)(lVar2 + 0x3c) = *(uint *)(lVar2 + 0x3c) | 4;
  }
  uVar1 = 0;
  if (param_8 != 0) {
    uVar1 = FUN_0011e2e0(param_8,&DAT_001649db);
  }
  lVar3 = DAT_001823b0;
  *(undefined8 *)(lVar2 + 0x28) = uVar1;
  if ((((lVar3 != 0) ||
       (lVar3 = FUN_00143bf0(0,0,FUN_0011e5b0,FUN_0011e2c0,0), DAT_001823b0 = lVar3, lVar3 != 0)) &&
      (lVar3 = FUN_00144240(lVar3,lVar2), lVar3 != 0)) &&
     (((DAT_001823a8 != 0 ||
       (DAT_001823a8 = FUN_00143bf0(0,0,FUN_0011e0d0,FUN_0011e0f0,0), DAT_001823a8 != 0)) &&
      (lVar3 = FUN_00144240(DAT_001823a8,lVar2), lVar3 != 0)))) {
    return lVar2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117020();
}




// Function: hash_directory_canonical_name @ 0x1e5b0

void hash_directory_canonical_name(long param_1)

{
  FUN_00143b80(*(undefined8 *)(param_1 + 0x50));
  return;
}




// Function: obstack_code_rename @ 0x1e5c0

void obstack_code_rename(long param_1,char *param_2,char *param_3)

{
  ulong uVar1;
  size_t sVar2;
  undefined1 *puVar3;
  void *pvVar4;
  
  if (*param_2 == '\0') {
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) == puVar3) {
LAB_0011e6aa:
      _obstack_newchunk(param_1,1);
      puVar3 = *(undefined1 **)(param_1 + 0x18);
    }
  }
  else {
    param_2 = (char *)FUN_00133640(param_2,0,DAT_00182b92);
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) == puVar3) goto LAB_0011e6aa;
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
    if (*(undefined1 **)(param_1 + 0x20) != puVar3) goto LAB_0011e640;
  }
  else {
    param_3 = (char *)FUN_00133640(param_3,0,DAT_00182b92);
    puVar3 = *(undefined1 **)(param_1 + 0x18);
    if (*(undefined1 **)(param_1 + 0x20) != puVar3) goto LAB_0011e640;
  }
  _obstack_newchunk(param_1,1);
  puVar3 = *(undefined1 **)(param_1 + 0x18);
LAB_0011e640:
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




// Function: read_obstack @ 0x1e730

uint read_obstack(_IO_FILE *param_1,long param_2,long *param_3)

{
  byte *pbVar1;
  undefined1 *puVar2;
  uint uVar3;
  long lVar4;
  
  pbVar1 = (byte *)param_1->_IO_read_ptr;
  if (pbVar1 < param_1->_IO_read_end) {
    param_1->_IO_read_ptr = (char *)(pbVar1 + 1);
    uVar3 = (uint)*pbVar1;
  }
  else {
    uVar3 = __uflow(param_1);
  }
  lVar4 = 0;
  while (1 < uVar3 + 1) {
    puVar2 = *(undefined1 **)(param_2 + 0x18);
    if (*(undefined1 **)(param_2 + 0x20) == puVar2) {
      _obstack_newchunk(param_2,1);
      puVar2 = *(undefined1 **)(param_2 + 0x18);
    }
    *(undefined1 **)(param_2 + 0x18) = puVar2 + 1;
    *puVar2 = (char)uVar3;
    pbVar1 = (byte *)param_1->_IO_read_ptr;
    if (pbVar1 < param_1->_IO_read_end) {
      param_1->_IO_read_ptr = (char *)(pbVar1 + 1);
      uVar3 = (uint)*pbVar1;
    }
    else {
      uVar3 = __uflow(param_1);
    }
    lVar4 = lVar4 + 1;
  }
  puVar2 = *(undefined1 **)(param_2 + 0x18);
  if (*(undefined1 **)(param_2 + 0x20) == puVar2) {
    _obstack_newchunk(param_2,1);
    puVar2 = *(undefined1 **)(param_2 + 0x18);
  }
  *(undefined1 **)(param_2 + 0x18) = puVar2 + 1;
  *puVar2 = 0;
  *param_3 = lVar4;
  return uVar3;
}




// Function: read_num @ 0x1e820

undefined8
read_num(_IO_FILE *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 *param_5)

{
  uint uVar1;
  undefined4 extraout_var;
  __off_t _Var3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  undefined4 extraout_var_00;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined1 uVar11;
  int iVar12;
  long lVar13;
  long in_FS_OFFSET;
  bool bVar14;
  undefined1 *puVar15;
  undefined1 *puVar16;
  undefined1 local_b8 [32];
  undefined1 local_98 [32];
  undefined1 local_78 [32];
  undefined1 local_58 [24];
  long local_40;
  byte *pbVar2;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pbVar2 = (byte *)param_1->_IO_read_ptr;
  if (pbVar2 < param_1->_IO_read_end) {
    param_1->_IO_read_ptr = (char *)(pbVar2 + 1);
    uVar1 = (uint)*pbVar2;
  }
  else {
    uVar1 = __uflow(param_1);
    pbVar2 = (byte *)CONCAT44(extraout_var_00,uVar1);
  }
  bVar14 = uVar1 == 0x2d;
  lVar13 = 0;
  do {
    iVar12 = (int)lVar13;
    uVar4 = CONCAT71((int7)((ulong)pbVar2 >> 8),iVar12 == 0 && bVar14);
    uVar11 = (undefined1)uVar1;
    if (iVar12 == 0 && bVar14) {
      pbVar2 = (byte *)param_1->_IO_read_ptr;
      local_b8[0] = uVar11;
      if (param_1->_IO_read_end <= pbVar2) goto LAB_0011e8cc;
LAB_0011e8a5:
      param_1->_IO_read_ptr = (char *)(pbVar2 + 1);
      uVar1 = (uint)*pbVar2;
    }
    else {
      if (9 < uVar1 - 0x30) {
        local_b8[iVar12] = 0;
        if ((int)uVar1 < 0) {
          if ((param_1->_flags & 0x20) != 0) {
            FUN_00133090(DAT_00182b28);
            goto LAB_0011ebab;
          }
          if (iVar12 != 0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar4 = dcgettext(0,"Unexpected EOF in snapshot file",5);
            uVar5 = FUN_0014ce40(DAT_00182b28);
            error(0,0,"%s: %s",uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
            FUN_00116fd0();
          }
        }
        else {
          if (uVar1 != 0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            _Var3 = ftello(param_1);
            uVar4 = FUN_001452d0(_Var3,local_98);
            uVar5 = FUN_0014ce40(DAT_00182b28);
            uVar6 = dcgettext(0,"%s: byte %s: %s %s followed by invalid byte 0x%02x",5);
            puVar16 = (undefined1 *)(ulong)uVar1;
            puVar15 = local_b8;
            goto LAB_0011e92f;
          }
          uVar4 = param_4;
          uVar5 = FUN_001255e0(local_b8,0,param_3);
          *param_5 = uVar5;
          piVar7 = __errno_location();
          iVar12 = *piVar7;
          if (iVar12 != 0) {
            if (iVar12 != 0x22) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              _Var3 = ftello(param_1);
              uVar4 = FUN_001452d0(_Var3,local_98);
              uVar5 = FUN_0014ce40(DAT_00182b28);
              uVar6 = dcgettext(0,"%s: byte %s: %s %s",5);
              error(0,iVar12,uVar6,uVar5,uVar4,param_2,local_b8,uVar6);
                    /* WARNING: Subroutine does not return */
              FUN_00116fd0();
            }
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar5 = FUN_00145370(param_4,local_58);
            uVar6 = FUN_00145230(param_3,local_78);
            _Var3 = ftello(param_1);
            uVar8 = FUN_001452d0(_Var3,local_98);
            uVar9 = FUN_0014ce40(DAT_00182b28);
            uVar10 = dcgettext(0,"%s: byte %s: (valid range %s..%s)\n\t%s %s",5);
            error(0,0x22,uVar10,uVar9,uVar8,uVar6,uVar5,param_2,local_b8,uVar4);
                    /* WARNING: Subroutine does not return */
            FUN_00116fd0();
          }
          uVar4 = 1;
        }
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return uVar4;
        }
LAB_0011ebab:
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      local_b8[lVar13] = uVar11;
      puVar16 = local_b8;
      if (lVar13 == 0x14) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        _Var3 = ftello(param_1);
        uVar4 = FUN_001452d0(_Var3,local_98);
        uVar5 = FUN_0014ce40(DAT_00182b28);
        uVar6 = dcgettext(0,"%s: byte %s: %s %.*s... too long",5);
        puVar15 = (undefined1 *)0x15;
LAB_0011e92f:
        error(0,0,uVar6,uVar5,uVar4,param_2,puVar15,puVar16);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      pbVar2 = (byte *)param_1->_IO_read_ptr;
      if (pbVar2 < param_1->_IO_read_end) goto LAB_0011e8a5;
LAB_0011e8cc:
      uVar1 = __uflow(param_1);
      pbVar2 = (byte *)CONCAT44(extraout_var,uVar1);
    }
    lVar13 = lVar13 + 1;
  } while( true );
}




// Function: read_timespec @ 0x1ebb0

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
  cVar1 = FUN_0011e820(param_1,"sec",0x8000000000000000,0x7fffffffffffffff,&local_30);
  if (cVar1 != '\0') {
    cVar1 = FUN_0011e820(param_1,"nsec",0,999999999,&local_28);
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
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar2 = dcgettext(0,"Unexpected EOF in snapshot file",5);
  uVar3 = FUN_0014ce40(DAT_00182b28);
  error(0,0,"%s: %s",uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: dumpdir_locate @ 0x1ec90

char * dumpdir_locate(long param_1,char *param_2)

{
  long lVar1;
  char *__s2;
  int iVar2;
  ulong uVar3;
  ulong uVar4;
  ulong uVar5;
  
  if (param_1 == 0) {
    return (char *)0x0;
  }
  uVar4 = 0;
  lVar1 = *(long *)(param_1 + 0x18);
  uVar3 = *(ulong *)(param_1 + 0x10);
  do {
    uVar5 = uVar3;
    if (uVar5 <= uVar4) {
      return (char *)0x0;
    }
    while( true ) {
      uVar3 = uVar5 + uVar4 >> 1;
      __s2 = *(char **)(lVar1 + uVar3 * 8);
      iVar2 = strcmp(param_2,__s2);
      if (iVar2 < 0) break;
      if (iVar2 == 0) {
        return __s2 + -1;
      }
      uVar4 = uVar3 + 1;
      if (uVar5 <= uVar4) {
        return (char *)0x0;
      }
    }
  } while( true );
}




// Function: find_directory_meta @ 0x1ed10

undefined8 find_directory_meta(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  void *__ptr;
  undefined8 uVar2;
  
  uVar2 = 0;
  if (DAT_001823a8 != 0) {
    __ptr = (void *)FUN_0011e190(&DAT_0016a7d3,0);
    lVar1 = DAT_001823a8;
    *(undefined8 *)((long)__ptr + 0x18) = param_1;
    *(undefined8 *)((long)__ptr + 0x20) = param_2;
    uVar2 = FUN_00143960(lVar1,__ptr);
    free(*(void **)((long)__ptr + 0x50));
    free(*(void **)((long)__ptr + 0x58));
    free(__ptr);
  }
  return uVar2;
}




// Function: procdir @ 0x1ed80

long procdir(char *param_1,long param_2,uint param_3,undefined1 *param_4)

{
  ulong uVar1;
  undefined8 *puVar2;
  void *__ptr;
  bool bVar3;
  int iVar4;
  uint uVar5;
  long lVar6;
  long lVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  size_t sVar11;
  char *pcVar12;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = *(ulong *)(param_2 + 0x58);
  lVar6 = FUN_0011e240();
  if (lVar6 == 0) {
    lVar7 = FUN_0011ed10(*(undefined8 *)(param_2 + 0x58),*(undefined8 *)(param_2 + 0x60));
    lVar6 = FUN_0011e440(param_1,*(undefined8 *)(param_2 + 0xb0),*(undefined8 *)(param_2 + 0xb8),
                         *(undefined8 *)(param_2 + 0x58),*(undefined8 *)(param_2 + 0x60),
                         uVar1 >> 0x3f);
    if (lVar7 == 0) {
      *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 8;
      if ((DAT_00181b80._1_1_ & 0x10) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar8 = FUN_0014ce40(param_1);
        uVar9 = dcgettext(0,"%s: Directory is new",5);
        error(0,0,uVar9,uVar8);
      }
      iVar4 = 2;
      if (((DAT_00182b28 == 0) &&
          (-1 < (int)(((uint)(DAT_00182b08 < *(long *)(param_2 + 0xb8)) -
                      (uint)(*(long *)(param_2 + 0xb8) < DAT_00182b08)) +
                     ((uint)(DAT_00182b00 < *(long *)(param_2 + 0xb0)) -
                     (uint)(*(long *)(param_2 + 0xb0) < DAT_00182b00)) * 2))) &&
         (iVar4 = 1, DAT_00182b8c != 0)) {
        iVar4 = 1 - ((int)(((uint)(DAT_00182b08 < *(long *)(param_2 + 200)) -
                           (uint)(*(long *)(param_2 + 200) < DAT_00182b08)) +
                          ((uint)(DAT_00182b00 < *(long *)(param_2 + 0xc0)) -
                          (uint)(*(long *)(param_2 + 0xc0) < DAT_00182b00)) * 2) >> 0x1f);
      }
      *(int *)(lVar6 + 0x38) = iVar4;
      bVar3 = false;
    }
    else {
      pcVar12 = *(char **)(lVar7 + 0x58);
      iVar4 = strcmp(pcVar12,param_1);
      if (iVar4 != 0) {
        if ((DAT_00181b80._1_1_ & 0x20) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
            pcVar12 = *(char **)(lVar7 + 0x58);
          }
          uVar8 = FUN_0014d2b0(1,pcVar12);
          uVar9 = FUN_0014ce40(param_1);
          uVar10 = dcgettext(0,"%s: Directory has been renamed from %s",5);
          error(0,0,uVar10,uVar9,uVar8);
          pcVar12 = *(char **)(lVar7 + 0x58);
        }
        *(long *)(lVar6 + 0x40) = lVar7;
        *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 0x10;
        *(uint *)(lVar7 + 0x3c) = *(uint *)(lVar7 + 0x3c) & 0xffffffef;
        sVar11 = strlen(pcVar12);
        strlen(param_1);
        for (puVar2 = DAT_001823c0; puVar2 != (undefined8 *)0x0; puVar2 = (undefined8 *)*puVar2) {
          FUN_00125500(puVar2 + 0xb,pcVar12,sVar11,param_1);
        }
      }
      *(undefined4 *)(lVar6 + 0x38) = 1;
      bVar3 = false;
    }
LAB_0011ef85:
    if (((DAT_00182ad1 == '\0') || (*(long *)(param_2 + 0x198) == 0)) ||
       (*(long *)(param_2 + 0x58) == *(long *)(*(long *)(param_2 + 0x198) + 0x58))) {
      if (((param_3 & 0x10) != 0) && (*(uint *)(lVar6 + 0x38) = param_3 & 3, (param_3 & 3) == 0)) {
        *param_4 = 0x4e;
      }
      if ((bVar3) && ((DAT_00181b80._1_1_ & 0x20) != 0)) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar8 = FUN_0014ce40(param_1);
        uVar9 = dcgettext(0,"%s: Directory has been renamed",5);
        error(0,0,uVar9,uVar8);
      }
      iVar4 = *(int *)(lVar6 + 0x38);
    }
    else {
      if ((DAT_00181b80._2_1_ & 4) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar8 = FUN_0014ce40(*(undefined8 *)(lVar6 + 0x58));
        uVar9 = dcgettext(0,"%s: directory is on a different filesystem; not dumped",5);
        error(0,0,uVar9,uVar8);
      }
      __ptr = *(void **)(lVar6 + 0x28);
      *(undefined4 *)(lVar6 + 0x38) = 0;
      if (__ptr == (void *)0x0) {
        *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 1;
        goto LAB_0011ef40;
      }
      free(*(void **)((long)__ptr + 0x18));
      free(__ptr);
      *(undefined8 *)(lVar6 + 0x28) = 0;
      iVar4 = *(int *)(lVar6 + 0x38);
    }
    *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 1;
    if (iVar4 != 0) {
      iVar4 = FUN_001149e0(param_2,&local_48);
      if (iVar4 == 2) {
        uVar8 = dcgettext(0,"contents not dumped",5);
        FUN_00112e70(param_1,local_48,uVar8);
        *(undefined8 *)(lVar6 + 0x48) = local_48;
      }
      else if (iVar4 == 3) {
        uVar8 = dcgettext(0,"directory not dumped",5);
        FUN_00112e70(param_1,local_48,uVar8);
        *param_4 = 0x4e;
        *(undefined4 *)(lVar6 + 0x38) = 0;
      }
      else if (iVar4 == 1) {
        uVar8 = dcgettext(0,"contents not dumped",5);
        FUN_00112e70(param_1,local_48,uVar8);
        *(undefined4 *)(lVar6 + 0x38) = 0;
        *(undefined8 *)(lVar6 + 0x48) = local_48;
      }
    }
  }
  else {
    if ((*(byte *)(lVar6 + 0x3c) & 1) == 0) {
LAB_0011edde:
      iVar4 = strcmp(*(char **)(lVar6 + 0x58),param_1);
      if (iVar4 == 0) {
        if (((DAT_00182b20 == '\0') || (((*(byte *)(lVar6 + 0x3c) & 2) != 0 && ((long)uVar1 < 0))))
           || (lVar7 = *(long *)(param_2 + 0x58), *(long *)(lVar6 + 0x18) == lVar7)) {
          if (*(long *)(lVar6 + 0x20) != *(long *)(param_2 + 0x60)) {
            lVar7 = *(long *)(param_2 + 0x58);
            goto LAB_0011ee2f;
          }
          *(undefined4 *)(lVar6 + 0x38) = 1;
          bVar3 = false;
          uVar5 = *(uint *)(lVar6 + 0x3c);
        }
        else {
LAB_0011ee2f:
          lVar7 = FUN_0011ed10(lVar7);
          if (lVar7 == 0) {
            uVar8 = *(undefined8 *)(param_2 + 0x58);
            *(undefined4 *)(lVar6 + 0x38) = 2;
            bVar3 = true;
            *(undefined8 *)(lVar6 + 0x18) = uVar8;
            *(undefined8 *)(lVar6 + 0x20) = *(undefined8 *)(param_2 + 0x60);
          }
          else {
            pcVar12 = *(char **)(lVar7 + 0x58);
            iVar4 = strcmp(pcVar12,param_1);
            if (iVar4 != 0) {
              if ((DAT_00181b80._1_1_ & 0x20) != 0) {
                if (DAT_00182d50 != (code *)0x0) {
                  (*DAT_00182d50)();
                  pcVar12 = *(char **)(lVar7 + 0x58);
                }
                uVar8 = FUN_0014d2b0(1,pcVar12);
                uVar9 = FUN_0014ce40(param_1);
                uVar10 = dcgettext(0,"%s: Directory has been renamed from %s",5);
                error(0,0,uVar10,uVar9,uVar8);
                pcVar12 = *(char **)(lVar7 + 0x58);
              }
              *(long *)(lVar6 + 0x40) = lVar7;
              *(uint *)(lVar6 + 0x3c) = *(uint *)(lVar6 + 0x3c) | 0x10;
              *(uint *)(lVar7 + 0x3c) = *(uint *)(lVar7 + 0x3c) & 0xffffffef;
              sVar11 = strlen(pcVar12);
              strlen(param_1);
              for (puVar2 = DAT_001823c0; puVar2 != (undefined8 *)0x0;
                  puVar2 = (undefined8 *)*puVar2) {
                FUN_00125500(puVar2 + 0xb,pcVar12,sVar11,param_1);
              }
            }
            *(undefined4 *)(lVar6 + 0x38) = 1;
            bVar3 = false;
          }
          uVar5 = *(uint *)(lVar6 + 0x3c);
          if ((long)uVar1 < 0) {
            uVar5 = *(uint *)(lVar6 + 0x3c) | 2;
          }
        }
        *(uint *)(lVar6 + 0x3c) = uVar5 | 4;
        goto LAB_0011ef85;
      }
    }
    else if ((param_3 & 0x20) != 0) {
      FUN_00125120(lVar6 + 0x58,param_1);
      goto LAB_0011edde;
    }
    *param_4 = 0x4e;
  }
LAB_0011ef40:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar6;
}




// Function: write_directory_file_entry @ 0x1f3f0

uint write_directory_file_entry(long param_1,_IO_FILE *param_2)

{
  byte *pbVar1;
  byte *pbVar2;
  long lVar3;
  long lVar4;
  uint uVar5;
  int iVar6;
  char *pcVar7;
  size_t sVar8;
  long *__ptr;
  byte bVar9;
  byte *pbVar10;
  long in_FS_OFFSET;
  undefined1 auStack_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(uint *)(param_1 + 0x3c) & 4) != 0) {
    uVar5 = *(uint *)(param_1 + 0x3c) & 2;
    pbVar10 = &DAT_00166c5b;
    bVar9 = 0x31 - (uVar5 == 0);
    if (uVar5 == 0) {
      pbVar10 = (byte *)0x16af34;
    }
    pbVar1 = pbVar10 + 1;
    do {
      pbVar2 = (byte *)param_2->_IO_write_ptr;
      if (pbVar2 < param_2->_IO_write_end) {
        param_2->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar9;
      }
      else {
        iVar6 = __overflow(param_2,(uint)bVar9);
        if (iVar6 == -1) goto LAB_0011f463;
      }
      if (pbVar1 == pbVar10) goto LAB_0011f463;
      pbVar10 = pbVar10 + 1;
      bVar9 = *pbVar10;
    } while( true );
  }
LAB_0011f602:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return ((uint)param_2->_flags >> 5 ^ 1) & 1;
LAB_0011f463:
  pcVar7 = (char *)FUN_001255c0(*(undefined8 *)(param_1 + 8),0x8000000000000000,0x7fffffffffffffff,
                                auStack_48);
  sVar8 = strlen(pcVar7);
  fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
  pcVar7 = (char *)FUN_00145230(*(undefined8 *)(param_1 + 0x10),auStack_48);
  sVar8 = strlen(pcVar7);
  fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
  pcVar7 = (char *)FUN_001255c0(*(undefined8 *)(param_1 + 0x18),0,0xffffffffffffffff,auStack_48);
  sVar8 = strlen(pcVar7);
  fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
  pcVar7 = (char *)FUN_001255c0(*(undefined8 *)(param_1 + 0x20),0,0xffffffffffffffff,auStack_48);
  sVar8 = strlen(pcVar7);
  fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
  pcVar7 = *(char **)(param_1 + 0x58);
  sVar8 = strlen(pcVar7);
  fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
  lVar3 = *(long *)(param_1 + 0x28);
  if (lVar3 != 0) {
    __ptr = (long *)FUN_0014ecc0(0x18);
    lVar4 = *(long *)(lVar3 + 0x10);
    *__ptr = lVar3;
    *(undefined4 *)(__ptr + 1) = 0;
    __ptr[2] = 0;
    if (lVar4 != 0) {
      lVar3 = **(long **)(lVar3 + 0x18);
      __ptr[2] = 1;
      pcVar7 = (char *)(lVar3 + -1);
      do {
        sVar8 = strlen(pcVar7);
        fwrite_unlocked(pcVar7,sVar8 + 1,1,param_2);
        pcVar7 = (char *)FUN_0011e120(__ptr);
      } while (pcVar7 != (char *)0x0);
    }
    free(__ptr);
  }
  pcVar7 = param_2->_IO_write_ptr;
  if (pcVar7 < param_2->_IO_write_end) {
    param_2->_IO_write_ptr = pcVar7 + 1;
    *pcVar7 = '\0';
  }
  else {
    iVar6 = __overflow(param_2,0);
    if (iVar6 == -1) goto LAB_0011f602;
  }
  pcVar7 = param_2->_IO_write_ptr;
  if (pcVar7 < param_2->_IO_write_end) {
    param_2->_IO_write_ptr = pcVar7 + 1;
    *pcVar7 = '\0';
  }
  else {
    __overflow(param_2,0);
  }
  goto LAB_0011f602;
}




// Function: clear_directory_table @ 0x1f6e0

void clear_directory_table(void)

{
  undefined8 *puVar1;
  undefined8 *__ptr;
  
  if (DAT_001823b0 != 0) {
    FUN_00143cf0();
  }
  __ptr = DAT_001823c0;
  if (DAT_001823a8 != 0) {
    FUN_00143cf0();
    __ptr = DAT_001823c0;
  }
  while (__ptr != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*__ptr;
    free((void *)__ptr[10]);
    free((void *)__ptr[0xb]);
    free(__ptr);
    __ptr = puVar1;
  }
  DAT_001823b8 = 0;
  DAT_001823c0 = (undefined8 *)0x0;
  return;
}




// Function: rebase_directory @ 0x1f770

void rebase_directory(long param_1)

{
  FUN_00125500(param_1 + 0x58);
  return;
}




// Function: update_parent_directory @ 0x1f780

void update_parent_directory(undefined8 *param_1)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  stat sStack_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_0011e240(*param_1);
  if (lVar2 != 0) {
    iVar1 = fstat(*(int *)(param_1 + 0x35),&sStack_b8);
    if (iVar1 == 0) {
      *(long *)(lVar2 + 0x10) = sStack_b8.st_mtim.tv_nsec;
      *(__time_t *)(lVar2 + 8) = sStack_b8.st_mtim.tv_sec;
    }
    else {
      FUN_00126460(*(undefined8 *)(lVar2 + 0x58));
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: scan_directory @ 0x1f810

long scan_directory(undefined8 *param_1)

{
  __dev_t _Var1;
  byte bVar2;
  char cVar3;
  int iVar4;
  char *__ptr;
  void *pvVar5;
  long lVar6;
  undefined8 uVar7;
  size_t sVar8;
  undefined1 *__ptr_00;
  undefined8 uVar9;
  size_t sVar10;
  undefined8 *puVar11;
  long lVar12;
  undefined8 *__ptr_01;
  int *piVar13;
  char *pcVar14;
  undefined4 uVar15;
  code *pcVar16;
  char *pcVar17;
  char *pcVar18;
  long in_FS_OFFSET;
  long local_238;
  undefined8 *local_230;
  long local_220;
  char *local_210;
  undefined1 local_1f9;
  undefined1 local_1f8 [88];
  stat local_1a0 [2];
  undefined8 *local_60;
  int local_50;
  long local_40;
  
  uVar7 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = (char *)FUN_00114860();
  _Var1 = param_1[0xb];
  lVar6 = param_1[0x33];
  if (__ptr == (char *)0x0) {
    FUN_001331a0(uVar7);
    FUN_00117440(param_1);
    pvVar5 = (void *)FUN_0014ee30(uVar7);
    FUN_001253a0(pvVar5);
    uVar9 = 0;
    if (lVar6 == 0) {
      uVar9 = 0x20;
    }
    lVar6 = FUN_0011ed80(pvVar5,param_1,uVar9,&local_1f9);
    free(pvVar5);
    uVar7 = FUN_001265f0(uVar7);
  }
  else {
    FUN_00117440(param_1);
    pvVar5 = (void *)FUN_0014ee30(uVar7);
    FUN_001253a0(pvVar5);
    uVar9 = 0;
    if (lVar6 == 0) {
      uVar9 = 0x20;
    }
    lVar6 = FUN_0011ed80(pvVar5,param_1,uVar9,&local_1f9);
    free(pvVar5);
    uVar7 = FUN_001265f0(uVar7);
    if (*(int *)(lVar6 + 0x38) == 0) {
      if (*(char **)(lVar6 + 0x48) != (char *)0x0) {
        sVar8 = strlen(*(char **)(lVar6 + 0x48));
        __ptr_00 = (undefined1 *)FUN_0014ecc0(sVar8 + 3);
        *__ptr_00 = 0x59;
        memcpy(__ptr_00 + 1,*(void **)(lVar6 + 0x48),sVar8 + 1);
        __ptr_00[sVar8 + 2] = 0;
        *(undefined8 *)(lVar6 + 0x30) = *(undefined8 *)(lVar6 + 0x28);
        uVar9 = FUN_0011e2e0(__ptr_00,0);
        *(undefined8 *)(lVar6 + 0x28) = uVar9;
        free(__ptr_00);
      }
    }
    else {
      local_220 = 0;
      if (*(int *)(lVar6 + 0x38) != 2) {
        lVar12 = *(long *)(lVar6 + 0x40);
        if (lVar12 == 0) {
          local_220 = *(long *)(lVar6 + 0x28);
        }
        else {
          local_220 = *(long *)(lVar12 + 0x30);
          if (local_220 == 0) {
            local_220 = *(long *)(lVar12 + 0x28);
          }
        }
      }
      pcVar18 = __ptr;
      if (*__ptr == '\0') {
        sVar8 = 0;
        puVar11 = (undefined8 *)FUN_0014edb0(0);
        local_238 = 1;
        local_230 = puVar11;
        if (*__ptr != '\0') goto LAB_0011f9f0;
        qsort(puVar11,0,8,FUN_0011e2b0);
        pcVar18 = (char *)FUN_0014ecc0(1);
        local_210 = pcVar18;
      }
      else {
        sVar8 = 0;
        local_238 = 0;
        pcVar14 = __ptr;
        do {
          sVar8 = sVar8 + 1;
          sVar10 = strlen(pcVar14);
          pcVar14 = pcVar14 + sVar10 + 1;
          local_238 = sVar10 + 2 + local_238;
        } while (*pcVar14 != '\0');
        local_238 = local_238 + 1;
        puVar11 = (undefined8 *)FUN_0014edb0(sVar8,8);
        cVar3 = *__ptr;
        local_230 = puVar11;
        while (cVar3 != '\0') {
LAB_0011f9f0:
          *puVar11 = pcVar18;
          puVar11 = puVar11 + 1;
          sVar10 = strlen(pcVar18);
          pcVar18 = pcVar18 + sVar10 + 1;
          cVar3 = *pcVar18;
        }
        sVar10 = 0;
        qsort(local_230,sVar8,8,FUN_0011e2b0);
        local_210 = (char *)FUN_0014ecc0(local_238);
        pcVar18 = local_210;
        if (sVar8 != 0) {
          do {
            pcVar14 = (char *)local_230[sVar10];
            lVar12 = FUN_0011ec90(local_220,pcVar14);
            if (lVar12 == 0) {
              bVar2 = -(*(long *)(lVar6 + 0x48) == 0) & 0x10;
            }
            else {
              bVar2 = -(*(long *)(lVar6 + 0x48) == 0) & 0xd7;
            }
            *pcVar18 = bVar2 + 0x49;
            pcVar17 = pcVar18 + 1;
            do {
              cVar3 = *pcVar14;
              pcVar14 = pcVar14 + 1;
              pcVar18 = pcVar17 + 1;
              *pcVar17 = cVar3;
              pcVar17 = pcVar18;
            } while (cVar3 != '\0');
            sVar10 = sVar10 + 1;
          } while (sVar8 != sVar10);
        }
      }
      *pcVar18 = '\0';
      *(undefined8 *)(lVar6 + 0x30) = *(undefined8 *)(lVar6 + 0x28);
      uVar9 = FUN_0011e2e0(local_210,0);
      *(undefined8 *)(lVar6 + 0x28) = uVar9;
      free(local_210);
      free(local_230);
      puVar11 = *(undefined8 **)(lVar6 + 0x28);
      __ptr_01 = (undefined8 *)FUN_0014ecc0(0x18);
      pcVar18 = (char *)*puVar11;
      *__ptr_01 = puVar11;
      *(undefined4 *)(__ptr_01 + 1) = 1;
      __ptr_01[2] = 0;
      if (*pcVar18 != '\0') {
        sVar8 = strlen(pcVar18);
        __ptr_01[2] = sVar8 + 1;
        do {
          while( true ) {
            pcVar14 = pcVar18 + 1;
            uVar9 = FUN_00126680(uVar7,pcVar14);
            if ((*pcVar18 != 'I') && (cVar3 = FUN_001176d0(uVar9,param_1), cVar3 == '\0')) break;
            *pcVar18 = 'N';
            pcVar18 = (char *)FUN_0011e120(__ptr_01);
            if (pcVar18 == (char *)0x0) goto LAB_0011fbe8;
          }
          iVar4 = *(int *)(param_1 + 0x35);
          FUN_0012fb40(local_1f8);
          if (iVar4 < 0) {
            piVar13 = __errno_location();
            pcVar16 = FUN_00126370;
            *piVar13 = -iVar4;
LAB_0011fbb6:
            FUN_00126490(uVar9,0,pcVar16);
            *pcVar18 = 'N';
          }
          else {
            iVar4 = fstatat(iVar4,pcVar14,local_1a0,DAT_00182800);
            if (iVar4 != 0) {
LAB_0011fcab:
              pcVar16 = FUN_00126460;
              goto LAB_0011fbb6;
            }
            if ((local_1a0[0].st_mode & 0xf000) == 0x4000) {
              iVar4 = FUN_00114950(param_1,pcVar14,DAT_00182808);
              if (iVar4 < 0) {
                pcVar16 = FUN_00126370;
                goto LAB_0011fbb6;
              }
              local_50 = iVar4;
              iVar4 = fstat(iVar4,local_1a0);
              if (iVar4 == 0) {
                if ((local_1a0[0].st_mode & 0xf000) != 0x4000) goto LAB_0011fcca;
                if (DAT_00182ad4 == 0) {
                  uVar15 = 0x10;
                }
                else {
                  uVar15 = 0;
                  if (*(int *)(lVar6 + 0x38) == 2) {
                    uVar15 = 0x12;
                  }
                }
                *pcVar18 = 'D';
                local_60 = param_1;
                FUN_0011ed80(uVar9,local_1f8,uVar15,pcVar18);
                FUN_00114a80(local_1f8);
                goto LAB_0011fbc5;
              }
              goto LAB_0011fcab;
            }
LAB_0011fcca:
            if ((DAT_00182ad1 == '\0') || (local_1a0[0].st_dev == _Var1)) {
              if (*pcVar18 != 'Y') {
                if (((int)(((uint)(DAT_00182b08 < local_1a0[0].st_mtim.tv_nsec) -
                           (uint)(local_1a0[0].st_mtim.tv_nsec < DAT_00182b08)) +
                          ((uint)(DAT_00182b00 < local_1a0[0].st_mtim.tv_sec) -
                          (uint)(local_1a0[0].st_mtim.tv_sec < DAT_00182b00)) * 2) < 0) &&
                   ((DAT_00182b8c == 0 ||
                    ((int)(((uint)(DAT_00182b08 < local_1a0[0].st_ctim.tv_nsec) -
                           (uint)(local_1a0[0].st_ctim.tv_nsec < DAT_00182b08)) +
                          ((uint)(DAT_00182b00 < local_1a0[0].st_ctim.tv_sec) -
                          (uint)(local_1a0[0].st_ctim.tv_sec < DAT_00182b00)) * 2) < 0))))
                goto LAB_0011fcea;
                *pcVar18 = 'Y';
              }
            }
            else {
LAB_0011fcea:
              *pcVar18 = 'N';
            }
          }
LAB_0011fbc5:
          FUN_0012fc00(local_1f8);
          pcVar18 = (char *)FUN_0011e120(__ptr_01);
        } while (pcVar18 != (char *)0x0);
      }
LAB_0011fbe8:
      free(__ptr_01);
    }
  }
  FUN_00126660(uVar7);
  free(__ptr);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: safe_directory_contents @ 0x1ff00

undefined * safe_directory_contents(long param_1)

{
  undefined *puVar1;
  
  puVar1 = &DAT_00164eec;
  if (((param_1 != 0) && (*(undefined8 **)(param_1 + 0x28) != (undefined8 *)0x0)) &&
     (puVar1 = (undefined *)**(undefined8 **)(param_1 + 0x28), puVar1 == (undefined *)0x0)) {
    puVar1 = &DAT_00164eec;
  }
  return puVar1;
}




// Function: append_incremental_renames @ 0x1ff30

void append_incremental_renames(long param_1)

{
  ulong __n;
  undefined8 *puVar1;
  void *__ptr;
  size_t sVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  char *pcVar5;
  char *__s;
  undefined8 *puVar6;
  ulong __n_00;
  undefined1 *puVar7;
  undefined8 *puVar8;
  long in_FS_OFFSET;
  undefined1 local_98 [8];
  long local_90;
  undefined1 *local_88;
  undefined1 *local_80;
  undefined1 *local_78;
  ulong local_68;
  byte local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001823c0 == (undefined8 *)0x0) goto LAB_00120132;
  _obstack_begin(local_98,0,0,FUN_0014ecc0,PTR_free_0017efc8);
  puVar7 = local_80;
  if (param_1 != 0) {
    if (*(undefined8 **)(param_1 + 0x28) == (undefined8 *)0x0) {
      __n_00 = 0;
    }
    else {
      pcVar5 = (char *)**(undefined8 **)(param_1 + 0x28);
      __n_00 = 0;
      if (pcVar5 != (char *)0x0) {
        __s = pcVar5;
        if (*pcVar5 != '\0') {
          do {
            sVar2 = strlen(__s);
            __s = __s + sVar2 + 1;
            __n_00 = __n_00 + sVar2 + 1;
          } while (*__s != '\0');
          if ((ulong)((long)local_78 - (long)puVar7) < __n_00) {
            _obstack_newchunk(local_98,__n_00);
            puVar7 = local_80;
          }
        }
        memcpy(puVar7,pcVar5,__n_00);
        local_80 = local_80 + __n_00;
      }
    }
    puVar6 = DAT_001823c0;
    if (DAT_001823c0 != (undefined8 *)0x0) goto LAB_00120018;
    goto LAB_00120084;
  }
  __n_00 = 0;
  puVar6 = DAT_001823c0;
  if (DAT_001823c0 != (undefined8 *)0x0) {
LAB_00120018:
    puVar3 = puVar6;
    if ((*(byte *)((long)puVar6 + 0x3c) & 0x10) != 0) {
LAB_00120028:
      puVar1 = (undefined8 *)puVar3[8];
      if (puVar1 != puVar6) goto code_r0x00120038;
      pcVar5 = (char *)FUN_00140230(puVar6[0xb]);
      if (local_78 == local_80) {
        _obstack_newchunk(local_98,1);
      }
      *local_80 = 0x58;
      local_80 = local_80 + 1;
      sVar2 = strlen(pcVar5);
      __n = sVar2 + 1;
      if ((ulong)((long)local_78 - (long)local_80) < __n) {
        _obstack_newchunk(local_98,__n);
      }
      memcpy(local_80,pcVar5,__n);
      local_80 = local_80 + __n;
      FUN_0011e5c0(local_98,puVar6[0xb],&DAT_0016a7d3);
      for (puVar1 = puVar6; puVar1 != puVar3; puVar1 = (undefined8 *)puVar1[8]) {
        FUN_0011e5c0(local_98,*(undefined8 *)(puVar1[8] + 0x58),puVar1[0xb]);
      }
      FUN_0011e5c0(local_98,&DAT_0016a7d3,puVar3[0xb]);
      free(pcVar5);
    }
    goto LAB_00120010;
  }
  goto LAB_00120128;
code_r0x00120038:
  puVar3 = puVar1;
  puVar8 = puVar6;
  if (puVar1 == (undefined8 *)0x0) goto LAB_00120061;
  goto LAB_00120028;
  while( true ) {
    FUN_0011e5c0(local_98,*(undefined8 *)(puVar8[8] + 0x58),puVar8[0xb]);
    puVar3 = puVar8 + 8;
    puVar8 = (undefined8 *)*puVar3;
    if ((undefined8 *)*puVar3 == (undefined8 *)0x0) break;
LAB_00120061:
    if (puVar8[8] == 0) {
      puVar6 = (undefined8 *)*puVar6;
      goto joined_r0x00120070;
    }
  }
LAB_00120010:
  puVar6 = (undefined8 *)*puVar6;
joined_r0x00120070:
  if (puVar6 == (undefined8 *)0x0) goto LAB_00120078;
  goto LAB_00120018;
LAB_00120078:
  if (param_1 != 0) {
LAB_00120084:
    if ((long)local_80 - (long)local_88 != __n_00) {
      if (local_80 == local_78) {
        _obstack_newchunk(local_98,1);
      }
      *local_80 = 0;
      __ptr = *(void **)(param_1 + 0x28);
      local_80 = local_80 + 1;
      free(*(void **)((long)__ptr + 0x18));
      free(__ptr);
      puVar7 = local_88;
      if (local_80 == local_88) {
        local_48 = local_48 | 2;
      }
      local_88 = local_78;
      if ((ulong)((long)((ulong)(local_80 + local_68) & ~local_68) - local_90) <=
          (ulong)((long)local_78 - local_90)) {
        local_88 = (undefined1 *)((ulong)(local_80 + local_68) & ~local_68);
      }
      local_80 = local_88;
      uVar4 = FUN_0011e2e0(puVar7,&DAT_001649db);
      *(undefined8 *)(param_1 + 0x28) = uVar4;
    }
  }
LAB_00120128:
  _obstack_free(local_98,0);
LAB_00120132:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: show_snapshot_field_ranges @ 0x202a0

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
  __printf_chk(1,"   (%-15s => [ %s, %s ]):\n\n","field name",&DAT_00164a0b,&DAT_00164a07);
  puVar4 = (undefined *)0x0;
  puVar5 = (undefined *)0x1;
  ppuVar3 = &PTR_DAT_0017d660;
  while( true ) {
    uVar1 = FUN_001255c0(puVar5,puVar4,puVar5,local_48);
    uVar2 = FUN_001255c0(ppuVar3[1],ppuVar3[1],ppuVar3[2],auStack_68);
    __printf_chk(1,"    %-15s => [ %s, %s ],\n",*ppuVar3,uVar2,uVar1);
    if (ppuVar3[3] == (undefined *)0x0) break;
    puVar5 = ppuVar3[5];
    puVar4 = ppuVar3[4];
    ppuVar3 = ppuVar3 + 3;
  }
  __printf_chk(1,&DAT_0016a7d2);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_directory_file @ 0x203b0

void read_directory_file(void)

{
  byte *pbVar1;
  long lVar2;
  long lVar3;
  ulong uVar4;
  char cVar5;
  int iVar6;
  uint uVar7;
  __ssize_t _Var8;
  undefined8 uVar9;
  uintmax_t uVar10;
  ulong uVar11;
  ulong uVar12;
  int *piVar13;
  __off_t _Var14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  char *pcVar18;
  char *pcVar19;
  long in_FS_OFFSET;
  bool bVar20;
  undefined1 auVar21 [16];
  long local_f8;
  char **local_f0;
  char *local_e8;
  size_t local_e0;
  long local_d8;
  char *local_d0;
  size_t local_c8;
  undefined8 local_c0;
  char *local_b8;
  ulong local_b0;
  ulong local_a8;
  ulong local_a0;
  ulong local_98;
  ulong local_88;
  byte local_68;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e8 = (char *)0x0;
  local_e0 = 0;
  iVar6 = open(DAT_00182b28,(-(uint)(DAT_00182b24 == 0) & 0x200) + 0x42,0x1b6);
  if (iVar6 < 0) {
    FUN_00132ea0(DAT_00182b28);
    goto LAB_001207b4;
  }
  DAT_001823a0 = fdopen(iVar6,"r+");
  if (DAT_001823a0 == (FILE *)0x0) {
    FUN_00132ea0(DAT_00182b28);
    close(iVar6);
    goto LAB_001207b4;
  }
  FUN_00129260();
  FUN_001292e0();
  _Var8 = __getdelim(&local_e8,&local_e0,10,DAT_001823a0);
  pcVar19 = local_e8;
  if (0 < _Var8) {
    iVar6 = strncmp(local_e8,"GNU tar",7);
    if (iVar6 == 0) {
      pcVar18 = pcVar19 + 8;
      if (pcVar19[7] != '-') {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar9 = dcgettext(0,"Bad incremental file format",5);
        error(1,0,uVar9);
        DAT_00182d58 = 2;
      }
      cVar5 = pcVar19[8];
      while (cVar5 != '-') {
        if (cVar5 == '\0') {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar9 = dcgettext(0,"Bad incremental file format",5);
          error(1,0,uVar9);
          DAT_00182d58 = 2;
        }
        pcVar19 = pcVar18 + 1;
        pcVar18 = pcVar18 + 1;
        cVar5 = *pcVar19;
      }
      uVar10 = strtoumax(pcVar18 + 1,(char **)0x0,10);
      if (1 < uVar10) {
        if (uVar10 == 2) {
          _obstack_begin(&local_b8,0,0,FUN_0014ecc0,PTR_free_0017efc8);
          FUN_0011ebb0(DAT_001823a0,&DAT_00182b00);
          uVar11 = local_a8;
          uVar4 = local_a0;
          while (local_a0 = uVar4, local_a8 = uVar11,
                cVar5 = FUN_0011e820(DAT_001823a0,&DAT_00164ada,0,1,&local_d8), cVar5 != '\0') {
            bVar20 = local_d8 != 0;
            FUN_0011ebb0(DAT_001823a0,&local_c8);
            cVar5 = FUN_0011e820(DAT_001823a0,&DAT_0016a193,0,0xffffffffffffffff,&local_d8);
            lVar2 = local_d8;
            if (((cVar5 == '\0') ||
                (cVar5 = FUN_0011e820(DAT_001823a0,&DAT_00164ade,0,0xffffffffffffffff,&local_d8),
                lVar3 = local_d8, cVar5 == '\0')) ||
               (iVar6 = FUN_0011e730(DAT_001823a0,&local_b8,&local_d0), uVar4 = local_a8, iVar6 != 0
               )) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar9 = dcgettext(0,"Unexpected EOF in snapshot file",5);
              uVar15 = FUN_0014ce40(DAT_00182b28);
              error(0,0,"%s: %s",uVar15,uVar9);
                    /* WARNING: Subroutine does not return */
              FUN_00116fd0();
            }
            if (local_a8 == local_a0) {
              local_68 = local_68 | 2;
            }
            uVar11 = local_a0 + local_88 & ~local_88;
            local_a8 = local_98;
            local_a0 = local_98;
            if (uVar11 - local_b0 <= local_98 - local_b0) {
              local_a8 = uVar11;
              local_a0 = uVar11;
            }
            do {
              iVar6 = FUN_0011e730(DAT_001823a0,&local_b8,&local_d0);
              if (iVar6 != 0) break;
            } while ((char *)0x1 < local_d0);
            pbVar1 = (byte *)DAT_001823a0->_IO_read_ptr;
            if (pbVar1 < DAT_001823a0->_IO_read_end) {
              DAT_001823a0->_IO_read_ptr = (char *)(pbVar1 + 1);
              uVar7 = (uint)*pbVar1;
            }
            else {
              uVar7 = __uflow(DAT_001823a0);
            }
            uVar11 = local_a8;
            if (uVar7 != 0) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar9 = dcgettext(0,"Missing record terminator",5);
              _Var14 = ftello(DAT_001823a0);
              uVar15 = FUN_001452d0(_Var14,local_58);
              uVar16 = FUN_0014ce40(DAT_00182b28);
              uVar17 = dcgettext(0,"%s: byte %s: %s",5);
              error(0,0,uVar17,uVar16,uVar15,uVar9);
                    /* WARNING: Subroutine does not return */
              FUN_00116fd0();
            }
            if (local_a8 == local_a0) {
              local_68 = local_68 | 2;
            }
            uVar12 = local_a0 + local_88 & ~local_88;
            local_a8 = local_98;
            if (uVar12 - local_b0 <= local_98 - local_b0) {
              local_a8 = uVar12;
            }
            local_a0 = local_a8;
            FUN_0011e440(uVar4,local_c8,local_c0,lVar2,lVar3,bVar20,0,uVar11);
            if ((uVar11 <= local_b0) || (uVar4 = uVar11, local_98 <= uVar11)) {
              _obstack_free(&local_b8,uVar11);
              uVar11 = local_a8;
              uVar4 = local_a0;
            }
          }
        }
        else {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar9 = dcgettext(0,"Unsupported incremental format version: %lu",5);
          error(1,0,uVar9,uVar10);
          DAT_00182d58 = 2;
        }
        goto LAB_0012079e;
      }
      local_d0 = (char *)0x0;
      local_c8 = 0;
      pcVar19 = local_e8;
      if (uVar10 != 1) goto LAB_00120b1c;
      _Var8 = __getdelim(&local_d0,&local_c8,10,DAT_001823a0);
      if (_Var8 < 1) {
        FUN_00132f00(DAT_00182b28);
        free(local_d0);
        goto LAB_0012079e;
      }
      auVar21 = FUN_00125930(local_d0,&local_b8,0);
      pcVar19 = local_b8;
      DAT_00182b08 = auVar21._8_8_;
      DAT_00182b00 = auVar21._0_8_;
      if (DAT_00182b08 < 0) {
        uVar9 = 2;
        goto LAB_00120dbf;
      }
      local_f8 = 2;
      bVar20 = true;
      if (*local_b8 != '\0') {
        piVar13 = __errno_location();
        *piVar13 = 0;
        uVar10 = strtoumax(pcVar19 + 1,&local_b8,10);
        auVar21._8_8_ = uVar10;
        auVar21._0_8_ = DAT_00182b00;
        if (*piVar13 == 0) {
          if (uVar10 < 1000000000) {
            if (pcVar19 + 1 != local_b8) {
              local_f8 = 2;
              goto LAB_0012092c;
            }
          }
          else {
            *piVar13 = 0x22;
          }
        }
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar9 = dcgettext(0,"Invalid time stamp",5);
        uVar15 = FUN_0014ce40(DAT_00182b28);
        error(0,*piVar13,"%s:%ld: %s",uVar15,2,uVar9);
        DAT_00182d58 = 2;
        auVar21._8_8_ = 0xffffffffffffffff;
        auVar21._0_8_ = 0x8000000000000000;
        local_f8 = 2;
      }
    }
    else {
LAB_00120b1c:
      local_c8 = 0;
      local_d0 = (char *)0x0;
      pcVar19 = strdup(pcVar19);
      local_d0 = pcVar19;
      local_c8 = strlen(pcVar19);
      local_c8 = local_c8 + 1;
      uVar9 = 1;
      auVar21 = FUN_00125930(pcVar19,&local_b8,0);
      if (auVar21._8_8_ < 0) {
LAB_00120dbf:
        DAT_00182b08 = auVar21._8_8_;
        DAT_00182b00 = auVar21._0_8_;
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
          auVar21._8_8_ = DAT_00182b08;
          auVar21._0_8_ = DAT_00182b00;
        }
        DAT_00182b08 = auVar21._8_8_;
        DAT_00182b00 = auVar21._0_8_;
        uVar15 = dcgettext(0,"Invalid time stamp",5);
        uVar16 = FUN_0014ce40(DAT_00182b28);
        piVar13 = __errno_location();
        error(0,*piVar13,"%s:%ld: %s",uVar16,uVar9,uVar15);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      bVar20 = false;
      local_f8 = 1;
    }
LAB_0012092c:
    while( true ) {
      DAT_00182b08 = auVar21._8_8_;
      DAT_00182b00 = auVar21._0_8_;
      local_f0 = &local_d0;
      _Var8 = __getdelim(local_f0,&local_c8,10,DAT_001823a0);
      if ((int)_Var8 < 1) break;
      cVar5 = *local_d0;
      local_f8 = local_f8 + 1;
      pcVar19 = local_d0 + (cVar5 == '+');
      if (local_d0[(long)(int)_Var8 + -1] == '\n') {
        local_d0[(long)(int)_Var8 + -1] = '\0';
      }
      piVar13 = __errno_location();
      uVar10 = 0;
      uVar9 = 0;
      if (bVar20) {
        auVar21 = FUN_00125930(pcVar19,&local_b8,0);
        pcVar19 = local_b8;
        uVar9 = auVar21._0_8_;
        if ((auVar21._8_8_ < 0) || (*local_b8 != ' ')) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          pcVar19 = "Invalid modification time";
          goto LAB_00120a14;
        }
        *piVar13 = 0;
        uVar10 = strtoumax(local_b8,&local_b8,10);
        if (uVar10 < 1000000000) {
          if (((*piVar13 == 0) && (pcVar19 != local_b8)) && (pcVar19 = local_b8, *local_b8 == ' '))
          goto LAB_00120898;
        }
        else if (*piVar13 == 0) {
          *piVar13 = 0x22;
        }
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar19 = "Invalid modification time (nanoseconds)";
LAB_00120a14:
        uVar9 = dcgettext(0,pcVar19,5);
        uVar15 = FUN_0014ce40(DAT_00182b28);
        error(0,*piVar13,"%s:%ld: %s",uVar15,local_f8,uVar9);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
LAB_00120898:
      uVar15 = FUN_001255e0(pcVar19,&local_b8,0,0xffffffffffffffff);
      if ((*piVar13 != 0) || (*local_b8 != ' ')) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar19 = "Invalid device number";
        goto LAB_00120a14;
      }
      uVar16 = FUN_001255e0(local_b8,&local_b8,0,0xffffffffffffffff);
      if ((*piVar13 != 0) || (*local_b8 != ' ')) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar19 = "Invalid inode number";
        goto LAB_00120a14;
      }
      pcVar19 = local_b8 + 1;
      FUN_001251b0(pcVar19);
      FUN_0011e440(pcVar19,uVar9,uVar10,uVar15,uVar16,cVar5 == '+',0,0);
      auVar21._8_8_ = DAT_00182b08;
      auVar21._0_8_ = DAT_00182b00;
    }
    free(local_d0);
  }
LAB_0012079e:
  if ((DAT_001823a0->_flags & 0x20) != 0) {
    FUN_00132f00(DAT_00182b28);
  }
  free(local_e8);
LAB_001207b4:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: write_directory_file @ 0x20e40

void write_directory_file(void)

{
  FILE *__stream;
  int iVar1;
  char *pcVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  __stream = DAT_001823a0;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001823a0 != (FILE *)0x0) {
    iVar1 = fseeko(DAT_001823a0,0,0);
    if (iVar1 != 0) {
      FUN_001331e0(DAT_00182b28);
    }
    iVar1 = fileno(__stream);
    iVar1 = FUN_0012c110(iVar1);
    if (iVar1 != 0) {
      FUN_00133450(DAT_00182b28);
    }
    __fprintf_chk(__stream,1,"%s-%s-%d\n","GNU tar",&DAT_00164b0c,2);
    pcVar2 = (char *)FUN_00145230(DAT_00182a20,auStack_38);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    pcVar2 = (char *)FUN_00145370(DAT_00182a28,auStack_38);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    if (((__stream->_flags & 0x20) != 0) ||
       ((DAT_001823b0 != 0 &&
        (FUN_00143b00(DAT_001823b0,FUN_0011f3f0,__stream), (__stream->_flags & 0x20) != 0)))) {
      FUN_001334f0(DAT_00182b28);
    }
    iVar1 = fclose(__stream);
    if (iVar1 != 0) {
      FUN_00132d60(DAT_00182b28);
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: is_dumpdir @ 0x20fc0

char is_dumpdir(long param_1)

{
  char cVar1;
  void *pvVar2;
  ulong __n;
  void *__src;
  undefined8 uVar3;
  ulong uVar4;
  void *__dest;
  
  cVar1 = *(char *)(param_1 + 0x188);
  if (cVar1 != '\0') {
    if (*(long *)(param_1 + 400) == 0) {
      uVar4 = *(ulong *)(param_1 + 0x88);
      pvVar2 = (void *)FUN_0014ecc0(uVar4);
      FUN_0010ccd0(DAT_00182408);
      FUN_0010c670(param_1);
      __dest = pvVar2;
      for (; uVar4 != 0; uVar4 = uVar4 - __n) {
        FUN_0010c710(uVar4);
        __src = (void *)FUN_0010d360();
        if (__src == (void *)0x0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar3 = dcgettext(0,"Unexpected EOF in archive",5);
          error(1,0,uVar3);
          DAT_00182d58 = 2;
        }
        __n = FUN_0010cd10(__src);
        if (uVar4 < __n) {
          __n = uVar4;
        }
        memcpy(__dest,__src,__n);
        __dest = (void *)((long)__dest + __n);
        FUN_0010ccd0((long)__src + (__n - 1));
      }
      FUN_0010c6a0();
      *(undefined1 *)(param_1 + 0x189) = 1;
      *(void **)(param_1 + 400) = pvVar2;
      cVar1 = *(char *)(param_1 + 0x188);
    }
    return cVar1;
  }
  return '\0';
}




// Function: purge_directory @ 0x21100

void purge_directory(undefined8 param_1)

{
  bool bVar1;
  char cVar2;
  int iVar3;
  char *__ptr;
  size_t sVar4;
  undefined8 uVar5;
  void *__ptr_00;
  undefined8 uVar6;
  void *__ptr_01;
  char *pcVar7;
  char *pcVar8;
  char *pcVar9;
  int *piVar10;
  undefined8 uVar11;
  char cVar12;
  char *pcVar13;
  long in_FS_OFFSET;
  long local_d8 [3];
  uint local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_00120fc0(&DAT_00182840);
  if ((cVar2 == '\0') ||
     (__ptr = (char *)FUN_00126970(param_1,0), pcVar8 = DAT_001829d0, __ptr == (char *)0x0)) {
LAB_0012113f:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00124660();
      return;
    }
  }
  else {
    cVar2 = *DAT_001829d0;
    if (cVar2 == '\0') {
LAB_001212b3:
      pcVar7 = (char *)0x0;
    }
    else {
      bVar1 = false;
      iVar3 = 0;
      pcVar7 = DAT_001829d0;
      cVar12 = cVar2;
      do {
        if (iVar3 == 0) {
          if (cVar12 == 'T') {
LAB_001217f2:
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            pcVar8 = "Malformed dumpdir: \'T\' not preceded by \'R\'";
LAB_001217ce:
            uVar5 = dcgettext(0,pcVar8,5);
            error(0,0,uVar5);
            DAT_00182d58 = 2;
            goto LAB_0012113f;
          }
LAB_00121204:
          if (cVar12 == 'X') {
            if (bVar1) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              pcVar8 = "Malformed dumpdir: \'X\' duplicated";
              goto LAB_001217ce;
            }
            bVar1 = true;
          }
          else if (cVar12 == 'R') {
            if (pcVar7[1] == '\0') {
              if (!bVar1) {
                if (DAT_00182d50 != (code *)0x0) {
                  (*DAT_00182d50)();
                }
                pcVar8 = "Malformed dumpdir: empty name in \'R\'";
                goto LAB_001217ce;
              }
              iVar3 = 0x54;
              bVar1 = false;
            }
            else {
              iVar3 = 0x54;
            }
          }
        }
        else {
          if (cVar12 != iVar3) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar5 = dcgettext(0,"Malformed dumpdir: expected \'%c\' but found %#3o",5);
            error(0,0,uVar5,iVar3,cVar12);
            DAT_00182d58 = 2;
            goto LAB_0012113f;
          }
          if (cVar12 != 'T') goto LAB_00121204;
          if (iVar3 != 0x54) goto LAB_001217f2;
          if ((pcVar7[1] == '\0') && (!bVar1)) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            pcVar8 = "Malformed dumpdir: empty name in \'T\'";
            goto LAB_001217ce;
          }
          iVar3 = 0;
        }
        sVar4 = strlen(pcVar7);
        pcVar7 = pcVar7 + sVar4 + 1;
        cVar12 = *pcVar7;
      } while (cVar12 != '\0');
      if (iVar3 != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar5 = dcgettext(0,"Malformed dumpdir: expected \'%c\' but found end of data",5);
        error(0,0,uVar5,iVar3);
        DAT_00182d58 = 2;
        goto LAB_0012113f;
      }
      if ((bVar1) && (((byte)DAT_00181b80 & 2) != 0)) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar5 = dcgettext(0,"Malformed dumpdir: \'X\' never used",5);
        error(0,0,uVar5);
        cVar2 = *DAT_001829d0;
        pcVar8 = DAT_001829d0;
        if (cVar2 == '\0') goto LAB_001212b3;
      }
      pcVar7 = (char *)0x0;
      do {
        pcVar13 = pcVar8;
        if (cVar2 == 'X') {
          sVar4 = strlen(pcVar8 + 1);
          pcVar7 = (char *)FUN_0014ece0(pcVar7,sVar4 + 0xc);
          memcpy(pcVar7,pcVar8 + 1,sVar4);
          builtin_strncpy(pcVar7 + sVar4,"/tar.XXXXXX",0xc);
          pcVar8 = mkdtemp(pcVar7);
          if (pcVar8 == (char *)0x0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar5 = FUN_0014d2d0(pcVar7);
            uVar6 = dcgettext(0,"Cannot create temporary directory using template %s",5);
            piVar10 = __errno_location();
            error(0,*piVar10,uVar6,uVar5);
            DAT_00182d58 = 2;
            free(pcVar7);
            free(__ptr);
            goto LAB_0012113f;
          }
        }
        else {
          sVar4 = strlen(pcVar8);
          if (cVar2 == 'R') {
            pcVar13 = pcVar8 + sVar4 + 1;
            pcVar9 = pcVar8 + 1;
            if (pcVar8[1] != '\0') {
              pcVar9 = (char *)FUN_00133640(pcVar9,0,DAT_00182b92);
            }
            if (pcVar13[1] != '\0') {
              FUN_00133640(pcVar13 + 1,0,DAT_00182b92);
            }
            pcVar8 = pcVar7;
            if (*pcVar9 != '\0') {
              pcVar8 = pcVar9;
            }
            cVar2 = FUN_0011a700(pcVar8);
            if (cVar2 == '\0') {
              free(pcVar7);
              free(__ptr);
              goto LAB_0012113f;
            }
          }
        }
        sVar4 = strlen(pcVar13);
        pcVar8 = pcVar13 + sVar4 + 1;
        cVar2 = *pcVar8;
      } while (cVar2 != '\0');
    }
    __ptr_01 = (void *)0x0;
    free(pcVar7);
    __ptr_00 = (void *)FUN_0011e2e0(DAT_001829d0,&DAT_001649db);
    if (*__ptr != '\0') {
      pcVar8 = __ptr;
      do {
        free(__ptr_01);
        __ptr_01 = (void *)FUN_00129320(param_1,pcVar8);
        iVar3 = FUN_00125fc0(__ptr_01,local_d8);
        if (iVar3 == 0) {
          pcVar7 = (char *)FUN_0011ec90(__ptr_00,pcVar8);
          if (pcVar7 == (char *)0x0) {
LAB_001213c0:
            if ((DAT_00182ad1 != '\0') && (local_d8[0] != DAT_001827f0)) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar5 = FUN_0014ce40(__ptr_01);
              pcVar7 = "%s: directory is on a different device: not purging";
              goto LAB_00121337;
            }
            if ((DAT_00182b40 == '\0') || (iVar3 = FUN_0012df50("delete",__ptr_01), iVar3 != 0)) {
              if (DAT_00182a54 != 0) {
                uVar6 = FUN_0014d2d0(__ptr_01);
                uVar5 = DAT_00182dd0;
                uVar11 = dcgettext(0,"%s: Deleting %s\n",5);
                __fprintf_chk(DAT_00181f28,1,uVar11,uVar5,uVar6);
              }
              iVar3 = FUN_00126a40(__ptr_01,1);
              if (iVar3 == 0) {
                piVar10 = __errno_location();
                iVar3 = *piVar10;
                if (DAT_00182d50 != (code *)0x0) {
                  (*DAT_00182d50)();
                }
                uVar5 = FUN_0014ce40(__ptr_01);
                uVar6 = dcgettext(0,"%s: Cannot remove",5);
                error(0,iVar3,uVar6,uVar5);
                DAT_00182d58 = 2;
              }
            }
          }
          else if (*pcVar7 == 'D') {
            if ((local_c0 & 0xf000) != 0x4000) goto LAB_001213c0;
          }
          else if ((*pcVar7 == 'Y') && ((local_c0 & 0xf000) == 0x4000)) goto LAB_001213c0;
        }
        else {
          piVar10 = __errno_location();
          if (*piVar10 == 2) goto LAB_0012134f;
          FUN_00126460(__ptr_01);
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar5 = FUN_0014ce40(__ptr_01);
          pcVar7 = "%s: Not purging directory: unable to stat";
LAB_00121337:
          uVar6 = dcgettext(0,pcVar7,5);
          error(0,0,uVar6,uVar5);
        }
LAB_0012134f:
        sVar4 = strlen(pcVar8);
        pcVar8 = pcVar8 + sVar4 + 1;
      } while (*pcVar8 != '\0');
    }
    free(__ptr_01);
    free(*(void **)((long)__ptr_00 + 0x18));
    free(__ptr_00);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      free(__ptr);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: list_dumpdir @ 0x21850

void list_dumpdir(byte *param_1,long param_2)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  bool bVar4;
  
  if (param_2 == 0) {
    return;
  }
  bVar4 = false;
  do {
    param_2 = param_2 + -1;
    bVar1 = *param_1;
    if (bVar1 == 0) {
      pcVar3 = DAT_00181f28->_IO_write_ptr;
      if (pcVar3 < DAT_00181f28->_IO_write_end) {
        DAT_00181f28->_IO_write_ptr = pcVar3 + 1;
        *pcVar3 = '\n';
      }
      else {
        __overflow(DAT_00181f28,10);
      }
      bVar4 = false;
    }
    else if (((byte)((int)(char)bVar1 - 0x44U) < 0x16) &&
            ((0x314401UL >> ((ulong)((int)(char)bVar1 - 0x44U) & 0x3f) & 1) != 0)) {
      __fprintf_chk(DAT_00181f28,1,"%c");
      if (!bVar4) {
        __fprintf_chk(DAT_00181f28,1,&DAT_0016aaba);
      }
      bVar4 = true;
    }
    else {
      pbVar2 = (byte *)DAT_00181f28->_IO_write_ptr;
      if (pbVar2 < DAT_00181f28->_IO_write_end) {
        DAT_00181f28->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar1;
      }
      else {
        __overflow(DAT_00181f28,(uint)bVar1);
      }
    }
    param_1 = param_1 + 1;
  } while (param_2 != 0);
  return;
}




// Function: decode_xform @ 0x21980

char * decode_xform(char *param_1,int *param_2)

{
  int iVar1;
  size_t sVar2;
  
  iVar1 = *param_2;
  if (iVar1 == 2) {
    param_1 = (char *)FUN_00133640(param_1,1,DAT_00182b92);
  }
  else {
    if (iVar1 == 4) {
      return param_1;
    }
    if (iVar1 == 1) {
      param_1 = (char *)FUN_00133640(param_1,0,DAT_00182b92);
    }
  }
  if (DAT_00182a90 == 0) {
    return param_1;
  }
  sVar2 = FUN_001293b0(param_1);
  if (sVar2 == 0xffffffffffffffff) {
    sVar2 = strlen(param_1);
  }
  return param_1 + sVar2;
}




// Function: from_header @ 0x21a20

ushort ** from_header(ushort **param_1,size_t param_2,long param_3,long param_4,ushort **param_5,
                      undefined4 param_6,undefined4 param_7)

{
  byte bVar1;
  uint uVar2;
  ushort **ppuVar3;
  ushort **ppuVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  long lVar8;
  ulong uVar9;
  ushort **ppuVar10;
  byte bVar11;
  ushort **ppuVar12;
  undefined1 *puVar13;
  undefined1 *puVar14;
  undefined1 *puVar15;
  char *pcVar17;
  ushort **ppuVar18;
  ushort **ppuVar19;
  void *__dest;
  int iVar20;
  ushort **ppuVar21;
  long in_FS_OFFSET;
  bool bVar22;
  bool bVar23;
  undefined1 auStack_498 [8];
  ushort **local_490;
  ushort **local_488;
  ushort **local_480;
  undefined4 local_474;
  ushort **local_470;
  undefined1 local_468 [33];
  undefined1 local_447 [31];
  undefined8 local_428;
  long local_40;
  undefined1 *puVar16;
  
  ppuVar12 = (ushort **)((long)param_1 + param_2);
  puVar16 = auStack_498;
  puVar15 = auStack_498;
  puVar13 = auStack_498;
  puVar14 = auStack_498;
  local_488 = (ushort **)CONCAT44(local_488._4_4_,param_6);
  local_474 = param_7;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  ppuVar10 = (ushort **)((long)param_1 + (ulong)(*(byte *)param_1 == 0));
  local_480 = param_5;
  if (ppuVar12 != ppuVar10) {
    local_470 = ppuVar10;
    ppuVar3 = __ctype_b_loc();
    ppuVar10 = local_470;
    do {
      bVar11 = *(byte *)ppuVar10;
      if ((*(byte *)((long)*ppuVar3 + (ulong)bVar11 * 2 + 1) & 0x20) == 0) {
        local_470 = (ushort **)-param_4;
        if ((int)(char)bVar11 - 0x30U < 8) {
          ppuVar18 = (ushort **)((long)ppuVar10 + 1);
          ppuVar19 = (ushort **)(long)(int)((int)(char)bVar11 - 0x30U);
          if (ppuVar12 != ppuVar18) {
            bVar23 = false;
            ppuVar21 = ppuVar18;
            goto LAB_00121b23;
          }
          bVar23 = false;
          ppuVar21 = ppuVar12;
          goto LAB_00121e48;
        }
        if ((char)local_488 == '\0') {
          if ((bVar11 - 0x2b & 0xfd) == 0) {
            if (((char)local_474 == '\0') && (DAT_00182450 == '\0')) {
              DAT_00182450 = '\x01';
              local_490 = ppuVar10;
              local_488 = ppuVar3;
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar5 = dcgettext(0,"Archive contains obsolescent base-64 headers",5);
              error(0,0,uVar5);
              bVar11 = *(byte *)local_490;
              ppuVar10 = local_490;
              ppuVar3 = local_488;
            }
            ppuVar10 = (ushort **)((long)ppuVar10 + 1);
            bVar23 = bVar11 == 0x2d;
            ppuVar19 = (ushort **)0x0;
            if (ppuVar12 != ppuVar10) goto LAB_00121d3f;
            ppuVar4 = (ushort **)0x0;
            if (bVar11 != 0x2d) goto LAB_001222d3;
            goto LAB_00121b97;
          }
          if ((bVar11 == 0x80) || (bVar11 == 0xff)) {
            ppuVar10 = (ushort **)((long)ppuVar10 + 1);
            uVar2 = bVar11 & 0x40;
            ppuVar19 = (ushort **)(long)(int)((bVar11 & 0x3f) - uVar2);
            goto LAB_00122082;
          }
        }
        puVar14 = auStack_498;
        ppuVar4 = (ushort **)0x0;
        if (bVar11 != 0) goto LAB_00121c23;
        goto LAB_00121b97;
      }
      ppuVar10 = (ushort **)((long)ppuVar10 + 1);
    } while (ppuVar12 != ppuVar10);
  }
  puVar13 = auStack_498;
  if ((param_3 != 0) && (puVar13 = auStack_498, (char)local_474 != '\x01')) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar17 = "Blanks in header where numeric %s value expected";
    goto LAB_00121be8;
  }
  goto LAB_00121b90;
LAB_00121d3f:
  bVar11 = *(byte *)ppuVar10;
  if ((char)(&DAT_00182460)[bVar11] < '@') {
    if ((ushort **)((ulong)ppuVar19 & 0x3ffffffffffffff) == ppuVar19) goto code_r0x00121d2f;
    puVar14 = auStack_498;
    while (puVar16 != auStack_498 + -(param_2 + 0x18 & 0xfffffffffffff000)) {
      puVar15 = puVar14 + -0x1000;
      *(undefined8 *)(puVar14 + -8) = *(undefined8 *)(puVar14 + -8);
      puVar16 = puVar14 + -0x1000;
      puVar14 = puVar14 + -0x1000;
    }
    uVar9 = (ulong)((uint)(param_2 + 0x18) & 0xff0);
    lVar8 = -uVar9;
    puVar14 = puVar15 + lVar8;
    if (uVar9 != 0) {
      *(undefined8 *)(puVar15 + -8) = *(undefined8 *)(puVar15 + -8);
    }
    __dest = (void *)((ulong)(puVar15 + lVar8 + 0xf) & 0xfffffffffffffff0);
    *(undefined8 *)(puVar15 + lVar8 + -8) = 0x1221a6;
    memcpy(__dest,param_1,param_2);
    *(undefined1 *)((long)__dest + param_2) = 0;
    puVar13 = puVar15 + lVar8;
    if (((char)local_474 != '\x01') && (puVar13 = puVar15 + lVar8, param_3 != 0)) {
      if (DAT_00182d50 != (code *)0x0) {
        *(undefined8 *)(puVar15 + lVar8 + -8) = 0x1221cf;
        (*DAT_00182d50)();
      }
      *(undefined8 *)(puVar15 + lVar8 + -8) = 0x1221d7;
      uVar5 = FUN_0014d2d0(__dest);
      *(undefined8 *)(puVar15 + lVar8 + -8) = 0x1221ed;
      uVar6 = dcgettext(0,"Archive signed base-64 string %s is out of %s range",5);
      *(undefined8 *)(puVar15 + lVar8 + -8) = 0x122201;
      error(0,0,uVar6,uVar5,param_3);
      ppuVar4 = (ushort **)0xffffffffffffffff;
      DAT_00182d58 = 2;
      goto LAB_00121b97;
    }
    goto LAB_00121b90;
  }
  if (((ppuVar10 != ppuVar12) && (bVar11 != 0)) &&
     ((*(byte *)((long)*ppuVar3 + (ulong)bVar11 * 2 + 1) & 0x20) == 0)) goto LAB_00121c23;
  goto LAB_00121d65;
code_r0x00121d2f:
  ppuVar10 = (ushort **)((long)ppuVar10 + 1);
  ppuVar19 = (ushort **)((long)ppuVar19 << 6 | (long)(char)(&DAT_00182460)[bVar11]);
  if (ppuVar12 == ppuVar10) goto LAB_00121d65;
  goto LAB_00121d3f;
LAB_00122082:
  bVar1 = *(byte *)ppuVar10;
  ppuVar10 = (ushort **)((long)ppuVar10 + 1);
  ppuVar19 = (ushort **)((long)ppuVar19 * 0x100 + (ulong)bVar1);
  if (ppuVar12 != ppuVar10) goto LAB_00122070;
  bVar23 = (bVar11 & 0x40) != 0;
  if ((bVar11 & 0x40) != 0) {
    ppuVar19 = (ushort **)-(long)ppuVar19;
  }
LAB_00121d65:
  if (!bVar23) goto LAB_00121b70;
  if (ppuVar19 <= local_470) goto LAB_00121fbb;
  puVar13 = auStack_498;
  if (((char)local_474 != '\x01') && (puVar13 = auStack_498, param_3 != 0)) {
    lVar7 = FUN_00145370(local_470,local_447);
    lVar8 = FUN_00145370(ppuVar19,(long)&local_428 + 1);
    *(undefined1 *)(lVar8 + -1) = 0x2d;
    lVar8 = lVar8 + -1;
    goto LAB_00121dbe;
  }
  goto LAB_00121b90;
LAB_00122070:
  if ((ushort **)((ulong)ppuVar19 & 0xffffffffffffff | (ulong)-uVar2 << 0x32) != ppuVar19)
  goto LAB_00122217;
  goto LAB_00122082;
LAB_00122217:
  puVar13 = auStack_498;
  if ((param_3 != 0) && (puVar13 = auStack_498, (char)local_474 != '\x01')) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar17 = "Archive base-256 value is out of %s range";
LAB_00121be8:
    uVar5 = dcgettext(0,pcVar17,5);
    error(0,0,uVar5,param_3);
    DAT_00182d58 = 2;
    puVar13 = auStack_498;
  }
  goto LAB_00121b90;
  while( true ) {
    bVar22 = (ushort **)((ulong)ppuVar19 & 0x1fffffffffffffff) != ppuVar19;
    ppuVar19 = (ushort **)((long)(int)uVar2 + (long)ppuVar19 * 8);
    ppuVar21 = (ushort **)((long)ppuVar21 + 1);
    bVar23 = (bool)(bVar23 | bVar22);
    if (ppuVar12 == ppuVar21) break;
LAB_00121b23:
    bVar1 = *(byte *)ppuVar21;
    uVar9 = (ulong)(uint)(int)(char)bVar1;
    uVar2 = (int)(char)bVar1 - 0x30;
    if (7 < uVar2) {
      if ((local_480 < ppuVar19) || (bVar23)) goto LAB_00121e5a;
      if ((ppuVar12 == ppuVar21) || (bVar1 == 0)) goto LAB_001222d3;
      goto LAB_00121b59;
    }
  }
LAB_00121e48:
  if ((ppuVar19 <= local_480) && (!bVar23)) {
LAB_001222d3:
    ppuVar4 = ppuVar19;
    puVar14 = auStack_498;
    goto LAB_00121b97;
  }
LAB_00121e5a:
  if ((char)bVar11 < '2') {
LAB_00121fc6:
    if (bVar23) {
LAB_00122261:
      puVar13 = auStack_498;
      if ((param_3 == 0) || (puVar13 = auStack_498, (char)local_474 == '\x01')) goto LAB_00121b90;
      local_470 = ppuVar10;
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcgettext(0,"Archive octal value %.*s is out of %s range",5);
      iVar20 = (int)ppuVar21 - (int)local_470;
      ppuVar12 = local_470;
      goto LAB_0012211d;
    }
LAB_00121fcf:
    if ((ppuVar12 != ppuVar21) && (uVar9 = (ulong)*(byte *)ppuVar21, *(byte *)ppuVar21 != 0)) {
LAB_00121b59:
      if ((*(byte *)((long)*ppuVar3 + (uVar9 & 0xff) * 2 + 1) & 0x20) == 0) {
LAB_00121c23:
        puVar13 = auStack_498;
        if (param_3 == 0) goto LAB_00121b90;
        goto LAB_00121c2c;
      }
    }
LAB_00121b70:
    if (ppuVar19 <= local_480) goto LAB_001222d3;
    puVar13 = auStack_498;
    if ((param_3 != 0) && (puVar13 = auStack_498, (char)local_474 != '\x01')) {
      lVar7 = FUN_00145370(local_470,local_447);
      lVar8 = FUN_00145370(ppuVar19,(long)&local_428 + 1);
LAB_00121dbe:
      if (local_470 != (ushort **)0x0) {
        *(undefined1 *)(lVar7 + -1) = 0x2d;
        lVar7 = lVar7 + -1;
      }
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = FUN_00145370(local_480,local_468);
      uVar6 = dcgettext(0,"Archive value %s is out of %s range %s..%s",5);
      error(0,0,uVar6,lVar8,param_3,lVar7,uVar5);
      ppuVar4 = (ushort **)0xffffffffffffffff;
      DAT_00182d58 = 2;
      puVar14 = auStack_498;
      goto LAB_00121b97;
    }
  }
  else {
    if (param_3 == 0) {
      if (bVar23) goto LAB_00121b90;
      goto LAB_00121fcf;
    }
    ppuVar4 = (ushort **)(long)(7 - (char)(bVar11 - 0x30 | 4));
    if (ppuVar12 == ppuVar18) {
      bVar11 = 0;
      ppuVar21 = ppuVar12;
    }
    else {
      bVar11 = 0;
      do {
        bVar1 = *(byte *)ppuVar18;
        ppuVar21 = ppuVar18;
        if (7 < (int)(char)bVar1 - 0x30U) break;
        ppuVar18 = (ushort **)((long)ppuVar18 + 1);
        bVar11 = bVar11 | (ushort **)((ulong)ppuVar4 & 0x1fffffffffffffff) != ppuVar4;
        ppuVar4 = (ushort **)((long)(int)(7 - ((int)(char)bVar1 - 0x30U)) + (long)ppuVar4 * 8);
        ppuVar21 = ppuVar12;
      } while (ppuVar12 != ppuVar18);
    }
    ppuVar19 = (ushort **)((long)ppuVar4 + 1);
    if ((bool)(ppuVar19 == (ushort **)0x0 | bVar11)) goto LAB_00122261;
    bVar23 = false;
    if (local_470 < ppuVar19) goto LAB_00121fc6;
    if ((char)local_474 == '\0') {
      local_488 = ppuVar10;
      local_480 = ppuVar4;
      local_470 = ppuVar3;
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcgettext(0,"Archive octal value %.*s is out of %s range; assuming two\'s complement",
                        5);
      error(0,0,uVar5,(int)ppuVar21 - (int)local_488,local_488,param_3);
      ppuVar4 = local_480;
      ppuVar3 = local_470;
    }
    if ((ppuVar12 == ppuVar21) || (*(byte *)ppuVar21 == 0)) {
      ppuVar4 = (ushort **)~(ulong)ppuVar4;
      puVar14 = auStack_498;
      goto LAB_00121b97;
    }
    if ((*(byte *)((long)*ppuVar3 + (ulong)*(byte *)ppuVar21 * 2 + 1) & 0x20) != 0) {
LAB_00121fbb:
      ppuVar4 = (ushort **)-(long)ppuVar19;
      puVar14 = auStack_498;
      goto LAB_00121b97;
    }
LAB_00121c2c:
    if (DAT_00182448 == 0) {
      DAT_00182448 = FUN_0014c700(0);
      FUN_0014c760(DAT_00182448,8);
    }
    for (; (param_1 != ppuVar12 && (*(byte *)((long)ppuVar12 + -1) == 0));
        ppuVar12 = (ushort **)((long)ppuVar12 + -1)) {
    }
    FUN_0014c820(&local_428,1000,param_1,(long)ppuVar12 - (long)param_1,DAT_00182448);
    puVar13 = auStack_498;
    if ((char)local_474 == '\0') {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcgettext(0,"Archive contains %.*s where numeric %s value expected",5);
      iVar20 = 1000;
      ppuVar12 = (ushort **)&local_428;
LAB_0012211d:
      error(0,0,uVar5,iVar20,ppuVar12,param_3);
      DAT_00182d58 = 2;
      puVar13 = auStack_498;
    }
  }
LAB_00121b90:
  ppuVar4 = (ushort **)0xffffffffffffffff;
  puVar14 = puVar13;
LAB_00121b97:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return ppuVar4;
  }
                    /* WARNING: Subroutine does not return */
  *(code **)(puVar14 + -8) = FUN_001222e0;
  __stack_chk_fail();
}




// Function: tar_checksum @ 0x22480

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
  iVar2 = FUN_00121a20(param_1 + 0x94,8,0,0,0x7fffffff,1);
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




// Function: read_header @ 0x22570

int read_header(undefined8 *param_1,long param_2,int param_3)

{
  char cVar1;
  undefined4 uVar2;
  undefined1 uVar3;
  int iVar4;
  int iVar5;
  undefined8 *puVar6;
  ulong uVar7;
  long lVar8;
  undefined8 *puVar9;
  void *__src;
  char *__s;
  size_t sVar10;
  long lVar11;
  ulong uVar12;
  undefined8 *puVar13;
  long in_FS_OFFSET;
  byte bVar14;
  undefined8 uVar15;
  undefined8 *local_190;
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
  
  bVar14 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_178 = 0;
  local_170 = 0;
  local_190 = (undefined8 *)0x0;
  local_180 = (undefined8 *)0x0;
LAB_001225e8:
  do {
    puVar6 = (undefined8 *)FUN_0010d360();
    *param_1 = puVar6;
    if (puVar6 == (undefined8 *)0x0) {
      iVar4 = 4;
      goto LAB_001226f8;
    }
    iVar4 = FUN_00122480(puVar6,0);
    if (iVar4 != 1) goto LAB_001226f8;
    if (*(char *)((long)puVar6 + 0x9c) == '1') {
      *(undefined8 *)(param_2 + 0x88) = 0;
LAB_00122638:
      free(DAT_001823f8);
      puVar9 = local_180 + 0x40;
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
            sVar10 = strlen(__s);
            __s[sVar10] = '/';
            __s = (char *)((long)&local_148 + sVar10 + 1);
          }
        }
        puVar9 = &local_148;
        local_170 = 0;
        uVar15 = puVar6[1];
        *(undefined8 *)__s = *puVar6;
        *(undefined8 *)(__s + 8) = uVar15;
        uVar15 = puVar6[3];
        *(undefined8 *)(__s + 0x10) = puVar6[2];
        *(undefined8 *)(__s + 0x18) = uVar15;
        uVar15 = puVar6[5];
        *(undefined8 *)(__s + 0x20) = puVar6[4];
        *(undefined8 *)(__s + 0x28) = uVar15;
        uVar15 = puVar6[7];
        *(undefined8 *)(__s + 0x30) = puVar6[6];
        *(undefined8 *)(__s + 0x38) = uVar15;
        uVar15 = puVar6[9];
        *(undefined8 *)(__s + 0x40) = puVar6[8];
        *(undefined8 *)(__s + 0x48) = uVar15;
        uVar15 = puVar6[0xb];
        *(undefined8 *)(__s + 0x50) = puVar6[10];
        *(undefined8 *)(__s + 0x58) = uVar15;
        uVar2 = *(undefined4 *)(puVar6 + 0xc);
        __s[100] = '\0';
        *(undefined4 *)(__s + 0x60) = uVar2;
      }
      DAT_001823f8 = local_180;
      DAT_001823e8 = local_170;
      FUN_00125120(param_2,puVar9);
      FUN_00125120(param_2 + 8,puVar9);
      uVar3 = FUN_001403c0(*(undefined8 *)(param_2 + 8));
      puVar9 = DAT_001823f0;
      *(undefined1 *)(param_2 + 0x10) = uVar3;
      free(puVar9);
      if (local_190 == (undefined8 *)0x0) {
        local_148 = *(undefined8 *)((long)puVar6 + 0x9d);
        uStack_140 = *(undefined8 *)((long)puVar6 + 0xa5);
        puVar9 = &local_148;
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
        puVar9 = local_190 + 0x40;
      }
      DAT_001823f0 = local_190;
      DAT_001823e0 = local_178;
      FUN_00125120(param_2 + 0x18,puVar9);
      local_190 = (undefined8 *)0x0;
      local_180 = (undefined8 *)0x0;
      goto LAB_001226f8;
    }
    lVar11 = (long)puVar6 + 0x7c;
    uVar15 = 0;
    uVar7 = FUN_00121a20(lVar11,0xc,"off_t",0,0x7fffffffffffffff,0);
    *(ulong *)(param_2 + 0x88) = uVar7;
    if ((long)uVar7 < 0) {
      iVar4 = 5;
      goto LAB_001226f8;
    }
    cVar1 = *(char *)((long)puVar6 + 0x9c);
    if (((byte)(cVar1 + 0xb5U) < 2) || (cVar1 == 'x')) {
      if (param_3 == 1) goto LAB_001228a8;
      if ((byte)(cVar1 + 0xb5U) < 2) {
        uVar12 = uVar7 + 0x200;
        if (((uVar7 & 0x1ff) != 0) &&
           (uVar12 = (uVar7 + 0x400) - (ulong)((uint)uVar7 & 0x1ff), uVar12 < uVar7)) {
                    /* WARNING: Subroutine does not return */
          FUN_00117020(uVar15);
        }
        puVar9 = (undefined8 *)FUN_0014ecc0(uVar12 + 1);
        if (*(char *)((long)puVar6 + 0x9c) == 'L') {
          free(local_180);
          local_180 = puVar9;
          local_170 = uVar12 >> 9;
        }
        else {
          free(local_190);
          local_190 = puVar9;
          local_178 = uVar12 >> 9;
        }
        FUN_0010ccd0(puVar6);
        puVar13 = puVar9 + 0x40;
        lVar11 = (long)puVar9 - (long)((ulong)(puVar9 + 1) & 0xfffffffffffffff8);
        *puVar9 = *puVar6;
        puVar9[0x3f] = puVar6[0x3f];
        puVar6 = (undefined8 *)((long)puVar6 - lVar11);
        puVar9 = (undefined8 *)((ulong)(puVar9 + 1) & 0xfffffffffffffff8);
        for (uVar7 = (ulong)((int)lVar11 + 0x200U >> 3); uVar7 != 0; uVar7 = uVar7 - 1) {
          *puVar9 = *puVar6;
          puVar6 = puVar6 + (ulong)bVar14 * -2 + 1;
          puVar9 = puVar9 + (ulong)bVar14 * -2 + 1;
        }
        for (uVar12 = uVar12 - 0x200; uVar12 != 0; uVar12 = uVar12 - uVar7) {
          __src = (void *)FUN_0010d360();
          if (__src == (void *)0x0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar15 = dcgettext(0,"Unexpected EOF in archive",5);
            error(0,0,uVar15);
            DAT_00182d58 = 2;
            break;
          }
          uVar7 = FUN_0010cd10(__src);
          if (uVar12 <= uVar7) {
            uVar7 = uVar12;
          }
          memcpy(puVar13,__src,uVar7);
          puVar13 = (undefined8 *)((long)puVar13 + uVar7);
          FUN_0010ccd0((long)__src + (uVar7 - 1));
        }
        *(undefined1 *)puVar13 = 0;
      }
      else {
LAB_001227ac:
        uVar15 = FUN_00121a20(lVar11,0xc,"off_t",0,0x7fffffffffffffff,0,0);
        FUN_0011db80(param_2 + 0x168,puVar6,uVar15);
      }
      goto LAB_001225e8;
    }
    if ((cVar1 != 'g') && (cVar1 != 'X')) goto LAB_00122638;
    if (param_3 == 1) goto LAB_001228a8;
    if (cVar1 == 'X') goto LAB_001227ac;
    if (DAT_00182560 == (undefined8 *)0x0) {
      DAT_00182560 = (undefined8 *)FUN_0014ecc0(0x200);
    }
    puVar13 = DAT_00182560;
    puVar9 = DAT_00182560 + 1;
    *DAT_00182560 = *puVar6;
    puVar13[0x3f] = puVar6[0x3f];
    lVar8 = (long)puVar13 - (long)((ulong)puVar9 & 0xfffffffffffffff8);
    puVar13 = (undefined8 *)((long)puVar6 - lVar8);
    puVar9 = (undefined8 *)((ulong)puVar9 & 0xfffffffffffffff8);
    for (uVar7 = (ulong)((int)lVar8 + 0x200U >> 3); uVar7 != 0; uVar7 = uVar7 - 1) {
      *puVar9 = *puVar13;
      puVar13 = puVar13 + (ulong)bVar14 * -2 + 1;
      puVar9 = puVar9 + (ulong)bVar14 * -2 + 1;
    }
    local_168 = (undefined1  [16])0x0;
    local_158 = (undefined1  [16])0x0;
    uVar15 = FUN_00121a20(lVar11,0xc,"off_t",0,0x7fffffffffffffff,0,0);
    FUN_0011db80(local_168,puVar6,uVar15);
    FUN_0011d980(local_168);
    FUN_0011de20(local_168);
    if (param_3 == 2) {
LAB_001228a8:
      iVar4 = 2;
LAB_001226f8:
      free(local_180);
      free(local_190);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return iVar4;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  } while( true );
}




// Function: decode_header @ 0x22c60

void decode_header(long param_1,long param_2,int *param_3,int param_4)

{
  undefined8 uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  ulong uVar6;
  undefined8 uVar7;
  long lVar8;
  int iVar9;
  
  uVar6 = FUN_00121a20(param_1 + 100,8,"mode_t",0x8000000000000000,0xffffffffffffffff,0,0);
  iVar3 = strcmp((char *)(param_1 + 0x101),"ustar");
  if (iVar3 == 0) {
    if ((((*(char *)(param_1 + 0x1db) != '\0') || (7 < (byte)(*(char *)(param_1 + 0x1dc) - 0x30U)))
        || (*(char *)(param_1 + 0x1e7) != ' ')) ||
       ((7 < (byte)(*(char *)(param_1 + 0x1e8) - 0x30U) ||
        (iVar9 = 5, *(char *)(param_1 + 499) != ' ')))) {
      iVar9 = 4 - (uint)(*(long *)(param_2 + 0x170) == 0);
    }
  }
  else {
    iVar9 = 1;
    iVar3 = strcmp((char *)(param_1 + 0x101),"ustar  ");
    if (iVar3 == 0) {
      iVar9 = (-(uint)((uVar6 & 0xfffffffffffff000) == 0) & 4) + 2;
    }
  }
  *param_3 = iVar9;
  *(uint *)(param_2 + 0x70) = (uint)uVar6 & 0xfff;
  uVar7 = FUN_00121a20(param_1 + 0x88,0xc,"time_t",0x8000000000000000,0x7fffffffffffffff,0,0);
  lVar8 = 0;
  cVar2 = *(char *)(param_1 + 0x109);
  *(undefined8 *)(param_2 + 0xf8) = uVar7;
  *(undefined8 *)(param_2 + 0x100) = 0;
  if (cVar2 != '\0') {
    lVar8 = param_1 + 0x109;
  }
  FUN_00125160(param_2 + 0x20,lVar8,0x20);
  lVar8 = 0;
  if (*(char *)(param_1 + 0x129) != '\0') {
    lVar8 = param_1 + 0x129;
  }
  FUN_00125160(param_2 + 0x28,lVar8,0x20);
  FUN_0011d530(param_2);
  uVar1 = DAT_00182a28;
  uVar7 = DAT_00182a20;
  if (iVar9 == 2) {
    if (DAT_00182b50 != '\0') {
      uVar6 = 0;
      uVar7 = FUN_00121a20(param_1 + 0x159,0xc,"time_t",0x8000000000000000,0x7fffffffffffffff,0,0);
      lVar8 = param_1 + 0x165;
      *(undefined8 *)(param_2 + 0xe8) = uVar7;
      goto LAB_00123142;
    }
    *(undefined8 *)(param_2 + 0x108) = DAT_00182a20;
    *(undefined8 *)(param_2 + 0x110) = uVar1;
    *(undefined8 *)(param_2 + 0xe8) = uVar7;
    *(undefined8 *)(param_2 + 0xf0) = uVar1;
  }
  else if (iVar9 == 5) {
    uVar6 = 0;
    uVar7 = FUN_00121a20(param_1 + 0x1dc,0xc,"time_t",0x8000000000000000,0x7fffffffffffffff,0,0);
    lVar8 = param_1 + 0x1e8;
    *(undefined8 *)(param_2 + 0xe8) = uVar7;
LAB_00123142:
    uVar7 = FUN_00121a20(lVar8,0xc,"time_t",0x8000000000000000,0x7fffffffffffffff,0,
                         uVar6 & 0xffffffff00000000);
    *(undefined8 *)(param_2 + 0x110) = 0;
    *(undefined8 *)(param_2 + 0x108) = uVar7;
    *(undefined8 *)(param_2 + 0xf0) = 0;
  }
  else {
    *(undefined8 *)(param_2 + 0x108) = DAT_00182a20;
    *(undefined8 *)(param_2 + 0x110) = uVar1;
    *(undefined8 *)(param_2 + 0xe8) = uVar7;
    *(undefined8 *)(param_2 + 0xf0) = uVar1;
    if (iVar9 == 1) {
      uVar6 = 0;
      uVar4 = FUN_00121a20(param_1 + 0x6c,8,"uid_t",0,0xffffffff,0,0);
      *(undefined4 *)(param_2 + 0x74) = uVar4;
      uVar4 = FUN_00121a20(param_1 + 0x74,8,"gid_t",0,0xffffffff,0,uVar6 & 0xffffffff00000000);
      *(undefined4 *)(param_2 + 0x78) = uVar4;
      uVar6 = 0;
      goto LAB_00122e6e;
    }
  }
  if (param_4 != 0) {
    if (((DAT_00182ad2 != '\0') || (*(char *)(param_1 + 0x109) == '\0')) ||
       (iVar3 = FUN_00127fd0(param_1 + 0x109,param_2 + 0x74), iVar3 == 0)) {
      uVar4 = FUN_00121a20(param_1 + 0x6c,8,"uid_t",0,0xffffffff,0,0);
      *(undefined4 *)(param_2 + 0x74) = uVar4;
    }
    if (((DAT_00182ad2 != '\0') || (*(char *)(param_1 + 0x129) == '\0')) ||
       (iVar3 = FUN_00128080(param_1 + 0x129,param_2 + 0x78), iVar3 == 0)) {
      uVar4 = FUN_00121a20(param_1 + 0x74,8,"gid_t",0,0xffffffff,0,0);
      *(undefined4 *)(param_2 + 0x78) = uVar4;
    }
  }
  uVar6 = 0;
  if ((int)*(char *)(param_1 + 0x9c) - 0x33U < 2) {
    uVar6 = FUN_00121a20(param_1 + 0x151,8,"minor_t",0xffffffff80000000,0x7fffffff,0,0);
    uVar5 = FUN_00121a20(param_1 + 0x149,8,"major_t",0xffffffff80000000,0x7fffffff,0);
    uVar6 = ((ulong)uVar5 & 0xfffff000) << 0x20 |
            (uVar6 & 0xffffff00) << 0xc | uVar6 & 0xff | (ulong)((uVar5 & 0xfff) << 8);
  }
LAB_00122e6e:
  *(ulong *)(param_2 + 0x80) = uVar6;
  FUN_0011d8a0(param_2);
  cVar2 = FUN_0012b3a0(param_2);
  uVar5 = DAT_00182400;
  if (cVar2 != '\0') {
    FUN_0012b400(param_2);
    *(undefined1 *)(param_2 + 0x120) = 1;
    return;
  }
  *(undefined1 *)(param_2 + 0x120) = 0;
  if ((((uVar5 & 0xfffffffb) == 2) && (*(char *)(DAT_00182408 + 0x9c) == 'D')) ||
     (*(long *)(param_2 + 400) != 0)) {
    *(undefined1 *)(param_2 + 0x188) = 1;
  }
  return;
}




// Function: tartime @ 0x23240

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
  if (DAT_00182b91 == '\0') {
    __tp = localtime(&local_28);
  }
  else {
    __tp = gmtime(&local_28);
  }
  if (__tp == (tm *)0x0) {
    if (param_1 < 0) {
      lVar5 = FUN_00145370(-local_28,&DAT_00182426);
      puVar1 = (undefined *)(lVar5 + -1);
      *(undefined1 *)(lVar5 + -1) = 0x2d;
    }
    else {
      puVar1 = (undefined *)FUN_00145370(local_28,&DAT_00182426);
    }
    puVar4 = puVar1;
    puVar3 = &DAT_00182441;
    if (param_3 == '\0') {
      puVar3 = &DAT_00182434;
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
      FUN_001256a0(param_2,&DAT_0018243a);
    }
  }
  else if (param_3 == '\0') {
    strftime(&DAT_00182420,0x25,"%Y-%m-%d %H:%M",__tp);
    puVar1 = &DAT_00182420;
  }
  else {
    puVar1 = &DAT_00182420;
    strftime(&DAT_00182420,0x25,"%Y-%m-%d %H:%M:%S",__tp);
    sVar2 = strlen(&DAT_00182420);
    FUN_001256a0(param_2,&DAT_00182420 + sVar2);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: simple_print_header @ 0x233e0

void simple_print_header(long *param_1,long param_2,long param_3)

{
  FILE *__stream;
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  size_t sVar5;
  char *__s;
  char *__s_00;
  size_t sVar6;
  int iVar7;
  long lVar8;
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
  if (DAT_001827e1 == '\0') {
    lVar8 = *param_1;
    if (lVar8 == 0) {
      lVar8 = param_1[1];
    }
  }
  else {
    lVar8 = param_1[1];
    if (lVar8 == 0) {
      lVar8 = *param_1;
    }
  }
  if (DAT_00182b7c != '\0') {
    if (param_3 < 0) {
      param_3 = FUN_0010cc60();
    }
    uVar2 = FUN_00145370((param_3 - DAT_001823e0) - DAT_001823e8,local_78);
    uVar3 = dcgettext(0,"block %s: ",5);
    __fprintf_chk(DAT_00181f28,1,uVar3,uVar2);
  }
  __stream = DAT_00181f28;
  if (DAT_00182a54 < 2) {
    pcVar4 = (char *)FUN_0014ca70(lVar8);
    fputs_unlocked(pcVar4,__stream);
    if ((DAT_001827e1 != '\0') && ((char)param_1[2] != '\0')) {
      pcVar4 = DAT_00181f28->_IO_write_ptr;
      if (pcVar4 < DAT_00181f28->_IO_write_end) {
        DAT_00181f28->_IO_write_ptr = pcVar4 + 1;
        *pcVar4 = '/';
      }
      else {
        __overflow(DAT_00181f28,0x2f);
      }
    }
switchD_001236af_caseD_0:
    pcVar4 = DAT_00181f28->_IO_write_ptr;
    if (pcVar4 < DAT_00181f28->_IO_write_end) {
      DAT_00181f28->_IO_write_ptr = pcVar4 + 1;
      *pcVar4 = '\n';
    }
    else {
      __overflow(DAT_00181f28,10);
    }
    goto LAB_001237cf;
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
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar2 = dcgettext(0,"Unexpected long name header",5);
    error(0,0,uVar2);
    DAT_00182d58 = 2;
    break;
  case 0x4d:
    local_e4 = 'M';
    break;
  case 0x56:
    DAT_00182451 = 1;
    local_e4 = 'V';
  }
  FUN_001329f0((int)param_1[0xe],local_e3);
  FUN_001326f0(param_1,local_da);
  pcVar4 = (char *)FUN_00123240(param_1[0x1f],param_1[0x20],DAT_00182b90);
  sVar5 = strlen(pcVar4);
  if (DAT_0017f228 < (int)sVar5) {
    DAT_0017f228 = (int)sVar5;
  }
  __s = (char *)param_1[4];
  if ((((__s == (char *)0x0) || (*__s == '\0')) || (DAT_00182400 == 1)) || (DAT_00182ad2 != '\0')) {
    __s = (char *)FUN_00145370(*(undefined4 *)((long)param_1 + 0x74),local_d8);
  }
  __s_00 = (char *)param_1[5];
  if (((__s_00 == (char *)0x0) || (*__s_00 == '\0')) ||
     ((DAT_00182400 == 1 || (DAT_00182ad2 != '\0')))) {
    __s_00 = (char *)FUN_00145370((int)param_1[0xf],local_b8);
  }
  if ((int)*(char *)(param_2 + 0x9c) - 0x33U < 2) {
    uVar2 = FUN_00145370((uint)((ulong)param_1[0x10] >> 0x20) & 0xfffff000 |
                         (uint)((ulong)param_1[0x10] >> 8) & 0xfff,local_98);
    __strcpy_chk(local_78,uVar2,0x2a);
    __strcat_chk(local_78,&DAT_0016379c,0x2a);
    uVar2 = FUN_00145370((uint)(((ulong)param_1[0x10] >> 0x14) << 8) | (uint)param_1[0x10] & 0xff,
                         local_98);
    __strcat_chk(local_78,uVar2,0x2a);
  }
  else {
    uVar2 = FUN_00145370(param_1[0x11],local_98);
    __strcpy_chk(local_78,uVar2,0x2a);
  }
  local_f0 = local_98;
  sVar5 = strlen(local_78);
  iVar7 = (int)sVar5;
  sVar5 = strlen(__s);
  sVar6 = strlen(__s_00);
  iVar1 = (int)sVar5 + 2 + (int)sVar6 + iVar7;
  if (iVar1 <= DAT_0017f22c) {
    iVar7 = (DAT_0017f22c - iVar1) + iVar7;
    iVar1 = DAT_0017f22c;
  }
  DAT_0017f22c = iVar1;
  __fprintf_chk(DAT_00181f28,1,"%s %s/%s %*s %-*s",&local_e4,__s,__s_00,iVar7,local_78,DAT_0017f228,
                pcVar4);
  uVar2 = FUN_0014ca70(lVar8);
  __fprintf_chk(DAT_00181f28,1," %s",uVar2);
  if ((DAT_001827e1 != '\0') && ((char)param_1[2] != '\0')) {
    pcVar4 = DAT_00181f28->_IO_write_ptr;
    if (pcVar4 < DAT_00181f28->_IO_write_end) {
      DAT_00181f28->_IO_write_ptr = pcVar4 + 1;
      *pcVar4 = '/';
    }
    else {
      __overflow(DAT_00181f28,0x2f);
    }
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
    goto switchD_001236af_caseD_0;
  default:
    local_e5 = 0;
    local_e6 = *(undefined1 *)(param_2 + 0x9c);
    uVar2 = FUN_0014d2d0(&local_e6);
    pcVar4 = " unknown file type %s\n";
    goto LAB_0012385a;
  case 0x31:
    uVar2 = FUN_0014ca70(param_1[3]);
    pcVar4 = " link to %s\n";
LAB_0012385a:
    uVar3 = dcgettext(0,pcVar4,5);
    __fprintf_chk(DAT_00181f28,1,uVar3,uVar2);
    goto LAB_001237cf;
  case 0x32:
    uVar2 = FUN_0014ca70(param_1[3]);
    __fprintf_chk(DAT_00181f28,1," -> %s\n",uVar2);
    goto LAB_001237cf;
  case 0x4b:
    pcVar4 = "--Long Link--\n";
    break;
  case 0x4c:
    pcVar4 = "--Long Name--\n";
    break;
  case 0x4d:
    uVar2 = FUN_00121a20(param_2 + 0x171,0xc,"uintmax_t",0,0xffffffffffffffff,0,0);
    uVar2 = FUN_00145370(uVar2,local_f0);
    __strcpy_chk(local_78,uVar2,0x2a);
    uVar2 = dcgettext(0,"--Continued at byte %s--\n",5);
    __fprintf_chk(DAT_00181f28,1,uVar2,local_78);
    goto LAB_001237cf;
  case 0x56:
    pcVar4 = "--Volume Header--\n";
  }
  uVar2 = dcgettext(0,pcVar4,5);
  __fprintf_chk(DAT_00181f28,1,uVar2);
LAB_001237cf:
  fflush_unlocked(DAT_00181f28);
  FUN_00132830(param_1);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_volume_label @ 0x23ae0

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
  if (DAT_00182560 != 0) {
    local_1a0 = *(undefined8 *)(DAT_00182560 + 0x88);
    local_198 = *(undefined4 *)(DAT_00182560 + 0x90);
  }
  FUN_0012fb40(local_3e8);
  FUN_00125120(local_3e0,".");
  FUN_00122c60(local_228,local_3e8,local_3ec,0);
  FUN_00125120(local_3e0,DAT_00181f18);
  FUN_001233e0(local_3e8,local_228,0);
  FUN_0012fc00(local_3e8);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_for_mkdir @ 0x23c30

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
  if (1 < DAT_00182a54) {
    local_43 = 100;
    FUN_001329f0(param_3,local_42);
    if (DAT_00182b7c != '\0') {
      uVar1 = FUN_0010cc60();
      uVar1 = FUN_00145370(uVar1,local_38);
      uVar2 = dcgettext(0,"block %s: ",5);
      __fprintf_chk(DAT_00181f28,1,uVar2,uVar1);
    }
    uVar1 = FUN_0014ca70(param_1);
    uVar2 = dcgettext(0,"Creating directory:",5);
    __fprintf_chk(DAT_00181f28,1,"%s %*s %s\n",&local_43,DAT_0017f22c + 1 + DAT_0017f228,uVar2,uVar1
                 );
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: skip_file @ 0x23d40

void skip_file(long param_1)

{
  ulong uVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  
  if (DAT_001827f8 != '\0') {
    lVar3 = FUN_0010d780();
    if (lVar3 < 0) {
      DAT_001827f8 = '\0';
    }
    else {
      param_1 = param_1 + lVar3 * -0x200;
    }
  }
  FUN_0010c710(param_1);
  if (0 < param_1) {
    uVar1 = param_1 - 1;
    lVar3 = param_1 + -0x200;
    do {
      lVar2 = FUN_0010d360();
      if (lVar2 == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar4 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      FUN_0010ccd0();
      param_1 = param_1 + -0x200;
      FUN_0010c710(param_1);
    } while (param_1 != lVar3 - (uVar1 & 0xfffffffffffffe00));
  }
  return;
}




// Function: read_and @ 0x23e80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void read_and(code *param_1)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  undefined8 uVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined1 local_58 [24];
  long local_40;
  
  lVar7 = 0x41;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  _DAT_00182460 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182468 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182470 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182478 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182480 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182488 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182490 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182498 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824a0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824a8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824b0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824b8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824c0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824c8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824d0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824d8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824e0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824e8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824f0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824f8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182500 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182508 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182510 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182518 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182520 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182528 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182530 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182538 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182540 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182548 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182550 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182558 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  lVar5 = 0;
  while( true ) {
    (&DAT_00182460)[lVar7] = (char)lVar5;
    if (lVar5 + 1 == 0x40) break;
    lVar7 = (long)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"
                  [lVar5 + 1];
    lVar5 = lVar5 + 1;
  }
  FUN_001282f0();
  iVar3 = 0;
  FUN_0010f710(0);
LAB_00123f7c:
  FUN_0012fc00(&DAT_00182840);
  uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
  switch(uVar2) {
  case 0:
  case 2:
switchD_00123f9f_caseD_0:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
switchD_00123f9f_caseD_1:
switchD_0012410b_caseD_1:
    FUN_00122c60(DAT_00182408,&DAT_00182840,&DAT_00182400,1);
    cVar1 = FUN_00128880(DAT_00182848);
    if (cVar1 != '\0') {
      if (-1 < DAT_00182b08) {
        DAT_00182938 = FUN_00121a20(DAT_00182408 + 0x88,0xc,"time_t",0x8000000000000000,
                                    0x7fffffffffffffff,0,0);
        DAT_00182940 = 0;
        if ((int)((-(uint)(0 < DAT_00182b08) - (int)(DAT_00182b08 >> 0x3f)) +
                 ((uint)(DAT_00182b00 < DAT_00182938) - (uint)(DAT_00182938 < DAT_00182b00)) * 2) <
            0) goto LAB_001240a0;
      }
      cVar1 = FUN_001176d0(DAT_00182848,DAT_001829d8);
      if (cVar1 != '\0') goto LAB_001240a0;
      iVar3 = (int)*(char *)(DAT_00182408 + 0x9c);
      if (iVar3 == 0x56) goto LAB_001242f7;
LAB_00124503:
      FUN_001222e0(iVar3,&DAT_00182840);
LAB_001242f7:
      (*param_1)();
      cVar1 = FUN_001289e0(&DAT_00182840);
      if (cVar1 == '\0') goto code_r0x0012430d;
      break;
    }
LAB_001240a0:
    cVar1 = *(char *)(DAT_00182408 + 0x9c);
    if (cVar1 == 'M') {
      iVar3 = 0x4d;
      goto LAB_00124503;
    }
    if (cVar1 == 'V') goto LAB_001242f7;
    if ((cVar1 == '5') && (DAT_00182a89 != '\0')) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar4 = FUN_0014ce40(DAT_00182848);
      uVar6 = dcgettext(0,"%s: Omitting",5);
      error(0,0,uVar6,uVar4);
    }
    if (DAT_001829c9 == '\0') {
      FUN_00123e00();
      cVar1 = FUN_001289e0(&DAT_00182840);
      if (cVar1 != '\0') break;
      FUN_0012fc00(&DAT_00182840);
      uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
      switch(uVar2) {
      case 0:
      case 2:
        goto switchD_00123f9f_caseD_0;
      case 1:
        goto switchD_0012410b_caseD_1;
      case 3:
        goto switchD_0012410b_caseD_3;
      case 4:
        goto switchD_00123f9f_caseD_4;
      case 5:
        goto switchD_0012410b_caseD_5;
      default:
        goto switchD_00123f9f_default;
      }
    }
    cVar1 = FUN_001289e0(&DAT_00182840);
    if (cVar1 != '\0') break;
    FUN_0012fc00(&DAT_00182840);
    uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
    switch(uVar2) {
    case 0:
    case 2:
      goto switchD_00123f9f_caseD_0;
    case 1:
      goto switchD_0012410b_caseD_1;
    case 3:
      goto switchD_0012410b_caseD_3;
    case 4:
      goto switchD_00123f9f_caseD_4;
    case 5:
      goto switchD_0012410b_caseD_5;
    default:
      break;
    }
  case 3:
    goto switchD_00123f9f_caseD_3;
  case 4:
switchD_00123f9f_caseD_4:
    if (DAT_00182b7c != '\0') {
      uVar4 = FUN_0010cc60();
      uVar4 = FUN_00145370(uVar4,local_58);
      uVar6 = dcgettext(0,"block %s: ** End of File **\n",5);
      __fprintf_chk(DAT_00181f28,1,uVar6,uVar4);
    }
    break;
  case 5:
    FUN_0010ccd0(DAT_00182408);
    if (iVar3 != 0) {
      if (iVar3 == 1) goto LAB_0012415c;
      goto LAB_001241f4;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar4 = dcgettext(0,"This does not look like a tar archive",5);
    error(0,0,uVar4);
    DAT_00182d58 = 2;
    goto LAB_0012415c;
  }
  goto switchD_00123f9f_default;
code_r0x0012430d:
  FUN_0012fc00(&DAT_00182840);
  uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
  switch(uVar2) {
  case 0:
  case 2:
    goto switchD_00123f9f_caseD_0;
  case 1:
    goto switchD_0012410b_caseD_1;
  case 3:
switchD_0012410b_caseD_3:
    iVar3 = 1;
    goto switchD_00123f9f_caseD_3;
  case 4:
    goto switchD_00123f9f_caseD_4;
  case 5:
    break;
  default:
    goto switchD_00123f9f_default;
  }
switchD_0012410b_caseD_5:
  FUN_0010ccd0(DAT_00182408);
LAB_0012415c:
  if (DAT_00182b7c != '\0') {
    lVar5 = FUN_0010cc60();
    uVar4 = FUN_00145370((lVar5 - DAT_001823e0) - DAT_001823e8,local_58);
    uVar6 = dcgettext(0,"block %s: ",5);
    __fprintf_chk(DAT_00181f28,1,uVar6,uVar4);
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar4 = dcgettext(0,"Skipping to next header",5);
  error(0,0,uVar4);
  DAT_00182d58 = 2;
  cVar1 = FUN_001289e0(&DAT_00182840);
  if (cVar1 != '\0') goto switchD_00123f9f_default;
  FUN_0012fc00(&DAT_00182840);
  uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
  switch(uVar2) {
  case 0:
  case 2:
    goto switchD_00123f9f_caseD_0;
  case 1:
    goto switchD_00123f9f_caseD_1;
  case 3:
    break;
  case 4:
    goto switchD_00123f9f_caseD_4;
  case 5:
    FUN_0010ccd0(DAT_00182408);
LAB_001241f4:
    do {
      cVar1 = FUN_001289e0(&DAT_00182840);
      if (cVar1 != '\0') goto switchD_00123f9f_default;
      FUN_0012fc00(&DAT_00182840);
      uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
      switch(uVar2) {
      case 0:
      case 2:
        goto switchD_00123f9f_caseD_0;
      case 1:
        goto switchD_00123f9f_caseD_1;
      case 3:
        goto switchD_001241e0_caseD_3;
      case 4:
        goto switchD_00123f9f_caseD_4;
      case 5:
        FUN_0010ccd0(DAT_00182408);
        cVar1 = FUN_001289e0(&DAT_00182840);
        if (cVar1 != '\0') goto switchD_00123f9f_default;
        FUN_0012fc00(&DAT_00182840);
        uVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
        switch(uVar2) {
        case 0:
        case 2:
          goto switchD_00123f9f_caseD_0;
        case 1:
          goto switchD_00123f9f_caseD_1;
        case 3:
          goto switchD_001241e0_caseD_3;
        case 4:
          goto switchD_00123f9f_caseD_4;
        case 5:
          FUN_0010ccd0(DAT_00182408);
          break;
        default:
          goto switchD_00123f9f_default;
        }
        break;
      default:
        goto switchD_00123f9f_default;
      }
    } while( true );
  default:
    goto switchD_00123f9f_default;
  }
switchD_001241e0_caseD_3:
  iVar3 = 5;
switchD_00123f9f_caseD_3:
  if (DAT_00182b7c != '\0') {
    uVar4 = FUN_0010cc60();
    uVar4 = FUN_00145370(uVar4,local_58);
    uVar6 = dcgettext(0,"block %s: ** Block of NULs **\n",5);
    __fprintf_chk(DAT_00181f28,1,uVar6,uVar4);
  }
  FUN_0010ccd0(DAT_00182408);
  if (DAT_00182b51 == '\0') {
    iVar3 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
    if ((iVar3 != 3) && (((byte)DAT_00181b80 & 1) != 0)) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar4 = FUN_0010cc60();
      uVar4 = FUN_00145370(uVar4,local_58);
      uVar6 = dcgettext(0,"A lone zero block at %s",5);
      error(0,0,uVar6,uVar4);
    }
    goto switchD_00123f9f_default;
  }
  cVar1 = FUN_001289e0(&DAT_00182840);
  if (cVar1 != '\0') {
switchD_00123f9f_default:
    FUN_0010d8d0();
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    FUN_00128a70();
    return;
  }
  goto LAB_00123f7c;
}




// Function: list_archive @ 0x24580

void list_archive(void)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  uVar2 = FUN_0010cc60();
  uVar3 = DAT_00182408;
  if (DAT_00182a54 != 0) {
    if (((DAT_00182400 == 4) && (DAT_00182451 == '\0')) && (DAT_00181f18 != 0)) {
      FUN_00123ae0();
      DAT_00182451 = '\x01';
    }
    FUN_001233e0(&DAT_00182840,uVar3,uVar2);
    if ((DAT_00182b50 != '\0') && (2 < DAT_00182a54)) {
      cVar1 = FUN_00120fc0(&DAT_00182840);
      if (cVar1 != '\0') {
        uVar3 = FUN_0011f690(DAT_001829d0);
        FUN_00121850(DAT_001829d0,uVar3);
      }
    }
  }
  if (DAT_001829c9 != '\0') {
    return;
  }
  FUN_00123e00();
  return;
}




// Function: test_archive_label @ 0x24680

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void test_archive_label(void)

{
  char cVar1;
  int iVar2;
  long lVar3;
  void *__ptr;
  long lVar4;
  
  lVar4 = 0x41;
  _DAT_00182460 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182468 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182470 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182478 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182480 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182488 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182490 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182498 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824a0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824a8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824b0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824b8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824c0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824c8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824d0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824d8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824e0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824e8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_001824f0 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam00000000001824f8 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182500 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182508 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182510 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182518 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182520 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182528 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182530 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182538 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182540 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182548 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  _DAT_00182550 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._64_8_;
  uRam0000000000182558 = s_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef_00165520._72_8_;
  lVar3 = 0;
  while( true ) {
    (&DAT_00182460)[lVar4] = (char)lVar3;
    if (lVar3 + 1 == 0x40) break;
    lVar4 = (long)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s"
                  [lVar3 + 1];
    lVar3 = lVar3 + 1;
  }
  FUN_001282f0();
  FUN_0010f710(0);
  iVar2 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
  if (iVar2 == 1) {
    FUN_00122c60(DAT_00182408,&DAT_00182840,&DAT_00182400,0);
    if (*(char *)(DAT_00182408 + 0x9c) == 'V') {
      FUN_00125160(&DAT_00181f18,DAT_00182408,100);
    }
    if (DAT_00181f18 != 0) {
      if (DAT_00182a54 != 0) {
        FUN_00123ae0();
      }
      cVar1 = FUN_00128880(DAT_00181f18);
      if ((cVar1 == '\0') && (DAT_00182b10 != '\0')) {
        __ptr = (void *)FUN_0010f3f0(DAT_00181f18);
        FUN_00128880(__ptr);
        free(__ptr);
      }
    }
  }
  FUN_0010d8d0();
  FUN_00128c00();
  return;
}




// Function: map_hash @ 0x247f0

ulong map_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: map_compare @ 0x24800

undefined8 map_compare(long *param_1,long *param_2)

{
  return CONCAT71((int7)((ulong)*param_2 >> 8),*param_1 == *param_2);
}




// Function: name_to_uid @ 0x24810

ulong name_to_uid(char *param_1)

{
  passwd *ppVar1;
  
  ppVar1 = getpwnam(param_1);
  if (ppVar1 != (passwd *)0x0) {
    return (ulong)ppVar1->pw_uid;
  }
  return 0xffffffffffffffff;
}




// Function: name_to_gid @ 0x24840

ulong name_to_gid(char *param_1)

{
  group *pgVar1;
  
  pgVar1 = getgrnam(param_1);
  if (pgVar1 != (group *)0x0) {
    return (ulong)pgVar1->gr_gid;
  }
  return 0xffffffffffffffff;
}




// Function: owner_map_read @ 0x24de0

void owner_map_read(undefined8 param_1)

{
  FUN_00124950(&DAT_00182570,param_1,FUN_00124810,"UID");
  return;
}




// Function: owner_map_translate @ 0x24e10

bool owner_map_translate(ulong param_1,int *param_2,long *param_3)

{
  long lVar1;
  long lVar2;
  long in_FS_OFFSET;
  bool bVar3;
  ulong local_38 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182570 != 0) {
    local_38[0] = param_1 & 0xffffffff;
    lVar2 = FUN_00143960(DAT_00182570,local_38);
    if (lVar2 != 0) {
      lVar1 = *(long *)(lVar2 + 0x10);
      *param_2 = (int)*(undefined8 *)(lVar2 + 8);
      *param_3 = lVar1;
      bVar3 = false;
      goto LAB_00124e62;
    }
  }
  bVar3 = DAT_00182abc == -1;
  if (!bVar3) {
    *param_2 = DAT_00182abc;
  }
  if (DAT_00182ac0 != 0) {
    *param_3 = DAT_00182ac0;
    bVar3 = false;
  }
LAB_00124e62:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: group_map_read @ 0x24eb0

void group_map_read(undefined8 param_1)

{
  FUN_00124950(&DAT_00182568,param_1,FUN_00124840,"GID");
  return;
}




// Function: group_map_translate @ 0x24ee0

bool group_map_translate(ulong param_1,int *param_2,long *param_3)

{
  long lVar1;
  long lVar2;
  long in_FS_OFFSET;
  bool bVar3;
  ulong local_38 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182568 != 0) {
    local_38[0] = param_1 & 0xffffffff;
    lVar2 = FUN_00143960(DAT_00182568,local_38);
    if (lVar2 != 0) {
      lVar1 = *(long *)(lVar2 + 0x10);
      *param_2 = (int)*(undefined8 *)(lVar2 + 8);
      *param_3 = lVar1;
      bVar3 = false;
      goto LAB_00124f32;
    }
  }
  bVar3 = DAT_00182b54 == -1;
  if (!bVar3) {
    *param_2 = DAT_00182b54;
  }
  if (DAT_00182b58 != 0) {
    *param_3 = DAT_00182b58;
    bVar3 = false;
  }
LAB_00124f32:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: quote_n_colon @ 0x250f0

void quote_n_colon(undefined4 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_0014c740(0);
  FUN_0014cf60(param_1,uVar1,param_2);
  return;
}




// Function: assign_string_n @ 0x25160

void assign_string_n(undefined8 *param_1,char *param_2,size_t param_3)

{
  size_t __n;
  void *pvVar1;
  
  free((void *)*param_1);
  if (param_2 != (char *)0x0) {
    __n = strnlen(param_2,param_3);
    pvVar1 = (void *)FUN_0014ecc0(__n + 1);
    pvVar1 = memcpy(pvVar1,param_2,__n);
    *(undefined1 *)((long)pvVar1 + __n) = 0;
    *param_1 = pvVar1;
  }
  return;
}




// Function: unquote_string @ 0x251b0

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
          goto LAB_00125230;
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
LAB_001251e9:
      param_1 = pcVar1;
      pcVar3 = pcVar4;
      if (cVar2 == '\0') goto LAB_00125230;
    }
    if (param_1 != pcVar3) {
      *param_1 = cVar2;
      cVar2 = pcVar3[1];
      goto LAB_001251e9;
    }
    cVar2 = param_1[1];
    param_1 = pcVar1;
    pcVar3 = pcVar4;
  } while (cVar2 != '\0');
LAB_00125230:
  if (pcVar4 != pcVar1) {
    *pcVar1 = '\0';
  }
  return uVar5;
}




// Function: zap_slashes @ 0x253a0

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




// Function: normalize_filename_x @ 0x253e0

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
      if (cVar3 == '\0') goto LAB_001254c0;
    }
    do {
      pcVar1 = pcVar1 + (ulong)(cVar3 == '.') + 1;
      cVar3 = *pcVar1;
    } while (pcVar1[cVar3 == '.'] == '/');
    *pcVar5 = cVar3;
  }
LAB_001254c0:
  if ((long)pcVar2 - (long)param_1 < 3) {
    return;
  }
  if (pcVar2[-2] == '.') {
    if ((pcVar2[-3] != '/') || (pcVar2 = pcVar2 + -1, (long)pcVar2 - (long)param_1 < 3))
    goto LAB_001254e0;
  }
  else if (pcVar2[-2] != '/') goto LAB_001254e0;
  pcVar2 = pcVar2 + -1;
LAB_001254e0:
  pcVar2[-1] = '\0';
  return;
}




// Function: replace_prefix @ 0x25500

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
        __s = (char *)FUN_0014ece0(__s,(sVar2 - param_3) + 1 + param_5);
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




// Function: sysinttostr @ 0x255c0

void sysinttostr(ulong param_1,undefined8 param_2,ulong param_3,undefined8 param_4)

{
  if (param_1 <= param_3) {
    FUN_00145370();
    return;
  }
  FUN_00145230(param_1,param_4);
  return;
}




// Function: strtosysint @ 0x255e0

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
      if (uVar2 <= param_4) {
        return uVar2;
      }
      *piVar1 = 0x22;
      return param_4;
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




// Function: code_timespec @ 0x257a0

long code_timespec(long param_1,uint param_2,long param_3)

{
  long lVar1;
  long lVar2;
  long lVar3;
  int iVar4;
  int iVar5;
  
  lVar1 = param_3 + 1;
  if (param_2 < 1000000000) {
    if ((param_2 != 0) && (param_1 < 0)) {
      param_1 = param_1 + 1;
      param_2 = 1000000000 - param_2;
      goto LAB_001257de;
    }
    if (param_1 < 0) goto LAB_001257de;
    lVar1 = FUN_00145370(param_1,lVar1);
  }
  else {
    if (-1 < param_1) {
      lVar1 = FUN_00145370(param_1,lVar1);
      goto LAB_001258e0;
    }
    param_2 = 0;
LAB_001257de:
    lVar1 = FUN_00145370(-param_1,lVar1);
    *(undefined1 *)(lVar1 + -1) = 0x2d;
    lVar1 = lVar1 + -1;
  }
  if (param_2 != 0) {
    lVar2 = (long)(int)param_2;
    *(undefined1 *)(param_3 + 0x15) = 0x2e;
    if ((int)param_2 % 10 == 0) {
      iVar4 = 9;
      do {
        iVar5 = iVar4;
        iVar4 = iVar5 + -1;
        param_2 = (int)(lVar2 * 0x66666667 >> 0x22) - ((int)param_2 >> 0x1f);
        lVar2 = (long)(int)param_2;
      } while ((int)param_2 % 10 == 0);
      iVar5 = iVar5 + -2;
      lVar3 = (long)iVar5;
      *(undefined1 *)(param_3 + 0x16 + (long)iVar4) = 0;
      *(char *)(param_3 + 0x16 + lVar3) = (char)((int)param_2 % 10) + '0';
      if (iVar5 == 0) {
        return lVar1;
      }
    }
    else {
      *(undefined1 *)(param_3 + 0x1f) = 0;
      lVar3 = 8;
      *(char *)(param_3 + 0x1e) = (char)((int)param_2 % 10) + '0';
    }
    do {
      param_2 = (int)(lVar2 * 0x66666667 >> 0x22) - ((int)param_2 >> 0x1f);
      lVar2 = (long)(int)param_2;
      *(char *)(param_3 + 0x15 + lVar3) = (char)param_2 + (char)((int)param_2 / 10) * -10 + '0';
      lVar3 = lVar3 + -1;
    } while ((int)lVar3 != 0);
    return lVar1;
  }
LAB_001258e0:
  *(undefined1 *)(param_3 + 0x15) = 0;
  return lVar1;
}




// Function: decode_timespec @ 0x25930

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
joined_r0x00125ab4:
    iVar3 = 0;
    if ((param_3 == '\0') || (iVar3 = 0, *pcVar7 != '.')) goto LAB_00125a43;
  }
  else {
    uVar5 = strtoumax(param_1,param_2,10);
    if (-1 < (long)uVar5) {
      pcVar7 = *param_2;
      iVar10 = *piVar4;
      goto joined_r0x00125ab4;
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
    if (iVar2 < 9) goto LAB_00125a30;
  }
  else {
    iVar3 = 0;
    uVar9 = 0;
    iVar2 = 0;
LAB_00125a30:
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
LAB_00125a43:
  lVar6 = (long)iVar3;
  *param_2 = pcVar7;
  if (iVar10 == 0x22) {
    lVar6 = -1;
  }
  auVar12._8_8_ = lVar6;
  auVar12._0_8_ = uVar5;
  return auVar12;
}




// Function: maybe_backup_file @ 0x25c10

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 maybe_backup_file(char *param_1,char param_2)

{
  int iVar1;
  char *pcVar2;
  void *pvVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  free(DAT_00182600);
  pcVar2 = (char *)0x0;
  if (param_1 != (char *)0x0) {
    pcVar2 = (char *)FUN_0014ee30(param_1);
  }
  DAT_00182600 = pcVar2;
  free(DAT_001825f8);
  DAT_001825f8 = (char *)0x0;
  if ((((param_2 == '\0') || (DAT_00182d70 != '\0')) ||
      (_DAT_00182d78 = strchr(param_1,0x3a), _DAT_00182d78 == (char *)0x0)) ||
     ((_DAT_00182d78 <= param_1 ||
      (pvVar3 = memchr(param_1,0x2f,(long)_DAT_00182d78 - (long)param_1), pvVar3 != (void *)0x0))))
  {
    iVar1 = fstatat(DAT_0017f234,param_1,&sStack_c8,DAT_00182800);
    if (iVar1 == 0) {
      if (((sStack_c8.st_mode & 0xf000) != 0x4000) &&
         ((param_2 == '\0' || ((sStack_c8.st_mode & 0xb000) != 0x2000)))) {
        pcVar2 = (char *)FUN_0013ffd0(DAT_0017f234,param_1,DAT_00182b80);
        DAT_001825f8 = pcVar2;
        if (pcVar2 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_00117020();
        }
        iVar1 = renameat(DAT_0017f234,DAT_00182600,DAT_0017f234,pcVar2);
        if (iVar1 != 0) {
          piVar4 = __errno_location();
          iVar1 = *piVar4;
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
            pcVar2 = DAT_001825f8;
          }
          uVar7 = FUN_0014d2b0(1,pcVar2);
          uVar5 = FUN_0014ce40(DAT_00182600);
          uVar6 = dcgettext(0,"%s: Cannot rename to %s",5);
          error(0,iVar1,uVar6,uVar5,uVar7);
          DAT_00182d58 = 2;
          free(DAT_001825f8);
          uVar7 = 0;
          DAT_001825f8 = (char *)0x0;
          goto LAB_00125cf3;
        }
        if (DAT_00182a54 != 0) {
          uVar7 = FUN_0014d2b0(1,pcVar2);
          uVar5 = FUN_0014d2b0(0,DAT_00182600);
          uVar6 = dcgettext(0,"Renaming %s to %s\n",5);
          __fprintf_chk(DAT_00181f28,1,uVar6,uVar5,uVar7);
        }
      }
    }
    else {
      piVar4 = __errno_location();
      if (*piVar4 != 2) {
        FUN_00133410(param_1);
        uVar7 = 0;
        goto LAB_00125cf3;
      }
    }
  }
  uVar7 = 1;
LAB_00125cf3:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: undo_last_backup @ 0x25ea0

void undo_last_backup(void)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  if (DAT_001825f8 != (char *)0x0) {
    iVar1 = renameat(DAT_0017f234,DAT_001825f8,DAT_0017f234,DAT_00182600);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar3 = FUN_0014d2b0(1,DAT_00182600);
      uVar4 = FUN_0014ce40(DAT_001825f8);
      uVar5 = dcgettext(0,"%s: Cannot rename to %s",5);
      error(0,iVar1,uVar5,uVar4,uVar3);
      DAT_00182d58 = 2;
    }
    if (DAT_00182a54 != 0) {
      uVar3 = FUN_0014d2b0(1,DAT_00182600);
      uVar4 = FUN_0014d2b0(0,DAT_001825f8);
      uVar5 = dcgettext(0,"Renaming %s back to %s\n",5);
      __fprintf_chk(DAT_00181f28,1,uVar5,uVar4,uVar3);
    }
    free(DAT_001825f8);
    DAT_001825f8 = (char *)0x0;
    return;
  }
  return;
}




// Function: blocking_read @ 0x25fe0

long blocking_read(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  uint uVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  
  lVar3 = FUN_0014d570();
  if (lVar3 == -1) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_0014f850(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_0014f850(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          lVar3 = FUN_0014d570(param_1,param_2,param_3);
          return lVar3;
        }
      }
    }
  }
  return lVar3;
}




// Function: blocking_write @ 0x26070

ulong blocking_write(undefined4 param_1,long param_2,ulong param_3)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  int *piVar4;
  long lVar5;
  
  uVar3 = FUN_001431a0();
  if (uVar3 < param_3) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_0014f850(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_0014f850(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          lVar5 = FUN_001431a0(param_1,param_2 + uVar3,param_3 - uVar3);
          uVar3 = uVar3 + lVar5;
        }
      }
    }
  }
  return uVar3;
}




// Function: set_file_atime @ 0x26100

void set_file_atime(void)

{
  long lVar1;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001412d0();
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: chdir_count @ 0x26160

int chdir_count(void)

{
  int iVar1;
  
  iVar1 = (int)DAT_001825e8 + -1;
  if (DAT_001825e8 == 0) {
    iVar1 = 0;
  }
  return iVar1;
}




// Function: chdir_arg @ 0x26180

ulong chdir_arg(char *param_1)

{
  char *pcVar1;
  long *plVar2;
  char cVar3;
  undefined1 auVar4 [16];
  undefined8 *puVar5;
  ulong uVar6;
  
  uVar6 = DAT_001825e8;
  if (DAT_001825e8 != DAT_001825e0) goto LAB_0012619b;
  if (DAT_001825e0 == 0) {
    DAT_001825e0 = 2;
    if (DAT_001825f0 != (undefined8 *)0x0) goto LAB_00126244;
  }
  else if (DAT_001825f0 == (undefined8 *)0x0) {
    auVar4._8_8_ = 0;
    auVar4._0_8_ = DAT_001825e0;
    if ((SUB168(ZEXT816(0x18) * auVar4,0) < 0) || (SUB168(ZEXT816(0x18) * auVar4,8) != 0))
    goto LAB_001262f7;
  }
  else {
    if (0x38e38e38e38e38d < DAT_001825e0) {
LAB_001262f7:
                    /* WARNING: Subroutine does not return */
      FUN_00117020();
    }
LAB_00126244:
    DAT_001825e0 = DAT_001825e0 + 1 + (DAT_001825e0 >> 1);
  }
  puVar5 = (undefined8 *)FUN_0014ece0(DAT_001825f0,DAT_001825e0 * 0x18);
  uVar6 = DAT_001825e8;
  DAT_001825f0 = puVar5;
  if (DAT_001825e8 == 0) {
    puVar5[1] = 0;
    *puVar5 = ".";
    uVar6 = 1;
    *(undefined4 *)(puVar5 + 2) = 0xffffff9c;
  }
LAB_0012619b:
  if (*param_1 == '.') {
    do {
      while( true ) {
        if (param_1[1] != '/') {
          cVar3 = param_1[1];
          goto joined_r0x001262b9;
        }
        pcVar1 = param_1 + 2;
        param_1 = param_1 + 2;
        if (*pcVar1 == '/') break;
        if (*pcVar1 != '.') goto LAB_00126207;
      }
      do {
        pcVar1 = param_1 + 1;
        param_1 = param_1 + 1;
      } while (*pcVar1 == '/');
    } while (*pcVar1 == '.');
LAB_00126207:
    cVar3 = *param_1;
joined_r0x001262b9:
    if (cVar3 == '\0') {
      DAT_001825e8 = uVar6;
      return (ulong)((int)uVar6 - 1);
    }
  }
  plVar2 = DAT_001825f0 + uVar6 * 3;
  *plVar2 = (long)param_1;
  plVar2[1] = 0;
  *(undefined4 *)(plVar2 + 2) = 0;
  DAT_001825e8 = uVar6 + 1;
  return uVar6;
}




// Function: tar_dirname @ 0x26320

undefined8 tar_dirname(void)

{
  return *(undefined8 *)(DAT_001825f0 + (long)DAT_00182580 * 0x18);
}




// Function: open_diag @ 0x26370

void open_diag(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_00132ea0();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132ee0();
  return;
}




// Function: read_diag_details @ 0x263a0

void read_diag_details(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_00132f20();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00132fe0();
  return;
}




// Function: readlink_diag @ 0x263d0

void readlink_diag(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_00133140();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00133160();
  return;
}




// Function: savedir_diag @ 0x26400

void savedir_diag(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_001331a0();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_001331c0();
  return;
}




// Function: seek_diag_details @ 0x26430

void seek_diag_details(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_00133200();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_001332d0();
  return;
}




// Function: stat_diag @ 0x26460

void stat_diag(void)

{
  if (DAT_00182b52 == '\0') {
    FUN_00133410();
    return;
  }
  if ((DAT_00181b80._2_1_ & 0x80) == 0) {
    return;
  }
  FUN_00133430();
  return;
}




// Function: file_removed_diag @ 0x26490

void file_removed_diag(undefined8 param_1,char param_2,code *UNRECOVERED_JUMPTABLE)

{
  int *piVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (param_2 == '\0') {
    piVar1 = __errno_location();
    if (*piVar1 == 2) {
      if (((byte)DAT_00181b80 & 0x40) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar2 = FUN_0014ce40(param_1);
        uVar3 = dcgettext(0,"%s: File removed before we read it",5);
        error(0,0,uVar3,uVar2);
      }
      FUN_0012fd00(1);
      return;
    }
  }
                    /* WARNING: Could not recover jumptable at 0x001264b7. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*UNRECOVERED_JUMPTABLE)(param_1);
  return;
}




// Function: xfork @ 0x26520

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
  FUN_00132b70(&DAT_00165779,uVar1);
}




// Function: xpipe @ 0x26560

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
  FUN_00132b70(&DAT_00165793,uVar1);
}




// Function: page_aligned_alloc @ 0x265a0

long page_aligned_alloc(long *param_1,ulong param_2)

{
  ulong uVar1;
  int iVar2;
  long lVar3;
  ulong uVar4;
  
  iVar2 = getpagesize();
  uVar4 = (ulong)iVar2;
  if (!CARRY8(param_2,uVar4)) {
    lVar3 = FUN_0014ecc0();
    uVar1 = lVar3 + -1 + uVar4;
    *param_1 = lVar3;
    return uVar1 - uVar1 % uVar4;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00117020(param_2 + uVar4);
}




// Function: namebuf_create @ 0x265f0

undefined8 * namebuf_create(char *param_1)

{
  char cVar1;
  undefined8 *puVar2;
  size_t sVar3;
  char *__dest;
  
  puVar2 = (undefined8 *)FUN_0014ecc0(0x18);
  sVar3 = strlen(param_1);
  puVar2[1] = sVar3 + 2;
  __dest = (char *)FUN_0014ecc0();
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




// Function: namebuf_free @ 0x26660

void namebuf_free(undefined8 *param_1)

{
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: namebuf_name @ 0x26680

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
      lVar2 = FUN_0014ed20(lVar2,param_1 + 1);
      *param_1 = lVar2;
      lVar3 = param_1[2];
    } while ((ulong)param_1[1] <= lVar3 + 1 + sVar1);
  }
  strcpy((char *)(lVar2 + lVar3),param_2);
  return *param_1;
}




// Function: normalize_filename @ 0x26700

char * normalize_filename(int param_1,char *param_2)

{
  long lVar1;
  undefined8 *puVar2;
  int iVar3;
  int iVar4;
  size_t sVar5;
  size_t sVar6;
  char *pcVar7;
  char *__s;
  long *plVar8;
  long lVar9;
  long lVar10;
  undefined8 uVar11;
  int iVar12;
  long lVar13;
  
  iVar3 = DAT_00182580;
  if (*param_2 == '/') {
    pcVar7 = (char *)FUN_0014ee30(param_2);
  }
  else {
    if (DAT_001825f0 == 0) {
      __s = DAT_00182588;
      if ((DAT_00182588 == (char *)0x0) &&
         (__s = (char *)FUN_0014ee70(), DAT_00182588 = __s, __s == (char *)0x0)) {
LAB_001267e4:
                    /* WARNING: Subroutine does not return */
        FUN_00132b70("getcwd",".");
      }
    }
    else {
      lVar1 = (long)param_1 * 0x18;
      __s = *(char **)(DAT_001825f0 + 8 + lVar1);
      if (__s == (char *)0x0) {
        plVar8 = (long *)(DAT_001825f0 + -0x10 + lVar1);
        iVar4 = param_1;
        if (param_1 < 0) {
          iVar12 = param_1 + 1;
        }
        else {
          do {
            iVar12 = iVar4;
            if (iVar12 == 0) goto LAB_001268ef;
            lVar13 = *plVar8;
            plVar8 = plVar8 + -3;
            iVar4 = iVar12 + -1;
          } while (lVar13 == 0);
        }
        if (iVar12 <= param_1) {
LAB_001268ef:
          do {
            if (iVar12 != DAT_00182580) {
              FUN_00124f80(iVar12);
            }
            lVar13 = DAT_001825f0;
            if (iVar12 == 0) {
              lVar10 = FUN_0014ee70();
              *(long *)(lVar13 + 8) = lVar10;
              if (lVar10 == 0) goto LAB_001267e4;
              iVar12 = 1;
              if (param_1 < 1) break;
              goto LAB_001268ef;
            }
            lVar10 = (long)iVar12;
            iVar12 = iVar12 + 1;
            lVar13 = lVar10 * 0x18;
            puVar2 = (undefined8 *)(DAT_001825f0 + lVar13);
            if (*(char *)*puVar2 == '/') {
              uVar11 = FUN_0014ee30();
              puVar2[1] = uVar11;
            }
            else {
              plVar8 = (long *)FUN_001265f0(*(undefined8 *)(DAT_001825f0 + -0x10 + lVar13));
              pcVar7 = *(char **)(DAT_001825f0 + lVar10 * 0x18);
              if (*(char *)(*plVar8 + -1 + plVar8[2]) != '/') {
                FUN_00126680(plVar8,&DAT_0017f230);
                plVar8[2] = plVar8[2] + 1;
              }
              FUN_00126680(plVar8,pcVar7);
              sVar5 = strlen(pcVar7);
              lVar10 = *plVar8;
              lVar9 = sVar5 + plVar8[2];
              plVar8[2] = lVar9;
              lVar13 = lVar13 + DAT_001825f0;
              if (*(char *)(lVar10 + -1 + lVar9) == '/') {
                *(undefined1 *)(lVar10 + lVar9) = 0;
              }
              free(plVar8);
              *(long *)(lVar13 + 8) = lVar10;
            }
          } while (iVar12 <= param_1);
          if (iVar3 != DAT_00182580) {
            FUN_00124f80(iVar3);
          }
          __s = *(char **)(DAT_001825f0 + 8 + lVar1);
        }
      }
    }
    sVar5 = strlen(__s);
    sVar6 = strlen(param_2);
    pcVar7 = (char *)FUN_0014ecc0(sVar5 + 2 + sVar6);
    strcpy(pcVar7,__s);
    pcVar7[sVar5] = '/';
    strcpy(pcVar7 + sVar5 + 1,param_2);
  }
  FUN_001253e0(pcVar7);
  return pcVar7;
}




// Function: tar_savedir @ 0x26970

long tar_savedir(undefined8 param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  DIR *__dirp;
  long lVar3;
  
  iVar1 = __openat_2(DAT_0017f234,param_1,DAT_00182808 | 0x10000);
  if (iVar1 < 0) {
    if ((param_2 != 0) || (piVar2 = __errno_location(), *piVar2 != 2)) {
      FUN_00132ea0(param_1);
    }
  }
  else {
    __dirp = fdopendir(iVar1);
    if (__dirp != (DIR *)0x0) {
      lVar3 = FUN_0014d730(__dirp,DAT_001827e4);
      if (lVar3 == 0) {
        FUN_001331a0(param_1);
      }
      iVar1 = closedir(__dirp);
      if (iVar1 == 0) {
        return lVar3;
      }
      FUN_001331a0(param_1);
      return lVar3;
    }
    FUN_001331a0(param_1);
    iVar1 = close(iVar1);
    if (iVar1 != 0) {
      FUN_001331a0(param_1);
      return 0;
    }
  }
  return 0;
}




// Function: remove_any_file @ 0x26a40

uint remove_any_file(char *param_1,int param_2)

{
  char *__s;
  uint uVar1;
  int iVar2;
  uint *puVar3;
  char *__s_00;
  size_t sVar4;
  void *__ptr;
  
  iVar2 = unlinkat(DAT_0017f234,param_1,0);
  if (iVar2 == 0) {
    return 1;
  }
  puVar3 = (uint *)__errno_location();
  if ((*puVar3 == 1) || (*puVar3 == 0x15)) {
    iVar2 = FUN_00125ba0(param_1);
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
    __s_00 = (char *)FUN_00126970(param_1,0);
    if (__s_00 != (char *)0x0) {
      sVar4 = strlen(__s_00);
      __s = __s_00;
      while( true ) {
        if (sVar4 == 0) {
          free(__s_00);
          iVar2 = FUN_00125ba0(param_1);
          return (uint)(iVar2 == 0);
        }
        __ptr = (void *)FUN_00129320(param_1,__s);
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




// Function: merge_sort_sll @ 0x26b90

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




// Function: name_elt_alloc @ 0x26d00

void name_elt_alloc(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_0014ecc0(0x38);
  if (DAT_001826b0 == (undefined8 *)0x0) {
    *puVar3 = 0;
    puVar3[1] = 0;
    *(undefined4 *)(puVar3 + 2) = 3;
    DAT_001826b0 = puVar3;
    puVar3 = (undefined8 *)FUN_0014ecc0(0x38);
  }
  puVar2 = DAT_001826b0;
  puVar1 = (undefined8 *)DAT_001826b0[1];
  puVar3[1] = puVar1;
  if (puVar1 != (undefined8 *)0x0) {
    *puVar1 = puVar3;
  }
  *puVar3 = puVar2;
  puVar2[1] = puVar3;
  return;
}




// Function: name_compare @ 0x26d70

bool name_compare(long param_1,long param_2)

{
  int iVar1;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x58),*(char **)(param_2 + 0x58));
  return iVar1 == 0;
}




// Function: compare_names @ 0x26d90

void compare_names(long param_1,long param_2)

{
  strcmp(*(char **)(param_1 + 0x10),*(char **)(param_2 + 0x10));
  return;
}




// Function: handle_option @ 0x26db0

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
          iVar2 = FUN_001375a0(param_1,&local_118,0x2000e46);
          if (iVar2 != 0) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
            }
            uVar5 = FUN_001358f0(&local_118);
            uVar6 = dcgettext(0,"cannot split string \'%s\': %s",5);
            error(0,0,uVar6,param_1,uVar5);
                    /* WARNING: Subroutine does not return */
            FUN_00116fd0();
          }
          *local_110 = DAT_00182dd0;
          local_130 = *(undefined8 *)(param_2 + 0x18);
          local_138[0] = 2;
          local_128 = *(undefined8 *)(param_2 + 0x20);
          FUN_0012fac0((int)local_108 + local_118,local_110,local_138);
          lVar4 = CONCAT44(uStack_114,local_118) + local_108;
          if (lVar4 != 0) {
            puVar1 = local_110 + lVar4;
            do {
              *local_110 = 0;
              local_110 = local_110 + 1;
            } while (puVar1 != local_110);
          }
          FUN_00135800(&local_118);
          uVar8 = 0;
          goto LAB_00126eb6;
        }
        break;
      }
      cVar7 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar7 != '\0');
  }
  uVar8 = 1;
LAB_00126eb6:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: regex_usage_warning @ 0x26f30

undefined4 regex_usage_warning(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  if (DAT_001826fc == 0) {
    cVar1 = FUN_00140850(param_1,0);
    if (cVar1 != '\0') {
      DAT_00182634 = 1;
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = dcgettext(0,"Pattern matching characters used in file names",5);
      error(0,0,uVar2);
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar2 = dcgettext(0,
                        "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning"
                        ,5);
      error(0,0,uVar2);
    }
    return DAT_00182634;
  }
  return DAT_00182634;
}




// Function: name_hash @ 0x26fd0

void name_hash(long param_1)

{
  FUN_00143b80(*(undefined8 *)(param_1 + 0x58));
  return;
}




// Function: name_list_advance @ 0x270f0

void name_list_advance(void)

{
  int iVar1;
  undefined8 *__ptr;
  long *__ptr_00;
  
  __ptr_00 = DAT_001826b0;
  DAT_001826b0 = (long *)*DAT_001826b0;
  if (DAT_001826b0 != (long *)0x0) {
    *(undefined8 *)((long)DAT_001826b0 + 8) = 0;
  }
  iVar1 = (int)__ptr_00[2];
  if ((iVar1 != 4) && (iVar1 != 1)) {
    if (iVar1 == 3) {
      free(__ptr_00);
      return;
    }
    while (__ptr = DAT_00182628, DAT_00182628 != (undefined8 *)0x0) {
      DAT_00182628 = (long *)DAT_00182628[1];
      free(__ptr);
    }
    free(__ptr_00);
    return;
  }
  if ((DAT_00182ba8 != 3) && (DAT_00182ba8 != 8)) {
    return;
  }
  *__ptr_00 = 0;
  __ptr_00[1] = (long)DAT_00182628;
  if (DAT_00182628 != (undefined8 *)0x0) {
    *DAT_00182628 = (long)__ptr_00;
  }
  DAT_00182628 = __ptr_00;
  return;
}




// Function: names_parse_opt @ 0x271a0

undefined8 names_parse_opt(int param_1,undefined8 param_2,long param_3)

{
  int *piVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long lVar4;
  undefined8 uVar5;
  long *plVar6;
  
  if (param_1 == 0x100) {
    lVar4 = FUN_00126d00();
    *(undefined4 *)(lVar4 + 0x10) = 0;
    *(undefined8 *)(lVar4 + 0x18) = param_2;
    if (DAT_00182630 == 0) {
      DAT_00182630 = 1;
    }
    else if (DAT_00182630 == 1) {
      DAT_00182630 = 2;
      return 0;
    }
    return 0;
  }
  if (param_1 < 0x101) {
    if (param_1 == 0x43) {
      lVar4 = FUN_00126d00();
      *(undefined8 *)(lVar4 + 0x18) = param_2;
      *(undefined4 *)(lVar4 + 0x10) = 1;
      return 0;
    }
    if (param_1 == 0x54) {
      lVar4 = FUN_00126d00();
      DAT_00182630 = 2;
      *(undefined4 *)(lVar4 + 0x10) = 2;
      *(undefined8 *)(lVar4 + 0x20) = 0;
      *(undefined8 *)(lVar4 + 0x30) = 0;
      *(undefined8 *)(lVar4 + 0x18) = param_2;
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
    FUN_0012fd00(2);
    return 7;
  }
  for (plVar6 = &DAT_0017f280; ((*plVar6 != 0 || ((int)plVar6[1] != 0)) || (plVar6[4] != 0));
      plVar6 = plVar6 + 6) {
    if (param_1 == (int)plVar6[1]) {
      lVar4 = FUN_00126d00();
      *(undefined8 *)(lVar4 + 0x20) = param_2;
      *(undefined4 *)(lVar4 + 0x10) = 4;
      *(int *)(lVar4 + 0x18) = param_1;
      return 0;
    }
  }
  return 7;
}




// Function: name_next_elt @ 0x27340

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined * name_next_elt(int param_1)

{
  byte bVar1;
  long lVar2;
  undefined1 uVar3;
  int iVar4;
  uint uVar5;
  undefined4 uVar6;
  char *pcVar7;
  size_t sVar8;
  char *pcVar9;
  long *plVar10;
  undefined8 uVar11;
  long *plVar12;
  undefined8 uVar13;
  long lVar14;
  undefined8 uVar15;
  undefined8 *puVar16;
  _IO_FILE *p_Var17;
  int *piVar18;
  byte *pbVar19;
  ulong uVar20;
  byte *pbVar21;
  char *pcVar22;
  undefined *puVar23;
  uint uVar24;
  undefined **ppuVar25;
  long in_FS_OFFSET;
  bool bVar26;
  bool bVar27;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar12 = DAT_001826b0;
switchD_00127393_default:
  if (plVar12 != (long *)0x0) goto code_r0x00127387;
  goto LAB_00127877;
code_r0x00127387:
  switch((int)plVar12[2]) {
  case 0:
    goto switchD_00127393_caseD_0;
  case 1:
    if (param_1 == 0) goto switchD_00127393_caseD_0;
    uVar11 = FUN_0014ee30(plVar12[3]);
    FUN_00126180(uVar11);
    FUN_00126300();
    FUN_001270f0();
    plVar12 = DAT_001826b0;
    goto switchD_00127393_default;
  case 2:
    break;
  case 4:
    lVar14 = plVar12[4];
    lVar2 = plVar12[3];
    switch((int)lVar2) {
    case 0x58:
      iVar4 = FUN_00141210(FUN_00140c90,DAT_00182b60,lVar14,
                           (uint)(DAT_001826fc != 1) << 0x1c | DAT_001826f8 | DAT_00182ad4,10);
      if (iVar4 != 0) {
        piVar18 = __errno_location();
        iVar4 = *piVar18;
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar11 = FUN_0014ce40(lVar14);
        error(0,iVar4,"%s",uVar11);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      break;
    default:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      error(0,0,"unhandled positional option %d",(int)lVar2);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
    case 0x101:
      puVar23 = &DAT_001657df;
      ppuVar25 = &PTR_DAT_0017d708;
      do {
        FUN_00140c90(DAT_00182b60,puVar23,0x10000000);
        puVar23 = *ppuVar25;
        ppuVar25 = ppuVar25 + 1;
      } while (puVar23 != (undefined *)0x0);
      break;
    case 0x102:
      FUN_00112e10("CACHEDIR.TAG",1,FUN_00112f10);
      break;
    case 0x103:
      FUN_00112e10("CACHEDIR.TAG",2,FUN_00112f10);
      break;
    case 0x104:
      FUN_00112e10("CACHEDIR.TAG",3,FUN_00112f10);
      break;
    case 0x105:
      FUN_00140c90(DAT_00182b60,lVar14,
                   DAT_001826f8 | DAT_00182ad4 | (uint)(DAT_001826fc != 1) << 0x1c);
      break;
    case 0x106:
      FUN_001173d0(lVar14,2);
      break;
    case 0x107:
      FUN_001173d0(lVar14,1);
      break;
    case 0x108:
      FUN_00112e10(lVar14,1,0);
      break;
    case 0x109:
      FUN_00112e10(lVar14,2,0);
      break;
    case 0x10a:
      FUN_00112e10(lVar14,3,0);
      break;
    case 0x10b:
      puVar23 = &DAT_001657e3;
      ppuVar25 = &PTR_s__cvsignore_0017d728;
      do {
        FUN_00140c90(DAT_00182b60,puVar23,0);
        puVar23 = *ppuVar25;
        ppuVar25 = ppuVar25 + 1;
      } while (puVar23 != (undefined *)0x0);
      break;
    case 0x10c:
      FUN_00117810();
      break;
    case 0x10d:
      DAT_001826f8 = DAT_001826f8 | 0x10;
      break;
    case 0x10e:
      DAT_001826f8 = DAT_001826f8 & 0xffffffef;
      break;
    case 0x10f:
      DAT_001826f8 = DAT_001826f8 | 0x40000000;
      break;
    case 0x110:
      DAT_001826f8 = DAT_001826f8 & 0xbfffffff;
      DAT_0017f238 = 0;
      break;
    case 0x111:
      DAT_00182ad4 = 8;
      break;
    case 0x112:
      DAT_00182ad4 = 0;
      break;
    case 0x113:
      DAT_001827e8 = '\x01';
      break;
    case 0x114:
      DAT_001827e8 = '\0';
      break;
    case 0x115:
      DAT_00182700 = 0;
      break;
    case 0x116:
      DAT_001826f8 = DAT_001826f8 | 1;
      break;
    case 0x117:
      DAT_001826fc = 1;
      break;
    case 0x118:
      DAT_0017f23c = '\0';
      DAT_00182700 = 1;
      break;
    case 0x119:
      DAT_0017f23c = '\n';
      DAT_00182700 = 0;
      break;
    case 0x11a:
      DAT_00182700 = 1;
      break;
    case 0x11b:
      DAT_001826f8 = DAT_001826f8 & 0xfffffffe;
      break;
    case 0x11c:
      DAT_001826fc = 2;
    }
  case 3:
    FUN_001270f0();
    plVar12 = DAT_001826b0;
  default:
    goto switchD_00127393_default;
  }
  p_Var17 = (_IO_FILE *)plVar12[6];
  if (p_Var17 != (FILE *)0x0) {
    uVar24 = *(uint *)(plVar12 + 5);
    goto LAB_00127503;
  }
  pcVar22 = (char *)plVar12[3];
  iVar4 = strcmp(pcVar22,"-");
  if (iVar4 != 0) {
    iVar4 = stat(pcVar22,&local_d8);
    plVar10 = plVar12;
    if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_001333f0(pcVar22);
    }
    do {
      if (((int)plVar10[2] == 2) && (plVar10[6] != 0)) {
        lVar14 = plVar10[3];
        puVar16 = DAT_00182698;
        goto joined_r0x00127a2e;
      }
      plVar10 = (long *)*plVar10;
    } while (plVar10 != (long *)0x0);
    lVar14 = dcgettext(0,"command line",5);
    puVar16 = DAT_00182698;
joined_r0x00127a2e:
    do {
      if (puVar16 == (undefined8 *)0x0) goto LAB_00127ae0;
      if ((puVar16[1] == local_d8.st_ino) && (puVar16[2] == local_d8.st_dev)) {
        uVar6 = FUN_0014c780(0,0x3a,1);
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar11 = puVar16[3];
        uVar13 = FUN_0014ca40(0,pcVar22);
        uVar15 = dcgettext(0,"%s: file list requested from %s already read from %s",5);
        error(0,0,uVar15,uVar13,lVar14,uVar11);
        DAT_00182d58 = 2;
        FUN_0014c780(0,0x3a,uVar6);
        FUN_001270f0();
        plVar12 = DAT_001826b0;
        goto switchD_00127393_default;
      }
      puVar16 = (undefined8 *)*puVar16;
    } while( true );
  }
  FUN_0012df30(&DAT_001657e7);
  p_Var17 = stdin;
  plVar12[6] = (long)stdin;
LAB_001277a0:
  uVar3 = DAT_00182700;
  uVar24 = (uint)DAT_0017f23c;
  *(uint *)(plVar12 + 5) = uVar24;
  *(undefined1 *)((long)plVar12 + 0x2c) = uVar3;
LAB_00127503:
  lVar14 = plVar12[4];
  pbVar21 = (byte *)p_Var17->_IO_read_end;
  pbVar19 = (byte *)p_Var17->_IO_read_ptr;
LAB_00127510:
  lVar14 = lVar14 + 1;
  plVar12[4] = lVar14;
  if (pbVar19 < pbVar21) goto code_r0x00127522;
  uVar5 = __uflow(p_Var17);
  bVar27 = uVar5 != 0xffffffff;
  if ((uVar24 != uVar5) && (bVar27)) goto LAB_00127537;
  bVar26 = true;
  uVar20 = 0;
  goto LAB_001276d0;
LAB_00127ae0:
  puVar16 = (undefined8 *)FUN_0014ecc0(0x20);
  pcVar22 = (char *)plVar12[3];
  puVar16[3] = lVar14;
  *puVar16 = DAT_00182698;
  DAT_00182698 = puVar16;
  puVar16[1] = local_d8.st_ino;
  puVar16[2] = local_d8.st_dev;
  p_Var17 = fopen(pcVar22,"r");
  plVar12[6] = (long)p_Var17;
  if (p_Var17 != (FILE *)0x0) goto LAB_001277a0;
  FUN_00132ec0(plVar12[3]);
LAB_00127b40:
  FUN_001251b0();
  goto LAB_00127616;
switchD_00127393_caseD_0:
  pcVar22 = (char *)plVar12[3];
  sVar8 = strlen(pcVar22);
  while (DAT_001826a0 <= sVar8) {
    DAT_001826a8 = (char *)FUN_0014ed20(DAT_001826a8,&DAT_001826a0);
  }
  pcVar7 = strcpy(DAT_001826a8,pcVar22);
  sVar8 = strlen(pcVar7);
  pcVar22 = pcVar7 + (sVar8 - 1);
  if (pcVar7 < pcVar7 + (sVar8 - 1)) {
    do {
      if (*pcVar22 != '/') break;
      pcVar9 = pcVar22 + -1;
      *pcVar22 = '\0';
      pcVar22 = pcVar9;
    } while (pcVar9 != pcVar7);
  }
  if (DAT_001827e8 != '\0') {
    FUN_001251b0();
    pcVar7 = DAT_001826a8;
  }
  _DAT_00182670 = (undefined4)plVar12[2];
  puVar23 = &DAT_00182660;
  _DAT_00182678 = pcVar7;
  FUN_001270f0();
  goto LAB_0012748d;
code_r0x00127522:
  p_Var17->_IO_read_ptr = (char *)(pbVar19 + 1);
  bVar1 = *pbVar19;
  uVar5 = (uint)bVar1;
  pbVar19 = pbVar19 + 1;
  if (uVar24 == bVar1) goto LAB_00127510;
LAB_00127537:
  uVar20 = 0;
  do {
    if (uVar20 == DAT_001826a0) {
      DAT_001826a8 = (char *)FUN_0014ed20(DAT_001826a8,&DAT_001826a0);
      DAT_001826a8[uVar20] = (char)uVar5;
    }
    else {
      DAT_001826a8[uVar20] = (char)uVar5;
    }
    if (uVar5 == 0) {
      if ((DAT_00181b80._1_1_ & 2) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        FUN_0014ce40(plVar12[3]);
        error(0,0,"%s: file name read contains nul character");
      }
      *(undefined4 *)(plVar12 + 5) = 0;
      pcVar22 = DAT_001826a8;
      goto LAB_00127601;
    }
    uVar20 = uVar20 + 1;
    pbVar21 = (byte *)p_Var17->_IO_read_ptr;
    if (pbVar21 < p_Var17->_IO_read_end) {
      p_Var17->_IO_read_ptr = (char *)(pbVar21 + 1);
      bVar26 = true;
      uVar5 = (uint)*pbVar21;
    }
    else {
      uVar5 = __uflow(p_Var17);
      bVar26 = uVar5 != 0xffffffff;
    }
    bVar27 = bVar26;
  } while ((uVar24 != uVar5) && (bVar27 = false, bVar26));
  bVar26 = uVar20 == 0;
  bVar27 = (bool)(bVar27 & bVar26);
LAB_001276d0:
  if (bVar27) {
    p_Var17 = (_IO_FILE *)plVar12[6];
    uVar24 = *(uint *)(plVar12 + 5);
    lVar14 = plVar12[4];
    pbVar21 = (byte *)p_Var17->_IO_read_end;
    pbVar19 = (byte *)p_Var17->_IO_read_ptr;
    goto LAB_00127510;
  }
  if (DAT_001826a0 == uVar20) {
    DAT_001826a8 = (char *)FUN_0014ed20(DAT_001826a8,&DAT_001826a0);
  }
  pcVar22 = DAT_001826a8;
  DAT_001826a8[uVar20] = '\0';
  sVar8 = strlen(pcVar22);
  pcVar7 = pcVar22 + (sVar8 - 1);
  if (pcVar22 < pcVar22 + (sVar8 - 1)) {
    do {
      if (*pcVar7 != '/') break;
      pcVar9 = pcVar7 + -1;
      *pcVar7 = '\0';
      pcVar7 = pcVar9;
    } while (pcVar9 != pcVar22);
  }
  if ((uVar5 == 0xffffffff) && (bVar26)) {
    iVar4 = strcmp((char *)plVar12[3],"-");
    if (iVar4 != 0) {
      fclose((FILE *)plVar12[6]);
    }
    plVar12[6] = 0;
    FUN_001270f0();
    plVar12 = DAT_001826b0;
    goto switchD_00127393_default;
  }
LAB_00127601:
  if (*(char *)((long)plVar12 + 0x2c) != '\0') goto LAB_0012762d;
  if (DAT_001827e8 != '\0') goto LAB_00127b40;
LAB_00127616:
  iVar4 = FUN_00126db0();
  pcVar22 = DAT_001826a8;
  if (iVar4 != 0) {
LAB_0012762d:
    puVar23 = &DAT_00182660;
    _DAT_00182670 = 0;
    _DAT_00182678 = pcVar22;
    goto LAB_0012748d;
  }
  if (DAT_001826b0 != (long *)0x0) {
    plVar12 = (long *)DAT_001826b0[1];
    while (plVar10 = plVar12, plVar12 = DAT_001826b0, plVar10 != (long *)0x0) {
      DAT_001826b0 = plVar10;
      plVar12 = (long *)plVar10[1];
    }
    goto switchD_00127393_default;
  }
LAB_00127877:
  puVar23 = (undefined *)0x0;
  if (DAT_00182628 == (long *)0x0) goto LAB_0012748d;
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar11 = dcgettext(0,
                     "The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly."
                     ,5);
  error(0,0,uVar11);
  plVar12 = DAT_00182628;
  do {
    plVar10 = plVar12;
    plVar12 = (long *)plVar10[1];
  } while ((long *)plVar10[1] != (long *)0x0);
  do {
    DAT_00182d58 = 2;
    if ((int)plVar10[2] == 1) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      lVar14 = FUN_0014d2d0(plVar10[3]);
      pcVar22 = "-C %s has no effect";
LAB_001279ea:
      uVar11 = dcgettext(0,pcVar22,5);
      error(0,0,uVar11,lVar14);
    }
    else if ((int)plVar10[2] == 4) {
      lVar14 = plVar10[4];
      if (lVar14 == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        plVar12 = &DAT_0017f280;
        while( true ) {
          lVar14 = *plVar12;
          if ((((lVar14 == 0) && ((int)plVar12[1] == 0)) && (plVar12[4] == 0)) ||
             ((int)plVar10[3] == (int)plVar12[1])) break;
          plVar12 = plVar12 + 6;
        }
        pcVar22 = "--%s has no effect";
        goto LAB_001279ea;
      }
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
        lVar14 = plVar10[4];
      }
      uVar11 = FUN_0014d2d0(lVar14);
      plVar12 = &DAT_0017f280;
      while( true ) {
        lVar14 = *plVar12;
        if ((((lVar14 == 0) && ((int)plVar12[1] == 0)) && (plVar12[4] == 0)) ||
           ((int)plVar10[3] == (int)plVar12[1])) break;
        plVar12 = plVar12 + 6;
      }
      uVar13 = dcgettext(0,"--%s %s has no effect",5);
      error(0,0,uVar13,lVar14,uVar11);
    }
    DAT_00182d58 = 2;
    plVar10 = (long *)*plVar10;
  } while (plVar10 != (long *)0x0);
  while (plVar12 = DAT_00182628, DAT_00182628 != (long *)0x0) {
    DAT_00182628 = (long *)DAT_00182628[1];
    free(plVar12);
  }
LAB_0012748d:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar23;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: uid_to_uname @ 0x27eb0

void uid_to_uname(__uid_t param_1,undefined8 *param_2)

{
  __uid_t _Var1;
  passwd *ppVar2;
  undefined8 uVar3;
  
  if ((param_1 != 0) && (_Var1 = DAT_001826cc, DAT_001826cc == param_1)) {
LAB_00127f26:
    DAT_001826cc = _Var1;
    uVar3 = FUN_0014ee30(&DAT_0016a7d3);
    *param_2 = uVar3;
    return;
  }
  if ((DAT_001826f0 == 0) || (DAT_001826e4 != param_1)) {
    ppVar2 = getpwuid(param_1);
    _Var1 = param_1;
    if (ppVar2 == (passwd *)0x0) goto LAB_00127f26;
    DAT_001826e4 = param_1;
    FUN_00125120(&DAT_001826f0,ppVar2->pw_name);
  }
  uVar3 = FUN_0014ee30(DAT_001826f0);
  *param_2 = uVar3;
  return;
}




// Function: gid_to_gname @ 0x27f40

void gid_to_gname(__gid_t param_1,undefined8 *param_2)

{
  __gid_t _Var1;
  group *pgVar2;
  undefined8 uVar3;
  
  if ((param_1 != 0) && (_Var1 = DAT_001826c8, DAT_001826c8 == param_1)) {
LAB_00127fb6:
    DAT_001826c8 = _Var1;
    uVar3 = FUN_0014ee30(&DAT_0016a7d3);
    *param_2 = uVar3;
    return;
  }
  if ((DAT_001826e8 == 0) || (DAT_001826e0 != param_1)) {
    pgVar2 = getgrgid(param_1);
    _Var1 = param_1;
    if (pgVar2 == (group *)0x0) goto LAB_00127fb6;
    DAT_001826e0 = param_1;
    FUN_00125120(&DAT_001826e8,pgVar2->gr_name);
  }
  uVar3 = FUN_0014ee30(DAT_001826e8);
  *param_2 = uVar3;
  return;
}




// Function: uname_to_uid @ 0x27fd0

undefined4 uname_to_uid(char *param_1,__uid_t *param_2)

{
  int iVar1;
  passwd *ppVar2;
  
  if ((DAT_001826d8 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_001826d8), iVar1 == 0)) {
    return 0;
  }
  if ((DAT_001826f0 == (char *)0x0) || (*param_1 != *DAT_001826f0)) {
    ppVar2 = getpwnam(param_1);
  }
  else {
    iVar1 = strcmp(param_1,DAT_001826f0);
    if (iVar1 == 0) goto LAB_0012802e;
    ppVar2 = getpwnam(param_1);
  }
  if (ppVar2 == (passwd *)0x0) {
    FUN_00125120(&DAT_001826d8,param_1);
    return 0;
  }
  DAT_001826e4 = ppVar2->pw_uid;
  FUN_00125120(&DAT_001826f0,ppVar2->pw_name);
LAB_0012802e:
  *param_2 = DAT_001826e4;
  return 1;
}




// Function: gname_to_gid @ 0x28080

undefined4 gname_to_gid(char *param_1,__gid_t *param_2)

{
  int iVar1;
  group *pgVar2;
  
  if ((DAT_001826d0 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_001826d0), iVar1 == 0)) {
    return 0;
  }
  if ((DAT_001826e8 == (char *)0x0) || (*param_1 != *DAT_001826e8)) {
    pgVar2 = getgrnam(param_1);
  }
  else {
    iVar1 = strcmp(param_1,DAT_001826e8);
    if (iVar1 == 0) goto LAB_001280de;
    pgVar2 = getgrnam(param_1);
  }
  if (pgVar2 == (group *)0x0) {
    FUN_00125120(&DAT_001826d0,param_1);
    return 0;
  }
  DAT_001826e0 = pgVar2->gr_gid;
  FUN_00125120(&DAT_001826e8,param_1);
LAB_001280de:
  *param_2 = DAT_001826e0;
  return 1;
}




// Function: name_init @ 0x28180

void name_init(void)

{
  long lVar1;
  long lVar2;
  
  DAT_001826a8 = FUN_0014ecc0(0x66);
  DAT_001826a0 = 100;
  if (DAT_001826b0 != 0) {
    lVar2 = *(long *)(DAT_001826b0 + 8);
    while (lVar1 = lVar2, lVar1 != 0) {
      DAT_001826b0 = lVar1;
      lVar2 = *(long *)(lVar1 + 8);
    }
  }
  return;
}




// Function: name_term @ 0x281e0

void name_term(void)

{
  free(DAT_001826a8);
  return;
}




// Function: addname @ 0x28210

undefined8 *
addname(undefined1 *param_1,undefined4 param_2,undefined1 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  uint uVar2;
  undefined8 *puVar3;
  char *__s;
  size_t sVar4;
  bool bVar5;
  
  puVar3 = (undefined8 *)FUN_0014ed80(0x60);
  if (param_1 == (undefined1 *)0x0) {
    param_1 = &DAT_0016a7d3;
  }
  __s = (char *)FUN_0014ee30(param_1);
  puVar3[2] = __s;
  sVar4 = strlen(__s);
  *(undefined4 *)(puVar3 + 5) = param_2;
  puVar3[3] = sVar4;
  uVar2 = DAT_0017f238 | DAT_001826f8 | DAT_00182ad4;
  puVar3[1] = DAT_001826b8;
  bVar5 = DAT_001826fc == 2;
  puVar3[8] = param_4;
  *(undefined1 *)((long)puVar3 + 0x24) = param_3;
  *puVar3 = 0;
  puVar3[6] = 0;
  *(uint *)(puVar3 + 4) = uVar2 | (uint)bVar5 << 0x1c;
  puVar3[7] = 0;
  puVar1 = puVar3;
  if (DAT_001826b8 != (undefined8 *)0x0) {
    *DAT_001826b8 = puVar3;
    puVar1 = DAT_001826c0;
  }
  DAT_001826c0 = puVar1;
  DAT_001826b8 = puVar3;
  return puVar3;
}




// Function: name_gather @ 0x282f0

void name_gather(void)

{
  void *__ptr;
  int iVar1;
  uint uVar2;
  long lVar3;
  undefined8 *puVar4;
  char *__s;
  size_t sVar5;
  int iVar6;
  undefined1 *puVar7;
  bool bVar8;
  
  iVar1 = 0;
  iVar6 = iVar1;
  if (DAT_00182aac == '\0') {
    while (lVar3 = FUN_00127340(0), lVar3 != 0) {
      if (*(int *)(lVar3 + 0x10) == 1) {
        FUN_0014ee30();
        iVar1 = FUN_00126180();
      }
      else {
        FUN_00128210(*(undefined8 *)(lVar3 + 0x18),iVar1,1,0);
        iVar6 = iVar1;
      }
    }
    if (iVar6 == iVar1) {
      return;
    }
  }
  else {
    while (lVar3 = FUN_00127340(0), __ptr = DAT_00182638, lVar3 != 0) {
      if (*(int *)(lVar3 + 0x10) != 1) {
        if (DAT_00182638 != (void *)0x0) {
          free(*(void **)((long)DAT_00182638 + 0x10));
          free(*(void **)((long)__ptr + 0x58));
          free(__ptr);
        }
        puVar7 = *(undefined1 **)(lVar3 + 0x18);
        puVar4 = (undefined8 *)FUN_0014ed80(0x60);
        if (puVar7 == (undefined1 *)0x0) {
          puVar7 = &DAT_0016a7d3;
        }
        __s = (char *)FUN_0014ee30(puVar7);
        puVar4[2] = __s;
        sVar5 = strlen(__s);
        *puVar4 = 0;
        puVar4[3] = sVar5;
        iVar1 = DAT_00182640;
        puVar4[6] = 0;
        *(int *)(puVar4 + 5) = iVar1;
        uVar2 = DAT_0017f238 | DAT_001826f8 | DAT_00182ad4;
        puVar4[7] = 0;
        bVar8 = DAT_001826fc == 2;
        puVar4[8] = 0;
        *(undefined1 *)((long)puVar4 + 0x24) = 1;
        DAT_00182638 = puVar4;
        DAT_001826b8 = puVar4;
        *(uint *)(puVar4 + 4) = uVar2 | (uint)bVar8 << 0x1c;
        DAT_001826c0 = puVar4;
        return;
      }
      FUN_0014ee30(*(undefined8 *)(lVar3 + 0x18));
      DAT_00182640 = FUN_00126180();
    }
    iVar1 = DAT_00182640;
    if (DAT_00182640 == 0) {
      return;
    }
  }
  FUN_00128210(0,iVar1,0,0);
  return;
}




// Function: add_hierarchy_to_namelist @ 0x28480

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
  uVar5 = FUN_0011f810();
  *(undefined8 *)(param_2 + 0x38) = uVar5;
  __s = (char *)FUN_0011fed0(uVar5);
  if (__s != (char *)0x0) {
    uVar2 = *(ulong *)(param_2 + 0x18);
    local_200 = uVar2 + 0x66;
    if (uVar2 < 100) {
      local_200 = 0x66;
    }
    __dest = (char *)FUN_0014ecc0();
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
              __dest = (char *)FUN_0014ed20(__dest,&local_200);
            } while (local_200 < local_238 + sVar6);
          }
          strcpy(__dest + local_238,__s + 1);
          lVar7 = FUN_00128210(__dest,uVar1,0,param_2);
          lVar3 = lVar7;
          if (local_230 != 0) {
            *(long *)(local_218 + 0x50) = lVar7;
            lVar3 = local_230;
          }
          local_230 = lVar3;
          FUN_0012fb40(local_1f8);
          iVar4 = *(int *)(param_1 + 0x1a8);
          local_60 = param_1;
          if (iVar4 < 0) {
            piVar8 = __errno_location();
            *piVar8 = -iVar4;
LAB_00128645:
            FUN_00126370(__dest);
          }
          else {
            iVar4 = FUN_00114950(param_1,__s + 1,DAT_00182808 | 0x10000);
            if (iVar4 < 0) goto LAB_00128645;
            local_50 = iVar4;
            iVar4 = fstat(iVar4,local_1a0);
            if (iVar4 == 0) {
              local_1f8[0] = FUN_0014ee30(__dest);
              add_hierarchy_to_namelist(local_1f8,lVar7);
              FUN_00114a80(local_1f8);
            }
            else {
              FUN_00126460();
            }
          }
          FUN_0012fc00(local_1f8);
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




// Function: add_starting_file @ 0x286f0

void add_starting_file(undefined1 *param_1)

{
  long *__ptr;
  undefined8 *puVar1;
  long *plVar2;
  uint uVar3;
  undefined8 *puVar4;
  char *__s;
  size_t sVar5;
  long *plVar6;
  undefined8 *puVar7;
  bool bVar8;
  
  puVar4 = (undefined8 *)FUN_0014ed80(0x60);
  if (param_1 == (undefined1 *)0x0) {
    param_1 = &DAT_0016a7d3;
  }
  __s = (char *)FUN_0014ee30(param_1);
  puVar4[2] = __s;
  sVar5 = strlen(__s);
  puVar7 = DAT_001826b8;
  bVar8 = DAT_00182a78 != '\0';
  puVar4[3] = sVar5;
  __ptr = DAT_001826c0;
  if (bVar8) {
    puVar7 = (undefined8 *)DAT_001826c0[1];
    plVar6 = (long *)*DAT_001826c0;
    plVar2 = plVar6;
    if (puVar7 != (undefined8 *)0x0) {
      *puVar7 = plVar6;
      plVar6 = (long *)*__ptr;
      plVar2 = DAT_001826c0;
    }
    DAT_001826c0 = plVar2;
    if (plVar6 != (long *)0x0) {
      plVar6[1] = (long)puVar7;
      puVar7 = DAT_001826b8;
    }
    DAT_001826b8 = puVar7;
    puVar7 = DAT_001826b8;
    free((void *)__ptr[2]);
    free((void *)__ptr[0xb]);
    free(__ptr);
  }
  puVar4[1] = 0;
  puVar1 = puVar4;
  *puVar4 = DAT_001826c0;
  DAT_001826c0 = puVar1;
  if (puVar7 == (undefined8 *)0x0) {
    DAT_001826b8 = puVar4;
  }
  uVar3 = DAT_0017f238 | DAT_001826f8;
  puVar4[6] = 0;
  uVar3 = uVar3 | DAT_00182ad4;
  bVar8 = DAT_001826fc == 2;
  *(undefined4 *)(puVar4 + 5) = 0;
  puVar4[7] = 0;
  puVar4[8] = 0;
  *(undefined1 *)((long)puVar4 + 0x24) = 1;
  *(uint *)(puVar4 + 4) = uVar3 | (uint)bVar8 << 0x1c;
  DAT_00182a78 = 1;
  return;
}




// Function: name_match @ 0x28880

bool name_match(long param_1)

{
  char cVar1;
  undefined8 *puVar2;
  char *pcVar3;
  
  while( true ) {
    if (DAT_001826c0 == (undefined8 *)0x0) {
      return true;
    }
    pcVar3 = (char *)DAT_001826c0[2];
    puVar2 = DAT_001826c0;
    if (*pcVar3 == '\0') {
      FUN_00126300(*(undefined4 *)(DAT_001826c0 + 5));
      DAT_001826c0 = (undefined8 *)0x0;
      DAT_001826b8 = 0;
      return true;
    }
    while (cVar1 = FUN_00140a20(pcVar3,param_1,*(undefined4 *)(puVar2 + 4)), cVar1 == '\0') {
      puVar2 = (undefined8 *)*puVar2;
      while( true ) {
        if (puVar2 == (undefined8 *)0x0) {
          if ((DAT_00182a78 != '\0') && (DAT_001826c0 == (undefined8 *)0x0)) {
            DAT_00182a78 = '\0';
          }
          goto LAB_001288df;
        }
        pcVar3 = (char *)puVar2[2];
        if (*pcVar3 != '\0') break;
        puVar2 = (undefined8 *)*puVar2;
      }
    }
    if (DAT_00182a78 == '\0') break;
    if (DAT_001826c0 == puVar2) {
      DAT_00182a78 = '\0';
      break;
    }
LAB_001288df:
    if (((DAT_00182aac == '\0') || (DAT_001826c0[6] == 0)) || (FUN_001282f0(), DAT_001826c0[6] != 0)
       ) {
      return false;
    }
  }
  if (((*(char *)(param_1 + puVar2[3]) != '/') || (DAT_00182ad4 == 0)) || (puVar2[6] == 0)) {
    puVar2[6] = puVar2[6] + 1;
  }
  FUN_00126300(*(undefined4 *)(puVar2 + 5));
  if (DAT_00182b38 == 0) {
    return puVar2[6] != 0;
  }
  return puVar2[6] == DAT_00182b38;
}




// Function: all_names_found @ 0x289e0

undefined8 all_names_found(long param_1)

{
  char *__s;
  undefined8 *puVar1;
  ulong uVar2;
  size_t sVar3;
  undefined8 uVar4;
  
  uVar2 = DAT_00182b38;
  __s = *(char **)(param_1 + 8);
  if (__s == (char *)0x0) {
    return 0;
  }
  uVar4 = 0;
  if (DAT_00182b38 != 0) {
    if (*(char *)(param_1 + 0x10) != '\0') {
      return 0;
    }
    sVar3 = strlen(__s);
    for (puVar1 = DAT_001826c0; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
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




// Function: names_notfound @ 0x28a70

void names_notfound(void)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  
  puVar2 = DAT_001826c0;
joined_r0x00128a96:
  do {
    if (puVar2 == (undefined8 *)0x0) {
      DAT_001826c0 = (undefined8 *)0x0;
      DAT_001826b8 = 0;
      if (DAT_00182aac != '\0') {
        while( true ) {
          lVar5 = FUN_00127340(1);
          if (lVar5 == 0) {
            return;
          }
          lVar5 = *(long *)(lVar5 + 0x18);
          if (lVar5 == 0) break;
          FUN_00126f30(lVar5);
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar3 = FUN_0014ce40(lVar5);
          uVar4 = dcgettext(0,"%s: Not found in archive",5);
          error(0,0,uVar4,uVar3);
          DAT_00182d58 = 2;
        }
      }
      return;
    }
    if (DAT_00182b38 == 0) {
      if (puVar2[6] == 0) {
        cVar1 = *(char *)puVar2[2];
joined_r0x00128ad2:
        if (cVar1 != '\0') {
          FUN_00126f30();
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar3 = FUN_0014ce40(puVar2[2]);
          if (puVar2[6] == 0) {
            uVar4 = dcgettext(0,"%s: Not found in archive",5);
          }
          else {
            uVar4 = dcgettext(0,"%s: Required occurrence not found in archive",5);
          }
          error(0,0,uVar4,uVar3);
          DAT_00182d58 = 2;
          puVar2 = (undefined8 *)*puVar2;
          goto joined_r0x00128a96;
        }
      }
    }
    else if ((ulong)puVar2[6] < DAT_00182b38) {
      cVar1 = *(char *)puVar2[2];
      goto joined_r0x00128ad2;
    }
    puVar2 = (undefined8 *)*puVar2;
  } while( true );
}




// Function: label_notfound @ 0x28c00

void label_notfound(void)

{
  int iVar1;
  long *plVar2;
  long lVar3;
  undefined8 uVar4;
  
  plVar2 = DAT_001826c0;
  if (DAT_001826c0 == (long *)0x0) {
    return;
  }
  do {
    if (DAT_00182b38 == 0) {
      if (plVar2[6] != 0) {
        return;
      }
    }
    else if (DAT_00182b38 <= (ulong)plVar2[6]) {
      return;
    }
    plVar2 = (long *)*plVar2;
  } while (plVar2 != (long *)0x0);
  if (DAT_00182a54 != 0) {
    uVar4 = dcgettext(0,"Archive label mismatch",5);
    error(0,0,uVar4);
  }
  FUN_0012fd00(1);
  plVar2 = DAT_001826c0;
  while ((plVar2 != (long *)0x0 && (iVar1 = FUN_00126f30(plVar2[2]), iVar1 == 0))) {
    plVar2 = (long *)*plVar2;
  }
  DAT_001826c0 = (long *)0x0;
  DAT_001826b8 = 0;
  if (DAT_00182aac != '\0') {
    do {
      lVar3 = FUN_00127340(1);
      if (lVar3 == 0) {
        return;
      }
      if (*(long *)(lVar3 + 0x18) == 0) {
        return;
      }
      iVar1 = FUN_00126f30();
    } while (iVar1 == 0);
  }
  return;
}




// Function: collect_and_sort_names @ 0x28cf0

void collect_and_sort_names(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  undefined8 *puVar6;
  int iVar7;
  int iVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  long lVar12;
  long *plVar13;
  long *plVar14;
  char *pcVar15;
  undefined8 *puVar16;
  undefined8 *puVar17;
  long in_FS_OFFSET;
  undefined8 local_1f8 [11];
  stat local_1a0 [2];
  int local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001282f0();
  if (DAT_001826c0 == (long *)0x0) {
    FUN_00128210(".",0,0);
    if (DAT_00182b28 != (long *)0x0) goto LAB_00128d37;
LAB_00128d60:
    if (DAT_001826c0 == (long *)0x0) {
      iVar7 = 0;
      goto LAB_00128e07;
    }
  }
  else if (DAT_00182b28 != (long *)0x0) {
LAB_00128d37:
    iVar7 = FUN_00126160();
    if (iVar7 != 0) {
      if (iVar7 != 1) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar15 = "Only one -C option is allowed with --listed-incremental";
LAB_00129189:
        uVar9 = dcgettext(0,pcVar15,5);
        error(0,0,uVar9);
                    /* WARNING: Subroutine does not return */
        FUN_0012dea0(2);
      }
      if ((int)DAT_001826c0[5] == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar15 = "Using -C option inside file list is not allowed with --listed-incremental";
        goto LAB_00129189;
      }
    }
    FUN_001203b0();
    goto LAB_00128d60;
  }
  plVar13 = DAT_001826c0;
  iVar8 = 0;
  do {
    while ((((iVar7 = iVar8, plVar13[6] != 0 || (plVar13[7] != 0)) ||
            ((*(byte *)((long)plVar13 + 0x23) & 0x10) != 0)) ||
           (FUN_00126300((int)plVar13[5]), *(char *)plVar13[2] == '\0'))) {
LAB_00128d80:
      plVar13 = (long *)*plVar13;
      iVar8 = iVar7 + 1;
      if (plVar13 == (long *)0x0) goto LAB_00128e00;
    }
    FUN_0012fb40(local_1f8);
    iVar8 = FUN_00125fc0(plVar13[2],local_1a0);
    if (iVar8 != 0) {
      FUN_00126460(plVar13[2]);
      goto LAB_00128d80;
    }
    if ((local_1a0[0].st_mode & 0xf000) == 0x4000) {
      iVar8 = __openat_2(DAT_0017f234,plVar13[2],DAT_00182808 | 0x10000);
      if (iVar8 < 0) {
        FUN_00126370(plVar13[2]);
      }
      else {
        local_50 = iVar8;
        iVar8 = fstat(iVar8,local_1a0);
        if (iVar8 == 0) {
          local_1f8[0] = FUN_0014ee30(plVar13[2]);
          plVar13[6] = plVar13[6] + 1;
          FUN_00128480(local_1f8,plVar13);
        }
        else {
          FUN_00126460();
        }
      }
    }
    FUN_0012fc00(local_1f8);
    plVar13 = (long *)*plVar13;
    iVar8 = iVar7 + 1;
  } while (plVar13 != (long *)0x0);
LAB_00128e00:
  iVar7 = iVar7 + 1;
LAB_00128e07:
  DAT_001826c0 = (long *)FUN_00126b90(DAT_001826c0,iVar7,FUN_00126d90);
  puVar17 = DAT_001826c0;
  puVar2 = (undefined8 *)0x0;
  while (puVar16 = puVar17, puVar16 != (undefined8 *)0x0) {
    puVar16[1] = puVar2;
    puVar2 = puVar16;
    puVar17 = (undefined8 *)*puVar16;
  }
  iVar7 = 0;
  uVar9 = FUN_00143bf0(0,0,FUN_00126fd0,FUN_00126d70,0);
  puVar17 = (undefined8 *)0x0;
  puVar2 = DAT_001826c0;
joined_r0x00128e6b:
  puVar16 = puVar17;
  if (puVar2 != (undefined8 *)0x0) {
    do {
      puVar17 = puVar2;
      puVar2 = (undefined8 *)*puVar17;
      uVar11 = FUN_00126700(*(undefined4 *)(puVar17 + 5),puVar17[2]);
      puVar17[0xb] = uVar11;
      if ((puVar16 != (undefined8 *)0x0) &&
         (puVar10 = (undefined8 *)FUN_00143960(uVar9,puVar17), puVar10 != (undefined8 *)0x0)) {
        if (puVar17[8] != 0) goto code_r0x00128ea2;
        if (puVar10[9] != 0) {
          FUN_00127040(puVar10[9],puVar17[2],puVar17[3]);
        }
        FUN_001442a0(uVar9,puVar17);
        puVar16 = (undefined8 *)puVar10[1];
        puVar1 = (undefined8 *)*puVar10;
        puVar6 = puVar1;
        if (puVar16 != (undefined8 *)0x0) {
          *puVar16 = puVar1;
          puVar6 = DAT_001826c0;
        }
        DAT_001826c0 = puVar6;
        if (puVar1 != (undefined8 *)0x0) {
          puVar1[1] = puVar16;
          puVar16 = DAT_001826b8;
        }
        DAT_001826b8 = puVar16;
        iVar7 = iVar7 + -1;
        free((void *)puVar10[2]);
        free((void *)puVar10[0xb]);
        free(puVar10);
      }
      puVar17[6] = 0;
      lVar12 = FUN_00144240(uVar9,puVar17);
      if (lVar12 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00117020();
      }
      iVar7 = iVar7 + 1;
      puVar16 = puVar17;
      if (puVar2 == (undefined8 *)0x0) break;
    } while( true );
  }
  DAT_001826b8 = puVar17;
  FUN_00143da0(uVar9);
  DAT_001826c0 = (long *)FUN_00126b90(DAT_001826c0,iVar7,FUN_00126fe0);
  if (DAT_001826c0 == (long *)0x0) {
    DAT_001826c0 = (long *)0x0;
  }
  else {
    plVar13 = (long *)0x0;
    plVar14 = DAT_001826c0;
    do {
      plVar14[1] = (long)plVar13;
      plVar3 = (long *)*plVar14;
      plVar13 = plVar14;
      plVar14 = plVar3;
      plVar4 = DAT_001826c0;
      plVar5 = DAT_00182b28;
    } while (plVar3 != (long *)0x0);
    while (plVar5 != (long *)0x0) {
      if (*(char *)plVar4[2] != '\0') {
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_0011ff30(plVar4[7]);
          return;
        }
        goto LAB_001291c4;
      }
      plVar4 = (long *)*plVar4;
      plVar5 = plVar4;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_001291c4:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
code_r0x00128ea2:
  if (puVar17[9] != 0) {
    FUN_00127040(puVar17[9],puVar10[2],puVar10[3]);
  }
  puVar10 = (undefined8 *)puVar17[1];
  puVar1 = (undefined8 *)*puVar17;
  puVar6 = puVar1;
  if (puVar10 != (undefined8 *)0x0) {
    *puVar10 = puVar1;
    puVar6 = DAT_001826c0;
  }
  DAT_001826c0 = puVar6;
  if (puVar1 != (undefined8 *)0x0) {
    puVar1[1] = puVar10;
    puVar10 = DAT_001826b8;
  }
  DAT_001826b8 = puVar10;
  free((void *)puVar17[2]);
  free((void *)puVar17[0xb]);
  free(puVar17);
  puVar17 = puVar16;
  goto joined_r0x00128e6b;
}




// Function: name_scan @ 0x291d0

long * name_scan(undefined8 param_1)

{
  char cVar1;
  long *plVar2;
  
  while( true ) {
    plVar2 = DAT_001826c0;
    if (DAT_001826c0 == (long *)0x0) {
      return (long *)0x0;
    }
    do {
      if ((*(char *)plVar2[2] != '\0') &&
         (cVar1 = FUN_00140a20((char *)plVar2[2],param_1,(int)plVar2[4]), cVar1 != '\0')) {
        return plVar2;
      }
      plVar2 = (long *)*plVar2;
    } while (plVar2 != (long *)0x0);
    if (DAT_00182aac == '\0') {
      return (long *)0x0;
    }
    if (DAT_001826c0 == (long *)0x0) break;
    if (DAT_001826c0[6] == 0) {
      return (long *)0x0;
    }
    FUN_001282f0();
    if (DAT_001826c0[6] != 0) {
      return (long *)0x0;
    }
  }
  return (long *)0x0;
}




// Function: name_from_list @ 0x29260

undefined8 * name_from_list(void)

{
  undefined8 *puVar1;
  
  puVar1 = DAT_001826c0;
  if (DAT_00182620 != (undefined8 *)0x0) goto LAB_00129270;
  while( true ) {
    DAT_00182620 = puVar1;
    if (DAT_00182620 == (undefined8 *)0x0) {
      return (undefined8 *)0x0;
    }
LAB_00129270:
    puVar1 = DAT_00182620;
    if ((DAT_00182620[6] == 0) && (*(char *)DAT_00182620[2] != '\0')) break;
    puVar1 = (undefined8 *)*DAT_00182620;
  }
  DAT_00182620[6] = 1;
  FUN_00126300(*(undefined4 *)(puVar1 + 5));
  return DAT_00182620;
}




// Function: blank_name_list @ 0x292e0

void blank_name_list(void)

{
  undefined8 *puVar1;
  
  DAT_00182620 = 0;
  for (puVar1 = DAT_001826c0; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    puVar1[6] = 0;
  }
  return;
}




// Function: make_file_name @ 0x29320

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
  __dest = (void *)FUN_0014ecc0(sVar1 + 1 + sVar2);
  memcpy(__dest,param_1,__n);
  *(undefined1 *)((long)__dest + __n) = 0x2f;
  memcpy((void *)((long)__dest + sVar2),param_2,sVar1 + 1);
  return __dest;
}




// Function: stripped_prefix_len @ 0x293b0

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




// Function: contains_dot_dot @ 0x29420

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




// Function: pax_sparse_member_p @ 0x29490

undefined4 pax_sparse_member_p(long param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 1;
  if (*(long *)(*(long *)(param_1 + 0x18) + 0x130) == 0) {
    iVar1 = *(int *)(*(long *)(param_1 + 0x18) + 0x124);
    uVar2 = CONCAT31((int3)((uint)iVar1 >> 8),iVar1 != 0);
  }
  return uVar2;
}




// Function: star_fixup_header @ 0x294c0

ulong star_fixup_header(long param_1)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  
  uVar2 = FUN_001231e0(DAT_00182408 + 0x1c4,0xc);
  lVar1 = *(long *)(param_1 + 0x18);
  *(undefined8 *)(lVar1 + 0x118) = *(undefined8 *)(lVar1 + 0x88);
  uVar3 = 0;
  if (-1 < (long)uVar2) {
    uVar3 = uVar2;
  }
  *(ulong *)(lVar1 + 0x88) = uVar3;
  return ~uVar2 >> 0x3f;
}




// Function: oldgnu_fixup_header @ 0x29510

ulong oldgnu_fixup_header(long param_1)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  
  uVar2 = FUN_001231e0(DAT_00182408 + 0x1e3,0xc);
  lVar1 = *(long *)(param_1 + 0x18);
  *(undefined8 *)(lVar1 + 0x118) = *(undefined8 *)(lVar1 + 0x88);
  uVar3 = 0;
  if (-1 < (long)uVar2) {
    uVar3 = uVar2;
  }
  *(ulong *)(lVar1 + 0x88) = uVar3;
  return ~uVar2 >> 0x3f;
}




// Function: sparse_add_map @ 0x29560

void sparse_add_map(long param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  ulong uVar2;
  undefined8 uVar3;
  long lVar4;
  ulong uVar5;
  long lVar6;
  
  lVar4 = *(long *)(param_1 + 0x140);
  uVar2 = *(ulong *)(param_1 + 0x130);
  uVar5 = *(ulong *)(param_1 + 0x138);
  if (uVar5 == uVar2) {
    if (lVar4 == 0) {
      if (uVar5 == 0) {
        lVar6 = 0x80;
        uVar5 = 8;
      }
      else {
        lVar6 = uVar5 << 4;
        if ((lVar6 < 0) || (uVar5 >> 0x3c != 0)) goto LAB_0012960f;
      }
    }
    else {
      if (0x555555555555554 < uVar5) {
LAB_0012960f:
                    /* WARNING: Subroutine does not return */
        FUN_00117020();
      }
      uVar5 = uVar5 + 1 + (uVar5 >> 1);
      lVar6 = uVar5 * 0x10;
    }
    *(ulong *)(param_1 + 0x138) = uVar5;
    lVar4 = FUN_0014ece0(lVar4,lVar6);
    *(long *)(param_1 + 0x140) = lVar4;
  }
  uVar3 = param_2[1];
  puVar1 = (undefined8 *)(lVar4 + uVar2 * 0x10);
  *puVar1 = *param_2;
  puVar1[1] = uVar3;
  *(ulong *)(param_1 + 0x130) = uVar2 + 1;
  return;
}




// Function: pax_dump_header @ 0x29630

undefined4 pax_dump_header(long param_1)

{
  long lVar1;
  int iVar2;
  char cVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char *pcVar7;
  size_t sVar8;
  size_t sVar9;
  undefined8 uVar10;
  char *pcVar11;
  char *pcVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  char *pcVar15;
  long lVar16;
  undefined8 *puVar17;
  long lVar18;
  ulong uVar19;
  undefined8 *puVar20;
  long in_FS_OFFSET;
  ulong local_60;
  undefined1 local_58 [24];
  long local_40;
  
  iVar2 = DAT_00182a84;
  lVar18 = *(long *)(param_1 + 0x18);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  *(int *)(lVar18 + 0x128) = DAT_00182a80;
  *(int *)(lVar18 + 0x124) = iVar2;
  if (iVar2 == 0) {
    uVar5 = FUN_0010cc60();
    lVar18 = *(long *)(*(long *)(param_1 + 0x18) + 0x140);
    FUN_0011da70("GNU.sparse.size",*(long *)(param_1 + 0x18),0);
    FUN_0011da70("GNU.sparse.numblocks",*(undefined8 *)(param_1 + 0x18),0);
    cVar3 = FUN_0011c9c0("GNU.sparse.map");
    if ((cVar3 == '\0') && (DAT_00182a80 != 0)) {
      FUN_0011da70("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
      lVar16 = *(long *)(param_1 + 0x18);
      lVar1 = *(long *)(lVar16 + 8);
      uVar13 = FUN_0011cdf0(lVar16,"%d/GNUSparseFile.%p/%f",0);
      *(undefined8 *)(lVar16 + 8) = uVar13;
      FUN_0011de70(*(long *)(param_1 + 0x18) + 0x168);
      lVar16 = *(long *)(param_1 + 0x18);
      local_60 = 0;
      if (*(long *)(lVar16 + 0x130) != 0) {
        while( true ) {
          uVar13 = FUN_00145370(*(undefined8 *)(lVar18 + local_60 * 0x10),local_58);
          FUN_0011de80(*(long *)(param_1 + 0x18) + 0x168,uVar13);
          FUN_0011de80(*(long *)(param_1 + 0x18) + 0x168,&DAT_0016379c);
          uVar13 = FUN_00145370(*(undefined8 *)(lVar18 + 8 + local_60 * 0x10),local_58);
          FUN_0011de80(*(long *)(param_1 + 0x18) + 0x168,uVar13);
          lVar16 = *(long *)(param_1 + 0x18);
          local_60 = local_60 + 1;
          if (*(ulong *)(lVar16 + 0x130) <= local_60) break;
          if (local_60 != 0) {
            FUN_0011de80(lVar16 + 0x168,&DAT_0016379c);
          }
        }
      }
      uVar4 = FUN_0011df00(lVar16 + 0x168,"GNU.sparse.map");
      if ((char)uVar4 == '\0') {
        free(*(void **)(*(long *)(param_1 + 0x18) + 8));
        *(long *)(*(long *)(param_1 + 0x18) + 8) = lVar1;
      }
      else {
        lVar18 = *(long *)(param_1 + 0x18);
        uVar13 = *(undefined8 *)(lVar18 + 0x88);
        *(undefined8 *)(lVar18 + 0x88) = *(undefined8 *)(lVar18 + 0x118);
        uVar14 = FUN_00113570(lVar18);
        *(undefined8 *)(lVar18 + 0x88) = uVar13;
        FUN_00113320(*(undefined8 *)(param_1 + 0x18),uVar14,uVar5);
        if (lVar1 != 0) {
          free(*(void **)(*(long *)(param_1 + 0x18) + 8));
          *(long *)(*(long *)(param_1 + 0x18) + 8) = lVar1;
        }
      }
    }
    else {
      local_60 = 0;
      lVar18 = *(long *)(param_1 + 0x18);
      if (*(long *)(lVar18 + 0x130) != 0) {
        do {
          FUN_0011da70("GNU.sparse.offset",lVar18,&local_60);
          FUN_0011da70("GNU.sparse.numbytes",*(undefined8 *)(param_1 + 0x18),&local_60);
          lVar18 = *(long *)(param_1 + 0x18);
          local_60 = local_60 + 1;
        } while (local_60 < *(ulong *)(lVar18 + 0x130));
      }
      uVar13 = *(undefined8 *)(lVar18 + 0x88);
      uVar4 = 1;
      *(undefined8 *)(lVar18 + 0x88) = *(undefined8 *)(lVar18 + 0x118);
      uVar6 = FUN_00113570(lVar18);
      uVar14 = *(undefined8 *)(param_1 + 0x18);
      *(undefined8 *)(lVar18 + 0x88) = uVar13;
      FUN_00113320(uVar14,uVar6,uVar5);
    }
  }
  else {
    uVar13 = FUN_0010cc60();
    lVar18 = *(long *)(param_1 + 0x18);
    uVar5 = *(undefined8 *)(lVar18 + 8);
    puVar17 = *(undefined8 **)(lVar18 + 0x140);
    pcVar7 = (char *)FUN_00145370(*(undefined8 *)(lVar18 + 0x130));
    sVar8 = strlen(pcVar7);
    lVar16 = *(long *)(param_1 + 0x18);
    lVar18 = sVar8 + 1;
    if (*(long *)(lVar16 + 0x130) != 0) {
      uVar19 = 0;
      puVar20 = puVar17;
      do {
        uVar19 = uVar19 + 1;
        pcVar7 = (char *)FUN_00145370(*puVar20,local_58);
        sVar8 = strlen(pcVar7);
        pcVar7 = (char *)FUN_00145370(puVar20[1],local_58);
        sVar9 = strlen(pcVar7);
        lVar16 = *(long *)(param_1 + 0x18);
        lVar18 = sVar8 + 2 + lVar18 + sVar9;
        puVar20 = puVar20 + 2;
      } while (uVar19 < *(ulong *)(lVar16 + 0x130));
    }
    uVar19 = lVar18 + 0x3fe;
    if (-1 < (long)(lVar18 + 0x1ffU)) {
      uVar19 = lVar18 + 0x1ffU;
    }
    *(long *)(lVar16 + 0x118) = *(long *)(lVar16 + 0x118) + (uVar19 & 0xfffffffffffffe00);
    *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + (uVar19 & 0xfffffffffffffe00);
    FUN_0011da70("GNU.sparse.major",lVar16,0);
    FUN_0011da70("GNU.sparse.minor",*(undefined8 *)(param_1 + 0x18),0);
    FUN_0011da70("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
    FUN_0011da70("GNU.sparse.realsize",*(undefined8 *)(param_1 + 0x18),0);
    lVar18 = *(long *)(param_1 + 0x18);
    uVar14 = FUN_0011cdf0(lVar18,"%d/GNUSparseFile.%p/%f",0);
    *(undefined8 *)(lVar18 + 8) = uVar14;
    lVar18 = *(long *)(param_1 + 0x18);
    pcVar7 = *(char **)(lVar18 + 8);
    sVar8 = strlen(pcVar7);
    if (100 < sVar8) {
      pcVar7[100] = '\0';
      lVar18 = *(long *)(param_1 + 0x18);
    }
    uVar14 = *(undefined8 *)(lVar18 + 0x88);
    *(undefined8 *)(lVar18 + 0x88) = *(undefined8 *)(lVar18 + 0x118);
    uVar10 = FUN_00113570(lVar18);
    uVar6 = *(undefined8 *)(param_1 + 0x18);
    *(undefined8 *)(lVar18 + 0x88) = uVar14;
    FUN_00113320(uVar6,uVar10,uVar13);
    free(*(void **)(*(long *)(param_1 + 0x18) + 8));
    *(undefined8 *)(*(long *)(param_1 + 0x18) + 8) = uVar5;
    pcVar11 = (char *)FUN_0010d360();
    pcVar12 = (char *)FUN_00145370(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130));
    pcVar15 = pcVar11 + 0x200;
    cVar3 = *pcVar12;
    pcVar7 = pcVar11;
    while (cVar3 != '\0') {
      if (pcVar15 == pcVar7) {
        FUN_0010ccd0(pcVar11);
        pcVar7 = (char *)FUN_0010d360();
        pcVar15 = pcVar7 + 0x200;
        pcVar11 = pcVar7;
      }
      cVar3 = *pcVar12;
      pcVar12 = pcVar12 + 1;
      *pcVar7 = cVar3;
      pcVar7 = pcVar7 + 1;
      cVar3 = *pcVar12;
    }
    if (pcVar15 == pcVar7) {
      FUN_0010ccd0(pcVar11);
      pcVar7 = (char *)FUN_0010d360();
      pcVar11 = pcVar7;
    }
    *pcVar7 = '\n';
    pcVar7 = pcVar7 + 1;
    if (*(long *)(*(long *)(param_1 + 0x18) + 0x130) != 0) {
      uVar19 = 0;
      do {
        pcVar12 = (char *)FUN_00145370(*puVar17,local_58);
        pcVar15 = pcVar11 + 0x200;
        cVar3 = *pcVar12;
        while (cVar3 != '\0') {
          if (pcVar7 == pcVar15) {
            FUN_0010ccd0(pcVar11);
            pcVar7 = (char *)FUN_0010d360();
            pcVar15 = pcVar7 + 0x200;
            pcVar11 = pcVar7;
          }
          cVar3 = *pcVar12;
          pcVar12 = pcVar12 + 1;
          *pcVar7 = cVar3;
          pcVar7 = pcVar7 + 1;
          cVar3 = *pcVar12;
        }
        if (pcVar15 == pcVar7) {
          FUN_0010ccd0(pcVar11);
          pcVar7 = (char *)FUN_0010d360();
          pcVar15 = pcVar7 + 0x200;
          pcVar11 = pcVar7;
        }
        *pcVar7 = '\n';
        pcVar12 = (char *)FUN_00145370(puVar17[1]);
        cVar3 = *pcVar12;
        while (pcVar7 = pcVar7 + 1, cVar3 != '\0') {
          if (pcVar7 == pcVar15) {
            FUN_0010ccd0(pcVar11);
            pcVar7 = (char *)FUN_0010d360();
            pcVar15 = pcVar7 + 0x200;
            pcVar11 = pcVar7;
          }
          cVar3 = *pcVar12;
          pcVar12 = pcVar12 + 1;
          *pcVar7 = cVar3;
          cVar3 = *pcVar12;
        }
        if (pcVar15 == pcVar7) {
          FUN_0010ccd0(pcVar11);
          pcVar7 = (char *)FUN_0010d360();
          pcVar11 = pcVar7;
        }
        *pcVar7 = '\n';
        pcVar7 = pcVar7 + 1;
        uVar19 = uVar19 + 1;
        puVar17 = puVar17 + 2;
      } while (uVar19 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130));
    }
    uVar4 = 1;
    memset(pcVar7,0,(size_t)(pcVar11 + (0x200 - (long)pcVar7)));
    FUN_0010ccd0(pcVar11);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: star_get_sparse_info @ 0x29e30

undefined8 star_get_sparse_info(long param_1)

{
  char cVar1;
  int iVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long lVar6;
  ulong uVar7;
  
  lVar3 = DAT_00182408;
  cVar1 = *(char *)(DAT_00182408 + 0x159);
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  if ((cVar1 == '\0') && (*(char *)(lVar3 + 0x16e) != '\0')) {
    lVar6 = lVar3 + 0x164;
    do {
      if (*(char *)(lVar6 + 0xc) == '\0') {
        return 1;
      }
      iVar2 = FUN_00129d90(param_1,lVar6);
      if (iVar2 != 0) goto LAB_00129ee9;
      lVar6 = lVar6 + 0x18;
    } while (lVar3 + 0x1c4 != lVar6);
    if (*(char *)(lVar3 + 0x163) == '\0') {
      return 1;
    }
  }
  do {
    lVar3 = FUN_0010d360();
    if (lVar3 == 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar5 = dcgettext(0,"Unexpected EOF in archive",5);
      error(0,0,uVar5);
      DAT_00182d58 = 2;
      return 0;
    }
    uVar7 = 0;
    FUN_0010ccd0(lVar3);
    lVar6 = lVar3;
    do {
      if (*(char *)(lVar6 + 0xc) == '\0') {
        *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + 0x200;
        return 1;
      }
      uVar7 = uVar7 + 1;
      iVar2 = FUN_00129d90(param_1,lVar6);
      lVar6 = lVar6 + 0x18;
    } while ((uVar7 < 0x15) && (iVar2 == 0));
    *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + 0x200;
  } while ((*(char *)(lVar3 + 0x1f8) != '\0') && (iVar2 == 0));
LAB_00129ee9:
  if (iVar2 != 2) {
    return 1;
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar5 = **(undefined8 **)(param_1 + 0x18);
  uVar4 = dcgettext(0,"%s: invalid sparse archive member",5);
  error(0,0,uVar4,uVar5);
  DAT_00182d58 = 2;
  return 0;
}




// Function: oldgnu_get_sparse_info @ 0x29fd0

undefined8 oldgnu_get_sparse_info(long param_1)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  long lVar6;
  ulong uVar7;
  
  lVar1 = DAT_00182408;
  lVar5 = DAT_00182408 + 0x182;
  lVar6 = DAT_00182408 + 0x1e2;
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  do {
    if (*(char *)(lVar5 + 0xc) == '\0') {
      return 1;
    }
    iVar2 = FUN_00129d90(param_1,lVar5);
    if (iVar2 != 0) goto LAB_0012a096;
    lVar5 = lVar5 + 0x18;
  } while (lVar5 != lVar6);
  if (*(char *)(lVar1 + 0x1e2) != '\0') {
    do {
      lVar5 = FUN_0010d360();
      if (lVar5 == 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar4 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar4);
        DAT_00182d58 = 2;
        return 0;
      }
      uVar7 = 0;
      FUN_0010ccd0(lVar5);
      lVar6 = lVar5;
      do {
        if (*(char *)(lVar6 + 0xc) == '\0') {
          return 1;
        }
        uVar7 = uVar7 + 1;
        iVar2 = FUN_00129d90(param_1,lVar6);
        lVar6 = lVar6 + 0x18;
      } while ((uVar7 < 0x15) && (iVar2 == 0));
    } while ((*(char *)(lVar5 + 0x1f8) != '\0') && (iVar2 == 0));
LAB_0012a096:
    if (iVar2 == 2) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar4 = **(undefined8 **)(param_1 + 0x18);
      uVar3 = dcgettext(0,"%s: invalid sparse archive member",5);
      error(0,0,uVar3,uVar4);
      DAT_00182d58 = 2;
      return 0;
    }
  }
  return 1;
}




// Function: sparse_extract_region @ 0x2a650

uint sparse_extract_region(undefined4 *param_1,long param_2)

{
  byte bVar1;
  uint uVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  
  uVar2 = FUN_0012a5b0(param_1,*(undefined8 *)
                                (*(long *)(*(long *)(param_1 + 6) + 0x140) + param_2 * 0x10));
  if ((char)uVar2 != '\0') {
    lVar7 = *(long *)(*(long *)(*(long *)(param_1 + 6) + 0x140) + 8 + param_2 * 0x10);
    if (lVar7 == 0) {
      bVar1 = *(byte *)(param_1 + 1);
      if ((bVar1 != 0) && (iVar3 = FUN_0012c110(*param_1), iVar3 != 0)) {
        uVar2 = (uint)bVar1;
        FUN_00133470(**(undefined8 **)(param_1 + 6));
      }
    }
    else {
      while (0 < lVar7) {
        lVar6 = 0x200;
        if (lVar7 < 0x201) {
          lVar6 = lVar7;
        }
        lVar4 = FUN_0010d360();
        if (lVar4 == 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar5 = dcgettext(0,"Unexpected EOF in archive",5);
          error(0,0,uVar5);
          DAT_00182d58 = 2;
          return 0;
        }
        FUN_0010ccd0(lVar4);
        *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
        lVar4 = FUN_00126070(*param_1,lVar4,lVar6);
        lVar7 = lVar7 - lVar4;
        FUN_0010c710();
        *(long *)(param_1 + 2) = *(long *)(param_1 + 2) + lVar4;
        if (lVar6 != lVar4) {
          FUN_00133510(**(undefined8 **)(param_1 + 6),lVar4,lVar6);
          return 0;
        }
      }
    }
  }
  return uVar2;
}




// Function: sparse_dump_region @ 0x2a7c0

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
  local_f1 = FUN_0012a5b0(param_1,*puVar8);
  if (local_f1 != '\0') {
    for (; 0 < lVar7; lVar7 = lVar7 - lVar3) {
      lVar9 = 0x200;
      if (lVar7 < 0x201) {
        lVar9 = lVar7;
      }
      lVar2 = FUN_0010d360();
      lVar3 = FUN_0014d570(*param_1,lVar2,lVar9);
      if (lVar3 == -1) {
        plVar10 = (long *)(param_2 + (*(undefined8 **)(param_1 + 6))[0x28]);
        FUN_001263a0(**(undefined8 **)(param_1 + 6),(plVar10[1] + *plVar10) - lVar7,lVar9);
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
        if (((byte)DAT_00181b80 & 0x80) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar4 = FUN_00145370(lVar7,local_58);
          uVar5 = FUN_0014ce40(**(undefined8 **)(param_1 + 6));
          uVar6 = dcngettext(0,"%s: File shrank by %s byte; padding with zeros",
                             "%s: File shrank by %s bytes; padding with zeros",lVar7,5);
          error(0,0,uVar6,uVar5,uVar4);
        }
        if (DAT_00182b52 == '\0') {
          FUN_0012fd00(1);
        }
        local_f1 = '\0';
        break;
      }
      memset((void *)(lVar2 + lVar3),0,0x200 - lVar3);
      *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + lVar3;
      FUN_0010ccd0(lVar2);
    }
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_f1;
}




// Function: check_sparse_region @ 0x2a9d0

undefined4 check_sparse_region(undefined4 *param_1,long param_2,long param_3)

{
  undefined4 uVar1;
  long lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
  long lVar7;
  long in_FS_OFFSET;
  undefined1 auStack_268 [32];
  char local_248 [520];
  long local_40;
  char *pcVar6;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0012a5b0();
  if ((char)uVar1 != '\0') {
    for (; param_2 < param_3; param_2 = param_2 + lVar2) {
      lVar7 = param_3 - param_2;
      if (0x200 < lVar7) {
        lVar7 = 0x200;
      }
      lVar2 = FUN_0014d570(*param_1,local_248,lVar7);
      if (lVar2 == -1) {
        FUN_001263a0(**(undefined8 **)(param_1 + 6),param_2,lVar7);
LAB_0012aaae:
        uVar1 = 0;
        break;
      }
      if (lVar2 == 0) {
        uVar3 = dcgettext(0,"Size differs",5);
        FUN_00111120(*(undefined8 *)(param_1 + 6),uVar3);
        goto LAB_0012aaae;
      }
      pcVar6 = local_248;
      do {
        pcVar5 = pcVar6 + 1;
        if (*pcVar6 != '\0') {
          uVar3 = FUN_001452d0(param_2,auStack_268);
          uVar4 = dcgettext(0,"File fragment at %s is not a hole",5);
          FUN_00111120(*(undefined8 *)(param_1 + 6),uVar4,uVar3);
          goto LAB_0012aaae;
        }
        pcVar6 = pcVar5;
      } while (local_248 + lVar2 != pcVar5);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_data_region @ 0x2ab20

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
  local_251 = FUN_0012a5b0(param_1,*(undefined8 *)
                                    (*(long *)(*(long *)(param_1 + 6) + 0x140) + param_2));
  if (local_251 != '\0') {
    lVar3 = *(long *)(*(long *)(*(long *)(param_1 + 6) + 0x140) + 8 + param_2);
    FUN_0010c710(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
    do {
      if (lVar3 < 1) goto LAB_0012ab6f;
      lVar4 = 0x200;
      if (lVar3 < 0x201) {
        lVar4 = lVar3;
      }
      __s1 = (void *)FUN_0010d360();
      if (__s1 == (void *)0x0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar2 = dcgettext(0,"Unexpected EOF in archive",5);
        error(0,0,uVar2);
        DAT_00182d58 = 2;
        goto LAB_0012ac90;
      }
      FUN_0010ccd0(__s1);
      *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
      __n = FUN_0014d570(*param_1,local_248,lVar4);
      if (__n == 0xffffffffffffffff) {
        plVar5 = (long *)(param_2 + (*(undefined8 **)(param_1 + 6))[0x28]);
        FUN_001263a0(**(undefined8 **)(param_1 + 6),(plVar5[1] + *plVar5) - lVar3,lVar4);
        goto LAB_0012ac90;
      }
      if (__n == 0) {
        uVar2 = dcgettext(0,"Size differs",5);
        FUN_00111120(&DAT_00182840,uVar2);
        goto LAB_0012ac90;
      }
      lVar3 = lVar3 - __n;
      FUN_0010c710(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
      iVar1 = memcmp(__s1,local_248,__n);
    } while (iVar1 == 0);
    uVar2 = dcgettext(0,"Contents differ",5);
    FUN_00111120(*(undefined8 *)(param_1 + 6),uVar2);
LAB_0012ac90:
    local_251 = '\0';
  }
LAB_0012ab6f:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_251;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: oldgnu_dump_header @ 0x2ad50

undefined8 oldgnu_dump_header(long param_1)

{
  undefined8 uVar1;
  long lVar2;
  undefined8 *puVar3;
  long lVar4;
  ulong uVar5;
  long lVar6;
  undefined8 *puVar7;
  ulong uVar8;
  long lVar9;
  bool bVar10;
  byte bVar11;
  
  bVar11 = 0;
  uVar1 = FUN_0010cc60();
  lVar2 = FUN_00113570(*(undefined8 *)(param_1 + 0x18));
  *(undefined1 *)(lVar2 + 0x9c) = 0x53;
  lVar4 = *(long *)(param_1 + 0x18);
  if (4 < *(ulong *)(lVar4 + 0x130)) {
    *(undefined1 *)(lVar2 + 0x1e2) = 1;
  }
  lVar6 = lVar2 + 0x182;
  FUN_00112f80(*(undefined8 *)(lVar4 + 0x88),lVar2 + 0x1e3,0xc);
  FUN_00112f80(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x118),lVar2 + 0x7c,0xc);
  lVar4 = *(long *)(param_1 + 0x18);
  if (*(long *)(lVar4 + 0x130) == 0) {
    bVar10 = false;
    uVar8 = 0;
  }
  else {
    uVar8 = 0;
    do {
      lVar9 = uVar8 * 0x10;
      uVar8 = uVar8 + 1;
      FUN_00112f80(*(undefined8 *)(*(long *)(lVar4 + 0x140) + lVar9),lVar6,0xc);
      lVar4 = lVar6 + 0xc;
      lVar6 = lVar6 + 0x18;
      FUN_00112f80(*(undefined8 *)(*(long *)(*(long *)(param_1 + 0x18) + 0x140) + 8 + lVar9),lVar4,
                   0xc);
      lVar4 = *(long *)(param_1 + 0x18);
      bVar10 = uVar8 < *(ulong *)(lVar4 + 0x130);
      if (uVar8 == 4) break;
    } while (bVar10);
  }
  *(bool *)(lVar2 + 0x1e2) = bVar10;
  FUN_00113320(lVar4,lVar2,uVar1);
  if (uVar8 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
    do {
      puVar3 = (undefined8 *)FUN_0010d360();
      *puVar3 = 0;
      puVar3[0x3f] = 0;
      puVar7 = (undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8);
      for (uVar5 = (ulong)(((int)puVar3 -
                           (int)(undefined8 *)((ulong)(puVar3 + 1) & 0xfffffffffffffff8)) + 0x200U
                          >> 3); uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar7 = 0;
        puVar7 = puVar7 + (ulong)bVar11 * -2 + 1;
      }
      lVar4 = *(long *)(param_1 + 0x18);
      if (uVar8 < *(ulong *)(lVar4 + 0x130)) {
        puVar7 = puVar3;
        do {
          lVar2 = uVar8 * 0x10;
          uVar8 = uVar8 + 1;
          FUN_00112f80(*(undefined8 *)(*(long *)(lVar4 + 0x140) + lVar2),puVar7,0xc);
          lVar4 = (long)puVar7 + 0xc;
          puVar7 = puVar7 + 3;
          FUN_00112f80(*(undefined8 *)(*(long *)(*(long *)(param_1 + 0x18) + 0x140) + 8 + lVar2),
                       lVar4,0xc);
          lVar4 = *(long *)(param_1 + 0x18);
          if (*(ulong *)(lVar4 + 0x130) <= uVar8) goto LAB_0012af2d;
        } while (puVar3 + 0x3f != puVar7);
        *(undefined1 *)(puVar3 + 0x3f) = 1;
      }
LAB_0012af2d:
      FUN_0010ccd0(puVar3);
    } while (uVar8 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130));
  }
  return 1;
}




// Function: sparse_dump_file @ 0x2af80

byte sparse_dump_file(int param_1,long param_2)

{
  int iVar1;
  byte bVar2;
  char cVar3;
  code *pcVar4;
  long lVar5;
  char *pcVar6;
  __off_t _Var8;
  __off_t __offset;
  int *piVar9;
  long lVar10;
  ulong uVar11;
  long lVar12;
  __off_t __offset_00;
  long in_FS_OFFSET;
  byte local_291;
  long local_288;
  long local_280;
  int local_278;
  undefined1 local_274;
  long local_268;
  long local_260;
  long local_258;
  char local_248 [520];
  long local_40;
  char *pcVar7;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_291 = FUN_00129d10(&local_278);
  bVar2 = 3;
  if (local_291 == 0) goto LAB_0012b035;
  local_274 = 1;
  local_280 = 0;
  local_278 = param_1;
  local_260 = param_2;
  if (*(long *)(param_2 + 0x98) == 0) {
    *(undefined8 *)(param_2 + 0x118) = 0;
LAB_0012b2ea:
    local_280 = 0;
    local_288 = *(long *)(param_2 + 0x88);
    FUN_00129560(param_2,&local_288);
LAB_0012b280:
    if (*(long *)(local_258 + 0x38) != 0) {
      if (*(code **)(local_258 + 0x18) != (code *)0x0) {
        (**(code **)(local_258 + 0x18))(&local_278);
      }
      if (-1 < param_1) {
        FUN_0010c640(*(undefined8 *)(local_260 + 8),*(undefined8 *)(local_260 + 0x88),
                     *(long *)(local_260 + 0x118) - local_268);
        uVar11 = 0;
        do {
          if (*(ulong *)(local_260 + 0x130) <= uVar11) goto LAB_0012b2aa;
          if (*(code **)(local_258 + 0x38) == (code *)0x0) break;
          cVar3 = (**(code **)(local_258 + 0x38))(&local_278,uVar11);
          uVar11 = uVar11 + 1;
        } while (cVar3 != '\0');
        goto LAB_0012b002;
      }
    }
LAB_0012b2aa:
    FUN_001147d0(*(long *)(local_260 + 0x118) - local_268);
    pcVar4 = *(code **)(local_258 + 8);
    bVar2 = 0;
    if (pcVar4 == (code *)0x0) goto LAB_0012b035;
  }
  else {
    if (DAT_00182a7c == 1) {
LAB_0012b069:
      lVar10 = local_260;
      iVar1 = local_278;
      local_288 = 0;
      local_280 = 0;
      *(undefined8 *)(local_260 + 0x118) = 0;
      if ((*(code **)(local_258 + 0x30) == (code *)0x0) ||
         (cVar3 = (**(code **)(local_258 + 0x30))(&local_278,0,0), cVar3 != '\0')) {
        lVar12 = 0;
        while ((lVar5 = FUN_00125fe0(iVar1,local_248,0x200), lVar5 != 0 && (lVar5 != -1))) {
          pcVar7 = local_248;
          do {
            pcVar6 = pcVar7 + 1;
            if (*pcVar7 != '\0') {
              if (local_280 == 0) {
                local_288 = lVar12;
              }
              local_280 = local_280 + lVar5;
              *(long *)(lVar10 + 0x118) = *(long *)(lVar10 + 0x118) + lVar5;
              pcVar4 = *(code **)(local_258 + 0x30);
              pcVar7 = local_248;
              goto joined_r0x0012b1ea;
            }
            pcVar7 = pcVar6;
          } while (pcVar6 != local_248 + lVar5);
          if (local_280 != 0) {
            FUN_00129560(lVar10,&local_288);
            pcVar7 = (char *)0x0;
            local_280 = 0;
            pcVar4 = *(code **)(local_258 + 0x30);
joined_r0x0012b1ea:
            if ((pcVar4 != (code *)0x0) && (cVar3 = (*pcVar4)(&local_278,1,pcVar7), cVar3 == '\0'))
            goto LAB_0012b002;
          }
          lVar12 = lVar12 + lVar5;
        }
        if (local_280 == 0) {
          local_288 = lVar12;
        }
        FUN_00129560(lVar10,&local_288);
        *(long *)(lVar10 + 0x118) = *(long *)(lVar10 + 0x118) + lVar5;
        if ((*(code **)(local_258 + 0x30) == (code *)0x0) ||
           (cVar3 = (**(code **)(local_258 + 0x30))(&local_278,2,0), cVar3 != '\0'))
        goto LAB_0012b280;
      }
    }
    else if ((DAT_00182a7c & 0xfffffffd) == 0) {
      lVar10 = 0;
      local_288 = 0;
      __offset_00 = 0;
      while( true ) {
        *(long *)(param_2 + 0x118) = lVar10;
        __offset = lseek(param_1,__offset_00,3);
        if (__offset == -1) break;
        _Var8 = lseek(param_1,__offset,4);
        if ((__offset_00 == 0 && __offset == 0) && (_Var8 == *(long *)(param_2 + 0x88))) {
          lseek(param_1,0,0);
          goto LAB_0012b069;
        }
        local_280 = _Var8 - __offset;
        local_288 = __offset;
        FUN_00129560(param_2,&local_288);
        lVar10 = local_280 + *(long *)(param_2 + 0x118);
        __offset_00 = _Var8;
      }
      piVar9 = __errno_location();
      if (*piVar9 == 6) goto LAB_0012b2ea;
      goto LAB_0012b069;
    }
LAB_0012b002:
    FUN_001147d0(*(long *)(local_260 + 0x118) - local_268);
    local_291 = 0;
    pcVar4 = *(code **)(local_258 + 8);
    if (pcVar4 == (code *)0x0) {
      bVar2 = 1;
      goto LAB_0012b035;
    }
  }
  bVar2 = (*pcVar4)(&local_278);
  bVar2 = bVar2 & local_291 ^ 1;
LAB_0012b035:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_member_p @ 0x2b3a0

undefined8 sparse_member_p(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 extraout_RDX;
  long in_FS_OFFSET;
  undefined1 auStack_48 [24];
  undefined8 local_30;
  long local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00129d10(auStack_48,param_2,param_1);
  if (cVar1 != '\0') {
    local_30 = extraout_RDX;
    if (*(code **)(local_28 + 0x10) != (code *)0x0) {
      uVar2 = (**(code **)(local_28 + 0x10))();
      goto LAB_0012b3e2;
    }
  }
  uVar2 = 0;
LAB_0012b3e2:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_fixup_header @ 0x2b400

void sparse_fixup_header(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  undefined8 extraout_RDX;
  long in_FS_OFFSET;
  undefined1 auStack_48 [24];
  undefined8 local_30;
  long local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_00129d10(auStack_48,param_2,param_1);
  if (cVar1 != '\0') {
    local_30 = extraout_RDX;
    if (*(code **)(local_28 + 0x20) != (code *)0x0) {
      (**(code **)(local_28 + 0x20))();
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_extract_file @ 0x2b460

byte sparse_extract_file(int param_1,long param_2)

{
  byte bVar1;
  char cVar2;
  byte bVar3;
  __off_t _Var4;
  code *pcVar5;
  long *extraout_RDX;
  ulong uVar6;
  ulong uVar7;
  long in_FS_OFFSET;
  int local_68;
  undefined1 local_64;
  undefined8 local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = FUN_00129d10(&local_68);
  bVar3 = 3;
  if (bVar1 != 0) {
    local_68 = param_1;
    local_50 = param_2;
    _Var4 = lseek(param_1,0,0);
    local_60 = 0;
    local_64 = _Var4 == 0;
    if ((*(code **)(local_48 + 0x28) == (code *)0x0) ||
       (cVar2 = (**(code **)(local_48 + 0x28))(&local_68), cVar2 != '\0')) {
      uVar6 = 0;
      if (*(long *)(local_50 + 0x130) != 0) {
        do {
          if (*(code **)(local_48 + 0x40) == (code *)0x0) goto LAB_0012b580;
          uVar7 = uVar6 + 1;
          cVar2 = (**(code **)(local_48 + 0x40))(&local_68,uVar6);
          if (cVar2 == '\0') goto LAB_0012b580;
          uVar6 = uVar7;
        } while (uVar7 < *(ulong *)(local_50 + 0x130));
      }
      pcVar5 = *(code **)(local_48 + 8);
      *extraout_RDX = *(long *)(local_50 + 0x118) - local_58;
      bVar3 = 0;
      if (pcVar5 == (code *)0x0) goto LAB_0012b553;
    }
    else {
LAB_0012b580:
      pcVar5 = *(code **)(local_48 + 8);
      bVar1 = 0;
      *extraout_RDX = *(long *)(local_50 + 0x118) - local_58;
      if (pcVar5 == (code *)0x0) {
        bVar3 = 1;
        goto LAB_0012b553;
      }
    }
    bVar3 = (*pcVar5)(&local_68);
    bVar3 = bVar3 & bVar1 ^ 1;
  }
LAB_0012b553:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar3;
}




// Function: sparse_skip_file @ 0x2b5b0

byte sparse_skip_file(undefined8 param_1,undefined8 param_2)

{
  byte bVar1;
  char cVar2;
  byte bVar3;
  code *pcVar4;
  long extraout_RDX;
  long in_FS_OFFSET;
  undefined4 local_58 [4];
  long local_48;
  long local_40;
  long local_38;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = FUN_00129d10(local_58,param_2,param_1);
  bVar3 = 3;
  if (bVar1 == 0) goto LAB_0012b63f;
  local_58[0] = 0xffffffff;
  local_40 = extraout_RDX;
  if (*(code **)(local_38 + 0x28) == (code *)0x0) {
    FUN_00123d40(*(long *)(extraout_RDX + 0x118) - local_48);
    pcVar4 = *(code **)(local_38 + 8);
    if (pcVar4 == (code *)0x0) {
      bVar3 = 0;
      goto LAB_0012b63f;
    }
LAB_0012b625:
    cVar2 = (*pcVar4)(local_58);
    bVar3 = 1;
    if (cVar2 == '\0') goto LAB_0012b63f;
  }
  else {
    bVar1 = (**(code **)(local_38 + 0x28))();
    FUN_00123d40(*(long *)(local_40 + 0x118) - local_48);
    pcVar4 = *(code **)(local_38 + 8);
    if (pcVar4 != (code *)0x0) goto LAB_0012b625;
  }
  bVar3 = bVar1 ^ 1;
LAB_0012b63f:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_diff_file @ 0x2b690

undefined4 sparse_diff_file(undefined4 param_1,long param_2)

{
  char cVar1;
  undefined4 uVar2;
  undefined4 extraout_EDX;
  long *plVar3;
  ulong uVar4;
  long lVar5;
  long in_FS_OFFSET;
  undefined4 local_68;
  undefined1 local_64;
  long local_58;
  long local_50;
  long local_48;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00129d10(&local_68,param_2,param_1);
  if ((char)uVar2 == '\0') goto LAB_0012b760;
  local_64 = 1;
  local_68 = extraout_EDX;
  local_50 = param_2;
  if (*(code **)(local_48 + 0x28) == (code *)0x0) {
    FUN_0010c670(param_2);
LAB_0012b6f8:
    lVar5 = 0;
    uVar4 = 0;
    do {
      if (*(ulong *)(local_50 + 0x130) <= uVar4) goto LAB_0012b748;
      cVar1 = FUN_0012a9d0(&local_68,lVar5,
                           *(undefined8 *)(*(long *)(local_50 + 0x140) + uVar4 * 0x10));
      if (cVar1 == '\0') break;
      cVar1 = FUN_0012ab20(&local_68,uVar4);
      plVar3 = (long *)(uVar4 * 0x10 + *(long *)(local_50 + 0x140));
      lVar5 = plVar3[1] + *plVar3;
      uVar4 = uVar4 + 1;
    } while (cVar1 != '\0');
  }
  else {
    cVar1 = (**(code **)(local_48 + 0x28))();
    FUN_0010c670(param_2);
    if (cVar1 != '\0') goto LAB_0012b6f8;
  }
  uVar2 = 0;
  FUN_00123d40(*(long *)(local_50 + 0x118) - local_58);
LAB_0012b748:
  FUN_0010c6a0();
  if (*(code **)(local_48 + 8) != (code *)0x0) {
    (**(code **)(local_48 + 8))(&local_68);
  }
LAB_0012b760:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: find_compression_suffix @ 0x2b7d0

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
    if (PTR_s_tar_00164a53_4_0017f920 != (undefined *)0x0) {
      ppuVar4 = &PTR_s_tar_00164a53_4_0017f920;
      __s1 = PTR_s_tar_00164a53_4_0017f920;
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




// Function: set_compression_program_by_suffix @ 0x2b880

void set_compression_program_by_suffix(undefined8 param_1,long param_2)

{
  long lVar1;
  
  lVar1 = FUN_0012b7d0(param_1,0);
  if (lVar1 != 0) {
    param_2 = *(long *)(lVar1 + 0x10);
  }
  if (param_2 != 0) {
    DAT_00182b70 = param_2;
  }
  return;
}




// Function: strip_compression_suffix @ 0x2b8b0

void * strip_compression_suffix(void *param_1)

{
  int iVar1;
  undefined8 *puVar2;
  void *__dest;
  long in_FS_OFFSET;
  ulong local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  puVar2 = (undefined8 *)FUN_0012b7d0(param_1,&local_38);
  __dest = (void *)0x0;
  if (puVar2 != (undefined8 *)0x0) {
    if (local_38 < 5) {
      __dest = (void *)0x0;
      if (local_38 == 0) goto LAB_0012b933;
    }
    else {
      iVar1 = strncmp((char *)((long)param_1 + (local_38 - 4)),".tar",4);
      if ((iVar1 == 0) && (*(char *)*puVar2 != 't')) {
        local_38 = local_38 - 4;
      }
    }
    __dest = (void *)FUN_0014ecc0(local_38 + 1);
    memcpy(__dest,param_1,local_38);
    *(undefined1 *)((long)__dest + local_38) = 0;
  }
LAB_0012b933:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return __dest;
}




// Function: xexec @ 0x2b970

void xexec(undefined8 param_1)

{
  __pid_t _Var1;
  int *piVar2;
  uint __status;
  long in_FS_OFFSET;
  uint uStack_5c;
  undefined8 uStack_58;
  char *local_38;
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_38 = "/bin/sh";
  local_30 = &DAT_00166997;
  local_20 = 0;
  local_28 = param_1;
  execv("/bin/sh",&local_38);
  FUN_00132da0();
  uStack_58 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  do {
    _Var1 = waitpid((__pid_t)param_1,(int *)&uStack_5c,0);
    if (_Var1 != -1) goto LAB_0012b9fb;
    piVar2 = __errno_location();
  } while (*piVar2 == 4);
  FUN_001334d0(DAT_00182b70);
LAB_0012b9fb:
  __status = uStack_5c >> 8 & 0xff;
  if ('\x01' < (char)(((byte)uStack_5c & 0x7f) + 1)) {
    raise(uStack_5c & 0x7f);
    __status = 0;
  }
                    /* WARNING: Subroutine does not return */
  exit(__status);
}




// Function: wait_for_grandchild @ 0x2b9c0

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
    if (_Var1 != -1) goto LAB_0012b9fb;
    piVar2 = __errno_location();
  } while (*piVar2 == 4);
  FUN_001334d0(DAT_00182b70);
LAB_0012b9fb:
  __status = local_24 >> 8 & 0xff;
  if ('\x01' < (char)(((byte)local_24 & 0x7f) + 1)) {
    raise(local_24 & 0x7f);
    __status = 0;
  }
                    /* WARNING: Subroutine does not return */
  exit(__status);
}




// Function: run_decompress_program @ 0x2ba30

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
  lVar2 = FUN_0010c810(local_12c);
  if (lVar2 != 0) {
    uVar6 = 0x2080e06;
    while( true ) {
      iVar1 = FUN_001375a0(lVar2,&local_128,uVar6);
      if (iVar1 != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar6 = FUN_001358f0(&local_128);
        uVar5 = dcgettext(0,"cannot split string \'%s\': %s",5);
        error(0,0,uVar5,lVar2,uVar6);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
      memmove(local_120,local_120 + local_118,local_128 * 8);
      local_120[local_128] = "-d";
      execvp(*local_120,local_120);
      local_120[local_128] = (char *)0x0;
      lVar3 = FUN_0010c890(local_12c);
      if (lVar3 == 0) break;
      if ((DAT_00181b80._2_1_ & 8) != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar6 = dcgettext(0,"cannot run %s",5);
        piVar4 = __errno_location();
        error(0,*piVar4,uVar6,lVar2);
        if ((DAT_00181b80._2_1_ & 8) != 0) {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar6 = dcgettext(0,"trying %s",5);
          error(0,0,uVar6,lVar3);
        }
      }
      uVar6 = 0x2080e0e;
      lVar2 = lVar3;
    }
    FUN_00132da0(lVar2);
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar6 = dcgettext(0,"unable to run decompression program",5);
  error(0,0,uVar6);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: sys_get_archive_stat @ 0x2bd50

bool sys_get_archive_stat(void)

{
  int iVar1;
  
  iVar1 = fstat(DAT_00182a34,(stat *)&DAT_00182740);
  return iVar1 == 0;
}




// Function: sys_file_is_archive @ 0x2bd80

bool sys_file_is_archive(long param_1)

{
  if ((DAT_00182818 != 0) && (DAT_00182818 == *(long *)(param_1 + 0x58))) {
    return *(long *)(param_1 + 0x60) == DAT_00182810;
  }
  return false;
}




// Function: sys_save_archive_dev_ino @ 0x2bdc0

void sys_save_archive_dev_ino(void)

{
  if ((DAT_00182a34 < 0x40000000) && ((DAT_00182758 & 0xf000) == 0x8000)) {
    DAT_00182818 = DAT_00182740;
    DAT_00182810 = DAT_00182748;
    return;
  }
  DAT_00182818 = 0;
  return;
}




// Function: sys_detect_dev_null_output @ 0x2be10

void sys_detect_dev_null_output(void)

{
  int iVar1;
  long in_FS_OFFSET;
  bool bVar2;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = strcmp((char *)*DAT_00182838,"/dev/null");
  bVar2 = true;
  if (iVar1 != 0) {
    if ((DAT_00182a34 < 0x40000000) && ((DAT_00182758 & 0xf000) == 0x2000)) {
      iVar1 = stat("/dev/null",&local_a8);
      if ((iVar1 == 0) && (DAT_00182740 == local_a8.st_dev)) {
        bVar2 = DAT_00182748 == local_a8.st_ino;
        goto LAB_0012be75;
      }
    }
    bVar2 = false;
  }
LAB_0012be75:
  DAT_00182a30 = bVar2;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_wait_for_child @ 0x2bee0

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
        if ((char)(((byte)local_24 & 0x7f) + 1) < '\x02') goto LAB_0012bf68;
        goto LAB_0012bf3f;
      }
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    FUN_001334d0(DAT_00182b70);
    if ((char)(((byte)local_24 & 0x7f) + 1) < '\x02') {
LAB_0012bf68:
      if ((char)(local_24 >> 8) != '\0') {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar4 = local_24 >> 8 & 0xff;
        uVar3 = dcgettext(0,"Child returned status %d",5);
LAB_0012bff7:
        error(0,0,uVar3,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_00116fd0();
      }
    }
    else {
LAB_0012bf3f:
      uVar4 = local_24 & 0x7f;
      if ((uVar4 != 0xd) || (param_2 != '\0')) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar3 = dcgettext(0,"Child died with signal %d",5);
        goto LAB_0012bff7;
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_spawn_shell @ 0x2c010

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
  __pid = FUN_00126520();
  if (__pid == 0) {
    execlp(__file,"-sh",&DAT_001669ed,0);
    FUN_00132da0(__file);
  }
  else {
    do {
      _Var1 = waitpid(__pid,&local_24,0);
      if (_Var1 != -1) goto LAB_0012c07b;
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    FUN_001334d0(__file);
LAB_0012c07b:
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_compare_uid @ 0x2c0d0

undefined4 sys_compare_uid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x1c) >> 8),
                  *(int *)(param_1 + 0x1c) == *(int *)(param_2 + 0x1c));
}




// Function: sys_compare_gid @ 0x2c0e0

undefined4 sys_compare_gid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x20) >> 8),
                  *(int *)(param_1 + 0x20) == *(int *)(param_2 + 0x20));
}




// Function: sys_compare_links @ 0x2c0f0

undefined8 sys_compare_links(long *param_1,long *param_2)

{
  if (*param_2 != *param_1) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_1[1] >> 8),param_2[1] == param_1[1]);
}




// Function: sys_truncate @ 0x2c110

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




// Function: sys_write_archive_buffer @ 0x2c140

void sys_write_archive_buffer(void)

{
  if (0x3fffffff < DAT_00182a34) {
    FUN_001345c0(DAT_00182a34 + -0x40000000);
    return;
  }
  FUN_001431a0(DAT_00182a34,DAT_00181f58,DAT_00182b98);
  return;
}




// Function: sys_child_open_for_compress @ 0x2c180

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int sys_child_open_for_compress(void)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  int *piVar5;
  void *pvVar6;
  int *piVar7;
  char *pcVar8;
  long lVar9;
  long in_FS_OFFSET;
  int local_30;
  int local_2c;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  signal(0xd,(__sighandler_t)0x1);
  FUN_00126560(&local_30);
  iVar1 = FUN_00126520();
  if (0 < iVar1) {
    DAT_00182a34 = local_2c;
    FUN_0010cd20(local_30);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar1;
    }
LAB_0012c52d:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar3 = dcgettext(0,"tar (child)",5);
  FUN_0014af40(uVar3);
  signal(0xd,(__sighandler_t)0x0);
  if (local_30 != 0) goto LAB_0012c2a6;
  do {
    FUN_0010cd20(local_2c);
    piVar7 = DAT_00182838;
    if (DAT_00182d70 == '\0') {
      _DAT_00182d78 = strchr(*(char **)DAT_00182838,0x3a);
      pcVar8 = *(char **)piVar7;
      if (((_DAT_00182d78 == (char *)0x0) || (_DAT_00182d78 <= pcVar8)) ||
         (pvVar6 = memchr(pcVar8,0x2f,(long)_DAT_00182d78 - (long)pcVar8), pvVar6 != (void *)0x0))
      goto LAB_0012c237;
    }
    else {
      pcVar8 = *(char **)DAT_00182838;
LAB_0012c237:
      iVar1 = FUN_0012bc20(pcVar8);
      if (iVar1 != 0) {
        if (DAT_00182b84 != '\0') goto LAB_0012c4a4;
        goto LAB_0012c251;
      }
    }
    FUN_00126560(&local_28);
    iVar1 = FUN_00126520();
    if (iVar1 != 0) {
      if (local_28 != 0) {
        FUN_0012bc90(local_28,0);
      }
      FUN_0010cd20(local_24);
      piVar7 = DAT_00182838;
      pcVar8 = *(char **)DAT_00182838;
      iVar2 = strcmp(pcVar8,"-");
      if (iVar2 == 0) {
        DAT_00182a34 = 1;
        piVar5 = DAT_00182b98;
        goto LAB_0012c370;
      }
      if (DAT_00182d70 == '\0') {
        _DAT_00182d78 = strchr(pcVar8,0x3a);
        pcVar8 = *(char **)piVar7;
        if (((_DAT_00182d78 == (char *)0x0) || (_DAT_00182d78 <= pcVar8)) ||
           (pvVar6 = memchr(pcVar8,0x2f,(long)_DAT_00182d78 - (long)pcVar8), pvVar6 != (void *)0x0))
        goto LAB_0012c4bd;
        DAT_00182a34 = FUN_00133a80(pcVar8,0x41,0x40000000,DAT_00182ab0);
      }
      else {
LAB_0012c4bd:
        DAT_00182a34 = creat(pcVar8,0x1b6);
      }
      piVar5 = DAT_00182b98;
      if (DAT_00182a34 < 0) {
        FUN_00132ec0(*(undefined8 *)DAT_00182838);
        goto LAB_0012c4f0;
      }
LAB_0012c370:
      while (piVar5 != (int *)0x0) {
        piVar7 = (int *)0x0;
        lVar9 = DAT_00181f58;
        do {
          lVar4 = FUN_0014d570(0,lVar9,(long)piVar5 - (long)piVar7);
          if (lVar4 == -1) {
            FUN_00133090(DAT_00182b70);
            goto LAB_0012c52d;
          }
          if (lVar4 == 0) {
            if (piVar7 == (int *)0x0) goto LAB_0012c510;
            memset((void *)(DAT_00181f58 + (long)piVar7),0,(long)DAT_00182b98 - (long)piVar7);
            piVar5 = (int *)FUN_0012c140();
            if (DAT_00182b98 == piVar5) goto LAB_0012c510;
            goto LAB_0012c3f3;
          }
          piVar7 = (int *)((long)piVar7 + lVar4);
          lVar9 = lVar9 + lVar4;
          piVar5 = DAT_00182b98;
        } while (piVar7 < DAT_00182b98);
LAB_0012c4f0:
        piVar5 = (int *)FUN_0012c140();
        if (DAT_00182b98 != piVar5) {
LAB_0012c3f3:
                    /* WARNING: Subroutine does not return */
          FUN_0010da70(piVar5);
        }
      }
LAB_0012c510:
      FUN_0012b9c0(iVar1);
      do {
        FUN_00125ea0();
        do {
          piVar5 = DAT_00182838;
          *piVar7 = iVar1;
          FUN_00132ec0(*(undefined8 *)piVar5);
LAB_0012c4a4:
          FUN_00125c10(*(undefined8 *)DAT_00182838,1);
LAB_0012c251:
          pcVar8 = *(char **)DAT_00182838;
          iVar1 = strcmp(pcVar8,"-");
          if (iVar1 == 0) goto LAB_0012c29a;
          DAT_00182a34 = creat(pcVar8,0x1b6);
          if (-1 < DAT_00182a34) {
            if (DAT_00182a34 != 1) {
              FUN_0012bc90(DAT_00182a34,1);
            }
            goto LAB_0012c29a;
          }
          piVar7 = __errno_location();
          iVar1 = *piVar7;
        } while (DAT_00182b84 == '\0');
      } while( true );
    }
    uVar3 = dcgettext(0,"tar (grandchild)",5);
    FUN_0014af40(uVar3);
    if (local_24 != 1) {
      FUN_0012bc90(local_24,1);
    }
    FUN_0010cd20(local_28);
LAB_0012c29a:
    FUN_0012b970(DAT_00182b70);
LAB_0012c2a6:
    FUN_0012bc90();
  } while( true );
}




// Function: sys_child_open_for_uncompress @ 0x2c570

/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int sys_child_open_for_uncompress(void)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  void *pvVar4;
  char *pcVar5;
  char *pcVar6;
  undefined8 *puVar7;
  char *pcVar8;
  char *pcVar9;
  long in_FS_OFFSET;
  undefined4 local_40;
  int local_3c;
  int local_38;
  int local_34;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00126560(&local_40);
  iVar1 = FUN_00126520();
  if (0 < iVar1) {
    DAT_00182a34 = local_40;
    FUN_0010cd20(local_3c);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar1;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar3 = dcgettext(0,"tar (child)",5);
  FUN_0014af40(uVar3);
  signal(0xd,(__sighandler_t)0x0);
  if (local_3c != 1) {
    FUN_0012bc90(local_3c,1);
  }
  FUN_0010cd20(local_40);
  puVar7 = DAT_00182838;
  pcVar8 = (char *)*DAT_00182838;
  iVar1 = strcmp(pcVar8,"-");
  if (iVar1 != 0) {
    if (DAT_00182d70 == '\0') {
      _DAT_00182d78 = strchr(pcVar8,0x3a);
      pcVar8 = (char *)*puVar7;
      if (((_DAT_00182d78 != (char *)0x0) && (pcVar8 < _DAT_00182d78)) &&
         (pvVar4 = memchr(pcVar8,0x2f,(long)_DAT_00182d78 - (long)pcVar8), pvVar4 == (void *)0x0))
      goto LAB_0012c658;
    }
    iVar1 = FUN_0012bc20(pcVar8);
    if (iVar1 != 0) goto LAB_0012c822;
  }
LAB_0012c658:
  FUN_00126560(&local_38);
  iVar1 = FUN_00126520();
  if (iVar1 == 0) {
    uVar3 = dcgettext(0,"tar (grandchild)",5);
    FUN_0014af40(uVar3);
    if (local_38 != 0) {
      FUN_0012bc90(local_38,0);
    }
    FUN_0010cd20(local_34);
  }
  else {
    if (local_34 != 1) {
      FUN_0012bc90(local_34,1);
    }
    FUN_0010cd20(local_38);
    puVar7 = DAT_00182838;
    pcVar8 = (char *)*DAT_00182838;
    iVar2 = strcmp(pcVar8,"-");
    if (iVar2 != 0) goto LAB_0012c7b2;
    DAT_00182a34 = 0;
    do {
      while( true ) {
        FUN_0010c730();
        while( true ) {
          if (DAT_00182a34 < 0x40000000) {
            pcVar5 = (char *)FUN_0014d570();
            pcVar9 = DAT_00181f58;
          }
          else {
            pcVar5 = (char *)FUN_00134460(DAT_00182a34 + -0x40000000,DAT_00181f58,DAT_00182b98);
            pcVar9 = DAT_00181f58;
          }
          DAT_00181f58 = pcVar9;
          if (pcVar5 != (char *)0xffffffffffffffff) break;
          FUN_0010cd60();
        }
        if (pcVar5 == (char *)0x0) break;
        do {
          pcVar8 = (char *)0x200;
          if (pcVar5 < (char *)0x201) {
            pcVar8 = pcVar5;
          }
          pcVar6 = (char *)FUN_001431a0(1,pcVar9,pcVar8);
          if (pcVar6 != pcVar8) {
            FUN_001334f0(DAT_00182b70);
          }
          pcVar9 = pcVar9 + (long)pcVar8;
          pcVar5 = pcVar5 + -(long)pcVar8;
        } while (pcVar5 != (char *)0x0);
      }
      FUN_0010cd20(1);
      FUN_0012b9c0(iVar1);
      puVar7 = (undefined8 *)0x0;
LAB_0012c7b2:
      if (DAT_00182d70 == '\0') {
        _DAT_00182d78 = strchr(pcVar8,0x3a);
        pcVar8 = (char *)*puVar7;
        if (((_DAT_00182d78 == (char *)0x0) || (_DAT_00182d78 <= pcVar8)) ||
           (pvVar4 = memchr(pcVar8,0x2f,(long)_DAT_00182d78 - (long)pcVar8), pvVar4 != (void *)0x0))
        goto LAB_0012c7f4;
        DAT_00182a34 = FUN_00133a80(pcVar8,0,0x40000000,DAT_00182ab0);
      }
      else {
LAB_0012c7f4:
        DAT_00182a34 = open(pcVar8,0,0x1b6);
      }
    } while (-1 < DAT_00182a34);
    do {
      FUN_00132ec0(*DAT_00182838);
LAB_0012c822:
      iVar1 = open((char *)*DAT_00182838,0,0x1b6);
      DAT_00182a34 = iVar1;
    } while (iVar1 < 0);
    if (iVar1 != 0) goto LAB_0012c84b;
  }
  do {
    iVar1 = FUN_0012ba30();
LAB_0012c84b:
    FUN_0012bc90(iVar1,0);
  } while( true );
}




// Function: sys_exec_command @ 0x2c8b0

undefined4 sys_exec_command(char *param_1,char param_2,long param_3)

{
  int iVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  int local_1a4;
  undefined4 local_1a0;
  char local_19a [34];
  undefined1 local_178 [32];
  undefined1 local_158 [32];
  undefined1 local_138 [32];
  undefined1 local_118 [32];
  undefined1 local_f8 [32];
  undefined1 local_d8 [32];
  char local_b8 [32];
  undefined1 local_98 [32];
  undefined1 local_78 [32];
  undefined1 local_58 [40];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00126560(&local_1a4);
  DAT_00182728 = signal(0xd,(__sighandler_t)0x1);
  DAT_00182730 = FUN_00126520();
  if (DAT_00182730 != 0) {
    FUN_0010cd20();
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_1a0;
    }
    goto LAB_0012cd2c;
  }
  if (local_1a4 != 0) {
    FUN_0012bc90(local_1a4,0);
  }
  FUN_0010cd20(local_1a0);
  iVar1 = setenv("TAR_VERSION","1.34",1);
  if (iVar1 != 0) goto LAB_0012cd98;
  if ((char *)*DAT_00182828 == (char *)0x0) {
    unsetenv("TAR_ARCHIVE");
  }
  else {
    iVar1 = setenv("TAR_ARCHIVE",(char *)*DAT_00182828,1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  pcVar2 = (char *)FUN_00145370(((long)DAT_00182828 - DAT_00182838 >> 3) + 1,local_178);
  iVar1 = setenv("TAR_VOLUME",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_00145370((long)DAT_00182ba0,local_19a + 2);
  iVar1 = setenv("TAR_BLOCKING_FACTOR",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  iVar1 = DAT_00182400;
  if (DAT_00182400 == 0) {
    iVar1 = DAT_00182ba4;
  }
  pcVar2 = (char *)FUN_0012dce0(iVar1);
  if (pcVar2 == (char *)0x0) {
    unsetenv("TAR_FORMAT");
  }
  else {
    iVar1 = setenv("TAR_FORMAT",pcVar2,1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  local_19a[1] = 0;
  local_19a[0] = param_2;
  iVar1 = setenv("TAR_FILETYPE",local_19a,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  __snprintf_chk(local_b8,0x18,1,0x18,&DAT_00166a60,*(undefined4 *)(param_3 + 0x70));
  iVar1 = setenv("TAR_MODE",local_b8,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  if (param_1 == (char *)0x0) {
    unsetenv("TAR_FILENAME");
  }
  else {
    iVar1 = setenv("TAR_FILENAME",param_1,1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  if (*(char **)(param_3 + 8) == (char *)0x0) {
    unsetenv("TAR_REALNAME");
  }
  else {
    iVar1 = setenv("TAR_REALNAME",*(char **)(param_3 + 8),1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  if (*(char **)(param_3 + 0x20) == (char *)0x0) {
    unsetenv("TAR_UNAME");
  }
  else {
    iVar1 = setenv("TAR_UNAME",*(char **)(param_3 + 0x20),1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  if (*(char **)(param_3 + 0x28) == (char *)0x0) {
    unsetenv("TAR_GNAME");
  }
  else {
    iVar1 = setenv("TAR_GNAME",*(char **)(param_3 + 0x28),1);
    if (iVar1 != 0) goto LAB_0012cd98;
  }
  pcVar2 = (char *)FUN_001257a0(*(undefined8 *)(param_3 + 0xe8),*(undefined8 *)(param_3 + 0xf0),
                                local_58);
  iVar1 = setenv("TAR_ATIME",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_001257a0(*(undefined8 *)(param_3 + 0xf8),*(undefined8 *)(param_3 + 0x100),
                                local_78);
  iVar1 = setenv("TAR_MTIME",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_001257a0(*(undefined8 *)(param_3 + 0x108),*(undefined8 *)(param_3 + 0x110),
                                local_98);
  iVar1 = setenv("TAR_CTIME",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_00145370(*(undefined8 *)(param_3 + 0x88),local_118);
  iVar1 = setenv("TAR_SIZE",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_00145370(*(undefined4 *)(param_3 + 0x74),local_138);
  iVar1 = setenv("TAR_UID",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  pcVar2 = (char *)FUN_00145370(*(undefined4 *)(param_3 + 0x78),local_158);
  iVar1 = setenv("TAR_GID",pcVar2,1);
  if (iVar1 != 0) goto LAB_0012cd98;
  if (param_2 == 'h') {
LAB_0012cd62:
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    if (*(char **)(param_3 + 0x18) == (char *)0x0) goto LAB_0012cd14;
    iVar1 = setenv("TAR_LINKNAME",*(char **)(param_3 + 0x18),1);
    if (iVar1 != 0) {
LAB_0012cd98:
                    /* WARNING: Subroutine does not return */
      FUN_00117020();
    }
  }
  else {
    if (param_2 < 'i') {
      if ((byte)(param_2 + 0x9eU) < 2) {
        pcVar2 = (char *)FUN_00145370((uint)((*(ulong *)(param_3 + 0x80) >> 0x14) << 8) |
                                      (uint)*(ulong *)(param_3 + 0x80) & 0xff,local_d8);
        iVar1 = setenv("TAR_MINOR",pcVar2,1);
        if (iVar1 != 0) goto LAB_0012cd98;
        pcVar2 = (char *)FUN_00145370((uint)((ulong)*(undefined8 *)(param_3 + 0x80) >> 0x20) &
                                      0xfffff000 |
                                      (uint)((ulong)*(undefined8 *)(param_3 + 0x80) >> 8) & 0xfff,
                                      local_f8);
        iVar1 = setenv("TAR_MAJOR",pcVar2,1);
        if (iVar1 != 0) goto LAB_0012cd98;
        goto LAB_0012cd14;
      }
    }
    else if (param_2 == 'l') goto LAB_0012cd62;
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
LAB_0012cd14:
    unsetenv("TAR_LINKNAME");
  }
  FUN_0012b970(DAT_00182a60);
LAB_0012cd2c:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_wait_command @ 0x2ce00

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
  
  __pid = DAT_00182730;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (-1 < DAT_00182730) {
    signal(0xd,DAT_00182728);
    do {
      _Var1 = waitpid(__pid,(int *)&local_24,0);
      if (_Var1 != -1) {
        uVar4 = local_24 & 0x7f;
        if (uVar4 == 0) {
          if ((DAT_00182a59 == '\0') && (uVar4 = local_24 >> 8, (char)(local_24 >> 8) != '\0')) {
            if (DAT_00182d50 != (code *)0x0) {
              (*DAT_00182d50)();
              uVar4 = local_24 >> 8;
            }
            lVar5 = (long)DAT_00182730;
            uVar3 = dcgettext(0,"%lu: Child returned status %d",5);
            error(0,0,uVar3,lVar5,uVar4 & 0xff);
            DAT_00182d58 = 2;
          }
        }
        else if ((char)((char)uVar4 + '\x01') < '\x02') {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          lVar5 = (long)DAT_00182730;
          uVar3 = dcgettext(0,"%lu: Child terminated on unknown reason",5);
          error(0,0,uVar3,lVar5);
          DAT_00182d58 = 2;
        }
        else {
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
            uVar4 = local_24 & 0x7f;
          }
          lVar5 = (long)DAT_00182730;
          uVar3 = dcgettext(0,"%lu: Child terminated on signal %d",5);
          error(0,0,uVar3,lVar5,uVar4);
        }
        DAT_00182730 = -1;
        goto LAB_0012ced5;
      }
      piVar2 = __errno_location();
      __pid = DAT_00182730;
    } while (*piVar2 == 4);
    DAT_00182730 = -1;
    FUN_001334d0(DAT_00182a60);
  }
LAB_0012ced5:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_exec_info_script @ 0x2cfc0

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
  FUN_00126560(&local_50);
  DAT_00182720 = signal(0xd,(__sighandler_t)0x1);
  iVar1 = FUN_00126520();
  if (iVar1 == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*param_1,1);
    pcVar5 = (char *)FUN_00145370((long)param_2,local_48);
    setenv("TAR_VOLUME",pcVar5,1);
    pcVar5 = (char *)FUN_00145370((long)DAT_00182ba0,local_48);
    setenv("TAR_BLOCKING_FACTOR",pcVar5,1);
    pcVar5 = (char *)FUN_0012dd30(DAT_00182ba8);
    setenv("TAR_SUBCOMMAND",pcVar5,1);
    iVar1 = DAT_00182400;
    if (DAT_00182400 == 0) {
      iVar1 = DAT_00182ba4;
    }
    pcVar5 = (char *)FUN_0012dce0(iVar1);
    setenv("TAR_FORMAT",pcVar5,1);
    pcVar5 = (char *)FUN_00145370((long)local_4c,local_48);
    setenv("TAR_FD",pcVar5,1);
    FUN_0010cd20(local_50);
    FUN_0012b970(DAT_00182b48);
  }
  else {
    local_60 = (char *)0x0;
    local_58 = 0;
    FUN_0010cd20(local_4c);
    __stream = fdopen(local_50,"r");
    _Var3 = __getdelim(&local_60,&local_58,10,__stream);
    iVar7 = (int)_Var3;
    fclose(__stream);
    if ((0 < iVar7) && (local_60[(long)iVar7 + -1] == '\n')) {
      local_60[(long)iVar7 + -1] = '\0';
      iVar7 = iVar7 + -1;
    }
    do {
      _Var2 = waitpid(iVar1,(int *)&local_64,0);
      if (_Var2 != -1) {
        signal(0xd,DAT_00182720);
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
        goto LAB_0012d0e3;
      }
      piVar4 = __errno_location();
    } while (*piVar4 == 4);
    signal(0xd,DAT_00182720);
    FUN_001334d0(DAT_00182b48);
    uVar6 = 0xffffffff;
LAB_0012d0e3:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return uVar6;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_exec_checkpoint_script @ 0x2d260

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
  iVar1 = FUN_00126520();
  if (iVar1 == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",param_2,1);
    pcVar4 = (char *)FUN_00145370((long)param_3,local_48);
    setenv("TAR_CHECKPOINT",pcVar4,1);
    pcVar4 = (char *)FUN_00145370((long)DAT_00182ba0,local_48);
    setenv("TAR_BLOCKING_FACTOR",pcVar4,1);
    pcVar4 = (char *)FUN_0012dd30(DAT_00182ba8);
    setenv("TAR_SUBCOMMAND",pcVar4,1);
    iVar1 = DAT_00182400;
    if (DAT_00182400 == 0) {
      iVar1 = DAT_00182ba4;
    }
    pcVar4 = (char *)FUN_0012dce0(iVar1);
    setenv("TAR_FORMAT",pcVar4,1);
    FUN_0012b970(param_1);
  }
  else {
    do {
      _Var2 = waitpid(iVar1,&local_4c,0);
      if (_Var2 != -1) goto LAB_0012d2bb;
      piVar3 = __errno_location();
    } while (*piVar3 == 4);
    FUN_001334d0(param_1);
LAB_0012d2bb:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_argp_option_key @ 0x2d3c0

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




// Function: format_default_settings @ 0x2d400

void format_default_settings(void)

{
  undefined **ppuVar1;
  char *pcVar2;
  
  pcVar2 = "oldgnu";
  ppuVar1 = &PTR_DAT_0017da00;
  while (pcVar2 != (char *)0x0) {
    if (*(int *)(ppuVar1 + 3) == 6) goto LAB_0012d42b;
    pcVar2 = ppuVar1[4];
    ppuVar1 = ppuVar1 + 2;
  }
  pcVar2 = "unknown?";
LAB_0012d42b:
  FUN_0014f3c0("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",pcVar2,
               &DAT_0016b716,0x14,DAT_0017e718,"/usr/local/libexec/rmt","/usr/bin/rsh");
  return;
}




// Function: optloc_save @ 0x2d480

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
  puVar4 = (undefined4 *)FUN_0014ecc0(lVar6);
  pcVar5 = (char *)0x0;
  if (*(char **)(param_2 + 2) != (char *)0x0) {
    pcVar5 = strcpy((char *)(puVar4 + 8),*(char **)(param_2 + 2));
  }
  uVar1 = *param_2;
  *(char **)(puVar4 + 2) = pcVar5;
  *puVar4 = uVar1;
  *(undefined8 *)(puVar4 + 4) = *(undefined8 *)(param_2 + 4);
  uVar2 = *(undefined8 *)(&DAT_00182bc0 + (ulong)param_1 * 8);
  *(undefined4 **)(&DAT_00182bc0 + (ulong)param_1 * 8) = puVar4;
  *(undefined8 *)(puVar4 + 6) = uVar2;
  return uVar2;
}




// Function: sigstat @ 0x2d500

void sigstat(int param_1)

{
  FUN_0010c770();
  FUN_0010cc30();
  signal(param_1,sigstat);
  return;
}




// Function: tar_help_filter @ 0x2d670

undefined8 tar_help_filter(int param_1,undefined8 param_2)

{
  char *pcVar1;
  size_t sVar2;
  size_t __n;
  undefined8 uVar3;
  undefined1 *puVar4;
  undefined **ppuVar5;
  undefined2 *puVar6;
  long in_FS_OFFSET;
  undefined1 auStack_98 [8];
  long local_90;
  undefined1 *local_88;
  undefined2 *local_80;
  undefined1 *local_78;
  ulong local_68;
  byte local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0x94) {
    uVar3 = dcgettext(0,"filter the archive through %s",5);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      pcVar1 = "lzip";
      goto LAB_0012d879;
    }
    goto LAB_0012dbf5;
  }
  if (param_1 < 0x95) {
    if (param_1 == 0x6a) {
      uVar3 = dcgettext(0,"filter the archive through %s",5);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "bzip2";
        goto LAB_0012d879;
      }
      goto LAB_0012dbf5;
    }
    if (param_1 < 0x6b) {
      if (param_1 == 0x4a) {
        uVar3 = dcgettext(0,"filter the archive through %s",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "xz";
          goto LAB_0012d879;
        }
        goto LAB_0012dbf5;
      }
      if (param_1 == 0x5a) {
        uVar3 = dcgettext(0,"filter the archive through %s",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "compress";
          goto LAB_0012d879;
        }
        goto LAB_0012dbf5;
      }
    }
    else if (param_1 == 0x7a) {
      uVar3 = dcgettext(0,"filter the archive through %s",5);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "gzip";
        goto LAB_0012d879;
      }
      goto LAB_0012dbf5;
    }
  }
  else {
    if (param_1 == 0xcf) {
      uVar3 = dcgettext(0,"filter the archive through %s",5);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        pcVar1 = "zstd";
        goto LAB_0012d879;
      }
      goto LAB_0012dbf5;
    }
    if (param_1 < 0xd0) {
      if (param_1 == 0x95) {
        uVar3 = dcgettext(0,"filter the archive through %s",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "lzma";
LAB_0012d879:
          uVar3 = FUN_0014f3c0(uVar3,pcVar1);
          return uVar3;
        }
        goto LAB_0012dbf5;
      }
      if (param_1 == 0x96) {
        uVar3 = dcgettext(0,"filter the archive through %s",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          pcVar1 = "lzop";
          goto LAB_0012d879;
        }
        goto LAB_0012dbf5;
      }
    }
    else if (param_1 == 0x2000004) {
      _obstack_begin(auStack_98,0,0,FUN_0014ecc0,PTR_free_0017efc8);
      pcVar1 = (char *)dcgettext(0,"Valid arguments for the --quoting-style option are:",5);
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_78 - (long)local_80) < sVar2) {
        _obstack_newchunk(auStack_98,sVar2);
      }
      memcpy(local_80,pcVar1,sVar2);
      local_80 = (undefined2 *)((long)local_80 + sVar2);
      if ((ulong)((long)local_78 - (long)local_80) < 2) {
        _obstack_newchunk(auStack_98,2);
      }
      *local_80 = 0xa0a;
      puVar6 = local_80 + 1;
      local_80 = puVar6;
      sVar2 = strlen("  ");
      if (PTR_s_literal_0017e6e0 != (undefined *)0x0) {
        ppuVar5 = &PTR_s_shell_0016a642_0x16_0017e6e8;
        pcVar1 = PTR_s_literal_0017e6e0;
        do {
          if ((ulong)((long)local_78 - (long)puVar6) < sVar2) {
            _obstack_newchunk(auStack_98,sVar2);
            puVar6 = local_80;
          }
          memcpy(puVar6,&DAT_0016aab9,sVar2);
          puVar6 = (undefined2 *)((long)local_80 + sVar2);
          local_80 = puVar6;
          __n = strlen(pcVar1);
          if ((ulong)((long)local_78 - (long)puVar6) < __n) {
            _obstack_newchunk(auStack_98,__n);
            puVar6 = local_80;
          }
          memcpy(puVar6,pcVar1,__n);
          local_80 = (undefined2 *)((long)local_80 + __n);
          if (local_80 == (undefined2 *)local_78) {
            _obstack_newchunk(auStack_98,1);
          }
          puVar6 = (undefined2 *)((long)local_80 + 1);
          *(undefined1 *)local_80 = 10;
          pcVar1 = *ppuVar5;
          ppuVar5 = ppuVar5 + 1;
          local_80 = puVar6;
        } while (pcVar1 != (char *)0x0);
      }
      pcVar1 = (char *)dcgettext(0,"\n*This* tar defaults to:\n",5);
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_78 - (long)local_80) < sVar2) {
        _obstack_newchunk(auStack_98,sVar2);
      }
      memcpy(local_80,pcVar1,sVar2);
      local_80 = (undefined2 *)((long)local_80 + sVar2);
      pcVar1 = (char *)FUN_0012d400();
      sVar2 = strlen(pcVar1);
      if ((ulong)((long)local_78 - (long)local_80) < sVar2) {
        _obstack_newchunk(auStack_98,sVar2);
      }
      memcpy(local_80,pcVar1,sVar2);
      local_80 = (undefined2 *)((long)local_80 + sVar2);
      puVar6 = local_80;
      if (local_80 == (undefined2 *)local_78) {
        _obstack_newchunk(auStack_98,1);
        puVar6 = local_80;
      }
      local_80 = (undefined2 *)((long)puVar6 + 1);
      *(undefined1 *)puVar6 = 10;
      if ((undefined2 *)local_78 == local_80) {
        _obstack_newchunk(auStack_98,1);
      }
      *(undefined1 *)local_80 = 0;
      if ((undefined1 *)((long)local_80 + 1) == local_88) {
        local_48 = local_48 | 2;
      }
      puVar4 = (undefined1 *)((ulong)((undefined1 *)((long)local_80 + 1) + local_68) & ~local_68);
      local_88 = local_78;
      if ((ulong)((long)puVar4 - local_90) <= (ulong)((long)local_78 - local_90)) {
        local_88 = puVar4;
      }
      local_80 = (undefined2 *)local_88;
      param_2 = FUN_0014ee30();
      _obstack_free(auStack_98,0);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2;
  }
LAB_0012dbf5:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: subcommand_string @ 0x2dd30

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




// Function: decode_signal @ 0x2ddf0

undefined4 decode_signal(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined **ppuVar3;
  char *__s2;
  
  ppuVar3 = &PTR_DAT_0017d940;
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
    if (ppuVar3 == &PTR_DAT_0017d990) break;
    iVar1 = strcmp(*ppuVar3,__s2);
  }
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar2 = dcgettext(0,"Unknown signal name: %s",5);
  error(0,0,uVar2,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00116fd0();
}




// Function: usage @ 0x2dea0

void usage(int param_1)

{
  argp_help(&PTR_DAT_0017fb20,stderr,4,DAT_00182dd0);
  FUN_00140180();
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: confirm @ 0x2df50

ulong confirm(ulong param_1,undefined8 param_2)

{
  char *pcVar1;
  uint uVar2;
  int iVar3;
  undefined8 uVar4;
  __ssize_t _Var5;
  long in_FS_OFFSET;
  char *local_30;
  size_t local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00182bb0 == (FILE *)0x0) {
    if ((DAT_00182a34 != 0) && (DAT_00182c10 == (undefined *)0x0)) {
      DAT_00182c10 = &DAT_00166c79;
      DAT_00182bb0 = stdin;
      goto LAB_0012df81;
    }
    DAT_00182bb0 = fopen("/dev/tty","r");
    if (DAT_00182bb0 != (FILE *)0x0) goto LAB_0012df81;
    FUN_00132ec0("/dev/tty");
LAB_0012e060:
    local_30 = (char *)0x0;
    local_28 = 0;
    _Var5 = __getdelim(&local_30,&local_28,10,DAT_00182bb0);
    if (_Var5 < 0) {
      DAT_00182bac = 1;
      free(local_30);
    }
    else {
      iVar3 = rpmatch(local_30);
      uVar2 = DAT_00182bac;
      param_1 = (ulong)(0 < iVar3);
      free(local_30);
      if (uVar2 == 0) goto LAB_0012dff4;
    }
  }
  else {
LAB_0012df81:
    uVar4 = FUN_0014d2d0(param_2);
    __fprintf_chk(DAT_00181f28,1,"%s %s?",param_1,uVar4);
    fflush_unlocked(DAT_00181f28);
    param_1 = (ulong)DAT_00182bac;
    if (DAT_00182bac == 0) goto LAB_0012e060;
    param_1 = 0;
  }
  pcVar1 = DAT_00181f28->_IO_write_ptr;
  if (pcVar1 < DAT_00181f28->_IO_write_end) {
    DAT_00181f28->_IO_write_ptr = pcVar1 + 1;
    *pcVar1 = '\n';
  }
  else {
    __overflow(DAT_00181f28,10);
  }
  fflush_unlocked(DAT_00181f28);
LAB_0012dff4:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_1 & 0xffffffff;
}




// Function: get_date_or_file @ 0x2e120

undefined4 get_date_or_file(long param_1,undefined8 param_2,char *param_3,__time_t *param_4)

{
  __time_t _Var1;
  __time_t _Var2;
  char cVar3;
  int iVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  long in_FS_OFFSET;
  stat sStack_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((byte)(*param_3 - 0x2eU) < 2) {
    iVar4 = stat(param_3,&sStack_c8);
    if (iVar4 != 0) {
      FUN_00133410(param_3);
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar9 = dcgettext(0,"Date sample file not found",5);
      error(0,0,uVar9);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
    *param_4 = sStack_c8.st_mtim.tv_sec;
    param_4[1] = sStack_c8.st_mtim.tv_nsec;
    uVar5 = 0;
  }
  else {
    cVar3 = FUN_0014aed0(param_4,param_3,0);
    if (cVar3 == '\0') {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar9 = FUN_0014d2d0(param_3);
      uVar6 = FUN_00123240(*param_4,param_4[1],0);
      uVar7 = dcgettext(0,"Substituting %s for unknown date format %s",5);
      error(0,0,uVar7,uVar6,uVar9);
      param_4[1] = 0;
      uVar5 = 1;
    }
    else {
      puVar8 = (undefined8 *)FUN_0014ecc0(0x28);
      _Var1 = *param_4;
      _Var2 = param_4[1];
      puVar8[3] = param_2;
      puVar8[1] = _Var1;
      puVar8[2] = _Var2;
      uVar9 = FUN_0014ee30(param_3);
      puVar8[4] = uVar9;
      uVar9 = *(undefined8 *)(param_1 + 8);
      *(undefined8 **)(param_1 + 8) = puVar8;
      *puVar8 = uVar9;
      uVar5 = 0;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: option_conflict_error @ 0x2e2f0

void option_conflict_error(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  uVar1 = dcgettext(0,"\'%s\' cannot be used with \'%s\'",5);
  error(0,0,uVar1,param_1,param_2);
                    /* WARNING: Subroutine does not return */
  FUN_0012dea0(2);
}




// Function: set_old_files_option @ 0x2e340

void set_old_files_option(uint param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  char *__s2;
  
  piVar2 = (int *)FUN_0012d480(9);
  if ((piVar2 != (int *)0x0) && (*param_2 == *piVar2)) {
    if ((*param_2 != 1) &&
       (iVar1 = strcmp(*(char **)(param_2 + 2),*(char **)(piVar2 + 2)), iVar1 != 0)) {
      DAT_00182b34 = param_1;
      return;
    }
    if (DAT_00182b34 != param_1) {
      __s2 = (&PTR_s___overwrite_dir_0017d900)[(int)param_1];
      FUN_0012e2f0(__s2,(&PTR_s___overwrite_dir_0017d900)[DAT_00182b34]);
      piVar2 = (int *)FUN_0012d480(0);
      if (((DAT_00182b70 != (char *)0x0) && (iVar1 = strcmp(DAT_00182b70,__s2), iVar1 != 0)) &&
         (*piVar2 == 1)) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar3 = dcgettext(0,"Conflicting compression options",5);
        error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
        FUN_0012dea0(2);
      }
      DAT_00182b70 = __s2;
      return;
    }
  }
  DAT_00182b34 = param_1;
  return;
}




// Function: parse_opt @ 0x2e4c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int parse_opt(int param_1,char *param_2,long *param_3)

{
  long *plVar1;
  byte bVar2;
  int *piVar3;
  undefined1 auVar4 [16];
  char cVar5;
  int iVar6;
  undefined4 uVar7;
  long lVar8;
  ulong uVar9;
  size_t __n;
  void *pvVar10;
  ushort **ppuVar11;
  void *__dest;
  size_t __n_00;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  long lVar15;
  ulong uVar16;
  char *pcVar17;
  undefined8 *puVar18;
  long in_FS_OFFSET;
  ulong local_d8;
  undefined8 local_d0;
  char *local_c8;
  long local_c0;
  char *local_b8;
  char *local_b0;
  char *local_a8;
  ulong local_98;
  byte local_78;
  undefined1 local_68 [40];
  long local_40;
  
  puVar18 = (undefined8 *)param_3[5];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (0xcf < param_1) {
    if (param_1 != 0x1000003) {
      if (param_1 == 0x1000005) {
        piVar3 = (int *)*puVar18;
        if (*piVar3 == 2) {
          uVar14 = *(undefined8 *)(piVar3 + 4);
          uVar12 = *(undefined8 *)(piVar3 + 2);
          uVar13 = dcgettext(0,"%s:%lu: location of the error",5);
          error(0,0,uVar13,uVar12,uVar14);
        }
        else if (*piVar3 == 0) {
          uVar14 = *(undefined8 *)(piVar3 + 2);
          uVar12 = dcgettext(0,"error parsing %s",5);
          error(0,0,uVar12,uVar14);
        }
                    /* WARNING: Subroutine does not return */
        exit(0x40);
      }
      param_1 = 7;
      goto LAB_0012e591;
    }
    if ((*(long **)(*param_3 + 0x20) != (long *)0x0) && (**(long **)(*param_3 + 0x20) != 0)) {
      lVar15 = 0x20;
      lVar8 = 0;
      while( true ) {
        *(undefined8 **)(param_3[6] + lVar8) = puVar18;
        lVar8 = lVar8 + 8;
        plVar1 = (long *)(*(long *)(*param_3 + 0x20) + lVar15);
        lVar15 = lVar15 + 0x20;
        if (*plVar1 == 0) break;
        puVar18 = (undefined8 *)param_3[5];
      }
    }
    goto LAB_0012e575;
  }
  if (param_1 < 0) {
    param_1 = 7;
    goto LAB_0012e591;
  }
  switch(param_1) {
  case 0:
    FUN_00128130(param_2);
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
    uVar14 = dcgettext(0,"Options \'-[0-7][lmh]\' not supported by *this* tar",5);
    argp_error(param_3,uVar14);
                    /* WARNING: Subroutine does not return */
    exit(0x40);
  case 0x41:
    FUN_0012e2a0(2);
    param_1 = 0;
    break;
  case 0x42:
    DAT_00182ab9 = 1;
    param_1 = 0;
    break;
  case 0x46:
    param_1 = 0;
    DAT_00182b10 = 1;
    DAT_00182b48 = param_2;
    break;
  case 0x47:
    goto switchD_0012e520_caseD_47;
  case 0x48:
    FUN_0012e430(param_2);
    param_1 = 0;
    break;
  case 0x49:
    FUN_0012e3b0(param_2,*puVar18);
    param_1 = 0;
    break;
  case 0x4a:
    FUN_0012e3b0(&DAT_00166987,*puVar18);
    param_1 = 0;
    break;
  case 0x4b:
    FUN_0012d480(5,*puVar18);
    FUN_001286f0(param_2);
    param_1 = 0;
    break;
  case 0x4c:
    iVar6 = FUN_0014eec0(param_2,&local_c8,10,&local_d8,"bBcGgkKMmPTtw");
    if (iVar6 != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid tape length";
LAB_0012f954:
      uVar14 = dcgettext(0,pcVar17,5);
      uVar12 = FUN_0014ce40(param_2);
      error(0,0,"%s: %s",uVar12,uVar14);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
    if ((param_2 < local_c8) &&
       (pcVar17 = strchr("bBcGgkKMmPTtw",(int)local_c8[-1]), pcVar17 == (char *)0x0)) {
      DAT_00182a70 = (double)local_d8 * _DAT_00169d60;
    }
    else if ((long)local_d8 < 0) {
      DAT_00182a70 = (double)local_d8;
    }
    else {
      DAT_00182a70 = (double)(long)local_d8;
    }
    DAT_00182b10 = 1;
    param_1 = 0;
    break;
  case 0x4d:
    DAT_00182b10 = 1;
    param_1 = 0;
    break;
  case 0x4e:
    DAT_00182b8c = 1;
    if (-1 < DAT_00182b08) {
LAB_0012f8e5:
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "More than one threshold date";
LAB_0012f8ff:
      uVar14 = dcgettext(0,pcVar17,5);
      error(0,0,uVar14);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
    pcVar17 = "--after-date";
    goto LAB_0012ea57;
  case 0x4f:
    DAT_00182a6a = 1;
    param_1 = 0;
    break;
  case 0x50:
    FUN_0012d480(8,*puVar18);
    DAT_00182b92 = 1;
    param_1 = 0;
    break;
  case 0x52:
    DAT_00182b7c = 1;
    param_1 = 0;
    break;
  case 0x53:
    DAT_00182a88 = 1;
    param_1 = 0;
    break;
  case 0x55:
    FUN_0012e340(3,*puVar18);
    param_1 = 0;
    break;
  case 0x56:
    param_1 = 0;
    DAT_00182a40 = param_2;
    break;
  case 0x57:
    FUN_0012d480(4,*puVar18);
    DAT_00182a50 = 1;
    param_1 = 0;
    break;
  case 0x5a:
    FUN_0012e3b0("compress",*puVar18);
    param_1 = 0;
    break;
  case 0x61:
    *(undefined1 *)((long)puVar18 + 0x12) = 1;
    param_1 = 0;
    break;
  case 0x62:
    iVar6 = FUN_0014eec0(param_2,0,10,&local_c8,&DAT_0016a7d3);
    if ((((iVar6 != 0) || (DAT_00182ba0 = (int)local_c8, DAT_00182ba0 < 1)) ||
        ((char *)(long)DAT_00182ba0 != local_c8)) ||
       (DAT_00182b98 = (char *)((long)local_c8 << 9),
       local_c8 != (char *)((ulong)local_c8 & 0x7fffffffffffff))) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid blocking factor";
      goto LAB_0012f954;
    }
    param_1 = 0;
    break;
  case 99:
    FUN_0012e2a0(3);
    param_1 = 0;
    break;
  case 100:
    FUN_0012e2a0(5);
    param_1 = 0;
    break;
  case 0x66:
    if (DAT_00182830 == DAT_00182c18) {
      if (DAT_00182838 == 0) {
        if (DAT_00182c18 == 0) {
          DAT_00182c18 = 0x10;
        }
        else {
          auVar4._8_8_ = 0;
          auVar4._0_8_ = DAT_00182c18;
          if ((SUB168(ZEXT816(8) * auVar4,0) < 0) || (SUB168(ZEXT816(8) * auVar4,8) != 0))
          goto LAB_0012f7bf;
        }
      }
      else {
        if (0xaaaaaaaaaaaaaa9 < DAT_00182c18) {
LAB_0012f7bf:
                    /* WARNING: Subroutine does not return */
          FUN_00117020();
        }
        DAT_00182c18 = DAT_00182c18 + 1 + (DAT_00182c18 >> 1);
      }
      DAT_00182838 = FUN_0014ece0(DAT_00182838,DAT_00182c18 * 8);
    }
    lVar8 = DAT_00182830 * 8;
    DAT_00182830 = DAT_00182830 + 1;
    *(char **)(DAT_00182838 + lVar8) = param_2;
    param_1 = 0;
    break;
  case 0x67:
    FUN_0012d480(2,*puVar18);
    DAT_00182b8c = 1;
    DAT_00182b28 = param_2;
    goto switchD_0012e520_caseD_47;
  case 0x68:
    DAT_00182b69 = 1;
    param_1 = 0;
    break;
  case 0x69:
    DAT_00182b51 = 1;
    param_1 = 0;
    break;
  case 0x6a:
    FUN_0012e3b0("bzip2",*puVar18);
    param_1 = 0;
    break;
  case 0x6b:
    FUN_0012e340(4,*puVar18);
    param_1 = 0;
    break;
  case 0x6c:
    DAT_00182c20 = 1;
    param_1 = 0;
    break;
  case 0x6d:
    DAT_00182a68 = 1;
    param_1 = 0;
    break;
  case 0x6e:
    DAT_001827fc = 1;
    param_1 = 0;
    break;
  case 0x6f:
    *(undefined1 *)(puVar18 + 2) = 1;
    param_1 = 0;
    break;
  case 0x70:
    DAT_00182aa4 = 1;
    param_1 = 0;
    break;
  case 0x72:
    FUN_0012e2a0(1);
    param_1 = 0;
    break;
  case 0x73:
    FUN_0012d480(6,*puVar18);
    DAT_00182aac = 1;
    param_1 = 0;
    break;
  case 0x74:
    FUN_0012e2a0(7);
    DAT_00182a54 = DAT_00182a54 + 1;
    param_1 = 0;
    break;
  case 0x75:
    FUN_0012e2a0(8);
    param_1 = 0;
    break;
  case 0x76:
    DAT_00182a54 = DAT_00182a54 + 1;
    param_1 = 0;
    DAT_00181b80 = DAT_00181b80 | 0x583000;
    break;
  case 0x77:
    DAT_00182b40 = 1;
    param_1 = 0;
    break;
  case 0x78:
    FUN_0012e2a0(6);
    param_1 = 0;
    break;
  case 0x7a:
    FUN_0012e3b0(&DAT_0016750c,*puVar18);
    param_1 = 0;
    break;
  case 0x80:
    FUN_0012e430("posix");
    param_1 = 0;
    DAT_00182a9c = 1;
    break;
  case 0x81:
    uVar7 = 1;
    if (param_2 != (char *)0x0) {
      lVar8 = FUN_0013a4e0("--atime-preserve",param_2,&PTR_s_replace_0017d9e0,&DAT_00169a90,4,
                           PTR_FUN_00181dd8);
      uVar7 = *(undefined4 *)(&DAT_00169a90 + lVar8 * 4);
    }
    param_1 = 0;
    DAT_00182b88 = uVar7;
    break;
  case 0x82:
    DAT_00182b84 = 1;
    if (param_2 != (char *)0x0) {
      puVar18[4] = param_2;
      param_1 = 0;
      break;
    }
    goto LAB_0012e575;
  case 0x83:
    DAT_00182b20 = 1;
    param_1 = 0;
    break;
  case 0x84:
    if (param_2 == (char *)0x0) {
      DAT_00182b78 = 10;
      param_1 = 0;
    }
    else {
      if (*param_2 == '.') {
        param_2 = param_2 + 1;
        FUN_00110710(".");
      }
      uVar9 = strtoul(param_2,&local_c8,0);
      DAT_00182b78 = (undefined4)uVar9;
      if (*local_c8 != '\0') {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar17 = "--checkpoint value is not an integer";
        goto LAB_0012f9a2;
      }
      param_1 = 0;
    }
    break;
  case 0x85:
    FUN_00110710(param_2);
    param_1 = 0;
    break;
  case 0x86:
    DAT_00182af0 = 2;
    param_1 = 0;
    break;
  case 0x87:
    DAT_001827e0 = 1;
    param_1 = 0;
    break;
  case 0x88:
    DAT_00182b68 = 1;
    param_1 = 0;
    break;
  case 0x89:
    FUN_0012e2a0(4);
    param_1 = 0;
    break;
  case 0x8a:
    DAT_00182d70 = 1;
    param_1 = 0;
    break;
  case 0x8b:
    DAT_00182b90 = 1;
    param_1 = 0;
    break;
  case 0x8c:
    lVar8 = FUN_0012d520(param_2,&DAT_00182b58);
    if (lVar8 != -1) {
      DAT_00182b54 = (undefined4)lVar8;
      param_1 = 0;
      break;
    }
    DAT_00182b54 = 0xffffffff;
    if (DAT_00182b58 != 0) {
      FUN_00128080(DAT_00182b58,&DAT_00182b54);
      param_1 = 0;
      break;
    }
    goto LAB_0012e575;
  case 0x8d:
    FUN_00124eb0(param_2);
    param_1 = 0;
    break;
  case 0x8e:
    DAT_00182a59 = 1;
    param_1 = 0;
    break;
  case 0x8f:
    DAT_00182b52 = 1;
    param_1 = 0;
    break;
  case 0x90:
    param_1 = 0;
    DAT_00182820 = param_2;
    break;
  case 0x91:
    DAT_00182b30 = 1;
    param_1 = 0;
    break;
  case 0x92:
    FUN_0012e340(6,*puVar18);
    param_1 = 0;
    break;
  case 0x93:
    uVar9 = strtoul(param_2,&local_c8,10);
    DAT_00182b24 = (undefined4)uVar9;
    if (*local_c8 == '\0') {
      param_1 = 0;
      break;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar17 = "Invalid incremental level value";
    goto LAB_0012f8ff;
  case 0x94:
    FUN_0012e3b0(&DAT_001631cd,*puVar18);
    param_1 = 0;
    break;
  case 0x95:
    FUN_0012e3b0(&DAT_001631d4,*puVar18);
    param_1 = 0;
    break;
  case 0x96:
    FUN_0012e3b0(&DAT_001631de,*puVar18);
    param_1 = 0;
    break;
  case 0x97:
    DAT_00182b18 = FUN_001458b0();
    if (DAT_00182b18 == 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid mode given on option";
LAB_0012f9a2:
      uVar14 = dcgettext(0,pcVar17,5);
      error(0,0,uVar14);
                    /* WARNING: Subroutine does not return */
      FUN_00116fd0();
    }
    DAT_00182b14 = umask(0);
    umask(DAT_00182b14);
    param_1 = 0;
    break;
  case 0x98:
    FUN_0012e120(puVar18,"--mtime",param_2,&DAT_00182ae0);
    if (DAT_00182af0 == 0) {
      DAT_00182af0 = 1;
      param_1 = 0;
      break;
    }
    goto LAB_0012e575;
  case 0x99:
    if (-1 < DAT_00182b08) goto LAB_0012f8e5;
    pcVar17 = "--newer-mtime";
LAB_0012ea57:
    FUN_0012e120(puVar18,pcVar17,param_2,&DAT_00182b00);
    FUN_0012d480(3,*puVar18);
    param_1 = 0;
    break;
  case 0x9a:
    DAT_00182a9c = 0xffffffff;
    param_1 = 0;
    break;
  case 0x9b:
    *(undefined1 *)((long)puVar18 + 0x12) = 0;
    param_1 = 0;
    break;
  case 0x9c:
    DAT_00182b20 = 0;
    param_1 = 0;
    break;
  case 0x9d:
    DAT_001827e0 = 0;
    param_1 = 0;
    break;
  case 0x9e:
    DAT_00182a59 = 0;
    param_1 = 0;
    break;
  case 0x9f:
    FUN_0012e340(1,*puVar18);
    param_1 = 0;
    break;
  case 0xa0:
    for (; *param_2 != '\0'; param_2 = param_2 + 1) {
      FUN_0014c780(0,(int)*param_2,0);
    }
    param_1 = 0;
    break;
  case 0xa1:
    DAT_00182aa8 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa2:
    DAT_00182aa4 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa3:
    DAT_001827fc = 0;
    param_1 = 0;
    break;
  case 0xa4:
    DAT_00182aa0 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa5:
    DAT_00182a98 = 0xffffffff;
    param_1 = 0;
    break;
  case 0xa6:
    DAT_00182ad2 = 1;
    param_1 = 0;
    break;
  case 0xa7:
    FUN_0012d480(1,*puVar18);
    if (param_2 == (char *)0x0) {
      DAT_00182b38 = (char *)0x1;
      param_1 = 0;
    }
    else {
      iVar6 = FUN_0014eec0(param_2,0,10,&local_c8,&DAT_0016a7d3);
      if (iVar6 != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar14 = dcgettext(0,"Invalid number",5);
        param_2 = (char *)FUN_0014ce40(param_2);
        pcVar17 = "%s: %s";
        goto LAB_0012f89a;
      }
      DAT_00182b38 = local_c8;
      param_1 = 0;
    }
    break;
  case 0xa8:
    FUN_0012e430(&DAT_001673ec);
    param_1 = 0;
    break;
  case 0xa9:
    DAT_00182ad1 = 1;
    param_1 = 0;
    break;
  case 0xaa:
    FUN_0012d480(7,*puVar18);
    DAT_00182ad0 = 1;
    param_1 = 0;
    DAT_00182ac8 = param_2;
    break;
  case 0xab:
    FUN_0012e340(0,*puVar18);
    param_1 = 0;
    break;
  case 0xac:
    FUN_0012e340(2,*puVar18);
    param_1 = 0;
    break;
  case 0xad:
    lVar8 = FUN_0012d520(param_2,&DAT_00182ac0);
    if (lVar8 != -1) {
      DAT_00182abc = (undefined4)lVar8;
      param_1 = 0;
      break;
    }
    DAT_00182abc = 0xffffffff;
    if (DAT_00182ac0 != 0) {
      FUN_00127fd0(DAT_00182ac0,&DAT_00182abc);
      param_1 = 0;
      break;
    }
LAB_0012e575:
    param_1 = 0;
    break;
  case 0xae:
    FUN_00124de0(param_2);
    param_1 = 0;
    break;
  case 0xaf:
    _obstack_begin(&local_c8,0,0,FUN_0014ecc0,PTR_free_0017efc8);
    for (; pcVar17 = local_b0, *param_2 != '\0'; param_2 = param_2 + 1) {
      uVar9 = (long)local_a8 - (long)local_b0;
      __n = strcspn(param_2,",");
      pvVar10 = memchr(param_2,0x3d,__n);
      if (pvVar10 == (void *)0x0) {
        if (uVar9 < __n) {
          _obstack_newchunk(&local_c8,__n);
          pcVar17 = local_b0;
        }
        memcpy(pcVar17,param_2,__n);
        local_b0 = local_b0 + __n;
      }
      else {
        uVar16 = (long)pvVar10 + (1 - (long)param_2);
        if (uVar9 < uVar16) {
          _obstack_newchunk(&local_c8,uVar16);
          pcVar17 = local_b0;
        }
        memcpy(pcVar17,param_2,uVar16);
        pcVar17 = local_b0 + uVar16;
        uVar16 = __n - uVar16;
        bVar2 = *(byte *)((long)pvVar10 + 1);
        pvVar10 = (void *)((long)pvVar10 + 1);
        local_b0 = pcVar17;
        if (bVar2 != 0) {
          ppuVar11 = __ctype_b_loc();
          do {
            if ((*(byte *)((long)*ppuVar11 + (ulong)bVar2 * 2 + 1) & 0x20) == 0) {
              if ((bVar2 == 0x7b) && (*(char *)((long)pvVar10 + (uVar16 - 1)) == '}')) {
                __dest = (void *)FUN_0014ecc0(uVar16);
                memcpy(__dest,(void *)((long)pvVar10 + 1),uVar16 - 2);
                *(undefined1 *)((long)__dest + (uVar16 - 2)) = 0;
                iVar6 = FUN_0012e120(puVar18,"--pax-option",__dest,&local_d8);
                if (iVar6 == 0) {
                  pcVar17 = (char *)FUN_001257a0(local_d8,local_d0,local_68);
                  __n_00 = strlen(pcVar17);
                  if ((ulong)((long)local_a8 - (long)local_b0) < __n_00) {
                    _obstack_newchunk(&local_c8,__n_00);
                  }
                  memcpy(local_b0,pcVar17,__n_00);
                  local_b0 = local_b0 + __n_00;
                }
                else {
                  if ((ulong)((long)local_a8 - (long)local_b0) < uVar16) {
                    _obstack_newchunk(&local_c8,uVar16);
                  }
                  memcpy(local_b0,pvVar10,uVar16);
                  local_b0 = local_b0 + uVar16;
                }
                free(__dest);
                goto LAB_0012ece9;
              }
              break;
            }
            bVar2 = *(byte *)((long)pvVar10 + 1);
            pvVar10 = (void *)((long)pvVar10 + 1);
            uVar16 = uVar16 - 1;
          } while (bVar2 != 0);
        }
        if ((ulong)((long)local_a8 - (long)pcVar17) < uVar16) {
          _obstack_newchunk(&local_c8,uVar16);
          pcVar17 = local_b0;
        }
        memcpy(pcVar17,pvVar10,uVar16);
        local_b0 = local_b0 + uVar16;
      }
LAB_0012ece9:
      param_2 = param_2 + __n;
      cVar5 = *param_2;
      if (cVar5 == '\0') break;
      if (local_a8 == local_b0) {
        _obstack_newchunk(&local_c8,1);
        cVar5 = *param_2;
      }
      *local_b0 = cVar5;
      local_b0 = local_b0 + 1;
    }
    if (local_b0 == local_a8) {
      _obstack_newchunk(&local_c8,1);
    }
    *local_b0 = '\0';
    if (local_b8 == local_b0 + 1) {
      local_78 = local_78 | 2;
    }
    pcVar17 = (char *)((ulong)(local_b0 + 1 + local_98) & ~local_98);
    local_b8 = local_a8;
    if ((ulong)((long)pcVar17 - local_c0) <= (ulong)((long)local_a8 - local_c0)) {
      local_b8 = pcVar17;
    }
    local_b0 = local_b8;
    pvVar10 = (void *)FUN_0014ee30();
    _obstack_free(&local_c8,0);
    *(undefined1 *)((long)puVar18 + 0x11) = 1;
    FUN_0011ca20(pvVar10);
    free(pvVar10);
    param_1 = 0;
    break;
  case 0xb0:
    FUN_0012e430("posix");
    param_1 = 0;
    break;
  case 0xb1:
    for (; *param_2 != '\0'; param_2 = param_2 + 1) {
      FUN_0014c780(0,(int)*param_2,1);
    }
    param_1 = 0;
    break;
  case 0xb2:
    if (PTR_s_literal_0017e6e0 != (undefined *)0x0) {
      uVar9 = 0;
      pcVar17 = PTR_s_literal_0017e6e0;
      do {
        iVar6 = strcmp(param_2,pcVar17);
        if (iVar6 == 0) {
          FUN_0014c760(0,uVar9 & 0xffffffff);
          param_1 = 0;
          goto LAB_0012e591;
        }
        uVar9 = uVar9 + 1;
        pcVar17 = (&PTR_s_literal_0017e6e0)[uVar9];
      } while (pcVar17 != (char *)0x0);
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar14 = DAT_00182dd0;
    pcVar17 = (char *)dcgettext(0,
                                "Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list."
                                ,5);
LAB_0012f89a:
    error(0,0,pcVar17,param_2,uVar14);
                    /* WARNING: Subroutine does not return */
    FUN_00116fd0();
  case 0xb3:
    iVar6 = FUN_0014eec0(param_2,0,10,&local_c8,"bBcGgkKMmPTtw");
    if (iVar6 != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid record size";
      goto LAB_0012f954;
    }
    DAT_00182b98 = local_c8;
    if (((ulong)local_c8 & 0x1ff) != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar14 = dcgettext(0,"Record size must be a multiple of %d.",5);
      error(0,0,uVar14,0x200);
                    /* WARNING: Subroutine does not return */
      FUN_0012dea0(2);
    }
    DAT_00182ba0 = (int)((ulong)local_c8 >> 9);
    param_1 = 0;
    break;
  case 0xb4:
    DAT_00182aba = 1;
    param_1 = 0;
    break;
  case 0xb5:
    DAT_00182ab8 = 1;
    param_1 = 0;
    break;
  case 0xb6:
    DAT_00182a58 = 1;
    param_1 = 0;
    break;
  case 0xb7:
    param_1 = 0;
    PTR_s__usr_local_libexec_rmt_00181c60 = param_2;
    break;
  case 0xb8:
    param_1 = 0;
    DAT_00182ab0 = param_2;
    break;
  case 0xb9:
    DAT_00182aa8 = 1;
    param_1 = 0;
    break;
  case 0xba:
    FUN_0012e430("posix");
    param_1 = 0;
    DAT_00182aa0 = 1;
    break;
  case 0xbb:
    pvVar10 = (void *)FUN_0012d400();
    __printf_chk(1,&DAT_0016a682,pvVar10);
    FUN_00140180();
    free(pvVar10);
                    /* WARNING: Subroutine does not return */
    exit(0);
  case 0xbc:
    DAT_00182a89 = 1;
    param_1 = 0;
    break;
  case 0xbd:
    FUN_001202a0();
    FUN_00140180();
                    /* WARNING: Subroutine does not return */
    exit(0);
  case 0xbe:
    DAT_001827e1 = 1;
    param_1 = 0;
    break;
  case 0xbf:
    FUN_0012e340(5,*puVar18);
    param_1 = 0;
    break;
  case 0xc0:
    lVar8 = FUN_0013a4e0("--sort",param_2,&DAT_0017d9c0,&DAT_0017fae8,4,PTR_FUN_00181dd8);
    DAT_001827e4 = *(undefined4 *)(&DAT_0017fae8 + lVar8 * 4);
    param_1 = 0;
    break;
  case 0xc1:
    lVar8 = FUN_0013a4e0("--hole-detection",param_2,&PTR_DAT_0017d990,&DAT_00169a88,4,
                         PTR_FUN_00181dd8);
    DAT_00182a88 = 1;
    DAT_00182a7c = *(undefined4 *)(&DAT_00169a88 + lVar8 * 4);
    param_1 = 0;
    break;
  case 0xc2:
    DAT_00182a88 = 1;
    uVar9 = strtoul(param_2,&local_c8,10);
    DAT_00182a84 = (undefined4)uVar9;
    if (*local_c8 == '\0') {
LAB_0012e7c4:
      param_1 = 0;
      break;
    }
    if (*local_c8 == '.') {
      uVar9 = strtoul(local_c8 + 1,&local_c8,10);
      DAT_00182a80 = (undefined4)uVar9;
      if (*local_c8 == '\0') goto LAB_0012e7c4;
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    pcVar17 = "Invalid sparse version value";
    goto LAB_0012f8ff;
  case 0xc3:
    iVar6 = FUN_0014eec0(param_2,0,10,&local_c8,&DAT_0016a7d3);
    if (iVar6 != 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid number of elements";
      goto LAB_0012f954;
    }
    DAT_00182a90 = local_c8;
    param_1 = 0;
    break;
  case 0xc4:
    DAT_00182b84 = 1;
    param_1 = 0;
    puVar18[3] = param_2;
    break;
  case 0xc5:
    FUN_0012e2a0(9);
    param_1 = 0;
    break;
  case 0xc6:
    if (param_2 == (char *)0x0) {
      DAT_00182a69 = 1;
      param_1 = 0;
    }
    else {
      iVar6 = FUN_0012ddf0(param_2);
      signal(iVar6,FUN_0012d500);
      param_1 = 0;
    }
    break;
  case 199:
    if (DAT_00182a60 != (char *)0x0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Only one --to-command option allowed";
      goto LAB_0012f8ff;
    }
    param_1 = 0;
    DAT_00182a60 = param_2;
    break;
  case 200:
    FUN_00130870(param_2);
    param_1 = 0;
    break;
  case 0xc9:
    DAT_00182b91 = 1;
    param_1 = 0;
    break;
  case 0xca:
    param_1 = 0;
    DAT_00182a48 = param_2;
    break;
  case 0xcb:
    FUN_00131ab0(param_2);
    param_1 = 0;
    break;
  case 0xcc:
    FUN_0012e430("posix");
    param_1 = 0;
    DAT_00182a98 = 1;
    break;
  case 0xcd:
  case 0xce:
    FUN_0012e430("posix");
    DAT_00182a98 = 1;
    FUN_00131fd0(param_2,param_1 == 0xce);
    param_1 = 0;
    break;
  case 0xcf:
    FUN_0012e3b0(&DAT_001631e5,*puVar18);
    param_1 = 0;
  }
LAB_0012e591:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_0012e520_caseD_47:
  DAT_00182b50 = 1;
  param_1 = 0;
  goto LAB_0012e591;
}




// Function: more_options @ 0x2fac0

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
  argp_parse(&PTR_DAT_0017f240,param_1,param_2,0x2a,0,&local_38);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: tar_stat_init @ 0x2fb40

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




// Function: tar_stat_close @ 0x2fb80

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
    FUN_00126340(*param_1);
  }
  return iVar1 == 0;
}




// Function: tar_stat_destroy @ 0x2fc00

void tar_stat_destroy(undefined8 *param_1)

{
  ulong uVar1;
  undefined8 *puVar2;
  byte bVar3;
  
  bVar3 = 0;
  FUN_0012fb80();
  FUN_0011d580(param_1[0x2c],param_1[0x2b]);
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
  FUN_0011de20(param_1 + 0x2d);
  FUN_00117680(param_1);
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




// Function: tar_timespec_cmp @ 0x2fcc0

int tar_timespec_cmp(long param_1,long param_2,long param_3,long param_4)

{
  int iVar1;
  
  iVar1 = 0;
  if (DAT_00182400 == 4) {
    iVar1 = (uint)(param_4 < param_2) - (uint)(param_2 < param_4);
  }
  return iVar1 + ((uint)(param_3 < param_1) - (uint)(param_1 < param_3)) * 2;
}




// Function: parse_xform_flags @ 0x2fd20

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




// Function: run_case_conv @ 0x2fdb0

byte * run_case_conv(uint param_1,void *param_2,ulong param_3)

{
  byte *pbVar1;
  __int32_t **pp_Var2;
  byte *pbVar3;
  byte *pbVar4;
  
  if (DAT_00182c40 < param_3) {
    DAT_00182c40 = param_3;
    DAT_00182cb8 = (byte *)FUN_0014ece0(DAT_00182cb8,param_3);
  }
  pbVar1 = DAT_00182cb8;
  memcpy(DAT_00182cb8,param_2,param_3);
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




// Function: add_char_segment @ 0x2fef0

void add_char_segment(long param_1,undefined1 param_2)

{
  undefined8 *puVar1;
  undefined1 *puVar2;
  
  puVar1 = (undefined8 *)FUN_0014ecc0(0x20);
  *puVar1 = 0;
  if (*(undefined8 **)(param_1 + 0x60) == (undefined8 *)0x0) {
    *(undefined8 **)(param_1 + 0x58) = puVar1;
  }
  else {
    **(undefined8 **)(param_1 + 0x60) = puVar1;
  }
  *(long *)(param_1 + 0x68) = *(long *)(param_1 + 0x68) + 1;
  *(undefined8 **)(param_1 + 0x60) = puVar1;
  *(undefined4 *)(puVar1 + 1) = 0;
  puVar2 = (undefined1 *)FUN_0014ecc0(2);
  puVar1[3] = 1;
  *puVar2 = param_2;
  puVar2[1] = 0;
  puVar1[2] = puVar2;
  return;
}




// Function: add_literal_segment @ 0x2ff60

void add_literal_segment(long param_1,void *param_2,long param_3)

{
  undefined8 *puVar1;
  void *pvVar2;
  size_t __n;
  
  __n = param_3 - (long)param_2;
  if (__n == 0) {
    return;
  }
  puVar1 = (undefined8 *)FUN_0014ecc0(0x20);
  *puVar1 = 0;
  if (*(undefined8 **)(param_1 + 0x60) == (undefined8 *)0x0) {
    *(undefined8 **)(param_1 + 0x58) = puVar1;
  }
  else {
    **(undefined8 **)(param_1 + 0x60) = puVar1;
  }
  *(long *)(param_1 + 0x68) = *(long *)(param_1 + 0x68) + 1;
  *(undefined8 **)(param_1 + 0x60) = puVar1;
  *(undefined4 *)(puVar1 + 1) = 0;
  pvVar2 = (void *)FUN_0014ecc0(__n + 1);
  puVar1[2] = pvVar2;
  pvVar2 = memcpy(pvVar2,param_2,__n);
  puVar1[3] = __n;
  *(undefined1 *)((long)pvVar2 + __n) = 0;
  return;
}




// Function: parse_transform_expr @ 0x30000

char * parse_transform_expr(char *param_1)

{
  char cVar1;
  undefined8 *puVar2;
  uint uVar3;
  undefined4 uVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  long lVar8;
  char *pcVar9;
  ulong uVar10;
  undefined8 *puVar11;
  undefined8 uVar12;
  undefined4 *puVar13;
  long lVar14;
  char cVar15;
  char cVar16;
  char *pcVar17;
  ulong uVar18;
  ulong uVar19;
  int iVar20;
  int iVar21;
  long in_FS_OFFSET;
  char *local_258;
  char *local_250;
  undefined1 local_248;
  char local_247;
  undefined1 local_246 [518];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar8 = FUN_0014ed80(0x70);
  lVar14 = lVar8;
  if (DAT_00182cc0 != (long *)0x0) {
    *DAT_00182cc0 = lVar8;
    lVar14 = DAT_00182cc8;
  }
  DAT_00182cc8 = lVar14;
  uVar4 = DAT_00181b60;
  DAT_00182cc0 = (long *)lVar8;
  if (*param_1 == 's') {
    cVar1 = param_1[1];
    if (cVar1 != '\0') {
      uVar19 = 2;
      cVar5 = param_1[2];
      while (cVar5 != '\0') {
        iVar6 = (int)uVar19;
        iVar20 = iVar6 + 1;
        if (cVar1 == cVar5) {
          lVar14 = (long)iVar20;
          pcVar17 = param_1 + lVar14;
          cVar15 = *pcVar17;
          iVar21 = iVar20;
          if (cVar15 != '\0') goto LAB_00130192;
          break;
        }
        if (cVar5 == '\\') {
          iVar20 = (iVar6 + 2) - (uint)(param_1[uVar19 + 1] == '\0');
        }
        uVar19 = (ulong)iVar20;
        cVar5 = param_1[uVar19];
      }
    }
    goto LAB_001307bb;
  }
  iVar6 = strncmp(param_1,"flags=",6);
  if (iVar6 != 0) goto LAB_001307bb;
  cVar1 = param_1[6];
  param_1 = param_1 + 6;
  DAT_00181b60 = 0;
  if (cVar1 != '\0') {
    puVar13 = &DAT_00181b60;
    do {
      if (cVar1 == ';') {
        param_1 = param_1 + 1;
        break;
      }
      iVar6 = FUN_0012fd20(puVar13);
      if (iVar6 != 0) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        cVar1 = *param_1;
        pcVar17 = "Unknown transform flag: %c";
LAB_00130774:
        uVar12 = dcgettext(0,pcVar17,5);
        error(0,0,uVar12,(int)cVar1);
                    /* WARNING: Subroutine does not return */
        FUN_0012dea0(2);
      }
      cVar1 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar1 != '\0');
  }
  goto LAB_001300c1;
LAB_00130250:
  if (cVar16 == ';') goto LAB_00130700;
  goto LAB_0013025a;
  while( true ) {
    if (cVar15 == '\\') {
      iVar21 = (iVar21 + 1) - (uint)(param_1[lVar14 + 1] == '\0');
    }
    iVar21 = iVar21 + 1;
    lVar14 = (long)iVar21;
    cVar15 = param_1[lVar14];
    if (cVar15 == '\0') break;
LAB_00130192:
    if (cVar15 == cVar5) {
      local_250 = param_1 + lVar14 + 1;
      *(undefined4 *)(lVar8 + 8) = 0;
      cVar5 = *local_250;
      *(undefined4 *)(lVar8 + 0xc) = uVar4;
      uVar3 = 0;
      cVar16 = cVar5;
      if ((cVar5 == '\0') || (cVar5 == ';')) goto LAB_00130250;
      uVar18 = uVar19;
      goto LAB_00130213;
    }
  }
LAB_001307bb:
  if (DAT_00182d50 != (code *)0x0) {
    (*DAT_00182d50)();
  }
  pcVar17 = "Invalid transform expression";
LAB_001307d5:
  uVar12 = dcgettext(0,pcVar17,5);
  error(0,0,uVar12);
                    /* WARNING: Subroutine does not return */
  FUN_0012dea0(2);
  while( true ) {
    cVar16 = '\0';
    if (cVar5 == '\0') goto LAB_00130250;
    iVar6 = (int)uVar18;
    if (cVar5 == ';') break;
LAB_00130213:
    iVar6 = (int)uVar18;
    pcVar9 = local_250;
    if (cVar5 == 'i') {
      uVar3 = uVar3 | 2;
      local_250 = local_250 + 1;
LAB_001301fb:
      iVar6 = (int)uVar18;
      cVar5 = pcVar9[1];
    }
    else {
      if ('i' < cVar5) {
        if (cVar5 != 'x') goto LAB_001301e2;
        uVar3 = uVar3 | 1;
        local_250 = local_250 + 1;
        goto LAB_001301fb;
      }
      if (cVar5 < ':') {
        if ('/' < cVar5) {
          uVar10 = strtoul(local_250,&local_250,0);
          uVar18 = uVar18 & 0xffffffff;
          *(int *)(lVar8 + 0x10) = (int)uVar10;
          pcVar9 = local_250 + -1;
          goto LAB_001301fb;
        }
LAB_001301e2:
        iVar6 = FUN_0012fd20(lVar8 + 0xc);
        if (iVar6 == 0) {
          pcVar9 = local_250;
          local_250 = local_250 + 1;
          goto LAB_001301fb;
        }
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        pcVar17 = "Unknown flag in transform expression: %c";
        cVar1 = *local_250;
        goto LAB_00130774;
      }
      if (cVar5 != 'g') goto LAB_001301e2;
      *(undefined4 *)(lVar8 + 8) = 1;
      cVar5 = local_250[1];
      local_250 = local_250 + 1;
    }
  }
LAB_00130700:
  local_250 = local_250 + 1;
LAB_0013025a:
  pcVar9 = (char *)FUN_0014ecc0((long)(iVar6 + -1));
  memcpy(pcVar9,param_1 + 2,(long)(iVar6 + -2));
  pcVar9[uVar19 - 2] = '\0';
  iVar7 = FUN_0015dc90(lVar8 + 0x18,pcVar9,uVar3);
  if (iVar7 != 0) {
    FUN_0015ddb0(iVar7,lVar8 + 0x18,&local_248,0x200);
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar12 = dcgettext(0,"Invalid transform expression: %s",5);
    error(0,0,uVar12,&local_248);
                    /* WARNING: Subroutine does not return */
    FUN_0012dea0(2);
  }
  if ((*pcVar9 == '^') || ((iVar6 != 2 && (pcVar9[uVar19 - 3] == '$')))) {
    *(undefined4 *)(lVar8 + 8) = 0;
  }
  iVar21 = iVar21 - iVar20;
  free(pcVar9);
  pcVar9 = (char *)FUN_0014ecc0((long)(iVar21 + 1));
  memcpy(pcVar9,pcVar17,(long)iVar21);
  pcVar9[iVar21] = '\0';
  cVar5 = *pcVar9;
  pcVar17 = pcVar9;
  while (local_258 = pcVar17, cVar5 != '\0') {
    while (cVar5 != '\\') {
      if (cVar5 == '&') {
        FUN_0012ff60(lVar8,pcVar17);
        puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
        puVar2 = *(undefined8 **)(lVar8 + 0x60);
        *puVar11 = 0;
        if (puVar2 == (undefined8 *)0x0) {
          *(undefined8 **)(lVar8 + 0x58) = puVar11;
        }
        else {
          *puVar2 = puVar11;
        }
        *(undefined8 **)(lVar8 + 0x60) = puVar11;
        *(undefined4 *)(puVar11 + 1) = 1;
        puVar11[2] = 0;
        *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
        local_258 = local_258 + 1;
        pcVar17 = local_258;
      }
      else {
        local_258 = local_258 + 1;
      }
      cVar5 = *local_258;
      if (cVar5 == '\0') goto LAB_00130490;
    }
    FUN_0012ff60(lVar8,pcVar17);
    pcVar17 = local_258 + 1;
    cVar5 = local_258[1];
    local_258 = pcVar17;
    switch(cVar5) {
    case '&':
      uVar12 = 0x26;
      break;
    default:
      if (cVar5 == cVar15) {
        FUN_0012fef0(lVar8,(int)cVar1);
      }
      else {
        local_248 = 0x5c;
        local_247 = cVar5;
        FUN_0012ff60(lVar8,&local_248,local_246);
      }
      local_258 = local_258 + 1;
      goto LAB_00130480;
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
      uVar19 = strtoul(pcVar17,&local_258,10);
      if (uVar19 <= *(ulong *)(lVar8 + 0x48)) {
        puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
        puVar2 = *(undefined8 **)(lVar8 + 0x60);
        *puVar11 = 0;
        if (puVar2 == (undefined8 *)0x0) {
          *(undefined8 **)(lVar8 + 0x58) = puVar11;
        }
        else {
          *puVar2 = puVar11;
        }
        *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
        *(undefined8 **)(lVar8 + 0x60) = puVar11;
        *(undefined4 *)(puVar11 + 1) = 1;
        puVar11[2] = uVar19;
        goto LAB_00130480;
      }
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      pcVar17 = "Invalid transform replacement: back reference out of range";
      goto LAB_001307d5;
    case 'E':
      puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
      puVar2 = *(undefined8 **)(lVar8 + 0x60);
      *puVar11 = 0;
      if (puVar2 == (undefined8 *)0x0) {
        *(undefined8 **)(lVar8 + 0x58) = puVar11;
      }
      else {
        *puVar2 = puVar11;
      }
      *(undefined8 **)(lVar8 + 0x60) = puVar11;
      *(undefined4 *)(puVar11 + 1) = 2;
      *(undefined4 *)(puVar11 + 2) = 0;
      *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
      local_258 = local_258 + 1;
      goto LAB_00130480;
    case 'L':
      puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
      puVar2 = *(undefined8 **)(lVar8 + 0x60);
      *puVar11 = 0;
      if (puVar2 == (undefined8 *)0x0) {
        *(undefined8 **)(lVar8 + 0x58) = puVar11;
      }
      else {
        *puVar2 = puVar11;
      }
      *(undefined8 **)(lVar8 + 0x60) = puVar11;
      *(undefined4 *)(puVar11 + 1) = 2;
      *(undefined4 *)(puVar11 + 2) = 4;
      *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
      local_258 = local_258 + 1;
      goto LAB_00130480;
    case 'U':
      puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
      puVar2 = *(undefined8 **)(lVar8 + 0x60);
      *puVar11 = 0;
      if (puVar2 == (undefined8 *)0x0) {
        *(undefined8 **)(lVar8 + 0x58) = puVar11;
      }
      else {
        *puVar2 = puVar11;
      }
      *(undefined8 **)(lVar8 + 0x60) = puVar11;
      *(undefined4 *)(puVar11 + 1) = 2;
      *(undefined4 *)(puVar11 + 2) = 3;
      *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
      local_258 = local_258 + 1;
      goto LAB_00130480;
    case '\\':
      uVar12 = 0x5c;
      break;
    case 'a':
      uVar12 = 7;
      break;
    case 'b':
      uVar12 = 8;
      break;
    case 'f':
      uVar12 = 0xc;
      break;
    case 'l':
      puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
      puVar2 = *(undefined8 **)(lVar8 + 0x60);
      *puVar11 = 0;
      if (puVar2 == (undefined8 *)0x0) {
        *(undefined8 **)(lVar8 + 0x58) = puVar11;
      }
      else {
        *puVar2 = puVar11;
      }
      *(undefined8 **)(lVar8 + 0x60) = puVar11;
      *(undefined4 *)(puVar11 + 1) = 2;
      *(undefined4 *)(puVar11 + 2) = 2;
      *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
      local_258 = local_258 + 1;
      goto LAB_00130480;
    case 'n':
      uVar12 = 10;
      break;
    case 'r':
      uVar12 = 0xd;
      break;
    case 't':
      uVar12 = 9;
      break;
    case 'u':
      puVar11 = (undefined8 *)FUN_0014ecc0(0x20);
      puVar2 = *(undefined8 **)(lVar8 + 0x60);
      *puVar11 = 0;
      if (puVar2 == (undefined8 *)0x0) {
        *(undefined8 **)(lVar8 + 0x58) = puVar11;
      }
      else {
        *puVar2 = puVar11;
      }
      *(undefined8 **)(lVar8 + 0x60) = puVar11;
      *(undefined4 *)(puVar11 + 1) = 2;
      *(undefined4 *)(puVar11 + 2) = 1;
      *(long *)(lVar8 + 0x68) = *(long *)(lVar8 + 0x68) + 1;
      local_258 = local_258 + 1;
      goto LAB_00130480;
    case 'v':
      uVar12 = 0xb;
    }
    FUN_0012fef0(lVar8,uVar12);
    local_258 = local_258 + 1;
LAB_00130480:
    pcVar17 = local_258;
    cVar5 = *local_258;
  }
LAB_00130490:
  FUN_0012ff60(lVar8,pcVar17);
  free(pcVar9);
  param_1 = local_250;
LAB_001300c1:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_1;
}




// Function: set_transform_expr @ 0x30870

void set_transform_expr(char *param_1)

{
  if (*param_1 == '\0') {
    return;
  }
  do {
    param_1 = (char *)FUN_00130000(param_1);
  } while (*param_1 != '\0');
  return;
}




// Function: transform_name_fp @ 0x308a0

bool transform_name_fp(undefined8 *param_1,uint param_2,code *param_3,undefined8 param_4)

{
  uint uVar1;
  ulong uVar2;
  uint uVar3;
  int iVar4;
  ulong *__ptr;
  ulong __n;
  size_t __n_00;
  char *pcVar5;
  undefined8 uVar6;
  ulong uVar7;
  uint uVar8;
  uint uVar9;
  bool bVar10;
  long *plVar11;
  long *plVar12;
  char *pcVar13;
  ulong local_78;
  uint local_58;
  
  pcVar5 = (char *)*param_1;
  if (DAT_00182c48 == '\0') {
    _obstack_begin(&DAT_00182c60,0,0,FUN_0014ecc0,PTR_free_0017efc8);
    DAT_00182c48 = '\x01';
  }
  if (DAT_00182cc8 != (long *)0x0) {
    bVar10 = false;
    plVar11 = DAT_00182cc8;
    do {
      while ((param_2 & *(uint *)((long)plVar11 + 0xc)) == 0) {
        plVar11 = (long *)*plVar11;
        if (plVar11 == (long *)0x0) goto LAB_00130b0c;
      }
      __ptr = (ulong *)FUN_0014ecc0((plVar11[9] + 1) * 0x10);
      if (*pcVar5 != '\0') {
        local_58 = 0;
        uVar8 = 0;
        local_78 = 0;
        do {
          iVar4 = FUN_0015de90(plVar11 + 3,pcVar5,plVar11[9] + 1,__ptr,0);
          if (iVar4 == 0) {
            uVar2 = *__ptr;
            __n = __ptr[1];
            if (uVar2 != 0) {
              if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < uVar2) {
                _obstack_newchunk(&DAT_00182c60,uVar2);
              }
              memcpy(DAT_00182c78,pcVar5,uVar2);
              DAT_00182c78 = DAT_00182c78 + uVar2;
            }
            local_78 = local_78 + 1;
            if ((*(uint *)(plVar11 + 2) == 0) || (*(uint *)(plVar11 + 2) <= local_78)) {
              plVar12 = (long *)plVar11[0xb];
joined_r0x00130b7f:
              uVar9 = uVar8;
              if (plVar12 != (long *)0x0) {
                do {
                  uVar3 = local_58;
                  iVar4 = (int)plVar12[1];
                  uVar8 = uVar9;
                  if (iVar4 == 1) {
                    uVar2 = __ptr[plVar12[2] * 2];
                    if ((uVar2 != 0xffffffffffffffff) &&
                       (uVar7 = (__ptr + plVar12[2] * 2)[1], uVar7 != 0xffffffffffffffff)) {
                      uVar7 = uVar7 - uVar2;
                      pcVar13 = pcVar5 + uVar2;
                      if ((uVar9 != 0) &&
                         (pcVar13 = (char *)FUN_0012fdb0(uVar9,pcVar13), uVar9 - 1 < 2)) {
                        local_58 = 0;
                        uVar9 = uVar3;
                      }
                      if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < uVar7) {
LAB_00130c72:
                        _obstack_newchunk(&DAT_00182c60,uVar7);
                      }
LAB_00130c26:
                      memcpy(DAT_00182c78,pcVar13,uVar7);
                      DAT_00182c78 = DAT_00182c78 + uVar7;
                      uVar3 = local_58;
                      uVar8 = uVar9;
                    }
                  }
                  else if (iVar4 == 2) {
                    uVar1 = *(uint *)(plVar12 + 2);
                    uVar8 = uVar1;
                    if (uVar1 < 3) {
                      if (uVar1 == 0) goto code_r0x00130bc3;
                      uVar3 = uVar9;
                      if ((local_58 != 0) && (1 < local_58 - 3)) {
                        uVar3 = local_58;
                      }
                    }
                    else {
                      uVar8 = uVar9;
                      if (uVar1 - 3 < 2) {
                        uVar8 = uVar1;
                      }
                    }
                  }
                  else if (iVar4 == 0) {
                    if (uVar9 == 0) {
                      pcVar13 = (char *)plVar12[2];
                    }
                    else {
                      pcVar13 = (char *)FUN_0012fdb0(uVar9,plVar12[2],plVar12[3]);
                      if (uVar9 - 1 < 2) {
                        local_58 = 0;
                        uVar9 = uVar3;
                      }
                    }
                    uVar7 = plVar12[3];
                    if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < uVar7) goto LAB_00130c72;
                    goto LAB_00130c26;
                  }
                  local_58 = uVar3;
                  plVar12 = (long *)*plVar12;
                  uVar9 = uVar8;
                  if (plVar12 == (long *)0x0) break;
                } while( true );
              }
              goto LAB_00130a33;
            }
            if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < __n) {
              _obstack_newchunk(&DAT_00182c60,__n);
            }
            pcVar13 = pcVar5 + __n;
            memcpy(DAT_00182c78,pcVar5,__n);
            DAT_00182c78 = DAT_00182c78 + __n;
          }
          else {
            __n = strlen(pcVar5);
            if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < __n) {
              _obstack_newchunk(&DAT_00182c60,__n);
            }
            memcpy(DAT_00182c78,pcVar5,__n);
            DAT_00182c78 = DAT_00182c78 + __n;
LAB_00130a33:
            pcVar13 = pcVar5 + __n;
            if ((int)plVar11[1] == 0) {
              __n_00 = strlen(pcVar13);
              if ((ulong)((long)DAT_00182c80 - (long)DAT_00182c78) < __n_00) {
                _obstack_newchunk(&DAT_00182c60,__n_00);
              }
              memcpy(DAT_00182c78,pcVar13,__n_00);
              DAT_00182c78 = DAT_00182c78 + __n_00;
              if (DAT_00182c80 == DAT_00182c78) goto LAB_00130d94;
              goto LAB_00130a8c;
            }
          }
          pcVar5 = pcVar13;
        } while (*pcVar13 != '\0');
      }
      if (DAT_00182c80 == DAT_00182c78) {
LAB_00130d94:
        _obstack_newchunk(&DAT_00182c60,1);
      }
LAB_00130a8c:
      pcVar5 = DAT_00182c78 + 1;
      *DAT_00182c78 = '\0';
      DAT_00182c78 = pcVar5;
      free(__ptr);
      if (DAT_00182c70 == DAT_00182c78) {
        DAT_00182cb0 = DAT_00182cb0 | 2;
      }
      bVar10 = true;
      pcVar5 = DAT_00182c78 + DAT_00182c90;
      DAT_00182c78 = DAT_00182c80;
      if ((ulong)((long)((ulong)pcVar5 & ~DAT_00182c90) - (long)DAT_00182c68) <=
          (ulong)((long)DAT_00182c80 - (long)DAT_00182c68)) {
        DAT_00182c78 = (char *)((ulong)pcVar5 & ~DAT_00182c90);
      }
      plVar11 = (long *)*plVar11;
      pcVar5 = DAT_00182c70;
      DAT_00182c70 = DAT_00182c78;
    } while (plVar11 != (long *)0x0);
LAB_00130b0c:
    if (bVar10 != false) {
      pcVar13 = pcVar5;
      if (param_3 != (code *)0x0) {
        pcVar13 = (char *)(*param_3)(pcVar5,param_4);
      }
      FUN_00125120(param_1,pcVar13);
      if ((DAT_00182c68 < pcVar5) && (pcVar5 < DAT_00182c80)) {
        DAT_00182c70 = pcVar5;
        DAT_00182c78 = pcVar5;
        return bVar10;
      }
      _obstack_free(&DAT_00182c60,pcVar5);
      return bVar10;
    }
  }
  if (param_3 != (code *)0x0) {
    *param_1 = 0;
    uVar6 = (*param_3)(pcVar5,param_4);
    FUN_00125120(param_1,uVar6);
    free(pcVar5);
  }
  return param_3 != (code *)0x0;
code_r0x00130bc3:
  plVar12 = (long *)*plVar12;
  uVar8 = 0;
  goto joined_r0x00130b7f;
}




// Function: transform_name @ 0x30e70

void transform_name(undefined8 param_1,undefined8 param_2)

{
  FUN_001308a0(param_1,param_2,0,0);
  return;
}




// Function: transform_program_p @ 0x30e80

bool transform_program_p(void)

{
  return DAT_00182cc8 != 0;
}




// Function: flush_deferred_unlinks @ 0x30e90

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
  void *__ptr;
  
  uVar2 = DAT_00182580;
  plVar7 = (long *)0x0;
  plVar8 = DAT_00182ce8;
  if (DAT_00182ce8 != (long *)0x0) {
    do {
      while ((plVar1 = (long *)*plVar8, param_1 == '\0' &&
             (plVar3 = DAT_00182ce8, DAT_00181f30 <= (ulong)plVar8[4]))) {
LAB_00130ed3:
        DAT_00182ce8 = plVar3;
        plVar7 = plVar8;
        plVar8 = plVar1;
        if (plVar1 == (long *)0x0) goto LAB_00130fa4;
      }
      FUN_00126300((int)plVar8[1]);
      if ((char)plVar8[3] != '\0') {
        pcVar6 = (char *)plVar8[2];
        if (((int)plVar8[1] == 0) ||
           ((plVar3 = DAT_00182ce8, *pcVar6 != '\0' &&
            (iVar4 = strcmp(pcVar6,"."), plVar3 = DAT_00182ce8, iVar4 != 0)))) {
          iVar4 = unlinkat(DAT_0017f234,pcVar6,0x200);
          if (iVar4 != 0) {
            piVar5 = __errno_location();
            iVar4 = *piVar5;
            plVar3 = DAT_00182ce8;
            if ((iVar4 == 0x11) || (iVar4 == 0x27)) goto LAB_00130ed3;
            if (iVar4 != 2) {
              FUN_00133180(pcVar6);
              __ptr = (void *)plVar8[2];
              goto LAB_00130f70;
            }
          }
          goto LAB_001310d8;
        }
        goto LAB_00130ed3;
      }
      iVar4 = unlinkat(DAT_0017f234,(char *)plVar8[2],0);
      if (iVar4 == 0) {
LAB_001310d8:
        __ptr = (void *)plVar8[2];
      }
      else {
        piVar5 = __errno_location();
        __ptr = (void *)plVar8[2];
        if (*piVar5 != 2) {
          FUN_00133490();
          goto LAB_001310d8;
        }
      }
LAB_00130f70:
      free(__ptr);
      DAT_00182cd8 = DAT_00182cd8 + -1;
      *plVar8 = (long)DAT_00182cd0;
      DAT_00182cd0 = plVar8;
      plVar8 = plVar7;
      plVar3 = plVar1;
      if (plVar7 == (long *)0x0) goto LAB_00130ed3;
      *plVar7 = (long)plVar1;
      plVar8 = plVar1;
    } while (plVar1 != (long *)0x0);
LAB_00130fa4:
    if (DAT_00182ce8 != (long *)0x0) {
      plVar8 = DAT_00182ce8;
      if (param_1 != '\0') {
        do {
          plVar7 = (long *)*plVar8;
          FUN_00126300((int)plVar8[1]);
          pcVar6 = (char *)plVar8[2];
          if ((((int)plVar8[1] != 0) && ((char)plVar8[3] != '\0')) &&
             ((*pcVar6 == '\0' || (iVar4 = strcmp(pcVar6,"."), iVar4 == 0)))) {
            pcVar6 = (char *)FUN_00126320();
            FUN_00126300((int)plVar8[1] + -1);
          }
          iVar4 = unlinkat(DAT_0017f234,pcVar6,0x200);
          if ((iVar4 != 0) && (piVar5 = __errno_location(), *piVar5 != 2)) {
            FUN_00133180(pcVar6);
          }
          free((void *)plVar8[2]);
          DAT_00182cd8 = DAT_00182cd8 + -1;
          *plVar8 = (long)DAT_00182cd0;
          DAT_00182cd0 = plVar8;
          plVar8 = plVar7;
        } while (plVar7 != (long *)0x0);
        DAT_00182ce0 = 0;
        DAT_00182ce8 = (long *)0x0;
      }
      goto LAB_0013108a;
    }
  }
  DAT_00182ce0 = 0;
LAB_0013108a:
  FUN_00126300(uVar2);
  return;
}




// Function: finish_deferred_unlinks @ 0x31110

void finish_deferred_unlinks(void)

{
  undefined8 *puVar1;
  
  FUN_00130e90(1);
  while (DAT_00182cd0 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*DAT_00182cd0;
    free(DAT_00182cd0);
    DAT_00182cd0 = puVar1;
  }
  return;
}




// Function: queue_deferred_unlink @ 0x31150

void queue_deferred_unlink(undefined8 param_1,char param_2)

{
  undefined8 *puVar1;
  ulong uVar2;
  undefined4 uVar3;
  undefined8 *puVar4;
  int iVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  
  puVar7 = DAT_00182cd0;
  if ((DAT_00182ce8 != (undefined8 *)0x0) && ((ulong)DAT_00182ce8[4] < DAT_00181f30)) {
    FUN_00130e90(0);
    puVar7 = DAT_00182cd0;
  }
  if (puVar7 == (undefined8 *)0x0) {
    DAT_00182cd0 = puVar7;
    puVar7 = (undefined8 *)FUN_0014ecc0(0x28);
  }
  else {
    DAT_00182cd0 = (undefined8 *)*puVar7;
    *puVar7 = 0;
  }
  uVar3 = DAT_00182580;
  *puVar7 = 0;
  *(undefined4 *)(puVar7 + 1) = uVar3;
  uVar6 = FUN_0014ee30(param_1);
  puVar7[2] = uVar6;
  FUN_001253e0(uVar6);
  uVar2 = DAT_00181f30;
  *(char *)(puVar7 + 3) = param_2;
  puVar7[4] = uVar2;
  if ((param_2 == '\0') ||
     ((*(char *)puVar7[2] != '\0' && (iVar5 = strcmp((char *)puVar7[2],"."), iVar5 != 0)))) {
    puVar1 = DAT_00182ce8;
    puVar4 = DAT_00182ce0;
    if (DAT_00182ce0 == (undefined8 *)0x0) {
      puVar4 = puVar7;
      *puVar7 = DAT_00182ce8;
      DAT_00182ce8 = puVar4;
    }
    else {
      *puVar7 = *DAT_00182ce0;
      *puVar4 = puVar7;
      puVar1 = (undefined8 *)*puVar7;
    }
  }
  else {
    puVar1 = DAT_00182ce8;
    if (DAT_00182ce8 != (undefined8 *)0x0) {
      puVar4 = DAT_00182ce8;
      puVar9 = (undefined8 *)0x0;
      do {
        puVar8 = puVar4;
        if ((*(char *)(puVar8 + 3) != '\0') &&
           (((*(char *)puVar8[2] == '\0' || (iVar5 = strcmp((char *)puVar8[2],"."), iVar5 == 0)) &&
            (*(int *)(puVar8 + 1) < *(int *)(puVar7 + 1))))) {
          if (puVar9 == (undefined8 *)0x0) {
            *puVar7 = puVar1;
            DAT_00182ce8 = puVar7;
            goto LAB_001312a7;
          }
          *puVar7 = *puVar9;
          *puVar9 = puVar7;
          puVar1 = (undefined8 *)*puVar7;
          goto joined_r0x0013130c;
        }
        puVar4 = (undefined8 *)*puVar8;
        puVar9 = puVar8;
      } while ((undefined8 *)*puVar8 != (undefined8 *)0x0);
    }
    puVar4 = DAT_00182ce0;
    if (DAT_00182ce0 == (undefined8 *)0x0) {
      *puVar7 = puVar1;
      DAT_00182ce8 = puVar7;
    }
    else {
      *puVar7 = *DAT_00182ce0;
      *puVar4 = puVar7;
      puVar1 = (undefined8 *)*puVar7;
    }
  }
joined_r0x0013130c:
  if (puVar1 == (undefined8 *)0x0) {
    DAT_00182ce0 = puVar7;
  }
LAB_001312a7:
  DAT_00182cd8 = DAT_00182cd8 + 1;
  return;
}




// Function: update_archive @ 0x31350

void update_archive(void)

{
  char *__s;
  undefined4 uVar1;
  ulong uVar2;
  char cVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  long lVar7;
  undefined8 uVar8;
  long lVar9;
  ulong uVar10;
  char *pcVar11;
  size_t sVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  ulong uVar15;
  undefined8 *puVar16;
  uint uVar17;
  long in_FS_OFFSET;
  byte bVar18;
  stat local_e8;
  undefined1 local_58 [24];
  long local_40;
  
  bVar18 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001282f0();
  FUN_0010f710(2);
  FUN_0011d4e0();
  uVar17 = 0;
LAB_001313b8:
  uVar4 = FUN_00122570(&DAT_00182408,&DAT_00182840,0);
  switch(uVar4) {
  case 0:
  case 2:
switchD_001313da_caseD_0:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    FUN_00122c60(DAT_00182408,&DAT_00182840,&DAT_00182400);
    FUN_00122460((int)*(char *)(DAT_00182408 + 0x9c),&DAT_00182840);
    DAT_00182ba4 = DAT_00182400;
    if ((DAT_00182ba8 == 8) && (lVar7 = FUN_001291d0(DAT_00182848), lVar7 != 0)) {
      FUN_00126300(*(undefined4 *)(lVar7 + 0x28));
      iVar5 = FUN_00125fc0(DAT_00182848,&local_e8);
      if (iVar5 == 0) {
        if ((local_e8.st_mode & 0xf000) == 0x4000) {
          pcVar11 = (char *)FUN_00126970(*(undefined8 *)(lVar7 + 0x10),1);
          if (pcVar11 != (char *)0x0) {
            uVar8 = FUN_001265f0(*(undefined8 *)(lVar7 + 0x10));
            cVar3 = *pcVar11;
            __s = pcVar11;
            while (cVar3 != '\0') {
              uVar1 = *(undefined4 *)(lVar7 + 0x28);
              uVar13 = FUN_00126680(uVar8,__s);
              FUN_00128210(uVar13,uVar1,0);
              sVar12 = strlen(__s);
              __s = __s + sVar12 + 1;
              cVar3 = *__s;
            }
            FUN_00126660(uVar8);
            free(pcVar11);
            FUN_00128840(lVar7);
          }
        }
        else {
          iVar5 = FUN_0012fcc0(local_e8.st_mtim.tv_sec,local_e8.st_mtim.tv_nsec,DAT_00182938);
          if (iVar5 < 1) {
            FUN_00128840(lVar7);
          }
        }
      }
    }
    FUN_00124660();
    FUN_0012fc00(&DAT_00182840);
    uVar17 = uVar4;
    goto LAB_001313b8;
  case 3:
    DAT_00181f48 = DAT_00182408;
    FUN_0012fc00(&DAT_00182840);
    break;
  case 4:
    FUN_0012fc00(&DAT_00182840);
    break;
  case 5:
    FUN_0010ccd0(DAT_00182408);
    if (uVar17 != 1) {
      if (uVar17 < 2) {
        if (DAT_00182d50 != (code *)0x0) {
          (*DAT_00182d50)();
        }
        uVar8 = dcgettext(0,"This does not look like a tar archive",5);
        error(0,0,uVar8);
      }
      else if (uVar17 != 3) {
        if (uVar17 != 4) goto switchD_001313da_default;
        goto switchD_001313da_caseD_0;
      }
    }
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar8 = dcgettext(0,"Skipping to next header",5);
    error(0,0,uVar8);
    DAT_00182d58 = 2;
    FUN_0012fc00(&DAT_00182840);
    uVar17 = uVar4;
    goto LAB_001313b8;
  default:
    goto switchD_001313da_default;
  }
  FUN_0010cc80();
  DAT_00182cf8 = 1;
  DAT_00182cf0 = DAT_00181f48;
  goto LAB_00131418;
switchD_001313da_default:
  FUN_0012fc00(&DAT_00182840);
  uVar17 = uVar4;
  goto LAB_001313b8;
LAB_00131418:
  lVar7 = FUN_00129260();
  while( true ) {
    if (lVar7 == 0) {
      FUN_00112fe0();
      FUN_0010d8d0();
      FUN_00131110();
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        FUN_00128a70();
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    pcVar11 = *(char **)(lVar7 + 0x10);
    cVar3 = FUN_001176d0(pcVar11,0);
    if ((cVar3 != '\0') ||
       ((DAT_00182b40 != '\0' && (iVar5 = FUN_0012df50(&DAT_00163e11), iVar5 == 0))))
    goto LAB_00131418;
    if (DAT_00182ba8 == 2) break;
    FUN_00114bb0(0,pcVar11,pcVar11);
    lVar7 = FUN_00129260();
  }
  iVar5 = openat(DAT_0017f234,pcVar11,0);
  if (-1 < iVar5) {
    iVar6 = fstat(iVar5,&local_e8);
    uVar2 = local_e8.st_size;
    if (iVar6 == 0) {
      for (; 0 < (long)uVar2; uVar2 = uVar2 - lVar9) {
        lVar7 = FUN_0010d360();
        uVar10 = FUN_0010cd10(lVar7);
        if ((uVar2 < uVar10) && (uVar10 = uVar2, (uVar2 & 0x1ff) != 0)) {
          uVar17 = 0x200 - ((uint)uVar2 & 0x1ff);
          puVar16 = (undefined8 *)(lVar7 + uVar2);
          if (uVar17 < 8) {
            if ((uVar17 & 4) == 0) {
              if ((uVar17 != 0) && (*(undefined1 *)puVar16 = 0, (uVar17 & 2) != 0)) {
                *(undefined2 *)((long)puVar16 + ((ulong)uVar17 - 2)) = 0;
              }
            }
            else {
              *(undefined4 *)puVar16 = 0;
              *(undefined4 *)((long)puVar16 + ((ulong)uVar17 - 4)) = 0;
            }
          }
          else {
            *puVar16 = 0;
            *(undefined8 *)((long)puVar16 + ((ulong)uVar17 - 8)) = 0;
            uVar15 = (ulong)(uVar17 + ((int)puVar16 -
                                      (int)(undefined8 *)((ulong)(puVar16 + 1) & 0xfffffffffffffff8)
                                      ) >> 3);
            puVar16 = (undefined8 *)((ulong)(puVar16 + 1) & 0xfffffffffffffff8);
            for (; uVar15 != 0; uVar15 = uVar15 - 1) {
              *puVar16 = 0;
              puVar16 = puVar16 + (ulong)bVar18 * -2 + 1;
            }
          }
        }
        lVar9 = FUN_0014d570(iVar5,lVar7,uVar10);
        if (lVar9 == -1) {
          FUN_001330b0(pcVar11,local_e8.st_size - uVar2,uVar10);
LAB_001318b8:
          if (DAT_00182d50 != (code *)0x0) {
            (*DAT_00182d50)();
          }
          uVar8 = FUN_00145370(uVar2,local_58);
          uVar13 = FUN_0014ce40(pcVar11);
          uVar14 = dcngettext(0,"%s: File shrank by %s byte","%s: File shrank by %s bytes",uVar2,5);
          error(0,0,uVar14,uVar13,uVar8);
                    /* WARNING: Subroutine does not return */
          FUN_00116fd0();
        }
        if (lVar9 == 0) goto LAB_001318b8;
        FUN_0010ccd0(lVar7 + (lVar9 - 1U & 0xfffffffffffffe00));
      }
    }
    else {
      FUN_00133410(pcVar11);
    }
    iVar5 = close(iVar5);
    if (iVar5 != 0) {
      FUN_00132d60(pcVar11);
    }
    goto LAB_00131418;
  }
  FUN_00132ea0(pcVar11);
  goto LAB_00131418;
}




// Function: utf8_convert @ 0x31920

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 utf8_convert(byte param_1,char *param_2,undefined8 *param_3)

{
  char *pcVar1;
  size_t sVar2;
  undefined8 uVar3;
  iconv_t __cd;
  long in_FS_OFFSET;
  char *local_50;
  char *local_48;
  size_t local_40;
  size_t local_38;
  long local_30;
  
  __cd = *(iconv_t *)(&DAT_00181b70 + (ulong)param_1 * 8);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (__cd == (iconv_t)0xffffffffffffffff) {
    if (param_1 == 0) {
      pcVar1 = (char *)FUN_001453d0();
      _DAT_00181b70 = iconv_open(pcVar1,"UTF-8");
    }
    else {
      pcVar1 = (char *)FUN_001453d0();
      _DAT_00181b78 = iconv_open("UTF-8",pcVar1);
    }
    __cd = *(iconv_t *)(&DAT_00181b70 + (ulong)param_1 * 8);
    if (__cd == (iconv_t)0x0) goto LAB_00131a60;
    if (__cd == (iconv_t)0xffffffffffffffff) {
      uVar3 = 0;
      goto LAB_001319ce;
    }
  }
  else if (__cd == (iconv_t)0x0) {
LAB_00131a60:
    uVar3 = FUN_0014ee30(param_2);
    *param_3 = uVar3;
    uVar3 = 1;
    goto LAB_001319ce;
  }
  local_40 = strlen(param_2);
  local_40 = local_40 + 1;
  local_38 = local_40 * 0x10 + 1;
  pcVar1 = (char *)FUN_0014ecc0();
  local_50 = param_2;
  local_48 = pcVar1;
  sVar2 = iconv(__cd,&local_50,&local_40,&local_48,&local_38);
  if (sVar2 == 0) {
    *local_48 = '\0';
    uVar3 = 1;
    *param_3 = pcVar1;
  }
  else {
    free(pcVar1);
    uVar3 = 0;
  }
LAB_001319ce:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_warning_option @ 0x31ab0

void set_warning_option(char *param_1)

{
  undefined *puVar1;
  int iVar2;
  size_t sVar3;
  long lVar4;
  
  iVar2 = strcmp(param_1,"none");
  if (iVar2 == 0) {
    DAT_00181b80 = 0;
    return;
  }
  sVar3 = strlen(param_1);
  puVar1 = PTR_FUN_00181dd8;
  if ((2 < sVar3) && (iVar2 = memcmp(param_1,&DAT_0016a08a,3), iVar2 == 0)) {
    lVar4 = FUN_0013a4e0("--warning",param_1 + 3,&PTR_s_all_001659f1_0xc_0017da80,&DAT_00181ba0,4,
                         puVar1);
    DAT_00181b80 = ~*(uint *)(&DAT_00181ba0 + lVar4 * 4) & DAT_00181b80;
    return;
  }
  lVar4 = FUN_0013a4e0("--warning",param_1,&PTR_s_all_001659f1_0xc_0017da80,&DAT_00181ba0,4,puVar1);
  DAT_00181b80 = DAT_00181b80 | *(uint *)(&DAT_00181ba0 + lVar4 * 4);
  return;
}




// Function: xattrs_masked_out @ 0x31e00

undefined8 xattrs_masked_out(char *param_1,char param_2)

{
  int iVar1;
  ulong uVar2;
  
  if (DAT_00182d28 == 0) {
    if ((param_2 != '\0') || (iVar1 = strncmp(param_1,"user.",5), iVar1 == 0)) {
LAB_00131e78:
      if ((DAT_00182d40 != 0) && (DAT_00182d48 != 0)) {
        uVar2 = 0;
        do {
          iVar1 = fnmatch(*(char **)(DAT_00182d38 + uVar2 * 8),param_1,0);
          if (iVar1 == 0) {
            return 1;
          }
          uVar2 = uVar2 + 1;
        } while (uVar2 < DAT_00182d48);
      }
      return 0;
    }
  }
  else if (DAT_00182d30 != 0) {
    uVar2 = 0;
    do {
      iVar1 = fnmatch(*(char **)(DAT_00182d20 + uVar2 * 8),param_1,0);
      if (iVar1 == 0) goto LAB_00131e78;
      uVar2 = uVar2 + 1;
    } while (uVar2 < DAT_00182d30);
  }
  return 1;
}




// Function: xattrs_acls_get @ 0x31ed0

void xattrs_acls_get(void)

{
  undefined8 uVar1;
  
  if (DAT_00182a9c < 1) {
    return;
  }
  if (DAT_00182d14 == 0) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar1 = dcgettext(0,"POSIX ACL support is not available",5);
    error(0,0,uVar1);
    DAT_00182d14 = 1;
    return;
  }
  DAT_00182d14 = 1;
  return;
}




// Function: xattrs_acls_set @ 0x31f50

void xattrs_acls_set(undefined8 param_1,undefined8 param_2,char param_3)

{
  undefined8 uVar1;
  
  if ((0 < DAT_00182a9c) && (param_3 != '2')) {
    if (DAT_00182d10 == 0) {
      if (DAT_00182d50 != (code *)0x0) {
        (*DAT_00182d50)();
      }
      uVar1 = dcgettext(0,"POSIX ACL support is not available",5);
      error(0,0,uVar1);
      DAT_00182d10 = 1;
      return;
    }
    DAT_00182d10 = 1;
    return;
  }
  return;
}




// Function: xattrs_mask_add @ 0x31fd0

void xattrs_mask_add(undefined8 param_1,char param_2)

{
  undefined1 auVar1 [16];
  ulong uVar2;
  long lVar3;
  ulong uVar4;
  long *plVar5;
  
  plVar5 = &DAT_00182d20;
  if (param_2 == '\0') {
    plVar5 = &DAT_00182d38;
  }
  uVar2 = plVar5[2];
  uVar4 = plVar5[1];
  lVar3 = *plVar5;
  if (uVar2 != uVar4) goto LAB_00131fff;
  if (uVar4 == 0) {
    plVar5[1] = 4;
    uVar4 = 4;
    if (lVar3 != 0) goto LAB_00132031;
  }
  else if (lVar3 == 0) {
    auVar1._8_8_ = 0;
    auVar1._0_8_ = uVar4;
    if ((SUB168(ZEXT816(8) * auVar1,0) < 0) || (SUB168(ZEXT816(8) * auVar1,8) != 0))
    goto LAB_0013208c;
  }
  else {
    if (0xaaaaaaaaaaaaaa9 < uVar4) {
LAB_0013208c:
                    /* WARNING: Subroutine does not return */
      FUN_00117020();
    }
LAB_00132031:
    uVar4 = uVar4 + 1 + (uVar4 >> 1);
  }
  plVar5[1] = uVar4;
  lVar3 = FUN_0014ece0(lVar3,uVar4 * 8);
  *plVar5 = lVar3;
  uVar2 = plVar5[2];
LAB_00131fff:
  plVar5[2] = uVar2 + 1;
  *(undefined8 *)(lVar3 + uVar2 * 8) = param_1;
  return;
}




// Function: xattrs_clear_setup @ 0x320a0

void xattrs_clear_setup(void)

{
  if (DAT_00182d28 != 0) {
    free(DAT_00182d20);
    if (DAT_00182d40 == 0) {
      return;
    }
    free(DAT_00182d38);
    return;
  }
  if (DAT_00182d40 == 0) {
    return;
  }
  free(DAT_00182d38);
  return;
}




// Function: xattrs_xattrs_get @ 0x32100

void xattrs_xattrs_get(undefined4 param_1,undefined8 param_2,undefined8 param_3,int param_4)

{
  char cVar1;
  ssize_t sVar2;
  char *pcVar3;
  size_t sVar4;
  int *piVar5;
  long lVar6;
  char *pcVar7;
  
  if (DAT_00182a98 < 1) {
    return;
  }
  if (DAT_00182d08 == (char *)0x0) {
    if (DAT_00181c10 == 0) {
      DAT_00181c10 = 0x80;
    }
    else if ((long)DAT_00181c10 < 0) goto LAB_0013231b;
    DAT_00182d08 = (char *)FUN_0014ece0(0);
  }
  while (pcVar7 = DAT_00182d08, param_4 == 0) {
    sVar2 = FUN_00139dd0(param_1,param_2,DAT_00182d08,DAT_00181c10);
    pcVar7 = DAT_00182d08;
    if (sVar2 != -1) goto LAB_001321e3;
LAB_00132176:
    piVar5 = __errno_location();
    if (*piVar5 != 0x22) {
      pcVar7 = "llistxattrat";
      if (param_4 != 0) {
        pcVar7 = "flistxattr";
      }
      FUN_00132bd0(pcVar7,param_2);
      return;
    }
    if (DAT_00182d08 == (char *)0x0) {
      if (DAT_00181c10 == 0) {
        DAT_00181c10 = 0x80;
      }
      else if ((long)DAT_00181c10 < 0) goto LAB_0013231b;
    }
    else {
      if (0x5555555555555553 < DAT_00181c10) goto LAB_0013231b;
      DAT_00181c10 = (DAT_00181c10 >> 1) + 1 + DAT_00181c10;
    }
    DAT_00182d08 = (char *)FUN_0014ece0();
  }
  sVar2 = flistxattr(param_4,DAT_00182d08,DAT_00181c10);
  if (sVar2 == -1) goto LAB_00132176;
LAB_001321e3:
  if (DAT_00182d00 == (void *)0x0) {
    if (DAT_00181c08 == 0) {
      DAT_00181c08 = 0x80;
    }
    else if ((long)DAT_00181c08 < 0) {
LAB_0013231b:
                    /* WARNING: Subroutine does not return */
      FUN_00117020();
    }
    DAT_00182d00 = (void *)FUN_0014ece0(0);
  }
  if (sVar2 < 1) {
    return;
  }
  pcVar3 = "fgetxattr";
  if (param_4 == 0) {
    pcVar3 = "lgetxattrat";
  }
  do {
    sVar4 = strlen(pcVar7);
    while( true ) {
      if (param_4 == 0) {
        lVar6 = FUN_001398d0(param_1,param_2,pcVar7,DAT_00182d00);
      }
      else {
        lVar6 = fgetxattr(param_4,pcVar7,DAT_00182d00,DAT_00181c08);
      }
      if (lVar6 != -1) {
        cVar1 = FUN_00131e00(pcVar7,1);
        if (cVar1 == '\0') {
          FUN_0011d5d0(param_3,pcVar7,DAT_00182d00,lVar6);
        }
        goto LAB_001322e3;
      }
      piVar5 = __errno_location();
      if (*piVar5 != 0x22) break;
      if (DAT_00182d00 == (void *)0x0) {
        if (DAT_00181c08 == 0) {
          DAT_00181c08 = 0x80;
        }
        else if ((long)DAT_00181c08 < 0) goto LAB_0013231b;
      }
      else {
        if (0x5555555555555553 < DAT_00181c08) goto LAB_0013231b;
        DAT_00181c08 = (DAT_00181c08 >> 1) + 1 + DAT_00181c08;
      }
      DAT_00182d00 = (void *)FUN_0014ece0();
    }
    if (*piVar5 != 0x3d) {
      FUN_00132bd0(pcVar3,param_2);
    }
LAB_001322e3:
    pcVar7 = pcVar7 + sVar4 + 1;
    sVar2 = (sVar2 - sVar4) + -1;
    if (sVar2 < 1) {
      return;
    }
  } while( true );
}




// Function: xattrs_selinux_get @ 0x32430

void xattrs_selinux_get(undefined8 param_1,undefined8 param_2,long param_3,int param_4)

{
  int iVar1;
  int *piVar2;
  char *pcVar3;
  
  if (DAT_00182aa0 < 1) {
    return;
  }
  if (param_4 == 0) {
    iVar1 = FUN_0014dd40();
  }
  else {
    iVar1 = FUN_0014fbc0(param_4,param_3 + 0x30);
  }
  if (iVar1 == -1) {
    piVar2 = __errno_location();
    if ((*piVar2 != 0x5f) && (*piVar2 != 0x3d)) {
      pcVar3 = "fgetfilecon";
      if (param_4 == 0) {
        pcVar3 = "lgetfileconat";
      }
      FUN_00132bd0(pcVar3,param_2);
      return;
    }
  }
  return;
}




// Function: xattrs_selinux_set @ 0x324c0

void xattrs_selinux_set(long param_1,undefined8 param_2,char param_3)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  
  if ((DAT_00182aa0 < 1) || (*(long *)(param_1 + 0x30) == 0)) {
    return;
  }
  if (param_3 == '2') {
    iVar1 = FUN_0014e180(DAT_0017f234,param_2,*(long *)(param_1 + 0x30));
    pcVar4 = "lsetfileconat";
  }
  else {
    iVar1 = FUN_0014df60();
    pcVar4 = "setfileconat";
  }
  if ((iVar1 == -1) && ((DAT_00181b80._2_1_ & 0x20) != 0)) {
    if (DAT_00182d50 != (code *)0x0) {
      (*DAT_00182d50)();
    }
    uVar2 = dcgettext(0,"%s: Cannot set SELinux context for file \'%s\'",5);
    piVar3 = __errno_location();
    error(0,*piVar3,uVar2,pcVar4,param_2);
    return;
  }
  return;
}




// Function: xattrs_xattrs_set @ 0x32570

void xattrs_xattrs_set(long param_1,undefined8 param_2,char param_3,uint param_4)

{
  long lVar1;
  char cVar2;
  int iVar3;
  size_t sVar4;
  undefined8 uVar5;
  int *piVar6;
  ulong uVar7;
  long lVar8;
  char *pcVar9;
  char *__s1;
  
  if ((0 < DAT_00182a98) && (*(long *)(param_1 + 0x158) != 0)) {
    uVar7 = 0;
    do {
      lVar1 = *(long *)(*(long *)(param_1 + 0x160) + uVar7 * 0x18);
      sVar4 = strlen("SCHILY.xattr.");
      __s1 = (char *)(lVar1 + sVar4);
      if (param_3 == '0') {
        iVar3 = strcmp(__s1,"security.capability");
        if ((iVar3 != 0) != param_4) goto LAB_00132605;
      }
      else {
LAB_00132605:
        cVar2 = FUN_00131e00(__s1,0);
        if (cVar2 == '\0') {
          lVar8 = uVar7 * 0x18 + *(long *)(param_1 + 0x160);
          lVar1 = *(long *)(lVar8 + 8);
          if (lVar1 != 0) {
            if (param_3 == '2') {
              iVar3 = FUN_001393c0(DAT_0017f234,param_2,__s1,lVar1,*(undefined8 *)(lVar8 + 0x10),0);
              pcVar9 = "lsetxattr";
            }
            else {
              iVar3 = FUN_00139140();
              pcVar9 = "setxattrat";
            }
            if ((iVar3 == -1) && ((DAT_00181b80._2_1_ & 0x20) != 0)) {
              if (DAT_00182d50 != (code *)0x0) {
                (*DAT_00182d50)();
              }
              uVar5 = dcgettext(0,"%s: Cannot set \'%s\' extended attribute for file \'%s\'",5);
              piVar6 = __errno_location();
              error(0,*piVar6,uVar5,pcVar9,__s1);
            }
          }
        }
      }
      uVar7 = uVar7 + 1;
    } while (uVar7 < *(ulong *)(param_1 + 0x158));
  }
  return;
}




// Function: xattrs_print_char @ 0x326f0

void xattrs_print_char(long param_1,undefined2 *param_2)

{
  char cVar1;
  size_t sVar2;
  ulong uVar3;
  
  if (DAT_00182a54 < 2) {
    *(undefined1 *)param_2 = 0;
    return;
  }
  if (((0 < DAT_00182a98) || (0 < DAT_00182aa0)) || (0 < DAT_00182a9c)) {
    *param_2 = 0x20;
    if ((0 < DAT_00182a98) && (*(long *)(param_1 + 0x158) != 0)) {
      uVar3 = 0;
      do {
        sVar2 = strlen("SCHILY.xattr.");
        cVar1 = FUN_00131e00(sVar2 + *(long *)(*(long *)(param_1 + 0x160) + uVar3 * 0x18),0);
        if (cVar1 == '\0') {
          *(undefined1 *)param_2 = 0x2a;
          if (DAT_00182aa0 < 1) goto joined_r0x001327ea;
          goto LAB_001327a0;
        }
        uVar3 = uVar3 + 1;
      } while (uVar3 < *(ulong *)(param_1 + 0x158));
    }
    if (0 < DAT_00182aa0) {
LAB_001327a0:
      if (*(long *)(param_1 + 0x30) != 0) {
        *(undefined1 *)param_2 = 0x2e;
      }
    }
  }
joined_r0x001327ea:
  if ((0 < DAT_00182a9c) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0))))
  {
    *(undefined1 *)param_2 = 0x2b;
  }
  return;
}




// Function: xattrs_print @ 0x32830

void xattrs_print(long param_1)

{
  char cVar1;
  size_t sVar2;
  long lVar3;
  long lVar4;
  ulong uVar5;
  
  if (DAT_00182a54 < 3) {
    return;
  }
  if ((0 < DAT_00182aa0) && (*(long *)(param_1 + 0x30) != 0)) {
    __fprintf_chk(DAT_00181f28,1,"  s: %s\n");
  }
  if ((0 < DAT_00182a9c) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0))))
  {
    __fprintf_chk(DAT_00181f28,1,"  a: ");
    FUN_00131b90(&DAT_0016a7d3,*(undefined8 *)(param_1 + 0x38),*(undefined8 *)(param_1 + 0x40));
    lVar4 = *(long *)(param_1 + 0x50);
    if ((*(long *)(param_1 + 0x40) != 0) && (lVar4 != 0)) {
      __fprintf_chk(DAT_00181f28,1,&DAT_0016379c);
      lVar4 = *(long *)(param_1 + 0x50);
    }
    FUN_00131b90("default:",*(undefined8 *)(param_1 + 0x48),lVar4);
    __fprintf_chk(DAT_00181f28,1,&DAT_0016a7d2);
  }
  if ((0 < DAT_00182a98) && (*(long *)(param_1 + 0x158) != 0)) {
    uVar5 = 0;
    do {
      while( true ) {
        sVar2 = strlen("SCHILY.xattr.");
        lVar4 = uVar5 * 0x18;
        lVar3 = sVar2 + *(long *)(*(long *)(param_1 + 0x160) + uVar5 * 0x18);
        cVar1 = FUN_00131e00(lVar3,0);
        if (cVar1 != '\0') break;
        uVar5 = uVar5 + 1;
        __fprintf_chk(DAT_00181f28,1,"  x: %lu %s\n",
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



