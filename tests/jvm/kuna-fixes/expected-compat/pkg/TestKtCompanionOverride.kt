package pkg

public abstract class TestKtCompanionOverride {
   public fun stepped(x: Int): Int {
      return this.step(x)
   }

   public companion object Default : TestKtCompanionOverride {
      public open val label: String

      public override fun nextBits(bits: Int): Int {
         return bits * 7
      }

      public override fun next(): Int {
         return 41
      }

      public override fun next(until: Int): Int {
         return until - 1
      }

      public open val size: Int
         public open get() {
            return 3
         }


      private fun step(x: Int): Int {
         return x + 2
      }

      public fun stepTwice(x: Int): Int {
         return this.step(this.step(x))
      }

      public fun twice(n: Int): Int {
         return n * 2
      }

      @JvmStatic
      public final val shared: Int
         public final get() {
            return 5
         }

   }

   public interface Shape {
      @JvmStatic
      TestKtCompanionOverride.Shape.Companion Companion = TestKtCompanionOverride.Shape.Companion.$$INSTANCE;

      public abstract fun area(): Int {
      }

      public companion object : TestKtCompanionOverride.Shape {
         public override fun area(): Int {
            return 0
         }
      }
   }
}
