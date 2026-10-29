// A hand-written stand-in for an SDK framework header (no Apple text): the
// declaration shapes the `objcsdk` reader must understand. Read by the
// kuna_objcsdk unit tests and the kuna-objcsdk stage test.
#import <Foundation/Foundation.h>

typedef double CGFloat;
typedef long NSInteger;
typedef struct CGRect CGRect;
typedef NS_ENUM(NSInteger, WidgetContentMode) {
    WidgetContentModeFill,
    WidgetContentModeFit,
};
typedef NSString * WidgetTitle NS_TYPED_ENUM;
typedef struct OpaqueThing OpaqueThing;

@class Widget;

@protocol WidgetDelegate <NSObject>
- (void)widget:(Widget *)widget didChange:(NSInteger)what;
@optional
- (void)widgetDidFinish:(Widget *)widget; // optional
@end

@interface Widget : NSObject {
    NSInteger _private; /* an ivar block the reader skips */
}
@property (nonatomic) CGFloat alpha;            // animatable
@property (nonatomic) CGRect frame;
@property (nonatomic) NSInteger tag API_AVAILABLE(ios(2.0));
@property (nonatomic, getter=isHidden) BOOL hidden;
@property (nullable, nonatomic, readonly) Widget *superview;
@property (nonatomic) WidgetContentMode contentMode;
@property (nullable, nonatomic, copy) WidgetTitle title;
@property (nonatomic) OpaqueThing opaqueThing;
- (void)insertSubview:(Widget *)view atIndex:(NSInteger)index;
+ (void)animateWithDuration:(CGFloat)duration
                 animations:(void (^)(void))animations API_AVAILABLE(ios(4.0));
- (CGRect)convertRect:(CGRect)rect;
- (void)setRemoteGreeting:(NSInteger)n;
- (void)setBadge:(NSInteger)badge;
@property (nonatomic, copy) void (^onChange)(NSInteger what);
@end

typedef void (^WidgetHandler)(NSInteger what);

@interface Gadget : NSObject
- (void)setBadge:(OpaqueThing)badge; // the same selector as Widget's, with a type the reader cannot spell
- (void)setHandler:(WidgetHandler)handler;
@end
