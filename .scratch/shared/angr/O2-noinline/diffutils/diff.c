// Function: main @ 0x4fa0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_41d5b0;
extern option g_425160;
extern long long g_4260a8;
extern unsigned int g_4260b0;
extern long long stdout;
extern unsigned int optind;
extern char optarg;
extern char g_426180;
extern unsigned long long g_426400;
extern char g_426408;
extern long long g_426410;
extern char g_426418;
extern unsigned long long g_426420;
extern unsigned long long g_426428;
extern unsigned long long g_426430;
extern char g_426438;
extern char g_426439;
extern char g_42643a;
extern struct_0 *g_426440;
extern unsigned long long g_426448;
extern unsigned long long g_426450;
extern struct_0 *g_426458;
extern void g_426460;
extern long long g_426468;
extern long long g_426470;
extern unsigned long long g_426478;
extern void g_426480;
extern long long g_426488;
extern char g_426490;
extern char g_426491;
extern char g_426492;
extern unsigned long long g_426498;
extern char g_4264a0;
extern char g_4264a1;
extern char g_4264c0;
extern char g_426500;
extern unsigned long long g_426540;
extern unsigned long long g_426548;
extern char g_426550;
extern char g_426551;
extern char g_426552;
extern char g_426553;
extern char g_426554;
extern unsigned int g_426558;
extern unsigned long long g_426560;
extern char g_426568;
extern unsigned long long g_426570;
extern char g_426578;
extern unsigned int g_42657c;
extern unsigned int g_426580;
extern char g_426584;
extern char g_426585;
extern char g_426586;
extern unsigned long long g_4265a0;
extern unsigned long long g_4265c0;
extern long long g_4265e0;
extern unsigned long long g_426600;
extern char g_426608;

unsigned int main(int a0, long long *a1)
{
    int iter;  // r14d
    unsigned long v19;  // rbp
    unsigned long v28;  // rax
    int v29;  // r10d
    unsigned int v30;  // eax
    long long v31;  // rax
    long long v32;  // rdx
    long long v33;  // r14
    struct_0 **v34;  // r14
    struct_0 **v35;  // rbx
    long long v36;  // rax
    long long v37;  // rdx
    long long *v20;  // rbx
    unsigned int v38;  // eax
    unsigned long v39;  // rax
    void* v40;  // rbp
    void* v41;  // rbp
    unsigned long v42;  // rax
    unsigned long v43;  // rax
    long long v44;  // rdi
    long long v45;  // r13
    long long v46;  // r12
    unsigned long v21;  // rdi
    long long v47;  // rbp
    long long v48;  // rax
    long long v49;  // rax
    char *v50;  // rax
    unsigned long v51;  // 4110
    long long v52;  // rsi
    unsigned long v53;  // rax
    unsigned long v54;  // rdi
    unsigned long v55;  // r11
    unsigned long long v56;  // rcx
    long long v22;  // rdi
    unsigned long v57;  // rdx
    unsigned long v58;  // rdi
    unsigned long v59;  // rax
    unsigned long v60;  // rdx
    long long v61;  // 4100
    unsigned int v62;  // eax
    unsigned long long v63;  // rax
    unsigned long v64;  // rdx
    unsigned long v65;  // rdx
    char *v66;  // rdi
    long long v23;  // rsi
    char v67;  // al
    char v68;  // al
    unsigned int v69;  // edx
    unsigned int v70;  // eax
    int v71;  // r12d
    int v72;  // eax
    unsigned long i;  // rax
    unsigned int v74;  // edx
    unsigned long v75;  // rax
    int v76;  // eax
    long long v24;  // rdx
    long long v25;  // rcx
    long long v26;  // r8
    long long v27;  // r9
    unsigned long long v0;  // [bp-0xc8]
    unsigned long long v1;  // [bp-0xc0]
    unsigned long long v2;  // [bp-0xb8]
    unsigned long long v3;  // [bp-0xb0]
    unsigned long long v4;  // [bp-0xa8]
    unsigned long long v5;  // [bp-0xa0]
    unsigned long long v6;  // [bp-0x98]
    unsigned long long v7;  // [bp-0x90]
    unsigned long v8;  // [bp-0x88]
    long long v9;  // [bp-0x80]
    char v10;  // [bp-0x74], Other Possible Types: int
    int v11;  // [bp-0x70], Other Possible Types: unsigned int
    long long *v12;  // [bp-0x68], Other Possible Types: unsigned int
    long long *v13;  // [bp-0x60]
    unsigned long v14;  // [bp-0x58]
    unsigned long v15;  // [bp-0x50]
    char *v16;  // [bp-0x48]

    iter = -0x1;
    v19 = a0;
    v20 = a1;
    v21 = *(a1);
    v14 = 0;
    v15 = 0;
    g_4260b0 = 2;
    sub_413d30(v21);
    setlocale(6, &g_41d5b0);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_40fa70(0);
    g_426600 = &g_426500;
    g_4265c0 = &g_4264c0;
    re_set_syntax(330310);
    v28 = sub_410050(v22, v23, v24, v25, v26, v27);
    g_426180 = 0;
    g_426410 = v28;
    sub_416390(v22, v23, v24, v25, v26, v27);
    v10 = 0;
    v8 = 0;
    *((char *)&v11) = 0;
    v9 = -0x1;
    while (1)
    {
        v29 = getopt_long(v19, v20, "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ", &g_425160.name, NULL);
        if (v29 != -0x1)
        {
            switch (v29)
            {
            case 0: case 104: case 128: case 133:
                iter = v29;
                continue;
            case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
                if (iter - 48 <= 9)
                {
                    if (v9 - (v29 <= 0x33) <= 461168601842738789)
                    {
                        iter = v29;
                        v9 = v29 - 48 + v9 * 10;
                        break;
                    }
                    else
                    {
                        iter = v29;
                        v9 = 0x3fffffffffffffff;
                        break;
                    }
                }
                else
                {
                    iter = v29;
                    v9 = v29 - 48;
                    break;
                }
            case 66:
                g_426554 = 1;
                iter = v29;
                continue;
            case 67: case 85:
                if (*((long long *)&optarg))
                {
                    v11 = v29;
                    v31 = strtoimax(*((long long *)&optarg), &v16, 10);
                    if (*(v16) || !(v29 = v11, v31 >= 0))
                        sub_408c00("invalid context length '%s'", *((long long *)&optarg)); /* do not return */
                    v32 = 0x3fffffffffffffff;
                    if (v31 <= 0x3fffffffffffffff)
                        v32 = v31;
                    v33 = v32;
                }
                else
                {
                    v33 = 3;
                }
                iter = v29;
                sub_408d50((unsigned long long)((v29 == 0x55) + 2));
                if (g_426570 < v33)
                {
                    g_426570 = v33;
                    v11 = _INSERT(iter, 0, 1);
                    break;
                }
                else
                {
                    v11 = _INSERT(iter, 0, 1);
                    break;
                }
            case 68:
                iter = v29;
                sub_408d50(7);
                v39 = sub_415f50(strlen(*((long long *)&optarg)) * 7 + 97);
                v7 = *((long long *)&optarg);
                v6 = *((long long *)&optarg);
                v5 = *((long long *)&optarg);
                v4 = 0;
                v3 = *((long long *)&optarg);
                v2 = *((long long *)&optarg);
                v1 = 0;
                v0 = *((long long *)&optarg);
                __sprintf_chk(v39, 0x1);
                v13 = v20;
                v12 = v19;
                v40 = &g_426460;
                do
                {
                    v41 = v40 + 8;
                    sub_408c60(v40, v39, "-D");
                    v39 = v39 + strlen(v39) + 1;
                    v40 = v41;
                } while (v40 != &g_426480);
                v19 = v12;
                v20 = v13;
                continue;
            case 69:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = g_426558 | 1;
                    iter = v29;
                    break;
                }
                break;
            case 70:
                iter = v29;
                sub_408930(&g_4265e0, *((long long *)&optarg));
                iter = iter;
                continue;
            case 72:
                g_426418 = 1;
                iter = v29;
                continue;
            case 73:
                iter = v29;
                sub_408930(&g_4265a0, *((long long *)&optarg));
                iter = iter;
                continue;
            case 76:
                if (g_426540)
                {
                    if (!g_426548)
                    {
                        iter = v29;
                        g_426548 = *((long long *)&optarg);
                        break;
                    }
                    else
                    {
                        sub_40e230("too many file label options"); /* do not return */
                    }
                }
                else
                {
                    iter = v29;
                    g_426540 = *((long long *)&optarg);
                    break;
                }
            case 78:
                g_426586 = 1;
                iter = v29;
                continue;
            case 80:
                g_426585 = 1;
                iter = v29;
                continue;
            case 83:
                iter = v29;
                sub_408c60(&g_426488, *((long long *)&optarg), "-S");
                iter = iter;
                continue;
            case 84:
                g_426492 = 1;
                iter = v29;
                continue;
            case 87:
                iter = v29;
                v43 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (v43 <= 0 || !(v29 = iter, !*(v16)))
                    sub_408c00("invalid width '%s'", *((long long *)&optarg)); /* do not return */
                if (v43 != v8)
                {
                    if (!v8)
                    {
                        v8 = v43;
                        iter = v29;
                        break;
                    }
                    else
                    {
                        sub_40e230("conflicting width options"); /* do not return */
                    }
                }
                break;
            case 88:
                iter = v29;
                v30 = sub_408910();
                v29 = iter;
                if (sub_410770(sub_4102e0, g_426410, *((long long *)&optarg), v30))
                    sub_40e1f0(*((long long *)&optarg)); /* do not return */
                break;
            case 90:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = g_426558 | 2;
                    iter = v29;
                    break;
                }
                break;
            case 97:
                g_426568 = 1;
                iter = v29;
                break;
            case 98:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = 4;
                    iter = v29;
                    break;
                }
                break;
            case 99:
                iter = v29;
                v44 = 2;
LABEL_405820:
                sub_408d50(v44);
                if (g_426570 <= 2)
                {
                    g_426570 = 3;
                    break;
                }
                break;
            case 100:
                g_426408 = 1;
                iter = v29;
                continue;
            case 101:
                iter = v29;
                sub_408d50(4);
                iter = iter;
                continue;
            case 102:
                iter = v29;
                sub_408d50(5);
                iter = iter;
                continue;
            case 105:
                g_426552 = 1;
                iter = v29;
                continue;
            case 108:
                if (1)
                {
                    iter = v29;
                    g_426480 = 1;
                    signal(0x11, NULL);
                    iter = iter;
                    break;
                }
                else
                {
                    sub_408c00("pagination not supported on this host", 0); /* do not return */
                }
            case 110:
                iter = v29;
                sub_408d50(6);
                iter = iter;
                continue;
            case 112:
                v10 = v29;
                sub_408930(&g_4265e0, "^[[:alpha:]$_]");
                iter = v10;
                v10 = 1;
                continue;
            case 113:
                g_4264a1 = 1;
                iter = v29;
                continue;
            case 114:
                g_426608 = 1;
                iter = v29;
                continue;
            case 115:
                g_426584 = 1;
                iter = v29;
                continue;
            case 116:
                g_4264a0 = 1;
                iter = v29;
                continue;
            case 117:
                iter = v29;
                v44 = 3;
                goto LABEL_405820;
            case 118:
                v45 = sub_414040("Len Tower");
                v46 = sub_414040("Richard Stallman");
                v47 = sub_414040("David Hayes");
                v48 = sub_414040("Mike Haertel");
                v49 = sub_414040("Paul Eggert");
                sub_415de0(stdout, "diff", "GNU diffutils", g_4260a8, v49, v48, v47, v46, v45, 0);
                sub_408e20(v22, v23, v24, v25, v26, v27);
                return 0;
            case 119:
                g_426558 = 5;
                iter = v29;
                continue;
            case 120:
                iter = v29;
                v38 = sub_408910();
                sub_4102e0(g_426410, *((long long *)&optarg), v38);
                iter = iter;
                continue;
            case 121:
                iter = v29;
                sub_408d50(8);
                iter = iter;
                continue;
            case 129:
                iter = v29;
                sub_408c60(&v14, *((long long *)&optarg), "--from-file");
                iter = iter;
                continue;
            case 130:
                sub_408a10();
                sub_408e20(v22, v23, v24, v25, v26, v27);
                return 0;
            case 131:
                iter = v29;
                v36 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (*(v16) || v36 < 0)
                    sub_408c00("invalid horizon length '%s'", *((long long *)&optarg)); /* do not return */
                v37 = g_426560;
                if (g_426560 < v36)
                    v37 = v36;
                g_426560 = v37;
                break;
            case 132:
                g_426551 = 1;
                iter = v29;
                continue;
            case 134:
                g_426439 = 1;
                iter = v29;
                continue;
            case 135:
                v34 = &g_426440;
                iter = v29;
                sub_408d50(7);
                v12 = v20;
                do
                {
                    v35 = v34 + 1;
                    sub_408c60(v34, *((long long *)&optarg), "--line-format");
                    v34 = v35;
                } while (v34 != &g_426458);
                v20 = v12;
                continue;
            case 136:
                g_426550 = 1;
                iter = v29;
                continue;
            case 137:
                g_426551 = 0;
                iter = v29;
                continue;
            case 138:
                iter = v29;
                sub_408d50(1);
                iter = iter;
                continue;
            case 139:
                iter = v29;
                sub_408d50(8);
                g_42643a = 1;
                continue;
            case 140:
                g_426490 = 1;
                iter = v29;
                continue;
            case 141:
                g_426491 = 1;
                iter = v29;
                continue;
            case 142:
                g_426438 = 1;
                iter = v29;
                continue;
            case 143:
                iter = v29;
                v42 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (v42 <= 0 || v42 > 0xfffffffffffffffc || !(v29 = iter, !*(v16)))
                    sub_408c00("invalid tabsize '%s'", *((long long *)&optarg)); /* do not return */
                if (v42 != g_426498)
                {
                    if (!g_426498)
                    {
                        g_426498 = v42;
                        iter = v29;
                        break;
                    }
                    else
                    {
                        sub_40e230("conflicting tabsize options"); /* do not return */
                    }
                }
                break;
            case 144:
                iter = v29;
                sub_408c60(&v15, *((long long *)&optarg), "--to-file");
                iter = iter;
                continue;
            case 145: case 146: case 147:
                iter = v29;
                sub_408d50(7);
                iter -= 145;
                sub_408c60(&(&g_426440)[iter], *((long long *)&optarg), (char *)(24 * iter + 4312960), &g_426440);
                continue;
            case 148: case 149: case 150: case 151:
                iter = v29;
                sub_408d50(7);
                iter -= 148;
                sub_408c60(&(&g_426460)[8 * iter], *((long long *)&optarg), (char *)(25 * iter + 4313056), &g_426460);
                continue;
            case 152:
                iter = v29;
                sub_408cc0(*((long long *)&optarg));
                iter = iter;
                continue;
            case 153:
                iter = v29;
                sub_40e270(*((long long *)&optarg));
                iter = iter;
                continue;
            case 154:
                g_426180 = 1;
                iter = v29;
                continue;
            default:
                sub_408c00(0, 0); /* do not return */
            }
        }
        else if (g_42657c == 1)
        {
            v50 = getenv("TERM");
            if (v50 && !strcmp(v50, "dumb"))
            {
                g_42657c = 0;
                break;
            }
        }
    }
    if (!g_426580)
    {
        if (v10)
        {
            sub_408d50(2);
            if (!(v9 - 0 >> 63 & 1))
            {
                if (g_426580 != 2 || (char)sub_410ab0(2))
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                else
                    g_426400 = "%a %b %e %T %Y";
                v52 = g_426570;
                if (g_426580 - 2 <= 1 && (v9 > v52 || v9 != v52 && !(char)v11))
                {
                    g_426570 = v9;
                    v52 = v9;
                    goto LABEL_4059ef;
                }
            }
            else
            {
                v51 = g_426580;
                g_426570 = 3;
                if ((unsigned int)v51 != 2)
                {
                    v52 = 3;
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                    goto LABEL_4059ef;
                }
                else if ((char)sub_410ab0(2))
                {
                    v52 = g_426570;
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                    goto LABEL_4059ef;
                }
                else
                {
                    g_426400 = "%a %b %e %T %Y";
LABEL_405b94:
                    v52 = g_426570;
                    goto LABEL_4059ef;
                }
            }
        }
        else
        {
            sub_408d50(1);
        }
    }
    if (g_426580 != 2 || (char)sub_410ab0(2))
        v53 = "%Y-%m-%d %H:%M:%S.%N %z";
    else
        v53 = "%a %b %e %T %Y";
    g_426400 = v53;
    if (v9 - 0 >> 63 & 1)
        goto LABEL_405b94;
LABEL_4059ef:
    if (!g_426498)
        g_426498 = 8;
    if (v8)
    {
        v54 = 1;
        if (g_4264a0)
            goto LABEL_405a26;
        goto LABEL_405a1f;
    }
    else if (!g_4264a0)
    {
        v8 = 130;
LABEL_405a1f:
        v54 = g_426498;
LABEL_405a26:
        v55 = v8;
        v56 = (v55 >> 1) + (v54 + 3 >> 1) + ((unsigned int)v54 + 3 & (unsigned int)v55 & 1);
        v57 = v56 % v54;
        v58 = v56 - v57;
        if (v58 > 3 && v58 < v55)
        {
            v59 = v57 - v56 + v55;
            v60 = v58 - 3;
            if (v59 > v60)
                v59 = v60;
            if (!v59)
                v58 = v55;
            v8 = v58;
        }
        else
        {
            v59 = 0;
        }
    }
    else
    {
        v8 = 67;
        v59 = 63;
    }
    g_426430 = v59;
    v61 = g_426560;
    g_426428 = v8;
    if (v61 < v52)
        g_426560 = v52;
    sub_408ba0(&g_4265e0);
    sub_408ba0(&g_4265a0);
    v62 = g_426580;
    if (g_426580 != 7)
    {
LABEL_405acd:
        v67 = v62 != 8 | g_426438;
        goto LABEL_405ad9;
    }
    if (!g_426440)
        g_426440 = "%l\n";
    if (!g_426448)
        g_426448 = "%l\n";
    if (!g_426450)
        g_426450 = "%l\n";
    v63 = g_426478;
    if (!g_426468)
    {
        v64 = "%<";
        if (v63)
            v64 = v63;
        g_426468 = v64;
    }
    if (!g_426470)
    {
        v65 = "%>";
        if (v63)
            v65 = v63;
        g_426470 = v65;
    }
    if (!*((long long *)&g_426460))
    {
        *((char **)&g_426460) = "%=";
        v66 = "%=";
        if (v63)
            goto LABEL_405dae;
    }
    else if (!(!v63))
    {
        goto LABEL_405c77;
    }
    g_426478 = sub_40f560(g_426468, g_426470, &g_41d5b0);
    v62 = g_426580;
    if (g_426580 != 7)
        goto LABEL_405acd;
LABEL_405c77:
    v66 = *((long long *)&g_426460);
    v68 = 1;
    if (!*((char *)*((long long *)&g_426460)))
        goto LABEL_405c93;
LABEL_405dae:
    v68 = 0;
    if (!strcmp(v66, "%="))
        v68 = !g_426440->field_0;
LABEL_405c93:
    v67 = v68 & 1;
LABEL_405ad9:
    g_426578 = v67;
    v69 = 1;
    if (!g_4265a0)
        v69 = g_426558;
    g_426553 = (char)~(g_426554 | g_426552 | g_426490 | v69) & g_4264a1;
    g_426553 = g_426553 & 1;
    g_426420 = sub_408d80(v20 + 1, optind - 1);
    if (!v14)
    {
        i = optind;
        if (!v15)
        {
            v74 = (v19 & 0xffffffff) - optind;
            if (v74 != 2)
            {
                if (v74 > 1)
                    sub_408c00("extra operand '%s'", v20[2 + optind]); /* do not return */
                sub_408c00("missing operand after '%s'", *((long long *)((char *)&v20[v19] - 8))); /* do not return */
            }
            v75 = optind + 1;
            v71 = sub_408e70(0, *((long long *)((char *)&v20[v75] - 8)), v20[v75]);
        }
        else
        {
            for (v71 = 0; (unsigned int)i < (unsigned int)v19; i = optind)
            {
                v76 = sub_408e70(0, v20[i], v15);
                if (v71 < v76)
                    v71 = v76;
                optind = optind + 1;
            }
        }
    }
    else if (v15)
    {
        sub_40e230("--from-file and --to-file both specified"); /* do not return */
    }
    else
    {
        v70 = optind;
        v71 = 0;
        if (optind < (unsigned int)v19)
        {
            do
            {
                v72 = sub_408e70(0, v14, v20[v70]);
                if (v71 < v72)
                    v71 = v72;
            } while ((v70 = optind + 1, optind = v70, v70 < (unsigned int)v19));
        }
    }
    sub_40e0f0();
    sub_408e20(v22, v23, v24, v25, v26, v27);
    exit(v71); /* do not return */
}



// Function: shift_boundaries @ 0x6a70
void shift_boundaries(unsigned long a0)
{
    unsigned long long v1;  // rbp
    unsigned long v2;  // rsi
    long long v11;  // rax
    long long iter;  // rax
    long long node;  // rdx
    long long v14;  // rax
    void* iter1;  // r10
    unsigned long long iter2;  // rcx
    long long i;  // r8
    unsigned long v5;  // r9
    long long v6;  // r11
    unsigned long v7;  // rdi
    long long v8;  // rax
    long long l;  // rdx
    long long v10;  // rdx

    v1 = 0;
    while (1)
    {
        v2 = *((long long *)(a0 + v1 + 280));
        iter2 = 0;
        i = 0;
        v5 = *((long long *)(a0 + v1 + 248));
        v6 = *((long long *)(a0 + v1 + 200));
        v7 = *((long long *)(a0 + -(v1) + 584));
        while (i < v6)
        {
LABEL_406ab5:
            if (*((char *)(v2 + i)))
            {
                do
                {
LABEL_406ac0:
                    v8 = i;
                    do
                    {
                        v8 += 1;
                    } while (*((char *)(v2 + v8)));
                    for (; *((char *)(v7 + iter2)); iter2 += 1);
                    l = i;
                    do
                    {
                        v10 = l;
                        v11 = v8;
                        iter = v11;
                        node = v10;
                        v8 = v11;
                        l = v10;
                        if (node)
                        {
                            do
                            {
                                v8 = iter;
                                l = node;
                                if (*((long long *)(v5 + l * 8 - 8)) != *((long long *)(v5 + v8 * 8 - 8)))
                                    break;
                                node -= 1;
                                iter -= 1;
                                *((char *)(v2 + node)) = 1;
                                *((char *)(v2 + iter)) = 0;
                                if (*((char *)(v2 + node - 2)))
                                {
                                    do
                                    {
                                        node -= 1;
                                    } while (*((char *)(v2 + node - 1)));
                                }
                                do
                                {
                                    iter2 -= 1;
                                } while (*((char *)(v7 + iter2)));
                                v8 = iter;
                                l = node;
                            } while (l);
                        }
                        if (!*((char *)(v7 + iter2 - 1)))
                            i = v6;
                        else
                            i = v8;
                        if (v6 != v8)
                        {
                            do
                            {
                                if (*((long long *)(v5 + l * 8)) != *((long long *)(v5 + v8 * 8)))
                                    break;
                                l += 1;
                                v14 = v8 + 1;
                                *((char *)(v2 + l - 1)) = 0;
                                *((char *)(v2 + v8)) = 1;
                                if (*((char *)(v2 + v14)))
                                {
                                    do
                                    {
                                        v14 += 1;
                                    } while (*((char *)(v2 + v14)));
                                    iter2 += 1;
                                    if (!*((char *)(v7 + iter2)))
                                        continue;
                                }
                                do
                                {
                                    i = v14;
                                } while ((iter2 += 1, *((char *)(v7 + iter2))));
                            } while ((v8 = v14, v6 != v8));
                        }
                    } while (v8 - l != v11 - v10);
                    if (v8 <= i)
                        i = v8;
                    iter1 = v2 + v8 - 1;
                    do
                    {
                        *((char *)iter1 - v8 + l) = 1;
                        *((char *)iter1) = 0;
                        do
                        {
                            iter2 -= 1;
                        } while (*((char *)(v7 + iter2)));
                        iter1 -= 1;
                    } while (v2 - 1 + i != iter1);
                    if (i < v6)
                        goto LABEL_406ab5;
                } while (i != v6);
                goto LABEL_406bf0;
            }
            else
            {
                do
                {
                    iter2 += 1;
                } while (*((char *)(v7 + iter2 - 1)));
                i += 1;
            }
        }
        if (i != v6)
            goto LABEL_406ac0;
LABEL_406bf0:
        v1 += 304;
        if (v1 == 608)
            return;
    }
}



// Function: add_change @ 0x6c30
void add_change(unsigned long a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4)
{
    unsigned long long *idx;  // rax

    idx = sub_415f50(48);
    idx[3] = a0;
    idx[4] = a1;
    idx[1] = a3;
    idx[2] = a2;
    *(idx) = a4;
    return;
}



// Function: build_reverse_script @ 0x6c70
long long build_reverse_script(void* a0)
{
    unsigned long i;  // rsi
    unsigned long v2;  // rdi
    unsigned long v3;  // r8
    unsigned long iter;  // rbx
    unsigned long long v5;  // r9
    unsigned long node;  // rbp
    unsigned long long v7;  // rcx

    if ((long long)a0[200] <= 0 && (long long)a0[504] <= 0)
        return 0;
    i = 0;
    v2 = 0;
    v3 = 0;
    do
    {
        do
        {
            if (*((char *)((long long)a0[280] + v2)) || *((char *)((long long)a0[584] + i)))
            {
                iter = v2;
                if (*((char *)((long long)a0[280] + v2)))
                {
                    do
                    {
                        iter += 1;
                    } while (*((char *)((long long)a0[280] + iter)));
                    v5 = iter - v2;
                }
                else
                {
                    v5 = 0;
                }
                node = i;
                if (*((char *)((long long)a0[584] + i)))
                {
                    do
                    {
                        node += 1;
                    } while (*((char *)((long long)a0[584] + node)));
                    v7 = node - i;
                }
                else
                {
                    v7 = 0;
                }
                v3 = (unsigned long long)sub_406c30(v2, i, v5, v7, v3);
                v2 = iter;
                i = node;
            }
        } while ((v2 = (unsigned long)(v2 + 1), i = (unsigned long)(i + 1), (long long)a0[200] > v2));
    } while ((long long)a0[504] > i);
    return v3;
}



