import unittest

from scripts.observatory import registry


class FallbackParserTests(unittest.TestCase):
    def test_escapes_and_types(self):
        text = (
            '# comment\n'
            '[[settable]]\n'
            'option = "alpha"\n'
            'summary = "says \\"hi\\" and\\nbreaks"\n'
            'destructive = false\n'
            'latent = true\n'
            'count = 7\n'
        )
        rows = registry._fallback_parse(text)["settable"]
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["summary"], 'says "hi" and\nbreaks')
        self.assertIs(rows[0]["destructive"], False)
        self.assertIs(rows[0]["latent"], True)
        self.assertEqual(rows[0]["count"], 7)

    @unittest.skipUnless(registry.tomllib, "needs tomllib (Python 3.11+)")
    def test_fallback_matches_tomllib_on_real_file(self):
        text = registry.default_path().read_text(encoding="utf-8")
        fallback = registry._fallback_parse(text)
        reference = registry.tomllib.loads(text)
        for table in registry.TABLES:
            self.assertEqual(fallback.get(table, []), reference.get(table, []), table)

    def test_real_file_shape(self):
        data = registry.load()
        for table in registry.TABLES:
            self.assertTrue(data[table], f"{table} empty")
        for row in data["settable"]:
            self.assertIn(row["tier"], ("core", "transform", "analysis"), row["option"])
            self.assertTrue(row["option"])
        phases = registry.group_phase_map(data)
        self.assertEqual(phases.get("base"), "infra")
        self.assertTrue(all(value for value in phases.values()))


if __name__ == "__main__":
    unittest.main()
