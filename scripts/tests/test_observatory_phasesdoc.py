import unittest

from scripts.observatory import phasesdoc


class PhasesDocTests(unittest.TestCase):
    def setUp(self):
        self.doc = phasesdoc.parse()

    def test_ten_phases_in_order(self):
        ids = [phase["id"] for phase in self.doc["phases"]]
        self.assertEqual(ids, [f"P{n}" for n in range(10)])

    def test_band_b_is_p3_to_p6(self):
        band = [phase["id"] for phase in self.doc["phases"] if phase["band"] == "B"]
        self.assertEqual(band, ["P3", "P4", "P5", "P6"])

    def test_p0_is_orthogonal(self):
        p0 = self.doc["phases"][0]
        self.assertTrue(p0["orthogonal"])
        self.assertFalse(any(phase["orthogonal"] for phase in self.doc["phases"][1:]))

    def test_folders_and_shared_layers(self):
        for phase in self.doc["phases"]:
            self.assertRegex(phase["folder"], r"^p\d_\w+$")
            self.assertTrue(phase["artifact"])
            self.assertTrue(phase["readsAs"])
        extra = {entry["folder"] for entry in self.doc["extraFolders"]}
        self.assertEqual(extra, {"substrate", "infra"})


if __name__ == "__main__":
    unittest.main()
