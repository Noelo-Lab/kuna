package pkg

public class TestNarrowBitwiseOperand {
   public final val bytes: ByteArray = ByteArray(8)
   public final var flags: Short
   public final var mark: Char = 97

   public fun unsigned(b: Byte): Int {
      return b.toInt() and 255
   }

   public fun high(s: Short): Int {
      return s.toInt() shr 8 or 1
   }

   public fun code(c: Char): Int {
      return c.code xor 32
   }

   public fun shiftBy(x: Long, n: Byte): Long {
      return x shl n.toInt()
   }

   public fun field(): Int {
      return this.flags.toInt() and 15
   }

   public fun element(i: Int): Int {
      return this.bytes[i].toInt() shl 8 or (this.bytes[i + 1].toInt() and 255)
   }

   public fun experimental(a: Byte, b: Byte): Byte {
      return ((a.toInt() and b.toInt()).toByte().toInt() xor (a.toInt().inv()).toByte().toInt()).toByte()
   }

   public fun mask(buffer: ByteArray, key: ByteArray, end: Int) {
      var i: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var j: Int = 0
      while (i < end) {
         buffer[i] = (buffer[i].toInt() xor key[j].toInt()).toByte()
         i++

         j++
      }
   }

   public fun toggle(c: Char): Char {
      return (c.code xor 32).toChar()
   }
}
