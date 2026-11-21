package pkg

public class TestExpressionValueCheck {
   public fun describe(lib: JavaLib): String {
      val var10000: String = lib.name()
      return "$var10000${lib.name().length}"
   }

   public companion object {
      public final val DEFAULT: String = JavaLib.version()
      public final val UPPER: String = "${JavaLib.version()}!"
   }
}
