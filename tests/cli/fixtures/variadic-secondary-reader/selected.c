#include <stdint.h>
extern uint64_t __attribute__((ms_abi)) render_value(const char *format, ...);
extern int __attribute__((ms_abi)) tag_value(int *sender);

uint64_t __attribute__((ms_abi)) caller_clamped_integer(int *value, int tag) {
    int next = *value + (tag == 1 ? 1 : -1);
    if (next < 6) next = 6;
    if (next > 12) next = 12;
    *value = next;
    return render_value("%i", next);
}

uint64_t __attribute__((ms_abi)) caller_clamped_call(int *value, int *sender) {
    int tag = tag_value(sender);
    int next = *value + (tag == 1 ? 1 : -1);
    if (next < 6) next = 6;
    if (next > 12) next = 12;
    *value = next;
    return render_value("%i", next);
}

uint64_t __attribute__((ms_abi)) caller_stored_comparison(int *value, int tag) {
    int next = *value + (tag == 1);
    *value = next;
    return render_value("%i", next);
}

uint64_t __attribute__((ms_abi)) caller_stored_boolean(int *value, int tag) {
    int next = *value + ((tag < 0) != (tag == 1));
    *value = next;
    return render_value("%i", next);
}
