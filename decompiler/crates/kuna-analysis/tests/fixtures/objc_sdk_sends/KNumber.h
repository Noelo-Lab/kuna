// Hand-written declarations for the macho_objcsdksend fixture (option objcsdk).
@interface KObject
+ (instancetype)alloc;
@end

@interface KValue : KObject
- (KString *)stringValue;
@end

@interface KNumber : KValue
@property (readonly) long long longLongValue;
@end

@interface KNumber (KNumberCreation)
+ (KNumber *)numberWithLongLong:(long long)value;
@end
