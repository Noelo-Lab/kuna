package pkg

public class TestSourceLocalsKt(id: Int, vararg names: Any) {
   private final val id: Int
   private final val names: Array<String>

   init {
      this.id = id
      this.names = names
   }

   public open fun hashCode(): Int {
      return 31 * this.id + this.names.length
   }

   public fun hex(b: Int): String {
      return java.lang.String(charArrayOf(DIGITS[b shr 2 and 3], DIGITS[b and 3]))
   }

   public fun pick(c: Boolean, a: String, b: String): String {
      return if (c) a else b
   }

   public companion object {
      private final val DIGITS: CharArray
   }
}
