/* A variadic argument read as an int for some commands and as a pointer for others.
   gcc -O2 -o retypehalves_x86_64 retypehalves_x86_64.c */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <unistd.h>

static int cloexec_state;

static int dup_cloexec(int fd, int target)
{
    int r;
    if (cloexec_state >= 0) {
        r = fcntl(fd, F_DUPFD_CLOEXEC, target);
        if (r >= 0 || errno != EINVAL) {
            cloexec_state = 1;
            return r;
        }
        r = fcntl(fd, F_DUPFD, target);
        if (r < 0)
            return r;
        cloexec_state = -1;
    } else {
        r = fcntl(fd, F_DUPFD, target);
        if (r < 0 || cloexec_state != -1)
            return r;
    }
    int flags = fcntl(r, F_GETFD);
    if (flags < 0 || fcntl(r, F_SETFD, flags | FD_CLOEXEC) == -1) {
        int saved = errno;
        close(r);
        errno = saved;
        return -1;
    }
    return r;
}

int control(int fd, int cmd, ...)
{
    va_list ap;
    int r;
    va_start(ap, cmd);
    switch (cmd) {
    case F_DUPFD:
        r = fcntl(fd, F_DUPFD, va_arg(ap, int));
        break;
    case F_DUPFD_CLOEXEC:
        r = dup_cloexec(fd, va_arg(ap, int));
        break;
    default:
        switch (cmd) {
        case F_GETFD: case F_GETFL: case F_GETOWN: case F_GETSIG:
        case F_GETLEASE: case F_GETPIPE_SZ: case F_GET_SEALS:
            r = fcntl(fd, cmd);
            break;
        case F_SETFD: case F_SETFL: case F_SETOWN: case F_SETSIG:
        case F_SETLEASE: case F_NOTIFY: case F_SETPIPE_SZ: case F_ADD_SEALS:
            r = fcntl(fd, cmd, va_arg(ap, int));
            break;
        default:
            r = fcntl(fd, cmd, va_arg(ap, void *));
            break;
        }
        break;
    }
    va_end(ap);
    return r;
}

int main(int argc, char **argv)
{
    return control(argc, argc > 2 ? F_GETFD : F_DUPFD, 3) + (argv[0] != 0);
}
