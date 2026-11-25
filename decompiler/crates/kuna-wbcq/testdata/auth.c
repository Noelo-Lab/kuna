/* The three-state authentication parser of Fission RFC #1093 section 9,
 * written from its prose: IDLE -> CHALLENGE on input byte 0x5a, CHALLENGE ->
 * AUTH_OK on 0xa5 (result 1) and back to IDLE on anything else, AUTH_OK
 * absorbing (result 1). `acc` sums every input and nothing reads it.
 *
 * auth_step is the plain form. auth_flat_step is the same machine behind a
 * flattened dispatcher with artificial state numbers and the opaque predicate
 * (x * (x + 1)) & 1, which is never true, guarding a decoy that would
 * clobber the mode. Built by build.sh. */

struct auth_ctx {
    unsigned mode;
    unsigned acc;
};

enum { IDLE, CHALLENGE, AUTH_OK };

unsigned auth_step(struct auth_ctx *c, unsigned cmd)
{
    unsigned char byte = (unsigned char)cmd;
    c->acc += cmd;
    switch (c->mode) {
    case IDLE:
        if (byte == 0x5a)
            c->mode = CHALLENGE;
        return 0;
    case CHALLENGE:
        if (byte == 0xa5) {
            c->mode = AUTH_OK;
            return 1;
        }
        c->mode = IDLE;
        return 0;
    default:
        return 1;
    }
}

unsigned auth_flat_step(struct auth_ctx *c, unsigned cmd)
{
    volatile unsigned opaque_seed = cmd;
    unsigned x = opaque_seed;
    unsigned char byte = (unsigned char)cmd;
    unsigned result = 0, junk = 0;
    unsigned s = 0x7a31;
    c->acc += cmd;
    for (;;) {
        switch (s) {
        case 0x7a31:
            if ((x * (x + 1)) & 1) {
                s = 0xc442;
                break;
            }
            s = c->mode == IDLE ? 0x3b77 : c->mode == CHALLENGE ? 0x1f09 : 0x99e1;
            break;
        case 0x3b77:
            if (byte == 0x5a)
                c->mode = CHALLENGE;
            s = 0x99e0;
            break;
        case 0x1f09:
            if (byte == 0xa5) {
                c->mode = AUTH_OK;
                result = 1;
            } else {
                c->mode = IDLE;
            }
            s = 0x99e0;
            break;
        case 0x99e1:
            result = 1;
            s = 0x99e0;
            break;
        case 0xc442:
            junk = (x << 5 | x >> 27) ^ 0x1337;
            s = 0x5d12;
            break;
        case 0x5d12:
            junk = junk * 31 + c->mode;
            c->mode = junk & 3;
            s = 0x99e0;
            break;
        case 0x99e0:
            return result;
        }
    }
}
