package pkg

public class TestBinaryNumericPromotion(section: Byte, length: Long) {
   private final val section: Byte
   private final val length: Long

   init {
      this.section = section
      this.length = length
   }

   public fun charPlus(c: Char): Int {
      return c + 4
   }

   public fun charMinus(c: Char): Int {
      return c - 48
   }

   public fun charMinusChar(a: Char, b: Char): Int {
      return a - b
   }

   public fun intPlusChar(i: Int, c: Char): Int {
      return i * 10 + c
   }

   public fun timesChar(c: Char): Int {
      return 31 * c
   }

   public fun nextChar(c: Char): Char {
      return (char)(c + 1)
   }

   public fun appendCode(sb: StringBuilder, c: Char): StringBuilder {
      val var10000: StringBuilder = sb.append(c + 1)
      return var10000
   }

   public fun describe(c: Char): String {
      return "code ${c + 1}"
   }

   public fun charLess(c: Char): Boolean {
      return c < 128
   }

   public fun lessChar(c: Char): Boolean {
      return '0' <= c && '9' >= c
   }

   public fun charEq(c: Char, i: Int): Boolean {
      return c == i
   }

   public fun byteEq(b: Byte): Boolean {
      return b == 61
   }

   public fun fieldEq(): Boolean {
      return this.section == 2
   }

   public fun elementEq(i: Int, a: ByteArray): Boolean {
      return i == a[0]
   }

   public fun longNe(l: Long, i: Int): Boolean {
      return l != i
   }

   public fun fieldNe(i: Int): Boolean {
      return this.length != i
   }

   public fun doubleEq(i: Int, d: Double): Boolean {
      return i == d
   }

   public fun shortEqByte(s: Short, b: Byte): Boolean {
      return s == b
   }

   public fun negChar(c: Char): Int {
      return -c
   }
}
