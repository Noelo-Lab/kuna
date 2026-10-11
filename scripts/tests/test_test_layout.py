import tempfile
import unittest
from pathlib import Path

from scripts import check_test_layout
from scripts.check_test_layout import check


class TestIntegrationLayout(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.crate = self.root / "decompiler/crates/example"
        self.tests = self.crate / "tests"
        self.tests.mkdir(parents=True)
        (self.crate / "Cargo.toml").write_text(
            '[package]\nname = "example"\nautotests = false\n'
            '[[test]]\nname = "integration"\npath = "tests/integration.rs"\n'
            '[[test]]\nname = "isolated"\npath = "tests/isolated.rs"\n'
        )
        (self.tests / "integration.rs").write_text('#[path = "grouped.rs"]\nmod grouped;\n')
        (self.tests / "grouped.rs").write_text("#[test]\nfn example() {}\n")
        (self.tests / "isolated.rs").write_text("#[test]\nfn example() {}\n")

    def test_grouped_and_isolated_tests_are_registered(self):
        self.assertEqual(check(self.root), [])

    def test_new_file_must_be_registered(self):
        (self.tests / "forgotten.rs").write_text("#[test]\nfn forgotten() {}\n")
        errors = check(self.root)
        self.assertEqual(len(errors), 1)
        self.assertIn("forgotten.rs: expected one registration, found 0", errors[0])

    def test_duplicate_registration_is_rejected(self):
        with (self.tests / "integration.rs").open("a") as stream:
            stream.write('#[path = "isolated.rs"]\nmod isolated;\n')
        self.assertTrue(any("isolated.rs: expected one registration, found 2" in e for e in check(self.root)))

    def test_missing_module_is_rejected(self):
        (self.tests / "grouped.rs").unlink()
        self.assertTrue(any("grouped.rs: missing suite module" in e for e in check(self.root)))

    def test_missing_entry_point_is_rejected(self):
        (self.tests / "integration.rs").unlink()
        self.assertTrue(any("integration.rs: missing test entry point" in e for e in check(self.root)))

    def test_auto_discovered_crates_are_unchanged(self):
        (self.crate / "Cargo.toml").write_text('[package]\nname = "example"\n')
        self.assertEqual(check(self.root), [])

    def test_commented_registration_does_not_count(self):
        (self.tests / "integration.rs").write_text('// #[path = "grouped.rs"]\n// mod grouped;\n')
        self.assertTrue(any("grouped.rs: expected one registration, found 0" in e for e in check(self.root)))

    def test_fallback_manifest_parser_matches_tomllib(self):
        text = (self.crate / "Cargo.toml").read_text()
        saved = check_test_layout.tomllib
        check_test_layout.tomllib = None
        try:
            fallback = check_test_layout.load_manifest(text)
            self.assertEqual(check(self.root), [])
        finally:
            check_test_layout.tomllib = saved
        self.assertFalse(fallback["package"]["autotests"])
        self.assertEqual(
            [(t["name"], t["path"]) for t in fallback["test"]],
            [("integration", "tests/integration.rs"), ("isolated", "tests/isolated.rs")],
        )
