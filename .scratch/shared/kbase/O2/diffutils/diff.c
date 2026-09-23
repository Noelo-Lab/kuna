// Function: sub_4f20 @ 0x4f20
unsigned long sub_4f20(int a0,char **a1)
{
  bool v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13;
  char *v14;
  unsigned long v15; // rcx
  unsigned long v16; // rdx
  long v17;
  char *v18; // stack - 0x48
  long v19; // stack - 0x58
  void *v2;
  long v20; // stack - 0x50
  int v21; // ebp
  long v22;
  char *v23;
  unsigned long v24;
  char **v25;
  char **v26;
  long v27;
  unsigned long v28; // stack - 0x88
  long v29; // stack - 0x80
  bool v3;
  int v4; // eax
  unsigned int v5; // eax
  int v6;
  char *v7; // rax
  unsigned long v8;
  unsigned long v9;
  
  v22 = (long)a0;
  v19 = 0;
  v20 = 0;
  dat_260b0 = 2;
  sub_14480(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_fd50(0);
  dat_26600 = 0x26500;
  dat_265c0 = 0x264c0;
  re_set_syntax(0x50a46);
  dat_26410 = sub_10310();
  dat_26180 = 0;
  sub_16f50();
  v3 = 0;
  v28 = 0;
  v1 = 0;
  v29 = -1;
  v6 = -1;
label_5020:
  v21 = (int)v22;
  v4 = getopt_long(v21,a1,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(void *)0x25180,NULL);
  v9 = dat_36860;
  v23 = optarg;
  if (v4 != -1) {
    switch(v4) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_8e40(0,0); // no-return
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
        if (10 <= (unsigned int)(v6 - 0x30U)) { // branch-flip
          v29 = (long)(v4 + -0x30);
          v6 = v4;
        }
        else if (0x666666666666666 <= (long)(v29 - (unsigned long)(v4 <= 0x33))) { // branch-flip
          v29 = 0x3fffffffffffffff;
          v6 = v4;
        }
        else {
          v29 = (long)(v4 + -0x30) + v29 * 10;
          v6 = v4;
        }
        goto label_5020;
      case 0x42:
        dat_26554 = 1;
        v6 = v4;
        goto label_5020;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v17 = strtoimax(optarg,&v18,10);
          if ((*v18) || (v17 < 0))
            sub_8e40("invalid context length \'%s\'",optarg); // no-return
          v27 = 0x3fffffffffffffff;
          if (v17 <= 0x3fffffffffffffff)
            v27 = v17;
        }
        else {
          v27 = 3;
        }
        sub_8f60((v4 == 0x55) + '\x02');
        if (v27 <= dat_26570) { // branch-flip
          v1 = 1;
          v6 = v4;
        }
        else {
          v1 = 1;
          dat_26570 = v27;
          v6 = v4;
        }
        goto label_5020;
      case 0x44:
        sub_8f60(7);
        v8 = strlen(optarg);
        v23 = (char *)sub_16a40(v8 * 7 + 0x61);
        __sprintf_chk(v23,1,0xffffffffffffffff,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        v22 = 0x26460;
        do {
          v27 = v22 + 8;
          sub_8f00(v22,v23,"-D");
          v23 = &v23[strlen(v23) + 1];
          v22 = v27;
        } while (v27 != 0x26480);
        v22 = (long)v21;
        v6 = v4;
        goto label_5020;
      case 0x45:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 |= 1;
        goto label_5020;
      case 0x46:
        sub_8d60(0x265e0,optarg);
        v6 = v4;
        goto label_5020;
      case 0x48:
        dat_26418 = 1;
        v6 = v4;
        goto label_5020;
      case 0x49:
        sub_8d60(0x265a0,optarg);
        v6 = v4;
        goto label_5020;
      case 0x4c:
        if (dat_26540) { // branch-flip
          if (dat_26548)
            sub_e7b0("too many file label options"); // no-return
          dat_26548 = optarg;
          v6 = v4;
        }
        else {
          dat_26540 = optarg;
          v6 = v4;
        }
        goto label_5020;
      case 0x4e:
        dat_26586 = 1;
        v6 = v4;
        goto label_5020;
      case 0x50:
        dat_26585 = 1;
        v6 = v4;
        goto label_5020;
      case 0x53:
        sub_8f00(0x26488,optarg,"-S");
        v6 = v4;
        goto label_5020;
      case 0x54:
        dat_26492 = 1;
        v6 = v4;
        goto label_5020;
      case 0x57:
        v8 = strtoimax(optarg,&v18,10);
        if (((long)v8 <= 0) || (*v18))
          sub_8e40("invalid width \'%s\'",optarg); // no-return
        if (v8 != v28) {
          if (v28)
            sub_e7b0("conflicting width options"); // no-return
          v28 = v8;
          v6 = v4;
          goto label_5020;
        }
        break;
      case 0x58:
        v6 = sub_10bd0(sub_10680,dat_26410,optarg,(-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010,10);
        if (v6)
          sub_e770(optarg); // no-return
        break;
      case 0x5a:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 |= 2;
        goto label_5020;
      case 0x61:
        dat_26568 = 1;
        v6 = v4;
        goto label_5020;
      case 0x62:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 = 4;
        goto label_5020;
      case 99:
        v9 = 2;
        goto label_5968;
      case 100:
        dat_26408 = 1;
        v6 = v4;
        goto label_5020;
      case 0x65:
        sub_8f60(4);
        v6 = v4;
        goto label_5020;
      case 0x66:
        sub_8f60(5);
        v6 = v4;
        goto label_5020;
      case 0x69:
        dat_26552 = 1;
        v6 = v4;
        goto label_5020;
      case 0x6c:
        if (!s_1e558[0])
          sub_8e40("pagination not supported on this host",0); // no-return
        dat_26480 = 1;
        signal(0x11,0);
        v6 = v4;
        goto label_5020;
      case 0x6e:
        sub_8f60(6);
        v6 = v4;
        goto label_5020;
      case 0x70:
        sub_8d60(0x265e0,"^[[:alpha:]$_]");
        v3 = 1;
        v6 = v4;
        goto label_5020;
      case 0x71:
        dat_264a1 = 1;
        v6 = v4;
        goto label_5020;
      case 0x72:
        dat_26608 = 1;
        v6 = v4;
        goto label_5020;
      case 0x73:
        dat_26584 = 1;
        v6 = v4;
        goto label_5020;
      case 0x74:
        dat_264a0 = '\x01';
        v6 = v4;
        goto label_5020;
      case 0x75:
        v9 = 3;
label_5968:
        sub_8f60(v9);
        v6 = v4;
        if (dat_26570 <= 2)
          dat_26570 = 3;
        goto label_5020;
      case 0x76:
        v9 = sub_14920("Len Tower");
        v10 = sub_14920("Richard Stallman");
        v11 = sub_14920("David Hayes");
        v12 = sub_14920("Mike Haertel");
        sub_16840(stdout,"diff","GNU diffutils",dat_260a8,sub_14920("Paul Eggert"),v12,v11,v10,v9,0);
        sub_9df0();
        return 0;
      case 0x77:
        dat_26558 = 5;
        v6 = v4;
        goto label_5020;
      case 0x78:
        sub_10680(dat_26410,optarg,(-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010);
        v6 = v4;
        goto label_5020;
      case 0x79:
        sub_8f60(8);
        v6 = v4;
        goto label_5020;
      case 0x81:
        sub_8f00(&v19,optarg,"--from-file");
        v6 = v4;
        goto label_5020;
      case 0x82:
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILES\n",5),v9);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare FILES line by line.",5));
        v2 = stdout;
        v22 = 0x24f40;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v2);
        v23 = "    --normal                  output a normal diff (the default)";
        do {
          if (*v23) { // branch-flip
            v23 = dcgettext(NULL,v23,5);
            while (v7 = strchr(v23,10), v7) {
              v8 = (long)&v7[1] - (long)v23;
              if (0xfff < (int)v8)
                __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
              __printf_chk(1,"  %.*s",v8 & 0xffffffff,v23);
              v23 = &v7[1];
            }
            __printf_chk(1,&"  %s\n"[(int)(((unsigned int)(*v23 != '-') & (unsigned int)CONCAT71((undefined7)((unsigned long)v16 >> 8),*v23 != ' ')) * 2)],v23);
            v23 = *(char **)(v22 + 8);
          }
          else {
            v14 = *(char **)((long)stdout + 0x28);
            if (*(char **)((long)stdout + 0x30) <= v14) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)((long)stdout + 0x28) = &v14[1];
              *v14 = 10;
            }
            v23 = *(char **)(v22 + 8);
          }
          v22 += 8;
        } while (v23);
        sub_16920();
        sub_9df0();
        return 0;
      case 0x83:
        v27 = strtoimax(optarg,&v18,10);
        if ((*v18) || (v27 < 0))
          sub_8e40("invalid horizon length \'%s\'",optarg); // no-return
        if (dat_26560 < v27)
          dat_26560 = v27;
        v6 = v4;
        goto label_5020;
      case 0x84:
        dat_26551 = '\x01';
        v6 = v4;
        goto label_5020;
      case 0x86:
        dat_26439 = 1;
        v6 = v4;
        goto label_5020;
      case 0x87:
        sub_8f60(7);
        v27 = 0x26440;
        do {
          v17 = v27 + 8;
          sub_8f00(v27,optarg,"--line-format");
          v27 = v17;
        } while (v17 != 0x26458);
        v6 = v4;
        goto label_5020;
      case 0x88:
        dat_26550 = 1;
        v6 = v4;
        goto label_5020;
      case 0x89:
        dat_26551 = '\0';
        v6 = v4;
        goto label_5020;
      case 0x8a:
        sub_8f60(1);
        v6 = v4;
        goto label_5020;
      case 0x8b:
        sub_8f60(8);
        dat_2643a = 1;
        v6 = v4;
        goto label_5020;
      case 0x8c:
        dat_26490 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8d:
        dat_26491 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8e:
        dat_26438 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8f:
        v8 = strtoimax(optarg,&v18,10);
        if ((((long)v8 <= 0) || (0xfffffffffffffffd <= v8)) || (*v18))
          sub_8e40("invalid tabsize \'%s\'",optarg); // no-return
        if (v8 != dat_26498) {
          if (dat_26498)
            sub_e7b0("conflicting tabsize options"); // no-return
          dat_26498 = v8;
          v6 = v4;
          goto label_5020;
        }
        break;
      case 0x90:
        sub_8f00(&v20,optarg,"--to-file");
        v6 = v4;
        goto label_5020;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_8f60(7);
        v6 = v4 + -0x91;
        sub_8f00((long)v6 * 8 + 0x26440,optarg,&"--unchanged-line-format"[(long)v6 * 0x18]);
        goto label_5020;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_8f60(7);
        v6 = v4 + -0x94;
        sub_8f00((long)v6 * 8 + 0x26460,optarg,&"--unchanged-group-format"[(long)v6 * 0x19]);
        goto label_5020;
      case 0x98:
        if ((optarg) && (v6 = strcmp(optarg,"auto"), v6)) {
          if (strcmp(v23,"always")) { // branch-flip
            if (strcmp(v23,"never"))
              sub_8e40("invalid color \'%s\'",v23); // no-return
            dat_2657c = 0;
            v6 = v4;
          }
          else {
            dat_2657c = 2;
            v6 = v4;
          }
        }
        else {
          dat_2657c = 1;
          v6 = v4;
        }
        goto label_5020;
      case 0x99:
        sub_e7f0(optarg);
        v6 = v4;
        goto label_5020;
      case 0x9a:
        goto label_5729;
      
    }
    v6 = v4;
    goto label_5020;
  }
  if (((dat_2657c == 1) && (v23 = getenv("TERM"), v23)) && (!strcmp(v23,"dumb")))
    dat_2657c = 0;
  if (dat_26580) { // branch-flip
label_5d2d:
    if ((dat_26580 != 2) || (sub_10f10(2)))
      dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    else {
      dat_26400 = "%a %b %e %T %Y";
    }
    if (0 <= v29) { // branch-flip
label_5b3f:
      v27 = dat_26570;
      if (((unsigned int)(dat_26580 - 2U) < 2) && ((dat_26570 < v29 || ((v29 != dat_26570 && (!v1)))))) {
        dat_26570 = v29;
        v27 = v29;
      }
    }
    else {
label_5d54:
      v27 = dat_26570;
    }
  }
  else {
    if (!v3) {
      sub_8f60(1);
      goto label_5d2d;
    }
    sub_8f60(2);
    if (0 <= v29) {
      if ((dat_26580 != 2) || (sub_10f10(2)))
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_26400 = "%a %b %e %T %Y";
      }
      goto label_5b3f;
    }
    dat_26570 = 3;
    if (dat_26580 == 2) {
      if (sub_10f10(2)) {
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
        v27 = dat_26570;
        goto label_5b58;
      }
      dat_26400 = "%a %b %e %T %Y";
      goto label_5d54;
    }
    dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    v27 = 3;
  }
label_5b58:
  if (!dat_26498)
    dat_26498 = 8;
  if (v28) { // branch-flip
    v8 = 1;
    if (!dat_264a0) {
label_5b88:
      v8 = dat_26498;
    }
    v15 = (v28 >> 1) + (v8 + 3 >> 1) + (unsigned long)((unsigned int)(v8 + 3) & (unsigned int)v28 & 1);
    v24 = v15 - v15 % v8;
    if ((4 <= v24) && (v24 < v28)) {
      dat_26430 = (v15 % v8 - v15) + v28;
      if (v24 - 3 < dat_26430)
        dat_26430 = v24 - 3;
      if (!dat_26430)
        v24 = v28;
      v28 = v24;
    }
    else {
      dat_26430 = 0;
    }
  }
  else {
    if (!dat_264a0) {
      v28 = 0x82;
      goto label_5b88;
    }
    v28 = 0x43;
    dat_26430 = 0x3f;
  }
  dat_26428 = v28;
  if (dat_26560 < v27)
    dat_26560 = v27;
  sub_8ea0(0x265e0);
  sub_8ea0(0x265a0);
  if (dat_26580 != 7) {
label_5c36:
    dat_26578 = dat_26580 != 8 | dat_26438;
    goto label_5c42;
  }
  if (!dat_26440)
    dat_26440 = "%l\n";
  if (!dat_26448)
    dat_26448 = "%l\n";
  if (!dat_26450)
    dat_26450 = "%l\n";
  if ((!dat_26468) && (dat_26468 = "%<", dat_26478))
    dat_26468 = dat_26478;
  if ((!dat_26470) && (dat_26470 = "%>", dat_26478))
    dat_26470 = dat_26478;
  if (dat_26460) { // branch-flip
    if (!dat_26478) {
label_604f:
      dat_26478 = (char *)sub_fa30(dat_26468,dat_26470,"");
      if (dat_26580 != 7) goto label_5c36;
    }
    v23 = dat_26460;
    v3 = 1;
    if (*dat_26460) goto label_5f6e;
  }
  else {
    v23 = "%=";
    dat_26460 = "%=";
    if (!dat_26478) goto label_604f;
label_5f6e:
    v3 = 0;
    if (!strcmp(v23,"%="))
      v3 = *dat_26440 == '\0';
  }
  dat_26578 = v3;
label_5c42:
  v25 = &a1[1];
  dat_26553 = ~(dat_26554 | dat_26552 | dat_26490 | (dat_265a0 || dat_26558)) & dat_264a1 & 1;
  if (2 <= optind) { // branch-flip
    v5 = optind - 2;
    v27 = 1;
    v26 = v25;
    do {
      v23 = *v26;
      v26 = &v26[1];
      v27 = v27 + 1 + sub_14d70(v23);
    } while (&a1[(unsigned long)v5 + 2] != v26);
    v13 = (char *)sub_16a40(v27);
    v14 = v13;
    do {
      *v14 = 0x20;
      v23 = *v25;
      v25 = &v25[1];
      v14 = (char *)sub_14de0(&v14[1],v23);
    } while (&a1[(unsigned long)v5 + 2] != v25);
  }
  else {
    v14 = (char *)sub_16a40(1);
    v13 = v14;
  }
  v17 = v20;
  v27 = v19;
  *v14 = 0;
  dat_26420 = v13;
  if (v19) { // branch-flip
    if (v20)
      sub_e7b0("--from-file and --to-file both specified"); // no-return
    v6 = 0;
    for (; optind < v21; optind = optind + 1) {
      v4 = sub_8f90(0,v27,a1[optind]);
      if (v6 < v4)
        v6 = v4;
    }
  }
  else {
    v27 = (long)optind;
    if (v20) { // branch-flip
      v6 = 0;
      while ((int)v27 < v21) {
        v4 = sub_8f90(0,a1[v27],v17);
        if (v6 < v4)
          v6 = v4;
        optind += 1;
        v27 = (long)optind;
      }
    }
    else {
      if (v21 - optind != 2) {
        if (2 > v21 - optind)
          sub_8e40("missing operand after \'%s\'",a1[v22 + -1]); // no-return
        sub_8e40("extra operand \'%s\'",a1[v27 + 2]); // no-return
      }
      v6 = sub_8f90(0,a1[v27],a1[v27 + 1]);
    }
  }
  sub_e670();
  sub_9df0();
  exit(v6); // no-return
label_5729:
  dat_26180 = 1;
  v6 = v4;
  goto label_5020;
}


