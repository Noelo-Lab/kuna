#!/usr/bin/env python3
"""Emit independent callwide sources and commands; never run a toolchain."""

import argparse
import hashlib
import json
from pathlib import Path
import struct


ROOT = Path(__file__).resolve().parent
COMMON = [
    "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-fno-pic",
    "-fno-pie", "-fno-unwind-tables", "-fno-asynchronous-unwind-tables",
    "-fno-inline", "-fno-optimize-sibling-calls", "-O2",
]
ARCH = {
    "arm32-be": {
        "source": "arm32_be.S", "start": "start_arm32_be.S",
        "flags": ["--target=armeb-linux-gnueabi", "-march=armv5te", "-marm", "-mfloat-abi=soft"],
        "qemu": "qemu-armeb", "machine": 40,
    },
    "mips32-be": {
        "source": "mips32_be.S", "start": "start_mips32_be.S",
        "flags": ["--target=mips-linux-gnu", "-march=mips32", "-mabi=32", "-msoft-float", "-mno-abicalls", "-G0"],
        "qemu": "qemu-mips", "machine": 8,
    },
}
CASES = {
    "positive": [], "xor-sink": [], "identity-sink": ["CALLWIDE_IDENTITY_SINK"],
    "mixed-refusal": ["CALLWIDE_MIXED"],
    "missing-root": ["CALLWIDE_MISSING_ROOT"], "swapped": ["CALLWIDE_SWAP"],
    "partial": ["CALLWIDE_PARTIAL"], "spill": ["CALLWIDE_SPILL"],
    "unrelated": ["CALLWIDE_UNRELATED"], "arithmetic": ["CALLWIDE_ARITHMETIC"],
    "intervening-call": ["CALLWIDE_INTERVENING_CALL"],
    "branch": ["CALLWIDE_BRANCH"], "opaque": ["CALLWIDE_OPAQUE"],
    "stale-word": ["CALLWIDE_STALE"],
    "two-formals": ["CALLWIDE_SINK=consume_two32"],
    "wrong-slot": ["CALLWIDE_SINK=consume_slot"],
    "low-only": [], "late-low-only": [], "site-limit": [],
    "address-table": ["CALLWIDE_ADDRESS_TABLE"],
    "ignored-zero": ["CALLWIDE_IGNORED_ZERO", "CALLWIDE_MISSING_ROOT"],
    "exported": ["CALLWIDE_EXPORT"], "declared-narrow": [],
    "jalr": ["CALLWIDE_JALR"], "pointer-ra": ["CALLWIDE_POINTER_RA"],
    "constant-ra": ["CALLWIDE_CONSTANT_RA"],
    "wrong-ra-slot": ["CALLWIDE_WRONG_RA_SLOT"],
    "alias-ra-slot": ["CALLWIDE_ALIAS_RA"],
    "bad-sp-delay": ["CALLWIDE_BAD_SP_DELAY"],
    "unknown-store": ["CALLWIDE_UNKNOWN_STORE"],
    "reserved-gap": ["CALLWIDE_RESERVED_WORD"], "pic": [],
}
MIPS_ONLY = {
    "ignored-zero", "missing-root", "mixed-refusal",
    "jalr", "pointer-ra", "constant-ra", "wrong-ra-slot", "alias-ra-slot",
    "bad-sp-delay", "unknown-store", "reserved-gap", "pic",
}
CALLERS = {
    "caller64": "unsigned long long", "caller_high32": "unsigned int",
    "caller_high16": "unsigned short", "caller_high8": "unsigned char",
    "caller_bool": "bool", "caller_ignored": "unsigned int",
}
LINKER = """ENTRY(caller64)
PHDRS {
  code PT_LOAD FLAGS(5);
  constants PT_LOAD FLAGS(4);
  runtime PT_LOAD FLAGS(5);
  runtime_data PT_LOAD FLAGS(6);
}
SECTIONS {
  . = 0x100000;
  .text.callwide : {
    *(.text)
    KEEP(*(.text.callwide))
    KEEP(*(.text.callwide_*))
    KEEP(*(.text.callwide.root))
    KEEP(*(.text.callwide.mixed))
  } :code
  . = 0x180000;
  .rodata.callwide : { *(.rodata.callwide) } :constants
  .MIPS.abiflags : { *(.MIPS.abiflags) } :constants
  .reginfo : { *(.reginfo) } :constants
  . = 0x200000;
  .text.harness : { *(.text.harness*) } :runtime
  .rodata.harness : { *(.rodata.harness*) *(.rodata .rodata.*) } :runtime_data
  .data : { *(.data .data.*) } :runtime_data
  .bss : { *(.bss .bss.* COMMON) } :runtime_data
  /DISCARD/ : { *(.comment) *(.note*) *(.eh_frame*) *(.ARM.exidx*) *(.ARM.extab*) *(.got .got.plt) }
}
"""


