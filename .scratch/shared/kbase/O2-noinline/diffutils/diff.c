// Function: sub_4fa0 @ 0x4fa0
unsigned long sub_4fa0(int a0,char **a1)
{
  bool v1;
  long v10;
  unsigned long v11; // rcx
  long v12;
  char *v13; // stack - 0x48
  long v14; // stack - 0x58
  long v15; // stack - 0x50
  int v16; // ebp
  long v17;
  char *v18;
  unsigned long v19;
  bool v2;
  unsigned long v20; // stack - 0x88
  long v21; // stack - 0x80
  int v3; // eax
  int v4;
  unsigned long v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v17 = (long)a0;
  v14 = 0;
  v15 = 0;
  dat_260b0 = 2;
  sub_13d30(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_fa70(0);
  dat_26600 = 0x26500;
  dat_265c0 = 0x264c0;
  re_set_syntax(0x50a46);
  dat_26410 = sub_10050();
  dat_26180 = 0;
  sub_16390();
  v2 = 0;
  v20 = 0;
  v1 = 0;
  v21 = -1;
  v4 = -1;
label_50a0:
  v16 = (int)v17;
  v3 = getopt_long(v16,a1,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(void *)0x25160,NULL);
  if (v3 != -1) {
    switch(v3) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_8c00(0,0); // no-return
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
        if (10 <= (unsigned int)(v4 - 0x30U)) { // branch-flip
          v21 = (long)(v3 + -0x30);
          v4 = v3;
        }
        else if (0x666666666666666 <= (long)(v21 - (unsigned long)(v3 <= 0x33))) { // branch-flip
          v21 = 0x3fffffffffffffff;
          v4 = v3;
        }
        else {
          v21 = (long)(v3 + -0x30) + v21 * 10;
          v4 = v3;
        }
        goto label_50a0;
      case 0x42:
        dat_26554 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v12 = strtoimax(optarg,&v13,10);
          if ((*v13) || (v12 < 0))
            sub_8c00("invalid context length \'%s\'",optarg); // no-return
          v10 = 0x3fffffffffffffff;
          if (v12 <= 0x3fffffffffffffff)
            v10 = v12;
        }
        else {
          v10 = 3;
        }
        sub_8d50((v3 == 0x55) + '\x02');
        if (v10 <= dat_26570) { // branch-flip
          v1 = 1;
          v4 = v3;
        }
        else {
          v1 = 1;
          dat_26570 = v10;
          v4 = v3;
        }
        goto label_50a0;
      case 0x44:
        sub_8d50(7);
        v5 = strlen(optarg);
        v18 = (char *)sub_15f50(v5 * 7 + 0x61);
        __sprintf_chk(v18,1,0xffffffffffffffff,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        v17 = 0x26460;
        do {
          v10 = v17 + 8;
          sub_8c60(v17,v18,"-D");
          v18 = &v18[strlen(v18) + 1];
          v17 = v10;
        } while (v10 != 0x26480);
        v17 = (long)v16;
        v4 = v3;
        goto label_50a0;
      case 0x45:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 |= 1;
        goto label_50a0;
      case 0x46:
        sub_8930(0x265e0,optarg);
        v4 = v3;
        goto label_50a0;
      case 0x48:
        dat_26418 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x49:
        sub_8930(0x265a0,optarg);
        v4 = v3;
        goto label_50a0;
      case 0x4c:
        if (dat_26540) { // branch-flip
          if (dat_26548)
            sub_e230("too many file label options"); // no-return
          dat_26548 = optarg;
          v4 = v3;
        }
        else {
          dat_26540 = optarg;
          v4 = v3;
        }
        goto label_50a0;
      case 0x4e:
        dat_26586 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x50:
        dat_26585 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x53:
        sub_8c60(0x26488,optarg,"-S");
        v4 = v3;
        goto label_50a0;
      case 0x54:
        dat_26492 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x57:
        v5 = strtoimax(optarg,&v13,10);
        if (((long)v5 <= 0) || (*v13))
          sub_8c00("invalid width \'%s\'",optarg); // no-return
        if (v5 != v20) {
          if (v20)
            sub_e230("conflicting width options"); // no-return
          v20 = v5;
          v4 = v3;
          goto label_50a0;
        }
        break;
      case 0x58:
        v4 = sub_10770(sub_102e0,dat_26410,optarg,sub_8910());
        if (v4)
          sub_e1f0(optarg); // no-return
        break;
      case 0x5a:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 |= 2;
        goto label_50a0;
      case 0x61:
        dat_26568 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x62:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 = 4;
        goto label_50a0;
      case 99:
        v6 = 2;
        goto label_5820;
      case 100:
        dat_26408 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x65:
        sub_8d50(4);
        v4 = v3;
        goto label_50a0;
      case 0x66:
        sub_8d50(5);
        v4 = v3;
        goto label_50a0;
      case 0x69:
        dat_26552 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x6c:
        if (!s_1d558[0])
          sub_8c00("pagination not supported on this host",0); // no-return
        dat_26480 = 1;
        signal(0x11,0);
        v4 = v3;
        goto label_50a0;
      case 0x6e:
        sub_8d50(6);
        v4 = v3;
        goto label_50a0;
      case 0x70:
        sub_8930(0x265e0,"^[[:alpha:]$_]");
        v2 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x71:
        dat_264a1 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x72:
        dat_26608 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x73:
        dat_26584 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x74:
        dat_264a0 = '\x01';
        v4 = v3;
        goto label_50a0;
      case 0x75:
        v6 = 3;
label_5820:
        sub_8d50(v6);
        v4 = v3;
        if (dat_26570 <= 2)
          dat_26570 = 3;
        goto label_50a0;
      case 0x76:
        v6 = sub_14040("Len Tower");
        v7 = sub_14040("Richard Stallman");
        v8 = sub_14040("David Hayes");
        v9 = sub_14040("Mike Haertel");
        sub_15de0(stdout,"diff","GNU diffutils",dat_260a8,sub_14040("Paul Eggert"),v9,v8,v7,v6,0);
        sub_8e20();
        return 0;
      case 0x77:
        dat_26558 = 5;
        v4 = v3;
        goto label_50a0;
      case 0x78:
        sub_102e0(dat_26410,optarg,sub_8910());
        v4 = v3;
        goto label_50a0;
      case 0x79:
        sub_8d50(8);
        v4 = v3;
        goto label_50a0;
      case 0x81:
        sub_8c60(&v14,optarg,"--from-file");
        v4 = v3;
        goto label_50a0;
      case 0x82:
        sub_8a10();
        sub_8e20();
        return 0;
      case 0x83:
        v10 = strtoimax(optarg,&v13,10);
        if ((*v13) || (v10 < 0))
          sub_8c00("invalid horizon length \'%s\'",optarg); // no-return
        if (dat_26560 < v10)
          dat_26560 = v10;
        v4 = v3;
        goto label_50a0;
      case 0x84:
        dat_26551 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x86:
        dat_26439 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x87:
        sub_8d50(7);
        v10 = 0x26440;
        do {
          v12 = v10 + 8;
          sub_8c60(v10,optarg,"--line-format");
          v10 = v12;
        } while (v12 != 0x26458);
        v4 = v3;
        goto label_50a0;
      case 0x88:
        dat_26550 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x89:
        dat_26551 = 0;
        v4 = v3;
        goto label_50a0;
      case 0x8a:
        sub_8d50(1);
        v4 = v3;
        goto label_50a0;
      case 0x8b:
        sub_8d50(8);
        dat_2643a = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8c:
        dat_26490 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8d:
        dat_26491 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8e:
        dat_26438 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8f:
        v5 = strtoimax(optarg,&v13,10);
        if ((((long)v5 <= 0) || (0xfffffffffffffffd <= v5)) || (*v13))
          sub_8c00("invalid tabsize \'%s\'",optarg); // no-return
        if (v5 != dat_26498) {
          if (dat_26498)
            sub_e230("conflicting tabsize options"); // no-return
          dat_26498 = v5;
          v4 = v3;
          goto label_50a0;
        }
        break;
      case 0x90:
        sub_8c60(&v15,optarg,"--to-file");
        v4 = v3;
        goto label_50a0;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_8d50(7);
        v4 = v3 + -0x91;
        sub_8c60((long)v4 * 8 + 0x26440,optarg,&"--unchanged-line-format"[(long)v4 * 0x18]);
        goto label_50a0;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_8d50(7);
        v4 = v3 + -0x94;
        sub_8c60((long)v4 * 8 + 0x26460,optarg,&"--unchanged-group-format"[(long)v4 * 0x19]);
        goto label_50a0;
      case 0x98:
        sub_8cc0(optarg);
        v4 = v3;
        goto label_50a0;
      case 0x99:
        sub_e270(optarg);
        v4 = v3;
        goto label_50a0;
      case 0x9a:
        goto label_564a;
      
    }
    v4 = v3;
    goto label_50a0;
  }
  if (((dat_2657c == 1) && (v18 = getenv("TERM"), v18)) && (!strcmp(v18,"dumb")))
    dat_2657c = 0;
  if (dat_26580) { // branch-flip
label_5b6d:
    if ((dat_26580 != 2) || (sub_10ab0(2)))
      dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    else {
      dat_26400 = "%a %b %e %T %Y";
    }
    if (0 <= v21) { // branch-flip
label_59d6:
      v10 = dat_26570;
      if (((unsigned int)(dat_26580 - 2U) < 2) && ((dat_26570 < v21 || ((v21 != dat_26570 && (!v1)))))) {
        dat_26570 = v21;
        v10 = v21;
      }
    }
    else {
label_5b94:
      v10 = dat_26570;
    }
  }
  else {
    if (!v2) {
      sub_8d50(1);
      goto label_5b6d;
    }
    sub_8d50(2);
    if (0 <= v21) {
      if ((dat_26580 != 2) || (sub_10ab0(2)))
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_26400 = "%a %b %e %T %Y";
      }
      goto label_59d6;
    }
    dat_26570 = 3;
    if (dat_26580 == 2) {
      if (sub_10ab0(2)) {
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
        v10 = dat_26570;
        goto label_59ef;
      }
      dat_26400 = "%a %b %e %T %Y";
      goto label_5b94;
    }
    dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    v10 = 3;
  }
label_59ef:
  if (!dat_26498)
    dat_26498 = 8;
  if (v20) { // branch-flip
    v5 = 1;
    if (!dat_264a0) {
label_5a1f:
      v5 = dat_26498;
    }
    v11 = (v20 >> 1) + (v5 + 3 >> 1) + (unsigned long)((unsigned int)(v5 + 3) & (unsigned int)v20 & 1);
    v19 = v11 - v11 % v5;
    if ((4 <= v19) && (v19 < v20)) {
      dat_26430 = (v11 % v5 - v11) + v20;
      if (v19 - 3 < dat_26430)
        dat_26430 = v19 - 3;
      if (!dat_26430)
        v19 = v20;
      v20 = v19;
    }
    else {
      dat_26430 = 0;
    }
  }
  else {
    if (!dat_264a0) {
      v20 = 0x82;
      goto label_5a1f;
    }
    v20 = 0x43;
    dat_26430 = 0x3f;
  }
  dat_26428 = v20;
  if (dat_26560 < v10)
    dat_26560 = v10;
  sub_8ba0(0x265e0);
  sub_8ba0(0x265a0);
  if (dat_26580 != 7) {
label_5acd:
    dat_26578 = dat_26580 != 8 | dat_26438;
    goto label_5ad9;
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
label_5e60:
      dat_26478 = (char *)sub_f560(dat_26468,dat_26470,"");
      if (dat_26580 != 7) goto label_5acd;
    }
    v18 = dat_26460;
    v2 = 1;
    if (*dat_26460) goto label_5dae;
  }
  else {
    v18 = "%=";
    dat_26460 = "%=";
    if (!dat_26478) goto label_5e60;
label_5dae:
    v2 = 0;
    if (!strcmp(v18,"%="))
      v2 = *dat_26440 == '\0';
  }
  dat_26578 = v2;
label_5ad9:
  dat_26553 = ~(dat_26554 | dat_26552 | dat_26490 | (dat_265a0 || dat_26558)) & dat_264a1 & 1;
  dat_26420 = sub_8d80(&a1[1],optind + -1);
  v12 = v15;
  v10 = v14;
  if (v14) { // branch-flip
    if (v15)
      sub_e230("--from-file and --to-file both specified"); // no-return
    v4 = 0;
    for (; optind < v16; optind = optind + 1) {
      v3 = sub_8e70(0,v10,a1[optind]);
      if (v4 < v3)
        v4 = v3;
    }
  }
  else {
    v10 = (long)optind;
    if (v15) { // branch-flip
      v4 = 0;
      while ((int)v10 < v16) {
        v3 = sub_8e70(0,a1[v10],v12);
        if (v4 < v3)
          v4 = v3;
        optind += 1;
        v10 = (long)optind;
      }
    }
    else {
      if (v16 - optind != 2) {
        if (v16 - optind > 1)
          sub_8c00("extra operand \'%s\'",a1[v10 + 2]); // no-return
        sub_8c00("missing operand after \'%s\'",a1[v17 + -1]); // no-return
      }
      v4 = sub_8e70(0,a1[v10],a1[v10 + 1]);
    }
  }
  sub_e0f0();
  sub_8e20();
  exit(v4); // no-return
label_564a:
  dat_26180 = 1;
  v4 = v3;
  goto label_50a0;
}


// Function: sub_6a70 @ 0x6a70
void sub_6a70(long a0)
{
  char v1;
  long v10; // rbx
  long v11; // rbp
  long v12;
  char *v13;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  long v7;
  long v8;
  long v9;
  
  v11 = 0;
label_6a7d:
  v2 = *(long *)(a0 + 0x118 + v11);
  v8 = 0;
  v6 = 0;
  v3 = *(long *)(a0 + 0xf8 + v11);
  v4 = *(long *)(a0 + 200 + v11);
  v5 = *(long *)((a0 + 0x248) - v11);
label_6aac:
  do {
    v12 = v6;
    if (v4 <= v6) goto label_6bea;
    while (v9 = v12, *(char *)(v2 + v12)) {
      while( true ) {
        v6 = v9;
        do {
          v6 += 1;
        } while (*(char *)(v2 + v6));
        for (; *(char *)(v5 + v8); v8 = v8 + 1) {
        }
        do {
          v10 = v6 - v9;
          while ((v9 && (*(long *)(v3 + -8 + v9 * 8) == *(long *)(v3 + -8 + v6 * 8)))) {
            v6 -= 1;
            *(char *)(v2 + v9 + -1) = 1;
            *(char *)(v2 + v6) = 0;
            v1 = *(char *)(v2 + -2 + v9);
            v9 -= 1;
            while (v1) {
              v9 -= 1;
              v1 = *(char *)(v2 + -1 + v9);
            }
            do {
              v8 -= 1;
            } while (*(char *)(v5 + v8));
          }
          v12 = v6;
          if (!*(char *)(v5 + -1 + v8))
            v12 = v4;
          while ((v4 != v6 && (*(long *)(v3 + v9 * 8) == *(long *)(v3 + v6 * 8)))) {
            v9 += 1;
            v7 = v6 + 1;
            *(char *)(v2 + -1 + v9) = 0;
            *(char *)(v2 + v6) = 1;
            if (!*(char *)(v2 + v7)) goto label_6b93;
            do {
              v7 += 1;
            } while (*(char *)(v2 + v7));
            v8 += 1;
            v1 = *(char *)(v5 + v8);
            while (v6 = v7, v1) {
              v12 = v7;
label_6b93:
              v8 += 1;
              v1 = *(char *)(v5 + v8);
            }
          }
        } while (v6 - v9 != v10);
        if (v6 <= v12) goto label_6aac;
        v13 = (char *)(v2 + -1 + v6);
        do {
          v13[v9 - v6] = 1;
          *v13 = 0;
          do {
            v8 -= 1;
          } while (*(char *)(v5 + v8));
          v13 = &v13[-1];
        } while ((char *)(v2 + -1 + v12) != v13);
        if (v12 < v4) break;
label_6bea:
        v9 = v12;
        if (v12 == v4) {
          v11 += 0x130;
          if (v11 == 0x260)
            return;
          goto label_6a7d;
        }
      }
    }
    do {
      v8 += 1;
    } while (*(char *)(v5 + -1 + v8));
    v6 = v12 + 1;
  } while( true );
}


// Function: sub_6c30 @ 0x6c30
void sub_6c30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_15f50(0x30);
  v1[3] = a0;
  v1[4] = a1;
  v1[1] = a3;
  v1[2] = a2;
  *v1 = a4;
}


// Function: sub_6c70 @ 0x6c70
unsigned long sub_6c70(long a0)
{
  char v1;
  long v10;
  long v11; // rsi
  long v12; // rdi
  long v13; // r9
  char v2;
  long v3;
  long v4;
  long v5;
  long v6;
  unsigned long v7; // rax
  long v8; // rcx
  long v9;
  
  v3 = *(long *)(a0 + 200);
  v4 = *(long *)(a0 + 0x118);
  v5 = *(long *)(a0 + 0x248);
  v6 = *(long *)(a0 + 0x1f8);
  if ((1 <= v3) || (1 <= v6)) {
    v11 = 0;
    v12 = 0;
    v7 = 0;
    do {
      v1 = *(char *)(v4 + v12);
      v2 = *(char *)(v5 + v11);
      v10 = v11;
      v9 = v12;
      if (v1 || v2) {
        if (v1) { // branch-flip
          do {
            v9 += 1;
          } while (*(char *)(v4 + v9));
          v13 = v9 - v12;
        }
        else {
          v13 = 0;
        }
        if (v2) { // branch-flip
          do {
            v10 += 1;
          } while (*(char *)(v5 + v10));
          v8 = v10 - v11;
        }
        else {
          v8 = 0;
        }
        v7 = sub_6c30(v12,v11,v13,v8);
      }
      v12 = v9 + 1;
      v11 = v10 + 1;
    } while ((v12 < v3) || (v11 < v6));
  }
  else {
    v7 = 0;
  }
  return v7;
}


