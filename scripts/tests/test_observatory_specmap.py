import unittest
from pathlib import Path

from scripts.observatory import specmap
from scripts.paths import repo_root


class SpecMapTests(unittest.TestCase):
    def test_every_decomp_folder_has_one_chapter(self):
        owners = specmap.folder_chapters()
        src = repo_root() / "decompiler" / "crates" / "kuna-decomp" / "src"
        for entry in sorted(src.iterdir()):
            if not entry.is_dir():
                continue
            chapter = specmap.chapter_for(owners, "kuna-decomp", entry.name)
            self.assertIsNotNone(chapter, f"no owning spec chapter for {entry.name}")
            self.assertTrue((repo_root() / chapter).is_file(), chapter)

    def test_analysis_crate_is_owned(self):
        owners = specmap.folder_chapters()
        self.assertEqual(
            specmap.chapter_for(owners, "kuna-analysis", "loader"),
            "docs/spec/01-program-prep.md",
        )


if __name__ == "__main__":
    unittest.main()
