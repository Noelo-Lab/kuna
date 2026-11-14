extern unsigned char sample_table[23];
extern short sample_halves[11];
#ifdef SLOT32
 typedef int slot_type;
#else
 typedef long slot_type;
int lookup32(unsigned long slot);
int lookup64(unsigned long slot);
int lookup63(unsigned long slot);
#endif
int lookup17(slot_type slot);
int lookup53(slot_type slot);
int lookup211(slot_type slot);
int lookup_half(slot_type slot);
void store53(slot_type slot, signed char value);

int main(void) {
    for (long i = 0; i < 23; ++i) {
        int expected = (signed char)sample_table[i];
        if (lookup17(i + 17) != expected || lookup53(i + 53) != expected ||
            lookup211(i + 211) != expected)
            return 1;
#ifndef SLOT32
        unsigned long index = (unsigned long)i;
        if (lookup32(index + 0x80000035ul) != expected ||
            lookup64(index + 0x1000000d3ul) != expected ||
            lookup63(index + 0x7fffffffffffffe0ul) != expected)
            return 2;
#endif
    }
    for (long i = 0; i < 11; ++i)
        if (lookup_half(i + 53) != sample_halves[i])
            return 3;
    unsigned char snapshot[23];
    for (long i = 0; i < 23; ++i)
        snapshot[i] = sample_table[i];
    for (long i = 0; i < 23; ++i) {
        for (int value = -128; value < 128; ++value) {
            store53(i + 53, (signed char)value);
            for (long j = 0; j < 23; ++j)
                if (sample_table[j] != (i == j ? (unsigned char)value : snapshot[j]))
                    return 4;
            sample_table[i] = snapshot[i];
        }
    }
    return 0;
}
