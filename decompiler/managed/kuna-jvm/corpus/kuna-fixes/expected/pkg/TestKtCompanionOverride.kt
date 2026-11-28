package pkg

public abstract class TestKtCompanionOverride {
   public abstract fun nextBits(bits: Int): Int

   public open fun next(): Int {
      return this.nextBits(31)
   }

   public open fun next(until: Int): Int {
      return this.next() % until
   }

   public abstract val size: Int

   public open val label: String
      public get() {
         return "base"
      }


   private fun step(x: Int): Int {
      return x + 1
   }

   public fun stepped(x: Int): Int {
      return this.step(x)
   }

   public companion object Default : TestKtCompanionOverride() {
      public override val label: String = "default"

      public override fun nextBits(bits: Int): Int {
         return bits * 7
      }

      public override fun next(): Int {
         return 41
      }

      public override fun next(until: Int): Int {
         return until - 1
      }

      public override val size: Int
         public get() {
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
         public get() {
            return 5
         }

   }

   public interface Shape {
      @JvmStatic
      TestKtCompanionOverride.Shape.Companion Companion = TestKtCompanionOverride.Shape.Companion.`$$INSTANCE`;

      public abstract fun area(): Int

      public companion object : TestKtCompanionOverride.Shape {
         public override fun area(): Int {
            return 0
         }
      }
   }
}
