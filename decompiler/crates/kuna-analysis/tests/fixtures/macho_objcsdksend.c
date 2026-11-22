/* macho_objcsdksend fixture: an arm64 Mach-O that sends class and instance
   messages to a class it only imports (`_OBJC_CLASS_$_KNumber`, a __got data
   import), the way an app sends to a Foundation class. The class is declared
   for `objcsdk` by objc_sdk_sends/KNumber.h. */
typedef struct objc_object *id;
extern char knumber_class __asm__("_OBJC_CLASS_$_KNumber");
id make_number(void *cls, long long v) __asm__("_objc_msgSend$numberWithLongLong:");
id make_from_int(void *cls, int v) __asm__("_objc_msgSend$numberWithInt:");
id string_value(id self) __asm__("_objc_msgSend$stringValue");
void consume(id a, id b, id c);
void send_to_import(long long v) {
    id n = make_number(&knumber_class, v);
    id m = make_from_int(&knumber_class, 7);
    consume(n, string_value(n), m);
}
int main(void) { send_to_import(42); return 0; }
