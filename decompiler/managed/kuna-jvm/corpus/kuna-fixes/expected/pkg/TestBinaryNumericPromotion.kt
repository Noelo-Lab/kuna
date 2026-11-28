package pkg

public class TestBinaryNumericPromotion(section: Byte, length: Long) {
   private final val section: Byte
   private final val length: Long

   init {
      this.section = section
      this.length = length
   }

   public fun charPlus(c: Char): Int {
      return c.code + 4
   }

   public fun charMinus(c: Char): Int {
      return c.code - 48
   }

   public fun charMinusChar(a: Char, b: Char): Int {
      return a - b
   }

   public fun intPlusChar(i: Int, c: Char): Int {
      return i * 10 + c.code
   }

   public fun timesChar(c: Char): Int {
      return 31 * c.code
   }

   public fun nextChar(c: Char): Char {
      return (c.code + 1).toChar()
   }

   public fun appendCode(sb: StringBuilder, c: Char): StringBuilder {
      val var10000: StringBuilder = sb.append(c.code + 1)
      return var10000
   }

   public fun describe(c: Char): String {
      return "code ${c.code + 1}"
   }

   public fun charLess(c: Char): Boolean {
      return c.code < 128
   }

   public fun lessChar(c: Char): Boolean {
      return '0' <= c && '9' >= c
   }

   public fun charEq(c: Char, i: Int): Boolean {
      return c.code == i
   }

   public fun byteEq(b: Byte): Boolean {
      return b.toInt() == 61
   }

   public fun fieldEq(): Boolean {
      return this.section.toInt() == 2
   }

   public fun elementEq(i: Int, a: ByteArray): Boolean {
      return i == a[0].toInt()
   }

   public fun longNe(l: Long, i: Int): Boolean {
      return l != i.toLong()
   }

   public fun fieldNe(i: Int): Boolean {
      return this.length != i.toLong()
   }

   public fun doubleEq(i: Int, d: Double): Boolean {
      return i.toDouble() == d
   }

   public fun shortEqByte(s: Short, b: Byte): Boolean {
      return s.toInt() == b.toInt()
   }

   public fun negChar(c: Char): Int {
      return -c.code
   }
}
