package pkg

class TestKtMemberExtensionReceiver {
   var seconds = -1
   var total = 0L

   class Unit(private val factor: Long) {
      fun toSeconds(value: Long): Long = value * factor
   }

   fun seconds(value: Int, unit: Unit): TestKtMemberExtensionReceiver = apply {
      require(value >= 0) { "value < 0: $value" }
      seconds = unit.toSeconds(value.toLong()).clampToInt()
   }

   fun add(value: Long): TestKtMemberExtensionReceiver = apply {
      total += value.half()
   }

   fun direct(value: Long): Int = value.clampToInt()

   private fun Long.clampToInt(): Int = if (this > Int.MAX_VALUE) Int.MAX_VALUE else toInt()

   private fun Long.half(): Long = this / 2 + total
}
