"""Cross-check a linked ELF in Unicorn and both Kuna interpreters."""

import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = Path(__file__).with_name("fixtures") / "function_oracle_x86_64.c"
RUNNER = Path(__file__).with_name("function_oracle.py")
MASK64 = (1 << 64) - 1
INITIAL_COUNTER = 0x123456789ABCDEF0


def value(observation, register):
    return int(observation["registers"][register], 16)


def memory_at(observation, address):
    for region in observation["memory"]:
        base = int(region["address"], 16)
        if base <= address < base + len(region["bytes"]) // 2:
            return bytes.fromhex(region["bytes"])[address - base:]
    raise AssertionError(f"no observed memory at {address:#x}")


class CrossBackendTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temp = tempfile.TemporaryDirectory(prefix="kuna-function-oracle-")
        cls.addClassCleanup(temp.cleanup)
        object_file = Path(temp.name) / "fixture.o"
        cls.elf = Path(temp.name) / "fixture.elf"
        cls.verify = os.environ.get("KUNA_VERIFY_ORACLE", ROOT / "decompiler/target/release/kuna-verify")
        subprocess.run([
            os.environ.get("CLANG", "clang"), "--target=x86_64-linux-gnu", "-O2", "-g",
            "-ffreestanding", "-fno-pic", "-fno-stack-protector", "-c", FIXTURE, "-o", object_file,
        ], check=True)
        subprocess.run([
            os.environ.get("LD_LLD", "ld.lld"), "-static", "-z", "separate-loadable-segments",
            "-e", "add", object_file, "-o", cls.elf,
        ], check=True)

    def observe(self, function, *options):
        capture = subprocess.check_output([
            self.verify, "oracle-case", self.elf, function, "--sleighpath", ROOT / "specs", *options,
        ])
        external = json.loads(subprocess.check_output([sys.executable, RUNNER, "-"], input=capture))
        data = json.loads(capture)
        return data["case"], external, data["observations"]["raw"], data["observations"]["ir"]

    def assert_returned(self, oracle, raw, ir, expected, register="RAX"):
        self.assertEqual([p["status"] for p in (oracle, raw, ir)], ["returned"] * 3)
        mask = (1 << (8 * ir["return"]["size"])) - 1
        self.assertEqual(value(oracle, register) & mask, expected)
        self.assertEqual(value(raw, register) & mask, expected)
        self.assertEqual(int(ir["return"]["value"], 16), expected)

    def test_full_width_integer_backend_agreement(self):
        for seed in ("0x6b756e61", "0x41228ff9e9c7b7a5"):
            with self.subTest(seed=seed):
                case, oracle, raw, ir = self.observe("add", "--seed", seed)
                expected = (value(case, "RDI") + value(case, "RSI")) & MASK64
                self.assertEqual(ir["return"]["size"], 8)
                self.assert_returned(oracle, raw, ir, expected)
                self.assertEqual(oracle["abi_violations"], [])
                self.assertEqual(raw["abi_violations"], [])

    def test_double_return_projection(self):
        case, oracle, raw, ir = self.observe("add_double")
        left, right = [struct.unpack("<d", (value(case, r) & MASK64).to_bytes(8, "little"))[0] for r in ("XMM0", "XMM1")]
        expected = int.from_bytes(struct.pack("<d", left + right), "little")
        self.assertEqual(ir["return"]["size"], 8)
        self.assert_returned(oracle, raw, ir, expected, "XMM0")

    def test_bounded_loop_and_observable_global_writes(self):
        case, oracle, raw, ir = self.observe("store_loop")
        count = value(case, "RSI") & 7
        self.assertGreater(count, 1)
        expected = INITIAL_COUNTER
        for _ in range(count):
            expected = (3 * expected + value(case, "RDI")) & MASK64
        self.assert_returned(oracle, raw, ir, expected)
        global_region = next(r for r in case["memory"] if r["bytes"].startswith("f0debc9a78563412"))
        self.assertEqual(oracle["memory"], raw["memory"])
        self.assertEqual(raw["memory"], ir["memory"])
        self.assertEqual(memory_at(raw, int(global_region["address"], 16))[:16], struct.pack("<QQ", expected, expected))
        _, _, limited, _ = self.observe("store_loop", "--budget", "10")
        self.assertEqual(limited["status"], "incomplete")
        self.assertEqual(limited["reason"], "UNKNOWN(budget)")

    def test_preserved_register_violation(self):
        case, oracle, raw, ir = self.observe("clobber_rbx")
        self.assert_returned(oracle, raw, ir, (value(case, "RDI") + 1) & MASK64)
        self.assertEqual(oracle["abi_violations"], ["RBX"])
        self.assertEqual(raw["abi_violations"], ["RBX"])

    def test_heap_pointer_argument_uses_the_explicit_mapping(self):
        address = 0x6000_0100
        case, oracle, raw, ir = self.observe(
            "pointer_update", "--input-reg", "RDI", hex(address), "--input-reg", "RSI", "0x13",
        )
        self.assertEqual(value(case, "RDI"), address)
        self.assert_returned(oracle, raw, ir, 0x13)
        for observation in (oracle, raw, ir):
            self.assertEqual(memory_at(observation, address)[:8], struct.pack("<Q", 0x13))

    def test_in_image_call_exposes_dependent_kuna_observations(self):
        case, oracle, raw, ir = self.observe("call_inner")
        expected = (INITIAL_COUNTER + value(case, "RDI") + 1) & MASK64
        self.assertEqual(oracle["status"], "returned")
        self.assertEqual([p["status"] for p in (raw, ir)], ["incomplete", "incomplete"])
        self.assertEqual(value(oracle, "RAX"), expected)
        self.assertEqual(value(raw, "RAX"), expected)
        self.assertEqual(int(ir["return"]["value"], 16), expected)
        self.assertEqual(len(raw["calls"]), 1)
        self.assertEqual(len(ir["calls"]), 1)
        self.assertEqual(raw["calls"][0]["target"], ir["calls"][0]["target"])
        self.assertEqual(ir["calls"][0]["arguments"], [case["registers"]["RDI"]])
        self.assertTrue(raw["calls"][0]["concrete_effect"])
        self.assertTrue(ir["raw_callee_effects_available"])
        self.assertEqual(oracle["memory"], raw["memory"])
        self.assertEqual(raw["memory"], ir["memory"])

    def test_system_boundary_keeps_prefix_and_stops_before_effect(self):
        case, oracle, raw, ir = self.observe("system_boundary")
        self.assertEqual([p["status"] for p in (oracle, raw, ir)], ["incomplete"] * 3)
        self.assertEqual(oracle["reason"], "system-call")
        event = oracle["event"]
        self.assertEqual(value(event, "RAX"), 39)
        for register in ("RAX", "RDI", "RSI", "RDX", "R10", "R8", "R9"):
            self.assertEqual(value(event, register), value(raw, register))
        self.assertEqual(raw["events"][0]["address"], event["address"])
        self.assertEqual(ir["events"][0]["inputs"], ["0x27"])
        self.assertIsNone(ir["return"])
        global_region = next(r for r in case["memory"] if r["bytes"].startswith("f0debc9a78563412"))
        self.assertEqual(oracle["memory"], raw["memory"])
        self.assertEqual(raw["memory"], ir["memory"])
        self.assertEqual(memory_at(raw, int(global_region["address"], 16))[:8], struct.pack("<Q", 7))

    def test_unmapped_memory_is_not_synthesized(self):
        _, oracle, raw, ir = self.observe("unmapped_read")
        self.assertEqual(oracle["status"], "fault")
        self.assertEqual(oracle["event"]["address"], "0x0")
        for p in (raw, ir):
            self.assertEqual(p["status"], "trap")
            self.assertIn("explicit memory access (r) at 0x0", p["reason"])

    def test_caller_stack_write_is_present_in_physical_observations(self):
        case, oracle, raw, ir = self.observe("caller_write")
        arg = value(case, "RDI")
        self.assert_returned(oracle, raw, ir, arg)
        at = value(case, "RSP") + 8
        expected = arg.to_bytes(8, "little")
        self.assertEqual(memory_at(oracle, at)[:8], expected)
        self.assertEqual(memory_at(raw, at)[:8], expected)
        # Final IR may omit this caller-owned store. Its bytes must remain
        # independently observable so comparison can surface that discrepancy.
        self.assertIn(memory_at(ir, at)[:8], (bytes(8), expected))


if __name__ == "__main__":
    unittest.main()
