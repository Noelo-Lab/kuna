// Function: main @ 0x3ec0

/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Enum "__rlimit_resource": Some values do not have unique names */

undefined8 main(uint param_1,undefined **param_2)

{
  byte *pbVar1;
  ushort uVar2;
  __int32_t _Var3;
  __int32_t *p_Var4;
  ushort *puVar5;
  long lVar6;
  rlim_t rVar7;
  long *plVar8;
  char cVar9;
  char cVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  char *pcVar14;
  char *pcVar15;
  lconv *plVar16;
  ushort **ppuVar17;
  char *pcVar18;
  long lVar19;
  byte *pbVar20;
  size_t sVar21;
  undefined *puVar22;
  undefined8 *puVar23;
  char *pcVar24;
  FILE *__stream;
  undefined8 *puVar25;
  void *pvVar26;
  undefined8 *puVar27;
  undefined8 *puVar28;
  ulong uVar29;
  void *pvVar30;
  char *pcVar31;
  long *plVar32;
  ulong uVar33;
  undefined8 uVar34;
  undefined8 uVar35;
  undefined8 uVar36;
  undefined8 uVar37;
  ulong uVar38;
  byte bVar39;
  pthread_mutex_t *ppVar40;
  void *pvVar41;
  uint uVar42;
  undefined **ppuVar43;
  undefined1 uVar44;
  long *plVar45;
  uint uVar46;
  int *piVar47;
  long lVar48;
  int *piVar49;
  pthread_mutex_t *__mutex;
  undefined **ppuVar50;
  long in_FS_OFFSET;
  bool bVar51;
  bool bVar52;
  byte bVar53;
  double dVar54;
  double dVar55;
  ulong local_458;
  bool local_440;
  pthread_mutex_t *local_438;
  ulong local_430;
  pthread_mutex_t *local_428;
  void **local_420;
  pthread_mutex_t *local_418;
  undefined **local_408;
  pthread_mutex_t *local_3f8;
  long local_3e0;
  pthread_mutex_t *local_3d8;
  ulong local_3d0;
  undefined *local_3c8;
  rlimit local_398;
  long local_388;
  long local_380;
  undefined1 local_378 [57];
  undefined1 local_33f;
  undefined8 local_328;
  undefined1 local_320 [16];
  undefined1 local_310 [16];
  undefined1 local_300 [16];
  undefined1 local_2f0 [16];
  void *local_2d8;
  long local_2d0;
  ulong local_2c8;
  undefined **local_2c0;
  long local_2b8;
  char local_2a8;
  rlimit local_288 [18];
  _union_1457 local_168;
  pthread_mutex_t local_160;
  pthread_cond_t pStack_138;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined4 local_e0;
  char local_78;
  undefined1 local_77;
  long local_40;
  
  bVar53 = 0;
  local_458 = (ulong)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar14 = getenv("POSIXLY_CORRECT");
  iVar11 = FUN_0010eb70();
  local_440 = 0x2b8 < iVar11 - 0x30db0U;
  FUN_0010ec00(*param_2);
  pcVar15 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  DAT_0011d130 = 2;
  DAT_0011d861 = FUN_0010cc40(3);
  cVar9 = FUN_0010cc40();
  DAT_0011d860 = cVar9;
  plVar16 = localeconv();
  DAT_0011d868 = *plVar16->decimal_point;
  if ((DAT_0011d868 == '\0') || (plVar16->decimal_point[1] != '\0')) {
    DAT_0011d868 = '.';
  }
  cVar10 = *plVar16->thousands_sep;
  DAT_0011d864 = (int)cVar10;
  if (cVar10 == '\0') goto LAB_00104712;
  if (plVar16->thousands_sep[1] != '\0') goto LAB_0010470b;
LAB_00103fc7:
  DAT_0011d418 = '\0';
  ppuVar17 = __ctype_b_loc();
  pcVar18 = (char *)__ctype_toupper_loc();
  lVar19 = 0;
  do {
    uVar2 = (*ppuVar17)[lVar19];
    if ((uVar2 & 1) == 0) {
      if (lVar19 != 10) {
        uVar44 = 0;
        bVar39 = ((byte)(uVar2 >> 3) ^ 1) & 1;
        goto LAB_0010402e;
      }
      DAT_0011d74a = 1;
      DAT_0011d54a = 0;
      DAT_0011d64a = ((byte)((*ppuVar17)[10] >> 0xe) ^ 1) & 1;
      DAT_0011d44a = (undefined1)((__int32_t *)((rlimit *)pcVar18)->rlim_cur)[10];
    }
    else {
      uVar44 = 1;
      bVar39 = 0;
LAB_0010402e:
      (&DAT_0011d540)[lVar19] = bVar39;
      p_Var4 = (__int32_t *)((rlimit *)pcVar18)->rlim_cur;
      (&DAT_0011d740)[lVar19] = uVar44;
      _Var3 = p_Var4[lVar19];
      (&DAT_0011d640)[lVar19] = ((byte)(uVar2 >> 0xe) ^ 1) & 1;
      (&DAT_0011d440)[lVar19] = (char)_Var3;
      if (lVar19 == 0xff) goto LAB_001040b8;
    }
    lVar19 = lVar19 + 1;
  } while( true );
switchD_0010445f_caseD_87:
  pcVar18 = "";
  local_418 = (pthread_mutex_t *)(ulong)uVar42;
  local_428 = optarg;
  iVar12 = FUN_001132f0(optarg,0,10,local_288);
  iVar11 = DAT_0011d028;
  if (iVar12 == 1) {
    local_3d0 = 0xffffffffffffffff;
  }
  else {
    if (iVar12 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_00113230(iVar12,rVar7 & 0xffffffff,0xffffff87,&PTR_s_ignore_leading_blanks_0011c5e0,
                   ppVar40);
    }
    local_3d0 = CONCAT44(local_288[0].rlim_cur._4_4_,(int)local_288[0].rlim_cur);
    if (local_3d0 == 0) {
      uVar36 = dcgettext(0,"number in parallel must be nonzero",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar36);
    }
  }
  goto LAB_001043e0;
switchD_0010445f_caseD_2:
  FUN_0010bab0(2);
  if (DAT_0011d410 != 0) {
    FUN_00107140(local_3c8,&local_78);
    sVar21 = strlen(&local_78);
    uVar36 = dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",sVar21,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar36,&local_78);
  }
LAB_00104699:
  DAT_0011d41b = local_300[0xf];
  if ((char)param_2 == '\0') {
LAB_001046b0:
    if (DAT_0011d428 == 0) {
      pcVar18 = getenv("TMPDIR");
      if (pcVar18 == (char *)0x0) {
        pcVar18 = "/tmp";
      }
      FUN_00107730(pcVar18);
    }
    if (ppuVar43 == (undefined **)0x0) {
      ppuVar43 = (undefined **)0x1;
      free(puVar23);
      puVar23 = (undefined8 *)FUN_00112ab0(8);
      *puVar23 = &DAT_00117ace;
    }
    pvVar26 = DAT_0011d438;
    if ((DAT_0011d438 != (void *)0x0) &&
       (pvVar26 = (void *)((ulong)DAT_0011d024 * 0x22),
       (void *)((ulong)DAT_0011d024 * 0x22) < DAT_0011d438)) {
      pvVar26 = DAT_0011d438;
    }
    DAT_0011d438 = pvVar26;
    if (cVar9 == '\0') {
      ppuVar50 = (undefined **)&DAT_00117ace;
      local_408 = (undefined **)0x0;
      local_3c8 = &DAT_00117ace;
      puVar25 = puVar23;
      goto LAB_00105230;
    }
    if (ppuVar43 != (undefined **)0x1) {
LAB_00106df3:
      uVar36 = FUN_001108e0(4,puVar23[1]);
      uVar37 = dcgettext(0,"extra operand %s not allowed with -%c",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar37,uVar36,(int)cVar9);
    }
    if (local_3f8 == (pthread_mutex_t *)0x0) goto LAB_001056f4;
    DAT_0011d010 = cVar9;
    FUN_00107310();
    cVar9 = '\0';
LAB_0010470b:
    DAT_0011d862 = 1;
LAB_00104712:
    DAT_0011d864 = 0x80;
    goto LAB_00103fc7;
  }
  lVar19 = FUN_00111060(local_3d8,0x10);
  if (lVar19 != 0) {
    FUN_00111220(lVar19,&local_78,0x10);
    iVar11 = FUN_001113c0(lVar19);
    if (iVar11 == 0) {
      param_2 = (undefined **)&DAT_0011d300;
      MD5_Init((MD5_CTX *)&DAT_0011d300);
      MD5_Update((MD5_CTX *)&DAT_0011d300,&local_78,0x10);
      goto LAB_001046b0;
    }
    pcVar14 = "close failed";
    goto LAB_00106b58;
  }
  goto LAB_00106b3d;
LAB_001060e5:
  FUN_00107790(&local_2d8,local_3e0,pvVar41);
LAB_00105385:
  local_420 = &local_2d8;
  local_408 = (undefined **)((long)local_408 + -1);
  local_2a8 = '\0';
  while (cVar10 = FUN_00107ef0(local_420,__stream,uVar36), cVar9 = local_2a8, ppuVar43 = local_2c0,
        uVar38 = local_2c8, pvVar26 = local_2d8, cVar10 != '\0') {
    if (local_2a8 == '\0') {
LAB_00105402:
      DAT_0011d840 = 0;
      local_458 = local_458 + 1;
      cVar9 = '\0';
      lVar19 = FUN_00108c20(&local_398,0);
      ppVar40 = (pthread_mutex_t *)(lVar19 + 0xd);
    }
    else {
      if (local_408 != (undefined **)0x0) {
        if ((undefined *)((long)local_2c0 + (-(local_3e0 * local_2c8) - local_2d0)) <=
            (undefined *)(local_3e0 + 1)) goto LAB_00105402;
        local_2b8 = local_2d0;
        FUN_00107660(__stream,uVar36);
        goto LAB_00105d45;
      }
      DAT_0011d840 = 0;
      if (local_458 != 0 || local_2b8 != 0) goto LAB_00105402;
      FUN_00107660(__stream,uVar36);
      uVar29 = FUN_001074e0(local_3f8,&DAT_00116be9);
      if (uVar29 == 0) goto LAB_001068f4;
      local_458 = 0;
      ppVar40 = local_3f8;
      uVar38 = local_2c8;
      local_398.rlim_cur = uVar29;
    }
    ppuVar50 = (undefined **)((long)ppuVar43 + (long)pvVar26);
    if (uVar38 < 2) {
      FUN_0010a3c0(ppuVar50 + -4,local_398.rlim_cur,ppVar40);
    }
    else {
      local_168.sa_handler = (__sighandler_t)FUN_0010deb0(FUN_00107c20);
      pthread_mutex_init(&local_160,(pthread_mutexattr_t *)0x0);
      pthread_cond_init(&pStack_138,(pthread_condattr_t *)0x0);
      puVar25 = (undefined8 *)FUN_00112ab0(uVar33 << 8);
      puVar25[3] = 0;
      local_418 = (pthread_mutex_t *)(puVar25 + 0xb);
      puVar25[2] = 0;
      puVar25[1] = 0;
      *puVar25 = 0;
      puVar25[4] = 0;
      puVar25[6] = uVar38;
      puVar25[5] = uVar38;
      puVar25[7] = 0;
      *(undefined4 *)(puVar25 + 10) = 0;
      *(undefined1 *)((long)puVar25 + 0x54) = 0;
      pthread_mutex_init(local_418,(pthread_mutexattr_t *)0x0);
      local_428 = (pthread_mutex_t *)(puVar25 + 0x10);
      FUN_00107af0(puVar25,local_428,ppuVar50,uVar33,uVar38,0);
      FUN_0010b220(ppuVar50,uVar33,uVar38,local_428,&local_168,local_398.rlim_cur,ppVar40);
      __mutex = local_418;
      ppuVar43 = (undefined **)(uVar33 * 2 + -1);
      if (uVar33 * 2 != 0) {
        do {
          ppuVar50 = (undefined **)((long)ppuVar43 + -1);
          pthread_mutex_destroy(__mutex);
          __mutex = (pthread_mutex_t *)((long)__mutex + 0x80);
          ppuVar43 = ppuVar50;
        } while (ppuVar50 != (undefined **)0xffffffffffffffff);
      }
      free(puVar25);
      FUN_0010df20(local_168.sa_handler);
      pthread_cond_destroy(&pStack_138);
      pthread_mutex_destroy(&local_160);
    }
    FUN_00107660(local_398.rlim_cur,ppVar40);
    if (cVar9 != '\0') {
      free(local_2d8);
      goto joined_r0x00105599;
    }
  }
  FUN_00107660(__stream,uVar36);
  if (local_408 == (undefined **)0x0) goto LAB_001064ce;
LAB_00105d45:
  puVar23 = puVar23 + 1;
  goto LAB_00105328;
LAB_001064ce:
  free(local_2d8);
  puVar23 = DAT_0011d368;
  plVar32 = (long *)FUN_00112bf0(local_458,0x10);
  plVar45 = plVar32;
  while (puVar23 != (undefined8 *)0x0) {
    plVar45[1] = (long)puVar23;
    lVar19 = (long)puVar23 + 0xd;
    puVar23 = (undefined8 *)*puVar23;
    *plVar45 = lVar19;
    plVar45 = plVar45 + 2;
  }
  FUN_0010ac90(plVar32,local_458,local_458,local_3f8);
  free(plVar32);
joined_r0x00105599:
  while (0 < DAT_0011d35c) {
    FUN_001078c0(0xffffffff);
  }
  goto LAB_001055b4;
LAB_001056f4:
  uVar36 = *puVar23;
  lVar48 = FUN_001074e0(uVar36,"r");
  lVar19 = DAT_0011d410;
  if (lVar48 == 0) {
    FUN_00107c60(uVar36);
LAB_00106a5b:
    uVar36 = FUN_00110fd0();
    uVar37 = dcgettext(0,"multi-character tab %s",5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar37,uVar36);
  }
  uVar42 = (uint)DAT_0011d419;
  pvVar26 = DAT_0011d030;
  if (DAT_0011d030 <= DAT_0011d438) {
    pvVar26 = DAT_0011d438;
  }
  uVar33 = 0;
  FUN_00107790(&local_2d8,0x20,pvVar26);
  local_398.rlim_cur = 0;
  local_438 = (pthread_mutex_t *)0x0;
  do {
    cVar10 = FUN_00107ef0(&local_2d8,lVar48,uVar36);
    uVar38 = local_2c8;
    if (cVar10 == '\0') {
      uVar42 = 1;
LAB_00105e04:
      FUN_00107660(lVar48,uVar36);
      free(local_2d8);
      free((void *)local_398.rlim_cur);
                    /* WARNING: Subroutine does not return */
      exit(uVar42 ^ 1);
    }
    plVar45 = (long *)((long)local_2c0 + (long)local_2d8);
    lVar6 = local_2c8 * -4;
    if ((uVar33 != 0) &&
       (iVar11 = FUN_0010a0b0(&local_398,plVar45 + -4), plVar32 = plVar45,
       (int)(uVar42 ^ 1) <= iVar11)) {
LAB_00105df7:
      uVar42 = 0;
      if (cVar9 == 'c') {
        uVar34 = FUN_0010e200(((long)plVar45 - (long)(plVar32 + -4) >> 5) + (long)local_438,
                              &local_78);
        uVar37 = DAT_0011d888;
        uVar35 = dcgettext(0,"%s: %s:%s: disorder: ",5);
        __fprintf_chk(stderr,1,uVar35,uVar37,uVar36,uVar34);
        uVar37 = dcgettext(0,"standard error",5);
        FUN_001082d0(plVar32 + -4,stderr,uVar37);
      }
      goto LAB_00105e04;
    }
    plVar8 = plVar45 + -4;
    while (plVar32 = plVar8, plVar45 + lVar6 < plVar32) {
      iVar11 = FUN_0010a0b0(plVar32,plVar32 + -4);
      plVar8 = plVar32 + -4;
      if ((int)(uVar42 ^ 1) <= iVar11) goto LAB_00105df7;
    }
    uVar29 = plVar32[1];
    local_438 = (pthread_mutex_t *)((long)local_438 + uVar38);
    uVar38 = uVar33;
    if (uVar33 < uVar29) {
      do {
        uVar38 = uVar38 * 2;
        uVar33 = uVar29;
        if (uVar38 == 0) break;
        uVar33 = uVar38;
      } while (uVar38 < uVar29);
      free((void *)local_398.rlim_cur);
      local_398.rlim_cur = FUN_00112ab0(uVar33);
      uVar29 = plVar32[1];
    }
    pvVar26 = memcpy((void *)local_398.rlim_cur,(void *)*plVar32,uVar29);
    local_398.rlim_max = plVar32[1];
    if (lVar19 != 0) {
      local_388 = (long)pvVar26 + (plVar32[2] - *plVar32);
      local_380 = (long)pvVar26 + (plVar32[3] - *plVar32);
    }
  } while( true );
LAB_001069d1:
  uVar36 = puVar23[(long)ppuVar50];
  pcVar14 = "stat failed";
LAB_001069e2:
  uVar37 = dcgettext(0,pcVar14,5);
                    /* WARNING: Subroutine does not return */
  FUN_00107480(uVar37,uVar36);
  while( true ) {
    local_408 = (undefined **)((long)local_408 + 1);
    puVar25 = puVar25 + 1;
    if (ppuVar43 == local_408) break;
LAB_00105230:
    pcVar14 = (char *)*puVar25;
    iVar11 = strcmp(pcVar14,"-");
    if ((iVar11 != 0) && (iVar11 = euidaccess(pcVar14,4), iVar11 != 0)) {
      uVar36 = *puVar25;
      pcVar14 = "cannot read";
      goto LAB_001069e2;
    }
  }
  if (local_3f8 != (pthread_mutex_t *)0x0) {
    iVar11 = open(local_3f8->__size,0x80041,0x1b6);
    if (iVar11 < 0) {
LAB_001068f4:
      uVar36 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_00107480(uVar36,local_3f8);
    }
    if (iVar11 != 1) {
      FUN_00107c90(iVar11,1);
    }
  }
  if (!bVar52) {
    if (local_3d0 == 0) {
      uVar33 = FUN_0010e570(2);
      local_3d0 = 8;
      if (uVar33 < 9) {
        local_3d0 = uVar33;
      }
    }
    local_458 = 0;
    local_2c0 = (undefined **)0x0;
    uVar33 = 0xffffffffffffff;
    if (local_3d0 < 0x100000000000000) {
      uVar33 = local_3d0;
    }
LAB_00105328:
    uVar36 = *puVar23;
    __stream = (FILE *)FUN_001074e0(uVar36,"r");
    if (__stream == (FILE *)0x0) {
      FUN_00107c60(uVar36);
      iVar12 = (int)local_418;
LAB_00106a94:
      local_418._0_4_ = iVar12;
      uVar36 = FUN_00110fd0(local_428);
      puVar22 = ppuVar50[(long)(int)local_418 * 4];
      uVar37 = dcgettext(0,"invalid --%s argument %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar37,puVar22,uVar36);
    }
    local_3e0 = 0x30;
    if (1 < local_3d0) {
      local_3e0 = 1;
      uVar38 = 1;
      do {
        uVar38 = uVar38 * 2;
        local_3e0 = local_3e0 + 1;
      } while (uVar38 < uVar33);
      local_3e0 = local_3e0 * 0x20;
    }
    ppuVar50 = local_2c0;
    if (local_2c0 == (undefined **)0x0) {
      pvVar26 = (void *)(local_3e0 + 2);
      do {
        if (ppuVar50 == (undefined **)0x0) {
          iVar11 = fileno(__stream);
          iVar11 = fstat(iVar11,(stat *)&local_168);
          bVar52 = iVar11 != 0;
        }
        else {
          pcVar14 = (char *)puVar23[(long)ppuVar50];
          iVar11 = strcmp(pcVar14,"-");
          if (iVar11 == 0) {
            iVar11 = fstat(0,(stat *)&local_168);
            bVar52 = iVar11 != 0;
          }
          else {
            iVar11 = stat(pcVar14,(stat *)&local_168);
            bVar52 = iVar11 != 0;
          }
        }
        if (bVar52) goto LAB_001069d1;
        if ((local_160.__data.__kind & 0xf000U) == 0x8000) {
          local_430 = pStack_138.__align;
          if (DAT_0011d240 == (void *)0x0) {
            DAT_0011d240 = DAT_0011d438;
            if (DAT_0011d438 == (void *)0x0) goto LAB_00105f41;
          }
        }
        else {
          pvVar41 = DAT_0011d438;
          if (DAT_0011d438 != (void *)0x0) goto LAB_001060e5;
          local_430 = 0x20000;
          if (DAT_0011d240 == (void *)0x0) {
            local_430 = 0x20000;
LAB_00105f41:
            iVar11 = getrlimit(RLIMIT_DATA,&local_398);
            uVar38 = 0xffffffffffffffff;
            if (iVar11 == 0) {
              uVar38 = local_398.rlim_cur;
            }
            iVar11 = getrlimit(RLIMIT_AS,&local_398);
            if ((iVar11 == 0) && (local_398.rlim_cur < uVar38)) {
              uVar38 = local_398.rlim_cur;
            }
            pvVar41 = (void *)(uVar38 >> 1);
            iVar11 = getrlimit(__RLIMIT_RSS,&local_398);
            if ((iVar11 == 0) &&
               (pvVar30 = (void *)((local_398.rlim_cur >> 4) * 0xf), pvVar30 < pvVar41)) {
              pvVar41 = pvVar30;
            }
            local_428 = (pthread_mutex_t *)FUN_0010e850();
            dVar54 = (double)FUN_0010e760();
            dVar55 = dVar54 * _DAT_00117110;
            ppVar40 = local_428;
            if ((double)local_428 <= DAT_00117108 * dVar54) {
              ppVar40 = (pthread_mutex_t *)(DAT_00117108 * dVar54);
            }
            if (dVar55 < (double)(long)pvVar41) {
              if (DAT_00117100 <= dVar55) {
                pvVar41 = (void *)((long)(dVar55 - DAT_00117100) ^ 0x8000000000000000);
              }
              else {
                pvVar41 = (void *)(long)dVar55;
              }
            }
            if ((double)ppVar40 < (double)pvVar41) {
              if (DAT_00117100 <= (double)ppVar40) {
                pvVar41 = (void *)((long)((double)ppVar40 - DAT_00117100) ^ 0x8000000000000000);
              }
              else {
                pvVar41 = (void *)(long)(double)ppVar40;
              }
            }
            DAT_0011d240 = (void *)((ulong)DAT_0011d024 * 0x22);
            if ((void *)((ulong)DAT_0011d024 * 0x22) < pvVar41) {
              DAT_0011d240 = pvVar41;
            }
          }
        }
        uVar38 = local_430 * (local_3e0 + 1U) + 1;
        pvVar41 = DAT_0011d240;
        if ((local_430 != uVar38 / (local_3e0 + 1U)) ||
           ((ulong)((long)DAT_0011d240 - (long)pvVar26) <= uVar38)) goto LAB_001060e5;
        pvVar26 = (void *)((long)pvVar26 + uVar38);
        ppuVar50 = (undefined **)((long)ppuVar50 + 1);
        pvVar41 = pvVar26;
        if (local_408 <= ppuVar50) goto LAB_001060e5;
      } while( true );
    }
    goto LAB_00105385;
  }
  puVar27 = (undefined8 *)FUN_00112e90(local_408,0x10);
  puVar25 = puVar23 + (long)local_408;
  puVar28 = puVar27;
  do {
    uVar36 = *puVar23;
    puVar23 = puVar23 + 1;
    *puVar28 = uVar36;
    puVar28 = puVar28 + 2;
  } while (puVar25 != puVar23);
  FUN_0010ac90(puVar27,0,local_408,local_3f8);
LAB_001055b4:
  if ((DAT_0011d418 != '\0') && (iVar11 = FUN_0010c5b0(stdin), iVar11 == -1)) {
    uVar36 = dcgettext(0,"close failed",5);
                    /* WARNING: Subroutine does not return */
    FUN_00107480(uVar36,local_3c8);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return 0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
LAB_001040b8:
  if (cVar9 != '\0') {
    lVar19 = 1;
    ppuVar43 = &PTR_DAT_0011d060;
    do {
      pbVar20 = (byte *)nl_langinfo((int)lVar19 + 0x2000d);
      sVar21 = strlen((char *)pbVar20);
      puVar22 = (undefined *)FUN_00112ab0(sVar21 + 1);
      *(int *)(ppuVar43 + 1) = (int)lVar19;
      *ppuVar43 = puVar22;
      lVar48 = 0;
      if (sVar21 != 0) {
        puVar5 = *ppuVar17;
        pbVar1 = pbVar20 + sVar21;
        lVar48 = 0;
        do {
          if ((puVar5[*pbVar20] & 1) == 0) {
            puVar22[lVar48] = (&DAT_0011d440)[*pbVar20];
            lVar48 = lVar48 + 1;
          }
          pbVar20 = pbVar20 + 1;
        } while (pbVar1 != pbVar20);
      }
      lVar19 = lVar19 + 1;
      puVar22[lVar48] = 0;
      ppuVar43 = ppuVar43 + 2;
    } while (lVar19 != 0xd);
    qsort(&PTR_DAT_0011d060,0xc,0x10,FUN_00107300);
  }
  piVar49 = &DAT_00115324;
  piVar47 = &DAT_00115324;
  sigemptyset((sigset_t *)&DAT_0011d380);
  iVar11 = 0xe;
  while( true ) {
    sigaction(iVar11,(sigaction *)0x0,(sigaction *)&local_168);
    if (local_168.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_0011d380,iVar11);
    }
    if (piVar47 == (int *)&DAT_0011534c) break;
    iVar11 = *piVar47;
    piVar47 = piVar47 + 1;
  }
  local_e0 = 0;
  local_168.sa_handler = FUN_00108270;
  iVar11 = 0xe;
  local_160.__align = _DAT_0011d380;
  local_160._8_8_ = uRam000000000011d388;
  local_160._16_8_ = _DAT_0011d390;
  local_160.__data.__list.__prev = p_Ram000000000011d398;
  local_160.__data.__list.__next = _DAT_0011d3a0;
  pStack_138.__align = uRam000000000011d3a8;
  pStack_138.__data.__total_seq = _DAT_0011d3b0;
  pStack_138.__data.__wakeup_seq = uRam000000000011d3b8;
  pStack_138.__data.__woken_seq = _DAT_0011d3c0;
  pStack_138.__data.__mutex = pvRam000000000011d3c8;
  pStack_138._40_8_ = _DAT_0011d3d0;
  uStack_108 = uRam000000000011d3d8;
  local_100 = _DAT_0011d3e0;
  uStack_f8 = uRam000000000011d3e8;
  local_f0 = _DAT_0011d3f0;
  uStack_e8 = uRam000000000011d3f8;
  while( true ) {
    iVar12 = sigismember((sigset_t *)&DAT_0011d380,iVar11);
    if (iVar12 != 0) {
      sigaction(iVar11,(sigaction *)&local_168,(sigaction *)0x0);
    }
    if (piVar49 == (int *)&DAT_0011534c) break;
    iVar11 = *piVar49;
    piVar49 = piVar49 + 1;
  }
  ppuVar43 = (undefined **)0x0;
  signal(0x11,(__sighandler_t)0x0);
  ppuVar50 = &PTR_s_ignore_leading_blanks_0011c5e0;
  FUN_001142c0(FUN_00108ed0);
  local_320 = ZEXT816(0xffffffffffffffff) << 0x40;
  local_328 = 0xffffffffffffffff;
  local_310 = (undefined1  [16])0x0;
  local_300 = (undefined1  [16])0x0;
  local_2f0 = (undefined1  [16])0x0;
  puVar23 = (undefined8 *)FUN_00112bf0((long)(int)param_1,8);
  bVar52 = false;
  uVar42 = 0;
  local_3f8 = (pthread_mutex_t *)0x0;
  local_438 = (pthread_mutex_t *)0x0;
  local_3d0 = 0;
  local_3d8 = (pthread_mutex_t *)0x0;
  cVar9 = '\0';
  iVar11 = DAT_0011d028;
LAB_001043e0:
  while( true ) {
    DAT_0011d028 = iVar11;
    local_398.rlim_cur = CONCAT44(local_398.rlim_cur._4_4_,0xffffffff);
    if (uVar42 != 0xffffffff) break;
LAB_001043f1:
    lVar19 = (long)(int)optind;
LAB_001043c0:
    if ((int)param_1 <= (int)lVar19) {
      if (local_438 != (pthread_mutex_t *)0x0) {
        if (ppuVar43 != (undefined **)0x0) {
LAB_001065b8:
          uVar36 = FUN_001108e0(4,*puVar23);
          uVar37 = dcgettext(0,"extra operand %s",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar37,uVar36);
        }
        lVar19 = FUN_001074e0(local_438,"r");
        if (lVar19 == 0) {
          FUN_00107c60(local_438);
LAB_00106d48:
          FUN_001128e0(stdout,"sort","GNU coreutils",PTR_DAT_0011d120,"Mike Haertel","Paul Eggert",0
                       ,pcVar18);
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        FUN_001119f0(local_288);
        cVar10 = FUN_00111aa0(lVar19,local_288);
        if (cVar10 == '\0') {
          uVar36 = FUN_001108e0(4,local_438);
          uVar37 = dcgettext(0,"cannot read file names from %s",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar37,uVar36);
        }
        FUN_00107660(lVar19);
        if (CONCAT44(local_288[0].rlim_cur._4_4_,(int)local_288[0].rlim_cur) == 0) {
          uVar36 = FUN_001108e0(4,local_438);
          uVar37 = dcgettext(0,"no input from %s",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar37,uVar36);
        }
        free(puVar23);
        puVar23 = (undefined8 *)local_288[0].rlim_max;
        param_2 = (undefined **)CONCAT44(local_288[0].rlim_cur._4_4_,(int)local_288[0].rlim_cur);
        if (param_2 != (undefined **)0x0) {
          ppuVar43 = (undefined **)0x0;
          do {
            pcVar18 = *(char **)((rlim_t)puVar23 + (long)ppuVar43);
            iVar11 = strcmp(pcVar18,"-");
            if (iVar11 == 0) {
              uVar36 = FUN_001108e0(4,pcVar18);
              uVar37 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",
                                 5);
                    /* WARNING: Subroutine does not return */
              error(2,0,uVar37,uVar36);
            }
            ppuVar43 = (undefined **)((long)ppuVar43 + 1);
            if (*pcVar18 == '\0') {
              uVar36 = FUN_00110c60(0,3,local_438);
              uVar37 = dcgettext(0,"%s:%lu: invalid zero-length file name",5);
                    /* WARNING: Subroutine does not return */
              error(2,0,uVar37,uVar36,ppuVar43);
            }
          } while (param_2 != ppuVar43);
        }
      }
      goto LAB_00104571;
    }
    optind = (int)lVar19 + 1;
    puVar23[(long)ppuVar43] = param_2[lVar19];
    ppuVar43 = (undefined **)((long)ppuVar43 + 1);
    iVar11 = DAT_0011d028;
  }
  if ((ppuVar43 != (undefined **)0x0) && (pcVar14 != (char *)0x0)) {
    lVar19 = (long)(int)optind;
    if ((local_440 == true) && (cVar9 == '\0')) {
      if (param_1 == optind) goto LAB_00104560;
      pcVar24 = param_2[(int)optind];
      if (((*pcVar24 == '-') && (pcVar24[1] == 'o')) &&
         ((pcVar24[2] != '\0' || (optind + 1 != param_1)))) goto LAB_00104410;
    }
    goto LAB_001043c0;
  }
LAB_00104410:
  pcVar18 = (char *)&local_398;
  uVar42 = getopt_long(param_1,param_2,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",
                       &PTR_s_ignore_leading_blanks_0011c5e0);
  rVar7 = local_398.rlim_cur;
  ppVar40 = optarg;
  if (uVar42 == 0xffffffff) goto LAB_001043f1;
  if (0x87 < (int)uVar42) goto switchD_0010445f_caseD_2;
  if (0 < (int)uVar42) {
    if (0x87 < uVar42) goto switchD_0010445f_caseD_2;
    iVar12 = (int)local_398.rlim_cur;
    iVar11 = DAT_0011d028;
    switch(uVar42) {
    case 1:
      if (optarg->__size[0] == '+') {
        if (optind == param_1) {
          local_418 = (pthread_mutex_t *)0x0;
        }
        else {
          local_418 = (pthread_mutex_t *)0x0;
          if (*param_2[(int)optind] == '-') {
            local_418 = (pthread_mutex_t *)(ulong)((int)(char)param_2[(int)optind][1] - 0x30U < 10);
          }
        }
        local_440 = (bool)(local_440 | pcVar14 == (char *)0x0 & (byte)local_418);
        if (local_440 != false) {
          local_428 = (pthread_mutex_t *)local_378;
          ppVar40 = local_428;
          for (lVar19 = 0x12; lVar19 != 0; lVar19 = lVar19 + -1) {
            (ppVar40->__data).__lock = 0;
            ppVar40 = (pthread_mutex_t *)((long)ppVar40 + (ulong)bVar53 * -8 + 4);
          }
          local_378._16_8_ = -1;
          pcVar24 = (char *)FUN_00107390((undefined1 *)((long)&(optarg->__data).__lock + 1),
                                         local_428,0);
          if (pcVar24 == (char *)0x0) {
            if (local_378._8_8_ == 0 && local_378._0_8_ == 0) {
              local_378._0_8_ = -1;
            }
          }
          else {
            if (*pcVar24 == '.') {
              pcVar24 = (char *)FUN_00107390(pcVar24 + 1,
                                             &(((pthread_mutex_t *)local_378)->__data).__owner,0);
              if (local_378._0_8_ == 0 && local_378._8_8_ == 0) goto LAB_001061b5;
LAB_001061c1:
              if (pcVar24 == (char *)0x0) goto LAB_001051bb;
            }
            else if (local_378._8_8_ == 0 && local_378._0_8_ == 0) {
LAB_001061b5:
              local_378._0_8_ = -1;
              goto LAB_001061c1;
            }
            pcVar24 = (char *)FUN_001071f0(pcVar24,local_428,0);
            if (*pcVar24 == '\0') {
              if ((int)local_418 != 0) {
                lVar19 = (long)(int)optind;
                optind = optind + 1;
                pcVar24 = param_2[lVar19];
                pcVar31 = (char *)FUN_00107390(pcVar24 + 1,
                                               &(((pthread_mutex_t *)local_378)->__data).__kind,
                                               "invalid number after \'-\'");
                if (pcVar31 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                  __assert_fail("s","src/sort.c",0x1161,"main");
                }
                if (*pcVar31 == '.') goto LAB_00106a05;
                while( true ) {
                  if (((__pthread_internal_list *)local_378._24_8_ == (__pthread_internal_list *)0x0
                      ) && (local_378._16_8_ != 0)) {
                    local_378._16_8_ = local_378._16_8_ + -1;
                  }
                  pcVar31 = (char *)FUN_001071f0(pcVar31,local_428,1);
                  if (*pcVar31 == '\0') break;
                  pcVar31 = pcVar24;
                  FUN_00107340(pcVar24,"stray character in field spec");
LAB_00106a05:
                  pcVar31 = (char *)FUN_00107390(pcVar31 + 1,
                                                 &(((pthread_mutex_t *)local_378)->__data).__list,
                                                 "invalid number after \'.\'");
                }
              }
              local_418 = (pthread_mutex_t *)(ulong)uVar42;
              local_33f = 1;
              FUN_001076e0(local_428);
              iVar11 = DAT_0011d028;
              break;
            }
          }
        }
      }
LAB_001051bb:
      puVar23[(long)ppuVar43] = optarg;
      ppuVar43 = (undefined **)((long)ppuVar43 + 1);
      iVar11 = DAT_0011d028;
      break;
    default:
      goto switchD_0010445f_caseD_2;
    case 0x4d:
    case 0x52:
    case 0x56:
    case 0x62:
    case 100:
    case 0x66:
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6e:
    case 0x72:
      goto switchD_0010445f_caseD_4d;
    case 0x53:
      pcVar18 = "EgGkKmMPtTYZ";
      local_418 = (pthread_mutex_t *)(ulong)uVar42;
      local_428 = optarg;
      iVar11 = FUN_001132f0(optarg,local_288,10,&local_2d8);
      if (iVar11 == 0) {
        if ((int)*(char *)(CONCAT44(local_288[0].rlim_cur._4_4_,(int)local_288[0].rlim_cur) + -1) -
            0x30U < 10) {
          if ((void *)0x3fffffffffffff < local_2d8) {
LAB_00106691:
            iVar11 = 1;
LAB_00105195:
                    /* WARNING: Subroutine does not return */
            FUN_00113230(iVar11,rVar7 & 0xffffffff,0x53,&PTR_s_ignore_leading_blanks_0011c5e0,
                         ppVar40);
          }
          local_2d8 = (void *)((long)local_2d8 << 10);
        }
      }
      else {
        if (((iVar11 != 2) ||
            (pcVar24 = (char *)CONCAT44(local_288[0].rlim_cur._4_4_,(int)local_288[0].rlim_cur),
            9 < (int)pcVar24[-1] - 0x30U)) || (pcVar24[1] != '\0')) goto LAB_00105195;
        if (*pcVar24 == '%') {
          local_418 = ppVar40;
          local_428 = (pthread_mutex_t *)(ulong)uVar42;
          dVar54 = (double)FUN_0010e760();
          dVar54 = ((double)local_2d8 * dVar54) / _DAT_001170f0;
          if (DAT_001170f8 <= dVar54) goto LAB_00106691;
          if (dVar54 < DAT_00117100) {
            local_2d8 = (void *)(long)dVar54;
          }
          else {
            local_2d8 = (void *)((long)(dVar54 - DAT_00117100) ^ 0x8000000000000000);
          }
        }
        else if (*pcVar24 != 'b') goto LAB_00105195;
      }
      iVar11 = DAT_0011d028;
      if ((DAT_0011d438 <= local_2d8) &&
         (DAT_0011d438 = (void *)((ulong)DAT_0011d024 * 0x22), DAT_0011d438 < local_2d8)) {
        DAT_0011d438 = local_2d8;
      }
      break;
    case 0x54:
      local_428 = (pthread_mutex_t *)(ulong)uVar42;
      FUN_00107730(optarg);
      iVar11 = DAT_0011d028;
      break;
    case 0x6b:
      local_428 = (pthread_mutex_t *)local_378;
      ppVar40 = local_428;
      for (lVar19 = 0x12; lVar19 != 0; lVar19 = lVar19 + -1) {
        (ppVar40->__data).__lock = 0;
        ppVar40 = (pthread_mutex_t *)((long)ppVar40 + (ulong)bVar53 * -8 + 4);
      }
      local_378._16_8_ = -1;
      pcVar24 = (char *)FUN_00107390(optarg,local_428,"invalid number at field start");
      lVar19 = local_378._0_8_ + -1;
      bVar51 = local_378._0_8_ == 0;
      local_378._0_8_ = lVar19;
      if (bVar51) {
LAB_00106de0:
        FUN_00107340(optarg,"field number is zero");
        goto LAB_00106df3;
      }
      if (*pcVar24 == '.') {
        pcVar24 = (char *)FUN_00107390(pcVar24 + 1,&(((pthread_mutex_t *)local_378)->__data).__owner
                                       ,"invalid number after \'.\'");
        lVar19 = local_378._8_8_ + -1;
        bVar51 = local_378._8_8_ == 0;
        local_378._8_8_ = lVar19;
        if (bVar51) {
          FUN_00107340(optarg,"character offset is zero");
          goto LAB_001064ce;
        }
      }
      if (local_378._0_8_ == 0 && local_378._8_8_ == 0) {
        local_378._0_8_ = -1;
      }
      pcVar24 = (char *)FUN_001071f0(pcVar24,local_428,0);
      if (*pcVar24 == ',') {
        pcVar24 = (char *)FUN_00107390(pcVar24 + 1,&(((pthread_mutex_t *)local_378)->__data).__kind,
                                       "invalid number after \',\'");
        lVar19 = local_378._16_8_ + -1;
        bVar51 = local_378._16_8_ == 0;
        local_378._16_8_ = lVar19;
        if (bVar51) goto LAB_00106de0;
        if (*pcVar24 == '.') {
          pcVar24 = (char *)FUN_00107390(pcVar24 + 1,
                                         &(((pthread_mutex_t *)local_378)->__data).__list,
                                         "invalid number after \'.\'");
        }
        pcVar24 = (char *)FUN_001071f0(pcVar24,local_428,1);
      }
      else {
        local_378._16_8_ = -1;
        local_378._24_8_ = (__pthread_internal_list *)0x0;
      }
      if (*pcVar24 != '\0') {
        FUN_00107340(optarg,"stray character in field spec");
        goto LAB_001069d1;
      }
      local_418 = (pthread_mutex_t *)(ulong)uVar42;
      FUN_001076e0(local_428);
      iVar11 = DAT_0011d028;
      break;
    case 0x6d:
      bVar52 = true;
      break;
    case 0x6f:
      if (local_3f8 != (pthread_mutex_t *)0x0) {
        local_418 = (pthread_mutex_t *)(ulong)uVar42;
        local_428 = optarg;
        iVar11 = strcmp(local_3f8->__size,optarg->__size);
        if (iVar11 != 0) {
          uVar36 = dcgettext(0,"multiple output files specified",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar36);
        }
      }
      local_3f8 = ppVar40;
      iVar11 = DAT_0011d028;
      break;
    case 0x73:
      DAT_0011d41a = 1;
      break;
    case 0x74:
      iVar11 = (int)optarg->__size[0];
      if (optarg->__size[0] == '\0') {
        uVar36 = dcgettext(0,"empty tab",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar36);
      }
      if (optarg->__size[1] != '\0') {
        local_418 = (pthread_mutex_t *)(ulong)uVar42;
        local_428 = optarg;
        iVar11 = strcmp(optarg->__size,"\\0");
        if (iVar11 != 0) goto LAB_00106a5b;
        iVar11 = 0;
      }
      if ((DAT_0011d028 != 0x80) && (DAT_0011d028 != iVar11)) {
        uVar36 = dcgettext(0,"incompatible tabs",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar36);
      }
      break;
    case 0x75:
      DAT_0011d419 = 1;
      break;
    case 0x79:
      if ((pthread_mutex_t *)param_2[(long)(int)optind + -1] == optarg) {
        for (; (int)ppVar40->__size[0] - 0x30U < 10;
            ppVar40 = (pthread_mutex_t *)((long)&(ppVar40->__data).__lock + 1)) {
        }
        optind = (optind - 1) + (uint)(ppVar40->__size[0] == '\0');
      }
      break;
    case 0x7a:
      DAT_0011d038 = 0;
      break;
    case 0x80:
      uVar42 = 99;
      if (optarg != (pthread_mutex_t *)0x0) {
        pcVar18 = (char *)0x1;
        lVar19 = FUN_0010c370("--check",optarg,&PTR_s_quiet_0011c5c0,&DAT_00115466,1,
                              PTR_FUN_0011d128,1,99);
        uVar42 = (uint)(char)(&DAT_00115466)[lVar19];
      }
    case 0x43:
    case 99:
      if ((cVar9 != '\0') && ((int)cVar9 != uVar42)) {
        FUN_00107310(&DAT_00116d69);
        goto LAB_00106b1a;
      }
      cVar9 = (char)uVar42;
      iVar11 = DAT_0011d028;
      break;
    case 0x81:
      if (DAT_0011d408 != (pthread_mutex_t *)0x0) {
        local_418 = (pthread_mutex_t *)(ulong)uVar42;
        local_428 = optarg;
        iVar11 = strcmp(DAT_0011d408->__size,optarg->__size);
        if (iVar11 != 0) {
          uVar36 = dcgettext(0,"multiple compress programs specified",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar36);
        }
      }
      DAT_0011d408 = ppVar40;
      iVar11 = DAT_0011d028;
      break;
    case 0x82:
      DAT_0011d400 = '\x01';
      break;
    case 0x83:
      local_438 = optarg;
      break;
    case 0x84:
      pcVar18 = "";
      local_428 = optarg;
      local_418 = (pthread_mutex_t *)(local_398.rlim_cur & 0xffffffff);
      iVar11 = FUN_001132f0(optarg,0,10,&local_2d8);
      iVar13 = getrlimit(RLIMIT_NOFILE,local_288);
      uVar46 = 0x11;
      if (iVar13 == 0) {
        uVar46 = (int)local_288[0].rlim_cur - 3;
      }
      if (iVar11 == 0) {
        DAT_0011d024 = (uint)local_2d8;
        if ((void *)((ulong)local_2d8 & 0xffffffff) == local_2d8) {
          if (DAT_0011d024 < 2) goto LAB_00106a94;
          iVar11 = DAT_0011d028;
          if (DAT_0011d024 <= uVar46) break;
        }
      }
      else if (iVar11 != 1) {
                    /* WARNING: Subroutine does not return */
        FUN_00113230(iVar11,rVar7 & 0xffffffff,0xffffff84,&PTR_s_ignore_leading_blanks_0011c5e0,
                     ppVar40);
      }
      uVar36 = FUN_00110fd0(ppVar40);
      puVar22 = (&PTR_s_ignore_leading_blanks_0011c5e0)[(long)iVar12 * 4];
      uVar37 = dcgettext(0,"--%s argument %s too large",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar37,puVar22,uVar36);
    case 0x85:
      if (local_3d8 != (pthread_mutex_t *)0x0) {
        local_418 = (pthread_mutex_t *)(ulong)uVar42;
        local_428 = optarg;
        iVar11 = strcmp(local_3d8->__size,optarg->__size);
        if (iVar11 != 0) {
          uVar36 = dcgettext(0,"multiple random sources specified",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar36);
        }
      }
      local_3d8 = ppVar40;
      iVar11 = DAT_0011d028;
      break;
    case 0x86:
      pcVar18 = (char *)0x1;
      lVar19 = FUN_0010c370("--sort",optarg,&PTR_s_general_numeric_0011c580,&DAT_00115460,1,
                            PTR_FUN_0011d128,1,
                            &DAT_001150cc + *(int *)(&DAT_001150cc + (ulong)uVar42 * 4));
      uVar42 = (uint)(char)(&DAT_00115460)[lVar19];
switchD_0010445f_caseD_4d:
      local_78 = (char)uVar42;
      local_428 = (pthread_mutex_t *)(ulong)uVar42;
      local_77 = 0;
      FUN_001071f0(&local_78,&local_328,2);
      iVar11 = DAT_0011d028;
      break;
    case 0x87:
      goto switchD_0010445f_caseD_87;
    }
    goto LAB_001043e0;
  }
  if (uVar42 == 0xffffff7d) goto LAB_00106d48;
  if (uVar42 != 0xffffff7e) goto switchD_0010445f_caseD_2;
  FUN_0010bab0(0);
LAB_00104560:
  if (local_438 != (pthread_mutex_t *)0x0) goto LAB_001065b8;
  cVar9 = '\0';
LAB_00104571:
  if (DAT_0011d410 == 0) {
    cVar10 = FUN_001070e0(&local_328);
    if (cVar10 == '\0') {
      FUN_001076e0();
      local_428 = (pthread_mutex_t *)0x1;
      param_2 = (undefined **)(ulong)local_300[0xb];
      lVar19 = DAT_0011d410;
      goto joined_r0x001058ff;
    }
    if (DAT_0011d400 == '\0') {
      DAT_0011d41b = local_300[0xf];
      goto LAB_001046b0;
    }
  }
  else {
    param_2 = (undefined **)0x0;
    lVar48 = DAT_0011d410;
    lVar19 = DAT_0011d410;
    do {
      while( true ) {
        cVar10 = FUN_001070e0(lVar48);
        if ((cVar10 != '\0') && (*(char *)(lVar48 + 0x37) == '\0')) break;
        pbVar20 = (byte *)(lVar48 + 0x33);
        lVar48 = *(long *)(lVar48 + 0x40);
        param_2 = (undefined **)(ulong)((uint)param_2 | (uint)*pbVar20);
        if (lVar48 == 0) goto LAB_0010463f;
      }
      *(undefined8 *)(lVar48 + 0x20) = local_310._8_8_;
      *(undefined8 *)(lVar48 + 0x28) = local_300._0_8_;
      *(undefined1 *)(lVar48 + 0x30) = local_300[8];
      *(undefined1 *)(lVar48 + 0x31) = local_300[9];
      *(undefined1 *)(lVar48 + 0x36) = local_300[0xe];
      *(undefined1 *)(lVar48 + 0x32) = local_300[10];
      *(undefined1 *)(lVar48 + 0x34) = local_300[0xc];
      *(undefined1 *)(lVar48 + 0x35) = local_300[0xd];
      *(undefined1 *)(lVar48 + 0x37) = local_300[0xf];
      *(undefined1 *)(lVar48 + 0x38) = local_2f0[0];
      *(byte *)(lVar48 + 0x33) = local_300[0xb];
      lVar48 = *(long *)(lVar48 + 0x40);
      param_2 = (undefined **)(ulong)((uint)param_2 | (uint)local_300[0xb]);
    } while (lVar48 != 0);
LAB_0010463f:
    local_428 = (pthread_mutex_t *)0x0;
    do {
      if (1 < (uint)*(byte *)(lVar19 + 0x32) + (uint)*(byte *)(lVar19 + 0x34) +
              (uint)*(byte *)(lVar19 + 0x35) + (uint)*(byte *)(lVar19 + 0x36) +
              (uint)(byte)(*(byte *)(lVar19 + 0x38) | *(byte *)(lVar19 + 0x33) |
                          *(long *)(lVar19 + 0x20) != 0)) {
        *(undefined1 *)(lVar19 + 0x37) = 0;
        pcVar14 = &local_78;
        *(undefined2 *)(lVar19 + 0x30) = 0;
        FUN_00107140(lVar19,pcVar14);
        FUN_00107310(pcVar14);
        goto LAB_001068f4;
      }
      lVar19 = *(long *)(lVar19 + 0x40);
joined_r0x001058ff:
    } while (lVar19 != 0);
    if (DAT_0011d400 == '\0') goto LAB_00104699;
  }
  if (cVar9 == '\0' && local_3f8 == (pthread_mutex_t *)0x0) {
    if ((pcVar15 != (char *)0x0) && (pcVar14 = setlocale(3,""), pcVar14 != (char *)0x0)) {
      if (DAT_0011d861 == '\0') {
        uVar36 = dcgettext(0,"text ordering performed using simple byte comparison",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,&DAT_00117173,uVar36);
      }
      pcVar14 = setlocale(3,(char *)0x0);
      uVar36 = FUN_00110fd0(pcVar14);
      uVar37 = dcgettext(0,"text ordering performed using %s sorting rules",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar37,uVar36);
    }
    uVar36 = dcgettext(0,"failed to set locale",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,&DAT_00117173,uVar36);
  }
LAB_00106b1a:
  s_X___debug_0011d018[0] = cVar9;
  if (cVar9 == '\0') {
    s_X___debug_0011d018[0] = 'o';
  }
  FUN_00107310(s_X___debug_0011d018);
LAB_00106b3d:
  if (local_3d8 == (pthread_mutex_t *)0x0) {
    local_3d8 = (pthread_mutex_t *)0x116cd8;
  }
  pcVar14 = "open failed";
LAB_00106b58:
  uVar36 = dcgettext(0,pcVar14,5);
                    /* WARNING: Subroutine does not return */
  FUN_00107480(uVar36,local_3d8);
}




// Function: proctab_hasher @ 0x6f40

ulong proctab_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 8) % param_2;
}




