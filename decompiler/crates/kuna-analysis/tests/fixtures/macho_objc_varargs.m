typedef struct { double x, y, width, height; } Rect;
extern void NSLog(id format, ...);

__attribute__((objc_root_class))
@interface View { Class isa; }
- (id)initWithFrame:(Rect)frame;
@end

@interface Panel : View
- (id)initWithFrame:(Rect)frame;
- (id)initWithWidth:(double)width height:(double)height;
@end

@implementation View
- (id)initWithFrame:(Rect)frame { return self; }
@end

@implementation Panel
- (id)initWithFrame:(Rect)frame { return [super initWithFrame:frame]; }
- (id)initWithWidth:(double)width height:(double)height {
  return [super initWithFrame:(Rect){ 8.0, 16.0, width, height }];
}
@end

void report(id error, id payload) { NSLog(@"Error %@ parsing token payload %@", error, payload); }

int main(void) { return 0; }
