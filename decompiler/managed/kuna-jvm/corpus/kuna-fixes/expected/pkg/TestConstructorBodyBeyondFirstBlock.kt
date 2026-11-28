package pkg

public class TestConstructorBodyBeyondFirstBlock {
   public open class Base(n: Int) {
      public final val n: Int

      init {
         this.n = n
      }
   }

   public class Checked(n: Int) : TestConstructorBodyBeyondFirstBlock.Base(n) {
      init {
         if (n > 5) {
            throw IllegalStateException("big")
         }
      }
   }

   public class Pair(a: Int, b: Int) {
      public final val a: Int
      public final val b: Int

      init {
         this.a = a
         this.b = b
      }

      public constructor(a: Int) : this(a, a) {
         if (a < 0) {
            throw IllegalArgumentException("negative")
         }
      }
   }

   public class Plain(n: Int) : TestConstructorBodyBeyondFirstBlock.Base(n)
}
