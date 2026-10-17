import unittest

from scripts.observatory import gitstats


class DestinationTests(unittest.TestCase):
    def test_rename_forms(self):
        self.assertEqual(gitstats.destination("a/{b => c}/d.rs"), "a/c/d.rs")
        self.assertEqual(gitstats.destination("old.rs => new.rs"), "new.rs")
        self.assertEqual(gitstats.destination("plain/path.rs"), "plain/path.rs")
        self.assertEqual(gitstats.destination("src/{ => sub}/x.rs"), "src/sub/x.rs")


class AttributionTests(unittest.TestCase):
    def test_matrix(self):
        self.assertTrue(gitstats.has_agent_attribution("[AUTOMATED] fix thing"))
        self.assertTrue(gitstats.has_agent_attribution("fix\n\nCo-authored-by: Claude F <x@y>"))
        self.assertTrue(gitstats.has_agent_attribution("fix\nco-authored-by: Codex <c@o>"))
        self.assertFalse(gitstats.has_agent_attribution("fix\n\nCo-authored-by: A Person <a@b>"))
        self.assertFalse(gitstats.has_agent_attribution("mention claude in prose only"))


class ParseCommitsTests(unittest.TestCase):
    def test_chunks(self):
        log = (
            "\x1e" + "a" * 40 + "\x1f2026-10-01T05:00:00+02:00\x1fAda\x1f[AUTOMATED] subj\x1f"
            "10\t2\tsrc/x.rs\n-\t-\tblob.bin\n3\t0\told.rs => new.rs\n"
            "\x1e" + "b" * 40 + "\x1f2026-09-01T00:00:00+00:00\x1fBo\x1fplain\x1f\n"
        )
        commits = gitstats.parse_commits(log)
        self.assertEqual(len(commits), 2)
        first = commits[0]
        self.assertEqual(first["date"], "2026-10-01")
        self.assertEqual(first["author"], "Ada")
        self.assertTrue(first["agent"])
        self.assertEqual(first["files"], [("src/x.rs", 10, 2), ("new.rs", 3, 0)])
        self.assertEqual(commits[1]["files"], [])
        self.assertFalse(commits[1]["agent"])


if __name__ == "__main__":
    unittest.main()
