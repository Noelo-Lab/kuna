// Function: main @ 0x4f20

/* WARNING: Removing unreachable block (ram,0x00105b0e) */
/* WARNING: Removing unreachable block (ram,0x001060cb) */
/* WARNING: Removing unreachable block (ram,0x001060df) */
/* WARNING: Removing unreachable block (ram,0x0010611d) */
/* WARNING: Removing unreachable block (ram,0x0010612b) */
/* WARNING: Removing unreachable block (ram,0x00106164) */
/* WARNING: Removing unreachable block (ram,0x00105b24) */
/* WARNING: Removing unreachable block (ram,0x00105d0e) */
/* WARNING: Removing unreachable block (ram,0x00105e5b) */
/* WARNING: Removing unreachable block (ram,0x00105e68) */
/* WARNING: Removing unreachable block (ram,0x00105e70) */
/* WARNING: Removing unreachable block (ram,0x00105e83) */
/* WARNING: Removing unreachable block (ram,0x00105e87) */
/* WARNING: Removing unreachable block (ram,0x00105d17) */
/* WARNING: Removing unreachable block (ram,0x0010618a) */
/* WARNING: Removing unreachable block (ram,0x001061b2) */
/* WARNING: Removing unreachable block (ram,0x0010618f) */
/* WARNING: Removing unreachable block (ram,0x001061a0) */
/* WARNING: Removing unreachable block (ram,0x001061a4) */
/* WARNING: Removing unreachable block (ram,0x001061b6) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void main(int param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  byte bVar2;
  char cVar3;
  uint uVar4;
  int iVar5;
  long lVar6;
  undefined1 *puVar7;
  undefined1 *puVar8;
  ulong uVar9;
  ulong uVar10;
  ulong uVar11;
  char *pcVar12;
  undefined **ppuVar13;
  char *pcVar14;
  undefined *puVar15;
  ulong local_88;
  undefined **local_58;
  
  puVar15 = &DAT_0011dc60;
  local_58 = (undefined **)0x0;
  DAT_001260b0 = 2;
  FUN_00114480(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  pcVar12 = "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ";
  textdomain("diffutils");
  FUN_0010fd50(0);
  _DAT_00126600 = &DAT_00126500;
  _DAT_001265c0 = &DAT_001264c0;
  re_set_syntax(0x50a46);
  DAT_00126410 = FUN_00110310();
  DAT_00126180 = 0;
  FUN_00116f50();
  local_88 = 0;
  ppuVar13 = &PTR_s_binary_00125180;
  do {
    uVar4 = getopt_long(param_1,param_2,pcVar12,ppuVar13,0);
    if (uVar4 == 0xffffffff) {
      if (((DAT_0012657c == 1) && (pcVar12 = getenv("TERM"), pcVar12 != (char *)0x0)) &&
         (iVar5 = strcmp(pcVar12,"dumb"), iVar5 == 0)) {
        DAT_0012657c = 0;
      }
      if (DAT_00126580 == 0) {
        FUN_00108f60(1);
      }
      if ((DAT_00126580 == 2) && (cVar3 = FUN_00110f10(2), cVar3 == '\0')) {
        DAT_00126400 = "%a %b %e %T %Y";
      }
      else {
        DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
    }
    else {
      if (uVar4 < 0x9b) {
                    /* WARNING: Could not recover jumptable at 0x00105056. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (*(code *)(puVar15 + *(int *)(puVar15 + (ulong)uVar4 * 4)))();
        return;
      }
      FUN_00108e40(0,0);
      cVar3 = FUN_00110f10(2);
      if (cVar3 == '\0') {
        DAT_00126400 = "%a %b %e %T %Y";
      }
      else {
        DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      if ((DAT_00126580 - 2U < 2) && ((DAT_00126570 < -1 || (DAT_00126570 != -1)))) {
        DAT_00126570 = -1;
      }
    }
    if (DAT_00126498 == 0) {
      DAT_00126498 = 8;
    }
    if (local_88 == 0) {
      if (DAT_001264a0 == '\0') {
        local_88 = 0x82;
        goto LAB_00105b88;
      }
      local_88 = 0x43;
      DAT_00126430 = 0x3f;
      uVar11 = local_88;
    }
    else {
      uVar10 = 1;
      if (DAT_001264a0 == '\0') {
LAB_00105b88:
        uVar10 = DAT_00126498;
      }
      uVar9 = (local_88 >> 1) + (uVar10 + 3 >> 1) + (ulong)((uint)(uVar10 + 3) & (uint)local_88 & 1)
      ;
      uVar11 = uVar9 - uVar9 % uVar10;
      if ((uVar11 < 4) || (local_88 <= uVar11)) {
        DAT_00126430 = 0;
        uVar11 = local_88;
      }
      else {
        DAT_00126430 = (uVar9 % uVar10 - uVar9) + local_88;
        if (uVar11 - 3 < DAT_00126430) {
          DAT_00126430 = uVar11 - 3;
        }
        if (DAT_00126430 == 0) {
          uVar11 = local_88;
        }
      }
    }
    local_88 = uVar11;
    DAT_00126428 = local_88;
    if (DAT_00126560 < DAT_00126570) {
      DAT_00126560 = DAT_00126570;
    }
    FUN_00108ea0(&DAT_001265e0);
    FUN_00108ea0(&DAT_001265a0);
    if (DAT_00126580 == 7) {
      if (DAT_00126440 == (char *)0x0) {
        DAT_00126440 = "%l\n";
      }
      if (DAT_00126448 == (undefined *)0x0) {
        DAT_00126448 = &DAT_0011c332;
      }
      if (DAT_00126450 == (undefined *)0x0) {
        DAT_00126450 = &DAT_0011c332;
      }
      if ((DAT_00126468 == (undefined *)0x0) &&
         (DAT_00126468 = &DAT_0011c155, DAT_00126478 != (undefined *)0x0)) {
        DAT_00126468 = DAT_00126478;
      }
      if ((DAT_00126470 == (undefined *)0x0) &&
         (DAT_00126470 = &DAT_0011c158, DAT_00126478 != (undefined *)0x0)) {
        DAT_00126470 = DAT_00126478;
      }
      if (DAT_00126460 == (char *)0x0) {
        DAT_00126460 = "%=";
        if (DAT_00126478 == (undefined *)0x0) goto LAB_0010604f;
      }
      else {
        if (DAT_00126478 == (undefined *)0x0) {
LAB_0010604f:
          DAT_00126478 = (undefined *)FUN_0010fa30(DAT_00126468,DAT_00126470,&DAT_0011e5b0);
          if (DAT_00126580 != 7) goto LAB_00105c36;
        }
        bVar2 = 1;
        if (*DAT_00126460 == '\0') goto LAB_00105c42;
      }
      iVar5 = strcmp(DAT_00126460,"%=");
      bVar2 = 0;
      if (iVar5 == 0) {
        bVar2 = *DAT_00126440 == '\0';
      }
    }
    else {
LAB_00105c36:
      bVar2 = DAT_00126580 != 8 | DAT_00126438;
    }
LAB_00105c42:
    pcVar12 = (char *)(param_2 + 1);
    DAT_00126553 = ~(DAT_00126554 | DAT_00126552 | DAT_00126490 |
                    (DAT_001265a0 != 0 || DAT_00126558 != 0)) & DAT_001264a1 & 1;
    DAT_00126578 = bVar2;
    if (optind < 2) goto LAB_00106023;
    uVar4 = optind - 2;
    puVar15 = (undefined *)0x1;
    pcVar14 = pcVar12;
    do {
      uVar1 = *(undefined8 *)pcVar14;
      pcVar14 = pcVar14 + 8;
      lVar6 = FUN_00114d70(uVar1);
      puVar15 = puVar15 + lVar6 + 1;
    } while ((char *)(param_2 + (ulong)uVar4 + 2) != pcVar14);
    puVar7 = (undefined1 *)FUN_00116a40(puVar15);
    puVar8 = puVar7;
    do {
      *puVar8 = 0x20;
      uVar1 = *(undefined8 *)pcVar12;
      pcVar12 = pcVar12 + 8;
      puVar8 = (undefined1 *)FUN_00114de0(puVar8 + 1,uVar1);
    } while ((char *)(param_2 + (ulong)uVar4 + 2) != pcVar12);
    while( true ) {
      *puVar8 = 0;
      lVar6 = (long)optind;
      DAT_00126420 = puVar7;
      if (param_1 - optind == 2) {
        iVar5 = FUN_00108f90(0,param_2[lVar6],param_2[lVar6 + 1]);
        FUN_0010e670();
        FUN_00109df0();
                    /* WARNING: Subroutine does not return */
        exit(iVar5);
      }
      if (1 < param_1 - optind) break;
      FUN_00108e40("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
LAB_00106023:
      puVar8 = (undefined1 *)FUN_00116a40();
      puVar7 = puVar8;
    }
    FUN_00108e40("extra operand \'%s\'",param_2[lVar6 + 2]);
    DAT_00126540 = optarg;
    ppuVar13 = local_58;
  } while( true );
}




// Function: diff_2_files @ 0x6e20

bool diff_2_files(int *param_1)

{
  char cVar1;
  size_t __n;
  ulong uVar2;
  undefined8 *puVar3;
  long lVar4;
  char cVar5;
  int iVar6;
  undefined8 uVar7;
  size_t sVar8;
  void *pvVar9;
  long lVar10;
  void *__ptr;
  long lVar11;
  void *__ptr_00;
  long lVar12;
  long lVar13;
  long *plVar14;
  undefined8 *puVar15;
  char *pcVar16;
  long lVar17;
  long lVar18;
  ulong uVar19;
  void *pvVar20;
  long lVar21;
  undefined8 *puVar22;
  ulong uVar23;
  int *piVar24;
  long lVar25;
  ulong uVar26;
  void *pvVar27;
  undefined1 *puVar28;
  void **ppvVar29;
  long lVar30;
  char *pcVar31;
  long lVar32;
  long in_FS_OFFSET;
  bool bVar33;
  byte bVar34;
  long local_f0;
  undefined8 *local_e8;
  undefined1 local_b8 [8];
  undefined1 local_b0 [8];
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  undefined8 local_98;
  undefined8 local_90;
  long local_88;
  long local_80;
  undefined1 local_78;
  long local_70;
  undefined8 local_68;
  long local_60;
  void *local_58 [3];
  long local_40;
  
  bVar34 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar5 = FUN_0010bbc0();
  if (cVar5 == '\0') {
    pvVar9 = (void *)FUN_0010fab0(*(long *)(param_1 + 0x7e) + *(long *)(param_1 + 0x32) + 4);
    *(long *)(param_1 + 0x46) = (long)pvVar9 + 1;
    *(long *)(param_1 + 0x92) = (long)pvVar9 + *(long *)(param_1 + 0x32) + 3;
    lVar10 = FUN_00116a40((*(long *)(param_1 + 0x32) + *(long *)(param_1 + 0x7e)) * 0x10);
    *(long *)(param_1 + 0x40) = lVar10;
    lVar10 = lVar10 + *(long *)(param_1 + 0x32) * 8;
    *(long *)(param_1 + 0x42) = lVar10;
    lVar10 = lVar10 + *(long *)(param_1 + 0x32) * 8;
    *(long *)(param_1 + 0x8c) = lVar10;
    *(long *)(param_1 + 0x8e) = lVar10 + *(long *)(param_1 + 0x7e) * 8;
    __ptr = (void *)FUN_0010fab0(*(long *)(param_1 + 0x4a) << 4);
    pvVar20 = (void *)((long)__ptr + *(long *)(param_1 + 0x4a) * 8);
    if (0 < *(long *)(param_1 + 0x32)) {
      lVar10 = *(long *)(param_1 + 0x3e);
      lVar11 = 0;
      do {
        lVar18 = lVar11 * 8;
        lVar11 = lVar11 + 1;
        plVar14 = (long *)((long)__ptr + *(long *)(lVar10 + lVar18) * 8);
        *plVar14 = *plVar14 + 1;
      } while (lVar11 < *(long *)(param_1 + 0x32));
    }
    lVar10 = *(long *)(param_1 + 0x7e);
    if (0 < lVar10) {
      lVar11 = *(long *)(param_1 + 0x8a);
      lVar18 = 0;
      do {
        lVar10 = lVar18 * 8;
        lVar18 = lVar18 + 1;
        plVar14 = (long *)((long)pvVar20 + *(long *)(lVar11 + lVar10) * 8);
        *plVar14 = *plVar14 + 1;
        lVar10 = *(long *)(param_1 + 0x7e);
      } while (lVar18 < lVar10);
    }
    puVar22 = &local_68;
    local_58[0] = __ptr;
    local_58[1] = pvVar20;
    __ptr_00 = (void *)FUN_0010fab0(lVar10 + *(long *)(param_1 + 0x32));
    ppvVar29 = local_58;
    uVar23 = *(ulong *)(param_1 + 0x32);
    local_60 = (long)__ptr_00 + uVar23;
    plVar14 = (long *)(param_1 + 0x3e);
    pvVar27 = __ptr_00;
    puVar15 = puVar22;
    do {
      lVar10 = *plVar14;
      uVar19 = uVar23 >> 8;
      if (uVar19 == 0) {
        if (uVar23 != 0) {
          uVar26 = 5;
          goto LAB_00107184;
        }
      }
      else {
        uVar26 = 5;
        do {
          uVar26 = uVar26 * 2;
          uVar19 = uVar19 >> 2;
        } while (uVar19 != 0);
LAB_00107184:
        uVar19 = 0;
LAB_00107196:
        do {
          lVar11 = *(long *)(lVar10 + uVar19 * 8);
          if (lVar11 != 0) {
            uVar2 = *(ulong *)((long)pvVar20 + lVar11 * 8);
            if (uVar2 == 0) {
              *(undefined1 *)((long)pvVar27 + uVar19) = 1;
            }
            else if (uVar26 < uVar2) {
              *(undefined1 *)((long)pvVar27 + uVar19) = 2;
              uVar19 = uVar19 + 1;
              if (uVar19 == uVar23) break;
              goto LAB_00107196;
            }
          }
          uVar19 = uVar19 + 1;
        } while (uVar19 != uVar23);
      }
      puVar15 = puVar15 + 1;
      if ((long *)(param_1 + 0xd6) == plVar14 + 0x26) goto code_r0x001071cf;
      pvVar20 = *ppvVar29;
      uVar23 = plVar14[0x20];
      ppvVar29 = ppvVar29 + -1;
      pvVar27 = (void *)*puVar15;
      plVar14 = plVar14 + 0x26;
    } while( true );
  }
  if ((((*(long *)(param_1 + 0x10) == *(long *)(param_1 + 0x5c) || *(long *)(param_1 + 0x10) < 1) ||
       (*(long *)(param_1 + 0x5c) < 1)) || ((-1 < *param_1 && ((param_1[10] & 0xf000U) != 0x8000))))
     || ((-1 < param_1[0x4c] && ((param_1[0x56] & 0xf000U) != 0x8000)))) {
    pvVar20 = *(void **)(param_1 + 0x28);
    if (*param_1 == param_1[0x4c]) {
      pvVar9 = *(void **)(param_1 + 0x74);
      bVar33 = false;
    }
    else {
      uVar7 = FUN_00117190(*(undefined8 *)(param_1 + 0x12),*(undefined8 *)(param_1 + 0x5e),
                           0x7ffffffffffffffe);
      sVar8 = FUN_00117190(8,uVar7,0x7ffffffffffffffe);
      uVar7 = FUN_00116a90(pvVar20,sVar8);
      *(undefined8 *)(param_1 + 0x28) = uVar7;
      uVar7 = FUN_00116a90(*(undefined8 *)(param_1 + 0x74),sVar8);
      *(undefined8 *)(param_1 + 0x74) = uVar7;
      while( true ) {
        if (-1 < *param_1) {
          FUN_0010b8a0(param_1,sVar8 - *(long *)(param_1 + 0x2c));
        }
        if (-1 < param_1[0x4c]) {
          FUN_0010b8a0(param_1 + 0x4c,sVar8 - *(long *)(param_1 + 0x78));
        }
        __n = *(size_t *)(param_1 + 0x2c);
        if (__n != *(size_t *)(param_1 + 0x78)) goto LAB_00106f97;
        pvVar20 = *(void **)(param_1 + 0x28);
        pvVar9 = *(void **)(param_1 + 0x74);
        iVar6 = memcmp(pvVar20,pvVar9,__n);
        if (iVar6 != 0) goto LAB_00106f97;
        if (__n != sVar8) break;
        param_1[0x78] = 0;
        param_1[0x79] = 0;
        param_1[0x2c] = 0;
        param_1[0x2d] = 0;
      }
      bVar33 = false;
    }
  }
  else {
LAB_00106f97:
    bVar33 = true;
    FUN_00106d90(1,param_1);
    pvVar20 = *(void **)(param_1 + 0x28);
    pvVar9 = *(void **)(param_1 + 0x74);
  }
LAB_00106fc4:
  if (pvVar20 != pvVar9) {
    free(pvVar20);
    pvVar9 = *(void **)(param_1 + 0x74);
  }
  free(pvVar9);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar33;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
code_r0x001071cf:
  local_f0 = 0;
  pvVar20 = __ptr_00;
  local_e8 = puVar22;
  do {
    lVar10 = *(long *)((long)param_1 + local_f0 + 200);
    lVar11 = 0;
    if (0 < lVar10) {
      do {
        while( true ) {
          pcVar16 = (char *)((long)pvVar20 + lVar11);
          if (*pcVar16 != '\x02') break;
          lVar11 = lVar11 + 1;
          *pcVar16 = '\0';
          if (lVar10 <= lVar11) goto LAB_0010722b;
        }
        lVar18 = lVar11;
        if (*pcVar16 != '\0') {
          lVar25 = 0;
          lVar32 = lVar11;
          while ((lVar32 = lVar32 + 1, lVar32 < lVar10 &&
                 (*(char *)((long)pvVar20 + lVar32) != '\0'))) {
            lVar25 = lVar25 + (ulong)(*(char *)((long)pvVar20 + lVar32) == '\x02');
          }
          if (lVar11 < lVar32) {
            do {
              if (*(char *)((long)pvVar20 + lVar32 + -1) != '\x02') {
                lVar12 = lVar32 - lVar11;
                if (lVar25 * 4 <= lVar12) goto LAB_00107380;
                if (lVar11 < lVar32) {
                  pcVar16 = (char *)((long)pvVar20 + lVar32 + -1);
                  do {
                    if (*pcVar16 == '\x02') {
                      *pcVar16 = '\0';
                    }
                    pcVar16 = pcVar16 + -1;
                  } while ((char *)((long)pvVar20 + lVar11 + -1) != pcVar16);
                }
                goto LAB_00107208;
              }
              lVar32 = lVar32 + -1;
              lVar25 = lVar25 + -1;
              *(undefined1 *)((long)pvVar20 + lVar32) = 0;
            } while (lVar11 != lVar32);
            lVar12 = 0;
            if (lVar25 < 1) {
LAB_001074b4:
              lVar18 = lVar11 + -1 + lVar12;
              if (0 < lVar12) {
                lVar32 = 1;
                goto LAB_001073a3;
              }
            }
          }
          else {
            lVar12 = lVar32 - lVar11;
            if (lVar25 * 4 <= lVar12) {
LAB_00107380:
              lVar18 = lVar12 >> 4;
              if (lVar18 < 1) goto LAB_001074b4;
              lVar32 = 1;
              do {
                lVar32 = lVar32 * 2;
                lVar18 = lVar18 >> 2;
              } while (lVar18 != 0);
              lVar18 = lVar11 + -1 + lVar12;
LAB_001073a3:
              lVar17 = 0;
              lVar25 = 0;
LAB_001073b6:
              do {
                pcVar31 = (char *)(lVar11 + lVar17 + (long)pvVar20);
                if (*pcVar31 == '\x02') {
                  lVar21 = lVar25 + 1;
                  if (lVar25 == lVar32) {
                    lVar17 = lVar17 - lVar21;
                  }
                  else if (lVar32 < lVar25) {
                    lVar17 = lVar17 + 1;
                    *pcVar31 = '\0';
                    lVar25 = lVar21;
                    if (lVar12 <= lVar17) break;
                    goto LAB_001073b6;
                  }
                }
                else {
                  lVar21 = 0;
                }
                lVar17 = lVar17 + 1;
                lVar25 = lVar21;
              } while (lVar17 < lVar12);
              lVar11 = 0;
              pcVar31 = pcVar16;
              do {
                while( true ) {
                  cVar5 = *pcVar31;
                  if ((7 < (long)pcVar31 - (long)pcVar16) && (cVar5 == '\x01')) goto LAB_00107433;
                  if (cVar5 != '\x02') break;
                  *pcVar31 = '\0';
                  pcVar31 = pcVar31 + 1;
                  lVar11 = 0;
                  if (pcVar16 + lVar12 == pcVar31) goto LAB_00107433;
                }
                if (cVar5 == '\0') {
                  lVar11 = 0;
                }
                else {
                  lVar11 = lVar11 + 1;
                  if (lVar11 == 3) break;
                }
                pcVar31 = pcVar31 + 1;
              } while (pcVar16 + lVar12 != pcVar31);
LAB_00107433:
              pcVar16 = (char *)((long)pvVar20 + (lVar18 - lVar12));
              lVar11 = 0;
              pcVar31 = (char *)((long)pvVar20 + lVar18);
              do {
                while( true ) {
                  if ((7 < ((long)pvVar20 + lVar18) - (long)pcVar31) && (*pcVar31 == '\x01'))
                  goto LAB_00107208;
                  if (*pcVar31 != '\x02') break;
                  *pcVar31 = '\0';
                  pcVar31 = pcVar31 + -1;
                  lVar11 = 0;
                  if (pcVar16 == pcVar31) goto LAB_00107208;
                }
                if (*pcVar31 == '\0') {
                  lVar11 = 0;
                }
                else {
                  lVar11 = lVar11 + 1;
                  if (lVar11 == 3) break;
                }
                pcVar31 = pcVar31 + -1;
              } while (pcVar16 != pcVar31);
            }
          }
        }
LAB_00107208:
        lVar11 = lVar18 + 1;
      } while (lVar11 < lVar10);
    }
LAB_0010722b:
    local_f0 = local_f0 + 0x130;
    local_e8 = local_e8 + 1;
    if (local_f0 == 0x260) break;
    pvVar20 = (void *)*local_e8;
  } while( true );
  plVar14 = (long *)(param_1 + 0x32);
  pvVar20 = __ptr_00;
  do {
    lVar10 = *plVar14;
    lVar11 = 0;
    if (0 < lVar10) {
      lVar18 = 0;
      do {
        while ((DAT_00126408 != '\0' || (*(char *)((long)pvVar20 + lVar18) == '\0'))) {
          lVar32 = lVar11 * 8;
          *(undefined8 *)(plVar14[7] + lVar11 * 8) = *(undefined8 *)(plVar14[6] + lVar18 * 8);
          lVar11 = lVar11 + 1;
          *(long *)(plVar14[8] + lVar32) = lVar18;
          lVar18 = lVar18 + 1;
          if (lVar10 == lVar18) goto LAB_001072b8;
        }
        *(undefined1 *)(plVar14[10] + lVar18) = 1;
        lVar18 = lVar18 + 1;
      } while (lVar10 != lVar18);
    }
LAB_001072b8:
    plVar14[9] = lVar11;
    plVar14 = plVar14 + 0x26;
    puVar22 = puVar22 + 1;
    if ((long *)(param_1 + 0xca) == plVar14) break;
    pvVar20 = (void *)*puVar22;
  } while( true );
  free(__ptr_00);
  free(__ptr);
  local_98 = *(undefined8 *)(param_1 + 0x40);
  lVar18 = *(long *)(param_1 + 0x90) + *(long *)(param_1 + 0x44) + 3;
  local_90 = *(undefined8 *)(param_1 + 0x8c);
  local_80 = FUN_00116a40(lVar18 * 0x10);
  cVar5 = DAT_00126408;
  lVar11 = *(long *)(param_1 + 0x90);
  lVar10 = lVar11 * 8 + 8;
  local_88 = local_80 + lVar10;
  local_80 = local_80 + lVar10 + lVar18 * 8;
  local_78 = DAT_00126418;
  if (lVar18 == 0) {
    local_70 = 0x1000;
  }
  else {
    local_70 = 1;
    do {
      local_70 = local_70 * 2;
      lVar18 = lVar18 >> 2;
    } while (lVar18 != 0);
    if (local_70 < 0x1000) {
      local_70 = 0x1000;
    }
  }
  lVar18 = 0;
  piVar24 = param_1;
  puVar22 = &DAT_001261a0;
  for (lVar10 = 0x26; lVar10 != 0; lVar10 = lVar10 + -1) {
    *puVar22 = *(undefined8 *)piVar24;
    piVar24 = piVar24 + (ulong)bVar34 * -4 + 2;
    puVar22 = puVar22 + (ulong)bVar34 * -2 + 1;
  }
  piVar24 = param_1 + 0x4c;
  for (lVar10 = 0x26; lVar10 != 0; lVar10 = lVar10 + -1) {
    *puVar22 = *(undefined8 *)piVar24;
    piVar24 = piVar24 + (ulong)bVar34 * -4 + 2;
    puVar22 = puVar22 + (ulong)bVar34 * -2 + 1;
  }
  FUN_00106360(0,*(undefined8 *)(param_1 + 0x44),0,lVar11,cVar5,&local_98);
  free((void *)(*(long *)(param_1 + 0x90) * -8 + -8 + local_88));
LAB_001075e0:
  lVar10 = *(long *)((long)param_1 + lVar18 + 0x118);
  lVar17 = 0;
  lVar12 = 0;
  lVar11 = *(long *)((long)param_1 + lVar18 + 0xf8);
  lVar32 = *(long *)((long)param_1 + lVar18 + 200);
  lVar25 = *(long *)((long)param_1 + (0x248 - lVar18));
LAB_0010760f:
  bVar33 = lVar32 == lVar12;
  if (lVar32 <= lVar12) goto LAB_00107765;
LAB_00107618:
  lVar21 = lVar12;
  if (*(char *)(lVar10 + lVar12) == '\0') goto LAB_00107921;
LAB_00107626:
  do {
    lVar12 = lVar12 + 1;
  } while (*(char *)(lVar10 + lVar12) != '\0');
  for (; *(char *)(lVar25 + lVar17) != '\0'; lVar17 = lVar17 + 1) {
  }
  do {
    lVar30 = lVar12 - lVar21;
    while ((lVar21 != 0 &&
           (*(long *)(lVar11 + -8 + lVar21 * 8) == *(long *)(lVar11 + -8 + lVar12 * 8)))) {
      lVar12 = lVar12 + -1;
      *(undefined1 *)(lVar10 + lVar21 + -1) = 1;
      *(undefined1 *)(lVar10 + lVar12) = 0;
      cVar5 = *(char *)(lVar10 + -2 + lVar21);
      lVar21 = lVar21 + -1;
      while (cVar5 != '\0') {
        lVar21 = lVar21 + -1;
        cVar5 = *(char *)(lVar10 + -1 + lVar21);
      }
      do {
        lVar17 = lVar17 + -1;
      } while (*(char *)(lVar25 + lVar17) != '\0');
    }
    lVar4 = lVar12;
    if (*(char *)(lVar25 + -1 + lVar17) == '\0') {
      lVar4 = lVar32;
    }
    while ((lVar12 != lVar32 && (*(long *)(lVar11 + lVar21 * 8) == *(long *)(lVar11 + lVar12 * 8))))
    {
      lVar21 = lVar21 + 1;
      lVar13 = lVar12 + 1;
      *(undefined1 *)(lVar10 + -1 + lVar21) = 0;
      *(undefined1 *)(lVar10 + lVar12) = 1;
      lVar12 = lVar4;
      if (*(char *)(lVar10 + lVar13) == '\0') goto LAB_00107703;
      do {
        lVar13 = lVar13 + 1;
      } while (*(char *)(lVar10 + lVar13) != '\0');
      lVar17 = lVar17 + 1;
      cVar5 = *(char *)(lVar25 + lVar17);
      while (lVar12 = lVar13, cVar5 != '\0') {
LAB_00107703:
        lVar17 = lVar17 + 1;
        lVar4 = lVar12;
        cVar5 = *(char *)(lVar25 + lVar17);
      }
    }
  } while (lVar30 != lVar12 - lVar21);
  if (lVar12 <= lVar4) goto LAB_0010760f;
  puVar28 = (undefined1 *)(lVar10 + -1 + lVar12);
  do {
    puVar28[lVar21 - lVar12] = 1;
    *puVar28 = 0;
    do {
      lVar17 = lVar17 + -1;
    } while (*(char *)(lVar25 + lVar17) != '\0');
    puVar28 = puVar28 + -1;
  } while ((undefined1 *)(lVar10 + -1 + lVar4) != puVar28);
  bVar33 = lVar32 == lVar4;
  lVar12 = lVar4;
  if (!bVar33 && lVar4 <= lVar32) goto LAB_00107618;
LAB_00107765:
  lVar21 = lVar12;
  if (!bVar33) goto LAB_00107626;
  lVar18 = lVar18 + 0x130;
  if (lVar18 != 0x260) goto LAB_001075e0;
  lVar10 = *(long *)(param_1 + 0x46);
  lVar11 = *(long *)(param_1 + 0x92);
  uVar23 = *(ulong *)(param_1 + 0x32);
  uVar19 = *(ulong *)(param_1 + 0x7e);
  if (DAT_00126580 == 4) {
    if (((long)uVar23 < 1) && ((long)uVar19 < 1)) goto LAB_00107b50;
    lVar32 = 0;
    lVar18 = 0;
    puVar22 = (undefined8 *)0x0;
    do {
      puVar15 = puVar22;
      lVar25 = lVar18;
      lVar12 = lVar32;
      if (*(char *)(lVar10 + lVar18) != '\0' || *(char *)(lVar11 + lVar32) != '\0') {
        if (*(char *)(lVar10 + lVar18) == '\0') {
          lVar17 = 0;
        }
        else {
          do {
            lVar25 = lVar25 + 1;
          } while (*(char *)(lVar10 + lVar25) != '\0');
          lVar17 = lVar25 - lVar18;
        }
        if (*(char *)(lVar11 + lVar32) == '\0') {
          lVar21 = 0;
        }
        else {
          do {
            lVar12 = lVar12 + 1;
          } while (*(char *)(lVar11 + lVar12) != '\0');
          lVar21 = lVar12 - lVar32;
        }
        puVar15 = (undefined8 *)FUN_00116a40(0x30);
        puVar15[3] = lVar18;
        puVar15[4] = lVar32;
        *puVar15 = puVar22;
        puVar15[1] = lVar21;
        puVar15[2] = lVar17;
      }
      lVar18 = lVar25 + 1;
      lVar32 = lVar12 + 1;
      puVar22 = puVar15;
    } while ((lVar18 < (long)uVar23) || (lVar32 < (long)uVar19));
  }
  else {
    if ((long)(uVar23 & uVar19) < 0) {
LAB_00107b50:
      if ((DAT_00126554 != '\0') || (puVar15 = (undefined8 *)0x0, DAT_001264e0 != 0))
      goto LAB_00107b69;
      goto LAB_00107c25;
    }
    puVar22 = (undefined8 *)0x0;
    do {
      cVar5 = *(char *)(lVar10 + -1 + uVar23);
      cVar1 = *(char *)(lVar11 + -1 + uVar19);
      puVar15 = puVar22;
      if (cVar5 != '\0' || cVar1 != '\0') {
        uVar26 = uVar23;
        if (cVar5 == '\0') {
          lVar18 = 0;
        }
        else {
          do {
            uVar26 = uVar26 - 1;
          } while (*(char *)(lVar10 + -1 + uVar26) != '\0');
          lVar18 = uVar23 - uVar26;
          uVar23 = uVar26;
        }
        uVar26 = uVar19;
        if (cVar1 == '\0') {
          lVar32 = 0;
        }
        else {
          do {
            uVar26 = uVar26 - 1;
          } while (*(char *)(lVar11 + -1 + uVar26) != '\0');
          lVar32 = uVar19 - uVar26;
          uVar19 = uVar26;
        }
        puVar15 = (undefined8 *)FUN_00116a40(0x30);
        *puVar15 = puVar22;
        puVar15[3] = uVar23;
        puVar15[4] = uVar19;
        puVar15[1] = lVar32;
        puVar15[2] = lVar18;
      }
      uVar19 = uVar19 - 1;
      uVar23 = uVar23 - 1;
      puVar22 = puVar15;
    } while ((-1 < (long)uVar23) || (-1 < (long)uVar19));
  }
  if ((DAT_00126554 == '\0') && (DAT_001264e0 == 0)) {
LAB_00107c25:
    bVar33 = puVar15 != (undefined8 *)0x0;
LAB_0010786c:
    if (DAT_001264a1 == '\0') {
      if (bVar33 == false) {
LAB_00107b7c:
        bVar33 = false;
        if (DAT_00126578 != '\0') goto LAB_0010788b;
      }
      lVar10 = DAT_00126548;
      if (DAT_00126548 == 0) {
        lVar10 = *(long *)(param_1 + 0x4e);
      }
      lVar11 = DAT_00126540;
      if (DAT_00126540 == 0) {
        lVar11 = *(long *)(param_1 + 2);
      }
      FUN_0010e800(lVar11,lVar10,*(long *)(param_1 + 0x98) != 0);
      switch(DAT_00126580) {
      default:
                    /* WARNING: Subroutine does not return */
        abort();
      case 1:
        FUN_0010cff0(puVar15);
        break;
      case 2:
        FUN_00108c10(puVar15,0);
        break;
      case 3:
        FUN_00108c10(puVar15,1);
        break;
      case 4:
        FUN_0010ac20(puVar15);
        break;
      case 5:
        FUN_0010ac40(puVar15);
        break;
      case 6:
        FUN_0010ac60(puVar15);
        break;
      case 7:
        FUN_0010b6c0(puVar15);
        break;
      case 8:
        FUN_0010da50(puVar15);
      }
      FUN_0010ead0();
    }
    else if (bVar33 != false) {
      FUN_00106d90(1,param_1);
    }
LAB_0010788b:
    free(*(void **)(param_1 + 0x40));
    free(pvVar9);
    free(*(void **)(param_1 + 0x3e));
    free((void *)(*(long *)(param_1 + 0x2e) + *(long *)(param_1 + 0x30) * 8));
    free(*(void **)(param_1 + 0x8a));
    free((void *)(*(long *)(param_1 + 0x7a) + *(long *)(param_1 + 0x7c) * 8));
    while (puVar15 != (undefined8 *)0x0) {
      puVar22 = (undefined8 *)*puVar15;
      free(puVar15);
      puVar15 = puVar22;
    }
    if (DAT_00126580 - 4U < 2) {
      plVar14 = &DAT_00126540;
      piVar24 = param_1 + 0x48;
      do {
        if ((char)*piVar24 != '\0') {
          uVar7 = dcgettext(0,"No newline at end of file",5);
          lVar10 = *plVar14;
          if (lVar10 == 0) {
            lVar10 = *(long *)(piVar24 + -0x46);
          }
                    /* WARNING: Subroutine does not return */
          error(0,0,"%s: %s\n",lVar10,uVar7);
        }
        plVar14 = plVar14 + 1;
        piVar24 = piVar24 + 0x4c;
      } while (plVar14 != (long *)&DAT_00126550);
    }
    pvVar20 = *(void **)(param_1 + 0x28);
    pvVar9 = *(void **)(param_1 + 0x74);
    goto LAB_00106fc4;
  }
  if (puVar15 == (undefined8 *)0x0) {
LAB_00107b69:
    puVar15 = (undefined8 *)0x0;
    bVar33 = false;
    if (DAT_001264a1 == '\0') goto LAB_00107b7c;
    goto LAB_0010788b;
  }
  puVar22 = puVar15;
  do {
    plVar14 = (long *)FUN_0010f060(puVar22);
    puVar3 = (undefined8 *)*plVar14;
    *plVar14 = 0;
    iVar6 = FUN_0010f670(puVar22,local_b8,local_b0,local_a8,local_a0);
    *plVar14 = (long)puVar3;
    if (puVar3 == (undefined8 *)0x0) break;
    puVar22 = puVar3;
  } while (iVar6 == 0);
  bVar33 = iVar6 != 0;
  goto LAB_0010786c;
LAB_00107921:
  do {
    lVar17 = lVar17 + 1;
  } while (*(char *)(lVar25 + -1 + lVar17) != '\0');
  lVar12 = lVar12 + 1;
  goto LAB_0010760f;
}




