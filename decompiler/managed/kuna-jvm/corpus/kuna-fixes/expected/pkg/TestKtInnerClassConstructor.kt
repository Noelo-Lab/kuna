package pkg

public class TestKtInnerClassConstructor(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun make(x: Int): pkg.TestKtInnerClassConstructor.Part {
      return Part(x)
   }

   public fun empty(): pkg.TestKtInnerClassConstructor.Empty {
      return Empty()
   }

   public fun other(o: TestKtInnerClassConstructor, x: Int): pkg.TestKtInnerClassConstructor.Part {
      return o.Part(x)
   }

   public inner class Empty {
      public fun base(): Int {
         return this@TestKtInnerClassConstructor.base
      }
   }

   public inner class Part(x: Int) {
      public final val x: Int

      init {
         this.x = x
      }

      public fun sum(): Int {
         return this@TestKtInnerClassConstructor.base + this.x
      }

      public fun next(): pkg.TestKtInnerClassConstructor.Part {
         return this@TestKtInnerClassConstructor.Part(this.x + 1)
      }
   }
}
