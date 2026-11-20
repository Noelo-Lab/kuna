__attribute__((visibility("hidden"), noinline)) int audit_target(int x) { return x + 7; }
struct __attribute__((packed)) audit_packed { char tag; int (*fn)(int); };
struct audit_packed audit_packed_slot = { 1, audit_target };
int (*audit_slot)(int) = audit_target;