// Function: build_script @ 0x6d50
long long build_script(void* idx)
{
    unsigned long v1;  // r8
    unsigned long v2;  // rbx
    unsigned long iter;  // rbp
    char v4;  // cl
    unsigned long v5;  // rbx
    unsigned long v6;  // rdx
    unsigned long long v7;  // r9
    unsigned long v8;  // rdx
    unsigned long long v9;  // rcx

    v1 = 0;
    v2 = (long long)idx[200];
    iter = (long long)idx[504];
    if (((long long)idx[504] & (long long)idx[200]) < 0)
        return 0;
    do
    {
        do
        {
            v4 = *((char *)((long long)idx[584] + iter - 1));
            v5 = v2;
            if (!*((char *)((long long)idx[280] + v5 - 1)) && !v4)
                continue;
            if (*((char *)((long long)idx[280] + v2 - 1)))
            {
                v6 = v2;
                do
                {
                    v6 -= 1;
                } while (*((char *)((long long)idx[280] + v6 - 1)));
                v7 = v2 - v6;
                v2 = v6;
                if (!v4)
                    goto LABEL_406e20;
                goto LABEL_406dd9;
            }
            else
            {
                v7 = 0;
                if (v4)
                {
LABEL_406dd9:
                    v8 = iter;
                    do
                    {
                        v8 -= 1;
                    } while (*((char *)((long long)idx[584] + v8 - 1)));
                    v9 = iter - v8;
                    iter = v8;
                    goto LABEL_406df5;
                }
                else
                {
LABEL_406e20:
                    v9 = 0;
LABEL_406df5:
                    v1 = (unsigned long long)sub_406c30(v2, iter, v7, v9, v1);
                }
            }
        } while ((iter = (unsigned long)(iter - 1), v2 = (unsigned long)(v5 - 1), !(v5 - 1 >> 63 & 1)));
    } while (iter >= 0);
    return v1;
}



// Function: discard_confusing_lines @ 0x6e30
typedef struct struct_0 {
    char padding_0[200];
    long long field_c8;
    char padding_d0[40];
    unsigned long long field_f8;
    unsigned long long field_100;
    unsigned long long field_108;
    unsigned long long field_110;
    unsigned long long field_118;
    char padding_120[8];
    unsigned long long field_128;
    char padding_130[200];
    long long field_1f8;
    char padding_200[40];
    unsigned long long field_228;
    unsigned long long field_230;
    unsigned long long field_238;
} struct_0;

extern char g_426408;

void discard_confusing_lines(struct_0 *idx)
{
    unsigned long long v6;  // rax
    long long v7;  // rsi
    long long v16;  // rax
    long long j;  // rdi
    long long idx1;  // rax
    long long v19;  // rax
    void* v20;  // rax
    long long k;  // rcx
    void* *iter;  // r9
    struct_0 *node;  // r8
    void* v24;  // r15
    void* iter1;  // r10
    unsigned long long v8;  // rdi
    unsigned long v26;  // rsi
    void* v27;  // rdi
    long long v28;  // rax
    unsigned long long v29;  // rax
    unsigned long long v30;  // rdx
    long long iter2;  // rax
    long long v32;  // rax
    unsigned long long v33;  // rax
    unsigned long long v34;  // rax
    void* v35;  // rdx
    unsigned long long v9;  // rdx
    unsigned long long v36;  // r13
    void* *v37;  // rbp
    long long v38;  // r8
    long long v39;  // rax
    char *v40;  // r9
    long long v41;  // rsi
    long long v42;  // rcx
    unsigned long long v43;  // rdi
    void* v44;  // rcx
    long long v45;  // rcx
    unsigned long long v10;  // rax
    long long v46;  // rsi
    unsigned long long v47;  // r11
    long long v48;  // rcx
    long long v49;  // rcx
    unsigned long long v50;  // r12
    unsigned long long v51;  // rcx
    unsigned long long v52;  // r12
    char *v53;  // r10
    unsigned long long v54;  // r15
    char *v55;  // rax
    unsigned long long v11;  // rax
    char *v56;  // r12
    unsigned long long v57;  // r11
    char v58;  // cl
    char *v59;  // rax
    unsigned long long v60;  // rdi
    char *v61;  // rax
    struct_0 *v62;  // rcx
    void* *v63;  // r14
    void* v64;  // r10
    long long v65;  // r9
    long long v12;  // rdx
    unsigned long long idx2;  // rdx
    unsigned long long v67;  // rdx
    long long v68;  // rax
    void* v13;  // rbp
    unsigned long long v14;  // r12
    long long i;  // rax
    void* v0;  // [bp-0x80]
    void* ptr;  // [bp-0x78]
    void* *v2;  // [bp-0x70]
    void* v3;  // [bp-0x68]
    unsigned long long v4;  // [bp-0x50]

    v6 = sub_415f50((idx->field_1f8 + idx->field_c8) * 16);
    v7 = idx->field_c8;
    v8 = idx->field_128;
    idx->field_100 = v6;
    v9 = v7 * 8;
    v10 = v6 + v9;
    idx->field_108 = v10;
    v11 = v10 + v9;
    v12 = idx->field_1f8;
    idx->field_230 = v11;
    idx->field_238 = v11 + v12 * 8;
    v13 = sub_40f5e0(v8 * 16, v7, v12);
    (&v3)[2] = v13;
    v14 = v13 + idx->field_128 * 8;
    v4 = v14;
    if (idx->field_c8 > 0)
    {
        i = 0;
        do
        {
            v16 = i + 1;
            *((unsigned long long *)((char *)v13 + 8 * *((long long *)(idx->field_f8 + i * 8)))) = *((long long *)((char *)v13 + 8 * *((long long *)(idx->field_f8 + i * 8)))) + 1;
            i = v16;
        } while (idx->field_c8 > i);
    }
    j = idx->field_1f8;
    if (j > 0)
    {
        idx1 = 0;
        do
        {
            v19 = idx1 + 1;
            *((unsigned long long *)(v14 + *((long long *)(idx->field_228 + idx1 * 8)) * 8)) = *((long long *)(v14 + *((long long *)(idx->field_228 + idx1 * 8)) * 8)) + 1;
            j = idx->field_1f8;
            idx1 = v19;
        } while (j > idx1);
    }
    v20 = sub_40f5e0(j + idx->field_c8);
    k = idx->field_c8;
    iter = &v3;
    node = &idx->field_f8;
    v24 = v20;
    iter1 = (char *)&v3 - 16;
    v26 = *((long long *)&node->padding_0[0]);
    v27 = v24;
    (&v3)[1] = v24 + k;
    v28 = k;
    v29 = v28 >> 8;
    if (v28 & 0xffffffffffffff00)
    {
LABEL_406f76:
        v30 = 5;
        do
        {
            v30 *= 2;
            v34 = v29 >> 2;
            v29 = v34;
        } while (v33 & 0xfffffffffffffffc);
    }
    else
    {
        while (!k)
        {
LABEL_406fc8:
            node = node->padding_130;
            iter += 1;
            if (&idx[1].field_118 == node)
            {
                v0 = v24;
                v35 = v24;
                v36 = 0;
                v2 = &v3;
                ptr = v13;
                v37 = &v3;
                while (1)
                {
                    v38 = *((long long *)&idx->padding_0[v36 + 200]);
                    v39 = 0;
                    if (v38 > 0)
                    {
                        while (1)
                        {
LABEL_407021:
                            v40 = v35 + v39;
                            if (*(v40) != 2)
                                break;
                            v39 += 1;
                            *(v40) = 0;
                            if (v38 <= v39)
                                goto LABEL_40703b;
                        }
                        if (*(v40))
                        {
                            v41 = 0;
                            v42 = v39;
                            while (1)
                            {
                                v42 += 1;
                                if (v38 <= v42 || !*((char *)v35 + v42))
                                    break;
                                v41 += *((char *)v35 + v42) == 2;
                            }
                            if (v42 > v39)
                            {
                                do
                                {
                                    if (*((char *)v35 + v42 - 1) != 2)
                                    {
                                        if (v41 * 4 > v42 - v39)
                                        {
                                            if (v42 > v39)
                                            {
                                                v44 = v35 + v42 - 1;
                                                do
                                                {
                                                    if (*((char *)v44) == 2)
                                                        *((char *)v44) = 0;
                                                } while ((v44 -= 1, v35 - 1 + v39 != v44));
                                            }
                                            goto LABEL_407018;
                                        }
                                        goto LABEL_4071cb;
                                    }
                                } while ((v42 = (long long)(v42 - 1), v41 = (long long)(v41 - 1), *((char *)((char *)v35 + v42)) = 0, v42 != v39));
                                v43 = 0;
                                if (v41 <= 0)
                                    goto LABEL_40719b;
                            }
                            else
                            {
                                v43 = v42 - v39;
                                if (v43 >= v41 * 4)
                                {
LABEL_4071cb:
                                    v45 = (long long)(v43) >> 4;
                                    if (v45 <= 0)
                                    {
LABEL_40719b:
                                        v46 = v43 + v39 - 1;
                                        if (v43 <= 0)
                                            goto LABEL_4071b0;
                                        v47 = 1;
                                        goto LABEL_4071ee;
                                    }
                                    else
                                    {
                                        v47 = 1;
                                        do
                                        {
                                            v47 *= 2;
                                            v49 = v45 >> 2;
                                            v45 = v49;
                                        } while (v48 >> 2);
                                        v46 = v43 + v39 - 1;
LABEL_4071ee:
                                        v50 = 0;
                                        v51 = 0;
                                        while (1)
                                        {
                                            v52 = v50;
                                            v53 = v51 + v39 + v35;
                                            if (*(v53) != 2)
                                            {
                                                v50 = 0;
                                            }
                                            else
                                            {
                                                v54 = v52 + 1;
                                                if (v52 != v47)
                                                {
                                                    v50 = v54;
                                                    if (v52 > v47)
                                                    {
                                                        v51 += 1;
                                                        *(v53) = 0;
                                                        if (v51 >= v43)
                                                            break;
                                                        else
                                                            continue;
                                                    }
                                                }
                                                else
                                                {
                                                    v51 -= v54;
                                                    v50 = v54;
                                                }
                                            }
                                            v51 += 1;
                                            if (v51 >= v43)
                                                break;
                                        }
                                        v55 = v40;
                                        v56 = &v40[v43];
                                        v57 = 0;
                                        while (1)
                                        {
                                            v58 = *(v55);
                                            if (v55 - v40 > 7 && v58 == 1)
                                                break;
                                            switch (v58)
                                            {
                                            case 2:
                                                *(v55) = 0;
                                                v55 += 1;
                                                v57 = 0;
                                                if (v56 == v55)
                                                    goto LABEL_407290;
                                                else
                                                    continue;
                                            case 0:
                                                v57 = 0;
                                                break;
                                            default:
                                                v57 += 1;
                                                if (v57 == 3)
                                                    goto LABEL_407290;
                                                else
                                                    break;
                                            }
                                            v55 += 1;
                                            if (v56 == v55)
                                                break;
                                        }
LABEL_407290:
                                        v59 = v35 + v46;
                                        v60 = 0;
                                        v61 = v59;
                                        do
                                        {
                                            if (v59 - v61 > 7 && *(v61) == 1)
                                                break;
                                            if (*(v61) == 2)
                                            {
                                                *(v61) = 0;
                                                v60 = 0;
                                            }
                                            else if (!*(v61))
                                            {
                                                v60 = 0;
                                            }
                                            else if (v60 + 1 == 3)
                                            {
                                                break;
                                            }
                                        } while ((v61 -= 1, v35 - v43 + v46 != v61));
LABEL_4071b0:
                                        v39 = v46;
                                    }
                                }
                            }
                        }
LABEL_407018:
                        v39 += 1;
                        if (v38 > v39)
                            goto LABEL_407021;
                    }
LABEL_40703b:
                    v36 += 304;
                    v37 += 1;
                    if (v36 == 608)
                        break;
                    v35 = *(v37);
                }
                v62 = &idx->field_c8;
                v63 = v2;
                v64 = v0;
                while (1)
                {
                    v65 = *((long long *)&v62->padding_0[0]);
                    idx2 = 0;
                    v67 = 0;
                    if (v65 > 0)
                    {
                        v68 = 0;
                        while (1)
                        {
                            if (!g_426408 && *((char *)v64 + v68))
                            {
                                *((char *)(*((long long *)&v62->padding_0[80]) + v68)) = 1;
                                v68 += 1;
                                v67 = idx2;
                                if (v65 == v68)
                                    break;
                            }
                            else
                            {
                                *((long long *)(*((long long *)&v62->padding_0[56]) + idx2 * 8)) = *((long long *)(*((long long *)&v62->padding_0[48]) + v68 * 8));
                                v67 = idx2 + 1;
                                *((long long *)(*((long long *)&v62->padding_0[64]) + idx2 * 8)) = v68;
                                v68 += 1;
                                idx2 = v67;
                                if (v65 == v68)
                                    break;
                            }
                        }
                    }
                    *((unsigned long long *)&v62->padding_0[72]) = v67;
                    v62 = v62->padding_130;
                    v63 += 1;
                    if (&idx[1].padding_d0[24] == v62)
                        break;
                    v64 = *(v63);
                }
                free(v0);
                free(ptr);
                return;
            }
            k = *((long long *)(&node->padding_0[0] - 48));
            v14 = *((long long *)iter1);
            iter1 -= 8;
            v27 = *(iter);
            v26 = *((long long *)&node->padding_0[0]);
            v32 = k;
            v29 = v32 >> 8;
            goto LABEL_406f76;
        }
        v30 = 5;
    }
    iter2 = 0;
    while (1)
    {
        if (!*((long long *)(v26 + iter2 * 8)))
        {
LABEL_406f94:
            iter2 += 1;
            if (iter2 == k)
                break;
        }
        else
        {
            if (!*((long long *)(v14 + *((long long *)(v26 + iter2 * 8)) * 8)))
            {
                *((char *)v27 + iter2) = 1;
                goto LABEL_406f94;
            }
            if (*((long long *)(v14 + *((long long *)(v26 + iter2 * 8)) * 8)) <= v30)
                goto LABEL_406f94;
            *((char *)v27 + iter2) = 2;
            iter2 += 1;
            if (iter2 == k)
                break;
        }
    }
    goto LABEL_406fc8;
}



// Function: briefly_report @ 0x73b0
typedef struct struct_0 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[296];
    long long field_138;
} struct_0;

extern char g_4264a1;
extern unsigned long long g_426540;
extern long long g_426548;

long long briefly_report(unsigned int a0, struct_0 *idx)
{
    unsigned long long v2;  // r12
    long long v3;  // r12
    unsigned long long v4;  // rbp
    unsigned long v5;  // rax
    unsigned long v6;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!a0)
        return v6;
    v0 = v2;
    v3 = g_426548;
    if (g_426548)
    {
        v4 = g_426540;
        if (!g_426540)
            goto LABEL_407428;
LABEL_4073d3:
        if (g_4264a1)
            goto LABEL_407408;
LABEL_4073e1:
        v5 = dcgettext(NULL, "Binary files %s and %s differ\n", 5);
    }
    else
    {
        v4 = g_426540;
        v3 = idx->field_138;
        if (g_426540)
            goto LABEL_4073d3;
LABEL_407428:
        v4 = idx->field_8;
        if (!g_4264a1)
            goto LABEL_4073e1;
LABEL_407408:
        v5 = dcgettext(NULL, "Files %s and %s differ\n", 5);
    }
    return sub_40e0e0(v5, v4, v3);
}



// Function: diff_2_files @ 0x7460
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern unsigned int g_41b058[4];
extern unsigned long long g_4261a0;
extern char g_426418;
extern char g_4264a1;
extern unsigned long long g_4264e0;
extern long long g_426540;
extern long long g_426550;
extern char g_426554;
extern char g_426578;
extern unsigned int g_426580;

long long diff_2_files(void* a0)
{
    void* idx;  // r15
    unsigned long v15;  // rbp
    long long v24;  // rbx
    long long v25;  // rbx
    unsigned long long v26;  // rcx
    unsigned long long *iter;  // rdi
    void* node;  // rsi
    void* j;  // rsi
    unsigned long long v30;  // rcx
    struct_0 **v32;  // rbx
    struct_0 **v33;  // rbp
    unsigned long long v16;  // rax
    struct_0 **v34;  // rax
    struct_0 **v35;  // rbp
    unsigned int v36;  // eax
    long long v37;  // rdi
    struct_0 **ptr;  // rbx
    struct_0 **v39;  // rbx
    long long *v40;  // rbx
    void* v41;  // rbp
    unsigned long long v17;  // rdi
    unsigned long v18;  // rbx
    unsigned int v19;  // r12d
    long long v20;  // rbx
    unsigned long long v21;  // rax
    unsigned long v22;  // rdx
    unsigned long long v23;  // rax
    char *v0;  // [bp-0xb0]
    char *v1;  // [bp-0xa8]
    void* v2;  // [bp-0xa0]
    char v3;  // [bp-0x98]
    char v4;  // [bp-0x90]
    char v5;  // [bp-0x88]
    char v6;  // [bp-0x80]
    unsigned long v7;  // [bp-0x78]
    unsigned long v8;  // [bp-0x70]
    unsigned long long v9;  // [bp-0x68]
    unsigned long long v10;  // [bp-0x60]
    char v11;  // [bp-0x58]
    unsigned long long v12;  // [bp-0x50]

    idx = a0;
    if ((char)sub_40c8f0())
    {
        if (!((long long)idx[64] != (long long)idx[368] & (long long)idx[64] > 0) || (long long)idx[368] <= 0 || *((int *)idx) >= 0 && ((short)(int)idx[40] & 0xf000) != 0x8000 || (int)idx[304] >= 0 && ((short)(int)idx[344] & 0xf000) != 0x8000)
        {
            if (*((int *)idx) != (int)idx[304])
            {
                v15 = sub_4165d0(8, sub_4165d0((long long)idx[72], (long long)idx[376], 0x7ffffffffffffffe), 0x7ffffffffffffffe);
                v16 = sub_415fa0((long long)idx[160], v15);
                v17 = (long long)idx[464];
                *((unsigned long long *)&idx[160]) = v16;
                *((unsigned long long *)&idx[464]) = sub_415fa0(v17, v15);
                while (1)
                {
                    if (*((int *)idx) >= 0)
                        sub_40bf50(idx, v15 - (long long)idx[176]);
                    if ((int)idx[304] >= 0)
                        sub_40bf50(idx + 304, v15 - (long long)idx[480]);
                    v18 = (long long)idx[176];
                    if (v18 != (long long)idx[480] || memcmp((long long)idx[160], (long long)idx[464], v18))
                        break;
                    if (v18 != v15)
                        goto LABEL_4075c8;
                    *((unsigned long *)&idx[480]) = 0;
                    *((unsigned long *)&idx[176]) = 0;
                }
                goto LABEL_4075bb;
            }
LABEL_4075c8:
            v19 = 0;
        }
        else
        {
LABEL_4075bb:
            v19 = 1;
        }
        sub_4073b0(v19, idx);
        goto LABEL_4075d6;
    }
    v2 = sub_40f5e0((long long)idx[504] + (long long)idx[200] + 4);
    *((void* *)&idx[280]) = v2 + 1;
    *((void* *)&idx[584]) = v2 + (long long)idx[200] + 3;
    sub_406e30(idx);
    v20 = (long long)idx[576] + (long long)idx[272] + 3;
    v7 = (long long)idx[0x100];
    v8 = (long long)idx[560];
    v21 = sub_415f50(v20 * 16);
    v22 = (long long)idx[576] * 8 + 8;
    v9 = v21 + v22;
    v10 = v21 + v22 + v20 * 8;
    v11 = g_426418;
    if (v20)
    {
        v23 = 1;
        do
        {
            v23 *= 2;
            v25 = v20 >> 2;
            v20 = v25;
        } while (v24 >> 2);
        if (v23 < 0x1000)
            v23 = 0x1000;
    }
    else
    {
        v23 = 0x1000;
    }
    v12 = v23;
    v26 = 38;
    iter = &g_4261a0;
    for (node = idx; v26; node += 8)
    {
        v26 -= 1;
        *(iter) = *((long long *)node);
        iter += 1;
    }
    j = idx + 304;
    for (v30 = 38; v30; j += 8)
    {
        v30 -= 1;
        *(iter) = *((long long *)j);
        iter += 1;
    }
    sub_406890(0, (long long)idx[272]);
    free(0xfffffffffffffff8 - (long long)idx[576] * 8 + v9);
    sub_406a70(idx);
    v32 = (g_426580 == 4 ? sub_406c70(idx) : sub_406d50(idx));
    if (g_426554 || g_4264e0)
    {
        if (v32)
        {
            v33 = v32;
            v1 = &v4;
            v0 = &v3;
            do
            {
                v34 = sub_40eaf0(v33);
            } while ((v35 = (struct_0 **)*(v34), *(v34) = (struct_0 *)NULL, v36 = (unsigned int)(unsigned long long)sub_40f1a0(v33, v0, v1, &v5, &v6), *(v34) = (struct_0 *)v35, v35 && (v33 = v35, !v36)));
            v19 = v36;
LABEL_40780b:
            if (g_4264a1)
                goto LABEL_4079cb;
            if (v19)
                goto LABEL_407821;
        }
        else if (g_4264a1)
        {
            v19 = 0;
LABEL_4079cb:
            sub_4073b0(v19, idx);
LABEL_4079d6:
            free((long long)idx[0x100]);
            free(v2);
            free((long long)idx[248]);
            free((long long)idx[184] + (long long)idx[192] * 8);
            free((long long)idx[552]);
            free((long long)idx[488] + (long long)idx[496] * 8);
            if (v32)
            {
                do
                {
                    ptr = v32;
                    v39 = *(ptr);
                    free(ptr);
                } while ((v32 = v39, *(ptr)));
            }
            if (g_426580 - 4 <= 1)
            {
                v40 = &g_426540;
                v41 = idx + 288;
                do
                {
                    if (*((char *)v41))
                    {
                        dcgettext(NULL, "No newline at end of file", 5);
                        error(0, 0, "%s: %s\n");
                        v19 = 2;
                    }
                } while ((v40 += 8, v41 += 304, v40 != &g_426550));
            }
LABEL_4075d6:
            if ((long long)idx[160] != (long long)idx[464])
            {
                free((long long)idx[160]);
                free((long long)idx[464]);
            }
            else
            {
                free((long long)idx[464]);
            }
            return v19;
        }
        v19 = 0;
        if (g_426578)
            goto LABEL_4079d6;
LABEL_407821:
        v37 = g_426540;
        if (!g_426540)
            v37 = (long long)idx[8];
        sub_40e280(v37);
        if (g_426580 > 8)
            abort(); /* do not return */
        goto *((void *)((long long)(g_41b058[g_426580] + (char *)&g_41b058[0])));
    }
    else
    {
        v19 = v32;
        goto LABEL_40780b;
    }
}



// Function: print_context_label @ 0x7ab0
typedef struct FILE {
} FILE;

extern FILE *g_426188;
extern long long g_426400;

int print_context_label(char *a0, unsigned long a1, char *a2, char *a3)
{
    unsigned long tm;  // r14
    unsigned int v3;  // eax
    char v0[56];  // [bp-0x78]

    sub_40eb70(0);
    if (a3)
    {
        __fprintf_chk(*((unsigned int *)&g_426188), 0x1, "%s %s", a0, a3);
    }
    else
    {
        tm = localtime(a1 + 104);
        v3 = sub_4154d0(a1 + 16);
        if (!tm || !sub_413d10(v0, 43, g_426400, tm, 0, v3))
            __sprintf_chk(v0, 0x1);
        __fprintf_chk(*((unsigned int *)&g_426188), 0x1, "%s %s\t%s", a0, a2, v0);
    }
    sub_40eb70(3);
    return putc_unlocked(10, g_426188);
}



// Function: mark_ignorable @ 0x7be0
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[32];
    char field_28;
} struct_0;

void mark_ignorable(struct_0 *a0)
{
    struct_0 *i;  // rbx
    struct_0 *v6;  // rbp
    unsigned int v7;  // eax
    char v0;  // [bp-0x60]
    char v1;  // [bp-0x58]
    char v2;  // [bp-0x50]
    char v3;  // [bp-0x48]

    if (!a0)
        return;
    i = a0;
    do
    {
        v6 = i;
        i = i->field_0;
        v6->field_0 = NULL;
        v7 = sub_40f1a0(v6, &v0, &v1, &v2, &v3);
        v6->field_0 = i;
        v6->field_28 = !v7;
    } while (i);
    return;
}



// Function: find_function @ 0x7c80
extern unsigned long long g_426170;
extern long long g_426178;

unsigned long long find_function(unsigned long a0, long long a1)
{
    long long v1;  // rbp
    long long idx;  // rbx
    unsigned long long v3;  // r12
    unsigned long long v4;  // r12

    v1 = g_426178;
    idx = a1;
    g_426178 = a1;
    while (1)
    {
        idx -= 1;
        if (v1 > idx)
            break;
        v3 = *((long long *)(a0 + idx * 8));
        if (re_search() >= 0)
        {
            g_426170 = idx;
            return v3;
        }
    }
    v4 = 0;
    if (g_426170 != 0x7fffffffffffffff)
        v4 = *((long long *)(a0 + g_426170 * 8));
    return v4;
}



// Function: print_context_function @ 0x7d30
void print_context_function(long long a0, unsigned long a1)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    sub_407d4e(32, a0);
    return;
}



// Function: print_context_number_range @ 0x7e00
extern unsigned long long g_426188;

void print_context_number_range(void)
{
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_40f0c0();
    if (v1 > v0)
    {
        __fprintf_chk(g_426188, 0x1, "%ld,%ld", v0, v1);
        return;
    }
    __fprintf_chk(g_426188, 0x1, "%ld", v1);
    return;
}



// Function: pr_context_hunk @ 0x7e90
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    long long field_10;
    char padding_18[8];
    long long field_20;
} struct_0;

extern unsigned long long g_426188;
extern unsigned int g_426258;
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_426388;
extern unsigned long long g_4263a0;
extern unsigned long long g_426520;
extern unsigned long long g_426570;