// Function: proctab_comparator @ 0x6f60

undefined4 proctab_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 8) >> 8),
                  *(int *)(param_1 + 8) == *(int *)(param_2 + 8));
}




// Function: traverse_raw_number @ 0x6f70

int traverse_raw_number(long *param_1)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  char cVar4;
  char *pcVar5;
  int iVar6;
  
  pcVar2 = (char *)*param_1;
  cVar4 = *pcVar2;
  iVar3 = (int)cVar4;
  pcVar5 = pcVar2 + 1;
  if (iVar3 - 0x30U < 10) {
    iVar6 = 0;
    do {
      if ((char)iVar6 < (char)iVar3) {
        iVar6 = iVar3;
      }
      pcVar1 = pcVar2 + 1;
      pcVar2 = pcVar2 + 2;
      if (*pcVar1 != DAT_0011d864) {
        pcVar2 = pcVar5;
      }
      cVar4 = *pcVar2;
      iVar3 = (int)cVar4;
      pcVar5 = pcVar2 + 1;
    } while (iVar3 - 0x30U < 10);
    if (*pcVar1 == DAT_0011d864) {
      *param_1 = (long)(pcVar2 + -1);
      return iVar6;
    }
  }
  else {
    iVar6 = 0;
  }
  if (DAT_0011d868 == cVar4) {
    cVar4 = *pcVar5;
    while ((int)cVar4 - 0x30U < 10) {
      pcVar5 = pcVar5 + 1;
      if ((char)iVar6 < cVar4) {
        iVar6 = (int)cVar4;
      }
      cVar4 = *pcVar5;
    }
  }
  else {
    pcVar5 = pcVar5 + -1;
  }
  *param_1 = (long)pcVar5;
  return iVar6;
}




