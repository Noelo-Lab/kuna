extern char ext_table[];
static const char letters[32] = "abcdefghijklmnopqrstuvwxyz";

unsigned int read_reg(unsigned long off)
{
    return *(volatile unsigned int *)(0x40020000UL + off);
}

int letter_rank(long c) { return letters[c - 'a']; }

int get_ext(long i) { return ext_table[i]; }
