// Function: discard_confusing_lines @ 0x6122

void discard_confusing_lines(long param_1)

{
  void *pvVar1;
  ulong uVar2;
  void *pvVar3;
  long lVar4;
  long *plVar5;
  long lVar6;
  ulong uVar7;
  long lVar8;
  long lVar9;
  long in_FS_OFFSET;
  int local_e4;
  ulong local_e0;
  long local_d8;
  ulong local_d0;
  ulong local_c8;
  long local_c0;
  long local_b8;
  long local_b0;
  long local_a8;
  void *local_58;
  long local_50;
  void *local_48 [3];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_d8 = FUN_0011c945((*(long *)(param_1 + 0x1f8) + *(long *)(param_1 + 200)) * 0x10);
  for (local_e4 = 0; local_e4 < 2; local_e4 = local_e4 + 1) {
    *(long *)((long)local_e4 * 0x130 + param_1 + 0x100) = local_d8;
    local_d8 = local_d8 + *(long *)(param_1 + (long)local_e4 * 0x130 + 200) * 8;
    *(long *)((long)local_e4 * 0x130 + param_1 + 0x108) = local_d8;
    local_d8 = local_d8 + *(long *)(param_1 + (long)local_e4 * 0x130 + 200) * 8;
  }
  local_48[0] = (void *)FUN_00113030(*(long *)(param_1 + 0x128) << 4);
  local_48[1] = (void *)((long)local_48[0] + *(long *)(param_1 + 0x128) * 8);
  for (local_e0 = 0; (long)local_e0 < *(long *)(param_1 + 200); local_e0 = local_e0 + 1) {
    plVar5 = (long *)(*(long *)(local_e0 * 8 + *(long *)(param_1 + 0xf8)) * 8 + (long)local_48[0]);
    *plVar5 = *plVar5 + 1;
  }
  for (local_e0 = 0; (long)local_e0 < *(long *)(param_1 + 0x1f8); local_e0 = local_e0 + 1) {
    plVar5 = (long *)(*(long *)(local_e0 * 8 + *(long *)(param_1 + 0x228)) * 8 + (long)local_48[1]);
    *plVar5 = *plVar5 + 1;
  }
  local_58 = (void *)FUN_00113030(*(long *)(param_1 + 0x1f8) + *(long *)(param_1 + 200));
  local_50 = *(long *)(param_1 + 200) + (long)local_58;
  for (local_e4 = 0; local_e4 < 2; local_e4 = local_e4 + 1) {
    uVar7 = *(ulong *)(param_1 + (long)local_e4 * 0x130 + 200);
    pvVar3 = (&local_58)[local_e4];
    pvVar1 = local_48[1 - local_e4];
    lVar4 = *(long *)(param_1 + (long)local_e4 * 0x130 + 0xf8);
    local_d0 = 5;
    local_c8 = uVar7 >> 6;
    while (local_c8 = local_c8 >> 2, local_c8 != 0) {
      local_d0 = local_d0 << 1;
    }
    for (local_e0 = 0; local_e0 < uVar7; local_e0 = local_e0 + 1) {
      if (*(long *)(lVar4 + local_e0 * 8) != 0) {
        uVar2 = *(ulong *)((long)pvVar1 + *(long *)(lVar4 + local_e0 * 8) * 8);
        if (uVar2 == 0) {
          *(undefined1 *)((long)pvVar3 + local_e0) = 1;
        }
        else if (local_d0 < uVar2) {
          *(undefined1 *)((long)pvVar3 + local_e0) = 2;
        }
      }
    }
  }
  local_e4 = 0;
  do {
    if (1 < local_e4) {
      for (local_e4 = 0; local_e4 < 2; local_e4 = local_e4 + 1) {
        pvVar3 = (&local_58)[local_e4];
        lVar4 = *(long *)(param_1 + (long)local_e4 * 0x130 + 200);
        local_a8 = 0;
        for (local_e0 = 0; (long)local_e0 < lVar4; local_e0 = local_e0 + 1) {
          if ((DAT_0012d348 == '\0') && (*(char *)((long)pvVar3 + local_e0) != '\0')) {
            *(undefined1 *)(local_e0 + *(long *)(param_1 + (long)local_e4 * 0x130 + 0x118)) = 1;
          }
          else {
            *(undefined8 *)(*(long *)(param_1 + (long)local_e4 * 0x130 + 0x100) + local_a8 * 8) =
                 *(undefined8 *)(*(long *)(param_1 + (long)local_e4 * 0x130 + 0xf8) + local_e0 * 8);
            *(ulong *)(*(long *)(param_1 + (long)local_e4 * 0x130 + 0x108) + local_a8 * 8) =
                 local_e0;
            local_a8 = local_a8 + 1;
          }
        }
        *(long *)((long)local_e4 * 0x130 + param_1 + 0x110) = local_a8;
      }
      free(local_58);
      free(local_48[0]);
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    lVar4 = *(long *)(param_1 + (long)local_e4 * 0x130 + 200);
    pvVar3 = (&local_58)[local_e4];
    for (local_e0 = 0; (long)local_e0 < lVar4; local_e0 = local_e0 + 1) {
      if (*(char *)(local_e0 + (long)pvVar3) == '\x02') {
        *(undefined1 *)(local_e0 + (long)pvVar3) = 0;
      }
      else if (*(char *)(local_e0 + (long)pvVar3) != '\0') {
        local_c0 = 0;
        for (uVar7 = local_e0; ((long)uVar7 < lVar4 && (*(char *)(uVar7 + (long)pvVar3) != '\0'));
            uVar7 = uVar7 + 1) {
          if (*(char *)(uVar7 + (long)pvVar3) == '\x02') {
            local_c0 = local_c0 + 1;
          }
        }
        while (((long)local_e0 < (long)uVar7 && (*(char *)(uVar7 + -1 + (long)pvVar3) == '\x02'))) {
          uVar7 = uVar7 + -1;
          *(undefined1 *)(uVar7 + (long)pvVar3) = 0;
          local_c0 = local_c0 + -1;
        }
        lVar6 = uVar7 - local_e0;
        if (lVar6 < local_c0 * 4) {
          while ((long)local_e0 < (long)uVar7) {
            uVar7 = uVar7 + -1;
            if (*(char *)(uVar7 + (long)pvVar3) == '\x02') {
              *(undefined1 *)(uVar7 + (long)pvVar3) = 0;
            }
          }
        }
        else {
          local_b8 = 1;
          local_b0 = lVar6 >> 2;
          while (local_b0 = local_b0 >> 2, 0 < local_b0) {
            local_b8 = local_b8 << 1;
          }
          lVar9 = 0;
          for (lVar8 = 0; lVar8 < lVar6; lVar8 = lVar8 + 1) {
            if (*(char *)(local_e0 + lVar8 + (long)pvVar3) == '\x02') {
              lVar9 = lVar9 + 1;
              if (lVar9 == local_b8 + 1) {
                lVar8 = lVar8 - lVar9;
              }
              else if (local_b8 + 1 < lVar9) {
                *(undefined1 *)(local_e0 + lVar8 + (long)pvVar3) = 0;
              }
            }
            else {
              lVar9 = 0;
            }
          }
          lVar9 = 0;
          for (lVar8 = 0;
              (lVar8 < lVar6 &&
              ((lVar8 < 8 || (*(char *)(local_e0 + lVar8 + (long)pvVar3) != '\x01'))));
              lVar8 = lVar8 + 1) {
            if (*(char *)(local_e0 + lVar8 + (long)pvVar3) == '\x02') {
              lVar9 = 0;
              *(undefined1 *)(local_e0 + lVar8 + (long)pvVar3) = 0;
            }
            else if (*(char *)(local_e0 + lVar8 + (long)pvVar3) == '\0') {
              lVar9 = 0;
            }
            else {
              lVar9 = lVar9 + 1;
            }
            if (lVar9 == 3) break;
          }
          local_e0 = local_e0 + lVar6 + -1;
          lVar9 = 0;
          for (lVar8 = 0;
              (lVar8 < lVar6 &&
              ((lVar8 < 8 || (*(char *)((local_e0 - lVar8) + (long)pvVar3) != '\x01'))));
              lVar8 = lVar8 + 1) {
            if (*(char *)((local_e0 - lVar8) + (long)pvVar3) == '\x02') {
              lVar9 = 0;
              *(undefined1 *)((local_e0 - lVar8) + (long)pvVar3) = 0;
            }
            else if (*(char *)((local_e0 - lVar8) + (long)pvVar3) == '\0') {
              lVar9 = 0;
            }
            else {
              lVar9 = lVar9 + 1;
            }
            if (lVar9 == 3) break;
          }
        }
      }
    }
    local_e4 = local_e4 + 1;
  } while( true );
}




// Function: shift_boundaries @ 0x6b31

void shift_boundaries(long param_1)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  char *pcVar6;
  long lVar7;
  int local_54;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  
  for (local_54 = 0; local_54 < 2; local_54 = local_54 + 1) {
    lVar2 = *(long *)(param_1 + (long)local_54 * 0x130 + 0x118);
    lVar3 = *(long *)(param_1 + (long)(1 - local_54) * 0x130 + 0x118);
    lVar4 = *(long *)(param_1 + (long)local_54 * 0x130 + 0xf8);
    local_50 = 0;
    local_48 = 0;
    lVar5 = *(long *)(param_1 + (long)local_54 * 0x130 + 200);
    while( true ) {
      for (; (local_50 < lVar5 && (*(char *)(lVar2 + local_50) == '\0')); local_50 = local_50 + 1) {
        do {
          lVar7 = local_48 + 1;
          pcVar6 = (char *)(lVar3 + local_48);
          local_48 = lVar7;
        } while (*pcVar6 != '\0');
      }
      if (local_50 == lVar5) break;
      local_40 = local_50;
      do {
        local_50 = local_50 + 1;
      } while (*(char *)(lVar2 + local_50) != '\0');
      for (; *(char *)(lVar3 + local_48) != '\0'; local_48 = local_48 + 1) {
      }
      do {
        lVar7 = local_50 - local_40;
        while ((local_40 != 0 &&
               (*(long *)(lVar4 + local_40 * 8 + -8) == *(long *)(lVar4 + local_50 * 8 + -8)))) {
          local_40 = local_40 + -1;
          *(undefined1 *)(lVar2 + local_40) = 1;
          local_50 = local_50 + -1;
          *(undefined1 *)(lVar2 + local_50) = 0;
          for (; *(char *)(lVar2 + local_40 + -1) != '\0'; local_40 = local_40 + -1) {
          }
          do {
            local_48 = local_48 + -1;
          } while (*(char *)(lVar3 + local_48) != '\0');
        }
        local_38 = lVar5;
        if (*(char *)(lVar3 + local_48 + -1) != '\0') {
          local_38 = local_50;
        }
        while ((local_50 != lVar5 &&
               (*(long *)(lVar4 + local_40 * 8) == *(long *)(lVar4 + local_50 * 8)))) {
          lVar1 = local_40 + 1;
          *(undefined1 *)(lVar2 + local_40) = 0;
          *(undefined1 *)(lVar2 + local_50) = 1;
          do {
            local_50 = local_50 + 1;
          } while (*(char *)(lVar2 + local_50) != '\0');
          while (local_48 = local_48 + 1, local_40 = lVar1, *(char *)(lVar3 + local_48) != '\0') {
            local_38 = local_50;
          }
        }
      } while (lVar7 != local_50 - local_40);
      while (local_38 < local_50) {
        local_40 = local_40 + -1;
        *(undefined1 *)(lVar2 + local_40) = 1;
        local_50 = local_50 + -1;
        *(undefined1 *)(lVar2 + local_50) = 0;
        do {
          local_48 = local_48 + -1;
        } while (*(char *)(lVar3 + local_48) != '\0');
      }
    }
  }
  return;
}




// Function: add_change @ 0x6e92

undefined8 *
add_change(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 param_5)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0011c945(0x30);
  puVar1[3] = param_1;
  puVar1[4] = param_2;
  puVar1[1] = param_4;
  puVar1[2] = param_3;
  *puVar1 = param_5;
  return puVar1;
}




// Function: build_reverse_script @ 0x6f01

undefined8 build_reverse_script(long param_1)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  
  local_50 = 0;
  lVar1 = *(long *)(param_1 + 0x118);
  lVar2 = *(long *)(param_1 + 0x248);
  lVar3 = *(long *)(param_1 + 200);
  lVar4 = *(long *)(param_1 + 0x1f8);
  local_48 = 0;
  for (local_40 = 0; (lVar6 = local_40, lVar5 = local_48, local_48 < lVar3 || (local_40 < lVar4));
      local_40 = local_40 + 1) {
    if (*(char *)(lVar2 + local_40) != '\0' || *(char *)(lVar1 + local_48) != '\0') {
      for (; *(char *)(lVar1 + local_48) != '\0'; local_48 = local_48 + 1) {
      }
      for (; *(char *)(lVar2 + local_40) != '\0'; local_40 = local_40 + 1) {
      }
      local_50 = FUN_00106e92(lVar5,lVar6,local_48 - lVar5,local_40 - lVar6,local_50);
    }
    local_48 = local_48 + 1;
  }
  return local_50;
}




// Function: build_script @ 0x7037

undefined8 build_script(long param_1)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  
  local_40 = 0;
  lVar1 = *(long *)(param_1 + 0x118);
  lVar2 = *(long *)(param_1 + 0x248);
  local_38 = *(long *)(param_1 + 200);
  for (local_30 = *(long *)(param_1 + 0x1f8);
      (lVar4 = local_30, lVar3 = local_38, -1 < local_38 || (-1 < local_30));
      local_30 = local_30 + -1) {
    if (*(char *)(lVar2 + local_30 + -1) != '\0' || *(char *)(lVar1 + local_38 + -1) != '\0') {
      for (; *(char *)(lVar1 + local_38 + -1) != '\0'; local_38 = local_38 + -1) {
      }
      for (; *(char *)(lVar2 + local_30 + -1) != '\0'; local_30 = local_30 + -1) {
      }
      local_40 = FUN_00106e92(local_38,local_30,lVar3 - local_38,lVar4 - local_30,local_40);
    }
    local_38 = local_38 + -1;
  }
  return local_40;
}




// Function: briefly_report @ 0x7167

void briefly_report(int param_1,long param_2)

{
  undefined8 uVar1;
  long lVar2;
  long lVar3;
  
  if (param_1 != 0) {
    lVar3 = DAT_0012d208;
    if (DAT_0012d208 == 0) {
      lVar3 = *(long *)(param_2 + 0x138);
    }
    lVar2 = DAT_0012d200;
    if (DAT_0012d200 == 0) {
      lVar2 = *(long *)(param_2 + 8);
    }
    if (DAT_0012d2a0 == '\0') {
      uVar1 = gettext("Binary files %s and %s differ\n");
    }
    else {
      uVar1 = gettext("Files %s and %s differ\n");
    }
    FUN_001109a8(uVar1,lVar2,lVar3);
  }
  return;
}




// Function: diff_2_files @ 0x7206

bool diff_2_files(int *param_1)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  undefined8 uVar6;
  int *piVar7;
  undefined8 *puVar8;
  long in_FS_OFFSET;
  bool bVar9;
  byte bVar10;
  int local_d0;
  undefined1 local_c8 [8];
  undefined1 local_c0 [8];
  undefined1 local_b8 [8];
  undefined1 local_b0 [8];
  undefined8 *local_a8;
  undefined8 *local_a0;
  long local_98;
  long local_90;
  undefined8 *local_88;
  long local_80;
  void *local_78;
  undefined8 *local_70;
  undefined8 *local_68;
  undefined8 *local_60;
  undefined8 local_58;
  long local_50;
  undefined8 local_48;
  undefined8 local_40;
  long local_38;
  long local_30;
  undefined1 local_28;
  long local_20;
  long local_10;
  
  bVar10 = 0;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0010fa13(param_1,DAT_0012d1ed);
  if (cVar1 == '\0') {
    local_80 = *(long *)(param_1 + 0x7e) + *(long *)(param_1 + 0x32) + 4;
    local_78 = (void *)FUN_00113030(local_80);
    *(long *)(param_1 + 0x46) = (long)local_78 + 1;
    *(long *)(param_1 + 0x92) = *(long *)(param_1 + 0x32) + 3 + (long)local_78;
    FUN_00106122(param_1);
    local_48 = *(undefined8 *)(param_1 + 0x40);
    local_40 = *(undefined8 *)(param_1 + 0x8c);
    local_98 = *(long *)(param_1 + 0x90) + *(long *)(param_1 + 0x44) + 3;
    lVar4 = FUN_0011c945(local_98 * 0x10);
    local_38 = (*(long *)(param_1 + 0x90) + 1) * 8 + lVar4;
    local_30 = (*(long *)(param_1 + 0x90) + 1) * 8 + local_98 * 8 + lVar4;
    local_28 = DAT_0012d338;
    local_90 = 1;
    for (; local_98 != 0; local_98 = local_98 >> 2) {
      local_90 = local_90 << 1;
    }
    local_20 = local_90;
    if (local_90 < 0x1000) {
      local_20 = 0x1000;
    }
    piVar7 = param_1;
    puVar8 = &DAT_0012d360;
    for (lVar4 = 0x26; lVar4 != 0; lVar4 = lVar4 + -1) {
      *puVar8 = *(undefined8 *)piVar7;
      piVar7 = piVar7 + (ulong)bVar10 * -4 + 2;
      puVar8 = puVar8 + (ulong)bVar10 * -2 + 1;
    }
    piVar7 = param_1 + 0x4c;
    puVar8 = &DAT_0012d490;
    for (lVar4 = 0x26; lVar4 != 0; lVar4 = lVar4 + -1) {
      *puVar8 = *(undefined8 *)piVar7;
      piVar7 = piVar7 + (ulong)bVar10 * -4 + 2;
      puVar8 = puVar8 + (ulong)bVar10 * -2 + 1;
    }
    FUN_00105d78(0,*(undefined8 *)(param_1 + 0x44),0,*(undefined8 *)(param_1 + 0x90),DAT_0012d348,
                 &local_48);
    free((void *)(*(long *)(param_1 + 0x90) * -8 + -8 + local_38));
    FUN_00106b31(param_1);
    if (DAT_0012d1c0 == 4) {
      local_a0 = (undefined8 *)FUN_00106f01(param_1);
    }
    else {
      local_a0 = (undefined8 *)FUN_00107037(param_1);
    }
    if ((DAT_0012d1ec == '\0') && (DAT_0012d280 == 0)) {
      bVar9 = local_a0 != (undefined8 *)0x0;
    }
    else {
      local_88 = local_a0;
      bVar9 = false;
      while ((local_88 != (undefined8 *)0x0 && (bVar9 == false))) {
        local_70 = local_88;
        local_68 = (undefined8 *)FUN_00112437(local_88);
        local_88 = (undefined8 *)*local_68;
        *local_68 = 0;
        iVar2 = FUN_00112af4(local_70,local_c8,local_c0,local_b8,local_b0);
        if (iVar2 != 0) {
          bVar9 = true;
        }
        *local_68 = local_88;
      }
    }
    if (DAT_0012d2a0 == '\0') {
      if ((bVar9 != false) || (DAT_0012d1c8 != '\x01')) {
        lVar4 = DAT_0012d208;
        if (DAT_0012d208 == 0) {
          lVar4 = *(long *)(param_1 + 0x4e);
        }
        lVar5 = DAT_0012d200;
        if (DAT_0012d200 == 0) {
          lVar5 = *(long *)(param_1 + 2);
        }
        FUN_001118ac(lVar5,lVar4,*(long *)(param_1 + 0x98) != 0);
        switch(DAT_0012d1c0) {
        default:
                    /* WARNING: Subroutine does not return */
          abort();
        case 1:
          FUN_0010fcfd(local_a0);
          break;
        case 2:
          FUN_00107f2c(local_a0,0);
          break;
        case 3:
          FUN_00107f2c(local_a0,1);
          break;
        case 4:
          FUN_0010cb85(local_a0);
          break;
        case 5:
          FUN_0010cd6a(local_a0);
          break;
        case 6:
          FUN_0010ceba(local_a0);
          break;
        case 7:
          FUN_0010d076(local_a0);
          break;
        case 8:
          FUN_0010ff70(local_a0);
        }
        FUN_00111e1b();
      }
    }
    else {
      FUN_00107167(bVar9,param_1);
    }
    free(*(void **)(param_1 + 0x40));
    free(local_78);
    for (local_d0 = 0; local_d0 < 2; local_d0 = local_d0 + 1) {
      free(*(void **)(param_1 + (long)local_d0 * 0x4c + 0x3e));
      free((void *)(*(long *)(param_1 + (long)local_d0 * 0x4c + 0x30) * 8 +
                   *(long *)(param_1 + (long)local_d0 * 0x4c + 0x2e)));
    }
    local_a8 = local_a0;
    while (local_a8 != (undefined8 *)0x0) {
      local_60 = (undefined8 *)*local_a8;
      free(local_a8);
      local_a8 = local_60;
    }
    if ((DAT_0012d1c0 == 4) || (DAT_0012d1c0 == 5)) {
      for (local_d0 = 0; local_d0 < 2; local_d0 = local_d0 + 1) {
        if ((char)param_1[(long)local_d0 * 0x4c + 0x48] != '\0') {
          uVar3 = gettext("No newline at end of file");
          if ((&DAT_0012d200)[local_d0] == 0) {
            uVar6 = *(undefined8 *)(param_1 + (long)local_d0 * 0x4c + 2);
          }
          else {
            uVar6 = (&DAT_0012d200)[local_d0];
          }
                    /* WARNING: Subroutine does not return */
          error(0,0,"%s: %s\n",uVar6,uVar3);
        }
      }
    }
  }
  else {
    if (((((*(long *)(param_1 + 0x10) == *(long *)(param_1 + 0x5c)) ||
          (*(long *)(param_1 + 0x10) < 1)) || (*(long *)(param_1 + 0x5c) < 1)) ||
        ((-1 < *param_1 && ((param_1[10] & 0xf000U) != 0x8000)))) ||
       ((-1 < param_1[0x4c] && ((param_1[0x56] & 0xf000U) != 0x8000)))) {
      if (*param_1 == param_1[0x4c]) {
        bVar9 = false;
      }
      else {
        local_58 = 0x7ffffffffffffffe;
        uVar3 = FUN_0011d2a2(*(undefined8 *)(param_1 + 0x12),*(undefined8 *)(param_1 + 0x5e),
                             0x7ffffffffffffffe);
        local_50 = FUN_0011d2a2(8,uVar3,local_58);
        for (local_d0 = 0; local_d0 < 2; local_d0 = local_d0 + 1) {
          uVar3 = FUN_0011c991(*(undefined8 *)(param_1 + (long)local_d0 * 0x4c + 0x28),local_50);
          *(undefined8 *)(param_1 + (long)local_d0 * 0x4c + 0x28) = uVar3;
        }
        while( true ) {
          for (local_d0 = 0; local_d0 < 2; local_d0 = local_d0 + 1) {
            if (-1 < param_1[(long)local_d0 * 0x4c]) {
              FUN_0010de51(param_1 + (long)local_d0 * 0x4c,
                           local_50 - *(long *)(param_1 + (long)local_d0 * 0x4c + 0x2c));
            }
          }
          if ((*(long *)(param_1 + 0x2c) != *(long *)(param_1 + 0x78)) ||
             (iVar2 = memcmp(*(void **)(param_1 + 0x28),*(void **)(param_1 + 0x74),
                             *(size_t *)(param_1 + 0x2c)), iVar2 != 0)) {
            bVar9 = true;
            goto LAB_00107572;
          }
          if (local_50 != *(long *)(param_1 + 0x2c)) break;
          param_1[0x78] = 0;
          param_1[0x79] = 0;
          *(undefined8 *)(param_1 + 0x2c) = *(undefined8 *)(param_1 + 0x78);
        }
        bVar9 = false;
      }
    }
    else {
      bVar9 = true;
    }
LAB_00107572:
    FUN_00107167(bVar9,param_1);
  }
  if (*(long *)(param_1 + 0x28) != *(long *)(param_1 + 0x74)) {
    free(*(void **)(param_1 + 0x28));
  }
  free(*(void **)(param_1 + 0x74));
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar9;
}