// Function: getmonth @ 0x7030

undefined4 getmonth(byte *param_1,undefined8 *param_2)

{
  char cVar1;
  byte bVar2;
  byte *pbVar3;
  byte *pbVar4;
  ulong uVar5;
  ulong uVar6;
  ulong uVar7;
  
  cVar1 = (&DAT_0011d740)[*param_1];
  while (cVar1 != '\0') {
    pbVar4 = param_1 + 1;
    param_1 = param_1 + 1;
    cVar1 = (&DAT_0011d740)[*pbVar4];
  }
  uVar7 = 0;
  uVar6 = 0xc;
  do {
    uVar5 = uVar7 + uVar6 >> 1;
    pbVar4 = (&PTR_DAT_0011d060)[uVar5 * 2];
    bVar2 = *pbVar4;
    pbVar3 = param_1;
    while( true ) {
      if (bVar2 == 0) {
        if (param_2 != (undefined8 *)0x0) {
          *param_2 = pbVar3;
        }
        return (&DAT_0011d068)[uVar5 * 4];
      }
      if ((byte)(&DAT_0011d440)[*pbVar3] < bVar2) goto LAB_001070ae;
      if (bVar2 < (byte)(&DAT_0011d440)[*pbVar3]) break;
      bVar2 = pbVar4[1];
      pbVar4 = pbVar4 + 1;
      pbVar3 = pbVar3 + 1;
    }
    uVar7 = uVar5 + 1;
    uVar5 = uVar6;
LAB_001070ae:
    uVar6 = uVar5;
    if (uVar5 <= uVar7) {
      return 0;
    }
  } while( true );
}




// Function: default_key_compare @ 0x70e0

byte default_key_compare(long param_1)

{
  if ((*(long *)(param_1 + 0x20) == 0) && (*(long *)(param_1 + 0x28) == 0)) {
    if (*(char *)(param_1 + 0x30) != '\0') {
      return 0;
    }
    if ((*(char *)(param_1 + 0x31) == '\0') && ((*(ulong *)(param_1 + 0x30) & 0xffff00ff0000) == 0))
    {
      if (*(char *)(param_1 + 0x36) != '\0') {
        return 0;
      }
      if (*(char *)(param_1 + 0x38) == '\0') {
        return *(byte *)(param_1 + 0x33) ^ 1;
      }
    }
  }
  return 0;
}




// Function: key_to_opts @ 0x7140

void key_to_opts(long param_1,undefined1 *param_2)

{
  if (*(short *)(param_1 + 0x30) != 0) {
    *param_2 = 0x62;
    param_2 = param_2 + 1;
  }
  if (*(undefined1 **)(param_1 + 0x20) == &DAT_0011d540) {
    *param_2 = 100;
    param_2 = param_2 + 1;
  }
  if (*(long *)(param_1 + 0x28) != 0) {
    *param_2 = 0x66;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x34) != '\0') {
    *param_2 = 0x67;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x35) != '\0') {
    *param_2 = 0x68;
    param_2 = param_2 + 1;
  }
  if (*(undefined1 **)(param_1 + 0x20) == &DAT_0011d640) {
    *param_2 = 0x69;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x36) != '\0') {
    *param_2 = 0x4d;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x32) != '\0') {
    *param_2 = 0x6e;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x33) != '\0') {
    *param_2 = 0x52;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x37) != '\0') {
    *param_2 = 0x72;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x38) != '\0') {
    *param_2 = 0x56;
    param_2 = param_2 + 1;
  }
  *param_2 = 0;
  return;
}




// Function: set_ordering @ 0x71f0

char * set_ordering(char *param_1,long param_2,uint param_3)

{
  char cVar1;
  
  cVar1 = *param_1;
  if (cVar1 != '\0') {
    do {
      switch(cVar1) {
      case 'M':
        *(undefined1 *)(param_2 + 0x36) = 1;
        break;
      default:
        goto switchD_00107231_caseD_4e;
      case 'R':
        *(undefined1 *)(param_2 + 0x33) = 1;
        break;
      case 'V':
        *(undefined1 *)(param_2 + 0x38) = 1;
        break;
      case 'b':
        if ((param_3 & 0xfffffffd) == 0) {
          *(undefined1 *)(param_2 + 0x30) = 1;
        }
        if (param_3 - 1 < 2) {
          *(undefined1 *)(param_2 + 0x31) = 1;
        }
        break;
      case 'd':
        *(undefined1 **)(param_2 + 0x20) = &DAT_0011d540;
        break;
      case 'f':
        *(undefined1 **)(param_2 + 0x28) = &DAT_0011d440;
        break;
      case 'g':
        *(undefined1 *)(param_2 + 0x34) = 1;
        break;
      case 'h':
        *(undefined1 *)(param_2 + 0x35) = 1;
        break;
      case 'i':
        if (*(long *)(param_2 + 0x20) == 0) {
          *(undefined1 **)(param_2 + 0x20) = &DAT_0011d640;
        }
        break;
      case 'n':
        *(undefined1 *)(param_2 + 0x32) = 1;
        break;
      case 'r':
        *(undefined1 *)(param_2 + 0x37) = 1;
      }
      cVar1 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar1 != '\0');
  }
switchD_00107231_caseD_4e:
  return param_1;
}




// Function: struct_month_cmp @ 0x7300

void struct_month_cmp(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: incompatible_options @ 0x7310

void incompatible_options(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"options \'-%s\' are incompatible",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar1,param_1);
}




// Function: badfieldspec @ 0x7340

void badfieldspec(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  uVar1 = FUN_00110fd0();
  uVar2 = dcgettext(0,param_2,5);
  uVar3 = dcgettext(0,"%s: invalid field specification %s",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3,uVar2,uVar1);
}




// Function: parse_field_count @ 0x7390

undefined8 parse_field_count(undefined8 param_1,undefined8 *param_2,long param_3)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001132f0(param_1,&local_30,10,&local_28,"");
  switch(uVar1) {
  case 0:
  case 2:
    *param_2 = local_28;
    break;
  case 1:
  case 3:
    *param_2 = 0xffffffffffffffff;
    break;
  case 4:
    if (param_3 != 0) {
      uVar2 = FUN_00110fd0(param_1);
      uVar3 = dcgettext(0,param_3,5);
      uVar4 = dcgettext(0,"%s: invalid count at start of %s",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar4,uVar3,uVar2);
    }
    local_30 = 0;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_30;
}




// Function: sort_die @ 0x7480

void sort_die(undefined8 param_1,long param_2)

{
  undefined8 uVar1;
  int *piVar2;
  
  if (param_2 == 0) {
    param_2 = dcgettext(0,"standard output",5);
  }
  uVar1 = FUN_00110c60(0,3,param_2);
  piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar2,"%s: %s",param_1,uVar1);
}




// Function: stream_open @ 0x74e0

FILE * stream_open(char *param_1,char *param_2)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  FILE *pFVar6;
  
  if (*param_2 == 'r') {
    iVar1 = strcmp(param_1,"-");
    if (iVar1 == 0) {
      DAT_0011d418 = 1;
      pFVar6 = stdin;
    }
    else {
      pFVar6 = (FILE *)0x0;
      iVar1 = open(param_1,0x80000);
      if (-1 < iVar1) {
        pFVar6 = fdopen(iVar1,param_2);
      }
    }
    FUN_0010c580(pFVar6,2);
    return pFVar6;
  }
  if (*param_2 != 'w') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open");
  }
  if (param_1 == (char *)0x0) {
    return stdout;
  }
  iVar1 = ftruncate(1,0);
  if (iVar1 == 0) {
    return stdout;
  }
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  if (DAT_0011d2f0 == 0) {
    iVar2 = fstat(1,(stat *)&DAT_0011d260);
    if (iVar2 != 0) {
      DAT_0011d2f0 = *piVar3;
      goto LAB_00107579;
    }
    DAT_0011d2f0 = -1;
  }
  else {
LAB_00107579:
    if (-1 < DAT_0011d2f0) goto LAB_00107593;
  }
  if ((DAT_0011d278 & 0xf000) != 0x8000) {
    return stdout;
  }
