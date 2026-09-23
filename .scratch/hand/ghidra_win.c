    __assert_fail("! command_line_arg || inode == NOT_AN_INODE_NUMBER","src/ls.c",0xd07,
                  "gobble_file");
  }
  if (DAT_0012b338 == DAT_0012b330) {
    DAT_0012b328 = FUN_0011c638(DAT_0012b328,DAT_0012b330,0x1a0);
    DAT_0012b330 = DAT_0012b330 * 2;
  }
  local_380 = (undefined8 *)(DAT_0012b338 * 0xd0 + DAT_0012b328);
  memset(local_380,0,0xd0);
  local_380[4] = local_3e0;
  *(int *)(local_380 + 0x15) = local_3d4;
  *(undefined4 *)((long)local_380 + 0xc4) = 0xffffffff;
  if ((DAT_0012b340 != '\x01') && (DAT_0012b341 != '\0')) {
    bVar6 = FUN_001099b1(local_3d0);
    *(uint *)((long)local_380 + 0xc4) = (uint)bVar6;
    if (*(int *)((long)local_380 + 0xc4) != 0) {
      DAT_0012b340 = '\x01';
    }
  }
  if (((((((local_3d8 != '\0') || (DAT_0012b3d5 != '\0')) || (DAT_0012b440 != '\0')) ||
        (((local_3d4 == 3 && (DAT_0012b3d4 != '\0')) &&
         ((cVar7 = FUN_00106325(0x13), cVar7 != '\0' ||
          ((cVar7 = FUN_00106325(0x12), cVar7 != '\0' || (cVar7 = FUN_00106325(0x14), cVar7 != '\0')
           ))))))) ||
       (((DAT_0012b3e9 != '\0' || (DAT_0012b441 != '\0')) &&
        (((local_3d4 == 6 || (local_3d4 == 0)) &&
         ((DAT_0012b3ec == 4 || ((DAT_0012b358 != '\0' || (DAT_0012b3e8 != '\0')))))))))) ||
      ((DAT_0012b3e9 != '\0' && (local_3e0 == 0)))) ||
     ((ppcVar19 = &local_3e8, DAT_0012b441 != '\0' &&
      (((local_3d4 == 0 || (local_3d8 != '\0')) ||
       ((ppcVar19 = &local_3e8, local_3d4 == 5 &&
        ((DAT_0012b3d0 == 3 ||
         ((ppcVar19 = &local_3e8, DAT_0012b3d4 != '\0' &&
          ((((cVar7 = FUN_00106325(0xe), cVar7 != '\0' ||
             (cVar7 = FUN_00106325(0x10), cVar7 != '\0')) ||
            (cVar7 = FUN_00106325(0x11), cVar7 != '\0')) ||
           (cVar7 = FUN_00106325(0x15), ppcVar19 = &local_3e8, cVar7 != '\0')))))))))))))) {
    if ((*local_3d0 == '/') || (*local_3e8 == '\0')) {
      local_388 = local_3d0;
      ppcVar19 = &local_3e8;
    }
    else {
      sVar16 = strlen(local_3d0);
      sVar12 = strlen(local_3e8);
      uVar13 = ((sVar12 + sVar16 + 0x19) / 0x10) * 0x10;
      for (; ppcVar18 != (char **)((long)&local_3e8 - (uVar13 & 0xfffffffffffff000));
          ppcVar18 = (char **)((long)ppcVar18 + -0x1000)) {
        *(undefined8 *)((long)ppcVar18 + -8) = *(undefined8 *)((long)ppcVar18 + -8);
      }
      lVar3 = -(ulong)((uint)uVar13 & 0xfff);
      ppcVar19 = (char **)((long)ppcVar18 + lVar3);
      if ((uVar13 & 0xfff) != 0) {
        *(undefined8 *)((long)ppcVar18 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar3) =
             *(undefined8 *)((long)ppcVar18 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar3);
      }
      pcVar4 = local_3d0;
      pcVar17 = local_3e8;
      local_388 = (char *)((ulong)((long)ppcVar18 + lVar3 + 0xf) & 0xfffffffffffffff0);
      *(undefined8 *)((long)ppcVar18 + lVar3 + -8) = 0x109e47;
      FUN_0010eb94((char *)((ulong)((long)ppcVar18 + lVar3 + 0xf) & 0xfffffffffffffff0),pcVar17,
                   pcVar4);
    }
    pcVar17 = local_388;
    if (DAT_0012b3d5 != '\0') {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109e66;
      uVar14 = FUN_0011101b(pcVar17,2);
      local_380[2] = uVar14;
      if (local_380[2] == 0) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109e93;
        uVar14 = gettext("error canonicalizing %s");
        pcVar17 = local_388;
        cVar7 = local_3d8;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109eae;
        FUN_00108de7(cVar7,uVar14,pcVar17);
      }
    }
    pcVar17 = local_388;
    if (DAT_0012b3ec < 4) {
      if ((DAT_0012b3ec < 2) || (local_3d8 == '\0')) goto LAB_00109f7c;
      puVar1 = local_380 + 3;
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109f21;
      local_3c0 = FUN_00105c0e(pcVar17,puVar1);
      local_3c4 = 1;
      if (DAT_0012b3ec != 2) {
        if ((int)local_3c0 < 0) {
          *(undefined8 *)((long)ppcVar19 + -8) = 0x109f47;
          piVar15 = __errno_location();
          local_3c1 = *piVar15 == 2;
        }
        else {
          local_3c1 = (*(uint *)(local_380 + 6) & 0xf000) != 0x4000;
        }
        if ((bool)local_3c1 == true) goto LAB_00109f7c;
      }
    }
    else {
      if (DAT_0012b3ec == 4) {
        puVar1 = local_380 + 3;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109ee9;
        local_3c0 = FUN_00105c0e(pcVar17,puVar1);
