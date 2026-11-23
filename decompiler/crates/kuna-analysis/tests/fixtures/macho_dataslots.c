/* macho_dataslots fixture: an arm64 Mach-O that reads, through __got slots,
   three DATA imports whose mangling names them as data (an Objective-C class
   object, a Swift type's metadata, a C++ static data member) and one plain C
   data import, and calls one function import through its stub. */
extern long objc_class __asm__("_OBJC_CLASS_$_NSObject");
extern long swift_metadata __asm__("_$sSSN");
extern int cpp_static_member __asm__("__ZNSt3__15ctypeIcE2idE");
extern int imported_counter;
int imported_call(void);
long read_objc_class(void) { return objc_class; }
long read_swift_metadata(void) { return swift_metadata; }
int read_cpp_static_member(void) { return cpp_static_member; }
int read_counter(void) { return imported_counter; }
int main(void) { return imported_call(); }
