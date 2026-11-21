package pkg

public class TestKtNarrowConstantTemps {
   public final var seen: Int
      internal set

   public fun note() {
      val var1: Int = this.seen++
   }

   public fun code(kind: Int): Int {
      var var10000: Byte
      when (kind) {
         1 -> {
            this.note()
            var10000 = 20
         }
         2 -> {
            this.note()
            var10000 = 10
         }
         else -> var10000 = 30
      }

      return var10000
   }

   public fun retry(): Int {
      var hits: Int = 0
      val var2: Byte = 3

      repeat(var2) { var3 ->
         hits += var3
      }

      return hits
   }

   public fun limit(values: IntArray): Int {
      var sum: Int = 0

      repeat(9) { i ->
         sum += values[i]
      }

      return sum
   }

   public fun store(target: ByteArray, kind: Int) {
      target[0] = (byte)(if (kind == 0) 7 else 9)
   }

   public fun wide(kind: Int): Int {
      val var10000: Short
      if (kind == 1) {
         this.note()
         var10000 = 300
      } else {
         var10000 = 256
      }

      return var10000
   }

   public fun many(): Int {
      var total: Int = 0
      val var2: Short = 300

      repeat(var2) { var3 ->
         total += var3
      }

      return total
   }

   public fun keepByte(kind: Int): Byte {
      val var10000: Byte
      if (kind == 1) {
         this.note()
         var10000 = 20
      } else {
         var10000 = 30
      }

      return var10000
   }

   public fun shorts(kind: Int): Long {
      val var10000: Int
      if (kind == 0) {
         this.note()
         var10000 = 300
      } else {
         var10000 = 400
      }

      return (long)var10000 * kind
   }
}
