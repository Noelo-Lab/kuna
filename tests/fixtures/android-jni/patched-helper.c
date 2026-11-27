/* JNI_OnLoad registers one table as written and one patched through a helper. */
#if defined(__x86_64__)
#define NOTE_TYPE "@note"
#else
#define NOTE_TYPE "%note"
#endif
__asm__(".pushsection .note.android.ident,\"a\"," NOTE_TYPE "\n.balign 4\n.long 8\n.long 4\n.long 1\n"
        ".asciz \"Android\"\n.balign 4\n.long 24\n.popsection\n");

typedef int jint;
typedef void *jclass;
typedef struct {
    const char *name, *signature;
    void *fnPtr;
} JNINativeMethod;
typedef const struct JNINativeInterface *JNIEnv;
typedef const struct JNIInvokeInterface *JavaVM;
struct JNINativeInterface {
    void *unused0[6];
    jclass (*FindClass)(JNIEnv *, const char *);
    void *unused1[208];
    jint (*RegisterNatives)(JNIEnv *, jclass, const JNINativeMethod *, jint);
};
struct JNIInvokeInterface {
    void *unused[6];
    jint (*GetEnv)(JavaVM *, void **, jint);
};

static jint kept(JNIEnv *env, jclass cls) { return 1; }
static jint decoy(JNIEnv *env, jclass cls) { return 2; }
static jint replacement(JNIEnv *env, jclass cls) { return 3; }
static JNINativeMethod kept_table[] = {{"kept", "()I", (void *)kept}};
static JNINativeMethod patched_table[] = {{"patched", "()I", (void *)decoy}};
static jclass remembered;

__attribute__((noinline, visibility("hidden"))) void remember(jclass cls) { remembered = cls; }
__attribute__((noinline, visibility("hidden"))) void set_fn(JNINativeMethod *m, int i, void *f) {
    m[i].fnPtr = f;
}

__attribute__((visibility("default"))) jint JNI_OnLoad(JavaVM *vm, void *reserved) {
    JNIEnv *env;
    if ((*vm)->GetEnv(vm, (void **)&env, 0x10006) != 0)
        return -1;
    JNIEnv *e = env;
    jclass first = (*e)->FindClass(e, "x/Z");
    jclass second = (*e)->FindClass(e, "x/Y");
    if (!first || !second)
        return -1;
    remember(first);
    if ((*e)->RegisterNatives(e, first, kept_table, 1) < 0)
        return -1;
    set_fn(patched_table, 0, (void *)replacement);
    if ((*e)->RegisterNatives(e, second, patched_table, 1) < 0)
        return -1;
    return 0x10006;
}
