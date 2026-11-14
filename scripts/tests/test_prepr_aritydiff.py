import hashlib
import io
import tempfile
import unittest
import zipfile
from pathlib import Path

from scripts.prepr import aritydiff, corpus


def record(address, code, error=None, name=None):
    return {address: {"address_hex": address, "name": name or f"sub_{address[2:]}", "code": code, "error": error}}


class CallParsingTests(unittest.TestCase):
    def test_counts_named_calls_and_skips_keywords_and_helpers(self):
        code = "int f(int a0)\n{\n  if (g(a0,1)) return h();\n  v = CONCAT44(x,y);\n  k(m(1),\"a,b\",'(');\n}"
        self.assertEqual(aritydiff.calls(code), [("g", 2), ("h", 0), ("k", 3), ("m", 1)])

    def test_names_a_call_through_a_pointer_and_not_a_cast(self):
        code = "{\n  (*(code *)*v1)(1,2);\n  v = (int *)(a0 + 4);\n  (**(void (**)(int))(*v2 + 8))(a0);\n}"
        self.assertEqual(aritydiff.calls(code), [("(*)", 2), ("(*)", 1)])

    def test_ignores_the_signature_before_the_body(self):
        self.assertEqual(aritydiff.calls("void f(int a0,int a1)\n{\n  g(a0);\n}"), [("g", 1)])


class CompareTests(unittest.TestCase):
    def test_a_lost_argument_blocks(self):
        before = record("0x401000", "{\n  CreateFileA(a,b,c,d,e,f,0);\n}")
        after = record("0x401000", "{\n  CreateFileA();\n}")
        report = aritydiff.compare("x.exe", before, after)
        self.assertTrue(report.blocking)
        self.assertEqual([(c.callee, c.before, c.after) for c in report.lost], [("CreateFileA", 7, 0)])

    def test_a_gained_argument_is_reported_without_blocking(self):
        before = record("0x401000", "{\n  sub_40c991();\n}")
        after = record("0x401000", "{\n  sub_40c991(v11,0);\n}")
        report = aritydiff.compare("x.exe", before, after)
        self.assertFalse(report.blocking)
        self.assertEqual(len(report.gained), 1)

    def test_a_dropped_call_or_a_new_error_blocks(self):
        before = record("0x401000", "{\n  g(1);\n  h(2);\n}") | record("0x402000", "{\n  k();\n}")
        after = record("0x401000", "{\n  g(1);\n}") | record("0x402000", "", error="budget exceeded")
        report = aritydiff.compare("x.exe", before, after)
        self.assertTrue(report.blocking)
        self.assertEqual(report.calls_before, 3)
        self.assertEqual(report.calls_after, 1)
        self.assertEqual(len(report.errored), 1)

    def test_identical_output_is_unchanged(self):
        before = record("0x401000", "{\n  g(1);\n}")
        report = aritydiff.compare("x.exe", before, dict(before))
        self.assertEqual((report.changed, report.blocking), (0, False))


class CorpusTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-prepr-test-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.member = b"MZ fake launcher"
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, "w") as archive:
            archive.writestr("pkg/cli-32.exe", self.member)
        self.archive = self.root / "pkg.whl"
        self.archive.write_bytes(buffer.getvalue())

    def entry(self, member_sha=None):
        return corpus.Entry("cli-32.exe", self.archive.as_uri(),
                            hashlib.sha256(self.archive.read_bytes()).hexdigest(),
                            member_sha or hashlib.sha256(self.member).hexdigest(), "pkg/cli-32.exe")

    def test_extracts_and_caches_a_pinned_member(self):
        entry = self.entry()
        path = corpus.fetch(entry, self.root / "cache")
        self.assertEqual(path.read_bytes(), self.member)
        self.archive.unlink()
        self.assertEqual(corpus.fetch(entry, self.root / "cache"), path)

    def test_refuses_a_member_whose_hash_moved(self):
        with self.assertRaises(ValueError):
            corpus.fetch(self.entry("0" * 64), self.root / "cache")

    def test_every_pinned_entry_has_both_hashes(self):
        for entries in corpus.CORPORA.values():
            for entry in entries:
                self.assertRegex(entry.url_sha256, r"^[0-9a-f]{64}$")
                self.assertRegex(entry.sha256, r"^[0-9a-f]{64}$")
                self.assertTrue(entry.url.startswith("https://"))


if __name__ == "__main__":
    unittest.main()
