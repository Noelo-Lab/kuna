package pkg

import kotlin.jvm.internal.Intrinsics

public class TestExpressionValueCheckLegacy {
   public fun describe(lib: JavaLib): String {
      Intrinsics.checkParameterIsNotNull(lib, "lib")
      val var10000: java.lang.String = lib.name()
      Intrinsics.checkExpressionValueIsNotNull(var10000, "name(...)")
      return "$var10000${lib.name().length()}"
   }

   @JvmStatic
   fun {
      val var10000: java.lang.String = JavaLib.version()
      Intrinsics.checkExpressionValueIsNotNull(var10000, "version(...)")
      DEFAULT = var10000
   }

   public companion object {
      public final val DEFAULT: String
      public final val UPPER: String
   }
}
