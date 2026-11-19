// Self-contained Objective-C Mach-O fixture for `objcstubagree`: the protocol
// `Measured` declares `-(double)length` and nothing in the image implements
// it; the class `Ruler` implements `-(double)tag`. `validate` sends both to a
// `Str` (declared, never implemented here), whose `length` and `tag` return
// integers in x0, through `objc_msgSend$<sel>` stubs, and passes each result
// to `check` with three more arguments. Root classes, so it needs NO macOS SDK.
// See fixtures/README.md ("objcstubagree").
typedef long NSInteger;
@class Protocol;
@protocol Measured
- (double)length;
@end
__attribute__((objc_root_class)) @interface Ruler
- (double)tag;
@end
@implementation Ruler
- (double)tag { return 2.5; }
@end
__attribute__((objc_root_class)) @interface Str
- (unsigned long)length;
- (NSInteger)tag;
@end
extern int check(unsigned long v, int *err, int code, const char *msg);
Protocol *measured(void) { return @protocol(Measured); }
int validate(Str *s, Str *t, int *err) {
  if (!check([s length], err, 1, "empty name"))
    return 0;
  return check((unsigned long)[t tag], err, 2, "bad tag");
}
int main(void) { return 0; }
