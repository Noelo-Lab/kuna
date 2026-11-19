/* dwarfnocall fixture: clang -O2 -g -fno-pic -no-pie -nostdlib -fno-stack-protector
 *   -fno-vectorize -fno-slp-vectorize -Wl,-e,main -Wl,--build-id=none
 * Clang's dead-argument elimination removes `unused` from work and the
 * constant `b` from pick, marks both DW_CC_nocall, and keeps every source
 * parameter in the DWARF; the removed ones carry no DW_AT_location. */
volatile int sink;

__attribute__((noinline)) static int work(int *p, int unused, int k) {
    int s = 0;
    for (int i = 0; i < k; i++) s += p[i] * i;
    return s;
}

__attribute__((noinline)) static int pick(int a, int b) {
    sink = a + b;
    return a;
}

int main(int argc, char **argv) {
    int v[4] = {argc, 2, 3, 4};
    pick(argc, 3);
    sink = work(v, 99, argc + 2);
    return 0;
}
