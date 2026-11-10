package pkg

import kotlin.jvm.internal.Intrinsics

public class TestExpressionValueCheckLegacy {
   public fun describe(lib: JavaLib): String {
      Intrinsics.checkParameterIsNotNull(lib, "lib")
      val var10000: String = lib.name()
      Intrinsics.checkExpressionValueIsNotNull(var10000, "name(...)")
      return "$var10000${lib.name().length}"
   }

   public companion object {
      public final val DEFAULT: String = JavaLib.version()
      public final val UPPER: String = "${JavaLib.version()}!"
   }
}