void pr_context_hunk(struct_0 *a0)
{
    struct_0 *v6;  // rbx
    unsigned int v7;  // eax
    unsigned long long v16;  // r13
    long long v17;  // rdi
    long long v18;  // rsi
    long long v19;  // rdx
    long long v20;  // rcx
    long long v21;  // r8
    long long v22;  // r9
    unsigned long v23;  // rbp
    long long k;  // r15
    struct_0 *v25;  // r14
    unsigned int v8;  // r12d
    long long v26;  // rdi
    unsigned long long v27;  // r13
    unsigned long l;  // r15
    long long v29;  // rdi
    unsigned long long v30;  // r13
    unsigned long v9;  // rcx
    long long v10;  // rdx
    long long v11;  // rdx
    long long v12;  // rdx
    long long v13;  // r8
    long long v14;  // rdx
    long long v15;  // rdi
    long long v0;  // [bp-0x60], Other Possible Types: unsigned long
    long long i;  // [bp-0x58]
    unsigned long v2;  // [bp-0x50]
    long long v3;  // [bp-0x48]
    long long j;  // [bp-0x48]

    v6 = a0;
    v7 = sub_40f1a0();
    if (!v7)
        return;
    v8 = v7;
    v9 = -(g_426288);
    v10 = v0 - g_426570;
    if (v10 < v9)
        v10 = v9;
    v11 = v2 - g_426570;
    v0 = v10;
    if (v11 >= v9)
        v9 = v11;
    v2 = v9;
    v12 = g_426270 - 1;
    v13 = g_426270 - g_426570;
    if (i < v13)
        v12 = g_426570 + i;
    i = v12;
    v14 = g_4263a0 - 1;
    v15 = g_4263a0 - g_426570;
    if (v3 < v15)
        v14 = g_426570 + v3;
    j = v14;
    if (g_426520)
    {
        v16 = sub_407c80(*((long long *)&g_426258), v10);
        sub_40e2b0(v17, v18, v19, v20, v21, v22);
        v23 = g_426188;
        fputs_unlocked("***************", g_426188);
        if (v16)
            sub_407d30(g_426188, v16);
    }
    else
    {
        sub_40e2b0(v15, v10, v14, v3, v13);
        v23 = g_426188;
        fputs_unlocked("***************", g_426188);
    }
    putc_unlocked(10, v23);
    sub_40eb70(4);
    fputs_unlocked("*** ", v23);
    sub_407e00();
    fputs_unlocked(" ****", v23);
    sub_40eb70(3);
    putc_unlocked(10, v23);
    if ((char)v8 & 1)
    {
        k = v0;
        if (k <= i)
        {
            v25 = v6;
            do
            {
                sub_40eb70(2);
                if (v25)
                {
                    while (v25->field_10 + v25->padding_18 <= k)
                    {
                        if (!v25->field_0)
                            goto LABEL_408240;
                    }
                    if (v25->padding_18 <= k)
                    {
                        v26 = "-";
                        if (NULL < v25->field_8)
                        {
                            v26 = "!";
                            goto LABEL_408084;
                        }
                        else
                        {
                            goto LABEL_408084;
                        }
                    }
                }
LABEL_408240:
                v26 = " ";
LABEL_408084:
                v27 = k * 8;
                sub_40ef20(v26, *((long long *)&g_426258) + v27, 1);
                sub_40eb70(3);
                if (*((char *)(*((long long *)(*((long long *)&g_426258) + v27 + 8)) - 1)) == 10)
                    putc_unlocked(10, v23);
                k += 1;
            } while (i >= k);
        }
    }
    sub_40eb70(4);
    fputs_unlocked("--- ", v23);
    sub_407e00();
    fputs_unlocked(" ----", v23);
    sub_40eb70(3);
    putc_unlocked(10, v23);
    if (!((char)v8 & 2))
        return;
    l = v2;
    if (l > j)
        return;
    do
    {
        sub_40eb70(1);
        if (v6)
        {
            while (v6->field_8 + v6->field_20 <= l)
            {
                if (!v6->field_0)
                    goto LABEL_408230;
            }
            if (v6->field_20 <= l)
            {
                v29 = "+";
                if (NULL < v6->field_10)
                {
                    v29 = "!";
                    goto LABEL_40819d;
                }
                else
                {
                    goto LABEL_40819d;
                }
            }
        }
LABEL_408230:
        v29 = " ";
LABEL_40819d:
        v30 = l * 8;
        sub_40ef20(v29, g_426388 + v30, 1);
        sub_40eb70(3);
        if (*((char *)(*((long long *)(g_426388 + v30 + 8)) - 1)) == 10)
            putc_unlocked(10, v23);
        l += 1;
    } while (j >= l);
    return;
}



// Function: print_unidiff_number_range @ 0x8280
extern unsigned long long g_426188;

void print_unidiff_number_range(void)
{
    char *v3;  // rdx
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_40f0c0();
    if (v1 > v0)
    {
        __fprintf_chk(g_426188, 0x1, "%ld,%ld", v0, v1 - v0 + 1);
        return;
    }
    v3 = "%ld,0";
    if (v1 >= v0)
        v3 = "%ld";
    __fprintf_chk(g_426188, 0x1, v3);
    return;
}



// Function: pr_unidiff_hunk @ 0x8320
typedef struct struct_2 {
    struct struct_0 *field_0;
    void* field_8;
} struct_2;

typedef struct struct_1 {
    struct struct_1 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_426188;
extern unsigned int g_426258;
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_426388;
extern unsigned long long g_4263a0;
extern char g_426491;
extern char g_426492;
extern unsigned long long g_426520;
extern unsigned long long g_426570;

void pr_unidiff_hunk(struct_1 *idx)
{
    long long v8;  // rdx
    unsigned long v9;  // rcx
    long long v18;  // rsi
    long long v19;  // rdx
    long long v20;  // rcx
    long long v21;  // r8
    long long v22;  // r9
    unsigned long v23;  // rbx
    struct_0 **v24;  // r12
    unsigned long long v25;  // rbp
    unsigned long long iter;  // r15
    unsigned long long v27;  // rbp
    long long v10;  // rsi
    struct_2 *v28;  // r12
    unsigned long long v29;  // rbp
    unsigned long long node;  // r15
    unsigned long long v31;  // rbp
    struct_2 *v32;  // r12
    long long v11;  // rdx
    long long v12;  // rdx
    long long v13;  // r8
    long long v14;  // rdx
    long long v15;  // rdi
    unsigned long long v16;  // rbp
    long long v17;  // rdi
    unsigned long v0;  // [bp-0x78]
    unsigned long v1;  // [bp-0x70]
    long long v2;  // [bp-0x60], Other Possible Types: unsigned long
    long long v3;  // [bp-0x58]
    unsigned long v4;  // [bp-0x50]
    long long v5;  // [bp-0x48]
    long long v6;  // [bp-0x48]

    if (!sub_40f1a0())
        return;
    v8 = v2 - g_426570;
    v9 = -(g_426288);
    if (v8 < v9)
        v8 = v9;
    v2 = v8;
    v10 = v8;
    v11 = v4 - g_426570;
    if (v11 >= v9)
        v9 = v11;
    v4 = v9;
    v12 = g_426270 - 1;
    v13 = g_426270 - g_426570;
    if (v3 < v13)
        v12 = g_426570 + v3;
    v3 = v12;
    v14 = g_4263a0 - 1;
    v15 = g_4263a0 - g_426570;
    if (v5 < v15)
        v14 = g_426570 + v5;
    v6 = v14;
    if (g_426520)
    {
        v16 = sub_407c80(*((long long *)&g_426258), v10);
        sub_40e2b0(v17, v18, v19, v20, v21, v22);
        v23 = g_426188;
        sub_40eb70(4);
        fputs_unlocked("@@ -", g_426188);
        sub_408280();
        fputs_unlocked(" +", g_426188);
        sub_408280();
        fputs_unlocked(" @@", g_426188);
        sub_40eb70(3);
        if (v16)
            sub_407d30(g_426188, v16);
    }
    else
    {
        sub_40e2b0(v15, v10, v14, v5, v13);
        v23 = g_426188;
        sub_40eb70(4);
        fputs_unlocked("@@ -", g_426188);
        sub_408280();
        fputs_unlocked(" +", g_426188);
        sub_408280();
        fputs_unlocked(" @@", g_426188);
        sub_40eb70(3);
    }
    putc_unlocked(10, v23);
    for (v0 = v4; v3 >= v2 || v6 >= v0; idx = idx->field_0)
    {
        while (!idx || idx[1].field_0 > v2)
        {
            v24 = *((long long *)&g_426258) + v2 * 8;
            if (!g_426491 || *(v24)->field_0 != 10)
                putc_unlocked((-(g_426492 < 1) & 23) + 9, v23);
            v2 += 1;
            sub_40f0a0(0, v24);
            v0 += 1;
            if (v3 < v2 && v6 < v0)
                return;
        }
        v25 = idx->field_10;
        v1 = v25 - 1;
        if (v25)
        {
            iter = v2 * 8;
            while (1)
            {
                v27 = v25;
                sub_40eb70(2);
                putc_unlocked(45, v23);
                v28 = *((long long *)&g_426258) + iter;
                if (g_426492 && (!g_426491 || v28->field_0->field_0 != 10))
                    putc_unlocked(9, v23);
                sub_40ef20(0, v28, 1);
                sub_40eb70(3);
                if (*((char *)v28->field_8 - 1) != 10)
                {
                    iter += 8;
                    v25 = v27 - 1;
                    if (v27 == 1)
                        break;
                }
                else
                {
                    iter += 8;
                    putc_unlocked(10, v23);
                    v25 = v27 - 1;
                    if (v27 == 1)
                        break;
                }
            }
            v2 = v2 + v1 + 1;
        }
        v29 = idx->field_8;
        v1 = v29 - 1;
        if (v29)
        {
            node = v0 * 8;
            while (1)
            {
                v31 = v29;
                sub_40eb70(1);
                putc_unlocked(43, v23);
                v32 = g_426388 + node;
                if (g_426492 && (!g_426491 || v32->field_0->field_0 != 10))
                    putc_unlocked(9, v23);
                sub_40ef20(0, v32, 1);
                sub_40eb70(3);
                if (*((char *)v32->field_8 - 1) != 10)
                {
                    node += 8;
                    v29 = v31 - 1;
                    if (v31 == 1)
                        break;
                }
                else
                {
                    node += 8;
                    putc_unlocked(10, v23);
                    v29 = v31 - 1;
                    if (v31 == 1)
                        break;
                }
            }
            v0 = v0 + v1 + 1;
        }
    }
    return;
}



// Function: find_hunk @ 0x8770
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char field_28;
} struct_0;

extern long long g_426570;

struct_0 * find_hunk(struct_0 *idx)
{
    long long v1;  // r11
    unsigned long v2;  // rdx
    unsigned long v3;  // rax
    struct_0 *v4;  // r8
    unsigned long v5;  // rsi
    long long v6;  // r9
    long long v7;  // rcx
    unsigned long v8;  // rdx

    v1 = g_426570;
    v2 = idx->field_18;
    v3 = idx->field_20;
    while (1)
    {
        v4 = idx;
        idx = idx->field_0;
        if (!idx)
            return v4;
        v5 = idx->field_18;
        v6 = g_426570 * 2 + 1;
        if (idx->field_28)
            v6 = v1;
        v7 = idx->field_18 - (v2 + v4->field_10);
        v8 = idx->field_20;
        if (v7 != idx->field_20 - (v3 + v4->field_8))
        {
            abort(); /* do not return */
        }
        else if (v7 < v6)
        {
            v2 = v5;
            v3 = v8;
        }
        else
        {
            return v4;
        }
    }
}



// Function: print_context_header @ 0x87e0
extern char *g_426540;
extern char *g_426548;

int print_context_header(unsigned long long a0, char **a1, char a2)
{
    unsigned long long v1;  // rbp
    unsigned long long v2;  // rsi

    v1 = a0 + 304;
    v2 = a0;
    if (!a2)
    {
        sub_407ab0("***", a0, *(a1), g_426540);
        return sub_407ab0("---", v1, a1[1], g_426548);
    }
    sub_407ab0("---", a0, *(a1), g_426540);
    return sub_407ab0("+++", v1, a1[1], g_426548);
}



// Function: print_context_script @ 0x8870
typedef struct struct_0 {
    struct struct_1 *field_0;
    char padding_8[32];
    char field_28;
} struct_0;

typedef struct struct_1 {
    struct struct_0 *field_0;
    char padding_8[32];
    char field_28;
} struct_1;

extern unsigned long long g_426170;
extern unsigned long long g_426178;
extern unsigned long long g_426288;
extern unsigned long long g_4264e0;
extern char g_426554;

long long print_context_script(struct_1 *a0, char a1)
{
    struct_1 *v1;  // rax
    struct_1 *v2;  // rax

    if (g_426554 || g_4264e0)
    {
        sub_407be0(a0);
    }
    else
    {
        v1 = a0;
        if (a0)
        {
            do
            {
                v1->field_28 = 0;
                v2 = v1->field_0;
            } while (v2 && (v2->field_28 = 0, v1 = (struct_1 *)v2->field_0, v2->field_0));
        }
    }
    g_426178 = -(g_426288);
    g_426170 = 0x7fffffffffffffff;
    return sub_40eb10(a0, sub_408770);
}



// Function: exclude_options @ 0x8910
extern char g_426551;

long long exclude_options(void)
{
    return (-(g_426551 < 1) & 0xfffffff0) + 268435472;
}



// Function: add_regexp @ 0x8930
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    long long field_20;
} struct_0;

long long add_regexp(struct_0 *idx, unsigned long a1)
{
    unsigned long len;  // rbp
    long long v2;  // r14
    unsigned long long v3;  // r13
    unsigned long v4;  // rsi
    long long v5;  // rax
    unsigned long i;  // rdx
    unsigned long v7;  // r13

    len = strlen(a1);
    if (re_compile_pattern(a1, len, idx->field_20))
        return (unsigned long long)error(2, 0, "%s: %s");
    v2 = idx->field_0;
    v3 = idx->field_8;
    v4 = idx->field_10;
    idx->field_18 = v2;
    v5 = v2;
    i = ((v2) * 2 & 2) + v3 + len;
    idx->field_8 = i;
    if (i >= v4)
    {
        if (!v4)
            v4 = 1;
        do
        {
            v4 *= 2;
        } while (i >= v4);
        idx->field_10 = v4;
        v5 = sub_415fa0(v2);
        idx->field_0 = v5;
    }
    if (v2)
    {
        *((char *)(v5 + v3)) = 92;
        v7 = v3 + 2;
        *((char *)(v5 + v3 + 1)) = 124;
        v3 = v7;
    }
    return memcpy(v5 + v3, a1, len + 1);
}



// Function: usage @ 0x8a10
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern char g_41b135;
extern struct_0 *g_424f20;
extern FILE *stdout;

int usage(void)
{
    unsigned long long v5;  // r14
    unsigned long long v6;  // r13
    char *v15;  // rax
    char *ptr;  // rax
    char *v17;  // rbx
    unsigned long long v7;  // r12
    unsigned long long v8;  // rbx
    struct_0 **iter;  // r12
    char *v12;  // rax
    char v13;  // al
    char *v14;  // rsi
    unsigned long long v0;  // [bp-0x28]
    unsigned long long v1;  // [bp-0x18]
    unsigned long long v2;  // [bp-0x10]
    unsigned long long v3;  // [bp-0x8]

    v3 = v5;
    v2 = v6;
    v1 = v7;
    v0 = v8;
    iter = &g_424f20;
    __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILES\n", 5));
    __printf_chk(1, "%s\n\n", dcgettext(NULL, "Compare FILES line by line.", 5));
    v12 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v12, stdout);
    v13 = 32;
    v14 = "    --normal                  output a normal diff (the default)";
    while (1)
    {
        if (!v13)
        {
            iter += 1;
            putchar_unlocked(10);
            v14 = &*(iter)->field_0;
            if (!*(iter))
                break;
        }
        else
        {
            v15 = dcgettext(NULL, v14, 5);
            while (1)
            {
                ptr = strchr(v15, 10);
                if (!ptr)
                    break;
                v17 = ptr + 1;
                if ((unsigned int)(v17 - v15) > 0xfff)
                    __assert_fail(); /* do not return */
                __printf_chk(1, "  %.*s");
                v15 = v17;
            }
            iter += 1;
            __printf_chk(1, &(&g_41b135)[2 * (*(v15) != 45 & _INSERT(0, 0, *(v15) != 32))]);
            v14 = &*(iter)->field_0;
            if (!*(iter))
                break;
        }
        v13 = *(v14);
    }
    fputs_unlocked("\n", stdout);
    __printf_chk(1, dcgettext(NULL, "Report bugs to: %s\n", 5));
    __printf_chk(1, dcgettext(NULL, "%s home page: <%s>\n", 5));
    return __printf_chk(1, dcgettext(NULL, "General help using GNU software: <%s>\n", 5));
}



// Function: summarize_regexp_list @ 0x8ba0
typedef struct struct_0 {
    char padding_0[32];
    unsigned long long field_20;
} struct_0;

typedef struct struct_1 {
    long long field_0;
    long long field_8;
    char padding_10[8];
    char field_18;
    char padding_19[7];
    struct struct_0 *field_20;
} struct_1;

long long summarize_regexp_list(struct_1 *a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rax
    unsigned long v4;  // 4115
    unsigned long v5;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!a0->field_0)
        return v5;
    v0 = v2;
    v3 = sub_415f50(0x100);
    v4 = a0->field_18;
    a0->field_20->field_20 = v3;
    if (!(char)v4)
        return a2;
    if (!re_compile_pattern(a0->field_0, a0->field_8, a0->field_20))
        return a2;
    error(2, 0, "%s: %s");
}



// Function: try_help @ 0x8c00
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: specify_value @ 0x8c60
int specify_value(char **a0, char *a1, unsigned long a2)
{
    unsigned int v1;  // eax

    if (*(a0))
    {
        v1 = strcmp(*(a0), a1);
        if (v1)
        {
            error(0, 0, dcgettext(NULL, "conflicting %s option value '%s'", 5));
            sub_408c00(NULL, 0); /* do not return */
        }
    }
    *(a0) = a1;
    return v1;
}



// Function: specify_colors_style @ 0x8cc0
extern unsigned int g_42657c;

unsigned int specify_colors_style(char *a0)
{
    if (!a0)
    {
        g_42657c = 1;
        return 1;
    }
    else if (!strcmp(a0, "auto"))
    {
        g_42657c = 1;
        return 1;
    }
    else if (!strcmp(a0, "always"))
    {
        g_42657c = 2;
        return 2;
    }
    else if (!strcmp(a0, "never"))
    {
        g_42657c = 0;
        return 0;
    }
    else
    {
        sub_408c00("invalid color '%s'", a0); /* do not return */
    }
}



// Function: specify_style @ 0x8d50
extern unsigned int g_426580;

long long specify_style(unsigned int a0)
{
    unsigned long v2;  // rax
    unsigned long v0;  // [bp-0x8]

    v2 = g_426580;
    if ((unsigned int)v2 == a0)
    {
        return v2;
    }
    else if (!(unsigned int)v2)
    {
        g_426580 = a0;
        return v2;
    }
    else
    {
        v0 = v2;
        sub_408c00("conflicting output style options", 0); /* do not return */
    }
}



// Function: option_list @ 0x8d80
char * option_list(long long *j, unsigned int a1)
{
    long long v1;  // r12
    long long *v2;  // r13
    long long *i;  // rbp
    long long *v4;  // rbp
    char *v5;  // rax
    char *v6;  // r12
    long long *v7;  // rbx

    if (a1 > 0)
    {
        v1 = 1;
        v2 = &j[a1];
        i = j;
        do
        {
            v4 = i + 1;
            v1 = v1 + sub_4144e0(*(i)) + 1;
            i = v4;
        } while (i != v2);
        v5 = sub_415f50(v1);
        v6 = v5;
        do
        {
            *(v5) = 32;
            v7 = j + 1;
            v5 = sub_414530(v5 + 1, *(j));
            j = v7;
        } while (j != v2);
    }
    else
    {
        v5 = sub_415f50(1);
        v6 = v5;
    }
    *(v5) = 0;
    return v6;
}



// Function: set_mtime_to_now @ 0x8e10
void set_mtime_to_now(long long a0)
{
    sub_410a50(a0 + 88);
    return;
}



// Function: check_stdout @ 0x8e20
typedef struct FILE {
} FILE;

extern FILE *stdout;

unsigned long long check_stdout(void)
{
    unsigned long long v1;  // rax

    if (ferror_unlocked(stdout))
        sub_40e230("write failed"); /* do not return */
    v1 = fclose(stdout);
    if ((unsigned int)v1)
        sub_40e1f0(dcgettext(NULL, "standard output", 5)); /* do not return */
    return v1;
}



