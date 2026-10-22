/* Frame-pointer fixture: built with optimization, so no function keeps a
 * frame pointer, and `read_name` holds its buffer's address in RBP
 * (`LEA RBP,[RSP + 0x...]` or `MOV RBP,RSP`) -- RBP as an ordinary register.
 *
 * Rebuild:  gcc -O2 -fno-stack-protector -fcf-protection=none -o frames-O2.elf frames.c
 */
#include <stdio.h>
#include <string.h>

int read_name(void) {
    char name[64];
    long sum = 0;
    if (!fgets(name, sizeof name, stdin))
        return -1;
    name[strcspn(name, "\n")] = 0;
    for (char *p = name; *p; p++)
        sum += *p;
    puts(name);
    return (int)sum;
}

int main(void) {
    return read_name() > 0;
}
