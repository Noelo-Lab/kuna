/* operand_refs fixture: a short rodata string ("hi", < 5 chars, so the
 * always-on StringLiteralPass (min_len 5) does NOT plant it) is loaded by its
 * address and passed to a function with NO known prototype (mystery). With
 * `--option operand_refs on` the scalar immediate that is &"hi" is typed as a
 * read-only char pointer so the call renders mystery("hi"); default-off it
 * renders the bare address. Kept -no-pie so the string is a fixed absolute VMA.
 */
__attribute__((noinline)) int mystery(const char *p);

int mystery(const char *p) { return p[0] + p[1]; }

int main(void) {
    const char *p = "hi";
    return mystery(p);
}
