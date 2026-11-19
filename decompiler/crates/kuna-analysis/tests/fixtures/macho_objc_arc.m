// Self-contained Mach-O fixture for ARC call handling (`objcarc`,
// tests/stages/kuna-objcarc.xml): alloc/init chains, retain/release traffic,
// autoreleased return values and an ivar store, built with -fobjc-arc so the
// compiler emits the runtime calls. `-x` strips local symbols so only the
// `__objc_*` metadata and the import stubs name anything. See
// fixtures/README.md ("Mach-O ARC") for the build recipe.
#import <stdint.h>
__attribute__((objc_root_class))
@interface Base { long count; Base *inner; }
+ (instancetype)alloc;
- (instancetype)init;
- (long)count;
- (void)setInner:(Base *)other;
- (Base *)inner;
@end
@implementation Base
+ (instancetype)alloc { return 0; }
- (instancetype)init { return self; }
- (long)count { return count; }
- (void)setInner:(Base *)other { inner = other; }
- (Base *)inner { return inner; }
@end
__attribute__((objc_root_class))
@interface Holder { Base *slot; }
- (Base *)make;
- (long)total:(Base *)b;
- (void)keep:(Base *)b;
@end
@implementation Holder
- (Base *)make { Base *b = [[Base alloc] init]; [b setInner:[[Base alloc] init]]; return b; }
- (long)total:(Base *)b { return [b count] + [[b inner] count]; }
- (void)keep:(Base *)b { slot = b; }
@end
int main(void) { return 0; }
