#include <stddef.h>
#include <stdint.h>

typedef int8_t int1;
typedef int16_t int2;
typedef int32_t int4;
typedef int64_t int8;
typedef uint8_t uint1;
typedef uint16_t uint2;
typedef uint32_t uint4;
typedef uint64_t uint8;
typedef uint8_t undefined1;

int8 fill_line(uint1 **line);
uint8 read_first(const uint1 *line);
uint1 *identity_pointer(uint1 *buffer);
void reset_line_reader(void);

#ifdef KUNA_NATIVE
uint8 outparam_branch_snapshot(int4 from_file, const uint1 *alternate);
uint8 outparam_expression_safe(uint1 *buffer);
#else
/* KUNA_EMITTED_CODE */

static size_t line_index;
static uint1 line_bytes[] = {3, 5};

int8 fill_line(uint1 **line) {
    if (line_index == sizeof line_bytes) return 0;
    *line = &line_bytes[line_index++];
    return 1;
}

uint8 read_first(const uint1 *line) {
    return *line;
}

uint1 *identity_pointer(uint1 *buffer) {
    *buffer += 1;
    return buffer;
}

void reset_line_reader(void) {
    line_index = 0;
}
#endif

int main(void) {
    uint1 alternate[] = {7, 11, 0};
    for (int trial = 0; trial < 4; ++trial) {
        reset_line_reader();
        if (outparam_branch_snapshot(1, NULL) != 8) return 1;
        reset_line_reader();
        if (outparam_branch_snapshot(0, alternate) != 18) return 2;
        reset_line_reader();
        if (outparam_branch_snapshot(0, NULL) != 0) return 3;
        uint1 byte = 41;
        if (outparam_expression_safe(&byte) != 43 || byte != 42) return 4;
    }
    return 0;
}
