#include <android/binder_ibinder.h>
#include <android/binder_parcel.h>

static const char *names[] = {"readLevel", "writeLevel"};

static void *create(void *user) { return user; }
static void destroy(void *user) { (void)user; }

static binder_status_t transact(AIBinder *binder, transaction_code_t code,
                                const AParcel *in, AParcel *out) {
    int32_t *level = AIBinder_getUserData(binder);
    switch (code) {
        case FIRST_CALL_TRANSACTION:
            return AParcel_writeInt32(out, *level);
        case FIRST_CALL_TRANSACTION + 1:
            return AParcel_readInt32(in, level);
#if DENSE
#define LEVEL_CASE(n) case n: *level += n; return AParcel_writeInt32(out, n * n)
        LEVEL_CASE(3);  LEVEL_CASE(4);  LEVEL_CASE(5);  LEVEL_CASE(6);
        LEVEL_CASE(7);  LEVEL_CASE(8);  LEVEL_CASE(9);  LEVEL_CASE(10);
        LEVEL_CASE(11); LEVEL_CASE(12); LEVEL_CASE(13); LEVEL_CASE(14);
        LEVEL_CASE(15); LEVEL_CASE(16); LEVEL_CASE(17); LEVEL_CASE(18);
        LEVEL_CASE(19); LEVEL_CASE(20); LEVEL_CASE(21); LEVEL_CASE(22);
        LEVEL_CASE(23); LEVEL_CASE(24); LEVEL_CASE(25); LEVEL_CASE(26);
        LEVEL_CASE(27); LEVEL_CASE(28); LEVEL_CASE(29); LEVEL_CASE(30);
        LEVEL_CASE(31); LEVEL_CASE(32);
#undef LEVEL_CASE
#endif
        default:
            return STATUS_UNKNOWN_TRANSACTION;
    }
}

#if DENSE
extern __typeof__(AIBinder_Class_define) forward_define;
#define CLASS_DEFINE forward_define
#else
#define CLASS_DEFINE AIBinder_Class_define
#endif

__attribute__((visibility("default")))
AIBinder_Class *define_level(void) {
    AIBinder_Class *cls = CLASS_DEFINE("org.example.ILevel",
                                                create, destroy, transact);
#if NAMED
    if (__builtin_available(android 36, *)) {
        if (cls) AIBinder_Class_setTransactionCodeToFunctionNameMap(cls, names, 2);
    }
#endif
    return cls;
}

#if MUTABLE_NAME
__attribute__((visibility("default")))
void replace_level_name(const char *name) {
    names[1] = name;
}
#endif