// Function: sub_6e20 @ 0x6e20
char sub_6e20(int *a0)
{
  int *v1;
  long v10;
  void *v11;
  long v12;
  void *v13; // rax
  long v14;
  long v15;
  long *v16;
  unsigned long *v17;
  char *v18;
  long v19;
  char v2;
  long v20;
  unsigned long v21;
  void *v22;
  long v23;
  unsigned long *v24;
  long v25;
  int *v26;
  long v27;
  long v28;
  void *v29 [3]; // stack - 0x58
  unsigned long v3;
  unsigned long v30; // stack - 0x98
  char v31 [8];
  char v32 [8];
  char v33 [8];
  char v34 [8];
  unsigned long v35;
  void *v36;
  char *v37;
  void **v38;
  long v39; // r13
  unsigned long v4;
  char *v40;
  long v41;
  bool v42; // zf
  char v43;
  long v44; // stack - 0xf0
  unsigned long *v45; // stack - 0xe8
  void *v46; // stack - 0xe0
  unsigned long v47; // stack - 0x90
  long v48; // stack - 0x88
  long v49; // stack - 0x80
  unsigned long *v5;
  char v50; // stack - 0x78
  long v51; // stack - 0x70
  long v52; // stack - 0x60
  char v6;
  int v7; // eax
  unsigned long v8;
  unsigned long v9;
  
  v6 = sub_bbc0(a0,dat_26553);
  if (!v6) {
    v46 = (void *)sub_fab0(*(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4);
    *(long *)&a0[0x46] = (long)v46 + 1;
    *(long *)&a0[0x92] = (long)v46 + *(long *)&a0[0x32] + 3;
    v10 = sub_16a40((*(long *)&a0[0x32] + *(long *)&a0[0x7e]) * 0x10);
    *(long *)&a0[0x40] = v10;
    v10 += *(long *)&a0[0x32] * 8;
    *(long *)&a0[0x42] = v10;
    v10 += *(long *)&a0[0x32] * 8;
    *(long *)&a0[0x8c] = v10;
    *(long *)&a0[0x8e] = v10 + *(long *)&a0[0x7e] * 8;
    v11 = (void *)sub_fab0(*(long *)&a0[0x4a] << 4);
    v29[0] = v11;
    v22 = (void *)((long)v11 + *(long *)&a0[0x4a] * 8);
    if (1 <= *(long *)&a0[0x32]) {
      v10 = *(long *)&a0[0x3e];
      v12 = 0;
      do {
        v20 = v12 * 8;
        v12 += 1;
        v16 = (long *)((long)v11 + *(long *)(v10 + v20) * 8);
        *v16 = *v16 + 1;
      } while (v12 < *(long *)&a0[0x32]);
    }
    v10 = *(long *)&a0[0x7e];
    if (1 <= v10) {
      v12 = *(long *)&a0[0x8a];
      v20 = 0;
      do {
        v10 = v20 * 8;
        v20 += 1;
        v16 = (long *)((long)v22 + *(long *)(v12 + v10) * 8);
        *v16 = *v16 + 1;
        v10 = *(long *)&a0[0x7e];
      } while (v20 < v10);
    }
    v24 = &v28;
    v29[1] = v22;
    v13 = (void *)sub_fab0(v10 + *(long *)&a0[0x32]);
    v38 = v29;
    v8 = *(unsigned long *)&a0[0x32];
    v52 = (long)v13 + v8;
    v16 = (long *)&a0[0x3e];
    v36 = v13;
    v17 = v24;
    do {
      v10 = *v16;
      v21 = v8 >> 8;
      if (v21) { // branch-flip
        v35 = 5;
        do {
          v35 *= 2;
          v21 >>= 2;
        } while (v21);
label_7184:
        v21 = 0;
        do {
          while (v12 = *(long *)(v10 + v21 * 8), !v12) {
label_718d:
            v21 += 1;
            if (v21 == v8) goto label_71bb;
          }
          v4 = *(unsigned long *)((long)v22 + v12 * 8);
          if (!v4) {
            *(char *)((long)v36 + v21) = 1;
            goto label_718d;
          }
          if (v35 >= v4) goto label_718d;
          *(char *)((long)v36 + v21) = 2;
          v21 += 1;
        } while (v21 != v8);
      }
      else if (v8) {
        v35 = 5;
        goto label_7184;
      }
label_71bb:
      v17 = &v17[1];
      if ((long *)&a0[0xd6] == &v16[0x26]) goto label_71cf;
      v22 = *v38;
      v8 = v16[0x20];
      v38 = &v38[-1];
      v36 = (void *)*v17;
      v16 = &v16[0x26];
    } while( true );
  }
  if ((((*(long *)&a0[0x10] != *(long *)&a0[0x5c] && 1 <= *(long *)&a0[0x10]) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000)))) {
label_6f97:
    v43 = 1;
    sub_6d90(1,a0);
    v22 = *(void **)&a0[0x28];
    v11 = *(void **)&a0[0x74];
  }
  else {
    v22 = *(void **)&a0[0x28];
    if (*a0 != a0[0x4c]) { // branch-flip
      v3 = *(unsigned long *)&a0[0x5e];
      v9 = *(unsigned long *)&a0[0x12];
      v8 = sub_17190(8,sub_17190(v9,v3,0x7ffffffffffffffe),0x7ffffffffffffffe);
      v9 = sub_16a90(v22,v8);
      v3 = *(unsigned long *)&a0[0x74];
      *(unsigned long *)&a0[0x28] = v9;
      *(unsigned long *)&a0[0x74] = sub_16a90(v3,v8);
      while( true ) {
        if (0 <= *a0)
          sub_b8a0(a0,v8 - *(long *)&a0[0x2c]);
        if (0 <= a0[0x4c])
          sub_b8a0(&a0[0x4c],v8 - *(long *)&a0[0x78]);
        v21 = *(unsigned long *)&a0[0x2c];
        if (v21 != *(unsigned long *)&a0[0x78]) goto label_6f97;
        v22 = *(void **)&a0[0x28];
        v11 = *(void **)&a0[0x74];
        if (memcmp(v22,v11,v21)) goto label_6f97;
        if (v21 != v8) break;
        a0[0x78] = 0;
        a0[0x79] = 0;
        a0[0x2c] = 0;
        a0[0x2d] = 0;
      }
      v43 = 0;
    }
    else {
      v11 = *(void **)&a0[0x74];
      v43 = 0;
    }
  }
label_6fc4:
  if (v22 != v11) {
    free(v22);
    v11 = *(void **)&a0[0x74];
  }
  free(v11);
  return v43;
label_71cf:
  v44 = 0;
  v22 = v13;
  v45 = v24;
  do {
    v10 = *(long *)((long)a0 + v44 + 200);
    v12 = 0;
    if (0 < v10) {
      do {
        while( true ) {
          v18 = (char *)((long)v22 + v12);
          if (*v18 != '\x02') break;
          v12 += 1;
          *v18 = '\0';
          if (v10 <= v12) goto label_722b;
        }
        if (*v18) {
          v25 = 0;
          v20 = v12;
          while ((v20 = v20 + 1, v20 < v10 && (*(char *)((long)v22 + v20)))) {
            v25 += (unsigned long)(*(char *)((long)v22 + v20) == '\x02');
          }
          if (v20 <= v12) { // branch-flip
            v27 = v20 - v12;
            if (v25 * 4 <= v27) {
label_7380:
              v20 = v27 >> 4;
              if (v20 <= 0) goto label_74b4;
              v25 = 1;
              do {
                v25 *= 2;
                v20 >>= 2;
              } while (v20);
              v20 = v12 + -1 + v27;
label_73a3:
              v19 = 0;
              v14 = 0;
              do {
                while (v40 = (char *)(v12 + v19 + (long)v22), *v40 != '\x02') {
                  v41 = 0;
label_73ad:
                  v19 += 1;
                  v14 = v41;
                  if (v19 >= v27) goto label_73e2;
                }
                v41 = v14 + 1;
                if (v14 == v25) {
                  v19 -= v41;
                  goto label_73ad;
                }
                if (v25 >= v14) goto label_73ad;
                v19 += 1;
                *v40 = '\0';
                v14 = v41;
              } while (v27 > v19);
label_73e2:
              v12 = 0;
              v40 = v18;
              do {
                while( true ) {
                  v6 = *v40;
                  if ((8 <= (long)v40 - (long)v18) && (v6 == '\x01')) goto label_7433;
                  if (v6 != '\x02') break;
                  *v40 = '\0';
                  v40 = &v40[1];
                  v12 = 0;
                  if (&v18[v27] == v40) goto label_7433;
                }
                if (v6) { // branch-flip
                  v12 += 1;
                  if (v12 == 3) break;
                }
                else {
                  v12 = 0;
                }
                v40 = &v40[1];
              } while (&v18[v27] != v40);
label_7433:
              v18 = (char *)((long)v22 + (v20 - v27));
              v12 = 0;
              v40 = (char *)((long)v22 + v20);
              do {
                while( true ) {
                  if ((8 <= ((long)v22 + v20) - (long)v40) && (*v40 == '\x01')) goto label_748d;
                  if (*v40 != '\x02') break;
                  *v40 = '\0';
                  v40 = &v40[-1];
                  v12 = 0;
                  if (v18 == v40) goto label_748d;
                }
                if (*v40) { // branch-flip
                  v12 += 1;
                  if (v12 == 3) break;
                }
                else {
                  v12 = 0;
                }
                v40 = &v40[-1];
              } while (v18 != v40);
label_748d:
              v12 = v20; // crossjump-dupe
            }
          }
          else {
            do {
              if (*(char *)((long)v22 + v20 + -1) != '\x02') {
                v27 = v20 - v12;
                if (v25 * 4 <= v27) goto label_7380;
                if (v12 < v20) {
                  v18 = (char *)((long)v22 + v20 + -1);
                  do {
                    if (*v18 == '\x02')
                      *v18 = '\0';
                    v18 = &v18[-1];
                  } while ((char *)((long)v22 + v12 + -1) != v18);
                }
                goto label_7208;
              }
              v20 -= 1;
              v25 -= 1;
              *(char *)((long)v22 + v20) = 0;
            } while (v12 != v20);
            v27 = 0;
            if (v25 <= 0) {
label_74b4:
              v20 = v12 + -1 + v27;
              if (1 <= v27) {
                v25 = 1;
                goto label_73a3;
              }
              v12 = v20;
            }
          }
        }
label_7208:
        v12 += 1;
      } while (v12 < v10);
    }
label_722b:
    v44 += 0x130;
    v45 = &v45[1];
    if (v44 == 0x260) goto label_724b;
    v22 = (void *)*v45;
  } while( true );
label_724b:
  v16 = (long *)&a0[0x32];
  v22 = v13;
  do {
    v10 = *v16;
    v12 = 0;
    if (1 <= v10) {
      v20 = 0;
      do {
        while ((!dat_26408 && (*(char *)((long)v22 + v20)))) {
          *(char *)(v16[10] + v20) = 1;
          v20 += 1;
          if (v10 == v20) goto label_72b8;
        }
        v25 = v12 * 8;
        *(unsigned long *)(v16[7] + v12 * 8) = *(unsigned long *)(v16[6] + v20 * 8);
        v12 += 1;
        *(long *)(v16[8] + v25) = v20;
        v20 += 1;
      } while (v10 != v20);
    }
label_72b8:
    v16[9] = v12;
    v16 = &v16[0x26];
    v24 = &v24[1];
    if ((long *)&a0[0xca] == v16) goto label_74db;
    v22 = (void *)*v24;
  } while( true );
label_74db:
  free(v13);
  free(v11);
  v30 = *(unsigned long *)&a0[0x40];
  v25 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
  v47 = *(unsigned long *)&a0[0x8c];
  v20 = sub_16a40(v25 * 0x10);
  v12 = *(long *)&a0[0x90];
  v10 = v12 * 8 + 8;
  v48 = v20 + v10;
  v49 = v20 + v10 + v25 * 8;
  v50 = dat_26418;
  if (v25) { // branch-flip
    v51 = 1;
    do {
      v51 *= 2;
      v25 >>= 2;
    } while (v25);
    if (v51 <= 0xfff)
      v51 = 0x1000;
  }
  else {
    v51 = 0x1000;
  }
  v20 = 0;
  v10 = 0x26;
  v26 = a0;
  v24 = (unsigned long *)0x261a0;
  while (v10) {
    v17 = &v24[1];
    v1 = &v26[2];
    *v24 = *(unsigned long *)v26;
    v10 -= 1;
    v26 = v1;
    v24 = v17;
  }
  v10 = 0x26;
  v26 = &a0[0x4c];
  while (v10) {
    v17 = &v24[1];
    v1 = &v26[2];
    *v24 = *(unsigned long *)v26;
    v10 -= 1;
    v26 = v1;
    v24 = v17;
  }
  sub_6360(0,*(unsigned long *)&a0[0x44],0,v12,dat_26408,&v30);
  free((void *)(*(long *)&a0[0x90] * -8 + -8 + v48));
label_75e0:
  v10 = *(long *)((long)a0 + v20 + 0x118);
  v19 = 0;
  v14 = 0;
  v12 = *(long *)((long)a0 + v20 + 0xf8);
  v25 = *(long *)((long)a0 + v20 + 200);
  v27 = *(long *)((long)a0 + (0x248U - v20));
label_760f:
  v42 = v25 == v14;
  v41 = v14;
  if (v25 <= v14) goto label_7765;
label_7618:
  v23 = v41;
  if (!*(char *)(v10 + v41)) goto label_7921;
  do {
    v14 = v23;
    do {
      v14 += 1;
    } while (*(char *)(v10 + v14));
    for (; *(char *)(v27 + v19); v19 = v19 + 1) {
    }
    do {
      v39 = v14 - v23;
      while ((v23 && (*(long *)(v12 + -8 + v23 * 8) == *(long *)(v12 + -8 + v14 * 8)))) {
        v14 -= 1;
        *(char *)(v10 + v23 + -1) = 1;
        *(char *)(v10 + v14) = 0;
        v6 = *(char *)(v10 + -2 + v23);
        v23 -= 1;
        while (v6) {
          v23 -= 1;
          v6 = *(char *)(v10 + -1 + v23);
        }
        do {
          v19 -= 1;
        } while (*(char *)(v27 + v19));
      }
      v41 = v14;
      if (!*(char *)(v27 + -1 + v19))
        v41 = v25;
      while ((v14 != v25 && (*(long *)(v12 + v23 * 8) == *(long *)(v12 + v14 * 8)))) {
        v23 += 1;
        v15 = v14 + 1;
        *(char *)(v10 + -1 + v23) = 0;
        *(char *)(v10 + v14) = 1;
        if (!*(char *)(v10 + v15)) goto label_7703;
        do {
          v15 += 1;
        } while (*(char *)(v10 + v15));
        v19 += 1;
        v6 = *(char *)(v27 + v19);
        while (v6) {
          v41 = v15;
label_7703:
          v19 += 1;
          v6 = *(char *)(v27 + v19);
        }
        v14 = v15;
      }
    } while (v39 != v14 - v23);
    if (v14 <= v41) goto label_760f;
    v37 = (char *)(v10 + -1 + v14);
    do {
      v37[v23 - v14] = 1;
      *v37 = 0;
      do {
        v19 -= 1;
      } while (*(char *)(v27 + v19));
      v37 = &v37[-1];
    } while ((char *)(v10 + -1 + v41) != v37);
    v42 = v25 == v41;
    if (!v42 && v41 <= v25) goto label_7618;
label_7765:
    v23 = v41;
  } while (!v42);
  v20 += 0x130;
  if (v20 != 0x260) goto label_75e0;
  v10 = *(long *)&a0[0x46];
  v12 = *(long *)&a0[0x92];
  v8 = *(unsigned long *)&a0[0x32];
  v21 = *(unsigned long *)&a0[0x7e];
  if (dat_26580 != 4) { // branch-flip
    if ((long)(v8 & v21) < 0) {
label_7b50:
      if ((dat_26554) || (v17 = NULL, dat_264e0)) goto label_7b69;
      goto label_7c25;
    }
    v24 = NULL;
    do {
      v6 = *(char *)(v10 + -1 + v8);
      v2 = *(char *)(v12 + -1 + v21);
      v17 = v24;
      if (v6 || v2) {
        if (v6) { // branch-flip
          v35 = v8;
          do {
            v35 -= 1;
          } while (*(char *)(v10 + -1 + v35));
          v20 = v8 - v35;
          v8 = v35;
        }
        else {
          v20 = 0;
        }
        if (v2) { // branch-flip
          v35 = v21;
          do {
            v35 -= 1;
          } while (*(char *)(v12 + -1 + v35));
          v25 = v21 - v35;
          v21 = v35;
        }
        else {
          v25 = 0;
        }
        v17 = (unsigned long *)sub_16a40(0x30);
        *v17 = v24;
        v17[3] = v8;
        v17[4] = v21;
        v17[1] = v25;
        v17[2] = v20;
      }
      v21 -= 1;
      v8 -= 1;
      v24 = v17;
    } while ((0 <= (long)v8) || (0 <= (long)v21));
  }
  else {
    if (((long)v8 <= 0) && ((long)v21 <= 0)) goto label_7b50;
    v25 = 0;
    v20 = 0;
    v24 = NULL;
    do {
      v17 = v24;
      v27 = v20;
      v14 = v25;
      if (*(char *)(v10 + v20) || *(char *)(v12 + v25)) {
        if (*(char *)(v10 + v20)) { // branch-flip
          do {
            v27 += 1;
          } while (*(char *)(v10 + v27));
          v19 = v27 - v20;
        }
        else {
          v19 = 0;
        }
        if (*(char *)(v12 + v25)) { // branch-flip
          do {
            v14 += 1;
          } while (*(char *)(v12 + v14));
          v41 = v14 - v25;
        }
        else {
          v41 = 0;
        }
        v17 = (unsigned long *)sub_16a40(0x30);
        v17[3] = v20;
        v17[4] = v25;
        *v17 = v24;
        v17[1] = v41;
        v17[2] = v19;
      }
      v20 = v27 + 1;
      v25 = v14 + 1;
      v24 = v17;
    } while ((v20 < (long)v8) || (v25 < (long)v21));
  }
  if ((!dat_26554) && (!dat_264e0)) {
label_7c25:
    v43 = v17 != NULL;
label_786c:
    if (dat_264a1) { // branch-flip
      if ((bool)v43)
        sub_6d90(1,a0);
    }
    else {
      if (!(bool)v43) {
label_7b7c:
        v43 = 0;
        if (dat_26578) goto label_788b;
      }
      v10 = dat_26548;
      if (!dat_26548)
        v10 = *(long *)&a0[0x4e];
      v12 = dat_26540;
      if (!dat_26540)
        v12 = *(long *)&a0[2];
      sub_e800(v12,v10,*(long *)&a0[0x98] != 0);
      switch(dat_26580) {
        default:
          abort(); // no-return
        case 1:
          sub_cff0(v17);
          break;
        case 2:
          sub_8c10(v17,0);
          break;
        case 3:
          sub_8c10(v17,1);
          break;
        case 4:
          sub_ac20(v17);
          break;
        case 5:
          sub_ac40(v17);
          break;
        case 6:
          sub_ac60(v17);
          break;
        case 7:
          sub_b6c0(v17);
          break;
        case 8:
          sub_da50(v17);
        
      }
      sub_ead0();
    }
label_788b:
    free(*(void **)&a0[0x40]);
    free(v46);
    free(*(void **)&a0[0x3e]);
    free((void *)(*(long *)&a0[0x2e] + *(long *)&a0[0x30] * 8));
    free(*(void **)&a0[0x8a]);
    free((void *)(*(long *)&a0[0x7a] + *(long *)&a0[0x7c] * 8));
    while (v17) {
      v24 = (unsigned long *)*v17;
      free(v17);
      v17 = v24;
    }
    if ((unsigned int)(dat_26580 - 4U) < 2) {
      v16 = (long *)0x26540;
      v26 = &a0[0x48];
      do {
        if ((char)*v26) {
          v18 = dcgettext(NULL,"No newline at end of file",5);
          v10 = *v16;
          if (!v10)
            v10 = *(long *)&v26[-0x46];
          error(0,0,"%s: %s\n",v10,v18);
          v43 = 2;
        }
        v16 = &v16[1];
        v26 = &v26[0x4c];
      } while (v16 != (long *)0x26550);
    }
    v22 = *(void **)&a0[0x28];
    v11 = *(void **)&a0[0x74];
    goto label_6fc4;
  }
  if (!v17) {
label_7b69:
    v17 = NULL;
    v43 = 0;
    if (!dat_264a1) goto label_7b7c;
    goto label_788b;
  }
  v24 = v17;
  do {
    v16 = (long *)sub_f060(v24);
    v5 = (unsigned long *)*v16;
    *v16 = 0;
    v7 = sub_f670(v24,v31,v34,v33,v32);
    *v16 = (long)v5;
    if (!v5) break;
    v24 = v5;
  } while (!v7);
  v43 = v7 != 0;
  goto label_786c;
label_7921:
  do {
    v19 += 1;
  } while (*(char *)(v27 + -1 + v19));
  v14 = v41 + 1;
  goto label_760f;
}


// Function: sub_7cf0 @ 0x7cf0
void sub_7cf0(unsigned long a0,long a1,unsigned long a2,long a3)
{
  char *v1;
  unsigned long v2;
  void *v3; // rax
  long v4; // rax
  char v5 [56];
  
  sub_f510(0);
  if (a3) // branch-flip
    __fprintf_chk(dat_26188,1,"%s %s",a0,a3);
  else {
    v3 = localtime((void *)(a1 + 0x68));
    v2 = *(unsigned long *)(a1 + 0x70);
    if ((!v3) || (v4 = sub_14460(v5,0x2b,dat_26400,v3,0,v2 & 0xffffffff), !v4))
      __sprintf_chk(v5,1,0x2b,"%ld.%.9d",*(unsigned long *)(a1 + 0x68),v2 & 0xffffffff);
    __fprintf_chk(dat_26188,1,"%s %s\t%s",a0,a2,v5);
  }
  sub_f510(3);
  v1 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v1) {
    __overflow(dat_26188,10); // tail-call
    return;
  }
  *(char **)((long)dat_26188 + 0x28) = &v1[1];
  *v1 = 10;
}


// Function: sub_7e50 @ 0x7e50
unsigned long sub_7e50(long a0,long a1)
{
  long v1;
  unsigned long v2; // r8
  unsigned long v3;
  
  v1 = dat_26178;
  dat_26178 = a1;
  do {
    a1 -= 1;
    if (a1 < v1) {
      v3 = 0;
      if (dat_26170 != 0x7fffffffffffffff)
        v3 = *(unsigned long *)(a0 + dat_26170 * 8);
      return v3;
    }
    v3 = *(unsigned long *)(a0 + a1 * 8);
    v2 = ~v3 + *(long *)(a0 + 8 + a1 * 8);
    if (0x80000000 <= v2)
      v2 = 0x7fffffff;
  } while ((int)re_search(0x26500,v3,v2 & 0xffffffff,0,v2,0) <= -1);
  dat_26170 = a1;
  return v3;
}


// Function: sub_7f00 @ 0x7f00
void sub_7f00(void *a0,char *a1)
{
  char v1;
  char *v2;
  int v3; // ecx
  int v4;
  char *v5;
  char *v6;
  
  v2 = *(char **)((long)a0 + 0x28);
  if (*(char **)((long)a0 + 0x30) <= v2) // branch-flip
    __overflow(a0,0x20);
  else {
    *(char **)((long)a0 + 0x28) = &v2[1];
    *v2 = 0x20;
  }
  v3 = 0;
  v5 = a1;
  do {
    v1 = *v5;
    if ('\x0e' <= v1) { // branch-flip
      if (v1 != ' ') goto label_7f3d;
    }
    else if ((v1 <= '\b') || (v1 == '\n')) goto label_7f3d;
    v5 = &v5[1];
    v3 += 1;
  } while( true );
label_7f3d:
  v6 = &a1[(long)v3 + 1];
  v4 = v3;
  while ((v1 != '\n' && (v4 = v4 + 1, v4 <= v3 + 0x27))) {
    v1 = *v6;
    v6 = &v6[1];
  }
  if (v3 < v4) {
    a1 = &a1[(long)v4 + -1];
    do {
      v1 = *a1;
      if ('\x0e' <= v1) { // branch-flip
        if (v1 != ' ') break;
      }
      else if (v1 <= '\b') break;
      v4 -= 1;
      a1 = &a1[-1];
    } while (v3 != v4);
  }
  fwrite_unlocked(v5,1,(long)(v4 - v3),a0); // tail-call
}


// Function: sub_7fe0 @ 0x7fe0
void sub_7fe0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  long v1; // stack - 0x20
  long v2; // stack - 0x18
  
  sub_f5e0(a0,a1,a2,&v1,&v2);
  if (v2 <= v1) {
    __fprintf_chk(dat_26188,1,"%ld",v2);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld");
}


