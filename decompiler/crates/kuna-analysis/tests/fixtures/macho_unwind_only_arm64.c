/* Source for the `macho_unwind_only_arm64` fixture.
 *
 * A linked arm64 Mach-O whose LC_FUNCTION_STARTS was stripped (-no_function_starts)
 * and whose local symbols were stripped (ld64.lld -x), leaving only __mh_execute_header
 * and _main. The four `uw_*` functions are kept in the image (`used`) but have NO
 * direct caller and NO symbol, so the entry-discovery oracles (LC_MAIN, exports,
 * __mod_init_func, symtab) never name them and the arm64 walk cannot reach them.
 * Each uses a different stack size so its compact-unwind encoding differs and the
 * linker does not fold the runs: __TEXT,__unwind_info records all four starts.
 *
 * It is the `unwindstarts` witness: with the option off the inventory is
 * __mh_execute_header + _main; with `--option unwindstarts on` the four uw_* starts
 * are recovered from __unwind_info. macho_imports is the negative case (it keeps
 * LC_FUNCTION_STARTS, so the oracle is a strict no-op there).
 *
 * Rebuild (needs clang + ld64.lld with a Mach-O arm64 target):
 *
 *   clang -target arm64-apple-macos11 -O1 -fno-stack-protector -funwind-tables \
 *       -c macho_unwind_only_arm64.c -o t.o
 *   ld64.lld -arch arm64 -platform_version macos 11.0 11.0 \
 *       -no_function_starts -x -e _main -o macho_unwind_only_arm64 t.o
 */
__attribute__((used,noinline)) static int uw_alpha(int n){ volatile char b[16];  for(int i=0;i<16;i++)  b[i]=(char)(n+i); int s=0; for(int i=0;i<16;i++)  s+=b[i]; return s; }
__attribute__((used,noinline)) static int uw_beta (int n){ volatile char b[64];  for(int i=0;i<64;i++)  b[i]=(char)(n^i); int s=0; for(int i=0;i<64;i++)  s+=b[i]; return s; }
__attribute__((used,noinline)) static int uw_gamma(int n){ volatile char b[176]; for(int i=0;i<176;i++) b[i]=(char)(n*i); int s=0; for(int i=0;i<176;i++) s+=b[i]; return s; }
__attribute__((used,noinline)) static int uw_delta(int n){ volatile char b[320]; for(int i=0;i<320;i++) b[i]=(char)(n-i); int s=0; for(int i=0;i<320;i++) s+=b[i]; return s; }
int main(int argc, char**argv){ volatile int r=argc*7; r+=11; return r; }
