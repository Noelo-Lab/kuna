#define _GNU_SOURCE
#include <stdint.h>
#include <sys/mman.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>

typedef uint8_t uint1; typedef uint16_t uint2;
typedef uint32_t uint4; typedef uint64_t uint8;
GENERATED_FUNCTIONS

typedef void (*Probe)(void *, unsigned);
static void read_fault(int signal) { _exit(signal == SIGSEGV ? 77 : 78); }
static int check(Probe fn, void *base, unsigned active, int fault) {
    pid_t child = fork();
    if (child < 0) abort();
    if (!child) {
        if (signal(SIGSEGV, read_fault) == SIG_ERR) _exit(79);
        fn(base, active);
        _exit(0);
    }
    int status;
    if (waitpid(child, &status, 0) != child) abort();
    return WIFEXITED(status) && WEXITSTATUS(status) == (fault ? 77 : 0);
}

int run_native_read_alignment(void) {
    struct rlimit limit = {0, 0};
    if (setrlimit(RLIMIT_CORE, &limit)) return 90;
    size_t size = (size_t)sysconf(_SC_PAGESIZE);
    unsigned char *pages = mmap(0, 2*size, PROT_READ|PROT_WRITE,
                               MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (pages == MAP_FAILED || mprotect(pages+size, size, PROT_NONE)) return 91;
    if (mprotect(pages, size, PROT_READ)) return 92;
    // Independently authored native x86-64 references, not typed C loads:
    // test esi,esi; je return; mov al/ax/eax/rax,[rdi]; ret.
    static const unsigned char machine[][8] = {
        {0x85,0xf6,0x74,0x02,0x8a,0x07,0xc3},
        {0x85,0xf6,0x74,0x03,0x66,0x8b,0x07,0xc3},
        {0x85,0xf6,0x74,0x02,0x8b,0x07,0xc3},
        {0x85,0xf6,0x74,0x03,0x48,0x8b,0x07,0xc3},
    };
    void *code = mmap(0, size, PROT_READ|PROT_WRITE,
                      MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (code == MAP_FAILED) return 93;
    memcpy(code, machine, sizeof machine);
    if (mprotect(code, size, PROT_READ|PROT_EXEC)) return 94;
    const struct { Probe printed; unsigned width; } cases[] = {
        {Probe8,1}, {Probe16,2}, {Probe32,4}, {Probe64,8},
    };
    for (unsigned f=0; f<4; ++f) {
        Probe original = (Probe)((unsigned char *)code + f*8);
        if (!check(original,(void *)1,0,0) || !check(cases[f].printed,(void *)1,0,0)) return 1;
        // Valid misaligned inputs must not acquire a language-level alignment precondition.
        for (unsigned offset=0; offset<8; ++offset)
            if (!check(original,pages+offset,1,0) ||
                !check(cases[f].printed,pages+offset,1,0)) return 2;
        void *edge = pages+size-cases[f].width;
        if (!check(original,edge,1,0) || !check(cases[f].printed,edge,1,0)) return 3;
        for (unsigned readable=0; readable<cases[f].width; ++readable) {
            void *crossing = pages+size-readable;
            if (!check(original,crossing,1,1) || !check(cases[f].printed,crossing,1,1)) return 4;
        }
    }
    if (munmap(code,size) || munmap(pages,2*size)) return 95;
    return 0;
}
#ifdef C_MAIN
int main(void) { return run_native_read_alignment(); }
#endif
