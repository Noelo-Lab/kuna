unsigned ordinary(unsigned index, unsigned value)
{
    switch (index) {
    case 0: return value - 31;
    case 1: return value << 3;
    case 2: return value ^ 15;
    case 3: return 63 - value;
    case 4: return value | 2;
    case 5: return value + 13;
    default: return 42;
    }
}