// Function: sub_6d50 @ 0x6d50
unsigned long sub_6d50(long a0)
{
  char v1;
  unsigned long v10;
  char v2;
  long v3;
  long v4;
  unsigned long v5;
  long v6; // rcx
  unsigned long v7;
  unsigned long v8;
  long v9; // rbx
  
  v5 = 0;
  v8 = *(unsigned long *)(a0 + 200);
  v10 = *(unsigned long *)(a0 + 0x1f8);
  v3 = *(long *)(a0 + 0x118);
  v4 = *(long *)(a0 + 0x248);
  v7 = v8 & v10;
  do {
    if ((long)v7 <= -1)
      return v5;
    do {
      v1 = *(char *)(v3 + -1 + v8);
      v2 = *(char *)(v4 + -1 + v10);
      if (v1 || v2) {
        if (v1) { // branch-flip
          v7 = v8;
          do {
            v7 -= 1;
          } while (*(char *)(v3 + -1 + v7));
          v9 = v8 - v7;
          v8 = v7;
          if (v2) goto label_6dd9;
label_6e20:
          v6 = 0;
        }
        else {
          v9 = 0;
          if (!v2) goto label_6e20;
label_6dd9:
          v7 = v10;
          do {
            v7 -= 1;
          } while (*(char *)(v4 + -1 + v7));
          v6 = v10 - v7;
          v10 = v7;
        }
        v5 = sub_6c30(v8,v10,v9,v6);
      }
      v10 -= 1;
      v8 -= 1;
      v7 = v10;
    } while (0 <= (long)v8);
  } while( true );
}


// Function: sub_6e30 @ 0x6e30
void sub_6e30(long a0)
{
  char v1;
  long *v10;
  long v11;
  long v12; // rcx
  unsigned long v13;
  void *v14;
  long v15;
  void *v16 [3]; // stack - 0x58
  unsigned long *v17;
  void *v18;
  long v19; // rdi
  unsigned long v2;
  long *v20;
  void **v21;
  char *v22;
  long v23;
  long v24;
  long v25;
  long v26;
  unsigned long *v27;
  long v28; // stack - 0x60
  long v3;
  void *v4; // rax
  long v5;
  void *v6; // rax
  unsigned long v7; // rax
  char *v8;
  unsigned long v9;
  
  v3 = sub_15f50((*(long *)(a0 + 0x1f8) + *(long *)(a0 + 200)) * 0x10);
  *(long *)(a0 + 0x100) = v3;
  v26 = *(long *)(a0 + 200) * 8;
  v3 += v26;
  *(long *)(a0 + 0x108) = v3;
  v3 += v26;
  *(long *)(a0 + 0x230) = v3;
  *(long *)(a0 + 0x238) = v3 + *(long *)(a0 + 0x1f8) * 8;
  v4 = (void *)sub_f5e0(*(long *)(a0 + 0x128) << 4);
  v16[0] = v4;
  v14 = (void *)((long)v4 + *(long *)(a0 + 0x128) * 8);
  if (1 <= *(long *)(a0 + 200)) {
    v26 = *(long *)(a0 + 0xf8);
    v3 = 0;
    do {
      v5 = v3 * 8;
      v3 += 1;
      v10 = (long *)((long)v4 + *(long *)(v26 + v5) * 8);
      *v10 = *v10 + 1;
    } while (v3 < *(long *)(a0 + 200));
  }
  v26 = *(long *)(a0 + 0x1f8);
  if (1 <= v26) {
    v3 = *(long *)(a0 + 0x228);
    v5 = 0;
    do {
      v26 = v5 * 8;
      v5 += 1;
      v10 = (long *)((long)v14 + *(long *)(v3 + v26) * 8);
      *v10 = *v10 + 1;
      v26 = *(long *)(a0 + 0x1f8);
    } while (v5 < v26);
  }
  v27 = &v15;
  v16[1] = v14;
  v6 = (void *)sub_f5e0(v26 + *(long *)(a0 + 200));
  v9 = *(unsigned long *)(a0 + 200);
  v10 = (long *)(a0 + 0xf8);
  v21 = v16;
  v26 = *v10;
  v28 = (long)v6 + v9;
  v7 = v9 >> 8;
  v18 = v6;
  v17 = v27;
  if (!v7) goto label_737e;
label_6f76:
  v13 = 5;
  do {
    v13 *= 2;
    v7 >>= 2;
    v20 = v10;
  } while (v7);
  do {
    v7 = 0;
    do {
      while (v3 = *(long *)(v26 + v7 * 8), !v3) {
label_6f94:
        v7 += 1;
        if (v7 == v9) goto label_6fc8;
      }
      v2 = *(unsigned long *)((long)v14 + v3 * 8);
      if (!v2) {
        *(char *)((long)v18 + v7) = 1;
        goto label_6f94;
      }
      if (v2 <= v13) goto label_6f94;
      *(char *)((long)v18 + v7) = 2;
      v7 += 1;
    } while (v7 != v9);
label_6fc8:
    do {
      v10 = &v20[0x26];
      v17 = &v17[1];
      if ((long *)(a0 + 0x358) == v10) {
        v26 = 0;
        v14 = v6;
        v17 = v27;
        goto label_6ff4;
      }
      v9 = v20[0x20];
      v14 = *v21;
      v21 = &v21[-1];
      v18 = (void *)*v17;
      v26 = *v10;
      v7 = v9 >> 8;
      if (v7) goto label_6f76;
label_737e:
      v20 = v10;
    } while (!v9);
    v13 = 5;
  } while( true );
label_6ff4:
  v3 = *(long *)(a0 + 200 + v26);
  v5 = 0;
  if (0 < v3) {
    do {
      while( true ) {
        v8 = (char *)((long)v14 + v5);
        if (*v8 != '\x02') break;
        v5 += 1;
        *v8 = '\0';
        if (v3 <= v5) goto label_703b;
      }
      if (*v8) {
        v23 = 0;
        v11 = v5;
        while ((v11 = v11 + 1, v11 < v3 && (*(char *)((long)v14 + v11)))) {
          v23 += (unsigned long)(*(char *)((long)v14 + v11) == '\x02');
        }
        if (v11 <= v5) { // branch-flip
          v19 = v11 - v5;
          if (v23 * 4 <= v19) {
label_71cb:
            v11 = v19 >> 4;
            if (v11 <= 0) goto label_719b;
            v23 = 1;
            do {
              v23 *= 2;
              v11 >>= 2;
            } while (v11);
            v11 = v19 + -1 + v5;
label_71ee:
            v12 = 0;
            v25 = 0;
            do {
              while (v22 = (char *)(v12 + v5 + (long)v14), *v22 != '\x02') {
                v24 = 0;
label_71fb:
                v12 += 1;
                v25 = v24;
                if (v19 <= v12) goto label_7238;
              }
              v24 = v25 + 1;
              if (v25 == v23) {
                v12 -= v24;
                goto label_71fb;
              }
              if (v25 <= v23) goto label_71fb;
              v12 += 1;
              *v22 = '\0';
              v25 = v24;
            } while (v12 < v19);
label_7238:
            v5 = 0;
            v22 = v8;
            do {
              while( true ) {
                v1 = *v22;
                if ((8 <= (long)v22 - (long)v8) && (v1 == '\x01')) goto label_7290;
                if (v1 != '\x02') break;
                *v22 = '\0';
                v22 = &v22[1];
                v5 = 0;
                if (&v8[v19] == v22) goto label_7290;
              }
              if (v1) { // branch-flip
                v5 += 1;
                if (v5 == 3) break;
              }
              else {
                v5 = 0;
              }
              v22 = &v22[1];
            } while (&v8[v19] != v22);
label_7290:
            v5 = 0;
            v8 = (char *)((long)v14 + v11);
            do {
              if ((8 <= ((long)v14 + v11) - (long)v8) && (*v8 == '\x01')) break;
              if (*v8 != '\x02') { // branch-flip
                if (*v8) { // branch-flip
                  v5 += 1;
                  if (v5 == 3) break;
                }
                else {
                  v5 = 0;
                }
              }
              else {
                *v8 = '\0';
                v5 = 0;
              }
              v8 = &v8[-1];
            } while ((char *)((long)v14 + (v11 - v19)) != v8);
label_71b0:
            v5 = v11; // crossjump-dupe
          }
        }
        else {
          do {
            if (*(char *)((long)v14 + v11 + -1) != '\x02') {
              v19 = v11 - v5;
              if (v23 * 4 <= v19) goto label_71cb;
              if (v5 < v11) {
                v8 = (char *)((long)v14 + v11 + -1);
                do {
                  if (*v8 == '\x02')
                    *v8 = '\0';
                  v8 = &v8[-1];
                } while ((char *)((long)v14 + v5 + -1) != v8);
              }
              goto label_7018;
            }
            v11 -= 1;
            v23 -= 1;
            *(char *)((long)v14 + v11) = 0;
          } while (v11 != v5);
          v19 = 0;
          if (v23 <= 0) {
label_719b:
            v11 = v19 + -1 + v5;
            if (0 < v19) {
              v23 = 1;
              goto label_71ee;
            }
            v5 = v11;
          }
        }
      }
label_7018:
      v5 += 1;
    } while (v5 < v3);
  }
label_703b:
  v26 += 0x130;
  v17 = &v17[1];
  if (v26 == 0x260) {
    v10 = (long *)(a0 + 200);
    v14 = v6;
    do {
      v26 = *v10;
      v3 = 0;
      if (1 <= v26) {
        v5 = 0;
        do {
          while ((!dat_26408 && (*(char *)((long)v14 + v5)))) {
            *(char *)(v10[10] + v5) = 1;
            v5 += 1;
            if (v26 == v5) goto label_70d6;
          }
          v11 = v3 * 8;
          *(unsigned long *)(v10[7] + v3 * 8) = *(unsigned long *)(v10[6] + v5 * 8);
          v3 += 1;
          *(long *)(v10[8] + v11) = v5;
          v5 += 1;
        } while (v26 != v5);
      }
label_70d6:
      v10[9] = v3;
      v10 = &v10[0x26];
      v27 = &v27[1];
      if ((long *)(a0 + 0x328) == v10) {
        free(v6);
        free(v4); // tail-call
        return;
      }
      v14 = (void *)*v27;
    } while( true );
  }
  v14 = (void *)*v17;
  goto label_6ff4;
}


// Function: sub_73b0 @ 0x73b0
void sub_73b0(int a0,long a1)
{
  long v1;
  long v2;
  char *v3; // rax
  
  if (!a0)
    return;
  v2 = dat_26548;
  if (!dat_26548)
    v2 = *(long *)(a1 + 0x138);
  v1 = dat_26540;
  if (!dat_26540)
    v1 = *(long *)(a1 + 8);
  if (dat_264a1) // branch-flip
    v3 = dcgettext(NULL,"Files %s and %s differ\n",5);
  else {
    v3 = dcgettext(NULL,"Binary files %s and %s differ\n",5);
  }
  sub_e0e0(v3,v1,v2); // tail-call
}


// Function: sub_7460 @ 0x7460
char sub_7460(int *a0) // ternary
{
  int *v1;
  long v10; // rax
  unsigned long *v11;
  long *v12; // rax
  char *v13; // rax
  long v14;
  long v15; // rbx
  unsigned long v16; // stack - 0x78
  char v17 [8];
  char v18 [8];
  char v19 [8];
  unsigned long v2;
  char v20 [8];
  unsigned long *v21;
  int *v22;
  void *v23;
  long v24;
  char v25; // r12b
  void *v26; // stack - 0xa0
  unsigned long v27; // stack - 0x70
  long v28; // stack - 0x68
  long v29; // stack - 0x60
  void *v3;
  char v30; // stack - 0x58
  long v31; // stack - 0x50
  unsigned long v4;
  unsigned long *v5;
  char v6; // al
  int v7; // eax
  unsigned long v8; // rax
  unsigned long v9;
  
  v6 = sub_c8f0(a0,dat_26553);
  if (v6) {
    if ((((1 <= *(long *)&a0[0x10] && *(long *)&a0[0x10] != *(long *)&a0[0x5c]) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000)))) {
label_75bb:
      v25 = 1;
    }
    else {
      if (*a0 != a0[0x4c]) {
        v2 = *(unsigned long *)&a0[0x5e];
        v9 = *(unsigned long *)&a0[0x12];
        v8 = sub_165d0(8,sub_165d0(v9,v2,0x7ffffffffffffffe),0x7ffffffffffffffe);
        v9 = sub_15fa0(*(unsigned long *)&a0[0x28],v8);
        v2 = *(unsigned long *)&a0[0x74];
        *(unsigned long *)&a0[0x28] = v9;
        *(unsigned long *)&a0[0x74] = sub_15fa0(v2,v8);
        while( true ) {
          if (0 <= *a0)
            sub_bf50(a0,v8 - *(long *)&a0[0x2c]);
          if (0 <= a0[0x4c])
            sub_bf50(&a0[0x4c],v8 - *(long *)&a0[0x78]);
          v4 = *(unsigned long *)&a0[0x2c];
          if ((v4 != *(unsigned long *)&a0[0x78]) || (v23 = *(void **)&a0[0x74], v3 = *(void **)&a0[0x28], memcmp(v3,v23,v4))) goto label_75bb;
          if (v4 != v8) break;
          a0[0x78] = 0;
          a0[0x79] = 0;
          a0[0x2c] = 0;
          a0[0x2d] = 0;
        }
      }
      v25 = 0;
    }
    sub_73b0(v25,a0);
    goto label_75d6;
  }
  v26 = (void *)sub_f5e0(*(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4);
  *(long *)&a0[0x46] = (long)v26 + 1;
  *(long *)&a0[0x92] = (long)v26 + *(long *)&a0[0x32] + 3;
  sub_6e30(a0);
  v16 = *(unsigned long *)&a0[0x40];
  v15 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
  v27 = *(unsigned long *)&a0[0x8c];
  v10 = sub_15f50(v15 * 0x10);
  v24 = *(long *)&a0[0x90];
  v14 = v24 * 8 + 8;
  v28 = v10 + v14;
  v29 = v10 + v14 + v15 * 8;
  v30 = dat_26418;
  if (v15) { // branch-flip
    v31 = 1;
    do {
      v31 *= 2;
      v15 >>= 2;
    } while (v15);
    if (v31 <= 0xfff)
      v31 = 0x1000;
  }
  else {
    v31 = 0x1000;
  }
  v14 = 0x26;
  v22 = a0;
  v11 = (unsigned long *)0x261a0;
  while (v14) {
    v21 = &v11[1];
    v1 = &v22[2];
    *v11 = *(unsigned long *)v22;
    v14 -= 1;
    v22 = v1;
    v11 = v21;
  }
  v14 = 0x26;
  v22 = &a0[0x4c];
  while (v14) {
    v21 = &v11[1];
    v1 = &v22[2];
    *v11 = *(unsigned long *)v22;
    v14 -= 1;
    v22 = v1;
    v11 = v21;
  }
  sub_6890(0,*(unsigned long *)&a0[0x44],0,v24,dat_26408,&v16);
  free((void *)(*(long *)&a0[0x90] * -8 + -8 + v28));
  sub_6a70(a0);
  v11 = (dat_26580 != 4) ? (unsigned long *)sub_6d50(a0) : (unsigned long *)sub_6c70(a0); // branch-flip
  if ((dat_26554) || (dat_264e0)) {
    if (v11) {
      v21 = v11;
      do {
        v12 = (long *)sub_eaf0(v21);
        v5 = (unsigned long *)*v12;
        *v12 = 0;
        v7 = sub_f1a0(v21,v19,v17,v20,v18);
        *v12 = (long)v5;
        if (!v5) break;
        v21 = v5;
      } while (!v7);
      v25 = v7 != 0;
      goto label_780b;
    }
    if (!dat_264a1) goto label_78dc;
    v25 = 0;
label_79cb:
    sub_73b0(v25,a0);
  }
  else {
    v25 = v11 != NULL;
label_780b:
    if (dat_264a1) goto label_79cb;
    if ((bool)v25) { // branch-flip
label_7821:
      v14 = dat_26548;
      if (!dat_26548)
        v14 = *(long *)&a0[0x4e];
      v24 = dat_26540;
      if (!dat_26540)
        v24 = *(long *)&a0[2];
      sub_e280(v24,v14,*(long *)&a0[0x98] != 0);
      switch(dat_26580) {
        default:
          abort(); // no-return
        case 1:
          sub_ccf0(v11);
          break;
        case 2:
          sub_8870(v11,0);
          break;
        case 3:
          sub_8870(v11,1);
          break;
        case 4:
          sub_aa10(v11);
          break;
        case 5:
          sub_aa30(v11);
          break;
        case 6:
          sub_aa50(v11);
          break;
        case 7:
          sub_b430(v11);
          break;
        case 8:
          sub_d5b0(v11);
        
      }
      sub_e550();
    }
    else {
label_78dc:
      v25 = 0;
      if (!dat_26578) goto label_7821;
    }
  }
  free(*(void **)&a0[0x40]);
  free(v26);
  free(*(void **)&a0[0x3e]);
  free((void *)(*(long *)&a0[0x2e] + *(long *)&a0[0x30] * 8));
  free(*(void **)&a0[0x8a]);
  free((void *)(*(long *)&a0[0x7a] + *(long *)&a0[0x7c] * 8));
  while (v11) {
    v21 = (unsigned long *)*v11;
    free(v11);
    v11 = v21;
  }
  if ((unsigned int)(dat_26580 - 4U) <= 1) {
    v12 = (long *)0x26540;
    v22 = &a0[0x48];
    do {
      if ((char)*v22) {
        v13 = dcgettext(NULL,"No newline at end of file",5);
        v14 = *v12;
        if (!v14)
          v14 = *(long *)&v22[-0x46];
        error(0,0,"%s: %s\n",v14,v13);
        v25 = 2;
      }
      v12 = &v12[1];
      v22 = &v22[0x4c];
    } while (v12 != (long *)0x26550);
  }
label_75d6:
  v23 = *(void **)&a0[0x74];
  if (*(void **)&a0[0x28] != v23) {
    free(*(void **)&a0[0x28]);
    v23 = *(void **)&a0[0x74];
  }
  free(v23);
  return v25;
}


// Function: sub_7ab0 @ 0x7ab0
void sub_7ab0(unsigned long a0,long a1,unsigned long a2,long a3)
{
  void *v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  char v4 [56];
  
  sub_eb70(0);
  if (a3) // branch-flip
    __fprintf_chk(dat_26188,1,"%s %s",a0,a3);
  else {
    v1 = localtime((void *)(a1 + 0x68));
    v2 = sub_154d0(a1 + 0x10);
    if ((!v1) || (v3 = sub_13d10(v4,0x2b,dat_26400,v1,0,v2 & 0xffffffff), !v3))
      __sprintf_chk(v4,1,0x2b,"%ld.%.9d",*(unsigned long *)(a1 + 0x68),v2 & 0xffffffff);
    __fprintf_chk(dat_26188,1,"%s %s\t%s",a0,a2,v4);
  }
  sub_eb70(3);
  putc_unlocked(10,dat_26188); // tail-call
}


