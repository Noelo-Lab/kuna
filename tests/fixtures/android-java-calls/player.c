#include <jni.h>
#include <stdarg.h>

#define NOINLINE __attribute__((noinline))
#define ENTRY(name) Java_demo_Bridge_##name##__Ldemo_Player_2I

struct methods {
    jmethodID seek;
    jmethodID stop;
};
static volatile struct methods cache;
static jmethodID replaceable;
static volatile jmethodID externally_written;
static jclass cached_local;
static jclass cached_weak;
static jclass cached_global;
extern void replace_method(jmethodID *);

static NOINLINE void save_methods(JNIEnv *env, jclass cls, volatile struct methods *out) {
    out->seek = (*env)->GetMethodID(env, cls, "seek", "(I)V");
    out->stop = (*env)->GetMethodID(env, cls, "stop", "()V");
}
static NOINLINE jmethodID cached_seek(void) {
    return cache.seek;
}

static jint registered_call(JNIEnv *env, jobject receiver, jbyteArray bytes) {
    jclass cls = (*env)->FindClass(env, "demo/Registered");
    jmethodID id = (*env)->GetStaticMethodID(env, cls, "touch", "()V");
    (*env)->CallStaticVoidMethod(env, cls, id);
    return (*env)->GetArrayLength(env, bytes);
}
static const JNINativeMethod registered[] = {
    {"registeredCall", "([B)I", (void *)registered_call}
};

JNIEXPORT jint JNI_OnLoad(JavaVM *vm, void *reserved) {
    JNIEnv *env;
    if ((*vm)->GetEnv(vm, (void **)&env, JNI_VERSION_1_6) != JNI_OK)
        return JNI_ERR;
    jclass cls = (*env)->FindClass(env, "demo/Player");
    if (!cls) return JNI_ERR;
    cached_local = cls;
    cached_weak = (jclass)(*env)->NewWeakGlobalRef(env, cls);
    cached_global = (jclass)(*env)->NewGlobalRef(env, cls);
    externally_written = (*env)->GetMethodID(env, cls, "original", "()V");
    save_methods(env, cls, &cache);
    replaceable = (*env)->GetMethodID(env, cls, "beforeReplacement", "()V");
    if ((*env)->RegisterNatives(env, cls, registered, 1) != JNI_OK) return JNI_ERR;
    return JNI_VERSION_1_6;
}

JNIEXPORT void ENTRY(direct)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jclass cls = (*env)->FindClass(env, "demo/Player");
    if (!cls) return;
    jmethodID method = (*env)->GetMethodID(env, cls, "seek", "(I)V");
    if (method) (*env)->CallVoidMethod(env, player, method, pos);
    method = (*env)->GetMethodID(env, cls, "stop", "()V");
    if (method) (*env)->CallVoidMethod(env, player, method);
}

JNIEXPORT void ENTRY(cached)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    (*env)->CallVoidMethod(env, player, cached_seek(), pos);
}

JNIEXPORT void ENTRY(choose)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jclass cls = (*env)->FindClass(env, "demo/Player");
    jmethodID method;
    if (pos < 0)
        method = (*env)->GetMethodID(env, cls, "left", "()V");
    else
        method = (*env)->GetMethodID(env, cls, "right", "()V");
    (*env)->CallVoidMethod(env, player, method);
}

JNIEXPORT void ENTRY(unknown)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    replace_method(&replaceable);
    (*env)->CallVoidMethod(env, player, replaceable);
}

static NOINLINE void call_v(JNIEnv *env, jobject player, jmethodID method, ...) {
    va_list args;
    va_start(args, method);
    (*env)->CallVoidMethodV(env, player, method, args);
    va_end(args);
}

JNIEXPORT void ENTRY(families)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jclass cls = (*env)->FindClass(env, "demo/Player");
    jvalue arg[1] = {{.i = pos}};
    jmethodID stat = (*env)->GetStaticMethodID(env, cls, "reset", "(I)V");
    (*env)->CallStaticVoidMethodA(env, cls, stat, arg);
    jmethodID init = (*env)->GetMethodID(env, cls, "<init>", "(I)V");
    (*env)->NewObjectA(env, cls, init, arg);
    jmethodID step = (*env)->GetMethodID(env, cls, "step", "(I)V");
    (*env)->CallNonvirtualVoidMethod(env, player, cls, step, pos);
    call_v(env, player, step, pos);
}

JNIEXPORT void ENTRY(wrongFamily)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jclass cls = (*env)->FindClass(env, "demo/Player");
    jmethodID method = (*env)->GetStaticMethodID(env, cls, "staticOnly", "()I");
    (*env)->CallVoidMethod(env, player, method);
}

JNIEXPORT void change_method(jmethodID new_method) {
    externally_written = new_method;
}

JNIEXPORT void ENTRY(externalWrite)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    (*env)->CallVoidMethod(env, player, externally_written);
}

JNIEXPORT void ENTRY(references)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jmethodID from_global = (*env)->GetMethodID(env, cached_global, "fromGlobal", "()V");
    (*env)->CallVoidMethod(env, player, from_global);
    jmethodID from_local = (*env)->GetMethodID(env, cached_local, "fromLocal", "()V");
    (*env)->CallVoidMethod(env, player, from_local);
    jmethodID from_weak = (*env)->GetMethodID(env, cached_weak, "fromWeak", "()V");
    (*env)->CallVoidMethod(env, player, from_weak);
}

JNIEXPORT void ENTRY(otherClass)(JNIEnv *env, jclass bridge, jobject player, jint pos) {
    jclass cls = (*env)->FindClass(env, "demo/Player");
    jmethodID id = (*env)->GetStaticMethodID(env, cls, "checkClass", "()V");
    jclass other = (*env)->FindClass(env, "demo/Other");
    (*env)->CallStaticVoidMethod(env, other, id);
    (*env)->CallStaticVoidMethod(env, bridge, id);
}

JNIEXPORT jint Java_demo_Bridge_shortName(JNIEnv *env, jobject receiver) {
    jclass cls = (*env)->FindClass(env, "demo/Short");
    jmethodID id = (*env)->GetMethodID(env, cls, "level", "()I");
    return (*env)->CallIntMethod(env, receiver, id);
}

/* A primitive-only native entry can use its own unrelated function table. */
struct native_ops {
    void *prefix[6];
    jint (*read)(void *, jint);
};
JNIEXPORT jint Java_demo_Bridge_numeric__JI(jlong address, jint index) {
    struct native_ops **ops = (struct native_ops **)(__UINTPTR_TYPE__)address;
    return (*ops)->read(ops, index);
}
