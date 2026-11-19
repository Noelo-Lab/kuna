// Self-contained Mach-O fixture for by-value aggregates in type encodings and
// global blocks (tests/stages/kuna-objcstructs.xml): an ivar and a method
// argument of struct type (`{CGRect={CGPoint=dd}{CGSize=dd}}`), an array ivar
// (`[4i]`), and a global block literal (`_NSConcreteGlobalBlock`) whose
// descriptor carries a signature. `-x` strips local symbols. See
// fixtures/README.md ("Mach-O ObjC structs and blocks") for the build recipe.
struct CGPoint { double x; double y; };
struct CGSize { double width; double height; };
struct CGRect { struct CGPoint origin; struct CGSize size; };
typedef struct CGRect CGRect;
__attribute__((objc_root_class))
@interface Shape { CGRect frame; int counts[4]; long total; }
- (void)setFrame:(CGRect)r;
- (double)area;
- (int)countAt:(int)i;
@end
@implementation Shape
- (void)setFrame:(CGRect)r { frame = r; total += 1; }
- (double)area { return frame.size.width * frame.size.height; }
- (int)countAt:(int)i { return counts[i & 3] + (int)total; }
@end
long (^scale)(long) = ^long(long x) { return x * 3; };
long apply(long v) { return scale(v); }
int main(void) { return 0; }
