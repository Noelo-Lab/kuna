// Self-contained Objective-C Mach-O fixture for a direct method: `-[Label
// bump:]` is `objc_direct`, so no method list names it and it takes no `_cmd`;
// only its symbol says it is a method of `Label`. Root class, no SDK; see
// fixtures/README.md ("Mach-O Objective-C direct methods").
__attribute__((objc_root_class)) @interface Label {
  Class isa;
  int flags;
  Label *next;
}
- (int)bump:(int)n __attribute__((objc_direct));
+ (int)scale:(int)n __attribute__((objc_direct));
- (int)flags;
@end
@implementation Label
- (int)bump:(int)n {
  flags += n;
  return next->flags;
}
+ (int)scale:(int)n { return n * 3; }
- (int)flags { return flags; }
@end

int main(void) { return 0; }
