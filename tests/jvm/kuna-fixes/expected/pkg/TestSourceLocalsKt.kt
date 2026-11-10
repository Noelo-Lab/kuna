package pkg

public class TestSourceLocalsKt(id: Int, names: Array<String>) {
   private final val id: Int
   private final val names: Array<String>

   init {
      this.id = id
      this.names = names
   }

   public open fun hashCode(): Int {
      return 31 * this.id + this.names.size
   }

   public fun hex(b: Int): String {
      val result: CharArray = CharArray(2)
      result[0] = DIGITS[b shr 2 and 3]
      result[1] = DIGITS[b and 3]
      return String(result)
   }

   public fun pick(c: Boolean, a: String, b: String): String {
      return if (c) a else b
   }

   public companion object {
      private final val DIGITS: CharArray = charArrayOf('0', '1', '2', '3')
   }
}
