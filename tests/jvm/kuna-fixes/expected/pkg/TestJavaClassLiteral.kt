package pkg

public class TestJavaClassLiteral {
   public final val tag: String

   public fun local(): String {
      val var10000: String = TestJavaClassLiteral::class.java.getSimpleName()
      return var10000
   }

   @JvmStatic
   fun {
      val var10000: String = TestJavaClassLiteral::class.java.getName()
      NAME = var10000
   }

   public companion object {
      public final val NAME: String
      public final val TYPE: Class<*> = TestJavaClassLiteral.Companion::class.java
   }
}
