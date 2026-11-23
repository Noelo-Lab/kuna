#!/usr/bin/env python3
"""Build and inspect an authored STORE16 witness; never execute an inspected image."""

import argparse
import hashlib
import json
from pathlib import Path
import platform
import re
import shutil
import subprocess

ASSEMBLY = r""".text
.globl frame_copy16
.type frame_copy16,@function
frame_copy16:
    sub $24,%rsp
    movdqu (%rdi),%xmm0
    movdqu %xmm0,(%rsp)
    lea (%rsp),%rdi
    call observe16
    add $24,%rsp
    ret
.size frame_copy16,.-frame_copy16
.globl observe16
.type observe16,@function
observe16:
    mov (%rdi),%rax
    xor 8(%rdi),%rax
    ret
.size observe16,.-observe16
.section .note.GNU-stack,"",@progbits
"""

OBSERVER = """#include <stdint.h>
#include <string.h>
static __attribute__((always_inline)) inline
unsigned long long observe16(unsigned char *p) {
    uint64_t lo, hi;
    memcpy(&lo, p, 8);
    memcpy(&hi, p + 8, 8);
    return lo ^ hi;
}
"""

REFERENCE = """#include <stdint.h>
#include <string.h>
unsigned long long reference_copy16(unsigned char *src) {
    unsigned char bytes[16];
    uint64_t lo, hi;
    memcpy(bytes, src, 16);
    memcpy(&lo, bytes, 8);
    memcpy(&hi, bytes + 8, 8);
    return lo ^ hi;
}
"""


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--kuna", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--cc", default="cc")
    parser.add_argument("--expect-stock", action="store_true")
    args = parser.parse_args()
    if platform.system() != "Linux" or platform.machine() != "x86_64":
        parser.error("the authored fixture uses the Linux AMD64 SysV ABI")
    compiler = shutil.which(args.cc)
    if not compiler:
        parser.error("a GCC-compatible C compiler is required")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    kuna = args.kuna.resolve()
    commands = []

    def run(label, argv):
        argv = [str(value) for value in argv]
        result = subprocess.run(argv, capture_output=True, timeout=90)
        (output / (label + ".stdout")).write_bytes(result.stdout)
        (output / (label + ".stderr")).write_bytes(result.stderr)
        commands.append({"label": label, "argv": argv, "exit": result.returncode})
        if result.returncode:
            raise RuntimeError(f"{label}: {result.stderr.decode(errors='replace')}")
        return result.stdout

    assembly = output / "frame.S"
    assembly.write_text(ASSEMBLY)
    run("assemble", ["as", "--64", assembly, "-o", output / "frame.o"])
    binary = output / "frame.elf"
    run("link", ["ld", "--build-id=none", "-e", "frame_copy16",
                 output / "frame.o", "-o", binary])
    binary.chmod(0o444)
    raw = json.loads(run("raw", [kuna, "raw-pcode", binary, "frame_copy16", "--json"]))
    memory = [op for instruction in raw["instructions"] if instruction["mnemonic"] == "MOVDQU"
              for op in instruction["ops"] if op["opcode"] in ("LOAD", "STORE")]
    assert len(memory) == 2
    assert memory[0]["opcode"] == "LOAD" and memory[0]["output"]["size"] == 16
    assert memory[1]["opcode"] == "STORE" and memory[1]["inputs"][2]["size"] == 16
    consumer = json.loads(run("consumer-raw", [kuna, "raw-pcode", binary, "observe16", "--json"]))
    observed_loads = [op for instruction in consumer["instructions"]
                      if instruction["mnemonic"] in ("MOV", "XOR")
                      for op in instruction["ops"] if op["opcode"] == "LOAD"]
    assert len(observed_loads) == 2 and all(op["output"]["size"] == 8 for op in observed_loads)
    assert not any(op["opcode"] in ("CALL", "CALLIND", "STORE")
                   for instruction in consumer["instructions"] for op in instruction["ops"])
    document = json.loads(run("decompile", [
        kuna, "decompile", binary, "frame_copy16", "--language", "c", "--json",
        "--assert-strict", "--assert",
        "prototype frame_copy16 unsigned long long frame_copy16(unsigned char *src)",
        "--assert", "prototype observe16 unsigned long long observe16(unsigned char *p)",
    ]))
    function = document["functions"][0]
    assert not function.get("error")
    code = function["code"]
    (output / "emitted.c").write_text(code + "\n")
    locals8 = {v["stack_offset"]: v["name"] for v in function["variables"]
               if v["kind"] == "stack" and v["size"] == 8}
    split = (-24 in locals8 and -16 in locals8
             and bool(re.search(r"observe16\([^;]*&" + re.escape(locals8[-24]) + r"\b", code)))
    if args.expect_stock:
        assert split, "stock split-object witness is no longer present; inspect emitted.c"
    emitted = output / "emitted-observer.c"
    emitted.write_text(OBSERVER + code + "\n")
    reference = output / "reference.c"
    reference.write_text(REFERENCE)
    for label, source in [("emitted", emitted), ("reference", reference)]:
        obj = output / (label + ".o")
        run(label + "-compile", [compiler, "-std=gnu11", "-O2", "-Wall", "-c", source, "-o", obj])
        run(label + "-assembly", ["objdump", "-d", obj])
    version = run("compiler-version", [compiler, "--version"]).decode().splitlines()[0]
    receipt = {
        "kuna_sha256": digest(kuna), "fixture_sha256": digest(binary),
        "script_sha256": digest(Path(__file__).resolve()), "compiler": version,
        "original_load_bytes": 16, "original_store_bytes": 16,
        "authored_observer_data_load_bytes": [8, 8],
        "general_consumer_contract_available": False,
        "split_local_objects": split, "commands": commands,
        "source_sha256": {p.name: digest(p) for p in [assembly, emitted, reference]},
        "inspected_image_execution": False, "generated_object_execution": False,
        "native_equivalence_claim": False,
    }
    (output / "proof.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(code)
    print(json.dumps({"split_local_objects": split, "evidence": str(output),
                      "inspected_image_execution": False}))


if __name__ == "__main__":
    main()
