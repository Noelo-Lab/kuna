// Self-contained Objective-C Mach-O fixture for the s1_objc metadata pass
// (kuna-console/tests/verify_objc.rs). A root class (objc_root_class) so it needs
// NO macOS SDK / Foundation to build — `-x` strips local symbols (incl. the IMP
// `-[Greeter greet:]`) so only the `__objc_*` metadata recovers the method name.
// See fixtures/README.md ("Mach-O Objective-C") for the build recipe + pinned VMAs.
__attribute__((objc_root_class)) @interface Greeter @end
@implementation Greeter
- (int)greet:(int)n { return n*3+7; }
@end
int main(){ return 0; }
