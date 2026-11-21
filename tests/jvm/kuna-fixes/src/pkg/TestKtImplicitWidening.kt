package pkg

class TestKtImplicitWidening {
    private var count: Int = 0
    private val ints = IntArray(4)

    fun encode(data: ByteArray, map: ByteArray): ByteArray {
        val out = ByteArray(data.size / 2 * 3)
        var index = 0
        var i = 0
        while (i + 1 < data.size) {
            val b0 = data[i++].toInt()
            val b1 = data[i++].toInt()
            out[index++] = map[(b0 and 0xff shr 2)]
            out[index++] = map[(b0 and 0x03 shl 4) or (b1 and 0xff shr 4)]
            out[index++] = map[(b1 and 0x0f shl 2)]
        }
        return out
    }

    fun digit(chars: CharArray, i: Int): Int {
        val c = chars[i].code
        return if (c < 128) c - 48 else c
    }

    fun pass(b: Byte): Int = takeInt(b.toInt()) + takeInt(widen(b.toShort()))

    fun widen(s: Short): Int = s.toInt()

    fun length(s: Short): Long {
        val n = readShort(s).toLong()
        return n * 2L
    }

    fun store(data: ByteArray) {
        count = data[0].toInt()
        ints[1] = data[1].toInt()
    }

    fun ratio(c: Char): Double = c.code.toDouble()

    private fun readShort(s: Short): Short = (s * 2).toShort()

    private fun takeInt(x: Int): Int = x + count + ints[0]
}
