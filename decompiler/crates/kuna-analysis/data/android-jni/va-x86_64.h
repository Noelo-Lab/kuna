typedef long jni_int64;

struct jni_va_list_tag {
    unsigned int gp_offset;
    unsigned int fp_offset;
    void *overflow_arg_area;
    void *reg_save_area;
};

typedef struct jni_va_list_tag *jni_va_list_arg;
