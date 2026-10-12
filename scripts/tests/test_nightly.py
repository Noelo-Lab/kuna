import copy
import math
import unittest

from scripts.nightly import compare, dataset


def results(functions, binaries=None, **meta):
    base_meta = {
        "dataset_revision": "rev", "config": "optimized", "sample_digest": "d",
        "decbench_commit": "c", "rust_joern": "j", "max_fn_seconds": 600,
        "label": "tonight", "started": "2026-10-11T07:17:00+00:00", "cpu_model": "cpu",
        "nproc": 4,
    }
    base_meta.update(meta)
    return {
        "schema": 1,
        "meta": base_meta,
        "functions": functions,
        "binaries": binaries or {"b": {"time": {"cpu": 1.0, "wall": 1.0, "exit": 0}}},
    }


def binary(cpu, base=None, exit_code=0):
    record = {"time": {"cpu": cpu, "wall": cpu, "exit": exit_code}}
    if base is not None:
        record["baseline_time"] = {"cpu": base, "wall": base, "exit": 0}
    return record


class StatisticsTests(unittest.TestCase):
    def test_sign_test_matches_the_exact_binomial(self):
        self.assertEqual(compare.binom_two_sided(0, 0), 1.0)
        self.assertAlmostEqual(compare.binom_two_sided(0, 10), 2 / 1024)
        self.assertAlmostEqual(compare.binom_two_sided(5, 10), 1.0)
        self.assertAlmostEqual(compare.binom_two_sided(8, 10), 2 * (1 + 10 + 45) / 1024)
        self.assertLess(compare.binom_two_sided(30, 5000), 1e-6)
        self.assertGreater(compare.binom_two_sided(2500, 5000), 0.9)

    def test_holm_is_monotone_and_capped(self):
        adjusted = compare.holm({"a": 0.01, "b": 0.04, "c": 0.03, "d": 0.5})
        self.assertAlmostEqual(adjusted["a"], 0.04)
        self.assertAlmostEqual(adjusted["c"], 0.09)
        self.assertAlmostEqual(adjusted["b"], 0.09)
        self.assertAlmostEqual(adjusted["d"], 0.5)

    def test_bootstrap_interval_brackets_the_ratio(self):
        pairs = [(1.0, 1.1), (2.0, 2.2), (4.0, 4.4), (0.5, 0.55)] * 10
        ratio, lo, hi = compare.bootstrap_ratio(pairs)
        self.assertAlmostEqual(ratio, 1.1)
        self.assertLessEqual(lo, ratio)
        self.assertGreaterEqual(hi, ratio)
        self.assertGreater(lo, 1.0)