// Function: sub_7be0 @ 0x7be0
void sub_7be0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  int v2; // eax
  char v3 [8];
  char v4 [8];
  char v5 [8];
  char v6 [8];
  
  if (!a0)
    return;
  do {
    v1 = (unsigned long *)*a0;
    *a0 = 0;
    v2 = sub_f1a0(a0,v6,v5,v4,v3);
    *a0 = v1;
    *(bool *)&a0[5] = v2 == 0;
    a0 = v1;
  } while (v1);
}


// Function: sub_7c80 @ 0x7c80
unsigned long sub_7c80(long a0,long a1)
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


// Function: sub_7d30 @ 0x7d30
void sub_7d30(void *a0,char *a1)
{
  char v1;
  char v2; // al
  int v3;
  char *v4;
  char *v5;
  int v6; // r14d
  
  v6 = 0;
  putc_unlocked(0x20,a0);
  v4 = a1;
  while( true ) {
    v1 = *v4;
    v5 = &v4[1];
    v2 = sub_f850(v1);
    if ((v1 == '\n') || (!v2)) break;
    v6 += 1;
    v4 = v5;
  }
  v5 = &a1[(long)v6 + 1];
  v3 = v6;
  while ((v1 != '\n' && (v3 = v3 + 1, v3 <= v6 + 0x27))) {
    v1 = *v5;
    v5 = &v5[1];
  }
  if (v6 < v3) {
    a1 = &a1[(long)v3 + -1];
    do {
      v1 = *a1;
      if (!sub_f850(v1)) break;
      v3 -= 1;
      a1 = &a1[-1];
    } while (v6 != v3);
  }
  fwrite_unlocked(v4,1,(long)(v3 - v6),a0); // tail-call
}


// Function: sub_7e00 @ 0x7e00
void sub_7e00(void) // return-dupe
{
  long v1; // stack - 0x20
  long v2; // stack - 0x18
  
  sub_f0c0();
  if (v2 <= v1) {
    __fprintf_chk(dat_26188,1,"%ld",v2);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld");
}


// Function: sub_7e90 @ 0x7e90
void sub_7e90(unsigned long *a0)
{
  bool v1;
  unsigned long *v10;
  unsigned int v2; // eax
  long v3;
  long v4; // stack - 0x50
  long v5; // stack - 0x58
  long v6; // stack - 0x60
  long v7; // stack - 0x48
  void *v8;
  char *v9;
  
  v2 = sub_f1a0(a0,&v6,&v5,&v4,&v7);
  if (v2) {
    v3 = -dat_26288;
    v6 -= dat_26570;
    if (v6 < v3)
      v6 = v3;
    v4 -= dat_26570;
    if (v4 < v3)
      v4 = v3;
    v3 = dat_26570 + v5;
    v1 = v5 < dat_26270 - dat_26570;
    v5 = dat_26270 + -1;
    if (v1)
      v5 = v3;
    v3 = dat_26570 + v7;
    v1 = v7 < dat_263a0 - dat_26570;
    v7 = dat_263a0 + -1;
    if (v1)
      v7 = v3;
    if (dat_26520) { // branch-flip
      v3 = sub_7c80(dat_26258);
      sub_e2b0();
      v8 = dat_26188;
      fputs_unlocked("***************",dat_26188);
      if (v3)
        sub_7d30(v8,v3);
    }
    else {
      sub_e2b0();
      v8 = dat_26188;
      fputs_unlocked("***************",dat_26188);
    }
    putc_unlocked(10,v8);
    sub_eb70(4);
    fputs_unlocked("*** ",v8);
    sub_7e00(0x261a0,v6,v5);
    fputs_unlocked(" ****",v8);
    sub_eb70(3);
    putc_unlocked(10,v8);
    if ((v2 & 1) && (v6 <= v5)) {
      v10 = a0;
      v3 = v6;
      do {
        sub_eb70(2);
        for (; v10; v10 = (unsigned long *)*v10) {
          if (v3 < (long)(v10[2] + v10[3])) {
            if ((long)v10[3] <= v3) {
              v9 = "-";
              if (1 <= (long)v10[1])
                v9 = "!";
              goto label_8084;
            }
            break;
          }
        }
        v9 = " ";
label_8084:
        sub_ef20(v9,dat_26258 + v3 * 8,1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26258 + 8 + v3 * 8) + -1) == '\n')
          putc_unlocked(10,v8);
        v3 += 1;
      } while (v3 <= v5);
    }
    sub_eb70(4);
    fputs_unlocked("--- ",v8);
    sub_7e00(0x262d0,v4,v7);
    fputs_unlocked(" ----",v8);
    sub_eb70(3);
    putc_unlocked(10,v8);
    if ((v2 & 2) && (v4 <= v7)) {
      v3 = v4;
      do {
        sub_eb70(1);
        for (; a0; a0 = (unsigned long *)*a0) {
          if (v3 < (long)(a0[1] + a0[4])) {
            if ((long)a0[4] <= v3) {
              v9 = "+";
              if (1 <= (long)a0[2])
                v9 = "!";
              goto label_819d;
            }
            break;
          }
        }
        v9 = " ";
label_819d:
        sub_ef20(v9,dat_26388 + v3 * 8,1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26388 + 8 + v3 * 8) + -1) == '\n')
          putc_unlocked(10,v8);
        v3 += 1;
      } while (v3 <= v7);
    }
  }
}


// Function: sub_8280 @ 0x8280
void sub_8280(void) // return-dupe
{
  char *v1; // rdx
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  sub_f0c0();
  if (v3 <= v2) {
    v1 = "%ld,0";
    if (v2 <= v3)
      v1 = "%ld";
    __fprintf_chk(dat_26188,1,v1);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld",v2,(v3 - v2) + 1);
}


// Function: sub_8320 @ 0x8320
void sub_8320(unsigned long *a0) // return-dupe
{
  long v1;
  long v10;
  unsigned long *v11;
  long v12;
  long v13; // stack - 0x78
  bool v2;
  long v3;
  long v4;
  void *v5;
  long v6; // stack - 0x50
  long v7; // stack - 0x58
  long v8; // stack - 0x60
  long v9; // stack - 0x48
  
  if (!sub_f1a0(a0,&v8,&v7,&v6,&v9))
    return;
  v8 -= dat_26570;
  v4 = -dat_26288;
  if (v8 < v4)
    v8 = v4;
  v6 -= dat_26570;
  if (v6 < v4)
    v6 = v4;
  v4 = dat_26570 + v7;
  v2 = v7 < dat_26270 - dat_26570;
  v7 = dat_26270 + -1;
  if (v2)
    v7 = v4;
  v4 = dat_26570 + v9;
  v2 = v9 < dat_263a0 - dat_26570;
  v9 = dat_263a0 + -1;
  if (v2)
    v9 = v4;
  if (dat_26520) { // branch-flip
    v4 = sub_7c80(dat_26258);
    sub_e2b0();
    v5 = dat_26188;
    sub_eb70(4);
    fputs_unlocked("@@ -",v5);
    sub_8280(0x261a0,v8,v7);
    fputs_unlocked(" +",v5);
    sub_8280(0x262d0,v6,v9);
    fputs_unlocked(" @@",v5);
    sub_eb70(3);
    if (v4)
      sub_7d30(v5,v4);
  }
  else {
    sub_e2b0(dat_263a0 - dat_26570,v8);
    v5 = dat_26188;
    sub_eb70(4);
    fputs_unlocked("@@ -",v5);
    sub_8280(0x261a0,v8,v7);
    fputs_unlocked(" +",v5);
    sub_8280(0x262d0,v6,v9);
    fputs_unlocked(" @@",v5);
    sub_eb70(3);
  }
  putc_unlocked(10,v5);
  v13 = v6;
  v4 = v8;
  do {
    if ((v7 < v4) && (v9 < v13))
      return;
    while ((!a0 || (v4 < (long)a0[3]))) {
      v1 = v4 + 1;
      v11 = (unsigned long *)(dat_26258 + v4 * 8);
      if ((!dat_26491) || (*(char *)*v11 != '\n'))
        putc_unlocked((-(unsigned int)(dat_26492 == '\0') & 0x17) + 9,v5);
      sub_f0a0(0,v11);
      v13 += 1;
      v4 = v1;
      if ((v7 < v1) && (v9 < v13))
        return;
    }
    v1 = a0[2];
    if (v1) {
      v12 = v4 * 8;
      v10 = v1;
      do {
        while( true ) {
          v3 = dat_26258;
          sub_eb70(2);
          putc_unlocked(0x2d,v5);
          v11 = (unsigned long *)(v3 + v12);
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n'))))
            putc_unlocked(9,v5);
          sub_ef20(0,v11,1);
          sub_eb70(3);
          if (*(char *)(v11[1] + -1) != '\n') break;
          v12 += 8;
          putc_unlocked(10,v5);
          v10 -= 1;
          if (!v10) goto label_8686;
        }
        v12 += 8;
        v10 -= 1;
      } while (v10);
label_8686:
      v4 += v1;
    }
    v1 = a0[1];
    if (v1) {
      v12 = v13 * 8;
      v10 = v1;
      do {
        while( true ) {
          v3 = dat_26388;
          sub_eb70(1);
          putc_unlocked(0x2b,v5);
          v11 = (unsigned long *)(v3 + v12);
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n'))))
            putc_unlocked(9,v5);
          sub_ef20(0,v11,1);
          sub_eb70(3);
          if (*(char *)(v11[1] + -1) != '\n') break;
          v12 += 8;
          putc_unlocked(10,v5);
          v10 -= 1;
          if (!v10) goto label_874e;
        }
        v12 += 8;
        v10 -= 1;
      } while (v10);
label_874e:
      v13 += v1;
    }
    a0 = (unsigned long *)*a0;
  } while( true );
}


// Function: sub_8770 @ 0x8770
long * sub_8770(long *a0)
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


// Function: sub_87e0 @ 0x87e0
void sub_87e0(long a0,unsigned long *a1,char a2)
{
  if (a2) {
    sub_7ab0("---",a0,*a1,dat_26540);
    sub_7ab0("+++",a0 + 0x130,a1[1],dat_26548); // tail-call
    return;
  }
  sub_7ab0("***",a0,*a1,dat_26540);
  sub_7ab0("---",a0 + 0x130,a1[1],dat_26548); // tail-call
}


// Function: sub_8870 @ 0x8870
void sub_8870(long *a0,char a1)
{
  unsigned long *v1;
  long *v2;
  void *v3; // rdx
  
  if ((dat_26554) || (v2 = a0, dat_264e0))
    sub_7be0(a0);
  else {
    while (v2) {
      *(char *)&v2[5] = 0;
      v1 = (unsigned long *)*v2;
      if (!v1) break;
      *(char *)&v1[5] = 0;
      v2 = (long *)*v1;
    }
  }
  v3 = sub_8320;
  dat_26178 = -dat_26288;
  dat_26170 = 0x7fffffffffffffff;
  if (!a1)
    v3 = sub_7e90;
  sub_eb10(a0,sub_8770,v3); // tail-call
}


// Function: sub_8910 @ 0x8910
int sub_8910(void)
{
  return (-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010;
}


// Function: sub_8930 @ 0x8930
void sub_8930(long *a0,char *a1)
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
    v3 = sub_15fa0(v2);
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


// Function: sub_8a10 @ 0x8a10
void sub_8a10(void)
{
  void *v1;
  unsigned long v2;
  char v3;
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // rdx
  char *v7;
  long v8; // r12
  
  v2 = dat_36860;
  v8 = 0x24f20;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILES\n",5),v2);
  __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare FILES line by line.",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v3 = ' ';
  v7 = "    --normal                  output a normal diff (the default)";
  do {
    if (v3) { // branch-flip
      v7 = dcgettext(NULL,v7,5);
      while (v5 = strchr(v7,10), v5) {
        v4 = (long)&v5[1] - (long)v7;
        if (0xfff < (int)v4)
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
        __printf_chk(1,"  %.*s",v4 & 0xffffffff,v7);
        v7 = &v5[1];
      }
      __printf_chk(1,&"  %s\n"[(int)(((unsigned int)(*v7 != '-') & (unsigned int)CONCAT71((undefined7)((unsigned long)v6 >> 8),*v7 != ' ')) * 2)],v7);
      v7 = *(char **)(v8 + 8);
    }
    else {
      putchar_unlocked(10);
      v7 = *(char **)(v8 + 8);
    }
    if (!v7) {
      sub_15e90(); // tail-call
      return;
    }
    v8 += 8;
    v3 = *v7;
  } while( true );
}


// Function: sub_8ba0 @ 0x8ba0
long sub_8ba0(long *a0,unsigned long a1,unsigned long a2)
{
  long v1;
  long v2;
  long v3; // rax
  
  if (!*a0)
    return v3;
  v1 = a0[4];
  v3 = sub_15f50(0x100);
  v2 = a0[3];
  *(long *)(v1 + 0x20) = v3;
  if ((char)v2) {
    v3 = re_compile_pattern(*a0,a0[1],a0[4]);
    if (v3) {
      error(2,0,"%s: %s",*a0,v3);
      return v3;
    }
  }
  v3 = a2;
  return v3;
}


// Function: sub_8c00 @ 0x8c00
void sub_8c00(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_36860;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_8c60 @ 0x8c60
void sub_8c60(unsigned long *a0,char *a1,unsigned long a2)
{
  char *v1;
  
  v1 = (char *)*a0;
  if (v1) {
    if (strcmp(v1,a1)) {
      error(0,0,dcgettext(NULL,"conflicting %s option value \'%s\'",5),a2,a1);
      sub_8c00(0,0); // no-return
    }
  }
  *a0 = a1;
}


// Function: sub_8cc0 @ 0x8cc0
void sub_8cc0(char *a0)
{
  if (!a0) {
    dat_2657c = 1;
    return;
  }
  if (!strcmp(a0,"auto")) {
    dat_2657c = 1;
    return;
  }
  if (!strcmp(a0,"always")) {
    dat_2657c = 2;
    return;
  }
  if (!strcmp(a0,"never")) {
    dat_2657c = 0;
    return;
  }
  sub_8c00("invalid color \'%s\'",a0); // no-return
}


// Function: sub_8d50 @ 0x8d50
void sub_8d50(int a0) // return-dupe
{
  if (dat_26580 == a0)
    return;
  if (!dat_26580) {
    dat_26580 = a0;
    return;
  }
  sub_8c00("conflicting output style options",0); // no-return
}


// Function: sub_8d80 @ 0x8d80
char * sub_8d80(unsigned long *a0,int a1)
{
  unsigned long *v1;
  unsigned long v2;
  char *v3;
  char *v4;
  unsigned long *v5;
  long v6;
  
  if (1 <= a1) { // branch-flip
    v6 = 1;
    v1 = &a0[(unsigned long)(unsigned int)(a1 - 1) + 1];
    v5 = a0;
    do {
      v2 = *v5;
      v5 = &v5[1];
      v6 = v6 + 1 + sub_144e0(v2);
    } while (v5 != v1);
    v3 = (char *)sub_15f50(v6);
    v4 = v3;
    do {
      *v4 = 0x20;
      v2 = *a0;
      a0 = &a0[1];
      v4 = (char *)sub_14530(&v4[1],v2);
    } while (a0 != v1);
  }
  else {
    v4 = (char *)sub_15f50(1);
    v3 = v4;
  }
  *v4 = 0;
  return v3;
}


// Function: sub_8e10 @ 0x8e10
void sub_8e10(long a0)
{
  sub_10a50(a0 + 0x58); // tail-call
}


// Function: sub_8e20 @ 0x8e20
void sub_8e20(void)
{
  int v1; // eax
  
  v1 = ferror_unlocked(stdout);
  if (v1)
    sub_e230("write failed"); // no-return
  v1 = fclose(stdout);
  if (!v1)
    return;
  sub_e1f0(dcgettext(NULL,"standard output",5)); // no-return
}


// Function: sub_8e70 @ 0x8e70
unsigned int sub_8e70(long a0,char *a1,char *a2)
{
  long *v1;
  int v10 [2]; // stack - 0x2a8
  long v11 [3]; // stack - 0x298
  long v12 [3]; // stack - 0x168
  char *v13; // stack - 0x2a0
  int *v14;
  char *v15;
  long *v16;
  unsigned int v17; // r9d
  char *v18;
  int v19; // ebx
  unsigned long v2;
  bool v20;
  bool v21;
  char *v22; // stack - 0x2c8
  unsigned int v23; // stack - 0x280
  int v24; // stack - 0x27c
  unsigned int v25; // stack - 0x278
  long v26; // stack - 0x270
  long v27; // stack - 0x268
  unsigned long v28; // stack - 0x260
  unsigned long v29; // stack - 0x258
  unsigned long v3;
  unsigned long v30; // stack - 0x250
  unsigned long v31; // stack - 0x248
  long v32; // stack - 0x240
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
label_9578:
        v10[0] = -1;
        v39 = -2;
        if (!a2) goto label_9560;
      }
      v39 = -2;
      a1 = a2;
      goto label_8f50;
    }
    if (!dat_26586) {
      v18 = a1;
      if (!a1)
        v18 = a2;
      v6 = (unsigned long)(unsigned long)(a1 == NULL);
      v6 = (long)((long)v6 * 0x130);
      sub_e0e0("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),v18);
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
      if (a2) goto label_9578;
      v39 = -1;
label_9560:
      v10[0] = -1;
      a1 = NULL;
      goto label_8f50;
    }
    v10[0] = -2;
    if (!a2) {
      v39 = -1;
      a2 = a1;
      goto label_8f50;
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
label_8f50:
  v56 = a0;
  if (a0) { // branch-flip
    v6 = (char *)sub_10920(*(unsigned long *)(a0 + 8),a1,0);
    v18 = (char *)v6;
    v13 = (char *)v6;
    v6 = (char *)sub_10920(*(unsigned long *)(a0 + 0x138),a2,0);
    v22 = (char *)v6;
  }
  else {
    v13 = a1;
    v18 = NULL;
    v22 = NULL;
    v6 = (int *)a2;
  }
  v40 = (char *)v6;
  v20 = 0;
  v16 = v11;
  do {
    if ((int)v16[-2] != -1) {
      if ((!v20) || (v5 = strcmp(v40,v13), v5)) {
        v9 = (char *)v16[-1];
        v5 = strcmp(v9,"-");
        if (v5) { // branch-flip
          if (dat_26550) // branch-flip
            v5 = lstat(v9,v16);
          else {
            v5 = stat(v9,v16);
          }
          if (v5) {
label_926f:
            v6 = __errno_location();
            v5 = 0xfffffffd - *v6;
            *(unsigned int *)&v16[-2] = v5;
          }
        }
        else {
          *(unsigned int *)&v16[-2] = 0;
          isatty(0);
          v5 = fstat(0,v16);
          if (v5) goto label_926f;
          v5 = *(unsigned int *)&v16[3] & 0xf000;
          if (v5 == 0x8000) {
            v6 = (long)lseek(0,0,1);
            if (0 <= (long)v6) { // branch-flip
              v16[6] = v16[6] - (long)v6;
              v6 = (int *)v16[6];
              if (v16[6] <= -1)
                v6 = (int *)0;
              v16[6] = (long)v6;
            }
            else {
              v6 = __errno_location();
              v5 = 0xfffffffd - *v6;
              *(unsigned int *)&v16[-2] = v5;
            }
          }
          sub_8e10(v16);
        }
      }
      else {
        v41 = CONCAT44(v24,v23);
        v42 = (unsigned long)v25;
        v12[0] = v11[0];
        v12[1] = v11[1];
        v39 = v10[0];
        v12[2] = v11[2];
        v43 = v26;
        v44 = v27;
        v45 = v28;
        v46 = v29;
        v47 = v30;
        v48 = v31;
        v49 = v32;
        v50 = v33;
        v51 = v34;
        v52 = v35;
        v53 = v36;
        v54 = v37;
        v55 = v38;
      }
    }
    v16 = &v16[0x26];
    if (v20) goto label_906c;
    v20 = 1;
  } while( true );
label_906c:
  v19 = 0;
  v6 = v10;
  v14 = v10;
label_9084:
  v19 += 1;
  if (!dat_26586) {
    if (v19 != 1) goto label_909b;
    if (dat_26585) goto label_91c0;
    goto label_920a;
  }
label_91c0:
  v8 = *v6;
  if (v8 != -2) { // branch-flip
    if (((v8 == -5 || v8 == -0xc) && (!a0)) && (!(v14[0x4c] + 2U & 0xfffffffd))) {
label_91fa:
      *v6 = -1;
      if (v19 != 2) goto label_920a;
label_90a5:
      if (v10[0] != -1) {
        v8 = 0xfffffffd - v10[0];
        if (v39 != -1) { // branch-flip
          if (v8 < 0) goto label_92b4;
        }
        else {
          v7 = 0x12;
          v16 = v12;
          while (v7) {
            v1 = &v16[1];
            *v16 = 0;
            v7 -= 1;
            v16 = v1;
          }
          v41 = CONCAT44(v41._4_4_,v23);
          if (v8 <= -1) goto label_92c2;
        }
        v6 = __errno_location();
        *v6 = v8;
        sub_df30(v13);
        v8 = 0xfffffffd - v39;
        if (0 <= v8) {
label_90fd:
          *v6 = v8;
          sub_df30(v40);
        }
        goto label_910c;
      }
      v7 = 0x12;
      v16 = v11;
      while (v7) {
        v1 = &v16[1];
        *v16 = 0;
        v7 -= 1;
        v16 = v1;
      }
      v23 = (unsigned int)v41;
      if (v39 != -1) { // branch-flip
label_92b4:
        v5 = 0xfffffffd - v39;
        v8 = v5;
        if (0 <= (int)v5) {
          v6 = __errno_location();
          goto label_90fd;
        }
      }
      else {
        v7 = 0x12;
        v16 = v12;
        while (v7) {
          v1 = &v16[1];
          *v16 = 0;
          v7 -= 1;
          v16 = v1;
        }
      }
label_92c2:
      v8 = v10[0];
      if (!a0) {
        v5 = v23 & 0xf000;
        v20 = v5 == 0x4000;
        if (v20 != (((unsigned int)v41 & 0xf000) == 0x4000)) {
          v6 = (unsigned long)(unsigned long)v20;
          v2 = (unsigned long)v6;
          v5 = (unsigned int)!v20;
          v7 = (long)(int)v5;
          v9 = (&v13)[v2 * 0x26];
          v6 = (unsigned long)sub_f670(v9);
          v6 = (char *)sub_a4f0((&v13)[v7 * 0x26],v6);
          v18 = (char *)v6;
          (&v13)[v7 * 0x26] = (char *)v6;
          v5 = strcmp(v9,"-");
          if (!v5)
            sub_e230("cannot compare \'-\' to a directory"); // no-return
          if (dat_26550) // branch-flip
            v5 = lstat(v18,&v11[v7 * 0x26]);
          else {
            v5 = stat(v18,&v11[v7 * 0x26]);
          }
          v4 = v5 != 0;
          v8 = v10[0];
          if ((bool)v4) {
            sub_df30(v18);
            goto label_910c;
          }
        }
      }
      if (v8 != -1) { // branch-flip
        if (v39 != -1) { // branch-flip
          if ((v11[1] == v12[1]) && (v11[0] == v12[0])) {
label_972b:
            if (v23 == (unsigned int)v41) {
              if (((((v11[2] != v12[2]) || (v24 != v41._4_4_)) || (v25 != (unsigned int)v42)) || ((v27 != v44 || (v32 != v49)))) || (v34 != v51)) {
                v20 = 0;
                v5 = (unsigned int)v41 & 0xf000;
              }
              else {
                if (dat_26578) goto label_9610;
                v20 = 1;
                v5 = (unsigned int)v41 & 0xf000;
              }
              goto label_95aa;
            }
            goto label_95a0;
          }
          v5 = v23 & 0xf000;
          v17 = (unsigned int)v41 & 0xf000;
          if (v5 != 0x6000) { // branch-flip
            v20 = 0;
            if (v5 != 0x2000) goto label_95aa;
            if (v17 == 0x2000) goto label_97ed;
          }
          else if (v17 == 0x6000) {
label_97ed:
            if (v43 == v26) goto label_972b;
          }
          if ((v5 == 0x4000) || (v20 = 0, v17 == 0x4000)) {
label_9820:
            if (v39 == -1) goto label_9829;
label_9780:
            v19 = 1;
            v6 = (unsigned long)sub_10830(v12);
            v3 = v6;
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v6 = (unsigned long)sub_10830(v11);
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v13;
            sub_df60("File %s is a %s while file %s is a %s\n",v15,v6,v9,v3);
            goto label_910c;
          }
label_9437:
          v17 = (unsigned int)v41 & 0xf000;
          if ((a0) && (((v5 & 0xffffdfff) != 0x8000 || (((unsigned int)v41 & 0xd000) != 0x8000)))) {
            if ((v8 == -1) || (v39 == -1)) goto label_9473;
            goto label_9780;
          }
          if ((v5 == 0xa000) || (v17 == 0xa000)) {
            if (!dat_26550)
              __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
            if ((v5 != 0xa000) || (v17 != 0xa000)) goto label_9780;
            v6 = (char *)sub_16360(v13);
            v9 = (char *)v6;
            if (v6) { // branch-flip
              v6 = (char *)sub_16360(v40);
              v15 = (char *)v6;
              v8 = 1;
              if (v6) {
                v5 = strcmp(v9,(char *)v6);
                if (!v5) {
                  free(v9);
                  free(v15);
                  goto label_9610;
                }
                v19 = 1;
                sub_e0e0("Symbolic links %s and %s differ\n",v13,v40);
                free(v9);
                free(v15);
                goto label_910c;
              }
            }
            else {
              v8 = 0;
            }
            v6 = (long)(long)v8;
            sub_df30((&v13)[(long)v6 * 0x26]);
            free(v9);
            free(NULL);
            goto label_910c;
          }
          v4 = v5 == 0x8000 & dat_26553;
          if ((((v4) && (v17 == 0x8000)) && (v27 != v44 && 1 <= v27)) && (1 <= v44)) {
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v13;
            v19 = 1;
            sub_e0e0("Files %s and %s differ\n",v15,v9);
            goto label_910c;
          }
          if (v8 == -2) {
            v5 = open(v13,0,0);
            v10[0] = v5;
            if (0 <= (int)v5) goto label_98e7;
            sub_df30(v13);
            if (v39 == -2) {
              v21 = 1;
              goto label_98f7;
            }
label_9912:
            if (0 <= v10[0]) {
              v8 = 2;
              goto label_9a55;
            }
            if (v39 < 0) goto label_910c;
label_992d:
            v5 = close(v39);
            if (v5) {
label_993a:
              sub_df30(v40);
            }
            goto label_910c;
          }
label_98e7:
          v21 = 0;
          if (v39 == -2) {
label_98f7:
            if (v20)
              v39 = v10[0];
            else {
              v5 = open(v40,0,0);
              v39 = v5;
              if ((int)v5 <= -1) {
                sub_df30(v40);
                goto label_9912;
              }
            }
            if (v21) goto label_9912;
          }
          v5 = sub_7460(v10);
          v8 = v5;
          if (0 <= v10[0]) {
label_9a55:
            v5 = close(v10[0]);
            if (v5) {
              sub_df30(v13);
              if ((v39 <= -1) || (v39 == v10[0])) goto label_910c;
              goto label_992d;
            }
          }
          v5 = v8;
          if ((0 <= v39) && (v39 != v10[0])) {
            v5 = close(v39);
            v5 = v8;
            if (v5) goto label_993a;
          }
label_9608:
          v19 = v5;
          if (!v19) goto label_9610;
        }
        else {
label_95a0:
          v20 = 0;
          v5 = v23 & 0xf000;
label_95aa:
          v21 = ((unsigned int)v41 & 0xf000) != 0x4000;
          if (v5 == 0x4000) {
            if (v21) goto label_9811;
            if (dat_26580 == 7)
              sub_e230("-D option not supported with directories"); // no-return
            if ((a0) && (!dat_26608)) {
              sub_e0e0("Common subdirectories: %s and %s\n",v13,v40);
              goto label_9610;
            }
label_95f7:
            v5 = sub_a0c0(v10,sub_8e70);
            goto label_9608;
          }
          if (v21) goto label_9437;
label_9811:
          if (v8 != -1) goto label_9820;
label_9829:
          if ((dat_26608) && ((dat_26586 || ((dat_26585 && (v8 == -1)))))) goto label_95f7;
          if (!a0)
            __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
label_9473:
          v19 = 1;
          v6 = (unsigned long)(unsigned long)(v8 == -1);
          v6 = (long)((long)v6 * 0x130);
          sub_e0e0("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),a1);
        }
label_910c:
        v5 = fflush_unlocked(stdout);
        if (v5) {
          v6 = (char *)dcgettext(NULL,"standard output",5);
          sub_e1f0(v6); // no-return
        }
      }
      else {
        if (v39 != -1) {
          v20 = 0;
          v5 = v23 & 0xf000;
          goto label_95aa;
        }
label_9610:
        v19 = 0;
        if ((dat_26584) && (v5 = v23 & 0xf000, v5 != 0x4000)) {
          v9 = dat_26548;
          if (!dat_26548)
            v9 = v40;
          v15 = dat_26540;
          if (!dat_26540)
            v15 = v13;
          sub_e0e0("Files %s and %s are identical\n",v15,v9);
        }
      }
      free(v18);
      free(v22);
      v5 = v19;
      return v5;
    }
  }
  else if ((((v6[10] & 0xf000U) == 0x8000) && (!(v6[10] & 0x1ffU))) && (!*(long *)&v6[0x10])) goto label_91fa;
label_909b:
  if (v19 == 2) goto label_90a5;
label_920a:
  v6 = &v6[0x4c];
  v14 = &v14[-0x4c];
  goto label_9084;
}


// Function: sub_9cd0 @ 0x9cd0
unsigned long sub_9cd0(long a0,int a1) // return-dupe x2
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
      if ((v3 != 0x2000) || ((*(unsigned int *)(a0 + 0x28) & 0xf000) != 0x2000)) goto label_9d1a;
      if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
        return 1;
label_9d5c:
      v1 = *(long *)(v1 + 0x260);
    }
    else {
      if ((*(unsigned int *)(a0 + 0x28) & 0xf000) == 0x6000) {
        if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
          return 1;
        goto label_9d5c;
      }
label_9d1a:
      v1 = *(long *)(v1 + 0x260);
    }
    if (!v1)
      return 0;
  } while( true );
}


// Function: sub_9da0 @ 0x9da0
void sub_9da0(char *a0,char *a1)
{
  int *v1; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  if (dat_26551) {
    strcasecmp(a0,a1); // tail-call
    return;
  }
  strcoll(a0,a1); // tail-call
}


// Function: sub_9df0 @ 0x9df0
void sub_9df0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_266e8) {
    if (sub_9da0(v1,v2))
      return;
  }
  strcmp(v1,v2); // tail-call
}


