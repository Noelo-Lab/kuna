package pkg

public class TestJvmOverloads(base: Int) {
   private final val base: Int

   init {
      this.base = base
   }

   @JvmOverloads
   public fun scaled(x: Int, factor: Int = 2, offset: Int = this.base): Int {
      return x * factor + offset
   }

   public companion object {
      @JvmOverloads
      public fun join(a: String, sep: String = ",", b: String = "z"): String {
         return "$a$sep$b"
      }
   }
}
