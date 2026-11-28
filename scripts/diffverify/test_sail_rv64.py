"""Check Sail's input bindings, path coverage, and independent RV64 semantics."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

from sail_rv64 import import_trace, sexps


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = Path(__file__).with_name("sail_rv64.py")
FIXTURE = Path(__file__).with_name("fixtures") / "formal_rv64.s"
PLATFORM = "(read-reg |cur_privilege| nil |Machine|)"
DECLARE = "(declare-const v0 (_ BitVec 64))"
READ = "(read-reg |x10| nil v0)"
WRITE = "(write-reg |x10| nil v0)"
INPUTS = {"|x10|": "kuna_x10"}


class TraceTests(unittest.TestCase):
    def trace(self, *events):
        return f"(trace (cycle) {PLATFORM} {DECLARE} {READ} {' '.join(events)} {WRITE})"

    def test_aliases_are_bound_to_the_common_input(self):
        terms, paths = import_trace(sexps(self.trace()), INPUTS, "|x10|")
        self.assertEqual(paths, [("sail_path_0", "sail_value_0")])
        self.assertIn("(define-fun sail_value_0 () (_ BitVec 64) kuna_x10)", terms)

    def test_unmodeled_effects_branch_conditions_and_extra_inputs_are_rejected(self):
        for event, reason in (
            ("(write-reg |x11| nil v0)", "event"),
            ("(write-mem v1 |k| v0 v0 8)", "event"),
            ('(branch 0 "example location")', "omits a branch condition"),
            ("(declare-const v1 (_ BitVec 64))", "declared input"),
            ("(define-const v2 (unknown v0))", "unsupported Isla expression"),
        ):
            with self.subTest(event=event), self.assertRaisesRegex(ValueError, reason):
                import_trace(sexps(self.trace(event)), INPUTS, "|x10|")

    @unittest.skipUnless(shutil.which("z3"), "Z3 is required for coverage checks")
    def test_a_missing_symbolic_trace_leaves_a_satisfiable_coverage_gap(self):
        half = self.trace('(branch 0 "example location")', "(assert (= v0 #x0000000000000000))")
        other = self.trace('(branch 0 "example location")', "(assert (distinct v0 #x0000000000000000))")
        for traces, expected in ((half, "sat"), (half + other, "unsat")):
            terms, labels = import_trace(sexps(traces), INPUTS, "|x10|")
            guards = " ".join(guard for guard, _ in labels)
            query = (f"(set-logic QF_BV) (declare-const kuna_x10 (_ BitVec 64)) {terms}"
                     f" (assert (not (or {guards} false))) (check-sat)")
            actual = subprocess.check_output(["z3", "-in", "-smt2"], input=query, text=True, timeout=10)
            self.assertEqual(actual.strip(), expected)


class SailIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        env = ("KUNA_ISLA_FOOTPRINT", "KUNA_SAIL_RV64_IR", "KUNA_ISLA_RV64_CONFIG")
        if not all(name in os.environ for name in env):
            raise unittest.SkipTest("configure an Isla executable, Sail snapshot and RV64 config")
        cls.isla, cls.model, cls.config = (os.environ[name] for name in env)
        cls.verify = os.environ.get("KUNA_VERIFY_ORACLE", ROOT / "decompiler/target/release/kuna-verify")
        directory = tempfile.TemporaryDirectory(prefix="kuna-sail-rv64-")
        cls.addClassCleanup(directory.cleanup)
        obj = Path(directory.name) / "rv64.o"
        cls.elf = Path(directory.name) / "rv64.elf"
        subprocess.run([os.environ.get("CLANG", "clang"), "--target=riscv64-linux-gnu",
                        "-march=rv64i", "-mabi=lp64", "-c", FIXTURE, "-o", obj], check=True)
        subprocess.run([os.environ.get("LD_LLD", "ld.lld"), "-static", "-z", "separate-loadable-segments",
                        "-e", "rv_add", obj, "-o", cls.elf], check=True)

    def prove(self, name, *flags):
        command = [sys.executable, SCRIPT, self.elf, name, "--isla", self.isla, "--architecture",
                   self.model, "--config", self.config, "--verify", self.verify, "--timeout-ms", "15000",
                   *flags]
        result = subprocess.run(command, capture_output=True, timeout=20)
        return result.returncode, json.loads(result.stdout)

    def test_composes_independent_and_decompiler_proofs(self):
        for function in ("rv_add", "rv_immediate", "rv_shift", "rv_zero_source"):
            with self.subTest(function=function):
                code, report = self.prove(function)
                self.assertEqual((code, report["classification"]), (0, "MODEL_EQUIVALENCE"), report.get("reason"))
                self.assertEqual([q["result"] for q in report["queries"]], ["sat", "unsat", "unsat", "unsat"])
                if function == "rv_zero_source":
                    self.assertEqual(report["kuna"]["input_registers"], [])
        code, report = self.prove("rv_memory")
        self.assertEqual((code, report["classification"]), (3, "INCONCLUSIVE"), report.get("reason"))
        self.assertEqual(report["phase"], "kuna-encoding")

    def test_the_reduced_register_file_is_outside_the_declared_machine_model(self):
        with tempfile.TemporaryDirectory(prefix="kuna-rv64e-") as directory:
            binary = Path(directory) / "rv64e.elf"
            data = bytearray(self.elf.read_bytes())
            data[48] |= 0x08  # EF_RISCV_RVE in the little-endian ELF64 flags.
            binary.write_bytes(data)
            result = subprocess.run([self.verify, "rv64-terms", binary, "rv_add", "--sleighpath", ROOT / "specs"],
                                    capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 2)
        self.assertIn("standard integer register file", result.stderr)

    def test_ir_counterexample_replays_machine_bytes(self):
        code, report = self.prove("rv_add", "--opcode-mutant", "0")
        self.assertEqual((code, report["classification"]), (1, "COUNTEREXAMPLE"), report.get("reason"))
        self.assertEqual(report["phase"], "raw-vs-final-ir")
        witness = report["witness"]
        self.assertEqual(witness["sail"], witness["raw"])
        self.assertNotEqual(witness["raw"], witness["final_ir"])
        if witness["replay"] != "unicorn-unavailable":
            self.assertEqual(witness["replay"], "confirmed")

    def test_shared_sleigh_mutation_breaks_the_independent_obligation(self):
        compiler = Path(os.environ.get("KUNA_SLACOMP_ORACLE", ROOT / "decompiler/target/release/slacomp"))
        if not compiler.is_file():
            self.skipTest("build slacomp to run the isolated reference mutation")
        language = Path("Ghidra/Processors/RISCV/data/languages")
        with tempfile.TemporaryDirectory(prefix="kuna-rv64-mutation-") as directory:
            specs = Path(directory)
            local = specs / language
            shutil.copytree(ROOT / "specs" / language, local, ignore=shutil.ignore_patterns("*.sla"))
            source = local / "riscv.rv32i.sinc"
            text = source.read_text()
            start = text.index(":add rd,rs1,rs2 ")
            end = text.index("}", start)
            original = text[start:end]
            self.assertEqual(original.count(" + "), 1)
            source.write_text(text[:start] + original.replace(" + ", " - ") + text[end:])
            spec = local / "riscv.lp64d.slaspec"
            subprocess.run([compiler, spec, spec.with_suffix(".sla")], check=True, capture_output=True)
            code, report = self.prove("rv_add", "--sleighpath", specs)
        self.assertEqual((code, report["classification"]), (1, "COUNTEREXAMPLE"), report.get("reason"))
        self.assertEqual(report["phase"], "sail-vs-raw")
        witness = report["witness"]
        self.assertEqual(witness["raw"], witness["final_ir"])
        self.assertNotEqual(witness["sail"], witness["raw"])
        if witness["replay"] != "unicorn-unavailable":
            self.assertEqual(witness["replay"], "confirmed")


if __name__ == "__main__":
    unittest.main()
