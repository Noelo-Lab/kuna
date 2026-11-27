"""Execute an explicit DiffVerify function case with Unicorn.

Run with `python3 scripts/diffverify/function_oracle.py CASE.json`.
The request describes original bytes, a CPU model, and the full address
space available to the guest. The runner emits one JSON observation.
"""

import argparse
import json
import sys
from dataclasses import dataclass

import unicorn as uc
from unicorn import x86_const as x86
from oracle_protocol import DEFAULT_LIMITS, MAX_MAPPED, case_digest, load_request

VERSION = 1
PAGE = 4096
MAX_USER_ADDRESS = 1 << 47
GPR = ("RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", "RSP", *(f"R{i}" for i in range(8, 16)))
PRESERVED = ("RBX", "RBP", "R12", "R13", "R14", "R15")
INTEGER_ARGS = ("RDI", "RSI", "RDX", "RCX", "R8", "R9")
VECTOR_ARGS = tuple(f"XMM{i}" for i in range(8))
FP = tuple(f"FP{i}" for i in range(8))
DEFAULTS = dict.fromkeys((*GPR, "FS_BASE", "GS_BASE", *(f"XMM{i}" for i in range(16)), *FP), 0)
DEFAULTS.update(RFLAGS=2, MXCSR=0x1f80, FPCW=0x037f, FPSW=0, FPTAG=0xffff)
REGISTER_WIDTHS = {name: 64 for name in (*GPR, "FS_BASE", "GS_BASE", "RFLAGS")}
REGISTER_WIDTHS.update({f"XMM{i}": 128 for i in range(16)})
REGISTER_WIDTHS.update({name: 80 for name in FP})
REGISTER_WIDTHS.update(MXCSR=32, FPCW=16, FPSW=16, FPTAG=16)
PERMISSIONS = {"r": uc.UC_PROT_READ, "w": uc.UC_PROT_WRITE, "x": uc.UC_PROT_EXEC}


def unsigned(value, bits, name):
    if not isinstance(value, str) or not value.startswith("0x"):
        raise ValueError(f"{name} requires a hexadecimal string")
    digits = value[2:]
    if not digits or any(c not in "0123456789abcdefABCDEF" for c in digits) or len(digits) > (bits + 3) // 4:
        raise ValueError(f"{name} requires an unsigned {bits}-bit value")
    n = int(digits, 16)
    if n >= 1 << bits:
        raise ValueError(f"{name} requires an unsigned {bits}-bit value")
    return n


def count(value, low, high, name):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f"{name} must be an integer in [{low}, {high}]")
    return value


def read_register(machine, name):
    value = machine.reg_read(getattr(x86, f"UC_X86_REG_{name}"))
    if name in FP:
        mantissa, sign_exponent = value
        return mantissa | (sign_exponent << 64)
    return value


def write_register(machine, name, value):
    if name in FP:
        value = (value & ((1 << 64) - 1), value >> 64)
    machine.reg_write(getattr(x86, f"UC_X86_REG_{name}"), value)


@dataclass(frozen=True)
class Region:
    start: int
    size: int
    permissions: str
    data: bytes
    observe: bool

    def contains(self, at, length=1):
        return self.start <= at and at + length <= self.start + self.size