def executable_snapshot(path, all_functions=False):
    """Read final ELF32-BE code bytes and function symbols without external tools."""
    data = Path(path).read_bytes()
    if data[:6] != b"\x7fELF\x01\x02":
        raise ValueError("expected an ELF32 big-endian executable")
    header = struct.unpack_from(">HHIIIIIHHHHHH", data, 16)
    kind, machine = header[:2]
    if kind != 2 or machine not in {8, 40}:
        raise ValueError("expected fixed-address ARM/MIPS ET_EXEC")
    executable = {}
    for index in range(header[9]):
        segment = struct.unpack_from(">IIIIIIII", data, header[4] + index * header[8])
        kind, offset, address, _, file_size, memory_size, flags, _ = segment
        if kind != 1 or not flags & 1 or not memory_size:
            continue
        if file_size != memory_size or flags != 5:
            raise ValueError("fixture code must be fully file-backed and read/execute only")
        executable[address] = data[offset:offset + file_size]
    shoff, shentsize, shnum, shstrndx = header[5], header[10], header[11], header[12]
    sections = [struct.unpack_from(">IIIIIIIIII", data, shoff + i * shentsize) for i in range(shnum)]
    names = sections[shstrndx]
    strings = data[names[4]:names[4] + names[5]]

    def string(table, offset):
        return table[offset:table.index(b"\0", offset)].decode()

    code = {}
    functions = {}
    for section in sections:
        name = string(strings, section[0])
        if name == ".text.callwide":
            code[name] = (section[3], data[section[4]:section[4] + section[5]])
        if section[1] != 2:
            continue
        symstrings = sections[section[6]]
        table = data[symstrings[4]:symstrings[4] + symstrings[5]]
        for at in range(section[4], section[4] + section[5], section[9]):
            symname, value, size, info, _, shndx = struct.unpack_from(">IIIBBH", data, at)
            if info & 15 != 2 or not shndx or shndx >= len(sections):
                continue
            owner = sections[shndx]
            if not all_functions and string(strings, owner[0]) != ".text.callwide":
                continue
            start = owner[4] + value - owner[3]
            body = data[start:start + size]
            functions[string(table, symname)] = {
                "address": value, "size": size,
                "sha256": hashlib.sha256(body).hexdigest(),
            }
    if not code or not functions or any(not value["size"] for value in functions.values()):
        raise ValueError("missing fixed fixture code/function sizes")
    return machine, code, functions, executable


def verify(proof, runtime):
    proof_machine, proof_code, proof_functions, proof_exec = executable_snapshot(proof)
    runtime_machine, runtime_code, runtime_functions, runtime_exec = executable_snapshot(runtime)
    if (proof_machine, proof_code, proof_functions) != (runtime_machine, runtime_code, runtime_functions):
        raise ValueError("runtime and proof images have different fixture addresses or bytes")
    section_address, section_bytes = proof_code[".text.callwide"]
    if proof_exec != {section_address: section_bytes}:
        raise ValueError("proof executable bytes must all belong to the fixture section")
    if any(runtime_exec.get(address) != body for address, body in proof_exec.items()):
        raise ValueError("runtime and proof fixture executable segments differ")
    return {"identical_fixture_bytes": True, "machine": proof_machine, "functions": proof_functions,
            "executable_sha256": hashlib.sha256(section_bytes).hexdigest()}


