package pkg

public class TestNarrowBitwiseOperand {
   public final val bytes: ByteArray = ByteArray(8)

   public final var flags: Short
      internal set

   public final var mark: Char = 97
      internal set

   public fun unsigned(b: Byte): Int {
      return b and 255
   }

   public fun high(s: Short): Int {
      return s shr 8 or 1
   }

   public fun code(c: Char): Int {
      return c xor 32
   }

   public fun shiftBy(x: Long, n: Byte): Long {
      return x shl n
   }

   public fun field(): Int {
      return this.flags and 15
   }

   public fun element(i: Int): Int {
      return this.bytes[i] shl 8 or this.bytes[i + 1] and 255
   }

   public fun experimental(a: Byte, b: Byte): Byte {
      return (byte)((byte)(a and b) xor (byte)(a.inv()))
   }

   public fun mask(buffer: ByteArray, key: ByteArray, end: Int) {
      var i: Int = 0
         }

   public fun toggle(c: Char): Char {
      return (char)(c xor 32)
   }
}
