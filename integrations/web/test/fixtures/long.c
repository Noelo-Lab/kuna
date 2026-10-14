/* A long main for the edits that keep the code where it was: its locals are
 * declared at the top and used again far below, so renaming one deep in the
 * function has a first use off screen.
 *
 * Rebuild:  gcc -O0 -o long.elf long.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int score(const char *word) {
    int total = 0;
    for (const char *p = word; *p; p++) total += *p;
    return total;
}

int main(int argc, char **argv) {
    int lines = 0, words = 0, longest = 0, shortest = 1 << 30;
    long sum = 0, best = 0;
    char buf[256];
    FILE *in = argc > 1 ? fopen(argv[1], "r") : stdin;
    if (!in) {
        perror("open");
        return 1;
    }
    while (fgets(buf, sizeof buf, in)) {
        lines++;
        size_t len = strlen(buf);
        if (len && buf[len - 1] == '\n') buf[--len] = 0;
        if ((int)len > longest) longest = (int)len;
        if ((int)len < shortest) shortest = (int)len;
        for (char *tok = strtok(buf, " \t"); tok; tok = strtok(NULL, " \t")) {
            words++;
            long s = score(tok);
            sum += s;
            if (s > best) best = s;
            if (s % 7 == 0) printf("lucky word %s\n", tok);
            if (s % 11 == 0) printf("odd word %s\n", tok);
            if (s % 13 == 0) printf("rare word %s\n", tok);
        }
        if (lines % 100 == 0) printf("%d lines so far\n", lines);
    }
    if (in != stdin) fclose(in);
    printf("lines: %d\n", lines);
    printf("words: %d\n", words);
    printf("longest: %d\n", longest);
    printf("shortest: %d\n", lines ? shortest : 0);
    printf("sum: %ld\n", sum);
    printf("best: %ld\n", best);
    if (words) printf("mean: %ld\n", sum / words);
    if (lines) printf("words per line: %d\n", words / lines);
    if (best > 1000) puts("a very long word");
    if (best > 500) puts("a long word");
    if (best > 100) puts("a word");
    if (sum > 100000) puts("a lot of text");
    if (sum > 10000) puts("some text");
    if (sum > 1000) puts("a little text");
    if (longest > 200) puts("a very long line");
    if (longest > 100) puts("a long line");
    if (longest > 50) puts("a line");
    if (shortest == 0) puts("an empty line");
    if (lines > 1000) puts("many lines");
    if (lines > 100) puts("some lines");
    if (lines > 10) puts("a few lines");
    if (words > 1000) puts("many words");
    if (words > 100) puts("some words");
    if (words > 10) puts("a few words");
    return lines > 0 ? 0 : 2;
}
