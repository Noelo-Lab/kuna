/* The C half of `objcsdk`: imports whose prototypes come from the stand-in
 * headers in `sdk_c/`.
 *
 *   clang --target=arm64-apple-ios16.0 -nostdinc -Isdk_c -O2 -c macho_sdk_c.c -o macho_sdk_c.o
 *   ld64.lld -arch arm64 -platform_version ios 16.0 16.0 -dylib -undefined dynamic_lookup \
 *       -o macho_sdk_c_arm64 macho_sdk_c.o
 */
typedef long gadget_handle_t;

#include "CoreFoundation/CFBase.h"
#include "dispatch/dispatch.h"
#include "gadget.h"

CFIndex name_len(const char *s) {
    CFStringRef str = CFStringCreateWithCString(kCFAllocatorDefault, s, 0x08000100);
    CFIndex n = CFStringGetLength(str);
    CFRelease(str);
    return n;
}

void later(dispatch_queue_t q, void *ctx, dispatch_function_t f) { dispatch_async_f(q, ctx, f); }

void raise_tier(dispatch_queue_t q) { dispatch_set_tier(q, DISPATCH_TIER_HIGH); }

int same(CFStringRef a, CFStringRef b) {
    CFRange all = {0, CFStringGetLength(a)};
    return CFStringCompareWithOptions(a, b, all, kCFCompareCaseInsensitive | kCFCompareNumerically) == kCFCompareEqualTo;
}

long find(CFStringRef a, CFStringRef b, long from, long len) {
    CFRange found;
    CFRange in = {from, len};
    if (!CFStringFindWithOptions(a, b, in, kCFCompareBackwards, &found))
        return -1;
    return found.location + found.length;
}

long info_count(void *g) {
    GadgetInfo info;
    if (GadgetGetInfo(g, &info) != 0)
        return -1;
    return info.count + info.flags[1];
}

double twice(double pad, GadgetRect r) { return pad + 2 * GadgetRectGetWidth(r); }

double inset_width(GadgetRect r) {
    GadgetRect i = GadgetRectInset(r, 2.0);
    return i.width + i.x;
}

long where(CFStringRef a, CFStringRef b) {
    CFRange r = CFStringFind(a, b, 0);
    return r.location + r.length;
}

long sum(const struct GadgetWide *w) { return GadgetSum(*w, 3); }

void show(CFStringRef fmt, long a, long b) { CFShowFormat(fmt, a, b); }

int unreadable(long a, long b) { return GadgetAmbiguous(a, b) + GadgetOpaque(a, (int)b); }