// Function: compare_files @ 0x8e70
typedef struct struct_1 {
    char field_0;
    char padding_1[303];
    unsigned int field_130;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[36];
    unsigned int field_28;
    char padding_2c[20];
    unsigned long long field_40;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_426550;
extern char g_426553;
extern char g_426578;
extern unsigned int g_426580;
extern char g_426584;
extern char g_426585;
extern char g_426586;
extern char g_426608;

unsigned long long compare_files(long long *a0, unsigned long a1, unsigned long a2)
{
    unsigned long v12;  // r13
    unsigned long v13;  // r12
    unsigned long v22;  // rax
    int v23;  // r13d
    void* iter;  // r15
    void* v25;  // rsi
    long long v26;  // rax
    long long v27;  // rax
    struct_0 *node;  // rax
    struct_1 *v29;  // rsi
    unsigned int v30;  // r15d
    unsigned int v31;  // edx
    char v14;  // dl
    unsigned int v32;  // esi
    unsigned int v33;  // r9d
    int v34;  // edx
    unsigned long long v35;  // rcx
    void* iter1;  // rdi
    int *err;  // rbx
    int v38;  // edx
    unsigned long long v39;  // rcx
    unsigned long long v40;  // rcx
    void* iter2;  // rdi
    unsigned long long v15;  // rcx
    unsigned int v42;  // eax
    unsigned long v43;  // rax
    unsigned long long index;  // rax
    long long v45;  // rax
    unsigned long long v46;  // rax
    unsigned int v48;  // edx
    unsigned int v49;  // r10d
    unsigned int v50;  // edi
    unsigned int v51;  // eax
    unsigned long long *j;  // rdi
    unsigned int v52;  // r9d
    unsigned int v53;  // ecx
    char v54;  // dil
    char v55;  // dl
    unsigned int v56;  // eax
    unsigned int v57;  // ebx
    unsigned int v58;  // eax
    unsigned int v59;  // edi
    unsigned int v60;  // ebx
    unsigned int v61;  // edi
    unsigned long long v17;  // rcx
    unsigned int v62;  // edi
    void* ptr;  // rbp
    void* ptr1;  // r12
    unsigned long long *i;  // rdi
    unsigned long long v19;  // rcx
    unsigned long long *k;  // rdi
    unsigned long v21;  // r14
    unsigned long v1;  // [bp-0x2c8]
    unsigned int v2;  // [bp-0x2c0], Other Possible Types: unsigned long, unsigned long long, int
    void* l;  // [bp-0x2b8]
    unsigned long v4;  // [bp-0x2b0]
    int v5;  // [bp-0x2a8], Other Possible Types: char
    uint128_t v6;  // [bp-0x118]
    uint128_t v7;  // [bp-0x108]
    int v8;  // [bp-0xf8]
    int v9;  // [bp-0xe8]
    long long *v10;  // [bp-0x48]

    v12 = a2;
    v13 = a1;
    v14 = !a2;
    if (a1 && !v14)
    {
        v15 = 76;
        v10 = a0;
        for (j = &v5; v15; j += 1)
        {
            v15 -= 1;
            *(j) = 0;
        }
        *((unsigned int *)&v5) = 0xfffffffe;
        goto LABEL_408f42;
    }
    if (g_426585 == 1 && !v14)
    {
        v17 = 76;
        v10 = a0;
        for (i = &v5; v17; i += 1)
        {
            v17 -= 1;
            *(i) = 0;
        }
        if (v13)
        {
            *((unsigned int *)&v5) = 0xfffffffe;
            *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
LABEL_408f00:
            v13 = v12;
            if (a0)
                goto LABEL_408f52;
            goto LABEL_408f08;
        }
        else
        {
            *((unsigned int *)&v5) = 0xffffffff;
        }
LABEL_409578:
        *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
        if (v12)
            goto LABEL_408f00;
        goto LABEL_409560;
    }
    else
    {
        if (!g_426586)
        {
            sub_40e0e0();
            return 1;
        }
        v19 = 76;
        v10 = a0;
        for (k = &v5; v19; k += 1)
        {
            v19 -= 1;
            *(k) = 0;
        }
        if (v13)
        {
            *((unsigned int *)&v5) = 0xfffffffe;
            if (v12)
            {
LABEL_408f42:
                *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
            }
            else
            {
                *((unsigned int *)&(&v5)[304]) = 0xffffffff;
                v12 = v13;
            }
        }
        else
        {
            *((unsigned int *)&v5) = 0xffffffff;
            if (v12)
                goto LABEL_409578;
            *((unsigned int *)&(&v5)[304]) = 0xffffffff;
LABEL_409560:
            v13 = 0;
        }
        if (a0)
        {
LABEL_408f52:
            v22 = sub_410920(a0[1], v13, 0);
            *((unsigned long *)&(&v5)[8]) = v22;
            v21 = v22;
            v1 = sub_410920(a0[39], v12, 0);
            v12 = v1;
LABEL_408f80:
            *((unsigned long *)&(&v5)[312]) = v12;
            v23 = 0;
            l = &v5 - 16;
            iter = &v5 - 16;
            while (1)
            {
                if (*((int *)((char *)iter - 16)) != 0xffffffff)
                {
                    if (v23 && !strcmp(*((unsigned long long *)(&v5 + 312)), *((unsigned long long *)(&v5 + 8))))
                    {
                        memcpy(&v5 - 320, &v5 - 16, 16);
                        *((unsigned int *)&(&v5)[304]) = *((unsigned int *)&v5);
                        *((uint128_t *)&(&v5)[336]) = *((uint128_t *)(&v5 + 32));
                        *((uint128_t *)&(&v5)[352]) = *((uint128_t *)(&v5 + 48));
                        *((uint128_t *)&(&v5)[368]) = *((uint128_t *)(&v5 + 64));
                        *((uint128_t *)&(&v5)[384]) = *((uint128_t *)(&v5 + 80));
                        v6 = *((uint128_t *)(&v5 + 96));
                        v7 = *((uint128_t *)(&v5 + 112));
                        v8 = (int)*((uint128_t *)(&v5 + 128));
                        v9 = (int)*((uint128_t *)(&v5 + 144));
                    }
                    else
                    {
                        v2 = *((long long *)((char *)iter - 8));
                        if (strcmp(*((long long *)((char *)iter - 8)), "-"))
                        {
                            v25 = iter;
                            if (g_426550)
                            {
                                if (lstat(v2, v25))
                                    goto LABEL_40926f;
                            }
                            else
                            {
                                if (stat(v2, v25))
                                    goto LABEL_40926f;
                            }
                        }
                        else
                        {
                            *((unsigned int *)((char *)iter - 16)) = 0;
                            isatty(0);
                            if (fstat(0, iter))
                            {
LABEL_40926f:
                                *((unsigned int *)((char *)iter - 16)) = 0xfffffffd - *(__errno_location());
                            }
                            else
                            {
                                if (((short)(int)iter[24] & 0xf000) == 0x8000)
                                {
                                    v26 = lseek(0, 0, 1);
                                    if (v26 >= 0)
                                    {
                                        *((long long *)&iter[48]) = (long long)iter[48] - v26;
                                        v27 = (long long)iter[48];
                                        if ((long long)iter[48] < 0)
                                            v27 = 0;
                                        *((long long *)&iter[48]) = v27;
                                    }
                                    else
                                    {
                                        *((unsigned int *)((char *)iter - 16)) = 0xfffffffd - *(__errno_location());
                                    }
                                }
                                sub_408e10(iter);
                            }
                        }
                    }
                }
                iter += 304;
                if (v23 == 1)
                    break;
                v23 = 1;
            }
            node = &v5;
            v29 = &v5;
            v30 = 0;
            while (1)
            {
                v30 += 1;
                if (!g_426586 && v30 == 1)
                {
                    if (g_426585)
                        goto LABEL_4091c0;
                    goto LABEL_40920a;
                }
LABEL_4091c0:
                v31 = node->field_0;
                if (v31 != 0xfffffffe)
                {
                    if ((v31 == 0xfffffffb || v31 == 0xfffffff4) && !a0 && !(v29->field_130 + 2 & 0xfffffffd))
                        goto LABEL_4091fa;
                    goto LABEL_40909b;
                }
                else if (((unsigned short)node->field_28 & 0xf000) != 0x8000 || (unsigned short)node->field_28 & 0x1ff || node->field_40)
                {
LABEL_40909b:
                    if (v30 == 2)
                        break;
                    else
                        goto LABEL_40920a;
                }
                else
                {
LABEL_4091fa:
                    node->field_0 = 0xffffffff;
                    if (v30 == 2)
                        break;
LABEL_40920a:
                    node = &node[4].padding_4[12];
                    v29 = (char *)v29 - 304;
                }
            }
            v32 = *((unsigned int *)&v5);
            v33 = *((unsigned int *)(&v5 + 304));
            if (v32 != 0xffffffff)
            {
                v34 = 0xfffffffd - v32;
                if (v33 != 0xffffffff)
                {
                    if (v34 < 0)
                        goto LABEL_4092b4;
                    goto LABEL_4090d3;
                }
                v35 = 18;
                for (iter1 = &v5 - 320; v35; iter1 += 8)
                {
                    v35 -= 1;
                    *((unsigned long long *)iter1) = 0;
                }
                *((unsigned int *)&(&v5)[344]) = *((unsigned int *)(&v5 + 40));
                if (v34 < 0)
                    goto LABEL_4092c2;
LABEL_4090d3:
                v2 = v34;
                err = __errno_location();
                *(err) = v2;
                sub_40df30(*((unsigned long long *)(&v5 + 8)));
                v38 = 0xfffffffd - *((unsigned int *)(&v5 + 304));
                if ((unsigned int)(0xfffffffd - *((unsigned int *)(&v5 + 304))) >= 0)
                    goto LABEL_4090fd;
            }
            else
            {
                for (v39 = 18; v39; l += 8)
                {
                    v39 -= 1;
                    *((unsigned long long *)l) = 0;
                }
                *((unsigned int *)&(&v5)[40]) = *((unsigned int *)(&v5 + 344));
                if (v33 != 0xffffffff)
                {
LABEL_4092b4:
                    if ((unsigned int)(0xfffffffd - v33) < 0)
                        goto LABEL_4092c2;
                    v2 = 0xfffffffd - v33;
                    v38 = v2;
                    err = __errno_location();
LABEL_4090fd:
                    *(err) = v38;
                    sub_40df30(*((unsigned long long *)(&v5 + 312)));
                }
                else
                {
                    v40 = 18;
                    for (iter2 = &v5 - 320; v40; iter2 += 8)
                    {
                        v40 -= 1;
                        *((unsigned long long *)iter2) = 0;
                    }
                    *((unsigned int *)&(&v5)[344]) = *((unsigned int *)(&v5 + 40));
LABEL_4092c2:
                    if (!a0 && (v42 = *((unsigned int *)((void*)&v5 + 40)) & 0xf000, (char)(char)(v42 == 0x4000) != (char)(char)((*((unsigned int *)((void*)&v5 + 344)) & 0xf000) == 0x4000)))
                    {
                        v43 = v42 == 0x4000;
                        index = v43 * 19;
                        v4 = *((long long *)((char *)&v5 + 16 * index - 8));
                        v45 = sub_40f670(*((long long *)((char *)&v5 + 16 * index - 8)));
                        v46 = (int)(1 - v43) * 304;
                        v2 = v46;
                        v21 = sub_40a4f0(*((long long *)((char *)&v5 + v46 - 8)), v45);
                        *((unsigned long *)((char *)&v5 + v2 - 8)) = v21;
                        if (!strcmp(v4, "-"))
                            sub_40e230("cannot compare '-' to a directory"); /* do not return */
                        v32 = *((unsigned int *)&v5);
                        if (!(!g_426550 ? stat(v21, &v5 - 16 + v2) : lstat(v21, &v5 - 16 + v2)))
                            goto LABEL_4093a2;
                        sub_40df30(v21);
                    }
                    else
                    {
LABEL_4093a2:
                        v48 = *((unsigned int *)(&v5 + 304));
                        if (v32 != 0xffffffff)
                        {
                            v49 = *((unsigned int *)(&v5 + 40));
                            v50 = *((unsigned int *)(&v5 + 344));
                            if (v48 == 0xffffffff)
                            {
LABEL_4095a0:
                                v53 = 0;
                                v51 = v49 & 0xf000;
                            }
                            else if (*((unsigned long long *)(&v5 + 24)) != *((unsigned long long *)(&v5 + 328)) || *((unsigned long long *)(&v5 + 16)) != *((unsigned long long *)(&v5 + 320)))
                            {
                                v51 = v49 & 0xf000;
                                v52 = v50 & 0xf000;
                                if (v51 != 0x6000)
                                {
                                    v53 = 0;
                                    if (v51 == 0x2000)
                                    {
                                        if (v52 == 0x2000)
                                            goto LABEL_4097ed;
                                        goto LABEL_409411;
                                    }
                                }
                                else
                                {
                                    if (v52 == 0x6000)
                                    {
LABEL_4097ed:
                                        if (*((unsigned long long *)(&v5 + 360)) == *((unsigned long long *)(&v5 + 56)))
                                            goto LABEL_40972b;
                                    }
LABEL_409411:
                                    if (v51 == 0x4000 || !(v53 = 0, v52 != 0x4000))
                                        goto LABEL_409820;
                                    goto LABEL_409437;
                                }
                            }
                            else
                            {
LABEL_40972b:
                                if (v49 != v50)
                                    goto LABEL_4095a0;
                                if (*((unsigned long long *)(&v5 + 32)) != *((unsigned long long *)(&v5 + 336)) || *((unsigned int *)(&v5 + 44)) != *((unsigned int *)(&v5 + 348)) || *((unsigned int *)(&v5 + 48)) != *((unsigned int *)(&v5 + 352)) || *((unsigned long long *)(&v5 + 64)) != *((unsigned long long *)(&v5 + 368)) || *((unsigned long long *)(&v5 + 104)) != *((unsigned long long *)((void*)&v6 + 8)) || *((unsigned long long *)(&v5 + 120)) != *((unsigned long long *)((void*)&v7 + 8)))
                                {
                                    v53 = 0;
                                    v51 = v50 & 0xf000;
                                }
                                else
                                {
                                    if (g_426578)
                                    {
LABEL_409610:
                                        if (g_426584 && ((unsigned short)*((unsigned int *)(&v5 + 40)) & 0xf000) != 0x4000)
                                            sub_40e0e0();
                                        free(v21);
                                        free(v1);
                                        return 0;
                                    }
                                    v53 = 1;
                                    v51 = v50 & 0xf000;
                                }
                            }
                        }
                        else
                        {
                            if (v48 == 0xffffffff)
                                goto LABEL_409610;
                            v50 = *((unsigned int *)(&v5 + 344));
                            v53 = 0;
                            v51 = *((unsigned int *)(&v5 + 40)) & 0xf000;
                        }
                        v52 = v50 & 0xf000;
                        v54 = v52 == 0x4000;
                        if (v51 == 0x4000)
                        {
                            if (v54)
                            {
                                if (g_426580 == 7)
                                    sub_40e230("-D option not supported with directories"); /* do not return */
                                if (!a0 || g_426608)
                                    goto LABEL_4095f7;
                                sub_40e0e0("Common subdirectories: %s and %s\n", *((unsigned long long *)(&v5 + 8)), *((unsigned long long *)(&v5 + 312)));
                                goto LABEL_409610;
                            }
LABEL_409811:
                            if (v32 == 0xffffffff)
                                goto LABEL_409829;
LABEL_409820:
                            if (v48 != 0xffffffff)
                                goto LABEL_409780;
LABEL_409829:
                            if (g_426608 && (g_426586 || g_426585 && v32 == 0xffffffff))
                            {
LABEL_4095f7:
                                v60 = sub_40a0c0(&v5, compare_files);
LABEL_409608:
                                if (!v60)
                                    goto LABEL_409610;
                                v30 = v60;
                            }
                            else
                            {
                                if (!a0)
                                    __assert_fail(); /* do not return */
LABEL_409473:
                                v30 = 1;
                                sub_40e0e0("Only in %s: %s\n", a0[1 + 38 * (v32 == 0xffffffff)], v13);
                            }
                        }
                        else
                        {
                            if (v54)
                                goto LABEL_409811;
LABEL_409437:
                            if (a0 && ((v51 & 0xffffdfff) != 0x8000 || (v52 & 0xffffdfff) != 0x8000))
                            {
                                if (v32 == 0xffffffff || v48 == 0xffffffff)
                                    goto LABEL_409473;
                                goto LABEL_409780;
                            }
                            v55 = v52 == 0xa000;
                            if (!(v51 != 0xa000 && !v55))
                            {
                                if (!g_426550)
                                    __assert_fail(); /* do not return */
                                if (v51 != 0xa000 || !v55)
                                {
LABEL_409780:
                                    v30 = 1;
                                    sub_410830(&v5 - 320);
                                    sub_410830();
                                    sub_40df60();
                                }
                                else
                                {
                                    ptr = sub_416360(*((unsigned long long *)(&v5 + 8)));
                                    if (ptr)
                                    {
                                        ptr1 = sub_416360(*((unsigned long long *)(&v5 + 312)));
                                        if (!ptr1)
                                            goto LABEL_409c4e;
                                        if (!strcmp(ptr, ptr1))
                                        {
                                            free(ptr);
                                            free(ptr1);
                                            goto LABEL_409610;
                                        }
                                        else
                                        {
                                            v30 = 1;
                                            sub_40e0e0("Symbolic links %s and %s differ\n", *((unsigned long long *)(&v5 + 8)), *((unsigned long long *)(&v5 + 312)));
                                            free(ptr);
                                            free(ptr1);
                                        }
                                    }
                                    else
                                    {
                                        v23 = 0;
LABEL_409c4e:
                                        sub_40df30(*((long long *)((char *)&v5 + 304 * v23 - 8)));
                                        free(ptr);
                                        free(NULL);
                                    }
                                }
                            }
                            else if (!(!(v51 == 0x8000 & g_426553) || v52 != 0x8000 || !(*((unsigned long long *)(&v5 + 64)) > 0 & *((unsigned long long *)(&v5 + 64)) != *((unsigned long long *)(&v5 + 368))) || *((unsigned long long *)(&v5 + 368)) <= 0))
                            {
                                v30 = 1;
                                sub_40e0e0("Files %s and %s differ\n");
                            }
                            else if (v32 != 0xfffffffe || !(v2 = v53, v56 = (unsigned int)(int)(long long)open(*((unsigned long long *)((void*)&v5 + 8)), 0, 0), v53 = v2, *((unsigned int *)&v5) = v56, v56 < 0))
                            {
                                v57 = 0;
                                if (*((unsigned int *)(&v5 + 304)) != 0xfffffffe)
                                    goto LABEL_409a0f;
                                goto LABEL_4098f7;
                            }
                            else
                            {
                                sub_40df30(*((unsigned long long *)(&v5 + 8)));
                                v53 = v2;
                                if (*((unsigned int *)(&v5 + 304)) != 0xfffffffe)
                                {
LABEL_409912:
                                    v59 = *((unsigned int *)&v5);
                                    if (v59 < 0)
                                    {
                                        v61 = *((unsigned int *)(&v5 + 304));
                                        if (v61 >= 0)
                                            goto LABEL_40992d;
                                    }
                                    else
                                    {
                                        v60 = 2;
LABEL_409a55:
                                        if (!close(v59))
                                        {
LABEL_409a21:
                                            v62 = *((unsigned int *)(&v5 + 304));
                                            if (v62 < 0 || v62 == *((unsigned int *)&v5) || !close(v62))
                                                goto LABEL_409608;
LABEL_40993a:
                                            sub_40df30(*((unsigned long long *)(&v5 + 312)));
                                        }
                                        else
                                        {
                                            sub_40df30(*((unsigned long long *)(&v5 + 8)));
                                            v61 = *((unsigned int *)(&v5 + 304));
                                            if (v61 >= 0 && v61 != *((unsigned int *)&v5))
                                            {
LABEL_40992d:
                                                if (close(v61))
                                                    goto LABEL_40993a;
                                            }
                                        }
                                    }
                                }
                                else
                                {
                                    v57 = 2;
LABEL_4098f7:
                                    if (v53)
                                    {
                                        *((unsigned int *)&(&v5)[304]) = *((unsigned int *)&v5);
                                        goto LABEL_40990a;
                                    }
                                    else
                                    {
                                        v58 = open(*((unsigned long long *)(&v5 + 312)), 0, 0);
                                        *((unsigned int *)&(&v5)[304]) = v58;
                                        if (v58 >= 0)
                                        {
LABEL_40990a:
                                            if (!v57)
                                            {
LABEL_409a0f:
                                                v59 = *((unsigned int *)&v5);
                                                v60 = sub_407460(&v5);
                                                if (v59 < 0)
                                                    goto LABEL_409a21;
                                                goto LABEL_409a55;
                                            }
                                        }
                                        else
                                        {
                                            sub_40df30(*((unsigned long long *)(&v5 + 312)));
                                            goto LABEL_409912;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if (fflush_unlocked(stdout))
                sub_40e1f0(dcgettext(NULL, "standard output", 5)); /* do not return */
            free(v21);
            free(v1);
            return v30;
        }
LABEL_408f08:
        *((unsigned long *)&(&v5)[8]) = v13;
        v21 = 0;
        v1 = 0;
        goto LABEL_408f80;
    }
}



// Function: dir_loop @ 0x9cd0
typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned int field_28;
    char padding_2c[12];
    unsigned long long field_38;
    char padding_40[544];
    struct struct_0 *field_260;
} struct_1;

typedef struct struct_0 {
    char padding_0[608];
    struct struct_0 *field_260;
} struct_0;

struct_0 * dir_loop(struct_1 *a0, int a1)
{
    struct_0 *iter;  // rax
    unsigned long long v2;  // rcx
    struct_1 *v3;  // rsi
    unsigned long long v4;  // rdx

    iter = a0->field_260;
    if (!a0->field_260)
        return NULL;
    v2 = a1 * 304;
    v3 = &a0->padding_0[v2];
    while (1)
    {
        v4 = &iter->padding_0[v2];
        if (*((long long *)(v4 + 24)) == v3->field_18 && *((long long *)(v4 + 16)) == v3->field_10)
            return 0x1;
        switch (*((int *)&iter->padding_0[40 + v2]) & 0xf000)
        {
        case 24576:
            if (((unsigned short)v3->field_28 & 0xf000) == 0x6000)
            {
                if (*((long long *)&iter->padding_0[56 + v2]) == v3->field_38)
                    return 0x1;
                break;
            }
            break;
        case 8192:
            if ((v3->field_28 & 0xf000) == 0x2000)
                goto LABEL_409d81;
            else
                goto LABEL_409d1a;
        default:
LABEL_409d1a:
            iter = iter->field_260;
            if (!iter)
                return iter;
            continue;
        }
LABEL_409d81:
        if (*((long long *)&iter->padding_0[56 + v2]) == v3->field_38)
            return 0x1;
        iter = iter->field_260;
        if (!iter)
            return iter;
    }
}



// Function: compare_collated @ 0x9da0
extern char g_426551;

int compare_collated(char *a0, char *a1)
{
    unsigned int *err;  // rax
    unsigned long v2;  // 4112

    err = __errno_location();
    v2 = g_426551;
    *(err) = 0;
    if (!(char)v2)
        return strcoll(a0, a1);
    return strcasecmp(a0, a1);
}



// Function: compare_names_for_qsort @ 0x9df0
extern char g_4266e8;

int compare_names_for_qsort(char **a0, char **a1)
{
    int v2;  // eax
    char v0[8];  // [bp+0x0]

    if (g_4266e8)
    {
        v2 = sub_409da0(*(a0), *(a1));
        if (v2)
            return v2;
    }
    return strcmp(v0, *(a1));
}



// Function: compare_names @ 0x9e40
extern char g_426551;
extern char g_4266e8;

int compare_names(char *a0, char *a1)
{
    int v1;  // eax

    if (!g_4266e8)
        return strcmp(a0, a1);
    v1 = sub_409da0(a0, a1);
    if (v1)
    {
        return v1;
    }
    else if (!g_426551)
    {
        return strcmp(a0, a1);
    }
    else
    {
        return v1;
    }
}



// Function: dir_read @ 0x9e90
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
} struct_0;

extern long long g_426410;

unsigned long long dir_read(struct_0 *a0, void* idx)
{
    unsigned long v6;  // 4143
    unsigned long long v7;  // r14
    void* v16;  // rax
    void* v17;  // rbx
    long long v18;  // rbp
    void* iter;  // rbx
    void* v20;  // rax
    unsigned long v8;  // rax
    unsigned long v9;  // r15
    dirent *count;  // rbx
    dirent *ptr;  // r13
    unsigned long v12;  // rax
    unsigned long long v13;  // rbx
    unsigned long v14;  // rax
    long long v15;  // r12
    unsigned long len;  // [bp-0x60]
    DIR *ptr1;  // [bp-0x58]
    unsigned int *err;  // [bp-0x50]
    unsigned long long v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x40]

    v6 = a0->field_0;
    *((unsigned long long **)&idx[8]) = NULL;
    *((unsigned long *)&idx[16]) = 0;
    if ((unsigned int)v6 != 0xffffffff)
    {
        ptr1 = opendir(a0->field_8);
        if (!ptr1)
            return 0;
        v7 = 0x200;
        v8 = sub_415f50(0x200);
        *((unsigned long *)&idx[16]) = v8;
        v9 = v8;
        v3 = 0;
        err = __errno_location();
        v4 = 0;
        while (1)
        {
            *(err) = 0;
            count = readdir(ptr1);
            if (!count)
                break;
            ptr = (char *)&count[1].d_ino + 6;
            len = strlen(ptr);
            if ((*((char *)&count[1].d_ino + 6) != 46 || *((char *)&count[1].d_ino + 7) && (*((char *)&count[1].d_ino + 7) != 46 || (char)count[1].d_reclen)) && !(char)sub_410220(g_426410, ptr))
            {
                v12 = len + 1;
                v13 = v3 + v12;
                len = v12;
                if (v7 < v13)
                {
                    do
                    {
                        if (v7 > 0x3ffffffffffffffe)
                            sub_416300(); /* do not return */
                    } while ((v7 *= 2, v14 = (unsigned long)(unsigned long long)sub_415fa0(v9, v7), *((unsigned long *)&idx[16]) = v14, v9 = v14, v7 < v13));
                }
                memcpy(v3 + v9, ptr, len);
                v4 += 1;
                v3 = v13;
            }
        }
        if (*(err))
        {
            closedir(ptr1);
            *(err) = *(err);
            return 0;
        }
        if (closedir(ptr1))
            return 0;
        if (v4 > 0xffffffffffffffd)
            sub_416300(); /* do not return */
        v15 = v4 * 8 + 8;
        v16 = sub_415f50(v15);
        *((unsigned long long *)idx) = v4;
        *((void* *)&idx[8]) = v16;
        v17 = v16;
        v18 = v15 - 8;
        if (v4)
        {
            iter = v17;
            do
            {
                *((unsigned long *)iter) = v9;
                iter += 8;
                v9 = v9 + strlen(v9) + 1;
            } while (iter != v18 + v17);
        }
        else
        {
            iter = v17 + v18;
        }
    }
    else
    {
        v20 = sub_415f50(8);
        *((unsigned long long *)idx) = 0;
        *((void* *)&idx[8]) = v20;
        iter = v20;
    }
    *((unsigned long *)iter) = 0;
    return 1;
}



// Function: diff_dirs @ 0xa0c0
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[288];
    unsigned int field_130;
    char padding_134[4];
    long long field_138;
    char padding_140[288];
    unsigned long long field_260;
} struct_0;

extern char *g_426488;
extern char g_426551;
extern long long g_426620;
extern char g_4266e8;

int diff_dirs(struct_0 *a0, unsigned long long *a1)
{
    unsigned int v10;  // eax
    unsigned int v11;  // eax
    unsigned long long idx;  // rax
    struct_1 *ptr;  // r14
    unsigned long v14;  // rbp
    unsigned long v15;  // r15
    unsigned long v16;  // r13
    unsigned long node;  // r12
    unsigned int v18;  // eax
    int v19;  // eax
    int v0;  // [bp-0x8c]
    void* iter;  // [bp-0x88]
    void* iter1;  // [bp-0x80]
    unsigned long v3;  // [bp-0x78]
    void* v4;  // [bp-0x70]
    void* v5;  // [bp-0x68]
    char v6;  // [bp-0x60]
    void* v7;  // [bp-0x58]
    void* v8;  // [bp-0x50]

    v0 = 0;
    if ((a0->field_0 == 0xffffffff || (char)sub_409cd0(a0, 0)) && (a0->field_130 == 0xffffffff || (char)sub_409cd0(a0, 1)))
    {
        error(0, 0, dcgettext(NULL, "%s: recursive directory loop", 5));
        return 2;
    }
    if (!(char)sub_409e90(a0, &v3))
    {
        sub_40df30(a0->field_8);
        v0 = 2;
    }
    if (!(char)sub_409e90(&a0->field_130, &v6))
    {
        sub_40df30(a0->field_138);
        v0 = 2;
        if (!v0)
        {
LABEL_40a1fa:
            g_4266e8 = 1;
            iter = v4;
            iter1 = v7;
            if (_setjmp(&g_426620))
                g_4266e8 = 0;
            qsort(iter, v3, 8, sub_409df0);
            qsort(iter1, v6, 8, sub_409df0);
            if (g_426488 && !a0->field_260)
            {
                for (; *((long long *)iter) && sub_409e40(*((long long *)iter), g_426488) < 0; iter += 8);
                for (; *((long long *)iter1) && sub_409e40(*((long long *)iter1), g_426488) < 0; iter1 += 8);
            }
            while (*((long long *)iter) || *((long long *)iter1))
            {
                if (!*((long long *)iter))
                {
                    iter1 += 8;
                    goto LABEL_40a2f5;
                }
                if (*((long long *)iter1))
                {
                    v10 = sub_409e40(*((long long *)iter), *((long long *)iter1));
                    if (!v10)
                    {
                        if (g_426551)
                        {
                            v11 = strcmp(*((long long *)iter), *((long long *)iter1));
                            if (v11)
                            {
                                idx = v11 >> 31;
                                ptr = (&iter)[1 + -1 * idx];
                                v14 = ptr->field_8;
                                v15 = &ptr->field_8;
                                v16 = *((long long *)(&iter)[idx]);
                                node = v15;
                                if (ptr->field_8)
                                {
                                    do
                                    {
                                        if (sub_409e40(v14, v16))
                                            break;
                                        v18 = strcmp(v14, v16);
                                        if (v18 >= 0 && !v18)
                                        {
                                            memmove(v15, ptr, node - (char *)ptr);
                                            *((unsigned long *)&ptr->padding_0[0]) = v16;
                                            break;
                                        }
                                    } while ((v14 = (unsigned long)*((long long *)(node + 8)), node = (unsigned long)(node + 8), v14));
                                }
                            }
                        }
                        iter1 += 8;
                        goto LABEL_40a2e4;
                    }
                    if (v10 < 0)
                        goto LABEL_40a33b;
                    iter1 += 8;
LABEL_40a2f5:
                    v19 = a1(a0);
                    if (v0 >= v19)
                        continue;
                    v0 = v19;
                }
                else
                {
LABEL_40a33b:
LABEL_40a2e4:
                    iter += 8;
                    goto LABEL_40a2f5;
                }
            }
        }
    }
    else if (!v0)
    {
        goto LABEL_40a1fa;
    }
    free(v4);
    free(v5);
    free(v7);
    free(v8);
    return v0;
}



// Function: find_dir_file_pathname @ 0xa4f0
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
} struct_0;

extern char g_426551;
extern long long g_426620;
extern char g_4266e8;

unsigned long long find_dir_file_pathname(long long a0, char *a1)
{
    struct_0 *iter;  // rbp
    char *v8;  // rbx
    char *v9;  // r12
    unsigned long long v10;  // rax
    char *v0;  // [bp-0x190]
    char v1;  // [bp-0x178]
    struct_0 *v2;  // [bp-0x170]
    void* v3;  // [bp-0x168]
    struct_0 v4;  // [bp-0x158]
    long long v5;  // [bp-0x150]

    v0 = a1;
    v2 = NULL;
    v3 = NULL;
    if (g_426551)
    {
        v5 = a0;
        v4 = (struct_0)0;
        if ((char)sub_409e90(&v4, &v1))
        {
            g_4266e8 = 1;
            if (!_setjmp(&g_426620))
            {
                iter = v2;
                v8 = iter->padding_0;
                if (iter->padding_0)
                {
                    do
                    {
                        v9 = a1;
                        if (sub_409e40(v8, v9))
                            continue;
                        if (strcmp(v8, v9))
                        {
                            if (v9 != v0)
                                v8 = v0;
                            v0 = v8;
                        }
                        else
                        {
                            v0 = v8;
                            break;
                        }
                    } while ((v8 = iter->field_8, iter += 8, v8));
                }
            }
            else
            {
                v0 = a1;
            }
        }
    }
    v10 = sub_410920(a0, v0, 0);
    free(v2);
    free(v3);
    return v10;
}



// Function: print_ed_hunk @ 0xa650
typedef struct FILE {
} FILE;

extern char g_41d550;
extern long long g_41d5b0;
extern FILE *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426388;

void print_ed_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // ebx
    long long v7;  // rbx
    long long v8;  // rbx
    long long v0;  // [bp-0x50]
    long long v1;  // [bp-0x48]
    long long v2;  // [bp-0x40]
    long long v3;  // [bp-0x38]

    v5 = sub_40f1a0();
    if (!v5)
        return;
    v6 = v5;
    sub_40e2b0();
    sub_40f0f0(44, &g_4261a0, v0, v1);
    fputc_unlocked((&g_41d550)[v6], g_426188);
    fputc_unlocked(10, g_426188);
    if (v6 == 1)
        return;
    v7 = v2;
    if (v3 >= v7)
    {
        while (1)
        {
            v8 = v7 + 1;
            if (*((char *)*((long long *)(g_426388 + v7 * 8))) != 46 || *((char *)(*((long long *)(g_426388 + v7 * 8)) + 1)) != 10)
            {
                sub_40f0a0(&g_41d5b0);
                v7 = v8;
                if (v3 < v7)
                    break;
            }
            else
            {
                fputs_unlocked("..\n.\ns/.//\n", g_426188);
                if (v3 < v8)
                    return;
                fputs_unlocked("a\n", g_426188);
                v7 = v8;
            }
        }
    }
    fputs_unlocked(".\n", g_426188);
    return;
}



// Function: pr_forward_ed_hunk @ 0xa7a0
typedef struct FILE {
} FILE;

extern char g_41d550;
extern long long g_41d5b0;
extern FILE *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426388;

void pr_forward_ed_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // ebx
    long long v7;  // rbx
    long long v8;  // rbx
    long long v0;  // [bp-0x40]
    long long v1;  // [bp-0x38]
    long long v2;  // [bp-0x30]
    long long i;  // [bp-0x28]

    v5 = sub_40f1a0();
    if (!v5)
        return;
    v6 = v5;
    sub_40e2b0();
    fputc_unlocked((&g_41d550)[v6], g_426188);
    sub_40f0f0(32, &g_4261a0, v0, v1);
    fputc_unlocked(10, g_426188);
    if (v6 == 1)
        return;
    v7 = v2;
    if (v7 <= i)
    {
        do
        {
            v8 = v7 + 1;
            sub_40f0a0(&g_41d5b0, g_426388 + v7 * 8);
            v7 = v8;
        } while (i >= v7);
    }
    fputs_unlocked(".\n", g_426188);
    return;
}



// Function: print_rcs_hunk @ 0xa8a0
extern long long g_41d5b0;
extern unsigned long long g_426188;
extern long long g_4261a0;
extern long long g_4262d0;
extern unsigned long long g_426388;

void print_rcs_hunk(void)
{
    unsigned int v9;  // eax
    unsigned int v10;  // ebx
    unsigned long long v11;  // r8
    unsigned long long v12;  // r8
    long long v13;  // rbx
    long long v14;  // rbx
    long long v0;  // [bp-0x60]
    long long v1;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    long long i;  // [bp-0x48]
    long long v4;  // [bp-0x40]
    long long v5;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    long long v7;  // [bp-0x28]

    v9 = sub_40f1a0();
    if (!v9)
        return;
    v10 = v9;
    sub_40e2b0();
    sub_40f0c0(&g_4261a0, v0, v1, &v4, &v5);
    if ((char)v10 & 1)
    {
        v11 = 1;
        if (v4 <= v5)
            v11 = v5 - v4 + 1;
        __fprintf_chk(g_426188, 0x1, "d%ld %ld\n", v4, v11);
    }
    if (!((char)v10 & 2))
        return;
    sub_40f0c0(&g_4262d0, v2, i, &v6, &v7);
    v12 = 1;
    if (v6 <= v7)
        v12 = v7 - v6 + 1;
    __fprintf_chk(g_426188, 0x1, "a%ld %ld\n", v5, v12);
    v13 = v2;
    if (v13 > i)
        return;
    do
    {
        v14 = v13 + 1;
        sub_40f0a0(&g_41d5b0, g_426388 + v13 * 8);
        v13 = v14;
    } while (i >= v13);
    return;
}



// Function: print_ed_script @ 0xaa10
void print_ed_script(unsigned long a0)
{
    sub_40eb10();
    return;
}



// Function: pr_forward_ed_script @ 0xaa30
void pr_forward_ed_script(unsigned long a0)
{
    sub_40eb10();
    return;
}



// Function: print_rcs_script @ 0xaa50
void print_rcs_script(unsigned long a0)
{
    sub_40eb10();
    return;
}



// Function: scan_char_literal @ 0xaa70
typedef struct struct_0 {
    char field_0;
    char field_1;
} struct_0;

struct_0 * scan_char_literal(struct_0 *a0, char *a1)
{
    char v1;  // dl
    unsigned int v2;  // eax
    struct_0 *v3;  // r8
    unsigned int v4;  // edx
    unsigned int v5;  // eax

    v1 = a0->field_0;
    switch (v1)
    {
    case 39:
        return NULL;
    case 92:
        v2 = a0->field_1;
        v3 = a0 + 1;
        v4 = 0;
        if ((char)v2 != 39)
        {
            do
            {
                if (v2 - 48 > 7)
                    return NULL;
            } while ((v3 += 1, v4 = v5 + v4 * 8, v2 = (unsigned int)(int)*((char *)((char *)v3 - 1)), (char)v2 != 39));
        }
        if (v3 - a0 - 3 <= 2)
        {
            *(a1) = v4;
            return v3;
        }
        return NULL;
    case 0:
        return NULL;
    default:
        if (a0->field_1 != 39)
            return NULL;
        *(a1) = v1;
        break;
        return NULL;
    }
}



// Function: groups_letter_value @ 0xaaf0

unsigned long long groups_letter_value(long long *a0, char a1)
{
    long long *iter;  // rdx

    iter = a0;
    switch (a1)
    {
    case 69:
        break;
    case 70:
        iter += 3;
        goto LABEL_40ab64;
    case 76:
        goto LABEL_40ab7c;
    case 77:
        iter += 3;
        goto LABEL_40ab9c;
    case 78:
        iter += 3;
        goto LABEL_40abb4;
    default:
        switch (a1)
        {
        case 101:
            return sub_40f0b0() - 1;
        case 102:
LABEL_40ab64:
            return sub_40f0b0(*(iter), iter[1]);
        case 108:
LABEL_40ab7c:
            return sub_40f0b0() - 1;
        case 109:
LABEL_40ab9c:
            return sub_40f0b0(*(iter), iter[2]);
        case 110:
LABEL_40abb4:
            return iter[2] - iter[1];
        default:
            return 0xffffffffffffffff;
        }
    }
}



// Function: do_printf_spec @ 0xabe0
typedef struct FILE {
} FILE;

void* do_printf_spec(FILE *a0, void* a1, unsigned long long a2, unsigned long a3, long long *a4)
{
    void* iter;  // rax
    unsigned long v10;  // rbx
    unsigned long v19;  // rdi
    unsigned long n;  // rdx
    unsigned long v21;  // rsi
    unsigned long v22;  // rax
    char *i;  // r8
    unsigned long long v24;  // rdi
    unsigned long long v25;  // r15
    unsigned long v26;  // rdx
    unsigned long v27;  // rsi
    unsigned int v11;  // edx
    unsigned int v12;  // edx
    char v13;  // dl
    void* v14;  // r13
    unsigned long long v15;  // rax
    void* v16;  // rax
    unsigned long v17;  // rax
    unsigned long v18;  // rax
    char v0;  // [bp-0x2068]
    char v1;  // [bp-0x1068]
    unsigned long v2;  // [bp-0x188]
    char v3;  // [bp-0x68]
    unsigned long v4;  // [bp-0x60]
    char *v5;  // [bp-0x58], Other Possible Types: unsigned long
    unsigned long v6;  // [bp-0x50]
    char v7;  // [bp-0x41]

    iter = a1 + 1;
    do
    {
        v10 = *((char *)iter);
        iter += 1;
        if ((char)v10 > 48)
        {
            if ((char)v10 - 48 > 9)
                goto LABEL_40ac40;
            goto LABEL_40aca0;
        }
    } while ((v2 = 0xfffedf7fffffffff, !(((char)v10 & 254 | *((char *)((char *)&v2 + ((long long)(v10 & 63) >> 3))) >> (unsigned long long)((char)v10 & 63 & 7) & 1) & 1)));
    if ((char)v10 - 48 > 9)
    {
        if ((char)v10 != 46)
            goto LABEL_40ac40;
        goto LABEL_40acb6;
    }
    do
    {
LABEL_40aca0:
        iter += 1;
        v10 = *((char *)iter);
    } while (v11 - 48 <= 9);
    if ((char)v10 != 46)
        goto LABEL_40ac40;
    do
    {
LABEL_40acb6:
        iter += 1;
        v10 = *((char *)iter);
    } while (v12 - 48 <= 9);
LABEL_40ac40:
    v13 = *((char *)iter);
    v14 = iter + 1;
    v15 = v10 - 88 & 0xffffffff;
    if ((char)v15 > 32)
        return NULL;
    v2 = 4303360001;
    if (!(((char)v15 & 254 | *((char *)&v2 + ((long long)(v15 & 63) >> 3)) >> ((char)v15 & 63 & 7) & 1) & 1))
    {
        if ((char)v15 == 11 && v13 == 39)
        {
            v16 = sub_40aa70(v14, &v7);
            if (!v16)
            {
                return v16;
            }
            else if (!a0)
            {
                return v16;
            }
            else
            {
                putc_unlocked(v7, a0);
                return v16;
            }
        }
    }
    else
    {
        if (!a2)
        {
            v17 = sub_40aaf0(a4, v13);
            if (v17 >= 0)
                goto LABEL_40ad1a;
        }
        else if (v13 == 110)
        {
            v17 = sub_40f0b0();
LABEL_40ad1a:
            if (!a0)
                return v14;
            v18 = v14 - a1;
            v19 = v18 + 1;
            n = v18 - 2;
            v21 = v18 - 1;
            if (v19 <= 4000)
            {
                v22 = v18 + 55;
                i = &(&v3)[-1 * (v22 & 0xfffffffffffff000)];
                if (&v3 != i)
                {
                    do
                    { } while (&v0 != i);
                }
                v24 = (unsigned int)v22 & 0xfffffff0 & 0xfff;
                /* unsupported instruction */ = (int)&(&v1)[-1 * v24];
                if (v24)
                    *((long long *)(/* unsupported instruction */ + v24 - 8)) = *((long long *)(/* unsupported instruction */ + v24 - 8));
                v25 = (/* unsupported instruction */ + 15 & 0xfffffffffffffff0) + 31 & 0xffffffffffffffe0;
            }
            else
            {
                v4 = n;
                v5 = v17;
                v6 = v21;
                v26 = v4;
                v17 = v5;
                v27 = v6;
                v25 = sub_416340(v19, v21, n);
                n = v26;
                v21 = v27;
            }
            v4 = v17;
            v5 = v25 + v21;
            v6 = n;
            memcpy(v25, a1, n);
            *((char *)(v25 + v6)) = 108;
            *(v5) = v10;
            v5[1] = 0;
            __fprintf_chk(*((unsigned int *)&a0), 0x1, v25);
            sub_411ce0(v25);
            return v14;
        }
    }
    return NULL;
}



// Function: print_ifdef_lines @ 0xae40
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[184];
    unsigned long long field_b8;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
} struct_2;

typedef struct struct_1 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
} struct_1;

