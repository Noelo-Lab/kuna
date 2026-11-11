package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtMemberExtensionReceiver.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtMemberExtensionReceiver.kt\npkg/TestKtMemberExtensionReceiver\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,26:1\n1#2:27\n*E\n"])
public class TestKtMemberExtensionReceiver {
   public final var seconds: Int = -1
      internal set

   public final var total: Long
      internal set

   public fun seconds(value: Int, unit: pkg.TestKtMemberExtensionReceiver.Unit): TestKtMemberExtensionReceiver {
      val `$this$seconds_u24lambda_u241`: TestKtMemberExtensionReceiver = this
      if (value < 0) {
         throw IllegalArgumentException(("value < 0: $value").toString())
      } else {
         `$this$seconds_u24lambda_u241`.seconds = `$this$seconds_u24lambda_u241`.clampToInt(unit.toSeconds((long)value))
         return this
      }
   }

   public fun add(value: Long): TestKtMemberExtensionReceiver {
      this.total = this.total + this.half(value)
      return this
   }

   public fun direct(value: Long): Int {
      return this.clampToInt(value)
   }

   private fun Long.clampToInt(): Int {
      return if (`$this$clampToInt` > 2147483647L) 2147483647 else (int)`$this$clampToInt`
   }

   private fun Long.half(): Long {
      return `$this$half` / 2 + this.total
   }

   public class Unit(factor: Long) {
      private final val factor: Long

      init {
         this.factor = factor
      }

      public fun toSeconds(value: Long): Long {
         return value * this.factor
      }
   }
}
