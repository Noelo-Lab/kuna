/*
 * Source locals and parameters named like kuna's own defaults (`v1`, `v2`), with
 * DWARF.  Their names are name-locked Symbols; the default numbering used to
 * hand `v1` to an unrelated local as well, so the source's `v1` printed as
 * `v1_1` (`widest`'s `long v1`) or a temporary took `v1_1` beside the parameter
 * `v1` (`same_kind`).
 *
 * Build: gcc -O0 -g -fno-pic -fno-asynchronous-unwind-tables -c dwarfvnames_x86_64.c -o dwarfvnames.o
 *        ld --build-id=none -e same_kind dwarfvnames.o -o dwarfvnames_x86_64
 */
struct item {
    int attributes;
};

int same_kind(struct item *v1, struct item *v2)
{
    if (v1 == v2)
        return 1;
    if (v1 == 0 || v2 == 0)
        return 0;
    return !((v1->attributes & 0x40) || (v2->attributes & 0x40));
}

long widest(long *values, int n)
{
    long v1 = values[0];
    long v2 = 0;
    for (int i = 1; i < n; i++) {
        if (values[i] > v1)
            v1 = values[i];
        v2 += values[i] & 1;
    }
    return v1 + v2;
}
