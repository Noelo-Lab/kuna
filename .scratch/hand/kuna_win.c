    *(unsigned int *)((long)v38 + 0xc4) = (unsigned int)v2;
    if (*(int *)((long)v38 + 0xc4))
      dat_2b340 = '\x01';
  }
  if (((((((!v17) && (!dat_2b3d5)) && (!dat_2b440)) && (((v18 != 3 || (!dat_2b3d4)) || ((!sub_6325(0x13) && ((!sub_6325(0x12) && (!sub_6325(0x14))))))))) && (((!dat_2b3e9 && (!dat_2b441)) || (((v18 != 6 && (v18)) || ((dat_2b3ec != 4 && ((!dat_2b358 && (!dat_2b3e8)))))))))) && ((!dat_2b3e9 || (v16)))) && ((v15 = &v10, !dat_2b441 || (((v18 && (!v17)) && ((v15 = &v10, v18 != 5 || ((dat_2b3d0 != 3 && ((v15 = &v10, !dat_2b3d4 || ((((!sub_6325(0xe) && (!sub_6325(0x10))) && (!sub_6325(0x11))) && (v15 = &v10, !sub_6325(0x15))))))))))))))) goto label_a7ca;
  if ((*v19 != '/') && (*v10)) {
    v6 = strlen(v19);
    v7 = strlen(v10);
    v6 = ((v7 + v6 + 0x19) / 0x10) * 0x10;
    for (; v14 != (char **)((long)&v10 - (v6 & 0xfffffffffffff000)); v14 = (char *)((long)v14 + -0x1000)) {
      *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
    }
    v1 = -(unsigned long)((unsigned int)v6 & 0xfff);
    v15 = (char *)((long)v14 + v1);
    if (v6 & 0xfff)
      *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v6 & 0xfff) - 8) + v1) = *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v6 & 0xfff) - 8) + v1);
    v37 = (char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0);
    *(unsigned long *)((long)v14 + v1 + -8) = 0x9e47;
    sub_eb94((char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0),v10,v19);
  }
  else {
    v37 = v19;
    v15 = &v10;
  }
  if (dat_2b3d5) {
    *(unsigned long *)((long)v15 + -8) = 0x9e66;
    v8 = sub_1101b(v37,2);
    v38[2] = v8;
    if (!v38[2]) {
      *(unsigned long *)((long)v15 + -8) = 0x9e93;
      v9 = gettext("error canonicalizing %s",*(char *)((long)v15 + -8));
      *(unsigned long *)((long)v15 + -8) = 0x9eae;
      sub_8de7(v17,v9,v37);
    }
  }
  if (4 <= dat_2b3ec) { // branch-flip
    if (dat_2b3ec == 4) {
      *(unsigned long *)((long)v15 + -8) = 0x9ee9;
      v24 = sub_5c0e(v37,&v38[3]);
      v20 = 1;
      goto label_9fac;
    }
label_9f7c:
    *(unsigned long *)((long)v15 + -8) = 0x9f99;
    v24 = sub_5c48(v37,&v38[3]);
    v20 = 0;
  }
  else {
    if ((dat_2b3ec <= 1) || (!v17)) goto label_9f7c;
    *(unsigned long *)((long)v15 + -8) = 0x9f21;
    v24 = sub_5c0e(v37,&v38[3]);
    v20 = 1;
    if (dat_2b3ec != 2) {
      if (0 <= (int)v24) // branch-flip
        v23 = (*(unsigned int *)&v38[6] & 0xf000) != 0x4000;
      else {
        *(unsigned long *)((long)v15 + -8) = 0x9f47;
        v3 = *(char *)((long)v15 + -8);
        v23 = *__errno_location(v3) == 2;
      }
      if (v23 == 1) goto label_9f7c;
    }
  }
label_9fac:
  if (v24) {
    *(unsigned long *)((long)v15 + -8) = 0x9fc8;
    v9 = gettext("cannot access %s",*(char *)((long)v15 + -8));
    *(unsigned long *)((long)v15 + -8) = 0x9fe3;
    sub_8de7(v17,v9,v37);
    v38[0x16] = 0x2b020;
    if (v17)
      return 0;
    *(unsigned long *)((long)v15 + -8) = 0xa01a;
    *v38 = sub_1cc92(v19);
    dat_2b338 += 1;
    return 0;
  }
  *(char *)&v38[0x17] = 1;
  if (((v18 == 5) || ((*(unsigned int *)&v38[6] & 0xf000) == 0x8000)) && (dat_2b3d4)) {
    *(unsigned long *)((long)v15 + -8) = 0xa085;
    if (sub_6325(0x15)) {
      *(unsigned long *)((long)v15 + -8) = 0xa0a2;
      v3 = sub_9936(v37,v38);
      *(char *)&v38[0x18] = v3;
    }
  }
  if ((!dat_2b3ac) || (dat_2b380)) {
    v21 = 0;
    v22 = 0;
    *(unsigned long *)((long)v15 + -8) = 0xa0f6;
    v25 = sub_97fa(v37,v38,v20);
    v24 = -(v25 >> 0x1f);
    if (-(v25 >> 0x1f)) { // branch-flip
      v38[0x16] = 0x2b020;
      *(unsigned long *)((long)v15 + -8) = 0xa194;
      v3 = *(char *)((long)v15 + -8);
      v4 = *__errno_location(v3);
      *(unsigned long *)((long)v15 + -8) = 0xa19d;
      if (!sub_524c(v4)) {
        *(unsigned long *)((long)v15 + -8) = 0xa1a6;
