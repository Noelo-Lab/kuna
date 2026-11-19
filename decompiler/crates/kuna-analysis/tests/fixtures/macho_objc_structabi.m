// Self-contained Mach-O fixture for Objective-C methods whose type encoding
// passes or returns an aggregate by value (tests/stages/kuna-objcstructabi.xml):
// a homogeneous floating-point aggregate (`CGRect`, `UIEdgeInsets`), an integer
// pair (`_NSRange`), aggregates larger than 16 bytes (`CGAffineTransform`,
// `Mixed`) as an argument and as a return, an array member (`[4d]`), a union,
// bit-fields, an unnamed struct and a pointer to a struct. `-x` strips local
// symbols. See fixtures/README.md ("Mach-O ObjC struct ABI") for the recipe.
struct CGPoint { double x; double y; };
struct CGSize { double width; double height; };
struct CGRect { struct CGPoint origin; struct CGSize size; };
typedef struct CGRect CGRect;
typedef struct _NSRange { unsigned long location; unsigned long length; } NSRange;
typedef struct UIEdgeInsets { double top, left, bottom, right; } UIEdgeInsets;
typedef struct CGAffineTransform { double a, b, c, d, tx, ty; } CGAffineTransform;
typedef struct Mixed { int tag; double weights[4]; char kind; } Mixed;
typedef union Word { long whole; float half; } Word;
typedef struct Flags { unsigned ready : 3; unsigned mode : 5; unsigned short level; } Flags;
typedef struct { int lo; int hi; } Span;
__attribute__((objc_root_class))
@interface Shape { CGRect frame; NSRange range; CGAffineTransform xform; Mixed mixed; long total; }
- (void)setFrame:(CGRect)r;
- (CGRect)frame;
- (NSRange)range;
- (void)setRange:(NSRange)r scale:(long)k;
- (double)inset:(UIEdgeInsets)i by:(double)k;
- (CGAffineTransform)transform;
- (void)setTransform:(CGAffineTransform)t tag:(long)k;
- (Mixed)mixed;
- (double)weigh:(Mixed)m at:(int)i;
- (long)word:(Word)w;
- (int)flags:(Flags)f;
- (int)span:(Span)s;
- (double)width:(const CGRect *)r;
@end
@implementation Shape
- (void)setFrame:(CGRect)r { frame = r; total += 1; }
- (CGRect)frame { return frame; }
- (NSRange)range { return range; }
- (void)setRange:(NSRange)r scale:(long)k { range.location = r.location * k; range.length = r.length + k; }
- (double)inset:(UIEdgeInsets)i by:(double)k { return (i.top + i.bottom) * k - i.left - i.right; }
- (CGAffineTransform)transform { return xform; }
- (void)setTransform:(CGAffineTransform)t tag:(long)k { xform = t; total = k; }
- (Mixed)mixed { return mixed; }
- (double)weigh:(Mixed)m at:(int)i { return m.weights[i & 3] * m.tag; }
- (long)word:(Word)w { return w.whole + total; }
- (int)flags:(Flags)f { return f.mode + f.level; }
- (int)span:(Span)s { return s.hi - s.lo; }
- (double)width:(const CGRect *)r { return r->size.width; }
@end
int main(void) { return 0; }