extern char g_4264a0;

struct_1 * print_ifdef_lines(FILE *a0, struct_1 *a1, struct_2 *idx)
{
    unsigned long idx1;  // rbx
    unsigned long long v5;  // rbp
    struct_1 *v14;  // r13
    struct_1 *v15;  // rax
    struct_1 *v6;  // rax
    unsigned long v7;  // rdi
    unsigned long v8;  // rdx
    struct_1 *v9;  // rax
    struct_1 *v10;  // r15
    unsigned int v11;  // edi
    struct_1 *v12;  // r14
    char v13;  // al
    struct_1 *index;  // [bp-0x50]
    struct_0 *v1;  // [bp-0x48]
    struct_1 *v2;  // [bp-0x40]

    idx1 = idx->field_8;
    v1 = idx->field_0;
    v5 = idx->field_0->field_b8;
    v6 = idx->field_10;
    index = v6;
    if (!a0)
        return v6;
    if (!g_4264a0)
    {
        v6 = a1;
        if (v6->field_0 == 37)
        {
            v6 = v6->field_1;
            if (*((char *)&v6) == 108)
            {
                v6 = a1;
                if (v6->field_2 == 10 && !v6->field_3)
                {
                    if (idx1 >= index)
                        return v6;
                    v7 = *((long long *)(v5 + idx1 * 8));
                    v8 = *((long long *)(v5 + index * 8)) + (*((char *)(*((long long *)(v5 + index * 8)) - 1)) != 10) - *((long long *)(v5 + idx1 * 8));
                    return fwrite_unlocked(v7, 1, v8, a0);
                }
            }
            else if (*((char *)&v6) == 76)
            {
                v6 = a1;
                if (!v6->field_2)
                {
                    v8 = *((long long *)(v5 + index * 8)) - *((long long *)(v5 + idx1 * 8));
                    v7 = *((long long *)(v5 + idx1 * 8));
                    return fwrite_unlocked(*((long long *)(v5 + idx1 * 8)), 1, *((long long *)(v5 + index * 8)) - *((long long *)(v5 + idx1 * 8)), a0);
                }
            }
        }
    }
    if (idx1 >= index)
        return v6;
    v9 = &a1->field_1;
    v2 = v9;
    do
    {
        v10 = a1;
        v11 = v10->field_0;
        v12 = v2;
        if ((char)v11)
        {
            while (1)
            {
                if ((char)v11 != 37)
                {
                    goto LABEL_40aec3;
                }
                else
                {
                    v13 = v10->field_1;
                    v14 = &v10->field_2;
                    if (v13 != 76)
                    {
                        if (v13 == 108)
                            goto LABEL_40af49;
                        if (v13 != 37)
                        {
                            v10 = sub_40abe0(a0, v10, v1, idx1, NULL);
                            if (v10)
                            {
LABEL_40aecb:
                                v11 = v10->field_0;
                                v12 = &v10->field_1;
                                v9 = v15;
                                if (!(char)v11)
                                    break;
                                else
                                    continue;
                            }
                            else
                            {
                                v11 = 37;
                            }
                        }
                        else
                        {
                            v11 = 37;
                            v12 = v14;
                        }
LABEL_40aec3:
                        putc_unlocked(v11, a0);
                        v10 = v12;
                        goto LABEL_40aecb;
                    }
                    else
                    {
LABEL_40af49:
                        v10 = v14;
                        v12 = &v10->field_1;
                        v9 = sub_40ed50(*((long long *)(v5 + idx1 * 8)));
                        v11 = v10->field_0;
                        if (!(char)v11)
                            break;
                    }
                }
            }
        }
    } while ((idx1 = (unsigned long)(idx1 + 1), index != idx1));
    return v9;
}



// Function: format_group @ 0xb040
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern unsigned long long g_426440;
extern unsigned long long g_426448;
extern unsigned long long g_426450;

struct_0 * format_group(void* a0, struct_0 *a1, char a2, unsigned long a3)
{
    char v9;  // bl
    struct_0 *v10;  // r14
    struct_0 *v19;  // rax
    unsigned long v20;  // rax
    void* v21;  // rdi
    void* v22;  // r14
    struct_0 *v23;  // rax
    struct_0 *v24;  // rax
    char v11;  // cl
    struct_0 *v12;  // r8
    unsigned long v13;  // rdx
    unsigned long v14;  // rsi
    unsigned long long v15;  // rcx
    unsigned int v16;  // esi
    unsigned int *err;  // rax
    unsigned long long idx;  // rcx
    unsigned long v0;  // [bp-0x90]
    char *v1;  // [bp-0x88], Other Possible Types: struct_0 *
    struct_0 *v2;  // [bp-0x80], Other Possible Types: unsigned int *, unsigned long long
    struct_0 *v3;  // [bp-0x78]
    unsigned long long v4;  // [bp-0x70]
    char v5;  // [bp-0x60]
    unsigned long v6;  // [bp-0x58]
    unsigned long v7;  // [bp-0x50]

    v9 = a1->field_0;
    if (v9 == a2)
    {
        return a1;
    }
    else if (v9)
    {
        v0 = a3 + 24;
        while (1)
        {
            v10 = &a1->field_1;
            if (v9 != 37)
            {
LABEL_40b08e:
                if (a0)
                    putc_unlocked(v9, a0);
                v9 = v10->field_0;
                a1 = v10;
                goto LABEL_40b0a5;
            }
            else
            {
                v11 = a1->field_1;
                v12 = &a1->field_2;
                switch (v11)
                {
                case 37:
                    v10 = v12;
                    v9 = v11;
                    goto LABEL_40b08e;
                case 40:
                    v15 = 0;
                    v1 = &v6;
                    while (1)
                    {
                        v16 = v12->field_0;
                        if (v16 - 48 <= 9)
                        {
                            v4 = v15;
                            v3 = v12;
                            err = __errno_location();
                            *(err) = 0;
                            v2 = err;
                            idx = v4;
                            *((unsigned long long *)&v1[8 * idx]) = strtoimax(v3, &v5, 10);
                            if (*(v2))
                                break;
                            v19 = (struct_0 *)v5;
                        }
                        else
                        {
                            v2 = v15;
                            v3 = v12;
                            v20 = sub_40aaf0(a3, v16);
                            idx = v2;
                            *((unsigned long *)&v1[8 * idx]) = v20;
                            if (v20 < NULL)
                                break;
                            v19 = &v3->field_1;
                        }
                        v12 = &v19->field_1;
                        if (v19->field_0 != *((char *)(idx + 4313492)))
                            break;
                        if (idx == 1)
                        {
                            v21 = NULL;
                            v22 = NULL;
                            if (v6 == v7)
                                v21 = a0;
                            if (v6 != v7)
                                v22 = a0;
                            v23 = format_group(v21, v12, 58, a3);
                            if (!v23->field_0)
                                return v23;
                            v24 = format_group(v22, &v23->field_1, 41, a3);
                            if (!v24->field_0)
                                return v24;
                            v9 = v24->field_1;
                            a1 = &v24->field_1;
                            goto LABEL_40b0a5;
                            goto LABEL_40b0a5;
                        }
                        else
                        {
                            v15 = 1;
                        }
                    }
                case 60:
                    v2 = a1;
                    v14 = g_426448;
                    v13 = a3;
                    v1 = v12;
                    break;
                case 61:
                    v2 = a1;
                    v14 = g_426440;
                    v13 = a3;
                    v1 = v12;
                    break;
                case 62:
                    v2 = a1;
                    v13 = v0;
                    v1 = v12;
                    v14 = g_426450;
                    break;
                default:
                    a1 = sub_40abe0(a0, a1, 0, 0, a3);
                    if (a1)
                    {
                        v9 = a1->field_0;
                        goto LABEL_40b0a5;
                    }
                }
                sub_40ae40(a0, v14, v13);
                v9 = v2->field_2;
                a1 = v1;
LABEL_40b0a5:
                if (!v9)
                {
                    return a1;
                }
                else if (a2 == v9)
                {
                    return a1;
                }
            }
        }
    }
    else
    {
        return a1;
    }
}



// Function: format_ifdef @ 0xb2e0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
} struct_0;

extern void* g_426188;
extern char g_4261a0;
extern unsigned long long g_4262d0;

void format_ifdef(struct_0 *a0, unsigned long long a1, unsigned long long a2, unsigned long long a3, unsigned long long a4)
{
    unsigned long v0;  // [bp-0x48]
    unsigned long long v1;  // [bp-0x40]
    unsigned long long v2;  // [bp-0x38]
    unsigned long long v3;  // [bp-0x30]
    unsigned long long v4;  // [bp-0x28]
    unsigned long long v5;  // [bp-0x20]

    v1 = a1;
    v2 = a2;
    v4 = a3;
    v5 = a4;
    v0 = &g_4261a0;
    v3 = &g_4262d0;
    sub_40b040(g_426188, a0, 0, &v0);
    return;
}



// Function: print_ifdef_hunk @ 0xb350
typedef struct struct_0 {
    char field_0;
} struct_0;

extern struct_0 g_426460;
extern unsigned long long g_4266f0;
extern unsigned long long g_4266f8;

void print_ifdef_hunk(void)
{
    unsigned long long idx;  // rbx
    unsigned long v6;  // rsi
    unsigned long v7;  // rcx
    unsigned long v8;  // rdi
    unsigned long v0;  // [bp-0x30]
    unsigned long v1;  // [bp-0x28]
    unsigned long v2;  // [bp-0x20]
    unsigned long v3;  // [bp-0x18]

    idx = sub_40f1a0() & 0xffffffff;
    if (!(unsigned int)idx)
        return;
    sub_40e2b0();
    v6 = v0;
    v7 = v2;
    if (g_4266f8 < v6 || g_4266f0 < v7)
    {
        sub_40b2e0(*((long long *)&g_426460), g_4266f8, v6, g_4266f0, v7);
        v7 = v2;
        v6 = v0;
    }
    g_4266f8 = v1 + 1;
    v8 = *((long long *)&(&g_426460.field_0)[8 * idx]);
    g_4266f0 = v3 + 1;
    sub_40b2e0(v8, v6, g_4266f8, v7, g_4266f0);
    return;
}



// Function: print_ifdef_script @ 0xb430
typedef struct struct_0 {
    char field_0;
} struct_0;

extern char g_426270;
extern unsigned long long g_426288;
extern char g_4263a0;
extern struct_0 *g_426460;
extern void g_4266f0;
extern void g_4266f8;

long long print_ifdef_script(void)
{
    *((unsigned long long *)&g_4266f0) = -(g_426288);
    *((long long *)&g_4266f8) = *((long long *)&g_4266f0);
    sub_40eb10();
    if (*((long long *)&g_426270) <= *((long long *)&g_4266f8) && *((long long *)&g_4263a0) <= *((long long *)&g_4266f0))
        return *((long long *)&g_4266f0);
    sub_40e2b0();
    return (unsigned long long)sub_40b2e0(g_426460, *((long long *)&g_4266f8), *((long long *)&g_426270), *((long long *)&g_4266f0), *((long long *)&g_4263a0));
}



// Function: guess_lines @ 0xb4c0
unsigned long long guess_lines(unsigned long a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long v1;  // rsi
    unsigned long long v2;  // rax
    unsigned long long v3;  // rdx
    unsigned long long v4;  // rdx

    v1 = 32;
    if (a0 > 9)
        v1 = a1 / (a0 - 1);
    v2 = a2 / v1;
    v3 = 1;
    if (!v2)
        v2 = v3;
    v4 = 542551296285575042;
    if (v2 > 542551296285575042)
        v2 = v4;
    return v2 + 5;
}



// Function: prepare_text @ 0xb510
typedef struct struct_0 {
    char padding_0[160];
    void* field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[104];
    char field_120;
} struct_0;

extern char g_426490;

void* prepare_text(struct_0 *idx)
{
    void* node;  // r13
    unsigned long long iter;  // r12
    void* v3;  // rbp
    void* ptr;  // rax
    void* iter1;  // rdx
    void* v6;  // rax

    node = idx->field_a0;
    if (!node)
        return v6;
    iter = idx->field_b0;
    if (g_426490)
    {
        v3 = node + iter;
        *((char *)v3) = 13;
        ptr = rawmemchr(node, 13);
        if (v3 != ptr)
        {
            iter1 = ptr;
            while (1)
            {
                if (*((char *)ptr) != 13 || (char)ptr[1] != 10)
                {
                    v6 = ptr + 1;
                    iter1 += 1;
                    *((char *)iter1 - 1) = *((char *)ptr);
                    ptr = v6;
                    if (v3 == ptr)
                        break;
                }
                else
                {
                    iter1 += 1;
                    ptr += 2;
                    *((char *)iter1 - 1) = 10;
                    if (v3 == ptr)
                        break;
                }
            }
        }
        else
        {
            iter1 = v3;
            v6 = ptr;
        }
        iter -= v3 - iter1;
    }
    if (iter)
    {
        v6 = node + iter;
        if (*((char *)node + iter - 1) != 10)
        {
            *((char *)v6) = 10;
            iter += 1;
            idx->field_120 = 1;
            node += iter;
        }
        else
        {
            node = v6;
        }
    }
    *((unsigned long *)node) = 0;
    idx->field_b0 = iter;
    return v6;
}



// Function: find_and_hash_each_line @ 0xb600
typedef struct struct_1 {
    unsigned long long field_0;
    unsigned long long field_8;
    void* field_10;
    char field_18;
} struct_1;

typedef struct struct_0 {
    char padding_0[160];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    unsigned long long field_b8;
    unsigned long long field_c0;
    long long field_c8;
    long long field_d0;
    long long field_d8;
    void* field_e0;
    char padding_e8[16];
    long long field_f8;
    char padding_100[32];
    char field_120;
} struct_0;

extern unsigned long long g_426498;
extern char g_426552;
extern unsigned int g_426558;
extern char g_426570;
extern char g_426578;
extern unsigned int g_426580;
extern unsigned long long g_426700;
extern unsigned long long g_426708;
extern unsigned long long g_426710;
extern unsigned long long g_426718;
extern unsigned long long g_426720;

