#include <assert.h>
#include <string.h>

extern int bounded_tail(int);
extern int returning_caller(int);

int main(int argc, char **argv) {
    if (argc > 1 && !strcmp(argv[1], "fail"))
        return bounded_tail(1);
    if (argc > 1 && !strcmp(argv[1], "control-fail"))
        return returning_caller(1);
    assert(bounded_tail(0) == 5);
    assert(returning_caller(0) == 9);
    return 0;
}
