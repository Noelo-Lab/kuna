/* macho_dataslotaddr fixture: an arm64 Mach-O that passes the addresses of
   two DATA imports, loaded from their __got slots, to a call, the way Swift
   passes a witness table and Objective-C a class object. */
extern char objc_class __asm__("_OBJC_CLASS_$_NSObject");
extern char witness_table __asm__("_$s7SwiftUI5ColorVAA4ViewAAWP");
void use_addresses(void *a, void *b);
void pass_addresses(void) { use_addresses(&objc_class, &witness_table); }
int main(void) { pass_addresses(); return 0; }
