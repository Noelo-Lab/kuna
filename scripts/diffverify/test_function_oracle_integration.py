"""Cross-check a linked ELF in Unicorn and both Kuna interpreters."""

import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest

from oracle_compare import Return, compare
from oracle_replay import replay
from proof_replay import replay_proof

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


def elf_load_bytes(elf, address, size):
    data = elf.read_bytes()
    phoff, = struct.unpack_from("<Q", data, 32)
    entsize, count = struct.unpack_from("<HH", data, 54)
    for index in range(count):
        kind, _, offset, start, _, file_size = struct.unpack_from("<IIQQQQ", data, phoff + index * entsize)
        if kind == 1 and start <= address and address + size <= start + file_size:
            offset += address - start
            return data[offset:offset + size]
    raise AssertionError(f"no file-backed ELF bytes at {address:#x}")


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

    def capture(self, function, *options, specs=ROOT / "specs"):
        capture = subprocess.check_output([
            self.verify, "oracle-case", self.elf, function, "--sleighpath", specs, *options,
        ])
        external = json.loads(subprocess.check_output([sys.executable, RUNNER, "-"], input=capture))
        return json.loads(capture), external

    def observe(self, function, *options):
        data, external = self.capture(function, *options)
        return data["case"], external, data["observations"]["raw"], data["observations"]["ir"]

    def prove(self, name, *options, inputs=("RDI", "RSI"), stop=None, whole=False):
        if not shutil.which("z3"):
            self.skipTest("install Z3 to run translation validation")
        if stop is None and not whole:
            symbols = subprocess.check_output([
                os.environ.get("LLVM_NM", "llvm-nm"), "--print-size", "--format=posix", self.elf,
            ], text=True)
            _, _, start, size = next(line.split() for line in symbols.splitlines() if line.split()[0] == name)
            stop = int(start, 16) + int(size, 16) - 1
        command = [
            self.verify, "prove", self.elf, name, "--sleighpath", ROOT / "specs",
            "--return", "RAX:8", *options,
        ]
        command.extend(("--whole-function",) if whole else ("--stop-at", hex(stop)))
        for reg in inputs:
            command.extend(("--input", reg))
        result = subprocess.run(command, capture_output=True, check=False, timeout=70)
        self.assertIn(result.returncode, (0, 1, 3), result.stderr)
        return result.returncode, json.loads(result.stdout)

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

    def test_double_rounding_and_special_values_are_observable_as_bits(self):
        cases = (
            ("round ties to even", 0x3ff0000000000000, 0x3ca0000000000000, 0x3ff0000000000000),
            ("negative zero", 0x8000000000000000, 0x8000000000000000, 0x8000000000000000),
            ("infinity", 0x7ff0000000000000, 0x3ff0000000000000, 0x7ff0000000000000),
            ("quiet NaN payload", 0x7ff8b00000000002, 0x3ff0000000000000, 0x7ff8b00000000002),
            ("signaling NaN quieting", 0x7ff0b00000000002, 0x3ff0000000000000, 0x7ff8b00000000002),
            ("opposite infinities", 0x7ff0000000000000, 0xfff0000000000000, 0xfff8000000000000),
        )
        for label, a, b, expect in cases:
            with self.subTest(label=label):
                _, oracle, raw, ir = self.observe(
                    "add_double", "--input-reg", "XMM0", hex(a), "--input-reg", "XMM1", hex(b),
                )
                self.assert_returned(oracle, raw, ir, expect, "XMM0")

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

    def test_declared_external_boundary_checks_the_target_and_argument_prefix(self):
        original, _ = self.capture("call_inner")
        target = original["observations"]["raw"]["calls"][0]["target"]
        capture, oracle = self.capture(
            "call_inner", "--external-entry", target, "1", "--input-reg", "RDI", "0x29",
        )
        raw, ir = capture["observations"]["raw"], capture["observations"]["ir"]
        self.assertEqual(oracle["event"]["target"], target)
        self.assertEqual(oracle["event"]["registers"]["RDI"], "0x29")
        self.assertEqual(ir["calls"][0]["arguments"], ["0x29"])
        self.assertFalse(raw["calls"][0]["concrete_effect"])
        self.assertEqual(oracle["memory"], raw["memory"])
        self.assertEqual(oracle["memory"], ir["memory"])
        expected = Return.parse("RAX:8")
        report = compare(capture, oracle, expected)
        self.assertEqual(report["classification"], "INCONCLUSIVE")
        self.assertEqual(report["coverage"]["boundary"], "external-call-prefix")
        self.assertEqual(replay(capture, self.elf, ROOT / "specs", self.verify, expected), report)

        ir["calls"][0]["arguments"] = ["0x2a"]
        report = compare(capture, oracle, expected)
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual(report["findings"][0]["observable"], "external-call-argument")
        ir["calls"][0]["arguments"] = []
        report = compare(capture, oracle, expected)
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual(report["findings"][0]["observable"], "external-call-arity")

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

    def test_three_way_agreement_is_scoped_to_the_case(self):
        capture, oracle = self.capture("add_double")
        report = compare(capture, oracle, Return.parse("XMM0:8"))
        self.assertEqual(report["classification"], "CROSS_BACKEND_AGREEMENT")
        self.assertTrue(report["coverage"]["full_return_comparison"])
        self.assertEqual(report["coverage"]["return"], "XMM0:8")
        self.assertEqual(report["case_sha256"], oracle["case_sha256"])
        self.assertEqual(report["provenance"]["elf_sha256"], hashlib.sha256(self.elf.read_bytes()).hexdigest())
        self.assertRegex(report["provenance"]["sla_sha256"], r"^[0-9a-f]{64}$")

    def test_wrong_case_cannot_be_paired_with_an_observation(self):
        capture, oracle = self.capture("add")
        capture["case"]["registers"]["RDI"] = "0x1"
        with self.assertRaisesRegex(ValueError, "same case"):
            compare(capture, oracle, Return.parse("RAX:8"))

    def test_replay_regenerates_the_case_and_all_three_observations(self):
        capture, oracle = self.capture("pointer_update", "--input-reg", "RDI", "0x60000100", "--input-reg", "RSI", "0x13")
        expected = Return.parse("RAX:8")
        report = replay(capture, self.elf, ROOT / "specs", self.verify, expected)
        self.assertEqual(report, compare(capture, oracle, expected))
        capture["observations"]["ir"]["return"]["value"] = "0x14"
        with self.assertRaisesRegex(ValueError, "observations differ"):
            replay(capture, self.elf, ROOT / "specs", self.verify, expected)

    def test_memory_difference_survives_matching_returns(self):
        capture, oracle = self.capture("caller_write")
        report = compare(capture, oracle, Return.parse("RAX:8"))
        actual = capture["observations"]["ir"]["memory"]
        if actual != oracle["memory"]:
            self.assertEqual(report["classification"], "COUNTEREXAMPLE")
            finding = report["findings"][0]
            self.assertEqual(finding["observable"], "memory-byte")
            self.assertIn("decompiler-transformation", finding["candidate_causes"])
        else:
            self.assertEqual(report["classification"], "CROSS_BACKEND_AGREEMENT")

    def test_shared_wrong_observation_still_disagrees_with_unicorn(self):
        capture, oracle = self.capture("add")
        raw, ir = capture["observations"]["raw"], capture["observations"]["ir"]
        wrong = hex(value(oracle, "RAX") ^ 1)
        raw["registers"]["RAX"] = ir["return"]["value"] = wrong
        report = compare(capture, oracle, Return.parse("RAX:8"))
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual(report["findings"][0]["observable"], "return")
        self.assertIn("shared-lift-or-pcode-semantics", report["findings"][0]["candidate_causes"])

    def test_shared_lift_fault_is_detected_with_the_original_executable(self):
        options = ("--input-reg", "RDI", "0x5", "--input-reg", "RSI", "0x1")
        reference, _ = self.capture("store_loop", *options)
        slacomp = Path(os.environ.get("KUNA_SLACOMP_ORACLE", ROOT / "decompiler/target/release/slacomp"))
        if not slacomp.is_file():
            self.skipTest("build slacomp to run the isolated SLEIGH fault injection")
        languages = Path("Ghidra/Processors/x86/data/languages")
        with tempfile.TemporaryDirectory(prefix="kuna-shared-lift-") as temp:
            specs = Path(temp)
            target = specs / languages
            shutil.copytree(ROOT / "specs" / languages, target, ignore=shutil.ignore_patterns("*.sla"))
            source = target / "ia.sinc"
            lines = source.read_text().splitlines(keepends=True)
            changed = 0
            for i, line in enumerate(lines):
                if line.startswith(":ADD Rmr64,Reg64 "):
                    self.assertEqual(line.count(" + "), 1)
                    lines[i] = line.replace(" + ", " - ")
                    changed += 1
            self.assertEqual(changed, 1)
            source.write_text("".join(lines))
            subprocess.run([slacomp, target / "x86-64.slaspec", target / "x86-64.sla"], check=True, capture_output=True)
            capture, oracle = self.capture("store_loop", *options, specs=specs)
        self.assertEqual(capture["case"], reference["case"])
        self.assertEqual(capture["provenance"]["elf_sha256"], reference["provenance"]["elf_sha256"])
        self.assertNotEqual(capture["provenance"]["sla_sha256"], reference["provenance"]["sla_sha256"])
        raw = capture["observations"]["raw"]
        ir = capture["observations"]["ir"]
        self.assertEqual(value(oracle, "RAX"), (3 * INITIAL_COUNTER + 5) & MASK64)
        self.assertEqual(value(raw, "RAX"), (3 * INITIAL_COUNTER - 5) & MASK64)
        self.assertEqual(value(raw, "RAX"), int(ir["return"]["value"], 16))
        self.assertEqual(raw["memory"], ir["memory"])
        report = compare(capture, oracle, Return.parse("RAX:8"))
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual({f["observable"] for f in report["findings"]}, {"return", "memory-byte"})
        for finding in report["findings"]:
            self.assertIn("shared-lift-or-pcode-semantics", finding["candidate_causes"])

    def test_incomplete_syscall_does_not_hide_a_prefix_difference(self):
        capture, oracle = self.capture("system_boundary")
        expected = Return.parse("RAX:8")
        baseline = compare(capture, oracle, expected)
        self.assertEqual(baseline["classification"], "INCONCLUSIVE")
        self.assertEqual(baseline["coverage"]["boundary"], "syscall-prefix")
        capture["observations"]["ir"]["events"][0]["inputs"] = ["0x2a"]
        report = compare(capture, oracle, expected)
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual(report["findings"][0]["observable"], "syscall-number")

    def test_matching_fault_or_abi_violation_does_not_count_as_agreement(self):
        for function in ("unmapped_read", "clobber_rbx"):
            with self.subTest(function=function):
                capture, oracle = self.capture(function)
                report = compare(capture, oracle, Return.parse("RAX:8"))
                self.assertEqual(report["classification"], "INCONCLUSIVE")
                self.assertEqual(report["findings"], [])

    def test_preserved_values_remain_distinguishable_if_both_violate_the_abi(self):
        capture, oracle = self.capture("clobber_rbx")
        capture["observations"]["raw"]["registers"]["RBX"] = hex(value(oracle, "RBX") + 1)
        report = compare(capture, oracle, Return.parse("RAX:8"))
        self.assertEqual(report["classification"], "COUNTEREXAMPLE")
        self.assertEqual(report["findings"][0]["observable"], "abi-preserved-register")
        self.assertEqual(report["findings"][0]["register"], "RBX")

    def test_mismatches_without_the_oracle_are_retained_as_inconclusive(self):
        capture, oracle = self.capture("add")
        oracle["status"] = "incomplete"
        capture["observations"]["ir"]["return"]["value"] = hex(value(oracle, "RAX") ^ 1)
        report = compare(capture, oracle, Return.parse("RAX:8"))
        self.assertEqual(report["classification"], "INCONCLUSIVE")
        self.assertEqual(report["findings"][0]["observable"], "return")
        self.assertEqual(set(report["findings"][0]["values"]), {"raw-pcode", "final-ir"})

    def test_symbolic_leaf_equivalence_archives_the_exact_smt_obligation(self):
        code, proof = self.prove("add")
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertEqual(proof["classification"], "MODEL_EQUIVALENCE")
        for kind, expected in (("domain", b"sat"), ("difference", b"unsat")):
            smt = proof["obligation"][f"{kind}_smt2"].encode()
            self.assertEqual(proof["obligation"][f"{kind}_sha256"], hashlib.sha256(smt).hexdigest())
            answer = subprocess.check_output(["z3", "-in", "-smt2"], input=smt, timeout=10)
            self.assertEqual(answer.splitlines()[0], expected)

    def test_symbolic_integer_widths_bit_ops_rotates_and_conditional_moves(self):
        for name, inputs in (
            ("extend_signed", ("RDI",)), ("extend_unsigned", ("RDI",)),
            ("bit_ops", ("RDI", "RSI")), ("rotate", ("RDI",)),
            ("select_cmov", ("RDI", "RSI", "RDX")),
        ):
            with self.subTest(name=name):
                code, proof = self.prove(name, inputs=inputs)
                self.assertEqual(code, 0, proof.get("reason"))
                self.assertEqual(proof["classification"], "MODEL_EQUIVALENCE")

    def test_symbolic_signed_extension_mutation_is_visible_at_full_width(self):
        code, proof = self.prove(
            "extend_signed", "--opcode-mutant", "0", "--bind", "RDI", "0x80000000", inputs=("RDI",),
        )
        self.assertEqual(code, 1, proof.get("reason"))
        replayed = replay_proof(proof, self.elf, ROOT / "specs", self.verify)
        self.assertEqual(replayed["independent_replay"], "confirmed")
        values = replayed["observation_report"]["findings"][0]["values"]
        self.assertEqual(values["unicorn"], "0xffffffff80000000")
        self.assertEqual(values["final-ir"], "0x80000000")

    def test_symbolic_zero_extension_mutation_is_visible_at_full_width(self):
        code, proof = self.prove(
            "extend_unsigned", "--opcode-mutant", "0", "--bind", "RDI", "0x80000000", inputs=("RDI",),
        )
        self.assertEqual(code, 1, proof.get("reason"))
        replayed = replay_proof(proof, self.elf, ROOT / "specs", self.verify)
        self.assertEqual(replayed["independent_replay"], "confirmed")
        values = replayed["observation_report"]["findings"][0]["values"]
        self.assertEqual(values["unicorn"], "0x80000000")
        self.assertEqual(values["final-ir"], "0xffffffff80000000")

    def test_symbolic_partial_register_carry_and_overflow_match_external_execution(self):
        cases = (
            ("partial_register", 0x123456789abcde00, 0xff, 0x123456789abcdeff),
            ("add_carry", 0xffffffffffffffff, 1, 1),
            ("signed_overflow", 0x7fffffffffffffff, 1, 0x8000000000000001),
        )
        for name, a, b, expected in cases:
            with self.subTest(function=name):
                code, proof = self.prove(name, whole=True)
                self.assertEqual((code, proof["classification"]), (0, "MODEL_EQUIVALENCE"), proof.get("reason"))
                _, oracle, raw, ir = self.observe(name, "--input-reg", "RDI", hex(a), "--input-reg", "RSI", hex(b))
                self.assert_returned(oracle, raw, ir, expected)

    def test_symbolic_mutation_gets_an_independently_reproduced_witness(self):
        code, proof = self.prove("add", "--opcode-mutant", "0")
        self.assertEqual(code, 1, proof.get("reason"))
        self.assertEqual(proof["classification"], "COUNTEREXAMPLE")
        self.assertEqual(proof["witness"]["independent_replay"], "pending")
        replayed = replay_proof(proof, self.elf, ROOT / "specs", self.verify)
        self.assertEqual(replayed["independent_replay"], "confirmed")
        mismatch = replayed["observation_report"]["findings"][0]
        self.assertEqual(mismatch["observable"], "return")
        self.assertEqual(mismatch["values"]["unicorn"], mismatch["values"]["raw-pcode"])
        self.assertNotEqual(mismatch["values"]["raw-pcode"], mismatch["values"]["final-ir"])
        proof["obligation"]["difference_smt2"] += "(assert false)\n"
        with self.assertRaisesRegex(ValueError, "query digest differs"):
            replay_proof(proof, self.elf, ROOT / "specs", self.verify)

    def test_symbolic_unsatisfiable_domain_is_inconclusive(self):
        code, proof = self.prove("add", "--bind", "RDI", "0", "--bind", "RDI", "1")
        self.assertEqual(code, 3)
        self.assertEqual(proof["classification"], "INCONCLUSIVE")
        self.assertEqual(proof["reason"], "unsatisfiable-precondition")
        self.assertNotIn("witness", proof)

    def test_symbolic_memory_and_control_flow_cannot_be_certified_as_a_leaf(self):
        for name in ("caller_write", "store_loop"):
            with self.subTest(name=name):
                code, proof = self.prove(name)
                self.assertEqual(code, 3)
                self.assertEqual(proof["classification"], "INCONCLUSIVE")
                self.assertEqual(proof["phase"], "encoding")
                self.assertNotIn("obligation", proof)

    def test_symbolic_memory_proves_in_range_aliasing_and_checks_the_entire_domain(self):
        options = ("--memory", "0x60000100:16", "--range", "RDI", "0x60000100")
        code, proof = self.prove("pointer_update", *options, "0x60000108")
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertEqual(proof["classification"], "MODEL_EQUIVALENCE")
        coverage = proof["obligation"]["coverage_smt2"].encode()
        self.assertEqual(hashlib.sha256(coverage).hexdigest(), proof["obligation"]["coverage_sha256"])

        code, proof = self.prove("pointer_update", *options, "0x60000109")
        self.assertEqual(code, 3)
        self.assertEqual(proof["classification"], "INCONCLUSIVE")
        self.assertEqual(proof["phase"], "execution-coverage")

    def test_symbolic_caller_memory_counterexample_replays_initial_bytes(self):
        code, proof = self.prove("caller_write", "--memory", "0x70003000:8", inputs=("RDI",))
        self.assertEqual(code, 1, proof.get("reason"))
        self.assertEqual(proof["classification"], "COUNTEREXAMPLE")
        self.assertEqual(len(proof["witness"]["memory"]), 8)
        report = replay_proof(proof, self.elf, ROOT / "specs", self.verify)
        self.assertEqual(report["independent_replay"], "confirmed")
        mismatch = report["observation_report"]["findings"][0]
        self.assertEqual(mismatch["observable"], "memory-byte")
        self.assertEqual(mismatch["values"]["unicorn"], mismatch["values"]["raw-pcode"])
        self.assertNotEqual(mismatch["values"]["unicorn"], mismatch["values"]["final-ir"])

    def test_whole_function_joins_direct_branches_and_conditional_memory(self):
        code, proof = self.prove("select_branch", inputs=("RDI", "RSI", "RDX"), whole=True)
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertGreater(proof["scope"]["ir_blocks"], 1)
        code, proof = self.prove(
            "store_if", "--memory", "0x60000100:16", "--range", "RDI", "0x60000100", "0x60000108",
            whole=True,
        )
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertEqual(proof["scope"]["control_flow"], "acyclic-function")
        self.assertIn("coverage_smt2", proof["obligation"])

    def test_whole_function_tracks_private_stack_bytes_and_the_return_slot(self):
        code, proof = self.prove("frame_add", "--private-memory", "0x70002fb0:72",
                                 inputs=("RDI", "RSI", "RBP"), whole=True)
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertEqual(proof["scope"]["stack_entry"], "0x70002ff8")

    def test_whole_function_rejects_a_loop(self):
        code, proof = self.prove("count_down", inputs=("RDI",), whole=True)
        self.assertEqual(code, 3)
        self.assertEqual(proof["classification"], "INCONCLUSIVE")
        self.assertIn("raw loop", proof["reason"])

    def test_unrolling_distinguishes_bounded_total_and_empty_domains(self):
        options = ("--loop-bound", "4")
        code, proof = self.prove("count_down", *options, inputs=("RDI",), whole=True)
        self.assertEqual((code, proof["classification"]), (0, "BOUNDED_EQUIVALENCE"))
        self.assertEqual(proof["scope"]["max_visits_per_raw_instruction_and_ir_block"], 4)
        self.assertGreater(proof["scope"]["ir_phis"], 0)
        for query, status in (("completion", "sat"), ("bounded_domain", "sat"), ("difference", "unsat")):
            obligation = proof["obligation"]
            text = obligation[f"{query}_smt2"]
            self.assertEqual(hashlib.sha256(text.encode()).hexdigest(), obligation[f"{query}_sha256"])
            output = subprocess.check_output(["z3", "-in", "-smt2"], input=text, text=True, timeout=10)
            self.assertEqual(output.splitlines()[0], status)

        for lower, upper, expected in ((1, 4, (0, "MODEL_EQUIVALENCE")), (5, 10, (3, "INCONCLUSIVE"))):
            code, proof = self.prove(
                "count_down", *options, "--range", "RDI", str(lower), str(upper),
                inputs=("RDI",), whole=True,
            )
            self.assertEqual((code, proof["classification"]), expected)
        self.assertEqual(proof["phase"], "bounded-domain")

        code, proof = self.prove("count_down", *options, "--budget", "5", inputs=("RDI",), whole=True)
        self.assertEqual(code, 3)
        self.assertIn("operation budget exhausted", proof["reason"])

    def test_unrolling_merges_memory_and_keeps_missing_coverage_inconclusive(self):
        rows = subprocess.check_output([
            os.environ.get("LLVM_NM", "llvm-nm"), "--format=posix", self.elf,
        ], text=True).splitlines()
        symbols = {name: address for name, _, address, *_ in (row.split() for row in rows)}
        counter = f'0x{symbols["counter"]}:8'
        zeroed = f'0x{symbols["zeroed"]}:8'
        options = ("--loop-bound", "2", "--memory", counter)
        code, proof = self.prove("store_loop", *options, "--memory", zeroed, whole=True)
        self.assertEqual((code, proof["classification"]), (0, "BOUNDED_EQUIVALENCE"), proof.get("reason"))
        code, proof = self.prove("store_loop", *options, whole=True)
        self.assertEqual(code, 3)
        self.assertEqual(proof["phase"], "execution-coverage")

    def test_unrolling_preserves_a_replayable_counterexample(self):
        code, proof = self.prove(
            "add_with_loop", "--loop-bound", "4", "--opcode-mutant", "0",
            inputs=("RDI", "RSI", "RDX"), whole=True,
        )
        self.assertEqual((code, proof["classification"]), (1, "COUNTEREXAMPLE"), proof.get("reason"))
        self.assertIn(int(proof["witness"]["registers"]["RDX"], 16), range(1, 5))
        self.assertEqual(replay_proof(proof, self.elf, ROOT / "specs", self.verify)["independent_replay"], "confirmed")
        proof["obligation"]["completion_smt2"] += "(assert false)\n"
        with self.assertRaisesRegex(ValueError, "completion query digest differs"):
            replay_proof(proof, self.elf, ROOT / "specs", self.verify)

    def test_induction_proves_a_wrapping_64_bit_loop_without_unrolling(self):
        code, proof = self.prove("count_down", "--induct", inputs=("RDI",), whole=True)
        self.assertEqual((code, proof["classification"]), (0, "MODEL_EQUIVALENCE"), proof.get("reason"))
        self.assertIsNone(proof["scope"]["max_visits_per_raw_instruction_and_ir_block"])
        self.assertEqual(proof["induction"]["ranking"], {"carried_index": 0, "subtract": 1})
        queries = proof["induction"]["queries"]
        self.assertEqual([query["result"] for query in queries], ["sat", "unsat", "sat", "unsat", "sat", "unsat"])
        for query in queries:
            text = query["smt2"]
            self.assertEqual(hashlib.sha256(text.encode()).hexdigest(), query["sha256"])
            output = subprocess.check_output(["z3", "-in", "-smt2"], input=text, text=True, timeout=10)
            self.assertEqual(output.splitlines()[0], query["result"])

    def test_induction_relates_two_carried_slots_and_refuses_a_bad_prefix(self):
        options = ("--induct",)
        inputs = ("RDI", "RSI", "RDX")
        code, proof = self.prove("add_with_loop", *options, inputs=inputs, whole=True)
        self.assertEqual((code, proof["classification"]), (0, "MODEL_EQUIVALENCE"), proof.get("reason"))
        self.assertEqual(len(proof["induction"]["cutpoint"]["carried"]), 2)
        code, proof = self.prove("add_with_loop", *options, "--opcode-mutant", "0", inputs=inputs, whole=True)
        self.assertEqual((code, proof["classification"]), (3, "INCONCLUSIVE"), proof.get("reason"))
        self.assertEqual(proof["phase"], "loop-initiation-and-prefix-exits")
        self.assertNotIn("witness", proof)

    def test_induction_relates_different_loop_control_flow_graphs(self):
        name = "loop_with_duplicate_paths"
        instructions = subprocess.check_output([
            os.environ.get("LLVM_OBJDUMP", "llvm-objdump"), f"--disassemble-symbols={name}", self.elf,
        ], text=True)
        self.assertEqual(len(re.findall(r"\b(?:je|jne|jz|jnz)\b", instructions)), 2)
        code, proof = self.prove(name, "--induct", inputs=("RDI", "RSI", "RDX"), whole=True)
        self.assertEqual((code, proof["classification"]), (0, "MODEL_EQUIVALENCE"), proof.get("reason"))
        self.assertEqual(proof["scope"]["ir_blocks"], 3)
        for flag in ("0x0", "0x1"):
            _, oracle, raw, ir = self.observe(
                name, "--input-reg", "RDI", "0x5", "--input-reg", "RSI", "0x3", "--input-reg", "RDX", flag,
            )
            self.assert_returned(oracle, raw, ir, 14)

    def test_induction_cannot_claim_termination_for_an_odd_stride_domain(self):
        code, proof = self.prove("cannot_rank_all_inputs", "--induct", inputs=("RDI",), whole=True)
        self.assertEqual((code, proof["classification"]), (3, "INCONCLUSIVE"), proof.get("reason"))
        self.assertEqual(proof["phase"], "rank-decrease")
        self.assertNotIn("ranking", proof["induction"])

    def test_whole_function_switch_requires_coverage_of_declared_indirect_targets(self):
        text = subprocess.check_output([
            os.environ.get("LLVM_OBJDUMP", "llvm-objdump"), "--disassemble-symbols=dispatch", self.elf,
        ], text=True)
        indirect = re.search(r"(?m)^\s+([0-9a-f]+):[^\n]*\bjmpq\s+\*(0x[0-9a-f]+)\(", text)
        self.assertIsNotNone(indirect, text)
        jump, table = (int(v, 16) for v in indirect.groups())
        count = 6
        size = count * 8
        targets = struct.unpack(f"<{count}Q", elf_load_bytes(self.elf, table, size))
        options = ("--image-memory", f"{table:#x}:{size}", "--timeout-ms", "30000")
        entries = [arg for target in targets for arg in ("--indirect-target", f"{jump:#x}:{target:#x}")]
        code, proof = self.prove("dispatch", *options, *entries, whole=True)
        self.assertEqual(code, 0, proof.get("reason"))
        self.assertEqual(proof["scope"]["raw_indirect_targets"][hex(jump)], list(map(hex, targets)))
        code, proof = self.prove("dispatch", *options, *entries[:-2], whole=True)
        self.assertEqual(code, 3, proof.get("reason"))
        self.assertEqual(proof["phase"], "execution-coverage")


if __name__ == "__main__":
    unittest.main()
