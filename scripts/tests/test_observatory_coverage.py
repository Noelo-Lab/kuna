import unittest
from pathlib import Path

from scripts.observatory import coverage


def summary(files):
    return {"data": [{"files": files}]}


def entry(path, line_count, line_covered, fn_count=4, fn_covered=2):
    return {
        "filename": path,
        "summary": {
            "lines": {"count": line_count, "covered": line_covered, "percent": 0},
            "functions": {"count": fn_count, "covered": fn_covered, "percent": 0},
        },
    }


class RollupTests(unittest.TestCase):
    ROOT = Path("/repo")

    def test_buckets_and_math(self):
        data = coverage.rollup(
            summary(
                [
                    entry("/repo/decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs", 100, 80),
                    entry("/repo/decompiler/crates/kuna-decomp/src/substrate/op.rs", 50, 10),
                    entry("/repo/decompiler/crates/kuna-decomp/src/lib.rs", 10, 10),
                    entry("/repo/decompiler/crates/kuna-cli/src/main.rs", 40, 20),
                    entry("/elsewhere/external.rs", 999, 999),
                ]
            ),
            self.ROOT,
            runs_failed="1",
            meta={"sha": "0" * 40, "generated": "2026-10-17T00:00:00Z"},
        )
        self.assertEqual(data["totals"]["lines"], {"count": 200, "covered": 120, "percent": 60.0})
        self.assertTrue(data["meta"]["runsFailed"])
        crates = {row["name"]: row for row in data["crates"]}
        self.assertEqual(crates["kuna-decomp"]["lines"]["count"], 160)
        self.assertEqual(crates["kuna-cli"]["lines"]["percent"], 50.0)
        phases = {row["folder"]: row for row in data["phases"]}
        self.assertEqual(phases["P3"]["lines"]["covered"], 80)
        self.assertEqual(phases["substrate"]["lines"]["percent"], 20.0)
        self.assertEqual(phases["(crate root)"]["lines"]["count"], 10)
        self.assertEqual(len(data["files"]), 4)

    def test_runs_failed_false(self):
        data = coverage.rollup(summary([]), self.ROOT, "0", {"sha": "0" * 40, "generated": "x"})
        self.assertFalse(data["meta"]["runsFailed"])
        self.assertEqual(data["totals"]["lines"]["percent"], 0.0)


if __name__ == "__main__":
    unittest.main()
