cliw core_product()
{
    a0 = a1l * a2l || nop || nop || nop || nop || nop;
}

code _issue_arithmetic()
{
    a0l = a0l + 1 || a1l = a0l;
    a2h = 0;
    a3l = -291;
    *r0 = 0x1234;
    *r0++ = -29108;
    nop;
    return;
}

code _issue_control()
{
    goto skip;
    a0l = a0l + 1;
skip:
    call callee;
    return;
callee:
    a1l = a1l + 1;
    return;
}

code _issue_cliw()
{
    cliw core_product();
    return;
}