LAB_00107593:
  uVar4 = FUN_00110c60(0,3,param_1);
  uVar5 = dcgettext(0,"%s: error truncating",5);
                    /* WARNING: Subroutine does not return */
  error(2,iVar1,uVar5,uVar4);
}




// Function: xfclose @ 0x7660

void xfclose(FILE *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  int iVar2;
  char *pcVar3;
  
  iVar2 = fileno(param_1);
  if (iVar2 == 0) {
    clearerr_unlocked(param_1);
    return;
  }
  if (iVar2 == 1) {
    iVar2 = fflush_unlocked(param_1);
    if (iVar2 != 0) {
      pcVar3 = "fflush failed";
      goto LAB_001076b8;
    }
  }
  else {
    iVar2 = FUN_0010c5b0(param_1);
    if (iVar2 != 0) {
      pcVar3 = "close failed";
LAB_001076b8:
      uVar1 = dcgettext(0,pcVar3,5);
                    /* WARNING: Subroutine does not return */
      FUN_00107480(uVar1,param_2);
    }
  }
  return;
}




// Function: insertkey @ 0x76e0

void insertkey(undefined8 param_1)

{
  long lVar1;
  long lVar2;
  long *plVar3;
  long lVar4;
  
  lVar2 = FUN_00112ef0(param_1,0x48);
  plVar3 = &DAT_0011d410;
  lVar1 = DAT_0011d410;
  if (DAT_0011d410 != 0) {
    do {
      lVar4 = lVar1;
      lVar1 = *(long *)(lVar4 + 0x40);
    } while (lVar1 != 0);
    plVar3 = (long *)(lVar4 + 0x40);
  }
  *plVar3 = lVar2;
  *(undefined8 *)(lVar2 + 0x40) = 0;
  return;
}




// Function: add_temp_dir @ 0x7730

void add_temp_dir(undefined8 param_1)

{
  if (DAT_0011d428 == DAT_0011d420) {
    DAT_0011d430 = FUN_00112d00(DAT_0011d430,&DAT_0011d420,8);
  }
  *(undefined8 *)(DAT_0011d430 + DAT_0011d428 * 8) = param_1;
  DAT_0011d428 = DAT_0011d428 + 1;
  return;
}




// Function: xstrxfrm @ 0x7800

void xstrxfrm(char *param_1,char *param_2,size_t param_3)

{
  int *piVar1;
  undefined8 uVar2;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  strxfrm(param_1,param_2,param_3);
  if (*piVar1 == 0) {
    return;
  }
  uVar2 = dcgettext(0,"string transformation failed",5);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar1,uVar2);
}




// Function: reap @ 0x78c0

__pid_t reap(int param_1)

{
  __pid_t _Var1;
  long lVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  uint local_3c;
  undefined1 local_38 [8];
  __pid_t local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  _Var1 = -1;
  if (param_1 != 0) {
    _Var1 = param_1;
  }
  _Var1 = waitpid(_Var1,(int *)&local_3c,(uint)(param_1 == 0));
  if (_Var1 < 0) {
    uVar4 = FUN_001108e0(4,DAT_0011d408);
    uVar5 = dcgettext(0,"waiting for %s [-d]",5);
    piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar3,uVar5,uVar4);
  }
  if (_Var1 != 0) {
    if (param_1 < 1) {
      local_30 = _Var1;
      lVar2 = FUN_0010dd00(DAT_0011d360,local_38);
      if (lVar2 == 0) goto LAB_0010791a;
      *(undefined1 *)(lVar2 + 0xc) = 2;
    }
    if ((local_3c >> 8 & 0xff) != 0 || (local_3c & 0x7f) != 0) {
      uVar4 = FUN_001108e0(4,DAT_0011d408);
      uVar5 = dcgettext(0,"%s [-d] terminated abnormally",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar5,uVar4);
    }
    DAT_0011d35c = DAT_0011d35c + -1;
  }
LAB_0010791a:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return _Var1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: async_safe_die @ 0x79f0

void async_safe_die(int param_1,char *param_2)

{
  size_t sVar1;
  char *__s;
  long in_FS_OFFSET;
  undefined1 local_2c [12];
  undefined8 local_20;
  
  local_20 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  sVar1 = strlen(param_2);
  write(2,param_2,sVar1);
  if (param_1 != 0) {
    __s = (char *)FUN_0010e120(param_1,local_2c);
    write(2,": errno ",8);
    sVar1 = strlen(__s);
    write(2,__s,sVar1);
  }
  write(2,"\n",1);
                    /* WARNING: Subroutine does not return */
  _exit(2);
}




// Function: register_proc @ 0x7a90

void register_proc(long param_1)

{
  long lVar1;
  
  if (DAT_0011d360 == 0) {
    DAT_0011d360 = FUN_0010d640(0x2f,0,FUN_00106f40,FUN_00106f60,0);
    if (DAT_0011d360 == 0) goto LAB_00107ae4;
  }
  lVar1 = DAT_0011d360;
  *(undefined1 *)(param_1 + 0xc) = 1;
  lVar1 = FUN_0010dca0(lVar1,param_1);
  if (lVar1 != 0) {
    return;
  }
LAB_00107ae4:
                    /* WARNING: Subroutine does not return */
  FUN_00112fb0();
}




// Function: init_node @ 0x7af0

long * init_node(long *param_1,long *param_2,long param_3,ulong param_4,long param_5,char param_6
                   )

{
  long *plVar1;
  long lVar2;
  ulong uVar3;
  long lVar4;
  long *plVar5;
  long *plVar6;
  long lVar7;
  
  while( true ) {
    plVar5 = param_2;
    lVar7 = param_3 + param_5 * -0x20;
    if (param_6 == '\0') {
      uVar3 = (ulong)param_1[6] >> 1;
      lVar4 = param_1[6] - uVar3;
      plVar6 = param_1 + 3;
    }
    else {
      uVar3 = (ulong)param_1[5] >> 1;
      lVar4 = param_1[5] - uVar3;
      plVar6 = param_1 + 2;
    }
    param_3 = lVar7 + uVar3 * -0x20;
    plVar5[7] = (long)param_1;
    lVar2 = param_1[10];
    plVar1 = plVar5 + 0x10;
    plVar5[4] = (long)plVar6;
    plVar5[2] = lVar7;
    *plVar5 = lVar7;
    plVar5[3] = param_3;
    plVar5[1] = param_3;
    plVar5[5] = uVar3;
    plVar5[6] = lVar4;
    *(int *)(plVar5 + 10) = (int)lVar2 + 1;
    *(undefined1 *)((long)plVar5 + 0x54) = 0;
    pthread_mutex_init((pthread_mutex_t *)(plVar5 + 0xb),(pthread_mutexattr_t *)0x0);
    if (param_4 < 2) break;
    plVar5[8] = (long)plVar1;
    uVar3 = param_4 >> 1;
    param_4 = param_4 - uVar3;
    param_2 = (long *)init_node(plVar5,plVar1,lVar7,uVar3,param_5);
    plVar5[9] = (long)param_2;
    param_6 = '\0';
    param_1 = plVar5;
  }
  plVar5[8] = 0;
  plVar5[9] = 0;
  return plVar1;
}




// Function: fillbuf @ 0x7ef0

undefined8 fillbuf(long *param_1,FILE *param_2,undefined8 param_3)

{
  long lVar1;
  byte bVar2;
  long *plVar3;
  byte *pbVar4;
  ulong uVar5;
  size_t sVar6;
  undefined8 uVar7;
  undefined1 *puVar8;
  long lVar9;
  byte *pbVar10;
  ulong uVar11;
  byte *pbVar12;
  size_t sVar13;
  byte *__ptr;
  byte *pbVar14;
  long lVar15;
  byte *pbVar16;
  long in_FS_OFFSET;
  byte *local_88;
  ulong local_48;
  long local_40;
  
  plVar3 = DAT_0011d410;
  bVar2 = DAT_0011d038;
  lVar15 = DAT_0011d030;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar1 = param_1[5];
  if ((char)param_1[6] == '\0') {
    sVar13 = param_1[1];
    sVar6 = param_1[4];
    if (sVar13 == sVar6) {
      lVar9 = param_1[2];
    }
    else {
      memmove((void *)*param_1,(void *)(*param_1 + (sVar13 - sVar6)),sVar6);
      sVar13 = param_1[4];
      param_1[2] = 0;
      lVar9 = 0;
      param_1[1] = sVar13;
    }
    uVar11 = lVar15 - 0x22;
    pbVar12 = (byte *)*param_1;
    lVar15 = param_1[3];
    do {
      __ptr = pbVar12 + sVar13;
      pbVar14 = pbVar12 + lVar9 * -0x20 + lVar15;
      pbVar10 = pbVar12 + ((lVar15 - lVar1 * lVar9) - (long)__ptr);
      if (lVar9 != 0) {
        pbVar12 = (byte *)(*(long *)(pbVar14 + 8) + *(long *)pbVar14);
      }
      do {
        if (pbVar10 <= (byte *)(lVar1 + 1)) break;
        uVar5 = (ulong)(pbVar10 + -1) / (ulong)(lVar1 + 1);
        sVar6 = fread_unlocked(__ptr,1,uVar5,param_2);
        pbVar10 = pbVar10 + -sVar6;
        local_88 = __ptr + sVar6;
        if (uVar5 != sVar6) {
          if ((param_2->_flags & 0x20U) != 0) {
            uVar7 = dcgettext(0,"read failed",5);
                    /* WARNING: Subroutine does not return */
            FUN_00107480(uVar7,param_3);
          }
          if ((param_2->_flags & 0x10U) != 0) {
            *(undefined1 *)(param_1 + 6) = 1;
            if ((byte *)*param_1 == local_88) goto LAB_001080f5;
            if ((pbVar12 != local_88) && (local_88[-1] != bVar2)) {
              *local_88 = bVar2;
              local_88 = local_88 + 1;
            }
          }
        }
        pbVar4 = pbVar12;
        pbVar16 = pbVar14;
        while (pbVar14 = pbVar16, pbVar12 = pbVar4,
              puVar8 = memchr(__ptr,(int)(char)bVar2,(long)local_88 - (long)__ptr),
              puVar8 != (undefined1 *)0x0) {
          __ptr = puVar8 + 1;
          pbVar16 = pbVar14 + -0x20;
          *puVar8 = 0;
          *(byte **)pbVar16 = pbVar12;
          uVar5 = (long)__ptr - (long)pbVar12;
          *(ulong *)(pbVar14 + -0x18) = uVar5;
          if (uVar11 < uVar5) {
            uVar11 = uVar5;
          }
          pbVar10 = pbVar10 + -lVar1;
          pbVar4 = __ptr;
          if (plVar3 != (long *)0x0) {
            if (plVar3[2] != -1) {
              puVar8 = (undefined1 *)FUN_00107ca0(pbVar12,uVar5,plVar3);
            }
            lVar15 = *plVar3;
            *(undefined1 **)(pbVar14 + -8) = puVar8;
            if (lVar15 == -1) {
              if ((char)plVar3[6] != '\0') {
                while ((&DAT_0011d740)[*pbVar12] != '\0') {
                  pbVar12 = pbVar12 + 1;
                }
              }
              *(byte **)(pbVar14 + -0x10) = pbVar12;
              pbVar4 = __ptr;
            }
            else {
              uVar7 = FUN_00107dd0(pbVar12,uVar5,plVar3);
              *(undefined8 *)(pbVar14 + -0x10) = uVar7;
              pbVar4 = __ptr;
            }
          }
        }
        __ptr = local_88;
      } while ((char)param_1[6] == '\0');
      lVar15 = *param_1;
      param_1[1] = (long)__ptr - lVar15;
      lVar9 = (lVar15 + param_1[3]) - (long)pbVar14 >> 5;
      param_1[2] = lVar9;
      if (lVar9 != 0) goto LAB_00108220;
      local_48 = (ulong)param_1[3] >> 5;
      pbVar12 = (byte *)FUN_00112d00(lVar15,&local_48,0x20);
      lVar15 = local_48 << 5;
      *param_1 = (long)pbVar12;
      sVar13 = param_1[1];
      param_1[3] = lVar15;
      lVar9 = param_1[2];
    } while( true );
  }
LAB_001080f5:
  uVar7 = 0;
LAB_001080f7:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
LAB_00108220:
  DAT_0011d030 = uVar11 + 0x22;
  uVar7 = 1;
  param_1[4] = (long)__ptr - (long)pbVar12;
  goto LAB_001080f7;
}




// Function: sighandler @ 0x8270

void sighandler(int param_1)

{
  undefined8 *puVar1;
  
  for (puVar1 = DAT_0011d368; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    unlink((char *)((long)puVar1 + 0xd));
  }
  DAT_0011d368 = (undefined8 *)0x0;
  signal(param_1,(__sighandler_t)0x0);
  raise(param_1);
  return;
}




// Function: write_line @ 0x82d0

void write_line(undefined8 *param_1,_IO_FILE *param_2,long param_3)

{
  char *pcVar1;
  byte bVar2;
  char cVar3;
  int iVar4;
  size_t sVar5;
  byte *pbVar6;
  byte *pbVar7;
  undefined8 uVar8;
  uint uVar9;
  byte *pbVar10;
  long *plVar11;
  byte *pbVar12;
  long lVar13;
  size_t __n;
  long lVar14;
  long in_FS_OFFSET;
  byte *local_50;
  byte *local_48;
  long local_40;
  
  pbVar10 = (byte *)*param_1;
  __n = param_1[1];
  pbVar7 = pbVar10 + __n;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_3 != 0) || (DAT_0011d400 == '\0')) {
    pbVar7[-1] = DAT_0011d038;
    sVar5 = fwrite_unlocked(pbVar10,1,__n,param_2);
    if (__n != sVar5) {
      uVar8 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_00107480(uVar8,param_3);
    }
    pbVar7[-1] = 0;
LAB_00108333:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if (pbVar10 < pbVar7) {
    do {
      bVar2 = *pbVar10;
      pbVar10 = pbVar10 + 1;
      if (bVar2 == 9) {
        pbVar6 = (byte *)param_2->_IO_write_ptr;
        uVar9 = 0x3e;
        bVar2 = 0x3e;
        if (pbVar6 < param_2->_IO_write_end) goto LAB_0010838c;
LAB_001083be:
        iVar4 = __overflow(param_2,uVar9 & 0xff);
        if (iVar4 == -1) {
          uVar8 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
          FUN_00107480(uVar8,0);
        }
      }
      else {
        uVar9 = (uint)(char)bVar2;
        if (pbVar7 == pbVar10) {
          uVar9 = 10;
          bVar2 = 10;
        }
        pbVar6 = (byte *)param_2->_IO_write_ptr;
        if (param_2->_IO_write_end <= pbVar6) goto LAB_001083be;
LAB_0010838c:
        param_2->_IO_write_ptr = (char *)(pbVar6 + 1);
        *pbVar6 = bVar2;
      }
    } while (pbVar7 != pbVar10);
    pbVar10 = (byte *)*param_1;
    __n = param_1[1];
  }
  pbVar7 = pbVar10 + (__n - 1);
  plVar11 = DAT_0011d410;
  do {
    pbVar6 = pbVar10;
    if (plVar11 != (long *)0x0) {
      lVar14 = *plVar11;
      if (lVar14 == -1) {
        if (plVar11[2] != -1) goto LAB_0010844e;
LAB_0010845f:
        if ((lVar14 != -1) || ((char)plVar11[6] == '\0')) goto LAB_0010846b;
      }
      else {
        pbVar6 = (byte *)FUN_00107dd0(pbVar10,__n,plVar11);
        if (plVar11[2] != -1) {
LAB_0010844e:
          pbVar7 = (byte *)FUN_00107ca0(pbVar10,__n,plVar11);
          goto LAB_0010845f;
        }
LAB_0010846b:
        if ((*(char *)((long)plVar11 + 0x36) == '\0') && ((plVar11[6] & 0xffff00ff0000U) == 0))
        goto LAB_00108531;
      }
      bVar2 = *pbVar7;
      *pbVar7 = 0;
      cVar3 = (&DAT_0011d740)[*pbVar6];
      while (cVar3 != '\0') {
        pbVar12 = pbVar6 + 1;
        pbVar6 = pbVar6 + 1;
        cVar3 = (&DAT_0011d740)[*pbVar12];
      }
      local_50 = pbVar7;
      if (pbVar6 <= pbVar7) {
        if (*(char *)((long)plVar11 + 0x36) == '\0') {
          if (*(char *)((long)plVar11 + 0x34) == '\0') {
            if ((plVar11[6] & 0xff0000ff0000U) != 0) {
              local_48 = pbVar6;
              if (pbVar6 < pbVar7) {
                local_48 = pbVar6 + (*pbVar6 == 0x2d);
              }
              local_50 = pbVar6;
              cVar3 = FUN_00106f70(&local_48);
              if (('/' < cVar3) && (local_50 = local_48, *(char *)((long)plVar11 + 0x35) != '\0')) {
                local_50 = local_48 + ((&DAT_00115360)[*local_48] != '\0');
              }
            }
          }
          else {
            local_50 = pbVar6;
            strtold((char *)pbVar6,(char **)&local_50);
          }
        }
        else {
          local_50 = pbVar6;
          FUN_00107030(pbVar6,&local_50);
        }
      }
      *pbVar7 = bVar2;
      pbVar7 = local_50;
    }
LAB_00108531:
    while( true ) {
      iVar4 = FUN_0010e260(pbVar10,(long)pbVar6 - (long)pbVar10,0);
      lVar14 = (long)iVar4;
      if (pbVar10 < pbVar6) {
        do {
          pbVar12 = pbVar10 + 1;
          lVar14 = lVar14 + (ulong)(*pbVar10 == 9);
          pbVar10 = pbVar12;
        } while (pbVar6 != pbVar12);
      }
      iVar4 = FUN_0010e260(pbVar6,(long)pbVar7 - (long)pbVar6,0);
      lVar13 = (long)iVar4;
      if (pbVar6 < pbVar7) {
        do {
          pbVar10 = pbVar6 + 1;
          lVar13 = lVar13 + (ulong)(*pbVar6 == 9);
          pbVar6 = pbVar10;
        } while (pbVar7 != pbVar10);
      }
      while (lVar14 != 0) {
        lVar14 = lVar14 + -1;
        pcVar1 = stdout->_IO_write_ptr;
        if (pcVar1 < stdout->_IO_write_end) {
          stdout->_IO_write_ptr = pcVar1 + 1;
          *pcVar1 = ' ';
        }
        else {
          __overflow(stdout,0x20);
        }
      }
      if (lVar13 == 0) {
        uVar8 = dcgettext(0,"^ no match for key\n",5);
        __printf_chk(1,uVar8);
      }
      else {
        do {
          pcVar1 = stdout->_IO_write_ptr;
          if (pcVar1 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = pcVar1 + 1;
            *pcVar1 = '_';
          }
          else {
            __overflow(stdout,0x5f);
          }
          lVar13 = lVar13 + -1;
        } while (lVar13 != 0);
        pcVar1 = stdout->_IO_write_ptr;
        if (pcVar1 < stdout->_IO_write_end) {
          stdout->_IO_write_ptr = pcVar1 + 1;
          *pcVar1 = '\n';
        }
        else {
          __overflow(stdout,10);
        }
      }
      if (plVar11 == (long *)0x0) goto LAB_00108333;
      plVar11 = (long *)plVar11[8];
      if (plVar11 != (long *)0x0) break;
      if ((DAT_0011d419 != '\0') || (DAT_0011d41a != '\0')) goto LAB_00108333;
      pbVar10 = (byte *)*param_1;
      pbVar7 = pbVar10 + param_1[1] + -1;
      pbVar6 = pbVar10;
    }
    pbVar10 = (byte *)*param_1;
    __n = param_1[1];
    pbVar7 = pbVar10 + (__n - 1);
  } while( true );
}




// Function: pipe_fork @ 0x8770

__pid_t pipe_fork(int *param_1,long param_2)

