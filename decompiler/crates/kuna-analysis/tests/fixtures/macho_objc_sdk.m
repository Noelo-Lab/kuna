// Self-contained Objective-C Mach-O fixture for the `objcsdk` header reader:
// `Widget` is declared but never implemented here, so every send goes through
// an `objc_msgSend$<sel>` stub whose selector only the headers describe. Root
// class so it needs NO macOS SDK; the types stand in for CoreGraphics' (not
// Apple's definitions). See fixtures/README.md ("Mach-O Objective-C SDK").
typedef double CGFloat;
typedef long NSInteger;
typedef signed char BOOL;
struct CGRect { double x; double y; double width; double height; };
typedef struct CGRect CGRect;
__attribute__((objc_root_class)) @interface Widget
- (void)setAlpha:(CGFloat)alpha;
- (void)setFrame:(CGRect)frame;
- (void)setTag:(NSInteger)tag;
- (BOOL)isHidden;
- (CGRect)frame;
- (void)insertSubview:(Widget *)view atIndex:(NSInteger)index;
@end
void style(Widget *w, Widget *child, double alpha, NSInteger tag) {
  [w setAlpha:alpha];
  [w setTag:tag];
  if ([w isHidden])
    return;
  CGRect r = [w frame];
  r.width = r.width + 10;
  [w setFrame:r];
  [w insertSubview:child atIndex:tag];
}
int main(void) { return 0; }
