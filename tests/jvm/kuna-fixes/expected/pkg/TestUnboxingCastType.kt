package pkg

public class TestUnboxingCastType {
   public fun test(p: (Int) -> Boolean, x: Int): Int {
      return if (p(x)) 1 else 0
   }

   public fun negated(p: (Int) -> Boolean, x: Int): Int {
      return if (!p(x)) 1 else 0
   }

   public fun both(p: (Int) -> Boolean, q: () -> Boolean, x: Int): Boolean {
      return !(p(x) as Boolean) && q() as Boolean
   }

   public fun stored(p: (Int) -> Boolean, x: Int): Boolean {
      return p(x) as Boolean
   }

   public fun char(f: () -> Char): Char {
      return f() as Char
   }

   public fun charPlusOne(f: () -> Char): Int {
      return (f() as Char).code + 1
   }

   public fun long(f: () -> Long): Long {
      return (f() as java.lang.Number).longValue() * 2L
   }

   public fun count(items: List<String>, p: (String) -> Boolean): Int {
      var n: Int = 0

      for (item in items) {
         if (!(p(item) as Boolean)) {
            n++
         }
      }

      return n
   }
}
