package pkg

public class TestMappedJavaType {
   public fun join(parts: Array<String>): String {
      var out: String = ""

      for (p in parts) {
         out = "$out$p"
      }

      return out
   }

   public fun text(cs: CharSequence): String {
      return cs.toString()
   }

   public fun boxed(values: List<Long>): Long {
      var sum: Long = 0L
      val var4: java.util.Iterator = values.iterator()

      while (var4.hasNext()) {
         sum += (var4.next() as Number).toLong()
      }

      return sum
   }

   public fun catching(block: () -> Unit): String {
      try {
         block()
         return ""
      } catch (var3: Throwable) {
         return var3.toString()
      }
   }

   public fun number(n: Number?): Number {
      var var10000: Number = n
      if (n == null) {
         var10000 = 0 as Number
      }

      return var10000
   }
}
