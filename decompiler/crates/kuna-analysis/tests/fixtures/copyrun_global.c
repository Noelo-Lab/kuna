/* Fixture for `copyrun` beside a global the copy's stores may alias.
   gcc -O2 -fno-pic -nostdlib -static -Wl,-e,copy_guarded copyrun_global.c
     -o copyrun_global_x86_64 */
struct big { long a[8]; };
long token;
__attribute__((noipa)) void init_token(long *t) { *t = -1; }
__attribute__((noipa)) void consume(struct big *b, long t) { b->a[0] = t; }
void copy_guarded(struct big *d, const struct big *s) {
    if (token != -1)
        init_token(&token);
    *d = *s;
    consume(d, token);
}
