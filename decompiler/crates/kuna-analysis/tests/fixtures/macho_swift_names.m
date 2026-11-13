// Self-contained Mach-O fixture for Swift name demangling (`swiftdemangle`,
// tests/stages/kuna-swiftdemangle.xml). No Swift toolchain is needed: the
// Objective-C root class carries a Swift runtime name through
// `objc_runtime_name` (what swiftc emits for an @objc class `main.Greeter`),
// and the exported C function carries a Swift 5 mangled symbol through an
// `asm` label (`main.countWords(of: Swift.Int) -> Swift.Int`). `-x` strips
// local symbols so only the `__objc_*` metadata recovers the method name.
// See fixtures/README.md ("Mach-O Swift names") for the build recipe.
__attribute__((objc_root_class))
__attribute__((objc_runtime_name("_TtC4main7Greeter")))
@interface Greeter @end
@implementation Greeter
- (int)greet:(int)n { return n * 3 + 7; }
@end
int count_words(int n) __asm__("_$s4main10countWords2ofS2i_tF");
int count_words(int n) { return n * 2 + 1; }
int main(void) { return 0; }
