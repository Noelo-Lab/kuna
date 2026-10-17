import unittest
from datetime import datetime, timezone

from scripts.observatory import churn


def commit(date, author="Ada", agent=False, files=()):
    return {"sha": "0" * 40, "date": date, "author": author, "agent": agent, "files": list(files)}


class ClassifyTests(unittest.TestCase):
    def test_buckets(self):
        tags = churn._classify("decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs")
        self.assertEqual(tags, {"area": "decompiler", "crate": "kuna-decomp", "phase": "P3"})
        tags = churn._classify("decompiler/crates/kuna-decomp/src/substrate/op.rs")
        self.assertEqual(tags["phase"], "substrate")
        self.assertEqual(churn._classify("docs/spec/05-types.md")["crate"], None)
        self.assertIsNone(churn._classify("specs/Ghidra/Processors/x86/foo.sinc"))
        self.assertIsNone(churn._classify("go.pid"))


class AggregateTests(unittest.TestCase):
    NOW = datetime(2026, 10, 17, tzinfo=timezone.utc)

    def test_windows_authors_and_tracking(self):
        commits = [
            commit("2026-10-10", agent=True, files=[("docs/a.md", 5, 1)]),
            commit("2026-08-01", author="Bo", files=[("docs/a.md", 7, 2)]),
            commit("2025-01-01", files=[("docs/a.md", 100, 0), ("docs/gone.md", 9, 9)]),
        ]
        rows = churn.aggregate(commits, {"docs/a.md"}, self.NOW)
        self.assertEqual([row["path"] for row in rows], ["docs/a.md"])
        row = rows[0]
        self.assertEqual(row["windows"]["d30"], {"added": 5, "removed": 1, "commits": 1, "agentCommits": 1})
        self.assertEqual(row["windows"]["d90"]["commits"], 2)
        self.assertEqual(row["windows"]["d365"]["commits"], 2)
        self.assertEqual(row["windows"]["all"], {"added": 112, "removed": 3, "commits": 3, "agentCommits": 1})
        self.assertEqual(row["last"], "2026-10-10")
        self.assertEqual(row["authors"][0], {"name": "Ada", "commits": 2})

    def test_window_boundary_inclusive(self):
        boundary = "2026-09-17"  # exactly 30 days before NOW
        rows = churn.aggregate([commit(boundary, files=[("docs/a.md", 1, 0)])], {"docs/a.md"}, self.NOW)
        self.assertEqual(rows[0]["windows"]["d30"]["commits"], 1)


if __name__ == "__main__":
    unittest.main()
