package pkg

public class TestKtImplicitWidening {
   private final var count: Int
   private final val ints: IntArray = IntArray(4)

   public fun encode(data: ByteArray, map: ByteArray): ByteArray {
      val out: ByteArray = ByteArray(data.length / 2 * 3)
      var index: Int = 0
      var i: Int = 0

      while (i + 1 < data.length) {
         val b0: Int = data[i++]
         val b1: Int = data[i++]
         out[index++] = map[(b0 and 255) shr 2]
         out[index++] = map[(b0 and 3) shl 4 or (b1 and 255) shr 4]
         out[index++] = map[(b1 and 15) shl 2]
      }

      return out
   }

   public fun digit(chars: CharArray, i: Int): Int {
      return if (chars[i] < 128) chars[i] - 48 else chars[i]
   }

   public fun pass(b: Byte): Int {
      return this.takeInt(b) + this.takeInt(this.widen((short)b))
   }

   public fun widen(s: Short): Int {
      return s
   }

   public fun length(s: Short): Long {
      return this.readShort(s) * 2L
   }

   public fun store(data: ByteArray) {
      this.count = data[0]
      this.ints[1] = data[1]
   }

   public fun ratio(c: Char): Double {
      return c
   }

   private fun readShort(s: Short): Short {
      return (short)(s * 2)
   }

   private fun takeInt(x: Int): Int {
      return x + this.count + this.ints[0]
   }
}
