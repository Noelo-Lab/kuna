int main(int argc,char **argv,char **envp)
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
  int v4; // eax
  char *v5; // rax
  FILE *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned long v9; // rdx
  
  sub_4350(*argv);
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
  v13 = argv;
  if (((2 <= argc) && (v7 = argv[1], *v7 == '-')) && ((unsigned int)((int)v7[1] - 0x30U) <= 9)) {
    v15 = &v7[1];
    v13 = &argv[1];
    argc -= 1;
    *v13 = *argv;
  }
  v14 = NULL;
  while (v2 = getopt_long(argc,v13,"0123456789cstuw:p:g:",(option *)0xbaa0,NULL), v2 != -1) {
    if (0x77 < v2) {
label_2acd:
      if ((unsigned int)(v2 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),(char)v2);
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
        v7 = optarg;
        if (*optarg == ' ') {
          do {
            dat_17118 = (1U - (int)optarg) + (int)v7;
            v7 = &v7[1];
          } while (*v7 == ' ');
        }
        dat_17128 = v7;
        dat_1711c = (int)strlen(v7);
        v5 = &v7[dat_1711c];
        if (v7 < v5) {
          do {
            if (v5[-1] != ' ') break;
            v5 = &v5[-1];
          } while (v7 != v5);
        }
        *v5 = '\0';
        dat_17114 = (int)v5 - (int)v7;
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
  if (optind != argc) { // branch-flip
    v1 = 0;
    v12 = 1;
    v2 = optind;
    if (argc <= optind) {
      return (v12 ^ 1) & 0xff; // return-dupe
    }
    do {
      v7 = v13[v2];
      if (strcmp(v7,"-")) { // branch-flip
        v6 = fopen(v7,"r");
        if (v6) // branch-flip
          v12 &= sub_3700(v6);
        else {
          v8 = sub_6030(4,v7);
          v7 = dcgettext(NULL,"cannot open %s for reading",5);
          v12 = 0;
          error(0,*__errno_location(),v7,v8);
        }
      }
      else {
        v3 = sub_3700(stdin,v7);
        v1 = 1;
        v12 &= v3;
      }
      v2 = optind + 1;
      optind = v2;
    } while (v2 < argc);
    if (!v1) {
      return (v12 ^ 1) & 0xff;
    }
  }
  else {
    v12 = sub_3700(stdin,"-",v10);
  }
  v2 = sub_41f0(stdin);
  if (v2) {
    v7 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v7);
    return v4;
  }
  return (v12 ^ 1) & 0xff;
}
