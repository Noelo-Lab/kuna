#define _GNU_SOURCE
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <obstack.h>
typedef struct obstack obstack;

#if defined(ORIGINAL_ONLY) || defined(NATIVE_REFERENCE)
__attribute__((noinline, visibility("default")))
void Probe(void **slot) {
    if (!slot || !*slot) return;
    (void)*(volatile uintptr_t *)((unsigned char *)*slot + sizeof(void *));
    _obstack_newchunk((struct obstack *)*slot, 32);
    (obstack_free)((struct obstack *)*slot, NULL);
    free(*slot);
    *slot = NULL;
}

__attribute__((noinline, visibility("default")))
void ProbeRead(obstack *object, unsigned active) {
    if (!object || !active) return;
    (void)*(volatile uintptr_t *)((unsigned char *)object + sizeof(void *));
    _obstack_newchunk(object, 32);
}

__attribute__((noinline, visibility("default")))
int ReadAlignment(obstack *object) {
    int value = object->alignment_mask;
    _obstack_newchunk(object, 0);
    return value;
}

__attribute__((noinline, visibility("default")))
void WriteAlignment(obstack *object, int value) {
    object->alignment_mask = value;
    _obstack_newchunk(object, 0);
}
#else
GENERATED_FUNCTIONS
#endif

#ifndef ORIGINAL_ONLY
static void *allocate_chunk(long size) { return malloc((size_t)size); }
int main(void) {
    Probe(NULL);
    void *slot = NULL;
    Probe((void *)&slot);
    obstack *object = malloc(sizeof(*object));
    assert(object);
    assert(_obstack_begin(object, 0, 0, allocate_chunk, free));
    int alignment = object->alignment_mask;
    assert(ReadAlignment(object) == alignment);
    WriteAlignment(object, alignment);
    assert(object->alignment_mask == alignment);
    ProbeRead(NULL, 1);
    ProbeRead(object, 0);
    ProbeRead(object, 1);
    slot = object;
    Probe((void *)&slot);
    assert(slot == NULL);
    return 0;
}
#endif
