package pkg

public class TestVarargFromMetadata(vararg names: Any, counts: IntArray) {
   public final val names: Array<String>
   public final val counts: IntArray

   init {
      this.names = names
      this.counts = counts
   }

   public constructor(vararg parts: String) : this(parts, IntArray(parts.length))
   public constructor(first: Int, rest: IntArray) : this(arrayOfNulls(0), rest)
   public fun sum(values: IntArray): Int {
      var total: Int = 0

      for (v in values) {
         total += v
      }

      return total
   }

   public fun count(sep: String, vararg items: Any): Int {
      return items.length + sep.length()
   }

   public fun first(items: Array<String>): String {
      return items[0]
   }
}