unsigned long long find_and_hash_each_line(struct_0 *idx)
{
    void* node;  // rbx
    unsigned long v22;  // r15
    unsigned long v31;  // rax
    unsigned long long v32;  // rbx
    unsigned long long v33;  // rcx
    unsigned long long *v35;  // rax
    unsigned long long j;  // rbx
    unsigned long long v38;  // r13
    unsigned long long *v39;  // rax
    unsigned long long *v40;  // rax
    char v23;  // dl
    unsigned long long *v41;  // rax
    unsigned long v43;  // rbx
    void* iter;  // r14
    unsigned long v45;  // rax
    unsigned long long *v47;  // rax
    void* v48;  // rbx
    unsigned long long *v49;  // rcx
    unsigned long i;  // rax
    void* iter1;  // r14
    unsigned long long v51;  // r14
    unsigned long v52;  // r13
    unsigned long long *v53;  // r12
    unsigned long v54;  // rbp
    unsigned long long v55;  // rdx
    struct_1 *index;  // rax
    long long v57;  // rdx
    unsigned long v58;  // r13
    long long v59;  // r12
    long long idx1;  // rax
    unsigned long v25;  // rbp
    long long v61;  // r13
    long long v62;  // rcx
    unsigned long v63;  // rsi
    unsigned long v64;  // r15
    long long iter2;  // rbp
    void* v66;  // r11
    void* *v67;  // rax
    long long v68;  // r9
    unsigned long long v26;  // r12
    unsigned long long v27;  // rsi
    unsigned long v28;  // r13
    unsigned long long *v29;  // rax
    void* v30;  // rcx
    long long v0;  // [bp-0xd0], Other Possible Types: unsigned int
    void* v1;  // [bp-0xc8], Other Possible Types: unsigned long, unsigned long long
    long long v2;  // [bp-0xc0]
    unsigned long long *v3;  // [bp-0xb8], Other Possible Types: void*, unsigned long
    unsigned long v4;  // [bp-0xb0]
    unsigned long long v5;  // [bp-0xa8]
    void* v6;  // [bp-0xa0], Other Possible Types: unsigned long
    void* v7;  // [bp-0x98]
    unsigned long long v8;  // [bp-0x90]
    unsigned long v9;  // [bp-0x88]
    long long v10;  // [bp-0x80]
    long long v11;  // [bp-0x78]
    void* v12;  // [bp-0x70]
    void* v13;  // [bp-0x68]
    unsigned int v14;  // [bp-0x60]
    char v15;  // [bp-0x5a]
    char v16;  // [bp-0x59]
    unsigned long long v17;  // [bp-0x58]
    unsigned long long v18;  // [bp-0x50]
    unsigned long v19;  // [bp-0x48]

    node = idx->field_e0;
    v9 = idx->field_b8;
    v19 = idx->field_c0;
    v11 = idx->field_d8;
    v10 = sub_415f50(idx->field_d8 * 8);
    v16 = g_426552;
    v5 = g_426708;
    v13 = *((long long *)&idx->padding_e8[8]);
    v8 = g_426700;
    v0 = g_426558;
    v12 = idx->field_b0 + idx->field_a0;
    v15 = g_426558 | g_426552;
    v17 = idx->field_c0 * 8;
    v18 = -(v17);
    if (node < *((long long *)&idx->padding_e8[8]))
    {
        v2 = 0;
        v22 = g_426710;
        v14 = g_426558 & 1;
        while (1)
        {
            v23 = *((char *)node);
            iter1 = node + 1;
            v25 = v23;
            if (v0 != 4)
            {
                if (v0 <= 4)
                {
                    if (v0 - 1 <= 2 && v23 != 10)
                    {
                        v26 = 0;
                        v27 = 0;
                        v6 = node;
                        v28 = v23;
                        v3 = g_426498;
                        v4 = v22;
                        do
                        {
                            if ((char)v0 & 2 && (v1 = v27, v29 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), v27 = v1, *((char *)(*(v29) + (unsigned long long)(char)v28 * 2 + 1)) & 32))
                            {
                                v30 = iter1;
                                do
                                {
                                    v30 += 1;
                                    if (*((char *)v30) == 10)
                                    {
                                        v22 = v4;
                                        v38 = v27;
                                        iter1 = v30;
                                        v4 = v30 - v6 - 1;
                                        goto LABEL_40b820;
                                    }
                                } while (*((char *)(*(v29) + v31 * 2 + 1)) & 32);
                            }
                            if (v14)
                            {
                                if ((char)v28 != 9)
                                {
                                    if ((char)v28 != 13)
                                    {
                                        if ((char)v28 != 8)
                                        {
                                            v26 += 1;
                                            v32 = 1;
                                            continue;
                                        }
                                        else
                                        {
                                            v28 = 8;
                                            v26 = v26 - 1 + (v26 < 1);
                                        }
                                    }
                                    else
                                    {
                                        v28 = 13;
                                        v32 = 1;
                                        v26 = 0;
                                        continue;
                                    }
                                }
                                else
                                {
                                    v28 = 32;
                                    v33 = v3 - v26 % v3;
                                    v32 = v33;
                                    if (__CFADD__(v26, v32) & 1)
                                        v26 = 0;
                                    else
                                        v26 += v33;
                                    continue;
                                }
                            }
                            v32 = 1;
                            if (v16)
                            {
                                v1 = v27;
                                v35 = __ctype_tolower_loc();
                                v27 = v1;
                                v28 = *((char *)(*(v35) + v28 * 4));
                            }
                            do
                            {
                                v27 = __ROL__(v27, 7) + v28;
                                v32 -= 1;
                            } while (j != 1);
                            v28 = *((char *)iter1);
                            iter1 += 1;
                        } while ((char)v28 != 10);
                        node = v6;
                        v22 = v4;
                        v38 = v27;
                        v4 = iter1 - node - 1;
                        goto LABEL_40b820;
                    }
                }
                else
                {
                    if (v0 == 5 && v23 != 10)
                    {
                        v39 = __ctype_b_loc();
                        v1 = node;
                        v38 = 0;
                        do
                        {
                            if (!(*((char *)(*(v39) + v25 * 2 + 1)) & 32))
                            {
                                if (v16)
                                {
                                    v40 = __ctype_tolower_loc();
                                    v25 = *((int *)(*(v40) + v25 * 4));
                                }
                                v38 = __ROL__(v38, 7) + v25;
                            }
                        } while ((v25 = (unsigned long)(unsigned long long)(char)*((char *)iter1), iter1 += 1, (char)v25 != 10));
                        node = v1;
                        v4 = iter1 - node - 1;
                        goto LABEL_40b820;
                    }
                }
                if (!v16)
                {
                    v38 = 0;
                    if (v23 != 10)
                    {
                        do
                        {
                            iter1 += 1;
                            v38 = __ROL__(v38, 7) + v25;
                            v25 = *((char *)iter1 - 1);
                        } while ((char)v25 != 10);
                        v4 = iter1 - node - 1;
                        goto LABEL_40b820;
                    }
                    else
                    {
                        v4 = 0;
                        goto LABEL_40b820;
                    }
                }
                if (v23 != 10)
                {
                    v41 = __ctype_tolower_loc();
                    v38 = 0;
                    do
                    {
                        iter1 += 1;
                        v38 = __ROL__(v38, 7) + *((int *)(*(v41) + v25 * 4));
                        v25 = *((char *)iter1 - 1);
                    } while ((char)v25 != 10);
                    v4 = iter1 - node - 1;
                    goto LABEL_40b820;
                }
            }
            else if (v23 != 10)
            {
                v38 = 0;
                v3 = node;
                v1 = v22;
                v43 = *((long long *)__ctype_b_loc());
                do
                {
                    iter = iter1;
                    if (!(*((char *)(*((long long *)__ctype_b_loc()) + v25 * 2 + 1)) & 32))
                        continue;
                    v45 = *((char *)iter1);
                    iter = iter1 + 1;
                    iter1 = iter;
                    if ((char)v45 == 10)
                        break;
                    do
                    {
                        if (!(*((char *)(*((long long *)__ctype_b_loc()) + v45 * 2 + 1)) & 32))
                        {
                            v25 = v45;
                            iter1 = iter;
                            v38 = __ROL__(v38, 7) + 32;
                            goto LABEL_40bc60;
                        }
                    } while ((v45 = (unsigned long)(unsigned long long)(char)*((char *)iter), iter += 1, iter1 = iter, (char)v45 != 10));
LABEL_40bc60:
                    if (v16)
                    {
                        v47 = __ctype_tolower_loc();
                        v25 = *((int *)(*(v47) + v25 * 4));
                    }
                    iter1 = iter + 1;
                    v38 = __ROL__(v38, 7) + v25;
                    v25 = *((char *)iter1);
                } while ((char)v25 != 10);
                node = v3;
                v22 = v1;
                v4 = iter1 - node - 1;
                goto LABEL_40b820;
            }
            v4 = 0;
            v38 = 0;
LABEL_40b820:
            v48 = node;
            v3 = g_426720 + v38 % g_426718 * 8;
            if (iter1 == v12 && idx->field_120 && g_426580 - 4 > 1)
            {
                v49 = g_426720 - 8;
                if (v0 > 1)
                    v49 = v3;
                v3 = v49;
            }
            i = *(v3);
            v6 = i;
            if (i)
            {
                v7 = iter1;
                v51 = v38;
                v52 = v22;
                do
                {
                    while (1)
                    {
                        v53 = i * 32 + v52;
                        if (v53[1] == v51)
                            break;
LABEL_40b878:
                        i = *(v53);
                        if (!i)
                            goto LABEL_40b948;
                    }
                    if (v53[3] != v4)
                    {
                        if (!v0)
                            goto LABEL_40b878;
                        goto LABEL_40b8aa;
                    }
                    v1 = v53[2];
                    if (!memcmp(v53[2], v48, v4))
                    {
LABEL_40b8b6:
                        v54 = i;
                        node = v7;
                        v22 = v52;
                        if (v2 != v11)
                            goto LABEL_40b8d8;
                        goto LABEL_40b9af;
                    }
                    if (v15)
                    {
LABEL_40b8aa:
                        if (!(char)sub_40e690())
                            goto LABEL_40b8b6;
                        goto LABEL_40b878;
                    }
                    i = *(v53);
                } while (i);
LABEL_40b948:
                v22 = v52;
                v38 = v51;
                iter1 = v7;
            }
            node = iter1;
            v55 = v5 + 1;
            if (v8 == v5)
            {
                if (v8 <= 0x1fffffffffffffe)
                {
                    v8 *= 2;
                    v1 = v55;
                    v55 = v1;
                    v22 = sub_415fa0(v22, v8 * 32);
                    v6 = *(v3);
                }
                else
                {
                    sub_416300(); /* do not return */
                }
            }
            v5 = v55;
            v54 = v5;
            index = v54 * 32 + v22;
            index->field_0 = v6;
            index->field_8 = v38;
            index->field_10 = v48;
            *((unsigned long *)&index->field_18) = v4;
            *(v3) = v54;
            if (v2 != v11)
                goto LABEL_40b8d8;
LABEL_40b9af:
            v57 = v2;
            if (v57 > 3074457345618258601)
                sub_416300(); /* do not return */
            v58 = v19;
            v59 = v57 * 2 - v58;
            if (v59 > 0xffffffffffffffe)
            {
                sub_416300(); /* do not return */
            }
            else if (v57 - v58 <= 0xffffffffffffffe)
            {
                v10 = sub_415fa0(v10, v59 * 8);
                v11 = v59;
                v9 = sub_415fa0(v9 + v17, (v59 - v58) * 8) + v18;
            }
            else
            {
                sub_416300(); /* do not return */
            }
LABEL_40b8d8:
            idx1 = v2;
            *((void* *)(v9 + idx1 * 8)) = v48;
            *((unsigned long *)(v10 + idx1 * 8)) = v54;
            v2 = idx1 + 1;
            if (node >= v13)
                break;
        }
    }
    else
    {
        v2 = 0;
    }
    v61 = 0;
    v62 = v11;
    v63 = v9;
    idx->field_c8 = v2;
    v64 = v19;
    iter2 = v2;
    v66 = v12;
    while (1)
    {
        if (v62 != iter2)
        {
            v67 = v63 + iter2 * 8;
            *(v67) = node;
            if (node == v66)
                goto LABEL_40be6e;
        }
        else
        {
            v1 = v66;
            if (v62 > 3074457345618258601 || !(v68 = (long long)(v62 * 2 - v64), v68 <= 0xffffffffffffffe && v62 - v64 <= 0xffffffffffffffe))
                sub_416300(); /* do not return */
            v0 = v68;
            v66 = v1;
            v62 = v0;
            v63 = sub_415fa0(v63 + v17, (v68 - v64) * 8) + v18;
            v67 = v63 + iter2 * 8;
            *(v67) = node;
            if (node == v66)
            {
LABEL_40be6e:
                v2 = iter2;
                v11 = v62;
                v9 = v63;
                if (idx->field_120 && g_426580 - 4 > 1)
                {
                    *(v67) = node - 1;
                    break;
                }
            }
        }
        if (v61 >= *((long long *)&g_426570) && g_426578)
        {
            v2 = iter2;
            v11 = v62;
            v9 = v63;
            break;
            g_426710 = v3;
            idx->field_b8 = v9;
            idx->field_d0 = v2;
            idx->field_d8 = v11;
            idx->field_f8 = v10;
            g_426700 = v8;
            g_426708 = v5;
            return v5;
        }
        iter2 += 1;
        do
        {
            node += 1;
        } while (*((char *)node - 1) != 10);
        v61 += 1;
    }
    g_426710 = v3;
    idx->field_b8 = v9;
    idx->field_d0 = v2;
    idx->field_d8 = v11;
    idx->field_f8 = v10;
    g_426700 = v8;
    g_426708 = v5;
    return v5;
}



// Function: file_block_read @ 0xbf50
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

void file_block_read(struct_0 *idx, long long a1)
{
    unsigned long long v2;  // rbx
    long long v3;  // rax
    unsigned long long v0;  // [bp-0x10]

    if (!a1)
        return;
    v0 = v2;
    if (idx->field_121)
        return;
    v3 = sub_416520(idx->field_0, idx->field_b0 + idx->field_a0, a1);
    if (v3 == -0x1)
        sub_40e1f0(idx->field_8); /* do not return */
    idx->field_b0 = idx->field_b0 + v3;
    idx->field_121 = a1 > v3;
    return;
}



// Function: sip @ 0xbfd0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

unsigned long long sip(struct_0 *idx, char a1)
{
    long long v1;  // rax
    unsigned int v2;  // eax
    unsigned long v3;  // r12
    void* ptr;  // rax

    if (idx->field_0 >= 0)
    {
        v1 = sub_4165d0(8, *((long long *)&idx->padding_10[56]), 0x7fffffffffffffef);
        *((long long *)&idx->padding_a8[0]) = v1;
        idx->field_a0 = sub_415f50(v1);
        if (!a1)
        {
            v2 = sub_40f710(idx->field_0, 0);
            sub_40bf50(idx, idx->padding_a8);
            v3 = idx->field_b0;
            if (v2)
            {
                if (lseek(idx->field_0, -(v3), 1) >= 0)
                {
                    idx->field_b0 = 0;
                    idx->field_121 = 0;
                }
                else
                {
                    sub_40e1f0(idx->field_8); /* do not return */
                }
            }
            ptr = memchr(idx->field_a0, 0, v3);
            return _INSERT(ptr, 0, ptr);
        }
    }
    else
    {
        *((long long *)&idx->padding_a8[0]) = 8;
        idx->field_a0 = sub_415f50(8);
    }
    idx->field_b0 = 0;
    idx->field_121 = 0;
    return 0;
}



// Function: slurp @ 0xc0d0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

unsigned long long slurp(struct_0 *idx)
{
    unsigned long long v2;  // rax
    unsigned long long v3;  // rbx
    long long v12;  // rsi
    unsigned long long v13;  // rdi
    unsigned long long v14;  // rsi
    unsigned long long v15;  // rax
    unsigned long long v4;  // rbp
    unsigned long long v5;  // rsi
    unsigned long long v6;  // rax
    unsigned long long i;  // rax
    unsigned long long v8;  // rsi
    unsigned long long v9;  // rsi
    unsigned long long v10;  // rdi
    unsigned long long v11;  // rax
    unsigned long long v0;  // [bp-0x10]

    v2 = idx->field_0;
    if ((unsigned int)v2 < 0)
        return v2;
    v0 = v3;
    if (((short)*((int *)&idx->padding_10[24]) & 0xf000) == 0x8000)
    {
        v4 = *((long long *)&idx->padding_10[48]);
        v5 = (v4 & 0xfffffffffffffff8) + 16;
        if (v4 > v5)
            sub_416300(); /* do not return */
        if (v5 > 0x7ffffffffffffffe)
            sub_416300(); /* do not return */
        if (idx->padding_a8 >= v5)
        {
            v6 = idx->field_b0;
            if (v6 <= v4)
                goto LABEL_40c242;
        }
        else
        {
            *((unsigned long long *)&idx->padding_a8[0]) = v5;
            idx->field_a0 = sub_415fa0(idx->field_a0);
            v6 = idx->field_b0;
            if (v6 <= v4)
            {
LABEL_40c242:
                sub_40bf50(idx, v4 - v6 + 1);
                v6 = idx->field_b0;
                if (v6 <= v4)
                    return v6;
            }
        }
    }
    else
    {
        v6 = idx->field_b0;
    }
    sub_40bf50(idx, idx->padding_a8 - v6);
    i = idx->field_b0;
    if (!i)
        return i;
    v8 = (unsigned long long)idx->padding_a8;
    if (i == v8)
    {
        do
        {
            if (v8 > 0x3ffffffffffffff7)
                sub_416300(); /* do not return */
            v9 = v8 * 2;
            v10 = idx->field_a0;
            *((unsigned long long *)&idx->padding_a8[0]) = v9;
            v11 = sub_415fa0(v10, v9);
            v12 = (long long)(idx->padding_a8 - idx->field_b0);
            idx->field_a0 = v11;
            sub_40bf50(idx, v12);
            i = idx->field_b0;
            v8 = (unsigned long long)idx->padding_a8;
        } while (i == v8);
    }
    v13 = idx->field_a0;
    v14 = i + 16 & 0xfffffffffffffff8;
    *((unsigned long long *)&idx->padding_a8[0]) = v14;
    v15 = sub_415fa0(v13, v14);
    idx->field_a0 = v15;
    return v15;
}



// Function: find_identical_ends @ 0xc270
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[156];
    void* field_a0;
    unsigned long long field_a8;
    unsigned long long field_b0;
    unsigned long long field_b8;
    unsigned long long field_c0;
    char padding_c8[16];
    unsigned long long field_d8;
    void* field_e0;
    unsigned long long field_e8;
    void* field_f0;
    char padding_f8[40];
    char field_120;
    char padding_121[175];
    void* field_1d0;
    unsigned long long field_1d8;
    unsigned long long field_1e0;
    unsigned long long field_1e8;
    unsigned long long field_1f0;
    char padding_1f8[16];
    unsigned long long field_208;
    void* field_210;
    unsigned long long field_218;
    void* field_220;
    char padding_228[40];
    char field_250;
} struct_0;

extern unsigned long long g_426520;
extern unsigned long long g_426560;
extern char g_426570;
extern char g_426578;
extern unsigned int g_426580;

unsigned long long find_identical_ends(struct_0 *idx)
{
    unsigned long v8;  // r10
    void* v9;  // r14
    unsigned long long v18;  // rsi
    unsigned long long v19;  // rcx
    void* iter;  // rbx
    void* v21;  // rdi
    void* iter1;  // r13
    unsigned long v23;  // rdx
    unsigned long v24;  // 4121
    unsigned long long v25;  // rbp
    unsigned long long v26;  // r11
    unsigned long long v27;  // rax
    char v10;  // al
    unsigned long v28;  // r10
    unsigned long long v29;  // r13
    long long v30;  // rax
    unsigned long long v31;  // r11
    void* v32;  // rdx
    long long v33;  // rbp
    void* v34;  // rax
    unsigned long long v35;  // rsi
    unsigned long long v36;  // rcx
    void* i;  // rbx
    unsigned long v11;  // rbp
    unsigned long long v38;  // rcx
    unsigned long long idx1;  // r12
    unsigned long long k;  // rbx
    unsigned long long v41;  // rax
    unsigned long v42;  // r8
    unsigned long long v43;  // r12
    unsigned long long v44;  // rsi
    unsigned long long v45;  // rax
    unsigned long long idx2;  // rax
    unsigned long long index;  // rax
    struct_0 *v12;  // rbp
    unsigned long long v48;  // rdx
    unsigned long long v49;  // rax
    unsigned long long v50;  // rax
    void* node;  // rax
    void* v14;  // r11
    void* iter2;  // rdx
    unsigned int v16;  // ebx
    void* v17;  // rax
    void* v0;  // [bp-0x70]
    void* v1;  // [bp-0x68], Other Possible Types: char, unsigned long long
    unsigned long long v2;  // [bp-0x60]
    unsigned long long v3;  // [bp-0x58]
    void* v4;  // [bp-0x50]
    void* v5;  // [bp-0x48]
    unsigned long long l;  // [bp-0x40]

    sub_40c0d0(idx);
    sub_40b510(idx);
    if (idx->field_0 == *((int *)&idx->padding_121[15]))
    {
        v8 = idx->field_b0;
        v9 = idx->field_a0;
        idx->field_1d8 = idx->field_a8;
        v10 = idx->field_120;
        v11 = v8;
        idx->field_1d0 = v9;
        idx->field_1e0 = v8;
        idx->field_250 = v10;
LABEL_40c2da:
        node = v9 + v11;
        v0 = v9;
        v14 = v9 + v8;
        v4 = node;
        iter2 = node;
        v16 = g_426580 - 4;
    }
    else
    {
        v12 = &idx->padding_121[15];
        sub_40c0d0(v12);
        sub_40b510(v12);
        v9 = idx->field_a0;
        v8 = idx->field_b0;
        v11 = idx->field_1e0;
        v0 = idx->field_1d0;
        if (v9 == idx->field_1d0)
            goto LABEL_40c2da;
        v17 = v0;
        v14 = v9 + v8;
        v4 = v17 + v11;
        if (v8 >= v11)
        {
            *((char *)v4) = ~(*((char *)v9 + v11));
        }
        else
        {
            v1 = *((char *)v17 + v8);
            *((char *)v14) = ~(v1);
        }
        if (*((long long *)v9) == *((long long *)v0))
        {
            node = v9;
            iter2 = v0;
            do
            {
                node += 8;
                iter2 += 8;
            } while (*((long long *)node) == *((long long *)iter2));
        }
        else
        {
            iter2 = v0;
            node = v9;
        }
        if (*((char *)iter2) == *((char *)node))
        {
            do
            {
                node += 1;
                iter2 += 1;
            } while (*((char *)node) == *((char *)iter2));
        }
        v16 = g_426580 - 4;
        if (v16 > 1 && node > v8 - idx->field_120 + v9 != iter2 > v11 - idx->field_250 + v0)
        {
            node -= 1;
            iter2 -= 1;
        }
    }
    v18 = g_426560;
    v19 = g_426560;
    if (v9 != node)
    {
        while (1)
        {
            if (*((char *)node - 1) != 10)
            {
                node -= 1;
                iter2 -= 1;
                if (v9 == node)
                    break;
            }
            else if (!v19 || !(node -= 1, v19 -= 1, iter2 -= 1, v9 != node))
            {
                break;
            }
        }
    }
    idx->field_e0 = node;
    idx->field_210 = iter2;
    if (v16 <= 1 || (iter = v14, v5 = v4, idx->field_120 == idx->field_250))
    {
        v21 = v8 - v11 + node;
        if (v8 < v11)
            v21 = node;
        iter1 = v4;
        iter = v14;
        while (1)
        {
            if (iter == v21)
            {
                if (v9 == iter)
                    goto LABEL_40c658;
                goto LABEL_40c4cb;
            }
            else if (*((char *)iter - 1) == *((char *)iter1 - 1))
            {
                iter1 -= 1;
                iter -= 1;
            }
            else
            {
                v21 = iter;
                if (v9 == iter)
                    goto LABEL_40c658;
LABEL_40c4cb:
                if (*((char *)iter - 1) != 10)
                {
                    v18 = g_426560 + 1;
                    break;
                }
LABEL_40c658:
                if (v0 != iter1)
                {
                    v18 = g_426560 + (*((char *)iter1 - 1) != 10);
                    break;
                }
            }
        }
        v23 = v18 - 1;
        if (v18 && iter != v14)
        {
            while (1)
            {
                iter += 1;
                if (*((char *)iter - 1) != 10)
                    continue;
                v23 -= 1;
                if (!(v23 & iter != v14))
                    break;
            }
        }
        v5 = iter1 + iter - v21;
    }
    v24 = g_426578;
    idx->field_f0 = iter;
    idx->field_220 = v5;
    if (!(char)v24 || g_426520 || (v25 = (unsigned long long)*((long long *)&g_426570), *((long long *)&g_426570) > 0x1ffffffffffffffe || *((long long *)&g_426570) >= v8))
    {
        l = 0;
        v29 = sub_40b4c0(0, 0, v8);
    }
    else
    {
        sub_40b4c0(0, 0, iter - node);
        v27 = sub_40b4c0(0, 0, v26 - iter);
        l = 1;
        if (*((long long *)&g_426570) > 0)
        {
            do
            {
                l *= 2;
            } while (*((long long *)&g_426570) >= l);
        }
        if (v27 < *((long long *)&g_426570))
            v25 = v27;
        v29 = v28 + l + v25;
    }
    v1 = v26;
    v30 = sub_415f50(v29 * 8);
    v31 = v1;
    v32 = idx->field_e0;
    v33 = v30;
    if (!g_426578 || v32 != iter)
    {
        i = v9;
        v36 = 0;
        if (v9 != v32)
            goto LABEL_40c46f;
LABEL_40c3fb:
        v34 = idx->field_210;
        v35 = 0;
    }
    else
    {
        v34 = v5;
        v35 = 0;
        v36 = 0;
        if (idx->field_210 != v34)
        {
            i = v9;
            v36 = 0;
            if (v9 == v32)
                goto LABEL_40c3fb;
            do
            {
LABEL_40c46f:
                v38 = v36 + 1;
                idx1 = v36 & v31;
                if (idx1 == v29)
                {
                    if (v29 <= 0x7fffffffffffffe)
                    {
                        v29 *= 2;
                        v3 = v31;
                        v2 = v38;
                        v1 = v32;
                        v31 = v3;
                        v38 = v2;
                        v32 = v1;
                        v33 = sub_415fa0(v33, v29 * 8);
                    }
                    else
                    {
                        sub_416300(); /* do not return */
                    }
                }
                *((void* *)(v33 + idx1 * 8)) = i;
                do
                {
                    i += 1;
                } while (*((char *)i - 1) != 10);
                v36 = v38;
            } while (i != v32);
            v34 = idx->field_210;
            v35 = i - v9;
            v36 = v38;
        }
    }
    v2 = v35;
    v1 = v36;
    k = *((long long *)&g_426570);
    v3 = v31;
    if (v36 <= *((long long *)&g_426570))
        k = v36;
    if (!l)
        k = v36;
    sub_40b4c0(v36, v35, v5 - v34);
    v41 = sub_40b4c0(v1, v2, v4 - v28);
    if (*((long long *)&g_426570) < v41)
        v41 = *((long long *)&g_426570);
    v43 = k + v42 + v41;
    if (k > v43)
    {
        sub_416300(); /* do not return */
    }
    else if (v43 <= 0xffffffffffffffe)
    {
        v44 = sub_415f50(v43 * 8);
        if (v1 != k)
        {
            if (k > 0)
            {
                v45 = v1 - *((long long *)&g_426570);
                idx2 = v45;
                do
                {
                    *((long long *)(v44 + -(v45) * 8 + idx2 * 8)) = *((long long *)(v33 + (v3 & idx2) * 8));
                    idx2 += 1;
                } while (k + v45 != idx2);
                index = 0;
                do
                {
                    *((long long *)(v33 + index * 8)) = *((long long *)(v44 + index * 8));
                    index += 1;
                } while (k != index);
LABEL_40c782:
                v48 = 0;
                do
                {
                    *((unsigned long *)(v44 + v48 * 8)) = *((long long *)(v33 + v48 * 8)) - v9 + v0;
                    v48 += 1;
                } while (k > v48);
            }
        }
        else
        {
            if (v1 > 0)
                goto LABEL_40c782;
        }
        v49 = k * 8;
        idx->field_218 = v1;
        idx->field_208 = v43 - k;
        idx->field_1e8 = v49 + v44;
        v50 = -(k);
        idx->field_b8 = v33 + v49;
        idx->field_1f0 = v50;
        idx->field_c0 = v50;
        idx->field_d8 = v29 - k;
        idx->field_e8 = v1;
        return v50;
    }
    else
    {
        sub_416300(); /* do not return */
    }
}



// Function: read_files @ 0xc8f0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[144];
    unsigned long long field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

extern char g_41d1a0;
extern char g_426568;
extern void g_426700;
extern unsigned long long g_426708;
extern void* g_426710;
extern unsigned long long g_426718;
extern unsigned long long g_426720;

long long read_files(struct_0 *idx, unsigned int a1)
{
    char v1;  // r13b
    unsigned int v2;  // r12d
    unsigned long v3;  // rax
    unsigned long long v4;  // rdx
    long long v5;  // rdi
    unsigned int v6;  // ecx
    unsigned long long i;  // rax
    long long v8;  // rdi

    v1 = g_426568 | (char)a1;
    v2 = a1 | (unsigned int)sub_40bfd0(idx, v1);
    if (idx->field_0 == *((int *)((char *)&idx[1].field_8 + 6)))
    {
        *((unsigned long long *)&idx[1].padding_a8[6]) = idx->field_a0;
        *((char [8])((char *)&idx[1].field_b0 + 6)) = idx->padding_a8;
        *((unsigned long long *)&idx[1].padding_b8[6]) = idx->field_b0;
        if ((char)v2)
            return v2;
    }
    else
    {
        v2 |= (unsigned int)sub_40bfd0((char *)&idx[1].field_8 + 6, v1 | (char)v2);
        if ((char)v2)
            return v2;
    }
    sub_40c270(idx);
    *((unsigned long long *)&g_426700) = *((long long *)&idx[1].padding_b8[46]) + *((long long *)&idx->padding_b8[32]) + 1;
    if (*((long long *)&g_426700) > 0x3fffffffffffffe)
        sub_416300(v8); /* do not return */
    v3 = sub_415f50(*((long long *)&g_426700) * 32);
    g_426708 = 1;
    g_426710 = v3;
    v4 = (*((long long *)&g_426700) * 6148914691236517206 >> 64) - (*((long long *)&g_426700) >> 63);
    if (v4 <= 0x200)
    {
        g_426718 = 509;
        v5 = 0xff0;
    }
    else
    {
        v6 = 9;
        do
        {
            v6 += 1;
            i = 1 << ((char)v6 & 63);
        } while (i < v4);
        g_426718 = i - (&g_41d1a0)[v6];
        if (g_426718 > 0xffffffffffffffe)
            sub_416300(v8); /* do not return */
        v5 = g_426718 * 8 + 8;
    }
    g_426720 = sub_40f5e0(v5) + 8;
    sub_40b600(idx);
    sub_40b600((char *)&idx[1].field_8 + 6);
    *((unsigned long long *)&idx[2].padding_10[4]) = g_426708;
    *((unsigned long long *)&idx[1].padding_4[2]) = g_426708;
    free(g_426710);
    free(g_426720 - 8);
    return v2;
}



