/* Shared native/emitted-C harness for XMM6-XMM9 preservation and the float result. */
#include <stdint.h>
#include <string.h>

#ifndef KUNA_EMITTED_RECT_DEFINED
typedef struct Rect {
  float x;
  float y;
  float width;
  float height;
} Rect;
#endif

typedef struct Probe {
  uint32_t xmm[16];
  uint32_t result_bits;
} Probe;

const Rect rect_source __attribute__((section(".saved_xmm_rect"), aligned(16))) =
    {1.25f, 2.5f, 17.0f, 23.0f};
uintptr_t volatile fixture_escaped_home;
#ifdef SAVED_XMM_ESCAPE_FRAME_LOW
uint32_t volatile fixture_escaped_frame_low;
#endif

#ifndef SAVED_XMM_READ_HOME
__attribute__((ms_abi, noinline))
void fixture_copy_rect(Rect *destination, const Rect *source) {
#ifdef SAVED_XMM_ESCAPE_FRAME_LOW
  __asm__ volatile("movl %%esp, (%0)" : : "r"(&fixture_escaped_frame_low) : "memory");
#endif
  destination->x = source->x;
  destination->y = source->y;
  destination->width = source->width;
  destination->height = source->height;
}
#endif

extern void fixture_probe(Probe *);

int main(void) {
  static const uint32_t expected[16] = {
      0x06060606, 0x06060607, 0x06060608, 0x06060609,
      0x07070706, 0x07070707, 0x07070708, 0x07070709,
      0x08080806, 0x08080807, 0x08080808, 0x08080809,
      0x09090906, 0x09090907, 0x09090908, 0x09090909,
  };
  Probe result = {0};
  fixture_probe(&result);
  return memcmp(result.xmm, expected, sizeof(expected)) != 0 ||
         result.result_bits != 0x40700000;
}
