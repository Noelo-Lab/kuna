package pkg

public class TestConstructorDiamond {
   private final val bag: pkg.TestConstructorDiamond.Bag<String> = TestConstructorDiamond.Bag(4)

   public fun empty(): pkg.TestConstructorDiamond.Bag<Long> {
      return TestConstructorDiamond.Bag(0)
   }

   public fun <A, B> pair(a: A, b: B): pkg.TestConstructorDiamond.Pair<A, B> {
      return TestConstructorDiamond.Pair(a, b) as TestConstructorDiamond.Pair<A, B>
   }

   public fun explicit(): Any {
      return TestConstructorDiamond.Bag(1)
   }

   public class Bag<T>(capacity: Int) {
      public final val capacity: Int

      init {
         this.capacity = capacity
      }
   }

   public class Pair<A, B>(first: A, second: B) {
      public final val first: A
      public final val second: B

      init {
         this.first = first as A
         this.second = second as B
      }
   }
}