// Function: print_normal_hunk @ 0xcac0
typedef struct FILE {
} FILE;

extern char g_41d550;
extern FILE *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426258;
extern long long g_4262d0;
extern unsigned long long g_426388;

void print_normal_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // r12d
    long long iter;  // rbp
    unsigned long long v8;  // rbx
    unsigned long long v9;  // rbx
    long long v10;  // rbp
    unsigned long long v11;  // rbx
    unsigned long long v12;  // rbx
    long long v0;  // [bp-0x50]
    long long v1;  // [bp-0x48]
    long long v2;  // [bp-0x40]
    long long v3;  // [bp-0x38]

    v5 = sub_40f1a0();
    if (!v5)
        return;
    v6 = v5;
    sub_40e2b0();
    sub_40eb70(4);
    sub_40f0f0(44, &g_4261a0, v0, v1);
    fputc_unlocked((&g_41d550)[v6], g_426188);
    sub_40f0f0(44, &g_4262d0, v2, v3);
    sub_40eb70(3);
    fputc_unlocked(10, g_426188);
    if ((char)v6 & 1)
    {
        iter = v0;
        if (iter <= v1)
        {
            v8 = iter * 8;
            while (1)
            {
                sub_40eb70(2);
                v9 = v8 + 8;
                sub_40ef20("<", g_426258 + v8, 1);
                sub_40eb70(3);
                if (*((char *)(*((long long *)(g_426258 + v9)) - 1)) != 10)
                {
                    iter += 1;
                    v8 = v9;
                    if (v1 < iter)
                        break;
                }
                else
                {
                    iter += 1;
                    putc_unlocked(10, g_426188);
                    v8 = v9;
                    if (v1 < iter)
                        break;
                }
            }
        }
    }
    if (v6 == 3)
    {
        fputs_unlocked("---\n", g_426188);
    }
    else if (!((char)v6 & 2))
    {
        return;
    }
    v10 = v2;
    if (v10 > v3)
        return;
    v11 = v10 * 8;
    do
    {
        sub_40eb70(1);
        v12 = v11 + 8;
        sub_40ef20(">", g_426388 + v11, 1);
        sub_40eb70(3);
        if (*((char *)(*((long long *)(g_426388 + v12)) - 1)) == 10)
            putc_unlocked(10, g_426188);
    } while ((v10 = (long long)(v10 + 1), v11 = v12, v3 >= v10));
    return;
}



// Function: print_normal_script @ 0xccf0
void print_normal_script(unsigned long a0)
{
    sub_40eb10();
    return;
}



// Function: tab_from_to @ 0xcd10
typedef struct FILE {
} FILE;

extern FILE *g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

unsigned long long tab_from_to(unsigned long long a0, unsigned long long i)
{
    unsigned long long v1;  // rbx
    unsigned long long v2;  // rbx
    unsigned long long v3;  // rbx

    if (!g_4264a0)
    {
        v1 = g_426498 + a0 - a0 % g_426498;
        if (v1 <= i)
        {
            do
            {
                a0 = v1;
                putc_unlocked(9, g_426188);
                v1 = a0 + g_426498;
            } while (i >= v1);
        }
    }
    v2 = a0 + 1;
    if (i <= a0)
        return i;
    do
    {
        v3 = v2;
        putc_unlocked(32, g_426188);
        v2 = v3 + 1;
    } while (i != v3);
    return i;
}



// Function: print_half_line @ 0xcda0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned long long g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

unsigned long long print_half_line(struct_0 **a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long iter;  // r14
    char *v6;  // rbx
    unsigned long long v14;  // rdx
    unsigned long v15;  // rax
    unsigned int v16;  // eax
    char *v7;  // r12
    unsigned long long node;  // rbp
    unsigned int v9;  // r11d
    char *v10;  // r15
    char v11;  // al
    unsigned long count;  // rcx, Other Possible Types: unsigned long long
    unsigned long long v13;  // rbx
    unsigned long long v0;  // [bp-0x78]
    unsigned long v1;  // [bp-0x70]
    char v2;  // [bp-0x4c], Other Possible Types: unsigned int
    unsigned long v3;  // [bp-0x48]

    iter = 0;
    v6 = &*(a0)->field_0;
    v7 = &a0[1]->field_0;
    v0 = a2;
    v3 = 0;
    v1 = g_426188;
    if (*(a0) < v7)
    {
        node = 0;
        while (1)
        {
            v9 = *(v6);
            v10 = v6 + 1;
            v11 = v9;
            if ((char)v9 <= 95)
            {
                if ((char)v9 <= 7)
                    goto LABEL_40ce80;
                switch (v11)
                {
                case 8:
                    v6 = v10;
                    if (node)
                    {
                        node -= 1;
                        v6 = v10;
                        if (v0 > node)
                        {
                            if (node < iter)
                            {
                                putc_unlocked(8, v1);
                                iter = node;
                                v6 = v10;
                                break;
                            }
                            else
                            {
                                v6 = v10;
                                if (node > iter)
                                {
                                    do
                                    {
                                        iter += 1;
                                        putc_unlocked(32, v1);
                                    } while (node != iter);
                                    v6 = v10;
                                    break;
                                }
                            }
                        }
                    }
                    break;
                case 9:
                    count = g_426498 - node % g_426498;
                    if (iter == node)
                    {
                        v13 = count + iter;
                        if (g_4264a0)
                        {
                            if (v0 <= v13)
                                v13 = v0;
                            if (v13 > iter)
                            {
                                do
                                {
                                    iter += 1;
                                    putc_unlocked(32, v1);
                                    count = count;
                                } while (iter != v13);
                            }
                        }
                        else if (v0 > v13)
                        {
                            putc_unlocked(9, v1);
                            iter = v13;
                        }
                    }
                    node += count;
                    v6 = v10;
                    break;
                case 10:
                    goto LABEL_40ce50;
                case 11: case 12:
LABEL_40ceb3:
                    v6 = v10;
                    if (v0 > node)
                    {
                        putc_unlocked(v9, v1);
                        v6 = v10;
                        break;
                    }
                    break;
                case 13:
                    iter = 0;
                    node = 0;
                    putc_unlocked(13, v1);
                    sub_40cd10(0, a1);
                    v6 = v10;
                    break;
                case 32: case 33: case 34: case 35: case 37: case 38: case 39: case 40: case 41: case 42: case 43: case 44: case 45: case 46: case 47: case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: case 58: case 59: case 60: case 61: case 62: case 63: case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: case 85: case 86: case 87: case 88: case 89: case 90: case 91: case 92: case 93: case 94: case 95:
LABEL_40ce2f:
                    v14 = node + 1;
                    if (v0 <= node)
                    {
                        v6 = v10;
                        node = v14;
                        if (v6 >= v7)
                            goto LABEL_40ce50;
                        else
                            continue;
                    }
                    else
                    {
                        count = v14;
                        putc_unlocked(v9, v1);
                        node = count;
                        iter = node;
                        v6 = v10;
                        break;
                    }
                default:
LABEL_40ce80:
                    v15 = sub_416640(&v2, v6, v7 - v6, &v3);
                    if (v15 - 1 > 0xfffffffffffffffc)
                        goto LABEL_40ceb3;
                    count = v15;
                    v16 = wcwidth(v2);
                    if (v16 > 0)
                        node += v16;
                    if (node <= v0)
                    {
                        fwrite_unlocked(v6, 1, count, stdout);
                        iter = node;
                    }
                    v6 = &v6[count];
                    break;
                }
                if (v6 >= v7)
                    break;
            }
            else
            {
                if (v11 - 97 <= 29)
                    goto LABEL_40ce2f;
                else
                    goto LABEL_40ce80;
            }
        }
    }
LABEL_40ce50:
    return iter;
}



// Function: print_1sdiff_line @ 0xd0a0
typedef struct struct_0 {
    char padding_0[8];
    void* field_8;
} struct_0;

typedef struct struct_1 {
    struct struct_2 *field_0;
    void* field_8;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

typedef struct FILE {
} FILE;

extern FILE *g_426188;
extern unsigned long long g_426428;
extern unsigned long long g_426430;

char * print_1sdiff_line(struct_0 *a0, char a1, struct_1 *a2)
{
    unsigned int v2;  // ebp
    char *v3;  // r8
    unsigned long v4;  // rbp
    char *v5;  // rax
    char *v6;  // r8
    int v7;  // edi
    unsigned long long v8;  // rax
    char *v9;  // rax
    char v0;  // [bp-0x48]

    switch (a1)
    {
    case 60:
        sub_40eb70(2);
        goto LABEL_40d202;
    case 62:
        sub_40eb70(1);
LABEL_40d202:
        v0 = 1;
        if (a0)
        {
LABEL_40d0ec:
            v2 = _INSERT(v4, 0, *((char *)a0->field_8 - 1) == 10);
            v5 = sub_40cda0(a0, 0, g_426430);
            v3 = v5;
            if (a1 != 32)
                goto LABEL_40d1ae;
            break;
        }
        else
        {
            v2 = 0;
            v6 = sub_40cd10(0, g_426430 + g_426428 - 1 >> 1) + 1;
            v7 = a1;
LABEL_40d227:
            v3 = v6;
            v5 = putc_unlocked(v7, g_426188);
            break;
        }
    default:
        v0 = 0;
        if (a0)
            goto LABEL_40d0ec;
        v2 = 0;
        v3 = NULL;
        if (a1 != 32)
        {
LABEL_40d1ae:
            v6 = sub_40cd10(v3, g_426430 + g_426428 - 1 >> 1) + 1;
            if (a1 == 124)
            {
                v7 = 124;
                if (*((char *)a2->field_8 - 1) == 10 != (char)v2)
                {
                    v7 = (-((char)v2 < 1) & 45) + 47;
                    goto LABEL_40d227;
                }
            }
        }
        break;
    }
    if (!a2 || !(v2 |= (unsigned int)_INSERT(a2->field_8, 0, (char)(char)(*((char *)((char *)a2->field_8 - 1)) == 10)), v5 = (char *)a2->field_0, a2->field_0->field_0 != 10))
    {
        if ((char)v2)
            goto LABEL_40d16e;
LABEL_40d131:
        if (!v0)
            return v5;
    }
    else
    {
        v8 = sub_40cd10(v3, g_426428);
        v5 = sub_40cda0(a2, v8, g_426430);
        if (!(char)v2)
            goto LABEL_40d131;
LABEL_40d16e:
        v9 = putc_unlocked(10, g_426188);
        if (!v0)
            return v9;
    }
    return sub_40eb70();
}



// Function: print_sdiff_common_lines @ 0xd260
extern unsigned long long g_426188;
extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern char g_426438;
extern char g_426439;
extern char g_42643a;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

char * print_sdiff_common_lines(unsigned long long a0, unsigned long long i)
{
    unsigned long long j;  // rbp
    unsigned long long v2;  // rbx
    char v3;  // r15b
    char *v4;  // rax

    if (!g_426438 && (j = g_426730, v2 = g_426728, v3 = (char)(char)(g_426728 != i), g_426730 != a0 || v3))
    {
        if (g_42643a)
            v4 = __fprintf_chk(g_426188, 0x1, "i%ld,%ld\n", a0 - g_426730, i - g_426728);
        if (!g_426439)
        {
            if (g_426730 != a0 && v3)
            {
                do
                {
                    v2 += 1;
                    j += 1;
                    v4 = sub_40d0a0(g_426258 + j * 8 - 8, 32, g_426388 + v2 * 8 - 8);
                    if (a0 == j)
                        goto LABEL_40d328;
                } while (i != v2);
            }
            else
            {
LABEL_40d328:
                if (i != v2)
                {
                    do
                    {
                        v4 = sub_40d0a0(NULL, 41, g_426388 + v2 * 8);
                        v2 += 1;
                    } while (i != v2);
                    if (a0 == j)
                    {
                        g_426730 = a0;
                        g_426728 = i;
                        return v4;
                    }
                }
            }
        }
        for (; a0 != j; j += 1)
        {
            v4 = sub_40d0a0(g_426258 + j * 8, 40, NULL);
        }
    }
    g_426730 = a0;
    g_426728 = i;
    return v4;
}



// Function: print_sdiff_hunk @ 0xd3c0
extern unsigned long long g_426188;
extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern char g_42643a;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

void print_sdiff_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // r12d
    unsigned long v15;  // rbx
    unsigned long v16;  // rbx
    unsigned long k;  // rcx
    unsigned long v8;  // rbx
    unsigned long v9;  // rdx
    unsigned long v10;  // rbp
    unsigned long v11;  // rbp
    unsigned long v12;  // rbx
    unsigned long v13;  // rbx
    unsigned long v14;  // rbx
    unsigned long v0;  // [bp-0x40]
    unsigned long j;  // [bp-0x38]
    unsigned long v2;  // [bp-0x30]
    unsigned long i;  // [bp-0x28]

    v5 = sub_40f1a0();
    if (!v5)
        return;
    v6 = v5;
    sub_40d260(v0, v2);
    if (g_42643a)
        __fprintf_chk(g_426188, 0x1, "c%ld,%ld\n", j - v0 + 1, i - v2 + 1);
    if (v6 == 3)
    {
        k = i;
        v8 = v0;
        v9 = j;
        if (v8 <= v9)
        {
            for (v10 = v2; k >= v10; k = i)
            {
                v11 = v10 + 1;
                v12 = v8 + 1;
                sub_40d0a0(g_426258 + v8 * 8, 124, g_426388 + v10 * 8);
                if (j >= v12)
                {
                    v10 = v11;
                    v8 = v12;
                }
                else
                {
                    v10 = v11;
                    v8 = v12;
                    k = i;
                    break;
                }
            }
        }
        v0 = v8;
        g_426730 = v8;
        v2 = v10;
        g_426728 = v10;
        v6 = (v8 <= v9) + (v10 <= k) * 2;
    }
    if ((char)v6 & 2)
    {
        v13 = v2;
        v14 = v13;
        if (v14 <= i)
        {
            do
            {
                v14 = v13 + 1;
                sub_40d0a0(NULL, 62, g_426388 + v13 * 8);
                v13 = v14;
            } while (i >= v13);
        }
        g_426728 = v14;
    }
    if (!((char)v6 & 1))
        return;
    v15 = v0;
    v16 = v15;
    if (j >= v16)
    {
        do
        {
            v16 = v15 + 1;
            sub_40d0a0(g_426258 + v15 * 8, 60, NULL);
            v15 = v16;
        } while (j >= v15);
    }
    g_426730 = v16;
    return;
}



// Function: print_sdiff_script @ 0xd5b0
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_4263a0;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

char * print_sdiff_script(long long a0)
{
    sub_40e2b0();
    g_426728 = -(g_426288);
    g_426730 = g_426728;
    sub_40eb10(a0, sub_40eaf0, sub_40d3c0);
    return sub_40d260(g_426270, g_4263a0);
}



// Function: sighandler @ 0xd600
extern unsigned int g_426774;

long long sighandler(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_426774;
    if (!(unsigned int)v1)
        g_426774 = a0;
    return v1;
}



// Function: stophandler @ 0xd620
extern unsigned int g_426770;
extern unsigned int g_426774;

long long stophandler(void)
{
    unsigned long v1;  // rax

    v1 = g_426774;
    if (!g_426774)
    {
        v1 = g_426770 + 1;
        g_426770 = v1;
    }
    return v1;
}



// Function: get_funky_string @ 0xd640
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;


unsigned int get_funky_string(void* *a0, struct_0 **a1, unsigned int a2, unsigned long long *a3)
{
    unsigned int v1;  // eax
    struct_0 *node;  // rcx
    unsigned long long iter;  // rdi
    unsigned int v4;  // edx
    unsigned long long v5;  // r8
    void* v7;  // r9

    v1 = a2;
    node = *(a1);
    iter = 1;
    v4 = node->field_0;
    v5 = 0;
    v7 = *(a0) - 0;
    if ((char)v4 != 92)
        if ((char)v4 <= 92)
            goto LABEL_0x40d67c;
        else
            goto LABEL_0x40d6f0;
}



// Function: c_escape_char @ 0xd8b0

unsigned int c_escape_char(unsigned int a0)
{
    if ((char)a0 <= 0x22)
    {
        if ((char)a0 <= 6)
            goto LABEL_40d8f0;
        switch ((char)a0)
        {
        case 8:
            a0 = 98;
            break;
        case 9:
            return 116;
        case 10:
            return 110;
        case 11:
            return 118;
        case 12:
            return 0x66;
        case 13:
            return 114;
        case 7:
            a0 = 97;
        case 34:
LABEL_40d8e6:
            return a0;
        default:
LABEL_40d8f0:
            return _INSERT(a0, 0, (char)a0 <= 31);
        }
    }
    else
    {
        if ((char)a0 == 92)
            goto LABEL_40d8e6;
        else
            goto LABEL_40d8f0;
    }
}



// Function: c_escape @ 0xd970
void* c_escape(void* a0)
{
    void* iter;  // r12
    char v2;  // dil
    void* v11;  // rsi
    char v12;  // al
    void* v13;  // r9
    void* v3;  // rsi
    void* v4;  // rsi
    char v5;  // al
    void* v6;  // rsi
    unsigned long long v7;  // rcx
    char v8;  // r9b
    void* v9;  // rax
    char v10;  // cl

    iter = a0;
    v2 = *((char *)a0);
    if (!*((char *)a0))
        return iter;
    v3 = iter;
    while (1)
    {
        v4 = v3;
        if (v2 != 32)
        {
            v5 = sub_40d8b0(v2);
            v2 = (char)v6[1];
            v3 = v6 + 1;
            if (!(char)v6[1])
                break;
        }
        else
        {
            v2 = (char)v4[1];
            v3 = v4 + 1;
            if (!(char)v4[1])
                break;
        }
    }
    if (!v7 && !v8)
        return iter;
    v9 = sub_415f50(v7 + v3 - iter + 3, v3 - iter);
    *((char *)v9) = 0x22;
    v10 = *((char *)iter);
    v11 = v9 + 1;
    if (v10)
    {
        do
        {
            v12 = sub_40d8b0(v10);
            if (!v12)
            {
                *((char *)v6) = v7;
                v11 = v6 + 1;
            }
            else if (v12 != 1)
            {
                *((char *)v6) = 92;
                *((char *)&v6[1]) = v12;
            }
            else
            {
                *((char *)v6) = 92;
                v11 = v6 + 4;
                *((char *)v11 - 3) = ((char)v7 >> 6) + 48;
                *((char *)v11 - 1) = ((char)v7 & 7) + 48;
                *((char *)v11 - 2) = ((char)((char)v7) >> 3 & 7) + 48;
            }
        } while ((v10 = (char)iter[1], iter += 1, v10));
    }
    *((unsigned short *)v11) = 0x22;
    return v13;
}



// Function: parse_diff_color @ 0xda80
typedef struct struct_2 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[8];
    unsigned long long field_18;
    void* field_20;
} struct_2;

typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
} struct_0;

typedef struct struct_1 {
    char *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
} struct_1;

extern struct_1 g_425900;
extern char g_426020;
extern struct_0 *g_426748;
extern unsigned long long g_426750;
extern char g_426758;

void parse_diff_color(void)
{
    char *iter;  // rbx
    char *v8;  // r14
    void* v16;  // rbx
    void* ptr;  // rbx
    void* v18;  // rbx
    char v9;  // al
    unsigned long long index;  // r15
    char *v11;  // rsi
    unsigned long long *v12;  // rcx
    struct_2 *idx;  // r12
    char *v0;  // [bp-0x68]
    struct_0 **v1;  // [bp-0x60]
    void* v2;  // [bp-0x50]
    char v3;  // [bp-0x43], Other Possible Types: unsigned short
    char v4;  // [bp-0x42]
    char v5;  // [bp-0x41]

    iter = &g_426748->field_0;
    iter = &g_426748->field_0;
    if (!g_426748)
    {
        return;
    }
    else if (!g_426748->field_0)
    {
        return;
    }
    else
    {
        v5 = 0;
        v8 = "lc";
        v3 = 0x3f3f;
        v2 = sub_4162e0(g_426748);
        while (1)
        {
            v9 = *(iter);
            switch (v9)
            {
            case 42:
                idx = sub_415f50(40);
                v1 = &iter;
                v0 = &v2;
                idx->field_20 = g_426750;
                g_426750 = idx;
                idx->field_8 = v2;
                iter += 1;
                if (!(char)sub_40d640(&v2, &iter, 1, idx) || !(iter += 1, *(iter) == 61 && (idx->field_18 = (unsigned long long)v2, (char)sub_40d640(v0, v1, 0, idx + 16))))
                    break;
                continue;
            case 58:
                iter += 1;
                continue;
            case 0:
                return;
            default:
                v3 = v9;
                iter += 1;
                if (!iter[1] || !(v4 = iter[1], iter += 3, iter[2] == 61))
                    break;
                index = 0;
                v11 = v8;
                while (strcmp(&v3, v11))
                {
                    index += 1;
                    v11 = (&g_425900.field_0)[index];
                    if (!v11)
                        goto LABEL_40dbb6;
                }
                v12 = &(&g_426020)[16 * (unsigned int)index];
                v12[1] = v2;
                iter = iter;
                if (!(char)sub_40d640(&v2, &iter, 0, v12))
                {
LABEL_40dbb6:
                    error(0, 0, dcgettext(NULL, "unrecognized prefix: %s", 5));
                    break;
                }
            }
        }
        error(0, 0, dcgettext(NULL, "unparsable value for --palette", 5));
        free(v2);
        v16 = g_426750;
        if (g_426750)
        {
            do
            {
                ptr = v16;
                v18 = (long long)ptr[32];
                free(ptr);
                v16 = v18;
            } while ((long long)ptr[32]);
        }
        g_426758 = 0;
        return;
    }
}



// Function: check_color_output @ 0xde60
typedef struct FILE {
} FILE;

extern char g_426180;
extern FILE *g_426188;
extern unsigned int g_42657c;
extern char g_426758;

long long check_color_output(char a0)
{
    unsigned long v2;  // rax
    unsigned long long v3;  // rbx
    unsigned int v4;  // ebx
    unsigned int v5;  // eax
    unsigned long v6;  // rax
    unsigned long long v0;  // [bp-0x8]

    if (!g_426188)
        return v6;
    v2 = g_42657c;
    if (!g_42657c)
        return g_42657c;
    v0 = v3;
    if (g_426180)
    {
        v4 = 1;
    }
    else
    {
        if (a0)
        {
            if (g_42657c != 2)
            {
                g_426758 = 0;
                return g_42657c;
            }
            g_426758 = 1;
            return (unsigned long long)sub_40da80();
        }
        v5 = isatty(fileno(g_426188));
        v2 = g_42657c;
        v4 = v5;
    }
    if ((unsigned int)v2 != 2 && ((unsigned int)v2 != 1 || !v4))
    {
        g_426758 = 0;
    }
    else
    {
        g_426758 = 1;
        v2 = (unsigned long long)sub_40da80();
    }
    if (!v4)
        return v2;
}



// Function: perror_with_name @ 0xdf30
void perror_with_name(unsigned long a0)
{
    int *err;  // rax

    err = __errno_location();
    error(0, *(err), "%s");
    return;
}



// Function: message5 @ 0xdf60
typedef struct struct_0 {
    char field_0;
} struct_0;

extern unsigned long long g_41d5b0;
extern struct_0 *g_4260a0;
extern char g_42643a;
extern char g_426480;

int message5(void* a0, unsigned long a1, unsigned long a2, unsigned long long a3, unsigned long long a4)
{
    long long v6;  // r13
    unsigned long long v7;  // rbx
    unsigned long v16;  // fs
    void* v8;  // rdi
    unsigned long v9;  // rax
    unsigned long long v10;  // rbx
    void* v11;  // r14
    void* v12;  // rcx
    unsigned long long v13;  // rbx
    void* dst;  // rax
    void* *v15;  // rax
    int v0;  // [bp-0x88]
    unsigned long long v1;  // [bp-0x70]
    unsigned long long v2;  // [bp-0x68]
    int v3;  // [bp-0x58]
    unsigned long v4;  // [bp-0x30]

    if (g_426480)
    {
        *((unsigned long *)&(&v0)[8]) = a1;
        v6 = 8;
        if (!a3)
            a3 = &g_41d5b0;
        *((unsigned long *)&(&v0)[16]) = a2;
        if (!a4)
            a4 = &g_41d5b0;
        v1 = a3;
        v7 = 0;
        v2 = a4;
        v8 = a0;
        while (1)
        {
            v9 = strlen(v8) + 1;
            *((unsigned long *)((char *)&v3 + v7)) = v9;
            v7 += 8;
            v6 += v9;
            if (v7 == 40)
                break;
            v8 = *((long long *)((char *)&v0 + v7));
        }
        v10 = 0;
        v11 = sub_415f50(v6);
        v12 = v11 + 8;
        while (1)
        {
            v13 = v10 + 8;
            dst = memcpy(v12, a0, *((long long *)((char *)&v3 + v10)));
            v12 = dst + *((long long *)((char *)&v3 + v10));
            if (v13 == 40)
                break;
            a0 = *((long long *)((char *)&v0 + v13));
            v10 = v13;
        }
        v15 = g_4260a0;
        g_4260a0 = v11;
        *(v15) = v11;
        *((unsigned long long *)v11) = 0;
        return v4 - *((long long *)(40 + v16));
    }
    else
    {
        if (!g_42643a)
            return __printf_chk(1, dcgettext(NULL, a0, 5));
        putchar_unlocked(32);
        return __printf_chk(1, dcgettext(NULL, a0, 5));
    }
}



// Function: message @ 0xe0e0
int message(void* a0, unsigned int a1, unsigned int a2)
{
    return sub_40df60(a0, a1, a2, 0, 0);
}



// Function: print_message_queue @ 0xe0f0
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_426800;

void print_message_queue(void)
{
    struct_0 **i;  // rbp
    struct_0 **ptr;  // r13
    char *iter;  // r15
    unsigned long v10;  // rbx
    unsigned long v11;  // rbx
    unsigned long long v0;  // [bp-0x80]
    unsigned long long v1;  // [bp-0x78]
    char *v2;  // [bp-0x70]
    char v3;  // [bp-0x68]
    unsigned long long v4;  // [bp-0x50]
    unsigned long long v5;  // [bp-0x48]

    i = g_426800;
    v2 = &v3;
    if (!g_426800)
        return;
    do
    {
        ptr = i;
        iter = v2;
        i = *(i);
        v10 = ptr + 1;
        v11 = v10;
        do
        {
            iter += 8;
            v11 = v11 + strlen(v11) + 1;
            *((unsigned long *)&iter) = v11;
        } while (&v5 != iter);
        v1 = v5;
        v0 = v4;
        __printf_chk(1, dcgettext(NULL, v10, 5));
        free(ptr);
    } while (i);
    return;
}



