package pkg

public class TestKtImplicitWidening {
   private final var count: Int = 0
   private final val ints: IntArray = IntArray(4)

   public fun encode(data: ByteArray, map: ByteArray): ByteArray {
      val out: ByteArray = ByteArray(data.size / 2 * 3)
      var index: Int = 0
      var i: Int = 0

      while (i + 1 < data.size) {
         val b0: Int = data[i++].toInt()
         val b1: Int = data[i++].toInt()
         out[index++] = map[(b0 and 255) shr 2]
         out[index++] = map[(b0 and 3) shl 4 or ((b1 and 255) shr 4)]
         out[index++] = map[(b1 and 15) shl 2]
      }

      return out
   }

   public fun digit(chars: CharArray, i: Int): Int {
      val c: Int = chars[i].code
      return if (c < 128) c - 48 else c
   }

   public fun pass(b: Byte): Int {
      return this.takeInt(b.toInt()) + this.takeInt(this.widen(b.toShort()))
   }

   public fun widen(s: Short): Int {
      return s.toInt()
   }

   public fun length(s: Short): Long {
      return this.readShort(s) * 2L
   }

   public fun store(data: ByteArray) {
      this.count = data[0].toInt()
      this.ints[1] = data[1].toInt()
   }

   public fun ratio(c: Char): Double {
      return c.code.toDouble()
   }

   private fun readShort(s: Short): Short {
      return (s * 2).toShort()
   }

   private fun takeInt(x: Int): Int {
      return x + this.count + this.ints[0]
   }
}
