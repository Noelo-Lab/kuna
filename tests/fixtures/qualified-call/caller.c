#include "decl.h"
static void callback(unsigned int event) { (void)event; }
int ready;
int mode(int enable) {
    result_t (*init)(callback_t) = initialize;
    ready = 0;
    if (init(callback) != 0) return 0;
    ready = 1;
    return 1;
}
int main(int count, char **values) { (void)values; return mode(count); }