{
  undefined8 uVar1;
  int iVar2;
  int iVar3;
  __pid_t _Var4;
  int *piVar5;
  long in_FS_OFFSET;
  double local_e0;
  __sigset_t local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_0010e990(param_1,0x80000);
  if (iVar2 < 0) {
    _Var4 = -1;
  }
  else {
    if (DAT_0011d024 + 1U < DAT_0011d35c) {
      FUN_001078c0();
      do {
        if ((int)DAT_0011d35c < 1) break;
        iVar2 = FUN_001078c0();
      } while (iVar2 != 0);
    }
    piVar5 = __errno_location();
    local_e0 = DAT_001170e8;
    do {
      param_2 = param_2 + -1;
      iVar3 = pthread_sigmask(0,(__sigset_t *)&DAT_0011d380,&local_c0);
      uVar1 = DAT_0011d368;
      DAT_0011d368 = 0;
      _Var4 = fork();
      iVar2 = *piVar5;
      if (_Var4 == 0) {
        if (iVar3 == 0) goto LAB_00108895;
LAB_001088f0:
        _Var4 = 0;
        close(0);
        close(1);
        goto LAB_001088ba;
      }
      DAT_0011d368 = uVar1;
      if (iVar3 == 0) {
LAB_00108895:
        pthread_sigmask(2,&local_c0,(__sigset_t *)0x0);
      }
      *piVar5 = iVar2;
      if (-1 < _Var4) {
        if (_Var4 == 0) goto LAB_001088f0;
        DAT_0011d35c = DAT_0011d35c + 1;
        goto LAB_001088ba;
      }
      if (iVar2 != 0xb) goto LAB_00108943;
      FUN_00113180(local_e0);
      local_e0 = local_e0 + local_e0;
      do {
        if ((int)DAT_0011d35c < 1) break;
        iVar2 = FUN_001078c0();
      } while (iVar2 != 0);
    } while (param_2 != 0);
    iVar2 = *piVar5;
LAB_00108943:
    close(*param_1);
    close(param_1[1]);
    *piVar5 = iVar2;
  }
LAB_001088ba:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return _Var4;
}




// Function: open_input_files @ 0x8980

long open_input_files(undefined8 *param_1,long param_2,undefined8 *param_3)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  long *plVar4;
  long lVar5;
  int *piVar6;
  FILE *pFVar7;
  long lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  long *plVar11;
  long lVar12;
  long in_FS_OFFSET;
  int local_58;
  int local_54;
  undefined4 local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar4 = (long *)FUN_00112bf0(param_2);
  *param_3 = plVar4;
  if (param_2 == 0) {
    lVar12 = 0;
  }
  else {
    lVar12 = 0;
    plVar11 = plVar4;
    do {
      lVar5 = param_1[1];
      if ((lVar5 == 0) || (*(char *)(lVar5 + 0xc) == '\0')) {
        lVar5 = FUN_001074e0(*param_1);
        *plVar11 = lVar5;
        if (lVar5 == 0) break;
      }
      else {
        if (*(char *)(lVar5 + 0xc) == '\x01') {
          uVar1 = *(undefined4 *)(lVar5 + 8);
          local_50 = uVar1;
          lVar8 = FUN_0010dd00(DAT_0011d360);
          if (lVar8 != 0) {
            *(undefined1 *)(lVar8 + 0xc) = 2;
            FUN_001078c0(uVar1);
          }
        }
        iVar2 = open((char *)(lVar5 + 0xd),0);
        if (iVar2 < 0) {
          plVar4[lVar12] = 0;
          break;
        }
        iVar3 = FUN_00108770(&local_58,9);
        if (iVar3 == -1) {
          piVar6 = __errno_location();
          if (*piVar6 != 0x18) {
            uVar9 = FUN_001108e0(4,DAT_0011d408);
            uVar10 = dcgettext(0,"couldn\'t create process for %s -d",5);
                    /* WARNING: Subroutine does not return */
            error(2,*piVar6,uVar10,uVar9);
          }
          close(iVar2);
          *piVar6 = 0x18;
          plVar4[lVar12] = 0;
          break;
        }
        if (iVar3 == 0) {
          close(local_58);
          if (iVar2 != 0) {
            FUN_00107c90(iVar2,0);
          }
          if (local_54 != 1) {
            FUN_00107c90(local_54,1);
          }
          execlp(DAT_0011d408,DAT_0011d408,&DAT_00116be6,0);
          piVar6 = __errno_location();
          FUN_001079f0(*piVar6,"couldn\'t execute compress program (with -d)");
          goto LAB_00108bbc;
        }
        *(int *)(lVar5 + 8) = iVar3;
        FUN_00107a90(lVar5);
        close(iVar2);
        close(local_54);
        pFVar7 = fdopen(local_58,"r");
        if (pFVar7 == (FILE *)0x0) {
          piVar6 = __errno_location();
          iVar2 = *piVar6;
          close(local_58);
          *plVar11 = 0;
          *piVar6 = iVar2;
          break;
        }
        *plVar11 = (long)pFVar7;
      }
      lVar12 = lVar12 + 1;
      param_1 = param_1 + 2;
      plVar11 = plVar11 + 1;
    } while (param_2 != lVar12);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar12;
  }
LAB_00108bbc:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: maybe_create_temp @ 0x8c20

undefined8 * maybe_create_temp(undefined8 *param_1,char param_2)

