"""The round-trip probe's comparison of two `javap -c -p` listings; needs no JDK."""

from __future__ import annotations

import unittest

from scripts.javabench.roundtrip import differing, parse_javap, renumber_slots

LISTING = """Compiled from "A.java"
public class pkg.A {
  public pkg.A();
    Code:
       0: aload_0
       1: invokespecial #1                  // Method java/lang/Object."<init>":()V
       4: return

  static int sum(int);
    Code:
       0: iconst_0
       1: istore_1
       2: iload_1
       3: iload_0
       4: iadd
       5: istore_1
       6: iinc          1, 2
       9: iload_1
      10: ireturn
}
"""


class RoundTripListing(unittest.TestCase):
    def test_members_without_offsets_and_pool_indexes(self):
        members = parse_javap(LISTING)
        self.assertEqual(list(members), ["public pkg.A();", "static int sum(int);"])
        self.assertEqual(members["public pkg.A();"][2].split(), ["invokespecial", "#", "//", "Method", 'java/lang/Object."<init>":()V'])
        self.assertEqual(members["static int sum(int);"][-1], "ireturn")

    def test_pool_index_does_not_decide(self):
        other = LISTING.replace("#1 ", "#7 ")
        self.assertEqual(differing(parse_javap(LISTING), parse_javap(other)), [])

    def test_slot_numbers_do_not_decide(self):
        moved = (
            LISTING.replace("istore_1", "istore        5").replace("iload_1", "iload         5").replace("1, 2", "5, 2")
        )
        self.assertEqual(differing(parse_javap(LISTING), parse_javap(moved)), [])
        self.assertEqual(renumber_slots(["istore 5", "iload_0", "iinc 5, -1"]), ["istore v0", "iload v1", "iinc v0, -1"])

    def test_changed_code_and_missing_member(self):
        changed = LISTING.replace("iadd", "isub")
        self.assertEqual(differing(parse_javap(LISTING), parse_javap(changed)), ["static int sum(int);"])
        without = LISTING[: LISTING.index("  static int sum")] + "}\n"
        self.assertEqual(differing(parse_javap(LISTING), parse_javap(without)), ["static int sum(int);"])


if __name__ == "__main__":
    unittest.main()
