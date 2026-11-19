/* Source for the `macho_bindslots_arm64` fixture.
 *
 * Two pointers in __DATA,__data are bound by dyld to imports the image only
 * names through chained-fixup binds: no indirect-symbol entry names either slot.
 *
 *   clang -target arm64-apple-macos11 -O1 -fno-stack-protector -c macho_bindslots_arm64.c -o t.o
 *   ld64.lld -arch arm64 -platform_version macos 11.0 11.0 -fixup_chains \
 *       -undefined dynamic_lookup -x -e _main -o macho_bindslots_arm64 t.o
 */
extern int ext_handler(int);
extern int ext_counter;
__attribute__((used)) static int (*handler)(int) = ext_handler;
__attribute__((used)) static int *counter = &ext_counter;
__attribute__((noinline)) int dispatch(int v) { return handler(v) + *counter; }
int main(int argc, char **argv) { return dispatch(argc); }
