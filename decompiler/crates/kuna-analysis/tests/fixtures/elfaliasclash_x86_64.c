/* (kuna) `shared` is spelled twice: as a global alias of `twin` here, and as a
 * static function in elfaliasclash_b_x86_64.c. Built with
 *   gcc -O1 -o elfaliasclash_x86_64 elfaliasclash_x86_64.c elfaliasclash_b_x86_64.c
 * See decompiler/crates/kuna-analysis/tests/fixtures/README.md. */
int twin(int x) { return x * 3 + 1; }
extern int shared(int x) __attribute__((alias("twin")));
int use_b(int);
int main(int c, char **v) { return use_b(c) + twin(c); }
