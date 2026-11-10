package pkg

public class TestSwitchHeadIndent {
   public final var seen: Int

   public fun note(v: Int): Int {
      this.seen += v
      return this.seen
   }

   public fun pick(s: String, n: Int): Int {
      val a: Int = this.note(n * 2)
      val b: Int = this.note(s.length() + a)
      var var10000: Int
      when (n) {
         1 -> var10000 = a
         2 -> var10000 = b
         3 -> var10000 = a - b
         else -> var10000 = a + b
      }

      return var10000
   }

   public fun name(s: String): Int {
      val k: Int = this.note(s.length())
      return if (s == "one") 1 + k else (if (s == "two") 2 + k else k)
   }
}
