/* Fixture for `option typelib auto|off` (tests/stages/kuna-typelib.xml).
 *
 * Built: gcc -O1 -fno-inline -o typelib_x86_64 typelib_x86_64.c
 *
 * No debug information, on purpose: the declarations of `poll`, `getrlimit`
 * and `setrlimit`, and the layouts of `struct pollfd` and `struct rlimit`, can
 * only come from the prebuilt type library. `puts` is a control: the built-in
 * prototype table already declares it, and the library leaves it alone.
 */
#include <poll.h>
#include <stdio.h>
#include <sys/resource.h>

int wait_readable(int fd, int ms) {
    struct pollfd p;
    p.fd = fd;
    p.events = POLLIN;
    if (poll(&p, 1, ms) <= 0)
        return 0;
    return p.revents & POLLIN;
}

int raise_files(void) {
    struct rlimit rl;
    if (getrlimit(RLIMIT_NOFILE, &rl) == 0 && rl.rlim_cur < 1024) {
        rl.rlim_cur = rl.rlim_max;
        return setrlimit(RLIMIT_NOFILE, &rl);
    }
    return 0;
}

int main(int argc, char **argv) {
    puts(argv[0]);
    return wait_readable(argc, 1000) + raise_files();
}
