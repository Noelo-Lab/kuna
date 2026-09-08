## The problem

Naming a callee tells kuna what it is, and kuna does nothing with the name. On a
stripped, statically linked i386 crackme the anti-debug state machine calls
glibc's `ptrace`; declaring that callee and turning on both call-argument
recovery options still prints the call with no arguments, with the four pushed
slots stranded as raw stores on the lines above it:

```
$ kuna decompile ./crackme sub_80483b0 --define-function 0x8048968=ptrace \
      --option calleearity on --option varargstackargs on
        *(unsigned int *)((int)v14 + -4) = v12[v7];
        *(int *)((int)v14 + -8) = v5;
        *(int *)((int)v14 + -0xc) = dat_804a4ac;
        *(unsigned int *)((int)v14 + -0x10) = 6;
        v16 += ptrace();
```

Thirteen call sites in that one function look like that. The arity cannot come
from the callee's body: glibc declares `long ptrace(enum __ptrace_request, ...)`
and fetches the rest with `va_arg`, so it can only come from a signature table —
and kuna's two tables match a name the *image* carries, which a stripped image
does not.

## The fix

- A declared function name (`--define-function START=NAME`, `--assert function`,
  console `function bounds ... as NAME`) is now looked up in the built-in libc
  signature tables at declaration time, and the matching signature parked on the
  entry. New option `declaredlibcproto`, default on (DIV-139).
- Parked by **entry address**, not by name: that is the key
  `ActionDefaultParams` reads a callee prototype back by, and the only key that
  survives two symbols sharing a spelling.
- Both tables are searched, `libcsigs`' imports-only restriction included. That
  restriction exists so a *coincidental* spelling cannot retype a function the
  image defines itself — a judgement about evidence, and the evidence is
  different once someone has identified the entry outright.
- `ptrace` added to the base table. The four fixed slots are glibc's own: its
  wrapper fetches `pid_t`, `void *`, `void *` after the request with `va_arg`.

The witness now reads
`ptrace(v18[-0xc],v18[-0xb],(void *)v18[-10],(void *)v18[-9])` at every site. An
explicit `--assert prototype` still wins; `--option declaredlibcproto off`
declares the name alone.

## The tests

`tests/stages/kuna-declaredlibcproto.xml` is the two-pass case — the callee is
named in both passes (`sub_8049027` appears zero times), so the only thing that
moves is the signature. `tests/cli/both-argument-recovery-options.json` and three
cargo tests in `kuna-console/tests/verify_declaredlibcproto.rs` run the same
shape over the vendored `declaredlibcproto_i386` fixture, including a name
neither table knows, which must park nothing.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
