"""Execute the reduced assembly and emitted C against identical scripted calls."""
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
HARNESS = r'''
#include <assert.h>
#include <stdbool.h>
#include <setjmp.h>
#include <stdio.h>
#include <string.h>
typedef int int4;
unsigned long retry_server(unsigned int);
int accept(unsigned int, unsigned long, unsigned long);
int fork(void);
int close(unsigned int);
void warn(int);
void session(unsigned int);
_Noreturn void abort(void);
int *__errno_location(void);
struct scenario { int peers[8], children[8], error, fatal; const char *trace; };
static const struct scenario scenarios[] = {
    {{21}, {0}, 4, 0, "A21 F0 C9 S21 "},
    {{21,22}, {7,0}, 4, 0, "A21 F7 C21 A22 F0 C9 S22 "},
    {{-1,22}, {0}, 4, 0, "A-1 A22 F0 C9 S22 "},
    {{21,22}, {-1,0}, 4, 0, "A21 F-1 W0 C21 A22 F0 C9 S22 "},
    {{-1}, {0}, 5, 1, "A-1 X0 "},
    {{-1,21,22,23}, {-1,7,0}, 4, 0,
     "A-1 A21 F-1 W0 C21 A22 F7 C22 A23 F0 C9 S23 "},
};
static const struct scenario *current;
static unsigned int peer_index, child_index;
static int error_value;
static jmp_buf stopped;
static char trace[512];
static void record(char event, int value) {
    size_t used = strlen(trace);
    assert(used < sizeof(trace) - 32);
    snprintf(trace + used, sizeof(trace) - used, "%c%d ", event, value);
}
int *__errno_location(void) { return &error_value; }
int accept(unsigned int fd, unsigned long addr, unsigned long len) {
    assert(fd == 9 && !addr && !len && peer_index < 8);
    int result = current->peers[peer_index++];
    record('A', result);
    return result;
}
int fork(void) {
    assert(child_index < 8);
    int result = current->children[child_index++];
    record('F', result);
    return result;
}
int close(unsigned int fd) { record('C', fd); return 0; }
void warn(int arg) { assert(!arg); record('W', 0); }
void session(unsigned int peer) { record('S', peer); }
_Noreturn void abort(void) { record('X', 0); longjmp(stopped, 1); }
int main(void) {
    for (unsigned int i = 0; i < sizeof(scenarios)/sizeof(*scenarios); ++i) {
        current = &scenarios[i];
        error_value = current->error;
        peer_index = child_index = 0;
        trace[0] = 0;
        int fatal = setjmp(stopped);
        if (!fatal) assert(retry_server(9) == 0);
        assert(!!fatal == current->fatal);
        assert(strcmp(trace, current->trace) == 0);
        puts(trace);
    }
}
'''


def main():
    engine = ROOT / "decompiler/target/release/decomp_dbg"
    script = "\n".join([
        f"load file {ROOT / 'tests/stages/kuna-loopcontinue.xml'}",
        "option loopcontinue on", "load function retry_server", "decompile",
        "print C", "quit", "",
    ])
    run = subprocess.run([str(engine), "-s", str(ROOT / "specs")],
                         input=script, text=True, capture_output=True, check=True)
    code = re.search(r"\[decomp\]> print C\n(.*?)(?=\[decomp\]>)",
                     run.stdout, re.S).group(1)
    assembly = (Path(__file__).with_name("repro.S").read_text()
                .split("__errno_location: ret")[0])
    assembly += '\n.section .note.GNU-stack,"",@progbits\n'
    with tempfile.TemporaryDirectory(prefix="kuna-loopcontinue-") as directory:
        directory = Path(directory)
        (directory / "harness.c").write_text(HARNESS)
        (directory / "original.S").write_text(assembly)
        (directory / "decompiled.c").write_text(HARNESS + code)
        outputs = []
        for variant, sources in [("original", ["harness.c", "original.S"]),
                                 ("decompiled", ["decompiled.c"])]:
            executable = directory / variant
            subprocess.run(["cc", "-O2", "-fno-builtin", "-no-pie",
                            *[str(directory / source) for source in sources],
                            "-o", str(executable)], check=True)
            outputs.append(subprocess.check_output([str(executable)], text=True))
        assert outputs[0] == outputs[1]
    print("Six scripted paths preserve calls, arguments, retries, closes, and exits.")


if __name__ == "__main__":
    main()