// Function: sub_9e40 @ 0x9e40
void sub_9e40(char *a0,char *a1)
{
  if (!dat_266e8) {
    strcmp(a0,a1); // tail-call
    return;
  }
  if (!sub_9da0(a0,a1)) {
    if (!dat_26551) {
      strcmp(a0,a1); // tail-call
      return;
    }
  }
}


// Function: sub_9e90 @ 0x9e90
unsigned long sub_9e90(int *a0,unsigned long *a1) // return-dupe x2
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
    v5 = (char *)sub_15f50(0x200);
    a1[2] = (unsigned long)v5;
    v6 = __errno_location();
    v13 = 0;
    v14 = 0;
label_9f10:
    *v6 = 0;
    v7 = readdir(v4);
    if (v7) {
      v1 = (char *)((long)v7 + 0x13);
      v8 = strlen(v1);
      if (*(char *)((long)v7 + 0x13) == '.') goto label_9f48;
      goto label_9f58;
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
      sub_16300(); // no-return, return-dupe
    v9 = (unsigned long *)sub_15f50(v14 * 8 + 8);
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
    v9 = (unsigned long *)sub_15f50(8);
    *a1 = 0;
    a1[1] = (unsigned long)v9;
  }
  *v9 = 0;
  return 1;
label_9f48:
  if ((*(char *)((long)v7 + 0x14)) && ((*(char *)((long)v7 + 0x14) != '.' || (*(char *)((long)v7 + 0x15))))) {
label_9f58:
    v3 = sub_10220(dat_26410,v1);
    if (!v3) {
      v10 = v13 + v8 + 1;
      while (v12 < v10) {
        if (0x3ffffffffffffffe < v12) {
          sub_16300();
        }
        v12 *= 2;
        v5 = (char *)sub_15fa0(v5,v12);
        a1[2] = (unsigned long)v5;
      }
      memcpy(&v5[v13],v1,v8 + 1);
      v14 += 1;
      v13 = v10;
    }
  }
  goto label_9f10;
}