// Function: print_context_label @ 0x7d13

void print_context_label(undefined8 param_1,long param_2,undefined8 param_3,long param_4)

{
  uint uVar1;
  tm *ptVar2;
  long lVar3;
  long in_FS_OFFSET;
  char local_48 [56];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001128c2(0);
  if (param_4 != 0) {
    fprintf(DAT_0012d5c0,"%s %s",param_1,param_4);
    goto LAB_00107e27;
  }
  ptVar2 = localtime((time_t *)(param_2 + 0x68));
  uVar1 = FUN_0011bb4f(param_2 + 0x10);
  if (ptVar2 == (tm *)0x0) {
LAB_00107dd2:
    sprintf(local_48,"%ld.%.9d",*(undefined8 *)(param_2 + 0x68),(ulong)uVar1);
  }
  else {
    lVar3 = FUN_0011710b(local_48,0x2b,DAT_0012d350,ptVar2,0,uVar1);
    if (lVar3 == 0) goto LAB_00107dd2;
  }
  fprintf(DAT_0012d5c0,"%s %s\t%s",param_1,param_3,local_48);
LAB_00107e27:
  FUN_001128c2(3);
  putc_unlocked(10,DAT_0012d5c0);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_context_header @ 0x7e5c

void print_context_header(long param_1,undefined8 *param_2,char param_3)

{
  if (param_3 == '\0') {
    FUN_00107d13(&DAT_001230a8,param_1,*param_2,DAT_0012d200);
    FUN_00107d13(&DAT_001230a0,param_1 + 0x130,param_2[1],DAT_0012d208);
  }
  else {
    FUN_00107d13(&DAT_001230a0,param_1,*param_2,DAT_0012d200);
    FUN_00107d13(&DAT_001230a4,param_1 + 0x130,param_2[1],DAT_0012d208);
  }
  return;
}




// Function: print_context_script @ 0x7f2c

void print_context_script(undefined8 *param_1,char param_2)

{
  undefined8 *local_10;
  
  if ((DAT_0012d1ec == '\0') && (local_10 = param_1, DAT_0012d280 == 0)) {
    for (; local_10 != (undefined8 *)0x0; local_10 = (undefined8 *)*local_10) {
      *(undefined1 *)(local_10 + 5) = 0;
    }
  }
  else {
    FUN_00108c9d(param_1);
  }
  DAT_0012d1b0 = -DAT_0012d448;
  DAT_0012d1b8 = 0x7fffffffffffffff;
  if (param_2 == '\0') {
    FUN_0011245b(param_1,FUN_00108bbf,FUN_0010818f);
  }
  else {
    FUN_0011245b(param_1,FUN_00108bbf,FUN_00108710);
  }
  return;
}




// Function: print_context_number_range @ 0x7ff1

void print_context_number_range(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001129d0(param_1,param_2,param_3,&local_20,&local_18);
  if (local_20 < local_18) {
    fprintf(DAT_0012d5c0,"%ld,%ld",local_20,local_18);
  }
  else {
    fprintf(DAT_0012d5c0,"%ld",local_18);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_context_function @ 0x80a2

void print_context_function(FILE *param_1,long param_2)

{
  char cVar1;
  int local_10;
  int local_c;
  
  putc_unlocked(0x20,param_1);
  for (local_10 = 0;
      (cVar1 = FUN_001133c9(*(undefined1 *)(param_2 + local_10)), cVar1 != '\0' &&
      (*(char *)(param_2 + local_10) != '\n')); local_10 = local_10 + 1) {
  }
  for (local_c = local_10; (local_c <= local_10 + 0x27 && (*(char *)(param_2 + local_c) != '\n'));
      local_c = local_c + 1) {
  }
  while ((local_10 < local_c &&
         (cVar1 = FUN_001133c9(*(undefined1 *)(param_2 + (long)local_c + -1)), cVar1 != '\0'))) {
    local_c = local_c + -1;
  }
  fwrite_unlocked((void *)(local_10 + param_2),1,(long)(local_c - local_10),param_1);
  return;
}




// Function: pr_context_hunk @ 0x818f

void pr_context_hunk(undefined8 *param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  undefined *local_38;
  long local_30;
  undefined8 *local_28;
  undefined8 *local_20;
  FILE *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00112af4(param_1,&local_60,&local_58,&local_50,&local_48);
  if (uVar1 != 0) {
    local_40 = -DAT_0012d448;
    lVar2 = local_60 - DAT_0012d1d0;
    local_60 = local_40;
    if (local_40 <= lVar2) {
      local_60 = lVar2;
    }
    lVar2 = local_50 - DAT_0012d1d0;
    local_50 = local_40;
    if (local_40 <= lVar2) {
      local_50 = lVar2;
    }
    if (local_58 < DAT_0012d430 - DAT_0012d1d0) {
      local_58 = DAT_0012d1d0 + local_58;
    }
    else {
      local_58 = DAT_0012d430 + -1;
    }
    if (local_48 < DAT_0012d560 - DAT_0012d1d0) {
      local_48 = DAT_0012d1d0 + local_48;
    }
    else {
      local_48 = DAT_0012d560 + -1;
    }
    local_30 = 0;
    if (DAT_0012d240 != 0) {
      local_30 = FUN_00108d30(DAT_0012d418,local_60);
    }
    FUN_00111b50();
    local_18 = DAT_0012d5c0;
    fputs_unlocked("***************",DAT_0012d5c0);
    if (local_30 != 0) {
      FUN_001080a2(local_18,local_30);
    }
    putc_unlocked(10,local_18);
    FUN_001128c2(4);
    fputs_unlocked("*** ",local_18);
    FUN_00107ff1(&DAT_0012d360,local_60,local_58);
    fputs_unlocked(" ****",local_18);
    FUN_001128c2(3);
    putc_unlocked(10,local_18);
    if ((uVar1 & 1) != 0) {
      local_28 = param_1;
      for (local_40 = local_60; local_40 <= local_58; local_40 = local_40 + 1) {
        FUN_001128c2(2);
        for (; (local_28 != (undefined8 *)0x0 && ((long)(local_28[2] + local_28[3]) <= local_40));
            local_28 = (undefined8 *)*local_28) {
        }
        local_38 = &DAT_001230d3;
        if ((local_28 != (undefined8 *)0x0) && ((long)local_28[3] <= local_40)) {
          if ((long)local_28[1] < 1) {
            local_38 = &DAT_001230d7;
          }
          else {
            local_38 = &DAT_001230d5;
          }
        }
        FUN_001124fe(local_38,DAT_0012d418 + local_40 * 8,1);
        FUN_001128c2(3);
        if (*(char *)(*(long *)((local_40 + 1) * 8 + DAT_0012d418) + -1) == '\n') {
          putc_unlocked(10,local_18);
        }
      }
    }
    FUN_001128c2(4);
    fputs_unlocked("--- ",local_18);
    FUN_00107ff1(&DAT_0012d490,local_50,local_48);
    fputs_unlocked(" ----",local_18);
    FUN_001128c2(3);
    putc_unlocked(10,local_18);
    if ((uVar1 & 2) != 0) {
      local_20 = param_1;
      for (local_40 = local_50; local_40 <= local_48; local_40 = local_40 + 1) {
        FUN_001128c2(1);
        for (; (local_20 != (undefined8 *)0x0 && ((long)(local_20[1] + local_20[4]) <= local_40));
            local_20 = (undefined8 *)*local_20) {
        }
        local_38 = &DAT_001230d3;
        if ((local_20 != (undefined8 *)0x0) && ((long)local_20[4] <= local_40)) {
          if ((long)local_20[2] < 1) {
            local_38 = &DAT_001230e4;
          }
          else {
            local_38 = &DAT_001230d5;
          }
        }
        FUN_001124fe(local_38,DAT_0012d548 + local_40 * 8,1);
        FUN_001128c2(3);
        if (*(char *)(*(long *)((local_40 + 1) * 8 + DAT_0012d548) + -1) == '\n') {
          putc_unlocked(10,local_18);
        }
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_unidiff_number_range @ 0x863e

void print_unidiff_number_range(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char *__format;
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001129d0(param_1,param_2,param_3,&local_20,&local_18);
  if (local_20 < local_18) {
    fprintf(DAT_0012d5c0,"%ld,%ld",local_20,(local_18 - local_20) + 1);
  }
  else {
    if (local_18 < local_20) {
      __format = "%ld,0";
    }
    else {
      __format = "%ld";
    }
    fprintf(DAT_0012d5c0,__format,local_18);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: pr_unidiff_hunk @ 0x8710

void pr_unidiff_hunk(undefined8 *param_1)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  long local_78;
  long local_70;
  long local_68;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  undefined8 *local_40;
  long local_38;
  FILE *local_30;
  undefined8 *local_28;
  undefined8 *local_20;
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00112af4(param_1,&local_78,&local_70,&local_68,&local_60);
  if (iVar1 != 0) {
    local_58 = -DAT_0012d448;
    lVar2 = local_78 - DAT_0012d1d0;
    local_78 = local_58;
    if (local_58 <= lVar2) {
      local_78 = lVar2;
    }
    lVar2 = local_68 - DAT_0012d1d0;
    local_68 = local_58;
    if (local_58 <= lVar2) {
      local_68 = lVar2;
    }
    if (local_70 < DAT_0012d430 - DAT_0012d1d0) {
      local_70 = DAT_0012d1d0 + local_70;
    }
    else {
      local_70 = DAT_0012d430 + -1;
    }
    if (local_60 < DAT_0012d560 - DAT_0012d1d0) {
      local_60 = DAT_0012d1d0 + local_60;
    }
    else {
      local_60 = DAT_0012d560 + -1;
    }
    local_38 = 0;
    if (DAT_0012d240 != 0) {
      local_38 = FUN_00108d30(DAT_0012d418,local_78);
    }
    FUN_00111b50();
    local_30 = DAT_0012d5c0;
    FUN_001128c2(4);
    fputs_unlocked("@@ -",local_30);
    FUN_0010863e(&DAT_0012d360,local_78,local_70);
    fputs_unlocked(" +",local_30);
    FUN_0010863e(&DAT_0012d490,local_68,local_60);
    fputs_unlocked(" @@",local_30);
    FUN_001128c2(3);
    if (local_38 != 0) {
      FUN_001080a2(local_30,local_38);
    }
    putc_unlocked(10,local_30);
    local_58 = local_78;
    local_50 = local_68;
    local_40 = param_1;
    while ((local_58 <= local_70 || (local_50 <= local_60))) {
      if ((local_40 == (undefined8 *)0x0) || (local_58 < (long)local_40[3])) {
        lVar2 = local_58 + 1;
        local_18 = (undefined8 *)(local_58 * 8 + DAT_0012d418);
        local_58 = lVar2;
        if ((DAT_0012d2b1 != '\x01') || (*(char *)*local_18 != '\n')) {
          if (DAT_0012d2b0 == '\0') {
            iVar1 = 0x20;
          }
          else {
            iVar1 = 9;
          }
          putc_unlocked(iVar1,local_30);
        }
        FUN_001124cf(0,local_18);
        local_50 = local_50 + 1;
      }
      else {
        local_48 = local_40[2];
        while (local_48 != 0) {
          local_20 = (undefined8 *)(local_58 * 8 + DAT_0012d418);
          local_58 = local_58 + 1;
          local_48 = local_48 + -1;
          FUN_001128c2(2);
          putc_unlocked(0x2d,local_30);
          if ((DAT_0012d2b0 != '\0') && ((DAT_0012d2b1 != '\x01' || (*(char *)*local_20 != '\n'))))
          {
            putc_unlocked(9,local_30);
          }
          FUN_001124fe(0,local_20,1);
          FUN_001128c2(3);
          if (*(char *)(local_20[1] + -1) == '\n') {
            putc_unlocked(10,local_30);
          }
        }
        local_48 = local_40[1];
        while (local_48 != 0) {
          local_28 = (undefined8 *)(local_50 * 8 + DAT_0012d548);
          local_50 = local_50 + 1;
          local_48 = local_48 + -1;
          FUN_001128c2(1);
          putc_unlocked(0x2b,local_30);
          if ((DAT_0012d2b0 != '\0') && ((DAT_0012d2b1 != '\x01' || (*(char *)*local_28 != '\n'))))
          {
            putc_unlocked(9,local_30);
          }
          FUN_001124fe(0,local_28,1);
          FUN_001128c2(3);
          if (*(char *)(local_28[1] + -1) == '\n') {
            putc_unlocked(10,local_30);
          }
        }
        local_40 = (undefined8 *)*local_40;
        local_48 = 0xffffffffffffffff;
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_hunk @ 0x8bbf

long * find_hunk(long *param_1)

{
  long *plVar1;
  long lVar2;
  long lVar3;
  long *local_40;
  
  lVar2 = DAT_0012d1d0 * 2 + 1;
  plVar1 = param_1;
  while( true ) {
    local_40 = plVar1;
    plVar1 = (long *)*local_40;
    lVar3 = lVar2;
    if ((plVar1 != (long *)0x0) && (*(char *)(plVar1 + 5) != '\0')) {
      lVar3 = DAT_0012d1d0;
    }
    if ((plVar1 != (long *)0x0) &&
       (plVar1[3] - (local_40[2] + local_40[3]) != plVar1[4] - (local_40[1] + local_40[4]))) break;
    if ((plVar1 == (long *)0x0) || (lVar3 <= plVar1[3] - (local_40[2] + local_40[3]))) {
      return local_40;
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: mark_ignorable @ 0x8c9d

void mark_ignorable(undefined8 *param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 *local_40;
  undefined1 local_38 [8];
  undefined1 local_30 [8];
  undefined1 local_28 [8];
  undefined1 local_20 [8];
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = param_1;
  while (local_40 != (undefined8 *)0x0) {
    local_18 = (undefined8 *)*local_40;
    *local_40 = 0;
    iVar1 = FUN_00112af4(local_40,local_38,local_30,local_28,local_20);
    *(bool *)(local_40 + 5) = iVar1 == 0;
    *local_40 = local_18;
    local_40 = local_18;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: find_function @ 0x8d30

char * find_function(long param_1,long param_2)

{
  long lVar1;
  char *__string;
  long lVar2;
  int iVar3;
  ulong uVar4;
  long local_28;
  
  lVar2 = DAT_0012d1b0;
  DAT_0012d1b0 = param_2;
  local_28 = param_2;
  do {
    lVar1 = local_28 + -1;
    if (lVar1 < lVar2) {
      if (DAT_0012d1b8 != 0x7fffffffffffffff) {
        return *(char **)(param_1 + DAT_0012d1b8 * 8);
      }
      return (char *)0x0;
    }
    __string = *(char **)(param_1 + lVar1 * 8);
    uVar4 = (*(long *)(param_1 + local_28 * 8) - (long)__string) - 1;
    if (0x7fffffff < uVar4) {
      uVar4 = 0x7fffffff;
    }
    iVar3 = re_search((re_pattern_buffer *)&DAT_0012d220,__string,(int)uVar4,0,(int)uVar4,
                      (re_registers *)0x0);
    local_28 = lVar1;
  } while (iVar3 < 0);
  DAT_0012d1b8 = lVar1;
  return __string;
}




// Function: option_list @ 0x8e44

undefined1 * option_list(long param_1,int param_2)

{
  long lVar1;
  undefined1 *puVar2;
  int local_24;
  long local_20;
  undefined1 *local_18;
  
  local_20 = 1;
  for (local_24 = 0; local_24 < param_2; local_24 = local_24 + 1) {
    lVar1 = FUN_0011a4f6(*(undefined8 *)(param_1 + (long)local_24 * 8));
    local_20 = local_20 + lVar1 + 1;
  }
  puVar2 = (undefined1 *)FUN_0011c945(local_20);
  local_18 = puVar2;
  for (local_24 = 0; local_24 < param_2; local_24 = local_24 + 1) {
    *local_18 = 0x20;
    local_18 = (undefined1 *)
               FUN_0011a550(local_18 + 1,*(undefined8 *)(param_1 + (long)local_24 * 8));
  }
  *local_18 = 0;
  return puVar2;
}




// Function: exclude_options @ 0x8f0e

undefined8 exclude_options(void)

{
  undefined8 uVar1;
  
  if (DAT_0012d1ef == '\0') {
    uVar1 = 0x10000000;
  }
  else {
    uVar1 = 0x10000010;
  }
  return uVar1;
}




// Function: main @ 0x8f2f

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  bool bVar2;
  ulong uVar3;
  char cVar4;
  byte bVar5;
  undefined4 uVar6;
  int iVar7;
  size_t sVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  long in_FS_OFFSET;
  int local_b0;
  int local_ac;
  uint local_a8;
  int local_a4;
  long local_98;
  long local_90;
  char *local_88;
  ulong local_80;
  ulong local_78;
  ulong local_70;
  char *local_68;
  char *local_60;
  ulong local_58;
  ulong local_50;
  ulong local_48;
  ulong local_40;
  ulong local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_b0 = 0;
  local_a4 = -1;
  local_80 = 0xffffffffffffffff;
  bVar2 = false;
  local_78 = 0;
  bVar1 = false;
  local_98 = 0;
  local_90 = 0;
  DAT_0012d0c8 = 2;
  FUN_00119a53(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00113721(0);
  _DAT_0012d600 = &DAT_0012d220;
  _DAT_0012d640 = &DAT_0012d260;
  re_set_syntax(0x50a46);
  DAT_0012d340 = FUN_001139c7();
  DAT_0012d5c8 = 0;
  FUN_0011cfb9();
  do {
    local_ac = getopt_long(param_1,param_2,
                           "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",
                           &PTR_s_binary_0012bf40,0);
    if (local_ac == -1) {
      if (((DAT_0012d1c4 == 1) && (local_60 = getenv("TERM"), local_60 != (char *)0x0)) &&
         (iVar7 = strcmp(local_60,"dumb"), iVar7 == 0)) {
        DAT_0012d1c4 = 0;
      }
      if (DAT_0012d1c0 == 0) {
        if (bVar1) {
          FUN_0010a63e(2);
          if ((long)local_80 < 0) {
            DAT_0012d1d0 = 3;
          }
        }
        else {
          FUN_0010a63e(1);
        }
      }
      if ((DAT_0012d1c0 == 2) && (cVar4 = FUN_00114cdf(2), cVar4 == '\0')) {
        DAT_0012d350 = "%a %b %e %T %Y";
      }
      else {
        DAT_0012d350 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      if (((-1 < (long)local_80) && ((DAT_0012d1c0 == 2 || (DAT_0012d1c0 == 3)))) &&
         (((long)DAT_0012d1d0 < (long)local_80 ||
          (((long)local_80 < (long)DAT_0012d1d0 && (!bVar2)))))) {
        DAT_0012d1d0 = local_80;
      }
      if (DAT_0012d2a8 == 0) {
        DAT_0012d2a8 = 8;
      }
      if (local_78 == 0) {
        local_78 = 0x82;
      }
      local_58 = DAT_0012d2a8;
      if (DAT_0012d2a1 != '\0') {
        local_58 = 1;
      }
      local_50 = local_78;
      local_48 = local_58 + 3;
      local_40 = (ulong)((uint)local_78 & (uint)local_48 & 1) + (local_78 >> 1) + (local_48 >> 1);
      local_38 = local_40 - local_40 % local_58;
      if ((local_38 < 4) || (local_78 <= local_38)) {
        DAT_0012d320 = 0;
      }
      else {
        DAT_0012d320 = local_78 - local_38;
        if (local_38 - 3 <= local_78 - local_38) {
          DAT_0012d320 = local_38 - 3;
        }
      }
      DAT_0012d328 = local_78;
      if (DAT_0012d320 != 0) {
        DAT_0012d328 = local_38;
      }
      if ((long)DAT_0012d1e0 < (long)DAT_0012d1d0) {
        DAT_0012d1e0 = DAT_0012d1d0;
      }
      FUN_0010a296(&DAT_0012d5e0);
      FUN_0010a296(&DAT_0012d620);
      if (DAT_0012d1c0 == 7) {
        for (local_a8 = 0; local_a8 < 3; local_a8 = local_a8 + 1) {
          if ((&DAT_0012d300)[(int)local_a8] == (char *)0x0) {
            (&DAT_0012d300)[(int)local_a8] = &DAT_001236f6;
          }
        }
        if ((DAT_0012d2e8 == (undefined *)0x0) &&
           (DAT_0012d2e8 = DAT_0012d2f8, DAT_0012d2f8 == (undefined *)0x0)) {
          DAT_0012d2e8 = &DAT_001236fa;
        }
        if ((DAT_0012d2f0 == (undefined *)0x0) &&
           (DAT_0012d2f0 = DAT_0012d2f8, DAT_0012d2f8 == (undefined *)0x0)) {
          DAT_0012d2f0 = &DAT_001236fd;
        }
        if (DAT_0012d2e0 == (char *)0x0) {
          DAT_0012d2e0 = "%=";
        }
        if (DAT_0012d2f8 == (undefined *)0x0) {
          DAT_0012d2f8 = (undefined *)FUN_00112fa9(DAT_0012d2e8,DAT_0012d2f0,&DAT_0012353e);
        }
      }
      if (DAT_0012d1c0 == 7) {
        if ((*DAT_0012d2e0 == '\0') ||
           ((iVar7 = strcmp(DAT_0012d2e0,"%="), iVar7 == 0 && (*DAT_0012d300 == '\0')))) {
          DAT_0012d1c8 = true;
        }
        else {
          DAT_0012d1c8 = false;
        }
      }
      else {
        DAT_0012d1c8 = DAT_0012d31a != '\0' || DAT_0012d1c0 != 8;
      }
      if ((DAT_0012d620 == 0) && (DAT_0012d1e8 == 0)) {
        bVar5 = 0;
      }
      else {
        bVar5 = 1;
      }
      DAT_0012d1ed = (~(bVar5 | DAT_0012d2b2 | DAT_0012d1ec | DAT_0012d1ee) & DAT_0012d2a0 & 1) != 0
      ;
      DAT_0012d330 = FUN_00108e44(param_2 + 1,optind + -1);
      if (local_98 == 0) {
        if (local_90 == 0) {
          if (param_1 - optind != 2) {
            if (param_1 - optind < 2) {
              FUN_0010a338("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
            }
            FUN_0010a338("extra operand \'%s\'",param_2[(long)optind + 2]);
          }
          local_b0 = FUN_0010a746(0,param_2[optind],param_2[(long)optind + 1]);
        }
        else {
          for (; optind < param_1; optind = optind + 1) {
            iVar7 = FUN_0010a746(0,param_2[optind],local_90);
            if (local_b0 < iVar7) {
              local_b0 = iVar7;
            }
          }
        }
      }
      else {
        if (local_90 == 0) goto LAB_00109f62;
        FUN_00110966("--from-file and --to-file both specified");
        do {
          iVar7 = FUN_0010a746(0,local_98,param_2[optind]);
          if (local_b0 < iVar7) {
            local_b0 = iVar7;
          }
          optind = optind + 1;
LAB_00109f62:
        } while (optind < param_1);
      }
      FUN_00110bca();
      FUN_0010a3ae();
                    /* WARNING: Subroutine does not return */
      exit(local_b0);
    }
    uVar3 = DAT_0012d1e0;
    switch(local_ac) {
    default:
      FUN_0010a338(0,0);
    case 0:
    case 0x68:
    case 0x80:
    case 0x85:
      uVar3 = DAT_0012d1e0;
      break;
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      if (local_a4 - 0x30U < 10) {
        if ((long)(local_80 - (local_ac < 0x34)) < 0x666666666666666) {
          local_80 = (long)(local_ac + -0x30) + local_80 * 10;
        }
        else {
          local_80 = 0x3fffffffffffffff;
        }
      }
      else {
        local_80 = (ulong)(local_ac + -0x30);
      }
      break;
    case 0x42:
      DAT_0012d1ec = 1;
      break;
    case 0x43:
    case 0x55:
      if (optarg == (char *)0x0) {
        local_70 = 3;
      }
      else {
        local_70 = strtoimax(optarg,&local_88,10);
        if ((*local_88 != '\0') || ((long)local_70 < 0)) {
          FUN_0010a338("invalid context length \'%s\'",optarg);
        }
        if (0x3fffffffffffffff < (long)local_70) {
          local_70 = 0x3fffffffffffffff;
        }
      }
      if (local_ac == 0x55) {
        uVar6 = 3;
      }
      else {
        uVar6 = 2;
      }
      FUN_0010a63e(uVar6);
      if ((long)DAT_0012d1d0 < (long)local_70) {
        DAT_0012d1d0 = local_70;
      }
      bVar2 = true;
      uVar3 = DAT_0012d1e0;
      break;
    case 0x44:
      FUN_0010a63e(7);
      sVar8 = strlen(optarg);
      local_68 = (char *)FUN_0011c945(sVar8 * 7 + 0x61);
      sprintf(local_68,
              "%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n"
              ,0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
      for (local_a8 = 0; uVar3 = DAT_0012d1e0, local_a8 < 4; local_a8 = local_a8 + 1) {
        FUN_0010a5af(&DAT_0012d2e0 + (int)local_a8,local_68,&DAT_0012357d);
        sVar8 = strlen(local_68);
        local_68 = local_68 + sVar8 + 1;
      }
      break;
    case 0x45:
      if (DAT_0012d1e8 < 4) {
        DAT_0012d1e8 = DAT_0012d1e8 | 1;
      }
      break;
    case 0x46:
      FUN_0010a107(&DAT_0012d5e0,optarg);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x48:
      DAT_0012d338 = 1;
      break;
    case 0x49:
      FUN_0010a107(&DAT_0012d620,optarg);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x4c:
      if (DAT_0012d200 == (char *)0x0) {
        DAT_0012d200 = optarg;
      }
      else {
        if (DAT_0012d208 != (char *)0x0) {
          FUN_00110966("too many file label options");
          goto switchD_00109082_caseD_6e;
        }
        DAT_0012d208 = optarg;
      }
      break;
    case 0x4e:
      DAT_0012d648 = 1;
      break;
    case 0x50:
      DAT_0012d649 = 1;
      break;
    case 0x53:
      FUN_0010a5af(&DAT_0012d2b8,optarg,&DAT_001235d1);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x54:
      DAT_0012d2b0 = 1;
      break;
    case 0x57:
      local_70 = strtoimax(optarg,&local_88,10);
      if (((long)local_70 < 1) || (*local_88 != '\0')) {
        FUN_0010a338("invalid width \'%s\'",optarg);
      }
      uVar3 = DAT_0012d1e0;
      if (local_78 != local_70) {
        if (local_78 != 0) {
          FUN_00110966("conflicting width options");
        }
        local_78 = local_70;
        uVar3 = DAT_0012d1e0;
      }
      break;
    case 0x58:
      uVar6 = FUN_00108f0e();
      iVar7 = FUN_001148b2(FUN_001142e9,DAT_0012d340,optarg,uVar6,10);
      uVar3 = DAT_0012d1e0;
      if (iVar7 != 0) {
        FUN_00110925(optarg);
        goto switchD_00109082_caseD_79;
      }
      break;
    case 0x5a:
      if (DAT_0012d1e8 < 4) {
        DAT_0012d1e8 = DAT_0012d1e8 | 2;
      }
      break;
    case 0x61:
      DAT_0012d1d8 = 1;
      break;
    case 0x62:
      if (DAT_0012d1e8 < 4) {
        DAT_0012d1e8 = 4;
      }
      break;
    case 99:
      FUN_0010a63e(2);
      uVar3 = DAT_0012d1e0;
      if ((long)DAT_0012d1d0 < 3) {
        DAT_0012d1d0 = 3;
      }
      break;
    case 100:
      DAT_0012d348 = 1;
      break;
    case 0x65:
      FUN_0010a63e(4);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x66:
      FUN_0010a63e(5);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x69:
      DAT_0012d1ee = 1;
      break;
    case 0x6c:
      if (s__usr_bin_pr_00125400[0] == '\0') {
        FUN_0010a338("pagination not supported on this host",0);
      }
      DAT_0012d2c0 = 1;
      signal(0x11,(__sighandler_t)0x0);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x6e:
switchD_00109082_caseD_6e:
      FUN_0010a63e(6);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x70:
      bVar1 = true;
      FUN_0010a107(&DAT_0012d5e0,"^[[:alpha:]$_]");
      uVar3 = DAT_0012d1e0;
      break;
    case 0x71:
      DAT_0012d2a0 = 1;
      break;
    case 0x72:
      DAT_0012d5c9 = 1;
      break;
    case 0x73:
      DAT_0012d64a = 1;
      break;
    case 0x74:
      DAT_0012d2a1 = '\x01';
      break;
    case 0x75:
      FUN_0010a63e(3);
      uVar3 = DAT_0012d1e0;
      if ((long)DAT_0012d1d0 < 3) {
        DAT_0012d1d0 = 3;
      }
      break;
    case 0x76:
      uVar9 = FUN_0011a014("Len Tower");
      uVar10 = FUN_0011a014("Richard Stallman");
      uVar11 = FUN_0011a014("David Hayes");
      uVar12 = FUN_0011a014("Mike Haertel");
      uVar13 = FUN_0011a014("Paul Eggert");
      FUN_0011c71c(stdout,&DAT_00123622,"GNU diffutils",PTR_DAT_0012d0c0,uVar13,uVar12,uVar11,uVar10
                   ,uVar9,0);
      FUN_0010a3ae();
      goto LAB_0010a0e6;
    case 0x77:
      DAT_0012d1e8 = 5;
      break;
    case 0x78:
      uVar6 = FUN_00108f0e();
      FUN_001142e9(DAT_0012d340,optarg,uVar6);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x79:
switchD_00109082_caseD_79:
      FUN_0010a63e(8);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x81:
      FUN_0010a5af(&local_98,optarg,"--from-file");
      uVar3 = DAT_0012d1e0;
      break;
    case 0x82:
      FUN_0010a405();
      FUN_0010a3ae();
LAB_0010a0e6:
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return 0;
    case 0x83:
      local_70 = strtoimax(optarg,&local_88,10);
      if ((*local_88 != '\0') || ((long)local_70 < 0)) {
        FUN_0010a338("invalid horizon length \'%s\'",optarg);
      }
      uVar3 = local_70;
      if ((long)local_70 <= (long)DAT_0012d1e0) {
        uVar3 = DAT_0012d1e0;
      }
      break;
    case 0x84:
      DAT_0012d1ef = 1;
      break;
    case 0x86:
      DAT_0012d319 = 1;
      break;
    case 0x87:
      FUN_0010a63e(7);
      for (local_a8 = 0; uVar3 = DAT_0012d1e0, local_a8 < 3; local_a8 = local_a8 + 1) {
        FUN_0010a5af(&DAT_0012d300 + (int)local_a8,optarg,"--line-format");
      }
      break;
    case 0x88:
      DAT_0012d1f0 = 1;
      break;
    case 0x89:
      DAT_0012d1ef = 0;
      break;
    case 0x8a:
      FUN_0010a63e(1);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x8b:
      FUN_0010a63e(8);
      DAT_0012d318 = 1;
      uVar3 = DAT_0012d1e0;
      break;
    case 0x8c:
      DAT_0012d2b2 = 1;
      break;
    case 0x8d:
      DAT_0012d2b1 = 1;
      break;
    case 0x8e:
      DAT_0012d31a = '\x01';
      break;
    case 0x8f:
      local_70 = strtoimax(optarg,&local_88,10);
      if ((((long)local_70 < 1) || (0xfffffffffffffffc < local_70)) || (*local_88 != '\0')) {
        FUN_0010a338("invalid tabsize \'%s\'",optarg);
      }
      uVar3 = DAT_0012d1e0;
      if (DAT_0012d2a8 != local_70) {
        if (DAT_0012d2a8 != 0) {
          FUN_00110966("conflicting tabsize options");
        }
        DAT_0012d2a8 = local_70;
        uVar3 = DAT_0012d1e0;
      }
      break;
    case 0x90:
      FUN_0010a5af(&local_90,optarg,"--to-file");
      uVar3 = DAT_0012d1e0;
      break;
    case 0x91:
    case 0x92:
    case 0x93:
      FUN_0010a63e(7);
      local_ac = local_ac + -0x91;
      FUN_0010a5af(&DAT_0012d300 + local_ac,optarg,"--unchanged-line-format" + (long)local_ac * 0x18
                  );
      uVar3 = DAT_0012d1e0;
      break;
    case 0x94:
    case 0x95:
    case 0x96:
    case 0x97:
      FUN_0010a63e(7);
      local_ac = local_ac + -0x94;
      FUN_0010a5af(&DAT_0012d2e0 + local_ac,optarg,
                   "--unchanged-group-format" + (long)local_ac * 0x19);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x98:
      FUN_0010a682(optarg);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x99:
      FUN_0011140f(optarg);
      uVar3 = DAT_0012d1e0;
      break;
    case 0x9a:
      DAT_0012d5c8 = 1;
    }
    DAT_0012d1e0 = uVar3;
    local_a4 = local_ac;
  } while( true );
}




// Function: add_regexp @ 0xa107

void add_regexp(long *param_1,char *param_2)

{
  long lVar1;
  byte bVar2;
  size_t __length;
  char *pcVar3;
  long local_38;
  long local_30;
  ulong local_28;
  
  __length = strlen(param_2);
  pcVar3 = re_compile_pattern(param_2,__length,(re_pattern_buffer *)param_1[4]);
  if (pcVar3 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
    error(2,0,"%s: %s",param_2,pcVar3);
  }
  local_38 = *param_1;
  local_30 = param_1[1];
  *(bool *)(param_1 + 3) = local_38 != 0;
  bVar2 = *(byte *)(param_1 + 3);
  param_1[1] = (int)((uint)bVar2 * 2) + local_30 + __length;
  local_28 = param_1[2];
  if (local_28 <= (ulong)param_1[1]) {
    if (local_28 == 0) {
      local_28 = 1;
    }
    do {
      local_28 = local_28 << 1;
    } while (local_28 <= (ulong)param_1[1]);
    param_1[2] = local_28;
    local_38 = FUN_0011c991(local_38,local_28);
    *param_1 = local_38;
  }
  if (bVar2 != 0) {
    lVar1 = local_30 + 1;
    *(undefined1 *)(local_30 + local_38) = 0x5c;
    local_30 = local_30 + 2;
    *(undefined1 *)(lVar1 + local_38) = 0x7c;
  }
  memcpy((void *)(local_38 + local_30),param_2,__length + 1);
  return;
}




// Function: summarize_regexp_list @ 0xa296

void summarize_regexp_list(long *param_1)

{
  long lVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  if (*param_1 != 0) {
    lVar1 = param_1[4];
    uVar2 = FUN_0011c945(0x100);
    *(undefined8 *)(lVar1 + 0x20) = uVar2;
    if ((char)param_1[3] != '\0') {
      pcVar3 = re_compile_pattern((char *)*param_1,param_1[1],(re_pattern_buffer *)param_1[4]);
      if (pcVar3 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
        error(2,0,"%s: %s",*param_1,pcVar3);
      }
    }
  }
  return;
}




// Function: try_help @ 0xa338

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_0013d8a8;
  if (param_1 != 0) {
    uVar1 = gettext(param_1);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = gettext("Try \'%s --help\' for more information.");
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: check_stdout @ 0xa3ae

void check_stdout(void)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 != 0) {
    FUN_00110966("write failed");
  }
  iVar1 = fclose(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("standard output");
    FUN_00110925(uVar2);
  }
  return;
}




// Function: usage @ 0xa405

void usage(void)

{
  FILE *__stream;
  uint uVar1;
  int iVar2;
  char *pcVar3;
  undefined8 uVar4;
  undefined **local_30;
  char *local_28;
  
  uVar4 = DAT_0013d8a8;
  pcVar3 = (char *)gettext("Usage: %s [OPTION]... FILES\n");
  printf(pcVar3,uVar4);
  uVar4 = gettext("Compare FILES line by line.");
  printf("%s\n\n",uVar4);
  __stream = stdout;
  pcVar3 = (char *)gettext(
                          "Mandatory arguments to long options are mandatory for short options too.\n"
                          );
  fputs_unlocked(pcVar3,__stream);
  local_30 = &PTR_s___normal_output_a_normal_diff__t_0012c6e0;
  do {
    if (*local_30 == (undefined *)0x0) {
      FUN_0011c800();
      return;
    }
    if (**local_30 == '\0') {
      putchar_unlocked(10);
    }
    else {
      local_28 = (char *)gettext(*local_30);
      while( true ) {
        pcVar3 = strchr(local_28,10);
        if (pcVar3 == (char *)0x0) break;
        uVar1 = ((int)pcVar3 + 1) - (int)local_28;
        if (0xfff < (int)uVar1) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage");
        }
        printf("  %.*s",(ulong)uVar1,local_28);
        local_28 = pcVar3 + 1;
      }
      if ((*local_28 == ' ') || (*local_28 == '-')) {
        iVar2 = 0;
      }
      else {
        iVar2 = 1;
      }
      printf(&DAT_00124ece + iVar2 * 2,local_28);
    }
    local_30 = local_30 + 1;
  } while( true );
}




// Function: specify_value @ 0xa5af

void specify_value(long *param_1,char *param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  if (*param_1 != 0) {
    iVar1 = strcmp((char *)*param_1,param_2);
    if (iVar1 != 0) {
      uVar2 = gettext("conflicting %s option value \'%s\'");
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar2,param_3,param_2);
    }
  }
  *param_1 = (long)param_2;
  return;
}




// Function: specify_style @ 0xa63e

void specify_style(int param_1)

{
  int iVar1;
  
  iVar1 = DAT_0012d1c0;
  if ((param_1 != DAT_0012d1c0) && (iVar1 = param_1, DAT_0012d1c0 != 0)) {
    FUN_0010a338("conflicting output style options",0);
    iVar1 = param_1;
  }
  DAT_0012d1c0 = iVar1;
  return;
}




// Function: specify_colors_style @ 0xa682

void specify_colors_style(char *param_1)

{
  int iVar1;
  
  if ((param_1 == (char *)0x0) || (iVar1 = strcmp(param_1,"auto"), iVar1 == 0)) {
    DAT_0012d1c4 = 1;
    return;
  }
  iVar1 = strcmp(param_1,"always");
  if (iVar1 == 0) {
    DAT_0012d1c4 = 2;
    return;
  }
  iVar1 = strcmp(param_1,"never");
  if (iVar1 == 0) {
    DAT_0012d1c4 = 0;
    return;
  }
  FUN_0010a338("invalid color \'%s\'",param_1);
  return;
}




// Function: set_mtime_to_now @ 0xa723

void set_mtime_to_now(long param_1)

{
  FUN_00114c76(param_1 + 0x58);
  return;
}




// Function: compare_files @ 0xa746

int compare_files(long param_1,char *param_2,char *param_3)

{
  __ino_t _Var1;
  __dev_t _Var2;
  __blksize_t _Var3;
  __time_t _Var4;
  bool bVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  long lVar9;
  __off_t _Var10;
  int *piVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  char *pcVar14;
  char *pcVar15;
  long lVar16;
  long in_FS_OFFSET;
  char *local_310;
  char *local_308;
  int local_2f4;
  char *local_2e0;
  char *local_2d8;
  char *local_298 [2];
  int local_288 [2];
  char *local_280;
  stat local_278 [2];
  int local_158;
  char *local_150;
  __dev_t local_148;
  __ino_t local_140;
  __nlink_t local_138;
  uint local_130;
  __uid_t local_12c;
  __gid_t local_128;
  __dev_t local_120;
  long local_118;
  long local_f0;
  long local_e0;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2f4 = 0;
  if ((((param_2 == (char *)0x0) || (param_3 == (char *)0x0)) &&
      ((DAT_0012d649 != '\x01' || (param_3 == (char *)0x0)))) && (DAT_0012d648 != '\x01')) {
    if (param_2 != (char *)0x0) {
      param_3 = param_2;
    }
    FUN_001109a8("Only in %s: %s\n",
                 *(undefined8 *)((long)(int)(uint)(param_2 == (char *)0x0) * 0x130 + param_1 + 8),
                 param_3);
    local_2f4 = 1;
    goto LAB_0010bdc2;
  }
  memset(local_288,0,0x260);
  local_28 = param_1;
  if (param_2 == (char *)0x0) {
    local_288[0] = -1;
  }
  else {
    local_288[0] = -2;
  }
  if (param_3 == (char *)0x0) {
    local_158 = -1;
  }
  else {
    local_158 = -2;
  }
  local_308 = param_2;
  if (param_2 == (char *)0x0) {
    local_308 = param_3;
  }
  local_310 = param_3;
  if (param_3 == (char *)0x0) {
    local_310 = local_308;
  }
  if (param_1 == 0) {
    local_2e0 = (char *)0x0;
    local_2d8 = (char *)0x0;
    local_280 = local_308;
    local_150 = local_310;
  }
  else {
    local_2e0 = (char *)FUN_00114ade(*(undefined8 *)(param_1 + 8),local_308,0);
    local_280 = local_2e0;
    local_2d8 = (char *)FUN_00114ade(*(undefined8 *)(param_1 + 0x138),local_310,0);
    local_150 = local_2d8;
  }
  for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
    if (local_288[(long)iVar7 * 0x4c] != -1) {
      if ((iVar7 == 0) || (iVar8 = strcmp((&local_280)[(long)iVar7 * 0x26],local_280), iVar8 != 0))
      {
        iVar8 = strcmp((&local_280)[(long)iVar7 * 0x26],"-");
        if (iVar8 == 0) {
          local_288[(long)iVar7 * 0x4c] = 0;
          iVar8 = isatty(0);
          if (iVar8 == 0) {
            FUN_001131e6(0,0);
          }
          iVar8 = fstat(0,(stat *)((long)local_278 + (long)iVar7 * 0x130));
          if (iVar8 == 0) {
            if ((*(uint *)((long)local_278 + (long)iVar7 * 0x130 + 0x18) & 0xf000) == 0x8000) {
              _Var10 = lseek(0,0,1);
              if (_Var10 < 0) {
                piVar11 = __errno_location();
                local_288[(long)iVar7 * 0x4c] = -3 - *piVar11;
              }
              else {
                lVar9 = *(long *)((long)local_278 + (long)iVar7 * 0x130 + 0x30) - _Var10;
                lVar16 = 0;
                if (-1 < lVar9) {
                  lVar16 = lVar9;
                }
                *(long *)((long)local_278 + (long)iVar7 * 0x130 + 0x30) = lVar16;
              }
            }
            FUN_0010a723((long)local_278 + (long)iVar7 * 0x130);
          }
          else {
            piVar11 = __errno_location();
            local_288[(long)iVar7 * 0x4c] = -3 - *piVar11;
          }
        }
        else {
          if (DAT_0012d1f0 == '\0') {
            iVar8 = stat((&local_280)[(long)iVar7 * 0x26],
                         (stat *)((long)local_278 + (long)iVar7 * 0x130));
          }
          else {
            iVar8 = lstat((&local_280)[(long)iVar7 * 0x26],
                          (stat *)((long)local_278 + (long)iVar7 * 0x130));
          }
          if (iVar8 != 0) {
            piVar11 = __errno_location();
            local_288[(long)iVar7 * 0x4c] = -3 - *piVar11;
          }
        }
      }
      else {
        local_288[(long)iVar7 * 0x4c] = local_288[0];
        _Var1 = local_278[0].st_ino;
        lVar9 = (long)iVar7 * 0x130;
        *(__dev_t *)((long)local_278 + lVar9) = local_278[0].st_dev;
        *(__ino_t *)((long)local_278 + lVar9 + 8) = _Var1;
        uVar12 = CONCAT44(local_278[0].st_uid,local_278[0].st_mode);
        *(__nlink_t *)((long)local_278 + lVar9 + 0x10) = local_278[0].st_nlink;
        *(undefined8 *)((long)local_278 + lVar9 + 0x18) = uVar12;
        _Var2 = local_278[0].st_rdev;
        *(ulong *)((long)local_278 + lVar9 + 0x20) =
             CONCAT44(local_278[0].__pad0,local_278[0].st_gid);
        *(__dev_t *)((long)local_278 + lVar9 + 0x28) = _Var2;
        _Var3 = local_278[0].st_blksize;
        *(__off_t *)((long)local_278 + lVar9 + 0x30) = local_278[0].st_size;
        *(__blksize_t *)((long)local_278 + lVar9 + 0x38) = _Var3;
        _Var4 = local_278[0].st_atim.tv_sec;
        *(__blkcnt_t *)((long)local_278 + lVar9 + 0x40) = local_278[0].st_blocks;
        *(__time_t *)((long)local_278 + lVar9 + 0x48) = _Var4;
        _Var4 = local_278[0].st_mtim.tv_sec;
        *(long *)((long)local_278 + lVar9 + 0x50) = local_278[0].st_atim.tv_nsec;
        *(__time_t *)((long)local_278 + lVar9 + 0x58) = _Var4;
        _Var4 = local_278[0].st_ctim.tv_sec;
        *(long *)((long)local_278 + lVar9 + 0x60) = local_278[0].st_mtim.tv_nsec;
        *(__time_t *)((long)local_278 + lVar9 + 0x68) = _Var4;
        lVar16 = local_278[0].__unused[0];
        *(long *)((long)local_278 + lVar9 + 0x70) = local_278[0].st_ctim.tv_nsec;
        *(long *)((long)local_278 + lVar9 + 0x78) = lVar16;
        lVar16 = local_278[0].__unused[2];
        *(long *)((long)local_278 + lVar9 + 0x80) = local_278[0].__unused[1];
        *(long *)((long)local_278 + lVar9 + 0x88) = lVar16;
      }
    }
  }
  for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
    if ((DAT_0012d648 != '\0') || ((iVar7 == 0 && (DAT_0012d649 != '\0')))) {
      if (local_288[(long)iVar7 * 0x4c] == -2) {
        if ((((*(uint *)((long)local_278 + (long)iVar7 * 0x130 + 0x18) & 0xf000) == 0x8000) &&
            ((*(uint *)((long)local_278 + (long)iVar7 * 0x130 + 0x18) & 0x1ff) == 0)) &&
           (*(long *)((long)local_278 + (long)iVar7 * 0x130 + 0x30) == 0)) {
          bVar5 = true;
        }
        else {
          bVar5 = false;
        }
      }
      else if ((((local_288[(long)iVar7 * 0x4c] == -5) || (local_288[(long)iVar7 * 0x4c] == -0xc))
               && (param_1 == 0)) &&
              ((local_288[(long)(1 - iVar7) * 0x4c] == -2 ||
               (local_288[(long)(1 - iVar7) * 0x4c] == 0)))) {
        bVar5 = true;
      }
      else {
        bVar5 = false;
      }
      if (bVar5) {
        local_288[(long)iVar7 * 0x4c] = -1;
      }
    }
  }
  for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
    if (local_288[(long)iVar7 * 0x4c] == -1) {
      memset((void *)((long)local_278 + (long)iVar7 * 0x130),0,0x90);
      *(undefined4 *)((long)local_278 + (long)iVar7 * 0x130 + 0x18) =
           *(undefined4 *)((long)local_278 + (long)(1 - iVar7) * 0x130 + 0x18);
    }
  }
  for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
    iVar8 = local_288[(long)iVar7 * 0x4c];
    if (-1 < -3 - iVar8) {
      piVar11 = __errno_location();
      *piVar11 = -3 - iVar8;
      FUN_001108ec((&local_280)[(long)iVar7 * 0x26]);
      local_2f4 = 2;
    }
  }
  if (((local_2f4 == 0) && (param_1 == 0)) &&
     (((local_130 & 0xf000) == 0x4000) != ((local_278[0].st_mode & 0xf000) == 0x4000))) {
    uVar6 = (uint)((local_278[0].st_mode & 0xf000) == 0x4000);
    iVar7 = 1 - uVar6;
    pcVar14 = (&local_280)[(long)(int)uVar6 * 0x26];
    pcVar15 = (&local_280)[(long)iVar7 * 0x26];
    uVar12 = FUN_00113111(pcVar14);
    local_2e0 = (char *)FUN_0010c9bd(pcVar15,uVar12);
    (&local_280)[(long)iVar7 * 0x26] = local_2e0;
    pcVar15 = (&local_280)[(long)iVar7 * 0x26];
    iVar8 = strcmp(pcVar14,"-");
    if (iVar8 == 0) {
      FUN_00110966("cannot compare \'-\' to a directory");
    }
    if (DAT_0012d1f0 == '\0') {
      iVar7 = stat(pcVar15,(stat *)((long)local_278 + (long)iVar7 * 0x130));
    }
    else {
      iVar7 = lstat(pcVar15,(stat *)((long)local_278 + (long)iVar7 * 0x130));
    }
    if (iVar7 != 0) {
      FUN_001108ec(pcVar15);
      local_2f4 = 2;
    }
  }
  if ((local_2f4 == 0) && ((local_288[0] != -1 || (local_158 != -1)))) {
    if ((local_288[0] == -1) || (local_158 == -1)) {
LAB_0010b57a:
      bVar5 = false;
    }
    else {
      if (((local_278[0].st_ino == local_140) && (local_278[0].st_dev == local_148)) ||
         (((((local_278[0].st_mode & 0xf000) == 0x6000 && ((local_130 & 0xf000) == 0x6000)) ||
           (((local_278[0].st_mode & 0xf000) == 0x2000 && ((local_130 & 0xf000) == 0x2000)))) &&
          (local_278[0].st_rdev == local_120)))) {
        bVar5 = true;
      }
      else {
        bVar5 = false;
      }
      if ((((!bVar5) || (local_278[0].st_mode != local_130)) || (local_278[0].st_nlink != local_138)
          ) || (((local_278[0].st_uid != local_12c || (local_278[0].st_gid != local_128)) ||
                ((local_278[0].st_size != local_118 ||
                 ((local_278[0].st_mtim.tv_sec != local_f0 ||
                  (local_278[0].st_ctim.tv_sec != local_e0)))))))) goto LAB_0010b57a;
      bVar5 = true;
    }
    if ((!bVar5) || (DAT_0012d1c8 == '\0')) {
      if ((local_130 & 0xf000) == 0x4000 && (local_278[0].st_mode & 0xf000) == 0x4000) {
        if (DAT_0012d1c0 == 7) {
          FUN_00110966("-D option not supported with directories");
        }
        if ((param_1 == 0) || (DAT_0012d5c9 == '\x01')) {
          local_2f4 = FUN_0010c21b(local_288,compare_files);
        }
        else {
          FUN_001109a8("Common subdirectories: %s and %s\n",local_280,local_150);
        }
      }
      else if (((local_130 & 0xf000) == 0x4000 || (local_278[0].st_mode & 0xf000) == 0x4000) ||
              ((param_1 != 0 &&
               ((((local_278[0].st_mode & 0xf000) != 0x8000 &&
                 ((local_278[0].st_mode & 0xf000) != 0xa000)) ||
                (((local_130 & 0xf000) != 0x8000 && ((local_130 & 0xf000) != 0xa000)))))))) {
        if ((local_288[0] == -1) || (local_158 == -1)) {
          if (((local_130 & 0xf000) == 0x4000 || (local_278[0].st_mode & 0xf000) == 0x4000) &&
             ((DAT_0012d5c9 != '\0' &&
              ((DAT_0012d648 != '\0' || ((DAT_0012d649 != '\0' && (local_288[0] == -1)))))))) {
            local_2f4 = FUN_0010c21b(local_288,compare_files);
          }
          else {
            if (param_1 == 0) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("parent","diff.c",0x52d,"compare_files");
            }
            FUN_001109a8("Only in %s: %s\n",
                         *(undefined8 *)
                          ((long)(int)(uint)(local_288[0] == -1) * 0x130 + param_1 + 8),local_308);
            local_2f4 = 1;
          }
        }
        else {
          uVar12 = FUN_00114997(&local_148);
          pcVar14 = local_150;
          if (DAT_0012d208 != (char *)0x0) {
            pcVar14 = DAT_0012d208;
          }
          uVar13 = FUN_00114997(local_278);
          pcVar15 = local_280;
          if (DAT_0012d200 != (char *)0x0) {
            pcVar15 = DAT_0012d200;
          }
          FUN_001109e2("File %s is a %s while file %s is a %s\n",pcVar15,uVar13,pcVar14,uVar12);
          local_2f4 = 1;
        }
      }
      else if (((local_278[0].st_mode & 0xf000) == 0xa000) || ((local_130 & 0xf000) == 0xa000)) {
        if (DAT_0012d1f0 == '\0') {
                    /* WARNING: Subroutine does not return */
          __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files");
        }
        if (((local_278[0].st_mode & 0xf000) == 0xa000) && ((local_130 & 0xf000) == 0xa000)) {
          local_298[0] = (char *)0x0;
          local_298[1] = (char *)0x0;
          for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
            pcVar14 = (char *)FUN_0011cf7b((&local_280)[(long)iVar7 * 0x26]);
            local_298[iVar7] = pcVar14;
            if (local_298[iVar7] == (char *)0x0) {
              FUN_001108ec((&local_280)[(long)iVar7 * 0x26]);
              local_2f4 = 2;
              break;
            }
          }
          if ((local_2f4 == 0) && (iVar7 = strcmp(local_298[0],local_298[1]), iVar7 != 0)) {
            FUN_001109a8("Symbolic links %s and %s differ\n",local_280,local_150);
            local_2f4 = 1;
          }
          for (iVar7 = 0; iVar7 < 2; iVar7 = iVar7 + 1) {
            free(local_298[iVar7]);
          }
        }
        else {
          uVar12 = FUN_00114997(&local_148);
          pcVar14 = local_150;
          if (DAT_0012d208 != (char *)0x0) {
            pcVar14 = DAT_0012d208;
          }
          uVar13 = FUN_00114997(local_278);
          pcVar15 = local_280;
          if (DAT_0012d200 != (char *)0x0) {
            pcVar15 = DAT_0012d200;
          }
          FUN_001109e2("File %s is a %s while file %s is a %s\n",pcVar15,uVar13,pcVar14,uVar12);
          local_2f4 = 1;
        }
      }
      else if (((((DAT_0012d1ed == '\0') || ((local_278[0].st_mode & 0xf000) != 0x8000)) ||
                ((local_130 & 0xf000) != 0x8000)) ||
               ((local_278[0].st_size == local_118 || (local_278[0].st_size < 1)))) ||
              (local_118 < 1)) {
        if ((local_288[0] == -2) && (local_288[0] = open(local_280,0,0), local_288[0] < 0)) {
          FUN_001108ec(local_280);
          local_2f4 = 2;
        }
        if (local_158 == -2) {
          if (bVar5) {
            local_158 = local_288[0];
          }
          else {
            local_158 = open(local_150,0,0);
            if (local_158 < 0) {
              FUN_001108ec(local_150);
              local_2f4 = 2;
            }
          }
        }
        if (local_2f4 == 0) {
          local_2f4 = FUN_00107206(local_288);
        }
        if ((-1 < local_288[0]) && (iVar7 = close(local_288[0]), iVar7 != 0)) {
          FUN_001108ec(local_280);
          local_2f4 = 2;
        }
        if (((-1 < local_158) && (local_288[0] != local_158)) &&
           (iVar7 = close(local_158), iVar7 != 0)) {
          FUN_001108ec(local_150);
          local_2f4 = 2;
        }
      }
      else {
        pcVar14 = local_150;
        if (DAT_0012d208 != (char *)0x0) {
          pcVar14 = DAT_0012d208;
        }
        pcVar15 = local_280;
        if (DAT_0012d200 != (char *)0x0) {
          pcVar15 = DAT_0012d200;
        }
        FUN_001109a8("Files %s and %s differ\n",pcVar15,pcVar14);
        local_2f4 = 1;
      }
    }
  }
  if (local_2f4 == 0) {
    if ((DAT_0012d64a != '\0') && ((local_278[0].st_mode & 0xf000) != 0x4000)) {
      pcVar14 = local_150;
      if (DAT_0012d208 != (char *)0x0) {
        pcVar14 = DAT_0012d208;
      }
      pcVar15 = local_280;
      if (DAT_0012d200 != (char *)0x0) {
        pcVar15 = DAT_0012d200;
      }
      FUN_001109a8("Files %s and %s are identical\n",pcVar15,pcVar14);
    }
  }
  else {
    iVar7 = fflush_unlocked(stdout);
    if (iVar7 != 0) {
      uVar12 = gettext("standard output");
      FUN_00110925(uVar12);
    }
  }
  free(local_2e0);
  free(local_2d8);
LAB_0010bdc2:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_2f4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dir_read @ 0xbde2

undefined8 dir_read(int *param_1,ulong *param_2)

{
  char *__src;
  char cVar1;
  int iVar2;
  DIR *__dirp;
  int *piVar3;
  dirent *pdVar4;
  ulong uVar5;
  size_t sVar6;
  ulong uVar7;
  ulong local_50;
  char *local_48;
  ulong local_40;
  long local_38;
  
  param_2[1] = 0;
  param_2[2] = 0;
  local_50 = 0;
  local_48 = (char *)0x0;
  if (*param_1 != -1) {
    __dirp = opendir(*(char **)(param_1 + 2));
    if (__dirp == (DIR *)0x0) {
      return 0;
    }
    local_40 = 0x200;
    local_38 = 0;
    local_48 = (char *)FUN_0011c945(0x200);
    param_2[2] = (ulong)local_48;
    while( true ) {
      piVar3 = __errno_location();
      *piVar3 = 0;
      pdVar4 = readdir(__dirp);
      if (pdVar4 == (dirent *)0x0) break;
      __src = pdVar4->d_name;
      sVar6 = strlen(pdVar4->d_name);
      sVar6 = sVar6 + 1;
      if (((*__src != '.') ||
          ((pdVar4->d_name[1] != '\0' && ((pdVar4->d_name[1] != '.' || (pdVar4->d_name[2] != '\0')))
           ))) && (cVar1 = FUN_001141f6(DAT_0012d340,__src), cVar1 == '\0')) {
        while (local_40 < sVar6 + local_38) {
          if (0x3ffffffffffffffe < local_40) {
            FUN_0011cf09();
          }
          local_40 = local_40 << 1;
          local_48 = (char *)FUN_0011c991(local_48,local_40);
          param_2[2] = (ulong)local_48;
        }
        memcpy(local_48 + local_38,__src,sVar6);
        local_38 = local_38 + sVar6;
        local_50 = local_50 + 1;
      }
    }
    piVar3 = __errno_location();
    if (*piVar3 != 0) {
      piVar3 = __errno_location();
      iVar2 = *piVar3;
      closedir(__dirp);
      piVar3 = __errno_location();
      *piVar3 = iVar2;
      return 0;
    }
    iVar2 = closedir(__dirp);
    if (iVar2 != 0) {
      return 0;
    }
  }
  if (0xffffffffffffffd < local_50) {
    FUN_0011cf09();
  }
  uVar5 = FUN_0011c945((local_50 + 1) * 8);
  param_2[1] = uVar5;
  *param_2 = local_50;
  for (uVar7 = 0; uVar7 < local_50; uVar7 = uVar7 + 1) {
    *(char **)(uVar7 * 8 + uVar5) = local_48;
    sVar6 = strlen(local_48);
    local_48 = local_48 + sVar6 + 1;
  }
  *(undefined8 *)(uVar5 + local_50 * 8) = 0;
  return 1;
}




// Function: compare_collated @ 0xc084

int compare_collated(char *param_1,char *param_2)

{
  int *piVar1;
  undefined8 uVar2;
  int local_1c;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  if (DAT_0012d1ef == '\0') {
    local_1c = strcoll(param_1,param_2);
  }
  else {
    local_1c = strcasecmp(param_1,param_2);
  }
  piVar1 = __errno_location();
  if (*piVar1 != 0) {
    uVar2 = gettext("cannot compare file names \'%s\' and \'%s\'");
    piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(0,*piVar1,uVar2,param_1,param_2);
  }
  return local_1c;
}




// Function: compare_names @ 0xc140

int compare_names(char *param_1,char *param_2)

{
  int iVar1;
  
  if ((DAT_0012d660 == '\0') ||
     ((iVar1 = FUN_0010c084(param_1,param_2), iVar1 == 0 && (DAT_0012d1ef == '\0')))) {
    iVar1 = strcmp(param_1,param_2);
  }
  return iVar1;
}




// Function: compare_names_for_qsort @ 0xc1a0

int compare_names_for_qsort(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  
  __s1 = (char *)*param_1;
  __s2 = (char *)*param_2;
  if ((DAT_0012d660 == '\0') || (iVar1 = FUN_0010c084(__s1,__s2), iVar1 == 0)) {
    iVar1 = strcmp(__s1,__s2);
  }
  return iVar1;
}




// Function: diff_dirs @ 0xc21b

int diff_dirs(int *param_1,code *param_2)

{
  undefined8 uVar1;
  long *__src;
  char *__s2;
  char cVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  long in_FS_OFFSET;
  int local_a0;
  int local_9c;
  long *local_80;
  long *local_68 [2];
  size_t local_58;
  long *local_50 [6];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_a0 = 0;
  if (((*param_1 == -1) || (cVar2 = FUN_0010c796(param_1,0), cVar2 != '\0')) &&
     ((param_1[0x4c] == -1 || (cVar2 = FUN_0010c796(param_1,1), cVar2 != '\0')))) {
    uVar1 = *(undefined8 *)(param_1 + (long)(int)(uint)(*param_1 == -1) * 0x4c + 2);
    uVar5 = gettext("%s: recursive directory loop");
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar5,uVar1);
  }
  for (local_9c = 0; local_9c < 2; local_9c = local_9c + 1) {
    cVar2 = FUN_0010bde2(param_1 + (long)local_9c * 0x4c,&local_58 + (long)local_9c * 3);
    if (cVar2 != '\x01') {
      FUN_001108ec(*(undefined8 *)(param_1 + (long)local_9c * 0x4c + 2));
      local_a0 = 2;
    }
  }
  if (local_a0 == 0) {
    local_68[0] = local_50[0];
    local_68[1] = local_50[3];
    DAT_0012d660 = 1;
    iVar3 = _setjmp((__jmp_buf_tag *)&DAT_0012d680);
    if (iVar3 != 0) {
      DAT_0012d660 = 0;
    }
    for (local_9c = 0; local_9c < 2; local_9c = local_9c + 1) {
      qsort(local_68[local_9c],(&local_58)[(long)local_9c * 3],8,FUN_0010c1a0);
    }
    if ((DAT_0012d2b8 != 0) && (*(long *)(param_1 + 0x98) == 0)) {
      while ((*local_68[0] != 0 && (iVar3 = FUN_0010c140(*local_68[0],DAT_0012d2b8), iVar3 < 0))) {
        local_68[0] = local_68[0] + 1;
      }
      while ((*local_68[1] != 0 && (iVar3 = FUN_0010c140(*local_68[1],DAT_0012d2b8), iVar3 < 0))) {
        local_68[1] = local_68[1] + 1;
      }
    }
    while ((*local_68[0] != 0 || (*local_68[1] != 0))) {
      if (*local_68[0] == 0) {
        iVar3 = 1;
      }
      else if (*local_68[1] == 0) {
        iVar3 = -1;
      }
      else {
        iVar3 = FUN_0010c140(*local_68[0],*local_68[1]);
      }
      if (((iVar3 == 0) && (DAT_0012d1ef != '\0')) &&
         (iVar4 = strcmp((char *)*local_68[0],(char *)*local_68[1]), iVar4 != 0)) {
        __src = local_68[(iVar4 >> 0x1f) + 1];
        __s2 = (char *)*local_68[-(iVar4 >> 0x1f)];
        local_80 = __src;
        do {
          local_80 = local_80 + 1;
          if ((*local_80 == 0) || (iVar4 = FUN_0010c140(*local_80,__s2), iVar4 != 0))
          goto LAB_0010c676;
          iVar4 = strcmp((char *)*local_80,__s2);
        } while (iVar4 < 0);
        if (iVar4 == 0) {
          memmove(__src + 1,__src,(long)local_80 - (long)__src);
          *__src = (long)__s2;
        }
      }
LAB_0010c676:
      if (iVar3 < 0) {
        lVar7 = 0;
      }
      else {
        lVar7 = *local_68[1];
        local_68[1] = local_68[1] + 1;
      }
      if (iVar3 < 1) {
        lVar6 = *local_68[0];
        local_68[0] = local_68[0] + 1;
      }
      else {
        lVar6 = 0;
      }
      iVar3 = (*param_2)(param_1,lVar6,lVar7);
      if (local_a0 < iVar3) {
        local_a0 = iVar3;
      }
    }
  }
  for (local_9c = 0; local_9c < 2; local_9c = local_9c + 1) {
    free(local_50[(long)local_9c * 3]);
    free(local_50[(long)local_9c * 3 + 1]);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_a0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dir_loop @ 0xc796

undefined8 dir_loop(long param_1,int param_2)

{
  bool bVar1;
  long local_10;
  
  local_10 = param_1;
  do {
    local_10 = *(long *)(local_10 + 0x260);
    if (local_10 == 0) {
      return 0;
    }
    if (((*(long *)((long)param_2 * 0x130 + local_10 + 0x18) ==
          *(long *)((long)param_2 * 0x130 + param_1 + 0x18)) &&
        (*(long *)((long)param_2 * 0x130 + local_10 + 0x10) ==
         *(long *)((long)param_2 * 0x130 + param_1 + 0x10))) ||
       (((((*(uint *)((long)param_2 * 0x130 + local_10 + 0x28) & 0xf000) == 0x6000 &&
          ((*(uint *)((long)param_2 * 0x130 + param_1 + 0x28) & 0xf000) == 0x6000)) ||
         (((*(uint *)((long)param_2 * 0x130 + local_10 + 0x28) & 0xf000) == 0x2000 &&
          ((*(uint *)((long)param_2 * 0x130 + param_1 + 0x28) & 0xf000) == 0x2000)))) &&
        (*(long *)((long)param_2 * 0x130 + local_10 + 0x38) ==
         *(long *)((long)param_2 * 0x130 + param_1 + 0x38))))) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
  } while (!bVar1);
  return 1;
}




// Function: find_dir_file_pathname @ 0xc9bd

undefined8 find_dir_file_pathname(undefined8 param_1,char *param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  char *local_180;
  long *local_178;
  undefined1 local_168 [8];
  long *local_160;
  void *local_158;
  undefined4 local_148 [2];
  undefined8 local_140;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_160 = (long *)0x0;
  local_158 = (void *)0x0;
  local_180 = param_2;
  if (DAT_0012d1ef != '\0') {
    local_148[0] = 0;
    local_140 = param_1;
    cVar1 = FUN_0010bde2(local_148,local_168);
    if (cVar1 != '\0') {
      DAT_0012d660 = 1;
      iVar2 = _setjmp((__jmp_buf_tag *)&DAT_0012d680);
      if (iVar2 == 0) {
        for (local_178 = local_160; *local_178 != 0; local_178 = local_178 + 1) {
          iVar2 = FUN_0010c140(*local_178,param_2);
          if (iVar2 == 0) {
            iVar2 = strcmp((char *)*local_178,param_2);
            if (iVar2 == 0) {
              local_180 = (char *)*local_178;
              break;
            }
            if (local_180 == param_2) {
              local_180 = (char *)*local_178;
            }
          }
        }
      }
    }
  }
  uVar3 = FUN_00114ade(param_1,local_180,0);
  free(local_160);
  free(local_158);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ed_script @ 0xcb85

void print_ed_script(undefined8 param_1)

{
  FUN_0011245b(param_1,FUN_00112449,FUN_0010cbb5);
  return;
}




// Function: print_ed_hunk @ 0xcbb5

void print_ed_hunk(undefined8 param_1)

{
  bool bVar1;
  uint uVar2;
  long in_FS_OFFSET;
  undefined8 local_38;
  undefined8 local_30;
  long local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00112af4(param_1,&local_38,&local_30,&local_28,&local_20);
  if (uVar2 != 0) {
    FUN_00111b50();
    FUN_00112a37(0x2c,&DAT_0012d360,local_38,local_30);
    fputc_unlocked((int)(char)(&DAT_00125780)[uVar2],DAT_0012d5c0);
    fputc_unlocked(10,DAT_0012d5c0);
    if (uVar2 != 1) {
      bVar1 = true;
      for (local_18 = local_28; local_18 <= local_20; local_18 = local_18 + 1) {
        if (!bVar1) {
          fputs_unlocked("a\n",DAT_0012d5c0);
          bVar1 = true;
        }
        if ((**(char **)(local_18 * 8 + DAT_0012d548) == '.') &&
           (*(char *)(*(long *)(local_18 * 8 + DAT_0012d548) + 1) == '\n')) {
          fputs_unlocked("..\n.\ns/.//\n",DAT_0012d5c0);
          bVar1 = false;
        }
        else {
          FUN_001124cf(&DAT_00125164,local_18 * 8 + DAT_0012d548);
        }
      }
      if (bVar1) {
        fputs_unlocked(".\n",DAT_0012d5c0);
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: pr_forward_ed_script @ 0xcd6a

void pr_forward_ed_script(undefined8 param_1)

{
  FUN_0011245b(param_1,FUN_00112437,FUN_0010cd9a);
  return;
}




// Function: pr_forward_ed_hunk @ 0xcd9a

void pr_forward_ed_hunk(undefined8 param_1)

{
  uint uVar1;
  long in_FS_OFFSET;
  undefined8 local_38;
  undefined8 local_30;
  long local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00112af4(param_1,&local_38,&local_30,&local_28,&local_20);
  if (uVar1 != 0) {
    FUN_00111b50();
    fputc_unlocked((int)(char)(&DAT_00125780)[uVar1],DAT_0012d5c0);
    FUN_00112a37(0x20,&DAT_0012d360,local_38,local_30);
    fputc_unlocked(10,DAT_0012d5c0);
    if (uVar1 != 1) {
      for (local_18 = local_28; local_18 <= local_20; local_18 = local_18 + 1) {
        FUN_001124cf(&DAT_00125164,local_18 * 8 + DAT_0012d548);
      }
      fputs_unlocked(".\n",DAT_0012d5c0);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_rcs_script @ 0xceba

void print_rcs_script(undefined8 param_1)

{
  FUN_0011245b(param_1,FUN_00112437,FUN_0010ceea);
  return;
}




// Function: print_rcs_hunk @ 0xceea

void print_rcs_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined8 local_58;
  undefined8 local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00112af4(param_1,&local_58,&local_50,&local_48,&local_40);
  if (uVar1 != 0) {
    FUN_00111b50();
    FUN_001129d0(&DAT_0012d360,local_58,local_50,&local_38,&local_30);
    if ((uVar1 & 1) != 0) {
      if (local_30 < local_38) {
        lVar2 = 1;
      }
      else {
        lVar2 = (local_30 - local_38) + 1;
      }
      fprintf(DAT_0012d5c0,"d%ld %ld\n",local_38,lVar2);
    }
    if ((uVar1 & 2) != 0) {
      FUN_001129d0(&DAT_0012d490,local_48,local_40,&local_28,&local_20);
      if (local_20 < local_28) {
        lVar2 = 1;
      }
      else {
        lVar2 = (local_20 - local_28) + 1;
      }
      fprintf(DAT_0012d5c0,"a%ld %ld\n",local_30,lVar2);
      for (local_18 = local_48; local_18 <= local_40; local_18 = local_18 + 1) {
        FUN_001124cf(&DAT_00125164,local_18 * 8 + DAT_0012d548);
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ifdef_script @ 0xd076

void print_ifdef_script(undefined8 param_1)

{
  DAT_0012d748 = -DAT_0012d448;
  DAT_0012d750 = DAT_0012d748;
  FUN_0011245b(param_1,FUN_00112437,FUN_0010d11e);
  if ((DAT_0012d748 < DAT_0012d430) || (DAT_0012d750 < DAT_0012d560)) {
    FUN_00111b50();
    FUN_0010d221(DAT_0012d2e0,DAT_0012d748,DAT_0012d430,DAT_0012d750,DAT_0012d560);
  }
  return;
}




// Function: print_ifdef_hunk @ 0xd11e

void print_ifdef_hunk(undefined8 param_1)

{
  uint uVar1;
  long in_FS_OFFSET;
  long local_30;
  long local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00112af4(param_1,&local_30,&local_28,&local_20,&local_18);
  if (uVar1 != 0) {
    FUN_00111b50();
    if ((DAT_0012d748 < local_30) || (DAT_0012d750 < local_20)) {
      FUN_0010d221(DAT_0012d2e0,DAT_0012d748,local_30,DAT_0012d750,local_20);
    }
    DAT_0012d748 = local_28 + 1;
    DAT_0012d750 = local_18 + 1;
    FUN_0010d221((&DAT_0012d2e0)[uVar1],local_30,DAT_0012d748,local_20,DAT_0012d750);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: format_ifdef @ 0xd221

void format_ifdef(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  long in_FS_OFFSET;
  undefined8 *local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 *local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = &DAT_0012d360;
  local_30 = &DAT_0012d490;
  local_40 = param_2;
  local_38 = param_3;
  local_28 = param_4;
  local_20 = param_5;
  FUN_0010d2bc(DAT_0012d5c0,param_1,0,&local_48);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: format_group @ 0xd2bc

char * format_group(FILE *param_1,char *param_2,char param_3,long param_4)

{
  int *piVar1;
  intmax_t iVar2;
  long lVar3;
  char *pcVar4;
  char *__nptr;
  char cVar5;
  long in_FS_OFFSET;
  int local_6c;
  char *local_68;
  FILE *local_60;
  FILE *local_58;
  char *local_50;
  long local_48 [5];
  
  local_48[3] = *(long *)(in_FS_OFFSET + 0x28);
LAB_0010d542:
  do {
    cVar5 = *param_2;
    if ((cVar5 == param_3) || (cVar5 == '\0')) {
      if (local_48[3] != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return param_2;
    }
    local_50 = param_2 + 1;
    __nptr = local_50;
    if (cVar5 != '%') goto LAB_0010d52f;
    __nptr = param_2 + 2;
    cVar5 = *local_50;
    switch(cVar5) {
    default:
      param_2 = (char *)FUN_0010da03(param_1,param_2,0,0,param_4);
      if (param_2 != (char *)0x0) break;
LAB_0010d522:
      cVar5 = '%';
      __nptr = local_50;
    case '%':
LAB_0010d52f:
      param_2 = __nptr;
      if (param_1 != (FILE *)0x0) {
        putc_unlocked((int)cVar5,param_1);
      }
      break;
    case '(':
      for (local_6c = 0; local_6c < 2; local_6c = local_6c + 1) {
        if ((int)*__nptr - 0x30U < 10) {
          piVar1 = __errno_location();
          *piVar1 = 0;
          iVar2 = strtoimax(__nptr,&local_68,10);
          local_48[local_6c] = iVar2;
          piVar1 = __errno_location();
          pcVar4 = local_68;
          if (*piVar1 != 0) goto LAB_0010d522;
        }
        else {
          lVar3 = FUN_0010d577(param_4,(int)*__nptr);
          local_48[local_6c] = lVar3;
          if (local_48[local_6c] < 0) goto LAB_0010d522;
          pcVar4 = __nptr + 1;
        }
        __nptr = pcVar4 + 1;
        if (*pcVar4 != (&DAT_0012517c)[local_6c]) goto LAB_0010d522;
      }
      if (local_48[0] == local_48[1]) {
        local_58 = (FILE *)0x0;
        local_60 = param_1;
      }
      else {
        local_60 = (FILE *)0x0;
        local_58 = param_1;
      }
      param_2 = (char *)format_group(local_60,__nptr,0x3a,param_4);
      if ((*param_2 != '\0') &&
         (param_2 = (char *)format_group(local_58,param_2 + 1,0x29,param_4), *param_2 != '\0')) {
        param_2 = param_2 + 1;
      }
      break;
    case '<':
      FUN_0010d6bd(param_1,DAT_0012d308,param_4);
      param_2 = __nptr;
      break;
    case '=':
      FUN_0010d6bd(param_1,DAT_0012d300,param_4);
      param_2 = __nptr;
      break;
    case '>':
      goto switchD_0010d345_caseD_3e;
    }
  } while( true );
switchD_0010d345_caseD_3e:
  FUN_0010d6bd(param_1,DAT_0012d310,param_4 + 0x18);
  param_2 = __nptr;
  goto LAB_0010d542;
}




// Function: groups_letter_value @ 0xd577

long groups_letter_value(undefined8 *param_1,undefined1 param_2)

{
  long lVar1;
  undefined1 local_14;
  undefined8 *local_10;
  
  local_14 = param_2;
  local_10 = param_1;
  switch(param_2) {
  case 0x45:
    local_14 = 0x65;
    local_10 = param_1 + 3;
    break;
  case 0x46:
    local_14 = 0x66;
    local_10 = param_1 + 3;
    break;
  case 0x4c:
    local_14 = 0x6c;
    local_10 = param_1 + 3;
    break;
  case 0x4d:
    local_14 = 0x6d;
    local_10 = param_1 + 3;
    break;
  case 0x4e:
    local_14 = 0x6e;
    local_10 = param_1 + 3;
  }
  switch(local_14) {
  case 0x65:
    lVar1 = FUN_001129a8(*local_10,local_10[1]);
    lVar1 = lVar1 + -1;
    break;
  case 0x66:
    lVar1 = FUN_001129a8(*local_10,local_10[1]);
    break;
  default:
    lVar1 = -1;
    break;
  case 0x6c:
    lVar1 = FUN_001129a8(*local_10,local_10[2]);
    lVar1 = lVar1 + -1;
    break;
  case 0x6d:
    lVar1 = FUN_001129a8(*local_10,local_10[2]);
    break;
  case 0x6e:
    lVar1 = local_10[2] - local_10[1];
  }
  return lVar1;
}




// Function: print_ifdef_lines @ 0xd6bd

void print_ifdef_lines(FILE *param_1,char *param_2,long *param_3)

{
  char *pcVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  char *pcVar5;
  char *pcVar6;
  char cVar7;
  long local_50;
  
  lVar2 = *param_3;
  lVar3 = *(long *)(lVar2 + 0xb8);
  local_50 = param_3[1];
  lVar4 = param_3[2];
  if (param_1 == (FILE *)0x0) {
    return;
  }
  if ((DAT_0012d2a1 != '\x01') && (*param_2 == '%')) {
    if ((param_2[1] == 'l') &&
       (((param_2[2] == '\n' && (param_2[3] == '\0')) && (local_50 < lVar4)))) {
      fwrite_unlocked(*(void **)(lVar3 + local_50 * 8),1,
                      (*(long *)(lVar3 + lVar4 * 8) +
                      (ulong)(*(char *)(*(long *)(lVar3 + lVar4 * 8) + -1) != '\n')) -
                      *(long *)(lVar3 + local_50 * 8),param_1);
      return;
    }
    if ((param_2[1] == 'L') && (param_2[2] == '\0')) {
      fwrite_unlocked(*(void **)(lVar3 + local_50 * 8),1,
                      *(long *)(lVar3 + lVar4 * 8) - *(long *)(lVar3 + local_50 * 8),param_1);
      return;
    }
  }
  do {
    pcVar5 = param_2;
    if (lVar4 <= local_50) {
      return;
    }
    while( true ) {
      pcVar1 = pcVar5 + 1;
      cVar7 = *pcVar5;
      if (cVar7 == '\0') break;
      pcVar6 = pcVar1;
      if (cVar7 == '%') {
        pcVar6 = pcVar5 + 2;
        cVar7 = *pcVar1;
        if (cVar7 == 'l') {
          FUN_0011267d(*(undefined8 *)(lVar3 + local_50 * 8),
                       *(long *)(lVar3 + (local_50 + 1) * 8) -
                       (ulong)(*(char *)(*(long *)(lVar3 + (local_50 + 1) * 8) + -1) == '\n'),0,0);
          pcVar5 = pcVar6;
        }
        else if (cVar7 < 'm') {
          if (cVar7 == '%') goto LAB_0010d9c0;
          if (cVar7 != 'L') goto LAB_0010d989;
          FUN_0011267d(*(undefined8 *)(lVar3 + local_50 * 8),
                       *(undefined8 *)(lVar3 + (local_50 + 1) * 8),0,0);
          pcVar5 = pcVar6;
        }
        else {
LAB_0010d989:
          pcVar5 = (char *)FUN_0010da03(param_1,pcVar5,lVar2,local_50,0);
          if (pcVar5 == (char *)0x0) {
            cVar7 = '%';
            pcVar6 = pcVar1;
            goto LAB_0010d9c0;
          }
        }
      }
      else {
LAB_0010d9c0:
        putc_unlocked((int)cVar7,param_1);
        pcVar5 = pcVar6;
      }
    }
    local_50 = local_50 + 1;
  } while( true );
}




// Function: do_printf_spec @ 0xda03

char * do_printf_spec(FILE *param_1,void *param_2,long param_3,undefined8 param_4,undefined8 param_5)

{
  long lVar1;
  void *__src;
  FILE *__stream;
  char *__n;
  size_t __n_00;
  char *pcVar2;
  ulong uVar3;
  undefined1 *puVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  long in_FS_OFFSET;
  undefined1 auStack_88 [8];
  undefined8 local_80;
  undefined8 local_78;
  long local_70;
  void *local_68;
  FILE *local_60;
  char local_4b;
  char local_4a;
  char local_49;
  char *local_48;
  long local_40;
  long local_38;
  char *local_30;
  size_t local_28;
  char *local_20;
  char *local_18;
  long local_10;
  
  puVar4 = auStack_88;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = (char *)((long)param_2 + 1);
  do {
    do {
      pcVar2 = local_48 + 1;
      local_4a = *local_48;
      local_48 = pcVar2;
    } while (local_4a == '-');
  } while ((local_4a == '\'') || (local_4a == '0'));
  while ((int)local_4a - 0x30U < 10) {
    local_4a = *local_48;
    local_48 = local_48 + 1;
  }
  if (local_4a == '.') {
    do {
      pcVar2 = local_48 + 1;
      local_4a = *local_48;
      local_48 = pcVar2;
    } while ((int)local_4a - 0x30U < 10);
  }
  pcVar2 = local_48 + 1;
  local_49 = *local_48;
  local_80 = param_5;
  local_78 = param_4;
  local_70 = param_3;
  local_68 = param_2;
  local_60 = param_1;
  local_48 = pcVar2;
  if ((int)local_4a - 0x58U < 0x21) {
    uVar3 = 1L << ((byte)((int)local_4a - 0x58U) & 0x3f);
    if ((uVar3 & 0x100801001) != 0) {
      if (param_3 == 0) {
        local_40 = FUN_0010d577(param_5,(int)local_49);
        if (local_40 < 0) {
          pcVar2 = (char *)0x0;
          puVar6 = auStack_88;
          goto LAB_0010dd5e;
        }
      }
      else {
        if (local_49 != 'n') {
          pcVar2 = (char *)0x0;
          puVar6 = auStack_88;
          goto LAB_0010dd5e;
        }
        local_40 = FUN_001129a8(param_3,param_4);
      }
      pcVar2 = local_48;
      puVar6 = auStack_88;
      if (local_60 != (FILE *)0x0) {
        local_38 = local_40;
        local_30 = local_48 + (-2 - (long)local_68);
        local_28 = 1;
        if (local_48 + (1 - (long)local_68) < (char *)0xfa1) {
          uVar3 = ((ulong)(local_48 + (0x37 - (long)local_68)) / 0x10) * 0x10;
          for (; puVar4 != auStack_88 + -(uVar3 & 0xfffffffffffff000); puVar4 = puVar4 + -0x1000) {
            *(undefined8 *)(puVar4 + -8) = *(undefined8 *)(puVar4 + -8);
          }
          lVar1 = -(ulong)((uint)uVar3 & 0xfff);
          puVar5 = puVar4 + lVar1;
          if ((uVar3 & 0xfff) != 0) {
            *(undefined8 *)(puVar4 + ((ulong)((uint)uVar3 & 0xfff) - 8) + lVar1) =
                 *(undefined8 *)(puVar4 + ((ulong)((uint)uVar3 & 0xfff) - 8) + lVar1);
          }
          pcVar2 = (char *)(((ulong)(puVar4 + lVar1 + 0xf) & 0xfffffffffffffff0) + 0x1f &
                           0xffffffffffffffe0);
        }
        else {
          pcVar2 = (char *)FUN_0011cf49(local_48 + (1 - (long)local_68));
          puVar5 = auStack_88;
        }
        __n = local_30;
        __src = local_68;
        local_18 = pcVar2 + (long)(local_30 + local_28);
        local_20 = pcVar2;
        *(undefined8 *)(puVar5 + -8) = 0x10dce7;
        memcpy(pcVar2,__src,(size_t)__n);
        __n_00 = local_28;
        pcVar2 = local_20 + (long)local_30;
        *(undefined8 *)(puVar5 + -8) = 0x10dd0c;
        memcpy(pcVar2,&DAT_00125238,__n_00);
        pcVar2 = local_20;
        lVar1 = local_38;
        __stream = local_60;
        *local_18 = local_4a;
        local_18[1] = '\0';
        local_18 = local_18 + 1;
        *(undefined8 *)(puVar5 + -8) = 0x10dd41;
        fprintf(__stream,pcVar2,lVar1);
        pcVar2 = local_20;
        *(undefined8 *)(puVar5 + -8) = 0x10dd4d;
        FUN_001168f3(pcVar2);
        pcVar2 = local_48;
        puVar6 = puVar5;
      }
      goto LAB_0010dd5e;
    }
    if ((uVar3 & 0x800) != 0) {
      if (local_49 == '\'') {
        local_48 = (char *)FUN_0010dd74(pcVar2,&local_4b);
        if (local_48 == (char *)0x0) {
          pcVar2 = (char *)0x0;
          puVar6 = auStack_88;
        }
        else {
          pcVar2 = local_48;
          puVar6 = auStack_88;
          if (local_60 != (FILE *)0x0) {
            putc_unlocked((int)local_4b,local_60);
            pcVar2 = local_48;
            puVar6 = auStack_88;
          }
        }
      }
      else {
        pcVar2 = (char *)0x0;
        puVar6 = auStack_88;
      }
      goto LAB_0010dd5e;
    }
  }
  pcVar2 = (char *)0x0;
  puVar6 = auStack_88;
LAB_0010dd5e:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pcVar2;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar6 + -8) = 0x10dd72;
  __stack_chk_fail();
}




// Function: scan_char_literal @ 0xdd74

char * scan_char_literal(char *param_1,char *param_2)

{
  uint uVar1;
  char *pcVar2;
  char *pcVar3;
  char local_1e;
  
  local_1e = *param_1;
  if (local_1e == '\\') {
    local_1e = '\0';
    pcVar2 = param_1 + 1;
    while( true ) {
      pcVar3 = pcVar2 + 1;
      if (*pcVar2 == '\'') break;
      uVar1 = (int)*pcVar2 - 0x30;
      if (7 < uVar1) {
        return (char *)0x0;
      }
      local_1e = (char)uVar1 + local_1e * '\b';
      pcVar2 = pcVar3;
    }
    if (((long)(pcVar3 + (-2 - (long)param_1)) < 1) || (3 < (long)(pcVar3 + (-2 - (long)param_1))))
    {
      return (char *)0x0;
    }
  }
  else {
    if ((local_1e < ']') && ((local_1e == '\0' || (local_1e == '\'')))) {
      return (char *)0x0;
    }
    pcVar3 = param_1 + 2;
    if (param_1[1] != '\'') {
      return (char *)0x0;
    }
  }
  *param_2 = local_1e;
  return pcVar3;
}




// Function: file_block_read @ 0xde51

void file_block_read(undefined4 *param_1,ulong param_2)

{
  ulong uVar1;
  
  if ((param_2 != 0) && (*(char *)((long)param_1 + 0x121) != '\x01')) {
    uVar1 = FUN_0011d1d3(*param_1,*(long *)(param_1 + 0x28) + *(long *)(param_1 + 0x2c),param_2);
    if (uVar1 == 0xffffffffffffffff) {
      FUN_00110925(*(undefined8 *)(param_1 + 2));
    }
    *(ulong *)(param_1 + 0x2c) = *(long *)(param_1 + 0x2c) + uVar1;
    *(bool *)((long)param_1 + 0x121) = uVar1 < param_2;
  }
  return;
}




// Function: sip @ 0xdf00

bool sip(int *param_1,char param_2)

{
  size_t __n;
  int iVar1;
  undefined8 uVar2;
  __off_t _Var3;
  void *pvVar4;
  
  if (*param_1 < 0) {
    param_1[0x2a] = 8;
    param_1[0x2b] = 0;
    uVar2 = FUN_0011c945(*(undefined8 *)(param_1 + 0x2a));
    *(undefined8 *)(param_1 + 0x28) = uVar2;
  }
  else {
    uVar2 = FUN_0011d2a2(8,*(undefined8 *)(param_1 + 0x12),0x7fffffffffffffef);
    *(undefined8 *)(param_1 + 0x2a) = uVar2;
    uVar2 = FUN_0011c945(*(undefined8 *)(param_1 + 0x2a));
    *(undefined8 *)(param_1 + 0x28) = uVar2;
    if (param_2 != '\x01') {
      iVar1 = FUN_001131e6(*param_1,0);
      FUN_0010de51(param_1,*(undefined8 *)(param_1 + 0x2a));
      __n = *(size_t *)(param_1 + 0x2c);
      if (iVar1 != 0) {
        _Var3 = lseek(*param_1,-__n,1);
        if (_Var3 < 0) {
          FUN_00110925(*(undefined8 *)(param_1 + 2));
        }
        FUN_001131e6(*param_1,iVar1);
        param_1[0x2c] = 0;
        param_1[0x2d] = 0;
        *(undefined1 *)((long)param_1 + 0x121) = 0;
      }
      pvVar4 = memchr(*(void **)(param_1 + 0x28),0,__n);
      return pvVar4 != (void *)0x0;
    }
  }
  param_1[0x2c] = 0;
  param_1[0x2d] = 0;
  *(undefined1 *)((long)param_1 + 0x121) = 0;
  return false;
}




// Function: slurp @ 0xe09d

void slurp(int *param_1)

{
  ulong uVar1;
  ulong uVar2;
  undefined8 uVar3;
  
  if (-1 < *param_1) {
    if ((param_1[10] & 0xf000U) == 0x8000) {
      uVar1 = *(ulong *)(param_1 + 0x10);
      uVar2 = (uVar1 & 0xfffffffffffffff8) + 0x10;
      if (((*(ulong *)(param_1 + 0x10) != uVar1) || (uVar2 < uVar1)) || (0x7ffffffffffffffe < uVar2)
         ) {
        FUN_0011cf09();
      }
      if (*(ulong *)(param_1 + 0x2a) < uVar2) {
        *(ulong *)(param_1 + 0x2a) = uVar2;
        uVar3 = FUN_0011c991(*(undefined8 *)(param_1 + 0x28),uVar2);
        *(undefined8 *)(param_1 + 0x28) = uVar3;
      }
      if ((*(ulong *)(param_1 + 0x2c) <= uVar1) &&
         (FUN_0010de51(param_1,(uVar1 - *(long *)(param_1 + 0x2c)) + 1),
         *(ulong *)(param_1 + 0x2c) <= uVar1)) {
        return;
      }
    }
    FUN_0010de51(param_1,*(long *)(param_1 + 0x2a) - *(long *)(param_1 + 0x2c));
    if (*(long *)(param_1 + 0x2c) != 0) {
      while (*(long *)(param_1 + 0x2c) == *(long *)(param_1 + 0x2a)) {
        if (0x3ffffffffffffff7 < *(ulong *)(param_1 + 0x2a)) {
          FUN_0011cf09();
        }
        *(long *)(param_1 + 0x2a) = *(long *)(param_1 + 0x2a) * 2;
        uVar3 = FUN_0011c991(*(undefined8 *)(param_1 + 0x28),*(undefined8 *)(param_1 + 0x2a));
        *(undefined8 *)(param_1 + 0x28) = uVar3;
        FUN_0010de51(param_1,*(long *)(param_1 + 0x2a) - *(long *)(param_1 + 0x2c));
      }
      *(ulong *)(param_1 + 0x2a) = *(long *)(param_1 + 0x2c) + 0x10U & 0xfffffffffffffff8;
      uVar3 = FUN_0011c991(*(undefined8 *)(param_1 + 0x28),*(undefined8 *)(param_1 + 0x2a));
      *(undefined8 *)(param_1 + 0x28) = uVar3;
    }
  }
  return;
}




// Function: find_and_hash_each_line @ 0xe2f8

void find_and_hash_each_line(long param_1)

{
  byte bVar1;
  ushort uVar2;
  long lVar3;
  byte *pbVar4;
  void *__s1;
  bool bVar5;
  byte *pbVar6;
  uint uVar7;
  char cVar8;
  byte *__s2;
  char cVar9;
  int iVar10;
  byte *pbVar11;
  ulong uVar12;
  ushort **ppuVar13;
  byte *pbVar14;
  long *plVar15;
  bool bVar16;
  byte local_b1;
  byte *local_a8;
  ulong local_a0;
  ulong *local_98;
  long local_90;
  long local_88;
  long local_80;
  long local_78;
  long local_70;
  ulong local_68;
  ulong local_60;
  ulong local_58;
  ulong local_50;
  long local_40;
  
  local_a8 = *(byte **)(param_1 + 0xe0);
  local_90 = *(long *)(param_1 + 0xb8);
  local_88 = *(long *)(param_1 + 0xd8);
  local_80 = 0;
  lVar3 = *(long *)(param_1 + 0xc0);
  local_78 = FUN_0011c945(local_88 << 3);
  cVar8 = DAT_0012d1ee;
  uVar7 = DAT_0012d1e8;
  local_70 = DAT_0012d768;
  local_68 = DAT_0012d770;
  local_60 = DAT_0012d778;
  pbVar4 = *(byte **)(param_1 + 0xf0);
  pbVar11 = (byte *)(*(long *)(param_1 + 0xb0) + *(long *)(param_1 + 0xa0));
  bVar16 = DAT_0012d1e8 != 0;
  bVar5 = DAT_0012d1ee != '\0';
  while (__s2 = local_a8, local_a8 < pbVar4) {
    local_58 = 0;
    if (uVar7 == 5) {
      while( true ) {
        pbVar14 = local_a8 + 1;
        bVar1 = *local_a8;
        local_a8 = pbVar14;
        if (bVar1 == 10) break;
        ppuVar13 = __ctype_b_loc();
        if (((*ppuVar13)[bVar1] & 0x2000) == 0) {
          if (cVar8 == '\0') {
            uVar12 = (ulong)bVar1;
          }
          else {
            iVar10 = tolower((uint)bVar1);
            uVar12 = (ulong)iVar10;
          }
          local_58 = uVar12 + (local_58 << 7 | local_58 >> 0x39);
        }
      }
      goto LAB_0010e811;
    }
    if (uVar7 < 6) {
      if (uVar7 < 4) {
        if (uVar7 == 0) goto LAB_0010e783;
        local_50 = 0;
        while( true ) {
          pbVar14 = local_a8 + 1;
          local_b1 = *local_a8;
          local_a8 = pbVar14;
          if (local_b1 == 10) break;
          if ((uVar7 & 2) != 0) {
            ppuVar13 = __ctype_b_loc();
            uVar2 = (*ppuVar13)[local_b1];
            pbVar6 = pbVar14;
            while ((uVar2 & 0x2000) != 0) {
              local_a8 = pbVar6 + 1;
              bVar1 = *pbVar6;
              if (bVar1 == 10) goto LAB_0010e811;
              ppuVar13 = __ctype_b_loc();
              pbVar6 = local_a8;
              uVar2 = (*ppuVar13)[bVar1];
            }
          }
          local_40 = 1;
          if ((uVar7 & 1) != 0) {
            if (local_b1 == 0xd) {
              local_50 = 0;
            }
            else if (local_b1 < 0xe) {
              if (local_b1 == 8) {
                local_50 = local_50 - (local_50 != 0);
              }
              else {
                if (local_b1 != 9) goto LAB_0010e711;
                local_b1 = 0x20;
                local_40 = DAT_0012d2a8 - local_50 % DAT_0012d2a8;
                if (local_40 + local_50 < local_50) {
                  local_50 = 0;
                }
                else {
                  local_50 = local_40 + local_50;
                }
              }
            }
            else {
LAB_0010e711:
              local_50 = local_50 + 1;
            }
          }
          if (cVar8 != '\0') {
            iVar10 = tolower((uint)local_b1);
            local_b1 = (byte)iVar10;
          }
          do {
            local_58 = (local_58 << 7 | local_58 >> 0x39) + (ulong)local_b1;
            local_40 = local_40 + -1;
            local_a8 = pbVar14;
          } while (local_40 != 0);
        }
      }
      else {
        if (uVar7 != 4) goto LAB_0010e783;
        while( true ) {
          pbVar14 = local_a8 + 1;
          local_b1 = *local_a8;
          local_a8 = pbVar14;
          if (local_b1 == 10) break;
          ppuVar13 = __ctype_b_loc();
          if (((*ppuVar13)[local_b1] & 0x2000) != 0) {
            do {
              pbVar14 = local_a8 + 1;
              local_b1 = *local_a8;
              local_a8 = pbVar14;
              if (local_b1 == 10) goto LAB_0010e811;
              ppuVar13 = __ctype_b_loc();
            } while (((*ppuVar13)[local_b1] & 0x2000) != 0);
            local_58 = (local_58 << 7 | local_58 >> 0x39) + 0x20;
          }
          if (cVar8 == '\0') {
            uVar12 = (ulong)local_b1;
          }
          else {
            iVar10 = tolower((uint)local_b1);
            uVar12 = (ulong)iVar10;
          }
          local_58 = uVar12 + (local_58 << 7 | local_58 >> 0x39);
        }
      }
    }
    else {
LAB_0010e783:
      if (cVar8 == '\0') {
        while( true ) {
          pbVar14 = local_a8 + 1;
          bVar1 = *local_a8;
          local_a8 = pbVar14;
          if (bVar1 == 10) break;
          local_58 = (local_58 << 7 | local_58 >> 0x39) + (ulong)bVar1;
        }
      }
      else {
        while( true ) {
          pbVar14 = local_a8 + 1;
          bVar1 = *local_a8;
          local_a8 = pbVar14;
          if (bVar1 == 10) break;
          iVar10 = tolower((uint)bVar1);
          local_58 = (local_58 << 7 | local_58 >> 0x39) + (long)iVar10;
        }
      }
    }
LAB_0010e811:
    local_98 = (ulong *)((local_58 % DAT_0012d760) * 8 + DAT_0012d758);
    pbVar14 = local_a8 + (-1 - (long)__s2);
    if ((((local_a8 == pbVar11) && (*(char *)(param_1 + 0x120) != '\0')) && (DAT_0012d1c0 != 4)) &&
       ((DAT_0012d1c0 != 5 && (uVar7 < 2)))) {
      local_98 = (ulong *)(DAT_0012d758 + -8);
    }
    for (local_a0 = *local_98; local_a0 != 0; local_a0 = *(ulong *)(local_70 + local_a0 * 0x20)) {
      if (local_58 == *(ulong *)(local_70 + local_a0 * 0x20 + 8)) {
        __s1 = *(void **)(local_70 + local_a0 * 0x20 + 0x10);
        if (pbVar14 == *(byte **)(local_70 + local_a0 * 0x20 + 0x18)) {
          iVar10 = memcmp(__s1,__s2,(size_t)pbVar14);
          if (iVar10 == 0) goto LAB_0010ea84;
          if (bVar16 || bVar5) {
LAB_0010ea3f:
            cVar9 = FUN_00111f90(__s1,__s2);
            if (cVar9 != '\x01') goto LAB_0010ea84;
          }
        }
        else if (bVar16) goto LAB_0010ea3f;
      }
    }
    local_a0 = local_68;
    if (local_68 == local_60) {
      if (0x1fffffffffffffe < local_60) {
        FUN_0011cf09();
      }
      local_70 = FUN_0011c991(local_70,local_60 << 6);
      local_60 = local_60 << 1;
    }
    *(ulong *)(local_68 * 0x20 + local_70) = *local_98;
    *(ulong *)(local_68 * 0x20 + local_70 + 8) = local_58;
    *(byte **)(local_68 * 0x20 + local_70 + 0x10) = __s2;
    *(byte **)(local_68 * 0x20 + local_70 + 0x18) = pbVar14;
    *local_98 = local_68;
    local_68 = local_68 + 1;
LAB_0010ea84:
    if (local_80 == local_88) {
      if (((0x2aaaaaaaaaaaaaa9 < local_88) || (0xffffffffffffffe < (ulong)(local_88 * 2 - lVar3)))
         || (0xffffffffffffffe < (ulong)(local_88 - lVar3))) {
        FUN_0011cf09();
      }
      local_88 = local_88 * 2 - lVar3;
      local_78 = FUN_0011c991(local_78,local_88 * 8);
      local_90 = FUN_0011c991(local_90 + lVar3 * 8,(local_88 - lVar3) * 8);
      local_90 = local_90 + lVar3 * -8;
    }
    *(byte **)(local_80 * 8 + local_90) = __s2;
    *(ulong *)(local_80 * 8 + local_78) = local_a0;
    local_80 = local_80 + 1;
  }
  *(long *)(param_1 + 200) = local_80;
  local_a0 = 0;
  while( true ) {
    if (local_80 == local_88) {
      if (((0x2aaaaaaaaaaaaaa9 < local_88) || (0xffffffffffffffe < (ulong)(local_88 * 2 - lVar3)))
         || (0xffffffffffffffe < (ulong)(local_88 - lVar3))) {
        FUN_0011cf09();
      }
      local_88 = local_88 * 2 - lVar3;
      local_90 = FUN_0011c991(local_90 + lVar3 * 8,(local_88 - lVar3) * 8);
      local_90 = local_90 + lVar3 * -8;
    }
    *(byte **)(local_80 * 8 + local_90) = local_a8;
    if (local_a8 == pbVar11) break;
    if ((DAT_0012d1d0 <= (long)local_a0) && (DAT_0012d1c8 != '\0')) goto LAB_0010ed3e;
    local_80 = local_80 + 1;
    do {
      pbVar4 = local_a8 + 1;
      bVar1 = *local_a8;
      local_a8 = pbVar4;
    } while (bVar1 != 10);
    local_a0 = local_a0 + 1;
  }
  if (((*(char *)(param_1 + 0x120) != '\0') && (DAT_0012d1c0 != 4)) && (DAT_0012d1c0 != 5)) {
    plVar15 = (long *)(local_90 + local_80 * 8);
    *plVar15 = *plVar15 + -1;
  }
LAB_0010ed3e:
  *(long *)(param_1 + 0xb8) = local_90;
  *(long *)(param_1 + 0xd0) = local_80;
  *(long *)(param_1 + 0xd8) = local_88;
  *(long *)(param_1 + 0xf8) = local_78;
  DAT_0012d768 = local_70;
  DAT_0012d778 = local_60;
  DAT_0012d770 = local_68;
  return;
}




// Function: prepare_text @ 0xedad

void prepare_text(long param_1)

{
  void *__s;
  int iVar1;
  char *pcVar2;
  long local_30;
  char *local_28;
  char *local_20;
  
  local_30 = *(long *)(param_1 + 0xb0);
  __s = *(void **)(param_1 + 0xa0);
  if (__s != (void *)0x0) {
    if (DAT_0012d2b2 != '\0') {
      pcVar2 = (char *)(local_30 + (long)__s);
      *pcVar2 = '\r';
      local_20 = rawmemchr(__s,0xd);
      local_28 = local_20;
      for (; local_20 != pcVar2; local_20 = local_20 + iVar1 + 1) {
        if ((*local_20 == '\r') && (local_20[1] == '\n')) {
          iVar1 = 1;
        }
        else {
          iVar1 = 0;
        }
        *local_28 = local_20[iVar1];
        local_28 = local_28 + 1;
      }
      local_30 = local_30 - ((long)pcVar2 - (long)local_28);
    }
    if ((local_30 != 0) && (*(char *)((long)__s + local_30 + -1) != '\n')) {
      *(undefined1 *)(local_30 + (long)__s) = 10;
      *(undefined1 *)(param_1 + 0x120) = 1;
      local_30 = local_30 + 1;
    }
    memset((void *)(local_30 + (long)__s),0,8);
    *(long *)(param_1 + 0xb0) = local_30;
  }
  return;
}




// Function: guess_lines @ 0xeef5

long guess_lines(long param_1,ulong param_2,ulong param_3)

{
  if (param_1 < 10) {
    param_2 = 0x20;
  }
  else {
    param_2 = param_2 / (param_1 - 1U);
  }
  param_3 = param_3 / param_2;
  if (param_3 == 0) {
    param_3 = 1;
  }
  if (0x787878787878782 < param_3) {
    param_3 = 0x787878787878782;
  }
  return param_3 + 5;
}




// Function: find_identical_ends @ 0xef6a

void find_identical_ends(int *param_1)

{
  long *plVar1;
  long *plVar2;
  long *plVar3;
  long lVar4;
  long *plVar5;
  int iVar6;
  long *plVar7;
  long lVar8;
  ulong uVar9;
  ulong uVar10;
  bool bVar11;
  long *local_b8;
  long *local_b0;
  long *local_a8;
  long *local_a0;
  long *local_98;
  long local_90;
  long local_88;
  ulong local_80;
  ulong local_78;
  long local_70;
  
  FUN_0010e09d(param_1);
  FUN_0010edad(param_1);
  if (*param_1 == param_1[0x4c]) {
    *(undefined8 *)(param_1 + 0x74) = *(undefined8 *)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(undefined8 *)(param_1 + 0x78) = *(undefined8 *)(param_1 + 0x2c);
    *(char *)(param_1 + 0x94) = (char)param_1[0x48];
  }
  else {
    FUN_0010e09d(param_1 + 0x4c);
    FUN_0010edad(param_1 + 0x4c);
  }
  plVar2 = *(long **)(param_1 + 0x28);
  plVar3 = *(long **)(param_1 + 0x74);
  uVar9 = *(ulong *)(param_1 + 0x2c);
  uVar10 = *(ulong *)(param_1 + 0x78);
  if (plVar2 == plVar3) {
    local_a8 = (long *)((long)plVar3 + uVar10);
    local_a0 = local_a8;
  }
  else {
    local_b8 = plVar2;
    local_b0 = plVar3;
    if (uVar9 < uVar10) {
      *(byte *)(uVar9 + (long)plVar2) = ~*(byte *)(uVar9 + (long)plVar3);
    }
    else {
      *(byte *)(uVar10 + (long)plVar3) = ~*(byte *)(uVar10 + (long)plVar2);
    }
    for (; *local_b8 == *local_b0; local_b8 = local_b8 + 1) {
      local_b0 = local_b0 + 1;
    }
    local_a0 = local_b0;
    for (local_a8 = local_b8; (char)*local_a8 == (char)*local_a0;
        local_a8 = (long *)((long)local_a8 + 1)) {
      local_a0 = (long *)((long)local_a0 + 1);
    }
    if (((DAT_0012d1c0 != 4) && (DAT_0012d1c0 != 5)) &&
       ((long *)((long)plVar3 + (uVar10 - *(byte *)(param_1 + 0x94))) < local_a0 !=
        (long *)((long)plVar2 + (uVar9 - *(byte *)(param_1 + 0x48))) < local_a8)) {
      local_a8 = (long *)((long)local_a8 + -1);
      local_a0 = (long *)((long)local_a0 + -1);
    }
  }
  local_88 = DAT_0012d1e0;
  while ((local_a8 != plVar2 &&
         ((*(char *)((long)local_a8 + -1) != '\n' ||
          (bVar11 = local_88 != 0, local_88 = local_88 + -1, bVar11))))) {
    local_a8 = (long *)((long)local_a8 + -1);
    local_a0 = (long *)((long)local_a0 + -1);
  }
  *(long **)(param_1 + 0x38) = local_a8;
  *(long **)(param_1 + 0x84) = local_a0;
  plVar7 = (long *)(uVar9 + (long)plVar2);
  local_a0 = (long *)(uVar10 + (long)plVar3);
  if (((DAT_0012d1c0 == 4) || (DAT_0012d1c0 == 5)) ||
     (local_a8 = plVar7, (char)param_1[0x48] == (char)param_1[0x94])) {
    if (uVar9 < uVar10) {
      lVar8 = 0;
    }
    else {
      lVar8 = uVar9 - uVar10;
    }
    local_98 = (long *)(lVar8 + *(long *)(param_1 + 0x38));
    plVar1 = plVar7;
    plVar5 = local_a0;
    do {
      local_a0 = plVar5;
      local_a8 = plVar1;
      if (local_a8 == local_98) goto LAB_0010f3ed;
      plVar1 = (long *)((long)local_a8 + -1);
      plVar5 = (long *)((long)local_a0 + -1);
    } while (*(char *)((long)local_a8 + -1) == *(char *)((long)local_a0 + -1));
    local_98 = local_a8;
LAB_0010f3ed:
    if (((plVar2 == local_a8) || (*(char *)((long)local_a8 + -1) == '\n')) &&
       ((plVar3 == local_a0 || (*(char *)((long)local_a0 + -1) == '\n')))) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    local_88 = DAT_0012d1e0 + iVar6;
    while ((lVar8 = local_88 + -1, local_88 != 0 && (local_a8 != plVar7))) {
      do {
        plVar1 = (long *)((long)local_a8 + 1);
        lVar4 = *local_a8;
        local_a8 = plVar1;
        local_88 = lVar8;
      } while ((char)lVar4 != '\n');
    }
    local_a0 = (long *)((long)local_a0 + ((long)local_a8 - (long)local_98));
  }
  *(long **)(param_1 + 0x3c) = local_a8;
  *(long **)(param_1 + 0x88) = local_a0;
  if ((((DAT_0012d1c8 == '\0') || (DAT_0012d240 != 0)) || (0x1ffffffffffffffe < (long)DAT_0012d1d0))
     || (uVar9 <= DAT_0012d1d0)) {
    local_70 = 0;
    local_78 = FUN_0010eef5(0,0,uVar9);
  }
  else {
    lVar8 = FUN_0010eef5(0,0,(long)local_a8 - *(long *)(param_1 + 0x38));
    local_78 = FUN_0010eef5(0,0,(char *)((long)plVar2 + (uVar9 - (long)local_a8)));
    for (local_70 = 1; local_70 <= (long)DAT_0012d1d0; local_70 = local_70 << 1) {
    }
    if ((long)DAT_0012d1d0 <= (long)local_78) {
      local_78 = DAT_0012d1d0;
    }
    local_78 = local_78 + local_70 + lVar8;
  }
  local_80 = 0;
  local_90 = FUN_0011c945(local_78 << 3);
  if (((DAT_0012d1c8 == '\x01') && (local_a8 == *(long **)(param_1 + 0x38))) &&
     (local_a0 == *(long **)(param_1 + 0x84))) {
    bVar11 = false;
  }
  else {
    bVar11 = true;
  }
  local_a8 = plVar2;
  if (bVar11) {
    plVar7 = *(long **)(param_1 + 0x38);
    while (local_a8 != plVar7) {
      uVar9 = local_80 + 1;
      local_80 = local_80 & local_70 - 1U;
      if (local_80 == local_78) {
        if (0x7fffffffffffffe < local_78) {
          FUN_0011cf09();
        }
        local_90 = FUN_0011c991(local_90,local_78 << 4);
        local_78 = local_78 * 2;
      }
      *(long **)(local_80 * 8 + local_90) = local_a8;
      do {
        plVar1 = (long *)((long)local_a8 + 1);
        lVar8 = *local_a8;
        local_a8 = plVar1;
        local_80 = uVar9;
      } while ((char)lVar8 != '\n');
    }
  }
  uVar9 = local_80;
  if ((local_70 != 0) && ((long)DAT_0012d1d0 < (long)local_80)) {
    uVar9 = DAT_0012d1d0;
  }
  lVar8 = FUN_0010eef5(local_80,(long)local_a8 - (long)plVar2,
                       (long)local_a0 - *(long *)(param_1 + 0x84));
  uVar10 = FUN_0010eef5(local_80,(long)local_a8 - (long)plVar2,
                        (char *)((long)plVar3 + (uVar10 - (long)local_a0)));
  if ((long)DAT_0012d1d0 <= (long)uVar10) {
    uVar10 = DAT_0012d1d0;
  }
  uVar10 = uVar10 + uVar9 + lVar8;
  if (((long)uVar10 < (long)uVar9) || (0xffffffffffffffe < uVar10)) {
    FUN_0011cf09();
  }
  lVar8 = FUN_0011c945(uVar10 * 8);
  if (uVar9 != local_80) {
    for (local_88 = 0; local_88 < (long)uVar9; local_88 = local_88 + 1) {
      *(undefined8 *)(lVar8 + local_88 * 8) =
           *(undefined8 *)(local_90 + (local_88 + (local_80 - DAT_0012d1d0) & local_70 - 1U) * 8);
    }
    for (local_88 = 0; local_88 < (long)uVar9; local_88 = local_88 + 1) {
      *(undefined8 *)(local_90 + local_88 * 8) = *(undefined8 *)(lVar8 + local_88 * 8);
    }
  }
  for (local_88 = 0; local_88 < (long)uVar9; local_88 = local_88 + 1) {
    *(char **)(lVar8 + local_88 * 8) =
         (char *)((long)plVar3 + (*(long *)(local_90 + local_88 * 8) - (long)plVar2));
  }
  *(ulong *)(param_1 + 0x2e) = uVar9 * 8 + local_90;
  *(ulong *)(param_1 + 0x7a) = lVar8 + uVar9 * 8;
  *(ulong *)(param_1 + 0x7c) = -uVar9;
  *(undefined8 *)(param_1 + 0x30) = *(undefined8 *)(param_1 + 0x7c);
  *(ulong *)(param_1 + 0x36) = local_78 - uVar9;
  *(ulong *)(param_1 + 0x82) = uVar10 - uVar9;
  *(ulong *)(param_1 + 0x86) = local_80;
  *(undefined8 *)(param_1 + 0x3a) = *(undefined8 *)(param_1 + 0x86);
  return;
}




// Function: read_files @ 0xfa13

undefined8 read_files(int *param_1,char param_2)

{
  bool bVar1;
  char cVar2;
  undefined8 uVar3;
  long lVar4;
  bool bVar5;
  int local_c;
  
  bVar1 = DAT_0012d1d8 != '\0';
  cVar2 = FUN_0010df00(param_1,bVar1 || param_2 != '\0');
  bVar5 = cVar2 != '\0' || param_2 != '\0';
  if (*param_1 == param_1[0x4c]) {
    *(undefined8 *)(param_1 + 0x74) = *(undefined8 *)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(undefined8 *)(param_1 + 0x78) = *(undefined8 *)(param_1 + 0x2c);
  }
  else {
    cVar2 = FUN_0010df00(param_1 + 0x4c,param_2 != '\0' || (bVar1 || cVar2 != '\0'));
    bVar5 = cVar2 != '\0' || bVar5;
  }
  if (bVar5) {
    FUN_001131e6(*param_1,0);
    FUN_001131e6(param_1[0x4c],0);
    uVar3 = 1;
  }
  else {
    FUN_0010ef6a(param_1);
    DAT_0012d778 = *(long *)(param_1 + 0x82) + *(long *)(param_1 + 0x36) + 1;
    if (0x3fffffffffffffe < DAT_0012d778) {
      FUN_0011cf09();
    }
    DAT_0012d768 = (void *)FUN_0011c945(DAT_0012d778 << 5);
    DAT_0012d770 = 1;
    local_c = 9;
    while( true ) {
      if ((ulong)((long)DAT_0012d778 / 3) <= (ulong)(1L << ((byte)local_c & 0x3f))) break;
      local_c = local_c + 1;
    }
    DAT_0012d760 = (1L << ((byte)local_c & 0x3f)) - (ulong)(byte)(&DAT_00125240)[local_c];
    if (0xffffffffffffffe < DAT_0012d760) {
      FUN_0011cf09();
    }
    lVar4 = FUN_00113030((DAT_0012d760 + 1) * 8);
    DAT_0012d758 = lVar4 + 8;
    for (local_c = 0; local_c < 2; local_c = local_c + 1) {
      FUN_0010e2f8(param_1 + (long)local_c * 0x4c);
    }
    *(undefined8 *)(param_1 + 0x96) = DAT_0012d770;
    *(undefined8 *)(param_1 + 0x4a) = *(undefined8 *)(param_1 + 0x96);
    free(DAT_0012d768);
    free((void *)(DAT_0012d758 + -8));
    uVar3 = 0;
  }
  return uVar3;
}




// Function: print_normal_script @ 0xfcfd

void print_normal_script(undefined8 param_1)

{
  FUN_0011245b(param_1,FUN_00112437,FUN_0010fd2d);
  return;
}




// Function: print_normal_hunk @ 0xfd2d

void print_normal_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00112af4(param_1,&local_40,&local_38,&local_30,&local_28);
  if (uVar1 != 0) {
    FUN_00111b50();
    FUN_001128c2(4);
    FUN_00112a37(0x2c,&DAT_0012d360,local_40,local_38);
    fputc_unlocked((int)(char)(&DAT_00125780)[uVar1],DAT_0012d5c0);
    FUN_00112a37(0x2c,&DAT_0012d490,local_30,local_28);
    FUN_001128c2(3);
    fputc_unlocked(10,DAT_0012d5c0);
    lVar2 = local_40;
    if ((uVar1 & 1) != 0) {
      for (; lVar2 <= local_38; lVar2 = lVar2 + 1) {
        FUN_001128c2(2);
        FUN_001124fe(&DAT_00125280,DAT_0012d418 + lVar2 * 8,1);
        FUN_001128c2(3);
        if (*(char *)(*(long *)((lVar2 + 1) * 8 + DAT_0012d418) + -1) == '\n') {
          putc_unlocked(10,DAT_0012d5c0);
        }
      }
    }
    if (uVar1 == 3) {
      fputs_unlocked("---\n",DAT_0012d5c0);
    }
    lVar2 = local_30;
    if ((uVar1 & 2) != 0) {
      for (; lVar2 <= local_28; lVar2 = lVar2 + 1) {
        FUN_001128c2(1);
        FUN_001124fe(&DAT_00125287,DAT_0012d548 + lVar2 * 8,1);
        FUN_001128c2(3);
        if (*(char *)(*(long *)((lVar2 + 1) * 8 + DAT_0012d548) + -1) == '\n') {
          putc_unlocked(10,DAT_0012d5c0);
        }
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_sdiff_script @ 0xff70

void print_sdiff_script(undefined8 param_1)

{
  FUN_00111b50();
  DAT_0012d780 = -DAT_0012d448;
  DAT_0012d788 = DAT_0012d780;
  FUN_0011245b(param_1,FUN_00112437,FUN_001106e7);
  FUN_0011055a(DAT_0012d430,DAT_0012d560);
  return;
}




// Function: tab_from_to @ 0xffdd

ulong tab_from_to(ulong param_1,ulong param_2)

{
  ulong uVar1;
  FILE *__stream;
  undefined8 local_30;
  undefined8 local_20;
  
  __stream = DAT_0012d5c0;
  uVar1 = DAT_0012d2a8;
  local_30 = param_1;
  if (DAT_0012d2a1 != '\x01') {
    local_30 = param_1;
    for (local_20 = (param_1 + DAT_0012d2a8) - param_1 % DAT_0012d2a8; local_20 <= param_2;
        local_20 = local_20 + uVar1) {
      putc_unlocked(9,__stream);
      local_30 = local_20;
    }
  }
  while (local_30 < param_2) {
    putc_unlocked(0x20,__stream);
    local_30 = local_30 + 1;
  }
  return param_2;
}




// Function: print_half_line @ 0x10090

ulong print_half_line(undefined8 *param_1,undefined8 param_2,ulong param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  bool bVar4;
  int iVar5;
  ulong uVar6;
  ulong uVar7;
  ulong uVar8;
  char *pcVar9;
  long in_FS_OFFSET;
  wchar_t local_78;
  int local_74;
  ulong local_70;
  FILE *local_68;
  char *local_60;
  long local_58;
  size_t local_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_68 = DAT_0012d5c0;
  pcVar3 = (char *)param_1[1];
  local_48 = 0;
  uVar7 = 0;
  uVar8 = 0;
  pcVar1 = (char *)*param_1;
  while (pcVar9 = pcVar1, pcVar9 < pcVar3) {
    pcVar1 = pcVar9 + 1;
    cVar2 = *pcVar9;
    iVar5 = (int)cVar2;
    local_60 = pcVar9;
    if (iVar5 < 0x60) {
      if (iVar5 < 8) goto switchD_00110142_caseD_e;
      switch(iVar5) {
      case 8:
        if ((uVar7 != 0) && (uVar7 = uVar7 - 1, uVar7 < param_3)) {
          if (uVar7 < uVar8) {
            putc_unlocked((int)cVar2,local_68);
            uVar8 = uVar7;
          }
          else {
            for (; uVar8 < uVar7; uVar8 = uVar8 + 1) {
              putc_unlocked(0x20,local_68);
            }
          }
        }
        break;
      case 9:
        local_58 = DAT_0012d2a8 - uVar7 % DAT_0012d2a8;
        if (uVar7 == uVar8) {
          uVar6 = local_58 + uVar8;
          local_70 = uVar6;
          if (DAT_0012d2a1 == '\0') {
            if (uVar6 < param_3) {
              putc_unlocked((int)cVar2,local_68);
              uVar8 = uVar6;
            }
          }
          else {
            if (param_3 < uVar6) {
              local_70 = param_3;
            }
            for (; uVar8 < local_70; uVar8 = uVar8 + 1) {
              putc_unlocked(0x20,local_68);
            }
          }
        }
        uVar7 = local_58 + uVar7;
        break;
      case 10:
        goto LAB_00110365;
      case 0xb:
      case 0xc:
        goto switchD_00110142_caseD_b;
      case 0xd:
        putc_unlocked((int)cVar2,local_68);
        FUN_0010ffdd(0,param_2);
        uVar7 = 0;
        uVar8 = 0;
        break;
      default:
        goto switchD_00110142_caseD_e;
      case 0x20:
      case 0x21:
      case 0x22:
      case 0x23:
      case 0x25:
      case 0x26:
      case 0x27:
      case 0x28:
      case 0x29:
      case 0x2a:
      case 0x2b:
      case 0x2c:
      case 0x2d:
      case 0x2e:
      case 0x2f:
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
      case 0x3a:
      case 0x3b:
      case 0x3c:
      case 0x3d:
      case 0x3e:
      case 0x3f:
      case 0x41:
      case 0x42:
      case 0x43:
      case 0x44:
      case 0x45:
      case 0x46:
      case 0x47:
      case 0x48:
      case 0x49:
      case 0x4a:
      case 0x4b:
      case 0x4c:
      case 0x4d:
      case 0x4e:
      case 0x4f:
      case 0x50:
      case 0x51:
      case 0x52:
      case 0x53:
      case 0x54:
      case 0x55:
      case 0x56:
      case 0x57:
      case 0x58:
      case 0x59:
      case 0x5a:
      case 0x5b:
      case 0x5c:
      case 0x5d:
      case 0x5e:
      case 0x5f:
        goto switchD_00110142_caseD_20;
      }
    }
    else if (iVar5 - 0x61U < 0x1e) {
switchD_00110142_caseD_20:
      uVar6 = uVar7 + 1;
      bVar4 = uVar7 < param_3;
      uVar7 = uVar6;
      if (bVar4) {
        putc_unlocked((int)cVar2,local_68);
        uVar8 = uVar6;
      }
    }
    else {
switchD_00110142_caseD_e:
      local_50 = FUN_0011d369(&local_78,pcVar9,(long)pcVar3 - (long)pcVar9,&local_48);
      if ((local_50 == 0) || (0xfffffffffffffffd < local_50)) {
switchD_00110142_caseD_b:
        if (uVar7 < param_3) {
          putc_unlocked((int)cVar2,local_68);
        }
      }
      else {
        local_74 = wcwidth(local_78);
        if (0 < local_74) {
          uVar7 = uVar7 + (long)local_74;
        }
        if (uVar7 <= param_3) {
          fwrite_unlocked(local_60,1,local_50,stdout);
          uVar8 = uVar7;
        }
        pcVar1 = local_60 + local_50;
      }
    }
  }
LAB_00110365:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar8;
}




// Function: print_1sdiff_line @ 0x10388

void print_1sdiff_line(long param_1,char param_2,undefined8 *param_3)

{
  bool bVar1;
  long lVar2;
  long lVar3;
  FILE *__stream;
  undefined8 uVar4;
  char local_44;
  bool local_2a;
  long local_28;
  
  __stream = DAT_0012d5c0;
  lVar3 = DAT_0012d328;
  lVar2 = DAT_0012d320;
  local_28 = 0;
  local_2a = false;
  bVar1 = false;
  if (param_2 == '<') {
    FUN_001128c2(2);
    bVar1 = true;
  }
  else if (param_2 == '>') {
    FUN_001128c2(1);
    bVar1 = true;
  }
  if (param_1 != 0) {
    local_2a = *(char *)(*(long *)(param_1 + 8) + -1) == '\n';
    local_28 = FUN_00110090(param_1,0,lVar2);
  }
  if (param_2 != ' ') {
    local_28 = FUN_0010ffdd(local_28,(lVar3 + lVar2) - 1U >> 1);
    local_28 = local_28 + 1;
    local_44 = param_2;
    if ((param_2 == '|') && (local_2a != (*(char *)(param_3[1] + -1) == '\n'))) {
      if (local_2a == false) {
        local_44 = '\\';
      }
      else {
        local_44 = '/';
      }
    }
    putc_unlocked((int)local_44,__stream);
  }
  if ((param_3 != (undefined8 *)0x0) &&
     (local_2a = *(char *)(param_3[1] + -1) == '\n' || local_2a != false, *(char *)*param_3 != '\n')
     ) {
    uVar4 = FUN_0010ffdd(local_28,lVar3);
    FUN_00110090(param_3,uVar4,lVar2);
  }
  if (local_2a != false) {
    putc_unlocked(10,__stream);
  }
  if (bVar1) {
    FUN_001128c2(3);
  }
  return;
}




// Function: print_sdiff_common_lines @ 0x1055a

void print_sdiff_common_lines(long param_1,long param_2)

{
  long local_28;
  long local_20;
  
  local_28 = DAT_0012d780;
  local_20 = DAT_0012d788;
  if ((DAT_0012d31a != '\x01') && ((DAT_0012d780 != param_1 || (DAT_0012d788 != param_2)))) {
    if (DAT_0012d318 != '\0') {
      fprintf(DAT_0012d5c0,"i%ld,%ld\n",param_1 - DAT_0012d780,param_2 - DAT_0012d788);
    }
    if (DAT_0012d319 != '\x01') {
      while ((local_28 != param_1 && (local_20 != param_2))) {
        FUN_00110388(local_28 * 8 + DAT_0012d418,0x20,DAT_0012d548 + local_20 * 8);
        local_28 = local_28 + 1;
        local_20 = local_20 + 1;
      }
      while (local_20 != param_2) {
        FUN_00110388(0,0x29,local_20 * 8 + DAT_0012d548);
        local_20 = local_20 + 1;
      }
    }
    while (local_28 != param_1) {
      FUN_00110388(local_28 * 8 + DAT_0012d418,0x28,0);
      local_28 = local_28 + 1;
    }
  }
  DAT_0012d780 = param_1;
  DAT_0012d788 = param_2;
  return;
}




// Function: print_sdiff_hunk @ 0x106e7

void print_sdiff_hunk(undefined8 param_1)

{
  int iVar1;
  long lVar2;
  long lVar3;
  long in_FS_OFFSET;
  uint local_54;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_54 = FUN_00112af4(param_1,&local_50,&local_48,&local_40,&local_38);
  lVar2 = DAT_0012d780;
  if (local_54 != 0) {
    FUN_0011055a(local_50,local_40);
    if (DAT_0012d318 != '\0') {
      local_30 = (local_48 - local_50) + 1;
      local_28 = (local_38 - local_40) + 1;
      fprintf(DAT_0012d5c0,"c%ld,%ld\n",local_30,local_28);
    }
    lVar2 = local_50;
    lVar3 = local_40;
    if (local_54 == 3) {
      for (; (lVar2 <= local_48 && (lVar3 <= local_38)); lVar3 = lVar3 + 1) {
        FUN_00110388(DAT_0012d418 + lVar2 * 8,0x7c,lVar3 * 8 + DAT_0012d548);
        lVar2 = lVar2 + 1;
      }
      if (local_38 < lVar3) {
        iVar1 = 0;
      }
      else {
        iVar1 = 2;
      }
      local_54 = iVar1 + (uint)(lVar2 <= local_48);
      DAT_0012d780 = lVar2;
      DAT_0012d788 = lVar3;
      local_50 = lVar2;
      local_40 = lVar3;
    }
    lVar2 = local_40;
    lVar3 = DAT_0012d788;
    if ((local_54 & 2) != 0) {
      for (; lVar3 = lVar2, lVar2 <= local_38; lVar2 = lVar2 + 1) {
        FUN_00110388(0,0x3e,DAT_0012d548 + lVar2 * 8);
      }
    }
    DAT_0012d788 = lVar3;
    lVar3 = local_50;
    lVar2 = DAT_0012d780;
    if ((local_54 & 1) != 0) {
      for (; lVar2 = lVar3, lVar3 <= local_48; lVar3 = lVar3 + 1) {
        FUN_00110388(DAT_0012d418 + lVar3 * 8,0x3c,0);
      }
    }
  }
  DAT_0012d780 = lVar2;
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: perror_with_name @ 0x108ec

void perror_with_name(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar1,&DAT_0012540c,param_1);
}




// Function: pfatal_with_name @ 0x10925

void pfatal_with_name(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_00110bca();
                    /* WARNING: Subroutine does not return */
  error(2,iVar1,&DAT_0012540c,param_1);
}




// Function: fatal @ 0x10966

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  FUN_00110bca();
  uVar1 = gettext(param_1);
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_0012540c,uVar1);
}




// Function: message @ 0x109a8

void message(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001109e2(param_1,param_2,param_3,0,0);
  return;
}




// Function: message5 @ 0x109e2

void message5(long param_1,undefined8 param_2,undefined8 param_3,undefined *param_4,
                 undefined *param_5)

{
  size_t sVar1;
  undefined8 *puVar2;
  char *__format;
  long in_FS_OFFSET;
  int local_84;
  undefined8 *local_80;
  long local_78;
  long local_68 [3];
  undefined *local_50;
  undefined *local_48;
  size_t asStack_38 [5];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0012d2c0 == '\0') {
    if (DAT_0012d318 != '\0') {
      putchar_unlocked(0x20);
    }
    __format = (char *)gettext(param_1);
    printf(__format,param_2,param_3,param_4,param_5);
  }
  else {
    local_78 = 8;
    local_68[0] = param_1;
    local_68[1] = param_2;
    local_68[2] = param_3;
    if (param_4 == (undefined *)0x0) {
      param_4 = &DAT_0012540f;
    }
    local_50 = param_4;
    if (param_5 == (undefined *)0x0) {
      param_5 = &DAT_0012540f;
    }
    local_48 = param_5;
    for (local_84 = 0; local_84 < 5; local_84 = local_84 + 1) {
      sVar1 = strlen((char *)local_68[local_84]);
      asStack_38[local_84] = sVar1 + 1;
      local_78 = local_78 + asStack_38[local_84];
    }
    puVar2 = (undefined8 *)FUN_0011c945(local_78);
    local_80 = puVar2 + 1;
    for (local_84 = 0; local_84 < 5; local_84 = local_84 + 1) {
      memcpy(local_80,(void *)local_68[local_84],asStack_38[local_84]);
      local_80 = (undefined8 *)((long)local_80 + asStack_38[local_84]);
    }
    *(undefined8 **)PTR_DAT_0012d020 = puVar2;
    *puVar2 = 0;
    PTR_DAT_0012d020 = (undefined *)puVar2;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_message_queue @ 0x10bca

void print_message_queue(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  size_t sVar6;
  char *__format;
  long in_FS_OFFSET;
  int local_6c;
  undefined8 *local_68;
  undefined8 *local_58 [4];
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_68 = DAT_0012d7a0;
  while (local_68 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*local_68;
    local_58[0] = local_68 + 1;
    for (local_6c = 0; uVar5 = local_38, puVar4 = local_58[3], puVar3 = local_58[2],
        puVar2 = local_58[1], local_6c < 4; local_6c = local_6c + 1) {
      puVar2 = local_58[local_6c];
      sVar6 = strlen((char *)local_58[local_6c]);
      local_58[local_6c + 1] = (undefined8 *)(sVar6 + 1 + (long)puVar2);
    }
    __format = (char *)gettext(local_58[0]);
    printf(__format,puVar2,puVar3,puVar4,uVar5);
    free(local_68);
    local_68 = puVar1;
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sighandler @ 0x10ccc

void sighandler(int param_1)

{
  if (DAT_0012d840 == 0) {
    DAT_0012d840 = param_1;
  }
  return;
}




// Function: stophandler @ 0x10ced

void stophandler(void)

{
  if (DAT_0012d840 == 0) {
    DAT_0012d844 = DAT_0012d844 + 1;
  }
  return;
}




// Function: process_signals @ 0x10d14

void process_signals(void)

{
  long in_FS_OFFSET;
  int local_a0;
  sigset_t local_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_0012d840 != 0 || (DAT_0012d844 != 0))) {
    FUN_001128c2(3);
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_0012d7c0,&local_98);
    local_a0 = DAT_0012d840;
    if (DAT_0012d844 == 0) {
      signal(DAT_0012d840,(__sighandler_t)0x0);
    }
    else {
      DAT_0012d844 = DAT_0012d844 + -1;
      local_a0 = 0x13;
    }
    raise(local_a0);
    sigprocmask(2,&local_98,(sigset_t *)0x0);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: install_signal_handlers @ 0x10e15

void install_signal_handlers(void)

{
  int iVar1;
  long in_FS_OFFSET;
  int local_ac;
  _union_1457 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined4 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sigemptyset((sigset_t *)&DAT_0012d7c0);
  for (local_ac = 0; local_ac < 0xc; local_ac = local_ac + 1) {
    sigaction(*(int *)(&DAT_001257c0 + (long)local_ac * 4),(sigaction *)0x0,(sigaction *)&local_a8);
    if (local_a8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_0012d7c0,*(int *)(&DAT_001257c0 + (long)local_ac * 4));
    }
  }
  local_a0 = DAT_0012d7c0;
  local_98 = DAT_0012d7c8;
  local_90 = DAT_0012d7d0;
  local_88 = DAT_0012d7d8;
  local_80 = DAT_0012d7e0;
  local_78 = DAT_0012d7e8;
  local_70 = DAT_0012d7f0;
  local_68 = DAT_0012d7f8;
  local_60 = DAT_0012d800;
  local_58 = DAT_0012d808;
  local_50 = DAT_0012d810;
  local_48 = DAT_0012d818;
  local_40 = DAT_0012d820;
  local_38 = DAT_0012d828;
  local_30 = DAT_0012d830;
  local_28 = DAT_0012d838;
  local_20 = 0x10000000;
  for (local_ac = 0; local_ac < 0xc; local_ac = local_ac + 1) {
    iVar1 = sigismember((sigset_t *)&DAT_0012d7c0,*(int *)(&DAT_001257c0 + (long)local_ac * 4));
    if (iVar1 != 0) {
      if (*(int *)(&DAT_001257c0 + (long)local_ac * 4) == 0x14) {
        local_a8.sa_handler = FUN_00110ced;
      }
      else {
        local_a8.sa_handler = FUN_00110ccc;
      }
      sigaction(*(int *)(&DAT_001257c0 + (long)local_ac * 4),(sigaction *)&local_a8,(sigaction *)0x0
               );
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: get_funky_string @ 0x11052

/* WARNING: Removing unreachable block (ram,0x001113d6) */
/* WARNING: Removing unreachable block (ram,0x001113e5) */

undefined8 get_funky_string(void)

{
  undefined8 uVar1;
  
                    /* WARNING: Could not recover jumptable at 0x001110c8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  uVar1 = (*(code *)((long)&switchD_001110c8::switchdataD_00125410 +
                    (long)(int)switchD_001110c8::switchdataD_00125410))();
  return uVar1;
}




// Function: set_color_palette @ 0x1140f

void set_color_palette(undefined8 param_1)

{
  DAT_0012d868 = param_1;
  return;
}




// Function: parse_diff_color @ 0x11429

/* WARNING: Removing unreachable block (ram,0x00111751) */

void parse_diff_color(void)

{
  char *pcVar1;
  long in_FS_OFFSET;
  char local_13 [3];
  long local_10;
  
  pcVar1 = DAT_0012d868;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0012d868 != (char *)0x0) && (*DAT_0012d868 != '\0')) {
    strcpy(local_13,"??");
    FUN_0011ced8(pcVar1);
                    /* WARNING: Could not recover jumptable at 0x001114d3. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)((long)&switchD_001114d3::switchdataD_001255b0 + (long)(int)UINT_001255b4))();
    return;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_color_output @ 0x117e9

void check_color_output(char param_1)

{
  bool bVar1;
  int iVar2;
  
  if (DAT_0012d5c0 == (FILE *)0x0) {
    return;
  }
  if (DAT_0012d1c4 == 0) {
    return;
  }
  if (DAT_0012d5c8 == '\0') {
    if (param_1 != '\x01') {
      iVar2 = fileno(DAT_0012d5c0);
      iVar2 = isatty(iVar2);
      if (iVar2 != 0) goto LAB_00111848;
    }
    bVar1 = false;
  }
  else {
LAB_00111848:
    bVar1 = true;
  }
  if ((DAT_0012d1c4 == 2) || ((DAT_0012d1c4 == 1 && (bVar1)))) {
    DAT_0012d859 = '\x01';
  }
  else {
    DAT_0012d859 = '\0';
  }
  if (DAT_0012d859 != '\0') {
    FUN_00111429();
  }
  if (bVar1) {
    FUN_00110e15();
  }
  return;
}




// Function: setup_output @ 0x118ac

void setup_output(undefined8 param_1,undefined8 param_2,undefined1 param_3)

{
  DAT_0012d848 = param_1;
  DAT_0012d850 = param_2;
  DAT_0012d858 = param_3;
  DAT_0012d5c0 = 0;
  return;
}




// Function: c_escape_char @ 0x118ef

undefined8 c_escape_char(char param_1)

{
  uint uVar1;
  ulong uVar2;
  
  uVar1 = (uint)param_1;
  uVar2 = (ulong)uVar1;
  if ((int)uVar1 < 0x23) {
    if (6 < (int)uVar1) {
      uVar1 = uVar1 - 7;
      uVar2 = (ulong)uVar1;
      if (uVar1 < 0x1c) {
        uVar2 = (long)&switchD_00111932::switchdataD_001255cc +
                (long)(int)(&switchD_00111932::switchdataD_001255cc)[uVar1];
        switch(uVar1) {
        case 0:
          return 0x61;
        case 1:
          return 0x62;
        case 2:
          return 0x74;
        case 3:
          return 0x6e;
        case 4:
          return 0x76;
        case 5:
          return 0x66;
        case 6:
          return 0x72;
        case 0x1b:
          return 0x22;
        }
      }
    }
  }
  else if (uVar1 == 0x5c) {
    return 0x5c;
  }
  return CONCAT71((int7)(uVar2 >> 8),param_1 < ' ');
}




// Function: c_escape @ 0x11984

byte * c_escape(byte *param_1)

{
  byte bVar1;
  bool bVar2;
  char cVar3;
  byte bVar4;
  byte *pbVar5;
  byte *local_30;
  long local_28;
  byte *local_20;
  
  local_28 = 0;
  bVar2 = false;
  for (local_30 = param_1; *local_30 != 0; local_30 = local_30 + 1) {
    if (*local_30 == 0x20) {
      bVar2 = true;
    }
    else {
      cVar3 = FUN_001118ef((int)(char)*local_30);
      if (cVar3 != '\0') {
        if (cVar3 == '\x01') {
          local_28 = local_28 + 3;
        }
        else {
          local_28 = local_28 + 1;
        }
      }
    }
  }
  if ((bVar2) || (local_28 != 0)) {
    pbVar5 = (byte *)FUN_0011c945(local_30 + (local_28 - (long)param_1) + 3);
    local_20 = pbVar5 + 1;
    *pbVar5 = 0x22;
    for (local_30 = param_1; *local_30 != 0; local_30 = local_30 + 1) {
      bVar1 = *local_30;
      bVar4 = FUN_001118ef((int)(char)bVar1);
      if (bVar4 == 0) {
        *local_20 = bVar1;
        local_20 = local_20 + 1;
      }
      else if (bVar4 == 1) {
        *local_20 = 0x5c;
        local_20[1] = (bVar1 >> 6) + 0x30;
        local_20[2] = ((char)bVar1 >> 3 & 7U) + 0x30;
        local_20[3] = (bVar1 & 7) + 0x30;
        local_20 = local_20 + 4;
      }
      else {
        *local_20 = 0x5c;
        local_20[1] = bVar4;
        local_20 = local_20 + 2;
      }
    }
    *local_20 = 0x22;
    local_20[1] = 0;
    param_1 = pbVar5;
  }
  return param_1;
}




// Function: begin_output @ 0x11b50

void begin_output(void)

{
  int iVar1;
  void *__ptr;
  undefined8 uVar2;
  int *piVar3;
  long in_FS_OFFSET;
  int local_50;
  int local_4c;
  void *local_48;
  void *local_40;
  char *local_38;
  undefined *local_30;
  void *local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0012d5c0 == (FILE *)0x0) {
    local_48 = (void *)FUN_00111984(DAT_0012d848);
    local_40 = (void *)FUN_00111984(DAT_0012d850);
    __ptr = (void *)FUN_0011d0f7("diff%s %s %s",DAT_0012d330,local_48,local_40);
    if (DAT_0012d2c0 == '\0') {
      DAT_0012d5c0 = stdout;
      FUN_001117e9(0);
      if (DAT_0012d858 != '\0') {
        printf("%s\n",__ptr);
      }
    }
    else {
      iVar1 = fflush_unlocked(stdout);
      if (iVar1 != 0) {
        uVar2 = gettext("write failed");
        FUN_00110925(uVar2);
      }
      local_38 = "/usr/bin/pr";
      local_30 = &DAT_00125656;
      local_20 = 0;
      local_28 = __ptr;
      iVar1 = pipe(&local_50);
      if (iVar1 != 0) {
        FUN_00110925(&DAT_00125659);
      }
      DAT_0012d870 = fork();
      if (DAT_0012d870 < 0) {
        FUN_00110925(&DAT_0012565e);
      }
      if (DAT_0012d870 == 0) {
        close(local_4c);
        if (local_50 != 0) {
          iVar1 = dup2(local_50,0);
          if (iVar1 < 0) {
            FUN_00110925(&DAT_00125663);
          }
          close(local_50);
        }
        execv("/usr/bin/pr",&local_38);
        piVar3 = __errno_location();
        if (*piVar3 == 2) {
          iVar1 = 0x7f;
        }
        else {
          iVar1 = 0x7e;
        }
                    /* WARNING: Subroutine does not return */
        _exit(iVar1);
      }
      close(local_50);
      DAT_0012d5c0 = fdopen(local_4c,"w");
      if (DAT_0012d5c0 == (FILE *)0x0) {
        FUN_00110925("fdopen");
      }
      FUN_001117e9(1);
    }
    free(__ptr);
    if (DAT_0012d1c0 == 2) {
      FUN_00107e5c(&DAT_0012d360,&local_48,0);
    }
    else if (DAT_0012d1c0 == 3) {
      FUN_00107e5c(&DAT_0012d360,&local_48,1);
    }
    if (local_48 != DAT_0012d848) {
      free(local_48);
    }
    if (local_40 != DAT_0012d850) {
      free(local_40);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: finish_output @ 0x11e1b

void finish_output(void)

{
  int iVar1;
  __pid_t _Var2;
  undefined8 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  uint local_1c;
  int local_18;
  uint local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0012d5c0 != (FILE *)0x0) && (DAT_0012d5c0 != stdout)) {
    local_18 = 0;
    iVar1 = ferror_unlocked(DAT_0012d5c0);
    if (iVar1 != 0) {
      FUN_00110966("write failed");
    }
    iVar1 = fclose(DAT_0012d5c0);
    if (iVar1 != 0) {
      uVar3 = gettext("write failed");
      FUN_00110925(uVar3);
    }
    _Var2 = waitpid(DAT_0012d870,(int *)&local_1c,0);
    if (_Var2 < 0) {
      FUN_00110925("waitpid");
    }
    if ((local_18 == 0) && ((local_1c & 0x7f) == 0)) {
      local_14 = (int)local_1c >> 8 & 0xff;
    }
    else {
      local_14 = 0x7fffffff;
    }
    if (local_14 != 0) {
      if (local_14 == 0x7e) {
        pcVar4 = "subsidiary program \'%s\' could not be invoked";
      }
      else if (local_14 == 0x7f) {
        pcVar4 = "subsidiary program \'%s\' not found";
      }
      else if (local_14 == 0x7fffffff) {
        pcVar4 = "subsidiary program \'%s\' failed";
      }
      else {
        pcVar4 = "subsidiary program \'%s\' failed (exit status %d)";
      }
      uVar3 = gettext(pcVar4);
                    /* WARNING: Subroutine does not return */
      error(2,local_18,uVar3,"/usr/bin/pr",local_14);
    }
  }
  DAT_0012d5c0 = (FILE *)0x0;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: lines_differ @ 0x11f90

undefined8 lines_differ(byte *param_1,byte *param_2)

{
  uint __c;
  uint __c_00;
  int iVar1;
  ushort **ppuVar2;
  long lVar3;
  char cVar4;
  byte bVar5;
  char cVar6;
  byte bVar7;
  byte *pbVar8;
  byte *pbVar9;
  byte *pbVar10;
  byte *pbVar11;
  ulong local_48;
  byte *local_40;
  byte *local_38;
  ulong local_30;
  
  local_48 = 0;
  pbVar8 = param_1;
  pbVar10 = param_2;
LAB_00111fbb:
  pbVar9 = pbVar8 + 1;
  bVar5 = *pbVar8;
  __c = (uint)bVar5;
  pbVar11 = pbVar10 + 1;
  bVar7 = *pbVar10;
  __c_00 = (uint)bVar7;
  if (bVar5 == bVar7) goto LAB_001123e3;
  switch(DAT_0012d1e8) {
  case 0:
    break;
  case 1:
switchD_0011200c_caseD_1:
    if (((bVar5 == 0x20) && (bVar7 == 9)) || ((bVar5 == 9 && (bVar7 == 0x20)))) {
      local_30 = local_48;
      do {
        if ((char)__c == ' ') {
          local_48 = local_48 + 1;
        }
        else {
          if ((char)__c != '\t') goto LAB_0011234b;
          local_48 = local_48 + (DAT_0012d2a8 - local_48 % DAT_0012d2a8);
        }
        __c = (uint)*pbVar9;
        pbVar9 = pbVar9 + 1;
      } while( true );
    }
    break;
  case 2:
  case 3:
    ppuVar2 = __ctype_b_loc();
    if ((((*ppuVar2)[__c] & 0x2000) == 0) ||
       (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[__c_00] & 0x2000) == 0)) {
      if (DAT_0012d1e8 != 2) goto switchD_0011200c_caseD_1;
    }
    else {
      local_40 = pbVar9;
      if (bVar5 != 10) {
        while ((bVar5 = *local_40, bVar5 != 10 &&
               (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[bVar5] & 0x2000) != 0))) {
          local_40 = local_40 + 1;
        }
        if (bVar5 != 10) break;
      }
      local_38 = pbVar11;
      if (bVar7 == 10) {
        return 0;
      }
      while ((bVar5 = *local_38, bVar5 != 10 &&
             (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[bVar5] & 0x2000) != 0))) {
        local_38 = local_38 + 1;
      }
      if (bVar5 == 10) {
        return 0;
      }
    }
    break;
  case 4:
    ppuVar2 = __ctype_b_loc();
    pbVar8 = pbVar9;
    if (((*ppuVar2)[bVar5] & 0x2000) != 0) {
      do {
        pbVar9 = pbVar8;
        if ((char)__c == '\n') goto LAB_001120dd;
        __c = (uint)*pbVar9;
        ppuVar2 = __ctype_b_loc();
        pbVar8 = pbVar9 + 1;
      } while (((*ppuVar2)[__c] & 0x2000) != 0);
      __c = 0x20;
    }
LAB_001120dd:
    ppuVar2 = __ctype_b_loc();
    pbVar8 = pbVar11;
    if (((*ppuVar2)[bVar7] & 0x2000) != 0) {
      do {
        pbVar11 = pbVar8;
        if ((char)__c_00 == '\n') goto LAB_00112140;
        __c_00 = (uint)*pbVar11;
        ppuVar2 = __ctype_b_loc();
        pbVar8 = pbVar11 + 1;
      } while (((*ppuVar2)[__c_00] & 0x2000) != 0);
      __c_00 = 0x20;
    }
LAB_00112140:
    cVar4 = (char)__c;
    cVar6 = (char)__c_00;
    if (cVar4 == cVar6) break;
    if ((((cVar6 == ' ') && (cVar4 != '\n')) && (param_1 + 1 < pbVar9)) &&
       (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[pbVar9[-2]] & 0x2000) != 0)) {
      pbVar8 = pbVar9 + -1;
      pbVar10 = pbVar11;
    }
    else {
      if (((cVar4 != ' ') || (cVar6 == '\n')) ||
         ((pbVar11 <= param_2 + 1 ||
          (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[pbVar11[-2]] & 0x2000) == 0)))) break;
      pbVar10 = pbVar11 + -1;
      pbVar8 = pbVar9;
    }
    goto LAB_00111fbb;
  case 5:
    while ((ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[__c] & 0x2000) != 0 && ((char)__c != '\n'))) {
      __c = (uint)*pbVar9;
      pbVar9 = pbVar9 + 1;
    }
    while ((ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[__c_00] & 0x2000) != 0 && ((char)__c_00 != '\n'))
          ) {
      __c_00 = (uint)*pbVar11;
      pbVar11 = pbVar11 + 1;
    }
  }
switchD_0011200c_default:
  bVar5 = (byte)__c;
  bVar7 = (byte)__c_00;
  if (DAT_0012d1ee != '\0') {
    iVar1 = tolower(__c);
    bVar5 = (byte)iVar1;
    iVar1 = tolower(__c_00);
    bVar7 = (byte)iVar1;
  }
  if (bVar5 != bVar7) {
    return 1;
  }
LAB_001123e3:
  if (bVar5 == 10) {
    return 0;
  }
  if (bVar5 == 9) {
    lVar3 = DAT_0012d2a8 - local_48 % DAT_0012d2a8;
  }
  else {
    lVar3 = 1;
  }
  local_48 = local_48 + lVar3;
  pbVar8 = pbVar9;
  pbVar10 = pbVar11;
  goto LAB_00111fbb;
LAB_0011234b:
  if ((char)__c_00 == ' ') {
    local_30 = local_30 + 1;
  }
  else {
    if ((char)__c_00 != '\t') goto code_r0x00112392;
    local_30 = local_30 + (DAT_0012d2a8 - local_30 % DAT_0012d2a8);
  }
  __c_00 = (uint)*pbVar11;
  pbVar11 = pbVar11 + 1;
  goto LAB_0011234b;
code_r0x00112392:
  if (local_48 != local_30) {
    return 1;
  }
  goto switchD_0011200c_default;
}




// Function: find_change @ 0x12437

undefined8 find_change(undefined8 param_1)

{
  return param_1;
}




// Function: find_reverse_change @ 0x12449

undefined8 find_reverse_change(undefined8 param_1)

{
  return param_1;
}




// Function: print_script @ 0x1245b

void print_script(long param_1,code *param_2,code *param_3)

{
  long lVar1;
  long *plVar2;
  undefined8 local_20;
  
  local_20 = param_1;
  while (local_20 != 0) {
    plVar2 = (long *)(*param_2)(local_20);
    lVar1 = *plVar2;
    *plVar2 = 0;
    (*param_3)(local_20);
    *plVar2 = lVar1;
    local_20 = lVar1;
  }
  return;
}




// Function: print_1_line @ 0x124cf

void print_1_line(undefined8 param_1,undefined8 param_2)

{
  FUN_001124fe(param_1,param_2,0);
  return;
}




// Function: print_1_line_nl @ 0x124fe

void print_1_line_nl(char *param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  FILE *__stream;
  int iVar2;
  undefined8 uVar3;
  char *local_38;
  char *local_30;
  char *local_28;
  
  __stream = DAT_0012d5c0;
  uVar3 = *param_2;
  lVar1 = param_2[1];
  local_38 = (char *)0x0;
  if ((param_1 != (char *)0x0) && (*param_1 != '\0')) {
    if (DAT_0012d2b0 == '\0') {
      local_38 = "%s ";
    }
    else {
      local_38 = "%s\t";
    }
    local_30 = local_38;
    local_28 = param_1;
    if ((DAT_0012d2b1 != '\0') && (*(char *)*param_2 == '\n')) {
      local_30 = "%s";
      local_28 = param_1 + (*param_1 == ' ');
    }
    fprintf(DAT_0012d5c0,local_30,local_28);
  }
  if ((param_3 == '\0') || (*(char *)(lVar1 + -1) != '\n')) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  FUN_0011267d(uVar3,lVar1 - iVar2,local_38,param_1);
  if (((param_1 == (char *)0x0) || (*param_1 != '\0')) && (*(char *)(lVar1 + -1) != '\n')) {
    FUN_001128c2(3);
    uVar3 = gettext("No newline at end of file");
    fprintf(__stream,"\n\\ %s\n",uVar3);
  }
  return;
}




// Function: output_1_line @ 0x1267d

void output_1_line(byte *param_1,byte *param_2,char *param_3,undefined8 param_4)

{
  byte *pbVar1;
  byte bVar2;
  FILE *__stream;
  ulong uVar3;
  size_t sVar4;
  ushort **ppuVar5;
  ulong uVar6;
  byte *local_70;
  ulong local_60;
  long local_58;
  long local_50;
  size_t local_48;
  
  __stream = DAT_0012d5c0;
  uVar3 = DAT_0012d2a8;
  local_48 = 0x400;
  if (DAT_0012d2a1 == '\x01') {
    uVar6 = 0;
    local_58 = 0;
    while (param_1 < param_2) {
      local_58 = local_58 + 1;
      if (local_58 == 0x400) {
        FUN_00110d14();
        local_58 = 0;
      }
      pbVar1 = param_1 + 1;
      bVar2 = *param_1;
      param_1 = pbVar1;
      if (bVar2 == 0xd) {
        putc_unlocked(0xd,__stream);
        if (((param_3 != (char *)0x0) && (pbVar1 < param_2)) && (*pbVar1 != 10)) {
          fprintf(__stream,param_3,param_4);
        }
        uVar6 = 0;
      }
      else if (bVar2 < 0xe) {
        if (bVar2 == 8) {
          if (uVar6 != 0) {
            uVar6 = uVar6 - 1;
            putc_unlocked(8,__stream);
          }
        }
        else {
          if (bVar2 != 9) goto LAB_00112838;
          local_50 = uVar3 - uVar6 % uVar3;
          uVar6 = local_50 + uVar6;
          do {
            putc_unlocked(0x20,__stream);
            local_50 = local_50 + -1;
          } while (local_50 != 0);
        }
      }
      else {
LAB_00112838:
        ppuVar5 = __ctype_b_loc();
        uVar6 = uVar6 + (((*ppuVar5)[bVar2] & 0x4000) != 0);
        putc_unlocked((uint)bVar2,__stream);
      }
    }
  }
  else {
    local_60 = (long)param_2 - (long)param_1;
    local_70 = param_1;
    while (local_60 != 0) {
      uVar3 = local_48;
      if (local_60 < 0x401) {
        uVar3 = local_60;
      }
      sVar4 = fwrite_unlocked(local_70,1,uVar3,DAT_0012d5c0);
      if (sVar4 < uVar3) {
        return;
      }
      local_70 = local_70 + sVar4;
      local_60 = local_60 - sVar4;
      FUN_00110d14();
    }
  }
  return;
}




// Function: put_indicator @ 0x12889

void put_indicator(size_t *param_1)

{
  fwrite_unlocked((void *)param_1[1],*param_1,1,DAT_0012d5c0);
  return;
}




// Function: set_color_context @ 0x128c2

void set_color_context(int param_1)

{
  if (param_1 != 3) {
    FUN_00110d14();
  }
  if ((DAT_0012d859 != '\0') && (param_1 != DAT_0012d010)) {
    FUN_00112889(&DAT_0012d040);
    switch(param_1) {
    case 0:
      FUN_00112889(&DAT_0012d080);
      break;
    case 1:
      FUN_00112889(&DAT_0012d090);
      break;
    case 2:
      FUN_00112889(&DAT_0012d0a0);
      break;
    case 3:
      FUN_00112889(&DAT_0012d070);
      break;
    case 4:
      FUN_00112889(&DAT_0012d0b0);
      break;
    default:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    FUN_00112889(&DAT_0012d050);
    DAT_0012d010 = param_1;
  }
  return;
}




// Function: translate_line_number @ 0x129a8

long translate_line_number(long param_1,long param_2)

{
  return param_2 + *(long *)(param_1 + 0xe8) + 1;
}




// Function: translate_range @ 0x129d0

void translate_range(undefined8 param_1,long param_2,long param_3,long *param_4,long *param_5)

{
  long lVar1;
  
  lVar1 = FUN_001129a8(param_1,param_2 + -1);
  *param_4 = lVar1 + 1;
  lVar1 = FUN_001129a8(param_1,param_3 + 1);
  *param_5 = lVar1 + -1;
  return;
}




// Function: print_number_range @ 0x12a37

void print_number_range(char param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001129d0(param_2,param_3,param_4,&local_20,&local_18);
  if (local_20 < local_18) {
    fprintf(DAT_0012d5c0,"%ld%c%ld",local_20,(ulong)(uint)(int)param_1,local_18);
  }
  else {
    fprintf(DAT_0012d5c0,"%ld",local_18);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: analyze_hunk @ 0x12af4

byte analyze_hunk(long *param_1,long *param_2,long *param_3,long *param_4,long *param_5)

{
  byte *pbVar1;
  bool bVar2;
  long lVar3;
  long lVar4;
  bool bVar5;
  bool bVar6;
  byte bVar7;
  uint uVar8;
  long lVar9;
  long lVar10;
  char *pcVar11;
  ushort **ppuVar12;
  int iVar13;
  long *local_a0;
  long local_98;
  long local_90;
  long local_88;
  byte *local_80;
  byte *local_78;
  
  lVar4 = DAT_0012d548;
  lVar3 = DAT_0012d418;
  if ((DAT_0012d1ec == 0) && (DAT_0012d280 == 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  uVar8 = (uint)DAT_0012d1ec;
  if ((DAT_0012d1ec == 0) || (DAT_0012d1e8 < 2)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  if ((bVar5) && (3 < DAT_0012d1e8)) {
    bVar6 = true;
  }
  else {
    bVar6 = false;
  }
  local_90 = 0;
  local_98 = 0;
  *param_2 = param_1[3];
  *param_4 = param_1[4];
  local_a0 = param_1;
  do {
    lVar9 = local_a0[2] + local_a0[3] + -1;
    lVar10 = local_a0[1] + local_a0[4] + -1;
    local_98 = local_98 + local_a0[2];
    local_90 = local_90 + local_a0[1];
    local_88 = local_a0[3];
    while ((local_88 <= lVar9 && (bVar2))) {
      pbVar1 = *(byte **)(lVar3 + local_88 * 8);
      pcVar11 = (char *)(*(long *)(lVar3 + (local_88 + 1) * 8) + -1);
      pcVar11 = pcVar11 + (*pcVar11 != '\n');
      iVar13 = (int)pcVar11 - (int)pbVar1;
      local_80 = pbVar1;
      if (bVar5) {
        for (; *local_80 != 10; local_80 = local_80 + 1) {
          ppuVar12 = __ctype_b_loc();
          if (((*ppuVar12)[*local_80] & 0x2000) == 0) {
            if (!bVar6) {
              local_80 = pbVar1;
            }
            break;
          }
        }
      }
      if (((long)pcVar11 - (long)local_80 != (long)(int)(uVar8 - 1)) &&
         ((DAT_0012d280 == 0 ||
          (iVar13 = re_search((re_pattern_buffer *)&DAT_0012d260,(char *)pbVar1,iVar13,0,iVar13,
                              (re_registers *)0x0), iVar13 < 0)))) {
        bVar2 = false;
      }
      local_88 = local_88 + 1;
    }
    local_88 = local_a0[4];
    while ((local_88 <= lVar10 && (bVar2))) {
      pbVar1 = *(byte **)(lVar4 + local_88 * 8);
      pcVar11 = (char *)(*(long *)(lVar4 + (local_88 + 1) * 8) + -1);
      pcVar11 = pcVar11 + (*pcVar11 != '\n');
      iVar13 = (int)pcVar11 - (int)pbVar1;
      local_78 = pbVar1;
      if (bVar5) {
        for (; *local_78 != 10; local_78 = local_78 + 1) {
          ppuVar12 = __ctype_b_loc();
          if (((*ppuVar12)[*local_78] & 0x2000) == 0) {
            if (!bVar6) {
              local_78 = pbVar1;
            }
            break;
          }
        }
      }
      if (((long)pcVar11 - (long)local_78 != (long)(int)(uVar8 - 1)) &&
         ((DAT_0012d280 == 0 ||
          (iVar13 = re_search((re_pattern_buffer *)&DAT_0012d260,(char *)pbVar1,iVar13,0,iVar13,
                              (re_registers *)0x0), iVar13 < 0)))) {
        bVar2 = false;
      }
      local_88 = local_88 + 1;
    }
    local_a0 = (long *)*local_a0;
    if (local_a0 == (long *)0x0) {
      *param_3 = lVar9;
      *param_5 = lVar10;
      if (bVar2) {
        bVar7 = 0;
      }
      else {
        if (local_90 == 0) {
          bVar7 = 0;
        }
        else {
          bVar7 = 2;
        }
        bVar7 = local_98 != 0 | bVar7;
      }
      return bVar7;
    }
  } while( true );
}




// Function: concat @ 0x12fa9

char * concat(char *param_1,char *param_2,char *param_3)

{
  size_t sVar1;
  size_t sVar2;
  size_t sVar3;
  char *__s;
  
  sVar1 = strlen(param_1);
  sVar2 = strlen(param_2);
  sVar3 = strlen(param_3);
  __s = (char *)FUN_0011c945(sVar3 + sVar1 + sVar2 + 1);
  sprintf(__s,"%s%s%s",param_1,param_2,param_3);
  return __s;
}




// Function: zalloc @ 0x13030

void * zalloc(size_t param_1)

{
  void *__s;
  
  __s = (void *)FUN_0011c945(param_1);
  memset(__s,0,param_1);
  return __s;
}




// Function: debug_script @ 0x1306b

void debug_script(undefined8 *param_1)

{
  undefined8 *local_30;
  
  fflush_unlocked(stdout);
  for (local_30 = param_1; local_30 != (undefined8 *)0x0; local_30 = (undefined8 *)*local_30) {
    fprintf(stderr,"%3ld %3ld delete %ld insert %ld\n",local_30[3],local_30[4],local_30[2],
            local_30[1]);
  }
  fflush_unlocked(stderr);
  return;
}