class ComparisonTests(unittest.TestCase):
    def test_a_missing_function_is_the_worst_value(self):
        self.assertEqual(compare.ordering_value("ged", None), -math.inf)
        self.assertGreater(compare.ordering_value("ged", 50.0), compare.ordering_value("ged", None))
        self.assertGreater(compare.ordering_value("ged", 0.0), compare.ordering_value("ged", 3.0))
        self.assertGreater(compare.ordering_value("type_match", 1.0),
                           compare.ordering_value("type_match", 0.5))

    def test_a_consistent_loss_is_a_significant_regression(self):
        prev = results({f"p/b/f{i}": {"ged": 0.0, "type_match": 1.0} for i in range(40)})
        cur = copy.deepcopy(prev)
        for i in range(20):
            cur["functions"][f"p/b/f{i}"]["ged"] = 4.0
        ev = compare.evaluate(cur, prev, alpha=0.01, speed_threshold=0.05)
        self.assertIsNone(ev["incompatible"])
        self.assertEqual(len(ev["paired"]["ged"]["lost"]), 20)
        self.assertEqual(ev["verdicts"]["ged:perfect"][0], "worse")
        self.assertEqual(ev["verdicts"]["type_match:perfect"][0], "no significant change")
        self.assertIn("GED perfect", ev["regressions"])

    def test_balanced_churn_is_not_significant(self):
        prev = results({f"p/b/f{i}": {"ged": float(i % 2) * 3} for i in range(40)})
        cur = copy.deepcopy(prev)
        for i in range(10):
            cur["functions"][f"p/b/f{i}"]["ged"] = 3.0 - cur["functions"][f"p/b/f{i}"]["ged"]
        ev = compare.evaluate(cur, prev, alpha=0.01, speed_threshold=0.05)
        self.assertEqual(ev["verdicts"]["ged:perfect"][0], "no significant change")
        self.assertEqual(ev["regressions"], [])

    def test_a_pinned_input_change_skips_the_score_comparison(self):
        prev = results({"p/b/f": {"ged": 0.0}})
        cur = results({"p/b/f": {"ged": 9.0}}, decbench_commit="other")
        ev = compare.evaluate(cur, prev, alpha=0.01, speed_threshold=0.05)
        self.assertIn("decbench_commit", ev["incompatible"])
        self.assertIsNone(ev["paired"])
        self.assertEqual(ev["regressions"], [])

    def test_same_runner_slowdown_beyond_the_threshold_is_a_regression(self):
        binaries = {f"b{i}": binary(1.2 * (i + 1), base=1.0 * (i + 1)) for i in range(30)}
        cur = results({}, binaries)
        ev = compare.evaluate(cur, None, alpha=0.01, speed_threshold=0.05)
        self.assertEqual(ev["speed"]["source"], compare.SAME_RUNNER)
        self.assertAlmostEqual(ev["speed"]["ratio"], 1.2)
        self.assertTrue(any("CPU time" in r for r in ev["regressions"]))

    def test_a_handful_of_binaries_is_never_a_speed_regression(self):
        binaries = {f"b{i}": binary(2.0, base=1.0) for i in range(3)}
        ev = compare.evaluate(results({}, binaries), None, alpha=0.01, speed_threshold=0.05)
        self.assertAlmostEqual(ev["speed"]["ratio"], 2.0)
        self.assertEqual(ev["regressions"], [])

    def test_cross_runner_speed_is_reported_but_never_a_regression(self):
        prev = results({}, {f"b{i}": binary(1.0) for i in range(30)})
        cur = results({}, {f"b{i}": binary(2.0) for i in range(30)})
        ev = compare.evaluate(cur, prev, alpha=0.01, speed_threshold=0.05)
        self.assertEqual(ev["speed"]["source"], compare.PREVIOUS_RUNNER)
        self.assertEqual(ev["regressions"], [])

    def test_a_newly_failing_binary_is_a_regression(self):
        prev = results({}, {"b": binary(1.0)})
        cur = results({}, {"b": binary(1.0, exit_code=101)})
        ev = compare.evaluate(cur, prev, alpha=0.01, speed_threshold=0.05)
        self.assertEqual(ev["new_failures"], ["b"])
        self.assertTrue(ev["regressions"])

    def test_report_renders_with_and_without_a_baseline(self):
        prev = results({"p/b/f": {"ged": 2.0, "type_match": None}})
        cur = results({"p/b/f": {"ged": 0.0, "type_match": 0.5}},
                      {"b": binary(1.0, base=1.1)})
        for previous in (None, prev):
            ev = compare.evaluate(cur, previous, alpha=0.01, speed_threshold=0.05)
            text = compare.render(cur, previous, ev, [compare.history_entry(cur, ev)], 0.01, 0.05, None)
            self.assertIn("### Scores", text)
            self.assertIn("### Speed", text)


class SampleTests(unittest.TestCase):
    def manifest(self):
        binaries = []
        for project, count in (("big", 10), ("mid", 3), ("one", 1)):
            for i in range(count):
                binaries.append({
                    "project": project, "opt": "O2-noinline", "binary": f"{project}{i}",
                    "binary_path": f"binaries/O2-noinline/{project}/{project}{i}",
                    "sha256": f"{project}{i}", "size": 1, "functions": ["f"],
                    "source_cfg_path": f"pipeline_data/{project}{i}.json",
                })
        return {"binaries": binaries, "projects": {}}

    def test_every_project_is_represented_before_any_repeats(self):
        picked = dataset.select(self.manifest(), 3, "seed")
        self.assertEqual(sorted(b["project"] for b in picked), ["big", "mid", "one"])
        picked = dataset.select(self.manifest(), 8, "seed")
        projects = [b["project"] for b in picked]
        self.assertEqual(projects.count("one"), 1)
        self.assertEqual(projects.count("mid"), 3)
        self.assertEqual(projects.count("big"), 4)

    def test_the_sample_is_a_function_of_the_seed(self):
        a = dataset.build_sample(self.manifest(), "rev", "optimized", 6, "seed")
        b = dataset.build_sample(self.manifest(), "rev", "optimized", 6, "seed")
        c = dataset.build_sample(self.manifest(), "rev", "optimized", 6, "other")
        self.assertEqual(a, b)
        self.assertEqual(a["digest"], b["digest"])
        self.assertNotEqual(a["digest"], c["digest"])


if __name__ == "__main__":
    unittest.main()
