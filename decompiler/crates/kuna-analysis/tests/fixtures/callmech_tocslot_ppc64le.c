/* clang --target=powerpc64le-linux-gnu -O2 -c callmech_tocslot_ppc64le.c
 * -o callmech_tocslot_ppc64le.o: v[1] lives at 0x28(r1), the ELFv1 TOC slot. */
extern void use(long *);
extern int ext(int);
long two_calls(long a)
{
    long v[2];
    v[0] = a;
    v[1] = a * 9;
    use(v);
    ext(1);
    return v[0] + v[1];
}