// Function: sub_a0c0 @ 0xa0c0
int sub_a0c0(int *a0,void *a1) // early-return
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
  if (((v4 == -1) || (sub_9cd0(a0,0))) && ((a0[0x4c] == -1 || (sub_9cd0(a0,1))))) {
    v1 = *(unsigned long *)&a0[(unsigned long)(v4 == -1) * 0x4c + 2];
    error(0,0,dcgettext(NULL,"%s: recursive directory loop",5),v1);
    return 2;
  }
  if (!sub_9e90(a0,&v6[2])) {
    sub_df30(*(unsigned long *)&a0[2]);
    v11 = 2;
  }
  if (sub_9e90(&a0[0x4c],&v7)) { // branch-flip
    if (!v11) {
      dat_266e8 = 1;
      v6[0] = v6[3];
      v6[1] = v13;
      if (_setjmp(0x26620))
        dat_266e8 = 0;
      qsort(v6[0],v6[2],8,sub_9df0);
      qsort(v6[1],v7,8,sub_9df0);
      v9 = dat_26488;
      if ((dat_26488) && (!*(long *)&a0[0x98])) {
        while ((*v6[0] && (v5 = *v6[0], (int)sub_9e40(v5,v9) <= -1))) {
          v6[0] = &v6[0][1];
        }
        while ((*v6[1] && (v5 = *v6[1], (int)sub_9e40(v5,v9) <= -1))) {
          v6[1] = &v6[1][1];
        }
      }
      while ((*v6[0] || (*v6[1]))) {
        if (*v6[0]) { // branch-flip
          if (*v6[1]) { // branch-flip
            v4 = sub_9e40(*v6[0],*v6[1]);
            if (v4) {
              if (0 <= v4) {
                v2 = &v6[1][1];
                v5 = *v6[1];
                v9 = 0;
                v6[1] = v2;
                goto label_a2f5;
              }
              goto label_a33b;
            }
            if ((dat_26551) && (v4 = strcmp((char *)*v6[0],(char *)*v6[1]), v4)) {
              v2 = v6[(v4 >> 0x1f) + 1];
              v8 = (char *)v2[1];
              v3 = (char *)*v6[(unsigned int)-(v4 >> 0x1f)];
              v10 = &v2[1];
              if (v8) {
                while (!sub_9e40(v8,v3)) {
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
label_a33b:
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
label_a2f5:
        v4 = (*a1)(a0,v9,v5);
        if (v11 < v4)
          v11 = v4;
      }
    }
  }
  else {
    sub_df30(*(unsigned long *)&a0[0x4e]);
    v11 = 2;
  }
  free(v6[3]);
  free(v12);
  free(v13);
  free(v14);
  return v11;
}


// Function: sub_a4f0 @ 0xa4f0
unsigned long sub_a4f0(unsigned long a0,char *a1)
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
    if (sub_9e90(v6,v5)) {
      dat_266e8 = 1;
      if (_setjmp(0x26620)) // branch-flip
        v8 = a1;
      else {
        v2 = (char *)*v9;
        v3 = a1;
        v7 = v9;
        while (v8 = v3, v2) {
          if (!sub_9e40(v2,a1)) {
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
  v4 = sub_10920(a0,v8,0);
  free(v9);
  free(v10);
  return v4;
}


// Function: sub_a650 @ 0xa650
void sub_a650(unsigned long a0) // return-dupe x2
{
  char *v1;
  unsigned int v2; // eax
  long v3; // rbx
  long v4; // stack - 0x40
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x50
  long v7; // stack - 0x38
  
  v2 = sub_f1a0(a0,&v6,&v5,&v4,&v7);
  if (!v2)
    return;
  sub_e2b0();
  sub_f0f0(0x2c,0x261a0,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v2 + 0x1d550),dat_26188);
  fputc_unlocked(10,dat_26188);
  if (v2 == 1)
    return;
  if (v4 <= v7) {
    do {
      while( true ) {
        v3 = v4 + 1;
        v1 = *(char **)(dat_26388 + v4 * 8);
        v4 = v3;
        if ((*v1 != '.') || (v1[1] != '\n')) break;
        fputs_unlocked("..\n.\ns/.//\n",dat_26188);
        if (v7 < v3)
          return;
        fputs_unlocked("a\n",dat_26188);
      }
      sub_f0a0("");
    } while (v3 <= v7);
  }
  fputs_unlocked(".\n",dat_26188);
}


// Function: sub_a7a0 @ 0xa7a0
void sub_a7a0(unsigned long a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2; // rbx
  long v3; // stack - 0x30
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x28
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x1d550),dat_26188);
  sub_f0f0(0x20,0x261a0,v5,v4);
  fputc_unlocked(10,dat_26188);
  if (v1 == 1)
    return;
  if (v3 <= v6) {
    do {
      v2 = v3 + 1;
      sub_f0a0("",dat_26388 + v3 * 8);
      v3 = v2;
    } while (v2 <= v6);
  }
  fputs_unlocked(".\n",dat_26188);
}


// Function: sub_a8a0 @ 0xa8a0
void sub_a8a0(unsigned long a0) // return-dupe x3
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
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  sub_f0c0(0x261a0,v5,v4,&v7,&v8);
  if (v1 & 1) {
    v2 = 1;
    if (v7 <= v8)
      v2 = (v8 - v7) + 1;
    __fprintf_chk(dat_26188,1,"d%ld %ld\n",v7,v2);
  }
  if (!(v1 & 2))
    return;
  sub_f0c0(0x262d0,v3,v6,&v9,&v10);
  v2 = 1;
  if (v9 <= v10)
    v2 = (v10 - v9) + 1;
  __fprintf_chk(dat_26188,1,"a%ld %ld\n",v8,v2);
  if (v6 < v3)
    return;
  do {
    v2 = v3 + 1;
    sub_f0a0("",dat_26388 + v3 * 8);
    v3 = v2;
  } while (v2 <= v6);
}


// Function: sub_aa10 @ 0xaa10
void sub_aa10(unsigned long a0)
{
  sub_eb10(a0,sub_eb00,sub_a650); // tail-call
}


// Function: sub_aa30 @ 0xaa30
void sub_aa30(unsigned long a0)
{
  sub_eb10(a0,sub_eaf0,sub_a7a0); // tail-call
}


// Function: sub_aa50 @ 0xaa50
void sub_aa50(unsigned long a0)
{
  sub_eb10(a0,sub_eaf0,sub_a8a0); // tail-call
}


// Function: sub_aa70 @ 0xaa70
char * sub_aa70(char *a0,char *a1) // return-dupe x3
{
  char v1;
  char v2;
  int v3;
  char *v4;
  char *v5; // r8
  
  v1 = *a0;
  if (v1 == '\'')
    return NULL;
  if (v1 != '\\') { // branch-flip
    if (!v1)
      return NULL;
    v4 = NULL;
    if (a0[1] == '\'') {
      *a1 = v1;
      v4 = &a0[2];
    }
  }
  else {
    v3 = 0;
    v2 = '\0';
    v1 = a0[1];
    v4 = &a0[2];
    while (v1 != '\'') {
      if (8 <= (unsigned int)((int)v1 - 0x30U))
        return NULL;
      v5 = &v4[1];
      v3 = ((int)v1 - 0x30U) + v3 * 8;
      v2 = (char)v3;
      v1 = *v4;
      v4 = v5;
    }
    if ((char *)0x3 <= &v4[0xfffffffffffffffd - (long)a0])
      return NULL;
    *a1 = v2;
  }
  return v4;
}


// Function: sub_aaf0 @ 0xaaf0
long sub_aaf0(unsigned long *a0,char a1) // early-return
{
  unsigned long v1;
  unsigned long v2;
  
  switch(a1) {
    case 0x45:
      a0 = &a0[3];
      break;
    case 0x46:
      a0 = &a0[3];
label_ab64:
      v1 = a0[1];
      v2 = *a0;
      return sub_f0b0(v2,v1); // tail-call
    default:
      switch(a1) {
        case 0x65:
          break;
        case 0x66:
          goto label_ab64;
        default:
          return -1;
        case 0x6c:
          goto label_ab7c;
        case 0x6d:
          goto label_ab9c;
        case 0x6e:
          goto label_abb4;
        
      }
    case 0x4c:
      a0 = &a0[3];
label_ab7c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f0b0(v2,v1) + -1;
    case 0x4d:
      a0 = &a0[3];
label_ab9c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f0b0(v2,v1); // tail-call
    case 0x4e:
      a0 = &a0[3];
label_abb4:
      return a0[2] - a0[1];
    
  }
  v1 = a0[1];
  v2 = *a0;
  return sub_f0b0(v2,v1) + -1;
}


// Function: sub_abe0 @ 0xabe0
unsigned char * sub_abe0(void *a0,void *a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned char v1;
  char v10; // stack - 0x41
  char *v11;
  char *v12; // rsp
  char *v13; // rsp
  char *v14; // rsp
  unsigned long v15; // rdi
  char *v16; // rax
  unsigned char *v17; // stack - 0x60
  char *v18; // stack - 0x58
  unsigned char *v19; // stack - 0x50
  long v2;
  char *v3;
  unsigned char v4; // al
  unsigned char *v5;
  unsigned int v6;
  unsigned int v7;
  unsigned char *v8;
  char v9 [8];
  
  v11 = v9;
  v13 = v9;
  v14 = v9;
  v5 = (unsigned char *)((long)a1 + 1);
  do {
    v1 = *v5;
    v7 = (unsigned int)v1;
    v5 = &v5[1];
    if (0x31 <= v1) {
      if ((unsigned int)((int)(char)v1 - 0x30U) <= 9) goto label_aca0;
      goto label_ac40;
    }
  } while (!(0xfffedf7fffffffffU >> ((unsigned long)v1 & 0x3f) & 1));
  v7 = (unsigned int)v1;
  v6 = (int)(char)v1;
  while (v6 - 0x30 <= 9) {
label_aca0:
    v1 = *v5;
    v5 = &v5[1];
    v7 = (unsigned int)(char)v1;
    v6 = v7;
  }
  if (v1 == 0x2e) {
    do {
      v7 = (unsigned int)(char)*v5;
      v5 = &v5[1];
    } while (v7 - 0x30 <= 9);
  }
label_ac40:
  v1 = *v5;
  v5 = &v5[1];
  v4 = (unsigned char)(v7 - 0x58);
  if (v4 <= 0x20) {
    if (0x100801001U >> ((unsigned long)(v7 - 0x58) & 0x3f) & 1) { // branch-flip
      if (a2) { // branch-flip
        if (v1 == 0x6e) {
          v18 = (long)sub_f0b0(a2,a3);
label_ad1a:
          if (!a0)
            return v5;
          v17 = &v5[0xfffffffffffffffe - (long)a1];
          v19 = &v5[0xffffffffffffffff - (long)a1];
          if (0xfa1 <= &v5[1 - (long)a1]) { // branch-flip
            v16 = (char *)sub_16340();
            v8 = v17;
          }
          else {
            v3 = v9;
            while (v13 != &v9[-((unsigned long)&v5[0x37 - (long)a1] & 0xfffffffffffff000)]) {
              v11 = &v3[-0x1000];
              v12 = &v3[-0x1000];
              *(unsigned long *)&v3[-8] = *(unsigned long *)&v3[-8];
              v13 = &v3[-0x1000];
              v3 = v12;
            }
            v15 = (unsigned long)((unsigned int)&v5[0x37 - (long)a1] & 0xff0);
            v2 = -v15;
            v14 = &v11[v2];
            if (v15)
              *(unsigned long *)&v11[-8] = *(unsigned long *)&v11[-8];
            v16 = (char *)(((unsigned long)&v11[v2 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
            v8 = v17;
          }
          v17 = (unsigned char *)v18;
          v18 = &v16[(long)v19];
          v19 = v8;
          *(unsigned long *)&v14[-8] = 0xadb3;
          memcpy(v16,a1,(unsigned long)v8,v14[-8]);
          v16[(long)v19] = 'l';
          *v18 = (char)v7;
          v18[1] = '\0';
          *(unsigned long *)&v14[-8] = 0xadde;
          __fprintf_chk(a0,1,v16,v17);
          *(unsigned long *)&v14[-8] = 0xade6;
          sub_11ce0(v16);
          return v5;
        }
      }
      else {
        v18 = (long)sub_aaf0(a4,(int)(char)v1);
        if (0 <= (long)v18) goto label_ad1a;
      }
    }
    else if ((v4 == 0xb) && (v1 == 0x27)) {
      v5 = (unsigned char *)sub_aa70(v5,&v10);
      if (!v5)
        return NULL;
      if (!a0)
        return v5;
      putc_unlocked((int)v10,a0);
      return v5;
    }
  }
  return NULL;
}


// Function: sub_ae40 @ 0xae40
void sub_ae40(void *a0,char *a1,long *a2) // return-dupe x3
{
  char *v1;
  int v10; // edi
  void *v11;
  char *v12;
  char v2;
  long v3;
  long v4;
  long v5;
  char *v6;
  unsigned long v7; // rax
  long v8;
  long v9;
  
  v3 = *a2;
  v8 = a2[1];
  v4 = *(long *)(v3 + 0xb8);
  v5 = a2[2];
  if (!a0)
    return;
  if ((!dat_264a0) && (*a1 == '%')) {
    if (a1[1] != 'l') { // branch-flip
      if ((a1[1] == 'L') && (!a1[2])) {
        v11 = *(void **)(v4 + v8 * 8);
        v7 = *(long *)(v4 + v5 * 8) - (long)v11;
        fwrite_unlocked(v11,1,v7,a0); // return-dupe, tail-call
        return;
      }
    }
    else if ((a1[2] == '\n') && (!a1[3])) {
      if (v5 <= v8)
        return;
      v11 = *(void **)(v4 + v8 * 8);
      v3 = *(long *)(v4 + v5 * 8);
      v7 = (v3 + (unsigned long)(*(char *)(v3 + -1) != '\n')) - (long)v11;
      fwrite_unlocked(v11,1,v7,a0);
      return;
    }
  }
  if (v5 <= v8)
    return;
  do {
    v2 = *a1;
    v6 = a1;
    while( true ) {
      v12 = &v6[1];
      if (!v2) break;
      while (v10 = (int)v2, v2 == '%') {
        v2 = v6[1];
        v1 = &v6[2];
        if (v2 != 'L') { // branch-flip
          if (v2 != 'l') {
            if (v2 != '%') { // branch-flip
              v6 = (char *)sub_abe0(a0,v6,v3,v8,0);
              if (v6) goto label_aecb;
              v10 = 0x25;
            }
            else {
              v10 = 0x25;
              v12 = v1;
            }
            break;
          }
          v9 = *(long *)(v4 + 8 + v8 * 8);
          v9 -= (unsigned long)(*(char *)(v9 + -1) == '\n');
        }
        else {
          v9 = *(long *)(v4 + 8 + v8 * 8);
        }
        v12 = &v6[3];
        sub_ed50(*(unsigned long *)(v4 + v8 * 8),v9,0,0);
        v2 = *v1;
        v6 = v1;
        if (!v2) goto label_af6b;
      }
      putc_unlocked(v10,a0);
      v6 = v12;
label_aecb:
      v2 = *v6;
    }
label_af6b:
    v8 += 1;
    if (v5 == v8)
      return;
  } while( true );
}


// Function: sub_b040 @ 0xb040
void sub_b040(void *a0,char *a1,char a2,long a3)
{
  void *v1;
  char *v10; // stack - 0x60
  long v11 [3]; // stack - 0x58
  unsigned long v12;
  void *v13;
  char *v2;
  int *v3; // rax
  char *v4;
  char *v5;
  long v6; // rax
  long v7;
  char v8;
  char v9;
  
  v8 = *a1;
  if ((v8 == a2) || (!v8))
    return;
  do {
    v5 = &a1[1];
    v9 = v8;
    if (v8 != '%') {
label_b08e:
      v2 = v5;
      if (a0)
        putc_unlocked((int)v9,a0);
      v8 = *v2;
      goto label_b0a5;
    }
    v9 = a1[1];
    v2 = &a1[2];
    v7 = a3;
    switch(v9) {
      case 0x25:
        v5 = v2;
        goto label_b08e;
      default:
        v2 = (char *)sub_abe0(a0,a1,0,0,a3);
        v9 = v8;
        if (!v2) goto label_b08e;
        v8 = *v2;
        goto label_b0a5;
      case 0x28:
        v7 = 0;
        while( true ) {
          if (10 <= (unsigned int)((int)*v2 - 0x30U)) { // branch-flip
            v6 = sub_aaf0(a3);
            v11[v7] = v6;
            v9 = v8;
            if (v6 < 0) goto label_b08e;
            v4 = &v2[1];
          }
          else {
            v3 = __errno_location();
            *v3 = 0;
            v11[v7] = strtoimax(v2,&v10,10);
            v9 = v8;
            if (*v3) goto label_b08e;
            v4 = v10;
          }
          v2 = &v4[1];
          v9 = v8;
          if (*v4 != "=?"[v7]) goto label_b08e;
          if (v7 == 1) break;
          v7 = 1;
        }
        if (v11[0] != v11[1]) { // branch-flip
          v13 = a0;
          v1 = NULL;
        }
        else {
          v13 = NULL;
          v1 = a0;
        }
        v5 = (char *)sub_b040(v1,v2,0x3a,a3);
        if (!*v5)
          return;
        v2 = (char *)sub_b040(v13,&v5[1],0x29,a3);
        if (!*v2)
          return;
        v8 = v2[1];
        v2 = &v2[1];
        goto label_b0a5;
      case 0x3c:
        v12 = dat_26448;
        break;
      case 0x3d:
        v12 = dat_26440;
        break;
      case 0x3e:
        v7 = a3 + 0x18;
        v12 = dat_26450;
      
    }
    sub_ae40(a0,v12,v7);
    v8 = a1[2];
label_b0a5:
    if (!v8)
      return;
    a1 = v2;
    if (a2 == v8)
      return;
  } while( true );
}


// Function: sub_b2e0 @ 0xb2e0
void sub_b2e0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  v1 = 0x261a0;
  v4 = 0x262d0;
  v2 = a1;
  v3 = a2;
  v5 = a3;
  v6 = a4;
  sub_b040(dat_26188,a0,0,&v1);
}


// Function: sub_b350 @ 0xb350
void sub_b350(unsigned long a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x28
  long v4; // stack - 0x30
  long v5; // stack - 0x18
  
  v1 = sub_f1a0(a0,&v4,&v3,&v2,&v5);
  if (!v1)
    return;
  sub_e2b0();
  if ((dat_266f8 < v4) || (dat_266f0 < v2))
    sub_b2e0(dat_26460,dat_266f8,v4,dat_266f0,v2);
  dat_266f8 = v3 + 1;
  dat_266f0 = v5 + 1;
  sub_b2e0(*(unsigned long *)((unsigned long)v1 * 8 + 0x26460),v4,dat_266f8,v2,dat_266f0);
}


// Function: sub_b430 @ 0xb430
void sub_b430(unsigned long a0)
{
  dat_266f0 = -dat_26288;
  dat_266f8 = dat_266f0;
  sub_eb10(a0,sub_eaf0,sub_b350);
  if ((dat_26270 <= dat_266f8) && (dat_263a0 <= dat_266f0))
    return;
  sub_e2b0();
  sub_b2e0(dat_26460,dat_266f8,dat_26270,dat_266f0,dat_263a0); // tail-call
}


// Function: sub_b4c0 @ 0xb4c0
long sub_b4c0(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = 0x20;
  if (10 <= a0)
    v1 = a1 / (unsigned long)(a0 - 1U);
  a2 /= v1;
  if (!a2)
    a2 = 1;
  if (0x787878787878783 <= a2)
    a2 = 0x787878787878782;
  return a2 + 5;
}


// Function: sub_b510 @ 0xb510
void sub_b510(long a0) // return-dupe
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
          if (v1 == v4) goto label_b5a0;
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
label_b5a0:
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


// Function: sub_b600 @ 0xb600
void sub_b600(long a0)
{
  long *v1;
  unsigned int v10; // edx
  unsigned char *v11;
  unsigned long v12; // rcx
  unsigned long v13; // rsi
  unsigned long v14;
  unsigned long v15; // rax
  unsigned long *v16;
  unsigned long v17; // r13
  long v18;
  unsigned char *v19;
  unsigned char v2;
  unsigned char *v20;
  unsigned char *v21;
  bool v22; // cf
  bool v23; // zf
  unsigned long v24; // stack - 0xc0
  unsigned long *v25; // stack - 0xb8
  unsigned char *v26; // stack - 0xb0
  unsigned long v27; // stack - 0xa8
  unsigned long v28; // stack - 0xa0
  unsigned long v29; // stack - 0x90
  unsigned char v3;
  long v30; // stack - 0x88
  long v31; // stack - 0x80
  unsigned long v32; // stack - 0x78
  long v4;
  void *v5;
  char v6;
  unsigned int v7;
  unsigned char *v8; // rax
  long v9;
  
  v30 = *(long *)(a0 + 0xb8);
  v4 = *(long *)(a0 + 0xc0);
  v21 = *(unsigned char **)(a0 + 0xe0);
  v32 = *(unsigned long *)(a0 + 0xd8);
  v31 = sub_15f50(v32 * 8);
  v7 = dat_26558;
  v6 = dat_26552;
  v20 = *(unsigned char **)(a0 + 0xf0);
  v27 = dat_26708;
  v29 = dat_26700;
  v8 = (unsigned char *)(*(long *)(a0 + 0xb0) + *(long *)(a0 + 0xa0));
  v23 = dat_26558 == 0;
  v9 = dat_26710;
  if (v21 < v20) {
    v24 = 0;
    v10 = dat_26558 & 1;
    v11 = v21;
    do {
      v14 = dat_26498;
      v2 = *v11;
      v21 = &v11[1];
      v15 = (unsigned long)v2;
      if (v7 != 4) { // branch-flip
        if (5 <= v7) { // branch-flip
          if (v7 != 5) { // branch-flip
label_bbe8:
            if (v6) { // branch-flip
              if (v2 == 10) goto label_bd57;
              v13 = 0;
              v19 = v21;
              do {
                v21 = &v19[1];
                v18 = v15 * 4;
                v2 = *v19;
                v15 = (unsigned long)v2;
                v13 = (v13 << 7 | v13 >> 0x39) + (long)*(int *)(*(long *)__ctype_tolower_loc() + v18);
                v19 = v21;
              } while (v2 != 10);
              v26 = &v21[0xffffffffffffffff - (long)v11];
            }
            else {
              v13 = 0;
              v19 = v21;
              if (v2 != 10) { // branch-flip
                do {
                  v21 = &v19[1];
                  v13 = (v13 << 7 | v13 >> 0x39) + v15;
                  v2 = *v19;
                  v15 = (unsigned long)v2;
                  v19 = v21;
                } while (v2 != 10);
                v26 = &v21[0xffffffffffffffff - (long)v11];
              }
              else {
                v26 = NULL;
              }
            }
          }
          else {
            if (v2 == 10) goto label_bd57;
            v13 = 0;
            v18 = *(long *)__ctype_b_loc();
            do {
              if (!(*(unsigned char *)(v18 + 1 + v15 * 2) & 0x20)) {
                if (v6)
                  v15 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v15 * 4);
                v13 = (v13 << 7 | v13 >> 0x39) + v15;
              }
              v2 = *v21;
              v15 = (unsigned long)v2;
              v21 = &v21[1];
            } while (v2 != 10);
            v26 = &v21[0xffffffffffffffff - (long)v11];
          }
        }
        else {
          if (v7 - 1 > 2) goto label_bbe8;
          if (v2 == 10) goto label_bd57;
          v15 = 0;
          v13 = 0;
          v19 = v21;
          do {
            v17 = (unsigned long)v2;
            if (v7 & 2) {
              v18 = *(long *)__ctype_b_loc();
              if (*(unsigned char *)(v18 + 1 + v17 * 2) & 0x20) {
                v21 = v19;
                do {
                  v3 = *v21;
                  v21 = &v21[1];
                  if (v3 == 10) {
                    v26 = &v21[0xffffffffffffffff - (long)v11];
                    goto label_b820;
                  }
                } while (*(unsigned char *)(v18 + 1 + (unsigned long)v3 * 2) & 0x20);
              }
            }
            if (v10) { // branch-flip
              if (v2 != 9) { // branch-flip
                if (v2 != 0xd) { // branch-flip
                  if (v2 == 8) {
                    v17 = 8;
                    v15 = (v15 - 1) + (unsigned long)(v15 == 0);
                    goto label_ba60;
                  }
                  v15 += 1;
                  v12 = 1;
                }
                else {
                  v17 = 0xd;
                  v12 = 1;
                  v15 = 0;
                }
              }
              else {
                v17 = 0x20;
                v12 = v14 - v15 % v14;
                v22 = CARRY8(v15,v12);
                v15 += v12;
                if (v22)
                  v15 = 0;
              }
            }
            else {
label_ba60:
              v12 = 1;
            }
            if (v6)
              v17 = (unsigned long)*(unsigned char *)(*(long *)__ctype_tolower_loc() + v17 * 4);
            do {
              v13 = (v13 << 7 | v13 >> 0x39) + v17;
              v12 -= 1;
            } while (v12);
            v2 = *v19;
            v21 = &v19[1];
            v19 = v21;
          } while (v2 != 10);
          v26 = &v21[0xffffffffffffffff - (long)v11];
        }
      }
      else {
        if (v2 != 10) {
          v13 = 0;
          v18 = *(long *)__ctype_b_loc();
          do {
            v19 = v21;
            if (*(unsigned char *)(v18 + 1 + v15 * 2) & 0x20) {
              v2 = *v21;
              while( true ) {
                v21 = &v21[1];
                if (v2 == 10) goto label_bcb3;
                v15 = (unsigned long)v2;
                if (!(*(unsigned char *)(v18 + 1 + v15 * 2) & 0x20)) break;
                v2 = *v21;
              }
              v13 = (v13 << 7 | v13 >> 0x39) + 0x20;
              v19 = v21;
            }
            if (v6)
              v15 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v15 * 4);
            v21 = &v19[1];
            v13 = (v13 << 7 | v13 >> 0x39) + v15;
            v15 = (unsigned long)*v19;
          } while (*v19 != 10);
label_bcb3:
          v26 = &v21[0xffffffffffffffff - (long)v11];
          goto label_b820;
        }
label_bd57:
        v26 = NULL;
        v13 = 0;
      }
label_b820:
      v16 = (unsigned long *)(dat_26720 + (v13 % dat_26718) * 8);
      v25 = v16;
      if ((((v21 == v8) && (*(char *)(a0 + 0x120))) && (2 <= (unsigned int)(dat_26580 - 4U))) && (v25 = (unsigned long *)(dat_26720 + -8), 2 <= v7))
        v25 = v16;
      v28 = *v25;
      if (v28) {
        v14 = v28;
label_b885:
        v16 = (unsigned long *)(v14 * 0x20 + v9);
        if (v16[1] == v13) {
          v5 = (void *)v16[2];
          if ((unsigned char *)v16[3] == v26) {
            if (!memcmp(v5,v11,(unsigned long)v26)) goto label_b8cb;
            if (!(v23 && !v6)) goto label_b8aa;
            v14 = *v16;
            if (!v14) goto label_b953;
            goto label_b885;
          }
          if (v7) {
label_b8aa:
            if (sub_e690(v5,v11)) goto label_b878;
            goto label_b8cb;
          }
        }
label_b878:
        v14 = *v16;
        if (!v14) goto label_b953;
        goto label_b885;
      }
label_b953:
      if (v29 == v27) {
        if (0x1fffffffffffffe < v29) {
          sub_16300(); // no-return, return-dupe
        }
        v14 = v29 << 1;
        v9 = sub_15fa0(v9,v29 << 6);
        v28 = *v25;
        v29 = v14;
      }
      v16 = (unsigned long *)(v27 * 0x20 + v9);
      *v16 = v28;
      v16[1] = v13;
      v16[2] = (unsigned long)v11;
      v16[3] = (unsigned long)v26;
      *v25 = v27;
      v14 = v27;
      v27 += 1;
label_b8cb:
      if (v24 == v32) {
        if (((0x2aaaaaaaaaaaaaa9 < (long)v24) || (v32 = v24 * 2 - v4, 0xffffffffffffffe < v32)) || (0xffffffffffffffe < v24 - v4)) {
          sub_16300();
        }
        v31 = sub_15fa0(v31,v32 * 8);
        v30 = sub_15fa0(v30 + v4 * 8,(v32 - v4) * 8) + v4 * -8;
      }
      *(unsigned char **)(v30 + v24 * 8) = v11;
      *(unsigned long *)(v31 + v24 * 8) = v14;
      v24 += 1;
      if (v20 <= v21) goto label_bd68;
      v11 = v21;
    } while( true );
  }
  v24 = 0;
label_bd6b:
  v18 = 0;
  *(unsigned long *)(a0 + 200) = v24;
  do {
    if (v32 != v24) { // branch-flip
      v1 = (long *)(v30 + v24 * 8);
      *v1 = (long)v21;
    }
    else {
      if (((0x2aaaaaaaaaaaaaa9 < (long)v32) || (v14 = v32 * 2 - v4, 0xffffffffffffffe < v14)) || (0xffffffffffffffe < v32 - v4))
        sub_16300();
      v30 = sub_15fa0(v30 + v4 * 8,(v14 - v4) * 8) + v4 * -8;
      v1 = (long *)(v30 + v24 * 8);
      *v1 = (long)v21;
      v32 = v14;
    }
    if (v21 == v8) {
      if ((*(char *)(a0 + 0x120)) && (2 <= (unsigned int)(dat_26580 - 4U)))
        *v1 = (long)&v21[-1];
label_bea8:
      dat_26710 = v9;
      *(long *)(a0 + 0xb8) = v30;
      *(unsigned long *)(a0 + 0xd0) = v24;
      *(unsigned long *)(a0 + 0xd8) = v32;
      *(long *)(a0 + 0xf8) = v31;
      dat_26700 = v29;
      dat_26708 = v27;
      return;
    }
    if ((dat_26570 <= v18) && (dat_26578)) goto label_bea8;
    v24 += 1;
    v20 = v21;
    do {
      v21 = &v20[1];
      v2 = *v20;
      v20 = v21;
    } while (v2 != 10);
    v18 += 1;
  } while( true );
label_bd68:
  goto label_bd6b;
}


// Function: sub_bf50 @ 0xbf50
void sub_bf50(unsigned int *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    return;
  if (*(char *)((long)a0 + 0x121))
    return;
  v1 = sub_16520(*a0,*(long *)&a0[0x2c] + *(long *)&a0[0x28],a1);
  if (v1 != 0xffffffffffffffff) {
    *(unsigned long *)&a0[0x2c] = *(long *)&a0[0x2c] + v1;
    *(bool *)((long)a0 + 0x121) = v1 < a1;
    return;
  }
  sub_e1f0(*(unsigned long *)&a0[2]); // no-return
}


// Function: sub_bfd0 @ 0xbfd0
unsigned long sub_bfd0(int *a0,char a1)
{
  unsigned long v1;
  int v2;
  unsigned long v3; // rax
  void *v4; // rax
  
  if (0 <= *a0) { // branch-flip
    v3 = sub_165d0(8,*(unsigned long *)&a0[0x12],0x7fffffffffffffef);
    *(unsigned long *)&a0[0x2a] = v3;
    *(unsigned long *)&a0[0x28] = sub_15f50(v3);
    if (!a1) {
      v2 = sub_f710(*a0,0);
      sub_bf50(a0,*(unsigned long *)&a0[0x2a]);
      v1 = *(unsigned long *)&a0[0x2c];
      if (v2) {
        v2 = *a0;
        if ((long)lseek(v2,-v1,1) < 0)
          sub_e1f0(*(unsigned long *)&a0[2]); // no-return
        a0[0x2c] = 0;
        a0[0x2d] = 0;
        *(char *)((long)a0 + 0x121) = 0;
      }
      v4 = memchr(*(void **)&a0[0x28],0,v1);
      return CONCAT71((undefined7)((unsigned long)v4 >> 8),v4 != NULL);
    }
  }
  else {
    a0[0x2a] = 8;
    a0[0x2b] = 0;
    *(unsigned long *)&a0[0x28] = sub_15f50(8);
  }
  a0[0x2c] = 0;
  a0[0x2d] = 0;
  *(char *)((long)a0 + 0x121) = 0;
  return 0;
}


// Function: sub_c0d0 @ 0xc0d0
void sub_c0d0(int *a0) // return-dupe x2
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  
  if (*a0 < 0)
    return;
  if ((a0[10] & 0xf000U) != 0x8000) // branch-flip
    v2 = *(unsigned long *)&a0[0x2c];
  else {
    v3 = *(unsigned long *)&a0[0x10];
    v2 = (v3 & 0xfffffffffffffff8) + 0x10;
    if ((v2 < v3) || (0x7ffffffffffffffe < v2))
      sub_16300(); // no-return, return-dupe
    if (v2 <= *(unsigned long *)&a0[0x2a]) // branch-flip
      v2 = *(unsigned long *)&a0[0x2c];
    else {
      *(unsigned long *)&a0[0x2a] = v2;
      v1 = *(unsigned long *)&a0[0x28];
      *(unsigned long *)&a0[0x28] = sub_15fa0(v1);
      v2 = *(unsigned long *)&a0[0x2c];
    }
    if (v2 <= v3) {
      sub_bf50(a0,(v3 - v2) + 1);
      v2 = *(unsigned long *)&a0[0x2c];
      if (v2 <= v3)
        return;
    }
  }
  sub_bf50(a0,*(long *)&a0[0x2a] - v2);
  v2 = *(unsigned long *)&a0[0x2c];
  if (!v2)
    return;
  v3 = *(unsigned long *)&a0[0x2a];
  if (v2 == v3) {
    do {
      if (0x3ffffffffffffff7 < v3) {
        sub_16300();
      }
      *(unsigned long *)&a0[0x2a] = v3 * 2;
      v1 = sub_15fa0(*(unsigned long *)&a0[0x28]);
      *(unsigned long *)&a0[0x28] = v1;
      sub_bf50(a0,*(long *)&a0[0x2a] - *(long *)&a0[0x2c]);
      v2 = *(unsigned long *)&a0[0x2c];
      v3 = *(unsigned long *)&a0[0x2a];
    } while (v2 == v3);
  }
  v1 = *(unsigned long *)&a0[0x28];
  *(unsigned long *)&a0[0x2a] = v2 + 0x10 & 0xfffffffffffffff8;
  *(unsigned long *)&a0[0x28] = sub_15fa0(v1);
}


// Function: sub_c270 @ 0xc270
void sub_c270(int *a0)
{
  unsigned char *v1;
  long v10;
  unsigned long v11; // rax
  unsigned long v12;
  unsigned long v13; // rcx
  unsigned int v14;
  unsigned char *v15;
  unsigned char *v16;
  unsigned long v17;
  unsigned long v18;
  unsigned char *v19;
  unsigned char *v2;
  unsigned char *v20;
  bool v21; // zf
  unsigned char *v22; // stack - 0x70
  unsigned char *v23; // stack - 0x50
  unsigned char *v24; // stack - 0x48
  long v25; // stack - 0x40
  unsigned char v3;
  unsigned char *v4;
  unsigned char *v5;
  unsigned long v6;
  long v7;
  long v8;
  unsigned long v9;
  
  sub_c0d0(a0);
  sub_b510(a0);
  if (*a0 != a0[0x4c]) { // branch-flip
    sub_c0d0(&a0[0x4c]);
    sub_b510(&a0[0x4c]);
    v22 = *(unsigned char **)&a0[0x74];
    v20 = *(unsigned char **)&a0[0x28];
    v12 = *(unsigned long *)&a0[0x2c];
    v18 = *(unsigned long *)&a0[0x78];
    if (v20 == v22) goto label_c2da;
    v19 = &v20[v12];
    v23 = &v22[v18];
    if (v18 <= v12) // branch-flip
      *v23 = ~v20[v18];
    else {
      *v19 = ~v22[v12];
    }
    v5 = v20;
    v15 = v22;
    if (*(long *)v20 == *(long *)v22) {
      do {
        v5 = &v5[8];
        v15 = &v15[8];
      } while (*(long *)v5 == *(long *)v15);
    }
    if (*v15 == *v5) {
      do {
        v5 = &v5[1];
        v15 = &v15[1];
      } while (*v5 == *v15);
    }
    v14 = dat_26580 - 4;
    v7 = dat_26560;
    if ((2 <= v14) && (&v20[v12 - *(unsigned char *)&a0[0x48]] < v5 != &v22[v18 - *(unsigned char *)&a0[0x94]] < v15)) {
      v5 = &v5[-1];
      v15 = &v15[-1];
    }
  }
  else {
    v18 = *(unsigned long *)&a0[0x2c];
    v20 = *(unsigned char **)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned char **)&a0[0x74] = v20;
    *(unsigned long *)&a0[0x78] = v18;
    *(char *)&a0[0x94] = (char)a0[0x48];
    v12 = v18;
label_c2da:
    v5 = &v20[v18];
    v19 = &v20[v12];
    v14 = dat_26580 - 4;
    v15 = v5;
    v7 = dat_26560;
    v23 = v5;
    v22 = v20;
  }
  while (v6 = dat_26570, v20 != v5) {
    while (v5[-1] != 10) {
      v5 = &v5[-1];
      v15 = &v15[-1];
      if (v20 == v5) goto label_c33c;
    }
    v10 = v7 + -1;
    if (!v7) break;
    v15 = &v15[-1];
    v5 = &v5[-1];
    v7 = v10;
  }
label_c33c:
  *(unsigned char **)&a0[0x38] = v5;
  *(unsigned char **)&a0[0x84] = v15;
  if (2 <= v14) { // branch-flip
    v24 = v23;
    v15 = v19;
    if ((char)a0[0x48] == (char)a0[0x94]) goto label_c480;
  }
  else {
label_c480:
    v16 = v19;
    v24 = v23;
    v15 = &v5[v12 - v18];
    if (v12 < v18)
      v15 = v5;
    while (v4 = v15, v16 != v15) {
      v1 = &v16[-1];
      v2 = &v24[-1];
      v4 = v16;
      if (v16[-1] != v24[-1]) break;
      v16 = v1;
      v24 = v2;
    }
    if ((v20 != v16) && (v16[-1] != 10))
      v7 = dat_26560 + 1;
    else {
      v7 = dat_26560;
      if (v22 != v24)
        v7 = dat_26560 + (unsigned long)(v24[-1] != 10);
    }
    v10 = v7 + -1;
    v15 = v16;
    if ((v7) && (v16 != v19)) {
      do {
        do {
          v15 = &v16[1];
          v3 = *v16;
          v16 = v15;
        } while (v3 != 10);
        v21 = v10 != 0;
        v10 -= 1;
      } while (v15 != v19 && v21);
    }
    v24 = &v24[(long)v15 - (long)v4];
  }
  *(unsigned char **)&a0[0x3c] = v15;
  *(unsigned char **)&a0[0x88] = v24;
  if ((((dat_26578) && (!dat_26520)) && ((long)dat_26570 <= 0x1ffffffffffffffe)) && (dat_26570 < v12)) {
    v7 = sub_b4c0(0,0,(long)v15 - (long)v5);
    v18 = 0;
    v12 = sub_b4c0(0,0,(long)v19 - (long)v15);
    v25 = 1;
    if (0 < (long)v6) {
      do {
        v25 *= 2;
      } while (v25 <= (long)v6);
      v18 = v25 - 1;
    }
    if ((long)v12 < (long)v6)
      v6 = v12;
    v6 = v7 + v25 + v6;
  }
  else {
    v18 = 0xffffffffffffffff;
    v6 = sub_b4c0(0,0,v12);
    v25 = 0;
  }
  v7 = sub_15f50(v6 * 8);
  v19 = *(unsigned char **)&a0[0x38];
  v5 = v20;
  if ((dat_26578) && (v19 == v15)) {
    v10 = 0;
    v13 = 0;
    v15 = v24;
    if (*(unsigned char **)&a0[0x84] != v24) {
      v12 = 0;
      if (v20 != v19) goto label_c46f;
label_c3fb:
      v13 = 0;
      v10 = 0;
      v15 = *(unsigned char **)&a0[0x84];
    }
  }
  else {
    v12 = 0;
    if (v20 == v19) goto label_c3fb;
label_c46f:
    do {
      v13 = v12 + 1;
      v17 = v6;
      if ((v12 & v18) == v6) {
        if (0x7fffffffffffffe < (long)v6) {
          sub_16300(); // no-return, return-dupe
        }
        v17 = v6 * 2;
        v7 = sub_15fa0(v7,v6 << 4);
      }
      *(unsigned char **)(v7 + (v12 & v18) * 8) = v5;
      do {
        v15 = &v5[1];
        v3 = *v5;
        v5 = v15;
      } while (v3 != 10);
      v12 = v13;
      v6 = v17;
    } while (v15 != v19);
    v10 = (long)v15 - (long)v20;
    v15 = *(unsigned char **)&a0[0x84];
  }
  v12 = dat_26570;
  v17 = dat_26570;
  if ((long)v13 <= (long)dat_26570)
    v17 = v13;
  if (!v25)
    v17 = v13;
  v8 = sub_b4c0(v13,v10,(long)v24 - (long)v15);
  v9 = sub_b4c0(v13,v10,(long)v23 - (long)v24);
  if ((long)v12 < (long)v9)
    v9 = v12;
  v9 = v17 + v8 + v9;
  if (((long)v9 < (long)v17) || (0xffffffffffffffe < v9))
    sub_16300();
  v10 = sub_15f50(v9 * 8);
  if (v13 != v17) { // branch-flip
    if ((long)v17 <= 0) goto label_c7a2;
    v11 = v13 - dat_26570;
    v12 = v17 + v11;
    v8 = v11 * -8;
    do {
      *(unsigned long *)(v10 + v8 + v11 * 8) = *(unsigned long *)(v7 + (v18 & v11) * 8);
      v11 += 1;
    } while (v12 != v11);
    v18 = 0;
    do {
      *(unsigned long *)(v7 + v18 * 8) = *(unsigned long *)(v10 + v18 * 8);
      v18 += 1;
    } while (v17 != v18);
  }
  else if ((long)v13 <= 0) goto label_c7a2;
  v8 = 0;
  do {
    *(unsigned char **)(v10 + v8 * 8) = &v22[*(long *)(v7 + v8 * 8) - (long)v20];
    v8 += 1;
  } while (v8 < (long)v17);
label_c7a2:
  *(unsigned long *)&a0[0x86] = v13;
  *(unsigned long *)&a0[0x82] = v9 - v17;
  *(unsigned long *)&a0[0x7a] = v17 * 8 + v10;
  *(unsigned long *)&a0[0x2e] = v7 + v17 * 8;
  *(unsigned long *)&a0[0x7c] = -v17;
  *(unsigned long *)&a0[0x30] = -v17;
  *(unsigned long *)&a0[0x36] = v6 - v17;
  *(unsigned long *)&a0[0x3a] = v13;
  return;
}


// Function: sub_c8f0 @ 0xc8f0
unsigned int sub_c8f0(int *a0,unsigned int a1)
{
  int v1; // ecx
  long v2;
  char v3; // r12b
  unsigned int v4;
  unsigned int v5; // r13d
  
  v5 = (unsigned int)dat_26568;
  v4 = a1 | sub_bfd0(a0,(v5 | a1) & 0xff);
  if (*a0 != a0[0x4c]) { // branch-flip
    v4 |= sub_bfd0(&a0[0x4c],(v5 | a1 | v4) & 0xff);
    v3 = (char)v4;
  }
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
    v3 = (char)v4;
  }
  if (v3)
    return v4;
  sub_c270(a0);
  dat_26700 = *(long *)&a0[0x82] + *(long *)&a0[0x36] + 1;
  if (0x3ffffffffffffff <= dat_26700)
    sub_16300(); // no-return, return-dupe
  dat_26710 = (void *)sub_15f50(dat_26700 * 0x20);
  dat_26708 = 1;
  if (0x201 <= (unsigned long)((long)dat_26700 / 3)) { // branch-flip
    v1 = 9;
    do {
      v1 += 1;
      dat_26718 = 1L << ((unsigned char)v1 & 0x3f);
    } while (dat_26718 < (unsigned long)((long)dat_26700 / 3));
    dat_26718 -= *(unsigned char *)((long)v1 + 0x1d1a0);
    if (0xffffffffffffffe < dat_26718) {
      sub_16300();
    }
    v2 = dat_26718 * 8 + 8;
  }
  else {
    dat_26718 = 0x1fd;
    v2 = 0xff0;
  }
  dat_26720 = sub_f5e0(v2) + 8;
  sub_b600(a0);
  sub_b600(&a0[0x4c]);
  *(unsigned long *)&a0[0x96] = dat_26708;
  *(unsigned long *)&a0[0x4a] = dat_26708;
  free(dat_26710);
  free((void *)(dat_26720 + -8));
  return v4;
}