// Function: sub_8070 @ 0x8070
void sub_8070(unsigned long *a0)
{
  char *v1;
  char *v10;
  unsigned long *v11;
  bool v2;
  unsigned int v3; // eax
  long v4;
  long v5; // stack - 0x50
  long v6; // stack - 0x58
  long v7; // stack - 0x60
  long v8; // stack - 0x48
  void *v9;
  
  v3 = sub_f670(a0,&v7,&v6,&v5,&v8);
  if (v3) {
    v4 = -dat_26288;
    v7 -= dat_26570;
    if (v7 < v4)
      v7 = v4;
    v5 -= dat_26570;
    if (v5 < v4)
      v5 = v4;
    v4 = dat_26570 + v6;
    v2 = v6 < dat_26270 - dat_26570;
    v6 = dat_26270 + -1;
    if (v2)
      v6 = v4;
    v4 = dat_26570 + v8;
    v2 = v8 < dat_263a0 - dat_26570;
    v8 = dat_263a0 + -1;
    if (v2)
      v8 = v4;
    if (dat_26520) { // branch-flip
      v4 = sub_7e50(dat_26258);
      sub_e830();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
      if (v4)
        sub_7f00(v9,v4);
    }
    else {
      sub_e830();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
    }
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    sub_f510(4);
    fputs_unlocked("*** ",v9);
    sub_7fe0(0x261a0,v7,v6);
    fputs_unlocked(" ****",v9);
    sub_f510(3);
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    if ((v3 & 1) && (v7 <= v6)) {
      v11 = a0;
      v4 = v7;
      do {
        sub_f510(2);
        for (; v11; v11 = (unsigned long *)*v11) {
          if (v4 < (long)(v11[2] + v11[3])) {
            if ((long)v11[3] <= v4) {
              v10 = "-";
              if (1 <= (long)v11[1])
                v10 = "!";
              goto label_8274;
            }
            break;
          }
        }
        v10 = " ";
label_8274:
        sub_f330(v10,dat_26258 + v4 * 8,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26258 + 8 + v4 * 8) + -1) == '\n') {
          v1 = *(char **)((long)v9 + 0x28);
          if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
            __overflow(v9,10);
          else {
            *(char **)((long)v9 + 0x28) = &v1[1];
            *v1 = 10;
          }
        }
        v4 += 1;
      } while (v4 <= v6);
    }
    sub_f510(4);
    fputs_unlocked("--- ",v9);
    sub_7fe0(0x262d0,v5,v8);
    fputs_unlocked(" ----",v9);
    sub_f510(3);
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    if ((v3 & 2) && (v5 <= v8)) {
      v4 = v5;
      do {
        sub_f510(1);
        for (; a0; a0 = (unsigned long *)*a0) {
          if (v4 < (long)(a0[1] + a0[4])) {
            if ((long)a0[4] <= v4) {
              v10 = "+";
              if (1 <= (long)a0[2])
                v10 = "!";
              goto label_839d;
            }
            break;
          }
        }
        v10 = " ";
label_839d:
        sub_f330(v10,dat_26388 + v4 * 8,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26388 + 8 + v4 * 8) + -1) == '\n') {
          v1 = *(char **)((long)v9 + 0x28);
          if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
            __overflow(v9,10);
          else {
            *(char **)((long)v9 + 0x28) = &v1[1];
            *v1 = 10;
          }
        }
        v4 += 1;
      } while (v4 <= v8);
    }
  }
}


// Function: sub_84f0 @ 0x84f0
void sub_84f0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char *v1; // rdx
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  sub_f5e0(a0,a1,a2,&v2,&v3);
  if (v3 <= v2) {
    v1 = "%ld,0";
    if (v2 <= v3)
      v1 = "%ld";
    __fprintf_chk(dat_26188,1,v1);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld",v2,(v3 - v2) + 1);
}


// Function: sub_8590 @ 0x8590
void sub_8590(unsigned long *a0) // return-dupe
{
  long v1;
  long v10; // stack - 0x48
  unsigned long *v11;
  int v12; // esi
  long v13;
  void *v14;
  long v15; // stack - 0x78
  char *v2;
  bool v3;
  long v4;
  long v5;
  long v6;
  long v7; // stack - 0x50
  long v8; // stack - 0x58
  long v9; // stack - 0x60
  
  if (!sub_f670(a0,&v9,&v8,&v7,&v10))
    return;
  v9 -= dat_26570;
  v5 = -dat_26288;
  if (v9 < v5)
    v9 = v5;
  v7 -= dat_26570;
  if (v7 < v5)
    v7 = v5;
  v5 = dat_26570 + v8;
  v3 = v8 < dat_26270 - dat_26570;
  v8 = dat_26270 + -1;
  if (v3)
    v8 = v5;
  v5 = dat_26570 + v10;
  v3 = v10 < dat_263a0 - dat_26570;
  v10 = dat_263a0 + -1;
  if (v3)
    v10 = v5;
  if (dat_26520) { // branch-flip
    v5 = sub_7e50(dat_26258);
    sub_e830();
    v14 = dat_26188;
    sub_f510(4);
    fputs_unlocked("@@ -",v14);
    sub_84f0(0x261a0,v9,v8);
    fputs_unlocked(" +",v14);
    sub_84f0(0x262d0,v7,v10);
    fputs_unlocked(" @@",v14);
    sub_f510(3);
    if (v5)
      sub_7f00(v14,v5);
  }
  else {
    sub_e830();
    v14 = dat_26188;
    sub_f510(4);
    fputs_unlocked("@@ -",v14);
    sub_84f0(0x261a0,v9,v8);
    fputs_unlocked(" +",v14);
    sub_84f0(0x262d0,v7,v10);
    fputs_unlocked(" @@",v14);
    sub_f510(3);
  }
  v2 = *(char **)((long)v14 + 0x28);
  if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
    __overflow(v14,10);
  else {
    *(char **)((long)v14 + 0x28) = &v2[1];
    *v2 = 10;
  }
  v15 = v7;
  v5 = v9;
  do {
    if ((v8 < v5) && (v10 < v15))
      return;
    while ((!a0 || (v5 < (long)a0[3]))) {
      v1 = v5 + 1;
      v11 = (unsigned long *)(dat_26258 + v5 * 8);
      if ((!dat_26491) || (*(char *)*v11 != '\n')) {
        v2 = *(char **)((long)v14 + 0x28);
        v12 = (-(unsigned int)(dat_26492 == '\0') & 0x17) + 9;
        if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
          __overflow(v14,v12);
        else {
          *(char **)((long)v14 + 0x28) = &v2[1];
          *v2 = (char)v12;
        }
      }
      sub_f500(0,v11);
      v15 += 1;
      v5 = v1;
      if ((v8 < v1) && (v10 < v15))
        return;
    }
    v1 = a0[2];
    if (v1) {
      v13 = v5 * 8;
      v6 = v1;
      do {
        while( true ) {
          v4 = dat_26258;
          sub_f510(2);
          v2 = *(char **)((long)v14 + 0x28);
          v11 = (unsigned long *)(v4 + v13);
          if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
            __overflow(v14,0x2d);
          else {
            *(char **)((long)v14 + 0x28) = &v2[1];
            *v2 = 0x2d;
          }
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n')))) {
            v2 = *(char **)((long)v14 + 0x28);
            if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
              __overflow(v14,9);
            else {
              *(char **)((long)v14 + 0x28) = &v2[1];
              *v2 = 9;
            }
          }
          sub_f330(0,v11,1);
          sub_f510(3);
          if (*(char *)(v11[1] + -1) == '\n') break;
label_8880:
          v13 += 8;
          v6 -= 1;
          if (!v6) goto label_893d;
        }
        v2 = *(char **)((long)v14 + 0x28);
        if (*(char **)((long)v14 + 0x30) <= v2) {
          __overflow(v14,10);
          goto label_8880;
        }
        v13 += 8;
        *(char **)((long)v14 + 0x28) = &v2[1];
        *v2 = 10;
        v6 -= 1;
      } while (v6);
label_893d:
      v5 += v1;
    }
    v1 = a0[1];
    if (v1) {
      v13 = v15 * 8;
      v6 = v1;
      do {
        while( true ) {
          v4 = dat_26388;
          sub_f510(1);
          v2 = *(char **)((long)v14 + 0x28);
          v11 = (unsigned long *)(v4 + v13);
          if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
            __overflow(v14,0x2b);
          else {
            *(char **)((long)v14 + 0x28) = &v2[1];
            *v2 = 0x2b;
          }
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n')))) {
            v2 = *(char **)((long)v14 + 0x28);
            if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
              __overflow(v14,9);
            else {
              *(char **)((long)v14 + 0x28) = &v2[1];
              *v2 = 9;
            }
          }
          sub_f330(0,v11,1);
          sub_f510(3);
          if (*(char *)(v11[1] + -1) == '\n') break;
label_8970:
          v13 += 8;
          v6 -= 1;
          if (!v6) goto label_8a2d;
        }
        v2 = *(char **)((long)v14 + 0x28);
        if (*(char **)((long)v14 + 0x30) <= v2) {
          __overflow(v14,10);
          goto label_8970;
        }
        v13 += 8;
        *(char **)((long)v14 + 0x28) = &v2[1];
        *v2 = 10;
        v6 -= 1;
      } while (v6);
label_8a2d:
      v15 += v1;
    }
    a0 = (unsigned long *)*a0;
  } while( true );
}


// Function: sub_8b10 @ 0x8b10
long * sub_8b10(long *a0)
{
  long *v1;
  long v2; // rax
  long v3;
  long v4;
  long v5;
  
  v3 = a0[4];
  v4 = a0[3];
  while( true ) {
    v1 = (long *)*a0;
    if (!v1)
      return a0;
    v5 = dat_26570 * 2 + 1;
    if (*(char *)&v1[5])
      v5 = dat_26570;
    v2 = v3 + a0[1];
    v4 = v1[3] - (v4 + a0[2]);
    v3 = v1[4];
    if (v4 != v3 - v2) break;
    if (v5 <= v4)
      return a0;
    v4 = v1[3];
    a0 = v1;
  }
  abort(); // no-return
}


// Function: sub_8b80 @ 0x8b80
void sub_8b80(long a0,unsigned long *a1,char a2)
{
  if (a2) {
    sub_7cf0("---",a0,*a1,dat_26540);
    sub_7cf0("+++",a0 + 0x130,a1[1],dat_26548); // tail-call
    return;
  }
  sub_7cf0("***",a0,*a1,dat_26540);
  sub_7cf0("---",a0 + 0x130,a1[1],dat_26548); // tail-call
}


// Function: sub_8c10 @ 0x8c10
void sub_8c10(unsigned long *a0,char a1) // ternary
{
  unsigned long *v1;
  int v2; // eax
  void *v3; // rdx
  unsigned long *v4;
  char v5 [8];
  char v6 [8];
  char v7 [8];
  char v8 [8];
  
  if ((dat_26554) || (v4 = a0, dat_264e0)) {
    if (a0) {
      v4 = a0;
      do {
        v1 = (unsigned long *)*v4;
        *v4 = 0;
        v2 = sub_f670(v4,v8,v7,v6,v5);
        *v4 = v1;
        *(bool *)&v4[5] = v2 == 0;
        v4 = v1;
      } while (v1);
    }
  }
  else {
    for (; v4; v4 = (unsigned long *)*v4) {
      *(char *)&v4[5] = 0;
      v4 = (unsigned long *)*v4;
      if (!v4) break;
      *(char *)&v4[5] = 0;
    }
  }
  dat_26178 = -dat_26288;
  dat_26170 = 0x7fffffffffffffff;
  v3 = (a1) ? sub_8590 : sub_8070; // branch-flip
  sub_f080(a0,sub_8b10,v3); // tail-call
}


// Function: sub_8d60 @ 0x8d60
void sub_8d60(long *a0,char *a1)
{
  unsigned long v1; // rax
  long v2;
  long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  long v6;
  
  v1 = strlen(a1);
  v2 = re_compile_pattern(a1,v1,a0[4]);
  if (v2) {
    error(2,0,"%s: %s",a1,v2); // tail-call
    return;
  }
  v2 = *a0;
  v6 = a0[1];
  v5 = a0[2];
  *(bool *)&a0[3] = v2 != 0;
  v4 = (unsigned long)((unsigned int)(v2 != 0) * 2) + v6 + v1;
  a0[1] = v4;
  v3 = v2;
  if (v5 <= v4) {
    if (!v5)
      v5 = 1;
    do {
      v5 *= 2;
    } while (v5 <= v4);
    a0[2] = v5;
    v3 = sub_16a90(v2);
    *a0 = v3;
  }
  if (v2) {
    v2 = v6 + 1;
    *(char *)(v3 + v6) = 0x5c;
    v6 += 2;
    *(char *)(v3 + v2) = 0x7c;
  }
  memcpy((void *)(v3 + v6),a1,v1 + 1); // tail-call
}


