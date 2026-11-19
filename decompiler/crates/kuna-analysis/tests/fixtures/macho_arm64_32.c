/* Source for the `macho_arm64_32` fixture: a thin arm64_32 (ILP32 AArch64,
 * watchOS) Mach-O. Its header is the 28-byte `mach_header`, its commands are
 * `LC_SEGMENT`, and its pointers are four bytes wide.
 *
 *   clang -target arm64_32-apple-watchos7.0 -O1 -fno-stack-protector -c macho_arm64_32.c -o t.o
 *   ld64.lld -arch arm64_32 -platform_version watchos 7.0 7.0 -e _main -o macho_arm64_32 t.o
 */
int add3(int a, int b) { return a + b + 3; }
int main(int argc, char **argv) { return add3(argc, 7); }
