import tempfile
import unittest
from pathlib import Path

from scripts.observatory import walkthrough


class ConsoleScriptTests(unittest.TestCase):
    def test_command_order_and_redirects(self):
        out = Path("/tmp/out")
        script = walkthrough.console_script(Path("/bin/fixture"), "main", out)
        lines = script.splitlines()
        self.assertEqual(lines[0], "load file /bin/fixture")
        self.assertEqual(lines[1], "load function main")
        self.assertEqual(lines[-1], "quit")
        self.assertEqual(script.count("openfile write"), script.count("closefile"))
        decompile_at = lines.index("decompile")
        first_raw = lines.index("print raw")
        self.assertLess(first_raw, decompile_at, "pre-action IR must dump before decompile")
        self.assertLess(decompile_at, len(lines) - lines[::-1].index("print raw") - 1)
        for name in ("03-rawpcode", "04-finalir", "05-regions", "06-c"):
            self.assertIn(f"openfile write {out / (name + '.txt')}", lines)

    def test_stage_table_shape(self):
        ids = [stage[0] for stage in walkthrough.STAGES]
        self.assertEqual(ids, ["bytes", "disasm", "rawpcode", "finalir", "regions", "c"])
        for stage in walkthrough.STAGES:
            self.assertIsInstance(stage[3], list)
            for phase in stage[3]:
                self.assertRegex(phase, r"^P\d$")


class StageAssemblyTests(unittest.TestCase):
    def test_kuna_bin_override(self):
        import os
        from unittest import mock

        with mock.patch.dict(os.environ, {"KUNA_BIN": "/custom/kuna"}):
            self.assertEqual(walkthrough._kuna_bin(Path("/r")), Path("/custom/kuna"))
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertEqual(
                walkthrough._kuna_bin(Path("/r")), Path("/r/decompiler/target/release/kuna")
            )


if __name__ == "__main__":
    unittest.main()
