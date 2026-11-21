package pkg

public class TestConstructorBodyBeyondFirstBlock {
   public open class Base(n: Int) {
      public final val n: Int

      init {
         this.n = n
      }
   }

   public class Checked(n: Int) : TestConstructorBodyBeyondFirstBlock.Base(n)

   public class Pair(a: Int, b: Int) {
      public final val a: Int
      public final val b: Int

      init {
         this.a = a
         this.b = b
      }

      public constructor(a: Int) : this(a, a)   }

   public class Plain(n: Int) : TestConstructorBodyBeyondFirstBlock.Base(n)
}