def verify_selected(proof, runtime, selected):
    proof_machine, proof_code, proof_functions, proof_exec = executable_snapshot(proof)
    runtime_machine, runtime_code, runtime_functions, runtime_exec = executable_snapshot(runtime, True)
    if (proof_machine, proof_code) != (runtime_machine, runtime_code):
        raise ValueError("selected replay changed the target section or architecture")
    address, body = proof_code[".text.callwide"]
    if proof_exec != {address: body} or runtime_exec.get(address) != body:
        raise ValueError("selected replay changed the complete target executable segment")
    end = address + len(body)
    if any(start != address and start < end and start + len(data) > address
           for start, data in runtime_exec.items()):
        raise ValueError("another executable segment overlaps the original target")
    if selected not in proof_functions:
        raise ValueError("selected function does not belong to the original target")
    expected_target_names = set()
    for name, function in proof_functions.items():
        native = "callwide_native_" + name
        expected_target_names.add(native)
        if runtime_functions.get(native) != function:
            raise ValueError(f"native function ownership changed: {name}")
        if name != selected:
            expected_target_names.add(name)
            if runtime_functions.get(name) != function:
                raise ValueError(f"unselected native alias changed: {name}")
    actual_target_names = {name for name, function in runtime_functions.items()
                           if address <= function["address"] < end}
    if actual_target_names != expected_target_names:
        raise ValueError("selected replay has unexpected target function ownership")
    emitted = runtime_functions.get(selected)
    if emitted is None or address <= emitted["address"] < end:
        raise ValueError("selected public symbol must belong to recompiled C outside target RX")
    if not any(start != address and start <= emitted["address"]
               and emitted["address"] + emitted["size"] <= start + len(data)
               for start, data in runtime_exec.items()):
        raise ValueError("selected C body is not fully owned by a separate executable segment")
    return {"identical_fixture_bytes": True, "machine": proof_machine,
            "native_function_prefix": "callwide_native_", "functions": proof_functions,
            "selected_recompiled_function": {"name": selected, **emitted},
            "executable_sha256": hashlib.sha256(body).hexdigest()}


