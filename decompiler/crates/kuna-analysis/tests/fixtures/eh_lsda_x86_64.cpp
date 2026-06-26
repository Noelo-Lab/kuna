// Exception-handling landing-pad fixture (the .gcc_except_table / LSDA case).
// A try/catch over a throwing helper forces g++ to emit a .gcc_except_table
// LSDA with a call-site table whose records carry landing-pad PCs (the catch
// blocks). Stripped of .symtab, those landing pads are reached ONLY by the
// unwinder, so the entry-disc oracles miss them — the LSDA call-site table is
// the only source that names them.
#include <stdexcept>
#include <cstdio>

// A throwing helper (out-of-line so the call site is a real call, with an FDE).
__attribute__((noinline))
int may_throw(int x) {
    if (x < 0) throw std::runtime_error("neg");
    if (x == 0) throw 42;            // a second throw type → second catch arm
    return x * 3 + 7;
}

__attribute__((noinline))
int guarded(int x) {
    try {
        return may_throw(x);
    } catch (const std::runtime_error &e) {   // landing pad #1
        std::printf("runtime_error: %s\n", e.what());
        return -1;
    } catch (int v) {                          // landing pad #2
        std::printf("int: %d\n", v);
        return -2;
    }
}

int main(int argc, char **argv) {
    int acc = 0;
    for (int i = 0; i < argc; ++i)
        acc += guarded(i - 1);
    std::printf("%d\n", acc);
    return acc;
}
