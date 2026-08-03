/* Fixture source for the PE import-call binding gate (`--option peimportcall`).
 *
 * `bail` ends in a `call [__imp_ExitProcess]` — a *direct indirect* call through
 * the Import Address Table slot, the shape MSVC and `__declspec(dllimport)`
 * always emit and the shape kuna could not resolve: the CALLIND target is the
 * contents of a global, so `ActionDeindirect` needs `Varnode::externref` to bind
 * it to the `ExitProcess` FunctionSymbol the PE loader already registered at the
 * slot.  Unbound, the call has no no-return flow effect and `bail` runs straight
 * on into `tally`, absorbing its whole loop body.
 *
 * `tally` is deliberately the next function in `.text` and deliberately contains
 * a loop, so the overrun is visible in one line of C.
 *
 * Build (see this directory's README.md):
 *   docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc \
 *     'x86_64-w64-mingw32-gcc -O1 -nostdlib -Wl,-e,entry \
 *        decompiler/crates/kuna-analysis/tests/fixtures/pe_noreturn_import.c \
 *        -o decompiler/crates/kuna-analysis/tests/fixtures/pe_noreturn_import.exe \
 *        -lkernel32'
 */
__declspec(dllimport) void __stdcall ExitProcess(unsigned int code);

int tally(int n, int step);

void bail(int code)
{
    ExitProcess((unsigned int)code);
}

int tally(int n, int step)
{
    int sum = 0;
    int i;
    for (i = 0; i < n; i++)
        sum += step * i;
    return sum;
}

int entry(void)
{
    int r = tally(3, 4);
    if (r == 12)
        bail(r);
    return r;
}