// Function: sub_8e40 @ 0x8e40
void sub_8e40(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_36860;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_8f90 @ 0x8f90
unsigned int sub_8f90(long a0,char *a1,char *a2)
{
  long *v1;
  int v10 [2]; // stack - 0x2a8
  long v11; // stack - 0x240
  long v12 [3]; // stack - 0x298
  long v13 [3]; // stack - 0x168
  int *v14;
  char *v15;
  long *v16;
  unsigned int v17; // r9d
  char *v18;
  int v19; // ebx
  unsigned long v2;
  bool v20;
  bool v21;
  char *v22; // stack - 0x2c0
  char *v23; // stack - 0x2a0
  unsigned int v24; // stack - 0x280
  int v25; // stack - 0x27c
  unsigned int v26; // stack - 0x278
  long v27; // stack - 0x270
  long v28; // stack - 0x268
  unsigned long v29; // stack - 0x260
  unsigned long v3;
  unsigned long v30; // stack - 0x258
  unsigned long v31; // stack - 0x250
  unsigned long v32; // stack - 0x248
  unsigned long v33; // stack - 0x238
  long v34; // stack - 0x230
  unsigned long v35; // stack - 0x228
  unsigned long v36; // stack - 0x220
  unsigned long v37; // stack - 0x218
  unsigned long v38; // stack - 0x210
  int v39; // stack - 0x178
  unsigned char v4; // al
  char *v40; // stack - 0x170
  unsigned long v41; // stack - 0x150
  unsigned long v42; // stack - 0x148
  long v43; // stack - 0x140
  long v44; // stack - 0x138
  unsigned long v45; // stack - 0x130
  unsigned long v46; // stack - 0x128
  unsigned long v47; // stack - 0x120
  unsigned long v48; // stack - 0x118
  long v49; // stack - 0x110
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x108
  long v51; // stack - 0x100
  unsigned long v52; // stack - 0xf8
  unsigned long v53; // stack - 0xf0
  unsigned long v54; // stack - 0xe8
  unsigned long v55; // stack - 0xe0
  long v56; // stack - 0x48
  int *v6;
  long v7; // rcx
  int v8;
  char *v9;
  
  if ((!a1) || (!a2)) {
    if ((dat_26585 == '\x01') && (a2)) {
      v56 = a0;
      v7 = 0x4c;
      v6 = v10;
      while (v7) {
        v14 = &v6[2];
        v6[0] = 0;
        v6[1] = 0;
        v7 -= 1;
        v6 = v14;
      }
      if (a1) { // branch-flip
        v10[0] = -2;
        v39 = 0xfffffffe;
      }
      else {
        v10[0] = -1;
label_9698:
        v10[0] = -1;
        v39 = -2;
        if (!a2) goto label_9688;
      }
      v39 = -2;
      a1 = a2;
      goto label_9070;
    }
    if (!dat_26586) {
      v18 = a1;
      if (!a1)
        v18 = a2;
      v6 = (unsigned long)(unsigned long)(a1 == NULL);
      v6 = (long)((long)v6 * 0x130);
      sub_e660("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),v18);
      v5 = 1;
      return v5;
    }
    v56 = a0;
    v7 = 0x4c;
    v6 = v10;
    while (v7) {
      v14 = &v6[2];
      v6[0] = 0;
      v6[1] = 0;
      v7 -= 1;
      v6 = v14;
    }
    if (!a1) {
      v10[0] = -1;
      if (a2) goto label_9698;
      v39 = -1;
label_9688:
      v10[0] = -1;
      a1 = NULL;
      goto label_9070;
    }
    v10[0] = -2;
    if (!a2) {
      v39 = -1;
      a2 = a1;
      goto label_9070;
    }
  }
  else {
    v56 = a0;
    v7 = 0x4c;
    v6 = v10;
    while (v7) {
      v14 = &v6[2];
      v6[0] = 0;
      v6[1] = 0;
      v7 -= 1;
      v6 = v14;
    }
    v10[0] = -2;
  }
  v10[0] = -2;
  v39 = -2;
label_9070:
  v56 = a0;
  if (a0) { // branch-flip
    v6 = (char *)sub_10d80(*(unsigned long *)(a0 + 8),a1,0);
    v18 = (char *)v6;
    v23 = (char *)v6;
    v6 = (char *)sub_10d80(*(unsigned long *)(a0 + 0x138),a2,0);
    v22 = (char *)v6;
  }
  else {
    v23 = a1;
    v18 = NULL;
    v22 = NULL;
    v6 = (int *)a2;
  }
  v40 = (char *)v6;
  v16 = &v11;
  v20 = 0;
  do {
    if ((int)v16[-0xd] != -1) {
      if ((!v20) || (v5 = strcmp(v40,v23), v5)) {
        v9 = (char *)v16[-0xc];
        v5 = strcmp(v9,"-");
        v1 = &v16[-0xb];
        if (v5) { // branch-flip
          if (dat_26550) // branch-flip
            v5 = lstat(v9,v1);
          else {
            v5 = stat(v9,v1);
          }
          if (v5) {
label_9390:
            v6 = __errno_location();
            v5 = 0xfffffffd - *v6;
            *(unsigned int *)&v16[-0xd] = v5;
          }
        }
        else {
          *(unsigned int *)&v16[-0xd] = 0;
          isatty(0);
          v5 = fstat(0,v1);
          if (v5) goto label_9390;
          v5 = *(unsigned int *)&v16[-8] & 0xf000;
          if (v5 == 0x8000) {
            v6 = (long)lseek(0,0,1);
            if (0 <= (long)v6) { // branch-flip
              v16[-5] = v16[-5] - (long)v6;
              v6 = (int *)v16[-5];
              if (v16[-5] <= -1)
                v6 = (int *)0;
              v16[-5] = (long)v6;
            }
            else {
              v6 = __errno_location();
              v5 = 0xfffffffd - *v6;
              *(unsigned int *)&v16[-0xd] = v5;
            }
          }
          sub_10eb0(v16);
        }
      }
      else {
        v41 = CONCAT44(v25,v24);
        v42 = (unsigned long)v26;
        v13[0] = v12[0];
        v13[1] = v12[1];
        v39 = v10[0];
        v13[2] = v12[2];
        v43 = v27;
        v44 = v28;
        v45 = v29;
        v46 = v30;
        v47 = v31;
        v48 = v32;
        v49 = v11;
        v50 = v33;
        v51 = v34;
        v52 = v35;
        v53 = v36;
        v54 = v37;
        v55 = v38;
      }
    }
    v16 = &v16[0x26];
    if (v20) goto label_9188;
    v20 = 1;
  } while( true );
label_9188:
  v19 = 0;
  v6 = v10;
  v14 = v10;
label_91a0:
  v19 += 1;
  if (!dat_26586) {
    if (v19 != 1) goto label_91b7;
    if (dat_26585) goto label_92e0;
    goto label_932a;
  }
label_92e0:
  v8 = *v6;
  if (v8 != -2) { // branch-flip
    if (((v8 == -5 || v8 == -0xc) && (!a0)) && (!(v14[0x4c] + 2U & 0xfffffffd))) {
label_931a:
      *v6 = -1;
      if (v19 != 2) goto label_932a;
label_91c1:
      if (v10[0] != -1) {
        v8 = 0xfffffffd - v10[0];
        if (v39 != -1) { // branch-flip
          if (v8 < 0) goto label_93d7;
        }
        else {
          v7 = 0x12;
          v16 = v13;
          while (v7) {
            v1 = &v16[1];
            *v16 = 0;
            v7 -= 1;
            v16 = v1;
          }
          v41 = CONCAT44(v41._4_4_,v24);
          if (v8 <= -1) goto label_93e5;
        }
        v6 = __errno_location();
        *v6 = v8;
        sub_e490(v23);
        v8 = 0xfffffffd - v39;
        if (0 <= v8) {
label_9219:
          *v6 = v8;
          sub_e490(v40);
        }
        goto label_9228;
      }
      v7 = 0x12;
      v16 = v12;
      while (v7) {
        v1 = &v16[1];
        *v16 = 0;
        v7 -= 1;
        v16 = v1;
      }
      v24 = (unsigned int)v41;
      if (v39 != -1) { // branch-flip
label_93d7:
        v5 = 0xfffffffd - v39;
        v8 = v5;
        if (0 <= (int)v5) {
          v6 = __errno_location();
          goto label_9219;
        }
      }
      else {
        v7 = 0x12;
        v16 = v13;
        while (v7) {
          v1 = &v16[1];
          *v16 = 0;
          v7 -= 1;
          v16 = v1;
        }
      }
label_93e5:
      v8 = v10[0];
      if (!a0) {
        v5 = v24 & 0xf000;
        v20 = v5 == 0x4000;
        if (v20 != (((unsigned int)v41 & 0xf000) == 0x4000)) {
          v6 = (unsigned long)(unsigned long)v20;
          v2 = (unsigned long)v6;
          v5 = (unsigned int)!v20;
          v7 = (long)(int)v5;
          v9 = (char *)v12[v2 * 0x26 + -1];
          v6 = (unsigned long)sub_fb40(v9);
          v6 = (char *)sub_a660(v12[v7 * 0x26 + -1],v6);
          v18 = (char *)v6;
          v12[v7 * 0x26 + -1] = (long)v6;
          v5 = strcmp(v9,"-");
          if (!v5)
            sub_e7b0("cannot compare \'-\' to a directory"); // no-return
          if (dat_26550) // branch-flip
            v5 = lstat(v18,&v12[v7 * 0x26]);
          else {
            v5 = stat(v18,&v12[v7 * 0x26]);
          }
          v4 = v5 != 0;
          v8 = v10[0];
          if ((bool)v4) {
            sub_e490(v18);
            goto label_9228;
          }
        }
      }
      if (v8 != -1) { // branch-flip
        if (v39 != -1) { // branch-flip
          if ((v12[1] == v13[1]) && (v12[0] == v13[0])) {
label_984b:
            if (v24 == (unsigned int)v41) {
              if (((((v12[2] != v13[2]) || (v25 != v41._4_4_)) || (v26 != (unsigned int)v42)) || ((v28 != v44 || (v11 != v49)))) || (v34 != v51)) {
                v20 = 0;
                v5 = (unsigned int)v41 & 0xf000;
              }
              else {
                if (dat_26578) goto label_9730;
                v20 = 1;
                v5 = (unsigned int)v41 & 0xf000;
              }
              goto label_96ca;
            }
            goto label_96c0;
          }
          v5 = v24 & 0xf000;
          v17 = (unsigned int)v41 & 0xf000;
          if (v5 != 0x6000) { // branch-flip
            v20 = 0;
            if (v5 != 0x2000) goto label_96ca;
            if (v17 == 0x2000) goto label_990d;
          }
          else if (v17 == 0x6000) {
label_990d:
            if (v43 == v27) goto label_984b;
          }
          if ((v5 == 0x4000) || (v20 = 0, v17 == 0x4000)) {
label_9940:
            if (v39 == -1) goto label_9949;
label_98a0:
            v19 = 1;
            v6 = (unsigned long)sub_10c90(v13);
            v3 = v6;
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v6 = (unsigned long)sub_10c90(v12);
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v23;
            sub_e4c0("File %s is a %s while file %s is a %s\n",v15,v6,v9,v3);
            goto label_9228;
          }
label_955c:
          v17 = (unsigned int)v41 & 0xf000;
          if ((a0) && (((v5 & 0xffffdfff) != 0x8000 || (((unsigned int)v41 & 0xd000) != 0x8000)))) {
            if ((v8 == -1) || (v39 == -1)) goto label_9598;
            goto label_98a0;
          }
          if ((v5 == 0xa000) || (v17 == 0xa000)) {
            if (!dat_26550)
              __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
            if ((v5 != 0xa000) || (v17 != 0xa000)) goto label_98a0;
            v6 = (char *)sub_16f20(v23);
            v9 = (char *)v6;
            if (v6) { // branch-flip
              v6 = (char *)sub_16f20(v40);
              v15 = (char *)v6;
              v8 = 1;
              if (v6) {
                v5 = strcmp(v9,(char *)v6);
                if (!v5) {
                  free(v9);
                  free(v15);
                  goto label_9730;
                }
                v19 = 1;
                sub_e660("Symbolic links %s and %s differ\n",v23,v40);
                free(v9);
                free(v15);
                goto label_9228;
              }
            }
            else {
              v8 = 0;
            }
            v6 = (long)(long)v8;
            sub_e490(v12[(long)v6 * 0x26 + -1]);
            free(v9);
            free(NULL);
            goto label_9228;
          }
          v4 = v5 == 0x8000 & dat_26553;
          if ((((v4) && (v17 == 0x8000)) && (v28 != v44 && 1 <= v28)) && (1 <= v44)) {
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v23;
            v19 = 1;
            sub_e660("Files %s and %s differ\n",v15,v9);
            goto label_9228;
          }
          if (v8 == -2) {
            v5 = open(v23,0,0);
            v10[0] = v5;
            if (0 <= (int)v5) goto label_9a07;
            sub_e490(v23);
            if (v39 == -2) {
              v21 = 1;
              goto label_9a17;
            }
label_9a32:
            if (0 <= v10[0]) {
              v8 = 2;
              goto label_9b75;
            }
            if (v39 < 0) goto label_9228;
label_9a4d:
            v5 = close(v39);
            if (v5) {
label_9a5a:
              sub_e490(v40);
            }
            goto label_9228;
          }
label_9a07:
          v21 = 0;
          if (v39 == -2) {
label_9a17:
            if (v20)
              v39 = v10[0];
            else {
              v5 = open(v40,0,0);
              v39 = v5;
              if ((int)v5 <= -1) {
                sub_e490(v40);
                goto label_9a32;
              }
            }
            if (v21) goto label_9a32;
          }
          v5 = sub_6e20(v10);
          v8 = v5;
          if (0 <= v10[0]) {
label_9b75:
            v5 = close(v10[0]);
            if (v5) {
              sub_e490(v23);
              if ((v39 <= -1) || (v10[0] == v39)) goto label_9228;
              goto label_9a4d;
            }
          }
          v5 = v8;
          if ((0 <= v39) && (v39 != v10[0])) {
            v5 = close(v39);
            v5 = v8;
            if (v5) goto label_9a5a;
          }
label_9728:
          v19 = v5;
          if (!v19) goto label_9730;
        }
        else {
label_96c0:
          v20 = 0;
          v5 = v24 & 0xf000;
label_96ca:
          v21 = ((unsigned int)v41 & 0xf000) != 0x4000;
          if (v5 == 0x4000) {
            if (v21) goto label_9931;
            if (dat_26580 == 7)
              sub_e7b0("-D option not supported with directories"); // no-return
            if ((a0) && (!dat_26608)) {
              sub_e660("Common subdirectories: %s and %s\n",v23,v40);
              goto label_9730;
            }
label_9717:
            v5 = sub_a230(v10,sub_8f90);
            goto label_9728;
          }
          if (v21) goto label_955c;
label_9931:
          if (v8 != -1) goto label_9940;
label_9949:
          if ((dat_26608) && ((dat_26586 || ((dat_26585 && (v8 == -1)))))) goto label_9717;
          if (!a0)
            __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
label_9598:
          v19 = 1;
          v6 = (unsigned long)(unsigned long)(v8 == -1);
          v6 = (long)((long)v6 * 0x130);
          sub_e660("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),a1);
        }
label_9228:
        v5 = fflush_unlocked(stdout);
        if (v5) {
          v6 = (char *)dcgettext(NULL,"standard output",5);
          sub_e770(v6); // no-return
        }
      }
      else {
        if (v39 != -1) {
          v20 = 0;
          v5 = v24 & 0xf000;
          goto label_96ca;
        }
label_9730:
        v19 = 0;
        if ((dat_26584) && (v5 = v24 & 0xf000, v5 != 0x4000)) {
          v9 = dat_26548;
          if (!dat_26548)
            v9 = v40;
          v15 = dat_26540;
          if (!dat_26540)
            v15 = v23;
          sub_e660("Files %s and %s are identical\n",v15,v9);
        }
      }
      free(v18);
      free(v22);
      v5 = v19;
      return v5;
    }
  }
  else if ((((v6[10] & 0xf000U) == 0x8000) && (!(v6[10] & 0x1ffU))) && (!*(long *)&v6[0x10])) goto label_931a;
label_91b7:
  if (v19 == 2) goto label_91c1;
label_932a:
  v6 = &v6[0x4c];
  v14 = &v14[-0x4c];
  goto label_91a0;
}


// Function: sub_9e40 @ 0x9e40
unsigned long sub_9e40(long a0,int a1) // return-dupe x2
{
  long v1;
  long v2; // rcx
  unsigned int v3; // edx
  
  v1 = *(long *)(a0 + 0x260);
  if (!v1)
    return 0;
  v2 = (long)a1 * 0x130;
  a0 += v2;
  do {
    if ((*(long *)(v1 + v2 + 0x18) == *(long *)(a0 + 0x18)) && (*(long *)(v1 + v2 + 0x10) == *(long *)(a0 + 0x10)))
      return 1;
    v3 = *(unsigned int *)(v1 + 0x28 + v2) & 0xf000;
    if (v3 != 0x6000) { // branch-flip
      if ((v3 != 0x2000) || ((*(unsigned int *)(a0 + 0x28) & 0xf000) != 0x2000)) goto label_9e8a;
      if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
        return 1;
label_9ecc:
      v1 = *(long *)(v1 + 0x260);
    }
    else {
      if ((*(unsigned int *)(a0 + 0x28) & 0xf000) == 0x6000) {
        if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
          return 1;
        goto label_9ecc;
      }
label_9e8a:
      v1 = *(long *)(v1 + 0x260);
    }
    if (!v1)
      return 0;
  } while( true );
}


// Function: sub_9f10 @ 0x9f10
void sub_9f10(char *a0,char *a1)
{
  int *v1; // rax
  
  if (dat_266e8) {
    v1 = __errno_location();
    *v1 = 0;
    if (dat_26551) {
      strcasecmp(a0,a1); // tail-call
      return;
    }
    if (strcoll(a0,a1))
      return;
  }
  strcmp(a0,a1); // tail-call
}


// Function: sub_9f90 @ 0x9f90
unsigned long sub_9f90(int *a0,unsigned long *a1) // return-dupe x2
{
  char *v1;
  unsigned long v10; // rbx
  unsigned long *v11; // rbp
  unsigned long v12;
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  int v2;
  char v3; // al
  void *v4; // rax
  char *v5;
  int *v6; // rax
  void *v7; // rax
  unsigned long v8; // rax
  unsigned long *v9; // rax
  
  v2 = *a0;
  a1[1] = 0;
  a1[2] = 0;
  if (v2 != -1) { // branch-flip
    v4 = opendir(*(char **)&a0[2]);
    if (!v4)
      return 0;
    v12 = 0x200;
    v5 = (char *)sub_16a40(0x200);
    a1[2] = (unsigned long)v5;
    v6 = __errno_location();
    v13 = 0;
    v14 = 0;
label_a010:
    *v6 = 0;
    v7 = readdir(v4);
    if (v7) {
      v1 = (char *)((long)v7 + 0x13);
      v8 = strlen(v1);
      if (*(char *)((long)v7 + 0x13) == '.') goto label_a048;
      goto label_a058;
    }
    v2 = *v6;
    if (v2) {
      closedir(v4);
      *v6 = v2;
      return 0;
    }
    if (closedir(v4))
      return 0;
    if (0xffffffffffffffd < v14)
      sub_16ec0(); // no-return, return-dupe
    v9 = (unsigned long *)sub_16a40(v14 * 8 + 8);
    *a1 = v14;
    a1[1] = (unsigned long)v9;
    if (v14) {
      v11 = &v9[v14];
      do {
        *v9 = v5;
        v9 = &v9[1];
        v5 = &v5[strlen(v5) + 1];
      } while (v9 != v11);
    }
  }
  else {
    v9 = (unsigned long *)sub_16a40(8);
    *a1 = 0;
    a1[1] = (unsigned long)v9;
  }
  *v9 = 0;
  return 1;
label_a048:
  if ((*(char *)((long)v7 + 0x14)) && ((*(char *)((long)v7 + 0x14) != '.' || (*(char *)((long)v7 + 0x15))))) {
label_a058:
    v3 = sub_104d0(dat_26410,v1);
    if (!v3) {
      v10 = v13 + v8 + 1;
      while (v12 < v10) {
        if (0x3ffffffffffffffe < v12) {
          sub_16ec0();
        }
        v12 *= 2;
        v5 = (char *)sub_16a90(v5,v12);
        a1[2] = (unsigned long)v5;
      }
      memcpy(&v5[v13],v1,v8 + 1);
      v14 += 1;
      v13 = v10;
    }
  }
  goto label_a010;
}


// Function: sub_a1c0 @ 0xa1c0
void sub_a1c0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  int v3; // eax
  int *v4; // rax
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_266e8) {
    v4 = __errno_location();
    *v4 = 0;
    if (dat_26551) // branch-flip
      v3 = strcasecmp(v1,v2);
    else {
      v3 = strcoll(v1,v2);
    }
    if (v3)
      return;
  }
  strcmp(v1,v2); // tail-call
}


// Function: sub_a230 @ 0xa230
int sub_a230(int *a0,void *a1) // early-return
{
  unsigned long v1;
  long *v10;
  int v11; // stack - 0x8c
  void *v12; // stack - 0x68
  long *v13; // stack - 0x58
  void *v14; // stack - 0x50
  long *v2;
  char *v3;
  int v4;
  long v5;
  long *v6 [4]; // stack - 0x88
  unsigned long v7; // stack - 0x60
  char *v8;
  long v9;
  
  v4 = *a0;
  v11 = 0;
  if (((v4 == -1) || (sub_9e40(a0,0))) && ((a0[0x4c] == -1 || (sub_9e40(a0,1))))) {
    v1 = *(unsigned long *)&a0[(unsigned long)(v4 == -1) * 0x4c + 2];
    error(0,0,dcgettext(NULL,"%s: recursive directory loop",5),v1);
    return 2;
  }
  if (!sub_9f90(a0,&v6[2])) {
    sub_e490(*(unsigned long *)&a0[2]);
    v11 = 2;
  }
  if (sub_9f90(&a0[0x4c],&v7)) { // branch-flip
    if (!v11) {
      dat_266e8 = 1;
      v6[0] = v6[3];
      v6[1] = v13;
      if (_setjmp(0x26620))
        dat_266e8 = 0;
      qsort(v6[0],v6[2],8,sub_a1c0);
      qsort(v6[1],v7,8,sub_a1c0);
      v9 = dat_26488;
      if ((dat_26488) && (!*(long *)&a0[0x98])) {
        while ((*v6[0] && (v5 = *v6[0], (int)sub_9f10(v5,v9) <= -1))) {
          v6[0] = &v6[0][1];
        }
        while ((*v6[1] && (v5 = *v6[1], (int)sub_9f10(v5,v9) <= -1))) {
          v6[1] = &v6[1][1];
        }
      }
      while ((*v6[0] || (*v6[1]))) {
        if (*v6[0]) { // branch-flip
          if (*v6[1]) { // branch-flip
            v4 = sub_9f10(*v6[0],*v6[1]);
            if (v4) {
              if (0 <= v4) {
                v2 = &v6[1][1];
                v5 = *v6[1];
                v9 = 0;
                v6[1] = v2;
                goto label_a465;
              }
              goto label_a4ab;
            }
            if ((dat_26551) && (v4 = strcmp((char *)*v6[0],(char *)*v6[1]), v4)) {
              v2 = v6[(v4 >> 0x1f) + 1];
              v8 = (char *)v2[1];
              v3 = (char *)*v6[(unsigned int)-(v4 >> 0x1f)];
              v10 = &v2[1];
              if (v8) {
                while (!sub_9f10(v8,v3)) {
                  v4 = strcmp(v8,v3);
                  if (0 <= v4) {
                    if (!v4) {
                      memmove(&v2[1],v2,(long)v10 - (long)v2);
                      *v2 = (long)v3;
                    }
                    break;
                  }
                  v8 = (char *)v10[1];
                  v10 = &v10[1];
                  if (!v8) break;
                }
              }
            }
            v2 = &v6[1][1];
            v5 = *v6[1];
            v6[1] = v2;
          }
          else {
label_a4ab:
            v5 = 0;
          }
          v2 = &v6[0][1];
          v9 = *v6[0];
          v6[0] = v2;
        }
        else {
          v2 = &v6[1][1];
          v5 = *v6[1];
          v9 = 0;
          v6[1] = v2;
        }
label_a465:
        v4 = (*a1)(a0,v9,v5);
        if (v11 < v4)
          v11 = v4;
      }
    }
  }
  else {
    sub_e490(*(unsigned long *)&a0[0x4e]);
    v11 = 2;
  }
  free(v6[3]);
  free(v12);
  free(v13);
  free(v14);
  return v11;
}


// Function: sub_a660 @ 0xa660
unsigned long sub_a660(unsigned long a0,char *a1)
{
  unsigned long *v1;
  void *v10; // stack - 0x168
  unsigned long v11; // stack - 0x150
  char *v2;
  char *v3;
  unsigned long v4; // rax
  char v5 [8];
  unsigned int v6 [2]; // stack - 0x158
  unsigned long *v7;
  char *v8; // stack - 0x190
  unsigned long *v9; // stack - 0x170
  
  v9 = NULL;
  v10 = NULL;
  v8 = a1;
  if (dat_26551) {
    v6[0] = 0;
    v11 = a0;
    v8 = a1;
    if (sub_9f90(v6,v5)) {
      dat_266e8 = 1;
      if (_setjmp(0x26620)) // branch-flip
        v8 = a1;
      else {
        v2 = (char *)*v9;
        v3 = a1;
        v7 = v9;
        while (v8 = v3, v2) {
          if (!sub_9f10(v2,a1)) {
            v8 = v2;
            if (!strcmp(v2,a1)) break;
            if (a1 != v3)
              v8 = v3;
          }
          v1 = &v7[1];
          v7 = &v7[1];
          v2 = (char *)*v1;
          v3 = v8;
        }
      }
    }
  }
  v4 = sub_10d80(a0,v8,0);
  free(v9);
  free(v10);
  return v4;
}


// Function: sub_a7c0 @ 0xa7c0
void sub_a7c0(unsigned long a0) // return-dupe x2
{
  long v1;
  unsigned long v10; // stack - 0x50
  long v11; // stack - 0x38
  unsigned char v2;
  unsigned char *v3;
  char *v4;
  char *v5;
  unsigned int v6; // eax
  long v7;
  long v8; // stack - 0x40
  unsigned long v9; // stack - 0x48
  
  v6 = sub_f670(a0,&v10,&v9,&v8,&v11);
  if (!v6)
    return;
  sub_e830();
  sub_f610(0x2c,0x261a0,v10,v9);
  v2 = *(unsigned char *)((unsigned long)v6 + 0x1e550);
  v3 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,(unsigned int)v2);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = v2;
  }
  v4 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v4) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v4[1];
    *v4 = 10;
  }
  if (v6 == 1)
    return;
  v7 = v8;
  if (v8 <= v11) {
    do {
      while( true ) {
        v1 = v7 * 8;
        v7 += 1;
        v5 = *(char **)(dat_26388 + v1);
        if ((*v5 != '.') || (v5[1] != '\n')) break;
        fputs_unlocked("..\n.\ns/.//\n",dat_26188);
        if (v11 < v7)
          return;
        fputs_unlocked("a\n",dat_26188);
      }
      sub_f500("");
    } while (v7 <= v11);
  }
  fputs_unlocked(".\n",dat_26188);
}


// Function: sub_a960 @ 0xa960
void sub_a960(unsigned long a0) // return-dupe x2
{
  unsigned char v1;
  unsigned char *v2;
  char *v3;
  unsigned int v4; // eax
  long v5; // rbx
  long v6; // stack - 0x30
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x40
  long v9; // stack - 0x28
  
  v4 = sub_f670(a0,&v8,&v7,&v6,&v9);
  if (!v4)
    return;
  sub_e830();
  v1 = *(unsigned char *)((unsigned long)v4 + 0x1e550);
  v2 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v2) // branch-flip
    __overflow(dat_26188,(unsigned int)v1);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v2[1];
    *v2 = v1;
  }
  sub_f610(0x20,0x261a0,v8,v7);
  v3 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = 10;
  }
  if (v4 == 1)
    return;
  if (v6 <= v9) {
    do {
      v5 = v6 + 1;
      sub_f500("",dat_26388 + v6 * 8);
      v6 = v5;
    } while (v5 <= v9);
  }
  fputs_unlocked(".\n",dat_26188);
}


// Function: sub_aab0 @ 0xaab0
void sub_aab0(unsigned long a0) // return-dupe x3
{
  unsigned int v1; // eax
  long v10; // stack - 0x28
  long v2;
  long v3; // stack - 0x50
  unsigned long v4; // stack - 0x58
  unsigned long v5; // stack - 0x60
  long v6; // stack - 0x48
  long v7; // stack - 0x40
  long v8; // stack - 0x38
  long v9; // stack - 0x30
  
  v1 = sub_f670(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e830();
  sub_f5e0(0x261a0,v5,v4,&v7,&v8);
  if (v1 & 1) {
    v2 = 1;
    if (v7 <= v8)
      v2 = (v8 - v7) + 1;
    __fprintf_chk(dat_26188,1,"d%ld %ld\n",v7,v2);
  }
  if (!(v1 & 2))
    return;
  sub_f5e0(0x262d0,v3,v6,&v9,&v10);
  v2 = 1;
  if (v9 <= v10)
    v2 = (v10 - v9) + 1;
  __fprintf_chk(dat_26188,1,"a%ld %ld\n",v8,v2);
  if (v6 < v3)
    return;
  do {
    v2 = v3 + 1;
    sub_f500("",dat_26388 + v3 * 8);
    v3 = v2;
  } while (v2 <= v6);
}


// Function: sub_ac20 @ 0xac20
void sub_ac20(unsigned long a0)
{
  sub_f080(a0,sub_f070,sub_a7c0); // tail-call
}


// Function: sub_ac40 @ 0xac40
void sub_ac40(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_a960); // tail-call
}


// Function: sub_ac60 @ 0xac60
void sub_ac60(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_aab0); // tail-call
}


// Function: sub_ac80 @ 0xac80
long sub_ac80(unsigned long *a0,char a1) // early-return
{
  unsigned long v1;
  unsigned long v2;
  
  switch(a1) {
    case 0x45:
      a0 = &a0[3];
      break;
    case 0x46:
      a0 = &a0[3];
label_acf4:
      v1 = a0[1];
      v2 = *a0;
      return sub_f5d0(v2,v1); // tail-call
    default:
      switch(a1) {
        case 0x65:
          break;
        case 0x66:
          goto label_acf4;
        default:
          return -1;
        case 0x6c:
          goto label_ad0c;
        case 0x6d:
          goto label_ad2c;
        case 0x6e:
          goto label_ad44;
        
      }
    case 0x4c:
      a0 = &a0[3];
label_ad0c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f5d0(v2,v1) + -1;
    case 0x4d:
      a0 = &a0[3];
label_ad2c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f5d0(v2,v1); // tail-call
    case 0x4e:
      a0 = &a0[3];
label_ad44:
      return a0[2] - a0[1];
    
  }
  v1 = a0[1];
  v2 = *a0;
  return sub_f5d0(v2,v1) + -1;
}


