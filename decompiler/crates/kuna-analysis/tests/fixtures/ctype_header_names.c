struct elm_tag { unsigned long size; };
typedef struct elm_tag elm_alias;
typedef elm_alias *elm_ref;
struct Header {
    const struct elm_tag *p;
    struct elm_tag *list[2];
    struct elm_tag (*table)[3];
    elm_ref alias;
};
struct Header sample;
