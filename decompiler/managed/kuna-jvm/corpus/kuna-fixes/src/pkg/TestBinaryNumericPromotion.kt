package pkg

class TestBinaryNumericPromotion(private val section: Byte, private val length: Long) {
    fun charPlus(c: Char): Int = c.code + 4
    fun charMinus(c: Char): Int = c.code - 48
    fun charMinusChar(a: Char, b: Char): Int = a - b
    fun intPlusChar(i: Int, c: Char): Int = i * 10 + c.code
    fun timesChar(c: Char): Int = 31 * c.code
    fun nextChar(c: Char): Char = (c.code + 1).toChar()
    fun appendCode(sb: StringBuilder, c: Char): StringBuilder = sb.append(c.code + 1)
    fun describe(c: Char): String = "code " + (c.code + 1)
    fun charLess(c: Char): Boolean = c.code < 128
    fun lessChar(c: Char): Boolean = 48 <= c.code && 57 >= c.code
    fun charEq(c: Char, i: Int): Boolean = c.code == i
    fun byteEq(b: Byte): Boolean = b.toInt() == 61
    fun fieldEq(): Boolean = section.toInt() == 2
    fun elementEq(i: Int, a: ByteArray): Boolean = i == a[0].toInt()
    fun longNe(l: Long, i: Int): Boolean = l != i.toLong()
    fun fieldNe(i: Int): Boolean = length != i.toLong()
    fun doubleEq(i: Int, d: Double): Boolean = i.toDouble() == d
    fun shortEqByte(s: Short, b: Byte): Boolean = s.toInt() == b.toInt()
    fun negChar(c: Char): Int = -c.code
}
