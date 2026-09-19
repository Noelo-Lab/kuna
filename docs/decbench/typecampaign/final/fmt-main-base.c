unsigned long sub_26a0(int a0,char **a1)
{
  bool v1;
  unsigned long v10;
  unsigned long v11; // rdx
  unsigned int v12; // eax
  char **v13;
  char *v14; // stack - 0x48
  char *v15; // stack - 0x40
  int v2;
  unsigned int v3; // eax
  char *v4; // rax
  void *v5; // rax
  char *v6;
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rdx
  
  sub_4350(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7b90(sub_4100);
  dat_17130 = 0;
  dat_17131 = 0;
  dat_17132 = 0;
  dat_17133 = 0;
  dat_17120 = 0x4b;
  dat_17128 = "";
  dat_1711c = 0;
  dat_17118 = 0;
  dat_17114 = 0;
  v15 = NULL;
  v13 = a1;
  if (((2 <= a0) && (v6 = a1[1], *v6 == '-')) && ((unsigned int)((int)v6[1] - 0x30U) <= 9)) {
    v15 = &v6[1];
    v13 = &a1[1];
    a0 -= 1;
    *v13 = *a1;
  }
  v14 = NULL;
  while (v2 = getopt_long(a0,v13,"0123456789cstuw:p:g:",(void *)0xbaa0,NULL), v2 != -1) {
    if (0x77 < v2) {
label_2acd:
      if ((unsigned int)(v2 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),v2);
      sub_3d70(1); // no-return
    }
    if (v2 <= 0x62) {
      if (v2 == -0x83) {
        sub_6c80(stdout,"fmt","GNU coreutils",dat_c010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_3d70(0); // no-return
      goto label_2acd;
    }
    switch(v2) {
      case 99:
        dat_17133 = 1;
        break;
      default:
        goto label_2acd;
      case 0x67:
        v14 = optarg;
        break;
      case 0x70:
        dat_17118 = 0;
        v6 = optarg;
        if (*optarg == ' ') {
          do {
            dat_17118 = (1U - (int)optarg) + (int)v6;
            v6 = &v6[1];
          } while (*v6 == ' ');
        }
        dat_17128 = v6;
        dat_1711c = (int)strlen(v6);
        v4 = &v6[dat_1711c];
        if (v6 < v4) {
          do {
            if (v4[-1] != ' ') break;
            v4 = &v4[-1];
          } while (v6 != v4);
        }
        *v4 = '\0';
        dat_17114 = (int)v4 - (int)v6;
        break;
      case 0x73:
        dat_17131 = 1;
        break;
      case 0x74:
        dat_17132 = 1;
        break;
      case 0x75:
        dat_17130 = 1;
        break;
      case 0x77:
        v15 = optarg;
      
    }
  }
  if (v15) { // branch-flip
    v2 = sub_74a0(v15,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = v2;
    if (v14) { // branch-flip
      dat_17110 = sub_74a0(v14,0,(long)v2,"",dcgettext(NULL,"invalid width",5),0);
      v10 = v9;
    }
    else {
label_2a19:
      dat_17110 = (dat_17120 * 0xbb) / 200;
      v10 = (long)(dat_17120 * 0xbb) % 200 & 0xffffffff;
    }
  }
  else {
    if (!v14) goto label_2a19;
    dat_17110 = sub_74a0(v14,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = dat_17110 + 10;
    v10 = v11;
  }
  if (optind != a0) { // branch-flip
    v1 = 0;
    v12 = 1;
    v2 = optind;
    if (a0 <= optind) {
      return (unsigned long)((v12 ^ 1) & 0xff); // return-dupe
    }
    do {
      v6 = v13[v2];
      if (strcmp(v6,"-")) { // branch-flip
        v5 = fopen(v6,"r");
        if (v5) // branch-flip
          v12 &= sub_3700(v5);
        else {
          v8 = sub_6030(4,v6);
          v6 = dcgettext(NULL,"cannot open %s for reading",5);
          v12 = 0;
          error(0,*__errno_location(),v6,v8);
        }
      }
      else {
        v3 = sub_3700(stdin,v6);
        v1 = 1;
        v12 &= v3;
      }
      v2 = optind + 1;
      optind = v2;
    } while (v2 < a0);
    if (!v1) {
      return (unsigned long)((v12 ^ 1) & 0xff);
    }
  }
  else {
    v12 = sub_3700(stdin,"-",v10);
  }
  v2 = sub_41f0(stdin);
  if (v2) {
    v6 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v6);
    return v7;
  }
  return (unsigned long)((v12 ^ 1) & 0xff);
}