@dataclass(frozen=True)
class Case:
    entry: int
    sentinel: int
    registers: dict
    regions: tuple
    external_entries: frozenset
    instructions: int
    timeout_us: int

    @classmethod
    def parse(cls, spec):
        count(spec["version"], VERSION, VERSION, "version")
        if spec["architecture"] != "x86-64" or spec["abi"] != "sysv-amd64":
            raise ValueError("expected version 1, x86-64, sysv-amd64")
        if spec.get("cpu_model", "qemu64") != "qemu64":
            raise ValueError("unsupported x86 CPU model")
        entry = unsigned(spec["entry"], 47, "entry")
        sentinel = unsigned(spec["return_address"], 47, "return_address")
        if not sentinel:
            raise ValueError("return_address must be nonzero")
        registers = DEFAULTS.copy()
        supplied = spec["registers"]
        if "RSP" not in supplied:
            raise ValueError("the caller must supply RSP")
        for name, value in supplied.items():
            if name not in REGISTER_WIDTHS:
                raise ValueError(f"unsupported register {name}")
            registers[name] = unsigned(value, REGISTER_WIDTHS[name], name)
        if registers["RFLAGS"] & ~0xed7 or not (registers["RFLAGS"] & 2):
            raise ValueError("only the arithmetic flags, DF, IF and fixed bit 1 are supported")
        if registers["RSP"] % 16 != 8:
            raise ValueError("SysV AMD64 function entry requires RSP mod 16 = 8")
        regions = []
        for item in spec["memory"]:
            at = unsigned(item["address"], 47, "memory address")
            size = count(item["size"], PAGE, MAX_MAPPED, "region size")
            if (at | size) & (PAGE - 1) or at + size > MAX_USER_ADDRESS:
                raise ValueError("memory regions must be page aligned within the low canonical half")
            permissions = item["permissions"]
            if permissions not in ("r", "rw", "rx", "x"):
                raise ValueError(f"unsupported memory permissions {permissions!r}")
            data = bytes.fromhex(item.get("bytes", ""))
            if len(data) > size:
                raise ValueError("region bytes exceed its declared size")
            observe = item.get("observe", False)
            if type(observe) is not bool or (observe and permissions != "rw"):
                raise ValueError("observe requires a writable region")
            regions.append(Region(at, size, permissions, data, observe))
        regions.sort(key=lambda r: r.start)
        if sum(r.size for r in regions) > MAX_MAPPED or any(a.start + a.size > b.start for a, b in zip(regions, regions[1:])):
            raise ValueError("memory regions overlap or exceed the mapping budget")
        if not any(r.contains(entry) and "x" in r.permissions for r in regions):
            raise ValueError("entry is outside executable memory")
        if any(r.contains(sentinel) for r in regions):
            raise ValueError("return address must be an unmapped sentinel")
        rsp = registers["RSP"]
        stack = next((r for r in regions if r.contains(rsp, 8) and r.permissions == "rw"), None)
        if stack is None:
            raise ValueError("RSP must name a writable region")
        off = rsp - stack.start
        if int.from_bytes(stack.data[off:off + 8], "little") != sentinel:
            raise ValueError("the initial stack return slot must contain the sentinel")
        external = frozenset(unsigned(v, 47, "external entry") for v in spec.get("external_entries", []))
        if entry in external or sentinel in external:
            raise ValueError("external entries cannot coincide with the entry or return address")
        limit = spec.get("limits", {})
        instructions = count(limit.get("instructions", DEFAULT_LIMITS["instructions"]), 1, 1_000_000, "instruction limit")
        timeout_us = count(limit.get("timeout_us", DEFAULT_LIMITS["timeout_us"]), 1, 10_000_000, "timeout_us")
        return cls(entry, sentinel, registers, tuple(regions), external, instructions, timeout_us)


def near_return(code):
    return code == b"\xc3" or code == b"\xf3\xc3" or (len(code) == 3 and code[0] == 0xc2)


def unmodeled_environment(code):
    prefixes = b"\xf0\xf2\xf3\x2e\x36\x3e\x26\x64\x65\x66\x67" + bytes(range(0x40, 0x50))
    instruction = code.lstrip(prefixes)
    if instruction in (b"\x0f\x31", b"\x0f\x01\xf9"):
        return "timestamp-counter"
    if instruction[:2] == b"\x0f\xc7" and len(instruction) == 3 and instruction[2] >= 0xf0:
        return "hardware-random"
    if instruction and instruction[0] in range(0x6c, 0x70):
        return "string-port-io"
    return None


