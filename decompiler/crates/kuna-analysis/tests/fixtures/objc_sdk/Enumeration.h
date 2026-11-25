// A hand-written stand-in for an SDK framework header (no Apple text): the
// fast-enumeration declarations a for-in loop sends. Read by the kuna_objcsdk
// unit tests and the kuna-objcforin stage test.
typedef unsigned long NSUInteger;

typedef struct {
    unsigned long state;
    id __unsafe_unretained _Nullable * _Nullable itemsPtr;
    unsigned long * _Nullable mutationsPtr;
    unsigned long extra[5];
} NSFastEnumerationState;

@protocol NSFastEnumeration
- (NSUInteger)countByEnumeratingWithState:(NSFastEnumerationState *)state objects:(id __unsafe_unretained _Nullable [_Nonnull])buffer count:(NSUInteger)len;
@end
