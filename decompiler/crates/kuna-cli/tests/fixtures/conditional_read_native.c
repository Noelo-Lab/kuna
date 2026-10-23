
#ifdef ORIGINAL_ONLY
#define PROBE(name, type) \
void name(unsigned state, unsigned mode, unsigned left, unsigned right, void *base) { \
    if (state != 7) return; \
    if (mode & 1) { \
        if ((left >> 2) == (right >> 2)) return; \
    } else if (left != 9 && right != 0) return; \
    (void)*(volatile type *)((unsigned char *)base + 24); \
}
PROBE(Probe8, unsigned char)
PROBE(Probe16, unsigned short)
PROBE(Probe32, unsigned int)
PROBE(Probe64, unsigned long long)
void ProbeSpill32(unsigned state, unsigned mode, unsigned left, unsigned right, void *base) {
    void *volatile saved = base;
    if (state != 7) return;
    if (mode & 1) {
        if ((left >> 2) == (right >> 2)) return;
    } else if (left != 9 && right != 0) return;
    (void)*(volatile unsigned int *)((unsigned char *)saved + 24);
}
#else
#define _GNU_SOURCE
#include <stdint.h>
#include <sys/mman.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>

typedef int8_t int1; typedef uint8_t uint1;
typedef int16_t int2; typedef uint16_t uint2;
typedef int32_t int4; typedef uint32_t uint4;
typedef int64_t int8; typedef uint64_t uint8;
typedef uint8_t undefined1; typedef uint16_t undefined2;
typedef uint32_t undefined4; typedef uint64_t undefined8;

#define Probe8 Printed8
#define Probe16 Printed16
#define Probe32 Printed32
#define Probe64 Printed64
#define ProbeSpill32 PrintedSpill32
GENERATED_FUNCTIONS
#undef Probe8
#undef Probe16
#undef Probe32
#undef Probe64
#undef ProbeSpill32

typedef void (*Probe)(unsigned, unsigned, unsigned, unsigned, void *);
void Probe8(unsigned, unsigned, unsigned, unsigned, void *);
void Probe16(unsigned, unsigned, unsigned, unsigned, void *);
void Probe32(unsigned, unsigned, unsigned, unsigned, void *);
void Probe64(unsigned, unsigned, unsigned, unsigned, void *);
void ProbeSpill32(unsigned, unsigned, unsigned, unsigned, void *);

static void *expected_address;
static void read_fault(int signal, siginfo_t *info, void *context) {
    (void)context;
    _exit(signal == SIGSEGV && info->si_addr == expected_address ? 77 : 78);
}

static int check(Probe fn, unsigned state, unsigned mode, unsigned left,
                 unsigned right, void *base, void *fault_address) {
    pid_t child = fork();
    if (child < 0) abort();
    if (child == 0) {
        expected_address = fault_address;
        struct sigaction action = {0};
        action.sa_sigaction = read_fault;
        action.sa_flags = SA_SIGINFO;
        sigemptyset(&action.sa_mask);
        if (sigaction(SIGSEGV, &action, 0)) _exit(79);
        fn(state, mode, left, right, base);
        _exit(0);
    }
    int status;
    if (waitpid(child, &status, 0) != child) abort();
    if (fault_address)
        return WIFEXITED(status) && WEXITSTATUS(status) == 77;
    return WIFEXITED(status) && WEXITSTATUS(status) == 0;
}

int main(void) {
    struct rlimit limit = {0, 0};
    if (setrlimit(RLIMIT_CORE, &limit)) return 90;
    size_t size = (size_t)sysconf(_SC_PAGESIZE);
    unsigned char *pages = mmap(0, 3*size, PROT_READ|PROT_WRITE,
                               MAP_PRIVATE|MAP_ANONYMOUS|MAP_32BIT, -1, 0);
    if (pages == MAP_FAILED || mprotect(pages+size, size, PROT_NONE)) return 91;
    if (mprotect(pages,size,PROT_READ)) return 93;
    const struct { Probe original, printed; unsigned width; } cases[] = {
        {Probe8, Printed8, 1}, {Probe16, Printed16, 2},
        {Probe32, Printed32, 4}, {Probe64, Printed64, 8},
        {ProbeSpill32, PrintedSpill32, 4},
    };
    const unsigned states[] = {0, 6, 7, 8};
    const unsigned values[] = {0, 1, 3, 4, 8, 9, 12, 0xffffffff};
    for (unsigned f=0; f<sizeof cases/sizeof cases[0]; ++f)
        for (unsigned s=0; s<4; ++s)
            for (unsigned mode=0; mode<2; ++mode)
                for (unsigned l=0; l<8; ++l)
                    for (unsigned r=0; r<8; ++r) {
                        unsigned state=states[s], left=values[l], right=values[r];
                        int active = state == 7 &&
                            ((mode & 1) ? (left/4 != right/4) : (left == 9 || right == 0));
                        void *blocked = pages+size+32-24;
                        void *fault = active ? pages+size+32 : 0;
                        if (!check(cases[f].original,state,mode,left,right,blocked,fault) ||
                            !check(cases[f].printed,state,mode,left,right,blocked,fault)) {
                            fprintf(stderr,"width=%u state=%u mode=%u left=%u right=%u active=%d\n",
                                    cases[f].width,state,mode,left,right,active);
                            return 1;
                        }
                    }
    // A read ending exactly at the protected page must not become a wider access.
    for (unsigned f=0; f<sizeof cases/sizeof cases[0]; ++f) {
        void *edge = pages+size-cases[f].width-24;
        if (!check(cases[f].original,7,0,9,1,edge,0) ||
            !check(cases[f].printed,7,0,9,1,edge,0)) return 2;
    }
    // Each partial overlap must still reach the protected page: no narrowing.
    for (unsigned f=0; f<sizeof cases/sizeof cases[0]; ++f)
        for (unsigned readable=1; readable<cases[f].width; ++readable) {
            void *crossing = pages+size-readable-24;
            if (!check(cases[f].original,7,0,9,1,crossing,pages+size) ||
                !check(cases[f].printed,7,0,9,1,crossing,pages+size)) return 3;
        }
    if (munmap(pages,3*size)) return 92;
    return 0;
}
#endif
