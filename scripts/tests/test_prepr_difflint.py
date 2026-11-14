import contextlib
import io
import subprocess
import tempfile
import unittest
from pathlib import Path

from scripts.prepr import difflint

FIXTURES = Path("decompiler/crates/kuna-analysis/tests/fixtures")
LEAKED = "/" + "home/someone/bin"
STAGE = '<stringmatch name="T #1: x" min="1" max="1">sub_100074\\(2,1\\)</stringmatch>\n'


class DiffLintTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-difflint-test-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.email", "t@example.invalid")
        self.git("config", "user.name", "t")
        self.write(FIXTURES / "README.md", "| `old.exe` | recipe |\n")
        self.write("tests/stages/kuna-old.xml", STAGE)
        self.write("tests/stages/kuna-catalog.xml", STAGE)
        self.commit("base")
        self.git("checkout", "-q", "-b", "feature")

    def git(self, *args):
        return subprocess.run(["git", "-C", str(self.root), *args], check=True, capture_output=True, text=True).stdout

    def write(self, path, content):
        file = self.root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        (file.write_bytes if isinstance(content, bytes) else file.write_text)(content)

    def commit(self, message):
        self.git("add", "-A")
        self.git("commit", "-q", "-m", message)

    def checks(self):
        return sorted((f.level, f.check, Path(f.path).name) for f in difflint.lint(self.root, "main"))

    def test_a_clean_change_passes(self):
        self.write("scripts/tool.py", "print('hi')\n")
        self.commit("tool")
        self.assertEqual(self.checks(), [])

    def test_an_unlisted_fixture_binary_is_an_error(self):
        self.write(FIXTURES / "new.exe", b"MZ\0\0binary")
        self.commit("fixture")
        self.assertEqual(self.checks(), [("error", "fixture-readme", "new.exe")])
        self.write(FIXTURES / "README.md", "| `old.exe` | recipe |\n| `new.exe` | recipe, SHA-256 |\n")
        self.commit("readme")
        self.assertEqual(self.checks(), [])

    def test_a_local_path_in_an_added_binary_or_test_line_is_an_error(self):
        self.write(FIXTURES / "README.md", "| `old.exe` |\n| `a.exe` |\n")
        self.write(FIXTURES / "a.exe", b"MZ\0\0C:\\Users\\me\\build\\a.pdb")
        self.write("tests/stages/kuna-new.xml", f"<com>load file {LEAKED}</com>\n")
        self.commit("leak")
        self.assertEqual(self.checks(), [("error", "local-path", "a.exe"), ("error", "local-path", "kuna-new.xml")])

    def test_an_unreferenced_intermediate_is_an_error(self):
        self.write(FIXTURES / "README.md", "| `old.exe` |\n| `x_c.obj` |\n")
        self.write(FIXTURES / "x_c.obj", b"\x4c\x01\0\0object")
        self.commit("obj")
        self.assertIn(("error", "intermediate", "x_c.obj"), self.checks())
        self.write("tests/stages/kuna-x.xml", "<com>load file fixtures/x_c.obj</com>\n")
        self.commit("used by a stage test")
        self.assertNotIn(("error", "intermediate", "x_c.obj"), self.checks())

    def test_an_edited_assertion_needs_a_trailer(self):
        self.write("tests/stages/kuna-old.xml", STAGE.replace("(2,1\\)", "(2\\)"))
        self.write("tests/stages/kuna-catalog.xml", STAGE.replace('min="1" max="1"', 'min="2" max="2"'))
        self.commit("re-pin")
        self.assertEqual(self.checks(), [("error", "assertion-edit", "kuna-old.xml")])
        self.git("commit", "-q", "--allow-empty", "-m", "why\n\nAssertion-change: kuna-old: the helper reads one argument")
        self.assertEqual(self.checks(), [])

    def test_a_new_module_without_an_option_row_is_a_warning(self):
        self.write("decompiler/crates/kuna-decomp/src/p4_calls/kuna_thing.rs", "fn f() {}\n")
        self.commit("module")
        self.assertEqual(self.checks(), [("warning", "option-policy", "kuna_thing.rs")])
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(difflint.main(["main", "--root", str(self.root)]), 0)


if __name__ == "__main__":
    unittest.main()