// Function: sub_ad70 @ 0xad70
unsigned char * sub_ad70(void *a0,void *a1,long a2,unsigned long a3,unsigned long a4) // return-dupe x6
{
  unsigned char v1;
  char *v10; // rax
  char v11 [8];
  char *v12;
  char *v13; // rsp
  char *v14; // rsp
  char *v15; // rsp
  unsigned long v16; // rdi
  unsigned char v17;
  unsigned char *v18; // stack - 0x60
  unsigned char *v19; // stack - 0x58
  long v2;
  unsigned char *v20; // stack - 0x50
  char *v3;
  unsigned char v4; // al
  unsigned int v5;
  unsigned int v6;
  unsigned char *v7;
  unsigned char *v8;
  unsigned char *v9;
  
  v9 = (unsigned char *)((long)a1 + 1);
  v12 = v11;
  v14 = v11;
  v15 = v11;
  do {
    v17 = *v9;
    v6 = (unsigned int)v17;
    v9 = &v9[1];
    if (0x31 <= v17) {
      if ((unsigned int)((int)(char)v17 - 0x30U) <= 9) goto label_ae30;
      goto label_add0;
    }
  } while (!(0xfffedf7fffffffffU >> ((unsigned long)v17 & 0x3f) & 1));
  v5 = (int)(char)v17;
  while (v5 - 0x30 <= 9) {
label_ae30:
    v17 = *v9;
    v9 = &v9[1];
    v6 = (unsigned int)(char)v17;
    v5 = v6;
  }
  if (v17 != 0x2e) { // branch-flip
label_add0:
    v17 = (unsigned char)v6;
    v6 -= 0x58;
    v1 = *v9;
    v4 = (unsigned char)v6;
  }
  else {
    do {
      v17 = *v9;
      v9 = &v9[1];
    } while ((unsigned int)((int)(char)v17 - 0x30U) <= 9);
    v6 = (int)(char)v17 - 0x58;
    v1 = *v9;
    v4 = (unsigned char)v6;
  }
  if (0x21 <= v4)
    return NULL;
  if (0x100801001U >> ((unsigned long)v6 & 0x3f) & 1) {
    if (a2) { // branch-flip
      if (v1 != 0x6e)
        return (unsigned char *)0;
      v19 = (long)sub_f5d0(a2,a3);
    }
    else {
      v19 = (long)sub_ac80(a4,(int)(char)v1);
      if ((long)v19 <= -1)
        return (unsigned char *)0;
    }
    v9 = &v9[1];
    if (!a0)
      return v9;
    v18 = &v9[0xfffffffffffffffe - (long)a1];
    v20 = &v9[0xffffffffffffffff - (long)a1];
    if (0xfa1 <= &v9[1 - (long)a1]) { // branch-flip
      v10 = (char *)sub_16f00();
      v8 = v18;
    }
    else {
      v3 = v11;
      while (v14 != &v11[-((unsigned long)&v9[0x37 - (long)a1] & 0xfffffffffffff000)]) {
        v12 = &v3[-0x1000];
        v13 = &v3[-0x1000];
        *(unsigned long *)&v3[-8] = *(unsigned long *)&v3[-8];
        v14 = &v3[-0x1000];
        v3 = v13;
      }
      v16 = (unsigned long)((unsigned int)&v9[0x37 - (long)a1] & 0xff0);
      v2 = -v16;
      v15 = &v12[v2];
      if (v16)
        *(unsigned long *)&v12[-8] = *(unsigned long *)&v12[-8];
      v10 = (char *)(((unsigned long)&v12[v2 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v8 = v18;
    }
    v18 = v19;
    v19 = (unsigned char *)&v10[(long)v20];
    v20 = v8;
    *(unsigned long *)&v15[-8] = 0xaf86;
    memcpy(v10,a1,(unsigned long)v8,v15[-8]);
    v10[(long)v20] = 'l';
    *v19 = v17;
    v19[1] = 0;
    *(unsigned long *)&v15[-8] = 0xafb0;
    __fprintf_chk(a0,1,v10,v18);
    *(unsigned long *)&v15[-8] = 0xafb8;
    sub_121c0(v10);
    return v9;
  }
  if ((char)v6 != '\v')
    return NULL;
  if (v1 != 0x27)
    return (unsigned char *)0;
  v17 = v9[1];
  v6 = (unsigned int)v17;
  if (v17 == 0x27)
    return NULL;
  if (v17 != 0x5c) { // branch-flip
    if (!v17)
      return NULL;
    v8 = &v9[3];
    if (v9[2] != 0x27)
      return (unsigned char *)0;
  }
  else {
    v17 = v9[2];
    v8 = &v9[3];
    if (v17 != 0x27) { // branch-flip
      v6 = 0;
      v7 = v8;
      while( true ) {
        if (8 <= (unsigned int)((int)(char)v17 - 0x30U))
          return (unsigned char *)0;
        v8 = &v7[1];
        v6 = ((int)(char)v17 - 0x30U) + v6 * 8;
        v17 = *v7;
        if (v17 == 0x27) break;
        v7 = v8;
      }
    }
    else {
      v6 = 0;
      v7 = &v9[2];
    }
    if (3 <= &v7[0xfffffffffffffffd - (long)v9])
      return (unsigned char *)0;
  }
  if (!a0)
    return v8;
  v12 = *(char **)((long)a0 + 0x28);
  if (v12 < *(char **)((long)a0 + 0x30)) {
    *(char **)((long)a0 + 0x28) = &v12[1];
    *v12 = (char)v6;
    return v8;
  }
  __overflow(a0,v6 & 0xff);
  return v8;
}


// Function: sub_b070 @ 0xb070
void sub_b070(void *a0,char *a1,long *a2) // return-dupe x3
{
  char v1;
  long v10;
  void *v11;
  char *v12;
  long v13;
  long v2;
  long v3;
  long v4;
  char *v5;
  char *v6;
  unsigned long v7; // rax
  char v8;
  unsigned int v9; // esi
  
  v2 = *a2;
  v13 = a2[1];
  v3 = *(long *)(v2 + 0xb8);
  v4 = a2[2];
  if (!a0)
    return;
  if ((!dat_264a0) && (*a1 == '%')) {
    if (a1[1] != 'l') { // branch-flip
      if ((a1[1] == 'L') && (!a1[2])) {
        v11 = *(void **)(v3 + v13 * 8);
        v7 = *(long *)(v3 + v4 * 8) - (long)v11;
        fwrite_unlocked(v11,1,v7,a0); // return-dupe, tail-call
        return;
      }
    }
    else if ((a1[2] == '\n') && (!a1[3])) {
      if (v4 <= v13)
        return;
      v11 = *(void **)(v3 + v13 * 8);
      v2 = *(long *)(v3 + v4 * 8);
      v7 = (v2 + (unsigned long)(*(char *)(v2 + -1) != '\n')) - (long)v11;
      fwrite_unlocked(v11,1,v7,a0);
      return;
    }
  }
  if (v4 <= v13)
    return;
  do {
    v8 = *a1;
    v12 = a1;
label_b0e8:
    v5 = v12;
    if (v8) {
      do {
        v12 = &v5[1];
        if (v8 != '%') { // branch-flip
          v9 = (unsigned int)v8;
label_b0f6:
          v5 = *(char **)((long)a0 + 0x28);
          if (*(char **)((long)a0 + 0x30) <= v5) goto label_b178;
label_b102:
          *(char **)((long)a0 + 0x28) = &v5[1];
          *v5 = v8;
          v6 = v12;
        }
        else {
          v1 = v5[1];
          v6 = &v5[2];
          if (v1 != 'L') { // branch-flip
            if (v1 != 'l') {
              if (v1 == '%') {
                v9 = 0x25;
                v12 = v6;
                goto label_b0f6;
              }
              v6 = (char *)sub_ad70(a0,v5,v2,v13,0);
              if (v6) goto label_b10d;
              v5 = *(char **)((long)a0 + 0x28);
              v9 = 0x25;
              if (v5 < *(char **)((long)a0 + 0x30)) goto label_b102;
              goto label_b178;
            }
            v10 = *(long *)(v3 + 8 + v13 * 8);
            v10 -= (unsigned long)(*(char *)(v10 + -1) == '\n');
          }
          else {
            v10 = *(long *)(v3 + 8 + v13 * 8);
          }
          sub_f0e0(*(unsigned long *)(v3 + v13 * 8),v10,0,0);
        }
label_b10d:
        v8 = *v6;
        v5 = v6;
        if (!v8) break;
      } while( true );
    }
    v13 += 1;
    if (v4 == v13)
      return;
  } while( true );
label_b178:
  __overflow(a0,v9 & 0xff);
  v8 = *v12;
  goto label_b0e8;
}


// Function: sub_b2a0 @ 0xb2a0
void sub_b2a0(void *a0,unsigned char *a1,unsigned char a2,long a3)
{
  unsigned char *v1;
  unsigned char v10;
  unsigned char *v11; // stack - 0x60
  long v12 [3]; // stack - 0x58
  unsigned long v13;
  void *v14;
  void *v2;
  unsigned char *v3;
  int *v4; // rax
  unsigned char *v5;
  char *v6; // rax
  long v7; // rax
  long v8;
  unsigned char v9;
  
  v9 = *a1;
  if ((v9 == a2) || (!v9))
    return;
  do {
    v1 = &a1[1];
    v10 = v9;
    if (v9 != 0x25) {
label_b2ee:
      v3 = v1;
      if (a0) {
        v1 = *(unsigned char **)((long)a0 + 0x28);
        if (*(unsigned char **)((long)a0 + 0x30) <= v1) {
          __overflow(a0,(unsigned int)v10);
          v9 = *v3;
          goto label_b313;
        }
        *(unsigned char **)((long)a0 + 0x28) = &v1[1];
        *v1 = v10;
      }
      v9 = *v3;
      goto label_b313;
    }
    v10 = a1[1];
    v3 = &a1[2];
    v8 = a3;
    switch(v10) {
      case 0x25:
        v1 = v3;
        goto label_b2ee;
      default:
        v3 = (unsigned char *)sub_ad70(a0,a1,0,0,a3);
        v10 = v9;
        if (!v3) goto label_b2ee;
        v9 = *v3;
        goto label_b313;
      case 0x28:
        v8 = 0;
        while( true ) {
          if (10 <= (unsigned int)((int)(char)*v3 - 0x30U)) { // branch-flip
            v7 = sub_ac80(a3);
            v12[v8] = v7;
            v10 = v9;
            if (v7 < 0) goto label_b2ee;
            v5 = &v3[1];
          }
          else {
            v4 = __errno_location();
            *v4 = 0;
            v12[v8] = strtoimax(v3,&v11,10);
            v10 = v9;
            if (*v4) goto label_b2ee;
            v5 = v11;
          }
          v3 = &v5[1];
          v10 = v9;
          if (*v5 != "=?"[v8]) goto label_b2ee;
          if (v8 == 1) break;
          v8 = 1;
        }
        if (v12[0] != v12[1]) { // branch-flip
          v14 = a0;
          v2 = NULL;
        }
        else {
          v14 = NULL;
          v2 = a0;
        }
        v6 = (char *)sub_b2a0(v2,v3,0x3a,a3);
        if (!*v6)
          return;
        v6 = (char *)sub_b2a0(v14,&v6[1],0x29,a3);
        if (!*v6)
          return;
        v9 = v6[1];
        v3 = (unsigned char *)&v6[1];
        goto label_b313;
      case 0x3c:
        v13 = dat_26448;
        break;
      case 0x3d:
        v13 = dat_26440;
        break;
      case 0x3e:
        v8 = a3 + 0x18;
        v13 = dat_26450;
      
    }
    sub_b070(a0,v13,v8);
    v9 = a1[2];
label_b313:
    if (!v9)
      return;
    a1 = v3;
    if (a2 == v9)
      return;
  } while( true );
}


// Function: sub_b560 @ 0xb560
void sub_b560(unsigned long a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x88
  long v3; // stack - 0x78
  long v4; // stack - 0x80
  long v5 [3]; // stack - 0x70
  long v6; // stack - 0x58
  unsigned long v7; // stack - 0x50
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v1 = sub_f670(a0,&v2,&v4,&v3,v5);
  if (!v1)
    return;
  sub_e830();
  if ((dat_266f8 < v2) || (dat_266f0 < v3)) {
    v9 = v3;
    v6 = v2;
    v8 = dat_266f0;
    v5[1] = 0x261a0;
    v5[2] = dat_266f8;
    v7 = 0x262d0;
    sub_b2a0(dat_26188,dat_26460,0,&v5[1]);
  }
  dat_266f8 = v4 + 1;
  v5[1] = 0x261a0;
  dat_266f0 = v5[0] + 1;
  v7 = 0x262d0;
  v6 = dat_266f8;
  v9 = dat_266f0;
  sub_b2a0(dat_26188,*(unsigned long *)((unsigned long)v1 * 8 + 0x26460),0,&v5[1]);
}


// Function: sub_b6c0 @ 0xb6c0
void sub_b6c0(unsigned long a0) // return-dupe
{
  unsigned long v1; // stack - 0x48
  long v2; // stack - 0x40
  long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  dat_266f0 = -dat_26288;
  dat_266f8 = dat_266f0;
  sub_f080(a0,sub_f060,sub_b560);
  if ((dat_26270 <= dat_266f8) && (dat_263a0 <= dat_266f0))
    return;
  sub_e830();
  v1 = 0x261a0;
  v2 = dat_266f8;
  v3 = dat_26270;
  v4 = 0x262d0;
  v5 = dat_266f0;
  v6 = dat_263a0;
  sub_b2a0(dat_26188,dat_26460,0,&v1);
}


// Function: sub_b7b0 @ 0xb7b0
void sub_b7b0(long a0) // return-dupe
{
  char *v1;
  char v2;
  unsigned long *v3;
  char *v4; // rax
  char *v5;
  char *v6;
  long v7;
  unsigned long *v8;
  
  v3 = *(unsigned long **)(a0 + 0xa0);
  if (!v3)
    return;
  v7 = *(long *)(a0 + 0xb0);
  if (dat_26490) {
    v1 = (char *)((long)v3 + v7);
    *v1 = '\r';
    v4 = rawmemchr(v3,0xd);
    if (v1 != v4) { // branch-flip
      v5 = v4;
      do {
        while ((v2 = *v4, v2 == '\r' && (v4[1] == '\n'))) {
          v6 = &v5[1];
          v4 = &v4[2];
          *v5 = '\n';
          v5 = v6;
          if (v1 == v4) goto label_b840;
        }
        v4 = &v4[1];
        v6 = &v5[1];
        *v5 = v2;
        v5 = v6;
      } while (v1 != v4);
    }
    else {
      v6 = v1;
    }
label_b840:
    v7 -= (long)v1 - (long)v6;
  }
  v8 = v3;
  if ((v7) && (v8 = (unsigned long *)((long)v3 + v7), *(char *)((long)v3 + v7 + -1) != '\n')) {
    *(char *)v8 = 10;
    v7 += 1;
    *(char *)(a0 + 0x120) = 1;
    v8 = (unsigned long *)((long)v3 + v7);
  }
  *v8 = 0;
  *(long *)(a0 + 0xb0) = v7;
}


// Function: sub_b920 @ 0xb920
unsigned long sub_b920(int *a0,char a1)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (0 <= *a0) { // branch-flip
    v1 = sub_17190(8,*(unsigned long *)&a0[0x12],0x7fffffffffffffef);
    *(unsigned long *)&a0[0x2a] = v1;
    *(unsigned long *)&a0[0x28] = sub_16a40(v1);
    if (!a1) {
      sub_b8a0(a0,*(unsigned long *)&a0[0x2a]);
      v2 = memchr(*(void **)&a0[0x28],0,*(unsigned long *)&a0[0x2c]);
      return CONCAT71((undefined7)((unsigned long)v2 >> 8),v2 != NULL);
    }
  }
  else {
    a0[0x2a] = 8;
    a0[0x2b] = 0;
    *(unsigned long *)&a0[0x28] = sub_16a40(8);
  }
  a0[0x2c] = 0;
  a0[0x2d] = 0;
  *(char *)((long)a0 + 0x121) = 0;
  return 0;
}


// Function: sub_b9e0 @ 0xb9e0
void sub_b9e0(int *a0) // return-dupe x2
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r12
  
  if (*a0 < 0)
    return;
  if ((a0[10] & 0xf000U) != 0x8000) // branch-flip
    v4 = *(unsigned long *)&a0[0x2c];
  else {
    v5 = *(unsigned long *)&a0[0x10];
    v4 = (v5 & 0xfffffffffffffff8) + 0x10;
    if ((v4 < v5) || (0x7ffffffffffffffe < v4))
      sub_16ec0(); // no-return, return-dupe
    if (v4 <= *(unsigned long *)&a0[0x2a]) // branch-flip
      v4 = *(unsigned long *)&a0[0x2c];
    else {
      *(unsigned long *)&a0[0x2a] = v4;
      v1 = *(unsigned long *)&a0[0x28];
      *(unsigned long *)&a0[0x28] = sub_16a90(v1);
      v4 = *(unsigned long *)&a0[0x2c];
    }
    if (v4 <= v5) {
      sub_b8a0(a0,(v5 - v4) + 1);
      v4 = *(unsigned long *)&a0[0x2c];
      if (v4 <= v5)
        return;
    }
  }
  sub_b8a0(a0,*(long *)&a0[0x2a] - v4);
  v4 = *(unsigned long *)&a0[0x2c];
  if (!v4)
    return;
  v5 = *(unsigned long *)&a0[0x2a];
  if (v4 == v5) {
    do {
      do {
        if (0x3ffffffffffffff7 < v5) {
          sub_16ec0();
        }
        *(unsigned long *)&a0[0x2a] = v5 * 2;
        v2 = sub_16a90(*(unsigned long *)&a0[0x28]);
        v5 = *(unsigned long *)&a0[0x2a];
        v4 = *(unsigned long *)&a0[0x2c];
        *(long *)&a0[0x28] = v2;
        v6 = v5 - v4;
      } while (!v6);
      if (!*(char *)((long)a0 + 0x121)) {
        v3 = sub_170e0(*a0,v2 + v4,v6);
        if (v3 == 0xffffffffffffffff)
          sub_e770(*(unsigned long *)&a0[2]); // no-return
        v5 = *(unsigned long *)&a0[0x2a];
        v4 = *(long *)&a0[0x2c] + v3;
        *(unsigned long *)&a0[0x2c] = v4;
        *(bool *)((long)a0 + 0x121) = v3 < v6;
      }
    } while (v4 == v5);
  }
  v1 = *(unsigned long *)&a0[0x28];
  *(unsigned long *)&a0[0x2a] = v4 + 0x10 & 0xfffffffffffffff8;
  *(unsigned long *)&a0[0x28] = sub_16a90(v1);
}


// Function: sub_bbc0 @ 0xbbc0
unsigned char sub_bbc0(int *a0,unsigned int a1)
{
  unsigned char *v1;
  unsigned char *v10;
  long *v11; // rax
  int v12; // ecx
  unsigned char *v13;
  unsigned char *v14;
  unsigned int v15; // ebx
  unsigned char *v16;
  unsigned char *v17;
  unsigned char *v18;
  unsigned long *v19;
  unsigned long *v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22; // rax
  unsigned long v23;
  unsigned char *v24;
  unsigned long v25;
  unsigned long v26;
  char v27; // r13b
  unsigned char *v28;
  unsigned long v29;
  void *v3;
  unsigned char v30;
  unsigned char *v31;
  bool v32; // cf
  bool v33; // zf
  unsigned long v34; // stack - 0xe0
  unsigned long v35; // stack - 0xd8
  unsigned long *v36; // stack - 0xd0
  unsigned long v37; // stack - 0xc0
  void *v38; // stack - 0xb8
  unsigned long v39; // stack - 0xb0
  char v4;
  long v40; // stack - 0xa0
  long v41; // stack - 0x98
  unsigned long v42; // stack - 0x90
  unsigned long v43; // stack - 0x88
  long *v44; // stack - 0x60
  unsigned int v5;
  long v6;
  unsigned long v7; // rax
  long v8;
  long v9;
  
  v15 = (unsigned int)dat_26568;
  v5 = sub_b920(a0,(v15 | a1) & 0xff);
  v30 = (unsigned char)(v5 | a1);
  if (*a0 != a0[0x4c]) // branch-flip
    v30 |= sub_b920(&a0[0x4c],(v15 | a1 | v5 | a1) & 0xff);
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
  }
  if (v30)
    return v30;
  sub_b9e0(a0);
  sub_b7b0(a0);
  if (*a0 != a0[0x4c]) { // branch-flip
    sub_b9e0(&a0[0x4c]);
    sub_b7b0(&a0[0x4c]);
    v28 = *(unsigned char **)&a0[0x28];
    v31 = *(unsigned char **)&a0[0x74];
    v21 = *(unsigned long *)&a0[0x2c];
    v25 = *(unsigned long *)&a0[0x78];
    if (v28 == v31) goto label_bcf5;
    v10 = &v28[v21];
    v24 = &v31[v25];
    if (v25 <= v21) // branch-flip
      *v24 = ~v28[v25];
    else {
      *v10 = ~v31[v21];
    }
    v16 = v28;
    v13 = v31;
    if (*(long *)v31 == *(long *)v28) {
      do {
        v16 = &v16[8];
        v13 = &v13[8];
      } while (*(long *)v16 == *(long *)v13);
    }
    if (*v13 == *v16) {
      do {
        v16 = &v16[1];
        v13 = &v13[1];
      } while (*v16 == *v13);
    }
    v5 = dat_26580 - 4;
    v8 = dat_26560;
    if ((2 <= v5) && (&v28[v21 - *(unsigned char *)&a0[0x48]] < v16 != &v31[v25 - *(unsigned char *)&a0[0x94]] < v13)) {
      v16 = &v16[-1];
      v13 = &v13[-1];
    }
  }
  else {
    v21 = *(unsigned long *)&a0[0x2c];
    v31 = *(unsigned char **)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned char **)&a0[0x74] = v31;
    *(unsigned long *)&a0[0x78] = v21;
    *(char *)&a0[0x94] = (char)a0[0x48];
    v25 = v21;
label_bcf5:
    v16 = &v31[v25];
    v10 = &v31[v21];
    v5 = dat_26580 - 4;
    v13 = v16;
    v8 = dat_26560;
    v28 = v31;
    v24 = v16;
  }
  while (v16 != v28) {
    while (v16[-1] != 10) {
      v16 = &v16[-1];
      v13 = &v13[-1];
      if (v16 == v28) goto label_bd54;
    }
    v6 = v8 + -1;
    if (!v8) break;
    v13 = &v13[-1];
    v16 = &v16[-1];
    v8 = v6;
  }
