import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktenum", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestEnumConstantInValuesArray(unittest.TestCase):
    """The recogniser of enum_constant_in_values_array: bare enum constants gain their arguments."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["enum_constant_in_values_array"])

    def test_constants_gain_arguments(self):
        old = ["   NO_ERROR,", "   PROTOCOL_ERROR,", "   HTTP_1_1_REQUIRED;"]
        new = ["   NO_ERROR(0),", "   PROTOCOL_ERROR(1),", "   HTTP_1_1_REQUIRED(13);"]
        self.assertTrue(self.f(old, new))
        self.assertTrue(self.f(["   TLS_1_3,"], ['   TLS_1_3("TLSv1.3"),']))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   A,", "   B;"], ["   A(1),", "   C(2);"]))
        self.assertFalse(self.f(["   A,"], ["   A(1);"]))
        self.assertFalse(self.f(["   A(1),"], ["   A(2),"]))
        self.assertFalse(self.f(["   A,"], ["   A,"]))
        self.assertFalse(self.f(["   val x: Int"], ["   val x: Int(1)"]))
        self.assertFalse(self.f(["   A,", "   B;"], ["   A(1),"]))


if __name__ == "__main__":
    unittest.main()