// Function: sub_cac0 @ 0xcac0
void sub_cac0(unsigned long a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2;
  long v3; // stack - 0x40
  long v4; // stack - 0x48
  long v5; // stack - 0x50
  long v6; // stack - 0x38
  long v7;
  long v8; // rsi
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  sub_eb70(4);
  sub_f0f0(0x2c,0x261a0,v5,v4);
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x1d550),dat_26188);
  sub_f0f0(0x2c,0x262d0,v3,v6);
  sub_eb70(3);
  fputc_unlocked(10,dat_26188);
  if ((v1 & 1) && (v5 <= v4)) {
    v2 = v5 * 8;
    v7 = v5;
    do {
      while( true ) {
        sub_eb70(2);
        v8 = dat_26258 + v2;
        v2 += 8;
        sub_ef20("<",v8,1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26258 + v2) + -1) != '\n') break;
        v7 += 1;
        putc_unlocked(10,dat_26188);
        if (v4 < v7) goto label_cc30;
      }
      v7 += 1;
    } while (v7 <= v4);
  }
label_cc30:
  if (v1 != 3) { // branch-flip
    if (!(v1 & 2))
      return;
  }
  else {
    fputs_unlocked("---\n",dat_26188);
  }
  if (v6 < v3)
    return;
  v2 = v3 * 8;
  v7 = v3;
  do {
    sub_eb70(1);
    v8 = dat_26388 + v2;
    v2 += 8;
    sub_ef20(">",v8,1);
    sub_eb70(3);
    if (*(char *)(*(long *)(dat_26388 + v2) + -1) == '\n')
      putc_unlocked(10,dat_26188);
    v7 += 1;
  } while (v7 <= v6);
}


