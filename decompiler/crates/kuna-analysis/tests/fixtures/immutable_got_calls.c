static unsigned hits, alternate_hits;
__attribute__((noinline,visibility("hidden"))) long callback(long value) { hits++; return value + 7; }
__attribute__((noinline,visibility("hidden"))) long alternate(long value) { alternate_hits++; return value + 99; }
extern long through_got(long);
extern long through_mutable(long);
extern long through_carried(long);
extern long (*mutable_slot)(long);
int main(void) {
    if (through_got(19) != 26 || hits != 1) return 1;
    mutable_slot = alternate;
    if (through_mutable(19) != 118 || alternate_hits != 1 || hits != 1) return 2;
    if (through_carried(19) != 26 || hits != 2) return 3;
    return 0;
}
