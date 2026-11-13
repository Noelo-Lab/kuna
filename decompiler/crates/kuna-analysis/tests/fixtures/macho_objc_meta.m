// Self-contained Objective-C Mach-O fixture for categories, protocols and ivars
// (`__objc_catlist`, `__objc_protolist`, `__objc_ivar`). Root classes so it needs
// NO macOS SDK; `Remote` has no implementation, so its category hangs off an
// imported (bind) class. See fixtures/README.md ("Mach-O Objective-C metadata").
__attribute__((objc_root_class)) @interface Base {
  Class isa;
  int count;
  double ratio;
  id name;
}
- (int)count;
- (void)setCount:(int)c;
- (double)ratio;
@end
@implementation Base
- (int)count { return count; }
- (void)setCount:(int)c { count = c; }
- (double)ratio { return ratio * 2; }
@end
@protocol Greeter
- (int)greet:(int)n;
@optional
- (void)wave;
@end
@interface Base (Extra) <Greeter>
+ (int)extraTag;
@end
@implementation Base (Extra)
- (int)greet:(int)n { return n + count; }
+ (int)extraTag { return 3; }
@end
__attribute__((objc_root_class)) @interface Remote
@end
@interface Remote (Extra)
- (int)remoteGreet:(int)n;
+ (int)remoteTag;
@end
@implementation Remote (Extra)
- (int)remoteGreet:(int)n { return n * 2; }
+ (int)remoteTag { return 5; }
@end
@interface Sub : Remote {
  int x;
  id tag;
}
- (int)x;
@end
@implementation Sub
- (int)x { return x; }
@end
int use(Base<Greeter> *b) { [b wave]; return [b greet:1] + [b count]; }
int main(void) { return 0; }