{
  void *__dest;
  char *__s;
  int iVar1;
  int iVar2;
  size_t __n;
  undefined8 *__ptr;
  int *piVar3;
  FILE *pFVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  long in_FS_OFFSET;
  bool bVar8;
  int local_c8;
  int local_c4;
  __sigset_t local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __s = *(char **)(DAT_0011d430 + DAT_0011d248 * 8);
  __n = strlen(__s);
  __ptr = (undefined8 *)FUN_00112ab0(__n + 0x20 & 0xfffffffffffffff8);
  __dest = (void *)((long)__ptr + 0xd);
  memcpy(__dest,__s,__n);
  *(undefined8 *)((long)__ptr + __n + 0xd) = s__sortXXXXXX_001152f0._0_8_;
  *(undefined4 *)((long)__ptr + __n + 0x15) = s__sortXXXXXX_001152f0._8_4_;
  *__ptr = 0;
  DAT_0011d248 = DAT_0011d248 + 1;
  if (DAT_0011d248 == DAT_0011d428) {
    DAT_0011d248 = 0;
  }
  iVar1 = pthread_sigmask(0,(__sigset_t *)&DAT_0011d380,&local_c0);
  local_c8 = CONCAT31(local_c8._1_3_,iVar1 == 0);
  iVar1 = FUN_00111d70(__dest,0x80000);
  piVar3 = __errno_location();
  if (iVar1 < 0) {
    iVar1 = *piVar3;
    if ((char)local_c8 != '\0') {
      pthread_sigmask(2,&local_c0,(__sigset_t *)0x0);
      *piVar3 = iVar1;
    }
    if ((iVar1 != 0x18) || (param_2 != '\x01')) {
      uVar5 = FUN_001108e0(4,__s);
      uVar6 = dcgettext(0,"cannot create temporary file in %s",5);
                    /* WARNING: Subroutine does not return */
      error(2,*piVar3,uVar6,uVar5);
    }
    puVar7 = (undefined8 *)0x0;
    free(__ptr);
  }
  else {
    puVar7 = __ptr;
    *(undefined8 **)PTR_DAT_0011d040 = __ptr;
    PTR_DAT_0011d040 = (undefined *)puVar7;
    if ((char)local_c8 != '\0') {
      iVar2 = *piVar3;
      pthread_sigmask(2,&local_c0,(__sigset_t *)0x0);
      *piVar3 = iVar2;
    }
    bVar8 = DAT_0011d408 != (char *)0x0;
    *(undefined1 *)((long)__ptr + 0xc) = 0;
    if (bVar8) {
      iVar2 = FUN_00108770(&local_c8,4);
      *(int *)(__ptr + 1) = iVar2;
      if (iVar2 < 1) {
        if (iVar2 == 0) {
          close(local_c4);
          if (iVar1 != 1) {
            FUN_00107c90(iVar1,1);
          }
          if (local_c8 != 0) {
            FUN_00107c90(local_c8,0);
          }
          execlp(DAT_0011d408,DAT_0011d408,0);
          FUN_001079f0(*piVar3,"couldn\'t execute compress program");
          goto LAB_00108e6d;
        }
      }
      else {
        close(iVar1);
        close(local_c8);
        FUN_00107a90(__ptr);
        iVar1 = local_c4;
      }
    }
    pFVar4 = fdopen(iVar1,"w");
    *param_1 = pFVar4;
    puVar7 = __ptr;
    if (pFVar4 == (FILE *)0x0) {
      uVar5 = dcgettext(0,"couldn\'t create temporary file",5);
                    /* WARNING: Subroutine does not return */
      FUN_00107480(uVar5,__dest);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar7;
  }
LAB_00108e6d:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: exit_cleanup @ 0x8ed0

void exit_cleanup(void)

{
  undefined8 *puVar1;
  int iVar2;
  long in_FS_OFFSET;
  __sigset_t local_a0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0011d368 != (undefined8 *)0x0) {
    iVar2 = pthread_sigmask(0,(__sigset_t *)&DAT_0011d380,&local_a0);
    for (puVar1 = DAT_0011d368; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
      unlink((char *)((long)puVar1 + 0xd));
    }
    DAT_0011d368 = (undefined8 *)0x0;
    if (iVar2 == 0) {
      pthread_sigmask(2,&local_a0,(__sigset_t *)0x0);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_0010c4c0();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: zaptemp @ 0x8f90

void zaptemp(char *param_1)

{
  long *plVar1;
  long *__ptr;
  int iVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  long in_FS_OFFSET;
  char local_c8 [8];
  __sigset_t local_c0;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar1 = (long *)&DAT_0011d368;
  for (__ptr = DAT_0011d368; param_1 != (char *)((long)__ptr + 0xd); __ptr = (long *)*__ptr) {
    plVar1 = __ptr;
  }
  if (*(char *)((long)__ptr + 0xc) == '\x01') {
    local_c0.__val[0]._0_4_ = (undefined4)__ptr[1];
    lVar7 = FUN_0010dd00(DAT_0011d360,local_c8);
    if (lVar7 != 0) {
      *(undefined1 *)(lVar7 + 0xc) = 2;
      FUN_001078c0();
    }
  }
  lVar7 = *__ptr;
  iVar2 = pthread_sigmask(0,(__sigset_t *)&DAT_0011d380,&local_c0);
  local_c8[0] = iVar2 == 0;
  iVar3 = unlink(param_1);
  piVar4 = __errno_location();
  iVar2 = *piVar4;
  *plVar1 = lVar7;
  if (local_c8[0] != '\0') {
    pthread_sigmask(2,&local_c0,(__sigset_t *)0x0);
  }
  if (iVar3 != 0) {
    uVar5 = FUN_00110c60(0,3,param_1);
    uVar6 = dcgettext(0,"warning: cannot remove: %s",5);
                    /* WARNING: Subroutine does not return */
    error(0,iVar2,uVar6,uVar5);
  }
  if (lVar7 == 0) {
    PTR_DAT_0011d040 = (undefined *)plVar1;
  }
  free(__ptr);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: keycompare @ 0x9120

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int keycompare(undefined8 *param_1,long *param_2)

{
  byte *pbVar1;
  byte bVar2;
  long lVar3;
  long lVar4;
  char cVar5;
  byte bVar6;
  int iVar7;
  int iVar8;
  byte *pbVar9;
  byte *__nptr;
  size_t sVar10;
  byte *pbVar11;
  long lVar12;
  byte *pbVar13;
  ulong uVar14;
  ulong len;
  long *plVar15;
  byte *pbVar16;
  byte *pbVar17;
  ulong len_00;
  size_t len_01;
  ulong uVar18;
  byte *pbVar19;
  long in_FS_OFFSET;
  longdouble in_ST0;
  longdouble lVar20;
  longdouble lVar21;
  longdouble in_ST1;
  longdouble in_ST2;
  longdouble in_ST3;
  longdouble in_ST4;
  longdouble in_ST5;
  longdouble lVar22;
  longdouble in_ST6;
  longdouble in_ST7;
  byte local_2132;
  byte local_2131;
  ulong local_2118;
  byte *local_2110;
  byte *local_2100;
  byte *local_20f8;
  ulong local_20f0;
  byte *local_2098;
  byte *local_2090;
  undefined1 local_2088 [16];
  longdouble local_2078;
  uchar local_2068 [16];
  uchar local_2058 [16];
  MD5_CTX local_2048;
  MD5_CTX local_1fec;
  byte local_1f88;
  byte local_1f87 [3999];
  byte local_fe8 [4008];
  long local_40;
  
  __nptr = (byte *)param_1[2];
  pbVar16 = (byte *)param_2[2];
  pbVar17 = (byte *)param_1[3];
  pbVar9 = (byte *)param_2[3];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar15 = DAT_0011d410;
LAB_001091a0:
  lVar3 = plVar15[5];
  lVar4 = plVar15[4];
  if (pbVar17 <= __nptr) {
    pbVar17 = __nptr;
  }
  if (pbVar9 <= pbVar16) {
    pbVar9 = pbVar16;
  }
  len_00 = (long)pbVar17 - (long)__nptr;
  len_01 = (long)pbVar9 - (long)pbVar16;
  if (((DAT_0011d861 != '\0') || ((plVar15[6] & 0xffffffffff0000U) != 0)) ||
     ((char)plVar15[7] != '\0')) {
    if (lVar3 == 0 && lVar4 == 0) {
      local_2132 = *pbVar17;
      *pbVar17 = 0;
      local_2131 = *pbVar9;
      *pbVar9 = 0;
      pbVar9 = pbVar16;
    }
    else {
      pbVar17 = &local_1f88;
      local_2110 = (byte *)0x0;
      if (len_00 + 2 + len_01 < 0xfa1) {
        pbVar9 = local_1f87 + len_00;
        if (len_00 == 0) goto LAB_00109887;
LAB_001095c1:
        pbVar19 = __nptr + len_00;
        len_00 = 0;
        do {
          bVar6 = *__nptr;
          if ((lVar4 == 0) || (*(char *)(lVar4 + (ulong)bVar6) == '\0')) {
            if (lVar3 != 0) {
              bVar6 = *(byte *)(lVar3 + (ulong)bVar6);
            }
            pbVar17[len_00] = bVar6;
            len_00 = len_00 + 1;
          }
          __nptr = __nptr + 1;
        } while (pbVar19 != __nptr);
        pbVar17[len_00] = 0;
        __nptr = pbVar17;
      }
      else {
        pbVar17 = (byte *)FUN_00112ab0();
        pbVar9 = pbVar17 + len_00 + 1;
        local_2110 = pbVar17;
        if (len_00 != 0) goto LAB_001095c1;
LAB_00109887:
        *pbVar17 = 0;
        __nptr = pbVar17;
      }
      pbVar17 = pbVar9;
      if (len_01 != 0) {
        pbVar17 = pbVar16 + len_01;
        len_01 = 0;
        do {
          bVar6 = *pbVar16;
          if ((lVar4 == 0) || (*(char *)(lVar4 + (ulong)bVar6) == '\0')) {
            if (lVar3 != 0) {
              bVar6 = *(byte *)(lVar3 + (ulong)bVar6);
            }
            pbVar9[len_01] = bVar6;
            len_01 = len_01 + 1;
          }
          pbVar16 = pbVar16 + 1;
        } while (pbVar17 != pbVar16);
        pbVar17 = pbVar9 + len_01;
      }
      *pbVar17 = 0;
    }
    if (*(char *)((long)plVar15 + 0x32) == '\0') {
      if (*(char *)((long)plVar15 + 0x34) != '\0') {
        lVar21 = in_ST2;
        lVar20 = in_ST3;
        in_ST2 = in_ST4;
        in_ST3 = in_ST5;
        in_ST4 = in_ST6;
        in_ST5 = in_ST7;
        strtold((char *)__nptr,(char **)&local_2098);
        strtold((char *)pbVar9,(char **)&local_2090);
        in_ST6 = in_ST5;
        in_ST7 = in_ST5;
        if (__nptr == local_2098) {
          iVar7 = -(uint)(local_2090 != pbVar9);
        }
        else if (pbVar9 == local_2090) {
LAB_00109748:
          iVar7 = 1;
        }
        else if (in_ST0 < in_ST1) {
LAB_0010a045:
          iVar7 = -1;
        }
        else {
          iVar7 = 1;
          if ((in_ST0 <= in_ST1) && (iVar7 = 0, in_ST0 != in_ST1)) {
            if (!NAN(in_ST1)) goto LAB_0010a045;
            if (!NAN(in_ST0)) goto LAB_00109748;
            lVar21 = in_ST2;
            in_ST2 = in_ST4;
            in_ST4 = in_ST5;
            lVar22 = in_ST5;
            strtold((char *)__nptr,(char **)0x0);
            in_ST6 = lVar22;
            strtold((char *)pbVar9,(char **)0x0);
            in_ST7 = in_ST6;
            local_2078 = lVar20;
            iVar7 = memcmp(local_2088,&local_2078,0x10);
            lVar20 = in_ST3;
            in_ST3 = in_ST5;
            in_ST5 = lVar22;
          }
        }
        goto LAB_001092a0;
      }
      if (*(char *)((long)plVar15 + 0x35) == '\0') {
        if (*(char *)((long)plVar15 + 0x36) == '\0') {
          if (*(char *)((long)plVar15 + 0x33) == '\0') {
            if ((char)plVar15[7] == '\0') {
              lVar21 = in_ST0;
              lVar20 = in_ST1;
              if (len_00 == 0) {
                iVar7 = -(uint)(len_01 != 0);
              }
              else {
                iVar7 = 1;
                if (len_01 != 0) {
                  iVar7 = FUN_00113120(__nptr,len_00 + 1,pbVar9,len_01 + 1);
                  lVar21 = in_ST0;
                  lVar20 = in_ST1;
                }
              }
            }
            else {
              iVar7 = FUN_0010ca00(__nptr,len_00,pbVar9,len_01);
              lVar21 = in_ST0;
              lVar20 = in_ST1;
            }
          }
          else {
            local_1fec.A = _DAT_0011d300;
            local_1fec.B = DAT_0011d300_4;
            local_1fec.C = (uint)uRam000000000011d308;
            local_1fec.D = uRam000000000011d308._4_4_;
            local_1fec.Nl = _DAT_0011d310;
            local_1fec.Nh = DAT_0011d310_4;
            local_1fec.data[0] = (uint)uRam000000000011d318;
            local_1fec.data[1] = uRam000000000011d318._4_4_;
            local_1fec.data[0xe] = (uint)DAT_0011d350;
            local_1fec.data[0xf] = DAT_0011d350._4_4_;
            local_1fec.num = DAT_0011d358;
            local_2048.data[0xe] = (uint)DAT_0011d350;
            local_2048.data[0xf] = DAT_0011d350._4_4_;
            local_2048.num = DAT_0011d358;
            local_1fec.data[2] = _DAT_0011d320;
            local_1fec.data[3] = DAT_0011d320_4;
            local_1fec.data[4] = (uint)uRam000000000011d328;
            local_1fec.data[5] = uRam000000000011d328._4_4_;
            local_1fec.data[6] = _DAT_0011d330;
            local_1fec.data[7] = DAT_0011d330_4;
            local_1fec.data[8] = (uint)uRam000000000011d338;
            local_1fec.data[9] = uRam000000000011d338._4_4_;
            local_1fec.data[10] = _DAT_0011d340;
            local_1fec.data[0xb] = DAT_0011d340_4;
            local_1fec.data[0xc] = (uint)uRam000000000011d348;
            local_1fec.data[0xd] = uRam000000000011d348._4_4_;
            local_2048.A = _DAT_0011d300;
            local_2048.B = DAT_0011d300_4;
            local_2048.C = (uint)uRam000000000011d308;
            local_2048.D = uRam000000000011d308._4_4_;
            local_2048.Nl = _DAT_0011d310;
            local_2048.Nh = DAT_0011d310_4;
            local_2048.data[0] = (uint)uRam000000000011d318;
            local_2048.data[1] = uRam000000000011d318._4_4_;
            local_2048.data[2] = _DAT_0011d320;
            local_2048.data[3] = DAT_0011d320_4;
            local_2048.data[4] = (uint)uRam000000000011d328;
            local_2048.data[5] = uRam000000000011d328._4_4_;
            local_2048.data[6] = _DAT_0011d330;
            local_2048.data[7] = DAT_0011d330_4;
            local_2048.data[8] = (uint)uRam000000000011d338;
            local_2048.data[9] = uRam000000000011d338._4_4_;
            local_2048.data[10] = _DAT_0011d340;
            local_2048.data[0xb] = DAT_0011d340_4;
            local_2048.data[0xc] = (uint)uRam000000000011d348;
            local_2048.data[0xd] = uRam000000000011d348._4_4_;
            if (DAT_0011d861 != '\0') {
              pbVar17 = __nptr + len_00;
              local_2100 = (byte *)0x0;
              pbVar1 = pbVar9 + len_01;
              uVar14 = (len_00 + len_01) * 3 + 2;
              local_2118 = 4000;
              iVar8 = 0;
              pbVar16 = pbVar9;
              pbVar13 = local_fe8;
              pbVar19 = __nptr;
LAB_00109a20:
              if (local_2118 < uVar14) goto LAB_00109b04;
LAB_00109a30:
              pbVar11 = pbVar13;
              if (pbVar19 < pbVar17) goto LAB_00109b4f;
LAB_00109a3b:
              pbVar13 = pbVar11;
              if (pbVar16 < pbVar1) {
                len = 0;
                sVar10 = local_2118;
                goto LAB_00109b7e;
              }
              local_20f0 = 0;
              len = 0;
LAB_00109a51:
              local_20f8 = pbVar13 + len;
              do {
                if (pbVar19 < pbVar17) {
                  sVar10 = strlen((char *)pbVar19);
                  pbVar19 = pbVar19 + sVar10 + 1;
                  if (pbVar16 < pbVar1) goto LAB_00109a70;
                }
                else {
                  if (pbVar1 <= pbVar16) {
LAB_00109c1a:
                    MD5_Update(&local_2048,pbVar13,len);
                    MD5_Final(local_2068,&local_2048);
                    MD5_Update(&local_1fec,local_20f8,local_20f0);
                    MD5_Final(local_2058,&local_1fec);
                    iVar7 = memcmp(local_2068,local_2058,0x10);
                    if ((iVar7 != 0) || (iVar7 = iVar8, iVar8 != 0)) goto LAB_00109cac;
                    goto LAB_00109d87;
                  }
LAB_00109a70:
                  sVar10 = strlen((char *)pbVar16);
                  pbVar16 = pbVar16 + sVar10 + 1;
                }
                while( true ) {
                  if ((pbVar17 <= pbVar19) && (pbVar1 <= pbVar16)) goto LAB_00109c1a;
                  MD5_Update(&local_2048,pbVar13,len);
                  MD5_Update(&local_1fec,local_20f8,local_20f0);
                  if (iVar8 != 0) goto LAB_00109a20;
                  uVar18 = local_20f0;
                  if (len <= local_20f0) {
                    uVar18 = len;
                  }
                  iVar8 = memcmp(pbVar13,local_20f8,uVar18);
                  if (iVar8 != 0) goto LAB_00109a20;
                  iVar8 = (uint)(len >= local_20f0 && len != local_20f0) - (uint)(len < local_20f0);
                  if (uVar14 <= local_2118) goto LAB_00109a30;
LAB_00109b04:
                  local_2118 = local_2118 * 3 >> 1;
                  if (local_2118 <= uVar14) {
                    local_2118 = uVar14;
                  }
                  free(local_2100);
                  pbVar11 = malloc(local_2118);
                  local_2100 = pbVar11;
                  if (pbVar11 == (byte *)0x0) {
                    local_2118 = 4000;
                    pbVar13 = local_fe8;
                    goto LAB_00109a30;
                  }
                  pbVar13 = pbVar11;
                  if (pbVar17 <= pbVar19) goto LAB_00109a3b;
LAB_00109b4f:
                  lVar12 = FUN_00107800(pbVar13,pbVar19,local_2118);
                  len = lVar12 + 1;
                  if (pbVar16 < pbVar1) {
                    if (local_2118 < len) {
                      pbVar11 = (byte *)0x0;
                      sVar10 = 0;
                    }
                    else {
                      pbVar11 = pbVar13 + len;
                      sVar10 = local_2118 - len;
                    }
LAB_00109b7e:
                    lVar12 = FUN_00107800(pbVar11,pbVar16,sVar10);
                    local_20f0 = lVar12 + 1;
                    uVar18 = local_20f0 + len;
                  }
                  else {
                    local_20f0 = 0;
                    uVar18 = len;
                  }
                  if ((len <= local_2118) && (uVar18 <= local_2118)) goto LAB_00109a51;
                  if (uVar18 < 0x5555555555555555) {
                    uVar18 = uVar18 * 3 >> 1;
                  }
                  free(local_2100);
                  pbVar13 = (byte *)FUN_00112ab0(uVar18);
                  local_2118 = uVar18;
                  local_2100 = pbVar13;
                  if (pbVar17 <= pbVar19) break;
                  strxfrm((char *)pbVar13,(char *)pbVar19,len);
                  if (pbVar16 < pbVar1) goto LAB_00109e3c;
                  local_20f8 = pbVar13 + len;
                  sVar10 = strlen((char *)pbVar19);
                  pbVar19 = pbVar19 + sVar10 + 1;
                }
                if (pbVar1 <= pbVar16) {
                  local_20f8 = pbVar13 + len;
                  goto LAB_00109c1a;
                }
LAB_00109e3c:
                local_20f8 = pbVar13 + len;
                strxfrm((char *)local_20f8,(char *)pbVar16,local_20f0);
              } while( true );
            }
            MD5_Update(&local_2048,__nptr,len_00);
            MD5_Final(local_2068,&local_2048);
            MD5_Update(&local_1fec,pbVar9,len_01);
            MD5_Final(local_2058,&local_1fec);
            iVar7 = memcmp(local_2068,local_2058,0x10);
            if (iVar7 == 0) {
              local_2100 = (byte *)0x0;
              len = len_00;
              pbVar13 = __nptr;
              local_20f8 = pbVar9;
              local_20f0 = len_01;
LAB_00109d87:
              uVar14 = local_20f0;
              if (len <= local_20f0) {
                uVar14 = len;
              }
              iVar7 = memcmp(pbVar13,local_20f8,uVar14);
              if (iVar7 == 0) {
                iVar7 = (uint)(local_20f0 < len) - (uint)(len < local_20f0);
              }
            }
            else {
              local_2100 = (byte *)0x0;
            }
LAB_00109cac:
            free(local_2100);
            lVar21 = in_ST0;
            lVar20 = in_ST1;
          }
        }
        else {
          iVar7 = FUN_00107030(__nptr,0);
          iVar8 = FUN_00107030(pbVar9);
          iVar7 = iVar7 - iVar8;
          lVar21 = in_ST0;
          lVar20 = in_ST1;
        }
        goto LAB_001092a0;
      }
      bVar6 = *__nptr;
      cVar5 = (&DAT_0011d740)[bVar6];
      pbVar17 = __nptr;
      while (cVar5 != '\0') {
        bVar6 = pbVar17[1];
        pbVar17 = pbVar17 + 1;
        cVar5 = (&DAT_0011d740)[bVar6];
      }
      bVar2 = *pbVar9;
      cVar5 = (&DAT_0011d740)[bVar2];
      pbVar16 = pbVar9;
      while (cVar5 != '\0') {
        bVar2 = pbVar16[1];
        pbVar16 = pbVar16 + 1;
        cVar5 = (&DAT_0011d740)[bVar2];
      }
      local_2078._0_8_ = pbVar17 + (bVar6 == 0x2d);
      cVar5 = FUN_00106f70();
      iVar8 = 0;
      if ('0' < cVar5) {
        iVar8 = (int)(char)(&DAT_00115360)[*local_2078._0_8_];
        if (bVar6 == 0x2d) {
          iVar8 = -(int)(char)(&DAT_00115360)[*local_2078._0_8_];
        }
      }
      local_2078 = (longdouble)CONCAT28(local_2078._8_2_,pbVar16 + (bVar2 == 0x2d));
      cVar5 = FUN_00106f70(&local_2078);
      iVar7 = iVar8;
      if ('0' < cVar5) {
        iVar7 = iVar8 - (char)(&DAT_00115360)[*local_2078._0_8_];
        if (bVar2 == 0x2d) {
          iVar7 = (char)(&DAT_00115360)[*local_2078._0_8_] + iVar8;
        }
      }
      lVar21 = in_ST0;
      lVar20 = in_ST1;
      if (iVar7 != 0) goto LAB_001092a0;
      iVar7 = FUN_00111d90(pbVar17,pbVar16,(int)DAT_0011d868,DAT_0011d864);
      if (lVar3 == 0 && lVar4 == 0) goto LAB_001092ac;
    }
    else {
      cVar5 = (&DAT_0011d740)[*__nptr];
      pbVar17 = __nptr;
      while (cVar5 != '\0') {
        pbVar16 = pbVar17 + 1;
        pbVar17 = pbVar17 + 1;
        cVar5 = (&DAT_0011d740)[*pbVar16];
      }
      cVar5 = (&DAT_0011d740)[*pbVar9];
      pbVar16 = pbVar9;
      while (cVar5 != '\0') {
        pbVar19 = pbVar16 + 1;
        pbVar16 = pbVar16 + 1;
        cVar5 = (&DAT_0011d740)[*pbVar19];
      }
      iVar7 = FUN_00111d90(pbVar17,pbVar16,(int)DAT_0011d868,DAT_0011d864);
      lVar21 = in_ST0;
      lVar20 = in_ST1;
LAB_001092a0:
      in_ST0 = lVar21;
      in_ST1 = lVar20;
      if (lVar3 == 0 && lVar4 == 0) {
LAB_001092ac:
        __nptr[len_00] = local_2132;
        pbVar9[len_01] = local_2131;
        goto joined_r0x001092c1;
      }
    }
    free(local_2110);
    goto joined_r0x001092c1;
  }
  if (lVar4 != 0) {
    if (lVar3 == 0) {
      do {
        if ((pbVar17 <= __nptr) || (*(char *)(lVar4 + (ulong)*__nptr) == '\0')) {
          if (pbVar9 <= pbVar16) goto LAB_0010a018;
          while (*(char *)(lVar4 + (ulong)*pbVar16) != '\0') {
            pbVar16 = pbVar16 + 1;
            if (pbVar9 == pbVar16) goto LAB_0010a018;
          }
          if ((pbVar17 <= __nptr) || (pbVar9 <= pbVar16)) goto LAB_0010a018;
          iVar7 = (uint)*__nptr - (uint)*pbVar16;
          if (iVar7 != 0) goto LAB_001094dd;
          pbVar16 = pbVar16 + 1;
        }
        __nptr = __nptr + 1;
      } while( true );
    }
    do {
      if ((pbVar17 <= __nptr) || (*(char *)(lVar4 + (ulong)*__nptr) == '\0')) {
        if (pbVar9 <= pbVar16) goto LAB_00109f28;
        while (*(char *)(lVar4 + (ulong)*pbVar16) != '\0') {
          pbVar16 = pbVar16 + 1;
          if (pbVar9 == pbVar16) goto LAB_00109f28;
        }
        if ((pbVar17 <= __nptr) || (pbVar9 <= pbVar16)) goto LAB_00109f28;
        iVar7 = (uint)*(byte *)(lVar3 + (ulong)*__nptr) - (uint)*(byte *)(lVar3 + (ulong)*pbVar16);
        if (iVar7 != 0) goto LAB_001094dd;
        pbVar16 = pbVar16 + 1;
      }
      __nptr = __nptr + 1;
    } while( true );
  }
  if (len_00 == 0) {
    iVar7 = -(uint)(len_01 != 0);
    goto joined_r0x001092c1;
  }
  if (len_01 == 0) {
LAB_00109f18:
    iVar7 = 1;
    goto LAB_001094dd;
  }
  if (lVar3 == 0) {
    uVar14 = len_01;
    if (len_00 <= len_01) {
      uVar14 = len_00;
    }
    iVar7 = memcmp(__nptr,pbVar16,uVar14);
    if (iVar7 == 0) goto LAB_00109f09;
    goto LAB_001094dd;
  }
  while ((__nptr < pbVar17 && (pbVar16 < pbVar9))) {
    bVar6 = *__nptr;
    __nptr = __nptr + 1;
    iVar7 = (uint)*(byte *)(lVar3 + (ulong)bVar6) - (uint)*(byte *)(lVar3 + (ulong)*pbVar16);
    pbVar16 = pbVar16 + 1;
    if (iVar7 != 0) goto LAB_001094dd;
  }
LAB_00109f09:
  if (len_00 < len_01) {
    iVar7 = -1;
    goto LAB_001094dd;
  }
  if (len_00 != len_01) goto LAB_00109f18;
  goto LAB_001092c7;
LAB_0010a018:
  iVar7 = (uint)(__nptr < pbVar17) - (uint)(pbVar16 < pbVar9);
  goto joined_r0x001092c1;
LAB_00109f28:
  iVar7 = (uint)(__nptr < pbVar17) - (uint)(pbVar16 < pbVar9);
joined_r0x001092c1:
  if (iVar7 != 0) {
LAB_001094dd:
    if (*(char *)((long)plVar15 + 0x37) != '\0') {
      iVar7 = -iVar7;
    }
    goto LAB_001094e7;
  }
LAB_001092c7:
  plVar15 = (long *)plVar15[8];
  if (plVar15 == (long *)0x0) {
    iVar7 = 0;
LAB_001094e7:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return iVar7;
  }
  __nptr = (byte *)*param_1;
  lVar3 = param_1[1];
  if (plVar15[2] == -1) {
    pbVar17 = __nptr + lVar3 + -1;
    pbVar9 = (byte *)(*param_2 + -1 + param_2[1]);
  }
  else {
    pbVar17 = (byte *)FUN_00107ca0(__nptr,lVar3,plVar15);
    pbVar9 = (byte *)FUN_00107ca0(*param_2,param_2[1],plVar15);
  }
  if (*plVar15 == -1) {
    pbVar16 = (byte *)*param_2;
    if ((char)plVar15[6] != '\0') {
      if (__nptr < pbVar17) {
        do {
          if ((&DAT_0011d740)[*__nptr] == '\0') break;
          __nptr = __nptr + 1;
        } while (pbVar17 != __nptr);
      }
      if (pbVar16 < pbVar9) {
        do {
          if ((&DAT_0011d740)[*pbVar16] == '\0') break;
          pbVar16 = pbVar16 + 1;
        } while (pbVar9 != pbVar16);
      }
    }
  }
  else {
    __nptr = (byte *)FUN_00107dd0(__nptr,lVar3,plVar15);
    pbVar16 = (byte *)FUN_00107dd0(*param_2,param_2[1],plVar15);
  }
  goto LAB_001091a0;
}




// Function: sequential_sort @ 0xa180

void sequential_sort(undefined8 *param_1,ulong param_2,undefined8 *param_3,char param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  int iVar9;
  long lVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  undefined8 *puVar13;
  ulong uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  long local_48;
  
  if (param_2 == 2) {
    iVar9 = FUN_0010a0b0(param_1 + -4,param_1 + -8);
    if (param_4 != '\0') {
      puVar13 = param_1 + (long)(int)((0 < iVar9) - 2) * 4;
      param_1 = param_1 + (long)(int)~(uint)(0 < iVar9) * 4;
      uVar1 = param_1[1];
      uVar2 = param_1[2];
      uVar3 = param_1[3];
      param_3[-4] = *param_1;
      param_3[-3] = uVar1;
      uVar1 = *puVar13;
      uVar4 = puVar13[1];
      param_3[-2] = uVar2;
      param_3[-1] = uVar3;
      uVar2 = puVar13[2];
      uVar3 = puVar13[3];
      param_3[-8] = uVar1;
      param_3[-7] = uVar4;
      param_3[-6] = uVar2;
      param_3[-5] = uVar3;
      return;
    }
    if (0 < iVar9) {
      uVar1 = param_1[-4];
      uVar2 = param_1[-3];
      uVar3 = param_1[-2];
      uVar4 = param_1[-1];
      uVar5 = param_1[-8];
      uVar6 = param_1[-7];
      uVar7 = param_1[-6];
      uVar8 = param_1[-5];
      param_3[-4] = uVar1;
      param_3[-3] = uVar2;
      param_3[-2] = uVar3;
      param_3[-1] = uVar4;
      param_1[-4] = uVar5;
      param_1[-3] = uVar6;
      param_1[-2] = uVar7;
      param_1[-1] = uVar8;
      param_1[-8] = uVar1;
      param_1[-7] = uVar2;
      param_1[-6] = uVar3;
      param_1[-5] = uVar4;
    }
  }
  else {
    uVar14 = param_2 >> 1;
    local_48 = param_2 - uVar14;
    puVar13 = param_1 + uVar14 * -4;
    if (param_4 == '\0') {
      sequential_sort(puVar13,local_48,param_3,0);
      puVar11 = param_1;
      puVar16 = puVar13;
      if (param_2 < 4) {
        uVar1 = param_1[-3];
        uVar2 = param_1[-2];
        uVar3 = param_1[-1];
        param_3[-4] = param_1[-4];
        param_3[-3] = uVar1;
        param_3[-2] = uVar2;
        param_3[-1] = uVar3;
        param_1 = param_3;
      }
      else {
        sequential_sort(param_1,uVar14,param_3,1);
        param_1 = param_3;
      }
    }
    else {
      puVar16 = param_3 + uVar14 * -4;
      sequential_sort(puVar13,local_48,puVar16,1);
      puVar11 = param_3;
      if (3 < param_2) {
        sequential_sort(param_1,uVar14,param_3,0);
      }
    }
    puVar13 = puVar16 + -4;
    puVar15 = param_1 + -4;
    while( true ) {
      while( true ) {
        puVar12 = puVar13;
        puVar13 = puVar11 + -4;
        iVar9 = FUN_0010a0b0(puVar15,puVar12);
        if (iVar9 < 1) break;
        uVar1 = puVar16[-3];
        local_48 = local_48 + -1;
        *puVar13 = puVar16[-4];
        puVar11[-3] = uVar1;
        uVar1 = puVar16[-1];
        puVar11[-2] = puVar16[-2];
        puVar11[-1] = uVar1;
        if (local_48 == 0) {
          lVar10 = -0x20;
          do {
            uVar1 = ((undefined8 *)((long)param_1 + lVar10))[1];
            puVar16 = (undefined8 *)((long)param_1 + lVar10 + 0x10);
            uVar2 = *puVar16;
            uVar3 = puVar16[1];
            *(undefined8 *)((long)puVar13 + lVar10) = *(undefined8 *)((long)param_1 + lVar10);
            ((undefined8 *)((long)puVar13 + lVar10))[1] = uVar1;
            puVar16 = (undefined8 *)((long)puVar11 + lVar10 + -0x10);
            *puVar16 = uVar2;
            puVar16[1] = uVar3;
            lVar10 = lVar10 + -0x20;
            uVar14 = uVar14 - 1;
          } while (uVar14 != 0);
          return;
        }
        puVar11 = puVar13;
        puVar13 = puVar12 + -4;
        puVar16 = puVar12;
      }
      uVar1 = param_1[-3];
      uVar2 = param_1[-2];
      uVar3 = param_1[-1];
      *puVar13 = param_1[-4];
      puVar11[-3] = uVar1;
      puVar11[-2] = uVar2;
      puVar11[-1] = uVar3;
      uVar14 = uVar14 - 1;
      if (uVar14 == 0) break;
      puVar11 = puVar13;
      puVar13 = puVar12;
      param_1 = puVar15;
      puVar15 = puVar15 + -4;
    }
  }
  return;
}




// Function: write_unique @ 0xa3c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void write_unique(long *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  if (DAT_0011d419 != '\0') {
    if (DAT_0011d840 != 0) {
      iVar1 = FUN_0010a0b0(param_1,&DAT_0011d840);
      if (iVar1 == 0) {
        return;
      }
    }
    DAT_0011d840 = *param_1;
    lRam000000000011d848 = param_1[1];
    _DAT_0011d850 = param_1[2];
    lRam000000000011d858 = param_1[3];
  }
  FUN_001082d0(param_1,param_2,param_3);
  return;
}




// Function: mergefps @ 0xa430

void mergefps(long param_1,ulong param_2,ulong param_3,undefined8 param_4,undefined8 param_5,
                 void *param_6)

{
  size_t __size;
  long *plVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  long lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  long lVar14;
  char cVar15;
  int iVar16;
  void *__ptr;
  void *__ptr_00;
  void *__ptr_01;
  ulong *__ptr_02;
  void *pvVar17;
  ulong *puVar18;
  long lVar19;
  ulong *puVar20;
  ulong uVar21;
  undefined8 *puVar22;
  ulong uVar23;
  ulong uVar24;
  ulong uVar25;
  long in_FS_OFFSET;
  ulong local_d0;
  ulong local_c8;
  void **local_b0;
  ulong local_88;
  ulong local_80;
  void *local_68;
  size_t local_60;
  long local_58;
  long local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = (void *)FUN_00112bf0(param_3,0x38);
  __ptr_00 = (void *)FUN_00112bf0(param_3,8);
  __ptr_01 = (void *)FUN_00112bf0(param_3,8);
  __ptr_02 = (ulong *)FUN_00112bf0(param_3,8);
  lVar14 = DAT_0011d410;
  local_68 = (void *)0x0;
  if (param_3 != 0) {
    uVar24 = 0;
    local_c8 = param_3;
    local_88 = param_2;
    do {
      while( true ) {
        uVar21 = DAT_0011d438 / local_c8;
        if (DAT_0011d438 / local_c8 < DAT_0011d030) {
          uVar21 = DAT_0011d030;
        }
        plVar1 = (long *)((long)__ptr + uVar24 * 0x38);
        while( true ) {
          __size = (uVar21 & 0xffffffffffffffe0) + 0x20;
          pvVar17 = malloc(__size);
          *plVar1 = (long)pvVar17;
          if (pvVar17 != (void *)0x0) break;
          uVar21 = __size >> 1;
          if (__size < 0x43) {
                    /* WARNING: Subroutine does not return */
            FUN_00112fb0();
          }
        }
        plVar1[3] = __size;
        lVar19 = uVar24 * 8;
        plVar1[5] = 0x20;
        puVar22 = (undefined8 *)(uVar24 * 0x10 + param_1);
        uVar3 = *(undefined8 *)((long)param_6 + lVar19);
        plVar1[2] = 0;
        plVar1[4] = 0;
        uVar4 = *puVar22;
        plVar1[1] = 0;
        *(undefined1 *)(plVar1 + 6) = 0;
        cVar15 = FUN_00107ef0(plVar1,uVar3,uVar4);
        if (cVar15 != '\0') break;
        FUN_00107660(*(undefined8 *)((long)param_6 + lVar19),*puVar22);
        if (uVar24 < local_88) {
          local_88 = local_88 - 1;
          FUN_00108f90(*puVar22);
        }
        free((void *)*plVar1);
        uVar21 = local_c8 - 1;
        if (uVar21 <= uVar24) {
          local_c8 = uVar21;
          if (uVar21 == 0) goto LAB_0010aa9e;
          goto LAB_0010a5e2;
        }
        lVar5 = local_c8 * 8;
        do {
          puVar22 = (undefined8 *)(param_1 + 0x10 + lVar19 * 2);
          uVar4 = puVar22[1];
          uVar3 = *(undefined8 *)((long)param_6 + lVar19 + 8);
          puVar2 = (undefined8 *)(param_1 + lVar19 * 2);
          *puVar2 = *puVar22;
          puVar2[1] = uVar4;
          *(undefined8 *)((long)param_6 + lVar19) = uVar3;
          lVar19 = lVar19 + 8;
          local_c8 = uVar21;
        } while (lVar5 + -8 != lVar19);
      }
      lVar19 = plVar1[3];
      lVar5 = *plVar1;
      *(long *)((long)__ptr_00 + uVar24 * 8) = lVar19 + lVar5 + -0x20;
      *(long *)((long)__ptr_01 + uVar24 * 8) = lVar19 + lVar5 + plVar1[2] * -0x20;
      uVar24 = uVar24 + 1;
    } while (uVar24 < local_c8);
LAB_0010a5e2:
    uVar24 = 0;
    do {
      __ptr_02[uVar24] = uVar24;
      uVar24 = uVar24 + 1;
    } while (uVar24 != local_c8);
    if (local_c8 != 1) {
      uVar24 = 1;
      do {
        while( true ) {
          puVar18 = __ptr_02 + uVar24;
          puVar20 = __ptr_02 + (uVar24 - 1);
          iVar16 = FUN_0010a0b0(*(undefined8 *)((long)__ptr_00 + *puVar20 * 8),
                                *(undefined8 *)((long)__ptr_00 + *puVar18 * 8));
          if (iVar16 < 1) break;
          uVar21 = *puVar20;
          uVar24 = 1;
          *puVar20 = *puVar18;
          *puVar18 = uVar21;
        }
        uVar24 = uVar24 + 1;
      } while (uVar24 < local_c8);
    }
    local_80 = 0;
    local_b0 = (void **)0x0;
LAB_0010a680:
    plVar1 = *(long **)((long)__ptr_00 + *__ptr_02 * 8);
    if (DAT_0011d419 == '\0') {
      FUN_001082d0(plVar1,param_4,param_5);
    }
    else {
      if (local_b0 != (void **)0x0) {
        iVar16 = FUN_0010a0b0(local_b0,plVar1);
        if (iVar16 == 0) goto LAB_0010a6b6;
        FUN_001082d0(&local_68,param_4,param_5);
      }
      uVar24 = plVar1[1];
      if (local_80 < uVar24) {
        do {
          uVar21 = uVar24;
          if (local_80 == 0) break;
          local_80 = local_80 * 2;
          uVar21 = local_80;
        } while (local_80 < uVar24);
        free(local_68);
        local_68 = (void *)FUN_00112ab0(uVar21);
        uVar24 = plVar1[1];
        local_80 = uVar21;
      }
      local_60 = uVar24;
      pvVar17 = memcpy(local_68,(void *)*plVar1,uVar24);
      local_b0 = &local_68;
      if (lVar14 != 0) {
        local_58 = (plVar1[2] - *plVar1) + (long)pvVar17;
        local_50 = (long)pvVar17 + (plVar1[3] - *plVar1);
      }
    }
LAB_0010a6b6:
    local_d0 = *__ptr_02;
    if (*(long **)((long)__ptr_01 + local_d0 * 8) < plVar1) {
      *(long **)((long)__ptr_00 + local_d0 * 8) = plVar1 + -4;
      if (local_c8 == 1) goto LAB_0010a81c;
LAB_0010a761:
      uVar24 = 1;
      uVar21 = 1;
      uVar25 = local_c8;
      do {
        while( true ) {
          uVar23 = uVar21;
          iVar16 = FUN_0010a0b0(*(undefined8 *)((long)__ptr_00 + local_d0 * 8),
                                *(undefined8 *)((long)__ptr_00 + __ptr_02[uVar23] * 8));
          if ((-1 < iVar16) && ((iVar16 != 0 || (__ptr_02[uVar23] <= local_d0)))) break;
          uVar21 = uVar24 + uVar23 >> 1;
          uVar25 = uVar23;
          if (uVar23 <= uVar24) goto LAB_0010a7c9;
        }
        uVar24 = uVar23 + 1;
        uVar21 = uVar24 + uVar25 >> 1;
      } while (uVar24 < uVar25);
LAB_0010a7c9:
      if (uVar24 - 1 != 0) {
        puVar20 = __ptr_02;
        do {
          puVar18 = puVar20 + 1;
          *puVar20 = puVar20[1];
          puVar20 = puVar18;
        } while (puVar18 != __ptr_02 + (uVar24 - 1));
      }
      __ptr_02[uVar24 - 1] = local_d0;
      goto LAB_0010a680;
    }
    cVar15 = FUN_00107ef0((void *)((long)__ptr + local_d0 * 0x38),
                          *(undefined8 *)((long)param_6 + local_d0 * 8),
                          *(undefined8 *)(param_1 + local_d0 * 0x10));
    if (cVar15 != '\0') {
      local_d0 = *__ptr_02;
      plVar1 = (long *)((long)__ptr + local_d0 * 0x38);
      lVar19 = plVar1[3];
      lVar5 = *plVar1;
      lVar6 = plVar1[2];
      *(long *)((long)__ptr_00 + local_d0 * 8) = lVar19 + lVar5 + -0x20;
      *(long *)((long)__ptr_01 + local_d0 * 8) = lVar19 + lVar5 + lVar6 * -0x20;
      if (local_c8 != 1) goto LAB_0010a761;
LAB_0010a81c:
      *__ptr_02 = local_d0;
      goto LAB_0010a680;
    }
    uVar24 = *__ptr_02;
    puVar20 = __ptr_02 + 1;
    if (local_c8 != 1) {
      do {
        if (uVar24 < *puVar20) {
          *puVar20 = *puVar20 - 1;
          uVar24 = *__ptr_02;
        }
        puVar20 = puVar20 + 1;
      } while (__ptr_02 + local_c8 != puVar20);
    }
    uVar21 = local_c8 - 1;
    FUN_00107660(*(undefined8 *)((long)param_6 + uVar24 * 8),
                 *(undefined8 *)(param_1 + uVar24 * 0x10));
    uVar24 = *__ptr_02;
    if (uVar24 < local_88) {
      local_88 = local_88 - 1;
      FUN_00108f90(*(undefined8 *)(param_1 + uVar24 * 0x10));
      uVar24 = *__ptr_02;
    }
    free(*(void **)((long)__ptr + uVar24 * 0x38));
    uVar24 = *__ptr_02;
    if (uVar24 < uVar21) {
      lVar19 = uVar24 * 8;
      puVar22 = (undefined8 *)((long)__ptr + uVar24 * 0x38);
      do {
        puVar2 = (undefined8 *)(param_1 + 0x10 + lVar19 * 2);
        uVar4 = *puVar2;
        uVar7 = puVar2[1];
        uVar8 = puVar22[7];
        uVar9 = puVar22[8];
        uVar10 = puVar22[9];
        uVar11 = puVar22[10];
        *(undefined8 *)((long)param_6 + lVar19) = *(undefined8 *)((long)param_6 + lVar19 + 8);
        uVar3 = puVar22[0xd];
        uVar12 = puVar22[0xb];
        uVar13 = puVar22[0xc];
        puVar2 = (undefined8 *)(param_1 + lVar19 * 2);
        *puVar2 = uVar4;
        puVar2[1] = uVar7;
        puVar22[6] = uVar3;
        uVar3 = *(undefined8 *)((long)__ptr_00 + lVar19 + 8);
        *puVar22 = uVar8;
        puVar22[1] = uVar9;
        *(undefined8 *)((long)__ptr_00 + lVar19) = uVar3;
        uVar3 = *(undefined8 *)((long)__ptr_01 + lVar19 + 8);
        puVar22[2] = uVar10;
        puVar22[3] = uVar11;
        *(undefined8 *)((long)__ptr_01 + lVar19) = uVar3;
        lVar19 = lVar19 + 8;
        puVar22[4] = uVar12;
        puVar22[5] = uVar13;
        puVar22 = puVar22 + 7;
      } while (local_c8 * 8 + -8 != lVar19);
    }
    if (uVar21 != 0) {
      lVar19 = local_c8 - 1;
      puVar20 = __ptr_02;
      do {
        puVar18 = puVar20 + 1;
        *puVar20 = puVar20[1];
        puVar20 = puVar18;
        local_c8 = uVar21;
      } while (__ptr_02 + lVar19 != puVar18);
      goto LAB_0010a680;
    }
    if ((local_b0 != (void **)0x0) && (DAT_0011d419 != '\0')) {
      FUN_001082d0(&local_68,param_4,param_5);
      free(local_68);
    }
  }
LAB_0010aa9e:
  FUN_00107660(param_4,param_5);
  free(param_6);
  free(__ptr);
  free(__ptr_02);
  free(__ptr_01);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  free(__ptr_00);
  return;
}




// Function: mergefiles @ 0xabe0

ulong mergefiles(long param_1,undefined8 param_2,ulong param_3,undefined8 param_4,
                  undefined8 param_5)

{
  undefined8 uVar1;
  ulong uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00108980(param_1,param_3,&local_48);
  if ((uVar2 < param_3) && (uVar2 < 2)) {
    uVar1 = *(undefined8 *)(param_1 + uVar2 * 0x10);
    uVar3 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
    FUN_00107480(uVar3,uVar1);
  }
  FUN_0010a430(param_1,param_2,uVar2,param_4,param_5,local_48);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: merge @ 0xac90

void merge(long *param_1,ulong param_2,ulong param_3,char *param_4)

{
  char *__s1;
  int iVar1;
  int iVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  int *piVar6;
  undefined8 uVar7;
  ulong uVar8;
  ulong uVar9;
  long lVar10;
  long *plVar11;
  ulong uVar12;
  ulong uVar13;
  long *__src;
  long in_FS_OFFSET;
  ulong local_110;
  ulong local_108;
  long local_e8;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  uVar5 = (ulong)DAT_0011d024;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_108 = param_3;
  if (uVar5 < param_3) {
    do {
      uVar4 = 0;
      lVar10 = 0;
      if (local_108 < uVar5) {
        uVar8 = 0;
        uVar9 = uVar5;
        uVar12 = local_108;
        plVar11 = param_1;
        __src = param_1;
      }
      else {
        do {
          uVar13 = uVar4;
          lVar3 = FUN_00108c20(&local_e0,0);
          uVar4 = (ulong)DAT_0011d024;
          uVar5 = param_2;
          if (uVar4 <= param_2) {
            uVar5 = uVar4;
          }
          uVar4 = FUN_0010abe0(param_1 + lVar10 * 2,uVar5,uVar4,local_e0);
          uVar9 = (ulong)DAT_0011d024;
          uVar5 = param_2;
          if (uVar4 <= param_2) {
            uVar5 = uVar4;
          }
          lVar10 = lVar10 + uVar4;
          uVar4 = uVar13 + 1;
          param_1[uVar13 * 2] = lVar3 + 0xd;
          param_1[uVar13 * 2 + 1] = lVar3;
          param_2 = param_2 - uVar5;
        } while (uVar9 <= local_108 - lVar10);
        uVar8 = uVar4 % uVar9;
        uVar5 = uVar9 - uVar8;
        uVar12 = local_108 - lVar10;
        plVar11 = param_1 + uVar13 * 2 + 2;
        __src = param_1 + lVar10 * 2;
      }
      if (uVar5 < uVar12) {
        uVar9 = (uVar12 + 1 + uVar8) - uVar9;
        lVar3 = FUN_00108c20(&local_e0,0);
        uVar5 = param_2;
        if (uVar9 <= param_2) {
          uVar5 = uVar9;
        }
        uVar9 = FUN_0010abe0(__src,uVar5,uVar9,local_e0);
        uVar5 = param_2;
        if (uVar9 <= param_2) {
          uVar5 = uVar9;
        }
        uVar4 = uVar4 + 1;
        lVar10 = lVar10 + uVar9;
        *plVar11 = lVar3 + 0xd;
        plVar11[1] = lVar3;
        param_2 = param_2 - uVar5;
        __src = param_1 + lVar10 * 2;
        plVar11 = param_1 + uVar4 * 2;
      }
      param_2 = param_2 + uVar4;
      memmove(plVar11,__src,(local_108 - lVar10) * 0x10);
      uVar5 = (ulong)DAT_0011d024;
      local_108 = local_108 + (uVar4 - lVar10);
    } while (uVar5 < local_108);
  }
  if (param_2 < local_108) {
    lVar10 = 0;
    plVar11 = param_1 + param_2 * 2;
    uVar5 = param_2;
    do {
      while( true ) {
        __s1 = (char *)*plVar11;
        iVar1 = strcmp(__s1,"-");
        if (((param_4 == (char *)0x0) || (iVar2 = strcmp(param_4,__s1), iVar2 != 0)) || (iVar1 == 0)
           ) break;
LAB_0010ae9e:
        lVar3 = lVar10 + 0xd;
        if (lVar10 == 0) {
          lVar10 = FUN_00108c20(&local_e0,0);
          lVar3 = lVar10 + 0xd;
          FUN_0010abe0(plVar11,0,1,local_e0,lVar3);
        }
        *plVar11 = lVar3;
        uVar5 = uVar5 + 1;
        plVar11[1] = lVar10;
        plVar11 = plVar11 + 2;
        if (uVar5 == local_108) goto LAB_0010aecf;
      }
      if (DAT_0011d2f0 == 0) {
        iVar2 = fstat(1,(stat *)&DAT_0011d260);
        if (iVar2 != 0) {
          piVar6 = __errno_location();
          DAT_0011d2f0 = *piVar6;
          goto LAB_0010ae64;
        }
        DAT_0011d2f0 = -1;
      }
      else {
LAB_0010ae64:
        if (-1 < DAT_0011d2f0) break;
      }
      if (iVar1 == 0) {
        iVar1 = fstat(0,&local_d8);
        if ((iVar1 == 0) && (local_d8.st_ino == DAT_0011d268)) goto LAB_0010ae8c;
      }
      else {
        iVar1 = stat((char *)*plVar11,&local_d8);
        if ((iVar1 == 0) && (local_d8.st_ino == DAT_0011d268)) {
LAB_0010ae8c:
          if (local_d8.st_dev == DAT_0011d260) goto LAB_0010ae9e;
        }
      }
      uVar5 = uVar5 + 1;
      plVar11 = plVar11 + 2;
    } while (uVar5 != local_108);
  }
LAB_0010aecf:
  local_110 = param_2;
  do {
    uVar5 = FUN_00108980(param_1,local_108,&local_e8);
    if (local_108 == uVar5) {
      lVar10 = FUN_001074e0(param_4,&DAT_00116be9);
      if (lVar10 != 0) {
        FUN_0010a430(param_1,local_110,local_108,lVar10,param_4,local_e8);
        if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return;
      }
      piVar6 = __errno_location();
      if ((*piVar6 != 0x18) || (local_108 < 3)) {
        uVar7 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
        FUN_00107480(uVar7,param_4);
      }
    }
    else if (uVar5 < 3) {
      lVar10 = param_1[uVar5 * 2];
      uVar7 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_00107480(uVar7,lVar10);
    }
    uVar5 = uVar5 - 1;
    plVar11 = param_1 + uVar5 * 2;
    while( true ) {
      FUN_00107660(*(undefined8 *)(local_e8 + uVar5 * 8),*plVar11);
      lVar10 = FUN_00108c20(&local_e0,2 < uVar5);
      if (lVar10 != 0) break;
      uVar5 = uVar5 - 1;
      plVar11 = plVar11 + -2;
    }
    uVar4 = local_110;
    if (uVar5 < local_110) {
      uVar4 = uVar5;
    }
    lVar3 = local_108 - uVar5;
    FUN_0010a430(param_1,uVar4,uVar5,local_e0,lVar10 + 0xd,local_e8);
    local_108 = lVar3 + 1;
    *param_1 = lVar10 + 0xd;
    param_1[1] = lVar10;
    memmove(param_1 + 2,plVar11,lVar3 * 0x10);
    local_110 = (local_110 - uVar4) + 1;
  } while( true );
}




// Function: sortlines @ 0xb220

void sortlines(long param_1,ulong param_2,ulong param_3,long *param_4,undefined8 *param_5,
                 undefined8 param_6,undefined8 param_7)

{
  pthread_cond_t *__cond;
  pthread_mutex_t *__mutex;
  pthread_mutex_t *__mutex_00;
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int iVar5;
  long *plVar6;
  undefined8 *puVar7;
  long lVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  long lVar13;
  undefined8 *puVar14;
  char cVar15;
  long lVar16;
  long lVar17;
  long lVar18;
  undefined8 *puVar19;
  ulong uVar20;
  ulong uVar21;
  ulong uVar22;
  long lVar23;
  long in_FS_OFFSET;
  pthread_t local_80;
  long local_78;
  ulong local_70;
  ulong local_68;
  long local_60;
  undefined8 *local_58;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  
  uVar22 = param_2 >> 1;
  uVar21 = param_4[5];
  uVar20 = param_4[6];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = param_4[8];
  local_48 = param_7;
  local_78 = param_1;
  local_70 = uVar22;
  local_68 = param_3;
  local_58 = param_5;
  local_50 = param_6;
  if ((0x1ffff < uVar21 + uVar20) && (1 < param_2)) {
    iVar5 = pthread_create(&local_80,(pthread_attr_t *)0x0,FUN_0010ba80,&local_78);
    if (iVar5 == 0) {
      sortlines(param_1 + param_4[5] * -0x20,param_2 - uVar22,param_3,param_4[9],param_5,param_6,
                   param_7);
      pthread_join(local_80,(void **)0x0);
      goto LAB_0010b922;
    }
    uVar21 = param_4[5];
    uVar20 = param_4[6];
  }
  lVar16 = param_1 + param_3 * -0x20;
  lVar23 = param_1 + uVar21 * -0x20;
  if (1 < uVar20) {
    FUN_0010a180(lVar23,uVar20,lVar16 + (uVar21 >> 1) * -0x20,0);
  }
  if (1 < uVar21) {
    FUN_0010a180(param_1,uVar21,lVar16,0);
  }
  *param_4 = param_1;
  __cond = (pthread_cond_t *)(param_5 + 6);
  param_4[1] = lVar23;
  param_4[2] = lVar23;
  param_4[3] = param_1 + uVar21 * -0x20 + uVar20 * -0x20;
  __mutex = (pthread_mutex_t *)(param_5 + 1);
  pthread_mutex_lock(__mutex);
  FUN_0010df40(*param_5,param_4);
  *(undefined1 *)((long)param_4 + 0x54) = 1;
  pthread_cond_signal(__cond);
  pthread_mutex_unlock(__mutex);
LAB_0010b380:
  pthread_mutex_lock(__mutex);
  while (plVar6 = (long *)FUN_0010e010(*param_5), plVar6 == (long *)0x0) {
    pthread_cond_wait(__cond,__mutex);
  }
  pthread_mutex_unlock(__mutex);
  __mutex_00 = (pthread_mutex_t *)(plVar6 + 0xb);
  pthread_mutex_lock(__mutex_00);
  iVar5 = (int)plVar6[10];
  *(undefined1 *)((long)plVar6 + 0x54) = 0;
  if (iVar5 != 0) {
    puVar1 = (undefined8 *)*plVar6;
    puVar2 = (undefined8 *)plVar6[1];
    puVar12 = (undefined8 *)plVar6[2];
    lVar16 = (param_3 >> ((char)iVar5 * '\x02' + 2U & 0x3f)) + 1;
    puVar14 = puVar1;
    if (iVar5 != 1) {
      puVar9 = (undefined8 *)plVar6[4];
      puVar10 = (undefined8 *)*puVar9;
      puVar7 = puVar2;
      lVar8 = lVar16;
      puVar19 = puVar1;
      if (puVar1 == puVar12) {
        cVar15 = '\0';
        lVar18 = 0;
        lVar17 = 0;
      }
      else {
LAB_0010b464:
        do {
          if ((undefined8 *)plVar6[3] == puVar7) {
            puVar14 = (undefined8 *)*plVar6;
            cVar15 = *(char *)((long)plVar6 + 0x54);
            puVar9 = (undefined8 *)plVar6[4];
LAB_0010b66d:
            lVar18 = (long)puVar2 - (long)puVar7 >> 5;
            lVar17 = (long)puVar1 - (long)puVar19 >> 5;
            lVar23 = plVar6[6];
            lVar13 = plVar6[5];
            if (lVar23 != lVar18) goto LAB_0010b4f5;
            if ((puVar12 == puVar14) ||
               (lVar16 = lVar8 + -1, puVar7 = puVar10, puVar2 = puVar14, lVar8 == 0))
            goto LAB_0010b4fe;
            goto LAB_0010b6b2;
          }
          lVar16 = lVar8 + -1;
          if (lVar8 == 0) {
            puVar14 = (undefined8 *)*plVar6;
            cVar15 = *(char *)((long)plVar6 + 0x54);
            lVar8 = -1;
            puVar9 = (undefined8 *)plVar6[4];
            goto LAB_0010b66d;
          }
          puVar11 = puVar10 + -4;
          iVar5 = FUN_0010a0b0(puVar19 + -4,puVar7 + -4);
          lVar8 = lVar16;
          if (iVar5 < 1) {
            lVar23 = *plVar6;
            puVar12 = (undefined8 *)plVar6[2];
            uVar3 = *(undefined8 *)(lVar23 + -0x20);
            uVar4 = *(undefined8 *)(lVar23 + -0x18);
            puVar19 = (undefined8 *)(lVar23 + -0x20);
            *plVar6 = (long)puVar19;
            *puVar11 = uVar3;
            puVar10[-3] = uVar4;
            uVar3 = *(undefined8 *)(lVar23 + -8);
            puVar7 = (undefined8 *)plVar6[1];
            puVar10[-2] = *(undefined8 *)(lVar23 + -0x10);
            puVar10[-1] = uVar3;
            puVar10 = puVar11;
            if (puVar12 == puVar19) break;
            goto LAB_0010b464;
          }
          lVar23 = plVar6[1];
          puVar19 = (undefined8 *)*plVar6;
          puVar12 = (undefined8 *)plVar6[2];
          uVar3 = *(undefined8 *)(lVar23 + -0x20);
          uVar4 = *(undefined8 *)(lVar23 + -0x18);
          puVar7 = (undefined8 *)(lVar23 + -0x20);
          plVar6[1] = (long)puVar7;
          *puVar11 = uVar3;
          puVar10[-3] = uVar4;
          uVar3 = *(undefined8 *)(lVar23 + -8);
          puVar10[-2] = *(undefined8 *)(lVar23 + -0x10);
          puVar10[-1] = uVar3;
          puVar10 = puVar11;
        } while (puVar12 != puVar19);
        cVar15 = *(char *)((long)plVar6 + 0x54);
        puVar9 = (undefined8 *)plVar6[4];
        lVar17 = (long)puVar1 - (long)puVar19 >> 5;
        lVar18 = (long)puVar2 - (long)puVar7 >> 5;
        puVar14 = puVar19;
        puVar10 = puVar11;
      }
      lVar23 = plVar6[6];
      lVar13 = plVar6[5];
      lVar8 = lVar16;
      if (lVar18 != lVar23) {
LAB_0010b4f5:
        if (((lVar17 == lVar13) && (puVar12 = (undefined8 *)plVar6[3], puVar12 != puVar7)) &&
           (lVar16 = lVar8 + -1, puVar19 = puVar10, lVar8 != 0)) {
          do {
            puVar11 = puVar7 + -4;
            uVar3 = *puVar11;
            uVar4 = puVar7[-3];
            puVar10 = puVar19 + -4;
            plVar6[1] = (long)puVar11;
            *puVar10 = uVar3;
            puVar19[-3] = uVar4;
            uVar3 = puVar7[-1];
            puVar19[-2] = puVar7[-2];
            puVar19[-1] = uVar3;
            if (puVar11 == puVar12) {
              lVar18 = (long)puVar2 - (long)puVar11 >> 5;
              goto LAB_0010b4fe;
            }
            lVar16 = lVar16 + -1;
            puVar19 = puVar10;
            puVar7 = puVar11;
          } while (lVar16 != -1);
          lVar18 = (long)puVar2 - (long)puVar11 >> 5;
        }
      }
      goto LAB_0010b4fe;
    }
    puVar7 = puVar2;
    lVar23 = lVar16;
    if (puVar1 == puVar12) {
      lVar18 = 0;
      lVar8 = 0;
    }
    else {
LAB_0010b5a8:
      do {
        if ((undefined8 *)plVar6[3] == puVar7) {
LAB_0010b808:
          lVar18 = plVar6[6];
          if ((long)puVar2 - (long)puVar7 >> 5 != lVar18) {
            lVar13 = plVar6[5];
            lVar8 = (long)puVar1 - (long)puVar14 >> 5;
            goto LAB_0010b8a8;
          }
          if (((undefined8 *)plVar6[2] != puVar14) && (lVar23 != 0)) goto LAB_0010b856;
          cVar15 = *(char *)((long)plVar6 + 0x54);
          lVar13 = plVar6[5];
          lVar23 = lVar18;
          goto LAB_0010b501;
        }
        lVar16 = lVar23 + -1;
        if (lVar23 == 0) {
          lVar23 = -1;
          goto LAB_0010b808;
        }
        iVar5 = FUN_0010a0b0(puVar14 + -4,puVar7 + -4);
        lVar23 = lVar16;
        if (iVar5 < 1) {
          lVar18 = *plVar6;
          *plVar6 = lVar18 + -0x20;
          FUN_0010a3c0(lVar18 + -0x20,param_6,param_7);
          puVar14 = (undefined8 *)*plVar6;
          puVar7 = (undefined8 *)plVar6[1];
          if (puVar14 == (undefined8 *)plVar6[2]) break;
          goto LAB_0010b5a8;
        }
        lVar18 = plVar6[1];
        plVar6[1] = lVar18 + -0x20;
        FUN_0010a3c0(lVar18 + -0x20,param_6,param_7);
        puVar14 = (undefined8 *)*plVar6;
        puVar7 = (undefined8 *)plVar6[1];
      } while (puVar14 != (undefined8 *)plVar6[2]);
      lVar8 = (long)puVar1 - (long)puVar14 >> 5;
      lVar18 = (long)puVar2 - (long)puVar7 >> 5;
    }
    lVar13 = plVar6[5];
    lVar23 = lVar16;
    if (plVar6[6] == lVar18) {
      cVar15 = *(char *)((long)plVar6 + 0x54);
      lVar23 = lVar18;
    }
    else {
LAB_0010b8a8:
      if (((lVar13 == lVar8) && (puVar7 != (undefined8 *)plVar6[3])) && (lVar23 != 0)) {
        do {
          lVar23 = lVar23 + -1;
          plVar6[1] = (long)(puVar7 + -4);
          FUN_0010a3c0(puVar7 + -4,param_6,param_7);
          puVar7 = (undefined8 *)plVar6[1];
          if (puVar7 == (undefined8 *)plVar6[3]) break;
        } while (lVar23 != 0);
        cVar15 = *(char *)((long)plVar6 + 0x54);
        puVar14 = (undefined8 *)*plVar6;
        lVar13 = plVar6[5];
        lVar18 = (long)puVar2 - (long)puVar7 >> 5;
        lVar23 = plVar6[6];
      }
      else {
        cVar15 = *(char *)((long)plVar6 + 0x54);
        lVar18 = (long)puVar2 - plVar6[1] >> 5;
        lVar23 = plVar6[6];
      }
    }
    goto LAB_0010b501;
  }
  pthread_mutex_unlock(__mutex_00);
  pthread_mutex_lock(__mutex);
  FUN_0010df40(*param_5,plVar6);
  *(undefined1 *)((long)plVar6 + 0x54) = 1;
  pthread_cond_signal(__cond);
  pthread_mutex_unlock(__mutex);
LAB_0010b922:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
  while (lVar23 != 0) {
LAB_0010b856:
    lVar23 = lVar23 + -1;
    *plVar6 = (long)(puVar14 + -4);
    FUN_0010a3c0(puVar14 + -4,param_6,param_7);
    puVar14 = (undefined8 *)*plVar6;
    if (puVar14 == (undefined8 *)plVar6[2]) break;
  }
  cVar15 = *(char *)((long)plVar6 + 0x54);
  lVar18 = (long)puVar2 - plVar6[1] >> 5;
  lVar13 = plVar6[5];
  lVar23 = plVar6[6];
  goto LAB_0010b501;
  while (lVar16 = lVar16 + -1, puVar7 = puVar10, puVar2 = puVar14, lVar16 != -1) {
LAB_0010b6b2:
    puVar14 = puVar2 + -4;
    uVar3 = *puVar14;
    uVar4 = puVar2[-3];
    puVar10 = puVar7 + -4;
    *plVar6 = (long)puVar14;
    *puVar10 = uVar3;
    puVar7[-3] = uVar4;
    uVar3 = puVar2[-1];
    puVar7[-2] = puVar2[-2];
    puVar7[-1] = uVar3;
    if (puVar14 == puVar12) break;
  }
LAB_0010b4fe:
  *puVar9 = puVar10;
LAB_0010b501:
  plVar6[5] = lVar13 - ((long)puVar1 - (long)puVar14 >> 5);
  plVar6[6] = lVar23 - lVar18;
  if (cVar15 == '\0') {
    FUN_0010b1a0(param_5,plVar6);
  }
  if (*(uint *)(plVar6 + 10) < 2) {
    if (plVar6[6] + plVar6[5] == 0) {
      lVar16 = plVar6[7];
      pthread_mutex_lock(__mutex);
      FUN_0010df40(*param_5,lVar16);
      *(undefined1 *)(lVar16 + 0x54) = 1;
      pthread_cond_signal(__cond);
      pthread_mutex_unlock(__mutex);
    }
  }
  else {
    pthread_mutex_lock((pthread_mutex_t *)(plVar6[7] + 0x58));
    lVar16 = plVar6[7];
    if (*(char *)(lVar16 + 0x54) == '\0') {
      FUN_0010b1a0(param_5);
      lVar16 = plVar6[7];
    }
    pthread_mutex_unlock((pthread_mutex_t *)(lVar16 + 0x58));
  }
  pthread_mutex_unlock(__mutex_00);
  goto LAB_0010b380;
}




// Function: sortlines_thread @ 0xba80

undefined8 sortlines_thread(undefined8 *param_1)

{
  FUN_0010b220(*param_1,param_1[1],param_1[2],param_1[3],param_1[4],param_1[5],param_1[6]);
  return 0;
}




// Function: usage @ 0xbab0

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  char *pcVar6;
  undefined **ppuVar7;
  char *pcVar8;
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
  
  uVar5 = DAT_0011d888;
  ppuVar7 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar3,uVar5,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"Write sorted concatenation of all FILE(s) to standard output.\n",5
                              );
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"\nWith no FILE, or when FILE is -, read standard input.\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"Ordering options:\n\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"Other options:\n\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    uVar5 = dcgettext(0,
                      "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"
                      ,5);
    __printf_chk(1,uVar5,&DAT_00116c32);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"  -z, --zero-terminated     line delimiter is NUL, not newline\n",
                               5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    local_b8 = &DAT_00116c06;
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
      iVar2 = strcmp("sort",pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar7 + 0x10);
      ppuVar7 = (undefined **)((long)ppuVar7 + 0x10);
    } while (pcVar4 != (char *)0x0);
    pcVar4 = *(char **)((long)ppuVar7 + 8);
    if (pcVar4 == (char *)0x0) {
      pcVar4 = "sort";
    }
    uVar5 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar5,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar6 = setlocale(5,(char *)0x0);
    if (pcVar6 != (char *)0x0) {
      iVar2 = strncmp(pcVar6,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar6 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar6,pFVar1);
      }
    }
    pcVar6 = "sort";
    iVar2 = strcmp("sort","[");
    if (iVar2 == 0) {
      pcVar6 = "test";
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar8 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",pcVar6);
    if (pcVar4 != "sort") {
      pcVar8 = "";
    }
    uVar5 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar5,pcVar4,pcVar8);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar5);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: raw_hasher @ 0xccd0

