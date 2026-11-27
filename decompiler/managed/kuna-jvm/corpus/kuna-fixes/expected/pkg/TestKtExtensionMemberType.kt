package pkg

public class TestKtExtensionMemberType {
   public fun rollIn(dice: pkg.TestKtExtensionMemberType.Dice, range: IntRange): Int {
      return dice.roll(range)
   }

   public fun markCount(dice: pkg.TestKtExtensionMemberType.Dice, count: Int): Int {
      return dice.mark(count)
   }

   public fun pickFirst(dice: pkg.TestKtExtensionMemberType.Dice, range: IntRange): Int {
      return TestKtExtensionMemberTypeKt.pick(dice, range)
   }

   public fun scaled(dice: pkg.TestKtExtensionMemberType.Dice, step: Int): Long {
      return TestKtExtensionMemberTypeKt.scale(dice, step)
   }

   public abstract class Dice {
      public abstract fun roll(until: Int): Int

      public abstract fun mark(label: String): Int

      public abstract fun pick(item: Any): Int

      public abstract fun scale(factor: Long): Long
   }
}
