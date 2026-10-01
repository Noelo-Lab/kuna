/* A flag check for the Strings list: its messages are loaded directly, and
 * the flag only through the pointer `secret`.
 *
 * Rebuild:  gcc -O0 -g -o crackme.elf crackme.c
 */
#include <stdio.h>
#include <string.h>

static const char *secret = "flag{str1ngs_4re_3asy}";

static int check(const char *input) {
    if (strlen(input) != strlen(secret)) {
        puts("Nope, that is not the flag.");
        return 0;
    }
    return strcmp(input, secret) == 0;
}

int main(void) {
    char buf[64];
    printf("Enter the flag: ");
    if (!fgets(buf, sizeof buf, stdin)) return 1;
    buf[strcspn(buf, "\n")] = 0;
    if (check(buf)) puts("Correct! You found the flag.");
    else puts("Nope, that is not the flag.");
    return 0;
}