ulong raw_hasher(ulong param_1,ulong param_2)

{
  return (param_1 >> 3 | param_1 << 0x3d) % param_2;
}




// Function: raw_comparator @ 0xccf0

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0xcd00

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_00117200) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_00117214 < fVar1) && (fVar1 < DAT_00117218)) && (_DAT_0011721c < pfVar2[3])) &&
     (_DAT_001170f0 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_00117214;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_00117220)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_00117200;
  return 0;
}




// Function: hash_find_entry @ 0xcd90

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
LAB_0010ce3f:
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
        goto LAB_0010ce3f;
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




// Function: transfer_entries @ 0xcfe0

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
            if (uVar4 <= uVar2) goto LAB_0010d14b;
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
LAB_0010d14b:
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




// Function: hash_print_statistics @ 0xd250

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
      if ((long *)param_1[1] <= plVar4) goto LAB_0010d2bd;
    }
    uVar5 = 1;
    for (lVar3 = plVar4[1]; lVar3 != 0; lVar3 = *(long *)(lVar3 + 8)) {
      uVar5 = uVar5 + 1;
    }
    if (uVar6 < uVar5) {
      uVar6 = uVar5;
    }
  }
LAB_0010d2bd:
  __fprintf_chk(param_2,1,"# entries:         %lu\n",param_1[4]);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar1);
  __fprintf_chk(((double)uVar2 * _DAT_001170f0) / (double)uVar1,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar2);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar6);
  return;
}




