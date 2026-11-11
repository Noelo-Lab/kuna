package pkg

public class TestExpressionValueCheck {
   public fun describe(lib: JavaLib): String {
      val var10000: java.lang.String = lib.name()
      return "$var10000${lib.name().length()}"
   }

   @JvmStatic
   fun {
      val var10000: java.lang.String = JavaLib.version()
      DEFAULT = var10000
   }

   public companion object {
      public final val DEFAULT: String
      public final val UPPER: String
   }
}