// Function: sub_ccf0 @ 0xccf0
void sub_ccf0(unsigned long a0)
{
  sub_eb10(a0,sub_eaf0,sub_cac0); // tail-call
}


// Function: sub_cd10 @ 0xcd10
unsigned long sub_cd10(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  void *v2;
  unsigned long v3;
  unsigned long v4;
  
  v3 = dat_26498;
  v2 = dat_26188;
  if (!dat_264a0) {
    v4 = (dat_26498 + a0) - a0 % dat_26498;
    while (v1 = v4, v1 <= a1) {
      putc_unlocked(9,v2);
      v4 = v1 + v3;
      a0 = v1;
    }
  }
  if (a0 < a1) {
    do {
      a0 += 1;
      putc_unlocked(0x20,v2);
    } while (a1 != a0);
  }
  return a1;
}


// Function: sub_cda0 @ 0xcda0
unsigned long sub_cda0(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  char v1;
  unsigned long v10;
  int v11; // eax
  unsigned long v12;
  char *v2;
  void *v3;
  unsigned long v4;
  long v5; // rcx
  char *v6;
  char *v7;
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x4c
  
  v3 = dat_26188;
  v12 = 0;
  v2 = (char *)a0[1];
  v8 = 0;
  if ((char *)*a0 < v2) {
    v6 = (char *)*a0;
    v10 = 0;
    do {
      while( true ) {
        v1 = *v6;
        v11 = (int)v1;
        v7 = &v6[1];
        if ('_' < v1) break;
        if ('\b' <= v1) { // branch-flip
          switch(v11 - 8U & 0xff) {
            case 0:
              if (v10) {
                v10 -= 1;
                if (v10 < a2) {
                  if (v12 <= v10) { // branch-flip
                    if (v12 < v10) {
                      do {
                        v12 += 1;
                        putc_unlocked(0x20,v3);
                      } while (v10 != v12);
                    }
                  }
                  else {
                    putc_unlocked(8,v3);
                    v12 = v10;
                  }
                }
              }
              break;
            case 1:
              v5 = dat_26498 - v10 % dat_26498;
              if (v12 == v10) {
                v4 = v5 + v12;
                if (dat_264a0) { // branch-flip
                  if (a2 <= v4)
                    v4 = a2;
                  if (v12 < v4) {
                    do {
                      v12 += 1;
                      putc_unlocked(0x20,v3);
                    } while (v12 != v4);
                  }
                }
                else if (v4 < a2) {
                  putc_unlocked(9,v3);
                  v12 = v4;
                }
              }
              v10 += v5;
              break;
            case 2:
              return v12;
            case 3:
            case 4:
              goto label_ceb3;
            case 5:
              v10 = 0;
              putc_unlocked(0xd,v3);
              sub_cd10(0,a1);
              v12 = 0;
              break;
            default:
              goto label_ce80;
            case 0x18:
            case 0x19:
            case 0x1a:
            case 0x1b:
            case 0x1d:
            case 0x1e:
            case 0x1f:
            case 0x20:
            case 0x21:
            case 0x22:
            case 0x23:
            case 0x24:
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
            case 0x39:
            case 0x3a:
            case 0x3b:
            case 0x3c:
            case 0x3d:
            case 0x3e:
            case 0x3f:
            case 0x40:
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
              goto label_ce2f;
            
          }
        }
        else {
label_ce80:
          v4 = sub_16640(&v9,v6,(long)v2 - (long)v6,&v8);
          if (0xfffffffffffffffd <= v4 - 1) { // branch-flip
label_ceb3:
            if (v10 < a2)
              putc_unlocked(v11,v3);
          }
          else {
            v11 = wcwidth(v9);
            if (1 <= v11)
              v10 = (long)v11 + v10;
            if (v10 <= a2) {
              fwrite_unlocked(v6,1,v4,stdout);
              v12 = v10;
            }
            v7 = &v6[v4];
          }
        }
label_cec0:
        v6 = v7;
        if (v2 <= v7)
          return v12;
      }
      if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_ce80;
label_ce2f:
      v4 = v10 + 1;
      if (v10 < a2) {
        putc_unlocked(v11,v3);
        v10 = v4;
        v12 = v4;
        goto label_cec0;
      }
      v6 = v7;
      v10 = v4;
    } while (v7 < v2);
  }
  return v12;
}


// Function: sub_d0a0 @ 0xd0a0
void sub_d0a0(long a0,char a1,unsigned long *a2)
{
  bool v1;
  bool v2;
  void *v3;
  long v4;
  long v5;
  bool v6;
  long v7;
  int v8; // edi
  bool v9;
  
  v5 = dat_26430;
  v4 = dat_26428;
  v3 = dat_26188;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_eb70(1);
      goto label_d202;
    }
    v1 = 0;
    if (a0) goto label_d0ec;
    v9 = 0;
    v7 = 0;
    v6 = 0;
    v2 = v1;
label_d10b:
    v1 = v6;
    if (a1 == ' ') goto label_d111;
    v7 = sub_cd10(v7,(unsigned long)(v5 + -1 + v4) >> 1) + 1;
    v1 = v2;
    if (a1 != '|') goto label_d224;
    v8 = 0x7c;
    if ((*(char *)(a2[1] + -1) == '\n') != v9)
      v8 = (-(unsigned int)(v9 == 0) & 0x2d) + 0x2f;
  }
  else {
    sub_eb70(2);
label_d202:
    v1 = 1;
    if (a0) {
label_d0ec:
      v9 = *(char *)(*(long *)(a0 + 8) + -1) == '\n';
      v7 = sub_cda0(a0,0,v5);
      v6 = v1;
      v2 = v1;
      goto label_d10b;
    }
    v9 = 0;
    v7 = sub_cd10(0,(unsigned long)(v5 + -1 + v4) >> 1) + 1;
label_d224:
    v8 = (int)a1;
  }
  putc_unlocked(v8,v3);
label_d111:
  if ((a2) && (v9 = (bool)(v9 | *(char *)(a2[1] + -1) == '\n'), *(char *)*a2 != '\n'))
    sub_cda0(a2,sub_cd10(v7,v4),v5);
  if (v9)
    putc_unlocked(10,v3);
  if (v1) {
    sub_eb70(3); // tail-call
    return;
  }
}


// Function: sub_d260 @ 0xd260
void sub_d260(long a0,long a1) // return-dupe x3
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
  if (dat_26439) goto label_d37b;
  if ((v4) && (v5)) {
    while( true ) {
      v2 += 1;
      v3 += 1;
      sub_d0a0(dat_26258 + -8 + v3 * 8,0x20,dat_26388 + -8 + v2 * 8);
      if (a0 == v3) break;
      if (a1 == v2) goto label_d37b;
    }
  }
  if (a1 == v2) goto label_d37b;
  do {
    v1 = v2 * 8;
    v2 += 1;
    sub_d0a0(0,0x29,dat_26388 + v1);
  } while (a1 != v2);
  if (a0 == v3) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  do {
    v2 = v3 * 8;
    v3 += 1;
    sub_d0a0(dat_26258 + v2,0x28,0);
label_d37b:
  } while (a0 != v3);
  dat_26728 = a1;
  dat_26730 = a0;
}


// Function: sub_d3c0 @ 0xd3c0
void sub_d3c0(unsigned long a0) // return-dupe x2, return-dupe
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
  
  v1 = sub_f1a0(a0,&v7,&v6,&v5,&v8);
  if (!v1)
    return;
  sub_d260(v7,v5);
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
        if (v9 > v8) goto label_d52f;
        v3 = v9 * 8;
        v9 += 1;
        v2 = v4 * 8;
        v4 += 1;
        sub_d0a0(dat_26258 + v2,0x7c,dat_26388 + v3);
      } while (v6 >= v4);
      v3 = v8;
      v2 = v6;
      v7 = v4;
      v5 = v9;
    }
label_d52f:
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
        sub_d0a0(0,0x3e,dat_26388 + v2 * 8);
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
      sub_d0a0(dat_26258 + v2,0x3c,0);
    } while (v3 <= v6);
    dat_26730 = v3;
    return;
  }
  dat_26730 = v7;
}


// Function: sub_d5b0 @ 0xd5b0
void sub_d5b0(unsigned long a0)
{
  sub_e2b0();
  dat_26728 = -dat_26288;
  dat_26730 = dat_26728;
  sub_eb10(a0,sub_eaf0,sub_d3c0);
  sub_d260(dat_26270,dat_263a0); // tail-call
}


// Function: sub_d600 @ 0xd600
void sub_d600(unsigned int a0) // return-dupe
{
  if (dat_26774)
    return;
  dat_26774 = a0;
}


// Function: sub_d620 @ 0xd620
void sub_d620(void) // return-dupe
{
  if (dat_26774)
    return;
  dat_26770 += 1;
}


// Function: sub_d640 @ 0xd640
unsigned int sub_d640(long *a0,long *a1,unsigned int a2,long *a3)
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
  if (v5 == '\\') goto label_d6a6;
label_d67a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_d6de:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_d6de;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_d696;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_d696;
    }
    v2 = 0;
    goto label_d6de;
  }
  v3 = &v3[1];
  v8 = v7;
label_d68b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_d696:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_d67a;
label_d6a6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_d6de;
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
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_d68b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_d696;
      default:
        goto label_d720;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_d7b7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_d720;
      case 0x32:
        v6 = '\b';
        goto label_d720;
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
label_d720:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_d7b7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_d802;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_d68b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_d802:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_d68b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}


// Function: sub_d8b0 @ 0xd8b0
int sub_d8b0(int a0)
{
  char v1; // dil
  
  v1 = (char)a0;
  if ('#' <= v1) { // branch-flip
    if (v1 == '\\')
      return a0;
  }
  else if ('\a' <= v1) {
    switch(a0 - 7U & 0xff) {
      case 0:
        a0 = 0x61;
        break;
      case 1:
        a0 = 0x62;
        break;
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
      default:
        goto label_d8f0;
      case 0x1b:
        break;
      
    }
    return a0;
  }
label_d8f0:
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),v1 <= '\x1f');
}


// Function: sub_d970 @ 0xd970
unsigned char * sub_d970(unsigned char *a0)
{
  long v1;
  int v10; // edi
  char v11; // r9b
  unsigned long v12; // r9
  char v2; // al
  unsigned char v3;
  unsigned char *v4;
  unsigned char v5; // cl
  long v6;
  unsigned long v7; // rcx
  unsigned char *v8;
  unsigned char *v9; // rsi
  
  v10 = (int)(char)*a0;
  if (*a0) {
    v12 = 0;
    v6 = 0;
    v8 = a0;
    do {
      while ((char)v10 != ' ') {
        v2 = sub_d8b0(v10);
        v11 = (char)v12;
        if (v2) {
          v1 = v6 + 3;
          v6 += 1;
          if (v2 == '\x01')
            v6 = v1;
        }
        v4 = &v8[1];
        v10 = (int)(char)*v4;
        v8 = &v8[1];
        if (!*v4) goto label_d9cd;
      }
      v4 = &v8[1];
      v10 = (int)(char)*v4;
      v8 = &v8[1];
      v12 = 1;
      v11 = '\x01';
    } while (*v4);
label_d9cd:
    if ((v6) || (v11)) {
      v4 = (unsigned char *)sub_15f50(&v8[(v6 + 3) - (long)a0]);
      *v4 = 0x22;
      v3 = *a0;
      v8 = &v4[1];
      while (v3) {
        v7 = (unsigned long)v3;
        v3 = sub_d8b0((int)(char)v3);
        v5 = (unsigned char)v7;
        if (v3) { // branch-flip
          if (v3 != 1) { // branch-flip
            *v8 = 0x5c;
            v9 = &v8[2];
            v8[1] = v3;
          }
          else {
            *v8 = 0x5c;
            v9 = &v8[4];
            v8[1] = ((unsigned char)((v7 & 0xffffffff) >> 6) & 3) + 0x30;
            v8[3] = (v5 & 7) + 0x30;
            v8[2] = ((char)v5 >> 3 & 7U) + 0x30;
          }
        }
        else {
          *v8 = v5;
          v9 = &v8[1];
        }
        v8 = &a0[1];
        a0 = &a0[1];
        v3 = *v8;
        v8 = v9;
      }
      v8[0] = 0x22;
      v8[1] = 0;
      return v4;
    }
  }
  return a0;
}


