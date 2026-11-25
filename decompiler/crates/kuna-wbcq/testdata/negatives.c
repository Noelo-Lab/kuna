/* Step functions outside the model, each of which must fail closed rather
 * than yield a machine: built by build.sh with auth.c. */

struct neg_ctx {
    unsigned mode;
    unsigned acc;
};

/* The result is the input: no finite machine over path classes. */
unsigned echo_step(struct neg_ctx *c, unsigned cmd)
{
    c->acc += 1;
    return cmd;
}

/* 100 states, past the default bound of 32. */
unsigned counter_step(struct neg_ctx *c, unsigned cmd)
{
    (void)cmd;
    c->mode = c->mode == 99 ? 0 : c->mode + 1;
    return c->mode == 0;
}

unsigned global_flag;

/* State outside the context. */
unsigned global_step(struct neg_ctx *c, unsigned cmd)
{
    if ((unsigned char)cmd == 0x5a)
        global_flag = 1;
    return global_flag + c->mode;
}

unsigned helper(unsigned x);

/* A call. */
unsigned call_step(struct neg_ctx *c, unsigned cmd)
{
    return helper(cmd) + c->mode;
}

unsigned helper(unsigned x)
{
    return x * 3;
}
