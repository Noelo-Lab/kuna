import json
from pathlib import Path
import tempfile
import unittest

from scripts.repipe import counters


class CatalogRowsTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-counters-test-")
        self.addCleanup(temporary.cleanup)
        self.dir = Path(temporary.name)

    def fake_kuna(self, rows):
        script = self.dir / "kuna"
        script.write_text("#!/bin/sh\ncat <<'EOF'\n{}\nEOF\n".format(json.dumps(rows)))
        script.chmod(0o755)
        return script

    def test_rows_without_engine_are_native(self):
        rows = [{"option": "loweredswitch", "tier": "transform"}]
        self.assertEqual(counters.catalog_rows(repo=self.dir, kuna=self.fake_kuna(rows)), rows)

    def test_jvm_rows_are_dropped(self):
        rows = [
            {"option": "loweredswitch", "tier": "transform", "engine": "native"},
            {"option": "jvm_decompile_finally", "tier": "transform", "engine": "jvm"},
        ]
        native = counters.catalog_rows(repo=self.dir, kuna=self.fake_kuna(rows))
        self.assertEqual([r["option"] for r in native], ["loweredswitch"])

    def test_a_catalog_of_only_jvm_rows_is_an_error(self):
        rows = [{"option": "jvm_decompile_finally", "engine": "jvm"}]
        with self.assertRaises(counters.DeriveError):
            counters.catalog_rows(repo=self.dir, kuna=self.fake_kuna(rows))


if __name__ == "__main__":
    unittest.main()
