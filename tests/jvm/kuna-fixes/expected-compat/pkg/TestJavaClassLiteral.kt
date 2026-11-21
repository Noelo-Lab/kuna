package pkg

public class TestJavaClassLiteral {
   public final val tag: String

   public fun local(): String {
      val var10000: java.lang.String = TestJavaClassLiteral.class.getSimpleName()
      return var10000
   }

   @JvmStatic
   fun {
      val var10000: java.lang.String = TestJavaClassLiteral.class.getName()
      NAME = var10000
   }

   public companion object {
      public final val NAME: String
      public final val TYPE: Class<*>
   }
}
