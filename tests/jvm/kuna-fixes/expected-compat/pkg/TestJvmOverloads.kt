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

   @JvmOverloads
   fun scaled(x: Int, factor: Int): Int {
      scaled$default(this, x, factor, 0, 4, null)
   }

   @JvmOverloads
   fun scaled(x: Int): Int {
      scaled$default(this, x, 0, 0, 6, null)
   }

   public companion object {
      @JvmOverloads
      public fun join(a: String, sep: String = ",", b: String = "z"): String {
         return "$a$sep$b"
      }

      @JvmOverloads
      fun join(a: java.lang.String, sep: java.lang.String): java.lang.String {
         join$default(this, a, sep, null, 4, null)
      }

      @JvmOverloads
      fun join(a: java.lang.String): java.lang.String {
         join$default(this, a, null, null, 6, null)
      }
   }
}
