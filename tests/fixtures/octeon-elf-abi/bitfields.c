/* Original witnesses for mixed floating-field/bitfield return classification. */
struct DoubleBit { double d; unsigned b:1; };
struct BitDouble { unsigned b:1; double d; };
struct DoubleBit double_bit(struct DoubleBit x) { return x; }
struct BitDouble bit_double(struct BitDouble x) { return x; }
