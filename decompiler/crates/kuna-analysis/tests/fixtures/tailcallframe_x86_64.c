/* Minimal reproduction of the round-2 RE-friction shape: a callback reachable
 * ONLY through a code pointer in initialized data, whose -O2 tail jump lands in
 * a routine no discovery oracle finds.  See tests/stages/tailcallframe.xml for
 * the byte-level twin and docs/spec/02-lift-and-flow.md for the rule. */
typedef __SIZE_TYPE__ size_t;
extern size_t strlen(const char *);

struct S { int key; int state; int w; int h; char msg[48]; };

__attribute__((noinline)) static void render(struct S *s)
{
    s->w = s->key * 7 + 3;
    s->h = s->key * 11 + 5;
    __builtin_memcpy(s->msg, "RENDERER-BODY-MARKER-0123456789", 32);
    s->state = 2;
}

__attribute__((noinline)) static void on_key(struct S *s, int k)
{
    s->key = k;
    if (k <= 0) { s->state = 0; return; }
    s->state = (int)strlen(s->msg) + 1;
    render(s);
}

typedef void (*cb_t)(struct S *, int);
static cb_t const listener[1] = { on_key };
volatile cb_t const *volatile sink = listener;

int main(void)
{
    static struct S s;
    cb_t f = ((cb_t const *)sink)[0];
    f(&s, 3);
    return s.state + s.w + s.h;
}
