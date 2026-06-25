/* Source for the stripped Mach-O fixture: `helper` is static (file-local) so
 * `ld64.lld -x` removes its symbol, leaving LC_FUNCTION_STARTS as the ONLY
 * source that recovers it.  `main` stays the entry (LC_MAIN). */
int printf(const char *fmt, ...);
static int helper(int n) { return n * 7 + 3; }
int main(int argc, char **argv) {
    printf("%d\n", helper(argc));
    return 0;
}
