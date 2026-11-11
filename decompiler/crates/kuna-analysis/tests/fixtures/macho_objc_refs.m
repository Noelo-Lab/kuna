// Self-contained Objective-C Mach-O fixture for the ObjC reference-slot naming
// (`__objc_selrefs` / `__objc_classrefs` / `__objc_superrefs` / `__objc_stubs`).
// Root classes (objc_root_class) so it needs NO macOS SDK / Foundation; `-x`
// strips local symbols so only the `__objc_*` metadata names anything.
// See fixtures/README.md ("Mach-O Objective-C references") for the build recipe.
__attribute__((objc_root_class)) @interface Base { Class isa; }
+ (int)tag;
- (int)greet:(int)n;
- (int)count;
@end
@implementation Base
+ (int)tag { return 11; }
- (int)greet:(int)n { return n*3+7; }
- (int)count { return 4; }
@end
@interface Derived : Base @end
@implementation Derived
- (int)greet:(int)n { return [super greet:n] + 1; }
@end
int use(Base *b) { return [b greet:2] + [b count]; }
int tag(void) { return [Derived tag]; }
int use2(Base *b) { return [b greet:5]; }
__attribute__((objc_root_class)) @interface Remote
+ (int)tag;
@end
int remote(void) { return [Remote tag]; }
int main(){ return 0; }