// Function: print_context_label @ 0x7cf0

void print_context_label(undefined8 param_1,long param_2,undefined8 param_3,long param_4)

{
  char *pcVar1;
  ulong uVar2;
  tm *ptVar3;
  long lVar4;
  long in_FS_OFFSET;
  undefined1 auStack_68 [56];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f510(0);
  if (param_4 != 0) {
    __fprintf_chk(DAT_00126188,1,"%s %s",param_1,param_4);
    goto LAB_00107d44;
  }
  ptVar3 = localtime((time_t *)(param_2 + 0x68));
  uVar2 = *(ulong *)(param_2 + 0x70);
  if (ptVar3 == (tm *)0x0) {
LAB_00107dc7:
    __sprintf_chk(auStack_68,1,0x2b,"%ld.%.9d",*(undefined8 *)(param_2 + 0x68),uVar2 & 0xffffffff);
  }
  else {
    lVar4 = FUN_00114460(auStack_68,0x2b,DAT_00126400,ptVar3,0,uVar2 & 0xffffffff);
    if (lVar4 == 0) goto LAB_00107dc7;
  }
  __fprintf_chk(DAT_00126188,1,"%s %s\t%s",param_1,param_3,auStack_68);
LAB_00107d44:
  FUN_0010f510(3);
  pcVar1 = DAT_00126188->_IO_write_ptr;
  if (pcVar1 < DAT_00126188->_IO_write_end) {
    DAT_00126188->_IO_write_ptr = pcVar1 + 1;
    *pcVar1 = '\n';
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    __overflow(DAT_00126188,10);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_function @ 0x7e50

char * find_function(long param_1,long param_2)

{
  long lVar1;
  int iVar2;
  ulong uVar3;
  char *pcVar4;
  
  lVar1 = DAT_00126178;
  DAT_00126178 = param_2;
  do {
    param_2 = param_2 + -1;
    if (param_2 < lVar1) {
      pcVar4 = (char *)0x0;
      if (DAT_00126170 != 0x7fffffffffffffff) {
        pcVar4 = *(char **)(param_1 + DAT_00126170 * 8);
      }
      return pcVar4;
    }
    pcVar4 = *(char **)(param_1 + param_2 * 8);
    uVar3 = ~(ulong)pcVar4 + *(long *)(param_1 + 8 + param_2 * 8);
    if (0x7fffffff < uVar3) {
      uVar3 = 0x7fffffff;
    }
    iVar2 = re_search((re_pattern_buffer *)&DAT_00126500,pcVar4,(int)uVar3,0,(int)uVar3,
                      (re_registers *)0x0);
  } while (iVar2 < 0);
  DAT_00126170 = param_2;
  return pcVar4;
}




// Function: print_context_function @ 0x7f00

void print_context_function(_IO_FILE *param_1,char *param_2)

{
  char cVar1;
  int iVar2;
  int iVar3;
  char *pcVar4;
  char *pcVar5;
  
  pcVar4 = param_1->_IO_write_ptr;
  if (pcVar4 < param_1->_IO_write_end) {
    param_1->_IO_write_ptr = pcVar4 + 1;
    *pcVar4 = ' ';
  }
  else {
    __overflow(param_1,0x20);
  }
  iVar2 = 0;
  pcVar4 = param_2;
  do {
    cVar1 = *pcVar4;
    if (cVar1 < '\x0e') {
      if ((cVar1 < '\t') || (cVar1 == '\n')) break;
    }
    else if (cVar1 != ' ') break;
    pcVar4 = pcVar4 + 1;
    iVar2 = iVar2 + 1;
  } while( true );
  pcVar5 = param_2 + (long)iVar2 + 1;
  iVar3 = iVar2;
  while ((cVar1 != '\n' && (iVar3 = iVar3 + 1, iVar3 <= iVar2 + 0x27))) {
    cVar1 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  }
  if (iVar2 < iVar3) {
    param_2 = param_2 + (long)iVar3 + -1;
    do {
      cVar1 = *param_2;
      if (cVar1 < '\x0e') {
        if (cVar1 < '\t') break;
      }
      else if (cVar1 != ' ') break;
      iVar3 = iVar3 + -1;
      param_2 = param_2 + -1;
    } while (iVar2 != iVar3);
  }
  fwrite_unlocked(pcVar4,1,(long)(iVar3 - iVar2),param_1);
  return;
}




// Function: print_context_number_range @ 0x7fe0

void print_context_number_range(void)

{
  long lVar1;
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f5e0();
  if (local_20 < local_18) {
    __fprintf_chk(DAT_00126188,1,&DAT_0011c08e);
  }
  else {
    __fprintf_chk(DAT_00126188,1,&DAT_0011c092,local_18);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pr_context_hunk @ 0x8070

void pr_context_hunk(undefined8 *param_1)

{
  char *pcVar1;
  bool bVar2;
  uint uVar3;
  long lVar4;
  FILE *__stream;
  undefined *puVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010f670(param_1,&local_60,&local_58,&local_50,&local_48);
  if (uVar3 != 0) {
    lVar4 = -DAT_00126288;
    local_60 = local_60 - DAT_00126570;
    if (local_60 < lVar4) {
      local_60 = lVar4;
    }
    local_50 = local_50 - DAT_00126570;
    if (local_50 < lVar4) {
      local_50 = lVar4;
    }
    lVar4 = DAT_00126570 + local_58;
    bVar2 = local_58 < DAT_00126270 - DAT_00126570;
    local_58 = DAT_00126270 + -1;
    if (bVar2) {
      local_58 = lVar4;
    }
    lVar4 = DAT_00126570 + local_48;
    bVar2 = local_48 < DAT_001263a0 - DAT_00126570;
    local_48 = DAT_001263a0 + -1;
    if (bVar2) {
      local_48 = lVar4;
    }
    if (DAT_00126520 == 0) {
      FUN_0010e830();
      __stream = DAT_00126188;
      fputs_unlocked("***************",DAT_00126188);
    }
    else {
      lVar4 = FUN_00107e50(DAT_00126258);
      FUN_0010e830();
      __stream = DAT_00126188;
      fputs_unlocked("***************",DAT_00126188);
      if (lVar4 != 0) {
        FUN_00107f00(__stream,lVar4);
      }
    }
    pcVar1 = __stream->_IO_write_ptr;
    if (pcVar1 < __stream->_IO_write_end) {
      __stream->_IO_write_ptr = pcVar1 + 1;
      *pcVar1 = '\n';
    }
    else {
      __overflow(__stream,10);
    }
    FUN_0010f510(4);
    fputs_unlocked("*** ",__stream);
    FUN_00107fe0(&DAT_001261a0,local_60,local_58);
    fputs_unlocked(" ****",__stream);
    FUN_0010f510(3);
    pcVar1 = __stream->_IO_write_ptr;
    if (pcVar1 < __stream->_IO_write_end) {
      __stream->_IO_write_ptr = pcVar1 + 1;
      *pcVar1 = '\n';
    }
    else {
      __overflow(__stream,10);
    }
    if (((uVar3 & 1) != 0) && (puVar6 = param_1, lVar4 = local_60, local_60 <= local_58)) {
      do {
        FUN_0010f510(2);
        for (; puVar6 != (undefined8 *)0x0; puVar6 = (undefined8 *)*puVar6) {
          if (lVar4 < (long)(puVar6[2] + puVar6[3])) {
            if ((long)puVar6[3] <= lVar4) {
              puVar5 = &DAT_0011c0c7;
              if (0 < (long)puVar6[1]) {
                puVar5 = &DAT_0011c096;
              }
              goto LAB_00108274;
            }
            break;
          }
        }
        puVar5 = &DAT_0011c0ab;
LAB_00108274:
        FUN_0010f330(puVar5,DAT_00126258 + lVar4 * 8,1);
        FUN_0010f510(3);
        if (*(char *)(*(long *)(DAT_00126258 + 8 + lVar4 * 8) + -1) == '\n') {
          pcVar1 = __stream->_IO_write_ptr;
          if (pcVar1 < __stream->_IO_write_end) {
            __stream->_IO_write_ptr = pcVar1 + 1;
            *pcVar1 = '\n';
          }
          else {
            __overflow(__stream,10);
          }
        }
        lVar4 = lVar4 + 1;
      } while (lVar4 <= local_58);
    }
    FUN_0010f510(4);
    fputs_unlocked("--- ",__stream);
    FUN_00107fe0(&DAT_001262d0,local_50,local_48);
    fputs_unlocked(" ----",__stream);
    FUN_0010f510(3);
    pcVar1 = __stream->_IO_write_ptr;
    if (pcVar1 < __stream->_IO_write_end) {
      __stream->_IO_write_ptr = pcVar1 + 1;
      *pcVar1 = '\n';
    }
    else {
      __overflow(__stream,10);
    }
    if (((uVar3 & 2) != 0) && (local_50 <= local_48)) {
      lVar4 = local_50;
      do {
        FUN_0010f510(1);
        for (; puVar5 = &DAT_0011c0ab, param_1 != (undefined8 *)0x0;
            param_1 = (undefined8 *)*param_1) {
          if (lVar4 < (long)(param_1[1] + param_1[4])) {
            if (((long)param_1[4] <= lVar4) && (puVar5 = &DAT_0011c0ca, 0 < (long)param_1[2])) {
              puVar5 = &DAT_0011c096;
            }
            break;
          }
        }
        FUN_0010f330(puVar5,DAT_00126388 + lVar4 * 8,1);
        FUN_0010f510(3);
        if (*(char *)(*(long *)(DAT_00126388 + 8 + lVar4 * 8) + -1) == '\n') {
          pcVar1 = __stream->_IO_write_ptr;
          if (pcVar1 < __stream->_IO_write_end) {
            __stream->_IO_write_ptr = pcVar1 + 1;
            *pcVar1 = '\n';
          }
          else {
            __overflow(__stream,10);
          }
        }
        lVar4 = lVar4 + 1;
      } while (lVar4 <= local_48);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_unidiff_number_range @ 0x84f0

void print_unidiff_number_range(void)

{
  long lVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f5e0();
  if (local_20 < local_18) {
    __fprintf_chk(DAT_00126188,1,&DAT_0011c08e,local_20,(local_18 - local_20) + 1);
  }
  else {
    pcVar2 = "%ld,0";
    if (local_20 <= local_18) {
      pcVar2 = "%ld";
    }
    __fprintf_chk(DAT_00126188,1,pcVar2);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pr_unidiff_hunk @ 0x8590

void pr_unidiff_hunk(undefined8 *param_1)

{
  long lVar1;
  char *pcVar2;
  bool bVar3;
  long lVar4;
  int iVar5;
  long lVar6;
  long lVar7;
  undefined8 *puVar8;
  long lVar9;
  FILE *__stream;
  long in_FS_OFFSET;
  long local_78;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar5 = FUN_0010f670(param_1,&local_60,&local_58,&local_50,&local_48);
  if (iVar5 != 0) {
    local_60 = local_60 - DAT_00126570;
    lVar6 = -DAT_00126288;
    if (local_60 < lVar6) {
      local_60 = lVar6;
    }
    local_50 = local_50 - DAT_00126570;
    if (local_50 < lVar6) {
      local_50 = lVar6;
    }
    lVar6 = DAT_00126570 + local_58;
    bVar3 = local_58 < DAT_00126270 - DAT_00126570;
    local_58 = DAT_00126270 + -1;
    if (bVar3) {
      local_58 = lVar6;
    }
    lVar6 = DAT_00126570 + local_48;
    bVar3 = local_48 < DAT_001263a0 - DAT_00126570;
    local_48 = DAT_001263a0 + -1;
    if (bVar3) {
      local_48 = lVar6;
    }
    if (DAT_00126520 == 0) {
      FUN_0010e830();
      __stream = DAT_00126188;
      FUN_0010f510(4);
      fputs_unlocked("@@ -",__stream);
      FUN_001084f0(&DAT_001261a0,local_60,local_58);
      fputs_unlocked(" +",__stream);
      FUN_001084f0(&DAT_001262d0,local_50,local_48);
      fputs_unlocked(" @@",__stream);
      FUN_0010f510(3);
    }
    else {
      lVar6 = FUN_00107e50(DAT_00126258);
      FUN_0010e830();
      __stream = DAT_00126188;
      FUN_0010f510(4);
      fputs_unlocked("@@ -",__stream);
      FUN_001084f0(&DAT_001261a0,local_60,local_58);
      fputs_unlocked(" +",__stream);
      FUN_001084f0(&DAT_001262d0,local_50,local_48);
      fputs_unlocked(" @@",__stream);
      FUN_0010f510(3);
      if (lVar6 != 0) {
        FUN_00107f00(__stream);
      }
    }
    pcVar2 = __stream->_IO_write_ptr;
    if (pcVar2 < __stream->_IO_write_end) {
      __stream->_IO_write_ptr = pcVar2 + 1;
      *pcVar2 = '\n';
    }
    else {
      __overflow(__stream,10);
    }
    local_78 = local_50;
    lVar6 = local_60;
    while ((lVar6 <= local_58 || (local_78 <= local_48))) {
      while ((param_1 == (undefined8 *)0x0 || (lVar6 < (long)param_1[3]))) {
        lVar1 = lVar6 + 1;
        if ((DAT_00126491 == '\0') || (**(char **)(DAT_00126258 + lVar6 * 8) != '\n')) {
          pcVar2 = __stream->_IO_write_ptr;
          iVar5 = (-(uint)(DAT_00126492 == '\0') & 0x17) + 9;
          if (pcVar2 < __stream->_IO_write_end) {
            __stream->_IO_write_ptr = pcVar2 + 1;
            *pcVar2 = (char)iVar5;
          }
          else {
            __overflow(__stream,iVar5);
          }
        }
        FUN_0010f500(0);
        local_78 = local_78 + 1;
        lVar6 = lVar1;
        if ((local_58 < lVar1) && (local_48 < local_78)) goto LAB_001087bc;
      }
      lVar1 = param_1[2];
      if (lVar1 != 0) {
        lVar9 = lVar6 * 8;
        lVar7 = lVar1;
LAB_0010888e:
        do {
          lVar4 = DAT_00126258;
          FUN_0010f510(2);
          pcVar2 = __stream->_IO_write_ptr;
          puVar8 = (undefined8 *)(lVar4 + lVar9);
          if (pcVar2 < __stream->_IO_write_end) {
            __stream->_IO_write_ptr = pcVar2 + 1;
            *pcVar2 = '-';
          }
          else {
            __overflow(__stream,0x2d);
          }
          if ((DAT_00126492 != '\0') && ((DAT_00126491 == '\0' || (*(char *)*puVar8 != '\n')))) {
            pcVar2 = __stream->_IO_write_ptr;
            if (pcVar2 < __stream->_IO_write_end) {
              __stream->_IO_write_ptr = pcVar2 + 1;
              *pcVar2 = '\t';
            }
            else {
              __overflow(__stream,9);
            }
          }
          FUN_0010f330(0,puVar8,1);
          FUN_0010f510(3);
          if (*(char *)(puVar8[1] + -1) == '\n') {
            pcVar2 = __stream->_IO_write_ptr;
            if (pcVar2 < __stream->_IO_write_end) {
              lVar9 = lVar9 + 8;
              __stream->_IO_write_ptr = pcVar2 + 1;
              *pcVar2 = '\n';
              lVar7 = lVar7 + -1;
              if (lVar7 == 0) break;
              goto LAB_0010888e;
            }
            __overflow(__stream,10);
          }
          lVar9 = lVar9 + 8;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
        lVar6 = lVar6 + lVar1;
      }
      lVar1 = param_1[1];
      if (lVar1 != 0) {
        lVar9 = local_78 * 8;
        lVar7 = lVar1;
LAB_0010897e:
        do {
          lVar4 = DAT_00126388;
          FUN_0010f510(1);
          pcVar2 = __stream->_IO_write_ptr;
          puVar8 = (undefined8 *)(lVar4 + lVar9);
          if (pcVar2 < __stream->_IO_write_end) {
            __stream->_IO_write_ptr = pcVar2 + 1;
            *pcVar2 = '+';
          }
          else {
            __overflow(__stream,0x2b);
          }
          if ((DAT_00126492 != '\0') && ((DAT_00126491 == '\0' || (*(char *)*puVar8 != '\n')))) {
            pcVar2 = __stream->_IO_write_ptr;
            if (pcVar2 < __stream->_IO_write_end) {
              __stream->_IO_write_ptr = pcVar2 + 1;
              *pcVar2 = '\t';
            }
            else {
              __overflow(__stream,9);
            }
          }
          FUN_0010f330(0,puVar8,1);
          FUN_0010f510(3);
          if (*(char *)(puVar8[1] + -1) == '\n') {
            pcVar2 = __stream->_IO_write_ptr;
            if (pcVar2 < __stream->_IO_write_end) {
              lVar9 = lVar9 + 8;
              __stream->_IO_write_ptr = pcVar2 + 1;
              *pcVar2 = '\n';
              lVar7 = lVar7 + -1;
              if (lVar7 == 0) break;
              goto LAB_0010897e;
            }
            __overflow(__stream,10);
          }
          lVar9 = lVar9 + 8;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
        local_78 = local_78 + lVar1;
      }
      param_1 = (undefined8 *)*param_1;
    }
  }
LAB_001087bc:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_hunk @ 0x8b10

long * find_hunk(long *param_1)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  long lVar6;
  
  lVar2 = param_1[4];
  lVar4 = param_1[3];
  while( true ) {
    plVar5 = param_1;
    param_1 = (long *)*plVar5;
    if (param_1 == (long *)0x0) {
      return plVar5;
    }
    lVar6 = DAT_00126570 * 2 + 1;
    if (*(char *)(param_1 + 5) != '\0') {
      lVar6 = DAT_00126570;
    }
    lVar1 = lVar2 + plVar5[1];
    lVar3 = param_1[3] - (lVar4 + plVar5[2]);
    lVar2 = param_1[4];
    if (lVar3 != lVar2 - lVar1) break;
    lVar4 = param_1[3];
    if (lVar6 <= lVar3) {
      return plVar5;
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: print_context_header @ 0x8b80

void print_context_header(long param_1,undefined8 *param_2,char param_3)

{
  if (param_3 != '\0') {
    FUN_00107cf0(&DAT_0011c0ba,param_1,*param_2,DAT_00126540);
    FUN_00107cf0(&DAT_0011c0d0,param_1 + 0x130,param_2[1],DAT_00126548);
    return;
  }
  FUN_00107cf0(&DAT_0011c0af,param_1,*param_2,DAT_00126540);
  FUN_00107cf0(&DAT_0011c0ba,param_1 + 0x130,param_2[1],DAT_00126548);
  return;
}




// Function: print_context_script @ 0x8c10

void print_context_script(undefined8 *param_1,char param_2)

{
  undefined8 *puVar1;
  int iVar2;
  code *pcVar3;
  undefined8 *puVar4;
  long in_FS_OFFSET;
  undefined1 local_60 [8];
  undefined1 local_58 [8];
  undefined1 local_50 [8];
  undefined1 local_48 [8];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00126554 == '\0') && (puVar4 = param_1, DAT_001264e0 == 0)) {
    for (; puVar4 != (undefined8 *)0x0; puVar4 = (undefined8 *)*puVar4) {
      *(undefined1 *)(puVar4 + 5) = 0;
      puVar4 = (undefined8 *)*puVar4;
      if (puVar4 == (undefined8 *)0x0) break;
      *(undefined1 *)(puVar4 + 5) = 0;
    }
  }
  else if (param_1 != (undefined8 *)0x0) {
    puVar4 = param_1;
    do {
      puVar1 = (undefined8 *)*puVar4;
      *puVar4 = 0;
      iVar2 = FUN_0010f670(puVar4,local_60,local_58,local_50,local_48);
      *puVar4 = puVar1;
      *(bool *)(puVar4 + 5) = iVar2 == 0;
      puVar4 = puVar1;
    } while (puVar1 != (undefined8 *)0x0);
  }
  DAT_00126178 = -DAT_00126288;
  DAT_00126170 = 0x7fffffffffffffff;
  if (param_2 == '\0') {
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_00108d51;
    pcVar3 = FUN_00108070;
  }
  else {
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
LAB_00108d51:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    pcVar3 = FUN_00108590;
  }
  FUN_0010f080(param_1,FUN_00108b10,pcVar3);
  return;
}




// Function: add_regexp @ 0x8d60

void add_regexp(long *param_1,char *param_2)

{
  long lVar1;
  size_t __length;
  char *pcVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  long lVar6;
  
  __length = strlen(param_2);
  pcVar2 = re_compile_pattern(param_2,__length,(re_pattern_buffer *)param_1[4]);
  if (pcVar2 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
    error(2,0,"%s: %s",param_2,pcVar2);
  }
  lVar1 = *param_1;
  lVar6 = param_1[1];
  uVar5 = param_1[2];
  *(bool *)(param_1 + 3) = lVar1 != 0;
  uVar4 = (ulong)((uint)(lVar1 != 0) * 2) + lVar6 + __length;
  param_1[1] = uVar4;
  lVar3 = lVar1;
  if (uVar5 <= uVar4) {
    if (uVar5 == 0) {
      uVar5 = 1;
    }
    do {
      uVar5 = uVar5 * 2;
    } while (uVar5 <= uVar4);
    param_1[2] = uVar5;
    lVar3 = FUN_00116a90(lVar1);
    *param_1 = lVar3;
  }
  if (lVar1 != 0) {
    lVar1 = lVar6 + 1;
    *(undefined1 *)(lVar3 + lVar6) = 0x5c;
    lVar6 = lVar6 + 2;
    *(undefined1 *)(lVar3 + lVar1) = 0x7c;
  }
  memcpy((void *)(lVar3 + lVar6),param_2,__length + 1);
  return;
}




// Function: try_help @ 0x8e40

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00136860;
  if (param_1 != 0) {
    uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = dcgettext(0,"Try \'%s --help\' for more information.",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: compare_files @ 0x8f90

int compare_files(ulong param_1,char *param_2,char *param_3)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  char *pcVar4;
  undefined8 uVar5;
  __off_t _Var6;
  char *pcVar7;
  char *pcVar8;
  undefined8 uVar9;
  long lVar10;
  int iVar11;
  int iVar12;
  int *piVar13;
  stat *psVar14;
  __dev_t *p_Var15;
  uint uVar16;
  timespec *ptVar17;
  long in_FS_OFFSET;
  bool bVar18;
  bool bVar19;
  byte bVar20;
  char *local_2c0;
  int local_2a8 [2];
  char *local_2a0;
  stat local_298 [2];
  int local_178;
  char *local_170;
  __dev_t local_168;
  __ino_t _Stack_160;
  __nlink_t local_158;
  undefined8 uStack_150;
  ulong local_148;
  __dev_t _Stack_140;
  long local_138;
  __blksize_t _Stack_130;
  __blkcnt_t local_128;
  __time_t _Stack_120;
  long local_118;
  long lStack_110;
  long local_108;
  long lStack_100;
  long local_f8;
  long lStack_f0;
  long local_e8;
  long lStack_e0;
  ulong local_48;
  long local_40;
  
  bVar20 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 == (char *)0x0) || (param_3 == (char *)0x0)) {
    if ((DAT_00126585 == '\x01') && (param_3 != (char *)0x0)) {
      local_48 = param_1;
      piVar3 = local_2a8;
      for (lVar10 = 0x4c; lVar10 != 0; lVar10 = lVar10 + -1) {
        piVar3[0] = 0;
        piVar3[1] = 0;
        piVar3 = piVar3 + 2;
      }
      if (param_2 == (char *)0x0) {
        local_2a8[0] = -1;
LAB_00109698:
        local_2a8[0] = -1;
        local_178 = -2;
        if (param_3 == (char *)0x0) {
LAB_00109688:
          local_2a8[0] = -1;
          param_2 = (char *)0x0;
          goto joined_r0x00109070;
        }
      }
      else {
        local_2a8[0] = -2;
        local_178 = 0xfffffffe;
      }
      local_178 = -2;
      param_2 = param_3;
      goto joined_r0x00109070;
    }
    if (DAT_00126586 == '\0') {
      iVar1 = 1;
      pcVar4 = param_2;
      if (param_2 == (char *)0x0) {
        pcVar4 = param_3;
      }
      FUN_0010e660("Only in %s: %s\n",
                   *(undefined8 *)(param_1 + 8 + (ulong)(param_2 == (char *)0x0) * 0x130),pcVar4);
      goto LAB_0010924e;
    }
    local_48 = param_1;
    piVar3 = local_2a8;
    for (lVar10 = 0x4c; lVar10 != 0; lVar10 = lVar10 + -1) {
      piVar3[0] = 0;
      piVar3[1] = 0;
      piVar3 = piVar3 + 2;
    }
    if (param_2 == (char *)0x0) {
      local_2a8[0] = -1;
      if (param_3 != (char *)0x0) goto LAB_00109698;
      local_178 = -1;
      goto LAB_00109688;
    }
    local_2a8[0] = -2;
    if (param_3 == (char *)0x0) {
      local_178 = -1;
      param_3 = param_2;
      goto joined_r0x00109070;
    }
  }
  else {
    local_48 = param_1;
    piVar3 = local_2a8;
    for (lVar10 = 0x4c; lVar10 != 0; lVar10 = lVar10 + -1) {
      piVar3[0] = 0;
      piVar3[1] = 0;
      piVar3 = piVar3 + 2;
    }
    local_2a8[0] = -2;
  }
  local_2a8[0] = -2;
  local_178 = -2;
joined_r0x00109070:
  if (param_1 == 0) {
    local_2a0 = param_2;
    pcVar4 = (char *)0x0;
    local_2c0 = (char *)0x0;
  }
  else {
    pcVar4 = (char *)FUN_00110d80(*(undefined8 *)(param_1 + 8),param_2,0);
    local_2a0 = pcVar4;
    param_3 = (char *)FUN_00110d80(*(undefined8 *)(param_1 + 0x138),param_3,0);
    local_2c0 = param_3;
  }
  local_170 = param_3;
  ptVar17 = &local_298[0].st_mtim;
  bVar18 = false;
  do {
    if (*(int *)((long)(ptVar17 + -7) + 8) != -1) {
      if ((bVar18) && (iVar1 = strcmp(local_170,local_2a0), iVar1 == 0)) {
        uStack_150 = CONCAT44(local_298[0].st_uid,local_298[0].st_mode);
        local_148 = (ulong)local_298[0].st_gid;
        local_168 = local_298[0].st_dev;
        _Stack_160 = local_298[0].st_ino;
        local_178 = local_2a8[0];
        local_158 = local_298[0].st_nlink;
        _Stack_140 = local_298[0].st_rdev;
        local_138 = local_298[0].st_size;
        _Stack_130 = local_298[0].st_blksize;
        local_128 = local_298[0].st_blocks;
        _Stack_120 = local_298[0].st_atim.tv_sec;
        local_118 = local_298[0].st_atim.tv_nsec;
        lStack_110 = local_298[0].st_mtim.tv_sec;
        local_108 = local_298[0].st_mtim.tv_nsec;
        lStack_100 = local_298[0].st_ctim.tv_sec;
        local_f8 = local_298[0].st_ctim.tv_nsec;
        lStack_f0 = local_298[0].__unused[0];
        local_e8 = local_298[0].__unused[1];
        lStack_e0 = local_298[0].__unused[2];
      }
      else {
        pcVar7 = *(char **)(ptVar17 + -6);
        iVar1 = strcmp(pcVar7,"-");
        psVar14 = (stat *)((long)(ptVar17 + -6) + 8);
        if (iVar1 == 0) {
          *(undefined4 *)((long)(ptVar17 + -7) + 8) = 0;
          isatty(0);
          iVar1 = fstat(0,psVar14);
          if (iVar1 != 0) goto LAB_00109390;
          if ((*(uint *)(ptVar17 + -4) & 0xf000) == 0x8000) {
            _Var6 = lseek(0,0,1);
            if (_Var6 < 0) {
              piVar3 = __errno_location();
              *(int *)((long)(ptVar17 + -7) + 8) = -3 - *piVar3;
            }
            else {
              *(__off_t *)((long)(ptVar17 + -3) + 8) =
                   *(__off_t *)((long)(ptVar17 + -3) + 8) - _Var6;
              lVar10 = *(__off_t *)((long)(ptVar17 + -3) + 8);
              if (*(__off_t *)((long)(ptVar17 + -3) + 8) < 0) {
                lVar10 = 0;
              }
              *(long *)((long)(ptVar17 + -3) + 8) = lVar10;
            }
          }
          FUN_00110eb0(ptVar17);
        }
        else {
          if (DAT_00126550 == '\0') {
            iVar1 = stat(pcVar7,psVar14);
          }
          else {
            iVar1 = lstat(pcVar7,psVar14);
          }
          if (iVar1 != 0) {
LAB_00109390:
            piVar3 = __errno_location();
            *(int *)((long)(ptVar17 + -7) + 8) = -3 - *piVar3;
          }
        }
      }
    }
    ptVar17 = ptVar17 + 0x13;
    if (bVar18) break;
    bVar18 = true;
  } while( true );
  iVar1 = 0;
  piVar3 = local_2a8;
  piVar13 = local_2a8;
LAB_001091a0:
  iVar1 = iVar1 + 1;
  if (DAT_00126586 == '\0') {
    if (iVar1 != 1) goto LAB_001091b7;
    if (DAT_00126585 != '\0') goto LAB_001092e0;
    goto LAB_0010932a;
  }
LAB_001092e0:
  iVar12 = *piVar3;
  if (iVar12 == -2) {
    if ((((piVar3[10] & 0xf000U) == 0x8000) && ((piVar3[10] & 0x1ffU) == 0)) &&
       (*(long *)(piVar3 + 0x10) == 0)) goto LAB_0010931a;
  }
  else if (((iVar12 == -5 || iVar12 == -0xc) && (param_1 == 0)) &&
          ((piVar13[0x4c] + 2U & 0xfffffffd) == 0)) {
LAB_0010931a:
    *piVar3 = -1;
    if (iVar1 != 2) goto LAB_0010932a;
LAB_001091c1:
    iVar12 = local_178;
    if (local_2a8[0] != -1) {
      iVar11 = -3 - local_2a8[0];
      if (local_178 == -1) {
        p_Var15 = &local_168;
        for (lVar10 = 0x12; lVar10 != 0; lVar10 = lVar10 + -1) {
          *p_Var15 = 0;
          p_Var15 = p_Var15 + (ulong)bVar20 * -2 + 1;
        }
        uStack_150 = CONCAT44(uStack_150._4_4_,local_298[0].st_mode);
        if (iVar11 < 0) goto LAB_001093e5;
      }
      else if (iVar11 < 0) goto LAB_001093d7;
      piVar3 = __errno_location();
      *piVar3 = iVar11;
      FUN_0010e490(local_2a0);
      iVar12 = -3 - local_178;
      if (-1 < iVar12) {
LAB_00109219:
        *piVar3 = iVar12;
        FUN_0010e490(local_170);
      }
      goto LAB_00109228;
    }
    uVar2 = (uint)uStack_150;
    psVar14 = local_298;
    for (lVar10 = 0x12; lVar10 != 0; lVar10 = lVar10 + -1) {
      psVar14->st_dev = 0;
      psVar14 = (stat *)((long)psVar14 + ((ulong)bVar20 * -2 + 1) * 8);
    }
    local_298[0].st_mode = uVar2;
    if (iVar12 == -1) {
      p_Var15 = &local_168;
      for (lVar10 = 0x12; lVar10 != 0; lVar10 = lVar10 + -1) {
        *p_Var15 = 0;
        p_Var15 = p_Var15 + (ulong)bVar20 * -2 + 1;
      }
      uStack_150 = CONCAT44(uStack_150._4_4_,uVar2);
    }
    else {
LAB_001093d7:
      iVar12 = -3 - iVar12;
      if (-1 < iVar12) {
        piVar3 = __errno_location();
        goto LAB_00109219;
      }
    }
LAB_001093e5:
    if (param_1 == 0) {
      bVar18 = (local_298[0].st_mode & 0xf000) == 0x4000;
      if (bVar18 == (((uint)uStack_150 & 0xf000) == 0x4000)) goto LAB_001094c7;
      lVar10 = (long)(int)(uint)!bVar18;
      pcVar7 = (&local_2a0)[(ulong)bVar18 * 0x26];
      uVar9 = FUN_0010fb40(pcVar7);
      pcVar4 = (char *)FUN_0010a660((&local_2a0)[lVar10 * 0x26],uVar9);
      (&local_2a0)[lVar10 * 0x26] = pcVar4;
      iVar12 = strcmp(pcVar7,"-");
      if (iVar12 != 0) {
        psVar14 = (stat *)((long)local_298 + lVar10 * 0x130);
        if (DAT_00126550 == '\0') {
          iVar12 = stat(pcVar4,psVar14);
        }
        else {
          iVar12 = lstat(pcVar4,psVar14);
        }
        if (iVar12 != 0) {
          FUN_0010e490(pcVar4);
          goto LAB_00109228;
        }
        goto LAB_001094c7;
      }
      FUN_0010e7b0("cannot compare \'-\' to a directory");
LAB_00109da2:
      FUN_0010e7b0("-D option not supported with directories");
LAB_00109dae:
      iVar1 = (int)param_1;
LAB_00109228:
      iVar12 = fflush_unlocked(stdout);
      if (iVar12 != 0) {
        uVar9 = dcgettext(0,"standard output",5);
                    /* WARNING: Subroutine does not return */
        FUN_0010e770(uVar9);
      }
    }
    else {
LAB_001094c7:
      if (local_2a8[0] == -1) {
        if (local_178 != -1) {
          bVar18 = false;
          uVar2 = local_298[0].st_mode & 0xf000;
          goto LAB_001096ca;
        }
      }
      else {
        if (local_178 == -1) {
LAB_001096c0:
          bVar18 = false;
          uVar2 = local_298[0].st_mode & 0xf000;
LAB_001096ca:
          bVar19 = ((uint)uStack_150 & 0xf000) != 0x4000;
          if (uVar2 == 0x4000) {
            if (bVar19) goto LAB_00109931;
            if (DAT_00126580 == 7) goto LAB_00109da2;
            if ((param_1 != 0) && (DAT_00126608 == '\0')) {
              FUN_0010e660("Common subdirectories: %s and %s\n",local_2a0,local_170);
              goto LAB_00109730;
            }
LAB_00109717:
            uVar2 = FUN_0010a230(local_2a8,compare_files);
            param_1 = (ulong)uVar2;
            goto LAB_00109728;
          }
          if (bVar19) goto LAB_0010955c;
LAB_00109931:
          if (local_2a8[0] != -1) goto LAB_00109940;
LAB_00109949:
          if ((DAT_00126608 != '\0') &&
             ((DAT_00126586 != '\0' || ((DAT_00126585 != '\0' && (local_2a8[0] == -1))))))
          goto LAB_00109717;
          if (param_1 == 0) {
                    /* WARNING: Subroutine does not return */
            __assert_fail("parent","diff.c",0x52d,"compare_files");
          }
LAB_00109598:
          iVar1 = 1;
          FUN_0010e660("Only in %s: %s\n",
                       *(undefined8 *)(param_1 + 8 + (ulong)(local_2a8[0] == -1) * 0x130),param_2);
          goto LAB_00109228;
        }
        if ((local_298[0].st_ino == _Stack_160) && (local_298[0].st_dev == local_168)) {
LAB_0010984b:
          if (local_298[0].st_mode == (uint)uStack_150) {
            if (((((local_298[0].st_nlink == local_158) && (local_298[0].st_uid == uStack_150._4_4_)
                  ) && (local_298[0].st_gid == (uint)local_148)) &&
                ((local_298[0].st_size == local_138 && (local_298[0].st_mtim.tv_sec == lStack_110)))
                ) && (local_298[0].st_ctim.tv_sec == lStack_100)) {
              if (DAT_00126578 != '\0') goto LAB_00109730;
              bVar18 = true;
              uVar2 = (uint)uStack_150 & 0xf000;
            }
            else {
              bVar18 = false;
              uVar2 = (uint)uStack_150 & 0xf000;
            }
            goto LAB_001096ca;
          }
          goto LAB_001096c0;
        }
        uVar2 = local_298[0].st_mode & 0xf000;
        uVar16 = (uint)uStack_150 & 0xf000;
        if (uVar2 == 0x6000) {
          if (uVar16 == 0x6000) {
LAB_0010990d:
            if (_Stack_140 == local_298[0].st_rdev) goto LAB_0010984b;
          }
        }
        else {
          bVar18 = false;
          if (uVar2 != 0x2000) goto LAB_001096ca;
          if (uVar16 == 0x2000) goto LAB_0010990d;
        }
        if ((uVar2 == 0x4000) || (bVar18 = false, uVar16 == 0x4000)) {
LAB_00109940:
          if (local_178 == -1) goto LAB_00109949;
LAB_001098a0:
          iVar1 = 1;
          uVar9 = FUN_00110c90(&local_168);
          pcVar7 = DAT_00126548;
          if (DAT_00126548 == (char *)0x0) {
            pcVar7 = local_170;
          }
          uVar5 = FUN_00110c90(local_298);
          pcVar8 = DAT_00126540;
          if (DAT_00126540 == (char *)0x0) {
            pcVar8 = local_2a0;
          }
          FUN_0010e4c0("File %s is a %s while file %s is a %s\n",pcVar8,uVar5,pcVar7,uVar9);
          goto LAB_00109228;
        }
LAB_0010955c:
        uVar16 = (uint)uStack_150 & 0xf000;
        if ((param_1 != 0) &&
           (((uVar2 & 0xffffdfff) != 0x8000 || (((uint)uStack_150 & 0xd000) != 0x8000)))) {
          if ((local_2a8[0] == -1) || (local_178 == -1)) goto LAB_00109598;
          goto LAB_001098a0;
        }
        if ((uVar2 == 0xa000) || (uVar16 == 0xa000)) {
          if (DAT_00126550 == '\0') {
                    /* WARNING: Subroutine does not return */
            __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files");
          }
          if ((uVar2 != 0xa000) || (uVar16 != 0xa000)) goto LAB_001098a0;
          pcVar7 = (char *)FUN_00116f20(local_2a0);
          if (pcVar7 == (char *)0x0) {
            iVar12 = 0;
          }
          else {
            pcVar8 = (char *)FUN_00116f20(local_170);
            iVar12 = 1;
            if (pcVar8 != (char *)0x0) {
              iVar1 = strcmp(pcVar7,pcVar8);
              if (iVar1 == 0) {
                free(pcVar7);
                free(pcVar8);
                goto LAB_00109730;
              }
              iVar1 = 1;
              FUN_0010e660("Symbolic links %s and %s differ\n",local_2a0,local_170);
              free(pcVar7);
              free(pcVar8);
              goto LAB_00109228;
            }
          }
          FUN_0010e490((&local_2a0)[(long)iVar12 * 0x26]);
          free(pcVar7);
          free((void *)0x0);
          goto LAB_00109228;
        }
        if (((((uVar2 == 0x8000 & DAT_00126553) != 0) && (uVar16 == 0x8000)) &&
            (local_298[0].st_size != local_138 && 0 < local_298[0].st_size)) && (0 < local_138)) {
          pcVar7 = DAT_00126548;
          if (DAT_00126548 == (char *)0x0) {
            pcVar7 = local_170;
          }
          pcVar8 = DAT_00126540;
          if (DAT_00126540 == (char *)0x0) {
            pcVar8 = local_2a0;
          }
          iVar1 = 1;
          FUN_0010e660("Files %s and %s differ\n",pcVar8,pcVar7);
          goto LAB_00109228;
        }
        if ((local_2a8[0] == -2) && (local_2a8[0] = open(local_2a0,0,0), local_2a8[0] < 0)) {
          FUN_0010e490(local_2a0);
          if (local_178 == -2) {
            bVar19 = true;
            goto LAB_00109a17;
          }
LAB_00109a32:
          if (-1 < local_2a8[0]) {
            param_1 = 2;
            goto LAB_00109b75;
          }
          if (local_178 < 0) goto LAB_00109228;
LAB_00109a4d:
          iVar12 = close(local_178);
          if (iVar12 != 0) goto LAB_00109a5a;
          goto LAB_00109228;
        }
        bVar19 = false;
        if (local_178 == -2) {
LAB_00109a17:
          if (bVar18) {
            local_178 = local_2a8[0];
          }
          else {
            local_178 = open(local_170,0,0);
            if (local_178 < 0) {
              FUN_0010e490(local_170);
              goto LAB_00109a32;
            }
          }
          if (bVar19) goto LAB_00109a32;
        }
        uVar2 = FUN_00106e20(local_2a8);
        param_1 = (ulong)uVar2;
        if (-1 < local_2a8[0]) {
LAB_00109b75:
          iVar12 = close(local_2a8[0]);
          if (iVar12 != 0) {
            FUN_0010e490(local_2a0);
            if ((local_178 < 0) || (local_2a8[0] == local_178)) goto LAB_00109228;
            goto LAB_00109a4d;
          }
        }
        if (((-1 < local_178) && (local_178 != local_2a8[0])) &&
           (iVar12 = close(local_178), iVar12 != 0)) {
LAB_00109a5a:
          FUN_0010e490(local_170);
          goto LAB_00109228;
        }
LAB_00109728:
        if ((int)param_1 != 0) goto LAB_00109dae;
      }
LAB_00109730:
      iVar1 = 0;
      if ((DAT_00126584 != '\0') && ((local_298[0].st_mode & 0xf000) != 0x4000)) {
        pcVar7 = DAT_00126548;
        if (DAT_00126548 == (char *)0x0) {
          pcVar7 = local_170;
        }
        pcVar8 = DAT_00126540;
        if (DAT_00126540 == (char *)0x0) {
          pcVar8 = local_2a0;
        }
        FUN_0010e660("Files %s and %s are identical\n",pcVar8,pcVar7);
      }
    }
    free(pcVar4);
    free(local_2c0);
LAB_0010924e:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return iVar1;
  }
LAB_001091b7:
  if (iVar1 == 2) goto LAB_001091c1;
LAB_0010932a:
  piVar3 = piVar3 + 0x4c;
  piVar13 = piVar13 + -0x4c;
  goto LAB_001091a0;
}




// Function: dir_loop @ 0x9e40

undefined8 dir_loop(long param_1,int param_2)

{
  long lVar1;
  long lVar2;
  uint uVar3;
  
  lVar1 = *(long *)(param_1 + 0x260);
  if (lVar1 == 0) {
    return 0;
  }
  lVar2 = (long)param_2 * 0x130;
  param_1 = param_1 + lVar2;
  do {
    if ((*(long *)(lVar1 + lVar2 + 0x18) == *(long *)(param_1 + 0x18)) &&
       (*(long *)(lVar1 + lVar2 + 0x10) == *(long *)(param_1 + 0x10))) {
      return 1;
    }
    uVar3 = *(uint *)(lVar1 + 0x28 + lVar2) & 0xf000;
    if (uVar3 == 0x6000) {
      if ((*(uint *)(param_1 + 0x28) & 0xf000) != 0x6000) goto LAB_00109e8a;
      if (*(long *)(lVar1 + 0x38 + lVar2) == *(long *)(param_1 + 0x38)) {
        return 1;
      }
LAB_00109ecc:
      lVar1 = *(long *)(lVar1 + 0x260);
    }
    else {
      if ((uVar3 == 0x2000) && ((*(uint *)(param_1 + 0x28) & 0xf000) == 0x2000)) {
        if (*(long *)(lVar1 + 0x38 + lVar2) == *(long *)(param_1 + 0x38)) {
          return 1;
        }
        goto LAB_00109ecc;
      }
LAB_00109e8a:
      lVar1 = *(long *)(lVar1 + 0x260);
    }
    if (lVar1 == 0) {
      return 0;
    }
  } while( true );
}




// Function: compare_names @ 0x9f10

void compare_names(char *param_1,char *param_2)

{
  int iVar1;
  int *piVar2;
  bool bVar3;
  
  if (DAT_001266e8 != '\0') {
    piVar2 = __errno_location();
    bVar3 = DAT_00126551 != '\0';
    *piVar2 = 0;
    if (bVar3) {
      strcasecmp(param_1,param_2);
      return;
    }
    iVar1 = strcoll(param_1,param_2);
    if (iVar1 != 0) {
      return;
    }
  }
  strcmp(param_1,param_2);
  return;
}




// Function: dir_read @ 0x9f90

undefined8 dir_read(int *param_1,ulong *param_2)

{
  char *__s;
  char cVar1;
  int iVar2;
  DIR *__dirp;
  char *__s_00;
  int *piVar3;
  dirent *pdVar4;
  size_t sVar5;
  undefined8 *puVar6;
  ulong uVar7;
  undefined8 *puVar8;
  ulong uVar9;
  ulong local_48;
  ulong local_40;
  
  iVar2 = *param_1;
  param_2[1] = 0;
  param_2[2] = 0;
  if (iVar2 == -1) {
    puVar6 = (undefined8 *)FUN_00116a40(8);
    *param_2 = 0;
    param_2[1] = (ulong)puVar6;
LAB_0010a179:
    *puVar6 = 0;
    return 1;
  }
  __dirp = opendir(*(char **)(param_1 + 2));
  if (__dirp != (DIR *)0x0) {
    uVar9 = 0x200;
    __s_00 = (char *)FUN_00116a40(0x200);
    param_2[2] = (ulong)__s_00;
    piVar3 = __errno_location();
    local_48 = 0;
    local_40 = 0;
    while( true ) {
      *piVar3 = 0;
      pdVar4 = readdir(__dirp);
      if (pdVar4 == (dirent *)0x0) break;
      __s = pdVar4->d_name;
      sVar5 = strlen(__s);
      if (((pdVar4->d_name[0] != '.') ||
          ((pdVar4->d_name[1] != '\0' && ((pdVar4->d_name[1] != '.' || (pdVar4->d_name[2] != '\0')))
           ))) && (cVar1 = FUN_001104d0(DAT_00126410,__s), cVar1 == '\0')) {
        uVar7 = local_48 + sVar5 + 1;
        while (uVar9 < uVar7) {
          if (0x3ffffffffffffffe < uVar9) goto LAB_0010a1af;
          uVar9 = uVar9 * 2;
          __s_00 = (char *)FUN_00116a90(__s_00,uVar9);
          param_2[2] = (ulong)__s_00;
        }
        memcpy(__s_00 + local_48,__s,sVar5 + 1);
        local_40 = local_40 + 1;
        local_48 = uVar7;
      }
    }
    iVar2 = *piVar3;
    if (iVar2 == 0) {
      iVar2 = closedir(__dirp);
      if (iVar2 == 0) {
        if (0xffffffffffffffd < local_40) {
LAB_0010a1af:
                    /* WARNING: Subroutine does not return */
          FUN_00116ec0();
        }
        puVar6 = (undefined8 *)FUN_00116a40(local_40 * 8 + 8);
        *param_2 = local_40;
        param_2[1] = (ulong)puVar6;
        if (local_40 != 0) {
          puVar8 = puVar6 + local_40;
          do {
            *puVar6 = __s_00;
            puVar6 = puVar6 + 1;
            sVar5 = strlen(__s_00);
            __s_00 = __s_00 + sVar5 + 1;
          } while (puVar6 != puVar8);
        }
        goto LAB_0010a179;
      }
    }
    else {
      closedir(__dirp);
      *piVar3 = iVar2;
    }
  }
  return 0;
}




// Function: compare_names_for_qsort @ 0xa1c0

void compare_names_for_qsort(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  int *piVar2;
  bool bVar3;
  
  __s1 = (char *)*param_1;
  __s2 = (char *)*param_2;
  if (DAT_001266e8 != '\0') {
    piVar2 = __errno_location();
    bVar3 = DAT_00126551 == '\0';
    *piVar2 = 0;
    if (bVar3) {
      iVar1 = strcoll(__s1,__s2);
    }
    else {
      iVar1 = strcasecmp(__s1,__s2);
    }
    if (iVar1 != 0) {
      return;
    }
  }
  strcmp(__s1,__s2);
  return;
}




// Function: diff_dirs @ 0xa230

int diff_dirs(int *param_1,code *param_2)

{
  undefined8 uVar1;
  long *__src;
  char *__s2;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  long lVar6;
  char *__s1;
  long lVar7;
  long *plVar8;
  long in_FS_OFFSET;
  int local_8c;
  long *local_88 [2];
  size_t local_78;
  long *local_70;
  void *local_68;
  size_t local_60;
  long *local_58;
  void *local_50;
  long local_40;
  
  iVar3 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_8c = 0;
  if (((iVar3 == -1) || (cVar2 = FUN_00109e40(param_1,0), cVar2 != '\0')) &&
     ((param_1[0x4c] == -1 || (cVar2 = FUN_00109e40(param_1,1), cVar2 != '\0')))) {
    uVar1 = *(undefined8 *)(param_1 + (ulong)(iVar3 == -1) * 0x4c + 2);
    uVar4 = dcgettext(0,"%s: recursive directory loop",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar4,uVar1);
  }
  cVar2 = FUN_00109f90(param_1,&local_78);
  if (cVar2 == '\0') {
    FUN_0010e490(*(undefined8 *)(param_1 + 2));
    local_8c = 2;
  }
  cVar2 = FUN_00109f90(param_1 + 0x4c,&local_60);
  if (cVar2 == '\0') {
    FUN_0010e490(*(undefined8 *)(param_1 + 0x4e));
    local_8c = 2;
  }
  else if (local_8c == 0) {
    DAT_001266e8 = 1;
    local_88[0] = local_70;
    local_88[1] = local_58;
    iVar3 = _setjmp((__jmp_buf_tag *)&DAT_00126620);
    if (iVar3 != 0) {
      DAT_001266e8 = 0;
    }
    qsort(local_88[0],local_78,8,FUN_0010a1c0);
    qsort(local_88[1],local_60,8,FUN_0010a1c0);
    lVar6 = DAT_00126488;
    if ((DAT_00126488 != 0) && (*(long *)(param_1 + 0x98) == 0)) {
      while ((*local_88[0] != 0 && (iVar3 = FUN_00109f10(*local_88[0],lVar6), iVar3 < 0))) {
        local_88[0] = local_88[0] + 1;
      }
      while ((*local_88[1] != 0 && (iVar3 = FUN_00109f10(*local_88[1],lVar6), iVar3 < 0))) {
        local_88[1] = local_88[1] + 1;
      }
    }
    lVar6 = 0;
    while ((*local_88[0] != 0 || (*local_88[1] != 0))) {
      if (*local_88[0] == 0) {
        lVar5 = *local_88[1];
        lVar7 = 0;
        local_88[1] = local_88[1] + 1;
      }
      else {
        lVar5 = lVar6;
        if (*local_88[1] != 0) {
          iVar3 = FUN_00109f10(*local_88[0],*local_88[1]);
          if (iVar3 == 0) {
            if ((DAT_00126551 != '\0') &&
               (iVar3 = strcmp((char *)*local_88[0],(char *)*local_88[1]), iVar3 != 0)) {
              __src = local_88[(iVar3 >> 0x1f) + 1];
              __s1 = (char *)__src[1];
              __s2 = (char *)*local_88[(uint)-(iVar3 >> 0x1f)];
              plVar8 = __src + 1;
              if (__s1 != (char *)0x0) {
                while (iVar3 = FUN_00109f10(__s1,__s2), iVar3 == 0) {
                  iVar3 = strcmp(__s1,__s2);
                  if (-1 < iVar3) {
                    if (iVar3 == 0) {
                      memmove(__src + 1,__src,(long)plVar8 - (long)__src);
                      *__src = (long)__s2;
                    }
                    break;
                  }
                  __s1 = (char *)plVar8[1];
                  plVar8 = plVar8 + 1;
                  if (__s1 == (char *)0x0) break;
                }
              }
            }
            lVar5 = *local_88[1];
            local_88[1] = local_88[1] + 1;
          }
          else if (-1 < iVar3) {
            lVar5 = *local_88[1];
            lVar7 = lVar6;
            local_88[1] = local_88[1] + 1;
            goto LAB_0010a465;
          }
        }
        lVar7 = *local_88[0];
        local_88[0] = local_88[0] + 1;
      }
LAB_0010a465:
      iVar3 = (*param_2)(param_1,lVar7,lVar5);
      if (local_8c < iVar3) {
        local_8c = iVar3;
      }
    }
  }
  free(local_70);
  free(local_68);
  free(local_58);
  free(local_50);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_8c;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_dir_file_pathname @ 0xa660

undefined8 find_dir_file_pathname(undefined8 param_1,char *param_2)

{
  undefined8 *puVar1;
  char *__s1;
  char *pcVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  char *local_190;
  undefined1 local_178 [8];
  undefined8 *local_170;
  void *local_168;
  undefined4 local_158 [2];
  undefined8 local_150;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_170 = (undefined8 *)0x0;
  local_168 = (void *)0x0;
  local_190 = param_2;
  if (DAT_00126551 != '\0') {
    local_158[0] = 0;
    local_150 = param_1;
    cVar3 = FUN_00109f90(local_158,local_178);
    local_190 = param_2;
    if (cVar3 != '\0') {
      DAT_001266e8 = 1;
      iVar4 = _setjmp((__jmp_buf_tag *)&DAT_00126620);
      local_190 = param_2;
      if (iVar4 == 0) {
        __s1 = (char *)*local_170;
        pcVar2 = param_2;
        puVar6 = local_170;
        while (local_190 = pcVar2, __s1 != (char *)0x0) {
          iVar4 = FUN_00109f10(__s1,param_2);
          if (iVar4 == 0) {
            iVar4 = strcmp(__s1,param_2);
            local_190 = __s1;
            if (iVar4 == 0) break;
            if (param_2 != pcVar2) {
              local_190 = pcVar2;
            }
          }
          puVar1 = puVar6 + 1;
          puVar6 = puVar6 + 1;
          pcVar2 = local_190;
          __s1 = (char *)*puVar1;
        }
      }
    }
  }
  uVar5 = FUN_00110d80(param_1,local_190,0);
  free(local_170);
  free(local_168);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ed_hunk @ 0xa7c0

void print_ed_hunk(undefined8 param_1)

{
  long lVar1;
  byte bVar2;
  byte *pbVar3;
  char *pcVar4;
  uint uVar5;
  long lVar6;
  long in_FS_OFFSET;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar5 = FUN_0010f670(param_1,&local_50,&local_48,&local_40,&local_38);
  if (uVar5 != 0) {
    FUN_0010e830();
    FUN_0010f610(0x2c,&DAT_001261a0,local_50,local_48);
    bVar2 = (&DAT_0011e550)[uVar5];
    pbVar3 = (byte *)DAT_00126188->_IO_write_ptr;
    if (pbVar3 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = (char *)(pbVar3 + 1);
      *pbVar3 = bVar2;
    }
    else {
      __overflow(DAT_00126188,(uint)bVar2);
    }
    pcVar4 = DAT_00126188->_IO_write_ptr;
    if (pcVar4 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = pcVar4 + 1;
      *pcVar4 = '\n';
    }
    else {
      __overflow(DAT_00126188,10);
    }
    if (uVar5 != 1) {
      lVar6 = local_40;
      if (local_40 <= local_38) {
        do {
          while( true ) {
            lVar1 = lVar6 * 8;
            lVar6 = lVar6 + 1;
            pcVar4 = *(char **)(DAT_00126388 + lVar1);
            if ((*pcVar4 != '.') || (pcVar4[1] != '\n')) break;
            fputs_unlocked("..\n.\ns/.//\n",DAT_00126188);
            if (local_38 < lVar6) goto LAB_0010a7fb;
            fputs_unlocked("a\n",DAT_00126188);
          }
          FUN_0010f500(&DAT_0011e5b0);
        } while (lVar6 <= local_38);
      }
      fputs_unlocked(".\n",DAT_00126188);
    }
  }
LAB_0010a7fb:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pr_forward_ed_hunk @ 0xa960

void pr_forward_ed_hunk(undefined8 param_1)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  uint uVar4;
  long lVar5;
  long in_FS_OFFSET;
  undefined8 local_40;
  undefined8 local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = FUN_0010f670(param_1,&local_40,&local_38,&local_30,&local_28);
  if (uVar4 != 0) {
    FUN_0010e830();
    bVar1 = (&DAT_0011e550)[uVar4];
    pbVar2 = (byte *)DAT_00126188->_IO_write_ptr;
    if (pbVar2 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = (char *)(pbVar2 + 1);
      *pbVar2 = bVar1;
    }
    else {
      __overflow(DAT_00126188,(uint)bVar1);
    }
    FUN_0010f610(0x20,&DAT_001261a0,local_40,local_38);
    pcVar3 = DAT_00126188->_IO_write_ptr;
    if (pcVar3 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = pcVar3 + 1;
      *pcVar3 = '\n';
    }
    else {
      __overflow(DAT_00126188,10);
    }
    if (uVar4 != 1) {
      if (local_30 <= local_28) {
        do {
          lVar5 = local_30 + 1;
          FUN_0010f500(&DAT_0011e5b0,DAT_00126388 + local_30 * 8);
          local_30 = lVar5;
        } while (lVar5 <= local_28);
      }
      fputs_unlocked(".\n",DAT_00126188);
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_rcs_hunk @ 0xaab0

void print_rcs_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined8 local_60;
  undefined8 local_58;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f670(param_1,&local_60,&local_58,&local_50,&local_48);
  if (uVar1 != 0) {
    FUN_0010e830();
    FUN_0010f5e0(&DAT_001261a0,local_60,local_58,&local_40,&local_38);
    if ((uVar1 & 1) != 0) {
      lVar2 = 1;
      if (local_40 <= local_38) {
        lVar2 = (local_38 - local_40) + 1;
      }
      __fprintf_chk(DAT_00126188,1,"d%ld %ld\n",local_40,lVar2);
    }
    if ((uVar1 & 2) != 0) {
      FUN_0010f5e0(&DAT_001262d0,local_50,local_48,&local_30,&local_28);
      lVar2 = 1;
      if (local_30 <= local_28) {
        lVar2 = (local_28 - local_30) + 1;
      }
      __fprintf_chk(DAT_00126188,1,"a%ld %ld\n",local_38,lVar2);
      if (local_50 <= local_48) {
        do {
          lVar2 = local_50 + 1;
          FUN_0010f500(&DAT_0011e5b0,DAT_00126388 + local_50 * 8);
          local_50 = lVar2;
        } while (lVar2 <= local_48);
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_ed_script @ 0xac20

void print_ed_script(undefined8 param_1)

{
  FUN_0010f080(param_1,FUN_0010f070,FUN_0010a7c0);
  return;
}




// Function: pr_forward_ed_script @ 0xac40

void pr_forward_ed_script(undefined8 param_1)

{
  FUN_0010f080(param_1,FUN_0010f060,FUN_0010a960);
  return;
}




// Function: print_rcs_script @ 0xac60

void print_rcs_script(undefined8 param_1)

{
  FUN_0010f080(param_1,FUN_0010f060,FUN_0010aab0);
  return;
}




// Function: groups_letter_value @ 0xac80

long groups_letter_value(undefined8 *param_1,undefined1 param_2)

{
  long lVar1;
  
  switch(param_2) {
  case 0x45:
    param_1 = param_1 + 3;
switchD_0010acc7_caseD_65:
    lVar1 = FUN_0010f5d0(*param_1,param_1[1]);
    return lVar1 + -1;
  case 0x46:
    param_1 = param_1 + 3;
switchD_0010acc7_caseD_66:
    lVar1 = FUN_0010f5d0(*param_1,param_1[1]);
    return lVar1;
  default:
    switch(param_2) {
    case 0x65:
      goto switchD_0010acc7_caseD_65;
    case 0x66:
      goto switchD_0010acc7_caseD_66;
    default:
      return -1;
    case 0x6c:
      goto switchD_0010acc7_caseD_6c;
    case 0x6d:
      goto switchD_0010acc7_caseD_6d;
    case 0x6e:
      goto switchD_0010acc7_caseD_6e;
    }
  case 0x4c:
    param_1 = param_1 + 3;
switchD_0010acc7_caseD_6c:
    lVar1 = FUN_0010f5d0(*param_1,param_1[2]);
    return lVar1 + -1;
  case 0x4d:
    param_1 = param_1 + 3;
switchD_0010acc7_caseD_6d:
    lVar1 = FUN_0010f5d0(*param_1,param_1[2]);
    return lVar1;
  case 0x4e:
    param_1 = param_1 + 3;
switchD_0010acc7_caseD_6e:
    return param_1[2] - param_1[1];
  }
}




// Function: do_printf_spec @ 0xad70

byte * do_printf_spec(_IO_FILE *param_1,void *param_2,long param_3,undefined8 param_4,
                   undefined8 param_5)

{
  byte bVar1;
  char *pcVar2;
  byte bVar3;
  uint uVar4;
  long lVar5;
  byte *pbVar6;
  byte *pbVar7;
  void *__dest;
  undefined1 *puVar8;
  undefined1 *puVar9;
  ulong uVar11;
  byte bVar12;
  byte *pbVar13;
  long in_FS_OFFSET;
  undefined1 auStack_68 [8];
  byte *local_60;
  byte *local_58;
  byte *local_50;
  long local_40;
  undefined1 *puVar10;
  
  pbVar7 = (byte *)((long)param_2 + 1);
  puVar8 = auStack_68;
  puVar9 = auStack_68;
  puVar10 = auStack_68;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    bVar12 = *pbVar7;
    uVar4 = (uint)bVar12;
    pbVar7 = pbVar7 + 1;
    if (0x30 < bVar12) {
      if (9 < (int)(char)bVar12 - 0x30U) goto LAB_0010add0;
      goto LAB_0010ae30;
    }
  } while ((0xfffedf7fffffffffU >> ((ulong)bVar12 & 0x3f) & 1) == 0);
  if ((int)(char)bVar12 - 0x30U < 10) {
LAB_0010ae30:
    do {
      bVar12 = *pbVar7;
      uVar4 = (uint)(char)bVar12;
      pbVar7 = pbVar7 + 1;
    } while (uVar4 - 0x30 < 10);
    if (bVar12 != 0x2e) goto LAB_0010add0;
LAB_0010ae48:
    do {
      bVar12 = *pbVar7;
      pbVar7 = pbVar7 + 1;
    } while ((int)(char)bVar12 - 0x30U < 10);
    uVar4 = (int)(char)bVar12 - 0x58;
    bVar1 = *pbVar7;
    bVar3 = (byte)uVar4;
  }
  else {
    if (bVar12 == 0x2e) goto LAB_0010ae48;
LAB_0010add0:
    bVar12 = (byte)uVar4;
    uVar4 = uVar4 - 0x58;
    bVar1 = *pbVar7;
    bVar3 = (byte)uVar4;
  }
  if (bVar3 < 0x21) {
    if ((0x100801001U >> ((ulong)uVar4 & 0x3f) & 1) == 0) {
      pbVar13 = (byte *)0x0;
      puVar8 = auStack_68;
      if ((char)uVar4 != '\v') goto LAB_0010adfb;
      if (bVar1 == 0x27) {
        bVar12 = pbVar7[1];
        uVar4 = (uint)bVar12;
        puVar8 = auStack_68;
        if (bVar12 == 0x27) goto LAB_0010adfb;
        if (bVar12 == 0x5c) {
          bVar12 = pbVar7[2];
          pbVar13 = pbVar7 + 3;
          if (bVar12 == 0x27) {
            uVar4 = 0;
            pbVar6 = pbVar7 + 2;
          }
          else {
            uVar4 = 0;
            do {
              pbVar6 = pbVar13;
              if (7 < (int)(char)bVar12 - 0x30U) goto LAB_0010ae70;
              pbVar13 = pbVar6 + 1;
              uVar4 = ((int)(char)bVar12 - 0x30U) + uVar4 * 8;
              bVar12 = *pbVar6;
            } while (bVar12 != 0x27);
          }
          if (pbVar6 + (-3 - (long)pbVar7) < (byte *)0x3) {
LAB_0010aea2:
            puVar8 = auStack_68;
            if (param_1 != (_IO_FILE *)0x0) {
              pcVar2 = param_1->_IO_write_ptr;
              if (pcVar2 < param_1->_IO_write_end) {
                param_1->_IO_write_ptr = pcVar2 + 1;
                *pcVar2 = (char)uVar4;
                puVar8 = auStack_68;
              }
              else {
                __overflow(param_1,uVar4 & 0xff);
                puVar8 = auStack_68;
              }
            }
            goto LAB_0010adfb;
          }
        }
        else {
          puVar8 = auStack_68;
          if (bVar12 == 0) goto LAB_0010adfb;
          pbVar13 = pbVar7 + 3;
          if (pbVar7[2] == 0x27) goto LAB_0010aea2;
        }
      }
    }
    else if (param_3 == 0) {
      lVar5 = FUN_0010ac80(param_5,(int)(char)bVar1);
      if (-1 < lVar5) goto LAB_0010aee9;
    }
    else if (bVar1 == 0x6e) {
      lVar5 = FUN_0010f5d0(param_3,param_4);
LAB_0010aee9:
      pbVar13 = pbVar7 + 1;
      if (param_1 != (_IO_FILE *)0x0) {
        local_60 = pbVar13 + (-2 - (long)param_2);
        local_50 = pbVar13 + (-1 - (long)param_2);
        local_58 = (byte *)lVar5;
        if (pbVar13 + (1 - (long)param_2) < (byte *)0xfa1) {
          puVar8 = auStack_68;
          while (puVar10 !=
                 auStack_68 + -((ulong)(pbVar13 + (0x37 - (long)param_2)) & 0xfffffffffffff000)) {
            puVar9 = puVar8 + -0x1000;
            *(undefined8 *)(puVar8 + -8) = *(undefined8 *)(puVar8 + -8);
            puVar10 = puVar8 + -0x1000;
            puVar8 = puVar8 + -0x1000;
          }
          uVar11 = (ulong)((uint)(pbVar13 + (0x37 - (long)param_2)) & 0xff0);
          lVar5 = -uVar11;
          if (uVar11 != 0) {
            *(undefined8 *)(puVar9 + -8) = *(undefined8 *)(puVar9 + -8);
          }
          __dest = (void *)(((ulong)(puVar9 + lVar5 + 0xf) & 0xfffffffffffffff0) + 0x1f &
                           0xffffffffffffffe0);
          puVar8 = puVar9 + lVar5;
        }
        else {
          __dest = (void *)FUN_00116f00();
          puVar8 = auStack_68;
        }
        pbVar7 = (byte *)((long)__dest + (long)local_50);
        *(undefined8 *)(puVar8 + -8) = 0x10af86;
        local_50 = local_60;
        local_60 = local_58;
        local_58 = pbVar7;
        memcpy(__dest,param_2,(size_t)local_50);
        *(byte *)((long)__dest + (long)local_50) = 0x6c;
        *local_58 = bVar12;
        local_58[1] = 0;
        *(undefined8 *)(puVar8 + -8) = 0x10afb0;
        __fprintf_chk(param_1,1,__dest,local_60);
        *(undefined8 *)(puVar8 + -8) = 0x10afb8;
        FUN_001121c0(__dest);
      }
      goto LAB_0010adfb;
    }
  }
LAB_0010ae70:
  pbVar13 = (byte *)0x0;
  puVar8 = auStack_68;
LAB_0010adfb:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)(puVar8 + -8) = &UNK_0010b065;
    __stack_chk_fail();
  }
  return pbVar13;
}




// Function: print_ifdef_lines @ 0xb070

void print_ifdef_lines(_IO_FILE *param_1,char *param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  char *pcVar5;
  char *pcVar6;
  size_t __n;
  char cVar7;
  uint uVar8;
  long lVar9;
  void *__ptr;
  char *pcVar10;
  long lVar11;
  
  lVar2 = *param_3;
  lVar11 = param_3[1];
  lVar3 = *(long *)(lVar2 + 0xb8);
  lVar4 = param_3[2];
  if (param_1 != (_IO_FILE *)0x0) {
    if ((DAT_001264a0 == '\0') && (*param_2 == '%')) {
      if (param_2[1] == 'l') {
        if ((param_2[2] == '\n') && (param_2[3] == '\0')) {
          if (lVar4 <= lVar11) {
            return;
          }
          __ptr = *(void **)(lVar3 + lVar11 * 8);
          lVar2 = *(long *)(lVar3 + lVar4 * 8);
          __n = (lVar2 + (ulong)(*(char *)(lVar2 + -1) != '\n')) - (long)__ptr;
          goto LAB_0010b234;
        }
      }
      else if ((param_2[1] == 'L') && (param_2[2] == '\0')) {
        __ptr = *(void **)(lVar3 + lVar11 * 8);
        __n = *(long *)(lVar3 + lVar4 * 8) - (long)__ptr;
LAB_0010b234:
        fwrite_unlocked(__ptr,1,__n,param_1);
        return;
      }
    }
    if (lVar11 < lVar4) {
      do {
        cVar7 = *param_2;
        pcVar10 = param_2;
joined_r0x0010b0e8:
        pcVar5 = pcVar10;
        if (cVar7 != '\0') {
          do {
            pcVar10 = pcVar5 + 1;
            if (cVar7 == '%') {
              cVar1 = pcVar5[1];
              pcVar6 = pcVar5 + 2;
              if (cVar1 == 'L') {
                lVar9 = *(long *)(lVar3 + 8 + lVar11 * 8);
              }
              else {
                if (cVar1 != 'l') {
                  if (cVar1 == '%') {
                    uVar8 = 0x25;
                    pcVar10 = pcVar6;
                    goto LAB_0010b0f6;
                  }
                  pcVar6 = (char *)FUN_0010ad70(param_1,pcVar5,lVar2,lVar11,0);
                  if (pcVar6 != (char *)0x0) goto LAB_0010b10d;
                  pcVar5 = param_1->_IO_write_ptr;
                  uVar8 = 0x25;
                  if (pcVar5 < param_1->_IO_write_end) goto LAB_0010b102;
                  goto LAB_0010b178;
                }
                lVar9 = *(long *)(lVar3 + 8 + lVar11 * 8);
                lVar9 = lVar9 - (ulong)(*(char *)(lVar9 + -1) == '\n');
              }
              FUN_0010f0e0(*(undefined8 *)(lVar3 + lVar11 * 8),lVar9,0,0);
            }
            else {
              uVar8 = (uint)cVar7;
LAB_0010b0f6:
              pcVar5 = param_1->_IO_write_ptr;
              if (param_1->_IO_write_end <= pcVar5) goto LAB_0010b178;
LAB_0010b102:
              param_1->_IO_write_ptr = pcVar5 + 1;
              *pcVar5 = cVar7;
              pcVar6 = pcVar10;
            }
LAB_0010b10d:
            cVar7 = *pcVar6;
            pcVar5 = pcVar6;
            if (cVar7 == '\0') break;
          } while( true );
        }
        lVar11 = lVar11 + 1;
        if (lVar4 == lVar11) {
          return;
        }
      } while( true );
    }
  }
  return;
LAB_0010b178:
  __overflow(param_1,uVar8 & 0xff);
  cVar7 = *pcVar10;
  goto joined_r0x0010b0e8;
}




// Function: format_group @ 0xb2a0

void format_group(_IO_FILE *param_1,byte *param_2,byte param_3,long param_4)

{
  byte *pbVar1;
  _IO_FILE *p_Var2;
  byte *__nptr;
  int *piVar3;
  intmax_t iVar4;
  byte *pbVar5;
  char *pcVar6;
  long lVar7;
  long lVar8;
  byte bVar9;
  byte bVar10;
  undefined8 uVar11;
  _IO_FILE *p_Var12;
  long in_FS_OFFSET;
  byte *local_60;
  long local_58 [5];
  
  bVar9 = *param_2;
  local_58[3] = *(long *)(in_FS_OFFSET + 0x28);
  if ((bVar9 != param_3) && (bVar9 != 0)) {
    do {
      pbVar1 = param_2 + 1;
      bVar10 = bVar9;
      if (bVar9 != 0x25) {
LAB_0010b2ee:
        __nptr = pbVar1;
        if (param_1 != (_IO_FILE *)0x0) {
          pbVar1 = (byte *)param_1->_IO_write_ptr;
          if (param_1->_IO_write_end <= pbVar1) {
            __overflow(param_1,(uint)bVar10);
            bVar9 = *__nptr;
            goto LAB_0010b313;
          }
          param_1->_IO_write_ptr = (char *)(pbVar1 + 1);
          *pbVar1 = bVar10;
        }
        bVar9 = *__nptr;
        goto LAB_0010b313;
      }
      bVar10 = param_2[1];
      __nptr = param_2 + 2;
      lVar8 = param_4;
      switch(bVar10) {
      case 0x25:
        pbVar1 = __nptr;
        goto LAB_0010b2ee;
      default:
        __nptr = (byte *)FUN_0010ad70(param_1,param_2,0,0,param_4);
        bVar10 = bVar9;
        if (__nptr == (byte *)0x0) goto LAB_0010b2ee;
        bVar9 = *__nptr;
        goto LAB_0010b313;
      case 0x28:
        lVar8 = 0;
        while( true ) {
          if ((int)(char)*__nptr - 0x30U < 10) {
            piVar3 = __errno_location();
            *piVar3 = 0;
            iVar4 = strtoimax((char *)__nptr,(char **)&local_60,10);
            local_58[lVar8] = iVar4;
            pbVar5 = local_60;
            bVar10 = bVar9;
            if (*piVar3 != 0) goto LAB_0010b2ee;
          }
          else {
            lVar7 = FUN_0010ac80(param_4);
            local_58[lVar8] = lVar7;
            bVar10 = bVar9;
            if (lVar7 < 0) goto LAB_0010b2ee;
            pbVar5 = __nptr + 1;
          }
          __nptr = pbVar5 + 1;
          bVar10 = bVar9;
          if (*pbVar5 != (&DAT_0011e194)[lVar8]) goto LAB_0010b2ee;
          if (lVar8 == 1) break;
          lVar8 = 1;
        }
        p_Var12 = (_IO_FILE *)0x0;
        p_Var2 = param_1;
        if (local_58[0] != local_58[1]) {
          p_Var12 = param_1;
          p_Var2 = (_IO_FILE *)0x0;
        }
        pcVar6 = (char *)format_group(p_Var2,__nptr,0x3a,param_4);
        if ((*pcVar6 != '\0') &&
           (pcVar6 = (char *)format_group(p_Var12,pcVar6 + 1,0x29,param_4), *pcVar6 != '\0')) {
          bVar9 = pcVar6[1];
          __nptr = (byte *)(pcVar6 + 1);
          goto LAB_0010b313;
        }
        goto LAB_0010b31c;
      case 0x3c:
        uVar11 = DAT_00126448;
        break;
      case 0x3d:
        uVar11 = DAT_00126440;
        break;
      case 0x3e:
        lVar8 = param_4 + 0x18;
        uVar11 = DAT_00126450;
      }
      FUN_0010b070(param_1,uVar11,lVar8);
      bVar9 = param_2[2];
LAB_0010b313:
      if ((bVar9 == 0) || (param_2 = __nptr, param_3 == bVar9)) break;
    } while( true );
  }
LAB_0010b31c:
  if (local_58[3] != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_ifdef_hunk @ 0xb560

void print_ifdef_hunk(undefined8 param_1)

{
  uint uVar1;
  long in_FS_OFFSET;
  long local_88;
  long local_80;
  long local_78;
  long local_70;
  undefined8 *local_68;
  long local_60;
  long local_58;
  undefined *local_50;
  long local_48;
  long local_40;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f670(param_1,&local_88,&local_80,&local_78,&local_70);
  if (uVar1 != 0) {
    FUN_0010e830();
    if ((DAT_001266f8 < local_88) || (DAT_001266f0 < local_78)) {
      local_40 = local_78;
      local_58 = local_88;
      local_48 = DAT_001266f0;
      local_68 = &DAT_001261a0;
      local_60 = DAT_001266f8;
      local_50 = &DAT_001262d0;
      FUN_0010b2a0(DAT_00126188,DAT_00126460,0,&local_68);
    }
    DAT_001266f8 = local_80 + 1;
    local_68 = &DAT_001261a0;
    DAT_001266f0 = local_70 + 1;
    local_50 = &DAT_001262d0;
    local_58 = DAT_001266f8;
    local_40 = DAT_001266f0;
    FUN_0010b2a0(DAT_00126188,(&DAT_00126460)[uVar1],0,&local_68);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ifdef_script @ 0xb6c0

void print_ifdef_script(undefined8 param_1)

{
  long in_FS_OFFSET;
  undefined8 *local_48;
  long local_40;
  long local_38;
  undefined *local_30;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001266f0 = -DAT_00126288;
  DAT_001266f8 = DAT_001266f0;
  FUN_0010f080(param_1,FUN_0010f060,FUN_0010b560);
  if ((DAT_001266f8 < DAT_00126270) || (DAT_001266f0 < DAT_001263a0)) {
    FUN_0010e830();
    local_48 = &DAT_001261a0;
    local_40 = DAT_001266f8;
    local_38 = DAT_00126270;
    local_30 = &DAT_001262d0;
    local_28 = DAT_001266f0;
    local_20 = DAT_001263a0;
    FUN_0010b2a0(DAT_00126188,DAT_00126460,0,&local_48);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: prepare_text @ 0xb7b0

void prepare_text(long param_1)

{
  char *pcVar1;
  char cVar2;
  undefined8 *__s;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  long lVar6;
  undefined8 *puVar7;
  
  __s = *(undefined8 **)(param_1 + 0xa0);
  if (__s != (undefined8 *)0x0) {
    lVar6 = *(long *)(param_1 + 0xb0);
    if (DAT_00126490 != '\0') {
      pcVar1 = (char *)((long)__s + lVar6);
      *pcVar1 = '\r';
      pcVar3 = rawmemchr(__s,0xd);
      pcVar5 = pcVar1;
      pcVar4 = pcVar3;
      for (; pcVar1 != pcVar3; pcVar3 = pcVar3 + 2) {
        while ((cVar2 = *pcVar3, cVar2 != '\r' || (pcVar3[1] != '\n'))) {
          pcVar3 = pcVar3 + 1;
          pcVar5 = pcVar4 + 1;
          *pcVar4 = cVar2;
          pcVar4 = pcVar5;
          if (pcVar1 == pcVar3) goto LAB_0010b840;
        }
        pcVar5 = pcVar4 + 1;
        *pcVar4 = '\n';
        pcVar4 = pcVar5;
      }
LAB_0010b840:
      lVar6 = lVar6 - ((long)pcVar1 - (long)pcVar5);
    }
    puVar7 = __s;
    if ((lVar6 != 0) &&
       (puVar7 = (undefined8 *)((long)__s + lVar6), *(char *)((long)__s + lVar6 + -1) != '\n')) {
      *(undefined1 *)puVar7 = 10;
      lVar6 = lVar6 + 1;
      *(undefined1 *)(param_1 + 0x120) = 1;
      puVar7 = (undefined8 *)((long)__s + lVar6);
    }
    *puVar7 = 0;
    *(long *)(param_1 + 0xb0) = lVar6;
  }
  return;
}




// Function: sip @ 0xb920

bool sip(int *param_1,char param_2)

{
  undefined8 uVar1;
  void *pvVar2;
  
  if (*param_1 < 0) {
    param_1[0x2a] = 8;
    param_1[0x2b] = 0;
    uVar1 = FUN_00116a40(8);
    *(undefined8 *)(param_1 + 0x28) = uVar1;
  }
  else {
    uVar1 = FUN_00117190(8,*(undefined8 *)(param_1 + 0x12),0x7fffffffffffffef);
    *(undefined8 *)(param_1 + 0x2a) = uVar1;
    uVar1 = FUN_00116a40(uVar1);
    *(undefined8 *)(param_1 + 0x28) = uVar1;
    if (param_2 == '\0') {
      FUN_0010b8a0(param_1);
      pvVar2 = memchr(*(void **)(param_1 + 0x28),0,*(size_t *)(param_1 + 0x2c));
      return pvVar2 != (void *)0x0;
    }
  }
  param_1[0x2c] = 0;
  param_1[0x2d] = 0;
  *(undefined1 *)((long)param_1 + 0x121) = 0;
  return false;
}




// Function: slurp @ 0xb9e0

void slurp(int *param_1)

{
  long lVar1;
  undefined8 uVar2;
  ulong uVar3;
  ulong uVar4;
  ulong uVar5;
  ulong uVar6;
  
  if (*param_1 < 0) {
    return;
  }
  if ((param_1[10] & 0xf000U) == 0x8000) {
    uVar5 = *(ulong *)(param_1 + 0x10);
    uVar4 = (uVar5 & 0xfffffffffffffff8) + 0x10;
    if ((uVar4 < uVar5) || (0x7ffffffffffffffe < uVar4)) {
LAB_0010bba9:
                    /* WARNING: Subroutine does not return */
      FUN_00116ec0();
    }
    if (*(ulong *)(param_1 + 0x2a) < uVar4) {
      *(ulong *)(param_1 + 0x2a) = uVar4;
      uVar2 = FUN_00116a90(*(undefined8 *)(param_1 + 0x28));
      *(undefined8 *)(param_1 + 0x28) = uVar2;
      uVar4 = *(ulong *)(param_1 + 0x2c);
    }
    else {
      uVar4 = *(ulong *)(param_1 + 0x2c);
    }
    if (uVar4 <= uVar5) {
      FUN_0010b8a0(param_1,(uVar5 - uVar4) + 1);
      uVar4 = *(ulong *)(param_1 + 0x2c);
      if (uVar4 <= uVar5) {
        return;
      }
    }
  }
  else {
    uVar4 = *(ulong *)(param_1 + 0x2c);
  }
  FUN_0010b8a0(param_1,*(long *)(param_1 + 0x2a) - uVar4);
  uVar4 = *(ulong *)(param_1 + 0x2c);
  if (uVar4 != 0) {
    uVar5 = *(ulong *)(param_1 + 0x2a);
    if (uVar4 == uVar5) {
      do {
        do {
          if (0x3ffffffffffffff7 < uVar5) goto LAB_0010bba9;
          *(ulong *)(param_1 + 0x2a) = uVar5 * 2;
          lVar1 = FUN_00116a90(*(undefined8 *)(param_1 + 0x28));
          uVar5 = *(ulong *)(param_1 + 0x2a);
          uVar4 = *(ulong *)(param_1 + 0x2c);
          *(long *)(param_1 + 0x28) = lVar1;
          uVar6 = uVar5 - uVar4;
        } while (uVar6 == 0);
        if (*(char *)((long)param_1 + 0x121) == '\0') {
          uVar3 = FUN_001170e0(*param_1,lVar1 + uVar4,uVar6);
          if (uVar3 == 0xffffffffffffffff) {
                    /* WARNING: Subroutine does not return */
            FUN_0010e770(*(undefined8 *)(param_1 + 2));
          }
          uVar5 = *(ulong *)(param_1 + 0x2a);
          uVar4 = *(long *)(param_1 + 0x2c) + uVar3;
          *(ulong *)(param_1 + 0x2c) = uVar4;
          *(bool *)((long)param_1 + 0x121) = uVar3 < uVar6;
        }
      } while (uVar4 == uVar5);
    }
    *(ulong *)(param_1 + 0x2a) = uVar4 + 0x10 & 0xfffffffffffffff8;
    uVar2 = FUN_00116a90(*(undefined8 *)(param_1 + 0x28));
    *(undefined8 *)(param_1 + 0x28) = uVar2;
  }
  return;
}




// Function: read_files @ 0xbbc0

/* WARNING: Type propagation algorithm not settling */

byte read_files(int *param_1,byte param_2)

{
  int *piVar1;
  undefined8 *puVar2;
  void *__s1;
  ushort *puVar3;
  char cVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  int iVar8;
  long lVar9;
  ulong uVar10;
  long lVar11;
  long lVar12;
  byte *pbVar13;
  ushort **ppuVar14;
  __int32_t **pp_Var15;
  long *plVar16;
  byte *pbVar17;
  byte *pbVar18;
  byte *pbVar19;
  byte *pbVar20;
  ulong uVar21;
  ulong *puVar22;
  ulong uVar23;
  ulong uVar24;
  ulong uVar25;
  uint uVar26;
  ulong uVar27;
  byte *pbVar28;
  uint uVar29;
  byte *pbVar30;
  ulong uVar31;
  ulong uVar32;
  byte *pbVar33;
  byte *pbVar34;
  byte *pbVar35;
  bool bVar36;
  bool bVar37;
  ulong local_e0;
  ulong local_d8;
  ulong *local_d0;
  ulong local_c0;
  void *local_b8;
  ulong local_b0;
  long local_a0;
  long local_98;
  ulong local_90;
  ulong local_88;
  long *local_60;
  
  bVar6 = DAT_00126568 | param_2;
  bVar5 = FUN_0010b920(param_1,DAT_00126568 | param_2);
  bVar5 = bVar5 | param_2;
  if (*param_1 == param_1[0x4c]) {
    *(undefined8 *)(param_1 + 0x74) = *(undefined8 *)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(undefined8 *)(param_1 + 0x78) = *(undefined8 *)(param_1 + 0x2c);
  }
  else {
    bVar6 = FUN_0010b920(param_1 + 0x4c,bVar6 | bVar5);
    bVar5 = bVar5 | bVar6;
  }
  if (bVar5 != 0) {
    return bVar5;
  }
  FUN_0010b9e0(param_1);
  FUN_0010b7b0(param_1);
  if (*param_1 == param_1[0x4c]) {
    uVar24 = *(ulong *)(param_1 + 0x2c);
    pbVar35 = *(byte **)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(byte **)(param_1 + 0x74) = pbVar35;
    *(ulong *)(param_1 + 0x78) = uVar24;
    *(char *)(param_1 + 0x94) = (char)param_1[0x48];
    uVar31 = uVar24;
LAB_0010bcf5:
    pbVar33 = pbVar35 + uVar31;
    pbVar13 = pbVar35 + uVar24;
    uVar29 = DAT_00126580 - 4;
    pbVar17 = pbVar33;
    lVar11 = DAT_00126560;
    pbVar34 = pbVar35;
    pbVar28 = pbVar33;
  }
  else {
    FUN_0010b9e0(param_1 + 0x4c);
    FUN_0010b7b0(param_1 + 0x4c);
    pbVar34 = *(byte **)(param_1 + 0x28);
    pbVar35 = *(byte **)(param_1 + 0x74);
    uVar24 = *(ulong *)(param_1 + 0x2c);
    uVar31 = *(ulong *)(param_1 + 0x78);
    if (pbVar34 == pbVar35) goto LAB_0010bcf5;
    pbVar13 = pbVar34 + uVar24;
    pbVar28 = pbVar35 + uVar31;
    if (uVar24 < uVar31) {
      *pbVar13 = ~pbVar35[uVar24];
    }
    else {
      *pbVar28 = ~pbVar34[uVar31];
    }
    pbVar33 = pbVar34;
    pbVar17 = pbVar35;
    if (*(long *)pbVar35 == *(long *)pbVar34) {
      do {
        pbVar33 = pbVar33 + 8;
        pbVar17 = pbVar17 + 8;
      } while (*(long *)pbVar33 == *(long *)pbVar17);
    }
    if (*pbVar17 == *pbVar33) {
      do {
        pbVar33 = pbVar33 + 1;
        pbVar17 = pbVar17 + 1;
      } while (*pbVar33 == *pbVar17);
    }
    uVar29 = DAT_00126580 - 4;
    lVar11 = DAT_00126560;
    if ((1 < uVar29) &&
       (pbVar34 + (uVar24 - *(byte *)(param_1 + 0x48)) < pbVar33 !=
        pbVar35 + (uVar31 - *(byte *)(param_1 + 0x94)) < pbVar17)) {
      pbVar33 = pbVar33 + -1;
      pbVar17 = pbVar17 + -1;
    }
  }
  for (; lVar9 = DAT_00126560, pbVar33 != pbVar34; pbVar33 = pbVar33 + -1) {
    while (pbVar33[-1] == 10) {
      if (lVar11 == 0) goto LAB_0010bd54;
      pbVar33 = pbVar33 + -1;
      pbVar17 = pbVar17 + -1;
      lVar11 = lVar11 + -1;
      if (pbVar33 == pbVar34) goto LAB_0010bd54;
    }
    pbVar17 = pbVar17 + -1;
  }
LAB_0010bd54:
  *(byte **)(param_1 + 0x38) = pbVar33;
  *(byte **)(param_1 + 0x84) = pbVar17;
  if ((uVar29 < 2) ||
     (pbVar17 = pbVar28, pbVar19 = pbVar13, (char)param_1[0x48] == (char)param_1[0x94])) {
    pbVar19 = pbVar28;
    pbVar20 = pbVar13;
    pbVar30 = pbVar33 + (uVar24 - uVar31);
    if (uVar24 < uVar31) {
      pbVar30 = pbVar33;
    }
    do {
      pbVar18 = pbVar20;
      pbVar17 = pbVar19;
      if (pbVar18 == pbVar30) break;
      pbVar19 = pbVar17 + -1;
      pbVar20 = pbVar18 + -1;
    } while (pbVar18[-1] == pbVar17[-1]);
    if ((pbVar18 == pbVar34) || (pbVar18[-1] == 10)) {
      if (pbVar17 != pbVar35) {
        lVar9 = lVar9 + (ulong)(pbVar17[-1] != 10);
      }
    }
    else {
      lVar9 = lVar9 + 1;
    }
    lVar11 = lVar9 + -1;
    pbVar19 = pbVar18;
    if ((lVar9 != 0) && (pbVar20 = pbVar18, pbVar18 != pbVar13)) {
      do {
        do {
          pbVar19 = pbVar20 + 1;
          bVar5 = *pbVar20;
          pbVar20 = pbVar19;
        } while (bVar5 != 10);
        bVar37 = lVar11 != 0;
        lVar11 = lVar11 + -1;
      } while (pbVar19 != pbVar13 && bVar37);
      pbVar17 = pbVar17 + ((long)pbVar19 - (long)pbVar18);
    }
  }
  bVar37 = DAT_00126578 == '\0';
  *(byte **)(param_1 + 0x3c) = pbVar19;
  *(byte **)(param_1 + 0x88) = pbVar17;
  if ((((bVar37) || (DAT_00126520 != 0)) || (0x1ffffffffffffffe < (long)DAT_00126570)) ||
     (uVar24 <= DAT_00126570)) {
    uVar24 = uVar24 >> 5;
    uVar31 = 0xffffffffffffffff;
    if (uVar24 == 0) {
      uVar24 = 1;
    }
    if (0x787878787878782 < uVar24) {
      uVar24 = 0x787878787878782;
    }
    lVar11 = 0;
    uVar24 = uVar24 + 5;
  }
  else {
    lVar11 = 1;
    uVar23 = (ulong)((long)pbVar19 - (long)pbVar33) >> 5;
    if (uVar23 == 0) {
      uVar23 = 1;
    }
    if (0x787878787878782 < uVar23) {
      uVar23 = 0x787878787878782;
    }
    uVar27 = (ulong)((long)pbVar13 - (long)pbVar19) >> 5;
    if (uVar27 == 0) {
      uVar27 = 1;
    }
    if (0x787878787878782 < uVar27) {
      uVar27 = 0x787878787878782;
    }
    uVar31 = 0;
    if (0 < (long)DAT_00126570) {
      do {
        lVar11 = lVar11 * 2;
      } while (lVar11 <= (long)DAT_00126570);
      uVar31 = lVar11 - 1;
    }
    uVar24 = DAT_00126570;
    if ((long)(uVar27 + 5) < (long)DAT_00126570) {
      uVar24 = uVar27 + 5;
    }
    uVar24 = uVar23 + 5 + lVar11 + uVar24;
  }
  lVar9 = FUN_00116a40(uVar24 * 8);
  pbVar13 = *(byte **)(param_1 + 0x38);
  if (((DAT_00126578 == '\0') || (pbVar19 != pbVar13)) ||
     (pbVar33 = *(byte **)(param_1 + 0x84), pbVar19 = pbVar34, pbVar17 != pbVar33)) {
    uVar23 = 0;
    pbVar33 = pbVar34;
    uVar27 = uVar24;
    if (pbVar34 == pbVar13) {
      pbVar33 = *(byte **)(param_1 + 0x84);
      pbVar19 = pbVar13;
      goto LAB_0010cbe8;
    }
    do {
      uVar32 = uVar23 + 1;
      uVar24 = uVar27;
      if ((uVar23 & uVar31) == uVar27) {
        if (0x7fffffffffffffe < uVar27) goto LAB_0010cd15;
        uVar24 = uVar27 * 2;
        lVar9 = FUN_00116a90(lVar9,uVar27 << 4);
      }
      *(byte **)(lVar9 + (uVar23 & uVar31) * 8) = pbVar33;
      do {
        pbVar19 = pbVar33 + 1;
        bVar5 = *pbVar33;
        pbVar33 = pbVar19;
      } while (bVar5 != 10);
      uVar23 = uVar32;
      uVar27 = uVar24;
    } while (pbVar19 != pbVar13);
    pbVar33 = *(byte **)(param_1 + 0x84);
    if (lVar11 != 0) goto LAB_0010bfa9;
LAB_0010bfb3:
    uVar27 = 0x20;
    uVar25 = (long)pbVar17 - (long)pbVar33;
    if (9 < (long)uVar32) {
      uVar27 = (ulong)((long)pbVar13 - (long)pbVar34) / (uVar32 - 1);
    }
  }
  else {
LAB_0010cbe8:
    pbVar13 = pbVar19;
    if (lVar11 != 0) {
      uVar32 = 0;
LAB_0010bfa9:
      uVar23 = uVar32;
      if ((long)DAT_00126570 <= (long)uVar32) {
        uVar23 = DAT_00126570;
      }
      goto LAB_0010bfb3;
    }
    uVar23 = 0;
    uVar27 = 0x20;
    uVar25 = (long)pbVar17 - (long)pbVar33;
    uVar32 = 0;
  }
  uVar10 = (ulong)((long)pbVar28 - (long)pbVar17) / uVar27;
  if (uVar10 == 0) {
    uVar10 = 1;
  }
  if (0x787878787878782 < uVar10) {
    uVar10 = 0x787878787878782;
  }
  uVar21 = DAT_00126570;
  if ((long)(uVar10 + 5) <= (long)DAT_00126570) {
    uVar21 = uVar10 + 5;
  }
  uVar25 = uVar25 / uVar27;
  if (uVar25 == 0) {
    uVar25 = 1;
  }
  if (0x787878787878782 < uVar25) {
    uVar25 = 0x787878787878782;
  }
  uVar21 = uVar21 + uVar23 + 5 + uVar25;
  if ((long)uVar21 < (long)uVar23 || 0xffffffffffffffe < uVar21) goto LAB_0010cd15;
  lVar11 = FUN_00116a40(uVar21 * 8);
  if (uVar23 == uVar32) {
    if (uVar23 != 0) goto LAB_0010c0d0;
  }
  else if (0 < (long)uVar23) {
    uVar25 = uVar32 - DAT_00126570;
    uVar27 = uVar23 + uVar25;
    lVar12 = uVar25 * -8;
    do {
      *(undefined8 *)(lVar11 + lVar12 + uVar25 * 8) = *(undefined8 *)(lVar9 + (uVar31 & uVar25) * 8)
      ;
      uVar25 = uVar25 + 1;
    } while (uVar27 != uVar25);
    uVar31 = 0;
    do {
      *(undefined8 *)(lVar9 + uVar31 * 8) = *(undefined8 *)(lVar11 + uVar31 * 8);
      uVar31 = uVar31 + 1;
    } while (uVar23 != uVar31);
LAB_0010c0d0:
    lVar12 = 0;
    do {
      *(byte **)(lVar11 + lVar12 * 8) = pbVar35 + (*(long *)(lVar9 + lVar12 * 8) - (long)pbVar34);
      lVar12 = lVar12 + 1;
    } while (lVar12 < (long)uVar23);
  }
  *(ulong *)(param_1 + 0x7a) = uVar23 * 8 + lVar11;
  *(ulong *)(param_1 + 0x2e) = lVar9 + uVar23 * 8;
  *(ulong *)(param_1 + 0x7c) = -uVar23;
  *(ulong *)(param_1 + 0x30) = -uVar23;
  DAT_00126700 = (uVar24 - uVar23) + 1 + (uVar21 - uVar23);
  *(ulong *)(param_1 + 0x36) = uVar24 - uVar23;
  *(ulong *)(param_1 + 0x82) = uVar21 - uVar23;
  *(ulong *)(param_1 + 0x86) = uVar32;
  *(ulong *)(param_1 + 0x3a) = uVar32;
  if (0x3fffffffffffffe < DAT_00126700) {
LAB_0010cd15:
                    /* WARNING: Subroutine does not return */
    FUN_00116ec0();
  }
  DAT_00126710 = (void *)FUN_00116a40(DAT_00126700 * 0x20);
  DAT_00126708 = 1;
  if ((ulong)((long)DAT_00126700 / 3) < 0x201) {
    DAT_00126718 = 0x1fd;
    lVar11 = 0xff0;
  }
  else {
    iVar8 = 9;
    do {
      iVar8 = iVar8 + 1;
      uVar24 = 1L << ((byte)iVar8 & 0x3f);
    } while (uVar24 < (ulong)((long)DAT_00126700 / 3));
    DAT_00126718 = uVar24 - (byte)(&DAT_0011e1a0)[iVar8];
    if (0xffffffffffffffe < DAT_00126718) goto LAB_0010cd15;
    lVar11 = DAT_00126718 * 8 + 8;
  }
  lVar11 = FUN_0010fab0(lVar11);
  DAT_00126720 = lVar11 + 8;
  local_60 = (long *)(param_1 + 0x28);
  do {
    local_a0 = local_60[3];
    lVar11 = local_60[4];
    pbVar35 = (byte *)local_60[8];
    local_90 = local_60[7];
    local_98 = FUN_00116a40(local_90 * 8);
    uVar29 = DAT_00126558;
    cVar4 = DAT_00126552;
    pbVar34 = (byte *)local_60[10];
    local_b8 = DAT_00126710;
    local_c0 = DAT_00126708;
    local_88 = DAT_00126700;
    pbVar13 = (byte *)(local_60[2] + *local_60);
    bVar37 = DAT_00126558 == 0;
    if (pbVar35 < pbVar34) {
      uVar26 = DAT_00126558 & 1;
      bVar5 = *pbVar35;
      local_d8 = 0;
      pbVar28 = pbVar35;
      uVar24 = DAT_00126700;
      uVar31 = DAT_00126498;
      do {
        uVar23 = (ulong)bVar5;
        pbVar35 = pbVar28 + 1;
        DAT_00126498 = uVar31;
        if (uVar29 == 4) {
          if (bVar5 != 10) {
            ppuVar14 = __ctype_b_loc();
            uVar27 = 0;
            puVar3 = *ppuVar14;
            do {
              pbVar33 = pbVar35;
              if ((*(byte *)((long)puVar3 + uVar23 * 2 + 1) & 0x20) != 0) {
                bVar5 = *pbVar35;
                while( true ) {
                  pbVar35 = pbVar35 + 1;
                  if (bVar5 == 10) goto LAB_0010c549;
                  uVar23 = (ulong)bVar5;
                  if ((*(byte *)((long)puVar3 + uVar23 * 2 + 1) & 0x20) == 0) break;
                  bVar5 = *pbVar35;
                }
                uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + 0x20;
                pbVar33 = pbVar35;
              }
              if (cVar4 != '\0') {
                pp_Var15 = __ctype_tolower_loc();
                uVar23 = (ulong)(*pp_Var15)[uVar23];
              }
              pbVar35 = pbVar33 + 1;
              uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + uVar23;
              uVar23 = (ulong)*pbVar33;
            } while (*pbVar33 != 10);
LAB_0010c549:
            pbVar33 = pbVar35 + (-1 - (long)pbVar28);
            goto LAB_0010c3d0;
          }
LAB_0010c8d1:
          pbVar33 = (byte *)0x0;
          uVar27 = 0;
        }
        else if (uVar29 < 5) {
          if (uVar29 - 1 < 3) {
            if (bVar5 == 10) goto LAB_0010c8d1;
            uVar27 = 0;
            uVar32 = 0;
            pbVar33 = pbVar35;
            do {
              if ((uVar29 & 2) != 0) {
                ppuVar14 = __ctype_b_loc();
                pbVar35 = pbVar33;
                for (bVar5 = *(byte *)((long)*ppuVar14 + uVar23 * 2 + 1); (bVar5 & 0x20) != 0;
                    bVar5 = *(byte *)((long)*ppuVar14 + (ulong)bVar5 * 2 + 1)) {
                  bVar5 = *pbVar35;
                  pbVar35 = pbVar35 + 1;
                  if (bVar5 == 10) {
                    pbVar33 = pbVar35 + (-1 - (long)pbVar28);
                    goto LAB_0010c3d0;
                  }
                }
              }
              if (uVar26 == 0) {
LAB_0010c6a0:
                uVar25 = 1;
              }
              else {
                cVar7 = (char)uVar23;
                if (cVar7 == '\t') {
                  uVar23 = 0x20;
                  uVar25 = uVar31 - uVar32 % uVar31;
                  bVar36 = CARRY8(uVar32,uVar25);
                  uVar32 = uVar32 + uVar25;
                  if (bVar36) {
                    uVar32 = 0;
                  }
                }
                else if (cVar7 == '\r') {
                  uVar23 = 0xd;
                  uVar25 = 1;
                  uVar32 = 0;
                }
                else {
                  if (cVar7 == '\b') {
                    uVar23 = 8;
                    uVar32 = (uVar32 - 1) + (ulong)(uVar32 == 0);
                    goto LAB_0010c6a0;
                  }
                  uVar32 = uVar32 + 1;
                  uVar25 = 1;
                }
              }
              if (cVar4 != '\0') {
                pp_Var15 = __ctype_tolower_loc();
                uVar23 = (ulong)*(byte *)(*pp_Var15 + uVar23);
              }
              do {
                uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + uVar23;
                uVar25 = uVar25 - 1;
              } while (uVar25 != 0);
              bVar5 = *pbVar33;
              uVar23 = (ulong)bVar5;
              pbVar35 = pbVar33 + 1;
              pbVar33 = pbVar35;
            } while (bVar5 != 10);
            pbVar33 = pbVar35 + (-1 - (long)pbVar28);
          }
          else {
LAB_0010c810:
            if (cVar4 == '\0') {
              uVar27 = 0;
              pbVar33 = (byte *)0x0;
              pbVar17 = pbVar35;
              if (bVar5 != 10) {
                do {
                  pbVar35 = pbVar17 + 1;
                  uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + uVar23;
                  bVar5 = *pbVar17;
                  uVar23 = (ulong)bVar5;
                  pbVar17 = pbVar35;
                } while (bVar5 != 10);
                pbVar33 = pbVar35 + (-1 - (long)pbVar28);
              }
            }
            else {
              if (bVar5 == 10) goto LAB_0010c8d1;
              pp_Var15 = __ctype_tolower_loc();
              uVar27 = 0;
              pbVar33 = pbVar35;
              do {
                pbVar35 = pbVar33 + 1;
                piVar1 = *pp_Var15 + uVar23;
                bVar5 = *pbVar33;
                uVar23 = (ulong)bVar5;
                uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + (long)*piVar1;
                pbVar33 = pbVar35;
              } while (bVar5 != 10);
              pbVar33 = pbVar35 + (-1 - (long)pbVar28);
            }
          }
        }
        else {
          if (uVar29 != 5) goto LAB_0010c810;
          if (bVar5 == 10) goto LAB_0010c8d1;
          ppuVar14 = __ctype_b_loc();
          uVar27 = 0;
          puVar3 = *ppuVar14;
          do {
            if ((*(byte *)((long)puVar3 + uVar23 * 2 + 1) & 0x20) == 0) {
              if (cVar4 != '\0') {
                pp_Var15 = __ctype_tolower_loc();
                uVar23 = (ulong)(*pp_Var15)[uVar23];
              }
              uVar27 = (uVar27 << 7 | uVar27 >> 0x39) + uVar23;
            }
            bVar5 = *pbVar35;
            uVar23 = (ulong)bVar5;
            pbVar35 = pbVar35 + 1;
          } while (bVar5 != 10);
          pbVar33 = pbVar35 + (-1 - (long)pbVar28);
        }
LAB_0010c3d0:
        puVar22 = (ulong *)(DAT_00126720 + (uVar27 % DAT_00126718) * 8);
        local_d0 = puVar22;
        if (((pbVar13 == pbVar35) && ((char)local_60[0x10] != '\0')) &&
           ((1 < DAT_00126580 - 4U && (local_d0 = (ulong *)(DAT_00126720 + -8), 1 < uVar29)))) {
          local_d0 = puVar22;
        }
        local_b0 = *local_d0;
        local_e0 = local_b0;
joined_r0x0010c40c:
        local_88 = uVar24;
        if (local_e0 != 0) {
          do {
            puVar22 = (ulong *)(local_e0 * 0x20 + (long)local_b8);
            if (uVar27 == puVar22[1]) {
              __s1 = (void *)puVar22[2];
              if ((byte *)puVar22[3] == pbVar33) {
                iVar8 = memcmp(__s1,pbVar28,(size_t)pbVar33);
                if (iVar8 == 0) goto joined_r0x0010c477;
                if (bVar37 && cVar4 == '\0') goto code_r0x0010c58c;
              }
              else if (uVar29 == 0) goto LAB_0010c420;
              cVar7 = FUN_0010ec00(__s1,pbVar28);
              if (cVar7 == '\0') goto joined_r0x0010c477;
            }
LAB_0010c420:
            local_e0 = *puVar22;
            if (local_e0 == 0) break;
          } while( true );
        }
        if (uVar24 == local_c0) {
          if (0x1fffffffffffffe < uVar24) goto LAB_0010cd15;
          local_88 = uVar24 * 2;
          local_b8 = (void *)FUN_00116a90(local_b8,uVar24 << 6);
          local_b0 = *local_d0;
        }
        puVar22 = (ulong *)(local_c0 * 0x20 + (long)local_b8);
        *puVar22 = local_b0;
        puVar22[1] = uVar27;
        puVar22[2] = (ulong)pbVar28;
        puVar22[3] = (ulong)pbVar33;
        *local_d0 = local_c0;
        local_e0 = local_c0;
        local_c0 = local_c0 + 1;
joined_r0x0010c477:
        if (local_d8 == local_90) {
          if (((0x2aaaaaaaaaaaaaa9 < (long)local_d8) ||
              (local_90 = local_d8 * 2 - lVar11, 0xffffffffffffffe < local_90)) ||
             (0xffffffffffffffe < local_d8 - lVar11)) goto LAB_0010cd15;
          local_98 = FUN_00116a90(local_98,local_90 * 8);
          local_a0 = FUN_00116a90(local_a0 + lVar11 * 8,(local_90 - lVar11) * 8);
          local_a0 = local_a0 + lVar11 * -8;
        }
        *(byte **)(local_a0 + local_d8 * 8) = pbVar28;
        *(ulong *)(local_98 + local_d8 * 8) = local_e0;
        local_d8 = local_d8 + 1;
        if (pbVar34 <= pbVar35) goto LAB_0010c8e7;
        bVar5 = *pbVar35;
        pbVar28 = pbVar35;
        uVar24 = local_88;
        uVar31 = DAT_00126498;
      } while( true );
    }
    local_d8 = 0;
LAB_0010c8e7:
    lVar9 = 0;
    local_60[5] = local_d8;
    while( true ) {
      if (local_90 == local_d8) {
        if (((0x2aaaaaaaaaaaaaa9 < (long)local_90) ||
            (uVar24 = local_90 * 2 - lVar11, 0xffffffffffffffe < uVar24)) ||
           (0xffffffffffffffe < local_90 - lVar11)) goto LAB_0010cd15;
        local_a0 = FUN_00116a90(local_a0 + lVar11 * 8,(uVar24 - lVar11) * 8);
        local_a0 = local_a0 + lVar11 * -8;
        puVar2 = (undefined8 *)(local_a0 + local_d8 * 8);
        *puVar2 = pbVar35;
        local_90 = uVar24;
      }
      else {
        puVar2 = (undefined8 *)(local_a0 + local_d8 * 8);
        *puVar2 = pbVar35;
      }
      if (pbVar13 == pbVar35) break;
      if (((long)DAT_00126570 <= lVar9) && (DAT_00126578 != '\0')) goto LAB_0010c9fc;
      local_d8 = local_d8 + 1;
      pbVar34 = pbVar35;
      do {
        pbVar35 = pbVar34 + 1;
        bVar5 = *pbVar34;
        pbVar34 = pbVar35;
      } while (bVar5 != 10);
      lVar9 = lVar9 + 1;
    }
    if (((char)local_60[0x10] != '\0') && (1 < DAT_00126580 - 4U)) {
      *puVar2 = pbVar13 + -1;
    }
LAB_0010c9fc:
    local_60[3] = local_a0;
    plVar16 = local_60 + 0x26;
    DAT_00126700 = local_88;
    local_60[6] = local_d8;
    DAT_00126708 = local_c0;
    local_60[7] = local_90;
    local_60[0xb] = local_98;
    DAT_00126710 = local_b8;
    local_60 = plVar16;
    if ((long *)(param_1 + 0xc0) == plVar16) {
      *(ulong *)(param_1 + 0x96) = local_c0;
      *(ulong *)(param_1 + 0x4a) = local_c0;
      free(local_b8);
      free((void *)(DAT_00126720 + -8));
      return 0;
    }
  } while( true );
code_r0x0010c58c:
  local_e0 = *puVar22;
  goto joined_r0x0010c40c;
}




// Function: print_normal_hunk @ 0xcd40

void print_normal_hunk(undefined8 param_1)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  uint uVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  long in_FS_OFFSET;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = FUN_0010f670(param_1,&local_50,&local_48,&local_40,&local_38);
  if (uVar4 != 0) {
    FUN_0010e830();
    FUN_0010f510(4);
    FUN_0010f610(0x2c,&DAT_001261a0,local_50,local_48);
    bVar1 = (&DAT_0011e550)[uVar4];
    pbVar2 = (byte *)DAT_00126188->_IO_write_ptr;
    if (pbVar2 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = (char *)(pbVar2 + 1);
      *pbVar2 = bVar1;
    }
    else {
      __overflow(DAT_00126188,(uint)bVar1);
    }
    FUN_0010f610(0x2c,&DAT_001262d0,local_40,local_38);
    FUN_0010f510(3);
    pcVar3 = DAT_00126188->_IO_write_ptr;
    if (pcVar3 < DAT_00126188->_IO_write_end) {
      DAT_00126188->_IO_write_ptr = pcVar3 + 1;
      *pcVar3 = '\n';
    }
    else {
      __overflow(DAT_00126188,10);
    }
    if (((uVar4 & 1) != 0) && (local_50 <= local_48)) {
      lVar5 = local_50 * 8;
      lVar6 = local_50;
      do {
        while( true ) {
          FUN_0010f510(2);
          lVar7 = DAT_00126258 + lVar5;
          lVar5 = lVar5 + 8;
          FUN_0010f330(&DAT_0011c156,lVar7,1);
          FUN_0010f510(3);
          if (*(char *)(*(long *)(DAT_00126258 + lVar5) + -1) == '\n') break;
LAB_0010ce70:
          lVar6 = lVar6 + 1;
          if (local_48 < lVar6) goto LAB_0010cef0;
        }
        pcVar3 = DAT_00126188->_IO_write_ptr;
        if (DAT_00126188->_IO_write_end <= pcVar3) {
          __overflow(DAT_00126188,10);
          goto LAB_0010ce70;
        }
        lVar6 = lVar6 + 1;
        DAT_00126188->_IO_write_ptr = pcVar3 + 1;
        *pcVar3 = '\n';
      } while (lVar6 <= local_48);
    }
LAB_0010cef0:
    if (uVar4 == 3) {
      fputs_unlocked("---\n",DAT_00126188);
    }
    else if ((uVar4 & 2) == 0) goto LAB_0010cd7b;
    if (local_40 <= local_38) {
      lVar5 = local_40 * 8;
      lVar6 = local_40;
      do {
        FUN_0010f510(1);
        lVar7 = DAT_00126388 + lVar5;
        lVar5 = lVar5 + 8;
        FUN_0010f330(&DAT_0011c159,lVar7,1);
        FUN_0010f510(3);
        if (*(char *)(*(long *)(DAT_00126388 + lVar5) + -1) == '\n') {
          pcVar3 = DAT_00126188->_IO_write_ptr;
          if (pcVar3 < DAT_00126188->_IO_write_end) {
            DAT_00126188->_IO_write_ptr = pcVar3 + 1;
            *pcVar3 = '\n';
          }
          else {
            __overflow(DAT_00126188,10);
          }
        }
        lVar6 = lVar6 + 1;
      } while (lVar6 <= local_38);
    }
  }
LAB_0010cd7b:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_normal_script @ 0xcff0

void print_normal_script(undefined8 param_1)

{
  FUN_0010f080(param_1,FUN_0010f060,FUN_0010cd40);
  return;
}




// Function: tab_from_to @ 0xd010

ulong tab_from_to(ulong param_1,ulong param_2)

{
  char *pcVar1;
  ulong uVar2;
  _IO_FILE *p_Var3;
  ulong uVar4;
  ulong uVar5;
  
  uVar4 = DAT_00126498;
  p_Var3 = DAT_00126188;
  if (DAT_001264a0 == '\0') {
    uVar5 = (DAT_00126498 + param_1) - param_1 % DAT_00126498;
    while (uVar2 = uVar5, uVar2 <= param_2) {
      pcVar1 = p_Var3->_IO_write_ptr;
      if (pcVar1 < p_Var3->_IO_write_end) {
        p_Var3->_IO_write_ptr = pcVar1 + 1;
        *pcVar1 = '\t';
      }
      else {
        __overflow(p_Var3,9);
      }
      param_1 = uVar2;
      uVar5 = uVar4 + uVar2;
    }
  }
  if (param_1 < param_2) {
    do {
      param_1 = param_1 + 1;
      pcVar1 = p_Var3->_IO_write_ptr;
      if (pcVar1 < p_Var3->_IO_write_end) {
        p_Var3->_IO_write_ptr = pcVar1 + 1;
        *pcVar1 = ' ';
      }
      else {
        __overflow(p_Var3,0x20);
      }
    } while (param_2 != param_1);
  }
  return param_2;
}




// Function: print_half_line @ 0xd0d0

ulong print_half_line(undefined8 *param_1,undefined8 param_2,ulong param_3)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  _IO_FILE *p_Var4;
  int iVar5;
  size_t __n;
  long lVar6;
  byte *pbVar7;
  byte *pbVar8;
  ulong uVar9;
  ulong uVar10;
  ulong uVar11;
  long in_FS_OFFSET;
  wchar_t local_4c;
  undefined8 local_48;
  long local_40;
  
  p_Var4 = DAT_00126188;
  pbVar2 = (byte *)param_1[1];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  uVar9 = 0;
  if ((byte *)*param_1 < pbVar2) {
    pbVar7 = (byte *)*param_1;
    uVar11 = 0;
    do {
      while( true ) {
        bVar1 = *pbVar7;
        pbVar8 = pbVar7 + 1;
        uVar10 = uVar11;
        if ('_' < (char)bVar1) break;
        if ((char)bVar1 < '\b') {
switchD_0010d150_caseD_e:
          __n = FUN_00117200(&local_4c,pbVar7,(long)pbVar2 - (long)pbVar7,&local_48);
          if (__n - 1 < 0xfffffffffffffffd) {
            iVar5 = wcwidth(local_4c);
            if (0 < iVar5) {
              uVar11 = (long)iVar5 + uVar11;
            }
            if (uVar11 <= param_3) {
              fwrite_unlocked(pbVar7,1,__n,stdout);
              uVar9 = uVar11;
            }
            pbVar8 = pbVar7 + __n;
            uVar10 = uVar11;
          }
          else {
switchD_0010d150_caseD_b:
            if (uVar11 < param_3) {
              pbVar7 = (byte *)p_Var4->_IO_write_ptr;
              if (pbVar7 < p_Var4->_IO_write_end) {
                p_Var4->_IO_write_ptr = (char *)(pbVar7 + 1);
                *pbVar7 = bVar1;
              }
              else {
                __overflow(p_Var4,(uint)bVar1);
              }
            }
          }
        }
        else {
          switch(bVar1) {
          case 8:
            if ((uVar11 != 0) && (uVar10 = uVar11 - 1, uVar10 < param_3)) {
              if (uVar10 < uVar9) {
                pcVar3 = p_Var4->_IO_write_ptr;
                uVar9 = uVar10;
                if (pcVar3 < p_Var4->_IO_write_end) {
                  p_Var4->_IO_write_ptr = pcVar3 + 1;
                  *pcVar3 = '\b';
                }
                else {
                  __overflow(p_Var4,8);
                }
              }
              else if (uVar9 < uVar10) {
                do {
                  pcVar3 = p_Var4->_IO_write_ptr;
                  if (pcVar3 < p_Var4->_IO_write_end) {
                    p_Var4->_IO_write_ptr = pcVar3 + 1;
                    *pcVar3 = ' ';
                  }
                  else {
                    __overflow(p_Var4,0x20);
                  }
                  uVar9 = uVar9 + 1;
                } while (uVar10 != uVar9);
              }
            }
            break;
          case 9:
            lVar6 = DAT_00126498 - uVar11 % DAT_00126498;
            if (uVar9 == uVar11) {
              uVar10 = lVar6 + uVar9;
              if (DAT_001264a0 == '\0') {
                if (uVar10 < param_3) {
                  pcVar3 = p_Var4->_IO_write_ptr;
                  uVar9 = uVar10;
                  if (pcVar3 < p_Var4->_IO_write_end) {
                    p_Var4->_IO_write_ptr = pcVar3 + 1;
                    *pcVar3 = '\t';
                  }
                  else {
                    __overflow(p_Var4,9);
                  }
                }
              }
              else {
                if (param_3 <= uVar10) {
                  uVar10 = param_3;
                }
                if (uVar9 < uVar10) {
                  do {
                    pcVar3 = p_Var4->_IO_write_ptr;
                    if (pcVar3 < p_Var4->_IO_write_end) {
                      p_Var4->_IO_write_ptr = pcVar3 + 1;
                      *pcVar3 = ' ';
                    }
                    else {
                      __overflow(p_Var4,0x20);
                    }
                    uVar9 = uVar9 + 1;
                  } while (uVar9 != uVar10);
                }
              }
            }
            uVar10 = uVar11 + lVar6;
            break;
          case 10:
            goto switchD_0010d150_caseD_a;
          case 0xb:
          case 0xc:
            goto switchD_0010d150_caseD_b;
          case 0xd:
            pcVar3 = p_Var4->_IO_write_ptr;
            if (pcVar3 < p_Var4->_IO_write_end) {
              p_Var4->_IO_write_ptr = pcVar3 + 1;
              *pcVar3 = '\r';
            }
            else {
              __overflow(p_Var4,0xd);
            }
            FUN_0010d010(0,param_2);
            uVar9 = 0;
            uVar10 = 0;
            break;
          default:
            goto switchD_0010d150_caseD_e;
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
            goto switchD_0010d150_caseD_20;
          }
        }
LAB_0010d220:
        pbVar7 = pbVar8;
        uVar11 = uVar10;
        if (pbVar2 <= pbVar8) goto switchD_0010d150_caseD_a;
      }
      if (0x1d < (byte)(bVar1 + 0x9f)) goto switchD_0010d150_caseD_e;
switchD_0010d150_caseD_20:
      uVar10 = uVar11 + 1;
      if (param_3 <= uVar11) goto LAB_0010d220;
      pbVar7 = (byte *)p_Var4->_IO_write_ptr;
      uVar9 = uVar10;
      if (p_Var4->_IO_write_end <= pbVar7) {
        __overflow(p_Var4,(uint)bVar1);
        goto LAB_0010d220;
      }
      p_Var4->_IO_write_ptr = (char *)(pbVar7 + 1);
      *pbVar7 = bVar1;
      pbVar7 = pbVar8;
      uVar11 = uVar10;
    } while (pbVar8 < pbVar2);
  }
switchD_0010d150_caseD_a:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar9;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_1sdiff_line @ 0xd500

void print_1sdiff_line(long param_1,char param_2,undefined8 *param_3)

{
  char *pcVar1;
  bool bVar2;
  _IO_FILE *p_Var3;
  long lVar4;
  long lVar5;
  long lVar6;
  undefined8 uVar7;
  int iVar8;
  bool bVar9;
  char local_39;
  
  lVar5 = DAT_00126430;
  lVar4 = DAT_00126428;
  p_Var3 = DAT_00126188;
  local_39 = param_2;
  if (param_2 == '<') {
    FUN_0010f510(2);
LAB_0010d67a:
    bVar2 = true;
    if (param_1 != 0) {
LAB_0010d556:
      bVar9 = *(char *)(*(long *)(param_1 + 8) + -1) == '\n';
      lVar6 = FUN_0010d0d0(param_1,0,lVar5);
      goto joined_r0x0010d5f8;
    }
    bVar9 = false;
    lVar6 = FUN_0010d010(0,(ulong)(lVar5 + -1 + lVar4) >> 1);
    lVar6 = lVar6 + 1;
LAB_0010d6a2:
    iVar8 = (int)param_2;
  }
  else {
    if (param_2 == '>') {
      FUN_0010f510(1);
      goto LAB_0010d67a;
    }
    bVar2 = false;
    if (param_1 != 0) goto LAB_0010d556;
    bVar9 = false;
    lVar6 = 0;
joined_r0x0010d5f8:
    if (param_2 == ' ') goto LAB_0010d57b;
    lVar6 = FUN_0010d010(lVar6,(ulong)(lVar5 + -1 + lVar4) >> 1);
    lVar6 = lVar6 + 1;
    if (param_2 != '|') goto LAB_0010d6a2;
    iVar8 = 0x7c;
    if ((*(char *)(param_3[1] + -1) == '\n') != bVar9) {
      iVar8 = (-(uint)(bVar9 == false) & 0x2d) + 0x2f;
      local_39 = (-(bVar9 == false) & 0x2dU) + 0x2f;
    }
  }
  pcVar1 = p_Var3->_IO_write_ptr;
  if (pcVar1 < p_Var3->_IO_write_end) {
    p_Var3->_IO_write_ptr = pcVar1 + 1;
    *pcVar1 = local_39;
  }
  else {
    __overflow(p_Var3,iVar8);
  }
LAB_0010d57b:
  if ((param_3 != (undefined8 *)0x0) &&
     (bVar9 = (bool)(bVar9 | *(char *)(param_3[1] + -1) == '\n'), *(char *)*param_3 != '\n')) {
    uVar7 = FUN_0010d010(lVar6,lVar4);
    FUN_0010d0d0(param_3,uVar7,lVar5);
  }
  if (bVar9 != false) {
    pcVar1 = p_Var3->_IO_write_ptr;
    if (pcVar1 < p_Var3->_IO_write_end) {
      p_Var3->_IO_write_ptr = pcVar1 + 1;
      *pcVar1 = '\n';
    }
    else {
      __overflow(p_Var3,10);
    }
  }
  if (bVar2) {
    FUN_0010f510(3);
    return;
  }
  return;
}




// Function: print_sdiff_common_lines @ 0xd700

void print_sdiff_common_lines(long param_1,long param_2)

{
  long lVar1;
  long lVar2;
  bool bVar3;
  bool bVar4;
  
  lVar2 = DAT_00126730;
  lVar1 = DAT_00126728;
  if (DAT_00126438 == '\0') {
    bVar3 = DAT_00126730 != param_1;
    bVar4 = DAT_00126728 != param_2;
    if ((bVar3) || (bVar4)) {
      if (DAT_0012643a != '\0') {
        __fprintf_chk(DAT_00126188,1,"i%ld,%ld\n",param_1 - DAT_00126730,param_2 - DAT_00126728);
      }
      if (DAT_00126439 == '\0') {
        if ((bVar3) && (bVar4)) {
          do {
            lVar1 = lVar1 + 1;
            lVar2 = lVar2 + 1;
            FUN_0010d500(DAT_00126258 + -8 + lVar2 * 8,0x20,DAT_00126388 + -8 + lVar1 * 8);
            if (param_1 == lVar2) goto joined_r0x0010d7cb;
          } while (param_2 != lVar1);
        }
        else {
joined_r0x0010d7cb:
          for (; param_2 != lVar1; lVar1 = lVar1 + 1) {
            FUN_0010d500(0,0x29,DAT_00126388 + lVar1 * 8);
          }
        }
      }
      for (; param_1 != lVar2; lVar2 = lVar2 + 1) {
        FUN_0010d500(DAT_00126258 + lVar2 * 8,0x28,0);
      }
    }
  }
  DAT_00126730 = param_1;
  DAT_00126728 = param_2;
  return;
}




// Function: print_sdiff_hunk @ 0xd860

void print_sdiff_hunk(undefined8 param_1)

{
  long lVar1;
  long lVar2;
  uint uVar3;
  long lVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010f670(param_1,&local_40,&local_38,&local_30,&local_28);
  lVar4 = DAT_00126730;
  if (uVar3 != 0) {
    FUN_0010d700(local_40,local_30);
    if (DAT_0012643a != '\0') {
      __fprintf_chk(DAT_00126188,1,"c%ld,%ld\n",(local_38 - local_40) + 1,(local_28 - local_30) + 1)
      ;
    }
    if (uVar3 == 3) {
      lVar4 = local_40;
      lVar5 = local_30;
      if (local_40 <= local_38) {
        do {
          local_40 = lVar4;
          local_30 = lVar5;
          if (local_28 < lVar5) break;
          lVar1 = lVar5 * 8;
          lVar5 = lVar5 + 1;
          lVar2 = lVar4 * 8;
          lVar4 = lVar4 + 1;
          FUN_0010d500(DAT_00126258 + lVar2,0x7c,DAT_00126388 + lVar1);
          local_40 = lVar4;
          local_30 = lVar5;
        } while (lVar4 <= local_38);
      }
      uVar3 = (uint)(local_40 <= local_38) + (uint)(local_30 <= local_28) * 2;
      DAT_00126728 = local_30;
      DAT_00126730 = local_40;
    }
    lVar4 = DAT_00126728;
    if (((uVar3 & 2) != 0) && (lVar5 = local_30, lVar4 = local_30, local_30 <= local_28)) {
      do {
        lVar4 = lVar5 + 1;
        FUN_0010d500(0,0x3e,DAT_00126388 + lVar5 * 8);
        lVar5 = lVar4;
      } while (lVar4 <= local_28);
    }
    DAT_00126728 = lVar4;
    lVar4 = DAT_00126730;
    if (((uVar3 & 1) != 0) && (lVar4 = local_40, local_40 <= local_38)) {
      do {
        lVar5 = lVar4 * 8;
        lVar4 = lVar4 + 1;
        FUN_0010d500(DAT_00126258 + lVar5,0x3c,0);
      } while (lVar4 <= local_38);
    }
  }
  DAT_00126730 = lVar4;
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_sdiff_script @ 0xda50

void print_sdiff_script(undefined8 param_1)

{
  FUN_0010e830();
  DAT_00126728 = -DAT_00126288;
  DAT_00126730 = DAT_00126728;
  FUN_0010f080(param_1,FUN_0010f060,FUN_0010d860);
  FUN_0010d700(DAT_00126270,DAT_001263a0);
  return;
}




// Function: sighandler @ 0xdaa0

void sighandler(int param_1)

{
  if (DAT_00126774 == 0) {
    DAT_00126774 = param_1;
  }
  return;
}




// Function: get_funky_string @ 0xdac0

undefined4 get_funky_string(long *param_1,long *param_2,undefined4 param_3,long *param_4)

{
  byte bVar1;
  char *pcVar2;
  char *pcVar3;
  char cVar4;
  char cVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  char *pcVar11;
  bool bVar12;
  
  pcVar2 = (char *)*param_2;
  lVar10 = *param_1;
  lVar8 = 1;
  cVar4 = *pcVar2;
  lVar9 = 0;
  lVar7 = lVar10 + 1;
  cVar5 = cVar4 + -0x5c;
  bVar12 = false;
  pcVar3 = pcVar2;
  lVar6 = lVar7;
  if (cVar4 == '\\') goto LAB_0010db26;
LAB_0010dafa:
  if (bVar12 || SBORROW1(cVar4,'\\') != cVar5 < '\0') {
    if (cVar4 == '=') {
      if ((char)param_3 != '\0') goto LAB_0010db5e;
    }
    else if ((cVar4 < '>') && ((cVar4 == '\0' || (cVar4 == ':')))) {
      param_3 = 1;
LAB_0010db5e:
      *param_1 = lVar10;
      *param_2 = (long)pcVar2;
      *param_4 = lVar9;
      return param_3;
    }
  }
  else if (cVar4 == '^') {
    bVar1 = pcVar2[1];
    if ((byte)(bVar1 - 0x40) < 0x3f) {
      pcVar2 = pcVar2 + 2;
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(byte *)(lVar6 + -1) = bVar1 & 0x1f;
      goto LAB_0010db16;
    }
    pcVar2 = pcVar2 + 1;
    if (bVar1 == 0x3f) {
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(undefined1 *)(lVar6 + -1) = 0x7f;
      goto LAB_0010db16;
    }
    param_3 = 0;
    goto LAB_0010db5e;
  }
  pcVar2 = pcVar2 + 1;
  lVar7 = lVar6;
LAB_0010db0b:
  *(char *)(lVar7 + -1) = cVar4;
  lVar8 = lVar8 + 1;
  lVar7 = lVar7 + 1;
LAB_0010db16:
  do {
    cVar4 = *pcVar2;
    lVar10 = lVar7 + -1;
    lVar9 = lVar8 + -1;
    cVar5 = cVar4 + -0x5c;
    bVar12 = cVar5 == '\0';
    pcVar3 = pcVar2;
    lVar6 = lVar7;
    if (!bVar12) goto LAB_0010dafa;
LAB_0010db26:
    cVar5 = pcVar3[1];
    pcVar2 = pcVar3 + 2;
    if (cVar5 == '\0') {
      param_3 = 0;
      goto LAB_0010db5e;
    }
    cVar4 = cVar5 + -0x30;
    switch(cVar4) {
    case '\0':
    case '\x01':
    case '\x02':
    case '\x03':
    case '\x04':
    case '\x05':
    case '\x06':
    case '\a':
      cVar5 = *pcVar2;
      if (7 < (byte)(cVar5 - 0x30U)) goto LAB_0010db0b;
      do {
        pcVar2 = pcVar2 + 1;
        cVar4 = cVar5 + -0x30 + cVar4 * '\b';
        cVar5 = *pcVar2;
      } while ((byte)(cVar5 - 0x30U) < 8);
      *(char *)(lVar7 + -1) = cVar4;
      lVar8 = lVar8 + 1;
      lVar7 = lVar7 + 1;
      goto LAB_0010db16;
    default:
      goto switchD_0010db4b_caseD_8;
    case '\x0f':
      cVar5 = '\x7f';
      break;
    case '(':
    case 'H':
      goto switchD_0010db4b_caseD_28;
    case '/':
      cVar5 = ' ';
      break;
    case '1':
      cVar5 = '\a';
      goto switchD_0010db4b_caseD_8;
    case '2':
      cVar5 = '\b';
      goto switchD_0010db4b_caseD_8;
    case '5':
      cVar5 = '\x1b';
      break;
    case '6':
      cVar5 = '\f';
      break;
    case '>':
      cVar5 = '\n';
      break;
    case 'B':
      cVar5 = '\r';
      break;
    case 'D':
      cVar5 = '\t';
      break;
    case 'F':
      cVar5 = '\v';
switchD_0010db4b_caseD_8:
    }
    *(char *)(lVar7 + -1) = cVar5;
    lVar8 = lVar8 + 1;
    lVar7 = lVar7 + 1;
  } while( true );
switchD_0010db4b_caseD_28:
  cVar5 = pcVar3[2];
  pcVar11 = pcVar3 + 3;
  cVar4 = '\0';
  pcVar2 = pcVar3 + 2;
  if ('F' < cVar5) goto LAB_0010dc82;
  do {
    if (cVar5 < 'A') {
      if (9 < (byte)(cVar5 - 0x30U)) goto LAB_0010db0b;
      cVar4 = cVar5 + -0x30 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
    else {
      cVar4 = cVar5 + -0x37 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
    while( true ) {
      cVar5 = *pcVar2;
      pcVar11 = pcVar2 + 1;
      if (cVar5 < 'G') break;
LAB_0010dc82:
      if (5 < (byte)(cVar5 + 0x9fU)) goto LAB_0010db0b;
      cVar4 = cVar5 + -0x57 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
  } while( true );
}




// Function: check_color_output @ 0xdd30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void check_color_output(char param_1)

{
  char cVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  int *piVar5;
  int *piVar6;
  char *pcVar7;
  long lVar8;
  long in_FS_OFFSET;
  bool bVar9;
  char *local_f8;
  undefined *local_f0;
  _union_1457 local_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined4 local_60;
  char local_43 [3];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00126188 == (FILE *)0x0) || (DAT_0012657c == 0)) goto LAB_0010df1c;
  if (DAT_00126180 == '\0') {
    if (param_1 == '\0') {
      iVar2 = fileno(DAT_00126188);
      iVar2 = isatty(iVar2);
      bVar9 = iVar2 != 0;
      goto LAB_0010dd84;
    }
    if (DAT_0012657c != 2) {
      DAT_00126758 = 0;
      goto LAB_0010df1c;
    }
    bVar9 = false;
LAB_0010dd9f:
    pcVar7 = DAT_00126748;
    DAT_00126758 = 1;
    local_f8 = DAT_00126748;
    if ((DAT_00126748 != (char *)0x0) && (*DAT_00126748 != '\0')) {
      local_43[2] = 0;
      local_43[0] = '?';
      local_43[1] = '?';
      local_f0 = (undefined *)FUN_00116ea0(DAT_00126748);
      while( true ) {
        while( true ) {
          while (cVar1 = *pcVar7, cVar1 == '*') {
            lVar8 = FUN_00116a40(0x28);
            *(long *)(lVar8 + 0x20) = DAT_00126750;
            DAT_00126750 = lVar8;
            *(undefined **)(lVar8 + 8) = local_f0;
            local_f8 = pcVar7 + 1;
            cVar1 = FUN_0010dac0(&local_f0,&local_f8,1,lVar8);
            if ((cVar1 == '\0') ||
               (pcVar7 = local_f8 + 1, cVar1 = *local_f8, local_f8 = pcVar7, cVar1 != '='))
            goto LAB_0010deca;
            *(undefined **)(lVar8 + 0x18) = local_f0;
            cVar1 = FUN_0010dac0(&local_f0,&local_f8,0,lVar8 + 0x10);
            pcVar7 = local_f8;
            if (cVar1 == '\0') goto LAB_0010deca;
          }
          if (cVar1 != ':') break;
          local_f8 = pcVar7 + 1;
          pcVar7 = local_f8;
        }
        if (cVar1 == '\0') goto LAB_0010df18;
        local_43[0] = cVar1;
        local_f8 = pcVar7 + 1;
        if (pcVar7[1] == '\0') break;
        local_43[1] = pcVar7[1];
        local_f8 = pcVar7 + 3;
        if (pcVar7[2] != '=') break;
        lVar8 = 0;
        pcVar7 = "lc";
        while (iVar2 = strcmp(local_43,pcVar7), iVar2 != 0) {
          lVar8 = lVar8 + 1;
          pcVar7 = (&PTR_DAT_00125920)[lVar8];
          if (pcVar7 == (char *)0x0) goto LAB_0010dea6;
        }
        (&PTR_DAT_00126028)[(long)(int)lVar8 * 2] = local_f0;
        cVar1 = FUN_0010dac0(&local_f0,&local_f8,0);
        pcVar7 = local_f8;
        if (cVar1 == '\0') {
LAB_0010dea6:
          uVar4 = dcgettext(0,"unrecognized prefix: %s",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar4,local_43);
        }
      }
LAB_0010deca:
      uVar4 = dcgettext(0,"unparsable value for --palette",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar4);
    }
  }
  else {
    bVar9 = true;
LAB_0010dd84:
    if (DAT_0012657c == 2) goto LAB_0010dd9f;
    if ((DAT_0012657c == 1) && (bVar9)) {
      bVar9 = true;
      goto LAB_0010dd9f;
    }
    DAT_00126758 = 0;
  }
LAB_0010df18:
  if (bVar9) {
    iVar2 = 0x14;
    piVar5 = &DAT_0011e524;
    sigemptyset((sigset_t *)&DAT_00126780);
    piVar6 = &DAT_0011e524;
    while( true ) {
      sigaction(iVar2,(sigaction *)0x0,(sigaction *)&local_e8);
      if (local_e8.sa_handler != (__sighandler_t)0x1) {
        sigaddset((sigset_t *)&DAT_00126780,iVar2);
      }
      if (piVar6 == (int *)&DAT_0011e550) break;
      iVar2 = *piVar6;
      piVar6 = piVar6 + 1;
    }
    local_60 = 0x10000000;
    iVar2 = 0x14;
    local_e0 = _DAT_00126780;
    uStack_d8 = uRam0000000000126788;
    local_d0 = _DAT_00126790;
    uStack_c8 = uRam0000000000126798;
    local_c0 = _DAT_001267a0;
    uStack_b8 = uRam00000000001267a8;
    local_b0 = _DAT_001267b0;
    uStack_a8 = uRam00000000001267b8;
    local_a0 = _DAT_001267c0;
    uStack_98 = uRam00000000001267c8;
    local_90 = _DAT_001267d0;
    uStack_88 = uRam00000000001267d8;
    local_80 = _DAT_001267e0;
    uStack_78 = uRam00000000001267e8;
    local_70 = _DAT_001267f0;
    uStack_68 = uRam00000000001267f8;
    while( true ) {
      iVar3 = sigismember((sigset_t *)&DAT_00126780,iVar2);
      if (iVar3 != 0) {
        local_e8.sa_handler = FUN_0010daa0;
        if (iVar2 == 0x14) {
          local_e8.sa_handler = FUN_0010e1a0;
        }
        sigaction(iVar2,(sigaction *)&local_e8,(sigaction *)0x0);
      }
      if (piVar5 == (int *)&DAT_0011e550) break;
      iVar2 = *piVar5;
      piVar5 = piVar5 + 1;
    }
  }
LAB_0010df1c:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: put_indicator @ 0xe180

void put_indicator(size_t *param_1)

{
  fwrite_unlocked((void *)param_1[1],*param_1,1,DAT_00126188);
  return;
}




// Function: process_signals @ 0xe1c0

void process_signals(void)

{
  int __sig;
  long in_FS_OFFSET;
  sigset_t sStack_c8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_00126774 != 0 || (DAT_00126770 != 0))) {
    if ((DAT_00126758 != '\0') && (DAT_00126010 != 3)) {
      FUN_0010e180(&DAT_00126020);
      FUN_0010e180(&DAT_00126050);
      FUN_0010e180(&DAT_00126030);
      DAT_00126010 = 3;
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_00126780,&sStack_c8);
    __sig = DAT_00126774;
    if (DAT_00126770 == 0) {
      signal(DAT_00126774,(__sighandler_t)0x0);
    }
    else {
      DAT_00126770 = DAT_00126770 + -1;
      __sig = 0x13;
    }
    raise(__sig);
    sigprocmask(2,&sStack_c8,(sigset_t *)0x0);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: c_escape @ 0xe2e0

byte * c_escape(byte *param_1)

{
  bool bVar1;
  byte *pbVar2;
  byte bVar3;
  byte *pbVar4;
  byte *pbVar5;
  long lVar6;
  
  bVar3 = *param_1;
  if (bVar3 != 0) {
    lVar6 = 0;
    bVar1 = false;
    pbVar2 = param_1;
LAB_0010e314:
    do {
      pbVar4 = pbVar2;
      if (bVar3 == 0x20) {
        bVar1 = true;
      }
      else {
        if (bVar3 != 0x22) {
          if ((char)bVar3 < '#') {
            if (6 < (byte)(bVar3 - 7)) {
              if ((char)bVar3 < ' ') {
                lVar6 = lVar6 + 3;
              }
              goto LAB_0010e308;
            }
          }
          else if (bVar3 != 0x5c) {
            bVar3 = pbVar4[1];
            pbVar2 = pbVar4 + 1;
            if (bVar3 == 0) break;
            goto LAB_0010e314;
          }
        }
        lVar6 = lVar6 + 1;
      }
LAB_0010e308:
      bVar3 = pbVar4[1];
      pbVar2 = pbVar4 + 1;
    } while (bVar3 != 0);
    if ((lVar6 != 0) || (bVar1)) {
      pbVar4 = (byte *)FUN_00116a40(pbVar4 + lVar6 + (4 - (long)param_1));
      *pbVar4 = 0x22;
      bVar3 = *param_1;
      pbVar2 = pbVar4 + 1;
      do {
        if (bVar3 == 0) {
          pbVar2[0] = 0x22;
          pbVar2[1] = 0;
          return pbVar4;
        }
        if ('\"' < (char)bVar3) {
          if (bVar3 == 0x5c) goto switchD_0010e39e_caseD_22;
LAB_0010e484:
          *pbVar2 = bVar3;
          pbVar5 = pbVar2 + 1;
          goto LAB_0010e3d9;
        }
        if ((char)bVar3 < '\a') {
switchD_0010e39e_caseD_e:
          if ('\x1f' < (char)bVar3) goto LAB_0010e484;
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 4;
          pbVar2[1] = (bVar3 >> 6) + 0x30;
          pbVar2[3] = (bVar3 & 7) + 0x30;
          pbVar2[2] = ((char)bVar3 >> 3 & 7U) + 0x30;
          goto LAB_0010e3d9;
        }
        switch(bVar3) {
        case 7:
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 2;
          pbVar2[1] = 0x61;
          break;
        case 8:
          bVar3 = 0x62;
        case 0x22:
switchD_0010e39e_caseD_22:
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 2;
          pbVar2[1] = bVar3;
          break;
        case 9:
          bVar3 = 0x74;
          goto switchD_0010e39e_caseD_22;
        case 10:
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 2;
          pbVar2[1] = 0x6e;
          break;
        case 0xb:
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 2;
          pbVar2[1] = 0x76;
          break;
        case 0xc:
          *pbVar2 = 0x5c;
          pbVar5 = pbVar2 + 2;
          pbVar2[1] = 0x66;
          break;
        case 0xd:
          bVar3 = 0x72;
          goto switchD_0010e39e_caseD_22;
        default:
          goto switchD_0010e39e_caseD_e;
        }
LAB_0010e3d9:
        bVar3 = param_1[1];
        param_1 = param_1 + 1;
        pbVar2 = pbVar5;
      } while( true );
    }
  }
  return param_1;
}




// Function: perror_with_name @ 0xe490

void perror_with_name(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar1,"%s",param_1);
}




// Function: message5 @ 0xe4c0

void message5(char *param_1,long param_2,undefined8 param_3,undefined1 *param_4,
                 undefined1 *param_5)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  void *pvVar4;
  undefined8 *puVar5;
  long lVar6;
  char *pcVar7;
  long lVar8;
  long in_FS_OFFSET;
  long local_80 [2];
  undefined1 *local_70;
  undefined1 *local_68;
  size_t local_58 [5];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126480 == '\0') {
    if (DAT_0012643a != '\0') {
      pcVar7 = stdout->_IO_write_ptr;
      if (pcVar7 < stdout->_IO_write_end) {
        stdout->_IO_write_ptr = pcVar7 + 1;
        *pcVar7 = ' ';
      }
      else {
        __overflow(stdout,0x20);
      }
    }
    uVar2 = dcgettext(0,param_1,5);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      __printf_chk(1,uVar2,param_2,param_3,param_4,param_5);
      return;
    }
  }
  else {
    lVar8 = 8;
    if (param_4 == (undefined1 *)0x0) {
      param_4 = &DAT_0011e5b0;
    }
    if (param_5 == (undefined1 *)0x0) {
      param_5 = &DAT_0011e5b0;
    }
    lVar6 = 0;
    pcVar7 = param_1;
    local_80[0] = param_2;
    local_80[1] = param_3;
    local_70 = param_4;
    local_68 = param_5;
    while( true ) {
      sVar1 = strlen(pcVar7);
      *(size_t *)((long)local_58 + lVar6) = sVar1 + 1;
      lVar8 = lVar8 + sVar1 + 1;
      if (lVar6 + 8 == 0x28) break;
      pcVar7 = *(char **)((long)local_80 + lVar6);
      lVar6 = lVar6 + 8;
    }
    puVar3 = (undefined8 *)FUN_00116a40(lVar8);
    puVar5 = puVar3 + 1;
    lVar8 = 0;
    while( true ) {
      sVar1 = *(size_t *)((long)local_58 + lVar8);
      pvVar4 = memcpy(puVar5,param_1,sVar1);
      puVar5 = (undefined8 *)((long)pvVar4 + sVar1);
      if (lVar8 + 8 == 0x28) break;
      param_1 = *(char **)((long)local_80 + lVar8);
      lVar8 = lVar8 + 8;
    }
    puVar5 = puVar3;
    *(undefined8 **)PTR_DAT_001260a0 = puVar3;
    PTR_DAT_001260a0 = (undefined *)puVar5;
    *puVar3 = 0;
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: message @ 0xe660

void message(void)

{
  FUN_0010e4c0();
  return;
}




// Function: print_message_queue @ 0xe670

void print_message_queue(void)

{
  long *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  size_t sVar6;
  undefined8 uVar7;
  long *__s;
  long *__ptr;
  long *plVar8;
  long in_FS_OFFSET;
  long local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126800 != (long *)0x0) {
    __ptr = DAT_00126800;
    do {
      plVar1 = (long *)*__ptr;
      __s = __ptr + 1;
      plVar8 = &local_68;
      do {
        plVar8 = plVar8 + 1;
        sVar6 = strlen((char *)__s);
        lVar5 = local_48;
        uVar4 = local_50;
        uVar3 = local_58;
        uVar2 = local_60;
        __s = (long *)((long)__s + sVar6 + 1);
        *plVar8 = (long)__s;
      } while (&local_48 != plVar8);
      uVar7 = dcgettext(0,__ptr + 1,5);
      __printf_chk(1,uVar7,uVar2,uVar3,uVar4,lVar5);
      free(__ptr);
      __ptr = plVar1;
    } while (plVar1 != (long *)0x0);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pfatal_with_name @ 0xe770

void pfatal_with_name(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010e670();
                    /* WARNING: Subroutine does not return */
  error(2,iVar1,"%s",param_1);
}




// Function: fatal @ 0xe7b0

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  FUN_0010e670();
  uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
  error(2,0,"%s",uVar1);
}




// Function: set_color_palette @ 0xe7f0

void set_color_palette(undefined8 param_1)

{
  DAT_00126748 = param_1;
  return;
}




// Function: setup_output @ 0xe800

void setup_output(undefined8 param_1,undefined8 param_2,undefined1 param_3)

{
  DAT_00126768 = param_1;
  DAT_00126760 = param_2;
  DAT_00126759 = param_3;
  DAT_00126188 = 0;
  return;
}




// Function: begin_output @ 0xe830

void begin_output(void)

{
  int iVar1;
  void *__ptr;
  int *piVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  int local_60;
  int local_5c;
  void *local_58;
  void *local_50;
  char *local_48;
  undefined *local_40;
  void *local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126188 == (FILE *)0x0) {
    local_58 = (void *)FUN_0010e2e0(DAT_00126768);
    local_50 = (void *)FUN_0010e2e0(DAT_00126760);
    __ptr = (void *)FUN_00117020("diff%s %s %s",DAT_00126420,local_58,local_50);
    if (DAT_00126480 == '\0') {
      DAT_00126188 = stdout;
      FUN_0010dd30(0);
      if (DAT_00126759 != '\0') {
        __printf_chk(1,&DAT_0011c2a1,__ptr);
      }
    }
    else {
      iVar1 = fflush_unlocked(stdout);
      if (iVar1 != 0) {
        uVar3 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
        FUN_0010e770(uVar3);
      }
      local_48 = "/usr/bin/pr";
      local_40 = &DAT_0011e58c;
      local_30 = 0;
      local_38 = __ptr;
      iVar1 = pipe(&local_60);
      if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e770(&DAT_0011e58f);
      }
      DAT_00126740 = fork();
      if (DAT_00126740 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e770(&DAT_0011e594);
      }
      if (DAT_00126740 == 0) {
        close(local_5c);
        if (local_60 != 0) {
          iVar1 = dup2(local_60,0);
          if (iVar1 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_0010e770(&DAT_0011e599);
          }
          close(local_60);
        }
        execv("/usr/bin/pr",&local_48);
        piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
        _exit((*piVar2 == 2) + 0x7e);
      }
      close(local_60);
      DAT_00126188 = fdopen(local_5c,"w");
      if (DAT_00126188 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e770("fdopen");
      }
      FUN_0010dd30(1);
    }
    free(__ptr);
    if (DAT_00126580 == 2) {
      FUN_00108b80(&DAT_001261a0,&local_58,0);
    }
    else if (DAT_00126580 == 3) {
      FUN_00108b80(&DAT_001261a0,&local_58,1);
    }
    if (local_58 != DAT_00126768) {
      free(local_58);
    }
    if (local_50 != DAT_00126760) {
      free(local_50);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: finish_output @ 0xead0

void finish_output(void)

{
  byte bVar4;
  int iVar1;
  __pid_t _Var2;
  undefined8 uVar3;
  uint uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  uint local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00126188 == (FILE *)0x0) || (DAT_00126188 == stdout)) {
LAB_0010eb42:
    DAT_00126188 = (FILE *)0x0;
    if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if ((DAT_00126188->_flags & 0x20) == 0) {
    iVar1 = fclose(DAT_00126188);
    if (iVar1 != 0) {
      uVar3 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_0010e770(uVar3);
    }
    _Var2 = waitpid(DAT_00126740,(int *)&local_14,0);
    if (_Var2 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010e770("waitpid");
    }
    if ((local_14 & 0x7f) == 0) {
      bVar4 = (byte)(local_14 >> 8);
      if (bVar4 == 0) goto LAB_0010eb42;
      pcVar6 = "subsidiary program \'%s\' could not be invoked";
      uVar5 = (uint)bVar4;
      if ((bVar4 != 0x7e) &&
         (pcVar6 = "subsidiary program \'%s\' failed (exit status %d)", uVar5 = (uint)bVar4,
         bVar4 == 0x7f)) {
        pcVar6 = "subsidiary program \'%s\' not found";
      }
      goto LAB_0010eb6f;
    }
  }
  else {
    FUN_0010e7b0("write failed");
  }
  pcVar6 = "subsidiary program \'%s\' failed";
  uVar5 = 0x7fffffff;
LAB_0010eb6f:
  uVar3 = dcgettext(0,pcVar6,5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3,"/usr/bin/pr",uVar5);
}




// Function: lines_differ @ 0xec00

undefined8 lines_differ(byte *param_1,byte *param_2)

{
  byte *pbVar1;
  byte bVar2;
  ushort *puVar3;
  ulong uVar4;
  char cVar5;
  uint uVar6;
  uint uVar7;
  ushort **ppuVar8;
  __int32_t **pp_Var9;
  long lVar10;
  byte bVar11;
  bool bVar12;
  byte bVar13;
  char cVar14;
  ulong uVar15;
  byte *pbVar16;
  byte *pbVar17;
  ulong uVar18;
  byte *pbVar19;
  byte *pbVar20;
  ulong uVar21;
  
  uVar6 = DAT_00126558;
  cVar5 = DAT_00126552;
  uVar4 = DAT_00126498;
  uVar21 = 0;
  uVar7 = DAT_00126558 - 2;
  pbVar19 = param_2;
  pbVar20 = param_1;
LAB_0010ec50:
  do {
    bVar11 = *pbVar20;
    bVar13 = *pbVar19;
    uVar15 = (ulong)bVar13;
    pbVar17 = pbVar20 + 1;
    pbVar16 = pbVar19 + 1;
    if (bVar11 == bVar13) goto LAB_0010ed3a;
    if (uVar6 != 4) {
      if (uVar6 < 5) {
        if (uVar6 == 1) {
LAB_0010edf0:
          uVar18 = uVar21;
          if ((bVar11 == 0x20) && (bVar13 == 9)) {
LAB_0010ee3a:
            do {
              if (bVar11 == 0x20) {
                uVar18 = uVar18 + 1;
              }
              else {
                if (bVar11 != 9) goto LAB_0010ee6c;
                uVar18 = (uVar18 + uVar4) - uVar18 % uVar4;
              }
              bVar11 = *pbVar17;
              pbVar17 = pbVar17 + 1;
            } while( true );
          }
          if ((bVar11 == 9) && (bVar13 == 0x20)) {
            bVar11 = 9;
            goto LAB_0010ee3a;
          }
        }
        else if (uVar7 < 2) {
          ppuVar8 = __ctype_b_loc();
          puVar3 = *ppuVar8;
          if (((*(byte *)((long)puVar3 + (ulong)bVar11 * 2 + 1) & 0x20) == 0) ||
             ((*(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1) & 0x20) == 0)) {
            if (uVar6 != 2) goto LAB_0010edf0;
          }
          else {
            if (bVar11 != 10) {
              bVar2 = pbVar20[1];
              pbVar20 = pbVar17;
              while (bVar2 != 10) {
                if ((*(byte *)((long)puVar3 + (ulong)bVar2 * 2 + 1) & 0x20) == 0) goto LAB_0010ed08;
                pbVar1 = pbVar20 + 1;
                pbVar20 = pbVar20 + 1;
                bVar2 = *pbVar1;
              }
            }
            if ((bVar13 == 10) || (bVar13 = pbVar19[1], pbVar19 = pbVar16, bVar13 == 10)) {
              return 0;
            }
            while ((*(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1) & 0x20) != 0) {
              bVar13 = pbVar19[1];
              pbVar19 = pbVar19 + 1;
              if (bVar13 == 10) {
                return 0;
              }
            }
          }
        }
      }
      else if (uVar6 == 5) {
        ppuVar8 = __ctype_b_loc();
        puVar3 = *ppuVar8;
        while (((*(byte *)((long)puVar3 + (ulong)bVar11 * 2 + 1) & 0x20) != 0 && (bVar11 != 10))) {
          bVar11 = *pbVar17;
          pbVar17 = pbVar17 + 1;
        }
        bVar13 = *(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1);
        while (((bVar13 & 0x20) != 0 && ((char)uVar15 != '\n'))) {
          uVar15 = (ulong)*pbVar16;
          pbVar16 = pbVar16 + 1;
          bVar13 = *(byte *)((long)puVar3 + uVar15 * 2 + 1);
        }
      }
      goto LAB_0010ed08;
    }
    ppuVar8 = __ctype_b_loc();
    puVar3 = *ppuVar8;
    uVar18 = (ulong)bVar11;
    pbVar20 = pbVar17;
    pbVar19 = pbVar16;
    if ((*(byte *)((long)puVar3 + uVar18 * 2 + 1) & 0x20) == 0) {
      if ((puVar3[bVar13] & 0x2000) != 0) {
LAB_0010ef8f:
        do {
          pbVar16 = pbVar19;
          bVar11 = (byte)uVar18;
          if ((char)uVar15 == '\n') goto LAB_0010ed08;
          uVar15 = (ulong)*pbVar16;
          pbVar19 = pbVar16 + 1;
        } while ((*(byte *)((long)puVar3 + uVar15 * 2 + 1) & 0x20) != 0);
        uVar15 = 0x20;
        goto LAB_0010ef65;
      }
    }
    else {
      do {
        pbVar17 = pbVar20;
        if ((char)uVar18 == '\n') goto LAB_0010ef5f;
        uVar18 = (ulong)*pbVar17;
        pbVar20 = pbVar17 + 1;
      } while ((*(byte *)((long)puVar3 + uVar18 * 2 + 1) & 0x20) != 0);
      uVar18 = 0x20;
LAB_0010ef5f:
      bVar11 = (byte)uVar18;
      if ((puVar3[bVar13] & 0x2000) != 0) goto LAB_0010ef8f;
LAB_0010ef65:
      if ((byte)uVar15 == bVar11) goto LAB_0010ed08;
    }
    cVar14 = (char)uVar15;
    bVar12 = cVar14 == ' ' && bVar11 != 10;
    if (cVar14 != ' ' || bVar11 == 10) {
      bVar12 = cVar14 != '\n';
      break;
    }
    if (pbVar17 <= param_1 + 1) {
      uVar15 = 0x20;
      break;
    }
    uVar15 = 0x20;
    pbVar19 = pbVar16;
    pbVar20 = pbVar17 + -1;
  } while ((*(byte *)((long)puVar3 + (ulong)pbVar17[-2] * 2 + 1) & 0x20) != 0);
  if ((bVar11 == 0x20) && (bVar12)) {
    if (param_2 + 1 < pbVar16) {
      pbVar19 = pbVar16 + -1;
      pbVar20 = pbVar17;
      if ((*(byte *)((long)puVar3 + (ulong)pbVar16[-2] * 2 + 1) & 0x20) != 0) goto LAB_0010ec50;
    }
    else {
      bVar11 = 0x20;
    }
  }
LAB_0010ed08:
  if (cVar5 != '\0') {
    pp_Var9 = __ctype_tolower_loc();
    bVar11 = *(byte *)(*pp_Var9 + bVar11);
    uVar15 = (ulong)*(byte *)(*pp_Var9 + uVar15);
  }
  bVar13 = (byte)uVar15;
  if (bVar11 != bVar13) {
    return 1;
  }
LAB_0010ed3a:
  if (bVar13 == 10) {
    return 0;
  }
  lVar10 = 1;
  if (bVar13 == 9) {
    lVar10 = uVar4 - uVar21 % uVar4;
  }
  uVar21 = uVar21 + lVar10;
  pbVar19 = pbVar16;
  pbVar20 = pbVar17;
  goto LAB_0010ec50;
LAB_0010ee6c:
  if ((char)uVar15 == ' ') {
    uVar21 = uVar21 + 1;
  }
  else {
    if ((char)uVar15 != '\t') goto LAB_0010efa0;
    uVar21 = (uVar4 + uVar21) - uVar21 % uVar4;
  }
  uVar15 = (ulong)*pbVar16;
  pbVar16 = pbVar16 + 1;
  goto LAB_0010ee6c;
LAB_0010efa0:
  if (uVar18 != uVar21) {
    return 1;
  }
  goto LAB_0010ed08;
}




// Function: print_script @ 0xf080

void print_script(long param_1,code *param_2,code *param_3)

{
  long lVar1;
  long *plVar2;
  
  if (param_1 == 0) {
    return;
  }
  do {
    plVar2 = (long *)(*param_2)(param_1);
    lVar1 = *plVar2;
    *plVar2 = 0;
    (*param_3)(param_1);
    *plVar2 = lVar1;
    param_1 = lVar1;
  } while (lVar1 != 0);
  return;
}




// Function: output_1_line @ 0xf0e0

void output_1_line(byte *param_1,byte *param_2,long param_3,undefined8 param_4)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  _IO_FILE *p_Var4;
  size_t sVar5;
  ushort **ppuVar6;
  ulong uVar7;
  long lVar8;
  ulong uVar9;
  long lVar10;
  
  uVar7 = DAT_00126498;
  p_Var4 = DAT_00126188;
  if (DAT_001264a0 == '\0') {
    for (uVar7 = (long)param_2 - (long)param_1; uVar7 != 0; uVar7 = uVar7 - sVar5) {
      uVar9 = 0x400;
      if (uVar7 < 0x401) {
        uVar9 = uVar7;
      }
      sVar5 = fwrite_unlocked(param_1,1,uVar9,DAT_00126188);
      if (sVar5 < uVar9) {
        return;
      }
      param_1 = param_1 + sVar5;
      FUN_0010e1c0();
    }
  }
  else {
    uVar9 = 0;
    lVar10 = 1;
    if (param_1 < param_2) {
LAB_0010f1a0:
      bVar1 = *param_1;
      param_1 = param_1 + 1;
      if (bVar1 == 9) goto LAB_0010f23a;
      do {
        if (bVar1 == 0xd) {
          pcVar3 = p_Var4->_IO_write_ptr;
          if (pcVar3 < p_Var4->_IO_write_end) {
            p_Var4->_IO_write_ptr = pcVar3 + 1;
            *pcVar3 = '\r';
          }
          else {
            __overflow(p_Var4,0xd);
          }
          if ((param_3 != 0) && (param_1 < param_2)) {
            uVar9 = 0;
            if (*param_1 == 10) goto LAB_0010f213;
            __fprintf_chk(p_Var4,1,param_3,param_4);
            goto LAB_0010f213;
          }
LAB_0010f300:
          uVar9 = 0;
        }
        else if (bVar1 == 8) {
          if (uVar9 == 0) goto LAB_0010f300;
          pcVar3 = p_Var4->_IO_write_ptr;
          uVar9 = uVar9 - 1;
          if (p_Var4->_IO_write_end <= pcVar3) goto LAB_0010f310;
          p_Var4->_IO_write_ptr = pcVar3 + 1;
          *pcVar3 = '\b';
        }
        else {
          ppuVar6 = __ctype_b_loc();
          uVar9 = uVar9 + ((*ppuVar6)[bVar1] >> 0xe & 1);
          pbVar2 = (byte *)p_Var4->_IO_write_ptr;
          if (pbVar2 < p_Var4->_IO_write_end) {
            p_Var4->_IO_write_ptr = (char *)(pbVar2 + 1);
            *pbVar2 = bVar1;
          }
          else {
LAB_0010f310:
            __overflow(p_Var4,(uint)bVar1);
          }
        }
        while( true ) {
          if (param_2 == param_1) {
            return;
          }
LAB_0010f213:
          lVar10 = lVar10 + 1;
          if (lVar10 != 0x400) goto LAB_0010f1a0;
          FUN_0010e1c0();
          bVar1 = *param_1;
          param_1 = param_1 + 1;
          lVar10 = 0;
          if (bVar1 != 9) break;
LAB_0010f23a:
          lVar8 = uVar7 - uVar9 % uVar7;
          uVar9 = uVar9 + lVar8;
          do {
            pcVar3 = p_Var4->_IO_write_ptr;
            if (pcVar3 < p_Var4->_IO_write_end) {
              p_Var4->_IO_write_ptr = pcVar3 + 1;
              *pcVar3 = ' ';
            }
            else {
              __overflow(p_Var4,0x20);
            }
            lVar8 = lVar8 + -1;
          } while (lVar8 != 0);
        }
      } while( true );
    }
  }
  return;
}




// Function: print_1_line_nl @ 0xf330

void print_1_line_nl(char *param_1,undefined8 *param_2,char param_3)

{
  char cVar1;
  char *pcVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *pcVar6;
  char *pcVar7;
  char *pcVar8;
  
  uVar4 = DAT_00126188;
  pcVar2 = (char *)*param_2;
  lVar3 = param_2[1];
  if (param_1 == (char *)0x0) {
    if (param_3 == '\0') {
      FUN_0010f0e0(pcVar2,lVar3,0,0);
    }
    else {
      FUN_0010f0e0(pcVar2,lVar3 - (ulong)(*(char *)(lVar3 + -1) == '\n'),0,0);
    }
  }
  else {
    pcVar8 = (char *)0x0;
    if (*param_1 != '\0') {
      pcVar8 = "%s\t";
      if (DAT_00126492 == '\0') {
        pcVar8 = "%s ";
      }
      pcVar6 = param_1;
      pcVar7 = pcVar8;
      if ((DAT_00126491 != '\0') && (*pcVar2 == '\n')) {
        pcVar7 = "%s";
        pcVar6 = param_1 + (*param_1 == ' ');
      }
      __fprintf_chk(DAT_00126188,1,pcVar7,pcVar6);
    }
    if (param_3 == '\0') {
      FUN_0010f0e0(pcVar2,lVar3,pcVar8,param_1);
      cVar1 = *param_1;
    }
    else {
      FUN_0010f0e0(pcVar2,lVar3 - (ulong)(*(char *)(lVar3 + -1) == '\n'),pcVar8,param_1);
      cVar1 = *param_1;
    }
    if (cVar1 == '\0') {
      return;
    }
  }
  if (*(char *)(lVar3 + -1) == '\n') {
    return;
  }
  if ((DAT_00126758 != '\0') && (DAT_00126010 != 3)) {
    FUN_0010e180(&DAT_00126020);
    FUN_0010e180(&DAT_00126050);
    FUN_0010e180(&DAT_00126030);
    DAT_00126010 = 3;
  }
  uVar5 = dcgettext(0,"No newline at end of file",5);
  __fprintf_chk(uVar4,1,"\n\\ %s\n",uVar5);
  return;
}




// Function: print_1_line @ 0xf500

void print_1_line(undefined8 param_1,undefined8 param_2)

{
  FUN_0010f330(param_1,param_2,0);
  return;
}




// Function: print_number_range @ 0xf610

void print_number_range(char param_1,long param_2,long param_3,long param_4)

{
  param_3 = *(long *)(param_2 + 0xe8) + 1 + param_3;
  param_4 = *(long *)(param_2 + 0xe8) + 1 + param_4;
  if (param_3 < param_4) {
    __fprintf_chk(DAT_00126188,1,"%ld%c%ld",param_3,(int)param_1);
    return;
  }
  __fprintf_chk(DAT_00126188,1,&DAT_0011c092,param_4);
  return;
}




// Function: analyze_hunk @ 0xf670

byte analyze_hunk(long *param_1,long *param_2,long *param_3,long *param_4,long *param_5)

{
  long lVar1;
  long lVar2;
  byte bVar3;
  byte *pbVar4;
  bool bVar5;
  bool bVar6;
  long lVar7;
  long lVar8;
  uint uVar9;
  ushort **ppuVar10;
  byte *pbVar11;
  byte bVar12;
  long lVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  byte local_69;
  int local_64;
  long local_60;
  long local_58;
  
  lVar8 = DAT_00126388;
  lVar7 = DAT_00126258;
  uVar9 = (uint)DAT_00126554;
  if (DAT_00126554 == 0) {
    local_64 = -1;
    bVar5 = false;
    uVar9 = (uint)(DAT_001264e0 != 0);
    bVar6 = false;
  }
  else if (DAT_00126558 < 2) {
    local_64 = 0;
    bVar6 = false;
    bVar5 = false;
  }
  else {
    local_64 = 0;
    bVar6 = true;
    bVar5 = 3 < DAT_00126558;
  }
  local_58 = 0;
  local_60 = 0;
  *param_2 = param_1[3];
  *param_4 = param_1[4];
  do {
    lVar13 = param_1[3];
    lVar16 = param_1[4];
    lVar1 = param_1[2] + -1 + lVar13;
    local_58 = local_58 + param_1[1];
    local_60 = local_60 + param_1[2];
    lVar2 = lVar16 + -1 + param_1[1];
    bVar12 = lVar13 <= lVar1 & (byte)uVar9;
    if (bVar12 == 0) {
      local_69 = lVar16 <= lVar2 & (byte)uVar9;
    }
    else {
      do {
        lVar16 = *(long *)(lVar7 + 8 + lVar13 * 8);
        pbVar4 = *(byte **)(lVar7 + lVar13 * 8);
        lVar16 = lVar16 + -1 + (ulong)(*(char *)(lVar16 + -1) != '\n');
        lVar15 = lVar16 - (long)pbVar4;
        lVar14 = lVar15;
        if ((bVar6) && (bVar3 = *pbVar4, bVar3 != 10)) {
          ppuVar10 = __ctype_b_loc();
          pbVar11 = pbVar4;
          do {
            if ((*(byte *)((long)*ppuVar10 + (ulong)bVar3 * 2 + 1) & 0x20) == 0) {
              lVar14 = lVar16 - (long)pbVar11;
              if (!bVar5) {
                lVar14 = lVar15;
              }
              goto LAB_0010f7b2;
            }
            bVar3 = pbVar11[1];
            pbVar11 = pbVar11 + 1;
          } while (bVar3 != 10);
          lVar14 = lVar16 - (long)pbVar11;
        }
LAB_0010f7b2:
        if (local_64 == lVar14) {
          uVar9 = (uint)bVar12;
        }
        else {
          if (DAT_001264e0 == 0) {
            uVar9 = 0;
            goto LAB_0010f7cb;
          }
          uVar9 = re_search((re_pattern_buffer *)&DAT_001264c0,(char *)pbVar4,(int)lVar15,0,
                            (int)lVar15,(re_registers *)0x0);
          uVar9 = ~uVar9 >> 0x1f;
        }
        lVar13 = lVar13 + 1;
      } while ((lVar13 <= lVar1) && ((byte)uVar9 != 0));
      lVar16 = param_1[4];
      local_69 = lVar16 <= lVar2 & (byte)uVar9;
    }
    if (local_69 != 0) {
      do {
        lVar13 = *(long *)(lVar8 + 8 + lVar16 * 8);
        pbVar4 = *(byte **)(lVar8 + lVar16 * 8);
        lVar13 = lVar13 + -1 + (ulong)(*(char *)(lVar13 + -1) != '\n');
        lVar15 = lVar13 - (long)pbVar4;
        lVar14 = lVar15;
        if ((bVar6) && (bVar12 = *pbVar4, bVar12 != 10)) {
          ppuVar10 = __ctype_b_loc();
          pbVar11 = pbVar4;
          do {
            if ((*(byte *)((long)*ppuVar10 + (ulong)bVar12 * 2 + 1) & 0x20) == 0) {
              lVar14 = lVar13 - (long)pbVar11;
              if (!bVar5) {
                lVar14 = lVar15;
              }
              goto LAB_0010f935;
            }
            bVar12 = pbVar11[1];
            pbVar11 = pbVar11 + 1;
          } while (bVar12 != 10);
          lVar14 = lVar13 - (long)pbVar11;
        }
LAB_0010f935:
        if (local_64 == lVar14) {
          uVar9 = (uint)local_69;
        }
        else {
          if (DAT_001264e0 == 0) {
            uVar9 = 0;
            break;
          }
          uVar9 = re_search((re_pattern_buffer *)&DAT_001264c0,(char *)pbVar4,(int)lVar15,0,
                            (int)lVar15,(re_registers *)0x0);
          uVar9 = ~uVar9 >> 0x1f;
        }
        lVar16 = lVar16 + 1;
        if ((lVar2 < lVar16) || ((char)uVar9 == '\0')) break;
      } while( true );
    }
LAB_0010f7cb:
    param_1 = (long *)*param_1;
    if (param_1 == (long *)0x0) {
      bVar12 = 0;
      *param_3 = lVar1;
      *param_5 = lVar2;
      if ((char)uVar9 == '\0') {
        bVar12 = (local_58 != 0) * '\x02' | local_60 != 0;
      }
      return bVar12;
    }
  } while( true );
}




// Function: concat @ 0xfa30

undefined1  [16] concat(char *param_1,char *param_2,char *param_3)

{
  size_t sVar1;
  size_t sVar2;
  size_t sVar3;
  undefined1 auVar4 [16];
  
  sVar1 = strlen(param_1);
  sVar2 = strlen(param_2);
  sVar3 = strlen(param_3);
  auVar4._0_8_ = FUN_00116a40(sVar1 + sVar2 + 1 + sVar3);
  __sprintf_chk(auVar4._0_8_,1,0xffffffffffffffff,"%s%s%s",param_1,param_2,param_3);
  auVar4._8_8_ = 0x10fa6a;
  return auVar4;
}




// Function: zalloc @ 0xfab0

void zalloc(size_t param_1)

{
  void *__s;
  
  __s = (void *)FUN_00116a40();
  memset(__s,0,param_1);
  return;
}




// Function: debug_script @ 0xfad0

void debug_script(undefined8 *param_1)

{
  fflush_unlocked(stdout);
  for (; param_1 != (undefined8 *)0x0; param_1 = (undefined8 *)*param_1) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",param_1[3],param_1[4],param_1[2],
                  param_1[1]);
  }
  fflush_unlocked(stderr);
  return;
}