// Function: pfatal_with_name @ 0xe1f0
void pfatal_with_name(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    sub_40e0f0();
    error(2, *(err), "%s");
}



// Function: fatal @ 0xe230
void fatal(char *a0)
{
    sub_40e0f0();
    dcgettext(NULL, a0, 5);
    error(2, 0, "%s");
}



// Function: set_color_palette @ 0xe270
extern unsigned long long g_426748;

void set_color_palette(unsigned long a0)
{
    g_426748 = a0;
    return;
}



// Function: setup_output @ 0xe280
extern unsigned long long g_426188;
extern char g_426759;
extern unsigned long long g_426760;
extern unsigned long long g_426768;

void setup_output(unsigned long a0, unsigned long a1, char a2)
{
    g_426768 = a0;
    g_426760 = a1;
    g_426759 = a2;
    g_426188 = 0;
    return;
}



// Function: begin_output @ 0xe2b0
extern char g_41d558;
extern unsigned long long stdout;
extern unsigned long long g_426188;
extern unsigned long long g_4261a0;
extern long long g_426420;
extern char g_426480;
extern unsigned int g_426580;
extern int g_426740;
extern char g_426759;
extern void g_426760;
extern void* g_426768;

void begin_output(void)
{
    void* v9;  // rbp
    unsigned int *err;  // rax
    int v0;  // [bp-0x60]
    int v1;  // [bp-0x5c]
    void* v2;  // [bp-0x58]
    unsigned long v3;  // [bp-0x50]
    unsigned long v4;  // [bp-0x48]
    unsigned long long v5;  // [bp-0x40]
    void* v6;  // [bp-0x38]
    unsigned long long v7;  // [bp-0x30]

    if (g_426188)
        return;
    v2 = sub_40d970(g_426768);
    v3 = sub_40d970(*((long long *)&g_426760));
    v9 = sub_416460("diff%s %s %s", g_426420, v2, v3);
    if (!g_426480)
    {
        g_426188 = stdout;
        sub_40de60(0);
        if (g_426759)
            __printf_chk(1, "%s\n", v9);
    }
    else if (!fflush_unlocked(stdout))
    {
        v6 = v9;
        v4 = &g_41d558;
        v5 = "-h";
        v7 = 0;
        if (pipe(&v0))
            sub_40e1f0("pipe"); /* do not return */
        g_426740 = fork();
        if (g_426740 < 0)
            sub_40e1f0("fork"); /* do not return */
        if (!g_426740)
        {
            close(v1);
            if (v0)
            {
                if (dup2(v0, 0) >= 0)
                    close(v0);
                else
                    sub_40e1f0("dup2"); /* do not return */
            }
            execv(&g_41d558, &v4);
            err = __errno_location();
            _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
        }
        close(v0);
        g_426188 = fdopen(v1, "w");
        if (!g_426188)
            sub_40e1f0("fdopen"); /* do not return */
        sub_40de60(1);
    }
    else
    {
        sub_40e1f0(dcgettext(NULL, "write failed", 5)); /* do not return */
    }
    free(v9);
    if (g_426580 == 2)
    {
        sub_4087e0(&g_4261a0, &v2, 0);
    }
    else if (g_426580 == 3)
    {
        sub_4087e0(&g_4261a0, &v2, 1);
    }
    if (v2 != g_426768)
        free(v2);
    if (v3 != *((long long *)&g_426760))
    {
        free(v3);
        return;
    }
    return;
}



// Function: finish_output @ 0xe550
typedef struct FILE {
} FILE;

extern char stdout;
extern FILE *g_426188;
extern int g_426740;

void finish_output(void)
{
    char v2;  // al
    unsigned int v3;  // ebx
    char *v4;  // rsi
    unsigned int v0;  // [bp-0x14]

    if (!g_426188)
    {
        g_426188 = 0;
        return;
    }
    else if (g_426188 == *((long long *)&stdout))
    {
        g_426188 = 0;
        return;
    }
    else if (ferror_unlocked(g_426188))
    {
        sub_40e230("write failed"); /* do not return */
    }
    else if (fclose(g_426188))
    {
        sub_40e1f0(dcgettext(NULL, "write failed", 5)); /* do not return */
    }
    else if (waitpid(g_426740, &v0, 0) >= 0)
    {
        v2 = v0;
        if (!(v2 & 127))
        {
            v3 = v2;
            if (!v2)
            {
                g_426188 = 0;
                return;
            }
            v4 = "subsidiary program '%s' could not be invoked";
            if (v3 != 126)
            {
                v4 = "subsidiary program '%s' failed (exit status %d)";
                if (v3 == 127)
                    v4 = "subsidiary program '%s' not found";
            }
        }
        else
        {
            v4 = "subsidiary program '%s' failed";
        }
        error(2, 0, dcgettext(NULL, v4, 5));
    }
    else
    {
        sub_40e1f0("waitpid"); /* do not return */
    }
}



// Function: lines_differ @ 0xe690
extern unsigned long long g_426498;
extern char g_426552;
extern unsigned int g_426558;

unsigned int lines_differ(void* a0, void* a1)
{
    void* v6;  // r15
    unsigned long long iter;  // r12
    unsigned long v15;  // rdx
    void* v16;  // r9
    unsigned long long iter1;  // r9
    unsigned long v18;  // rax
    void* v19;  // rbp
    unsigned long v20;  // rax
    unsigned long v21;  // rdx
    unsigned short v22;  // r11w
    unsigned long v23;  // r9
    char v24;  // dl
    char v8;  // cl
    void* iter2;  // rsi
    unsigned long long *v26;  // rax
    unsigned long long v27;  // rax
    unsigned long v9;  // rbx
    void* node;  // rbp
    unsigned long v11;  // r11
    unsigned long v12;  // rax
    unsigned long v13;  // rdx
    void* v14;  // r11
    unsigned int v0;  // [bp-0x5c]
    unsigned int v1;  // [bp-0x50]
    char v2;  // [bp-0x49]
    void* v3;  // [bp-0x48]
    void* v4;  // [bp-0x40]

    v6 = a0;
    iter = 0;
    v4 = a1 + 1;
    v0 = g_426558;
    v2 = g_426552;
    v3 = v6 + 1;
    v1 = g_426558 - 2;
    while (1)
    {
        v8 = *((char *)v6);
        v9 = *((char *)a1);
        iter2 = v6 + 1;
        node = a1 + 1;
        if (v8 != (char)v9)
            break;
LABEL_40e7ca:
        a1 = node;
        v6 = iter2;
        if ((char)v9 == 10)
            return 0;
        v27 = 1;
        if ((char)v9 == 9)
            v27 = g_426498 - iter % g_426498;
        iter += v27;
    }
    if (v0 == 4)
    {
        v8 = v8;
        iter2 = iter2;
        v20 = *((long long *)__ctype_b_loc());
        v21 = v8;
        v22 = *((short *)(v20 + (char)v9 * 2)) & 0x2000;
        if (!(*((char *)(v20 + v21 * 2 + 1)) & 32))
        {
            if (!v22)
                goto LABEL_40e951;
        }
        else
        {
            for (v8 = v21; v8 != 10; iter2 += 1)
            {
                v23 = *((char *)iter2);
                v8 = v23;
                if (!(*((char *)(v20 + v23 * 2 + 1)) & 32))
                {
                    v8 = 32;
                    break;
                }
            }
            if (!v22)
                goto LABEL_40e9f5;
        }
        while ((char)v9 != 10)
        {
            v9 = *((char *)node);
            if (*((char *)(v20 + v9 * 2 + 1)) & 32)
            {
                node += 1;
            }
            else
            {
                v9 = 32;
                goto LABEL_40e9f5;
            }
        }
LABEL_40e9f5:
        if ((char)v9 != v8)
        {
LABEL_40e951:
            v24 = (char)v9 == 32 & v8 != 10;
            if ((char)v9 == 32 & v8 != 10)
            {
                if (iter2 > v3)
                {
                    v9 = 32;
                    v6 = iter2 - 1;
                    a1 = node;
                    if (*((char *)(v20 + *((char *)iter2 - 2) * 2 + 1)) & 32)
                        continue;
                }
                else
                {
                    v9 = 32;
                }
            }
            else
            {
                v24 = (char)v9 != 10;
            }
            if (v8 == 32 && v24)
            {
                if (node > v4)
                {
                    a1 = node - 1;
                    v6 = iter2;
                    if (*((char *)(v20 + *((char *)node - 2) * 2 + 1)) & 32)
                        continue;
                }
                else
                {
                    v8 = 32;
                }
            }
        }
    }
    else if (v0 <= 4)
    {
        if (v0 == 1)
        {
LABEL_40e880:
            if (v8 != 32 || (char)v9 != 9)
            {
                if (v8 == 9 && (char)v9 == 32)
                {
                    v8 = 9;
                    goto LABEL_40e8a1;
                }
            }
            else
            {
LABEL_40e8a1:
                iter1 = iter;
                while (1)
                {
                    switch (v8)
                    {
                    case 32:
                        iter1 += 1;
                        break;
                    case 9:
                        iter1 = iter1 + g_426498 - iter1 % g_426498;
                        break;
                    default:
                        while (1)
                        {
                            if ((char)v9 != 32)
                            {
                                if ((char)v9 != 9)
                                    break;
                                iter = g_426498 + iter - iter % g_426498;
                            }
                            else
                            {
                                iter += 1;
                            }
                            v9 = *((char *)node);
                            node += 1;
                        }
                    }
                    v8 = *((char *)iter2);
                    iter2 += 1;
                }
                if (iter1 != iter)
                    return 1;
            }
        }
        else if (v1 <= 1)
        {
            v8 = v8;
            iter2 = iter2;
            v11 = v8;
            v12 = *((long long *)__ctype_b_loc());
            v8 = v11;
            if (*((char *)(v12 + v11 * 2 + 1)) & 32 && *((char *)(v12 + (char)v9 * 2 + 1)) & 32)
            {
                if (v8 != 10 && !(v13 = (unsigned long)(unsigned long long)(char)(char)v6[1], v14 = iter2, (char)v13 == 10))
                {
                    do
                    {
                        if (!(*((char *)(v12 + v13 * 2 + 1)) & 32))
                            goto LABEL_40e798;
                    } while ((v13 = (unsigned long)(unsigned long long)(char)(char)v14[1], v14 += 1, (char)v13 != 10));
                }
                else if ((char)v9 == 10)
                {
                    return 0;
                }
                else if ((char)a1[1] == 10)
                {
                    return 0;
                }
                else
                {
                    v16 = node;
                    while (*((char *)(v12 + v15 * 2 + 1)) & 32)
                    {
                        v16 += 1;
                        if ((char)v16[1] == 10)
                            return 0;
                    }
                }
            }
            else
            {
                if (v0 != 2)
                    goto LABEL_40e880;
            }
        }
    }
    else
    {
        if (v0 == 5)
        {
            v8 = v8;
            iter2 = iter2;
            for (v18 = *((long long *)__ctype_b_loc()); *((char *)(v18 + v8 * 2 + 1)) & 32 && v8 != 10; iter2 += 1)
            {
                v8 = *((char *)iter2);
            }
            v19 = node;
            if (*((char *)(v18 + (char)v9 * 2 + 1)) & 32)
            {
                do
                {
                    node = v19;
                } while ((char)v9 != 10 && (v19 = node + 1, v9 = (unsigned long)(unsigned long long)(char)*((char *)node), node = v19, *((char *)(v18 + v9 * 2 + 1)) & 32));
            }
        }
    }
LABEL_40e798:
    if (v2)
    {
        v26 = __ctype_tolower_loc();
        v8 = *((char *)(*(v26) + v8 * 4));
        v9 = *((char *)(*(v26) + v9 * 4));
    }
    if (v8 != (char)v9)
        return 1;
    goto LABEL_40e7ca;
}



// Function: find_change @ 0xeaf0
long long find_change(unsigned long a0)
{
    return a0;
}



// Function: print_script @ 0xeb10
void print_script(unsigned long long a0)
{
    if (!a0)
        goto LABEL_0x40eb60;
    else
        goto LABEL_0x40eb19;
}



// Function: set_color_context @ 0xeb70
extern unsigned int g_426010;
extern long long g_426020;
extern long long g_426028;
extern long long g_426030;
extern long long g_426038;
extern long long g_426050;
extern long long g_426058;
extern long long g_426060;
extern long long g_426068;
extern long long g_426070;
extern long long g_426078;
extern long long g_426080;
extern long long g_426088;
extern long long g_426090;
extern long long g_426098;
extern char g_426758;

unsigned long long set_color_context(unsigned int a0)
{
    unsigned long long v1;  // rax
    unsigned long long v2;  // rax

    if (a0 != 3)
        v1 = sub_40ec70();
    if (!g_426758)
        return v1;
    if (g_426010 == a0)
        return v1;
    sub_40df10(g_426020, g_426028);
    switch (a0)
    {
    case 0:
        sub_40df10(g_426060, g_426068);
        break;
    case 1:
        sub_40df10(g_426070, g_426078);
        break;
    case 2:
        sub_40df10(g_426080, g_426088);
        v2 = sub_40df10(g_426030, g_426038);
        g_426010 = a0;
        return v2;
    case 3:
        sub_40df10(g_426050, g_426058);
        break;
    case 4:
        sub_40df10(g_426090, g_426098);
        break;
    default:
        abort(); /* do not return */
    }
}



// Function: process_signals @ 0xec70
typedef struct FILE {
} FILE;

extern FILE *stdout;
extern unsigned int g_426770;
extern unsigned int g_426774;
extern int g_426780;

void process_signals(void)
{
    unsigned int v2;  // r12d
    int v0[34];  // [bp-0xa8]

    while (g_426774 || g_426770)
    {
        sub_40eb70(3);
        fflush_unlocked(stdout);
        sigprocmask(0, &g_426780, v0);
        v2 = g_426774;
        if (g_426770)
        {
            v2 = 19;
            g_426770 = g_426770 - 1;
        }
        else
        {
            signal(g_426774, NULL);
        }
        raise(v2);
        sigprocmask(2, v0, NULL);
    }
    return;
}



// Function: output_1_line @ 0xed50
typedef struct FILE {
} FILE;

extern FILE *g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

long long output_1_line(char *a0, char *a1, char *a2, unsigned long long a3)
{
    char *node;  // rbx
    unsigned long v5;  // rbp
    unsigned long long *v14;  // rax
    unsigned long v15;  // rax
    unsigned long long v16;  // rcx
    unsigned long long i;  // r12
    unsigned long v18;  // rax
    unsigned long v6;  // rax
    unsigned long v7;  // r12
    unsigned long count;  // rax
    unsigned long v9;  // rax
    unsigned long v10;  // rax
    unsigned long long iter;  // r15
    unsigned long long v12;  // r14
    unsigned long v13;  // rdi
    unsigned int v0;  // [bp-0x4c]
    unsigned long v1;  // [bp-0x48]
    unsigned long long v2;  // [bp-0x40]

    node = a0;
    v2 = a3;
    if (!g_4264a0)
    {
        v5 = a1 - a0;
        if (a1 == a0)
            return v6;
        while (1)
        {
            v7 = 0x400;
            if (v5 <= 0x400)
                v7 = v5;
            count = fwrite_unlocked(node, 1, v7, g_426188);
            if (v7 > count)
                return count;
            v5 -= count;
            node = &node[count];
            v9 = (unsigned long long)sub_40ec70();
            if (!v5)
                return v9;
        }
    }
    else
    {
        v10 = g_426498;
        iter = 0;
        v12 = 1;
        v1 = g_426498;
        if (a0 >= a1)
            return g_426498;
        while (1)
        {
            v13 = *(node);
            node += 1;
            if ((char)v13 == 9)
            {
                v16 = v1 - iter % v1;
                iter += v16;
                do
                {
                    i = v16;
                    v10 = putc_unlocked(32, g_426188);
                    v16 = i - 1;
                } while (i != 1);
                goto LABEL_40ee4f;
            }
            if ((char)v13 != 13)
            {
                if ((char)v13 != 8)
                {
                    v0 = v13;
                    v14 = __ctype_b_loc();
                    v15 = *((short *)(*(v14) + v13 * 2));
                    iter += _INSERT(v15, 0, (unsigned short)v15 >> 14) & 1;
                    v10 = putc_unlocked(v0, g_426188);
                    goto LABEL_40ee4f;
                }
                if (iter)
                {
                    iter -= 1;
                    v10 = putc_unlocked(v13, g_426188);
                    goto LABEL_40ee4f;
                }
            }
            else
            {
                v10 = putc_unlocked(v13, g_426188);
                if (a2 && a1 > node)
                {
                    iter = 0;
                    if (*(node) != 10)
                        __fprintf_chk(*((unsigned int *)&g_426188), 0x1, a2);
LABEL_40ee58:
                    v12 += 1;
                    v10 = v18;
                    if (v12 == 0x400)
                    {
                        v10 = (unsigned long long)sub_40ec70();
                        v12 = 0;
                    }
                }
                else
                {
                    iter = 0;
LABEL_40ee4f:
                    if (a1 == node)
                        return v10;
                    goto LABEL_40ee58;
                }
            }
        }
    }
}



// Function: print_1_line_nl @ 0xef20
typedef struct struct_0 {
    struct struct_1 *field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_426188;
extern char g_426491;
extern char g_426492;

long long print_1_line_nl(char *a0, struct_0 *a1, char a2)
{
    char *v1;  // r14
    void* v2;  // r12
    char *v3;  // r13
    char *v4;  // r13
    char *v5;  // rdx
    long long v6;  // rax
    char *v7;  // rax

    v1 = &a1->field_0->field_0;
    v2 = a1->field_8;
    if (a0)
    {
        v3 = NULL;
        if (*(a0))
        {
            v3 = "%s ";
            v4 = "%s\t";
            if (g_426492)
                v3 = v4;
            v5 = v3;
            if (g_426491)
            {
                v5 = v3;
                if (*(v1) == 10)
                    v5 = "%s";
            }
            __fprintf_chk(g_426188, 0x1, v5);
        }
        if (!a2)
        {
            v6 = sub_40ed50(v1, v2, v3, a0);
            if (!*(a0))
                return v6;
        }
        else
        {
            v6 = sub_40ed50(v1, v2 - (*((char *)v2 - 1) == 10), v3, a0);
            if (!*(a0))
                return v6;
        }
    }
    else
    {
        v6 = (!a2 ? sub_40ed50(v1, v2, NULL, 0) : sub_40ed50(v1, v2 - (*((char *)v2 - 1) == 10), NULL, 0));
    }
    if (*((char *)v2 - 1) == 10)
        return v6;
    sub_40eb70(3);
    v7 = dcgettext(NULL, "No newline at end of file", 5);
    return __fprintf_chk(g_426188, 0x1, "\n\\ %s\n", v7);
}



// Function: print_1_line @ 0xf0a0
typedef struct struct_0 {
    struct struct_1 *field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

long long print_1_line(char *a0, struct_0 *a1)
{
    return sub_40ef20(a0, a1, 0);
}



// Function: translate_line_number @ 0xf0b0
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

long long translate_line_number(struct_0 *a0, unsigned long a1)
{
    return a1 + a0->field_e8 + 1;
}



// Function: translate_range @ 0xf0c0
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

long long translate_range(struct_0 *a0, unsigned int a1)
{
    long long *v1;  // rcx
    struct_0 *v2;  // rdi
    unsigned int v3;  // rdx
    long long v4;  // rax
    long long *v5;  // r8

    *(v1) = sub_40f0b0(a0, a1 - 1) + 1;
    v4 = sub_40f0b0(v2, v3 + 1);
    *(v5) = v4 - 1;
    return v4 - 1;
}



// Function: print_number_range @ 0xf0f0
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

extern unsigned long long g_426188;

int print_number_range(unsigned long a0, struct_0 *a1, unsigned int a2, unsigned long a3)
{
    char v3;  // r10b
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_40f0c0(a1, a2);
    if (v1 <= v0)
        return __fprintf_chk(g_426188, 0x1, "%ld", v1);
    return __fprintf_chk(g_426188, 0x1, "%ld%c%ld", v0, v3, v1);
}



// Function: analyze_hunk @ 0xf1a0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern long long g_4264c0;
extern unsigned long long g_4264e0;
extern char g_426554;
extern unsigned int g_426558;

unsigned int analyze_hunk(struct_0 *index, unsigned long long *a1, unsigned long long *a2, unsigned long long *a3, unsigned long long *a4)
{
    char v13;  // al
    unsigned int v14;  // ebp
    unsigned long v23;  // r8
    unsigned long long *v24;  // rax
    unsigned long v25;  // rdx
    unsigned long v26;  // r12
    char *v27;  // rsi
    unsigned long long v28;  // r13
    unsigned long v29;  // r8
    unsigned long long *v30;  // rax
    unsigned long v31;  // rdx
    unsigned int v32;  // r8d
    unsigned long long v15;  // r14
    struct_0 *idx;  // r12
    unsigned long idx1;  // rbx
    unsigned long idx2;  // r15
    unsigned long long v19;  // r12
    struct_0 *v20;  // r14
    char *v21;  // rsi
    unsigned long long v22;  // r13
    unsigned long v0;  // [bp-0x98]
    unsigned long v1;  // [bp-0x90]
    unsigned long v2;  // [bp-0x88]
    char *iter;  // [bp-0x80]
    struct_0 *v4;  // [bp-0x78], Other Possible Types: char
    char v5;  // [bp-0x6a]
    char v6;  // [bp-0x69]
    unsigned int v7;  // [bp-0x68]
    int v8;  // [bp-0x64]
    unsigned long long v9;  // [bp-0x60]
    unsigned long long v10;  // [bp-0x58]
    unsigned long v11;  // [bp-0x50]

    v13 = g_426554;
    if (!g_426554)
    {
        v8 = -0x1;
        v7 = 0;
        v13 = g_4264e0;
        v14 = 0;
    }
    else if (g_426558 > 1)
    {
        v8 = 0;
        v14 = 1;
        v7 = 3 < g_426558;
    }
    else
    {
        v8 = 0;
        v14 = 0;
        v7 = 0;
    }
    v10 = 0;
    v9 = 0;
    v15 = g_426258;
    *(a1) = index->field_18;
    v11 = g_426388;
    idx = index;
    *(a3) = index->field_20;
    do
    {
        idx1 = idx->field_18;
        idx2 = idx->field_20;
        v10 += idx->field_8;
        v9 += idx->field_10;
        v1 = idx->field_10 + idx1 - 1;
        v0 = idx2 + idx->field_8 - 1;
        v4 = idx1 <= v1 & v13;
        if (idx1 <= v1 & v13)
        {
            v19 = v15;
            v20 = idx;
            do
            {
                v21 = *((long long *)(v19 + idx1 * 8));
                v22 = *((long long *)(v19 + idx1 * 8 + 8)) + (*((char *)(*((long long *)(v19 + idx1 * 8 + 8)) - 1)) != 10) - 1;
                v23 = v22 - v21;
                if (v14)
                {
                    v5 = *(v21);
                    if (*(v21) != 10)
                    {
                        iter = v21;
                        v2 = v23;
                        v24 = __ctype_b_loc();
                        v25 = v5;
                        do
                        {
                            if (!(*((char *)(*(v24) + v25 * 2 + 1)) & 32))
                            {
                                v23 = v22 - iter;
                                if (!v7)
                                    v23 = v2;
                                goto LABEL_40f2e2;
                            }
                        } while ((v25 = (unsigned long)(unsigned long long)iter[1], iter += 1, (char)v25 != 10));
                        v23 = v22 - iter;
                    }
                }
LABEL_40f2e2:
                if (v8 == v23)
                {
                    v13 = v4;
                }
                else if (g_4264e0)
                {
                    v13 = ~((unsigned int)re_search(&g_4264c0)) >> 31;
                }
                else
                {
                    v15 = v19;
                    idx = v20;
                    v13 = 0;
                }
            } while ((idx1 = (unsigned long)(idx1 + 1), v1 >= idx1 && v13));
            idx = v20;
            v15 = v19;
            idx2 = idx->field_20;
            v6 = idx2 <= v0 & v13;
        }
        else
        {
            v6 = idx2 <= v0 & v13;
        }
        if (v6)
        {
            v4 = idx;
            v26 = v11;
            do
            {
                v27 = *((long long *)(v26 + idx2 * 8));
                v28 = *((long long *)(v26 + idx2 * 8 + 8)) + (*((char *)(*((long long *)(v26 + idx2 * 8 + 8)) - 1)) != 10) - 1;
                v29 = v28 - v27;
                if (v14)
                {
                    v5 = *(v27);
                    if (*(v27) != 10)
                    {
                        iter = v27;
                        v2 = v29;
                        v30 = __ctype_b_loc();
                        v31 = v5;
                        do
                        {
                            if (!(*((char *)(*(v30) + v31 * 2 + 1)) & 32))
                            {
                                v29 = v28 - iter;
                                if (!v7)
                                    v29 = v2;
                                goto LABEL_40f465;
                            }
                        } while ((v31 = (unsigned long)(unsigned long long)iter[1], iter += 1, (char)v31 != 10));
                        v29 = v28 - iter;
                    }
                }
LABEL_40f465:
                if (v8 == v29)
                {
                    v13 = v6;
                }
                else if (g_4264e0)
                {
                    v13 = ~((unsigned int)re_search(&g_4264c0)) >> 31;
                }
                else
                {
                    idx = v4;
                    v13 = 0;
                }
            } while ((idx2 = (unsigned long)(idx2 + 1), v0 >= idx2 && v13));
            idx = v4;
        }
    } while ((idx = (struct_0 *)idx->field_0, idx));
    v32 = 0;
    *(a2) = v1;
    *(a4) = v0;
    if (!v13)
        v32 = (v10) * 2 | v9;
    return v32;
}



// Function: concat @ 0xf560
char * concat(char *a0, char *a1, char *a2)
{
    unsigned long len;  // rax
    unsigned long len1;  // rax
    char *v4;  // rax
    char *v0;  // [bp-0x38]

    len = strlen(a0);
    len1 = strlen(a1);
    v4 = sub_415f50(len + len1 + strlen(a2) + 1);
    v0 = a2;
    __sprintf_chk(v4, 0x1);
    return v4;
}



// Function: zalloc @ 0xf5e0
void* zalloc(unsigned long a0)
{
    return memset(sub_415f50(), 0, a0);
}



// Function: debug_script @ 0xf600
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern FILE *stdout;
extern FILE *stderr;

int debug_script(struct_0 *a0)
{
    struct_0 *v2;  // rbx
    struct_0 *v3;  // rbx
    unsigned long v0;  // [bp-0x28]

    v2 = a0;
    fflush_unlocked(stdout);
    if (v2)
    {
        do
        {
            v3 = v2;
            v0 = v3->field_8;
            __fprintf_chk(*((unsigned int *)&stderr), 0x1, "%3ld %3ld delete %ld insert %ld\n");
            v2 = v3->field_0;
        } while (v3->field_0);
    }
    return fflush_unlocked(stderr);
}


