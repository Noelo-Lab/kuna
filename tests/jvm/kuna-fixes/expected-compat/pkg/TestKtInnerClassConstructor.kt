package pkg

public class TestKtInnerClassConstructor(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun make(x: Int): pkg.TestKtInnerClassConstructor.Part {
      return TestKtInnerClassConstructor.Part(x)
   }

   public fun empty(): pkg.TestKtInnerClassConstructor.Empty {
      return TestKtInnerClassConstructor.Empty()
   }

   public fun other(o: TestKtInnerClassConstructor, x: Int): pkg.TestKtInnerClassConstructor.Part {
      return o.Part(x)
   }

   public inner class Empty {
      public fun base(): Int {
         return TestKtInnerClassConstructor.this.base
      }
   }

   public inner class Part(x: Int) {
      public final val x: Int

      init {
         this.x = x
      }

      public fun sum(): Int {
         return TestKtInnerClassConstructor.this.base + this.x
      }

      public fun next(): pkg.TestKtInnerClassConstructor.Part {
         return TestKtInnerClassConstructor.this.Part(this.x + 1)
      }
   }
}
