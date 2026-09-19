/* Fixture for the `structsynth` layout ledger's convergence sweep under
 * `kuna decompile-all` (PR: share one layout across functions by subsumption).
 *
 * Three readers of one record, in address order:
 *
 *   fb  claims {0: char *, 8: int, 0xc: int, 0x10: long}
 *   fa  claims {0: char *, 8: int}               -- two of fb's four
 *   fc  claims fb's four plus 0x18, 0x20, 0x28, 0x30
 *
 * `fb` mints a structure and `fa` reuses it. `fc` then mints the larger one and
 * supersedes it, and the sweep after the batch moves `fb` onto `fc`'s structure.
 * `fc` claims four times what `fa` measured, past the ledger's growth bound, so
 * `fa` keeps the structure it was first given. A fresh name for `fa` would be
 * superseded by that same structure the moment it existed, so none is minted:
 * the program has two names for the record and never a third.
 *
 * Build (fixed VMAs, symbols kept so the probe can name the functions):
 *   gcc -O2 -no-pie -fno-stack-protector -fcf-protection=none -fno-inline \
 *       -fno-reorder-functions -o structsynthchain_x86_64 structsynthchain_x86_64.c
 */
struct S { char *p; int a; int b; long c; short d; long e; char f; long g; };

__attribute__((noinline)) long fb(struct S *s) { return *s->p + s->a + s->b + s->c; }
__attribute__((noinline)) long fa(struct S *s) { return *s->p + s->a; }
__attribute__((noinline)) long fc(struct S *s)
{
    return *s->p + s->a + s->b + s->c + s->d + s->e + s->f + s->g;
}

long (*tab[])(struct S *) = { fb, fa, fc };

int main(int argc, char **argv)
{
    long t = 0;
    for (int i = 0; i < 3; i++)
        t += tab[i]((struct S *)argv[i % argc]);
    return (int)t;
}