// Function: hash_lookup @ 0xd3b0

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




// Function: hash_get_next @ 0xd470

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




// Function: hash_do_for_each @ 0xd550

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




// Function: hash_initialize @ 0xd640

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_0010ccd0;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_0010ccf0;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_00117200;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_0010cd00(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_0010cea0(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0xd750

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
        if ((long *)param_1[1] <= plVar7) goto LAB_0010d7eb;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010d7eb:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0xd800

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
    if (plVar3 <= plVar5) goto LAB_0010d8ac;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_0010d86f;
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
LAB_0010d86f:
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
LAB_0010d8ac:
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




// Function: hash_rehash @ 0xd8f0

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
  __nmemb = FUN_0010cea0(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_0010d9f6;
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
      uVar2 = FUN_0010cfe0(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_0010cfe0(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_0010cfe0(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_0010d9f6;
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
      goto LAB_0010d9f6;
    }
  }
  uVar2 = 0;
LAB_0010d9f6:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0xda70

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
  if (param_2 == 0) goto LAB_0010dc73;
  lVar2 = FUN_0010cd90(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar6 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0010dabe;
  }
  uVar5 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar5 < 0) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar3) goto LAB_0010daff;
LAB_0010db86:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar3;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar3 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar3 < 0) goto LAB_0010db86;
LAB_0010daff:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar3;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar8._0_4_ < auVar9._0_4_) {
    FUN_0010cd00(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar9._0_4_ <= auVar8._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0010db1e;
    fVar7 = auVar8._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar7 = fVar7 * *(float *)(lVar2 + 8);
    }
    if (fVar7 < _DAT_00117224) {
      if (_DAT_00117228 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00117228) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar1 = FUN_0010d8f0(param_1,uVar5);
      if (cVar1 != '\0') {
        lVar2 = FUN_0010cd90(param_1,param_2,&local_38,0);
        if (lVar2 != 0) {
LAB_0010dc73:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        goto LAB_0010db1e;
      }
    }
LAB_0010dbdd:
    uVar6 = 0xffffffff;
  }
  else {
LAB_0010db1e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar6 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0010dabe;
    }
    plVar4 = *(long **)(param_1 + 0x48);
    if (plVar4 == (long *)0x0) {
      plVar4 = malloc(0x10);
      if (plVar4 == (long *)0x0) goto LAB_0010dbdd;
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
LAB_0010dabe:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: hash_insert @ 0xdca0

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010da70(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xdd00

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
  lVar4 = FUN_0010cd90(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_0010dd43;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_0010dd90;
LAB_0010de46:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_0010de46;
LAB_0010dd90:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_0010cd00(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_00117228 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00117228) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_0010d8f0(param_1,uVar5);
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
LAB_0010dd43:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}




// Function: num_processors @ 0xe570

ulong num_processors(int param_1)

{
  int iVar1;
  ulong uVar2;
  char *pcVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  cpu_set_t cStack_a8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == 2) {
    pcVar3 = getenv("OMP_NUM_THREADS");
    if (pcVar3 == (char *)0x0) {
      pcVar3 = getenv("OMP_THREAD_LIMIT");
      if ((pcVar3 == (char *)0x0) || (uVar2 = FUN_0010e4b0(), uVar2 == 0)) goto LAB_0010e735;
    }
    else {
      uVar4 = FUN_0010e4b0(pcVar3);
      pcVar3 = getenv("OMP_THREAD_LIMIT");
      if (pcVar3 == (char *)0x0) {
        uVar2 = 0xffffffffffffffff;
      }
      else {
        uVar2 = FUN_0010e4b0();
        if (uVar2 == 0) {
          uVar2 = 0xffffffffffffffff;
        }
      }
      if (uVar4 != 0) {
        if (uVar4 < uVar2) {
          uVar2 = uVar4;
        }
        goto LAB_0010e5cb;
      }
    }
  }
  else {
    if (param_1 != 1) {
      uVar2 = sysconf(0x53);
      uVar4 = uVar2;
      if (uVar2 - 1 < 2) {
        iVar1 = sched_getaffinity(0,0x80,&cStack_a8);
        if (iVar1 != 0) goto LAB_0010e5cb;
        iVar1 = __sched_cpucount(0x80,&cStack_a8);
        uVar4 = (ulong)iVar1;
        if ((uVar4 == 0) || (uVar4 <= uVar2)) goto LAB_0010e5cb;
      }
      uVar2 = 1;
      if (0 < (long)uVar4) {
        uVar2 = uVar4;
      }
      goto LAB_0010e5cb;
    }
LAB_0010e735:
    uVar2 = 0xffffffffffffffff;
  }
  iVar1 = sched_getaffinity(0,0x80,&cStack_a8);
  if (iVar1 == 0) {
    iVar1 = __sched_cpucount(0x80,&cStack_a8);
    uVar4 = (ulong)iVar1;
    if (uVar4 != 0) {
      if (uVar4 < uVar2) {
        uVar2 = uVar4;
      }
      goto LAB_0010e5cb;
    }
  }
  uVar4 = sysconf(0x54);
  if (uVar4 <= uVar2) {
    uVar2 = uVar4;
  }
  if ((long)uVar4 < 1) {
    uVar2 = 1;
  }
LAB_0010e5cb:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}