// Function: sub_da80 @ 0xda80
void sub_da80(void)
{
  char v1;
  void *v2;
  void *v3;
  long v4; // rcx
  unsigned short v5; // stack - 0x43
  char *v6; // stack - 0x58
  void *v7; // stack - 0x50
  char *v8;
  char v9; // stack - 0x41
  
  v8 = dat_26748;
  v6 = dat_26748;
  if ((!dat_26748) || (!*dat_26748))
    return;
  v9 = 0;
  v5 = 0x3f3f;
  v2 = (void *)sub_162e0(dat_26748);
  v7 = v2;
label_db0d:
  do {
    v1 = *v8;
    if (v1 != '*') { // branch-flip
      if (v1 == ':') {
        v8 = &v8[1];
        v6 = v8;
        goto label_db0d;
      }
      if (!v1)
        return;
      v5 = CONCAT11(v5._1_1_,v1);
      v6 = &v8[1];
      if (!v8[1]) goto label_dbda;
      v5 = CONCAT11(v8[1],v1);
      v6 = &v8[3];
      if (v8[2] != '=') goto label_dbda;
      v4 = 0;
      v8 = "lc";
      while (strcmp((char *)&v5,v8)) {
        v4 += 1;
        v8 = *(char **)(v4 * 8 + 0x25900);
        if (!v8) goto label_dbb6;
      }
      v4 = (long)(int)v4 * 0x10;
      *(void **)(v4 + 0x26028) = v7;
      if (!sub_d640(&v7,&v6,0,v4 + 0x26020)) {
label_dbb6:
        error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),&v5);
label_dbda:
        error(0,0,dcgettext(NULL,"unparsable value for --palette",5));
        free(v2);
        v2 = dat_26750;
        while (v2) {
          v3 = *(void **)((long)v2 + 0x20);
          free(v2);
          v2 = v3;
        }
        dat_26758 = 0;
        return;
      }
    }
    else {
      v3 = (void *)sub_15f50(0x28);
      *(void **)((long)v3 + 0x20) = dat_26750;
      *(void **)((long)v3 + 8) = v7;
      dat_26750 = v3;
      v6 = &v8[1];
      if ((!sub_d640(&v7,&v6,1,v3)) || (v8 = &v6[1], v1 = *v6, v6 = v8, v1 != '=')) goto label_dbda;
      *(void **)((long)v3 + 0x18) = v7;
      if (!sub_d640(&v7,&v6,0,(long)v3 + 0x10)) goto label_dbda;
    }
    v8 = v6;
  } while( true );
}


// Function: sub_dcf0 @ 0xdcf0
void sub_dcf0(void)
{
  int *v1; // rbx
  unsigned long v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  unsigned long v12; // stack - 0x98
  unsigned long v13; // stack - 0x90
  unsigned long v14; // stack - 0x88
  unsigned long v15; // stack - 0x80
  unsigned long v16; // stack - 0x78
  unsigned long v17; // stack - 0x70
  unsigned long v18; // stack - 0x68
  unsigned long v19; // stack - 0x60
  void *v2; // stack - 0xd8
  unsigned long v20; // stack - 0x58
  unsigned int v21; // stack - 0x50
  int *v3; // rbp
  int v4;
  unsigned long v5; // stack - 0xd0
  unsigned long v6; // stack - 0xc8
  unsigned long v7; // stack - 0xc0
  unsigned long v8; // stack - 0xb8
  unsigned long v9; // stack - 0xb0
  
  v4 = 0x14;
  v1 = (int *)0x1d524;
  v3 = (int *)0x1d524;
  sigemptyset((void *)0x26780);
  while( true ) {
    sigaction(v4,NULL,&v2);
    if (v2 != (void *)0x1)
      sigaddset((void *)0x26780,v4);
    if (v3 == (int *)0x1d550) break;
    v4 = *v3;
    v3 = &v3[1];
  }
  v21 = 0x10000000;
  v4 = 0x14;
  v5 = dat_26780;
  v6 = dat_26788;
  v7 = dat_26790;
  v8 = dat_26798;
  v9 = dat_267a0;
  v10 = dat_267a8;
  v11 = dat_267b0;
  v12 = dat_267b8;
  v13 = dat_267c0;
  v14 = dat_267c8;
  v15 = dat_267d0;
  v16 = dat_267d8;
  v17 = dat_267e0;
  v18 = dat_267e8;
  v19 = dat_267f0;
  v20 = dat_267f8;
  while( true ) {
    if (sigismember(0x26780,v4)) {
      v2 = sub_d600;
      if (v4 == 0x14)
        v2 = sub_d620;
      sigaction(v4,&v2,NULL);
    }
    if (v1 == (int *)0x1d550) break;
    v4 = *v1;
    v1 = &v1[1];
  }
}


// Function: sub_de60 @ 0xde60
void sub_de60(char a0)
{
  int v1; // eax
  bool v2;
  
  if ((!dat_26188) || (!dat_2657c))
    return;
  if (dat_26180) // branch-flip
    v2 = 1;
  else {
    if (a0) {
      if (dat_2657c != 2) {
        dat_26758 = 0;
        return;
      }
      dat_26758 = 1;
      sub_da80(); // tail-call
      return;
    }
    v1 = fileno(dat_26188);
    v2 = isatty(v1) != 0;
  }
  if ((dat_2657c == 2) || ((dat_2657c == 1 && (v2)))) {
    dat_26758 = 1;
    sub_da80();
  }
  else {
    dat_26758 = 0;
  }
  if (!v2)
    return;
  sub_dcf0(); // tail-call
}


// Function: sub_df30 @ 0xdf30
void sub_df30(unsigned long a0)
{
  error(0,*__errno_location(),"%s",a0); // tail-call
}


// Function: sub_df60 @ 0xdf60
void sub_df60(char *a0,long a1,unsigned long a2,char *a3,char *a4)
{
  unsigned long v1;
  char *v10;
  long v11;
  char *v12; // stack - 0x70
  char *v13; // stack - 0x68
  unsigned long *v2;
  long v3; // rax
  unsigned long *v4; // rax
  void *v5; // rax
  unsigned long *v6;
  long v7;
  unsigned long v8 [5];
  long v9 [2]; // stack - 0x80
  
  if (dat_26480) {
    v11 = 8;
    if (!a3)
      a3 = "";
    if (!a4)
      a4 = "";
    v7 = 0;
    v10 = a0;
    v9[0] = a1;
    v9[1] = a2;
    v12 = a3;
    v13 = a4;
    while( true ) {
      v3 = strlen(v10) + 1;
      *(long *)((long)v8 + v7) = v3;
      v11 += v3;
      if (v7 + 8 == 0x28) break;
      v10 = *(char **)((long)v9 + v7);
      v7 += 8;
    }
    v4 = (unsigned long *)sub_15f50(v11);
    v6 = &v4[1];
    v11 = 0;
    while( true ) {
      v1 = *(unsigned long *)((long)v8 + v11);
      v5 = memcpy(v6,a0,v1);
      v2 = dat_260a0;
      v6 = (unsigned long *)((long)v5 + v1);
      if (v11 + 8 == 0x28) break;
      a0 = *(char **)((long)v9 + v11);
      v11 += 8;
    }
    dat_260a0 = v4;
    *v2 = v4;
    *v4 = 0;
    return;
  }
  if (dat_2643a)
    putchar_unlocked(0x20);
  __printf_chk(1,dcgettext(NULL,a0,5),a1,a2,a3,a4); // tail-call
}


// Function: sub_e0e0 @ 0xe0e0
void sub_e0e0(void)
{
  sub_df60(); // tail-call
}


// Function: sub_e0f0 @ 0xe0f0
void sub_e0f0(void) // return-dupe
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


// Function: sub_e1f0 @ 0xe1f0
void sub_e1f0(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_e0f0();
  error(2,v1,"%s",a0);
}


// Function: sub_e230 @ 0xe230
void sub_e230(char *a0)
{
  sub_e0f0();
  error(2,0,"%s",dcgettext(NULL,a0,5));
}


// Function: sub_e270 @ 0xe270
void sub_e270(unsigned long a0)
{
  dat_26748 = a0;
}


// Function: sub_e280 @ 0xe280
void sub_e280(unsigned long a0,unsigned long a1,char a2)
{
  dat_26768 = a0;
  dat_26760 = a1;
  dat_26759 = a2;
  dat_26188 = 0;
}


// Function: sub_e2b0 @ 0xe2b0
void sub_e2b0(void) // return-dupe x2
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
  v3 = (void *)sub_d970(dat_26768);
  v7 = (void *)sub_d970(dat_26760);
  v2 = (void *)sub_16460("diff%s %s %s",dat_26420,v3,v7);
  if (dat_26480) { // branch-flip
    v1 = fflush_unlocked(stdout);
    if (v1)
      sub_e1f0(dcgettext(NULL,"write failed",5)); // no-return
    v9 = v2;
    v5 = "/usr/bin/pr";
    v8 = "-h";
    v10 = 0;
    if (pipe(&v4))
      sub_e1f0("pipe"); // no-return
    dat_26740 = fork();
    if (dat_26740 < 0)
      sub_e1f0("fork"); // no-return
    if (!dat_26740) {
      close(v6);
      if (v4) {
        v1 = dup2(v4,0);
        if (v1 < 0)
          sub_e1f0("dup2"); // no-return
        close(v4);
      }
      execv("/usr/bin/pr",&v5);
      _exit((*__errno_location() == 2) + 0x7e); // no-return
    }
    close(v4);
    dat_26188 = fdopen(v6,"w");
    if (!dat_26188)
      sub_e1f0("fdopen"); // no-return
    sub_de60(1);
  }
  else {
    dat_26188 = stdout;
    sub_de60(0);
    if (dat_26759)
      __printf_chk(1,"%s\n",v2);
  }
  free(v2);
  if (dat_26580 != 2) { // branch-flip
    if (dat_26580 == 3)
      sub_87e0(0x261a0,&v3,1);
  }
  else {
    sub_87e0(0x261a0,&v3,0);
  }
  if (v3 != dat_26768)
    free(v3);
  if (v7 == dat_26760)
    return;
  free(v7);
}


// Function: sub_e550 @ 0xe550
void sub_e550(void)
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
  v1 = ferror_unlocked(dat_26188);
  if (v1)
    sub_e230("write failed"); // no-return
  v1 = fclose(dat_26188);
  if (!v1) {
    v1 = waitpid(dat_26740,(int *)&v4,0);
    if (v1 < 0)
      sub_e1f0("waitpid"); // no-return
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
  sub_e1f0(dcgettext(NULL,"write failed",5)); // no-return
}


// Function: sub_e690 @ 0xe690
unsigned long sub_e690(unsigned char *a0,unsigned char *a1)
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
label_e6e0:
  do {
    v8 = *v18;
    v10 = *v16;
    v12 = (unsigned long)v10;
    v14 = &v18[1];
    v13 = &v16[1];
    if (v8 == v10) goto label_e7ca;
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
                if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v2 * 2) & 0x20)) goto label_e798;
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
          else if (v5 != 2) goto label_e880;
        }
      }
      else {
label_e880:
        if ((v8 == 0x20) && (v10 == 9)) {
label_e8a1:
          v15 = v19;
          do {
            if (v8 != 0x20) { // branch-flip
              if (v8 != 9) goto label_e8fc;
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
          goto label_e8a1;
        }
      }
      goto label_e798;
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
      if (v17) goto label_ea1f;
label_e9f5:
      if ((unsigned char)v12 == v8) goto label_e798;
    }
    else if (v17) {
label_ea1f:
      while (v8 = (unsigned char)v15, (char)v12 != '\n') {
        v12 = (unsigned long)*v13;
        v16 = &v13[1];
        if (!(*(unsigned char *)(v7 + 1 + v12 * 2) & 0x20)) {
          v12 = 0x20;
          goto label_e9f5;
        }
        v13 = v16;
      }
      goto label_e798;
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
      if (*(unsigned char *)(v7 + 1 + (unsigned long)v13[-2] * 2) & 0x20) goto label_e6e0;
    }
  }
label_e798:
  if (v4) {
    v7 = *(long *)__ctype_tolower_loc();
    v8 = *(unsigned char *)(v7 + (unsigned long)v8 * 4);
    v12 = (unsigned long)*(unsigned char *)(v7 + v12 * 4);
  }
  v10 = (unsigned char)v12;
  if (v8 != v10)
    return 1;
label_e7ca:
  if (v10 == 10)
    return 0;
  v7 = 1;
  if (v10 == 9)
    v7 = v3 - v19 % v3;
  v19 += v7;
  v16 = v13;
  v18 = v14;
  goto label_e6e0;
label_e8fc:
  if ((char)v12 != ' ') { // branch-flip
    if ((char)v12 != '\t') goto label_ea30;
    v19 = (v3 + v19) - v19 % v3;
  }
  else {
    v19 += 1;
  }
  v12 = (unsigned long)*v13;
  v13 = &v13[1];
  goto label_e8fc;
label_ea30:
  if (v15 != v19)
    return 1;
  goto label_e798;
}


// Function: sub_eaf0 @ 0xeaf0
unsigned long sub_eaf0(unsigned long a0)
{
  return a0;
}


// Function: sub_eb10 @ 0xeb10
void sub_eb10(long a0,void *a1,void *a2)
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


// Function: sub_eb70 @ 0xeb70
void sub_eb70(int a0)
{
  if (a0 != 3)
    sub_ec70();
  if ((dat_26758) && (dat_26010 != a0)) {
    sub_df10(dat_26020,dat_26028);
    switch(a0) {
      case 0:
        sub_df10(dat_26060,dat_26068);
        break;
      case 1:
        sub_df10(dat_26070,dat_26078);
        break;
      case 2:
        sub_df10(dat_26080,dat_26088);
        break;
      case 3:
        sub_df10(dat_26050,dat_26058);
        break;
      case 4:
        sub_df10(dat_26090,dat_26098);
        break;
      default:
        abort(); // no-return
      
    }
    sub_df10(dat_26030,dat_26038);
    dat_26010 = a0;
    return;
  }
}


// Function: sub_ec70 @ 0xec70
void sub_ec70(void)
{
  char v1 [136];
  int v2;
  
  while ((dat_26774 || (dat_26770))) {
    sub_eb70(3);
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


// Function: sub_ed50 @ 0xed50
void sub_ed50(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char v1;
  void *v2;
  unsigned long v3; // rax
  long v4; // rcx
  unsigned long v5;
  long v6; // r14
  unsigned long v7;
  
  v5 = dat_26498;
  v2 = dat_26188;
  if (!dat_264a0) {
    v5 = (long)a1 - (long)a0;
    dat_264a0 = 0;
    while( true ) {
      if (!v5)
        return;
      v7 = 0x400;
      if (v5 <= 0x400)
        v7 = v5;
      v3 = fwrite_unlocked(a0,1,v7,dat_26188);
      if (v3 < v7) break;
      v5 -= v3;
      a0 = &a0[v3];
      sub_ec70();
    }
    return;
  }
  v7 = 0;
  v6 = 1;
  if (a1 <= a0)
    return;
  do {
    v1 = *a0;
    a0 = &a0[1];
    if (v1 != 9) { // branch-flip
      if (v1 != 0xd) {
        if (v1 != 8) { // branch-flip
          v7 += *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v1 * 2) >> 0xe & 1;
          putc_unlocked((unsigned int)v1,v2);
        }
        else {
          if (!v7) goto label_ef10;
          v7 -= 1;
          putc_unlocked(8,v2);
        }
        goto label_ee4f;
      }
      putc_unlocked(0xd,v2);
      if ((!a2) || (a1 <= a0)) {
label_ef10:
        v7 = 0;
        goto label_ee4f;
      }
      v7 = 0;
      if (*a0 != 10)
        __fprintf_chk(v2,1,a2,a3);
    }
    else {
      v4 = v5 - v7 % v5;
      v7 += v4;
      do {
        putc_unlocked(0x20,v2);
        v4 -= 1;
      } while (v4);
label_ee4f:
      if (a1 == a0)
        return;
    }
    v6 += 1;
    if (v6 == 0x400) {
      sub_ec70();
      v6 = 0;
    }
  } while( true );
}


// Function: sub_ef20 @ 0xef20
void sub_ef20(char *a0,unsigned long *a1,char a2)
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
      sub_ed50(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),v7,a0);
      v1 = *a0;
    }
    else {
      sub_ed50(v2,v3,v7,a0);
      v1 = *a0;
    }
    if (!v1)
      return;
  }
  else if (a2) // branch-flip
    sub_ed50(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),0,0);
  else {
    sub_ed50(v2,v3,0,0);
  }
  if (*(char *)(v3 + -1) == '\n')
    return;
  sub_eb70(3);
  __fprintf_chk(v4,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5)); // tail-call
}


// Function: sub_f0a0 @ 0xf0a0
void sub_f0a0(unsigned long a0,unsigned long a1)
{
  sub_ef20(a0,a1,0); // tail-call
}


// Function: sub_f0b0 @ 0xf0b0
long sub_f0b0(long a0,long a1)
{
  return a1 + *(long *)(a0 + 0xe8) + 1;
}


// Function: sub_f0c0 @ 0xf0c0
void sub_f0c0(unsigned long a0,long a1,unsigned long a2,long *a3,long *a4)
{
  long v1; // rax
  long v2; // rdx
  
  sub_f0b0(a0,a1 + -1);
  *a3 = v1 + 1;
  *a4 = sub_f0b0(a0,v2 + 1) + -1;
}


// Function: sub_f0f0 @ 0xf0f0
void sub_f0f0(char a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_f0c0(a1,a2,a3,&v2,&v1);
  if (v2 < v1) {
    __fprintf_chk(dat_26188,1,"%ld%c%ld",v2,(unsigned long)(unsigned int)(int)a0); // tail-call
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld",v1); // tail-call
}


// Function: sub_f1a0 @ 0xf1a0
unsigned char sub_f1a0(long *a0,long *a1,long *a2,long *a3,long *a4)
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
              goto label_f2e2;
            }
            v3 = v10[1];
            v10 = &v10[1];
          } while (v3 != 10);
          v13 = v15 - (long)v10;
        }
label_f2e2:
        if ((long)v17 != v13) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            goto label_f2fb;
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
              goto label_f465;
            }
            v11 = v10[1];
            v10 = &v10[1];
          } while (v11 != 10);
          v13 = v12 - (long)v10;
        }
label_f465:
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
label_f2fb:
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


// Function: sub_f560 @ 0xf560
undefined16 sub_f560(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [16];
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3._0_8_ = (char *)sub_15f50(v1 + v2 + 1 + strlen(a2));
  __sprintf_chk(v3._0_8_,1,0xffffffffffffffff,"%s%s%s",a0,a1,a2);
  v3._8_8_ = 0xf59a;
  return v3._0_16_;
}


// Function: sub_f5e0 @ 0xf5e0
void sub_f5e0(unsigned long a0)
{
  memset((void *)sub_15f50(),0,a0);
}


// Function: sub_f600 @ 0xf600
void sub_f600(unsigned long *a0)
{
  fflush_unlocked(stdout);
  for (; a0; a0 = (unsigned long *)*a0) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",a0[3],a0[4],a0[2],a0[1]);
  }
  fflush_unlocked(stderr); // tail-call
}

