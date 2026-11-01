"""Validate the sealed MIPS matrix without installing a cross toolchain."""
import hashlib
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2] / "tests/fixtures/mips-import-flow"


class MipsImportFixtures(unittest.TestCase):
    def test_original_sources_and_images_match_provenance(self):
        manifest = json.loads((ROOT / "provenance.json").read_text())
        for name, digest in manifest["sources"].items():
            self.assertEqual(hashlib.sha256((ROOT / name).read_bytes()).hexdigest(), digest)
        names = set()
        dimensions = set()
        for row in manifest["images"]:
            with self.subTest(image=row["file"]):
                self.assertNotIn(row["file"], names)
                names.add(row["file"])
                data = (ROOT / row["file"]).read_bytes()
                self.assertEqual(hashlib.sha256(data).hexdigest(), row["sha256"])
                self.assertEqual(data[:4], b"\x7fELF")
                flags = row["flags"]
                self.assertEqual(data[4], 2 if "-mabi=64" in flags else 1)
                self.assertEqual(data[5], 1 if "-EL" in flags else 2)
                byteorder = "little" if data[5] == 1 else "big"
                self.assertEqual(int.from_bytes(data[18:20], byteorder), 8)
                if row["source"] == "flow.c" and "-mabi=32" in flags:
                    dimensions.add((next(f for f in flags if f.startswith("-march=")),
                                    data[5], "-DEXPORTED" in flags,
                                    "-fstack-protector-strong" in flags, row.get("stripped", False)))
                    self.assertGreater(row["symbols"]["duplicate"]["size"], 0)
        self.assertEqual(len(names), 44)
        self.assertEqual(len(dimensions), 32)


if __name__ == "__main__":
    unittest.main()
