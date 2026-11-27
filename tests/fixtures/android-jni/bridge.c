#include <jni.h>

static jint count_bytes(JNIEnv *env, jobject receiver, jbyteArray bytes) {
    return (*env)->GetArrayLength(env, bytes);
}

static jint increment(jint value) {
    return value + 1;
}

static const JNINativeMethod methods[] = {
    { "countBytes", "([B)I", (void *)count_bytes },
    { "increment", "(I)I", (void *)increment }
};

JNIEXPORT jint JNI_OnLoad(JavaVM *vm, void *reserved) {
    JNIEnv *env;
    if ((*vm)->GetEnv(vm, (void **)&env, JNI_VERSION_1_6) != JNI_OK)
        return JNI_ERR;
    jclass cls = (*env)->FindClass(env, "org/example/Bridge");
    if (!cls || (*env)->RegisterNatives(env, cls, methods, sizeof(methods) / sizeof(methods[0])) != JNI_OK)
        return JNI_ERR;
    return JNI_VERSION_1_6;
}

JNIEXPORT jintArray Java_org_example_Bridge_make__I(JNIEnv *env, jclass cls, jint n) {
    return (*env)->NewIntArray(env, n);
}

JNIEXPORT jint Java_org_example_Bridge_size___3B(JNIEnv *env, jobject receiver, jbyteArray data) {
    return (*env)->GetArrayLength(env, data) + 1;
}

struct callbacks {
    void *reserved[6];
    jint (*lookup)(void *ctx, const char *name);
};

JNIEXPORT jint ordinary_lookup(struct callbacks **ctx) {
    return (*ctx)->lookup(ctx, "ordinary");
}
