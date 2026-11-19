
#include <stdio.h>
#include <string.h>
int checksum(const char *msg, int rounds) {
    int total = 0;
    for (int i = 0; i < rounds; i++) {
        char scratch[32];
        strncpy(scratch, msg, sizeof scratch - 1);
        scratch[31] = 0;
        int partial = (int)strlen(scratch) * (i + 1);
        total += partial;
    }
    if (rounds > 3) {
        long bonus = total * 7L;
        total += (int)bonus;
    }
    return total;
}
int main(int argc, char **argv) { printf("%d\n", checksum(argv[0], argc)); return 0; }
