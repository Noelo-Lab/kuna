import unittest
import xml.dom.minidom

from scripts.decbench import card

AGG = {
    "generated_at": "2026-11-04T04:21:51",
    "sample_set_only": ["agent"],
    "decompiler_presets": {"llm@m": ["optimized"]},
    "decompiler_versions": {"kuna": "1.5"},
    "decompiler_registry": {"ida": {"display_name": "Hex-Rays"}, "llm@m": {"display_name": "LLM"}},
    "combos": {
        "unoptimized|0": {"overall": {"kuna": [60, 100], "ida": [50, 100], "agent": [99, 100], "llm@m": [0, 100]}},
        "optimized|0": {"overall": {"kuna": [30, 100], "ida": [20, 100], "llm@m": [70, 100]}},
    },
}


class DecBenchCardTests(unittest.TestCase):
    def test_first_place_names_the_runner_up(self):
        row = card.standings(AGG, "unoptimized")
        self.assertEqual((row["rank"], row["of"]), (1, 2))
        self.assertEqual(row["other"], {"name": "Hex-Rays", "share": 0.5, "rank": 2})

    def test_second_place_names_the_leader_with_its_model(self):
        row = card.standings(AGG, "optimized")
        self.assertEqual((row["rank"], row["of"]), (2, 3))
        self.assertEqual(row["other"]["name"], "LLM m")

    def test_render_is_wellformed_and_carries_the_date(self):
        for palette in card.PALETTES.values():
            for svg in (card.render(AGG, palette), card.render_narrow(AGG, palette)):
                xml.dom.minidom.parseString(svg)
                self.assertIn("2026-11-04", svg)
                self.assertIn("60.0%", svg)


if __name__ == "__main__":
    unittest.main()
