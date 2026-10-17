import unittest

from scripts.observatory import atlas


def canned():
    doc = {
        "phases": [
            {
                "id": "P3",
                "name": "Definition Web",
                "artifact": "ssa",
                "band": "B",
                "orthogonal": False,
                "folder": "p3_dataflow",
                "readsAs": "definition web",
            }
        ],
        "extraFolders": [{"folder": "infra", "readsAs": "framework"}],
    }
    reg = {
        "group": [{"group": "analysis", "phase": "P3", "subphase": "", "note": "n"}],
        "subphase": [{"name": "s", "phase": "P3", "decision": "d", "assertion": "a",
                      "strength": "HARD", "rewind": "P3", "latent": False, "exposure": ""}],
        "surface": [{"surface": "option x", "phase": "P3", "subphase": "s", "note": ""}],
        "settable": [{"option": "x", "values": "on|off", "default": "on", "phase": "P3",
                      "subphase": "s", "tier": "core", "summary": "sum", "symptoms": "sym"}],
    }
    code = {
        "crates": [{"name": "kuna-decomp", "path": "decompiler/crates/kuna-decomp",
                    "description": "core", "files": 2, "loc": 10, "deps": []}],
        "folders": {"p3_dataflow": {"files": 1, "loc": 7, "modules": [
            {"file": "heritage.rs", "loc": 7, "native": False}]}},
    }
    owners = {"kuna-decomp": {"p3_dataflow": "docs/spec/03-ssa-and-simplification.md",
                              "*": "docs/spec/00-overview.md"}, "kuna-analysis": {}}
    sched = {"source": "s", "note": "n", "legend": {}, "passes": [], "unmapped": []}
    meta = {"sha": "0" * 40, "generated": "2026-10-17T00:00:00Z"}
    return doc, reg, code, owners, sched, meta


class AssembleTests(unittest.TestCase):
    def test_schema(self):
        data = atlas.assemble(*canned())
        self.assertEqual(data["schemaVersion"], 1)
        self.assertEqual(data["counts"], {"group": 1, "subphase": 1, "surface": 1, "settable": 1})
        phase = data["phases"][0]
        self.assertEqual(phase["specChapter"], "docs/spec/03-ssa-and-simplification.md")
        self.assertEqual(phase["loc"], 7)
        self.assertEqual(phase["options"][0]["option"], "x")
        self.assertEqual(phase["groups"][0]["group"], "analysis")
        self.assertEqual(phase["subphases"][0]["name"], "s")
        self.assertEqual(phase["surfaces"][0]["surface"], "option x")
        extra = data["extraFolders"][0]
        self.assertEqual(extra["specChapter"], "docs/spec/00-overview.md")
        self.assertEqual(extra["loc"], 0)


if __name__ == "__main__":
    unittest.main()
