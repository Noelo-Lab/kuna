/* Fixture for the `libctypes` stdio stream data symbols
 * (tests/stages/kuna-libctypes.xml, passes 6-8).
 *
 * Built: gcc -O1 -o libctypes_streams_x86_64 libctypes_streams_x86_64.c
 *        gcc -O1 -shared -fPIC -o libctypes_streams_so_x86_64.so \
 *                                 libctypes_streams_x86_64.c
 *        gcc -O1 -shared -fPIC -DKUNA_STREAM_NAME_COLLISION \
 *                -o libctypes_streams_collide_so_x86_64.so \
 *                   libctypes_streams_x86_64.c
 *        (no `-g`, on purpose — the point is what the image says WITHOUT
 *         debug info)
 *
 * The same source is built twice because the two link kinds bind a stream
 * differently, and the two slots hold different things:
 *
 *   - the executable gets an `R_X86_64_COPY` relocation and a `.bss` word
 *     holding libc's own `FILE *stdout`;
 *   - the shared object gets an `R_X86_64_GLOB_DAT` relocation on an
 *     UNDEFINED `stdout` and a GOT word holding its ADDRESS.
 *
 * `pending` is the witness: it reads two fields of `stdout` and calls nothing,
 * so the relocation is the only evidence in the image about what the word
 * holds. `hand_off` and `say` are the controls — the stream OBJECT read, once
 * into one of the image's own functions and once into a libc call whose
 * prototype already names it. `flush_mark` stores THROUGH the stream pointer.
 *
 * `noinline` keeps each one out of `main`, so the stage script can load it by
 * name out of `.symtab`.
 *
 * The third build adds a global of the image's own spelled `stdout_ptr` — the
 * one name kuna MINTS rather than reads. It is `static`, so it lands in
 * `.symtab` and not in `.dynsym`, which is the half a `.dynsym`-only lookup
 * would miss. `stdin` is untouched there, so the decline has to be per name.
 */
#define _GNU_SOURCE
#include <stdio.h>

#ifdef KUNA_STREAM_NAME_COLLISION
static long stdout_ptr = 7;
long *keep_stdout_ptr = &stdout_ptr;
__attribute__((noinline)) long own_ptr(void) { return stdout_ptr; }
#endif

__attribute__((noinline)) long pending(void)
{
    return (long)(stdout->_IO_write_ptr - stdout->_IO_write_base);
}

__attribute__((noinline)) void flush_mark(char *p)
{
    stdout->_IO_write_ptr = p;
}

__attribute__((noinline)) int sink(void *p) { return p != 0; }

__attribute__((noinline)) int hand_off(void) { return sink(stdin); }

__attribute__((noinline)) int say(const char *s) { return fputs(s, stdout); }

int main(int argc, char **argv)
{
    (void)argc;
    flush_mark(argv[0]);
    return (int)pending() + hand_off() + say("x");
}
