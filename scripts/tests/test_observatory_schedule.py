import unittest

from scripts.observatory import registry, schedule


def action_line(index, name, depth, repeat=False, once=False):
    return (
        f"{index:04}"
        + (" repeat " if repeat else "        ")
        + ("!" if once else " ")
        + "  "
        + " " * (depth * 5 + 2)
        + name
    )


def rule_line(index, name, depth, disabled=False):
    return f"{index:04}" + ("D" if disabled else " ") + " " + " " * (depth * 5 + 2) + name


class SnapshotParserTests(unittest.TestCase):
    def test_column_arithmetic_recovers_kind_and_depth(self):
        text = "\n".join(
            [
                action_line(0, "universal", 0, once=True),
                action_line(1, "fullloop", 1, repeat=True),
                rule_line(2, "subcancel", 2),
                rule_line(3, "deadrule", 3, disabled=True),
            ]
        )
        rows = schedule.parse_snapshot(text)
        self.assertEqual(
            [(row["lineKind"], row["depth"]) for row in rows],
            [("action-format", 0), ("action-format", 1), ("rule-format", 2), ("rule-format", 3)],
        )
        self.assertTrue(rows[0]["once"])
        self.assertTrue(rows[1]["repeat"])
        self.assertTrue(rows[3]["disabled"])
        self.assertEqual(rows[2]["name"], "subcancel")

    def test_rejects_bad_columns(self):
        with self.assertRaises(ValueError):
            schedule.parse_snapshot("0000     x")  # name col 9: neither line format


class LiveTreeTests(unittest.TestCase):
    """Run against the committed snapshot + source so registry-format drift
    (rrow!/ActionBase refactors) fails here the day it lands."""

    @classmethod
    def setUpClass(cls):
        data = registry.load()
        cls.groups = registry.group_phase_map(data)
        cls.sched = schedule.build(cls.groups)

    def test_every_snapshot_line_parses(self):
        self.assertGreater(len(self.sched["passes"]), 200)

    def test_unmapped_stays_tiny(self):
        self.assertLessEqual(len(self.sched["unmapped"]), 5, self.sched["unmapped"])

    def test_groups_resolve_to_phases(self):
        for item in self.sched["passes"]:
            if item["group"] is not None:
                self.assertIn(item["group"], self.groups, item["name"])
            if item["folderPhase"] is not None:
                self.assertRegex(item["folderPhase"], r"^(P\d|substrate|infra|src)$")

    def test_rules_carry_defining_module(self):
        rules = [item for item in self.sched["passes"] if item["kind"] == "rule"]
        self.assertGreater(len(rules), 100)
        missing = [item["name"] for item in rules if not item["definedIn"]]
        self.assertEqual(missing, [])


if __name__ == "__main__":
    unittest.main()
