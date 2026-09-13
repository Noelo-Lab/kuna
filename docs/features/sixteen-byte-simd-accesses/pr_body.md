kuna swaps C's pointer-to-array and array-of-pointers spellings: a pointer to a 16-byte block is cast as an array of 16 pointers, which is not C, and a declared pointer to an array loses its array. The `--json` variable types have the same inversion and list nested array dimensions backwards (`char[16][2]` for two 16-byte arrays).

```
$ cat > t.c <<'EOF'
static char rows[4][16];
char (*last)[16];
char (*get_row(int i))[16] { return &rows[i & 3]; }
int use_rows(int i) {
  char (*row)[16] = get_row(i);
  last = get_row(i + 1);
  return (*row)[3] + (*last)[2];
}
int main(int argc, char **argv) { return use_rows(argc); }
EOF
$ gcc -O2 -g -fno-inline -o t t.c
$ kuna decompile t get_row
char * get_row(int i)
{
  return (char *[16])&(&rows)[(unsigned long)(i & 3) * 4];
}
$ kuna decompile t use_rows
int use_rows(int i)
{
  char *v1; // rax
  ...
```

With this change the same commands print `char (* get_row(int i))[16]`, `(char (*)[16])` and `char (*v1)[16]; // rax`, and the two functions compile with `gcc -Wall` given the two globals. The same cast shape appears on SIMD loads and stores in MSVC `/O2` code (`*(char *[16])&v21[v14]`).

## Fix

- The C declarator builder walks the modifiers from the outermost type to the base and, before an array suffix, parenthesises the whole declarator built so far: `T (*p)[N]`, `T *p[N]`, `T (*p[2])[3]`, `T p[2][16]`.
- Declarations place both halves around the name: a local prints `)[N]` after its name and any `[count]` (`char (*pair [2])[16];`), a return type prints it after the parameter list, and the duplicate-declaration check compares it too.
- No option: every old spelling named a different type or was not C. Across 637 binaries (186,030 functions) the 42,472 changed lines only move declarator parentheses and stars or add the missing `[N]` to a declaration; Rust output is unchanged.

## Tests

- `verify_ptrarray_declarators` on a new `-O2 -g` fixture: pointer-to-array return types, a pointer-to-array local, an array of them, and an array-of-pointers control that must stay unparenthesised.
- Unit tests pin the mirror and nested spellings and the declaration halves; a parser round trip covers every pointer/array nesting up to four modifiers.
- `verify_return_uncomputed` pins `passthru(char (*a0)[16])`; no datatest or stage expectation moved.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01Ec4F113Qm7hnvUycXdFLbL
