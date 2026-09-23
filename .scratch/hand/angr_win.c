        __assert_fail(); /* do not return */
    if (g_42b338 == g_42b330)
    {
        g_42b328 = sub_41c638(g_42b328, g_42b330, 416, g_42b330);
        g_42b330 = g_42b330 * 2;
    }
    ptr = g_42b338 * 208 + g_42b328;
    memset(ptr, 0, 208);
    *((unsigned long long *)&ptr[32]) = a2;
    *((unsigned int *)&ptr[168]) = a1;
    *((unsigned int *)&ptr[196]) = 0xffffffff;
    if (g_42b340 != 1 && g_42b341)
    {
        *((unsigned int *)&ptr[196]) = (char)sub_4099b1(a0);
        if ((int)ptr[196])
            g_42b340 = 1;
    }
    if (a3 || g_42b3d5 || g_42b440 || a1 == 3 && g_42b3d4 && ((char)sub_406325(19) || (char)sub_406325(18) || (char)sub_406325(20)) || (g_42b3e9 || g_42b441) && (a1 == 6 || !a1) && (*((int *)&g_42b3ec) == 4 || g_42b358 || g_42b3e8) || g_42b3e9 && !a2 || g_42b441 && (!a1 || a3 || a1 == 5 && (g_42b3d0 == 3 || g_42b3d4 && ((char)sub_406325(14) || (char)sub_406325(16) || (char)sub_406325(0x11) || (char)sub_406325(21)))))
    {
        if (*((char *)a0) == 47 || !*(v1))
        {
            v19 = a0;
        }
        else
        {
            len = strlen(a0);
            v29 = (unsigned long long)((15 + strlen(v1) + len + 10) / 16) * 16;
            while (&v0 != (char *)&v1 - (v29 & 0xfffffffffffff000))
            ;
            /* unsupported instruction */ = (int)&(&v0)[-1 * ((unsigned int)v29 & 0xfff)];
            if ((unsigned short)v29 & 0xfff)
                *((long long *)(((unsigned int)v29 & 0xfff) - 8 + /* unsupported instruction */)) = *((long long *)(((unsigned int)v29 & 0xfff) - 8 + /* unsupported instruction */));
            v19 = (/* unsupported instruction */ + 15 >> 4) * 16;
            sub_40eb94(v19, v1, a0, v1);
        }
        if (g_42b3d5)
        {
            *((unsigned long long *)&ptr[16]) = sub_41101b(v19, 2);
            if (!(long long)ptr[16])
                sub_408de7(a3, gettext("error canonicalizing %s"), v19);
        }
        if (*((int *)&g_42b3ec) <= 3)
        {
            if (*((int *)&g_42b3ec) < 2 || !a3 || (v6 = (unsigned int)(int)sub_405c0e(v19, ptr + 24), v2 = 1, *((int *)&g_42b3ec) != 2 && !(v31 = (v6 < 0 ? (char)(char)(*(__errno_location()) == 2) : (char)(char)(((int)ptr[48] & 0xf000) != 0x4000)), v5 = v31, v5 != 1)))
                goto LABEL_409f7c;
        }
        else
        {
            if (*((int *)&g_42b3ec) == 4)
            {
                v6 = sub_405c0e(v19, ptr + 24);
                v2 = 1;
            }
            else
            {
LABEL_409f7c:
                v6 = sub_405c48(v19, ptr + 24);
                v2 = 0;
            }
        }
        if (v6)
        {
            sub_408de7(a3, gettext("cannot access %s"), v19);
            *((char **)&ptr[176]) = "?";
            if (!a3)
            {
                *((unsigned long long *)ptr) = sub_41cc92(a0);
                g_42b338 = g_42b338 + 1;
                return 0;
            }
            return 0;
        }
        else
        {
            *((char *)&ptr[184]) = 1;
            if ((a1 == 5 || ((short)(int)ptr[48] & 0xf000) == 0x8000) && g_42b3d4 && (char)sub_406325(21))
                *((char *)&ptr[192]) = sub_409936(v19, ptr);
            if (!g_42b3ac || g_42b380)
            {
                v3 = 0;
                v4 = 0;
                v7 = sub_4097fa(v19, ptr, v2);
                v6 = v7 >> 31 & 0xff;
                if (!v6)
                {
                    v3 = (!(char)sub_40525c() ? strcmp("unlabeled", (long long)ptr[176]) : strcmp("_", (long long)ptr[176]));
                }
                else
                {
                    *((char **)&ptr[176]) = "?";
                    if ((char)sub_40524c(*(__errno_location())) || *(__errno_location()) == 61)
                        v6 = 0;
                }
                if (!v6 && !g_42b3ac)
                {
                    v8 = sub_4098ab(v19, ptr);
                    v6 = v8 >> 31 & 0xff;
                    v4 = 0 < v8;
                }
                if (!(v3 || v4))
