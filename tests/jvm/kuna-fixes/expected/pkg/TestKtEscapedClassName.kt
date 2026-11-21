package pkg

public class TestKtEscapedClassName {
   public final val legacy: pkg.TestKtEscapedClassName.`-Legacy` = TestKtEscapedClassName.`-Legacy`

   public fun make(v: Int): pkg.TestKtEscapedClassName.`in` {
      return TestKtEscapedClassName.`in`(v)
   }

   public fun read(x: pkg.TestKtEscapedClassName.`in`): Int {
      return x.v + TestKtEscapedClassName.`-Legacy`.id()
   }

   public fun pick(flag: Boolean): Any {
      return if (flag) TestKtEscapedClassName.`-Legacy` else TestKtEscapedClassName.`in`(1)
   }

   public object `-Legacy` {
      public fun id(): Int {
         return 7
      }
   }

   public class `in`(v: Int) {
      public final val v: Int

      init {
         this.v = v
      }
   }
}
