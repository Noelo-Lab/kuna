/* Stand-in declarations for the `objcsdk` C reader (hand-written, no SDK text). */
#ifndef SDK_C_CFBASE_H
#define SDK_C_CFBASE_H

#define CF_EXPORT extern
#define CF_ENUM(_type, _name) _type _name; enum
#define CF_OPTIONS(_type, _name) _type _name; enum
#define CF_BRIDGED_TYPE(T)
#define API_AVAILABLE(...)
#define CF_RETURNS_RETAINED

#if defined(__cplusplus)
extern "C" {
#endif

#if __LP64__
typedef signed long CFIndex;
typedef unsigned long CFOptionFlags;
#else
typedef signed int CFIndex;
typedef unsigned int CFOptionFlags;
#endif
typedef unsigned int UInt32;
typedef unsigned char Boolean;
typedef UInt32 CFStringEncoding;
typedef const void *CFTypeRef;

typedef const struct CF_BRIDGED_TYPE(id) __CFAllocator *CFAllocatorRef;
typedef const struct CF_BRIDGED_TYPE(NSString) __CFString *CFStringRef;

typedef struct {
    CFIndex location;
    CFIndex length;
} CFRange;

typedef CF_ENUM(CFIndex, CFComparisonResult) {
    kCFCompareLessThan = -1L,
    kCFCompareEqualTo = 0,
    kCFCompareGreaterThan = 1
};

typedef CF_OPTIONS(CFOptionFlags, CFStringCompareFlags) {
    kCFCompareCaseInsensitive = 1,
    kCFCompareBackwards = 4,
    kCFCompareNumerically = (1UL << 6) /* 64 */
};

CF_EXPORT const CFAllocatorRef kCFAllocatorDefault;

CF_EXPORT
CFStringRef CFStringCreateWithCString(CFAllocatorRef alloc, const char *cStr, CFStringEncoding encoding) CF_RETURNS_RETAINED;

CF_EXPORT
CFIndex CFStringGetLength(CFStringRef theString);

CF_EXPORT
CFComparisonResult CFStringCompareWithOptions(CFStringRef theString1, CFStringRef theString2, CFRange rangeToCompare, CFStringCompareFlags compareOptions) API_AVAILABLE(macos(10.0), ios(2.0));

CF_EXPORT
Boolean CFStringFindWithOptions(CFStringRef theString, CFStringRef stringToFind, CFRange rangeToSearch, CFStringCompareFlags searchOptions, CFRange *result);

CF_EXPORT
CFRange CFStringFind(CFStringRef theString, CFStringRef stringToFind, CFStringCompareFlags compareOptions);

CF_EXPORT
void CFRelease(CFTypeRef cf);

CF_EXPORT
void CFShowFormat(CFStringRef format, ...);

#if defined(__cplusplus)
}
#endif

#endif
