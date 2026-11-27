#include <android/binder_ibinder.h>

AIBinder_Class *forward_define(const char *descriptor, AIBinder_Class_onCreate create,
                              AIBinder_Class_onDestroy destroy, AIBinder_Class_onTransact transact) {
    return AIBinder_Class_define(descriptor, create, destroy, transact);
}
