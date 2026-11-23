extern unsigned wrap_mix(unsigned x);
extern unsigned wrap_sum(const unsigned char *p);
extern unsigned wrap_seven(unsigned x);
extern unsigned twin_one(unsigned x);
extern unsigned twin_two(unsigned x);

void _start(void)
{
    static const unsigned char text[] = "kuna flirt";
    unsigned r = wrap_mix(5) + wrap_sum(text) + wrap_seven(2) + twin_one(1) + twin_two(2);
    __asm__ volatile("syscall" : : "a"(60), "D"(r & 0xff));
    __builtin_unreachable();
}
