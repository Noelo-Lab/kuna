package pkg

public class TestCopiedEquality {
   public fun eqDefault(a: String, b: Boolean = a === "x"): Boolean {
      return b
   }

   public fun neDefault(a: String, b: Boolean = !(a === "x")): Boolean {
      return b
   }

   public fun idDefault(a: Any, b: Boolean = a === "x"): Boolean {
      return b
   }

   public fun eqVarDefault(a: Any?, c: Any?, b: Boolean = a === c): Boolean {
      return b
   }

   public fun superArg(a: String, c: String): pkg.TestCopiedEquality.Base {
      return object : pkg.TestCopiedEquality.Base
   }

   public open class Base(flag: Boolean) {
      public final val flag: Boolean

      init {
         this.flag = flag
      }
   }
}
