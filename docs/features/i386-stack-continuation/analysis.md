# Evidence for i386 stack continuations

The pinned dynamic ESP predicate asks C output to retain the stack-pointer
assignment in a variable-length i386 frame. The authored `dynamic_move`
isolates the underlying problem with flat memory and a single terminal RET.
Its `dynamic_restore` sibling restores the frame before returning.
[Pinned predicate](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_unmapped_register_rendering.py#L19)

On `ed7a24ed0`, the authored moved frame lifts `SUB ESP,EAX` at
`0x8049006`. RET at `0x8049010` loads the target from the live 32-bit
ESP address, increments ESP by four, then returns through that load.
Final high P-code retains ESP-derived store addresses and emits a
`RETURN(#0x0)`. In C, the surviving memory address is rendered from a
synthetic local array.

Let `S` be entry ESP and `N` the word read from `S+4`. Addresses in
this witness are 32-bit and wrap modulo 2³².

| Order | Operation | Address |
| --- | --- | --- |
| 1 | Save incoming EBP | `S-4` |
| 2 | Read the allocation amount | `S+4` |
| 3 | Write the marker | `S-N-8` |
| 4 | Read the caller's return slot | `S` |
| 5 | Write the result of step 4 to the moved stack | `S-N-12` |
| 6 | RET reads its target | `S-N-12` |

Post-RET ESP is `S-N-8`. In the synthetic alias case
`N=0xfffffff8`, step 3 reaches `S` and overwrites the return slot.
Step 4 must read the new word. Capturing the starting word as a constant
target would produce the wrong continuation even if the stack address
arithmetic were correct. The related case `N=0xfffffff4` overwrites the
amount slot after its value has already been read. These are ordered
memory checks over modeled words.

## Runnable symbolic and native observer

Run the Python block from a shell with Python 3 and a C11 compiler.
Set `ESP_SYMBOLIC_ONLY=1` for the formula check alone. The script computes
the expected address, value, and event order from symbolic formulas. Its
separately authored C observer takes an explicit caller ESP, saved EBP,
and continuation word, and exercises those operations through a bounded
scratch word map. It returns a continuation address and post-RET ESP as
data. The caller inputs supply every register value read by the observer.
The observer uses its host ABI for execution and models only aligned
32-bit words in the witness.

```python
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

cases = (0, 4, 20, 0xfffffff4, 0xfffffff8)
mask = 0xffffffff
caller_sp = 0x10008000
caller_ebp = 0x10008808
continuation = 0x08049100
marker = 0x12345678

expected = []
for amount in cases:
    marker_slot = (caller_sp - amount - 8) & mask
    return_slot = (marker_slot - 4) & mask
    late_target = marker if marker_slot == caller_sp else continuation
    rows = [
        ("case", amount),
        ("W", caller_sp - 4, caller_ebp),
        ("R", caller_sp + 4, amount),
        ("W", marker_slot, marker),
        ("R", caller_sp, late_target),
        ("W", return_slot, late_target),
        ("R", return_slot, late_target),
        ("resume", late_target, marker_slot),
    ]
    assert (late_target == marker) == (amount == 0xfffffff8)
    assert (marker_slot == caller_sp + 4) == (amount == 0xfffffff4)
    expected.extend(f"{kind} " + " ".join(f"{value:08x}" for value in values)
                    for kind, *values in rows)

observer = r"""
#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct caller_state {
    uint32_t esp, ebp, continuation, amount;
};

struct continuation_state {
    uint32_t target, esp;
};

struct word {
    uint32_t address, value;
};

struct memory {
    struct word words[5];
    size_t used;
};

static void event(char kind, uint32_t address, uint32_t value) {
    printf("%c %08" PRIx32 " %08" PRIx32 "\n", kind, address, value);
}

static void write_word(struct memory *mem, uint32_t address, uint32_t value,
                       int observed) {
    assert(address % 4 == 0);
    size_t i = 0;
    while (i < mem->used && mem->words[i].address != address)
        ++i;
    if (i == mem->used) {
        assert(mem->used < sizeof mem->words / sizeof mem->words[0]);
        ++mem->used;
    }
    mem->words[i] = (struct word){address, value};
    if (observed)
        event('W', address, value);
}

static uint32_t read_word(const struct memory *mem, uint32_t address) {
    for (size_t i = 0; i < mem->used; ++i) {
        if (mem->words[i].address == address) {
            event('R', address, mem->words[i].value);
            return mem->words[i].value;
        }
    }
    abort();
}

static struct continuation_state observe(struct caller_state caller) {
    struct memory mem = {0};
    write_word(&mem, caller.esp, caller.continuation, 0);
    write_word(&mem, caller.esp + UINT32_C(4), caller.amount, 0);

    uint32_t esp = caller.esp - UINT32_C(4);
    write_word(&mem, esp, caller.ebp, 1);
    uint32_t ebp = esp;
    esp -= read_word(&mem, ebp + UINT32_C(8));
    esp -= UINT32_C(4);
    write_word(&mem, esp, UINT32_C(0x12345678), 1);

    uint32_t late_target = read_word(&mem, ebp + UINT32_C(4));
    esp -= UINT32_C(4);
    write_word(&mem, esp, late_target, 1);
    uint32_t target = read_word(&mem, esp);
    return (struct continuation_state){target, esp + UINT32_C(4)};
}

int main(void) {
    const uint32_t amounts[] = {0, 4, 20, UINT32_C(0xfffffff4),
                                UINT32_C(0xfffffff8)};
    for (size_t i = 0; i < sizeof amounts / sizeof amounts[0]; ++i) {
        printf("case %08" PRIx32 "\n", amounts[i]);
        const struct caller_state caller = {
            UINT32_C(0x10008000), UINT32_C(0x10008808),
            UINT32_C(0x08049100), amounts[i]
        };
        struct continuation_state result = observe(caller);
        printf("resume %08" PRIx32 " %08" PRIx32 "\n",
               result.target, result.esp);
    }
    return 0;
}
"""

assert len(expected) == 8 * len(cases)
assert expected[-1] == "resume 12345678 10008000"
if os.environ.get("ESP_SYMBOLIC_ONLY") == "1":
    print("\n".join(expected))
    print(f"Symbolic chronology checked for {len(cases)} cases.")
else:
    with TemporaryDirectory(prefix="i386-stack-continuation-") as scratch:
        root = Path(scratch)
        source, program = root / "observe.c", root / "observe"
        source.write_text(observer)
        subprocess.run([os.environ.get("CC", "cc"), "-std=c11", "-O2",
                        "-Wall", "-Wextra", "-Werror", str(source), "-o",
                        str(program)], check=True)
        observed = subprocess.check_output([program], text=True).splitlines()
    assert observed == expected, (observed, expected)
    print(f"Native and symbolic chronology agree for {len(cases)} cases.")
```

The proposal's symbolic replay covers five cases. The native branch is a
runnable reviewer check for those same values and events.