label_bd54:
  *(unsigned char **)&a0[0x38] = v16;
  *(unsigned char **)&a0[0x84] = v13;
  v13 = v24;
  if ((v5 < 2) || (v17 = v10, (char)a0[0x48] == (char)a0[0x94])) {
    v14 = v10;
    v17 = &v16[v21 - v25];
    if (v21 < v25)
      v17 = v16;
    while (v14 != v17) {
      v18 = &v14[-1];
      v1 = &v13[-1];
      if (v14[-1] != v13[-1]) break;
      v13 = v1;
      v14 = v18;
    }
    if ((v14 != v28) && (v14[-1] != 10))
      v8 = dat_26560 + 1;
    else {
      v8 = dat_26560;
      if (v13 != v31)
        v8 = dat_26560 + (unsigned long)(v13[-1] != 10);
    }
    v6 = v8 + -1;
    v17 = v14;
    if ((v8) && (v18 = v14, v14 != v10)) {
      do {
        do {
          v17 = &v18[1];
          v30 = *v18;
          v18 = v17;
        } while (v30 != 10);
        v33 = v6 != 0;
        v6 -= 1;
      } while (v17 != v10 && v33);
      v13 = &v13[(long)v17 - (long)v14];
    }
  }
  *(unsigned char **)&a0[0x3c] = v17;
  *(unsigned char **)&a0[0x88] = v13;
  if ((((dat_26578) && (!dat_26520)) && ((long)dat_26570 <= 0x1ffffffffffffffe)) && (dat_26570 < v21)) {
    v8 = 1;
    v20 = (unsigned long)((long)v17 - (long)v16) >> 5;
    if (!v20)
      v20 = 1;
    if (0x787878787878783 <= v20)
      v20 = 0x787878787878782;
    v29 = (unsigned long)((long)v10 - (long)v17) >> 5;
    if (!v29)
      v29 = 1;
    if (0x787878787878783 <= v29)
      v29 = 0x787878787878782;
    v25 = 0;
    if (0 < (long)dat_26570) {
      do {
        v8 *= 2;
      } while (v8 <= (long)dat_26570);
      v25 = v8 - 1;
    }
    v21 = dat_26570;
    if ((long)(v29 + 5) < (long)dat_26570)
      v21 = v29 + 5;
    v21 = v20 + 5 + v8 + v21;
    v20 = v21;
  }
  else {
    v21 >>= 5;
    v25 = 0xffffffffffffffff;
    if (!v21)
      v21 = 1;
    if (0x787878787878783 <= v21)
      v21 = 0x787878787878782;
    v8 = 0;
    v21 += 5;
    v20 = v21;
  }
  v6 = sub_16a40(v21 * 8);
  v10 = *(unsigned char **)&a0[0x38];
  if (((dat_26578) && (v17 == v10)) && (v16 = *(unsigned char **)&a0[0x84], v13 == v16)) {
    v10 = v28;
label_cbe8:
    if (v8) {
      v26 = 0;
label_bfa9:
      v21 = dat_26570;
      v29 = v26;
      if ((long)dat_26570 <= (long)v26)
        v29 = dat_26570;
      goto label_bfb3;
    }
    v29 = 0;
    v23 = 0x20;
    v22 = (long)v13 - (long)v16;
    v26 = 0;
    v21 = dat_26570;
  }
  else {
    if (v28 == v10) {
      v16 = *(unsigned char **)&a0[0x84];
      goto label_cbe8;
    }
    v16 = v28;
    v21 = 0;
    v29 = v20;
    do {
      v26 = v21 + 1;
      v20 = v29;
      if ((v21 & v25) == v29) {
        if (0x7fffffffffffffe < v29) {
          sub_16ec0(); // no-return, return-dupe
        }
        v20 = v29 * 2;
        v6 = sub_16a90(v6,v29 << 4);
      }
      *(unsigned char **)(v6 + (v21 & v25) * 8) = v16;
      v17 = v16;
      do {
        v16 = &v17[1];
        v30 = *v17;
        v17 = v16;
      } while (v30 != 10);
      v21 = v26;
      v29 = v20;
    } while (v16 != v10);
    v16 = *(unsigned char **)&a0[0x84];
    if (v8) goto label_bfa9;
    v21 = dat_26570;
    v29 = v26;
label_bfb3:
    v23 = 0x20;
    v22 = (long)v13 - (long)v16;
    if (10 <= (long)v26)
      v23 = (unsigned long)((long)v10 - (long)v28) / (v26 - 1);
  }
  v7 = (unsigned long)((long)v24 - (long)v13) / v23;
  if (!v7)
    v7 = 1;
  if (0x787878787878783 <= v7)
    v7 = 0x787878787878782;
  if ((long)(v7 + 5) <= (long)v21)
    v21 = v7 + 5;
  v22 /= v23;
  if (!v22)
    v22 = 1;
  if (0x787878787878783 <= v22)
    v22 = 0x787878787878782;
  v21 += v29 + 5 + v22;
  if ((long)v21 < (long)v29 || 0xffffffffffffffe < v21) {
    sub_16ec0();
  }
  v8 = sub_16a40(v21 * 8);
  if (v29 != v26) { // branch-flip
    if (1 <= (long)v29) {
      v22 = v26 - dat_26570;
      v23 = v29 + v22;
      v9 = v22 * -8;
      do {
        *(unsigned long *)(v8 + v9 + v22 * 8) = *(unsigned long *)(v6 + (v25 & v22) * 8);
        v22 += 1;
      } while (v23 != v22);
      v25 = 0;
      do {
        *(unsigned long *)(v6 + v25 * 8) = *(unsigned long *)(v8 + v25 * 8);
        v25 += 1;
      } while (v29 != v25);
label_c0d0:
      v9 = 0;
      do {
        *(unsigned char **)(v8 + v9 * 8) = &v31[*(long *)(v6 + v9 * 8) - (long)v28];
        v9 += 1;
      } while (v9 < (long)v29);
    }
  }
  else if (v29) goto label_c0d0;
  *(unsigned long *)&a0[0x7a] = v29 * 8 + v8;
  *(unsigned long *)&a0[0x2e] = v6 + v29 * 8;
  *(unsigned long *)&a0[0x7c] = -v29;
  *(unsigned long *)&a0[0x30] = -v29;
  dat_26700 = (v20 - v29) + 1 + (v21 - v29);
  *(unsigned long *)&a0[0x36] = v20 - v29;
  *(unsigned long *)&a0[0x82] = v21 - v29;
  *(unsigned long *)&a0[0x86] = v26;
  *(unsigned long *)&a0[0x3a] = v26;
  if (dat_26700 > 0x3fffffffffffffe)
    sub_16ec0();
  dat_26710 = (void *)sub_16a40(dat_26700 * 0x20);
  dat_26708 = 1;
  if (0x201 <= (unsigned long)((long)dat_26700 / 3)) { // branch-flip
    v12 = 9;
    do {
      v12 += 1;
      dat_26718 = 1L << ((unsigned char)v12 & 0x3f);
    } while (dat_26718 < (unsigned long)((long)dat_26700 / 3));
    dat_26718 -= *(unsigned char *)((long)v12 + 0x1e1a0);
    if (0xffffffffffffffe < dat_26718) {
      sub_16ec0();
    }
    v8 = dat_26718 * 8 + 8;
  }
  else {
    dat_26718 = 0x1fd;
    v8 = 0xff0;
  }
  dat_26720 = sub_fab0(v8) + 8;
  v44 = (long *)&a0[0x28];
  do {
    v40 = v44[3];
    v8 = v44[4];
    v31 = (unsigned char *)v44[8];
    v42 = v44[7];
    v41 = sub_16a40(v42 * 8);
    v5 = dat_26558;
    v4 = dat_26552;
    v28 = (unsigned char *)v44[10];
    v38 = dat_26710;
    v37 = dat_26708;
    v43 = dat_26700;
    v10 = (unsigned char *)(v44[2] + *v44);
    v33 = dat_26558 != 0;
    if (v31 < v28) {
      v15 = dat_26558 & 1;
      v30 = *v31;
      v35 = 0;
      v24 = v31;
      v21 = dat_26700;
      v25 = dat_26498;
      do {
        v20 = (unsigned long)v30;
        v31 = &v24[1];
        dat_26498 = v25;
        if (v5 != 4) { // branch-flip
          if (5 <= v5) { // branch-flip
            if (v5 != 5) { // branch-flip
label_c810:
              if (v4) { // branch-flip
                if (v30 == 10) goto label_c8d1;
                v29 = 0;
                v16 = v31;
                do {
                  v31 = &v16[1];
                  v6 = v20 * 4;
                  v30 = *v16;
                  v20 = (unsigned long)v30;
                  v29 = (v29 << 7 | v29 >> 0x39) + (long)*(int *)(*(long *)__ctype_tolower_loc() + v6);
                  v16 = v31;
                } while (v30 != 10);
                v16 = &v31[0xffffffffffffffff - (long)v24];
              }
              else {
                v29 = 0;
                v16 = NULL;
                v13 = v31;
                if (v30 != 10) {
                  do {
                    v31 = &v13[1];
                    v29 = (v29 << 7 | v29 >> 0x39) + v20;
                    v30 = *v13;
                    v20 = (unsigned long)v30;
                    v13 = v31;
                  } while (v30 != 10);
                  v16 = &v31[0xffffffffffffffff - (long)v24];
                }
              }
            }
            else {
              if (v30 == 10) goto label_c8d1;
              v29 = 0;
              v6 = *(long *)__ctype_b_loc();
              do {
                if (!(*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20)) {
                  if (v4)
                    v20 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v20 * 4);
                  v29 = (v29 << 7 | v29 >> 0x39) + v20;
                }
                v30 = *v31;
                v20 = (unsigned long)v30;
                v31 = &v31[1];
              } while (v30 != 10);
              v16 = &v31[0xffffffffffffffff - (long)v24];
            }
          }
          else {
            if (v5 - 1 > 2) goto label_c810;
            if (v30 == 10) goto label_c8d1;
            v29 = 0;
            v23 = 0;
            v26 = v20;
            v16 = v31;
            do {
              if (v5 & 2) {
                v6 = *(long *)__ctype_b_loc();
                if (*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20) {
                  v31 = v16;
                  do {
                    v30 = *v31;
                    v31 = &v31[1];
                    if (v30 == 10) {
                      v16 = &v31[0xffffffffffffffff - (long)v24];
                      goto label_c3d0;
                    }
                  } while (*(unsigned char *)(v6 + 1 + (unsigned long)v30 * 2) & 0x20);
                }
              }
              if (v15) { // branch-flip
                v27 = (char)v26;
                if (v27 != '\t') { // branch-flip
                  if (v27 != '\r') { // branch-flip
                    if (v27 == '\b') {
                      v20 = 8;
                      v23 = (v23 - 1) + (unsigned long)(v23 == 0);
                      goto label_c6a0;
                    }
                    v23 += 1;
                    v26 = 1;
                  }
                  else {
                    v20 = 0xd;
                    v26 = 1;
                    v23 = 0;
                  }
                }
                else {
                  v20 = 0x20;
                  v26 = v25 - v23 % v25;
                  v32 = CARRY8(v23,v26);
                  v23 += v26;
                  if (v32)
                    v23 = 0;
                }
              }
              else {
label_c6a0:
                v26 = 1;
              }
              if (v4)
                v20 = (unsigned long)*(unsigned char *)(*(long *)__ctype_tolower_loc() + v20 * 4);
              do {
                v29 = (v29 << 7 | v29 >> 0x39) + v20;
                v26 -= 1;
              } while (v26);
              v30 = *v16;
              v20 = (unsigned long)v30;
              v31 = &v16[1];
              v26 = v20;
              v16 = v31;
            } while (v30 != 10);
            v16 = &v31[0xffffffffffffffff - (long)v24];
          }
        }
        else {
          if (v30 != 10) {
            v29 = 0;
            v6 = *(long *)__ctype_b_loc();
            do {
              v16 = v31;
              if (*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20) {
                v30 = *v31;
                while( true ) {
                  v31 = &v31[1];
                  if (v30 == 10) goto label_c549;
                  v20 = (unsigned long)v30;
                  if (!(*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20)) break;
                  v30 = *v31;
                }
                v29 = (v29 << 7 | v29 >> 0x39) + 0x20;
                v16 = v31;
              }
              if (v4)
                v20 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v20 * 4);
              v31 = &v16[1];
              v29 = (v29 << 7 | v29 >> 0x39) + v20;
              v20 = (unsigned long)*v16;
            } while (*v16 != 10);
label_c549:
            v16 = &v31[0xffffffffffffffff - (long)v24];
            goto label_c3d0;
          }
label_c8d1:
          v16 = NULL;
          v29 = 0;
        }
label_c3d0:
        v19 = (unsigned long *)(dat_26720 + (v29 % dat_26718) * 8);
        v36 = v19;
        if (((v10 == v31) && ((char)v44[0x10])) && ((2 <= (unsigned int)(dat_26580 - 4U) && (v36 = (unsigned long *)(dat_26720 + -8), 2 <= v5))))
          v36 = v19;
        v39 = *v36;
        v43 = v21;
        if (v39) {
          v34 = v39;
label_c432:
          do {
            v19 = (unsigned long *)(v34 * 0x20 + (long)v38);
            if (v29 == v19[1]) {
              v3 = (void *)v19[2];
              if ((unsigned char *)v19[3] != v16) { // branch-flip
                if (!v5) goto label_c420;
              }
              else {
                if (!memcmp(v3,v24,(unsigned long)v16)) goto label_c477;
                if (!(v33 || v4)) {
                  v34 = *v19;
                  if (!v34) break;
                  goto label_c432;
                }
              }
              if (!sub_ec00(v3,v24)) goto label_c477;
            }
label_c420:
            v34 = *v19;
          } while (v34);
        }
        if (v21 == v37) {
          if (0x1fffffffffffffe < v21) {
            sub_16ec0();
          }
          v43 = v21 * 2;
          v38 = (void *)sub_16a90(v38,v21 << 6);
          v39 = *v36;
        }
        v19 = (unsigned long *)(v37 * 0x20 + (long)v38);
        *v19 = v39;
        v19[1] = v29;
        v19[2] = (unsigned long)v24;
        v19[3] = (unsigned long)v16;
        *v36 = v37;
        v34 = v37;
        v37 += 1;
label_c477:
        if (v35 == v42) {
          if (((0x2aaaaaaaaaaaaaa9 < (long)v35) || (v42 = v35 * 2 - v8, 0xffffffffffffffe < v42)) || (0xffffffffffffffe < v35 - v8)) {
            sub_16ec0();
          }
          v41 = sub_16a90(v41,v42 * 8);
          v40 = sub_16a90(v40 + v8 * 8,(v42 - v8) * 8) + v8 * -8;
        }
        *(unsigned char **)(v40 + v35 * 8) = v24;
        *(unsigned long *)(v41 + v35 * 8) = v34;
        v35 += 1;
        if (v28 <= v31) goto label_c8e7;
        v30 = *v31;
        v24 = v31;
        v21 = v43;
        v25 = dat_26498;
      } while( true );
    }
    v35 = 0;
label_c8e7:
    v6 = 0;
    v44[5] = v35;
    while( true ) {
      if (v42 != v35) { // branch-flip
        v2 = (unsigned long *)(v40 + v35 * 8);
        *v2 = v31;
      }
      else {
        if (((0x2aaaaaaaaaaaaaa9 < (long)v42) || (v21 = v42 * 2 - v8, 0xffffffffffffffe < v21)) || (0xffffffffffffffe < v42 - v8)) {
          sub_16ec0();
        }
        v40 = sub_16a90(v40 + v8 * 8,(v21 - v8) * 8) + v8 * -8;
        v2 = (unsigned long *)(v40 + v35 * 8);
        *v2 = v31;
        v42 = v21;
      }
      if (v10 == v31) break;
      if (((long)dat_26570 <= v6) && (dat_26578)) goto label_c9fc;
      v35 += 1;
      v28 = v31;
      do {
        v31 = &v28[1];
        v30 = *v28;
        v28 = v31;
      } while (v30 != 10);
      v6 += 1;
    }
    if (((char)v44[0x10]) && (2 <= (unsigned int)(dat_26580 - 4U)))
      *v2 = &v10[-1];
label_c9fc:
    dat_26700 = v43;
    v44[3] = v40;
    v11 = &v44[0x26];
    dat_26708 = v37;
    v44[6] = v35;
    v44[7] = v42;
    v44[0xb] = v41;
    dat_26710 = v38;
    v44 = v11;
    if ((long *)&a0[0xc0] == v11) {
      *(unsigned long *)&a0[0x96] = v37;
      *(unsigned long *)&a0[0x4a] = v37;
      free(v38);
      free((void *)(dat_26720 + -8));
      return 0;
    }
  } while( true );
}


// Function: sub_cd40 @ 0xcd40
void sub_cd40(unsigned long a0) // return-dupe x2
{
  unsigned char v1;
  long v10;
  long v11; // rsi
  unsigned char *v2;
  char *v3;
  unsigned int v4; // eax
  long v5;
  long v6; // stack - 0x40
  long v7; // stack - 0x48
  long v8; // stack - 0x50
  long v9; // stack - 0x38
  
  v4 = sub_f670(a0,&v8,&v7,&v6,&v9);
  if (!v4)
    return;
  sub_e830();
  sub_f510(4);
  sub_f610(0x2c,0x261a0,v8,v7);
  v1 = *(unsigned char *)((unsigned long)v4 + 0x1e550);
  v2 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v2) // branch-flip
    __overflow(dat_26188,(unsigned int)v1);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v2[1];
    *v2 = v1;
  }
  sub_f610(0x2c,0x262d0,v6,v9);
  sub_f510(3);
  v3 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = 10;
  }
  if ((v4 & 1) && (v8 <= v7)) {
    v5 = v8 * 8;
    v10 = v8;
    do {
      while( true ) {
        sub_f510(2);
        v11 = dat_26258 + v5;
        v5 += 8;
        sub_f330("<",v11,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26258 + v5) + -1) == '\n') break;
label_ce70:
        v10 += 1;
        if (v7 < v10) goto label_cef0;
      }
      v3 = *(char **)((long)dat_26188 + 0x28);
      if (*(char **)((long)dat_26188 + 0x30) <= v3) {
        __overflow(dat_26188,10);
        goto label_ce70;
      }
      v10 += 1;
      *(char **)((long)dat_26188 + 0x28) = &v3[1];
      *v3 = 10;
    } while (v10 <= v7);
  }
label_cef0:
  if (v4 != 3) { // branch-flip
    if (!(v4 & 2))
      return;
  }
  else {
    fputs_unlocked("---\n",dat_26188);
  }
  if (v9 < v6)
    return;
  v5 = v6 * 8;
  v10 = v6;
  do {
    sub_f510(1);
    v11 = dat_26388 + v5;
    v5 += 8;
    sub_f330(">",v11,1);
    sub_f510(3);
    if (*(char *)(*(long *)(dat_26388 + v5) + -1) == '\n') {
      v3 = *(char **)((long)dat_26188 + 0x28);
      if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
        __overflow(dat_26188,10);
      else {
        *(char **)((long)dat_26188 + 0x28) = &v3[1];
        *v3 = 10;
      }
    }
    v10 += 1;
  } while (v10 <= v9);
}


// Function: sub_cff0 @ 0xcff0
void sub_cff0(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_cd40); // tail-call
}


// Function: sub_d010 @ 0xd010
unsigned long sub_d010(unsigned long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2;
  void *v3;
  unsigned long v4;
  unsigned long v5;
  
  v4 = dat_26498;
  v3 = dat_26188;
  if (!dat_264a0) {
    v5 = (dat_26498 + a0) - a0 % dat_26498;
    while (v2 = v5, v2 <= a1) {
      v1 = *(char **)((long)v3 + 0x28);
      if (*(char **)((long)v3 + 0x30) <= v1) // branch-flip
        __overflow(v3,9);
      else {
        *(char **)((long)v3 + 0x28) = &v1[1];
        *v1 = 9;
      }
      v5 = v4 + v2;
      a0 = v2;
    }
  }
  if (a0 < a1) {
    do {
      a0 += 1;
      v1 = *(char **)((long)v3 + 0x28);
      if (*(char **)((long)v3 + 0x30) <= v1) // branch-flip
        __overflow(v3,0x20);
      else {
        *(char **)((long)v3 + 0x28) = &v1[1];
        *v1 = 0x20;
      }
    } while (a1 != a0);
  }
  return a1;
}


