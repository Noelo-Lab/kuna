/*
 * inputparamgap fixture (x86-64 ELF, non-PIE): a callback whose ABI signature is
 * fixed by the API it is registered with, ignores its middle parameters, and is
 * reachable ONLY through a function-pointer table -- so nothing but its own body
 * can say what its prototype is.
 *
 * Build (reproducible; the committed binary was produced by this exact line):
 *   gcc -O2 -no-pie -fno-pic -fcf-protection=none -fno-stack-protector \
 *       -fno-asynchronous-unwind-tables -fno-unwind-tables \
 *       -o inputparamgap_x86_64 inputparamgap_x86_64.c
 *
 * `on_key` is the shape of the Wayland `wl_keyboard_listener` key handler the
 * need was filed against (`lugosiii` sub_6500): SysV puts `data` in rdi, ignores
 * rsi/rdx/rcx (`kb`, `serial`, `time`) and delivers `key` in r8d and `state` in
 * r9d -- a THREE-register hole, one past `forceInactiveChain`'s maxchain of 2.
 * Without `inputparamgap` the recovered prototype is `void on_key(long a0)` and
 * the body reads `int v1; // r8d` / `int v2; // r9d` with nothing assigning them.
 *
 * `on_two` is the control: identical in every respect except that its hole is
 * TWO registers wide (rsi/rdx), which the upstream rule already tolerates. It
 * recovers its full four-parameter signature under BOTH settings, which is what
 * shows the option changes the gap limit and nothing else.
 */

volatile int sel;                 /* defeats constant folding of the table index */

struct ctx { long pad[13]; long xkb; int mode; };   /* xkb at +0x68, mode at +0x70 */

__attribute__((noinline)) static int lookup(long xkb, int code)
{
    return (int)(xkb >> 3) ^ (code * 31);
}

/* 6-parameter callback; rsi/rdx/rcx are ignored -> three-register hole. */
__attribute__((noinline))
static void on_key(void *data, void *kb, unsigned serial, unsigned time,
                   unsigned key, unsigned state)
{
    struct ctx *c = (struct ctx *)data;
    if (state != 1 || !c->xkb)
        return;
    c->mode = lookup(c->xkb, (int)key + 8);
}

/* 4-parameter control; rsi/rdx are ignored -> two-register hole. */
__attribute__((noinline))
static void on_two(void *data, void *kb, unsigned serial, unsigned key)
{
    struct ctx *c = (struct ctx *)data;
    if (!c->xkb)
        return;
    c->mode = lookup(c->xkb, (int)key + 3);
}

typedef void (*key_fn)(void *, void *, unsigned, unsigned, unsigned, unsigned);
typedef void (*two_fn)(void *, void *, unsigned, unsigned);

/* Both handlers appear ONLY here, so no static CALL names either of them. */
static const key_fn key_listener[2] = { 0, on_key };
static const two_fn two_listener[2] = { 0, on_two };

int main(void)
{
    static struct ctx c;
    int i = sel & 1;
    c.xkb = (long)&c + 0x40;
    key_listener[i](&c, 0, 1, 2, 3, 4);
    two_listener[i](&c, 0, 1, 2);
    return c.mode;
}
