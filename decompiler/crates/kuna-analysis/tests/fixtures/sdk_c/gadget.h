/* Stand-in declarations for the `objcsdk` C reader (hand-written, no SDK text). */
#ifndef SDK_C_GADGET_H
#define SDK_C_GADGET_H

struct GadgetRect {
    double x;
    double y;
    double width;
    double height;
};
typedef struct GadgetRect GadgetRect;

typedef struct GadgetInfo {
    unsigned int version;
    long count;
    const char *label;
    unsigned short flags[2];
} GadgetInfo;

struct GadgetWide {
    long words[4];
};

extern double GadgetRectGetWidth(GadgetRect rect);
extern GadgetRect GadgetRectInset(GadgetRect rect, double by);
extern int GadgetGetInfo(void *gadget, GadgetInfo *info);
extern long GadgetSum(struct GadgetWide wide, int scale);

/* Declared two ways: no prototype. */
#ifdef GADGET_LEGACY
extern int GadgetAmbiguous(int a);
#else
extern int GadgetAmbiguous(long a, long b);
#endif

/* A type the reader cannot resolve by value: no prototype. */
extern int GadgetOpaque(gadget_handle_t handle, int b);

#endif