// Function: sub_d0d0 @ 0xd0d0
unsigned long sub_d0d0(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  unsigned char v1;
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned char *v2;
  char *v3;
  void *v4;
  int v5; // eax
  unsigned long v6;
  long v7; // rcx
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x4c
  
  v4 = dat_26188;
  v6 = 0;
  v2 = (unsigned char *)a0[1];
  v8 = 0;
  if ((unsigned char *)*a0 < v2) {
    v10 = (unsigned char *)*a0;
    v12 = v6;
    v13 = 0;
    do {
      while( true ) {
        v1 = *v10;
        v11 = &v10[1];
        if ('_' < (char)v1) break;
        if ('\b' <= (char)v1) { // branch-flip
          switch(v1) {
            case 8:
              if (v13) {
                v13 -= 1;
                if (v13 < a2) {
                  if (v12 <= v13) { // branch-flip
                    if (v12 < v13) {
                      do {
                        v3 = *(char **)((long)v4 + 0x28);
                        if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                          __overflow(v4,0x20);
                        else {
                          *(char **)((long)v4 + 0x28) = &v3[1];
                          *v3 = 0x20;
                        }
                        v12 += 1;
                      } while (v13 != v12);
                    }
                  }
                  else {
                    v3 = *(char **)((long)v4 + 0x28);
                    v12 = v13;
                    if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                      __overflow(v4,8);
                    else {
                      *(char **)((long)v4 + 0x28) = &v3[1];
                      *v3 = 8;
                    }
                  }
                }
              }
              break;
            case 9:
              v7 = dat_26498 - v13 % dat_26498;
              if (v12 == v13) {
                v6 = v7 + v12;
                if (dat_264a0) { // branch-flip
                  if (a2 <= v6)
                    v6 = a2;
                  if (v12 < v6) {
                    do {
                      v3 = *(char **)((long)v4 + 0x28);
                      if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                        __overflow(v4,0x20);
                      else {
                        *(char **)((long)v4 + 0x28) = &v3[1];
                        *v3 = 0x20;
                      }
                      v12 += 1;
                    } while (v12 != v6);
                  }
                }
                else if (v6 < a2) {
                  v3 = *(char **)((long)v4 + 0x28);
                  v12 = v6;
                  if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                    __overflow(v4,9);
                  else {
                    *(char **)((long)v4 + 0x28) = &v3[1];
                    *v3 = 9;
                  }
                }
              }
              v13 += v7;
              break;
            case 10:
              return v12;
            case 0xb:
            case 0xc:
              goto label_d1fd;
            case 0xd:
              v3 = *(char **)((long)v4 + 0x28);
              if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                __overflow(v4,0xd);
              else {
                *(char **)((long)v4 + 0x28) = &v3[1];
                *v3 = 0xd;
              }
              v13 = 0;
              sub_d010(0,a1);
              v12 = 0;
              break;
            default:
              goto label_d1c0;
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
              goto label_d15f;
            
          }
        }
        else {
label_d1c0:
          v6 = sub_17200(&v9,v10,(long)v2 - (long)v10,&v8);
          if (0xfffffffffffffffd <= v6 - 1) { // branch-flip
label_d1fd:
            if (v13 < a2) {
              v10 = *(unsigned char **)((long)v4 + 0x28);
              if (*(unsigned char **)((long)v4 + 0x30) <= v10) // branch-flip
                __overflow(v4,(unsigned int)v1);
              else {
                *(unsigned char **)((long)v4 + 0x28) = &v10[1];
                *v10 = v1;
              }
            }
          }
          else {
            v5 = wcwidth(v9);
            if (1 <= v5)
              v13 = (long)v5 + v13;
            if (v13 <= a2) {
              fwrite_unlocked(v10,1,v6,stdout);
              v12 = v13;
            }
            v11 = &v10[v6];
          }
        }
label_d220:
        v10 = v11;
        if (v2 <= v11)
          return v12;
      }
      if (0x1d < (unsigned char)(v1 + 0x9f)) goto label_d1c0;
label_d15f:
      v6 = v13 + 1;
      if (a2 <= v13) {
        v13 = v6;
        goto label_d220;
      }
      v10 = *(unsigned char **)((long)v4 + 0x28);
      v12 = v6;
      if (*(unsigned char **)((long)v4 + 0x30) <= v10) {
        __overflow(v4,(unsigned int)v1);
        v13 = v6;
        goto label_d220;
      }
      *(unsigned char **)((long)v4 + 0x28) = &v10[1];
      *v10 = v1;
      v10 = v11;
      v13 = v6;
    } while (v11 < v2);
  }
  return v6;
}


// Function: sub_d500 @ 0xd500
void sub_d500(long a0,char a1,unsigned long *a2)
{
  char *v1;
  char v10; // stack - 0x39
  char *v2;
  bool v3;
  void *v4;
  long v5;
  long v6;
  long v7;
  int v8; // esi
  bool v9;
  
  v6 = dat_26430;
  v5 = dat_26428;
  v4 = dat_26188;
  v10 = a1;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_f510(1);
      goto label_d67a;
    }
    v3 = 0;
    if (a0) goto label_d556;
    v9 = 0;
    v7 = 0;
label_d5f8:
    if (a1 == ' ') goto label_d57b;
    v7 = sub_d010(v7,(unsigned long)(v6 + -1 + v5) >> 1) + 1;
    if (a1 != '|') goto label_d6a2;
    v8 = 0x7c;
    if ((*(char *)(a2[1] + -1) == '\n') != v9) {
      v8 = (-(unsigned int)(v9 == 0) & 0x2d) + 0x2f;
      v10 = (-(v9 == 0) & 0x2dU) + 0x2f;
    }
  }
  else {
    sub_f510(2);
label_d67a:
    v3 = 1;
    if (a0) {
label_d556:
      v9 = *(char *)(*(long *)(a0 + 8) + -1) == '\n';
      v7 = sub_d0d0(a0,0,v6);
      goto label_d5f8;
    }
    v9 = 0;
    v7 = sub_d010(0,(unsigned long)(v6 + -1 + v5) >> 1) + 1;
label_d6a2:
    v8 = (int)a1;
  }
  v2 = *(char **)((long)v4 + 0x28);
  if (*(char **)((long)v4 + 0x30) <= v2) // branch-flip
    __overflow(v4,v8);
  else {
    *(char **)((long)v4 + 0x28) = &v2[1];
    *v2 = v10;
  }
label_d57b:
  if ((a2) && (v9 = (bool)(v9 | *(char *)(a2[1] + -1) == '\n'), *(char *)*a2 != '\n'))
    sub_d0d0(a2,sub_d010(v7,v5),v6);
  if (v9) {
    v1 = *(char **)((long)v4 + 0x28);
    if (*(char **)((long)v4 + 0x30) <= v1) // branch-flip
      __overflow(v4,10);
    else {
      *(char **)((long)v4 + 0x28) = &v1[1];
      *v1 = 10;
    }
  }
  if (v3) {
    sub_f510(3); // tail-call
    return;
  }
}


// Function: sub_d700 @ 0xd700
void sub_d700(long a0,long a1) // return-dupe x3
{
  long v1;
  long v2;
  long v3;
  bool v4; // zf
  bool v5; // zf
  
  v3 = dat_26730;
  v2 = dat_26728;
  if (dat_26438) {
    dat_26730 = a0;
    dat_26728 = a1;
    return;
  }
  v4 = dat_26730 != a0;
  v5 = dat_26728 != a1;
  if ((!v4) && (!v5)) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  if (dat_2643a)
    __fprintf_chk(dat_26188,1,"i%ld,%ld\n",a0 - dat_26730,a1 - dat_26728);
  if (dat_26439) goto label_d81b;
  if ((v4) && (v5)) {
    while( true ) {
      v2 += 1;
      v3 += 1;
      sub_d500(dat_26258 + -8 + v3 * 8,0x20,dat_26388 + -8 + v2 * 8);
      if (a0 == v3) break;
      if (a1 == v2) goto label_d81b;
    }
  }
  if (a1 == v2) goto label_d81b;
  do {
    v1 = v2 * 8;
    v2 += 1;
    sub_d500(0,0x29,dat_26388 + v1);
  } while (a1 != v2);
  if (a0 == v3) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  do {
    v2 = v3 * 8;
    v3 += 1;
    sub_d500(dat_26258 + v2,0x28,0);
label_d81b:
  } while (a0 != v3);
  dat_26728 = a1;
  dat_26730 = a0;
}


// Function: sub_d860 @ 0xd860
void sub_d860(unsigned long a0) // return-dupe x2, return-dupe
{
  unsigned int v1;
  long v2;
  long v3;
  long v4;
  long v5; // stack - 0x30
  long v6; // stack - 0x38
  long v7; // stack - 0x40
  long v8; // stack - 0x28
  long v9;
  
  v1 = sub_f670(a0,&v7,&v6,&v5,&v8);
  if (!v1)
    return;
  sub_d700(v7,v5);
  if (dat_2643a)
    __fprintf_chk(dat_26188,1,"c%ld,%ld\n",(v6 - v7) + 1,(v8 - v5) + 1);
  if (v1 == 3) {
    v3 = v8;
    v2 = v6;
    v4 = v7;
    v9 = v5;
    if (v7 <= v6) {
      do {
        v3 = v8;
        v2 = v6;
        v7 = v4;
        v5 = v9;
        if (v9 > v8) goto label_d9cf;
        v3 = v9 * 8;
        v9 += 1;
        v2 = v4 * 8;
        v4 += 1;
        sub_d500(dat_26258 + v2,0x7c,dat_26388 + v3);
      } while (v6 >= v4);
      v3 = v8;
      v2 = v6;
      v7 = v4;
      v5 = v9;
    }
label_d9cf:
    v1 = (unsigned int)(v7 <= v2) + (unsigned int)(v5 <= v3) * 2;
    dat_26728 = v5;
    dat_26730 = v7;
  }
  if (v1 & 2) {
    v3 = v5;
    v2 = v5;
    if (v5 <= v8) {
      do {
        v3 = v2 + 1;
        sub_d500(0,0x3e,dat_26388 + v2 * 8);
        v2 = v3;
      } while (v3 <= v8);
    }
    dat_26728 = v3;
  }
  if (!(v1 & 1))
    return;
  v3 = v7;
  if (v6 >= v7) {
    do {
      v2 = v3 * 8;
      v3 += 1;
      sub_d500(dat_26258 + v2,0x3c,0);
    } while (v3 <= v6);
    dat_26730 = v3;
    return;
  }
  dat_26730 = v7;
}


// Function: sub_da50 @ 0xda50
void sub_da50(unsigned long a0)
{
  sub_e830();
  dat_26728 = -dat_26288;
  dat_26730 = dat_26728;
  sub_f080(a0,sub_f060,sub_d860);
  sub_d700(dat_26270,dat_263a0); // tail-call
}


// Function: sub_daa0 @ 0xdaa0
void sub_daa0(unsigned int a0) // return-dupe
{
  if (dat_26774)
    return;
  dat_26774 = a0;
}


// Function: sub_dac0 @ 0xdac0
unsigned int sub_dac0(long *a0,long *a1,unsigned int a2,long *a3)
{
  unsigned char v1;
  long v10;
  long v11;
  char *v12;
  bool v13;
  unsigned int v2; // eax
  char *v3;
  char *v4;
  char v5;
  char v6;
  long v7;
  long v8;
  long v9; // rdi
  
  v3 = (char *)*a1;
  v11 = *a0;
  v9 = 1;
  v5 = *v3;
  v10 = 0;
  v8 = v11 + 1;
  v6 = v5 + '\xa4';
  v13 = 0;
  v4 = v3;
  v7 = v8;
  if (v5 == '\\') goto label_db26;
label_dafa:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_db5e:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_db5e;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_db16;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_db16;
    }
    v2 = 0;
    goto label_db5e;
  }
  v3 = &v3[1];
  v8 = v7;
label_db0b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_db16:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_dafa;
label_db26:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_db5e;
    }
    v5 = v6 + '\xd0';
    switch(v5) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
      case 6:
      case 7:
        v6 = *v3;
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_db0b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_db16;
      default:
        goto label_dba0;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_dc37;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_dba0;
      case 0x32:
        v6 = '\b';
        goto label_dba0;
      case 0x35:
        v6 = '\x1b';
        break;
      case 0x36:
        v6 = '\f';
        break;
      case 0x3e:
        v6 = '\n';
        break;
      case 0x42:
        v6 = '\r';
        break;
      case 0x44:
        v6 = '\t';
        break;
      case 0x46:
        v6 = '\v';
label_dba0:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_dc37:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_dc82;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_db0b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_dc82:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_db0b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}


// Function: sub_dd30 @ 0xdd30
void sub_dd30(char a0) // return-dupe
{
  char v1;
  void *v10; // stack - 0xe8
  int *v11; // rbp
  char *v12;
  bool v13;
  unsigned long v14; // stack - 0xe0
  unsigned long v15; // stack - 0xd8
  unsigned long v16; // stack - 0xd0
  unsigned long v17; // stack - 0xc8
  unsigned long v18; // stack - 0xc0
  unsigned long v19; // stack - 0xb8
  int v2;
  unsigned long v20; // stack - 0xb0
  unsigned long v21; // stack - 0xa8
  unsigned long v22; // stack - 0xa0
  unsigned long v23; // stack - 0x98
  unsigned long v24; // stack - 0x90
  unsigned long v25; // stack - 0x88
  unsigned long v26; // stack - 0x80
  unsigned long v27; // stack - 0x78
  unsigned long v28; // stack - 0x70
  unsigned long v29; // stack - 0x68
  void *v3;
  unsigned int v30; // stack - 0x60
  char v31; // stack - 0x41
  void *v4;
  long v5; // rcx
  int *v6; // rbx
  unsigned short v7; // stack - 0x43
  char *v8; // stack - 0xf8
  void *v9; // stack - 0xf0
  
  if (!dat_26188)
    return;
  if (!dat_2657c)
    return;
  if (dat_26180) { // branch-flip
    v13 = 1;
label_dd84:
    if (dat_2657c == 2) goto label_dd9f;
    if ((dat_2657c == 1) && (v13)) {
      v13 = 1;
      goto label_dd9f;
    }
  }
  else {
    if (!a0) {
      v2 = fileno(dat_26188);
      v13 = isatty(v2) != 0;
      goto label_dd84;
    }
    if (dat_2657c != 2) {
      dat_26758 = 0;
      return;
    }
    v13 = 0;
label_dd9f:
    v12 = dat_26748;
    dat_26758 = 1;
    v8 = dat_26748;
    if ((!dat_26748) || (!*dat_26748)) goto label_df18;
    v31 = 0;
    v7 = 0x3f3f;
    v3 = (void *)sub_16ea0(dat_26748);
    v9 = v3;
label_ddf0:
    while (v1 = *v12, v1 == '*') {
      v4 = (void *)sub_16a40(0x28);
      *(void **)((long)v4 + 0x20) = dat_26750;
      *(void **)((long)v4 + 8) = v9;
      dat_26750 = v4;
      v8 = &v12[1];
      if ((!sub_dac0(&v9,&v8,1,v4)) || (v12 = &v8[1], v1 = *v8, v8 = v12, v1 != '=')) goto label_deca;
      *(void **)((long)v4 + 0x18) = v9;
      if (!sub_dac0(&v9,&v8,0,(long)v4 + 0x10)) goto label_deca;
label_e14b:
      v12 = v8;
    }
    if (v1 == ':') {
      v12 = &v12[1];
      v8 = v12;
      goto label_ddf0;
    }
    if (!v1) goto label_df18;
    v7 = CONCAT11(v7._1_1_,v1);
    v8 = &v12[1];
    if (v12[1]) {
      v7 = CONCAT11(v12[1],v1);
      v8 = &v12[3];
      if (v12[2] == '=') {
        v5 = 0;
        v12 = "lc";
        while (strcmp((char *)&v7,v12)) {
          v5 += 1;
          v12 = *(char **)(v5 * 8 + 0x25920);
          if (!v12) goto label_dea6;
        }
        v5 = (long)(int)v5 * 0x10;
        *(void **)(v5 + 0x26028) = v9;
        if (sub_dac0(&v9,&v8,0,v5 + 0x26020)) goto label_e14b;
label_dea6:
        error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),&v7);
      }
    }
label_deca:
    error(0,0,dcgettext(NULL,"unparsable value for --palette",5));
    free(v3);
    v3 = dat_26750;
    while (v3) {
      v4 = *(void **)((long)v3 + 0x20);
      free(v3);
      v3 = v4;
    }
  }
  dat_26758 = 0;
label_df18:
  if (!v13)
    return;
  v2 = 0x14;
  v6 = (int *)0x1e524;
  sigemptyset((void *)0x26780);
  v11 = (int *)0x1e524;
  while( true ) {
    sigaction(v2,NULL,&v10);
    if (v10 != (void *)0x1)
      sigaddset((void *)0x26780,v2);
    if (v11 == (int *)0x1e550) break;
    v2 = *v11;
    v11 = &v11[1];
  }
  v30 = 0x10000000;
  v2 = 0x14;
  v14 = dat_26780;
  v15 = dat_26788;
  v16 = dat_26790;
  v17 = dat_26798;
  v18 = dat_267a0;
  v19 = dat_267a8;
  v20 = dat_267b0;
  v21 = dat_267b8;
  v22 = dat_267c0;
  v23 = dat_267c8;
  v24 = dat_267d0;
  v25 = dat_267d8;
  v26 = dat_267e0;
  v27 = dat_267e8;
  v28 = dat_267f0;
  v29 = dat_267f8;
  while( true ) {
    if (sigismember(0x26780,v2)) {
      v10 = sub_daa0;
      if (v2 == 0x14)
        v10 = sub_e1a0;
      sigaction(v2,&v10,NULL);
    }
    if (v6 == (int *)0x1e550) break;
    v2 = *v6;
    v6 = &v6[1];
  }
}


// Function: sub_e180 @ 0xe180
void sub_e180(unsigned long *a0)
{
  fwrite_unlocked((void *)a0[1],*a0,1,dat_26188); // tail-call
}


// Function: sub_e1c0 @ 0xe1c0
void sub_e1c0(void)
{
  char v1 [136];
  int v2;
  
  while ((dat_26774 || (dat_26770))) {
    if ((dat_26758) && (dat_26010 != 3)) {
      sub_e180(0x26020);
      sub_e180(0x26050);
      sub_e180(0x26030);
      dat_26010 = 3;
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(void *)0x26780,v1);
    v2 = dat_26774;
    if (dat_26770) { // branch-flip
      dat_26770 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_26774,0);
    }
    raise(v2);
    sigprocmask(2,v1,NULL);
  }
}


// Function: sub_e2e0 @ 0xe2e0
unsigned char * sub_e2e0(unsigned char *a0)
{
  bool v1;
  unsigned char *v2;
  unsigned char v3;
  unsigned char *v4; // rax
  unsigned char *v5; // rdx
  long v6; // rsi
  
  v3 = *a0;
  if (v3) {
    v6 = 0;
    v1 = 0;
    v2 = a0;
label_e314:
    do {
      v4 = v2;
      if (v3 != 0x20) { // branch-flip
        if (v3 != 0x22) {
          if ('#' <= (char)v3) { // branch-flip
            if (v3 != 0x5c) {
              v3 = v4[1];
              v2 = &v4[1];
              if (!v3) break;
              goto label_e314;
            }
          }
          else if (7 <= (unsigned char)(v3 - 7)) {
            if ((char)v3 <= '\x1f')
              v6 += 3;
            goto label_e308;
          }
        }
        v6 += 1;
      }
      else {
        v1 = 1;
      }
label_e308:
      v3 = v4[1];
      v2 = &v4[1];
    } while (v3);
    if ((v6) || (v1)) {
      v4 = (unsigned char *)sub_16a40(&v4[v6 + (4U - (long)a0)]);
      *v4 = 0x22;
      v3 = *a0;
      v2 = &v4[1];
      do {
        if (!v3) {
          v2[0] = 0x22;
          v2[1] = 0;
          return v4;
        }
        if ('\"' < (char)v3) {
          if (v3 == 0x5c) goto label_e416;
label_e484:
          *v2 = v3;
          v5 = &v2[1];
          goto label_e3d9;
        }
        if ((char)v3 <= '\x06') {
label_e3a8:
          if ('\x1f' < (char)v3) goto label_e484;
          *v2 = 0x5c;
          v5 = &v2[4];
          v2[1] = (v3 >> 6) + 0x30;
          v2[3] = (v3 & 7) + 0x30;
          v2[2] = ((char)v3 >> 3 & 7U) + 0x30;
          goto label_e3d9;
        }
        switch(v3) {
          case 7:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x61;
            break;
          case 8:
            v3 = 0x62;
            goto label_e416;
          case 9:
            v3 = 0x74;
            goto label_e416;
          case 10:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x6e;
            break;
          case 0xb:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x76;
            break;
          case 0xc:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x66;
            break;
          case 0xd:
            v3 = 0x72;
            goto label_e416;
          default:
            goto label_e3a8;
          case 0x22:
label_e416:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = v3;
          
        }
label_e3d9:
        v3 = a0[1];
        a0 = &a0[1];
        v2 = v5;
      } while( true );
    }
  }
  return a0;
}


// Function: sub_e490 @ 0xe490
void sub_e490(unsigned long a0)
{
  error(0,*__errno_location(),"%s",a0); // tail-call
}


// Function: sub_e4c0 @ 0xe4c0
void sub_e4c0(char *a0,long a1,unsigned long a2,char *a3,char *a4)
{
  char *v1;
  long v10 [2]; // stack - 0x80
  char *v11;
  long v12;
  char *v13; // stack - 0x70
  char *v14; // stack - 0x68
  unsigned long v2;
  unsigned long *v3;
  long v4; // rax
  unsigned long *v5; // rax
  void *v6; // rax
  unsigned long *v7;
  long v8;
  unsigned long v9 [5];
  
  if (dat_26480) {
    v12 = 8;
    if (!a3)
      a3 = "";
    if (!a4)
      a4 = "";
    v8 = 0;
    v11 = a0;
    v10[0] = a1;
    v10[1] = a2;
    v13 = a3;
    v14 = a4;
    while( true ) {
      v4 = strlen(v11) + 1;
      *(long *)((long)v9 + v8) = v4;
      v12 += v4;
      if (v8 + 8 == 0x28) break;
      v11 = *(char **)((long)v10 + v8);
      v8 += 8;
    }
    v5 = (unsigned long *)sub_16a40(v12);
    v7 = &v5[1];
    v12 = 0;
    while( true ) {
      v2 = *(unsigned long *)((long)v9 + v12);
      v6 = memcpy(v7,a0,v2);
      v3 = dat_260a0;
      v7 = (unsigned long *)((long)v6 + v2);
      if (v12 + 8 == 0x28) break;
      a0 = *(char **)((long)v10 + v12);
      v12 += 8;
    }
    dat_260a0 = v5;
    *v3 = v5;
    *v5 = 0;
    return;
  }
  if (dat_2643a) {
    v1 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v1) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)((long)stdout + 0x28) = &v1[1];
      *v1 = 0x20;
    }
  }
  __printf_chk(1,dcgettext(NULL,a0,5),a1,a2,a3,a4); // tail-call
}


