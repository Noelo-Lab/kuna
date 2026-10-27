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
    slot = object;
    Probe((void *)&slot);
    assert(slot == NULL);
    return 0;
}
#endif
