package pkg

import kotlin.experimental.and
import kotlin.experimental.inv
import kotlin.experimental.xor

class TestNarrowBitwiseOperand {
    val bytes = ByteArray(8)
    var flags: Short = 0
    var mark: Char = 'a'

    fun unsigned(b: Byte): Int = b.toInt() and 0xff
    fun high(s: Short): Int = (s.toInt() shr 8) or 1
    fun code(c: Char): Int = c.code xor 0x20
    fun shiftBy(x: Long, n: Byte): Long = x shl n.toInt()
    fun field(): Int = flags.toInt() and 0x0f
    fun element(i: Int): Int = (bytes[i].toInt() shl 8) or (bytes[i + 1].toInt() and 0xff)
    fun experimental(a: Byte, b: Byte): Byte = (a and b) xor a.inv()
    fun mask(buffer: ByteArray, key: ByteArray, end: Int) {
        var i = 0
        var j = 0
        while (i < end) {
            buffer[i] = (buffer[i].toInt() xor key[j].toInt()).toByte()
            i++
            j++
        }
    }
    fun toggle(c: Char): Char = (c.code xor 0x20).toChar()
}
