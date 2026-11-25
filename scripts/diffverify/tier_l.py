"""diffverify Tier L: kuna's lift of one instruction against Unicorn.

`kuna-verify step` runs one instruction's SLEIGH p-code from a random state of
the named registers and prints what it read, wrote and where it went next.
This driver runs the same bytes from the same state under Unicorn (QEMU's TCG
as a library) and compares the registers, the flags the architecture defines,
the memory written and the next address.

    python3 scripts/diffverify/tier_l.py x86-64 BINARY [--limit N] [--states N]

Prints one line per instruction form that differs, and a summary. Memory the
kuna side read is mapped for Unicorn with the same bytes; a byte Unicorn reads
that kuna did not is itself a difference.
"""

import argparse
import collections
import json
import os
import subprocess
import sys

import unicorn
from unicorn import arm64_const as A64
from unicorn import arm_const as ARM
from unicorn import x86_const as X86

PAGE = 0x1000
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


class Arch:
    def __init__(self, name, uc_arch, uc_mode, regs, flags, pc, undefined, spec_gaps=()):
        self.name, self.uc_arch, self.uc_mode = name, uc_arch, uc_mode
        self.regs, self.flags, self.pc, self.undefined = regs, flags, pc, undefined
        # Flags the SLEIGH spec does not compute at all, counted apart.
        self.spec_gaps = set(spec_gaps)


