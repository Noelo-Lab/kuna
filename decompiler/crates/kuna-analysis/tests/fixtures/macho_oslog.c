typedef struct os_log_s *os_log_t;
extern struct os_log_s _os_log_default;
extern void *__dso_handle;
extern void _os_log_impl(void *dso, os_log_t log, unsigned char type,
                         const char *format, unsigned char *buf, unsigned int size);
extern int os_log_type_enabled(os_log_t log, unsigned char type);

#define OSLOG(log, type, fmt, ...)                                            \
    do {                                                                      \
        if (os_log_type_enabled(log, type)) {                                 \
            static const char _f[]                                            \
                __attribute__((section("__TEXT,__oslogstring,cstring_literals"))) = fmt; \
            __attribute__((aligned(16))) unsigned char _b[                    \
                __builtin_os_log_format_buffer_size(fmt, ##__VA_ARGS__)];     \
            _os_log_impl(&__dso_handle, log, type, _f,                        \
                         (unsigned char *)__builtin_os_log_format(_b, fmt, ##__VA_ARGS__), \
                         (unsigned int)sizeof(_b));                           \
        }                                                                     \
    } while (0)

void log_code(os_log_t log, int code) {
    OSLOG(log, 0x10, "failed to read parents: %d", code);
}

void log_pair(os_log_t log, long count, const char *name) {
    OSLOG(log, 0, "loaded %ld entries from %s", count, name);
}

void log_private(os_log_t log, const char *path, unsigned int flags) {
    OSLOG(log, 0x11, "open %{private}s flags=0x%x", path, flags);
}

void log_plain(void) {
    OSLOG(&_os_log_default, 0, "no arguments here");
}
