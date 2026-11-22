/* macho_objcsdksend fixture: an arm64 Mach-O that sends class and instance
   messages to a class it only imports (`_OBJC_CLASS_$_KNumber`, a __got data
   import), the way an app sends to a Foundation class. The class is declared
   for `objcsdk` by objc_sdk_sends/KNumber.h. The `objc_msgSend$<sel>` stubs
   set x1 to the selector themselves, so the C prototypes carry an unused
   second argument. send_via_self and send_joined reach the class through
   objc_opt_self and through a join of two paths. */
typedef struct objc_object *id;
extern char knumber_class __asm__("_OBJC_CLASS_$_KNumber");
id make_number(void *cls, void *cmd, long long v) __asm__("_objc_msgSend$numberWithLongLong:");
id make_from_int(void *cls, void *cmd, int v) __asm__("_objc_msgSend$numberWithInt:");
id string_value(id self, void *cmd) __asm__("_objc_msgSend$stringValue");
void consume(id a, id b, id c);
id objc_opt_self(void *obj);
void send_to_import(long long v) {
    id n = make_number(&knumber_class, 0, v);
    id m = make_from_int(&knumber_class, 0, 7);
    consume(n, string_value(n, 0), m);
}
void send_via_self(long long v) {
    id n = make_number(objc_opt_self(&knumber_class), 0, v);
    consume(n, string_value(n, 0), 0);
}
void send_joined(int which, long long v) {
    void *cls = which ? (void *)objc_opt_self(&knumber_class) : (void *)&knumber_class;
    id n = make_number(cls, 0, v);
    consume(n, string_value(n, 0), 0);
}
int main(void) {
    send_to_import(42);
    send_via_self(1);
    send_joined(1, 3);
    return 0;
}
