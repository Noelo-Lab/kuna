package pkg

class TestJavaKeywordNames {
   val same: (Any?, Any?) -> Boolean = @JvmSerializableLambda { old, new -> old == new }

   fun shift(new: Int, default: Int): Int {
      return new * 31 + default
   }

   fun count(s: String, char: Char): Int {
      var int = 0
      for (c in s) {
         if (c == char) {
            int++
         }
      }
      return int
   }

   fun pick(flag: Boolean): (Int, Int) -> Int {
      return if (flag) { old: Int, new: Int -> old - new } else { a: Int, b: Int -> a + b }
   }
}
