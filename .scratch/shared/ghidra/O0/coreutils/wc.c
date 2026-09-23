// Function: avx2_supported @ 0x2daf

/* WARNING: Type propagation algorithm not settling */

undefined8 avx2_supported(void)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined4 local_20;
  uint local_1c;
  uint local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = 0;
  local_1c = 0;
  local_18[0] = 0;
  local_18[1] = 0;
  bVar2 = false;
  bVar1 = false;
  iVar3 = FUN_00102cb7(1,&local_20,&local_1c,local_18,local_18 + 1);
  if ((iVar3 != 0) && (bVar2 = true, (local_18[0] & 0x8000000) != 0)) {
    bVar1 = true;
  }
  if (bVar1) {
    local_18[1] = 0;
    local_18[0] = 0;
    local_1c = 0;
    local_20 = 0;
    iVar3 = FUN_00102d2f(7,0,&local_20,&local_1c,local_18,local_18 + 1);
    if (iVar3 == 0) {
      bVar2 = false;
    }
    else if ((local_1c & 0x20) == 0) {
      bVar1 = false;
    }
  }
  if (bVar2) {
    if (bVar1) {
      if (DAT_0010f110 != '\0') {
        uVar4 = gettext("using avx2 hardware support");
        error(0,0,&DAT_0010b206,uVar4);
      }
      uVar4 = 1;
    }
    else {
      if (DAT_0010f110 != '\0') {
        uVar4 = gettext("avx2 support not detected");
        error(0,0,&DAT_0010b206,uVar4);
      }
      uVar4 = 0;
    }
  }
  else {
    if (DAT_0010f110 != '\0') {
      uVar4 = gettext("failed to get cpuid");
      error(0,0,&DAT_0010b206,uVar4);
    }
    uVar4 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: usage @ 0x2f7b

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_0010f178;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext(
                            "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"
                            );
    printf(pcVar3,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00102978();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_001029a9(&DAT_0010b641);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: iswnbspace @ 0x30be

undefined8 iswnbspace(int param_1)

{
  undefined8 uVar1;
  
  if ((DAT_0010f158 == '\x01') ||
     ((((param_1 != 0xa0 && (param_1 != 0x2007)) && (param_1 != 0x202f)) && (param_1 != 0x2060)))) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: isnbspace @ 0x3109

void isnbspace(int param_1)

{
  wint_t wVar1;
  
  wVar1 = btowc(param_1);
  FUN_001030be(wVar1);
  return;
}




// Function: write_counts @ 0x312b

void write_counts(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,char *param_6)

{
  undefined8 uVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  char *local_30;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = "%*s";
  if (DAT_0010f140 != '\0') {
    uVar1 = FUN_00105729(param_1,local_28);
    printf("%*s",(ulong)DAT_0010f148,uVar1);
    local_30 = " %*s";
  }
  if (DAT_0010f141 != '\0') {
    uVar1 = FUN_00105729(param_2,local_28);
    printf(local_30,(ulong)DAT_0010f148,uVar1);
    local_30 = " %*s";
  }
  if (DAT_0010f142 != '\0') {
    uVar1 = FUN_00105729(param_3,local_28);
    printf(local_30,(ulong)DAT_0010f148,uVar1);
    local_30 = " %*s";
  }
  if (DAT_0010f143 != '\0') {
    uVar1 = FUN_00105729(param_4,local_28);
    printf(local_30,(ulong)DAT_0010f148,uVar1);
    local_30 = " %*s";
  }
  if (DAT_0010f144 != '\0') {
    uVar1 = FUN_00105729(param_5,local_28);
    printf(local_30,(ulong)DAT_0010f148,uVar1);
  }
  if (param_6 != (char *)0x0) {
    pcVar2 = strchr(param_6,10);
    if (pcVar2 != (char *)0x0) {
      param_6 = (char *)FUN_001079a7(0,3,param_6);
    }
    printf(" %s",param_6);
  }
  putchar_unlocked(10);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wc_lines @ 0x3328

undefined8 wc_lines(undefined8 param_1,undefined4 param_2,long *param_3,long *param_4)

{
  int iVar1;
  bool bVar2;
  long lVar3;
  undefined1 *puVar4;
  undefined8 uVar5;
  int *piVar6;
  char *pcVar7;
  ulong uVar8;
  undefined1 *puVar9;
  long in_FS_OFFSET;
  long local_4058;
  long local_4050;
  char *local_4048;
  char local_4028 [24];
  undefined1 local_4010 [16368];
  long local_20;
  
  puVar4 = &stack0xfffffffffffffff0;
  do {
    puVar9 = puVar4;
    *(undefined8 *)(puVar9 + -0x1000) = *(undefined8 *)(puVar9 + -0x1000);
    puVar4 = puVar9 + -0x1000;
  } while (puVar9 + -0x1000 != local_4010);
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar2 = false;
  if ((param_3 == (long *)0x0) || (param_4 == (long *)0x0)) {
    uVar5 = 0;
  }
  else {
    local_4050 = 0;
    local_4058 = 0;
    while( true ) {
      lVar3 = local_4058;
      *(undefined8 *)(puVar9 + -0x1080) = 0x10353b;
      uVar8 = FUN_00108412(param_2,local_4028,0x4000);
      if (uVar8 == 0) break;
      if (uVar8 == 0xffffffffffffffff) {
        *(undefined8 *)(puVar9 + -0x1080) = 0x1033de;
        uVar5 = FUN_001079a7(0,3,param_1);
        *(undefined8 *)(puVar9 + -0x1080) = 0x1033e6;
        piVar6 = __errno_location();
        iVar1 = *piVar6;
        *(undefined8 *)(puVar9 + -0x1080) = 0x103403;
        error(0,iVar1,&DAT_0010b206,uVar5);
        uVar5 = 0;
        goto LAB_00103577;
      }
      local_4050 = local_4050 + uVar8;
      local_4048 = local_4028;
      pcVar7 = local_4028 + uVar8;
      if (bVar2) {
        *pcVar7 = '\n';
        while( true ) {
          *(undefined8 *)(puVar9 + -0x1080) = 0x1034c7;
          local_4048 = rawmemchr(local_4048,10);
          if (pcVar7 <= local_4048) break;
          local_4048 = local_4048 + 1;
          local_4058 = local_4058 + 1;
        }
      }
      else {
        for (; local_4048 != pcVar7; local_4048 = local_4048 + 1) {
          local_4058 = local_4058 + (int)(uint)(*local_4048 == '\n');
        }
      }
      if (uVar8 / 0xf < (ulong)(local_4058 - lVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    *param_4 = local_4050;
    *param_3 = local_4058;
    uVar5 = 1;
  }
LAB_00103577:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar9 + -0x1080) = 0x10358b;
  __stack_chk_fail();
}




// Function: wc @ 0x3591

undefined1 wc(int param_1,long param_2,int *param_3,ulong param_4)

{
  char *pcVar1;
  ushort *puVar2;
  bool bVar3;
  undefined1 *puVar4;
  char cVar5;
  byte bVar6;
  undefined1 uVar7;
  int iVar8;
  size_t sVar9;
  ulong uVar10;
  __off_t _Var11;
  undefined8 uVar12;
  int *piVar13;
  ushort **ppuVar14;
  undefined1 *puVar15;
  long in_FS_OFFSET;
  bool bVar16;
  ulong local_40e8;
  undefined1 local_40c1;
  char local_40c0;
  byte local_40bf;
  byte local_40bd;
  byte local_40ba;
  wchar_t local_40b8;
  int local_40b4;
  long local_40b0;
  long local_40a8;
  size_t local_40a0;
  long local_4098;
  long local_4090;
  ulong local_4088;
  ulong local_4080;
  size_t local_4078;
  char *local_4070;
  long local_4068;
  ulong local_4060;
  char *local_4058;
  long local_4050;
  ulong local_4048;
  long local_4040;
  mbstate_t local_4038;
  mbstate_t local_4030;
  char local_4028 [24];
  undefined1 local_4010 [16368];
  long local_20;
  
  puVar4 = &stack0xfffffffffffffff0;
  do {
    puVar15 = puVar4;
    *(undefined8 *)(puVar15 + -0x1000) = *(undefined8 *)(puVar15 + -0x1000);
    puVar4 = puVar15 + -0x1000;
  } while (puVar15 + -0x1000 != local_4010);
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_40c1 = 1;
  local_4050 = param_2;
  if (param_2 == 0) {
    *(undefined8 *)(puVar15 + -0x10e0) = 0x103604;
    local_4050 = gettext("standard input");
  }
  local_4088 = 0;
  local_40a8 = 0;
  local_4090 = 0;
  local_4098 = 0;
  local_40b0 = 0;
  *(undefined8 *)(puVar15 + -0x10e0) = 0x10365c;
  sVar9 = __ctype_get_mb_cur_max();
  if (sVar9 < 2) {
    if ((DAT_0010f143 == '\0') && (DAT_0010f142 == 0)) {
      local_40c0 = '\0';
    }
    else {
      local_40c0 = '\x01';
    }
    local_40bf = 0;
  }
  else {
    local_40c0 = DAT_0010f143;
    local_40bf = DAT_0010f142;
  }
  if ((DAT_0010f141 == '\0') && (DAT_0010f144 == '\0')) {
    bVar16 = false;
  }
  else {
    bVar16 = true;
  }
  if ((((local_40c0 != '\x01') || (local_40bf != 0)) || (DAT_0010f140 != '\0')) || (bVar16)) {
    *(undefined8 *)(puVar15 + -0x10e0) = 0x10372a;
    FUN_001054b7(param_1,0,0,2);
  }
  if (((local_40c0 == '\0') || (local_40bf == 1)) || ((DAT_0010f140 == '\x01' || (bVar16)))) {
    if ((local_40bf == 1) || (bVar16)) {
      *(undefined8 *)(puVar15 + -0x10e0) = 0x103a6c;
      sVar9 = __ctype_get_mb_cur_max();
      if (sVar9 < 2) {
        local_40ba = 0;
        local_4060 = 0;
        while( true ) {
          *(undefined8 *)(puVar15 + -0x10e0) = 0x10412b;
          local_40a0 = FUN_00108412(param_1,local_4028,0x4000);
          if (local_40a0 == 0) break;
          local_4058 = local_4028;
          if (local_40a0 == 0xffffffffffffffff) {
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103f41;
            uVar12 = FUN_001079a7(0,3,local_4050);
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103f49;
            piVar13 = __errno_location();
            iVar8 = *piVar13;
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103f66;
            error(0,iVar8,&DAT_0010b206,uVar12);
            local_40c1 = 0;
            break;
          }
          local_40a8 = local_40a0 + local_40a8;
          do {
            pcVar1 = local_4058 + 1;
            cVar5 = *local_4058;
            local_4058 = pcVar1;
            switch(cVar5) {
            case '\t':
              local_4060 = (local_4060 & 0xfffffffffffffff8) + 8;
              break;
            case '\n':
              local_40b0 = local_40b0 + 1;
            case '\f':
            case '\r':
              if (local_4088 < local_4060) {
                local_4088 = local_4060;
              }
              local_4060 = 0;
              break;
            case '\v':
              break;
            default:
              *(undefined8 *)(puVar15 + -0x10e0) = 0x104050;
              ppuVar14 = __ctype_b_loc();
              puVar2 = *ppuVar14;
              cVar5 = local_4058[-1];
              *(undefined8 *)(puVar15 + -0x10e0) = 0x10406b;
              bVar6 = FUN_00102969((int)cVar5);
              if ((puVar2[bVar6] & 0x4000) != 0) {
                local_4060 = local_4060 + 1;
                *(undefined8 *)(puVar15 + -0x10e0) = 0x104090;
                ppuVar14 = __ctype_b_loc();
                puVar2 = *ppuVar14;
                cVar5 = local_4058[-1];
                *(undefined8 *)(puVar15 + -0x10e0) = 0x1040ab;
                bVar6 = FUN_00102969((int)cVar5);
                if ((puVar2[bVar6] & 0x2000) == 0) {
                  cVar5 = local_4058[-1];
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x1040df;
                  uVar7 = FUN_00102969((int)cVar5);
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x1040e9;
                  iVar8 = FUN_00103109(uVar7);
                  if (iVar8 == 0) {
                    local_40ba = 1;
                    goto LAB_001040f9;
                  }
                }
                break;
              }
              goto LAB_001040f9;
            case ' ':
              local_4060 = local_4060 + 1;
            }
            local_4098 = local_4098 + (ulong)local_40ba;
            local_40ba = 0;
LAB_001040f9:
            local_40a0 = local_40a0 - 1;
          } while (local_40a0 != 0);
        }
        if (local_4088 < local_4060) {
          local_4088 = local_4060;
        }
        local_4098 = local_4098 + (ulong)local_40ba;
      }
      else {
        local_40bd = 0;
        local_4080 = 0;
        local_4038.__count = 0;
        local_4038.__value = (_union_27)0x0;
        bVar16 = false;
        local_4078 = 0;
        while( true ) {
          *(undefined8 *)(puVar15 + -0x10e0) = 0x103eb3;
          local_40a0 = FUN_00108412(param_1,local_4028 + local_4078,0x4000 - local_4078);
          if (local_40a0 == 0) break;
          if (local_40a0 == 0xffffffffffffffff) {
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103acd;
            uVar12 = FUN_001079a7(0,3,local_4050);
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103ad5;
            piVar13 = __errno_location();
            iVar8 = *piVar13;
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103af2;
            error(0,iVar8,&DAT_0010b206,uVar12);
            local_40c1 = 0;
            break;
          }
          local_40a8 = local_40a0 + local_40a8;
          local_4070 = local_4028;
          local_40a0 = local_40a0 + local_4078;
          do {
            bVar3 = true;
            if (bVar16) {
LAB_00103b89:
              bVar16 = true;
              local_4030 = local_4038;
              *(undefined8 *)(puVar15 + -0x10e0) = 0x103bc2;
              local_4068 = FUN_001058d8(&local_40b8,local_4070,local_40a0,&local_4038);
              if (local_4068 != -2) {
                if (local_4068 != -1) {
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x103c14;
                  iVar8 = mbsinit(&local_4038);
                  bVar16 = iVar8 == 0;
                  if (local_4068 == 0) {
                    local_40b8 = L'\0';
                    local_4068 = 1;
                  }
                  goto LAB_00103c3e;
                }
                local_4070 = local_4070 + 1;
                local_40a0 = local_40a0 - 1;
                goto LAB_00103e20;
              }
              local_4038 = local_4030;
              break;
            }
            cVar5 = *local_4070;
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103b5b;
            cVar5 = FUN_00105893((int)cVar5);
            if (cVar5 == '\0') goto LAB_00103b89;
            local_4068 = 1;
            local_40b8 = (wchar_t)*local_4070;
            bVar3 = false;
LAB_00103c3e:
            switch(local_40b8) {
            case L'\t':
              local_4080 = (local_4080 & 0xfffffffffffffff8) + 8;
              break;
            case L'\n':
              local_40b0 = local_40b0 + 1;
            case L'\f':
            case L'\r':
              if (local_4088 < local_4080) {
                local_4088 = local_4080;
              }
              local_4080 = 0;
              break;
            case L'\v':
              break;
            default:
              if (bVar3) {
                *(undefined8 *)(puVar15 + -0x10e0) = 0x103d06;
                iVar8 = iswprint(local_40b8);
                if (iVar8 == 0) goto LAB_00103d72;
                if (DAT_0010f144 != '\0') {
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x103d22;
                  local_40b4 = wcwidth(local_40b8);
                  if (0 < local_40b4) {
                    local_4080 = local_4080 + (long)local_40b4;
                  }
                }
                *(undefined8 *)(puVar15 + -0x10e0) = 0x103d4d;
                iVar8 = iswspace(local_40b8);
                if (iVar8 != 0) break;
                *(undefined8 *)(puVar15 + -0x10e0) = 0x103d5e;
                iVar8 = FUN_001030be(local_40b8);
                if (iVar8 != 0) break;
                local_40bd = 1;
              }
              else {
LAB_00103d72:
                if (!bVar3) {
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x103d85;
                  ppuVar14 = __ctype_b_loc();
                  puVar2 = *ppuVar14;
                  cVar5 = *local_4070;
                  *(undefined8 *)(puVar15 + -0x10e0) = 0x103d9c;
                  bVar6 = FUN_00102969((int)cVar5);
                  if ((puVar2[bVar6] & 0x4000) != 0) {
                    local_4080 = local_4080 + 1;
                    *(undefined8 *)(puVar15 + -0x10e0) = 0x103dc1;
                    ppuVar14 = __ctype_b_loc();
                    puVar2 = *ppuVar14;
                    cVar5 = *local_4070;
                    *(undefined8 *)(puVar15 + -0x10e0) = 0x103dd8;
                    bVar6 = FUN_00102969((int)cVar5);
                    if ((puVar2[bVar6] & 0x2000) != 0) break;
                    local_40bd = 1;
                  }
                }
              }
              goto LAB_00103dfc;
            case L' ':
              local_4080 = local_4080 + 1;
            }
            local_4098 = local_4098 + (ulong)local_40bd;
            local_40bd = 0;
LAB_00103dfc:
            local_4070 = local_4070 + local_4068;
            local_40a0 = local_40a0 - local_4068;
            local_4090 = local_4090 + 1;
LAB_00103e20:
          } while (local_40a0 != 0);
          if (local_40a0 != 0) {
            if (local_40a0 == 0x4000) {
              local_4070 = local_4070 + 1;
              local_40a0 = 0x3fff;
            }
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103e75;
            memmove(local_4028,local_4070,local_40a0);
          }
          local_4078 = local_40a0;
        }
        if (local_4088 < local_4080) {
          local_4088 = local_4080;
        }
        local_4098 = local_4098 + (ulong)local_40bd;
      }
    }
    else {
      *(undefined8 *)(puVar15 + -0x10e0) = 0x103a22;
      cVar5 = FUN_00102daf();
      if (cVar5 != '\0') {
        PTR_FUN_0010f010 = FUN_0010a1b1;
      }
      *(undefined8 *)(puVar15 + -0x10e0) = 0x103a5c;
      local_40c1 = (*(code *)PTR_FUN_0010f010)(local_4050,param_1,&local_40b0,&local_40a8);
    }
  }
  else {
    bVar16 = false;
    if (0 < *param_3) {
      *(undefined8 *)(puVar15 + -0x10e0) = 0x10379c;
      iVar8 = fstat(param_1,(stat *)(param_3 + 2));
      *param_3 = iVar8;
    }
    if (*param_3 == 0) {
      *(undefined8 *)(puVar15 + -0x10e0) = 0x1037c9;
      cVar5 = FUN_00102c40(param_3 + 2);
      if ((cVar5 != '\0') && (-1 < *(long *)(param_3 + 0xe))) {
        local_4048 = *(ulong *)(param_3 + 0xe);
        local_40e8 = param_4;
        if ((long)param_4 < 0) {
          *(undefined8 *)(puVar15 + -0x10e0) = 0x103818;
          local_40e8 = lseek(param_1,0,1);
        }
        if (local_4048 % DAT_0010f150 == 0) {
          if ((*(long *)(param_3 + 0x10) < 1) || (0x2000000000000000 < *(ulong *)(param_3 + 0x10)))
          {
            uVar10 = 0x201;
          }
          else {
            uVar10 = *(long *)(param_3 + 0x10) + 1;
          }
          local_4040 = local_4048 - local_4048 % uVar10;
          if ((-1 < (long)local_40e8) && ((long)local_40e8 < local_4040)) {
            *(undefined8 *)(puVar15 + -0x10e0) = 0x103914;
            _Var11 = lseek(param_1,local_4040,1);
            if (-1 < _Var11) {
              local_40a8 = local_4040 - local_40e8;
            }
          }
        }
        else {
          if (local_4048 < local_40e8) {
            local_40a8 = 0;
          }
          else {
            local_40a8 = local_4048 - local_40e8;
          }
          bVar16 = true;
        }
      }
    }
    if (!bVar16) {
      *(undefined8 *)(puVar15 + -0x10e0) = 0x10395c;
      FUN_001054b7(param_1,0,0,2);
      while( true ) {
        *(undefined8 *)(puVar15 + -0x10e0) = 0x1039e7;
        local_40a0 = FUN_00108412(param_1,local_4028,0x4000);
        if (local_40a0 == 0) break;
        if (local_40a0 == 0xffffffffffffffff) {
          *(undefined8 *)(puVar15 + -0x10e0) = 0x103981;
          uVar12 = FUN_001079a7(0,3,local_4050);
          *(undefined8 *)(puVar15 + -0x10e0) = 0x103989;
          piVar13 = __errno_location();
          iVar8 = *piVar13;
          *(undefined8 *)(puVar15 + -0x10e0) = 0x1039a6;
          error(0,iVar8,&DAT_0010b206,uVar12);
          local_40c1 = 0;
          break;
        }
        local_40a8 = local_40a0 + local_40a8;
      }
    }
  }
  if (local_40bf < DAT_0010f142) {
    local_4090 = local_40a8;
  }
  *(undefined8 *)(puVar15 + -0x10e0) = 0x1041c4;
  FUN_0010312b(local_40b0,local_4098,local_4090,local_40a8,local_4088,param_2);
  DAT_0010f118 = local_40b0 + DAT_0010f118;
  DAT_0010f120 = local_4098 + DAT_0010f120;
  DAT_0010f128 = local_4090 + DAT_0010f128;
  DAT_0010f130 = local_40a8 + DAT_0010f130;
  if (DAT_0010f138 < local_4088) {
    DAT_0010f138 = local_4088;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar15 + -0x10e0) = 0x10425d;
    __stack_chk_fail();
  }
  return local_40c1;
}




// Function: wc_file @ 0x4263

ulong wc_file(char *param_1,undefined8 param_2)

{
  byte bVar1;
  int iVar2;
  ulong uVar3;
  undefined8 uVar4;
  int *piVar5;
  
  if ((param_1 == (char *)0x0) || (iVar2 = strcmp(param_1,"-"), iVar2 == 0)) {
    DAT_0010f14c = 1;
    FUN_0010945e(0,0);
    uVar3 = FUN_00103591(0,param_1,param_2,0xffffffffffffffff);
    return uVar3;
  }
  iVar2 = open(param_1,0);
  if (iVar2 == -1) {
    uVar4 = FUN_001079a7(0,3,param_1);
    piVar5 = __errno_location();
    error(0,*piVar5,&DAT_0010b206,uVar4);
    return 0;
  }
  bVar1 = FUN_00103591(iVar2,param_1,param_2,0);
  iVar2 = close(iVar2);
  if (iVar2 != 0) {
    uVar4 = FUN_001079a7(0,3,param_1);
    piVar5 = __errno_location();
    error(0,*piVar5,&DAT_0010b206,uVar4);
    return 0;
  }
  return (ulong)bVar1;
}




// Function: get_input_fstatus @ 0x43a5

undefined4 * get_input_fstatus(ulong param_1,long param_2)

{
  int iVar1;
  ulong uVar2;
  undefined4 *puVar3;
  ulong local_18;
  
  uVar2 = param_1;
  if (param_1 == 0) {
    uVar2 = 1;
  }
  puVar3 = (undefined4 *)FUN_00108f54(uVar2,0x98);
  if ((param_1 == 0) ||
     ((param_1 == 1 &&
      ((uint)DAT_0010f144 +
       (uint)DAT_0010f140 + (uint)DAT_0010f141 + (uint)DAT_0010f142 + (uint)DAT_0010f143 == 1)))) {
    *puVar3 = 1;
  }
  else {
    for (local_18 = 0; local_18 < param_1; local_18 = local_18 + 1) {
      if ((*(long *)(param_2 + local_18 * 8) == 0) ||
         (iVar1 = strcmp(*(char **)(param_2 + local_18 * 8),"-"), iVar1 == 0)) {
        iVar1 = fstat(0,(stat *)(puVar3 + local_18 * 0x26 + 2));
      }
      else {
        iVar1 = stat(*(char **)(param_2 + local_18 * 8),(stat *)(puVar3 + local_18 * 0x26 + 2));
      }
      puVar3[local_18 * 0x26] = iVar1;
    }
  }
  return puVar3;
}




// Function: compute_number_width @ 0x4549

int compute_number_width(ulong param_1,int *param_2)

{
  int local_20;
  int local_1c;
  ulong local_18;
  ulong local_10;
  
  local_20 = 1;
  if ((param_1 != 0) && (*param_2 < 1)) {
    local_1c = 1;
    local_18 = 0;
    for (local_10 = 0; local_10 < param_1; local_10 = local_10 + 1) {
      if (param_2[local_10 * 0x26] == 0) {
        if ((param_2[local_10 * 0x26 + 8] & 0xf000U) == 0x8000) {
          local_18 = local_18 + *(long *)(param_2 + local_10 * 0x26 + 0xe);
        }
        else {
          local_1c = 7;
        }
      }
    }
    for (; 9 < local_18; local_18 = local_18 / 10) {
      local_20 = local_20 + 1;
    }
    if (local_20 < local_1c) {
      local_20 = local_1c;
    }
  }
  return local_20;
}




// Function: main @ 0x4670

byte main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  bool bVar2;
  char cVar3;
  byte bVar4;
  int iVar5;
  char *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long lVar9;
  ulong uVar10;
  int *piVar11;
  long in_FS_OFFSET;
  double dVar12;
  double dVar13;
  bool local_227;
  uint local_224;
  int local_220;
  int local_21c;
  long local_218;
  undefined8 *local_210;
  char *local_208;
  long local_200;
  FILE *local_1f8;
  undefined4 *local_1f0;
  char *local_1e8;
  undefined8 local_1e0;
  long local_1d8;
  undefined8 *local_1d0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_208 = (char *)0x0;
  FUN_00105c1b(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_0010ac00(FUN_001053bf);
  iVar5 = getpagesize();
  DAT_0010f150 = (long)iVar5;
  setvbuf(stdout,(char *)0x0,1,0);
  pcVar6 = getenv("POSIXLY_CORRECT");
  DAT_0010f158 = pcVar6 != (char *)0x0;
  DAT_0010f143 = '\0';
  DAT_0010f142 = '\0';
  DAT_0010f141 = '\0';
  DAT_0010f140 = '\0';
  DAT_0010f144 = '\0';
  DAT_0010f138 = 0;
  DAT_0010f130 = 0;
  DAT_0010f128 = 0;
  DAT_0010f120 = 0;
  DAT_0010f118 = 0;
  while (local_21c = getopt_long(param_1,param_2,"clLmw",&PTR_s_bytes_0010ea20,0), local_21c != -1)
  {
    if (local_21c < 0x82) {
      if (local_21c < 0x4c) {
        if (local_21c == -0x83) {
LAB_00104899:
          FUN_00108bfb(stdout,&DAT_0010b641,"GNU coreutils",PTR_DAT_0010f018,"Paul Rubin",
                       "David MacKenzie",0);
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        if (local_21c == -0x82) {
          FUN_00102f7b(0);
          goto LAB_00104899;
        }
        goto switchD_0010483a_caseD_4d;
      }
      switch(local_21c) {
      case 0x4c:
        DAT_0010f144 = '\x01';
        break;
      default:
        goto switchD_0010483a_caseD_4d;
      case 99:
        DAT_0010f143 = '\x01';
        break;
      case 0x6c:
        DAT_0010f140 = '\x01';
        break;
      case 0x6d:
        DAT_0010f142 = '\x01';
        break;
      case 0x77:
        DAT_0010f141 = '\x01';
        break;
      case 0x80:
        DAT_0010f110 = 1;
        break;
      case 0x81:
        local_208 = optarg;
      }
    }
    else {
switchD_0010483a_caseD_4d:
      FUN_00102f7b(1);
    }
  }
  if ((((DAT_0010f140 != '\x01') && (DAT_0010f141 != '\x01')) && (DAT_0010f142 != '\x01')) &&
     ((DAT_0010f143 != '\x01' && (DAT_0010f144 != '\x01')))) {
    DAT_0010f143 = '\x01';
    DAT_0010f141 = '\x01';
    DAT_0010f140 = '\x01';
  }
  bVar2 = false;
  if (local_208 == (char *)0x0) {
    if (optind < param_1) {
      local_210 = param_2 + optind;
      local_218 = (long)(param_1 - optind);
    }
    else {
      local_210 = (undefined8 *)&DAT_0010f160;
      local_218 = 1;
    }
    local_200 = FUN_00105165(local_210);
  }
  else {
    if (optind < param_1) {
      uVar7 = FUN_00107815(4,param_2[optind]);
      uVar8 = gettext("extra operand %s");
      error(0,0,uVar8,uVar7);
      uVar7 = gettext("file operands cannot be combined with --files0-from");
      fprintf(stderr,"%s\n",uVar7);
      FUN_00102f7b(1);
    }
    iVar5 = strcmp(local_208,"-");
    if (iVar5 == 0) {
      local_1f8 = stdin;
    }
    else {
      local_1f8 = fopen(local_208,"r");
      if (local_1f8 == (FILE *)0x0) {
        uVar7 = FUN_00107815(4,local_208);
        uVar8 = gettext("cannot open %s for reading");
        piVar11 = __errno_location();
        error(1,*piVar11,uVar8,uVar7);
      }
    }
    iVar5 = fileno(local_1f8);
    iVar5 = fstat(iVar5,&local_b8);
    if ((iVar5 == 0) && ((local_b8.st_mode & 0xf000) == 0x8000)) {
      dVar12 = (double)FUN_00105aa9();
      dVar13 = DAT_0010b990;
      if (dVar12 / DAT_0010b988 <= DAT_0010b990) {
        dVar13 = (double)FUN_00105aa9();
        dVar13 = dVar13 / DAT_0010b988;
      }
      if ((double)local_b8.st_size <= dVar13) {
        bVar2 = true;
        FUN_00107c7f(&local_1d8);
        cVar3 = FUN_0010809e(local_1f8,&local_1d8);
        if ((cVar3 != '\x01') || (iVar5 = FUN_0010552b(local_1f8), iVar5 != 0)) {
          uVar7 = FUN_00107815(4,local_208);
          uVar8 = gettext("cannot read file names from %s");
          error(1,0,uVar8,uVar7);
        }
        local_210 = local_1d0;
        local_218 = local_1d8;
        local_200 = FUN_00105165(local_1d0);
        goto LAB_00104d00;
      }
    }
    local_210 = (undefined8 *)0x0;
    local_218 = 0;
    local_200 = FUN_001051ba(local_1f8);
  }
LAB_00104d00:
  if (local_200 == 0) {
    FUN_00109413();
  }
  local_1f0 = (undefined4 *)FUN_001043a5(local_218,local_210);
  DAT_0010f148 = FUN_00104549(local_218,local_1f0);
  local_227 = true;
  local_220 = 0;
  while( true ) {
    bVar1 = false;
    local_1e8 = (char *)FUN_00105227(local_200,&local_224);
    if (local_1e8 == (char *)0x0) break;
    if (((local_208 != (char *)0x0) && (iVar5 = strcmp(local_208,"-"), iVar5 == 0)) &&
       (iVar5 = strcmp(local_1e8,"-"), iVar5 == 0)) {
      uVar7 = FUN_00107815(4,local_1e8);
      uVar8 = gettext("when reading file names from stdin, no file name of %s allowed");
      error(0,0,uVar8,uVar7);
      bVar1 = true;
    }
    if (*local_1e8 == '\0') {
      if (local_208 == (char *)0x0) {
        uVar7 = gettext("invalid zero-length file name");
        error(0,0,&DAT_0010b206,uVar7);
      }
      else {
        local_1e0 = FUN_00105312(local_200);
        uVar7 = gettext("invalid zero-length file name");
        uVar8 = FUN_001079a7(0,3,local_208);
        error(0,0,"%s:%lu: %s",uVar8,local_1e0,uVar7);
      }
      bVar1 = true;
    }
    if (bVar1) {
      local_227 = false;
    }
    else {
      if (local_218 == 0) {
        lVar9 = 0;
      }
      else {
        lVar9 = (long)local_220 * 0x98;
      }
      bVar4 = FUN_00104263(local_1e8,lVar9 + (long)local_1f0);
      local_227 = (bVar4 & local_227) != 0;
    }
    if (local_218 == 0) {
      *local_1f0 = 1;
    }
    local_220 = local_220 + 1;
  }
  if (local_224 == 4) {
    uVar7 = FUN_001079a7(0,3,local_208);
    uVar8 = gettext("%s: read error");
    piVar11 = __errno_location();
    error(0,*piVar11,uVar8,uVar7);
    local_227 = false;
LAB_00105026:
    if (((local_227 != false) && (local_208 == (char *)0x0)) &&
       (lVar9 = FUN_00105312(local_200), lVar9 == 0)) {
      bVar4 = FUN_00104263(0,local_1f0);
      local_227 = (bVar4 & local_227) != 0;
    }
    if (bVar2) {
      FUN_00107d41(&local_1d8);
    }
    uVar10 = FUN_00105312(local_200);
    if (1 < uVar10) {
      uVar7 = gettext("total");
      FUN_0010312b(DAT_0010f118,DAT_0010f120,DAT_0010f128,DAT_0010f130,DAT_0010f138,uVar7);
    }
    FUN_00105350(local_200);
    free(local_1f0);
    if ((DAT_0010f14c != '\0') && (iVar5 = close(0), iVar5 != 0)) {
      piVar11 = __errno_location();
      error(1,*piVar11,&DAT_0010b718);
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_227 ^ 1;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if (local_224 < 5) {
    if (local_224 == 2) goto LAB_00105026;
    if (local_224 == 3) {
      FUN_00109413();
    }
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main");
}




// Function: wc_lines_avx2 @ 0xa1b1

undefined8 wc_lines_avx2(undefined8 param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4)

{
  int iVar1;
  undefined8 *puVar2;
  char *pcVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined8 uVar13;
  undefined8 uVar14;
  undefined1 *puVar15;
  uint uVar16;
  uint uVar17;
  uint uVar18;
  uint uVar19;
  undefined8 uVar20;
  int *piVar21;
  undefined1 *puVar22;
  long in_FS_OFFSET;
  undefined1 auVar23 [32];
  undefined1 local_4040 [16384];
  undefined1 auStack_40 [16];
  
  puVar15 = auStack_40;
  do {
    puVar22 = puVar15;
    *(undefined8 *)(puVar22 + -0x1000) = *(undefined8 *)(puVar22 + -0x1000);
    puVar15 = puVar22 + -0x1000;
  } while (puVar22 + -0x1000 != local_4040);
  *(undefined8 *)(puVar22 + -0x1358) = param_1;
  *(undefined4 *)(puVar22 + -0x135c) = param_2;
  *(undefined8 *)(puVar22 + -0x1368) = param_3;
  *(undefined8 *)(puVar22 + -0x1370) = param_4;
  *(undefined8 *)(puVar22 + 0x2ff8) = *(undefined8 *)(in_FS_OFFSET + 0x28);
  *(undefined8 *)(puVar22 + -0x1318) = 0;
  *(undefined8 *)(puVar22 + -0x1310) = 0;
  *(undefined8 *)(puVar22 + -0x1308) = 0;
  if ((*(long *)(puVar22 + -0x1368) == 0) || (*(long *)(puVar22 + -0x1370) == 0)) {
    uVar20 = 0;
  }
  else {
    *(undefined8 *)(puVar22 + -0x1260) = 0;
    *(undefined8 *)(puVar22 + -0x1258) = 0;
    *(undefined8 *)(puVar22 + -0x1250) = 0;
    *(undefined8 *)(puVar22 + -0x1248) = 0;
    *(undefined8 *)(puVar22 + -0x1240) = 0;
    *(undefined8 *)(puVar22 + -0x1238) = 0;
    *(undefined8 *)(puVar22 + -0x1230) = 0;
    *(undefined8 *)(puVar22 + -0x1228) = 0;
    *(undefined8 *)(puVar22 + -0x1220) = 0;
    *(undefined8 *)(puVar22 + -0x1218) = 0;
    *(undefined8 *)(puVar22 + -0x1210) = 0;
    *(undefined8 *)(puVar22 + -0x1208) = 0;
    puVar22[-0x1341] = 10;
    *(int *)(puVar22 + -0x1374) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1378) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x137c) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1380) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1360) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1384) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -5000) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x138c) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1390) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1394) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x1398) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x139c) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13a0) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13a4) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13a8) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13ac) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13b0) = (int)(char)puVar22[-0x1341];
    *(int *)(puVar22 + -0x13b4) = (int)(char)puVar22[-0x1341];
    puVar22[-0x1340] = puVar22[-0x1341];
    puVar22[-0x133f] = puVar22[-0x1341];
    puVar22[-0x133e] = puVar22[-0x1341];
    puVar22[-0x133d] = puVar22[-0x1341];
    puVar22[-0x133c] = puVar22[-0x1341];
    puVar22[-0x133b] = puVar22[-0x1341];
    puVar22[-0x133a] = puVar22[-0x1341];
    puVar22[-0x1339] = puVar22[-0x1341];
    puVar22[-0x1338] = puVar22[-0x1341];
    puVar22[-0x1337] = puVar22[-0x1341];
    puVar22[-0x1336] = puVar22[-0x1341];
    puVar22[-0x1335] = puVar22[-0x1341];
    puVar22[-0x1334] = puVar22[-0x1341];
    puVar22[-0x1333] = puVar22[-0x1341];
    puVar22[-0x1332] = puVar22[-0x13b4];
    puVar22[-0x1331] = puVar22[-0x13b0];
    puVar22[-0x1330] = puVar22[-0x13ac];
    puVar22[-0x132f] = puVar22[-0x13a8];
    puVar22[-0x132e] = puVar22[-0x13a4];
    puVar22[-0x132d] = puVar22[-0x13a0];
    puVar22[-0x132c] = puVar22[-0x139c];
    puVar22[-0x132b] = puVar22[-0x1398];
    puVar22[-0x132a] = puVar22[-0x1394];
    puVar22[-0x1329] = puVar22[-0x1390];
    puVar22[-0x1328] = puVar22[-0x138c];
    puVar22[-0x1327] = puVar22[-5000];
    puVar22[-0x1326] = puVar22[-0x1384];
    puVar22[-0x1325] = puVar22[-0x1360];
    puVar22[-0x1324] = puVar22[-0x1380];
    puVar22[-0x1323] = puVar22[-0x137c];
    puVar22[-0x1322] = puVar22[-0x1378];
    puVar22[-0x1321] = puVar22[-0x1374];
    auVar4 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1321]),(uint)(byte)puVar22[-0x1322],1);
    auVar5 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1323]),(uint)(byte)puVar22[-0x1324],1);
    auVar6 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1325]),(uint)(byte)puVar22[-0x1326],1);
    auVar7 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1327]),(uint)(byte)puVar22[-0x1328],1);
    auVar8 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1329]),(uint)(byte)puVar22[-0x132a],1);
    auVar9 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x132b]),(uint)(byte)puVar22[-0x132c],1);
    auVar10 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x132d]),(uint)(byte)puVar22[-0x132e],1);
    auVar11 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x132f]),(uint)(byte)puVar22[-0x1330],1);
    auVar4 = vpunpcklwd_avx(auVar4,auVar5);
    auVar5 = vpunpcklwd_avx(auVar6,auVar7);
    auVar6 = vpunpcklwd_avx(auVar8,auVar9);
    auVar7 = vpunpcklwd_avx(auVar10,auVar11);
    auVar4 = vpunpckldq_avx(auVar4,auVar5);
    auVar5 = vpunpckldq_avx(auVar6,auVar7);
    auVar12 = vpunpcklqdq_avx(auVar4,auVar5);
    auVar4 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1331]),(uint)(byte)puVar22[-0x1332],1);
    auVar5 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1333]),(uint)(byte)puVar22[-0x1334],1);
    auVar6 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1335]),(uint)(byte)puVar22[-0x1336],1);
    auVar7 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1337]),(uint)(byte)puVar22[-0x1338],1);
    auVar8 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x1339]),(uint)(byte)puVar22[-0x133a],1);
    auVar9 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x133b]),(uint)(byte)puVar22[-0x133c],1);
    auVar10 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x133d]),(uint)(byte)puVar22[-0x133e],1);
    auVar11 = vpinsrb_avx(ZEXT116((byte)puVar22[-0x133f]),(uint)(byte)puVar22[-0x1340],1);
    auVar4 = vpunpcklwd_avx(auVar4,auVar5);
    auVar5 = vpunpcklwd_avx(auVar6,auVar7);
    auVar6 = vpunpcklwd_avx(auVar8,auVar9);
    auVar7 = vpunpcklwd_avx(auVar10,auVar11);
    auVar4 = vpunpckldq_avx(auVar4,auVar5);
    auVar5 = vpunpckldq_avx(auVar6,auVar7);
    auVar4 = vpunpcklqdq_avx(auVar4,auVar5);
    auVar23._0_16_ = ZEXT116(0) * auVar4 + ZEXT116(1) * auVar12;
    auVar23._16_16_ = ZEXT116(1) * auVar4;
    *(undefined1 (*) [32])(puVar22 + -0x1200) = auVar23;
    while( true ) {
      *(undefined8 *)(puVar22 + -0x13c8) = 0x10ab8e;
      uVar20 = FUN_00108412(*(undefined4 *)(puVar22 + -0x135c),puVar22 + -0xfe0,0x3fc0);
      *(undefined8 *)(puVar22 + -0x1308) = uVar20;
      if (*(long *)(puVar22 + -0x1308) == 0) break;
      if (*(long *)(puVar22 + -0x1308) == -1) {
        *(undefined8 *)(puVar22 + -0x13c8) = 0x10a74b;
        uVar20 = FUN_001079a7(0,3,*(undefined8 *)(puVar22 + -0x1358));
        *(undefined8 *)(puVar22 + -0x13c8) = 0x10a753;
        piVar21 = __errno_location();
        iVar1 = *piVar21;
        *(undefined8 *)(puVar22 + -0x13c8) = 0x10a770;
        error(0,iVar1,&DAT_0010bfe7,uVar20);
        uVar20 = 0;
        goto LAB_0010abca;
      }
      *(long *)(puVar22 + -0x1310) = *(long *)(puVar22 + -0x1310) + *(long *)(puVar22 + -0x1308);
      *(undefined1 **)(puVar22 + -0x1320) = puVar22 + -0xfe0;
      *(undefined1 **)(puVar22 + -0x12f8) = puVar22 + *(long *)(puVar22 + -0x1308) + -0xfe0;
      while (0x3f < *(ulong *)(puVar22 + -0x1308)) {
        *(undefined8 *)(puVar22 + -0x12e8) = *(undefined8 *)(puVar22 + -0x1320);
        puVar2 = *(undefined8 **)(puVar22 + -0x12e8);
        uVar20 = puVar2[1];
        uVar13 = puVar2[2];
        uVar14 = puVar2[3];
        *(undefined8 *)(puVar22 + -0x11e0) = *puVar2;
        *(undefined8 *)(puVar22 + -0x11d8) = uVar20;
        *(undefined8 *)(puVar22 + -0x11d0) = uVar13;
        *(undefined8 *)(puVar22 + -0x11c8) = uVar14;
        *(long *)(puVar22 + -0x12f0) = *(long *)(puVar22 + -0x1320) + 0x20;
        puVar2 = *(undefined8 **)(puVar22 + -0x12f0);
        uVar20 = puVar2[1];
        uVar13 = puVar2[2];
        uVar14 = puVar2[3];
        *(undefined8 *)(puVar22 + -0x11c0) = *puVar2;
        *(undefined8 *)(puVar22 + -0x11b8) = uVar20;
        *(undefined8 *)(puVar22 + -0x11b0) = uVar13;
        *(undefined8 *)(puVar22 + -0x11a8) = uVar14;
        *(undefined8 *)(puVar22 + -0x10a0) = *(undefined8 *)(puVar22 + -0x11e0);
        *(undefined8 *)(puVar22 + -0x1098) = *(undefined8 *)(puVar22 + -0x11d8);
        *(undefined8 *)(puVar22 + -0x1090) = *(undefined8 *)(puVar22 + -0x11d0);
        *(undefined8 *)(puVar22 + -0x1088) = *(undefined8 *)(puVar22 + -0x11c8);
        *(undefined8 *)(puVar22 + -0x1080) = *(undefined8 *)(puVar22 + -0x1200);
        *(undefined8 *)(puVar22 + -0x1078) = *(undefined8 *)(puVar22 + -0x11f8);
        *(undefined8 *)(puVar22 + -0x1070) = *(undefined8 *)(puVar22 + -0x11f0);
        *(undefined8 *)(puVar22 + -0x1068) = *(undefined8 *)(puVar22 + -0x11e8);
        auVar23 = vpcmpeqb_avx2(*(undefined1 (*) [32])(puVar22 + -0x10a0),
                                *(undefined1 (*) [32])(puVar22 + -0x1080));
        *(undefined1 (*) [32])(puVar22 + -0x11a0) = auVar23;
        *(undefined8 *)(puVar22 + -0x10e0) = *(undefined8 *)(puVar22 + -0x11c0);
        *(undefined8 *)(puVar22 + -0x10d8) = *(undefined8 *)(puVar22 + -0x11b8);
        *(undefined8 *)(puVar22 + -0x10d0) = *(undefined8 *)(puVar22 + -0x11b0);
        *(undefined8 *)(puVar22 + -0x10c8) = *(undefined8 *)(puVar22 + -0x11a8);
        *(undefined8 *)(puVar22 + -0x10c0) = *(undefined8 *)(puVar22 + -0x1200);
        *(undefined8 *)(puVar22 + -0x10b8) = *(undefined8 *)(puVar22 + -0x11f8);
        *(undefined8 *)(puVar22 + -0x10b0) = *(undefined8 *)(puVar22 + -0x11f0);
        *(undefined8 *)(puVar22 + -0x10a8) = *(undefined8 *)(puVar22 + -0x11e8);
        auVar23 = vpcmpeqb_avx2(*(undefined1 (*) [32])(puVar22 + -0x10e0),
                                *(undefined1 (*) [32])(puVar22 + -0x10c0));
        *(undefined1 (*) [32])(puVar22 + -0x1180) = auVar23;
        *(undefined8 *)(puVar22 + -0x1120) = *(undefined8 *)(puVar22 + -0x1260);
        *(undefined8 *)(puVar22 + -0x1118) = *(undefined8 *)(puVar22 + -0x1258);
        *(undefined8 *)(puVar22 + -0x1110) = *(undefined8 *)(puVar22 + -0x1250);
        *(undefined8 *)(puVar22 + -0x1108) = *(undefined8 *)(puVar22 + -0x1248);
        *(undefined8 *)(puVar22 + -0x1100) = *(undefined8 *)(puVar22 + -0x11a0);
        *(undefined8 *)(puVar22 + -0x10f8) = *(undefined8 *)(puVar22 + -0x1198);
        *(undefined8 *)(puVar22 + -0x10f0) = *(undefined8 *)(puVar22 + -0x1190);
        *(undefined8 *)(puVar22 + -0x10e8) = *(undefined8 *)(puVar22 + -0x1188);
        auVar23 = vpsubb_avx2(*(undefined1 (*) [32])(puVar22 + -0x1120),
                              *(undefined1 (*) [32])(puVar22 + -0x1100));
        *(undefined1 (*) [32])(puVar22 + -0x1260) = auVar23;
        *(undefined8 *)(puVar22 + -0x1160) = *(undefined8 *)(puVar22 + -0x1240);
        *(undefined8 *)(puVar22 + -0x1158) = *(undefined8 *)(puVar22 + -0x1238);
        *(undefined8 *)(puVar22 + -0x1150) = *(undefined8 *)(puVar22 + -0x1230);
        *(undefined8 *)(puVar22 + -0x1148) = *(undefined8 *)(puVar22 + -0x1228);
        *(undefined8 *)(puVar22 + -0x1140) = *(undefined8 *)(puVar22 + -0x1180);
        *(undefined8 *)(puVar22 + -0x1138) = *(undefined8 *)(puVar22 + -0x1178);
        *(undefined8 *)(puVar22 + -0x1130) = *(undefined8 *)(puVar22 + -0x1170);
        *(undefined8 *)(puVar22 + -0x1128) = *(undefined8 *)(puVar22 + -0x1168);
        auVar23 = vpsubb_avx2(*(undefined1 (*) [32])(puVar22 + -0x1160),
                              *(undefined1 (*) [32])(puVar22 + -0x1140));
        *(undefined1 (*) [32])(puVar22 + -0x1240) = auVar23;
        *(long *)(puVar22 + -0x1320) = *(long *)(puVar22 + -0x1320) + 0x40;
        *(long *)(puVar22 + -0x1308) = *(long *)(puVar22 + -0x1308) + -0x40;
      }
      *(undefined8 *)(puVar22 + -0x1020) = *(undefined8 *)(puVar22 + -0x1260);
      *(undefined8 *)(puVar22 + -0x1018) = *(undefined8 *)(puVar22 + -0x1258);
      *(undefined8 *)(puVar22 + -0x1010) = *(undefined8 *)(puVar22 + -0x1250);
      *(undefined8 *)(puVar22 + -0x1008) = *(undefined8 *)(puVar22 + -0x1248);
      *(undefined8 *)(puVar22 + -0x1000) = *(undefined8 *)(puVar22 + -0x1220);
      *(undefined8 *)(puVar22 + -0xff8) = *(undefined8 *)(puVar22 + -0x1218);
      *(undefined8 *)(puVar22 + -0xff0) = *(undefined8 *)(puVar22 + -0x1210);
      *(undefined8 *)(puVar22 + -0xfe8) = *(undefined8 *)(puVar22 + -0x1208);
      auVar23 = vpsadbw_avx2(*(undefined1 (*) [32])(puVar22 + -0x1020),
                             *(undefined1 (*) [32])(puVar22 + -0x1000));
      *(undefined1 (*) [32])(puVar22 + -0x1260) = auVar23;
      *(undefined8 *)(puVar22 + -0x12e0) = *(undefined8 *)(puVar22 + -0x1260);
      *(undefined8 *)(puVar22 + -0x12d8) = *(undefined8 *)(puVar22 + -0x1258);
      uVar16 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x12e0),0);
      *(undefined8 *)(puVar22 + -0x12d0) = *(undefined8 *)(puVar22 + -0x1260);
      *(undefined8 *)(puVar22 + -0x12c8) = *(undefined8 *)(puVar22 + -0x1258);
      uVar17 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x12d0),4);
      *(undefined8 *)(puVar22 + -0x12c0) = *(undefined8 *)(puVar22 + -0x1250);
      *(undefined8 *)(puVar22 + -0x12b8) = *(undefined8 *)(puVar22 + -0x1248);
      uVar18 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x12c0),0);
      *(undefined8 *)(puVar22 + -0x12b0) = *(undefined8 *)(puVar22 + -0x1250);
      *(undefined8 *)(puVar22 + -0x12a8) = *(undefined8 *)(puVar22 + -0x1248);
      uVar19 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x12b0),4);
      *(long *)(puVar22 + -0x1318) =
           *(long *)(puVar22 + -0x1318) +
           (long)(int)((uVar19 & 0xffff) + (uVar16 & 0xffff) + (uVar17 & 0xffff) + (uVar18 & 0xffff)
                      );
      *(undefined8 *)(puVar22 + -0x1260) = 0;
      *(undefined8 *)(puVar22 + -0x1258) = 0;
      *(undefined8 *)(puVar22 + -0x1250) = 0;
      *(undefined8 *)(puVar22 + -0x1248) = 0;
      *(undefined8 *)(puVar22 + -0x1060) = *(undefined8 *)(puVar22 + -0x1240);
      *(undefined8 *)(puVar22 + -0x1058) = *(undefined8 *)(puVar22 + -0x1238);
      *(undefined8 *)(puVar22 + -0x1050) = *(undefined8 *)(puVar22 + -0x1230);
      *(undefined8 *)(puVar22 + -0x1048) = *(undefined8 *)(puVar22 + -0x1228);
      *(undefined8 *)(puVar22 + -0x1040) = *(undefined8 *)(puVar22 + -0x1220);
      *(undefined8 *)(puVar22 + -0x1038) = *(undefined8 *)(puVar22 + -0x1218);
      *(undefined8 *)(puVar22 + -0x1030) = *(undefined8 *)(puVar22 + -0x1210);
      *(undefined8 *)(puVar22 + -0x1028) = *(undefined8 *)(puVar22 + -0x1208);
      auVar23 = vpsadbw_avx2(*(undefined1 (*) [32])(puVar22 + -0x1060),
                             *(undefined1 (*) [32])(puVar22 + -0x1040));
      *(undefined1 (*) [32])(puVar22 + -0x1240) = auVar23;
      *(undefined8 *)(puVar22 + -0x12a0) = *(undefined8 *)(puVar22 + -0x1240);
      *(undefined8 *)(puVar22 + -0x1298) = *(undefined8 *)(puVar22 + -0x1238);
      uVar16 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x12a0),0);
      *(undefined8 *)(puVar22 + -0x1290) = *(undefined8 *)(puVar22 + -0x1240);
      *(undefined8 *)(puVar22 + -0x1288) = *(undefined8 *)(puVar22 + -0x1238);
      uVar17 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x1290),4);
      *(undefined8 *)(puVar22 + -0x1280) = *(undefined8 *)(puVar22 + -0x1230);
      *(undefined8 *)(puVar22 + -0x1278) = *(undefined8 *)(puVar22 + -0x1228);
      uVar18 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x1280),0);
      *(undefined8 *)(puVar22 + -0x1270) = *(undefined8 *)(puVar22 + -0x1230);
      *(undefined8 *)(puVar22 + -0x1268) = *(undefined8 *)(puVar22 + -0x1228);
      uVar19 = vpextrw_avx(*(undefined1 (*) [16])(puVar22 + -0x1270),4);
      *(long *)(puVar22 + -0x1318) =
           *(long *)(puVar22 + -0x1318) +
           (long)(int)((uVar19 & 0xffff) + (uVar16 & 0xffff) + (uVar17 & 0xffff) + (uVar18 & 0xffff)
                      );
      *(undefined8 *)(puVar22 + -0x1240) = 0;
      *(undefined8 *)(puVar22 + -0x1238) = 0;
      *(undefined8 *)(puVar22 + -0x1230) = 0;
      *(undefined8 *)(puVar22 + -0x1228) = 0;
      *(undefined8 *)(puVar22 + -0x1300) = *(undefined8 *)(puVar22 + -0x1320);
      while (*(long *)(puVar22 + -0x1300) != *(long *)(puVar22 + -0x12f8)) {
        pcVar3 = *(char **)(puVar22 + -0x1300);
        *(char **)(puVar22 + -0x1300) = pcVar3 + 1;
        *(long *)(puVar22 + -0x1318) =
             *(long *)(puVar22 + -0x1318) + (long)(int)(uint)(*pcVar3 == '\n');
      }
    }
    **(undefined8 **)(puVar22 + -0x1368) = *(undefined8 *)(puVar22 + -0x1318);
    **(undefined8 **)(puVar22 + -0x1370) = *(undefined8 *)(puVar22 + -0x1310);
    uVar20 = 1;
  }
LAB_0010abca:
  if (*(long *)(puVar22 + 0x2ff8) == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar20;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar22 + -0x13c8) = 0x10abe2;
  __stack_chk_fail();
}