X86_REGS = {n: getattr(X86, f"UC_X86_REG_{n}") for n in
            ["RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", "RSP", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15"]
            + [f"XMM{i}" for i in range(16)]}
X86_REGS["FS_OFFSET"] = X86.UC_X86_REG_FS_BASE
X86_FLAGS = {"CF": 0, "PF": 2, "AF": 4, "ZF": 6, "SF": 7, "DF": 10, "OF": 11}
ALL6 = {"CF", "PF", "AF", "ZF", "SF", "OF"}

# Flags the Intel SDM leaves undefined after an instruction, by mnemonic.
X86_UNDEFINED = {
    **{m: {"AF"} for m in ["and", "or", "xor", "test"]},
    **{m: {"SF", "ZF", "AF", "PF"} for m in ["mul", "imul"]},
    **{m: ALL6 for m in ["div", "idiv"]},
    **{m: {"AF", "OF"} for m in ["shl", "sal", "shr", "sar", "shld", "shrd"]},
    **{m: {"OF"} for m in ["rol", "ror", "rcl", "rcr"]},
    **{m: {"CF", "OF", "SF", "AF", "PF"} for m in ["bsf", "bsr"]},
    **{m: {"OF", "SF", "AF", "PF"} for m in ["bt", "bts", "btr", "btc"]},
    **{m: {"CF", "OF", "SF", "AF", "PF"} for m in ["lzcnt", "tzcnt"]},
}
ARM_CONDITIONS = {"eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le"}


def in_it_block(form):
    """A Thumb IT instruction, or one inside an IT block: its condition comes
    from ITSTATE, which one instruction run alone does not carry."""
    mnemonic = form.split()[0] if form.split() else ""
    parts = mnemonic.split(".")
    return parts[0] in ("it", "itt", "ite", "ittt", "itte", "itet", "itee", "itttt", "ittte", "ittet", "ittee",
                        "itett", "itete", "iteet", "iteee") or any(p in ARM_CONDITIONS for p in parts[1:])


# Privileged or system instructions whose effect is the kernel's.
X86_SKIP = {"hlt", "syscall", "int", "int3", "cpuid", "rdtsc", "rdtscp", "xgetbv"}

A64_REGS = {**{f"x{i}": getattr(A64, f"UC_ARM64_REG_X{i}") for i in range(29)},
            "x29": A64.UC_ARM64_REG_X29, "x30": A64.UC_ARM64_REG_X30, "sp": A64.UC_ARM64_REG_SP}
A64_FLAGS = {"NG": 31, "ZR": 30, "CY": 29, "OV": 28}

ARM_REGS = {**{f"r{i}": getattr(ARM, f"UC_ARM_REG_R{i}") for i in range(13)}, "sp": ARM.UC_ARM_REG_SP, "lr": ARM.UC_ARM_REG_LR,
            **{f"s{i}": getattr(ARM, f"UC_ARM_REG_S{i}") for i in range(32)}}
ARM_FLAGS = {"NG": 31, "ZR": 30, "CY": 29, "OV": 28}

ARCHES = {
    # Ghidra's x86 spec leaves AF alone in addflags/subflags and most other
    # arithmetic: a known gap no decompiled C reads.
    "x86-64": Arch("x86-64", unicorn.UC_ARCH_X86, unicorn.UC_MODE_64, X86_REGS, X86_FLAGS, X86.UC_X86_REG_RIP, X86_UNDEFINED, ["AF"]),
    "aarch64": Arch("aarch64", unicorn.UC_ARCH_ARM64, unicorn.UC_MODE_ARM, A64_REGS, A64_FLAGS, A64.UC_ARM64_REG_PC, {}),
    "thumb": Arch("thumb", unicorn.UC_ARCH_ARM, unicorn.UC_MODE_THUMB | unicorn.UC_MODE_MCLASS, ARM_REGS, ARM_FLAGS, ARM.UC_ARM_REG_PC, {}),
}


def flags_reg(arch):
    return {"x86-64": X86.UC_X86_REG_EFLAGS, "aarch64": A64.UC_ARM64_REG_NZCV, "thumb": ARM.UC_ARM_REG_CPSR}[arch.name]


def run_unicorn(arch, case):
    addr = int(case["addr"], 16)
    code = bytes.fromhex(case["bytes"])
    reads = {int(a, 16): b for a, b in case["reads"].items()}
    uc = unicorn.Uc(arch.uc_arch, arch.uc_mode)
    mapped = set()

    def ensure(at, size=1):
        for page in range(at & ~(PAGE - 1), (at + size + PAGE - 1) & ~(PAGE - 1), PAGE):
            if page not in mapped:
                uc.mem_map(page, PAGE)
                mapped.add(page)

    ensure(addr, len(code))
    for a, b in reads.items():
        ensure(a)
        uc.mem_write(a, bytes([b]))
    uc.mem_write(addr, code)
    for name, v in case["in"].items():
        if name in arch.regs:
            uc.reg_write(arch.regs[name], int(v, 16))
    word = 0
    for name, bit in arch.flags.items():
        if name in case["in"]:
            word |= (int(case["in"][name], 16) & 1) << bit
    if arch.name == "x86-64":
        word |= 2
    elif arch.name == "thumb":
        word |= 1 << 24
    uc.reg_write(flags_reg(arch), word)
    seen_reads, writes, stray = set(), {}, set()

    def on_read(_uc, _access, at, size, _value, _data):
        for i in range(size):
            seen_reads.add(at + i)

    def on_write(_uc, _access, at, size, value, _data):
        for i in range(size):
            writes[at + i] = (value >> (8 * i)) & 0xff

    def on_unmapped(_uc, access, at, size, _value, _data):
        if access == unicorn.UC_MEM_READ_UNMAPPED:
            for i in range(size):
                stray.add(at + i)
        ensure(at, size)
        return True

    uc.hook_add(unicorn.UC_HOOK_MEM_READ, on_read)
    uc.hook_add(unicorn.UC_HOOK_MEM_WRITE, on_write)
    uc.hook_add(unicorn.UC_HOOK_MEM_UNMAPPED, on_unmapped)
    start = addr | 1 if arch.name == "thumb" else addr
    uc.emu_start(start, 0xffff_ffff_ffff_ffff if arch.name != "thumb" else 0xffff_ffff, count=1)
    out = {name: uc.reg_read(r) for name, r in arch.regs.items()}
    word = uc.reg_read(flags_reg(arch))
    for name, bit in arch.flags.items():
        out[name] = (word >> bit) & 1
    pc = uc.reg_read(arch.pc)
    return out, writes, pc, (seen_reads | stray) - set(reads) - set(range(addr, addr + len(code)))


def compare(arch, case):
    if arch.name == "x86-64" and case["form"].split()[:1] and case["form"].split()[0] in X86_SKIP:
        return "skip", "system instruction"
    if arch.name == "thumb" and in_it_block(case["form"]):
        return "skip", "inside an IT block"
    if case.get("error") or case.get("unlisted"):
        return "skip", case.get("error") or "reads " + ",".join(case["unlisted"])
    try:
        out, writes, pc, extra = run_unicorn(arch, case)
    except unicorn.UcError as e:
        return "skip", f"unicorn: {e}"
    mnemonic = case["form"].split()[0] if case["form"].split() else ""
    undefined = arch.undefined.get(mnemonic, set())
    diffs, gaps = [], []
    for name, v in case["out"].items():
        if name in undefined or name not in out:
            continue
        if int(v, 16) != out[name]:
            (gaps if name in arch.spec_gaps else diffs).append(f"{name}: kuna {v}, unicorn {out[name]:#x}")
    kw = {int(a, 16): b for a, b in case["writes"].items()}
    if kw != writes:
        only_k = sorted(set(kw) - set(writes))
        only_u = sorted(set(writes) - set(kw))
        bad = sorted(a for a in set(kw) & set(writes) if kw[a] != writes[a])
        diffs.append(f"writes: kuna-only {[hex(a) for a in only_k[:4]]} unicorn-only {[hex(a) for a in only_u[:4]]} differing {[hex(a) for a in bad[:4]]}")
    if case.get("next") and int(case["next"], 16) != pc:
        diffs.append(f"next: kuna {case['next']}, unicorn {pc:#x}")
    if extra:
        diffs.append(f"unicorn reads bytes kuna did not: {[hex(a) for a in sorted(extra)[:4]]}")
    if diffs:
        return "differs", "; ".join(diffs)
    return ("gap", "; ".join(gaps)) if gaps else ("same", "")


def main():
    p = argparse.ArgumentParser()
    p.add_argument("arch", choices=sorted(ARCHES))
    p.add_argument("binary")
    p.add_argument("--limit", type=int, default=2000)
    p.add_argument("--states", type=int, default=4)
    p.add_argument("--per-form", type=int, default=2)
    p.add_argument("--kuna-verify", default=os.path.join(ROOT, "decompiler/target/release/kuna-verify"))
    p.add_argument("--sleighpath", default=os.path.join(ROOT, "specs"))
    p.add_argument("--json", action="store_true")
    a = p.parse_args()
    arch = ARCHES[a.arch]
    cmd = [a.kuna_verify, "step", a.binary, "--regs", ",".join(arch.regs), "--flags", ",".join(arch.flags),
           "--states", str(a.states), "--per-form", str(a.per_form), "--limit", str(a.limit), "--sleighpath", a.sleighpath]
    if arch.name in ("x86-64", "aarch64"):
        cmd.append("--canonical")
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode != 0:
        sys.exit(proc.stderr.strip())
    by_form = collections.defaultdict(list)
    for line in proc.stdout.splitlines():
        case = json.loads(line)
        by_form[case["form"]].append((case, compare(arch, case)))
    counts = collections.Counter()
    for form, results in sorted(by_form.items()):
        kinds = {k for _, (k, _) in results}
        kind = next((k for k in ("differs", "gap", "same") if k in kinds), "skip")
        counts[kind] += 1
        if kind == "differs":
            case, (_, why) = next(r for r in results if r[1][0] == "differs")
            if a.json:
                print(json.dumps({"form": form, "asm": case["asm"], "addr": case["addr"], "why": why}))
            else:
                print(f"DIFFERS {case['addr']} {case['asm']:<40} {why}")
    skips = collections.Counter(why.split(":")[0][:40] for rs in by_form.values() for _, (k, why) in rs if k == "skip")
    print(f"forms {len(by_form)}  same {counts['same']}  differs {counts['differs']}  "
          f"spec-gap only {counts['gap']}  skipped {counts['skip']}", file=sys.stderr)
    for why, n in skips.most_common(8):
        print(f"  skipped {n:5}  {why}", file=sys.stderr)


if __name__ == "__main__":
    main()
