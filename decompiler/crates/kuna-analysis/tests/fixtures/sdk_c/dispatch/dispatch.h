/* Stand-in declarations for the `objcsdk` C reader (hand-written, no SDK text). */
#ifndef SDK_C_DISPATCH_H
#define SDK_C_DISPATCH_H

#define DISPATCH_EXPORT extern __attribute__((visibility("default")))
#define DISPATCH_NONNULL1 __attribute__((__nonnull__(1)))
#define DISPATCH_NONNULL3 __attribute__((__nonnull__(3)))
#define DISPATCH_NOTHROW __attribute__((__nothrow__))
#define DISPATCH_DECL(name) typedef struct name##_s *name##_t
#define DISPATCH_ENUM(name, type, ...) typedef enum : type { __VA_ARGS__ } name##_t
#define API_AVAILABLE(...)

DISPATCH_DECL(dispatch_queue);
DISPATCH_DECL(dispatch_group);

typedef void (*dispatch_function_t)(void *);

typedef union {
    struct dispatch_queue_s *_dq;
    struct dispatch_group_s *_dg;
} dispatch_object_t __attribute__((__transparent_union__));

DISPATCH_ENUM(dispatch_tier, unsigned int,
    DISPATCH_TIER_LOW = 0x09,
    DISPATCH_TIER_HIGH = 0x21,
);

API_AVAILABLE(macos(10.6), ios(4.0))
DISPATCH_EXPORT DISPATCH_NONNULL1 DISPATCH_NONNULL3 DISPATCH_NOTHROW
void
dispatch_async_f(dispatch_queue_t queue,
        void *context, dispatch_function_t work);

API_AVAILABLE(macos(10.6), ios(4.0))
DISPATCH_EXPORT DISPATCH_NOTHROW
void
dispatch_set_tier(dispatch_object_t object, dispatch_tier_t tier);

#endif
