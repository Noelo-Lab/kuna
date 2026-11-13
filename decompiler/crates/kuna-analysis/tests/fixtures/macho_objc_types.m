// Self-contained Objective-C Mach-O fixture for class-typed declarations:
// a property whose attributes name a class of this image (`T@"Base"`) types
// the synthesized getter/setter and the ivar behind it with that class's
// struct, so a field read through the result renders. Root classes, no SDK;
// see fixtures/README.md ("Mach-O Objective-C types").
__attribute__((objc_root_class)) @interface Base {
  Class isa;
  int count;
}
@property (nonatomic) int count;
@end
@implementation Base
@synthesize count = count;
@end

@interface Holder : Base {
  Base *inner;
}
@property (nonatomic, assign) Base *inner;
- (int)innerCount;
@end
@implementation Holder
@synthesize inner = inner;
- (int)innerCount { return inner->count + [inner count] + [[self inner] count]; }
@end

int main(void) { return 0; }
