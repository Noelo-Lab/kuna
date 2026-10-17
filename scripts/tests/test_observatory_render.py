import json
import tempfile
import unittest
from pathlib import Path

from scripts.observatory import common, render


class InlineJsonTests(unittest.TestCase):
    def test_escapes_script_close(self):
        self.assertEqual(common.inline_json({"x": "</script><b>"}), '{"x":"<\\/script><b>"}')


class RenderTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-observatory-")
        self.addCleanup(temporary.cleanup)
        self.out = Path(temporary.name)

    def test_renders_without_any_data(self):
        pages = render.render(self.out)
        names = {page.name for page in pages}
        self.assertIn("index.html", names)
        self.assertIn("atlas.html", names)
        atlas_page = (self.out / "atlas.html").read_text(encoding="utf-8")
        self.assertIn('id="data-status"', atlas_page)
        self.assertNotIn('id="data-atlas"', atlas_page)
        self.assertNotIn("fetch(", atlas_page)
        self.assertNotIn("http://", atlas_page)
        self.assertNotIn("https://", atlas_page)

    def test_inlines_available_data(self):
        payload = {"meta": {"sha": "a" * 40, "generated": "2026-10-17T00:00:00Z"}, "x": "</tag>"}
        common.write_json(self.out / "data" / "atlas.json", payload)
        render.render(self.out)
        atlas_page = (self.out / "atlas.html").read_text(encoding="utf-8")
        self.assertIn('id="data-atlas"', atlas_page)
        self.assertIn("<\\/tag>", atlas_page)
        status = json.loads(
            atlas_page.split('id="data-status">', 1)[1].split("</script>", 1)[0]
        )
        self.assertTrue(status["datasets"]["atlas"]["available"])


if __name__ == "__main__":
    unittest.main()