def emit(args):
    arch = ARCH[args.arch]
    if args.case in MIPS_ONLY and args.arch != "mips32-be":
        raise ValueError("this admission/return-token refusal is MIPS-specific")
    if args.case == "declared-narrow" and args.mode != "declared":
        raise ValueError("the declared-narrow refusal requires its explicit producer declaration")
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    source = (ROOT / arch["source"]).read_text()
    consumers = (ROOT / "consumers.c").read_text()
    extra = []
    extra_count = {"late-low-only": 26, "site-limit": 251}.get(args.case, 0)
    for index in range(extra_count):
        extra.append((f"caller_extra_{index:03}", 32))
    if args.case in {"low-only", "late-low-only"}:
        extra.append(("caller_low_only", -1))
    source = source.replace("/* CALLWIDE_EXTRA_CALLERS */", "\n".join(f"CW_HIGH {name},{width}" for name, width in extra))
    for name, _ in extra:
        consumers += (
            "\n#if CALLWIDE_DWARF_CALLER_CONTRACTS\n"
            f"extern u32 {name}(u64 left, u64 right);\n"
            f"SINK(contract_{name}) u32 contract_{name}(u64 left, u64 right)\n"
            "{\n"
            f"    return {name}(left, right);\n"
            "}\n#endif\n"
        )
    (out / "fixture.S").write_text(source)
    (out / "consumers.c").write_text(consumers)
    for name, origin in [("oracle.c", "oracle.c"), ("start.S", arch["start"])]:
        (out / name).write_text((ROOT / origin).read_text())
    proof_linker = LINKER.replace("  runtime PT_LOAD FLAGS(5);\n", "").replace(
        "  .text.harness : { *(.text.harness*) } :runtime\n", "")
    entry = "callwide_entry" if args.arch == "mips32-be" and "CALLWIDE_MISSING_ROOT" not in CASES[args.case] else "caller64"
    proof_linker = proof_linker.replace("ENTRY(caller64)", f"ENTRY({entry})")
    (out / "layout.ld").write_text(proof_linker)
    (out / "runtime.ld").write_text(LINKER)
    flags = [*arch["flags"], *COMMON]
    if args.case == "pic":
        flags = [flag for flag in flags if flag not in {"-mno-abicalls", "-fno-pic", "-fno-pie"}]
        flags += ["-fPIC", "-mabicalls"]
    defines = ["-D" + name for name in CASES[args.case]]
    dwarf = args.mode == "consumer-dwarf"
    cc = [args.cc, *flags]
    consumer_debug = ["-g", "-gdwarf-4"] if dwarf else ["-g0"]
    commands = [
        [*cc, "-g0", *defines, "-c", str(out / "fixture.S"), "-o", str(out / "fixture.o")],
        [*cc, *consumer_debug, *defines, f"-DCALLWIDE_DWARF_CALLER_CONTRACTS={int(dwarf)}", "-c", str(out / "consumers.c"), "-o", str(out / "consumers.o")],
    ]
    link = [args.ld, "-EB", "-static", "--build-id=none", "-z", "max-page-size=4096"]
    if args.case == "exported":
        link += ["--export-dynamic"]
    commands.append([*link, "-T", str(out / "layout.ld"), str(out / "fixture.o"), str(out / "consumers.o"), "-o", str(out / "proof.elf")])
    commands += [
        [*cc, "-g0", *defines, f"-DCALLWIDE_DWARF_CALLER_CONTRACTS={int(dwarf)}", "-c", str(out / "oracle.c"), "-o", str(out / "oracle.o")],
        [*cc, "-g0", "-c", str(out / "start.S"), "-o", str(out / "start.o")],
        [*link, "-T", str(out / "runtime.ld"), "-e", "_start", str(out / "fixture.o"), str(out / "consumers.o"), str(out / "oracle.o"), str(out / "start.o"), "-o", str(out / "runtime.elf")],
    ]
    types = CALLERS | {name: "unsigned int" for name, _ in extra}
    if args.case == "mixed-refusal":
        types["caller_rejected"] = "unsigned long long"
    assertions = [
        "prototype consume64 unsigned long long consume64(unsigned long long value)",
        "prototype consume_two32 unsigned int consume_two32(unsigned int high,unsigned int low)",
        "prototype consume_slot unsigned long long consume_slot(unsigned int tag,unsigned long long value)",
        *[f"prototype {name} {ret} {name}(unsigned long long left,unsigned long long right)" for name, ret in types.items()],
    ]
    if args.case == "declared-narrow":
        assertions.append("prototype produce_sum unsigned int produce_sum(unsigned long long left,unsigned long long right)")
    (out / "assertions.txt").write_text("\n".join(assertions) + "\n")
    manifest = {
        "architecture": args.arch, "case": args.case, "mode": args.mode,
        "status": "authored; builds and all proof/runtime checks unverified",
        "expected": "admit-produce_sum-refuse-produce_rejected" if args.case == "mixed-refusal" else "admit-produce_sum" if args.arch == "mips32-be" and args.case in {"positive", "xor-sink", "identity-sink"} else "refuse-produce_sum-entirely",
        "direct_producer_sites": 6 + len(extra),
        "commands": commands,
        "native_oracle": [arch["qemu"], str(out / "runtime.elf")] if args.case in {"positive", "xor-sink", "identity-sink", "mixed-refusal"} else None,
        "mandatory_pre_native_check": ["python3", str(ROOT / "generate.py"), "--verify", str(out / "proof.elf"), str(out / "runtime.elf")],
        "proof_image": str(out / "proof.elf"),
        "proof_entry": entry,
        "runtime_image_excluded_from_admission": str(out / "runtime.elf"),
        "consumer_only_dwarf": dwarf,
        "producer_dwarf_or_declaration": args.case == "declared-narrow",
        "dwarf_validation_required": dwarf,
        "kuna_base_options": ["--option", "bejoin", "evidence", "--option", "dwarf", "on" if dwarf else "off"] + (["--option", "narrowext", "compiler"] if dwarf and args.arch == "mips32-be" else []),
        "serial_assertions": None if dwarf else str(out / "assertions.txt"),
        "callwide_ablation": ["off", "on"],
        "refusal_requires_full_off_on_output_equality": args.arch != "mips32-be" or args.case not in {"positive", "xor-sink", "identity-sink", "mixed-refusal"},
        "unchanged_refusal_functions": ["produce_rejected", "caller_rejected", "contract_rejected"] if args.case == "mixed-refusal" else [],
    }
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--verify", nargs=2, metavar=("PROOF", "RUNTIME"))
    parser.add_argument("--verify-selected", nargs=3, metavar=("PROOF", "RUNTIME", "FUNCTION"))
    parser.add_argument("--out")
    parser.add_argument("--arch", choices=ARCH)
    parser.add_argument("--case", choices=CASES, default="positive")
    parser.add_argument("--mode", choices=["declared", "consumer-dwarf"], default="declared")
    parser.add_argument("--cc", default="clang")
    parser.add_argument("--ld", default="ld.lld")
    args = parser.parse_args()
    if args.verify and args.verify_selected:
        parser.error("choose one verification mode")
    if args.verify_selected:
        result = verify_selected(*args.verify_selected)
    elif args.verify:
        result = verify(*args.verify)
    else:
        if not args.out or not args.arch:
            parser.error("emission requires --out and --arch")
        result = emit(args)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