// Function: sub_e660 @ 0xe660
void sub_e660(void)
{
  sub_e4c0(); // tail-call
}


// Function: sub_e670 @ 0xe670
void sub_e670(void) // return-dupe
{
  long *v1;
  long *v10;
  long *v11;
  unsigned long v12; // stack - 0x60
  unsigned long v13; // stack - 0x58
  unsigned long v14; // stack - 0x50
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  long v5;
  unsigned long v6; // rax
  long *v7;
  long v8;
  long v9; // stack - 0x48
  
  if (!dat_26800)
    return;
  v10 = dat_26800;
  do {
    v1 = (long *)*v10;
    v7 = &v10[1];
    v11 = &v8;
    do {
      v11 = &v11[1];
      v6 = strlen((char *)v7);
      v5 = v9;
      v4 = v14;
      v3 = v13;
      v2 = v12;
      v7 = (long *)((long)v7 + v6 + 1);
      *v11 = (long)v7;
    } while (&v9 != v11);
    __printf_chk(1,dcgettext(NULL,(char *)&v10[1],5),v2,v3,v4,v5);
    free(v10);
    v10 = v1;
  } while (v1);
}


// Function: sub_e770 @ 0xe770
void sub_e770(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_e670();
  error(2,v1,"%s",a0);
}


// Function: sub_e7b0 @ 0xe7b0
void sub_e7b0(char *a0)
{
  sub_e670();
  error(2,0,"%s",dcgettext(NULL,a0,5));
}


// Function: sub_e7f0 @ 0xe7f0
void sub_e7f0(unsigned long a0)
{
  dat_26748 = a0;
}


// Function: sub_e800 @ 0xe800
void sub_e800(unsigned long a0,unsigned long a1,char a2)
{
  dat_26768 = a0;
  dat_26760 = a1;
  dat_26759 = a2;
  dat_26188 = 0;
}


// Function: sub_e830 @ 0xe830
void sub_e830(void) // return-dupe x2
{
  int v1; // eax
  unsigned long v10; // stack - 0x30
  void *v2; // rax
  void *v3; // stack - 0x58
  int v4; // stack - 0x60
  char *v5; // stack - 0x48
  int v6; // stack - 0x5c
  void *v7; // stack - 0x50
  char *v8; // stack - 0x40
  void *v9; // stack - 0x38
  
  if (dat_26188)
    return;
  v3 = (void *)sub_e2e0(dat_26768);
  v7 = (void *)sub_e2e0(dat_26760);
  v2 = (void *)sub_17020("diff%s %s %s",dat_26420,v3,v7);
  if (dat_26480) { // branch-flip
    v1 = fflush_unlocked(stdout);
    if (v1)
      sub_e770(dcgettext(NULL,"write failed",5)); // no-return
    v9 = v2;
    v5 = "/usr/bin/pr";
    v8 = "-h";
    v10 = 0;
    if (pipe(&v4))
      sub_e770("pipe"); // no-return
    dat_26740 = fork();
    if (dat_26740 < 0)
      sub_e770("fork"); // no-return
    if (!dat_26740) {
      close(v6);
      if (v4) {
        v1 = dup2(v4,0);
        if (v1 < 0)
          sub_e770("dup2"); // no-return
        close(v4);
      }
      execv("/usr/bin/pr",&v5);
      _exit((*__errno_location() == 2) + 0x7e); // no-return
    }
    close(v4);
    dat_26188 = fdopen(v6,"w");
    if (!dat_26188)
      sub_e770("fdopen"); // no-return
    sub_dd30(1);
  }
  else {
    dat_26188 = stdout;
    sub_dd30(0);
    if (dat_26759)
      __printf_chk(1,"%s\n",v2);
  }
  free(v2);
  if (dat_26580 != 2) { // branch-flip
    if (dat_26580 == 3)
      sub_8b80(0x261a0,&v3,1);
  }
  else {
    sub_8b80(0x261a0,&v3,0);
  }
  if (v3 != dat_26768)
    free(v3);
  if (v7 == dat_26760)
    return;
  free(v7);
}


// Function: sub_ead0 @ 0xead0
void sub_ead0(void)
{
  int v1; // eax
  unsigned char v2; // ah
  unsigned int v3;
  unsigned int v4; // stack - 0x14
  char *v5; // rsi
  
  if ((!dat_26188) || (dat_26188 == stdout)) {
    dat_26188 = NULL;
    return;
  }
  if (*dat_26188 & 0x20)
    sub_e7b0("write failed"); // no-return
  v1 = fclose(dat_26188);
  if (!v1) {
    v1 = waitpid(dat_26740,(int *)&v4,0);
    if (v1 < 0)
      sub_e770("waitpid"); // no-return
    if (v4 & 0x7f) { // branch-flip
      v5 = "subsidiary program \'%s\' failed";
      v3 = 0x7fffffff;
    }
    else {
      v2 = (unsigned char)(v4 >> 8);
      if (!v2) {
        dat_26188 = NULL;
        return;
      }
      v5 = "subsidiary program \'%s\' could not be invoked";
      v3 = (unsigned int)v2;
      if ((v2 != 0x7e) && (v5 = "subsidiary program \'%s\' failed (exit status %d)", v3 = (unsigned int)v2, v2 == 0x7f))
        v5 = "subsidiary program \'%s\' not found";
    }
    error(2,0,dcgettext(NULL,v5,5),"/usr/bin/pr",(unsigned long)v3);
    return;
  }
  sub_e770(dcgettext(NULL,"write failed",5)); // no-return
}


// Function: sub_ec00 @ 0xec00
unsigned long sub_ec00(unsigned char *a0,unsigned char *a1)
{
  unsigned char *v1;
  unsigned char v10;
  char v11; // bl
  unsigned long v12;
  unsigned char *v13;
  unsigned char *v14;
  unsigned long v15;
  unsigned char *v16;
  unsigned short v17; // r11w
  unsigned char *v18;
  unsigned long v19; // r8
  unsigned char v2;
  unsigned long v3;
  char v4;
  unsigned int v5;
  unsigned int v6; // eax
  long v7;
  unsigned char v8;
  bool v9;
  
  v5 = dat_26558;
  v4 = dat_26552;
  v3 = dat_26498;
  v19 = 0;
  v6 = dat_26558 - 2;
  v16 = a1;
  v18 = a0;
label_ec50:
  do {
    v8 = *v18;
    v10 = *v16;
    v12 = (unsigned long)v10;
    v14 = &v18[1];
    v13 = &v16[1];
    if (v8 == v10) goto label_ed3a;
    if (v5 != 4) {
      if (5 <= v5) { // branch-flip
        if (v5 == 5) {
          v7 = *(long *)__ctype_b_loc();
          while ((*(unsigned char *)(v7 + 1 + (unsigned long)v8 * 2) & 0x20 && (v8 != 10))) {
            v16 = &v14[1];
            v8 = *v14;
            v14 = v16;
          }
          v10 = *(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2);
          while ((v10 & 0x20 && ((char)v12 != '\n'))) {
            v12 = (unsigned long)*v13;
            v13 = &v13[1];
            v10 = *(unsigned char *)(v7 + 1 + v12 * 2);
          }
        }
      }
      else if (v5 != 1) { // branch-flip
        if (v6 <= 1) {
          v7 = *(long *)__ctype_b_loc();
          if ((*(unsigned char *)(v7 + 1 + (unsigned long)v8 * 2) & 0x20) && (*(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2) & 0x20)) {
            if (v8 != 10) {
              v2 = v18[1];
              v18 = v14;
              while (v2 != 10) {
                if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v2 * 2) & 0x20)) goto label_ed08;
                v1 = &v18[1];
                v18 = &v18[1];
                v2 = *v1;
              }
            }
            if ((v10 == 10) || (v10 = v16[1], v10 == 10))
              return 0;
            v16 = v13;
            while (*(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2) & 0x20) {
              v10 = v16[1];
              v16 = &v16[1];
              if (v10 == 10)
                return 0;
            }
          }
          else if (v5 != 2) goto label_edf0;
        }
      }
      else {
label_edf0:
        if ((v8 == 0x20) && (v10 == 9)) {
label_ee11:
          v15 = v19;
          do {
            if (v8 != 0x20) { // branch-flip
              if (v8 != 9) goto label_ee6c;
              v15 = (v15 + v3) - v15 % v3;
            }
            else {
              v15 += 1;
            }
            v8 = *v14;
            v14 = &v14[1];
          } while( true );
        }
        if ((v8 == 9) && (v10 == 0x20)) {
          v8 = 9;
          goto label_ee11;
        }
      }
      goto label_ed08;
    }
    v7 = *(long *)__ctype_b_loc();
    v15 = (unsigned long)v8;
    v17 = *(unsigned short *)(v7 + (unsigned long)v10 * 2) & 0x2000;
    if (*(unsigned char *)(v7 + 1 + v15 * 2) & 0x20) { // branch-flip
      while ((char)v15 != '\n') {
        v15 = (unsigned long)*v14;
        v16 = &v14[1];
        if (!(*(unsigned char *)(v7 + 1 + v15 * 2) & 0x20)) {
          v15 = 0x20;
          break;
        }
        v14 = v16;
      }
      v8 = (unsigned char)v15;
      if (v17) goto label_ef8f;
label_ef65:
      if ((unsigned char)v12 == v8) goto label_ed08;
    }
    else if (v17) {
label_ef8f:
      while (v8 = (unsigned char)v15, (char)v12 != '\n') {
        v12 = (unsigned long)*v13;
        v16 = &v13[1];
        if (!(*(unsigned char *)(v7 + 1 + v12 * 2) & 0x20)) {
          v12 = 0x20;
          goto label_ef65;
        }
        v13 = v16;
      }
      goto label_ed08;
    }
    v11 = (char)v12;
    v9 = v11 == ' ' && v8 != 10;
    if (v11 != ' ' || v8 == 10) {
      v9 = v11 != '\n';
      break;
    }
    if (v14 <= &a0[1]) {
      v12 = 0x20;
      break;
    }
    v12 = 0x20;
    v16 = v13;
    v18 = &v14[-1];
  } while (*(unsigned char *)(v7 + 1 + (unsigned long)v14[-2] * 2) & 0x20);
  if ((v8 == 0x20) && (v9)) {
    if (v13 <= &a1[1]) // branch-flip
      v8 = 0x20;
    else {
      v16 = &v13[-1];
      v18 = v14;
      if (*(unsigned char *)(v7 + 1 + (unsigned long)v13[-2] * 2) & 0x20) goto label_ec50;
    }
  }
label_ed08:
  if (v4) {
    v7 = *(long *)__ctype_tolower_loc();
    v8 = *(unsigned char *)(v7 + (unsigned long)v8 * 4);
    v12 = (unsigned long)*(unsigned char *)(v7 + v12 * 4);
  }
  v10 = (unsigned char)v12;
  if (v8 != v10)
    return 1;
label_ed3a:
  if (v10 == 10)
    return 0;
  v7 = 1;
  if (v10 == 9)
    v7 = v3 - v19 % v3;
  v19 += v7;
  v16 = v13;
  v18 = v14;
  goto label_ec50;
label_ee6c:
  if ((char)v12 != ' ') { // branch-flip
    if ((char)v12 != '\t') goto label_efa0;
    v19 = (v3 + v19) - v19 % v3;
  }
  else {
    v19 += 1;
  }
  v12 = (unsigned long)*v13;
  v13 = &v13[1];
  goto label_ee6c;
label_efa0:
  if (v15 != v19)
    return 1;
  goto label_ed08;
}


// Function: sub_f080 @ 0xf080
void sub_f080(long a0,void *a1,void *a2)
{
  long v1;
  long *v2; // rax
  
  if (a0) {
    do {
      v2 = (long *)(*a1)(a0);
      v1 = *v2;
      *v2 = 0;
      (*a2)(a0);
      *v2 = v1;
      a0 = v1;
    } while (v1);
    return;
  }
}


// Function: sub_f0e0 @ 0xf0e0
void sub_f0e0(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char v1;
  unsigned char *v2;
  char *v3;
  void *v4;
  unsigned long v5; // rax
  unsigned long v6;
  long v7; // rcx
  unsigned long v8;
  long v9; // r14
  
  v6 = dat_26498;
  v4 = dat_26188;
  if (!dat_264a0) {
    v6 = (long)a1 - (long)a0;
    dat_264a0 = 0;
    while( true ) {
      if (!v6)
        return;
      v8 = 0x400;
      if (v6 <= 0x400)
        v8 = v6;
      v5 = fwrite_unlocked(a0,1,v8,dat_26188);
      if (v5 < v8) break;
      v6 -= v5;
      a0 = &a0[v5];
      sub_e1c0();
    }
    return;
  }
  v8 = 0;
  v9 = 1;
  if (a1 <= a0)
    return;
label_f1a0:
  v1 = *a0;
  a0 = &a0[1];
  if (v1 == 9) goto label_f23a;
  do {
    if (v1 != 0xd) { // branch-flip
      if (v1 != 8) { // branch-flip
        v8 += *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v1 * 2) >> 0xe & 1;
        v2 = *(unsigned char **)((long)v4 + 0x28);
        if (*(unsigned char **)((long)v4 + 0x30) <= v2) { // branch-flip
label_f310:
          __overflow(v4,(unsigned int)v1);
        }
        else {
          *(unsigned char **)((long)v4 + 0x28) = &v2[1];
          *v2 = v1;
        }
      }
      else {
        if (!v8) goto label_f300;
        v3 = *(char **)((long)v4 + 0x28);
        v8 -= 1;
        if (*(char **)((long)v4 + 0x30) <= v3) goto label_f310;
        *(char **)((long)v4 + 0x28) = &v3[1];
        *v3 = 8;
      }
    }
    else {
      v3 = *(char **)((long)v4 + 0x28);
      if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
        __overflow(v4,0xd);
      else {
        *(char **)((long)v4 + 0x28) = &v3[1];
        *v3 = 0xd;
      }
      if ((a2) && (a0 < a1)) {
        v8 = 0;
        if (*a0 == 10) goto label_f213;
        __fprintf_chk(v4,1,a2,a3);
        goto label_f213;
      }
label_f300:
      v8 = 0;
    }
    while( true ) {
      if (a1 == a0)
        return;
label_f213:
      v9 += 1;
      if (v9 != 0x400) goto label_f1a0;
      sub_e1c0();
      v1 = *a0;
      a0 = &a0[1];
      v9 = 0;
      if (v1 != 9) break;
label_f23a:
      v7 = v6 - v8 % v6;
      v8 += v7;
      do {
        v3 = *(char **)((long)v4 + 0x28);
        if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
          __overflow(v4,0x20);
        else {
          *(char **)((long)v4 + 0x28) = &v3[1];
          *v3 = 0x20;
        }
        v7 -= 1;
      } while (v7);
    }
  } while( true );
}


// Function: sub_f330 @ 0xf330
void sub_f330(char *a0,unsigned long *a1,char a2)
{
  char v1;
  char *v2;
  long v3;
  void *v4;
  char *v5;
  char *v6;
  char *v7; // r13
  
  v4 = dat_26188;
  v2 = (char *)*a1;
  v3 = a1[1];
  if (a0) { // branch-flip
    v7 = NULL;
    if (*a0) {
      v7 = "%s\t";
      if (!dat_26492)
        v7 = "%s ";
      v5 = a0;
      v6 = v7;
      if ((dat_26491) && (*v2 == '\n')) {
        v6 = "%s";
        v5 = &a0[*a0 == ' '];
      }
      __fprintf_chk(dat_26188,1,v6,v5);
    }
    if (a2) { // branch-flip
      sub_f0e0(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),v7,a0);
      v1 = *a0;
    }
    else {
      sub_f0e0(v2,v3,v7,a0);
      v1 = *a0;
    }
    if (!v1)
      return;
  }
  else if (a2) // branch-flip
    sub_f0e0(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),0,0);
  else {
    sub_f0e0(v2,v3,0,0);
  }
  if (*(char *)(v3 + -1) == '\n')
    return;
  if ((dat_26758) && (dat_26010 != 3)) {
    sub_e180(0x26020);
    sub_e180(0x26050);
    sub_e180(0x26030);
    dat_26010 = 3;
  }
  __fprintf_chk(v4,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5)); // tail-call
}


// Function: sub_f500 @ 0xf500
void sub_f500(unsigned long a0,unsigned long a1)
{
  sub_f330(a0,a1,0); // tail-call
}


// Function: sub_f610 @ 0xf610
void sub_f610(char a0,long a1,long a2,long a3)
{
  a2 = *(long *)(a1 + 0xe8) + 1 + a2;
  a3 = *(long *)(a1 + 0xe8) + 1 + a3;
  if (a2 < a3) {
    __fprintf_chk(dat_26188,1,"%ld%c%ld",a2,(unsigned long)(unsigned int)(int)a0); // tail-call
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld",a3); // tail-call
}


// Function: sub_f670 @ 0xf670
unsigned char sub_f670(long *a0,long *a1,long *a2,long *a3,long *a4)
{
  long v1;
  unsigned char *v10;
  unsigned char v11;
  long v12;
  unsigned long v13;
  unsigned long v14; // r8
  long v15;
  unsigned char v16; // stack - 0x69
  int v17; // stack - 0x64
  long v18; // stack - 0x60
  long v19; // stack - 0x58
  long v2;
  unsigned char v3;
  unsigned char *v4;
  bool v5;
  bool v6;
  long v7;
  long v8;
  unsigned int v9; // eax
  
  v8 = dat_26388;
  v7 = dat_26258;
  v9 = (unsigned int)dat_26554;
  if (dat_26554) { // branch-flip
    if (2 <= dat_26558) { // branch-flip
      v17 = 0;
      v6 = 1;
      v5 = 3 < dat_26558;
    }
    else {
      v17 = 0;
      v6 = 0;
      v5 = 0;
    }
  }
  else {
    v17 = -1;
    v5 = 0;
    v9 = (unsigned int)(dat_264e0 != 0);
    v6 = 0;
  }
  v19 = 0;
  v18 = 0;
  *a1 = a0[3];
  *a3 = a0[4];
  do {
    v12 = a0[3];
    v15 = a0[4];
    v1 = a0[2] + -1 + v12;
    v19 += a0[1];
    v18 += a0[2];
    v2 = v15 + -1 + a0[1];
    v11 = v12 <= v1 & (unsigned char)v9;
    if (v11) { // branch-flip
      do {
        v15 = *(long *)(v7 + 8 + v12 * 8);
        v4 = *(unsigned char **)(v7 + v12 * 8);
        v15 = v15 + -1 + (unsigned long)(*(char *)(v15 + -1) != '\n');
        v14 = v15 - (long)v4;
        v13 = v14;
        if ((v6) && (v3 = *v4, v3 != 10)) {
          v10 = v4;
          do {
            if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v3 * 2) & 0x20)) {
              v13 = v15 - (long)v10;
              if (!v5)
                v13 = v14;
              goto label_f7b2;
            }
            v3 = v10[1];
            v10 = &v10[1];
          } while (v3 != 10);
          v13 = v15 - (long)v10;
        }
label_f7b2:
        if ((long)v17 != v13) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            goto label_f7cb;
          }
          v9 = (unsigned int)~re_search(0x264c0,v4,v14 & 0xffffffff,0,v14,0) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v11;
        }
        v12 += 1;
      } while ((v12 <= v1) && ((unsigned char)v9));
      v15 = a0[4];
      v16 = v15 <= v2 & (unsigned char)v9;
    }
    else {
      v16 = v15 <= v2 & (unsigned char)v9;
    }
    if (v16) {
      do {
        v12 = *(long *)(v8 + 8 + v15 * 8);
        v4 = *(unsigned char **)(v8 + v15 * 8);
        v12 = v12 + -1 + (unsigned long)(*(char *)(v12 + -1) != '\n');
        v14 = v12 - (long)v4;
        v13 = v14;
        if ((v6) && (v11 = *v4, v11 != 10)) {
          v10 = v4;
          do {
            if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v11 * 2) & 0x20)) {
              v13 = v12 - (long)v10;
              if (!v5)
                v13 = v14;
              goto label_f935;
            }
            v11 = v10[1];
            v10 = &v10[1];
          } while (v11 != 10);
          v13 = v12 - (long)v10;
        }
label_f935:
        if ((long)v17 != v13) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            break;
          }
          v9 = (unsigned int)~re_search(0x264c0,v4,v14 & 0xffffffff,0,v14,0) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v16;
        }
        v15 += 1;
        if ((v15 > v2) || (!(char)v9)) break;
      } while( true );
    }
label_f7cb:
    a0 = (long *)*a0;
    if (!a0) {
      v11 = 0;
      *a2 = v1;
      *a4 = v2;
      if (!(char)v9)
        v11 = (v19 != 0) * '\x02' | v18 != 0;
      return v11;
    }
  } while( true );
}


// Function: sub_fa30 @ 0xfa30
undefined16 sub_fa30(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [16];
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3._0_8_ = (char *)sub_16a40(v1 + v2 + 1 + strlen(a2));
  __sprintf_chk(v3._0_8_,1,0xffffffffffffffff,"%s%s%s",a0,a1,a2);
  v3._8_8_ = 0xfa6a;
  return v3._0_16_;
}


// Function: sub_fab0 @ 0xfab0
void sub_fab0(unsigned long a0)
{
  memset((void *)sub_16a40(),0,a0);
}


// Function: sub_fad0 @ 0xfad0
void sub_fad0(unsigned long *a0)
{
  fflush_unlocked(stdout);
  for (; a0; a0 = (unsigned long *)*a0) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",a0[3],a0[4],a0[2],a0[1]);
  }
  fflush_unlocked(stderr); // tail-call
}