class UnicornBackend:
    """One execution backend for the versioned function case."""

    def run(self, case):
        machine = uc.Uc(uc.UC_ARCH_X86, uc.UC_MODE_64)
        machine.ctl_set_cpu_model(x86.UC_CPU_X86_QEMU64)
        for region in case.regions:
            mask = sum(PERMISSIONS[p] for p in region.permissions)
            machine.mem_map(region.start, region.size, mask)
            if region.data:
                machine.mem_write(region.start, region.data)
        for name, value in case.registers.items():
            write_register(machine, name, value)

        status, reason, event = None, None, None
        steps, last_pc, last_returns = 0, None, False

        def stop(outcome, why, observed_event=None):
            nonlocal status, reason, event
            status, reason, event = outcome, why, observed_event
            machine.emu_stop()

        def regs(names):
            return {name: hex(read_register(machine, name)) for name in names}

        def external(at):
            stop("incomplete", "external-entry", {
                "kind": "external-entry", "target": hex(at), "from": None if last_pc is None else hex(last_pc),
                "registers": regs((*INTEGER_ARGS, *VECTOR_ARGS, "RAX", "RSP")),
            })

        def on_code(_machine, at, size, _user):
            nonlocal steps, last_pc, last_returns
            if at in case.external_entries:
                external(at)
                return
            steps += 1
            last_pc = at
            last_returns = False
            if not 1 <= size <= 15:
                return
            code = bytes(machine.mem_read(at, size))
            last_returns = near_return(code)
            boundary = unmodeled_environment(code)
            if boundary:
                stop("incomplete", boundary, {"kind": boundary, "address": hex(at)})

        def on_invalid_memory(_machine, access, at, size, _value, _user):
            if access == uc.UC_MEM_FETCH_UNMAPPED and at in case.external_entries:
                external(at)
            else:
                stop("fault", "memory-access", {"kind": "memory-access", "access": access, "address": hex(at), "size": size})
            return False

        def on_syscall(_machine, _user):
            stop("incomplete", "system-call", {
                "kind": "system-call", "address": hex(last_pc),
                "registers": regs(("RAX", "RDI", "RSI", "RDX", "R10", "R8", "R9")),
            })

        def on_interrupt(_machine, number, _user):
            stop("trap", "interrupt", {"kind": "interrupt", "number": number})

        def on_port_in(_machine, port, size, _user):
            stop("incomplete", "port-read", {"kind": "port-read", "port": port, "size": size})
            return 0

        def on_port_out(_machine, port, size, value, _user):
            stop("incomplete", "port-write", {"kind": "port-write", "port": port, "size": size, "value": hex(value)})

        machine.hook_add(uc.UC_HOOK_CODE, on_code)
        machine.hook_add(uc.UC_HOOK_MEM_INVALID, on_invalid_memory)
        machine.hook_add(uc.UC_HOOK_INTR, on_interrupt)
        machine.hook_add(uc.UC_HOOK_INSN, on_syscall, None, 1, 0, x86.UC_X86_INS_SYSCALL)
        machine.hook_add(uc.UC_HOOK_INSN, on_syscall, None, 1, 0, x86.UC_X86_INS_SYSENTER)
        machine.hook_add(uc.UC_HOOK_INSN, on_port_in, None, 1, 0, x86.UC_X86_INS_IN)
        machine.hook_add(uc.UC_HOOK_INSN, on_port_out, None, 1, 0, x86.UC_X86_INS_OUT)

        try:
            machine.emu_start(case.entry, case.sentinel, timeout=case.timeout_us, count=case.instructions)
        except uc.UcError as error:
            if status is None:
                status = "trap" if error.errno == uc.UC_ERR_INSN_INVALID else "incomplete"
                reason = f"unicorn-error-{error.errno}"

        rip = machine.reg_read(x86.UC_X86_REG_RIP)
        rsp = read_register(machine, "RSP")
        if status is None:
            if machine.query(uc.UC_QUERY_TIMEOUT):
                status, reason = "incomplete", "timeout"
            elif rip == case.sentinel:
                if last_returns:
                    status = "returned"
                else:
                    status, reason = "incomplete", "non-return-transfer-to-sentinel"
            elif steps >= case.instructions:
                status, reason = "incomplete", "instruction-limit"
            else:
                status, reason = "incomplete", "stopped-before-return"

        return {
            "version": VERSION,
            "backend": "unicorn",
            "backend_version": uc.__version__,
            "cpu_model": "qemu64",
            "status": status,
            "reason": reason,
            "event": event,
            "instructions": steps,
            "pc": hex(rip),
            "registers": regs(DEFAULTS),
            "memory": [
                {"address": hex(r.start), "bytes": bytes(machine.mem_read(r.start, r.size)).hex()}
                for r in case.regions if r.observe
            ],
            "abi_violations": [
                name for name in PRESERVED if read_register(machine, name) != case.registers[name]
            ] + (["RSP"] if rsp != case.registers["RSP"] + 8 else []) if status == "returned" else [],
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("case", help="JSON case file, or - for standard input")
    args = parser.parse_args()
    try:
        if args.case == "-":
            request = load_request(sys.stdin.buffer)
        else:
            with open(args.case, "rb") as file:
                request = load_request(file)
        spec = request.get("case", request)
        observation = UnicornBackend().run(Case.parse(spec))
        observation["case_sha256"] = case_digest(spec)
        print(json.dumps(observation, sort_keys=True))
    except (AttributeError, KeyError, TypeError, ValueError, OSError, uc.UcError) as error:
        parser.exit(2, f"function oracle: {error}\n")


if __name__ == "__main__":
    main()
